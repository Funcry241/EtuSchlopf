fn cleanup_program_temp_leftovers(project_root: &Path) -> TempCleanupReport {
    let current_exe = env::current_exe().ok();
    let mut report = TempCleanupReport::default();
    cleanup_os_temp_leftovers(&mut report, current_exe.as_deref());
    cleanup_project_bootstrap_leftovers(project_root, &mut report, current_exe.as_deref());
    cleanup_download_extract_leftovers(&mut report, current_exe.as_deref());
    report
}

fn cleanup_build_cache_hygiene(project_root: &Path) -> BuildCacheHygieneReport {
    let current_exe = env::current_exe().ok();
    let specs = [
        (
            "legacy_bootstrap_cache",
            project_root.join("target").join("etuschlopf_bootstrap_cache"),
            CachePathPolicy::PlainDirectory,
        ),
        (
            "target_debug",
            project_root.join("target").join("debug"),
            CachePathPolicy::CargoProfile {
                protect_active_executable: true,
            },
        ),
        (
            "target_release",
            project_root.join("target").join("release"),
            CachePathPolicy::CargoProfile {
                protect_active_executable: true,
            },
        ),
        (
            "legacy_root_debug",
            project_root.join("debug"),
            CachePathPolicy::CargoProfile {
                protect_active_executable: false,
            },
        ),
    ];

    let mut report = BuildCacheHygieneReport::default();
    for (_, path, _) in &specs {
        report
            .before
            .add(directory_tree_stats(path).unwrap_or_default());
    }
    report.entries = specs
        .into_iter()
        .map(|(name, path, policy)| CacheCleanupEntry {
            name,
            outcome: cleanup_cache_path(&path, policy, current_exe.as_deref()),
            path,
        })
        .collect();
    for entry in &report.entries {
        report
            .after
            .add(directory_tree_stats(&entry.path).unwrap_or_default());
    }
    report
}

fn cleanup_cache_path(
    path: &Path,
    policy: CachePathPolicy,
    current_exe: Option<&Path>,
) -> CachePathOutcome {
    if !path.is_dir() {
        return CachePathOutcome::Absent;
    }
    if matches!(
        policy,
        CachePathPolicy::CargoProfile {
            protect_active_executable: true
        }
    ) && current_exe.is_some_and(|exe| exe.starts_with(path))
    {
        return CachePathOutcome::SkippedActiveExecutable;
    }
    if matches!(policy, CachePathPolicy::CargoProfile { .. })
        && !looks_like_cargo_profile_directory(path)
    {
        return CachePathOutcome::SkippedNotCargoProfile;
    }
    match fs::remove_dir_all(path) {
        Ok(()) => CachePathOutcome::Deleted,
        Err(error) => CachePathOutcome::Failed(error.to_string()),
    }
}

fn looks_like_cargo_profile_directory(path: &Path) -> bool {
    path.is_dir()
        && path.join(".cargo-lock").is_file()
        && path.join(".fingerprint").is_dir()
        && path.join("deps").is_dir()
        && (path.join("build").is_dir() || path.join("incremental").is_dir())
}

fn directory_tree_stats(path: &Path) -> io::Result<DirectoryTreeStats> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Ok(DirectoryTreeStats::default());
    }
    if metadata.is_file() {
        return Ok(DirectoryTreeStats {
            files: 1,
            directories: 0,
            bytes: metadata.len(),
        });
    }
    if !metadata.is_dir() {
        return Ok(DirectoryTreeStats::default());
    }

    let mut stats = DirectoryTreeStats {
        files: 0,
        directories: 1,
        bytes: 0,
    };
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        stats.add(directory_tree_stats(&entry.path())?);
    }
    Ok(stats)
}

