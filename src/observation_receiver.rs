use crate::observation_protocol::{
    SourceSnapshotV2, SOURCE_SNAPSHOT_LOOPBACK_PORT, SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES,
};
use crate::observation_session_aggregator::{SessionAggregator, SnapshotIngestOutcome};
use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ReceiverDrainReport {
    pub datagrams_seen: usize,
    pub new_sources: usize,
    pub advanced: usize,
    pub reannounced: usize,
    pub stale_ignored: usize,
    pub semantic_conflicts: usize,
    pub malformed: usize,
}

pub struct HeadlessSnapshotDatagramReceiver {
    socket: UdpSocket,
}

impl HeadlessSnapshotDatagramReceiver {
    pub fn bind_local() -> io::Result<Self> {
        Self::bind_address(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::LOCALHOST,
            SOURCE_SNAPSHOT_LOOPBACK_PORT,
        )))
    }

    #[cfg(test)]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn drain_available(
        &self,
        aggregator: &mut SessionAggregator,
    ) -> io::Result<ReceiverDrainReport> {
        let mut report = ReceiverDrainReport::default();
        let mut buffer = [0_u8; SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES + 1];
        loop {
            match self.socket.recv_from(&mut buffer) {
                Ok((length, peer)) => {
                    report.datagrams_seen = report.datagrams_seen.saturating_add(1);
                    if length > SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES || !peer.ip().is_loopback() {
                        report.malformed = report.malformed.saturating_add(1);
                        continue;
                    }
                    let Ok(snapshot) = SourceSnapshotV2::decode_v2(&buffer[..length]) else {
                        report.malformed = report.malformed.saturating_add(1);
                        continue;
                    };
                    match aggregator.ingest(snapshot) {
                        SnapshotIngestOutcome::NewSource => {
                            report.new_sources = report.new_sources.saturating_add(1);
                        }
                        SnapshotIngestOutcome::Advanced => {
                            report.advanced = report.advanced.saturating_add(1);
                        }
                        SnapshotIngestOutcome::Reannounced => {
                            report.reannounced = report.reannounced.saturating_add(1);
                        }
                        SnapshotIngestOutcome::StaleIgnored => {
                            report.stale_ignored = report.stale_ignored.saturating_add(1);
                        }
                        SnapshotIngestOutcome::SameRevisionConflict
                        | SnapshotIngestOutcome::SourceIdentityConflict => {
                            report.semantic_conflicts = report.semantic_conflicts.saturating_add(1);
                        }
                    }
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => break,
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
        Ok(report)
    }

    fn bind_address(address: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(address)?;
        socket.set_nonblocking(true)?;
        Ok(Self { socket })
    }
}

#[cfg(test)]
mod tests {
    use super::HeadlessSnapshotDatagramReceiver;
    use crate::observation_protocol::{
        SnapshotSourceRoleV2, SnapshotSourceStatusV2, SnapshotStageV2, SourceSnapshotV2,
        SOURCE_SNAPSHOT_PROTOCOL_VERSION,
    };
    use crate::observation_session_aggregator::SessionAggregator;
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
    use std::thread;
    use std::time::{Duration, Instant};

    fn snapshot(revision: u64) -> SourceSnapshotV2 {
        SourceSnapshotV2 {
            protocol_version: SOURCE_SNAPSHOT_PROTOCOL_VERSION,
            observation_session_id: "session-a".to_string(),
            source_id: "project:6".to_string(),
            parent_source_id: Some("corpus".to_string()),
            source_role: SnapshotSourceRoleV2::Project,
            revision,
            status: SnapshotSourceStatusV2::Running,
            stage: SnapshotStageV2::ProjectInitialRenderTransaction,
            activity: None,
            project_label: Some("Project Label".to_string()),
            progress: None,
            started_unix_millis: 1_000,
            terminal_duration_millis: None,
            outcome_color: None,
        }
    }

    fn bind_ephemeral_receiver() -> HeadlessSnapshotDatagramReceiver {
        HeadlessSnapshotDatagramReceiver::bind_address(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::LOCALHOST,
            0,
        )))
        .expect("bind headless receiver")
    }

    fn send_snapshot(sender: &UdpSocket, target: SocketAddr, snapshot: &SourceSnapshotV2) {
        let bytes = receiver_test_datagram(snapshot);
        sender.send_to(&bytes, target).expect("send snapshot");
    }

    fn receiver_test_datagram(snapshot: &SourceSnapshotV2) -> Vec<u8> {
        assert_eq!(snapshot.protocol_version, SOURCE_SNAPSHOT_PROTOCOL_VERSION);
        assert_eq!(snapshot.source_role, SnapshotSourceRoleV2::Project);
        assert_eq!(snapshot.status, SnapshotSourceStatusV2::Running);
        assert_eq!(snapshot.stage, SnapshotStageV2::ProjectInitialRenderTransaction);
        assert_eq!(snapshot.activity, None);
        assert_eq!(snapshot.progress, None);

        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ETSOBS2\0");
        bytes.extend_from_slice(&snapshot.protocol_version.to_le_bytes());
        push_test_string(&mut bytes, &snapshot.observation_session_id);
        push_test_string(&mut bytes, &snapshot.source_id);
        push_test_optional_string(&mut bytes, snapshot.parent_source_id.as_deref());
        bytes.push(3);
        bytes.extend_from_slice(&snapshot.revision.to_le_bytes());
        bytes.push(1);
        bytes.push(19);
        bytes.push(0);
        push_test_optional_string(&mut bytes, snapshot.project_label.as_deref());
        bytes.push(0);
        bytes.extend_from_slice(&snapshot.started_unix_millis.to_le_bytes());
        bytes.push(0);
        bytes.push(0);
        bytes
    }

    fn push_test_string(bytes: &mut Vec<u8>, value: &str) {
        let length = u16::try_from(value.len()).expect("test string length");
        bytes.extend_from_slice(&length.to_le_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }

    fn push_test_optional_string(bytes: &mut Vec<u8>, value: Option<&str>) {
        match value {
            Some(value) => {
                bytes.push(1);
                push_test_string(bytes, value);
            }
            None => bytes.push(0),
        }
    }

    #[test]
    fn receiver_decodes_and_converges_when_datagrams_arrive_out_of_order() {
        let receiver = bind_ephemeral_receiver();
        let target = receiver.local_addr().expect("receiver address");
        let sender = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .expect("bind sender");
        send_snapshot(&sender, target, &snapshot(3));
        send_snapshot(&sender, target, &snapshot(2));

        let mut aggregator = SessionAggregator::new();
        let deadline = Instant::now() + Duration::from_millis(500);
        let mut seen = 0;
        while Instant::now() < deadline && seen < 2 {
            let report = receiver
                .drain_available(&mut aggregator)
                .expect("drain receiver");
            seen += report.datagrams_seen;
            if seen < 2 {
                thread::sleep(Duration::from_millis(5));
            }
        }

        assert_eq!(seen, 2);
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
    fn malformed_datagram_is_observed_but_does_not_poison_aggregator() {
        let receiver = bind_ephemeral_receiver();
        let target = receiver.local_addr().expect("receiver address");
        let sender = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .expect("bind sender");
        sender.send_to(b"not-a-snapshot", target).expect("send malformed");
        send_snapshot(&sender, target, &snapshot(1));

        let mut aggregator = SessionAggregator::new();
        let deadline = Instant::now() + Duration::from_millis(500);
        let mut malformed = 0;
        let mut accepted = 0;
        while Instant::now() < deadline && malformed + accepted < 2 {
            let report = receiver
                .drain_available(&mut aggregator)
                .expect("drain receiver");
            malformed += report.malformed;
            accepted += report.new_sources + report.advanced + report.reannounced;
            if malformed + accepted < 2 {
                thread::sleep(Duration::from_millis(5));
            }
        }

        assert_eq!(malformed, 1);
        assert_eq!(accepted, 1);
        assert_eq!(aggregator.source_count("session-a"), 1);
    }
}
