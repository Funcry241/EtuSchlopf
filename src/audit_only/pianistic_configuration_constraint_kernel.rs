//! r295 read-only static configuration and capability kernel; direct authority closure on r292.
//!
//! The kernel evaluates a proposed single-hand finger/key configuration against two explicit
//! authorities: standard 88-key keyboard geometry and an optional, named capability envelope.
//! It never infers hand from staff or voice, never invents a maximum-span rule, and never turns
//! missing capability knowledge into a negative or positive playability claim.

use crate::audit_only::pianistic_execution_score::{self, ProjectionStatus};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-CONFIGURATION-CONSTRAINT-KERNEL-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticConfigurationConstraintKernel";
pub const SCHEMA: &str = "pianistic-configuration-constraint-kernel-v1";
pub const INPUT_AUTHORITY: &str = "PianisticExecutionScoreAuthority";
pub const STATIC_CAPABILITY_CLAIM: &str = "NOT_EVALUATED";
pub const KEYBOARD_MIN_MIDI: i32 = 21;
pub const KEYBOARD_MAX_MIDI: i32 = 108;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hand {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Finger {
    Thumb,
    Index,
    Middle,
    Ring,
    Little,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeySurface {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardKey {
    pub midi: i32,
    pub ordinal: u8,
    pub surface: KeySurface,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardGeometry;

impl KeyboardGeometry {
    pub fn standard_88_key() -> Self {
        Self
    }

    pub fn key(self, midi: i32) -> Option<KeyboardKey> {
        if !(KEYBOARD_MIN_MIDI..=KEYBOARD_MAX_MIDI).contains(&midi) {
            return None;
        }
        let ordinal = u8::try_from(midi - KEYBOARD_MIN_MIDI).ok()?;
        Some(KeyboardKey {
            midi,
            ordinal,
            surface: key_surface(midi),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FingerKeyAssignment {
    pub finger: Finger,
    pub midi: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticConfigurationRequest {
    pub hand: Hand,
    pub assignments: Vec<FingerKeyAssignment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StaticCapabilityDecision {
    Supported,
    Rejected,
    Unmodeled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticConfigurationAssessment {
    pub decision: StaticCapabilityDecision,
    pub reason: String,
    pub authority: String,
    pub matched_evidence_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticAssignmentSignature {
    pub finger: Finger,
    pub relative_semitones: i32,
    pub surface: KeySurface,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticConfigurationSignature {
    pub hand: Hand,
    pub assignments: Vec<StaticAssignmentSignature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityEntry {
    pub signature: StaticConfigurationSignature,
    pub decision: StaticCapabilityDecision,
    pub evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapabilityEvidence {
    decision: StaticCapabilityDecision,
    evidence_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PianistCapabilityEnvelope {
    pub profile_id: String,
    pub provenance: String,
    entries: BTreeMap<StaticConfigurationSignature, CapabilityEvidence>,
}

impl PianistCapabilityEnvelope {
    pub fn from_entries(
        profile_id: impl Into<String>,
        provenance: impl Into<String>,
        entries: Vec<CapabilityEntry>,
    ) -> Result<Self, String> {
        let profile_id = profile_id.into();
        let provenance = provenance.into();
        if profile_id.trim().is_empty() {
            return Err("capability-profile-id-empty".to_string());
        }
        if provenance.trim().is_empty() {
            return Err("capability-provenance-empty".to_string());
        }

        let mut indexed = BTreeMap::new();
        for entry in entries {
            if entry.decision == StaticCapabilityDecision::Unmodeled {
                return Err("explicit-unmodeled-capability-entry-forbidden".to_string());
            }
            if entry.evidence_id.trim().is_empty() {
                return Err("capability-evidence-id-empty".to_string());
            }
            let evidence = CapabilityEvidence {
                decision: entry.decision,
                evidence_id: entry.evidence_id,
            };
            if let Some(previous) = indexed.insert(entry.signature, evidence.clone())
                && previous != evidence {
                    return Err("conflicting-capability-entry".to_string());
                }
        }
        Ok(Self {
            profile_id,
            provenance,
            entries: indexed,
        })
    }
}

pub fn assess_static_configuration(
    geometry: KeyboardGeometry,
    envelope: Option<&PianistCapabilityEnvelope>,
    request: &StaticConfigurationRequest,
) -> StaticConfigurationAssessment {
    if request.assignments.is_empty() {
        return StaticConfigurationAssessment {
            decision: StaticCapabilityDecision::Supported,
            reason: "empty-hand-configuration-has-no-static-key-demand".to_string(),
            authority: AUTHORITY.to_string(),
            matched_evidence_id: None,
        };
    }

    for assignment in &request.assignments {
        if geometry.key(assignment.midi).is_none() {
            return StaticConfigurationAssessment {
                decision: StaticCapabilityDecision::Rejected,
                reason: format!("keyboard-key-outside-standard-88:midi-{}", assignment.midi),
                authority: "KeyboardGeometry".to_string(),
                matched_evidence_id: None,
            };
        }
    }

    let mut fingers = BTreeSet::new();
    let mut keys = BTreeSet::new();
    for assignment in &request.assignments {
        if !fingers.insert(assignment.finger) {
            return StaticConfigurationAssessment {
                decision: StaticCapabilityDecision::Unmodeled,
                reason: "multi-key-per-finger-unmodeled-v1".to_string(),
                authority: AUTHORITY.to_string(),
                matched_evidence_id: None,
            };
        }
        if !keys.insert(assignment.midi) {
            return StaticConfigurationAssessment {
                decision: StaticCapabilityDecision::Unmodeled,
                reason: "multi-finger-one-key-unmodeled-v1".to_string(),
                authority: AUTHORITY.to_string(),
                matched_evidence_id: None,
            };
        }
    }

    let Some(signature) = configuration_signature(geometry, request) else {
        return StaticConfigurationAssessment {
            decision: StaticCapabilityDecision::Unmodeled,
            reason: "validated-request-signature-unavailable".to_string(),
            authority: AUTHORITY.to_string(),
            matched_evidence_id: None,
        };
    };
    let Some(envelope) = envelope else {
        return StaticConfigurationAssessment {
            decision: StaticCapabilityDecision::Unmodeled,
            reason: "no-pianist-capability-envelope-bound".to_string(),
            authority: AUTHORITY.to_string(),
            matched_evidence_id: None,
        };
    };
    let Some(evidence) = envelope.entries.get(&signature) else {
        return StaticConfigurationAssessment {
            decision: StaticCapabilityDecision::Unmodeled,
            reason: format!("configuration-not-covered-by-profile:{}", envelope.profile_id),
            authority: envelope.profile_id.clone(),
            matched_evidence_id: None,
        };
    };
    StaticConfigurationAssessment {
        decision: evidence.decision,
        reason: format!(
            "explicit-whole-configuration-capability-entry:{}",
            envelope.profile_id
        ),
        authority: envelope.profile_id.clone(),
        matched_evidence_id: Some(evidence.evidence_id.clone()),
    }
}

pub fn configuration_signature(
    geometry: KeyboardGeometry,
    request: &StaticConfigurationRequest,
) -> Option<StaticConfigurationSignature> {
    let lowest_midi = request.assignments.iter().map(|item| item.midi).min()?;
    let mut assignments = request
        .assignments
        .iter()
        .map(|assignment| {
            let key = geometry.key(assignment.midi)?;
            Some(StaticAssignmentSignature {
                finger: assignment.finger,
                relative_semitones: assignment.midi.checked_sub(lowest_midi)?,
                surface: key.surface,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    assignments.sort();
    Some(StaticConfigurationSignature {
        hand: request.hand,
        assignments,
    })
}

fn verify_runtime_kernel_integrity() -> Result<String, String> {
    let geometry = KeyboardGeometry::standard_88_key();
    if geometry.key(KEYBOARD_MIN_MIDI - 1).is_some()
        || geometry.key(KEYBOARD_MAX_MIDI + 1).is_some()
    {
        return Err("keyboard-boundary-open".to_string());
    }

    let mut white_keys = 0usize;
    let mut black_keys = 0usize;
    for (expected_ordinal, midi) in (KEYBOARD_MIN_MIDI..=KEYBOARD_MAX_MIDI).enumerate() {
        let key = geometry
            .key(midi)
            .ok_or_else(|| format!("keyboard-key-missing:midi-{midi}"))?;
        if key.midi != midi || usize::from(key.ordinal) != expected_ordinal {
            return Err(format!("keyboard-key-identity-mismatch:midi-{midi}"));
        }
        match key.surface {
            KeySurface::White => white_keys += 1,
            KeySurface::Black => black_keys += 1,
        }
    }
    if white_keys != 52 || black_keys != 36 {
        return Err(format!(
            "keyboard-surface-cardinality-mismatch:white-{white_keys}:black-{black_keys}"
        ));
    }

    let supported_request = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![
            FingerKeyAssignment {
                finger: Finger::Thumb,
                midi: 60,
            },
            FingerKeyAssignment {
                finger: Finger::Index,
                midi: 62,
            },
            FingerKeyAssignment {
                finger: Finger::Middle,
                midi: 64,
            },
            FingerKeyAssignment {
                finger: Finger::Ring,
                midi: 65,
            },
            FingerKeyAssignment {
                finger: Finger::Little,
                midi: 67,
            },
        ],
    };
    let rejected_request = StaticConfigurationRequest {
        hand: Hand::Left,
        assignments: supported_request.assignments.clone(),
    };
    let supported_signature = configuration_signature(geometry, &supported_request)
        .ok_or_else(|| "runtime-supported-signature-unavailable".to_string())?;
    let rejected_signature = configuration_signature(geometry, &rejected_request)
        .ok_or_else(|| "runtime-rejected-signature-unavailable".to_string())?;
    if supported_signature.hand != Hand::Right
        || rejected_signature.hand != Hand::Left
        || supported_signature.assignments.len() != 5
        || rejected_signature.assignments.len() != 5
    {
        return Err("runtime-signature-domain-mismatch".to_string());
    }
    let signature_fingers = supported_signature
        .assignments
        .iter()
        .map(|assignment| assignment.finger)
        .collect::<BTreeSet<_>>();
    if signature_fingers.len() != 5 {
        return Err("runtime-signature-finger-domain-incomplete".to_string());
    }
    if supported_signature
        .assignments
        .iter()
        .any(|assignment| assignment.relative_semitones < 0)
    {
        return Err("runtime-signature-negative-relative-offset".to_string());
    }
    let signature_surface_count = supported_signature
        .assignments
        .iter()
        .filter(|assignment| matches!(assignment.surface, KeySurface::White | KeySurface::Black))
        .count();
    if signature_surface_count != supported_signature.assignments.len() {
        return Err("runtime-signature-key-surface-unavailable".to_string());
    }

    let envelope = PianistCapabilityEnvelope::from_entries(
        "internal-kernel-integrity-contract-v1",
        "synthetic-runtime-integrity-evidence-not-a-pianist-profile",
        vec![
            CapabilityEntry {
                signature: supported_signature,
                decision: StaticCapabilityDecision::Supported,
                evidence_id: "runtime-supported-contract".to_string(),
            },
            CapabilityEntry {
                signature: rejected_signature,
                decision: StaticCapabilityDecision::Rejected,
                evidence_id: "runtime-rejected-contract".to_string(),
            },
        ],
    )?;
    if envelope.provenance != "synthetic-runtime-integrity-evidence-not-a-pianist-profile" {
        return Err("runtime-envelope-provenance-mismatch".to_string());
    }

    let supported = assess_static_configuration(geometry, Some(&envelope), &supported_request);
    if supported.decision != StaticCapabilityDecision::Supported
        || supported.reason
            != "explicit-whole-configuration-capability-entry:internal-kernel-integrity-contract-v1"
        || supported.authority != "internal-kernel-integrity-contract-v1"
        || supported.matched_evidence_id.as_deref() != Some("runtime-supported-contract")
    {
        return Err("runtime-supported-semantics-mismatch".to_string());
    }

    let rejected = assess_static_configuration(geometry, Some(&envelope), &rejected_request);
    if rejected.decision != StaticCapabilityDecision::Rejected
        || rejected.matched_evidence_id.as_deref() != Some("runtime-rejected-contract")
    {
        return Err("runtime-rejected-semantics-mismatch".to_string());
    }

    let unmodeled_request = StaticConfigurationRequest {
        hand: Hand::Right,
        assignments: vec![
            FingerKeyAssignment {
                finger: Finger::Thumb,
                midi: 61,
            },
            FingerKeyAssignment {
                finger: Finger::Little,
                midi: 68,
            },
        ],
    };
    let unmodeled = assess_static_configuration(geometry, Some(&envelope), &unmodeled_request);
    if unmodeled.decision != StaticCapabilityDecision::Unmodeled
        || unmodeled.matched_evidence_id.is_some()
    {
        return Err("runtime-unmodeled-semantics-mismatch".to_string());
    }

    Ok(format!(
        "PASS; KeyboardKeys 88; WhiteKeys {white_keys}; BlackKeys {black_keys}; HandDomain 2/2; FingerDomain 5/5; DecisionSemantics SUPPORTED/REJECTED/UNMODELED; SyntheticIntegrityEnvelope yes; ProductivePianistProfile no"
    ))
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("=================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; InputAuthority: {INPUT_AUTHORITY}; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("AuthorityBoundary: StaffIsHand false; VoiceIsHand false; HandIsFinger false. A request names one physical hand and an explicit whole finger-to-key configuration; no hand is inferred from source staff or voice.\n");
    out.push_str("KeyboardGeometry: standard-88-key; MidiRange 21..108; KeyCount 88; KeySurface white/black explicit; physical millimeter model none.\n");
    out.push_str("CapabilitySemantics: exact whole-configuration evidence only; pairwise-span aggregation none; MaxSpanHeuristic none; missing profile or missing entry => UNMODELED; explicit negative entry => REJECTED; explicit positive entry => SUPPORTED.\n");
    out.push_str("TechniqueBoundaryV1: one-key-per-finger and one-finger-per-key are the covered request shape. Multi-key-per-finger or multi-finger-one-key requests are UNMODELED, not REJECTED.\n");

    let integrity = verify_runtime_kernel_integrity()
        .unwrap_or_else(|reason| panic!("PianisticConfigurationConstraintKernel runtime integrity failed: {reason}"));
    let _ = writeln!(out, "KernelRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason score-title-missing; PianistCapabilityEnvelope none; StaticCapabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let path = PathBuf::from(format!("{title}.musicxml"));
    let score = pianistic_execution_score::project_file(&path);
    let projection_status = match score.status {
        ProjectionStatus::Complete => "COMPLETE",
        ProjectionStatus::ModelInputUnavailable => "MODEL_INPUT_UNAVAILABLE",
    };
    let _ = writeln!(
        out,
        "InputProjectionStatus: {}; InputExecutionEvents {}; InputReason {}; PianistCapabilityEnvelope none; BoundProfile none; StaticConfigurationEnumeration none; StaticCapabilityClaim {STATIC_CAPABILITY_CLAIM}.",
        projection_status,
        score.execution_events.len(),
        score.reason,
    );
    out.push_str("NonClaims: no score-level hand assignment, no score-level finger assignment, no reach claim, no transition capability, no difficulty score, no feasibility claim, no tempo frontier, no repair candidate. r295 directly establishes and runtime-adopts the static request/checking kernel on the real r292 authority; score-wide search begins later.\n\n");
    out
}

fn key_surface(midi: i32) -> KeySurface {
    match midi.rem_euclid(12) {
        1 | 3 | 6 | 8 | 10 => KeySurface::Black,
        _ => KeySurface::White,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a(finger: Finger, midi: i32) -> FingerKeyAssignment {
        FingerKeyAssignment { finger, midi }
    }

    fn request(hand: Hand, assignments: Vec<FingerKeyAssignment>) -> StaticConfigurationRequest {
        StaticConfigurationRequest { hand, assignments }
    }

    fn entry(
        geometry: KeyboardGeometry,
        request: &StaticConfigurationRequest,
        decision: StaticCapabilityDecision,
        evidence_id: &str,
    ) -> CapabilityEntry {
        CapabilityEntry {
            signature: configuration_signature(geometry, request).expect("signature"),
            decision,
            evidence_id: evidence_id.to_string(),
        }
    }

    #[test]
    fn standard_keyboard_geometry_has_exact_88_key_bounds_and_surfaces() {
        let geometry = KeyboardGeometry::standard_88_key();
        assert_eq!(geometry.key(20), None);
        assert_eq!(geometry.key(109), None);
        assert_eq!(geometry.key(21).expect("A0").ordinal, 0);
        assert_eq!(geometry.key(108).expect("C8").ordinal, 87);
        assert_eq!(geometry.key(60).expect("C4").surface, KeySurface::White);
        assert_eq!(geometry.key(61).expect("C sharp 4").surface, KeySurface::Black);
    }

    #[test]
    fn key_outside_keyboard_is_rejected_by_geometry() {
        let geometry = KeyboardGeometry::standard_88_key();
        let assessment = assess_static_configuration(
            geometry,
            None,
            &request(Hand::Right, vec![a(Finger::Thumb, 109)]),
        );
        assert_eq!(assessment.decision, StaticCapabilityDecision::Rejected);
        assert_eq!(assessment.authority, "KeyboardGeometry");
    }

    #[test]
    fn absent_profile_is_unmodeled_not_rejected() {
        let geometry = KeyboardGeometry::standard_88_key();
        let assessment = assess_static_configuration(
            geometry,
            None,
            &request(
                Hand::Right,
                vec![a(Finger::Thumb, 60), a(Finger::Little, 67)],
            ),
        );
        assert_eq!(assessment.decision, StaticCapabilityDecision::Unmodeled);
        assert_eq!(assessment.reason, "no-pianist-capability-envelope-bound");
    }

    #[test]
    fn explicit_supported_whole_configuration_is_supported() {
        let geometry = KeyboardGeometry::standard_88_key();
        let candidate = request(
            Hand::Right,
            vec![a(Finger::Thumb, 60), a(Finger::Middle, 64), a(Finger::Little, 67)],
        );
        let envelope = PianistCapabilityEnvelope::from_entries(
            "synthetic-test-profile",
            "unit-test-only",
            vec![entry(
                geometry,
                &candidate,
                StaticCapabilityDecision::Supported,
                "support-1",
            )],
        )
        .expect("envelope");
        let assessment = assess_static_configuration(geometry, Some(&envelope), &candidate);
        assert_eq!(assessment.decision, StaticCapabilityDecision::Supported);
        assert_eq!(assessment.matched_evidence_id.as_deref(), Some("support-1"));
    }

    #[test]
    fn explicit_rejected_whole_configuration_is_rejected() {
        let geometry = KeyboardGeometry::standard_88_key();
        let candidate = request(
            Hand::Left,
            vec![a(Finger::Little, 48), a(Finger::Index, 55), a(Finger::Thumb, 60)],
        );
        let envelope = PianistCapabilityEnvelope::from_entries(
            "synthetic-test-profile",
            "unit-test-only",
            vec![entry(
                geometry,
                &candidate,
                StaticCapabilityDecision::Rejected,
                "reject-1",
            )],
        )
        .expect("envelope");
        let assessment = assess_static_configuration(geometry, Some(&envelope), &candidate);
        assert_eq!(assessment.decision, StaticCapabilityDecision::Rejected);
        assert_eq!(assessment.matched_evidence_id.as_deref(), Some("reject-1"));
    }

    #[test]
    fn same_outer_span_with_different_coupled_middle_assignment_is_not_the_same_capability() {
        let geometry = KeyboardGeometry::standard_88_key();
        let supported = request(
            Hand::Right,
            vec![a(Finger::Thumb, 60), a(Finger::Middle, 64), a(Finger::Little, 67)],
        );
        let different_middle = request(
            Hand::Right,
            vec![a(Finger::Thumb, 60), a(Finger::Index, 64), a(Finger::Little, 67)],
        );
        let envelope = PianistCapabilityEnvelope::from_entries(
            "synthetic-test-profile",
            "unit-test-only",
            vec![entry(
                geometry,
                &supported,
                StaticCapabilityDecision::Supported,
                "coupled-shape",
            )],
        )
        .expect("envelope");
        assert_eq!(
            assess_static_configuration(geometry, Some(&envelope), &supported).decision,
            StaticCapabilityDecision::Supported
        );
        assert_eq!(
            assess_static_configuration(geometry, Some(&envelope), &different_middle).decision,
            StaticCapabilityDecision::Unmodeled
        );
    }

    #[test]
    fn left_and_right_hand_signatures_are_distinct() {
        let geometry = KeyboardGeometry::standard_88_key();
        let right = request(
            Hand::Right,
            vec![a(Finger::Thumb, 60), a(Finger::Little, 67)],
        );
        let left = request(
            Hand::Left,
            vec![a(Finger::Thumb, 60), a(Finger::Little, 67)],
        );
        assert_ne!(
            configuration_signature(geometry, &right),
            configuration_signature(geometry, &left)
        );
    }

    #[test]
    fn same_shape_and_key_surface_pattern_is_octave_transposition_invariant() {
        let geometry = KeyboardGeometry::standard_88_key();
        let lower = request(
            Hand::Right,
            vec![a(Finger::Thumb, 60), a(Finger::Middle, 64), a(Finger::Little, 67)],
        );
        let upper = request(
            Hand::Right,
            vec![a(Finger::Thumb, 72), a(Finger::Middle, 76), a(Finger::Little, 79)],
        );
        assert_eq!(
            configuration_signature(geometry, &lower),
            configuration_signature(geometry, &upper)
        );
    }

    #[test]
    fn multi_key_per_finger_is_unmodeled_not_rejected() {
        let geometry = KeyboardGeometry::standard_88_key();
        let assessment = assess_static_configuration(
            geometry,
            None,
            &request(
                Hand::Right,
                vec![a(Finger::Thumb, 60), a(Finger::Thumb, 62)],
            ),
        );
        assert_eq!(assessment.decision, StaticCapabilityDecision::Unmodeled);
        assert_eq!(assessment.reason, "multi-key-per-finger-unmodeled-v1");
    }

    #[test]
    fn runtime_kernel_integrity_contract_is_complete_and_nonclaiming() {
        let result = verify_runtime_kernel_integrity().expect("runtime kernel integrity");
        assert!(result.starts_with("PASS;"));
        assert!(result.contains("HandDomain 2/2"));
        assert!(result.contains("FingerDomain 5/5"));
        assert!(result.contains("DecisionSemantics SUPPORTED/REJECTED/UNMODELED"));
        assert!(result.contains("ProductivePianistProfile no"));
    }

    #[test]
    fn conflicting_capability_entries_fail_closed() {
        let geometry = KeyboardGeometry::standard_88_key();
        let candidate = request(
            Hand::Right,
            vec![a(Finger::Thumb, 60), a(Finger::Little, 67)],
        );
        let support = entry(
            geometry,
            &candidate,
            StaticCapabilityDecision::Supported,
            "same-claim",
        );
        let reject = entry(
            geometry,
            &candidate,
            StaticCapabilityDecision::Rejected,
            "same-claim",
        );
        let result = PianistCapabilityEnvelope::from_entries(
            "synthetic-test-profile",
            "unit-test-only",
            vec![support, reject],
        );
        assert_eq!(result, Err("conflicting-capability-entry".to_string()));
    }
}