fn append_build_cache_hygiene_to_console_log(
    label: &str,
    report: &BuildCacheHygieneReport,
) -> AppResult<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(CONSOLE_LOG)?;
    writeln!(file, "BUILD-CACHE-HYGIENE-{label}")?;
    writeln!(file, "--------------------{}", "-".repeat(label.len()))?;
    writeln!(
        file,
        "BeforeBytes: {}; BeforeFiles: {}; BeforeDirectories: {}; AfterBytes: {}; AfterFiles: {}; AfterDirectories: {}; ReclaimedBytes: {}; ActiveProfileSkipped: {}; FailedRemovals: {}",
        report.before.bytes,
        report.before.files,
        report.before.directories,
        report.after.bytes,
        report.after.files,
        report.after.directories,
        report.bytes_reclaimed(),
        report.active_profile_skips(),
        report.failed_removals(),
    )?;
    writeln!(file, "PATH-OUTCOMES")?;
    for entry in &report.entries {
        match &entry.outcome {
            CachePathOutcome::Failed(error) => writeln!(
                file,
                "{}: failed; Path: {}; Error: {}",
                entry.name,
                entry.path.display(),
                error
            )?,
            outcome => writeln!(
                file,
                "{}: {}; Path: {}",
                entry.name,
                outcome.label(),
                entry.path.display()
            )?,
        }
    }
    writeln!(
        file,
        "Policy: CARGO_INCREMENTAL=1 is set by run.cmd. Productive target/strict, strict all-target test and target/bc/debug bootstrap incremental state are retained as derived Cargo cache, so a take rebuilds bootstrap and launcher incrementally; only the retired legacy target/etuschlopf_bootstrap_cache root, fingerprinted stale target/debug and target/release profiles, and a fingerprinted root-level debug directory may be removed. Reuse authority remains source-signature + Cargo fingerprint + executable hash; recovery/transaction state, source files and project artifacts remain untouched."
    )?;
    writeln!(file)?;
    Ok(())
}

/// During a take the parent take bootstrap still runs from its own run directory while
/// it waits for this launcher; run.cmd passes that directory down as
/// `ETUSCHLOPF_TAKE_BOOTSTRAP_RUN`. Cleanup must never touch it.
fn is_running_parent_take_bootstrap_dir(path: &Path) -> bool {
    let Some(run) = env::var_os("ETUSCHLOPF_TAKE_BOOTSTRAP_RUN") else {
        return false;
    };
    let run = PathBuf::from(run);
    match (fs::canonicalize(&run), fs::canonicalize(path)) {
        (Ok(run), Ok(path)) => run == path,
        _ => false,
    }
}

/// Extraction directories of interrupted take bootstraps. Only directories older than
/// this are removed, so the directory of a take that is still running is never touched.
const STALE_TAKE_EXTRACTION_AGE: std::time::Duration = std::time::Duration::from_secs(24 * 60 * 60);

fn cleanup_stale_take_extraction_dirs(report: &mut TempCleanupReport, current_exe: Option<&Path>) {
    let Ok(entries) = fs::read_dir(env::temp_dir()) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let stale = entry
            .metadata()
            .ok()
            .filter(fs::Metadata::is_dir)
            .and_then(|meta| meta.modified().ok())
            .and_then(|modified| modified.elapsed().ok())
            .is_some_and(|age| age > STALE_TAKE_EXTRACTION_AGE);
        let named = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("etuschlopf_rust_take_bootstrap_"));
        if stale && named {
            cleanup_candidate_dir(&path, current_exe, report, CleanupBucket::OsTemp, false);
        }
    }
}

fn cleanup_os_temp_leftovers(report: &mut TempCleanupReport, current_exe: Option<&Path>) {
    cleanup_stale_take_extraction_dirs(report, current_exe);
    let temp_root = env::temp_dir();
    let prefixes = [
        "etuschlopf_take_",
        "etuschlopf_launcher_",
        "etuschlopf_bootstrap_",
        "etuschlopf_unpack_",
        "etuschlopf_temp_",
    ];
    cleanup_prefixed_child_dirs(
        &temp_root,
        &prefixes,
        current_exe,
        report,
        CleanupBucket::OsTemp,
        false,
    );
}

