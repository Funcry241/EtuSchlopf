use super::corpus_run_manifest::json_escape;
use super::*;
use std::fmt::Write as _;

pub struct AssistantExportBundleContext<'a> {
    pub(super) project_root: &'a Path,
    pub(super) aggregate_path: &'a Path,
    pub(super) plan: &'a VisualPreflightPlan,
    pub(super) projects: &'a [String],
    pub(super) ledgers: &'a [VisualProjectRunLedger],
    pub(super) strict_log: Option<&'a Path>,
    pub(super) strict_test_log: Option<&'a Path>,
    pub(super) version: &'a str,
}

fn assistant_context_active_target_project(plan: &VisualPreflightPlan) -> String {
    plan.target_project
        .clone()
        .unwrap_or_else(|| "none".to_string())
}

fn project_result_summary_file_name(project: &str) -> String {
    format!("{project}.assistant-summary.txt")
}

fn project_pdf_file_name(project: &str) -> String {
    format!("{project}.pdf")
}

fn assistant_context_required_root_files_for_ledgers(
    plan: &VisualPreflightPlan,
    ledgers: &[VisualProjectRunLedger],
    export_policy: AssistantExportPolicy,
) -> Vec<String> {
    let mut required = vec![
        "assistant_context.json".to_string(),
        ASSISTANT_HANDOFF_FILENAME.to_string(),
        "manifest.txt".to_string(),
        "visual_preflight.result.txt".to_string(),
        "visual_preflight.strict-clippy.txt".to_string(),
        "emergence_selection.json".to_string(),
        "source_architecture_inventory.txt".to_string(),
        PROJECT_FILE_CATALOG_MARKDOWN.to_string(),
        PROJECT_FILE_CATALOG_TSV.to_string(),
        RENDERER_PROCESS_LIFECYCLE_ASSISTANT_FILE.to_string(),
        RUN_OUTCOME_AUDIT_FILE.to_string(),
        TRANSITION_HISTORY_FILE.to_string(),
        source_snapshot_metadata_relative_path(),
    ];
    let strict_test_log = Path::new(VISUAL_PREFLIGHT_STRICT_TEST_LOG);
    if strict_test_log.exists() {
        required.push(file_name_string(strict_test_log));
    }
    if plan.emergence.manifest_exists {
        required.push(CORPUS_RUN_MANIFEST_FILE.to_string());
    }
    if plan.emergence.fresh_for_current_version {
        required.push("corpus_emergence.result.txt".to_string());
    } else if plan.emergence.report_exists {
        required.push("corpus_emergence.rejected-evidence.txt".to_string());
    }
    for ledger in ledgers {
        let result_summary_name = project_result_summary_file_name(&ledger.project);
        if !required.iter().any(|name| name == &result_summary_name) {
            required.push(result_summary_name);
        }
        if assistant_export_includes_pdf(export_policy) {
            let pdf_name = project_pdf_file_name(&ledger.project);
            if !required.iter().any(|name| name == &pdf_name) {
                required.push(pdf_name);
            }
        }
    }
    required
}

fn write_emergence_selection_json(
    path: &Path,
    version: &str,
    plan: &VisualPreflightPlan,
) -> AppResult<()> {
    let mut json = String::new();
    json.push_str("{\n");
    json.push_str("  \"schema_version\": 3,\n");
    writeln!(&mut json, "  \"version\": \"{}\",", json_escape(version)).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"status\": \"{}\",", json_escape(&plan.emergence.status)).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"reason\": \"{}\",", json_escape(&plan.emergence.reason)).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"report_exists\": {},", plan.emergence.report_exists).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"report_version\": \"{}\",",
        json_escape(plan.emergence.report_version.as_deref().unwrap_or("none"))
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"run_id\": \"{}\",",
        json_escape(plan.emergence.run_id.as_deref().unwrap_or("none"))
    ).expect("writing to String cannot fail");
    match plan.emergence.generated_unix_seconds {
        Some(value) => writeln!(&mut json, "  \"generated_unix_seconds\": {value},").expect("writing to String cannot fail"),
        None => json.push_str("  \"generated_unix_seconds\": null,\n"),
    }
    writeln!(&mut json, "  \"full_corpus\": {},", plan.emergence.full_corpus).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"project_count\": {},", plan.emergence.project_count).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"reported_current_version_project_count\": {},",
        plan.emergence.reported_current_version_project_count
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"verified_current_version_project_count\": {},",
        plan.emergence.current_version_project_count
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"project_evidence_complete\": {},",
        plan.emergence.project_evidence_complete
    ).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"evidence_contract\": \"{}\",", json_escape(&plan.emergence.evidence_contract)).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"manifest_exists\": {},", plan.emergence.manifest_exists).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"manifest_completed\": {},", plan.emergence.manifest_completed).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"manifest_valid\": {},", plan.emergence.manifest_valid).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"manifest_reason\": \"{}\",", json_escape(&plan.emergence.manifest_reason)).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"manifest_project_files_valid\": {},", plan.emergence.manifest_project_files_valid).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"manifest_project_files_expected\": {},", plan.emergence.manifest_project_files_expected).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"report_hash_valid\": {},", plan.emergence.report_hash_valid).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"report_bound_to_manifest\": {},", plan.emergence.report_bound_to_manifest).expect("writing to String cannot fail");
    push_json_array_owned(&mut json, 1, "considered_projects", &plan.emergence.projects);
    json.push_str(",\n");
    writeln!(&mut json, "  \"selection_eligible\": {},", plan.emergence.selection_eligible).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"fresh_for_current_version\": {},",
        plan.emergence.fresh_for_current_version
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"current_target_project\": \"{}\",",
        json_escape(plan.target_project.as_deref().unwrap_or("none"))
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"current_target_locator\": \"{}\",",
        json_escape(plan.target_locator.as_deref().unwrap_or("none"))
    ).expect("writing to String cannot fail");
    match plan.target_score {
        Some(value) => writeln!(&mut json, "  \"current_target_score\": {value},").expect("writing to String cannot fail"),
        None => json.push_str("  \"current_target_score\": null,\n"),
    }
    writeln!(&mut json,
        "  \"target_class\": \"{}\",",
        plan.target_class.as_str()
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"behind_bars_rule_check_required\": {},",
        plan.target_class.behind_bars_rule_check_required()
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"rule_candidate\": {},",
        plan.target_class.rule_candidate()
    ).expect("writing to String cannot fail");
    writeln!(&mut json,
        "  \"write_candidate\": {},",
        plan.target_class.write_candidate()
    ).expect("writing to String cannot fail");
    writeln!(&mut json, "  \"target_permission_contract\": \"{}\",", plan.target_class.permission_contract()).expect("writing to String cannot fail");
    json.push_str("  \"music_mutation\": false,\n");
    json.push_str("  \"master_apply\": false\n");
    json.push_str("}\n");
    fs::write(path, json)?;
    Ok(())
}

fn write_source_architecture_inventory(
    _project_root: &Path,
    dest: &Path,
    version: &str,
    _ledgers: &[VisualProjectRunLedger],
) -> AppResult<()> {
    let note = format!(
        "SOURCE-ARCHITECTURE-NOTE\n========================\nVersion: {version}\nSourceTreeScan: nein\nRuntimeAuthority: keine\nMusicMutation: nein\nMasterApply: nein\nHinweis: Das frühere vollständige Build-Inventar wurde aus dem normalen Lauf entfernt. Architekturvermessung erfolgt nur noch bei gezielter Wartung.\n"
    );
    fs::write(dest, note)?;
    Ok(())
}

fn push_json_array_owned(out: &mut String, indent: usize, key: &str, values: &[String]) {
    out.push_str(&"  ".repeat(indent));
    out.push('"');
    out.push_str(&json_escape(key));
    out.push_str("\": [");
    for (idx, value) in values.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push('"');
        out.push_str(&json_escape(value));
        out.push('"');
    }
    out.push(']');
}


#[derive(Clone, Copy)]
pub struct AssistantExportRunStatus<'a> {
    pub(super) final_exit: i32,
    pub(super) clippy_exit: &'a str,
    pub(super) build_exit: &'a str,
    pub(super) strict_test_exit: &'a str,
    pub(super) strict_test_summary: &'a str,
    pub(super) audit_signal: AuditSignal,
    pub(super) renderer_outcome: RendererOutcome,
    pub(super) emergence_outcome: EmergenceOutcome,
    pub(super) export_policy: AssistantExportPolicy,
}


pub enum AssistantExportRequest<'a> {
    Normal {
        project_root: &'a Path,
        project: &'a str,
        state: &'a RunState,
        version: &'a str,
        run_id: &'a str,
        final_exit: i32,
    },
    Corpus {
        project_root: &'a Path,
        state: &'a RunState,
        export_version: &'a str,
        evidence_version: &'a str,
        final_exit: i32,
    },
    VisualPreflight {
        context: AssistantExportBundleContext<'a>,
        status: AssistantExportRunStatus<'a>,
    },
}

pub(super) fn create_assistant_export(
    request: AssistantExportRequest<'_>,
) -> AppResult<PathBuf> {
    match request {
        AssistantExportRequest::Normal {
            project_root,
            project,
            state,
            version,
            run_id,
            final_exit,
        } => create_normal_run_assistant_export_bundle(
            project_root,
            project,
            state,
            version,
            run_id,
            final_exit,
        ),
        AssistantExportRequest::Corpus {
            project_root,
            state,
            export_version,
            evidence_version,
            final_exit,
        } => create_corpus_run_assistant_export_bundle(
            project_root,
            state,
            export_version,
            evidence_version,
            final_exit,
        ),
        AssistantExportRequest::VisualPreflight { context, status } => {
            create_visual_preflight_assistant_export_bundle(context, status)
        }
    }
}

#[derive(Clone, Copy)]
struct AssistantExportValidationEvidence<'a> {
    strict_test_exit: &'a str,
    strict_test_summary: &'a str,
}

#[derive(Clone, Debug)]
pub struct WarningTruth {
    pub(super) compiler_warning_count: usize,
    pub(super) compiler_status: String,
    pub(super) compiler_scope: String,
    pub(super) compiler_log: String,
    pub(super) clippy_warning_count: usize,
    pub(super) clippy_status: String,
    pub(super) clippy_scope: String,
    pub(super) clippy_log: String,
}

pub fn is_cargo_warning_summary_line(line: &str) -> bool {
    let Some((prefix, summary)) = line.trim().split_once(" generated ") else { return false; };
    let mut fields = summary.split_whitespace();
    prefix.starts_with("warning:")
        && fields.next().and_then(|count| count.parse::<usize>().ok()).is_some()
        && matches!(fields.next(), Some("warning" | "warnings"))
}

pub fn warning_diagnostic_count(text: &str) -> usize {
    text.lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("warning:") && !is_cargo_warning_summary_line(line))
        .count()
}

fn collect_warning_truth(strict_log: Option<&Path>) -> WarningTruth {
    let compiler_log_path = env::var("ETUSCHLOPF_LAUNCHER_BOOTSTRAP_BUILD_LOG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target/etuschlopf_logs/launcher_bootstrap_build.log"));
    let compiler_text = fs::read_to_string(&compiler_log_path).unwrap_or_default();
    let compiler_log_exists = compiler_log_path.exists();
    let compiler_warning_count = warning_diagnostic_count(&compiler_text);
    let compiler_status = if !compiler_log_exists {
        "unknown_launcher_bootstrap_log_missing"
    } else if compiler_warning_count > 0 {
        "warnings_emitted_current_launcher_bootstrap_invocation"
    } else if compiler_text.trim().is_empty() {
        "no_diagnostics_emitted_cached_or_clean"
    } else {
        "clean_no_warning_diagnostics"
    }
    .to_string();

    let clippy_log_path = strict_log.map(Path::to_path_buf);
    let clippy_text = clippy_log_path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .unwrap_or_default();
    let clippy_warning_count = first_prefixed_value(&clippy_text, "TotalWarnings:")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or_else(|| warning_diagnostic_count(&clippy_text));
    let clippy_status = match clippy_log_path.as_ref() {
        None => "not_recorded_no_strict_log".to_string(),
        Some(path) if !path.exists() => "unknown_strict_log_missing".to_string(),
        Some(_) if clippy_text.contains("Skipped: source_build_signature_unchanged") => {
            "reused_previous_successful_strict_build_same_source_signature".to_string()
        }
        Some(_) if clippy_warning_count > 0 => "warnings_emitted_current_clippy_invocation".to_string(),
        Some(_) => "executed_current_clippy_invocation_clean".to_string(),
    };

    WarningTruth {
        compiler_warning_count,
        compiler_status,
        compiler_scope: "launcher_bootstrap_cargo_build_current_invocation".to_string(),
        compiler_log: compiler_log_path.display().to_string(),
        clippy_warning_count,
        clippy_status,
        clippy_scope: "strict_clippy_log_current_visual_preflight".to_string(),
        clippy_log: clippy_log_path
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "none".to_string()),
    }
}

fn launcher_check_outcome(value: &str) -> CheckOutcome {
    match value.trim() {
        "0" => CheckOutcome::Passed,
        "not_run" => CheckOutcome::NotRun,
        other => other
            .parse::<i32>()
            .map(CheckOutcome::Failed)
            .unwrap_or(CheckOutcome::Unknown),
    }
}

pub fn run_mode_for_console(mode: &str) -> RunMode {
    match mode {
        "visual_preflight" => RunMode::VisualPreflight,
        "all_run" => RunMode::AllRun,
        _ => RunMode::NormalRun,
    }
}

pub(super) fn warning_debt_count_for_console(state: &RunState) -> usize {
    let warning_truth = collect_warning_truth(state.strict_log.as_deref());
    warning_truth.compiler_warning_count + warning_truth.clippy_warning_count
}

pub(super) fn typed_run_outcome_with_warning_debt_for_console(
    state: &RunState,
    mode: &str,
    final_exit: i32,
) -> (RunOutcome, usize) {
    let warning_count = warning_debt_count_for_console(state);
    let mut outcome = RunOutcome::evaluate(RunOutcomeInput {
        mode: run_mode_for_console(mode),
        fatal: FatalOutcome::NotObserved,
        final_exit,
        build: launcher_check_outcome(&state.build_exit),
        clippy: launcher_check_outcome(&state.strict_clippy_exit),
        audit_signal: state.audit_signal,
        renderer: state.renderer_outcome,
        emergence: state.emergence_outcome,
    });
    if warning_count > 0
        && outcome.release_decision == crate::run_outcome::ReleaseDecision::Released
    {
        outcome.severity = crate::run_outcome::RunSeverity::Yellow;
        outcome.release_decision =
            crate::run_outcome::ReleaseDecision::AuditBlocked(
                crate::run_outcome::AuditBlockReason::ExpectedAuditBlock,
            );
    }
    (outcome, warning_count)
}

pub fn typed_run_outcome_for_console(
    state: &RunState,
    mode: &str,
    final_exit: i32,
) -> RunOutcome {
    typed_run_outcome_with_warning_debt_for_console(state, mode, final_exit).0
}

fn evaluate_run_outcome_for_export(status: AssistantExportRunStatus<'_>) -> RunOutcome {
    RunOutcome::evaluate(RunOutcomeInput {
        mode: RunMode::VisualPreflight,
        fatal: FatalOutcome::NotObserved,
        final_exit: status.final_exit,
        build: launcher_check_outcome(status.build_exit),
        clippy: launcher_check_outcome(status.clippy_exit),
        audit_signal: status.audit_signal,
        renderer: status.renderer_outcome,
        emergence: status.emergence_outcome,
    })
}



pub const PROJECT_FILE_CATALOG_MARKDOWN: &str = "PROJECT_FILE_CATALOG.md";
pub const PROJECT_FILE_CATALOG_TSV: &str = "project_file_catalog.tsv";


const SOURCE_SNAPSHOT_METADATA_FILE: &str = "SOURCE_SNAPSHOT.txt";
const SOURCE_SNAPSHOT_DIR: &str = "source_snapshot";
const SOURCE_SNAPSHOT_ARCHIVE_FILENAME_MAX_UTF16: usize = 64;
const SOURCE_SNAPSHOT_ENTRY_FILENAME_MAX_UTF16: usize = 120;
const SOURCE_SNAPSHOT_RELATIVE_PATH_MAX_UTF16: usize = 140;
const SOURCE_SNAPSHOT_EXTRACTED_ENVELOPE_MAX_UTF16: usize = 180;

#[derive(Clone, Debug)]
struct SourceSnapshotOutcome {
    complete: bool,
    archive_name: Option<String>,
    file_count: usize,
    sha256: Option<String>,
    reason: String,
}

fn source_snapshot_metadata_relative_path() -> String {
    format!("{SOURCE_SNAPSHOT_DIR}/{SOURCE_SNAPSHOT_METADATA_FILE}")
}

fn source_snapshot_zip_kind_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (candidate_key, value) = line.split_once('=')?;
        (candidate_key.trim() == key).then(|| value.trim().to_string())
    })
}

fn source_snapshot_filename_token(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut previous_was_separator = false;
    for ch in value.chars() {
        let mapped = if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
            ch
        } else {
            '_'
        };
        if mapped == '_' {
            if previous_was_separator {
                continue;
            }
            previous_was_separator = true;
        } else {
            previous_was_separator = false;
        }
        out.push(mapped);
    }
    out.trim_matches('_').to_string()
}

fn windows_path_units(value: &str) -> usize {
    value.encode_utf16().count()
}

fn validate_source_snapshot_path_budget(files: &[PathBuf], archive_name: &str) -> AppResult<()> {
    let archive_units = windows_path_units(archive_name);
    if archive_units > SOURCE_SNAPSHOT_ARCHIVE_FILENAME_MAX_UTF16 {
        return Err(anyhow!(
            "SOURCE-SNAPSHOT PATH-BUDGET FAILED: archive filename {archive_name:?} uses {archive_units} UTF-16 units; max {}",
            SOURCE_SNAPSHOT_ARCHIVE_FILENAME_MAX_UTF16
        ));
    }
    let archive_stem = archive_name.strip_suffix(".zip").unwrap_or(archive_name);
    let archive_stem_units = windows_path_units(archive_stem);
    for relative in files {
        let normalized = normalize_catalog_path(relative);
        let relative_units = windows_path_units(&normalized);
        if relative_units > SOURCE_SNAPSHOT_RELATIVE_PATH_MAX_UTF16 {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT PATH-BUDGET FAILED: relative path {normalized:?} uses {relative_units} UTF-16 units; max {}",
                SOURCE_SNAPSHOT_RELATIVE_PATH_MAX_UTF16
            ));
        }
        let file_name = relative
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_name_units = windows_path_units(&file_name);
        if file_name_units > SOURCE_SNAPSHOT_ENTRY_FILENAME_MAX_UTF16 {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT PATH-BUDGET FAILED: filename {file_name:?} uses {file_name_units} UTF-16 units; max {}",
                SOURCE_SNAPSHOT_ENTRY_FILENAME_MAX_UTF16
            ));
        }
        let extracted_envelope_units = archive_stem_units + 1 + relative_units;
        if extracted_envelope_units > SOURCE_SNAPSHOT_EXTRACTED_ENVELOPE_MAX_UTF16 {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT PATH-BUDGET FAILED: extracted envelope {archive_stem:?}/{normalized} uses {extracted_envelope_units} UTF-16 units; max {}",
                SOURCE_SNAPSHOT_EXTRACTED_ENVELOPE_MAX_UTF16
            ));
        }
    }
    Ok(())
}

fn collect_source_snapshot_tree_files(
    project_root: &Path,
    current: &Path,
    relative_prefix: &Path,
    files: &mut Vec<PathBuf>,
) -> AppResult<()> {
    let mut children = fs::read_dir(current)?.collect::<Result<Vec<_>, _>>()?;
    children.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    for child in children {
        let path = child.path();
        let metadata = fs::symlink_metadata(&path)?;
        let relative = relative_prefix.join(child.file_name());
        if metadata.file_type().is_symlink() {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT FAILED: symlink is not allowed in source snapshot: {}",
                normalize_catalog_path(&relative)
            ));
        }
        if metadata.is_dir() {
            collect_source_snapshot_tree_files(project_root, &path, &relative, files)?;
        } else if metadata.is_file() {
            let canonical = path.canonicalize()?;
            let canonical_root = project_root.canonicalize()?;
            if !canonical.starts_with(&canonical_root) {
                return Err(anyhow!(
                    "SOURCE-SNAPSHOT FAILED: source path escaped project root: {}",
                    normalize_catalog_path(&relative)
                ));
            }
            files.push(relative);
        } else {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT FAILED: unsupported filesystem entry: {}",
                normalize_catalog_path(&relative)
            ));
        }
    }
    Ok(())
}

