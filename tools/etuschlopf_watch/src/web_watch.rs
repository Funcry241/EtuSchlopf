use crate::mqtt_client::MqttSessionStateClient;
use crate::session_json::{decode_session_snapshot_json_v2, mqtt_topic_for_session};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

pub const WATCH_WEB_LOOPBACK_PORT: u16 = 47_623;
const MAX_HTTP_REQUEST_BYTES: usize = 8 * 1024;
const SSE_KEEPALIVE: Duration = Duration::from_secs(10);
const MQTT_RECONNECT_DELAY: Duration = Duration::from_millis(500);
const PDF_WAIT_POLL: Duration = Duration::from_millis(100);
const PDF_WAIT_TIMEOUT: Duration = Duration::from_secs(20 * 60);

#[derive(Clone, Copy, PartialEq, Eq)]
enum MqttBridgeState {
    Connecting,
    Live,
    Reconnecting,
}

impl MqttBridgeState {
    const fn label(self) -> &'static str {
        match self {
            Self::Connecting => "connecting",
            Self::Live => "live",
            Self::Reconnecting => "reconnecting",
        }
    }
}

struct WebState {
    generation: u64,
    mqtt: MqttBridgeState,
    current_session_id: Option<String>,
    sessions: BTreeMap<String, Vec<u8>>,
}

struct SharedWebState {
    state: Mutex<WebState>,
    changed: Condvar,
}

impl SharedWebState {
    fn new(current_session_id: Option<String>) -> Self {
        Self {
            state: Mutex::new(WebState {
                generation: 1,
                mqtt: MqttBridgeState::Connecting,
                current_session_id,
                sessions: BTreeMap::new(),
            }),
            changed: Condvar::new(),
        }
    }

    fn set_mqtt(&self, mqtt: MqttBridgeState) {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if state.mqtt == mqtt {
            return;
        }
        state.mqtt = mqtt;
        state.generation = state.generation.saturating_add(1);
        self.changed.notify_all();
    }

    fn put_session(&self, session_id: String, payload: Vec<u8>) {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        match state.current_session_id.as_deref() {
            Some(current) if current != session_id => return,
            None => {
                state.current_session_id = Some(session_id.clone());
                state.sessions.clear();
            }
            _ => {}
        }
        if state
            .sessions
            .get(&session_id)
            .is_some_and(|current| current == &payload)
        {
            return;
        }
        state.sessions.insert(session_id, payload);
        state.generation = state.generation.saturating_add(1);
        self.changed.notify_all();
    }

    fn snapshot(&self) -> (u64, String) {
        let state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        (state.generation, encode_browser_state(&state))
    }
}

pub fn spawn_mqtt_web_ui(
    mqtt_address: SocketAddr,
    current_session_id: Option<String>,
) -> io::Result<thread::JoinHandle<()>> {
    let project_root = Arc::new(std::env::current_dir()?);
    let watch_started_at = SystemTime::now();
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, WATCH_WEB_LOOPBACK_PORT))?;
    listener.set_nonblocking(true)?;
    let web_address = listener.local_addr()?;
    let shared = Arc::new(SharedWebState::new(current_session_id));
    println!("EtuSchlopf Watch Web: http://{web_address}/");

    let mqtt_shared = Arc::clone(&shared);
    thread::Builder::new()
        .name("etuschlopf-watch-web-mqtt-client".to_string())
        .spawn(move || run_mqtt_bridge(mqtt_address, mqtt_shared))?;

    open_browser_best_effort(web_address);

    thread::Builder::new()
        .name("etuschlopf-watch-web-http".to_string())
        .spawn(move || run_http_server(listener, shared, project_root, watch_started_at))
}

fn run_mqtt_bridge(mqtt_address: SocketAddr, shared: Arc<SharedWebState>) {
    loop {
        match MqttSessionStateClient::connect(mqtt_address) {
            Ok(mut client) => {
                shared.set_mqtt(MqttBridgeState::Live);
                loop {
                    match client.read_next() {
                        Ok(Some(publication)) => {
                            let Ok(snapshot) = decode_session_snapshot_json_v2(&publication.payload)
                            else {
                                continue;
                            };
                            let topic_matches = mqtt_topic_for_session(
                                &snapshot.observation_session_id,
                            )
                            .as_deref()
                                == Some(publication.topic.as_str());
                            if topic_matches {
                                shared.put_session(
                                    snapshot.observation_session_id,
                                    publication.payload,
                                );
                            }
                        }
                        Ok(None) => {}
                        Err(_) => break,
                    }
                }
            }
            Err(_) => {}
        }
        shared.set_mqtt(MqttBridgeState::Reconnecting);
        thread::sleep(MQTT_RECONNECT_DELAY);
    }
}

fn run_http_server(
    listener: TcpListener,
    shared: Arc<SharedWebState>,
    project_root: Arc<PathBuf>,
    watch_started_at: SystemTime,
) {
    loop {
        match listener.accept() {
            Ok((stream, peer)) if peer.ip().is_loopback() => {
                let client_state = Arc::clone(&shared);
                let client_root = Arc::clone(&project_root);
                let _ = thread::Builder::new()
                    .name("etuschlopf-watch-web-client".to_string())
                    .spawn(move || {
                        let _ = serve_http_client(
                            stream,
                            client_state,
                            client_root.as_ref().as_path(),
                            watch_started_at,
                        );
                    });
            }
            Ok((_stream, _peer)) => {}
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(20));
            }
            Err(_) => thread::sleep(Duration::from_millis(100)),
        }
    }
}

fn serve_http_client(
    mut stream: TcpStream,
    shared: Arc<SharedWebState>,
    project_root: &Path,
    watch_started_at: SystemTime,
) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(12)))?;
    let request = read_http_request(&mut stream)?;
    let Some(path) = request_path(&request) else {
        return write_http_response(
            &mut stream,
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            b"GET only",
        );
    };

    if let Some(encoded_label) = path.strip_prefix("/pdf-data/") {
        return serve_project_pdf_when_ready(
            &mut stream,
            project_root,
            watch_started_at,
            encoded_label,
        );
    }
    match path {
        "/" | "/index.html" => write_html(&mut stream),
        "/events" => serve_sse(&mut stream, &shared),
        "/state" => {
            let (_, body) = shared.snapshot();
            write_http_response(
                &mut stream,
                "200 OK",
                "application/json; charset=utf-8",
                body.as_bytes(),
            )
        }
        "/favicon.ico" => write_http_response(&mut stream, "204 No Content", "image/x-icon", &[]),
        _ => write_http_response(
            &mut stream,
            "404 Not Found",
            "text/plain; charset=utf-8",
            b"Not found",
        ),
    }
}