fn cleanup_project_bootstrap_leftovers(
    project_root: &Path,
    report: &mut TempCleanupReport,
    current_exe: Option<&Path>,
) {
    let target_root = project_root.join("target");
    for run_dir in [
        RUN_LAUNCHER_BOOTSTRAP_RUNS_DIR,
        TAKE_BOOTSTRAP_RUNS_DIR,
        LEGACY_LAUNCHER_BOOTSTRAP_RUNS_DIR,
    ] {
        cleanup_children_dirs(
            &target_root.join(run_dir),
            current_exe,
            report,
            CleanupBucket::BootstrapRuns,
            false,
        );
    }
    cleanup_prefixed_child_dirs(
        &target_root,
        &["launcher_bootstrap_v0_10_"],
        current_exe,
        report,
        CleanupBucket::VersionedBootstrap,
        false,
    );
}

fn cleanup_download_extract_leftovers(report: &mut TempCleanupReport, current_exe: Option<&Path>) {
    let Ok(downloads) = downloads_dir() else {
        return;
    };
    cleanup_prefixed_child_dirs(
        &downloads,
        &["work", "EtuSchlopf_v"],
        current_exe,
        report,
        CleanupBucket::DownloadExtract,
        true,
    );
}

#[derive(Clone, Copy)]
enum CleanupBucket {
    OsTemp,
    BootstrapRuns,
    VersionedBootstrap,
    DownloadExtract,
}

fn cleanup_prefixed_child_dirs(
    parent: &Path,
    prefixes: &[&str],
    current_exe: Option<&Path>,
    report: &mut TempCleanupReport,
    bucket: CleanupBucket,
    require_cargo_project: bool,
) {
    let Ok(entries) = fs::read_dir(parent) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if !prefixes.iter().any(|prefix| name.starts_with(prefix)) {
            continue;
        }
        cleanup_candidate_dir(&path, current_exe, report, bucket, require_cargo_project);
    }
}

fn cleanup_children_dirs(
    parent: &Path,
    current_exe: Option<&Path>,
    report: &mut TempCleanupReport,
    bucket: CleanupBucket,
    require_cargo_project: bool,
) {
    let Ok(entries) = fs::read_dir(parent) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            cleanup_candidate_dir(&path, current_exe, report, bucket, require_cargo_project);
        }
    }
}

fn directory_contains_cargo_toml(path: &Path) -> AppResult<bool> {
    if path.join("Cargo.toml").exists() {
        return Ok(true);
    }
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() && child.join("Cargo.toml").exists() {
            return Ok(true);
        }
    }
    Ok(false)
}

fn cleanup_candidate_dir(
    path: &Path,
    current_exe: Option<&Path>,
    report: &mut TempCleanupReport,
    bucket: CleanupBucket,
    require_cargo_project: bool,
) {
    if current_exe
        .map(|exe| exe.starts_with(path))
        .unwrap_or(false)
        || is_running_parent_take_bootstrap_dir(path)
    {
        report.skipped_current_dirs += 1;
        return;
    }
    if require_cargo_project && !directory_contains_cargo_toml(path).unwrap_or(false) {
        return;
    }
    match fs::remove_dir_all(path) {
        Ok(()) => match bucket {
            CleanupBucket::OsTemp => report.os_temp_dirs_deleted += 1,
            CleanupBucket::BootstrapRuns => report.bootstrap_dirs_deleted += 1,
            CleanupBucket::VersionedBootstrap => report.versioned_bootstrap_dirs_deleted += 1,
            CleanupBucket::DownloadExtract => report.download_extract_dirs_deleted += 1,
        },
        Err(error) => report.failures.push(CleanupFailure {
            path: path.to_path_buf(),
            error: error.to_string(),
        }),
    }
}

fn append_temp_cleanup_to_console_log(label: &str, report: &TempCleanupReport) -> AppResult<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(CONSOLE_LOG)?;
    writeln!(file, "TEMP-CLEANUP-{label}")?;
    writeln!(file, "----------------{}", "-".repeat(label.len()))?;
    writeln!(
        file,
        "DeletedTotal: {}; OsTempDirs: {}; BootstrapRunTargets: {}; VersionedBootstrapTargets: {}; DownloadExtractDirs: {}; CurrentRunningSkipped: {}; FailedRemovals: {}",
        report.deleted_total(),
        report.os_temp_dirs_deleted,
        report.bootstrap_dirs_deleted,
        report.versioned_bootstrap_dirs_deleted,
        report.download_extract_dirs_deleted,
        report.skipped_current_dirs,
        report.failed_removals()
    )?;
    for failure in &report.failures {
        writeln!(
            file,
            "FailurePath: {}; Error: {}",
            failure.path.display(),
            failure.error
        )?;
    }
    writeln!(file)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RootCleanupPolicy {
    CurrentTarget,
    PreserveGeneratedOutputs,
}

