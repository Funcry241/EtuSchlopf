#![forbid(unsafe_code)]

#[path = "etuschlopf_launcher/startup_timing.rs"]
mod startup_timing;

#[path = "../build_gate/sha256.rs"]
mod sha256;
#[path = "../build_gate/managed_source_tree_definition.rs"]
mod managed_source_tree_definition;
#[path = "../corpus_run_manifest.rs"]
#[allow(dead_code)] // Shared manifest contract is used asymmetrically by launcher and main binary.
mod corpus_run_manifest;
#[path = "../run_outcome.rs"]
mod run_outcome;
#[path = "../audit_only/run_outcome_release_projection.rs"]
mod run_outcome_release_projection;
#[path = "../kodex_control_plane/public_artifact_recovery.rs"]
mod public_artifact_recovery;
#[path = "../visual_preflight.rs"]
mod visual_preflight;
#[allow(dead_code)]
#[path = "../console_presentation_contract.rs"]
mod console_presentation_contract;
#[path = "etuschlopf_launcher/assistant_export.rs"]
mod assistant_export;
#[path = "etuschlopf_launcher/console_live_status.rs"]
mod console_live_status;
#[path = "etuschlopf_launcher/neuro_evidence_export.rs"]
mod neuro_evidence_export;
#[path = "etuschlopf_launcher/inventory_export.rs"]
mod inventory_export;

use anyhow::anyhow;
use flate2::read::ZlibDecoder;
use zip::write::FileOptions;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
use corpus_run_manifest::{
    file_evidence as corpus_run_file_evidence,
    read_manifest as read_corpus_run_manifest,
    validate_manifest as validate_corpus_run_manifest,
    CORPUS_RUN_MANIFEST_FILE,
};
use run_outcome::{
    AuditSignal, CheckOutcome, EmergenceOutcome, FatalOutcome, RendererOutcome,
    RunMode, RunOutcome, RunOutcomeInput, RunOutcomeProjection,
};
use run_outcome_release_projection::{truthful_run_outcome_projection, OutcomeColor};
use console_presentation_contract::{
    resolve_stream_render_policy, Attention, ColorPreference, ConsolePreferences, Emphasis,
    GlyphCapability, GlyphMode, GlyphPreference, LocalStatus, PresentationChannel,
    PresentationLine, PresentationRole, ProgressMode, ProgressPreference, StreamCapability,
    StreamDestination, StreamRenderPolicy, StyleMode, Visibility,
};
use visual_preflight::{
    VisualPreflightExecution, VisualPreflightProjectBatch, VisualPreflightSession,
};
use assistant_export::*;
use inventory_export::*;
use etuschlopf::{observation, observation_protocol, observation_transport};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONSOLE_LOG: &str = "target/etuschlopf_logs/etuschlopf.console.txt";
const VISUAL_PREFLIGHT_REUSE_DIR: &str = "target/etuschlopf_visual_preflight_reuse";
const RENDERER_PROCESS_LIFECYCLE_ASSISTANT_FILE: &str = "renderer_process_lifecycle_audit.txt";
const PERSISTENT_REFERENCE_RENDERER_EVIDENCE_FILE: &str =
    "docs/reference_renderer_written_pdf_nonzero_exit_v1111.txt";
const RUN_OUTCOME_AUDIT_FILE: &str = "run_outcome_audit.txt";
const VISUAL_PREFLIGHT_STRICT_TEST_LOG: &str =
    "target/etuschlopf_logs/visual_preflight.strict-tests.txt";
// r1108 TakeFailureEvidenceFoundationArchiveClosure: the accepted parent bootstrap owns the
// current Take transaction. This launcher only emits typed write-ahead diagnostics when
// a parent bootstrap explicitly supplies an attempt-local evidence directory; therefore
// the capability becomes operative for Takes after the installing Take, never retroactively.
const TAKE_ATTEMPT_EVIDENCE_DIR_ENV: &str = "ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR";
// r1112 TakeFailureEvidenceWriterOwnershipClosure: only the primary launcher process may mutate
// attempt-local stage/artifact write-ahead; inherited Cargo/test descendants remain read-only.
const TAKE_ATTEMPT_EVIDENCE_WRITER_OWNER_FILE: &str = "WRITER_OWNER_PID.txt";
const STRICT_TEST_VALIDATION_RECEIPT: &str =
    "target/etuschlopf_strict_test_validation.receipt";
const STRICT_TEST_VALIDATION_CONTRACT: &str =
    "cargo-test-locked-profile-strict-all-targets-no-fail-fast-opt0-build-override-opt1-repository-observation-v4";
const STRICT_CLIPPY_VALIDATION_RECEIPT: &str =
    "target/etuschlopf_strict_clippy_validation.receipt";
const STRICT_CLIPPY_VALIDATION_CONTRACT: &str =
    "cargo-clippy-locked-profile-strict-color-never-deny-correctness-suspicious-perf-build-override-opt1-clean-v3";
const RUN_LAUNCHER_BOOTSTRAP_RUNS_DIR: &str = "run_launcher_bootstrap_runs";
const TAKE_BOOTSTRAP_RUNS_DIR: &str = "take_bootstrap_runs";
const LEGACY_LAUNCHER_BOOTSTRAP_RUNS_DIR: &str = "launcher_bootstrap_runs";
const ANIMATION_INTERVAL_MS: u64 = 250;

// Historical, version-bound corpus handover and acceptance contracts remain
// compiled because they are negative/kodex evidence, not disposable migration text.
// They are not a generic post-Take full-corpus policy: current same-version Takes
// keep the requested validation target unless an explicit current contract says otherwise.
const CARRY_FORWARD_HANDOVER_FROM_VERSION: &str = "0.10.1265";
const CARRY_FORWARD_HANDOVER_TO_VERSION: &str = "0.10.1266";
const CORPUS_ACCEPTANCE_CONTRACT_VERSION: &str = "0.10.1286";
const CORPUS_ACCEPTANCE_STAMP_DIR: &str = "target/etuschlopf_corpus_acceptance";

type AppResult<T> = anyhow::Result<T>;

fn take_attempt_evidence_dir() -> Option<PathBuf> {
    env::var_os(TAKE_ATTEMPT_EVIDENCE_DIR_ENV)
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
}

fn take_attempt_stage_label(stage: observation::LauncherStage) -> &'static str {
    match stage {
        observation::LauncherStage::Preparing => "preparing",
        observation::LauncherStage::Building => "building",
        observation::LauncherStage::LauncherTests => "launcher_tests",
        observation::LauncherStage::RunningTarget => "running_target",
        observation::LauncherStage::Finalizing => "finalizing",
    }
}

fn claim_take_attempt_evidence_writer_owner(dir: &Path) {
    let owner_path = dir.join(TAKE_ATTEMPT_EVIDENCE_WRITER_OWNER_FILE);
    let body = format!(
        "schema=1\nauthority=none\nwriter_pid={}\n",
        std::process::id()
    );
    if let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(owner_path)
    {
        let _ = file.write_all(body.as_bytes());
        let _ = file.flush();
    }
}

fn initialize_take_attempt_evidence_writer_owner() {
    let Some(dir) = take_attempt_evidence_dir() else {
        return;
    };
    claim_take_attempt_evidence_writer_owner(&dir);
}

fn take_attempt_evidence_writer_owner_pid(dir: &Path) -> Option<u32> {
    fs::read_to_string(dir.join(TAKE_ATTEMPT_EVIDENCE_WRITER_OWNER_FILE))
        .ok()?
        .lines()
        .find_map(|line| line.strip_prefix("writer_pid="))?
        .parse::<u32>()
        .ok()
}

fn current_process_owns_take_attempt_evidence(dir: &Path) -> bool {
    take_attempt_evidence_writer_owner_pid(dir) == Some(std::process::id())
}

fn write_take_attempt_stage_to_dir(dir: &Path, stage: observation::LauncherStage) {
    if !current_process_owns_take_attempt_evidence(dir) {
        return;
    }
    let body = format!(
        "schema=1\nauthority=none\nlauncher_stage={}\n",
        take_attempt_stage_label(stage)
    );
    let _ = fs::write(dir.join("CHILD_STAGE.txt"), body);
}

fn write_take_attempt_stage(stage: observation::LauncherStage) {
    let Some(dir) = take_attempt_evidence_dir() else {
        return;
    };
    write_take_attempt_stage_to_dir(&dir, stage);
}

fn register_take_attempt_artifact_to_dir(dir: &Path, role: &str, path: &Path) {
    if !current_process_owns_take_attempt_evidence(dir) {
        return;
    }
    let raw = path.to_string_lossy().replace('\\', "/");
    if raw.chars().any(|ch| matches!(ch, '\n' | '\r' | '\t')) {
        return;
    }
    let line = format!("role={role}\tpath={raw}\n");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("CHILD_ARTIFACT_REGISTRY.txt"))
    {
        let _ = file.write_all(line.as_bytes());
        let _ = file.flush();
    }
}

fn register_take_attempt_artifact(role: &str, path: &Path) {
    let Some(dir) = take_attempt_evidence_dir() else {
        return;
    };
    register_take_attempt_artifact_to_dir(&dir, role, path);
}

fn observation_outcome_color(color: OutcomeColor) -> observation::SourceOutcomeColor {
    match color {
        OutcomeColor::DarkGreen => observation::SourceOutcomeColor::DarkGreen,
        OutcomeColor::LightGreen => observation::SourceOutcomeColor::LightGreen,
        OutcomeColor::Yellow => observation::SourceOutcomeColor::Yellow,
        OutcomeColor::Red => observation::SourceOutcomeColor::Red,
    }
}

#[derive(Debug)]
struct LauncherRequestedExit {
    code: i32,
}

impl std::fmt::Display for LauncherRequestedExit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "launcher requested exit {}", self.code)
    }
}

impl std::error::Error for LauncherRequestedExit {}

fn launcher_exit<T>(code: i32) -> AppResult<T> {
    Err(anyhow::Error::new(LauncherRequestedExit { code }))
}

fn is_launcher_requested_exit(error: &anyhow::Error) -> bool {
    error.downcast_ref::<LauncherRequestedExit>().is_some()
}

fn normalized_launcher_exit_code(code: i32) -> u8 {
    u8::try_from(code).unwrap_or(1)
}

#[derive(Clone, Copy)]
struct ProgressCompletion<'a> {
    role: PresentationRole,
    label: &'a str,
    elapsed: Duration,
    detail: CompletionDetail<'a>,
    exit_code: i32,
}

/// Muted tail of a completion line.
#[derive(Clone, Copy)]
enum CompletionDetail<'a> {
    LogSize(u64),
    Text(&'a str),
    None,
}

impl CompletionDetail<'_> {
    fn suffix(self) -> String {
        match self {
            Self::LogSize(bytes) => format!(" · log {}", format_log_size(bytes)),
            Self::Text(text) if !text.is_empty() => format!(" · {text}"),
            Self::Text(_) | Self::None => String::new(),
        }
    }
}

/// What the completion line of a logged process shows after its duration.
#[derive(Clone, Copy, PartialEq, Eq)]
enum CompletionSummary {
    LogSize,
    CargoTestCounts,
}

struct ConsoleSession {
    stdout_policy: StreamRenderPolicy,
    stderr_policy: StreamRenderPolicy,
    live_line_active: bool,
    live_line_width: usize,
}

impl ConsoleSession {
    fn detect() -> Self {
        let preferences = console_preferences_from_env();
        let stdout_capability = stream_capability(io::stdout().is_terminal());
        let stderr_capability = stream_capability(io::stderr().is_terminal());
        Self {
            stdout_policy: resolve_stream_render_policy(stdout_capability, preferences),
            stderr_policy: resolve_stream_render_policy(stderr_capability, preferences),
            live_line_active: false,
            live_line_width: 0,
        }
    }

    fn stdout_progress_mode(&self) -> ProgressMode {
        self.stdout_policy.progress
    }

    fn stdout_glyph_mode(&self) -> GlyphMode {
        self.stdout_policy.glyphs
    }

    /// Removes a still visible in-place running line, so the next regular line replaces it
    /// instead of leaving its last frame behind.
    fn clear_live_line(&mut self) {
        if self.live_line_active {
            print!("\r{}\r", " ".repeat(self.live_line_width));
            self.live_line_active = false;
            self.live_line_width = 0;
        }
    }

    fn write_stdout_line(&mut self, line: PresentationLine<'_>) {
        self.clear_live_line();
        write_presentation_line(io::stdout(), self.stdout_policy, line);
    }

    fn write_stdout_segments(&mut self, segments: &[ConsoleSegment<'_>]) {
        self.clear_live_line();
        let mut stdout = io::stdout().lock();
        let _ = write_console_segments(&mut stdout, self.stdout_policy, segments);
        let _ = writeln!(stdout);
        let _ = stdout.flush();
    }

    fn write_stdout_inline(&mut self, line: PresentationLine<'_>) -> io::Result<()> {
        debug_assert_eq!(self.stdout_policy.progress, ProgressMode::Inline);
        let mut stdout = io::stdout().lock();
        let has_line_break = line.text.ends_with('\n');
        let without_break = line.text.strip_suffix('\n').unwrap_or(line.text);
        let content = without_break.strip_prefix('\r').unwrap_or(without_break);
        let visible_width = visible_console_width(content);
        write!(stdout, "\r")?;
        write_presentation(&mut stdout, self.stdout_policy, line, content)?;
        let trailing_clear = self.live_line_width.saturating_sub(visible_width);
        if trailing_clear > 0 {
            write!(stdout, "{}", " ".repeat(trailing_clear))?;
        }
        if has_line_break {
            writeln!(stdout)?;
            self.live_line_active = false;
            self.live_line_width = 0;
        } else {
            self.live_line_active = true;
            self.live_line_width = visible_width;
        }
        stdout.flush()?;
        Ok(())
    }

    fn write_stdout_inline_segments(
        &mut self,
        role: PresentationRole,
        running: &RunningLine<'_>,
    ) -> io::Result<()> {
        debug_assert_eq!(self.stdout_policy.progress, ProgressMode::Inline);
        let plain = running.plain();
        let visible_width = visible_console_width(&plain);
        let mut stdout = io::stdout().lock();
        write!(stdout, "\r")?;
        write_console_segments(
            &mut stdout,
            self.stdout_policy,
            &[
                ConsoleSegment::muted("[EtuSchlopf]"),
                ConsoleSegment::plain(" "),
                ConsoleSegment::role(role, running.label),
                ConsoleSegment::plain(" "),
                ConsoleSegment::muted(&running.tail),
            ],
        )?;
        let trailing_clear = self.live_line_width.saturating_sub(visible_width);
        if trailing_clear > 0 {
            write!(stdout, "{}", " ".repeat(trailing_clear))?;
        }
        self.live_line_active = true;
        self.live_line_width = visible_width;
        stdout.flush()?;
        Ok(())
    }

    fn write_stderr_line(&mut self, line: PresentationLine<'_>) {
        write_presentation_line(io::stderr(), self.stderr_policy, line);
    }

    fn write_progress_done(
        &mut self,
        version: &str,
        completion: ProgressCompletion<'_>,
    ) -> io::Result<()> {
        let plain = progress_done_plain(version, completion);
        let visible_width = visible_console_width(&plain);
        let mut stdout = io::stdout().lock();
        if self.live_line_active {
            write!(stdout, "\r")?;
        }
        write_progress_done_segments(&mut stdout, self.stdout_policy, version, completion)?;
        let trailing_clear = self.live_line_width.saturating_sub(visible_width);
        if trailing_clear > 0 {
            write!(stdout, "{}", " ".repeat(trailing_clear))?;
        }
        writeln!(stdout)?;
        stdout.flush()?;
        self.live_line_active = false;
        self.live_line_width = 0;
        Ok(())
    }

    fn write_outcome_headline(
        &mut self,
        _version: &str,
        outcome: OutcomeColor,
        status: &str,
        project: &str,
        exit_code: i32,
        total: &str,
    ) {
        self.clear_live_line();
        let mut stdout = io::stdout().lock();
        if self.stdout_policy.style == StyleMode::Styled {
            let _ = write!(
                stdout,
                "\x1b[90m[EtuSchlopf]\x1b[0m {}{}\x1b[0m \x1b[37m| {project}\x1b[0m \x1b[90m| exit {exit_code} | gesamt {total} ab Launcherstart\x1b[0m",
                ansi_outcome_style(outcome),
                status
            );
        } else {
            let _ = write!(
                stdout,
                "[EtuSchlopf] {status} | {project} | exit {exit_code} | gesamt {total} ab Launcherstart"
            );
        }
        let _ = writeln!(stdout);
        let _ = stdout.flush();
    }
}

impl Drop for ConsoleSession {
    fn drop(&mut self) {
        if self.live_line_active {
            println!();
            self.live_line_active = false;
            self.live_line_width = 0;
        }
    }
}

fn stream_capability(is_terminal: bool) -> StreamCapability {
    StreamCapability {
        destination: if is_terminal {
            StreamDestination::Terminal
        } else {
            StreamDestination::Redirected
        },
        glyphs: GlyphCapability::Unicode,
    }
}

fn console_preferences_from_env() -> ConsolePreferences {
    let color = if env::var_os("NO_COLOR").is_some() {
        ColorPreference::Never
    } else {
        match normalized_env_value("ETUSCHLOPF_COLOR").as_deref() {
            Some("0" | "false" | "off" | "no" | "never") => ColorPreference::Never,
            Some("1" | "true" | "on" | "yes" | "always") => ColorPreference::Always,
            _ => ColorPreference::Auto,
        }
    };
    let progress = match normalized_env_value("ETUSCHLOPF_PROGRESS").as_deref() {
        Some("0" | "false" | "off" | "no" | "plain") => ProgressPreference::Plain,
        _ => ProgressPreference::Auto,
    };
    let glyphs = match normalized_env_value("ETUSCHLOPF_ASCII_PROGRESS").as_deref() {
        Some("1" | "true" | "on" | "yes" | "ascii") => GlyphPreference::Ascii,
        _ => GlyphPreference::Auto,
    };
    ConsolePreferences {
        color,
        progress,
        glyphs,
    }
}

fn normalized_env_value(name: &str) -> Option<String> {
    env::var(name).ok().map(|value| value.trim().to_ascii_lowercase())
}

#[derive(Clone, Copy)]
enum ConsoleSegmentStyle {
    Plain,
    Muted,
    Role(PresentationRole),
    Status(LocalStatus),
}

#[derive(Clone, Copy)]
struct ConsoleSegment<'a> {
    text: &'a str,
    style: ConsoleSegmentStyle,
}

impl<'a> ConsoleSegment<'a> {
    fn plain(text: &'a str) -> Self {
        Self { text, style: ConsoleSegmentStyle::Plain }
    }

    fn muted(text: &'a str) -> Self {
        Self { text, style: ConsoleSegmentStyle::Muted }
    }

    fn role(role: PresentationRole, text: &'a str) -> Self {
        Self { text, style: ConsoleSegmentStyle::Role(role) }
    }

    fn status(status: LocalStatus, text: &'a str) -> Self {
        Self { text, style: ConsoleSegmentStyle::Status(status) }
    }
}

fn write_console_segments<W: Write>(
    writer: &mut W,
    policy: StreamRenderPolicy,
    segments: &[ConsoleSegment<'_>],
) -> io::Result<()> {
    for segment in segments {
        if policy.style != StyleMode::Styled {
            write!(writer, "{}", segment.text)?;
            continue;
        }
        let style = match segment.style {
            ConsoleSegmentStyle::Plain => "",
            ConsoleSegmentStyle::Muted => "\x1b[90m",
            ConsoleSegmentStyle::Role(role) => ansi_role_style(role),
            ConsoleSegmentStyle::Status(status) => ansi_local_status_style(status),
        };
        if style.is_empty() {
            write!(writer, "{}", segment.text)?;
        } else {
            write!(writer, "{style}{}\x1b[0m", segment.text)?;
        }
    }
    Ok(())
}

fn ansi_local_status_style(status: LocalStatus) -> &'static str {
    match status {
        LocalStatus::Ok | LocalStatus::Committed => "\x1b[32m",
        LocalStatus::Blocked | LocalStatus::RolledBack => "\x1b[93m",
        LocalStatus::Failed => "\x1b[91m",
        LocalStatus::Pending | LocalStatus::Running => "\x1b[37m",
    }
}

fn write_presentation_line<W: Write>(
    mut writer: W,
    policy: StreamRenderPolicy,
    line: PresentationLine<'_>,
) {
    let _ = write_presentation(&mut writer, policy, line, line.text);
    let _ = writeln!(writer);
    let _ = writer.flush();
}

fn write_presentation<W: Write>(
    writer: &mut W,
    policy: StreamRenderPolicy,
    line: PresentationLine<'_>,
    message: &str,
) -> io::Result<()> {
    if policy.style == StyleMode::Styled {
        write!(writer, "{}{}\x1b[0m", ansi_style_for_presentation(line), message)
    } else {
        write!(writer, "{message}")
    }
}

fn ansi_style_for_presentation(line: PresentationLine<'_>) -> &'static str {
    match line.attention {
        Attention::Error => return "\x1b[91m",
        Attention::Warning => return "\x1b[93m",
        Attention::Neutral | Attention::Notice => {}
    }
    if line.emphasis == Emphasis::Muted {
        return "\x1b[90m";
    }
    match line.role {
        PresentationRole::Infrastructure => "\x1b[90m",
        PresentationRole::Take => "\x1b[36m",
        PresentationRole::Kodex | PresentationRole::Check | PresentationRole::Test => "\x1b[36m",
        PresentationRole::Build => "\x1b[35m",
        PresentationRole::Program | PresentationRole::Renderer => "\x1b[37m",
        PresentationRole::Artifact | PresentationRole::Export => "\x1b[94m",
        PresentationRole::Result => "\x1b[37m",
    }
}

fn ansi_outcome_style(outcome: OutcomeColor) -> &'static str {
    match outcome {
        OutcomeColor::DarkGreen => "\x1b[1;32m",
        OutcomeColor::LightGreen => "\x1b[92m",
        OutcomeColor::Yellow => "\x1b[1;33m",
        OutcomeColor::Red => "\x1b[1;31m",
    }
}

fn ansi_role_style(role: PresentationRole) -> &'static str {
    match role {
        PresentationRole::Infrastructure => "\x1b[90m",
        PresentationRole::Take => "\x1b[36m",
        PresentationRole::Kodex | PresentationRole::Check | PresentationRole::Test => "\x1b[36m",
        PresentationRole::Build => "\x1b[35m",
        PresentationRole::Program | PresentationRole::Renderer => "\x1b[37m",
        PresentationRole::Artifact | PresentationRole::Export => "\x1b[94m",
        PresentationRole::Result => "\x1b[37m",
    }
}

fn progress_done_plain(_version: &str, completion: ProgressCompletion<'_>) -> String {
    let ProgressCompletion {
        label,
        elapsed,
        detail,
        exit_code,
        ..
    } = completion;
    let completion_label = if exit_code == 0 {
        "OK".to_string()
    } else {
        format!("ENDE exit {exit_code}")
    };
    format!(
        "[EtuSchlopf] {label} {completion_label} {}{}",
        format_elapsed(elapsed),
        detail.suffix()
    )
}

fn write_progress_done_segments<W: Write>(
    writer: &mut W,
    policy: StreamRenderPolicy,
    version: &str,
    completion: ProgressCompletion<'_>,
) -> io::Result<()> {
    if policy.style != StyleMode::Styled {
        return write!(writer, "{}", progress_done_plain(version, completion));
    }

    write!(writer, "\x1b[90m[EtuSchlopf]\x1b[0m ")?;
    write!(
        writer,
        "{}{}\x1b[0m",
        ansi_role_style(completion.role),
        completion.label
    )?;
    if completion.exit_code == 0 {
        write!(writer, " \x1b[32mOK\x1b[0m")?;
    } else {
        write!(
            writer,
            " \x1b[91mENDE exit {}\x1b[0m",
            completion.exit_code
        )?;
    }
    write!(
        writer,
        " \x1b[90m{}{}\x1b[0m",
        format_elapsed(completion.elapsed),
        completion.detail.suffix()
    )
}

fn presentation_line<'a>(
    role: PresentationRole,
    status: Option<LocalStatus>,
    attention: Attention,
    emphasis: Emphasis,
    text: &'a str,
) -> PresentationLine<'a> {
    PresentationLine {
        role,
        status,
        attention,
        emphasis,
        visibility: Visibility::InteractiveOnly,
        channel: PresentationChannel::Narrative,
        text,
    }
}

fn progress_line_presentation(role: PresentationRole, text: &str) -> PresentationLine<'_> {
    let (display_role, attention, emphasis) = match role {
        PresentationRole::Infrastructure => (
            PresentationRole::Infrastructure,
            Attention::Neutral,
            Emphasis::Secondary,
        ),
        PresentationRole::Take => (
            PresentationRole::Take,
            Attention::Notice,
            Emphasis::Secondary,
        ),
        PresentationRole::Kodex | PresentationRole::Check | PresentationRole::Test => (
            PresentationRole::Check,
            Attention::Neutral,
            Emphasis::Secondary,
        ),
        PresentationRole::Build => (
            PresentationRole::Build,
            Attention::Neutral,
            Emphasis::Secondary,
        ),
        PresentationRole::Program | PresentationRole::Renderer | PresentationRole::Artifact => (
            PresentationRole::Program,
            Attention::Neutral,
            Emphasis::Primary,
        ),
        PresentationRole::Export | PresentationRole::Result => (
            PresentationRole::Result,
            Attention::Notice,
            Emphasis::Secondary,
        ),
    };
    presentation_line(display_role, None, attention, emphasis, text)
}

#[derive(Clone, Copy)]
enum ConsolePrefix {
    None,
    Product,
    Versioned,
}

struct Launcher {
    version: String,
    thread_plan: ThreadPlan,
    console: ConsoleSession,
    observation: observation::SourceStateOwner,
    observation_transport: observation_transport::SnapshotDatagramPublisher,
    watch_process: Option<Child>,
    started: Instant,
}

fn projekt_count(count: usize) -> String {
    if count == 1 {
        "1 Projekt".to_string()
    } else {
        format!("{} Projekte", count)
    }
}

fn artefakt_count(count: usize) -> String {
    if count == 1 {
        "1 Artefakt".to_string()
    } else {
        format!("{} Artefakte", count)
    }
}

#[derive(Clone)]
struct ThreadPlan {
    logical_threads: usize,
    cargo_jobs: usize,
    pipeline_auto_hint_workers: usize,
}

impl ThreadPlan {
    fn detect() -> Self {
        let logical_threads = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1)
            .max(1);
        let reserved_for_ui = usize::from(logical_threads >= 4);
        let cargo_jobs = logical_threads.saturating_sub(reserved_for_ui).max(1);
        let pipeline_auto_hint_workers = logical_threads.saturating_sub(1).max(1);
        Self {
            logical_threads,
            cargo_jobs,
            pipeline_auto_hint_workers,
        }
    }

    fn summary_label(&self) -> String {
        format!(
            "System {} | CargoJobs {} | PipelineAutoHint {}",
            self.logical_threads, self.cargo_jobs, self.pipeline_auto_hint_workers
        )
    }

    fn interactive_summary_label(&self) -> String {
        format!("System {} · Jobs {}", self.logical_threads, self.cargo_jobs)
    }
}

struct RunResult {
    exit_code: i32,
    elapsed_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanupFailure {
    path: PathBuf,
    error: String,
}

#[derive(Default)]
struct TempCleanupReport {
    os_temp_dirs_deleted: usize,
    bootstrap_dirs_deleted: usize,
    versioned_bootstrap_dirs_deleted: usize,
    download_extract_dirs_deleted: usize,
    skipped_current_dirs: usize,
    failures: Vec<CleanupFailure>,
}

impl TempCleanupReport {
    fn deleted_total(&self) -> usize {
        self.os_temp_dirs_deleted
            + self.bootstrap_dirs_deleted
            + self.versioned_bootstrap_dirs_deleted
            + self.download_extract_dirs_deleted
    }