fn serve_sse(stream: &mut TcpStream, shared: &SharedWebState) -> io::Result<()> {
    stream.write_all(
        b"HTTP/1.1 200 OK\r\n\
Content-Type: text/event-stream; charset=utf-8\r\n\
Cache-Control: no-store\r\n\
Connection: keep-alive\r\n\
X-Content-Type-Options: nosniff\r\n\
X-Frame-Options: DENY\r\n\
Content-Kodex-Policy: default-src 'none'\r\n\r\n",
    )?;

    let (mut generation, initial) = shared.snapshot();
    write_sse_state(stream, &initial)?;

    loop {
        let guard = shared
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let (guard, timeout) = shared
            .changed
            .wait_timeout_while(guard, SSE_KEEPALIVE, |state| state.generation == generation)
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if guard.generation != generation {
            generation = guard.generation;
            let body = encode_browser_state(&guard);
            drop(guard);
            write_sse_state(stream, &body)?;
        } else {
            drop(guard);
            if timeout.timed_out() {
                stream.write_all(b": keepalive\n\n")?;
                stream.flush()?;
            }
        }
    }
}

fn write_sse_state(stream: &mut TcpStream, json: &str) -> io::Result<()> {
    stream.write_all(b"event: state\ndata: ")?;
    stream.write_all(json.as_bytes())?;
    stream.write_all(b"\n\n")?;
    stream.flush()
}

fn read_http_request(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut request = Vec::with_capacity(1024);
    let mut buffer = [0_u8; 1024];
    while request.len() < MAX_HTTP_REQUEST_BYTES {
        let read = stream.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        request.extend_from_slice(&buffer[..read]);
        if request.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }
    if request.len() >= MAX_HTTP_REQUEST_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Watch HTTP request exceeds local limit",
        ));
    }
    Ok(request)
}

fn request_path(request: &[u8]) -> Option<&str> {
    let first_line_end = request.windows(2).position(|window| window == b"\r\n")?;
    let first_line = std::str::from_utf8(request.get(..first_line_end)?).ok()?;
    let mut parts = first_line.split_ascii_whitespace();
    if parts.next()? != "GET" {
        return None;
    }
    parts.next()
}

fn write_html(stream: &mut TcpStream) -> io::Result<()> {
    write_http_response(
        stream,
        "200 OK",
        "text/html; charset=utf-8",
        WATCH_HTML.as_bytes(),
    )
}

fn serve_project_pdf_when_ready(
    stream: &mut TcpStream,
    project_root: &Path,
    watch_started_at: SystemTime,
    encoded_label: &str,
) -> io::Result<()> {
    let Some(project_label) = percent_decode_path_segment(encoded_label)
        .filter(|label| valid_project_label(label))
    else {
        return write_http_response(
            stream,
            "400 Bad Request",
            "text/plain; charset=utf-8",
            b"Ungueltiger Projektname",
        );
    };
    let deadline = Instant::now() + PDF_WAIT_TIMEOUT;
    loop {
        if let Some((mut file, length)) = open_current_run_project_pdf(
            project_root,
            &project_label,
            watch_started_at,
        ) {
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\nContent-Length: {length}\r\nCache-Control: no-store\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nContent-Kodex-Policy: default-src 'none'; frame-ancestors 'none'\r\n\r\n"
            )?;
            io::copy(&mut file, stream)?;
            return stream.flush();
        }
        if Instant::now() >= deadline {
            return write_http_response(
                stream,
                "404 Not Found",
                "text/plain; charset=utf-8",
                b"PDF im aktuellen Watch-Lauf nicht verfuegbar",
            );
        }
        thread::sleep(PDF_WAIT_POLL);
    }
}

fn open_current_run_project_pdf(
    project_root: &Path,
    project_label: &str,
    watch_started_at: SystemTime,
) -> Option<(File, u64)> {
    let candidate = project_root.join(format!("{project_label}.pdf"));
    let metadata = candidate.symlink_metadata().ok()?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.modified().ok()? < watch_started_at
    {
        return None;
    }
    let mut file = File::open(candidate).ok()?;
    let length = file.metadata().ok()?.len();
    if length < 10 {
        return None;
    }
    let mut header = [0_u8; 5];
    file.read_exact(&mut header).ok()?;
    if header != *b"%PDF-" {
        return None;
    }
    let tail_length = length.min(2_048) as usize;
    file.seek(SeekFrom::End(-(tail_length as i64))).ok()?;
    let mut tail = vec![0_u8; tail_length];
    file.read_exact(&mut tail).ok()?;
    if !tail.windows(5).any(|window| window == b"%%EOF") {
        return None;
    }
    file.seek(SeekFrom::Start(0)).ok()?;
    Some((file, length))
}

fn valid_project_label(project_label: &str) -> bool {
    !project_label.is_empty()
        && project_label.trim() == project_label
        && project_label != "."
        && project_label != ".."
        && !project_label.chars().any(|ch| {
            ch.is_control() || matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*')
        })
}

fn percent_decode_path_segment(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = *bytes.get(index + 1)?;
            let low = *bytes.get(index + 2)?;
            decoded.push((hex_value(high)? << 4) | hex_value(low)?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

const fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn write_http_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nContent-Kodex-Policy: default-src 'self'; style-src 'unsafe-inline'; script-src 'unsafe-inline'; connect-src 'self'\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)?;
    stream.flush()
}

fn encode_browser_state(state: &WebState) -> String {
    let mut json = String::from("{\"mqtt\":\"");
    json.push_str(state.mqtt.label());
    json.push_str("\",\"current_session_id\":");
    match state.current_session_id.as_deref() {
        Some(value) => push_browser_json_string(&mut json, value),
        None => json.push_str("null"),
    }
    json.push_str(",\"sessions\":[");
    for (index, payload) in state.sessions.values().enumerate() {
        if index != 0 {
            json.push(',');
        }
        match std::str::from_utf8(payload) {
            Ok(payload) => json.push_str(payload),
            Err(_) => json.push_str("null"),
        }
    }
    json.push_str("]}");
    json
}