impl ArgsState {
    fn startup_root_cleanup_policy(&self) -> RootCleanupPolicy {
        match &self.request {
            RunRequest::DeepClippy { .. } => RootCleanupPolicy::PreserveGeneratedOutputs,
            RunRequest::Execute(_) => RootCleanupPolicy::CurrentTarget,
        }
    }
}

fn cleanup_startup_project_root_residue(
    project_root: &Path,
    args: &ArgsState,
) -> RootCleanupReport {
    cleanup_project_root_residue_with_policy(
        project_root,
        args.input_arg(),
        args.startup_root_cleanup_policy(),
        args.execution().is_some_and(|request| request.inventory.requested()),
    )
}

fn cleanup_project_root_residue(
    project_root: &Path,
    input_arg: Option<&str>,
    preserve_inventory: bool,
) -> RootCleanupReport {
    cleanup_project_root_residue_with_policy(
        project_root,
        input_arg,
        RootCleanupPolicy::CurrentTarget,
        preserve_inventory,
    )
}

fn cleanup_project_root_residue_with_policy(
    project_root: &Path,
    input_arg: Option<&str>,
    policy: RootCleanupPolicy,
    preserve_inventory: bool,
) -> RootCleanupReport {
    let mut report = RootCleanupReport::default();
    let mut protected_outputs = current_standard_output_names(project_root, input_arg);
    if preserve_inventory {
        protected_outputs.insert(INVENTORY_JSON_FILE.to_string());
        protected_outputs.insert(INVENTORY_XLSX_FILE.to_string());
    }
    let Ok(entries) = fs::read_dir(project_root) else {
        report.failed_removals += 1;
        return report;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if is_root_hygiene_protected(&path, name, &protected_outputs, policy) {
            report.skipped_protected += 1;
            continue;
        }
        if path.is_file() {
            cleanup_root_file(&path, name, &protected_outputs, &mut report);
        } else if path.is_dir() {
            cleanup_root_dir(&path, name, &mut report);
        }
    }
    report
}

fn is_root_hygiene_protected(
    path: &Path,
    name: &str,
    protected_outputs: &HashSet<String>,
    policy: RootCleanupPolicy,
) -> bool {
    if path.is_dir() {
        return matches!(
            name,
            "src" | "docs" | "tools" | "target" | ".git" | ".github" | ".vscode"
        );
    }
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".wav") || lower.ends_with(".mid") {
        return true;
    }
    if is_core_root_file(name) {
        return true;
    }
    if policy == RootCleanupPolicy::PreserveGeneratedOutputs && is_standard_output_name(name) {
        return true;
    }
    protected_outputs.contains(name)
}

fn is_core_root_file(name: &str) -> bool {
    matches!(
        name,
        "Cargo.toml"
            | "Cargo.lock"
            | "README.md"
            | "run.cmd"
            | "strict.cmd"
            | "rust-toolchain.toml"
            | "rustfmt.toml"
            | "clippy.toml"
            | ".gitignore"
    )
}

fn current_standard_output_names(project_root: &Path, input_arg: Option<&str>) -> HashSet<String> {
    let mut names = HashSet::new();
    let Some(input) = input_arg else {
        return names;
    };
    if is_all_arg(input) {
        names.insert("corpus_emergence.result.txt".to_string());
        names.insert(CORPUS_RUN_MANIFEST_FILE.to_string());
        names.insert("corpus_emergence.strict-clippy.txt".to_string());
        for project in scan_current_wav_midi_project_names(project_root) {
            names.extend(standard_output_names_for_project(&project));
        }
        return names;
    }
    if is_visual_preflight_arg(input) {
        names.insert("visual_preflight.result.txt".to_string());
        names.insert("visual_preflight.strict-clippy.txt".to_string());
        names.insert(VISUAL_PREFLIGHT_STRICT_TEST_LOG.to_string());
        names.insert("corpus_emergence.result.txt".to_string());
        names.insert(CORPUS_RUN_MANIFEST_FILE.to_string());
        let plan = visual_preflight_projects(project_root, env!("CARGO_PKG_VERSION"));
        for project in &plan.emergence.projects {
            names.insert(format!("{project}.result.txt"));
        }
        for project in plan.projects {
            names.extend(standard_output_names_for_project(&project));
        }
        return names;
    }
    names.extend(standard_output_names_for_project(input));
    names
}

