//! Interactive console projections of the launcher: aggregated cargo test counts, live
//! project status read back from the technical log, and "last run" phase durations.
//!
//! Display only. Nothing here decides an exit code, a release state or a gate. The exit
//! code of every child process stays the only verdict; a missing or unreadable input only
//! removes information from the screen and is never an error.

use etuschlopf::console_status_record::{ProjectOutcome, StatusEvent, StatusRecord};
use etuschlopf::observation::ProjectStage;
use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Summed result of every `test result:` line of one cargo test log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct CargoTestCounts {
    pub(crate) passed: u64,
    pub(crate) failed: u64,
    pub(crate) ignored: u64,
    pub(crate) measured: u64,
    pub(crate) filtered_out: u64,
    pub(crate) finished_centis: u64,
    pub(crate) result_lines: usize,
    pub(crate) all_ok: bool,
    /// False when at least one `test result:` line could not be parsed completely.
    pub(crate) complete: bool,
}

fn count_before(text: &str, label: &str) -> Option<u64> {
    text.split(';')
        .map(str::trim)
        .find_map(|part| part.strip_suffix(label)?.trim().rsplit(' ').next()?.parse().ok())
}

fn finished_centis(text: &str) -> Option<u64> {
    let value = text
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix("finished in "))?
        .strip_suffix('s')?;
    let seconds = value.parse::<f64>().ok()?;
    if !seconds.is_finite() || seconds < 0.0 {
        return None;
    }
    // Rounded hundredths; cargo prints two decimals.
    Some((seconds * 100.0).round() as u64)
}

/// Sums every `test result:` line. Cargo prints one line per test binary; with
/// `--all-targets` the last line belongs to the last binary only.
pub(crate) fn cargo_test_counts(log_text: &str) -> CargoTestCounts {
    let mut counts = CargoTestCounts {
        all_ok: true,
        complete: true,
        ..CargoTestCounts::default()
    };
    for line in log_text.lines().map(str::trim) {
        let Some(body) = line.strip_prefix("test result:") else {
            continue;
        };
        let body = body.trim();
        counts.result_lines += 1;
        let verdict_ok = body.starts_with("ok.");
        if !verdict_ok {
            counts.all_ok = false;
        }
        let parsed = (
            count_before(body, "passed"),
            count_before(body, "failed"),
            count_before(body, "ignored"),
            count_before(body, "measured"),
            count_before(body, "filtered out"),
            finished_centis(body),
        );
        if let (Some(passed), Some(failed), Some(ignored), Some(measured), Some(filtered), Some(centis)) =
            parsed
        {
            counts.passed = counts.passed.saturating_add(passed);
            counts.failed = counts.failed.saturating_add(failed);
            counts.ignored = counts.ignored.saturating_add(ignored);
            counts.measured = counts.measured.saturating_add(measured);
            counts.filtered_out = counts.filtered_out.saturating_add(filtered);
            counts.finished_centis = counts.finished_centis.saturating_add(centis);
        } else {
            counts.complete = false;
        }
        if !verdict_ok && !body.starts_with("FAILED.") {
            counts.complete = false;
        }
    }
    counts
}

/// Cargo-shaped aggregate summary for the existing `strict_test_summary` field.
/// One `test result:` line reproduces that line exactly.
pub(crate) fn cargo_test_summary_text(log_text: &str) -> String {
    let counts = cargo_test_counts(log_text);
    if counts.result_lines == 0 {
        return "summary_missing".to_string();
    }
    let verdict = if counts.all_ok { "ok" } else { "FAILED" };
    let mut text = format!(
        "{verdict}. {} passed; {} failed; {} ignored; {} measured; {} filtered out; finished in {}.{:02}s",
        counts.passed,
        counts.failed,
        counts.ignored,
        counts.measured,
        counts.filtered_out,
        counts.finished_centis / 100,
        counts.finished_centis % 100
    );
    if counts.result_lines > 1 {
        text.push_str(&format!(" ({} test binaries)", counts.result_lines));
    }
    if !counts.complete {
        text.push_str(" (summary incomplete)");
    }
    text
}