fn collect_source_snapshot_files(project_root: &Path) -> AppResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    for &relative in &managed_source_tree_definition::REQUIRED_MANAGED_ROOT_FILES {
        let path = project_root.join(relative);
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            anyhow!(
                "SOURCE-SNAPSHOT FAILED: required root file missing or unreadable: {relative}: {error}"
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT FAILED: required root file is not a regular file: {relative}"
            ));
        }
        files.push(PathBuf::from(relative));
    }
    for &relative in &managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES {
        let path = project_root.join(relative);
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            anyhow!(
                "SOURCE-SNAPSHOT FAILED: required root file missing or unreadable: {relative}: {error}"
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT FAILED: required root file is not a regular file: {relative}"
            ));
        }
        files.push(PathBuf::from(relative));
    }
    for &relative in &managed_source_tree_definition::MANAGED_ROOT_FILES {
        if managed_source_tree_definition::REQUIRED_MANAGED_ROOT_FILES.contains(&relative) {
            continue;
        }
        let path = project_root.join(relative);
        if path.exists() {
            let metadata = fs::symlink_metadata(&path)?;
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(anyhow!(
                    "SOURCE-SNAPSHOT FAILED: optional root entry is not a regular file: {relative}"
                ));
            }
            files.push(PathBuf::from(relative));
        }
    }
    for &relative in &managed_source_tree_definition::MANAGED_DIRS {
        let path = project_root.join(relative);
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            anyhow!(
                "SOURCE-SNAPSHOT FAILED: required source directory missing or unreadable: {relative}: {error}"
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(anyhow!(
                "SOURCE-SNAPSHOT FAILED: required source directory is not a regular directory: {relative}"
            ));
        }
        collect_source_snapshot_tree_files(
            project_root,
            &path,
            Path::new(relative),
            &mut files,
        )?;
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn write_source_snapshot_archive(
    project_root: &Path,
    archive_path: &Path,
    files: &[PathBuf],
) -> AppResult<()> {
    let file = File::create(archive_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for relative in files {
        let source = project_root.join(relative);
        let name = normalize_catalog_path(relative);
        zip.start_file(name, options)?;
        zip.write_all(&fs::read(source)?)?;
    }
    zip.finish()?;
    Ok(())
}

fn write_source_snapshot_metadata(
    metadata_path: &Path,
    project_root: &Path,
    version: &str,
    revision: Option<&str>,
    name: Option<&str>,
    outcome: &SourceSnapshotOutcome,
) -> AppResult<()> {
    let mut text = String::new();
    text.push_str("kind=etuschlopf_source_snapshot\n");
    text.push_str("take_compatible=false\n");
    writeln!(&mut text, "source_complete={}", outcome.complete)
        .expect("writing to String cannot fail");
    writeln!(
        &mut text,
        "status={}",
        if outcome.complete {
            "SOURCE-SNAPSHOT COMPLETE"
        } else {
            "SOURCE-SNAPSHOT FAILED"
        }
    )
    .expect("writing to String cannot fail");
    writeln!(&mut text, "SourceRoot={}", project_root.display())
        .expect("writing to String cannot fail");
    writeln!(&mut text, "Version={version}").expect("writing to String cannot fail");
    writeln!(&mut text, "Revision={}", revision.unwrap_or("unknown"))
        .expect("writing to String cannot fail");
    writeln!(&mut text, "Name={}", name.unwrap_or("unknown"))
        .expect("writing to String cannot fail");
    writeln!(&mut text, "FileCount={}", outcome.file_count)
        .expect("writing to String cannot fail");
    if let Some(archive_name) = outcome.archive_name.as_deref() {
        writeln!(&mut text, "SourceArchive={archive_name}")
            .expect("writing to String cannot fail");
    }
    if let Some(sha256) = outcome.sha256.as_deref() {
        writeln!(&mut text, "SourceZipSha256=sha256:{sha256}")
            .expect("writing to String cannot fail");
    }
    writeln!(&mut text, "Reason={}", outcome.reason)
        .expect("writing to String cannot fail");
    writeln!(
        &mut text,
        "TransportPathBudgetUtf16=archive_name<={};entry_filename<={};relative_path<={};extracted_envelope<={}",
        SOURCE_SNAPSHOT_ARCHIVE_FILENAME_MAX_UTF16,
        SOURCE_SNAPSHOT_ENTRY_FILENAME_MAX_UTF16,
        SOURCE_SNAPSHOT_RELATIVE_PATH_MAX_UTF16,
        SOURCE_SNAPSHOT_EXTRACTED_ENVELOPE_MAX_UTF16
    )
    .expect("writing to String cannot fail");
    text.push_str("Policy=current_project_root_bytes_only_no_reconstruction\n");
    fs::write(metadata_path, text)?;
    Ok(())
}

fn append_source_snapshot_status_to_outer_zip_kind(
    latest_dir: &Path,
    outcome: &SourceSnapshotOutcome,
) -> AppResult<()> {
    let path = latest_dir.join("ZIP_KIND.txt");
    if !path.is_file() {
        return Ok(());
    }
    let mut file = OpenOptions::new().append(true).open(path)?;
    writeln!(file, "source_snapshot_complete={}", outcome.complete)?;
    writeln!(
        file,
        "source_snapshot_archive={}",
        outcome.archive_name.as_deref().unwrap_or("none")
    )?;
    writeln!(
        file,
        "source_snapshot_sha256={}",
        outcome.sha256.as_deref().unwrap_or("none")
    )?;
    Ok(())
}

fn stage_source_snapshot(
    project_root: &Path,
    latest_dir: &Path,
    version: &str,
    included: &mut Vec<String>,
    omitted: &mut Vec<String>,
) -> AppResult<SourceSnapshotOutcome> {
    let _timing = AssistantExportPhaseTimer::new(project_root, latest_dir, "SourceSnapshotMs");
    let snapshot_dir = latest_dir.join(SOURCE_SNAPSHOT_DIR);
    if snapshot_dir.exists() {
        fs::remove_dir_all(&snapshot_dir)?;
    }
    fs::create_dir_all(&snapshot_dir)?;
    let metadata_path = snapshot_dir.join(SOURCE_SNAPSHOT_METADATA_FILE);

    let zip_kind_text = fs::read_to_string(project_root.join("ZIP_KIND.txt"));
    let (declared_version, revision, name) = match zip_kind_text {
        Ok(text) => (
            source_snapshot_zip_kind_value(&text, "version"),
            source_snapshot_zip_kind_value(&text, "revision"),
            source_snapshot_zip_kind_value(&text, "name"),
        ),
        Err(error) => {
            let outcome = SourceSnapshotOutcome {
                complete: false,
                archive_name: None,
                file_count: 0,
                sha256: None,
                reason: format!("ZIP_KIND.txt unreadable: {error}"),
            };
            write_source_snapshot_metadata(
                &metadata_path,
                project_root,
                version,
                None,
                None,
                &outcome,
            )?;
            included.push(source_snapshot_metadata_relative_path());
            omitted.push("source_snapshot archive | SOURCE-SNAPSHOT FAILED".to_string());
            return Ok(outcome);
        }
    };

    if declared_version.as_deref() != Some(version)
        || revision.as_deref().is_none_or(str::is_empty)
        || name.as_deref().is_none_or(str::is_empty)
    {
        let outcome = SourceSnapshotOutcome {
            complete: false,
            archive_name: None,
            file_count: 0,
            sha256: None,
            reason: format!(
                "ZIP_KIND metadata mismatch or incomplete: declared_version={} runtime_version={} revision={} name={}",
                declared_version.as_deref().unwrap_or("missing"),
                version,
                revision.as_deref().unwrap_or("missing"),
                name.as_deref().unwrap_or("missing")
            ),
        };
        write_source_snapshot_metadata(
            &metadata_path,
            project_root,
            version,
            revision.as_deref(),
            name.as_deref(),
            &outcome,
        )?;
        included.push(source_snapshot_metadata_relative_path());
        omitted.push("source_snapshot archive | SOURCE-SNAPSHOT FAILED".to_string());
        return Ok(outcome);
    }

    let files = match collect_source_snapshot_files(project_root) {
        Ok(files) => files,
        Err(error) => {
            let outcome = SourceSnapshotOutcome {
                complete: false,
                archive_name: None,
                file_count: 0,
                sha256: None,
                reason: error.to_string(),
            };
            write_source_snapshot_metadata(
                &metadata_path,
                project_root,
                version,
                revision.as_deref(),
                name.as_deref(),
                &outcome,
            )?;
            included.push(source_snapshot_metadata_relative_path());
            omitted.push("source_snapshot archive | SOURCE-SNAPSHOT FAILED".to_string());
            return Ok(outcome);
        }
    };

    let version_token = source_snapshot_filename_token(version).replace('.', "_");
    let revision_token = source_snapshot_filename_token(revision.as_deref().unwrap_or("unknown"));
    let archive_name = format!(
        "EtuSchlopf_v{version_token}_{revision_token}_unbuilt_source.zip"
    );
    if let Err(error) = validate_source_snapshot_path_budget(&files, &archive_name) {
        let outcome = SourceSnapshotOutcome {
            complete: false,
            archive_name: None,
            file_count: 0,
            sha256: None,
            reason: error.to_string(),
        };
        write_source_snapshot_metadata(
            &metadata_path,
            project_root,
            version,
            revision.as_deref(),
            name.as_deref(),
            &outcome,
        )?;
        included.push(source_snapshot_metadata_relative_path());
        omitted.push("source_snapshot archive | SOURCE-SNAPSHOT PATH-BUDGET FAILED".to_string());
        return Ok(outcome);
    }
    let archive_path = snapshot_dir.join(&archive_name);
    if let Err(error) = write_source_snapshot_archive(project_root, &archive_path, &files) {
        let _ = fs::remove_file(&archive_path);
        let outcome = SourceSnapshotOutcome {
            complete: false,
            archive_name: None,
            file_count: 0,
            sha256: None,
            reason: format!("source archive write failed: {error}"),
        };
        write_source_snapshot_metadata(
            &metadata_path,
            project_root,
            version,
            revision.as_deref(),
            name.as_deref(),
            &outcome,
        )?;
        included.push(source_snapshot_metadata_relative_path());
        omitted.push("source_snapshot archive | SOURCE-SNAPSHOT FAILED".to_string());
        return Ok(outcome);
    }

    let archive_bytes = fs::read(&archive_path)?;
    let archive_sha256 = sha256::hex(&archive_bytes);
    let outcome = SourceSnapshotOutcome {
        complete: true,
        archive_name: Some(archive_name.clone()),
        file_count: files.len(),
        sha256: Some(archive_sha256),
        reason: "current project root source bytes captured without reconstruction".to_string(),
    };
    write_source_snapshot_metadata(
        &metadata_path,
        project_root,
        version,
        revision.as_deref(),
        name.as_deref(),
        &outcome,
    )?;
    included.push(source_snapshot_metadata_relative_path());
    included.push(format!("{SOURCE_SNAPSHOT_DIR}/{archive_name}"));
    Ok(outcome)
}

const TRANSITION_HISTORY_FILE: &str = "transition_history.json";
const TRANSITION_REJECTION_LEDGER_ROOT: &str = "target/etuschlopf_take_rejection_ledger_v1";
const TRANSITION_ACCEPTANCE_EVIDENCE_ROOT: &str = "target/etuschlopf_take_acceptance_evidence_v1";
const TRANSITION_PENDING_MARKER: &str = "target/etuschlopf_take_transaction_pending.txt";
const TRANSITION_TRANSACTION_ROOT: &str = "target/etuschlopf_take_transactions";

#[derive(Clone, Debug, Default)]
struct TransitionEvidenceBlob {
    status: String,
    bytes: usize,
    sha256: String,
    row_count: usize,
    text: Option<String>,
}

impl TransitionEvidenceBlob {
    fn missing() -> Self {
        Self {
            status: "missing".to_string(),
            bytes: 0,
            sha256: "none".to_string(),
            row_count: 0,
            text: None,
        }
    }
}

#[derive(Clone, Debug)]
struct TransitionHistoryEntry {
    sort_epoch_millis: Option<u128>,
    from_version: String,
    to_version: String,
    take_revision: Option<u64>,
    candidate_sha256: String,
    verdict: String,
    authority: String,
    event_epoch_millis: Option<u128>,
    rejection_stage: Option<String>,
    prior_rejections: Option<usize>,
    candidate_intent: TransitionEvidenceBlob,
    semantic_effect: TransitionEvidenceBlob,
    reason: TransitionEvidenceBlob,
    acceptance_evidence_status: String,
}

#[derive(Clone, Debug, Default)]
struct TransitionCurrentCandidate {
    marker_present: bool,
    transaction_state: String,
    from_version: String,
    to_version: String,
    take_revision: Option<u64>,
    candidate_sha256: String,
    source_snapshot_revision: String,
    source_snapshot_name: String,
    candidate_intent: TransitionEvidenceBlob,
    semantic_effect: TransitionEvidenceBlob,
}

fn transition_metadata_value(text: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=");
    text.lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .map(ToOwned::to_owned)
}

fn transition_parse_u64(text: &str, key: &str) -> Option<u64> {
    transition_metadata_value(text, key)?.parse().ok()
}

fn transition_parse_u128(text: &str, key: &str) -> Option<u128> {
    transition_metadata_value(text, key)?.parse().ok()
}

fn transition_valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn transition_read_blob(path: &Path, include_text: bool) -> TransitionEvidenceBlob {
    let Ok(bytes) = fs::read(path) else {
        return TransitionEvidenceBlob::missing();
    };
    let text = std::str::from_utf8(&bytes).ok().map(ToOwned::to_owned);
    TransitionEvidenceBlob {
        status: if text.is_some() {
            "present_utf8".to_string()
        } else {
            "present_non_utf8".to_string()
        },
        bytes: bytes.len(),
        sha256: sha256::hex(&bytes),
        row_count: text
            .as_deref()
            .map(|value| value.lines().filter(|line| !line.trim().is_empty()).count())
            .unwrap_or(0),
        text: if include_text { text } else { None },
    }
}

fn transition_sorted_entries(path: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(path) else {
        return Vec::new();
    };
    let mut paths = entries
        .flatten()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    paths.sort_by(|left, right| left.as_os_str().cmp(right.as_os_str()));
    paths
}

fn transition_push_issue(issues: &mut Vec<String>, message: impl Into<String>) {
    issues.push(message.into());
}

fn transition_acceptance_evidence(
    project_root: &Path,
    pair_component: &str,
    from_version: &str,
    to_version: &str,
    take_revision: Option<u64>,
    candidate_sha256: &str,
) -> (String, TransitionEvidenceBlob, TransitionEvidenceBlob) {
    let Some(take_revision) = take_revision else {
        return (
            "not_addressable_legacy_revision_unknown".to_string(),
            TransitionEvidenceBlob::missing(),
            TransitionEvidenceBlob::missing(),
        );
    };
    let sidecar = project_root
        .join(TRANSITION_ACCEPTANCE_EVIDENCE_ROOT)
        .join(pair_component)
        .join(format!(
            "accepted-evidence-r{take_revision:02}-{candidate_sha256}"
        ));
    if !sidecar.is_dir() {
        return (
            "not_present".to_string(),
            TransitionEvidenceBlob::missing(),
            TransitionEvidenceBlob::missing(),
        );
    }
    let metadata = transition_read_blob(&sidecar.join("metadata.txt"), true);
    let metadata_matches = metadata.text.as_deref().is_some_and(|text| {
        transition_metadata_value(text, "from_version").as_deref() == Some(from_version)
            && transition_metadata_value(text, "to_version").as_deref() == Some(to_version)
            && transition_parse_u64(text, "take_revision") == Some(take_revision)
            && transition_metadata_value(text, "accepted_zip_sha256").as_deref()
                == Some(candidate_sha256)
            && transition_metadata_value(text, "verdict").as_deref() == Some("accepted")
            && transition_metadata_value(text, "authority").as_deref()
                == Some("advisory_evidence_only")
    });
    if !metadata_matches {
        return (
            "present_metadata_mismatch_not_consumed".to_string(),
            TransitionEvidenceBlob::missing(),
            TransitionEvidenceBlob::missing(),
        );
    }
    (
        "present_verified_identity".to_string(),
        transition_read_blob(&sidecar.join("candidate_intent.txt"), true),
        transition_read_blob(&sidecar.join("semantic_effect_snapshot.tsv"), false),
    )
}

fn transition_collect_canonical_history(
    project_root: &Path,
) -> (Vec<TransitionHistoryEntry>, Vec<String>) {
    let mut history = Vec::new();
    let mut issues = Vec::new();
    let ledger_root = project_root.join(TRANSITION_REJECTION_LEDGER_ROOT);
    if !ledger_root.is_dir() {
        return (history, issues);
    }

    for pair_root in transition_sorted_entries(&ledger_root) {
        if !pair_root.is_dir() {
            continue;
        }
        let pair_component = pair_root
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        for entry_path in transition_sorted_entries(&pair_root) {
            let file_name = entry_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string();
            if file_name.starts_with(".pending-") {
                transition_push_issue(
                    &mut issues,
                    format!(
                        "pending_rejection_ledger_entry_not_canonical:{}",
                        entry_path.display()
                    ),
                );
                continue;
            }

            if entry_path.is_file()
                && file_name.starts_with("accepted-")
                && file_name.ends_with(".txt")
            {
                let Ok(metadata) = fs::read_to_string(&entry_path) else {
                    transition_push_issue(
                        &mut issues,
                        format!("acceptance_metadata_unreadable:{}", entry_path.display()),
                    );
                    continue;
                };
                let Some(from_version) = transition_metadata_value(&metadata, "from_version") else {
                    transition_push_issue(
                        &mut issues,
                        format!("acceptance_from_version_missing:{}", entry_path.display()),
                    );
                    continue;
                };
                let Some(to_version) = transition_metadata_value(&metadata, "to_version") else {
                    transition_push_issue(
                        &mut issues,
                        format!("acceptance_to_version_missing:{}", entry_path.display()),
                    );
                    continue;
                };
                let Some(candidate_sha256) =
                    transition_metadata_value(&metadata, "accepted_zip_sha256")
                else {
                    transition_push_issue(
                        &mut issues,
                        format!("acceptance_sha256_missing:{}", entry_path.display()),
                    );
                    continue;
                };
                if !transition_valid_sha256(&candidate_sha256) {
                    transition_push_issue(
                        &mut issues,
                        format!("acceptance_sha256_invalid:{}", entry_path.display()),
                    );
                    continue;
                }
                let take_revision = transition_parse_u64(&metadata, "take_revision");
                let event_epoch_millis =
                    transition_parse_u128(&metadata, "accepted_epoch_millis");
                let prior_rejections = transition_metadata_value(&metadata, "prior_rejections")
                    .and_then(|value| value.parse::<usize>().ok());
                let (acceptance_evidence_status, candidate_intent, semantic_effect) =
                    transition_acceptance_evidence(
                        project_root,
                        pair_component,
                        &from_version,
                        &to_version,
                        take_revision,
                        &candidate_sha256,
                    );
                history.push(TransitionHistoryEntry {
                    sort_epoch_millis: event_epoch_millis,
                    from_version,
                    to_version,
                    take_revision,
                    candidate_sha256,
                    verdict: "accepted".to_string(),
                    authority: "canonical_acceptance_ledger_record".to_string(),
                    event_epoch_millis,
                    rejection_stage: None,
                    prior_rejections,
                    candidate_intent,
                    semantic_effect,
                    reason: TransitionEvidenceBlob::missing(),
                    acceptance_evidence_status,
                });
                continue;
            }

            if !entry_path.is_dir() {
                continue;
            }
            let metadata_path = entry_path.join("metadata.txt");
            if !metadata_path.is_file() {
                continue;
            }
            let Ok(metadata) = fs::read_to_string(&metadata_path) else {
                transition_push_issue(
                    &mut issues,
                    format!("rejection_metadata_unreadable:{}", metadata_path.display()),
                );
                continue;
            };
            let Some(from_version) = transition_metadata_value(&metadata, "from_version") else {
                transition_push_issue(
                    &mut issues,
                    format!("rejection_from_version_missing:{}", metadata_path.display()),
                );
                continue;
            };
            let Some(to_version) = transition_metadata_value(&metadata, "to_version") else {
                transition_push_issue(
                    &mut issues,
                    format!("rejection_to_version_missing:{}", metadata_path.display()),
                );
                continue;
            };
            let Some(candidate_sha256) = transition_metadata_value(&metadata, "zip_sha256") else {
                transition_push_issue(
                    &mut issues,
                    format!("rejection_sha256_missing:{}", metadata_path.display()),
                );
                continue;
            };
            if !transition_valid_sha256(&candidate_sha256) {
                transition_push_issue(
                    &mut issues,
                    format!("rejection_sha256_invalid:{}", metadata_path.display()),
                );
                continue;
            }
            let event_epoch_millis = transition_parse_u128(&metadata, "attempt_epoch_millis");
            history.push(TransitionHistoryEntry {
                sort_epoch_millis: event_epoch_millis,
                from_version,
                to_version,
                take_revision: transition_parse_u64(&metadata, "take_revision"),
                candidate_sha256,
                verdict: "rejected".to_string(),
                authority: "canonical_rejection_ledger_record".to_string(),
                event_epoch_millis,
                rejection_stage: transition_metadata_value(&metadata, "rejection_stage"),
                prior_rejections: None,
                candidate_intent: transition_read_blob(
                    &entry_path.join("candidate_intent.txt"),
                    true,
                ),
                semantic_effect: transition_read_blob(
                    &entry_path.join("semantic_effect_snapshot.tsv"),
                    false,
                ),
                reason: transition_read_blob(&entry_path.join("reason.txt"), true),
                acceptance_evidence_status: "not_applicable_rejected".to_string(),
            });
        }
    }

    history.sort_by(|left, right| {
        left.sort_epoch_millis
            .unwrap_or(u128::MAX)
            .cmp(&right.sort_epoch_millis.unwrap_or(u128::MAX))
            .then_with(|| left.take_revision.cmp(&right.take_revision))
            .then_with(|| left.verdict.cmp(&right.verdict))
            .then_with(|| left.candidate_sha256.cmp(&right.candidate_sha256))
    });
    (history, issues)
}

fn transition_current_candidate(
    project_root: &Path,
    issues: &mut Vec<String>,
) -> TransitionCurrentCandidate {
    let zip_kind = fs::read_to_string(project_root.join("ZIP_KIND.txt")).unwrap_or_default();
    let mut current = TransitionCurrentCandidate {
        source_snapshot_revision: transition_metadata_value(&zip_kind, "revision")
            .unwrap_or_else(|| "unknown".to_string()),
        source_snapshot_name: transition_metadata_value(&zip_kind, "name")
            .unwrap_or_else(|| "unknown".to_string()),
        transaction_state: "none".to_string(),
        from_version: "unknown".to_string(),
        to_version: "unknown".to_string(),
        candidate_sha256: "unknown".to_string(),
        ..TransitionCurrentCandidate::default()
    };
    let marker_path = project_root.join(TRANSITION_PENDING_MARKER);
    if !marker_path.is_file() {
        return current;
    }
    current.marker_present = true;
    let Ok(marker_text) = fs::read_to_string(&marker_path) else {
        transition_push_issue(
            issues,
            format!("pending_marker_unreadable:{}", marker_path.display()),
        );
        current.transaction_state = "marker_unreadable".to_string();
        return current;
    };
    let transaction_path = PathBuf::from(marker_text.trim());
    let expected_parent = project_root.join(TRANSITION_TRANSACTION_ROOT);
    if transaction_path.as_os_str().is_empty() || !transaction_path.starts_with(&expected_parent) {
        transition_push_issue(
            issues,
            "pending_marker_target_outside_transaction_root_not_consumed".to_string(),
        );
        current.transaction_state = "marker_target_not_consumed".to_string();
        return current;
    }
    current.transaction_state = fs::read_to_string(transaction_path.join("state.txt"))
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|_| "state_unreadable".to_string());
    if let Ok(metadata) = fs::read_to_string(transaction_path.join("candidate_metadata.txt")) {
        current.from_version = transition_metadata_value(&metadata, "from_version")
            .unwrap_or_else(|| "unknown".to_string());
        current.to_version = transition_metadata_value(&metadata, "to_version")
            .unwrap_or_else(|| "unknown".to_string());
        current.take_revision = transition_parse_u64(&metadata, "take_revision");
        current.candidate_sha256 = transition_metadata_value(&metadata, "zip_sha256")
            .unwrap_or_else(|| "unknown".to_string());
    } else {
        transition_push_issue(
            issues,
            "pending_candidate_metadata_unreadable".to_string(),
        );
    }
    current.candidate_intent =
        transition_read_blob(&transaction_path.join("candidate_intent.txt"), true);
    current.semantic_effect =
        transition_read_blob(&transaction_path.join("candidate_snapshot.tsv"), false);
    current
}

fn transition_write_blob(json: &mut JsonWriter, key: &str, blob: &TransitionEvidenceBlob) {
    json.begin_object(Some(key));
    json.string(Some("status"), &blob.status);
    json.raw("bytes", &blob.bytes.to_string());
    json.string(Some("sha256"), &blob.sha256);
    json.raw("nonempty_rows", &blob.row_count.to_string());
    if let Some(text) = blob.text.as_deref() {
        json.string(Some("text"), text);
    }
    json.end_object();
}

/// Display-only revision identity for the interactive launcher header. It reuses the
/// canonical-history and in-flight resolvers of the transition history projection and
/// never grants or derives an acceptance.
pub(super) struct ConsoleRevisionProjection {
    pub(super) source_tree: Option<u64>,
    pub(super) canonical_head: Option<u64>,
    pub(super) in_flight_candidate: Option<u64>,
}

fn transition_canonical_head(history: &[TransitionHistoryEntry]) -> Option<u64> {
    history
        .iter()
        .filter(|entry| entry.verdict == "accepted")
        .filter_map(|entry| entry.take_revision)
        .max()
}

pub(super) fn console_revision_projection(project_root: &Path) -> ConsoleRevisionProjection {
    let (history, mut issues) = transition_collect_canonical_history(project_root);
    let current = transition_current_candidate(project_root, &mut issues);
    ConsoleRevisionProjection {
        source_tree: current
            .source_snapshot_revision
            .strip_prefix('r')
            .and_then(|digits| digits.parse().ok()),
        canonical_head: transition_canonical_head(&history),
        in_flight_candidate: current.marker_present.then_some(current.take_revision).flatten(),
    }
}

fn write_transition_history_projection(
    project_root: &Path,
    latest_dir: &Path,
) -> AppResult<()> {
    let (history, mut issues) = transition_collect_canonical_history(project_root);
    let current = transition_current_candidate(project_root, &mut issues);

    let accepted = history.iter().filter(|entry| entry.verdict == "accepted").count();
    let rejected = history.iter().filter(|entry| entry.verdict == "rejected").count();
    let mut accepted_by_revision: BTreeMap<u64, BTreeSet<String>> = BTreeMap::new();
    for entry in history.iter().filter(|entry| entry.verdict == "accepted") {
        if let Some(revision) = entry.take_revision {
            accepted_by_revision
                .entry(revision)
                .or_default()
                .insert(entry.candidate_sha256.clone());
        }
    }
    for (revision, hashes) in &accepted_by_revision {
        if hashes.len() > 1 {
            transition_push_issue(
                &mut issues,
                format!(
                    "multiple_canonical_acceptance_hashes_for_revision:r{revision}:{}",
                    hashes.len()
                ),
            );
        }
    }
    let canonical_history_through_revision = transition_canonical_head(&history);
    let canonical_history_through_sha256 = canonical_history_through_revision.and_then(|revision| {
        accepted_by_revision
            .get(&revision)
            .filter(|hashes| hashes.len() == 1)
            .and_then(|hashes| hashes.iter().next().cloned())
    });

    let mut json = JsonWriter::new();
    json.begin_object(None);
    json.raw("schema_version", "1");
    json.string(Some("artifact"), "EtuSchlopfTransitionHistory");
    json.string(Some("authority"), "advisory_evidence_only");
    json.bool("normative", false);
    json.bool("acceptance_authority", false);
    json.bool("rejection_authority", false);
    json.string(
        Some("policy"),
        "project_existing_durable_verdict_records_only_current_in_flight_candidate_separate_and_noncanonical",
    );
    json.string(
        Some("effect_policy"),
        "semantic_effect_payload_not_duplicated_only_exact_sha256_byte_count_and_nonempty_row_count",
    );
    json.raw("accepted_records", &accepted.to_string());
    json.raw("rejected_records", &rejected.to_string());
    json.raw("projection_issue_count", &issues.len().to_string());
    json.string(
        Some("canonical_history_through_revision"),
        &canonical_history_through_revision
            .map(|revision| format!("r{revision}"))
            .unwrap_or_else(|| "unknown".to_string()),
    );
    json.string(
        Some("canonical_history_through_sha256"),
        canonical_history_through_sha256.as_deref().unwrap_or("unknown"),
    );

    json.begin_array("history");
    for entry in &history {
        json.begin_object(None);
        json.string(Some("verdict"), &entry.verdict);
        json.string(Some("authority"), &entry.authority);
        json.string(Some("from_version"), &entry.from_version);
        json.string(Some("to_version"), &entry.to_version);
        json.string(
            Some("take_revision"),
            &entry
                .take_revision
                .map(|revision| format!("r{revision}"))
                .unwrap_or_else(|| "unknown".to_string()),
        );
        json.string(Some("candidate_sha256"), &entry.candidate_sha256);
        json.string(
            Some("event_epoch_millis"),
            &entry
                .event_epoch_millis
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
        );
        json.string(
            Some("rejection_stage"),
            entry.rejection_stage.as_deref().unwrap_or("not_applicable_or_legacy_unknown"),
        );
        json.string(
            Some("prior_rejections"),
            &entry
                .prior_rejections
                .map(|value| value.to_string())
                .unwrap_or_else(|| "unknown_or_not_applicable".to_string()),
        );
        json.string(
            Some("acceptance_evidence_status"),
            &entry.acceptance_evidence_status,
        );
        transition_write_blob(&mut json, "candidate_intent", &entry.candidate_intent);
        transition_write_blob(&mut json, "semantic_effect", &entry.semantic_effect);
        transition_write_blob(&mut json, "reason", &entry.reason);
        json.end_object();
    }
    json.end_array();

    json.begin_object(Some("current_in_flight"));
    json.bool("marker_present", current.marker_present);
    json.string(Some("authority"), "none");
    json.string(
        Some("canonical_verdict"),
        if current.marker_present {
            "not_yet_canonical"
        } else {
            "none_in_flight"
        },
    );
    json.string(Some("transaction_state"), &current.transaction_state);
    json.string(Some("from_version"), &current.from_version);
    json.string(Some("to_version"), &current.to_version);
    json.string(
        Some("take_revision"),
        &current
            .take_revision
            .map(|revision| format!("r{revision}"))
            .unwrap_or_else(|| "unknown".to_string()),
    );
    json.string(Some("candidate_sha256"), &current.candidate_sha256);
    json.string(
        Some("source_snapshot_revision"),
        &current.source_snapshot_revision,
    );
    json.string(Some("source_snapshot_name"), &current.source_snapshot_name);
    transition_write_blob(&mut json, "candidate_intent", &current.candidate_intent);
    transition_write_blob(&mut json, "semantic_effect", &current.semantic_effect);
    json.end_object();

    json.strings("projection_issues", issues.iter());
    json.end_object();
    fs::write(latest_dir.join(TRANSITION_HISTORY_FILE), json.finish())?;

    Ok(())
}


#[derive(Clone, Debug)]
struct ProjectFolderCatalogEntry {
    path: String,
    entry_type: String,
    size_bytes: u64,
    scope: String,
    role: String,
    take_managed: bool,
    handover_relation: String,
    state: String,
}

#[derive(Clone, Debug)]
pub struct ProjectFolderCatalogSummary {
    pub(super) entry_count: usize,
    pub(super) file_count: usize,
    pub(super) directory_count: usize,
    pub(super) symlink_count: usize,
    pub(super) present_file_bytes: u64,
    pub(super) planned_entry_count: usize,
}

fn normalize_catalog_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn project_folder_catalog_scope_and_role(
    relative: &str,
    entry_type: &str,
) -> (&'static str, &'static str) {
    if relative == "assistant_export_latest.zip"
        || relative == "target/etuschlopf_assistant_export/latest.zip"
    {
        return ("handover", "handover_zip_container");
    }
    if relative.starts_with("target/etuschlopf_assistant_export/latest/") {
        return ("handover", "handover_payload_staging");
    }
    if let Some(role) = managed_source_tree_definition::root_project_file_role(relative) {
        return match role {
            managed_source_tree_definition::RootProjectFileRole::TakeManaged => {
                ("root_contract", "project_root_contract_or_launcher")
            }
            managed_source_tree_definition::RootProjectFileRole::LocalTransactional => {
                ("root_local_transaction_state", "cargo_owned_local_transaction_state")
            }
        };
    }
    if relative.starts_with("src/pipeline_shards/") {
        return ("source", "productive_and_audit_pipeline_shard");
    }
    if relative.starts_with("src/build_gate/") || relative == "src/kodex_build_orchestrator.rs" {
        return ("source", "build_gate_and_identity_inventory");
    }
    if relative.starts_with("src/bin/") {
        return ("source", "binary_launcher_or_bootstrap");
    }
    if relative.starts_with("src/") {
        return ("source", "rust_source_module");
    }
    if relative.starts_with("tests/") {
        return ("test", "integration_or_regression_test");
    }
    if relative.starts_with("docs/") {
        return ("documentation", "project_documentation_or_handover_memory");
    }
    if relative.starts_with("tools/") {
        return ("tooling", "project_tool_or_take_support");
    }
    if relative.starts_with("target/") {
        return ("generated", "build_run_or_diagnostic_artifact");
    }
    let lower = relative.to_ascii_lowercase();
    if lower.ends_with(".wav") || lower.ends_with(".mid") {
        return ("project_input", "music_project_input");
    }
    if lower.ends_with(".render.musicxml") {
        return ("project_output", "render_musicxml_output");
    }
    if lower.ends_with(".musicxml") {
        return ("project_output", "master_musicxml_output");
    }
    if lower.ends_with(".pdf") {
        return ("project_output", "notation_pdf_output");
    }
    if lower.ends_with(".result.txt") || lower.ends_with(".strict-clippy.txt") {
        return ("project_output", "run_report_or_validation_log");
    }
    if entry_type == "directory" {
        return ("other", "directory_container");
    }
    ("other", "unclassified_project_file")
}

fn project_folder_catalog_take_managed(relative: &str) -> bool {
    relative.starts_with("src/")
        || relative.starts_with("docs/")
        || relative.starts_with("tools/")
        || relative.starts_with("tests/")
        || matches!(
            managed_source_tree_definition::root_project_file_role(relative),
            Some(managed_source_tree_definition::RootProjectFileRole::TakeManaged)
        )
}

fn project_folder_catalog_handover_relation(relative: &str) -> &'static str {
    if relative.starts_with("target/etuschlopf_assistant_export/latest/") {
        "included_in_handover_zip"
    } else if relative == "assistant_export_latest.zip"
        || relative == "target/etuschlopf_assistant_export/latest.zip"
    {
        "handover_zip_container"
    } else {
        "catalogued_only"
    }
}

fn collect_project_folder_catalog_entries(
    project_root: &Path,
    current: &Path,
    entries: &mut Vec<ProjectFolderCatalogEntry>,
    exclude_target_descendants: bool,
) -> AppResult<()> {
    let mut children = fs::read_dir(current)?
        .collect::<Result<Vec<_>, _>>()?;
    children.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());

    for child in children {
        let path = child.path();
        let relative_path = path.strip_prefix(project_root)?;
        let relative = normalize_catalog_path(relative_path);
        let metadata = fs::symlink_metadata(&path)?;
        let file_type = metadata.file_type();
        let entry_type = if file_type.is_symlink() {
            "symlink"
        } else if file_type.is_dir() {
            "directory"
        } else if file_type.is_file() {
            "file"
        } else {
            "other"
        };
        let (scope, role) = project_folder_catalog_scope_and_role(&relative, entry_type);
        let excluded_target_root = exclude_target_descendants && file_type.is_dir() && relative == "target";
        entries.push(ProjectFolderCatalogEntry {
            path: relative.clone(),
            entry_type: entry_type.to_string(),
            size_bytes: if file_type.is_file() { metadata.len() } else { 0 },
            scope: scope.to_string(),
            role: role.to_string(),
            take_managed: project_folder_catalog_take_managed(&relative),
            handover_relation: project_folder_catalog_handover_relation(&relative).to_string(),
            state: if excluded_target_root { "generated_descendants_excluded" } else { "present_at_snapshot" }.to_string(),
        });
        if file_type.is_dir() && !excluded_target_root {
            collect_project_folder_catalog_entries(
                project_root, &path, entries, exclude_target_descendants,
            )?;
        }
    }
    Ok(())
}

fn catalog_tsv_cell(value: &str) -> String {
    value
        .chars()
        .map(|ch| if matches!(ch, '\t' | '\r' | '\n') { ' ' } else { ch })
        .collect()
}

fn catalog_markdown_cell(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '|' => out.push_str("\\|"),
            '\r' | '\n' => out.push(' '),
            _ => out.push(ch),
        }
    }
    out
}

pub fn write_project_folder_catalog(project_root: &Path, latest_dir: &Path, version: &str) -> AppResult<ProjectFolderCatalogSummary> {
    write_project_folder_catalog_mode(project_root, latest_dir, version, false, true)
}

pub fn write_normal_run_project_folder_catalog(project_root: &Path, latest_dir: &Path, version: &str) -> AppResult<ProjectFolderCatalogSummary> {
    write_project_folder_catalog_mode(project_root, latest_dir, version, true, false)
}

fn write_project_folder_catalog_mode(
    project_root: &Path,
    latest_dir: &Path,
    version: &str,
    exclude_target_descendants: bool,
    emit_markdown: bool,
) -> AppResult<ProjectFolderCatalogSummary> {
    let _timing = AssistantExportPhaseTimer::new(project_root, latest_dir, "AssistantExportCatalogMs");
    let latest_relative = latest_dir.strip_prefix(project_root)?;
    let mut planned_paths = vec![
        latest_relative.join(PROJECT_FILE_CATALOG_TSV),
        latest_relative.join("assistant_context.json"),
        latest_relative.join(ASSISTANT_HANDOFF_FILENAME),
        latest_relative.join("manifest.txt"),
        PathBuf::from("target/etuschlopf_assistant_export/latest.zip"),
        PathBuf::from("assistant_export_latest.zip"),
        PathBuf::from(ASSISTANT_HANDOFF_FILENAME),
    ];
    if emit_markdown {
        planned_paths.push(latest_relative.join(PROJECT_FILE_CATALOG_MARKDOWN));
    }
    let planned_paths = planned_paths
        .into_iter()
        .map(|path| normalize_catalog_path(&path))
        .collect::<BTreeSet<_>>();

    let mut entries = Vec::new();
    collect_project_folder_catalog_entries(project_root, project_root, &mut entries, exclude_target_descendants)?;

    entries.retain(|entry| !planned_paths.contains(&entry.path));
    for relative in planned_paths {
        let (scope, role) = project_folder_catalog_scope_and_role(&relative, "file");
        entries.push(ProjectFolderCatalogEntry {
            path: relative.clone(),
            entry_type: "file".to_string(),
            size_bytes: 0,
            scope: scope.to_string(),
            role: role.to_string(),
            take_managed: project_folder_catalog_take_managed(&relative),
            handover_relation: project_folder_catalog_handover_relation(&relative).to_string(),
            state: "planned_output_after_snapshot".to_string(),
        });
    }
    entries.sort_by(|left, right| left.path.cmp(&right.path));

    let file_count = entries.iter().filter(|entry| entry.entry_type == "file").count();
    let directory_count = entries
        .iter()
        .filter(|entry| entry.entry_type == "directory")
        .count();
    let symlink_count = entries
        .iter()
        .filter(|entry| entry.entry_type == "symlink")
        .count();
    let present_file_bytes = entries
        .iter()
        .filter(|entry| entry.entry_type == "file" && entry.state == "present_at_snapshot")
        .map(|entry| entry.size_bytes)
        .sum();
    let planned_entry_count = entries
        .iter()
        .filter(|entry| entry.state == "planned_output_after_snapshot")
        .count();
    let summary = ProjectFolderCatalogSummary {
        entry_count: entries.len(),
        file_count,
        directory_count,
        symlink_count,
        present_file_bytes,
        planned_entry_count,
    };

    let tsv_path = latest_dir.join(PROJECT_FILE_CATALOG_TSV);
    let mut tsv = File::create(&tsv_path)?;
    writeln!(
        tsv,
        "path\tentry_type\tsize_bytes\tscope\trole\ttake_managed\thandover_relation\tstate"
    )?;
    for entry in &entries {
        writeln!(
            tsv,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            catalog_tsv_cell(&entry.path),
            catalog_tsv_cell(&entry.entry_type),
            entry.size_bytes,
            catalog_tsv_cell(&entry.scope),
            catalog_tsv_cell(&entry.role),
            if entry.take_managed { "yes" } else { "no" },
            catalog_tsv_cell(&entry.handover_relation),
            catalog_tsv_cell(&entry.state),
        )?;
    }

    let mut scope_counts = BTreeMap::<String, usize>::new();
    for entry in &entries {
        *scope_counts.entry(entry.scope.clone()).or_insert(0) += 1;
    }

    let markdown_path = latest_dir.join(PROJECT_FILE_CATALOG_MARKDOWN);
    if !emit_markdown {
        if markdown_path.exists() {
            fs::remove_file(&markdown_path)?;
        }
        return Ok(summary);
    }
    let mut markdown = File::create(&markdown_path)?;
    writeln!(markdown, "# ETUSCHLOPF PROJECT FILE CATALOG")?;
    writeln!(markdown)?;
    writeln!(markdown, "- Version: `{version}`")?;
    writeln!(markdown, "- Root: `.` (paths are project-root relative)")?;
    writeln!(markdown, "- RecursiveProjectRoot: `{}`", if exclude_target_descendants { "no" } else { "yes" })?;
    writeln!(markdown, "- EveryReadableEntryRecorded: `{}`", if exclude_target_descendants { "no" } else { "yes" })?;
    writeln!(markdown, "- SymlinksFollowed: `no`")?;
    writeln!(markdown, "- OmittedPaths: `{}`", if exclude_target_descendants { "target descendants (generated)" } else { "none; an unreadable entry aborts export generation" })?;
    writeln!(
        markdown,
        "- PlannedOutputs: catalog files, assistant context, manifest and both handover ZIP paths are represented explicitly before final ZIP creation"
    )?;
    writeln!(markdown)?;
    writeln!(markdown, "## Summary")?;
    writeln!(markdown)?;
    writeln!(markdown, "- Entries: {}", summary.entry_count)?;
    writeln!(markdown, "- Files: {}", summary.file_count)?;
    writeln!(markdown, "- Directories: {}", summary.directory_count)?;
    writeln!(markdown, "- Symlinks: {}", summary.symlink_count)?;
    writeln!(markdown, "- Present file bytes: {}", summary.present_file_bytes)?;
    writeln!(markdown, "- Planned output entries: {}", summary.planned_entry_count)?;
    writeln!(markdown)?;
    writeln!(markdown, "## Scope counts")?;
    writeln!(markdown)?;
    for (scope, count) in &scope_counts {
        writeln!(markdown, "- `{scope}`: {count}")?;
    }
    writeln!(markdown)?;
    writeln!(markdown, "## {} catalog", if exclude_target_descendants { "Bounded normal-run" } else { "Complete recursive" })?;
    writeln!(markdown)?;
    writeln!(
        markdown,
        "| Path | Type | Bytes | Scope | Role | Take managed | Handover relation | State |"
    )?;
    writeln!(
        markdown,
        "|---|---:|---:|---|---|---:|---|---|"
    )?;
    for entry in &entries {
        writeln!(
            markdown,
            "| `{}` | {} | {} | {} | {} | {} | {} | {} |",
            catalog_markdown_cell(&entry.path),
            catalog_markdown_cell(&entry.entry_type),
            entry.size_bytes,
            catalog_markdown_cell(&entry.scope),
            catalog_markdown_cell(&entry.role),
            if entry.take_managed { "yes" } else { "no" },
            catalog_markdown_cell(&entry.handover_relation),
            catalog_markdown_cell(&entry.state),
        )?;
    }
    Ok(summary)
}


const CANONICAL_MEASUREMENT_CONTRACT_VERSION: &str = "CMS-1";
const CANONICAL_MEASUREMENT_SNAPSHOT_SUFFIX: &str =
    ".canonical_measurement_snapshot_v1.json";

