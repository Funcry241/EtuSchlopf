use crate::observation::{
    CorpusStage, LauncherStage, ProjectStage, SourceOutcomeColor, SourceProgress, SourceRole,
    SourceStage, SourceStateOwner, SourceStatus,
};
use crate::observation_protocol::{
    SnapshotOutcomeColorV2, SnapshotProgressV2, SnapshotSourceRoleV2, SnapshotSourceStatusV2,
    SnapshotStageV2, SourceSnapshotV2, SOURCE_SNAPSHOT_PROTOCOL_VERSION,
};

pub fn project_source_snapshot_v2(owner: &SourceStateOwner) -> SourceSnapshotV2 {
    let state = owner.state_for_snapshot_projection();
    SourceSnapshotV2 {
        protocol_version: SOURCE_SNAPSHOT_PROTOCOL_VERSION,
        observation_session_id: state.context.session_id().as_str().to_string(),
        source_id: state.context.source_id().as_str().to_string(),
        parent_source_id: state
            .context
            .parent_source_id()
            .map(|source_id| source_id.as_str().to_string()),
        source_role: map_role(state.context.role()),
        revision: state.revision,
        status: map_status(state.status),
        stage: map_stage(state.stage),
        activity: None,
        project_label: state.project_label.clone(),
        progress: state.progress.map(map_progress),
        started_unix_millis: owner.started_unix_millis_for_snapshot_projection(),
        terminal_duration_millis: owner.terminal_duration_millis_for_snapshot_projection(),
        outcome_color: state.outcome_color.map(map_outcome_color),
    }
}

fn map_role(role: SourceRole) -> SnapshotSourceRoleV2 {
    match role {
        SourceRole::Launcher => SnapshotSourceRoleV2::Launcher,
        SourceRole::Corpus => SnapshotSourceRoleV2::Corpus,
        SourceRole::Project => SnapshotSourceRoleV2::Project,
    }
}

fn map_status(status: SourceStatus) -> SnapshotSourceStatusV2 {
    match status {
        SourceStatus::Running => SnapshotSourceStatusV2::Running,
        SourceStatus::Completed => SnapshotSourceStatusV2::Completed,
        SourceStatus::Failed => SnapshotSourceStatusV2::Failed,
    }
}

fn map_outcome_color(color: SourceOutcomeColor) -> SnapshotOutcomeColorV2 {
    match color {
        SourceOutcomeColor::DarkGreen => SnapshotOutcomeColorV2::DarkGreen,
        SourceOutcomeColor::LightGreen => SnapshotOutcomeColorV2::LightGreen,
        SourceOutcomeColor::Yellow => SnapshotOutcomeColorV2::Yellow,
        SourceOutcomeColor::Red => SnapshotOutcomeColorV2::Red,
    }
}

fn map_stage(stage: SourceStage) -> SnapshotStageV2 {
    match stage {
        SourceStage::Launcher(LauncherStage::Preparing) => SnapshotStageV2::LauncherPreparing,
        SourceStage::Launcher(LauncherStage::Building) => SnapshotStageV2::LauncherBuilding,
        SourceStage::Launcher(LauncherStage::LauncherTests) => SnapshotStageV2::LauncherTests,
        SourceStage::Launcher(LauncherStage::RunningTarget) => SnapshotStageV2::LauncherRunningTarget,
        SourceStage::Launcher(LauncherStage::Finalizing) => SnapshotStageV2::LauncherFinalizing,
        SourceStage::Project(ProjectStage::ProjectRunPreparation) => {
            SnapshotStageV2::ProjectRunPreparation
        }
        SourceStage::Project(ProjectStage::ArrangementSolver) => {
            SnapshotStageV2::ProjectArrangementSolver
        }
        SourceStage::Project(ProjectStage::MasterNotationAndSalience) => {
            SnapshotStageV2::ProjectMasterNotationAndSalience
        }
        SourceStage::Project(ProjectStage::InitialRenderTransaction) => {
            SnapshotStageV2::ProjectInitialRenderTransaction
        }
        SourceStage::Project(ProjectStage::MetricBeamAndFinalNotation) => {
            SnapshotStageV2::ProjectMetricBeamAndFinalNotation
        }
        SourceStage::Project(ProjectStage::ReleaseReportAndExit) => {
            SnapshotStageV2::ProjectReleaseReportAndExit
        }
        SourceStage::Corpus(CorpusStage::Planning) => SnapshotStageV2::CorpusPlanning,
        SourceStage::Corpus(CorpusStage::PreparingProjects) => {
            SnapshotStageV2::CorpusPreparingProjects
        }
        SourceStage::Corpus(CorpusStage::RunningProjects) => SnapshotStageV2::CorpusRunningProjects,
        SourceStage::Corpus(CorpusStage::CollectingProjectResult) => {
            SnapshotStageV2::CorpusCollectingProjectResult
        }
        SourceStage::Corpus(CorpusStage::FinalizingCorpusReport) => {
            SnapshotStageV2::CorpusFinalizingCorpusReport
        }
    }
}

fn map_progress(progress: SourceProgress) -> SnapshotProgressV2 {
    SnapshotProgressV2 {
        completed: u64::try_from(progress.completed).unwrap_or(u64::MAX),
        total: u64::try_from(progress.total).unwrap_or(u64::MAX),
        active: u64::try_from(progress.active).unwrap_or(u64::MAX),
    }
}

#[cfg(test)]
mod tests {
    use super::project_source_snapshot_v2;
    use crate::observation::{ProjectStage, SourceStateOwner};
    use crate::observation_protocol::{SnapshotSourceRoleV2, SnapshotStageV2, SOURCE_SNAPSHOT_PROTOCOL_VERSION};

    #[test]
    fn project_snapshot_is_projection_not_internal_state_alias() {
        let mut owner = SourceStateOwner::project(Some("Synthetic Project Nr. 6".to_string()));
        owner.enter_project_stage(ProjectStage::InitialRenderTransaction);
        let snapshot = project_source_snapshot_v2(&owner);

        assert_eq!(snapshot.protocol_version, SOURCE_SNAPSHOT_PROTOCOL_VERSION);
        assert_eq!(snapshot.source_role, SnapshotSourceRoleV2::Project);
        assert_eq!(snapshot.stage, SnapshotStageV2::ProjectInitialRenderTransaction);
        assert_eq!(snapshot.project_label.as_deref(), Some("Synthetic Project Nr. 6"));
        assert_eq!(snapshot.activity, None);
        assert!(snapshot.started_unix_millis > 0);
        assert_eq!(snapshot.terminal_duration_millis, None);
        assert_eq!(snapshot.outcome_color, None);
    }
}
