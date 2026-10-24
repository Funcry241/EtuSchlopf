use crate::sha256;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const SOURCE_FINGERPRINT_SCOPE: &str = "generator_source_snapshot_v1";
const PIPELINE_CLOSURE_STATE: &str =
    "final_report_generation_after_final_notation_layout_snapshot_close_and_renderer";
const REQUIRED_ROOT_FILES: [&str; 4] = [
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    "ZIP_KIND.txt",
];
const OPTIONAL_ROOT_FILES: [&str; 2] = ["rustfmt.toml", "clippy.toml"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorSnapshotIdentity {
    pub software_version: String,
    pub revision: Option<String>,
    pub take_name: Option<String>,
    pub source_file_count: usize,
    pub source_fingerprint_sha256: Option<String>,
    pub executable_sha256: Option<String>,
    pub generator_identity_sha256: Option<String>,
    pub qualified: bool,
    pub reason: String,
}

impl GeneratorSnapshotIdentity {
    pub fn revision_bound(&self) -> bool {
        self.revision.is_some()
    }

    pub fn source_fingerprint_bound(&self) -> bool {
        self.source_fingerprint_sha256.is_some()
    }

    pub fn executable_fingerprint_bound(&self) -> bool {
        self.executable_sha256.is_some()
    }
}

static CURRENT_IDENTITY: OnceLock<GeneratorSnapshotIdentity> = OnceLock::new();

pub fn current() -> GeneratorSnapshotIdentity {
    CURRENT_IDENTITY
        .get_or_init(|| {
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            match std::env::current_exe() {
                Ok(executable) => build_for_paths(
                    &root,
                    &executable,
                    env!("CARGO_PKG_VERSION"),
                ),
                Err(error) => unqualified(
                    env!("CARGO_PKG_VERSION"),
                    format!("current_executable_unavailable:{error}"),
                ),
            }
        })
        .clone()
}

pub fn pipeline_closure_state() -> &'static str {
    PIPELINE_CLOSURE_STATE
}

pub fn source_fingerprint_scope() -> &'static str {
    SOURCE_FINGERPRINT_SCOPE
}

fn build_for_paths(
    root: &Path,
    executable: &Path,
    expected_version: &str,
) -> GeneratorSnapshotIdentity {
    let metadata_text = match fs::read_to_string(root.join("ZIP_KIND.txt")) {
        Ok(text) => text,
        Err(error) => {
            return unqualified(
                expected_version,
                format!("take_metadata_unavailable:{error}"),
            );
        }
    };
    let metadata_version = metadata_value(&metadata_text, "version");
    let revision = metadata_value(&metadata_text, "revision");
    let take_name = metadata_value(&metadata_text, "name");
    let kind = metadata_value(&metadata_text, "kind");
    if kind.as_deref() != Some("take") {
        return unqualified(expected_version, "take_metadata_kind_not_take".to_string());
    }
    if metadata_version.as_deref() != Some(expected_version) {
        return unqualified(expected_version, "take_metadata_version_mismatch".to_string());
    }
    if !revision.as_deref().is_some_and(valid_revision) {
        return unqualified(expected_version, "take_metadata_revision_invalid".to_string());
    }
    if take_name.as_deref().is_none_or(str::is_empty) {
        return unqualified(expected_version, "take_metadata_name_missing".to_string());
    }

    let source_files = match collect_source_files(root) {
        Ok(files) => files,
        Err(error) => return unqualified(expected_version, error),
    };
    let source_fingerprint_sha256 = match hash_source_files(root, &source_files) {
        Ok(hash) => hash,
        Err(error) => return unqualified(expected_version, error),
    };
    let executable_bytes = match fs::read(executable) {
        Ok(bytes) => bytes,
        Err(error) => {
            return unqualified(
                expected_version,
                format!("generator_executable_unreadable:{error}"),
            );
        }
    };
    let executable_sha256 = sha256::hex(&executable_bytes);
    let revision_text = revision.as_deref().unwrap_or_default();
    let take_name_text = take_name.as_deref().unwrap_or_default();
    let identity_payload = format!(
        "etuschlopf_generator_snapshot_identity_v1\0{expected_version}\0{revision_text}\0{take_name_text}\0{SOURCE_FINGERPRINT_SCOPE}\0{source_fingerprint_sha256}\0{executable_sha256}"
    );
    let generator_identity_sha256 = sha256::hex(identity_payload.as_bytes());

    GeneratorSnapshotIdentity {
        software_version: expected_version.to_string(),
        revision,
        take_name,
        source_file_count: source_files.len(),
        source_fingerprint_sha256: Some(source_fingerprint_sha256),
        executable_sha256: Some(executable_sha256),
        generator_identity_sha256: Some(generator_identity_sha256),
        qualified: true,
        reason: "version_revision_source_snapshot_and_running_executable_bound".to_string(),
    }
}

