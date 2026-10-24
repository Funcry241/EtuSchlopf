use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const MQTT_LOOPBACK_PORT: u16 = 47_622;
const MQTT_MAX_PACKET_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone)]
struct BrokerPublication {
    topic: String,
    payload: Vec<u8>,
    retained: bool,
}

#[derive(Debug)]
struct ClientRegistration {
    id: u64,
    subscriptions: Arc<Mutex<Vec<String>>>,
    sender: mpsc::Sender<BrokerPublication>,
}

#[derive(Debug, Default)]
struct BrokerState {
    retained: BTreeMap<String, Vec<u8>>,
    clients: Vec<ClientRegistration>,
    next_client_id: u64,
}

impl BrokerState {
    fn register_client(
        &mut self,
        subscriptions: Arc<Mutex<Vec<String>>>,
        sender: mpsc::Sender<BrokerPublication>,
    ) -> u64 {
        loop {
            let id = self.next_client_id;
            self.next_client_id = self.next_client_id.wrapping_add(1);
            if self.clients.iter().any(|client| client.id == id) {
                continue;
            }
            self.clients.push(ClientRegistration {
                id,
                subscriptions,
                sender,
            });
            return id;
        }
    }

    fn unregister_client(&mut self, id: u64) {
        self.clients.retain(|client| client.id != id);
    }
}

struct ClientRegistrationGuard {
    state: Arc<Mutex<BrokerState>>,
    client_id: u64,
}

impl ClientRegistrationGuard {
    fn new(state: Arc<Mutex<BrokerState>>, client_id: u64) -> Self {
        Self { state, client_id }
    }
}

impl Drop for ClientRegistrationGuard {
    fn drop(&mut self) {
        let mut broker = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        broker.unregister_client(self.client_id);
    }
}

pub struct EmbeddedMqttBroker {
    local_addr: SocketAddr,
    state: Arc<Mutex<BrokerState>>,
    stop: Arc<AtomicBool>,
    listener_thread: Option<thread::JoinHandle<()>>,
}

impl EmbeddedMqttBroker {
    pub fn start_loopback(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port))?;
        listener.set_nonblocking(true)?;
        let local_addr = listener.local_addr()?;
        let state = Arc::new(Mutex::new(BrokerState::default()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_state = Arc::clone(&state);
        let thread_stop = Arc::clone(&stop);
        let listener_thread = thread::Builder::new()
            .name("etuschlopf-watch-mqtt-listener".to_string())
            .spawn(move || accept_loop(listener, thread_state, thread_stop))?;

        Ok(Self {
            local_addr,
            state,
            stop,
            listener_thread: Some(listener_thread),
        })
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.local_addr)
    }

    #[cfg(test)]
    fn registered_client_count(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clients
            .len()
    }

    #[cfg(test)]
    fn registered_subscription_count(&self) -> usize {
        let state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state
            .clients
            .iter()
            .map(|client| {
                client
                    .subscriptions
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .len()
            })
            .sum()
    }

    pub fn publish_retained(&self, topic: String, payload: Vec<u8>) {
        let publication = BrokerPublication {
            topic: topic.clone(),
            payload: payload.clone(),
            retained: true,
        };
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.retained.insert(topic, payload);
        state.clients.retain(|client| {
            let subscriptions = client
                .subscriptions
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let interested = subscriptions
                .iter()
                .any(|filter| topic_filter_matches(filter, &publication.topic));
            !interested || client.sender.send(publication.clone()).is_ok()
        });
    }
}

impl Drop for EmbeddedMqttBroker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(listener_thread) = self.listener_thread.take() {
            let _ = listener_thread.join();
        }
    }
}

fn accept_loop(listener: TcpListener, state: Arc<Mutex<BrokerState>>, stop: Arc<AtomicBool>) {
    while !stop.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, peer)) if peer.ip().is_loopback() => {
                let client_state = Arc::clone(&state);
                let _ = thread::Builder::new()
                    .name("etuschlopf-watch-mqtt-client".to_string())
                    .spawn(move || {
                        let _ = serve_client(stream, client_state);
                    });
            }
            Ok((_stream, _peer)) => {}
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => break,
        }
    }
}

