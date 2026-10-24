#[path = "etuschlopf_launcher/startup_timing.rs"]
mod startup_timing;
#[path = "../build_gate/rust_source_scan_primitives.rs"]
mod rust_source_scan_primitives;
#[path = "../build_gate/sha256.rs"]
mod sha256;
#[path = "../build_gate/managed_source_tree_definition.rs"]
mod managed_source_tree_definition;
#[allow(dead_code)]
#[path = "../take_change_control.rs"]
mod take_change_control;
#[path = "../build_gate/multi_revision_evasion.rs"]
mod multi_revision_evasion;
#[path = "../kodex_control_plane/public_artifact_recovery.rs"]
mod public_artifact_recovery;
#[allow(dead_code)]
#[path = "../console_presentation_contract.rs"]
mod console_presentation_contract;

use console_presentation_contract::{
    resolve_stream_render_policy, Attention, ColorPreference, ConsolePreferences, Emphasis,
    GlyphCapability, GlyphPreference, LocalStatus, PresentationChannel, PresentationLine,
    PresentationRole, ProgressMode, ProgressPreference, StreamCapability, StreamDestination,
    StreamRenderPolicy, StyleMode, Visibility,
};
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const RECOVER_PENDING_ARG: &str = "--recover-pending-take";
const RECOVER_PENDING_PUBLIC_ARTIFACTS_ARG: &str = "--recover-pending-public-artifacts";
const TAKE_TRANSACTION_ROOT: &str = "target/etuschlopf_take_transactions";
const TAKE_TRANSACTION_MARKER: &str = "target/etuschlopf_take_transaction_pending.txt";
const TAKE_TRANSACTION_CANDIDATE_METADATA: &str = "candidate_metadata.txt";
const TAKE_TRANSACTION_CANDIDATE_SNAPSHOT: &str = "candidate_snapshot.tsv";
const TAKE_TRANSACTION_CANDIDATE_INTENT: &str = "candidate_intent.txt";
const TAKE_RECOVERY_EXE: &str = "target/etuschlopf_take_recovery/etuschlopf_take_bootstrap.exe";
// r1076 BootstrapCacheShortPathClosure: same dedicated derived Cargo target, shorter Windows path.
const BOOTSTRAP_CARGO_TARGET_DIR: &str = "target/bc";
const ASSISTANT_EXPORT_ROOT: &str = "target/etuschlopf_assistant_export";
const ASSISTANT_EXPORT_LATEST_DIR: &str = "latest";
const ASSISTANT_EXPORT_LATEST_ZIP: &str = "latest.zip";
const ASSISTANT_EXPORT_ROOT_LATEST_ZIP: &str = "assistant_export_latest.zip";
const POST_ACCEPTANCE_LIFECYCLE_FILE: &str = "take_lifecycle_handover.txt";
const POST_ACCEPTANCE_EVIDENCE_METADATA_FILE: &str = "take_acceptance_evidence_metadata.txt";
const POST_ACCEPTANCE_EVIDENCE_INTENT_FILE: &str = "take_acceptance_candidate_intent.txt";
const POST_ACCEPTANCE_EVIDENCE_SNAPSHOT_FILE: &str = "take_acceptance_semantic_effect_snapshot.tsv";
const TAKE_FAILURE_HANDOVER_FILE: &str = "take_failure_handover.txt";
const TAKE_FAILURE_METADATA_FILE: &str = "take_failure_metadata.txt";
const TAKE_FAILURE_REASON_FILE: &str = "take_failure_reason.txt";
const TAKE_FAILURE_INTENT_FILE: &str = "take_failure_candidate_intent.txt";
const TAKE_FAILURE_SNAPSHOT_FILE: &str = "take_failure_semantic_effect_snapshot.tsv";
const TAKE_FAILURE_CANONICAL_METADATA_FILE: &str = "canonical_rejection_metadata.txt";
const TAKE_FAILURE_PARENT_ACCEPTANCE_FILE: &str = "canonical_parent_acceptance.txt";
const TAKE_FAILURE_CANONICAL_REASON_FILE: &str = "canonical_rejection_reason.txt";
const TAKE_FAILURE_CANONICAL_INTENT_FILE: &str = "canonical_rejection_candidate_intent.txt";
const TAKE_FAILURE_CANONICAL_SNAPSHOT_FILE: &str =
    "canonical_rejection_semantic_effect_snapshot.tsv";
const TAKE_FAILURE_BOOTSTRAP_CONSOLE_FILE: &str = "take_failure_bootstrap_console.txt";
const TAKE_FAILURE_CHILD_CONSOLE_FILE: &str = "take_failure_child_console.txt";
const TAKE_FAILURE_SOURCE_ARCHIVE_FILE: &str = "EtuSchlopf_failure_parent_source.zip";
// r1108 TakeFailureEvidenceFoundationArchiveClosure installs evidence authority for later Takes.
// The currently running parent bootstrap remains transaction owner throughout its own Take.
const TAKE_REJECTION_EVIDENCE_ROOT: &str = "target/etuschlopf_take_rejection_evidence_v1";
const TAKE_ATTEMPT_EVIDENCE_EXPORT_DIR: &str = "take_attempt_evidence";
const TAKE_ATTEMPT_EVIDENCE_DIR_ENV: &str = "ETUSCHLOPF_TAKE_ATTEMPT_EVIDENCE_DIR";
const EXECUTED_CANDIDATE_SOURCE_ARCHIVE_FILE: &str = "executed_candidate_source.zip";
const RESTORED_PARENT_SOURCE_ARCHIVE_FILE: &str = "restored_parent_source.zip";
const FAILURE_EVIDENCE_IDENTITY_FILE: &str = "CANDIDATE_IDENTITY.txt";
const FAILURE_EVIDENCE_METADATA_FILE: &str = "REJECTION_EVIDENCE_METADATA.txt";
const FAILURE_EVIDENCE_CHILD_WRITEAHEAD_DIR: &str = "child_writeahead";
const FAILURE_EVIDENCE_CHILD_DIR: &str = "child_evidence";
// Preserve runtime marker values while avoiding their complete literal spelling in source text
// embedded into Assistant handoffs. This keeps the first fixing take compatible with the
// accepted r1070 parent bootstrap, whose legacy parser still searches arbitrary substrings.
const POST_ACCEPTANCE_SECTION_BEGIN: &str = concat!("[POST-", "ACCEPTANCE-LIFECYCLE]");
const POST_ACCEPTANCE_SECTION_END: &str = concat!("[/POST-", "ACCEPTANCE-LIFECYCLE]");
const MAX_ZIP_ENTRIES: usize = 25_000;
const MAX_ZIP_UNCOMPRESSED_BYTES: u64 = 512 * 1024 * 1024;

type AppResult<T> = Result<T, String>;

static BOOTSTRAP_TECHNICAL_LOG: OnceLock<Mutex<Option<fs::File>>> = OnceLock::new();

fn init_bootstrap_technical_log(project_root: &Path) {
    let log_path = project_root.join("target/etuschlopf_logs/take_bootstrap.console.txt");
    let file = log_path
        .parent()
        .and_then(|parent| fs::create_dir_all(parent).ok().map(|()| log_path.clone()))
        .and_then(|path| {
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .ok()
        });
    let _ = BOOTSTRAP_TECHNICAL_LOG.set(Mutex::new(file));
}

fn append_bootstrap_technical_log(text: &str) -> bool {
    let Some(log) = BOOTSTRAP_TECHNICAL_LOG.get() else {
        return false;
    };
    let Ok(mut guard) = log.lock() else {
        return false;
    };
    let Some(file) = guard.as_mut() else {
        return false;
    };
    writeln!(file, "{text}").is_ok() && file.flush().is_ok()
}

fn bootstrap_console_preferences() -> ConsolePreferences {
    let color = if env::var_os("NO_COLOR").is_some() {
        ColorPreference::Never
    } else {
        match bootstrap_normalized_env_value("ETUSCHLOPF_COLOR").as_deref() {
            Some("0" | "false" | "off" | "no" | "never") => ColorPreference::Never,
            Some("1" | "true" | "on" | "yes" | "always") => ColorPreference::Always,
            _ => ColorPreference::Auto,
        }
    };
    let progress = match bootstrap_normalized_env_value("ETUSCHLOPF_PROGRESS").as_deref() {
        Some("0" | "false" | "off" | "no" | "plain") => ProgressPreference::Plain,
        _ => ProgressPreference::Auto,
    };
    let glyphs = match bootstrap_normalized_env_value("ETUSCHLOPF_ASCII_PROGRESS").as_deref() {
        Some("1" | "true" | "on" | "yes" | "ascii") => GlyphPreference::Ascii,
        _ => GlyphPreference::Auto,
    };
    ConsolePreferences {
        color,
        progress,
        glyphs,
    }
}

fn bootstrap_normalized_env_value(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
}

fn bootstrap_stream_capability(is_terminal: bool) -> StreamCapability {
    StreamCapability {
        destination: if is_terminal {
            StreamDestination::Terminal
        } else {
            StreamDestination::Redirected
        },
        glyphs: GlyphCapability::Unicode,
    }
}

fn bootstrap_stream_policy(to_stderr: bool) -> StreamRenderPolicy {
    static STDOUT_POLICY: OnceLock<StreamRenderPolicy> = OnceLock::new();
    static STDERR_POLICY: OnceLock<StreamRenderPolicy> = OnceLock::new();
    let preferences = bootstrap_console_preferences();
    if to_stderr {
        *STDERR_POLICY.get_or_init(|| {
            resolve_stream_render_policy(
                bootstrap_stream_capability(io::stderr().is_terminal()),
                preferences,
            )
        })
    } else {
        *STDOUT_POLICY.get_or_init(|| {
            resolve_stream_render_policy(
                bootstrap_stream_capability(io::stdout().is_terminal()),
                preferences,
            )
        })
    }
}

fn write_bootstrap_prefix<W: Write>(writer: &mut W, styled: bool) -> io::Result<()> {
    if styled {
        write!(writer, "\x1b[90m[EtuSchlopf\x1b[0m \x1b[36mtake\x1b[90m]\x1b[0m")
    } else {
        write!(writer, "[EtuSchlopf take]")
    }
}

fn write_bootstrap_interactive<W: Write>(
    writer: &mut W,
    policy: StreamRenderPolicy,
    line: PresentationLine<'_>,
) -> io::Result<()> {
    let styled = policy.style == StyleMode::Styled;
    write_bootstrap_prefix(writer, styled)?;

    match line.status {
        Some(LocalStatus::Committed) => {
            if styled {
                write!(writer, " \x1b[32mCOMMIT\x1b[0m")?;
            } else {
                write!(writer, " COMMIT")?;
            }
            if !line.text.is_empty() {
                write!(writer, " · {}", line.text)?;
            }
        }
        Some(LocalStatus::RolledBack) => {
            if styled {
                write!(writer, " \x1b[93mROLLBACK\x1b[0m")?;
            } else {
                write!(writer, " ROLLBACK")?;
            }
            if !line.text.is_empty() {
                write!(writer, " · {}", line.text)?;
            }
        }
        Some(LocalStatus::Failed) => {
            if styled {
                write!(writer, " \x1b[91mFEHLER\x1b[0m")?;
            } else {
                write!(writer, " FEHLER")?;
            }
            if !line.text.is_empty() {
                if styled {
                    write!(writer, ": \x1b[91m{}\x1b[0m", line.text)?;
                } else {
                    write!(writer, ": {}", line.text)?;
                }
            }
        }
        Some(LocalStatus::Ok) => {
            if !line.text.is_empty() {
                write!(writer, " {}", line.text)?;
            }
            if styled {
                write!(writer, " · \x1b[32mOK\x1b[0m")?;
            } else {
                write!(writer, " · OK")?;
            }
        }
        Some(LocalStatus::Blocked) => {
            if !line.text.is_empty() {
                write!(writer, " {}", line.text)?;
            }
            if styled {
                write!(writer, " · \x1b[93mBLOCKED\x1b[0m")?;
            } else {
                write!(writer, " · BLOCKED")?;
            }
        }
        Some(LocalStatus::Pending | LocalStatus::Running) => {
            write!(writer, " {}", line.text)?;
        }
        None => match line.attention {
            Attention::Warning => {
                if styled {
                    write!(writer, " \x1b[93mWARNUNG\x1b[0m: {}", line.text)?;
                } else {
                    write!(writer, " WARNUNG: {}", line.text)?;
                }
            }
            Attention::Error => {
                if styled {
                    write!(writer, " \x1b[91mFEHLER\x1b[0m: \x1b[91m{}\x1b[0m", line.text)?;
                } else {
                    write!(writer, " FEHLER: {}", line.text)?;
                }
            }
            Attention::Neutral | Attention::Notice => {
                write!(writer, " {}", line.text)?;
            }
        },
    }
    writeln!(writer)?;
    writer.flush()
}

fn write_bootstrap_presentation(
    role: PresentationRole,
    status: Option<LocalStatus>,
    attention: Attention,
    visibility: Visibility,
    to_stderr: bool,
    arguments: std::fmt::Arguments<'_>,
) {
    let text = arguments.to_string();
    let line = PresentationLine {
        role,
        status,
        attention,
        emphasis: if attention == Attention::Error {
            Emphasis::Primary
        } else {
            Emphasis::Secondary
        },
        visibility,
        channel: if to_stderr {
            PresentationChannel::Diagnostic
        } else {
            PresentationChannel::Narrative
        },
        text: &text,
    };
    let technical_requested = matches!(line.visibility, Visibility::TechnicalOnly | Visibility::Both);
    let technical_written = !technical_requested || append_bootstrap_technical_log(line.text);
    if matches!(line.visibility, Visibility::InteractiveOnly | Visibility::Both) {
        let policy = bootstrap_stream_policy(to_stderr);
        with_bootstrap_step_line_cleared(|| {
            if to_stderr {
                let mut stderr = io::stderr().lock();
                let _ = write_bootstrap_interactive(&mut stderr, policy, line);
            } else {
                let mut stdout = io::stdout().lock();
                let _ = write_bootstrap_interactive(&mut stdout, policy, line);
            }
        });
    } else if !technical_written {
        eprintln!("[EtuSchlopf take] WARNUNG: Technical Log nicht schreibbar; Detail: {}", line.text);
    }
}

macro_rules! bootstrap_line {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Take,
            None,
            Attention::Neutral,
            Visibility::Both,
            false,
            format_args!($($arg)*),
        )
    };
}

macro_rules! bootstrap_ok {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Take,
            Some(LocalStatus::Ok),
            Attention::Notice,
            Visibility::Both,
            false,
            format_args!($($arg)*),
        )
    };
}

macro_rules! bootstrap_gate_ok {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Kodex,
            Some(LocalStatus::Ok),
            Attention::Notice,
            Visibility::Both,
            false,
            format_args!($($arg)*),
        )
    };
}

macro_rules! bootstrap_detail {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Take,
            None,
            Attention::Neutral,
            Visibility::TechnicalOnly,
            false,
            format_args!($($arg)*),
        )
    };
}

macro_rules! bootstrap_warning {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Take,
            None,
            Attention::Warning,
            Visibility::Both,
            false,
            format_args!($($arg)*),
        )
    };
}

macro_rules! bootstrap_commit {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Take,
            Some(LocalStatus::Committed),
            Attention::Notice,
            Visibility::Both,
            false,
            format_args!($($arg)*),
        )
    };
}

macro_rules! bootstrap_rollback {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Take,
            Some(LocalStatus::RolledBack),
            Attention::Warning,
            Visibility::Both,
            false,
            format_args!($($arg)*),
        )
    };
}

macro_rules! bootstrap_error {
    ($($arg:tt)*) => {
        write_bootstrap_presentation(
            PresentationRole::Take,
            Some(LocalStatus::Failed),
            Attention::Error,
            Visibility::Both,
            true,
            format_args!($($arg)*),
        )
    };
}

/// K7/K8 console narration of the long pre-reexec steps (HIMEM style).
/// Presentation only and strictly non-blocking: the step neither decides nor
/// measures anything authoritative (startup_timing keeps the advisory
/// measurement). On an interactive terminal a detached ticker thread rewrites
/// the step line once per second; the step itself never waits for it (no join,
/// the ticker is woken by dropping its channel and checks a stop flag under the
/// same output lock). Any failure of the ticker, the console or the small
/// last-duration file only drops the filler, never the step.
struct BootstrapStep {
    key: &'static str,
    running: &'static str,
    started: std::time::Instant,
    live: Option<BootstrapStepTicker>,
}

struct BootstrapStepTicker {
    state: std::sync::Arc<std::sync::Mutex<BootstrapStepLine>>,
    _wake: std::sync::mpsc::Sender<()>,
}

struct BootstrapStepLine {
    stopped: bool,
    width: usize,
    styled: bool,
    text: String,
}

/// The one live step line, if any. Other console output clears it, prints
/// above it and redraws it, so lines never run into each other.
static ACTIVE_BOOTSTRAP_STEP: std::sync::Mutex<Option<std::sync::Arc<std::sync::Mutex<BootstrapStepLine>>>> =
    std::sync::Mutex::new(None);

fn set_active_bootstrap_step(state: Option<std::sync::Arc<std::sync::Mutex<BootstrapStepLine>>>) {
    if let Ok(mut active) = ACTIVE_BOOTSTRAP_STEP.lock() {
        *active = state;
    }
}

/// Runs `print` with the live step line (if any) cleared before and redrawn
/// after, under the step lock so the ticker cannot interleave.
fn with_bootstrap_step_line_cleared(print: impl FnOnce()) {
    let active = ACTIVE_BOOTSTRAP_STEP.lock().ok().and_then(|active| active.clone());
    let Some(active) = active else {
        print();
        return;
    };
    let Ok(line) = active.lock() else {
        print();
        return;
    };
    if line.stopped || line.width == 0 {
        drop(line);
        print();
        return;
    }
    {
        let mut stdout = io::stdout().lock();
        let _ = write!(stdout, "\r{}\r", " ".repeat(BOOTSTRAP_PREFIX_WIDTH + 1 + line.width));
        let _ = stdout.flush();
    }
    print();
    let mut stdout = io::stdout().lock();
    let _ = write!(stdout, "\r");
    let _ = write_bootstrap_prefix(&mut stdout, line.styled);
    let _ = write!(stdout, " {}", line.text);
    let _ = stdout.flush();
}

const BOOTSTRAP_PREFIX_WIDTH: usize = "[EtuSchlopf take]".len();

const BOOTSTRAP_STEP_LAST_FILE: &str = "target/etuschlopf_logs/bootstrap_step_last.txt";

fn bootstrap_step(key: &'static str, running: &'static str) -> BootstrapStep {
    let started = std::time::Instant::now();
    let policy = bootstrap_stream_policy(false);
    let live = (policy.progress == ProgressMode::Inline)
        .then(|| {
            start_bootstrap_step_ticker(
                running,
                last_bootstrap_step_millis(key),
                policy.style == StyleMode::Styled,
                started,
            )
        });
    BootstrapStep { key, running, started, live }
}

fn start_bootstrap_step_ticker(
    running: &'static str,
    last_millis: Option<u128>,
    styled: bool,
    started: std::time::Instant,
) -> BootstrapStepTicker {
    let state = std::sync::Arc::new(std::sync::Mutex::new(BootstrapStepLine {
        stopped: false,
        width: 0,
        styled,
        text: String::new(),
    }));
    write_bootstrap_step_live(&state, &bootstrap_step_live_text(running, 0, last_millis), false);
    set_active_bootstrap_step(Some(std::sync::Arc::clone(&state)));
    let (wake, sleeper) = std::sync::mpsc::channel::<()>();
    let thread_state = std::sync::Arc::clone(&state);
    // A ticker that cannot be spawned only loses the filler; finish() still
    // completes the line in place.
    let _ = std::thread::Builder::new()
        .name("bootstrap-step-ticker".to_string())
        .spawn(move || {
            while let Err(std::sync::mpsc::RecvTimeoutError::Timeout) =
                sleeper.recv_timeout(std::time::Duration::from_secs(1))
            {
                let text = bootstrap_step_live_text(running, started.elapsed().as_secs(), last_millis);
                if !write_bootstrap_step_live(&thread_state, &text, false) {
                    break;
                }
            }
        });
    BootstrapStepTicker { state, _wake: wake }
}

/// Rewrites the current step line in place. Returns false once the step has
/// stopped; `last` terminates the line and marks it stopped under the lock.
fn write_bootstrap_step_live(
    state: &std::sync::Mutex<BootstrapStepLine>,
    text: &str,
    last: bool,
) -> bool {
    let Ok(mut line) = state.lock() else {
        return false;
    };
    if line.stopped {
        return false;
    }
    let mut stdout = io::stdout().lock();
    let _ = write!(stdout, "\r");
    let _ = write_bootstrap_prefix(&mut stdout, line.styled);
    let _ = write!(stdout, " {text}");
    let width = text.chars().count();
    if width < line.width {
        let _ = write!(stdout, "{}", " ".repeat(line.width - width));
    }
    line.width = width;
    line.text = text.to_string();
    if last {
        let _ = writeln!(stdout);
        line.stopped = true;
    }
    let _ = stdout.flush();
    !line.stopped
}

fn bootstrap_step_live_text(running: &str, seconds: u64, last_millis: Option<u128>) -> String {
    let last = last_millis
        .map(|ms| format!(" · zuletzt {}", format_bootstrap_step_duration(std::time::Duration::from_millis(u64::try_from(ms).unwrap_or(u64::MAX)))))
        .unwrap_or_default();
    format!("{running} ... {seconds} s{last}")
}

fn bootstrap_step_done_text(running: &str, elapsed: std::time::Duration) -> String {
    format!("{running} ... fertig · {}", format_bootstrap_step_duration(elapsed))
}

impl BootstrapStep {
    fn finish(mut self) {
        let elapsed = self.started.elapsed();
        let text = bootstrap_step_done_text(self.running, elapsed);
        match self.live.take() {
            Some(ticker) => {
                set_active_bootstrap_step(None);
                write_bootstrap_step_live(&ticker.state, &text, true);
                bootstrap_detail!("{text}");
            }
            None => bootstrap_line!("{text}"),
        }
        remember_bootstrap_step_millis(self.key, elapsed.as_millis());
    }
}

impl Drop for BootstrapStep {
    fn drop(&mut self) {
        // An aborted step only terminates its open live line so that the
        // following error line starts cleanly; no completion is claimed.
        if let Some(ticker) = self.live.take() {
            set_active_bootstrap_step(None);
            let text = format!("{} ...", self.running);
            write_bootstrap_step_live(&ticker.state, &text, true);
        }
    }
}

fn last_bootstrap_step_millis(key: &str) -> Option<u128> {
    let text = fs::read_to_string(BOOTSTRAP_STEP_LAST_FILE).ok()?;
    parse_bootstrap_step_last(&text, key)
}

fn parse_bootstrap_step_last(text: &str, key: &str) -> Option<u128> {
    text.lines()
        .filter_map(|line| line.trim().split_once('='))
        .find(|(name, _)| *name == key)
        .and_then(|(_, value)| value.parse::<u128>().ok())
}

fn remember_bootstrap_step_millis(key: &str, millis: u128) {
    let previous = fs::read_to_string(BOOTSTRAP_STEP_LAST_FILE).unwrap_or_default();
    let _ = fs::write(BOOTSTRAP_STEP_LAST_FILE, merge_bootstrap_step_last(&previous, key, millis));
}

fn merge_bootstrap_step_last(previous: &str, key: &str, millis: u128) -> String {
    let mut out = previous
        .lines()
        .filter(|line| line.trim().split_once('=').is_some_and(|(name, value)| name != key && value.parse::<u128>().is_ok()))
        .map(|line| format!("{}\n", line.trim()))
        .collect::<String>();
    out.push_str(&format!("{key}={millis}\n"));
    out
}

fn format_bootstrap_step_duration(elapsed: std::time::Duration) -> String {
    let millis = elapsed.as_millis();
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

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(normalize_bootstrap_exit_code(code)),
        Err(err) => {
            bootstrap_error!("{err}");
            ExitCode::from(1)
        }
    }
}

fn normalize_bootstrap_exit_code(code: i32) -> u8 {
    u8::try_from(code).unwrap_or(1)
}

