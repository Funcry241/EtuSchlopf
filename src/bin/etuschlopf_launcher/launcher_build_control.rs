fn is_deep_clippy_warning_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("warning:") || trimmed.contains(": warning: ")
}

fn execute_deep_clippy_diagnostic(
    launcher: &mut Launcher,
    build_plan: BuildPlan,
) -> AppResult<()> {
    let log_path = PathBuf::from(DEEP_CLIPPY_LOG);
    write_deep_clippy_log_header(
        &log_path,
        &launcher.version,
        launcher.thread_plan.cargo_jobs,
    )?;
    launcher.line(
        PresentationRole::Check,
        None,
        Attention::Neutral,
        Emphasis::Secondary,
        "",
    );
    launcher.line(
        PresentationRole::Check,
        None,
        Attention::Neutral,
        Emphasis::Secondary,
        "[EtuSchlopf] Deep Clippy: pedantic + nursery + cargo, nur diagnostisch.",
    );
    let clippy_args = vec![
        "clippy".to_string(),
        "--locked".to_string(),
        "--message-format".to_string(),
        "short".to_string(),
        "--profile".to_string(),
        "strict".to_string(),
        "--all-targets".to_string(),
        "--jobs".to_string(),
        launcher.thread_plan.cargo_jobs.to_string(),
        "--".to_string(),
        "-W".to_string(),
        "clippy::pedantic".to_string(),
        "-W".to_string(),
        "clippy::nursery".to_string(),
        "-W".to_string(),
        "clippy::cargo".to_string(),
        "-W".to_string(),
        "rust-2024-compatibility".to_string(),
    ];
    let result = invoke_logged_process(
        launcher,
        "cargo",
        &clippy_args,
        &log_path,
        "CARGO-CLIPPY-DEEP",
        PresentationRole::Check,
        "Deep Clippy",
    )?;
    if result.exit_code != 0 {
        launcher.line(
            PresentationRole::Result,
            Some(LocalStatus::Failed),
            Attention::Error,
            Emphasis::Primary,
            &format!(
                "[EtuSchlopf] Deep Clippy konnte nicht vollständig ausgeführt werden: exit {} | {}",
                result.exit_code,
                log_path.display()
            ),
        );
        launcher.fail_observation();
        return launcher_exit(result.exit_code);
    }
    let build_decision = materialize_diagnostic_build_plan(launcher, build_plan)?;
    let warning_count = append_deep_clippy_summary(
        &log_path,
        result.exit_code,
        result.elapsed_ms,
        &build_decision,
    )?;
    launcher.line(
        PresentationRole::Check,
        Some(LocalStatus::Ok),
        Attention::Notice,
        Emphasis::Secondary,
        &format!(
            "[EtuSchlopf] Deep Clippy abgeschlossen: {} Warnungen | {}",
            warning_count,
            log_path.display()
        ),
    );
    launcher.line(
        PresentationRole::Check,
        None,
        Attention::Neutral,
        Emphasis::Muted,
        "[EtuSchlopf] Diagnose und Build verwenden denselben BuildPlan; die Musikpipeline wurde nicht gestartet.",
    );
    launcher.complete_observation();
    Ok(())
}

fn write_deep_clippy_log_header(path: &Path, version: &str, cargo_jobs: usize) -> AppResult<()> {
    let mut file = File::create(path)?;
    writeln!(file, "ETUSCHLOPF DEEP CLIPPY DIAGNOSTIC")?;
    writeln!(file, "==================================")?;
    writeln!(file, "Version: {version}")?;
    writeln!(file, "Mode: diagnostic_only_non_blocking_warnings")?;
    writeln!(file, "Projection: cargo_message_format_short")?;
    writeln!(file, "Targets: all")?;
    writeln!(file, "LintGroups: clippy::pedantic, clippy::nursery, clippy::cargo, rust-2024-compatibility")?;
    writeln!(
        file,
        "Befehl: cargo clippy --locked --message-format short --profile strict --all-targets --jobs {cargo_jobs} -- -W clippy::pedantic -W clippy::nursery -W clippy::cargo -W rust-2024-compatibility"
    )?;
    writeln!(file, "CargoJobsExplicit: {cargo_jobs}")?;
    writeln!(file)?;
    Ok(())
}