fn serve_client(mut stream: TcpStream, state: Arc<Mutex<BrokerState>>) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;
    stream.set_write_timeout(Some(Duration::from_secs(1)))?;

    let Some((fixed_header, connect_payload)) = read_packet(&mut stream)? else {
        return Ok(());
    };
    if fixed_header >> 4 != 1 || !valid_connect_v311(&connect_payload) {
        return Ok(());
    }
    stream.write_all(&[0x20, 0x02, 0x00, 0x00])?;

    let subscriptions = Arc::new(Mutex::new(Vec::<String>::new()));
    let (sender, receiver) = mpsc::channel::<BrokerPublication>();
    let client_id = {
        let mut broker = state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        broker.register_client(Arc::clone(&subscriptions), sender)
    };
    let _registration = ClientRegistrationGuard::new(Arc::clone(&state), client_id);

    loop {
        while let Ok(publication) = receiver.try_recv() {
            write_publish(&mut stream, &publication)?;
        }

        match read_packet(&mut stream) {
            Ok(Some((fixed_header, payload))) => {
                match fixed_header >> 4 {
                    8 if fixed_header & 0x0f == 0x02 => {
                        let (packet_id, filters) = parse_subscribe(&payload)?;
                        {
                            let mut current = subscriptions
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner);
                            for filter in &filters {
                                if !current.contains(filter) {
                                    current.push(filter.clone());
                                }
                            }
                        }
                        write_suback(&mut stream, packet_id, filters.len())?;
                        let retained = retained_for_filters(&state, &filters);
                        for publication in retained {
                            write_publish(&mut stream, &publication)?;
                        }
                    }
                    12 => stream.write_all(&[0xd0, 0x00])?,
                    14 => return Ok(()),
                    3 => return Ok(()),
                    _ => {}
                }
            }
            Ok(None) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::UnexpectedEof
                        | io::ErrorKind::ConnectionReset
                        | io::ErrorKind::ConnectionAborted
                        | io::ErrorKind::BrokenPipe
                ) =>
            {
                return Ok(());
            }
            Err(error) => return Err(error),
        }
    }
}

fn retained_for_filters(
    state: &Arc<Mutex<BrokerState>>,
    filters: &[String],
) -> Vec<BrokerPublication> {
    let broker = state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut topics = BTreeSet::<String>::new();
    for topic in broker.retained.keys() {
        if filters
            .iter()
            .any(|filter| topic_filter_matches(filter, topic))
        {
            topics.insert(topic.clone());
        }
    }
    topics
        .into_iter()
        .filter_map(|topic| {
            broker.retained.get(&topic).map(|payload| BrokerPublication {
                topic,
                payload: payload.clone(),
                retained: true,
            })
        })
        .collect()
}

fn valid_connect_v311(payload: &[u8]) -> bool {
    let mut cursor = 0usize;
    let Ok(protocol_name) = take_mqtt_string(payload, &mut cursor) else {
        return false;
    };
    if protocol_name != "MQTT" {
        return false;
    }
    let Some(protocol_level) = payload.get(cursor).copied() else {
        return false;
    };
    cursor = cursor.saturating_add(1);
    if protocol_level != 4 {
        return false;
    }
    let Some(connect_flags) = payload.get(cursor).copied() else {
        return false;
    };
    cursor = cursor.saturating_add(1);
    if connect_flags & 0x01 != 0
        || connect_flags & 0x02 == 0
        || payload.len().saturating_sub(cursor) < 2
    {
        return false;
    }
    cursor = cursor.saturating_add(2);
    take_mqtt_string(payload, &mut cursor).is_ok()
}

fn parse_subscribe(payload: &[u8]) -> io::Result<(u16, Vec<String>)> {
    if payload.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MQTT SUBSCRIBE without packet id",
        ));
    }
    let packet_id = u16::from_be_bytes([payload[0], payload[1]]);
    let mut cursor = 2usize;
    let mut filters = Vec::new();
    while cursor < payload.len() {
        let filter = take_mqtt_string(payload, &mut cursor)?.to_string();
        let Some(requested_qos) = payload.get(cursor).copied() else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "MQTT SUBSCRIBE without requested QoS",
            ));
        };
        cursor = cursor.saturating_add(1);
        if requested_qos > 1 || filter.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unsupported MQTT subscription",
            ));
        }
        filters.push(filter);
    }
    if filters.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "empty MQTT SUBSCRIBE",
        ));
    }
    Ok((packet_id, filters))
}

fn write_suback(stream: &mut TcpStream, packet_id: u16, subscriptions: usize) -> io::Result<()> {
    let remaining = 2usize.saturating_add(subscriptions);
    let mut packet = Vec::with_capacity(2 + remaining);
    packet.push(0x90);
    push_remaining_length(&mut packet, remaining)?;
    packet.extend_from_slice(&packet_id.to_be_bytes());
    packet.extend(std::iter::repeat(0x00).take(subscriptions));
    stream.write_all(&packet)
}

