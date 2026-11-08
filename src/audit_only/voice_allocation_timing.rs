//! Read-only allocation profiling. Clock values and counters never leave this
//! diagnostic module as decision inputs. No notation, plan or proof is received.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
struct State {
    last_pulse: Instant,
    visits: u64,
    totals: BTreeMap<&'static str, (u64, u128, u128)>,
}
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State {
        last_pulse: Instant::now(), visits: 0, totals: BTreeMap::new(),
    });
}

pub(crate) struct Timer {
    phase: &'static str,
    id: u64,
    start: Instant,
    boundary: bool,
}
impl Timer {
    pub(crate) fn span(phase: &'static str) -> Self {
        let value = Self { phase, id: NEXT_ID.fetch_add(1, Ordering::Relaxed), start: Instant::now(), boundary: true };
        eprintln!("VoiceAllocationTiming: id={}; phase={phase}; event=begin; authority=none", value.id);
        value
    }
    pub(crate) fn probe(phase: &'static str) -> Self {
        Self { phase, id: NEXT_ID.fetch_add(1, Ordering::Relaxed), start: Instant::now(), boundary: false }
    }
}
impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        let micros = elapsed.as_micros();
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            let row = state.totals.entry(self.phase).or_default();
            row.0 = row.0.saturating_add(1);
            row.1 = row.1.saturating_add(micros);
            row.2 = row.2.max(micros);
        });
        if self.boundary || elapsed >= Duration::from_secs(5) {
            eprintln!("VoiceAllocationTiming: id={}; phase={}; event=end; us={micros}; authority=none", self.id, self.phase);
        }
    }
}

pub(crate) fn pulse() {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.visits = state.visits.saturating_add(1);
        if state.last_pulse.elapsed() >= Duration::from_secs(5) {
            eprintln!("VoiceAllocationProgress: visited={}; authority=none", state.visits);
            state.last_pulse = Instant::now();
        }
    });
}

pub(crate) fn summary() {
    STATE.with(|state| {
        for (phase, (calls, us, max_us)) in &state.borrow().totals {
            eprintln!("VoiceAllocationTimingSummary: phase={phase}; calls={calls}; inclusive_us={us}; max_us={max_us}; authority=none");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nested_audit_spans_count_each_completed_call_exactly_once() {
        STATE.with(|state| state.borrow_mut().totals.clear());
        { let _parent = Timer::span("test_parent"); let _child = Timer::probe("test_child"); }
        STATE.with(|state| {
            let state = state.borrow();
            assert_eq!(state.totals["test_parent"].0, 1);
            assert_eq!(state.totals["test_child"].0, 1);
            assert!(state.totals["test_parent"].1 >= state.totals["test_child"].1);
        });
    }
}