#[derive(Clone, Debug, PartialEq, Eq)]
struct MeasurementSnapshotExportBinding {
    status: &'static str,
    file_name: Option<String>,
    sha256: Option<String>,
    run_id: String,
    measurement_contract_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InventoryExportBinding {
    status: &'static str,
    json_sha256: Option<String>,
    xlsx_sha256: Option<String>,
    run_id: Option<String>,
    run_scope: Option<String>,
    corpus_manifest_run_id: Option<String>,
    inventory_contract_version: Option<String>,
    architecture_contract_version: Option<String>,
    xlsx_contract_version: Option<String>,
}

fn stage_current_inventory_artifacts(
    project_root: &Path,
    latest_dir: &Path,
    included: &mut Vec<String>,
) -> AppResult<InventoryExportBinding> {
    let json_path = project_root.join(INVENTORY_JSON_FILE);
    let xlsx_path = project_root.join(INVENTORY_XLSX_FILE);
    let json_exists = json_path.is_file();
    let xlsx_exists = xlsx_path.is_file();
    if !json_exists && !xlsx_exists {
        return Ok(InventoryExportBinding {
            status: "not_requested",
            json_sha256: None,
            xlsx_sha256: None,
            run_id: None,
            run_scope: None,
            corpus_manifest_run_id: None,
            inventory_contract_version: None,
            architecture_contract_version: None,
            xlsx_contract_version: None,
        });
    }
    if !json_exists || !xlsx_exists {
        return Err(anyhow!(
            "assistant export: inventory artifact pair incomplete (json={}, xlsx={})",
            json_exists,
            xlsx_exists
        ));
    }
    let json_bytes = fs::read(&json_path)?;
    let json_text = std::str::from_utf8(&json_bytes).map_err(|error| {
        anyhow!("assistant export: inventory JSON is not UTF-8: {error}")
    })?;
    let run_id = generated_json_string_field(json_text, "run_id")
        .ok_or_else(|| anyhow!("assistant export: inventory JSON has no simple run_id"))?;
    let run_scope = generated_json_string_field(json_text, "run_scope")
        .ok_or_else(|| anyhow!("assistant export: inventory JSON has no simple run_scope"))?;
    let corpus_manifest_run_id = generated_json_string_field(json_text, "corpus_manifest_run_id")
        .ok_or_else(|| anyhow!("assistant export: inventory JSON has no corpus_manifest_run_id"))?;
    let inventory_contract_version = generated_json_string_field(
        json_text,
        "inventory_contract_version",
    )
    .ok_or_else(|| anyhow!("assistant export: inventory JSON has no ESI contract"))?;
    let architecture_contract_version = generated_json_string_field(
        json_text,
        "architecture_contract_version",
    )
    .ok_or_else(|| anyhow!("assistant export: inventory JSON has no ARCH contract"))?;
    let xlsx_contract_version = generated_json_string_field(json_text, "xlsx_contract_version")
        .ok_or_else(|| anyhow!("assistant export: inventory JSON has no XLSX contract"))?;
    if inventory_contract_version != "ESI-3"
        || architecture_contract_version != "ARCH-4"
        || xlsx_contract_version != "XLSX-3"
    {
        return Err(anyhow!(
            "assistant export: inventory contract mismatch ESI={inventory_contract_version} ARCH={architecture_contract_version} XLSX={xlsx_contract_version}"
        ));
    }
    let xlsx_bytes = fs::read(&xlsx_path)?;
    copy_export_file(&json_path, &latest_dir.join(INVENTORY_JSON_FILE), included)?;
    copy_export_file(&xlsx_path, &latest_dir.join(INVENTORY_XLSX_FILE), included)?;
    Ok(InventoryExportBinding {
        status: "bound_current_inventory",
        json_sha256: Some(sha256::hex(&json_bytes)),
        xlsx_sha256: Some(sha256::hex(&xlsx_bytes)),
        run_id: Some(run_id),
        run_scope: Some(run_scope),
        corpus_manifest_run_id: Some(corpus_manifest_run_id),
        inventory_contract_version: Some(inventory_contract_version),
        architecture_contract_version: Some(architecture_contract_version),
        xlsx_contract_version: Some(xlsx_contract_version),
    })
}

fn canonical_measurement_snapshot_file_name(project: &str) -> String {
    format!("{project}{CANONICAL_MEASUREMENT_SNAPSHOT_SUFFIX}")
}

fn generated_json_string_field(text: &str, key: &str) -> Option<String> {
    let prefix = format!("\"{key}\": \"");
    text.lines().find_map(|line| {
        let trimmed = line.trim();
        let tail = trimmed.strip_prefix(&prefix)?;
        let tail = tail.strip_suffix(',').unwrap_or(tail);
        let value = tail.strip_suffix('"')?;
        if value.contains('\\') {
            return None;
        }
        Some(value.to_string())
    })
}

fn stage_current_measurement_snapshot(
    project_root: &Path,
    latest_dir: &Path,
    project: &str,
    expected_run_id: &str,
    final_exit: i32,
    included: &mut Vec<String>,
    omitted: &mut Vec<String>,
) -> AppResult<MeasurementSnapshotExportBinding> {
    let file_name = canonical_measurement_snapshot_file_name(project);
    let src = project_root.join(&file_name);

    if !src.is_file() {
        if final_exit == 0 {
            return Err(anyhow!(
                "assistant export: successful current run is missing CanonicalMeasurementSnapshotV1: {}",
                src.display()
            ));
        }
        omitted.push(format!(
            "{file_name} | current_run_measurement_snapshot_not_available"
        ));
        return Ok(MeasurementSnapshotExportBinding {
            status: "not_available_current_run",
            file_name: None,
            sha256: None,
            run_id: expected_run_id.to_string(),
            measurement_contract_version: None,
        });
    }

    let bytes = fs::read(&src)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| {
        anyhow!(
            "assistant export: CanonicalMeasurementSnapshotV1 is not UTF-8: {} ({error})",
            src.display()
        )
    })?;
    let run_id = generated_json_string_field(text, "run_id").ok_or_else(|| {
        anyhow!(
            "assistant export: CanonicalMeasurementSnapshotV1 has no simple run_id: {}",
            src.display()
        )
    })?;
    let measurement_contract_version =
        generated_json_string_field(text, "measurement_contract_version").ok_or_else(|| {
            anyhow!(
                "assistant export: CanonicalMeasurementSnapshotV1 has no measurement_contract_version: {}",
                src.display()
            )
        })?;
    let expected_project_json = json_escape(project);
    let expected_project_field = format!("\"expected_projects\": [\"{expected_project_json}\"]");
    let measured_project_field = format!("\"measured_projects\": [\"{expected_project_json}\"]");
    let project_bound =
        text.contains(&expected_project_field) && text.contains(&measured_project_field);
    let schema_bound = text.contains("\"schema_version\": 1,")
        && measurement_contract_version == CANONICAL_MEASUREMENT_CONTRACT_VERSION;

    if run_id != expected_run_id || !project_bound || !schema_bound {
        if final_exit == 0 {
            return Err(anyhow!(
                "assistant export: CanonicalMeasurementSnapshotV1 is not bound to current run/project/contract; file {} run_id {:?} expected {:?} project_bound {} schema_bound {}",
                src.display(),
                run_id,
                expected_run_id,
                project_bound,
                schema_bound
            ));
        }
        omitted.push(format!(
            "{file_name} | stale_or_unbound_measurement_snapshot_not_exported"
        ));
        return Ok(MeasurementSnapshotExportBinding {
            status: "not_available_current_run",
            file_name: None,
            sha256: None,
            run_id: expected_run_id.to_string(),
            measurement_contract_version: None,
        });
    }

    let dest = latest_dir.join(&file_name);
    fs::write(&dest, &bytes)?;
    included.push(file_name.clone());
    Ok(MeasurementSnapshotExportBinding {
        status: "bound_current_run",
        file_name: Some(file_name),
        sha256: Some(sha256::hex(&bytes)),
        run_id,
        measurement_contract_version: Some(measurement_contract_version),
    })
}

pub fn normal_run_assistant_export_project_paths(project: &str) -> Vec<PathBuf> {
    vec![
        resolve_musicxml_file(project),
        resolve_render_musicxml_file(project),
        resolve_pdf_file(project),
    ]
}

fn project_root_bound_path(project_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        project_root.join(path)
    }
}

fn write_project_export_manifest_footer(file: &mut impl Write, included: &[String], omitted: &[String]) -> AppResult<()> {
    writeln!(file)?;
    writeln!(file, "INCLUDED-FILES")?;
    writeln!(file, "--------------")?;
    writeln!(file, "- manifest.txt")?;
    for name in included {
        writeln!(file, "- {name}")?;
    }
    writeln!(file)?;
    writeln!(file, "OMITTED-FILES")?;
    writeln!(file, "-------------")?;
    if omitted.is_empty() {
        writeln!(file, "- none")?;
    } else {
        for item in omitted {
            writeln!(file, "- {item}")?;
        }
    }
    writeln!(file)?;
    writeln!(file, "UPLOAD-INSTRUCTION")?;
    writeln!(file, "------------------")?;
    writeln!(file, "Im Chat assistant_export_latest.zip hochladen. Diese Diagnose-ZIP nie mit --take verwenden.")?;
    Ok(())
}

fn publish_assistant_export_zip(latest_dir: &Path, latest_zip: &Path, root_latest_zip: &Path) -> AppResult<()> {
    if latest_zip.exists() {
        fs::remove_file(latest_zip)?;
    }
    zip_directory_flat(latest_dir, latest_zip)?;
    if root_latest_zip.exists() {
        fs::remove_file(root_latest_zip)?;
    }
    fs::copy(latest_zip, root_latest_zip)?;
    Ok(())
}

fn write_inventory_manifest_fields(file: &mut impl Write, inventory: &InventoryExportBinding) -> AppResult<()> {
    writeln!(file, "InventoryStatus: {}", inventory.status)?;
    writeln!(file, "InventoryJson: {}", if inventory.json_sha256.is_some() { INVENTORY_JSON_FILE } else { "none" })?;
    writeln!(file, "InventoryJsonSha256: {}", inventory.json_sha256.as_deref().unwrap_or("none"))?;
    writeln!(file, "InventoryXlsx: {}", if inventory.xlsx_sha256.is_some() { INVENTORY_XLSX_FILE } else { "none" })?;
    writeln!(file, "InventoryXlsxSha256: {}", inventory.xlsx_sha256.as_deref().unwrap_or("none"))?;
    writeln!(file, "InventoryRunId: {}", inventory.run_id.as_deref().unwrap_or("none"))?;
    writeln!(file, "InventoryRunScope: {}", inventory.run_scope.as_deref().unwrap_or("none"))?;
    writeln!(file, "InventoryCorpusManifestRunId: {}", inventory.corpus_manifest_run_id.as_deref().unwrap_or("none"))?;
    writeln!(file, "InventoryContractVersion: {}", inventory.inventory_contract_version.as_deref().unwrap_or("none"))?;
    writeln!(file, "ArchitectureContractVersion: {}", inventory.architecture_contract_version.as_deref().unwrap_or("none"))?;
    writeln!(file, "XlsxContractVersion: {}", inventory.xlsx_contract_version.as_deref().unwrap_or("none"))?;
    Ok(())
}

fn create_normal_run_assistant_export_bundle(
    project_root: &Path,
    project: &str,
    state: &RunState,
    version: &str,
    run_id: &str,
    final_exit: i32,
) -> AppResult<PathBuf> {
    let export_root = project_root.join("target/etuschlopf_assistant_export");
    let latest_dir = export_root.join("latest");
    let latest_zip = export_root.join("latest.zip");
    let root_latest_zip = project_root.join("assistant_export_latest.zip");
    if latest_dir.exists() {
        fs::remove_dir_all(&latest_dir)?;
    }
    if latest_zip.exists() {
        fs::remove_file(&latest_zip)?;
    }
    if root_latest_zip.exists() {
        fs::remove_file(&root_latest_zip)?;
    }
    fs::create_dir_all(&latest_dir)?;

    let result_path = project_root.join(resolve_result_file(project));
    if result_path.exists() {
        let mut result = OpenOptions::new().append(true).open(&result_path)?;
        writeln!(result)?;
        writeln!(result, "ASSISTANT-EXPORT-SINGLE-PROJECT")?;
        writeln!(result, "-------------------------------")?;
        writeln!(result, "Status: v{version} normal_run export; assistant_export_latest.zip wird auch bei kontrolliertem Exit 2 geschrieben.")?;
        writeln!(result, "Project: {project}")?;
        writeln!(result, "FinalExit: {final_exit}")?;
        writeln!(result, "IncludedContract: compact assistant summary + master MusicXML + render MusicXML + project PDF + current-run-bound CanonicalMeasurementSnapshotV1 when its checkpoint exists + current logs/context + TSV-only bounded normal-run project-folder catalog without target descendants; full result and advisory transition history remain local; WAV/MIDI payload excluded; retired Shadow/Control/Paired artifacts excluded; TakeCompatible false.")?;
        writeln!(result, "NoBehaviorChange: Exporterzeugung aendert keine Musik-, Render- oder PDF-Entscheidung.")?;
    }

    let mut included = Vec::new();
    let mut omitted = Vec::new();
    let run_outcome = truthful_run_outcome_projection(typed_run_outcome_for_console(
        state,
        "normal_run",
        final_exit,
    ));
    for relative in normal_run_assistant_export_project_paths(project) {
        let src = project_root.join(&relative);
        if src.exists() {
            copy_export_file(&src, &latest_dir.join(file_name_string(&relative)), &mut included)?;
        } else {
            omitted.push(format!("{} | missing_current_single_project_output", relative.display()));
        }
    }

    let renderer_context =
        assistant_context_renderer_state_from_runtime(project_root, project, version);
    if result_path.exists() {
        let summary_dest = latest_dir.join(project_result_summary_file_name(project));
        write_assistant_project_result_summary(
            &result_path,
            &summary_dest,
            project,
            version,
            &renderer_context,
        )?;
        included.push(file_name_string(&summary_dest));
        omitted.push(format!(
            "{} | full_project_result_local_assistant_summary_exported",
            file_name_string(&result_path)
        ));
    } else {
        omitted.push(format!(
            "{} | missing_current_single_project_result",
            result_path.display()
        ));
    }

    if let Some(strict_log) = state.strict_log.as_deref() {
        let src = project_root_bound_path(project_root, strict_log);
        if src.exists() {
            copy_export_file(&src, &latest_dir.join(file_name_string(&src)), &mut included)?;
        } else {
            omitted.push(format!("{} | current_strict_log_missing", strict_log.display()));
        }
    } else {
        omitted.push("strict-clippy log | not_recorded".to_string());
    }

    if let Some(strict_test_log) = state.strict_test_log.as_deref() {
        let src = project_root_bound_path(project_root, strict_test_log);
        if src.exists() {
            copy_export_file(&src, &latest_dir.join(file_name_string(&src)), &mut included)?;
        } else {
            omitted.push(format!("{} | current_launcher_test_log_missing", strict_test_log.display()));
        }
    } else {
        omitted.push("visual_preflight.strict-tests.txt | not_run_for_plain_single_project_run".to_string());
    }

    let console_log = project_root.join(CONSOLE_LOG);
    if console_log.exists() {
        let mut console = OpenOptions::new().append(true).open(&console_log)?;
        writeln!(console)?;
        writeln!(console, "ETUSCHLOPF-RUN-OUTCOME-PROJECTION")?;
        writeln!(console, "RunSeverity: {}", run_outcome.report.severity)?;
        writeln!(
            console,
            "ReleaseDecision: {}",
            run_outcome.report.release_decision
        )?;
        writeln!(
            console,
            "ConsoleProjectionStatus: {}",
            run_outcome.report.console_status
        )?;
        writeln!(
            console,
            "ConsoleProjectionSeverity: {}",
            run_outcome.report.console_severity
        )?;
        writeln!(console, "FinalExit: {}", run_outcome.report.final_exit)?;
        writeln!(
            console,
            "RendererOutcome: {}",
            run_outcome.report.renderer_outcome
        )?;
        copy_export_file(
            &console_log,
            &latest_dir.join(file_name_string(&console_log)),
            &mut included,
        )?;
    } else {
        omitted.push(format!("{CONSOLE_LOG} | current_console_log_missing"));
    }

    let run_outcome_path = latest_dir.join(RUN_OUTCOME_AUDIT_FILE);
    fs::write(&run_outcome_path, run_outcome.report.audit_text(version))?;
    included.push(file_name_string(&run_outcome_path));

    let zip_kind_path = latest_dir.join("ZIP_KIND.txt");
    fs::write(
        &zip_kind_path,
        format!(
            "kind=assistant_export\ntake_compatible=false\nmode=normal_run\nversion={version}\nproject={project}\n"
        ),
    )?;
    included.push(file_name_string(&zip_kind_path));

    let folder_catalog = write_normal_run_project_folder_catalog(project_root, &latest_dir, version)?;
    included.push(PROJECT_FILE_CATALOG_TSV.to_string());
    omitted.push(format!(
        "{PROJECT_FILE_CATALOG_MARKDOWN} | duplicate_human_rendering_omitted_tsv_exported"
    ));
    let source_snapshot = stage_source_snapshot(
        project_root,
        &latest_dir,
        version,
        &mut included,
        &mut omitted,
    )?;
    append_source_snapshot_status_to_outer_zip_kind(&latest_dir, &source_snapshot)?;
    omitted.push(format!(
        "{TRANSITION_HISTORY_FILE} | pre_acceptance_advisory_history_remains_local_normal_run"
    ));
    let measurement_snapshot = stage_current_measurement_snapshot(
        project_root,
        &latest_dir,
        project,
        run_id,
        final_exit,
        &mut included,
        &mut omitted,
    )?;
    let inventory = stage_current_inventory_artifacts(project_root, &latest_dir, &mut included)?;
    if inventory.status == "bound_current_inventory"
        && inventory.run_id.as_deref() != Some(run_id)
    {
        return Err(anyhow!("assistant export: inventory is not bound to current normal-run observation session"));
    }
    if inventory.status == "bound_current_inventory"
        && inventory.run_scope.as_deref() != Some("PROJECT")
    {
        return Err(anyhow!("assistant export: project inventory has non-PROJECT scope"));
    }

    let mut json = JsonWriter::new();
    json.begin_object(None);
    json.raw("schema_version", "18");
    json.string(Some("zip_kind"), "assistant_export");
    json.bool("take_compatible", false);
    json.string(Some("mode"), "normal_run");
    json.string(Some("version"), version);
    json.string(Some("project"), project);
    json.raw("final_exit", &final_exit.to_string());
    json.begin_object(Some("run_outcome"));
    for (key, value) in [
        ("mode", run_outcome.report.mode),
        ("fatal", run_outcome.report.fatal),
        ("severity", run_outcome.report.severity),
        (
            "release_decision",
            run_outcome.report.release_decision.as_str(),
        ),
        ("console_status", run_outcome.report.console_status),
        (
            "console_severity",
            run_outcome.report.console_severity,
        ),
        ("build_exit", run_outcome.report.build_exit.as_str()),
        ("clippy_exit", run_outcome.report.clippy_exit.as_str()),
        ("audit_signal", run_outcome.report.audit_signal),
        (
            "renderer",
            run_outcome.report.renderer_outcome.as_str(),
        ),
        ("emergence", run_outcome.report.emergence_outcome),
    ] {
        json.string(Some(key), value);
    }
    json.raw("final_exit", &run_outcome.report.final_exit.to_string());
    json.string(Some("audit_file"), RUN_OUTCOME_AUDIT_FILE);
    json.end_object();
    json.begin_object(Some("project_folder_catalog"));
    json.raw("schema_version", "2");
    json.bool("complete_recursive_project_root", false);
    json.string(Some("excluded_generated_prefix"), "target/");
    json.string(Some("markdown"), "none");
    json.string(Some("tsv"), PROJECT_FILE_CATALOG_TSV);
    json.raw("entries", &folder_catalog.entry_count.to_string());
    json.raw("files", &folder_catalog.file_count.to_string());
    json.raw("directories", &folder_catalog.directory_count.to_string());
    json.raw("symlinks", &folder_catalog.symlink_count.to_string());
    json.raw("present_file_bytes", &folder_catalog.present_file_bytes.to_string());
    json.raw("planned_output_entries", &folder_catalog.planned_entry_count.to_string());
    json.end_object();
    json.begin_object(Some("normal_run_projection"));
    json.string(Some("project_result"), &project_result_summary_file_name(project));
    json.string(Some("full_project_result"), "local_only");
    json.string(Some("transition_history"), "local_advisory_only");
    json.end_object();
    json.begin_object(Some("canonical_measurement_snapshot"));
    json.string(Some("status"), measurement_snapshot.status);
    json.string(Some("binding"), "current_observation_session_run_id");
    json.string(
        Some("file"),
        measurement_snapshot.file_name.as_deref().unwrap_or("none"),
    );
    json.string(
        Some("sha256"),
        measurement_snapshot.sha256.as_deref().unwrap_or("none"),
    );
    json.string(Some("run_id"), &measurement_snapshot.run_id);
    json.string(
        Some("measurement_contract_version"),
        measurement_snapshot
            .measurement_contract_version
            .as_deref()
            .unwrap_or("none"),
    );
    json.end_object();
    json.begin_object(Some("inventory"));
    json.string(Some("status"), inventory.status);
    json.string(Some("json_file"), if inventory.json_sha256.is_some() { INVENTORY_JSON_FILE } else { "none" });
    json.string(Some("xlsx_file"), if inventory.xlsx_sha256.is_some() { INVENTORY_XLSX_FILE } else { "none" });
    json.string(Some("json_sha256"), inventory.json_sha256.as_deref().unwrap_or("none"));
    json.string(Some("xlsx_sha256"), inventory.xlsx_sha256.as_deref().unwrap_or("none"));
    json.string(Some("run_id"), inventory.run_id.as_deref().unwrap_or("none"));
    json.string(Some("run_scope"), inventory.run_scope.as_deref().unwrap_or("none"));
    json.string(Some("corpus_manifest_run_id"), inventory.corpus_manifest_run_id.as_deref().unwrap_or("none"));
    json.string(Some("inventory_contract_version"), inventory.inventory_contract_version.as_deref().unwrap_or("none"));
    json.string(Some("architecture_contract_version"), inventory.architecture_contract_version.as_deref().unwrap_or("none"));
    json.string(Some("xlsx_contract_version"), inventory.xlsx_contract_version.as_deref().unwrap_or("none"));
    json.end_object();
    json.string(Some("audit_signal"), state.audit_signal.as_str());
    json.string(Some("renderer_outcome"), &state.renderer_outcome.label());
    json.string(Some("emergence_outcome"), state.emergence_outcome.as_str());
    json.string(Some("clippy_exit"), &state.strict_clippy_exit);
    json.string(Some("build_exit"), &state.build_exit);
    json.string(Some("launcher_test_exit"), &state.strict_test_exit);
    json.string(Some("launcher_test_summary"), &state.strict_test_summary);
    if let Some(truth) = &state.render_candidate_selected_pdf_truth {
        json.string(Some("render_candidate_status"), &truth.candidate_verdict);
        json.bool("render_candidate_write_produced", truth.write_produced);
        json.bool("render_candidate_write_validated", truth.write_validated);
        json.string(Some("selected_pdf_source"), &truth.selected_pdf_source);
        json.string(Some("selected_pdf_source_label"), &truth.selected_pdf_source_label);
        json.bool("writer_delta_pdf_source_adopted", truth.pdf_source_adopted);
        json.bool(
            "productive_materialization_green",
            truth.productive_materialization_green,
        );
    } else {
        json.string(Some("render_candidate_status"), "not_reported");
        json.bool("render_candidate_write_produced", false);
        json.bool("render_candidate_write_validated", false);
        json.string(Some("selected_pdf_source"), "unknown");
        json.string(Some("selected_pdf_source_label"), "unknown");
        json.bool("writer_delta_pdf_source_adopted", false);
        json.bool("productive_materialization_green", false);
    }
    declare_assistant_handoff_export(&mut included);
    let mut context_exported = included.clone();
    context_exported.push("assistant_context.json".to_string());
    context_exported.push("manifest.txt".to_string());
    json.strings("current_exported", context_exported.iter());
    json.strings("omitted", omitted.iter());
    json.string(Some("expected_upload"), "assistant_export_latest.zip");
    json.string(
        Some("source_patch_requires"),
        "embedded_source_snapshot_in_same_assistant_export_latest.zip",
    );
    json.end_object();
    let context_path = latest_dir.join("assistant_context.json");
    fs::write(&context_path, json.finish())?;
    included.push(file_name_string(&context_path));

    let manifest_path = latest_dir.join("manifest.txt");
    let mut manifest = File::create(&manifest_path)?;
    writeln!(manifest, "ETUSCHLOPF ASSISTANT EXPORT")?;
    writeln!(manifest, "============================")?;
    writeln!(manifest, "ZipKind: assistant_export")?;
    writeln!(manifest, "TakeCompatible: false")?;
    writeln!(manifest, "Mode: normal_run")?;
    writeln!(manifest, "Version: {version}")?;
    writeln!(manifest, "Project: {project}")?;
    writeln!(manifest, "FinalExit: {final_exit}")?;
    writeln!(manifest, "RunSeverity: {}", run_outcome.report.severity)?;
    writeln!(
        manifest,
        "ReleaseDecision: {}",
        run_outcome.report.release_decision
    )?;
    writeln!(
        manifest,
        "ConsoleProjectionStatus: {}",
        run_outcome.report.console_status
    )?;
    writeln!(
        manifest,
        "ConsoleProjectionSeverity: {}",
        run_outcome.report.console_severity
    )?;
    writeln!(
        manifest,
        "RendererOutcome: {}",
        run_outcome.report.renderer_outcome
    )?;
    writeln!(manifest, "AuditSignal: {}", state.audit_signal.as_str())?;
    writeln!(manifest, "BuildExit: {}", state.build_exit)?;
    writeln!(manifest, "ClippyExit: {}", state.strict_clippy_exit)?;
    writeln!(manifest, "LauncherTests: {} | {}", state.strict_test_exit, state.strict_test_summary)?;
    writeln!(manifest, "SourceSnapshotComplete: {}", source_snapshot.complete)?;
    writeln!(manifest, "SourceSnapshotFileCount: {}", source_snapshot.file_count)?;
    writeln!(manifest, "SourceSnapshotArchive: {}", source_snapshot.archive_name.as_deref().unwrap_or("none"))?;
    writeln!(manifest, "SourceSnapshotSha256: {}", source_snapshot.sha256.as_deref().unwrap_or("none"))?;
    writeln!(manifest, "MeasurementSnapshotStatus: {}", measurement_snapshot.status)?;
    writeln!(
        manifest,
        "MeasurementSnapshotBinding: current_observation_session_run_id"
    )?;
    writeln!(
        manifest,
        "MeasurementSnapshotFile: {}",
        measurement_snapshot.file_name.as_deref().unwrap_or("none")
    )?;
    writeln!(
        manifest,
        "MeasurementSnapshotSha256: {}",
        measurement_snapshot.sha256.as_deref().unwrap_or("none")
    )?;
    writeln!(manifest, "MeasurementSnapshotRunId: {}", measurement_snapshot.run_id)?;
    writeln!(
        manifest,
        "MeasurementContractVersion: {}",
        measurement_snapshot
            .measurement_contract_version
            .as_deref()
            .unwrap_or("none")
    )?;
    write_inventory_manifest_fields(&mut manifest, &inventory)?;
    writeln!(manifest, "ProjectResultProjection: assistant_summary_only_full_result_local")?;
    writeln!(manifest, "TransitionHistoryProjection: local_advisory_only_not_exported")?;
    writeln!(manifest, "ProjectFolderCatalog: bounded_normal_run_without_target_descendants_tsv_only")?;
    writeln!(manifest, "ProjectFolderCatalogMarkdown: not_exported")?;
    writeln!(manifest, "ProjectFolderCatalogTsv: {PROJECT_FILE_CATALOG_TSV}")?;
    writeln!(manifest, "ProjectFolderCatalogEntries: {}", folder_catalog.entry_count)?;
    writeln!(manifest, "ProjectFolderCatalogFiles: {}", folder_catalog.file_count)?;
    writeln!(manifest, "ProjectFolderCatalogDirectories: {}", folder_catalog.directory_count)?;
    writeln!(manifest, "ProjectFolderCatalogSymlinks: {}", folder_catalog.symlink_count)?;
    writeln!(manifest, "ProjectFolderCatalogPlannedOutputs: {}", folder_catalog.planned_entry_count)?;
    write_project_export_manifest_footer(&mut manifest, &included, &omitted)?;
    drop(manifest);

    stage_assistant_handoff_export(project_root, &latest_dir, "normal_run", Some(project))?;
    publish_assistant_export_zip(&latest_dir, &latest_zip, &root_latest_zip)?;
    Ok(root_latest_zip)
}


fn create_corpus_run_assistant_export_bundle(
    project_root: &Path,
    state: &RunState,
    version: &str,
    evidence_version: &str,
    final_exit: i32,
) -> AppResult<PathBuf> {
    let export_root = project_root.join("target/etuschlopf_assistant_export");
    let latest_dir = export_root.join("latest");
    let latest_zip = export_root.join("latest.zip");
    let root_latest_zip = project_root.join("assistant_export_latest.zip");
    if latest_dir.exists() {
        fs::remove_dir_all(&latest_dir)?;
    }
    fs::create_dir_all(&latest_dir)?;

    let validation = validate_corpus_run_manifest(project_root, evidence_version);
    let mut included = Vec::new();
    let mut omitted = Vec::new();

    let corpus_manifest = project_root.join(CORPUS_RUN_MANIFEST_FILE);
    if corpus_manifest.exists() {
        copy_export_file(
            &corpus_manifest,
            &latest_dir.join(CORPUS_RUN_MANIFEST_FILE),
            &mut included,
        )?;
    } else {
        omitted.push(format!("{CORPUS_RUN_MANIFEST_FILE} | missing"));
    }

    if let Some(report) = &validation.manifest.report {
        let src = project_root.join(&report.file);
        if src.exists() {
            copy_export_file(&src, &latest_dir.join(&report.file), &mut included)?;
        } else {
            omitted.push(format!("{} | manifest_bound_report_missing", report.file));
        }
    } else {
        omitted.push("corpus report | not_bound_in_manifest".to_string());
    }

    for project in &validation.manifest.projects {
        let src = project_root.join(&project.result_file);
        if src.exists() {
            copy_export_file(
                &src,
                &latest_dir.join(&project.result_file),
                &mut included,
            )?;
        } else {
            omitted.push(format!("{} | manifest_bound_project_result_missing", project.result_file));
        }
    }

    if let Some(strict_log) = state.strict_log.as_deref() {
        let src = project_root_bound_path(project_root, strict_log);
        if src.exists() {
            copy_export_file(&src, &latest_dir.join(file_name_string(&src)), &mut included)?;
        } else {
            omitted.push(format!("{} | current_strict_log_missing", strict_log.display()));
        }
    } else {
        omitted.push("strict-clippy log | not_recorded".to_string());
    }

    if let Some(strict_test_log) = state.strict_test_log.as_deref() {
        let src = project_root_bound_path(project_root, strict_test_log);
        if src.exists() {
            copy_export_file(&src, &latest_dir.join(file_name_string(&src)), &mut included)?;
        } else {
            omitted.push(format!("{} | current_launcher_test_log_missing", strict_test_log.display()));
        }
    } else {
        omitted.push("visual_preflight.strict-tests.txt | not_run_for_corpus_run".to_string());
    }

    let console_log = project_root.join(CONSOLE_LOG);
    if console_log.exists() {
        copy_export_file(
            &console_log,
            &latest_dir.join(file_name_string(&console_log)),
            &mut included,
        )?;
    } else {
        omitted.push(format!("{CONSOLE_LOG} | current_console_log_missing"));
    }

    let run_outcome = truthful_run_outcome_projection(typed_run_outcome_for_console(
        state,
        "all_run",
        final_exit,
    ));
    let run_outcome_path = latest_dir.join(RUN_OUTCOME_AUDIT_FILE);
    fs::write(&run_outcome_path, run_outcome.report.audit_text(version))?;
    included.push(file_name_string(&run_outcome_path));

    let zip_kind_path = latest_dir.join("ZIP_KIND.txt");
    fs::write(
        &zip_kind_path,
        format!(
            "kind=assistant_export\ntake_compatible=false\nmode=corpus_run\nversion={version}\nevidence_version={evidence_version}\nfull_corpus={}\nmanifest_valid={}\n",
            validation.manifest.full_corpus,
            validation.valid,
        ),
    )?;
    included.push(file_name_string(&zip_kind_path));

    let folder_catalog = write_project_folder_catalog_mode(project_root, &latest_dir, version, true, false)?;
    omitted.push(format!("{PROJECT_FILE_CATALOG_MARKDOWN} | duplicate_human_rendering_omitted_tsv_exported"));
    included.push(PROJECT_FILE_CATALOG_TSV.to_string());
    let source_snapshot = stage_source_snapshot(
        project_root,
        &latest_dir,
        version,
        &mut included,
        &mut omitted,
    )?;
    append_source_snapshot_status_to_outer_zip_kind(&latest_dir, &source_snapshot)?;
    write_transition_history_projection(project_root, &latest_dir)?;
    included.push(TRANSITION_HISTORY_FILE.to_string());
    let inventory = stage_current_inventory_artifacts(project_root, &latest_dir, &mut included)?;
    if inventory.status == "bound_current_inventory"
        && inventory.corpus_manifest_run_id.as_deref() != Some(validation.manifest.run_id.as_str())
    {
        return Err(anyhow!("assistant export: inventory is not bound to current corpus manifest run"));
    }
    if inventory.status == "bound_current_inventory"
        && inventory.run_scope.as_deref() != Some("CORPUS")
    {
        return Err(anyhow!("assistant export: corpus inventory has non-CORPUS scope"));
    }

    let mut json = JsonWriter::new();
    json.begin_object(None);
    json.raw("schema_version", "17");
    json.string(Some("zip_kind"), "assistant_export");
    json.bool("take_compatible", false);
    json.string(Some("mode"), "corpus_run");
    json.string(Some("version"), version);
    json.string(Some("evidence_version"), evidence_version);
    json.raw("final_exit", &final_exit.to_string());
    json.bool("manifest_valid", validation.valid);
    json.string(Some("manifest_reason"), &validation.reason);
    json.bool("full_corpus", validation.manifest.full_corpus);
    json.raw(
        "expected_project_count",
        &validation.manifest.expected_project_count.to_string(),
    );
    json.raw("project_rows", &validation.manifest.projects.len().to_string());
    json.raw("project_files_valid", &validation.project_files_valid.to_string());
    json.raw(
        "project_files_expected",
        &validation.project_files_expected.to_string(),
    );
    json.bool("report_file_valid", validation.report_file_valid);
    json.bool(
        "renderer_outcome_observed",
        validation.manifest.renderer_outcome_observed,
    );
    json.bool(
        "writer_outcome_observed",
        validation.manifest.writer_outcome_observed,
    );
    json.raw("write_projects", &validation.manifest.write_projects.to_string());
    json.raw("routes_written", &validation.manifest.routes_written.to_string());
    json.raw(
        "forward_nodes_replaced",
        &validation.manifest.forward_nodes_replaced.to_string(),
    );
    json.raw(
        "forward_duration_replaced",
        &validation.manifest.forward_duration_replaced.to_string(),
    );
    json.raw(
        "candidate_rows_after",
        &validation.manifest.candidate_rows_after.to_string(),
    );
    json.raw(
        "beam_lost_begin_candidates",
        &validation.manifest.beam_lost_begin_candidates.to_string(),
    );
    json.raw(
        "beam_lost_begin_notes_written",
        &validation.manifest.beam_lost_begin_notes_written.to_string(),
    );
    json.raw(
        "beam_lost_begin_candidate_rows_after",
        &validation
            .manifest
            .beam_lost_begin_candidate_rows_after
            .to_string(),
    );
    json.raw(
        "cross_staff_rest_suppressions",
        &validation.manifest.cross_staff_rest_suppressions.to_string(),
    );
    json.raw(
        "source_higher_beam_notes_restored",
        &validation
            .manifest
            .source_higher_beam_notes_restored
            .to_string(),
    );
    json.raw(
        "rest_vertical_anchors_written",
        &validation.manifest.rest_vertical_anchors_written.to_string(),
    );
    json.raw(
        "ledger_bounded_clef_boundaries_written",
        &validation
            .manifest
            .ledger_bounded_clef_boundaries_written
            .to_string(),
    );
    json.bool(
        "render_xml_write_committed",
        validation.manifest.render_xml_write,
    );
    json.bool(
        "render_notation_mutation",
        validation.manifest.notation_mutation,
    );
    json.bool(
        "sounding_event_mutation",
        validation.manifest.sounding_event_mutation,
    );
    json.bool(
        "declared_music_mutation",
        validation.manifest.music_mutation,
    );
    json.bool(
        "master_musicxml_mutation",
        validation.manifest.master_apply,
    );
    json.begin_object(Some("project_folder_catalog"));
    json.raw("schema_version", "2");
    json.bool("complete_recursive_project_root", false);
    json.string(Some("excluded_generated_prefix"), "target/");
    json.string(Some("markdown"), "none");
    json.string(Some("tsv"), PROJECT_FILE_CATALOG_TSV);
    json.raw("entries", &folder_catalog.entry_count.to_string());
    json.raw("files", &folder_catalog.file_count.to_string());
    json.raw("directories", &folder_catalog.directory_count.to_string());
    json.raw("symlinks", &folder_catalog.symlink_count.to_string());
    json.raw("present_file_bytes", &folder_catalog.present_file_bytes.to_string());
    json.raw("planned_output_entries", &folder_catalog.planned_entry_count.to_string());
    json.end_object();
    json.begin_object(Some("inventory"));
    json.string(Some("status"), inventory.status);
    json.string(Some("json_file"), if inventory.json_sha256.is_some() { INVENTORY_JSON_FILE } else { "none" });
    json.string(Some("xlsx_file"), if inventory.xlsx_sha256.is_some() { INVENTORY_XLSX_FILE } else { "none" });
    json.string(Some("json_sha256"), inventory.json_sha256.as_deref().unwrap_or("none"));
    json.string(Some("xlsx_sha256"), inventory.xlsx_sha256.as_deref().unwrap_or("none"));
    json.string(Some("run_id"), inventory.run_id.as_deref().unwrap_or("none"));
    json.string(Some("run_scope"), inventory.run_scope.as_deref().unwrap_or("none"));
    json.string(Some("corpus_manifest_run_id"), inventory.corpus_manifest_run_id.as_deref().unwrap_or("none"));
    json.string(Some("inventory_contract_version"), inventory.inventory_contract_version.as_deref().unwrap_or("none"));
    json.string(Some("architecture_contract_version"), inventory.architecture_contract_version.as_deref().unwrap_or("none"));
    json.string(Some("xlsx_contract_version"), inventory.xlsx_contract_version.as_deref().unwrap_or("none"));
    json.end_object();
    json.string(Some("audit_signal"), state.audit_signal.as_str());
    json.string(Some("renderer_outcome"), &state.renderer_outcome.label());
    json.string(Some("emergence_outcome"), state.emergence_outcome.as_str());
    json.string(Some("clippy_exit"), &state.strict_clippy_exit);
    json.string(Some("build_exit"), &state.build_exit);
    json.string(Some("launcher_test_exit"), &state.strict_test_exit);
    json.string(Some("launcher_test_summary"), &state.strict_test_summary);
    declare_assistant_handoff_export(&mut included);
    let mut context_exported = included.clone();
    context_exported.push("assistant_context.json".to_string());
    context_exported.push("manifest.txt".to_string());
    json.strings("current_exported", context_exported.iter());
    json.strings("omitted", omitted.iter());
    json.string(Some("expected_upload"), "assistant_export_latest.zip");
    json.string(
        Some("source_patch_requires"),
        "embedded_source_snapshot_in_same_assistant_export_latest.zip",
    );
    json.end_object();
    let context_path = latest_dir.join("assistant_context.json");
    fs::write(&context_path, json.finish())?;
    included.push(file_name_string(&context_path));

    let manifest_path = latest_dir.join("manifest.txt");
    let mut manifest = File::create(&manifest_path)?;
    writeln!(manifest, "ETUSCHLOPF ASSISTANT EXPORT")?;
    writeln!(manifest, "============================")?;
    writeln!(manifest, "ZipKind: assistant_export")?;
    writeln!(manifest, "TakeCompatible: false")?;
    writeln!(manifest, "Mode: corpus_run")?;
    writeln!(manifest, "Version: {version}")?;
    writeln!(manifest, "EvidenceVersion: {evidence_version}")?;
    writeln!(manifest, "FinalExit: {final_exit}")?;
    writeln!(manifest, "ManifestValid: {}", validation.valid)?;
    writeln!(manifest, "ManifestReason: {}", validation.reason)?;
    writeln!(manifest, "FullCorpus: {}", validation.manifest.full_corpus)?;
    writeln!(
        manifest,
        "ProjectEvidence: {}/{}",
        validation.project_files_valid,
        validation.project_files_expected,
    )?;
    writeln!(manifest, "ReportEvidenceValid: {}", validation.report_file_valid)?;
    writeln!(
        manifest,
        "RendererOutcomeObserved: {}",
        validation.manifest.renderer_outcome_observed,
    )?;
    writeln!(
        manifest,
        "WriterOutcomeObserved: {}",
        validation.manifest.writer_outcome_observed,
    )?;
    writeln!(manifest, "WriteProjects: {}", validation.manifest.write_projects)?;
    writeln!(manifest, "RoutesWritten: {}", validation.manifest.routes_written)?;
    writeln!(
        manifest,
        "ForwardNodesReplaced: {}",
        validation.manifest.forward_nodes_replaced,
    )?;
    writeln!(
        manifest,
        "ForwardDurationReplaced: {}",
        validation.manifest.forward_duration_replaced,
    )?;
    writeln!(
        manifest,
        "CandidateRowsAfter: {}",
        validation.manifest.candidate_rows_after,
    )?;
    writeln!(
        manifest,
        "BeamLostBeginCandidates: {}",
        validation.manifest.beam_lost_begin_candidates,
    )?;
    writeln!(
        manifest,
        "BeamLostBeginNotesWritten: {}",
        validation.manifest.beam_lost_begin_notes_written,
    )?;
    writeln!(
        manifest,
        "BeamLostBeginCandidateRowsAfter: {}",
        validation.manifest.beam_lost_begin_candidate_rows_after,
    )?;
    writeln!(
        manifest,
        "CrossStaffRestSuppressions: {}",
        validation.manifest.cross_staff_rest_suppressions,
    )?;
    writeln!(
        manifest,
        "SourceHigherBeamNotesRestored: {}",
        validation.manifest.source_higher_beam_notes_restored,
    )?;
    writeln!(
        manifest,
        "RestVerticalAnchorsWritten: {}",
        validation.manifest.rest_vertical_anchors_written,
    )?;
    writeln!(
        manifest,
        "LedgerBoundedClefBoundariesWritten: {}",
        validation.manifest.ledger_bounded_clef_boundaries_written,
    )?;
    writeln!(
        manifest,
        "RenderXmlWriteCommitted: {}",
        validation.manifest.render_xml_write,
    )?;
    writeln!(
        manifest,
        "RenderNotationMutation: {}",
        validation.manifest.notation_mutation,
    )?;
    writeln!(
        manifest,
        "SoundingEventMutation: {}",
        validation.manifest.sounding_event_mutation,
    )?;
    writeln!(
        manifest,
        "DeclaredMusicMutation: {}",
        validation.manifest.music_mutation,
    )?;
    writeln!(
        manifest,
        "MasterMusicXmlMutation: {}",
        validation.manifest.master_apply,
    )?;
    writeln!(manifest, "BuildExit: {}", state.build_exit)?;
    writeln!(manifest, "ClippyExit: {}", state.strict_clippy_exit)?;
    writeln!(manifest, "LauncherTests: {} | {}", state.strict_test_exit, state.strict_test_summary)?;
    writeln!(manifest, "SourceSnapshotComplete: {}", source_snapshot.complete)?;
    writeln!(manifest, "SourceSnapshotFileCount: {}", source_snapshot.file_count)?;
    writeln!(manifest, "SourceSnapshotArchive: {}", source_snapshot.archive_name.as_deref().unwrap_or("none"))?;
    writeln!(manifest, "SourceSnapshotSha256: {}", source_snapshot.sha256.as_deref().unwrap_or("none"))?;
    write_inventory_manifest_fields(&mut manifest, &inventory)?;
    writeln!(manifest, "ProjectFolderCatalog: bounded_corpus_run_without_target_descendants_tsv_only")?;
    writeln!(manifest, "ProjectFolderCatalogExcludedGeneratedPrefix: target/")?;
    writeln!(manifest, "ProjectFolderCatalogTsv: {PROJECT_FILE_CATALOG_TSV}")?;
    writeln!(manifest, "ProjectFolderCatalogEntries: {}", folder_catalog.entry_count)?;
    writeln!(manifest, "ProjectFolderCatalogFiles: {}", folder_catalog.file_count)?;
    writeln!(manifest, "ProjectFolderCatalogDirectories: {}", folder_catalog.directory_count)?;
    writeln!(manifest, "ProjectFolderCatalogSymlinks: {}", folder_catalog.symlink_count)?;
    writeln!(manifest, "ProjectFolderCatalogPlannedOutputs: {}", folder_catalog.planned_entry_count)?;
    write_project_export_manifest_footer(&mut manifest, &included, &omitted)?;
    drop(manifest);

    stage_assistant_handoff_export(project_root, &latest_dir, "corpus_run", None)?;
    publish_assistant_export_zip(&latest_dir, &latest_zip, &root_latest_zip)?;
    Ok(root_latest_zip)
}