fn write_publish(stream: &mut TcpStream, publication: &BrokerPublication) -> io::Result<()> {
    let topic = publication.topic.as_bytes();
    let remaining = 2usize
        .saturating_add(topic.len())
        .saturating_add(publication.payload.len());
    let mut packet = Vec::with_capacity(5 + remaining);
    packet.push(if publication.retained { 0x31 } else { 0x30 });
    push_remaining_length(&mut packet, remaining)?;
    let topic_len = u16::try_from(topic.len()).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "MQTT topic exceeds u16")
    })?;
    packet.extend_from_slice(&topic_len.to_be_bytes());
    packet.extend_from_slice(topic);
    packet.extend_from_slice(&publication.payload);
    stream.write_all(&packet)
}

fn read_packet(stream: &mut TcpStream) -> io::Result<Option<(u8, Vec<u8>)>> {
    let mut fixed_header = [0_u8; 1];
    match stream.read_exact(&mut fixed_header) {
        Ok(()) => {}
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
            ) =>
        {
            return Ok(None);
        }
        Err(error) => return Err(error),
    }

    let remaining = read_remaining_length(stream)?;
    if remaining > MQTT_MAX_PACKET_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MQTT packet exceeds local Watch limit",
        ));
    }
    let mut payload = vec![0_u8; remaining];
    stream.read_exact(&mut payload)?;
    Ok(Some((fixed_header[0], payload)))
}

fn read_remaining_length(stream: &mut TcpStream) -> io::Result<usize> {
    let mut multiplier = 1usize;
    let mut value = 0usize;
    for _ in 0..4 {
        let mut encoded = [0_u8; 1];
        stream.read_exact(&mut encoded)?;
        value = value.saturating_add(usize::from(encoded[0] & 0x7f).saturating_mul(multiplier));
        if encoded[0] & 0x80 == 0 {
            return Ok(value);
        }
        multiplier = multiplier.saturating_mul(128);
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "invalid MQTT remaining length",
    ))
}

fn push_remaining_length(packet: &mut Vec<u8>, mut remaining: usize) -> io::Result<()> {
    if remaining > 268_435_455 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "MQTT remaining length exceeds protocol limit",
        ));
    }
    loop {
        let mut encoded = u8::try_from(remaining % 128).unwrap_or(0);
        remaining /= 128;
        if remaining != 0 {
            encoded |= 0x80;
        }
        packet.push(encoded);
        if remaining == 0 {
            return Ok(());
        }
    }
}

fn take_mqtt_string<'a>(payload: &'a [u8], cursor: &mut usize) -> io::Result<&'a str> {
    if payload.len().saturating_sub(*cursor) < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "truncated MQTT string length",
        ));
    }
    let length = usize::from(u16::from_be_bytes([
        payload[*cursor],
        payload[*cursor + 1],
    ]));
    *cursor = (*cursor).saturating_add(2);
    let end = (*cursor).saturating_add(length);
    let bytes = payload.get(*cursor..end).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "truncated MQTT string")
    })?;
    *cursor = end;
    std::str::from_utf8(bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid MQTT UTF-8"))
}

fn topic_filter_matches(filter: &str, topic: &str) -> bool {
    let filter_levels = filter.split('/').collect::<Vec<_>>();
    let topic_levels = topic.split('/').collect::<Vec<_>>();
    let mut topic_index = 0usize;
    for (filter_index, filter_level) in filter_levels.iter().enumerate() {
        if *filter_level == "#" {
            return filter_index + 1 == filter_levels.len();
        }
        let Some(topic_level) = topic_levels.get(topic_index) else {
            return false;
        };
        if *filter_level != "+" && *filter_level != *topic_level {
            return false;
        }
        topic_index = topic_index.saturating_add(1);
    }
    topic_index == topic_levels.len()
}

#[cfg(test)]
mod tests {
    use super::{read_packet, valid_connect_v311, EmbeddedMqttBroker};
    use std::io::{Read, Write};
    use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
    use std::thread;
    use std::time::{Duration, Instant};

    fn push_string(packet: &mut Vec<u8>, value: &str) {
        let length = u16::try_from(value.len()).expect("test MQTT string length");
        packet.extend_from_slice(&length.to_be_bytes());
        packet.extend_from_slice(value.as_bytes());
    }

    fn connect_payload(connect_flags: u8) -> Vec<u8> {
        let mut variable = Vec::new();
        push_string(&mut variable, "MQTT");
        variable.push(4);
        variable.push(connect_flags);
        variable.extend_from_slice(&30_u16.to_be_bytes());
        push_string(&mut variable, "etuschlopf-watch-test");
        variable
    }

    fn connect_packet() -> Vec<u8> {
        let variable = connect_payload(0x02);
        let mut packet = vec![0x10, u8::try_from(variable.len()).expect("short CONNECT")];
        packet.extend_from_slice(&variable);
        packet
    }

