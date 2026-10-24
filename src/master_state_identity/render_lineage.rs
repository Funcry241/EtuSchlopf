//! Sovereign Render lineage (r1230 RenderSovereigntyClosureMaxi).
//!
//! The productive Render document is owned by `RenderSovereignState` in the
//! StructuralWriteAuthority adoption boundary. This module provides its typed
//! provenance: the closed Render origins G1-G3,G5 / S4, the Master epochs they
//! bind, the move-only chain of not yet committed owner receipts of one Render
//! transition, the byte-exact replay that verifies such a chain against its
//! real pre-basis before anything is written, and the committed Render lineage
//! `GENESIS -> (owner event | FromMaster)*`.
//!
//! Identity stays the opaque minted identity of the owner event. Fingerprints
//! bind continuity only. A Render transition is adopted only if the replay of
//! its owners' receipts from the exact pre-basis reproduces the candidate
//! bytes; otherwise the run aborts with a fatal Render sovereignty violation.
//! Files, availability flags and thread-locals are never a source of Render
//! state: a file is an artifact whose readback must equal the sovereign bytes.

use super::*;

#[cfg(test)]
#[path = "../../tests/support/render_authority/isolated_render_measurement.rs"]
mod isolated_render_measurement;
#[cfg(test)]
use isolated_render_measurement::{AtomicUsize, Mutex, count};

// ---------------------------------------------------------------------------
// Runtime closure metrics
// ---------------------------------------------------------------------------

/// The dedicated per-project Render closure metrics. Every metric is a failure
/// counter: closure requires all of them to be zero together with a complete
/// stage record and a recorded final lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RenderClosureMetric {
    LineagePending,
    AdoptionWithoutGenesis,
    ContinuityViolations,
    ReceiptCoverageFailures,
    OwnerWriteWithoutReceipt,
    CandidateCoverageFailures,
    TrialCoverageFailures,
    ArtifactReadbackMismatch,
    FileStateAuthorityReads,
    ThreadLocalStateAuthorityReads,
    UnauthorizedGenesis,
    UnauthorizedMasterSubstitution,
    HistoryLoss,
}

impl RenderClosureMetric {
    pub(crate) const ALL: [Self; 13] = [
        Self::LineagePending,
        Self::AdoptionWithoutGenesis,
        Self::ContinuityViolations,
        Self::ReceiptCoverageFailures,
        Self::OwnerWriteWithoutReceipt,
        Self::CandidateCoverageFailures,
        Self::TrialCoverageFailures,
        Self::ArtifactReadbackMismatch,
        Self::FileStateAuthorityReads,
        Self::ThreadLocalStateAuthorityReads,
        Self::UnauthorizedGenesis,
        Self::UnauthorizedMasterSubstitution,
        Self::HistoryLoss,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::LineagePending => "RenderLineagePending",
            Self::AdoptionWithoutGenesis => "RenderAdoptionWithoutGenesis",
            Self::ContinuityViolations => "RenderContinuityViolations",
            Self::ReceiptCoverageFailures => "RenderReceiptCoverageFailures",
            Self::OwnerWriteWithoutReceipt => "RenderOwnerWriteWithoutReceipt",
            Self::CandidateCoverageFailures => "RenderCandidateCoverageFailures",
            Self::TrialCoverageFailures => "RenderTrialCoverageFailures",
            Self::ArtifactReadbackMismatch => "RenderArtifactReadbackMismatch",
            Self::FileStateAuthorityReads => "RenderFileStateAuthorityReads",
            Self::ThreadLocalStateAuthorityReads => "RenderThreadLocalStateAuthorityReads",
            Self::UnauthorizedGenesis => "RenderUnauthorizedGenesis",
            Self::UnauthorizedMasterSubstitution => "RenderUnauthorizedMasterSubstitution",
            Self::HistoryLoss => "RenderHistoryLoss",
        }
    }

    fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|metric| *metric == self)
            .unwrap_or_default()
    }
}

static RENDER_CLOSURE_FAILURES: [AtomicUsize; 13] = [const { AtomicUsize::new(0) }; 13];
static RENDER_TRANSITIONS_ADOPTED: AtomicUsize = AtomicUsize::new(0);
static RENDER_RECEIPTS_COMMITTED: AtomicUsize = AtomicUsize::new(0);
static RENDER_GENESIS_ADOPTED: AtomicUsize = AtomicUsize::new(0);
static RENDER_FROM_MASTER_ADOPTED: AtomicUsize = AtomicUsize::new(0);
static RENDER_UNCHANGED_SINK_WRITES: AtomicUsize = AtomicUsize::new(0);
static RENDER_DECLINED_SINK_WRITES: AtomicUsize = AtomicUsize::new(0);
static RENDER_ARTIFACT_WITNESSES: AtomicUsize = AtomicUsize::new(0);
static RENDER_ARTIFACT_GUARD_READS: AtomicUsize = AtomicUsize::new(0);
static RENDER_UNADOPTED_DISCARDS: AtomicUsize = AtomicUsize::new(0);
static RENDER_HISTORICAL_CHECKPOINT_BASES: AtomicUsize = AtomicUsize::new(0);
/// Live, non-empty receipt chains that have not yet been verified or explicitly discarded.
/// This is a gauge, not a failure counter: a correctly consumed chain must not poison closure.
static RENDER_OPEN_RECEIPT_CHAINS: AtomicUsize = AtomicUsize::new(0);
static RENDER_STAGE_RECORD: Mutex<Vec<RenderStageBoundary>> = Mutex::new(Vec::new());
static FINAL_RENDER_LINEAGE: Mutex<Option<FinalRenderLineageSummary>> = Mutex::new(None);

/// Counts one Render closure failure without aborting (used where the failure
/// is detected outside a violation path, e.g. a dropped pending chain).
pub(crate) fn count_render_closure_failure(metric: RenderClosureMetric) {
    count(&RENDER_CLOSURE_FAILURES[metric.index()]);
}

/// Fatal Render sovereignty violation. The failure is counted and the run
/// aborts on the same fatal path as a Master sovereignty violation; there is
/// no fallback to another candidate, file or Master document.
#[track_caller]
pub(crate) fn render_sovereignty_violation(metric: RenderClosureMetric, detail: &str) -> ! {
    count_render_closure_failure(metric);
    panic!("RENDER SOVEREIGNTY VIOLATION: {}: {detail}", metric.label())
}

pub(crate) fn count_render_unchanged_sink_write() {
    count(&RENDER_UNCHANGED_SINK_WRITES);
}

pub(crate) fn count_render_declined_sink_write() {
    count(&RENDER_DECLINED_SINK_WRITES);
}

/// Ordered stage boundaries of one project's Render pass. Closure requires
/// that every boundary was reached exactly once in this order; an incomplete
/// record is not a closure proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderStageBoundary {
    InitialRender,
    RestArticulation,
    MetricBeam,
    FinalNotation,
    PdfSourceSelection,
    Release,
}

impl RenderStageBoundary {
    const ORDER: [Self; 6] = [
        Self::InitialRender,
        Self::RestArticulation,
        Self::MetricBeam,
        Self::FinalNotation,
        Self::PdfSourceSelection,
        Self::Release,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::InitialRender => "InitialRender",
            Self::RestArticulation => "RestArticulation",
            Self::MetricBeam => "MetricBeam",
            Self::FinalNotation => "FinalNotation",
            Self::PdfSourceSelection => "PdfSourceSelection",
            Self::Release => "Release",
        }
    }
}

pub(crate) fn record_render_stage_boundary(boundary: RenderStageBoundary) {
    RENDER_STAGE_RECORD
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .push(boundary);
}

fn render_stage_record_complete() -> bool {
    let record = RENDER_STAGE_RECORD
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    record.as_slice() == RenderStageBoundary::ORDER
}

