//! r297 read-only capability-evidence demand bridge for the constructive piano proof.
//!
//! This owner does not declare any configuration or transition playable. It projects the real
//! execution demand through a deliberately non-authoritative source-staff affinity probe in order
//! to enumerate calibration questions. Staff 1/2 are used only as probe affinities; a failed probe
//! is evidence that staff cannot be treated as physical hand truth.

use crate::sha256;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;

use super::pianistic_configuration_constraint_kernel::{
    Finger, FingerKeyAssignment, Hand, KeySurface, KeyboardGeometry, StaticConfigurationRequest,
    StaticConfigurationSignature, configuration_signature,
};
use super::pianistic_execution_plan_witness::{
    BoundarySlice, DemandTopology, PhysicalKeyObligation, TransitionSignature, build_boundaries,
    build_demand_topology, transition_signature,
};
use super::pianistic_execution_score::{
    PianisticExecutionEvent, PianisticExecutionScore, ProjectionStatus, TempoPoint,
    project_file,
};

pub const SECTION: &str = "PIANISTIC-CAPABILITY-EVIDENCE-DEMAND-PROBE-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticCapabilityEvidenceDemandProbe";
pub const SCHEMA: &str = "pianistic-capability-evidence-demand-probe-v1";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ProbeSlot {
    hand: Hand,
    finger: Finger,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransitionDemandEvidence {
    occurrences: usize,
    minimum_available_time_us: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProbeAudit {
    status: &'static str,
    reason: &'static str,
    source_execution_events: usize,
    physical_key_obligations: usize,
    affinity_left_obligations: usize,
    affinity_right_obligations: usize,
    unresolved_affinity_obligations: usize,
    boundary_count: usize,
    over_five_key_hand_boundaries: usize,
    hold_finger_reassignment_conflicts: usize,
    transition_chain_breaks: usize,
    unique_static_configuration_demands: usize,
    static_configuration_occurrences: usize,
    unique_transition_demands: usize,
    transition_occurrences: usize,
    evidence_demand_sha256: String,
}

#[derive(Debug, Clone)]
struct BoundaryProbe {
    requests: BTreeMap<Hand, Option<StaticConfigurationRequest>>,
    assignments: BTreeMap<usize, ProbeSlot>,
    unresolved_hands: BTreeSet<Hand>,
}

fn affinity_hand(
    score: &PianisticExecutionScore,
    obligation: &PhysicalKeyObligation,
) -> Option<Hand> {
    let mut resolved = None;
    for source_index in &obligation.source_event_indices {
        let event = score.execution_events.get(*source_index)?;
        let candidate = match event.source_staff.trim() {
            "1" => Hand::Right,
            "2" => Hand::Left,
            _ => return None,
        };
        if resolved.is_some_and(|previous| previous != candidate) {
            return None;
        }
        resolved = Some(candidate);
    }
    resolved
}

fn canonical_fingers(count: usize, hand: Hand) -> Option<Vec<Finger>> {
    let mut fingers = match count {
        0 => Vec::new(),
        1 => vec![Finger::Middle],
        2 => vec![Finger::Thumb, Finger::Little],
        3 => vec![Finger::Thumb, Finger::Middle, Finger::Little],
        4 => vec![Finger::Thumb, Finger::Index, Finger::Ring, Finger::Little],
        5 => vec![
            Finger::Thumb,
            Finger::Index,
            Finger::Middle,
            Finger::Ring,
            Finger::Little,
        ],
        _ => return None,
    };
    if hand == Hand::Left {
        fingers.reverse();
    }
    Some(fingers)
}

fn probe_boundary(
    topology: &DemandTopology,
    affinities: &[Option<Hand>],
    boundary: &BoundarySlice,
) -> BoundaryProbe {
    let mut requests = BTreeMap::new();
    let mut assignments = BTreeMap::new();
    let mut unresolved_hands = BTreeSet::new();

    for hand in [Hand::Left, Hand::Right] {
        let mut ids = boundary
            .active
            .iter()
            .copied()
            .filter(|id| affinities.get(*id).copied().flatten() == Some(hand))
            .collect::<Vec<_>>();
        ids.sort_by_key(|id| {
            topology
                .obligations
                .get(*id)
                .map(|obligation| (obligation.midi, obligation.id))
                .unwrap_or((i32::MAX, usize::MAX))
        });

        let Some(fingers) = canonical_fingers(ids.len(), hand) else {
            requests.insert(hand, None);
            unresolved_hands.insert(hand);
            continue;
        };

        let mut request_assignments = Vec::with_capacity(ids.len());
        for (obligation_id, finger) in ids.into_iter().zip(fingers) {
            let Some(obligation) = topology.obligations.get(obligation_id) else {
                requests.insert(hand, None);
                unresolved_hands.insert(hand);
                request_assignments.clear();
                break;
            };
            assignments.insert(obligation_id, ProbeSlot { hand, finger });
            request_assignments.push(FingerKeyAssignment {
                finger,
                midi: obligation.midi,
            });
        }
        if unresolved_hands.contains(&hand) {
            continue;
        }
        requests.insert(
            hand,
            Some(StaticConfigurationRequest {
                hand,
                assignments: request_assignments,
            }),
        );
    }

    BoundaryProbe {
        requests,
        assignments,
        unresolved_hands,
    }
}

fn request_signature(
    geometry: KeyboardGeometry,
    request: &StaticConfigurationRequest,
) -> Option<StaticConfigurationSignature> {
    if request.assignments.is_empty() {
        None
    } else {
        configuration_signature(geometry, request)
    }
}

fn collect_probe(score: &PianisticExecutionScore) -> Result<ProbeAudit, String> {
    let topology = build_demand_topology(score)?;
    let boundaries = build_boundaries(&topology.obligations)?;
    let geometry = KeyboardGeometry::standard_88_key();

    let affinities = topology
        .obligations
        .iter()
        .map(|obligation| affinity_hand(score, obligation))
        .collect::<Vec<_>>();
    let affinity_left_obligations = affinities
        .iter()
        .filter(|hand| **hand == Some(Hand::Left))
        .count();
    let affinity_right_obligations = affinities
        .iter()
        .filter(|hand| **hand == Some(Hand::Right))
        .count();
    let unresolved_affinity_obligations = affinities.iter().filter(|hand| hand.is_none()).count();

    let mut static_demands = BTreeMap::<StaticConfigurationSignature, usize>::new();
    let mut transition_demands = BTreeMap::<TransitionSignature, TransitionDemandEvidence>::new();
    let mut previous_assignments = BTreeMap::<usize, ProbeSlot>::new();
    let mut previous_requests = BTreeMap::<Hand, StaticConfigurationRequest>::new();
    let mut last_nonempty = BTreeMap::<Hand, StaticConfigurationRequest>::new();
    let mut empty_since = BTreeMap::<Hand, u64>::new();
    let mut previous_boundary_time = None;
    let mut over_five_key_hand_boundaries = 0usize;
    let mut hold_finger_reassignment_conflicts = 0usize;
    let mut transition_chain_breaks = 0usize;

    for boundary in &boundaries {
        let probe = probe_boundary(&topology, &affinities, boundary);
        over_five_key_hand_boundaries = over_five_key_hand_boundaries
            .saturating_add(probe.unresolved_hands.len());

        for (obligation_id, current_slot) in &probe.assignments {
            if boundary.active.contains(obligation_id)
                && previous_assignments
                    .get(obligation_id)
                    .is_some_and(|previous_slot| previous_slot != current_slot)
            {
                hold_finger_reassignment_conflicts =
                    hold_finger_reassignment_conflicts.saturating_add(1);
            }
        }

        for hand in [Hand::Left, Hand::Right] {
            let Some(current_option) = probe.requests.get(&hand) else {
                transition_chain_breaks = transition_chain_breaks.saturating_add(1);
                previous_requests.remove(&hand);
                last_nonempty.remove(&hand);
                empty_since.remove(&hand);
                continue;
            };
            let Some(current) = current_option.as_ref() else {
                transition_chain_breaks = transition_chain_breaks.saturating_add(1);
                previous_requests.remove(&hand);
                last_nonempty.remove(&hand);
                empty_since.remove(&hand);
                continue;
            };

            if let Some(signature) = request_signature(geometry, current) {
                *static_demands.entry(signature).or_default() += 1;
            }

            if current.assignments.is_empty() {
                if let Some(previous) = previous_requests.get(&hand).cloned()
                    && !previous.assignments.is_empty() {
                        last_nonempty.insert(hand, previous);
                        empty_since.insert(hand, boundary.time_us_floor);
                    }
                previous_requests.insert(hand, current.clone());
                continue;
            }

            let transition_from_and_available = if let Some(previous) = previous_requests
                .get(&hand)
                .filter(|request| !request.assignments.is_empty())
                .cloned()
            {
                let available = previous_boundary_time
                    .and_then(|time| boundary.time_us_floor.checked_sub(time))
                    .unwrap_or(0);
                Some((previous, available))
            } else if let (Some(previous), Some(since)) = (
                last_nonempty.get(&hand).cloned(),
                empty_since.get(&hand).copied(),
            ) {
                Some((
                    previous,
                    boundary.time_us_floor.saturating_sub(since),
                ))
            } else {
                None
            };

            if let Some((previous, available)) = transition_from_and_available
                && previous != *current {
                    let signature = transition_signature(geometry, &previous, current)
                        .ok_or_else(|| "probe-transition-signature-unavailable".to_string())?;
                    transition_demands
                        .entry(signature)
                        .and_modify(|evidence| {
                            evidence.occurrences = evidence.occurrences.saturating_add(1);
                            evidence.minimum_available_time_us =
                                evidence.minimum_available_time_us.min(available);
                        })
                        .or_insert(TransitionDemandEvidence {
                            occurrences: 1,
                            minimum_available_time_us: available,
                        });
                }
            previous_requests.insert(hand, current.clone());
            last_nonempty.insert(hand, current.clone());
            empty_since.remove(&hand);
        }

        previous_assignments = probe.assignments;
        previous_boundary_time = Some(boundary.time_us_floor);
    }

    let static_configuration_occurrences = static_demands.values().copied().sum();
    let transition_occurrences = transition_demands
        .values()
        .map(|evidence| evidence.occurrences)
        .sum();
    let evidence_demand_sha256 = evidence_demand_fingerprint(
        &score.musical_execution_fingerprint_sha256,
        &static_demands,
        &transition_demands,
        unresolved_affinity_obligations,
        over_five_key_hand_boundaries,
        hold_finger_reassignment_conflicts,
        transition_chain_breaks,
    );

    let complete = unresolved_affinity_obligations == 0
        && over_five_key_hand_boundaries == 0
        && hold_finger_reassignment_conflicts == 0
        && transition_chain_breaks == 0;
    Ok(ProbeAudit {
        status: if complete { "COMPLETE" } else { "INCOMPLETE" },
        reason: if complete {
            "canonical-source-affinity-probe-forms-v1-consistent-calibration-path"
        } else {
            "canonical-source-affinity-probe-is-not-a-v1-witness"
        },
        source_execution_events: topology.source_execution_events,
        physical_key_obligations: topology.obligations.len(),
        affinity_left_obligations,
        affinity_right_obligations,
        unresolved_affinity_obligations,
        boundary_count: topology.boundary_count,
        over_five_key_hand_boundaries,
        hold_finger_reassignment_conflicts,
        transition_chain_breaks,
        unique_static_configuration_demands: static_demands.len(),
        static_configuration_occurrences,
        unique_transition_demands: transition_demands.len(),
        transition_occurrences,
        evidence_demand_sha256,
    })
}

fn evidence_demand_fingerprint(
    score_fingerprint: &str,
    static_demands: &BTreeMap<StaticConfigurationSignature, usize>,
    transition_demands: &BTreeMap<TransitionSignature, TransitionDemandEvidence>,
    unresolved_affinity_obligations: usize,
    over_five_key_hand_boundaries: usize,
    hold_finger_reassignment_conflicts: usize,
    transition_chain_breaks: usize,
) -> String {
    let mut bytes = Vec::new();
    append_len_prefixed(&mut bytes, score_fingerprint.as_bytes());
    bytes.extend_from_slice(
        &u64::try_from(unresolved_affinity_obligations)
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    bytes.extend_from_slice(
        &u64::try_from(over_five_key_hand_boundaries)
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    bytes.extend_from_slice(
        &u64::try_from(hold_finger_reassignment_conflicts)
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    bytes.extend_from_slice(
        &u64::try_from(transition_chain_breaks)
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    for (signature, occurrences) in static_demands {
        append_static_signature(&mut bytes, signature);
        bytes.extend_from_slice(
            &u64::try_from(*occurrences)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
    }
    for (signature, evidence) in transition_demands {
        append_transition_signature(&mut bytes, signature);
        bytes.extend_from_slice(
            &u64::try_from(evidence.occurrences)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        bytes.extend_from_slice(&evidence.minimum_available_time_us.to_le_bytes());
    }
    sha256::hex(&bytes)
}

fn append_static_signature(out: &mut Vec<u8>, signature: &StaticConfigurationSignature) {
    out.push(hand_byte(signature.hand));
    out.extend_from_slice(
        &u64::try_from(signature.assignments.len())
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    for assignment in &signature.assignments {
        out.push(finger_byte(assignment.finger));
        out.extend_from_slice(&assignment.relative_semitones.to_le_bytes());
        out.push(surface_byte(assignment.surface));
    }
}

fn append_transition_signature(out: &mut Vec<u8>, signature: &TransitionSignature) {
    out.push(hand_byte(signature.hand));
    append_static_signature(out, &signature.from);
    append_static_signature(out, &signature.to);
    out.extend_from_slice(&signature.lowest_key_delta_semitones.to_le_bytes());
}

fn hand_byte(hand: Hand) -> u8 {
    match hand {
        Hand::Left => 0,
        Hand::Right => 1,
    }
}

fn finger_byte(finger: Finger) -> u8 {
    match finger {
        Finger::Thumb => 0,
        Finger::Index => 1,
        Finger::Middle => 2,
        Finger::Ring => 3,
        Finger::Little => 4,
    }
}

fn surface_byte(surface: KeySurface) -> u8 {
    match surface {
        KeySurface::White => 0,
        KeySurface::Black => 1,
    }
}

fn append_len_prefixed(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(
        &u64::try_from(bytes.len())
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    out.extend_from_slice(bytes);
}

fn synthetic_score(events: Vec<PianisticExecutionEvent>) -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "synthetic-capability-demand-probe".to_string(),
        source_path: PathBuf::from("synthetic.musicxml"),
        pitched_note_segments: events.len(),
        execution_events: events,
        tie_continuation_segments: 0,
        tempo_points: vec![TempoPoint {
            tick: 0,
            milli_bpm: 120_000,
        }],
        terminal_tick: 192,
        terminal_physical_time_us_floor: Some(1_000_000),
        musical_execution_fingerprint_sha256: "synthetic-musical-fingerprint".to_string(),
        source_affinity_fingerprint_sha256: "synthetic-affinity-fingerprint".to_string(),
    }
}

fn synthetic_event(
    staff: &str,
    voice: &str,
    midi: i32,
    attack_tick: i64,
    release_tick: i64,
    attack_time_us_floor: u64,
    release_time_us_floor: u64,
) -> PianisticExecutionEvent {
    PianisticExecutionEvent {
        part_id: "P1".to_string(),
        source_staff: staff.to_string(),
        source_voice: voice.to_string(),
        midi,
        attack_tick,
        release_tick,
        attack_time_us_floor,
        release_time_us_floor,
        note_segments: 1,
    }
}

fn verify_runtime_integrity() -> Result<String, String> {
    let sequential = synthetic_score(vec![
        synthetic_event("1", "1", 60, 0, 96, 0, 500_000),
        synthetic_event("1", "9", 62, 96, 192, 500_000, 1_000_000),
    ]);
    let sequential_audit = collect_probe(&sequential)?;
    if sequential_audit.affinity_right_obligations != 2
        || sequential_audit.affinity_left_obligations != 0
        || sequential_audit.unresolved_affinity_obligations != 0
        || sequential_audit.unique_static_configuration_demands != 1
        || sequential_audit.unique_transition_demands != 1
        || sequential_audit.transition_occurrences != 1
    {
        return Err("synthetic-sequential-demand-probe-mismatch".to_string());
    }

    let six_right = synthetic_score(
        (0..6)
            .map(|index| {
                synthetic_event(
                    "1",
                    "1",
                    60 + index,
                    0,
                    96,
                    0,
                    500_000,
                )
            })
            .collect(),
    );
    let six_right_audit = collect_probe(&six_right)?;
    if six_right_audit.status != "INCOMPLETE"
        || six_right_audit.over_five_key_hand_boundaries == 0
    {
        return Err("source-affinity-over-five-probe-did-not-fail-closed".to_string());
    }

    Ok("PASS; SourceStaffAffinity probe_only_not_hand_truth; ExactDemandDedup yes; OverFiveProbeFailsClosed yes; CapabilityDecision none; PlayabilityClaim NOT_EVALUATED".to_string())
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("====================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; InputAuthority: PianisticExecutionScoreAuthority+PianisticExecutionPlanWitnessDemandTopology; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("ProbeBoundary: source staff is used only as a provenance affinity probe (staff 1 -> right, staff 2 -> left) to enumerate capability-evidence questions. StaffIsHand false; probe assignments are not search constraints and never create a playability claim.\n");
    out.push_str("CanonicalProbeFingering: deterministic calibration-question projection only; it is not a fingering recommendation, not CapabilityEvidence and not a Witness. A held obligation whose canonical probe finger changes is counted as a probe conflict instead of being silently reassigned.\n");
    out.push_str("FailClosedProbeSemantics: more than five simultaneous keys on one affinity hand, mixed/unmapped staff affinity, or a broken transition chain makes the probe incomplete. IncompleteProbe != unplayable and CompleteProbe != playable.\n");

    let integrity = verify_runtime_integrity()
        .unwrap_or_else(|reason| panic!("PianisticCapabilityEvidenceDemandProbe runtime integrity failed: {reason}"));
    let _ = writeln!(out, "ProbeRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; ProbeStatus NOT_EVALUATED; CapabilityModel none; PlayabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let path = PathBuf::from(format!("{title}.musicxml"));
    let score = project_file(&path);
    if score.status != ProjectionStatus::Complete {
        let _ = writeln!(
            out,
            "InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason {}; ProbeStatus NOT_EVALUATED; CapabilityModel none; PlayabilityClaim {PLAYABILITY_CLAIM}.\n",
            score.reason,
        );
        return out;
    }

    let audit = match collect_probe(&score) {
        Ok(audit) => audit,
        Err(reason) => {
            let _ = writeln!(
                out,
                "InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason {reason}; ProbeStatus NOT_EVALUATED; CapabilityModel none; PlayabilityClaim {PLAYABILITY_CLAIM}.\n"
            );
            return out;
        }
    };
    let _ = writeln!(
        out,
        "InputProjectionStatus: COMPLETE; SourceExecutionEvents {}; PhysicalKeyObligations {}; BoundaryCount {}; AffinityLeftObligations {}; AffinityRightObligations {}; UnresolvedAffinityObligations {}.",
        audit.source_execution_events,
        audit.physical_key_obligations,
        audit.boundary_count,
        audit.affinity_left_obligations,
        audit.affinity_right_obligations,
        audit.unresolved_affinity_obligations,
    );
    let _ = writeln!(
        out,
        "ProbeStatus: {}; Reason {}; OverFiveKeyHandBoundaries {}; HoldFingerReassignmentConflicts {}; TransitionChainBreaks {}; UniqueStaticConfigurationDemands {}; StaticConfigurationOccurrences {}; UniqueTransitionDemands {}; TransitionOccurrences {}; EvidenceDemandSha256 {}.",
        audit.status,
        audit.reason,
        audit.over_five_key_hand_boundaries,
        audit.hold_finger_reassignment_conflicts,
        audit.transition_chain_breaks,
        audit.unique_static_configuration_demands,
        audit.static_configuration_occurrences,
        audit.unique_transition_demands,
        audit.transition_occurrences,
        audit.evidence_demand_sha256,
    );
    out.push_str("Interpretation: the probe is a finite calibration-demand inventory, not a physical truth. In particular, an affinity hand with more than five simultaneous keys is a constructive counterexample to StaffIsHand and requires an alternative hand allocation in any later Witness.\n");
    out.push_str("NonClaims: no supported configuration, no rejected configuration, no transition minimum, no productive capability model, no score-level hand assignment, no score-level finger assignment, no feasibility claim, no obstruction claim, no tempo frontier, no repair candidate. PlayabilityClaim NOT_EVALUATED.\n\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn staff_affinity_maps_only_one_and_two_without_becoming_hand_truth() {
        let score = synthetic_score(vec![
            synthetic_event("1", "1", 60, 0, 96, 0, 500_000),
            synthetic_event("2", "2", 48, 0, 96, 0, 500_000),
        ]);
        let audit = collect_probe(&score).expect("probe");
        assert_eq!(audit.affinity_right_obligations, 1);
        assert_eq!(audit.affinity_left_obligations, 1);
        assert_eq!(audit.unresolved_affinity_obligations, 0);
    }

    #[test]
    fn exact_duplicate_with_mixed_staff_affinity_is_unresolved() {
        let score = synthetic_score(vec![
            synthetic_event("1", "1", 60, 0, 96, 0, 500_000),
            synthetic_event("2", "2", 60, 0, 96, 0, 500_000),
        ]);
        let audit = collect_probe(&score).expect("probe");
        assert_eq!(audit.physical_key_obligations, 1);
        assert_eq!(audit.unresolved_affinity_obligations, 1);
        assert_eq!(audit.status, "INCOMPLETE");
    }

    #[test]
    fn translated_single_note_demands_share_one_relative_static_signature() {
        let score = synthetic_score(vec![
            synthetic_event("1", "1", 60, 0, 96, 0, 500_000),
            synthetic_event("1", "1", 62, 96, 192, 500_000, 1_000_000),
        ]);
        let audit = collect_probe(&score).expect("probe");
        assert_eq!(audit.unique_static_configuration_demands, 1);
        assert_eq!(audit.static_configuration_occurrences, 2);
    }

    #[test]
    fn sequential_translation_creates_one_explicit_transition_demand() {
        let score = synthetic_score(vec![
            synthetic_event("1", "1", 60, 0, 96, 0, 500_000),
            synthetic_event("1", "1", 62, 96, 192, 500_000, 1_000_000),
        ]);
        let audit = collect_probe(&score).expect("probe");
        assert_eq!(audit.unique_transition_demands, 1);
        assert_eq!(audit.transition_occurrences, 1);
    }

    #[test]
    fn six_same_affinity_keys_fail_probe_without_playability_claim() {
        let score = synthetic_score(
            (0..6)
                .map(|index| {
                    synthetic_event(
                        "1",
                        "1",
                        60 + index,
                        0,
                        96,
                        0,
                        500_000,
                    )
                })
                .collect(),
        );
        let audit = collect_probe(&score).expect("probe");
        assert_eq!(audit.status, "INCOMPLETE");
        assert_eq!(audit.over_five_key_hand_boundaries, 1);
    }

    #[test]
    fn canonical_probe_detects_held_finger_reassignment_instead_of_hiding_it() {
        let score = synthetic_score(vec![
            synthetic_event("1", "1", 60, 0, 192, 0, 1_000_000),
            synthetic_event("1", "1", 64, 96, 192, 500_000, 1_000_000),
        ]);
        let audit = collect_probe(&score).expect("probe");
        assert!(audit.hold_finger_reassignment_conflicts > 0);
        assert_eq!(audit.status, "INCOMPLETE");
    }

    #[test]
    fn source_voice_relabel_does_not_change_evidence_demand_fingerprint() {
        let first = synthetic_score(vec![
            synthetic_event("1", "1", 60, 0, 96, 0, 500_000),
            synthetic_event("1", "1", 62, 96, 192, 500_000, 1_000_000),
        ]);
        let second = synthetic_score(vec![
            synthetic_event("1", "99", 60, 0, 96, 0, 500_000),
            synthetic_event("1", "7", 62, 96, 192, 500_000, 1_000_000),
        ]);
        let left = collect_probe(&first).expect("first");
        let right = collect_probe(&second).expect("second");
        assert_eq!(left.evidence_demand_sha256, right.evidence_demand_sha256);
    }

    #[test]
    fn report_keeps_probe_separate_from_capability_and_playability() {
        let text = build_for_score_title(None);
        assert!(text.contains("StaffIsHand false"));
        assert!(text.contains("CapabilityModel none"));
        assert!(text.contains("PlayabilityClaim NOT_EVALUATED"));
        assert!(!text.contains("REALIZATION_WITNESS_FOUND"));
    }
}
