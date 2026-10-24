//! r299 read-only reference-assumption capability model authority.
//!
//! This model is deliberately not empirical pianist truth. It is an independently identified,
//! SHA-256-bound stress model used only for model-relative solver falsification. The score demand
//! inventory does not define its answers. Static support is a whole-configuration assumption:
//! monotonic simultaneous finger order on a standard 88-key keyboard with an outer span no larger
//! than one octave. Transition timing is intentionally relaxed to a zero lower bound in this stage,
//! so r299 cannot support a tempo-feasibility or human-playability claim.

use crate::audit_only::pianistic_capability_model_binding::validate_reference_assumption_binding;
use crate::audit_only::pianistic_configuration_constraint_kernel::{
    Finger, Hand, KeyboardGeometry, StaticCapabilityDecision, StaticConfigurationRequest,
};
use crate::audit_only::pianistic_execution_score::{self, ProjectionStatus};
use crate::sha256;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-REFERENCE-ASSUMPTION-CAPABILITY-MODEL-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticReferenceAssumptionCapabilityModel";
pub const SCHEMA: &str = "pianistic-reference-assumption-capability-model-v1";
pub const MODEL_ID: &str = "etuschlopf-reference-assumption-standard88-static-v1";
pub const MODEL_SOURCE_SHA256: &str = "4d6eedf088607e4e9bc44a53483df1238b2e8ee9d6a1346db89dbf6f93d518b0";
pub const ASSUMPTION_CONTRACT: &str =
    "MODEL_RELATIVE_ONLY; whole-configuration monotonic simultaneous finger order; outer span <= 12 semitones; transition timing relaxed to zero lower bound; no human playability authority";
pub const MAX_OUTER_SPAN_SEMITONES: i32 = 12;
pub const RELAXED_TRANSITION_MINIMUM_TIME_US: u64 = 0;
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

const MODEL_SOURCE_DESCRIPTOR: &str = r#"model_id=etuschlopf-reference-assumption-standard88-static-v1
evidence_basis=REFERENCE_ASSUMPTION
claim_boundary=MODEL_RELATIVE_ONLY
keyboard=standard_88_key
static_policy=whole_configuration_monotonic_finger_order_and_outer_span
outer_span_limit_semitones=12
transition_policy=TEMPORALLY_RELAXED_ZERO_LOWER_BOUND
minimum_transition_time_us=0
score_demand_used_to_define_model=no
human_playability_authority=no
interpretation=solver_stress_model_not_empirical_pianist_capability
context_reference_1=doi:10.1016/j.jhsa.2006.02.009
context_reference_2=doi:10.1016/j.apergo.2021.103518
context_reference_3=doi:10.1016/j.apergo.2025.104709
"#;

fn finger_rank(finger: Finger) -> u8 {
    match finger {
        Finger::Thumb => 0,
        Finger::Index => 1,
        Finger::Middle => 2,
        Finger::Ring => 3,
        Finger::Little => 4,
    }
}

fn validate_request_shape(
    geometry: KeyboardGeometry,
    request: &StaticConfigurationRequest,
) -> Result<Vec<(i32, u8)>, &'static str> {
    if request.assignments.is_empty() {
        return Ok(Vec::new());
    }
    if request.assignments.len() > 5 {
        return Err("more-than-five-simultaneous-keys-on-one-hand");
    }

    let mut fingers = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut ordered = Vec::with_capacity(request.assignments.len());
    for assignment in &request.assignments {
        if geometry.key(assignment.midi).is_none() {
            return Err("key-outside-standard-88");
        }
        if !fingers.insert(assignment.finger) {
            return Err("multi-key-per-finger-unmodeled-v1");
        }
        if !keys.insert(assignment.midi) {
            return Err("multi-finger-one-key-unmodeled-v1");
        }
        ordered.push((assignment.midi, finger_rank(assignment.finger)));
    }
    ordered.sort_by_key(|item| item.0);
    Ok(ordered)
}

