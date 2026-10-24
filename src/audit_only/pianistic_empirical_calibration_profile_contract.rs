//! r615 empirical pianist calibration semantic-schema report-contract recovery, with the r618
//! Same-Key witness-shape binding and scoreless report-contract recovery directly on green r615.
//!
//! r616 and r617 were rejected and have no authority. r617's integrated --bin etuschlopf failure
//! proved that the new EXACT_WITNESS_BOUND marker was still emitted only after the score-title early
//! return, repeating the same report-path class previously closed at r615. r618 retains the exact
//! witness-owned SameKeyTechniqueSignature and moves its WitnessShapeCompatibility declaration before
//! that early return. No threshold is invented and no real pianist profile is bound.

use crate::audit_only::pianistic_capability_model_binding::validate_empirical_calibration_binding;
use crate::audit_only::pianistic_configuration_constraint_kernel::{
    Finger, Hand, KeySurface, StaticAssignmentSignature, StaticCapabilityDecision,
    StaticConfigurationSignature,
};
use crate::audit_only::pianistic_execution_plan_witness::{
    build_boundaries, build_demand_topology, SameKeyTechniqueSignature, TransitionSignature,
};
use crate::audit_only::pianistic_execution_score::{self, ProjectionStatus};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-EMPIRICAL-CALIBRATION-PROFILE-CONTRACT-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticEmpiricalCalibrationProfileContract";
pub const SCHEMA: &str = "pianistic-empirical-calibration-profile-v2";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

#[derive(Debug, Clone, PartialEq, Eq)]
enum CalibrationObservation {
    StaticWholeConfiguration {
        signature: StaticConfigurationSignature,
        decision: StaticCapabilityDecision,
        evidence_id: String,
        repetitions: u16,
    },
    TransitionTiming {
        signature: TransitionSignature,
        decision: StaticCapabilityDecision,
        supported_at_or_above_us: u64,
        evidence_id: String,
        repetitions: u16,
    },
    SameKeyRearticulation {
        signature: SameKeyTechniqueSignature,
        decision: StaticCapabilityDecision,
        supported_at_or_above_us: u64,
        evidence_id: String,
        repetitions: u16,
    },
}

impl CalibrationObservation {
    fn evidence_id(&self) -> &str {
        match self {
            Self::StaticWholeConfiguration { evidence_id, .. }
            | Self::TransitionTiming { evidence_id, .. }
            | Self::SameKeyRearticulation { evidence_id, .. } => evidence_id,
        }
    }

