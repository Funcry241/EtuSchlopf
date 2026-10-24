//! r1188/r1192/r1193 report-only observation of opaque destructive-edit identity transport.
//!
//! r1188 established one opaque transaction-local identity per concrete caller-supplied
//! replacement edit. r1192 extends only the receipt lineage of identities that actually
//! reached `Applied`: the complete pre/post document state is fingerprint-bound at the
//! authority boundary and may then be continued through exact in-memory state succession
//! or re-sealed by an explicitly observed write on the configured final Render-MusicXML
//! path. No measure, staff, voice, onset, range, pitch, XML locator, function name or
//! report label is used to recreate an identity.
//!
//! r1193 also observes exact receipt-bearing states only after existing productive selection
//! handoffs, separating candidate application without a covered selection witness from selected
//! states without changing any selector. The final-epoch receipt remains historical provenance.
//! It proves that the same edit receipt was carried to the final observed Render artifact epoch;
//! it does not by itself prove that the edit's musical effect remains semantically necessary or
//! visible in those bytes.
//!
//! r1198 closes the four r1196-selected lineage loss boundaries without rediscovering
//! identity: an explicit causal selected-state successor may carry the union of the
//! predecessor receipt set and receipts already produced by the selected child state,
//! because the productive selection edge disambiguates which producer instance became
//! authoritative. Final-artifact-only selection witnesses are additionally scoped to the
//! configured final Render path, so renderer-feasibility shadow materialization cannot
//! populate the productive selection set.
//!
//! r1201 keeps this thread-local ledger unchanged and adds an observation-only process-wide
//! mirror plus a report that compares both, measuring identity loss caused only by thread
//! locality. It also projects the static identity coverage and nominal-gate inventory.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread::ThreadId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditState {
    OriginOnly,
    Bound,
    Applied,
}

#[derive(Debug, Clone)]
struct BatchObservation {
    origin_file: String,
    origin_line: u32,
    origin_column: u32,
    edits: Vec<EditState>,
}

type EditKey = (u64, usize);
type StateFingerprint = [u8; 64];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ReceiptLineageMode {
    Continued,
    Rebased,
    Resealed,
}