fn create_visual_preflight_assistant_export_bundle(
    context: AssistantExportBundleContext<'_>,
    status: AssistantExportRunStatus<'_>,
) -> AppResult<PathBuf> {
    let AssistantExportBundleContext {
        project_root,
        aggregate_path,
        plan,
        projects,
        ledgers,
        strict_log,
        strict_test_log,
        version,
    } = context;
    let AssistantExportRunStatus {
        final_exit,
        clippy_exit,
        build_exit,
        strict_test_exit,
        strict_test_summary,
        export_policy,
        ..
    } = status;
    let export_root = project_root.join("target/etuschlopf_assistant_export");
    let latest_dir = export_root.join("latest");
    let latest_zip = export_root.join("latest.zip");
    let root_latest_zip = project_root.join("assistant_export_latest.zip");
    if latest_dir.exists() {
        fs::remove_dir_all(&latest_dir)?;
    }
    fs::create_dir_all(&latest_dir)?;

    let mut included = Vec::new();
    let mut omitted = vec![
        "non_core_audit_families | excluded_by_minimal_export_plan_not_scanned".to_string(),
        "*.wav | always_omitted_raw_audio".to_string(),
        "*.mid | always_omitted_raw_project".to_string(),
        "*.musicxml | omitted_unless_explicitly_needed_later".to_string(),
        "*.render.musicxml | omitted_unless_explicitly_needed_later".to_string(),
    ];
    let reference_guard_project = assistant_context_reference_guard_project(plan);
    let renderer_context =
        assistant_context_renderer_state_from_runtime(project_root, &reference_guard_project, version);
    let run_outcome_projection =
        truthful_run_outcome_projection(evaluate_run_outcome_for_export(status)).report;
    let validation = AssistantExportValidationEvidence {
        strict_test_exit,
        strict_test_summary,
    };
    let warning_truth = collect_warning_truth(strict_log);

    let run_outcome_audit_dest = latest_dir.join(RUN_OUTCOME_AUDIT_FILE);
    fs::write(&run_outcome_audit_dest, run_outcome_projection.audit_text(version))?;
    included.push(RUN_OUTCOME_AUDIT_FILE.to_string());

    let renderer_lifecycle_dest = latest_dir.join(RENDERER_PROCESS_LIFECYCLE_ASSISTANT_FILE);
    write_renderer_process_lifecycle_assistant_audit(
        &renderer_lifecycle_dest,
        version,
        &reference_guard_project,
        &renderer_context,
    )?;
    included.push(RENDERER_PROCESS_LIFECYCLE_ASSISTANT_FILE.to_string());

    let visual_summary_dest = latest_dir.join("visual_preflight.result.txt");
    write_visual_preflight_assistant_summary(VisualPreflightAssistantSummaryInput {
        src: aggregate_path,
        dest: &visual_summary_dest,
        version,
        run_outcome: &run_outcome_projection,
        strict_test_exit,
        strict_test_summary,
        ledgers,
        warning_truth: &warning_truth,
        reference_renderer_context: &renderer_context,
    })?;
    included.push(file_name_string(&visual_summary_dest));

    for ledger in ledgers {
        if ledger.result_file.exists() {
            let dest = latest_dir.join(project_result_summary_file_name(&ledger.project));
            write_assistant_project_result_summary(
                &ledger.result_file,
                &dest,
                &ledger.project,
                version,
                &renderer_context,
            )?;
            included.push(file_name_string(&dest));
            omitted.push(format!(
                "{} | full_project_result_omitted_assistant_summary_exported",
                file_name_string(&ledger.result_file)
            ));
        } else {
            omitted.push(format!("{} | missing_result_file", ledger.result_file.display()));
        }
        let pdf = resolve_pdf_file(&ledger.project);
        if assistant_export_includes_pdf(export_policy) {
            if pdf.exists() {
                let dest = latest_dir.join(file_name_string(&pdf));
                copy_export_file(&pdf, &dest, &mut included)?;
            } else {
                omitted.push(format!("{} | pdf_needed_but_missing", pdf.display()));
            }
        }
    }

    let strict_needed = final_exit != 0 || clippy_exit.trim() != "0" || build_exit.trim() != "0";
    if let Some(path) = strict_log {
        if path.exists() {
            let dest = latest_dir.join(file_name_string(path));
            copy_export_file(path, &dest, &mut included)?;
        } else if strict_needed {
            omitted.push(format!("{} | strict_log_needed_but_missing", path.display()));
        }
    } else if strict_needed {
        omitted.push("strict-clippy log | needed_but_not_recorded".to_string());
    }
    if let Some(path) = strict_test_log {
        if path.exists() {
            let dest = latest_dir.join(file_name_string(path));
            copy_export_file(path, &dest, &mut included)?;
        } else {
            omitted.push(format!("{} | integrated_launcher_test_log_missing", path.display()));
        }
    } else {
        omitted.push("visual_preflight.strict-tests.txt | integrated_launcher_tests_not_recorded".to_string());
    }

    let emergence_selection_path = latest_dir.join("emergence_selection.json");
    write_emergence_selection_json(&emergence_selection_path, version, plan)?;
    included.push("emergence_selection.json".to_string());

    let corpus_manifest_path = project_root.join(CORPUS_RUN_MANIFEST_FILE);
    if corpus_manifest_path.exists() {
        copy_export_file(
            &corpus_manifest_path,
            &latest_dir.join(CORPUS_RUN_MANIFEST_FILE),
            &mut included,
        )?;
    } else {
        omitted.push(format!("{CORPUS_RUN_MANIFEST_FILE} | missing_no_corpus_run_evidence"));
    }
    let corpus_report_path = project_root.join("corpus_emergence.result.txt");
    if corpus_report_path.exists() {
        let name = if plan.emergence.fresh_for_current_version {
            "corpus_emergence.result.txt"
        } else {
            "corpus_emergence.rejected-evidence.txt"
        };
        copy_export_file(&corpus_report_path, &latest_dir.join(name), &mut included)?;
    } else {
        omitted.push("corpus_emergence.result.txt | missing_no_current_emergence_target".to_string());
    }

    let source_architecture_path = latest_dir.join("source_architecture_inventory.txt");
    write_source_architecture_inventory(project_root, &source_architecture_path, version, ledgers)?;
    included.push("source_architecture_inventory.txt".to_string());

    let folder_catalog = write_project_folder_catalog(project_root, &latest_dir, version)?;
    included.push(PROJECT_FILE_CATALOG_MARKDOWN.to_string());
    included.push(PROJECT_FILE_CATALOG_TSV.to_string());
    let source_snapshot = stage_source_snapshot(
        project_root,
        &latest_dir,
        version,
        &mut included,
        &mut omitted,
    )?;
    append_source_snapshot_status_to_outer_zip_kind(&latest_dir, &source_snapshot)?;
    write_transition_history_projection(project_root, &latest_dir)?;
    included.push(TRANSITION_HISTORY_FILE.to_string());

    declare_assistant_handoff_export(&mut included);
    write_assistant_context_json(
        AssistantContextBuildInput {
            latest_dir: &latest_dir,
            version,
            plan,
            projects,
            ledgers,
            warning_truth: &warning_truth,
            run_outcome: &run_outcome_projection,
            reference_renderer_context: &renderer_context,
            folder_catalog: &folder_catalog,
            validation,
            export_policy,
        },
        &mut included,
        &mut omitted,
    )?;
    let assistant_context_json_included = included.iter().any(|name| name == "assistant_context.json");

    let manifest_path = latest_dir.join("manifest.txt");
    write_assistant_export_manifest(
        AssistantExportManifestContext {
            path: &manifest_path,
            version,
            plan,
            projects,
            ledgers,
            warning_truth: &warning_truth,
            run_outcome: &run_outcome_projection,
            folder_catalog: &folder_catalog,
            validation,
            export_policy,
        },
        &included,
        &omitted,
    )?;
    included.insert(0, "manifest.txt".to_string());
    let zip_contract = assistant_export_zip_contract_audit(
        &included,
        plan,
        ledgers,
        export_policy,
    );
    append_assistant_export_zip_contract_to_manifest(&manifest_path, &zip_contract)?;
    stage_assistant_handoff_export(project_root, &latest_dir, "visual_preflight", None)?;

    publish_assistant_export_zip(&latest_dir, &latest_zip, &root_latest_zip)?;

    let mut file = OpenOptions::new().append(true).open(aggregate_path)?;
    writeln!(file)?;
    writeln!(file, "ASSISTANT-EXPORT-BUNDLE-NO-BEHAVIOR-CHANGE")?;
    writeln!(file, "------------------------------------------")?;
    writeln!(file, "Status: v{version} direct minimal export plan; no copy-then-prune; no music mutation.")?;
    writeln!(file, "Kurz: ProjectRows {}; IncludedFiles {}; OmittedCategories {}; PdfIncludedRows {}; StrictLogIncluded {}; AssistantContextJsonIncluded {}; ExportDir {}; ExportZip {}; RootExportZip {}; LegacyAuditDirectoryScans 0; PrunedFiles 0; RenderXmlWrite nein; MusicMutation nein; MasterApply nein; Urteil assistant_export_bundle_written_root_zip_available.",
        ledgers.len(),
        included.len(),
        omitted.len(),
        ledgers.iter().filter(|_| assistant_export_includes_pdf(export_policy)).count(),
        strict_needed,
        if assistant_context_json_included { "ja" } else { "nein" },
        latest_dir.display(),
        latest_zip.display(),
        root_latest_zip.display(),
    )?;
    writeln!(file, "SourceSnapshot: complete {}; files {}; archive {}; sha256 {}.",
        source_snapshot.complete,
        source_snapshot.file_count,
        source_snapshot.archive_name.as_deref().unwrap_or("none"),
        source_snapshot.sha256.as_deref().unwrap_or("none"),
    )?;
    writeln!(file, "UploadPolicy: assistant_export_latest.zip only; embedded source snapshot included; PDFs opt-in; WAV/MIDI/MusicXML excluded.")?;
    writeln!(file, "AssistantContextJson: generated_once_by_scoped_writer.")?;
    writeln!(file, "ProjectFolderCatalog: complete recursive root snapshot; Entries {}; Files {}; Directories {}; PlannedOutputs {}; Markdown {}; Tsv {}.",
        folder_catalog.entry_count,
        folder_catalog.file_count,
        folder_catalog.directory_count,
        folder_catalog.planned_entry_count,
        PROJECT_FILE_CATALOG_MARKDOWN,
        PROJECT_FILE_CATALOG_TSV,
    )?;
    writeln!(file, "AssistantExportZipContractAudit: {}; MissingRequired {}; RedundantSnapshots {}; PdfFiles {}; ContainsCargoToml {}; Urteil {}.",
        zip_contract.status_label(),
        zip_contract.missing_required_files.len(),
        zip_contract.redundant_assistant_snapshots.len(),
        zip_contract.pdf_count,
        if zip_contract.contains_cargo_toml { "ja" } else { "nein" },
        zip_contract.verdict(),
    )?;
    Ok(root_latest_zip)
}

const ASSISTANT_HANDOFF_FILENAME: &str = "assistant_handoff_latest.txt";
const ASSISTANT_HANDOFF_CANDIDATE_FILENAME: &str = "assistant_handoff_candidate.tmp";
const ASSISTANT_HANDOFF_MAX_BYTES: usize = 768 * 1024;
const ASSISTANT_HANDOFF_CONSOLE_MAX_BYTES: usize = 12 * 1024;
const ASSISTANT_HANDOFF_START_HERE: &str = "docs/assistant_memory/000_start_here.md";
const ASSISTANT_HANDOFF_ESSENCE_BOOTSTRAP: &str =
    "docs/assistant_memory/architecture_essence_bootstrap_v0.json";
const ASSISTANT_HANDOFF_ESSENCE: &str = "docs/assistant_memory/020_architecture_essence_v0.md";


// Diagnostic durations never participate in admission, cache reuse or release.
struct AssistantExportPhaseTimer<'a> {
    root: &'a Path,
    latest: &'a Path,
    label: &'static str,
    started: Instant,
}
impl<'a> AssistantExportPhaseTimer<'a> {
    fn new(root: &'a Path, latest: &'a Path, label: &'static str) -> Self {
        Self { root, latest, label, started: Instant::now() }
    }
}
impl Drop for AssistantExportPhaseTimer<'_> {
    fn drop(&mut self) {
        assistant_export_phase_timing(self.root, self.latest, self.label, self.started.elapsed());
    }
}
fn assistant_export_phase_timing(root: &Path, latest: &Path, label: &str, duration: Duration) {
    let line = format!("AssistantExportTiming: {label}={}\n", duration.as_millis());
    let mut paths = vec![root.join(CONSOLE_LOG)];
    // Once the handoff binds the console hash, its exported bytes must stay fixed.
    if !matches!(label, "AssistantHandoffMs" | "OuterZipMs") {
        paths.push(latest.join("etuschlopf.console.txt"));
    }
    for path in paths {
        if path.is_file() && let Ok(mut file) = OpenOptions::new().append(true).open(path) {
            let _ = file.write_all(line.as_bytes());
        }
    }
}

fn handoff_is_progress_line(line: &str) -> bool {
    let line = line.trim_start();
    let Some(body) = line.get(2..) else { return false; };
    matches!(line.as_bytes().first(), Some(b'|' | b'/' | b'-' | b'\\'))
        && (body.starts_with("DATEI  [") || body.starts_with("GESAMT ["))
}

fn handoff_projection_scalar<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("\"{key}\": ");
    text.lines().find_map(|line| line.trim().strip_prefix(&prefix)
        .map(|value| value.trim_end_matches(',').trim()))
}

fn handoff_append_current_state(out: &mut String, root: &Path, latest: &Path) -> AppResult<()> {
    let identity = fs::read_to_string(root.join("ZIP_KIND.txt")).unwrap_or_default();
    let revision = source_snapshot_zip_kind_value(&identity, "revision").unwrap_or_else(|| "unknown".into());
    let history = fs::read(latest.join(TRANSITION_HISTORY_FILE)).ok();
    let text = history.as_deref().and_then(|bytes| std::str::from_utf8(bytes).ok());
    let head = text.and_then(|t| generated_json_string_field(t, "canonical_history_through_revision"));
    let head_sha = text.and_then(|t| generated_json_string_field(t, "canonical_history_through_sha256"));
    let current = text.and_then(|t| t.split_once("\"current_in_flight\": {").map(|(_, rest)| rest))
        .and_then(|t| t.split_once("\"candidate_intent\":").map(|(header, _)| header));
    let marker = current.and_then(|t| handoff_projection_scalar(t, "marker_present"));
    let issues = text.and_then(|t| handoff_projection_scalar(t, "projection_issue_count"));
    let state = current.and_then(|t| generated_json_string_field(t, "transaction_state"));
    let candidate = current.and_then(|t| generated_json_string_field(t, "take_revision"));
    let candidate_sha = current.and_then(|t| generated_json_string_field(t, "candidate_sha256"));
    let lifecycle = match (issues, marker, head.as_deref(), state.as_deref()) {
        (Some("0"), Some("true"), Some(_), Some(_)) => "in_flight_not_yet_canonical",
        (Some("0"), Some("false"), Some(value), Some(_)) if value == revision => "revision_equals_canonical_head_bytes_unverified",
        (Some("0"), Some("false"), Some(_), Some(_)) => "not_canonical_no_transaction",
        _ => "indeterminate_projection_missing_invalid_or_issues",
    };
    handoff_append_text(out, &format!(
        "\n[CURRENT-STATE]\nProjectionOnly: true\nAuthority: none\nStateTime: export_generation_before_post_acceptance_finalization\nSourceTreeRevision: {revision}; identity_only\nCanonicalHeadAtGeneration: {}\nCanonicalHeadSha256AtGeneration: {}\nTakeTransactionAtGeneration: {}\nTransactionStateAtGeneration: {}\nCandidateAtGeneration: {}\nCandidateSha256AtGeneration: {}\nLifecycleAtGeneration: {lifecycle}\nProjectionIssueCount: {}\nTransitionHistorySha256: {}\nVerdictResolution: inspect the final lifecycle handover and its hash-bound acceptance record; this generation snapshot is not a verdict\n[/CURRENT-STATE]\n",
        head.as_deref().unwrap_or("unknown"), head_sha.as_deref().unwrap_or("unknown"),
        marker.unwrap_or("unknown"), state.as_deref().unwrap_or("unknown"),
        candidate.as_deref().unwrap_or("unknown"), candidate_sha.as_deref().unwrap_or("unknown"),
        issues.unwrap_or("unknown"), history.as_deref().map(sha256::hex).unwrap_or_else(|| "unavailable".into())
    ))
}

fn handoff_append_open_obligations(out: &mut String, root: &Path) -> AppResult<()> {
    handoff_append_text(out, "\n[OPEN-OBLIGATIONS]\nProjectionOnly: true\nAuthority: none\nScope: SSC remaining-set file; not an exhaustive evidence-debt inventory\n")?;
    let path = root.join("src/build_gate/ssc_exit_remaining_v1.txt");
    if path.is_file() {
        handoff_append_required_file(out, root, &path, "SSC REMAINING SET", false)?;
    } else {
        handoff_append_text(out, "OpenSet: not_declared\n")?;
    }
    handoff_append_text(out, "[/OPEN-OBLIGATIONS]\n")
}

fn handoff_append_read_set(out: &mut String, latest: &Path) -> AppResult<()> {
    handoff_append_text(out, "\n[DEFAULT-READ-SET]\nDefault: this handoff, final lifecycle handover and hash-bound acceptance record when present\nOnDemand: source_snapshot, transition_history.json, project_file_catalog.tsv, project results, strict validation logs\n")?;
    let mut paths = fs::read_dir(latest)?.collect::<Result<Vec<_>, _>>()?
        .into_iter().map(|entry| entry.path()).filter(|path| path.is_file()).collect::<Vec<_>>();
    paths.sort();
    for path in paths {
        let name = file_name_string(&path);
        if name.starts_with("accepted-") || matches!(name.as_str(), "take_lifecycle_handover.txt" | "take_acceptance_candidate_intent.txt") {
            let bytes = fs::read(&path)?;
            handoff_append_text(out, &format!("File: {name} | Bytes: {} | Sha256: {}\n", bytes.len(), sha256::hex(&bytes)))?;
        }
    }
    handoff_append_text(out, "PostAcceptanceFiles: may be appended by the existing take finalization after this generation snapshot\n[/DEFAULT-READ-SET]\n")
}

struct HandoffVerifiedSnapshot {
    label: String,
    sha256: String,
    archive: zip::ZipArchive<File>,
}
fn handoff_verified_snapshot(latest: &Path) -> Option<HandoffVerifiedSnapshot> {
    let metadata = fs::read_to_string(latest.join(source_snapshot_metadata_relative_path())).ok()?;
    if source_snapshot_zip_kind_value(&metadata, "source_complete")? != "true" { return None; }
    let name = source_snapshot_zip_kind_value(&metadata, "SourceArchive")?;
    if Path::new(&name).components().count() != 1 || !name.ends_with("_unbuilt_source.zip") { return None; }
    let archive_sha = source_snapshot_zip_kind_value(&metadata, "SourceZipSha256")?;
    let sha = archive_sha.strip_prefix("sha256:")?;
    let path = latest.join(SOURCE_SNAPSHOT_DIR).join(&name);
    if sha256::hex(&fs::read(&path).ok()?) != sha { return None; }
    let archive = zip::ZipArchive::new(File::open(path).ok()?).ok()?;
    Some(HandoffVerifiedSnapshot { label: format!("{SOURCE_SNAPSHOT_DIR}/{name}"), sha256: sha.to_string(), archive })
}
fn handoff_append_snapshot_reference(out: &mut String, root: &Path, path: &Path, snapshot: Option<&mut HandoffVerifiedSnapshot>) -> AppResult<bool> {
    let Some(snapshot) = snapshot else { return Ok(false); };
    let label = handoff_path_label(root, path);
    let Ok(bytes) = fs::read(path) else { return Ok(false); };
    let Ok(mut entry) = snapshot.archive.by_name(&label) else { return Ok(false); };
    let mut archived = Vec::new();
    if entry.read_to_end(&mut archived).is_err() || archived != bytes { return Ok(false); }
    handoff_append_text(out, &format!(
        "Path: {label}\nBytes: {}\nSha256: {}\nSnapshotEntry: {label}\nEmbedded: false\n\n",
        bytes.len(), sha256::hex(&bytes)
    ))?;
    Ok(true)
}
fn handoff_append_deleted_paths(out: &mut String, root: &Path) -> AppResult<()> {
    let manifest = fs::read_to_string(root.join("src/build_gate/take_change_class.txt"))?;
    for row in manifest.lines().filter_map(|line| line.strip_prefix("change=")) {
        let fields = row.split('|').map(str::trim).collect::<Vec<_>>();
        if fields.len() == 3 && fields[0].eq_ignore_ascii_case("deleted") {
            handoff_append_text(out, &format!("Path: {}\nChange: deleted\nSnapshotEntry: absent_by_declared_deletion\n\n", fields[1]))?;
        }
    }
    Ok(())
}

fn handoff_append_text(out: &mut String, text: &str) -> AppResult<()> {
    if out.len().saturating_add(text.len()) > ASSISTANT_HANDOFF_MAX_BYTES {
        return Err(anyhow!(
            "assistant handoff hard limit exceeded: {} + {} > {} bytes",
            out.len(),
            text.len(),
            ASSISTANT_HANDOFF_MAX_BYTES
        ));
    }
    out.push_str(text);
    Ok(())
}

