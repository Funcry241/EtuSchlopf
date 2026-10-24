//! Console status records: one owner for the line format that project runs and the
//! corpus batch append to the technical log, and that the launcher may read back for
//! its interactive console.
//!
//! Records go to stdout. A single project run started by the launcher writes stdout into
//! the technical log; a corpus batch child's stdout is read line by line by the batch,
//! which forwards valid records immediately to its own stderr (the technical log), while
//! the child's diagnostic stderr is kept in a per-project log.
//!
//! Projection only. A record never grants authority and is never an input to a musical,
//! release or gate decision. Emission is opt-in through [`ENABLE_ENV`] (set by the
//! launcher for the program it starts and inherited by batch children) and best effort:
//! a failed write is ignored and can never fail a run.
//!
//! Format (one line, every value percent-encoded):
//! `ETUSCHLOPF-STATUS v1: session=<s>; source=<s>; project=<s>; event=stage; stage=<stage>`
//! `ETUSCHLOPF-STATUS v1: session=<s>; source=<s>; project=<s>; event=finished; outcome=<ok|failed>; ms=<n>`
//!
//! Records are built completely before a single write. This does not make concurrent
//! appends from several processes atomic, so readers must accept only complete lines that
//! decode as a valid record and must ignore everything else.

use crate::observation::{ObservationSessionContext, ProjectStage, SourceStage, SourceStateOwner};
use std::io::Write;

/// Environment switch for emission. Only the exact value [`ENABLE_VALUE`] enables it.
pub const ENABLE_ENV: &str = "ETUSCHLOPF_CONSOLE_STATUS";
/// The only accepted value of [`ENABLE_ENV`].
pub const ENABLE_VALUE: &str = "v1";
/// Line prefix of every record.
pub const RECORD_PREFIX: &str = "ETUSCHLOPF-STATUS v1: ";

/// Final outcome of one project as decided by the corpus batch after exit code and
/// output-file checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectOutcome {
    Ok,
    Failed,
}

/// Event carried by a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEvent {
    Stage(ProjectStage),
    Finished {
        outcome: ProjectOutcome,
        elapsed_ms: u64,
    },
}

/// One decoded console status record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusRecord {
    pub session: String,
    pub source: String,
    pub project: String,
    pub event: StatusEvent,
}

const fn stage_wire(stage: ProjectStage) -> &'static str {
    match stage {
        ProjectStage::ProjectRunPreparation => "project_run_preparation",
        ProjectStage::ArrangementSolver => "arrangement_solver",
        ProjectStage::MasterNotationAndSalience => "master_notation_and_salience",
        ProjectStage::InitialRenderTransaction => "initial_render_transaction",
        ProjectStage::MetricBeamAndFinalNotation => "metric_beam_and_final_notation",
        ProjectStage::ReleaseReportAndExit => "release_report_and_exit",
    }
}

fn stage_from_wire(value: &str) -> Option<ProjectStage> {
    [
        ProjectStage::ProjectRunPreparation,
        ProjectStage::ArrangementSolver,
        ProjectStage::MasterNotationAndSalience,
        ProjectStage::InitialRenderTransaction,
        ProjectStage::MetricBeamAndFinalNotation,
        ProjectStage::ReleaseReportAndExit,
    ]
    .into_iter()
    .find(|stage| stage_wire(*stage) == value)
}

const fn outcome_wire(outcome: ProjectOutcome) -> &'static str {
    match outcome {
        ProjectOutcome::Ok => "ok",
        ProjectOutcome::Failed => "failed",
    }
}

fn outcome_from_wire(value: &str) -> Option<ProjectOutcome> {
    match value {
        "ok" => Some(ProjectOutcome::Ok),
        "failed" => Some(ProjectOutcome::Failed),
        _ => None,
    }
}

const fn is_unreserved(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_')
}

/// Percent-encodes every byte outside `[A-Za-z0-9._-]`; the result is unambiguous and
/// contains no separator, space or line break.
pub fn encode_value(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        if is_unreserved(byte) {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(b"0123456789ABCDEF"[usize::from(byte >> 4)]));
            out.push(char::from(b"0123456789ABCDEF"[usize::from(byte & 0x0f)]));
        }
    }
    out
}

fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Exact inverse of [`encode_value`]. Rejects anything [`encode_value`] cannot produce.
pub fn decode_value(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'%' {
            let high = hex_digit(*bytes.get(index + 1)?)?;
            let low = hex_digit(*bytes.get(index + 2)?)?;
            let decoded = (high << 4) | low;
            if is_unreserved(decoded) {
                return None;
            }
            out.push(decoded);
            index += 3;
        } else if is_unreserved(byte) {
            out.push(byte);
            index += 1;
        } else {
            return None;
        }
    }
    String::from_utf8(out).ok()
}

