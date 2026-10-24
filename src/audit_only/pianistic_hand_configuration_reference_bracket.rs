//! r611 read-only hand-configuration reference bracket audit.
//!
//! This audit binds a second, explicitly identified REFERENCE_ASSUMPTION beside r299. The new
//! structural-ceiling model is intentionally maximally permissive inside the existing v1 hand/
//! finger ontology: standard 88-key geometry, at most five distinct keys per hand and monotonic
//! simultaneous finger order, but no outer-span ceiling. It is not empirical human capability.
//!
//! Comparing r299 with this structural ceiling separates model failures caused specifically by the
//! one-octave reference span from failures that remain even after that span assumption is removed.
//! Same-key overlapping obligations remain a separate temporal/technique demand owned by r608.

use crate::audit_only::pianistic_capability_model_binding::validate_reference_assumption_binding;
use crate::audit_only::pianistic_configuration_constraint_kernel::{
    Finger, FingerKeyAssignment, Hand, KeyboardGeometry, StaticCapabilityDecision,
    StaticConfigurationRequest,
};
use crate::audit_only::pianistic_execution_plan_witness::{
    build_boundaries, build_demand_topology, PhysicalKeyObligation,
};
use crate::audit_only::pianistic_execution_score::{
    self, PianisticExecutionScore, ProjectionStatus,
};
use crate::audit_only::pianistic_reference_assumption_model::assess_reference_static_configuration;
use crate::sha256;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-HAND-CONFIGURATION-REFERENCE-BRACKET-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticHandConfigurationReferenceBracketAudit";
pub const SCHEMA: &str = "pianistic-hand-configuration-reference-bracket-v1";
pub const MODEL_ID: &str = "etuschlopf-reference-assumption-five-finger-no-span-v1";
pub const MODEL_SOURCE_SHA256: &str = "73b70489937c8987f2a949b25704da3b5eb2e9603e1463f86e94a51e6768efdf";
pub const ASSUMPTION_CONTRACT: &str =
    "MODEL_RELATIVE_ONLY; standard 88-key geometry; at most five distinct keys per hand; monotonic simultaneous finger order; no outer-span ceiling; no transition capability; no human playability authority";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

const MODEL_SOURCE_DESCRIPTOR: &str = r#"model_id=etuschlopf-reference-assumption-five-finger-no-span-v1
evidence_basis=REFERENCE_ASSUMPTION
claim_boundary=MODEL_RELATIVE_ONLY
keyboard=standard_88_key
static_policy=whole_configuration_monotonic_finger_order
maximum_distinct_keys_per_hand=5
outer_span_limit_semitones=none
transition_policy=NOT_MODELED
score_demand_used_to_define_model=no
human_playability_authority=no
interpretation=structural_ceiling_reference_inside_v1_hand_finger_ontology_not_empirical_pianist_capability
"#;