fn push_browser_json_string(json: &mut String, value: &str) {
    json.push('"');
    for ch in value.chars() {
        match ch {
            '"' => json.push_str("\\\""),
            '\\' => json.push_str("\\\\"),
            '\n' => json.push_str("\\n"),
            '\r' => json.push_str("\\r"),
            '\t' => json.push_str("\\t"),
            ch if ch <= '\u{001f}' => json.push_str(&format!("\\u{:04x}", u32::from(ch))),
            ch => json.push(ch),
        }
    }
    json.push('"');
}

fn open_browser_best_effort(address: SocketAddr) {
    let url = format!("http://{address}/");

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let mut command = Command::new("cmd");
        command.args(["/C", "start", "", &url]);
        command.creation_flags(CREATE_NO_WINDOW);
        let _ = command.spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").arg(&url).spawn();
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let _ = Command::new("xdg-open").arg(&url).spawn();
    }
}

const WATCH_HTML: &str = r#"<!doctype html>
<html lang="de">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>EtuSchlopf Watch</title>
<style>
:root{color-scheme:dark;--bg:#0b1014;--panel:#111820;--panel2:#151e27;--line:#26323d;--text:#e8eef3;--muted:#8f9da9;--soft:#b9c5cf;--live:#6ce5b1;--cyan:#7cd7ff;--warn:#f5ca72;--bad:#ff7f8a;--radius:18px}*{box-sizing:border-box}html,body{margin:0;min-height:100%;background:radial-gradient(circle at 80% -10%,#142432 0,transparent 35%),var(--bg);color:var(--text);font-family:Inter,"Segoe UI",system-ui,-apple-system,sans-serif}body{padding:32px}main{max-width:1120px;margin:0 auto}.top{display:flex;align-items:center;justify-content:space-between;gap:20px;margin-bottom:28px}.brand{display:flex;align-items:center;gap:14px}.mark{width:42px;height:42px;border:1px solid var(--line);border-radius:13px;display:grid;place-items:center;background:linear-gradient(145deg,#16212b,#0e151b);font-weight:800;letter-spacing:-1px;color:var(--cyan)}h1{font-size:17px;letter-spacing:.16em;margin:0;font-weight:700}.sub{color:var(--muted);font-size:12px;margin-top:4px}.runstate{display:inline-flex;align-items:center;gap:9px;color:var(--soft);font-size:12px;letter-spacing:.08em;font-weight:700}.dot{width:9px;height:9px;border-radius:50%;background:var(--muted);box-shadow:0 0 0 4px rgba(143,157,169,.08)}.runstate[data-state=running] .dot{background:var(--live);box-shadow:0 0 18px rgba(108,229,177,.55);animation:softPulse 1.8s ease-in-out infinite}.runstate[data-state=completed] .dot{background:var(--cyan)}.runstate[data-state=failed] .dot{background:var(--bad)}.runstate[data-state=cancelled] .dot{background:var(--warn)}.runstate[data-state=dark_green] .dot{background:var(--live);box-shadow:0 0 14px rgba(108,229,177,.35)}.runstate[data-state=light_green] .dot{background:#a8e6c4}.runstate[data-state=yellow] .dot{background:var(--warn)}.runstate[data-state=red] .dot{background:var(--bad)}.shell{border:1px solid var(--line);border-radius:var(--radius);overflow:hidden;background:rgba(17,24,32,.92);box-shadow:0 24px 70px rgba(0,0,0,.28)}.empty{padding:72px 34px;text-align:center;color:var(--muted)}.empty strong{display:block;color:var(--text);font-size:22px;margin-bottom:8px}.session{padding:30px}.session+.session{border-top:1px solid var(--line)}.hero{display:flex;justify-content:space-between;gap:24px;align-items:flex-start;margin-bottom:24px}.herometric{text-align:right;color:var(--muted);font-size:10px;letter-spacing:.08em;text-transform:uppercase}.herometric b{display:block;color:var(--text);font-size:20px;letter-spacing:0;margin-top:3px;font-weight:600;font-variant-numeric:tabular-nums}.eyebrow{font-size:10px;letter-spacing:.13em;color:#70808d;text-transform:uppercase;margin-bottom:8px}.title{font-size:30px;font-weight:680;letter-spacing:-.025em}.path{margin-top:8px;color:var(--soft);font-size:15px}.activity{color:var(--cyan)}.clock{font-variant-numeric:tabular-nums;font-feature-settings:"tnum" 1;white-space:nowrap}.progress{position:relative;height:7px;background:#0b1116;border-radius:99px;overflow:hidden;margin:0 0 22px}.progress>i{display:block;position:relative;height:100%;background:linear-gradient(90deg,var(--cyan),var(--live));border-radius:99px;transition:width .25s ease}.progress.running>i::after{content:"";position:absolute;inset:0;width:42%;background:linear-gradient(90deg,transparent,rgba(255,255,255,.24),transparent);transform:translateX(-140%);animation:progressSweep 2.8s ease-in-out infinite}.corpusstats{display:flex;flex-wrap:wrap;gap:10px 18px;margin:0 0 20px;color:var(--muted);font-size:12px}.corpusstats b{color:var(--text);font-weight:650}.activecallouts{display:flex;flex-direction:column;gap:8px;margin:-8px 0 22px}.activecallout{display:flex;align-items:center;gap:9px;margin:0;color:var(--soft);font-size:13px}.activecallout .activepulse{width:7px;height:7px;border-radius:50%;background:var(--live);box-shadow:0 0 14px rgba(108,229,177,.45);animation:softPulse 1.8s ease-in-out infinite}.activecallout b{color:var(--text);font-weight:650}.projectlist{display:flex;flex-direction:column;border:1px solid var(--line);border-radius:14px;overflow:hidden;background:#0f161d}.projectcolumns,.projectrow>summary{display:grid;grid-template-columns:22px minmax(180px,.9fr) minmax(250px,1.2fr) 118px;align-items:center;gap:12px}.projectcolumns{padding:9px 16px;border-bottom:1px solid var(--line);color:#657480;font-size:9px;letter-spacing:.1em;text-transform:uppercase}.projectcolumns span:last-child{text-align:right}.projectwaiting{padding:22px 16px;color:var(--muted);font-size:13px;text-align:center}.projectrow{border:0;border-bottom:1px solid var(--line);background:transparent}.projectrow:last-child{border-bottom:0}.projectrow>summary{list-style:none;padding:13px 16px;cursor:pointer;transition:background .18s ease}.projectrow>summary::-webkit-details-marker{display:none}.projectrow>summary:hover{background:rgba(124,215,255,.035)}.projectrow.failed>summary{background:rgba(255,127,138,.035)}.projectrow.running>summary{background:linear-gradient(90deg,rgba(108,229,177,.045),rgba(108,229,177,.012) 38%,transparent 70%)}.projectdot{font-size:13px;text-align:center;color:var(--muted)}.projectrow.running .projectdot{color:var(--live);animation:softPulse 1.8s ease-in-out infinite}.projectrow.completed .projectdot{color:var(--cyan)}.projectrow.failed .projectdot{color:var(--bad)}.projectrow.cancelled .projectdot{color:var(--warn)}.projectname{font-size:14px;font-weight:650;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}.projectstate{display:block;justify-self:stretch;text-align:left;font-size:13px;color:var(--soft);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}.projectage{font-size:12px;color:var(--muted);font-variant-numeric:tabular-nums;font-feature-settings:"tnum" 1;white-space:nowrap;text-align:right}.projectdetails{display:flex;flex-wrap:wrap;gap:12px 22px;padding:0 16px 14px 50px;color:var(--muted);font-size:11px}.projectdetails b{color:var(--soft);font-weight:600;margin-left:5px}.systemstrip{display:flex;flex-wrap:wrap;gap:8px 20px;margin-top:15px;padding:0 2px;color:#64727e;font-size:11px}.systemitem{display:inline-flex;gap:8px;align-items:center}.systemitem strong{color:#84929e;font-weight:600}.systemitem .sysstate{color:#6f7e89}.terminalstate{font-weight:650}.terminalstate.completed{color:var(--cyan)}.terminalstate.failed{color:var(--bad)}.terminalstate.cancelled{color:var(--warn)}.phasebox{margin:4px 0 22px;padding:18px;border:1px solid var(--line);border-radius:14px;background:#0f161d}.phasehead{color:var(--muted);font-size:11px;margin-bottom:15px}.phaserail{display:grid;grid-template-columns:repeat(6,1fr);gap:7px}.phase{min-width:0}.phasebar{height:5px;border-radius:99px;background:#27323b;overflow:hidden;position:relative}.phase.done .phasebar{background:#45687a}.phase.active .phasebar{background:linear-gradient(90deg,var(--cyan),var(--live));box-shadow:0 0 13px rgba(108,229,177,.18)}.phase.active .phasebar::after{content:"";position:absolute;inset:0;width:50%;background:linear-gradient(90deg,transparent,rgba(255,255,255,.25),transparent);animation:progressSweep 2.4s ease-in-out infinite}.phase.problem .phasebar{background:var(--bad)}.phaselabel{display:block;margin-top:7px;color:#61717d;font-size:9px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}.phase.done .phaselabel{color:#9caab5}.phase.active .phaselabel{color:var(--text);font-weight:650}.currentline{display:flex;align-items:flex-start;justify-content:space-between;gap:20px;margin-bottom:22px;padding:0 2px}.currentmetrics{display:flex;justify-content:flex-end;gap:26px;flex-wrap:wrap}.currentlabel{font-size:10px;letter-spacing:.1em;text-transform:uppercase;color:#657480;margin-bottom:5px}.currentstate{font-size:15px;color:var(--soft)}.currenttime{text-align:right;color:var(--muted);font-size:10px;letter-spacing:.08em;text-transform:uppercase}.currenttime b{display:block;color:var(--text);font-size:18px;letter-spacing:0;margin-top:3px;font-weight:600;font-variant-numeric:tabular-nums}.detailsbox{border-top:1px solid var(--line);padding-top:16px}.detailsbox>summary{list-style:none;cursor:pointer;color:#7d8d99;font-size:11px;letter-spacing:.05em}.detailsbox>summary::-webkit-details-marker{display:none}.detailsbox>summary::after{content:"  ›"}.detailsbox[open]>summary::after{content:"  ⌄"}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(270px,1fr));gap:12px;margin-top:14px}.card{border:1px solid var(--line);background:var(--panel2);border-radius:14px;padding:18px}.cardhead{display:flex;justify-content:space-between;gap:14px;align-items:center}.role{font-size:12px;color:var(--muted);text-transform:uppercase;letter-spacing:.11em}.name{font-size:16px;font-weight:650;margin-top:4px}.state{font-size:13px;color:var(--soft);margin-top:14px;min-height:20px}.ages{display:flex;gap:20px;margin-top:18px;padding-top:14px;border-top:1px solid var(--line);font-variant-numeric:tabular-nums}.age{font-size:12px;color:var(--muted)}.age b{display:block;color:var(--text);font-size:13px;margin-top:3px;font-weight:600}.mini{font-size:12px;color:var(--muted);white-space:nowrap}.footer{display:flex;justify-content:flex-end;margin-top:14px;color:#53616d;font-size:10px;letter-spacing:.02em}.transport[data-state=live]{color:#61756f}.transport[data-state=reconnecting]{color:#806f52}.transport[data-state=closed]{color:#53616d}.pdf-link{color:inherit;text-decoration:none;display:inline-flex;align-items:center;gap:8px;min-width:0;border-radius:7px}.pdf-link:hover,.pdf-link:focus-visible{color:var(--cyan);outline:none}.pdf-label{min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.pdf-chip{flex:0 0 auto;border:1px solid #33424f;border-radius:999px;padding:2px 6px;color:#71818e;font-size:8px;letter-spacing:.08em;font-weight:700;vertical-align:middle;transition:color .15s ease,border-color .15s ease}.pdf-link:hover .pdf-chip,.pdf-link:focus-visible .pdf-chip{color:var(--cyan);border-color:#41687a}@keyframes softPulse{0%,100%{opacity:.58;transform:scale(.9)}50%{opacity:1;transform:scale(1.08)}}@keyframes progressSweep{0%,30%{transform:translateX(-140%)}72%,100%{transform:translateX(270%)}}@media(prefers-reduced-motion:reduce){.runstate[data-state=running] .dot,.projectrow.running .projectdot,.activecallout .activepulse,.progress.running>i::after,.phase.active .phasebar::after{animation:none}}@media(max-width:760px){body{padding:18px}.top,.hero,.currentline{flex-direction:column}.session{padding:22px}.title{font-size:25px}.footer{justify-content:flex-start}.projectcolumns{display:none}.projectrow>summary{grid-template-columns:18px minmax(0,1fr) auto}.projectstate{grid-column:2/4;padding-top:2px}.projectage{grid-column:3}.projectdetails{padding-left:46px}.systemstrip{flex-direction:column;gap:6px}.phaserail{grid-template-columns:repeat(3,1fr);row-gap:14px}.currenttime{text-align:left}}
</style>
</head>
<body>
<main>
<div class="top"><div class="brand"><div class="mark">ES</div><div><h1>ETUSCHLOPF WATCH</h1><div class="sub">Live runtime observation</div></div></div><div id="runStatus" class="runstate" data-state="waiting"><span class="dot"></span><span id="runStatusLabel">WAITING</span></div></div>
<div id="shell" class="shell"><div class="empty"><strong>Watch wird verbunden</strong>Warte auf den ersten Runtime-Snapshot …</div></div>
<div class="footer"><span id="transport" class="transport" data-state="connecting" title="127.0.0.1:47622 · 127.0.0.1:47623">MQTT · lokal · read-only · CONNECTING</span></div>
</main>
<script>
const $=s=>document.querySelector(s);const esc=v=>String(v??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const stage={launcher_preparing:'Preparing',launcher_building:'Building',launcher_tests:'Launcher Tests',launcher_running_target:'Running Target',launcher_finalizing:'Finalizing',project_run_preparation:'Preparation',project_arrangement_solver:'Arrangement Solver',project_master_notation_and_salience:'Master Notation',project_initial_render_transaction:'Initial Render',project_metric_beam_and_final_notation:'Final Notation',project_release_report_and_exit:'Release',corpus_planning:'Planning',corpus_preparing_projects:'Preparing Projects',corpus_running_projects:'Running Projects',corpus_collecting_project_result:'Collecting Results',corpus_finalizing_corpus_report:'Finalizing Report'};
const activity={clef_planning:'Clef Planning',beam_planning:'Beam Planning'};const statusLabel={running:'RUNNING',completed:'COMPLETED',failed:'FAILED',cancelled:'CANCELLED',dark_green:'DUNKELGRÜN',light_green:'HELLGRÜN',yellow:'GELB',red:'ROT'};const sourceStatusLabel={completed:'Completed',failed:'Failed',cancelled:'Cancelled'};
const projectStages=['project_run_preparation','project_arrangement_solver','project_master_notation_and_salience','project_initial_render_transaction','project_metric_beam_and_final_notation','project_release_report_and_exit'];const projectStageShort=['Vorbereitung','Solver','Master','Render','Notation','Release'];
let lastModel={mqtt:'connecting',current_session_id:null,sessions:[]};let receivedAt=performance.now();let transportState='connecting';let lastStructureKey='';
function formatMillis(ms){const s=Math.floor(Math.max(0,Number(ms||0))/1000);if(s<1)return'<1 s';if(s<60)return s+' s';const m=Math.floor(s/60),r=s%60;if(m<60)return m+':'+String(r).padStart(2,'0');return Math.floor(m/60)+'h '+String(m%60).padStart(2,'0');}
function age(ms){return formatMillis(Number(ms||0)+Math.max(0,performance.now()-receivedAt));}
function runDuration(o){const live=o?.source?.status==='running'?Math.max(0,performance.now()-receivedAt):0;return formatMillis(Number(o?.run_elapsed_millis||0)+live);}
function path(src){const a=activity[src.activity];return esc(stage[src.stage]||src.stage)+(a?' <span class="activity">› '+esc(a)+'</span>':'');}
function summary(s){const ss=s.sources.map(x=>x.source.status);if(ss.includes('failed'))return'failed';if(ss.includes('running'))return'running';if(ss.includes('cancelled'))return'cancelled';return'completed';}
function currentSession(model){if(!model?.current_session_id)return null;return model.sessions.find(s=>s.observation_session_id===model.current_session_id)||null;}
function visibleSessions(model){const current=currentSession(model);return current?[current]:[];}
function sessionOutcome(s){const launcher=s?.sources.find(o=>o.source.source_role==='launcher');return launcher?.source?.outcome_color||null;}
function overall(model){const current=currentSession(model);if(!current)return'waiting';const lifecycle=summary(current);return lifecycle==='running'?lifecycle:(sessionOutcome(current)||lifecycle);}
function structureKey(model){const current=currentSession(model);return JSON.stringify(current?[current.observation_session_id,current.sources.map(o=>o.source)]:null);}
function terminalAgeLabel(status){return status==='failed'?'fehlgeschlagen vor':status==='cancelled'?'abgebrochen vor':'abgeschlossen vor';}
function clock(sessionId,o,kind){const value=kind==='seen'?o.last_seen_age_millis:kind==='run'?o.run_elapsed_millis:o.last_state_change_age_millis;const text=kind==='run'?runDuration(o):age(value);return `<span class="clock" data-session="${esc(sessionId)}" data-source="${esc(o.source.source_id)}" data-age-kind="${kind}">${text}</span>`;}
function sourceLookup(){const map=new Map();const current=currentSession(lastModel);if(current)current.sources.forEach(o=>map.set(current.observation_session_id+'\u0000'+o.source.source_id,o));return map;}
function tick(){const lookup=sourceLookup();document.querySelectorAll('.clock[data-session][data-source]').forEach(el=>{const o=lookup.get(el.dataset.session+'\u0000'+el.dataset.source);if(!o)return;const kind=el.dataset.ageKind;const value=kind==='seen'?o.last_seen_age_millis:o.last_state_change_age_millis;el.textContent=kind==='run'?runDuration(o):age(value);});setRunStatus(overall(lastModel));setTransport(transportState);}
const pdfCache=new Map(),pdfPending=new Set();
function pdfRoute(label){return '/pdf-data/'+encodeURIComponent(label);}
function pdfLink(label,extraClass=''){const ready=pdfCache.get(label);return `<a class="pdf-link ${extraClass}" data-pdf="${encodeURIComponent(label)}" href="${ready||'#'}" target="_blank" rel="noopener" title="${ready?'PDF öffnen':'PDF wird vorbereitet'}"><span class="pdf-label">${esc(label)}</span><span class="pdf-chip">${ready?'PDF ↗':'PDF …'}</span></a>`;}
function refreshPdfLinks(label){const ready=pdfCache.get(label);if(!ready)return;document.querySelectorAll('.pdf-link').forEach(link=>{if(decodeURIComponent(link.dataset.pdf||'')!==label)return;link.href=ready;link.title='PDF öffnen';const chip=link.querySelector('.pdf-chip');if(chip)chip.textContent='PDF ↗';});}
function preparePdf(label){if(!label||pdfCache.has(label)||pdfPending.has(label))return;pdfPending.add(label);fetch(pdfRoute(label),{cache:'no-store'}).then(r=>{if(!r.ok||!String(r.headers.get('content-type')||'').startsWith('application/pdf'))throw 0;return r.blob();}).then(blob=>{pdfCache.set(label,URL.createObjectURL(blob));refreshPdfLinks(label);}).catch(()=>{}).finally(()=>pdfPending.delete(label));}
function primeProjectPdfs(sessions){sessions.forEach(s=>s.sources.filter(x=>x.source.source_role==='project'&&x.source.project_label).forEach(x=>preparePdf(x.source.project_label)));}
function sourceCard(sessionId,o){const s=o.source;const role=s.source_role==='project'?'Projekt':s.source_role==='corpus'?'Korpus':'Launcher';const name=s.project_label||role;const displayName=s.source_role==='project'?pdfLink(name):esc(name);let extra='';if(s.progress){extra=`<div class="mini">${s.progress.completed} / ${s.progress.total} · ${s.progress.active} aktiv</div>`;}const terminal=s.status!=='running';const state=terminal?`<span class="terminalstate ${esc(s.status)}">${esc(sourceStatusLabel[s.status]||s.status)}</span>`:path(s);const ages=terminal?`<div class="age">Dauer<b>${clock(sessionId,o,'run')}</b></div><div class="age">${terminalAgeLabel(s.status)}<b>${clock(sessionId,o,'state')}</b></div>`:`<div class="age">Laufzeit<b>${clock(sessionId,o,'run')}</b></div><div class="age">Phase seit<b>${clock(sessionId,o,'state')}</b></div><div class="age">gesehen<b>${clock(sessionId,o,'seen')}</b></div>`;return `<div class="card"><div class="cardhead"><div><div class="role">${role}</div><div class="name">${displayName}</div></div>${extra}</div><div class="state">${state}</div><div class="ages">${ages}</div></div>`;}
const projectCollator=new Intl.Collator('de',{numeric:true,sensitivity:'base'});
function projectStatusText(s){return s.status==='running'?path(s):`<span class="terminalstate ${esc(s.status)}">${esc(sourceStatusLabel[s.status]||s.status)}</span>`;}
function projectRow(sessionId,o){const s=o.source;const terminal=s.status!=='running';const symbol=s.status==='running'?'●':s.status==='completed'?'✓':s.status==='failed'?'✕':'○';const name=s.project_label||s.source_id;return `<details class="projectrow ${esc(s.status)}" data-source-id="${esc(s.source_id)}"><summary><span class="projectdot">${symbol}</span>${pdfLink(name,'projectname')}<span class="projectstate">${projectStatusText(s)}</span><span class="projectage">${clock(sessionId,o,'run')}</span></summary><div class="projectdetails"><span>${terminal?'Dauer':'Laufzeit'} <b>${clock(sessionId,o,'run')}</b></span><span>Phase seit <b>${clock(sessionId,o,'state')}</b></span><span>gesehen <b>${clock(sessionId,o,'seen')}</b></span></div></details>`;}
function systemItem(o){if(!o)return'';const s=o.source;const label=s.source_role==='corpus'?'Korpus':'Launcher';const state=s.status==='running'?path(s):esc(sourceStatusLabel[s.status]||s.status);return `<span class="systemitem"><strong>${label}</strong><span class="sysstate">${state}</span></span>`;}
function corpusStats(corpus,projects){const p=corpus?.source.progress;const failed=projects.filter(x=>x.source.status==='failed').length;const active=p?.active??projects.filter(x=>x.source.status==='running').length;const completed=p?.completed??projects.filter(x=>x.source.status==='completed').length;const pending=p&&Number.isFinite(p.total)?Math.max(0,p.total-completed-active):null;const parts=[`<span><b>${completed}</b> abgeschlossen</span>`,`<span><b>${active}</b> aktiv</span>`];if(pending!==null)parts.push(`<span><b>${pending}</b> ausstehend</span>`);if(failed)parts.push(`<span><b>${failed}</b> fehlgeschlagen</span>`);return parts.join('');}
function activeProjectCallouts(sessionId,projects){const active=projects.filter(x=>x.source.status==='running');if(!active.length)return'';return `<div class="activecallouts">${active.map(o=>{const name=o.source.project_label||o.source.source_id;return `<div class="activecallout"><span class="activepulse"></span><span><b>${esc(name)}</b> · ${path(o.source)} · Laufzeit ${clock(sessionId,o,'run')} · Phase ${clock(sessionId,o,'state')}</span></div>`;}).join('')}</div>`;}
function renderCorpusSession(s){const projects=s.sources.filter(x=>x.source.source_role==='project').sort((a,b)=>projectCollator.compare(a.source.project_label||a.source.source_id,b.source.project_label||b.source.source_id));const corpus=s.sources.find(x=>x.source.source_role==='corpus');const launcher=s.sources.find(x=>x.source.source_role==='launcher');const timing=corpus||launcher||projects[0];const st=summary(s);const p=corpus?.source.progress;const width=p&&p.total?Math.max(0,Math.min(100,100*p.completed/p.total)):null;const headline=p&&p.total&&st==='completed'?`${p.completed} / ${p.total} abgeschlossen`:'Korpuslauf';const rows=projects.length?projects.map(o=>projectRow(s.observation_session_id,o)).join(''):'<div class="projectwaiting">Projekte werden vorbereitet …</div>';const columns=projects.length?'<div class="projectcolumns"><span></span><span>Projekt</span><span>Aktueller Stand</span><span>Zeit</span></div>':'';const metric=timing?`<div class="herometric">${timing.source.status==='running'?'Laufzeit':'Gesamtdauer'}<b>${clock(s.observation_session_id,timing,'run')}</b></div>`:'';return `<section class="session corpus"><div class="hero"><div><div class="eyebrow">Korpus</div><div class="title">${esc(headline)}</div></div>${metric}</div>${activeProjectCallouts(s.observation_session_id,projects)}${width===null?'':`<div class="progress ${st==='running'?'running':''}"><i style="width:${width}%"></i></div>`}<div class="corpusstats">${corpusStats(corpus,projects)}</div><div class="projectlist">${columns}${rows}</div><details class="detailsbox"><summary>Technische Details</summary><div class="grid">${[launcher,corpus].filter(Boolean).map(o=>sourceCard(s.observation_session_id,o)).join('')}</div></details></section>`;}
function phaseRail(source){if(!source||source.source_role!=='project')return'';const index=projectStages.indexOf(source.stage);if(index<0)return'';return `<div class="phaserail">${projectStages.map((_,i)=>{let cls='future';if(source.status==='completed'||i<index)cls='done';else if(i===index)cls=source.status==='running'?'active':source.status==='failed'||source.status==='cancelled'?'problem':'done';return `<div class="phase ${cls}"><div class="phasebar"></div><span class="phaselabel">${projectStageShort[i]}</span></div>`;}).join('')}</div>`;}
function renderSingleSession(s){const projects=s.sources.filter(x=>x.source.source_role==='project');const primary=projects.find(x=>x.source.status==='running')||projects[0]||s.sources.find(x=>x.source.source_role==='corpus')||s.sources[0];const label=primary?.source.project_label||'EtuSchlopf';const title=primary?.source.source_role==='project'?pdfLink(label):esc(label);const ordered=[...s.sources].sort((a,b)=>{const r={launcher:0,corpus:1,project:2};return(r[a.source.source_role]??9)-(r[b.source.source_role]??9)||a.source.source_id.localeCompare(b.source.source_id)});const state=primary?projectStatusText(primary.source):'';const phase=primary?.source.source_role==='project'?`<div class="phasebox"><div class="phasehead">Arbeitsfluss</div>${phaseRail(primary.source)}</div>`:'';const metrics=primary?(primary.source.status==='running'?`<div class="currentmetrics"><div class="currenttime">Laufzeit<b>${clock(s.observation_session_id,primary,'run')}</b></div><div class="currenttime">Phase seit<b>${clock(s.observation_session_id,primary,'state')}</b></div></div>`:`<div class="currentmetrics"><div class="currenttime">Dauer<b>${clock(s.observation_session_id,primary,'run')}</b></div></div>`):'';const current=primary?`<div class="currentline"><div><div class="currentlabel">Aktueller Stand</div><div class="currentstate">${state}</div></div>${metrics}</div>`:'';const eyebrow=primary?.source.source_role==='project'?'Einzelprojekt':primary?.source.source_role==='corpus'?'Korpus':'Systemstart';return `<section class="session"><div class="hero"><div><div class="eyebrow">${eyebrow}</div><div class="title">${title}</div><div class="path">${primary&&primary.source.status==='running'?path(primary.source):''}</div></div></div>${phase}${current}<details class="detailsbox"><summary>Technische Details</summary><div class="grid">${ordered.map(o=>sourceCard(s.observation_session_id,o)).join('')}</div></details></section>`;}
function renderSession(s){const projects=s.sources.filter(x=>x.source.source_role==='project');const corpus=s.sources.some(x=>x.source.source_role==='corpus');return corpus||projects.length>1?renderCorpusSession(s):renderSingleSession(s);}
function setTransport(state){transportState=state;const el=$('#transport');el.dataset.state=state;const label=state==='live'?'LIVE':state==='reconnecting'?'RECONNECTING':state==='closed'?'CLOSED':'CONNECTING';el.textContent='MQTT · lokal · read-only · '+label;}
function setRunStatus(state){const el=$('#runStatus');el.dataset.state=state;$('#runStatusLabel').textContent=state==='waiting'?'WAITING':statusLabel[state]||state.toUpperCase();}
function renderStructure(){const visible=visibleSessions(lastModel);setRunStatus(overall(lastModel));setTransport(transportState);primeProjectPdfs(visible);const shell=$('#shell');if(!visible.length){shell.innerHTML='<div class="empty"><strong>Watch ist bereit</strong>Warte auf den ersten Runtime-Snapshot …</div>';return;}const openRows=new Set([...shell.querySelectorAll('details.projectrow[open]')].map(x=>x.dataset.sourceId));const detailsOpen=!!shell.querySelector('details.detailsbox[open]');shell.innerHTML=visible.map(renderSession).join('');shell.querySelectorAll('details.projectrow').forEach(x=>{if(openRows.has(x.dataset.sourceId))x.open=true;});if(detailsOpen){const details=shell.querySelector('details.detailsbox');if(details)details.open=true;}pdfCache.forEach((_,label)=>refreshPdfLinks(label));tick();}
const shell=$('#shell');shell.addEventListener('click',e=>{const link=e.target.closest('.pdf-link');if(!link)return;e.stopPropagation();if(!pdfCache.has(decodeURIComponent(link.dataset.pdf)))e.preventDefault();});
const events=new EventSource('/events');events.addEventListener('state',e=>{try{const next=JSON.parse(e.data);lastModel=next;receivedAt=performance.now();transportState=next.mqtt;primeProjectPdfs(visibleSessions(next));const key=structureKey(next);if(key!==lastStructureKey){lastStructureKey=key;renderStructure();}else tick();}catch{}});events.onerror=()=>{const current=currentSession(lastModel);const terminal=!!current&&summary(current)!=='running';transportState=terminal?'closed':'reconnecting';setTransport(transportState);};setInterval(tick,1000);
</script>
</body>
</html>"#;

#[cfg(test)]
mod tests {
    use super::{
        encode_browser_state, open_current_run_project_pdf, percent_decode_path_segment,
        request_path, valid_project_label, MqttBridgeState, SharedWebState, WebState, WATCH_HTML,
    };
    use std::collections::BTreeMap;
    use std::fs;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn browser_state_wraps_public_session_payload_without_rewriting_it() {
        let payload = br#"{"protocol_version":2,"observation_session_id":"obs-web","sources":[]}"#.to_vec();
        let mut sessions = BTreeMap::new();
        sessions.insert("obs-web".to_string(), payload.clone());
        let state = WebState {
            generation: 7,
            mqtt: MqttBridgeState::Live,
            current_session_id: Some("obs-web".to_string()),
            sessions,
        };
        let json = encode_browser_state(&state);
        assert_eq!(
            json,
            format!(
                "{{\"mqtt\":\"live\",\"current_session_id\":\"obs-web\",\"sessions\":[{}]}}",
                String::from_utf8(payload).expect("fixture UTF-8")
            )
        );
    }

    #[test]
    fn server_owned_current_session_filters_unrelated_retained_sessions() {
        let shared = SharedWebState::new(Some("obs-current".to_string()));
        shared.put_session(
            "obs-old".to_string(),
            br#"{"protocol_version":2,"observation_session_id":"obs-old","sources":[]}"#.to_vec(),
        );
        shared.put_session(
            "obs-current".to_string(),
            br#"{"protocol_version":2,"observation_session_id":"obs-current","sources":[]}"#.to_vec(),
        );
        let (_, json) = shared.snapshot();
        assert!(json.contains("\"current_session_id\":\"obs-current\""));
        assert!(json.contains("\"observation_session_id\":\"obs-current\""));
        assert!(!json.contains("obs-old"));
    }

    #[test]
    fn http_surface_exposes_only_requested_local_path() {
        assert_eq!(request_path(b"GET /events HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n"), Some("/events"));
        assert_eq!(request_path(b"POST /events HTTP/1.1\r\n\r\n"), None);
    }

    #[test]
    fn pdf_route_decodes_unicode_but_rejects_path_shaping_labels() {
        assert_eq!(
            percent_decode_path_segment("Et%C3%BCde%20Nr.%206").as_deref(),
            Some("Etüde Nr. 6")
        );
        assert!(valid_project_label("Etüde Nr. 6"));
        assert!(!valid_project_label("../Etüde Nr. 6"));
        assert!(!valid_project_label("Etüde Nr. 6\\alt"));
        assert!(percent_decode_path_segment("Et%C3%ZZde").is_none());
    }

    #[test]
    fn pdf_route_only_admits_current_run_pdf_inside_project_root() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("etuschlopf-watch-pdf-{unique}"));
        fs::create_dir_all(&root).expect("create fixture root");
        let pdf = root.join("Etüde Nr. 6.pdf");
        fs::write(&pdf, b"%PDF-1.7\nfixture\n%%EOF\n").expect("write fixture PDF");
        assert!(open_current_run_project_pdf(&root, "Etüde Nr. 6", UNIX_EPOCH).is_some());
        assert!(open_current_run_project_pdf(
            &root,
            "Etüde Nr. 6",
            SystemTime::now() + Duration::from_secs(60),
        )
        .is_none());
        fs::write(&pdf, b"%PDF-1.7\npartial").expect("write partial PDF");
        assert!(open_current_run_project_pdf(&root, "Etüde Nr. 6", UNIX_EPOCH).is_none());
        fs::remove_dir_all(root).expect("remove fixture root");
    }

    #[test]
    fn web_surface_separates_runtime_truth_from_transport_lifecycle() {
        assert!(WATCH_HTML.contains("id=\"runStatus\""));
        assert!(WATCH_HTML.contains("id=\"transport\""));
        assert!(WATCH_HTML.contains("abgeschlossen vor"));
        assert!(WATCH_HTML.contains("run_elapsed_millis"));
        assert!(WATCH_HTML.contains("function currentSession"));
        assert!(WATCH_HTML.contains("current_session_id"));
        assert!(!WATCH_HTML.contains("function currentSessions"));
        assert!(!WATCH_HTML.contains("sessionFreshness"));
        assert!(WATCH_HTML.contains("Laufzeit"));
        assert!(WATCH_HTML.contains("Dauer"));
        assert!(WATCH_HTML.contains("Phase seit"));
        assert!(WATCH_HTML.contains("sourceStatusLabel"));
        assert!(WATCH_HTML.contains("renderCorpusSession"));
        assert!(WATCH_HTML.contains("projectlist"));
        assert!(!WATCH_HTML.contains("projectStatusRank"));
        assert!(WATCH_HTML.contains("activeProjectCallouts"));
        assert!(WATCH_HTML.contains("outcome_color"));
        assert!(WATCH_HTML.contains("ausstehend"));
        assert!(WATCH_HTML.contains("function pdfLink"));
        assert!(WATCH_HTML.contains("function preparePdf"));
        assert!(WATCH_HTML.contains("/pdf-data/"));
        assert!(WATCH_HTML.contains("URL.createObjectURL"));
        assert!(WATCH_HTML.contains("PDF wird vorbereitet"));
        assert!(WATCH_HTML.contains("target=\"_blank\""));
        assert!(WATCH_HTML.contains("PDF ↗"));
        assert!(WATCH_HTML.contains("Aktueller Stand"));
        assert!(WATCH_HTML.contains("function structureKey"));
        assert!(WATCH_HTML.contains("function phaseRail"));
        assert!(WATCH_HTML.contains("function refreshPdfLinks"));
        assert!(WATCH_HTML.contains("setInterval(tick,1000)"));
        assert!(WATCH_HTML.contains("prefers-reduced-motion:reduce"));
        assert!(!WATCH_HTML.contains("setInterval(render,1000)"));
        assert!(!WATCH_HTML.contains("FINISHED"));
    }
}
