//! Small, fail-closed source Take mechanism for SchemaSchlopf.
//!
//! This module is governance infrastructure. It does not participate in XSD
//! qualification or MusicXML instance semantics.

use crate::assistant_export::{EXPORT_ZIP_NAME, run_assistant_export};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const TAKE_FORMAT: &str = "SchemaSchlopfTake/1";
const TAKE_PROJECT: &str = "SchemaSchlopf";
const TAKE_MANIFEST_NAME: &str = "SCHEMASCHLOPF_TAKE.txt";
const TAKE_PAYLOAD_PREFIX: &str = "payload/";
const TAKE_DISCOVERY_PREFIX: &str = "SchemaSchlopf_Take_";
const MAX_PAYLOAD_FILE_BYTES: u64 = 32 * 1024 * 1024;
const MAX_TOTAL_PAYLOAD_BYTES: u64 = 128 * 1024 * 1024;
pub(crate) const SOURCE_MANIFEST_PATH: &str = "SOURCE_MANIFEST.sha256";

#[derive(Debug, Clone, PartialEq, Eq)]
struct TakeFile {
    path: String,
    target_sha256: String,
    base: BaseState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BaseState {
    Absent,
    Sha256(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TakeManifest {
    base_version: String,
    base_revision: String,
    target_version: String,
    target_revision: String,
    name: String,
    allow_dependency_change: bool,
    allow_normative_change: bool,
    files: Vec<TakeFile>,
}

#[derive(Debug)]
struct LoadedTake {
    manifest: TakeManifest,
    payload: BTreeMap<String, Vec<u8>>,
    package_sha256: String,
}

#[derive(Debug)]
struct CurrentIdentity {
    version: String,
    revision: String,
}

#[derive(Debug)]
struct BackupEntry {
    path: String,
    previous: Option<Vec<u8>>,
}

pub fn run_take_bootstrap(project_root: &Path) -> Result<(), String> {
    let project_root = project_root
        .canonicalize()
        .map_err(|error| format!("project root cannot be resolved: {error}"))?;
    verify_source_manifest(&project_root)?;
    let current = read_current_identity(&project_root)?;
    let rejected = read_rejected_hashes(&project_root)?;

    let candidate = discover_candidate(&project_root, &current, &rejected)?;

    let loaded = load_take(&candidate)?;
    if rejected.contains(&loaded.package_sha256) {
        return Err(format!(
            "take package was already rejected fail-closed: {}",
            loaded.package_sha256
        ));
    }

    verify_manifest_against_current(&loaded.manifest, &current)?;
    verify_take_policy(&loaded.manifest)?;
    verify_base_states(&project_root, &loaded.manifest)?;

    println!("[SchemaSchlopf take] Paket: {}", candidate.display());
    println!(
        "[SchemaSchlopf take] SHA-256: {}",
        loaded.package_sha256
    );
    println!(
        "[SchemaSchlopf take] PRE-TAKE-GATE · {} -> {} · {} Dateien · OK",
        loaded.manifest.base_revision,
        loaded.manifest.target_revision,
        loaded.manifest.files.len()
    );

    let backups = match install_payload(&project_root, &loaded) {
        Ok(backups) => backups,
        Err(error) => {
            append_rejected_hash(&project_root, &loaded.package_sha256)?;
            return Err(format!(
                "take rejected during installation: {error}; package SHA-256 {} is recorded as rejected",
                loaded.package_sha256
            ));
        }
    };

    if let Err(gate_error) = run_post_install_gates(&project_root) {
        eprintln!("[SchemaSchlopf take] POST-TAKE-GATE FEHLER: {gate_error}");
        return reject_installed_take(
            &project_root,
            &loaded,
            &backups,
            "post-install gate failure",
        );
    }

    println!("[SchemaSchlopf take] ASSISTANT-EXPORT ...");
    let export = match run_assistant_export(&project_root) {
        Ok(summary) => summary,
        Err(export_error) => {
            eprintln!("[SchemaSchlopf take] ASSISTANT-EXPORT FEHLER: {export_error}");
            return reject_installed_take(
                &project_root,
                &loaded,
                &backups,
                "assistant-export failure",
            );
        }
    };

    println!(
        "[SchemaSchlopf take] Assistant export: {}",
        export.zip_path.display()
    );
    println!(
        "[SchemaSchlopf take] Assistant export SHA-256: {}",
        export.zip_sha256
    );
    println!(
        "[SchemaSchlopf take] Installiert · {} · {}",
        loaded.manifest.target_revision, loaded.manifest.name
    );
    Ok(())
}

fn reject_installed_take(
    project_root: &Path,
    loaded: &LoadedTake,
    backups: &[BackupEntry],
    reason: &str,
) -> Result<(), String> {
    rollback(project_root, backups)?;
    verify_base_states(project_root, &loaded.manifest)?;
    verify_source_manifest(project_root)?;
    append_rejected_hash(project_root, &loaded.package_sha256)?;
    eprintln!("[SchemaSchlopf take] ROLLBACK · Ausgangsdateien wiederhergestellt");

    if let Err(rollback_gate_error) = run_post_install_gates(project_root) {
        return Err(format!(
            "take rejected after {reason} and files restored, but rollback verification failed: {rollback_gate_error}"
        ));
    }

    Err(format!(
        "take rejected after {reason}; package SHA-256 {} is recorded as rejected",
        loaded.package_sha256
    ))
}

fn discover_candidate(
    project_root: &Path,
    current: &CurrentIdentity,
    rejected: &BTreeSet<String>,
) -> Result<PathBuf, String> {
    let mut search_dirs = vec![project_root.to_path_buf()];
    if let Some(downloads) = downloads_dir().filter(|downloads| downloads.as_path() != project_root) {
        search_dirs.push(downloads);
    }

    let mut applicable = Vec::new();
    for directory in search_dirs {
        let Ok(entries) = fs::read_dir(&directory) else {
            continue;
        };
        for entry in entries {
            let entry = entry.map_err(|error| {
                format!("cannot enumerate take directory {}: {error}", directory.display())
            })?;
            let path = entry.path();
            let Some(file_name) = path.file_name().and_then(OsStr::to_str) else {
                continue;
            };
            if !file_name.starts_with(TAKE_DISCOVERY_PREFIX)
                || !file_name.to_ascii_lowercase().ends_with(".zip")
            {
                continue;
            }

            let loaded = load_take(&path).map_err(|error| {
                format!("candidate with Take filename is malformed {}: {error}", path.display())
            })?;
            if rejected.contains(&loaded.package_sha256) {
                continue;
            }
            if loaded.manifest.base_version == current.version
                && loaded.manifest.base_revision == current.revision
            {
                applicable.push(path);
            }
        }
    }

    applicable.sort();
    applicable.dedup();
    match applicable.as_slice() {
        [] => Err(format!(
            "no applicable SchemaSchlopf take found for {}-{}",
            current.version, current.revision
        )),
        [only] => Ok(only.clone()),
        many => Err(format!(
            "ambiguous applicable takes for {}-{}: {}",
            current.version,
            current.revision,
            many.iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn downloads_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
        .map(|home| home.join("Downloads"))
}

fn load_take(path: &Path) -> Result<LoadedTake, String> {
    let package_bytes = fs::read(path)
        .map_err(|error| format!("cannot read take package {}: {error}", path.display()))?;
    let package_sha256 = sha256_hex(&package_bytes);
    let reader = std::io::Cursor::new(package_bytes);
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|error| format!("invalid ZIP {}: {error}", path.display()))?;

    let mut entries = BTreeMap::<String, Vec<u8>>::new();
    let mut total_payload_bytes = 0_u64;

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("cannot read ZIP entry {index}: {error}"))?;
        if entry.is_dir() {
            return Err(format!("directory entries are not allowed in take ZIP: {}", entry.name()));
        }
        if entry.is_symlink() {
            return Err(format!("symlink entries are not allowed in take ZIP: {}", entry.name()));
        }
        if entry.name().contains('\\') {
            return Err(format!("backslash path is not canonical in take ZIP: {}", entry.name()));
        }
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| format!("unsafe ZIP path: {}", entry.name()))?;
        let canonical = canonical_relative_path(&enclosed)?;
        if entry.size() > MAX_PAYLOAD_FILE_BYTES {
            return Err(format!("take entry exceeds size limit: {canonical}"));
        }
        if canonical.starts_with(TAKE_PAYLOAD_PREFIX) {
            total_payload_bytes = total_payload_bytes
                .checked_add(entry.size())
                .ok_or_else(|| "take payload size overflow".to_owned())?;
            if total_payload_bytes > MAX_TOTAL_PAYLOAD_BYTES {
                return Err("take payload exceeds total size limit".to_owned());
            }
        }
        let mut bytes = Vec::with_capacity(entry.size() as usize);
        entry
            .read_to_end(&mut bytes)
            .map_err(|error| format!("cannot read ZIP entry {canonical}: {error}"))?;
        if entries.insert(canonical.clone(), bytes).is_some() {
            return Err(format!("duplicate ZIP entry: {canonical}"));
        }
    }

    let manifest_bytes = entries
        .remove(TAKE_MANIFEST_NAME)
        .ok_or_else(|| format!("missing {TAKE_MANIFEST_NAME}"))?;
    let manifest_text = std::str::from_utf8(&manifest_bytes)
        .map_err(|_| format!("{TAKE_MANIFEST_NAME} must be UTF-8"))?;
    let manifest = parse_manifest(manifest_text)?;

    let mut payload = BTreeMap::new();
    let expected_paths: BTreeSet<String> = manifest
        .files
        .iter()
        .map(|file| format!("{TAKE_PAYLOAD_PREFIX}{}", file.path))
        .collect();
    let actual_paths: BTreeSet<String> = entries.keys().cloned().collect();
    if actual_paths != expected_paths {
        let unexpected = actual_paths.difference(&expected_paths).cloned().collect::<Vec<_>>();
        let missing = expected_paths.difference(&actual_paths).cloned().collect::<Vec<_>>();
        return Err(format!(
            "take ZIP tree does not match manifest; unexpected={unexpected:?}; missing={missing:?}"
        ));
    }

    for file in &manifest.files {
        let archive_path = format!("{TAKE_PAYLOAD_PREFIX}{}", file.path);
        let bytes = entries
            .remove(&archive_path)
            .ok_or_else(|| format!("payload disappeared after tree gate: {archive_path}"))?;
        let actual_sha256 = sha256_hex(&bytes);
        if actual_sha256 != file.target_sha256 {
            return Err(format!(
                "payload SHA-256 mismatch for {}: expected {}, got {}",
                file.path, file.target_sha256, actual_sha256
            ));
        }
        payload.insert(file.path.clone(), bytes);
    }

    Ok(LoadedTake {
        manifest,
        payload,
        package_sha256,
    })
}

fn parse_manifest(text: &str) -> Result<TakeManifest, String> {
    let mut scalars = BTreeMap::<String, String>::new();
    let mut files = Vec::new();
    let mut file_paths = BTreeSet::new();

    for (line_number, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=').ok_or_else(|| {
            format!("invalid take manifest line {}: missing '='", line_number + 1)
        })?;
        let key = key.trim();
        let value = value.trim();

        if key == "file" {
            let parts = value.split('|').collect::<Vec<_>>();
            if parts.len() != 3 {
                return Err(format!(
                    "invalid file declaration on manifest line {}",
                    line_number + 1
                ));
            }
            let path = parts[0].trim().to_owned();
            validate_project_relative_path(&path)?;
            if !file_paths.insert(path.clone()) {
                return Err(format!("duplicate file declaration: {path}"));
            }
            let target_sha256 = validate_sha256(parts[1].trim())?;
            let base_text = parts[2].trim();
            let base = if base_text == "ABSENT" {
                BaseState::Absent
            } else {
                BaseState::Sha256(validate_sha256(base_text)?)
            };
            files.push(TakeFile {
                path,
                target_sha256,
                base,
            });
            continue;
        }

        const KEYS: &[&str] = &[
            "format",
            "project",
            "base_version",
            "base_revision",
            "target_version",
            "target_revision",
            "name",
            "allow_dependency_change",
            "allow_normative_change",
        ];
        if !KEYS.contains(&key) {
            return Err(format!("unknown take manifest key: {key}"));
        }
        if scalars.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(format!("duplicate take manifest key: {key}"));
        }
    }

    require_scalar(&scalars, "format", TAKE_FORMAT)?;
    require_scalar(&scalars, "project", TAKE_PROJECT)?;
    let base_version = required_nonempty(&scalars, "base_version")?;
    let base_revision = required_nonempty(&scalars, "base_revision")?;
    let target_version = required_nonempty(&scalars, "target_version")?;
    let target_revision = required_nonempty(&scalars, "target_revision")?;
    let name = required_nonempty(&scalars, "name")?;
    let allow_dependency_change = parse_bool(&scalars, "allow_dependency_change")?;
    let allow_normative_change = parse_bool(&scalars, "allow_normative_change")?;

    if files.is_empty() {
        return Err("take manifest declares no files".to_owned());
    }
    if !file_paths.contains("VERSION") {
        return Err("every take must declare VERSION".to_owned());
    }
    if !file_paths.contains(SOURCE_MANIFEST_PATH) {
        return Err(format!("every take must declare {SOURCE_MANIFEST_PATH}"));
    }

    Ok(TakeManifest {
        base_version,
        base_revision,
        target_version,
        target_revision,
        name,
        allow_dependency_change,
        allow_normative_change,
        files,
    })
}

