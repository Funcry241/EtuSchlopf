//! r300 read-only integration of the r299 reference-assumption model with the execution demand topology; r618 reintroduces the rejected r616/r617 Same-Key static-precheck closure directly on green r615.
//!
//! A model-relative negative result is allowed only when an exact necessary condition fails.
//! The first integration stage therefore exhausts every left/right partition of every simultaneous
//! physical-key boundary under the r299 whole-hand static authority. If one boundary has no supported
//! two-hand partition, no global v1 witness can exist under that reference assumption and the expensive
//! global search is logically unnecessary. This is model-relative falsification only; it is not a human
//! playability claim and it does not mutate score, render, layout, or writer state.

use crate::audit_only::pianistic_configuration_constraint_kernel::{
    Finger, FingerKeyAssignment, Hand, KeyboardGeometry, StaticCapabilityDecision,
    StaticConfigurationRequest,
};
use crate::audit_only::pianistic_execution_plan_witness::{
    build_boundaries, build_demand_topology, DemandTopology, PhysicalKeyObligation,
};
use crate::audit_only::pianistic_execution_score::{self, PianisticExecutionScore, ProjectionStatus};
use crate::audit_only::pianistic_reference_assumption_model::{
    assess_reference_static_configuration, MODEL_ID, MODEL_SOURCE_SHA256,
};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-REFERENCE-ASSUMPTION-WITNESS-INTEGRATION-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticReferenceAssumptionWitnessIntegration";
pub const SCHEMA: &str = "pianistic-reference-assumption-witness-integration-v1";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

