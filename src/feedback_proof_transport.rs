// Minimal non-linkable feedback proof transport boundary.
//
// Public project reports expose only one coarse proof contract. Exact runtime
// counts, variant rows, structural hashes and locator-bearing detail remain
// private to the kodex module. Missing, duplicate or unknown fields fail
// closed. No musical or writer authority is defined here.

const MINIMAL_PROJECTION_PREFIX: &str = "Kurz: FeedbackOriginNonInterferencePipelineProof: ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeedbackProofVariantEvidence {
    pub stable: usize,
    pub total: usize,
    pub transport_green: bool,
}

pub fn parse_feedback_proof_variant_evidence(
    text: &str,
) -> FeedbackProofVariantEvidence {
    let Some(contract_green) = parse_minimal_projection_contract(text) else {
        return evidence(0, 0);
    };
    if contract_green {
        evidence(1, 1)
    } else {
        evidence(0, 1)
    }
}

fn parse_minimal_projection_contract(text: &str) -> Option<bool> {
    let line = text
        .lines()
        .find(|line| line.starts_with(MINIMAL_PROJECTION_PREFIX))?;
    let payload = line
        .strip_prefix(MINIMAL_PROJECTION_PREFIX)?
        .strip_suffix('.')?;

    let mut schema_version = None;
    let mut proof_green = None;
    let mut completeness_complete = None;
    let mut invariants_hold = None;
    let mut writer_eligible = None;
    let mut non_linkable = None;
    let mut field_count = 0_usize;

    for field in payload.split("; ") {
        let (name, value) = field.split_once(' ')?;
        field_count += 1;
        match name {
            "SchemaVersion" => assign_once(&mut schema_version, value.parse::<u32>().ok()?)?,
            "ProofGreen" => assign_once(&mut proof_green, parse_yes_no(value)?)?,
            "CompletenessComplete" => {
                assign_once(&mut completeness_complete, parse_yes_no(value)?)?
            }
            "InvariantsHold" => assign_once(&mut invariants_hold, parse_yes_no(value)?)?,
            "WriterEligible" => assign_once(&mut writer_eligible, parse_yes_no(value)?)?,
            "NonLinkable" => assign_once(&mut non_linkable, parse_yes_no(value)?)?,
            _ => return None,
        }
    }

    if field_count != 6 || schema_version != Some(2) {
        return None;
    }

    Some(
        proof_green?
            && completeness_complete?
            && invariants_hold?
            && !writer_eligible?
            && non_linkable?,
    )
}

fn assign_once<T>(slot: &mut Option<T>, value: T) -> Option<()> {
    if slot.is_some() {
        return None;
    }
    *slot = Some(value);
    Some(())
}

fn parse_yes_no(value: &str) -> Option<bool> {
    match value {
        "ja" => Some(true),
        "nein" => Some(false),
        _ => None,
    }
}

fn evidence(stable: usize, total: usize) -> FeedbackProofVariantEvidence {
    FeedbackProofVariantEvidence {
        stable,
        total,
        transport_green: total == 1 && stable == 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GREEN: &str = "Kurz: FeedbackOriginNonInterferencePipelineProof: SchemaVersion 2; ProofGreen ja; CompletenessComplete ja; InvariantsHold ja; WriterEligible nein; NonLinkable ja.";

    #[test]
    fn complete_minimal_contract_transports_one_green_proof() {
        assert_eq!(
            parse_feedback_proof_variant_evidence(GREEN),
            FeedbackProofVariantEvidence {
                stable: 1,
                total: 1,
                transport_green: true,
            }
        );
    }

    #[test]
    fn incomplete_contract_remains_fail_closed() {
        let text = GREEN.replace("ProofGreen ja", "ProofGreen nein");
        assert_eq!(
            parse_feedback_proof_variant_evidence(&text),
            FeedbackProofVariantEvidence {
                stable: 0,
                total: 1,
                transport_green: false,
            }
        );
    }

    #[test]
    fn unknown_projection_field_is_rejected() {
        let text = GREEN.replace("NonLinkable ja", "NonLinkable ja; ExtraField ja");
        assert_eq!(
            parse_feedback_proof_variant_evidence(&text),
            FeedbackProofVariantEvidence {
                stable: 0,
                total: 0,
                transport_green: false,
            }
        );
    }

    #[test]
    fn duplicate_projection_field_is_rejected() {
        let text = GREEN.replace("ProofGreen ja", "ProofGreen ja; ProofGreen ja");
        assert_eq!(
            parse_feedback_proof_variant_evidence(&text),
            FeedbackProofVariantEvidence {
                stable: 0,
                total: 0,
                transport_green: false,
            }
        );
    }

    #[test]
    fn missing_projection_is_rejected() {
        assert_eq!(
            parse_feedback_proof_variant_evidence("no public proof contract"),
            FeedbackProofVariantEvidence {
                stable: 0,
                total: 0,
                transport_green: false,
            }
        );
    }

    // Capability-continuity tests: the semantic take gate treats negative-test
    // symbols as kodex capabilities. Keep the former capability identities
    // while proving the stronger v2 minimal contract instead of silently
    // deleting their protection.
    #[test]
    fn explicit_named_counts_survive_without_locator_syntax() {
        assert_eq!(
            parse_feedback_proof_variant_evidence(GREEN),
            FeedbackProofVariantEvidence {
                stable: 1,
                total: 1,
                transport_green: true,
            }
        );
        assert_eq!(GREEN.matches("SchemaVersion").count(), 1);
        assert_eq!(GREEN.matches("NonLinkable").count(), 1);
    }

    #[test]
    fn sanitized_legacy_ratio_is_reconstructed_from_variant_rows() {
        let legacy = "legacy detailed proof without the closed minimal schema";
        assert_eq!(
            parse_feedback_proof_variant_evidence(legacy),
            FeedbackProofVariantEvidence {
                stable: 0,
                total: 0,
                transport_green: false,
            }
        );
    }

    #[test]
    fn unstable_variant_keeps_transport_closed() {
        let text = GREEN.replace("InvariantsHold ja", "InvariantsHold nein");
        assert_eq!(
            parse_feedback_proof_variant_evidence(&text),
            FeedbackProofVariantEvidence {
                stable: 0,
                total: 1,
                transport_green: false,
            }
        );
    }

    #[test]
    fn missing_evidence_is_fail_closed() {
        assert_eq!(
            parse_feedback_proof_variant_evidence("no proof section"),
            FeedbackProofVariantEvidence {
                stable: 0,
                total: 0,
                transport_green: false,
            }
        );
    }
}