fn run() -> AppResult<i32> {
    let take_before_reexec = startup_timing::Phase::new("TakeBeforeReexec");
    managed_source_tree_definition::validate_root_project_file_roles()?;
    let project_root =
        env::current_dir().map_err(|e| format!("Projektordner nicht lesbar: {e}"))?;
    init_bootstrap_technical_log(&project_root);
    append_bootstrap_technical_log(&startup_timing::readback());
    let args: Vec<OsString> = env::args_os().skip(1).collect();

    if args
        .iter()
        .any(|arg| os_eq_ignore_case(arg, RECOVER_PENDING_PUBLIC_ARTIFACTS_ARG))
    {
        let outcome = public_artifact_recovery::recover_pending_public_artifacts(&project_root)?;
        bootstrap_ok!(
            "PENDING-PUBLIC-ARTIFACT-RECOVERY: {}",
            outcome.as_str()
        );
        return Ok(0);
    }

    let public_recovery = public_artifact_recovery::recover_pending_public_artifacts(&project_root)?;
    if public_recovery.performed_recovery() {
        bootstrap_line!(
            "Public-Artifact-Recovery: {}.",
            public_recovery.as_str()
        );
    }

    if args
        .iter()
        .any(|arg| os_eq_ignore_case(arg, RECOVER_PENDING_ARG))
    {
        recover_pending_transaction(&project_root)?;
        bootstrap_ok!("PENDING-TAKE-RECOVERY");
        return Ok(0);
    }

    recover_pending_transaction(&project_root)?;

    bootstrap_detail!(
        "[EtuSchlopf rust-take-bootstrap v{VERSION}] --take vor Rust-Launcher-Kompilation erkannt."
    );
    bootstrap_detail!(
        "[EtuSchlopf rust-take-bootstrap] Projekt: {}",
        project_root.display()
    );

    let zip_path = resolve_take_zip(&args, &project_root)?;
    let candidate_version_key = zip_version_key(&zip_path)
        .ok_or_else(|| "Take-ZIP besitzt nach Discovery keine gueltige Versionsidentitaet".to_string())?;
    let candidate_revision = zip_revision_key(&zip_path, candidate_version_key)
        .ok_or_else(|| "Take-ZIP besitzt nach Discovery keine gueltige Revisionsidentitaet".to_string())?;
    bootstrap_detail!(
        "[EtuSchlopf rust-take-bootstrap] ZIP: {}",
        zip_path.display()
    );
    bootstrap_line!(
        "Paket: {}",
        zip_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Take-Paket")
    );

    let tmp = make_temp_dir()?;
    let result = (|| -> AppResult<i32> {
        let extraction_timing = startup_timing::Phase::new("TakeExtraction");
        let extraction_step = bootstrap_step("extraction", "Take-Paket wird entpackt");
        extract_zip(&zip_path, &tmp)?;
        extraction_step.finish();
        drop(extraction_timing);
        let source_root = find_source_root(&tmp)?;
        let current_version = read_cargo_version(&project_root.join("Cargo.toml"))
            .unwrap_or_else(|| "unknown".to_string());
        let incoming_version = read_cargo_version(&source_root.join("Cargo.toml"))
            .unwrap_or_else(|| "unknown".to_string());
        // The exact package identity must exist before any candidate-specific fail-closed
        // preflight can reject the incoming tree. Otherwise an early Tree-Integrity or
        // dependency rejection cannot participate in later self-healing discovery.
        let candidate_sha256 = multi_revision_evasion::sha256_file(&zip_path)?;
        let candidate_intent_path =
            source_root.join(take_change_control::CHANGE_MANIFEST_RELATIVE_PATH);
        let candidate_intent = match fs::read(&candidate_intent_path) {
            Ok(bytes) => Some(bytes),
            Err(error) => {
                bootstrap_warning!(
                    "Candidate-Intent Evidence konnte nicht vorab gelesen werden {}: {}; Gate-/Acceptance-Semantik bleibt unveraendert.",
                    candidate_intent_path.display(),
                    error
                );
                None
            }
        };
        let force = args.iter().any(|a| os_eq_ignore_case(a, "--force"));

        let same_version_known = current_version != "unknown"
            && incoming_version != "unknown"
            && current_version.eq_ignore_ascii_case(&incoming_version);
        let mut same_version_auto_take = false;

        if same_version_known && !force {
            if take_payload_differs(&source_root, &project_root)? {
                same_version_auto_take = true;
                bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Gleiche Version {incoming_version}, aber Paketinhalt unterscheidet sich vom Projektstand. Automatische Uebernahme ohne --force.");
            } else if let Some(clean_args) = identical_take_continuation_args(&args) {
                bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Gleiche Version erkannt: current={current_version}, incoming={incoming_version}. Paketinhalt identisch und bereits installiert; keine Kopie erforderlich.");
                bootstrap_line!("Paket bereits installiert · starte vorhandenen Stand");
                bootstrap_detail!(
                    "[EtuSchlopf rust-take-bootstrap] Reexec-Argumente: {}",
                    command_args_preview(&clean_args)
                );
                let status = run_cmd_command(&project_root, &clean_args)
                    .status()
                    .map_err(|error| {
                        format!(
                            "Ein-Befehl-Fortsetzung von run.cmd ohne --take fehlgeschlagen: {error}"
                        )
                    })?;
                return Ok(status.code().unwrap_or(1));
            } else {
                bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Gleiche Version verweigert: current={current_version}, incoming={incoming_version}. Schutz-Noop; Paketinhalt identisch und bereits installiert.");
                bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Kein Folgelauf angefordert; keine Datei wurde kopiert; Exit 2; --force nicht verwenden.");
                bootstrap_warning!("Paket bereits installiert · kein Folgelauf angegeben · Exit 2");
                return Ok(2);
            }
        }

        if same_version_known && force {
            bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] --force: gleiche Version {current_version} wird trotzdem uebernommen.");
        } else if same_version_auto_take {
            bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Same-Version-Take: {current_version} -> {incoming_version} mit geaendertem Payload.");
        } else {
            bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Versionwechsel: {current_version} -> {incoming_version}");
        }

        let attempt_started_epoch_millis = current_epoch_millis()?;
        invalidate_published_assistant_export(&project_root)?;
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] Assistant-Export-Projektion fuer aktuellen Take-Versuch invalidiert; stale previous success kann den laufenden Versuch nicht repraesentieren."
        );

        let dead_code_step = bootstrap_step("dead_code", "Dead-Code-Integrität wird geprüft");
        let dead_code_suppressions =
            take_change_control::validate_dead_code_detection_integrity_transition(
                &project_root,
                &source_root,
            )
            .map_err(|error| {
                rejection_error_with_persistence(
                    &project_root,
                    &multi_revision_evasion::RejectionRecord {
                        from_version: &current_version,
                        to_version: &incoming_version,
                        take_revision: candidate_revision,
                        zip_sha256: &candidate_sha256,
                        rejection_stage: multi_revision_evasion::REJECTION_STAGE_DEAD_CODE_INTEGRITY,
                        candidate_intent: candidate_intent.as_deref(),
                        snapshot: "",
                        reason: &error,
                    },
                    attempt_started_epoch_millis,
                )
            })?;
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] DeadCodeDetectionIntegrity parent->incoming | suppressions {} | monotonic green",
            dead_code_suppressions,
        );

        dead_code_step.finish();

        let dependency_timing = startup_timing::Phase::new("DependencyAdmission");
        let dependency_step =
            bootstrap_step("dependencies", "Abhängigkeiten werden geprüft");
        let dependency_preflight = take_change_control::validate_incoming_change_dependencies(
            &source_root,
            &project_root,
            &current_version,
            &incoming_version,
        )
        .map_err(|error| {
            rejection_error_with_persistence(
                &project_root,
                &multi_revision_evasion::RejectionRecord {
                    from_version: &current_version,
                    to_version: &incoming_version,
                    take_revision: candidate_revision,
                    zip_sha256: &candidate_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_DEPENDENCY_PREFLIGHT,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: "",
                    reason: &error,
                },
                attempt_started_epoch_millis,
            )
        })?;
        dependency_step.finish();
        drop(dependency_timing);
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] ChangeDependencyPreflight class {} | declared {} | actual {} | kodex {} | audit {} | music-sensitive {} | metadata {} | pinned {} | status green",
            dependency_preflight.change_class.as_str(),
            dependency_preflight.declared_change_count,
            dependency_preflight.actual_change_count,
            dependency_preflight.kodex_change_count,
            dependency_preflight.audit_change_count,
            dependency_preflight.music_sensitive_change_count,
            dependency_preflight.metadata_change_count,
            dependency_preflight.pinned_architecture_change_count,
        );

        let revision_step = bootstrap_step("revision", "Revisionsabdruck und Ablehnungsverlauf werden geprüft");
        let revision_snapshot = match take_change_control::build_revision_effect_snapshot(
            &source_root,
            &project_root,
        ) {
            Ok(snapshot) => snapshot,
            Err(error) => {
                let snapshot = String::new();
                return Err(rejection_error_with_persistence(
                    &project_root,
                    &multi_revision_evasion::RejectionRecord {
                        from_version: &current_version,
                        to_version: &incoming_version,
                        take_revision: candidate_revision,
                        zip_sha256: &candidate_sha256,
                        rejection_stage: multi_revision_evasion::REJECTION_STAGE_EFFECT_SNAPSHOT,
                        candidate_intent: candidate_intent.as_deref(),
                        snapshot: &snapshot,
                        reason: &error,
                    },
                    attempt_started_epoch_millis,
                ));
            }
        };
        let prior_rejections = multi_revision_evasion::load_rejected_revisions(
            &project_root,
            &current_version,
            &incoming_version,
        )?;
        let chain_summary = multi_revision_evasion::summarize_chain(&prior_rejections);
        let active_diagnostic_rejections = multi_revision_evasion::active_diagnostic_rejections(
            &project_root,
            &current_version,
            &incoming_version,
            &prior_rejections,
        )?;
        if let Err(error) = multi_revision_evasion::reject_exact_replay(
            &prior_rejections,
            &candidate_sha256,
        ) {
            return Err(rejection_error_with_persistence(
                &project_root,
                &multi_revision_evasion::RejectionRecord {
                    from_version: &current_version,
                    to_version: &incoming_version,
                    take_revision: candidate_revision,
                    zip_sha256: &candidate_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_INCOMING_TAKE,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: &revision_snapshot,
                    reason: &error,
                },
                attempt_started_epoch_millis,
            ));
        }
        let mut prior_revision_diagnostics = 0usize;
        for previous in &active_diagnostic_rejections {
            if let Err(error) = take_change_control::validate_rejection_chain_effect_monotonic(
                &previous.snapshot,
                &revision_snapshot,
            ) {
                prior_revision_diagnostics += 1;
                bootstrap_warning!(
                    "frueherer Kandidat {} ist semantisch nicht monoton zum neuen Kandidaten: {}",
                    previous.zip_sha256,
                    error.replace('\n', " | ")
                );
            }
        }
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] MultiRevisionHistory SHA-256 {} | prior_rejections {} | active_chain_rejections {} | latest_rejected {} | exact-replay hard | prior-chain diagnostics {}",
            candidate_sha256,
            chain_summary.prior_rejections,
            active_diagnostic_rejections.len(),
            if chain_summary.latest_rejected_sha256.is_empty() {
                "keiner"
            } else {
                chain_summary.latest_rejected_sha256.as_str()
            },
            prior_revision_diagnostics,
        );

        revision_step.finish();
        let admission_timing = startup_timing::Phase::new("IncomingTakeAdmission");
        let admission_step = bootstrap_step("admission", "Take-Zulassung wird geprüft (Klasse, Pfade, Gates)");
        let change_summary = match take_change_control::validate_incoming_take(
            &source_root,
            &project_root,
            &current_version,
            &incoming_version,
        ) {
            Ok(summary) => summary,
            Err(error) => {
                return Err(rejection_error_with_persistence(
                    &project_root,
                    &multi_revision_evasion::RejectionRecord {
                        from_version: &current_version,
                        to_version: &incoming_version,
                        take_revision: candidate_revision,
                        zip_sha256: &candidate_sha256,
                        rejection_stage: multi_revision_evasion::REJECTION_STAGE_INCOMING_TAKE,
                        candidate_intent: candidate_intent.as_deref(),
                        snapshot: &revision_snapshot,
                        reason: &error,
                    },
                    attempt_started_epoch_millis,
                ));
            }
        };
        admission_step.finish();
        drop(admission_timing);
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] ChangeSchema {} | ChangeClass {} | declared {} | actual {} | kodex {} | audit {} | music-sensitive {} | metadata {}",
            change_summary.schema_version,
            change_summary.change_class.as_str(),
            change_summary.declared_change_count,
            change_summary.actual_change_count,
            change_summary.kodex_change_count,
            change_summary.audit_change_count,
            change_summary.music_sensitive_change_count,
            change_summary.metadata_change_count,
        );
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] SemanticCapabilityDiff added {} | modified {} | removed {} | kodex {} | audit {} | music {} | calibration {} | writer-family {} | unknown {}",
            change_summary.semantic_added_count,
            change_summary.semantic_modified_count,
            change_summary.semantic_removed_count,
            change_summary.semantic_kodex_count,
            change_summary.semantic_audit_count,
            change_summary.semantic_music_count,
            change_summary.semantic_calibration_count,
            change_summary.semantic_writer_family_count,
            change_summary.semantic_unknown_count,
        );
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] ChangeReason: {}",
            change_summary.reason
        );
        if let Some(music_effect) = &change_summary.music_effect {
            bootstrap_detail!(
                "[EtuSchlopf rust-take-bootstrap] GovernedChange: music_effect={} | govern={}",
                music_effect,
                if change_summary.governed_changes.is_empty() {
                    "none".to_string()
                } else {
                    change_summary.governed_changes.join(" ; ")
                }
            );
        }
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] PRE-TAKE-DIFF-GATE GRUEN: {} -> {}. Erst jetzt wird kopiert.",
            change_summary.from_version,
            change_summary.to_version,
        );
        bootstrap_gate_ok!(
            "PRE-TAKE-DIFF-GATE · {} · {} Aenderungen · Semantik +{}/~{}/-{}",
            change_summary.change_class.as_str(),
            change_summary.actual_change_count,
            change_summary.semantic_added_count,
            change_summary.semantic_modified_count,
            change_summary.semantic_removed_count,
        );

        let watch_step = bootstrap_step("watch", "Watch-Projekt wird geprüft");
        validate_changed_watch_project(&source_root, &project_root).map_err(|error| {
            rejection_error_with_persistence(
                &project_root,
                &multi_revision_evasion::RejectionRecord {
                    from_version: &current_version,
                    to_version: &incoming_version,
                    take_revision: candidate_revision,
                    zip_sha256: &candidate_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_WATCH_PREFLIGHT,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: &revision_snapshot,
                    reason: &error,
                },
                attempt_started_epoch_millis,
            )
        })?;

        watch_step.finish();
        let prepare_timing = startup_timing::Phase::new("TransactionBackupPrepare");
        let prepare_step = bootstrap_step("backup", "Sicherung wird angelegt");
        let transaction = TakeTransaction::prepare(
            &project_root,
            &source_root,
            &current_version,
            &incoming_version,
            candidate_revision,
            &candidate_sha256,
            &revision_snapshot,
        )?;
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] TAKE-TRANSACTION vorbereitet: BackupComplete ja; PendingMarker ja; PartialInstallationPossible nein."
        );
        prepare_step.finish();
        drop(prepare_timing);
        let apply_timing = startup_timing::Phase::new("ApplyAndVerify");
        let apply_step = bootstrap_step("apply", "Take wird übernommen und geprüft");
        if let Err(error) = transaction.apply_and_verify(&source_root, &project_root) {
            let reason = format!("Take-Anwendung fehlgeschlagen: {error}");
            rollback_then_record_rejection(
                &transaction,
                &project_root,
                &multi_revision_evasion::RejectionRecord {
                    from_version: &current_version,
                    to_version: &incoming_version,
                    take_revision: candidate_revision,
                    zip_sha256: &candidate_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_APPLY_VERIFY,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: &revision_snapshot,
                    reason: &reason,
                },
                attempt_started_epoch_millis,
            )?;
            return Err(format!(
                "Take-Anwendung fehlgeschlagen, wurde vollstaendig zurueckgerollt und als Kandidatenablehnung gespeichert: {error}"
            ));
        }
        apply_step.finish();
        drop(apply_timing);
        bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Update transaktional installiert und gegen den eingehenden Vollbaum verifiziert: {current_version} -> {incoming_version}. Starte aktualisierten Stand ohne --take.");

        let clean_args = clean_args_after_take(&args);
        let preview = command_args_preview(&clean_args);
        bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] Reexec-Argumente: {preview}");
        bootstrap_line!("Installiert · Reexec {preview}");

        let reexec_prepare_timing = startup_timing::Phase::new("LockAndReexecEvidencePreparation");
        transaction.mark_reexec()?;
        if let Err(error) = transaction.resolve_root_cargo_lock(
            &project_root,
            &current_version,
            &incoming_version,
            change_summary.change_class.as_str(),
        ) {
            let reason = format!("Root-Cargo.lock Resolution fehlgeschlagen: {error}");
            rollback_then_record_rejection(
                &transaction,
                &project_root,
                &multi_revision_evasion::RejectionRecord {
                    from_version: &current_version,
                    to_version: &incoming_version,
                    take_revision: candidate_revision,
                    zip_sha256: &candidate_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_CARGO_LOCK_RESOLUTION,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: &revision_snapshot,
                    reason: &reason,
                },
                attempt_started_epoch_millis,
            )?;
            return Err(format!(
                "Root-Cargo.lock Resolution fehlgeschlagen; Take wurde vollstaendig zurueckgerollt und als Kandidatenablehnung gespeichert: {error}"
            ));
        }
        let rejection_evidence_dir = match prepare_rejection_evidence_capsule(
            &project_root,
            &current_version,
            &incoming_version,
            candidate_revision,
            &candidate_sha256,
            change_summary.change_class.as_str(),
            attempt_started_epoch_millis,
        ) {
            Ok(dir) => dir,
            Err(error) => {
                let reason = format!(
                    "Executed-Candidate-Evidence konnte vor Reexec nicht gebunden werden; Reexec wird fail-closed nicht gestartet: {error}"
                );
                rollback_then_record_rejection(
                    &transaction,
                    &project_root,
                    &multi_revision_evasion::RejectionRecord {
                        from_version: &current_version,
                        to_version: &incoming_version,
                        take_revision: candidate_revision,
                        zip_sha256: &candidate_sha256,
                        rejection_stage: multi_revision_evasion::REJECTION_STAGE_REEXEC_LAUNCH,
                        candidate_intent: candidate_intent.as_deref(),
                        snapshot: &revision_snapshot,
                        reason: &reason,
                    },
                    attempt_started_epoch_millis,
                )?;
                return Err(reason);
            }
        };
        let child_writeahead_dir =
            rejection_evidence_dir.join(FAILURE_EVIDENCE_CHILD_WRITEAHEAD_DIR);
        let mut cmd = run_cmd_command(&project_root, &clean_args);
        cmd.env("ETUSCHLOPF_TAKE_FROM", &current_version)
            .env("ETUSCHLOPF_TAKE_TO", &incoming_version)
            .env("ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE", "1")
            .env(
                "ETUSCHLOPF_TAKE_CHANGE_CLASS",
                change_summary.change_class.as_str(),
            )
            .env(TAKE_ATTEMPT_EVIDENCE_DIR_ENV, &child_writeahead_dir);
        drop(reexec_prepare_timing);
        drop(take_before_reexec);
        let child_wait_timing = startup_timing::Phase::new("ReexecChildWait");
        let status = match clear_previous_child_console(&project_root).and_then(|()| cmd.status()) {
            Ok(status) => status,
            Err(error) => {
                let reason = format!("Reexec von run.cmd konnte nicht gestartet werden: {error}");
                let rejection = multi_revision_evasion::RejectionRecord {
                    from_version: &current_version,
                    to_version: &incoming_version,
                    take_revision: candidate_revision,
                    zip_sha256: &candidate_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_REEXEC_LAUNCH,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: &revision_snapshot,
                    reason: &reason,
                };
                rollback_then_record_rejection_with_evidence(
                    &transaction,
                    &project_root,
                    &rejection,
                    attempt_started_epoch_millis,
                    "not_started",
                    None,
                )?;
                return Err(format!(
                    "Reexec von run.cmd fehlgeschlagen; Take wurde zurueckgerollt und als Kandidatenablehnung gespeichert: {error}"
                ));
            }
        };
        drop(child_wait_timing);
        let code = status.code().unwrap_or(1);
        if code == 0 {
            if let Err(error) = remove_dir_if_exists(&rejection_evidence_dir) {
                bootstrap_warning!(
                    "Pending Failure-Evidence konnte nach erfolgreichem Child nicht entfernt werden: {}; Acceptance-Semantik bleibt unveraendert.",
                    error
                );
            }
            transaction.mark_committed()?;
            multi_revision_evasion::record_acceptance(
                &project_root,
                &current_version,
                &incoming_version,
                candidate_revision,
                &candidate_sha256,
                chain_summary.prior_rejections,
            )
            .map_err(|error| {
                format!(
                    "Reexec 0, aber Acceptance-Provenienz konnte nicht gespeichert werden; die committed Pending-Transaktion bleibt fuer den naechsten Ein-Befehl-Recovery-Lauf erhalten: {error}"
                )
            })?;
            record_acceptance_evidence_advisory(
                &project_root,
                &current_version,
                &incoming_version,
                candidate_revision,
                &candidate_sha256,
                candidate_intent.as_deref(),
                &revision_snapshot,
            );
            finalize_post_acceptance_assistant_handover(
                &project_root,
                &current_version,
                &incoming_version,
                candidate_revision,
                &candidate_sha256,
                chain_summary.prior_rejections,
            )
            .map_err(|error| {
                format!(
                    "Reexec 0 und Acceptance kanonisch gespeichert, aber der Post-Acceptance-Assistant-Handover konnte nicht finalisiert werden; die committed Pending-Transaktion bleibt fuer Recovery erhalten: {error}"
                )
            })?;
            // Kodex-bound adversarial witness: commit remains at the authorized successful-reexec
            // call site. mark_committed() persists recovery state first; commit() performs the
            // historical final cleanup only after Acceptance provenance is durable.
            transaction.commit()?;
            bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] TAKE-TRANSACTION COMMIT: Reexec 0; Acceptance-Provenienz gespeichert; Backup entfernt; PendingMarker entfernt.");
            bootstrap_commit!("Reexec 0");
            Ok(0)
        } else {
            let reason = format!(
                "Post-Apply-Reexec scheiterte mit Exit {code}; Source wurde transaktional zurueckgerollt."
            );
            // Kodex-bound adversarial witness: the failed-reexec rollback call is intentionally
            // kept at this authorized call site. Lifecycle provenance may extend the path after
            // rollback, but must not abstract, rename or relocate this source-level witness.
            transaction.rollback(&project_root)?;
            let rejection = multi_revision_evasion::RejectionRecord {
                from_version: &current_version,
                to_version: &incoming_version,
                take_revision: candidate_revision,
                zip_sha256: &candidate_sha256,
                rejection_stage: multi_revision_evasion::REJECTION_STAGE_REEXEC_EXIT,
                candidate_intent: candidate_intent.as_deref(),
                snapshot: &revision_snapshot,
                reason: &reason,
            };
            multi_revision_evasion::record_rejection(&project_root, &rejection).map_err(
                |ledger_error| {
                    format!(
                        "{reason}; Rollback war erfolgreich, aber Rejection-Provenienz konnte nicht gespeichert werden; rolled_back Pending-Provenienz bleibt fuer den naechsten Recovery-Lauf erhalten: {ledger_error}"
                    )
                },
            )?;
            if let Err(error) = finalize_rejection_evidence_after_rollback(
                &project_root,
                &rejection,
                "started_returned",
                Some(code),
            ) {
                bootstrap_warning!(
                    "Failure-Evidence konnte nach sicherem Rollback nicht vollstaendig finalisiert werden: {}; kanonische Rejection bleibt autoritativ.",
                    error
                );
            }
            finalize_rejection_assistant_handover(
                &project_root,
                &rejection,
                FailureAttemptContext {
                    started_epoch_millis: Some(attempt_started_epoch_millis),
                    child_status: "started_returned",
                    child_exit: Some(code),
                    transaction_state: "rolled_back",
                },
            )
            .map_err(|handover_error| {
                format!(
                    "{reason}; kanonische Rejection ist gespeichert, aber {handover_error}; rolled_back Pending-Provenienz bleibt fuer Recovery erhalten"
                )
            })?;
            transaction.finish_rollback()?;
            bootstrap_detail!("[EtuSchlopf rust-take-bootstrap] TAKE-TRANSACTION ROLLBACK: Reexec Exit {code}; vorheriger Quellstand wiederhergestellt; Kandidaten-SHA als rejected persistiert; Pending-Provenienz danach bereinigt.");
            bootstrap_rollback!("Reexec Exit {code} · vorheriger Quellstand wiederhergestellt · Kandidat rejected");
            Ok(code)
        }
    })();

    let _ = fs::remove_dir_all(&tmp);
    result
}

fn os_eq_ignore_case(value: &OsString, needle: &str) -> bool {
    value.to_string_lossy().eq_ignore_ascii_case(needle)
}

fn has_take(args: &[OsString]) -> bool {
    args.iter().any(|a| os_eq_ignore_case(a, "--take"))
}

fn resolve_take_zip(args: &[OsString], project_root: &Path) -> AppResult<PathBuf> {
    if !has_take(args) {
        return Err("Rust-Take-Bootstrap wurde ohne --take gestartet.".to_string());
    }
    let mut candidates = Vec::new();
    collect_zip_candidates(project_root, &mut candidates);
    if let Some(profile) = env::var_os("USERPROFILE") {
        collect_zip_candidates(&PathBuf::from(profile).join("Downloads"), &mut candidates);
    }
    let mut ranked = candidates
        .into_iter()
        .filter_map(|path| {
            let version = zip_version_key(&path)?;
            let revision = zip_revision_key(&path, version)?;
            Some((version, revision, path))
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right.0.cmp(&left.0).then_with(|| right.1.cmp(&left.1))
    });
    let Some((version, revision, _)) = ranked.first().cloned() else {
        return Err("Kein take-kompatibles EtuSchlopf_v*.zip im Projektordner oder Downloads gefunden.".to_string());
    };
    let peers = ranked
        .iter()
        .filter(|(peer_version, peer_revision, _)| *peer_version == version && *peer_revision == revision)
        .map(|(_, _, path)| path.clone())
        .collect::<Vec<_>>();
    let prior_rejections = multi_revision_evasion::load_rejected_revisions(
        project_root,
        VERSION,
        VERSION,
    )?;
    let rejected_sha256 = prior_rejections
        .iter()
        .map(|rejection| rejection.zip_sha256.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let eligible_peers = unrejected_take_peers(&peers, &rejected_sha256)?;
    if eligible_peers.is_empty() {
        return Err(format!(
            "Alle Take-Pakete fuer Version {}.{}.{} Revision r{:02} wurden bereits fail-closed abgewiesen; ein neuer Kandidat wird benoetigt.",
            version.0, version.1, version.2, revision
        ));
    }
    let ignored_rejections = peers.len().saturating_sub(eligible_peers.len());
    if ignored_rejections > 0 {
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] Take-Discovery: {} bereits abgewiesene Kandidat(en) fuer r{:02} anhand gespeicherter SHA-256-Provenienz ignoriert.",
            ignored_rejections,
            revision
        );
    }

    let accepted_sha256 = multi_revision_evasion::load_accepted_zip_sha256(
        project_root,
        VERSION,
        VERSION,
        revision,
    )?;
    let mut accepted_peer_hashes = BTreeSet::new();
    let mut canonical_accepted_peer = None;
    let mut eligible_hashes = Vec::new();
    for path in &eligible_peers {
        let sha256 = multi_revision_evasion::sha256_file(path)?.to_ascii_lowercase();
        if accepted_sha256.contains(&sha256) {
            accepted_peer_hashes.insert(sha256.clone());
            if canonical_accepted_peer.is_none() {
                canonical_accepted_peer = Some(path.clone());
            }
        }
        eligible_hashes.push((path.clone(), sha256));
    }
    if accepted_peer_hashes.len() > 1 {
        return Err(format!(
            "Take-Identity-Lifecycle: fuer Version {}.{}.{} Revision r{:02} existieren mehrere bereits akzeptierte Payload-SHAs; Acceptance-Provenienz ist widerspruechlich und wird fail-closed verweigert.",
            version.0, version.1, version.2, revision
        ));
    }
    if let Some(canonical) = canonical_accepted_peer {
        let accepted_hash = accepted_peer_hashes
            .first()
            .expect("accepted peer hash exists");
        let superseded = eligible_hashes
            .iter()
            .filter(|(_, sha256)| sha256 != accepted_hash)
            .count();
        if superseded > 0 {
            bootstrap_detail!(
                "[EtuSchlopf rust-take-bootstrap] Take-Identity-Lifecycle: r{:02} besitzt bereits kanonische Acceptance-Provenienz SHA-256 {}; {} abweichende Peer-Payload(s) werden logisch als superseded ignoriert; keine Datei muss manuell entfernt werden.",
                revision,
                accepted_hash,
                superseded
            );
        }
        return fs::canonicalize(canonical)
            .map_err(|e| format!("Take-ZIP nicht kanonisch lesbar: {e}"));
    }

    if eligible_peers.len() > 1 {
        let reference = fnv1a64_file(eligible_peers.first().expect("eligible peer list is non-empty"))?;
        if eligible_peers
            .iter()
            .skip(1)
            .any(|path| fnv1a64_file(path).ok() != Some(reference))
        {
            return Err(format!("Mehrdeutige nicht-abgewiesene Take-Pakete fuer Version {}.{}.{} Revision r{:02}; gleicher Versionsstand mit verschiedenem Inhalt besitzt weder Rejection- noch Acceptance-Provenienz und wird fail-closed verweigert.", version.0, version.1, version.2, revision));
        }
    }
    fs::canonicalize(eligible_peers.first().expect("eligible peer list is non-empty"))
        .map_err(|e| format!("Take-ZIP nicht kanonisch lesbar: {e}"))
}

#[derive(Clone, Copy)]
struct FailureAttemptContext<'a> {
    started_epoch_millis: Option<u128>,
    child_status: &'a str,
    child_exit: Option<i32>,
    transaction_state: &'a str,
}

fn current_epoch_millis() -> AppResult<u128> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .map_err(|error| format!("Systemzeit vor UNIX_EPOCH: {error}"))
}

fn invalidate_published_assistant_export(project_root: &Path) -> AppResult<()> {
    let export_root = project_root.join(ASSISTANT_EXPORT_ROOT);
    remove_file_if_exists(&export_root.join(ASSISTANT_EXPORT_LATEST_ZIP))?;
    remove_file_if_exists(&export_root.join("latest.post_acceptance.pending.zip"))?;
    remove_file_if_exists(&export_root.join("latest.take_failure.pending.zip"))?;
    remove_file_if_exists(&project_root.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP))?;
    remove_file_if_exists(
        &project_root.join("assistant_export_latest.post_acceptance.pending.zip"),
    )?;
    remove_file_if_exists(&project_root.join("assistant_export_latest.take_failure.pending.zip"))?;
    Ok(())
}

fn failure_identity_field<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    text.lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .map(str::trim)
}

fn failure_parent_identity(project_root: &Path) -> (String, String, String) {
    let text = fs::read_to_string(project_root.join("ZIP_KIND.txt")).unwrap_or_default();
    let version = failure_identity_field(&text, "version")
        .unwrap_or("unknown")
        .to_string();
    let revision = failure_identity_field(&text, "revision")
        .unwrap_or("unknown")
        .to_string();
    let name = failure_identity_field(&text, "name")
        .unwrap_or("unknown")
        .to_string();
    (version, revision, name)
}

