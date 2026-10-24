use crate::observation::SourceStateOwner;
use crate::observation_protocol::{
    SOURCE_SNAPSHOT_LOOPBACK_PORT, SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES,
};
use crate::observation_protocol_encoder::encode_source_snapshot_v2;
use crate::observation_snapshot_projection::project_source_snapshot_v2;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

const SOURCE_SNAPSHOT_REANNOUNCE_INTERVAL: Duration = Duration::from_secs(1);

pub struct SnapshotDatagramPublisher {
    immediate_socket: Option<UdpSocket>,
    target: SocketAddr,
    latest: Arc<Mutex<Vec<u8>>>,
}

impl SnapshotDatagramPublisher {
    pub fn local(owner: &SourceStateOwner) -> Self {
        Self::new_with_target(
            owner,
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::LOCALHOST,
                SOURCE_SNAPSHOT_LOOPBACK_PORT,
            )),
            SOURCE_SNAPSHOT_REANNOUNCE_INTERVAL,
        )
    }

    pub fn publish_current(&self, owner: &SourceStateOwner) {
        let Some(bytes) = encoded_snapshot(owner) else {
            return;
        };
        {
            let mut latest = self
                .latest
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if *latest != bytes {
                *latest = bytes.clone();
            }
        }
        if let Some(socket) = &self.immediate_socket {
            let _ = socket.send_to(&bytes, self.target);
        }
    }

    fn new_with_target(
        owner: &SourceStateOwner,
        target: SocketAddr,
        reannounce_interval: Duration,
    ) -> Self {
        let latest = Arc::new(Mutex::new(encoded_snapshot(owner).unwrap_or_default()));
        let immediate_socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .ok()
            .and_then(|socket| socket.set_nonblocking(true).ok().map(|()| socket));

        if let Some(socket) = &immediate_socket {
            let initial = latest
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .clone();
            if !initial.is_empty() {
                let _ = socket.send_to(&initial, target);
            }
            if let Ok(reannounce_socket) = socket.try_clone() {
                spawn_reannouncement(
                    Arc::downgrade(&latest),
                    reannounce_socket,
                    target,
                    reannounce_interval,
                );
            }
        }

        Self {
            immediate_socket,
            target,
            latest,
        }
    }
}

fn encoded_snapshot(owner: &SourceStateOwner) -> Option<Vec<u8>> {
    encode_source_snapshot_v2(&project_source_snapshot_v2(owner))
        .ok()
        .filter(|bytes| bytes.len() <= SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES)
}

fn spawn_reannouncement(
    latest: Weak<Mutex<Vec<u8>>>,
    socket: UdpSocket,
    target: SocketAddr,
    interval: Duration,
) {
    drop(
        std::thread::Builder::new()
            .name("etuschlopf-observation-reannounce".to_string())
            .spawn(move || loop {
                std::thread::sleep(interval);
                let Some(latest) = latest.upgrade() else {
                    break;
                };
                let bytes = latest
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .clone();
                if !bytes.is_empty() {
                    let _ = socket.send_to(&bytes, target);
                }
            }),
    );
}

#[cfg(test)]
mod tests {
    use super::SnapshotDatagramPublisher;
    use crate::observation_protocol::SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES;
    use crate::observation::{ProjectStage, SourceStateOwner};
    use crate::observation_protocol::SourceSnapshotV2;
    use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
    use std::time::Duration;

    fn receive_snapshot(receiver: &UdpSocket) -> SourceSnapshotV2 {
        let mut buffer = vec![0_u8; SOURCE_SNAPSHOT_MAX_DATAGRAM_BYTES];
        let (length, _) = receiver.recv_from(&mut buffer).expect("receive snapshot");
        SourceSnapshotV2::decode_v2(&buffer[..length]).expect("decode snapshot")
    }

    #[test]
    fn absent_receiver_cannot_turn_observation_into_pipeline_failure() {
        let probe = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .expect("bind temporary receiver");
        let target = probe.local_addr().expect("temporary receiver address");
        drop(probe);

        let mut owner = SourceStateOwner::project(Some("Synthetic Project Nr. 6".to_string()));
        let publisher =
            SnapshotDatagramPublisher::new_with_target(&owner, target, Duration::from_millis(10));
        owner.enter_project_stage(ProjectStage::ArrangementSolver);
        publisher.publish_current(&owner);
        owner.mark_completed();
        publisher.publish_current(&owner);
    }

    #[test]
    fn latest_snapshot_is_reannounced_without_revision_change() {
        let receiver = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .expect("bind receiver");
        receiver
            .set_read_timeout(Some(Duration::from_millis(500)))
            .expect("set receiver timeout");
        let target = receiver.local_addr().expect("receiver address");

        let mut owner = SourceStateOwner::project(Some("Synthetic Project Nr. 6".to_string()));
        let publisher =
            SnapshotDatagramPublisher::new_with_target(&owner, target, Duration::from_millis(20));
        let initial = receive_snapshot(&receiver);
        assert_eq!(initial.revision, 1);

        owner.enter_project_stage(ProjectStage::InitialRenderTransaction);
        publisher.publish_current(&owner);

        let changed = loop {
            let snapshot = receive_snapshot(&receiver);
            if snapshot.revision == 2 {
                break snapshot;
            }
        };
        let reannounced = loop {
            let snapshot = receive_snapshot(&receiver);
            if snapshot.revision == changed.revision {
                break snapshot;
            }
        };

        assert_eq!(reannounced, changed);
    }
}
