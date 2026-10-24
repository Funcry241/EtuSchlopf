//! r296 read-only constructive execution-plan witness foundation.
//!
//! This module turns the r292 attack/hold/release projection into physical key obligations and
//! provides a bounded exhaustive hand/finger assignment search plus an independent witness replay.
//! r618 binds the previously unbound same-key adapter on the real green r615 line. r619 closes one
//! remaining score-demand omission without inventing a capability threshold: coincident same-key
//! attacks may still share one physical slot, but every later attack on the same MIDI that reuses
//! the same hand/finger slot requires explicit SameKeyTechniqueSignature capability evidence,
//! whether the prior abstract obligation still overlaps or has already released. Missing static,
//! transition, or same-key evidence
//! is MODEL_INSUFFICIENT, never an implicit rejection or support. Staff and voice remain provenance
//! only. The productive score still binds no pianist capability model and makes no human claim.
//! r621 additionally binds the future-relevant latest same-key attack history into search-state
//! deduplication so post-release rearticulation alternatives with different slot histories are not
//! collapsed merely because their currently active surface is identical.

use crate::audit_only::pianistic_configuration_constraint_kernel::{
    assess_static_configuration, configuration_signature, CapabilityEntry, Finger,
    FingerKeyAssignment, Hand, KeySurface, KeyboardGeometry, PianistCapabilityEnvelope,
    StaticCapabilityDecision, StaticConfigurationRequest, StaticConfigurationSignature,
};
use crate::audit_only::pianistic_execution_score::{self, PianisticExecutionEvent, PianisticExecutionScore, ProjectionStatus};
use crate::sha256;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-EXECUTION-PLAN-WITNESS-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticExecutionPlanWitness";
pub const SCHEMA: &str = "pianistic-execution-plan-witness-v2";
pub const INPUT_AUTHORITY: &str = "PianisticExecutionScoreAuthority+PianisticConfigurationConstraintKernel";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";
const DEFAULT_STATE_BUDGET: usize = 250_000;