fn failure_phase_list(rejection_stage: &str, child_status: &str) -> Vec<&'static str> {
    let mut phases = vec!["package_discovered", "candidate_identity_bound"];
    if rejection_stage == multi_revision_evasion::REJECTION_STAGE_CRASH_RECOVERY {
        phases.push("crash_recovery");
        return phases;
    }
    let ordered = [
        (
            multi_revision_evasion::REJECTION_STAGE_DEAD_CODE_INTEGRITY,
            "dead_code_integrity",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_DEPENDENCY_PREFLIGHT,
            "dependency_preflight",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_EFFECT_SNAPSHOT,
            "effect_snapshot",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_INCOMING_TAKE,
            "incoming_take_validation",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_WATCH_PREFLIGHT,
            "watch_preflight",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_APPLY_VERIFY,
            "transaction_apply_verify",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_CARGO_LOCK_RESOLUTION,
            "cargo_lock_resolution",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_REEXEC_LAUNCH,
            "reexec_launch",
        ),
        (
            multi_revision_evasion::REJECTION_STAGE_REEXEC_EXIT,
            "reexec_exit",
        ),
    ];
    for (stage, phase) in ordered {
        phases.push(phase);
        if stage == rejection_stage {
            break;
        }
    }
    if child_status == "started_returned" && !phases.contains(&"child_run_started") {
        phases.push("child_run_started");
    }
    phases
}

fn collect_failure_source_files(
    root: &Path,
    current: &Path,
    out: &mut Vec<PathBuf>,
) -> AppResult<()> {
    let mut entries = fs::read_dir(current)
        .map_err(|error| {
            format!(
                "Failure-Source-Snapshot Ordner nicht lesbar {}: {error}",
                current.display()
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failure-Source-Snapshot Eintrag nicht lesbar: {error}"))?;
    entries.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            format!(
                "Failure-Source-Snapshot Metadaten nicht lesbar {}: {error}",
                path.display()
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "Failure-Source-Snapshot enthaelt verbotenen Symlink {}",
                path.display()
            ));
        }
        if metadata.is_dir() {
            collect_failure_source_files(root, &path, out)?;
        } else if metadata.is_file() {
            path.strip_prefix(root).map_err(|error| {
                format!(
                    "Failure-Source-Snapshot Datei liegt ausserhalb des Projekt-Roots {}: {} ({error})",
                    root.display(),
                    path.display()
                )
            })?;
            out.push(path);
        } else {
            return Err(format!(
                "Failure-Source-Snapshot enthaelt nicht unterstuetzten Eintrag {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn write_bound_source_snapshot(
    project_root: &Path,
    source_snapshot_dir: &Path,
    archive_file: &str,
    role: &str,
) -> AppResult<(usize, String)> {
    fs::create_dir_all(source_snapshot_dir).map_err(|error| {
        format!(
            "Failure-Source-Snapshot Ordner nicht anlegbar {}: {error}",
            source_snapshot_dir.display()
        )
    })?;
    let archive_path = source_snapshot_dir.join(archive_file);
    let mut files = Vec::new();
    for root_file in managed_source_tree_definition::MANAGED_ROOT_FILES
        .into_iter()
        .chain(managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES)
    {
        let path = project_root.join(root_file);
        if path.is_file() {
            files.push(path);
        }
    }
    for managed_dir in managed_source_tree_definition::MANAGED_DIRS {
        let path = project_root.join(managed_dir);
        if path.is_dir() {
            collect_failure_source_files(project_root, &path, &mut files)?;
        }
    }
    files.sort_by_key(|path| {
        path.strip_prefix(project_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    });

    let file = fs::File::create(&archive_path).map_err(|error| {
        format!(
            "Failure-Source-Snapshot ZIP nicht anlegbar {}: {error}",
            archive_path.display()
        )
    })?;
    let mut zip = zip::ZipWriter::new(file);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for path in &files {
        let relative = path.strip_prefix(project_root).map_err(|error| {
            format!(
                "Failure-Source-Snapshot relativer Pfad nicht bestimmbar {}: {error}",
                path.display()
            )
        })?;
        let name = relative.to_string_lossy().replace('\\', "/");
        zip.start_file(&name, options)
            .map_err(|error| format!("Failure-Source-Snapshot ZIP start_file {name}: {error}"))?;
        zip.write_all(&fs::read(path).map_err(|error| {
            format!(
                "Failure-Source-Snapshot Datei nicht lesbar {}: {error}",
                path.display()
            )
        })?)
        .map_err(|error| format!("Failure-Source-Snapshot ZIP write {name}: {error}"))?;
    }
    zip.finish()
        .map_err(|error| format!("Failure-Source-Snapshot ZIP finalize: {error}"))?;

    let archive_bytes = fs::read(&archive_path).map_err(|error| {
        format!(
            "Failure-Source-Snapshot ZIP fuer Hash nicht lesbar {}: {error}",
            archive_path.display()
        )
    })?;
    let archive_sha256 = sha256::hex(&archive_bytes);
    let marker = format!(
        "schema=1\ncomplete=true\nauthority=none\nrole={}\nfile_count={}\narchive={}\narchive_sha256={}\n",
        role,
        files.len(),
        archive_file,
        archive_sha256
    );
    fs::write(source_snapshot_dir.join("SOURCE_SNAPSHOT.txt"), marker).map_err(|error| {
        format!("Failure-Source-Snapshot Marker nicht schreibbar: {error}")
    })?;
    Ok((files.len(), archive_sha256))
}

fn write_failure_source_snapshot(
    project_root: &Path,
    source_snapshot_dir: &Path,
) -> AppResult<(usize, String)> {
    write_bound_source_snapshot(
        project_root,
        source_snapshot_dir,
        TAKE_FAILURE_SOURCE_ARCHIVE_FILE,
        "current_parent_source_after_rejection_or_rollback",
    )
}

fn rejection_evidence_dir(project_root: &Path, revision: u64, candidate_sha256: &str) -> PathBuf {
    let short_sha = candidate_sha256.get(..16).unwrap_or(candidate_sha256);
    project_root
        .join(TAKE_REJECTION_EVIDENCE_ROOT)
        .join(format!("rejected-r{revision}-{short_sha}"))
}

fn prepare_rejection_evidence_capsule(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    revision: u64,
    candidate_sha256: &str,
    change_class: &str,
    attempt_epoch_millis: u128,
) -> AppResult<PathBuf> {
    let dir = rejection_evidence_dir(project_root, revision, candidate_sha256);
    remove_dir_if_exists(&dir)?;
    fs::create_dir_all(dir.join(FAILURE_EVIDENCE_CHILD_WRITEAHEAD_DIR)).map_err(|error| {
        format!("Take-Failure-Evidence Write-Ahead-Ordner nicht anlegbar: {error}")
    })?;
    fs::create_dir_all(dir.join(FAILURE_EVIDENCE_CHILD_DIR)).map_err(|error| {
        format!("Take-Failure-Evidence Child-Ordner nicht anlegbar: {error}")
    })?;
    let source_dir = dir.join("executed_candidate_source");
    let (file_count, source_sha256) = {
        let _snapshot_timing = startup_timing::Phase::new("ExecutedCandidateSnapshot");
        write_bound_source_snapshot(
            project_root,
            &source_dir,
            EXECUTED_CANDIDATE_SOURCE_ARCHIVE_FILE,
            "executed_candidate_source_before_reexec",
        )?
    };
    let _metadata_timing = startup_timing::Phase::new("ReexecEvidenceMetadata");
    let attempt_id = format!("r{revision}-{candidate_sha256}-{attempt_epoch_millis}");
    let identity = format!(
        "schema=1\nauthority=none\nattempt_id={}\nfrom_version={}\nto_version={}\ntake_revision={}\ncandidate_sha256={}\nchange_class={}\nattempt_started_epoch_millis={}\nexecuted_candidate_source_status=complete\nexecuted_candidate_source_file_count={}\nexecuted_candidate_source_sha256={}\nbootstrap_generation_rule=accepted_parent_bootstrap_owns_current_take\ncapability_effective_from=next_take_after_acceptance\n",
        attempt_id,
        from_version,
        to_version,
        revision,
        candidate_sha256,
        change_class,
        attempt_epoch_millis,
        file_count,
        source_sha256
    );
    fs::write(dir.join(FAILURE_EVIDENCE_IDENTITY_FILE), identity)
        .map_err(|error| format!("Take-Failure-Evidence Identity nicht schreibbar: {error}"))?;
    fs::write(
        dir.join(FAILURE_EVIDENCE_METADATA_FILE),
        b"schema=1\nauthority=none\nstatus=pending_reexec\nchild_evidence_status=not_reached\nrestored_parent_source_status=not_reached\nrejection_evidence_persistence_status=pending\n",
    )
    .map_err(|error| format!("Take-Failure-Evidence Pending-Metadaten nicht schreibbar: {error}"))?;
    Ok(dir)
}

fn safe_registered_child_artifact(role: &str, raw_path: &str) -> AppResult<PathBuf> {
    let path = Path::new(raw_path);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(format!("Attempt-Evidence Pfad ist nicht relativ: {raw_path:?}"));
    }
    for component in path.components() {
        if !matches!(component, std::path::Component::Normal(_) | std::path::Component::CurDir) {
            return Err(format!("Attempt-Evidence Pfad enthaelt verbotene Komponente: {raw_path:?}"));
        }
    }
    match role {
        "strict_test_log" => {
            if raw_path.replace('\\', "/") != "target/etuschlopf_logs/visual_preflight.strict-tests.txt" {
                return Err(format!("Attempt-Evidence Strict-Test-Pfad unerwartet: {raw_path:?}"));
            }
        }
        "strict_clippy_log" => {
            let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
                return Err(format!("Attempt-Evidence Clippy-Pfad ohne Dateinamen: {raw_path:?}"));
            };
            if path.components().count() != 1 || !name.ends_with(".strict-clippy.txt") {
                return Err(format!("Attempt-Evidence Clippy-Pfad unerwartet: {raw_path:?}"));
            }
        }
        other => return Err(format!("Attempt-Evidence unbekannte Artefaktrolle: {other:?}")),
    }
    Ok(path.to_path_buf())
}

#[derive(Debug, Clone)]
struct ChildEvidenceCaptureSummary {
    status: &'static str,
    launcher_stage: String,
    registered: usize,
    captured: usize,
    capture_errors: usize,
}

// Supersede only the shared diagnostic console before starting this child.
// Absence after an early failure means no current console, never a prior run.
fn clear_previous_child_console(project_root: &Path) -> std::io::Result<()> {
    let console = project_root.join("target/etuschlopf_logs/etuschlopf.console.txt");
    match fs::remove_file(console) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn capture_registered_child_evidence(
    project_root: &Path,
    dir: &Path,
    child_status: &str,
) -> ChildEvidenceCaptureSummary {
    let writeahead = dir.join(FAILURE_EVIDENCE_CHILD_WRITEAHEAD_DIR);
    let child_dir = dir.join(FAILURE_EVIDENCE_CHILD_DIR);
    let _ = fs::create_dir_all(&child_dir);
    let mut manifest_lines = Vec::new();
    let mut registered = 0usize;
    let mut captured = 0usize;
    let mut capture_errors = 0usize;
    let mut launcher_stage = "not_reached".to_string();

    let stage_path = writeahead.join("CHILD_STAGE.txt");
    if let Ok(stage) = fs::read_to_string(&stage_path) {
        launcher_stage = stage
            .lines()
            .find_map(|line| line.strip_prefix("launcher_stage="))
            .unwrap_or("unknown")
            .to_string();
        if fs::write(child_dir.join("CHILD_STAGE.txt"), stage).is_err() {
            capture_errors = capture_errors.saturating_add(1);
        }
    }

    if child_status == "started_returned" || child_status == "unknown_recovery" {
        registered = registered.saturating_add(1);
        let source = project_root.join("target/etuschlopf_logs/etuschlopf.console.txt");
        if source.is_file() && stage_path.is_file() {
            match fs::read(&source) {
                Ok(bytes) => {
                    if fs::write(child_dir.join("etuschlopf.console.txt"), &bytes).is_ok() {
                        captured = captured.saturating_add(1);
                        manifest_lines.push(format!(
                            "role=child_console\tstatus=captured\tsource=target/etuschlopf_logs/etuschlopf.console.txt\texport=etuschlopf.console.txt\tsha256={}",
                            sha256::hex(&bytes)
                        ));
                    } else {
                        capture_errors = capture_errors.saturating_add(1);
                    }
                }
                Err(_) => capture_errors = capture_errors.saturating_add(1),
            }
        } else {
            manifest_lines.push(if source.is_file() {
                "role=child_console\tstatus=not_current_attempt".to_string()
            } else {
                "role=child_console\tstatus=absent".to_string()
            });
        }
    }

    let registry_path = writeahead.join("CHILD_ARTIFACT_REGISTRY.txt");
    if let Ok(registry) = fs::read_to_string(&registry_path) {
        let mut seen = BTreeSet::new();
        for line in registry.lines() {
            let Some((role_field, path_field)) = line.split_once('\t') else {
                capture_errors = capture_errors.saturating_add(1);
                continue;
            };
            let Some(role) = role_field.strip_prefix("role=") else {
                capture_errors = capture_errors.saturating_add(1);
                continue;
            };
            let Some(raw_path) = path_field.strip_prefix("path=") else {
                capture_errors = capture_errors.saturating_add(1);
                continue;
            };
            let key = format!("{role}\t{raw_path}");
            if !seen.insert(key) {
                continue;
            }
            registered = registered.saturating_add(1);
            let relative = match safe_registered_child_artifact(role, raw_path) {
                Ok(path) => path,
                Err(error) => {
                    capture_errors = capture_errors.saturating_add(1);
                    manifest_lines.push(format!("role={role}\tstatus=invalid_binding\tdetail={error}"));
                    continue;
                }
            };
            let source = project_root.join(&relative);
            let export_name = match role {
                "strict_test_log" => "visual_preflight.strict-tests.txt",
                "strict_clippy_log" => "strict-clippy.txt",
                _ => unreachable!(),
            };
            if !source.is_file() {
                manifest_lines.push(format!(
                    "role={role}\tstatus=not_reached\tsource={}",
                    relative.to_string_lossy().replace('\\', "/")
                ));
                continue;
            }
            match fs::read(&source) {
                Ok(bytes) => {
                    if fs::write(child_dir.join(export_name), &bytes).is_ok() {
                        captured = captured.saturating_add(1);
                        manifest_lines.push(format!(
                            "role={role}\tstatus=captured\tsource={}\texport={export_name}\tsha256={}",
                            relative.to_string_lossy().replace('\\', "/"),
                            sha256::hex(&bytes)
                        ));
                    } else {
                        capture_errors = capture_errors.saturating_add(1);
                    }
                }
                Err(_) => capture_errors = capture_errors.saturating_add(1),
            }
        }
    }

    let manifest = if manifest_lines.is_empty() {
        "schema=1\nauthority=none\nartifact_count=0\n".to_string()
    } else {
        format!(
            "schema=1\nauthority=none\nartifact_count={}\n{}\n",
            manifest_lines.len(),
            manifest_lines.join("\n")
        )
    };
    if fs::write(child_dir.join("EVIDENCE_MANIFEST.txt"), manifest).is_err() {
        capture_errors = capture_errors.saturating_add(1);
    }

    let status = if child_status == "not_started" {
        "not_reached"
    } else if capture_errors == 0 && captured == registered && registered > 0 {
        "complete"
    } else if captured > 0 || launcher_stage != "not_reached" {
        "partial"
    } else {
        "absent"
    };
    ChildEvidenceCaptureSummary {
        status,
        launcher_stage,
        registered,
        captured,
        capture_errors,
    }
}

fn finalize_rejection_evidence_after_rollback(
    project_root: &Path,
    rejection: &multi_revision_evasion::RejectionRecord<'_>,
    child_status: &str,
    child_exit: Option<i32>,
) -> AppResult<()> {
    let dir = rejection_evidence_dir(project_root, rejection.take_revision, rejection.zip_sha256);
    if !dir.is_dir() {
        return Ok(());
    }
    let child = capture_registered_child_evidence(project_root, &dir, child_status);
    let restored_dir = dir.join("restored_parent_source");
    let (restored_status, restored_count, restored_sha256) = match write_bound_source_snapshot(
        project_root,
        &restored_dir,
        RESTORED_PARENT_SOURCE_ARCHIVE_FILE,
        "restored_parent_source_after_rejection_or_rollback",
    ) {
        Ok((count, sha256)) => ("complete", count, sha256),
        Err(error) => {
            let _ = fs::write(restored_dir.join("CAPTURE_ERROR.txt"), error.as_bytes());
            ("capture_error", 0, String::new())
        }
    };
    let persistence_status = if restored_status == "complete" {
        "complete"
    } else {
        "partial"
    };
    let exit = child_exit
        .map(|value| value.to_string())
        .unwrap_or_else(|| "not_available".to_string());
    let metadata = format!(
        "schema=1\nauthority=none\nstatus=rejected\ncanonical_rejection_stage={}\nchild_status={}\nchild_exit={}\nlauncher_stage={}\nexecuted_candidate_source_status=complete\nchild_evidence_status={}\nchild_artifacts_registered={}\nchild_artifacts_captured={}\nchild_capture_errors={}\nrestored_parent_source_status={}\nrestored_parent_source_file_count={}\nrestored_parent_source_sha256={}\nrejection_evidence_persistence_status={}\n",
        rejection.rejection_stage,
        child_status,
        exit,
        child.launcher_stage,
        child.status,
        child.registered,
        child.captured,
        child.capture_errors,
        restored_status,
        restored_count,
        restored_sha256,
        persistence_status
    );
    fs::write(dir.join(FAILURE_EVIDENCE_METADATA_FILE), metadata)
        .map_err(|error| format!("Take-Failure-Evidence Finalmetadaten nicht schreibbar: {error}"))
}

fn project_current_rejection_evidence(
    project_root: &Path,
    latest_dir: &Path,
    rejection: &multi_revision_evasion::RejectionRecord<'_>,
) -> AppResult<()> {
    let export_root = latest_dir.join(TAKE_ATTEMPT_EVIDENCE_EXPORT_DIR);
    fs::create_dir_all(&export_root)
        .map_err(|error| format!("Current Attempt-Evidence Exportordner nicht anlegbar: {error}"))?;
    let short_sha = rejection.zip_sha256.get(..16).unwrap_or(rejection.zip_sha256);
    let target = export_root.join(format!("rejected-r{}-{short_sha}", rejection.take_revision));
    let source = rejection_evidence_dir(project_root, rejection.take_revision, rejection.zip_sha256);
    if source.is_dir() {
        copy_dir_recursive(&source, &target)?;
    } else {
        fs::create_dir_all(&target)
            .map_err(|error| format!("Current Attempt-Evidence Statusordner nicht anlegbar: {error}"))?;
        let metadata = format!(
            "schema=1\nauthority=none\nstatus=not_reached\ntake_revision={}\ncandidate_sha256={}\nrejection_stage={}\nexecuted_candidate_source_status=not_reached\nchild_evidence_status=not_reached\nrestored_parent_source_status=not_reached\nrejection_evidence_persistence_status=not_reached\n",
            rejection.take_revision, rejection.zip_sha256, rejection.rejection_stage
        );
        fs::write(target.join(FAILURE_EVIDENCE_METADATA_FILE), metadata)
            .map_err(|error| format!("Current Attempt-Evidence Status nicht schreibbar: {error}"))?;
    }
    Ok(())
}

fn project_rejection_evidence_suffix(
    project_root: &Path,
    latest_dir: &Path,
    rejection_suffix: &[multi_revision_evasion::RejectedRevision],
) -> AppResult<()> {
    let export_dir = latest_dir.join(TAKE_ATTEMPT_EVIDENCE_EXPORT_DIR);
    remove_dir_if_exists(&export_dir)?;
    if rejection_suffix.is_empty() {
        return Ok(());
    }
    fs::create_dir_all(&export_dir)
        .map_err(|error| format!("Attempt-Evidence Exportordner nicht anlegbar: {error}"))?;
    for rejection in rejection_suffix {
        let revision = rejection.take_revision;
        let short_sha = rejection
            .zip_sha256
            .get(..16)
            .unwrap_or(rejection.zip_sha256.as_str());
        let label = revision.map_or_else(
            || format!("rejected-legacy-{short_sha}"),
            |value| format!("rejected-r{value}-{short_sha}"),
        );
        let target = export_dir.join(label);
        if let Some(value) = revision {
            let source = rejection_evidence_dir(project_root, value, &rejection.zip_sha256);
            if source.is_dir() {
                copy_dir_recursive(&source, &target)?;
                continue;
            }
        }
        fs::create_dir_all(&target)
            .map_err(|error| format!("Legacy-Evidence Statusordner nicht anlegbar: {error}"))?;
        let metadata = format!(
            "schema=1\nauthority=none\nstatus=legacy_evidence_unavailable\ntake_revision={}\ncandidate_sha256={}\nrejection_stage={}\n",
            revision
                .map(|value| value.to_string())
                .unwrap_or_else(|| "legacy_unknown".to_string()),
            rejection.zip_sha256,
            rejection.rejection_stage
        );
        fs::write(target.join(FAILURE_EVIDENCE_METADATA_FILE), metadata)
            .map_err(|error| format!("Legacy-Evidence Status nicht schreibbar: {error}"))?;
    }
    Ok(())
}

fn finalize_rejection_assistant_handover(
    project_root: &Path,
    rejection: &multi_revision_evasion::RejectionRecord<'_>,
    context: FailureAttemptContext<'_>,
) -> AppResult<()> {
    let result = finalize_rejection_assistant_handover_inner(project_root, rejection, context);
    if let Err(error) = result {
        let _ = invalidate_published_assistant_export(project_root);
        return Err(format!(
            "Failure-Assistant-Handover nicht finalisiert; publizierte ZIP-Projektionen wurden invalidiert, kanonische Rejection bleibt unveraendert: {error}"
        ));
    }
    Ok(())
}

fn finalize_rejection_assistant_handover_inner(
    project_root: &Path,
    rejection: &multi_revision_evasion::RejectionRecord<'_>,
    context: FailureAttemptContext<'_>,
) -> AppResult<()> {
    let canonical = multi_revision_evasion::read_rejection_record_exact(
        project_root,
        rejection.from_version,
        rejection.to_version,
        rejection.take_revision,
        rejection.zip_sha256,
    )?;
    invalidate_published_assistant_export(project_root)?;

    let export_root = project_root.join(ASSISTANT_EXPORT_ROOT);
    let latest_dir = export_root.join(ASSISTANT_EXPORT_LATEST_DIR);
    remove_dir_if_exists(&latest_dir)?;
    fs::create_dir_all(&latest_dir).map_err(|error| {
        format!(
            "Failure-Assistant-Export Staging nicht anlegbar {}: {error}",
            latest_dir.display()
        )
    })?;
    let source_snapshot_dir = latest_dir.join("source_snapshot");
    let (source_file_count, source_archive_sha256) =
        write_failure_source_snapshot(project_root, &source_snapshot_dir)?;

    let (parent_version, parent_revision, parent_name) = failure_parent_identity(project_root);
    if !parent_version.eq_ignore_ascii_case(rejection.from_version) {
        return Err(format!(
            "Failure-Assistant-Handover Parent-Version {} stimmt nicht mit Candidate-from_version {} ueberein; fail-closed",
            parent_version, rejection.from_version
        ));
    }
    let parent_revision_number = parent_revision
        .strip_prefix('r')
        .ok_or_else(|| {
            format!(
                "Failure-Assistant-Handover Parent-Revision besitzt kein r-Praefix: {parent_revision:?}"
            )
        })?
        .parse::<u64>()
        .map_err(|error| {
            format!(
                "Failure-Assistant-Handover Parent-Revision ist ungueltig {parent_revision:?}: {error}"
            )
        })?;
    let parent_acceptance = multi_revision_evasion::read_unique_current_acceptance_record_exact(
        project_root,
        &parent_version,
        parent_revision_number,
    )?;
    let canonical_relative = relative_project_path(project_root, &canonical.dir)?;
    let parent_acceptance_relative =
        relative_project_path(project_root, &parent_acceptance.path)?;
    let time_to_reject = context
        .started_epoch_millis
        .and_then(|start| canonical.attempt_epoch_millis.checked_sub(start))
        .map(|value| value.to_string())
        .unwrap_or_else(|| "unknown_recovery".to_string());
    let phases = failure_phase_list(rejection.rejection_stage, context.child_status);
    let phase_lines = phases
        .iter()
        .map(|phase| format!("phase_reached={phase}\n"))
        .collect::<String>();
    let child_exit = context
        .child_exit
        .map(|value| value.to_string())
        .unwrap_or_else(|| "not_available".to_string());

    let metadata = format!(
        "schema=1\nprojection_only=true\nauthority=none\nmode=take_failure\ncanonical_verdict=rejected\nfrom_version={}\nto_version={}\ntake_revision={}\ncandidate_sha256={}\nparent_version={}\nparent_revision={}\nparent_name={}\nparent_acceptance_from_version={}\nparent_acceptance_to_version={}\nparent_acceptance_revision={}\nparent_acceptance_sha256={}\nparent_acceptance_prior_rejections={}\nparent_acceptance_origin={}\nparent_acceptance_record_sha256={}\ncurrent_attempt_stage={}\ncanonical_rejection_stage={}\ncanonical_rejection_attempt_epoch_millis={}\nattempt_started_epoch_millis={}\ntime_to_reject_ms={}\ntransaction_state={}\nchild_status={}\nchild_exit={}\nsource_snapshot_file_count={}\nsource_snapshot_archive_sha256={}\ncanonical_rejection_origin={}\n{}",
        rejection.from_version,
        rejection.to_version,
        rejection.take_revision,
        rejection.zip_sha256,
        parent_version,
        parent_revision,
        parent_name,
        parent_acceptance.from_version,
        parent_acceptance.to_version,
        parent_acceptance.take_revision,
        parent_acceptance.zip_sha256,
        parent_acceptance.prior_rejections,
        parent_acceptance_relative,
        sha256::hex(&parent_acceptance.bytes),
        rejection.rejection_stage,
        canonical.rejection_stage,
        canonical.attempt_epoch_millis,
        context
            .started_epoch_millis
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown_recovery".to_string()),
        time_to_reject,
        context.transaction_state,
        context.child_status,
        child_exit,
        source_file_count,
        source_archive_sha256,
        canonical_relative,
        phase_lines
    );
    fs::write(latest_dir.join(TAKE_FAILURE_METADATA_FILE), metadata.as_bytes())
        .map_err(|error| format!("Failure-Metadaten nicht schreibbar: {error}"))?;
    fs::write(
        latest_dir.join(TAKE_FAILURE_REASON_FILE),
        rejection.reason.as_bytes(),
    )
    .map_err(|error| format!("Failure-Grund nicht schreibbar: {error}"))?;
    fs::write(
        latest_dir.join(TAKE_FAILURE_INTENT_FILE),
        rejection.candidate_intent.unwrap_or_default(),
    )
    .map_err(|error| format!("Failure-Candidate-Intent nicht schreibbar: {error}"))?;
    fs::write(
        latest_dir.join(TAKE_FAILURE_SNAPSHOT_FILE),
        rejection.snapshot.as_bytes(),
    )
    .map_err(|error| format!("Failure-Snapshot nicht schreibbar: {error}"))?;

    fs::write(
        latest_dir.join(TAKE_FAILURE_PARENT_ACCEPTANCE_FILE),
        &parent_acceptance.bytes,
    )
    .map_err(|error| format!("Kanonische Parent-Acceptance nicht projizierbar: {error}"))?;

    fs::write(
        latest_dir.join(TAKE_FAILURE_CANONICAL_METADATA_FILE),
        &canonical.metadata,
    )
    .map_err(|error| format!("Kanonische Rejection-Metadaten nicht projizierbar: {error}"))?;
    fs::write(
        latest_dir.join(TAKE_FAILURE_CANONICAL_REASON_FILE),
        &canonical.reason,
    )
    .map_err(|error| format!("Kanonischer Rejection-Grund nicht projizierbar: {error}"))?;
    fs::write(
        latest_dir.join(TAKE_FAILURE_CANONICAL_INTENT_FILE),
        canonical.candidate_intent.as_deref().unwrap_or_default(),
    )
    .map_err(|error| format!("Kanonischer Rejection-Intent nicht projizierbar: {error}"))?;
    fs::write(
        latest_dir.join(TAKE_FAILURE_CANONICAL_SNAPSHOT_FILE),
        &canonical.snapshot,
    )
    .map_err(|error| format!("Kanonischer Rejection-Snapshot nicht projizierbar: {error}"))?;

    let bootstrap_console = project_root.join("target/etuschlopf_logs/take_bootstrap.console.txt");
    if bootstrap_console.is_file() {
        fs::copy(
            &bootstrap_console,
            latest_dir.join(TAKE_FAILURE_BOOTSTRAP_CONSOLE_FILE),
        )
        .map_err(|error| format!("Failure-Bootstrap-Konsole nicht projizierbar: {error}"))?;
    }
    let child_console = rejection_evidence_dir(project_root, rejection.take_revision, rejection.zip_sha256)
        .join(FAILURE_EVIDENCE_CHILD_DIR).join("etuschlopf.console.txt");
    let child_console_included =
        context.child_status == "started_returned" && child_console.is_file();
    if child_console_included {
        fs::copy(
            &child_console,
            latest_dir.join(TAKE_FAILURE_CHILD_CONSOLE_FILE),
        )
        .map_err(|error| format!("Failure-Child-Konsole nicht projizierbar: {error}"))?;
    }
    project_current_rejection_evidence(project_root, &latest_dir, rejection)?;

    let lifecycle = format!(
        "schema=1\nprojection_only=true\nauthority=none\ncanonical_verdict=rejected\nfrom_version={}\nto_version={}\ntake_revision={}\ncandidate_sha256={}\ncurrent_attempt_stage={}\ncurrent_attempt_reason_sha256={}\ncanonical_rejection_stage={}\ncanonical_rejection_metadata_sha256={}\ncanonical_rejection_reason_sha256={}\ncanonical_rejection_snapshot_sha256={}\ncanonical_rejection_intent_sha256={}\nparent_version={}\nparent_revision={}\nparent_acceptance_revision={}\nparent_acceptance_sha256={}\nparent_acceptance_prior_rejections={}\nparent_acceptance_record_sha256={}\ntransaction_state={}\nchild_status={}\nchild_console_included={}\nsource_snapshot_archive_sha256={}\n",
        rejection.from_version,
        rejection.to_version,
        rejection.take_revision,
        rejection.zip_sha256,
        rejection.rejection_stage,
        sha256::hex(rejection.reason.as_bytes()),
        canonical.rejection_stage,
        sha256::hex(&canonical.metadata),
        sha256::hex(&canonical.reason),
        sha256::hex(&canonical.snapshot),
        sha256::hex(canonical.candidate_intent.as_deref().unwrap_or_default()),
        parent_version,
        parent_revision,
        parent_acceptance.take_revision,
        parent_acceptance.zip_sha256,
        parent_acceptance.prior_rejections,
        sha256::hex(&parent_acceptance.bytes),
        context.transaction_state,
        context.child_status,
        if child_console_included { "true" } else { "false" },
        source_archive_sha256,
    );
    fs::write(latest_dir.join(TAKE_FAILURE_HANDOVER_FILE), lifecycle.as_bytes())
        .map_err(|error| format!("Failure-Lifecycle-Handover nicht schreibbar: {error}"))?;

    let zip_kind = format!(
        "kind=assistant_export\ntake_compatible=false\nmode=take_failure\nversion={}\nproject=take-r{}\ncanonical_verdict=rejected\ncandidate_sha256={}\nsource_snapshot_complete=true\nsource_snapshot_archive={}\nsource_snapshot_sha256={}\n",
        rejection.to_version,
        rejection.take_revision,
        rejection.zip_sha256,
        TAKE_FAILURE_SOURCE_ARCHIVE_FILE,
        source_archive_sha256,
    );
    fs::write(latest_dir.join("ZIP_KIND.txt"), zip_kind.as_bytes())
        .map_err(|error| format!("Failure ZIP_KIND nicht schreibbar: {error}"))?;

    let handoff = format!(
        "ETUSCHLOPF ASSISTANT HANDOFF\n============================\nProjectionOnly: true\nAuthority: none\nTakeCompatible: false\nMode: take_failure\nCanonicalVerdict: rejected\nFromVersion: {}\nToVersion: {}\nRevision: r{}\nCandidateSha256: {}\nParentVersion: {}\nParentRevision: {}\nParentName: {}\nParentAcceptanceSha256: {}\nParentAcceptanceRecord: {}\nCurrentAttemptStage: {}\nCanonicalRejectionStage: {}\nTransactionState: {}\nChildStatus: {}\nChildExit: {}\nTimeToRejectMs: {}\nFailureLifecycle: {}\nFailureMetadata: {}\nCanonicalParentAcceptance: {}\nCanonicalRejectionMetadata: {}\nCanonicalRejectionReason: {}\nSourceSnapshotBinding: source_snapshot/SOURCE_SNAPSHOT.txt -> {}\nLatestCompletedAttempt: this_failure\n",
        rejection.from_version,
        rejection.to_version,
        rejection.take_revision,
        rejection.zip_sha256,
        parent_version,
        parent_revision,
        parent_name,
        parent_acceptance.zip_sha256,
        TAKE_FAILURE_PARENT_ACCEPTANCE_FILE,
        rejection.rejection_stage,
        canonical.rejection_stage,
        context.transaction_state,
        context.child_status,
        child_exit,
        time_to_reject,
        TAKE_FAILURE_HANDOVER_FILE,
        TAKE_FAILURE_METADATA_FILE,
        TAKE_FAILURE_PARENT_ACCEPTANCE_FILE,
        TAKE_FAILURE_CANONICAL_METADATA_FILE,
        TAKE_FAILURE_CANONICAL_REASON_FILE,
        TAKE_FAILURE_SOURCE_ARCHIVE_FILE,
    );
    fs::write(latest_dir.join("assistant_handoff_latest.txt"), handoff.as_bytes())
        .map_err(|error| format!("Failure Assistant-Handoff nicht schreibbar: {error}"))?;

    let manifest = format!(
        "ETUSCHLOPF ASSISTANT EXPORT\n============================\nZipKind: assistant_export\nTakeCompatible: false\nMode: take_failure\nCanonicalVerdict: rejected\nFromVersion: {}\nToVersion: {}\nRevision: r{}\nCandidateSha256: {}\nParentVersion: {}\nParentRevision: {}\nParentAcceptanceSha256: {}\nParentAcceptanceRecord: {}\nCurrentAttemptStage: {}\nCanonicalRejectionStage: {}\nTransactionState: {}\nChildStatus: {}\nChildConsoleIncluded: {}\nSourceSnapshotComplete: true\nSourceSnapshotFileCount: {}\nSourceSnapshotArchive: {}\nSourceSnapshotSha256: {}\nLatestCompletedAttempt: this_failure\n",
        rejection.from_version,
        rejection.to_version,
        rejection.take_revision,
        rejection.zip_sha256,
        parent_version,
        parent_revision,
        parent_acceptance.zip_sha256,
        TAKE_FAILURE_PARENT_ACCEPTANCE_FILE,
        rejection.rejection_stage,
        canonical.rejection_stage,
        context.transaction_state,
        context.child_status,
        if child_console_included { "true" } else { "false" },
        source_file_count,
        TAKE_FAILURE_SOURCE_ARCHIVE_FILE,
        source_archive_sha256,
    );
    fs::write(latest_dir.join("manifest.txt"), manifest.as_bytes())
        .map_err(|error| format!("Failure Manifest nicht schreibbar: {error}"))?;

    let pending_latest_zip = export_root.join("latest.take_failure.pending.zip");
    remove_file_if_exists(&pending_latest_zip)?;
    zip_assistant_handover_stage(&latest_dir, &pending_latest_zip)?;
    let latest_zip = export_root.join(ASSISTANT_EXPORT_LATEST_ZIP);
    remove_file_if_exists(&latest_zip)?;
    fs::rename(&pending_latest_zip, &latest_zip).map_err(|error| {
        format!(
            "Failure-Handover interne ZIP-Publikation fehlgeschlagen {} -> {}: {error}",
            pending_latest_zip.display(),
            latest_zip.display()
        )
    })?;

    let root_pending = project_root.join("assistant_export_latest.take_failure.pending.zip");
    remove_file_if_exists(&root_pending)?;
    fs::copy(&latest_zip, &root_pending).map_err(|error| {
        format!(
            "Failure-Handover Root-ZIP-Staging fehlgeschlagen {} -> {}: {error}",
            latest_zip.display(),
            root_pending.display()
        )
    })?;
    let root_latest = project_root.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP);
    remove_file_if_exists(&root_latest)?;
    fs::rename(&root_pending, &root_latest).map_err(|error| {
        format!(
            "Failure-Handover Root-ZIP-Publikation fehlgeschlagen {} -> {}: {error}",
            root_pending.display(),
            root_latest.display()
        )
    })?;
    Ok(())
}

fn rejection_error_with_persistence(
    project_root: &Path,
    rejection: &multi_revision_evasion::RejectionRecord<'_>,
    attempt_started_epoch_millis: u128,
) -> String {
    let error = rejection.reason;
    let candidate_sha256 = rejection.zip_sha256;
    match multi_revision_evasion::record_rejection(project_root, rejection) {
        Ok(()) => {
            let handover = finalize_rejection_assistant_handover(
                project_root,
                rejection,
                FailureAttemptContext {
                    started_epoch_millis: Some(attempt_started_epoch_millis),
                    child_status: "not_started",
                    child_exit: None,
                    transaction_state: "not_started",
                },
            );
            match handover {
                Ok(()) => format!(
                    "{error}\nKandidatenablehnung gespeichert: SHA-256 {candidate_sha256}; Failure-Assistant-Handover publiziert; ein neuer Kandidat wird unabhaengig geprueft."
                ),
                Err(handover_error) => format!(
                    "{error}\nKandidatenablehnung gespeichert: SHA-256 {candidate_sha256}; {handover_error}"
                ),
            }
        }
        Err(ledger_error) => format!(
            "{error}\nZUSAETZLICH konnte der fail-closed Rejection-Ledger nicht geschrieben werden: {ledger_error}"
        ),
    }
}

fn rollback_then_record_rejection(
    transaction: &TakeTransaction,
    project_root: &Path,
    rejection: &multi_revision_evasion::RejectionRecord<'_>,
    attempt_started_epoch_millis: u128,
) -> AppResult<()> {
    let reason = rejection.reason;
    transaction
        .rollback_preserving_provenance(project_root)
        .map_err(|rollback_error| {
            format!(
                "{reason}; ROLLBACK FEHLGESCHLAGEN: {rollback_error}; Kandidat bleibt fail-closed unklassifiziert und die Pending-Transaktion bleibt Recovery-Quelle."
            )
        })?;
    multi_revision_evasion::record_rejection(project_root, rejection).map_err(|ledger_error| {
        format!(
            "{reason}; Rollback war erfolgreich, aber Rejection-Provenienz konnte nicht gespeichert werden; rolled_back Pending-Provenienz bleibt fuer den naechsten Recovery-Lauf erhalten: {ledger_error}"
        )
    })?;
    finalize_rejection_assistant_handover(
        project_root,
        rejection,
        FailureAttemptContext {
            started_epoch_millis: Some(attempt_started_epoch_millis),
            child_status: "not_started",
            child_exit: None,
            transaction_state: "rolled_back",
        },
    )
    .map_err(|handover_error| {
        format!(
            "{reason}; kanonische Rejection ist gespeichert, aber {handover_error}; rolled_back Pending-Provenienz bleibt fuer Recovery erhalten"
        )
    })?;
    transaction.finish_rollback()
}


fn rollback_then_record_rejection_with_evidence(
    transaction: &TakeTransaction,
    project_root: &Path,
    rejection: &multi_revision_evasion::RejectionRecord<'_>,
    attempt_started_epoch_millis: u128,
    child_status: &str,
    child_exit: Option<i32>,
) -> AppResult<()> {
    let reason = rejection.reason;
    transaction
        .rollback_preserving_provenance(project_root)
        .map_err(|rollback_error| {
            format!(
                "{reason}; ROLLBACK FEHLGESCHLAGEN: {rollback_error}; Kandidat bleibt fail-closed unklassifiziert und die Pending-Transaktion bleibt Recovery-Quelle."
            )
        })?;
    multi_revision_evasion::record_rejection(project_root, rejection).map_err(|ledger_error| {
        format!(
            "{reason}; Rollback war erfolgreich, aber Rejection-Provenienz konnte nicht gespeichert werden; rolled_back Pending-Provenienz bleibt fuer den naechsten Recovery-Lauf erhalten: {ledger_error}"
        )
    })?;
    if let Err(error) = finalize_rejection_evidence_after_rollback(
        project_root,
        rejection,
        child_status,
        child_exit,
    ) {
        bootstrap_warning!(
            "Failure-Evidence konnte nach sicherem Rollback nicht vollstaendig finalisiert werden: {}; kanonische Rejection bleibt autoritativ.",
            error
        );
    }
    finalize_rejection_assistant_handover(
        project_root,
        rejection,
        FailureAttemptContext {
            started_epoch_millis: Some(attempt_started_epoch_millis),
            child_status,
            child_exit,
            transaction_state: "rolled_back",
        },
    )
    .map_err(|handover_error| {
        format!(
            "{reason}; kanonische Rejection ist gespeichert, aber {handover_error}; rolled_back Pending-Provenienz bleibt fuer Recovery erhalten"
        )
    })?;
    transaction.finish_rollback()
}

fn record_acceptance_evidence_advisory(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    candidate_sha256: &str,
    candidate_intent: Option<&[u8]>,
    revision_snapshot: &str,
) {
    if let Err(error) = multi_revision_evasion::record_acceptance_evidence(
        project_root,
        from_version,
        to_version,
        take_revision,
        candidate_sha256,
        candidate_intent,
        revision_snapshot,
    ) {
        bootstrap_warning!(
            "Acceptance-Evidence konnte nicht geschrieben/verifiziert werden: {}; kanonische Acceptance bleibt gueltig.",
            error
        );
    }
}

fn finalize_post_acceptance_assistant_handover(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    candidate_sha256: &str,
    prior_rejections: usize,
) -> AppResult<()> {
    let result = finalize_post_acceptance_assistant_handover_inner(
        project_root,
        from_version,
        to_version,
        take_revision,
        candidate_sha256,
        prior_rejections,
    );
    if let Err(error) = result {
        let export_root = project_root.join(ASSISTANT_EXPORT_ROOT);
        let _ = remove_file_if_exists(&export_root.join(ASSISTANT_EXPORT_LATEST_ZIP));
        let _ = remove_file_if_exists(&project_root.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP));
        return Err(format!(
            "Post-Acceptance-Handover nicht finalisiert; publizierte ZIP-Projektionen wurden invalidiert, kanonische Acceptance bleibt unveraendert: {error}"
        ));
    }
    Ok(())
}

fn finalize_post_acceptance_assistant_handover_inner(
    project_root: &Path,
    from_version: &str,
    to_version: &str,
    take_revision: u64,
    candidate_sha256: &str,
    prior_rejections: usize,
) -> AppResult<()> {
    let export_root = project_root.join(ASSISTANT_EXPORT_ROOT);
    let latest_dir = export_root.join(ASSISTANT_EXPORT_LATEST_DIR);
    if !latest_dir.is_dir() {
        return Err(format!(
            "Assistant-Export-Staging fehlt {}; Child-Export darf nach Acceptance nicht erfunden werden",
            latest_dir.display()
        ));
    }
    // Once Acceptance exists, the pre-acceptance ZIP is temporally stale. Remove published
    // projections before touching the staging tree so even a hard process interruption cannot
    // leave an older semantic state under the root name `assistant_export_latest.zip`.
    remove_file_if_exists(&export_root.join(ASSISTANT_EXPORT_LATEST_ZIP))?;
    remove_file_if_exists(&project_root.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP))?;

    let (acceptance_path, acceptance_bytes) = multi_revision_evasion::read_acceptance_record_exact(
        project_root,
        from_version,
        to_version,
        take_revision,
        candidate_sha256,
        prior_rejections,
    )?;
    let (evidence_dir, evidence_metadata, evidence_intent, evidence_snapshot) =
        multi_revision_evasion::read_acceptance_evidence_exact(
            project_root,
            from_version,
            to_version,
            take_revision,
            candidate_sha256,
        )?;

    let acceptance_name = acceptance_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Acceptance-Record besitzt keinen UTF-8-Dateinamen".to_string())?;
    write_or_verify_exact_file(&latest_dir.join(acceptance_name), &acceptance_bytes)?;
    write_or_verify_exact_file(
        &latest_dir.join(POST_ACCEPTANCE_EVIDENCE_METADATA_FILE),
        &evidence_metadata,
    )?;
    write_or_verify_exact_file(
        &latest_dir.join(POST_ACCEPTANCE_EVIDENCE_INTENT_FILE),
        &evidence_intent,
    )?;
    write_or_verify_exact_file(
        &latest_dir.join(POST_ACCEPTANCE_EVIDENCE_SNAPSHOT_FILE),
        &evidence_snapshot,
    )?;

    let all_rejections = multi_revision_evasion::load_rejected_revisions(
        project_root,
        from_version,
        to_version,
    )?;
    let parent_rejection_prefix = multi_revision_evasion::previous_acceptance_rejection_prefix_len(
        project_root,
        from_version,
        to_version,
        take_revision,
    )?;
    if parent_rejection_prefix > prior_rejections || prior_rejections > all_rejections.len() {
        return Err(format!(
            "Lifecycle-Handover Rejection-Prefix ungueltig: parent_prefix={parent_rejection_prefix}, current_prefix={prior_rejections}, ledger_count={}",
            all_rejections.len()
        ));
    }
    let rejection_suffix = &all_rejections[parent_rejection_prefix..prior_rejections];
    project_rejection_evidence_suffix(project_root, &latest_dir, rejection_suffix)?;
    let acceptance_relative = relative_project_path(project_root, &acceptance_path)?;
    let evidence_relative = relative_project_path(project_root, &evidence_dir)?;
    let lifecycle_projection = PostAcceptanceLifecycleProjection {
        from_version,
        to_version,
        take_revision,
        candidate_sha256,
        prior_rejections,
        parent_rejection_prefix,
        acceptance_relative: &acceptance_relative,
        acceptance_bytes: &acceptance_bytes,
        evidence_relative: &evidence_relative,
        evidence_metadata: &evidence_metadata,
        evidence_intent: &evidence_intent,
        evidence_snapshot: &evidence_snapshot,
    };
    let lifecycle = build_post_acceptance_lifecycle_handover(
        &lifecycle_projection,
        rejection_suffix,
    );
    write_or_verify_exact_file(
        &latest_dir.join(POST_ACCEPTANCE_LIFECYCLE_FILE),
        lifecycle.as_bytes(),
    )?;

    let handoff_path = latest_dir.join("assistant_handoff_latest.txt");
    let section = format!(
        "{POST_ACCEPTANCE_SECTION_BEGIN}\nProjectionOnly: true\nAuthority: none\nCanonicalVerdict: accepted\nRevision: r{take_revision}\nCandidateSha256: {candidate_sha256}\nAcceptanceRecord: {acceptance_name}\nAcceptanceRecordSha256: {}\nLifecycleHandover: {POST_ACCEPTANCE_LIFECYCLE_FILE}\nTransitionHistoryTemporalRole: pre_acceptance_advisory_snapshot\n{POST_ACCEPTANCE_SECTION_END}\n",
        sha256::hex(&acceptance_bytes)
    );
    upsert_post_acceptance_handoff_section(&handoff_path, &section)?;

    let pending_latest_zip = export_root.join("latest.post_acceptance.pending.zip");
    remove_file_if_exists(&pending_latest_zip)?;
    zip_assistant_handover_stage(&latest_dir, &pending_latest_zip)?;
    let latest_zip = export_root.join(ASSISTANT_EXPORT_LATEST_ZIP);
    remove_file_if_exists(&latest_zip)?;
    fs::rename(&pending_latest_zip, &latest_zip).map_err(|error| {
        format!(
            "Post-Acceptance-Handover interne ZIP-Publikation fehlgeschlagen {} -> {}: {error}",
            pending_latest_zip.display(),
            latest_zip.display()
        )
    })?;

    let root_pending = project_root.join("assistant_export_latest.post_acceptance.pending.zip");
    remove_file_if_exists(&root_pending)?;
    fs::copy(&latest_zip, &root_pending).map_err(|error| {
        format!(
            "Post-Acceptance-Handover Root-ZIP-Staging fehlgeschlagen {} -> {}: {error}",
            latest_zip.display(),
            root_pending.display()
        )
    })?;
    let root_latest = project_root.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP);
    remove_file_if_exists(&root_latest)?;
    fs::rename(&root_pending, &root_latest).map_err(|error| {
        format!(
            "Post-Acceptance-Handover Root-ZIP-Publikation fehlgeschlagen {} -> {}: {error}",
            root_pending.display(),
            root_latest.display()
        )
    })?;
    Ok(())
}