fn require_scalar(
    scalars: &BTreeMap<String, String>,
    key: &str,
    expected: &str,
) -> Result<(), String> {
    let actual = scalars
        .get(key)
        .ok_or_else(|| format!("missing take manifest key: {key}"))?;
    if actual != expected {
        return Err(format!(
            "unexpected take manifest {key}: expected {expected:?}, got {actual:?}"
        ));
    }
    Ok(())
}

fn required_nonempty(scalars: &BTreeMap<String, String>, key: &str) -> Result<String, String> {
    let value = scalars
        .get(key)
        .ok_or_else(|| format!("missing take manifest key: {key}"))?;
    if value.is_empty() {
        return Err(format!("take manifest key is empty: {key}"));
    }
    Ok(value.clone())
}

fn parse_bool(scalars: &BTreeMap<String, String>, key: &str) -> Result<bool, String> {
    match scalars
        .get(key)
        .ok_or_else(|| format!("missing take manifest key: {key}"))?
        .as_str()
    {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(format!("take manifest {key} must be true or false, got {other:?}")),
    }
}

fn verify_manifest_against_current(
    manifest: &TakeManifest,
    current: &CurrentIdentity,
) -> Result<(), String> {
    if manifest.base_version != current.version || manifest.base_revision != current.revision {
        return Err(format!(
            "take base {}-{} does not match current {}-{}",
            manifest.base_version, manifest.base_revision, current.version, current.revision
        ));
    }
    let base_revision = parse_revision_number(&manifest.base_revision)?;
    let target_revision = parse_revision_number(&manifest.target_revision)?;
    if target_revision != base_revision + 1 {
        return Err(format!(
            "take target revision must be exactly next revision: {} -> {}",
            manifest.base_revision, manifest.target_revision
        ));
    }
    Ok(())
}

