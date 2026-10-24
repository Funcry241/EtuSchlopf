//! Read-only assistant export for a sealed SchemaSchlopf GreenBase.
//!
//! The export is convenience/provenance infrastructure only. It creates no
//! schema or instance-validation authority. The stable latest ZIP is written
//! directly to the project root and is explicitly excluded from source authority.

use crate::take::{
    SOURCE_MANIFEST_PATH, parse_source_manifest, sha256_hex, verify_source_manifest,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

pub(crate) const EXPORT_ZIP_NAME: &str = "assistant_export_latest.zip";
const HANDOFF_NAME: &str = "assistant_handoff_latest.txt";
const EXPORT_MANIFEST_NAME: &str = "EXPORT_MANIFEST.sha256";
const EXPORT_PROJECT_PREFIX: &str = "SchemaSchlopf/";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistantExportSummary {
    pub zip_path: PathBuf,
    pub zip_sha256: String,
    pub source_manifest_sha256: String,
    pub source_file_count: usize,
}

pub fn run_assistant_export(project_root: &Path) -> Result<AssistantExportSummary, String> {
    let project_root = project_root
        .canonicalize()
        .map_err(|error| format!("project root cannot be resolved: {error}"))?;
    verify_source_manifest(&project_root)?;

    let source_manifest_bytes = fs::read(project_root.join(SOURCE_MANIFEST_PATH))
        .map_err(|error| format!("cannot read {SOURCE_MANIFEST_PATH}: {error}"))?;
    let source_manifest_text = std::str::from_utf8(&source_manifest_bytes)
        .map_err(|_| format!("{SOURCE_MANIFEST_PATH} must be UTF-8"))?;
    let declared = parse_source_manifest(source_manifest_text)?;
    let source_manifest_sha256 = sha256_hex(&source_manifest_bytes);

    let (version, revision, name) = read_version_identity(&project_root)?;
    let handoff = render_handoff(
        &version,
        &revision,
        &name,
        &source_manifest_sha256,
        declared.len(),
        crate::INSTANCE_VALIDATION_QUALIFIED,
    );

    let mut entries = BTreeMap::<String, Vec<u8>>::new();
    entries.insert(HANDOFF_NAME.to_owned(), handoff.as_bytes().to_vec());
    entries.insert(
        format!("{EXPORT_PROJECT_PREFIX}{SOURCE_MANIFEST_PATH}"),
        source_manifest_bytes,
    );

    for (relative, expected_sha256) in &declared {
        let bytes = fs::read(project_root.join(relative))
            .map_err(|error| format!("cannot read sealed source {relative}: {error}"))?;
        let actual = sha256_hex(&bytes);
        if &actual != expected_sha256 {
            return Err(format!(
                "sealed source changed during export for {relative}: expected {expected_sha256}, got {actual}"
            ));
        }
        entries.insert(format!("{EXPORT_PROJECT_PREFIX}{relative}"), bytes);
    }

    let export_manifest = render_export_manifest(&entries);
    entries.insert(
        EXPORT_MANIFEST_NAME.to_owned(),
        export_manifest.as_bytes().to_vec(),
    );

    let zip_bytes = build_zip(&entries)?;
    verify_export_zip(&zip_bytes, &entries)?;

    let zip_path = export_zip_path(&project_root);
    replace_preserving_previous(&zip_path, &zip_bytes)?;

    Ok(AssistantExportSummary {
        zip_path,
        zip_sha256: sha256_hex(&zip_bytes),
        source_manifest_sha256,
        source_file_count: declared.len(),
    })
}

fn export_zip_path(project_root: &Path) -> PathBuf {
    project_root.join(EXPORT_ZIP_NAME)
}

fn render_handoff(
    version: &str,
    revision: &str,
    name: &str,
    source_manifest_sha256: &str,
    source_file_count: usize,
    instance_validation_qualified: bool,
) -> String {
    format!(
        "SchemaSchlopf Assistant Handoff\n\
project=SchemaSchlopf\n\
version={version}\n\
revision={revision}\n\
name={name}\n\
source_tree_seal_verified=yes\n\
source_manifest_sha256={source_manifest_sha256}\n\
sealed_source_files={source_file_count}\n\
export_authority=read-only-copy\n\
instance_validation_qualified={instance_validation_qualified}\n\
instance_validation_authority_created=no\n\
\n\
Start with SchemaSchlopf/VERSION and SchemaSchlopf/docs/assistant_memory/000_start_here.md.\n\
Repository files and bound normative source bytes outrank this handoff text.\n"
    )
}

fn render_export_manifest(entries: &BTreeMap<String, Vec<u8>>) -> String {
    let mut output = String::new();
    for (path, bytes) in entries {
        output.push_str(&sha256_hex(bytes));
        output.push_str("  ");
        output.push_str(path);
        output.push('\n');
    }
    output
}

fn build_zip(entries: &BTreeMap<String, Vec<u8>>) -> Result<Vec<u8>, String> {
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = ZipWriter::new(&mut cursor);
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        for (path, bytes) in entries {
            writer
                .start_file(path, options)
                .map_err(|error| format!("cannot start export ZIP entry {path}: {error}"))?;
            writer
                .write_all(bytes)
                .map_err(|error| format!("cannot write export ZIP entry {path}: {error}"))?;
        }
        writer
            .finish()
            .map_err(|error| format!("cannot finish assistant export ZIP: {error}"))?;
    }
    Ok(cursor.into_inner())
}

