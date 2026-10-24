#![forbid(unsafe_code)]

#[path = "../../../src/observation_protocol.rs"]
mod observation_protocol;
#[path = "../../../src/observation_receiver.rs"]
mod observation_receiver;
#[path = "../../../src/observation_session_aggregator.rs"]
mod observation_session_aggregator;

mod mqtt_broker;
mod mqtt_client;
mod session_json;
mod watch_ui;
mod web_watch;

use mqtt_broker::{EmbeddedMqttBroker, MQTT_LOOPBACK_PORT};
use observation_receiver::HeadlessSnapshotDatagramReceiver;
use observation_session_aggregator::SessionAggregator;
use session_json::{encode_session_snapshot_json_v2, mqtt_topic_for_session};
use std::collections::BTreeMap;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    if let Err(error) = run() {
        eprintln!("EtuSchlopf Watch: {error}");
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let receiver = HeadlessSnapshotDatagramReceiver::bind_local()?;
    let broker = EmbeddedMqttBroker::start_loopback(MQTT_LOOPBACK_PORT)?;
    let broker_address = broker.local_addr()?;
    let active_session_id = std::env::var(observation_protocol::WATCH_ACTIVE_SESSION_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let fallback_session_id = active_session_id.clone();
    let _ui_thread = match web_watch::spawn_mqtt_web_ui(broker_address, active_session_id) {
        Ok(thread) => thread,
        Err(error) if std::env::var_os("ETUSCHLOPF_WATCH_WEB_ONLY").is_some() => {
            return Err(error);
        }
        Err(_) => watch_ui::spawn_mqtt_ui(broker_address, fallback_session_id)?,
    };
    let mut aggregator = SessionAggregator::new();
    let mut last_published = BTreeMap::<String, Vec<u8>>::new();

    loop {
        let report = receiver.drain_available(&mut aggregator)?;
        if report.datagrams_seen != 0 {
            publish_current_sessions(&broker, &aggregator, &mut last_published);
        }
        thread::sleep(Duration::from_millis(25));
    }
}

fn publish_current_sessions(
    broker: &EmbeddedMqttBroker,
    aggregator: &SessionAggregator,
    last_published: &mut BTreeMap<String, Vec<u8>>,
) {
    for snapshot in aggregator.session_snapshots_v2() {
        let Some(topic) = mqtt_topic_for_session(&snapshot.observation_session_id) else {
            continue;
        };
        let payload = encode_session_snapshot_json_v2(&snapshot).into_bytes();
        if last_published.get(&topic).is_some_and(|current| current == &payload) {
            continue;
        }
        broker.publish_retained(topic.clone(), payload.clone());
        last_published.insert(topic, payload);
    }
}