fn scan_current_wav_midi_project_names(project_root: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(project_root) else {
        return Vec::new();
    };
    let mut projects = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file()
            || !path
                .extension()
                .and_then(|value| value.to_str())
                .map(|value| value.eq_ignore_ascii_case("wav"))
                .unwrap_or(false)
        {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
            continue;
        };
        if project_root.join(format!("{stem}.mid")).exists() {
            projects.push(stem.to_string());
        }
    }
    projects
}

fn standard_output_names_for_project(project: &str) -> Vec<String> {
    vec![
        format!("{project}.result.txt"),
        format!("{project}.strict-clippy.txt"),
        format!("{project}.musicxml"),
        format!("{project}.render.musicxml"),
        format!("{project}.pdf"),
    ]
}

fn cleanup_root_file(path: &Path, name: &str, protected_outputs: &HashSet<String>, report: &mut RootCleanupReport) {
    if is_stale_root_zip(name) {
        remove_root_file(path, report, RootFileBucket::Zip);
    } else if is_stale_root_console_log(name) {
        remove_root_file(path, report, RootFileBucket::Log);
    } else if is_stale_standard_output_name(name, protected_outputs) {
        remove_root_file(path, report, RootFileBucket::Output);
    } else if inventory_output_name(name) {
        if protected_outputs.contains(name) {
            return;
        }
        remove_root_file(path, report, RootFileBucket::Output);
    } else if is_stale_assistant_root_doc_name(name) || is_root_temp_file_name(name) {
        remove_root_file(path, report, RootFileBucket::Temp);
    }
}

#[derive(Clone, Copy)]
enum RootFileBucket {
    Zip,
    Output,
    Log,
    Temp,
}

fn remove_root_file(path: &Path, report: &mut RootCleanupReport, bucket: RootFileBucket) {
    match fs::remove_file(path) {
        Ok(()) => match bucket {
            RootFileBucket::Zip => report.stale_zip_files_deleted += 1,
            RootFileBucket::Output => report.stale_output_files_deleted += 1,
            RootFileBucket::Log => report.stale_log_files_deleted += 1,
            RootFileBucket::Temp => report.temp_files_deleted += 1,
        },
        Err(_) => report.failed_removals += 1,
    }
}

fn cleanup_root_dir(path: &Path, name: &str, report: &mut RootCleanupReport) {
    if !is_root_temp_dir_name(name) {
        return;
    }
    match fs::remove_dir_all(path) {
        Ok(()) => report.temp_dirs_deleted += 1,
        Err(_) => report.failed_removals += 1,
    }
}

fn is_stale_root_zip(name: &str) -> bool {
    name.starts_with("EtuSchlopf_v") && name.ends_with(".zip")
}

fn is_stale_root_console_log(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == "etuschlopf.console.txt"
        || (lower.starts_with("etuschlopf.console") && lower.ends_with(".txt"))
        || lower == "console.txt"
}

fn is_stale_assistant_root_doc_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    (lower.starts_with("readme_assistant_") && lower.ends_with(".txt"))
        || lower == "manifest_1053_external_memory_addendum.txt"
        || lower == "manifest_1054_external_memory_export_snapshot_addendum.txt"
        || lower == "readme_gouldregisterislandaudit_handoff.txt"
        || lower == "gould_register_island_audit_contract.txt"
        || lower == "readme_take_bootstrap_contract_1064.txt"
        || lower == "source_release_freshness_audit.txt"
        || lower == "workflow_closure_proof.txt"
        || matches!(
            lower.as_str(),
            ".etuschlopf_complete_09_lock_ready"
                | ".etuschlopf_regenerate_lock_1131"
                | "package_complete_09.txt"
                | "paket_complete_08.txt"
                | "gould_register_island_audit.tsv"
                | "v0.10.1336.launcher-tests.txt"
                | "v0.10.1337.launcher-tests.txt"
                | "v0.10.1338.public-artifact-fault-audit.txt"
                | "v0.10.1339.launcher-tests.txt"
                | "v0.10.1339.public-artifact-fault-audit.txt"
                | "v0.10.1340.launcher-tests.txt"
                | "visual_preflight.strict-tests.txt"
                | "desktop.ini"
                | "files.txt"
        )
}