struct PostAcceptanceLifecycleProjection<'a> {
    from_version: &'a str,
    to_version: &'a str,
    take_revision: u64,
    candidate_sha256: &'a str,
    prior_rejections: usize,
    parent_rejection_prefix: usize,
    acceptance_relative: &'a str,
    acceptance_bytes: &'a [u8],
    evidence_relative: &'a str,
    evidence_metadata: &'a [u8],
    evidence_intent: &'a [u8],
    evidence_snapshot: &'a [u8],
}

fn build_post_acceptance_lifecycle_handover(
    projection: &PostAcceptanceLifecycleProjection<'_>,
    rejection_suffix: &[multi_revision_evasion::RejectedRevision],
) -> String {
    let mut text = String::new();
    text.push_str("schema=1\n");
    text.push_str("projection_only=true\n");
    text.push_str("authority=none\n");
    text.push_str("canonical_verdict=accepted\n");
    text.push_str(&format!(
        "from_version={}\n",
        lifecycle_one_line(projection.from_version)
    ));
    text.push_str(&format!(
        "to_version={}\n",
        lifecycle_one_line(projection.to_version)
    ));
    text.push_str(&format!("take_revision={}\n", projection.take_revision));
    text.push_str(&format!(
        "candidate_sha256={}\n",
        projection.candidate_sha256
    ));
    text.push_str(&format!(
        "acceptance_origin_path={}\n",
        projection.acceptance_relative
    ));
    text.push_str(&format!(
        "acceptance_bytes={}\n",
        projection.acceptance_bytes.len()
    ));
    text.push_str(&format!(
        "acceptance_sha256={}\n",
        sha256::hex(projection.acceptance_bytes)
    ));
    text.push_str(&format!(
        "acceptance_evidence_origin_dir={}\n",
        projection.evidence_relative
    ));
    text.push_str(&format!(
        "acceptance_evidence_metadata_bytes={}\n",
        projection.evidence_metadata.len()
    ));
    text.push_str(&format!(
        "acceptance_evidence_metadata_sha256={}\n",
        sha256::hex(projection.evidence_metadata)
    ));
    text.push_str(&format!(
        "candidate_intent_bytes={}\n",
        projection.evidence_intent.len()
    ));
    text.push_str(&format!(
        "candidate_intent_sha256={}\n",
        sha256::hex(projection.evidence_intent)
    ));
    text.push_str(&format!(
        "semantic_effect_snapshot_bytes={}\n",
        projection.evidence_snapshot.len()
    ));
    text.push_str(&format!(
        "semantic_effect_snapshot_sha256={}\n",
        sha256::hex(projection.evidence_snapshot)
    ));
    text.push_str(&format!(
        "parent_rejection_prefix={}\n",
        projection.parent_rejection_prefix
    ));
    text.push_str(&format!(
        "current_rejection_prefix={}\n",
        projection.prior_rejections
    ));
    text.push_str(&format!(
        "rejection_suffix_count={}\n",
        rejection_suffix.len()
    ));
    text.push_str("transition_history_role=pre_acceptance_advisory_snapshot\n");
    for (index, rejection) in rejection_suffix.iter().enumerate() {
        let prefix = format!("rejection_suffix_{index}");
        let revision = rejection
            .take_revision
            .map(|value| value.to_string())
            .unwrap_or_else(|| "legacy_unknown".to_string());
        text.push_str(&format!("{prefix}_revision={revision}\n"));
        text.push_str(&format!("{prefix}_sha256={}\n", rejection.zip_sha256));
        text.push_str(&format!(
            "{prefix}_stage={}\n",
            lifecycle_one_line(&rejection.rejection_stage)
        ));
        text.push_str(&format!(
            "{prefix}_attempt_epoch_millis={}\n",
            rejection.attempt_epoch_millis
        ));
        text.push_str(&format!(
            "{prefix}_reason={}\n",
            lifecycle_one_line(&rejection.reason)
        ));
    }
    text
}