fn verify_take_policy(manifest: &TakeManifest) -> Result<(), String> {
    let paths = manifest
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<BTreeSet<_>>();

    let dependency_touched = paths.contains("Cargo.toml") || paths.contains("Cargo.lock");
    if dependency_touched && !manifest.allow_dependency_change {
        return Err("take touches dependency surfaces without allow_dependency_change=true".to_owned());
    }
    if manifest.allow_dependency_change
        && !(paths.contains("Cargo.toml") && paths.contains("Cargo.lock"))
    {
        return Err(
            "dependency-changing take must bind both Cargo.toml and Cargo.lock".to_owned(),
        );
    }

    let normative_touched = paths
        .iter()
        .any(|path| path.starts_with("docs/normative_sources/"));
    if normative_touched && !manifest.allow_normative_change {
        return Err("take touches normative sources without allow_normative_change=true".to_owned());
    }

    Ok(())
}

fn verify_base_states(project_root: &Path, manifest: &TakeManifest) -> Result<(), String> {
    for file in &manifest.files {
        let path = project_root.join(&file.path);
        reject_symlink_chain(project_root, &file.path)?;
        match &file.base {
            BaseState::Absent => {
                if path.exists() {
                    return Err(format!("take expected base file to be absent: {}", file.path));
                }
            }
            BaseState::Sha256(expected) => {
                let bytes = fs::read(&path)
                    .map_err(|error| format!("cannot read base file {}: {error}", file.path))?;
                let actual = sha256_hex(&bytes);
                if &actual != expected {
                    return Err(format!(
                        "base SHA-256 mismatch for {}: expected {}, got {}",
                        file.path, expected, actual
                    ));
                }
            }
        }
    }
    Ok(())
}