/// Why a Render artifact is read. Only a witness/guard read against the
/// sovereign bytes is legal; any read that would supply Render state is a
/// fatal file-state-authority violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderArtifactReadPurpose {
    /// Readback after a sovereign write: must equal the adopted bytes.
    WitnessAgainstSovereignState,
    /// Before a path-bound owner reads the artifact itself: must equal the
    /// current sovereign bytes, so the owner's file read is the sovereign state.
    GuardAgainstWitnessedArtifact,
}

/// Compares an artifact readback with the sovereign bytes. A mismatch or a
/// missing artifact is fatal.
pub(crate) fn record_render_artifact_read(
    purpose: RenderArtifactReadPurpose,
    artifact: Option<&str>,
    sovereign: &str,
    label: &str,
) {
    match purpose {
        RenderArtifactReadPurpose::WitnessAgainstSovereignState => {
            count(&RENDER_ARTIFACT_WITNESSES)
        }
        RenderArtifactReadPurpose::GuardAgainstWitnessedArtifact => {
            count(&RENDER_ARTIFACT_GUARD_READS)
        }
    }
    if artifact != Some(sovereign) {
        render_sovereignty_violation(
            RenderClosureMetric::ArtifactReadbackMismatch,
            &format!("{label}: Render artifact bytes differ from the sovereign Render state"),
        );
    }
}

/// Resets the process-local Render measurement at the start of one project run.
pub(crate) fn reset_render_lineage_measurement() {
    for counter in RENDER_CLOSURE_FAILURES.iter().chain([
        &RENDER_TRANSITIONS_ADOPTED,
        &RENDER_RECEIPTS_COMMITTED,
        &RENDER_GENESIS_ADOPTED,
        &RENDER_FROM_MASTER_ADOPTED,
        &RENDER_UNCHANGED_SINK_WRITES,
        &RENDER_DECLINED_SINK_WRITES,
        &RENDER_ARTIFACT_WITNESSES,
        &RENDER_ARTIFACT_GUARD_READS,
        &RENDER_UNADOPTED_DISCARDS,
        &RENDER_HISTORICAL_CHECKPOINT_BASES,
        &RENDER_OPEN_RECEIPT_CHAINS,
    ]) {
        counter.store(0, Ordering::Relaxed);
    }
    RENDER_STAGE_RECORD
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clear();
    *FINAL_RENDER_LINEAGE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
}

// ---------------------------------------------------------------------------
// Closed Render origins (mirror of src/build_gate/render_origins_v1.tsv)
// ---------------------------------------------------------------------------

/// The seven closed Render origins. Nothing else may create or replace a
/// Render state from Master bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderOriginId {
    G1,
    G2,
    G3,
    G5,
    S4,
}

/// The actual typed fork point at which a Render transition requests an
/// origin. It is passed by the owning stage, never parsed from strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderOriginFork {
    PrunedPdfTrial,
    FirstInitialTrial2To8,
    FinalNotationB1,
    FinalNotationB4,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RenderOriginTuple {
    pub(crate) id: &'static str,
    pub(crate) kind: &'static str,
    pub(crate) fork_point: &'static str,
    pub(crate) owner: &'static str,
    pub(crate) reason: &'static str,
    pub(crate) source: &'static str,
    pub(crate) source_epoch: &'static str,
    pub(crate) legacy_selection: &'static str,
    pub(crate) history: &'static str,
}

const IR_ORIGIN_OWNER: &str = "src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs::run_initial_render_transaction_stage";
const FN_ORIGIN_OWNER: &str = "src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs::run_final_render_notation_authority_stage";
const EARLY_EPOCH: &str = "selected Master before initial Render trials";
const LATE_EPOCH: &str = "Master after #56 and before #65";

pub(crate) const RENDER_ORIGIN_TUPLES: [RenderOriginTuple; 5] = [
    RenderOriginTuple {
        id: "G1",
        kind: "Genesis",
        fork_point: "PrunedPdfTrial",
        owner: IR_ORIGIN_OWNER,
        reason: "RenderGenesisPrunedPdf",
        source: "best.layout_xml()",
        source_epoch: EARLY_EPOCH,
        legacy_selection: "pruned trial writes first Render state",
        history: "absent",
    },
    RenderOriginTuple {
        id: "G2",
        kind: "Genesis",
        fork_point: "FirstInitialTrial2To8",
        owner: IR_ORIGIN_OWNER,
        reason: "RenderGenesisFirstTrial",
        source: "best.layout_xml()",
        source_epoch: EARLY_EPOCH,
        legacy_selection: "first writing initial trial 2..8 after no pruned write",
        history: "absent",
    },
    RenderOriginTuple {
        id: "G3",
        kind: "Genesis",
        fork_point: "FinalNotationB1",
        owner: FN_ORIGIN_OWNER,
        reason: "RenderGenesisLateFinalNotation",
        source: "final_export_xml",
        source_epoch: LATE_EPOCH,
        legacy_selection: "!avail && !path.exists() && B1 writes",
        history: "absent",
    },
    RenderOriginTuple {
        id: "G5",
        kind: "Genesis",
        fork_point: "FinalNotationB4",
        owner: FN_ORIGIN_OWNER,
        reason: "RenderGenesisLateFinalNotation",
        source: "final_export_xml",
        source_epoch: LATE_EPOCH,
        legacy_selection: "!avail && B4 writes",
        history: "absent",
    },
    RenderOriginTuple {
        id: "S4",
        kind: "FromMaster",
        fork_point: "FinalNotationB4",
        owner: FN_ORIGIN_OWNER,
        reason: "CursorNeutralWriterWithoutRenderSource",
        source: "final_export_xml",
        source_epoch: LATE_EPOCH,
        legacy_selection: "!avail && B4 writes",
        history: "present_preserved",
    },
];

