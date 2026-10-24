#[cfg(test)]
use super::authority_primitives::yes_no;
use super::serialization_plan::AuthorizedSerializationPlan;
use super::{
    GenericityEvidence, IdentityFreeDetectorModuleBoundaryAudit,
    IdentityFreeMetamorphicSafetyNetAudit, IdentityFreeSemanticEventEquivalenceAudit,
    LocatedStructuralRepairIntent, StructuralLocator, StructuralRepairIntent,
};

const VERIFIER_MODULE_SOURCE: &str = include_str!("authorization_verifier.rs");
#[cfg(test)]
const VERIFIER_MODULE_PATH: &str = "src/structural_write_authority/authorization_verifier.rs";
const TOKEN_CONSTRUCTOR_PATTERN: &str =
    concat!("AuthorizedRepair", "Candidate { candidate, locator, plan }");
const TOKEN_STRUCT_DECLARATION_PATTERN: &str =
    concat!("struct AuthorizedRepair", "Candidate<C> {");
const PRIVATE_ISSUE_FUNCTION_PATTERN: &str =
    concat!("fn issue_authorized_", "repair_candidate(");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthorizationBlock {
    EvidenceIncomplete,
    SerializationPlanMissing,
}

/// Private capability token. The fields and the only constructor remain inside
/// the independent verifier. The parent authority module may move the opaque
/// value into the writer entry, but no detector, planner, writer or orchestrator
/// can construct or clone it.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct AuthorizedRepairCandidate<C> {
    candidate: C,
    locator: StructuralLocator,
    plan: AuthorizedSerializationPlan,
}

impl AuthorizedRepairCandidate<StructuralRepairIntent> {
    pub(super) fn into_writer_parts(
        self,
    ) -> (
        StructuralRepairIntent,
        StructuralLocator,
        AuthorizedSerializationPlan,
    ) {
        (self.candidate, self.locator, self.plan)
    }
}

fn issue_authorized_repair_candidate(
    located_intent: LocatedStructuralRepairIntent,
    evidence: GenericityEvidence,
    plan: AuthorizedSerializationPlan,
) -> Result<AuthorizedRepairCandidate<StructuralRepairIntent>, AuthorizationBlock> {
    if !evidence.authorizes_write() {
        return Err(AuthorizationBlock::EvidenceIncomplete);
    }
    if plan.rows.is_empty()
        || plan.full_backups_to_measure_start != 2
        || plan.partial_backups_to_nonzero != 0
    {
        return Err(AuthorizationBlock::SerializationPlanMissing);
    }

    let (candidate, located_view) = located_intent.into_parts();
    let (_identity_free_view, locator) = located_view.into_parts();
    Ok(AuthorizedRepairCandidate { candidate, locator, plan })
}

#[derive(Debug, Clone, Default)]
pub struct IndependentVerifierTokenIssuanceAudit {
    #[cfg(test)]
    source_stage: String,
    #[cfg(test)]
    verifier_module_path: String,
    #[cfg(test)]
    repair_intent_count: usize,
    #[cfg(test)]
    detector_module_boundary_green: bool,
    #[cfg(test)]
    semantic_serialization_proof_green: bool,
    #[cfg(test)]
    genericity_evidence_green: bool,
    #[cfg(test)]
    local_conflict_group_unique: bool,
    authorization_attempted: bool,
    authorization_granted: usize,
    tokens_issued: usize,
    #[cfg(test)]
    token_constructor_sites: usize,
    #[cfg(test)]
    token_forgery_paths: usize,
    token_candidate_kind_bound: bool,
    token_locator_retained_opaque: bool,
    token_serialization_plan_bound: bool,
    #[cfg(test)]
    token_origin: String,
    token_handed_to_writer: bool,
    token_consumed: bool,
    tokens_dropped_without_writer: usize,
    writer_token_used: bool,
    write_produced: bool,
    #[cfg(test)]
    verdict: String,
}

impl IndependentVerifierTokenIssuanceAudit {
    #[cfg(test)]
    pub fn token_origin_proof_green(&self) -> bool {
        self.authorization_attempted
            && self.authorization_granted == 1
            && self.tokens_issued == 1
            && self.token_constructor_sites == 1
            && self.token_forgery_paths == 0
            && self.token_candidate_kind_bound
            && self.token_locator_retained_opaque
            && self.token_serialization_plan_bound
            && self.token_origin == "verifier_only"
    }