fn verify_export_zip(
    zip_bytes: &[u8],
    expected_entries: &BTreeMap<String, Vec<u8>>,
) -> Result<(), String> {
    let mut archive = ZipArchive::new(Cursor::new(zip_bytes))
        .map_err(|error| format!("cannot reopen assistant export ZIP: {error}"))?;
    let mut seen = BTreeSet::new();

    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("cannot read assistant export ZIP entry {index}: {error}"))?;
        if entry.is_dir() || entry.is_symlink() {
            return Err(format!("assistant export contains non-file entry: {}", entry.name()));
        }
        let name = entry.name().to_owned();
        if !seen.insert(name.clone()) {
            return Err(format!("assistant export contains duplicate entry: {name}"));
        }
        let expected = expected_entries
            .get(&name)
            .ok_or_else(|| format!("assistant export contains undeclared entry: {name}"))?;
        let mut actual = Vec::new();
        entry
            .read_to_end(&mut actual)
            .map_err(|error| format!("cannot read assistant export entry {name}: {error}"))?;
        if &actual != expected {
            return Err(format!("assistant export read-back mismatch for {name}"));
        }
    }

    let expected_names = expected_entries.keys().cloned().collect::<BTreeSet<_>>();
    if seen != expected_names {
        return Err("assistant export ZIP tree differs from expected export tree".to_owned());
    }

    let manifest_bytes = expected_entries
        .get(EXPORT_MANIFEST_NAME)
        .ok_or_else(|| format!("assistant export lacks {EXPORT_MANIFEST_NAME}"))?;
    let manifest_text = std::str::from_utf8(manifest_bytes)
        .map_err(|_| format!("{EXPORT_MANIFEST_NAME} must be UTF-8"))?;
    verify_export_manifest(manifest_text, expected_entries)?;
    Ok(())
}

fn verify_export_manifest(
    manifest_text: &str,
    entries: &BTreeMap<String, Vec<u8>>,
) -> Result<(), String> {
    let mut declared = BTreeMap::new();
    for (line_number, line) in manifest_text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (hash, path) = line.split_once("  ").ok_or_else(|| {
            format!("invalid {EXPORT_MANIFEST_NAME} line {}", line_number + 1)
        })?;
        if path == EXPORT_MANIFEST_NAME {
            return Err(format!("{EXPORT_MANIFEST_NAME} must not hash itself"));
        }
        if hash.len() != 64
            || !hash
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(format!("invalid export SHA-256 for {path}"));
        }
        if declared.insert(path.to_owned(), hash.to_owned()).is_some() {
            return Err(format!("duplicate export manifest path: {path}"));
        }
    }

    let expected_paths = entries
        .keys()
        .filter(|path| path.as_str() != EXPORT_MANIFEST_NAME)
        .cloned()
        .collect::<BTreeSet<_>>();
    let declared_paths = declared.keys().cloned().collect::<BTreeSet<_>>();
    if expected_paths != declared_paths {
        return Err("assistant export manifest tree mismatch".to_owned());
    }

    for (path, expected_hash) in declared {
        let bytes = entries
            .get(&path)
            .ok_or_else(|| format!("export manifest references missing entry: {path}"))?;
        let actual = sha256_hex(bytes);
        if actual != expected_hash {
            return Err(format!(
                "export manifest SHA-256 mismatch for {path}: expected {expected_hash}, got {actual}"
            ));
        }
    }
    Ok(())
}