fn install_payload(project_root: &Path, loaded: &LoadedTake) -> Result<Vec<BackupEntry>, String> {
    let mut backups = Vec::new();
    for file in &loaded.manifest.files {
        let target = project_root.join(&file.path);
        reject_symlink_chain(project_root, &file.path)?;
        let previous = if target.exists() {
            Some(
                fs::read(&target)
                    .map_err(|error| format!("cannot back up {}: {error}", file.path))?,
            )
        } else {
            None
        };
        backups.push(BackupEntry {
            path: file.path.clone(),
            previous,
        });
    }

    let install_result = (|| -> Result<(), String> {
        for file in &loaded.manifest.files {
            let bytes = loaded
                .payload
                .get(&file.path)
                .ok_or_else(|| format!("missing verified payload for {}", file.path))?;
            write_file_replace(project_root, &file.path, bytes)?;
        }

        for file in &loaded.manifest.files {
            let bytes = fs::read(project_root.join(&file.path))
                .map_err(|error| format!("cannot verify installed {}: {error}", file.path))?;
            let actual = sha256_hex(&bytes);
            if actual != file.target_sha256 {
                return Err(format!(
                    "installed SHA-256 mismatch for {}: expected {}, got {}",
                    file.path, file.target_sha256, actual
                ));
            }
        }

        let installed = read_current_identity(project_root)?;
        if installed.version != loaded.manifest.target_version
            || installed.revision != loaded.manifest.target_revision
        {
            return Err(format!(
                "installed VERSION does not match take target: expected {}-{}, got {}-{}",
                loaded.manifest.target_version,
                loaded.manifest.target_revision,
                installed.version,
                installed.revision
            ));
        }
        Ok(())
    })();

    if let Err(error) = install_result {
        rollback(project_root, &backups).map_err(|rollback_error| {
            format!("installation failed ({error}); rollback also failed: {rollback_error}")
        })?;
        return Err(format!("installation failed and was rolled back: {error}"));
    }

    Ok(backups)
}