fn append_deep_clippy_summary(
    path: &Path,
    exit_code: i32,
    elapsed_ms: u128,
    build_decision: &str,
) -> AppResult<usize> {
    let text = fs::read_to_string(path).unwrap_or_default();
    let raw_warnings = text
        .lines()
        .filter(|line| is_deep_clippy_warning_line(line))
        .count();
    let summary_warnings = text
        .lines()
        .filter(|line| is_cargo_warning_summary_line(line))
        .count();
    let warnings = raw_warnings.saturating_sub(summary_warnings);
    let mut file = OpenOptions::new().append(true).open(path)?;
    writeln!(file)?;
    writeln!(file, "DEEP-CLIPPY-SUMMARY")?;
    writeln!(file, "--------------------")?;
    writeln!(file, "CargoExit: {exit_code}")?;
    writeln!(file, "ElapsedMs: {elapsed_ms}")?;
    writeln!(file, "Warnings: {warnings}")?;
    writeln!(file, "RawWarningLines: {raw_warnings}")?;
    writeln!(file, "SummaryWarningLinesExcluded: {summary_warnings}")?;
    writeln!(file, "BuildProfile: strict")?;
    writeln!(file, "BuildDecision: {build_decision}")?;
    writeln!(file, "SharedBuildPlan: ja")?;
    writeln!(file, "GateChanged: nein")?;
    writeln!(file, "MusicPipelineRun: nein")?;
    writeln!(file, "Decision: diagnostic_only_cargo_short_rendering_no_warning_target_lint_or_release_gate_change")?;
    Ok(warnings)
}

fn verified_artifact_message(profile: CargoBuildProfile) -> String {
    format!(
        "{} wiederverwenden (Profil, Quelle und Binary verifiziert).",
        profile.as_str()
    )
}

fn compile_reason_message(profile: CargoBuildProfile, reason: CompileReason) -> String {
    match reason {
        CompileReason::Forced => format!("{} neu (--force).", profile.as_str()),
        CompileReason::MissingVerifiedArtifact => {
            format!("{} neu (kein passendes verifiziertes Artefakt).", profile.as_str())
        }
    }
}

fn refresh_build_plan_after_diagnostic(
    version: &str,
    build_plan: BuildPlan,
) -> AppResult<BuildPlan> {
    match build_plan {
        BuildPlan::Compile {
            reason: CompileReason::Forced,
            ..
        } => Ok(build_plan),
        BuildPlan::Reuse(profile) | BuildPlan::Compile { profile, .. } => {
            plan_build(
                &env::current_dir()?,
                version,
                profile,
                BuildPreference::ReuseVerified,
            )
        }
    }
}

fn materialize_diagnostic_build_plan(
    launcher: &mut Launcher,
    build_plan: BuildPlan,
) -> AppResult<String> {
    let build_plan = refresh_build_plan_after_diagnostic(&launcher.version, build_plan)?;
    match build_plan {
        BuildPlan::Reuse(profile) => {
            launcher.role_detail_line(
                ConsolePrefix::Product,
                PresentationRole::Build,
                "Buildplan",
                &verified_artifact_message(profile),
                Emphasis::Secondary,
            );
            Ok(format!("reused_verified_{}_artifact", profile.as_str()))
        }
        BuildPlan::Compile { profile, reason } => {
            launcher.role_detail_line(
                ConsolePrefix::Product,
                PresentationRole::Build,
                "Buildplan",
                &compile_reason_message(profile, reason),
                Emphasis::Secondary,
            );
            run_cargo_build(launcher, profile)?;
            write_successful_build_stamp(&launcher.version, profile)?;
            Ok(format!("compiled_{}_artifact", profile.as_str()))
        }
    }
}

fn reuse_verified_build(
    launcher: &mut Launcher,
    state: &mut RunState,
    profile: CargoBuildProfile,
    strict_log_path: &Path,
) -> AppResult<()> {
    if state.validation.is_strict() {
        // A verified executable still requires the exact strict Clippy contract.
        run_or_reuse_strict_clippy(launcher, state, strict_log_path)?;
    } else {
        state.strict_clippy_exit = "not_run".to_string();
        state.clippy_ms = "not_run".to_string();
    }
    state.build_exit = "0".to_string();
    state.build_ms = "skipped_verified_build_artifact".to_string();
    state.build_outcome = BuildOutcome::Reused(profile);
    launcher.role_detail_line(
        ConsolePrefix::Product,
        PresentationRole::Build,
        "Buildplan",
        &verified_artifact_message(profile),
        Emphasis::Secondary,
    );
    Ok(())
}

