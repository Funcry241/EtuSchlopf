//! Advisory startup observations. No gate, receipt, cache or transaction authority.
use std::time::Instant;

#[cfg(not(test))]
const LOG: &str = "target/etuschlopf_logs/startup_timing.txt";
#[cfg(not(test))]
const SESSION_ENV: &str = "ETUSCHLOPF_STARTUP_TIMING_SESSION";

#[cfg(not(test))]
fn session() -> Option<String> {
    std::env::var(SESSION_ENV).ok().filter(|s| {
        !s.is_empty() && s.len() <= 80 && s.bytes().all(|b| b.is_ascii_digit() || b == b'_')
    })
}

pub struct Phase {
    name: &'static str,
    started: Instant,
}

impl Phase {
    pub fn new(name: &'static str) -> Self {
        append(&format!("StartupRustTiming: Phase={name}; Event=begin; Pid={}", std::process::id()));
        Self { name, started: Instant::now() }
    }
}

impl Drop for Phase {
    fn drop(&mut self) {
        append(&format!(
            "StartupRustTiming: Phase={}; Event=end; WallUs={}; Pid={}",
            self.name, self.started.elapsed().as_micros(), std::process::id(),
        ));
    }
}

fn append(line: &str) {
    #[cfg(test)]
    let _ = line;
    #[cfg(not(test))]
    {
        use std::io::Write;
        if let Some(id) = session()
            && let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(LOG)
        {
            let _ = writeln!(file, "Session={id}; {line}");
        }
    }
}

pub fn readback() -> String {
    #[cfg(test)]
    let (text, id) = (String::new(), None::<String>);
    #[cfg(not(test))]
    let text = std::fs::metadata(LOG).ok().filter(|m| m.len() <= 262_144)
        .and_then(|_| std::fs::read_to_string(LOG).ok()).unwrap_or_default();
    #[cfg(not(test))]
    let id = session();
    project(&text, id.as_deref())
}

fn clock_ms(clock: &str) -> Option<u64> {
    let normalized = clock.trim().replace(',', ".");
    let (h, tail) = normalized.split_once(':')?;
    let (m, tail) = tail.split_once(':')?;
    let (s, cs) = tail.split_once('.')?;
    if cs.len() != 2 { return None; }
    let (h, m, s, cs): (u64, u64, u64, u64) = (h.parse().ok()?, m.parse().ok()?, s.parse().ok()?, cs.parse().ok()?);
    if h >= 24 || m >= 60 || s >= 60 || cs >= 100 { return None; }
    Some(((h * 60 + m) * 60 + s) * 1000 + cs * 10)
}

fn field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    line.split_once(": ").map_or(line, |(_, body)| body).split(';').find_map(|part| {
        let (key, value) = part.trim().split_once('=')?;
        (key == name).then_some(value)
    })
}