fn unqualified(version: &str, reason: String) -> GeneratorSnapshotIdentity {
    GeneratorSnapshotIdentity {
        software_version: version.to_string(),
        revision: None,
        take_name: None,
        source_file_count: 0,
        source_fingerprint_sha256: None,
        executable_sha256: None,
        generator_identity_sha256: None,
        qualified: false,
        reason,
    }
}

fn metadata_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (candidate, value) = line.split_once('=')?;
        (candidate.trim() == key)
            .then(|| value.trim().to_string())
            .filter(|value| !value.is_empty())
    })
}

fn valid_revision(value: &str) -> bool {
    let Some(digits) = value.strip_prefix('r') else {
        return false;
    };
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}

fn collect_source_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let canonical_root = root
        .canonicalize()
        .map_err(|error| format!("project_root_unreadable:{error}"))?;
    let mut files = Vec::new();
    for relative in REQUIRED_ROOT_FILES {
        let path = root.join(relative);
        require_regular_file(&canonical_root, &path, relative)?;
        files.push(PathBuf::from(relative));
    }
    for relative in OPTIONAL_ROOT_FILES {
        let path = root.join(relative);
        if path.exists() {
            require_regular_file(&canonical_root, &path, relative)?;
            files.push(PathBuf::from(relative));
        }
    }
    let src_root = root.join("src");
    let metadata = fs::symlink_metadata(&src_root)
        .map_err(|error| format!("source_directory_unavailable:src:{error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err("source_directory_not_regular:src".to_string());
    }
    collect_tree_files(&canonical_root, &src_root, Path::new("src"), &mut files)?;
    files.sort_by_key(|left| normalize_path(left));
    files.dedup();
    Ok(files)
}

fn collect_tree_files(
    canonical_root: &Path,
    current: &Path,
    relative_prefix: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let mut children = fs::read_dir(current)
        .map_err(|error| format!("source_directory_read_failed:{}:{error}", current.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("source_directory_entry_failed:{}:{error}", current.display()))?;
    children.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    for child in children {
        let path = child.path();
        let relative = relative_prefix.join(child.file_name());
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            format!(
                "source_entry_unreadable:{}:{error}",
                normalize_path(&relative)
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!("source_symlink_forbidden:{}", normalize_path(&relative)));
        }
        if metadata.is_dir() {
            collect_tree_files(canonical_root, &path, &relative, files)?;
        } else if metadata.is_file() {
            let canonical = path.canonicalize().map_err(|error| {
                format!(
                    "source_file_canonicalize_failed:{}:{error}",
                    normalize_path(&relative)
                )
            })?;
            if !canonical.starts_with(canonical_root) {
                return Err(format!("source_path_escaped_root:{}", normalize_path(&relative)));
            }
            files.push(relative);
        } else {
            return Err(format!("source_entry_unsupported:{}", normalize_path(&relative)));
        }
    }
    Ok(())
}

fn require_regular_file(canonical_root: &Path, path: &Path, relative: &str) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("source_root_file_unavailable:{relative}:{error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!("source_root_file_not_regular:{relative}"));
    }
    let canonical = path
        .canonicalize()
        .map_err(|error| format!("source_root_file_canonicalize_failed:{relative}:{error}"))?;
    if !canonical.starts_with(canonical_root) {
        return Err(format!("source_root_file_escaped_root:{relative}"));
    }
    Ok(())
}

