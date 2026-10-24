use crate::observation_protocol::{
    ObservedSourceSnapshotV2, SessionSnapshotV2, SourceSnapshotV2,
    SESSION_SNAPSHOT_PROTOCOL_VERSION,
};
use std::collections::BTreeMap;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotIngestOutcome {
    NewSource,
    Advanced,
    Reannounced,
    StaleIgnored,
    SameRevisionConflict,
    SourceIdentityConflict,
}

#[derive(Debug)]
pub struct ObservedSource {
    snapshot: SourceSnapshotV2,
    last_seen: Instant,
    last_state_change: Instant,
}

impl ObservedSource {
    #[cfg(test)]
    pub fn snapshot(&self) -> &SourceSnapshotV2 {
        &self.snapshot
    }

    #[cfg(test)]
    pub fn last_seen(&self) -> Instant {
        self.last_seen
    }

    #[cfg(test)]
    pub fn last_state_change(&self) -> Instant {
        self.last_state_change
    }
}

#[derive(Debug, Default)]
struct SessionState {
    sources: BTreeMap<String, ObservedSource>,
}

#[derive(Debug, Default)]
pub struct SessionAggregator {
    sessions: BTreeMap<String, SessionState>,
}

impl SessionAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ingest(&mut self, snapshot: SourceSnapshotV2) -> SnapshotIngestOutcome {
        self.ingest_at(snapshot, Instant::now())
    }

    #[cfg(test)]
    pub fn source(&self, session_id: &str, source_id: &str) -> Option<&ObservedSource> {
        self.sessions
            .get(session_id)
            .and_then(|session| session.sources.get(source_id))
    }

    #[cfg(test)]
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    #[cfg(test)]
    pub fn source_count(&self, session_id: &str) -> usize {
        self.sessions
            .get(session_id)
            .map_or(0, |session| session.sources.len())
    }

    #[cfg(test)]
    pub fn session_snapshot_v2(&self, session_id: &str) -> Option<SessionSnapshotV2> {
        self.session_snapshot_v2_at(session_id, Instant::now(), unix_millis_now())
    }

    pub fn session_snapshots_v2(&self) -> Vec<SessionSnapshotV2> {
        let now = Instant::now();
        let now_unix_millis = unix_millis_now();
        self.sessions
            .keys()
            .filter_map(|session_id| {
                self.session_snapshot_v2_at(session_id, now, now_unix_millis)
            })
            .collect()
    }

    fn session_snapshot_v2_at(
        &self,
        session_id: &str,
        now: Instant,
        now_unix_millis: u64,
    ) -> Option<SessionSnapshotV2> {
        let session = self.sessions.get(session_id)?;
        let sources = session
            .sources
            .values()
            .map(|observed| ObservedSourceSnapshotV2 {
                source: observed.snapshot.clone(),
                last_seen_age_millis: duration_millis_saturating(
                    now.saturating_duration_since(observed.last_seen),
                ),
                last_state_change_age_millis: duration_millis_saturating(
                    now.saturating_duration_since(observed.last_state_change),
                ),
                run_elapsed_millis: observed
                    .snapshot
                    .terminal_duration_millis
                    .unwrap_or_else(|| {
                        now_unix_millis.saturating_sub(observed.snapshot.started_unix_millis)
                    }),
            })
            .collect();
        Some(SessionSnapshotV2 {
            protocol_version: SESSION_SNAPSHOT_PROTOCOL_VERSION,
            observation_session_id: session_id.to_string(),
            sources,
        })
    }

    fn ingest_at(&mut self, snapshot: SourceSnapshotV2, now: Instant) -> SnapshotIngestOutcome {
        let session_id = snapshot.observation_session_id.clone();
        let source_id = snapshot.source_id.clone();
        let session = self.sessions.entry(session_id).or_default();
        let Some(current) = session.sources.get_mut(&source_id) else {
            session.sources.insert(
                source_id,
                ObservedSource {
                    snapshot,
                    last_seen: now,
                    last_state_change: now,
                },
            );
            return SnapshotIngestOutcome::NewSource;
        };

        if !same_source_identity(&current.snapshot, &snapshot) {
            return SnapshotIngestOutcome::SourceIdentityConflict;
        }

        if snapshot.revision < current.snapshot.revision {
            return SnapshotIngestOutcome::StaleIgnored;
        }

        if snapshot.revision == current.snapshot.revision {
            if snapshot != current.snapshot {
                return SnapshotIngestOutcome::SameRevisionConflict;
            }
            current.last_seen = now;
            return SnapshotIngestOutcome::Reannounced;
        }

        current.snapshot = snapshot;
        current.last_seen = now;
        current.last_state_change = now;
        SnapshotIngestOutcome::Advanced
    }
}

fn unix_millis_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(duration_millis_saturating)
        .unwrap_or(0)
}

