//! No-seed finalization phase for the visual preflight.
//!
//! This module owns the current no-target production path: console guidance,
//! result-file completion, typed run-state finalization, and assistant export.
//! It does not select targets, mutate music, or reinterpret report text.

use std::fs::OpenOptions;
use std::io::Write;

use super::VisualPreflightSession;
use crate::run_outcome::AuditSignal;
use crate::{
    append_run_diagnostic, assistant_context_renderer_state_from_runtime,
    create_assistant_export, AppResult, AssistantExportBundleContext, AssistantExportRequest,
    AssistantExportRunStatus, Attention, Emphasis, Launcher, PresentationRole,
    RunState, VisualProjectRunLedger,
};

pub(super) struct NoSeedFinalizationRequest<'a> {
    launcher: &'a mut Launcher,
    state: &'a mut RunState,
    session: &'a VisualPreflightSession,
}

impl<'a> NoSeedFinalizationRequest<'a> {
    pub(super) fn new(
        launcher: &'a mut Launcher,
        state: &'a mut RunState,
        session: &'a VisualPreflightSession,
    ) -> Self {
        Self {
            launcher,
            state,
            session,
        }
    }
}

pub(super) struct NoSeedFinalizationResult {
    exit_code: i32,
}

impl NoSeedFinalizationResult {
    pub(super) fn exit_code(self) -> i32 {
        self.exit_code
    }
}