/// Short German display text for the console.
pub(crate) fn cargo_test_counts_display(counts: &CargoTestCounts) -> String {
    if counts.result_lines == 0 {
        return "keine Testsumme im Log".to_string();
    }
    let mut text = format!(
        "{} bestanden · {} fehlgeschlagen",
        counts.passed, counts.failed
    );
    if counts.ignored > 0 {
        text.push_str(&format!(" · {} ignoriert", counts.ignored));
    }
    if !counts.complete {
        text.push_str(" · Summe unvollständig");
    }
    text
}

/// One place for the screen wording of the six project stages.
pub(crate) const fn stage_short_label(stage: ProjectStage) -> &'static str {
    match stage {
        ProjectStage::ProjectRunPreparation => "Vorb.",
        ProjectStage::ArrangementSolver => "Arrang.",
        ProjectStage::MasterNotationAndSalience => "Notation",
        ProjectStage::InitialRenderTransaction => "Rendern",
        ProjectStage::MetricBeamAndFinalNotation => "Balken",
        ProjectStage::ReleaseReportAndExit => "Bericht",
    }
}

/// Minutes with one decimal, the unit of every console duration.
pub(crate) fn format_minutes(duration: Duration) -> String {
    format!("{:.1}min", duration.as_secs_f64() / 60.0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LiveEvent {
    Stage { project: String, stage: ProjectStage },
    Finished {
        project: String,
        outcome: ProjectOutcome,
        elapsed: Duration,
    },
}

#[derive(Debug)]
struct LiveProject {
    project: String,
    stage: Option<ProjectStage>,
    finished: bool,
}

/// Reads status records appended to one technical log after a fixed start offset.
/// Only complete lines are decoded; a trailing partial line stays buffered until the
/// next poll. Records of other observation sessions are ignored.
#[derive(Debug)]
pub(crate) struct LiveStatusReader {
    log_path: PathBuf,
    session: String,
    offset: u64,
    pending: Vec<u8>,
    projects: BTreeMap<String, LiveProject>,
    order: Vec<String>,
    finished_ok: usize,
    finished_failed: usize,
    longest: Option<(String, Duration)>,
}

const MAX_PENDING_BYTES: usize = 64 * 1024;
const MAX_READ_PER_POLL: u64 = 4 * 1024 * 1024;

impl LiveStatusReader {
    /// Starts reading at the current end of `log_path`, so earlier runs never appear.
    pub(crate) fn starting_at_current_end(log_path: &Path, session: &str) -> Self {
        let offset = fs::metadata(log_path).map(|meta| meta.len()).unwrap_or(0);
        Self::starting_at(log_path, session, offset)
    }

    fn starting_at(log_path: &Path, session: &str, offset: u64) -> Self {
        Self {
            log_path: log_path.to_path_buf(),
            session: session.to_string(),
            offset,
            pending: Vec::new(),
            projects: BTreeMap::new(),
            order: Vec::new(),
            finished_ok: 0,
            finished_failed: 0,
            longest: None,
        }
    }

    fn read_new_bytes(&mut self) -> Vec<u8> {
        let Ok(mut file) = fs::File::open(&self.log_path) else {
            return Vec::new();
        };
        let Ok(len) = file.metadata().map(|meta| meta.len()) else {
            return Vec::new();
        };
        if len < self.offset {
            // The log was replaced; nothing after our start can be attributed safely.
            self.offset = len;
            self.pending.clear();
            return Vec::new();
        }
        if len == self.offset || file.seek(SeekFrom::Start(self.offset)).is_err() {
            return Vec::new();
        }
        let mut bytes = Vec::new();
        let _ = (&mut file).take(MAX_READ_PER_POLL).read_to_end(&mut bytes);
        self.offset = self.offset.saturating_add(bytes.len() as u64);
        bytes
    }

    /// Consumes newly appended complete lines and returns the resulting events.
    pub(crate) fn poll(&mut self) -> Vec<LiveEvent> {
        let bytes = self.read_new_bytes();
        self.ingest(&bytes)
    }

    fn ingest(&mut self, bytes: &[u8]) -> Vec<LiveEvent> {
        self.pending.extend_from_slice(bytes);
        let mut events = Vec::new();
        while let Some(position) = self.pending.iter().position(|byte| *byte == b'\n') {
            let line = self.pending.drain(..=position).collect::<Vec<_>>();
            if let Ok(text) = std::str::from_utf8(&line)
                && let Some(record) = StatusRecord::decode_line(text)
                && record.session == self.session
                && let Some(event) = self.apply(record)
            {
                events.push(event);
            }
        }
        if self.pending.len() > MAX_PENDING_BYTES {
            // A line this long is no status record; drop it up to the next line break.
            self.pending.clear();
        }
        events
    }

    fn apply(&mut self, record: StatusRecord) -> Option<LiveEvent> {
        if !self.projects.contains_key(&record.source) {
            self.order.push(record.source.clone());
        }
        let entry = self
            .projects
            .entry(record.source)
            .or_insert_with(|| LiveProject {
                project: record.project.clone(),
                stage: None,
                finished: false,
            });
        if entry.finished {
            return None;
        }
        match record.event {
            StatusEvent::Stage(stage) => {
                if entry.stage == Some(stage) {
                    return None;
                }
                entry.stage = Some(stage);
                Some(LiveEvent::Stage {
                    project: entry.project.clone(),
                    stage,
                })
            }
            StatusEvent::Finished {
                outcome,
                elapsed_ms,
            } => {
                entry.finished = true;
                let elapsed = Duration::from_millis(elapsed_ms);
                if self.longest.as_ref().is_none_or(|(_, longest)| elapsed > *longest) {
                    self.longest = Some((entry.project.clone(), elapsed));
                }
                match outcome {
                    ProjectOutcome::Ok => self.finished_ok += 1,
                    ProjectOutcome::Failed => self.finished_failed += 1,
                }
                Some(LiveEvent::Finished {
                    project: entry.project.clone(),
                    outcome,
                    elapsed: Duration::from_millis(elapsed_ms),
                })
            }
        }
    }

    /// One closing line over all finished projects, e.g. `10/10 OK · längste Stück 5 7.9min`;
    /// `None` when no project reported an outcome.
    pub(crate) fn completion_summary(&self) -> Option<String> {
        let done = self.finished_ok + self.finished_failed;
        if done == 0 {
            return None;
        }
        let mut text = if self.finished_failed == 0 {
            format!("{done}/{} OK", self.order.len())
        } else {
            format!("{}/{} OK · {} fehlgeschlagen", self.finished_ok, self.order.len(), self.finished_failed)
        };
        if let Some((project, elapsed)) = &self.longest {
            text.push_str(&format!(" · längste {project} {}", format_minutes(*elapsed)));
        }
        Some(text)
    }

    pub(crate) fn has_projects(&self) -> bool {
        !self.order.is_empty()
    }

    /// Compact single-line overview of at most `max_chars` characters, e.g.
    /// `fertig 2 · 5 Rendern · 9 Balken · +1`. Live only the stage is shown; each project's
    /// duration appears in its completion line. A name prefix shared by all running
    /// projects is left out; projects that do not fit are counted as `+N`.
    pub(crate) fn running_summary(&self, max_chars: usize) -> String {
        let done = self.finished_ok + self.finished_failed;
        let head = (done > 0).then(|| {
            if self.finished_failed > 0 {
                format!("fertig {done} ({} Fehler)", self.finished_failed)
            } else {
                format!("fertig {done}")
            }
        });
        let running = self
            .order
            .iter()
            .filter_map(|source| self.projects.get(source))
            .filter(|project| !project.finished)
            .collect::<Vec<_>>();
        let names = running.iter().map(|project| project.project.as_str()).collect::<Vec<_>>();
        let prefix_len = shared_word_prefix_len(&names);
        let mut text = head.unwrap_or_default();
        for (index, project) in running.iter().enumerate() {
            let stage = project.stage.map_or("…", stage_short_label);
            let part = format!("{} {stage}", &project.project[prefix_len..]);
            let separator = if text.is_empty() { "" } else { " · " };
            let remaining = running.len() - index - 1;
            let reserve = if remaining > 0 { format!(" · +{remaining}").chars().count() } else { 0 };
            if text.chars().count() + separator.chars().count() + part.chars().count() + reserve > max_chars {
                let rest = running.len() - index;
                let overflow = format!("{separator}+{rest}");
                if text.chars().count() + overflow.chars().count() <= max_chars {
                    text.push_str(&overflow);
                }
                break;
            }
            text.push_str(separator);
            text.push_str(&part);
        }
        truncate_chars(&text, max_chars)
    }
}

/// Byte length of the longest prefix shared by all `names` that ends after a space, so
/// that `Stück A 1` / `Stück A 12` share `Stück A `. Needs at least two names.
fn shared_word_prefix_len(names: &[&str]) -> usize {
    let [first, rest @ ..] = names else {
        return 0;
    };
    if rest.is_empty() {
        return 0;
    }
    let mut common = first.len();
    for name in rest {
        common = first
            .char_indices()
            .zip(name.char_indices())
            .take_while(|((_, a), (_, b))| a == b)
            .last()
            .map_or(0, |((index, ch), _)| index + ch.len_utf8())
            .min(common);
    }
    first[..common].rfind(' ').map_or(0, |space| space + 1)
}

/// Cuts `text` to at most `max_chars` characters, marking a cut with `…`.
pub(crate) fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    if max_chars == 0 {
        return String::new();
    }
    let mut out = text.chars().take(max_chars - 1).collect::<String>();
    out.push('…');
    out
}

/// Durations of the last successful run per phase label. Display only.
pub(crate) const LAST_PHASE_DURATIONS_FILE: &str = "target/etuschlopf_logs/last_phase_durations.txt";

pub(crate) fn read_last_phase_durations(path: &Path) -> BTreeMap<String, u64> {
    let Ok(text) = fs::read_to_string(path) else {
        return BTreeMap::new();
    };
    text.lines()
        .filter_map(|line| {
            let (label, ms) = line.split_once('\t')?;
            Some((label.to_string(), ms.trim().parse::<u64>().ok()?))
        })
        .collect()
}

/// Records one successful phase; other labels keep their previous values. Best effort.
pub(crate) fn record_last_phase_duration(path: &Path, label: &str, elapsed: Duration) {
    if label.contains(['\t', '\n', '\r']) {
        return;
    }
    let mut durations = read_last_phase_durations(path);
    durations.insert(
        label.to_string(),
        u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX),
    );
    let mut text = String::new();
    for (label, ms) in &durations {
        text.push_str(&format!("{label}\t{ms}\n"));
    }
    let temp = path.with_extension("txt.pending");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if fs::write(&temp, text).is_ok() {
        let _ = fs::rename(&temp, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use etuschlopf::console_status_record::StatusRecord;
    use std::time::Instant;

    const MULTI_BINARY_LOG: &str = "\
running 6 tests
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out; finished in 0.50s

running 2410 tests
test result: ok. 2410 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 12.25s

running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

    #[test]
    fn counts_sum_every_test_binary_not_only_the_last() {
        let counts = cargo_test_counts(MULTI_BINARY_LOG);
        assert_eq!(counts.passed, 2417);
        assert_eq!(counts.failed, 0);
        assert_eq!(counts.ignored, 1);
        assert_eq!(counts.result_lines, 3);
        assert!(counts.all_ok && counts.complete);
        assert_eq!(
            cargo_test_counts_display(&counts),
            "2417 bestanden · 0 fehlgeschlagen · 1 ignoriert"
        );
        assert_eq!(
            cargo_test_summary_text(MULTI_BINARY_LOG),
            "ok. 2417 passed; 0 failed; 1 ignored; 0 measured; 41 filtered out; finished in 12.75s (3 test binaries)"
        );
    }

    #[test]
    fn single_result_line_keeps_the_previous_summary_shape() {
        let log = "running 38 tests\ntest result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s\n";
        assert_eq!(
            cargo_test_summary_text(log),
            "ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s"
        );
    }

    #[test]
    fn failures_and_damaged_lines_are_reported_not_hidden() {
        let log = "test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s\ntest result: ok. 4 passed; garbage\n";
        let counts = cargo_test_counts(log);
        assert!(!counts.all_ok);
        assert!(!counts.complete);
        assert_eq!((counts.passed, counts.failed), (3, 2));
        assert!(cargo_test_counts_display(&counts).ends_with("Summe unvollständig"));
        assert!(cargo_test_summary_text(log).starts_with("FAILED. 3 passed; 2 failed"));
        assert_eq!(cargo_test_summary_text("no results"), "summary_missing");
    }

    fn line(session: &str, source: &str, project: &str, event: StatusEvent) -> String {
        StatusRecord {
            session: session.to_string(),
            source: source.to_string(),
            project: project.to_string(),
            event,
        }
        .encode_line()
    }

    #[test]
    fn reader_buffers_split_lines_and_ignores_foreign_or_corrupt_records() {
        let mut reader = LiveStatusReader::starting_at(Path::new("unused"), "s1", 0);
        let stage = line("s1", "p0", "Stück A", StatusEvent::Stage(ProjectStage::ArrangementSolver));
        let (first, second) = stage.split_at(17);
        assert!(reader.ingest(first.as_bytes()).is_empty());
        let events = reader.ingest(second.as_bytes());
        assert_eq!(
            events,
            vec![LiveEvent::Stage {
                project: "Stück A".to_string(),
                stage: ProjectStage::ArrangementSolver
            }]
        );
        let foreign = line("other", "p1", "Fremd", StatusEvent::Stage(ProjectStage::ArrangementSolver));
        let corrupt = "ETUSCHLOPF-STATUS v1: session=s1; source=p2; project=x; event=stage; stage=nope\n";
        let noise = "OriginalBeamOriginCensus: group=1\n";
        let batch = format!("{foreign}{corrupt}{noise}");
        assert!(reader.ingest(batch.as_bytes()).is_empty());
        assert_eq!(reader.order, vec!["p0".to_string()]);
    }

    #[test]
    fn reader_tracks_parallel_projects_and_their_outcomes() {
        let mut reader = LiveStatusReader::starting_at(Path::new("unused"), "s", 0);
        let mut text = String::new();
        for (source, project, stage) in [
            ("p0", "A", ProjectStage::InitialRenderTransaction),
            ("p1", "B", ProjectStage::MetricBeamAndFinalNotation),
            ("p2", "C", ProjectStage::ArrangementSolver),
            ("p3", "D", ProjectStage::ProjectRunPreparation),
        ] {
            text.push_str(&line("s", source, project, StatusEvent::Stage(stage)));
        }
        text.push_str(&line(
            "s",
            "p1",
            "B",
            StatusEvent::Finished {
                outcome: ProjectOutcome::Failed,
                elapsed_ms: 90_000,
            },
        ));
        let events = reader.ingest(text.as_bytes());
        assert_eq!(events.len(), 5);
        assert_eq!(
            events.last(),
            Some(&LiveEvent::Finished {
                project: "B".to_string(),
                outcome: ProjectOutcome::Failed,
                elapsed: Duration::from_millis(90_000)
            })
        );
        assert_eq!(
            reader.completion_summary().as_deref(),
            Some("0/4 OK · 1 fehlgeschlagen · längste B 1.5min")
        );
        let summary = reader.running_summary(200);
        assert_eq!(summary, "fertig 1 (1 Fehler) · A Rendern · C Arrang. · D Vorb.");
        let mut single = LiveStatusReader::starting_at(Path::new("unused"), "s", 0);
        let only = line("s", "p0", "Solo", StatusEvent::Stage(ProjectStage::ArrangementSolver));
        single.ingest(only.as_bytes());
        assert_eq!(single.running_summary(200), "Solo Arrang.", "no counter before the first finish");
        let after = line("s", "p1", "B", StatusEvent::Stage(ProjectStage::ReleaseReportAndExit));
        assert!(reader.ingest(after.as_bytes()).is_empty(), "finished projects stay finished");
    }

    #[test]
    fn reader_starts_after_existing_log_content() {
        let dir = std::env::temp_dir().join(format!(
            "etuschlopf_live_status_{}_{}",
            std::process::id(),
            Instant::now().elapsed().as_nanos()
        ));
        fs::create_dir_all(&dir).expect("dir");
        let log = dir.join("console.txt");
        let old = line("s", "p0", "Alt", StatusEvent::Stage(ProjectStage::ArrangementSolver));
        fs::write(&log, &old).expect("old");
        let mut reader = LiveStatusReader::starting_at_current_end(&log, "s");
        assert!(reader.poll().is_empty());
        let new = line("s", "p1", "Neu", StatusEvent::Stage(ProjectStage::ArrangementSolver));
        fs::write(&log, format!("{old}{new}")).expect("append");
        let events = reader.poll();
        assert_eq!(events.len(), 1);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn running_summary_drops_shared_prefix_and_never_exceeds_its_budget() {
        let mut reader = LiveStatusReader::starting_at(Path::new("unused"), "s", 0);
        let mut text = String::new();
        for (source, project) in [("p0", "Stück Ä 5"), ("p1", "Stück Ä 12"), ("p2", "Stück Ä 1"), ("p3", "Stück Ä 7")] {
            text.push_str(&line("s", source, project, StatusEvent::Stage(ProjectStage::ArrangementSolver)));
        }
        reader.ingest(text.as_bytes());
        assert_eq!(
            reader.running_summary(200),
            "5 Arrang. · 12 Arrang. · 1 Arrang. · 7 Arrang."
        );
        assert_eq!(reader.running_summary(27), "5 Arrang. · 12 Arrang. · +2");
        for budget in 0..120 {
            assert!(reader.running_summary(budget).chars().count() <= budget, "{budget}");
        }
        assert_eq!(shared_word_prefix_len(&["Solo"]), 0);
        assert_eq!(shared_word_prefix_len(&["Ab 1", "Ab 10"]), 3);
        assert_eq!(shared_word_prefix_len(&["Ab1", "Ab2"]), 0, "only whole words are dropped");
        assert_eq!(truncate_chars("äöüß", 3), "äö…");
    }

    #[test]
    fn last_durations_keep_other_phases_and_tolerate_missing_files() {
        let dir = std::env::temp_dir().join(format!("etuschlopf_last_phase_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let path = dir.join("last_phase_durations.txt");
        assert!(read_last_phase_durations(&path).is_empty());
        record_last_phase_duration(&path, "Clippy", Duration::from_millis(78_000));
        record_last_phase_duration(&path, "Deep Clippy", Duration::from_millis(300_000));
        record_last_phase_duration(&path, "Clippy", Duration::from_millis(80_000));
        let durations = read_last_phase_durations(&path);
        assert_eq!(durations.get("Clippy"), Some(&80_000));
        assert_eq!(durations.get("Deep Clippy"), Some(&300_000));
        record_last_phase_duration(&path, "bad\tlabel", Duration::from_millis(1));
        assert_eq!(read_last_phase_durations(&path).len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn minutes_and_stage_labels_are_stable() {
        assert_eq!(format_minutes(Duration::from_secs(45)), "0.8min");
        assert_eq!(format_minutes(Duration::from_secs(168)), "2.8min");
        assert_eq!(stage_short_label(ProjectStage::InitialRenderTransaction), "Rendern");
    }
}