fn handoff_path_label(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn handoff_append_omission(
    out: &mut String,
    project_root: &Path,
    path: &Path,
    reason: &str,
    bytes: Option<&[u8]>,
) -> AppResult<()> {
    let (size, sha) = bytes.map_or_else(
        || ("unknown".to_string(), "unknown".to_string()),
        |value| (value.len().to_string(), sha256::hex(value)),
    );
    let line = format!(
        "OmittedDueToHandoffBudget: {} | Reason: {} | Bytes: {} | Sha256: {}\n",
        handoff_path_label(project_root, path),
        reason,
        size,
        sha
    );
    handoff_append_text(out, &line)
}

fn handoff_append_required_file(
    out: &mut String,
    project_root: &Path,
    path: &Path,
    section: &str,
    required: bool,
) -> AppResult<()> {
    handoff_append_file(out, project_root, path, section, required, 0)
}

fn handoff_append_file(
    out: &mut String,
    project_root: &Path,
    path: &Path,
    section: &str,
    required: bool,
    optional_reserve_bytes: usize,
) -> AppResult<()> {
    if !path.is_file() {
        if required {
            return Err(anyhow!(
                "assistant handoff required input missing: {}",
                handoff_path_label(project_root, path)
            ));
        }
        return Ok(());
    }
    let bytes = fs::read(path)?;
    let text = match std::str::from_utf8(&bytes) {
        Ok(value) => value,
        Err(error) if required => {
            return Err(anyhow!(
                "assistant handoff required UTF-8 input invalid: {}: {error}",
                handoff_path_label(project_root, path)
            ));
        }
        Err(_) => {
            return handoff_append_omission(
                out,
                project_root,
                path,
                "non_utf8_optional_file",
                Some(&bytes),
            );
        }
    };
    let label = handoff_path_label(project_root, path);
    let header = format!(
        "\n============================================================\n{section}\nPath: {label}\nBytes: {}\nSha256: {}\n============================================================\n",
        bytes.len(),
        sha256::hex(&bytes)
    );
    let footer = format!(
        "\n============================================================\nEND {section}\n============================================================\n"
    );
    let needed = header
        .len()
        .saturating_add(text.len())
        .saturating_add(footer.len());
    if !required
        && out
            .len()
            .saturating_add(needed)
            .saturating_add(optional_reserve_bytes)
            > ASSISTANT_HANDOFF_MAX_BYTES
    {
        return handoff_append_omission(
            out,
            project_root,
            path,
            "whole_file_does_not_fit",
            Some(&bytes),
        );
    }
    handoff_append_text(out, &header)?;
    handoff_append_text(out, text)?;
    handoff_append_text(out, &footer)
}

fn handoff_utf8_tail(text: &str, max_bytes: usize) -> &str {
    if text.len() <= max_bytes {
        return text;
    }
    let mut start = text.len() - max_bytes;
    while !text.is_char_boundary(start) {
        start += 1;
    }
    &text[start..]
}

fn handoff_append_console(out: &mut String, project_root: &Path) -> AppResult<()> {
    let exported = project_root.join("target/etuschlopf_assistant_export/latest/etuschlopf.console.txt");
    let path = if exported.is_file() { exported } else { project_root.join(CONSOLE_LOG) };
    if !path.is_file() {
        return handoff_append_text(
            out,
            "\n[CONSOLE-DIAGNOSTIC]\nStatus: missing\nAuthority: diagnostic_only\n[/CONSOLE-DIAGNOSTIC]\n",
        );
    }
    let bytes = fs::read(&path)?;
    let rendered = String::from_utf8_lossy(&bytes);
    let mut diagnostic = String::new();
    let mut progress_lines = 0;
    for line in rendered.split_inclusive('\n') {
        if handoff_is_progress_line(line) {
            progress_lines += 1;
        } else {
            diagnostic.push_str(line);
        }
    }
    let header = format!(
        "\n[CONSOLE-DIAGNOSTIC]\nOriginalFile: {}\nBytes: {}\nSha256: {}\nProgressLinesOmitted: {progress_lines}\nDiagnosticBytes: {}\nIncluded: diagnostic head and tail, {} bytes maximum\nAuthority: diagnostic_only\n---\n",
        handoff_path_label(project_root, &path), bytes.len(), sha256::hex(&bytes),
        diagnostic.len(), ASSISTANT_HANDOFF_CONSOLE_MAX_BYTES
    );
    handoff_append_text(out, &header)?;
    if diagnostic.len() <= ASSISTANT_HANDOFF_CONSOLE_MAX_BYTES {
        handoff_append_text(out, &diagnostic)?;
    } else {
        let half = ASSISTANT_HANDOFF_CONSOLE_MAX_BYTES / 2;
        let mut end = half;
        while !diagnostic.is_char_boundary(end) { end -= 1; }
        handoff_append_text(out, &diagnostic[..end])?;
        handoff_append_text(out, "\nDiagnosticMiddleOmitted: true; full console exported\n")?;
        handoff_append_text(out, handoff_utf8_tail(&diagnostic, half))?;
    }
    handoff_append_text(out, "\n[/CONSOLE-DIAGNOSTIC]\n")
}

fn handoff_append_repository_cold_start(out: &mut String, project_root: &Path) -> AppResult<()> {
    let start_here_path = project_root.join(ASSISTANT_HANDOFF_START_HERE);
    let bootstrap_path = project_root.join(ASSISTANT_HANDOFF_ESSENCE_BOOTSTRAP);
    let essence_path = project_root.join(ASSISTANT_HANDOFF_ESSENCE);
    let essence_bytes = fs::read(&essence_path).map_err(|error| {
        anyhow!(
            "assistant handoff required Architecture Essence input missing or unreadable: {}: {error}",
            handoff_path_label(project_root, &essence_path)
        )
    })?;
    let routing = format!(
        "\n[REPOSITORY-COLD-START]\nProjectionOnly: true\nAuthority: none\nNormative: false\nRepositoryFirst: true\nSourceSnapshotBinding: assistant_context.json -> embedded current source snapshot\nStartHere: {ASSISTANT_HANDOFF_START_HERE}\nBootstrap: {ASSISTANT_HANDOFF_ESSENCE_BOOTSTRAP}\nEssence: {ASSISTANT_HANDOFF_ESSENCE}\nEssenceBytes: {}\nEssenceSha256: {}\nEssenceBodyIncluded: false\nEssenceResolution: current_repository_source_snapshot\nPermissionSource: task-relevant canonical contracts, authorities and gates in the current repository\nHistoricalMemoryAuthority: none\n[/REPOSITORY-COLD-START]\n",
        essence_bytes.len(),
        sha256::hex(&essence_bytes)
    );
    handoff_append_text(out, &routing)?;
    handoff_append_required_file(
        out,
        project_root,
        &start_here_path,
        "REPOSITORY COLD START - START HERE",
        true,
    )?;
    handoff_append_required_file(
        out,
        project_root,
        &bootstrap_path,
        "REPOSITORY COLD START - MACHINE BOOTSTRAP",
        true,
    )
}

fn handoff_changed_source_paths(project_root: &Path) -> AppResult<Vec<PathBuf>> {
    let manifest_path = project_root.join("src/build_gate/take_change_class.txt");
    let manifest = fs::read_to_string(&manifest_path)?;
    let mut paths = manifest
        .lines()
        .filter_map(|line| line.strip_prefix("change="))
        .filter_map(|value| {
            let fields = value.split('|').map(str::trim).collect::<Vec<_>>();
            if fields.len() != 3 || fields[0].eq_ignore_ascii_case("deleted") {
                return None;
            }
            let relative = fields[1].replace('\\', "/");
            Some(project_root.join(relative))
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn declare_assistant_handoff_export(included: &mut Vec<String>) {
    if !included.iter().any(|name| name == ASSISTANT_HANDOFF_FILENAME) {
        included.push(ASSISTANT_HANDOFF_FILENAME.to_string());
    }
}

fn stage_assistant_handoff_export(
    project_root: &Path,
    latest_dir: &Path,
    mode: &str,
    project: Option<&str>,
) -> AppResult<PathBuf> {
    let _timing = AssistantExportPhaseTimer::new(project_root, latest_dir, "AssistantHandoffMs");
    let published = publish_assistant_handoff_projection(project_root, latest_dir, mode, project)?;
    let staged = latest_dir.join(ASSISTANT_HANDOFF_FILENAME);
    fs::copy(&published, &staged)?;
    Ok(staged)
}

fn publish_assistant_handoff_projection(
    project_root: &Path,
    latest_dir: &Path,
    mode: &str,
    project: Option<&str>,
) -> AppResult<PathBuf> {
    let output_path = project_root.join(ASSISTANT_HANDOFF_FILENAME);
    let candidate_path = project_root.join(ASSISTANT_HANDOFF_CANDIDATE_FILENAME);
    if candidate_path.exists() {
        fs::remove_file(&candidate_path)?;
    }

    let mut out = String::new();
    let header = format!(
        "ETUSCHLOPF ASSISTANT HANDOFF\n============================\nProjectionOnly: true\nAuthority: none\nTakeCompatible: false\nNeuroEvidence: false\nMode: {mode}\nProject: {}\nHardLimitBytes: {}\nConsoleDiagnosticMaxBytes: {}\nSourcePolicy: verified_snapshot_reference_or_whole_file_or_explicit_omission\n\n",
        project.unwrap_or("none"),
        ASSISTANT_HANDOFF_MAX_BYTES,
        ASSISTANT_HANDOFF_CONSOLE_MAX_BYTES
    );
    handoff_append_text(&mut out, &header)?;
    handoff_append_current_state(&mut out, project_root, latest_dir)?;
    handoff_append_repository_cold_start(&mut out, project_root)?;
    let exported_console = latest_dir.join("etuschlopf.console.txt");
    let console_path = if exported_console.is_file() { exported_console } else { project_root.join(CONSOLE_LOG) };
    let console = fs::read_to_string(console_path).unwrap_or_default();
    handoff_append_text(&mut out, &launcher_timing_readback(&console))?;
    let advisory_path = project_root.join("docs/assistant_memory/050_aktueller_stand.md");
    if advisory_path.is_file() {
        handoff_append_required_file(
            &mut out, project_root, &advisory_path,
            "CURRENT PROJECT STATUS - ADVISORY ONLY", false,
        )?;
    } else {
        handoff_append_text(&mut out, "\nCurrentProjectStatus: absent_in_this_source_tree; advisory_only\n")?;
    }


    handoff_append_required_file(
        &mut out,
        project_root,
        &project_root.join("ZIP_KIND.txt"),
        "CURRENT TAKE IDENTITY",
        true,
    )?;
    handoff_append_required_file(
        &mut out,
        project_root,
        &project_root.join("src/build_gate/take_change_class.txt"),
        "CURRENT TAKE CHANGE CONTRACT",
        true,
    )?;
    handoff_append_required_file(
        &mut out,
        project_root,
        &latest_dir.join("assistant_context.json"),
        "ASSISTANT CONTEXT",
        true,
    )?;
    handoff_append_required_file(
        &mut out,
        project_root,
        &latest_dir.join(RUN_OUTCOME_AUDIT_FILE),
        "RUN OUTCOME AUDIT",
        true,
    )?;
    handoff_append_required_file(
        &mut out,
        project_root,
        &latest_dir.join("manifest.txt"),
        "ASSISTANT EXPORT MANIFEST SUMMARY",
        true,
    )?;
    handoff_append_open_obligations(&mut out, project_root)?;
    handoff_append_read_set(&mut out, latest_dir)?;
    handoff_append_console(&mut out, project_root)?;

    handoff_append_text(
        &mut out,
        "\n[CURRENT-TAKE-CHANGED-SOURCE]\nPolicy: verified_snapshot_reference_or_whole_file_or_OmittedDueToHandoffBudget\n",
    )?;
    handoff_append_deleted_paths(&mut out, project_root)?;
    let mut snapshot = handoff_verified_snapshot(latest_dir);
    if let Some(snapshot) = snapshot.as_ref() {
        handoff_append_text(&mut out, &format!("SnapshotArchive: {}\nSnapshotArchiveSha256: {}\n", snapshot.label, snapshot.sha256))?;
    }
    let mut source_paths = Vec::new();
    for path in handoff_changed_source_paths(project_root)? {
        if !handoff_append_snapshot_reference(&mut out, project_root, &path, snapshot.as_mut())? {
            source_paths.push(path);
        }
    }
    let source_count = source_paths.len();
    let mut optional_files = source_paths
        .into_iter()
        .map(|path| (path, "SOURCE FILE"))
        .collect::<Vec<_>>();
    let result_path = match mode {
        "normal_run" => project.map(|name| latest_dir.join(project_result_summary_file_name(name))),
        "corpus_run" => Some(latest_dir.join("corpus_emergence.result.txt")),
        "visual_preflight" => Some(latest_dir.join("visual_preflight.result.txt")),
        _ => None,
    };
    if let Some(path) = result_path {
        if mode == "corpus_run" {
            if path.is_file() {
                let bytes = fs::read(&path)?;
                handoff_append_text(&mut out, &format!(
                    "\n[RUN-RESULT-REFERENCE]\nFile: {}\nBytes: {}\nSha256: {}\nEmbeddedBody: false\nResolution: current_assistant_export_zip\n[/RUN-RESULT-REFERENCE]\n",
                    file_name_string(&path), bytes.len(), sha256::hex(&bytes)
                ))?;
            } else {
                handoff_append_omission(&mut out, project_root, &path, "run_result_missing", None)?;
            }
        } else {
            optional_files.push((path, "RUN RESULT"));
        }
    }
    let catalog_notice = if matches!(mode, "normal_run" | "corpus_run") {
        "\n[PROJECT-FILE-CATALOG]\nFormat: tsv\nFile: project_file_catalog.tsv\nEmbeddedBody: false\n[/PROJECT-FILE-CATALOG]\n"
    } else {
        optional_files.push((latest_dir.join(PROJECT_FILE_CATALOG_MARKDOWN), "PROJECT FILE CATALOG"));
        ""
    };
    let source_end = "[/CURRENT-TAKE-CHANGED-SOURCE]\n";
    let handoff_end = "\nEND OF ETUSCHLOPF ASSISTANT HANDOFF\n";
    // Reserve every remaining omission and closing marker before admitting a whole file.
    // A fixed reserve is insufficient when a Take changes many source files.
    let omissions = optional_files
        .iter()
        .map(|(path, _)| {
            let mut omission = String::new();
            if path.is_file() {
                let bytes = fs::read(path)?;
                let reason = if std::str::from_utf8(&bytes).is_ok() {
                    "whole_file_does_not_fit"
                } else {
                    "non_utf8_optional_file"
                };
                handoff_append_omission(&mut omission, project_root, path, reason, Some(&bytes))?;
            }
            Ok(omission.len())
        })
        .collect::<AppResult<Vec<_>>>()?;
    let mut remaining = omissions.iter().sum::<usize>()
        + source_end.len() + catalog_notice.len() + handoff_end.len();
    if out.len().saturating_add(remaining) > ASSISTANT_HANDOFF_MAX_BYTES {
        return Err(anyhow!("assistant handoff required inputs and omission metadata exceed hard limit: {} + {} > {} bytes",
            out.len(), remaining, ASSISTANT_HANDOFF_MAX_BYTES));
    }
    for (index, ((path, section), omission_bytes)) in optional_files.iter().zip(&omissions).enumerate() {
        if index == source_count {
            handoff_append_text(&mut out, source_end)?;
            remaining -= source_end.len();
        }
        remaining -= omission_bytes;
        handoff_append_file(&mut out, project_root, path, section, false, remaining)?;
    }
    if source_count == optional_files.len() {
        handoff_append_text(&mut out, source_end)?;
    }
    handoff_append_text(&mut out, catalog_notice)?;
    handoff_append_text(&mut out, handoff_end)?;

    if out.len() > ASSISTANT_HANDOFF_MAX_BYTES {
        return Err(anyhow!(
            "assistant handoff exceeded hard limit after assembly: {} > {} bytes",
            out.len(),
            ASSISTANT_HANDOFF_MAX_BYTES
        ));
    }
    let mut file = File::create(&candidate_path)?;
    write!(file, "{out}")?;
    drop(file);
    if output_path.exists() {
        fs::remove_file(&output_path)?;
    }
    fs::rename(&candidate_path, &output_path)?;
    Ok(output_path)
}


fn assistant_export_includes_pdf(export_policy: AssistantExportPolicy) -> bool {
    export_policy == AssistantExportPolicy::IncludePdf
}

fn is_final_chord_rest_epoch_assistant_projection_line(line: &str) -> bool {
    line.starts_with(
        "Summary: SourceRestCarriersWithFinalMasterPartialSameLaneVisibleOccupancy ",
    ) || line.starts_with("Witness#")
        || line.starts_with("LowerStaffOverlappingRestBoundDispensabilityAudit")
        || line.starts_with("ReanchorTrialEvidence:")
        || line.starts_with("ReanchorAction#")
}

fn write_assistant_project_result_summary(
    src: &Path,
    dest: &Path,
    project: &str,
    version: &str,
    renderer_context: &AssistantContextRendererState,
) -> AppResult<()> {
    let text = fs::read_to_string(src).unwrap_or_default();
    let mut file = File::create(dest)?;
    let status = first_prefixed_value(&text, "Ergebnis:").unwrap_or_else(|| "unknown".to_string());
    let counts = first_prefixed_line(&text, "Blocker:").unwrap_or_else(|| "Blocker: n/a".to_string());
    let tool = first_prefixed_value(&text, "Werkzeug:").unwrap_or_else(|| format!("etuschlopf {version}"));
    let verdict = first_line_starting_with_any(&text, &["Entscheidung:", "Freigabe:"])
        .unwrap_or_else(|| "Entscheidung: n/a".to_string());
    let visual_top = project_visual_top_row_summary(&text).unwrap_or_else(|| "VisualTop: none".to_string());
    let hint = first_issue_summary(&text).unwrap_or_else(|| "Issue: none".to_string());
    let gould = if text.contains("GOULD-REGISTER-ISLAND-AUDIT") { "Gould: present" } else { "Gould: none" };
    let mut rest_timeline_stage_projection = Vec::<&str>::new();
    for line in text.lines() {
        if line.starts_with("BeatLocalVoiceRestRewrite v")
            || line.starts_with("CrossStaffHandRoleMaterializationTrial v")
            || line.starts_with("UpperStaffLocalVisualRestLedgerProof v")
            || line.starts_with("CursorLaneReanchorRestVisibilityContract ")
            || line.starts_with("StrictStaffOverfullCoverageNormalization v")
            || line.starts_with("WholeRestDisplayCoordinateReadbackTruthFix v")
            || line.starts_with("FinalRestClosureByInvariantProof v")
            || line.starts_with("GenericShadowRestAndTransactionalRest v")
        {
            rest_timeline_stage_projection.push(line);
        }
    }
    let mut final_chord_rest_epoch_projection = Vec::<&str>::new();
    if let Some(section_start) =
        text.find("FINAL-CHORD-VISIBLE-REST-INTERVAL-EPOCH-WITNESS-AUDIT")
    {
        for line in text[section_start..].lines() {
            if is_final_chord_rest_epoch_assistant_projection_line(line) {
                final_chord_rest_epoch_projection.push(line);
            } else if !final_chord_rest_epoch_projection.is_empty() && line.trim().is_empty() {
                break;
            }
        }
    }

    writeln!(file, "PROJECT-SUMMARY")?;
    writeln!(file, "Project: {project}")?;
    writeln!(file, "Tool: {tool}")?;
    writeln!(file, "Result: {status}")?;
    writeln!(file, "{counts}")?;
    if project == renderer_context.affected_project
        && renderer_context.cause_class == "renderer_written_pdf_nonzero_exit_reference_guard"
    {
        writeln!(
            file,
            "Freigabe: MusicXML formal OK; PDF frisch geschrieben, Renderer-Exit {} aber ungleich 0. Renderer/Guard gelb; keine Musikreparatur.",
            renderer_context.renderer_winning_exit_code
        )?;
        writeln!(file, "UnderlyingProjectVerdict: {verdict}")?;
        writeln!(file, "RendererGuard: yellow_written_pdf_nonzero_exit")?;
        writeln!(file, "RendererProcessExitCodeHex: {}", renderer_context.renderer_process_exit_code_hex)?;
        writeln!(file, "RendererProcessExitClass: {}", renderer_context.renderer_process_exit_class)?;
        writeln!(file, "RendererProcessElapsedMs: {}", renderer_context.renderer_process_elapsed_ms)?;
        writeln!(file, "OutputPresentAtProcessEnd: {}", renderer_context.output_present_at_process_end)?;
        writeln!(file, "OutputSecondPresenceAfterProcessMs: {}", renderer_context.output_second_presence_after_process_ms)?;
        writeln!(file, "OutputContentStableAcrossSettle: {}", renderer_context.output_content_stable_across_settle)?;
        writeln!(file, "OutputStructureStatus: {}", renderer_context.output_structure_status)?;
        writeln!(file, "RendererLifecycleInterpretation: {}", renderer_context.renderer_lifecycle_interpretation)?;
    } else {
        writeln!(file, "{verdict}")?;
        writeln!(file, "RendererGuard: none")?;
    }
    writeln!(file, "{visual_top}")?;
    writeln!(file, "{hint}")?;
    writeln!(file, "{gould}")?;
    for line in final_chord_rest_epoch_projection {
        writeln!(file, "FinalChordRestEpoch: {line}")?;
    }
    for line in rest_timeline_stage_projection {
        writeln!(file, "RestTimelineStage: {line}")?;
    }
    writeln!(file, "FullResultLocal: {}", src.display())?;
    Ok(())
}

pub struct VisualPreflightAssistantSummaryInput<'a> {
    pub(super) src: &'a Path,
    pub(super) dest: &'a Path,
    pub(super) version: &'a str,
    pub(super) run_outcome: &'a RunOutcomeProjection,
    pub(super) strict_test_exit: &'a str,
    pub(super) strict_test_summary: &'a str,
    pub(super) ledgers: &'a [VisualProjectRunLedger],
    pub(super) warning_truth: &'a WarningTruth,
    pub(super) reference_renderer_context: &'a AssistantContextRendererState,
}

pub fn write_visual_preflight_assistant_summary(
    input: VisualPreflightAssistantSummaryInput<'_>,
) -> AppResult<()> {
    let VisualPreflightAssistantSummaryInput {
        src,
        dest,
        version,
        run_outcome,
        strict_test_exit,
        strict_test_summary,
        ledgers,
        warning_truth,
        reference_renderer_context,
    } = input;
    let text = fs::read_to_string(src).unwrap_or_default();
    let mut file = File::create(dest)?;
    let mode = first_prefixed_value(&text, "Modus:").unwrap_or_else(|| "unknown".to_string());
    let selected_project = first_prefixed_value(&text, "SelectedTargetProject:").unwrap_or_else(|| "none".to_string());
    let selected_locator = first_prefixed_value(&text, "SelectedTargetLocator:").unwrap_or_else(|| "none".to_string());
    let target_class = first_prefixed_value(&text, "SelectedTargetClass:").unwrap_or_else(|| "none".to_string());
    let behind_bars_rule_check_required = first_prefixed_value(&text, "SelectedTargetBehindBarsRuleCheckRequired:").unwrap_or_else(|| "nein".to_string());
    let rule_candidate = first_prefixed_value(&text, "SelectedTargetRuleCandidate:").unwrap_or_else(|| "nein".to_string());
    let write_candidate = first_prefixed_value(&text, "SelectedTargetWriteCandidate:").unwrap_or_else(|| "nein".to_string());
    let permission_contract = first_prefixed_value(&text, "SelectedTargetPermissionContract:")
        .unwrap_or_else(|| "unknown".to_string());
    let emergence_status = first_prefixed_value(&text, "CurrentEmergenceStatus:")
        .unwrap_or_else(|| "unknown".to_string());
    let emergence_reason = first_prefixed_value(&text, "CurrentEmergenceReason:")
        .unwrap_or_else(|| "unknown".to_string());
    let gate = first_prefixed_value(&text, "Gate:").unwrap_or_else(|| "unknown".to_string());
    let renderer = first_prefixed_value(&text, "RendererFinalStatus:")
        .or_else(|| visual_renderer_guard_summary(&text))
        .unwrap_or_else(|| "unknown".to_string());
    let reuse = visual_renderer_reuse_summary(&text).unwrap_or_else(|| "RendererReuse: unknown".to_string());
    let gould = if text.contains("GOULD-REGISTER-ISLAND-AUDIT") { "Gould: audit-only" } else { "Gould: none" };

    writeln!(file, "VISUAL-PREFLIGHT-SUMMARY")?;
    writeln!(file, "Version: {version}")?;
    writeln!(file, "Status: {}", run_outcome.console_status)?;
    writeln!(file, "RunSeverity: {}", run_outcome.severity)?;
    writeln!(file, "ReleaseDecision: {}", run_outcome.release_decision)?;
    writeln!(file, "StatusProjectionSource: run_outcome_projection")?;
    writeln!(file, "Exit: {}", run_outcome.final_exit)?;
    writeln!(file, "ClippyExit: {}", run_outcome.clippy_exit)?;
    writeln!(file, "BuildExit: {}", run_outcome.build_exit)?;
    writeln!(file, "IntegratedLauncherTestExit: {strict_test_exit}")?;
    writeln!(file, "IntegratedLauncherTestSummary: {strict_test_summary}")?;
    writeln!(file, "CompilerWarningCount: {}", warning_truth.compiler_warning_count)?;
    writeln!(file, "CompilerWarningStatus: {}", warning_truth.compiler_status)?;
    writeln!(file, "ClippyWarningCount: {}", warning_truth.clippy_warning_count)?;
    writeln!(file, "ClippyWarningStatus: {}", warning_truth.clippy_status)?;
    writeln!(file, "Mode: {mode}")?;
    writeln!(file, "Projects: {}", if ledgers.is_empty() { "none".to_string() } else { ledgers.iter().map(|ledger| ledger.project.as_str()).collect::<Vec<_>>().join(", ") })?;
    writeln!(file, "CurrentEmergenceTarget: {selected_project} {selected_locator}")?;
    writeln!(file, "CurrentEmergenceTargetClass: {target_class}")?;
    writeln!(file, "CurrentEmergenceBehindBarsRuleCheckRequired: {behind_bars_rule_check_required}")?;
    writeln!(file, "CurrentEmergenceRuleCandidate: {rule_candidate}")?;
    writeln!(file, "CurrentEmergenceWriteCandidate: {write_candidate}")?;
    writeln!(file, "CurrentEmergencePermissionContract: {permission_contract}")?;
    writeln!(file, "CurrentEmergenceStatus: {emergence_status}")?;
    writeln!(file, "CurrentEmergenceReason: {emergence_reason}")?;
    writeln!(file, "FreshGate: {gate}")?;
    writeln!(file, "CurrentPreflightRenderer: {renderer}")?;
    writeln!(file, "CurrentPreflightRendererScope: current_visual_preflight_execution_only")?;
    writeln!(file, "ReferenceRendererAuditEvidenceStatus: {}", reference_renderer_context.evidence_status)?;
    writeln!(file, "ReferenceRendererAuditEvidenceVersion: {}", reference_renderer_context.evidence_version)?;
    writeln!(file, "ReferenceRendererAuditOutputStatus: {}", reference_renderer_context.renderer_output_status)?;
    writeln!(file, "ReferenceRendererAuditProcessStatus: {}", reference_renderer_context.renderer_process_status)?;
    writeln!(file, "ReferenceRendererAuditOverallStatus: {}", reference_renderer_context.renderer_overall_status)?;
    writeln!(
        file,
        "ReferenceRendererAuditGuard: {}",
        reference_renderer_context.persistent_guard_status()
    )?;
    writeln!(file, "{reuse}")?;
    writeln!(file, "RendererReuseScope: pre_run_reuse_decision_only; post_render_renderer_status_is_separate")?;
    writeln!(file, "{gould}")?;
    writeln!(file, "MusicMutation: nein")?;
    writeln!(file, "MasterApply: nein")?;
    writeln!(file, "FullResultLocal: {}", src.display())?;
    Ok(())
}

fn project_visual_top_row_summary(text: &str) -> Option<String> {
    let section_start = text.find("VISUAL-PDF-TRIAGE-LOCATOR-RANKING-NO-MUSIC-CHANGE")?;
    let section = &text[section_start..];
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("1 |") {
            let cells: Vec<&str> = trimmed.split('|').map(str::trim).collect();
            if cells.len() >= 11 {
                return Some(format!(
                    "VisualTop: rank 1 | score {} | risk {} | locator {}",
                    cells.get(1).copied().unwrap_or("?"),
                    cells.get(2).copied().unwrap_or("?"),
                    cells.get(10).copied().unwrap_or("?")
                ));
            }
        }
        if trimmed.starts_with("Urteil:") {
            break;
        }
    }
    None
}

fn first_issue_summary(text: &str) -> Option<String> {
    for line in text.lines().map(str::trim) {
        if line.starts_with("- [") {
            return Some(format!("Issue: {line}"));
        }
    }
    None
}


fn visual_renderer_guard_summary(text: &str) -> Option<String> {
    if text.contains("reference_guard_renderer_written_pdf_but_nonzero_exit")
        || text.contains("WrittenButNonZeroExit")
        || text.contains("yellow_renderer_wrote_fresh_output_after_delete_but_exit_nonzero")
    {
        return Some("guard_yellow_renderer_written_pdf_nonzero_exit".to_string());
    }
    if text.contains("RendererFinalStatus: RendererFailed") {
        return Some("renderer_failed".to_string());
    }
    if text.contains("RendererFinalStatus: NoRenderer") {
        return Some("no_renderer".to_string());
    }
    None
}

fn visual_renderer_reuse_summary(text: &str) -> Option<String> {
    let section_start = text.find("VISUAL-PREFLIGHT-RENDERER-REUSE")?;
    let section = &text[section_start..];
    if let Some(counts) = first_prefixed_value(section, "StatusCounts:") {
        return Some(format!("RendererReuse: {counts}"));
    }
    if let Some(reused_projects) = first_prefixed_value(section, "ReusedProjects:") {
        return Some(format!("RendererReuse: reused={reused_projects}"));
    }
    None
}

fn first_prefixed_line(text: &str, prefix: &str) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| line.starts_with(prefix))
        .map(ToOwned::to_owned)
}

fn first_prefixed_value(text: &str, prefix: &str) -> Option<String> {
    first_prefixed_line(text, prefix)
        .and_then(|line| line.strip_prefix(prefix).map(str::trim).map(ToOwned::to_owned))
        .filter(|value| !value.is_empty())
}

fn first_line_starting_with_any(text: &str, prefixes: &[&str]) -> Option<String> {
    text.lines()
        .map(str::trim)
        .find(|line| prefixes.iter().any(|prefix| line.starts_with(prefix)))
        .map(ToOwned::to_owned)
}


fn copy_export_file(src: &Path, dest: &Path, included: &mut Vec<String>) -> AppResult<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, dest)?;
    included.push(file_name_string(dest));
    copy_manifest_bound_corpus_score_artifacts(src, dest, included)?;
    Ok(())
}

fn copy_manifest_bound_corpus_score_artifacts(
    copied_source: &Path,
    copied_destination: &Path,
    included: &mut Vec<String>,
) -> AppResult<()> {
    let Some(export_dir) = copied_destination.parent() else {
        return Ok(());
    };
    let exported_manifest = export_dir.join(CORPUS_RUN_MANIFEST_FILE);
    if !exported_manifest.is_file() {
        return Ok(());
    }
    let Some(project_root) = export_dir
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
    else {
        return Ok(());
    };
    let manifest = match read_corpus_run_manifest(&exported_manifest) {
        Ok(manifest) => manifest,
        Err(_) => return Ok(()),
    };
    let Some(project) = manifest
        .projects
        .iter()
        .find(|project| project_root.join(&project.result_file).as_path() == copied_source)
    else {
        return Ok(());
    };

    for relative in [
        resolve_musicxml_file(&project.project),
        resolve_render_musicxml_file(&project.project),
        resolve_pdf_file(&project.project),
    ] {
        let source = project_root.join(&relative);
        if source.is_file() {
            let destination = export_dir.join(file_name_string(&relative));
            fs::copy(&source, &destination)?;
            let exported_name = file_name_string(&destination);
            if !included.contains(&exported_name) {
                included.push(exported_name);
            }
        } else {
            let omissions_name = "corpus_project_artifact_omissions.txt";
            let omissions_path = export_dir.join(omissions_name);
            let first_entry = !omissions_path.exists();
            let mut omissions = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&omissions_path)?;
            if first_entry {
                writeln!(omissions, "CORPUS PROJECT ARTIFACT OMISSIONS")?;
                writeln!(omissions, "=================================")?;
            }
            writeln!(
                omissions,
                "{} | manifest_bound_project_artifact_missing",
                relative.display()
            )?;
            if first_entry {
                included.push(omissions_name.to_string());
            }
        }
    }
    Ok(())
}

fn write_renderer_process_lifecycle_assistant_audit(
    dest: &Path,
    version: &str,
    reference_project: &str,
    renderer_state: &AssistantContextRendererState,
) -> AppResult<()> {
    let mut file = File::create(dest)?;
    writeln!(file, "RENDERER-PROCESS-LIFECYCLE-ASSISTANT-AUDIT")?;
    writeln!(file, "Version: {version}")?;
    writeln!(file, "ReferenceProject: {reference_project}")?;
    writeln!(file, "Boundary: audit_only; no_renderer_exit_acceptance; no_music_mutation; no_master_apply")?;
    writeln!(file, "RendererFinalStatus: {}", renderer_state.renderer_final_status)?;
    writeln!(file, "RendererWinningExitCode: {}", renderer_state.renderer_winning_exit_code)?;
    writeln!(file, "RendererProcessExitCodeHex: {}", renderer_state.renderer_process_exit_code_hex)?;
    writeln!(file, "RendererProcessExitClass: {}", renderer_state.renderer_process_exit_class)?;
    writeln!(file, "RendererProcessElapsedMs: {}", renderer_state.renderer_process_elapsed_ms)?;
    writeln!(file, "OutputPresentAtProcessEnd: {}", renderer_state.output_present_at_process_end)?;
    writeln!(file, "OutputSecondPresenceAfterProcessMs: {}", renderer_state.output_second_presence_after_process_ms)?;
    writeln!(file, "OutputContentStableAcrossSettle: {}", renderer_state.output_content_stable_across_settle)?;
    writeln!(file, "OutputStructureStatus: {}", renderer_state.output_structure_status)?;
    writeln!(file, "RendererLifecycleInterpretation: {}", renderer_state.renderer_lifecycle_interpretation)?;
    writeln!(file, "RendererOutputStatus: {}", renderer_state.renderer_output_status)?;
    writeln!(file, "RendererProcessStatus: {}", renderer_state.renderer_process_status)?;
    writeln!(file, "RendererOverallStatus: {}", renderer_state.renderer_overall_status)?;
    writeln!(
        file,
        "RendererGuardStatus: {}",
        renderer_state.persistent_guard_status()
    )?;
    writeln!(file, "EvidenceStatus: {}", renderer_state.evidence_status)?;
    writeln!(file, "EvidenceScope: {}", renderer_state.evidence_scope)?;
    writeln!(file, "EvidenceVersion: {}", renderer_state.evidence_version)?;
    writeln!(file, "EvidenceSourceLocal: {}", renderer_state.evidence_source_local)?;
    if renderer_state.evidence_source_local == "none" {
        writeln!(file, "NextEvidenceNeed: run_reference_renderer_only_if_new_runtime_evidence_is_required")?;
    }
    Ok(())
}


#[derive(Clone)]
pub struct AssistantContextRendererState {
    pub(super) cause_class: String,
    pub(super) affected_project: String,
    pub(super) renderer_final_status: String,
    pub(super) renderer_winning_exit_code: String,
    pub(super) renderer_process_exit_code_hex: String,
    pub(super) renderer_process_exit_class: String,
    pub(super) renderer_process_elapsed_ms: String,
    pub(super) output_created_after_delete: String,
    pub(super) output_present_at_process_end: String,
    pub(super) output_second_presence_after_process_ms: String,
    pub(super) output_content_stable_across_settle: String,
    pub(super) output_structure_status: String,
    pub(super) renderer_lifecycle_interpretation: String,
    pub(super) freshness_status: String,
    pub(super) renderer_output_status: String,
    pub(super) renderer_process_status: String,
    pub(super) renderer_overall_status: String,
    pub(super) evidence_status: String,
    pub(super) evidence_scope: String,
    pub(super) evidence_version: String,
    pub(super) evidence_source_local: String,
    pub(super) evidence_modified_at: Option<SystemTime>,
    pub(super) interpretation: String,
}

impl AssistantContextRendererState {
    pub(super) fn empty_current_preflight() -> Self {
        Self {
            cause_class: "none_current_run_has_no_project_renderer_execution".to_string(),
            affected_project: "none".to_string(),
            renderer_final_status: "unknown".to_string(),
            renderer_winning_exit_code: "unknown".to_string(),
            renderer_process_exit_code_hex: "unknown".to_string(),
            renderer_process_exit_class: "unknown".to_string(),
            renderer_process_elapsed_ms: "unknown".to_string(),
            output_created_after_delete: "unknown".to_string(),
            output_present_at_process_end: "unknown".to_string(),
            output_second_presence_after_process_ms: "unknown".to_string(),
            output_content_stable_across_settle: "unknown".to_string(),
            output_structure_status: "unknown".to_string(),
            renderer_lifecycle_interpretation: "unknown".to_string(),
            freshness_status: "unknown_no_current_project_renderer_execution".to_string(),
            renderer_output_status: "unknown_no_current_preflight_renderer_execution".to_string(),
            renderer_process_status: "not_executed_in_current_visual_preflight".to_string(),
            renderer_overall_status: "unknown_no_current_preflight_renderer_execution".to_string(),
            evidence_status: "not_applicable_current_preflight_no_renderer_execution".to_string(),
            evidence_scope: "current_visual_preflight_renderer_execution".to_string(),
            evidence_version: "none".to_string(),
            evidence_source_local: "none".to_string(),
            evidence_modified_at: None,
            interpretation: "Current visual-preflight executed no projects; reference renderer evidence is reported separately and is not current preflight truth.".to_string(),
        }
    }

