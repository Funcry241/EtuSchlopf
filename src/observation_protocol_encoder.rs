use crate::observation_protocol::{
    role_matches_stage,
    SnapshotActivityV2, SnapshotOutcomeColorV2, SnapshotSourceRoleV2, SnapshotSourceStatusV2,
    SnapshotStageV2, SourceSnapshotV2, SOURCE_SNAPSHOT_PROTOCOL_VERSION,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotEncodeError {
    UnsupportedProtocolVersion(u16),
    StringTooLong(&'static str),
    InvalidRoleStageCombination,
}

pub fn encode_source_snapshot_v2(
    snapshot: &SourceSnapshotV2,
) -> Result<Vec<u8>, SnapshotEncodeError> {
    if snapshot.protocol_version != SOURCE_SNAPSHOT_PROTOCOL_VERSION {
        return Err(SnapshotEncodeError::UnsupportedProtocolVersion(
            snapshot.protocol_version,
        ));
    }
    if !role_matches_stage(snapshot.source_role, snapshot.stage) {
        return Err(SnapshotEncodeError::InvalidRoleStageCombination);
    }

    let mut bytes = Vec::with_capacity(128);
    bytes.extend_from_slice(b"ETSOBS2\0");
    bytes.extend_from_slice(&snapshot.protocol_version.to_le_bytes());
    push_string(
        &mut bytes,
        "observation_session_id",
        &snapshot.observation_session_id,
    )?;
    push_string(&mut bytes, "source_id", &snapshot.source_id)?;
    push_optional_string(
        &mut bytes,
        "parent_source_id",
        snapshot.parent_source_id.as_deref(),
    )?;
    bytes.push(role_code(snapshot.source_role));
    bytes.extend_from_slice(&snapshot.revision.to_le_bytes());
    bytes.push(status_code(snapshot.status));
    bytes.push(stage_code(snapshot.stage));
    bytes.push(snapshot.activity.map_or(0, activity_code));
    push_optional_string(
        &mut bytes,
        "project_label",
        snapshot.project_label.as_deref(),
    )?;
    match snapshot.progress {
        Some(progress) => {
            bytes.push(1);
            bytes.extend_from_slice(&progress.completed.to_le_bytes());
            bytes.extend_from_slice(&progress.total.to_le_bytes());
            bytes.extend_from_slice(&progress.active.to_le_bytes());
        }
        None => bytes.push(0),
    }
    bytes.extend_from_slice(&snapshot.started_unix_millis.to_le_bytes());
    match snapshot.terminal_duration_millis {
        Some(duration_millis) => {
            bytes.push(1);
            bytes.extend_from_slice(&duration_millis.to_le_bytes());
        }
        None => bytes.push(0),
    }
    bytes.push(snapshot.outcome_color.map_or(0, outcome_color_code));
    Ok(bytes)
}

fn outcome_color_code(color: SnapshotOutcomeColorV2) -> u8 {
    match color {
        SnapshotOutcomeColorV2::DarkGreen => 1,
        SnapshotOutcomeColorV2::LightGreen => 2,
        SnapshotOutcomeColorV2::Yellow => 3,
        SnapshotOutcomeColorV2::Red => 4,
    }
}

fn role_code(role: SnapshotSourceRoleV2) -> u8 {
    match role {
        SnapshotSourceRoleV2::Launcher => 1,
        SnapshotSourceRoleV2::Corpus => 2,
        SnapshotSourceRoleV2::Project => 3,
    }
}

fn status_code(status: SnapshotSourceStatusV2) -> u8 {
    match status {
        SnapshotSourceStatusV2::Running => 1,
        SnapshotSourceStatusV2::Completed => 2,
        SnapshotSourceStatusV2::Failed => 3,
        SnapshotSourceStatusV2::Cancelled => 4,
    }
}

fn stage_code(stage: SnapshotStageV2) -> u8 {
    match stage {
        SnapshotStageV2::LauncherPreparing => 1,
        SnapshotStageV2::LauncherBuilding => 2,
        SnapshotStageV2::LauncherTests => 3,
        SnapshotStageV2::LauncherRunningTarget => 4,
        SnapshotStageV2::LauncherFinalizing => 5,
        SnapshotStageV2::ProjectRunPreparation => 16,
        SnapshotStageV2::ProjectArrangementSolver => 17,
        SnapshotStageV2::ProjectMasterNotationAndSalience => 18,
        SnapshotStageV2::ProjectInitialRenderTransaction => 19,
        SnapshotStageV2::ProjectMetricBeamAndFinalNotation => 20,
        SnapshotStageV2::ProjectReleaseReportAndExit => 21,
        SnapshotStageV2::CorpusPlanning => 32,
        SnapshotStageV2::CorpusPreparingProjects => 33,
        SnapshotStageV2::CorpusRunningProjects => 34,
        SnapshotStageV2::CorpusCollectingProjectResult => 35,
        SnapshotStageV2::CorpusFinalizingCorpusReport => 36,
    }
}

fn activity_code(activity: SnapshotActivityV2) -> u8 {
    match activity {
        SnapshotActivityV2::ClefPlanning => 1,
        SnapshotActivityV2::BeamPlanning => 2,
    }
}

fn push_string(
    bytes: &mut Vec<u8>,
    field: &'static str,
    value: &str,
) -> Result<(), SnapshotEncodeError> {
    let length =
        u16::try_from(value.len()).map_err(|_| SnapshotEncodeError::StringTooLong(field))?;
    bytes.extend_from_slice(&length.to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

fn push_optional_string(
    bytes: &mut Vec<u8>,
    field: &'static str,
    value: Option<&str>,
) -> Result<(), SnapshotEncodeError> {
    match value {
        Some(value) => {
            bytes.push(1);
            push_string(bytes, field, value)
        }
        None => {
            bytes.push(0);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::encode_source_snapshot_v2;
    use crate::observation::{CorpusStage, LauncherStage, ProjectStage, SourceProgress, SourceStateOwner};
    use crate::observation_protocol::SourceSnapshotV2;
    use crate::observation_snapshot_projection::project_source_snapshot_v2;

    #[test]
    fn identical_revision_and_state_encode_to_identical_bytes() {
        let owner = SourceStateOwner::launcher();
        let snapshot = project_source_snapshot_v2(&owner);
        let first = encode_source_snapshot_v2(&snapshot).expect("encode first snapshot");
        let second = encode_source_snapshot_v2(&snapshot).expect("encode second snapshot");
        assert_eq!(first, second);
    }

    #[test]
    fn launcher_project_and_corpus_snapshots_round_trip() {
        let mut launcher = SourceStateOwner::launcher();
        launcher.enter_launcher_stage(LauncherStage::Building);
        let mut project = SourceStateOwner::project(Some("Synthetic Project Nr. 6".to_string()));
        project.enter_project_stage(ProjectStage::MetricBeamAndFinalNotation);
        let mut corpus = SourceStateOwner::corpus(10);
        corpus.enter_corpus_stage(
            CorpusStage::RunningProjects,
            SourceProgress {
                completed: 6,
                total: 10,
                active: 2,
            },
        );

        for owner in [&launcher, &project, &corpus] {
            let snapshot = project_source_snapshot_v2(owner);
            let encoded = encode_source_snapshot_v2(&snapshot).expect("encode snapshot");
            let decoded = SourceSnapshotV2::decode_v2(&encoded).expect("decode snapshot");
            assert_eq!(decoded, snapshot);
        }
    }

    #[test]
    fn decoder_rejects_trailing_bytes_from_encoder_output() {
        let snapshot = project_source_snapshot_v2(&SourceStateOwner::launcher());
        let mut encoded = encode_source_snapshot_v2(&snapshot).expect("encode snapshot");
        encoded.push(0);
        assert!(SourceSnapshotV2::decode_v2(&encoded).is_err());
    }
}