const RIGHT_ASCENDING: [Finger; 5] = [
    Finger::Thumb,
    Finger::Index,
    Finger::Middle,
    Finger::Ring,
    Finger::Little,
];
const LEFT_ASCENDING: [Finger; 5] = [
    Finger::Little,
    Finger::Ring,
    Finger::Middle,
    Finger::Index,
    Finger::Thumb,
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryBracket {
    tick: i64,
    active_distinct_midi: Vec<i32>,
    same_key_overlap_excess: usize,
    structural_partitions: usize,
    r299_supported_partitions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BracketAudit {
    projection_status: &'static str,
    reason: String,
    source_execution_events: usize,
    physical_key_obligations: usize,
    boundary_count: usize,
    evaluated_boundaries: usize,
    structural_supported_boundaries: usize,
    r299_supported_boundaries: usize,
    span_sensitive_boundaries: usize,
    structurally_unresolved_boundaries: usize,
    same_key_overlap_boundaries: usize,
    same_key_overlap_excess_obligations: usize,
    minimum_r299_supported_partitions_on_supported_boundary: Option<usize>,
    maximum_structural_partitions: usize,
    span_sensitive_samples: Vec<BoundaryBracket>,
    unresolved_reason_histogram: BTreeMap<&'static str, usize>,
}

fn canonical_request(hand: Hand, midi: &[i32]) -> StaticConfigurationRequest {
    let fingers = match hand {
        Hand::Left => &LEFT_ASCENDING,
        Hand::Right => &RIGHT_ASCENDING,
    };
    StaticConfigurationRequest {
        hand,
        assignments: midi
            .iter()
            .copied()
            .zip(fingers.iter().copied())
            .map(|(midi, finger)| FingerKeyAssignment { finger, midi })
            .collect(),
    }
}

fn assess_structural_ceiling(
    geometry: KeyboardGeometry,
    request: &StaticConfigurationRequest,
) -> (StaticCapabilityDecision, &'static str) {
    if request.assignments.len() > 5 {
        return (
            StaticCapabilityDecision::Rejected,
            "structural-ceiling-more-than-five-distinct-keys-on-one-hand",
        );
    }
    let mut fingers = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut ordered = Vec::new();
    for assignment in &request.assignments {
        if geometry.key(assignment.midi).is_none() {
            return (
                StaticCapabilityDecision::Unmodeled,
                "structural-ceiling-key-outside-standard-88",
            );
        }
        if !fingers.insert(assignment.finger) {
            return (
                StaticCapabilityDecision::Unmodeled,
                "structural-ceiling-multi-key-per-finger-unmodeled",
            );
        }
        if !keys.insert(assignment.midi) {
            return (
                StaticCapabilityDecision::Unmodeled,
                "structural-ceiling-multi-finger-one-key-unmodeled",
            );
        }
        let rank = match assignment.finger {
            Finger::Thumb => 0u8,
            Finger::Index => 1,
            Finger::Middle => 2,
            Finger::Ring => 3,
            Finger::Little => 4,
        };
        ordered.push((assignment.midi, rank));
    }
    ordered.sort_by_key(|item| item.0);
    let monotonic = match request.hand {
        Hand::Right => ordered.windows(2).all(|pair| pair[0].1 < pair[1].1),
        Hand::Left => ordered.windows(2).all(|pair| pair[0].1 > pair[1].1),
    };
    if !monotonic {
        return (
            StaticCapabilityDecision::Rejected,
            "structural-ceiling-simultaneous-finger-order-crossing",
        );
    }
    (
        StaticCapabilityDecision::Supported,
        "structural-ceiling-whole-configuration-supported",
    )
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
    Ok(out)
}

fn bracket_boundary(active_midi: &[i32], tick: i64) -> Result<BoundaryBracket, &'static str> {
    if active_midi.iter().any(|midi| !(21..=108).contains(midi)) {
        return Err("key-outside-standard-88");
    }
    let original_len = active_midi.len();
    let mut distinct = active_midi.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    let same_key_overlap_excess = original_len.saturating_sub(distinct.len());
    if distinct.len() > 10 {
        return Err("more-than-ten-simultaneous-distinct-keys");
    }

    let geometry = KeyboardGeometry::standard_88_key();
    let partition_count = 1usize.checked_shl(distinct.len() as u32).unwrap_or(0);
    let mut structural_partitions = 0usize;
    let mut r299_supported_partitions = 0usize;

    for mask in 0..partition_count {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (index, midi) in distinct.iter().copied().enumerate() {
            if mask & (1usize << index) == 0 {
                left.push(midi);
            } else {
                right.push(midi);
            }
        }
        if left.len() > 5 || right.len() > 5 {
            continue;
        }
        let left_request = canonical_request(Hand::Left, &left);
        let right_request = canonical_request(Hand::Right, &right);
        let left_structural = assess_structural_ceiling(geometry, &left_request).0;
        let right_structural = assess_structural_ceiling(geometry, &right_request).0;
        if left_structural != StaticCapabilityDecision::Supported
            || right_structural != StaticCapabilityDecision::Supported
        {
            continue;
        }
        structural_partitions = structural_partitions.saturating_add(1);

        let left_r299 = assess_reference_static_configuration(geometry, &left_request).0;
        let right_r299 = assess_reference_static_configuration(geometry, &right_request).0;
        if left_r299 == StaticCapabilityDecision::Supported
            && right_r299 == StaticCapabilityDecision::Supported
        {
            r299_supported_partitions = r299_supported_partitions.saturating_add(1);
        }
    }

    Ok(BoundaryBracket {
        tick,
        active_distinct_midi: distinct,
        same_key_overlap_excess,
        structural_partitions,
        r299_supported_partitions,
    })
}

fn audit_score(score: &PianisticExecutionScore) -> BracketAudit {
    if score.status != ProjectionStatus::Complete {
        return BracketAudit {
            projection_status: "MODEL_INPUT_UNAVAILABLE",
            reason: score.reason.clone(),
            source_execution_events: score.execution_events.len(),
            physical_key_obligations: 0,
            boundary_count: 0,
            evaluated_boundaries: 0,
            structural_supported_boundaries: 0,
            r299_supported_boundaries: 0,
            span_sensitive_boundaries: 0,
            structurally_unresolved_boundaries: 0,
            same_key_overlap_boundaries: 0,
            same_key_overlap_excess_obligations: 0,
            minimum_r299_supported_partitions_on_supported_boundary: None,
            maximum_structural_partitions: 0,
            span_sensitive_samples: Vec::new(),
            unresolved_reason_histogram: BTreeMap::new(),
        };
    }

    let topology = match build_demand_topology(score) {
        Ok(topology) => topology,
        Err(reason) => {
            return BracketAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: score.execution_events.len(),
                physical_key_obligations: 0,
                boundary_count: 0,
                evaluated_boundaries: 0,
                structural_supported_boundaries: 0,
                r299_supported_boundaries: 0,
                span_sensitive_boundaries: 0,
                structurally_unresolved_boundaries: 0,
                same_key_overlap_boundaries: 0,
                same_key_overlap_excess_obligations: 0,
                minimum_r299_supported_partitions_on_supported_boundary: None,
                maximum_structural_partitions: 0,
                span_sensitive_samples: Vec::new(),
                unresolved_reason_histogram: BTreeMap::new(),
            };
        }
    };
    let boundaries = match build_boundaries(&topology.obligations) {
        Ok(boundaries) => boundaries,
        Err(reason) => {
            return BracketAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: topology.source_execution_events,
                physical_key_obligations: topology.obligations.len(),
                boundary_count: topology.boundary_count,
                evaluated_boundaries: 0,
                structural_supported_boundaries: 0,
                r299_supported_boundaries: 0,
                span_sensitive_boundaries: 0,
                structurally_unresolved_boundaries: 0,
                same_key_overlap_boundaries: 0,
                same_key_overlap_excess_obligations: 0,
                minimum_r299_supported_partitions_on_supported_boundary: None,
                maximum_structural_partitions: 0,
                span_sensitive_samples: Vec::new(),
                unresolved_reason_histogram: BTreeMap::new(),
            };
        }
    };

    let mut evaluated_boundaries = 0usize;
    let mut structural_supported_boundaries = 0usize;
    let mut r299_supported_boundaries = 0usize;
    let mut span_sensitive_boundaries = 0usize;
    let mut structurally_unresolved_boundaries = 0usize;
    let mut same_key_overlap_boundaries = 0usize;
    let mut same_key_overlap_excess_obligations = 0usize;
    let mut minimum_r299_supported_partitions_on_supported_boundary = None::<usize>;
    let mut maximum_structural_partitions = 0usize;
    let mut span_sensitive_samples = Vec::new();
    let mut unresolved_reason_histogram = BTreeMap::<&'static str, usize>::new();

    for boundary in boundaries {
        let active_midi = match active_midi_for_boundary(&boundary.active, &topology.obligations) {
            Ok(active_midi) => active_midi,
            Err(reason) => {
                return BracketAudit {
                    projection_status: "MODEL_INPUT_UNAVAILABLE",
                    reason,
                    source_execution_events: topology.source_execution_events,
                    physical_key_obligations: topology.obligations.len(),
                    boundary_count: topology.boundary_count,
                    evaluated_boundaries,
                    structural_supported_boundaries,
                    r299_supported_boundaries,
                    span_sensitive_boundaries,
                    structurally_unresolved_boundaries,
                    same_key_overlap_boundaries,
                    same_key_overlap_excess_obligations,
                    minimum_r299_supported_partitions_on_supported_boundary,
                    maximum_structural_partitions,
                    span_sensitive_samples,
                    unresolved_reason_histogram,
                };
            }
        };
        match bracket_boundary(&active_midi, boundary.tick) {
            Ok(bracket) => {
                evaluated_boundaries = evaluated_boundaries.saturating_add(1);
                maximum_structural_partitions =
                    maximum_structural_partitions.max(bracket.structural_partitions);
                if bracket.same_key_overlap_excess > 0 {
                    same_key_overlap_boundaries = same_key_overlap_boundaries.saturating_add(1);
                    same_key_overlap_excess_obligations = same_key_overlap_excess_obligations
                        .saturating_add(bracket.same_key_overlap_excess);
                }
                if bracket.structural_partitions > 0 {
                    structural_supported_boundaries =
                        structural_supported_boundaries.saturating_add(1);
                } else {
                    structurally_unresolved_boundaries =
                        structurally_unresolved_boundaries.saturating_add(1);
                    *unresolved_reason_histogram
                        .entry("no-structural-ceiling-two-hand-partition")
                        .or_default() += 1;
                }

                if bracket.r299_supported_partitions > 0 {
                    r299_supported_boundaries = r299_supported_boundaries.saturating_add(1);
                    minimum_r299_supported_partitions_on_supported_boundary = Some(
                        minimum_r299_supported_partitions_on_supported_boundary.map_or(
                            bracket.r299_supported_partitions,
                            |current| current.min(bracket.r299_supported_partitions),
                        ),
                    );
                } else if bracket.structural_partitions > 0 {
                    span_sensitive_boundaries = span_sensitive_boundaries.saturating_add(1);
                    if span_sensitive_samples.len() < 12 {
                        span_sensitive_samples.push(bracket);
                    }
                }
            }
            Err(reason) => {
                structurally_unresolved_boundaries =
                    structurally_unresolved_boundaries.saturating_add(1);
                *unresolved_reason_histogram.entry(reason).or_default() += 1;
            }
        }
    }

    BracketAudit {
        projection_status: if structurally_unresolved_boundaries == 0 {
            "COMPLETE"
        } else {
            "PARTIAL_UNMODELED_BOUNDARIES"
        },
        reason: if structurally_unresolved_boundaries == 0 {
            "reference-hand-configuration-bracket-complete".to_string()
        } else {
            format!(
                "{structurally_unresolved_boundaries}-boundaries-outside-structural-ceiling-domain"
            )
        },
        source_execution_events: topology.source_execution_events,
        physical_key_obligations: topology.obligations.len(),
        boundary_count: topology.boundary_count,
        evaluated_boundaries,
        structural_supported_boundaries,
        r299_supported_boundaries,
        span_sensitive_boundaries,
        structurally_unresolved_boundaries,
        same_key_overlap_boundaries,
        same_key_overlap_excess_obligations,
        minimum_r299_supported_partitions_on_supported_boundary,
        maximum_structural_partitions,
        span_sensitive_samples,
        unresolved_reason_histogram,
    }
}

