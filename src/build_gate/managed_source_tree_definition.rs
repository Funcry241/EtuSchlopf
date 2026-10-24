#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootProjectFileRole {
    TakeManaged,
    LocalTransactional,
}

pub const MANAGED_DIRS: [&str; 4] = ["src", "docs", "tools", "tests"];

pub const MANAGED_ROOT_FILES: [&str; 8] = [
    "Cargo.toml",
    "README.md",
    "ZIP_KIND.txt",
    "run.cmd",
    "strict.cmd",
    "rust-toolchain.toml",
    "rustfmt.toml",
    "clippy.toml",
];

pub const REQUIRED_MANAGED_ROOT_FILES: [&str; 5] = [
    "Cargo.toml",
    "README.md",
    "ZIP_KIND.txt",
    "run.cmd",
    "strict.cmd",
];

// Root Cargo.lock is deliberately not TakeManaged. Cargo owns its bytes locally,
// while EtuSchlopf treats the file as local project state represented by complete
// source snapshots and protected byte-exactly by the Take transaction without ever
// becoming incoming Take payload.
pub const LOCAL_TRANSACTIONAL_ROOT_FILES: [&str; 1] = ["Cargo.lock"];

pub fn root_project_file_role(relative: &str) -> Option<RootProjectFileRole> {
    if MANAGED_ROOT_FILES.contains(&relative) {
        Some(RootProjectFileRole::TakeManaged)
    } else if LOCAL_TRANSACTIONAL_ROOT_FILES.contains(&relative) {
        Some(RootProjectFileRole::LocalTransactional)
    } else {
        None
    }
}

pub fn validate_root_project_file_roles() -> Result<(), String> {
    for relative in MANAGED_ROOT_FILES {
        if root_project_file_role(relative) != Some(RootProjectFileRole::TakeManaged) {
            return Err(format!(
                "ROOT PROJECT FILE ROLE REGRESSION: Take-managed root file {relative:?} has no unique TakeManaged role"
            ));
        }
    }
    for relative in LOCAL_TRANSACTIONAL_ROOT_FILES {
        if root_project_file_role(relative) != Some(RootProjectFileRole::LocalTransactional) {
            return Err(format!(
                "ROOT PROJECT FILE ROLE REGRESSION: local transaction root file {relative:?} has no unique LocalTransactional role"
            ));
        }
        if MANAGED_ROOT_FILES.contains(&relative) {
            return Err(format!(
                "ROOT PROJECT FILE ROLE REGRESSION: {relative:?} is both TakeManaged and LocalTransactional"
            ));
        }
    }
    if root_project_file_role("Cargo.lock") != Some(RootProjectFileRole::LocalTransactional) {
        return Err(
            "ROOT PROJECT FILE ROLE REGRESSION: root Cargo.lock must remain LocalTransactional and never incoming TakeManaged payload"
                .to_string(),
        );
    }
    Ok(())
}