fn run_or_reuse_strict_clippy(
    launcher: &mut Launcher,
    state: &mut RunState,
    strict_log_path: &Path,
) -> AppResult<()> {
    let _timing = LauncherPhaseTimer::new("ClippyControlIncludingReceipt");
    register_take_attempt_artifact("strict_clippy_log", strict_log_path);
    if strict_clippy_validation_receipt_matches(&launcher.version) {
        write_reused_strict_clippy_log(strict_log_path, &launcher.version)?;
        state.strict_log = Some(strict_log_path.to_path_buf());
        state.strict_clippy_exit = "0".to_string();
        state.clippy_ms = "0".to_string();
        launcher.role_status_detail_line(
            ConsolePrefix::Versioned,
            PresentationRole::Check,
            "Clippy",
            LocalStatus::Ok,
            "REUSE",
            "exaktes sauberes Strict-Clippy-Receipt",
        );
        return Ok(());
    }
    run_strict_clippy(launcher, state, strict_log_path)
}

fn run_strict_clippy(
    launcher: &mut Launcher,
    state: &mut RunState,
    strict_log_path: &Path,
) -> AppResult<()> {
    register_take_attempt_artifact("strict_clippy_log", strict_log_path);
    state.strict_log = Some(strict_log_path.to_path_buf());
    let _ = fs::remove_file(STRICT_CLIPPY_VALIDATION_RECEIPT);
    write_strict_log_header(
        strict_log_path,
        &launcher.version,
        launcher.thread_plan.cargo_jobs,
    )?;
    let clippy_args = vec![
        "clippy".to_string(),
        "--locked".to_string(),
        "--profile".to_string(),
        "strict".to_string(),
        "--jobs".to_string(),
        launcher.thread_plan.cargo_jobs.to_string(),
        "--color".to_string(),
        "never".to_string(),
        "--".to_string(),
        "-D".to_string(),
        "clippy::correctness".to_string(),
        "-D".to_string(),
        "clippy::suspicious".to_string(),
        "-D".to_string(),
        "clippy::perf".to_string(),
    ];
    let result = invoke_logged_process(
        launcher,
        "cargo",
        &clippy_args,
        strict_log_path,
        "CARGO-CLIPPY",
        PresentationRole::Check,
        "Clippy",
    )?;
    state.strict_clippy_exit = result.exit_code.to_string();
    state.clippy_ms = result.elapsed_ms.to_string();
    let warning_count = write_clippy_warning_budget(strict_log_path, result.elapsed_ms)?;
    if result.exit_code != 0 {
        launcher.line(
            PresentationRole::Result,
            Some(LocalStatus::Failed),
            Attention::Error,
            Emphasis::Primary,
            &format!(
                "[EtuSchlopf] Clippy ROT: {}",
                strict_log_path.display()
            ),
        );
        launcher.fail_observation();
        return launcher_exit(result.exit_code);
    }
    if warning_count == 0 {
        write_strict_clippy_validation_receipt(&launcher.version)?;
    }
    Ok(())
}

fn run_cargo_build(
    launcher: &mut Launcher,
    profile: CargoBuildProfile,
) -> AppResult<RunResult> {
    let _timing = LauncherPhaseTimer::new("CargoBuildControl");
    let build_args = vec![
        "build".to_string(),
        "--locked".to_string(),
        "--profile".to_string(),
        profile.as_str().to_string(),
        "--bin".to_string(),
        "etuschlopf".to_string(),
        "--jobs".to_string(),
        launcher.thread_plan.cargo_jobs.to_string(),
    ];
    let result = invoke_logged_process(
        launcher,
        "cargo",
        &build_args,
        Path::new(CONSOLE_LOG),
        "CARGO-BUILD",
        PresentationRole::Build,
        "Build",
    )?;
    if result.exit_code != 0 {
        launcher.line(
            PresentationRole::Result,
            Some(LocalStatus::Failed),
            Attention::Error,
            Emphasis::Primary,
            &format!("[EtuSchlopf] Build ROT: {CONSOLE_LOG}"),
        );
        print_log_tail(launcher, Path::new(CONSOLE_LOG), 40)?;
        launcher.fail_observation();
        return launcher_exit(result.exit_code);
    }
    Ok(result)
}

