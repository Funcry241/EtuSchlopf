//! Read-only Cargo console hints. Log messages never decide a child verdict.
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const READ_LIMIT: u64 = 64 * 1024;
const LINE_LIMIT: usize = 8 * 1024;

#[derive(Debug, Default, PartialEq, Eq)]
enum Phase {
    #[default]
    Unknown,
    Compiling,
    Checking,
    WaitingForLock,
    RunningTests,
    Prepared,
}
impl Phase {
    fn label(&self) -> &'static str {
        match self {
            Self::Unknown => "Prozess läuft",
            Self::Compiling => "kompiliert",
            Self::Checking => "prüft Code",
            Self::WaitingForLock => "wartet auf Cargo-Sperre",
            Self::RunningTests => "Tests laufen",
            Self::Prepared => "Vorbereitung fertig",
        }
    }
}

fn observed_phase(line: &str) -> Option<Phase> {
    // Cargo may inherit forced colour even though stdout goes to a file.
    // Accept only SGR colour escapes, never pass raw log text to the terminal.
    let mut plain = String::new();
    let mut chars = line.chars();
    while let Some(ch) = chars.next() {
        if ch != '\x1b' {
            plain.push(ch);
            continue;
        }
        if chars.next()? != '[' {
            return None;
        }
        loop {
            match chars.next()? {
                'm' => break,
                '0'..='9' | ';' => {}
                _ => return None,
            }
        }
    }
    let text = plain.trim();
    if text.starts_with("Blocking waiting for file lock ") {
        Some(Phase::WaitingForLock)
    } else if text.starts_with("Compiling ") {
        Some(Phase::Compiling)
    } else if text.starts_with("Checking ") {
        Some(Phase::Checking)
    } else if text.starts_with("Finished ") {
        Some(Phase::Prepared)
    } else if text.starts_with("Running unittests ")
        || text.starts_with("Running tests/")
        || text.starts_with("Running tests\\")
    {
        Some(Phase::RunningTests)
    } else {
        let words = text.split_whitespace().collect::<Vec<_>>();
        (words.len() == 3
            && words[0] == "running"
            && words[1].parse::<u64>().is_ok()
            && matches!(words[2], "test" | "tests"))
        .then_some(Phase::RunningTests)
    }
}