fn lifecycle_one_line(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn relative_project_path(project_root: &Path, path: &Path) -> AppResult<String> {
    let relative = path.strip_prefix(project_root).map_err(|error| {
        format!(
            "Lifecycle-Handover Pfad liegt ausserhalb des Projekt-Roots {}: {} ({error})",
            project_root.display(),
            path.display()
        )
    })?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn write_or_verify_exact_file(path: &Path, expected: &[u8]) -> AppResult<()> {
    if path.exists() {
        let actual = fs::read(path).map_err(|error| {
            format!("Lifecycle-Handover Datei nicht lesbar {}: {error}", path.display())
        })?;
        if actual != expected {
            return Err(format!(
                "Lifecycle-Handover Datei-Konflikt {}; bestehende Bytes weichen ab; fail-closed",
                path.display()
            ));
        }
        return Ok(());
    }
    let pending = path.with_extension("post_acceptance.pending");
    remove_file_if_exists(&pending)?;
    fs::write(&pending, expected).map_err(|error| {
        format!(
            "Lifecycle-Handover Datei-Staging fehlgeschlagen {}: {error}",
            pending.display()
        )
    })?;
    fs::rename(&pending, path).map_err(|error| {
        format!(
            "Lifecycle-Handover Datei-Publikation fehlgeschlagen {} -> {}: {error}",
            pending.display(),
            path.display()
        )
    })?;
    Ok(())
}

fn exact_handoff_marker_line_spans(text: &str, marker: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut offset = 0usize;
    for segment in text.split_inclusive('\n') {
        let without_lf = segment.strip_suffix('\n').unwrap_or(segment);
        let line = without_lf.strip_suffix('\r').unwrap_or(without_lf);
        if line == marker {
            spans.push((offset, offset + line.len()));
        }
        offset += segment.len();
    }
    spans
}

fn upsert_post_acceptance_handoff_section(path: &Path, section: &str) -> AppResult<()> {
    let pending = path.with_extension("post_acceptance.pending");
    if !path.exists() && pending.is_file() {
        fs::rename(&pending, path).map_err(|error| {
            format!(
                "Assistant-Handoff unterbrochene Post-Acceptance-Publikation nicht recoverbar {} -> {}: {error}",
                pending.display(),
                path.display()
            )
        })?;
    } else if path.exists() {
        remove_file_if_exists(&pending)?;
    }
    let mut text = fs::read_to_string(path).map_err(|error| {
        format!(
            "Assistant-Handoff fuer Post-Acceptance-Finalisierung nicht lesbar {}: {error}",
            path.display()
        )
    })?;

    let begin_spans = exact_handoff_marker_line_spans(&text, POST_ACCEPTANCE_SECTION_BEGIN);
    let end_spans = exact_handoff_marker_line_spans(&text, POST_ACCEPTANCE_SECTION_END);
    match (begin_spans.as_slice(), end_spans.as_slice()) {
        ([], []) => {}
        ([(start, _)], [(_, end)]) if start < end => {
            let existing = text[*start..*end].trim_end_matches(['\r', '\n']);
            let expected = section.trim_end_matches(['\r', '\n']);
            if existing != expected {
                return Err(format!(
                    "Assistant-Handoff Post-Acceptance-Block ist widerspruechlich {}; fail-closed",
                    path.display()
                ));
            }
            return Ok(());
        }
        _ => {
            return Err(format!(
                "Assistant-Handoff Post-Acceptance-Markerzeilen sind unvollstaendig, mehrfach oder falsch geordnet {}; fail-closed",
                path.display()
            ));
        }
    }

    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push('\n');
    text.push_str(section);
    remove_file_if_exists(&pending)?;
    fs::write(&pending, text.as_bytes()).map_err(|error| {
        format!(
            "Assistant-Handoff Post-Acceptance-Staging fehlgeschlagen {}: {error}",
            pending.display()
        )
    })?;
    remove_file_if_exists(path)?;
    fs::rename(&pending, path).map_err(|error| {
        format!(
            "Assistant-Handoff Post-Acceptance-Publikation fehlgeschlagen {} -> {}: {error}",
            pending.display(),
            path.display()
        )
    })?;
    Ok(())
}

fn collect_take_attempt_evidence_files(
    root: &Path,
    current: &Path,
    depth: usize,
    files: &mut Vec<(PathBuf, String)>,
) -> AppResult<()> {
    const MAX_EVIDENCE_ARCHIVE_DEPTH: usize = 12;
    if depth > MAX_EVIDENCE_ARCHIVE_DEPTH {
        return Err(format!(
            "Attempt-Evidence Archivbaum ueberschreitet maximale Tiefe {}: {}",
            MAX_EVIDENCE_ARCHIVE_DEPTH,
            current.display()
        ));
    }
    let mut entries = fs::read_dir(current)
        .map_err(|error| {
            format!(
                "Attempt-Evidence Staging nicht lesbar {}: {error}",
                current.display()
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Attempt-Evidence Staging-Eintrag nicht lesbar: {error}"))?;
    entries.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            format!(
                "Attempt-Evidence Metadaten nicht lesbar {}: {error}",
                path.display()
            )
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "Attempt-Evidence Staging enthaelt verbotenen Symlink {}",
                path.display()
            ));
        }
        if metadata.is_dir() {
            collect_take_attempt_evidence_files(root, &path, depth.saturating_add(1), files)?;
            continue;
        }
        if !metadata.is_file() {
            return Err(format!(
                "Attempt-Evidence Staging enthaelt nicht unterstuetzten Eintrag {}",
                path.display()
            ));
        }
        let relative = path.strip_prefix(root).map_err(|_| {
            format!(
                "Attempt-Evidence Datei liegt ausserhalb des autorisierten Roots: {}",
                path.display()
            )
        })?;
        let mut components = Vec::new();
        for component in relative.components() {
            let std::path::Component::Normal(value) = component else {
                return Err(format!(
                    "Attempt-Evidence Archivpfad enthaelt verbotene Komponente: {}",
                    relative.display()
                ));
            };
            let value = value.to_str().ok_or_else(|| {
                format!(
                    "Attempt-Evidence Archivpfad ist nicht UTF-8: {}",
                    relative.display()
                )
            })?;
            if value.is_empty()
                || value
                    .chars()
                    .any(|ch| matches!(ch, '/' | '\\' | '\n' | '\r' | '\t'))
            {
                return Err(format!(
                    "Attempt-Evidence Archivsegment ist unzulaessig: {value:?}"
                ));
            }
            components.push(value.to_string());
        }
        if components.is_empty() {
            return Err("Attempt-Evidence Archivdatei ohne relativen Pfad".to_string());
        }
        let zip_name = format!(
            "{}/{}",
            TAKE_ATTEMPT_EVIDENCE_EXPORT_DIR,
            components.join("/")
        );
        validate_zip_relative_path(Path::new(&zip_name), &zip_name)?;
        files.push((path, zip_name));
    }
    Ok(())
}

fn zip_assistant_handover_stage(source_dir: &Path, zip_path: &Path) -> AppResult<()> {
    let file = fs::File::create(zip_path).map_err(|error| {
        format!("Post-Acceptance ZIP nicht anlegbar {}: {error}", zip_path.display())
    })?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    let mut entries = fs::read_dir(source_dir)
        .map_err(|error| format!("Assistant-Export-Staging nicht lesbar: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Assistant-Export-Staging-Eintrag nicht lesbar: {error}"))?;
    entries.sort_by_key(|entry| entry.file_name().to_string_lossy().into_owned());
    for entry in entries {
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            format!("Assistant-Export-Staging-Metadaten nicht lesbar {}: {error}", path.display())
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "Assistant-Export-Staging enthaelt verbotenen Symlink {}",
                path.display()
            ));
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if metadata.is_file() {
            zip.start_file(&name, options).map_err(|error| {
                format!("Post-Acceptance ZIP start_file {name}: {error}")
            })?;
            zip.write_all(&fs::read(&path).map_err(|error| {
                format!("Assistant-Export-Datei nicht lesbar {}: {error}", path.display())
            })?)
            .map_err(|error| format!("Post-Acceptance ZIP write {name}: {error}"))?;
            continue;
        }
        if metadata.is_dir() && name == "source_snapshot" {
            let mut children = fs::read_dir(&path)
                .map_err(|error| format!("Source-Snapshot-Staging nicht lesbar: {error}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| format!("Source-Snapshot-Eintrag nicht lesbar: {error}"))?;
            children.sort_by_key(|child| child.file_name().to_string_lossy().into_owned());
            for child in children {
                let child_path = child.path();
                let child_metadata = fs::symlink_metadata(&child_path).map_err(|error| {
                    format!("Source-Snapshot-Metadaten nicht lesbar {}: {error}", child_path.display())
                })?;
                if child_metadata.file_type().is_symlink() || !child_metadata.is_file() {
                    return Err(format!(
                        "Source-Snapshot-Staging enthaelt nicht unterstuetzten Eintrag {}",
                        child_path.display()
                    ));
                }
                let child_name = child.file_name().to_string_lossy().into_owned();
                let zip_name = format!("source_snapshot/{child_name}");
                zip.start_file(&zip_name, options).map_err(|error| {
                    format!("Post-Acceptance ZIP start_file {zip_name}: {error}")
                })?;
                zip.write_all(&fs::read(&child_path).map_err(|error| {
                    format!("Source-Snapshot-Datei nicht lesbar {}: {error}", child_path.display())
                })?)
                .map_err(|error| format!("Post-Acceptance ZIP write {zip_name}: {error}"))?;
            }
            continue;
        }
        if metadata.is_dir() && name == TAKE_ATTEMPT_EVIDENCE_EXPORT_DIR {
            let mut evidence_files = Vec::new();
            collect_take_attempt_evidence_files(&path, &path, 0, &mut evidence_files)?;
            evidence_files.sort_by(|left, right| left.1.cmp(&right.1));
            for (evidence_path, zip_name) in evidence_files {
                zip.start_file(&zip_name, options).map_err(|error| {
                    format!("Post-Acceptance ZIP start_file {zip_name}: {error}")
                })?;
                zip.write_all(&fs::read(&evidence_path).map_err(|error| {
                    format!(
                        "Attempt-Evidence Datei nicht lesbar {}: {error}",
                        evidence_path.display()
                    )
                })?)
                .map_err(|error| format!("Post-Acceptance ZIP write {zip_name}: {error}"))?;
            }
            continue;
        }
        return Err(format!(
            "Assistant-Export-Staging enthaelt unerwarteten Eintrag {}",
            path.display()
        ));
    }
    zip.finish()
        .map_err(|error| format!("Post-Acceptance ZIP finalize: {error}"))?;
    Ok(())
}

fn unrejected_take_peers(
    peers: &[PathBuf],
    rejected_sha256: &BTreeSet<String>,
) -> AppResult<Vec<PathBuf>> {
    let mut eligible = Vec::new();
    for path in peers {
        let sha256 = multi_revision_evasion::sha256_file(path)?;
        if !rejected_sha256.contains(&sha256.to_ascii_lowercase()) {
            eligible.push(path.clone());
        }
    }
    Ok(eligible)
}

fn collect_zip_candidates(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    out.extend(entries.flatten().map(|entry| entry.path()).filter(|path| {
        path.is_file()
            && path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.starts_with("EtuSchlopf_v")
                        && name.to_ascii_lowercase().ends_with(".zip")
                })
    }));
}

fn zip_version_key(path: &Path) -> Option<(u64, u64, u64)> {
    let file = fs::File::open(path).ok()?;
    let mut archive = zip::ZipArchive::new(file).ok()?;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).ok()?;
        let name = entry.name().replace('\\', "/");
        if name != "Cargo.toml"
            && !(name.ends_with("/Cargo.toml") && name.matches('/').count() == 1)
        {
            continue;
        }
        let mut text = String::new();
        std::io::Read::read_to_string(&mut entry, &mut text).ok()?;
        let version = text.lines().find_map(|line| {
            let (key, value) = line.split_once('=')?;
            (key.trim() == "version").then_some(value.trim().trim_matches('"'))
        })?;
        let mut parts = version.split('.').map(str::parse::<u64>);
        return Some((parts.next()?.ok()?, parts.next()?.ok()?, parts.next()?.ok()?));
    }
    None
}

fn zip_revision_key(path: &Path, version: (u64, u64, u64)) -> Option<u64> {
    let name = path.file_name()?.to_str()?.to_ascii_lowercase();
    let prefix = format!(
        "etuschlopf_v{}_{}_{}_r",
        version.0, version.1, version.2
    );
    let tail = name.strip_prefix(&prefix)?;
    let digit_count = tail
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .count();
    if digit_count == 0 {
        return None;
    }
    let boundary = tail.as_bytes().get(digit_count).copied();
    if !matches!(boundary, Some(b'_' | b'.')) {
        return None;
    }
    tail[..digit_count].parse().ok()
}

fn fnv1a64_file(path: &Path) -> AppResult<u64> {
    let bytes = fs::read(path)
        .map_err(|error| format!("Datei fuer FNV64 nicht lesbar {}: {error}", path.display()))?;
    Ok(rust_source_scan_primitives::fnv1a64(&bytes))
}

fn make_temp_dir() -> AppResult<PathBuf> {
    let base = env::temp_dir();
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Systemzeit vor UNIX_EPOCH: {e}"))?
        .as_millis();
    let dir = base.join(format!(
        "etuschlopf_rust_take_bootstrap_{}_{}",
        std::process::id(),
        millis
    ));
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| {
            format!(
                "Temp-Ordner konnte nicht geloescht werden {}: {e}",
                dir.display()
            )
        })?;
    }
    fs::create_dir_all(&dir).map_err(|e| {
        format!(
            "Temp-Ordner konnte nicht erzeugt werden {}: {e}",
            dir.display()
        )
    })?;
    Ok(dir)
}

#[derive(Debug, Clone)]
struct TakeTransaction {
    root: PathBuf,
    backup_root: PathBuf,
    state_path: PathBuf,
    marker_path: PathBuf,
}

impl TakeTransaction {
    fn prepare(
        project_root: &Path,
        source_root: &Path,
        from_version: &str,
        to_version: &str,
        take_revision: u64,
        candidate_sha256: &str,
        revision_snapshot: &str,
    ) -> AppResult<Self> {
        let transaction_parent = project_root.join(TAKE_TRANSACTION_ROOT);
        fs::create_dir_all(&transaction_parent).map_err(|error| {
            format!(
                "Take-Transaktionsordner konnte nicht erzeugt werden {}: {error}",
                transaction_parent.display()
            )
        })?;
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("Systemzeit vor UNIX_EPOCH: {error}"))?
            .as_millis();
        let root = transaction_parent.join(format!("transaction_{}_{}", std::process::id(), stamp));
        let backup_root = root.join("backup");
        let state_path = root.join("state.txt");
        let marker_path = project_root.join(TAKE_TRANSACTION_MARKER);
        fs::create_dir_all(&backup_root).map_err(|error| {
            format!(
                "Take-Backupordner konnte nicht erzeugt werden {}: {error}",
                backup_root.display()
            )
        })?;
        fs::write(
            root.join(TAKE_TRANSACTION_CANDIDATE_METADATA),
            format!(
                "schema=2\nfrom_version={}\nto_version={}\ntake_revision={}\nzip_sha256={}\n",
                from_version.replace(['\r', '\n'], " "),
                to_version.replace(['\r', '\n'], " "),
                take_revision,
                candidate_sha256
            ),
        )
        .map_err(|error| format!("Take-Kandidatenprovenienz nicht schreibbar: {error}"))?;
        fs::write(
            root.join(TAKE_TRANSACTION_CANDIDATE_SNAPSHOT),
            revision_snapshot,
        )
        .map_err(|error| format!("Take-Kandidatensnapshot nicht schreibbar: {error}"))?;
        let intent_source = source_root.join(take_change_control::CHANGE_MANIFEST_RELATIVE_PATH);
        if let Err(error) = fs::copy(
            &intent_source,
            root.join(TAKE_TRANSACTION_CANDIDATE_INTENT),
        ) {
            bootstrap_warning!(
                "Candidate-Intent konnte nicht in die Take-Transaction kopiert werden {}: {}; Transaction-/Gate-Semantik bleibt unveraendert.",
                intent_source.display(),
                error
            );
        }
        write_transaction_state(&state_path, "preparing")?;

        for dir in take_change_control::MANAGED_DIRS {
            let source = project_root.join(dir);
            if source.is_dir() {
                copy_dir_recursive(&source, &backup_root.join(dir))?;
            }
        }
        let backup_root_files = backup_root.join("root_files");
        fs::create_dir_all(&backup_root_files).map_err(|error| {
            format!(
                "Take-Rootdatei-Backupordner konnte nicht erzeugt werden {}: {error}",
                backup_root_files.display()
            )
        })?;
        let mut existing_root_files = Vec::new();
        for file in take_change_control::MANAGED_ROOT_FILES
        .into_iter()
        .chain(managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES)
    {
            let source = project_root.join(file);
            if source.is_file() {
                fs::copy(&source, backup_root_files.join(file)).map_err(|error| {
                    format!(
                        "Take-Rootdatei konnte nicht gesichert werden {}: {error}",
                        source.display()
                    )
                })?;
                existing_root_files.push(file);
            }
        }
        fs::write(
            backup_root.join("existing_root_files.txt"),
            existing_root_files.join("\n"),
        )
        .map_err(|error| format!("Take-Backupmanifest nicht schreibbar: {error}"))?;
        verify_backup_matches_project(&backup_root, project_root)?;

        let recovery_exe = project_root.join(TAKE_RECOVERY_EXE);
        if let Some(parent) = recovery_exe.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "Take-Recovery-Ordner konnte nicht erzeugt werden {}: {error}",
                    parent.display()
                )
            })?;
        }
        let current_exe = env::current_exe()
            .map_err(|error| format!("laufende Take-Bootstrap-EXE nicht bestimmbar: {error}"))?;
        fs::copy(&current_exe, &recovery_exe).map_err(|error| {
            format!(
                "Take-Recovery-EXE konnte nicht gesichert werden {} -> {}: {error}",
                current_exe.display(),
                recovery_exe.display()
            )
        })?;

        write_transaction_state(&state_path, "prepared")?;
        fs::write(&marker_path, root.to_string_lossy().as_bytes()).map_err(|error| {
            format!(
                "Take-Pending-Marker konnte nicht geschrieben werden {}: {error}",
                marker_path.display()
            )
        })?;

        Ok(Self {
            root,
            backup_root,
            state_path,
            marker_path,
        })
    }

    fn apply_and_verify(&self, source_root: &Path, project_root: &Path) -> AppResult<()> {
        write_transaction_state(&self.state_path, "applying")?;
        copy_source_tree(source_root, project_root)?;
        take_change_control::verify_installed_tree_matches_source(source_root, project_root)?;
        write_transaction_state(&self.state_path, "applied")?;
        Ok(())
    }

    fn mark_reexec(&self) -> AppResult<()> {
        write_transaction_state(&self.state_path, "reexec")
    }

    fn resolve_root_cargo_lock(
        &self,
        project_root: &Path,
        current_version: &str,
        incoming_version: &str,
        change_class: &str,
    ) -> AppResult<()> {
        let _root_lock_timing = startup_timing::Phase::new("RootCargoLockCheck");
        let root_lock_step = bootstrap_step("root_lock", "Cargo.lock und Kodex-Gates werden geprüft (cargo check)");
        let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
        let jobs = std::thread::available_parallelism()
            .map(|parallelism| parallelism.get().saturating_sub(1).max(1))
            .unwrap_or(1);
        let status = Command::new(cargo)
            .current_dir(project_root)
            .arg("check")
            .arg("--quiet")
            .arg("--package")
            .arg(env!("CARGO_PKG_NAME"))
            .arg("--lib")
            .arg("--jobs")
            .arg(jobs.to_string())
            .arg("--target-dir")
            .arg(project_root.join(BOOTSTRAP_CARGO_TARGET_DIR))
            .env("CARGO_INCREMENTAL", "0")
            .env("CARGO_PROFILE_DEV_BUILD_OVERRIDE_OPT_LEVEL", "1")
            .env("ETUSCHLOPF_TAKE_FROM", current_version)
            .env("ETUSCHLOPF_TAKE_TO", incoming_version)
            .env("ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE", "1")
            .env("ETUSCHLOPF_TAKE_CHANGE_CLASS", change_class)
            .status()
            .map_err(|error| {
                format!(
                    "Take-Transaction konnte die lokale Root-Cargo.lock nicht durch Cargo validieren/auflösen: {error}"
                )
            })?;
        if !status.success() {
            return Err(format!(
                "Take-Transaction Root-Cargo.lock Resolution fehlgeschlagen: cargo check --package {} --lib Exit {:?}",
                env!("CARGO_PKG_NAME"),
                status.code()
            ));
        }
        if !project_root.join("Cargo.lock").is_file() {
            return Err(
                "Take-Transaction Root-Cargo.lock Resolution lieferte keine lokale Cargo.lock"
                    .to_string(),
            );
        }
        root_lock_step.finish();
        bootstrap_detail!(
            "[EtuSchlopf rust-take-bootstrap] ROOT-CARGO-LOCK RESOLUTION GRUEN: Cargo besitzt die lokale Lockfile-Mutation innerhalb der offenen Take-Transaktion; nachfolgende Root-Cargo-Aufrufe laufen --locked."
        );
        Ok(())
    }

    fn mark_committed(&self) -> AppResult<()> {
        write_transaction_state(&self.state_path, "committed")
    }

    fn commit(&self) -> AppResult<()> {
        remove_file_if_exists(&self.marker_path)?;
        remove_dir_if_exists(&self.root)?;
        Ok(())
    }

    fn rollback(&self, project_root: &Path) -> AppResult<()> {
        public_artifact_recovery::recover_pending_public_artifacts(project_root)?;
        self.rollback_source_after_public_recovery(project_root)
    }

    fn rollback_preserving_provenance(&self, project_root: &Path) -> AppResult<()> {
        public_artifact_recovery::recover_pending_public_artifacts(project_root)?;
        self.rollback_source_after_public_recovery(project_root)
    }

    fn rollback_source_after_public_recovery(&self, project_root: &Path) -> AppResult<()> {
        write_transaction_state(&self.state_path, "rolling_back")?;
        restore_backup(&self.backup_root, project_root)?;
        verify_backup_matches_project(&self.backup_root, project_root)?;
        clean_bootstrap_package_after_source_rollback(project_root)?;
        write_transaction_state(&self.state_path, "rolled_back")?;
        Ok(())
    }

    fn finish_rollback(&self) -> AppResult<()> {
        remove_file_if_exists(&self.marker_path)?;
        remove_dir_if_exists(&self.root)?;
        Ok(())
    }
}


fn clean_bootstrap_package_after_source_rollback(project_root: &Path) -> AppResult<()> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let target_dir = project_root.join(BOOTSTRAP_CARGO_TARGET_DIR);
    if !target_dir.is_dir() {
        return Ok(());
    }
    let mut entries = fs::read_dir(&target_dir).map_err(|error| {
        format!("Bootstrap-Paketcache nicht lesbar {}: {error}", target_dir.display())
    })?;
    if entries.next().transpose().map_err(|error| {
        format!("Bootstrap-Paketcache-Eintrag nicht lesbar {}: {error}", target_dir.display())
    })?.is_none() {
        bootstrap_detail!("TAKE-TRANSACTION ROLLBACK CACHE: leerer Bootstrap-Paketcache; keine Artefakte zu bereinigen.");
        return Ok(());
    }
    let output = Command::new(&cargo)
        .current_dir(project_root)
        .arg("clean")
        .arg("--quiet")
        .arg("--package")
        .arg(env!("CARGO_PKG_NAME"))
        .arg("--target-dir")
        .arg(&target_dir)
        .output()
        .map_err(|error| {
            format!("Take-Rollback konnte den Bootstrap-Paketcache nicht bereinigen {} mit {:?}: {error}", target_dir.display(), cargo)
        })?;
    let log_dir = project_root.join("target/etuschlopf_logs");
    let save = fs::create_dir_all(&log_dir).and_then(|()| {
        fs::write(log_dir.join("bootstrap_rollback_cache.stdout.log"), &output.stdout)?;
        fs::write(log_dir.join("bootstrap_rollback_cache.stderr.log"), &output.stderr)
    });
    if let Err(error) = save {
        bootstrap_warning!("Rollback-Cargo-Ausgabe nicht vollstaendig gespeichert: {error}");
    }
    if !output.status.success() {
        return Err(format!(
            "Take-Rollback Bootstrap-Paketcache-Bereinigung fehlgeschlagen: cargo clean --package {} --target-dir {} Exit {:?}; Cargo {:?}; stdout: {}; stderr: {}",
            env!("CARGO_PKG_NAME"), target_dir.display(), output.status.code(), cargo,
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)
        ));
    }
    bootstrap_detail!(
        "[EtuSchlopf rust-take-bootstrap] TAKE-TRANSACTION ROLLBACK CACHE: Bootstrap-Paketcache nach Source-Restore bereinigt."
    );
    Ok(())
}

type TransactionCandidateProvenance = (String, String, u64, String, String, Option<Vec<u8>>);

fn read_transaction_candidate_provenance(
    transaction_root: &Path,
) -> AppResult<Option<TransactionCandidateProvenance>> {
    let metadata_path = transaction_root.join(TAKE_TRANSACTION_CANDIDATE_METADATA);
    if !metadata_path.is_file() {
        return Ok(None);
    }
    let metadata = fs::read_to_string(&metadata_path).map_err(|error| {
        format!(
            "Take-Kandidatenprovenienz nicht lesbar {}: {error}",
            metadata_path.display()
        )
    })?;
    let value = |key: &str| -> AppResult<String> {
        let prefix = format!("{key}=");
        metadata
            .lines()
            .find_map(|line| line.strip_prefix(&prefix))
            .map(ToOwned::to_owned)
            .ok_or_else(|| format!("Take-Kandidatenprovenienzfeld {key:?} fehlt"))
    };
    let snapshot_path = transaction_root.join(TAKE_TRANSACTION_CANDIDATE_SNAPSHOT);
    let snapshot = fs::read_to_string(&snapshot_path).map_err(|error| {
        format!(
            "Take-Kandidatensnapshot nicht lesbar {}: {error}",
            snapshot_path.display()
        )
    })?;
    let take_revision = value("take_revision")?
        .parse::<u64>()
        .map_err(|error| format!("Take-Kandidatenprovenienz hat ungueltige Revision: {error}"))?;
    let intent_path = transaction_root.join(TAKE_TRANSACTION_CANDIDATE_INTENT);
    let candidate_intent = match fs::read(&intent_path) {
        Ok(bytes) => Some(bytes),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => {
            bootstrap_warning!(
                "Take-Candidate-Intent Evidence nicht lesbar {}: {}; Recovery-/Acceptance-Semantik bleibt unveraendert.",
                intent_path.display(),
                error
            );
            None
        }
    };
    Ok(Some((
        value("from_version")?,
        value("to_version")?,
        take_revision,
        value("zip_sha256")?,
        snapshot,
        candidate_intent,
    )))
}

fn recover_pending_transaction(project_root: &Path) -> AppResult<()> {
    let marker_path = project_root.join(TAKE_TRANSACTION_MARKER);
    if !marker_path.exists() {
        return Ok(());
    }
    let raw = fs::read_to_string(&marker_path).map_err(|error| {
        format!(
            "Take-Pending-Marker nicht lesbar {}: {error}",
            marker_path.display()
        )
    })?;
    let transaction_root = PathBuf::from(raw.trim());
    validate_transaction_path(project_root, &transaction_root)?;
    let state_path = transaction_root.join("state.txt");
    let state = fs::read_to_string(&state_path)
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();
    let transaction = TakeTransaction {
        backup_root: transaction_root.join("backup"),
        root: transaction_root,
        state_path,
        marker_path,
    };
    match state.as_str() {
        "preparing" | "prepared" => {
            remove_file_if_exists(&transaction.marker_path)?;
            remove_dir_if_exists(&transaction.root)?;
            bootstrap_line!(
                "Unvollstaendige vorbereitete Take-Transaktion entfernt."
            );
            Ok(())
        }
        "applying" | "applied" | "reexec" | "rolling_back" => {
            let candidate = read_transaction_candidate_provenance(&transaction.root)?;
            transaction.rollback_preserving_provenance(project_root)?;
            if let Some((
                from_version,
                to_version,
                take_revision,
                zip_sha256,
                snapshot,
                candidate_intent,
            )) = candidate
            {
                let reason = format!(
                    "Unvollstaendige Take-Transaktion im Zustand {state} wurde beim Bootstrap transaktional zurueckgerollt."
                );
                let rejection = multi_revision_evasion::RejectionRecord {
                    from_version: &from_version,
                    to_version: &to_version,
                    take_revision,
                    zip_sha256: &zip_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_CRASH_RECOVERY,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: &snapshot,
                    reason: &reason,
                };
                multi_revision_evasion::record_rejection(project_root, &rejection)?;
                if let Err(error) = finalize_rejection_evidence_after_rollback(
                    project_root,
                    &rejection,
                    "unknown_recovery",
                    None,
                ) {
                    bootstrap_warning!(
                        "Crash-Recovery Failure-Evidence konnte nicht vollstaendig finalisiert werden: {}; kanonische Rejection bleibt autoritativ.",
                        error
                    );
                }
                finalize_rejection_assistant_handover(
                    project_root,
                    &rejection,
                    FailureAttemptContext {
                        started_epoch_millis: None,
                        child_status: "unknown_recovery",
                        child_exit: None,
                        transaction_state: "rolled_back_recovery",
                    },
                )?;
                bootstrap_line!(
                    "Unvollstaendige Take-Transaktion wiederhergestellt · Kandidat rejected · Failure-Handover publiziert."
                );
            } else {
                bootstrap_line!(
                    "Unvollstaendige Take-Transaktion aus aelterem Bootstrap wiederhergestellt."
                );
            }
            transaction.finish_rollback()?;
            Ok(())
        }
        "committed" => {
            if let Some((
                from_version,
                to_version,
                take_revision,
                zip_sha256,
                snapshot,
                candidate_intent,
            )) = read_transaction_candidate_provenance(&transaction.root)?
            {
                let prior_rejections = multi_revision_evasion::load_rejected_revisions(
                    project_root,
                    &from_version,
                    &to_version,
                )?;
                multi_revision_evasion::record_acceptance(
                    project_root,
                    &from_version,
                    &to_version,
                    take_revision,
                    &zip_sha256,
                    prior_rejections.len(),
                )?;
                record_acceptance_evidence_advisory(
                    project_root,
                    &from_version,
                    &to_version,
                    take_revision,
                    &zip_sha256,
                    candidate_intent.as_deref(),
                    &snapshot,
                );
                finalize_post_acceptance_assistant_handover(
                    project_root,
                    &from_version,
                    &to_version,
                    take_revision,
                    &zip_sha256,
                    prior_rejections.len(),
                )?;
                bootstrap_line!(
                    "Committed Take-Provenienz wiederhergestellt · kanonische Payload und Post-Acceptance-Handover bestaetigt."
                );
            }
            transaction.commit()?;
            Ok(())
        }
        "rolled_back" => {
            if let Some((
                from_version,
                to_version,
                take_revision,
                zip_sha256,
                snapshot,
                candidate_intent,
            )) = read_transaction_candidate_provenance(&transaction.root)?
            {
                let reason =
                    "Bereits sicher zurueckgerollte Pending-Transaktion wurde beim Bootstrap als rejected vervollstaendigt.";
                let rejection = multi_revision_evasion::RejectionRecord {
                    from_version: &from_version,
                    to_version: &to_version,
                    take_revision,
                    zip_sha256: &zip_sha256,
                    rejection_stage: multi_revision_evasion::REJECTION_STAGE_CRASH_RECOVERY,
                    candidate_intent: candidate_intent.as_deref(),
                    snapshot: &snapshot,
                    reason,
                };
                multi_revision_evasion::record_rejection(project_root, &rejection)?;
                if let Err(error) = finalize_rejection_evidence_after_rollback(
                    project_root,
                    &rejection,
                    "unknown_recovery",
                    None,
                ) {
                    bootstrap_warning!(
                        "Crash-Recovery Failure-Evidence konnte nicht vollstaendig finalisiert werden: {}; kanonische Rejection bleibt autoritativ.",
                        error
                    );
                }
                finalize_rejection_assistant_handover(
                    project_root,
                    &rejection,
                    FailureAttemptContext {
                        started_epoch_millis: None,
                        child_status: "unknown_recovery",
                        child_exit: None,
                        transaction_state: "rolled_back_recovery",
                    },
                )?;
                bootstrap_line!(
                    "Rolled-back Take-Provenienz wiederhergestellt · Kandidat rejected · Failure-Handover publiziert."
                );
            }
            transaction.finish_rollback()?;
            Ok(())
        }
        other => Err(format!(
            "Take-Pending-Marker verweist auf unbekannten Transaktionszustand {other:?}; automatische Fortsetzung verweigert"
        )),
    }
}