#[derive(Debug, Clone, Default)]
struct StateReceiptEvidence {
    certain_receipts: BTreeMap<EditKey, ReceiptLineageMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FinalEpochReceiptObservation {
    epoch_serial: u64,
    mode: ReceiptLineageMode,
}

#[derive(Debug, Clone)]
struct LineageCheckpointObservation {
    label: String,
    state_receipts: BTreeSet<EditKey>,
    selected_receipts: BTreeSet<EditKey>,
}

#[derive(Debug, Default)]
struct AttributionState {
    project: String,
    batches: BTreeMap<u64, BatchObservation>,
    integrity_errors: usize,
    final_render_path: Option<String>,
    state_receipts: BTreeMap<StateFingerprint, StateReceiptEvidence>,
    final_path_current_state: Option<StateFingerprint>,
    final_path_current_receipts: BTreeMap<EditKey, ReceiptLineageMode>,
    final_epoch_receipts: BTreeMap<EditKey, FinalEpochReceiptObservation>,
    selected_receipts: BTreeSet<EditKey>,
    selected_state_observations: usize,
    render_epoch_boundaries: usize,
    final_render_boundaries: usize,
    last_final_render_epoch_serial: Option<u64>,
    receipt_integrity_errors: usize,
    state_provenance_ambiguity_drops: usize,
    lineage_checkpoints: Vec<LineageCheckpointObservation>,
}

thread_local! {
    static ATTRIBUTION_STATE: RefCell<Option<AttributionState>> = const { RefCell::new(None) };
}

/// r1201: observation-only process-wide mirror of the same ledger. The thread-local
/// ledger above stays the unchanged r1188-r1200 measurement. The mirror applies exactly the
/// same transitions, additionally receives observations issued on threads without an
/// active thread-local ledger (for example scoped worker threads), and counts them. It is
/// read only by the report projection and never by any productive code path.
#[derive(Debug)]
struct ProcessWideSession {
    owner: ThreadId,
    ledger: AttributionState,
    detached: DetachedTally,
}

#[derive(Debug, Default)]
struct DetachedTally {
    origin: usize,
    bound: usize,
    applied: usize,
    state_successor: usize,
    state_successor_with_receipts: usize,
    receipts_carried: usize,
    state_only: usize,
    batches: BTreeSet<u64>,
}

#[derive(Debug, Clone, Copy)]
enum ObservationKind {
    Origin(u64),
    Bound(u64),
    Applied(u64),
    Successor(StateFingerprint),
    StateOnly,
}

static PROCESS_WIDE_SESSIONS: Mutex<Vec<ProcessWideSession>> = Mutex::new(Vec::new());
static UNROUTABLE_DETACHED_OBSERVATIONS: AtomicUsize = AtomicUsize::new(0);

impl DetachedTally {
    fn count(&mut self, kind: ObservationKind, ledger: &AttributionState) {
        match kind {
            ObservationKind::Origin(batch_nonce) => {
                self.origin = self.origin.saturating_add(1);
                self.batches.insert(batch_nonce);
            }
            ObservationKind::Bound(batch_nonce) => {
                self.bound = self.bound.saturating_add(1);
                self.batches.insert(batch_nonce);
            }
            ObservationKind::Applied(batch_nonce) => {
                self.applied = self.applied.saturating_add(1);
                self.batches.insert(batch_nonce);
            }
            ObservationKind::Successor(before) => {
                self.state_successor = self.state_successor.saturating_add(1);
                let carried = ledger
                    .state_receipts
                    .get(&before)
                    .map_or(0, |evidence| evidence.certain_receipts.len());
                if carried > 0 {
                    self.state_successor_with_receipts =
                        self.state_successor_with_receipts.saturating_add(1);
                    self.receipts_carried = self.receipts_carried.saturating_add(carried);
                }
            }
            ObservationKind::StateOnly => self.state_only = self.state_only.saturating_add(1),
        }
    }
}

/// Applies one ledger transition to the thread-local ledger (when active) and to the
/// process-wide mirror. Observations from a thread without its own ledger are routed to the
/// mirror only when exactly one session exists; otherwise they are counted as unroutable.
fn with_ledgers(kind: ObservationKind, mut transition: impl FnMut(&mut AttributionState)) {
    let owned = ATTRIBUTION_STATE.with(|slot| match slot.borrow_mut().as_mut() {
        Some(state) => {
            transition(state);
            true
        }
        None => false,
    });
    let Ok(mut sessions) = PROCESS_WIDE_SESSIONS.lock() else {
        return;
    };
    if owned {
        let thread = std::thread::current().id();
        if let Some(session) = sessions.iter_mut().find(|session| session.owner == thread) {
            transition(&mut session.ledger);
        }
        return;
    }
    match sessions.as_mut_slice() {
        [] => {}
        [session] => {
            session.detached.count(kind, &session.ledger);
            transition(&mut session.ledger);
        }
        _ => {
            UNROUTABLE_DETACHED_OBSERVATIONS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn fresh_state(project: &str, final_render_path: Option<&Path>) -> AttributionState {
    AttributionState {
        project: project.to_string(),
        batches: BTreeMap::new(),
        integrity_errors: 0,
        final_render_path: final_render_path.map(path_identity),
        state_receipts: BTreeMap::new(),
        final_path_current_state: None,
        final_path_current_receipts: BTreeMap::new(),
        final_epoch_receipts: BTreeMap::new(),
        selected_receipts: BTreeSet::new(),
        selected_state_observations: 0,
        render_epoch_boundaries: 0,
        final_render_boundaries: 0,
        last_final_render_epoch_serial: None,
        receipt_integrity_errors: 0,
        state_provenance_ambiguity_drops: 0,
        lineage_checkpoints: Vec::new(),
    }
}

pub(crate) fn start(project: &str, final_render_path: Option<&Path>) {
    super::voice_run_collision_origin::start();
    ATTRIBUTION_STATE.with(|slot| {
        *slot.borrow_mut() = Some(fresh_state(project, final_render_path));
    });
    let thread = std::thread::current().id();
    if let Ok(mut sessions) = PROCESS_WIDE_SESSIONS.lock() {
        sessions.retain(|session| session.owner != thread);
        sessions.push(ProcessWideSession {
            owner: thread,
            ledger: fresh_state(project, final_render_path),
            detached: DetachedTally::default(),
        });
    }
}

pub(crate) fn clear() {
    ATTRIBUTION_STATE.with(|slot| {
        *slot.borrow_mut() = None;
    });
    let thread = std::thread::current().id();
    if let Ok(mut sessions) = PROCESS_WIDE_SESSIONS.lock() {
        sessions.retain(|session| session.owner != thread);
    }
}

#[derive(Debug, Clone, Copy)]
struct OriginRef<'a> {
    batch_nonce: u64,
    file: &'a str,
    line: u32,
    column: u32,
    edit_count: usize,
}

pub(crate) fn observe_origin(
    batch_nonce: u64,
    origin_file: &str,
    origin_line: u32,
    origin_column: u32,
    edit_count: usize,
) {
    let origin = OriginRef {
        batch_nonce,
        file: origin_file,
        line: origin_line,
        column: origin_column,
        edit_count,
    };
    with_ledgers(ObservationKind::Origin(batch_nonce), |state| origin_on(state, origin));
}

fn origin_on(state: &mut AttributionState, origin: OriginRef<'_>) {
    if let Some(existing) = state.batches.get(&origin.batch_nonce) {
        if existing.origin_file != origin.file
            || existing.origin_line != origin.line
            || existing.origin_column != origin.column
            || existing.edits.len() != origin.edit_count
        {
            state.integrity_errors = state.integrity_errors.saturating_add(1);
        }
        return;
    }
    state.batches.insert(
        origin.batch_nonce,
        BatchObservation {
            origin_file: origin.file.to_string(),
            origin_line: origin.line,
            origin_column: origin.column,
            edits: vec![EditState::OriginOnly; origin.edit_count],
        },
    );
}

fn phase_on(state: &mut AttributionState, origin: OriginRef<'_>, ordinal: usize, phase: EditState) {
    origin_on(state, origin);
    let Some(batch) = state.batches.get_mut(&origin.batch_nonce) else {
        state.integrity_errors = state.integrity_errors.saturating_add(1);
        return;
    };
    let metadata_matches = batch.origin_file == origin.file
        && batch.origin_line == origin.line
        && batch.origin_column == origin.column
        && batch.edits.len() == origin.edit_count;
    let Some(edit) = batch.edits.get_mut(ordinal) else {
        state.integrity_errors = state.integrity_errors.saturating_add(1);
        return;
    };
    if !metadata_matches {
        state.integrity_errors = state.integrity_errors.saturating_add(1);
        return;
    }
    match (*edit, phase) {
        (EditState::OriginOnly, EditState::Bound)
        | (EditState::OriginOnly, EditState::Applied)
        | (EditState::Bound, EditState::Applied) => *edit = phase,
        (EditState::Bound, EditState::Bound) | (EditState::Applied, EditState::Applied) => {}
        (EditState::Applied, EditState::Bound) => {
            state.integrity_errors = state.integrity_errors.saturating_add(1);
        }
        (_, EditState::OriginOnly) => {}
    }
}

pub(crate) fn observe_bound(
    batch_nonce: u64,
    ordinal: usize,
    origin_file: &str,
    origin_line: u32,
    origin_column: u32,
    edit_count: usize,
) {
    let origin = OriginRef {
        batch_nonce,
        file: origin_file,
        line: origin_line,
        column: origin_column,
        edit_count,
    };
    with_ledgers(ObservationKind::Bound(batch_nonce), |state| {
        phase_on(state, origin, ordinal, EditState::Bound);
    });
}

pub(crate) fn observe_applied_batch_transition(
    batch_nonce: u64,
    ordinals: &[usize],
    origin_file: &str,
    origin_line: u32,
    origin_column: u32,
    edit_count: usize,
    source_transition: (&str, &str),
) {
    let (source_before, source_after) = source_transition;
    if ordinals.is_empty() {
        return;
    }
    super::voice_run_collision_origin::observe_writer_transition(
        &format!("{origin_file}:{origin_line}"),
        source_before,
        source_after,
    );
    let origin = OriginRef {
        batch_nonce,
        file: origin_file,
        line: origin_line,
        column: origin_column,
        edit_count,
    };
    let unique_ordinals = ordinals.iter().copied().collect::<BTreeSet<_>>();
    let duplicate_ordinals = unique_ordinals.len() != ordinals.len();
    let fingerprints = if duplicate_ordinals {
        None
    } else {
        Some((state_fingerprint(source_before), state_fingerprint(source_after)))
    };
    with_ledgers(ObservationKind::Applied(batch_nonce), |state| {
        let Some((before, after)) = fingerprints else {
            state.receipt_integrity_errors = state.receipt_integrity_errors.saturating_add(1);
            return;
        };
        applied_on(state, origin, &unique_ordinals, before, after);
    });
}

fn applied_on(
    state: &mut AttributionState,
    origin: OriginRef<'_>,
    unique_ordinals: &BTreeSet<usize>,
    before: StateFingerprint,
    after: StateFingerprint,
) {
    for ordinal in unique_ordinals {
        phase_on(state, origin, *ordinal, EditState::Applied);
    }
    let batch_nonce = origin.batch_nonce;
    let all_applied = unique_ordinals.iter().all(|ordinal| {
        state
            .batches
            .get(&batch_nonce)
            .and_then(|batch| batch.edits.get(*ordinal))
            .is_some_and(|edit| *edit == EditState::Applied)
    });
    if !all_applied {
        state.receipt_integrity_errors = state.receipt_integrity_errors.saturating_add(1);
        return;
    }

    let inherited = state
        .state_receipts
        .get(&before)
        .map(|evidence| evidence.certain_receipts.clone())
        .unwrap_or_default();
    let mut candidate_receipts = BTreeMap::<EditKey, ReceiptLineageMode>::new();
    for (key, mode) in inherited {
        insert_conservative_mode(
            &mut candidate_receipts,
            key,
            advance_mode(mode, ReceiptLineageMode::Rebased),
        );
    }
    for ordinal in unique_ordinals {
        insert_conservative_mode(
            &mut candidate_receipts,
            (batch_nonce, *ordinal),
            ReceiptLineageMode::Continued,
        );
    }
    register_state_producer(state, after, candidate_receipts);
}

pub(crate) fn observe_exact_state_successor(source_before: &str, source_after: &str) {
    let before = state_fingerprint(source_before);
    let after = state_fingerprint(source_after);
    with_ledgers(ObservationKind::Successor(before), |state| {
        exact_successor_on(state, before, after);
    });
}

fn exact_successor_on(state: &mut AttributionState, before: StateFingerprint, after: StateFingerprint) {
    if before == after {
        return;
    }
    let Some(evidence) = state.state_receipts.get(&before) else {
        return;
    };
    let mut candidate_receipts = BTreeMap::<EditKey, ReceiptLineageMode>::new();
    for (key, mode) in &evidence.certain_receipts {
        insert_conservative_mode(
            &mut candidate_receipts,
            *key,
            advance_mode(*mode, ReceiptLineageMode::Rebased),
        );
    }
    register_state_producer(state, after, candidate_receipts);
}

#[track_caller]
pub(crate) fn observe_selected_state(xml: &str) {
    super::voice_run_collision_origin::observe_reported_state(
        &std::panic::Location::caller().to_string(),
        xml,
    );
    let fingerprint = state_fingerprint(xml);
    with_ledgers(ObservationKind::StateOnly, |state| {
        selected_state_on(state, fingerprint);
    });
}

fn selected_state_on(state: &mut AttributionState, fingerprint: StateFingerprint) {
    state.selected_state_observations = state.selected_state_observations.saturating_add(1);
    let Some(evidence) = state.state_receipts.get(&fingerprint) else {
        return;
    };
    state
        .selected_receipts
        .extend(evidence.certain_receipts.keys().copied());
}

#[track_caller]
pub(crate) fn observe_selected_state_successor(source_before: &str, source_after: &str) {
    let before = state_fingerprint(source_before);
    let after = state_fingerprint(source_after);
    if before == after {
        observe_selected_state(source_after);
        return;
    }
    super::voice_run_collision_origin::observe_reported_state(
        &std::panic::Location::caller().to_string(),
        source_after,
    );
    with_ledgers(ObservationKind::Successor(before), |state| {
        selected_successor_on(state, before, after);
    });
}

fn selected_successor_on(state: &mut AttributionState, before: StateFingerprint, after: StateFingerprint) {
    state.selected_state_observations = state.selected_state_observations.saturating_add(1);

    let mut selected_receipts = state
        .state_receipts
        .get(&before)
        .map(|evidence| evidence.certain_receipts.clone())
        .unwrap_or_default();
    for mode in selected_receipts.values_mut() {
        *mode = advance_mode(*mode, ReceiptLineageMode::Rebased);
    }
    if let Some(existing) = state.state_receipts.get(&after) {
        for (key, mode) in &existing.certain_receipts {
            insert_conservative_mode(&mut selected_receipts, *key, *mode);
        }
    }
    if selected_receipts.is_empty() {
        return;
    }

    state.state_receipts.insert(
        after,
        StateReceiptEvidence {
            certain_receipts: selected_receipts.clone(),
        },
    );
    state
        .selected_receipts
        .extend(selected_receipts.keys().copied());
}

pub(crate) fn observe_selected_final_artifact_state(path: &Path, xml: &str) {
    super::voice_run_collision_origin::observe_reported_state("final_artifact_state", xml);
    let observed_path = path_identity(path);
    let fingerprint = state_fingerprint(xml);
    with_ledgers(ObservationKind::StateOnly, |state| {
        final_artifact_state_on(state, &observed_path, fingerprint);
    });
}

fn final_artifact_state_on(state: &mut AttributionState, observed_path: &str, fingerprint: StateFingerprint) {
    if state.final_render_path.as_deref() != Some(observed_path) {
        return;
    }
    state.selected_state_observations = state.selected_state_observations.saturating_add(1);
    let Some(evidence) = state.state_receipts.get(&fingerprint) else {
        return;
    };
    state
        .selected_receipts
        .extend(evidence.certain_receipts.keys().copied());
}

pub(crate) fn observe_lineage_checkpoint(label: &str, xml: &str) {
    super::voice_run_collision_origin::observe_reported_state(label, xml);
    let fingerprint = state_fingerprint(xml);
    with_ledgers(ObservationKind::StateOnly, |state| {
        lineage_checkpoint_on(state, label, fingerprint);
    });
}

fn lineage_checkpoint_on(state: &mut AttributionState, label: &str, fingerprint: StateFingerprint) {
    let state_receipts = state
        .state_receipts
        .get(&fingerprint)
        .map(|evidence| evidence.certain_receipts.keys().copied().collect())
        .unwrap_or_default();
    let selected_receipts = state.selected_receipts.clone();
    state.lineage_checkpoints.push(LineageCheckpointObservation {
        label: label.to_string(),
        state_receipts,
        selected_receipts,
    });
}

pub(crate) fn observe_render_epoch_boundary(
    path: &Path,
    epoch_serial: u64,
    before: Option<&str>,
    after: &str,
) {
    let observed_path = path_identity(path);
    let before_state = before.map(state_fingerprint);
    let after_state = state_fingerprint(after);
    with_ledgers(ObservationKind::StateOnly, |state| {
        render_epoch_boundary_on(state, &observed_path, epoch_serial, before_state, after_state);
    });
}

fn render_epoch_boundary_on(
    state: &mut AttributionState,
    observed_path: &str,
    epoch_serial: u64,
    before_state: Option<StateFingerprint>,
    after_state: StateFingerprint,
) {
    state.render_epoch_boundaries = state.render_epoch_boundaries.saturating_add(1);
    if state.final_render_path.as_deref() != Some(observed_path) {
        return;
    }

    state.final_render_boundaries = state.final_render_boundaries.saturating_add(1);
    state.last_final_render_epoch_serial = Some(epoch_serial);

    let mut final_receipts = state
        .state_receipts
        .get(&after_state)
        .map(|evidence| evidence.certain_receipts.clone())
        .unwrap_or_default();
    if before_state.is_some() && before_state == state.final_path_current_state {
        for (key, mode) in &state.final_path_current_receipts {
            insert_conservative_mode(
                &mut final_receipts,
                *key,
                advance_mode(*mode, ReceiptLineageMode::Resealed),
            );
        }
    }

    state.final_path_current_state = Some(after_state);
    state.final_path_current_receipts = final_receipts.clone();
    for (key, mode) in final_receipts {
        state.final_epoch_receipts.insert(
            key,
            FinalEpochReceiptObservation {
                epoch_serial,
                mode,
            },
        );
    }
}

fn register_state_producer(
    state: &mut AttributionState,
    fingerprint: StateFingerprint,
    candidate_receipts: BTreeMap<EditKey, ReceiptLineageMode>,
) {
    let dropped = match state.state_receipts.entry(fingerprint) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(StateReceiptEvidence {
                certain_receipts: candidate_receipts,
            });
            return;
        }
        std::collections::btree_map::Entry::Occupied(mut entry) => {
            let existing = entry.get_mut();
            let prior_len = existing.certain_receipts.len();
            existing.certain_receipts.retain(|key, existing_mode| {
                let Some(candidate_mode) = candidate_receipts.get(key) else {
                    return false;
                };
                *existing_mode = std::cmp::max(*existing_mode, *candidate_mode);
                true
            });
            prior_len.saturating_sub(existing.certain_receipts.len())
        }
    };
    state.state_provenance_ambiguity_drops = state
        .state_provenance_ambiguity_drops
        .saturating_add(dropped);
}

fn advance_mode(current: ReceiptLineageMode, boundary: ReceiptLineageMode) -> ReceiptLineageMode {
    std::cmp::max(current, boundary)
}

fn insert_conservative_mode(
    target: &mut BTreeMap<EditKey, ReceiptLineageMode>,
    key: EditKey,
    candidate: ReceiptLineageMode,
) {
    match target.get_mut(&key) {
        Some(existing) if candidate > *existing => *existing = candidate,
        Some(_) => {}
        None => {
            target.insert(key, candidate);
        }
    }
}

fn state_fingerprint(text: &str) -> StateFingerprint {
    let sha256 = crate::sha256::hex(text.as_bytes());
    let mut sha256_hex = [0_u8; 64];
    sha256_hex.copy_from_slice(sha256.as_bytes());
    sha256_hex
}

fn path_identity(path: &Path) -> String {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|current| current.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };
    absolute
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

fn applied_edit_keys(state: &AttributionState) -> BTreeSet<EditKey> {
    let mut keys = BTreeSet::new();
    for (batch_nonce, batch) in &state.batches {
        for (ordinal, edit) in batch.edits.iter().enumerate() {
            if *edit == EditState::Applied {
                keys.insert((*batch_nonce, ordinal));
            }
        }
    }
    keys
}

fn final_receipt_mode(state: &AttributionState, key: EditKey) -> Option<ReceiptLineageMode> {
    let final_epoch = state.last_final_render_epoch_serial?;
    state
        .final_epoch_receipts
        .get(&key)
        .filter(|receipt| receipt.epoch_serial == final_epoch)
        .map(|receipt| receipt.mode)
}

pub(crate) fn append_report(out: &mut String) {
    ATTRIBUTION_STATE.with(|slot| {
        let slot = slot.borrow();
        let Some(state) = slot.as_ref() else {
            out.push_str("DestructiveEditBatchAttribution: state unavailable; authority none; music mutation none.\n");
            return;
        };

        let batch_exact = state
            .batches
            .values()
            .filter(|batch| {
                !batch.edits.is_empty()
                    && batch.edits.iter().all(|edit| *edit == EditState::Applied)
            })
            .count();
        let batch_only = state
            .batches
            .values()
            .filter(|batch| {
                !batch.edits.is_empty()
                    && !batch.edits.iter().all(|edit| *edit == EditState::Applied)
                    && batch.edits.iter().any(|edit| *edit != EditState::OriginOnly)
            })
            .count();
        let origin_only = state.batches.len().saturating_sub(batch_exact + batch_only);
        let edits_exact = state
            .batches
            .values()
            .flat_map(|batch| batch.edits.iter())
            .filter(|edit| **edit == EditState::Applied)
            .count();
        let edits_batch_only = state
            .batches
            .values()
            .flat_map(|batch| batch.edits.iter())
            .filter(|edit| **edit == EditState::Bound)
            .count();
        let edits_origin_only = state
            .batches
            .values()
            .flat_map(|batch| batch.edits.iter())
            .filter(|edit| **edit == EditState::OriginOnly)
            .count();

        out.push_str("DestructiveEditBatchAttributionContract: opaque batch nonce plus transaction-local edit ordinal; origin is the exact productive authority callsite; each surviving bound edit carries its own ordinal through reordering; no physical locator is used to recover identity; observation grants no authority.\n");
        out.push_str("DestructiveEditBatchAttributionCoverage: frozen r1186 core is all caller-supplied replacement authority surfaces in bounded_semantic_rewrite_authority, final_buffer_surface_authority, render_measure_articulation_authority, rest_voice_family_authority, staff_voice_atomic_authority and strict_staff_visible_lane_authority; 30 entry surfaces total, 25 productive and 5 retired. Shared typed capabilities also transport identity for adjacent non-batch entries that use the same exact binding path.\n");
        out.push_str(&format!(
            "DestructiveEditBatchAttribution: Project {}; Batches {}; EXACT {}; BATCH_ONLY {}; ORIGIN_ONLY {}; IndividualEdits {}; EditEXACT {}; EditBATCH_ONLY {}; EditORIGIN_ONLY {}; IntegrityErrors {}; MusicMutation none; WriteAuthority none.\n",
            state.project,
            state.batches.len(),
            batch_exact,
            batch_only,
            origin_only,
            edits_exact + edits_batch_only + edits_origin_only,
            edits_exact,
            edits_batch_only,
            edits_origin_only,
            state.integrity_errors,
        ));

        let applied_keys = applied_edit_keys(state);
        let mut continued = 0usize;
        let mut rebased = 0usize;
        let mut resealed = 0usize;
        let mut unknown = 0usize;
        for key in &applied_keys {
            match final_receipt_mode(state, *key) {
                Some(ReceiptLineageMode::Continued) => continued = continued.saturating_add(1),
                Some(ReceiptLineageMode::Rebased) => rebased = rebased.saturating_add(1),
                Some(ReceiptLineageMode::Resealed) => resealed = resealed.saturating_add(1),
                None => unknown = unknown.saturating_add(1),
            }
        }
        out.push_str("DestructiveEditFinalEpochReceiptContract: receipt lineage is bound only by the same opaque IndividualEditIdentity plus exact complete-document state succession and explicitly observed writes on the configured final Render-MusicXML path; physical XML locators are forbidden. Identical document bytes from multiple independent producers retain only the intersection of their receipt provenance. CONTINUED means the applied state itself reaches the final epoch, REBASED means the receipt crossed one or more exact in-memory authority state transitions, RESEALED means an already receipt-bearing persisted final-artifact state crossed a later observed final-artifact write, and UNKNOWN means no identity-safe path to the final observed Render epoch was established. Historical receipt continuity is not semantic proof that the edit effect remains visible, necessary or permitted.\n");
        let accounted = continued
            .saturating_add(rebased)
            .saturating_add(resealed)
            .saturating_add(unknown);
        let accounting_mismatch = applied_keys.len().abs_diff(accounted);
        out.push_str(&format!(
            "DestructiveEditFinalEpochReceipt: Project {}; AppliedEdits {}; Accounted {}; AccountingMismatch {}; RenderEpochBoundaries {}; FinalRenderBoundaries {}; FinalRenderEpoch {}; CONTINUED {}; REBASED {}; RESEALED {}; UNKNOWN {}; StateProvenanceAmbiguityDrops {}; ReceiptIntegrityErrors {}; MusicMutation none; WriteAuthority none; ProofAuthority none.\n",
            state.project,
            applied_keys.len(),
            accounted,
            accounting_mismatch,
            state.render_epoch_boundaries,
            state.final_render_boundaries,
            state
                .last_final_render_epoch_serial
                .map_or_else(|| "none".to_string(), |value| value.to_string()),
            continued,
            rebased,
            resealed,
            unknown,
            state.state_provenance_ambiguity_drops,
            state.receipt_integrity_errors,
        ));

        let mut final_receipt = 0usize;
        let mut selected_no_final_receipt = 0usize;
        let mut no_selection_witness = 0usize;
        for key in &applied_keys {
            if final_receipt_mode(state, *key).is_some() {
                final_receipt = final_receipt.saturating_add(1);
            } else if state.selected_receipts.contains(key) {
                selected_no_final_receipt = selected_no_final_receipt.saturating_add(1);
            } else {
                no_selection_witness = no_selection_witness.saturating_add(1);
            }
        }
        let disposition_accounted = final_receipt
            .saturating_add(selected_no_final_receipt)
            .saturating_add(no_selection_witness);
        out.push_str("DestructiveEditAppliedDispositionContract: r1193 observes exact complete-document states at explicit productive selection/commit handoffs; r1198 additionally carries receipts across an explicit causal selected parent->child state edge and scopes final-artifact-only selection witnesses to the configured canonical final Render path so renderer-feasibility shadow materialization cannot populate productive selection. Identity remains the same opaque IndividualEditIdentity already carried by r1188/r1192; no measure, staff, voice, onset, range, pitch, XML locator, proximity, function name or report label is used to recreate identity. FINAL_RECEIPT means the receipt reaches the final Render epoch; SELECTED_NO_FINAL_RECEIPT means an exact receipt-bearing productive selected state was observed but its identity-safe final-epoch lineage is not closed; NO_SELECTION_WITNESS means no exact receipt-bearing productive selected state was observed at any covered selection handoff; it does not by itself prove why that witness is absent. Classification is AuditOnly and grants no Proof or Permit.\n");
        out.push_str(&format!(
            "DestructiveEditAppliedDisposition: Project {}; AppliedEdits {}; Accounted {}; AccountingMismatch {}; SelectionObservations {}; FINAL_RECEIPT {}; SELECTED_NO_FINAL_RECEIPT {}; NO_SELECTION_WITNESS {}; MusicMutation none; WriteAuthority none; ProofAuthority none.\n",
            state.project,
            applied_keys.len(),
            disposition_accounted,
            applied_keys.len().abs_diff(disposition_accounted),
            state.selected_state_observations,
            final_receipt,
            selected_no_final_receipt,
            no_selection_witness,
        ));

        out.push_str("DestructiveEditLineageCheckpointContract: r1195 records read-only exact complete-document checkpoint membership for the eventual selected-without-final-receipt set; checkpoints neither create nor propagate receipts and do not participate in selection, writing, Proof or Permit.\n");
        let selected_gap_keys = state
            .selected_receipts
            .iter()
            .copied()
            .filter(|key| final_receipt_mode(state, *key).is_none())
            .collect::<BTreeSet<_>>();
        for checkpoint in &state.lineage_checkpoints {
            let gap_known = checkpoint
                .selected_receipts
                .intersection(&selected_gap_keys)
                .copied()
                .collect::<BTreeSet<_>>();
            let gap_present = gap_known.intersection(&checkpoint.state_receipts).count();
            let gap_absent = gap_known.len().saturating_sub(gap_present);
            out.push_str(&format!(
                "DestructiveEditLineageCheckpoint: Project {}; Label {}; StateReceipts {}; SelectedGapKnown {}; SelectedGapPresent {}; SelectedGapAbsent {}.\n",
                state.project,
                checkpoint.label,
                checkpoint.state_receipts.len(),
                gap_known.len(),
                gap_present,
                gap_absent,
            ));

            let mut by_origin = BTreeMap::<(String, u32, u32), (usize, usize)>::new();
            for key in &gap_known {
                let Some(batch) = state.batches.get(&key.0) else {
                    continue;
                };
                let entry = by_origin
                    .entry((
                        batch.origin_file.clone(),
                        batch.origin_line,
                        batch.origin_column,
                    ))
                    .or_insert((0, 0));
                if checkpoint.state_receipts.contains(key) {
                    entry.0 = entry.0.saturating_add(1);
                } else {
                    entry.1 = entry.1.saturating_add(1);
                }
            }
            for ((file, line, column), (present, absent)) in by_origin {
                if present + absent == 0 {
                    continue;
                }
                out.push_str(&format!(
                    "DestructiveEditLineageCheckpointOrigin: Label {}; {}:{}:{}; selected_gap_present {}; selected_gap_absent {}.\n",
                    checkpoint.label, file, line, column, present, absent,
                ));
            }
        }

        let mut origins = BTreeMap::<(String, u32, u32), (usize, usize, usize, usize)>::new();
        let mut receipt_origins =
            BTreeMap::<(String, u32, u32), (usize, usize, usize, usize)>::new();
        let mut disposition_origins =
            BTreeMap::<(String, u32, u32), (usize, usize, usize)>::new();
        for (batch_nonce, batch) in &state.batches {
            let entry = origins
                .entry((
                    batch.origin_file.clone(),
                    batch.origin_line,
                    batch.origin_column,
                ))
                .or_insert((0, 0, 0, 0));
            entry.0 += 1;
            entry.1 += batch.edits.len();
            entry.2 += batch
                .edits
                .iter()
                .filter(|edit| **edit == EditState::Applied)
                .count();
            entry.3 += batch
                .edits
                .iter()
                .filter(|edit| **edit == EditState::Bound)
                .count();

            let origin_key = (
                batch.origin_file.clone(),
                batch.origin_line,
                batch.origin_column,
            );
            let receipt_entry = receipt_origins
                .entry(origin_key.clone())
                .or_insert((0, 0, 0, 0));
            let disposition_entry = disposition_origins
                .entry(origin_key)
                .or_insert((0, 0, 0));
            for (ordinal, edit) in batch.edits.iter().enumerate() {
                if *edit != EditState::Applied {
                    continue;
                }
                let key = (*batch_nonce, ordinal);
                match final_receipt_mode(state, key) {
                    Some(ReceiptLineageMode::Continued) => receipt_entry.0 += 1,
                    Some(ReceiptLineageMode::Rebased) => receipt_entry.1 += 1,
                    Some(ReceiptLineageMode::Resealed) => receipt_entry.2 += 1,
                    None => receipt_entry.3 += 1,
                }
                if final_receipt_mode(state, key).is_some() {
                    disposition_entry.0 += 1;
                } else if state.selected_receipts.contains(&key) {
                    disposition_entry.1 += 1;
                } else {
                    disposition_entry.2 += 1;
                }
            }
        }
        for ((file, line, column), (batches, edits, exact, batch_only_edits)) in origins {
            out.push_str(&format!(
                "DestructiveEditCallsiteOrigin: {}:{}:{}; batches {}; edits {}; exact {}; batch_only {}; origin_only {}.\n",
                file,
                line,
                column,
                batches,
                edits,
                exact,
                batch_only_edits,
                edits.saturating_sub(exact + batch_only_edits),
            ));
        }
        for ((file, line, column), (continued, rebased, resealed, unknown)) in receipt_origins {
            if continued + rebased + resealed + unknown == 0 {
                continue;
            }
            out.push_str(&format!(
                "DestructiveEditFinalEpochReceiptOrigin: {}:{}:{}; continued {}; rebased {}; resealed {}; unknown {}.\n",
                file, line, column, continued, rebased, resealed, unknown,
            ));
        }
        for ((file, line, column), (final_receipt, selected_no_final, no_selection_witness)) in
            disposition_origins
        {
            if final_receipt + selected_no_final + no_selection_witness == 0 {
                continue;
            }
            out.push_str(&format!(
                "DestructiveEditAppliedDispositionOrigin: {}:{}:{}; final_receipt {}; selected_no_final_receipt {}; no_selection_witness {}.\n",
                file, line, column, final_receipt, selected_no_final, no_selection_witness,
            ));
        }
    });
}

const DISPOSITION_FINAL: &str = "FINAL_RECEIPT";
const DISPOSITION_SELECTED_OPEN: &str = "SELECTED_NO_FINAL_RECEIPT";
const DISPOSITION_NO_WITNESS: &str = "NO_SELECTION_WITNESS";
const DISPOSITION_NOT_ON_OWNER: &str = "NOT_APPLIED_ON_OWNER_THREAD";
const COVERAGE_LOST_CROSS_THREAD: &str = "IDENTITY_LOST_CROSS_THREAD";

fn applied_disposition(state: &AttributionState, key: EditKey) -> &'static str {
    if final_receipt_mode(state, key).is_some() {
        DISPOSITION_FINAL
    } else if state.selected_receipts.contains(&key) {
        DISPOSITION_SELECTED_OPEN
    } else {
        DISPOSITION_NO_WITNESS
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ThreadLossComparison {
    cells: BTreeMap<(&'static str, &'static str), usize>,
    owner_no_witness: usize,
    owner_no_witness_reaching_final: usize,
    owner_no_witness_reaching_open_selection: usize,
    detached_edits: usize,
    detached_applied_edits: usize,
    detached_applied_reaching_final: usize,
    detached_applied_reaching_open_selection: usize,
}

/// Compares the unchanged owner-thread ledger with the process-wide mirror, edit by edit.
fn compare_thread_loss(
    owner: &AttributionState,
    mirror: &AttributionState,
    detached_batches: &BTreeSet<u64>,
) -> ThreadLossComparison {
    let owner_applied = applied_edit_keys(owner);
    let mut comparison = ThreadLossComparison::default();
    for key in applied_edit_keys(mirror) {
        let owner_class = if owner_applied.contains(&key) {
            applied_disposition(owner, key)
        } else {
            DISPOSITION_NOT_ON_OWNER
        };
        let mirror_class = applied_disposition(mirror, key);
        *comparison.cells.entry((owner_class, mirror_class)).or_insert(0) += 1;
        if owner_class == DISPOSITION_NO_WITNESS {
            comparison.owner_no_witness += 1;
            match mirror_class {
                DISPOSITION_FINAL => comparison.owner_no_witness_reaching_final += 1,
                DISPOSITION_SELECTED_OPEN => comparison.owner_no_witness_reaching_open_selection += 1,
                _ => {}
            }
        }
        if detached_batches.contains(&key.0) {
            comparison.detached_applied_edits += 1;
            match mirror_class {
                DISPOSITION_FINAL => comparison.detached_applied_reaching_final += 1,
                DISPOSITION_SELECTED_OPEN => comparison.detached_applied_reaching_open_selection += 1,
                _ => {}
            }
        }
    }
    comparison.detached_edits = detached_batches
        .iter()
        .filter_map(|batch_nonce| mirror.batches.get(batch_nonce))
        .map(|batch| batch.edits.len())
        .sum();
    comparison
}

fn normalized_origin_path(file: &str) -> String {
    file.replace('\\', "/")
}

/// Static minting sites (path, line) -> identity-bearing entry locators.
fn minting_site_index() -> BTreeMap<(String, usize), Vec<u64>> {
    let mut index = BTreeMap::<(String, usize), Vec<u64>>::new();
    for site in crate::audit_only::destructive_carrier_effect_inventory::IDENTITY_MINTING_SITES {
        index
            .entry((site.path.to_string(), site.line))
            .or_default()
            .push(site.entry);
    }
    index
}

#[derive(Debug, Default, Clone, Copy)]
struct OriginRuntime {
    batches: usize,
    detached_batches: usize,
    applied_edits: usize,
    final_edits: usize,
    owner_final_edits: usize,
}

fn origin_runtime(
    owner: &AttributionState,
    mirror: &AttributionState,
    detached_batches: &BTreeSet<u64>,
) -> BTreeMap<(String, usize), OriginRuntime> {
    let mut out = BTreeMap::<(String, usize), OriginRuntime>::new();
    for (batch_nonce, batch) in &mirror.batches {
        let entry = out
            .entry((normalized_origin_path(&batch.origin_file), batch.origin_line as usize))
            .or_default();
        entry.batches += 1;
        if detached_batches.contains(batch_nonce) {
            entry.detached_batches += 1;
        }
        for (ordinal, edit) in batch.edits.iter().enumerate() {
            if *edit != EditState::Applied {
                continue;
            }
            let key = (*batch_nonce, ordinal);
            entry.applied_edits += 1;
            if final_receipt_mode(mirror, key).is_some() {
                entry.final_edits += 1;
            }
            if final_receipt_mode(owner, key).is_some() {
                entry.owner_final_edits += 1;
            }
        }
    }
    out
}

/// r1201 DestructiveIdentityCoverageAndThreadLossSuperAudit report projection.
pub(crate) fn append_identity_coverage_and_thread_loss_report(out: &mut String) {
    use crate::audit_only::destructive_carrier_effect_inventory::{
        IDENTITY_COVERAGE_FNV1A64, IDENTITY_COVERAGE_ROWS, IDENTITY_MINTING_SITES, NOMINAL_GATE_ROWS,
    };
    out.push_str("DestructiveIdentityCoverageContract: coverage classes are fixed from the architecture before measurement. IDENTITY_PRESENT means a productive transformation surface whose typed entry mints the opaque IndividualEditIdentity; IDENTITY_MISSING means a productive transformation surface without such minting; NOT_APPLICABLE means a persisting whole-file sink, which transports documents rather than edits; RETIRED means not productively reachable. IDENTITY_LOST_CROSS_THREAD is the runtime refinement of IDENTITY_PRESENT when at least one mint of that surface was issued on a thread without an active owner ledger. Runtime origins are matched to static minting sites only by exact source path and line of the track_caller callsite; no musical coordinate is read.\n");
    out.push_str("DestructiveIdentityThreadLossContract: the owner-thread ledger is the unchanged r1188-r1200 measurement. An observation-only process-wide mirror applies the same transitions and additionally receives observations from threads without an active owner ledger. Comparing both ledgers edit by edit measures how many dispositions depend only on thread locality. The mirror is read by this projection only; it changes no selection, write, Proof or Permit.\n");
    out.push_str("NominalGateInventoryContract: syntactic rules fixed before measurement: LITERAL_COORDINATE_TABLE (array of at least two integer-literal tuples with at least three components plus an equality test on a measure or scope binding), LITERAL_MEASURE_BINDING (a measure binding initialised with or compared to an integer literal greater than one), CONSTANT_TRUE_CHECK_FIELD (a field whose name declares a check outcome, read in a logical condition, and only ever assigned the literal true), CONSTANT_ZERO_GATE_FIELD (a numeric field only ever assigned the literal zero and compared with zero), CONSTANT_OPEN_GATE_PARAMETER (a bool gate parameter that every caller passes as literal true). Runtime association is REGION: identity-bearing edits whose callsite lies in the flagged function, its direct callers or its direct callees; it is not proof that the flagged construct decided the edit.\n");

    let sites = minting_site_index();
    ATTRIBUTION_STATE.with(|slot| {
        let slot = slot.borrow();
        let Some(owner) = slot.as_ref() else {
            out.push_str("DestructiveIdentityThreadLoss: owner ledger unavailable; MusicMutation none.\n");
            return;
        };
        let Ok(sessions) = PROCESS_WIDE_SESSIONS.lock() else {
            out.push_str("DestructiveIdentityThreadLoss: process-wide mirror unavailable; MusicMutation none.\n");
            return;
        };
        let thread = std::thread::current().id();
        let Some(session) = sessions.iter().find(|session| session.owner == thread) else {
            out.push_str("DestructiveIdentityThreadLoss: process-wide mirror unavailable; MusicMutation none.\n");
            return;
        };
        let mirror = &session.ledger;
        let detached = &session.detached;
        let runtime = origin_runtime(owner, mirror, &detached.batches);

        // Coverage per surface with runtime refinement.
        let mut per_entry = BTreeMap::<u64, (usize, usize)>::new();
        let mut unmatched_origins = 0usize;
        for ((path, line), origin) in &runtime {
            match sites.get(&(path.clone(), *line)) {
                Some(entries) => {
                    for entry in entries {
                        let counts = per_entry.entry(*entry).or_insert((0, 0));
                        counts.0 += origin.batches - origin.detached_batches;
                        counts.1 += origin.detached_batches;
                    }
                }
                None => unmatched_origins += 1,
            }
        }
        let mut families = BTreeMap::<(&str, &str), BTreeMap<&str, usize>>::new();
        let mut totals = BTreeMap::<&str, usize>::new();
        for row in IDENTITY_COVERAGE_ROWS {
            let (owner_batches, detached_batches) = per_entry.get(&row.locator).copied().unwrap_or((0, 0));
            let runtime_class = if row.coverage == "IDENTITY_PRESENT" && detached_batches > 0 {
                COVERAGE_LOST_CROSS_THREAD
            } else {
                row.coverage
            };
            *families
                .entry((row.surface, row.physical))
                .or_default()
                .entry(runtime_class)
                .or_insert(0) += 1;
            *totals.entry(runtime_class).or_insert(0) += 1;
            if row.surface == "FILE_SINK" {
                continue;
            }
            out.push_str(&format!(
                "DestructiveIdentityCoverageSurface: owner {}; locator {:016x}; surface {}; physical {}; reachability {}; static_coverage {}; coverage {}; owner_thread_batches {}; detached_batches {}.\n",
                row.owner, row.locator, row.surface, row.physical, row.reachability, row.coverage, runtime_class, owner_batches, detached_batches,
            ));
        }
        for ((surface, physical), classes) in &families {
            let rendered = classes
                .iter()
                .map(|(class, count)| format!("{class} {count}"))
                .collect::<Vec<_>>()
                .join("; ");
            out.push_str(&format!(
                "DestructiveIdentityCoverageFamily: surface {surface}; physical {physical}; {rendered}.\n"
            ));
        }
        let total = |class: &str| totals.get(class).copied().unwrap_or(0);
        out.push_str(&format!(
            "DestructiveIdentityCoverage: Project {}; Surfaces {}; IDENTITY_PRESENT {}; IDENTITY_LOST_CROSS_THREAD {}; IDENTITY_MISSING {}; NOT_APPLICABLE {}; RETIRED {}; StaticMintingSites {}; RuntimeOrigins {}; RuntimeOriginsWithoutStaticSite {}; Fingerprint FNV1A64:{:016x}; MusicMutation none; WriteAuthority none.\n",
            owner.project,
            IDENTITY_COVERAGE_ROWS.len(),
            total("IDENTITY_PRESENT"),
            total(COVERAGE_LOST_CROSS_THREAD),
            total("IDENTITY_MISSING"),
            total("NOT_APPLICABLE"),
            total("RETIRED"),
            IDENTITY_MINTING_SITES.len(),
            runtime.len(),
            unmatched_origins,
            IDENTITY_COVERAGE_FNV1A64,
        ));

        // Thread-loss measurement.
        let comparison = compare_thread_loss(owner, mirror, &detached.batches);
        out.push_str(&format!(
            "DestructiveIdentityThreadLoss: Project {}; DetachedObservations {}; DetachedOrigin {}; DetachedBound {}; DetachedApplied {}; DetachedStateSuccessor {}; DetachedStateSuccessorWithReceipts {}; DetachedReceiptsCarried {}; DetachedStateOnly {}; UnroutableDetachedObservations {}; DetachedBatches {}; DetachedEdits {}; DetachedAppliedEdits {}; DetachedAppliedReachingFinal {}; DetachedAppliedReachingOpenSelection {}; MusicMutation none.\n",
            owner.project,
            detached.origin + detached.bound + detached.applied + detached.state_successor + detached.state_only,
            detached.origin,
            detached.bound,
            detached.applied,
            detached.state_successor,
            detached.state_successor_with_receipts,
            detached.receipts_carried,
            detached.state_only,
            UNROUTABLE_DETACHED_OBSERVATIONS.load(Ordering::Relaxed),
            detached.batches.len(),
            comparison.detached_edits,
            comparison.detached_applied_edits,
            comparison.detached_applied_reaching_final,
            comparison.detached_applied_reaching_open_selection,
        ));
        out.push_str(&format!(
            "DestructiveIdentityThreadLossNoSelectionWitness: Project {}; OwnerThreadNoSelectionWitness {}; ReachingFinalInMirror {}; ReachingOpenSelectionInMirror {}; ExplainedByThreadLocality {}; RemainingWithoutWitness {}.\n",
            owner.project,
            comparison.owner_no_witness,
            comparison.owner_no_witness_reaching_final,
            comparison.owner_no_witness_reaching_open_selection,
            comparison.owner_no_witness_reaching_final + comparison.owner_no_witness_reaching_open_selection,
            comparison.owner_no_witness
                - comparison.owner_no_witness_reaching_final
                - comparison.owner_no_witness_reaching_open_selection,
        ));
        for ((owner_class, mirror_class), edits) in &comparison.cells {
            out.push_str(&format!(
                "DestructiveIdentityThreadLossDisposition: owner_thread {owner_class}; process_wide {mirror_class}; edits {edits}.\n"
            ));
        }
        for ((path, line), origin) in &runtime {
            if origin.detached_batches == 0 && origin.final_edits == origin.owner_final_edits {
                continue;
            }
            out.push_str(&format!(
                "DestructiveIdentityThreadLossOrigin: {}:{}; batches {}; detached_batches {}; applied_edits {}; final_process_wide {}; final_owner_thread {}.\n",
                path, line, origin.batches, origin.detached_batches, origin.applied_edits, origin.final_edits, origin.owner_final_edits,
            ));
        }

        // Nominal gate inventory with region association.
        let mut influence_totals = BTreeMap::<&str, usize>::new();
        let mut productive_rows = 0usize;
        for gate in NOMINAL_GATE_ROWS {
            let mut linked = OriginRuntime::default();
            for (path, line) in gate.linked_sites {
                if let Some(origin) = runtime.get(&((*path).to_string(), *line)) {
                    linked.batches += origin.batches;
                    linked.detached_batches += origin.detached_batches;
                    linked.applied_edits += origin.applied_edits;
                    linked.final_edits += origin.final_edits;
                    linked.owner_final_edits += origin.owner_final_edits;
                }
            }
            let influence = if gate.linked_sites.is_empty() {
                "UNKNOWN_NO_IDENTITY_LINK"
            } else if linked.batches == 0 {
                "LINKED_PATH_NOT_EXERCISED"
            } else if linked.applied_edits == 0 {
                "EXERCISED_NO_APPLIED_EDIT"
            } else if linked.final_edits == 0 {
                "APPLIED_WITHOUT_FINAL_RECEIPT"
            } else {
                "APPLIED_WITH_FINAL_RECEIPT"
            };
            if gate.reachability == "PRODUCTIVE" {
                productive_rows += 1;
            }
            *influence_totals.entry(influence).or_insert(0) += 1;
            out.push_str(&format!(
                "NominalGateInventory: rule {}; owner {}; locator {:016x}; reachability {}; subject {}; linked_identity_sites {}; batches {}; detached_batches {}; applied_edits {}; final_process_wide {}; final_owner_thread {}; influence {}; association REGION.\n",
                gate.rule,
                gate.owner,
                gate.locator,
                gate.reachability,
                gate.subject,
                gate.linked_sites.len(),
                linked.batches,
                linked.detached_batches,
                linked.applied_edits,
                linked.final_edits,
                linked.owner_final_edits,
                influence,
            ));
        }
        let rendered = influence_totals
            .iter()
            .map(|(class, count)| format!("{class} {count}"))
            .collect::<Vec<_>>()
            .join("; ");
        out.push_str(&format!(
            "NominalGateInventorySummary: Project {}; Rows {}; Productive {}; {}; MusicMutation none; RemovalPerformed none.\n",
            owner.project,
            NOMINAL_GATE_ROWS.len(),
            productive_rows,
            if rendered.is_empty() { "none".to_string() } else { rendered },
        ));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn final_path() -> std::path::PathBuf {
        std::env::temp_dir().join("etuschlopf-r1193-final.render.musicxml")
    }

    fn shadow_path() -> std::path::PathBuf {
        std::env::temp_dir().join("etuschlopf-r1193-shadow.render.musicxml")
    }

    fn apply_one(batch_nonce: u64, before: &str, after: &str) {
        observe_origin(batch_nonce, "synthetic.rs", 10, 20, 1);
        observe_bound(batch_nonce, 0, "synthetic.rs", 10, 20, 1);
        observe_applied_batch_transition(
            batch_nonce,
            &[0],
            "synthetic.rs",
            10,
            20,
            1,
            (before, after),
        );
    }

    fn report() -> String {
        let mut out = String::new();
        append_report(&mut out);
        out
    }

    #[test]
    fn exact_applied_state_can_continue_to_final_epoch_without_locator_recovery() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        observe_render_epoch_boundary(&path, 1, None, "B");
        let out = report();
        assert!(out.contains("CONTINUED 1; REBASED 0; RESEALED 0; UNKNOWN 0"));
        clear();
    }

    #[test]
    fn exact_in_memory_state_successor_rebases_older_receipt() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        apply_one(2, "B", "C");
        observe_render_epoch_boundary(&path, 1, None, "C");
        let out = report();
        assert!(out.contains("CONTINUED 1; REBASED 1; RESEALED 0; UNKNOWN 0"));
        clear();
    }

    #[test]
    fn observed_final_artifact_write_reseals_existing_receipt() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        observe_render_epoch_boundary(&path, 1, None, "B");
        observe_render_epoch_boundary(&path, 2, Some("B"), "C");
        let out = report();
        assert!(out.contains("CONTINUED 0; REBASED 0; RESEALED 1; UNKNOWN 0"));
        clear();
    }

    #[test]
    fn shadow_only_materialization_remains_unknown_for_final_render_epoch() {
        let final_path = final_path();
        let shadow_path = shadow_path();
        start("synthetic", Some(&final_path));
        apply_one(1, "A", "B");
        observe_render_epoch_boundary(&shadow_path, 1, None, "B");
        let out = report();
        assert!(out.contains("CONTINUED 0; REBASED 0; RESEALED 0; UNKNOWN 1"));
        clear();
    }

    #[test]
    fn identical_bytes_from_independent_producers_do_not_merge_branch_receipts() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        apply_one(2, "X", "B");
        observe_render_epoch_boundary(&path, 1, None, "B");
        let out = report();
        assert!(out.contains("CONTINUED 0; REBASED 0; RESEALED 0; UNKNOWN 2"));
        assert!(out.contains("StateProvenanceAmbiguityDrops 1"));
        clear();
    }