    fn subscribe_packet(filter: &str) -> Vec<u8> {
        let mut variable = Vec::new();
        variable.extend_from_slice(&1_u16.to_be_bytes());
        push_string(&mut variable, filter);
        variable.push(0);
        let mut packet = vec![0x82, u8::try_from(variable.len()).expect("short SUBSCRIBE")];
        packet.extend_from_slice(&variable);
        packet
    }

    fn connect_and_subscribe(broker: &EmbeddedMqttBroker) -> TcpStream {
        let address = broker.local_addr().expect("broker address");
        let mut client = TcpStream::connect(SocketAddrV4::new(Ipv4Addr::LOCALHOST, address.port()))
            .expect("connect MQTT client");
        client
            .set_read_timeout(Some(Duration::from_secs(1)))
            .expect("set read timeout");
        client.write_all(&connect_packet()).expect("send CONNECT");
        let mut connack = [0_u8; 4];
        client.read_exact(&mut connack).expect("read CONNACK");
        assert_eq!(connack, [0x20, 0x02, 0x00, 0x00]);
        client
            .write_all(&subscribe_packet("etuschlopf/v2/session/+/state"))
            .expect("send SUBSCRIBE");
        let mut suback = [0_u8; 5];
        client.read_exact(&mut suback).expect("read SUBACK");
        assert_eq!(suback, [0x90, 0x03, 0x00, 0x01, 0x00]);
        client
    }

    fn wait_for_registration_counts(
        broker: &EmbeddedMqttBroker,
        expected_clients: usize,
        expected_subscriptions: usize,
    ) {
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let clients = broker.registered_client_count();
            let subscriptions = broker.registered_subscription_count();
            if clients == expected_clients && subscriptions == expected_subscriptions {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "MQTT registration counts did not converge: clients={clients}, subscriptions={subscriptions}"
            );
            thread::sleep(Duration::from_millis(10));
        }
    }

    #[test]
    fn broker_is_loopback_only_and_delivers_retained_session_state_to_wildcard_subscriber() {
        let broker = EmbeddedMqttBroker::start_loopback(0).expect("start broker");
        let address = broker.local_addr().expect("broker address");
        assert_eq!(address.ip(), Ipv4Addr::LOCALHOST);

        broker.publish_retained(
            "etuschlopf/v2/session/obs-1/state".to_string(),
            br#"{"protocol_version":1}"#.to_vec(),
        );

        let mut client = TcpStream::connect(SocketAddrV4::new(Ipv4Addr::LOCALHOST, address.port()))
            .expect("connect MQTT client");
        client
            .set_read_timeout(Some(Duration::from_secs(1)))
            .expect("set read timeout");
        client.write_all(&connect_packet()).expect("send CONNECT");

        let mut connack = [0_u8; 4];
        client.read_exact(&mut connack).expect("read CONNACK");
        assert_eq!(connack, [0x20, 0x02, 0x00, 0x00]);

        client
            .write_all(&subscribe_packet("etuschlopf/v2/session/+/state"))
            .expect("send SUBSCRIBE");

        let mut suback = [0_u8; 5];
        client.read_exact(&mut suback).expect("read SUBACK");
        assert_eq!(suback, [0x90, 0x03, 0x00, 0x01, 0x00]);

        let (header, payload) = read_packet(&mut client)
            .expect("read retained PUBLISH")
            .expect("retained packet");
        assert_eq!(header, 0x31);
        let topic_len = usize::from(u16::from_be_bytes([payload[0], payload[1]]));
        let topic = std::str::from_utf8(&payload[2..2 + topic_len]).expect("topic UTF-8");
        assert_eq!(topic, "etuschlopf/v2/session/obs-1/state");
        assert_eq!(&payload[2 + topic_len..], br#"{"protocol_version":1}"#);
    }

    #[test]
    fn broker_requires_clean_session_for_ephemeral_watch_clients() {
        assert!(valid_connect_v311(&connect_payload(0x02)));
        assert!(!valid_connect_v311(&connect_payload(0x00)));
    }

    #[test]
    fn disconnect_removes_client_registration_and_all_subscriptions_immediately() {
        let broker = EmbeddedMqttBroker::start_loopback(0).expect("start broker");
        let mut client = connect_and_subscribe(&broker);
        wait_for_registration_counts(&broker, 1, 1);

        client
            .write_all(&[0xe0, 0x00])
            .expect("send MQTT DISCONNECT");
        wait_for_registration_counts(&broker, 0, 0);
    }

    #[test]
    fn connection_loss_removes_client_registration_and_all_subscriptions() {
        let broker = EmbeddedMqttBroker::start_loopback(0).expect("start broker");
        let client = connect_and_subscribe(&broker);
        wait_for_registration_counts(&broker, 1, 1);

        drop(client);
        wait_for_registration_counts(&broker, 0, 0);
    }
}