pub(crate) struct CargoActivityReader {
    path: PathBuf,
    offset: u64,
    pending: Vec<u8>,
    discard_line: bool,
    phase: Phase,
    modified: Option<SystemTime>,
    readable: bool,
}
impl CargoActivityReader {
    /// Ignore earlier commands in the shared log. Called before this child starts.
    pub(crate) fn starting_at_current_end(path: &Path) -> Self {
        Self::at_offset(path, std::fs::metadata(path).map_or(0, |m| m.len()))
    }
    /// Only for the separate prewarm log just truncated by this same invocation.
    pub(crate) fn from_fresh_log(path: &Path) -> Self {
        Self::at_offset(path, 0)
    }
    fn at_offset(path: &Path, offset: u64) -> Self {
        Self {
            path: path.to_path_buf(),
            offset,
            pending: Vec::new(),
            discard_line: false,
            phase: Phase::Unknown,
            modified: None,
            readable: true,
        }
    }
    pub(crate) fn poll(&mut self) {
        self.readable = self.read().is_some();
    }
    fn read(&mut self) -> Option<()> {
        let mut file = File::open(&self.path).ok()?;
        let metadata = file.metadata().ok()?;
        if metadata.len() < self.offset {
            self.offset = metadata.len();
            self.pending.clear();
            self.discard_line = false;
            self.phase = Phase::Unknown;
            self.modified = None;
            return Some(());
        }
        if metadata.len() == self.offset {
            return Some(());
        }
        file.seek(SeekFrom::Start(self.offset)).ok()?;
        let mut bytes = Vec::new();
        (&mut file).take(READ_LIMIT).read_to_end(&mut bytes).ok()?;
        self.offset = self.offset.saturating_add(bytes.len() as u64);
        if !bytes.is_empty() {
            self.modified = metadata.modified().ok();
        }
        self.ingest(&bytes);
        Some(())
    }
    fn ingest(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            if byte == b'\n' {
                if !self.discard_line
                    && let Ok(line) = std::str::from_utf8(&self.pending)
                    && let Some(phase) = observed_phase(line)
                {
                    self.phase = phase;
                }
                self.pending.clear();
                self.discard_line = false;
            } else if !self.discard_line {
                if self.pending.len() < LINE_LIMIT {
                    self.pending.push(byte);
                } else {
                    self.pending.clear();
                    self.discard_line = true;
                }
            }
        }
    }
    pub(crate) fn summary(&self, now: SystemTime, budget: usize) -> Option<String> {
        if !self.readable {
            return ("Log nicht lesbar".chars().count() <= budget)
                .then(|| "Log nicht lesbar".to_owned());
        }
        let phase = self.phase.label();
        let text = if let Some(age) = self.modified.and_then(|time| now.duration_since(time).ok()) {
            format!("{phase} · Log vor {} s", age.as_secs())
        } else {
            phase.to_owned()
        };
        if text.chars().count() <= budget {
            Some(text)
        } else if phase.chars().count() <= budget {
            Some(phase.to_owned())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Duration;
    static NEXT: AtomicU64 = AtomicU64::new(0);
    fn log_path() -> PathBuf {
        std::env::temp_dir().join(format!(
            "etuschlopf-cargo-ui-{}-{}.log",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }
    #[test]
    fn phases_follow_real_messages_without_inferring_success_or_cpu_activity() {
        assert_eq!(
            observed_phase("  Compiling example v1"),
            Some(Phase::Compiling)
        );
        assert_eq!(observed_phase("Checking example"), Some(Phase::Checking));
        assert_eq!(
            observed_phase("Blocking waiting for file lock on build directory"),
            Some(Phase::WaitingForLock)
        );
        assert_eq!(
            observed_phase("Running unittests src/main.rs (binary)"),
            Some(Phase::RunningTests)
        );
        assert_eq!(
            observed_phase("Running tests\\probe.rs (binary)"),
            Some(Phase::RunningTests)
        );
        assert_eq!(observed_phase("running 3 tests"), Some(Phase::RunningTests));
        assert_eq!(observed_phase("test result: ok. 3 passed"), None);
        assert_eq!(observed_phase("Running build-script-build"), None);
        assert_eq!(observed_phase("an unrelated diagnostic"), None);
        assert_eq!(
            observed_phase("\x1b[32mCompiling\x1b[0m example"),
            Some(Phase::Compiling)
        );
        assert_eq!(observed_phase("\x1b[2JCompiling forged"), None);
    }
    #[test]
    fn fragmented_lines_and_oversized_or_invalid_messages_cannot_invent_phases() {
        let mut r = CargoActivityReader::from_fresh_log(Path::new("unused"));
        r.ingest(b"Compil");
        assert_eq!(r.phase, Phase::Unknown);
        r.ingest(b"ing example\r\n");
        assert_eq!(r.phase, Phase::Compiling);
        r.ingest(&vec![b'x'; LINE_LIMIT + 1]);
        assert!(r.pending.is_empty());
        r.ingest(b"Running unittests hidden\n");
        assert_eq!(r.phase, Phase::Compiling);
        r.ingest(b"\xffRunning tests/bad\n");
        assert_eq!(r.phase, Phase::Compiling);
        r.ingest(b"Checking next\n");
        assert_eq!(r.phase, Phase::Checking);
    }
    #[test]
    fn shared_log_ignores_previous_command_and_truncation_discards_stale_state() {
        let path = log_path();
        std::fs::write(&path, b"Compiling previous\n").unwrap();
        let mut r = CargoActivityReader::starting_at_current_end(&path);
        r.poll();
        assert_eq!(r.phase, Phase::Unknown);
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        writeln!(file, "Checking current").unwrap();
        r.poll();
        assert_eq!(r.phase, Phase::Checking);
        let stamp = std::fs::metadata(&path).unwrap().modified().unwrap();
        assert!(
            r.summary(stamp + Duration::from_secs(37), 80)
                .unwrap()
                .contains("Log vor 37 s")
        );
        // An unchanged poll never resets the displayed age.
        r.poll();
        assert!(
            r.summary(stamp + Duration::from_secs(73), 80)
                .unwrap()
                .contains("Log vor 73 s")
        );
        drop(file);
        // Truncation requires a write handle; append-only access is insufficient on Windows.
        std::fs::File::create(&path).unwrap();
        r.poll();
        assert_eq!(r.phase, Phase::Unknown);
        assert!(r.modified.is_none());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn prewarm_uses_original_log_time_even_when_first_observed_at_join() {
        let path = log_path();
        std::fs::write(&path, b"Compiling test-binaries\n").unwrap();
        let stamp = std::fs::metadata(&path).unwrap().modified().unwrap();
        let mut r = CargoActivityReader::from_fresh_log(&path);
        r.poll();
        assert_eq!(
            r.summary(stamp + Duration::from_secs(91), 80).unwrap(),
            "kompiliert · Log vor 91 s"
        );
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn missing_log_clock_changes_and_small_console_widths_only_remove_hints() {
        let mut r = CargoActivityReader::from_fresh_log(&log_path());
        r.poll();
        assert_eq!(
            r.summary(SystemTime::now(), 80).as_deref(),
            Some("Log nicht lesbar")
        );
        r.readable = true;
        r.phase = Phase::RunningTests;
        r.modified = Some(SystemTime::UNIX_EPOCH + Duration::from_secs(100));
        for budget in 0..80 {
            assert!(
                r.summary(SystemTime::UNIX_EPOCH + Duration::from_secs(90), budget)
                    .is_none_or(|s| s.chars().count() <= budget)
            );
        }
        assert_eq!(
            r.summary(SystemTime::UNIX_EPOCH + Duration::from_secs(90), 80)
                .as_deref(),
            Some("Tests laufen")
        );
    }
}
