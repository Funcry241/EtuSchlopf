// Observations only: timers never decide validation, reuse, release or process arguments.
static LAUNCHER_TIMING_EPOCH: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

struct LauncherPhaseTimer {
    phase: &'static str,
    started: Instant,
}

impl LauncherPhaseTimer {
    fn new(phase: &'static str) -> Self {
        LAUNCHER_TIMING_EPOCH.get_or_init(Instant::now);
        Self { phase, started: Instant::now() }
    }
}

impl Drop for LauncherPhaseTimer {
    fn drop(&mut self) {
        let ended = Instant::now();
        append_launcher_timing_line(&format!(
            "LauncherPhaseTiming: Phase={}; StartUs={}; EndUs={}; WallUs={}; Pid={}",
            self.phase, launcher_timing_offset(self.started), launcher_timing_offset(ended),
            ended.duration_since(self.started).as_micros(), std::process::id(),
        ));
    }
}

fn launcher_timing_offset(instant: Instant) -> u128 {
    instant.saturating_duration_since(*LAUNCHER_TIMING_EPOCH.get_or_init(Instant::now)).as_micros()
}

fn append_launcher_timing_line(line: &str) {
    // Test binaries share the project cwd with their parent launcher. Their
    // signature-fixture timings must never append to the parent's real clock.
    #[cfg(test)]
    let _ = line;
    // Diagnostic I/O failure cannot change an otherwise successful validation/run.
    #[cfg(not(test))]
    if let Ok(mut file) = OpenOptions::new().append(true).open(CONSOLE_LOG) {
        let _ = writeln!(file, "{line}");
    }
}

struct LauncherProcessTiming<'a> {
    phase: &'a str,
    started: Instant,
    observed_end: Instant,
    spawn_us: u128,
    exit_lower_us: u128,
    exit_upper_us: u128,
    completion_ui_us: u128,
    exit_code: i32,
}

fn launcher_process_timing_line(timing: LauncherProcessTiming<'_>) -> String {
    let LauncherProcessTiming { phase, started, observed_end, spawn_us, exit_lower_us, exit_upper_us, completion_ui_us, exit_code } = timing;
    format!(
        "LauncherProcessTiming: Phase={phase}; StartUs={}; EndUs={}; WallUs={}; SpawnUs={spawn_us}; ExitAfterSpawnCallLowerUs={exit_lower_us}; ExitAfterSpawnCallUpperUs={exit_upper_us}; ExitUncertaintyUs={}; CompletionUiUs={completion_ui_us}; ExitCode={exit_code}; Pid={}",
        launcher_timing_offset(started), launcher_timing_offset(observed_end),
        observed_end.duration_since(started).as_micros(), exit_upper_us.saturating_sub(exit_lower_us),
        std::process::id(),
    )
}

#[derive(Debug)]
struct LauncherTimingInterval<'a> {
    line: &'a str,
    start: u128,
    end: u128,
}

fn launcher_timing_interval(line: &str) -> Option<LauncherTimingInterval<'_>> {
    let body = line.strip_prefix("LauncherPhaseTiming: ")
        .or_else(|| line.strip_prefix("LauncherProcessTiming: "))?;
    let field = |name: &str| body.split(';').find_map(|part| {
        let (key, value) = part.trim().split_once('=')?;
        (key == name).then_some(value)
    });
    let start = field("StartUs")?.parse().ok()?;
    let end = field("EndUs")?.parse().ok()?;
    (end >= start).then_some(LauncherTimingInterval { line, start, end })
}

fn launcher_timing_exclusive_us(outer: &LauncherTimingInterval<'_>, all: &[LauncherTimingInterval<'_>]) -> u128 {
    let mut children = all.iter().filter(|child| {
        outer.start <= child.start && child.end <= outer.end
            && (outer.start < child.start || child.end < outer.end)
    }).map(|child| (child.start, child.end)).collect::<Vec<_>>();
    children.sort_unstable();
    let mut covered = 0;
    let mut cursor = outer.start;
    for (start, end) in children {
        let uncovered_start = start.max(cursor);
        covered += end.saturating_sub(uncovered_start);
        cursor = cursor.max(end);
    }
    (outer.end - outer.start).saturating_sub(covered)
}