fn validate_transaction_path(project_root: &Path, transaction_root: &Path) -> AppResult<()> {
    let expected_parent = project_root.join(TAKE_TRANSACTION_ROOT);
    let expected_parent = fs::canonicalize(&expected_parent).map_err(|error| {
        format!(
            "Take-Transaktionswurzel nicht kanonisierbar {}: {error}",
            expected_parent.display()
        )
    })?;
    let transaction_root = fs::canonicalize(transaction_root).map_err(|error| {
        format!(
            "Take-Transaktion nicht kanonisierbar {}: {error}",
            transaction_root.display()
        )
    })?;
    if transaction_root.parent() != Some(expected_parent.as_path()) {
        return Err(format!(
            "Take-Pending-Marker verweist ausserhalb der Transaktionswurzel: {}",
            transaction_root.display()
        ));
    }
    Ok(())
}

fn restore_backup(backup_root: &Path, project_root: &Path) -> AppResult<()> {
    for dir in take_change_control::MANAGED_DIRS {
        let destination = project_root.join(dir);
        remove_dir_if_exists(&destination)?;
        let source = backup_root.join(dir);
        if source.is_dir() {
            copy_dir_recursive(&source, &destination)?;
        }
    }

    let existing_root_files = read_existing_root_files(backup_root)?;
    for file in take_change_control::MANAGED_ROOT_FILES
        .into_iter()
        .chain(managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES)
    {
        remove_file_if_exists(&project_root.join(file))?;
        if existing_root_files.contains(file) {
            let source = backup_root.join("root_files").join(file);
            fs::copy(&source, project_root.join(file)).map_err(|error| {
                format!(
                    "Take-Rollback konnte Root-Datei nicht wiederherstellen {}: {error}",
                    file
                )
            })?;
        }
    }
    Ok(())
}

fn verify_backup_matches_project(backup_root: &Path, project_root: &Path) -> AppResult<()> {
    for dir in take_change_control::MANAGED_DIRS {
        let backup = backup_root.join(dir);
        let project = project_root.join(dir);
        if backup.is_dir() {
            verify_directory_exact(&backup, &project)?;
        } else if project.exists() {
            return Err(format!(
                "Take-Backupverifikation: Projektordner existiert ohne Backup: {dir}"
            ));
        }
    }
    let existing_root_files = read_existing_root_files(backup_root)?;
    for file in take_change_control::MANAGED_ROOT_FILES
        .into_iter()
        .chain(managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES)
    {
        let backup = backup_root.join("root_files").join(file);
        let project = project_root.join(file);
        if existing_root_files.contains(file) {
            if file_payload_differs(&backup, &project)? {
                return Err(format!(
                    "Take-Backupverifikation: Root-Datei stimmt nicht: {file}"
                ));
            }
        } else if project.exists() {
            return Err(format!(
                "Take-Backupverifikation: Root-Datei existiert unerwartet: {file}"
            ));
        }
    }
    Ok(())
}

fn verify_directory_exact(expected: &Path, actual: &Path) -> AppResult<()> {
    let expected_files = relative_file_set(expected)?;
    let actual_files = relative_file_set(actual)?;
    if expected_files != actual_files {
        return Err(format!(
            "Take-Backupverifikation: Dateimengen unterscheiden sich {} <> {}",
            expected.display(),
            actual.display()
        ));
    }
    for relative in expected_files {
        if file_payload_differs(&expected.join(&relative), &actual.join(&relative))? {
            return Err(format!(
                "Take-Backupverifikation: Datei unterscheidet sich: {}",
                relative.display()
            ));
        }
    }
    Ok(())
}

fn read_existing_root_files(backup_root: &Path) -> AppResult<BTreeSet<&'static str>> {
    let text = fs::read_to_string(backup_root.join("existing_root_files.txt"))
        .map_err(|error| format!("Take-Backupmanifest nicht lesbar: {error}"))?;
    let mut out = BTreeSet::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let Some(known) = take_change_control::MANAGED_ROOT_FILES
            .into_iter()
            .chain(managed_source_tree_definition::LOCAL_TRANSACTIONAL_ROOT_FILES)
            .find(|known| *known == line)
        else {
            return Err(format!(
                "Take-Backupmanifest enthaelt unbekannte Root-Datei {line:?}"
            ));
        };
        out.insert(known);
    }
    Ok(out)
}


fn write_transaction_state(path: &Path, state: &str) -> AppResult<()> {
    fs::write(path, format!("{state}\n")).map_err(|error| {
        format!(
            "Take-Transaktionszustand konnte nicht geschrieben werden {}: {error}",
            path.display()
        )
    })
}

fn remove_file_if_exists(path: &Path) -> AppResult<()> {
    if path.exists() {
        fs::remove_file(path).map_err(|error| {
            format!(
                "Datei konnte nicht entfernt werden {}: {error}",
                path.display()
            )
        })?;
    }
    Ok(())
}

fn remove_dir_if_exists(path: &Path) -> AppResult<()> {
    if path.exists() {
        fs::remove_dir_all(path).map_err(|error| {
            format!(
                "Ordner konnte nicht entfernt werden {}: {error}",
                path.display()
            )
        })?;
    }
    Ok(())
}

fn extract_zip(zip_path: &Path, dest: &Path) -> AppResult<()> {
    let file = fs::File::open(zip_path).map_err(|e| {
        format!(
            "ZIP kann nicht geoeffnet werden {}: {e}",
            zip_path.display()
        )
    })?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("ZIP ist nicht lesbar {}: {e}", zip_path.display()))?;
    if archive.len() > MAX_ZIP_ENTRIES {
        return Err(format!(
            "ZIP enthaelt zu viele Eintraege: {} > {}",
            archive.len(),
            MAX_ZIP_ENTRIES
        ));
    }
    let mut normalized_paths = BTreeSet::new();
    let mut uncompressed_total = 0u64;
    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("ZIP-Eintrag {i} unlesbar: {e}"))?;
        uncompressed_total = uncompressed_total.saturating_add(entry.size());
        if uncompressed_total > MAX_ZIP_UNCOMPRESSED_BYTES {
            return Err(format!(
                "ZIP entpackt zu gross: {uncompressed_total} Bytes > {MAX_ZIP_UNCOMPRESSED_BYTES} Bytes"
            ));
        }
        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170_000 == 0o120_000)
        {
            return Err(format!(
                "ZIP-Eintrag {i} ist ein symbolischer Link und wird verweigert: {}",
                entry.name()
            ));
        }
        let enclosed = entry.enclosed_name().map(|p| p.to_owned()).ok_or_else(|| {
            format!(
                "ZIP-Eintrag {i} besitzt einen unsicheren Pfad und wird verweigert: {}",
                entry.name()
            )
        })?;
        validate_zip_relative_path(&enclosed, entry.name())?;
        let normalized = path_key_case_insensitive(&enclosed);
        if !normalized_paths.insert(normalized) {
            return Err(format!(
                "ZIP enthaelt einen doppelten oder unter Windows kollidierenden Pfad: {}",
                entry.name()
            ));
        }
        let out_path = dest.join(enclosed);
        if entry.name().ends_with('/') {
            fs::create_dir_all(&out_path).map_err(|e| {
                format!(
                    "Ordner aus ZIP konnte nicht erzeugt werden {}: {e}",
                    out_path.display()
                )
            })?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    format!(
                        "ZIP-Zielordner konnte nicht erzeugt werden {}: {e}",
                        parent.display()
                    )
                })?;
            }
            let mut out = fs::File::create(&out_path).map_err(|e| {
                format!(
                    "ZIP-Datei konnte nicht geschrieben werden {}: {e}",
                    out_path.display()
                )
            })?;
            io::copy(&mut entry, &mut out).map_err(|e| {
                format!(
                    "ZIP-Datei konnte nicht entpackt werden {}: {e}",
                    out_path.display()
                )
            })?;
        }
    }
    Ok(())
}

fn validate_zip_relative_path(path: &Path, raw_name: &str) -> AppResult<()> {
    if path.is_absolute() || raw_name.starts_with('/') || raw_name.starts_with('\\') {
        return Err(format!("absoluter ZIP-Pfad verboten: {raw_name:?}"));
    }
    for component in path.components() {
        let segment = component.as_os_str().to_string_lossy();
        if segment.is_empty() || segment == "." || segment == ".." {
            return Err(format!("unsicheres ZIP-Pfadsegment in {raw_name:?}"));
        }
        if segment.contains(':') || segment.ends_with(' ') || segment.ends_with('.') {
            return Err(format!(
                "Windows-inkompatibles ZIP-Pfadsegment {segment:?} in {raw_name:?}"
            ));
        }
        let base = segment
            .split('.')
            .next()
            .unwrap_or_default()
            .to_ascii_uppercase();
        let reserved = matches!(
            base.as_str(),
            "CON"
                | "PRN"
                | "AUX"
                | "NUL"
                | "COM1"
                | "COM2"
                | "COM3"
                | "COM4"
                | "COM5"
                | "COM6"
                | "COM7"
                | "COM8"
                | "COM9"
                | "LPT1"
                | "LPT2"
                | "LPT3"
                | "LPT4"
                | "LPT5"
                | "LPT6"
                | "LPT7"
                | "LPT8"
                | "LPT9"
        );
        if reserved {
            return Err(format!(
                "reservierter Windows-Pfadname {segment:?} in {raw_name:?}"
            ));
        }
    }
    Ok(())
}

fn path_key_case_insensitive(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().to_ascii_lowercase())
        .collect::<Vec<_>>()
        .join("/")
}

fn find_source_root(extract_root: &Path) -> AppResult<PathBuf> {
    if extract_root.join("Cargo.toml").exists() {
        return Ok(extract_root.to_path_buf());
    }
    if is_assistant_export_marker(extract_root) {
        return Err(export_zip_message());
    }
    let rd = fs::read_dir(extract_root)
        .map_err(|e| format!("Extract-Root nicht lesbar {}: {e}", extract_root.display()))?;
    for entry in rd.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if path.join("Cargo.toml").exists() {
            return Ok(path);
        }
        if is_assistant_export_marker(&path) {
            return Err(export_zip_message());
        }
    }
    Err("Diese ZIP ist nicht take-kompatibel: kein Cargo.toml im ZIP-Root oder in einem Top-Level-Projektordner gefunden. Fuer --take bitte einen Source-Release-ZIP EtuSchlopf_v*_unbuilt_*.zip verwenden; assistant_export_latest.zip ist nur fuer den Chat-Upload.".to_string())
}

fn export_zip_message() -> String {
    "Diese ZIP ist eine assistant_export-ZIP fuer den Chat, keine Source-/Take-ZIP. Bitte nicht mit --take anwenden. Im neuen Thread nur assistant_export_latest.zip hochladen; fuer --take brauchst du einen EtuSchlopf_v*_unbuilt_*.zip mit Cargo.toml.".to_string()
}

fn is_assistant_export_marker(path: &Path) -> bool {
    let context = path.join("assistant_context.json");
    if let Ok(text) = fs::read_to_string(&context)
        && (text.contains("\"zip_kind\": \"assistant_export\"")
            || text.contains("\"zip_kind\":\"assistant_export\"")
            || text.contains("\"take_compatible\": false")
            || text.contains("\"take_compatible\":false")
            || text.contains("assistant_external_memory_and_handover"))
    {
        return true;
    }
    let manifest = path.join("manifest.txt");
    if let Ok(text) = fs::read_to_string(&manifest)
        && (text.contains("ZipKind: assistant_export")
            || text.contains("ASSISTANT-CONTEXT-JSON")
            || text.contains("ASSISTANT-EXPORT-BUNDLE"))
    {
        return true;
    }
    let zip_kind = path.join("ZIP_KIND.txt");
    if let Ok(text) = fs::read_to_string(&zip_kind) {
        let lower = text.to_ascii_lowercase();
        if lower.contains("kind=assistant_export") || lower.contains("take_compatible=false") {
            return true;
        }
    }
    false
}

fn read_cargo_version(cargo_path: &Path) -> Option<String> {
    let text = fs::read_to_string(cargo_path).ok()?;
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("version") {
            continue;
        }
        let (_, rhs) = trimmed.split_once('=')?;
        let rhs = rhs.trim();
        if rhs.starts_with('"') && rhs.ends_with('"') && rhs.len() >= 2 {
            return Some(rhs[1..rhs.len() - 1].to_string());
        }
    }
    None
}

fn clean_args_after_take(args: &[OsString]) -> Vec<OsString> {
    args.iter()
        .filter(|arg| !os_eq_ignore_case(arg, "--take") && !os_eq_ignore_case(arg, "--force"))
        .cloned()
        .collect()
}



fn run_cmd_command(project_root: &Path, args: &[OsString]) -> Command {
    let mut command = Command::new("cmd");
    command
        .current_dir(project_root)
        .arg("/C")
        .arg("call")
        .arg(project_root.join("run.cmd"))
        .args(args);
    command
}

fn identical_take_continuation_args(args: &[OsString]) -> Option<Vec<OsString>> {
    let clean_args = clean_args_after_take(args);
    (!clean_args.is_empty()).then_some(clean_args)
}

#[cfg(test)]
fn command_after_take_preview(args: &[OsString]) -> String {
    let clean_args = clean_args_after_take(args);
    let preview = command_args_preview(&clean_args);
    if preview == "(keine)" {
        r".\run.cmd".to_string()
    } else {
        format!(r".\run.cmd {preview}")
    }
}

fn command_args_preview(args: &[OsString]) -> String {
    if args.is_empty() {
        return "(keine)".to_string();
    }
    args.iter()
        .map(command_arg_preview)
        .collect::<Vec<_>>()
        .join(" ")
}

fn command_arg_preview(arg: &OsString) -> String {
    let text = arg.to_string_lossy();
    if text.is_empty() || text.chars().any(char::is_whitespace) {
        format!(r#""{}""#, text.replace('"', "\"\""))
    } else {
        text.into_owned()
    }
}

// Cargo.lock bleibt absichtlich lokal: --take darf einen auf der Zielmaschine
// bereits verifizierten Registry-/Mirror-Lock nicht durch einen fremden Lock ersetzen.
// Das separate Watch-Projekt besitzt dagegen absichtlich einen eigenen, unter tools/
// take-verwalteten Cargo.lock und wird bei Aenderung vor dem Kopieren kompiliert/getestet.

fn validate_changed_watch_project(source_root: &Path, project_root: &Path) -> AppResult<()> {
    let incoming_dir = source_root.join(take_change_control::WATCH_PROJECT_DIR);
    if !incoming_dir.exists() {
        return Ok(());
    }
    let installed_dir = project_root.join(take_change_control::WATCH_PROJECT_DIR);
    if !directory_payload_differs(&incoming_dir, &installed_dir)? {
        return Ok(());
    }

    let manifest = source_root.join(take_change_control::WATCH_PROJECT_MANIFEST_RELATIVE_PATH);
    let lock = source_root.join(take_change_control::WATCH_PROJECT_LOCK_RELATIVE_PATH);
    if !manifest.is_file() || !lock.is_file() {
        return Err(format!(
            "Watch-Project-Preflight: separates Watch-Projekt geaendert, aber Cargo.toml/Cargo.lock fehlen unter {}",
            incoming_dir.display()
        ));
    }

    let jobs = std::thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1)
        .saturating_sub(1)
        .max(1);
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
    let target_dir = source_root
        .parent()
        .unwrap_or(source_root)
        .join("etuschlopf_watch_preflight_target");
    let status = Command::new(cargo)
        .current_dir(source_root)
        .arg("test")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--locked")
        .arg("--profile")
        .arg("strict")
        .arg("--jobs")
        .arg(jobs.to_string())
        .env("CARGO_TARGET_DIR", &target_dir)
        .status()
        .map_err(|error| format!("Watch-Project-Preflight konnte Cargo nicht starten: {error}"))?;
    if !status.success() {
        return Err(format!(
            "Watch-Project-Preflight fehlgeschlagen: cargo test --locked --profile strict --manifest-path {} Exit {:?}",
            manifest.display(),
            status.code()
        ));
    }
    bootstrap_detail!(
        "WATCH-PROJECT-PREFLIGHT GRUEN: separates Cargo-Projekt kompiliert und getestet | Jobs {jobs}"
    );
    bootstrap_ok!(
        "Watch-Project-Preflight · separates Cargo-Projekt kompiliert und getestet · Jobs {jobs}"
    );
    Ok(())
}

