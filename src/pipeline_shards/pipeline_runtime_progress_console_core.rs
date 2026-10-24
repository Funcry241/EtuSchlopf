use anyhow::{anyhow, Context};
use arrangement::{arrange_piano_musicxml_with_profile, normalize_visible_rhythm_musicxml, ArrangementProfile, ArrangementResult, ArrangementStats};
use cli::Cli;
use crate::pipeline_profile::{
    broken_gesture_name, combined_name, critical_micro_repair_profiles, hybrid_tail_name,
    local_best_refinement_profiles, residual_afterburner_profiles, targeted_critical_profiles,
    CandidateLineage, CandidateLineageStep, CandidateProfile,
};
use midi_seed::{
    midi_seed_solver_profiles, midi_to_musicxml_seed_with_profile_and_hotspots, MidiSeedStats,
    HotspotEvidenceAudit, HotspotFocus, HotspotPlan, HotspotPlanAuditProjection,
};
use layout::{apply_pianist_layout, choose_auto_pianist_layout, AutoLayoutInput, LayoutPlan, HorizontalLayoutSeedPlan, HorizontalLayoutSeedSlot};
use model::{CheckReport, Issue, Pitch, ReportStatus, ReportSummary, Severity};
use musicxml::parse_musicxml;
use notation::{decompose_lower_staff_clef_islands, polish_musicxml, PolishStats};
use reference::{build_reference_signature, check_reference_fidelity, inspect_reference_files, FidelityCheck, ReferenceReport, ReferenceSignature};
use render::{count_pdf_pages, render_pdf_if_possible, RenderStatus};
use report::{write_console_summary, write_text_report, PdfFileFingerprintReport, ReportOptions};
use rules::{run_checks, CheckOptions};
use source::resolve_source_plan;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::io::{self, IsTerminal, Write};
use std::hash::{Hash, Hasher};
use std::thread;
use std::time::{Duration, Instant};
#[derive(Debug, Clone)]
struct ProgressTimingEntry {
    label: String,
    total: usize,
    workers: usize,
    started: Instant,
    elapsed: Option<Duration>,
    ticks_seen: usize,
}
static LIVE_PROGRESS_ACTIVE: Mutex<bool> = Mutex::new(false);
static BATCH_PROGRESS_ACTIVE: Mutex<bool> = Mutex::new(false);
static BATCH_PROGRESS_FRAME_COUNTER: AtomicUsize = AtomicUsize::new(0);
static CONSOLE_CAPTURE: Mutex<Vec<String>> = Mutex::new(Vec::new());
static PROGRESS_TIMING_CAPTURE: Mutex<Vec<ProgressTimingEntry>> = Mutex::new(Vec::new());
static PROGRESS_RENDER_STATE: Mutex<BTreeMap<String, usize>> = Mutex::new(BTreeMap::new());
static PIPELINE_STDOUT_RENDER_POLICY: OnceLock<StreamRenderPolicy> = OnceLock::new();
fn strip_ansi_codes(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            let _ = chars.next();
            for c in chars.by_ref() {
                if c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else if ch != '\r' {
            out.push(ch);
        }
    }
    out
}
fn capture_console_line(line: &str) {
    let clean = strip_ansi_codes(line);
    let mut log = CONSOLE_CAPTURE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    log.push(clean);
}
fn record_progress_timing(label: &str, done: usize, total: usize, workers: usize) {
    if total == 0 {
        return;
    }
    let now = Instant::now();
    let mut entries = PROGRESS_TIMING_CAPTURE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    // v0.10.200: Die Berichtsmessung darf nicht den eventuell übergeordneten
    // Runden-/Phasenstart aus der Konsolenanzeige übernehmen. Sonst werden
    // mehrere sichtbare Fortschrittsdurchläufe kumulativ gezählt und die
    // Summe kann größer werden als die Engine-Gesamtzeit. Für die exklusive
    // Kostenbilanz beginnt jeder sichtbare Durchlauf beim ersten eigenen
    // report_progress-Aufruf.
    if done != 0
        && let Some(entry) = entries
            .iter_mut()
            .rev()
            .find(|entry| entry.label.as_str() == label && entry.total == total && entry.elapsed.is_none())
    {
        entry.workers = workers.max(1);
        entry.ticks_seen += 1;
        if done >= total {
            entry.elapsed = Some(entry.started.elapsed());
        }
        return;
    }
    entries.push(ProgressTimingEntry {
        label: label.to_string(),
        total,
        workers: workers.max(1),
        started: now,
        elapsed: if done >= total { Some(Duration::from_secs(0)) } else { None },
        ticks_seen: 1,
    });
}
fn console_progress(message: impl AsRef<str>) {
    clear_live_progress_line();
    let line = format!("ETUSCHLOPF: {}", message.as_ref());
    println!("{line}");
    capture_console_line(&line);
}
fn normalized_console_env_value(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
}
fn pipeline_console_preferences_from_env() -> ConsolePreferences {
    let color = if std::env::var_os("NO_COLOR").is_some() {
        ColorPreference::Never
    } else {
        match normalized_console_env_value("ETUSCHLOPF_COLOR").as_deref() {
            Some("0" | "false" | "off" | "no" | "never") => ColorPreference::Never,
            Some("1" | "true" | "on" | "yes" | "always") => ColorPreference::Always,
            _ => ColorPreference::Auto,
        }
    };
    let progress = match normalized_console_env_value("ETUSCHLOPF_PROGRESS").as_deref() {
        Some("0" | "false" | "off" | "no" | "plain") => ProgressPreference::Plain,
        _ => ProgressPreference::Auto,
    };
    let glyphs = match normalized_console_env_value("ETUSCHLOPF_ASCII_PROGRESS").as_deref() {
        Some("1" | "true" | "on" | "yes" | "ascii" | "plain") => GlyphPreference::Ascii,
        _ => GlyphPreference::Auto,
    };
    ConsolePreferences {
        color,
        progress,
        glyphs,
    }
}
fn pipeline_stdout_render_policy() -> StreamRenderPolicy {
    *PIPELINE_STDOUT_RENDER_POLICY.get_or_init(|| {
        resolve_stream_render_policy(
            StreamCapability {
                destination: if io::stdout().is_terminal() {
                    StreamDestination::Terminal
                } else {
                    StreamDestination::Redirected
                },
                glyphs: GlyphCapability::Unicode,
            },
            pipeline_console_preferences_from_env(),
        )
    })
}
fn live_progress_enabled() -> bool {
    pipeline_stdout_render_policy().progress == ProgressMode::Inline
}
fn clear_live_progress_line() {
    let mut active = LIVE_PROGRESS_ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if *active {
        if live_progress_enabled() {
            print!("\r\x1b[2K");
            let _ = io::stdout().flush();
        } else {
            println!();
        }
        *active = false;
    }
}
fn render_live_progress_line(line: &str, final_line: bool) {
    if !live_progress_enabled() {
        println!("{line}");
        let mut active = LIVE_PROGRESS_ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *active = false;
        return;
    }
    let mut active = LIVE_PROGRESS_ACTIVE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    print!("\r\x1b[2K{line}");
    if final_line {
        println!();
    }
    *active = !final_line;
    let _ = io::stdout().flush();
}
fn should_report_progress_tick(done: usize, total: usize) -> bool {
    if total == 0 {
        return false;
    }
    done == 1 || done >= total || done.checked_rem((total / 20).max(1)) == Some(0)
}
fn color_progress_enabled() -> bool {
    pipeline_stdout_render_policy().style == StyleMode::Styled
}
fn colorize(code: &str, text: impl AsRef<str>) -> String {
    if color_progress_enabled() {
        format!("\x1b[{}m{}\x1b[0m", code, text.as_ref())
    } else {
        text.as_ref().to_string()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProgressColorContext {
    DefaultArtifact,
    RenderXml,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConsoleArtifactLine {
    Core,
    MasterMusicXml,
    PreRenderXml,
    PostRenderXml,
    Pdf,
    Report,
    BehindBarsRuleCheck,
}
fn progress_color_code_for_context(done: usize, total: usize, context: ProgressColorContext) -> &'static str {
    if total == 0 {
        return "90";
    }
    let done = done.min(total);
    match (context, done >= total) {
        (ProgressColorContext::RenderXml, true) => "1;94",
        (ProgressColorContext::RenderXml, false) => "94",
        (ProgressColorContext::DefaultArtifact, true) => "1;92",
        (ProgressColorContext::DefaultArtifact, false) => "92",
    }
}
fn console_artifact_line_context(line: ConsoleArtifactLine) -> ProgressColorContext {
    match line {
        ConsoleArtifactLine::PreRenderXml | ConsoleArtifactLine::PostRenderXml => ProgressColorContext::RenderXml,
        _ => ProgressColorContext::DefaultArtifact,
    }
}
fn console_artifact_line_tag(line: ConsoleArtifactLine) -> &'static str {
    match line {
        ConsoleArtifactLine::Core => "CORE",
        ConsoleArtifactLine::MasterMusicXml => "MASTER",
        ConsoleArtifactLine::PreRenderXml => "PRE-RENDER",
        ConsoleArtifactLine::PostRenderXml => "POST-RENDER",
        ConsoleArtifactLine::Pdf => "PDF",
        ConsoleArtifactLine::Report => "REPORT",
        ConsoleArtifactLine::BehindBarsRuleCheck => "BEHIND-BARS-RULE-CHECK",
    }
}
fn console_artifact_line(line: ConsoleArtifactLine, message: impl AsRef<str>) {
    clear_live_progress_line();
    let tag = console_artifact_line_tag(line);
    let plain = format!("ETUSCHLOPF: [{:<12}] {}", tag, message.as_ref());
    let tag_code = if line == ConsoleArtifactLine::BehindBarsRuleCheck {
        "93"
    } else {
        progress_color_code_for_context(100, 100, console_artifact_line_context(line))
    };
    let shown = format!("ETUSCHLOPF: {} {}", colorize(tag_code, format!("[{tag:<12}]")), message.as_ref());
    println!("{shown}");
    capture_console_line(&plain);
}
fn console_artifact_progress(line: ConsoleArtifactLine, label: impl AsRef<str>, done: usize, total: usize, started: Instant) {
    let tag = console_artifact_line_tag(line);
    let context = console_artifact_line_context(line);
    let done = done.min(total.max(1));
    let total = total.max(1);
    let percent = (done.saturating_mul(100) / total).min(100);
    let elapsed = started.elapsed();
    let label_plain = format!("{:<28}", label.as_ref());
    let tag_colored = colorize(progress_color_code_for_context(100, 100, context), format!("[{tag:<12}]"));
    let label_colored = colorize(progress_color_code_for_context(done, total, context), label_plain.clone());
    let percent_text = colorize(progress_color_code_for_context(done, total, context), format!("{percent}%"));
    let shown = format!(
        "ETUSCHLOPF: {} {} {} {}/{} | {} | Laufzeit {}",
        tag_colored,
        label_colored,
        progress_bar(done, total, context),
        done,
        total,
        percent_text,
        format_duration_short(elapsed),
    );
    let plain = format!(
        "ETUSCHLOPF: [{:<12}] {} [{}] {}/{} | {}% | Laufzeit {}",
        tag,
        label_plain.trim_end(),
        progress_bar_plain(done, total),
        done,
        total,
        percent,
        format_duration_short(elapsed),
    );
    clear_live_progress_line();
    println!("{shown}");
    capture_console_line(&plain);
}
fn console_artifact_legend() {
    console_artifact_line(ConsoleArtifactLine::Core, "Farbvertrag: Render-XML blau/hellblau; alle anderen Fortschrittsbalken gruen/hellgruen");
}
fn ascii_progress_enabled() -> bool {
    pipeline_stdout_render_policy().glyphs == GlyphMode::Ascii
}
fn progress_bar(done: usize, total: usize, context: ProgressColorContext) -> String {
    const WIDTH: usize = 20;
    if total == 0 {
        let body = if ascii_progress_enabled() { "--------------------" } else { "────────────────────" };
        return format!("[{}]", colorize("90", body));
    }
    let filled = (done.saturating_mul(WIDTH) + total / 2) / total;
    let filled = filled.min(WIDTH);
    let (done_glyph, open_glyph) = if ascii_progress_enabled() { ("#", ".") } else { ("█", "░") };
    let done_part = colorize(progress_color_code_for_context(done, total, context), done_glyph.repeat(filled));
    let open_part = colorize("90", open_glyph.repeat(WIDTH - filled));
    format!("[{done_part}{open_part}]")
}
fn progress_bar_plain(done: usize, total: usize) -> String {
    const WIDTH: usize = 20;
    let total = total.max(1);
    let filled = ((done.min(total).saturating_mul(WIDTH) + total / 2) / total).min(WIDTH);
    let (done_glyph, open_glyph) = if ascii_progress_enabled() { ("#", ".") } else { ("█", "░") };
    format!("{}{}", done_glyph.repeat(filled), open_glyph.repeat(WIDTH - filled))
}
fn format_duration_short(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{minutes:02}:{seconds:02}")
}
fn should_render_progress_state(label: &str, done: usize, total: usize) -> bool {
    let key = format!("{label}\u{1f}{total}");
    let mut states = PROGRESS_RENDER_STATE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let last_done = states.entry(key).or_insert(0);
    if done == 0 || done > *last_done {
        *last_done = done;
        return true;
    }
    false
}
fn report_progress(label: &str, done: usize, total: usize, workers: usize, started: Instant) {
    if total == 0 {
        return;
    }
    record_progress_timing(label, done, total, workers);
    let percent = (done.saturating_mul(100) / total).min(100);
    let elapsed = started.elapsed();
    let eta = if done > 0 && done < total {
        let per_item = elapsed.as_secs_f64() / done as f64;
        Duration::from_secs_f64(per_item * (total - done) as f64)
    } else {
        Duration::from_secs(0)
    };
    const PROGRESS_LABEL_WIDTH: usize = 28;
    let progress_key = label.to_string();
    let label_plain = format!("{label:<PROGRESS_LABEL_WIDTH$}");
    let context = ProgressColorContext::DefaultArtifact;
    let tag = colorize(
        progress_color_code_for_context(100, 100, context),
        format!("[{:<10}]", "CORE"),
    );
    let label = colorize(progress_color_code_for_context(done, total, context), label_plain);
    let percent_text = colorize(progress_color_code_for_context(done, total, context), format!("{percent}%"));
    let mut parts = vec![
        format!("{} {} {} {}/{}", tag, label, progress_bar(done, total, context), done, total),
        percent_text,
        format!("{} Worker", workers.max(1)),
        format!("Laufzeit {}", format_duration_short(elapsed)),
    ];
    if elapsed >= Duration::from_secs(2) && done > 0 && done < total {
        parts.push(format!("ETA {}", format_duration_short(eta)));
    }
    let line = format!("ETUSCHLOPF: {}", parts.join(" | "));
    capture_console_line(&line);
    if should_render_progress_state(&progress_key, done, total) {
        render_live_progress_line(&line, done >= total);
    }
}

#[cfg(test)]
mod pipeline_console_policy_tests {
    use super::{should_render_progress_state, StreamCapability, StreamDestination};
    use super::{
        resolve_stream_render_policy, ColorPreference, ConsolePreferences, GlyphCapability,
        GlyphPreference, ProgressMode, ProgressPreference, StyleMode,
    };

    #[test]
    fn redirected_pipeline_policy_is_plain_and_line_oriented() {
        let policy = resolve_stream_render_policy(
            StreamCapability {
                destination: StreamDestination::Redirected,
                glyphs: GlyphCapability::Unicode,
            },
            ConsolePreferences {
                color: ColorPreference::Always,
                progress: ProgressPreference::Auto,
                glyphs: GlyphPreference::Auto,
            },
        );
        assert_eq!(policy.style, StyleMode::Plain);
        assert_eq!(policy.progress, ProgressMode::Lines);
    }

    #[test]
    fn parallel_progress_projection_is_monotonic_at_the_sink_boundary() {
        let label = "pipeline-console-monotonic-regression";
        assert!(should_render_progress_state(label, 0, 32));
        assert!(should_render_progress_state(label, 14, 32));
        assert!(!should_render_progress_state(label, 13, 32));
        assert!(should_render_progress_state(label, 15, 32));
        assert!(!should_render_progress_state(label, 15, 32));
        assert!(should_render_progress_state(label, 32, 32));
    }
}