    #[test]
    fn selected_applied_state_is_distinguished_from_no_selection_witness() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        apply_one(2, "X", "Y");
        observe_selected_state("B");
        let out = report();
        assert!(out.contains("FINAL_RECEIPT 0; SELECTED_NO_FINAL_RECEIPT 1; NO_SELECTION_WITNESS 1"));
        clear();
    }

    #[test]
    fn final_receipt_takes_precedence_over_prior_selection_observation() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        observe_selected_state("B");
        observe_render_epoch_boundary(&path, 1, None, "B");
        let out = report();
        assert!(out.contains("FINAL_RECEIPT 1; SELECTED_NO_FINAL_RECEIPT 0; NO_SELECTION_WITNESS 0"));
        clear();
    }

    #[test]
    fn exact_non_authority_state_successor_can_preserve_receipt_for_later_selection() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        observe_exact_state_successor("B", "C");
        observe_selected_state("C");
        let out = report();
        assert!(out.contains("FINAL_RECEIPT 0; SELECTED_NO_FINAL_RECEIPT 1; NO_SELECTION_WITNESS 0"));
        clear();
    }

    #[test]
    fn explicit_selected_successor_unites_parent_and_selected_child_receipts() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        apply_one(2, "X", "C");
        observe_selected_state_successor("B", "C");
        observe_render_epoch_boundary(&path, 1, None, "C");
        let out = report();
        assert!(out.contains("CONTINUED 1; REBASED 1; RESEALED 0; UNKNOWN 0"));
        assert!(out.contains("FINAL_RECEIPT 2; SELECTED_NO_FINAL_RECEIPT 0; NO_SELECTION_WITNESS 0"));
        clear();
    }

    fn ledger_apply(state: &mut AttributionState, batch_nonce: u64, before: &str, after: &str) {
        let origin = OriginRef {
            batch_nonce,
            file: "src/synthetic.rs",
            line: 7,
            column: 3,
            edit_count: 1,
        };
        origin_on(state, origin);
        phase_on(state, origin, 0, EditState::Bound);
        applied_on(
            state,
            origin,
            &BTreeSet::from([0usize]),
            state_fingerprint(before),
            state_fingerprint(after),
        );
    }

    #[test]
    fn mirror_ledger_recovers_disposition_lost_on_detached_successor() {
        let path = final_path();
        let mut owner = fresh_state("synthetic", Some(&path));
        let mut mirror = fresh_state("synthetic", Some(&path));
        for ledger in [&mut owner, &mut mirror] {
            ledger_apply(ledger, 1, "A", "B");
        }
        // Worker-thread normalisation is seen by the mirror only.
        let mut tally = DetachedTally::default();
        tally.count(ObservationKind::Successor(state_fingerprint("B")), &mirror);
        exact_successor_on(&mut mirror, state_fingerprint("B"), state_fingerprint("C"));
        let final_identity = path_identity(&path);
        for ledger in [&mut owner, &mut mirror] {
            selected_state_on(ledger, state_fingerprint("C"));
            render_epoch_boundary_on(ledger, &final_identity, 1, None, state_fingerprint("C"));
        }
        let comparison = compare_thread_loss(&owner, &mirror, &tally.batches);
        assert_eq!(tally.state_successor_with_receipts, 1);
        assert_eq!(tally.receipts_carried, 1);
        assert_eq!(comparison.owner_no_witness, 1);
        assert_eq!(comparison.owner_no_witness_reaching_final, 1);
        assert_eq!(
            comparison.cells.get(&(DISPOSITION_NO_WITNESS, DISPOSITION_FINAL)),
            Some(&1)
        );
    }

    #[test]
    fn mirror_ledger_counts_detached_minting_without_changing_owner_ledger() {
        let path = final_path();
        let mut owner = fresh_state("synthetic", Some(&path));
        let mut mirror = fresh_state("synthetic", Some(&path));
        let mut tally = DetachedTally::default();
        tally.count(ObservationKind::Origin(9), &mirror);
        tally.count(ObservationKind::Applied(9), &mirror);
        ledger_apply(&mut mirror, 9, "P", "Q");
        let final_identity = path_identity(&path);
        for ledger in [&mut owner, &mut mirror] {
            selected_state_on(ledger, state_fingerprint("Q"));
            render_epoch_boundary_on(ledger, &final_identity, 1, None, state_fingerprint("Q"));
        }
        let comparison = compare_thread_loss(&owner, &mirror, &tally.batches);
        assert!(applied_edit_keys(&owner).is_empty());
        assert_eq!(comparison.detached_edits, 1);
        assert_eq!(comparison.detached_applied_edits, 1);
        assert_eq!(comparison.detached_applied_reaching_final, 1);
        assert_eq!(
            comparison.cells.get(&(DISPOSITION_NOT_ON_OWNER, DISPOSITION_FINAL)),
            Some(&1)
        );
    }

    #[test]
    fn mirror_ledger_equal_to_owner_ledger_reports_no_thread_locality() {
        let path = final_path();
        let mut owner = fresh_state("synthetic", Some(&path));
        let mut mirror = fresh_state("synthetic", Some(&path));
        for ledger in [&mut owner, &mut mirror] {
            ledger_apply(ledger, 3, "K", "L");
            ledger_apply(ledger, 4, "X", "Y");
            render_epoch_boundary_on(ledger, &path_identity(&path), 1, None, state_fingerprint("L"));
        }
        let comparison = compare_thread_loss(&owner, &mirror, &BTreeSet::new());
        assert_eq!(comparison.owner_no_witness, 1);
        assert_eq!(comparison.owner_no_witness_reaching_final, 0);
        assert_eq!(comparison.owner_no_witness_reaching_open_selection, 0);
        assert_eq!(comparison.detached_edits, 0);
        assert_eq!(comparison.cells.get(&(DISPOSITION_FINAL, DISPOSITION_FINAL)), Some(&1));
    }

    #[test]
    fn identity_coverage_projection_reports_static_tables_and_owner_ledger() {
        let path = final_path();
        start("synthetic", Some(&path));
        apply_one(21, "M", "N");
        observe_render_epoch_boundary(&path, 1, None, "N");
        let mut out = String::new();
        append_identity_coverage_and_thread_loss_report(&mut out);
        assert!(out.contains("DestructiveIdentityCoverage: Project synthetic; Surfaces "));
        assert!(out.contains("DestructiveIdentityThreadLossNoSelectionWitness: Project synthetic; OwnerThreadNoSelectionWitness 0"));
        assert!(out.contains("DestructiveIdentityThreadLossDisposition: owner_thread FINAL_RECEIPT; process_wide FINAL_RECEIPT; edits 1."));
        assert!(out.contains("NominalGateInventorySummary: Project synthetic; Rows "));
        clear();
    }

    #[test]
    fn final_artifact_selection_witness_excludes_shadow_path() {
        let path = final_path();
        let shadow = shadow_path();
        start("synthetic", Some(&path));
        apply_one(1, "A", "B");
        observe_selected_final_artifact_state(&shadow, "B");
        let out = report();
        assert!(out.contains("FINAL_RECEIPT 0; SELECTED_NO_FINAL_RECEIPT 0; NO_SELECTION_WITNESS 1"));

        observe_selected_final_artifact_state(&path, "B");
        let out = report();
        assert!(out.contains("FINAL_RECEIPT 0; SELECTED_NO_FINAL_RECEIPT 1; NO_SELECTION_WITNESS 0"));
        clear();
    }
}