impl StatusRecord {
    /// Complete line including the trailing line break.
    pub fn encode_line(&self) -> String {
        let mut line = format!(
            "{RECORD_PREFIX}session={}; source={}; project={}; ",
            encode_value(&self.session),
            encode_value(&self.source),
            encode_value(&self.project)
        );
        match self.event {
            StatusEvent::Stage(stage) => {
                line.push_str("event=stage; stage=");
                line.push_str(stage_wire(stage));
            }
            StatusEvent::Finished {
                outcome,
                elapsed_ms,
            } => {
                line.push_str("event=finished; outcome=");
                line.push_str(outcome_wire(outcome));
                line.push_str("; ms=");
                line.push_str(&elapsed_ms.to_string());
            }
        }
        line.push('\n');
        line
    }

    /// Strict decoder for one complete line (with or without its line break). Returns
    /// `None` for every line that is not exactly a valid v1 record.
    pub fn decode_line(line: &str) -> Option<Self> {
        let line = line.strip_suffix('\n').unwrap_or(line);
        let line = line.strip_suffix('\r').unwrap_or(line);
        let body = line.strip_prefix(RECORD_PREFIX)?;
        let fields = body.split("; ").collect::<Vec<_>>();
        let value = |index: usize, key: &str| -> Option<&str> {
            fields.get(index)?.strip_prefix(key)?.strip_prefix('=')
        };
        let session = decode_value(value(0, "session")?)?;
        let source = decode_value(value(1, "source")?)?;
        let project = decode_value(value(2, "project")?)?;
        let event = match value(3, "event")? {
            "stage" if fields.len() == 5 => StatusEvent::Stage(stage_from_wire(value(4, "stage")?)?),
            "finished" if fields.len() == 6 => {
                let ms = value(5, "ms")?;
                if ms.is_empty() || !ms.bytes().all(|byte| byte.is_ascii_digit()) {
                    return None;
                }
                StatusEvent::Finished {
                    outcome: outcome_from_wire(value(4, "outcome")?)?,
                    elapsed_ms: ms.parse().ok()?,
                }
            }
            _ => return None,
        };
        Some(Self {
            session,
            source,
            project,
            event,
        })
    }
}

fn emission_enabled_for(value: Option<&str>) -> bool {
    value == Some(ENABLE_VALUE)
}

fn emission_enabled() -> bool {
    emission_enabled_for(std::env::var(ENABLE_ENV).ok().as_deref())
}

fn emit_record_best_effort(record: &StatusRecord) {
    let line = record.encode_line();
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(line.as_bytes());
    let _ = stdout.flush();
}

/// Forwards one line read from a batch child's stdout to this process's stderr when it is
/// exactly a valid record; every other line is ignored. Best effort.
pub fn forward_record_line(line: &str) {
    if StatusRecord::decode_line(line).is_none() {
        return;
    }
    let mut forwarded = line.trim_end_matches(['\r', '\n']).to_string();
    forwarded.push('\n');
    let _ = std::io::stderr().lock().write_all(forwarded.as_bytes());
}

/// Emits the current project stage of `owner` when emission is enabled. Does nothing for
/// non-project stages or when the owner has no project label.
pub fn emit_project_stage(owner: &SourceStateOwner) {
    if !emission_enabled() {
        return;
    }
    let state = owner.state_for_snapshot_projection();
    let SourceStage::Project(stage) = state.stage else {
        return;
    };
    let Some(project) = state.project_label.as_deref() else {
        return;
    };
    let context = owner.context();
    emit_record_best_effort(&StatusRecord {
        session: context.session_id().as_str().to_string(),
        source: context.source_id().as_str().to_string(),
        project: project.to_string(),
        event: StatusEvent::Stage(stage),
    });
}

