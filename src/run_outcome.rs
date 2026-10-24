//! Typed launcher run outcome model.
//!
//! Callers provide typed facts and this module evaluates the shared end state.
//! Console and assistant-export status projections are derived only from the
//! typed outcome. Process exit ownership remains unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunMode {
    VisualPreflight,
    AllRun,
    NormalRun,
}
impl RunMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VisualPreflight => "visual_preflight",
            Self::AllRun => "all_run",
            Self::NormalRun => "normal_run",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckOutcome {
    NotRun,
    Passed,
    Failed(i32),
    Unknown,
}
impl CheckOutcome {
    pub fn label(self) -> String {
        match self {
            Self::NotRun => "not_run".to_string(),
            Self::Passed => "passed".to_string(),
            Self::Failed(exit_code) => format!("failed_exit_{exit_code}"),
            Self::Unknown => "unknown".to_string(),
        }
    }
    pub fn exit_text(self) -> String {
        match self {
            Self::NotRun => "not_run".to_string(),
            Self::Passed => "0".to_string(),
            Self::Failed(exit_code) => exit_code.to_string(),
            Self::Unknown => "unknown".to_string(),
        }
    }
    fn failure(self, stage: FailureStage) -> Option<FailureReason> {
        match self {
            Self::Failed(exit_code) => Some(FailureReason::Process {
                stage,
                exit_code,
            }),
            Self::Unknown => Some(FailureReason::UnknownCheckState { stage }),
            Self::NotRun | Self::Passed => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuditSignal {
    NotEvaluated,
    ExpectedNoSeed,
    ExpectedBlocked,
    AcceptedClosureNormalized,
}
impl AuditSignal {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotEvaluated => "not_evaluated",
            Self::ExpectedNoSeed => "expected_no_seed",
            Self::ExpectedBlocked => "expected_audit_blocked",
            Self::AcceptedClosureNormalized => "accepted_closure_normalized",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendererOutcome {
    NotObserved,
    Passed,
    GuardBlocked,
    OutputValidProcessFailed(i32),
    ProcessFailed(i32),
}
impl RendererOutcome {
    pub fn label(self) -> String {
        match self {
            Self::NotObserved => "not_observed".to_string(),
            Self::Passed => "passed".to_string(),
            Self::GuardBlocked => "guard_blocked".to_string(),
            Self::OutputValidProcessFailed(exit_code) => {
                format!("output_valid_process_failed_exit_{exit_code}")
            }
            Self::ProcessFailed(exit_code) => format!("process_failed_exit_{exit_code}"),
        }
    }
    pub fn merge(self, next: Self) -> Self {
        if self.priority() >= next.priority() {
            self
        } else {
            next
        }
    }
    fn priority(self) -> u8 {
        match self {
            Self::NotObserved => 0,
            Self::Passed => 1,
            Self::GuardBlocked => 2,
            Self::OutputValidProcessFailed(_) => 3,
            Self::ProcessFailed(_) => 4,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmergenceOutcome {
    NotApplicable,
    NoCurrentTarget,
    CurrentTarget,
}
impl EmergenceOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::NoCurrentTarget => "no_current_target",
            Self::CurrentTarget => "current_target",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FatalOutcome {
    NotObserved,
    Panic,
}
impl FatalOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotObserved => "not_observed",
            Self::Panic => "panic",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunSeverity {
    Green,
    Yellow,
    Red,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsoleStatus {
    Ok,
    NoSeed,
    AuditBlocked,
    Error,
}
impl ConsoleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::NoSeed => "NO-SEED",
            Self::AuditBlocked => "AUDIT-BLOCKED",
            Self::Error => "ERROR",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConsoleProjection {
    pub status: ConsoleStatus,
    pub severity: RunSeverity,
    pub hint: Option<&'static str>,
}
impl ConsoleProjection {
    fn from_outcome(outcome: RunOutcome) -> Self {
        match outcome.release_decision {
            ReleaseDecision::Released => Self {
                status: ConsoleStatus::Ok,
                severity: RunSeverity::Green,
                hint: None,
            },
            ReleaseDecision::AuditBlocked(AuditBlockReason::NoSeed) => Self {
                status: ConsoleStatus::NoSeed,
                severity: RunSeverity::Yellow,
                hint: Some(
                    "Hinweis: erwarteter NO-SEED-Audit-Stop; kein Build-/Clippy-/Panic-Fehler.",
                ),
            },
            ReleaseDecision::AuditBlocked(_) => Self {
                status: ConsoleStatus::AuditBlocked,
                severity: RunSeverity::Yellow,
                hint: Some(
                    "Hinweis: erwarteter Audit-Stop; kein Build-/Clippy-/Panic-Fehler.",
                ),
            },
            ReleaseDecision::Failed(_) => Self {
                status: ConsoleStatus::Error,
                severity: RunSeverity::Red,
                hint: None,
            },
        }
    }
}
impl RunSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuditBlockReason {
    NoSeed,
    ExpectedAuditBlock,
    RendererOutputValidProcessFailed,
}
impl AuditBlockReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoSeed => "no_seed",
            Self::ExpectedAuditBlock => "expected_audit_block",
            Self::RendererOutputValidProcessFailed => {
                "renderer_output_valid_process_failed"
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureStage {
    Build,
    Clippy,
    Renderer,
    Program,
}
impl FailureStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Clippy => "clippy",
            Self::Renderer => "renderer",
            Self::Program => "program",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FailureReason {
    Process { stage: FailureStage, exit_code: i32 },
    UnknownCheckState { stage: FailureStage },
    InconsistentState,
    Panic,
}
impl FailureReason {
    pub fn label(self) -> String {
        match self {
            Self::Process { stage, exit_code } => {
                format!("{}_failed_exit_{exit_code}", stage.as_str())
            }
            Self::UnknownCheckState { stage } => {
                format!("{}_state_unknown", stage.as_str())
            }
            Self::InconsistentState => "inconsistent_typed_state".to_string(),
            Self::Panic => "panic".to_string(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReleaseDecision {
    Released,
    AuditBlocked(AuditBlockReason),
    Failed(FailureReason),
}
impl ReleaseDecision {
    pub fn label(self) -> String {
        match self {
            Self::Released => "released".to_string(),
            Self::AuditBlocked(reason) => format!("audit_blocked_{}", reason.as_str()),
            Self::Failed(reason) => format!("failed_{}", reason.label()),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunOutcomeInput {
    pub mode: RunMode,
    pub fatal: FatalOutcome,
    pub final_exit: i32,
    pub build: CheckOutcome,
    pub clippy: CheckOutcome,
    pub audit_signal: AuditSignal,
    pub renderer: RendererOutcome,
    pub emergence: EmergenceOutcome,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunOutcome {
    pub mode: RunMode,
    pub fatal: FatalOutcome,
    pub final_exit: i32,
    pub build: CheckOutcome,
    pub clippy: CheckOutcome,
    pub audit_signal: AuditSignal,
    pub renderer: RendererOutcome,
    pub emergence: EmergenceOutcome,
    pub severity: RunSeverity,
    pub release_decision: ReleaseDecision,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunOutcomeProjection {
    pub mode: &'static str,
    pub fatal: &'static str,
    pub final_exit: i32,
    pub build_outcome: String,
    pub build_exit: String,
    pub clippy_outcome: String,
    pub clippy_exit: String,
    pub audit_signal: &'static str,
    pub renderer_outcome: String,
    pub emergence_outcome: &'static str,
    pub severity: &'static str,
    pub release_decision: String,
    pub console_status: &'static str,
    pub console_severity: &'static str,
}
impl RunOutcomeProjection {
    pub fn audit_text(&self, version: &str) -> String {
        format!(
            "ETUSCHLOPF-RUN-OUTCOME-AUDIT\n\
================================\n\
Version: {version}\n\
Mode: {}\n\
FatalOutcome: {}\n\
IntegrationMode: typed_run_outcome_export_projection_owner\n\
IntegrationScope: console_visual_summary_manifest_context_and_run_audit\n\
PanicCapture: typed_state_existing_panic_hook_authoritative\n\
TypedOutcomeOwnsConsoleProjection: ja\n\
TypedOutcomeOwnsVisualPreflightSummaryStatus: ja\n\
TypedOutcomeOwnsAssistantContextStatus: ja\n\
TypedOutcomeOwnsAssistantManifestStatus: ja\n\
TypedOutcomeOwnsRunAuditStatus: ja\n\
TypedOutcomeOwnsExitCode: nein\n\
RunSeverity: {}\n\
ReleaseDecision: {}\n\
ConsoleProjectionStatus: {}\n\
ConsoleProjectionSeverity: {}\n\
FinalExit: {}\n\
BuildOutcome: {}\n\
BuildExit: {}\n\
ClippyOutcome: {}\n\
ClippyExit: {}\n\
AuditSignal: {}\n\
RendererOutcome: {}\n\
EmergenceOutcome: {}\n\
Status: typed_projection_recorded\n\
TextHeuristicUsedByRunOutcomeModel: nein\n\
LegacyConsoleClassificationPresent: nein\n\
Policy: One typed RunOutcome projection owns console, visual-preflight summary, assistant context, assistant manifest and run-audit status fields. Process exit ownership remains unchanged.\n",
            self.mode,
            self.fatal,
            self.severity,
            self.release_decision,
            self.console_status,
            self.console_severity,
            self.final_exit,
            self.build_outcome,
            self.build_exit,
            self.clippy_outcome,
            self.clippy_exit,
            self.audit_signal,
            self.renderer_outcome,
            self.emergence_outcome,
        )
    }
}
impl RunOutcome {
    pub fn evaluate(input: RunOutcomeInput) -> Self {
        let release_decision = if input.fatal == FatalOutcome::Panic {
            ReleaseDecision::Failed(FailureReason::Panic)
        } else if let Some(reason) = input.build.failure(FailureStage::Build) {
            ReleaseDecision::Failed(reason)
        } else if let Some(reason) = input.clippy.failure(FailureStage::Clippy) {
            ReleaseDecision::Failed(reason)
        } else if let RendererOutcome::OutputValidProcessFailed(exit_code) = input.renderer {
            if input.final_exit == 2 {
                ReleaseDecision::AuditBlocked(
                    AuditBlockReason::RendererOutputValidProcessFailed,
                )
            } else {
                ReleaseDecision::Failed(FailureReason::Process {
                    stage: FailureStage::Renderer,
                    exit_code,
                })
            }
        } else if let RendererOutcome::ProcessFailed(exit_code) = input.renderer {
            ReleaseDecision::Failed(FailureReason::Process {
                stage: FailureStage::Renderer,
                exit_code,
            })
        } else if input.final_exit == 0 {
            let unresolved_blocker = input.renderer == RendererOutcome::GuardBlocked
                || matches!(
                    input.audit_signal,
                    AuditSignal::ExpectedNoSeed | AuditSignal::ExpectedBlocked
                );
            if unresolved_blocker {
                ReleaseDecision::Failed(FailureReason::InconsistentState)
            } else {
                ReleaseDecision::Released
            }
        } else if input.final_exit == 2 {
            if input.renderer == RendererOutcome::GuardBlocked {
                ReleaseDecision::AuditBlocked(AuditBlockReason::ExpectedAuditBlock)
            } else {
                match input.audit_signal {
                    AuditSignal::ExpectedNoSeed
                        if input.emergence == EmergenceOutcome::NoCurrentTarget =>
                    {
                        ReleaseDecision::AuditBlocked(AuditBlockReason::NoSeed)
                    }
                    AuditSignal::ExpectedNoSeed
                    | AuditSignal::AcceptedClosureNormalized => {
                        ReleaseDecision::Failed(FailureReason::InconsistentState)
                    }
                    AuditSignal::ExpectedBlocked => {
                        ReleaseDecision::AuditBlocked(AuditBlockReason::ExpectedAuditBlock)
                    }
                    AuditSignal::NotEvaluated => {
                        ReleaseDecision::Failed(FailureReason::Process {
                            stage: FailureStage::Program,
                            exit_code: input.final_exit,
                        })
                    }
                }
            }
        } else if input.audit_signal == AuditSignal::AcceptedClosureNormalized {
            ReleaseDecision::Failed(FailureReason::InconsistentState)
        } else {
            ReleaseDecision::Failed(FailureReason::Process {
                stage: FailureStage::Program,
                exit_code: input.final_exit,
            })
        };
        let severity = match release_decision {
            ReleaseDecision::Released => RunSeverity::Green,
            ReleaseDecision::AuditBlocked(_) => RunSeverity::Yellow,
            ReleaseDecision::Failed(_) => RunSeverity::Red,
        };
        Self {
            mode: input.mode,
            fatal: input.fatal,
            final_exit: input.final_exit,
            build: input.build,
            clippy: input.clippy,
            audit_signal: input.audit_signal,
            renderer: input.renderer,
            emergence: input.emergence,
            severity,
            release_decision,
        }
    }
    pub fn console_projection(self) -> ConsoleProjection {
        ConsoleProjection::from_outcome(self)
    }
    pub fn report_projection(self) -> RunOutcomeProjection {
        let console_projection = self.console_projection();
        RunOutcomeProjection {
            mode: self.mode.as_str(),
            fatal: self.fatal.as_str(),
            final_exit: self.final_exit,
            build_outcome: self.build.label(),
            build_exit: self.build.exit_text(),
            clippy_outcome: self.clippy.label(),
            clippy_exit: self.clippy.exit_text(),
            audit_signal: self.audit_signal.as_str(),
            renderer_outcome: self.renderer.label(),
            emergence_outcome: self.emergence.as_str(),
            severity: self.severity.as_str(),
            release_decision: self.release_decision.label(),
            console_status: console_projection.status.as_str(),
            console_severity: console_projection.severity.as_str(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn base(final_exit: i32) -> RunOutcomeInput {
        RunOutcomeInput {
            mode: RunMode::VisualPreflight,
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
    fn released_run_is_green() {
        let outcome = RunOutcome::evaluate(base(0));
        assert_eq!(outcome.severity, RunSeverity::Green);
        assert_eq!(outcome.release_decision, ReleaseDecision::Released);
    }
    #[test]
    fn expected_no_seed_exit_two_is_yellow() {
        let mut input = base(2);
        input.audit_signal = AuditSignal::ExpectedNoSeed;
        input.emergence = EmergenceOutcome::NoCurrentTarget;
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Yellow);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::AuditBlocked(AuditBlockReason::NoSeed)
        );
    }
    #[test]
    fn expected_audit_block_exit_two_is_yellow() {
        let mut input = base(2);
        input.audit_signal = AuditSignal::ExpectedBlocked;
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Yellow);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::AuditBlocked(AuditBlockReason::ExpectedAuditBlock)
        );
    }
    #[test]
    fn renderer_guard_is_yellow_even_without_legacy_text_signal() {
        let mut input = base(2);
        input.renderer = RendererOutcome::GuardBlocked;
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Yellow);
    }
    #[test]
    fn accepted_closure_normalization_is_green_at_exit_zero() {
        let mut input = base(0);
        input.audit_signal = AuditSignal::AcceptedClosureNormalized;
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Green);
        assert_eq!(outcome.release_decision, ReleaseDecision::Released);
    }
    #[test]
    fn unresolved_audit_blocker_cannot_be_green_at_exit_zero() {
        let mut input = base(0);
        input.audit_signal = AuditSignal::ExpectedBlocked;
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Red);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::Failed(FailureReason::InconsistentState)
        );
    }
    #[test]
    fn no_seed_requires_no_current_emergence_target() {
        let mut input = base(2);
        input.audit_signal = AuditSignal::ExpectedNoSeed;
        input.emergence = EmergenceOutcome::CurrentTarget;
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Red);
    }
    #[test]
    fn unexplained_exit_two_is_red() {
        let outcome = RunOutcome::evaluate(base(2));
        assert_eq!(outcome.severity, RunSeverity::Red);
    }
    #[test]
    fn panic_is_an_explicit_red_end_state() {
        let mut input = base(0);
        input.fatal = FatalOutcome::Panic;
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Red);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::Failed(FailureReason::Panic)
        );
    }
    #[test]
    fn build_failure_is_red_even_when_final_exit_is_zero() {
        let mut input = base(0);
        input.build = CheckOutcome::Failed(101);
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Red);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::Failed(FailureReason::Process {
                stage: FailureStage::Build,
                exit_code: 101,
            })
        );
    }
    #[test]
    fn clippy_failure_is_red_even_when_final_exit_is_zero() {
        let mut input = base(0);
        input.clippy = CheckOutcome::Failed(1);
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Red);
    }
    #[test]
    fn renderer_merge_keeps_the_most_severe_observation() {
        assert_eq!(
            RendererOutcome::ProcessFailed(9).merge(RendererOutcome::Passed),
            RendererOutcome::ProcessFailed(9)
        );
        assert_eq!(
            RendererOutcome::Passed.merge(RendererOutcome::GuardBlocked),
            RendererOutcome::GuardBlocked
        );
        assert_eq!(
            RendererOutcome::NotObserved.merge(RendererOutcome::Passed),
            RendererOutcome::Passed
        );
    }
    #[test]
    fn valid_renderer_output_with_failed_process_and_exit_two_is_yellow() {
        let mut input = base(2);
        input.renderer = RendererOutcome::OutputValidProcessFailed(-1_073_741_819);
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Yellow);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::AuditBlocked(
                AuditBlockReason::RendererOutputValidProcessFailed,
            )
        );
    }
    #[test]
    fn valid_renderer_output_does_not_hide_unexpected_program_exit() {
        let mut input = base(7);
        input.renderer = RendererOutcome::OutputValidProcessFailed(-1_073_741_819);
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Red);
    }
    #[test]
    fn renderer_process_failure_is_red_even_with_expected_audit_signal() {
        let mut input = base(2);
        input.audit_signal = AuditSignal::ExpectedBlocked;
        input.renderer = RendererOutcome::ProcessFailed(2);
        let outcome = RunOutcome::evaluate(input);
        assert_eq!(outcome.severity, RunSeverity::Red);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::Failed(FailureReason::Process {
                stage: FailureStage::Renderer,
                exit_code: 2,
            })
        );
    }
    #[test]
    fn audit_records_authoritative_shared_status_projection() {
        let mut input = base(2);
        input.audit_signal = AuditSignal::ExpectedNoSeed;
        input.emergence = EmergenceOutcome::NoCurrentTarget;
        let projection = RunOutcome::evaluate(input).report_projection();
        let audit = projection.audit_text("0.10.test");
        assert!(audit.contains("TypedOutcomeOwnsConsoleProjection: ja"));
        assert!(audit.contains("TypedOutcomeOwnsVisualPreflightSummaryStatus: ja"));
        assert!(audit.contains("TypedOutcomeOwnsAssistantContextStatus: ja"));
        assert!(audit.contains("TypedOutcomeOwnsAssistantManifestStatus: ja"));
        assert!(audit.contains("TypedOutcomeOwnsRunAuditStatus: ja"));
        assert!(audit.contains("ConsoleProjectionStatus: NO-SEED"));
        assert!(audit.contains("ReleaseDecision: audit_blocked_no_seed"));
        assert!(audit.contains("BuildExit: 0"));
        assert!(audit.contains("ClippyExit: 0"));
        assert!(audit.contains("LegacyConsoleClassificationPresent: nein"));
        assert!(audit.contains("TextHeuristicUsedByRunOutcomeModel: nein"));
    }
    #[test]
    fn report_projection_is_the_single_export_status_surface() {
        let mut input = base(2);
        input.audit_signal = AuditSignal::ExpectedNoSeed;
        input.emergence = EmergenceOutcome::NoCurrentTarget;
        let projection = RunOutcome::evaluate(input).report_projection();
        assert_eq!(projection.final_exit, 2);
        assert_eq!(projection.build_exit, "0");
        assert_eq!(projection.clippy_exit, "0");
        assert_eq!(projection.severity, "yellow");
        assert_eq!(projection.release_decision, "audit_blocked_no_seed");
        assert_eq!(projection.console_status, "NO-SEED");
        assert_eq!(projection.console_severity, "yellow");
    }
    #[test]
    fn console_projection_maps_release_decisions_without_text_heuristics() {
        let released = RunOutcome::evaluate(base(0)).console_projection();
        assert_eq!(released.status, ConsoleStatus::Ok);
        assert_eq!(released.severity, RunSeverity::Green);
        assert!(released.hint.is_none());
        let mut no_seed_input = base(2);
        no_seed_input.audit_signal = AuditSignal::ExpectedNoSeed;
        no_seed_input.emergence = EmergenceOutcome::NoCurrentTarget;
        let no_seed = RunOutcome::evaluate(no_seed_input).console_projection();
        assert_eq!(no_seed.status, ConsoleStatus::NoSeed);
        assert_eq!(no_seed.severity, RunSeverity::Yellow);
        assert!(no_seed.hint.is_some());
        let mut audit_input = base(2);
        audit_input.audit_signal = AuditSignal::ExpectedBlocked;
        let audit = RunOutcome::evaluate(audit_input).console_projection();
        assert_eq!(audit.status, ConsoleStatus::AuditBlocked);
        assert_eq!(audit.severity, RunSeverity::Yellow);
        let error = RunOutcome::evaluate(base(7)).console_projection();
        assert_eq!(error.status, ConsoleStatus::Error);
        assert_eq!(error.severity, RunSeverity::Red);
    }
    #[test]
    fn run_modes_cover_all_console_summary_paths() {
        assert_eq!(RunMode::VisualPreflight.as_str(), "visual_preflight");
        assert_eq!(RunMode::AllRun.as_str(), "all_run");
        assert_eq!(RunMode::NormalRun.as_str(), "normal_run");
    }
    #[test]
    fn regular_program_failure_is_red() {
        let outcome = RunOutcome::evaluate(base(7));
        assert_eq!(outcome.severity, RunSeverity::Red);
        assert_eq!(
            outcome.release_decision,
            ReleaseDecision::Failed(FailureReason::Process {
                stage: FailureStage::Program,
                exit_code: 7,
            })
        );
    }
}