    pub(super) fn bind_writer_outcome(&mut self, consumed: bool, write_produced: bool) {
        self.token_handed_to_writer = true;
        self.token_consumed = consumed;
        self.tokens_dropped_without_writer = 0;
        self.writer_token_used = consumed;
        self.write_produced = write_produced;
        #[cfg(test)]
        {
            self.verdict = if self.token_origin_proof_green() && consumed {
            if write_produced {
                "independent_verifier_token_issued_handed_to_writer_and_consumed_once"
                    .to_string()
            } else {
                "independent_verifier_token_consumed_without_candidate_write".to_string()
            }
        } else {
            "independent_verifier_token_handoff_incomplete".to_string()
            };
        }
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "IndependentVerifierTokenIssuanceAudit v{}: Project {}; SourceStage {}; VerifierModule {}; RepairIntents {}; DetectorBoundaryGreen {}; SemanticSerializationProofGreen {}; GenericityEvidenceGreen {}; LocalConflictGroupUnique {}; AuthorizationAttempted {}; AuthorizationGranted {}; TokensIssued {}; TokenConstructorSites {}; TokenForgeryPaths {}; TokenCandidateKindBound {}; TokenLocatorRetainedOpaque {}; TokenSerializationPlanBound {}; TokenOrigin {}; TokenHandedToWriter {}; TokenConsumed {}; TokensDroppedWithoutWriter {}; WriterTokenUsed {}; WriteProduced {}; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            self.source_stage,
            self.verifier_module_path,
            self.repair_intent_count,
            yes_no(self.detector_module_boundary_green),
            yes_no(self.semantic_serialization_proof_green),
            yes_no(self.genericity_evidence_green),
            yes_no(self.local_conflict_group_unique),
            yes_no(self.authorization_attempted),
            self.authorization_granted,
            self.tokens_issued,
            self.token_constructor_sites,
            self.token_forgery_paths,
            yes_no(self.token_candidate_kind_bound),
            yes_no(self.token_locator_retained_opaque),
            yes_no(self.token_serialization_plan_bound),
            self.token_origin,
            yes_no(self.token_handed_to_writer),
            yes_no(self.token_consumed),
            self.tokens_dropped_without_writer,
            yes_no(self.writer_token_used),
            yes_no(self.write_produced),
            yes_no(self.write_produced),
            self.verdict,
        )
    }

}

fn token_struct_fields_are_private() -> bool {
    let Some(start) = VERIFIER_MODULE_SOURCE.find(TOKEN_STRUCT_DECLARATION_PATTERN) else {
        return false;
    };
    let tail = &VERIFIER_MODULE_SOURCE[start..];
    let Some(end) = tail.find('}') else {
        return false;
    };
    tail[..end]
        .lines()
        .skip(1)
        .filter(|line| line.contains(':'))
        .all(|line| !line.trim_start().starts_with("pub"))
}