pub(crate) fn assess_reference_static_configuration(
    geometry: KeyboardGeometry,
    request: &StaticConfigurationRequest,
) -> (StaticCapabilityDecision, &'static str) {
    let ordered = match validate_request_shape(geometry, request) {
        Ok(ordered) => ordered,
        Err(reason) => return (StaticCapabilityDecision::Unmodeled, reason),
    };
    if ordered.is_empty() {
        return (
            StaticCapabilityDecision::Supported,
            "empty-hand-has-no-static-demand",
        );
    }

    let lowest = ordered.first().map(|item| item.0).unwrap_or(0);
    let highest = ordered.last().map(|item| item.0).unwrap_or(lowest);
    if highest.saturating_sub(lowest) > MAX_OUTER_SPAN_SEMITONES {
        return (
            StaticCapabilityDecision::Rejected,
            "reference-assumption-outer-span-exceeds-one-octave",
        );
    }

    let monotonic = match request.hand {
        Hand::Right => ordered.windows(2).all(|pair| pair[0].1 < pair[1].1),
        Hand::Left => ordered.windows(2).all(|pair| pair[0].1 > pair[1].1),
    };
    if !monotonic {
        return (
            StaticCapabilityDecision::Rejected,
            "reference-assumption-simultaneous-finger-order-crossing",
        );
    }

    (
        StaticCapabilityDecision::Supported,
        "reference-assumption-whole-configuration-supported",
    )
}

pub(crate) fn assess_reference_transition(
    geometry: KeyboardGeometry,
    from: &StaticConfigurationRequest,
    to: &StaticConfigurationRequest,
) -> (StaticCapabilityDecision, u64, &'static str) {
    if from.hand != to.hand {
        return (
            StaticCapabilityDecision::Unmodeled,
            RELAXED_TRANSITION_MINIMUM_TIME_US,
            "transition-hand-identity-mismatch",
        );
    }
    let (from_decision, _) = assess_reference_static_configuration(geometry, from);
    let (to_decision, _) = assess_reference_static_configuration(geometry, to);
    match (from_decision, to_decision) {
        (StaticCapabilityDecision::Supported, StaticCapabilityDecision::Supported) => (
            StaticCapabilityDecision::Supported,
            RELAXED_TRANSITION_MINIMUM_TIME_US,
            "reference-assumption-temporally-relaxed-transition",
        ),
        (StaticCapabilityDecision::Rejected, _) | (_, StaticCapabilityDecision::Rejected) => (
            StaticCapabilityDecision::Rejected,
            RELAXED_TRANSITION_MINIMUM_TIME_US,
            "reference-assumption-transition-endpoint-rejected",
        ),
        _ => (
            StaticCapabilityDecision::Unmodeled,
            RELAXED_TRANSITION_MINIMUM_TIME_US,
            "reference-assumption-transition-endpoint-unmodeled",
        ),
    }
}