    fn failed_removals(&self) -> usize {
        self.failures.len()
    }
}

#[derive(Default)]
struct RootCleanupReport {
    stale_zip_files_deleted: usize,
    stale_output_files_deleted: usize,
    stale_log_files_deleted: usize,
    temp_files_deleted: usize,
    temp_dirs_deleted: usize,
    skipped_protected: usize,
    failed_removals: usize,
}

impl RootCleanupReport {
    fn deleted_total(&self) -> usize {
        self.stale_zip_files_deleted
            + self.stale_output_files_deleted
            + self.stale_log_files_deleted
            + self.temp_files_deleted
            + self.temp_dirs_deleted
    }
}

#[derive(Clone, Copy, Default)]
struct DirectoryTreeStats {
    files: usize,
    directories: usize,
    bytes: u64,
}

impl DirectoryTreeStats {
    fn add(&mut self, other: Self) {
        self.files = self.files.saturating_add(other.files);
        self.directories = self.directories.saturating_add(other.directories);
        self.bytes = self.bytes.saturating_add(other.bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CachePathPolicy {
    PlainDirectory,
    CargoProfile { protect_active_executable: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CachePathOutcome {
    Absent,
    Deleted,
    SkippedActiveExecutable,
    SkippedNotCargoProfile,
    Failed(String),
}

impl CachePathOutcome {
    fn label(&self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Deleted => "deleted",
            Self::SkippedActiveExecutable => "skipped_active_executable",
            Self::SkippedNotCargoProfile => "skipped_not_cargo_profile",
            Self::Failed(_) => "failed",
        }
    }

    fn is_failure(&self) -> bool {
        matches!(self, Self::Failed(_))
    }

    fn is_active_skip(&self) -> bool {
        matches!(self, Self::SkippedActiveExecutable)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CacheCleanupEntry {
    name: &'static str,
    path: PathBuf,
    outcome: CachePathOutcome,
}

#[derive(Default)]
struct BuildCacheHygieneReport {
    before: DirectoryTreeStats,
    after: DirectoryTreeStats,
    entries: Vec<CacheCleanupEntry>,
}

impl BuildCacheHygieneReport {
    fn bytes_reclaimed(&self) -> u64 {
        self.before.bytes.saturating_sub(self.after.bytes)
    }

    fn failed_removals(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.outcome.is_failure())
            .count()
    }

    fn active_profile_skips(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.outcome.is_active_skip())
            .count()
    }

    #[cfg(test)]
    fn outcome(&self, name: &str) -> Option<&CachePathOutcome> {
        self.entries
            .iter()
            .find(|entry| entry.name == name)
            .map(|entry| &entry.outcome)
    }
}

/// Upper bound for the visible width of the in-place running line.
const RUNNING_LINE_MAX_CHARS: usize = 78;

/// One in-place running line: role-coloured label, muted tail (r79 segment roles).
struct RunningLine<'a> {
    label: &'a str,
    tail: String,
}

impl RunningLine<'_> {
    fn plain(&self) -> String {
        format!("[EtuSchlopf] {} {}", self.label, self.tail)
    }
}

struct ProgressAnimation {
    tick: usize,
}

impl ProgressAnimation {
    fn new() -> Self {
        Self { tick: 0 }
    }

    fn advance(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    fn running_line<'a>(
        &self,
        heartbeat: &'a str,
        elapsed: Duration,
        previous: Option<Duration>,
        live_summary: Option<&str>,
        glyphs: GlyphMode,
    ) -> RunningLine<'a> {
        // With a previous duration: "2.5/9.1min" = elapsed / last successful run.
        let time = previous.map_or_else(
            || format_elapsed(elapsed),
            |previous| format!("{:.1}/{}", elapsed.as_secs_f64() / 60.0, format_elapsed(previous)),
        );
        let mut tail = format!("{} {time}", spinner_symbol(self.tick, glyphs));
        if let Some(summary) = live_summary {
            tail.push_str(" · ");
            tail.push_str(summary);
        }
        RunningLine { label: heartbeat, tail }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArgsState {
    request: RunRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RunRequest {
    DeepClippy {
        build_preference: BuildPreference,
    },
    Execute(ExecutionRequest),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExecutionRequest {
    target: RunTarget,
    program_args: Vec<OsString>,
    validation: ValidationPlan,
    build_preference: BuildPreference,
    launcher_tests: LauncherTestPlan,
    export_policy: AssistantExportPolicy,
    watch: WatchPlan,
    neuro_corpus_publish: bool,
    inventory: InventoryPlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RunTarget {
    Missing,
    Project(String),
    Corpus,
    VisualPreflight,
}

impl RunTarget {
    fn input_arg(&self) -> Option<&str> {
        match self {
            Self::Missing => None,
            Self::Project(project) => Some(project.as_str()),
            Self::Corpus => Some("--all"),
            Self::VisualPreflight => Some("--visual-preflight"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BuildPreference {
    ReuseVerified,
    ForceCompile,
}

impl BuildPreference {
    const fn forces_compile(self) -> bool {
        matches!(self, Self::ForceCompile)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LauncherTestPlan {
    Standard,
    VisualPreflight,
}

impl LauncherTestPlan {
    const fn visual_preflight_requested(self) -> bool {
        matches!(self, Self::VisualPreflight)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssistantExportPolicy {
    Minimal,
    IncludePdf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WatchPlan {
    Disabled,
    Enabled,
}

impl WatchPlan {
    const fn requested(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

impl ArgsState {
    fn execution(&self) -> Option<&ExecutionRequest> {
        match &self.request {
            RunRequest::Execute(request) => Some(request),
            RunRequest::DeepClippy { .. } => None,
        }
    }

    fn validation(&self) -> ValidationPlan {
        match &self.request {
            RunRequest::DeepClippy { .. } => ValidationPlan::DeepClippy,
            RunRequest::Execute(request) => request.validation,
        }
    }

    fn build_preference(&self) -> BuildPreference {
        match &self.request {
            RunRequest::DeepClippy { build_preference } => *build_preference,
            RunRequest::Execute(request) => request.build_preference,
        }
    }


    fn input_arg(&self) -> Option<&str> {
        self.execution().and_then(|request| request.target.input_arg())
    }

    #[cfg(test)]
    fn program_args(&self) -> &[OsString] {
        self.execution()
            .map(|request| request.program_args.as_slice())
            .unwrap_or(&[])
    }

    #[cfg(test)]
    fn visual_preflight_requested(&self) -> bool {
        self.execution()
            .is_some_and(|request| request.launcher_tests.visual_preflight_requested())
    }

    #[cfg(test)]
    fn watch_requested(&self) -> bool {
        self.execution()
            .is_some_and(|request| request.watch.requested())
    }

    #[cfg(test)]
    fn inventory_requested(&self) -> bool {
        self.execution()
            .is_some_and(|request| request.inventory.requested())
    }

}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RenderCandidateSelectedPdfTruth {
    write_produced: bool,
    write_validated: bool,
    selected_pdf_source: String,
    selected_pdf_source_label: String,
    pdf_source_adopted: bool,
    productive_materialization_green: bool,
    candidate_verdict: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CargoBuildProfile {
    Release,
    Strict,
}

impl CargoBuildProfile {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Strict => "strict",
        }
    }

    fn target_exe(self) -> PathBuf {
        target_exe_for_profile(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationPlan {
    Strict,
    DeepClippy,
}

impl ValidationPlan {
    const fn with_strict(self) -> Self {
        match self {
            Self::DeepClippy => Self::DeepClippy,
            Self::Strict => Self::Strict,
        }
    }

    const fn build_profile(self) -> CargoBuildProfile {
        match self {
            Self::Strict | Self::DeepClippy => CargoBuildProfile::Strict,
        }
    }

    const fn is_strict(self) -> bool {
        matches!(self, Self::Strict | Self::DeepClippy)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompileReason {
    Forced,
    MissingVerifiedArtifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BuildPlan {
    Reuse(CargoBuildProfile),
    Compile {
        profile: CargoBuildProfile,
        reason: CompileReason,
    },
}

impl BuildPlan {
    const fn profile(self) -> CargoBuildProfile {
        match self {
            Self::Reuse(profile) | Self::Compile { profile, .. } => profile,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ArtifactAvailability {
    release: bool,
    strict: bool,
}

impl ArtifactAvailability {
    fn verified_profile(self, requested: CargoBuildProfile) -> Option<CargoBuildProfile> {
        match requested {
            CargoBuildProfile::Strict => self.strict.then_some(CargoBuildProfile::Strict),
            CargoBuildProfile::Release if self.release => Some(CargoBuildProfile::Release),
            CargoBuildProfile::Release => self.strict.then_some(CargoBuildProfile::Strict),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BuildOutcome {
    Pending(CargoBuildProfile),
    Reused(CargoBuildProfile),
    Compiled(CargoBuildProfile),
}

impl BuildOutcome {
    const fn profile(self) -> CargoBuildProfile {
        match self {
            Self::Pending(profile) | Self::Reused(profile) | Self::Compiled(profile) => profile,
        }
    }

    fn decision_label(self) -> String {
        match self {
            Self::Pending(_) => "pending".to_string(),
            Self::Reused(profile) => format!("reused_verified_{}_artifact", profile.as_str()),
            Self::Compiled(profile) => format!("compiled_{}_artifact", profile.as_str()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LauncherPlan {
    validation: ValidationPlan,
    build: BuildPlan,
}

struct RunState {
    validation: ValidationPlan,
    build_outcome: BuildOutcome,
    thread_plan: ThreadPlan,
    target_exe: PathBuf,
    strict_clippy_exit: String,
    build_exit: String,
    strict_test_exit: String,
    run_exit: String,
    clippy_ms: String,
    build_ms: String,
    strict_test_ms: String,
    strict_test_summary: String,
    program_ms: String,
    total_start: Instant,
    strict_log: Option<PathBuf>,
    strict_test_log: Option<PathBuf>,
    result_file: Option<PathBuf>,
    input_arg: Option<String>,
    force_mode: bool,
    audit_signal: AuditSignal,
    renderer_outcome: RendererOutcome,
    emergence_outcome: EmergenceOutcome,
    render_candidate_selected_pdf_truth: Option<RenderCandidateSelectedPdfTruth>,
    export_policy: AssistantExportPolicy,
}

fn main() -> ExitCode {
    install_panic_console_hook();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            if let Some(requested) = err.downcast_ref::<LauncherRequestedExit>() {
                return ExitCode::from(normalized_launcher_exit_code(requested.code));
            }
            let mut console = ConsoleSession::detect();
            let message = format!("[EtuSchlopf] FEHLER: {err}");
            let line = presentation_line(
                PresentationRole::Result,
                Some(LocalStatus::Failed),
                Attention::Error,
                Emphasis::Primary,
                &message,
            );
            console.write_stderr_line(line);
            ExitCode::from(1)
        }
    }
}

fn install_panic_console_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        default_hook(panic_info);
    }));
}

fn run() -> AppResult<()> {
    let prelude_timing = LauncherPhaseTimer::new("LauncherPrelude");
    let startup_entry_timing = startup_timing::Phase::new("LauncherEntryBeforeConsole");
    initialize_take_attempt_evidence_writer_owner();
    managed_source_tree_definition::validate_root_project_file_roles().map_err(|error| anyhow!(error))?;
    let project_root = env::current_dir()?;
    let raw_args: Vec<OsString> = env::args_os().skip(1).collect();
    if raw_args.iter().any(|arg| {
        let value = arg.to_string_lossy();
        value.eq_ignore_ascii_case("--take")
            || value.eq_ignore_ascii_case("--take-zip")
            || strip_option_value_ignore_ascii_case(&value, "--take-zip=").is_some()
    }) {
        return Err(anyhow!(
            "Direkter Launcher-Take ist fail-closed pensioniert; run.cmd besitzt den transaktionalen Rust-Take-Bootstrap."
        ));
    }
    let mut args = parse_args(raw_args);
    let public_transaction_enabled = is_public_artifact_transaction_eligible(&args);
    let _project_run_lock = if public_transaction_enabled {
        Some(
            public_artifact_recovery::acquire_project_run_lock(&project_root)
                .map_err(|error| anyhow!(error))?,
        )
    } else {
        None
    };
    let public_recovery = public_artifact_recovery::recover_pending_public_artifacts(&project_root)
        .map_err(|error| anyhow!(error))?;
    if public_recovery.performed_recovery() {
        let message = format!(
            "[EtuSchlopf] PUBLIC-ARTIFACT-RECOVERY vor Launcher-Cleanup: {}.",
            public_recovery.as_str()
        );
        let mut startup_console = ConsoleSession::detect();
        let line = presentation_line(
            PresentationRole::Check,
            None,
            Attention::Neutral,
            Emphasis::Secondary,
            &message,
        );
        startup_console.write_stderr_line(line);
    }
    apply_version_bound_post_take_contract(&project_root, &mut args)?;
    remove_obsolete_powershell_files(&project_root);
    let version = read_cargo_version(&project_root.join("Cargo.toml")).unwrap_or_else(|| VERSION.to_string());
    let mut launcher = Launcher::new(version);

    drop(startup_entry_timing);
    init_console_log(&launcher.version, &launcher.thread_plan)?;
    drop(prelude_timing);

    launcher.line(
        PresentationRole::Infrastructure,
        None,
        Attention::Neutral,
        Emphasis::Secondary,
        "",
    );
    let target = match &args.request {
        RunRequest::DeepClippy { .. } => "Deep Clippy".to_string(),
        RunRequest::Execute(request) => request
            .target
            .input_arg()
            .unwrap_or("Start")
            .to_string(),
    };
    let revisions = assistant_export::console_revision_projection(&env::current_dir()?);
    let take_active = env::var("ETUSCHLOPF_TAKE_FROM").is_ok() && env::var("ETUSCHLOPF_TAKE_TO").is_ok();
    launcher.line(
        PresentationRole::Infrastructure,
        None,
        Attention::Neutral,
        Emphasis::Secondary,
        &startup_header_text(
            &revisions,
            take_active,
            &target,
            &launcher.thread_plan.interactive_summary_label(),
        ),
    );
    if take_active {
        let change_class = env::var("ETUSCHLOPF_TAKE_CHANGE_CLASS").unwrap_or_default();
        launcher.role_detail_line(
            ConsolePrefix::None,
            PresentationRole::Take,
            "Take",
            &take_header_detail(&revisions, &change_class),
            Emphasis::Secondary,
        );
    }
    let preparation_times = startup_preparation_times(&startup_timing::readback());
    remember_bootstrap_build_seconds(&preparation_times);
    if let Some(preparation) = preparation_times.summary() {
        launcher.role_detail_line(
            ConsolePrefix::None,
            PresentationRole::Infrastructure,
            "Vorlauf",
            &preparation,
            Emphasis::Secondary,
        );
    }

    if args
        .execution()
        .is_some_and(|request| request.watch.requested())
    {
        launcher.start_watch_best_effort(&project_root);
    }

    let cleanup_timing = LauncherPhaseTimer::new("StartupCleanup");
    let startup_cleanup = cleanup_program_temp_leftovers(&project_root);
    append_temp_cleanup_to_console_log("Start", &startup_cleanup)?;
    let startup_build_cache_hygiene = cleanup_build_cache_hygiene(&project_root);
    append_build_cache_hygiene_to_console_log("Start", &startup_build_cache_hygiene)?;
    let startup_root_cleanup = cleanup_startup_project_root_residue(&project_root, &args);
    append_root_cleanup_to_console_log("Start", &startup_root_cleanup)?;

    drop(cleanup_timing);
    let plan_timing = LauncherPhaseTimer::new("BuildPlanAndArtifactVerification");
    let plan = plan_launcher(&project_root, &launcher.version, &args)?;
    drop(plan_timing);
    match &args.request {
        RunRequest::DeepClippy { .. } => {
            execute_deep_clippy_diagnostic(&mut launcher, plan.build)
        }
        RunRequest::Execute(request) => {
            let result = execute_normal_run(
                &mut launcher,
                request,
                plan,
                &project_root,
                public_transaction_enabled,
            );
            if result
                .as_ref()
                .err()
                .is_some_and(|error| !is_launcher_requested_exit(error))
            {
                launcher.fail_observation();
            }
            result
        }
    }
}

impl Launcher {
    fn new(version: String) -> Self {
        let observation = observation::SourceStateOwner::launcher();
        let observation_transport =
            observation_transport::SnapshotDatagramPublisher::local(&observation);
        Self {
            version,
            thread_plan: ThreadPlan::detect(),
            console: ConsoleSession::detect(),
            observation,
            observation_transport,
            watch_process: None,
            started: Instant::now(),
        }
    }

    fn start_watch_best_effort(&mut self, project_root: &Path) {
        const WATCH_MANIFEST: &str = "tools/etuschlopf_watch/Cargo.toml";
        const WATCH_TARGET_DIR: &str = "target/etuschlopf_watch_runtime";

        let manifest = project_root.join(WATCH_MANIFEST);
        let target_dir = project_root.join(WATCH_TARGET_DIR);
        let build_status = Command::new("cargo")
            .arg("build")
            .arg("--quiet")
            .arg("--locked")
            .arg("--profile")
            .arg("strict")
            .arg("--manifest-path")
            .arg(&manifest)
            .arg("--target-dir")
            .arg(&target_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        if !matches!(build_status, Ok(status) if status.success()) {
            self.line(
                PresentationRole::Result,
                None,
                Attention::Warning,
                Emphasis::Primary,
                "[EtuSchlopf] Watch nicht verfuegbar; musikalischer Lauf wird fortgesetzt.",
            );
            return;
        }

        let watch_exe = target_dir
            .join("strict")
            .join(format!("etuschlopf-watch{}", env::consts::EXE_SUFFIX));
        let mut command = Command::new(&watch_exe);
        command.current_dir(project_root);
        command.env(
            observation_protocol::WATCH_ACTIVE_SESSION_ENV,
            self.observation.context().session_id().as_str(),
        );
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            command
                .env("ETUSCHLOPF_WATCH_WEB_ONLY", "1")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            command.creation_flags(CREATE_NO_WINDOW);
        }

        match command.spawn() {
            Ok(mut child) => {
                std::thread::sleep(Duration::from_millis(120));
                if child.try_wait().ok().flatten().is_some() {
                    self.line(
                        PresentationRole::Result,
                        None,
                        Attention::Warning,
                        Emphasis::Primary,
                        "[EtuSchlopf] Watch wurde sofort beendet; musikalischer Lauf wird fortgesetzt.",
                    );
                } else {
                    self.watch_process = Some(child);
                    self.line(
                        PresentationRole::Check,
                        None,
                        Attention::Neutral,
                        Emphasis::Secondary,
                        "[EtuSchlopf] Watch aktiv.",
                    );
                }
            }
            Err(_) => self.line(
                PresentationRole::Result,
                None,
                Attention::Warning,
                Emphasis::Primary,
                "[EtuSchlopf] Watch konnte nicht gestartet werden; musikalischer Lauf wird fortgesetzt.",
            ),
        };
    }

    fn finish_watch_best_effort(&mut self) {
        let Some(mut child) = self.watch_process.take() else {
            return;
        };
        std::thread::sleep(Duration::from_millis(500));
        if child.try_wait().ok().flatten().is_none() {
            let _ = child.kill();
        }
        let _ = child.wait();
    }

    fn enter_observation_stage(&mut self, stage: observation::LauncherStage) {
        self.observation.enter_launcher_stage(stage);
        self.observation_transport.publish_current(&self.observation);
        write_take_attempt_stage(stage);
    }

    fn complete_observation(&mut self) {
        self.observation.mark_completed();
        self.observation_transport.publish_current(&self.observation);
        self.finish_watch_best_effort();
    }

    fn fail_observation(&mut self) {
        self.observation.mark_failed();
        self.observation_transport.publish_current(&self.observation);
        self.finish_watch_best_effort();
    }

    fn complete_observation_with_outcome(&mut self, outcome: OutcomeColor) {
        self.observation
            .mark_completed_with_outcome(observation_outcome_color(outcome));
        self.observation_transport.publish_current(&self.observation);
        self.finish_watch_best_effort();
    }

    fn fail_observation_with_outcome(&mut self, outcome: OutcomeColor) {
        self.observation
            .mark_failed_with_outcome(observation_outcome_color(outcome));
        self.observation_transport.publish_current(&self.observation);
        self.finish_watch_best_effort();
    }

    fn line(
        &mut self,
        role: PresentationRole,
        status: Option<LocalStatus>,
        attention: Attention,
        emphasis: Emphasis,
        message: &str,
    ) {
        self.console
            .write_stdout_line(presentation_line(role, status, attention, emphasis, message));
    }

    fn role_detail_line(
        &mut self,
        prefix: ConsolePrefix,
        role: PresentationRole,
        label: &str,
        detail: &str,
        detail_emphasis: Emphasis,
    ) {
        let prefix_text = match prefix {
            ConsolePrefix::None => None,
            ConsolePrefix::Product => Some("[EtuSchlopf]".to_string()),
            ConsolePrefix::Versioned => Some("[EtuSchlopf]".to_string()),
        };
        let mut segments = Vec::with_capacity(5);
        if let Some(prefix_text) = prefix_text.as_deref() {
            segments.push(ConsoleSegment::muted(prefix_text));
            segments.push(ConsoleSegment::plain(" "));
        }
        segments.push(ConsoleSegment::role(role, label));
        segments.push(ConsoleSegment::plain(": "));
        segments.push(match detail_emphasis {
            Emphasis::Muted => ConsoleSegment::muted(detail),
            Emphasis::Primary | Emphasis::Secondary => ConsoleSegment::plain(detail),
        });
        self.console.write_stdout_segments(&segments);
    }

    fn role_status_detail_line(
        &mut self,
        prefix: ConsolePrefix,
        role: PresentationRole,
        label: &str,
        status: LocalStatus,
        status_text: &str,
        detail: &str,
    ) {
        let prefix_text = match prefix {
            ConsolePrefix::None => None,
            ConsolePrefix::Product => Some("[EtuSchlopf]".to_string()),
            ConsolePrefix::Versioned => Some("[EtuSchlopf]".to_string()),
        };
        let mut segments = Vec::with_capacity(8);
        if let Some(prefix_text) = prefix_text.as_deref() {
            segments.push(ConsoleSegment::muted(prefix_text));
            segments.push(ConsoleSegment::plain(" "));
        }
        segments.push(ConsoleSegment::role(role, label));
        segments.push(ConsoleSegment::plain(" "));
        segments.push(ConsoleSegment::status(status, status_text));
        if !detail.is_empty() {
            segments.push(ConsoleSegment::plain(": "));
            segments.push(ConsoleSegment::muted(detail));
        }
        self.console.write_stdout_segments(&segments);
    }

    fn role_detail_status_line(
        &mut self,
        prefix: ConsolePrefix,
        role: PresentationRole,
        label: &str,
        detail: &str,
        status: LocalStatus,
        status_text: &str,
    ) {
        let prefix_text = match prefix {
            ConsolePrefix::None => None,
            ConsolePrefix::Product => Some("[EtuSchlopf]".to_string()),
            ConsolePrefix::Versioned => Some("[EtuSchlopf]".to_string()),
        };
        let mut segments = Vec::with_capacity(8);
        if let Some(prefix_text) = prefix_text.as_deref() {
            segments.push(ConsoleSegment::muted(prefix_text));
            segments.push(ConsoleSegment::plain(" "));
        }
        segments.push(ConsoleSegment::role(role, label));
        segments.push(ConsoleSegment::plain(": "));
        if !detail.is_empty() {
            segments.push(ConsoleSegment::plain(detail));
            segments.push(ConsoleSegment::plain(" "));
        }
        segments.push(ConsoleSegment::status(status, status_text));
        self.console.write_stdout_segments(&segments);
    }

    fn diagnostic_context_line(&mut self, message: &str) {
        self.console.write_stdout_line(PresentationLine {
            role: PresentationRole::Result,
            status: None,
            attention: Attention::Neutral,
            emphasis: Emphasis::Muted,
            visibility: Visibility::InteractiveOnly,
            channel: PresentationChannel::Diagnostic,
            text: message,
        });
    }

    fn progress_done(&mut self, completion: ProgressCompletion<'_>) -> io::Result<()> {
        self.console
            .write_progress_done(&self.version, completion)
    }

    /// Projects one live status record: a finished project becomes a completion line in the
    /// usual "<label> OK <m.m>min" form; stage changes are shown in line mode only, the
    /// interactive running line already carries them.
    fn live_status_event(
        &mut self,
        event: &console_live_status::LiveEvent,
        progress_mode: ProgressMode,
    ) -> io::Result<()> {
        match event {
            console_live_status::LiveEvent::Finished {
                project,
                outcome,
                elapsed,
            } => self.progress_done(ProgressCompletion {
                role: PresentationRole::Program,
                label: project,
                elapsed: *elapsed,
                detail: CompletionDetail::None,
                exit_code: match outcome {
                    etuschlopf::console_status_record::ProjectOutcome::Ok => 0,
                    etuschlopf::console_status_record::ProjectOutcome::Failed => 1,
                },
            }),
            console_live_status::LiveEvent::Stage { project, stage } => {
                if progress_mode == ProgressMode::Lines {
                    let message = format!(
                        "[EtuSchlopf] {project} → {}",
                        console_live_status::stage_short_label(*stage)
                    );
                    self.console
                        .write_stdout_line(progress_line_presentation(PresentationRole::Program, &message));
                }
                Ok(())
            }
        }
    }

    fn outcome_headline(
        &mut self,
        outcome: OutcomeColor,
        status: &str,
        project: &str,
        exit_code: i32,
    ) {
        let total = format_elapsed(self.started.elapsed());
        self.console
            .write_outcome_headline(&self.version, outcome, status, project, exit_code, &total);
    }

    fn stdout_progress_mode(&self) -> ProgressMode {
        self.console.stdout_progress_mode()
    }

    fn stdout_glyph_mode(&self) -> GlyphMode {
        self.console.stdout_glyph_mode()
    }
}

#[cfg(test)]
mod launcher_observation_tests {
    use super::{observation, Launcher};

    #[test]
    fn observation_stage_does_not_drive_console_phase() {
        use std::fs;
        use std::path::Path;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut launcher = Launcher::new("test".to_string());
        let progress_mode = launcher.stdout_progress_mode();
        let glyph_mode = launcher.stdout_glyph_mode();
        launcher.enter_observation_stage(observation::LauncherStage::Building);
        assert_eq!(launcher.stdout_progress_mode(), progress_mode);
        assert_eq!(launcher.stdout_glyph_mode(), glyph_mode);

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let evidence_dir = std::env::temp_dir().join(format!(
            "etuschlopf_take_attempt_writer_owner_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&evidence_dir).expect("create evidence dir");
        let owner_file = evidence_dir.join(super::TAKE_ATTEMPT_EVIDENCE_WRITER_OWNER_FILE);
        let non_owner_pid = std::process::id().wrapping_add(1);
        fs::write(
            &owner_file,
            format!("schema=1\nauthority=none\nwriter_pid={non_owner_pid}\n"),
        )
        .expect("write non-owner evidence pid");

        super::claim_take_attempt_evidence_writer_owner(&evidence_dir);
        assert_eq!(
            super::take_attempt_evidence_writer_owner_pid(&evidence_dir),
            Some(non_owner_pid),
            "a descendant launcher must not replace an existing attempt writer owner"
        );
        super::write_take_attempt_stage_to_dir(
            &evidence_dir,
            observation::LauncherStage::Building,
        );
        super::register_take_attempt_artifact_to_dir(
            &evidence_dir,
            "strict_test_log",
            Path::new("target/etuschlopf_logs/visual_preflight.strict-tests.txt"),
        );
        assert!(!evidence_dir.join("CHILD_STAGE.txt").exists());
        assert!(!evidence_dir.join("CHILD_ARTIFACT_REGISTRY.txt").exists());

        fs::write(
            &owner_file,
            format!(
                "schema=1\nauthority=none\nwriter_pid={}\n",
                std::process::id()
            ),
        )
        .expect("write current evidence owner pid");
        super::write_take_attempt_stage_to_dir(
            &evidence_dir,
            observation::LauncherStage::LauncherTests,
        );
        super::register_take_attempt_artifact_to_dir(
            &evidence_dir,
            "strict_test_log",
            Path::new("target/etuschlopf_logs/visual_preflight.strict-tests.txt"),
        );
        assert!(
            fs::read_to_string(evidence_dir.join("CHILD_STAGE.txt"))
                .expect("read owned stage")
                .contains("launcher_stage=launcher_tests")
        );
        assert!(
            fs::read_to_string(evidence_dir.join("CHILD_ARTIFACT_REGISTRY.txt"))
                .expect("read owned registry")
                .contains("role=strict_test_log")
        );
        fs::remove_dir_all(evidence_dir).expect("remove evidence dir");
    }

    #[test]
    fn launcher_children_share_session_and_keep_scope_parentage() {
        let launcher = Launcher::new("test".to_string());
        let corpus = launcher.observation.context().child_corpus();
        let project = launcher.observation.context().child_project(1);

        assert_eq!(
            corpus.session_id(),
            launcher.observation.context().session_id()
        );
        assert_eq!(corpus.source_id().as_str(), "corpus");
        assert_eq!(
            corpus.parent_source_id(),
            Some(launcher.observation.context().source_id())
        );
        assert_eq!(
            project.session_id(),
            launcher.observation.context().session_id()
        );
        assert_eq!(project.source_id().as_str(), "project:1");
        assert_eq!(
            project.parent_source_id(),
            Some(launcher.observation.context().source_id())
        );
    }
}

fn parse_args(raw_args: Vec<OsString>) -> ArgsState {
    let mut program_args = Vec::new();
    let mut input_arg = None;
    let mut force = false;
    // Strict is the public baseline. `--strict` remains an idempotent compatibility marker.
    let mut validation = ValidationPlan::Strict;
    let mut visual_preflight_gate = false;
    let mut export_policy = AssistantExportPolicy::Minimal;
    let mut watch = WatchPlan::Disabled;
    let mut neuro_corpus_publish = false;
    let mut inventory = InventoryPlan::Disabled;
    let mut index = 0usize;

    while index < raw_args.len() {
        let arg = &raw_args[index];
        let arg_text = arg.to_string_lossy();
        if arg_text.eq_ignore_ascii_case("--force") {
            force = true;
            index = index.saturating_add(1);
            continue;
        }
        if arg_text.eq_ignore_ascii_case("--strict") {
            validation = validation.with_strict();
            index = index.saturating_add(1);
            continue;
        }
        if arg_text.eq_ignore_ascii_case("--clippy-deep") {
            validation = ValidationPlan::DeepClippy;
            index = index.saturating_add(1);
            continue;
        }
        if is_visual_preflight_arg(&arg_text) {
            visual_preflight_gate = true;
            index = index.saturating_add(1);
            continue;
        }
        if is_assistant_export_pdf_arg(&arg_text) {
            export_policy = AssistantExportPolicy::IncludePdf;
            index = index.saturating_add(1);
            continue;
        }
        if is_watch_ui_arg(&arg_text) {
            watch = WatchPlan::Enabled;
            index = index.saturating_add(1);
            continue;
        }
        if arg_text.eq_ignore_ascii_case("--inventory") {
            inventory = InventoryPlan::Enabled;
            index = index.saturating_add(1);
            continue;
        }
        if arg_text.eq_ignore_ascii_case("--project") {
            if let Some(value) = raw_args.get(index.saturating_add(1)) {
                let project = value.to_string_lossy();
                let looks_like_option = project.starts_with('-') || project.starts_with('/');
                if !project.trim().is_empty() && !looks_like_option {
                    input_arg = Some(project.trim().to_string());
                    program_args.push(value.clone());
                    index = index.saturating_add(2);
                    continue;
                }
            }
            program_args.push(arg.clone());
            index = index.saturating_add(1);
            continue;
        }
        if let Some(project) = strip_option_value_ignore_ascii_case(&arg_text, "--project=") {
            if !project.trim().is_empty() {
                input_arg = Some(project.trim().to_string());
                program_args.push(OsString::from(project.trim()));
            } else {
                program_args.push(arg.clone());
            }
            index = index.saturating_add(1);
            continue;
        }
        if is_all_arg(&arg_text) {
            neuro_corpus_publish = true;
            input_arg = Some("--all".to_string());
            program_args.push(OsString::from("--all"));
            index = index.saturating_add(1);
            continue;
        }

        let option_takes_separate_value = program_option_takes_separate_value(&arg_text);
        let is_option = arg_text.starts_with('-') || arg_text.starts_with('/');
        if input_arg.is_none() && !is_option {
            input_arg = Some(arg_text.to_string());
        }
        program_args.push(arg.clone());
        if option_takes_separate_value && index.saturating_add(1) < raw_args.len() {
            program_args.push(raw_args[index + 1].clone());
            index = index.saturating_add(2);
        } else {
            index = index.saturating_add(1);
        }
    }

    if validation == ValidationPlan::DeepClippy {
        return ArgsState {
            request: RunRequest::DeepClippy {
                build_preference: if force {
                    BuildPreference::ForceCompile
                } else {
                    BuildPreference::ReuseVerified
                },
            },
        };
    }

    let target = match input_arg {
        Some(value) if is_all_arg(&value) => RunTarget::Corpus,
        Some(value) => RunTarget::Project(value),
        None if visual_preflight_gate => RunTarget::VisualPreflight,
        None => RunTarget::Missing,
    };
    ArgsState {
        request: RunRequest::Execute(ExecutionRequest {
            target,
            program_args,
            validation,
            build_preference: if force {
                BuildPreference::ForceCompile
            } else {
                BuildPreference::ReuseVerified
            },
            launcher_tests: if validation == ValidationPlan::Strict {
                LauncherTestPlan::VisualPreflight
            } else {
                LauncherTestPlan::Standard
            },
            export_policy,
            watch,
            neuro_corpus_publish,
            inventory,
        }),
    }
}


fn is_watch_ui_arg(value: &str) -> bool {
    ["--watch", "--show", "--display"]
        .iter()
        .any(|candidate| value.eq_ignore_ascii_case(candidate))
}

fn strip_option_value_ignore_ascii_case<'a>(value: &'a str, prefix: &str) -> Option<&'a str> {
    let head = value.get(..prefix.len())?;
    if !head.eq_ignore_ascii_case(prefix) {
        return None;
    }
    value.get(prefix.len()..)
}

fn program_option_takes_separate_value(value: &str) -> bool {
    [
        "--corpus-out",
        "--out",
        "--musicxml-out",
        "--pdf-out",
        "--reference-wav",
        "--layout-pages",
        "--top",
        "--jobs",
    ]
    .iter()
    .any(|candidate| value.eq_ignore_ascii_case(candidate))
}

fn read_cargo_version(path: &Path) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("version") {
            let rest = rest.trim_start();
            if let Some(value) = rest.strip_prefix('=') {
                return Some(value.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}

include!("etuschlopf_launcher/launcher_hygiene.rs");

fn init_console_log(version: &str, thread_plan: &ThreadPlan) -> AppResult<()> {
    if let Some(parent) = Path::new(CONSOLE_LOG).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(CONSOLE_LOG)?;
    writeln!(file, "ETUSCHLOPF-LOG")?;
    writeln!(file, "Version: {version}")?;
    writeln!(file, "Threads: {}", thread_plan.summary_label())?;
    writeln!(file, "Policy: compact_global")?;
    writeln!(file, "{}", startup_timing::readback())?;
    if let (Ok(from), Ok(to)) = (env::var("ETUSCHLOPF_TAKE_FROM"), env::var("ETUSCHLOPF_TAKE_TO")) {
        writeln!(file, "Update: {from}->{to}")?;
    }
    writeln!(file)?;
    Ok(())
}


const DEEP_CLIPPY_LOG: &str = "etuschlopf.deep-clippy.txt";

fn kodex_gate_core_source_root(root: &Path) -> PathBuf {
    root.join("tools/RUST_KODEX_GATE_CORE/src")
}

include!("etuschlopf_launcher/launcher_build_control.rs");
include!("etuschlopf_launcher/launcher_timing.rs");

fn execute_normal_run(
    launcher: &mut Launcher,
    request: &ExecutionRequest,
    plan: LauncherPlan,
    project_root: &Path,
    public_transaction_enabled: bool,
) -> AppResult<()> {
    debug_assert_ne!(plan.validation, ValidationPlan::DeepClippy);
    let mut state = initialize_run_state(launcher, request, plan);
    announce_integrated_visual_preflight_gate(launcher, request);
    launcher.enter_observation_stage(observation::LauncherStage::Building);
    let test_prewarm = StrictTestPrewarm::start(launcher, &state, request.launcher_tests);
    materialize_launcher_build(launcher, &mut state, request, plan.build)?;
    let prewarm_wait_timing = LauncherPhaseTimer::new("TestPrewarmResidualWait");
    StrictTestPrewarm::finish(test_prewarm, launcher);
    drop(prewarm_wait_timing);
    run_integrated_launcher_tests(launcher, &mut state, request.launcher_tests)?;

    match &request.target {
        RunTarget::Missing => reject_missing_run_target(launcher),
        RunTarget::VisualPreflight => execute_visual_preflight_target(launcher, &mut state),
        RunTarget::Corpus => execute_corpus_target(launcher, &mut state, project_root, request),
        RunTarget::Project(project) => execute_project_target(
            launcher,
            &mut state,
            project_root,
            request,
            project,
            public_transaction_enabled,
        ),
    }
}

fn initialize_run_state(
    launcher: &Launcher,
    request: &ExecutionRequest,
    plan: LauncherPlan,
) -> RunState {
    let selected_build_profile = plan.build.profile();
    RunState {
        validation: plan.validation,
        build_outcome: BuildOutcome::Pending(selected_build_profile),
        thread_plan: launcher.thread_plan.clone(),
        target_exe: selected_build_profile.target_exe(),
        strict_clippy_exit: "not_run".to_string(),
        build_exit: "not_run".to_string(),
        strict_test_exit: "not_run".to_string(),
        run_exit: "not_run".to_string(),
        clippy_ms: "not_run".to_string(),
        build_ms: "not_measured".to_string(),
        strict_test_ms: "not_run".to_string(),
        strict_test_summary: "not_run".to_string(),
        program_ms: "not_measured".to_string(),
        total_start: Instant::now(),
        strict_log: None,
        strict_test_log: None,
        result_file: None,
        input_arg: request.target.input_arg().map(str::to_string),
        force_mode: request.build_preference.forces_compile(),
        audit_signal: AuditSignal::NotEvaluated,
        renderer_outcome: RendererOutcome::NotObserved,
        emergence_outcome: EmergenceOutcome::NotApplicable,
        render_candidate_selected_pdf_truth: None,
        export_policy: request.export_policy,
    }
}

fn announce_integrated_visual_preflight_gate(
    _launcher: &mut Launcher,
    request: &ExecutionRequest,
) {
    if request.launcher_tests.visual_preflight_requested()
        && !matches!(&request.target, RunTarget::VisualPreflight)
    {
        // Technical log only: the same notice on every run carries no news on screen.
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(CONSOLE_LOG)
            .and_then(|mut log| writeln!(log, "Strict: Visual-Preflight ist als Launcher-Testgate integriert."));
    }
}

fn materialize_launcher_build(
    launcher: &mut Launcher,
    state: &mut RunState,
    request: &ExecutionRequest,
    build: BuildPlan,
) -> AppResult<()> {
    let _timing = LauncherPhaseTimer::new("BuildControl");
    let strict_log_path = if matches!(&request.target, RunTarget::VisualPreflight) {
        PathBuf::from("visual_preflight.strict-clippy.txt")
    } else {
        strict_log_name(request.target.input_arg())
    };
    match build {
        BuildPlan::Reuse(profile) => reuse_verified_build(launcher, state, profile, &strict_log_path),
        BuildPlan::Compile { profile, reason } => {
            compile_planned_build(launcher, state, profile, reason, &strict_log_path)
        }
    }
}

fn run_integrated_launcher_tests(
    launcher: &mut Launcher,
    state: &mut RunState,
    launcher_tests: LauncherTestPlan,
) -> AppResult<()> {
    let _timing = LauncherPhaseTimer::new("IntegratedTestsControl");
    if !should_run_integrated_launcher_tests(state.validation, launcher_tests) {
        return Ok(());
    }

    launcher.enter_observation_stage(observation::LauncherStage::LauncherTests);
    let strict_test_log = PathBuf::from(VISUAL_PREFLIGHT_STRICT_TEST_LOG);
    register_take_attempt_artifact("strict_test_log", &strict_test_log);
    if strict_test_validation_receipt_matches(&launcher.version, &state.target_exe) {
        write_reused_strict_test_log(&strict_test_log, &launcher.version, &state.target_exe)?;
        state.strict_test_log = Some(strict_test_log.clone());
        state.strict_test_exit = "0".to_string();
        state.strict_test_ms = "0".to_string();
        state.strict_test_summary =
            "reused exact source/toolchain/binary-bound all-targets validation receipt".to_string();
        launcher.role_status_detail_line(
            ConsolePrefix::Versioned,
            PresentationRole::Test,
            "Tests",
            LocalStatus::Ok,
            "REUSE",
            "exaktes Strict-Test-Receipt",
        );
        return Ok(());
    }
    let receipt_path = Path::new(STRICT_TEST_VALIDATION_RECEIPT);
    write_strict_test_validation_receipt(receipt_path, None)?;
    let receipt_before =
        strict_test_validation_receipt_material(&launcher.version, &state.target_exe)?;
    prepare_current_run_test_log(&strict_test_log)?;
    state.strict_test_log = Some(strict_test_log.clone());
    let test_args = strict_test_cargo_args(launcher.thread_plan.cargo_jobs);
    let os_args: Vec<OsString> = test_args.iter().map(OsString::from).collect();
    let test_result = invoke_logged_program_with_env(
        launcher,
        LoggedProcessSpec {
            exe: Path::new("cargo"),
            args: &os_args,
            log_path: &strict_test_log,
            header: "CARGO-TEST-ETUSCHLOPF-ALL-TARGETS",
            progress_role: PresentationRole::Test,
            heartbeat: "Tests",
            completion_summary: CompletionSummary::CargoTestCounts,
        },
        ProcessEnvMode::NormalCargoTarget,
    )?;
    state.strict_test_exit = test_result.exit_code.to_string();
    state.strict_test_ms = test_result.elapsed_ms.to_string();
    state.strict_test_summary = cargo_test_summary_from_log(&strict_test_log);
    if test_result.exit_code != 0 {
        launcher.line(
            PresentationRole::Result,
            Some(LocalStatus::Failed),
            Attention::Error,
            Emphasis::Primary,
            &format!(
                "[EtuSchlopf] Tests ROT: {}",
                strict_test_log.display()
            ),
        );
        print_log_tail(launcher, &strict_test_log, 60)?;
        launcher.fail_observation();
        return launcher_exit(test_result.exit_code);
    }
    let receipt_after =
        strict_test_validation_receipt_material(&launcher.version, &state.target_exe)?;
    write_strict_test_validation_receipt(
        receipt_path,
        Some((&receipt_before, &receipt_after)),
    )?;
    Ok(())
}

fn strict_test_cargo_args(cargo_jobs: usize) -> Vec<String> {
    vec![
        "--config".to_string(),
        "profile.strict.package.etuschlopf.opt-level=0".to_string(),
        "test".to_string(),
        "--locked".to_string(),
        "--profile".to_string(),
        "strict".to_string(),
        "--all-targets".to_string(),
        "--no-fail-fast".to_string(),
        "--jobs".to_string(),
        cargo_jobs.to_string(),
        "--target-dir".to_string(),
        "target/bc".to_string(),
    ]
}

const STRICT_TEST_PREWARM_LOG: &str = "target/etuschlopf_logs/strict_test_prewarm.log";

/// The prewarm runs beside Clippy and the productive build. With the full job
/// count both Cargo invocations started up to twice as many compilers as the
/// machine has threads, the peak memory phase of a take. Half the jobs keep the
/// warm-up useful while the combined compiler count stays near the thread count.
/// The later authoritative test step keeps the full job count.
fn test_prewarm_cargo_jobs(cargo_jobs: usize) -> usize {
    (cargo_jobs / 2).max(1)
}

/// Compiles the strict all-target test binaries (`--no-run`) while Clippy and
/// the productive build run. It only warms the separate `target/bc` Cargo
/// cache: the unchanged test step afterwards still runs the full test command,
/// decides alone and writes the only receipt. A failed or killed prewarm
/// therefore costs time, never validation.
struct StrictTestPrewarm {
    child: Option<Child>,
    started: Instant,
    jobs: usize,
}

/// Key of the measured residual wait for the test prewarm in the last-durations file.
const TEST_PREWARM_WAIT_KEY: &str = "TEST-PREWARM-WAIT";

impl StrictTestPrewarm {
    fn start(
        launcher: &Launcher,
        state: &RunState,
        launcher_tests: LauncherTestPlan,
    ) -> Option<Self> {
        if !should_run_integrated_launcher_tests(state.validation, launcher_tests)
            || strict_test_validation_receipt_matches(&launcher.version, &state.target_exe)
        {
            return None;
        }
        let _ = fs::create_dir_all("target/etuschlopf_logs");
        let log = File::create(STRICT_TEST_PREWARM_LOG).ok()?;
        let log_err = log.try_clone().ok()?;
        let jobs = test_prewarm_cargo_jobs(launcher.thread_plan.cargo_jobs);
        let mut args = strict_test_cargo_args(jobs);
        args.push("--no-run".to_string());
        let started = Instant::now();
        let child = Command::new("cargo")
            .args(&args)
            .env_remove("CARGO_TARGET_DIR")
            .env("CARGO_PROFILE_STRICT_BUILD_OVERRIDE_OPT_LEVEL", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::from(log))
            .stderr(Stdio::from(log_err))
            .spawn()
            .ok()?;
        Some(Self {
            child: Some(child),
            started,
            jobs,
        })
    }

    fn finish(prewarm: Option<Self>, launcher: &mut Launcher) {
        let Some(mut prewarm) = prewarm else {
            return;
        };
        let Some(mut child) = prewarm.child.take() else {
            return;
        };
        let join_started = Instant::now();
        let durations_path = Path::new(console_live_status::LAST_PHASE_DURATIONS_FILE);
        let previous_wait = console_live_status::read_last_phase_durations(durations_path)
            .get(TEST_PREWARM_WAIT_KEY)
            .copied()
            .map(Duration::from_millis);
        let progress_mode = launcher.stdout_progress_mode();
        let glyph_mode = launcher.stdout_glyph_mode();
        let mut animation = ProgressAnimation::new();
        // The residual wait is shown live; the prewarm lifetime itself is only an upper
        // bound (observed at join), so no duration is claimed for it on screen.
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Ok(status),
                Ok(None) => {}
                Err(error) => break Err(error),
            }
            if progress_mode == ProgressMode::Inline {
                let running = animation.running_line(
                    "Test-Vorbau wartet",
                    join_started.elapsed(),
                    previous_wait,
                    None,
                    glyph_mode,
                );
                let text = running.plain();
                let _ = launcher
                    .console
                    .write_stdout_inline(progress_line_presentation(PresentationRole::Test, &text));
            }
            thread::sleep(Duration::from_millis(ANIMATION_INTERVAL_MS));
            animation.advance();
        };
        let status_ok = status.as_ref().is_ok_and(std::process::ExitStatus::success);
        let status = status.map_or_else(|error| error.to_string(), |status| status.to_string());
        append_launcher_timing_line(&format!(
            "LauncherPrewarmTiming: LifetimeObservedUpperUs={}; ResidualWaitUs={}; CompletionPrecision=not_observed_until_join; Overlap=Clippy_and_Build; AddLifetimeToSerialPhases=no; Pid={}",
            prewarm.started.elapsed().as_micros(), join_started.elapsed().as_micros(), std::process::id(),
        ));
        append_launcher_timing_line(&format!(
            "LauncherPrewarmResult: Status={status}; Jobs={}; CargoJobs={}; Log={STRICT_TEST_PREWARM_LOG}",
            prewarm.jobs,
            launcher.thread_plan.cargo_jobs
        ));
        if status_ok {
            console_live_status::record_last_phase_duration(
                durations_path,
                TEST_PREWARM_WAIT_KEY,
                join_started.elapsed(),
            );
            let waited = format!("gewartet {}", format_elapsed(join_started.elapsed()));
            launcher.role_status_detail_line(
                ConsolePrefix::Versioned,
                PresentationRole::Test,
                "Test-Vorbau",
                LocalStatus::Ok,
                "bereit",
                &waited,
            );
        } else {
            launcher.role_detail_line(
                ConsolePrefix::Versioned,
                PresentationRole::Test,
                "Test-Vorbau",
                &format!("beendet: {status} · Tests laufen trotzdem vollständig"),
                Emphasis::Muted,
            );
        }
    }
}

impl Drop for StrictTestPrewarm {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn reject_missing_run_target(launcher: &mut Launcher) -> AppResult<()> {
    launcher.fail_observation();
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        "",
    );
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        "[EtuSchlopf] Aufruf fehlt.",
    );
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        r#"Einzelprojekt: .\run.cmd --project "Etuede Nr. 6"  (Strict + Clippy sind automatisch)"#,
    );
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        r#"Mit UI:        .\run.cmd --watch "Etuede Nr. 6"  (Strict + Clippy automatisch; Aliase: --show, --display)"#,
    );
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        r"Korpus:        .\run.cmd --all  (Strict + Clippy sind automatisch)",
    );
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        r#"Inventur:      --inventory an Einzelprojekt oder --all anhaengen (ESI-3/ARCH-4/CMS-1 + XLSX-3)"#,
    );
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        "",
    );
    launcher_exit(1)
}

fn execute_visual_preflight_target(
    launcher: &mut Launcher,
    state: &mut RunState,
) -> AppResult<()> {
    launcher.enter_observation_stage(observation::LauncherStage::RunningTarget);
    let visual_exit_code = visual_preflight::execute_visual_preflight_run(launcher, state)?;
    launcher.enter_observation_stage(observation::LauncherStage::Finalizing);
    let outcome = print_summary(launcher, state, "visual_preflight", visual_exit_code)?;
    if visual_exit_code == 0 {
        launcher.complete_observation_with_outcome(outcome);
    } else {
        launcher.fail_observation_with_outcome(outcome);
    }
    if visual_exit_code == 0 {
        Ok(())
    } else {
        launcher_exit(visual_exit_code)
    }
}

fn inventory_summary_text(inventory: &InventoryArtifacts) -> String {
    format!("{} · {} Zeilen · A {} / B {} / C {} / D {} / OFFEN {}",
        inventory.scope_label, inventory.row_count, inventory.class_counts.a,
        inventory.class_counts.b, inventory.class_counts.c, inventory.class_counts.d,
        inventory.class_counts.open)
}

// Preserve the existing corpus invocation, forwarding only the optional trace.
// Other runtime argument behavior is outside this diagnostic correction.
fn corpus_program_args(request: &ExecutionRequest) -> Vec<OsString> {
    let mut args = vec![OsString::from("--all")];
    args.extend(request.program_args.iter().filter(|arg| {
        arg.to_str().is_some_and(|text| text == "--writer-trace" || text.starts_with("--writer-trace="))
    }).cloned());
    args
}

fn execute_corpus_target(
    launcher: &mut Launcher,
    state: &mut RunState,
    project_root: &Path,
    request: &ExecutionRequest,
) -> AppResult<()> {
    state.result_file = Some(PathBuf::from("corpus_emergence.result.txt"));
    let program_args = corpus_program_args(request);
    launcher.enter_observation_stage(observation::LauncherStage::RunningTarget);
    let child_observation = launcher.observation.context().child_corpus();
    let program_result = invoke_logged_observed_target(
        launcher,
        LoggedProcessSpec {
            exe: &state.target_exe,
            args: &program_args,
            log_path: Path::new(CONSOLE_LOG),
            header: "PROGRAM-RUN-ALL",
            progress_role: PresentationRole::Program,
            heartbeat: "Etüden",
            completion_summary: CompletionSummary::LogSize,
        },
        child_observation,
    )?;
    launcher.enter_observation_stage(observation::LauncherStage::Finalizing);
    state.run_exit = program_result.exit_code.to_string();
    state.program_ms = program_result.elapsed_ms.to_string();
    let finalization_timing = LauncherPhaseTimer::new("PostProgramFinalizationBeforeExport");
    let root_cleanup = cleanup_project_root_residue(project_root, state.input_arg.as_deref(), request.inventory.requested());
    append_root_cleanup_to_console_log("NachRun", &root_cleanup)?;
    bind_corpus_run_renderer_outcome(project_root, &launcher.version, state)?;
    append_run_diagnostic(state, "all_run")?;
    refresh_corpus_run_manifest_report_binding(
        project_root,
        &launcher.version,
        program_result.exit_code == 0,
    )?;
    if request.neuro_corpus_publish {
        if program_result.exit_code != 0 {
            return Err(anyhow!(
                "Neuro-Evidence-Corpus: Korpuslauf fehlgeschlagen; keine Kohorte publiziert"
            ));
        }
        let commit_path =
            neuro_evidence_export::publish_neuro_evidence_for_successful_corpus_run(
                project_root,
                &launcher.version,
            )
            .map_err(|error| anyhow!("Neuro-Evidence-Corpus: CORPUS RUN FAILED: {error}"))?;
        launcher.role_detail_line(
            ConsolePrefix::None,
            PresentationRole::Export,
            "Neuro-Corpus",
            &commit_path.display().to_string(),
            Emphasis::Muted,
        );
    }
    if program_result.exit_code == 0 && launcher.version == CORPUS_ACCEPTANCE_CONTRACT_VERSION {
        write_version_bound_corpus_acceptance_stamp(project_root, &launcher.version)?;
    }
    if request.inventory.requested() {
        let inventory = generate_corpus_inventory(
            project_root,
            &launcher.version,
            launcher.observation.context().session_id().as_str(),
            &state.target_exe,
            program_result.exit_code,
        )?;
        launcher.role_detail_line(
            ConsolePrefix::None,
            PresentationRole::Export,
            "Inventory",
            &inventory_summary_text(&inventory),
            Emphasis::Muted,
        );
    }
    let outcome = print_summary(launcher, state, "all_run", program_result.exit_code)?;
    drop(finalization_timing);
    let export_path = create_assistant_export(AssistantExportRequest::Corpus {
        project_root,
        state,
        export_version: &launcher.version,
        evidence_version: &launcher.version,
        final_exit: program_result.exit_code,
    })?;
    launcher.role_detail_line(
        ConsolePrefix::None,
        PresentationRole::Export,
        "Export",
        &export_path.display().to_string(),
        Emphasis::Muted,
    );
    if program_result.exit_code == 0 {
        launcher.complete_observation_with_outcome(outcome);
    } else {
        launcher.fail_observation_with_outcome(outcome);
    }
    if program_result.exit_code == 0 {
        Ok(())
    } else {
        launcher_exit(program_result.exit_code)
    }
}

fn execute_project_target(
    launcher: &mut Launcher,
    state: &mut RunState,
    project_root: &Path,
    request: &ExecutionRequest,
    project: &str,
    public_transaction_enabled: bool,
) -> AppResult<()> {
    state.result_file = Some(resolve_result_file(project));
    let public_transaction = begin_project_public_transaction(
        launcher,
        project_root,
        project,
        public_transaction_enabled,
    )?;
    launcher.enter_observation_stage(observation::LauncherStage::RunningTarget);
    let child_observation = launcher.observation.context().child_project(1);
    let program_result = match invoke_logged_observed_target(
        launcher,
        LoggedProcessSpec {
            exe: &state.target_exe,
            args: &request.program_args,
            log_path: Path::new(CONSOLE_LOG),
            header: "PROGRAM-RUN-NORMAL",
            progress_role: PresentationRole::Program,
            heartbeat: "Etüden",
            completion_summary: CompletionSummary::LogSize,
        },
        child_observation,
    ) {
        Ok(result) => result,
        Err(run_error) => {
            restore_project_public_transaction_after_start_failure(
                public_transaction.as_ref(),
                &run_error,
            )?;
            return Err(run_error);
        }
    };
    launcher.enter_observation_stage(observation::LauncherStage::Finalizing);
    complete_project_public_transaction(
        launcher,
        public_transaction.as_ref(),
        program_result.exit_code,
    )?;
    state.run_exit = program_result.exit_code.to_string();
    state.program_ms = program_result.elapsed_ms.to_string();
    let finalization_timing = LauncherPhaseTimer::new("PostProgramFinalizationBeforeExport");
    update_normal_run_result_evidence(state, program_result.exit_code);
    let root_cleanup = cleanup_project_root_residue(project_root, state.input_arg.as_deref(), request.inventory.requested());
    append_root_cleanup_to_console_log("NachRun", &root_cleanup)?;
    append_run_diagnostic(state, "normal_run")?;
    if request.inventory.requested() {
        let inventory = generate_project_inventory(
            project_root,
            &launcher.version,
            launcher.observation.context().session_id().as_str(),
            project,
            &state.target_exe,
            program_result.exit_code,
        )?;
        launcher.role_detail_line(
            ConsolePrefix::None,
            PresentationRole::Export,
            "Inventory",
            &inventory_summary_text(&inventory),
            Emphasis::Muted,
        );
    }
    drop(finalization_timing);
    match create_assistant_export(AssistantExportRequest::Normal {
        project_root,
        project,
        state,
        version: &launcher.version,
        run_id: launcher.observation.context().session_id().as_str(),
        final_exit: program_result.exit_code,
    }) {
        Ok(path) => launcher.role_detail_line(
            ConsolePrefix::None,
            PresentationRole::Export,
            "Export",
            &path.display().to_string(),
            Emphasis::Muted,
        ),
        Err(error) if request.inventory.requested() => {
            return Err(anyhow!(
                "[EtuSchlopf assistant-export] --inventory verlangt gebundenen Assistant-Export: {error}"
            ));
        }
        Err(error) => launcher.line(
            PresentationRole::Export,
            Some(LocalStatus::Failed),
            Attention::Error,
            Emphasis::Primary,
            &format!(
                "[EtuSchlopf assistant-export] Einzelprojekt-Export fehlgeschlagen: {error}"
            ),
        ),
    }
    if let Some(result_path) = state.result_file.as_deref() {
        match neuro_evidence_export::publish_neuro_evidence_for_successful_normal_run(
            project_root,
            result_path,
            project,
            &launcher.version,
            launcher.observation.context().session_id().as_str(),
            program_result.exit_code,
        ) {
            Ok(Some(path)) => launcher.role_detail_line(
                ConsolePrefix::None,
                PresentationRole::Export,
                "Neuro-Evidence",
                &path.display().to_string(),
                Emphasis::Muted,
            ),
            Ok(None) => {}
            Err(error) => launcher.line(
                PresentationRole::Export,
                Some(LocalStatus::Blocked),
                Attention::Warning,
                Emphasis::Primary,
                &format!(
                    "[EtuSchlopf neuro-evidence] Export blockiert; EtuSchlopf bleibt fail-independent: {error}"
                ),
            ),
        }
    } else {
        launcher.line(
            PresentationRole::Export,
            Some(LocalStatus::Blocked),
            Attention::Warning,
            Emphasis::Primary,
            concat!(
                "[EtuSchlopf neuro-evidence] Export blockiert: Ergebnisberichtspfad fehlt; ",
                "EtuSchlopf bleibt fail-independent."
            ),
        );
    }
    let outcome = print_summary(launcher, state, "normal_run", program_result.exit_code)?;
    if program_result.exit_code == 0 {
        launcher.complete_observation_with_outcome(outcome);
    } else {
        launcher.fail_observation_with_outcome(outcome);
    }
    if program_result.exit_code == 0 {
        Ok(())
    } else {
        launcher_exit(program_result.exit_code)
    }
}

fn begin_project_public_transaction(
    launcher: &mut Launcher,
    project_root: &Path,
    project: &str,
    enabled: bool,
) -> AppResult<Option<public_artifact_recovery::PublicArtifactTransaction>> {
    if !enabled {
        return Ok(None);
    }
    let transaction = public_artifact_recovery::begin_public_artifact_transaction(
        project_root,
        project,
        &launcher.version,
    )
    .map_err(|error| anyhow!(error))?;
    launcher.role_detail_line(
        ConsolePrefix::None,
        PresentationRole::Artifact,
        "Artefakte",
        "Sicherung aktiv.",
        Emphasis::Secondary,
    );
    Ok(Some(transaction))
}

fn restore_project_public_transaction_after_start_failure(
    transaction: Option<&public_artifact_recovery::PublicArtifactTransaction>,
    run_error: &anyhow::Error,
) -> AppResult<()> {
    if let Some(transaction) = transaction {
        public_artifact_recovery::restore_public_artifact_transaction(transaction).map_err(
            |restore_error| {
                anyhow!(
                    "Kindprozess konnte nicht gestartet/überwacht werden: {run_error}; anschließender Restore ebenfalls fehlgeschlagen: {restore_error}"
                )
            },
        )?;
    }
    Ok(())
}

fn complete_project_public_transaction(
    launcher: &mut Launcher,
    transaction: Option<&public_artifact_recovery::PublicArtifactTransaction>,
    exit_code: i32,
) -> AppResult<()> {
    let Some(transaction) = transaction else {
        return Ok(());
    };
    if exit_code == 0 {
        if let Err(commit_error) =
            public_artifact_recovery::commit_public_artifact_transaction(transaction, &launcher.version)
        {
            public_artifact_recovery::restore_public_artifact_transaction(transaction).map_err(
                |restore_error| {
                    anyhow!(
                        "Public-Artifact-Commit fehlgeschlagen: {commit_error}; anschließender Restore ebenfalls fehlgeschlagen: {restore_error}"
                    )
                },
            )?;
            return Err(anyhow!(
                "Public-Artifact-Kandidatenvalidierung/Commit fehlgeschlagen; alter Vierersatz wurde wiederhergestellt: {commit_error}"
            ));
        }
        launcher.role_detail_status_line(
            ConsolePrefix::None,
            PresentationRole::Artifact,
            "Artefakte",
            "Commit",
            LocalStatus::Committed,
            "OK",
        );
        return Ok(());
    }

    public_artifact_recovery::restore_public_artifact_transaction(transaction).map_err(
        |restore_error| {
            anyhow!(
                "Kindprozess Exit {exit_code}; alter öffentlicher Vierersatz konnte nicht vollständig wiederhergestellt werden: {restore_error}"
            )
        },
    )?;
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        &format!(
            "[EtuSchlopf public-artifact-transaction] Kindprozess Exit {exit_code}; alter Vierersatz vollständig wiederhergestellt."
        ),
    );
    Ok(())
}

fn bind_corpus_run_renderer_outcome(
    project_root: &Path,
    expected_version: &str,
    state: &mut RunState,
) -> AppResult<()> {
    let manifest_path = project_root.join(CORPUS_RUN_MANIFEST_FILE);
    if !manifest_path.exists() {
        return Ok(());
    }
    let manifest = read_corpus_run_manifest(&manifest_path)
        .map_err(|reason| anyhow!("Korpus-Laufmanifest fuer RendererOutcome nicht lesbar: {reason}"))?;
    if manifest.version != expected_version {
        return Err(anyhow!(
            "Korpus-Laufmanifest-Version {} passt nicht zu Launcher-Version {}",
            manifest.version,
            expected_version
        ));
    }
    if manifest.renderer_outcome_observed {
        state.renderer_outcome = state.renderer_outcome.merge(RendererOutcome::Passed);
    }
    Ok(())
}

fn refresh_corpus_run_manifest_report_binding(
    project_root: &Path,
    expected_version: &str,
    require_valid_manifest: bool,
) -> AppResult<()> {
    let manifest_path = project_root.join(CORPUS_RUN_MANIFEST_FILE);
    if !manifest_path.exists() {
        return Ok(());
    }
    let mut manifest = read_corpus_run_manifest(&manifest_path)
        .map_err(|reason| anyhow!("Korpus-Laufmanifest nach --all nicht lesbar: {reason}"))?;
    if manifest.version != expected_version {
        return Err(anyhow!(
            "Korpus-Laufmanifest-Version {} passt nicht zu Launcher-Version {}",
            manifest.version,
            expected_version
        ));
    }
    let report_path = project_root.join("corpus_emergence.result.txt");
    if !report_path.is_file() {
        if require_valid_manifest {
            return Err(anyhow!(
                "Korpusbericht fehlt nach erfolgreichem --all: {}",
                report_path.display()
            ));
        }
        return Ok(());
    }
    let report_evidence = corpus_run_file_evidence(&report_path)
        .map_err(|error| anyhow!("Korpusbericht kann nach Launcher-Diagnose nicht gebunden werden: {error}"))?;
    let completed_unix_seconds = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(manifest.started_unix_seconds);
    manifest.finalize(completed_unix_seconds, report_evidence);
    manifest.write_atomic(&manifest_path)?;
    let validation = validate_corpus_run_manifest(project_root, expected_version);
    if require_valid_manifest && !validation.valid {
        return Err(anyhow!(
            "Korpus-Laufmanifest-Endbindung fehlgeschlagen: {}",
            validation.reason
        ));
    }
    Ok(())
}

struct VisualProjectRunLedger {
    project: String,
    exit_code: i32,
    result_file: PathBuf,
    result_exists: bool,
    result_status: String,
    result_version: String,
    fresh: bool,
    verdict: String,
    shadow_run_started_at: Option<SystemTime>,
    previous_pdf_fingerprint: Option<PdfVisualFingerprintSnapshot>,
    current_pdf_fingerprint: Option<PdfVisualFingerprintSnapshot>,
    previous_pdf_visual_fingerprint: Option<PdfAutoVisualFingerprintSnapshot>,
    current_pdf_visual_fingerprint: Option<PdfAutoVisualFingerprintSnapshot>,
}

#[derive(Clone)]
struct VisualPreflightPlan {
    projects: Vec<String>,
    target_project: Option<String>,
    target_locator: Option<String>,
    target_score: Option<usize>,
    target_class: EmergenceTargetClass,
    selector_source: String,
    selector_status: String,
    reference_project: String,
    emergence: CorpusEmergenceSelectionState,
}

include!("etuschlopf_launcher/launcher_visual_audit.rs");

fn is_all_arg(arg: &str) -> bool {
    arg.eq_ignore_ascii_case("--all") || arg.eq_ignore_ascii_case("-all") || arg.eq_ignore_ascii_case("/all")
}

fn is_visual_preflight_arg(arg: &str) -> bool {
    arg.eq_ignore_ascii_case("--visual-preflight")
        || arg.eq_ignore_ascii_case("-visual-preflight")
        || arg.eq_ignore_ascii_case("/visual-preflight")
        || arg.eq_ignore_ascii_case("--quick-visual")
        || arg.eq_ignore_ascii_case("--fast-visual")
}

fn is_assistant_export_pdf_arg(arg: &str) -> bool {
    arg.eq_ignore_ascii_case("--assistant-export-pdf")
        || arg.eq_ignore_ascii_case("--export-pdf")
        || arg.eq_ignore_ascii_case("--with-pdf")
}


fn is_public_artifact_transaction_eligible(args: &ArgsState) -> bool {
    let RunRequest::Execute(request) = &args.request else {
        return false;
    };
    let RunTarget::Project(project_name) = &request.target else {
        return false;
    };
    request.validation == ValidationPlan::Strict
        && request.build_preference == BuildPreference::ReuseVerified
        && request.export_policy == AssistantExportPolicy::Minimal
        && program_args_select_exact_project(&request.program_args, project_name)
}

fn program_args_select_exact_project(program_args: &[OsString], project_name: &str) -> bool {
    match program_args {
        [only] => {
            let value = only.to_string_lossy();
            value == project_name
                || strip_option_value_ignore_ascii_case(&value, "--project=")
                    .is_some_and(|project| project == project_name)
        }
        [flag, value] => {
            flag.to_string_lossy().eq_ignore_ascii_case("--project")
                && value.to_string_lossy() == project_name
        }
        _ => false,
    }
}

#[cfg(test)]
fn is_visual_preflight_launcher_mode(args: &ArgsState) -> bool {
    matches!(
        &args.request,
        RunRequest::Execute(ExecutionRequest {
            target: RunTarget::VisualPreflight,
            ..
        })
    )
}

fn plan_launcher(
    project_root: &Path,
    version: &str,
    args: &ArgsState,
) -> AppResult<LauncherPlan> {
    let validation = args.validation();
    let build = plan_build(
        project_root,
        version,
        validation.build_profile(),
        args.build_preference(),
    )?;
    Ok(LauncherPlan { validation, build })
}

fn plan_build(
    project_root: &Path,
    version: &str,
    requested: CargoBuildProfile,
    build_preference: BuildPreference,
) -> AppResult<BuildPlan> {
    if build_preference == BuildPreference::ForceCompile {
        return Ok(build_plan_from_verification(
            requested,
            BuildPreference::ForceCompile,
            ArtifactAvailability { release: false, strict: false },
        ));
    }
    let current_signature = source_build_signature(project_root)?;
    let available = ArtifactAvailability {
        release: profile_build_artifact_is_verified(
            version,
            CargoBuildProfile::Release,
            &current_signature,
        ),
        strict: profile_build_artifact_is_verified(
            version,
            CargoBuildProfile::Strict,
            &current_signature,
        ),
    };
    Ok(build_plan_from_verification(
        requested,
        BuildPreference::ReuseVerified,
        available,
    ))
}

#[cfg(test)]
mod build_plan_tests {
    use super::{
        build_plan_from_verification, ArtifactAvailability, BuildPlan, BuildPreference,
        CompileReason, CargoBuildProfile, ValidationPlan,
    };

    #[test]
    fn plain_all_prefers_verified_release_artifact() {
        assert_eq!(
            build_plan_from_verification(
                CargoBuildProfile::Release,
                BuildPreference::ReuseVerified,
                ArtifactAvailability { release: true, strict: true },
            ),
            BuildPlan::Reuse(CargoBuildProfile::Release)
        );
    }

    #[test]
    fn plain_all_falls_back_to_verified_strict_artifact() {
        assert_eq!(
            build_plan_from_verification(
                CargoBuildProfile::Release,
                BuildPreference::ReuseVerified,
                ArtifactAvailability { release: false, strict: true },
            ),
            BuildPlan::Reuse(CargoBuildProfile::Strict)
        );
    }

    #[test]
    fn explicit_strict_never_reuses_release_artifact() {
        assert_eq!(
            build_plan_from_verification(
                CargoBuildProfile::Strict,
                BuildPreference::ReuseVerified,
                ArtifactAvailability { release: true, strict: false },
            ),
            BuildPlan::Compile {
                profile: CargoBuildProfile::Strict,
                reason: CompileReason::MissingVerifiedArtifact,
            }
        );
    }

    #[test]
    fn missing_verified_artifact_requires_a_build() {
        assert_eq!(
            build_plan_from_verification(
                CargoBuildProfile::Release,
                BuildPreference::ReuseVerified,
                ArtifactAvailability { release: false, strict: false },
            ),
            BuildPlan::Compile {
                profile: CargoBuildProfile::Release,
                reason: CompileReason::MissingVerifiedArtifact,
            }
        );
    }

    #[test]
    fn same_version_take_continuation_can_reuse_build() {
        assert_eq!(
            build_plan_from_verification(
                CargoBuildProfile::Strict,
                BuildPreference::ReuseVerified,
                ArtifactAvailability { release: false, strict: true },
            ),
            BuildPlan::Reuse(CargoBuildProfile::Strict)
        );
    }

    #[test]
    fn force_always_requests_a_fresh_build() {
        assert_eq!(
            build_plan_from_verification(
                CargoBuildProfile::Strict,
                BuildPreference::ForceCompile,
                ArtifactAvailability { release: true, strict: true },
            ),
            BuildPlan::Compile {
                profile: CargoBuildProfile::Strict,
                reason: CompileReason::Forced,
            }
        );
    }

    #[test]
    fn deep_clippy_requests_the_same_verified_strict_build_plan() {
        let validation = ValidationPlan::DeepClippy;
        assert_eq!(validation.build_profile(), CargoBuildProfile::Strict);
        assert_eq!(
            build_plan_from_verification(
                validation.build_profile(),
                BuildPreference::ReuseVerified,
                ArtifactAvailability { release: true, strict: true },
            ),
            BuildPlan::Reuse(CargoBuildProfile::Strict)
        );
    }
}

#[cfg(test)]
mod retired_same_version_token_tests {
    #[test]
    fn continuation_token_requires_rust_bootstrap_and_matching_version() {
        let source = include_str!("etuschlopf_launcher.rs");
        let token = ["ETUSCHLOPF_SAME_VERSION_", "TAKE_CONTINUATION"].concat();
        let parser = ["parse_same_version_take_", "continuation_signature"].concat();
        assert!(!source.contains(&token));
        assert!(!source.contains(&parser));
    }

    #[test]
    fn continuation_token_rejects_empty_or_malformed_signature() {
        let source = include_str!("etuschlopf_launcher.rs");
        let bootstrap = ["ETUSCHLOPF_TAKE_", "BOOTSTRAP"].concat();
        let global_stamp = ["target/etuschlopf_last_successful_", "build.version"].concat();
        assert!(!source.contains(&bootstrap));
        assert!(!source.contains(&global_stamp));
    }
}

fn process_identity(exe: &str, args: &[&str]) -> AppResult<String> {
    let output = Command::new(exe).args(args).output()?;
    if !output.status.success() {
        return Err(anyhow!("{exe} identity command failed with status {}", output.status));
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        return Err(anyhow!("{exe} identity command returned empty stdout"));
    }
    Ok(stdout)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CanonicalStrictValidationBasis {
    version: String,
    source_signature: String,
    rustc_identity: String,
    cargo_identity: String,
}

impl CanonicalStrictValidationBasis {
    fn material(&self) -> String {
        format!(
            "Version: {}\nSourceSignature: {}\nRustcIdentity: {}\nCargoIdentity: {}\n",
            self.version, self.source_signature, self.rustc_identity, self.cargo_identity
        )
    }
}

fn canonical_strict_validation_basis(version: &str) -> AppResult<CanonicalStrictValidationBasis> {
    let root = env::current_dir()?;
    Ok(CanonicalStrictValidationBasis {
        version: version.to_string(),
        source_signature: source_build_signature(&root)?,
        rustc_identity: process_identity("rustc", &["--version", "--verbose"])?
            .replace('\n', " | "),
        cargo_identity: process_identity("cargo", &["--version"])?,
    })
}

fn strict_clippy_validation_receipt_material_from_basis(
    basis: &CanonicalStrictValidationBasis,
    clippy_identity: &str,
) -> String {
    format!(
        "{}ClippyIdentity: {clippy_identity}\nClippyContract: {STRICT_CLIPPY_VALIDATION_CONTRACT}\n",
        basis.material()
    )
}

fn strict_clippy_validation_receipt_material(version: &str) -> AppResult<String> {
    let basis = canonical_strict_validation_basis(version)?;
    let clippy_identity = process_identity("cargo", &["clippy", "--version"])?;
    Ok(strict_clippy_validation_receipt_material_from_basis(
        &basis,
        &clippy_identity,
    ))
}

fn strict_clippy_validation_receipt_matches(version: &str) -> bool {
    let expected = match strict_clippy_validation_receipt_material(version) {
        Ok(value) => value,
        Err(_) => return false,
    };
    fs::read_to_string(STRICT_CLIPPY_VALIDATION_RECEIPT)
        .map(|actual| actual == expected)
        .unwrap_or(false)
}

fn write_strict_clippy_validation_receipt(version: &str) -> AppResult<()> {
    let material = strict_clippy_validation_receipt_material(version)?;
    let path = Path::new(STRICT_CLIPPY_VALIDATION_RECEIPT);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, material)?;
    Ok(())
}

fn write_reused_strict_clippy_log(path: &Path, version: &str) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let material = strict_clippy_validation_receipt_material(version)?;
    let mut file = File::create(path)?;
    writeln!(file, "ETUSCHLOPF STRICT CLIPPY RECEIPT REUSE")?;
    writeln!(file, "======================================")?;
    write!(file, "{material}")?;
    writeln!(file)?;
    writeln!(file, "CLIPPY-WARNING-BUDGET")?;
    writeln!(file, "---------------------")?;
    writeln!(file, "TotalWarnings: 0")?;
    writeln!(file, "RawWarningLines: 0")?;
    writeln!(file, "SummaryWarningLinesExcluded: 0")?;
    writeln!(file, "too_many_arguments: 0")?;
    writeln!(file, "type_complexity: 0")?;
    writeln!(file, "field_reassign_with_default: 0")?;
    writeln!(file, "needless_range_loop: 0")?;
    writeln!(file, "architectural_budget: 0")?;
    writeln!(file, "small_mechanical_budget: 0")?;
    writeln!(file, "LauncherClippyElapsedMs: 0")?;
    writeln!(file, "Decision: reused_exact_clean_validation_receipt; no current cargo clippy process required")?;
    Ok(())
}

fn strict_test_validation_receipt_material_from_basis(
    basis: &CanonicalStrictValidationBasis,
    strict_all_targets_source_signature: &str,
    binary_fnv: &str,
) -> String {
    format!(
        "{}StrictAllTargetsSourceSignature: {strict_all_targets_source_signature}\nStrictBinaryFNV: {binary_fnv}\nTestContract: {STRICT_TEST_VALIDATION_CONTRACT}\n",
        basis.material()
    )
}

fn strict_test_validation_receipt_material(version: &str, target_exe: &Path) -> AppResult<String> {
    let root = env::current_dir()?;
    let basis = canonical_strict_validation_basis(version)?;
    let strict_all_targets_source_signature = strict_all_targets_source_signature(&root)?;
    let binary_fnv = file_fnv1a64(target_exe)?;
    Ok(strict_test_validation_receipt_material_from_basis(
        &basis,
        &strict_all_targets_source_signature,
        &binary_fnv,
    ))
}

fn strict_test_validation_receipt_matches(version: &str, target_exe: &Path) -> bool {
    let expected = match strict_test_validation_receipt_material(version, target_exe) {
        Ok(value) => value,
        Err(_) => return false,
    };
    fs::read_to_string(STRICT_TEST_VALIDATION_RECEIPT)
        .map(|actual| actual == expected)
        .unwrap_or(false)
}

fn write_strict_test_validation_receipt(
    path: &Path,
    observed_material: Option<(&str, &str)>,
) -> AppResult<()> {
    // Invalidation and persistence stay at the existing receipt writer owner.
    match fs::remove_file(path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    let Some((before, after)) = observed_material else {
        return Ok(());
    };
    if before != after {
        return Err(anyhow!(
            "Strict-All-Targets: Receipt-Eingaben waehrend des Testlaufs geaendert; kein gebundener Validierungsnachweis"
        ));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, before)?;
    Ok(())
}

fn write_reused_strict_test_log(path: &Path, version: &str, target_exe: &Path) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let material = strict_test_validation_receipt_material(version, target_exe)?;
    let mut file = File::create(path)?;
    writeln!(file, "ETUSCHLOPF STRICT ALL-TARGET TEST RECEIPT REUSE")?;
    writeln!(file, "===============================================")?;
    write!(file, "{material}")?;
    writeln!(file, "Decision: reused_exact_validation_receipt; no current cargo test process required")?;
    Ok(())
}

fn should_run_integrated_launcher_tests(
    validation: ValidationPlan,
    launcher_tests: LauncherTestPlan,
) -> bool {
    validation == ValidationPlan::Strict
        && launcher_tests == LauncherTestPlan::VisualPreflight
}

fn cargo_test_summary_from_log(path: &Path) -> String {
    console_live_status::cargo_test_summary_text(&fs::read_to_string(path).unwrap_or_default())
}

fn prepare_current_run_test_log(path: &Path) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, "")?;
    Ok(())
}

fn resolve_result_file(project: &str) -> PathBuf {
    if project.trim().is_empty() {
        PathBuf::from("etuschlopf_run.result.txt")
    } else {
        PathBuf::from(format!("{project}.result.txt"))
    }
}

fn resolve_pdf_file(project: &str) -> PathBuf {
    if project.trim().is_empty() {
        PathBuf::from("etuschlopf_run.pdf")
    } else {
        PathBuf::from(format!("{project}.pdf"))
    }
}

fn resolve_render_musicxml_file(project: &str) -> PathBuf {
    if project.trim().is_empty() {
        PathBuf::from("etuschlopf_run.render.musicxml")
    } else {
        PathBuf::from(format!("{project}.render.musicxml"))
    }
}

fn resolve_musicxml_file(project: &str) -> PathBuf {
    if project.trim().is_empty() {
        PathBuf::from("etuschlopf_run.musicxml")
    } else {
        PathBuf::from(format!("{project}.musicxml"))
    }
}

fn resolve_project_wav_file(project: &str) -> PathBuf {
    PathBuf::from(format!("{project}.wav"))
}

fn resolve_project_midi_file(project: &str) -> PathBuf {
    PathBuf::from(format!("{project}.mid"))
}

fn visual_preflight_reuse_stamp_path(project: &str) -> PathBuf {
    Path::new(VISUAL_PREFLIGHT_REUSE_DIR).join(format!("{}.stamp", sanitize_checkpoint_component(project)))
}

fn project_run_input_signature(project: &str, version: &str) -> AppResult<String> {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    fnv1a_update(&mut hash, version.as_bytes());
    fnv1a_update(&mut hash, b"\nsource:");
    let source_signature = source_build_signature(&env::current_dir()?)?;
    fnv1a_update(&mut hash, source_signature.as_bytes());
    if let Ok(renderer) = env::var("ETUSCHLOPF_PDF_RENDERER") {
        fnv1a_update(&mut hash, b"\nrenderer:");
        fnv1a_update(&mut hash, renderer.as_bytes());
    }
    for path in [resolve_project_wav_file(project), resolve_project_midi_file(project)] {
        append_path_signature(&mut hash, &path)?;
    }
    Ok(format!("input:{hash:016x}"))
}

fn project_run_output_signature(project: &str) -> AppResult<String> {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    // Stable renderer reuse must depend on notation/render artifacts, not on
    // verbose report text. *.result.txt may change through report wording,
    // summaries or diagnostics while the rendered notation is unchanged.
    for path in [
        resolve_musicxml_file(project),
        resolve_render_musicxml_file(project),
        resolve_pdf_file(project),
    ] {
        append_path_signature(&mut hash, &path)?;
    }
    Ok(format!("output:{hash:016x}"))
}

fn append_path_signature(hash: &mut u64, path: &Path) -> AppResult<()> {
    let normalized = path.to_string_lossy().replace('\\', "/");
    fnv1a_update(hash, normalized.as_bytes());
    fnv1a_update(hash, b"\0");
    if !path.exists() || !path.is_file() {
        fnv1a_update(hash, b"missing\n");
        return Ok(());
    }
    let bytes = fs::read(path)?;
    fnv1a_update(hash, b"file:");
    fnv1a_update(hash, bytes.len().to_string().as_bytes());
    fnv1a_update(hash, b":");
    fnv1a_update(hash, &bytes);
    fnv1a_update(hash, b"\n");
    Ok(())
}

fn visual_preflight_project_renderer_reuse_status(project: &str, version: &str) -> AppResult<String> {
    let stamp_path = visual_preflight_reuse_stamp_path(project);
    if !stamp_path.exists() {
        return Ok("missing_stamp".to_string());
    }
    let result_file = resolve_result_file(project);
    let pdf_file = resolve_pdf_file(project);
    let render_xml_file = resolve_render_musicxml_file(project);
    let musicxml_file = resolve_musicxml_file(project);
    if !result_file.exists() || !pdf_file.exists() || !render_xml_file.exists() || !musicxml_file.exists() {
        return Ok("blocked_missing_output".to_string());
    }
    let result_text = fs::read_to_string(&result_file).unwrap_or_default();
    if parse_result_tool_version(&result_text).as_deref() != Some(version) {
        return Ok("blocked_result_version".to_string());
    }
    if parse_result_status(&result_text).as_deref() != Some("OK") {
        return Ok("blocked_result_not_ok".to_string());
    }
    let stamp = fs::read_to_string(stamp_path).unwrap_or_default();
    let current_input = project_run_input_signature(project, version)?;
    let current_output = project_run_output_signature(project)?;
    let mut stamp_version = "";
    let mut stamp_input = "";
    let mut stamp_output = "";
    for line in stamp.lines() {
        if let Some(value) = line.strip_prefix("Version: ") {
            stamp_version = value.trim();
        } else if let Some(value) = line.strip_prefix("InputSignature: ") {
            stamp_input = value.trim();
        } else if let Some(value) = line.strip_prefix("OutputSignature: ") {
            stamp_output = value.trim();
        }
    }
    if stamp_version != version {
        return Ok("blocked_stamp_version".to_string());
    }
    if stamp_input != current_input {
        return Ok("blocked_input_changed".to_string());
    }
    if stamp_output != current_output {
        return Ok("blocked_output_changed".to_string());
    }
    Ok("reused".to_string())
}

fn write_visual_preflight_project_reuse_stamp(project: &str, version: &str) -> AppResult<()> {
    fs::create_dir_all(VISUAL_PREFLIGHT_REUSE_DIR)?;
    let input_signature = project_run_input_signature(project, version)?;
    let output_signature = project_run_output_signature(project)?;
    let stamp_path = visual_preflight_reuse_stamp_path(project);
    fs::write(
        stamp_path,
        format!(
            "Version: {version}\nProject: {project}\nInputSignature: {input_signature}\nOutputSignature: {output_signature}\nDecision: renderer_reuse_allowed_for_unchanged_project_inputs\n"
        ),
    )?;
    Ok(())
}

fn append_visual_preflight_renderer_reuse_section(
    aggregate_path: &Path,
    version: &str,
    rows: &[String],
    project_count: usize,
) -> AppResult<()> {
    let reused = rows.iter().filter(|row| visual_preflight_renderer_reuse_row_status(row) == "reused").count();
    let rendered = rows.iter().filter(|row| visual_preflight_renderer_reuse_row_status(row) == "rendered").count();
    let blocked = rows.iter().filter(|row| visual_preflight_renderer_reuse_row_status(row) == "blocked").count();
    let missing_stamp = rows.iter().filter(|row| visual_preflight_renderer_reuse_row_status(row) == "missing_stamp").count();
    let mut file = OpenOptions::new().append(true).open(aggregate_path)?;
    writeln!(file)?;
    writeln!(file, "VISUAL-PREFLIGHT-RENDERER-REUSE")?;
    writeln!(file, "---------------------------------")?;
    writeln!(file, "Version: {version}")?;
    writeln!(file, "PlannedProjects: {project_count}")?;
    writeln!(file, "StatusCounts: reused={reused}; rendered={rendered}; blocked={blocked}; missing_stamp={missing_stamp}")?;
    writeln!(file, "Scope: pre_run_renderer_reuse_decision_only; a later WrittenButNonZeroExit is reported by RendererFinalStatus and does not increment blocked here.")?;
    writeln!(file, "Rows: project | status | reason | run_action")?;
    for row in rows {
        writeln!(file, "{row}")?;
    }
    writeln!(file, "MusicMutation: nein")?;
    writeln!(file, "MasterApply: nein")?;
    Ok(())
}

fn visual_preflight_renderer_reuse_action(status: &str) -> &'static str {
    if status == "reused" {
        "reused"
    } else if status == "missing_stamp" {
        "missing_stamp"
    } else {
        "rendered"
    }
}

fn visual_preflight_renderer_reuse_row_status(row: &str) -> &str {
    row.split('|').nth(1).map(str::trim).unwrap_or("unknown")
}



fn write_successful_build_stamp(
    version: &str,
    profile: CargoBuildProfile,
) -> AppResult<()> {
    fs::create_dir_all("target")?;
    let signature = source_build_signature(&env::current_dir()?)?;
    let artifact_signature = file_fnv1a64(&profile.target_exe())?;
    fs::write(profile_build_version_stamp(profile), format!("{version}\n"))?;
    fs::write(
        profile_build_signature_stamp(profile),
        format!("{signature}\n"),
    )?;
    fs::write(
        profile_build_artifact_stamp(profile),
        format!("{artifact_signature}\n"),
    )?;
    Ok(())
}

struct LoggedProcessSpec<'a> {
    exe: &'a Path,
    args: &'a [OsString],
    log_path: &'a Path,
    header: &'a str,
    progress_role: PresentationRole,
    heartbeat: &'a str,
    completion_summary: CompletionSummary,
}

fn invoke_logged_program(
    launcher: &mut Launcher,
    exe: &Path,
    args: &[OsString],
    log_path: &Path,
    header: &str,
    progress_role: PresentationRole,
    heartbeat: &str,
) -> AppResult<RunResult> {
    invoke_logged_program_with_env(
        launcher,
        LoggedProcessSpec {
            exe,
            args,
            log_path,
            header,
            progress_role,
            heartbeat,
            completion_summary: CompletionSummary::LogSize,
        },
        ProcessEnvMode::Inherit,
    )
}

fn invoke_logged_observed_target(
    launcher: &mut Launcher,
    spec: LoggedProcessSpec<'_>,
    context: observation::ObservationSessionContext,
) -> AppResult<RunResult> {
    invoke_logged_program_with_env(launcher, spec, ProcessEnvMode::ObservedTarget(context))
}

#[derive(Clone)]
enum ProcessEnvMode {
    Inherit,
    NormalCargoTarget,
    ObservedTarget(observation::ObservationSessionContext),
}

const RUNTIME_PRODUCER_VERSION_ENV: &str = "ETUSCHLOPF_RUNTIME_PRODUCER_VERSION";
const RUNTIME_PRODUCER_SOURCE_SIGNATURE_ENV: &str =
    "ETUSCHLOPF_RUNTIME_PRODUCER_SOURCE_SIGNATURE";
const RUNTIME_PRODUCER_BINARY_FINGERPRINT_ENV: &str =
    "ETUSCHLOPF_RUNTIME_PRODUCER_BINARY_FINGERPRINT";
const RUNTIME_PRODUCER_BINDING_STATE_ENV: &str =
    "ETUSCHLOPF_RUNTIME_PRODUCER_BINDING_STATE";

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeProducerBinding {
    source_signature: Option<String>,
    binary_fingerprint: Option<String>,
    binding_state: &'static str,
}

fn runtime_producer_binding_state(
    source_and_binary_bound: bool,
    binary_fingerprint_present: bool,
) -> &'static str {
    if source_and_binary_bound && binary_fingerprint_present {
        "bound"
    } else if binary_fingerprint_present {
        "partially_bound"
    } else {
        "unbound"
    }
}

fn runtime_producer_binding_for_executable(exe: &Path, version: &str) -> RuntimeProducerBinding {
    let verified_profile = [CargoBuildProfile::Strict, CargoBuildProfile::Release]
        .into_iter()
        .find(|profile| profile.target_exe().as_path() == exe);
    let Some(profile) = verified_profile else {
        return RuntimeProducerBinding {
            source_signature: None,
            binary_fingerprint: None,
            binding_state: "unbound",
        };
    };

    let source_signature = env::current_dir()
        .ok()
        .and_then(|root| source_build_signature(&root).ok());
    let binary_fingerprint = file_fnv1a64(exe).ok();
    let source_and_binary_bound = source_signature.as_deref().is_some_and(|signature| {
        profile_build_artifact_is_verified(version, profile, signature)
    });
    let binding_state = runtime_producer_binding_state(
        source_and_binary_bound,
        binary_fingerprint.is_some(),
    );
    let bound_source_signature = if binding_state == "bound" {
        source_signature
    } else {
        None
    };

    RuntimeProducerBinding {
        source_signature: bound_source_signature,
        binary_fingerprint,
        binding_state,
    }
}

fn bind_runtime_producer_environment(command: &mut Command, exe: &Path, version: &str) {
    let producer = runtime_producer_binding_for_executable(exe, version);
    command.env(RUNTIME_PRODUCER_BINDING_STATE_ENV, producer.binding_state);
    if producer.binding_state == "bound" {
        command.env(RUNTIME_PRODUCER_VERSION_ENV, version);
    } else {
        command.env_remove(RUNTIME_PRODUCER_VERSION_ENV);
    }
    if let Some(source_signature) = producer.source_signature.as_deref() {
        command.env(RUNTIME_PRODUCER_SOURCE_SIGNATURE_ENV, source_signature);
    } else {
        command.env_remove(RUNTIME_PRODUCER_SOURCE_SIGNATURE_ENV);
    }
    if let Some(binary_fingerprint) = producer.binary_fingerprint.as_deref() {
        command.env(RUNTIME_PRODUCER_BINARY_FINGERPRINT_ENV, binary_fingerprint);
    } else {
        command.env_remove(RUNTIME_PRODUCER_BINARY_FINGERPRINT_ENV);
    }
}

fn invoke_logged_program_with_env(
    launcher: &mut Launcher,
    spec: LoggedProcessSpec<'_>,
    env_mode: ProcessEnvMode,
) -> AppResult<RunResult> {
    let LoggedProcessSpec {
        exe,
        args,
        log_path,
        header,
        progress_role,
        heartbeat,
        completion_summary,
    } = spec;
    append_command_header(log_path, header, exe, args)?;
    if matches!(env_mode, ProcessEnvMode::NormalCargoTarget) {
        writeln!(OpenOptions::new().create(true).append(true).open(log_path)?,
            "CargoHostBuildProfile: strict.build-override.opt-level=1; gates unchanged")?;
    }
    let stdout_file = OpenOptions::new().create(true).append(true).open(log_path)?;
    let stderr_file = stdout_file.try_clone()?;
    let start = Instant::now();
    let mut command = Command::new(exe);
    command
        .args(args)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file));
    match &env_mode {
        ProcessEnvMode::NormalCargoTarget => {
            // run.cmd verwendet fuer beide kleinen Bootstrap-Binaries ein gemeinsames
            // --target-dir. Die eigentlichen Cargo-Builds des Werkzeugs muessen weiter
            // das normale target/ nutzen, sonst zeigt target_exe_for_profile() falsch.
            // Ein geerbtes CARGO_TARGET_DIR wird deshalb weiterhin entfernt.
            command.env_remove("CARGO_TARGET_DIR");
            // Optimize host build scripts and proc macros; productive/test profiles stay unchanged.
            command.env("CARGO_PROFILE_STRICT_BUILD_OVERRIDE_OPT_LEVEL", "1");
        }
        ProcessEnvMode::ObservedTarget(context) => {
            bind_runtime_producer_environment(&mut command, exe, &launcher.version);
            command.env(observation::SESSION_ENV, context.session_id().as_str());
            command.env(observation::SOURCE_ENV, context.source_id().as_str());
            if let Some(parent) = context.parent_source_id() {
                command.env(observation::PARENT_SOURCE_ENV, parent.as_str());
            } else {
                command.env_remove(observation::PARENT_SOURCE_ENV);
            }
        }
        ProcessEnvMode::Inherit => {}
    }
    let mut live_status = match &env_mode {
        ProcessEnvMode::ObservedTarget(context) => {
            command.env(
                etuschlopf::console_status_record::ENABLE_ENV,
                etuschlopf::console_status_record::ENABLE_VALUE,
            );
            Some(console_live_status::LiveStatusReader::starting_at_current_end(
                log_path,
                context.session_id().as_str(),
            ))
        }
        _ => None,
    };
    let durations_path = Path::new(console_live_status::LAST_PHASE_DURATIONS_FILE);
    // Keyed by the technical phase header: one value per mode (e.g. --all vs. one
    // project, Clippy vs. Deep Clippy), never mixed through a shared display label.
    let previous_ms = console_live_status::read_last_phase_durations(durations_path)
        .get(header)
        .copied();
    let spawn_started = Instant::now();
    let mut child = command.spawn()?;
    let spawn_us = spawn_started.elapsed().as_micros();
    let mut last_alive_lower_us = 0;
    let progress_mode = launcher.stdout_progress_mode();
    let glyph_mode = launcher.stdout_glyph_mode();
    let mut animation = ProgressAnimation::new();
    if progress_mode == ProgressMode::Lines {
        let message = format!("[EtuSchlopf] {heartbeat} läuft ...");
        launcher
            .console
            .write_stdout_line(progress_line_presentation(progress_role, &message));
    }
    loop {
        let elapsed = start.elapsed();
        let poll_started_us = spawn_started.elapsed().as_micros();
        if let Some(reader) = live_status.as_mut() {
            for event in reader.poll() {
                launcher.live_status_event(&event, progress_mode)?;
            }
        }
        if let Some(status) = child.try_wait()? {
            let elapsed = start.elapsed();
            let exit_upper_us = spawn_started.elapsed().as_micros();
            let completion_started = Instant::now();
            let exit_code = status.code().unwrap_or(1);
            if let Some(reader) = live_status.as_mut() {
                for event in reader.poll() {
                    launcher.live_status_event(&event, progress_mode)?;
                }
            }
            let test_counts_text = match completion_summary {
                CompletionSummary::CargoTestCounts => Some(console_live_status::cargo_test_counts_display(
                    &console_live_status::cargo_test_counts(&fs::read_to_string(log_path).unwrap_or_default()),
                )),
                CompletionSummary::LogSize => None,
            };
            // Success: summed test counts and/or the last successful duration for
            // comparison; the log size is only shown when something went wrong.
            let success_detail = [
                test_counts_text,
                previous_ms.map(|ms| format!("zuletzt {}", format_elapsed(Duration::from_millis(ms)))),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" · ");
            let detail = if exit_code != 0 {
                CompletionDetail::LogSize(log_file_size(log_path))
            } else if success_detail.is_empty() {
                CompletionDetail::None
            } else {
                CompletionDetail::Text(&success_detail)
            };
            launcher.progress_done(ProgressCompletion {
                role: progress_role,
                label: heartbeat,
                elapsed,
                detail,
                exit_code,
            })?;
            if exit_code == 0 {
                console_live_status::record_last_phase_duration(durations_path, header, elapsed);
            }
            if let Some(summary) = live_status.as_ref().and_then(console_live_status::LiveStatusReader::completion_summary) {
                launcher.role_detail_line(
                    ConsolePrefix::Product,
                    progress_role,
                    heartbeat,
                    &summary,
                    Emphasis::Secondary,
                );
            }
            append_launcher_timing_line(&launcher_process_timing_line(LauncherProcessTiming {
                phase: header, started: start, observed_end: start + elapsed, spawn_us,
                exit_lower_us: last_alive_lower_us, exit_upper_us,
                completion_ui_us: completion_started.elapsed().as_micros(), exit_code,
            }));
            return Ok(RunResult {
                exit_code,
                elapsed_ms: elapsed.as_millis(),
            });
        }
        last_alive_lower_us = poll_started_us;
        if progress_mode == ProgressMode::Inline {
            // r80: no width detection; the in-place line is kept short at its source so it
            // never wraps (a wrapped line cannot be redrawn with a carriage return).
            let base_width = animation
                .running_line(heartbeat, elapsed, previous_ms.map(Duration::from_millis), None, glyph_mode)
                .plain()
                .chars()
                .count();
            let summary_budget = RUNNING_LINE_MAX_CHARS.saturating_sub(base_width + " · ".chars().count());
            let live_summary = live_status
                .as_ref()
                .filter(|reader| reader.has_projects())
                .map(|reader| reader.running_summary(summary_budget))
                .filter(|summary| !summary.is_empty());
            let running_line = animation.running_line(
                heartbeat,
                elapsed,
                previous_ms.map(Duration::from_millis),
                live_summary.as_deref(),
                glyph_mode,
            );
            launcher.console.write_stdout_inline_segments(progress_role, &running_line)?;
        }
        thread::sleep(Duration::from_millis(ANIMATION_INTERVAL_MS));
        animation.advance();
    }
}

#[cfg(test)]
mod runtime_producer_binding_tests {
    use super::*;

    #[test]
    fn binding_state_is_fail_closed_and_distinguishes_partial_binding() {
        assert_eq!(runtime_producer_binding_state(true, true), "bound");
        assert_eq!(runtime_producer_binding_state(true, false), "unbound");
        assert_eq!(
            runtime_producer_binding_state(false, true),
            "partially_bound"
        );
        assert_eq!(runtime_producer_binding_state(false, false), "unbound");
    }

    #[test]
    fn non_product_target_cannot_claim_runtime_producer_binding() {
        let binding = runtime_producer_binding_for_executable(
            Path::new("target/not-a-product-profile/etuschlopf"),
            VERSION,
        );
        assert_eq!(binding.binding_state, "unbound");
        assert!(binding.source_signature.is_none());
        assert!(binding.binary_fingerprint.is_none());
    }

    #[test]
    fn observed_target_transport_names_are_explicit_and_stable() {
        assert_eq!(
            RUNTIME_PRODUCER_VERSION_ENV,
            "ETUSCHLOPF_RUNTIME_PRODUCER_VERSION"
        );
        assert_eq!(
            RUNTIME_PRODUCER_SOURCE_SIGNATURE_ENV,
            "ETUSCHLOPF_RUNTIME_PRODUCER_SOURCE_SIGNATURE"
        );
        assert_eq!(
            RUNTIME_PRODUCER_BINARY_FINGERPRINT_ENV,
            "ETUSCHLOPF_RUNTIME_PRODUCER_BINARY_FINGERPRINT"
        );
        assert_eq!(
            RUNTIME_PRODUCER_BINDING_STATE_ENV,
            "ETUSCHLOPF_RUNTIME_PRODUCER_BINDING_STATE"
        );
    }
}

fn append_command_header(path: &Path, header: &str, exe: &Path, args: &[OsString]) -> AppResult<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file)?;
    writeln!(file, "{header}")?;
    writeln!(file, "{}", "-".repeat(header.len()))?;
    writeln!(file, "Befehl: {} {}", exe.display(), join_windows_args(args))?;
    writeln!(file)?;
    Ok(())
}

fn join_windows_args(args: &[OsString]) -> String {
    args.iter()
        .map(|arg| quote_windows_arg(&arg.to_string_lossy()))
        .collect::<Vec<String>>()
        .join(" ")
}

fn quote_windows_arg(arg: &str) -> String {
    if arg.is_empty() {
        return "\"\"".to_string();
    }
    if !arg.chars().any(|ch| ch.is_whitespace() || ch == '"') {
        return arg.to_string();
    }
    let escaped = arg.replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn revision_label(revision: Option<u64>) -> String {
    revision.map_or_else(|| "r?".to_string(), |revision| format!("r{revision}"))
}

/// Interactive startup header: running revision first, the frozen package version only in
/// brackets. During a take the running tree is the candidate, not yet the accepted head.
fn startup_header_text(
    revisions: &assistant_export::ConsoleRevisionProjection,
    take_active: bool,
    target: &str,
    thread_summary: &str,
) -> String {
    let running = if take_active {
        revisions.in_flight_candidate.or(revisions.source_tree)
    } else {
        revisions.source_tree
    };
    let state = if take_active {
        "Take-Kandidat"
    } else if running.is_some() && running == revisions.canonical_head {
        "akzeptiert"
    } else {
        "nicht als akzeptiert belegt"
    };
    format!(
        "EtuSchlopf {} ({VERSION}) · {state} · {target} · {thread_summary}",
        revision_label(running)
    )
}

fn take_header_detail(revisions: &assistant_export::ConsoleRevisionProjection, change_class: &str) -> String {
    let mut text = format!(
        "{} akzeptiert → {}",
        revision_label(revisions.canonical_head),
        revision_label(revisions.in_flight_candidate.or(revisions.source_tree))
    );
    if !change_class.is_empty() {
        text.push_str(" · ");
        text.push_str(change_class);
    }
    text
}

/// K7: one console line that recaps the silent pre-launcher steps of this
/// session from the advisory startup-timing readback (run.cmd Cargo builds and
/// the take bootstrap before reexec). Presentation only; missing or unpaired
/// measurements are simply omitted, never shown as zero.
#[derive(Debug, Default, PartialEq, Eq)]
struct StartupPreparationTimes {
    take_build_ms: Option<u64>,
    take_check_ms: Option<u64>,
    launcher_build_ms: Option<u64>,
}

impl StartupPreparationTimes {
    fn summary(&self) -> Option<String> {
        let parts = [
            ("Bootstrap-Build", self.take_build_ms),
            ("Take-Prüfung", self.take_check_ms),
            ("Launcher-Build", self.launcher_build_ms),
        ]
        .into_iter()
        .filter_map(|(label, millis)| {
            millis.map(|ms| format!("{label} {}", format_preparation_duration(ms)))
        })
        .collect::<Vec<_>>();
        (!parts.is_empty()).then(|| parts.join(" · "))
    }
}

fn startup_preparation_times(readback: &str) -> StartupPreparationTimes {
    fn value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
        line.split_once(": ")?.1.split(';').find_map(|part| {
            let (name, value) = part.trim().split_once('=')?;
            (name == key).then_some(value)
        })
    }
    let mut times = StartupPreparationTimes::default();
    for line in readback.lines() {
        if line.starts_with("StartupCmdDuration:") {
            let millis = value(line, "WallMs").and_then(|ms| ms.parse::<u64>().ok());
            match value(line, "Phase") {
                Some("TakeBootstrapCargoBuild") => times.take_build_ms = millis.or(times.take_build_ms),
                Some("LauncherBootstrapCargoBuild") => {
                    times.launcher_build_ms = millis.or(times.launcher_build_ms);
                }
                _ => {}
            }
        } else if line.starts_with("StartupRustTiming:")
            && value(line, "Phase") == Some("TakeBeforeReexec")
            && value(line, "Event") == Some("end")
        {
            times.take_check_ms = value(line, "WallUs")
                .and_then(|us| us.parse::<u64>().ok())
                .map(|us| us / 1000)
                .or(times.take_check_ms);
        }
    }
    times
}

#[cfg(test)]
fn startup_preparation_summary(readback: &str) -> Option<String> {
    startup_preparation_times(readback).summary()
}

/// K8: run.cmd shows "zuletzt N s" for its two silent Cargo builds. It reads
/// these two tiny files with `set /p`; writing them is best effort and a
/// missing or failed write only drops that hint.
const TAKE_BOOTSTRAP_BUILD_LAST_FILE: &str = "target/etuschlopf_logs/take_bootstrap_build_last_s.txt";
const LAUNCHER_BUILD_LAST_FILE: &str = "target/etuschlopf_logs/launcher_build_last_s.txt";

fn remember_bootstrap_build_seconds(times: &StartupPreparationTimes) {
    for (path, millis) in [
        (TAKE_BOOTSTRAP_BUILD_LAST_FILE, times.take_build_ms),
        (LAUNCHER_BUILD_LAST_FILE, times.launcher_build_ms),
    ] {
        if let Some(millis) = millis {
            let _ = fs::write(path, build_seconds_text(millis));
        }
    }
}

fn build_seconds_text(millis: u64) -> String {
    ((millis + 500) / 1000).to_string()
}

fn format_preparation_duration(millis: u64) -> String {
    if millis < 10_000 {
        format!("{},{} s", millis / 1000, (millis % 1000) / 100)
    } else {
        let seconds = (millis + 500) / 1000;
        if seconds < 60 {
            format!("{seconds} s")
        } else {
            format!("{} min {:02} s", seconds / 60, seconds % 60)
        }
    }
}

fn format_elapsed(duration: Duration) -> String {
    console_live_status::format_minutes(duration)
}

fn spinner_symbol(tick: usize, glyphs: GlyphMode) -> &'static str {
    const UNICODE_SPINNER: [&str; 8] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
    const ASCII_SPINNER: [&str; 4] = ["|", "/", "-", "\\"];
    match glyphs {
        GlyphMode::Unicode => UNICODE_SPINNER[tick % UNICODE_SPINNER.len()],
        GlyphMode::Ascii => ASCII_SPINNER[tick % ASCII_SPINNER.len()],
    }
}

fn visible_console_width(line: &str) -> usize {
    let mut width = 0usize;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            for esc_ch in chars.by_ref() {
                if esc_ch == 'm' {
                    break;
                }
            }
        } else {
            width = width.saturating_add(1);
        }
    }
    width
}

fn log_file_size(path: &Path) -> u64 {
    fs::metadata(path).map(|metadata| metadata.len()).unwrap_or(0)
}

fn format_log_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MiB", bytes as f64 / (1024.0 * 1024.0))
    }
}

fn append_run_diagnostic(state: &RunState, run_mode: &str) -> AppResult<()> {
    let Some(result_file) = &state.result_file else {
        return Ok(());
    };
    if !result_file.exists() {
        return Ok(());
    }
    let mut file = OpenOptions::new().append(true).open(result_file)?;
    let total_ms = state.total_start.elapsed().as_millis();
    let strict_text = if state.validation.is_strict() { "ja" } else { "nein" };
    writeln!(file)?;
    writeln!(file, "RUN-COMMAND-DIAGNOSTIC")?;
    writeln!(file, "----------------------")?;
    writeln!(
        file,
        "Status: v{} rust launcher diagnostic. This section is ASCII-only to keep the UTF-8 result file valid.",
        VERSION
    )?;
    writeln!(
        file,
        "Kurz: RunMode {run_mode}; StrictMode {strict_text}; BuildProfile {}; BuildDecision {}; CargoBuildExitCode {}; ClippyExitCode {}; IntegratedLauncherTestExitCode {}; ProgramExitCode {}; ConsoleCaptureMode quiet_console_full_output_in_target_etuschlopf_logs; ConsoleRenderPolicy stream_capability_plus_environment_preferences_no_global_color_state; CleanupProjection technical_log_only; HeartbeatProgress typed_role_compact_spinner_elapsed_log_size_terminal_only_redirect_line_mode; LauncherOwner rust_no_powershell.",
        state.build_outcome.profile().as_str(),
        state.build_outcome.decision_label(),
        state.build_exit,
        state.strict_clippy_exit,
        state.strict_test_exit,
        state.run_exit
    )?;
    writeln!(
        file,
        "TimingBaseline: TotalMs {total_ms}; ClippyMs {}; CargoBuildMs {}; IntegratedLauncherTestMs {}; ProgramMs {}; MeasurementOwner rust_launcher_no_powershell; TotalScope managed_run_initialization_to_diagnostic_before_export; Excluded launcher_prelude_run_cmd_take_bootstrap_and_export_tail; ChildMs completion_detection_wall_not_CPU; ParallelPrewarmLifetimeNotAdditive ja; DetailedReadback assistant_handoff_launcher_timing; DomainBehaviorChange nein; ConsoleInfrastructureChange ja.",
        state.clippy_ms, state.build_ms, state.strict_test_ms, state.program_ms
    )?;
    writeln!(
        file,
        "StrictPolicy: Strict is the unconditional public baseline: project, corpus and visual-preflight execution default to ValidationPlan::Strict even without --strict. Every normal run requires the exact cargo clippy --locked --profile strict --jobs {} --color never blocking correctness/suspicious/perf contract. A prior clean Clippy invocation may satisfy that contract only through an exact derived receipt bound to current source signature, rustc identity, cargo identity, clippy identity and the immutable Clippy-contract id; receipts are written only for zero-warning successful Clippy runs, and any mismatch, unreadable receipt or non-clean prior run executes current Clippy again. The productive build profile is {}, cargo build uses --locked --bin etuschlopf --jobs {}, and every normal project/corpus run requires the exact cargo --config profile.strict.package.etuschlopf.opt-level=0 test --locked --profile strict --all-targets --no-fail-fast validation contract. A prior successful invocation may satisfy that test contract only through an exact derived receipt bound to current source signature, strict executable FNV, rustc identity, cargo identity, the complete managed repository plus existing test-target input scope and the immutable test-contract id; any mismatch or unreadable receipt runs the full test command again. Only EtuSchlopf test-target codegen changes, while the productive strict/opt3 build remains separate; no-fail-fast erweitert nur die Fehlerinventur, jeder Testfehler bleibt blockierend. --strict remains only an idempotent backwards-compatible marker; no separate Clippy CLI switch is required.",
        state.thread_plan.cargo_jobs, state.build_outcome.profile().as_str(), state.thread_plan.cargo_jobs
    )?;
    writeln!(
        file,
        "ThreadPlan: LogicalThreads {}; CargoJobs {}; PipelineAutoHint {}; CargoJobsExplicit ja; PipelineJobsOwner etuschlopf_cli_auto_or_user_cli.",
        state.thread_plan.logical_threads, state.thread_plan.cargo_jobs, state.thread_plan.pipeline_auto_hint_workers
    )?;
    writeln!(
        file,
        "MultithreadingPolicy: Launcher animation stays one UI loop; Cargo gets explicit --jobs; EtuSchlopf pipeline keeps deterministic --jobs auto/CLI worker planning and reports actual workers in the result."
    )?;
    writeln!(
        file,
        "BuildReusePolicy: every launcher path uses one ValidationPlan and one BuildPlan, including --clippy-deep. Public execution is Strict by default. Successful cargo builds write profile-bound release/strict version, source-signature and executable-hash stamps. Reuse requires matching version, current source signature and executable bytes; a reused strict executable also requires the strict Clippy contract, satisfied either by current Clippy or an exact clean source/toolchain/contract-bound Clippy receipt. Strict and Deep Clippy never fall back to release. Changed source requires a build, unchanged source may reuse, and --force always builds. Cargo incremental artifacts are derived cache only and may accelerate that required build; they never satisfy the verified-binary reuse decision."
    )?;
    writeln!(
        file,
        "ConsoleUxPolicy: Launcher presentation is rendered from the shared authority-free presentation contract plus per-stream render policy. stdout and stderr terminal capability are resolved independently; NO_COLOR disables styling; redirected streams stay plain and line-oriented; ETUSCHLOPF_ASCII_PROGRESS selects ASCII progress glyphs. Long child phases use typed PresentationRole instead of heartbeat-label parsing. No run.cmd color command, cmd /C color subprocess or inherited Windows default-color state is presentation authority. Normal roles use a restrained palette, child completion styles the local status separately from duration/log detail, diagnostic tails are secondary context, live-line cleanup follows the previous rendered width instead of a fixed terminal width, and strong final outcome styling continues to come only from TruthfulRunOutcomeProjection."
    )?;
    writeln!(
        file,
        "TempCleanupPolicy: stale OS temp dirs and old Rust-launcher bootstrap targets are cleaned by the Rust launcher; active bootstrap target is skipped until a later run. BuildCacheHygiene retains productive target/strict and target/bc Cargo caches, including their incremental state; run.cmd sets CARGO_INCREMENTAL=1. Cleanup removes only the retired target/etuschlopf_bootstrap_cache root and fingerprinted stale target/debug, target/release and root-level debug directories, while protecting active executables and current project output files. Retained Cargo artifacts are derived cache, never build-reuse or validation authority. RootHygiene keeps Cargo/run/src/docs/tools/target plus WAV/MIDI inputs and current standard outputs; stale root ZIP/log/temp/output residue and legacy assistant README/manifest root docs are removed."
    )?;
    writeln!(
        file,
        "PowerShellPolicy: Startprogramm, Take, Herzschlag, Prozessprotokollierung und Versionswechsel liegen in Rust; run.cmd ist nur ein minimaler Aufruf. Die Take-Startkompilation durchlaeuft die verpflichtende Architekturpruefung; ein besonderer Umgehungskontext bleibt nicht bestehen; der erneut ausgefuehrte normale Bau prueft alle Schranken. Es gibt keinen PowerShell-Wiederherstellungspfad."
    )?;
    writeln!(file, "Entscheidung: Ergebnisdatei bleibt die Primaerdiagnose. Konsole nicht standardmaessig anfordern.")?;
    Ok(())
}


fn update_normal_run_result_evidence(state: &mut RunState, exit_code: i32) {
    let Some(result_path) = state.result_file.as_ref() else {
        return;
    };
    let Ok(text) = fs::read_to_string(result_path) else {
        return;
    };

    if exit_code == 2 && normal_result_confirms_controlled_audit_block(&text) {
        state.audit_signal = AuditSignal::ExpectedBlocked;
    }
    if exit_code == 0 {
        state.renderer_outcome = state
            .renderer_outcome
            .merge(normal_result_renderer_outcome(&text));
    }
    state.render_candidate_selected_pdf_truth = render_candidate_selected_pdf_truth(&text);
}

fn normal_result_confirms_controlled_audit_block(text: &str) -> bool {
    text.lines()
        .take(24)
        .any(|line| line.trim().starts_with("Ergebnis: BLOCKIERT"))
}

fn normal_result_silent_voice_renderer_section(text: &str) -> Option<&str> {
    let section = text
        .split("SILENT-VOICE-COVERAGE-WRITE-OUTCOME")
        .nth(1)?;
    Some(
        section
            .split("\nRUN-COMMAND-DIAGNOSTIC")
            .next()
            .unwrap_or(section),
    )
}

#[cfg(test)]
fn normal_result_confirms_renderer_outcome_observed(text: &str) -> bool {
    normal_result_silent_voice_renderer_section(text).is_some_and(|section| {
        section
            .lines()
            .any(|line| line.trim() == "RendererOutcome: observed_from_existing_writer_audit.")
    })
}

fn normal_result_renderer_outcome(text: &str) -> RendererOutcome {
    let Some(section) = normal_result_silent_voice_renderer_section(text) else {
        return RendererOutcome::NotObserved;
    };
    if !section
        .lines()
        .any(|line| line.trim() == "RendererOutcome: observed_from_existing_writer_audit.")
    {
        return RendererOutcome::NotObserved;
    }

    section
        .lines()
        .rev()
        .find_map(normal_result_pdf_renderer_outcome_line)
        .unwrap_or(RendererOutcome::NotObserved)
}

fn normal_result_pdf_renderer_outcome_line(line: &str) -> Option<RendererOutcome> {
    let line = line.trim();
    if !line.starts_with("PDF: ") {
        return None;
    }

    if let Some((_, exit_and_renderer)) =
        line.split_once(" wurde erzeugt, aber Renderer-ExitCode ist ")
    {
        let (exit_code, _) = exit_and_renderer.split_once(". Renderer:")?;
        let exit_code = exit_code.trim().parse::<i32>().ok()?;
        return Some(if exit_code == 0 {
            RendererOutcome::Passed
        } else {
            RendererOutcome::OutputValidProcessFailed(exit_code)
        });
    }

    if line.contains(" geschrieben. Renderer:") && line.ends_with(". ExitCode 0.") {
        return Some(RendererOutcome::Passed);
    }

    None
}

fn render_candidate_selected_pdf_truth(
    text: &str,
) -> Option<RenderCandidateSelectedPdfTruth> {
    let section = text
        .split("RENDER-WRITE-PDF-SOURCE-ADOPTION-TRUTH-GATE")
        .nth(1)?;
    let short = section.lines().map(str::trim).find(|line| line.starts_with("Kurz:"))?;
    let pdf_source = section
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("PdfSource:"))?;
    let selected_pdf_source_label = pdf_source
        .split("label ")
        .nth(1)?
        .trim()
        .trim_end_matches('.')
        .to_string();
    let selected_pdf_source = if selected_pdf_source_label == "MusicXML" {
        "Master-MusicXML".to_string()
    } else if selected_pdf_source_label.starts_with("Render-") {
        "Render-MusicXML".to_string()
    } else {
        selected_pdf_source_label.clone()
    };
    let candidate_verdict = short
        .split("Urteil ")
        .nth(1)
        .unwrap_or("unknown")
        .trim()
        .trim_end_matches('.')
        .to_string();
    Some(RenderCandidateSelectedPdfTruth {
        write_produced: short.contains("WriteProduced ja"),
        write_validated: short.contains("WriteValidated ja"),
        selected_pdf_source,
        selected_pdf_source_label,
        pdf_source_adopted: short.contains("PdfSourceAdopted ja"),
        productive_materialization_green: short.contains("ProductiveMaterializationGreen ja"),
        candidate_verdict,
    })
}

fn compact_check_console_label(value: &str) -> &'static str {
    match value.trim() {
        "0" => "OK",
        "not_run" => "nicht gelaufen",
        _ => "FEHLER",
    }
}

fn compact_technical_console_summary(state: &RunState, warning_count: usize) -> String {
    let build_action = match state.build_outcome {
        BuildOutcome::Pending(_) => "ausstehend",
        BuildOutcome::Reused(_) => "wiederverwendet",
        BuildOutcome::Compiled(_) => "neu",
    };
    format!(
        "Technik: {} {build_action} | {} Jobs | Build {} | Clippy {} | Tests {} | Warnungen {warning_count}",
        state.build_outcome.profile().as_str(),
        state.thread_plan.cargo_jobs,
        compact_check_console_label(&state.build_exit),
        compact_check_console_label(&state.strict_clippy_exit),
        compact_check_console_label(&state.strict_test_exit),
    )
}

fn final_console_status_label(
    console_status: &str,
    release_decision: &str,
    warning_count: usize,
) -> &'static str {
    if console_status == "AUDIT-BLOCKED"
        && warning_count > 0
        && release_decision == "audit_blocked_expected_audit_block"
    {
        "WARNING-DEBT"
    } else {
        match console_status {
            "AUDIT-BLOCKED" => "AUDIT-STOP",
            "NO-SEED" => "NO-SEED",
            "ERROR" => "ERROR",
            "OK" => "OK",
            _ => "STATUS?",
        }
    }
}

fn final_console_hint(
    base_hint: Option<&'static str>,
    console_status: &str,
    release_decision: &str,
    warning_count: usize,
) -> Option<&'static str> {
    if warning_count > 0 && release_decision == "audit_blocked_expected_audit_block" {
        Some("Freigabe: gelb | sichtbare Warning-Debt; technischer Lauf erfolgreich.")
    } else {
        match console_status {
            "AUDIT-BLOCKED" => Some("Freigabe: gelb | erwarteter Audit-Stop."),
            "NO-SEED" => Some("Freigabe: gelb | NO-SEED; kein aktuelles Audit-Ziel."),
            _ => base_hint,
        }
    }
}

fn print_summary(
    launcher: &mut Launcher,
    state: &RunState,
    mode: &str,
    exit_code: i32,
) -> AppResult<OutcomeColor> {
    let (typed_outcome, warning_count) =
        typed_run_outcome_with_warning_debt_for_console(state, mode, exit_code);
    let run_outcome = truthful_run_outcome_projection(typed_outcome);
    let console_status = final_console_status_label(
        run_outcome.report.console_status,
        &run_outcome.report.release_decision,
        warning_count,
    );

    let project = state.input_arg.as_deref().unwrap_or("-");
    let result = state
        .result_file
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "-".to_string());
    launcher.line(
        PresentationRole::Infrastructure,
        None,
        Attention::Neutral,
        Emphasis::Secondary,
        "",
    );
    launcher.outcome_headline(run_outcome.severity, console_status, project, exit_code);
    launcher.line(
        PresentationRole::Build,
        None,
        Attention::Neutral,
        Emphasis::Muted,
        &compact_technical_console_summary(state, warning_count),
    );
    if let Some(hint) = final_console_hint(
        run_outcome.hint,
        run_outcome.report.console_status,
        &run_outcome.report.release_decision,
        warning_count,
    ) {
        launcher.line(
            PresentationRole::Result,
            None,
            Attention::Warning,
            Emphasis::Secondary,
            hint,
        );
    }
    launcher.line(
        PresentationRole::Result,
        None,
        Attention::Neutral,
        Emphasis::Muted,
        &format!("Result: {result} | Log: {CONSOLE_LOG}"),
    );
    Ok(run_outcome.severity)
}