fn compile_artifact(
    launcher: &mut Launcher,
    state: &mut RunState,
    profile: CargoBuildProfile,
) -> AppResult<()> {
    let result = run_cargo_build(launcher, profile)?;
    state.build_exit = result.exit_code.to_string();
    state.build_ms = result.elapsed_ms.to_string();
    state.build_outcome = BuildOutcome::Compiled(profile);
    Ok(())
}

fn compile_planned_build(
    launcher: &mut Launcher,
    state: &mut RunState,
    profile: CargoBuildProfile,
    reason: CompileReason,
    strict_log_path: &Path,
) -> AppResult<()> {
    launcher.role_detail_line(
        ConsolePrefix::Product,
        PresentationRole::Build,
        "Buildplan",
        &compile_reason_message(profile, reason),
        Emphasis::Secondary,
    );
    if state.validation.is_strict() {
        run_strict_clippy(launcher, state, strict_log_path)?;
    }
    compile_artifact(launcher, state, profile)?;
    write_successful_build_stamp(&launcher.version, profile)
}

fn target_exe_for_profile(profile: &str) -> PathBuf {
    if cfg!(windows) {
        PathBuf::from("target").join(profile).join("etuschlopf.exe")
    } else {
        PathBuf::from("target").join(profile).join("etuschlopf")
    }
}

fn build_plan_from_verification(
    requested: CargoBuildProfile,
    build_preference: BuildPreference,
    available: ArtifactAvailability,
) -> BuildPlan {
    if build_preference == BuildPreference::ForceCompile {
        return BuildPlan::Compile {
            profile: requested,
            reason: CompileReason::Forced,
        };
    }
    match available.verified_profile(requested) {
        Some(profile) => BuildPlan::Reuse(profile),
        None => BuildPlan::Compile {
            profile: requested,
            reason: CompileReason::MissingVerifiedArtifact,
        },
    }
}

fn strict_log_name(input: Option<&str>) -> PathBuf {
    match input {
        Some(value) if is_all_arg(value) => PathBuf::from("corpus_emergence.strict-clippy.txt"),
        Some(value) => PathBuf::from(format!("{value}.strict-clippy.txt")),
        None => PathBuf::from("etuschlopf.strict-clippy.txt"),
    }
}


fn profile_build_version_stamp(profile: CargoBuildProfile) -> PathBuf {
    PathBuf::from(format!(
        "target/etuschlopf_last_successful_build.{}.version",
        profile.as_str()
    ))
}

fn profile_build_signature_stamp(profile: CargoBuildProfile) -> PathBuf {
    PathBuf::from(format!(
        "target/etuschlopf_last_successful_build.{}.signature",
        profile.as_str()
    ))
}

fn profile_build_artifact_stamp(profile: CargoBuildProfile) -> PathBuf {
    PathBuf::from(format!(
        "target/etuschlopf_last_successful_build.{}.artifact",
        profile.as_str()
    ))
}

fn profile_build_artifact_is_verified(
    version: &str,
    profile: CargoBuildProfile,
    current_signature: &str,
) -> bool {
    if !profile.target_exe().is_file() {
        return false;
    }
    let stamp_version =
        fs::read_to_string(profile_build_version_stamp(profile)).unwrap_or_default();
    if !stamp_version.trim().eq_ignore_ascii_case(version) {
        return false;
    }
    let stamp_signature =
        fs::read_to_string(profile_build_signature_stamp(profile)).unwrap_or_default();
    if stamp_signature.trim().is_empty() || stamp_signature.trim() != current_signature {
        return false;
    }
    let expected_artifact_signature =
        fs::read_to_string(profile_build_artifact_stamp(profile)).unwrap_or_default();
    if expected_artifact_signature.trim().is_empty() {
        return false;
    }
    file_fnv1a64(&profile.target_exe())
        .map(|actual| actual == expected_artifact_signature.trim())
        .unwrap_or(false)
}