fn verify_runtime_integrity() -> Result<String, String> {
    if sha256::hex(MODEL_SOURCE_DESCRIPTOR.as_bytes()) != MODEL_SOURCE_SHA256 {
        return Err("reference-assumption-model-source-sha256-mismatch".to_string());
    }
    let claim_boundary = validate_reference_assumption_binding(
        MODEL_ID,
        MODEL_SOURCE_SHA256,
        "independent-reference-assumption-descriptor-with-published-biomechanics-context",
        ASSUMPTION_CONTRACT,
    )?;
    if claim_boundary != "MODEL_RELATIVE_ONLY" {
        return Err("reference-assumption-claim-boundary-mismatch".to_string());
    }

    let geometry = KeyboardGeometry::standard_88_key();
    let right_octave = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Thumb,
                midi: 60,
            },
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Little,
                midi: 72,
            },
        ],
    };
    let left_octave = StaticConfigurationRequest {
        hand: Hand::Left,
        assignments: vec![
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Little,
                midi: 48,
            },
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Thumb,
                midi: 60,
            },
        ],
    };
    let crossed = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Index,
                midi: 60,
            },
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Thumb,
                midi: 64,
            },
        ],
    };
    let too_wide = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Thumb,
                midi: 60,
            },
            crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment {
                finger: Finger::Little,
                midi: 73,
            },
        ],
    };

    if assess_reference_static_configuration(geometry, &right_octave).0
        != StaticCapabilityDecision::Supported
        || assess_reference_static_configuration(geometry, &left_octave).0
            != StaticCapabilityDecision::Supported
        || assess_reference_static_configuration(geometry, &crossed).0
            != StaticCapabilityDecision::Rejected
        || assess_reference_static_configuration(geometry, &too_wide).0
            != StaticCapabilityDecision::Rejected
    {
        return Err("reference-assumption-static-runtime-integrity-failed".to_string());
    }

    let transition = assess_reference_transition(geometry, &right_octave, &right_octave);
    if transition.0 != StaticCapabilityDecision::Supported
        || transition.1 != RELAXED_TRANSITION_MINIMUM_TIME_US
    {
        return Err("reference-assumption-transition-runtime-integrity-failed".to_string());
    }

    Ok(format!(
        "PASS; ModelSourceSha256 verified; BindingStatus BOUND_REFERENCE_ASSUMPTION; ClaimBoundary {claim_boundary}; StaticPolicy whole_configuration_monotonic_outer_span_12; TransitionPolicy TEMPORALLY_RELAXED_ZERO_LOWER_BOUND"
    ))
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("===========================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    let _ = writeln!(
        out,
        "ModelIdentity: ModelId {MODEL_ID}; ModelSourceSha256 {MODEL_SOURCE_SHA256}; EvidenceBasis REFERENCE_ASSUMPTION; ClaimBoundary MODEL_RELATIVE_ONLY."
    );
    out.push_str("IndependenceBoundary: the model descriptor is fixed independently from the score-demand inventory. ScoreDemandUsedToDefineModel no; demand signatures remain questions, not model answers.\n");
    let _ = writeln!(
        out,
        "StaticAssumption: standard-88-key; whole simultaneous hand configuration; monotonic finger order; OuterSpanLimitSemitones {MAX_OUTER_SPAN_SEMITONES}. This is a reference assumption, not empirical reach truth."
    );
    let _ = writeln!(
        out,
        "TransitionAssumption: TEMPORALLY_RELAXED_ZERO_LOWER_BOUND; MinimumTransitionTimeUs {RELAXED_TRANSITION_MINIMUM_TIME_US}; no tempo-feasibility truth from this capability model."
    );
    out.push_str("PublishedContextOnly: doi:10.1016/j.jhsa.2006.02.009; doi:10.1016/j.apergo.2021.103518; doi:10.1016/j.apergo.2025.104709. These references motivate explicit hand-size/keyboard-size separation but do not authorize the model thresholds.\n");

    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticReferenceAssumptionCapabilityModel runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "ReferenceModelRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("ScoreContextAvailable: no; WitnessIntegration none; PlayabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let score = pianistic_execution_score::project_file(&PathBuf::from(format!("{title}.musicxml")));
    if score.status != ProjectionStatus::Complete {
        let _ = writeln!(
            out,
            "ScoreContextAvailable: no; Reason {}; WitnessIntegration none; PlayabilityClaim {PLAYABILITY_CLAIM}.\n",
            score.reason,
        );
        return out;
    }
    let _ = writeln!(
        out,
        "ScoreContextAvailable: yes; MusicalExecutionSha256 {}; SourceExecutionEvents {}; ModelSourceIndependentFromScore yes; IntegrationStateOwner PianisticReferenceAssumptionWitnessIntegration; IntegrationStateClaim none; PlayabilityClaim {PLAYABILITY_CLAIM}.",
        score.musical_execution_fingerprint_sha256,
        score.execution_events.len(),
    );
    out.push_str("AuthorityScopedNonClaims: no empirical pianist profile, no human reach truth, no transition-time truth, no score-level hand assignment, no score-level finger assignment, no witness claim, no obstruction claim, no tempo frontier, no repair candidate. This authority defines only the model-relative assumption; integration status and integration results are owned by PianisticReferenceAssumptionWitnessIntegration.\n\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit_only::pianistic_configuration_constraint_kernel::FingerKeyAssignment;

    fn request(hand: Hand, values: &[(Finger, i32)]) -> StaticConfigurationRequest {
        StaticConfigurationRequest {
            hand,
            assignments: values
                .iter()
                .map(|(finger, midi)| FingerKeyAssignment {
                    finger: *finger,
                    midi: *midi,
                })
                .collect(),
        }
    }

    #[test]
    fn model_source_descriptor_sha256_is_stable() {
        assert_eq!(sha256::hex(MODEL_SOURCE_DESCRIPTOR.as_bytes()), MODEL_SOURCE_SHA256);
    }

    #[test]
    fn right_hand_octave_is_supported_by_reference_assumption() {
        let geometry = KeyboardGeometry::standard_88_key();
        let req = request(Hand::Right, &[(Finger::Thumb, 60), (Finger::Little, 72)]);
        assert_eq!(
            assess_reference_static_configuration(geometry, &req).0,
            StaticCapabilityDecision::Supported
        );
    }

    #[test]
    fn left_hand_octave_uses_reverse_pitch_finger_order() {
        let geometry = KeyboardGeometry::standard_88_key();
        let req = request(Hand::Left, &[(Finger::Little, 48), (Finger::Thumb, 60)]);
        assert_eq!(
            assess_reference_static_configuration(geometry, &req).0,
            StaticCapabilityDecision::Supported
        );
    }

    #[test]
    fn crossed_simultaneous_finger_order_is_rejected_model_relatively() {
        let geometry = KeyboardGeometry::standard_88_key();
        let req = request(Hand::Right, &[(Finger::Index, 60), (Finger::Thumb, 64)]);
        assert_eq!(
            assess_reference_static_configuration(geometry, &req).0,
            StaticCapabilityDecision::Rejected
        );
    }

    #[test]
    fn beyond_octave_outer_span_is_rejected_model_relatively() {
        let geometry = KeyboardGeometry::standard_88_key();
        let req = request(Hand::Right, &[(Finger::Thumb, 60), (Finger::Little, 73)]);
        assert_eq!(
            assess_reference_static_configuration(geometry, &req).0,
            StaticCapabilityDecision::Rejected
        );
    }

    #[test]
    fn translated_octave_configuration_has_same_reference_decision() {
        let geometry = KeyboardGeometry::standard_88_key();
        let c = request(Hand::Right, &[(Finger::Thumb, 48), (Finger::Little, 60)]);
        let d = request(Hand::Right, &[(Finger::Thumb, 61), (Finger::Little, 73)]);
        assert_eq!(
            assess_reference_static_configuration(geometry, &c).0,
            assess_reference_static_configuration(geometry, &d).0
        );
    }

    #[test]
    fn transition_time_is_explicitly_relaxed_not_empirical() {
        let geometry = KeyboardGeometry::standard_88_key();
        let from = request(Hand::Right, &[(Finger::Thumb, 60), (Finger::Little, 72)]);
        let to = request(Hand::Right, &[(Finger::Thumb, 62), (Finger::Little, 74)]);
        let assessment = assess_reference_transition(geometry, &from, &to);
        assert_eq!(assessment.0, StaticCapabilityDecision::Supported);
        assert_eq!(assessment.1, 0);
        assert_eq!(
            assessment.2,
            "reference-assumption-temporally-relaxed-transition"
        );
    }
}