fn print_log_tail(launcher: &mut Launcher, path: &Path, count: usize) -> AppResult<()> {
    if !path.exists() {
        return Ok(());
    }
    let text = fs::read_to_string(path).unwrap_or_default();
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(count);
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        "",
    );
    launcher.line(
        PresentationRole::Result,
        Some(LocalStatus::Failed),
        Attention::Error,
        Emphasis::Primary,
        &format!("--- letzte {count} Zeilen aus {} ---", path.display()),
    );
    for line in &lines[start..] {
        launcher.diagnostic_context_line(line);
    }
    launcher.diagnostic_context_line("--- Ende Logauszug ---");
    Ok(())
}

fn downloads_dir() -> AppResult<PathBuf> {
    let user_profile = env::var_os("USERPROFILE")
        .ok_or_else(|| anyhow!("USERPROFILE ist nicht gesetzt; Downloads-Ordner unbekannt."))?;
    let downloads = PathBuf::from(user_profile).join("Downloads");
    if !downloads.is_dir() {
        return Err(anyhow!("Downloads folder not found: {}", downloads.display()));
    }
    Ok(downloads)
}

fn parse_simple_field_value(text: &str, field: &str) -> Option<String> {
    let needle = format!("{field}:");
    text.lines()
        .find_map(|line| line.trim().strip_prefix(&needle).map(|value| value.trim().to_string()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VersionBoundPostTakeDecision {
    NoAction,
    ReconstructVerifiedPredecessorHandover,
    ForceStrictAll,
}

fn version_bound_post_take_decision(
    from_version: &str,
    to_version: &str,
    assistant_export_exists: bool,
    predecessor_evidence_valid: bool,
) -> VersionBoundPostTakeDecision {
    if from_version == CARRY_FORWARD_HANDOVER_FROM_VERSION
        && to_version == CARRY_FORWARD_HANDOVER_TO_VERSION
        && !assistant_export_exists
    {
        return if predecessor_evidence_valid {
            VersionBoundPostTakeDecision::ReconstructVerifiedPredecessorHandover
        } else {
            VersionBoundPostTakeDecision::ForceStrictAll
        };
    }
    if to_version == CORPUS_ACCEPTANCE_CONTRACT_VERSION {
        return VersionBoundPostTakeDecision::ForceStrictAll;
    }
    VersionBoundPostTakeDecision::NoAction
}

fn corpus_acceptance_stamp_path(project_root: &Path, version: &str) -> AppResult<PathBuf> {
    if version.is_empty()
        || !version
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
    {
        return Err(anyhow!(
            "Corpus-Acceptance-Version ist nicht strikt numerisch gebunden: {version:?}"
        ));
    }
    Ok(project_root
        .join(CORPUS_ACCEPTANCE_STAMP_DIR)
        .join(format!("accepted-{version}.stamp")))
}

fn corpus_acceptance_contract_is_satisfied(project_root: &Path, version: &str) -> bool {
    let Ok(path) = corpus_acceptance_stamp_path(project_root, version) else {
        return false;
    };
    let expected = format!("version={version}");
    fs::read_to_string(path)
        .map(|text| text.trim() == expected)
        .unwrap_or(false)
}

fn write_version_bound_corpus_acceptance_stamp(
    project_root: &Path,
    version: &str,
) -> AppResult<PathBuf> {
    let path = corpus_acceptance_stamp_path(project_root, version)?;
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Corpus-Acceptance-Stempel besitzt keinen Elternpfad"))?;
    fs::create_dir_all(parent)?;
    fs::write(&path, format!("version={version}\n"))?;
    if !corpus_acceptance_contract_is_satisfied(project_root, version) {
        return Err(anyhow!(
            "Corpus-Acceptance-Stempel konnte nicht versionsgebunden zurückgelesen werden"
        ));
    }
    Ok(path)
}

fn promote_post_take_args_to_strict_all(args: &mut ArgsState) {
    let inventory = args
        .execution()
        .map(|request| request.inventory)
        .unwrap_or(InventoryPlan::Disabled);
    args.request = RunRequest::Execute(ExecutionRequest {
        target: RunTarget::Corpus,
        program_args: vec![OsString::from("--all")],
        validation: ValidationPlan::Strict,
        build_preference: BuildPreference::ReuseVerified,
        launcher_tests: LauncherTestPlan::VisualPreflight,
        export_policy: AssistantExportPolicy::Minimal,
        watch: WatchPlan::Disabled,
        neuro_corpus_publish: false,
        inventory,
    });
}

fn execute_previous_corpus_handover_recovery(
    project_root: &Path,
    export_version: &str,
    evidence_version: &str,
) -> AppResult<PathBuf> {
    let validation = validate_corpus_run_manifest(project_root, evidence_version);
    if !validation.valid || !validation.manifest.full_corpus {
        return Err(anyhow!(
            "Versionsgebundener Vorgänger-Korpusbeleg ist nicht vollständig gültig: {}",
            validation.reason
        ));
    }
    let state = RunState {
        validation: ValidationPlan::Strict,
        build_outcome: BuildOutcome::Reused(CargoBuildProfile::Strict),
        thread_plan: ThreadPlan::detect(),
        target_exe: target_exe_for_profile("strict"),
        strict_clippy_exit: "0".to_string(),
        build_exit: "0".to_string(),
        strict_test_exit: "not_run".to_string(),
        run_exit: "0".to_string(),
        clippy_ms: "carried_forward".to_string(),
        build_ms: "carried_forward".to_string(),
        strict_test_ms: "not_run".to_string(),
        strict_test_summary: "not_run".to_string(),
        program_ms: "carried_forward".to_string(),
        total_start: Instant::now(),
        strict_log: Some(PathBuf::from("corpus_emergence.strict-clippy.txt")),
        strict_test_log: None,
        result_file: Some(PathBuf::from("corpus_emergence.result.txt")),
        input_arg: Some("--all".to_string()),
        force_mode: false,
        audit_signal: AuditSignal::NotEvaluated,
        renderer_outcome: RendererOutcome::NotObserved,
        emergence_outcome: EmergenceOutcome::NotApplicable,
        render_candidate_selected_pdf_truth: None,
        export_policy: AssistantExportPolicy::Minimal,
    };
    create_assistant_export(AssistantExportRequest::Corpus {
        project_root,
        state: &state,
        export_version,
        evidence_version,
        final_exit: 0,
    })
}

fn apply_version_bound_post_take_contract(
    project_root: &Path,
    args: &mut ArgsState,
) -> AppResult<()> {
    let (Ok(from_version), Ok(to_version)) = (
        env::var("ETUSCHLOPF_TAKE_FROM"),
        env::var("ETUSCHLOPF_TAKE_TO"),
    ) else {
        return Ok(());
    };

    let export_exists = project_root.join("assistant_export_latest.zip").is_file();
    let predecessor_validation =
        validate_corpus_run_manifest(project_root, CARRY_FORWARD_HANDOVER_FROM_VERSION);
    let predecessor_evidence_valid =
        predecessor_validation.valid && predecessor_validation.manifest.full_corpus;

    match version_bound_post_take_decision(
        &from_version,
        &to_version,
        export_exists,
        predecessor_evidence_valid,
    ) {
        VersionBoundPostTakeDecision::NoAction => Ok(()),
        VersionBoundPostTakeDecision::ReconstructVerifiedPredecessorHandover => {
            execute_previous_corpus_handover_recovery(
                project_root,
                &to_version,
                &from_version,
            )?;
            Ok(())
        }
        VersionBoundPostTakeDecision::ForceStrictAll => {
            promote_post_take_args_to_strict_all(args);
            Ok(())
        }
    }
}




#[cfg(test)]
mod accepted_closure_retirement_tests {
    #[test]
    fn historical_closure_markers_do_not_normalize_exit_two() {
        let source = include_str!("etuschlopf_launcher/launcher_visual_audit.rs");
        let gate_start = source
            .find("fn execute_visual_preflight_clef_audits(")
            .expect("typed visual-preflight gate must remain present");
        let gate_end = gate_start
            + source[gate_start..]
                .find("\nfn finalize_visual_preflight_execution(")
                .expect("typed visual-preflight gate must remain locally bounded");
        let gate_source = &source[gate_start..gate_end];

        assert!(gate_source.contains("execution.final_exit = 2"));
        assert!(!gate_source.contains("execution.final_exit = 0"));

        let compatibility_wrapper = ["fn normalize_visual_preflight_", "accepted_closure("].concat();
        assert!(
            !source.contains(&compatibility_wrapper),
            "retired temporary cross-domain compatibility wrapper must stay absent"
        );
        assert!(!source.contains(
            "etuschlopf_m18_renderer_exit_code_strictness_and_musescore_surface_audit"
        ));

        let retired_normalizer = [
            "normalize_visual_preflight_exit_after_",
            "accepted_register_pressure_closure",
        ]
        .concat();
        assert!(!source.contains(&retired_normalizer));
    }
}

#[cfg(test)]
mod fresh_run_gate_tests {
    use super::{
        source_build_signature, strict_all_targets_source_signature,
        strict_clippy_validation_receipt_material_from_basis,
        strict_test_validation_receipt_material_from_basis,
        visual_preflight_freshness_gate_green, write_clippy_warning_budget,
        CanonicalStrictValidationBasis,
    };

    #[test]
    fn carry_forward_handover_is_pinned_to_the_verified_predecessor() {
        use super::VersionBoundPostTakeDecision;
        assert_eq!(
            super::version_bound_post_take_decision("0.10.1265", "0.10.1266", false, true),
            VersionBoundPostTakeDecision::ReconstructVerifiedPredecessorHandover
        );
        assert_eq!(
            super::version_bound_post_take_decision("0.10.1264", "0.10.1266", false, true),
            VersionBoundPostTakeDecision::NoAction
        );
        assert_eq!(
            super::version_bound_post_take_decision("0.10.1265", "0.10.1267", false, true),
            VersionBoundPostTakeDecision::NoAction
        );
    }

    #[test]
    fn corpus_acceptance_contract_is_version_bound() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "etuschlopf-corpus-acceptance-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("temp root");
        super::write_version_bound_corpus_acceptance_stamp(&root, "0.10.1286")
            .expect("write exact stamp");
        assert!(super::corpus_acceptance_contract_is_satisfied(&root, "0.10.1286"));
        assert!(!super::corpus_acceptance_contract_is_satisfied(&root, "0.10.1287"));
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn post_take_promotion_discards_single_project_and_enforces_strict_all() {
        let mut args = super::parse_args(vec![
            std::ffi::OsString::from("--strict"),
            std::ffi::OsString::from("Synthetic Project Alpha"),
            std::ffi::OsString::from("--inventory"),
        ]);
        super::promote_post_take_args_to_strict_all(&mut args);
        assert_eq!(args.validation(), super::ValidationPlan::Strict);
        assert_eq!(
            args.build_preference(),
            super::BuildPreference::ReuseVerified
        );
        assert!(args.visual_preflight_requested());
        assert_eq!(args.input_arg(), Some("--all"));
        assert_eq!(args.program_args(), vec![std::ffi::OsString::from("--all")]);
        assert!(!args.watch_requested());
        assert!(args.inventory_requested());
    }

    #[test]
    fn current_same_version_take_is_not_unconditionally_promoted_to_full_corpus() {
        use super::VersionBoundPostTakeDecision;

        assert_eq!(
            super::version_bound_post_take_decision(
                super::VERSION,
                super::VERSION,
                true,
                true,
            ),
            VersionBoundPostTakeDecision::NoAction
        );
        assert_eq!(
            super::version_bound_post_take_decision(
                super::VERSION,
                super::VERSION,
                false,
                false,
            ),
            VersionBoundPostTakeDecision::NoAction
        );
    }

    #[test]
    fn recovery_precedes_console_log_reset_and_root_cleanup() {
        let source = include_str!("etuschlopf_launcher.rs");
        let run_start = source.find("fn run() -> AppResult<()>").expect("run function");
        let run_end = source[run_start..]
            .find("\nimpl Launcher")
            .map(|offset| run_start + offset)
            .expect("run end");
        let run = &source[run_start..run_end];
        let recovery = run
            .find("recover_pending_public_artifacts")
            .expect("public recovery");
        let log_reset = run.find("init_console_log").expect("console reset");
        let root_cleanup = run
            .find("cleanup_startup_project_root_residue")
            .expect("root cleanup");
        assert!(recovery < log_reset);
        assert!(recovery < root_cleanup);
    }

    #[test]
    fn corpus_acceptance_stamp_is_version_specific() {
        let root = std::path::Path::new("project");
        let a = super::corpus_acceptance_stamp_path(root, "0.10.1286")
            .expect("valid version");
        let b = super::corpus_acceptance_stamp_path(root, "0.10.1287")
            .expect("valid version");
        assert_ne!(a, b);
        assert!(a.to_string_lossy().contains("accepted-0.10.1286.stamp"));
        assert!(super::corpus_acceptance_stamp_path(root, "../0.10.1286").is_err());
    }

    #[test]
    fn root_hygiene_recognizes_known_residue() {
        for name in [
            ".etuschlopf_complete_09_lock_ready",
            ".etuschlopf_regenerate_lock_1131",
            "PACKAGE_COMPLETE_09.txt",
            "PAKET_COMPLETE_08.txt",
            "gould_register_island_audit.tsv",
            "v0.10.1336.launcher-tests.txt",
            "v0.10.1338.public-artifact-fault-audit.txt",
            "visual_preflight.strict-tests.txt",
            "desktop.ini",
            "files.txt",
        ] {
            assert!(super::is_stale_assistant_root_doc_name(name), "{name}");
        }
        assert!(!super::is_stale_assistant_root_doc_name("assistant_export_latest.zip"));
    }

    #[test]
    fn all_run_branch_writes_corpus_handover_before_exit() {
        let source = include_str!("etuschlopf_launcher.rs");
        let function_start = source
            .find("fn execute_corpus_target(")
            .expect("typed corpus target stage");
        let function_end = source[function_start + 1..]
            .find("\nfn execute_project_target(")
            .map(|offset| function_start + 1 + offset)
            .expect("next target stage");
        let function = &source[function_start..function_end];
        let export_call = function
            .find("create_assistant_export(AssistantExportRequest::Corpus")
            .expect("corpus export request");
        let requested_exit = function
            .rfind("launcher_exit(program_result.exit_code)")
            .expect("all-run requested exit");
        assert!(export_call < requested_exit);
    }

    #[test]
    fn corpus_handover_derives_project_results_from_manifest() {
        let source = include_str!("etuschlopf_launcher/assistant_export.rs");
        let function_start = source
            .find("fn create_corpus_run_assistant_export_bundle")
            .expect("corpus export function");
        let next_function = source[function_start + 1..]
            .find("\nfn create_visual_preflight_assistant_export_bundle")
            .map(|offset| function_start + 1 + offset)
            .expect("next export function");
        let function = &source[function_start..next_function];
        assert!(function.contains("for project in &validation.manifest.projects"));
        assert!(function.contains("project.result_file"));
        assert!(!function.contains("Etüde Nr."));
        assert!(function.contains("Mode: corpus_run"));

        let helper_start = source
            .find("fn copy_manifest_bound_corpus_score_artifacts")
            .expect("manifest-bound corpus artifact helper");
        let helper_end = source[helper_start + 1..]
            .find("\nfn write_renderer_process_lifecycle_assistant_audit")
            .map(|offset| helper_start + 1 + offset)
            .expect("next helper");
        let helper = &source[helper_start..helper_end];
        assert!(helper.contains("read_corpus_run_manifest"));
        assert!(helper.contains("project.result_file"));
        assert!(helper.contains("resolve_musicxml_file(&project.project)"));
        assert!(helper.contains("resolve_render_musicxml_file(&project.project)"));
        assert!(helper.contains("resolve_pdf_file(&project.project)"));
        assert!(!helper.contains("Etüde Nr."));
    }

    #[test]
    fn normal_run_assistant_export_keeps_score_outputs_and_projects_compact_result() {
        let project = "Synthetic Project Alpha";
        let paths = super::normal_run_assistant_export_project_paths(project);
        assert_eq!(paths.len(), 3);
        assert!(!paths.contains(&std::path::PathBuf::from(format!("{project}.result.txt"))));
        assert!(paths.contains(&std::path::PathBuf::from(format!("{project}.musicxml"))));
        assert!(paths.contains(&std::path::PathBuf::from(format!("{project}.render.musicxml"))));
        assert!(paths.contains(&std::path::PathBuf::from(format!("{project}.pdf"))));

        let source = include_str!("etuschlopf_launcher/assistant_export.rs");
        let start = source
            .find("fn create_normal_run_assistant_export_bundle")
            .expect("normal-run export owner");
        let end = source[start + 1..]
            .find("\nfn create_corpus_run_assistant_export_bundle")
            .map(|offset| start + 1 + offset)
            .expect("next export owner");
        let normal_run = &source[start..end];
        assert!(normal_run.contains("write_assistant_project_result_summary"));
        assert!(normal_run.contains("PROJECT_FILE_CATALOG_TSV.to_string()"));
        assert!(!normal_run.contains("included.push(PROJECT_FILE_CATALOG_MARKDOWN.to_string())"));
        assert!(!normal_run.contains("write_transition_history_projection(project_root, &latest_dir)"));
        assert!(normal_run.contains("pre_acceptance_advisory_history_remains_local_normal_run"));
    }

    #[test]
    fn project_folder_catalog_recursively_records_all_entries_and_planned_handover_outputs() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_project_folder_catalog_{}_{}",
            std::process::id(),
            unique
        ));
        let latest = root.join("target/etuschlopf_assistant_export/latest");
        std::fs::create_dir_all(root.join("src/nested")).expect("create source tree");
        std::fs::create_dir_all(root.join("docs")).expect("create docs");
        std::fs::create_dir_all(&latest).expect("create latest");
        std::fs::write(root.join("Cargo.toml"), "[package]\nname=\"x\"\n").expect("cargo");
        std::fs::write(root.join("src/nested/module.rs"), "pub fn value() {}\n")
            .expect("source");
        std::fs::write(root.join("docs/info.txt"), "documentation\n").expect("docs");
        std::fs::write(latest.join("run_outcome_audit.txt"), "green\n").expect("staged");

        let summary =
            super::write_project_folder_catalog(&root, &latest, "0.10.test").expect("catalog");
        let tsv = std::fs::read_to_string(latest.join(super::PROJECT_FILE_CATALOG_TSV))
            .expect("read tsv");
        let markdown =
            std::fs::read_to_string(latest.join(super::PROJECT_FILE_CATALOG_MARKDOWN))
                .expect("read markdown");

        assert!(summary.entry_count >= 13);
        assert!(summary.file_count >= 9);
        assert!(tsv.contains("src/nested/module.rs\tfile"));
        assert!(tsv.contains("docs/info.txt\tfile"));
        assert!(tsv.contains("target/etuschlopf_assistant_export/latest/run_outcome_audit.txt"));
        assert!(tsv.contains("target/etuschlopf_assistant_export/latest/PROJECT_FILE_CATALOG.md"));
        assert!(tsv.contains("target/etuschlopf_assistant_export/latest/project_file_catalog.tsv"));
        assert!(tsv.contains("assistant_export_latest.zip\tfile\t0\thandover"));
        assert!(tsv.contains("planned_output_after_snapshot"));
        assert!(markdown.contains("EveryReadableEntryRecorded: `yes`"));
        assert!(markdown.contains("OmittedPaths: `none; an unreadable entry aborts export generation`"));

        let bounded = super::write_normal_run_project_folder_catalog(&root, &latest, "0.10.test")
            .expect("bounded catalog");
        let bounded_tsv = std::fs::read_to_string(latest.join(super::PROJECT_FILE_CATALOG_TSV))
            .expect("read bounded tsv");
        assert!(bounded.entry_count < summary.entry_count);
        assert!(bounded_tsv.contains("generated_descendants_excluded"));
        assert!(!bounded_tsv.contains("target/etuschlopf_assistant_export/latest/run_outcome_audit.txt"));
        assert!(!bounded_tsv.contains("target/etuschlopf_assistant_export/latest/PROJECT_FILE_CATALOG.md"));
        assert!(!latest.join(super::PROJECT_FILE_CATALOG_MARKDOWN).exists());

        std::fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn assistant_export_required_files_include_complete_project_folder_catalog() {
        let source = include_str!("etuschlopf_launcher/assistant_export.rs");
        let required_files = ["fn assistant_context_required_root_", "files_for_ledgers"].concat();
        assert!(source.contains(&required_files));
        assert!(source.contains("PROJECT_FILE_CATALOG_MARKDOWN.to_string()"));
        assert!(source.contains("PROJECT_FILE_CATALOG_TSV.to_string()"));
    }

    #[test]
    fn request_model_closes_boolean_mode_combinations() {
        let source = include_str!("etuschlopf_launcher.rs");
        assert!(source.contains("enum RunRequest"));
        assert!(source.contains("enum RunTarget"));
        assert!(source.contains("enum BuildPreference"));
        assert!(source.contains("enum LauncherTestPlan"));
        for retired_field in [
            ["take_mode", ": bool"].concat(),
            ["force", ": bool"].concat(),
            ["visual_preflight_gate", ": bool"].concat(),
            ["assistant_export_pdf", ": bool"].concat(),
        ] {
            assert!(!source.contains(&retired_field));
        }
    }

    #[test]
    fn assistant_export_policy_is_typed_without_process_environment_mutation() {
        let launcher = include_str!("etuschlopf_launcher.rs");
        let export = include_str!("etuschlopf_launcher/assistant_export.rs");
        assert!(launcher.contains("export_policy: AssistantExportPolicy"));
        assert!(launcher.contains("export_policy: request.export_policy"));
        assert!(!launcher.contains("env::set_var(\"ETUSCHLOPF_ASSISTANT_EXPORT_PDF\""));
        assert!(!export.contains("ETUSCHLOPF_ASSISTANT_EXPORT_PDF"));
    }

    #[test]
    fn normal_run_is_a_linear_stage_coordinator() {
        let source = include_str!("etuschlopf_launcher.rs");
        let start = source.find("fn execute_normal_run(").expect("normal run stage");
        let end = source[start + 1..]
            .find("\nfn initialize_run_state(")
            .map(|offset| start + 1 + offset)
            .expect("next stage");
        let function = &source[start..end];
        let initialize = function.find("initialize_run_state").expect("initialize");
        let build = function.find("materialize_launcher_build").expect("build");
        let tests = function.find("run_integrated_launcher_tests").expect("tests");
        let dispatch = function.find("match &request.target").expect("dispatch");
        assert!(initialize < build && build < tests && tests < dispatch);
        assert!(function.lines().count() < 45);
    }

    #[test]
    fn assistant_export_has_one_typed_creation_boundary() {
        let source = include_str!("etuschlopf_launcher/assistant_export.rs");
        assert!(source.contains("enum AssistantExportRequest"));
        assert!(source.contains("pub(super) fn create_assistant_export("));
        assert!(!source.contains("pub(super) fn create_normal_run_assistant_export_bundle("));
        assert!(!source.contains("pub(super) fn create_corpus_run_assistant_export_bundle("));
        assert!(!source.contains("pub(super) fn create_visual_preflight_assistant_export_bundle("));
    }

    #[test]
    fn cleanup_reports_preserve_path_bound_failures() {
        let source = [
            include_str!("etuschlopf_launcher.rs"),
            include_str!("etuschlopf_launcher/launcher_hygiene.rs"),
        ]
        .join("\n");
        assert!(source.contains("enum CachePathOutcome"));
        assert!(source.contains("CachePathOutcome::Failed(String)"));
        assert!(source.contains("struct CleanupFailure"));
        assert!(source.contains("FailurePath: {}; Error: {}"));
        let retired_cache_flag = ["bootstrap_incremental_deleted", ": bool"].concat();
        assert!(!source.contains(&retired_cache_flag));
    }

    #[test]
    fn one_command_contract_runs_launcher_tests_only_for_strict_visual_preflight() {
        assert!(super::should_run_integrated_launcher_tests(
            super::ValidationPlan::Strict,
            super::LauncherTestPlan::VisualPreflight,
        ));
        assert!(!super::should_run_integrated_launcher_tests(
            super::ValidationPlan::DeepClippy,
            super::LauncherTestPlan::VisualPreflight,
        ));
        assert!(!super::should_run_integrated_launcher_tests(
            super::ValidationPlan::Strict,
            super::LauncherTestPlan::Standard,
        ));
    }

    #[test]
    fn public_project_without_strict_flag_defaults_to_strict_clippy_basis() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![OsString::from("Synthetic Project Alpha")]);
        assert_eq!(args.validation(), super::ValidationPlan::Strict);
        assert!(args.visual_preflight_requested());
        assert_eq!(
            args.build_preference(),
            super::BuildPreference::ReuseVerified
        );
        assert_eq!(args.program_args(), vec![OsString::from("Synthetic Project Alpha")]);
    }

    #[test]
    fn explicit_strict_flag_is_idempotent_backwards_compatibility_only() {
        use std::ffi::OsString;

        let implicit = super::parse_args(vec![OsString::from("Synthetic Project Alpha")]);
        let explicit = super::parse_args(vec![
            OsString::from("--strict"),
            OsString::from("Synthetic Project Alpha"),
        ]);
        assert_eq!(implicit.validation(), explicit.validation());
        assert_eq!(implicit.program_args(), explicit.program_args());
        assert_eq!(
            implicit.visual_preflight_requested(),
            explicit.visual_preflight_requested()
        );
    }

    #[test]
    fn verified_build_reuse_never_skips_current_strict_clippy() {
        let source = include_str!("etuschlopf_launcher/launcher_build_control.rs");
        let reuse_start = source
            .find("fn reuse_verified_build(")
            .expect("reuse function");
        let reuse_end = source[reuse_start..]
            .find("\nfn run_or_reuse_strict_clippy(")
            .map(|offset| reuse_start + offset)
            .expect("reuse function end");
        let reuse = &source[reuse_start..reuse_end];
        assert!(reuse.contains("run_or_reuse_strict_clippy(launcher, state, strict_log_path)?;"));
        assert!(!reuse.contains("write_skipped_strict_log"));
        assert!(!source.contains("fn write_skipped_strict_log("));
    }

    #[test]
    fn strict_all_targets_receipt_is_exact_and_fail_closed() {
        let source = include_str!("etuschlopf_launcher.rs");
        assert!(source.contains("STRICT_TEST_VALIDATION_RECEIPT"));
        assert!(source.contains("SourceSignature:"));
        assert!(source.contains("StrictAllTargetsSourceSignature:"));
        assert!(source.contains("StrictBinaryFNV:"));
        assert!(source.contains("RustcIdentity:"));
        assert!(source.contains("CargoIdentity:"));
        assert!(source.contains("TestContract:"));
        assert!(source.contains("Err(_) => return false"));
        assert!(source.contains("reused_exact_validation_receipt"));
    }

    #[test]
    fn strict_all_targets_receipt_reuse_does_not_relax_current_clippy() {
        let build_control = include_str!("etuschlopf_launcher/launcher_build_control.rs");
        let launcher = include_str!("etuschlopf_launcher.rs");
        assert!(launcher.contains("STRICT_CLIPPY_VALIDATION_RECEIPT"));
        assert!(launcher.contains("ClippyIdentity:"));
        assert!(launcher.contains("ClippyContract:"));
        assert!(launcher.contains("strict_clippy_validation_receipt_matches"));
        assert!(build_control.contains("warning_count == 0"));
        assert!(build_control.contains("write_strict_clippy_validation_receipt"));
        assert!(build_control.contains("run_strict_clippy(launcher, state, strict_log_path)?;"));
    }

    #[test]
    fn strict_test_and_clippy_receipts_are_independent_contracts() {
        let launcher = include_str!("etuschlopf_launcher.rs");
        assert!(launcher.contains("STRICT_TEST_VALIDATION_CONTRACT"));
        assert!(launcher.contains("STRICT_CLIPPY_VALIDATION_CONTRACT"));
        assert_ne!(super::STRICT_TEST_VALIDATION_RECEIPT, super::STRICT_CLIPPY_VALIDATION_RECEIPT);
    }

    fn validation_basis_fixture() -> CanonicalStrictValidationBasis {
        CanonicalStrictValidationBasis {
            version: "0.10.test".to_string(),
            source_signature: "source-a".to_string(),
            rustc_identity: "rustc-a".to_string(),
            cargo_identity: "cargo-a".to_string(),
        }
    }

    #[test]
    fn strict_validation_external_specialized_dimensions_are_independent() {
        let basis = validation_basis_fixture();
        let clippy_before =
            strict_clippy_validation_receipt_material_from_basis(&basis, "clippy-a");
        let clippy_after =
            strict_clippy_validation_receipt_material_from_basis(&basis, "clippy-b");
        let test_before = strict_test_validation_receipt_material_from_basis(
            &basis,
            "test-universe-a",
            "fnv64:000000000000000b",
        );
        let test_after_clippy_change = strict_test_validation_receipt_material_from_basis(
            &basis,
            "test-universe-a",
            "fnv64:000000000000000b",
        );
        assert_ne!(clippy_before, clippy_after);
        assert_eq!(test_before, test_after_clippy_change);

        let test_after_binary_change = strict_test_validation_receipt_material_from_basis(
            &basis,
            "test-universe-a",
            "fnv64:000000000000000c",
        );
        let clippy_after_binary_change =
            strict_clippy_validation_receipt_material_from_basis(&basis, "clippy-a");
        assert_ne!(test_before, test_after_binary_change);
        assert_eq!(clippy_before, clippy_after_binary_change);

        let test_after_universe_change = strict_test_validation_receipt_material_from_basis(
            &basis,
            "test-universe-b",
            "fnv64:000000000000000b",
        );
        let clippy_after_universe_change =
            strict_clippy_validation_receipt_material_from_basis(&basis, "clippy-a");
        assert_ne!(test_before, test_after_universe_change);
        assert_eq!(clippy_before, clippy_after_universe_change);
    }

    #[test]
    fn strict_all_targets_source_signature_binds_tests_and_fixtures_without_invalidating_product_source(
    ) {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "etuschlopf-r1110-test-universe-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(root.join("src")).expect("src fixture");
        std::fs::create_dir_all(root.join("tests/fixtures/synthetic/")).expect("test fixture dir");
        std::fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"receipt-probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\n",
        )
        .expect("Cargo.toml fixture");
        std::fs::write(root.join("src/lib.rs"), "pub fn stable() -> u8 { 1 }\n")
            .expect("source fixture");
        std::fs::write(root.join("tests/probe.rs"), "#[test] fn probe() {}\n")
            .expect("test source fixture");
        std::fs::write(root.join("tests/fixtures/synthetic/probe.xml"), "<score a=\"1\"/>\n")
            .expect("test data fixture");

        let product_before = source_build_signature(&root).expect("product signature before");
        let tests_before =
            strict_all_targets_source_signature(&root).expect("all-target signature before");

        std::fs::write(root.join("tests/probe.rs"), "#[test] fn probe_changed() {}\n")
            .expect("changed test source fixture");
        let product_after_test =
            source_build_signature(&root).expect("product signature after test change");
        let tests_after_test = strict_all_targets_source_signature(&root)
            .expect("all-target signature after test change");
        assert_eq!(product_before, product_after_test);
        assert_ne!(tests_before, tests_after_test);

        std::fs::write(root.join("tests/fixtures/synthetic/probe.xml"), "<score a=\"2\"/>\n")
            .expect("changed test data fixture");
        let product_after_fixture =
            source_build_signature(&root).expect("product signature after fixture change");
        let tests_after_fixture = strict_all_targets_source_signature(&root)
            .expect("all-target signature after fixture change");
        assert_eq!(product_before, product_after_fixture);
        assert_ne!(tests_after_test, tests_after_fixture);

        for relative in [
            "docs/synthetic/contract.txt",
            "tools/synthetic/contract.txt",
            "src/synthetic_contract.txt",
            "run.cmd",
            "README.md",
            "examples/synthetic/data.txt",
            "benches/synthetic/data.txt",
        ] {
            let input = root.join(relative);
            std::fs::create_dir_all(input.parent().expect("fixture parent"))
                .expect("fixture directory");
            let absent = strict_all_targets_source_signature(&root).expect("absent signature");
            std::fs::write(&input, "allowed").expect("external fixture");
            let present = strict_all_targets_source_signature(&root).expect("present signature");
            assert_ne!(absent, present);
            assert_eq!(present, strict_all_targets_source_signature(&root).expect("stable signature"));
            // The test's observed contract changes without changing its source.
            assert_eq!(std::fs::read_to_string(&input).expect("observed contract"), "allowed");
            std::fs::write(&input, "blocked").expect("changed external fixture");
            assert_ne!(std::fs::read_to_string(&input).expect("observed contract"), "allowed");
            let changed = strict_all_targets_source_signature(&root).expect("changed signature");
            assert_ne!(present, changed);
            assert_eq!(product_before, source_build_signature(&root).expect("stable product"));
            let renamed = input.with_extension("renamed");
            std::fs::rename(&input, &renamed).expect("rename fixture");
            assert_ne!(changed, strict_all_targets_source_signature(&root).expect("renamed signature"));
            std::fs::remove_file(&renamed).expect("remove renamed fixture");
            assert_eq!(absent, strict_all_targets_source_signature(&root).expect("removed signature"));
        }
        std::fs::create_dir(root.join("run.cmd")).expect("invalid root file fixture");
        assert!(strict_all_targets_source_signature(&root).is_err());
        std::fs::remove_dir(root.join("run.cmd")).expect("remove invalid root file");
        std::fs::remove_dir_all(root.join("docs")).expect("remove docs fixture");
        std::fs::write(root.join("docs"), "not a directory").expect("invalid directory fixture");
        assert!(strict_all_targets_source_signature(&root).is_err());

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn strict_test_receipt_requires_stable_material_and_invalidates_previous_proof() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "etuschlopf-receipt-material-{}-{unique}", std::process::id()
        ));
        let path = root.join("proof.receipt");
        let basis = validation_basis_fixture();
        let before = strict_test_validation_receipt_material_from_basis(
            &basis, "repository-a", "binary-a",
        );
        super::write_strict_test_validation_receipt(&path, Some((&before, &before)))
            .expect("stable receipt");
        assert_eq!(std::fs::read_to_string(&path).expect("exact receipt"), before);
        super::write_strict_test_validation_receipt(&path, None).expect("pre-run invalidation");
        assert!(!path.exists());
        // Failed test execution never calls the success writer: proof stays absent.
        let changed = strict_test_validation_receipt_material_from_basis(
            &basis, "repository-b", "binary-a",
        );
        super::write_strict_test_validation_receipt(&path, Some((&before, &before)))
            .expect("old proof");
        assert!(super::write_strict_test_validation_receipt(&path, Some((&before, &changed))).is_err());
        assert!(!path.exists());
        let changed_binary = strict_test_validation_receipt_material_from_basis(
            &basis, "repository-a", "binary-b",
        );
        assert!(super::write_strict_test_validation_receipt(&path, Some((&before, &changed_binary))).is_err());
        assert!(!path.exists());
        std::fs::create_dir(&path).expect("unremovable receipt fixture");
        assert!(super::write_strict_test_validation_receipt(&path, None).is_err());
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn strict_validation_basis_changes_invalidate_both_proofs() {
        let basis = validation_basis_fixture();
        let clippy_before =
            strict_clippy_validation_receipt_material_from_basis(&basis, "clippy-a");
        let test_before =
            strict_test_validation_receipt_material_from_basis(
                &basis,
                "test-universe-a",
                "fnv64:000000000000000b",
            );

        let mut version_changed = basis.clone();
        version_changed.version.push_str("-changed");
        assert_ne!(
            clippy_before,
            strict_clippy_validation_receipt_material_from_basis(&version_changed, "clippy-a")
        );
        assert_ne!(
            test_before,
            strict_test_validation_receipt_material_from_basis(
                &version_changed,
                "test-universe-a",
                "fnv64:000000000000000b",
            )
        );

        let mut source_changed = basis.clone();
        source_changed.source_signature.push_str("-changed");
        assert_ne!(
            clippy_before,
            strict_clippy_validation_receipt_material_from_basis(&source_changed, "clippy-a")
        );
        assert_ne!(
            test_before,
            strict_test_validation_receipt_material_from_basis(
                &source_changed,
                "test-universe-a",
                "fnv64:000000000000000b",
            )
        );

        let mut rustc_changed = basis.clone();
        rustc_changed.rustc_identity.push_str("-changed");
        assert_ne!(
            clippy_before,
            strict_clippy_validation_receipt_material_from_basis(&rustc_changed, "clippy-a")
        );
        assert_ne!(
            test_before,
            strict_test_validation_receipt_material_from_basis(
                &rustc_changed,
                "test-universe-a",
                "fnv64:000000000000000b",
            )
        );

        let mut cargo_changed = basis;
        cargo_changed.cargo_identity.push_str("-changed");
        assert_ne!(
            clippy_before,
            strict_clippy_validation_receipt_material_from_basis(&cargo_changed, "clippy-a")
        );
        assert_ne!(
            test_before,
            strict_test_validation_receipt_material_from_basis(
                &cargo_changed,
                "test-universe-a",
                "fnv64:000000000000000b",
            )
        );
    }

    #[test]
    fn strict_clippy_warning_count_excludes_only_cargo_summary() {
        let text = "warning: first\nwarning: `etuschlopf` generated 1 warning\n";
        assert_eq!(super::warning_diagnostic_count(text), 1);
        assert_eq!(super::warning_diagnostic_count(""), 0);
    }

    #[test]
    fn strict_clippy_warning_budget_rejects_non_utf8_log() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "etuschlopf-r937-clippy-invalid-utf8-{}-{unique}.log",
            std::process::id()
        ));
        std::fs::write(&path, [0xff, 0xfe, 0xfd]).expect("write invalid utf8 fixture");
        let result = write_clippy_warning_budget(&path, 0);
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err(), "invalid UTF-8 must never become zero warnings");
    }

    #[test]
    fn explicit_project_flag_is_unambiguous_and_forwarded() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--strict"),
            OsString::from("--project"),
            OsString::from("Synthetic Project Beta"),
            OsString::from("--writer-trace=5"),
        ]);

        assert_eq!(args.input_arg(), Some("Synthetic Project Beta"));
        assert_eq!(
            args.program_args(),
            vec![
                OsString::from("Synthetic Project Beta"),
                OsString::from("--writer-trace=5"),
            ]
        );
        assert_eq!(args.validation(), super::ValidationPlan::Strict);
        assert!(args.visual_preflight_requested());
    }

    #[test]
    fn explicit_project_equals_form_is_unambiguous() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--project=Synthetic Project Beta"),
            OsString::from("--writer-trace=5"),
        ]);

        assert_eq!(args.input_arg(), Some("Synthetic Project Beta"));
        assert_eq!(
            args.program_args(),
            vec![
                OsString::from("Synthetic Project Beta"),
                OsString::from("--writer-trace=5"),
            ]
        );
    }

    #[test]
    fn option_value_before_explicit_project_is_not_misread_as_project() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--jobs"),
            OsString::from("4"),
            OsString::from("--project"),
            OsString::from("Synthetic Project Beta"),
        ]);

        assert_eq!(args.input_arg(), Some("Synthetic Project Beta"));
    }

    #[test]
    fn strict_explicit_project_keeps_public_transaction_eligibility() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--strict"),
            OsString::from("--project"),
            OsString::from("Synthetic Project Beta"),
        ]);

        assert!(super::is_public_artifact_transaction_eligible(&args));
    }

    #[test]
    fn watch_is_launcher_owned_and_not_forwarded_to_music_binary() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--watch"),
            OsString::from("--strict"),
            OsString::from("Synthetic Project Alpha"),
        ]);

        assert!(args.watch_requested());
        assert_eq!(args.input_arg(), Some("Synthetic Project Alpha"));
        assert_eq!(args.program_args(), vec![OsString::from("Synthetic Project Alpha")]);
        assert_eq!(args.validation(), super::ValidationPlan::Strict);
        assert!(args.visual_preflight_requested());
        assert!(super::is_public_artifact_transaction_eligible(&args));
    }

    #[test]
    fn show_and_display_are_launcher_owned_and_not_forwarded_to_music_binary() {
        use std::ffi::OsString;

        for ui_arg in ["--show", "--display"] {
            let args = super::parse_args(vec![
                OsString::from(ui_arg),
                OsString::from("--strict"),
                OsString::from("Synthetic Project Alpha"),
            ]);

            assert!(args.watch_requested(), "{ui_arg} must enable the Watch UI");
            assert_eq!(args.input_arg(), Some("Synthetic Project Alpha"));
            assert_eq!(args.program_args(), vec![OsString::from("Synthetic Project Alpha")]);
            assert_eq!(args.validation(), super::ValidationPlan::Strict);
            assert!(args.visual_preflight_requested());
            assert!(super::is_public_artifact_transaction_eligible(&args));
        }
    }

    #[test]
    fn strict_all_integrates_visual_preflight_without_extra_argument() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--strict"),
            OsString::from("--all"),
            OsString::from("--display"),
            OsString::from("--inventory"),
        ]);

        assert_eq!(args.input_arg(), Some("--all"));
        assert_eq!(args.validation(), super::ValidationPlan::Strict);
        assert!(args.visual_preflight_requested());
        assert!(args.inventory_requested());
        assert_eq!(args.program_args(), vec![OsString::from("--all")]);
    }

    #[test]
    fn explicit_project_wins_over_visual_preflight_launcher_mode() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--force"),
            OsString::from("--strict"),
            OsString::from("Synthetic Project Alpha"),
            OsString::from("--visual-preflight"),
        ]);

        assert_eq!(args.input_arg(), Some("Synthetic Project Alpha"));
        assert_eq!(args.program_args(), vec![OsString::from("Synthetic Project Alpha")]);
        assert!(args.visual_preflight_requested());
        assert!(!super::is_visual_preflight_launcher_mode(&args));
    }

    #[test]
    fn standalone_visual_preflight_remains_its_own_launcher_mode() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--strict"),
            OsString::from("--visual-preflight"),
        ]);

        assert_eq!(args.input_arg(), Some("--visual-preflight"));
        assert!(super::is_visual_preflight_launcher_mode(&args));
    }

    #[test]
    fn explicit_all_wins_over_visual_preflight_but_stays_multi_project() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--all"),
            OsString::from("--visual-preflight"),
        ]);

        assert_eq!(args.input_arg(), Some("--all"));
        assert!(!super::is_visual_preflight_launcher_mode(&args));
        assert!(args.input_arg().is_some_and(super::is_all_arg));
    }

    #[test]
    fn legacy_visual_preflight_arg_does_not_create_a_non_strict_test_plan() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![
            OsString::from("--all"),
            OsString::from("--visual-preflight"),
        ]);

        assert_eq!(args.input_arg(), Some("--all"));
        assert_eq!(args.validation(), super::ValidationPlan::Strict);
        assert!(args.visual_preflight_requested());
        assert!(!super::is_visual_preflight_launcher_mode(&args));
        assert!(args.input_arg().is_some_and(super::is_all_arg));
        assert_eq!(args.program_args(), vec![OsString::from("--all")]);
    }

    #[test]
    fn progress_completion_never_labels_nonzero_exit_as_ok() {
        let line = super::progress_done_plain(
            "0.10.test",
            super::ProgressCompletion {
                role: super::PresentationRole::Program,
                label: "Programm/PDF",
                elapsed: std::time::Duration::from_secs(3),
                detail: super::CompletionDetail::LogSize(1024),
                exit_code: 2,
            },
        );
        assert!(line.contains("Programm/PDF ENDE exit 2"));
        assert!(!line.contains("Programm/PDF OK"));

        let ok_line = super::progress_done_plain(
            "0.10.test",
            super::ProgressCompletion {
                role: super::PresentationRole::Program,
                label: "Programm/PDF",
                elapsed: std::time::Duration::from_secs(3),
                detail: super::CompletionDetail::LogSize(1024),
                exit_code: 0,
            },
        );
        assert!(ok_line.contains("Programm/PDF OK"));
    }

    #[test]
    fn normal_result_binds_only_confirmed_exit_two_to_audit_blocked() {
        assert!(super::normal_result_confirms_controlled_audit_block(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: BLOCKIERT\nBlocker: 1 | Warnungen: 0 | Hinweise: 0\n"
        ));
        assert!(super::normal_result_confirms_controlled_audit_block(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: BLOCKIERT: FinalRestClosureByInvariantProof\nFINAL-PDF-GROUNDTRUTH-REST-ARITHMETIC-GATE\nFinalRestClosureByInvariantProof BLOCKIERT; BehindBarsRuleGreen nein.\n"
        ));
        assert!(!super::normal_result_confirms_controlled_audit_block(
            "ETUSCHLOPF-ERGEBNIS\nErgebnis: OK\nBlocker: 0 | Warnungen: 0 | Hinweise: 0\n"
        ));
    }

    #[test]
    fn normal_result_binds_only_explicit_silent_voice_renderer_outcome() {
        assert!(super::normal_result_confirms_renderer_outcome_observed(
            "SILENT-VOICE-COVERAGE-WRITE-OUTCOME\n-----------------------------------\nSource: existing_writer_audit_pipeline_line.\nRendererOutcome: observed_from_existing_writer_audit.\n\nRUN-COMMAND-DIAGNOSTIC\n"
        ));
        assert!(!super::normal_result_confirms_renderer_outcome_observed(
            "RendererOutcome: observed_from_existing_writer_audit.\n"
        ));
        assert!(!super::normal_result_confirms_renderer_outcome_observed(
            "SILENT-VOICE-COVERAGE-WRITE-OUTCOME\n-----------------------------------\nRendererOutcome: not_observed.\n"
        ));
    }

    #[test]
    fn normal_result_projects_output_valid_renderer_process_failure() {
        use super::run_outcome::RendererOutcome;

        let result = "SILENT-VOICE-COVERAGE-WRITE-OUTCOME\n-----------------------------------\nSource: existing_writer_audit_pipeline_line.\nRendererOutcome: observed_from_existing_writer_audit.\n\nPDF: C:\\tmp\\Synthetic Project Alpha.pdf wurde erzeugt, aber Renderer-ExitCode ist -1073741819. Renderer: 'MuseScore4.exe' (Windows-Standardpfad). Aufruf: -f -o <pdf> <input>.\n\nRUN-COMMAND-DIAGNOSTIC\n";
        assert_eq!(
            super::normal_result_renderer_outcome(result),
            RendererOutcome::OutputValidProcessFailed(-1_073_741_819)
        );
    }

    #[test]
    fn normal_result_projects_confirmed_renderer_success() {
        use super::run_outcome::RendererOutcome;

        let result = "SILENT-VOICE-COVERAGE-WRITE-OUTCOME\n-----------------------------------\nSource: existing_writer_audit_pipeline_line.\nRendererOutcome: observed_from_existing_writer_audit.\n\nPDF: C:\\tmp\\Synthetic Project Alpha.pdf geschrieben. Renderer: 'MuseScore4.exe' (Windows-Standardpfad). Aufruf: -f -o <pdf> <input>. ExitCode 0.\n\nRUN-COMMAND-DIAGNOSTIC\n";
        assert_eq!(
            super::normal_result_renderer_outcome(result),
            RendererOutcome::Passed
        );
    }

    #[test]
    fn normal_result_does_not_invent_renderer_outcome_from_partial_evidence() {
        use super::run_outcome::RendererOutcome;

        assert_eq!(
            super::normal_result_renderer_outcome(
                "PDF: C:\\tmp\\Synthetic Project Alpha.pdf wurde erzeugt, aber Renderer-ExitCode ist -1073741819. Renderer: 'MuseScore4.exe' (Windows-Standardpfad). Aufruf: -f.\n"
            ),
            RendererOutcome::NotObserved
        );
        assert_eq!(
            super::normal_result_renderer_outcome(
                "SILENT-VOICE-COVERAGE-WRITE-OUTCOME\n-----------------------------------\nRendererOutcome: observed_from_existing_writer_audit.\n"
            ),
            RendererOutcome::NotObserved
        );
    }

    #[test]
    fn render_candidate_truth_parses_rejected_candidate_and_master_fallback() {
        let result = "RENDER-WRITE-PDF-SOURCE-ADOPTION-TRUTH-GATE\n\
------------------------------------------------\n\
Kurz: Writer BeatAlignedCrossStaffTailBeamRepair; PdfRequested ja; WriteProduced ja; WriteValidated nein; PdfSourceAdopted nein; ProductiveMaterializationGreen nein; ReleaseBlocker ja; Urteil render_write_produced_but_quarantined_raw_cursor_red.\n\
RawCursorGuard: checked ja; green nein; Measures 44; HardViolations 1; MaxOverflow 0; PartialBackups 1; PostExpectedEvents 0.\n\
PdfSource: adopted nein; label MusicXML.\n";
        let truth = super::render_candidate_selected_pdf_truth(result).expect("truth");
        assert!(truth.write_produced);
        assert!(!truth.write_validated);
        assert_eq!(truth.selected_pdf_source, "Master-MusicXML");
        assert_eq!(truth.selected_pdf_source_label, "MusicXML");
        assert!(!truth.pdf_source_adopted);
        assert!(!truth.productive_materialization_green);
        assert_eq!(
            truth.candidate_verdict,
            "render_write_produced_but_quarantined_raw_cursor_red"
        );
    }

    #[test]
    fn no_write_truth_exports_nonmaterialized_without_false_adoption() {
        let result = "RENDER-WRITE-PDF-SOURCE-ADOPTION-TRUTH-GATE
\
------------------------------------------------
\
Kurz: Writer BeatAlignedCrossStaffTailBeamRepair; PdfRequested ja; WriteProduced nein; WriteValidated nein; PdfSourceAdopted nein; ProductiveMaterializationGreen nein; ReleaseBlocker nein; Urteil render_write_pdf_source_adoption_not_applicable_no_write.
\
PdfSource: adopted nein; label MusicXML.
";
        let truth = super::render_candidate_selected_pdf_truth(result).expect("truth");
        assert!(!truth.write_produced);
        assert!(!truth.write_validated);
        assert_eq!(truth.selected_pdf_source, "Master-MusicXML");
        assert!(!truth.pdf_source_adopted);
        assert!(!truth.productive_materialization_green);
    }

    #[test]
    fn cargo_test_summary_parser_extracts_compact_result() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_launcher_test_summary_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test root");
        let log = root.join("strict-tests.txt");
        fs::write(
            &log,
            "running 38 tests\ntest result: ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s\n",
        )
        .expect("write test log");
        assert_eq!(
            super::cargo_test_summary_from_log(&log),
            "ok. 38 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s"
        );
        fs::remove_dir_all(root).expect("remove test root");
    }

    #[test]
    fn visual_preflight_summary_status_comes_from_run_outcome_projection() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_visual_summary_projection_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test root");
        let src = root.join("visual_preflight.full.txt");
        let dest = root.join("visual_preflight.result.txt");
        fs::write(
            &src,
            "Modus: test_visual_preflight\nSelectedTargetProject: none\nSelectedTargetLocator: n/a\nCurrentEmergenceStatus: no_current_target_awaiting_fresh_full_corpus_emergence\nCurrentEmergenceReason: test_no_seed\nGate: yellow_no_fresh_full_corpus_emergence_target\n",
        )
        .expect("write source summary");

        let run_outcome = super::RunOutcome::evaluate(super::RunOutcomeInput {
            mode: super::RunMode::VisualPreflight,
            fatal: super::FatalOutcome::NotObserved,
            final_exit: 2,
            build: super::CheckOutcome::Passed,
            clippy: super::CheckOutcome::Passed,
            audit_signal: super::AuditSignal::ExpectedNoSeed,
            renderer: super::RendererOutcome::NotObserved,
            emergence: super::EmergenceOutcome::NoCurrentTarget,
        })
        .report_projection();
        let warning_truth = super::WarningTruth {
            compiler_warning_count: 0,
            compiler_status: "clean".to_string(),
            compiler_scope: "test".to_string(),
            compiler_log: "test".to_string(),
            clippy_warning_count: 0,
            clippy_status: "clean".to_string(),
            clippy_scope: "test".to_string(),
            clippy_log: "test".to_string(),
        };
        let reference_renderer_context =
            super::AssistantContextRendererState::empty_reference_audit();

        super::write_visual_preflight_assistant_summary(
            super::VisualPreflightAssistantSummaryInput {
                src: &src,
                dest: &dest,
                version: "0.10.test",
                run_outcome: &run_outcome,
                strict_test_exit: "0",
                strict_test_summary: "ok. test",
                ledgers: &[],
                warning_truth: &warning_truth,
                reference_renderer_context: &reference_renderer_context,
            },
        )
        .expect("write projected summary");

        let result = fs::read_to_string(&dest).expect("read projected summary");
        assert!(result.contains("Status: NO-SEED"));
        assert!(result.contains("RunSeverity: yellow"));
        assert!(result.contains("ReleaseDecision: audit_blocked_no_seed"));
        assert!(result.contains("StatusProjectionSource: run_outcome_projection"));
        assert!(!result.contains("Status: AUDIT-BLOCKED"));
        fs::remove_dir_all(root).expect("remove test root");
    }

    #[test]
    fn renderer_truth_separates_current_preflight_absence_from_persistent_reference() {
        let current = super::AssistantContextRendererState::empty_current_preflight();
        assert_eq!(
            super::current_preflight_reference_guard_status(
                &[],
                "Synthetic Project Alpha",
                &current,
            ),
            "not_evaluated_no_current_reference_project_run"
        );

        let mut persistent = super::AssistantContextRendererState::empty_reference_audit();
        persistent.evidence_status =
            "persistent_confirmed_reference_renderer_evidence_found".to_string();
        persistent.renderer_overall_status =
            "yellow_output_valid_process_failed".to_string();
        assert_eq!(
            persistent.persistent_guard_status(),
            "yellow_no_renderer_exit_acceptance"
        );
    }

    #[test]
    fn current_run_test_log_replaces_stale_previous_run_evidence() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_launcher_test_log_reset_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test root");
        let log = root.join("strict-tests.txt");
        fs::write(&log, "stale-version-test-evidence\n").expect("write stale test log");
        super::prepare_current_run_test_log(&log).expect("reset test log");
        assert_eq!(fs::read_to_string(&log).expect("read reset log"), "");
        fs::remove_dir_all(root).expect("remove test root");
    }

    #[test]
    fn fresh_gate_stays_green_when_all_project_rows_are_fresh() {
        assert!(visual_preflight_freshness_gate_green(2, 2, 0));
    }

    #[test]
    fn fresh_gate_is_red_for_stale_project_rows() {
        assert!(!visual_preflight_freshness_gate_green(2, 2, 1));
    }

    #[test]
    fn fresh_gate_is_red_for_incomplete_or_empty_runs() {
        assert!(!visual_preflight_freshness_gate_green(2, 1, 0));
        assert!(!visual_preflight_freshness_gate_green(0, 0, 0));
    }

    #[test]
    fn bootstrap_cleanup_contract_covers_current_and_legacy_run_directories() {
        assert_eq!(super::RUN_LAUNCHER_BOOTSTRAP_RUNS_DIR, "run_launcher_bootstrap_runs");
        assert_eq!(super::TAKE_BOOTSTRAP_RUNS_DIR, "take_bootstrap_runs");
        assert_eq!(
            super::LEGACY_LAUNCHER_BOOTSTRAP_RUNS_DIR,
            "launcher_bootstrap_runs"
        );
    }

    #[test]
    fn build_cache_hygiene_removes_only_fingerprinted_cargo_cache_directories() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_build_cache_hygiene_{}_{}",
            std::process::id(),
            nonce
        ));
        let bootstrap_incremental =
            root.join("target/bc/debug/incremental");
        let legacy_bootstrap_cache = root.join("target/etuschlopf_bootstrap_cache");
        let target_debug = root.join("target/debug");
        let target_release = root.join("target/release");
        let legacy_debug = root.join("debug");
        fs::create_dir_all(bootstrap_incremental.join("crate-a")).expect("bootstrap cache");
        fs::write(bootstrap_incremental.join("crate-a/object.o"), vec![1_u8; 64])
            .expect("bootstrap object");
        fs::create_dir_all(legacy_bootstrap_cache.join("debug/incremental/crate-old"))
            .expect("legacy bootstrap cache");
        fs::write(
            legacy_bootstrap_cache.join("debug/incremental/crate-old/object.o"),
            vec![5_u8; 24],
        )
        .expect("legacy bootstrap object");
        for (profile, value, size) in [
            (&target_debug, 3_u8, 16_usize),
            (&target_release, 4_u8, 8_usize),
            (&legacy_debug, 2_u8, 32_usize),
        ] {
            fs::create_dir_all(profile.join(".fingerprint/crate-b")).expect("fingerprint");
            fs::create_dir_all(profile.join("deps")).expect("deps");
            fs::create_dir_all(profile.join("incremental")).expect("incremental");
            fs::write(profile.join(".cargo-lock"), "").expect("cargo lock");
            fs::write(profile.join("deps/libcrate.rlib"), vec![value; size])
                .expect("profile object");
        }

        let report = super::cleanup_build_cache_hygiene(&root);
        assert_eq!(report.outcome("bootstrap_incremental"), None);
        assert_eq!(
            report.outcome("legacy_bootstrap_cache"),
            Some(&super::CachePathOutcome::Deleted)
        );
        assert_eq!(
            report.outcome("target_debug"),
            Some(&super::CachePathOutcome::Deleted)
        );
        assert_eq!(
            report.outcome("target_release"),
            Some(&super::CachePathOutcome::Deleted)
        );
        assert_eq!(
            report.outcome("legacy_root_debug"),
            Some(&super::CachePathOutcome::Deleted)
        );
        assert_eq!(report.active_profile_skips(), 0);
        assert_eq!(report.failed_removals(), 0);
        assert_eq!(report.before.bytes, 80);
        assert_eq!(report.after.bytes, 0);
        assert!(bootstrap_incremental.join("crate-a/object.o").exists());
        assert!(!legacy_bootstrap_cache.exists());
        assert!(!target_debug.exists());
        assert!(!target_release.exists());
        assert!(!legacy_debug.exists());

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn build_cache_hygiene_preserves_arbitrary_root_debug_directory() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_build_cache_hygiene_preserve_{}_{}",
            std::process::id(),
            nonce
        ));
        let debug = root.join("debug");
        let target_debug = root.join("target/debug");
        let target_release = root.join("target/release");
        for profile in [&debug, &target_debug, &target_release] {
            fs::create_dir_all(profile).expect("profile");
            fs::write(profile.join("notes.txt"), "keep me").expect("notes");
        }

        let report = super::cleanup_build_cache_hygiene(&root);
        assert_eq!(
            report.outcome("target_debug"),
            Some(&super::CachePathOutcome::SkippedNotCargoProfile)
        );
        assert_eq!(
            report.outcome("target_release"),
            Some(&super::CachePathOutcome::SkippedNotCargoProfile)
        );
        assert_eq!(
            report.outcome("legacy_root_debug"),
            Some(&super::CachePathOutcome::SkippedNotCargoProfile)
        );
        assert_eq!(report.failed_removals(), 0);
        assert!(debug.join("notes.txt").exists());
        assert!(target_debug.join("notes.txt").exists());
        assert!(target_release.join("notes.txt").exists());

        fs::remove_dir_all(root).expect("cleanup");
    }

    #[test]
    fn test_prewarm_uses_half_the_cargo_jobs_and_never_zero() {
        assert_eq!(crate::test_prewarm_cargo_jobs(19), 9);
        assert_eq!(crate::test_prewarm_cargo_jobs(8), 4);
        assert_eq!(crate::test_prewarm_cargo_jobs(2), 1);
        assert_eq!(crate::test_prewarm_cargo_jobs(1), 1);
        let prewarm = crate::strict_test_cargo_args(crate::test_prewarm_cargo_jobs(19));
        let jobs = prewarm.iter().position(|arg| arg == "--jobs").unwrap();
        assert_eq!(prewarm[jobs + 1], "9");
        let authoritative = crate::strict_test_cargo_args(19);
        let jobs = authoritative.iter().position(|arg| arg == "--jobs").unwrap();
        assert_eq!(authoritative[jobs + 1], "19");
    }

    // Historical KodexNegativeCheck symbol retained intentionally: the protected invariant evolved
    // from global incremental-off to fail-closed derived-cache reuse with verified binary authority.
    #[test]
    fn run_cmd_disables_cargo_incremental_compilation() {
        let run_cmd = include_str!("../../run.cmd");
        assert!(run_cmd.contains("set \"CARGO_INCREMENTAL=1\""));
        assert!(run_cmd.contains("StrictProfilePackageClean=retired_incremental_reuse"));
        assert!(run_cmd.contains("BootstrapPackageClean=retired_incremental_reuse"));
        assert!(run_cmd.contains("ReuseAuthority=source_signature+cargo_fingerprint+binary_fnv"));
        assert!(!run_cmd.contains("cargo clean --quiet --package etuschlopf"));
        assert!(run_cmd.contains("ETUSCHLOPF_BOOTSTRAP_CACHE=target\\bc"));
        assert!(!run_cmd.contains("ETUSCHLOPF_BOOTSTRAP_CACHE=target\\etuschlopf_bootstrap_cache"));
    }

    #[test]
    fn visual_preflight_cleanup_protects_all_hash_bound_corpus_result_evidence() {
        use super::corpus_run_manifest::{file_evidence, project_evidence, CorpusRunManifest, CORPUS_RUN_MANIFEST_FILE};
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_launcher_manifest_cleanup_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test dir");
        let version = env!("CARGO_PKG_VERSION");
        let projects = (1..=10)
            .map(|number| format!("Etüde Nr. {number}"))
            .collect::<Vec<_>>();
        let mut manifest = CorpusRunManifest::started(
            version,
            "corpus-launcher-test".to_string(),
            100,
            projects.clone(),
        );
        for project in &projects {
            let result_path = root.join(format!("{project}.result.txt"));
            let text = format!(
                "ETUSCHLOPF-ERGEBNIS\nWerkzeug: etuschlopf {version}\nPartitur: {project}\nErgebnis: OK\nSilentVoiceVisibleMeasureCoverageWriterStage v{version}: Attempted ja; SelectedRenderSource ja; WriteEnabled ja; Candidates 0; MeasuresWritten 0; ForwardNodesReplaced 0; ForwardDurationReplaced 0; BeamLostBeginNotesWritten 0; CandidateRowsAfter 0; RenderXmlWrite nein; NotationMutation nein; MusicMutation nein; MasterApply nein; Urteil test.\nFinalBeamLostBeginRenderWriterStage v{version}: Attempted ja; SelectedRenderSource ja; MasterSourceBound ja; WriteEnabled ja; Candidates 0; NotesWritten 0; CandidateRowsAfter 0; RenderXmlWrite nein; NotationMutation nein; MusicMutation nein; MasterApply nein; Urteil test.\n"
            );
            fs::write(&result_path, &text).expect("write result");
            manifest.upsert_project(project_evidence(
                project,
                &result_path,
                &text,
                Some(0),
                true,
            ));
        }
        let report_path = root.join("corpus_emergence.result.txt");
        fs::write(
            &report_path,
            format!(
                "ETUSCHLOPF-CORPUS-EMERGENCE\nWerkzeug: etuschlopf {version}\nRunId: corpus-launcher-test\nGeneratedUnixSeconds: 200\nFullCorpus: ja\nProjectCount: 10\nCurrentVersionProjectCount: 10\nProjects: {}\nSelectionEligible: ja\n",
                projects.join(", ")
            ),
        )
        .expect("write report");
        manifest.finalize(200, file_evidence(&report_path).expect("report evidence"));
        manifest
            .write_atomic(&root.join(CORPUS_RUN_MANIFEST_FILE))
            .expect("write manifest");

        let protected = super::current_standard_output_names(&root, Some("--visual-preflight"));
        assert!(protected.contains(CORPUS_RUN_MANIFEST_FILE));
        assert!(protected.contains("corpus_emergence.result.txt"));
        assert!(protected.contains(super::VISUAL_PREFLIGHT_STRICT_TEST_LOG));
        for project in &projects {
            assert!(
                protected.contains(&format!("{project}.result.txt")),
                "missing protected result for {project}"
            );
        }

        fs::remove_dir_all(root).expect("remove test dir");
    }


    #[test]
    fn emergence_hotspot_class_is_behind_bars_check_only_without_write_authority() {
        let class = super::EmergenceTargetClass::BehindBarsRuleCheckHotspot;
        assert_eq!(class.as_str(), "behind_bars_rule_check_hotspot");
        assert!(class.behind_bars_rule_check_required());
        assert!(!class.rule_candidate());
        assert!(!class.write_candidate());
        assert_eq!(class.permission_contract(), "behind_bars_check_only_no_rule_or_write_authority");
    }

    #[test]
    fn no_current_target_has_no_rule_check_or_write_authority() {
        let class = super::EmergenceTargetClass::NoCurrentTarget;
        assert_eq!(class.as_str(), "none");
        assert!(!class.behind_bars_rule_check_required());
        assert!(!class.rule_candidate());
        assert!(!class.write_candidate());
        assert_eq!(
            class.permission_contract(),
            "no_current_target_no_rule_check_or_write_authority"
        );
    }

    #[test]
    fn warning_diagnostic_count_excludes_cargo_summary_lines() {
        let text = r#"warning: method `x` is never used
warning: `etuschlopf` (bin "etuschlopf_launcher") generated 1 warning
warning: `etuschlopf` (bin "etuschlopf") generated 2 warnings
"#;
        assert_eq!(super::warning_diagnostic_count(text), 1);
        assert!(super::is_cargo_warning_summary_line(
            "warning: `etuschlopf` generated 1 warning"
        ));
        assert!(super::is_cargo_warning_summary_line(
            "warning: `etuschlopf` generated 2 warnings (run `cargo clippy --fix` to apply suggestions)"
        ));
    }

    #[test]
    fn deep_clippy_warning_line_accepts_human_and_cargo_short_rendering() {
        assert!(super::is_deep_clippy_warning_line(
            "warning: package `etuschlopf` is missing metadata"
        ));
        assert!(super::is_deep_clippy_warning_line(
            "src/main.rs:12:5: warning: redundant closure [clippy::redundant_closure_for_method_calls]"
        ));
        assert!(!super::is_deep_clippy_warning_line(
            "src/main.rs:12:5: note: generated by macro expansion"
        ));
    }

    #[test]
    fn typed_progress_role_maps_test_without_label_parsing() {
        use super::{progress_line_presentation, Attention, Emphasis, PresentationRole};

        let test = progress_line_presentation(PresentationRole::Test, "x");
        assert_eq!(test.role, PresentationRole::Check);
        assert_eq!(test.attention, Attention::Neutral);
        assert_eq!(test.emphasis, Emphasis::Secondary);

        let build = progress_line_presentation(PresentationRole::Build, "x");
        assert_eq!(build.role, PresentationRole::Build);
        assert_eq!(build.attention, Attention::Neutral);
        assert_eq!(build.emphasis, Emphasis::Secondary);

        let program = progress_line_presentation(PresentationRole::Program, "x");
        assert_eq!(program.role, PresentationRole::Program);
        assert_eq!(program.attention, Attention::Neutral);
        assert_eq!(program.emphasis, Emphasis::Primary);
    }

    #[test]
    fn launcher_console_runtime_has_no_global_color_subprocess_or_process_exit() {
        let launcher = include_str!("etuschlopf_launcher.rs");
        let build_control = include_str!("etuschlopf_launcher/launcher_build_control.rs");
        let color_subprocess = [".args([\"/C\", ", "\"color\""].concat();
        let process_exit = ["std::process::", "exit("].concat();
        let heartbeat_parser = ["phase_for_", "heartbeat"].concat();
        let legacy_phase_enum = ["enum P", "hase {"].concat();
        let legacy_presentation_adapter = ["legacy_", "presentation_line("].concat();
        let presentation_role_phase_adapter = ["phase_for_", "presentation_role"].concat();
        assert!(!launcher.contains(&color_subprocess));
        assert!(!launcher.contains(&process_exit));
        assert!(!build_control.contains(&process_exit));
        assert!(!launcher.contains(&heartbeat_parser));
        assert!(!launcher.contains(&legacy_phase_enum));
        assert!(!launcher.contains(&legacy_presentation_adapter));
        assert!(!launcher.contains(&presentation_role_phase_adapter));
    }

    #[test]
    fn console_summary_phases_use_green_yellow_and_red() {
        use super::{ansi_outcome_style, OutcomeColor};

        assert_eq!(ansi_outcome_style(OutcomeColor::DarkGreen), "\x1b[1;32m");
        assert_eq!(ansi_outcome_style(OutcomeColor::LightGreen), "\x1b[92m");
        assert_eq!(ansi_outcome_style(OutcomeColor::Yellow), "\x1b[1;33m");
        assert_eq!(ansi_outcome_style(OutcomeColor::Red), "\x1b[1;31m");
    }

    #[test]
    fn console_run_mode_adapter_covers_all_launcher_summary_paths() {
        use super::{run_mode_for_console, RunMode};

        assert_eq!(run_mode_for_console("visual_preflight"), RunMode::VisualPreflight);
        assert_eq!(run_mode_for_console("all_run"), RunMode::AllRun);
        assert_eq!(run_mode_for_console("normal_run"), RunMode::NormalRun);
    }

    #[test]
    fn launcher_startup_header_is_compact_and_nonredundant() {
        let thread_plan = super::ThreadPlan {
            logical_threads: 20,
            cargo_jobs: 19,
            pipeline_auto_hint_workers: 19,
        };
        let header = format!(
            "[EtuSchlopf v{}] {} · {}",
            "0.10.test",
            "Synthetic Project Alpha",
            thread_plan.interactive_summary_label()
        );

        assert_eq!(
            header,
            "[EtuSchlopf v0.10.test] Synthetic Project Alpha · System 20 · Jobs 19"
        );
        assert!(!header.contains("CargoJobs"));
        assert!(!header.contains("PipelineAutoHint"));
        assert!(!header.contains("Cargo --jobs"));
        assert!(!header.contains("Log:"));
    }

    #[test]
    fn launcher_visual_hierarchy_is_semantic_and_state_free() {
        let launcher = include_str!("etuschlopf_launcher.rs");
        let final_phase_state = ["final_phase", "_seen"].concat();
        let word_diet = ["console_word", "_diet"].concat();
        let fixed_width = ["ANIMATION_WINDOW", "_WIDTH"].concat();

        assert!(!launcher.contains(&final_phase_state));
        assert!(!launcher.contains(&word_diet));
        assert!(!launcher.contains(&fixed_width));
        assert!(launcher.contains("write_progress_done_segments"));
        assert!(launcher.contains("write_outcome_headline"));
    }

    #[test]
    fn launcher_role_palette_is_muted_while_status_keeps_signal_color() {
        use super::{
            ansi_role_style, ansi_style_for_presentation, Attention, Emphasis,
            PresentationChannel, PresentationLine, PresentationRole, Visibility,
        };

        let build = PresentationLine {
            role: PresentationRole::Build,
            status: None,
            attention: Attention::Neutral,
            emphasis: Emphasis::Secondary,
            visibility: Visibility::InteractiveOnly,
            channel: PresentationChannel::Narrative,
            text: "Build",
        };
        assert_eq!(ansi_role_style(PresentationRole::Infrastructure), "\x1b[90m");
        assert_eq!(ansi_role_style(PresentationRole::Build), "\x1b[35m");
        assert_eq!(ansi_style_for_presentation(build), "\x1b[35m");
    }

    #[test]
    fn launcher_segment_renderer_separates_role_status_and_detail_color() {
        use super::{
            write_console_segments, ConsoleSegment, GlyphMode, LocalStatus, PresentationRole,
            ProgressMode, StreamRenderPolicy, StyleMode,
        };

        let policy = StreamRenderPolicy {
            style: StyleMode::Styled,
            progress: ProgressMode::Lines,
            glyphs: GlyphMode::Unicode,
        };
        let mut rendered = Vec::new();
        write_console_segments(
            &mut rendered,
            policy,
            &[
                ConsoleSegment::muted("[EtuSchlopf v0.10.test]"),
                ConsoleSegment::plain(" "),
                ConsoleSegment::role(PresentationRole::Test, "Rust-Tests"),
                ConsoleSegment::plain(" "),
                ConsoleSegment::status(LocalStatus::Ok, "OK"),
                ConsoleSegment::plain(": "),
                ConsoleSegment::muted("2 passed; 0 failed"),
            ],
        )
        .expect("segment renderer");
        let rendered = String::from_utf8(rendered).expect("utf8");
        assert!(rendered.contains("\x1b[36mRust-Tests\x1b[0m"));
        assert!(rendered.contains("\x1b[32mOK\x1b[0m"));
        assert!(rendered.contains("\x1b[90m2 passed; 0 failed\x1b[0m"));
        assert!(!rendered.contains("\x1b[36mRust-Tests OK: 2 passed"));
    }

    fn shadow_renderer_test_ledger(project: &str, exit_code: i32) -> super::VisualProjectRunLedger {
        super::VisualProjectRunLedger {
            project: project.to_string(),
            exit_code,
            result_file: std::path::PathBuf::new(),
            result_exists: true,
            result_status: "AUDIT-BLOCKED".to_string(),
            result_version: "0.10.test".to_string(),
            fresh: false,
            verdict: "program_exit_red".to_string(),
            shadow_run_started_at: Some(std::time::SystemTime::UNIX_EPOCH),
            previous_pdf_fingerprint: None,
            current_pdf_fingerprint: None,
            previous_pdf_visual_fingerprint: None,
            current_pdf_visual_fingerprint: None,
        }
    }

    fn valid_failed_renderer_context(version: &str, project: &str) -> super::AssistantContextRendererState {
        let mut context = super::AssistantContextRendererState::empty_current_preflight();
        context.affected_project = project.to_string();
        context.evidence_version = version.to_string();
        context.renderer_winning_exit_code = "-1073741819".to_string();
        context.renderer_process_exit_code_hex = "0xC0000005".to_string();
        context.renderer_output_status = "complete_and_structurally_plausible".to_string();
        context.renderer_process_status = "crashed_with_windows_access_violation".to_string();
        context.renderer_overall_status = "yellow_output_valid_process_failed".to_string();
        context.evidence_modified_at = Some(
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1),
        );
        context
    }

    #[test]
    fn shadow_renderer_reconciliation_accepts_only_matching_current_evidence() {
        use super::run_outcome::RendererOutcome;

        let version = "0.10.test";
        let project = "Synthetic Project Alpha";
        let context = valid_failed_renderer_context(version, project);
        let ledgers = vec![shadow_renderer_test_ledger(project, 2)];
        assert_eq!(
            super::reconcile_renderer_outcome_with_current_evidence(
                RendererOutcome::ProcessFailed(2),
                &ledgers,
                &context,
                version,
            ),
            RendererOutcome::OutputValidProcessFailed(-1_073_741_819)
        );

        let successful_outer_ledger = vec![shadow_renderer_test_ledger(project, 0)];
        assert_eq!(
            super::reconcile_renderer_outcome_with_current_evidence(
                RendererOutcome::Passed,
                &successful_outer_ledger,
                &context,
                version,
            ),
            RendererOutcome::OutputValidProcessFailed(-1_073_741_819)
        );
    }

    #[test]
    fn persistent_reference_renderer_evidence_survives_version_and_target_changes() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_persistent_renderer_reference_{}_{}",
            std::process::id(),
            nonce
        ));
        let docs = root.join("docs");
        fs::create_dir_all(&docs).expect("create docs dir");
        fs::write(
            root.join(super::PERSISTENT_REFERENCE_RENDERER_EVIDENCE_FILE),
            "ETUSCHLOPF PERSISTENT CONFIRMED REFERENCE RENDERER EVIDENCE\n\
EvidenceContract: persistent_confirmed_reference_v1\n\
Version: 0.10.1111\n\
Project: Synthetic Project Alpha\n\
Instrumentation: render_rs_output_freshness_and_process_lifecycle_v1110\n\
RendererFinalStatus: WrittenButNonZeroExit\n\
RendererWinningExitCode: -1073741819\n\
RendererProcessExitCodeHex: 0xC0000005\n\
RendererProcessExitClass: windows_ntstatus_access_violation\n\
RendererProcessElapsedMs: 1793\n\
OutputCreatedAfterDelete: ja\n\
OutputPresentAtProcessEnd: ja\n\
OutputSecondPresenceAfterProcessMs: 251\n\
OutputContentStableAcrossSettle: ja\n\
OutputStructureStatus: green_basic_pdf_structure_plausible\n\
FreshnessStatus: yellow_renderer_wrote_fresh_output_after_delete_but_exit_nonzero\n\
RendererLifecycleInterpretation: persistent_reference_test\n",
        )
        .expect("write persistent reference");

        let state = super::assistant_context_renderer_state_from_runtime(
            &root,
            "none_project_neutral_renderer_evidence_selection",
            "0.10.future",
        );
        assert_eq!(
            state.evidence_status,
            "persistent_confirmed_reference_renderer_evidence_found"
        );
        assert_eq!(
            state.evidence_scope,
            "persistent_confirmed_reference_renderer_audit_independent_of_target_and_version"
        );
        assert_eq!(
            state.renderer_overall_status,
            "yellow_output_valid_process_failed"
        );
        assert!(state.has_usable_reference_evidence());
        fs::remove_dir_all(root).expect("remove test root");
    }

    #[test]
    fn shadow_renderer_reconciliation_rejects_stale_or_foreign_evidence() {
        use super::run_outcome::RendererOutcome;

        let version = "0.10.test";
        let project = "Synthetic Project Alpha";
        let ledgers = vec![shadow_renderer_test_ledger(project, 2)];
        let stale = valid_failed_renderer_context("0.10.old", project);
        assert_eq!(
            super::reconcile_renderer_outcome_with_current_evidence(
                RendererOutcome::ProcessFailed(2),
                &ledgers,
                &stale,
                version,
            ),
            RendererOutcome::ProcessFailed(2)
        );

        let mut old_same_version = valid_failed_renderer_context(version, project);
        old_same_version.evidence_modified_at = Some(std::time::SystemTime::UNIX_EPOCH);
        let mut later_ledgers = vec![shadow_renderer_test_ledger(project, 2)];
        later_ledgers[0].shadow_run_started_at = Some(
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(2),
        );
        assert_eq!(
            super::reconcile_renderer_outcome_with_current_evidence(
                RendererOutcome::ProcessFailed(2),
                &later_ledgers,
                &old_same_version,
                version,
            ),
            RendererOutcome::ProcessFailed(2)
        );

        let foreign = valid_failed_renderer_context(version, "Synthetic Project Beta");
        assert_eq!(
            super::reconcile_renderer_outcome_with_current_evidence(
                RendererOutcome::ProcessFailed(2),
                &ledgers,
                &foreign,
                version,
            ),
            RendererOutcome::ProcessFailed(2)
        );

        let multiple_failures = vec![
            shadow_renderer_test_ledger(project, 2),
            shadow_renderer_test_ledger("Synthetic Project Beta", 3),
        ];
        let current = valid_failed_renderer_context(version, project);
        assert_eq!(
            super::reconcile_renderer_outcome_with_current_evidence(
                RendererOutcome::ProcessFailed(2),
                &multiple_failures,
                &current,
                version,
            ),
            RendererOutcome::ProcessFailed(2)
        );
    }

    #[test]
    fn recovery_take_script_is_root_hygiene_residue() {
        assert!(super::is_root_temp_file_name(
            "EtuSchlopf_RECOVERY_TAKE_v0_10_1214.ps1"
        ));
        assert!(!super::is_root_temp_file_name("run.cmd"));
    }


    #[test]
    fn deep_clippy_is_a_standalone_launcher_mode() {
        use std::ffi::OsString;

        let args = super::parse_args(vec![OsString::from("--clippy-deep")]);
        let strict_first = super::parse_args(vec![
            OsString::from("--strict"),
            OsString::from("--clippy-deep"),
        ]);
        let strict_last = super::parse_args(vec![
            OsString::from("--clippy-deep"),
            OsString::from("--strict"),
        ]);
        let corpus = super::parse_args(vec![OsString::from("--all")]);

        assert_eq!(args.validation(), super::ValidationPlan::DeepClippy);
        assert_eq!(strict_first.validation(), super::ValidationPlan::DeepClippy);
        assert_eq!(strict_last.validation(), super::ValidationPlan::DeepClippy);
        assert_eq!(
            args.validation().build_profile(),
            super::CargoBuildProfile::Strict
        );
        assert!(args.input_arg().is_none());
        assert_eq!(
            args.startup_root_cleanup_policy(),
            super::RootCleanupPolicy::PreserveGeneratedOutputs
        );
        assert_eq!(
            corpus.startup_root_cleanup_policy(),
            super::RootCleanupPolicy::CurrentTarget
        );
        assert!(args.program_args().is_empty());
        assert!(!super::is_public_artifact_transaction_eligible(&args));
    }


    #[test]
    fn deep_clippy_startup_cleanup_preserves_generated_outputs() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let root = std::env::temp_dir().join(format!(
            "etuschlopf_deep_clippy_cleanup_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir_all(&root).expect("create test root");
        let outputs = [
            "notation_test.result.txt",
            "notation_test.strict-clippy.txt",
            "notation_test.musicxml",
            "notation_test.render.musicxml",
            "notation_test.pdf",
        ];
        for name in outputs {
            fs::write(root.join(name), "keep").expect("write generated output");
        }
        let stale_zip = root.join("EtuSchlopf_v0_10_1000_old.zip");
        fs::write(&stale_zip, "delete").expect("write stale zip");

        let report = super::cleanup_project_root_residue_with_policy(
            &root,
            None,
            super::RootCleanupPolicy::PreserveGeneratedOutputs,
            false,
        );

        for name in outputs {
            assert!(root.join(name).exists(), "deep clippy removed {name}");
        }
        assert!(!stale_zip.exists(), "ordinary root residue should still be removed");
        assert_eq!(report.stale_output_files_deleted, 0);
        assert_eq!(report.stale_zip_files_deleted, 1);
        fs::remove_dir_all(root).expect("remove test root");
    }

    #[test]
    fn final_console_projection_distinguishes_warning_debt_from_audit_stop() {
        assert_eq!(
            super::final_console_status_label(
                "AUDIT-BLOCKED",
                "audit_blocked_expected_audit_block",
                2,
            ),
            "WARNING-DEBT"
        );
        assert_eq!(
            super::final_console_hint(
                Some("legacy generic hint"),
                "AUDIT-BLOCKED",
                "audit_blocked_expected_audit_block",
                2,
            ),
            Some("Freigabe: gelb | sichtbare Warning-Debt; technischer Lauf erfolgreich.")
        );
    }

    #[test]
    fn final_console_projection_keeps_machine_audit_block_visibly_nonfatal() {
        assert_eq!(
            super::final_console_status_label(
                "AUDIT-BLOCKED",
                "audit_blocked_expected_audit_block",
                0,
            ),
            "AUDIT-STOP"
        );
        assert_eq!(
            super::final_console_hint(
                Some("legacy generic hint"),
                "AUDIT-BLOCKED",
                "audit_blocked_expected_audit_block",
                0,
            ),
            Some("Freigabe: gelb | erwarteter Audit-Stop.")
        );
    }

    #[test]
    fn final_console_projection_keeps_clean_release_compact() {
        assert_eq!(
            super::final_console_status_label("OK", "released", 0),
            "OK"
        );
        assert_eq!(super::final_console_hint(None, "OK", "released", 0), None);
    }

    #[test]
    fn compiled_version_is_owned_by_cargo_package_metadata() {
        assert_eq!(super::VERSION, env!("CARGO_PKG_VERSION"));
    }
}