fn read_version_identity(project_root: &Path) -> Result<(String, String, String), String> {
    let text = fs::read_to_string(project_root.join("VERSION"))
        .map_err(|error| format!("cannot read VERSION: {error}"))?;
    let mut version = None;
    let mut revision = None;
    let mut name = None;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("version=") {
            version = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("revision=") {
            revision = Some(value.trim().to_owned());
        } else if let Some(value) = line.strip_prefix("name=") {
            name = Some(value.trim().to_owned());
        }
    }
    let require = |label: &str, value: Option<String>| {
        value
            .filter(|value| !value.is_empty())
            .ok_or_else(|| format!("VERSION has no {label}="))
    };
    Ok((
        require("version", version)?,
        require("revision", revision)?,
        require("name", name)?,
    ))
}

fn replace_preserving_previous(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("invalid export filename: {}", path.display()))?;

    let previous = if path.exists() {
        let metadata = fs::symlink_metadata(path)
            .map_err(|error| format!("cannot inspect existing export {}: {error}", path.display()))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(format!("assistant export target must be a regular file: {}", path.display()));
        }
        Some(
            fs::read(path)
                .map_err(|error| format!("cannot preserve existing export {}: {error}", path.display()))?,
        )
    } else {
        None
    };

    let temp = path.with_file_name(format!(".{file_name}.tmp-{}", std::process::id()));
    if temp.exists() {
        fs::remove_file(&temp)
            .map_err(|error| format!("cannot remove stale export temp {}: {error}", temp.display()))?;
    }
    fs::write(&temp, bytes)
        .map_err(|error| format!("cannot write export temp {}: {error}", temp.display()))?;

    if path.exists() {
        fs::remove_file(path)
            .map_err(|error| format!("cannot replace export {}: {error}", path.display()))?;
    }

    if let Err(error) = fs::rename(&temp, path) {
        let _ = fs::remove_file(&temp);
        if let Some(previous) = previous {
            let _ = fs::write(path, previous);
        }
        return Err(format!("cannot install export {}: {error}", path.display()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handoff_reflects_but_does_not_create_instance_validation_authority() {
        let text = render_handoff(
            "0.1.0",
            "r020",
            "InstanceValidationClosure",
            "abc",
            42,
            true,
        );
        assert!(text.contains("source_tree_seal_verified=yes"));
        assert!(text.contains("instance_validation_qualified=true"));
        assert!(text.contains("instance_validation_authority_created=no"));
        assert!(!text.contains("instance_validation_authority_created=yes"));
    }

    #[test]
    fn export_manifest_roundtrip_is_exact_and_does_not_self_hash() {
        let mut entries = BTreeMap::new();
        entries.insert("SchemaSchlopf/VERSION".to_owned(), b"version=0.1.0\n".to_vec());
        entries.insert(HANDOFF_NAME.to_owned(), b"handoff\n".to_vec());
        let manifest = render_export_manifest(&entries);
        verify_export_manifest(&manifest, &entries).expect("manifest must verify");
        assert!(!manifest.contains(EXPORT_MANIFEST_NAME));
    }

    #[test]
    fn export_zip_readback_rejects_no_declared_bytes() {
        let mut entries = BTreeMap::new();
        entries.insert(HANDOFF_NAME.to_owned(), b"handoff\n".to_vec());
        let manifest = render_export_manifest(&entries);
        entries.insert(EXPORT_MANIFEST_NAME.to_owned(), manifest.into_bytes());
        let zip = build_zip(&entries).expect("zip must build");
        verify_export_zip(&zip, &entries).expect("zip must read back exactly");
    }

    #[test]
    fn latest_export_path_is_project_root_stable_name() {
        let root = Path::new("C:/SchemaSchlopf/greenbase");
        assert_eq!(export_zip_path(root), root.join("assistant_export_latest.zip"));
    }
}