fn project(text: &str, id: Option<&str>) -> String {
    let mut out = String::from("\n[STARTUP-TIMING-READBACK]\nAuthority: none; diagnostic only\nCmdClock: local wall clock, 10ms granularity; not monotonic; clock adjustments are not detectable\nRustClock: monotonic phase elapsed *Us; separate processes have no shared Instant epoch\nDoNotSum: inclusive pre-reexec spans contain admission/apply; RunCmdBeforeChild contains cargo build; ReexecChildWait includes the entire child run\nCaptureBoundary: before launcher handoff; parent child-wait/acceptance/ZIP/exit tail may still be open\nFirstInstallingTake: an older parent cannot emit newly added admission/apply phases; missing is not zero\n");
    let Some(id) = id.filter(|id| text.lines().next() == Some(&format!("StartupTimingSession={id}"))) else {
        out.push_str("TimingStatus: not_captured_or_session_mismatch; no zero-duration inference\n[/STARTUP-TIMING-READBACK]\n");
        return out;
    };
    let prefix = format!("Session={id}; ");
    let rows = text.lines().filter_map(|line| line.strip_prefix(&prefix)).collect::<Vec<_>>();
    for (index, line) in rows.iter().enumerate() {
        out.push_str(line);
        out.push('\n');
        if line.starts_with("StartupCmdTiming:") && field(line, "Event") == Some("end") {
            let begin = rows[..index].iter().rev().find(|before| {
                before.starts_with("StartupCmdTiming:") && field(before, "Event") == Some("begin")
                    && field(before, "Invocation") == field(line, "Invocation")
                    && field(before, "Phase") == field(line, "Phase")
            });
            let elapsed = begin.and_then(|before| {
                // Date changes and backward clocks are explicit gaps, never inferred rollover.
                if field(before, "Date") != field(line, "Date") { return None; }
                clock_ms(field(line, "Clock")?)?.checked_sub(clock_ms(field(before, "Clock")?)?)
            });
            out.push_str(&format!("StartupCmdDuration: Phase={}; Invocation={}; WallMs={}; Precision=10ms_local_clock\n",
                field(line, "Phase").unwrap_or("unknown"), field(line, "Invocation").unwrap_or("unknown"),
                elapsed.map(|ms| ms.to_string()).unwrap_or_else(|| "not_captured_date_or_clock_discontinuity".to_string())));
        }
    }
    for phase in ["DependencyAdmission", "IncomingTakeAdmission", "ApplyAndVerify", "TakeBeforeReexec"] {
        if !rows.iter().any(|line| field(line, "Phase") == Some(phase) && field(line, "Event") == Some("end")) {
            out.push_str(&format!("StartupPhaseStatus: Phase={phase}; Status=not_captured_or_still_open\n"));
        }
    }
    out.push_str("[/STARTUP-TIMING-READBACK]\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_duration_binds_invocation_and_preserves_precision() {
        let text = "StartupTimingSession=12_34\nSession=12_34; StartupCmdTiming: Phase=Build; Invocation=1; Event=begin; Date=d; Clock= 9:00:00,00\nSession=12_34; StartupCmdTiming: Phase=Build; Invocation=2; Event=begin; Date=d; Clock=09:00:01.00\nSession=12_34; StartupCmdTiming: Phase=Build; Invocation=1; Event=end; Date=d; Clock=09:00:02.50";
        assert!(project(text, Some("12_34")).contains("Phase=Build; Invocation=1; WallMs=2500; Precision=10ms"));
    }

    #[test]
    fn stale_sessions_and_clock_discontinuities_are_not_zero() {
        assert!(project("StartupTimingSession=1", Some("2")).contains("session_mismatch"));
        let text = "StartupTimingSession=1\nSession=1; StartupCmdTiming: Phase=Build; Invocation=1; Event=begin; Date=a; Clock=23:59:59.00\nSession=1; StartupCmdTiming: Phase=Build; Invocation=1; Event=end; Date=b; Clock=00:00:01.00";
        assert!(project(text, Some("1")).contains("WallMs=not_captured_date_or_clock_discontinuity"));
        assert!(project(text, Some("1")).contains("Phase=DependencyAdmission; Status=not_captured"));
        assert_eq!(clock_ms("24:00:00.00"), None);
        assert_eq!(clock_ms("18446744073709551615:00:00.00"), None);
        assert_eq!(clock_ms("00:00:00.bad"), None);
        assert!(project(&text.replace("Date=b", "Date=a"), Some("1"))
            .contains("WallMs=not_captured_date_or_clock_discontinuity"));
    }

    #[test]
    fn open_parent_wait_is_preserved_without_a_fabricated_end() {
        let text = "StartupTimingSession=1\nSession=1; StartupRustTiming: Phase=TakeBeforeReexec; Event=end; WallUs=900; Pid=1\nSession=1; StartupRustTiming: Phase=ReexecChildWait; Event=begin; Pid=1";
        let out = project(text, Some("1"));
        assert!(out.contains("Phase=TakeBeforeReexec; Event=end; WallUs=900"));
        assert!(out.contains("Phase=ReexecChildWait; Event=begin"));
        assert!(!out.contains("Phase=ReexecChildWait; Event=end"));
        assert!(out.contains("inclusive pre-reexec"));
    }
}
