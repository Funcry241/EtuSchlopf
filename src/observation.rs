use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub const SESSION_ENV: &str = "ETUSCHLOPF_OBSERVATION_SESSION";
pub const SOURCE_ENV: &str = "ETUSCHLOPF_OBSERVATION_SOURCE";
pub const PARENT_SOURCE_ENV: &str = "ETUSCHLOPF_OBSERVATION_PARENT_SOURCE";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationSessionId(String);

impl ObservationSessionId {
    pub fn from_environment_or_new() -> Self {
        std::env::var(SESSION_ENV)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .map_or_else(Self::new_local, Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new_local() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(format!(
            "obs-{}-{now}-{}-{counter}",
            env!("CARGO_PKG_VERSION"),
            std::process::id()
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceId(String);

impl SourceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceRole {
    Launcher,
    Corpus,
    Project,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationSessionContext {
    session_id: ObservationSessionId,
    source_id: SourceId,
    parent_source_id: Option<SourceId>,
    role: SourceRole,
}

impl ObservationSessionContext {
    pub fn launcher_new() -> Self {
        Self {
            session_id: ObservationSessionId::new_local(),
            source_id: SourceId::new("launcher"),
            parent_source_id: None,
            role: SourceRole::Launcher,
        }
    }

    pub fn corpus_from_environment_or_new() -> Self {
        Self {
            session_id: ObservationSessionId::from_environment_or_new(),
            source_id: source_id_from_environment("corpus"),
            parent_source_id: parent_source_id_from_environment(),
            role: SourceRole::Corpus,
        }
    }

    pub fn project_from_environment_or_new() -> Self {
        Self {
            session_id: ObservationSessionId::from_environment_or_new(),
            source_id: source_id_from_environment("project:1"),
            parent_source_id: parent_source_id_from_environment(),
            role: SourceRole::Project,
        }
    }

    pub fn child_corpus(&self) -> Self {
        debug_assert_eq!(self.role, SourceRole::Launcher);
        Self {
            session_id: self.session_id.clone(),
            source_id: SourceId::new("corpus"),
            parent_source_id: Some(self.source_id.clone()),
            role: SourceRole::Corpus,
        }
    }

    pub fn child_project(&self, project_index: usize) -> Self {
        debug_assert!(matches!(self.role, SourceRole::Launcher | SourceRole::Corpus));
        Self {
            session_id: self.session_id.clone(),
            source_id: SourceId::new(format!("project:{project_index}")),
            parent_source_id: Some(self.source_id.clone()),
            role: SourceRole::Project,
        }
    }

    pub fn session_id(&self) -> &ObservationSessionId {
        &self.session_id
    }

    pub fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    pub fn parent_source_id(&self) -> Option<&SourceId> {
        self.parent_source_id.as_ref()
    }

    pub const fn role(&self) -> SourceRole {
        self.role
    }
}

fn source_id_from_environment(fallback: &str) -> SourceId {
    std::env::var(SOURCE_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map_or_else(|| SourceId::new(fallback), SourceId::new)
}

fn parent_source_id_from_environment() -> Option<SourceId> {
    std::env::var(PARENT_SOURCE_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(SourceId::new)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceStatus {
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceOutcomeColor {
    DarkGreen,
    LightGreen,
    Yellow,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherStage {
    Preparing,
    Building,
    LauncherTests,
    RunningTarget,
    Finalizing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectStage {
    ProjectRunPreparation,
    ArrangementSolver,
    MasterNotationAndSalience,
    InitialRenderTransaction,
    MetricBeamAndFinalNotation,
    ReleaseReportAndExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorpusStage {
    Planning,
    PreparingProjects,
    RunningProjects,
    CollectingProjectResult,
    FinalizingCorpusReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceStage {
    Launcher(LauncherStage),
    Project(ProjectStage),
    Corpus(CorpusStage),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceProgress {
    pub completed: usize,
    pub total: usize,
    pub active: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceState {
    pub context: ObservationSessionContext,
    pub revision: u64,
    pub status: SourceStatus,
    pub stage: SourceStage,
    pub project_label: Option<String>,
    pub progress: Option<SourceProgress>,
    pub outcome_color: Option<SourceOutcomeColor>,
}

#[derive(Debug)]
pub struct SourceStateOwner {
    state: SourceState,
    started_at: Instant,
    started_unix_millis: u64,
    terminal_duration_millis: Option<u64>,
}

impl SourceStateOwner {
    pub fn launcher() -> Self {
        Self::from_state(SourceState {
            context: ObservationSessionContext::launcher_new(),
            revision: 1,
            status: SourceStatus::Running,
            stage: SourceStage::Launcher(LauncherStage::Preparing),
            project_label: None,
            progress: None,
            outcome_color: None,
        })
    }

    pub fn project(project_label: Option<String>) -> Self {
        Self::from_state(SourceState {
            context: ObservationSessionContext::project_from_environment_or_new(),
            revision: 1,
            status: SourceStatus::Running,
            stage: SourceStage::Project(ProjectStage::ProjectRunPreparation),
            project_label,
            progress: None,
            outcome_color: None,
        })
    }

    pub fn corpus(total_projects: usize) -> Self {
        Self::from_state(SourceState {
            context: ObservationSessionContext::corpus_from_environment_or_new(),
            revision: 1,
            status: SourceStatus::Running,
            stage: SourceStage::Corpus(CorpusStage::Planning),
            project_label: None,
            progress: Some(SourceProgress {
                completed: 0,
                total: total_projects,
                active: 0,
            }),
            outcome_color: None,
        })
    }

    fn from_state(state: SourceState) -> Self {
        Self {
            state,
            started_at: Instant::now(),
            started_unix_millis: unix_millis_now(),
            terminal_duration_millis: None,
        }
    }

    #[cfg(test)]
    pub fn state(&self) -> &SourceState {
        &self.state
    }

    pub fn context(&self) -> &ObservationSessionContext {
        &self.state.context
    }

    pub(crate) fn state_for_snapshot_projection(&self) -> &SourceState {
        &self.state
    }

    pub(crate) const fn started_unix_millis_for_snapshot_projection(&self) -> u64 {
        self.started_unix_millis
    }

    pub(crate) const fn terminal_duration_millis_for_snapshot_projection(&self) -> Option<u64> {
        self.terminal_duration_millis
    }

    pub fn enter_launcher_stage(&mut self, stage: LauncherStage) {
        debug_assert_eq!(self.state.context.role(), SourceRole::Launcher);
        self.replace_semantic_state(
            SourceStatus::Running,
            SourceStage::Launcher(stage),
            None,
            None,
        );
    }

    pub fn enter_project_stage(&mut self, stage: ProjectStage) {
        debug_assert_eq!(self.state.context.role(), SourceRole::Project);
        debug_assert!(self.state.project_label.is_some());
        self.replace_semantic_state(
            SourceStatus::Running,
            SourceStage::Project(stage),
            self.state.progress,
            None,
        );
    }

    pub fn enter_corpus_stage(&mut self, stage: CorpusStage, progress: SourceProgress) {
        debug_assert_eq!(self.state.context.role(), SourceRole::Corpus);
        self.replace_semantic_state(
            SourceStatus::Running,
            SourceStage::Corpus(stage),
            Some(progress),
            None,
        );
    }

    pub fn mark_completed(&mut self) {
        self.replace_semantic_state(
            SourceStatus::Completed,
            self.state.stage,
            self.state.progress,
            None,
        );
    }

    pub fn mark_failed(&mut self) {
        self.replace_semantic_state(
            SourceStatus::Failed,
            self.state.stage,
            self.state.progress,
            None,
        );
    }

    pub fn mark_completed_with_outcome(&mut self, outcome_color: SourceOutcomeColor) {
        self.replace_semantic_state(
            SourceStatus::Completed,
            self.state.stage,
            self.state.progress,
            Some(outcome_color),
        );
    }

    pub fn mark_failed_with_outcome(&mut self, outcome_color: SourceOutcomeColor) {
        self.replace_semantic_state(
            SourceStatus::Failed,
            self.state.stage,
            self.state.progress,
            Some(outcome_color),
        );
    }

    fn replace_semantic_state(
        &mut self,
        status: SourceStatus,
        stage: SourceStage,
        progress: Option<SourceProgress>,
        outcome_color: Option<SourceOutcomeColor>,
    ) {
        if self.state.status == status
            && self.state.stage == stage
            && self.state.progress == progress
            && self.state.outcome_color == outcome_color
        {
            return;
        }
        if status != SourceStatus::Running && self.terminal_duration_millis.is_none() {
            self.terminal_duration_millis = Some(duration_millis_saturating(self.started_at.elapsed()));
        }
        self.state.status = status;
        self.state.stage = stage;
        self.state.progress = progress;
        self.state.outcome_color = outcome_color;
        self.state.revision = self.state.revision.saturating_add(1);
    }
}

fn unix_millis_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or(0)
}

fn duration_millis_saturating(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::{
        CorpusStage, LauncherStage, ObservationSessionContext, ProjectStage, SourceOutcomeColor,
        SourceProgress, SourceRole, SourceStage, SourceStateOwner, SourceStatus,
    };

    #[test]
    fn launcher_context_is_session_root_and_children_keep_session() {
        let launcher = ObservationSessionContext::launcher_new();
        let corpus = launcher.child_corpus();
        let project = launcher.child_project(1);

        assert_eq!(launcher.role(), SourceRole::Launcher);
        assert_eq!(launcher.source_id().as_str(), "launcher");
        assert_eq!(launcher.parent_source_id(), None);
        assert_eq!(corpus.session_id(), launcher.session_id());
        assert_eq!(corpus.source_id().as_str(), "corpus");
        assert_eq!(corpus.parent_source_id(), Some(launcher.source_id()));
        assert_eq!(project.session_id(), launcher.session_id());
        assert_eq!(project.source_id().as_str(), "project:1");
        assert_eq!(project.parent_source_id(), Some(launcher.source_id()));
    }

    #[test]
    fn launcher_revision_changes_only_on_semantic_transition() {
        let mut owner = SourceStateOwner::launcher();
        let initial_revision = owner.state().revision;

        owner.enter_launcher_stage(LauncherStage::Preparing);
        assert_eq!(owner.state().revision, initial_revision);

        owner.enter_launcher_stage(LauncherStage::Building);
        assert_eq!(owner.state().revision, initial_revision + 1);
        assert_eq!(
            owner.state().stage,
            SourceStage::Launcher(LauncherStage::Building)
        );
    }

    #[test]
    fn project_revision_changes_only_when_semantic_state_changes() {
        let mut owner = SourceStateOwner::project(Some("Synthetic Project Nr. 6".to_string()));
        let initial_revision = owner.state().revision;

        owner.enter_project_stage(ProjectStage::ProjectRunPreparation);
        assert_eq!(owner.state().revision, initial_revision);

        owner.enter_project_stage(ProjectStage::ArrangementSolver);
        assert_eq!(owner.state().revision, initial_revision + 1);
        assert_eq!(
            owner.state().stage,
            SourceStage::Project(ProjectStage::ArrangementSolver)
        );

        owner.mark_completed();
        assert_eq!(owner.state().status, SourceStatus::Completed);
        assert_eq!(owner.state().revision, initial_revision + 2);
        owner.mark_completed();
        assert_eq!(owner.state().revision, initial_revision + 2);
    }

    #[test]
    fn corpus_child_keeps_session_and_gets_unique_source_address() {
        let corpus = ObservationSessionContext::corpus_from_environment_or_new();
        let child = corpus.child_project(6);

        assert_eq!(child.session_id(), corpus.session_id());
        assert_eq!(child.source_id().as_str(), "project:6");
        assert_eq!(child.parent_source_id(), Some(corpus.source_id()));
    }

    #[test]
    fn corpus_progress_is_part_of_revision_semantics() {
        let mut owner = SourceStateOwner::corpus(10);
        let initial_revision = owner.state().revision;
        let progress = SourceProgress {
            completed: 0,
            total: 10,
            active: 2,
        };

        owner.enter_corpus_stage(CorpusStage::RunningProjects, progress);
        assert_eq!(owner.state().revision, initial_revision + 1);
        owner.enter_corpus_stage(CorpusStage::RunningProjects, progress);
        assert_eq!(owner.state().revision, initial_revision + 1);
    }
    #[test]
    fn terminal_transition_freezes_duration_and_accepts_typed_outcome_without_second_owner() {
        let mut owner = SourceStateOwner::launcher();
        owner.mark_completed_with_outcome(SourceOutcomeColor::DarkGreen);
        let terminal_duration = owner
            .terminal_duration_millis_for_snapshot_projection()
            .expect("terminal duration");
        assert_eq!(owner.state().outcome_color, Some(SourceOutcomeColor::DarkGreen));
        owner.mark_completed_with_outcome(SourceOutcomeColor::DarkGreen);
        assert_eq!(
            owner.terminal_duration_millis_for_snapshot_projection(),
            Some(terminal_duration)
        );
    }

}