impl RenderOriginId {
    pub(crate) fn tuple(self) -> &'static RenderOriginTuple {
        let index = match self {
            Self::G1 => 0,
            Self::G2 => 1,
            Self::G3 => 2,
            Self::G5 => 3,
            Self::S4 => 4,
        };
        &RENDER_ORIGIN_TUPLES[index]
    }

    pub(crate) fn is_genesis(self) -> bool {
        self.tuple().kind == "Genesis"
    }

    fn fork(self) -> RenderOriginFork {
        match self {
            Self::G1 => RenderOriginFork::PrunedPdfTrial,
            Self::G2 => RenderOriginFork::FirstInitialTrial2To8,
            Self::G3 => RenderOriginFork::FinalNotationB1,
            Self::G5 | Self::S4 => RenderOriginFork::FinalNotationB4,
        }
    }

    fn epoch_kind(self) -> RenderMasterEpochKind {
        match self {
            Self::G1 | Self::G2 => RenderMasterEpochKind::BeforeInitialRenderTrials,
            _ => RenderMasterEpochKind::AfterCursorLaneBeforeFinalChord,
        }
    }

    /// Resolves the only origin admitted at a typed fork for the actual
    /// history state and legacy availability. `None` means no origin exists:
    /// the transition must then build on the existing sovereign state.
    pub(crate) fn admitted_at(
        fork: RenderOriginFork,
        history_present: bool,
        legacy_render_source_available: bool,
    ) -> Option<Self> {
        match (fork, history_present) {
            (RenderOriginFork::PrunedPdfTrial, false) => Some(Self::G1),
            (RenderOriginFork::FirstInitialTrial2To8, false) => Some(Self::G2),
            (RenderOriginFork::FinalNotationB1, false) if !legacy_render_source_available => {
                Some(Self::G3)
            }
            (RenderOriginFork::FinalNotationB4, false) if !legacy_render_source_available => {
                Some(Self::G5)
            }
            (RenderOriginFork::FinalNotationB4, true) if !legacy_render_source_available => {
                Some(Self::S4)
            }
            _ => None,
        }
    }

    /// Validates a concrete origin request against the closed tuple. Any
    /// deviation (wrong fork, epoch, history state or availability) is fatal.
    fn validate_request(
        self,
        fork: RenderOriginFork,
        epoch: &RenderMasterEpoch,
        history_present: bool,
        legacy_render_source_available: bool,
    ) {
        let tuple = self.tuple();
        let metric = if self.is_genesis() {
            RenderClosureMetric::UnauthorizedGenesis
        } else {
            RenderClosureMetric::UnauthorizedMasterSubstitution
        };
        if self.fork() != fork {
            render_sovereignty_violation(
                metric,
                &format!("{} requested at foreign fork {:?}", tuple.id, fork),
            );
        }
        if self.epoch_kind() != epoch.kind() {
            render_sovereignty_violation(
                metric,
                &format!("{} bound to a foreign Master epoch", tuple.id),
            );
        }
        if self.is_genesis() && history_present {
            render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedGenesis,
                &format!("{} Genesis requested on existing Render history", tuple.id),
            );
        }
        if !self.is_genesis() && !history_present {
            render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedMasterSubstitution,
                &format!("{} substitution requested without Render history", tuple.id),
            );
        }
        if Self::admitted_at(fork, history_present, legacy_render_source_available) != Some(self) {
            render_sovereignty_violation(
                metric,
                &format!(
                    "{} is not the admitted origin of its fork for the actual history/availability",
                    tuple.id
                ),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Master epochs bound by Render origins
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderMasterEpochKind {
    BeforeInitialRenderTrials,
    AfterCursorLaneBeforeFinalChord,
}

/// Opaque identity of one `RenderSovereignState` instance. It binds roots to
/// the state that issued them; it grants no write authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderSovereignIdentity(MintedIdentity);

impl RenderSovereignIdentity {
    pub(crate) fn mint() -> Self {
        Self(MintedIdentity::mint_fresh())
    }
}

/// Frozen identity of the Master epoch captured by an Origin root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderMasterEpochBinding {
    kind: RenderMasterEpochKind,
    document_fingerprint: u64,
    master_head_fingerprint: u64,
    master_committed_events: usize,
}

/// The exact Master bytes a Render origin may consume, bound to the sovereign
/// Master lineage head (and, for the late epoch, to the #56 owner event).
#[derive(Debug, Clone)]
pub(crate) struct RenderMasterEpoch {
    kind: RenderMasterEpochKind,
    document: String,
    master_head_fingerprint: u64,
    master_committed_events: usize,
}

impl RenderMasterEpoch {
    /// The selected Master before the initial Render trials (G1/G2).
    pub(crate) fn before_initial_render_trials(master: &MasterLineage, document: &str) -> Self {
        if !master.lineage_head_binds(document) {
            render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedGenesis,
                "early Render epoch: document is not the sovereign Master head",
            );
        }
        Self {
            kind: RenderMasterEpochKind::BeforeInitialRenderTrials,
            document: document.to_string(),
            master_head_fingerprint: fingerprint(document),
            master_committed_events: master.events.len(),
        }
    }

    /// Bind the unchanged sovereign Master after the successful temporal check
    /// and before #65. A privately constructed proof for other bytes is refused.
    pub(crate) fn after_cursor_lane_before_final_chord(
        master: &MasterLineage,
        master_document: &str,
        checked: &crate::cursor_lane_temporal_check::CheckedCursorTemporalInput<'_>,
        final_export_xml: &str,
    ) -> Self {
        if !master.lineage_head_binds(master_document)
            || master_document != final_export_xml
            || !checked.binds(master_document)
            || !checked.binds(final_export_xml)
        {
            render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedMasterSubstitution,
                "late Render epoch: export must be the unchanged checked sovereign Master",
            );
        }
        Self {
            kind: RenderMasterEpochKind::AfterCursorLaneBeforeFinalChord,
            document: final_export_xml.to_string(),
            master_head_fingerprint: fingerprint(master_document),
            master_committed_events: master.events.len(),
        }
    }

    pub(crate) fn document(&self) -> &str {
        &self.document
    }

    pub(crate) fn kind(&self) -> RenderMasterEpochKind {
        self.kind
    }

    pub(crate) fn binding(&self) -> RenderMasterEpochBinding {
        RenderMasterEpochBinding {
            kind: self.kind,
            document_fingerprint: fingerprint(&self.document),
            master_head_fingerprint: self.master_head_fingerprint,
            master_committed_events: self.master_committed_events,
        }
    }
}

// ---------------------------------------------------------------------------
// Not yet committed owner receipts of one Render transition
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StepPlacement {
    /// The receipt's pre-state is the whole current document.
    Document,
    /// The receipt's pre-state is `doc[start..end]` of the current document;
    /// its post replaces exactly that range.
    Fragment { start: usize, end: usize },
    /// Inner owner whose output is a generated fragment consumed by the next
    /// advancing owner. Its pre may be located in the current document.
    Generated { located: Option<usize> },
    /// The #2 cursor-neutral event whose output only derives the
    /// DirectionRelocation instruction; it does not advance the document.
    InstructionSource,
    /// A document owner whose pre->post equals the composition of the inner
    /// owner steps pushed since `from_step`.
    Enclosing { from_step: usize },
}

impl StepPlacement {
    fn advances(self) -> bool {
        matches!(self, Self::Document | Self::Fragment { .. })
    }

    fn label(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Fragment { .. } => "fragment",
            Self::Generated { .. } => "generated",
            Self::InstructionSource => "instruction_source",
            Self::Enclosing { .. } => "enclosing",
        }
    }
}

#[derive(Debug)]
struct RenderStep {
    receipt: TransformationReceipt,
    placement: StepPlacement,
    pre_bytes: Option<String>,
    post_bytes: Option<String>,
}

/// Why an unadopted receipt chain is explicitly closed without commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderDiscardReason {
    /// A trial or candidate lost its selection; nothing was written.
    LosingCandidate,
    /// A read-only probe/audit executed owners on a copy that is never adopted.
    ReadOnlyProbe,
}

#[derive(Debug, Default)]
struct PendingRenderObligation {
    active: bool,
}

impl PendingRenderObligation {
    fn activate(&mut self) {
        if !self.active {
            count(&RENDER_OPEN_RECEIPT_CHAINS);
            self.active = true;
        }
    }

    fn close_successfully(&mut self) {
        if self.active {
            RENDER_OPEN_RECEIPT_CHAINS.fetch_sub(1, Ordering::Relaxed);
            self.active = false;
        }
    }
}

impl Drop for PendingRenderObligation {
    fn drop(&mut self) {
        if self.active {
            RENDER_OPEN_RECEIPT_CHAINS.fetch_sub(1, Ordering::Relaxed);
            count_render_closure_failure(RenderClosureMetric::LineagePending);
            self.active = false;
        }
    }
}

/// Ordered, move-only chain of the owner receipts of one not yet committed
/// Render transition. Its live obligation moves into `VerifiedRenderTransition`
/// during verification and closes only on commit or explicit discard.
#[derive(Debug, Default)]
pub(crate) struct RenderTransitionReceipts {
    steps: Vec<RenderStep>,
    obligation: PendingRenderObligation,
}

impl RenderTransitionReceipts {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.steps.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    fn push(
        &mut self,
        receipt: Option<TransformationReceipt>,
        placement: StepPlacement,
        pre_bytes: Option<&str>,
        post_bytes: Option<&str>,
    ) {
        if let Some(receipt) = receipt {
            self.obligation.activate();
            self.steps.push(RenderStep {
                receipt,
                placement,
                pre_bytes: pre_bytes.map(str::to_string),
                post_bytes: post_bytes.map(str::to_string),
            });
        }
    }

    /// EXACT owner on the whole current document.
    pub(crate) fn push_exact(&mut self, receipt: Option<ExactTransformationReceipt>) {
        self.push(receipt.map(TransformationReceipt::Exact), StepPlacement::Document, None, None);
    }

    /// EXACT owner on `doc[start..end]` of the current document.
    pub(crate) fn push_exact_fragment(
        &mut self,
        receipt: Option<ExactTransformationReceipt>,
        start: usize,
        end: usize,
    ) {
        self.push(
            receipt.map(TransformationReceipt::Exact),
            StepPlacement::Fragment { start, end },
            None,
            None,
        );
    }