#[cfg(test)]
mod corpus_writer_trace_forwarding_tests {
    use super::*;

    fn forwarded(args: &[&str]) -> Vec<OsString> {
        let parsed = parse_args(args.iter().map(OsString::from).collect());
        let request = parsed.execution().expect("execution request");
        assert!(matches!(request.target, RunTarget::Corpus));
        corpus_program_args(request)
    }

    #[test]
    fn corpus_bare_trace_reaches_the_actual_corpus_argument_builder() {
        assert_eq!(forwarded(&["--all", "--writer-trace"]),
            vec![OsString::from("--all"), OsString::from("--writer-trace")]);
    }

    #[test]
    fn corpus_measure_trace_is_preserved_before_or_after_all() {
        for trace in ["--writer-trace=3,71", "--writer-trace=all"] {
            for args in [vec!["--all", trace], vec![trace, "--all"]] {
                assert_eq!(forwarded(&args), vec![OsString::from("--all"), OsString::from(trace)]);
            }
        }
    }

    #[test]
    fn corpus_without_trace_retains_the_exact_previous_invocation() {
        assert_eq!(forwarded(&["--all"]), vec![OsString::from("--all")]);
        assert_eq!(forwarded(&["--strict", "--all", "--jobs", "4", "--inventory"]),
            vec![OsString::from("--all")]);
    }