fn duration_millis_saturating(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn same_source_identity(left: &SourceSnapshotV2, right: &SourceSnapshotV2) -> bool {
    left.observation_session_id == right.observation_session_id
        && left.source_id == right.source_id
        && left.parent_source_id == right.parent_source_id
        && left.source_role == right.source_role
}

#[cfg(test)]
mod tests {
    use super::{SessionAggregator, SnapshotIngestOutcome};
    use crate::observation_protocol::{
        SnapshotSourceRoleV2, SnapshotSourceStatusV2, SnapshotStageV2, SourceSnapshotV2,
        SESSION_SNAPSHOT_PROTOCOL_VERSION, SOURCE_SNAPSHOT_PROTOCOL_VERSION,
    };
    use std::time::{Duration, Instant};

    fn project_snapshot(session: &str, source: &str, revision: u64) -> SourceSnapshotV2 {
        SourceSnapshotV2 {
            protocol_version: SOURCE_SNAPSHOT_PROTOCOL_VERSION,
            observation_session_id: session.to_string(),
            source_id: source.to_string(),
            parent_source_id: Some("corpus".to_string()),
            source_role: SnapshotSourceRoleV2::Project,
            revision,
            status: SnapshotSourceStatusV2::Running,
            stage: SnapshotStageV2::ProjectArrangementSolver,
            activity: None,
            project_label: Some("Project Label".to_string()),
            progress: None,
            started_unix_millis: 1_000,
            terminal_duration_millis: None,
            outcome_color: None,
        }
    }

    #[test]
    fn higher_revision_wins_and_older_datagram_is_ignored() {
        let mut aggregator = SessionAggregator::new();
        assert_eq!(
            aggregator.ingest(project_snapshot("session-a", "project:6", 2)),
            SnapshotIngestOutcome::NewSource
        );
        assert_eq!(
            aggregator.ingest(project_snapshot("session-a", "project:6", 1)),
            SnapshotIngestOutcome::StaleIgnored
        );
        assert_eq!(
            aggregator
                .source("session-a", "project:6")
                .expect("stored source")
                .snapshot()
                .revision,
            2
        );
    }

    #[test]
    fn skipped_revision_does_not_prevent_convergence_to_latest_state() {
        let mut aggregator = SessionAggregator::new();
        assert_eq!(
            aggregator.ingest(project_snapshot("session-a", "project:6", 1)),
            SnapshotIngestOutcome::NewSource
        );
        assert_eq!(
            aggregator.ingest(project_snapshot("session-a", "project:6", 3)),
            SnapshotIngestOutcome::Advanced
        );
        assert_eq!(
            aggregator
                .source("session-a", "project:6")
                .expect("stored source")
                .snapshot()
                .revision,
            3
        );
    }

    #[test]
    fn reannouncement_updates_last_seen_without_changing_state_age() {
        let mut aggregator = SessionAggregator::new();
        let first = Instant::now();
        let second = first + Duration::from_secs(3);
        let snapshot = project_snapshot("session-a", "project:6", 4);
        assert_eq!(
            aggregator.ingest_at(snapshot.clone(), first),
            SnapshotIngestOutcome::NewSource
        );
        assert_eq!(
            aggregator.ingest_at(snapshot, second),
            SnapshotIngestOutcome::Reannounced
        );
        let source = aggregator
            .source("session-a", "project:6")
            .expect("stored source");
        assert_eq!(source.last_seen(), second);
        assert_eq!(source.last_state_change(), first);
    }

    #[test]
    fn same_revision_with_different_payload_is_rejected() {
        let mut aggregator = SessionAggregator::new();
        let original = project_snapshot("session-a", "project:6", 7);
        assert_eq!(
            aggregator.ingest(original.clone()),
            SnapshotIngestOutcome::NewSource
        );
        let mut conflicting = original.clone();
        conflicting.status = SnapshotSourceStatusV2::Completed;
        assert_eq!(
            aggregator.ingest(conflicting),
            SnapshotIngestOutcome::SameRevisionConflict
        );
        assert_eq!(
            aggregator
                .source("session-a", "project:6")
                .expect("stored source")
                .snapshot(),
            &original
        );
    }

    #[test]
    fn source_identity_cannot_change_inside_same_session_source_address() {
        let mut aggregator = SessionAggregator::new();
        let original = project_snapshot("session-a", "project:6", 1);
        assert_eq!(
            aggregator.ingest(original.clone()),
            SnapshotIngestOutcome::NewSource
        );
        let mut conflicting = project_snapshot("session-a", "project:6", 2);
        conflicting.parent_source_id = Some("launcher".to_string());
        assert_eq!(
            aggregator.ingest(conflicting),
            SnapshotIngestOutcome::SourceIdentityConflict
        );
        assert_eq!(
            aggregator
                .source("session-a", "project:6")
                .expect("stored source")
                .snapshot(),
            &original
        );
    }

    #[test]
    fn sessions_remain_independent() {
        let mut aggregator = SessionAggregator::new();
        assert_eq!(
            aggregator.ingest(project_snapshot("session-a", "project:1", 1)),
            SnapshotIngestOutcome::NewSource
        );
        assert_eq!(
            aggregator.ingest(project_snapshot("session-b", "project:1", 1)),
            SnapshotIngestOutcome::NewSource
        );
        assert_eq!(aggregator.session_count(), 2);
        assert_eq!(aggregator.source_count("session-a"), 1);
        assert_eq!(aggregator.source_count("session-b"), 1);
    }

    #[test]
    fn session_snapshot_is_deterministic_and_keeps_source_freshness_separate() {
        let mut aggregator = SessionAggregator::new();
        let first = Instant::now();
        let second = first + Duration::from_secs(2);
        let third = first + Duration::from_secs(5);

        assert_eq!(
            aggregator.ingest_at(project_snapshot("session-a", "project:6", 1), first),
            SnapshotIngestOutcome::NewSource
        );
        assert_eq!(
            aggregator.ingest_at({
                let mut snapshot = project_snapshot("session-a", "project:2", 1);
                snapshot.started_unix_millis = 3_000;
                snapshot
            }, second),
            SnapshotIngestOutcome::NewSource
        );
        assert_eq!(
            aggregator.ingest_at(project_snapshot("session-a", "project:6", 1), third),
            SnapshotIngestOutcome::Reannounced
        );

        let snapshot = aggregator
            .session_snapshot_v2_at("session-a", third, 6_000)
            .expect("session snapshot");
        assert_eq!(snapshot.protocol_version, SESSION_SNAPSHOT_PROTOCOL_VERSION);
        assert_eq!(snapshot.observation_session_id, "session-a");
        assert_eq!(snapshot.sources.len(), 2);
        assert_eq!(snapshot.sources[0].source.source_id, "project:2");
        assert_eq!(snapshot.sources[1].source.source_id, "project:6");
        assert_eq!(snapshot.sources[0].last_seen_age_millis, 3_000);
        assert_eq!(snapshot.sources[0].last_state_change_age_millis, 3_000);
        assert_eq!(snapshot.sources[0].run_elapsed_millis, 3_000);
        assert_eq!(snapshot.sources[1].last_seen_age_millis, 0);
        assert_eq!(snapshot.sources[1].last_state_change_age_millis, 5_000);
        assert_eq!(snapshot.sources[1].run_elapsed_millis, 5_000);
    }

    #[test]
    fn run_elapsed_freezes_at_first_terminal_transition() {
        let mut aggregator = SessionAggregator::new();
        let first = Instant::now();
        let terminal = first + Duration::from_secs(7);
        let later = first + Duration::from_secs(20);
        let running = project_snapshot("session-a", "project:6", 1);
        assert_eq!(
            aggregator.ingest_at(running, first),
            SnapshotIngestOutcome::NewSource
        );
        let mut completed = project_snapshot("session-a", "project:6", 2);
        completed.status = SnapshotSourceStatusV2::Completed;
        completed.terminal_duration_millis = Some(7_000);
        assert_eq!(
            aggregator.ingest_at(completed, terminal),
            SnapshotIngestOutcome::Advanced
        );
        let snapshot = aggregator
            .session_snapshot_v2_at("session-a", later, 21_000)
            .expect("session snapshot");
        assert_eq!(snapshot.sources[0].run_elapsed_millis, 7_000);
        assert_eq!(snapshot.sources[0].last_state_change_age_millis, 13_000);
    }

    #[test]
    fn all_session_snapshots_are_projected_in_deterministic_session_order() {
        let mut aggregator = SessionAggregator::new();
        aggregator.ingest(project_snapshot("session-z", "project:1", 1));
        aggregator.ingest(project_snapshot("session-a", "project:1", 1));

        let snapshots = aggregator.session_snapshots_v2();
        assert_eq!(snapshots.len(), 2);
        assert_eq!(snapshots[0].observation_session_id, "session-a");
        assert_eq!(snapshots[1].observation_session_id, "session-z");
    }

    #[test]
    fn session_snapshot_does_not_invent_unobserved_sources() {
        let mut aggregator = SessionAggregator::new();
        aggregator.ingest(project_snapshot("session-a", "project:6", 4));
        let snapshot = aggregator
            .session_snapshot_v2("session-a")
            .expect("partial session snapshot");
        assert_eq!(snapshot.sources.len(), 1);
        assert_eq!(snapshot.sources[0].source.source_id, "project:6");
        assert!(aggregator.session_snapshot_v2("missing-session").is_none());
    }

}