    /// BLOCK owner on the whole current document with its post bytes.
    pub(crate) fn push_block(&mut self, receipt: impl Into<CapturedBlockReceiptInput>, post: &str) {
        let input=receipt.into();
        // Beam observation is not a Render adoption capability.
        self.push(
            input.receipt.map(TransformationReceipt::Block),
            StepPlacement::Document,
            None,
            Some(post),
        );
    }

    /// BLOCK owner on `doc[start..end]` with its post fragment.
    pub(crate) fn push_block_fragment(
        &mut self,
        receipt: Option<BlockTransformationReceipt>,
        start: usize,
        end: usize,
        post_fragment: &str,
    ) {
        self.push(
            receipt.map(TransformationReceipt::Block),
            StepPlacement::Fragment { start, end },
            None,
            Some(post_fragment),
        );
    }

    /// PRESERVING owner on the whole current document. A path-bound receipt
    /// carries its own realized post bytes; otherwise `post` is used.
    pub(crate) fn push_preserving(
        &mut self,
        receipt: Option<PreservingTransformationReceipt>,
        post: &str,
    ) {
        let Some(receipt) = receipt else {
            return;
        };
        let (receipt, realized) = receipt.split_realized_post_document();
        let post = realized.unwrap_or_else(|| post.to_string());
        self.push(
            Some(TransformationReceipt::Preserving(receipt)),
            StepPlacement::Document,
            None,
            Some(&post),
        );
    }

    /// Inner EXACT owner producing a generated fragment that the next
    /// advancing owner consumes.
    #[cfg(test)]
    pub(crate) fn push_generated_exact(
        &mut self,
        receipt: Option<ExactTransformationReceipt>,
        pre_fragment: &str,
        post_fragment: &str,
        located: Option<usize>,
    ) {
        self.push(
            receipt.map(TransformationReceipt::Exact),
            StepPlacement::Generated { located },
            Some(pre_fragment),
            Some(post_fragment),
        );
    }

    /// Inner BLOCK owner producing a generated fragment that the next
    /// advancing owner consumes.
    pub(crate) fn push_generated_block(
        &mut self,
        receipt: Option<BlockTransformationReceipt>,
        pre_fragment: &str,
        post_fragment: &str,
        located: Option<usize>,
    ) {
        self.push(
            receipt.map(TransformationReceipt::Block),
            StepPlacement::Generated { located },
            Some(pre_fragment),
            Some(post_fragment),
        );
    }



    /// Outer BLOCK owner whose pre->post is exactly the composition of the
    /// inner owner steps pushed since `from_step` (its real nested execution).
    #[cfg(test)]
    pub(crate) fn push_enclosing_block(
        &mut self,
        receipt: Option<BlockTransformationReceipt>,
        from_step: usize,
    ) {
        self.push(
            receipt.map(TransformationReceipt::Block),
            StepPlacement::Enclosing { from_step },
            None,
            None,
        );
    }

    /// The #2 cursor-neutral event whose output only derives the
    /// DirectionRelocation instruction (its own pre/post bytes are bound).
    pub(crate) fn push_instruction_source_exact(
        &mut self,
        receipt: Option<ExactTransformationReceipt>,
        source: &str,
        post: &str,
    ) {
        self.push(
            receipt.map(TransformationReceipt::Exact),
            StepPlacement::InstructionSource,
            Some(source),
            Some(post),
        );
    }

    /// Appends another chain in execution order.
    pub(crate) fn append(&mut self, mut other: RenderTransitionReceipts) {
        let offset = self.steps.len();
        for mut step in other.steps.drain(..) {
            if let StepPlacement::Enclosing { from_step } = step.placement {
                step.placement = StepPlacement::Enclosing {
                    from_step: from_step + offset,
                };
            }
            self.steps.push(step);
        }
        if other.obligation.active {
            if self.obligation.active {
                // Two live chains become one live chain.
                other.obligation.close_successfully();
            } else {
                // Transfer the one live obligation without changing the gauge.
                self.obligation.active = true;
                other.obligation.active = false;
            }
        }
    }

    /// Explicitly closes a chain that is not adopted (losing candidate or
    /// read-only probe). Nothing is committed; the chain is closed, not pending.
    pub(crate) fn discard_unadopted(mut self, _reason: RenderDiscardReason) {
        if !self.steps.is_empty() {
            count(&RENDER_UNADOPTED_DISCARDS);
        }
        self.steps.clear();
        self.obligation.close_successfully();
    }

    fn take_steps(mut self) -> (Vec<RenderStep>, PendingRenderObligation) {
        let steps = std::mem::take(&mut self.steps);
        let obligation = std::mem::take(&mut self.obligation);
        (steps, obligation)
    }
}

// ---------------------------------------------------------------------------
// Byte-exact replay of one transition
// ---------------------------------------------------------------------------

fn realize_receipt_post(receipt: &TransformationReceipt, pre: &str, supplied: Option<&str>) -> Option<String> {
    let post = match receipt {
        TransformationReceipt::Exact(exact) => exact.realize_post_document(pre)?,
        _ => supplied?.to_string(),
    };
    (fingerprint(pre) == receipt.pre_fingerprint() && fingerprint(&post) == receipt.post_fingerprint())
        .then_some(post)
}

/// Continuity trace of one verified step against the evolving document.
#[derive(Debug, Clone, Copy)]
struct StepTrace {
    placement: StepPlacement,
    doc_pre: u64,
    doc_post: u64,
}

#[derive(Debug)]
struct PendingGenerated {
    step_index: usize,
    pre: String,
    post: String,
    located: Option<usize>,
}

fn generated_consumer_edit_range(
    document: &str,
    consumer_index: usize,
    consumer: &RenderStep,
    generated: &PendingGenerated,
) -> Result<(usize, usize), String> {
    let TransformationReceipt::Exact(receipt) = &consumer.receipt else {
        return Err(format!(
            "step {}: generated fragment from step {} reaches an advancing owner without EXACT replacement geometry",
            consumer_index, generated.step_index
        ));
    };
    let base = match consumer.placement {
        StepPlacement::Document => 0,
        StepPlacement::Fragment { start, end } => {
            if document.get(start..end).is_none() {
                return Err(format!(
                    "step {consumer_index}: fragment range outside the current Render state while binding generated step {}",
                    generated.step_index
                ));
            }
            start
        }
        _ => {
            return Err(format!(
                "step {consumer_index}: generated fragment from step {} is not followed by an advancing replacement owner",
                generated.step_index
            ));
        }
    };

    let mut matches = Vec::new();
    for edit in &receipt.edits {
        let Some(start) = base.checked_add(edit.start) else {
            continue;
        };
        let Some(end) = base.checked_add(edit.end) else {
            continue;
        };
        if edit.replacement != generated.post
            || document.get(start..end) != Some(generated.pre.as_str())
        {
            continue;
        }
        if let Some(located) = generated.located
            && (start != located || end != located.saturating_add(generated.pre.len()))
        {
            continue;
        }
        matches.push((start, end));
    }

    match matches.as_slice() {
        [range] => Ok(*range),
        [] => Err(format!(
            "step {}: generated fragment from step {} is not consumed by this owner's bound replacement geometry",
            consumer_index, generated.step_index
        )),
        _ => Err(format!(
            "step {}: generated fragment from step {} has ambiguous consumer replacement geometry",
            consumer_index, generated.step_index
        )),
    }
}

fn verify_pending_generated_consumer(
    document: &str,
    consumer_index: usize,
    consumer: &RenderStep,
    pending: &[PendingGenerated],
) -> Result<(), String> {
    let mut claimed_ranges = std::collections::BTreeSet::new();
    for generated in pending {
        let range = generated_consumer_edit_range(document, consumer_index, consumer, generated)?;
        if !claimed_ranges.insert(range) {
            return Err(format!(
                "step {consumer_index}: multiple generated fragments claim the same consumer replacement range {}..{}",
                range.0, range.1
            ));
        }
    }
    Ok(())
}

