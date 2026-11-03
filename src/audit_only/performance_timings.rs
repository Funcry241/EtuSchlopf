//! Monotonic, observation-only timings. No reader grants musical authority.
//! Worker calls accumulate locally; only their drop merges counters. One batch
//! line is emitted after its workers have joined, including error returns.
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<(String, &'static str)>> = const { RefCell::new(None) };
}

/// Observation labels only. This context never crosses worker threads and is
/// restored on every ordinary exit, including an existing error return.
pub(crate) struct ContextGuard { previous: Option<(String, &'static str)> }

pub(crate) fn enter_context(project: &str, scope: &'static str) -> ContextGuard {
    ContextGuard { previous: CONTEXT.with(|c| c.replace(Some((project.to_owned(), scope)))) }
}

impl Drop for ContextGuard {
    fn drop(&mut self) { CONTEXT.with(|c| { c.replace(self.previous.take()); }); }
}

struct ScopedRecord {
    project: String,
    scope: &'static str,
    component: &'static str,
    owner: String,
    started: Instant,
}

pub(crate) struct ScopedCallTiming { record: Option<ScopedRecord> }

impl ScopedCallTiming {
    pub(crate) fn current(component: &'static str, owner: &str) -> Self {
        Self { record: CONTEXT.with(|c| c.borrow().as_ref().map(|(project, scope)|
            ScopedRecord { project: project.clone(), scope, component,
                owner: owner.to_owned(), started: Instant::now() })) }
    }
}

impl Drop for ScopedCallTiming {
    fn drop(&mut self) {
        let _ = self.record.as_ref().map(|r| writeln!(std::io::stderr().lock(),
            "ETUSCHLOPF: DetailTiming: schema=1; project={:?}; scope={:?}; component={:?}; owner={:?}; wall_us={}; authority=none",
            r.project, r.scope, r.component, r.owner, micros(r.started)));
    }
}

fn micros(start: Instant) -> u64 {
    u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX)
}

pub(crate) struct BatchTimings<'a, const N: usize> {
    project: &'a str,
    batch: &'a str,
    sequence: usize,
    names: [&'static str; N],
    started: Instant,
    calls: AtomicU64,
    tail_calls: AtomicU64,
    total_us: AtomicU64,
    max_us: AtomicU64,
    phase_us: [AtomicU64; N],
    phase_max_us: [AtomicU64; N],
    phase_calls: [AtomicU64; N],
}

impl<'a, const N: usize> BatchTimings<'a, N> {
    pub(crate) fn new(project: &'a str, batch: &'a str, sequence: usize,
        names: [&'static str; N]) -> Self {
        Self { project, batch, sequence, names, started: Instant::now(),
            calls: AtomicU64::new(0), tail_calls: AtomicU64::new(0),
            total_us: AtomicU64::new(0), max_us: AtomicU64::new(0),
            phase_us: std::array::from_fn(|_| AtomicU64::new(0)),
            phase_max_us: std::array::from_fn(|_| AtomicU64::new(0)),
            phase_calls: std::array::from_fn(|_| AtomicU64::new(0)) }
    }

    pub(crate) fn call(&self) -> CallTiming<'_, 'a, N> {
        let now = Instant::now();
        let mut visited = [false; N];
        visited[0] = true;
        CallTiming { batch: self, started: now, phase_started: now,
            phase: 0, phase_us: [0; N], visited, reached_tail: false }
    }
}

impl<const N: usize> Drop for BatchTimings<'_, N> {
    fn drop(&mut self) {
        let sums = std::array::from_fn::<_, N, _>(|i| self.phase_us[i].load(Ordering::Relaxed));
        let maxima = std::array::from_fn::<_, N, _>(|i| self.phase_max_us[i].load(Ordering::Relaxed));
        let counts = std::array::from_fn::<_, N, _>(|i| self.phase_calls[i].load(Ordering::Relaxed));
        let _ = writeln!(std::io::stderr().lock(),
            "ETUSCHLOPF: CandidateTiming: schema=1; project={:?}; batch={:?}; sequence={}; calls={}; tail_calls={}; batch_wall_us={}; call_sum_us={}; call_max_us={}; phases={:?}; phase_calls={counts:?}; phase_sum_us={sums:?}; phase_max_us={maxima:?}; interpretation=sum_of_call_wall_not_cpu; authority=none",
            self.project, self.batch, self.sequence, self.calls.load(Ordering::Relaxed),
            self.tail_calls.load(Ordering::Relaxed), micros(self.started),
            self.total_us.load(Ordering::Relaxed), self.max_us.load(Ordering::Relaxed), self.names);
    }
}

pub(crate) struct CallTiming<'a, 'b, const N: usize> {
    batch: &'a BatchTimings<'b, N>,
    started: Instant,
    phase_started: Instant,
    phase: usize,
    phase_us: [u64; N],
    visited: [bool; N],
    reached_tail: bool,
}

impl<const N: usize> CallTiming<'_, '_, N> {
    pub(crate) fn enter(&mut self, phase: usize) {
        self.phase_us[self.phase] = self.phase_us[self.phase].saturating_add(micros(self.phase_started));
        self.phase = phase;
        self.visited[phase] = true;
        self.phase_started = Instant::now();
    }

    /// Reaching the normal tail is not a claim about the musical verdict.
    /// A false value can mean an error or a successful early-return path.
    pub(crate) fn reached_tail(&mut self) { self.reached_tail = true; }
}

impl<const N: usize> Drop for CallTiming<'_, '_, N> {
    fn drop(&mut self) {
        self.phase_us[self.phase] = self.phase_us[self.phase].saturating_add(micros(self.phase_started));
        let total = micros(self.started);
        for (i, elapsed) in self.phase_us.iter().copied().enumerate() {
            self.batch.phase_us[i].fetch_add(elapsed, Ordering::Relaxed);
            self.batch.phase_max_us[i].fetch_max(elapsed, Ordering::Relaxed);
            self.batch.phase_calls[i].fetch_add(u64::from(self.visited[i]), Ordering::Relaxed);
        }
        self.batch.total_us.fetch_add(total, Ordering::Relaxed);
        self.batch.max_us.fetch_max(total, Ordering::Relaxed);
        self.batch.tail_calls.fetch_add(u64::from(self.reached_tail), Ordering::Relaxed);
        self.batch.calls.fetch_add(1, Ordering::Relaxed);
    }
}

/// Sequential components within one existing pipeline stage. Nested worker
/// totals are separate inclusive observations and must not be added again.
pub(crate) struct SequentialTimings<'a> {
    project: &'a str,
    scope: &'a str,
    component: &'static str,
    started: Instant,
}

impl<'a> SequentialTimings<'a> {
    pub(crate) fn new(project: &'a str, scope: &'a str, component: &'static str) -> Self {
        Self { project, scope, component, started: Instant::now() }
    }

    pub(crate) fn enter(&mut self, component: &'static str) {
        self.emit();
        self.component = component;
        self.started = Instant::now();
    }

    fn emit(&self) {
        let _ = writeln!(std::io::stderr().lock(),
            "ETUSCHLOPF: DetailTiming: schema=1; project={:?}; scope={:?}; component={:?}; wall_us={}; authority=none",
            self.project, self.scope, self.component, micros(self.started));
    }
}

impl Drop for SequentialTimings<'_> {
    fn drop(&mut self) { self.emit(); }
}

