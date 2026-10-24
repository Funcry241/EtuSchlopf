use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

const MQTT_MAX_PACKET_BYTES: usize = 1024 * 1024;
const SESSION_STATE_FILTER: &str = "etuschlopf/v2/session/+/state";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MqttPublication {
    pub topic: String,
    pub payload: Vec<u8>,
}

pub struct MqttSessionStateClient {
    stream: TcpStream,
}

impl MqttSessionStateClient {
    pub fn connect(address: SocketAddr) -> io::Result<Self> {
        if !address.ip().is_loopback() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "EtuSchlopf Watch MQTT client requires loopback broker",
            ));
        }

        let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(1))?;
        stream.set_read_timeout(Some(Duration::from_millis(500)))?;
        stream.set_write_timeout(Some(Duration::from_secs(1)))?;
        stream.write_all(&connect_packet("etuschlopf-watch-ui")?)?;

        let Some((header, payload)) = read_packet(&mut stream)? else {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "MQTT CONNACK timeout"));
        };
        if header != 0x20 || payload.as_slice() != &[0x00, 0x00] {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionRefused,
                "MQTT broker rejected Watch UI connection",
            ));
        }

        stream.write_all(&subscribe_packet(1, SESSION_STATE_FILTER)?)?;
        let Some((header, payload)) = read_packet(&mut stream)? else {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "MQTT SUBACK timeout"));
        };
        if header != 0x90
            || payload.len() != 3
            || payload[0] != 0
            || payload[1] != 1
            || payload[2] != 0
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unexpected MQTT SUBACK for Watch UI",
            ));
        }

        Ok(Self { stream })
    }

    pub fn read_next(&mut self) -> io::Result<Option<MqttPublication>> {
        let Some((header, payload)) = read_packet(&mut self.stream)? else {
            return Ok(None);
        };
        if header >> 4 != 3 {
            return Ok(None);
        }
        parse_publish(&payload).map(Some)
    }
}

fn connect_packet(client_id: &str) -> io::Result<Vec<u8>> {
    let mut variable = Vec::new();
    push_mqtt_string(&mut variable, "MQTT")?;
    variable.push(4);
    variable.push(0x02);
    variable.extend_from_slice(&30_u16.to_be_bytes());
    push_mqtt_string(&mut variable, client_id)?;

    let mut packet = vec![0x10];
    push_remaining_length(&mut packet, variable.len())?;
    packet.extend_from_slice(&variable);
    Ok(packet)
}

fn subscribe_packet(packet_id: u16, filter: &str) -> io::Result<Vec<u8>> {
    let mut variable = Vec::new();
    variable.extend_from_slice(&packet_id.to_be_bytes());
    push_mqtt_string(&mut variable, filter)?;
    variable.push(0);

    let mut packet = vec![0x82];
    push_remaining_length(&mut packet, variable.len())?;
    packet.extend_from_slice(&variable);
    Ok(packet)
}

fn parse_publish(payload: &[u8]) -> io::Result<MqttPublication> {
    if payload.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MQTT PUBLISH without topic length",
        ));
    }
    let topic_len = usize::from(u16::from_be_bytes([payload[0], payload[1]]));
    let topic_end = 2usize.saturating_add(topic_len);
    let topic_bytes = payload.get(2..topic_end).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "truncated MQTT PUBLISH topic")
    })?;
    let topic = std::str::from_utf8(topic_bytes)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid MQTT topic UTF-8"))?
        .to_string();
    Ok(MqttPublication {
        topic,
        payload: payload.get(topic_end..).unwrap_or_default().to_vec(),
    })
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
            "MQTT packet exceeds Watch UI limit",
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

fn push_mqtt_string(packet: &mut Vec<u8>, value: &str) -> io::Result<()> {
    let length = u16::try_from(value.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "MQTT string exceeds u16"))?;
    packet.extend_from_slice(&length.to_be_bytes());
    packet.extend_from_slice(value.as_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{MqttSessionStateClient, SESSION_STATE_FILTER};
    use crate::mqtt_broker::EmbeddedMqttBroker;
    use std::net::Ipv4Addr;
    use std::time::Duration;

    #[test]
    fn ui_client_consumes_retained_session_state_through_public_mqtt_contract() {
        let broker = EmbeddedMqttBroker::start_loopback(0).expect("start broker");
        let address = broker.local_addr().expect("broker address");
        assert_eq!(address.ip(), Ipv4Addr::LOCALHOST);
        broker.publish_retained(
            "etuschlopf/v2/session/obs-ui/state".to_string(),
            br#"{"protocol_version":2,"observation_session_id":"obs-ui","sources":[]}"#
                .to_vec(),
        );

        let mut client = MqttSessionStateClient::connect(address).expect("connect Watch UI");
        let publication = (0..10)
            .find_map(|_| {
                client
                    .read_next()
                    .expect("read MQTT state")
                    .or_else(|| {
                        std::thread::sleep(Duration::from_millis(10));
                        None
                    })
            })
            .expect("retained session publication");

        assert_eq!(SESSION_STATE_FILTER, "etuschlopf/v2/session/+/state");
        assert_eq!(publication.topic, "etuschlopf/v2/session/obs-ui/state");
        assert!(publication.payload.windows(6).any(|window| window == b"obs-ui"));
    }
}