/// Emits the batch-decided final outcome of one project when emission is enabled.
/// `child` is the observation context the batch handed to that project's process.
pub fn emit_project_finished(
    child: &ObservationSessionContext,
    project: &str,
    outcome: ProjectOutcome,
    elapsed_ms: u64,
) {
    if !emission_enabled() {
        return;
    }
    emit_record_best_effort(&StatusRecord {
        session: child.session_id().as_str().to_string(),
        source: child.source_id().as_str().to_string(),
        project: project.to_string(),
        event: StatusEvent::Finished {
            outcome,
            elapsed_ms,
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage_record(project: &str, stage: ProjectStage) -> StatusRecord {
        StatusRecord {
            session: "obs-0.10.1839-1-2-0".to_string(),
            source: "project-3".to_string(),
            project: project.to_string(),
            event: StatusEvent::Stage(stage),
        }
    }

    #[test]
    fn every_stage_round_trips() {
        for stage in [
            ProjectStage::ProjectRunPreparation,
            ProjectStage::ArrangementSolver,
            ProjectStage::MasterNotationAndSalience,
            ProjectStage::InitialRenderTransaction,
            ProjectStage::MetricBeamAndFinalNotation,
            ProjectStage::ReleaseReportAndExit,
        ] {
            let record = stage_record("Stück Ä 7", stage);
            assert_eq!(StatusRecord::decode_line(&record.encode_line()), Some(record));
        }
    }

    #[test]
    fn finished_round_trips_for_both_outcomes() {
        for outcome in [ProjectOutcome::Ok, ProjectOutcome::Failed] {
            let record = StatusRecord {
                session: "s".to_string(),
                source: "project-0".to_string(),
                project: "Probe Ö 12".to_string(),
                event: StatusEvent::Finished {
                    outcome,
                    elapsed_ms: 147_452,
                },
            };
            assert_eq!(StatusRecord::decode_line(&record.encode_line()), Some(record));
        }
    }

    #[test]
    fn hostile_project_names_stay_one_unambiguous_line() {
        let name = "a; event=finished; outcome=ok\nETUSCHLOPF-STATUS v1: x=%41 ü";
        let line = stage_record(name, ProjectStage::ArrangementSolver).encode_line();
        assert_eq!(line.matches('\n').count(), 1);
        assert_eq!(line.matches("; ").count(), 4);
        assert_eq!(
            StatusRecord::decode_line(&line).map(|record| record.project),
            Some(name.to_string())
        );
    }

    #[test]
    fn value_encoding_is_canonical() {
        assert_eq!(encode_value("Stück Ä 7"), "St%C3%BCck%20%C3%84%207");
        assert_eq!(decode_value("St%C3%BCck%20%C3%84%207").as_deref(), Some("Stück Ä 7"));
        assert_eq!(decode_value("%41"), None, "unreserved bytes must not be encoded");
        assert_eq!(decode_value("%c3%bc"), None, "lower-case hex is not canonical");
        assert_eq!(decode_value("a b"), None);
        assert_eq!(decode_value("%2"), None);
        assert_eq!(decode_value("%FF"), None, "invalid UTF-8 is rejected");
    }

    #[test]
    fn incomplete_corrupt_and_unknown_lines_are_rejected() {
        let valid = stage_record("Stück Ä 7", ProjectStage::InitialRenderTransaction).encode_line();
        let trimmed = valid.trim_end();
        for cut in 0..trimmed.len() {
            if valid.is_char_boundary(cut) && cut != trimmed.len() {
                let partial = &valid[..cut];
                assert_eq!(StatusRecord::decode_line(partial), None, "{partial:?}");
            }
        }
        for line in [
            "",
            "ETUSCHLOPF-STATUS v2: session=s; source=p; project=x; event=stage; stage=arrangement_solver",
            "ETUSCHLOPF-STATUS v1: session=s; source=p; project=x; event=stage; stage=unknown",
            "ETUSCHLOPF-STATUS v1: session=s; source=p; project=x; event=stage; stage=arrangement_solver; extra=1",
            "ETUSCHLOPF-STATUS v1: source=p; session=s; project=x; event=stage; stage=arrangement_solver",
            "ETUSCHLOPF-STATUS v1: session=s; source=p; project=x; event=finished; outcome=ok; ms=",
            "ETUSCHLOPF-STATUS v1: session=s; source=p; project=x; event=finished; outcome=ok; ms=+5",
            "ETUSCHLOPF-STATUS v1: session=s; source=p; project=x; event=finished; outcome=maybe; ms=5",
            "ETUSCHLOPF-STATUS v1: session=s; source=p; project=x; event=finished; outcome=ok; ms=99999999999999999999",
            "  ETUSCHLOPF-STATUS v1: session=s; source=p; project=x; event=stage; stage=arrangement_solver",
            "OriginalBeamOriginCensus: group=x; ETUSCHLOPF-STATUS v1: session=s",
        ] {
            assert_eq!(StatusRecord::decode_line(line), None, "{line:?}");
        }
        assert!(StatusRecord::decode_line(trimmed).is_some());
        assert!(StatusRecord::decode_line(&format!("{trimmed}\r\n")).is_some());
    }

    #[test]
    fn emission_requires_the_exact_switch_value() {
        assert!(emission_enabled_for(Some("v1")));
        for value in [None, Some(""), Some("1"), Some("V1"), Some("v1 "), Some("true")] {
            assert!(!emission_enabled_for(value), "{value:?}");
        }
    }
}
