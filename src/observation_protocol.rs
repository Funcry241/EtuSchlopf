pub const SOURCE_SNAPSHOT_PROTOCOL_VERSION: u16 = 2;
pub const SOURCE_SNAPSHOT_LOOPBACK_PORT: u16 = 47_621;
pub const SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES: usize = 8 * 1024;
pub const WATCH_ACTIVE_SESSION_ENV: &str = "ETUSCHLOPF_WATCH_ACTIVE_SESSION";
const SOURCE_SNAPSHOT_MAGIC: &[u8; 8] = b"ETSOBS2\0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotSourceRoleV2 {
    Launcher,
    Corpus,
    Project,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotSourceStatusV2 {
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotStageV2 {
    LauncherPreparing,
    LauncherBuilding,
    LauncherTests,
    LauncherRunningTarget,
    LauncherFinalizing,
    ProjectRunPreparation,
    ProjectArrangementSolver,
    ProjectMasterNotationAndSalience,
    ProjectInitialRenderTransaction,
    ProjectMetricBeamAndFinalNotation,
    ProjectReleaseReportAndExit,
    CorpusPlanning,
    CorpusPreparingProjects,
    CorpusRunningProjects,
    CorpusCollectingProjectResult,
    CorpusFinalizingCorpusReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotActivityV2 {
    ClefPlanning,
    BeamPlanning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotProgressV2 {
    pub completed: u64,
    pub total: u64,
    pub active: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotOutcomeColorV2 {
    DarkGreen,
    LightGreen,
    Yellow,
    Red,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSnapshotV2 {
    pub protocol_version: u16,
    pub observation_session_id: String,
    pub source_id: String,
    pub parent_source_id: Option<String>,
    pub source_role: SnapshotSourceRoleV2,
    pub revision: u64,
    pub status: SnapshotSourceStatusV2,
    pub stage: SnapshotStageV2,
    pub activity: Option<SnapshotActivityV2>,
    pub project_label: Option<String>,
    pub progress: Option<SnapshotProgressV2>,
    pub started_unix_millis: u64,
    pub terminal_duration_millis: Option<u64>,
    pub outcome_color: Option<SnapshotOutcomeColorV2>,
}

pub const SESSION_SNAPSHOT_PROTOCOL_VERSION: u16 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedSourceSnapshotV2 {
    pub source: SourceSnapshotV2,
    pub last_seen_age_millis: u64,
    pub last_state_change_age_millis: u64,
    pub run_elapsed_millis: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSnapshotV2 {
    pub protocol_version: u16,
    pub observation_session_id: String,
    pub sources: Vec<ObservedSourceSnapshotV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotCodecError {
    Truncated,
    InvalidMagic,
    UnsupportedProtocolVersion(u16),
    InvalidEnumValue { field: &'static str, value: u8 },
    InvalidUtf8(&'static str),
    InvalidRoleStageCombination,
    TrailingBytes,
}

impl SourceSnapshotV2 {
    pub fn decode_v2(bytes: &[u8]) -> Result<Self, SnapshotCodecError> {
        let mut cursor = Cursor::new(bytes);
        let magic = cursor.take(SOURCE_SNAPSHOT_MAGIC.len())?;
        if magic != &SOURCE_SNAPSHOT_MAGIC[..] {
            return Err(SnapshotCodecError::InvalidMagic);
        }
        let protocol_version = cursor.take_u16()?;
        if protocol_version != SOURCE_SNAPSHOT_PROTOCOL_VERSION {
            return Err(SnapshotCodecError::UnsupportedProtocolVersion(protocol_version));
        }
        let observation_session_id = cursor.take_string("observation_session_id")?;
        let source_id = cursor.take_string("source_id")?;
        let parent_source_id = cursor.take_optional_string("parent_source_id")?;
        let source_role = decode_role(cursor.take_u8()?)?;
        let revision = cursor.take_u64()?;
        let status = decode_status(cursor.take_u8()?)?;
        let stage = decode_stage(cursor.take_u8()?)?;
        let activity = decode_activity(cursor.take_u8()?)?;
        let project_label = cursor.take_optional_string("project_label")?;
        let progress = match cursor.take_u8()? {
            0 => None,
            1 => Some(SnapshotProgressV2 {
                completed: cursor.take_u64()?,
                total: cursor.take_u64()?,
                active: cursor.take_u64()?,
            }),
            value => {
                return Err(SnapshotCodecError::InvalidEnumValue {
                    field: "progress_presence",
                    value,
                });
            }
        };
        let started_unix_millis = cursor.take_u64()?;
        let terminal_duration_millis = match cursor.take_u8()? {
            0 => None,
            1 => Some(cursor.take_u64()?),
            value => {
                return Err(SnapshotCodecError::InvalidEnumValue {
                    field: "terminal_duration_presence",
                    value,
                });
            }
        };
        let outcome_color = decode_outcome_color(cursor.take_u8()?)?;
        if !cursor.is_finished() {
            return Err(SnapshotCodecError::TrailingBytes);
        }
        if !role_matches_stage(source_role, stage) {
            return Err(SnapshotCodecError::InvalidRoleStageCombination);
        }
        Ok(Self {
            protocol_version,
            observation_session_id,
            source_id,
            parent_source_id,
            source_role,
            revision,
            status,
            stage,
            activity,
            project_label,
            progress,
            started_unix_millis,
            terminal_duration_millis,
            outcome_color,
        })
    }

}

fn decode_role(value: u8) -> Result<SnapshotSourceRoleV2, SnapshotCodecError> {
    match value {
        1 => Ok(SnapshotSourceRoleV2::Launcher),
        2 => Ok(SnapshotSourceRoleV2::Corpus),
        3 => Ok(SnapshotSourceRoleV2::Project),
        _ => Err(SnapshotCodecError::InvalidEnumValue {
            field: "source_role",
            value,
        }),
    }
}

fn decode_status(value: u8) -> Result<SnapshotSourceStatusV2, SnapshotCodecError> {
    match value {
        1 => Ok(SnapshotSourceStatusV2::Running),
        2 => Ok(SnapshotSourceStatusV2::Completed),
        3 => Ok(SnapshotSourceStatusV2::Failed),
        4 => Ok(SnapshotSourceStatusV2::Cancelled),
        _ => Err(SnapshotCodecError::InvalidEnumValue {
            field: "status",
            value,
        }),
    }
}

fn decode_stage(value: u8) -> Result<SnapshotStageV2, SnapshotCodecError> {
    match value {
        1 => Ok(SnapshotStageV2::LauncherPreparing),
        2 => Ok(SnapshotStageV2::LauncherBuilding),
        3 => Ok(SnapshotStageV2::LauncherTests),
        4 => Ok(SnapshotStageV2::LauncherRunningTarget),
        5 => Ok(SnapshotStageV2::LauncherFinalizing),
        16 => Ok(SnapshotStageV2::ProjectRunPreparation),
        17 => Ok(SnapshotStageV2::ProjectArrangementSolver),
        18 => Ok(SnapshotStageV2::ProjectMasterNotationAndSalience),
        19 => Ok(SnapshotStageV2::ProjectInitialRenderTransaction),
        20 => Ok(SnapshotStageV2::ProjectMetricBeamAndFinalNotation),
        21 => Ok(SnapshotStageV2::ProjectReleaseReportAndExit),
        32 => Ok(SnapshotStageV2::CorpusPlanning),
        33 => Ok(SnapshotStageV2::CorpusPreparingProjects),
        34 => Ok(SnapshotStageV2::CorpusRunningProjects),
        35 => Ok(SnapshotStageV2::CorpusCollectingProjectResult),
        36 => Ok(SnapshotStageV2::CorpusFinalizingCorpusReport),
        _ => Err(SnapshotCodecError::InvalidEnumValue {
            field: "stage",
            value,
        }),
    }
}

fn decode_activity(value: u8) -> Result<Option<SnapshotActivityV2>, SnapshotCodecError> {
    match value {
        0 => Ok(None),
        1 => Ok(Some(SnapshotActivityV2::ClefPlanning)),
        2 => Ok(Some(SnapshotActivityV2::BeamPlanning)),
        _ => Err(SnapshotCodecError::InvalidEnumValue {
            field: "activity",
            value,
        }),
    }
}

fn decode_outcome_color(value: u8) -> Result<Option<SnapshotOutcomeColorV2>, SnapshotCodecError> {
    match value {
        0 => Ok(None),
        1 => Ok(Some(SnapshotOutcomeColorV2::DarkGreen)),
        2 => Ok(Some(SnapshotOutcomeColorV2::LightGreen)),
        3 => Ok(Some(SnapshotOutcomeColorV2::Yellow)),
        4 => Ok(Some(SnapshotOutcomeColorV2::Red)),
        _ => Err(SnapshotCodecError::InvalidEnumValue {
            field: "outcome_color",
            value,
        }),
    }
}

pub(crate) fn role_matches_stage(role: SnapshotSourceRoleV2, stage: SnapshotStageV2) -> bool {
    matches!(
        (role, stage),
        (
            SnapshotSourceRoleV2::Launcher,
            SnapshotStageV2::LauncherPreparing
                | SnapshotStageV2::LauncherBuilding
                | SnapshotStageV2::LauncherTests
                | SnapshotStageV2::LauncherRunningTarget
                | SnapshotStageV2::LauncherFinalizing
        ) | (
            SnapshotSourceRoleV2::Project,
            SnapshotStageV2::ProjectRunPreparation
                | SnapshotStageV2::ProjectArrangementSolver
                | SnapshotStageV2::ProjectMasterNotationAndSalience
                | SnapshotStageV2::ProjectInitialRenderTransaction
                | SnapshotStageV2::ProjectMetricBeamAndFinalNotation
                | SnapshotStageV2::ProjectReleaseReportAndExit
        ) | (
            SnapshotSourceRoleV2::Corpus,
            SnapshotStageV2::CorpusPlanning
                | SnapshotStageV2::CorpusPreparingProjects
                | SnapshotStageV2::CorpusRunningProjects
                | SnapshotStageV2::CorpusCollectingProjectResult
                | SnapshotStageV2::CorpusFinalizingCorpusReport
        )
    )
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], SnapshotCodecError> {
        let end = self
            .offset
            .checked_add(length)
            .filter(|end| *end <= self.bytes.len())
            .ok_or(SnapshotCodecError::Truncated)?;
        let value = &self.bytes[self.offset..end];
        self.offset = end;
        Ok(value)
    }

    fn take_u8(&mut self) -> Result<u8, SnapshotCodecError> {
        Ok(self.take(1)?[0])
    }

    fn take_u16(&mut self) -> Result<u16, SnapshotCodecError> {
        let bytes: [u8; 2] = self
            .take(2)?
            .try_into()
            .map_err(|_| SnapshotCodecError::Truncated)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn take_u64(&mut self) -> Result<u64, SnapshotCodecError> {
        let bytes: [u8; 8] = self
            .take(8)?
            .try_into()
            .map_err(|_| SnapshotCodecError::Truncated)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn take_string(&mut self, field: &'static str) -> Result<String, SnapshotCodecError> {
        let length = usize::from(self.take_u16()?);
        let bytes = self.take(length)?;
        std::str::from_utf8(bytes)
            .map(str::to_string)
            .map_err(|_| SnapshotCodecError::InvalidUtf8(field))
    }

    fn take_optional_string(
        &mut self,
        field: &'static str,
    ) -> Result<Option<String>, SnapshotCodecError> {
        match self.take_u8()? {
            0 => Ok(None),
            1 => self.take_string(field).map(Some),
            value => Err(SnapshotCodecError::InvalidEnumValue {
                field,
                value,
            }),
        }
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SnapshotCodecError, SnapshotSourceRoleV2, SnapshotSourceStatusV2, SnapshotStageV2,
        SourceSnapshotV2, SOURCE_SNAPSHOT_PROTOCOL_VERSION,
    };

    fn minimal_project_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ETSOBS2\0");
        bytes.extend_from_slice(&SOURCE_SNAPSHOT_PROTOCOL_VERSION.to_le_bytes());
        push_string(&mut bytes, "session-a");
        push_string(&mut bytes, "project:1");
        bytes.push(0);
        bytes.push(3);
        bytes.extend_from_slice(&7u64.to_le_bytes());
        bytes.push(1);
        bytes.push(16);
        bytes.push(0);
        bytes.push(0);
        bytes.push(0);
        bytes.extend_from_slice(&1_000u64.to_le_bytes());
        bytes.push(0);
        bytes.push(0);
        bytes
    }

    fn push_string(bytes: &mut Vec<u8>, value: &str) {
        let len = u16::try_from(value.len()).expect("test string length");
        bytes.extend_from_slice(&len.to_le_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }

    #[test]
    fn decoder_accepts_minimal_project_snapshot() {
        let decoded = SourceSnapshotV2::decode_v2(&minimal_project_bytes()).expect("decode");
        assert_eq!(decoded.source_role, SnapshotSourceRoleV2::Project);
        assert_eq!(decoded.status, SnapshotSourceStatusV2::Running);
        assert_eq!(decoded.stage, SnapshotStageV2::ProjectRunPreparation);
        assert_eq!(decoded.revision, 7);
    }

    #[test]
    fn decoder_rejects_trailing_bytes() {
        let mut bytes = minimal_project_bytes();
        bytes.push(0);
        assert_eq!(
            SourceSnapshotV2::decode_v2(&bytes),
            Err(SnapshotCodecError::TrailingBytes)
        );
    }
}