pub(super) fn authorize_repair_candidate_for_writer(
    repair_intents: &[LocatedStructuralRepairIntent],
    detector_boundary_audit: &IdentityFreeDetectorModuleBoundaryAudit,
    semantic_event_audit: &IdentityFreeSemanticEventEquivalenceAudit,
    metamorphic_audit: &IdentityFreeMetamorphicSafetyNetAudit,
    _source_stage: &str,
) -> (
    IndependentVerifierTokenIssuanceAudit,
    Option<AuthorizedRepairCandidate<StructuralRepairIntent>>,
) {
    let detector_module_boundary_green = detector_boundary_audit.authorizes_token_origin();
    let semantic_serialization_proof_green = semantic_event_audit.authorizes_token_origin();
    let genericity_evidence = metamorphic_audit.authorization_evidence();
    let genericity_evidence_green = metamorphic_audit.authorizes_token_origin();
    // The caller passes exactly one explicit local conflict group member.
    // Scorewide candidate multiplicity is intentionally outside this verifier.
    let local_conflict_group_unique = repair_intents.len() == 1;
    let token_constructor_sites = VERIFIER_MODULE_SOURCE
        .matches(TOKEN_CONSTRUCTOR_PATTERN)
        .count();
    let token_fields_private = token_struct_fields_are_private();
    let issue_function_private = VERIFIER_MODULE_SOURCE.lines().any(|line| {
        line.trim_start().starts_with(PRIVATE_ISSUE_FUNCTION_PATTERN)
            && !line.trim_start().starts_with("pub")
    });
    let token_forgery_paths = if token_constructor_sites != 1
        || !token_fields_private
        || !issue_function_private
    {
        1
    } else {
        0
    };

    let mut audit = IndependentVerifierTokenIssuanceAudit {
        #[cfg(test)]
        source_stage: _source_stage.to_string(),
        #[cfg(test)]
        verifier_module_path: VERIFIER_MODULE_PATH.to_string(),
        #[cfg(test)]
        repair_intent_count: repair_intents.len(),
        #[cfg(test)]
        detector_module_boundary_green,
        #[cfg(test)]
        semantic_serialization_proof_green,
        #[cfg(test)]
        genericity_evidence_green,
        #[cfg(test)]
        local_conflict_group_unique,
        authorization_attempted: false,
        authorization_granted: 0,
        tokens_issued: 0,
        #[cfg(test)]
        token_constructor_sites,
        #[cfg(test)]
        token_forgery_paths,
        token_candidate_kind_bound: false,
        token_locator_retained_opaque: false,
        token_serialization_plan_bound: false,
        #[cfg(test)]
        token_origin: "verifier_only".to_string(),
        token_handed_to_writer: false,
        token_consumed: false,
        tokens_dropped_without_writer: 0,
        writer_token_used: false,
        write_produced: false,
        #[cfg(test)]
        verdict: "independent_verifier_token_not_issued".to_string(),
    };

    let prerequisites_green = detector_module_boundary_green
        && semantic_serialization_proof_green
        && genericity_evidence_green
        && local_conflict_group_unique
        && token_forgery_paths == 0;
    if !prerequisites_green {
        return (audit, None);
    }

    let Some(plan) = semantic_event_audit.authorized_serialization_plan() else {
        #[cfg(test)]
        { audit.verdict = "independent_verifier_serialization_plan_missing".to_string(); }
        return (audit, None);
    };
    let Some(located_intent) = repair_intents.first().cloned() else {
        return (audit, None);
    };

    audit.authorization_attempted = true;
    match issue_authorized_repair_candidate(located_intent, genericity_evidence, plan) {
        Ok(token) => {
            audit.authorization_granted = 1;
            audit.tokens_issued = 1;
            audit.token_candidate_kind_bound =
                token.candidate == StructuralRepairIntent::CompleteTailBeatBeam;
            audit.token_locator_retained_opaque = std::mem::size_of_val(&token.locator) > 0;
            audit.token_serialization_plan_bound = !token.plan.rows.is_empty();
            #[cfg(test)]
            { audit.verdict = "independent_verifier_token_ready_for_single_writer_handoff".to_string(); }
            (audit, Some(token))
        }
        Err(_) => {
            #[cfg(test)]
            { audit.verdict = "independent_verifier_authorization_blocked".to_string(); }
            (audit, None)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::structural_write_authority::serialization_plan::{
        AuthorizedPlannedLane, AuthorizedSerializationPlanRow,
    };
    use crate::structural_write_authority::{
        LocatedStructuralView, StructuralBeamState, StructuralCarrierRole, StructuralEvent,
        StructuralStaffRelation, StructuralView, StructuralVoiceRelation,
    };

    fn located_intent(nonce: u64) -> LocatedStructuralRepairIntent {
        let view = StructuralView::new(
            96,
            vec![StructuralEvent {
                onset_in_window: 0,
                duration: 24,
                primary_beam: StructuralBeamState::Absent,
                secondary_beam: StructuralBeamState::Absent,
                staff_relation: StructuralStaffRelation::SameAsAnchor,
                voice_relation: StructuralVoiceRelation::SameRun,
                carrier_role: StructuralCarrierRole::Sounding,
                visible: true,
                rest_like: false,
                chord_member: false,
            }],
        );
        LocatedStructuralRepairIntent::new(
            StructuralRepairIntent::CompleteTailBeatBeam,
            LocatedStructuralView::new(view, StructuralLocator::from_orchestrator_nonce(nonce)),
        )
    }

    fn complete_evidence() -> GenericityEvidence {
        GenericityEvidence {
            real_pipeline_snapshot_passed: true,
            metamorphic_positive_variants: 5,
            metamorphic_negative_variants: 6,
            positive_delta_kind_stable: true,
            negatives_rejected: true,
            independent_candidate_duplication_invariant: true,
            candidate_count: 2,
        }
    }

    fn plan() -> AuthorizedSerializationPlan {
        AuthorizedSerializationPlan {
            rows: vec![AuthorizedSerializationPlanRow {
                source_event_ordinal: 1,
                original_cursor_before: 0,
                original_cursor_after: 24,
                planned_chord_member: false,
                planned_lane: AuthorizedPlannedLane::MainVoiceRun,
                planned_xml_order: 0,
                planned_cursor_before: 0,
                planned_cursor_after: 24,
                planned_primary_beam: StructuralBeamState::Begin,
                planned_secondary_beam: StructuralBeamState::Absent,
            }],
            full_backups_to_measure_start: 2,
            partial_backups_to_nonzero: 0,
        }
    }

    #[test]
    fn complete_evidence_and_plan_issue_one_private_token() {
        let token = issue_authorized_repair_candidate(
            located_intent(99),
            complete_evidence(),
            plan(),
        )
        .expect("complete evidence must issue the private token");
        let (candidate, locator, bound_plan) = token.into_writer_parts();
        assert_eq!(candidate, StructuralRepairIntent::CompleteTailBeatBeam);
        assert!(std::mem::size_of_val(&locator) > 0);
        assert_eq!(bound_plan.rows.len(), 1);
    }

    #[test]
    fn incomplete_evidence_issues_no_token() {
        let incomplete = GenericityEvidence {
            metamorphic_positive_variants: 0,
            ..complete_evidence()
        };
        assert_eq!(
            issue_authorized_repair_candidate(located_intent(1), incomplete, plan()),
            Err(AuthorizationBlock::EvidenceIncomplete)
        );
    }

    #[test]
    fn verifier_source_has_one_private_token_constructor() {
        assert_eq!(
            VERIFIER_MODULE_SOURCE
                .matches(TOKEN_CONSTRUCTOR_PATTERN)
                .count(),
            1
        );
        assert!(token_struct_fields_are_private());
    }
}
