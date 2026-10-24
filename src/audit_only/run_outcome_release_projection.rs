//! Truthful launcher-owned release projection for typed run outcomes.
//!
//! The base evaluator intentionally remains unchanged. This audit-domain
//! boundary owns the user-visible four-colour contract:
//!
//! - dark green: completely clean successful run,
//! - light green: successful run with valid output and only the known external
//!   MuseScore process exit,
//! - yellow: a real unresolved/audit-blocked deviation,
//! - red: an actual failure or invalid/missing output.
//!
//! Process exit ownership and renderer evidence remain unchanged.

use crate::run_outcome::{
    AuditSignal, FailureReason, FailureStage, ReleaseDecision, RendererOutcome, RunOutcome,
    RunOutcomeProjection, RunSeverity,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutcomeColor {
    DarkGreen,
    LightGreen,
    Yellow,
    Red,
}

impl OutcomeColor {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DarkGreen => "dark_green",
            Self::LightGreen => "light_green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ProjectionDecision {
    Native,
    ReleasedLightGreenExternalRendererExit,
}

pub struct TruthfulRunOutcomeProjection {
    pub report: RunOutcomeProjection,
    pub severity: OutcomeColor,
    pub hint: Option<&'static str>,
}

pub fn truthful_run_outcome_projection(
    outcome: RunOutcome,
) -> TruthfulRunOutcomeProjection {
    let native_console = outcome.console_projection();
    let mut report = outcome.report_projection();
    let mut severity = match native_console.severity {
        RunSeverity::Green => OutcomeColor::DarkGreen,
        RunSeverity::Yellow => OutcomeColor::Yellow,
        RunSeverity::Red => OutcomeColor::Red,
    };
    let mut hint = native_console.hint;

    report.severity = severity.as_str();
    report.console_severity = severity.as_str();

    if projection_decision(outcome)
        == ProjectionDecision::ReleasedLightGreenExternalRendererExit
    {
        severity = OutcomeColor::LightGreen;
        report.severity = severity.as_str();
        report.release_decision =
            "released_external_renderer_exit_output_valid".to_string();
        report.console_status = "OK";
        report.console_severity = severity.as_str();
        hint = Some(
            "Hinweis: Ausgabe vollständig und gültig; nur bekannter externer MuseScore-Abgang - hellgrün, kein Wiederholungslauf oder Rollback.",
        );
    }

    TruthfulRunOutcomeProjection {
        report,
        severity,
        hint,
    }
}

fn projection_decision(outcome: RunOutcome) -> ProjectionDecision {
    match (outcome.release_decision, outcome.renderer) {
        (
            ReleaseDecision::Failed(FailureReason::Process {
                stage: FailureStage::Renderer,
                exit_code: failed_exit,
            }),
            RendererOutcome::OutputValidProcessFailed(renderer_exit),
        ) if outcome.final_exit == 0
            && failed_exit == renderer_exit
            && !matches!(
                outcome.audit_signal,
                AuditSignal::ExpectedNoSeed | AuditSignal::ExpectedBlocked
            ) => ProjectionDecision::ReleasedLightGreenExternalRendererExit,
        _ => ProjectionDecision::Native,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run_outcome::{
        CheckOutcome, EmergenceOutcome, FatalOutcome, RunMode, RunOutcomeInput,
    };

    fn base(final_exit: i32) -> RunOutcomeInput {
        RunOutcomeInput {
            mode: RunMode::NormalRun,
            fatal: FatalOutcome::NotObserved,
            final_exit,
            build: CheckOutcome::Passed,
            clippy: CheckOutcome::Passed,
            audit_signal: AuditSignal::NotEvaluated,
            renderer: RendererOutcome::NotObserved,
            emergence: EmergenceOutcome::NotApplicable,
        }
    }

    #[test]
    fn valid_renderer_output_with_final_exit_zero_is_light_green_and_released() {
        let mut input = base(0);
        input.renderer = RendererOutcome::OutputValidProcessFailed(-1_073_741_819);
        let projection = truthful_run_outcome_projection(RunOutcome::evaluate(input));
        assert_eq!(projection.severity, OutcomeColor::LightGreen);
        assert_eq!(projection.report.severity, "light_green");
        assert_eq!(
            projection.report.release_decision,
            "released_external_renderer_exit_output_valid"
        );
        assert_eq!(projection.report.console_status, "OK");
        assert_eq!(projection.report.console_severity, "light_green");
        assert_eq!(projection.report.final_exit, 0);
        assert_eq!(
            projection.report.renderer_outcome,
            "output_valid_process_failed_exit_-1073741819"
        );
        assert!(projection.hint.is_some());
    }

    #[test]
    fn output_valid_renderer_exit_two_keeps_native_yellow_audit_block() {
        let mut input = base(2);
        input.renderer = RendererOutcome::OutputValidProcessFailed(-1_073_741_819);
        let projection = truthful_run_outcome_projection(RunOutcome::evaluate(input));
        assert_eq!(projection.severity, OutcomeColor::Yellow);
        assert_eq!(projection.report.severity, "yellow");
        assert_eq!(
            projection.report.release_decision,
            "audit_blocked_renderer_output_valid_process_failed"
        );
        assert_eq!(projection.report.console_status, "AUDIT-BLOCKED");
    }

    #[test]
    fn true_renderer_process_failure_remains_red() {
        let mut input = base(0);
        input.renderer = RendererOutcome::ProcessFailed(-1_073_741_819);
        let projection = truthful_run_outcome_projection(RunOutcome::evaluate(input));
        assert_eq!(projection.severity, OutcomeColor::Red);
        assert_eq!(projection.report.severity, "red");
        assert_eq!(projection.report.console_status, "ERROR");
        assert_eq!(
            projection.report.release_decision,
            "failed_renderer_failed_exit_-1073741819"
        );
    }

    #[test]
    fn passed_renderer_with_final_exit_zero_is_dark_green() {
        let mut input = base(0);
        input.renderer = RendererOutcome::Passed;
        let projection = truthful_run_outcome_projection(RunOutcome::evaluate(input));
        assert_eq!(projection.severity, OutcomeColor::DarkGreen);
        assert_eq!(projection.report.severity, "dark_green");
        assert_eq!(projection.report.release_decision, "released");
        assert_eq!(projection.report.console_status, "OK");
        assert_eq!(projection.report.console_severity, "dark_green");
    }

    #[test]
    fn release_projection_does_not_hide_unresolved_audit_blocker() {
        let mut input = base(0);
        input.audit_signal = AuditSignal::ExpectedBlocked;
        input.renderer = RendererOutcome::OutputValidProcessFailed(-1_073_741_819);
        let projection = truthful_run_outcome_projection(RunOutcome::evaluate(input));
        assert_eq!(projection.severity, OutcomeColor::Red);
        assert_eq!(projection.report.console_status, "ERROR");
    }
}