fn replay_steps(base: &str, steps: &[RenderStep]) -> Result<(String, Vec<StepTrace>), String> {
    let mut document = base.to_string();
    let mut traces = Vec::with_capacity(steps.len());
    let mut document_before_step = Vec::<u64>::with_capacity(steps.len());
    let mut pending_generated = Vec::<PendingGenerated>::new();
    for (index, step) in steps.iter().enumerate() {
        let doc_pre = fingerprint(&document);
        document_before_step.push(doc_pre);
        if step.placement.advances() && !pending_generated.is_empty() {
            verify_pending_generated_consumer(&document, index, step, &pending_generated)?;
        }
        match step.placement {
            StepPlacement::Document => {
                document = realize_receipt_post(&step.receipt, &document, step.post_bytes.as_deref())
                    .ok_or_else(|| format!("step {index}: document receipt does not replay on the current Render state"))?;
            }
            StepPlacement::Fragment { start, end } => {
                let fragment = document
                    .get(start..end)
                    .ok_or_else(|| format!("step {index}: fragment range outside the current Render state"))?
                    .to_string();
                let post = realize_receipt_post(&step.receipt, &fragment, step.post_bytes.as_deref())
                    .ok_or_else(|| format!("step {index}: fragment receipt does not replay on its located pre-fragment"))?;
                // Verification constructs an immutable replay value; it does not
                // mutate a productive document or acquire writer authority.
                document = [&document[..start], &post, &document[end..]].concat();
            }
            StepPlacement::Generated { located } => {
                let pre = step
                    .pre_bytes
                    .as_deref()
                    .ok_or_else(|| format!("step {index}: generated owner without pre bytes"))?;
                if let Some(start) = located
                    && document.get(start..start + pre.len()) != Some(pre)
                {
                    return Err(format!("step {index}: generated owner pre-fragment is not located in the current Render state"));
                }
                let post = realize_receipt_post(&step.receipt, pre, step.post_bytes.as_deref())
                    .ok_or_else(|| format!("step {index}: generated owner receipt does not bind its fragment"))?;
                pending_generated.push(PendingGenerated {
                    step_index: index,
                    pre: pre.to_string(),
                    post,
                    located,
                });
            }
            StepPlacement::InstructionSource => {
                let TransformationReceipt::Exact(exact) = &step.receipt else {
                    return Err(format!(
                        "step {index}: instruction source must carry the cursor-neutral EXACT receipt"
                    ));
                };
                if !exact.minted_by::<crate::structural_write_authority::CursorNeutralRenderExactMint>() {
                    return Err(format!(
                        "step {index}: instruction source was not minted by the cursor-neutral owner"
                    ));
                }
                let pre = step
                    .pre_bytes
                    .as_deref()
                    .ok_or_else(|| format!("step {index}: instruction source without bytes"))?;
                realize_receipt_post(&step.receipt, pre, step.post_bytes.as_deref())
                    .ok_or_else(|| format!("step {index}: instruction source receipt does not bind its bytes"))?;
            }
            StepPlacement::Enclosing { from_step } => {
                if !matches!(step.receipt, TransformationReceipt::Block(_)) {
                    return Err(format!("step {index}: only a BLOCK owner may enclose inner owner steps"));
                }
                if let Some(generated) = pending_generated
                    .iter()
                    .find(|generated| generated.step_index >= from_step)
                {
                    return Err(format!(
                        "step {index}: enclosing owner closes over generated step {} before its consuming owner",
                        generated.step_index
                    ));
                }
                let Some(inner_pre) = document_before_step.get(from_step).copied() else {
                    return Err(format!("step {index}: enclosing range is empty or out of order"));
                };
                if from_step >= index
                    || step.receipt.pre_fingerprint() != inner_pre
                    || step.receipt.post_fingerprint() != fingerprint(&document)
                {
                    return Err(format!("step {index}: enclosing owner does not equal the composition of its inner owners"));
                }
            }
        }
        if step.placement.advances() {
            pending_generated.clear();
        }
        traces.push(StepTrace {
            placement: step.placement,
            doc_pre,
            doc_post: fingerprint(&document),
        });
    }
    if let Some(generated) = pending_generated.first() {
        return Err(format!(
            "step {}: generated fragment has no consuming owner",
            generated.step_index
        ));
    }
    Ok((document, traces))
}

// ---------------------------------------------------------------------------
// Committed Render lineage
// ---------------------------------------------------------------------------

/// Capability of the Render adoption boundary to start and extend a Render
/// lineage. Only `RenderAdoptionCommitBoundary` implements it.
pub(crate) trait RenderLineageCommitBoundary: sealed::MintOwner {}

impl sealed::MintOwner for crate::structural_write_authority::RenderAdoptionCommitBoundary {}
impl RenderLineageCommitBoundary for crate::structural_write_authority::RenderAdoptionCommitBoundary {}

#[derive(Debug, Clone)]
struct RenderGenesis {
    identity: MintedIdentity,
    origin: RenderOriginId,
    epoch_kind: RenderMasterEpochKind,
    master_head_fingerprint: u64,
    master_committed_events: usize,
    post_fingerprint: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommittedPlacement {
    Document,
    Fragment,
    Generated,
    InstructionSource,
    Enclosing,
    HistoricalCheckpoint58,
}

impl CommittedPlacement {
    fn advances(self) -> bool {
        matches!(self, Self::Document | Self::Fragment | Self::HistoricalCheckpoint58)
    }
}

#[derive(Debug, Clone)]
enum RenderLineageElement {
    Owner {
        event: CommittedTransformationEvent,
        placement: CommittedPlacement,
        doc_pre: u64,
        doc_post: u64,
    },
    FromMaster {
        identity: MintedIdentity,
        origin: RenderOriginId,
        master_head_fingerprint: u64,
        doc_pre: u64,
        doc_post: u64,
    },
}

/// `GENESIS(G1,G2,G3,G5) -> (owner event | FromMaster S4)*`. Provenance only.
#[derive(Debug, Clone)]
pub(crate) struct RenderLineage {
    genesis: RenderGenesis,
    elements: Vec<RenderLineageElement>,
}

/// Opaque binding of one committed lineage position (used to bind a trial root
/// or the last #58 commit to an exact state, never to authorize a write).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderLineagePosition {
    genesis: MintedIdentity,
    elements: usize,
    head: u64,
}

impl RenderLineage {
    fn head(&self) -> u64 {
        self.elements.last().map_or(self.genesis.post_fingerprint, |element| match element {
            RenderLineageElement::Owner { doc_post, .. }
            | RenderLineageElement::FromMaster { doc_post, .. } => *doc_post,
        })
    }

    pub(crate) fn position(&self) -> RenderLineagePosition {
        RenderLineagePosition {
            genesis: self.genesis.identity,
            elements: self.elements.len(),
            head: self.head(),
        }
    }

    pub(crate) fn head_binds(&self, document: &str) -> bool {
        self.head() == fingerprint(document)
    }

    #[cfg(test)]
    pub(crate) fn committed_element_count(&self) -> usize {
        self.elements.len()
    }

    pub(crate) fn origin(&self) -> RenderOriginId {
        self.genesis.origin
    }

    /// Mechanical continuity: genesis post = first pre, every advancing
    /// element continues the head, non-advancing elements sit on the head,
    /// identities are unique and the head binds `current`.
    pub(crate) fn lineage_continuity_holds(&self, current: &str) -> bool {
        let mut head = self.genesis.post_fingerprint;
        let mut seen = std::collections::BTreeSet::new();
        seen.insert(self.genesis.identity.opaque_value());
        for element in &self.elements {
            match element {
                RenderLineageElement::Owner {
                    event,
                    placement,
                    doc_pre,
                    doc_post,
                } => {
                    if *doc_pre != head
                        || !seen.insert(event.identity.opaque_value())
                        || (!placement.advances() && doc_post != doc_pre)
                    {
                        return false;
                    }
                    head = *doc_post;
                }
                RenderLineageElement::FromMaster {
                    identity,
                    doc_pre,
                    doc_post,
                    ..
                } => {
                    if *doc_pre != head || !seen.insert(identity.opaque_value()) {
                        return false;
                    }
                    head = *doc_post;
                }
            }
        }
        head == fingerprint(current)
    }