pub(super) fn complete(
    request: NoSeedFinalizationRequest<'_>,
) -> AppResult<NoSeedFinalizationResult> {
    let NoSeedFinalizationRequest {
        launcher,
        state,
        session,
    } = request;
    let project_root = session.project_root.as_path();
    let plan = &session.plan;
    let projects = session.projects.as_slice();
    let aggregate_path = session.aggregate_path.as_path();
    let fresh_corpus_without_target = plan.emergence.fresh_for_current_version;
    let reference_renderer_audit = assistant_context_renderer_state_from_runtime(
        project_root,
        plan.reference_project.as_str(),
        &launcher.version,
    );
    let reference_renderer_audit_complete = reference_renderer_audit.has_usable_reference_evidence();
    launcher.line(
        PresentationRole::Result,
        None,
        Attention::Notice,
        Emphasis::Secondary,
        "",
    );
    launcher.line(
        PresentationRole::Result,
        None,
        Attention::Notice,
        Emphasis::Secondary,
        &format!(
            "[EtuSchlopf v{}] Visual-Preflight: kein aktuelles Emergenzziel. Status={} Grund={}. Build/Clippy sind verwendbar; kein Ziel aus Einzelresultaten ableiten.",
            launcher.version, plan.emergence.status, plan.emergence.reason
        ),
    );
    launcher.line(
        PresentationRole::Result,
        None,
        Attention::Notice,
        Emphasis::Secondary,
        if fresh_corpus_without_target {
            "[EtuSchlopf visual-preflight] Frischer 10er-Korpus ist belegt. Die automatische Zielauswahl ist pensioniert; musikalische Arbeit nur aus frischer PDF-/MusicXML-Evidenz oder konkretem Nutzerbefund."
        } else if reference_renderer_audit_complete {
            "[EtuSchlopf visual-preflight] Referenz-Renderer-Audit ist vollständig; kein --all für diesen Befund nötig. Ein Vollkorpus dient nur der aktuellen Zustandsprüfung, nicht der automatischen Zielwahl."
        } else {
            "[EtuSchlopf visual-preflight] Kein automatisch gewähltes Ziel. --all ist nur für einen frischen Korpus-Zustandsnachweis nötig; Build-/Clippy-/Renderer-Audits benötigen keinen Vollkorpus."
        },
    );
    let mut file = OpenOptions::new().append(true).open(aggregate_path)?;
    writeln!(
        file,
        "kein_projekt | 2 | n/a | {} | n/a | nein",
        if fresh_corpus_without_target {
            "fresh_full_corpus_without_automatic_target_selection"
        } else {
            "no_current_target_awaiting_fresh_full_corpus_emergence"
        }
    )?;
    writeln!(file)?;
    writeln!(file, "VISUAL-PREFLIGHT-FRESH-RUN-GATE")?;
    writeln!(file, "-------------------------------")?;
    writeln!(file, "ProjectRows: 0")?;
    writeln!(file, "FreshRows: 0")?;
    writeln!(file, "StaleOrMissingRows: 0")?;
    writeln!(file, "RunLedgerComplete: nein")?;
    writeln!(
        file,
        "Gate: {}",
        if fresh_corpus_without_target {
            "yellow_fresh_full_corpus_without_automatic_target_selection"
        } else {
            "yellow_no_fresh_full_corpus_emergence_target"
        }
    )?;
    writeln!(file)?;
    writeln!(file, "VISUAL-PREFLIGHT-SEED-FALLBACK-AND-GUIDANCE")?;
    writeln!(file, "--------------------------------------------")?;
    writeln!(file, "Status: {}", plan.emergence.status)?;
    writeln!(file, "CauseClass: {}", plan.emergence.reason)?;
    writeln!(file, "MusicMutation: nein")?;
    writeln!(file, "TargetProjectHardcoded: nein")?;
    writeln!(file, "ProjectIdUsedInPredicate: nein")?;
    writeln!(
        file,
        "Interpretation: {}",
        if fresh_corpus_without_target {
            "Ein frischer vollständiger 10er-Korpus ist belegt. Die automatische Zielauswahl ist abgeschafft; der Korpus bestätigt den Zustand, erzeugt aber keinen Reparaturauftrag."
        } else if reference_renderer_audit_complete {
            "Ohne frischen vollständigen 10er-Korpus fehlt nur der aktuelle Korpus-Zustandsnachweis. Der unabhängige Referenz-Renderer-Audit bleibt dennoch vollständig bewertbar."
        } else {
            "Ohne frischen vollständigen 10er-Korpus fehlt der aktuelle Korpus-Zustandsnachweis. Ein Vollkorpus dient nicht der automatischen Neuwahl eines Ziels und ist für technische Einzel-Audits nicht erforderlich."
        }
    )?;
    if fresh_corpus_without_target {
        writeln!(file, "RecommendedAction: review_fresh_full_corpus_current_facts_without_automatic_target")?;
    } else if reference_renderer_audit_complete {
        writeln!(file, "RecommendedAction: reference_renderer_audit_complete_no_full_corpus_required")?;
        writeln!(file, "ReferenceRendererOutputStatus: {}", reference_renderer_audit.renderer_output_status)?;
        writeln!(file, "ReferenceRendererProcessStatus: {}", reference_renderer_audit.renderer_process_status)?;
        writeln!(file, "ReferenceRendererOverallStatus: {}", reference_renderer_audit.renderer_overall_status)?;
    } else {
        writeln!(file, "RecommendedAction: no_current_target_full_corpus_only_for_current_state_evidence")?;
    }
    writeln!(file, "AssistantPolicy: Historische Reports und accepted closures sind Archivbelege, nie Zielwahl, Reparaturauftrag oder Musik-Praedikat.")?;
    state.input_arg = Some("--visual-preflight".to_string());
    state.result_file = Some(session.aggregate_path.clone());
    state.audit_signal = AuditSignal::ExpectedNoSeed;
    state.run_exit = "2".to_string();
    state.program_ms = "0".to_string();
    append_run_diagnostic(state, "visual_preflight")?;

    let strict_log_for_export = state.strict_log.clone();
    let clippy_exit_for_export = state.strict_clippy_exit.clone();
    let build_exit_for_export = state.build_exit.clone();
    let no_seed_ledgers: Vec<VisualProjectRunLedger> = Vec::new();
    let export_context = AssistantExportBundleContext {
        project_root,
        aggregate_path,
        plan,
        projects,
        ledgers: &no_seed_ledgers,
        strict_log: strict_log_for_export.as_deref(),
        strict_test_log: state.strict_test_log.as_deref(),
        version: &launcher.version,
    };
    let export_status = AssistantExportRunStatus {
        final_exit: 2,
        clippy_exit: &clippy_exit_for_export,
        build_exit: &build_exit_for_export,
        strict_test_exit: &state.strict_test_exit,
        strict_test_summary: &state.strict_test_summary,
        audit_signal: state.audit_signal,
        renderer_outcome: state.renderer_outcome,
        emergence_outcome: state.emergence_outcome,
        export_policy: state.export_policy,
    };
    create_assistant_export(AssistantExportRequest::VisualPreflight {
        context: export_context,
        status: export_status,
    })?;
    launcher.line(
        PresentationRole::Result,
        None,
        Attention::Notice,
        Emphasis::Secondary,
        "[EtuSchlopf assistant-export] Frischer assistant_export_latest.zip fuer No-Seed-Zustand geschrieben.",
    );
    Ok(NoSeedFinalizationResult { exit_code: 2 })
}