fn write_file_replace(project_root: &Path, relative: &str, bytes: &[u8]) -> Result<(), String> {
    let target = project_root.join(relative);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create parent for {relative}: {error}"))?;
    }
    let file_name = target
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("invalid target filename: {relative}"))?;
    let temp = target.with_file_name(format!(
        ".{file_name}.schemaschlopf-new-{}",
        std::process::id()
    ));
    if temp.exists() {
        fs::remove_file(&temp)
            .map_err(|error| format!("cannot remove stale temp file {}: {error}", temp.display()))?;
    }
    {
        let mut handle = File::create(&temp)
            .map_err(|error| format!("cannot create temp file {}: {error}", temp.display()))?;
        handle
            .write_all(bytes)
            .map_err(|error| format!("cannot write temp file {}: {error}", temp.display()))?;
        handle
            .sync_all()
            .map_err(|error| format!("cannot sync temp file {}: {error}", temp.display()))?;
    }
    if target.exists() {
        fs::remove_file(&target)
            .map_err(|error| format!("cannot replace {relative}: {error}"))?;
    }
    fs::rename(&temp, &target)
        .map_err(|error| format!("cannot install {relative}: {error}"))?;
    Ok(())
}

fn rollback(project_root: &Path, backups: &[BackupEntry]) -> Result<(), String> {
    for backup in backups.iter().rev() {
        let target = project_root.join(&backup.path);
        match &backup.previous {
            Some(bytes) => write_file_replace(project_root, &backup.path, bytes)?,
            None => {
                if target.exists() {
                    fs::remove_file(&target).map_err(|error| {
                        format!("cannot remove newly-added file {} during rollback: {error}", backup.path)
                    })?;
                }
            }
        }
    }
    Ok(())
}

fn run_post_install_gates(project_root: &Path) -> Result<(), String> {
    println!("[SchemaSchlopf take] TREE ...");
    verify_source_manifest(project_root)?;
    run_cargo_gate(project_root, "TEST-CORE", &["test", "--locked"])?;
    run_cargo_gate(
        project_root,
        "TEST-ALL-FEATURES",
        &["test", "--locked", "--all-features"],
    )?;
    run_cargo_gate(
        project_root,
        "CLIPPY",
        &[
            "clippy",
            "--locked",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_cargo_gate(
        project_root,
        "RUN",
        &[
            "run",
            "--locked",
            "--release",
            "--bin",
            "schemaschlopf",
        ],
    )?;
    Ok(())
}

fn run_cargo_gate(project_root: &Path, label: &str, args: &[&str]) -> Result<(), String> {
    println!("[SchemaSchlopf take] {label} ...");
    let status = Command::new("cargo")
        .args(args)
        .current_dir(project_root)
        .status()
        .map_err(|error| format!("cannot start cargo for {label}: {error}"))?;
    if !status.success() {
        return Err(format!("{label} failed with exit status {status}"));
    }
    Ok(())
}

fn read_current_identity(project_root: &Path) -> Result<CurrentIdentity, String> {
    let text = fs::read_to_string(project_root.join("VERSION"))
        .map_err(|error| format!("cannot read VERSION: {error}"))?;
    let mut version = None;
    let mut revision = None;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("version=") {
            version = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("revision=") {
            revision = Some(value.trim().to_owned());
        }
    }
    let version = version.filter(|value| !value.is_empty()).ok_or("VERSION has no version=")?;
    let revision = revision
        .filter(|value| !value.is_empty())
        .ok_or("VERSION has no revision=")?;
    parse_revision_number(&revision)?;
    Ok(CurrentIdentity { version, revision })
}

fn parse_revision_number(revision: &str) -> Result<u32, String> {
    let digits = revision
        .strip_prefix('r')
        .ok_or_else(|| format!("invalid revision: {revision}"))?;
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("invalid revision: {revision}"));
    }
    digits
        .parse::<u32>()
        .map_err(|_| format!("revision out of range: {revision}"))
}

fn validate_project_relative_path(path: &str) -> Result<(), String> {
    if path.is_empty() || path.contains('\\') || path.starts_with('/') || path.contains(':') {
        return Err(format!("invalid project-relative take path: {path:?}"));
    }
    let canonical = canonical_relative_path(Path::new(path))?;
    if canonical != path {
        return Err(format!("non-canonical take path: {path:?}"));
    }
    let top_level_allowed = matches!(
        path,
        "Cargo.toml"
            | "Cargo.lock"
            | "README.md"
            | "VERSION"
            | "run.cmd"
            | ".gitignore"
            | "SOURCE_MANIFEST.sha256"
            | "BUILD_STATUS.md"
            | "PACKAGE_KIND.txt"
            | "BOOTSTRAP_CONSTRUCTION_EVIDENCE.txt"
    ) || path.starts_with("src/")
        || path.starts_with("docs/")
        || path.starts_with("tests/")
        || path.starts_with("tools/");
    if !top_level_allowed {
        return Err(format!("take path is outside allowed project tree: {path}"));
    }
    Ok(())
}