    #[test]
    fn corpus_forwards_trace_verbatim_without_widening_other_options() {
        assert_eq!(forwarded(&["--all", "--jobs=4", "--writer-trace=03,71", "--no-pdf"]),
            vec![OsString::from("--all"), OsString::from("--writer-trace=03,71")]);
    }
}

#[cfg(test)]
mod console_header_tests {
    use super::assistant_export::ConsoleRevisionProjection;

    fn revisions(source: Option<u64>, head: Option<u64>, candidate: Option<u64>) -> ConsoleRevisionProjection {
        ConsoleRevisionProjection {
            source_tree: source,
            canonical_head: head,
            in_flight_candidate: candidate,
        }
    }

    #[test]
    fn take_header_names_running_candidate_and_accepted_parent() {
        let projection = revisions(Some(1501), Some(1500), Some(1501));
        let header = super::startup_header_text(&projection, true, "--all", "System 20 · Jobs 19");
        assert!(header.starts_with("EtuSchlopf r1501 ("), "{header}");
        assert!(header.contains("· Take-Kandidat · --all · System 20 · Jobs 19"), "{header}");
        assert_eq!(
            super::take_header_detail(&projection, "RefactorOnly"),
            "r1500 akzeptiert → r1501 · RefactorOnly"
        );
    }