const RIGHT_FINGERS_ASCENDING: [Finger; 5] = [
    Finger::Thumb,
    Finger::Index,
    Finger::Middle,
    Finger::Ring,
    Finger::Little,
];
const LEFT_FINGERS_ASCENDING_PITCH: [Finger; 5] = [
    Finger::Little,
    Finger::Ring,
    Finger::Middle,
    Finger::Index,
    Finger::Thumb,
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct StaticBoundaryObstruction {
    tick: i64,
    active_midi: Vec<i32>,
    partitions_checked: usize,
    minimum_reference_hand_clusters: usize,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IntegrationAudit {
    projection_status: String,
    reason: String,
    source_execution_events: usize,
    physical_key_obligations: usize,
    boundary_count: usize,
    static_boundaries_checked: usize,
    obstruction: Option<StaticBoundaryObstruction>,
    model_relative_execution_plan_status: String,
    global_search_disposition: String,
}

fn canonical_request(hand: Hand, pitches: &[i32]) -> StaticConfigurationRequest {
    let mut sorted = pitches.to_vec();
    sorted.sort_unstable();
    let fingers = match hand {
        Hand::Left => &LEFT_FINGERS_ASCENDING_PITCH,
        Hand::Right => &RIGHT_FINGERS_ASCENDING,
    };
    let assignments = sorted
        .iter()
        .zip(fingers.iter())
        .map(|(midi, finger)| FingerKeyAssignment {
            finger: *finger,
            midi: *midi,
        })
        .collect();
    StaticConfigurationRequest { hand, assignments }
}

fn reference_hand_supported(geometry: KeyboardGeometry, hand: Hand, pitches: &[i32]) -> bool {
    if pitches.len() > 5 {
        return false;
    }
    assess_reference_static_configuration(geometry, &canonical_request(hand, pitches)).0
        == StaticCapabilityDecision::Supported
}

fn two_hand_reference_partition_exists(active_midi: &[i32]) -> (bool, usize) {
    if active_midi.iter().any(|midi| !(21..=108).contains(midi)) {
        return (false, 0);
    }
    let distinct_midi = active_midi.iter().copied().collect::<BTreeSet<_>>();
    if distinct_midi.len() > 10 {
        return (false, 0);
    }
    let distinct_midi = distinct_midi.into_iter().collect::<Vec<_>>();
    let geometry = KeyboardGeometry::standard_88_key();
    let partition_count = 1usize.checked_shl(distinct_midi.len() as u32).unwrap_or(0);
    let mut checked = 0usize;
    for mask in 0..partition_count {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (index, midi) in distinct_midi.iter().copied().enumerate() {
            if mask & (1usize << index) == 0 {
                left.push(midi);
            } else {
                right.push(midi);
            }
        }
        checked = checked.saturating_add(1);
        if reference_hand_supported(geometry, Hand::Left, &left)
            && reference_hand_supported(geometry, Hand::Right, &right)
        {
            return (true, checked);
        }
    }
    (false, checked)
}

fn minimum_reference_hand_clusters(active_midi: &[i32]) -> usize {
    if active_midi.is_empty() {
        return 0;
    }
    let mut pitches = active_midi.to_vec();
    pitches.sort_unstable();
    pitches.dedup();
    let count = pitches.len();
    let mut dp = vec![usize::MAX; count + 1];
    dp[count] = 0;
    for index in (0..count).rev() {
        let mut end = index;
        while end < count
            && end.saturating_sub(index) < 5
            && pitches[end].saturating_sub(pitches[index]) <= 12
        {
            if dp[end + 1] != usize::MAX {
                dp[index] = dp[index].min(dp[end + 1].saturating_add(1));
            }
            end = end.saturating_add(1);
        }
    }
    dp[0]
}

fn active_midi_for_boundary(
    active: &[usize],
    obligations: &[PhysicalKeyObligation],
) -> Result<Vec<i32>, String> {
    let mut out = Vec::with_capacity(active.len());
    for obligation_id in active {
        let obligation = obligations
            .get(*obligation_id)
            .ok_or_else(|| format!("boundary-obligation-id-out-of-range:{obligation_id}"))?;
        out.push(obligation.midi);
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

fn first_static_obstruction(topology: &DemandTopology) -> Result<(usize, Option<StaticBoundaryObstruction>), String> {
    let boundaries = build_boundaries(&topology.obligations)?;
    let mut checked_boundaries = 0usize;
    for boundary in boundaries {
        checked_boundaries = checked_boundaries.saturating_add(1);
        let active_midi = active_midi_for_boundary(&boundary.active, &topology.obligations)?;
        let (supported, partitions_checked) = two_hand_reference_partition_exists(&active_midi);
        if supported {
            continue;
        }
        let reason = "no-supported-two-hand-static-partition-under-reference-assumption".to_string();
        return Ok((
            checked_boundaries,
            Some(StaticBoundaryObstruction {
                tick: boundary.tick,
                minimum_reference_hand_clusters: minimum_reference_hand_clusters(&active_midi),
                active_midi,
                partitions_checked,
                reason,
            }),
        ));
    }
    Ok((checked_boundaries, None))
}

fn integrate_score(score: &PianisticExecutionScore) -> IntegrationAudit {
    if score.status != ProjectionStatus::Complete {
        return IntegrationAudit {
            projection_status: "MODEL_INPUT_UNAVAILABLE".to_string(),
            reason: score.reason.clone(),
            source_execution_events: score.execution_events.len(),
            physical_key_obligations: 0,
            boundary_count: 0,
            static_boundaries_checked: 0,
            obstruction: None,
            model_relative_execution_plan_status: "NOT_EVALUATED".to_string(),
            global_search_disposition: "NOT_RUN_MODEL_INPUT_UNAVAILABLE".to_string(),
        };
    }
    let topology = match build_demand_topology(score) {
        Ok(topology) => topology,
        Err(reason) => {
            return IntegrationAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE".to_string(),
                reason,
                source_execution_events: score.execution_events.len(),
                physical_key_obligations: 0,
                boundary_count: 0,
                static_boundaries_checked: 0,
                obstruction: None,
                model_relative_execution_plan_status: "NOT_EVALUATED".to_string(),
                global_search_disposition: "NOT_RUN_MODEL_INPUT_UNAVAILABLE".to_string(),
            };
        }
    };
    let boundary_count = topology.boundary_count;
    let source_execution_events = topology.source_execution_events;
    let physical_key_obligations = topology.obligations.len();
    match first_static_obstruction(&topology) {
        Ok((static_boundaries_checked, Some(obstruction))) => IntegrationAudit {
            projection_status: "COMPLETE".to_string(),
            reason: obstruction.reason.clone(),
            source_execution_events,
            physical_key_obligations,
            boundary_count,
            static_boundaries_checked,
            obstruction: Some(obstruction),
            model_relative_execution_plan_status:
                "NO_REALIZATION_UNDER_REFERENCE_ASSUMPTION".to_string(),
            global_search_disposition:
                "SKIPPED_EXACT_STATIC_NECESSARY_CONDITION_FAILED".to_string(),
        },
        Ok((static_boundaries_checked, None)) => IntegrationAudit {
            projection_status: "COMPLETE".to_string(),
            reason: "all-static-boundaries-have-supported-two-hand-partition".to_string(),
            source_execution_events,
            physical_key_obligations,
            boundary_count,
            static_boundaries_checked,
            obstruction: None,
            model_relative_execution_plan_status: "STATIC_PRECHECK_PASSED_GLOBAL_WITNESS_REQUIRED".to_string(),
            global_search_disposition: "DEFERRED_GLOBAL_WITNESS_SEARCH_REQUIRED".to_string(),
        },
        Err(reason) => IntegrationAudit {
            projection_status: "MODEL_INPUT_UNAVAILABLE".to_string(),
            reason,
            source_execution_events,
            physical_key_obligations,
            boundary_count,
            static_boundaries_checked: 0,
            obstruction: None,
            model_relative_execution_plan_status: "NOT_EVALUATED".to_string(),
            global_search_disposition: "NOT_RUN_STATIC_PRECHECK_ERROR".to_string(),
        },
    }
}

fn verify_runtime_integrity() -> Result<String, String> {
    let feasible = [48, 55, 60, 67];
    let impossible = [35, 49, 59, 63];
    let duplicate_same_key = [60, 60];
    let (feasible_supported, _) = two_hand_reference_partition_exists(&feasible);
    let (impossible_supported, impossible_checked) = two_hand_reference_partition_exists(&impossible);
    let (duplicate_supported, duplicate_checked) =
        two_hand_reference_partition_exists(&duplicate_same_key);
    if !feasible_supported {
        return Err("reference-integration-feasible-control-rejected".to_string());
    }
    if !duplicate_supported || duplicate_checked != 1 {
        return Err("same-key-multiplicity-not-deferred-from-static-precheck".to_string());
    }
    if impossible_supported || impossible_checked != 16 {
        return Err("reference-integration-impossible-control-not-exhausted".to_string());
    }
    if minimum_reference_hand_clusters(&impossible) != 3 {
        return Err("reference-integration-cluster-count-mismatch".to_string());
    }
    Ok(format!(
        "PASS; ModelId {MODEL_ID}; ModelSourceSha256 {MODEL_SOURCE_SHA256}; ExactTwoHandPartitionEnumeration yes; SameKeyStaticPrecheckDeferred yes; HumanPlayabilityAuthority no"
    ))
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("=================================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; InputAuthority: PianisticExecutionScoreAuthority+PianisticExecutionPlanWitnessDemandTopology+PianisticReferenceAssumptionCapabilityModel; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    let _ = writeln!(
        out,
        "ModelBinding: ModelId {MODEL_ID}; ModelSourceSha256 {MODEL_SOURCE_SHA256}; EvidenceBasis REFERENCE_ASSUMPTION; ClaimBoundary MODEL_RELATIVE_ONLY; HumanPlayabilityAuthority no."
    );
    out.push_str("ProofContract: every simultaneous DISTINCT-MIDI boundary is exhaustively partitioned across labelled left/right hands; each hand partition is checked by the r299 whole-configuration static authority. Same-Key obligation multiplicity is removed from this static precheck and delegated to the execution witness Same-Key adapter. Failure of the remaining distinct-key necessary condition proves that no global execution witness exists under the reference assumption. Search-budget exhaustion is not used for this negative result.\n");
    out.push_str("ScopeBoundary: static simultaneous distinct-key allocation only; Same-Key overlap technique belongs to the witness adapter; transition timing remains relaxed to zero lower bound; no empirical pianist claim, no tempo frontier, no repair candidate.\n");
    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticReferenceAssumptionWitnessIntegration runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "ReferenceWitnessIntegrationRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason score-title-missing; ModelRelativeExecutionPlanStatus NOT_EVALUATED; HumanPlayabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let score = pianistic_execution_score::project_file(&PathBuf::from(format!("{title}.musicxml")));
    let audit = integrate_score(&score);
    let _ = writeln!(
        out,
        "InputProjectionStatus: {}; Reason {}; SourceExecutionEvents {}; PhysicalKeyObligations {}; BoundaryCount {}; StaticBoundariesChecked {}.",
        audit.projection_status,
        audit.reason,
        audit.source_execution_events,
        audit.physical_key_obligations,
        audit.boundary_count,
        audit.static_boundaries_checked,
    );
    if let Some(obstruction) = &audit.obstruction {
        let midi = obstruction
            .active_midi
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let _ = writeln!(
            out,
            "FirstStaticObstruction: Tick {}; ActiveMidi [{}]; ActiveKeyCount {}; PartitionsChecked {}; MinimumReferenceHandClusters {}; AvailableHands 2; ObstructionClass EXACT_STATIC_TWO_HAND_PARTITION_EMPTY.",
            obstruction.tick,
            midi,
            obstruction.active_midi.len(),
            obstruction.partitions_checked,
            obstruction.minimum_reference_hand_clusters,
        );
    } else {
        out.push_str("FirstStaticObstruction: none.\n");
    }
    let _ = writeln!(
        out,
        "ModelRelativeExecutionPlanStatus: {}; GlobalSearchDisposition {}; IndependentWitnessReplay NOT_RUN; HumanPlayabilityClaim {PLAYABILITY_CLAIM}.",
        audit.model_relative_execution_plan_status,
        audit.global_search_disposition,
    );
    out.push_str("InterpretationBoundary: NO_REALIZATION_UNDER_REFERENCE_ASSUMPTION is a falsification of this explicit reference model/score combination only. It is not a claim that every human pianist or every alternative capability model must fail. A score mutation is not authorized by this audit result.\n\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_split_has_reference_two_hand_partition() {
        assert!(two_hand_reference_partition_exists(&[48, 55, 60, 67]).0);
    }

    #[test]
    fn three_span_clusters_have_no_two_hand_partition() {
        let (supported, checked) = two_hand_reference_partition_exists(&[35, 49, 59, 63]);
        assert!(!supported);
        assert_eq!(checked, 16);
    }

    #[test]
    fn three_span_clusters_report_three_required_reference_hands() {
        assert_eq!(minimum_reference_hand_clusters(&[35, 49, 59, 63]), 3);
    }

    #[test]
    fn duplicate_active_key_is_deferred_from_static_partition_to_same_key_adapter() {
        let (supported, checked) = two_hand_reference_partition_exists(&[60, 60]);
        assert!(supported);
        assert_eq!(checked, 1);
    }

    #[test]
    fn runtime_integrity_exercises_positive_and_negative_controls() {
        assert!(verify_runtime_integrity().is_ok());
    }

    #[test]
    fn report_keeps_human_playability_claim_outside_reference_result() {
        let text = build_for_score_title(None);
        assert!(text.contains("HumanPlayabilityClaim NOT_EVALUATED"));
        assert!(text.contains("HumanPlayabilityAuthority no"));
    }
}