fn format_reason_histogram(histogram: &BTreeMap<&'static str, usize>) -> String {
    if histogram.is_empty() {
        return "none".to_string();
    }
    histogram
        .iter()
        .map(|(reason, count)| format!("{reason}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn verify_runtime_integrity() -> Result<String, String> {
    if sha256::hex(MODEL_SOURCE_DESCRIPTOR.as_bytes()) != MODEL_SOURCE_SHA256 {
        return Err("structural-ceiling-model-source-sha256-mismatch".to_string());
    }
    let claim_boundary = validate_reference_assumption_binding(
        MODEL_ID,
        MODEL_SOURCE_SHA256,
        "independent-structural-ceiling-reference-assumption",
        ASSUMPTION_CONTRACT,
    )?;
    if claim_boundary != "MODEL_RELATIVE_ONLY" {
        return Err("structural-ceiling-claim-boundary-mismatch".to_string());
    }

    let former_r299_obstruction = bracket_boundary(&[35, 49, 59, 63], 984)
        .map_err(|reason| format!("synthetic-bracket-failed:{reason}"))?;
    if former_r299_obstruction.structural_partitions == 0
        || former_r299_obstruction.r299_supported_partitions != 0
    {
        return Err("former-r299-obstruction-not-separated-by-bracket".to_string());
    }

    let ordinary = bracket_boundary(&[48, 55, 60, 67], 0)
        .map_err(|reason| format!("synthetic-ordinary-bracket-failed:{reason}"))?;
    if ordinary.structural_partitions == 0 || ordinary.r299_supported_partitions == 0 {
        return Err("ordinary-static-configuration-not-supported-by-both-references".to_string());
    }

    let eleven = bracket_boundary(&(40..=50).collect::<Vec<_>>(), 0);
    if eleven != Err("more-than-ten-simultaneous-distinct-keys") {
        return Err("structural-ceiling-cardinality-control-mismatch".to_string());
    }

    Ok("PASS; IndependentReferenceBinding yes; StructuralCeilingFiveDistinctKeysPerHand yes; MonotonicFingerOrder yes; OuterSpanLimit none; r299Comparison yes; SameKeyTechniqueSeparated yes; HumanPlayabilityAuthority no".to_string())
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("=================================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; EvidenceBasis REFERENCE_ASSUMPTION; ClaimBoundary MODEL_RELATIVE_ONLY; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    let _ = writeln!(
        out,
        "StructuralCeilingReference: ModelId {MODEL_ID}; ModelSourceSha256 {MODEL_SOURCE_SHA256}; Standard88 yes; MaximumDistinctKeysPerHand 5; MonotonicFingerOrder yes; OuterSpanLimitSemitones none; TransitionCapability NOT_BOUND."
    );
    out.push_str("BracketContract: every simultaneous distinct-key boundary is exhaustively partitioned across labelled left/right hands. r299 support and the independent structural-ceiling reference are counted separately. A boundary supported only by the structural ceiling is span-assumption-sensitive; it is not thereby human-supported.\n");
    out.push_str("SameKeyBoundary: overlapping non-identical obligations on one physical key remain an r608 temporal/technique demand and are counted but never resolved by this static hand bracket.\n");
    out.push_str("InterpretationBoundary: the structural ceiling is deliberately more permissive than any empirically calibrated hand model because it removes outer-span limits. It is a model-relative upper bracket inside the v1 five-finger ontology, not evidence that a human can realize the configuration.\n");
    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticHandConfigurationReferenceBracketAudit runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "ReferenceBracketRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason score-title-missing; HumanPlayabilityClaim NOT_EVALUATED; EmpiricalCalibration NOT_BOUND.\n\n");
        return out;
    };

    let score = pianistic_execution_score::project_file(&PathBuf::from(format!("{title}.musicxml")));
    let audit = audit_score(&score);
    let _ = writeln!(
        out,
        "InputProjectionStatus: {}; Reason {}; SourceExecutionEvents {}; PhysicalKeyObligations {}; BoundaryCount {}; EvaluatedBoundaries {}.",
        audit.projection_status,
        audit.reason,
        audit.source_execution_events,
        audit.physical_key_obligations,
        audit.boundary_count,
        audit.evaluated_boundaries,
    );
    let _ = writeln!(
        out,
        "HandConfigurationReferenceBracket: StructuralCeilingSupportedBoundaries {}; R299SupportedBoundaries {}; SpanAssumptionSensitiveBoundaries {}; StructurallyUnresolvedBoundaries {}; SameKeyOverlapBoundaries {}; SameKeyOverlapExcessObligations {}; MaximumStructuralPartitions {}; MinimumR299SupportedPartitionsOnSupportedBoundary {}; UnresolvedReasonHistogram [{}].",
        audit.structural_supported_boundaries,
        audit.r299_supported_boundaries,
        audit.span_sensitive_boundaries,
        audit.structurally_unresolved_boundaries,
        audit.same_key_overlap_boundaries,
        audit.same_key_overlap_excess_obligations,
        audit.maximum_structural_partitions,
        audit.minimum_r299_supported_partitions_on_supported_boundary
            .map_or_else(|| "none".to_string(), |value| value.to_string()),
        format_reason_histogram(&audit.unresolved_reason_histogram),
    );
    if audit.span_sensitive_samples.is_empty() {
        out.push_str("SpanAssumptionSensitiveSample: none.\n");
    } else {
        for sample in &audit.span_sensitive_samples {
            let midi = sample
                .active_distinct_midi
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            let _ = writeln!(
                out,
                "SpanAssumptionSensitiveSample: Tick {}; DistinctMidi [{}]; StructuralPartitions {}; R299SupportedPartitions {}; SameKeyOverlapExcess {}.",
                sample.tick,
                midi,
                sample.structural_partitions,
                sample.r299_supported_partitions,
                sample.same_key_overlap_excess,
            );
        }
    }
    let _ = writeln!(
        out,
        "CapabilityHandoff: empirical per-hand configuration evidence is still required to replace the model-relative bracket. Score-derived frontiers cannot self-authorize capability support."
    );
    let _ = writeln!(
        out,
        "HumanPlayabilityClaim {PLAYABILITY_CLAIM}; EmpiricalCalibration NOT_BOUND; HandTransitionCapability NOT_BOUND; FingerTransitionCapability NOT_BOUND; TempoFeasibility NOT_EVALUATED.\n"
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn former_r299_obstruction_is_supported_only_by_structural_ceiling() {
        let bracket = bracket_boundary(&[35, 49, 59, 63], 984).expect("bracket");
        assert!(bracket.structural_partitions > 0);
        assert_eq!(bracket.r299_supported_partitions, 0);
    }

    #[test]
    fn ordinary_boundary_is_supported_by_both_reference_models() {
        let bracket = bracket_boundary(&[48, 55, 60, 67], 0).expect("bracket");
        assert!(bracket.structural_partitions > 0);
        assert!(bracket.r299_supported_partitions > 0);
    }

    #[test]
    fn structural_ceiling_rejects_more_than_ten_distinct_keys() {
        assert_eq!(
            bracket_boundary(&(40..=50).collect::<Vec<_>>(), 0),
            Err("more-than-ten-simultaneous-distinct-keys")
        );
    }

    #[test]
    fn report_keeps_human_claim_and_empirical_calibration_unbound() {
        let text = build_for_score_title(None);
        assert!(text.contains("HumanPlayabilityClaim NOT_EVALUATED"));
        assert!(text.contains("EmpiricalCalibration NOT_BOUND"));
        assert!(text.contains("OuterSpanLimitSemitones none"));
    }
}