    fn contains_identity(&self, identity: MintedIdentity) -> bool {
        self.genesis.identity == identity
            || self.elements.iter().any(|element| match element {
                RenderLineageElement::Owner { event, .. } => event.identity == identity,
                RenderLineageElement::FromMaster { identity: own, .. } => *own == identity,
            })
    }

    fn summary(&self, current: &str) -> FinalRenderLineageSummary {
        let mut summary = FinalRenderLineageSummary {
            origin: Some(self.origin()),
            epoch_kind: Some(self.genesis.epoch_kind),
            genesis_master_events: self.genesis.master_committed_events,
            genesis_master_head: self.genesis.master_head_fingerprint != 0,
            continuity_holds: self.lineage_continuity_holds(current),
            ..FinalRenderLineageSummary::default()
        };
        for element in &self.elements {
            match element {
                RenderLineageElement::Owner { event, placement, .. } => {
                    *summary.placements.entry(placement_label(*placement)).or_default() += 1;
                    match event.contract {
                        CommittedContract::Block { row, .. } => {
                            summary.block_receipts += 1;
                            *summary.block_rows.entry(row).or_default() += 1;
                        }
                        CommittedContract::Exact { row, edits } => {
                            summary.exact_receipts += 1;
                            summary.exact_edits += edits;
                            *summary.exact_rows.entry(row).or_default() += 1;
                        }
                        CommittedContract::Preserving { exclusion } => {
                            summary.preserving_receipts += 1;
                            *summary.preserving_contracts.entry(exclusion).or_default() += 1;
                        }
                        CommittedContract::Additive { exclusion, .. } => {
                            summary.additive_receipts += 1;
                            *summary.additive_contracts.entry(exclusion).or_default() += 1;
                        }
                    }
                }
                RenderLineageElement::FromMaster { origin, master_head_fingerprint, .. } => {
                    summary.from_master_heads.push(*master_head_fingerprint);
                    *summary.from_master.entry(origin.tuple().id).or_default() += 1;
                }
            }
        }
        summary
    }
}

fn placement_label(placement: CommittedPlacement) -> &'static str {
    match placement {
        CommittedPlacement::Document => "document",
        CommittedPlacement::Fragment => "fragment",
        CommittedPlacement::Generated => "generated",
        CommittedPlacement::InstructionSource => "instruction_source",
        CommittedPlacement::Enclosing => "enclosing",
        CommittedPlacement::HistoricalCheckpoint58 => "historical_checkpoint_58",
    }
}

/// The pre-basis of one Render transition.
#[derive(Debug, Clone, Copy)]
pub(crate) enum RenderTransitionBasis<'a> {
    /// The current sovereign Render state (history present).
    State,
    /// A closed Genesis origin on its bound Master epoch (no history).
    Genesis {
        origin: RenderOriginId,
        fork: RenderOriginFork,
        epoch: &'a RenderMasterEpoch,
        legacy_render_source_available: bool,
    },
    /// A closed history-preserving FromMaster substitution (S4).
    Substitution {
        origin: RenderOriginId,
        fork: RenderOriginFork,
        epoch: &'a RenderMasterEpoch,
        legacy_render_source_available: bool,
    },
    /// The last #58 committed Render state, admitted only for the
    /// DirectionRelocation event.
    HistoricalCheckpoint58,
}

/// A Render transition whose receipt chain was replayed byte-exactly from its
/// real pre-basis to the candidate. It is move-only and committed at most once.
#[derive(Debug)]
pub(crate) struct VerifiedRenderTransition {
    origin: Option<(RenderOriginId, RenderMasterEpochKind, u64, usize)>,
    from_master: bool,
    historical_checkpoint_58: bool,
    base_fingerprint: u64,
    steps: Vec<RenderStep>,
    obligation: PendingRenderObligation,
    traces: Vec<StepTrace>,
    candidate: String,
}

impl VerifiedRenderTransition {
    /// Verifies a chain against its basis. `current` is the existing sovereign
    /// Render document (None = no history). Every failure is fatal.
    pub(crate) fn verify<B: RenderLineageCommitBoundary>(
        _boundary: &B,
        basis: RenderTransitionBasis<'_>,
        current: Option<&str>,
        historical_checkpoint_58: Option<&str>,
        receipts: RenderTransitionReceipts,
        candidate: &str,
        label: &str,
    ) -> Self {
        let history_present = current.is_some();
        let (base, origin, from_master, historical) = match basis {
            RenderTransitionBasis::State => {
                let Some(current) = current else {
                    render_sovereignty_violation(
                        RenderClosureMetric::AdoptionWithoutGenesis,
                        &format!("{label}: transition on the Render state without any Render history"),
                    );
                };
                (current.to_string(), None, false, false)
            }
            RenderTransitionBasis::Genesis {
                origin,
                fork,
                epoch,
                legacy_render_source_available,
            } => {
                if !origin.is_genesis() {
                    render_sovereignty_violation(
                        RenderClosureMetric::UnauthorizedGenesis,
                        &format!("{label}: {} is not a Genesis origin", origin.tuple().id),
                    );
                }
                origin.validate_request(fork, epoch, history_present, legacy_render_source_available);
                (
                    epoch.document.clone(),
                    Some((origin, epoch.kind, epoch.master_head_fingerprint, epoch.master_committed_events)),
                    false,
                    false,
                )
            }
            RenderTransitionBasis::Substitution {
                origin,
                fork,
                epoch,
                legacy_render_source_available,
            } => {
                if origin.is_genesis() {
                    render_sovereignty_violation(
                        RenderClosureMetric::UnauthorizedMasterSubstitution,
                        &format!("{label}: {} is not a substitution origin", origin.tuple().id),
                    );
                }
                origin.validate_request(fork, epoch, history_present, legacy_render_source_available);
                (
                    epoch.document.clone(),
                    Some((origin, epoch.kind, epoch.master_head_fingerprint, epoch.master_committed_events)),
                    true,
                    false,
                )
            }
            RenderTransitionBasis::HistoricalCheckpoint58 => {
                let (Some(current), Some(historical)) = (current, historical_checkpoint_58) else {
                    render_sovereignty_violation(
                        RenderClosureMetric::ContinuityViolations,
                        &format!("{label}: DirectionRelocation without a committed #58 Render state"),
                    );
                };
                if current != historical {
                    render_sovereignty_violation(
                        RenderClosureMetric::ContinuityViolations,
                        &format!("{label}: DirectionRelocation basis is not the last #58 committed Render state"),
                    );
                }
                (historical.to_string(), None, false, true)
            }
        };
        let (steps, obligation) = receipts.take_steps();
        if historical
            && !steps.iter().any(|step| {
                matches!(&step.receipt, TransformationReceipt::Exact(exact)
                    if exact.minted_by::<crate::structural_write_authority::DirectionRelocationExactMint>())
                    && step.placement == StepPlacement::Document
            })
        {
            render_sovereignty_violation(
                RenderClosureMetric::ContinuityViolations,
                &format!("{label}: the historical #58 basis is admitted only for the DirectionRelocation event"),
            );
        }
        let (realized, traces) = match replay_steps(&base, &steps) {
            Ok(result) => result,
            Err(reason) => {
                let metric = if steps.is_empty() {
                    RenderClosureMetric::OwnerWriteWithoutReceipt
                } else {
                    RenderClosureMetric::ReceiptCoverageFailures
                };
                render_sovereignty_violation(metric, &format!("{label}: {reason}"))
            }
        };
        if realized != candidate {
            let metric = if steps.iter().any(|step| step.placement.advances()) {
                RenderClosureMetric::ReceiptCoverageFailures
            } else {
                RenderClosureMetric::OwnerWriteWithoutReceipt
            };
            render_sovereignty_violation(
                metric,
                &format!("{label}: owner receipts do not reproduce the Render candidate bytes"),
            );
        }
        Self {
            origin,
            from_master,
            historical_checkpoint_58: historical,
            base_fingerprint: fingerprint(&base),
            steps,
            obligation,
            traces,
            candidate: candidate.to_string(),
        }
    }