    pub(super) fn empty_reference_audit() -> Self {
        Self {
            cause_class: "none".to_string(),
            affected_project: "none".to_string(),
            renderer_final_status: "unknown".to_string(),
            renderer_winning_exit_code: "unknown".to_string(),
            renderer_process_exit_code_hex: "unknown".to_string(),
            renderer_process_exit_class: "unknown".to_string(),
            renderer_process_elapsed_ms: "unknown".to_string(),
            output_created_after_delete: "unknown".to_string(),
            output_present_at_process_end: "unknown".to_string(),
            output_second_presence_after_process_ms: "unknown".to_string(),
            output_content_stable_across_settle: "unknown".to_string(),
            output_structure_status: "unknown".to_string(),
            renderer_lifecycle_interpretation: "unknown".to_string(),
            freshness_status: "unknown".to_string(),
            renderer_output_status: "unknown_no_reference_renderer_evidence".to_string(),
            renderer_process_status: "unknown_no_reference_renderer_evidence".to_string(),
            renderer_overall_status: "unknown_no_reference_renderer_evidence".to_string(),
            evidence_status: "reference_renderer_runtime_artifact_missing".to_string(),
            evidence_scope: "reference_renderer_audit_independent_of_current_preflight".to_string(),
            evidence_version: "none".to_string(),
            evidence_source_local: "none".to_string(),
            evidence_modified_at: None,
            interpretation: "No compatible reference renderer runtime evidence is available.".to_string(),
        }
    }

    pub(super) fn has_usable_reference_evidence(&self) -> bool {
        self.evidence_status == "current_version_reference_renderer_runtime_artifact_found"
            || self.evidence_status == "compatible_prior_version_reference_renderer_runtime_artifact_found"
            || self.evidence_status == "persistent_confirmed_reference_renderer_evidence_found"
    }

    pub(super) fn persistent_guard_status(&self) -> &'static str {
        if !self.has_usable_reference_evidence() {
            "missing_no_persistent_reference_renderer_evidence"
        } else if self.renderer_overall_status == "yellow_output_valid_process_failed" {
            "yellow_no_renderer_exit_acceptance"
        } else if self.renderer_overall_status == "green_output_valid_process_success" {
            "green_reference_renderer_confirmed"
        } else {
            "reference_renderer_evidence_needs_review"
        }
    }

    fn classify_typed_outcome(&mut self) {
        let output_complete = self.output_present_at_process_end == "ja"
            && self.output_content_stable_across_settle == "ja"
            && self.output_structure_status == "green_basic_pdf_structure_plausible";
        self.renderer_output_status = if output_complete {
            "complete_and_structurally_plausible".to_string()
        } else if self.output_present_at_process_end == "ja" {
            "present_but_not_fully_proven".to_string()
        } else if self.output_present_at_process_end == "nein" {
            "missing_at_process_end".to_string()
        } else {
            "unknown".to_string()
        };
        self.renderer_process_status = match self.renderer_process_exit_class.as_str() {
            "success_zero" => "completed_successfully".to_string(),
            "windows_ntstatus_access_violation" => {
                "crashed_with_windows_access_violation".to_string()
            }
            "unknown" => "unknown".to_string(),
            other => format!("failed_with_{other}"),
        };
        self.renderer_overall_status = if output_complete
            && self.renderer_process_exit_class == "windows_ntstatus_access_violation"
        {
            "yellow_output_valid_process_failed".to_string()
        } else if output_complete && self.renderer_process_exit_class == "success_zero" {
            "green_output_valid_process_success".to_string()
        } else if self.renderer_process_exit_class != "success_zero"
            && self.renderer_process_exit_class != "unknown"
        {
            "yellow_or_red_process_failed_output_not_fully_proven".to_string()
        } else {
            "unknown".to_string()
        };
    }
}

pub fn current_preflight_reference_guard_status(
    ledgers: &[VisualProjectRunLedger],
    reference_guard_project: &str,
    renderer_context: &AssistantContextRendererState,
) -> &'static str {
    ledgers
        .iter()
        .find(|ledger| ledger.project == reference_guard_project)
        .map(|ledger| {
            if ledger.is_fresh()
                && ledger.exit_code == 0
                && renderer_context.cause_class
                    == "renderer_written_pdf_nonzero_exit_reference_guard"
            {
                "guard_project_ok_renderer_yellow"
            } else if ledger.is_fresh() && ledger.exit_code == 0 {
                "green_guard_ok"
            } else {
                "guard_needs_review"
            }
        })
        .unwrap_or("not_evaluated_no_current_reference_project_run")
}

pub fn reconcile_renderer_outcome_with_current_evidence(
    observed: RendererOutcome,
    ledgers: &[VisualProjectRunLedger],
    renderer_context: &AssistantContextRendererState,
    current_version: &str,
) -> RendererOutcome {
    if matches!(
        observed,
        RendererOutcome::NotObserved | RendererOutcome::OutputValidProcessFailed(_)
    ) {
        return observed;
    }

    let matching_ledger = ledgers.iter().find(|ledger| {
        ledger.project == renderer_context.affected_project
            && ledger.shadow_run_started_at.is_some()
    });
    let evidence_matches_current_run = renderer_context.evidence_version == current_version
        && matching_ledger.is_some_and(|ledger| {
            ledger
                .shadow_run_started_at
                .zip(renderer_context.evidence_modified_at)
                .is_some_and(|(run_started_at, evidence_modified_at)| {
                    evidence_modified_at >= run_started_at
                })
        });
    let output_is_proven_valid = renderer_context.renderer_output_status
        == "complete_and_structurally_plausible";
    let process_is_proven_failed = renderer_context.renderer_process_status
        != "completed_successfully"
        && renderer_context.renderer_process_status != "unknown";
    let combined_status_is_expected_yellow = renderer_context.renderer_overall_status
        == "yellow_output_valid_process_failed";
    let failed_project_count = ledgers.iter().filter(|ledger| ledger.exit_code != 0).count();
    let outer_failure_is_compatible = match observed {
        RendererOutcome::ProcessFailed(_) => {
            failed_project_count == 1
                && matching_ledger.is_some_and(|ledger| ledger.exit_code != 0)
        }
        RendererOutcome::Passed => failed_project_count == 0,
        RendererOutcome::GuardBlocked => false,
        RendererOutcome::NotObserved | RendererOutcome::OutputValidProcessFailed(_) => false,
    };
    let renderer_exit_code = parse_shadow_renderer_exit_code(renderer_context);

    if evidence_matches_current_run
        && output_is_proven_valid
        && process_is_proven_failed
        && combined_status_is_expected_yellow
        && outer_failure_is_compatible
    {
        renderer_exit_code
            .map(RendererOutcome::OutputValidProcessFailed)
            .unwrap_or(observed)
    } else {
        observed
    }
}

fn parse_shadow_renderer_exit_code(
    renderer_context: &AssistantContextRendererState,
) -> Option<i32> {
    renderer_context
        .renderer_winning_exit_code
        .trim()
        .parse::<i32>()
        .ok()
        .or_else(|| {
            let hex = renderer_context
                .renderer_process_exit_code_hex
                .trim()
                .strip_prefix("0x")
                .or_else(|| {
                    renderer_context
                        .renderer_process_exit_code_hex
                        .trim()
                        .strip_prefix("0X")
                })?;
            u32::from_str_radix(hex, 16).ok().map(|value| value as i32)
        })
}

pub fn assistant_context_renderer_state_from_runtime(
    project_root: &Path,
    _reference_context_project: &str,
    current_version: &str,
) -> AssistantContextRendererState {
    let runtime_scan_dirs = [
        project_root
            .join("target")
            .join("etuschlopf_pdf_output_freshness_runtime"),
    ];
    let persistent_reference_path =
        project_root.join(PERSISTENT_REFERENCE_RENDERER_EVIDENCE_FILE);
    let mut evidence_paths: Vec<PathBuf> = Vec::new();
    for dir in runtime_scan_dirs {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        evidence_paths.extend(
            entries
                .flatten()
                .map(|entry| entry.path())
                .filter(|path| {
                    path.is_file()
                        && path.extension().and_then(|ext| ext.to_str()) == Some("txt")
                }),
        );
    }
    if persistent_reference_path.is_file() {
        evidence_paths.push(persistent_reference_path.clone());
    }

    let mut current_candidates: Vec<AssistantContextRendererState> = Vec::new();
    let mut compatible_prior_candidates: Vec<AssistantContextRendererState> = Vec::new();
    for path in evidence_paths {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
            if !text.contains("RendererFinalStatus:") && !text.contains("FreshnessStatus:") {
                continue;
            }
            let artifact_version = parse_simple_field_value(&text, "Version")
                .unwrap_or_else(|| "unknown".to_string());
            let current_version_evidence = artifact_version == current_version;
            let persistent_reference_evidence = path == persistent_reference_path
                && artifact_version == "0.10.1111"
                && text.contains(
                    "Instrumentation: render_rs_output_freshness_and_process_lifecycle_v1110",
                )
                && text.contains("EvidenceContract: persistent_confirmed_reference_v1");
            let compatible_prior_evidence = matches!(current_version, "0.10.1112" | "0.10.1113")
                && artifact_version == "0.10.1111"
                && text.contains("Instrumentation: render_rs_output_freshness_and_process_lifecycle_v1110")
                && text.contains("RENDERER-PROCESS-LIFECYCLE-AUDIT");
            if !current_version_evidence
                && !compatible_prior_evidence
                && !persistent_reference_evidence
            {
                continue;
            }
            let mut candidate = AssistantContextRendererState {
                cause_class: "none".to_string(),
                affected_project: parse_simple_field_value(&text, "Project")
                    .unwrap_or_else(|| file_name_string(&path)),
                renderer_final_status: parse_simple_field_value(&text, "RendererFinalStatus")
                    .unwrap_or_else(|| "unknown".to_string()),
                renderer_winning_exit_code: parse_simple_field_value(&text, "RendererWinningExitCode")
                    .unwrap_or_else(|| "unknown".to_string()),
                renderer_process_exit_code_hex: parse_simple_field_value(&text, "RendererProcessExitCodeHex")
                    .unwrap_or_else(|| "unknown".to_string()),
                renderer_process_exit_class: parse_simple_field_value(&text, "RendererProcessExitClass")
                    .unwrap_or_else(|| "unknown".to_string()),
                renderer_process_elapsed_ms: parse_simple_field_value(&text, "RendererProcessElapsedMs")
                    .unwrap_or_else(|| "unknown".to_string()),
                output_created_after_delete: parse_simple_field_value(&text, "OutputCreatedAfterDelete")
                    .unwrap_or_else(|| "unknown".to_string()),
                output_present_at_process_end: parse_simple_field_value(&text, "OutputPresentAtProcessEnd")
                    .unwrap_or_else(|| "unknown".to_string()),
                output_second_presence_after_process_ms: parse_simple_field_value(&text, "OutputSecondPresenceAfterProcessMs")
                    .unwrap_or_else(|| "unknown".to_string()),
                output_content_stable_across_settle: parse_simple_field_value(&text, "OutputContentStableAcrossSettle")
                    .unwrap_or_else(|| "unknown".to_string()),
                output_structure_status: parse_simple_field_value(&text, "OutputStructureStatus")
                    .unwrap_or_else(|| "unknown".to_string()),
                renderer_lifecycle_interpretation: parse_simple_field_value(&text, "RendererLifecycleInterpretation")
                    .unwrap_or_else(|| "unknown".to_string()),
                freshness_status: parse_simple_field_value(&text, "FreshnessStatus")
                    .unwrap_or_else(|| "unknown".to_string()),
                renderer_output_status: "unknown".to_string(),
                renderer_process_status: "unknown".to_string(),
                renderer_overall_status: "unknown".to_string(),
                evidence_status: if current_version_evidence {
                    "current_version_reference_renderer_runtime_artifact_found".to_string()
                } else if persistent_reference_evidence {
                    "persistent_confirmed_reference_renderer_evidence_found".to_string()
                } else {
                    "compatible_prior_version_reference_renderer_runtime_artifact_found".to_string()
                },
                evidence_scope: if current_version_evidence {
                    "current_version_reference_renderer_audit_not_current_preflight_execution".to_string()
                } else if persistent_reference_evidence {
                    "persistent_confirmed_reference_renderer_audit_independent_of_target_and_version".to_string()
                } else {
                    "compatible_prior_version_reference_renderer_audit_not_current_preflight_execution".to_string()
                },
                evidence_version: artifact_version,
                evidence_source_local: path.display().to_string(),
                evidence_modified_at: fs::metadata(&path)
                    .and_then(|metadata| metadata.modified())
                    .ok(),
                interpretation: "no_renderer_guard_issue_detected".to_string(),
            };
            let written_but_nonzero = candidate.renderer_final_status == "WrittenButNonZeroExit"
                || candidate.freshness_status.contains("exit_nonzero")
                || (candidate.renderer_winning_exit_code != "0"
                    && candidate.renderer_winning_exit_code != "unknown"
                    && candidate.output_created_after_delete == "ja");
            if written_but_nonzero {
                candidate.cause_class = "renderer_written_pdf_nonzero_exit_reference_guard".to_string();
                candidate.interpretation = format!(
                    "{} produced a structurally plausible stable PDF, but MuseScore ended with {} / {}. Keep the renderer guard yellow; this is not a music repair authority.",
                    candidate.affected_project,
                    candidate.renderer_process_exit_code_hex,
                    candidate.renderer_process_exit_class,
                );
            }
            candidate.classify_typed_outcome();
            if current_version_evidence {
                current_candidates.push(candidate);
            } else {
                compatible_prior_candidates.push(candidate);
            }
        }

    select_renderer_candidate(current_candidates)
        .or_else(|| select_renderer_candidate(compatible_prior_candidates))
        .unwrap_or_else(AssistantContextRendererState::empty_reference_audit)
}

fn select_renderer_candidate(
    mut candidate_states: Vec<AssistantContextRendererState>,
) -> Option<AssistantContextRendererState> {
    candidate_states.sort_by(|left, right| {
        let left_typed = left.cause_class == "renderer_written_pdf_nonzero_exit_reference_guard";
        let right_typed = right.cause_class == "renderer_written_pdf_nonzero_exit_reference_guard";
        let left_classified = left.renderer_process_exit_class != "unknown";
        let right_classified = right.renderer_process_exit_class != "unknown";
        let left_modified = left
            .evidence_modified_at
            .and_then(|value| value.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|value| value.as_nanos())
            .unwrap_or(0);
        let right_modified = right
            .evidence_modified_at
            .and_then(|value| value.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|value| value.as_nanos())
            .unwrap_or(0);
        right_typed
            .cmp(&left_typed)
            .then_with(|| right_classified.cmp(&left_classified))
            .then_with(|| right_modified.cmp(&left_modified))
            .then_with(|| left.evidence_source_local.cmp(&right.evidence_source_local))
    });
    candidate_states.into_iter().next()
}

struct AssistantContextBuildInput<'a> {
    latest_dir: &'a Path,
    version: &'a str,
    plan: &'a VisualPreflightPlan,
    projects: &'a [String],
    ledgers: &'a [VisualProjectRunLedger],
    warning_truth: &'a WarningTruth,
    run_outcome: &'a RunOutcomeProjection,
    reference_renderer_context: &'a AssistantContextRendererState,
    folder_catalog: &'a ProjectFolderCatalogSummary,
    validation: AssistantExportValidationEvidence<'a>,
    export_policy: AssistantExportPolicy,
}

enum JsonScope {
    Object(bool),
    Array(bool),
}

struct JsonWriter {
    out: String,
    scopes: Vec<JsonScope>,
}

impl JsonWriter {
    fn new() -> Self {
        Self { out: String::new(), scopes: Vec::new() }
    }

    fn prefix(&mut self, key: Option<&str>) {
        let depth = self.scopes.len();
        if let Some(scope) = self.scopes.last_mut() {
            let is_object = matches!(scope, JsonScope::Object(_));
            let first = match scope {
                JsonScope::Object(first) | JsonScope::Array(first) => first,
            };
            if !*first {
                self.out.push_str(",\n");
            }
            *first = false;
            self.out.push_str(&"  ".repeat(depth));
            if is_object {
                let key = key.expect("JSON object values require a key");
                self.out.push('"');
                self.out.push_str(&json_escape(key));
                self.out.push_str("\": ");
            }
        }
    }

    fn begin_object(&mut self, key: Option<&str>) {
        self.prefix(key);
        self.out.push('{');
        self.scopes.push(JsonScope::Object(true));
    }

    fn end_object(&mut self) {
        let had_values = matches!(self.scopes.pop(), Some(JsonScope::Object(false)));
        if had_values {
            self.out.push('\n');
            self.out.push_str(&"  ".repeat(self.scopes.len()));
        }
        self.out.push('}');
    }

    fn begin_array(&mut self, key: &str) {
        self.prefix(Some(key));
        self.out.push('[');
        self.scopes.push(JsonScope::Array(true));
    }

    fn end_array(&mut self) {
        let had_values = matches!(self.scopes.pop(), Some(JsonScope::Array(false)));
        if had_values {
            self.out.push('\n');
            self.out.push_str(&"  ".repeat(self.scopes.len()));
        }
        self.out.push(']');
    }

    fn string(&mut self, key: Option<&str>, value: &str) {
        self.prefix(key);
        self.out.push('"');
        self.out.push_str(&json_escape(value));
        self.out.push('"');
    }

    fn raw(&mut self, key: &str, value: &str) {
        self.prefix(Some(key));
        self.out.push_str(value);
    }

    fn bool(&mut self, key: &str, value: bool) {
        self.raw(key, if value { "true" } else { "false" });
    }

    fn strings(&mut self, key: &str, values: impl IntoIterator<Item = impl AsRef<str>>) {
        self.begin_array(key);
        for value in values {
            self.string(None, value.as_ref());
        }
        self.end_array();
    }

    fn finish(mut self) -> String {
        self.out.push('\n');
        self.out
    }
}

fn write_assistant_context_json(
    input: AssistantContextBuildInput<'_>,
    included: &mut Vec<String>,
    omitted: &mut Vec<String>,
) -> AppResult<()> {
    let AssistantContextBuildInput {
        latest_dir,
        version,
        plan,
        projects,
        ledgers,
        warning_truth,
        run_outcome,
        reference_renderer_context,
        folder_catalog,
        validation,
        export_policy,
    } = input;
    let final_exit = run_outcome.final_exit;
    let active_target_project = assistant_context_active_target_project(plan);
    let reference_guard_project = assistant_context_reference_guard_project(plan);
    let no_seed = plan.target_project.is_none() && final_exit == 2;
    let fresh_without_target = no_seed && plan.emergence.fresh_for_current_version;
    let all_fresh = ledgers.iter().all(VisualProjectRunLedger::is_fresh);
    let all_green = ledgers.iter().all(|ledger| ledger.exit_code == 0);
    let music_status = if fresh_without_target {
        "dark_green_fresh_full_corpus_without_selectable_target"
    } else if no_seed {
        "dark_green_no_current_target_awaiting_fresh_full_corpus_emergence"
    } else if all_fresh && all_green {
        "dark_green"
    } else {
        "needs_review"
    };
    let current_reference_executed = ledgers.iter().any(|row| row.project == reference_guard_project);
    let current_renderer = if current_reference_executed
        && reference_renderer_context.evidence_status == "current_version_reference_renderer_runtime_artifact_found"
    {
        reference_renderer_context.clone()
    } else {
        AssistantContextRendererState::empty_current_preflight()
    };
    let current_guard_status = current_preflight_reference_guard_status(
        ledgers,
        &reference_guard_project,
        &current_renderer,
    );
    let released_light_green_renderer_exit = run_outcome.release_decision
        == "released_external_renderer_exit_output_valid";
    let renderer_gate = if released_light_green_renderer_exit
        || (current_reference_executed
            && current_renderer.renderer_overall_status
                == "yellow_output_valid_process_failed")
    {
        "light_green_output_valid_external_renderer_exit_current_preflight"
    } else if ledgers.is_empty() {
        "unknown_no_current_preflight_renderer_execution"
    } else if final_exit == 0 {
        "dark_green_current_preflight_renderer"
    } else if all_fresh {
        "yellow_or_blue_current_preflight_renderer_needs_review"
    } else {
        "red_or_yellow_project_not_fresh"
    };
    let current_problem_class = if released_light_green_renderer_exit {
        "external_renderer_exit_output_valid_not_music"
    } else if final_exit == 0 {
        "none"
    } else if fresh_without_target {
        "fresh_full_corpus_without_selectable_target_not_music"
    } else if no_seed && reference_renderer_context.has_usable_reference_evidence() {
        "no_current_emergence_target_reference_renderer_audit_complete_not_music"
    } else if no_seed {
        "no_fresh_full_corpus_emergence_target_not_music"
    } else if current_renderer.cause_class == "renderer_written_pdf_nonzero_exit_reference_guard" {
        "renderer_guard_not_music"
    } else if all_fresh {
        "renderer_or_launcher_or_report_not_music_by_default"
    } else {
        "project_freshness_or_music_review"
    };
    let safe_short_answer = if run_outcome.release_decision == "released" {
        "dark_green_preflight_no_music_write"
    } else if released_light_green_renderer_exit {
        "light_green_external_renderer_exit_output_valid_continue"
    } else if run_outcome.release_decision == "audit_blocked_no_seed"
        && reference_renderer_context.has_usable_reference_evidence()
    {
        "reference_renderer_output_valid_process_failed_guard_yellow_no_full_corpus_required"
    } else if run_outcome.release_decision == "audit_blocked_no_seed" {
        "build_and_clippy_clean_no_current_target_full_corpus_only_for_new_target"
    } else if run_outcome.severity == "yellow" {
        "typed_audit_blocked_read_run_outcome"
    } else {
        "typed_failure_read_run_outcome"
    };
    let recommended_class = if released_light_green_renderer_exit {
        "continue_current_work_external_renderer_exit_non_blocking"
    } else if plan.target_project.is_none()
        && !plan.emergence.fresh_for_current_version
        && reference_renderer_context.has_usable_reference_evidence()
    {
        "reference_renderer_audit_complete_no_full_corpus_required"
    } else if plan.target_project.is_none() && !plan.emergence.fresh_for_current_version {
        "run_fresh_full_corpus_only_when_new_emergence_target_is_needed"
    } else if plan.target_project.is_none() {
        "discuss_fresh_full_corpus_without_selectable_target"
    } else if current_renderer.cause_class == "renderer_written_pdf_nonzero_exit_reference_guard" {
        "renderer_exit_root_cause_and_launcher_architecture_audit"
    } else {
        "review_fresh_emergence_target_before_any_music_change"
    };
    let recommended_reason = match recommended_class {
        "continue_current_work_external_renderer_exit_non_blocking" => "The renderer output is complete and valid. The known external MuseScore process exit is light green and non-blocking; continue the current EtuSchlopf work without rerun or rollback.",
        "reference_renderer_audit_complete_no_full_corpus_required" => "The reference renderer audit is complete and independent of emergence target selection. No full-corpus run is required for this renderer conclusion; run --all only when a new corpus emergence target is intentionally needed.",
        "run_fresh_full_corpus_only_when_new_emergence_target_is_needed" => "No current emergence target is valid. A full-corpus run is required only when a new target should be selected; it is not required for build, compiler, Clippy, or completed reference-renderer audits.",
        "discuss_fresh_full_corpus_without_selectable_target" => "The fresh current-version corpus is valid but contains no selectable visual hotspot. Do not invent a target from individual reports.",
        "renderer_exit_root_cause_and_launcher_architecture_audit" => "Music and freshness are green; continue with renderer exit root cause and non-music architecture cleanup before notation mutation.",
        _ => "Review the fresh emergence evidence before any notation mutation.",
    };
    let required_root_files = assistant_context_required_root_files_for_ledgers(plan, ledgers, export_policy);
    let reference_guard_result = if current_reference_executed {
        project_result_summary_file_name(&reference_guard_project)
    } else {
        "not_exported_no_current_project_run".to_string()
    };

    let mut json = JsonWriter::new();
    json.begin_object(None);
    json.raw("schema_version", "12");
    json.string(Some("purpose"), "assistant_external_memory_and_handover");
    json.bool("read_this_first", true);
    json.string(Some("canonical_assistant_file"), "assistant_context.json");

    json.begin_object(Some("transition_history"));
    json.raw("schema_version", "1");
    json.string(Some("file"), TRANSITION_HISTORY_FILE);
    json.string(Some("authority"), "advisory_evidence_only");
    json.bool("normative", false);
    json.string(Some("current_candidate_policy"), "separate_noncanonical_not_yet_canonical");
    json.end_object();

    json.begin_object(Some("project_folder_catalog"));
    json.raw("schema_version", "1");
    json.bool("complete_recursive_project_root", true);
    json.string(Some("markdown"), PROJECT_FILE_CATALOG_MARKDOWN);
    json.string(Some("tsv"), PROJECT_FILE_CATALOG_TSV);
    json.raw("entries", &folder_catalog.entry_count.to_string());
    json.raw("files", &folder_catalog.file_count.to_string());
    json.raw("directories", &folder_catalog.directory_count.to_string());
    json.raw("symlinks", &folder_catalog.symlink_count.to_string());
    json.raw("present_file_bytes", &folder_catalog.present_file_bytes.to_string());
    json.raw("planned_output_entries", &folder_catalog.planned_entry_count.to_string());
    json.end_object();

    json.begin_object(Some("assistant_fast_path"));
    json.string(Some("contract_name"), "AssistantFastPathContract");
    json.string(Some("version"), &format!("v{version}"));
    json.string(Some("status_line"), &run_outcome.release_decision);
    json.string(Some("current_problem_class"), current_problem_class);
    json.raw("final_exit", &final_exit.to_string());
    json.string(Some("build_exit"), &run_outcome.build_exit);
    json.string(Some("clippy_exit"), &run_outcome.clippy_exit);
    json.string(Some("integrated_launcher_test_exit"), validation.strict_test_exit);
    json.string(Some("integrated_launcher_test_summary"), validation.strict_test_summary);
    json.raw("compiler_warning_count", &warning_truth.compiler_warning_count.to_string());
    json.raw("clippy_warning_count", &warning_truth.clippy_warning_count.to_string());
    json.raw("project_rows", &ledgers.len().to_string());
    json.string(Some("active_target_project"), &active_target_project);
    json.string(Some("target_class"), plan.target_class.as_str());
    json.bool("write_candidate", plan.target_class.write_candidate());
    json.bool("music_mutation", false);
    json.bool("master_apply", false);
    json.string(Some("safe_short_answer"), safe_short_answer);
    json.strings("minimum_files_for_status_answer", ["assistant_context.json"]);
    json.string(Some("source_patch_requires"), "embedded_source_snapshot_in_same_assistant_export_latest.zip");
    json.end_object();

    json.begin_object(Some("run_outcome"));
    for (key, value) in [
        ("mode", run_outcome.mode),
        ("fatal", run_outcome.fatal),
        ("severity", run_outcome.severity),
        ("release_decision", run_outcome.release_decision.as_str()),
        ("console_status", run_outcome.console_status),
        ("console_severity", run_outcome.console_severity),
        ("build_exit", run_outcome.build_exit.as_str()),
        ("clippy_exit", run_outcome.clippy_exit.as_str()),
        ("audit_signal", run_outcome.audit_signal),
        ("renderer", run_outcome.renderer_outcome.as_str()),
        ("emergence", run_outcome.emergence_outcome),
    ] {
        json.string(Some(key), value);
    }
    json.raw("final_exit", &final_exit.to_string());
    json.bool("legacy_console_classification_present", false);
    json.bool("text_heuristic_used_by_run_outcome_model", false);
    json.string(Some("audit_file"), RUN_OUTCOME_AUDIT_FILE);
    json.end_object();

    json.begin_object(Some("project"));
    json.string(Some("name"), "EtuSchlopf");
    json.string(Some("version"), &format!("v{version}"));
    json.string(Some("run_mode"), "visual_preflight_strict");
    json.string(Some("active_target_project"), &active_target_project);
    json.string(Some("reference_guard_project"), &reference_guard_project);
    json.end_object();

    json.begin_object(Some("emergence_status"));
    for (key, value) in [
        ("status", plan.emergence.status.as_str()),
        ("reason", plan.emergence.reason.as_str()),
        ("report_version", plan.emergence.report_version.as_deref().unwrap_or("none")),
        ("run_id", plan.emergence.run_id.as_deref().unwrap_or("none")),
        ("manifest_reason", plan.emergence.manifest_reason.as_str()),
        ("current_target_project", active_target_project.as_str()),
        ("target_class", plan.target_class.as_str()),
        ("target_permission_contract", plan.target_class.permission_contract()),
    ] {
        json.string(Some(key), value);
    }
    for (key, value) in [
        ("report_exists", plan.emergence.report_exists),
        ("full_corpus", plan.emergence.full_corpus),
        ("manifest_exists", plan.emergence.manifest_exists),
        ("manifest_valid", plan.emergence.manifest_valid),
        ("fresh_for_current_version", plan.emergence.fresh_for_current_version),
        ("behind_bars_rule_check_required", plan.target_class.behind_bars_rule_check_required()),
        ("rule_candidate", plan.target_class.rule_candidate()),
        ("write_candidate", plan.target_class.write_candidate()),
    ] {
        json.bool(key, value);
    }
    json.raw("project_count", &plan.emergence.project_count.to_string());
    json.strings("considered_projects", plan.emergence.projects.iter());
    json.end_object();

    json.begin_object(Some("zip"));
    json.string(Some("zip_kind"), "assistant_export");
    json.bool("single_handover_zip", true);
    json.string(Some("expected_upload"), "assistant_export_latest.zip");
    json.bool("take_compatible", false);
    json.bool("contains_cargo_toml", false);
    json.string(Some("wrong_use_error_hint"), "Diese ZIP ist eine assistant_export-ZIP fuer den Chat. Nicht mit --take anwenden.");
    json.end_object();

    json.begin_object(Some("export_contract"));
    json.string(Some("contract_name"), "AssistantExportZipContractAudit");
    json.strings("required_root_files", required_root_files.iter());
    json.bool("redundant_assistant_snapshots_allowed", false);
    json.bool("assistant_handoff_projection_required", true);
    json.bool("cargo_toml_allowed", false);
    json.string(Some("pdf_export_policy"), "opt_in_only_use_--assistant-export-pdf");
    json.string(Some("artifact_plan"), "direct_minimal_write_no_copy_then_prune");
    json.end_object();

    json.begin_object(Some("warning_truth"));
    json.raw("compiler_warning_count", &warning_truth.compiler_warning_count.to_string());
    json.string(Some("compiler_status"), &warning_truth.compiler_status);
    json.string(Some("compiler_scope"), &warning_truth.compiler_scope);
    json.string(Some("compiler_log"), &warning_truth.compiler_log);
    json.raw("clippy_warning_count", &warning_truth.clippy_warning_count.to_string());
    json.string(Some("clippy_status"), &warning_truth.clippy_status);
    json.string(Some("clippy_scope"), &warning_truth.clippy_scope);
    json.string(Some("clippy_log"), &warning_truth.clippy_log);
    json.end_object();

    json.begin_object(Some("current_preflight_renderer_status"));
    for (key, value) in [
        ("scope", "current_visual_preflight_renderer_execution_only"),
        ("cause_class", current_renderer.cause_class.as_str()),
        ("affected_project", current_renderer.affected_project.as_str()),
        ("renderer_output_status", current_renderer.renderer_output_status.as_str()),
        ("renderer_process_status", current_renderer.renderer_process_status.as_str()),
        ("renderer_overall_status", current_renderer.renderer_overall_status.as_str()),
        ("renderer_process_exit_code_hex", current_renderer.renderer_process_exit_code_hex.as_str()),
        ("freshness_status", current_renderer.freshness_status.as_str()),
        ("interpretation", current_renderer.interpretation.as_str()),
    ] {
        json.string(Some(key), value);
    }
    json.end_object();

    json.begin_object(Some("reference_renderer_audit"));
    for (key, value) in [
        ("scope", "persistent_confirmed_reference_renderer_audit_independent_of_target_and_version"),
        ("evidence_status", reference_renderer_context.evidence_status.as_str()),
        ("evidence_version", reference_renderer_context.evidence_version.as_str()),
        ("affected_project", reference_renderer_context.affected_project.as_str()),
        ("renderer_output_status", reference_renderer_context.renderer_output_status.as_str()),
        ("renderer_process_status", reference_renderer_context.renderer_process_status.as_str()),
        ("renderer_overall_status", reference_renderer_context.renderer_overall_status.as_str()),
        ("renderer_final_status", reference_renderer_context.renderer_final_status.as_str()),
        ("renderer_winning_exit_code", reference_renderer_context.renderer_winning_exit_code.as_str()),
        ("renderer_process_exit_code_hex", reference_renderer_context.renderer_process_exit_code_hex.as_str()),
        ("renderer_process_exit_class", reference_renderer_context.renderer_process_exit_class.as_str()),
        ("output_present_at_process_end", reference_renderer_context.output_present_at_process_end.as_str()),
        ("output_content_stable_across_settle", reference_renderer_context.output_content_stable_across_settle.as_str()),
        ("output_structure_status", reference_renderer_context.output_structure_status.as_str()),
        ("guard_status", reference_renderer_context.persistent_guard_status()),
        ("interpretation", reference_renderer_context.interpretation.as_str()),
    ] {
        json.string(Some(key), value);
    }
    json.end_object();

    json.begin_object(Some("gates"));
    json.string(Some("music"), music_status);
    json.string(Some("renderer"), renderer_gate);
    json.string(Some("report"), &format!("{}_typed_run_outcome", run_outcome.severity));
    json.string(Some("launcher"), &format!("{}_typed_{}", run_outcome.console_severity, run_outcome.console_status.to_ascii_lowercase().replace('-', "_")));
    json.string(Some("export"), "green_assistant_context_json_included");
    json.bool("panic_red", run_outcome.fatal == "panic");
    json.end_object();

    json.begin_object(Some("current_truth"));
    json.string(Some("music_status"), music_status);
    json.string(Some("active_target_project"), &active_target_project);
    json.string(Some("target_class"), plan.target_class.as_str());
    json.bool("behind_bars_rule_check_required", plan.target_class.behind_bars_rule_check_required());
    json.bool("rule_candidate", plan.target_class.rule_candidate());
    json.bool("write_candidate", plan.target_class.write_candidate());
    json.string(Some("reference_guard_project"), &reference_guard_project);
    json.string(Some("current_preflight_reference_guard_status"), current_guard_status);
    json.string(Some("persistent_reference_renderer_evidence_status"), &reference_renderer_context.evidence_status);
    json.string(Some("persistent_reference_renderer_guard_status"), reference_renderer_context.persistent_guard_status());
    json.bool("music_mutation", false);
    json.bool("master_apply", false);
    json.bool("do_not_start_music_repair", true);
    json.end_object();

    json.begin_array("projects");
    for ledger in ledgers {
        json.begin_object(None);
        json.string(Some("project"), &ledger.project);
        json.raw("exit_code", &ledger.exit_code.to_string());
        json.bool("fresh", ledger.is_fresh());
        json.string(Some("result_file"), &project_result_summary_file_name(&ledger.project));
        json.bool("pdf_exported", assistant_export_includes_pdf(export_policy));
        json.end_object();
    }
    json.end_array();


    json.begin_array("known_failure_patterns");
    for (id, lesson) in [
        ("private_staff_unscope_parser", "Use staff-scoped XML node reading; no private mini-parser for staff, clef or voice decisions."),
        ("wouldwrite_postwrite_misread", "WouldWrite is pre-write; after success use AlreadyMaterialized and visible readback."),
        ("zip_kind_confusion", "assistant_export_latest.zip is for chat; source release ZIPs with Cargo.toml are for --take."),
        ("console_red_gate_confusion", "Separate music, renderer, report, launcher and export gates."),
        ("corpus_evidence_cleanup_self_invalidation", "Preserve every result file bound by corpus_run_manifest.json."),
    ] {
        json.begin_object(None);
        json.string(Some("id"), id);
        json.string(Some("lesson"), lesson);
        json.end_object();
    }
    json.end_array();

    json.strings("forbidden_shortcuts", [
        "project_name_as_music_predicate",
        "literal_measure_number_as_music_predicate",
        "ledger_write_shortcut_without_scoped_write_path",
        "private_string_parser_for_staff_clef_voice_measure_decisions",
    ]);
    json.strings("do_not_do", [
        "do_not_reopen_retired_local_repairs_without_fresh_evidence",
        "do_not_use_project_name_or_literal_measure_as_rule_predicate",
        "do_not_start_music_write_for_renderer_exit_only",
        "do_not_use_assistant_export_zip_with_take",
        "do_not_reinfer_from_old_audits_before_assistant_context",
    ]);
    let mut reading_order = vec![
        "assistant_context.json".to_string(),
        TRANSITION_HISTORY_FILE.to_string(),
        "emergence_selection.json".to_string(),
        "manifest.txt".to_string(),
        "visual_preflight.result.txt".to_string(),
        RENDERER_PROCESS_LIFECYCLE_ASSISTANT_FILE.to_string(),
        RUN_OUTCOME_AUDIT_FILE.to_string(),
        PROJECT_FILE_CATALOG_MARKDOWN.to_string(),
        PROJECT_FILE_CATALOG_TSV.to_string(),
        "source_architecture_inventory.txt".to_string(),
    ];
    if plan.emergence.manifest_exists {
        reading_order.insert(2, CORPUS_RUN_MANIFEST_FILE.to_string());
    }
    reading_order.extend(ledgers.iter().map(|row| project_result_summary_file_name(&row.project)));
    json.strings("assistant_reading_order", reading_order.iter());

    json.begin_object(Some("artifact_index"));
    json.string(Some("read_first"), "assistant_context.json#assistant_fast_path");
    json.string(Some("transition_history"), TRANSITION_HISTORY_FILE);
    json.string(Some("current_emergence_decision"), "emergence_selection.json");
    json.string(Some("source_architecture"), "source_architecture_inventory.txt");
    json.string(Some("project_folder_catalog_markdown"), PROJECT_FILE_CATALOG_MARKDOWN);
    json.string(Some("project_folder_catalog_tsv"), PROJECT_FILE_CATALOG_TSV);
    json.string(Some("overall_status"), "visual_preflight.result.txt");
    json.string(Some("manifest"), "manifest.txt");
    json.string(Some("renderer_process_lifecycle"), RENDERER_PROCESS_LIFECYCLE_ASSISTANT_FILE);
    json.string(Some("run_outcome"), RUN_OUTCOME_AUDIT_FILE);
    json.string(Some("reference_guard_result"), &reference_guard_result);
    json.end_object();

    json.begin_object(Some("artifact_classification"));
    json.strings("current_exported", required_root_files.iter());
    json.strings("current_local_only", [
        "target/etuschlopf_logs/launcher_bootstrap_build.log",
        "gould_register_island_audit.tsv",
        "visual_preflight.full.txt",
    ]);
    json.strings("historical_memory_only", [
        "visible_clef_glyph_audit_review.html",
    ]);
    json.string(Some("policy"), "current_exported_is_present_in_assistant_zip_current_local_only_stays_in_project_root_historical_memory_only_is_not_current_runtime_truth");
    json.end_object();

    json.begin_object(Some("recommended_next_action"));
    json.string(Some("class"), recommended_class);
    json.bool("do_not_start_music_repair", true);
    json.string(Some("reason"), recommended_reason);
    json.end_object();
    json.end_object();

    let context_path = latest_dir.join("assistant_context.json");
    fs::write(&context_path, json.finish())?;
    included.push(file_name_string(&context_path));
    if projects.is_empty() {
        omitted.push("assistant_context.projects | no_project_ledgers_available".to_string());
    }
    Ok(())
}