    #[test]
    fn normal_header_claims_acceptance_only_for_the_canonical_head() {
        let accepted = revisions(Some(1500), Some(1500), None);
        assert!(super::startup_header_text(&accepted, false, "--all", "S").contains("· akzeptiert ·"));
        let unaccepted = revisions(Some(1502), Some(1500), None);
        assert!(super::startup_header_text(&unaccepted, false, "--all", "S")
            .contains("r1502 (") );
        assert!(super::startup_header_text(&unaccepted, false, "--all", "S")
            .contains("nicht als akzeptiert belegt"));
        let unknown = revisions(None, None, None);
        let header = super::startup_header_text(&unknown, false, "--all", "S");
        assert!(header.starts_with("EtuSchlopf r? ("), "{header}");
        assert!(header.contains("nicht als akzeptiert belegt"));
        assert_eq!(super::take_header_detail(&unknown, ""), "r? akzeptiert → r?");
    }

    #[test]
    fn preparation_summary_recaps_take_run_steps_in_order() {
        let readback = "\n[STARTUP-TIMING-READBACK]\n\
StartupCmdDuration: Phase=TakeBootstrapCargoBuild; Invocation=1; WallMs=26890; Precision=10ms_local_clock\n\
StartupRustTiming: Phase=TakeExtraction; Event=end; WallUs=1088309; Pid=7\n\
StartupRustTiming: Phase=TakeBeforeReexec; Event=end; WallUs=94360279; Pid=7\n\
StartupCmdDuration: Phase=LauncherBootstrapCargoBuild; Invocation=2; WallMs=49790; Precision=10ms_local_clock\n\
[/STARTUP-TIMING-READBACK]\n";
        assert_eq!(
            super::startup_preparation_summary(readback).as_deref(),
            Some("Bootstrap-Build 27 s · Take-Prüfung 1 min 34 s · Launcher-Build 50 s")
        );
    }