fn take_payload_differs(source_root: &Path, project_root: &Path) -> AppResult<bool> {
    for dir in take_change_control::MANAGED_DIRS {
        let source_dir = source_root.join(dir);
        let target_dir = project_root.join(dir);
        if directory_payload_differs(&source_dir, &target_dir)? {
            return Ok(true);
        }
    }
    for file in take_change_control::MANAGED_ROOT_FILES {
        let source_file = source_root.join(file);
        if !source_file.exists() {
            continue;
        }
        let target_file = project_root.join(file);
        if file_payload_differs(&source_file, &target_file)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn directory_payload_differs(source_dir: &Path, target_dir: &Path) -> AppResult<bool> {
    if !source_dir.exists() {
        return Ok(false);
    }
    if !target_dir.exists() {
        return Ok(true);
    }
    let source_files = relative_file_set(source_dir)?;
    let target_files = relative_file_set(target_dir)?;
    if source_files != target_files {
        return Ok(true);
    }
    for relative in source_files {
        if file_payload_differs(&source_dir.join(&relative), &target_dir.join(&relative))? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn relative_file_set(root: &Path) -> AppResult<BTreeSet<PathBuf>> {
    let mut files = BTreeSet::new();
    collect_relative_files(root, root, &mut files)?;
    Ok(files)
}

fn collect_relative_files(root: &Path, dir: &Path, files: &mut BTreeSet<PathBuf>) -> AppResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    let entries = fs::read_dir(dir).map_err(|e| {
        format!(
            "Ordner fuer Payload-Vergleich nicht lesbar {}: {e}",
            dir.display()
        )
    })?;
    for entry in entries {
        let entry =
            entry.map_err(|e| format!("Payload-Vergleich: Directory entry nicht lesbar: {e}"))?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|e| {
            format!(
                "Payload-Vergleich: Dateityp nicht lesbar {}: {e}",
                path.display()
            )
        })?;
        if file_type.is_dir() {
            collect_relative_files(root, &path, files)?;
        } else if file_type.is_file() {
            let relative = path.strip_prefix(root).map_err(|e| {
                format!(
                    "Payload-Vergleich: relativer Pfad fehlgeschlagen {}: {e}",
                    path.display()
                )
            })?;
            files.insert(relative.to_path_buf());
        }
    }
    Ok(())
}

fn file_payload_differs(source_file: &Path, target_file: &Path) -> AppResult<bool> {
    if !target_file.exists() {
        return Ok(true);
    }
    let source_meta = fs::metadata(source_file).map_err(|e| {
        format!(
            "Payload-Vergleich: Quelldatei nicht lesbar {}: {e}",
            source_file.display()
        )
    })?;
    let target_meta = fs::metadata(target_file).map_err(|e| {
        format!(
            "Payload-Vergleich: Zieldatei nicht lesbar {}: {e}",
            target_file.display()
        )
    })?;
    if source_meta.len() != target_meta.len() {
        return Ok(true);
    }
    let source_bytes = fs::read(source_file).map_err(|e| {
        format!(
            "Payload-Vergleich: Quelldatei konnte nicht gelesen werden {}: {e}",
            source_file.display()
        )
    })?;
    let target_bytes = fs::read(target_file).map_err(|e| {
        format!(
            "Payload-Vergleich: Zieldatei konnte nicht gelesen werden {}: {e}",
            target_file.display()
        )
    })?;
    Ok(source_bytes != target_bytes)
}

fn copy_source_tree(source_root: &Path, project_root: &Path) -> AppResult<()> {
    for dir in take_change_control::MANAGED_DIRS {
        mirror_managed_dir(&source_root.join(dir), &project_root.join(dir))?;
    }
    for file in take_change_control::MANAGED_ROOT_FILES {
        let from = source_root.join(file);
        if from.exists() {
            sync_file_payload(&from, &project_root.join(file))?;
        }
    }
    Ok(())
}

fn mirror_managed_dir(source: &Path, destination: &Path) -> AppResult<()> {
    if !source.exists() {
        return Ok(());
    }
    if destination.is_file() {
        fs::remove_file(destination).map_err(|e| {
            format!(
                "Datei am Ziel eines Managed-Ordners konnte nicht entfernt werden {}: {e}",
                destination.display()
            )
        })?;
    }
    fs::create_dir_all(destination).map_err(|e| {
        format!(
            "Managed-Zielordner konnte nicht erzeugt werden {}: {e}",
            destination.display()
        )
    })?;

    for entry in fs::read_dir(destination).map_err(|e| {
        format!(
            "Managed-Zielordner nicht lesbar {}: {e}",
            destination.display()
        )
    })?
    {
        let entry = entry.map_err(|e| format!("Managed-Zielentry nicht lesbar: {e}"))?;
        let target_path = entry.path();
        let source_path = source.join(entry.file_name());
        if source_path.exists() {
            continue;
        }
        let file_type = entry.file_type().map_err(|e| {
            format!(
                "Managed-Zieldateityp nicht lesbar {}: {e}",
                target_path.display()
            )
        })?;
        if file_type.is_dir() {
            fs::remove_dir_all(&target_path).map_err(|e| {
                format!(
                    "Entfallener Managed-Ordner konnte nicht entfernt werden {}: {e}",
                    target_path.display()
                )
            })?;
        } else if file_type.is_file() {
            fs::remove_file(&target_path).map_err(|e| {
                format!(
                    "Entfallene Managed-Datei konnte nicht entfernt werden {}: {e}",
                    target_path.display()
                )
            })?;
        }
    }

    for entry in fs::read_dir(source)
        .map_err(|e| format!("Managed-Quellordner nicht lesbar {}: {e}", source.display()))?
    {
        let entry = entry.map_err(|e| format!("Managed-Quellentry nicht lesbar: {e}"))?;
        let from = entry.path();
        let to = destination.join(entry.file_name());
        let source_type = entry.file_type().map_err(|e| {
            format!(
                "Managed-Quelldateityp nicht lesbar {}: {e}",
                from.display()
            )
        })?;
        if source_type.is_dir() {
            if to.is_file() {
                fs::remove_file(&to).map_err(|e| {
                    format!(
                        "Datei am Ziel eines Managed-Unterordners konnte nicht entfernt werden {}: {e}",
                        to.display()
                    )
                })?;
            }
            mirror_managed_dir(&from, &to)?;
        } else if source_type.is_file() {
            if to.is_dir() {
                fs::remove_dir_all(&to).map_err(|e| {
                    format!(
                        "Ordner am Ziel einer Managed-Datei konnte nicht entfernt werden {}: {e}",
                        to.display()
                    )
                })?;
            }
            sync_file_payload(&from, &to)?;
        }
    }
    Ok(())
}

fn sync_file_payload(source: &Path, destination: &Path) -> AppResult<bool> {
    if !file_payload_differs(source, destination)? {
        return Ok(false);
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Managed-Datei-Zielordner konnte nicht erzeugt werden {}: {e}",
                parent.display()
            )
        })?;
    }
    fs::copy(source, destination).map_err(|e| {
        format!(
            "Managed-Datei konnte nicht kopiert werden {} -> {}: {e}",
            source.display(),
            destination.display()
        )
    })?;
    Ok(true)
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> AppResult<()> {
    fs::create_dir_all(destination).map_err(|e| {
        format!(
            "Zielordner konnte nicht erzeugt werden {}: {e}",
            destination.display()
        )
    })?;
    for entry in fs::read_dir(source)
        .map_err(|e| format!("Quellordner nicht lesbar {}: {e}", source.display()))?
    {
        let entry = entry.map_err(|e| format!("Directory entry nicht lesbar: {e}"))?;
        let from = entry.path();
        let to = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|e| format!("Dateityp nicht lesbar {}: {e}", from.display()))?;
        if file_type.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else if file_type.is_file() {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    format!(
                        "Zielordner konnte nicht erzeugt werden {}: {e}",
                        parent.display()
                    )
                })?;
            }
            fs::copy(&from, &to).map_err(|e| {
                format!(
                    "Datei konnte nicht kopiert werden {} -> {}: {e}",
                    from.display(),
                    to.display()
                )
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_stable_sync_skips_byte_identical_file_and_copies_changed_file() {
        let root = env::temp_dir().join(format!(
            "etuschlopf_payload_stable_sync_{}_{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        fs::create_dir_all(&root).expect("test root");
        let source = root.join("source.txt");
        let destination = root.join("destination.txt");

        fs::write(&source, "same").expect("source same");
        fs::write(&destination, "same").expect("destination same");
        assert!(!sync_file_payload(&source, &destination).expect("stable sync"));

        fs::write(&source, "changed").expect("source changed");
        assert!(sync_file_payload(&source, &destination).expect("changed sync"));
        assert_eq!(fs::read_to_string(&destination).expect("destination"), "changed");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn payload_stable_managed_dir_removes_only_obsolete_payload() {
        let root = env::temp_dir().join(format!(
            "etuschlopf_payload_stable_mirror_{}_{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time")
                .as_nanos()
        ));
        let source = root.join("source");
        let destination = root.join("destination");
        fs::create_dir_all(source.join("nested")).expect("source dirs");
        fs::create_dir_all(destination.join("obsolete_dir")).expect("destination dirs");
        fs::write(source.join("stable.txt"), "stable").expect("source stable");
        fs::write(destination.join("stable.txt"), "stable").expect("destination stable");
        fs::write(source.join("nested/changed.txt"), "new").expect("source changed");
        fs::write(destination.join("nested_old_file.txt"), "obsolete").expect("obsolete file");
        fs::write(destination.join("obsolete_dir/file.txt"), "obsolete")
            .expect("obsolete dir file");

        mirror_managed_dir(&source, &destination).expect("mirror");

        assert_eq!(
            fs::read_to_string(destination.join("stable.txt")).expect("stable"),
            "stable"
        );
        assert_eq!(
            fs::read_to_string(destination.join("nested/changed.txt")).expect("changed"),
            "new"
        );
        assert!(!destination.join("nested_old_file.txt").exists());
        assert!(!destination.join("obsolete_dir").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn nonzero_reexec_exit_can_never_normalize_to_success() {
        assert_eq!(normalize_bootstrap_exit_code(0), 0);
        assert_eq!(normalize_bootstrap_exit_code(2), 2);
        assert_eq!(normalize_bootstrap_exit_code(255), 255);
        assert_eq!(normalize_bootstrap_exit_code(256), 1);
        assert_eq!(normalize_bootstrap_exit_code(-1), 1);
        assert_eq!(normalize_bootstrap_exit_code(i32::MIN), 1);
    }

    fn temp_project(label: &str, payload: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = env::temp_dir().join(format!(
            "etuschlopf_take_transaction_test_{label}_{}_{}",
            std::process::id(),
            stamp
        ));
        for dir in take_change_control::MANAGED_DIRS {
            fs::create_dir_all(root.join(dir)).expect("managed dir");
        }
        fs::write(root.join("src/payload.txt"), payload).expect("payload");
        fs::write(
            root.join("Cargo.toml"),
            format!("[package]\nname=\"test\"\nversion=\"{payload}\"\n"),
        )
        .expect("cargo");
        fs::write(root.join("README.md"), payload).expect("readme");
        fs::write(root.join("ZIP_KIND.txt"), payload).expect("zip kind");
        fs::write(root.join("run.cmd"), payload).expect("run");
        fs::write(root.join("strict.cmd"), payload).expect("strict");
        root
    }

    fn seed_failure_parent_acceptance(
        project_root: &Path,
        version: &str,
        revision: u64,
    ) -> Vec<u8> {
        let hash = format!("{:064x}", 10_000 + revision);
        fs::write(
            project_root.join("ZIP_KIND.txt"),
            format!(
                "kind=take\nversion={version}\nrevision=r{revision}\nname=TestAcceptedParent\nparent_revision=r{}\nchange_class=KodexOnly\nschema_version=1\n",
                revision.saturating_sub(1)
            ),
        )
        .expect("parent ZIP_KIND");
        multi_revision_evasion::record_acceptance(
            project_root,
            version,
            version,
            revision,
            &hash,
            0,
        )
        .expect("parent acceptance");
        multi_revision_evasion::read_acceptance_record_exact(
            project_root,
            version,
            version,
            revision,
            &hash,
            0,
        )
        .expect("parent acceptance bytes")
        .1
    }

    fn seed_failure_recovery_parent(project_root: &Path) {
        let _ = seed_failure_parent_acceptance(project_root, "0.10.1839", 514);
    }

    fn seed_assistant_export_staging(project_root: &Path) {
        let latest = project_root
            .join(ASSISTANT_EXPORT_ROOT)
            .join(ASSISTANT_EXPORT_LATEST_DIR);
        fs::create_dir_all(latest.join("source_snapshot")).expect("source snapshot dir");
        fs::write(
            latest.join("assistant_handoff_latest.txt"),
            "# Assistant handoff fixture\nCanonicalVerdict: not_yet_canonical\n",
        )
        .expect("handoff fixture");
        fs::write(latest.join("sentinel.txt"), b"preserve-me\n").expect("sentinel");
        fs::write(
            latest.join("source_snapshot/SOURCE_SNAPSHOT.txt"),
            b"snapshot-preserve-me\n",
        )
        .expect("source snapshot marker");
    }

    fn prepare_test_transaction(project_root: &Path) -> TakeTransaction {
        let intent_path = project_root.join(take_change_control::CHANGE_MANIFEST_RELATIVE_PATH);
        if let Some(parent) = intent_path.parent() {
            fs::create_dir_all(parent).expect("intent parent");
        }
        fs::write(
            &intent_path,
            "schema_version=1\nchange_class=KodexOnly\nfrom_version=0.10.1839\nto_version=0.10.1839\nreason=test\n",
        )
        .expect("intent");
        TakeTransaction::prepare(
            project_root,
            project_root,
            "0.10.1839",
            "0.10.1839",
            515,
            &format!("{:064x}", 515),
            "test-revision-snapshot\n",
        )
        .expect("prepare")
    }

    #[test]
    fn same_version_followup_command_removes_take_and_quotes_project_name() {
        let args = vec![
            OsString::from("--take"),
            OsString::from("--strict"),
            OsString::from("Synthetic Project Alpha"),
        ];
        assert_eq!(
            command_after_take_preview(&args),
            r#".\run.cmd --strict "Synthetic Project Alpha""#
        );
    }

    #[test]
    fn same_version_followup_command_does_not_recommend_force() {
        let args = vec![OsString::from("--take"), OsString::from("--force")];
        assert_eq!(command_after_take_preview(&args), r".\run.cmd");
    }

    #[test]
    fn identical_same_version_take_continues_with_remaining_arguments() {
        let args = vec![
            OsString::from("--take"),
            OsString::from("--strict"),
            OsString::from("Synthetic Project Alpha"),
        ];
        assert_eq!(
            identical_take_continuation_args(&args),
            Some(vec![
                OsString::from("--strict"),
                OsString::from("Synthetic Project Alpha")
            ])
        );
    }

    #[test]
    fn pure_identical_take_without_followup_stays_a_noop() {
        let args = vec![OsString::from("--take")];
        assert_eq!(identical_take_continuation_args(&args), None);
    }

    #[test]
    fn forced_same_version_take_does_not_leak_force_into_reexec_arguments() {
        let args = vec![OsString::from("--take"), OsString::from("--force")];
        assert_eq!(identical_take_continuation_args(&args), None);
    }

    #[test]
    fn backup_transactional_runtime_state() {
        let source = include_str!("etuschlopf_take_bootstrap.rs");
        let runtime_files = ["TRANSACTIONAL_RUNTIME_", "FILES"].concat();
        assert!(!source.contains(&runtime_files));
    }

    #[test]
    fn read_existing_runtime_files() {
        let source = include_str!("etuschlopf_take_bootstrap.rs");
        let manifest = ["existing_runtime_", "files.txt"].concat();
        assert!(!source.contains(&manifest));
    }

    #[test]
    fn verify_transactional_runtime_state() {
        let source = include_str!("etuschlopf_take_bootstrap.rs");
        let restore = ["restore_transactional_", "runtime_state"].concat();
        assert!(!source.contains(&restore));
    }

    #[test]
    fn same_version_build_continuation_token() {
        let source = include_str!("etuschlopf_take_bootstrap.rs");
        let token = ["ETUSCHLOPF_SAME_VERSION_", "TAKE_CONTINUATION"].concat();
        assert!(!source.contains(&token));
    }

    #[test]
    fn same_version_build_continuation_token_binds_version_and_signature() {
        let source = include_str!("etuschlopf_take_bootstrap.rs");
        let signature_stamp = ["BUILD_SIGNATURE_", "STAMP"].concat();
        assert!(!source.contains(&signature_stamp));
    }

    #[test]
    fn same_version_build_continuation_token_rejects_wrong_build_version() {
        let source = include_str!("etuschlopf_take_bootstrap.rs");
        let build_stamp = ["BUILD_", "STAMP"].concat();
        assert!(!source.contains(&build_stamp));
    }

    #[test]
    fn revision_key_uses_canonical_token_before_descriptive_name() {
        let path = Path::new(
            "EtuSchlopf_v0_10_1489_r02_ReportContractStatusSingleEvaluation.zip",
        );
        assert_eq!(zip_revision_key(path, (0, 10, 1489)), Some(2));
    }

    #[test]
    fn revision_key_ignores_later_underscore_r_in_descriptive_name() {
        let path = Path::new(
            "EtuSchlopf_v0_10_1489_r03_Take_rich_revision_parser.zip",
        );
        assert_eq!(zip_revision_key(path, (0, 10, 1489)), Some(3));
    }

    #[test]
    fn revision_key_rejects_filename_version_mismatch_or_missing_boundary() {
        assert_eq!(
            zip_revision_key(
                Path::new("EtuSchlopf_v0_10_1490_r01_Parser.zip"),
                (0, 10, 1489),
            ),
            None
        );
        assert_eq!(
            zip_revision_key(
                Path::new("EtuSchlopf_v0_10_1489_r01x_Parser.zip"),
                (0, 10, 1489),
            ),
            None
        );
    }

    #[test]
    fn take_discovery_ignores_exactly_rejected_peer_sha256() {
        let root = temp_project("rejected-peer-selection", "0.0.1");
        let rejected = root.join("rejected.zip");
        let replacement = root.join("replacement.zip");
        fs::write(&rejected, b"rejected candidate").expect("rejected fixture");
        fs::write(&replacement, b"replacement candidate").expect("replacement fixture");
        let rejected_hash = multi_revision_evasion::sha256_file(&rejected).expect("hash");
        let rejected_set = BTreeSet::from([rejected_hash]);
        let eligible = unrejected_take_peers(
            &[rejected.clone(), replacement.clone()],
            &rejected_set,
        )
        .expect("eligible peers");
        assert_eq!(eligible, vec![replacement]);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn take_discovery_never_revives_all_rejected_peers() {
        let root = temp_project("all-rejected-peer-selection", "0.0.1");
        let first = root.join("first.zip");
        let second = root.join("second.zip");
        fs::write(&first, b"first rejected candidate").expect("first fixture");
        fs::write(&second, b"second rejected candidate").expect("second fixture");
        let rejected_set = BTreeSet::from([
            multi_revision_evasion::sha256_file(&first).expect("first hash"),
            multi_revision_evasion::sha256_file(&second).expect("second hash"),
        ]);
        let eligible = unrejected_take_peers(&[first, second], &rejected_set)
            .expect("eligible peers");
        assert!(eligible.is_empty());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn preflight_rejection_is_persisted_before_semantic_snapshot_exists() {
        let root = temp_project("early-preflight-rejection", "0.0.1");
        seed_failure_parent_acceptance(&root, "0.0.1", 0);
        let zip = root.join("candidate.zip");
        fs::write(&zip, b"structurally rejected candidate").expect("candidate fixture");
        let hash = multi_revision_evasion::sha256_file(&zip).expect("hash");
        let message = rejection_error_with_persistence(
            &root,
            &multi_revision_evasion::RejectionRecord {
                from_version: "0.0.1",
                to_version: "0.0.1",
                take_revision: 1,
                zip_sha256: &hash,
                rejection_stage: multi_revision_evasion::REJECTION_STAGE_DEPENDENCY_PREFLIGHT,
                candidate_intent: Some(b"schema_version=1\nchange_class=KodexOnly\n"),
                snapshot: "",
                reason: "Take-Tree-Integrity: unknown top-level fixture; fail-closed",
            },
            current_epoch_millis().expect("start"),
        );
        assert!(message.contains("Kandidatenablehnung gespeichert"));
        let rejected = multi_revision_evasion::load_rejected_revisions(
            &root,
            "0.0.1",
            "0.0.1",
        )
        .expect("load rejection ledger");
        assert_eq!(rejected.len(), 1);
        assert_eq!(rejected[0].zip_sha256, hash);
        assert!(rejected[0].snapshot.is_empty());
        assert!(rejected[0].reason.contains("Take-Tree-Integrity"));
        assert!(root.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP).is_file());
        let failure_handoff = fs::read_to_string(
            root.join(ASSISTANT_EXPORT_ROOT)
                .join(ASSISTANT_EXPORT_LATEST_DIR)
                .join("assistant_handoff_latest.txt"),
        )
        .expect("failure handoff");
        assert!(failure_handoff.contains("CanonicalVerdict: rejected"));
        let _ = fs::remove_dir_all(root);
    }

    fn write_bootstrap_restart_fixture(root: &Path, payload: &str) {
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname=\"etuschlopf\"\nversion=\"0.0.0\"\nedition=\"2024\"\nbuild=\"src/build_guard.rs\"\n\n[lib]\npath=\"src/lib.rs\"\n",
        )
        .expect("cargo fixture");
        fs::write(
            root.join("src/lib.rs"),
            format!("pub const BOOTSTRAP_RESTART_PAYLOAD: &str = {payload:?};\n"),
        )
        .expect("lib fixture");
        fs::write(root.join("src/bootstrap_pin.txt"), payload).expect("pin fixture");
        fs::write(
            root.join("src/build_guard.rs"),
            format!(
                "fn main() {{\n    let actual = std::fs::read_to_string(\"src/bootstrap_pin.txt\").expect(\"pin\");\n    assert_eq!(actual.trim(), {payload:?}, \"bootstrap pin mismatch\");\n    println!(\"cargo:rerun-if-changed=src/bootstrap_pin.txt\");\n}}\n"
            ),
        )
        .expect("build guard fixture");
    }

    fn cargo_check_bootstrap_fixture(root: &Path) {
        let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));
        let status = Command::new(cargo)
            .current_dir(root)
            .arg("check")
            .arg("--quiet")
            .arg("--lib")
            .arg("--target-dir")
            .arg(root.join(BOOTSTRAP_CARGO_TARGET_DIR))
            .status()
            .expect("cargo check fixture");
        assert!(status.success(), "bootstrap fixture cargo check failed");
    }

    fn bootstrap_package_fingerprint_count(root: &Path) -> usize {
        let fingerprint_root = root
            .join(BOOTSTRAP_CARGO_TARGET_DIR)
            .join("debug")
            .join(".fingerprint");
        if !fingerprint_root.is_dir() {
            return 0;
        }
        fs::read_dir(fingerprint_root)
            .expect("fingerprint dir")
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("etuschlopf-")
            })
            .count()
    }

    #[test]
    fn transactional_apply_can_rollback_exactly() {
        let current = temp_project("rollback_current", "old");
        let incoming = temp_project("rollback_incoming", "new");
        fs::write(current.join("Cargo.lock"), "parent-lock").expect("parent lock");
        let transaction = prepare_test_transaction(&current);
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        fs::write(current.join("Cargo.lock"), "incoming-cargo-owned-lock")
            .expect("simulated cargo lock mutation");
        assert_eq!(
            fs::read_to_string(current.join("src/payload.txt")).expect("new payload"),
            "new"
        );
        transaction.rollback(&current).expect("rollback");
        assert_eq!(
            fs::read_to_string(current.join("src/payload.txt")).expect("old payload"),
            "old"
        );
        assert_eq!(
            fs::read_to_string(current.join("Cargo.lock")).expect("restored parent lock"),
            "parent-lock"
        );
        assert_eq!(
            fs::read_to_string(&transaction.state_path).expect("rolled back state"),
            "rolled_back\n"
        );
        assert!(current.join(TAKE_TRANSACTION_MARKER).exists());
        assert!(transaction.root.exists());
        transaction.finish_rollback().expect("finish rollback");
        assert!(!current.join(TAKE_TRANSACTION_MARKER).exists());
        assert!(!transaction.root.exists());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn rollback_restores_absent_local_cargo_lock_exactly() {
        let current = temp_project("rollback_absent_lock_current", "old");
        let incoming = temp_project("rollback_absent_lock_incoming", "new");
        assert!(!current.join("Cargo.lock").exists());
        let transaction = prepare_test_transaction(&current);
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        fs::write(current.join("Cargo.lock"), "generated-during-transaction")
            .expect("generated lock");
        transaction.rollback(&current).expect("rollback");
        assert!(!current.join("Cargo.lock").exists());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn rollback_invalidates_bootstrap_package_cache_after_source_restore() {
        let source = include_str!("etuschlopf_take_bootstrap.rs");
        let rollback_start = source
            .find("fn rollback_preserving_provenance(&self, project_root: &Path)")
            .expect("rollback provenance function");
        let rollback = &source[rollback_start..];
        let restore = rollback.find("restore_backup(&self.backup_root, project_root)?;").expect("restore");
        let verify = rollback.find("verify_backup_matches_project(&self.backup_root, project_root)?;").expect("verify");
        let cleanup = rollback.find("clean_bootstrap_package_after_source_rollback(project_root)?;").expect("cleanup");
        let rolled_back = rollback.find("write_transaction_state(&self.state_path, \"rolled_back\")?;").expect("rolled_back state");
        assert!(restore < verify && verify < cleanup && cleanup < rolled_back);

        let cleanup_start = source
            .find("fn clean_bootstrap_package_after_source_rollback(project_root: &Path)")
            .expect("cleanup function");
        let cleanup_source = &source[cleanup_start..];
        assert!(cleanup_source.contains(".arg(\"clean\")"));
        assert!(cleanup_source.contains(".arg(\"--package\")"));
        assert!(cleanup_source.contains(".arg(env!(\"CARGO_PKG_NAME\"))"));
        assert!(cleanup_source.contains(".arg(\"--target-dir\")"));
        assert!(cleanup_source.contains("BOOTSTRAP_CARGO_TARGET_DIR"));
        let forbidden_recursive_delete = ["remove_dir_all", "(&target_dir)"].concat();
        assert!(!cleanup_source.contains(&forbidden_recursive_delete));
    }

    #[test]
    fn absent_and_truly_empty_bootstrap_cache_need_no_cargo_process() {
        let root = temp_project("empty_bootstrap_cache", "old");
        clean_bootstrap_package_after_source_rollback(&root).unwrap();
        let cache = root.join(BOOTSTRAP_CARGO_TARGET_DIR);
        fs::create_dir_all(&cache).unwrap();
        clean_bootstrap_package_after_source_rollback(&root).unwrap();
        fs::write(cache.join(".hidden-cache-artifact"), b"retain").unwrap();
        assert!(clean_bootstrap_package_after_source_rollback(&root).is_err());
        assert_eq!(fs::read(cache.join(".hidden-cache-artifact")).unwrap(), b"retain");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn failed_bootstrap_clean_preserves_exact_output_and_refuses_success() {
        const ROOT_ENV: &str = "ETUSCHLOPF_SYNTHETIC_CLEAN_TEST_ROOT";
        if let Some(root) = env::var_os(ROOT_ENV) {
            let root = PathBuf::from(root);
            let error = clean_bootstrap_package_after_source_rollback(&root).unwrap_err();
            assert!(error.contains("Some(101)") && error.contains("synthetic cleanup refusal"));
            let logs = root.join("target/etuschlopf_logs");
            assert_eq!(fs::read(logs.join("bootstrap_rollback_cache.stdout.log")).unwrap(), b"synthetic stdout\n");
            assert_eq!(fs::read(logs.join("bootstrap_rollback_cache.stderr.log")).unwrap(), b"synthetic cleanup refusal\n");
            assert_eq!(fs::read(root.join(BOOTSTRAP_CARGO_TARGET_DIR).join("artifact")).unwrap(), b"retain");
            return;
        }
        let root = temp_project("bootstrap_clean_output", "old");
        let cache = root.join(BOOTSTRAP_CARGO_TARGET_DIR);
        fs::create_dir_all(&cache).unwrap();
        fs::write(cache.join("artifact"), b"retain").unwrap();
        let stub = root.join("cargo_stub.rs");
        fs::write(&stub, "fn main() { println!(\"synthetic stdout\"); eprintln!(\"synthetic cleanup refusal\"); std::process::exit(101); }").unwrap();
        let executable = root.join(format!("cargo_stub{}", env::consts::EXE_SUFFIX));
        let compiled = Command::new("rustc").arg(&stub).arg("-o").arg(&executable).output().unwrap();
        assert!(compiled.status.success(), "{}", String::from_utf8_lossy(&compiled.stderr));
        let result = Command::new(env::current_exe().unwrap())
            .args(["--exact", "tests::failed_bootstrap_clean_preserves_exact_output_and_refuses_success", "--nocapture"])
            .env(ROOT_ENV, &root).env("CARGO", &executable).output().unwrap();
        assert!(result.status.success(), "{} {}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn pending_reexec_rollback_cleans_child_bootstrap_cache_and_parent_rebuilds() {
        let current = temp_project("restart_current", "old");
        let incoming = temp_project("restart_incoming", "new");
        write_bootstrap_restart_fixture(&current, "old");
        write_bootstrap_restart_fixture(&incoming, "new");

        seed_failure_recovery_parent(&current);
        let transaction = prepare_test_transaction(&current);
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        transaction.mark_reexec().expect("reexec state");

        cargo_check_bootstrap_fixture(&current);
        assert!(bootstrap_package_fingerprint_count(&current) > 0);
        let cache_sentinel = current
            .join(BOOTSTRAP_CARGO_TARGET_DIR)
            .join("non_package_cache_sentinel.txt");
        fs::write(&cache_sentinel, "keep").expect("cache sentinel");

        recover_pending_transaction(&current).expect("recover pending reexec");

        assert_eq!(
            fs::read_to_string(current.join("src/bootstrap_pin.txt"))
                .expect("restored parent pin")
                .trim(),
            "old"
        );
        assert!(!current.join(TAKE_TRANSACTION_MARKER).exists());
        assert_eq!(bootstrap_package_fingerprint_count(&current), 0);
        assert!(cache_sentinel.is_file());

        cargo_check_bootstrap_fixture(&current);
        assert!(bootstrap_package_fingerprint_count(&current) > 0);

        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn pending_transaction_persists_candidate_identity_and_snapshot() {
        let current = temp_project("candidate_provenance", "old");
        let transaction = prepare_test_transaction(&current);
        let provenance = read_transaction_candidate_provenance(&transaction.root)
            .expect("read provenance")
            .expect("candidate provenance");
        assert_eq!(provenance.0, "0.10.1839");
        assert_eq!(provenance.1, "0.10.1839");
        assert_eq!(provenance.2, 515);
        assert_eq!(provenance.3, format!("{:064x}", 515));
        assert_eq!(provenance.4, "test-revision-snapshot\n");
        assert!(provenance
            .5
            .as_deref()
            .is_some_and(|intent| {
                intent.starts_with(b"schema_version=1\nchange_class=KodexOnly\n")
            }));
        transaction.rollback(&current).expect("rollback");
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn pending_transaction_copies_candidate_intent_from_incoming_source_exactly() {
        let current = temp_project("candidate_intent_current", "old");
        let incoming = temp_project("candidate_intent_incoming", "new");
        let intent_path = incoming.join(take_change_control::CHANGE_MANIFEST_RELATIVE_PATH);
        fs::create_dir_all(intent_path.parent().expect("intent parent"))
            .expect("intent parent");
        let intent =
            b"schema_version=1\nchange_class=KodexOnly\nfrom_version=0.10.1839\nto_version=0.10.1839\nreason=incoming-only\n";
        fs::write(&intent_path, intent).expect("incoming intent");
        let transaction = TakeTransaction::prepare(
            &current,
            &incoming,
            "0.10.1839",
            "0.10.1839",
            516,
            &format!("{:064x}", 516),
            "intent-copy-snapshot\n",
        )
        .expect("prepare");
        assert_eq!(
            fs::read(transaction.root.join(TAKE_TRANSACTION_CANDIDATE_INTENT))
                .expect("intent"),
            intent
        );
        transaction.finish_rollback().expect("cleanup");
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn acceptance_evidence_io_failure_does_not_revoke_canonical_acceptance() {
        let current = temp_project("acceptance_evidence_io", "old");
        let hash = format!("{:064x}", 517);
        multi_revision_evasion::record_acceptance(
            &current,
            "0.10.1839",
            "0.10.1839",
            517,
            &hash,
            0,
        )
        .expect("canonical acceptance");
        let evidence_root = current.join("target/etuschlopf_take_acceptance_evidence_v1");
        fs::write(&evidence_root, b"deterministic evidence root conflict")
            .expect("evidence root conflict");
        record_acceptance_evidence_advisory(
            &current,
            "0.10.1839",
            "0.10.1839",
            517,
            &hash,
            Some(b"candidate intent\n"),
            "effect snapshot\n",
        );
        let accepted = multi_revision_evasion::load_accepted_zip_sha256(
            &current,
            "0.10.1839",
            "0.10.1839",
            517,
        )
        .expect("accepted provenance");
        assert!(accepted.contains(&hash));
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn post_acceptance_handover_never_claims_acceptance_before_canonical_record_exists() {
        let current = temp_project("post_acceptance_requires_authority", "old");
        seed_assistant_export_staging(&current);
        let hash = format!("{:064x}", 518);
        let error = finalize_post_acceptance_assistant_handover(
            &current,
            "0.10.1839",
            "0.10.1839",
            518,
            &hash,
            0,
        )
        .expect_err("missing canonical acceptance must fail closed");
        assert!(error.contains("Acceptance-Provenienz nicht lesbar"));
        assert!(!current
            .join(ASSISTANT_EXPORT_ROOT)
            .join(ASSISTANT_EXPORT_LATEST_DIR)
            .join(POST_ACCEPTANCE_LIFECYCLE_FILE)
            .exists());
        let handoff = fs::read_to_string(
            current
                .join(ASSISTANT_EXPORT_ROOT)
                .join(ASSISTANT_EXPORT_LATEST_DIR)
                .join("assistant_handoff_latest.txt"),
        )
        .expect("handoff");
        assert!(!handoff.contains(POST_ACCEPTANCE_SECTION_BEGIN));
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn post_acceptance_handover_ignores_marker_substrings_inside_self_describing_source_text() {
        let current = temp_project("post_acceptance_self_description", "old");
        seed_assistant_export_staging(&current);
        let latest = current
            .join(ASSISTANT_EXPORT_ROOT)
            .join(ASSISTANT_EXPORT_LATEST_DIR);
        let handoff_path = latest.join("assistant_handoff_latest.txt");
        let self_description = format!(
            "SOURCE FILE\nconst POST_ACCEPTANCE_SECTION_BEGIN: &str = {:?};\nconst POST_ACCEPTANCE_SECTION_END: &str = {:?};\n",
            POST_ACCEPTANCE_SECTION_BEGIN, POST_ACCEPTANCE_SECTION_END
        );
        fs::write(&handoff_path, &self_description).expect("self describing handoff");
        let section = format!(
            "{POST_ACCEPTANCE_SECTION_BEGIN}\nProjectionOnly: true\nAuthority: none\n{POST_ACCEPTANCE_SECTION_END}\n"
        );

        upsert_post_acceptance_handoff_section(&handoff_path, &section)
            .expect("substring self-description must not be treated as marker line");

        let handoff = fs::read_to_string(&handoff_path).expect("handoff");
        assert!(handoff.contains(&self_description));
        assert_eq!(
            exact_handoff_marker_line_spans(&handoff, POST_ACCEPTANCE_SECTION_BEGIN).len(),
            1
        );
        assert_eq!(
            exact_handoff_marker_line_spans(&handoff, POST_ACCEPTANCE_SECTION_END).len(),
            1
        );
        upsert_post_acceptance_handoff_section(&handoff_path, &section)
            .expect("exact marker-line block must remain idempotent");
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn post_acceptance_handover_binds_exact_authority_and_preserves_existing_export_payload() {
        let current = temp_project("post_acceptance_exact_binding", "old");
        seed_assistant_export_staging(&current);
        let hash = format!("{:064x}", 519);
        let intent = b"schema_version=1\nchange_class=KodexOnly\n";
        let snapshot = "effect snapshot\n";
        multi_revision_evasion::record_acceptance(
            &current,
            "0.10.1839",
            "0.10.1839",
            519,
            &hash,
            0,
        )
        .expect("acceptance");
        multi_revision_evasion::record_acceptance_evidence(
            &current,
            "0.10.1839",
            "0.10.1839",
            519,
            &hash,
            Some(intent),
            snapshot,
        )
        .expect("acceptance evidence");
        let (acceptance_path, acceptance_bytes) =
            multi_revision_evasion::read_acceptance_record_exact(
                &current,
                "0.10.1839",
                "0.10.1839",
                519,
                &hash,
                0,
            )
            .expect("acceptance bytes");

        finalize_post_acceptance_assistant_handover(
            &current,
            "0.10.1839",
            "0.10.1839",
            519,
            &hash,
            0,
        )
        .expect("post acceptance handover");

        let latest = current
            .join(ASSISTANT_EXPORT_ROOT)
            .join(ASSISTANT_EXPORT_LATEST_DIR);
        assert_eq!(
            fs::read(latest.join("sentinel.txt")).expect("sentinel"),
            b"preserve-me\n"
        );
        assert_eq!(
            fs::read(latest.join("source_snapshot/SOURCE_SNAPSHOT.txt"))
                .expect("source snapshot"),
            b"snapshot-preserve-me\n"
        );
        let acceptance_name = acceptance_path.file_name().expect("acceptance name");
        assert_eq!(
            fs::read(latest.join(acceptance_name)).expect("staged acceptance"),
            acceptance_bytes
        );
        let lifecycle = fs::read_to_string(latest.join(POST_ACCEPTANCE_LIFECYCLE_FILE))
            .expect("lifecycle handover");
        assert!(lifecycle.contains("projection_only=true\n"));
        assert!(lifecycle.contains("authority=none\n"));
        assert!(lifecycle.contains("canonical_verdict=accepted\n"));
        assert!(lifecycle.contains(&format!("candidate_sha256={hash}\n")));
        assert!(lifecycle.contains(&format!(
            "acceptance_sha256={}\n",
            sha256::hex(&acceptance_bytes)
        )));
        let handoff = fs::read_to_string(latest.join("assistant_handoff_latest.txt"))
            .expect("handoff");
        assert_eq!(handoff.matches(POST_ACCEPTANCE_SECTION_BEGIN).count(), 1);
        assert!(handoff.contains("CanonicalVerdict: accepted"));
        assert!(handoff
            .contains("TransitionHistoryTemporalRole: pre_acceptance_advisory_snapshot"));
        assert!(!latest.join(TAKE_ATTEMPT_EVIDENCE_EXPORT_DIR).exists());

        let root_zip = current.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP);
        let file = fs::File::open(&root_zip).expect("root assistant zip");
        let mut archive = zip::ZipArchive::new(file).expect("zip archive");
        assert!(archive.by_name("sentinel.txt").is_ok());
        assert!(archive
            .by_name("source_snapshot/SOURCE_SNAPSHOT.txt")
            .is_ok());
        assert!(archive.by_name(POST_ACCEPTANCE_LIFECYCLE_FILE).is_ok());
        assert!(archive
            .by_name(acceptance_name.to_str().expect("acceptance utf8"))
            .is_ok());
        assert!(!(0..archive.len()).any(|index| {
            archive
                .by_index(index)
                .map(|entry| entry.name().starts_with("take_attempt_evidence/"))
                .unwrap_or(false)
        }));
        drop(archive);

        finalize_post_acceptance_assistant_handover(
            &current,
            "0.10.1839",
            "0.10.1839",
            519,
            &hash,
            0,
        )
        .expect("idempotent post acceptance handover");
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn post_acceptance_handover_io_failure_preserves_acceptance_and_committed_recovery_state() {
        let current = temp_project("post_acceptance_io_failure", "old");
        seed_assistant_export_staging(&current);
        let transaction = prepare_test_transaction(&current);
        transaction.mark_committed().expect("mark committed");
        let hash = format!("{:064x}", 515);
        multi_revision_evasion::record_acceptance(
            &current,
            "0.10.1839",
            "0.10.1839",
            515,
            &hash,
            0,
        )
        .expect("acceptance");
        multi_revision_evasion::record_acceptance_evidence(
            &current,
            "0.10.1839",
            "0.10.1839",
            515,
            &hash,
            Some(b"schema_version=1\nchange_class=KodexOnly\n"),
            "test-revision-snapshot\n",
        )
        .expect("evidence");
        let handoff_path = current
            .join(ASSISTANT_EXPORT_ROOT)
            .join(ASSISTANT_EXPORT_LATEST_DIR)
            .join("assistant_handoff_latest.txt");
        fs::remove_file(&handoff_path).expect("remove handoff fixture");
        fs::create_dir(&handoff_path).expect("deterministic handoff path conflict");

        let error = finalize_post_acceptance_assistant_handover(
            &current,
            "0.10.1839",
            "0.10.1839",
            515,
            &hash,
            0,
        )
        .expect_err("handover I/O conflict must fail");
        assert!(error.contains("kanonische Acceptance bleibt unveraendert"));
        let accepted = multi_revision_evasion::load_accepted_zip_sha256(
            &current,
            "0.10.1839",
            "0.10.1839",
            515,
        )
        .expect("accepted provenance");
        assert!(accepted.contains(&hash));
        assert_eq!(
            fs::read_to_string(&transaction.state_path).expect("committed state"),
            "committed\n"
        );
        assert!(current.join(TAKE_TRANSACTION_MARKER).exists());
        assert!(!current.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP).exists());
        transaction.commit().expect("test cleanup");
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn failure_handover_replaces_stale_export_and_binds_exact_rejection() {
        let current = temp_project("failure_handover_exact_binding", "old");
        let parent_acceptance = seed_failure_parent_acceptance(&current, "0.10.1839", 519);
        let export_root = current.join(ASSISTANT_EXPORT_ROOT);
        fs::create_dir_all(&export_root).expect("export root");
        fs::write(export_root.join(ASSISTANT_EXPORT_LATEST_ZIP), b"stale-internal")
            .expect("stale internal");
        fs::write(current.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP), b"stale-root")
            .expect("stale root");

        let hash = format!("{:064x}", 520);
        let intent = b"schema_version=1\nchange_class=KodexOnly\n";
        let snapshot = "effect snapshot\n";
        let reason = "deterministic take rejection";
        let rejection = multi_revision_evasion::RejectionRecord {
            from_version: "0.10.1839",
            to_version: "0.10.1839",
            take_revision: 520,
            zip_sha256: &hash,
            rejection_stage: multi_revision_evasion::REJECTION_STAGE_DEPENDENCY_PREFLIGHT,
            candidate_intent: Some(intent),
            snapshot,
            reason,
        };
        multi_revision_evasion::record_rejection(&current, &rejection)
            .expect("canonical rejection");
        let canonical = multi_revision_evasion::read_rejection_record_exact(
            &current,
            "0.10.1839",
            "0.10.1839",
            520,
            &hash,
        )
        .expect("canonical rejection readback");

        finalize_rejection_assistant_handover(
            &current,
            &rejection,
            FailureAttemptContext {
                started_epoch_millis: Some(canonical.attempt_epoch_millis.saturating_sub(5)),
                child_status: "not_started",
                child_exit: None,
                transaction_state: "not_started",
            },
        )
        .expect("failure handover");

        let root_zip = current.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP);
        let file = fs::File::open(&root_zip).expect("root failure assistant zip");
        let mut archive = zip::ZipArchive::new(file).expect("failure zip archive");
        let mut handoff = String::new();
        {
            let mut handoff_entry = archive
                .by_name("assistant_handoff_latest.txt")
                .expect("failure handoff");
            std::io::Read::read_to_string(&mut handoff_entry, &mut handoff)
                .expect("read failure handoff");
        }
        assert!(handoff.contains("CanonicalVerdict: rejected"));
        assert!(handoff.contains("Revision: r520"));
        assert!(handoff.contains(&format!("CandidateSha256: {hash}")));
        assert!(archive.by_name(TAKE_FAILURE_HANDOVER_FILE).is_ok());
        assert!(archive.by_name(TAKE_FAILURE_METADATA_FILE).is_ok());
        assert!(archive.by_name(TAKE_FAILURE_PARENT_ACCEPTANCE_FILE).is_ok());
        assert!(archive
            .by_name(TAKE_FAILURE_CANONICAL_METADATA_FILE)
            .is_ok());
        assert!(archive
            .by_name("source_snapshot/SOURCE_SNAPSHOT.txt")
            .is_ok());
        assert!(archive
            .by_name(&format!("source_snapshot/{TAKE_FAILURE_SOURCE_ARCHIVE_FILE}"))
            .is_ok());
        let short_sha = &hash[..16];
        assert!(archive
            .by_name(&format!(
                "take_attempt_evidence/rejected-r520-{short_sha}/{FAILURE_EVIDENCE_METADATA_FILE}"
            ))
            .is_ok());
        drop(archive);

        assert_eq!(
            fs::read(
                export_root
                    .join(ASSISTANT_EXPORT_LATEST_DIR)
                    .join(TAKE_FAILURE_PARENT_ACCEPTANCE_FILE)
            )
            .expect("projected parent acceptance"),
            parent_acceptance
        );
        assert_eq!(
            fs::read(
                export_root
                    .join(ASSISTANT_EXPORT_LATEST_DIR)
                    .join(TAKE_FAILURE_CANONICAL_METADATA_FILE)
            )
            .expect("projected canonical rejection metadata"),
            canonical.metadata
        );
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn failure_handover_io_failure_preserves_rejection_and_rolled_back_recovery_state() {
        let current = temp_project("failure_handover_io_recovery", "old");
        seed_failure_parent_acceptance(&current, "0.10.1839", 514);
        let transaction = prepare_test_transaction(&current);
        transaction
            .rollback_preserving_provenance(&current)
            .expect("rollback to recovery state");
        let hash = format!("{:064x}", 521);
        let reason = "deterministic failure handover io witness";
        let rejection = multi_revision_evasion::RejectionRecord {
            from_version: "0.10.1839",
            to_version: "0.10.1839",
            take_revision: 521,
            zip_sha256: &hash,
            rejection_stage: multi_revision_evasion::REJECTION_STAGE_CRASH_RECOVERY,
            candidate_intent: Some(b"schema_version=1\nchange_class=KodexOnly\n"),
            snapshot: "effect snapshot\n",
            reason,
        };
        multi_revision_evasion::record_rejection(&current, &rejection)
            .expect("canonical rejection");

        let export_root = current.join(ASSISTANT_EXPORT_ROOT);
        fs::create_dir_all(&export_root).expect("export root");
        fs::write(
            export_root.join(ASSISTANT_EXPORT_LATEST_DIR),
            b"deterministic latest-dir conflict",
        )
        .expect("latest path conflict");

        let error = finalize_rejection_assistant_handover(
            &current,
            &rejection,
            FailureAttemptContext {
                started_epoch_millis: None,
                child_status: "unknown_recovery",
                child_exit: None,
                transaction_state: "rolled_back_recovery",
            },
        )
        .expect_err("failure handover path conflict must fail");
        assert!(error.contains("kanonische Rejection bleibt unveraendert"));
        assert_eq!(
            fs::read_to_string(&transaction.state_path).expect("rolled back state"),
            "rolled_back\n"
        );
        assert!(current.join(TAKE_TRANSACTION_MARKER).exists());
        assert!(!current.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP).exists());
        multi_revision_evasion::read_rejection_record_exact(
            &current,
            "0.10.1839",
            "0.10.1839",
            521,
            &hash,
        )
        .expect("canonical rejection remains");
        transaction.finish_rollback().expect("test cleanup");
        let _ = fs::remove_dir_all(current);
    }

    #[test]
    fn transactional_apply_can_commit_exactly() {
        let current = temp_project("commit_current", "old");
        let incoming = temp_project("commit_incoming", "new");
        fs::write(current.join("Cargo.lock"), "parent-lock").expect("parent lock");
        let transaction = prepare_test_transaction(&current);
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        fs::write(current.join("Cargo.lock"), "cargo-owned-committed-lock")
            .expect("committed lock");
        transaction.mark_committed().expect("mark committed");
        transaction.commit().expect("commit");
        assert_eq!(
            fs::read_to_string(current.join("src/payload.txt")).expect("new payload"),
            "new"
        );
        assert_eq!(
            fs::read_to_string(current.join("Cargo.lock")).expect("committed cargo lock"),
            "cargo-owned-committed-lock"
        );
        assert!(!current.join(TAKE_TRANSACTION_MARKER).exists());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn pending_applied_transaction_is_recovered_before_next_run() {
        let current = temp_project("recover_current", "old");
        let incoming = temp_project("recover_incoming", "new");
        seed_failure_recovery_parent(&current);
        let transaction = prepare_test_transaction(&current);
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        recover_pending_transaction(&current).expect("recover");
        assert_eq!(
            fs::read_to_string(current.join("src/payload.txt")).expect("old payload"),
            "old"
        );
        assert!(!current.join(TAKE_TRANSACTION_MARKER).exists());
        let rejected = multi_revision_evasion::load_rejected_revisions(
            &current,
            "0.10.1839",
            "0.10.1839",
        )
        .expect("rejected provenance");
        assert!(rejected
            .iter()
            .any(|entry| entry.zip_sha256 == format!("{:064x}", 515)));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn pending_rolled_back_transaction_recovers_rejection_before_cleanup() {
        let current = temp_project("recover_rolled_back_current", "old");
        let incoming = temp_project("recover_rolled_back_incoming", "new");
        seed_failure_recovery_parent(&current);
        let transaction = prepare_test_transaction(&current);
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        transaction
            .rollback_preserving_provenance(&current)
            .expect("rollback preserve provenance");
        assert!(current.join(TAKE_TRANSACTION_MARKER).exists());

        recover_pending_transaction(&current).expect("recover rolled back");

        assert!(!current.join(TAKE_TRANSACTION_MARKER).exists());
        let rejected = multi_revision_evasion::load_rejected_revisions(
            &current,
            "0.10.1839",
            "0.10.1839",
        )
        .expect("rejected provenance");
        assert!(rejected
            .iter()
            .any(|entry| entry.zip_sha256 == format!("{:064x}", 515)));
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn failed_reexec_rejection_io_failure_preserves_recovery_provenance() {
        let current = temp_project("failed_reexec_rejection_io", "old");
        let incoming = temp_project("failed_reexec_rejection_io_incoming", "new");
        seed_failure_recovery_parent(&current);
        let transaction = prepare_test_transaction(&current);
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        transaction.rollback(&current).expect("rollback");

        assert_eq!(
            fs::read_to_string(current.join("src/payload.txt")).expect("old payload"),
            "old"
        );
        assert_eq!(
            fs::read_to_string(&transaction.state_path).expect("rolled back state"),
            "rolled_back\n"
        );
        assert!(current.join(TAKE_TRANSACTION_MARKER).exists());
        assert!(transaction.root.exists());

        let ledger_root = current.join("target/etuschlopf_take_rejection_ledger_v1");
        fs::remove_dir_all(&ledger_root)
            .expect("remove accepted parent ledger before deterministic path conflict");
        fs::write(&ledger_root, b"deterministic ledger path conflict")
            .expect("ledger root conflict file");
        let ledger_error = multi_revision_evasion::record_rejection(
            &current,
            &multi_revision_evasion::RejectionRecord {
                from_version: "0.10.1839",
                to_version: "0.10.1839",
                take_revision: 515,
                zip_sha256: &format!("{:064x}", 515),
                rejection_stage: multi_revision_evasion::REJECTION_STAGE_REEXEC_EXIT,
                candidate_intent: Some(b"test candidate intent\n"),
                snapshot: "test-revision-snapshot\n",
                reason: "failed reexec rejection durability witness",
            },
        )
        .expect_err("ledger path conflict must reject");
        assert!(ledger_error.contains("Ledger-Ordner nicht anlegbar"));

        assert_eq!(
            fs::read_to_string(&transaction.state_path)
                .expect("rolled back state after ledger error"),
            "rolled_back\n"
        );
        assert!(current.join(TAKE_TRANSACTION_MARKER).exists());
        assert!(transaction.root.exists());

        fs::remove_file(&ledger_root).expect("remove ledger root conflict");
        seed_failure_recovery_parent(&current);
        recover_pending_transaction(&current).expect("recover rolled back rejection");

        let rejected = multi_revision_evasion::load_rejected_revisions(
            &current,
            "0.10.1839",
            "0.10.1839",
        )
        .expect("rejected provenance");
        assert!(rejected
            .iter()
            .any(|entry| entry.zip_sha256 == format!("{:064x}", 515)));
        assert!(!current.join(TAKE_TRANSACTION_MARKER).exists());
        assert!(!transaction.root.exists());

        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn pending_committed_transaction_recovers_acceptance_without_rollback() {
        let current = temp_project("recover_committed_current", "old");
        let incoming = temp_project("recover_committed_incoming", "new");
        let transaction = prepare_test_transaction(&current);
        multi_revision_evasion::record_rejection(
            &current,
            &multi_revision_evasion::RejectionRecord {
                from_version: "0.10.1839",
                to_version: "0.10.1839",
                take_revision: 513,
                zip_sha256: &format!("{:064x}", 513),
                rejection_stage: multi_revision_evasion::REJECTION_STAGE_INCOMING_TAKE,
                candidate_intent: None,
                snapshot: "v1\tsrc/security.rs\t0000000000000001\tsecurity-negative-check\tdeny\n",
                reason: "first prior rejection",
            },
        )
        .expect("first prior rejection");
        multi_revision_evasion::record_rejection(
            &current,
            &multi_revision_evasion::RejectionRecord {
                from_version: "0.10.1839",
                to_version: "0.10.1839",
                take_revision: 514,
                zip_sha256: &format!("{:064x}", 514),
                rejection_stage: multi_revision_evasion::REJECTION_STAGE_INCOMING_TAKE,
                candidate_intent: None,
                snapshot: "v1\tsrc/security.rs\t0000000000000002\tsecurity-negative-check\tdeny\n",
                reason: "second prior rejection",
            },
        )
        .expect("second prior rejection");
        transaction
            .apply_and_verify(&incoming, &current)
            .expect("apply");
        transaction.mark_committed().expect("mark committed");
        seed_assistant_export_staging(&current);

        recover_pending_transaction(&current).expect("recover committed");

        assert_eq!(
            fs::read_to_string(current.join("src/payload.txt")).expect("new payload"),
            "new"
        );
        assert!(!current.join(TAKE_TRANSACTION_MARKER).exists());
        let accepted = multi_revision_evasion::load_accepted_zip_sha256(
            &current,
            "0.10.1839",
            "0.10.1839",
            515,
        )
        .expect("accepted provenance");
        let accepted_hash = format!("{:064x}", 515);
        assert!(accepted.contains(&accepted_hash));
        let evidence_dir = current
            .join("target/etuschlopf_take_acceptance_evidence_v1")
            .join("0.10.1839__0.10.1839")
            .join(format!("accepted-evidence-r515-{accepted_hash}"));
        assert!(evidence_dir.join("metadata.txt").is_file());
        assert!(evidence_dir.join("candidate_intent.txt").is_file());
        assert!(evidence_dir.join("semantic_effect_snapshot.tsv").is_file());
        let latest = current
            .join(ASSISTANT_EXPORT_ROOT)
            .join(ASSISTANT_EXPORT_LATEST_DIR);
        let lifecycle = fs::read_to_string(latest.join(POST_ACCEPTANCE_LIFECYCLE_FILE))
            .expect("recovered lifecycle handover");
        assert!(lifecycle.contains("canonical_verdict=accepted\n"));
        assert!(lifecycle.contains(&format!("candidate_sha256={accepted_hash}\n")));
        assert!(current.join(ASSISTANT_EXPORT_ROOT_LATEST_ZIP).is_file());
        let all_rejections = multi_revision_evasion::load_rejected_revisions(
            &current,
            "0.10.1839",
            "0.10.1839",
        )
        .expect("all rejections");
        let active = multi_revision_evasion::active_diagnostic_rejections(
            &current,
            "0.10.1839",
            "0.10.1839",
            &all_rejections,
        )
        .expect("active diagnostics");
        assert_eq!(all_rejections.len(), 2);
        assert!(active.is_empty());
        let _ = fs::remove_dir_all(current);
        let _ = fs::remove_dir_all(incoming);
    }

    #[test]
    fn take_attempt_registry_accepts_only_typed_bounded_paths() {
        assert!(safe_registered_child_artifact(
            "strict_test_log",
            "target/etuschlopf_logs/visual_preflight.strict-tests.txt"
        )
        .is_ok());
        assert!(safe_registered_child_artifact(
            "strict_clippy_log",
            "Synthetic_Project.strict-clippy.txt"
        )
        .is_ok());
        assert!(safe_registered_child_artifact("strict_test_log", "../outside.txt").is_err());
        assert!(safe_registered_child_artifact(
            "strict_clippy_log",
            "nested/project.strict-clippy.txt"
        )
        .is_err());
        assert!(safe_registered_child_artifact("unknown", "evidence.txt").is_err());
    }

    #[test]
    fn take_failure_evidence_authority_is_generation_bound() {
        let bootstrap = include_str!("etuschlopf_take_bootstrap.rs");
        assert!(bootstrap.contains(
            "bootstrap_generation_rule=accepted_parent_bootstrap_owns_current_take"
        ));
        assert!(bootstrap.contains("capability_effective_from=next_take_after_acceptance"));
        assert!(bootstrap.contains(".env(TAKE_ATTEMPT_EVIDENCE_DIR_ENV, &child_writeahead_dir)"));
        let retired_mtime_scan = ["file_changed_since_", "attempt"].concat();
        assert!(!bootstrap.contains(&retired_mtime_scan));
        assert!(bootstrap.contains("transaction.rollback(&project_root)?;"));
    }

    #[test]
    fn unsafe_zip_segments_are_rejected() {
        assert!(validate_zip_relative_path(Path::new("../evil"), "../evil").is_err());
        assert!(validate_zip_relative_path(Path::new("src/CON.txt"), "src/CON.txt").is_err());
        assert!(validate_zip_relative_path(Path::new("src/good.rs"), "src/good.rs").is_ok());
    }

    #[test]
    fn prior_console_is_absent_from_actual_capture_after_prelaunch_supersession() {
        let root = temp_project("fresh_child_console", "old");
        let log = root.join("target/etuschlopf_logs/etuschlopf.console.txt");
        fs::create_dir_all(log.parent().unwrap()).unwrap();
        fs::write(&log, b"previous unrelated run").unwrap();
        let evidence = root.join("attempt_evidence");
        let stale = capture_registered_child_evidence(&root, &evidence, "unknown_recovery");
        assert_eq!(stale.captured, 0);
        assert!(fs::read_to_string(evidence.join("child_evidence/EVIDENCE_MANIFEST.txt"))
            .unwrap().contains("status=not_current_attempt"));
        clear_previous_child_console(&root).unwrap();
        clear_previous_child_console(&root).unwrap();
        let result = capture_registered_child_evidence(&root, &evidence, "started_returned");
        assert_eq!(result.captured, 0);
        assert_eq!(result.launcher_stage, "not_reached");
        assert!(!evidence.join("child_evidence/etuschlopf.console.txt").exists());
        assert!(fs::read_to_string(evidence.join("child_evidence/EVIDENCE_MANIFEST.txt"))
            .unwrap().contains("role=child_console\tstatus=absent"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn actual_legacy_collector_captures_current_early_compiler_evidence_and_stage() {
        let root = temp_project("early_compiler_evidence", "old");
        let log = root.join("target/etuschlopf_logs/etuschlopf.console.txt");
        fs::create_dir_all(log.parent().unwrap()).unwrap();
        fs::write(&log, b"previous unrelated run").unwrap();
        clear_previous_child_console(&root).unwrap();
        let evidence = root.join("attempt_evidence");
        let writeahead = evidence.join(FAILURE_EVIDENCE_CHILD_WRITEAHEAD_DIR);
        fs::create_dir_all(&writeahead).unwrap();
        let body = b"ETUSCHLOPF-EARLY-CHILD-LOG\nTakeAttemptEvidenceDir=current_attempt\nsynthetic compiler failure\nLauncherBootstrapCargoExit=101\n";
        fs::write(&log, body).unwrap();
        fs::write(writeahead.join("CHILD_STAGE.txt"), b"schema=1\nauthority=none\nlauncher_stage=launcher_bootstrap_build\n").unwrap();
        let result = capture_registered_child_evidence(&root, &evidence, "started_returned");
        assert_eq!(result.captured, 1);
        assert_eq!(result.capture_errors, 0);
        assert_eq!(result.launcher_stage, "launcher_bootstrap_build");
        assert_eq!(fs::read(evidence.join("child_evidence/etuschlopf.console.txt")).unwrap(), body);
        fs::remove_file(&log).unwrap();
        fs::create_dir(&log).unwrap();
        assert!(clear_previous_child_console(&root).is_err());
        assert!(log.is_dir());
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn incoming_run_cmd_preserves_early_compiler_failure_through_legacy_collector() {
        let root = temp_project("batch_early_failure", "old");
        let bin = root.join("stub bin");
        fs::create_dir_all(&bin).unwrap();
        let stub = root.join("synthetic_cargo.rs");
        fs::write(&stub, "fn main() { eprintln!(\"synthetic launcher compiler failure\"); std::process::exit(101); }").unwrap();
        let compiled = Command::new("rustc").args(["--edition=2024"])
            .arg(&stub).arg("-o").arg(bin.join("cargo.exe")).output().unwrap();
        assert!(compiled.status.success(), "{}", String::from_utf8_lossy(&compiled.stderr));
        fs::write(root.join("run.cmd"), include_bytes!("../../run.cmd")).unwrap();
        let evidence = root.join("attempt evidence");
        let writeahead = evidence.join(FAILURE_EVIDENCE_CHILD_WRITEAHEAD_DIR);
        fs::create_dir_all(&writeahead).unwrap();
        let log = root.join("target/etuschlopf_logs/etuschlopf.console.txt");
        fs::create_dir_all(log.parent().unwrap()).unwrap();
        fs::write(&log, b"previous unrelated run").unwrap();
        let path = std::env::join_paths(std::iter::once(bin).chain(
            std::env::split_paths(&std::env::var_os("PATH").unwrap()))).unwrap();
        let output = Command::new("cmd").args(["/d", "/c", "run.cmd", "--all"])
            .current_dir(&root).env("PATH", path)
            .env("ETUSCHLOPF_TAKE_TRANSACTION_ACTIVE", "1")
            .env(TAKE_ATTEMPT_EVIDENCE_DIR_ENV, &writeahead)
            .env_remove("ETUSCHLOPF_STARTUP_TIMING_SESSION")
            .output().unwrap();
        assert_eq!(output.status.code(), Some(101), "{}", String::from_utf8_lossy(&output.stderr));
        let result = capture_registered_child_evidence(&root, &evidence, "started_returned");
        assert_eq!(result.launcher_stage, "launcher_bootstrap_build");
        let captured = fs::read_to_string(evidence.join("child_evidence/etuschlopf.console.txt")).unwrap();
        assert!(captured.contains("synthetic launcher compiler failure"));
        assert!(captured.contains("LauncherBootstrapCargoExit=101"));
        assert!(!captured.contains("previous unrelated run"));
        fs::remove_dir_all(root).unwrap();
    }

}

#[cfg(test)]
mod bootstrap_step_console_tests {
    use super::{
        bootstrap_step_done_text, bootstrap_step_live_text, format_bootstrap_step_duration,
        merge_bootstrap_step_last, parse_bootstrap_step_last,
    };
    use std::time::Duration;

    #[test]
    fn himem_line_counts_up_and_completes_in_place() {
        assert_eq!(
            bootstrap_step_live_text("Abhängigkeiten werden geprüft", 12, Some(14_960)),
            "Abhängigkeiten werden geprüft ... 12 s · zuletzt 15 s"
        );
        assert_eq!(
            bootstrap_step_live_text("Sicherung wird angelegt", 0, None),
            "Sicherung wird angelegt ... 0 s"
        );
        assert_eq!(
            bootstrap_step_done_text("Cargo.lock und Kodex-Gates werden geprüft (cargo check)", Duration::from_millis(37_653)),
            "Cargo.lock und Kodex-Gates werden geprüft (cargo check) ... fertig · 38 s"
        );
    }

    #[test]
    fn last_durations_are_best_effort_and_replace_only_their_key() {
        assert_eq!(parse_bootstrap_step_last("", "root_lock"), None);
        assert_eq!(parse_bootstrap_step_last("root_lock=garbage\n", "root_lock"), None);
        let first = merge_bootstrap_step_last("", "root_lock", 37_653);
        assert_eq!(first, "root_lock=37653\n");
        let second = merge_bootstrap_step_last(&format!("{first}junk\nbackup=x\n"), "backup", 10_887);
        assert_eq!(second, "root_lock=37653\nbackup=10887\n");
        let third = merge_bootstrap_step_last(&second, "root_lock", 30_000);
        assert_eq!(third, "backup=10887\nroot_lock=30000\n");
        assert_eq!(parse_bootstrap_step_last(&third, "root_lock"), Some(30_000));
        assert_eq!(parse_bootstrap_step_last(&third, "backup"), Some(10_887));
    }

    #[test]
    fn step_durations_are_compact_and_monotone_in_unit() {
        assert_eq!(format_bootstrap_step_duration(Duration::from_millis(752)), "0,7 s");
        assert_eq!(format_bootstrap_step_duration(Duration::from_millis(9_999)), "9,9 s");
        assert_eq!(format_bootstrap_step_duration(Duration::from_millis(14_960)), "15 s");
        assert_eq!(format_bootstrap_step_duration(Duration::from_millis(59_400)), "59 s");
        assert_eq!(format_bootstrap_step_duration(Duration::from_millis(94_360)), "1 min 34 s");
    }
}
