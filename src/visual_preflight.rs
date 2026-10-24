//! Typed orchestration boundary for the launcher visual preflight.
//!
//! This module owns session setup, phase ordering, and the no-seed finalization path.
//! The historical project and audit implementations remain in the launcher until their
//! interfaces are narrow enough for later extraction. No music or renderer decision is
//! implemented here.

#[path = "visual_preflight/no_seed.rs"]
mod no_seed;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use super::run_outcome::EmergenceOutcome;
use super::{
    execute_visual_preflight_clef_audits, execute_visual_preflight_general_audits,
    execute_visual_preflight_project_batch, finalize_visual_preflight_execution,
    visual_preflight_projects, yes_no_bool,
    AppResult, Launcher, RunState, VisualPreflightPlan, VisualProjectRunLedger,
};

pub struct VisualPreflightSession {
    pub(super) project_root: PathBuf,
    pub(super) plan: VisualPreflightPlan,
    pub(super) projects: Vec<String>,
    pub(super) aggregate_path: PathBuf,
}

impl VisualPreflightSession {
    fn has_current_target(&self) -> bool {
        !self.projects.is_empty() && self.plan.target_project.is_some()
    }
}

pub struct VisualPreflightProjectBatch {
    pub(super) final_exit: i32,
    pub(super) total_program_ms: u128,
    pub(super) ledgers: Vec<VisualProjectRunLedger>,
    pub(super) renderer_reuse_rows: Vec<String>,
}

pub struct VisualPreflightExecution {
    pub(super) final_exit: i32,
    pub(super) total_program_ms: u128,
    pub(super) ledgers: Vec<VisualProjectRunLedger>,
}

pub(super) fn execute_visual_preflight_run(
    launcher: &mut Launcher,
    state: &mut RunState,
) -> AppResult<i32> {
    let session = prepare_visual_preflight_session(launcher, state)?;
    if !session.has_current_target() {
        let request = no_seed::NoSeedFinalizationRequest::new(launcher, state, &session);
        let result = no_seed::complete(request)?;
        return Ok(result.exit_code());
    }

    let project_batch = execute_visual_preflight_project_batch(launcher, state, &session)?;
    let mut execution = execute_visual_preflight_general_audits(
        launcher,
        state,
        &session,
        project_batch,
    )?;
    execute_visual_preflight_clef_audits(
        launcher,
        state,
        &session,
        &mut execution,
    )?;
    finalize_visual_preflight_execution(launcher, state, &session, execution)
}

fn prepare_visual_preflight_session(
    launcher: &Launcher,
    state: &mut RunState,
) -> AppResult<VisualPreflightSession> {
    let project_root = env::current_dir()?;
    let plan = visual_preflight_projects(&project_root, &launcher.version);
    let projects = plan.projects.clone();
    state.emergence_outcome = if plan.target_project.is_some() {
        EmergenceOutcome::CurrentTarget
    } else {
        EmergenceOutcome::NoCurrentTarget
    };
    let aggregate_path = PathBuf::from("visual_preflight.result.txt");
    state.input_arg = Some("--visual-preflight".to_string());
    state.result_file = Some(aggregate_path.clone());
    let session = VisualPreflightSession {
        project_root,
        plan,
        projects,
        aggregate_path,
    };
    write_visual_preflight_header(&session, &launcher.version)?;
    Ok(session)
}