    #[test]
    fn preparation_summary_omits_missing_or_uncaptured_steps() {
        let normal = "StartupCmdDuration: Phase=LauncherBootstrapCargoBuild; Invocation=1; WallMs=3140; Precision=10ms_local_clock\n";
        assert_eq!(
            super::startup_preparation_summary(normal).as_deref(),
            Some("Launcher-Build 3,1 s")
        );
        let gap = "StartupCmdDuration: Phase=LauncherBootstrapCargoBuild; Invocation=1; WallMs=not_captured_date_or_clock_discontinuity; Precision=10ms_local_clock\n\
StartupRustTiming: Phase=TakeBeforeReexec; Event=begin; Pid=7\n";
        assert_eq!(super::startup_preparation_summary(gap), None);
        assert_eq!(super::startup_preparation_summary("TimingStatus: not_captured_or_session_mismatch\n"), None);
    }

    #[test]
    fn run_cmd_build_hint_is_whole_seconds_without_newline() {
        assert_eq!(super::build_seconds_text(49_790), "50");
        assert_eq!(super::build_seconds_text(26_490), "26");
        assert_eq!(super::build_seconds_text(400), "0");
        let times = super::startup_preparation_times(
            "StartupCmdDuration: Phase=LauncherBootstrapCargoBuild; Invocation=1; WallMs=3140; Precision=10ms_local_clock\n",
        );
        assert_eq!(times.take_build_ms, None);
        assert_eq!(times.launcher_build_ms, Some(3140));
    }
}