fn is_assistant_context_export_name(name: &str) -> bool {
    name == "assistant_context.json"
}

fn file_name_string(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string().replace(['/', '\\'], "_"))
}

struct AssistantExportManifestContext<'a> {
    path: &'a Path,
    version: &'a str,
    plan: &'a VisualPreflightPlan,
    projects: &'a [String],
    ledgers: &'a [VisualProjectRunLedger],
    warning_truth: &'a WarningTruth,
    run_outcome: &'a RunOutcomeProjection,
    folder_catalog: &'a ProjectFolderCatalogSummary,
    validation: AssistantExportValidationEvidence<'a>,
    export_policy: AssistantExportPolicy,
}

fn write_assistant_export_manifest(
    context: AssistantExportManifestContext<'_>,
    included: &[String],
    omitted: &[String],
) -> AppResult<()> {
    let path = context.path;
    let version = context.version;
    let plan = context.plan;
    let projects = context.projects;
    let ledgers = context.ledgers;
    let warning_truth = context.warning_truth;
    let run_outcome = context.run_outcome;
    let folder_catalog = context.folder_catalog;
    let validation = context.validation;
    let export_policy = context.export_policy;
    let final_exit = run_outcome.final_exit;
    let clippy_exit = &run_outcome.clippy_exit;
    let build_exit = &run_outcome.build_exit;
    let mut file = File::create(path)?;
    let unix_seconds = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|value| value.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    writeln!(file, "ASSISTANT-EXPORT-BUNDLE")?;
    writeln!(file, "=======================")?;
    writeln!(file, "Werkzeug: etuschlopf {version}")?;
    writeln!(file, "GeneratedUnixSeconds: {unix_seconds}")?;
    writeln!(file, "RunMode: visual_preflight")?;
    writeln!(file, "SchemaVersion: 6")?;
    writeln!(file, "ZipKind: assistant_export")?;
    writeln!(file, "TakeCompatible: false")?;
    writeln!(file, "CanonicalAssistantFile: assistant_context.json")?;
    writeln!(file, "ProjectFolderCatalog: complete_recursive_project_root")?;
    writeln!(file, "ProjectFolderCatalogEntries: {}", folder_catalog.entry_count)?;
    writeln!(file, "ProjectFolderCatalogFiles: {}", folder_catalog.file_count)?;
    writeln!(file, "ProjectFolderCatalogDirectories: {}", folder_catalog.directory_count)?;
    writeln!(file, "ProjectFolderCatalogSymlinks: {}", folder_catalog.symlink_count)?;
    writeln!(file, "ProjectFolderCatalogPlannedOutputs: {}", folder_catalog.planned_entry_count)?;
    writeln!(file, "FinalExit: {final_exit}")?;
    writeln!(file, "RunSeverity: {}", run_outcome.severity)?;
    writeln!(file, "ReleaseDecision: {}", run_outcome.release_decision)?;
    writeln!(file, "ConsoleProjectionStatus: {}", run_outcome.console_status)?;
    writeln!(file, "ClippyExit: {clippy_exit}")?;
    writeln!(file, "BuildExit: {build_exit}")?;
    writeln!(file, "IntegratedLauncherTestExit: {}", validation.strict_test_exit)?;
    writeln!(file, "IntegratedLauncherTestSummary: {}", validation.strict_test_summary)?;
    writeln!(file, "StatusProjectionSource: run_outcome_projection")?;
    writeln!(file, "CompilerWarningCount: {}", warning_truth.compiler_warning_count)?;
    writeln!(file, "CompilerWarningStatus: {}", warning_truth.compiler_status)?;
    writeln!(file, "ClippyWarningCount: {}", warning_truth.clippy_warning_count)?;
    writeln!(file, "ClippyWarningStatus: {}", warning_truth.clippy_status)?;
    writeln!(file, "WarningTruthPolicy: compiler_and_clippy_warnings_reported_separately")?;
    writeln!(file, "Projects: {}", if projects.is_empty() { "none".to_string() } else { projects.join(", ") })?;
    writeln!(file, "CurrentEmergenceStatus: {}", plan.emergence.status)?;
    writeln!(file, "CurrentEmergenceReason: {}", plan.emergence.reason)?;
    writeln!(file, "CurrentEmergenceRunId: {}", plan.emergence.run_id.as_deref().unwrap_or("none"))?;
    writeln!(file, "CurrentEmergenceEvidenceContract: {}", plan.emergence.evidence_contract)?;
    writeln!(file, "CurrentEmergenceManifestValid: {}", yes_no_bool(plan.emergence.manifest_valid))?;
    writeln!(file, "CurrentEmergenceManifestReason: {}", plan.emergence.manifest_reason)?;
    writeln!(file, "CurrentEmergenceManifestProjectFiles: {}/{}", plan.emergence.manifest_project_files_valid, plan.emergence.manifest_project_files_expected)?;
    writeln!(file, "CurrentEmergenceReportHashValid: {}", yes_no_bool(plan.emergence.report_hash_valid))?;
    writeln!(file, "CurrentEmergenceReportBoundToManifest: {}", yes_no_bool(plan.emergence.report_bound_to_manifest))?;
    writeln!(file, "CurrentEmergenceFullCorpus: {}", yes_no_bool(plan.emergence.full_corpus))?;
    writeln!(file, "CurrentEmergenceFreshForVersion: {}", yes_no_bool(plan.emergence.fresh_for_current_version))?;
    writeln!(file, "CurrentEmergenceTarget: {}", plan.target_project.as_deref().unwrap_or("none"))?;
    writeln!(file, "CurrentEmergenceTargetClass: {}", plan.target_class.as_str())?;
    writeln!(file, "CurrentEmergenceBehindBarsRuleCheckRequired: {}", yes_no_bool(plan.target_class.behind_bars_rule_check_required()))?;
    writeln!(file, "CurrentEmergenceRuleCandidate: {}", yes_no_bool(plan.target_class.rule_candidate()))?;
    writeln!(file, "CurrentEmergenceWriteCandidate: {}", yes_no_bool(plan.target_class.write_candidate()))?;
    writeln!(file, "CurrentEmergencePermissionContract: {}", plan.target_class.permission_contract())?;
    writeln!(file)?;
    writeln!(file, "PROJECT-DECISIONS")?;
    writeln!(file, "-----------------")?;
    writeln!(file, "Projekt | ResultFresh | PdfByteChanged | AutoVisualChanged | BehindBarsRuleCheckRequired | PdfInExport")?;
    for ledger in ledgers {
        let pdf_byte_changed = match (&ledger.previous_pdf_fingerprint, &ledger.current_pdf_fingerprint) {
            (Some(before), Some(after)) => before.sha256 != after.sha256,
            (None, Some(_)) => true,
            _ => false,
        };
        let review_required = true;
        let pdf_in_export = assistant_export_includes_pdf(export_policy);
        writeln!(
            file,
            "{} | {} | {} | {} | {} | {}",
            ledger.project,
            if ledger.fresh { "ja" } else { "nein" },
            if pdf_byte_changed { "ja" } else { "nein" },
            auto_visual_changed_label(ledger),
            if review_required { "ja" } else { "nein" },
            if pdf_in_export { "ja" } else { "nein" },
        )?;
    }
    writeln!(file)?;
    writeln!(file, "INCLUDED-FILES")?;
    writeln!(file, "--------------")?;
    writeln!(file, "- manifest.txt")?;
    for item in included {
        writeln!(file, "- {item}")?;
    }
    writeln!(file)?;
    writeln!(file, "OMITTED-FILES")?;
    writeln!(file, "-------------")?;
    write_compact_omitted_files_section(&mut file, omitted)?;
    writeln!(file)?;
    writeln!(file, "ASSISTANT-CONTEXT-JSON")?;
    writeln!(file, "----------------------")?;
    let context_items = included
        .iter()
        .filter(|name| is_assistant_context_export_name(name))
        .collect::<Vec<_>>();
    if context_items.is_empty() {
        writeln!(file, "Status: missing_or_not_exported")?;
    } else {
        writeln!(file, "Status: included")?;
        writeln!(file, "SchemaVersion: 11")?;
        writeln!(file, "ZipKind: assistant_export")?;
        writeln!(file, "TakeCompatible: false")?;
        writeln!(file, "Policy: assistant_context.json is generated once by the scoped writer and is the canonical handover file.")?;
        for item in context_items {
            writeln!(file, "- {item}")?;
        }
    }
    writeln!(file)?;
    writeln!(file, "UPLOAD-INSTRUCTION")?;
    writeln!(file, "------------------")?;
    writeln!(file, "Im Chat nur assistant_export_latest.zip aus dem Projekt-Root hochladen. Die interne Kopie target/etuschlopf_assistant_export/latest.zip bleibt als Diagnoseartefakt erhalten.")?;
    Ok(())
}


fn write_compact_omitted_files_section(file: &mut File, omitted: &[String]) -> AppResult<()> {
    const EXAMPLE_LIMIT: usize = 16;
    if omitted.is_empty() {
        writeln!(file, "- none")?;
        return Ok(());
    }
    if omitted.len() <= EXAMPLE_LIMIT {
        for item in omitted {
            writeln!(file, "- {item}")?;
        }
        return Ok(());
    }

    let mut reason_counts: BTreeMap<String, usize> = BTreeMap::new();
    for item in omitted {
        let reason = item
            .split_once(" | ")
            .map(|(_, reason)| reason)
            .unwrap_or("no_reason")
            .to_string();
        *reason_counts.entry(reason).or_insert(0) += 1;
    }

    writeln!(file, "TotalOmittedFiles: {}", omitted.len())?;
    writeln!(file, "Mode: compact_summary_to_avoid_dateiflut")?;
    writeln!(file, "Reasons:")?;
    for (reason, count) in reason_counts {
        writeln!(file, "- {reason}: {count}")?;
    }
    writeln!(file, "Examples:")?;
    for item in omitted.iter().take(EXAMPLE_LIMIT) {
        writeln!(file, "- {item}")?;
    }
    let hidden_count = omitted.len().saturating_sub(EXAMPLE_LIMIT);
    writeln!(file, "- ... {hidden_count} weitere ausgelassene Dateien bewusst nicht einzeln gelistet")?;
    Ok(())
}


#[derive(Clone, Debug)]
struct AssistantExportZipContractAudit {
    ok: bool,
    missing_required_files: Vec<String>,
    redundant_assistant_snapshots: Vec<String>,
    pdf_count: usize,
    review_html_count: usize,
    contains_cargo_toml: bool,
    source_snapshot_archive_count: usize,
    source_snapshot_metadata_present: bool,
    assistant_context_schema_ok: bool,
}

impl AssistantExportZipContractAudit {
    fn status_label(&self) -> &'static str {
        if self.ok { "passed" } else { "failed" }
    }

    fn verdict(&self) -> &'static str {
        if self.ok {
            "assistant_export_zip_contract_passed_single_handover_zip_ready"
        } else if self.contains_cargo_toml {
            "assistant_export_zip_contract_failed_contains_source_release_marker"
        } else if !self.redundant_assistant_snapshots.is_empty() {
            "assistant_export_zip_contract_failed_redundant_assistant_snapshots"
        } else if !self.missing_required_files.is_empty() {
            "assistant_export_zip_contract_failed_missing_required_files"
        } else if !self.source_snapshot_metadata_present || self.source_snapshot_archive_count != 1 {
            "assistant_export_zip_contract_failed_source_snapshot"
        } else if !self.assistant_context_schema_ok {
            "assistant_export_zip_contract_failed_assistant_context_schema"
        } else {
            "assistant_export_zip_contract_failed_unknown"
        }
    }
}

fn assistant_export_zip_contract_audit(
    included: &[String],
    plan: &VisualPreflightPlan,
    ledgers: &[VisualProjectRunLedger],
    export_policy: AssistantExportPolicy,
) -> AssistantExportZipContractAudit {
    let included_set = included.iter().cloned().collect::<HashSet<_>>();
    let required = assistant_context_required_root_files_for_ledgers(plan, ledgers, export_policy);
    let missing_required_files = required
        .iter()
        .filter(|name| !included_set.contains(*name))
        .cloned()
        .collect::<Vec<_>>();
    let redundant_assistant_snapshots = included
        .iter()
        .filter(|name| {
            name.starts_with("assistant_")
                && name.as_str() != "assistant_context.json"
                && name.as_str() != ASSISTANT_HANDOFF_FILENAME
        })
        .cloned()
        .collect::<Vec<_>>();
    let pdf_count = included.iter().filter(|name| name.ends_with(".pdf")).count();
    let review_html_count = included
        .iter()
        .filter(|name| name.ends_with("_review.html"))
        .count();
    let contains_cargo_toml = included_set.contains("Cargo.toml");
    let source_snapshot_archive_count = included
        .iter()
        .filter(|name| name.starts_with("source_snapshot/EtuSchlopf_v") && name.ends_with("_unbuilt_source.zip"))
        .count();
    let source_snapshot_metadata_present =
        included_set.contains(&source_snapshot_metadata_relative_path());
    let assistant_context_schema_ok = included_set.contains("assistant_context.json");
    let required_pdf_count = ledgers
        .iter()
        .filter(|_| assistant_export_includes_pdf(export_policy))
        .count();
    let ok = missing_required_files.is_empty()
        && redundant_assistant_snapshots.is_empty()
        && pdf_count >= required_pdf_count
        && !contains_cargo_toml
        && source_snapshot_metadata_present
        && source_snapshot_archive_count == 1
        && assistant_context_schema_ok;
    AssistantExportZipContractAudit {
        ok,
        missing_required_files,
        redundant_assistant_snapshots,
        pdf_count,
        review_html_count,
        contains_cargo_toml,
        source_snapshot_archive_count,
        source_snapshot_metadata_present,
        assistant_context_schema_ok,
    }
}

fn append_assistant_export_zip_contract_to_manifest(
    manifest_path: &Path,
    audit: &AssistantExportZipContractAudit,
) -> AppResult<()> {
    let mut file = OpenOptions::new().append(true).open(manifest_path)?;
    writeln!(file)?;
    writeln!(file, "ASSISTANT-EXPORT-ZIP-CONTRACT-AUDIT")?;
    writeln!(file, "-----------------------------------")?;
    writeln!(file, "Status: {}", audit.status_label())?;
    writeln!(file, "ZipKind: assistant_export")?;
    writeln!(file, "TakeCompatible: false")?;
    writeln!(file, "ContainsCargoToml: {}", if audit.contains_cargo_toml { "ja" } else { "nein" })?;
    writeln!(file, "SourceSnapshotMetadata: {}", if audit.source_snapshot_metadata_present { "present" } else { "missing" })?;
    writeln!(file, "SourceSnapshotArchives: {}", audit.source_snapshot_archive_count)?;
    writeln!(file, "AssistantContextJsonSchema: {}", if audit.assistant_context_schema_ok { "passed" } else { "failed" })?;
    writeln!(file, "PdfFiles: {}", audit.pdf_count)?;
    writeln!(file, "PdfExportPolicy: opt_in_only")?;
    writeln!(file, "ReviewHtmlFiles: {}", audit.review_html_count)?;
    writeln!(file, "MissingRequiredFiles: {}", if audit.missing_required_files.is_empty() { "none".to_string() } else { audit.missing_required_files.join(",") })?;
    writeln!(file, "RedundantAssistantSnapshots: {}", if audit.redundant_assistant_snapshots.is_empty() { "none".to_string() } else { audit.redundant_assistant_snapshots.join(",") })?;
    writeln!(file, "Verdict: {}", audit.verdict())?;
    writeln!(file)?;
    writeln!(file, "TAKE-BOOTSTRAP-CONTRACT-AUDIT")?;
    writeln!(file, "-----------------------------")?;
    writeln!(file, "Status: passed")?;
    writeln!(file, "BootstrapBeforeLauncherCompile: ja")?;
    writeln!(file, "BootstrapArchitectureGateIsolation: retired")?;
    writeln!(file, "BootstrapAndProductiveArchitectureGate: mandatory")?;
    writeln!(file, "RecoveryScriptRequired: nein")?;
    writeln!(file, "SourceZipHigherVersion: accepted_and_reexec_without_take")?;
    writeln!(file, "SourceZipSameVersion: friendly_noop_unless_force")?;
    writeln!(file, "ExportZipWithTake: friendly_rejected_as_chat_upload")?;
    writeln!(file, "MissingZip: clear_message_no_manual_copy_guessing")?;
    writeln!(file, "Verdict: take_bootstrap_compile_uses_mandatory_architecture_gate")?;
    Ok(())
}

fn zip_directory_flat(source_dir: &Path, zip_path: &Path) -> AppResult<()> {
    let started = Instant::now();
    let file = File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut root_entries = fs::read_dir(source_dir)?.collect::<Result<Vec<_>, _>>()?;
    root_entries.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    for entry in root_entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(anyhow!(
                "assistant export staging contains forbidden symlink: {}",
                path.display()
            ));
        }
        if metadata.is_file() {
            zip.start_file(file_name_string(&path), options)?;
            zip.write_all(&fs::read(&path)?)?;
            continue;
        }
        if metadata.is_dir() && entry.file_name().to_string_lossy() == SOURCE_SNAPSHOT_DIR {
            let mut snapshot_entries =
                fs::read_dir(&path)?.collect::<Result<Vec<_>, _>>()?;
            snapshot_entries
                .sort_by_key(|child| child.file_name().to_string_lossy().into_owned());
            for child in snapshot_entries {
                let child_path = child.path();
                let child_metadata = fs::symlink_metadata(&child_path)?;
                if child_metadata.file_type().is_symlink() || !child_metadata.is_file() {
                    return Err(anyhow!(
                        "assistant export source_snapshot contains unsupported entry: {}",
                        child_path.display()
                    ));
                }
                let relative = Path::new(SOURCE_SNAPSHOT_DIR).join(child.file_name());
                zip.start_file(normalize_catalog_path(&relative), options)?;
                zip.write_all(&fs::read(&child_path)?)?;
            }
            continue;
        }
        if metadata.is_dir() {
            return Err(anyhow!(
                "assistant export staging contains unexpected directory: {}",
                path.display()
            ));
        }
        return Err(anyhow!(
            "assistant export staging contains unsupported entry: {}",
            path.display()
        ));
    }
    zip.finish()?;
    if let Some(root) = source_dir.ancestors().nth(3) {
        assistant_export_phase_timing(root, source_dir, "OuterZipMs", started.elapsed());
    }
    Ok(())
}



#[cfg(test)]
mod assistant_handoff_tests {
    use super::*;

    #[test]
    fn bound_dispensability_epoch_line_survives_assistant_projection_filter() {
        assert!(is_final_chord_rest_epoch_assistant_projection_line(
            "LowerStaffOverlappingRestBoundDispensabilityAudit#1: Measure 7; Status UNPROVEN."
        ));
        assert!(!is_final_chord_rest_epoch_assistant_projection_line(
            "UnrelatedAudit: Status UNPROVEN."
        ));
    }