fn write_visual_preflight_header(
    session: &VisualPreflightSession,
    version: &str,
) -> AppResult<()> {
    let aggregate_path = session.aggregate_path.as_path();
    let plan = &session.plan;
    let projects = session.projects.as_slice();
    let mut file = File::create(aggregate_path)?;
    writeln!(file, "ETUSCHLOPF-VISUAL-PREFLIGHT")?;
    writeln!(file, "============================")?;
    writeln!(file, "Werkzeug: etuschlopf {}", version)?;
    writeln!(file, "Modus: Ein-Befehl-Schnellgate fuer regelbasierte VisualPdf-Layout-Relief-Preflight-Pruefung")?;
    writeln!(file, "FullCorpus: nein")?;
    writeln!(file, "MusicMutation: nein")?;
    writeln!(file, "FreshRunLedgerGate: active")?;
    writeln!(file, "ExpectedRunVersion: etuschlopf {}", version)?;
    writeln!(file, "ProjectIdUsedInPredicate: nein")?;
    writeln!(file, "ReferenceLockUsedInPredicate: nein")?;
    writeln!(file, "TargetProjectHardcoded: nein")?;
    writeln!(file, "TargetProjectSelector: {}", plan.selector_source)?;
    writeln!(file, "SelectorStatus: {}", plan.selector_status)?;
    writeln!(file, "CurrentEmergenceStatus: {}", plan.emergence.status)?;
    writeln!(file, "CurrentEmergenceReason: {}", plan.emergence.reason)?;
    writeln!(file, "CurrentEmergenceRunId: {}", plan.emergence.run_id.as_deref().unwrap_or("none"))?;
    writeln!(file, "CurrentEmergenceReportVersion: {}", plan.emergence.report_version.as_deref().unwrap_or("none"))?;
    writeln!(file, "CurrentEmergenceFullCorpus: {}", yes_no_bool(plan.emergence.full_corpus))?;
    writeln!(file, "CurrentEmergenceProjectCount: {}", plan.emergence.project_count)?;
    writeln!(file, "CurrentEmergenceReportedCurrentVersionProjectCount: {}", plan.emergence.reported_current_version_project_count)?;
    writeln!(file, "CurrentEmergenceVerifiedCurrentVersionProjectCount: {}", plan.emergence.current_version_project_count)?;
    writeln!(file, "CurrentEmergenceProjectEvidenceComplete: {}", yes_no_bool(plan.emergence.project_evidence_complete))?;
    writeln!(file, "CurrentEmergenceEvidenceContract: {}", plan.emergence.evidence_contract)?;
    writeln!(file, "CurrentEmergenceManifestExists: {}", yes_no_bool(plan.emergence.manifest_exists))?;
    writeln!(file, "CurrentEmergenceManifestCompleted: {}", yes_no_bool(plan.emergence.manifest_completed))?;
    writeln!(file, "CurrentEmergenceManifestValid: {}", yes_no_bool(plan.emergence.manifest_valid))?;
    writeln!(file, "CurrentEmergenceManifestReason: {}", plan.emergence.manifest_reason)?;
    writeln!(file, "CurrentEmergenceManifestProjectFiles: {}/{}", plan.emergence.manifest_project_files_valid, plan.emergence.manifest_project_files_expected)?;
    writeln!(file, "CurrentEmergenceReportHashValid: {}", yes_no_bool(plan.emergence.report_hash_valid))?;
    writeln!(file, "CurrentEmergenceReportBoundToManifest: {}", yes_no_bool(plan.emergence.report_bound_to_manifest))?;
    writeln!(file, "SelectedTargetProject: {}", plan.target_project.as_deref().unwrap_or("none"))?;
    writeln!(file, "SelectedTargetLocator: {}", plan.target_locator.as_deref().unwrap_or("n/a"))?;
    writeln!(file, "SelectedTargetScore: {}", plan.target_score.map(|value| value.to_string()).unwrap_or_else(|| "n/a".to_string()))?;
    writeln!(file, "SelectedTargetClass: {}", plan.target_class.as_str())?;
    writeln!(file, "SelectedTargetBehindBarsRuleCheckRequired: {}", yes_no_bool(plan.target_class.behind_bars_rule_check_required()))?;
    writeln!(file, "SelectedTargetRuleCandidate: {}", yes_no_bool(plan.target_class.rule_candidate()))?;
    writeln!(file, "SelectedTargetWriteCandidate: {}", yes_no_bool(plan.target_class.write_candidate()))?;
    writeln!(file, "SelectedTargetPermissionContract: {}", plan.target_class.permission_contract())?;
    writeln!(file, "ReferenceProject: {}", plan.reference_project)?;
    writeln!(file, "ReferenceRole: Regression/Beweisanker, nicht Operator-Ausnahme")?;
    writeln!(file, "Regel: generisch; Zielprojekt kommt aus Ranking-/Resultdaten, nicht aus einer Etuedennummer")?;
    writeln!(file, "Projekte: {}", if projects.is_empty() { "none".to_string() } else { projects.join(", ") })?;
    writeln!(file)?;
    writeln!(file, "PROJECT-RUNS")?;
    writeln!(file, "------------")?;
    writeln!(file, "Projekt | Exit | ResultFile | ResultStatus | ResultWerkzeug | Fresh")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    fn source_between<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
        let start_index = source.find(start).expect("start marker present");
        let end_index = source[start_index..]
            .find(end)
            .map(|offset| start_index + offset)
            .expect("end marker present");
        &source[start_index..end_index]
    }

    #[test]
    fn orchestrator_is_small_and_phase_only() {
        let launcher_source = include_str!("bin/etuschlopf_launcher.rs");
        let module_source = include_str!("visual_preflight.rs");
        let launcher_function_marker = ["fn execute_visual_preflight_", "run("].concat();
        let module_call_marker = ["visual_preflight::execute_visual_preflight_", "run"].concat();
        assert!(!launcher_source.contains(launcher_function_marker.as_str()));
        assert!(launcher_source.contains(module_call_marker.as_str()));

        let module_function_marker =
            ["pub(super) fn execute_visual_preflight_", "run("].concat();
        let prepare_marker = ["fn prepare_visual_preflight_", "session("].concat();
        let orchestrator = source_between(
            module_source,
            module_function_marker.as_str(),
            prepare_marker.as_str(),
        );
        assert!(orchestrator.lines().count() <= 40);
        for phase in [
            "prepare_visual_preflight_session",
            "no_seed::complete",
            "execute_visual_preflight_project_batch",
            "execute_visual_preflight_general_audits",
            "execute_visual_preflight_clef_audits",
            "finalize_visual_preflight_execution",
        ] {
            assert!(orchestrator.contains(phase), "missing phase {phase}");
        }
        assert!(!orchestrator.contains("File::create"));
        assert!(!orchestrator.contains("append_m18_"));
        assert!(!orchestrator.contains("create_assistant_export"));
    }

    #[test]
    fn phase_boundaries_are_typed_and_bounded() {
        let launcher_source =
            include_str!("bin/etuschlopf_launcher/launcher_visual_audit.rs");
        let module_source = include_str!("visual_preflight.rs");
        let no_seed_source = include_str!("visual_preflight/no_seed.rs");
        for type_name in [
            "struct VisualPreflightSession",
            "struct VisualPreflightProjectBatch",
            "struct VisualPreflightExecution",
        ] {
            assert!(module_source.contains(type_name), "missing typed boundary {type_name}");
        }
        for type_name in [
            "struct NoSeedFinalizationRequest",
            "struct NoSeedFinalizationResult",
        ] {
            assert!(
                no_seed_source.contains(type_name),
                "missing typed no-seed boundary {type_name}"
            );
        }
        let no_seed_function = source_between(
            no_seed_source,
            "pub(super) fn complete(",
            "Ok(NoSeedFinalizationResult",
        );
        assert!(no_seed_function.lines().count() < 500);
        for (start, end) in [
            (
                "fn prepare_visual_preflight_session(",
                "fn write_visual_preflight_header(",
            ),
            ("fn write_visual_preflight_header(", "Ok(())\n}"),
        ] {
            let function = source_between(module_source, start, end);
            assert!(
                function.lines().count() < 500,
                "module phase {start} grew beyond the Maulwurf boundary"
            );
        }
        let launcher_functions = [
            (
                "fn execute_visual_preflight_project_batch(",
                "fn execute_visual_preflight_general_audits(",
            ),
            (
                "fn execute_visual_preflight_general_audits(",
                "fn execute_visual_preflight_clef_audits(",
            ),
            (
                "fn execute_visual_preflight_clef_audits(",
                "fn finalize_visual_preflight_execution(",
            ),
            (
                "fn finalize_visual_preflight_execution(",
                "impl VisualProjectRunLedger",
            ),
        ];
        for (start, end) in launcher_functions {
            let function = source_between(launcher_source, start, end);
            assert!(
                function.lines().count() < 500,
                "launcher phase {start} grew beyond the Maulwurf boundary"
            );
        }
        let production_source = module_source
            .split("#[cfg(test)]")
            .next()
            .expect("production module source present");
        let no_seed_production_source = no_seed_source
            .split("#[cfg(test)]")
            .next()
            .expect("no-seed production module source present");
        let include_macro = ["include", "!("].concat();
        assert!(!production_source.contains(include_macro.as_str()));
        assert!(!no_seed_production_source.contains(include_macro.as_str()));
    }

    #[test]
    fn no_seed_finalization_is_owned_by_the_visual_preflight_module() {
        let launcher_source = include_str!("bin/etuschlopf_launcher.rs");
        let module_source = include_str!("visual_preflight.rs");
        let no_seed_source = include_str!("visual_preflight/no_seed.rs");
        let old_function_marker = ["fn complete_visual_preflight_no_", "seed("].concat();
        assert!(!launcher_source.contains(old_function_marker.as_str()));
        assert!(!module_source.contains(old_function_marker.as_str()));
        assert!(module_source.contains("#[path = \"visual_preflight/no_seed.rs\"]"));
        assert!(module_source.contains("mod no_seed;"));
        assert!(module_source.contains("no_seed::NoSeedFinalizationRequest::new"));
        assert!(module_source.contains("no_seed::complete(request)"));
        assert!(no_seed_source.contains("struct NoSeedFinalizationRequest"));
        assert!(no_seed_source.contains("struct NoSeedFinalizationResult"));
        let function = source_between(
            no_seed_source,
            "pub(super) fn complete(",
            "Ok(NoSeedFinalizationResult",
        );
        assert!(function.contains("AuditSignal::ExpectedNoSeed"));
        assert!(function.contains("create_assistant_export"));
        assert!(!function.contains("contains(\"NO-SEED\")"));
    }

    #[test]
    fn orchestration_uses_a_real_rust_module_boundary() {
        let launcher_source = include_str!("bin/etuschlopf_launcher.rs");
        let module_source = include_str!("visual_preflight.rs");
        let no_seed_source = include_str!("visual_preflight/no_seed.rs");
        let module_path = ["#[path = \"../", "visual_preflight.rs", "\"]"].concat();
        let include_shard = ["include!", "(\"visual_preflight"].concat();
        assert!(launcher_source.contains(module_path.as_str()));
        assert!(!launcher_source.contains(include_shard.as_str()));
        let module_function_marker =
            ["pub(super) fn execute_visual_preflight_", "run("].concat();
        assert!(module_source.contains(module_function_marker.as_str()));
        assert!(module_source.contains("use super::{"));
        let production_source = module_source
            .split("#[cfg(test)]")
            .next()
            .expect("production module source present");
        assert!(production_source.lines().count() < 250);
        assert!(no_seed_source.lines().count() < 250);
    }
}