fn canonical_relative_path(path: &Path) -> Result<String, String> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let text = part
                    .to_str()
                    .ok_or_else(|| "take path must be UTF-8".to_owned())?;
                if text.is_empty() {
                    return Err("take path contains empty component".to_owned());
                }
                parts.push(text);
            }
            _ => return Err(format!("unsafe/non-canonical take path: {}", path.display())),
        }
    }
    if parts.is_empty() {
        return Err("empty take path".to_owned());
    }
    Ok(parts.join("/"))
}

fn reject_symlink_chain(project_root: &Path, relative: &str) -> Result<(), String> {
    let mut current = project_root.to_path_buf();
    let path = Path::new(relative);
    for component in path.components() {
        let Component::Normal(part) = component else {
            return Err(format!("unsafe project path while checking symlinks: {relative}"));
        };
        current.push(part);
        let Ok(metadata) = fs::symlink_metadata(&current) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            return Err(format!("symlink project path is not allowed: {}", current.display()));
        }
        if current == project_root.join(relative) && !metadata.is_file() {
            return Err(format!("project target is not a regular file: {}", current.display()));
        }
        if current != project_root.join(relative) && !metadata.is_dir() {
            return Err(format!("project parent is not a directory: {}", current.display()));
        }
    }
    Ok(())
}

pub fn seal_source_manifest(project_root: &Path) -> Result<String, String> {
    let project_root = project_root
        .canonicalize()
        .map_err(|error| format!("project root cannot be resolved: {error}"))?;
    let paths = managed_source_paths(&project_root)?;
    if !paths.contains("Cargo.lock") {
        return Err("cannot seal GreenBase without Cargo.lock".to_owned());
    }

    let mut output = String::new();
    for relative in paths {
        let bytes = fs::read(project_root.join(&relative))
            .map_err(|error| format!("cannot read managed source {relative}: {error}"))?;
        output.push_str(&sha256_hex(&bytes));
        output.push_str("  ");
        output.push_str(&relative);
        output.push('\n');
    }
    fs::write(project_root.join(SOURCE_MANIFEST_PATH), output.as_bytes())
        .map_err(|error| format!("cannot write {SOURCE_MANIFEST_PATH}: {error}"))?;
    verify_source_manifest(&project_root)?;
    Ok(sha256_hex(output.as_bytes()))
}

