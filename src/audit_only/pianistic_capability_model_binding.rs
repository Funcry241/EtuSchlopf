//! r298 read-only capability-model provenance and binding contract.
//!
//! The constructive piano proof needs an explicit boundary between empirical calibration evidence
//! and a reference assumption model. Both can be useful inputs, but only empirical calibration may
//! ever be eligible for a later human playability claim. A reference assumption can produce model-
//! relative search evidence only. This owner binds no productive model for the real score.

use std::fmt::Write as _;

pub const SECTION: &str = "PIANISTIC-CAPABILITY-MODEL-BINDING-CONTRACT-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticCapabilityModelBindingContract";
pub const SCHEMA: &str = "pianistic-capability-model-binding-v1";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CapabilityEvidenceBasis {
    EmpiricalCalibration,
    ReferenceAssumption,
}

impl CapabilityEvidenceBasis {
    fn as_str(self) -> &'static str {
        match self {
            Self::EmpiricalCalibration => "EMPIRICAL_CALIBRATION",
            Self::ReferenceAssumption => "REFERENCE_ASSUMPTION",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClaimBoundary {
    HumanCalibrationEligible,
    ModelRelativeOnly,
}

impl ClaimBoundary {
    fn as_str(self) -> &'static str {
        match self {
            Self::HumanCalibrationEligible => "HUMAN_CALIBRATION_ELIGIBLE",
            Self::ModelRelativeOnly => "MODEL_RELATIVE_ONLY",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapabilityModelBindingRequest {
    model_id: String,
    model_source_sha256: String,
    provenance: String,
    evidence_basis: CapabilityEvidenceBasis,
    assumption_contract: Option<String>,
    calibration_profile_id: Option<String>,
    authorizes_human_playability_claim: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CapabilityModelBindingAssessment {
    accepted: bool,
    reason: &'static str,
    evidence_basis: CapabilityEvidenceBasis,
    claim_boundary: ClaimBoundary,
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

fn assess_binding(request: &CapabilityModelBindingRequest) -> CapabilityModelBindingAssessment {
    let boundary = match request.evidence_basis {
        CapabilityEvidenceBasis::EmpiricalCalibration => ClaimBoundary::HumanCalibrationEligible,
        CapabilityEvidenceBasis::ReferenceAssumption => ClaimBoundary::ModelRelativeOnly,
    };

    if !nonempty(&request.model_id) {
        return CapabilityModelBindingAssessment {
            accepted: false,
            reason: "model-id-empty",
            evidence_basis: request.evidence_basis,
            claim_boundary: boundary,
        };
    }
    if !nonempty(&request.provenance) {
        return CapabilityModelBindingAssessment {
            accepted: false,
            reason: "provenance-empty",
            evidence_basis: request.evidence_basis,
            claim_boundary: boundary,
        };
    }
    if !is_lower_hex_sha256(&request.model_source_sha256) {
        return CapabilityModelBindingAssessment {
            accepted: false,
            reason: "model-source-sha256-invalid",
            evidence_basis: request.evidence_basis,
            claim_boundary: boundary,
        };
    }

    match request.evidence_basis {
        CapabilityEvidenceBasis::EmpiricalCalibration => {
            if request
                .calibration_profile_id
                .as_deref()
                .is_none_or(|value| !nonempty(value))
            {
                return CapabilityModelBindingAssessment {
                    accepted: false,
                    reason: "empirical-calibration-profile-id-missing",
                    evidence_basis: request.evidence_basis,
                    claim_boundary: boundary,
                };
            }
        }
        CapabilityEvidenceBasis::ReferenceAssumption => {
            if request
                .assumption_contract
                .as_deref()
                .is_none_or(|value| !nonempty(value))
            {
                return CapabilityModelBindingAssessment {
                    accepted: false,
                    reason: "reference-assumption-contract-missing",
                    evidence_basis: request.evidence_basis,
                    claim_boundary: boundary,
                };
            }
            if request.authorizes_human_playability_claim {
                return CapabilityModelBindingAssessment {
                    accepted: false,
                    reason: "reference-assumption-cannot-authorize-human-playability-claim",
                    evidence_basis: request.evidence_basis,
                    claim_boundary: boundary,
                };
            }
        }
    }

    CapabilityModelBindingAssessment {
        accepted: true,
        reason: "binding-contract-satisfied",
        evidence_basis: request.evidence_basis,
        claim_boundary: boundary,
    }
}


pub(crate) fn validate_empirical_calibration_binding(
    model_id: &str,
    model_source_sha256: &str,
    provenance: &str,
    calibration_profile_id: &str,
) -> Result<&'static str, String> {
    let request = CapabilityModelBindingRequest {
        model_id: model_id.to_string(),
        model_source_sha256: model_source_sha256.to_string(),
        provenance: provenance.to_string(),
        evidence_basis: CapabilityEvidenceBasis::EmpiricalCalibration,
        assumption_contract: None,
        calibration_profile_id: Some(calibration_profile_id.to_string()),
        authorizes_human_playability_claim: false,
    };
    let assessment = assess_binding(&request);
    if !assessment.accepted {
        return Err(assessment.reason.to_string());
    }
    if assessment.evidence_basis != CapabilityEvidenceBasis::EmpiricalCalibration
        || assessment.claim_boundary != ClaimBoundary::HumanCalibrationEligible
    {
        return Err("empirical-calibration-binding-boundary-mismatch".to_string());
    }
    Ok(assessment.claim_boundary.as_str())
}

pub(crate) fn validate_reference_assumption_binding(
    model_id: &str,
    model_source_sha256: &str,
    provenance: &str,
    assumption_contract: &str,
) -> Result<&'static str, String> {
    let request = CapabilityModelBindingRequest {
        model_id: model_id.to_string(),
        model_source_sha256: model_source_sha256.to_string(),
        provenance: provenance.to_string(),
        evidence_basis: CapabilityEvidenceBasis::ReferenceAssumption,
        assumption_contract: Some(assumption_contract.to_string()),
        calibration_profile_id: None,
        authorizes_human_playability_claim: false,
    };
    let assessment = assess_binding(&request);
    if !assessment.accepted {
        return Err(assessment.reason.to_string());
    }
    if assessment.evidence_basis != CapabilityEvidenceBasis::ReferenceAssumption
        || assessment.claim_boundary != ClaimBoundary::ModelRelativeOnly
    {
        return Err("reference-assumption-binding-boundary-mismatch".to_string());
    }
    Ok(assessment.claim_boundary.as_str())
}

fn verify_runtime_integrity() -> Result<String, String> {
    let empirical = CapabilityModelBindingRequest {
        model_id: "synthetic-empirical-v1".to_string(),
        model_source_sha256: "1".repeat(64),
        provenance: "synthetic-runtime-integrity-only".to_string(),
        evidence_basis: CapabilityEvidenceBasis::EmpiricalCalibration,
        assumption_contract: None,
        calibration_profile_id: Some("synthetic-profile".to_string()),
        authorizes_human_playability_claim: true,
    };
    let empirical_assessment = assess_binding(&empirical);
    if !empirical_assessment.accepted
        || empirical_assessment.claim_boundary != ClaimBoundary::HumanCalibrationEligible
        || empirical_assessment.evidence_basis != CapabilityEvidenceBasis::EmpiricalCalibration
    {
        return Err("empirical-binding-integrity-failed".to_string());
    }

    let reference = CapabilityModelBindingRequest {
        model_id: "synthetic-reference-assumption-v1".to_string(),
        model_source_sha256: "2".repeat(64),
        provenance: "synthetic-runtime-integrity-only".to_string(),
        evidence_basis: CapabilityEvidenceBasis::ReferenceAssumption,
        assumption_contract: Some("synthetic-explicit-assumption-boundary".to_string()),
        calibration_profile_id: None,
        authorizes_human_playability_claim: false,
    };
    let reference_assessment = assess_binding(&reference);
    if !reference_assessment.accepted
        || reference_assessment.claim_boundary != ClaimBoundary::ModelRelativeOnly
        || reference_assessment.evidence_basis != CapabilityEvidenceBasis::ReferenceAssumption
    {
        return Err("reference-binding-integrity-failed".to_string());
    }

    let mut illegal_reference = reference;
    illegal_reference.authorizes_human_playability_claim = true;
    let rejected = assess_binding(&illegal_reference);
    if rejected.accepted
        || rejected.reason != "reference-assumption-cannot-authorize-human-playability-claim"
    {
        return Err("reference-claim-escalation-not-rejected".to_string());
    }

    Ok(format!(
        "PASS; EvidenceBasis {}/{}; ClaimBoundary {}/{}; ReferenceHumanClaimEscalation rejected",
        empirical_assessment.evidence_basis.as_str(),
        reference_assessment.evidence_basis.as_str(),
        empirical_assessment.claim_boundary.as_str(),
        reference_assessment.claim_boundary.as_str(),
    ))
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("===================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("BindingBoundary: empirical calibration evidence and reference assumptions are separate provenance classes. Reference assumptions may support model-relative search/falsification only and can never authorize a human playability claim.\n");
    out.push_str("IndependenceBoundary: a score-demand inventory defines questions, not answers. Capability support/rejection must come from a separately identified model source with its own SHA-256 and provenance.\n");
    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticCapabilityModelBindingContract runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "BindingRuntimeIntegrity: {integrity}.");

    let score_available = score_title.map(str::trim).is_some_and(|value| !value.is_empty());
    let _ = writeln!(
        out,
        "ScoreContextAvailable: {}; ProductiveCapabilityModel none; BindingStatus NONE; EvidenceBasis none; ClaimBoundary none; PlayabilityClaim {PLAYABILITY_CLAIM}.",
        if score_available { "yes" } else { "no" }
    );
    out.push_str("NextBindingRequirement: explicit independently sourced capability model descriptor + source SHA-256 + provenance + EMPIRICAL_CALIBRATION or REFERENCE_ASSUMPTION basis. A REFERENCE_ASSUMPTION must carry an explicit assumption contract and remains MODEL_RELATIVE_ONLY.\n");
    out.push_str("NonClaims: no supported configuration, no rejected configuration, no transition timing truth, no pianist identity, no score-level hand assignment, no score-level finger assignment, no feasibility claim, no obstruction claim, no tempo frontier, no repair candidate.\n\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference_request() -> CapabilityModelBindingRequest {
        CapabilityModelBindingRequest {
            model_id: "reference-v1".to_string(),
            model_source_sha256: "a".repeat(64),
            provenance: "explicit-reference-assumption".to_string(),
            evidence_basis: CapabilityEvidenceBasis::ReferenceAssumption,
            assumption_contract: Some("model-relative-only".to_string()),
            calibration_profile_id: None,
            authorizes_human_playability_claim: false,
        }
    }

    #[test]
    fn reference_assumption_is_model_relative_only() {
        let assessment = assess_binding(&reference_request());
        assert!(assessment.accepted);
        assert_eq!(assessment.claim_boundary, ClaimBoundary::ModelRelativeOnly);
    }

    #[test]
    fn reference_assumption_cannot_authorize_human_playability_claim() {
        let mut request = reference_request();
        request.authorizes_human_playability_claim = true;
        let assessment = assess_binding(&request);
        assert!(!assessment.accepted);
        assert_eq!(
            assessment.reason,
            "reference-assumption-cannot-authorize-human-playability-claim"
        );
    }

    #[test]
    fn reference_assumption_requires_explicit_contract() {
        let mut request = reference_request();
        request.assumption_contract = None;
        let assessment = assess_binding(&request);
        assert!(!assessment.accepted);
        assert_eq!(assessment.reason, "reference-assumption-contract-missing");
    }

    #[test]
    fn empirical_calibration_requires_profile_id() {
        let request = CapabilityModelBindingRequest {
            model_id: "empirical-v1".to_string(),
            model_source_sha256: "b".repeat(64),
            provenance: "calibration-session".to_string(),
            evidence_basis: CapabilityEvidenceBasis::EmpiricalCalibration,
            assumption_contract: None,
            calibration_profile_id: None,
            authorizes_human_playability_claim: false,
        };
        let assessment = assess_binding(&request);
        assert!(!assessment.accepted);
        assert_eq!(assessment.reason, "empirical-calibration-profile-id-missing");
    }

    #[test]
    fn empirical_calibration_is_human_claim_eligible_not_automatically_playable() {
        let request = CapabilityModelBindingRequest {
            model_id: "empirical-v1".to_string(),
            model_source_sha256: "c".repeat(64),
            provenance: "calibration-session".to_string(),
            evidence_basis: CapabilityEvidenceBasis::EmpiricalCalibration,
            assumption_contract: None,
            calibration_profile_id: Some("profile-opaque-1".to_string()),
            authorizes_human_playability_claim: true,
        };
        let assessment = assess_binding(&request);
        assert!(assessment.accepted);
        assert_eq!(
            assessment.claim_boundary,
            ClaimBoundary::HumanCalibrationEligible
        );
    }

    #[test]
    fn empirical_validation_helper_binds_profile_without_playability_claim() {
        let boundary = validate_empirical_calibration_binding(
            "empirical-v2",
            &"d".repeat(64),
            "independent-calibration-session",
            "profile-v2",
        )
        .expect("empirical binding should satisfy the contract");
        assert_eq!(boundary, "HUMAN_CALIBRATION_ELIGIBLE");
    }

    #[test]
    fn model_source_sha256_is_fail_closed() {
        let mut request = reference_request();
        request.model_source_sha256 = "not-a-sha256".to_string();
        let assessment = assess_binding(&request);
        assert!(!assessment.accepted);
        assert_eq!(assessment.reason, "model-source-sha256-invalid");
    }
}