    pub(crate) fn candidate(&self) -> &str {
        &self.candidate
    }

    pub(crate) fn requires_lineage_commit(&self, current: Option<&str>) -> bool {
        self.origin.is_some()
            || !self.steps.is_empty()
            || current != Some(self.candidate.as_str())
    }

    /// Explicitly closes a fully verified transition that lost a later
    /// non-authority publication/probe gate. Its receipts were proven, but no
    /// productive artifact/state was adopted, so no lineage event is committed.
    pub(crate) fn discard_verified(mut self, _reason: RenderDiscardReason) {
        if !self.steps.is_empty() {
            count(&RENDER_UNADOPTED_DISCARDS);
        }
        self.steps.clear();
        self.obligation.close_successfully();
    }

    /// Appends the verified #58 sink-owner event (its own ClefReadability
    /// edits) to this transition. The receipt must start at the verified
    /// candidate; the effective written bytes are realized from it.
    pub(crate) fn extend_with_sink_owner<B: RenderLineageCommitBoundary>(
        mut self,
        _boundary: &B,
        sink_receipt: Option<ExactTransformationReceipt>,
        label: &str,
    ) -> Self {
        let Some(receipt) = sink_receipt else {
            return self;
        };
        if !receipt.minted_by::<crate::structural_write_authority::RenderRow58ExactMint>() {
            render_sovereignty_violation(
                RenderClosureMetric::ReceiptCoverageFailures,
                &format!("{label}: sink owner receipt is not the #58 checkpoint owner"),
            );
        }
        let step = RenderStep {
            receipt: TransformationReceipt::Exact(receipt),
            placement: StepPlacement::Document,
            pre_bytes: None,
            post_bytes: None,
        };
        let doc_pre = fingerprint(&self.candidate);
        let Some(effective) = realize_receipt_post(&step.receipt, &self.candidate, None) else {
            render_sovereignty_violation(
                RenderClosureMetric::ReceiptCoverageFailures,
                &format!("{label}: #58 sink receipt does not replay on the verified candidate"),
            );
        };
        self.traces.push(StepTrace {
            placement: StepPlacement::Document,
            doc_pre,
            doc_post: fingerprint(&effective),
        });
        self.obligation.activate();
        self.steps.push(step);
        self.candidate = effective;
        self
    }

    /// Commits the verified transition. Without a lineage (no history) the
    /// transition must carry its Genesis origin; a substitution keeps the
    /// existing history and records an explicit FromMaster element.
    pub(crate) fn commit<B: RenderLineageCommitBoundary>(
        mut self,
        _boundary: &B,
        lineage: Option<RenderLineage>,
        label: &str,
    ) -> (RenderLineage, String) {
        // Contract v4 A7 (r1287): the SSC adoption verdict of every step precedes any
        // lineage change; a refusal is a Render sovereignty violation without partial commit.
        for step in &self.steps {
            if let Err(refusal) = crate::ssc_evidence_proof::ssc_adoption_verdict(
                &super::SSC_ORIGIN_ACCESS,
                step.receipt.adoption_view(),
                crate::ssc_evidence_proof::SscAdoptionDomain::Render,
            ) {
                render_sovereignty_violation(
                    RenderClosureMetric::ReceiptCoverageFailures,
                    &format!("{label}: {refusal}"),
                );
            }
        }
        let mut lineage = match (lineage, self.origin) {
            (None, Some((origin, epoch_kind, master_head, master_events))) if origin.is_genesis() => {
                count(&RENDER_GENESIS_ADOPTED);
                RenderLineage {
                    genesis: RenderGenesis {
                        identity: MintedIdentity::mint_fresh(),
                        origin,
                        epoch_kind,
                        master_head_fingerprint: master_head,
                        master_committed_events: master_events,
                        post_fingerprint: self.base_fingerprint,
                    },
                    elements: Vec::new(),
                }
            }
            (None, _) => render_sovereignty_violation(
                RenderClosureMetric::AdoptionWithoutGenesis,
                &format!("{label}: first Render state without a closed Genesis origin"),
            ),
            (Some(_), Some((origin, ..))) if origin.is_genesis() => render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedGenesis,
                &format!("{label}: Genesis on existing Render history"),
            ),
            (Some(mut lineage), Some((origin, _, master_head, _))) => {
                if !self.from_master {
                    render_sovereignty_violation(
                        RenderClosureMetric::UnauthorizedMasterSubstitution,
                        &format!("{label}: Master bytes replacing Render history outside S4"),
                    );
                }
                count(&RENDER_FROM_MASTER_ADOPTED);
                let doc_pre = lineage.head();
                lineage.elements.push(RenderLineageElement::FromMaster {
                    identity: MintedIdentity::mint_fresh(),
                    origin,
                    master_head_fingerprint: master_head,
                    doc_pre,
                    doc_post: self.base_fingerprint,
                });
                lineage
            }
            (Some(lineage), None) => {
                if lineage.head() != self.base_fingerprint {
                    render_sovereignty_violation(
                        RenderClosureMetric::ContinuityViolations,
                        &format!("{label}: transition basis is not the committed Render head"),
                    );
                }
                lineage
            }
        };
        let historical = self.historical_checkpoint_58;
        for (step, trace) in self.steps.into_iter().zip(self.traces) {
            if lineage.contains_identity(step.receipt.identity()) {
                render_sovereignty_violation(
                    RenderClosureMetric::ContinuityViolations,
                    &format!("{label}: receipt identity already committed"),
                );
            }
            let head = lineage.head();
            let (placement, doc_pre, doc_post) = match trace.placement {
                StepPlacement::Document | StepPlacement::Fragment { .. } => {
                    let placement = match (trace.placement, historical) {
                        (StepPlacement::Document, true)
                            if matches!(&step.receipt, TransformationReceipt::Exact(exact)
                                if exact.minted_by::<crate::structural_write_authority::DirectionRelocationExactMint>()) =>
                        {
                            count(&RENDER_HISTORICAL_CHECKPOINT_BASES);
                            CommittedPlacement::HistoricalCheckpoint58
                        }
                        (StepPlacement::Document, _) => CommittedPlacement::Document,
                        _ => CommittedPlacement::Fragment,
                    };
                    (placement, trace.doc_pre, trace.doc_post)
                }
                StepPlacement::Generated { .. } => (CommittedPlacement::Generated, head, head),
                StepPlacement::InstructionSource => (CommittedPlacement::InstructionSource, head, head),
                StepPlacement::Enclosing { .. } => (CommittedPlacement::Enclosing, head, head),
            };
            if doc_pre != head {
                render_sovereignty_violation(
                    RenderClosureMetric::ContinuityViolations,
                    &format!("{label}: {} receipt does not continue the Render head", step.placement.label()),
                );
            }
            count(&RENDER_RECEIPTS_COMMITTED);
            lineage.elements.push(RenderLineageElement::Owner {
                event: step.receipt.commit(crate::ssc_evidence_proof::SscAdoptionDomain::Render),
                placement,
                doc_pre,
                doc_post,
            });
        }
        if !lineage.head_binds(&self.candidate) {
            render_sovereignty_violation(
                RenderClosureMetric::ContinuityViolations,
                &format!("{label}: committed Render lineage does not bind the adopted document"),
            );
        }
        self.obligation.close_successfully();
        count(&RENDER_TRANSITIONS_ADOPTED);
        (lineage, self.candidate)
    }
}

// ---------------------------------------------------------------------------
// Final projection and closure
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct FinalRenderLineageSummary {
    render_state_present: bool,
    origin: Option<RenderOriginId>,
    epoch_kind: Option<RenderMasterEpochKind>,
    genesis_master_events: usize,
    genesis_master_head: bool,
    from_master_heads: Vec<u64>,
    continuity_holds: bool,
    artifact_witnessed: bool,
    block_receipts: usize,
    exact_receipts: usize,
    exact_edits: usize,
    preserving_receipts: usize,
    additive_receipts: usize,
    block_rows: BTreeMap<usize, usize>,
    exact_rows: BTreeMap<usize, usize>,
    preserving_contracts: BTreeMap<&'static str, usize>,
    additive_contracts: BTreeMap<&'static str, usize>,
    placements: BTreeMap<&'static str, usize>,
    from_master: BTreeMap<&'static str, usize>,
}