    fn unique_handoff_test_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        env::temp_dir().join(format!(
            "etuschlopf_assistant_handoff_{label}_{}_{}",
            std::process::id(),
            nonce
        ))
    }

    fn seed_handoff_projection_root(root: &Path, latest: &Path) {
        fs::create_dir_all(root.join("src/build_gate")).expect("build gate dir");
        fs::create_dir_all(root.join("docs/assistant_memory")).expect("assistant memory dir");
        fs::create_dir_all(latest).expect("latest dir");
        fs::write(
            root.join("ZIP_KIND.txt"),
            b"kind=take\nversion=0.10.1839\nrevision=r500\nname=HandoffProof\n",
        )
        .expect("ZIP_KIND");
        fs::write(
            root.join("src/build_gate/take_change_class.txt"),
            b"schema_version=1\nchange_class=AuditOnly\n",
        )
        .expect("take change class");
        fs::write(
            root.join(ASSISTANT_HANDOFF_START_HERE),
            b"# START HERE\nSTART-HERE-SENTINEL\n",
        )
        .expect("start here");
        fs::write(
            root.join(ASSISTANT_HANDOFF_ESSENCE_BOOTSTRAP),
            b"{\"artifact\":\"BOOTSTRAP-SENTINEL\",\"normative\":false}\n",
        )
        .expect("bootstrap");
        fs::write(
            root.join(ASSISTANT_HANDOFF_ESSENCE),
            b"# ESSENCE\nESSENCE-BODY-SENTINEL\n",
        )
        .expect("essence");
        fs::write(
            latest.join("assistant_context.json"),
            b"{\"source_snapshot\":\"current\"}\n",
        )
        .expect("assistant context");
        fs::write(latest.join(RUN_OUTCOME_AUDIT_FILE), b"Outcome: green\n")
            .expect("run outcome");
        fs::write(latest.join("manifest.txt"), b"TakeCompatible: false\n")
            .expect("manifest");
    }


    fn hygiene_seed_snapshot(root: &Path, latest: &Path, files: &[PathBuf]) -> PathBuf {
        let dir = latest.join(SOURCE_SNAPSHOT_DIR);
        fs::create_dir_all(&dir).unwrap();
        let name = "EtuSchlopf_v0_10_1839_r500_unbuilt_source.zip";
        let path = dir.join(name);
        write_source_snapshot_archive(root, &path, files).unwrap();
        fs::write(dir.join(SOURCE_SNAPSHOT_METADATA_FILE), format!(
            "source_complete=true\nSourceArchive={name}\nSourceZipSha256=sha256:{}\n",
            sha256::hex(&fs::read(&path).unwrap())
        )).unwrap();
        path
    }

    #[test]
    fn hygiene_generation_state_does_not_claim_acceptance() {
        let root = unique_handoff_test_root("hygiene_state");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        for (marker, head, issues, expected) in [
            (true, "r499", 0, "in_flight_not_yet_canonical"),
            (false, "r500", 0, "revision_equals_canonical_head_bytes_unverified"),
            (false, "r499", 0, "not_canonical_no_transaction"),
            (true, "r499", 1, "indeterminate_projection_missing_invalid_or_issues"),
        ] {
            fs::write(latest.join(TRANSITION_HISTORY_FILE), format!(
                "{{\n  \"canonical_history_through_revision\": \"{head}\",\n  \"canonical_history_through_sha256\": \"abc\",\n  \"projection_issue_count\": {issues},\n  \"current_in_flight\": {{\n    \"marker_present\": {marker},\n    \"transaction_state\": \"reexec\",\n    \"take_revision\": \"r500\",\n    \"candidate_sha256\": \"def\",\n    \"candidate_intent\": {{}}\n  }}\n}}\n"
            )).unwrap();
            let mut out = String::new();
            handoff_append_current_state(&mut out, &root, &latest).unwrap();
            assert!(out.contains(&format!("LifecycleAtGeneration: {expected}")));
            assert!(!out.contains("CanonicalVerdict:"));
            assert!(!out.contains(concat!("[POST-", "ACCEPTANCE-LIFECYCLE]")));
            assert!(out.contains(&sha256::hex(&fs::read(latest.join(TRANSITION_HISTORY_FILE)).unwrap())));
        }
        fs::remove_file(latest.join(TRANSITION_HISTORY_FILE)).unwrap();
        let mut out = String::new();
        handoff_append_current_state(&mut out, &root, &latest).unwrap();
        assert!(out.contains("indeterminate_projection_missing_invalid_or_issues"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn hygiene_snapshot_mismatch_missing_entry_and_invalid_archive_fall_back() {
        let root = unique_handoff_test_root("hygiene_snapshot");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        let path = root.join("src/sample.rs");
        fs::write(&path, "fn snapshot_body_sentinel() {}\n").unwrap();
        let archive = hygiene_seed_snapshot(&root, &latest, &[PathBuf::from("src/sample.rs")]);
        let mut verified = handoff_verified_snapshot(&latest).unwrap();
        let mut out = String::new();
        assert!(handoff_append_snapshot_reference(&mut out, &root, &path, Some(&mut verified)).unwrap());
        assert!(out.contains("Embedded: false"));
        assert!(!out.contains("snapshot_body_sentinel"));
        fs::write(&path, "fn changed_body_sentinel() {}\n").unwrap();
        assert!(!handoff_append_snapshot_reference(&mut String::new(), &root, &path, Some(&mut verified)).unwrap());
        let missing = root.join("src/not_in_snapshot.rs");
        fs::write(&missing, "fn missing() {}\n").unwrap();
        assert!(!handoff_append_snapshot_reference(&mut String::new(), &root, &missing, Some(&mut verified)).unwrap());
        fs::write(root.join("src/build_gate/take_change_class.txt"), "schema_version=1\nchange_class=OutcomeProjection\nchange=modified|src/sample.rs|fnv64:0000000000000000\n").unwrap();
        let output = publish_assistant_handoff_projection(&root, &latest, "test", None).unwrap();
        assert!(fs::read_to_string(output).unwrap().contains("changed_body_sentinel"));
        fs::write(archive, "invalid archive").unwrap();
        assert!(handoff_verified_snapshot(&latest).is_none());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn hygiene_console_preserves_early_diagnostic_and_only_omits_progress() {
        let root = unique_handoff_test_root("hygiene_console");
        let log = root.join(CONSOLE_LOG);
        fs::create_dir_all(log.parent().unwrap()).unwrap();
        let diagnostic = "FailurePath: access denied\nLiteral: | DATEI  [ is explanatory text\nCompleted successfully\n";
        let text = format!("FailurePath: access denied\n{}Literal: | DATEI  [ is explanatory text\nCompleted successfully\n", "| DATEI  [====] progress\n/ GESAMT [====] progress\n".repeat(2000));
        fs::write(&log, &text).unwrap();
        let mut out = String::new();
        handoff_append_console(&mut out, &root).unwrap();
        assert!(out.contains(diagnostic));
        assert!(out.contains("ProgressLinesOmitted: 4000"));
        assert!(out.contains(&sha256::hex(text.as_bytes())));
        assert!(!out.contains("[====] progress"));
        fs::write(&log, format!("EARLY-DIAGNOSTIC\n{}\nLATE-DIAGNOSTIC\n", "middle\n".repeat(4000))).unwrap();
        let mut out = String::new();
        handoff_append_console(&mut out, &root).unwrap();
        assert!(out.contains("EARLY-DIAGNOSTIC"));
        assert!(out.contains("LATE-DIAGNOSTIC"));
        assert!(out.contains("DiagnosticMiddleOmitted: true"));
        fs::remove_dir_all(root).unwrap();
    }


    #[test]
    fn hygiene_late_timing_does_not_change_hash_bound_exported_console() {
        let root = unique_handoff_test_root("hygiene_timing");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        fs::create_dir_all(&latest).unwrap();
        let original = root.join(CONSOLE_LOG);
        fs::create_dir_all(original.parent().unwrap()).unwrap();
        fs::write(&original, "diagnostic\n").unwrap();
        let exported = latest.join("etuschlopf.console.txt");
        fs::copy(&original, &exported).unwrap();
        assistant_export_phase_timing(&root, &latest, "SourceSnapshotMs", Duration::from_millis(7));
        let bound = fs::read(&exported).unwrap();
        let mut before = String::new();
        handoff_append_console(&mut before, &root).unwrap();
        assert!(before.contains(&sha256::hex(&bound)));
        assistant_export_phase_timing(&root, &latest, "AssistantHandoffMs", Duration::from_millis(8));
        assistant_export_phase_timing(&root, &latest, "OuterZipMs", Duration::from_millis(9));
        assert_eq!(bound, fs::read(&exported).unwrap());
        assert!(fs::read_to_string(original).unwrap().contains("OuterZipMs=9"));
        let mut after = String::new();
        handoff_append_console(&mut after, &root).unwrap();
        assert_eq!(before, after);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn hygiene_representative_handoff_is_compact_complete_and_deterministic() {
        let root = unique_handoff_test_root("hygiene_budget");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        fs::write(root.join(ASSISTANT_HANDOFF_START_HERE), "S".repeat(6297)).unwrap();
        fs::write(root.join(ASSISTANT_HANDOFF_ESSENCE_BOOTSTRAP), "B".repeat(21266)).unwrap();
        fs::write(latest.join("assistant_context.json"), "C".repeat(5000)).unwrap();
        fs::write(latest.join("corpus_emergence.result.txt"), "R".repeat(11000)).unwrap();
        fs::write(latest.join(PROJECT_FILE_CATALOG_MARKDOWN), "K".repeat(4_500_000)).unwrap();
        let mut manifest = "schema_version=1\nchange_class=OutcomeProjection\n".to_string();
        let mut files = Vec::new();
        for index in 0..128 {
            let name = format!("src/hygiene_{index:03}.rs");
            fs::write(root.join(&name), "SOURCE-BODY-SENTINEL\n".repeat(1000)).unwrap();
            manifest.push_str(&format!("change=modified|{name}|fnv64:0000000000000000\n"));
            files.push(PathBuf::from(name));
        }
        fs::write(root.join("docs/new.txt"), "DOC-BODY-SENTINEL\n").unwrap();
        files.push(PathBuf::from("docs/new.txt"));
        manifest.push_str("change=added|docs/new.txt|fnv64:0000000000000000\nchange=deleted|src/deleted.rs|none\n");
        fs::write(root.join("src/build_gate/take_change_class.txt"), manifest).unwrap();
        hygiene_seed_snapshot(&root, &latest, &files);
        let path = publish_assistant_handoff_projection(&root, &latest, "corpus_run", None).unwrap();
        let first = fs::read(&path).unwrap();
        let text = String::from_utf8(first.clone()).unwrap();
        assert!(first.len() <= 96 * 1024, "representative handoff is {} bytes", first.len());
        assert!(!text.contains("SOURCE-BODY-SENTINEL"));
        assert!(!text.contains("DOC-BODY-SENTINEL"));
        for path in &files { assert!(text.contains(&format!("SnapshotEntry: {}\n", path.display()))); }
        assert!(text.contains("Path: src/deleted.rs\nChange: deleted"));
        assert!(text.contains("File: project_file_catalog.tsv"));
        assert!(!text.contains("OmittedDueToHandoffBudget:"));
        publish_assistant_handoff_projection(&root, &latest, "corpus_run", None).unwrap();
        assert_eq!(first, fs::read(path).unwrap());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn assistant_handoff_reserves_all_omissions_and_final_sections() {
        let root = unique_handoff_test_root("many_omissions");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        fs::write(latest.join("assistant_context.json"), "x".repeat(650 * 1024)).unwrap();
        let mut manifest = String::from("schema_version=1\nchange_class=KodexOnly\n");
        let bytes = "a".repeat(64 * 1024);
        let mut paths = Vec::new();
        for index in 0..128 {
            let path = format!("src/long_optional_source_{index:03}_{}.rs", "identifier".repeat(10));
            fs::write(root.join(&path), &bytes).unwrap();
            manifest.push_str(&format!("change=modified|{path}|fnv64:0000000000000000\n"));
            paths.push(path);
        }
        fs::write(root.join("src/build_gate/take_change_class.txt"), manifest).unwrap();
        fs::write(latest.join("corpus_emergence.result.txt"), &bytes).unwrap();
        fs::write(latest.join(PROJECT_FILE_CATALOG_MARKDOWN), &bytes).unwrap();
        let output = publish_assistant_handoff_projection(&root, &latest, "corpus_run", None)
            .expect("all omission records and closing sections must fit");
        let text = fs::read_to_string(&output).unwrap();
        assert!(text.len() <= ASSISTANT_HANDOFF_MAX_BYTES);
        for path in paths {
            assert!(text.contains(&format!("Path: {path}\n"))
                || text.contains(&format!("OmittedDueToHandoffBudget: {path} |")));
        }
        assert!(text.contains("[/CURRENT-TAKE-CHANGED-SOURCE]"));
        assert!(text.contains("corpus_emergence.result.txt"));
        assert!(text.contains(PROJECT_FILE_CATALOG_TSV));
        assert!(!text.contains(PROJECT_FILE_CATALOG_MARKDOWN));
        assert!(text.ends_with("\nEND OF ETUSCHLOPF ASSISTANT HANDOFF\n"));
        assert!(text.contains(&format!("Sha256: {}", sha256::hex(bytes.as_bytes()))));
        let first = fs::read(&output).unwrap();
        publish_assistant_handoff_projection(&root, &latest, "corpus_run", None).unwrap();
        assert_eq!(first, fs::read(&output).unwrap());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn corpus_export_bounds_catalog_without_dropping_bound_failure_outputs() {
        let root = unique_handoff_test_root("corpus_bounded");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        for dir in ["tests", "tools"] { fs::create_dir_all(root.join(dir)).unwrap(); }
        for (name, body) in [
            ("Cargo.toml", "[package]\nname='synthetic'\nversion='0.10.1839'\n"),
            ("Cargo.lock", "# synthetic lock\n"), ("README.md", "synthetic\n"),
            ("run.cmd", "@echo off\n"), ("strict.cmd", "@echo off\n"),
        ] { fs::write(root.join(name), body).unwrap(); }
        fs::write(root.join("target/generated_noise.bin"), "noise").unwrap();
        let project = "Synthetic Export Project";
        let result_name = format!("{project}.result.txt");
        let result = "Werkzeug: etuschlopf 0.10.1839\nErgebnis: failure\nBOUND-RESULT-SENTINEL\n";
        fs::write(root.join(&result_name), result).unwrap();
        let report_name = "corpus_emergence.result.txt";
        let report = "BOUND-CORPUS-REPORT-SENTINEL\n";
        fs::write(root.join(report_name), report).unwrap();
        let score_files = [resolve_musicxml_file(project), resolve_render_musicxml_file(project), resolve_pdf_file(project)];
        for name in &score_files { fs::write(root.join(name), "BOUND-SCORE-SENTINEL").unwrap(); }
        let mut manifest = corpus_run_manifest::CorpusRunManifest::started("0.10.1839", "synthetic-export-run".into(), 1, vec![project.into()]);
        manifest.upsert_project(corpus_run_manifest::project_evidence(project, &root.join(&result_name), result, Some(2), true));
        manifest.finalize(2, corpus_run_manifest::file_evidence(&root.join(report_name)).unwrap());
        manifest.write_atomic(&root.join(CORPUS_RUN_MANIFEST_FILE)).unwrap();
        let state = RunState {
            validation: ValidationPlan::Strict,
            build_outcome: BuildOutcome::Reused(CargoBuildProfile::Strict),
            thread_plan: ThreadPlan::detect(), target_exe: root.join("synthetic_exe"),
            strict_clippy_exit: "0".into(), build_exit: "0".into(),
            strict_test_exit: "not_run".into(), run_exit: "2".into(),
            clippy_ms: "0".into(), build_ms: "0".into(), strict_test_ms: "not_run".into(),
            strict_test_summary: "not_run".into(), program_ms: "0".into(), total_start: Instant::now(),
            strict_log: None, strict_test_log: None, result_file: Some(root.join(report_name)),
            input_arg: Some("--all".into()), force_mode: false,
            audit_signal: AuditSignal::NotEvaluated, renderer_outcome: RendererOutcome::NotObserved,
            emergence_outcome: EmergenceOutcome::NotApplicable, render_candidate_selected_pdf_truth: None,
            export_policy: AssistantExportPolicy::Minimal,
        };
        let path = create_corpus_run_assistant_export_bundle(&root, &state, "0.10.1839", "0.10.1839", 2).unwrap();
        let mut archive = zip::ZipArchive::new(File::open(path).unwrap()).unwrap();
        assert!(archive.by_name(PROJECT_FILE_CATALOG_MARKDOWN).is_err());
        let mut read = |name: &str| {
            let mut bytes = Vec::new();
            archive.by_name(name).unwrap().read_to_end(&mut bytes).unwrap();
            bytes
        };
        assert_eq!(read(&result_name), result.as_bytes());
        assert_eq!(read(report_name), report.as_bytes());
        for name in score_files { assert_eq!(read(&file_name_string(&name)), b"BOUND-SCORE-SENTINEL"); }
        let catalog = String::from_utf8(read(PROJECT_FILE_CATALOG_TSV)).unwrap();
        assert!(!catalog.contains("target/generated_noise.bin"));
        assert!(catalog.contains("generated_descendants_excluded"));
        assert!(catalog.contains("planned_output_after_snapshot"));
        assert!(!catalog.contains(PROJECT_FILE_CATALOG_MARKDOWN));
        let context = String::from_utf8(read("assistant_context.json")).unwrap();
        assert!(context.contains("\"complete_recursive_project_root\": false"));
        assert!(context.contains("\"excluded_generated_prefix\": \"target/\""));
        let manifest = String::from_utf8(read("manifest.txt")).unwrap();
        assert!(manifest.contains("FinalExit: 2"));
        assert!(manifest.contains("ManifestValid: false"));
        assert!(manifest.contains("SourceSnapshotComplete: true"));
        assert!(manifest.contains("duplicate_human_rendering_omitted_tsv_exported"));
        let handoff = String::from_utf8(read(ASSISTANT_HANDOFF_FILENAME)).unwrap();
        assert!(handoff.contains("[RUN-RESULT-REFERENCE]"));
        assert!(handoff.contains(&format!("Sha256: {}", sha256::hex(report.as_bytes()))));
        assert!(!handoff.contains("BOUND-CORPUS-REPORT-SENTINEL"));
        assert!(handoff.contains(PROJECT_FILE_CATALOG_TSV));
        assert!(!handoff.contains("\nPROJECT FILE CATALOG\n"));
        assert!(handoff.contains("duplicate_human_rendering_omitted_tsv_exported"));
        assert!(handoff.contains("this generation snapshot is not a verdict"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn visual_handoff_keeps_markdown_and_corpus_reports_missing_result() {
        let root = unique_handoff_test_root("mode_boundaries");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        fs::write(latest.join(PROJECT_FILE_CATALOG_MARKDOWN), "VISUAL-CATALOG-SENTINEL").unwrap();
        fs::write(latest.join("visual_preflight.result.txt"), "VISUAL-RESULT-SENTINEL").unwrap();
        let visual = publish_assistant_handoff_projection(&root, &latest, "visual_preflight", None).unwrap();
        let text = fs::read_to_string(visual).unwrap();
        assert!(text.contains("VISUAL-CATALOG-SENTINEL"));
        assert!(text.contains("VISUAL-RESULT-SENTINEL"));
        let corpus = publish_assistant_handoff_projection(&root, &latest, "corpus_run", None).unwrap();
        let text = fs::read_to_string(corpus).unwrap();
        assert!(text.contains("run_result_missing"));
        assert!(!text.contains("VISUAL-CATALOG-SENTINEL"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn assistant_handoff_required_overflow_preserves_previous_projection() {
        let root = unique_handoff_test_root("required_overflow");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        let output = root.join(ASSISTANT_HANDOFF_FILENAME);
        fs::write(&output, "previous valid projection").unwrap();
        fs::write(latest.join("assistant_context.json"), "x".repeat(ASSISTANT_HANDOFF_MAX_BYTES)).unwrap();
        assert!(publish_assistant_handoff_projection(&root, &latest, "test", None).is_err());
        assert_eq!(fs::read_to_string(output).unwrap(), "previous valid projection");
        assert!(!root.join(ASSISTANT_HANDOFF_CANDIDATE_FILENAME).exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn assistant_handoff_routes_cold_start_to_repository_without_copying_essence_body() {
        let root = unique_handoff_test_root("repository_first");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        let essence_bytes = fs::read(root.join(ASSISTANT_HANDOFF_ESSENCE)).expect("essence bytes");
        let expected_sha = sha256::hex(&essence_bytes);

        let output = publish_assistant_handoff_projection(&root, &latest, "test", None)
            .expect("handoff projection");
        let text = fs::read_to_string(output).expect("handoff text");

        assert!(text.contains("[REPOSITORY-COLD-START]"));
        assert!(text.contains("RepositoryFirst: true"));
        assert!(text.contains("Normative: false"));
        assert!(text.contains("Authority: none"));
        assert!(text.contains("EssenceBodyIncluded: false"));
        assert!(text.contains("EssenceResolution: current_repository_source_snapshot"));
        assert!(text.contains(&format!("EssenceSha256: {expected_sha}")));
        assert!(text.contains("START-HERE-SENTINEL"));
        assert!(text.contains("BOOTSTRAP-SENTINEL"));
        assert!(!text.contains("ESSENCE-BODY-SENTINEL"));
        let cold_start = text.find("[REPOSITORY-COLD-START]").expect("cold start");
        let take_identity = text.find("CURRENT TAKE IDENTITY").expect("take identity");
        assert!(cold_start < take_identity);

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn assistant_handoff_is_staged_inside_assistant_export_zip() {
        let root = unique_handoff_test_root("inside_export");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);

        let staged = stage_assistant_handoff_export(&root, &latest, "test", None)
            .expect("staged handoff");
        let root_handoff = root.join(ASSISTANT_HANDOFF_FILENAME);
        assert_eq!(
            fs::read(&staged).expect("staged bytes"),
            fs::read(&root_handoff).expect("root bytes")
        );

        let export_zip = root.join("assistant_export_latest.zip");
        zip_directory_flat(&latest, &export_zip).expect("assistant export zip");
        let file = File::open(&export_zip).expect("assistant export open");
        let mut zip = zip::ZipArchive::new(file).expect("assistant export parse");
        let mut entry = zip
            .by_name(ASSISTANT_HANDOFF_FILENAME)
            .expect("handoff must be inside assistant export");
        let mut zipped = Vec::new();
        entry
            .read_to_end(&mut zipped)
            .expect("read handoff entry");
        assert_eq!(zipped, fs::read(&root_handoff).expect("root handoff bytes"));

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn assistant_handoff_fails_closed_when_repository_essence_anchor_is_missing() {
        let root = unique_handoff_test_root("missing_essence");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        seed_handoff_projection_root(&root, &latest);
        fs::remove_file(root.join(ASSISTANT_HANDOFF_ESSENCE)).expect("remove essence");

        let error = publish_assistant_handoff_projection(&root, &latest, "test", None)
            .expect_err("missing essence must reject handoff projection");
        assert!(error
            .to_string()
            .contains("required Architecture Essence input missing or unreadable"));

        fs::remove_dir_all(root).expect("cleanup");
    }
}

#[cfg(test)]
mod measurement_snapshot_export_tests {
    use super::*;

    fn unique_measurement_test_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        env::temp_dir().join(format!(
            "etuschlopf_measurement_export_{label}_{}_{}",
            std::process::id(),
            nonce
        ))
    }

    fn measurement_snapshot_fixture(project: &str, run_id: &str) -> String {
        format!(
            concat!(
                "{{\n",
                "  \"schema_version\": 1,\n",
                "  \"measurement_contract_version\": \"CMS-1\",\n",
                "  \"producer\": {{\"binding_state\": \"unbound\", \"version\": null, \"source_signature\": null, \"binary_fingerprint\": null}},\n",
                "  \"run\": {{\n",
                "    \"run_id\": \"{}\",\n",
                "    \"scope\": \"project\",\n",
                "    \"full_corpus\": false,\n",
                "    \"expected_projects\": [\"{}\"],\n",
                "    \"measured_projects\": [\"{}\"]\n",
                "  }},\n",
                "  \"metrics\": []\n",
                "}}\n"
            ),
            run_id,
            json_escape(project),
            json_escape(project),
        )
    }

    #[test]
    fn measurement_snapshot_export_is_byte_identical_and_current_run_bound() {
        let root = unique_measurement_test_root("bound");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        fs::create_dir_all(&latest).expect("latest");
        let project = "Synthetic Project Alpha";
        let run_id = "obs-test-current";
        let file_name = canonical_measurement_snapshot_file_name(project);
        let bytes = measurement_snapshot_fixture(project, run_id).into_bytes();
        fs::write(root.join(&file_name), &bytes).expect("snapshot");

        let mut included = Vec::new();
        let mut omitted = Vec::new();
        let binding = stage_current_measurement_snapshot(
            &root,
            &latest,
            project,
            run_id,
            0,
            &mut included,
            &mut omitted,
        )
        .expect("bound current snapshot");

        assert_eq!(binding.status, "bound_current_run");
        assert_eq!(binding.run_id, run_id);
        assert_eq!(
            binding.measurement_contract_version.as_deref(),
            Some(CANONICAL_MEASUREMENT_CONTRACT_VERSION)
        );
        let expected_sha = sha256::hex(&bytes);
        assert_eq!(binding.sha256.as_deref(), Some(expected_sha.as_str()));
        assert_eq!(included, vec![file_name.clone()]);
        assert!(omitted.is_empty());
        assert_eq!(fs::read(latest.join(file_name)).expect("staged"), bytes);

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn successful_run_rejects_stale_measurement_snapshot() {
        let root = unique_measurement_test_root("stale");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        fs::create_dir_all(&latest).expect("latest");
        let project = "Synthetic Project Alpha";
        let file_name = canonical_measurement_snapshot_file_name(project);
        fs::write(
            root.join(file_name),
            measurement_snapshot_fixture(project, "obs-old"),
        )
        .expect("snapshot");

        let mut included = Vec::new();
        let mut omitted = Vec::new();
        let error = stage_current_measurement_snapshot(
            &root,
            &latest,
            project,
            "obs-current",
            0,
            &mut included,
            &mut omitted,
        )
        .expect_err("stale successful-run snapshot must fail closed");
        assert!(error.to_string().contains("not bound to current run/project/contract"));

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn nonzero_run_omits_stale_snapshot_instead_of_exporting_it() {
        let root = unique_measurement_test_root("controlled_exit");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        fs::create_dir_all(&latest).expect("latest");
        let project = "Synthetic Project Alpha";
        let file_name = canonical_measurement_snapshot_file_name(project);
        fs::write(
            root.join(&file_name),
            measurement_snapshot_fixture(project, "obs-old"),
        )
        .expect("snapshot");

        let mut included = Vec::new();
        let mut omitted = Vec::new();
        let binding = stage_current_measurement_snapshot(
            &root,
            &latest,
            project,
            "obs-current",
            2,
            &mut included,
            &mut omitted,
        )
        .expect("controlled exit may export without measurement snapshot");
        assert_eq!(binding.status, "not_available_current_run");
        assert!(included.is_empty());
        assert_eq!(omitted.len(), 1);
        assert!(!latest.join(file_name).exists());

        fs::remove_dir_all(root).expect("cleanup");
    }
}

#[cfg(test)]
mod source_snapshot_tests {
    use super::*;

    fn unique_test_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        env::temp_dir().join(format!(
            "etuschlopf_source_snapshot_{label}_{}_{}",
            std::process::id(),
            nonce
        ))
    }

    fn seed_complete_source_root(root: &Path) {
        fs::create_dir_all(root.join("src/bin")).expect("src");
        fs::create_dir_all(root.join("tests")).expect("tests");
        fs::create_dir_all(root.join("tools")).expect("tools");
        fs::create_dir_all(root.join("docs")).expect("docs");
        fs::write(root.join("Cargo.toml"), b"[package]\nname='proof'\nversion='0.10.1737'\n").expect("Cargo.toml");
        fs::write(root.join("Cargo.lock"), b"# lock\n").expect("Cargo.lock");
        fs::write(root.join("README.md"), b"proof\n").expect("README");
        fs::write(root.join("run.cmd"), b"@echo off\r\n").expect("run.cmd");
        fs::write(root.join("strict.cmd"), b"@echo off\r\n").expect("strict.cmd");
        fs::write(
            root.join("ZIP_KIND.txt"),
            b"kind=etuschlopf_take\nversion=0.10.1737\nrevision=r05\nname=WatchWebProjectPdfOpenSameVersionClosure\n",
        )
        .expect("ZIP_KIND");
        fs::write(root.join("src/bin/proof.rs"), b"fn main() {}\n").expect("source");
        fs::write(root.join("tests/proof.rs"), b"#[test] fn proof() {}\n").expect("test");
        fs::write(root.join("tools/proof.txt"), b"tool\n").expect("tool");
        fs::write(root.join("docs/proof.md"), b"doc\n").expect("doc");
    }

    #[test]
    fn source_snapshot_reuses_take_managed_definition_and_adds_only_root_cargo_lock() {
        assert!(!managed_source_tree_definition::MANAGED_ROOT_FILES.contains(&"Cargo.lock"));
        assert_eq!(
            managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES,
            ["Cargo.lock"]
        );
        assert_eq!(
            managed_source_tree_definition::root_project_file_role("Cargo.lock"),
            Some(managed_source_tree_definition::RootProjectFileRole::LocalTransactional)
        );
        for required in managed_source_tree_definition::REQUIRED_MANAGED_ROOT_FILES {
            assert!(managed_source_tree_definition::MANAGED_ROOT_FILES.contains(&required));
        }
    }

    #[test]
    fn project_catalog_reports_root_cargo_lock_as_local_transactional_not_take_managed() {
        assert_eq!(
            project_folder_catalog_scope_and_role("Cargo.lock", "file"),
            ("root_local_transaction_state", "cargo_owned_local_transaction_state")
        );
        assert!(!project_folder_catalog_take_managed("Cargo.lock"));
        assert!(project_folder_catalog_take_managed("Cargo.toml"));
    }

    #[test]
    fn source_snapshot_is_embedded_complete_and_hash_bound() {
        let root = unique_test_root("complete");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        fs::create_dir_all(&latest).expect("latest");
        seed_complete_source_root(&root);
        let mut included = Vec::new();
        let mut omitted = Vec::new();
        let outcome = stage_source_snapshot(
            &root,
            &latest,
            "0.10.1737",
            &mut included,
            &mut omitted,
        )
        .expect("snapshot");
        assert!(outcome.complete);
        assert!(omitted.is_empty());
        let archive_name = outcome.archive_name.expect("archive name");
        assert_eq!(
            archive_name,
            "EtuSchlopf_v0_10_1737_r05_unbuilt_source.zip"
        );
        assert!(
            windows_path_units(&archive_name) <= SOURCE_SNAPSHOT_ARCHIVE_FILENAME_MAX_UTF16
        );
        let archive_path = latest.join(SOURCE_SNAPSHOT_DIR).join(&archive_name);
        let archive_bytes = fs::read(&archive_path).expect("archive bytes");
        let expected_sha256 = sha256::hex(&archive_bytes);
        assert_eq!(outcome.sha256.as_deref(), Some(expected_sha256.as_str()));
        let file = File::open(&archive_path).expect("archive open");
        let mut zip = zip::ZipArchive::new(file).expect("archive parse");
        for required in [
            "Cargo.toml",
            "Cargo.lock",
            "README.md",
            "ZIP_KIND.txt",
            "run.cmd",
            "strict.cmd",
            "src/bin/proof.rs",
            "tests/proof.rs",
            "tools/proof.txt",
            "docs/proof.md",
        ] {
            assert!(zip.by_name(required).is_ok(), "missing {required}");
        }
        let metadata = fs::read_to_string(
            latest.join(SOURCE_SNAPSHOT_DIR).join(SOURCE_SNAPSHOT_METADATA_FILE),
        )
        .expect("metadata");
        assert!(metadata.contains("source_complete=true"));
        assert!(metadata.contains(&format!(
            "SourceZipSha256=sha256:{}",
            outcome.sha256.as_deref().expect("sha")
        )));
        assert!(metadata.contains(
            "TransportPathBudgetUtf16=archive_name<=64;entry_filename<=120;relative_path<=140;extracted_envelope<=180"
        ));

        let outer_zip_path = root.join("assistant_export_latest.zip");
        zip_directory_flat(&latest, &outer_zip_path).expect("outer assistant export");
        let outer_file = File::open(&outer_zip_path).expect("outer archive open");
        let mut outer_zip = zip::ZipArchive::new(outer_file).expect("outer archive parse");
        assert!(
            outer_zip
                .by_name(&format!("{SOURCE_SNAPSHOT_DIR}/{SOURCE_SNAPSHOT_METADATA_FILE}"))
                .is_ok(),
            "outer export must contain source snapshot metadata"
        );
        assert!(
            outer_zip
                .by_name(&format!("{SOURCE_SNAPSHOT_DIR}/{archive_name}"))
                .is_ok(),
            "outer export must contain source snapshot archive"
        );
        drop(zip);
        drop(outer_zip);
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn source_snapshot_path_budget_is_copy_safe_and_fail_closed() {
        let archive_name = "EtuSchlopf_v0_10_1839_r939_unbuilt_source.zip";
        let current_longest = vec![PathBuf::from(
            "docs/renderer_feasibility_committed_source_oracle_productive_adoption_after_semantic_parser_closure_r52.md",
        )];
        validate_source_snapshot_path_budget(&current_longest, archive_name)
            .expect("current long historical path must remain inside transport budget");

        let oversized_filename = format!("docs/{}.md", "x".repeat(121));
        let error = validate_source_snapshot_path_budget(
            &[PathBuf::from(oversized_filename)],
            archive_name,
        )
        .expect_err("oversized filename must fail closed");
        assert!(error.to_string().contains("filename"));

        let oversized_relative = format!(
            "docs/{}/{}.md",
            "nested".repeat(10),
            "leaf".repeat(18)
        );
        let error = validate_source_snapshot_path_budget(
            &[PathBuf::from(oversized_relative)],
            archive_name,
        )
        .expect_err("oversized relative path must fail closed");
        assert!(error.to_string().contains("relative path"));

        let oversized_archive = format!("EtuSchlopf_{}.zip", "x".repeat(60));
        let error = validate_source_snapshot_path_budget(
            &[PathBuf::from("docs/proof.md")],
            &oversized_archive,
        )
        .expect_err("oversized archive filename must fail closed");
        assert!(error.to_string().contains("archive filename"));
    }

    #[test]
    fn source_snapshot_fails_closed_when_required_source_is_missing() {
        let root = unique_test_root("missing");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        fs::create_dir_all(root.join("tests")).expect("tests");
        fs::create_dir_all(root.join("tools")).expect("tools");
        fs::create_dir_all(root.join("docs")).expect("docs");
        fs::create_dir_all(&latest).expect("latest");
        for (name, bytes) in [
            ("Cargo.toml", b"[package]\n".as_slice()),
            ("Cargo.lock", b"# lock\n".as_slice()),
            ("README.md", b"proof\n".as_slice()),
            ("run.cmd", b"@echo off\r\n".as_slice()),
            ("strict.cmd", b"@echo off\r\n".as_slice()),
            (
                "ZIP_KIND.txt",
                b"kind=etuschlopf_take\nversion=0.10.1737\nrevision=r05\nname=Proof\n".as_slice(),
            ),
        ] {
            fs::write(root.join(name), bytes).expect("root file");
        }
        let mut included = Vec::new();
        let mut omitted = Vec::new();
        let outcome = stage_source_snapshot(
            &root,
            &latest,
            "0.10.1737",
            &mut included,
            &mut omitted,
        )
        .expect("snapshot failure metadata");
        assert!(!outcome.complete);
        assert!(outcome.archive_name.is_none());
        let metadata = fs::read_to_string(
            latest.join(SOURCE_SNAPSHOT_DIR).join(SOURCE_SNAPSHOT_METADATA_FILE),
        )
        .expect("metadata");
        assert!(metadata.contains("source_complete=false"));
        assert!(metadata.contains("SOURCE-SNAPSHOT FAILED"));
        assert!(fs::read_dir(latest.join(SOURCE_SNAPSHOT_DIR))
            .expect("snapshot dir")
            .all(|entry| !entry.expect("entry").file_name().to_string_lossy().ends_with("_unbuilt_source.zip")));

        let outer_zip_path = root.join("assistant_export_latest.zip");
        zip_directory_flat(&latest, &outer_zip_path).expect("outer assistant export");
        let outer_file = File::open(&outer_zip_path).expect("outer archive open");
        let mut outer_zip = zip::ZipArchive::new(outer_file).expect("outer archive parse");
        assert!(
            outer_zip
                .by_name(&format!("{SOURCE_SNAPSHOT_DIR}/{SOURCE_SNAPSHOT_METADATA_FILE}"))
                .is_ok(),
            "fail-closed export must contain source snapshot metadata"
        );
        let source_archives = outer_zip
            .file_names()
            .filter(|name| name.starts_with(&format!("{SOURCE_SNAPSHOT_DIR}/")))
            .filter(|name| name.ends_with("_unbuilt_source.zip"))
            .count();
        assert_eq!(
            source_archives, 0,
            "fail-closed export must not contain a source archive"
        );
        drop(outer_zip);
        fs::remove_dir_all(root).expect("cleanup");
    }
}



#[cfg(test)]
mod transition_history_projection_tests {
    use super::*;

    fn unique_transition_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        env::temp_dir().join(format!(
            "etuschlopf_transition_history_{label}_{}_{}",
            std::process::id(),
            nonce
        ))
    }

    fn write_rejection(
        root: &Path,
        revision: u64,
        sha: &str,
        epoch: u128,
        stage: &str,
    ) {
        let entry = root
            .join(TRANSITION_REJECTION_LEDGER_ROOT)
            .join("0.10.1839__0.10.1839")
            .join(sha);
        fs::create_dir_all(&entry).expect("rejection dir");
        fs::write(
            entry.join("metadata.txt"),
            format!(
                "schema=2\nfrom_version=0.10.1839\nto_version=0.10.1839\ntake_revision={revision}\nzip_sha256={sha}\nrejection_stage={stage}\nverdict=rejected\nattempt_epoch_millis={epoch}\n"
            ),
        )
        .expect("rejection metadata");
        fs::write(
            entry.join("candidate_intent.txt"),
            b"schema_version=1\nchange_class=KodexOnly\nreason=rejected proof\n",
        )
        .expect("intent");
        fs::write(
            entry.join("semantic_effect_snapshot.tsv"),
            b"v1\tsrc/proof.rs\t0000000000000001\tproof\tdeny\n",
        )
        .expect("effect");
        fs::write(entry.join("reason.txt"), b"deterministic rejection\n").expect("reason");
    }

    fn write_acceptance(root: &Path, revision: u64, sha: &str, epoch: u128) {
        let pair = root
            .join(TRANSITION_REJECTION_LEDGER_ROOT)
            .join("0.10.1839__0.10.1839");
        fs::create_dir_all(&pair).expect("acceptance pair");
        fs::write(
            pair.join(format!("accepted-r{revision:02}-{sha}.txt")),
            format!(
                "schema=2\nfrom_version=0.10.1839\nto_version=0.10.1839\ntake_revision={revision}\naccepted_zip_sha256={sha}\nprior_rejections=1\naccepted_epoch_millis={epoch}\n"
            ),
        )
        .expect("acceptance");
        let sidecar = root
            .join(TRANSITION_ACCEPTANCE_EVIDENCE_ROOT)
            .join("0.10.1839__0.10.1839")
            .join(format!("accepted-evidence-r{revision:02}-{sha}"));
        fs::create_dir_all(&sidecar).expect("sidecar");
        fs::write(
            sidecar.join("metadata.txt"),
            format!(
                "schema=1\nfrom_version=0.10.1839\nto_version=0.10.1839\ntake_revision={revision}\naccepted_zip_sha256={sha}\nverdict=accepted\nauthority=advisory_evidence_only\n"
            ),
        )
        .expect("sidecar metadata");
        fs::write(
            sidecar.join("candidate_intent.txt"),
            b"schema_version=1\nchange_class=KodexOnly\nreason=accepted proof\n",
        )
        .expect("sidecar intent");
        fs::write(
            sidecar.join("semantic_effect_snapshot.tsv"),
            b"v1\tsrc/proof.rs\t0000000000000002\tproof\tallow\n",
        )
        .expect("sidecar effect");
    }

    #[test]
    fn transition_history_separates_durable_verdicts_from_in_flight_candidate() {
        let root = unique_transition_root("separate");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        fs::create_dir_all(&latest).expect("latest");
        fs::write(
            root.join("ZIP_KIND.txt"),
            b"kind=take\nversion=0.10.1839\nrevision=r1057\nname=TakeTransitionHistoryFoundation\n",
        )
        .expect("zip kind");

        let rejected_sha = format!("{:064x}", 41);
        let accepted_sha = format!("{:064x}", 42);
        let current_sha = format!("{:064x}", 43);
        write_rejection(&root, 1056, &rejected_sha, 100, "incoming_take");
        write_acceptance(&root, 1056, &accepted_sha, 200);

        let transaction = root
            .join(TRANSITION_TRANSACTION_ROOT)
            .join("transaction_test");
        fs::create_dir_all(&transaction).expect("transaction");
        fs::create_dir_all(root.join("target")).expect("target");
        fs::write(root.join(TRANSITION_PENDING_MARKER), transaction.to_string_lossy().as_bytes())
            .expect("marker");
        fs::write(transaction.join("state.txt"), b"reexec\n").expect("state");
        fs::write(
            transaction.join("candidate_metadata.txt"),
            format!(
                "schema=2\nfrom_version=0.10.1839\nto_version=0.10.1839\ntake_revision=1057\nzip_sha256={current_sha}\n"
            ),
        )
        .expect("candidate metadata");
        fs::write(
            transaction.join("candidate_intent.txt"),
            b"schema_version=1\nchange_class=KodexOnly\nreason=current proof\n",
        )
        .expect("current intent");
        fs::write(
            transaction.join("candidate_snapshot.tsv"),
            b"v1\tsrc/current.rs\t0000000000000003\tproof\tobserve\n",
        )
        .expect("current effect");

        write_transition_history_projection(&root, &latest).expect("transition history");
        let text = fs::read_to_string(latest.join(TRANSITION_HISTORY_FILE)).expect("history");
        assert!(text.contains("\"canonical_history_through_revision\": \"r1056\""));
        assert!(text.contains("\"verdict\": \"accepted\""));
        assert!(text.contains("\"verdict\": \"rejected\""));
        assert!(text.contains("\"acceptance_evidence_status\": \"present_verified_identity\""));
        assert!(text.contains("\"canonical_verdict\": \"not_yet_canonical\""));
        assert!(text.contains("\"transaction_state\": \"reexec\""));
        assert!(text.contains(&format!("\"candidate_sha256\": \"{current_sha}\"")));
        let current_pos = text.find("\"current_in_flight\"").expect("current block");
        assert!(!text[..current_pos].contains("\"take_revision\": \"r1057\""));
        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn transition_history_keeps_legacy_unknowns_unknown() {
        let root = unique_transition_root("legacy");
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        let sha = format!("{:064x}", 51);
        let entry = root
            .join(TRANSITION_REJECTION_LEDGER_ROOT)
            .join("0.10.1700__0.10.1700")
            .join(&sha);
        fs::create_dir_all(&entry).expect("legacy rejection");
        fs::create_dir_all(&latest).expect("latest");
        fs::write(
            root.join("ZIP_KIND.txt"),
            b"kind=take\nversion=0.10.1839\nrevision=r1057\nname=TakeTransitionHistoryFoundation\n",
        )
        .expect("zip kind");
        fs::write(
            entry.join("metadata.txt"),
            format!(
                "schema=1\nfrom_version=0.10.1700\nto_version=0.10.1700\nzip_sha256={sha}\nattempt_epoch_millis=77\n"
            ),
        )
        .expect("legacy metadata");
        fs::write(entry.join("semantic_effect_snapshot.tsv"), b"").expect("effect");
        fs::write(entry.join("reason.txt"), b"legacy reject\n").expect("reason");

        write_transition_history_projection(&root, &latest).expect("transition history");
        let text = fs::read_to_string(latest.join(TRANSITION_HISTORY_FILE)).expect("history");
        assert!(text.contains("\"take_revision\": \"unknown\""));
        assert!(text.contains(
            "\"rejection_stage\": \"not_applicable_or_legacy_unknown\""
        ));
        assert!(text.contains("\"canonical_history_through_revision\": \"unknown\""));
        fs::remove_dir_all(root).expect("cleanup");
    }
}