fn file_fnv1a64(path: &Path) -> AppResult<String> {
    let bytes = fs::read(path)?;
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    fnv1a_update(&mut hash, &bytes);
    Ok(format!("fnv64:{hash:016x}"))
}

fn append_source_signature_files(hash: &mut u64, root: &Path, files: Vec<PathBuf>) -> AppResult<()> {
    for path in files {
        let rel = path.strip_prefix(root).unwrap_or(&path).to_string_lossy().replace('\\', "/");
        fnv1a_update(hash, rel.as_bytes());
        fnv1a_update(hash, b"\0");
        let bytes = fs::read(&path)?;
        fnv1a_update(hash, bytes.len().to_string().as_bytes());
        fnv1a_update(hash, b":");
        fnv1a_update(hash, &bytes);
        fnv1a_update(hash, b"\n");
    }
    Ok(())
}

#[cfg(test)]
mod cleanup_signature_tests {
    use super::*;

    #[test]
    fn source_signatures_keep_parent_byte_recipe() {
        let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("clock").as_nanos();
        let root = std::env::temp_dir().join(format!("etuschlopf-signature-recipe-{}-{unique}", std::process::id()));
        for (name, bytes) in [("Cargo.toml", "fixture\n"), ("src/lib.rs", "pub fn x() {}\n"), ("tests/probe.rs", "test\n"), ("docs/readme.txt", "doc\n")] {
            let path = root.join(name);
            fs::create_dir_all(path.parent().expect("parent")).expect("directory");
            fs::write(path, bytes).expect("fixture");
        }
        // Golden bytes from the parent's two independent serialization recipes.
        assert_eq!(source_build_signature(&root).expect("product"), "v0.10.1839:57edc34aab4adc34");
        assert_eq!(strict_all_targets_source_signature(&root).expect("all targets"), "v0.10.1839:57edc34aab4adc34:all-targets:1e7a310a6657cedb");
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn inventory_summary_keeps_existing_text() {
        let inventory = InventoryArtifacts { scope_label: "probe".to_string(), row_count: 15,
            class_counts: ArchCounts { a: 1, b: 2, c: 3, d: 4, open: 5 } };
        assert_eq!(inventory_summary_text(&inventory), "probe · 15 Zeilen · A 1 / B 2 / C 3 / D 4 / OFFEN 5");
    }
}

fn source_build_signature(root: &Path) -> AppResult<String> {
    let _timing = LauncherPhaseTimer::new("SourceBuildSignature");
    let mut files = Vec::new();
    for name in ["Cargo.toml", "Cargo.lock", "build.rs", "clippy.toml", "rustfmt.toml", "rust-toolchain.toml"] {
        let path = root.join(name);
        if path.exists() && path.is_file() {
            files.push(path);
        }
    }
    let kodex_gate_core = kodex_gate_core_source_root(root);
    collect_rs_files(&root.join("src"), &mut files)?;
    collect_rs_files(&kodex_gate_core, &mut files)?;
    files.sort();
    files.dedup();
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    append_source_signature_files(&mut hash, root, files)?;
    Ok(format!("v{}:{hash:016x}", env!("CARGO_PKG_VERSION")))
}

fn strict_all_targets_source_signature(root: &Path) -> AppResult<String> {
    let _timing = LauncherPhaseTimer::new("AllTargetsSourceSignature");
    let source_signature = source_build_signature(root)?;
    let mut files = Vec::new();
    for relative in managed_source_tree_definition::MANAGED_DIRS
        .into_iter()
        .chain(["tests", "examples", "benches"])
    {
        collect_regular_files(&root.join(relative), &mut files)?;
    }
    for relative in managed_source_tree_definition::MANAGED_ROOT_FILES {
        let path = root.join(relative);
        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.is_file() && !metadata.file_type().is_symlink() => {
                files.push(path);
            }
            Ok(_) => return Err(anyhow!(
                "Strict-All-Targets-Source-Signatur verweigert nicht regulaere Root-Datei {}",
                path.display()
            )),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    files.sort();
    files.dedup();

    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    fnv1a_update(&mut hash, b"SourceBuildSignature\0");
    fnv1a_update(&mut hash, source_signature.as_bytes());
    fnv1a_update(&mut hash, b"\n");
    append_source_signature_files(&mut hash, root, files)?;
    Ok(format!("{source_signature}:all-targets:{hash:016x}"))
}

fn collect_regular_files(dir: &Path, files: &mut Vec<PathBuf>) -> AppResult<()> {
    match fs::symlink_metadata(dir) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
        Ok(_) => return Err(anyhow!(
            "Strict-All-Targets-Source-Signatur verweigert nicht regulaeren Ordner {}",
            dir.display()
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    }
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        entries.push(entry?.path());
    }
    entries.sort();
    for path in entries {
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(anyhow!(
                "Strict-All-Targets-Source-Signatur verweigert Symlink {}",
                path.display()
            ));
        }
        if metadata.is_dir() {
            collect_regular_files(&path, files)?;
        } else if metadata.is_file() {
            files.push(path);
        } else {
            return Err(anyhow!(
                "Strict-All-Targets-Source-Signatur verweigert unbekannten Dateityp {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) -> AppResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        entries.push(entry?.path());
    }
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rs_files(&path, files)?;
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn fnv1a_update(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
}

fn write_strict_log_header(path: &Path, version: &str, cargo_jobs: usize) -> AppResult<()> {
    let mut file = File::create(path)?;
    writeln!(file, "ETUSCHLOPF STRICT CLIPPY LOG")?;
    writeln!(file, "=============================")?;
    writeln!(file, "Version: {version}")?;
    writeln!(
        file,
        "Befehl: cargo clippy --locked --profile strict --jobs {cargo_jobs} --color never -- -D clippy::correctness -D clippy::suspicious -D clippy::perf"
    )?;
    writeln!(file, "CargoJobsExplicit: {cargo_jobs}")?;
    writeln!(file)?;
    Ok(())
}

fn invoke_logged_process(
    launcher: &mut Launcher,
    exe: &str,
    args: &[String],
    log_path: &Path,
    header: &str,
    progress_role: PresentationRole,
    heartbeat: &str,
) -> AppResult<RunResult> {
    let os_args: Vec<OsString> = args.iter().map(OsString::from).collect();
    invoke_logged_program_with_env(
        launcher,
        LoggedProcessSpec {
            exe: Path::new(exe),
            args: &os_args,
            log_path,
            header,
            progress_role,
            heartbeat,
            completion_summary: CompletionSummary::LogSize,
        },
        ProcessEnvMode::NormalCargoTarget,
    )
}

fn write_clippy_warning_budget(path: &Path, elapsed_ms: u128) -> AppResult<usize> {
    let text = fs::read_to_string(path)?;
    let raw_warnings = text
        .lines()
        .filter(|line| line.trim_start().starts_with("warning:"))
        .count();
    let summary_warnings = text
        .lines()
        .filter(|line| is_cargo_warning_summary_line(line))
        .count();
    let total = warning_diagnostic_count(&text);
    let too_many = text.matches("#too_many_arguments").count();
    let type_complexity = text.matches("#type_complexity").count();
    let field_reassign = text.matches("#field_reassign_with_default").count();
    let range_loop = text.matches("#needless_range_loop").count();
    let architectural_budget = too_many + type_complexity;
    let known = too_many + type_complexity + field_reassign + range_loop;
    let small_mechanical_budget = total.saturating_sub(known);

    let mut file = OpenOptions::new().append(true).open(path)?;
    writeln!(file)?;
    writeln!(file, "CLIPPY-WARNING-BUDGET")?;
    writeln!(file, "---------------------")?;
    writeln!(file, "TotalWarnings: {total}")?;
    writeln!(file, "RawWarningLines: {raw_warnings}")?;
    writeln!(file, "SummaryWarningLinesExcluded: {summary_warnings}")?;
    writeln!(file, "too_many_arguments: {too_many}")?;
    writeln!(file, "type_complexity: {type_complexity}")?;
    writeln!(file, "field_reassign_with_default: {field_reassign}")?;
    writeln!(file, "needless_range_loop: {range_loop}")?;
    writeln!(file, "architectural_budget: {architectural_budget}")?;
    writeln!(file, "small_mechanical_budget: {small_mechanical_budget}")?;
    writeln!(file, "LauncherClippyElapsedMs: {elapsed_ms}")?;
    writeln!(
        file,
        "Decision: zero_warning_receipt_gate_fail_closed; unreadable_log_is_error."
    )?;
    Ok(total)
}