fn is_stale_standard_output_name(name: &str, protected_outputs: &HashSet<String>) -> bool {
    !protected_outputs.contains(name) && is_standard_output_name(name)
}

fn is_standard_output_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".result.txt")
        || lower.ends_with(".strict-clippy.txt")
        || lower.ends_with(".musicxml")
        || matches!(lower.as_str(), "etuschlopf.inventory.json" | "etuschlopf.inventory.xlsx")
        || is_root_pdf_output_name(&lower)
}

fn is_root_pdf_output_name(lower_name: &str) -> bool {
    if !lower_name.ends_with(".pdf") {
        return false;
    }
    lower_name.contains("preview")
        || lower_name.contains("shadow")
        || lower_name.contains("variant")
        || lower_name.contains("questions")
        || lower_name.contains("notation")
        || lower_name.contains("etüde")
        || lower_name.contains("etuede")
        || lower_name.contains("etude")
}

fn is_root_temp_file_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".tmp")
        || lower.ends_with(".temp")
        || lower.ends_with(".bak")
        || lower.ends_with(".old")
        || lower.ends_with(".orig")
        || lower.ends_with(".rej")
        || lower.ends_with('~')
        || (lower.starts_with("etuschlopf_recovery_take_v0_10_")
            && lower.ends_with(".ps1"))
        || lower.starts_with("etuschlopf_tmp_")
        || lower.starts_with("etuschlopf_temp_")
}

fn is_root_temp_dir_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.starts_with("etuschlopf_take_")
        || lower.starts_with("etuschlopf_unpack_")
        || lower.starts_with("etuschlopf_temp_")
        || lower.starts_with("etuschlopf_launcher_")
        || lower.starts_with("etuschlopf_v0_10_")
        || lower.starts_with("etusclhopf_v0_10_")
        || lower.starts_with("work") && lower.contains("etuschlopf")
        || lower.starts_with("etuschlopf-")
}

fn append_root_cleanup_to_console_log(label: &str, report: &RootCleanupReport) -> AppResult<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(CONSOLE_LOG)?;
    writeln!(file, "ROOT-HYGIENE-{label}")?;
    writeln!(file, "-----------------{}", "-".repeat(label.len()))?;
    writeln!(
        file,
        "DeletedTotal: {}; StaleZipFiles: {}; StaleOutputFiles: {}; StaleLogFiles: {}; TempFiles: {}; TempDirs: {}; ProtectedSkipped: {}; FailedRemovals: {}",
        report.deleted_total(),
        report.stale_zip_files_deleted,
        report.stale_output_files_deleted,
        report.stale_log_files_deleted,
        report.temp_files_deleted,
        report.temp_dirs_deleted,
        report.skipped_protected,
        report.failed_removals
    )?;
    writeln!(
        file,
        "Policy: Root keeps Cargo/run/src/docs/tools/target, WAV/MIDI inputs plus current standard outputs; stale ZIP/log/temp/output residue is removed. Full console log is below target/etuschlopf_logs, not loose in project root."
    )?;
    writeln!(file)?;
    Ok(())
}

fn remove_obsolete_powershell_files(project_root: &Path) {
    let obsolete_files = [
        project_root.join("tools").join("run_impl.ps1"),
        project_root.join("tools").join("synergy_baseline_metrics.ps1"),
        project_root.join("update_from_zip.cmd"),
        project_root.join("update_from_zip.ps1"),
    ];
    for path in obsolete_files {
        if path.exists() {
            let _ = fs::remove_file(path);
        }
    }
}