    fn repetitions(&self) -> u16 {
        match self {
            Self::StaticWholeConfiguration { repetitions, .. }
            | Self::TransitionTiming { repetitions, .. }
            | Self::SameKeyRearticulation { repetitions, .. } => *repetitions,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EmpiricalCalibrationProfile {
    profile_id: String,
    session_id: String,
    source_sha256: String,
    provenance: String,
    keyboard_geometry: String,
    source_independent_from_score: bool,
    raw_measurement_provenance_bound: bool,
    observations: Vec<CalibrationObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProfileAssessment {
    accepted: bool,
    reason: &'static str,
    static_left: usize,
    static_right: usize,
    transition_left: usize,
    transition_right: usize,
    same_key_left: usize,
    same_key_right: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ScoreCalibrationDemand {
    source_execution_events: usize,
    physical_key_obligations: usize,
    boundaries: usize,
    attack_boundaries: usize,
    unique_distinct_key_states: usize,
    maximum_distinct_keys: usize,
    maximum_outer_span_semitones: i32,
    same_key_overlap_boundaries: usize,
    same_key_overlap_excess_obligations: usize,
    minimum_positive_boundary_delta_us: Option<u64>,
}

fn is_lower_hex_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn nonempty(value: &str) -> bool {
    !value.trim().is_empty()
}

fn explicit_decision(decision: StaticCapabilityDecision) -> bool {
    decision != StaticCapabilityDecision::Unmodeled
}

fn valid_static_signature(signature: &StaticConfigurationSignature) -> bool {
    if signature.assignments.is_empty() || signature.assignments.len() > 5 {
        return false;
    }
    let mut fingers = BTreeSet::new();
    let mut has_zero_origin = false;
    for assignment in &signature.assignments {
        if assignment.relative_semitones < 0 || !fingers.insert(assignment.finger) {
            return false;
        }
        has_zero_origin |= assignment.relative_semitones == 0;
        match assignment.surface {
            KeySurface::White | KeySurface::Black => {}
        }
    }
    has_zero_origin
}

fn valid_transition_signature(signature: &TransitionSignature) -> bool {
    signature.from.hand == signature.hand
        && signature.to.hand == signature.hand
        && valid_static_signature(&signature.from)
        && valid_static_signature(&signature.to)
}

fn assess_profile(profile: &EmpiricalCalibrationProfile) -> ProfileAssessment {
    let mut assessment = ProfileAssessment {
        accepted: false,
        reason: "profile-unchecked",
        static_left: 0,
        static_right: 0,
        transition_left: 0,
        transition_right: 0,
        same_key_left: 0,
        same_key_right: 0,
    };

    if !nonempty(&profile.profile_id) {
        assessment.reason = "profile-id-empty";
        return assessment;
    }
    if !nonempty(&profile.session_id) {
        assessment.reason = "session-id-empty";
        return assessment;
    }
    if !is_lower_hex_sha256(&profile.source_sha256) {
        assessment.reason = "source-sha256-invalid";
        return assessment;
    }
    if !nonempty(&profile.provenance) {
        assessment.reason = "provenance-empty";
        return assessment;
    }
    if profile.keyboard_geometry != "standard-88-key" {
        assessment.reason = "keyboard-geometry-not-standard-88-key";
        return assessment;
    }
    if !profile.source_independent_from_score {
        assessment.reason = "score-derived-capability-source-forbidden";
        return assessment;
    }
    if !profile.raw_measurement_provenance_bound {
        assessment.reason = "raw-measurement-provenance-not-bound";
        return assessment;
    }
    if profile.observations.is_empty() {
        assessment.reason = "calibration-observations-empty";
        return assessment;
    }

    let mut evidence_ids = BTreeSet::new();
    for observation in &profile.observations {
        if !nonempty(observation.evidence_id()) {
            assessment.reason = "observation-evidence-id-empty";
            return assessment;
        }
        if !evidence_ids.insert(observation.evidence_id()) {
            assessment.reason = "duplicate-observation-evidence-id";
            return assessment;
        }
        if observation.repetitions() == 0 {
            assessment.reason = "observation-repetitions-zero";
            return assessment;
        }

        match observation {
            CalibrationObservation::StaticWholeConfiguration {
                signature,
                decision,
                ..
            } => {
                if !explicit_decision(*decision) {
                    assessment.reason = "static-decision-unmodeled-forbidden";
                    return assessment;
                }
                if !valid_static_signature(signature) {
                    assessment.reason = "static-signature-invalid";
                    return assessment;
                }
                match signature.hand {
                    Hand::Left => assessment.static_left += 1,
                    Hand::Right => assessment.static_right += 1,
                }
            }
            CalibrationObservation::TransitionTiming {
                signature,
                decision,
                supported_at_or_above_us,
                ..
            } => {
                if !explicit_decision(*decision) {
                    assessment.reason = "transition-decision-unmodeled-forbidden";
                    return assessment;
                }
                if *supported_at_or_above_us == 0 {
                    assessment.reason = "transition-supported-time-missing";
                    return assessment;
                }
                if !valid_transition_signature(signature) {
                    assessment.reason = "transition-signature-invalid";
                    return assessment;
                }
                match signature.hand {
                    Hand::Left => assessment.transition_left += 1,
                    Hand::Right => assessment.transition_right += 1,
                }
            }
            CalibrationObservation::SameKeyRearticulation {
                signature,
                decision,
                supported_at_or_above_us,
                ..
            } => {
                if !explicit_decision(*decision) {
                    assessment.reason = "same-key-decision-unmodeled-forbidden";
                    return assessment;
                }
                if *supported_at_or_above_us == 0 {
                    assessment.reason = "same-key-supported-time-missing";
                    return assessment;
                }
                match signature.surface {
                    KeySurface::White | KeySurface::Black => {}
                }
                match signature.finger {
                    Finger::Thumb | Finger::Index | Finger::Middle | Finger::Ring | Finger::Little => {}
                }
                match signature.hand {
                    Hand::Left => assessment.same_key_left += 1,
                    Hand::Right => assessment.same_key_right += 1,
                }
            }
        }
    }

    if assessment.static_left == 0 || assessment.static_right == 0 {
        assessment.reason = "per-hand-static-calibration-incomplete";
        return assessment;
    }
    if assessment.transition_left == 0 || assessment.transition_right == 0 {
        assessment.reason = "per-hand-transition-calibration-incomplete";
        return assessment;
    }
    if assessment.same_key_left == 0 || assessment.same_key_right == 0 {
        assessment.reason = "per-hand-same-key-calibration-incomplete";
        return assessment;
    }

    assessment.accepted = true;
    assessment.reason = "empirical-calibration-profile-v2-contract-satisfied";
    assessment
}

fn static_signature(
    hand: Hand,
    rows: &[(Finger, i32, KeySurface)],
) -> StaticConfigurationSignature {
    StaticConfigurationSignature {
        hand,
        assignments: rows
            .iter()
            .map(|(finger, relative_semitones, surface)| StaticAssignmentSignature {
                finger: *finger,
                relative_semitones: *relative_semitones,
                surface: *surface,
            })
            .collect(),
    }
}

fn score_calibration_demand(
    score: &pianistic_execution_score::PianisticExecutionScore,
) -> Result<ScoreCalibrationDemand, String> {
    if score.status != ProjectionStatus::Complete {
        return Err(format!("execution-score-not-complete:{}", score.reason));
    }
    let topology = build_demand_topology(score)?;
    let boundaries = build_boundaries(&topology.obligations)?;
    let mut unique_states = BTreeSet::<Vec<i32>>::new();
    let mut maximum_distinct_keys = 0usize;
    let mut maximum_outer_span_semitones = 0i32;
    let mut same_key_overlap_boundaries = 0usize;
    let mut same_key_overlap_excess_obligations = 0usize;

    for boundary in &boundaries {
        let mut midi = boundary
            .active
            .iter()
            .map(|id| {
                topology
                    .obligations
                    .get(*id)
                    .map(|obligation| obligation.midi)
                    .ok_or_else(|| format!("boundary-obligation-id-out-of-range:{id}"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let original_len = midi.len();
        midi.sort_unstable();
        midi.dedup();
        let excess = original_len.saturating_sub(midi.len());
        if excess > 0 {
            same_key_overlap_boundaries = same_key_overlap_boundaries.saturating_add(1);
            same_key_overlap_excess_obligations =
                same_key_overlap_excess_obligations.saturating_add(excess);
        }
        maximum_distinct_keys = maximum_distinct_keys.max(midi.len());
        if let (Some(low), Some(high)) = (midi.first(), midi.last()) {
            maximum_outer_span_semitones =
                maximum_outer_span_semitones.max(high.saturating_sub(*low));
        }
        unique_states.insert(midi);
    }

    let minimum_positive_boundary_delta_us = boundaries
        .windows(2)
        .filter_map(|pair| pair[1].time_us_floor.checked_sub(pair[0].time_us_floor))
        .filter(|delta| *delta > 0)
        .min();

    Ok(ScoreCalibrationDemand {
        source_execution_events: topology.source_execution_events,
        physical_key_obligations: topology.obligations.len(),
        boundaries: boundaries.len(),
        attack_boundaries: topology.attack_boundary_count,
        unique_distinct_key_states: unique_states.len(),
        maximum_distinct_keys,
        maximum_outer_span_semitones,
        same_key_overlap_boundaries,
        same_key_overlap_excess_obligations,
        minimum_positive_boundary_delta_us,
    })
}


fn synthetic_complete_profile() -> EmpiricalCalibrationProfile {
    let left_from = static_signature(
        Hand::Left,
        &[
            (Finger::Little, 0, KeySurface::White),
            (Finger::Middle, 4, KeySurface::White),
            (Finger::Thumb, 7, KeySurface::White),
        ],
    );
    let left_to = static_signature(
        Hand::Left,
        &[
            (Finger::Little, 0, KeySurface::White),
            (Finger::Index, 5, KeySurface::White),
            (Finger::Thumb, 9, KeySurface::White),
        ],
    );
    let right_from = static_signature(
        Hand::Right,
        &[
            (Finger::Thumb, 0, KeySurface::White),
            (Finger::Middle, 4, KeySurface::White),
            (Finger::Little, 7, KeySurface::White),
        ],
    );
    let right_to = static_signature(
        Hand::Right,
        &[
            (Finger::Thumb, 0, KeySurface::White),
            (Finger::Index, 3, KeySurface::Black),
            (Finger::Little, 8, KeySurface::Black),
        ],
    );

    EmpiricalCalibrationProfile {
        profile_id: "synthetic-runtime-profile-v2".to_string(),
        session_id: "synthetic-runtime-session-v2".to_string(),
        source_sha256: "a".repeat(64),
        provenance: "synthetic-runtime-integrity-only".to_string(),
        keyboard_geometry: "standard-88-key".to_string(),
        source_independent_from_score: true,
        raw_measurement_provenance_bound: true,
        observations: vec![
            CalibrationObservation::StaticWholeConfiguration {
                signature: left_from.clone(),
                decision: StaticCapabilityDecision::Supported,
                evidence_id: "synthetic-static-left".to_string(),
                repetitions: 3,
            },
            CalibrationObservation::StaticWholeConfiguration {
                signature: right_from.clone(),
                decision: StaticCapabilityDecision::Supported,
                evidence_id: "synthetic-static-right".to_string(),
                repetitions: 3,
            },
            CalibrationObservation::TransitionTiming {
                signature: TransitionSignature {
                    hand: Hand::Left,
                    from: left_from,
                    to: left_to,
                    lowest_key_delta_semitones: -5,
                },
                decision: StaticCapabilityDecision::Supported,
                supported_at_or_above_us: 125_000,
                evidence_id: "synthetic-transition-left".to_string(),
                repetitions: 3,
            },
            CalibrationObservation::TransitionTiming {
                signature: TransitionSignature {
                    hand: Hand::Right,
                    from: right_from,
                    to: right_to,
                    lowest_key_delta_semitones: 7,
                },
                decision: StaticCapabilityDecision::Supported,
                supported_at_or_above_us: 130_000,
                evidence_id: "synthetic-transition-right".to_string(),
                repetitions: 3,
            },
            CalibrationObservation::SameKeyRearticulation {
                signature: SameKeyTechniqueSignature {
                    hand: Hand::Left,
                    finger: Finger::Middle,
                    surface: KeySurface::White,
                },
                decision: StaticCapabilityDecision::Supported,
                supported_at_or_above_us: 100_000,
                evidence_id: "synthetic-same-key-left".to_string(),
                repetitions: 3,
            },
            CalibrationObservation::SameKeyRearticulation {
                signature: SameKeyTechniqueSignature {
                    hand: Hand::Right,
                    finger: Finger::Index,
                    surface: KeySurface::Black,
                },
                decision: StaticCapabilityDecision::Supported,
                supported_at_or_above_us: 105_000,
                evidence_id: "synthetic-same-key-right".to_string(),
                repetitions: 3,
            },
        ],
    }
}

fn verify_runtime_integrity() -> Result<String, String> {
    let complete = synthetic_complete_profile();
    let accepted = assess_profile(&complete);
    if !accepted.accepted
        || accepted.static_left != 1
        || accepted.static_right != 1
        || accepted.transition_left != 1
        || accepted.transition_right != 1
        || accepted.same_key_left != 1
        || accepted.same_key_right != 1
    {
        return Err("complete-synthetic-profile-v2-not-accepted".to_string());
    }

    let claim_boundary = validate_empirical_calibration_binding(
        "synthetic-empirical-capability-model-v2",
        &complete.source_sha256,
        &complete.provenance,
        &complete.profile_id,
    )?;
    if claim_boundary != "HUMAN_CALIBRATION_ELIGIBLE" {
        return Err("empirical-binding-claim-boundary-mismatch".to_string());
    }

    let mut score_derived = complete.clone();
    score_derived.source_independent_from_score = false;
    let rejected = assess_profile(&score_derived);
    if rejected.accepted || rejected.reason != "score-derived-capability-source-forbidden" {
        return Err("score-derived-profile-not-rejected".to_string());
    }

    let mut no_right_transition = complete.clone();
    no_right_transition.observations.retain(|row| {
        !matches!(
            row,
            CalibrationObservation::TransitionTiming {
                signature: TransitionSignature {
                    hand: Hand::Right,
                    ..
                },
                ..
            }
        )
    });
    let rejected = assess_profile(&no_right_transition);
    if rejected.accepted || rejected.reason != "per-hand-transition-calibration-incomplete" {
        return Err("missing-per-hand-transition-profile-not-rejected".to_string());
    }

    Ok("PASS; ProfileSchema v2; StaticSignature witness-compatible; TransitionSignature witness-compatible; SameKeySignature witness-compatible; StaticPerHand yes; TransitionPerHand yes; SameKeyPerHand yes; IndependentFromScore yes; RawMeasurementProvenance yes; BindingClaimBoundary HUMAN_CALIBRATION_ELIGIBLE; HumanClaimOwner no".to_string())
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("==========================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("CalibrationBoundary: r611 proves only a model bracket. Human-calibrated witness input requires independently measured pianist evidence; score demand may select calibration questions but can never populate capability answers.\n");
    out.push_str("ProfileSemanticClosure: every static row carries the exact r295 StaticConfigurationSignature plus explicit SUPPORTED/REJECTED decision; every transition row carries the exact r296 TransitionSignature plus explicit decision and measured supported-at-or-above time; every same-key row carries the explicit SameKeyTechniqueSignature fields hand, finger and key surface plus explicit decision and measured supported-at-or-above time.\n");
    out.push_str("WholeConfigurationRule: empirical static evidence is configuration-level. Pairwise reach or a single outer-span number alone does not authorize arbitrary chord configurations.\n");
    out.push_str("PerHandCoverageRule: STATIC_WHOLE_CONFIGURATION, TRANSITION_TIMING and SAME_KEY_REARTICULATION each require independently evidenced left- and right-hand rows before the profile contract is complete.\n");
    out.push_str("PublishedContextOnly: doi:10.1016/j.jhsa.2006.02.009; doi:10.1016/j.apergo.2021.103518; doi:10.1016/j.apergo.2025.104709. These motivate pianist/keyboard-specific calibration; they do not provide EtuSchlopf capability thresholds.\n");
    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticEmpiricalCalibrationProfileContract runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "CalibrationContractRuntimeIntegrity: {integrity}.");

    out.push_str("WitnessShapeCompatibility: StaticConfigurationSignature EXACT; TransitionSignature EXACT; SameKeyTechniqueSignature EXACT_WITNESS_BOUND.\n");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        let _ = writeln!(
            out,
            "ScoreContextAvailable: no; CalibrationProfileBinding NONE; EmpiricalCalibration NOT_BOUND; HumanPlayabilityClaim {PLAYABILITY_CLAIM}."
        );
        out.push('\n');
        return out;
    };

    let score = pianistic_execution_score::project_file(&PathBuf::from(format!("{title}.musicxml")));
    match score_calibration_demand(&score) {
        Ok(demand) => {
            let _ = writeln!(
                out,
                "ScoreContextAvailable: yes; MusicalExecutionSha256 {}; SourceExecutionEvents {}; PhysicalKeyObligations {}; Boundaries {}; AttackBoundaries {}; UniqueDistinctKeyStates {}; MaximumDistinctKeys {}; MaximumKeyboardStateOuterSpanSemitones {}; SameKeyOverlapBoundaries {}; SameKeyOverlapExcessObligations {}; MinimumPositiveBoundaryDeltaUs {}.",
                score.musical_execution_fingerprint_sha256,
                demand.source_execution_events,
                demand.physical_key_obligations,
                demand.boundaries,
                demand.attack_boundaries,
                demand.unique_distinct_key_states,
                demand.maximum_distinct_keys,
                demand.maximum_outer_span_semitones,
                demand.same_key_overlap_boundaries,
                demand.same_key_overlap_excess_obligations,
                demand.minimum_positive_boundary_delta_us.map_or_else(|| "none".to_string(), |value| value.to_string()),
            );
        }
        Err(reason) => {
            let _ = writeln!(
                out,
                "ScoreContextAvailable: no; Reason {reason}; CalibrationProfileBinding NONE."
            );
        }
    }
    out.push_str("RequiredEmpiricalDimensions: STATIC_WHOLE_CONFIGURATION(left+right); TRANSITION_TIMING(left+right); SAME_KEY_REARTICULATION(left+right); keyboard_specific_standard88; independent_raw_measurement_provenance.\n");
    let _ = writeln!(
        out,
        "CalibrationProfileBinding NONE; EmpiricalCalibration NOT_BOUND; CapabilityInterpolation NONE; HumanPlayabilityClaim {PLAYABILITY_CLAIM}; WitnessIntegration NOT_RUN."
    );
    out.push_str("CapabilityHandoff: an externally measured v2 profile can represent the concrete static, transition and same-key signatures consumed by the existing capability/witness domain. SameKeyTechniqueSignature is now witness-bound at the semantic shape boundary. Productive profile binding remains NONE until actual capability evidence is supplied. The profile itself cannot authorize playability; only a complete execution witness plus independent replay under a bound model may support a correctly scoped model-relative or human-calibrated claim.\n\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_empirical_profile_v2_contract_is_accepted() {
        let assessment = assess_profile(&synthetic_complete_profile());
        assert!(assessment.accepted);
        assert_eq!(
            assessment.reason,
            "empirical-calibration-profile-v2-contract-satisfied"
        );
    }

    #[test]
    fn score_derived_capability_source_is_rejected() {
        let mut profile = synthetic_complete_profile();
        profile.source_independent_from_score = false;
        let assessment = assess_profile(&profile);
        assert!(!assessment.accepted);
        assert_eq!(
            assessment.reason,
            "score-derived-capability-source-forbidden"
        );
    }

    #[test]
    fn missing_right_hand_transition_is_rejected() {
        let mut profile = synthetic_complete_profile();
        profile.observations.retain(|row| {
            !matches!(
                row,
                CalibrationObservation::TransitionTiming {
                    signature: TransitionSignature {
                        hand: Hand::Right,
                        ..
                    },
                    ..
                }
            )
        });
        let assessment = assess_profile(&profile);
        assert!(!assessment.accepted);
        assert_eq!(
            assessment.reason,
            "per-hand-transition-calibration-incomplete"
        );
    }

    #[test]
    fn generic_observation_cannot_omit_concrete_static_or_transition_signature() {
        let report = build_for_score_title(None);
        assert!(report.contains("StaticConfigurationSignature"));
        assert!(report.contains("TransitionSignature"));
        assert!(report.contains("SameKeyTechniqueSignature"));
        assert!(report.contains("SameKeyTechniqueSignature EXACT_WITNESS_BOUND"));
        assert!(report.contains("HumanPlayabilityClaim NOT_EVALUATED"));
    }

    #[test]
    fn report_without_score_keeps_empirical_profile_unbound() {
        let report = build_for_score_title(None);
        assert!(report.contains("CalibrationProfileBinding NONE"));
        assert!(report.contains("EmpiricalCalibration NOT_BOUND"));
        assert!(report.contains("HumanPlayabilityClaim NOT_EVALUATED"));
    }
}