pub(crate) fn verify_source_manifest(project_root: &Path) -> Result<(), String> {
    let manifest_path = project_root.join(SOURCE_MANIFEST_PATH);
    let text = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("cannot read {SOURCE_MANIFEST_PATH}: {error}"))?;
    let declared = parse_source_manifest(&text)?;
    let actual_paths = managed_source_paths(project_root)?;
    let declared_paths = declared.keys().cloned().collect::<BTreeSet<_>>();
    if declared_paths != actual_paths {
        let unexpected = actual_paths
            .difference(&declared_paths)
            .cloned()
            .collect::<Vec<_>>();
        let missing = declared_paths
            .difference(&actual_paths)
            .cloned()
            .collect::<Vec<_>>();
        return Err(format!(
            "GreenBase tree differs from {SOURCE_MANIFEST_PATH}; unexpected={unexpected:?}; missing={missing:?}"
        ));
    }

    for (relative, expected) in declared {
        let bytes = fs::read(project_root.join(&relative))
            .map_err(|error| format!("cannot read managed source {relative}: {error}"))?;
        let actual = sha256_hex(&bytes);
        if actual != expected {
            return Err(format!(
                "GreenBase SHA-256 mismatch for {relative}: expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

pub(crate) fn parse_source_manifest(text: &str) -> Result<BTreeMap<String, String>, String> {
    let mut entries = BTreeMap::new();
    for (line_number, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (hash, relative) = line.split_once("  ").ok_or_else(|| {
            format!(
                "invalid {SOURCE_MANIFEST_PATH} line {}: expected '<sha256>  <path>'",
                line_number + 1
            )
        })?;
        let hash = validate_sha256(hash)?;
        validate_project_relative_path(relative)?;
        if relative == SOURCE_MANIFEST_PATH {
            return Err(format!("{SOURCE_MANIFEST_PATH} must not hash itself"));
        }
        if entries.insert(relative.to_owned(), hash).is_some() {
            return Err(format!("duplicate {SOURCE_MANIFEST_PATH} path: {relative}"));
        }
    }
    if entries.is_empty() {
        return Err(format!("{SOURCE_MANIFEST_PATH} is empty"));
    }
    Ok(entries)
}

fn managed_source_paths(project_root: &Path) -> Result<BTreeSet<String>, String> {
    let mut paths = BTreeSet::new();
    const TOP_LEVEL: &[&str] = &[
        ".gitignore",
        "BOOTSTRAP_CONSTRUCTION_EVIDENCE.txt",
        "BUILD_STATUS.md",
        "Cargo.lock",
        "Cargo.toml",
        "PACKAGE_KIND.txt",
        "README.md",
        "VERSION",
        "run.cmd",
    ];
    const MANAGED_DIRS: &[&str] = &["src", "docs", "tests", "tools"];
    const LOCAL_DIRS: &[&str] = &["target", ".schemaschlopf", ".git"];

    let root_entries = fs::read_dir(project_root)
        .map_err(|error| format!("cannot enumerate project root: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate project root: {error}"))?;
    for entry in root_entries {
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| "project root entry name must be UTF-8".to_owned())?
            .to_owned();
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|error| format!("cannot inspect project root entry {name}: {error}"))?;
        if metadata.file_type().is_symlink() {
            return Err(format!("symlink at project root is not allowed: {name}"));
        }
        if metadata.is_dir() {
            if !MANAGED_DIRS.contains(&name.as_str()) && !LOCAL_DIRS.contains(&name.as_str()) {
                return Err(format!("unknown top-level directory: {name}"));
            }
        } else if metadata.is_file() {
            let is_take_package = name.starts_with(TAKE_DISCOVERY_PREFIX)
                && name.to_ascii_lowercase().ends_with(".zip");
            let is_assistant_export = is_local_generated_root_file(&name);
            if !TOP_LEVEL.contains(&name.as_str())
                && name != SOURCE_MANIFEST_PATH
                && !is_take_package
                && !is_assistant_export
            {
                return Err(format!("unknown top-level file: {name}"));
            }
        } else {
            return Err(format!("unsupported top-level filesystem entry: {name}"));
        }
    }
    for relative in TOP_LEVEL {
        let path = project_root.join(relative);
        if path.exists() {
            reject_symlink_chain(project_root, relative)?;
            paths.insert((*relative).to_owned());
        }
    }
    for &directory in MANAGED_DIRS {
        let root = project_root.join(directory);
        if root.exists() {
            collect_managed_files(project_root, &root, &mut paths)?;
        }
    }
    Ok(paths)
}


fn is_local_generated_root_file(name: &str) -> bool {
    name == EXPORT_ZIP_NAME
}

fn collect_managed_files(
    project_root: &Path,
    directory: &Path,
    paths: &mut BTreeSet<String>,
) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot enumerate managed directory {}: {error}", directory.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate managed directory {}: {error}", directory.display()))?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("cannot inspect managed path {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() {
            return Err(format!("symlink inside managed source tree: {}", path.display()));
        }
        if metadata.is_dir() {
            collect_managed_files(project_root, &path, paths)?;
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(project_root)
                .map_err(|_| format!("managed path escaped project root: {}", path.display()))?;
            let relative = canonical_relative_path(relative)?;
            validate_project_relative_path(&relative)?;
            if relative != SOURCE_MANIFEST_PATH {
                paths.insert(relative);
            }
        } else {
            return Err(format!("unsupported managed filesystem entry: {}", path.display()));
        }
    }
    Ok(())
}

fn validate_sha256(value: &str) -> Result<String, String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(format!("invalid lowercase SHA-256: {value:?}"));
    }
    Ok(value.to_owned())
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

fn read_rejected_hashes(project_root: &Path) -> Result<BTreeSet<String>, String> {
    let path = project_root.join(".schemaschlopf/rejected_take_sha256.txt");
    if !path.exists() {
        return Ok(BTreeSet::new());
    }
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read rejection registry {}: {error}", path.display()))?;
    let mut hashes = BTreeSet::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        hashes.insert(validate_sha256(line)?);
    }
    Ok(hashes)
}