const HANDS: [Hand; 2] = [Hand::Left, Hand::Right];
const FINGERS: [Finger; 5] = [
    Finger::Thumb,
    Finger::Index,
    Finger::Middle,
    Finger::Ring,
    Finger::Little,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionPlanStatus {
    WitnessFound,
    NoRealizationUnderModel,
    ModelInsufficient,
    ResourceLimit,
    NotEvaluated,
}

impl ExecutionPlanStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::WitnessFound => "REALIZATION_WITNESS_FOUND",
            Self::NoRealizationUnderModel => "NO_REALIZATION_UNDER_MODEL",
            Self::ModelInsufficient => "MODEL_INSUFFICIENT",
            Self::ResourceLimit => "RESOURCE_LIMIT",
            Self::NotEvaluated => "NOT_EVALUATED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct FingerSlot {
    hand: Hand,
    finger: Finger,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PhysicalKeyObligation {
    pub(crate) id: usize,
    pub(crate) midi: i32,
    pub(crate) attack_tick: i64,
    pub(crate) release_tick: i64,
    pub(crate) attack_time_us_floor: u64,
    pub(crate) release_time_us_floor: u64,
    pub(crate) source_event_indices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DemandTopology {
    pub(crate) obligations: Vec<PhysicalKeyObligation>,
    pub(crate) source_execution_events: usize,
    pub(crate) coalesced_exact_duplicate_events: usize,
    pub(crate) boundary_count: usize,
    pub(crate) attack_boundary_count: usize,
    pub(crate) maximum_simultaneous_attacks: usize,
    pub(crate) maximum_concurrent_key_obligations: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TransitionSignature {
    pub(crate) hand: Hand,
    pub(crate) from: StaticConfigurationSignature,
    pub(crate) to: StaticConfigurationSignature,
    pub(crate) lowest_key_delta_semitones: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SameKeyTechniqueSignature {
    pub(crate) hand: Hand,
    pub(crate) finger: Finger,
    pub(crate) surface: KeySurface,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransitionCapabilityEntry {
    signature: TransitionSignature,
    decision: StaticCapabilityDecision,
    minimum_transition_time_us: u64,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransitionEvidence {
    decision: StaticCapabilityDecision,
    minimum_transition_time_us: u64,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SameKeyCapabilityEntry {
    signature: SameKeyTechniqueSignature,
    decision: StaticCapabilityDecision,
    minimum_rearticulation_time_us: u64,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SameKeyEvidence {
    decision: StaticCapabilityDecision,
    minimum_rearticulation_time_us: u64,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PianistExecutionCapabilityModel {
    model_id: String,
    provenance: String,
    static_envelope: PianistCapabilityEnvelope,
    transitions: BTreeMap<TransitionSignature, TransitionEvidence>,
    same_key_techniques: BTreeMap<SameKeyTechniqueSignature, SameKeyEvidence>,
}

impl PianistExecutionCapabilityModel {
    fn from_entries(
        model_id: impl Into<String>,
        provenance: impl Into<String>,
        static_envelope: PianistCapabilityEnvelope,
        transitions: Vec<TransitionCapabilityEntry>,
        same_key_techniques: Vec<SameKeyCapabilityEntry>,
    ) -> Result<Self, String> {
        let model_id = model_id.into();
        let provenance = provenance.into();
        if model_id.trim().is_empty() {
            return Err("execution-capability-model-id-empty".to_string());
        }
        if provenance.trim().is_empty() {
            return Err("execution-capability-provenance-empty".to_string());
        }
        let mut indexed = BTreeMap::new();
        for entry in transitions {
            if entry.decision == StaticCapabilityDecision::Unmodeled {
                return Err("explicit-unmodeled-transition-entry-forbidden".to_string());
            }
            if entry.evidence_id.trim().is_empty() {
                return Err("transition-evidence-id-empty".to_string());
            }
            let evidence = TransitionEvidence {
                decision: entry.decision,
                minimum_transition_time_us: entry.minimum_transition_time_us,
                evidence_id: entry.evidence_id,
            };
            if let Some(previous) = indexed.insert(entry.signature, evidence.clone())
                && previous != evidence {
                    return Err("conflicting-transition-capability-entry".to_string());
                }
        }
        let mut indexed_same_key = BTreeMap::new();
        for entry in same_key_techniques {
            if entry.decision == StaticCapabilityDecision::Unmodeled {
                return Err("explicit-unmodeled-same-key-entry-forbidden".to_string());
            }
            if entry.evidence_id.trim().is_empty() {
                return Err("same-key-evidence-id-empty".to_string());
            }
            let evidence = SameKeyEvidence {
                decision: entry.decision,
                minimum_rearticulation_time_us: entry.minimum_rearticulation_time_us,
                evidence_id: entry.evidence_id,
            };
            if let Some(previous) = indexed_same_key.insert(entry.signature, evidence.clone())
                && previous != evidence
            {
                return Err("conflicting-same-key-capability-entry".to_string());
            }
        }
        Ok(Self {
            model_id,
            provenance,
            static_envelope,
            transitions: indexed,
            same_key_techniques: indexed_same_key,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlanAssignment {
    obligation_id: usize,
    hand: Hand,
    finger: Finger,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PianisticExecutionPlan {
    score_fingerprint_sha256: String,
    capability_model_id: String,
    assignments: Vec<PlanAssignment>,
    physical_key_obligations: usize,
    source_execution_events: usize,
    witness_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WitnessReplayStatus {
    Verified,
    Rejected,
    ModelInsufficient,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WitnessReplay {
    status: WitnessReplayStatus,
    reason: String,
    checked_boundaries: usize,
    checked_obligations: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchOutcome {
    status: ExecutionPlanStatus,
    reason: String,
    explored_states: usize,
    static_unmodeled_branches: usize,
    transition_unmodeled_branches: usize,
    same_key_unmodeled_branches: usize,
    plan: Option<PianisticExecutionPlan>,
    replay: Option<WitnessReplay>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoundarySlice {
    pub(crate) tick: i64,
    pub(crate) time_us_floor: u64,
    pub(crate) active: Vec<usize>,
    pub(crate) attacking: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandHistory {
    last_nonempty: Option<StaticConfigurationRequest>,
    empty_since_us: Option<u64>,
}

impl HandHistory {
    fn initial() -> Self {
        Self {
            last_nonempty: None,
            empty_since_us: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchNode {
    active_slots: BTreeMap<usize, FingerSlot>,
    all_assignments: Vec<Option<FingerSlot>>,
    left_history: HandHistory,
    right_history: HandHistory,
}

impl SearchNode {
    fn initial(obligation_count: usize) -> Self {
        Self {
            active_slots: BTreeMap::new(),
            all_assignments: vec![None; obligation_count],
            left_history: HandHistory::initial(),
            right_history: HandHistory::initial(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SearchCounters {
    explored_states: usize,
    static_unmodeled_branches: usize,
    transition_unmodeled_branches: usize,
    same_key_unmodeled_branches: usize,
    resource_limit: bool,
}

impl SearchCounters {
    fn new() -> Self {
        Self {
            explored_states: 0,
            static_unmodeled_branches: 0,
            transition_unmodeled_branches: 0,
            same_key_unmodeled_branches: 0,
            resource_limit: false,
        }
    }
}

struct CandidateValidationContext<'a> {
    boundary: &'a BoundarySlice,
    previous_boundary_time: Option<u64>,
    topology: &'a DemandTopology,
    geometry: KeyboardGeometry,
    model: &'a PianistExecutionCapabilityModel,
}

pub(crate) fn build_demand_topology(score: &PianisticExecutionScore) -> Result<DemandTopology, String> {
    if score.status != ProjectionStatus::Complete {
        return Err(format!("execution-score-not-complete:{}", score.reason));
    }
    let mut grouped = BTreeMap::<(i32, i64, i64, u64, u64), Vec<usize>>::new();
    for (index, event) in score.execution_events.iter().enumerate() {
        validate_execution_event(event, index)?;
        grouped
            .entry((
                event.midi,
                event.attack_tick,
                event.release_tick,
                event.attack_time_us_floor,
                event.release_time_us_floor,
            ))
            .or_default()
            .push(index);
    }

    let mut obligations = grouped
        .into_iter()
        .map(
            |((midi, attack_tick, release_tick, attack_time, release_time), source_event_indices)| {
                PhysicalKeyObligation {
                    id: 0,
                    midi,
                    attack_tick,
                    release_tick,
                    attack_time_us_floor: attack_time,
                    release_time_us_floor: release_time,
                    source_event_indices,
                }
            },
        )
        .collect::<Vec<_>>();
    obligations.sort_by_key(|item| (item.attack_tick, item.midi, item.release_tick));
    for (id, obligation) in obligations.iter_mut().enumerate() {
        obligation.id = id;
    }

    let source_execution_events = score.execution_events.len();
    let represented_source_events = obligations
        .iter()
        .map(|item| item.source_event_indices.len())
        .sum::<usize>();
    if represented_source_events != source_execution_events
        || obligations
            .iter()
            .any(|item| item.source_event_indices.is_empty())
    {
        return Err("physical-obligation-source-coverage-mismatch".to_string());
    }
    let coalesced_exact_duplicate_events = source_execution_events.saturating_sub(obligations.len());
    let boundaries = build_boundaries(&obligations)?;
    let attack_boundary_count = boundaries
        .iter()
        .filter(|boundary| !boundary.attacking.is_empty())
        .count();
    let maximum_simultaneous_attacks = boundaries
        .iter()
        .map(|boundary| boundary.attacking.len())
        .max()
        .unwrap_or(0);
    let maximum_concurrent_key_obligations = boundaries
        .iter()
        .map(|boundary| boundary.active.len())
        .max()
        .unwrap_or(0);

    Ok(DemandTopology {
        obligations,
        source_execution_events,
        coalesced_exact_duplicate_events,
        boundary_count: boundaries.len(),
        attack_boundary_count,
        maximum_simultaneous_attacks,
        maximum_concurrent_key_obligations,
    })
}

fn validate_execution_event(event: &PianisticExecutionEvent, index: usize) -> Result<(), String> {
    if event.attack_tick < 0 || event.release_tick <= event.attack_tick {
        return Err(format!("invalid-execution-event-ticks:index-{index}"));
    }
    if event.release_time_us_floor <= event.attack_time_us_floor {
        return Err(format!("invalid-execution-event-physical-time:index-{index}"));
    }
    if !(21..=108).contains(&event.midi) {
        return Err(format!("execution-event-outside-keyboard:index-{index}:midi-{}", event.midi));
    }
    Ok(())
}

pub(crate) fn build_boundaries(obligations: &[PhysicalKeyObligation]) -> Result<Vec<BoundarySlice>, String> {
    let mut tick_to_time = BTreeMap::<i64, u64>::new();
    for obligation in obligations {
        bind_boundary_time(
            &mut tick_to_time,
            obligation.attack_tick,
            obligation.attack_time_us_floor,
        )?;
        bind_boundary_time(
            &mut tick_to_time,
            obligation.release_tick,
            obligation.release_time_us_floor,
        )?;
    }
    let mut boundaries = Vec::with_capacity(tick_to_time.len());
    for (tick, time_us_floor) in tick_to_time {
        let active = obligations
            .iter()
            .filter(|item| item.attack_tick <= tick && tick < item.release_tick)
            .map(|item| item.id)
            .collect::<Vec<_>>();
        let attacking = obligations
            .iter()
            .filter(|item| item.attack_tick == tick)
            .map(|item| item.id)
            .collect::<Vec<_>>();
        boundaries.push(BoundarySlice {
            tick,
            time_us_floor,
            active,
            attacking,
        });
    }
    Ok(boundaries)
}

fn bind_boundary_time(
    tick_to_time: &mut BTreeMap<i64, u64>,
    tick: i64,
    time_us_floor: u64,
) -> Result<(), String> {
    if let Some(previous) = tick_to_time.insert(tick, time_us_floor)
        && previous != time_us_floor {
            return Err(format!("conflicting-physical-time-for-tick:{tick}"));
        }
    Ok(())
}

pub(crate) fn transition_signature(
    geometry: KeyboardGeometry,
    from: &StaticConfigurationRequest,
    to: &StaticConfigurationRequest,
) -> Option<TransitionSignature> {
    if from.hand != to.hand || from.assignments.is_empty() || to.assignments.is_empty() {
        return None;
    }
    let from_signature = configuration_signature(geometry, from)?;
    let to_signature = configuration_signature(geometry, to)?;
    let from_low = from.assignments.iter().map(|item| item.midi).min()?;
    let to_low = to.assignments.iter().map(|item| item.midi).min()?;
    Some(TransitionSignature {
        hand: from.hand,
        from: from_signature,
        to: to_signature,
        lowest_key_delta_semitones: to_low.checked_sub(from_low)?,
    })
}

fn assess_transition(
    geometry: KeyboardGeometry,
    model: &PianistExecutionCapabilityModel,
    from: &StaticConfigurationRequest,
    to: &StaticConfigurationRequest,
    available_time_us: u64,
) -> (StaticCapabilityDecision, String, Option<String>) {
    if from == to {
        return (
            StaticCapabilityDecision::Supported,
            "identical-absolute-hand-configuration".to_string(),
            None,
        );
    }
    let Some(signature) = transition_signature(geometry, from, to) else {
        return (
            StaticCapabilityDecision::Unmodeled,
            "transition-signature-unavailable".to_string(),
            None,
        );
    };
    let Some(evidence) = model.transitions.get(&signature) else {
        return (
            StaticCapabilityDecision::Unmodeled,
            "transition-not-covered-by-model".to_string(),
            None,
        );
    };
    if evidence.decision == StaticCapabilityDecision::Rejected {
        return (
            StaticCapabilityDecision::Rejected,
            "explicit-transition-rejection".to_string(),
            Some(evidence.evidence_id.clone()),
        );
    }
    if available_time_us < evidence.minimum_transition_time_us {
        return (
            StaticCapabilityDecision::Rejected,
            format!(
                "transition-time-below-explicit-minimum:{}<{}",
                available_time_us, evidence.minimum_transition_time_us
            ),
            Some(evidence.evidence_id.clone()),
        );
    }
    (
        StaticCapabilityDecision::Supported,
        "explicit-transition-capability-entry".to_string(),
        Some(evidence.evidence_id.clone()),
    )
}

fn same_key_signature(
    geometry: KeyboardGeometry,
    slot: FingerSlot,
    midi: i32,
) -> Option<SameKeyTechniqueSignature> {
    Some(SameKeyTechniqueSignature {
        hand: slot.hand,
        finger: slot.finger,
        surface: geometry.key(midi)?.surface,
    })
}

fn assess_same_key_rearticulation(
    geometry: KeyboardGeometry,
    model: &PianistExecutionCapabilityModel,
    slot: FingerSlot,
    midi: i32,
    available_time_us: u64,
) -> (StaticCapabilityDecision, String, Option<String>) {
    let Some(signature) = same_key_signature(geometry, slot, midi) else {
        return (
            StaticCapabilityDecision::Unmodeled,
            "same-key-technique-signature-unavailable".to_string(),
            None,
        );
    };
    let Some(evidence) = model.same_key_techniques.get(&signature) else {
        return (
            StaticCapabilityDecision::Unmodeled,
            "same-key-technique-not-covered-by-model".to_string(),
            None,
        );
    };
    if evidence.decision == StaticCapabilityDecision::Rejected {
        return (
            StaticCapabilityDecision::Rejected,
            "explicit-same-key-technique-rejection".to_string(),
            Some(evidence.evidence_id.clone()),
        );
    }
    if available_time_us < evidence.minimum_rearticulation_time_us {
        return (
            StaticCapabilityDecision::Rejected,
            format!(
                "same-key-rearticulation-time-below-explicit-minimum:{}<{}",
                available_time_us, evidence.minimum_rearticulation_time_us
            ),
            Some(evidence.evidence_id.clone()),
        );
    }
    (
        StaticCapabilityDecision::Supported,
        "explicit-same-key-technique-capability-entry".to_string(),
        Some(evidence.evidence_id.clone()),
    )
}

fn latest_prior_same_key_attack_time_for_slot(
    assignments: &[Option<FingerSlot>],
    obligations: &[PhysicalKeyObligation],
    current: &PhysicalKeyObligation,
    slot: FingerSlot,
) -> Option<u64> {
    obligations
        .iter()
        .filter(|prior| {
            prior.id != current.id
                && prior.midi == current.midi
                && prior.attack_tick < current.attack_tick
                && assignments.get(prior.id).copied().flatten() == Some(slot)
        })
        .map(|prior| prior.attack_time_us_floor)
        .max()
}

fn request_for_hand(
    hand: Hand,
    active_slots: &BTreeMap<usize, FingerSlot>,
    obligations: &[PhysicalKeyObligation],
) -> StaticConfigurationRequest {
    let assignments = active_slots
        .iter()
        .filter_map(|(obligation_id, slot)| {
            if slot.hand != hand {
                return None;
            }
            let obligation = obligations.get(*obligation_id)?;
            Some(FingerKeyAssignment {
                finger: slot.finger,
                midi: obligation.midi,
            })
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    StaticConfigurationRequest { hand, assignments }
}

fn available_slots(node: &SearchNode) -> Vec<FingerSlot> {
    let occupied = node.active_slots.values().copied().collect::<BTreeSet<_>>();
    HANDS
        .iter()
        .flat_map(|hand| {
            FINGERS.iter().filter_map(|finger| {
                let slot = FingerSlot {
                    hand: *hand,
                    finger: *finger,
                };
                if occupied.contains(&slot) {
                    None
                } else {
                    Some(slot)
                }
            })
        })
        .collect()
}

fn candidate_slots_for_attack(
    node: &SearchNode,
    obligation_id: usize,
    obligations: &[PhysicalKeyObligation],
) -> Vec<FingerSlot> {
    let Some(obligation) = obligations.get(obligation_id) else {
        return Vec::new();
    };
    let same_key_slots = node
        .active_slots
        .iter()
        .filter_map(|(active_id, slot)| {
            obligations
                .get(*active_id)
                .filter(|active| active.midi == obligation.midi)
                .map(|_| *slot)
        })
        .collect::<BTreeSet<_>>();
    if same_key_slots.len() > 1 {
        return Vec::new();
    }
    same_key_slots
        .into_iter()
        .next()
        .map_or_else(|| available_slots(node), |slot| vec![slot])
}

fn trim_released(node: &mut SearchNode, active: &BTreeSet<usize>) {
    node.active_slots.retain(|id, _| active.contains(id));
}

fn direct_boundary_reuses_occupied_finger_on_different_key(
    previous: &SearchNode,
    candidate: &SearchNode,
    topology: &DemandTopology,
) -> bool {
    let mut previous_by_slot = BTreeMap::<FingerSlot, i32>::new();
    for (obligation_id, slot) in &previous.active_slots {
        let Some(obligation) = topology.obligations.get(*obligation_id) else {
            return true;
        };
        previous_by_slot.insert(*slot, obligation.midi);
    }
    for (obligation_id, slot) in &candidate.active_slots {
        let Some(previous_midi) = previous_by_slot.get(slot) else {
            continue;
        };
        let Some(obligation) = topology.obligations.get(*obligation_id) else {
            return true;
        };
        if *previous_midi != obligation.midi {
            return true;
        }
    }
    false
}

fn assign_new_obligations(
    node: &SearchNode,
    attacking: &[usize],
    obligations: &[PhysicalKeyObligation],
    position: usize,
    candidates: &mut Vec<SearchNode>,
    budget: usize,
    counters: &mut SearchCounters,
) {
    if counters.resource_limit {
        return;
    }
    if position == attacking.len() {
        counters.explored_states = counters.explored_states.saturating_add(1);
        if counters.explored_states > budget {
            counters.resource_limit = true;
            return;
        }
        candidates.push(node.clone());
        return;
    }
    let obligation_id = attacking[position];
    let slots = candidate_slots_for_attack(node, obligation_id, obligations);
    if slots.is_empty() {
        counters.static_unmodeled_branches = counters.static_unmodeled_branches.saturating_add(1);
        return;
    }
    for slot in slots {
        let mut next = node.clone();
        next.active_slots.insert(obligation_id, slot);
        if let Some(cell) = next.all_assignments.get_mut(obligation_id) {
            *cell = Some(slot);
        }
        assign_new_obligations(
            &next,
            attacking,
            obligations,
            position + 1,
            candidates,
            budget,
            counters,
        );
        if counters.resource_limit {
            return;
        }
    }
}

fn hand_history_mut(node: &mut SearchNode, hand: Hand) -> &mut HandHistory {
    match hand {
        Hand::Left => &mut node.left_history,
        Hand::Right => &mut node.right_history,
    }
}

fn hand_history(node: &SearchNode, hand: Hand) -> &HandHistory {
    match hand {
        Hand::Left => &node.left_history,
        Hand::Right => &node.right_history,
    }
}

fn validate_candidate_state(
    previous: Option<&SearchNode>,
    candidate: &mut SearchNode,
    context: &CandidateValidationContext<'_>,
    counters: &mut SearchCounters,
) -> bool {
    let boundary = context.boundary;
    let previous_boundary_time = context.previous_boundary_time;
    let topology = context.topology;
    let geometry = context.geometry;
    let model = context.model;
    if previous.is_some_and(|node| {
        direct_boundary_reuses_occupied_finger_on_different_key(node, candidate, topology)
    }) {
        return false;
    }

    for obligation_id in &boundary.attacking {
        let Some(obligation) = topology.obligations.get(*obligation_id) else {
            return false;
        };
        let Some(slot) = candidate.active_slots.get(obligation_id).copied() else {
            return false;
        };
        let Some(prior_attack_time) = latest_prior_same_key_attack_time_for_slot(
            &candidate.all_assignments,
            &topology.obligations,
            obligation,
            slot,
        ) else {
            continue;
        };
        let available = obligation.attack_time_us_floor.saturating_sub(prior_attack_time);
        let (decision, _, _) =
            assess_same_key_rearticulation(geometry, model, slot, obligation.midi, available);
        match decision {
            StaticCapabilityDecision::Supported => {}
            StaticCapabilityDecision::Rejected => return false,
            StaticCapabilityDecision::Unmodeled => {
                counters.same_key_unmodeled_branches =
                    counters.same_key_unmodeled_branches.saturating_add(1);
                return false;
            }
        }
    }

    for hand in HANDS {
        let current_request = request_for_hand(hand, &candidate.active_slots, &topology.obligations);
        let static_assessment = assess_static_configuration(
            geometry,
            Some(&model.static_envelope),
            &current_request,
        );
        match static_assessment.decision {
            StaticCapabilityDecision::Supported => {}
            StaticCapabilityDecision::Rejected => return false,
            StaticCapabilityDecision::Unmodeled => {
                counters.static_unmodeled_branches = counters.static_unmodeled_branches.saturating_add(1);
                return false;
            }
        }

        let previous_request = previous
            .map(|node| request_for_hand(hand, &node.active_slots, &topology.obligations))
            .unwrap_or_else(|| StaticConfigurationRequest {
                hand,
                assignments: Vec::new(),
            });
        let previous_history = previous
            .map(|node| hand_history(node, hand).clone())
            .unwrap_or_else(HandHistory::initial);

        let mut next_history = previous_history.clone();
        if current_request.assignments.is_empty() {
            if !previous_request.assignments.is_empty() {
                next_history.last_nonempty = Some(previous_request);
                next_history.empty_since_us = Some(boundary.time_us_floor);
            }
            *hand_history_mut(candidate, hand) = next_history;
            continue;
        }

        let transition_from_and_available = if !previous_request.assignments.is_empty() {
            let available = previous_boundary_time
                .and_then(|time| boundary.time_us_floor.checked_sub(time))
                .unwrap_or(0);
            Some((previous_request, available))
        } else if let (Some(last_nonempty), Some(empty_since)) = (
            previous_history.last_nonempty,
            previous_history.empty_since_us,
        ) {
            let available = boundary.time_us_floor.saturating_sub(empty_since);
            Some((last_nonempty, available))
        } else {
            None
        };

        if let Some((from_request, available)) = transition_from_and_available {
            let (decision, _, _) = assess_transition(
                geometry,
                model,
                &from_request,
                &current_request,
                available,
            );
            match decision {
                StaticCapabilityDecision::Supported => {}
                StaticCapabilityDecision::Rejected => return false,
                StaticCapabilityDecision::Unmodeled => {
                    counters.transition_unmodeled_branches = counters
                        .transition_unmodeled_branches
                        .saturating_add(1);
                    return false;
                }
            }
        }
        next_history.last_nonempty = Some(current_request);
        next_history.empty_since_us = None;
        *hand_history_mut(candidate, hand) = next_history;
    }
    true
}

fn canonical_node_key(node: &SearchNode, obligations: &[PhysicalKeyObligation]) -> String {
    let mut out = String::new();
    for (id, slot) in &node.active_slots {
        let _ = write!(out, "{id}:{:?}:{:?};", slot.hand, slot.finger);
    }
    for hand in HANDS {
        let history = hand_history(node, hand);
        let _ = write!(out, "H{:?}:", hand);
        if let Some(request) = &history.last_nonempty {
            for item in &request.assignments {
                let _ = write!(out, "{:?}@{},", item.finger, item.midi);
            }
        } else {
            out.push_str("none");
        }
        let _ = write!(out, ":{:?};", history.empty_since_us);
    }

    let mut latest_same_key_attack = BTreeMap::<(i32, FingerSlot), u64>::new();
    for obligation in obligations {
        let Some(slot) = node
            .all_assignments
            .get(obligation.id)
            .copied()
            .flatten()
        else {
            continue;
        };
        latest_same_key_attack
            .entry((obligation.midi, slot))
            .and_modify(|time| *time = (*time).max(obligation.attack_time_us_floor))
            .or_insert(obligation.attack_time_us_floor);
    }
    for ((midi, slot), attack_time) in latest_same_key_attack {
        let _ = write!(
            out,
            "SK{midi}:{:?}:{:?}:{attack_time};",
            slot.hand, slot.finger
        );
    }
    out
}

fn search_execution_plan(
    score: &PianisticExecutionScore,
    model: Option<&PianistExecutionCapabilityModel>,
    state_budget: usize,
) -> SearchOutcome {
    let topology = match build_demand_topology(score) {
        Ok(topology) => topology,
        Err(reason) => {
            return SearchOutcome {
                status: ExecutionPlanStatus::NotEvaluated,
                reason,
                explored_states: 0,
                static_unmodeled_branches: 0,
                transition_unmodeled_branches: 0,
                same_key_unmodeled_branches: 0,
                plan: None,
                replay: None,
            };
        }
    };
    let Some(model) = model else {
        return SearchOutcome {
            status: ExecutionPlanStatus::ModelInsufficient,
            reason: "no-productive-pianist-execution-capability-model-bound".to_string(),
            explored_states: 0,
            static_unmodeled_branches: 0,
            transition_unmodeled_branches: 0,
            same_key_unmodeled_branches: 0,
            plan: None,
            replay: None,
        };
    };
    let boundaries = match build_boundaries(&topology.obligations) {
        Ok(boundaries) => boundaries,
        Err(reason) => {
            return SearchOutcome {
                status: ExecutionPlanStatus::NotEvaluated,
                reason,
                explored_states: 0,
                static_unmodeled_branches: 0,
                transition_unmodeled_branches: 0,
                same_key_unmodeled_branches: 0,
                plan: None,
                replay: None,
            };
        }
    };
    let geometry = KeyboardGeometry::standard_88_key();
    let mut counters = SearchCounters::new();
    let mut nodes = vec![SearchNode::initial(topology.obligations.len())];
    let mut previous_boundary_time = None;

    for boundary in &boundaries {
        let distinct_active_keys = boundary
            .active
            .iter()
            .filter_map(|id| topology.obligations.get(*id).map(|item| item.midi))
            .collect::<BTreeSet<_>>()
            .len();
        if distinct_active_keys > 10 {
            counters.static_unmodeled_branches = counters.static_unmodeled_branches.saturating_add(1);
            return SearchOutcome {
                status: ExecutionPlanStatus::ModelInsufficient,
                reason: format!("more-than-ten-concurrent-distinct-keys-v2-technique-unmodeled-at-tick-{}", boundary.tick),
                explored_states: counters.explored_states,
                static_unmodeled_branches: counters.static_unmodeled_branches,
                transition_unmodeled_branches: counters.transition_unmodeled_branches,
                same_key_unmodeled_branches: counters.same_key_unmodeled_branches,
                plan: None,
                replay: None,
            };
        }

        let active_set = boundary.active.iter().copied().collect::<BTreeSet<_>>();
        let mut next_nodes = BTreeMap::<String, SearchNode>::new();
        for node in &nodes {
            let mut base = node.clone();
            trim_released(&mut base, &active_set);
            let already_active = base.active_slots.keys().copied().collect::<BTreeSet<_>>();
            let attacking = boundary
                .attacking
                .iter()
                .copied()
                .filter(|id| !already_active.contains(id))
                .collect::<Vec<_>>();
            let mut assigned = Vec::new();
            assign_new_obligations(
                &base,
                &attacking,
                &topology.obligations,
                0,
                &mut assigned,
                state_budget,
                &mut counters,
            );
            if counters.resource_limit {
                break;
            }
            let validation_context = CandidateValidationContext {
                boundary,
                previous_boundary_time,
                topology: &topology,
                geometry,
                model,
            };
            for mut candidate in assigned {
                if validate_candidate_state(
                    Some(node),
                    &mut candidate,
                    &validation_context,
                    &mut counters,
                ) {
                    let key = canonical_node_key(&candidate, &topology.obligations);
                    next_nodes.entry(key).or_insert(candidate);
                }
            }
        }
        if counters.resource_limit {
            return SearchOutcome {
                status: ExecutionPlanStatus::ResourceLimit,
                reason: format!("state-budget-exhausted:{state_budget}"),
                explored_states: counters.explored_states,
                static_unmodeled_branches: counters.static_unmodeled_branches,
                transition_unmodeled_branches: counters.transition_unmodeled_branches,
                same_key_unmodeled_branches: counters.same_key_unmodeled_branches,
                plan: None,
                replay: None,
            };
        }
        if next_nodes.is_empty() {
            let model_insufficient = counters.static_unmodeled_branches > 0
                || counters.transition_unmodeled_branches > 0
                || counters.same_key_unmodeled_branches > 0;
            return SearchOutcome {
                status: if model_insufficient {
                    ExecutionPlanStatus::ModelInsufficient
                } else {
                    ExecutionPlanStatus::NoRealizationUnderModel
                },
                reason: if model_insufficient {
                    format!("all-modeled-paths-exhausted-with-unmodeled-branches-at-tick-{}", boundary.tick)
                } else {
                    format!("all-v2-paths-exhausted-under-explicit-model-at-tick-{}", boundary.tick)
                },
                explored_states: counters.explored_states,
                static_unmodeled_branches: counters.static_unmodeled_branches,
                transition_unmodeled_branches: counters.transition_unmodeled_branches,
                same_key_unmodeled_branches: counters.same_key_unmodeled_branches,
                plan: None,
                replay: None,
            };
        }
        nodes = next_nodes.into_values().collect();
        previous_boundary_time = Some(boundary.time_us_floor);
    }

    let Some(winner) = nodes.into_iter().next() else {
        return SearchOutcome {
            status: ExecutionPlanStatus::NoRealizationUnderModel,
            reason: "search-finished-without-state".to_string(),
            explored_states: counters.explored_states,
            static_unmodeled_branches: counters.static_unmodeled_branches,
            transition_unmodeled_branches: counters.transition_unmodeled_branches,
            same_key_unmodeled_branches: counters.same_key_unmodeled_branches,
            plan: None,
            replay: None,
        };
    };
    let mut assignments = winner
        .all_assignments
        .iter()
        .enumerate()
        .filter_map(|(obligation_id, slot)| {
            slot.map(|slot| PlanAssignment {
                obligation_id,
                hand: slot.hand,
                finger: slot.finger,
            })
        })
        .collect::<Vec<_>>();
    assignments.sort_by_key(|item| item.obligation_id);
    if assignments.len() != topology.obligations.len() {
        return SearchOutcome {
            status: ExecutionPlanStatus::ModelInsufficient,
            reason: "internal-search-produced-incomplete-assignment".to_string(),
            explored_states: counters.explored_states,
            static_unmodeled_branches: counters.static_unmodeled_branches,
            transition_unmodeled_branches: counters.transition_unmodeled_branches,
            same_key_unmodeled_branches: counters.same_key_unmodeled_branches,
            plan: None,
            replay: None,
        };
    }
    let witness_sha256 = witness_fingerprint(score, model, &assignments);
    let plan = PianisticExecutionPlan {
        score_fingerprint_sha256: score.musical_execution_fingerprint_sha256.clone(),
        capability_model_id: model.model_id.clone(),
        assignments,
        physical_key_obligations: topology.obligations.len(),
        source_execution_events: topology.source_execution_events,
        witness_sha256,
    };
    let replay = replay_execution_plan(score, model, &plan);
    if replay.status != WitnessReplayStatus::Verified {
        return SearchOutcome {
            status: ExecutionPlanStatus::ModelInsufficient,
            reason: format!("independent-witness-replay-failed:{}", replay.reason),
            explored_states: counters.explored_states,
            static_unmodeled_branches: counters.static_unmodeled_branches,
            transition_unmodeled_branches: counters.transition_unmodeled_branches,
            same_key_unmodeled_branches: counters.same_key_unmodeled_branches,
            plan: None,
            replay: Some(replay),
        };
    }
    SearchOutcome {
        status: ExecutionPlanStatus::WitnessFound,
        reason: "complete-v2-plan-found-and-independently-replayed".to_string(),
        explored_states: counters.explored_states,
        static_unmodeled_branches: counters.static_unmodeled_branches,
        transition_unmodeled_branches: counters.transition_unmodeled_branches,
        same_key_unmodeled_branches: counters.same_key_unmodeled_branches,
        plan: Some(plan),
        replay: Some(replay),
    }
}

fn replay_execution_plan(
    score: &PianisticExecutionScore,
    model: &PianistExecutionCapabilityModel,
    plan: &PianisticExecutionPlan,
) -> WitnessReplay {
    let topology = match build_demand_topology(score) {
        Ok(topology) => topology,
        Err(reason) => return replay_rejected(reason, 0, 0),
    };
    if plan.score_fingerprint_sha256 != score.musical_execution_fingerprint_sha256 {
        return replay_rejected("score-fingerprint-mismatch".to_string(), 0, 0);
    }
    if plan.capability_model_id != model.model_id {
        return replay_rejected("capability-model-id-mismatch".to_string(), 0, 0);
    }
    if plan.physical_key_obligations != topology.obligations.len()
        || plan.source_execution_events != topology.source_execution_events
    {
        return replay_rejected("plan-cardinality-mismatch".to_string(), 0, 0);
    }
    if plan.witness_sha256 != witness_fingerprint(score, model, &plan.assignments) {
        return replay_rejected("witness-fingerprint-mismatch".to_string(), 0, 0);
    }

    let mut assignment_by_id = BTreeMap::<usize, FingerSlot>::new();
    for assignment in &plan.assignments {
        if assignment.obligation_id >= topology.obligations.len() {
            return replay_rejected("assignment-obligation-id-out-of-range".to_string(), 0, 0);
        }
        let slot = FingerSlot {
            hand: assignment.hand,
            finger: assignment.finger,
        };
        if assignment_by_id.insert(assignment.obligation_id, slot).is_some() {
            return replay_rejected("duplicate-obligation-assignment".to_string(), 0, 0);
        }
    }
    if assignment_by_id.len() != topology.obligations.len() {
        return replay_rejected("incomplete-obligation-coverage".to_string(), 0, assignment_by_id.len());
    }

    let boundaries = match build_boundaries(&topology.obligations) {
        Ok(boundaries) => boundaries,
        Err(reason) => return replay_rejected(reason, 0, 0),
    };
    let geometry = KeyboardGeometry::standard_88_key();
    let mut previous_active = BTreeMap::<usize, FingerSlot>::new();
    let mut left_history = HandHistory::initial();
    let mut right_history = HandHistory::initial();
    let mut previous_boundary_time = None;
    let mut checked = 0usize;

    for boundary in &boundaries {
        let mut active = BTreeMap::new();
        let mut occupied_slots = BTreeMap::<FingerSlot, i32>::new();
        let mut occupied_keys = BTreeMap::<i32, FingerSlot>::new();
        for obligation_id in &boundary.active {
            let Some(slot) = assignment_by_id.get(obligation_id).copied() else {
                return replay_rejected("active-obligation-without-assignment".to_string(), checked, assignment_by_id.len());
            };
            let Some(obligation) = topology.obligations.get(*obligation_id) else {
                return replay_rejected("active-obligation-out-of-range".to_string(), checked, assignment_by_id.len());
            };
            if let Some(previous_midi) = occupied_slots.insert(slot, obligation.midi)
                && previous_midi != obligation.midi
            {
                return replay_rejected("simultaneous-finger-slot-double-booking".to_string(), checked, assignment_by_id.len());
            }
            if let Some(previous_slot) = occupied_keys.insert(obligation.midi, slot)
                && previous_slot != slot
            {
                return replay_rejected("same-key-overlap-assigned-to-multiple-finger-slots".to_string(), checked, assignment_by_id.len());
            }
            active.insert(*obligation_id, slot);
        }

        for obligation_id in &boundary.attacking {
            let Some(obligation) = topology.obligations.get(*obligation_id) else {
                return replay_rejected("attacking-obligation-out-of-range".to_string(), checked, assignment_by_id.len());
            };
            let Some(slot) = assignment_by_id.get(obligation_id).copied() else {
                return replay_rejected("attacking-obligation-without-assignment".to_string(), checked, assignment_by_id.len());
            };
            let latest_prior_attack = topology
                .obligations
                .iter()
                .filter(|prior| {
                    prior.id != *obligation_id
                        && prior.midi == obligation.midi
                        && prior.attack_tick < obligation.attack_tick
                        && assignment_by_id.get(&prior.id).copied() == Some(slot)
                })
                .map(|prior| prior.attack_time_us_floor)
                .max();
            let Some(prior_attack_time) = latest_prior_attack else {
                continue;
            };
            let available = obligation.attack_time_us_floor.saturating_sub(prior_attack_time);
            let (decision, reason, _) = assess_same_key_rearticulation(
                geometry,
                model,
                slot,
                obligation.midi,
                available,
            );
            match decision {
                StaticCapabilityDecision::Supported => {}
                StaticCapabilityDecision::Rejected => {
                    return replay_rejected(
                        format!("same-key-rearticulation-rejected-at-tick-{}:{reason}", boundary.tick),
                        checked,
                        assignment_by_id.len(),
                    );
                }
                StaticCapabilityDecision::Unmodeled => {
                    return WitnessReplay {
                        status: WitnessReplayStatus::ModelInsufficient,
                        reason: format!(
                            "same-key-rearticulation-unmodeled-at-tick-{}:{reason}",
                            boundary.tick
                        ),
                        checked_boundaries: checked,
                        checked_obligations: assignment_by_id.len(),
                    };
                }
            }
        }

        let previous_node_for_replay = SearchNode {
            active_slots: previous_active.clone(),
            all_assignments: Vec::new(),
            left_history: left_history.clone(),
            right_history: right_history.clone(),
        };
        let current_node_for_replay = SearchNode {
            active_slots: active.clone(),
            all_assignments: Vec::new(),
            left_history: left_history.clone(),
            right_history: right_history.clone(),
        };
        if direct_boundary_reuses_occupied_finger_on_different_key(
            &previous_node_for_replay,
            &current_node_for_replay,
            &topology,
        ) {
            return replay_rejected(
                format!("direct-boundary-finger-key-change-at-tick-{}", boundary.tick),
                checked,
                assignment_by_id.len(),
            );
        }

        for hand in HANDS {
            let current_request = request_for_hand(hand, &active, &topology.obligations);
            let static_assessment = assess_static_configuration(
                geometry,
                Some(&model.static_envelope),
                &current_request,
            );
            match static_assessment.decision {
                StaticCapabilityDecision::Supported => {}
                StaticCapabilityDecision::Rejected => {
                    return replay_rejected(
                        format!("static-configuration-rejected-at-tick-{}", boundary.tick),
                        checked,
                        assignment_by_id.len(),
                    );
                }
                StaticCapabilityDecision::Unmodeled => {
                    return WitnessReplay {
                        status: WitnessReplayStatus::ModelInsufficient,
                        reason: format!("static-configuration-unmodeled-at-tick-{}", boundary.tick),
                        checked_boundaries: checked,
                        checked_obligations: assignment_by_id.len(),
                    };
                }
            }

            let previous_request = request_for_hand(hand, &previous_active, &topology.obligations);
            let history = match hand {
                Hand::Left => &mut left_history,
                Hand::Right => &mut right_history,
            };
            if current_request.assignments.is_empty() {
                if !previous_request.assignments.is_empty() {
                    history.last_nonempty = Some(previous_request);
                    history.empty_since_us = Some(boundary.time_us_floor);
                }
                continue;
            }

            let transition_from_and_available = if !previous_request.assignments.is_empty() {
                let available = previous_boundary_time
                    .and_then(|time| boundary.time_us_floor.checked_sub(time))
                    .unwrap_or(0);
                Some((previous_request, available))
            } else if let (Some(last_nonempty), Some(empty_since)) = (
                history.last_nonempty.clone(),
                history.empty_since_us,
            ) {
                Some((
                    last_nonempty,
                    boundary.time_us_floor.saturating_sub(empty_since),
                ))
            } else {
                None
            };
            if let Some((from_request, available)) = transition_from_and_available {
                let (decision, reason, _) = assess_transition(
                    geometry,
                    model,
                    &from_request,
                    &current_request,
                    available,
                );
                match decision {
                    StaticCapabilityDecision::Supported => {}
                    StaticCapabilityDecision::Rejected => {
                        return replay_rejected(
                            format!("transition-rejected-at-tick-{}:{reason}", boundary.tick),
                            checked,
                            assignment_by_id.len(),
                        );
                    }
                    StaticCapabilityDecision::Unmodeled => {
                        return WitnessReplay {
                            status: WitnessReplayStatus::ModelInsufficient,
                            reason: format!("transition-unmodeled-at-tick-{}:{reason}", boundary.tick),
                            checked_boundaries: checked,
                            checked_obligations: assignment_by_id.len(),
                        };
                    }
                }
            }
            history.last_nonempty = Some(current_request);
            history.empty_since_us = None;
        }

        previous_active = active;
        previous_boundary_time = Some(boundary.time_us_floor);
        checked = checked.saturating_add(1);
    }
    WitnessReplay {
        status: WitnessReplayStatus::Verified,
        reason: "independent-replay-complete".to_string(),
        checked_boundaries: checked,
        checked_obligations: assignment_by_id.len(),
    }
}

fn replay_rejected(reason: String, checked_boundaries: usize, checked_obligations: usize) -> WitnessReplay {
    WitnessReplay {
        status: WitnessReplayStatus::Rejected,
        reason,
        checked_boundaries,
        checked_obligations,
    }
}

fn witness_fingerprint(
    score: &PianisticExecutionScore,
    model: &PianistExecutionCapabilityModel,
    assignments: &[PlanAssignment],
) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(SCHEMA.as_bytes());
    append_len_prefixed(&mut bytes, score.musical_execution_fingerprint_sha256.as_bytes());
    append_len_prefixed(&mut bytes, model.model_id.as_bytes());
    for assignment in assignments {
        bytes.extend_from_slice(&(assignment.obligation_id as u64).to_le_bytes());
        bytes.push(match assignment.hand {
            Hand::Left => 0,
            Hand::Right => 1,
        });
        bytes.push(match assignment.finger {
            Finger::Thumb => 0,
            Finger::Index => 1,
            Finger::Middle => 2,
            Finger::Ring => 3,
            Finger::Little => 4,
        });
    }
    sha256::hex(&bytes)
}

fn append_len_prefixed(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    out.extend_from_slice(bytes);
}

fn synthetic_score() -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "synthetic-runtime-integrity-score".to_string(),
        source_path: PathBuf::from("synthetic-runtime-integrity.musicxml"),
        pitched_note_segments: 2,
        execution_events: vec![
            PianisticExecutionEvent {
                part_id: "P1".to_string(),
                source_staff: "1".to_string(),
                source_voice: "1".to_string(),
                midi: 60,
                attack_tick: 0,
                release_tick: 96,
                attack_time_us_floor: 0,
                release_time_us_floor: 500_000,
                note_segments: 1,
            },
            PianisticExecutionEvent {
                part_id: "P1".to_string(),
                source_staff: "2".to_string(),
                source_voice: "7".to_string(),
                midi: 62,
                attack_tick: 192,
                release_tick: 288,
                attack_time_us_floor: 1_000_000,
                release_time_us_floor: 1_500_000,
                note_segments: 1,
            },
        ],
        tie_continuation_segments: 0,
        tempo_points: Vec::new(),
        terminal_tick: 288,
        terminal_physical_time_us_floor: Some(1_500_000),
        musical_execution_fingerprint_sha256: "synthetic-score-fingerprint".to_string(),
        source_affinity_fingerprint_sha256: "synthetic-affinity-fingerprint".to_string(),
    }
}

fn synthetic_supported_model() -> Result<PianistExecutionCapabilityModel, String> {
    let geometry = KeyboardGeometry::standard_88_key();
    let c_request = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![FingerKeyAssignment {
            finger: Finger::Thumb,
            midi: 60,
        }],
    };
    let d_request = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![FingerKeyAssignment {
            finger: Finger::Thumb,
            midi: 62,
        }],
    };
    let signature = configuration_signature(geometry, &c_request)
        .ok_or_else(|| "synthetic-static-signature-unavailable".to_string())?;
    let static_envelope = PianistCapabilityEnvelope::from_entries(
        "synthetic-execution-static-profile",
        "synthetic-runtime-integrity-evidence-not-a-pianist-profile",
        vec![CapabilityEntry {
            signature,
            decision: StaticCapabilityDecision::Supported,
            evidence_id: "synthetic-single-thumb-white-key".to_string(),
        }],
    )?;
    let transition = transition_signature(geometry, &c_request, &d_request)
        .ok_or_else(|| "synthetic-transition-signature-unavailable".to_string())?;
    PianistExecutionCapabilityModel::from_entries(
        "synthetic-execution-capability-model",
        "synthetic-runtime-integrity-evidence-not-a-pianist-profile",
        static_envelope,
        vec![TransitionCapabilityEntry {
            signature: transition,
            decision: StaticCapabilityDecision::Supported,
            minimum_transition_time_us: 400_000,
            evidence_id: "synthetic-c4-d4-rested-transition".to_string(),
        }],
        Vec::new(),
    )
}

fn synthetic_same_key_overlap_score() -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "synthetic-same-key-overlap".to_string(),
        source_path: PathBuf::from("synthetic-same-key-overlap.musicxml"),
        pitched_note_segments: 2,
        execution_events: vec![
            PianisticExecutionEvent {
                part_id: "P1".to_string(),
                source_staff: "1".to_string(),
                source_voice: "1".to_string(),
                midi: 60,
                attack_tick: 0,
                release_tick: 192,
                attack_time_us_floor: 0,
                release_time_us_floor: 1_000_000,
                note_segments: 1,
            },
            PianisticExecutionEvent {
                part_id: "P1".to_string(),
                source_staff: "2".to_string(),
                source_voice: "2".to_string(),
                midi: 60,
                attack_tick: 96,
                release_tick: 288,
                attack_time_us_floor: 500_000,
                release_time_us_floor: 1_500_000,
                note_segments: 1,
            },
        ],
        tie_continuation_segments: 0,
        tempo_points: Vec::new(),
        terminal_tick: 288,
        terminal_physical_time_us_floor: Some(1_500_000),
        musical_execution_fingerprint_sha256: "synthetic-same-key-score-fingerprint".to_string(),
        source_affinity_fingerprint_sha256: "synthetic-same-key-affinity-fingerprint".to_string(),
    }
}

fn synthetic_same_key_post_release_score() -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "synthetic-same-key-post-release".to_string(),
        source_path: PathBuf::from("synthetic-same-key-post-release.musicxml"),
        pitched_note_segments: 2,
        execution_events: vec![
            PianisticExecutionEvent {
                part_id: "P1".to_string(),
                source_staff: "1".to_string(),
                source_voice: "1".to_string(),
                midi: 60,
                attack_tick: 0,
                release_tick: 96,
                attack_time_us_floor: 0,
                release_time_us_floor: 500_000,
                note_segments: 1,
            },
            PianisticExecutionEvent {
                part_id: "P1".to_string(),
                source_staff: "1".to_string(),
                source_voice: "1".to_string(),
                midi: 60,
                attack_tick: 96,
                release_tick: 192,
                attack_time_us_floor: 500_000,
                release_time_us_floor: 1_000_000,
                note_segments: 1,
            },
        ],
        tie_continuation_segments: 0,
        tempo_points: Vec::new(),
        terminal_tick: 192,
        terminal_physical_time_us_floor: Some(1_000_000),
        musical_execution_fingerprint_sha256: "synthetic-same-key-post-release-score-fingerprint".to_string(),
        source_affinity_fingerprint_sha256: "synthetic-same-key-post-release-affinity-fingerprint".to_string(),
    }
}

fn synthetic_same_key_supported_model(
    minimum_rearticulation_time_us: u64,
) -> Result<PianistExecutionCapabilityModel, String> {
    let geometry = KeyboardGeometry::standard_88_key();
    let request = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![FingerKeyAssignment {
            finger: Finger::Thumb,
            midi: 60,
        }],
    };
    let envelope = PianistCapabilityEnvelope::from_entries(
        "synthetic-same-key-static-profile",
        "synthetic-runtime-integrity-evidence-not-a-pianist-profile",
        vec![CapabilityEntry {
            signature: configuration_signature(geometry, &request)
                .ok_or_else(|| "synthetic-same-key-static-signature-unavailable".to_string())?,
            decision: StaticCapabilityDecision::Supported,
            evidence_id: "synthetic-right-thumb-c4".to_string(),
        }],
    )?;
    PianistExecutionCapabilityModel::from_entries(
        "synthetic-same-key-execution-capability-model",
        "synthetic-runtime-integrity-evidence-not-a-pianist-profile",
        envelope,
        Vec::new(),
        vec![SameKeyCapabilityEntry {
            signature: SameKeyTechniqueSignature {
                hand: Hand::Right,
                finger: Finger::Thumb,
                surface: KeySurface::White,
            },
            decision: StaticCapabilityDecision::Supported,
            minimum_rearticulation_time_us,
            evidence_id: "synthetic-same-key-right-thumb-white".to_string(),
        }],
    )
}

fn verify_runtime_witness_integrity() -> Result<String, String> {
    let score = synthetic_score();
    let topology = build_demand_topology(&score)?;
    if topology.obligations.len() != 2
        || topology.source_execution_events != 2
        || topology.coalesced_exact_duplicate_events != 0
        || topology.attack_boundary_count != 2
        || topology.maximum_simultaneous_attacks != 1
        || topology.maximum_concurrent_key_obligations != 1
    {
        return Err("synthetic-demand-topology-mismatch".to_string());
    }
    let model = synthetic_supported_model()?;
    if model.provenance != "synthetic-runtime-integrity-evidence-not-a-pianist-profile"
        || model.static_envelope.profile_id != "synthetic-execution-static-profile"
        || model.transitions.len() != 1
        || !model.same_key_techniques.is_empty()
    {
        return Err("synthetic-execution-model-binding-mismatch".to_string());
    }
    let witness = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
    if witness.status != ExecutionPlanStatus::WitnessFound {
        return Err(format!("synthetic-witness-not-found:{}", witness.reason));
    }
    let plan = witness
        .plan
        .as_ref()
        .ok_or_else(|| "synthetic-witness-plan-missing".to_string())?;
    let replay = witness
        .replay
        .as_ref()
        .ok_or_else(|| "synthetic-witness-replay-missing".to_string())?;
    if plan.assignments.len() != 2
        || plan.physical_key_obligations != 2
        || plan.source_execution_events != 2
        || plan.witness_sha256.len() != 64
        || replay.status != WitnessReplayStatus::Verified
        || replay.checked_boundaries != topology.boundary_count
        || replay.checked_obligations != 2
    {
        return Err("synthetic-witness-or-replay-mismatch".to_string());
    }

    let missing_model = search_execution_plan(&score, None, DEFAULT_STATE_BUDGET);
    if missing_model.status != ExecutionPlanStatus::ModelInsufficient {
        return Err("missing-model-does-not-fail-closed".to_string());
    }
    let resource_limited = search_execution_plan(&score, Some(&model), 0);
    if resource_limited.status != ExecutionPlanStatus::ResourceLimit {
        return Err("resource-limit-status-mismatch".to_string());
    }

    let same_key_score = synthetic_same_key_overlap_score();
    let same_key_model = synthetic_same_key_supported_model(400_000)?;
    let same_key_witness = search_execution_plan(
        &same_key_score,
        Some(&same_key_model),
        DEFAULT_STATE_BUDGET,
    );
    if same_key_witness.status != ExecutionPlanStatus::WitnessFound
        || same_key_witness
            .replay
            .as_ref()
            .is_none_or(|replay| replay.status != WitnessReplayStatus::Verified)
    {
        return Err(format!(
            "synthetic-same-key-witness-not-verified:{}",
            same_key_witness.reason
        ));
    }
    let mut missing_same_key_model = same_key_model;
    missing_same_key_model.same_key_techniques.clear();
    let missing_same_key = search_execution_plan(
        &same_key_score,
        Some(&missing_same_key_model),
        DEFAULT_STATE_BUDGET,
    );
    if missing_same_key.status != ExecutionPlanStatus::ModelInsufficient
        || missing_same_key.same_key_unmodeled_branches == 0
    {
        return Err("missing-same-key-capability-does-not-fail-closed".to_string());
    }

    let post_release_score = synthetic_same_key_post_release_score();
    let post_release_model = synthetic_same_key_supported_model(400_000)?;
    let post_release_witness = search_execution_plan(
        &post_release_score,
        Some(&post_release_model),
        DEFAULT_STATE_BUDGET,
    );
    if post_release_witness.status != ExecutionPlanStatus::WitnessFound
        || post_release_witness
            .replay
            .as_ref()
            .is_none_or(|replay| replay.status != WitnessReplayStatus::Verified)
    {
        return Err(format!(
            "synthetic-post-release-same-key-witness-not-verified:{}",
            post_release_witness.reason
        ));
    }
    let mut missing_post_release_model = post_release_model;
    missing_post_release_model.same_key_techniques.clear();
    let missing_post_release = search_execution_plan(
        &post_release_score,
        Some(&missing_post_release_model),
        DEFAULT_STATE_BUDGET,
    );
    if missing_post_release.status != ExecutionPlanStatus::ModelInsufficient
        || missing_post_release.same_key_unmodeled_branches == 0
    {
        return Err("post-release-same-key-capability-does-not-fail-closed".to_string());
    }

    let no_realization_marker = ExecutionPlanStatus::NoRealizationUnderModel.as_str();
    let not_evaluated_marker = ExecutionPlanStatus::NotEvaluated.as_str();
    if no_realization_marker != "NO_REALIZATION_UNDER_MODEL"
        || not_evaluated_marker != "NOT_EVALUATED"
    {
        return Err("execution-plan-status-vocabulary-mismatch".to_string());
    }

    Ok(format!(
        "PASS; SyntheticWitness yes; IndependentReplay VERIFIED; SameKeyAdapter VERIFIED; SameKeyPostReleaseRearticulation VERIFIED; MissingSameKeyEvidence MODEL_INSUFFICIENT; SyntheticObligations {}; SyntheticBoundaries {}; StatusVocabulary REALIZATION_WITNESS_FOUND/NO_REALIZATION_UNDER_MODEL/MODEL_INSUFFICIENT/RESOURCE_LIMIT/NOT_EVALUATED; ProductiveCapabilityModel no",
        topology.obligations.len(),
        topology.boundary_count,
    ))
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("===========================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; InputAuthority: {INPUT_AUTHORITY}; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("WitnessContract: a positive claim requires a complete assignment of every abstract key obligation to a physical hand/finger control slot plus IndependentWitnessReplay=VERIFIED. Search failure alone is never a negative human-playability claim.\n");
    out.push_str("TechniqueBoundaryV2: one physical finger slot controls at most one MIDI key at a boundary; multiple overlapping abstract obligations for that same MIDI may share that one slot. Hand/finger identity is stable for each obligation; finger substitution and hand takeover during a hold remain unmodeled. StaffIsHand false; VoiceIsHand false.\n");
    out.push_str("DuplicatePhysicalDemandContract: execution events with identical MIDI, attack and release are coalesced into one physical key obligation with source multiplicity. Non-identical same-key overlaps remain separate abstract obligations but are no longer automatic impossibility. Coincident attacks may be co-satisfied by one physical attack; every later same-key attack that reuses the same physical hand/finger slot requires explicit SameKeyTechniqueSignature capability evidence, including ordinary post-release rearticulation.\n");
    out.push_str("SameKeyAdapterContract: signature fields are Hand+Finger+KeySurface. Rearticulation demand is measured attack-to-attack for a later attack on the same MIDI when the same hand/finger slot is reused; overlap is not required. Missing same-key evidence is UNMODELED/MODEL_INSUFFICIENT; an explicit rejection or insufficient rearticulation time rejects only that modeled branch. No same-key threshold is inferred from the score.\n");
    out.push_str("TransitionContract: changed nonempty hand configurations require explicit whole-configuration transition evidence with a minimum physical time; missing transition evidence is UNMODELED. Pure release to an empty hand and initial pre-positioning require no inferred reach rule.\n");

    let integrity = verify_runtime_witness_integrity()
        .unwrap_or_else(|reason| panic!("PianisticExecutionPlanWitness runtime integrity failed: {reason}"));
    let _ = writeln!(out, "WitnessRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason score-title-missing; ExecutionPlanStatus NOT_EVALUATED; PlayabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let path = PathBuf::from(format!("{title}.musicxml"));
    let score = pianistic_execution_score::project_file(&path);
    if score.status != ProjectionStatus::Complete {
        let _ = writeln!(
            out,
            "InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason {}; ExecutionPlanStatus NOT_EVALUATED; PlayabilityClaim {PLAYABILITY_CLAIM}.\n",
            score.reason,
        );
        return out;
    }
    let topology = match build_demand_topology(&score) {
        Ok(topology) => topology,
        Err(reason) => {
            let _ = writeln!(
                out,
                "InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason {}; ExecutionPlanStatus NOT_EVALUATED; PlayabilityClaim {PLAYABILITY_CLAIM}.\n",
                reason,
            );
            return out;
        }
    };
    let outcome = search_execution_plan(&score, None, DEFAULT_STATE_BUDGET);
    let _ = writeln!(
        out,
        "InputProjectionStatus: COMPLETE; SourceExecutionEvents {}; PhysicalKeyObligations {}; CoalescedExactDuplicateEvents {}; BoundaryCount {}; AttackBoundaryCount {}; MaxSimultaneousAttacks {}; MaxConcurrentKeyObligations {}.",
        topology.source_execution_events,
        topology.obligations.len(),
        topology.coalesced_exact_duplicate_events,
        topology.boundary_count,
        topology.attack_boundary_count,
        topology.maximum_simultaneous_attacks,
        topology.maximum_concurrent_key_obligations,
    );
    let _ = writeln!(
        out,
        "ExecutionPlanStatus: {}; Reason {}; ProductivePianistExecutionCapabilityModel none; SearchStateBudget {}; ExploredStates {}; StaticUnmodeledBranches {}; TransitionUnmodeledBranches {}; SameKeyUnmodeledBranches {}; SameKeyTechniqueSignature EXACT_WITNESS_BOUND; Witness none; IndependentWitnessReplay NOT_RUN; PlayabilityClaim {PLAYABILITY_CLAIM}.",
        outcome.status.as_str(),
        outcome.reason,
        DEFAULT_STATE_BUDGET,
        outcome.explored_states,
        outcome.static_unmodeled_branches,
        outcome.transition_unmodeled_branches,
        outcome.same_key_unmodeled_branches,
    );
    out.push_str("NonClaims: no productive hand assignment, no productive finger assignment, no reach truth, no transition truth, no same-key human threshold, no difficulty score, no feasibility frontier, no repair candidate. The same-key adapter is now semantically bound to the witness, but a productive model-relative witness remains blocked until an explicit capability model supplies Static+Transition+SameKey evidence.\n\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(midi: i32, attack: i64, release: i64, attack_us: u64, release_us: u64) -> PianisticExecutionEvent {
        PianisticExecutionEvent {
            part_id: "P1".to_string(),
            source_staff: "9".to_string(),
            source_voice: "99".to_string(),
            midi,
            attack_tick: attack,
            release_tick: release,
            attack_time_us_floor: attack_us,
            release_time_us_floor: release_us,
            note_segments: 1,
        }
    }

    fn score(events: Vec<PianisticExecutionEvent>) -> PianisticExecutionScore {
        PianisticExecutionScore {
            status: ProjectionStatus::Complete,
            reason: "test".to_string(),
            source_path: PathBuf::from("test.musicxml"),
            pitched_note_segments: events.len(),
            execution_events: events,
            tie_continuation_segments: 0,
            tempo_points: Vec::new(),
            terminal_tick: 384,
            terminal_physical_time_us_floor: Some(2_000_000),
            musical_execution_fingerprint_sha256: "test-score-fingerprint".to_string(),
            source_affinity_fingerprint_sha256: "test-affinity-fingerprint".to_string(),
        }
    }

    fn rejected_single_note_model() -> PianistExecutionCapabilityModel {
        let geometry = KeyboardGeometry::standard_88_key();
        let mut entries = Vec::new();
        for hand in HANDS {
            for finger in FINGERS {
                let request = StaticConfigurationRequest {
                    hand,
                    assignments: vec![FingerKeyAssignment { finger, midi: 60 }],
                };
                entries.push(CapabilityEntry {
                    signature: configuration_signature(geometry, &request).expect("signature"),
                    decision: StaticCapabilityDecision::Rejected,
                    evidence_id: format!("reject-{hand:?}-{finger:?}"),
                });
            }
        }
        let envelope = PianistCapabilityEnvelope::from_entries(
            "reject-all-single-note",
            "unit-test-only",
            entries,
        )
        .expect("envelope");
        PianistExecutionCapabilityModel::from_entries(
            "reject-all-model",
            "unit-test-only",
            envelope,
            Vec::new(),
            Vec::new(),
        )
        .expect("model")
    }

    fn exhaustive_same_key_single_note_model(
        minimum_rearticulation_time_us: u64,
    ) -> PianistExecutionCapabilityModel {
        let geometry = KeyboardGeometry::standard_88_key();
        let mut static_entries = Vec::new();
        let mut same_key_entries = Vec::new();
        for hand in HANDS {
            for finger in FINGERS {
                let request = StaticConfigurationRequest {
                    hand,
                    assignments: vec![FingerKeyAssignment { finger, midi: 60 }],
                };
                static_entries.push(CapabilityEntry {
                    signature: configuration_signature(geometry, &request)
                        .expect("single-note static signature"),
                    decision: StaticCapabilityDecision::Supported,
                    evidence_id: format!("exhaustive-static-{hand:?}-{finger:?}-c4"),
                });
                same_key_entries.push(SameKeyCapabilityEntry {
                    signature: SameKeyTechniqueSignature {
                        hand,
                        finger,
                        surface: KeySurface::White,
                    },
                    decision: StaticCapabilityDecision::Supported,
                    minimum_rearticulation_time_us,
                    evidence_id: format!("exhaustive-same-key-{hand:?}-{finger:?}-white"),
                });
            }
        }
        let envelope = PianistCapabilityEnvelope::from_entries(
            "exhaustive-same-key-single-note-static-profile",
            "unit-test-only",
            static_entries,
        )
        .expect("exhaustive static envelope");
        PianistExecutionCapabilityModel::from_entries(
            "exhaustive-same-key-single-note-model",
            "unit-test-only",
            envelope,
            Vec::new(),
            same_key_entries,
        )
        .expect("exhaustive same-key model")
    }

    fn single_slot_same_key_model(
        minimum_rearticulation_time_us: Option<u64>,
    ) -> PianistExecutionCapabilityModel {
        let geometry = KeyboardGeometry::standard_88_key();
        let mut static_entries = Vec::new();
        for hand in HANDS {
            for finger in FINGERS {
                let request = StaticConfigurationRequest {
                    hand,
                    assignments: vec![FingerKeyAssignment { finger, midi: 60 }],
                };
                let supported = hand == Hand::Right && finger == Finger::Thumb;
                static_entries.push(CapabilityEntry {
                    signature: configuration_signature(geometry, &request)
                        .expect("single-slot static signature"),
                    decision: if supported {
                        StaticCapabilityDecision::Supported
                    } else {
                        StaticCapabilityDecision::Rejected
                    },
                    evidence_id: format!("single-slot-static-{hand:?}-{finger:?}"),
                });
            }
        }
        let envelope = PianistCapabilityEnvelope::from_entries(
            "single-slot-same-key-static-profile",
            "unit-test-only",
            static_entries,
        )
        .expect("single-slot static envelope");
        let same_key_entries = minimum_rearticulation_time_us
            .map(|minimum| {
                vec![SameKeyCapabilityEntry {
                    signature: SameKeyTechniqueSignature {
                        hand: Hand::Right,
                        finger: Finger::Thumb,
                        surface: KeySurface::White,
                    },
                    decision: StaticCapabilityDecision::Supported,
                    minimum_rearticulation_time_us: minimum,
                    evidence_id: "single-slot-right-thumb-white".to_string(),
                }]
            })
            .unwrap_or_default();
        PianistExecutionCapabilityModel::from_entries(
            "single-slot-same-key-model",
            "unit-test-only",
            envelope,
            Vec::new(),
            same_key_entries,
        )
        .expect("single-slot same-key model")
    }

    #[test]
    fn canonical_node_key_binds_released_same_key_attack_history() {
        let score = score(vec![
            event(60, 0, 96, 0, 500_000),
            event(60, 192, 288, 1_000_000, 1_500_000),
        ]);
        let topology = build_demand_topology(&score).expect("topology");
        let mut right_thumb_history = SearchNode::initial(topology.obligations.len());
        right_thumb_history.all_assignments[0] = Some(FingerSlot {
            hand: Hand::Right,
            finger: Finger::Thumb,
        });
        let mut left_thumb_history = SearchNode::initial(topology.obligations.len());
        left_thumb_history.all_assignments[0] = Some(FingerSlot {
            hand: Hand::Left,
            finger: Finger::Thumb,
        });
        assert_ne!(
            canonical_node_key(&right_thumb_history, &topology.obligations),
            canonical_node_key(&left_thumb_history, &topology.obligations)
        );
    }

    #[test]
    fn exact_duplicate_sound_events_coalesce_to_one_physical_key_obligation() {
        let score = score(vec![
            event(60, 0, 96, 0, 500_000),
            event(60, 0, 96, 0, 500_000),
        ]);
        let topology = build_demand_topology(&score).expect("topology");
        assert_eq!(topology.source_execution_events, 2);
        assert_eq!(topology.obligations.len(), 1);
        assert_eq!(topology.coalesced_exact_duplicate_events, 1);
        assert_eq!(topology.obligations[0].source_event_indices, vec![0, 1]);
    }

    #[test]
    fn same_pitch_with_different_release_is_not_silently_coalesced() {
        let score = score(vec![
            event(60, 0, 96, 0, 500_000),
            event(60, 0, 192, 0, 1_000_000),
        ]);
        let topology = build_demand_topology(&score).expect("topology");
        assert_eq!(topology.obligations.len(), 2);
        let boundaries = build_boundaries(&topology.obligations).expect("boundaries");
        assert!(boundaries.iter().any(|boundary| {
            boundary.active.len() == 2
                && boundary
                    .active
                    .iter()
                    .filter_map(|id| topology.obligations.get(*id).map(|item| item.midi))
                    .all(|midi| midi == 60)
        }));
    }

    #[test]
    fn staggered_same_key_overlap_without_technique_evidence_is_model_insufficient() {
        let score = score(vec![
            event(60, 0, 192, 0, 1_000_000),
            event(60, 96, 288, 500_000, 1_500_000),
        ]);
        let mut model = synthetic_same_key_supported_model(400_000).expect("same-key model");
        model.same_key_techniques.clear();
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::ModelInsufficient);
        assert!(outcome.same_key_unmodeled_branches > 0);
    }

    #[test]
    fn staggered_same_key_overlap_with_explicit_technique_replays() {
        let score = score(vec![
            event(60, 0, 192, 0, 1_000_000),
            event(60, 96, 288, 500_000, 1_500_000),
        ]);
        let model = synthetic_same_key_supported_model(400_000).expect("same-key model");
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::WitnessFound);
        let plan = outcome.plan.expect("plan");
        assert_eq!(plan.assignments.len(), 2);
        assert_eq!(plan.assignments[0].hand, plan.assignments[1].hand);
        assert_eq!(plan.assignments[0].finger, plan.assignments[1].finger);
        let replay = replay_execution_plan(&score, &model, &plan);
        assert_eq!(replay.status, WitnessReplayStatus::Verified);
    }

    #[test]
    fn same_key_rearticulation_below_explicit_minimum_is_model_relative_negative() {
        let score = score(vec![
            event(60, 0, 192, 0, 1_000_000),
            event(60, 96, 288, 500_000, 1_500_000),
        ]);
        let model = exhaustive_same_key_single_note_model(600_000);
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::NoRealizationUnderModel);
        assert_eq!(outcome.static_unmodeled_branches, 0);
        assert_eq!(outcome.transition_unmodeled_branches, 0);
        assert_eq!(outcome.same_key_unmodeled_branches, 0);
    }

    #[test]
    fn coincident_same_key_nonduplicate_attacks_share_one_physical_slot() {
        let score = score(vec![
            event(60, 0, 96, 0, 500_000),
            event(60, 0, 192, 0, 1_000_000),
        ]);
        let mut model = synthetic_same_key_supported_model(400_000).expect("same-key model");
        model.same_key_techniques.clear();
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::WitnessFound);
        assert_eq!(outcome.same_key_unmodeled_branches, 0);
    }

    #[test]
    fn post_release_same_slot_rearticulation_without_technique_evidence_is_model_insufficient() {
        let score = score(vec![
            event(60, 0, 96, 0, 500_000),
            event(60, 96, 192, 500_000, 1_000_000),
        ]);
        let model = single_slot_same_key_model(None);
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::ModelInsufficient);
        assert!(outcome.same_key_unmodeled_branches > 0);
    }

    #[test]
    fn post_release_same_slot_rearticulation_with_explicit_technique_replays() {
        let score = score(vec![
            event(60, 0, 96, 0, 500_000),
            event(60, 96, 192, 500_000, 1_000_000),
        ]);
        let model = single_slot_same_key_model(Some(400_000));
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::WitnessFound);
        let plan = outcome.plan.expect("plan");
        assert_eq!(plan.assignments.len(), 2);
        assert_eq!(plan.assignments[0].hand, Hand::Right);
        assert_eq!(plan.assignments[0].finger, Finger::Thumb);
        assert_eq!(plan.assignments[1].hand, Hand::Right);
        assert_eq!(plan.assignments[1].finger, Finger::Thumb);
        let replay = replay_execution_plan(&score, &model, &plan);
        assert_eq!(replay.status, WitnessReplayStatus::Verified);
    }

    #[test]
    fn post_release_same_slot_rearticulation_below_explicit_minimum_is_model_relative_negative() {
        let score = score(vec![
            event(60, 0, 96, 0, 500_000),
            event(60, 96, 192, 500_000, 1_000_000),
        ]);
        let model = single_slot_same_key_model(Some(600_000));
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::NoRealizationUnderModel);
        assert_eq!(outcome.static_unmodeled_branches, 0);
        assert_eq!(outcome.transition_unmodeled_branches, 0);
        assert_eq!(outcome.same_key_unmodeled_branches, 0);
    }

    #[test]
    fn absent_productive_execution_model_is_model_insufficient() {
        let score = synthetic_score();
        let outcome = search_execution_plan(&score, None, DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::ModelInsufficient);
        assert_eq!(
            outcome.reason,
            "no-productive-pianist-execution-capability-model-bound"
        );
    }

    #[test]
    fn complete_synthetic_model_finds_and_replays_constructive_witness() {
        let score = synthetic_score();
        let model = synthetic_supported_model().expect("model");
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::WitnessFound);
        let plan = outcome.plan.expect("plan");
        let replay = replay_execution_plan(&score, &model, &plan);
        assert_eq!(replay.status, WitnessReplayStatus::Verified);
        assert_eq!(plan.assignments.len(), 2);
    }

    #[test]
    fn incomplete_static_profile_never_becomes_negative_playability_claim() {
        let score = score(vec![event(61, 0, 96, 0, 500_000)]);
        let model = synthetic_supported_model().expect("model");
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::ModelInsufficient);
        assert!(outcome.static_unmodeled_branches > 0);
    }

    #[test]
    fn explicit_exhaustive_single_note_rejection_can_produce_model_relative_negative() {
        let score = score(vec![event(60, 0, 96, 0, 500_000)]);
        let model = rejected_single_note_model();
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::NoRealizationUnderModel);
        assert_eq!(outcome.static_unmodeled_branches, 0);
    }

    #[test]
    fn missing_transition_evidence_is_model_insufficient_not_rejected() {
        let score = synthetic_score();
        let mut model = synthetic_supported_model().expect("model");
        model.transitions.clear();
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        assert_eq!(outcome.status, ExecutionPlanStatus::ModelInsufficient);
        assert!(outcome.transition_unmodeled_branches > 0);
    }

    #[test]
    fn bounded_search_reports_resource_limit_instead_of_negative() {
        let score = synthetic_score();
        let model = synthetic_supported_model().expect("model");
        let outcome = search_execution_plan(&score, Some(&model), 0);
        assert_eq!(outcome.status, ExecutionPlanStatus::ResourceLimit);
    }

    #[test]
    fn independent_replay_rejects_foreign_score_fingerprint() {
        let score = synthetic_score();
        let model = synthetic_supported_model().expect("model");
        let outcome = search_execution_plan(&score, Some(&model), DEFAULT_STATE_BUDGET);
        let mut plan = outcome.plan.expect("plan");
        plan.score_fingerprint_sha256 = "foreign-score".to_string();
        let replay = replay_execution_plan(&score, &model, &plan);
        assert_eq!(replay.status, WitnessReplayStatus::Rejected);
        assert_eq!(replay.reason, "score-fingerprint-mismatch");
    }

    #[test]
    fn runtime_integrity_exercises_search_replay_and_fail_closed_statuses() {
        let summary = verify_runtime_witness_integrity().expect("runtime integrity");
        assert!(summary.contains("SyntheticWitness yes"));
        assert!(summary.contains("IndependentReplay VERIFIED"));
        assert!(summary.contains("ProductiveCapabilityModel no"));
    }
}