/// Result of the per-project Render closure evaluation.
#[derive(Debug, Clone)]
pub(crate) struct RenderClosureVerdict {
    pub(crate) closed: bool,
    pub(crate) failures: Vec<(RenderClosureMetric, usize)>,
    pub(crate) detail: String,
}

fn render_closure_failures_snapshot() -> Vec<(RenderClosureMetric, usize)> {
    let open = RENDER_OPEN_RECEIPT_CHAINS.load(Ordering::Relaxed);
    RenderClosureMetric::ALL
        .iter()
        .map(|metric| {
            let recorded = RENDER_CLOSURE_FAILURES[metric.index()].load(Ordering::Relaxed);
            let live = if *metric == RenderClosureMetric::LineagePending { open } else { 0 };
            (*metric, recorded + live)
        })
        .collect()
}

/// Records the released Render state (or its explicit absence) and evaluates
/// the closure gate. The verdict is closed only if the stage record is
/// complete, the released lineage is continuous and witnessed, every
/// failure counter is zero, and no live receipt obligation remains.
pub(crate) fn record_final_render_lineage(
    released: Option<(&RenderLineage, &str)>,
    artifact_witnessed: bool,
) -> RenderClosureVerdict {
    let identities = released.map(|(lineage, _)| lineage.elements.iter().filter_map(|element| match element { RenderLineageElement::Owner { event, .. } => Some(event.identity), RenderLineageElement::FromMaster { .. } => None }).collect::<Vec<_>>());
    crate::ssc_evidence_proof::retain_released_render_evidence(&super::SSC_ORIGIN_ACCESS, identities.as_deref());
    let mut summary = match released {
        Some((lineage, document)) => {
            let mut summary = lineage.summary(document);
            summary.render_state_present = true;
            summary
        }
        None => FinalRenderLineageSummary {
            continuity_holds: true,
            ..FinalRenderLineageSummary::default()
        },
    };
    summary.artifact_witnessed = artifact_witnessed;
    if !summary.continuity_holds {
        count_render_closure_failure(RenderClosureMetric::ContinuityViolations);
    }
    if !artifact_witnessed {
        count_render_closure_failure(RenderClosureMetric::ArtifactReadbackMismatch);
    }
    let failures = render_closure_failures_snapshot();
    let stages_complete = render_stage_record_complete();
    let closed = stages_complete && failures.iter().all(|(_, value)| *value == 0);
    let detail = failures
        .iter()
        .filter(|(_, value)| *value > 0)
        .map(|(metric, value)| format!("{} {value}", metric.label()))
        .chain((!stages_complete).then(|| "RenderStageRecord incomplete".to_string()))
        .collect::<Vec<_>>()
        .join("; ");
    *FINAL_RENDER_LINEAGE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(summary);
    RenderClosureVerdict {
        closed,
        failures,
        detail,
    }
}

fn format_static_counts(contracts: &BTreeMap<&'static str, usize>) -> String {
    format_contract_counts(contracts)
}

/// Appends the Render lineage projection and the closure line. Counts only.
pub(crate) fn append_render_lineage_report(out: &mut String) {
    let load = |counter: &AtomicUsize| counter.load(Ordering::Relaxed);
    out.push_str("RenderLineageContract: the productive Render document is owned by RenderSovereignState; its first state exists only through a closed Genesis G1,G2,G3,G5 bound to its typed fork and Master epoch, a Master->Render replacement only through the history-preserving S4; every changed Render transition is adopted only after the byte-exact replay of its real owners' receipts from the exact pre-basis reproduces the candidate; files are artifacts whose readback must equal the sovereign bytes; availability flags and thread-locals select legacy branches only and never supply Render state; class-F fallbacks are fatal.\n");
    out.push_str(&format!(
        "RenderLineageMints: transitions_adopted {}; receipts_committed {}; genesis {}; from_master {}; historical_checkpoint58_bases {}; unchanged_sink_writes {}; declined_sink_writes {}; artifact_witnesses {}; artifact_guard_reads {}; unadopted_chains_discarded {}.\n",
        load(&RENDER_TRANSITIONS_ADOPTED),
        load(&RENDER_RECEIPTS_COMMITTED),
        load(&RENDER_GENESIS_ADOPTED),
        load(&RENDER_FROM_MASTER_ADOPTED),
        load(&RENDER_HISTORICAL_CHECKPOINT_BASES),
        load(&RENDER_UNCHANGED_SINK_WRITES),
        load(&RENDER_DECLINED_SINK_WRITES),
        load(&RENDER_ARTIFACT_WITNESSES),
        load(&RENDER_ARTIFACT_GUARD_READS),
        load(&RENDER_UNADOPTED_DISCARDS),
    ));
    let stages = RENDER_STAGE_RECORD
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .iter()
        .map(|stage| stage.label())
        .collect::<Vec<_>>()
        .join(">");
    let slot = FINAL_RENDER_LINEAGE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(summary) = slot.as_ref() else {
        out.push_str("RenderLineageReleased: not recorded.\n");
        out.push_str("RenderSovereigntyClosure: RenderSovereigntyClosed: false; final Render lineage not recorded.\n");
        return;
    };
    if summary.render_state_present {
        out.push_str(&format!(
            "RenderLineageReleased: genesis {} ({}); master_epoch {}; master_events_at_genesis {}; master_head_recorded {}; from_master_heads {}; from_master {}; block_receipts {} ({}); exact_receipts {} ({}); exact_edits {}; preserving_receipts {} ({}); additive_receipts {} ({}); placements {}; continuity {}; artifact_witness {}.\n",
            summary.origin.map_or("none", |origin| origin.tuple().id),
            summary.origin.map_or("none", |origin| origin.tuple().reason),
            match summary.epoch_kind {
                Some(RenderMasterEpochKind::BeforeInitialRenderTrials) => "before_initial_render_trials",
                Some(RenderMasterEpochKind::AfterCursorLaneBeforeFinalChord) => "after_56_before_65",
                None => "none",
            },
            summary.genesis_master_events,
            summary.genesis_master_head,
            summary.from_master_heads.len(),
            format_static_counts(&summary.from_master),
            summary.block_receipts,
            format_row_counts(&summary.block_rows),
            summary.exact_receipts,
            format_row_counts(&summary.exact_rows),
            summary.exact_edits,
            summary.preserving_receipts,
            format_static_counts(&summary.preserving_contracts),
            summary.additive_receipts,
            format_static_counts(&summary.additive_contracts),
            format_static_counts(&summary.placements),
            if summary.continuity_holds { "holds" } else { "violated" },
            if summary.artifact_witnessed { "matches" } else { "mismatch" },
        ));
    } else {
        out.push_str(&format!(
            "RenderLineageReleased: no Render state (explicit absence; Master is the PDF source); artifact_absent_witness {}.\n",
            if summary.artifact_witnessed { "matches" } else { "mismatch" },
        ));
    }
    let failures = render_closure_failures_snapshot();
    let stages_complete = render_stage_record_complete();
    let closed = stages_complete && failures.iter().all(|(_, value)| *value == 0);
    out.push_str(&format!(
        "RenderSovereigntyClosure: RenderSovereigntyClosed: {closed}; RenderStageRecord {} ({}); {}.\n",
        if stages_complete { "complete" } else { "incomplete" },
        if stages.is_empty() { "none".to_string() } else { stages },
        failures
            .iter()
            .map(|(metric, value)| format!("{}: {value}", metric.label()))
            .collect::<Vec<_>>()
            .join("; "),
    ));
    if closed {
        out.push_str("RENDER SOVEREIGNTY CLOSED\n");
    }
}

#[cfg(test)]
#[path = "../structural_write_authority/render_lineage_receipt_tests.rs"]
mod tests;