fn append_rejected_hash(project_root: &Path, hash: &str) -> Result<(), String> {
    validate_sha256(hash)?;
    let directory = project_root.join(".schemaschlopf");
    fs::create_dir_all(&directory)
        .map_err(|error| format!("cannot create rejection registry directory: {error}"))?;
    let path = directory.join("rejected_take_sha256.txt");
    let mut existing = read_rejected_hashes(project_root)?;
    if existing.insert(hash.to_owned()) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|error| format!("cannot open rejection registry: {error}"))?;
        writeln!(file, "{hash}").map_err(|error| format!("cannot write rejection registry: {error}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

    fn valid_manifest(extra: &str) -> String {
        format!(
            "format={TAKE_FORMAT}\n\
project={TAKE_PROJECT}\n\
base_version=0.1.0\n\
base_revision=r003\n\
target_version=0.1.0\n\
target_revision=r004\n\
name=SimpleTypeValidationFoundation\n\
allow_dependency_change=false\n\
allow_normative_change=false\n\
file=VERSION|{A}|{B}\n\
file=SOURCE_MANIFEST.sha256|{A}|{B}\n{extra}"
        )
    }

    #[test]
    fn manifest_parses_strict_valid_take() {
        let manifest = parse_manifest(&valid_manifest("")).expect("valid manifest");
        assert_eq!(manifest.base_revision, "r003");
        assert_eq!(manifest.target_revision, "r004");
        assert_eq!(manifest.files.len(), 2);
    }

    #[test]
    fn manifest_rejects_unknown_key() {
        let error = parse_manifest(&valid_manifest("surprise=yes\n")).expect_err("must reject");
        assert!(error.contains("unknown take manifest key"));
    }

    #[test]
    fn path_gate_rejects_traversal_and_unknown_top_level() {
        assert!(validate_project_relative_path("../src/lib.rs").is_err());
        assert!(validate_project_relative_path("mystery/file.rs").is_err());
        assert!(validate_project_relative_path("src/lib.rs").is_ok());
    }

    #[test]
    fn dependency_surface_requires_explicit_pair_and_authority() {
        let mut manifest = parse_manifest(&valid_manifest(&format!(
            "file=Cargo.toml|{A}|{B}\n"
        )))
        .expect("manifest parses before policy");
        assert!(verify_take_policy(&manifest).is_err());

        manifest.allow_dependency_change = true;
        assert!(verify_take_policy(&manifest).is_err());

        manifest.files.push(TakeFile {
            path: "Cargo.lock".to_owned(),
            target_sha256: A.to_owned(),
            base: BaseState::Sha256(B.to_owned()),
        });
        assert!(verify_take_policy(&manifest).is_ok());
    }

    #[test]
    fn normative_sources_require_explicit_authority() {
        let mut manifest = parse_manifest(&valid_manifest(&format!(
            "file=docs/normative_sources/musicxml/3.1/schema/musicxml.xsd|{A}|{B}\n"
        )))
        .expect("manifest parses before policy");
        assert!(verify_take_policy(&manifest).is_err());
        manifest.allow_normative_change = true;
        assert!(verify_take_policy(&manifest).is_ok());
    }

    #[test]
    fn take_must_advance_exactly_one_revision() {
        let current = CurrentIdentity {
            version: "0.1.0".to_owned(),
            revision: "r003".to_owned(),
        };
        let manifest = parse_manifest(&valid_manifest("")).expect("valid manifest");
        assert!(verify_manifest_against_current(&manifest, &current).is_ok());

        let mut skipped = manifest.clone();
        skipped.target_revision = "r005".to_owned();
        assert!(verify_manifest_against_current(&skipped, &current).is_err());
    }

    #[test]
    fn take_zip_reader_accepts_exact_declared_tree() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/take_valid.zip");
        let loaded = load_take(&path).expect("fixture take must load");
        assert_eq!(loaded.manifest.base_revision, "r003");
        assert_eq!(loaded.manifest.target_revision, "r004");
        assert_eq!(loaded.payload.len(), 2);
        assert!(loaded.payload.contains_key("VERSION"));
        assert!(loaded.payload.contains_key(SOURCE_MANIFEST_PATH));
    }

    #[test]
    fn take_zip_reader_rejects_undeclared_extra_payload() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/take_undeclared_extra.zip");
        let error = load_take(&path).expect_err("extra payload must fail closed");
        assert!(error.contains("tree does not match manifest"));
    }

    #[test]
    fn take_zip_reader_rejects_traversal_entry() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/take_traversal.zip");
        let error = load_take(&path).expect_err("traversal must fail closed");
        assert!(error.contains("unsafe ZIP path") || error.contains("unsafe/non-canonical"));
    }


    #[test]
    fn only_stable_assistant_export_name_is_excluded_from_source_authority() {
        assert!(is_local_generated_root_file("assistant_export_latest.zip"));
        assert!(!is_local_generated_root_file("assistant_export_old.zip"));
        assert!(!is_local_generated_root_file("arbitrary.zip"));
    }

    #[test]
    fn source_manifest_rejects_self_hash_and_duplicate_paths() {
        let self_hash = format!("{A}  {SOURCE_MANIFEST_PATH}\n");
        assert!(parse_source_manifest(&self_hash).is_err());

        let duplicate = format!("{A}  VERSION\n{B}  VERSION\n");
        assert!(parse_source_manifest(&duplicate).is_err());
    }

}