fn hash_source_files(root: &Path, files: &[PathBuf]) -> Result<String, String> {
    let mut payload = Vec::new();
    payload.extend_from_slice(b"etuschlopf_generator_source_snapshot_v1\0");
    for relative in files {
        let normalized = normalize_path(relative);
        let bytes = fs::read(root.join(relative))
            .map_err(|error| format!("source_file_read_failed:{normalized}:{error}"))?;
        payload.extend_from_slice(normalized.as_bytes());
        payload.push(0);
        payload.extend_from_slice(bytes.len().to_string().as_bytes());
        payload.push(b':');
        payload.extend_from_slice(&bytes);
        payload.push(b'\n');
    }
    Ok(sha256::hex(&payload))
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fixture_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "etuschlopf_musicxml_generator_identity_{label}_{}_{}",
            std::process::id(),
            nonce
        ))
    }

    fn write_fixture(root: &Path, version: &str, revision: &str) -> PathBuf {
        fs::create_dir_all(root.join("src/audit_only")).expect("create fixture src");
        fs::write(root.join("Cargo.toml"), format!("[package]\nversion=\"{version}\"\n"))
            .expect("write Cargo.toml");
        fs::write(root.join("Cargo.lock"), b"lock").expect("write Cargo.lock");
        fs::write(root.join("rust-toolchain.toml"), b"toolchain").expect("write toolchain");
        fs::write(
            root.join("ZIP_KIND.txt"),
            format!("kind=take\nversion={version}\nrevision={revision}\nname=FixtureTake\n"),
        )
        .expect("write take metadata");
        fs::write(root.join("src/main.rs"), b"fn main() {}\n").expect("write main source");
        fs::write(root.join("src/audit_only/probe.rs"), b"pub fn probe() {}\n")
            .expect("write audit source");
        let executable = root.join("generator.bin");
        fs::write(&executable, b"generator-v1").expect("write executable fixture");
        executable
    }

    #[test]
    fn generator_identity_binds_version_revision_source_and_executable() {
        let root = fixture_root("qualified");
        let executable = write_fixture(&root, "0.10.1839", "r550");
        let identity = build_for_paths(&root, &executable, "0.10.1839");
        let _ = fs::remove_dir_all(&root);

        assert!(identity.qualified, "{}", identity.reason);
        assert_eq!(identity.revision.as_deref(), Some("r550"));
        assert_eq!(identity.take_name.as_deref(), Some("FixtureTake"));
        assert!(identity.source_file_count >= 6);
        assert!(identity.source_fingerprint_bound());
        assert!(identity.executable_fingerprint_bound());
        assert!(identity.generator_identity_sha256.is_some());
    }

    #[test]
    fn source_or_executable_change_changes_generator_identity() {
        let root = fixture_root("change");
        let executable = write_fixture(&root, "0.10.1839", "r550");
        let first = build_for_paths(&root, &executable, "0.10.1839");
        fs::write(root.join("src/main.rs"), b"fn main() { println!(\"changed\"); }\n")
            .expect("change source");
        let source_changed = build_for_paths(&root, &executable, "0.10.1839");
        fs::write(&executable, b"generator-v2").expect("change executable");
        let executable_changed = build_for_paths(&root, &executable, "0.10.1839");
        let _ = fs::remove_dir_all(&root);

        assert_ne!(
            first.source_fingerprint_sha256,
            source_changed.source_fingerprint_sha256
        );
        assert_ne!(
            first.generator_identity_sha256,
            source_changed.generator_identity_sha256
        );
        assert_ne!(
            source_changed.executable_sha256,
            executable_changed.executable_sha256
        );
        assert_ne!(
            source_changed.generator_identity_sha256,
            executable_changed.generator_identity_sha256
        );
    }

    #[test]
    fn version_mismatch_fails_closed() {
        let root = fixture_root("version_mismatch");
        let executable = write_fixture(&root, "0.10.1838", "r550");
        let identity = build_for_paths(&root, &executable, "0.10.1839");
        let _ = fs::remove_dir_all(&root);

        assert!(!identity.qualified);
        assert_eq!(identity.reason, "take_metadata_version_mismatch");
        assert!(!identity.revision_bound());
        assert!(!identity.source_fingerprint_bound());
        assert!(!identity.executable_fingerprint_bound());
    }
}