fn launcher_timing_readback(console: &str) -> String {
    let intervals = console.lines().filter_map(launcher_timing_interval).collect::<Vec<_>>();
    let mut out = console.split_once("[STARTUP-TIMING-READBACK]")
        .and_then(|(_, body)| body.split_once("[/STARTUP-TIMING-READBACK]"))
        .map(|(body, _)| format!("\n[STARTUP-TIMING-READBACK]{body}[/STARTUP-TIMING-READBACK]\n"))
        .unwrap_or_else(|| "\nStartupTimingStatus: not_captured_in_bound_console; no zero-duration inference\n".to_string());
    out.push_str("\n[LAUNCHER-TIMING-READBACK]\nProjectionOnly: true\nAuthority: none\nUnit: *Us=microseconds; elapsed wall time, not CPU time; existing AssistantExportTiming uses *Ms, KorpusExecutionTiming uses mm:ss\nClockScope: launcher run() entry; excludes run.cmd compilation and Take bootstrap/apply/reexec\nCaptureBoundary: before assistant handoff; later handoff/outer-ZIP/launcher-exit time is not present in this archive\nWallUs: inclusive; do not sum parent/child or parallel-prewarm durations\nExclusiveObservedUs: subtract the union of fully contained measured spans; includes uninstrumented work and completion/UI overhead; crossing/equal-bound spans have no inferred parentage\nProcessExit: OS completion lies between last running poll start and first completed poll return, measured from spawn-call start; uncertainty includes polling and scheduler delay\nPrewarm: LifetimeObservedUpperUs is an upper bound observed at join, not exact process runtime; only ResidualWaitUs adds serial waiting after BuildControl\n");
    if intervals.is_empty() {
        out.push_str("TimingStatus: not_captured; no zero-duration inference\n");
    }
    for interval in &intervals {
        out.push_str(&format!("{}; ExclusiveObservedUs={}\n", interval.line, launcher_timing_exclusive_us(interval, &intervals)));
    }
    for line in console.lines().filter(|line| {
        line.starts_with("LauncherPrewarmTiming:") || line.starts_with("AssistantExportTiming:")
            || line.starts_with("KorpusExecutionTiming:")
    }) {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("[/LAUNCHER-TIMING-READBACK]\n");
    out
}

#[cfg(test)]
mod launcher_timing_tests {
    use super::*;

    #[test]
    fn exclusive_time_subtracts_overlapping_children_once() {
        let console = "LauncherPhaseTiming: Phase=outer; StartUs=0; EndUs=100; WallUs=100\nLauncherPhaseTiming: Phase=a; StartUs=10; EndUs=60; WallUs=50\nLauncherProcessTiming: Phase=b; StartUs=40; EndUs=90; WallUs=50\nLauncherPhaseTiming: Phase=nested; StartUs=20; EndUs=30; WallUs=10";
        let out = launcher_timing_readback(console);
        assert!(out.contains("Phase=outer; StartUs=0; EndUs=100; WallUs=100; ExclusiveObservedUs=20"));
        assert!(out.contains("Phase=a; StartUs=10; EndUs=60; WallUs=50; ExclusiveObservedUs=40"));
    }

    #[test]
    fn parallel_prewarm_is_reported_without_additive_runtime_claim() {
        let console = "LauncherPhaseTiming: Phase=BuildControl; StartUs=10; EndUs=110; WallUs=100\nLauncherPrewarmTiming: LifetimeObservedUpperUs=140; ResidualWaitUs=30";
        let out = launcher_timing_readback(console);
        assert!(out.contains("WallUs=100; ExclusiveObservedUs=100"));
        assert!(out.contains("LifetimeObservedUpperUs=140; ResidualWaitUs=30"));
        assert!(out.contains("not exact process runtime"));
    }

    #[test]
    fn missing_or_malformed_measurements_are_not_zero_timings() {
        let out = launcher_timing_readback("LauncherPhaseTiming: Phase=broken; StartUs=20; EndUs=10");
        assert!(out.contains("TimingStatus: not_captured"));
        assert!(out.contains("excludes run.cmd compilation and Take bootstrap"));
        assert!(out.contains("later handoff/outer-ZIP/launcher-exit time is not present"));
    }

    #[test]
    fn process_exit_uncertainty_is_preserved_in_readback() {
        let clock = Instant::now();
        let line = launcher_process_timing_line(LauncherProcessTiming { phase: "child", started: clock, observed_end: clock + Duration::from_micros(300), spawn_us: 10, exit_lower_us: 100, exit_upper_us: 280, completion_ui_us: 20, exit_code: 0 });
        assert!(line.contains("WallUs=300; SpawnUs=10; ExitAfterSpawnCallLowerUs=100; ExitAfterSpawnCallUpperUs=280; ExitUncertaintyUs=180; CompletionUiUs=20; ExitCode=0"));
        assert!(launcher_timing_readback(&line).contains("ExitUncertaintyUs=180"));
    }
}
