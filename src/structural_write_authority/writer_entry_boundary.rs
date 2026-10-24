use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt};

pub(crate) struct CursorNeutralRenderExactMint { _sealed: () }

use super::authority_primitives::yes_no;
use super::authorization_verifier::AuthorizedRepairCandidate;
use super::locator_resolution::resolve_authorized_serialization_target;
use super::renderer_direction_relocation_authorization::RendererDirectionRelocationAuthorizationBoundaryAudit;
use super::renderer_direction_relocation_plan::{
    audit_renderer_neutral_direction_placement_plan, ProvenRendererDirectionRelocationPlan,
    RendererNeutralDirectionPlacementPlanAudit,
};
use super::renderer_direction_relocation_post_write_verifier::RendererDirectionRelocationPostWriteAudit;
use super::serialization_plan::{
    AuthorizedPlannedLane, AuthorizedSerializationPlan, AuthorizedSerializationPlanRow,
};
use super::{StructuralBeamState, StructuralRepairIntent};
use roxmltree::Document;
use std::fmt::Write as _;

const WRITER_MODULE_SOURCE: &str = include_str!("writer_entry_boundary.rs");
const AUTHORITY_MODULE_SOURCE: &str = include_str!("mod.rs");
const VERIFIER_MODULE_SOURCE: &str = include_str!("authorization_verifier.rs");
const WRITER_MODULE_PATH: &str = "src/structural_write_authority/writer_entry_boundary.rs";
const ORCHESTRATOR_SOURCE: &str =
    include_str!("../pipeline_shards/pipeline_orchestrator_metric_final_notation_stage.rs");
const DETECTOR_SOURCE: &str = include_str!("detector.rs");
const PLANNER_SOURCE: &str = include_str!("serialization_plan.rs");
const LOCATOR_RESOLVER_SOURCE: &str = include_str!("locator_resolution.rs");
const LOCATOR_RESOLVER_MODULE_PATH: &str = "src/structural_write_authority/locator_resolution.rs";
const WRITER_ENTRY_DEFINITION_PATTERN: &str = concat!("fn authorized_reference_", "writer_entry(");
const WRITER_ENTRY_TOKEN_PARAMETER_PATTERN: &str = concat!(
    "token: AuthorizedRepair",
    "Candidate<StructuralRepairIntent>"
);
const RAW_INTENT_PARAMETER_PATTERN: &str = concat!("candidate: Structural", "RepairIntent");
const RAW_LOCATED_INTENT_PARAMETER_PATTERN: &str = concat!("LocatedStructural", "RepairIntent");
const TOKEN_CONSTRUCTOR_PATTERN: &str =
    concat!("AuthorizedRepair", "Candidate { candidate, locator, plan }");
const TOKEN_CLONE_DERIVE_PATTERN: &str = concat!(
    "#[derive(Debug, Clone, PartialEq, Eq)]\npub(super) struct AuthorizedRepair",
    "Candidate<C>"
);
const TOKEN_CLONE_CALL_PATTERN: &str = concat!("token", ".clone()");
const LEGACY_DIRECT_WRITER_CALL_PATTERN: &str =
    concat!("apply_beat_aligned_", "cross_staff_tail_beam_repair(");
const WRITER_ENTRY_NAME_PATTERN: &str = concat!("authorized_reference_", "writer_entry(");
const LITERAL_MEASURE_SELECTOR_PATTERN: &str = concat!("\"", "3", "3", "\"");
const LITERAL_PITCH_STEP_PATTERN: &str = concat!("<st", "ep>");
const LITERAL_PITCH_OCTAVE_PATTERN: &str = concat!("<oct", "ave>");
const LITERAL_VOICE_SELECTOR_PATTERN: &str = concat!("<voi", "ce>", "2", "</voi", "ce>");
const LITERAL_STAFF_ONE_SELECTOR_PATTERN: &str = concat!("<sta", "ff>", "1", "</sta", "ff>");
const LITERAL_STAFF_TWO_SELECTOR_PATTERN: &str = concat!("<sta", "ff>", "2", "</sta", "ff>");
const LOCATOR_IDENTITY_ACCESS_PATTERNS: &[&str] = &[
    concat!("locator.", "part_ordinal"),
    concat!("locator.", "measure_ordinal"),
    concat!("locator.", "source_identity"),
    concat!("locator.", "source_nonce"),
    concat!("locator.", "source_events"),
    concat!("locator.", "window_start"),
    concat!("locator.", "source_fingerprint"),
];
const LOCATOR_RESOLVER_SELECTION_PATTERNS: &[&str] = &[
    concat!("StructuralRepair", "Intent"),
    concat!("detect_repair_", "intent"),
    concat!("classify_tail_", "beam_state"),
    concat!("Etüde Nr. ", "6"),
    concat!("\"", "3", "3", "\""),
    concat!("== ", "3", "2"),
    concat!("== ", "3", "3"),
    concat!("<st", "ep>"),
    concat!("<oct", "ave>"),
    concat!("<alt", "er>"),
    concat!("<voi", "ce>", "2", "</voi", "ce>"),
    concat!("<sta", "ff>", "1", "</sta", "ff>"),
    concat!("<sta", "ff>", "2", "</sta", "ff>"),
];

#[derive(Debug, Clone)]
struct TextEdit {
    start: usize,
    end: usize,
    replacement: String,
}

#[derive(Debug, Clone, Default)]
pub struct AuthorizedWriterEntryBoundaryAudit {
    source_stage: String,
    writer_module_path: String,
    locator_resolver_module_path: String,
    verifier_token_origin_green: bool,
    writer_entry_definitions: usize,
    writer_entry_token_parameter_sites: usize,
    raw_candidate_writer_entry_points: usize,
    token_constructor_sites_in_writer: usize,
    token_clone_paths: usize,
    authority_writer_call_sites: usize,
    detector_writer_call_sites: usize,
    planner_writer_call_sites: usize,
    direct_legacy_writer_orchestrator_calls: usize,
    writer_input_authorized_token_only: bool,
    direct_legacy_writer_bypass_blocked: bool,
    literal_measure_selectors: usize,
    literal_pitch_selectors: usize,
    literal_voice_selectors: usize,
    literal_staff_selectors: usize,
    locator_identity_access_references: usize,
    locator_resolver_musical_selection_references: usize,
    boundary_invoked: bool,
    token_handed_to_writer: bool,
    token_consumed: bool,
    writer_token_used: bool,
    plan_rows_bound: usize,
    source_measure_found: bool,
    source_fingerprint_match: bool,
    source_events_found: usize,
    source_event_cursor_match: bool,
    main_lane_events: usize,
    hidden_carrier_events: usize,
    overlap_rest_events: usize,
    chord_normalizations: usize,
    beam_tags_written: usize,
    full_backups_inserted: usize,
    partial_backups_inserted: usize,
    parse_before_ok: bool,
    parse_after_ok: bool,
    note_count_before: usize,
    note_count_after: usize,
    candidate_raw_cursor_precheck_green: bool,
    candidate_partial_backups: usize,
    renderer_beam_groups_checked: usize,
    renderer_beam_member_pairs_checked: usize,
    renderer_interstitial_elements: usize,
    renderer_interstitial_directions: usize,
    renderer_fragment_parse_failures: usize,
    renderer_contiguous_target_members: bool,
    renderer_cross_staff_capability_green: bool,
    renderer_direction_relocation_plan: RendererNeutralDirectionPlacementPlanAudit,
    renderer_direction_relocation_authorization:
        RendererDirectionRelocationAuthorizationBoundaryAudit,
    renderer_direction_relocation_post_write: RendererDirectionRelocationPostWriteAudit,
    write_produced: bool,
    write_validated: bool,
    materialization_equivalence_green: bool,
    candidate_file_exact_match: bool,
    pdf_source_adopted: bool,
    productive_materialization_green: bool,
    pdf_source_label: String,
    pdf_hold_required: bool,
    verdict: String,
}

impl AuthorizedWriterEntryBoundaryAudit {
    #[cfg(test)]
    pub fn requires_pdf_hold(&self) -> bool {
        self.pdf_hold_required
    }

    pub fn write_produced(&self) -> bool {
        self.write_produced
    }

    pub fn renderer_contiguity_green(&self) -> bool {
        self.renderer_contiguous_target_members
    }

    #[cfg(test)]
    pub fn renderer_direction_relocation_plan(
        &self,
    ) -> &RendererNeutralDirectionPlacementPlanAudit {
        &self.renderer_direction_relocation_plan
    }

    #[cfg(test)]
    pub fn renderer_direction_relocation_authorization(
        &self,
    ) -> &RendererDirectionRelocationAuthorizationBoundaryAudit {
        &self.renderer_direction_relocation_authorization
    }

    pub(super) fn bind_renderer_direction_relocation_authorization(
        &mut self,
        audit: RendererDirectionRelocationAuthorizationBoundaryAudit,
    ) {
        self.renderer_direction_relocation_authorization = audit;
    }

    #[cfg(test)]
    pub fn renderer_direction_relocation_post_write(
        &self,
    ) -> &RendererDirectionRelocationPostWriteAudit {
        &self.renderer_direction_relocation_post_write
    }

    pub(super) fn bind_renderer_direction_relocation_post_write(
        &mut self,
        audit: RendererDirectionRelocationPostWriteAudit,
    ) {
        self.renderer_interstitial_elements = audit.renderer_interstitial_elements();
        self.renderer_interstitial_directions = 0;
        self.renderer_fragment_parse_failures = 0;
        self.renderer_contiguous_target_members = audit.renderer_contiguous_target_members();
        self.pdf_hold_required = self.write_produced && !audit.green();
        self.renderer_direction_relocation_post_write = audit;
    }

    pub(super) fn token_consumed(&self) -> bool {
        self.token_consumed
    }

    pub fn bind_pdf_source_adoption_truth(
        &mut self,
        raw_cursor_validated: bool,
        renderer_cross_staff_capability_green: bool,
        materialization_equivalence_green: bool,
        candidate_file_exact_match: bool,
        pdf_source_adopted: bool,
        pdf_source_label: &str,
    ) {
        self.renderer_cross_staff_capability_green = renderer_cross_staff_capability_green;
        self.materialization_equivalence_green = materialization_equivalence_green;
        self.candidate_file_exact_match = candidate_file_exact_match;
        self.write_validated = self.write_produced
            && raw_cursor_validated
            && self.renderer_contiguous_target_members
            && self.renderer_cross_staff_capability_green
            && self.materialization_equivalence_green
            && self.candidate_file_exact_match;
        self.pdf_source_adopted = self.write_produced && self.write_validated && pdf_source_adopted;
        self.productive_materialization_green =
            self.write_produced && self.write_validated && self.pdf_source_adopted;
        self.pdf_hold_required = self.write_produced && !self.productive_materialization_green;
        self.pdf_source_label = pdf_source_label.to_string();
        if self.productive_materialization_green {
            self.verdict =
                "authorized_cursor_neutral_candidate_validated_and_pdf_adopted_phase6c".to_string();
        } else if self.write_produced && !self.renderer_contiguous_target_members {
            self.verdict =
                "authorized_cursor_neutral_candidate_held_by_renderer_contiguity_precondition"
                    .to_string();
        } else if self.write_produced && !self.renderer_cross_staff_capability_green {
            self.verdict =
                "authorized_cursor_neutral_candidate_xml_green_but_renderer_capability_unproven"
                    .to_string();
        } else if self.write_produced && !self.write_validated {
            self.verdict =
                "authorized_cursor_neutral_candidate_quarantined_by_post_write_validation"
                    .to_string();
        } else if self.write_produced && self.write_validated && !self.pdf_source_adopted {
            self.verdict =
                "authorized_cursor_neutral_candidate_validated_but_pdf_not_adopted".to_string();
        }
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "AuthorizedWriterEntryBoundaryAudit v{}: Project {}; SourceStage {}; WriterModule {}; LocatorResolverModule {}; VerifierTokenOriginGreen {}; WriterEntryDefinitions {}; WriterEntryTokenParameterSites {}; RawCandidateWriterEntryPoints {}; TokenConstructorSitesInWriter {}; TokenClonePaths {}; AuthorityWriterCallSites {}; DetectorWriterCallSites {}; PlannerWriterCallSites {}; DirectLegacyWriterOrchestratorCalls {}; WriterInputAuthorizedTokenOnly {}; DirectLegacyWriterBypassBlocked {}; LiteralMeasureSelectors {}; LiteralPitchSelectors {}; LiteralVoiceSelectors {}; LiteralStaffSelectors {}; LocatorIdentityAccessReferences {}; LocatorResolverMusicalSelectionReferences {}; BoundaryInvoked {}; TokenHandedToWriter {}; TokenConsumed {}; WriterTokenUsed {}; PlanRowsBound {}; SourceMeasureFound {}; SourceFingerprintMatch {}; SourceEventsFound {}; SourceEventCursorMatch {}; MainLaneEvents {}; HiddenCarrierEvents {}; OverlapRestEvents {}; ChordNormalizations {}; BeamTagsWritten {}; FullBackupsInserted {}; PartialBackupsInserted {}; ParseBeforeAfter {}/{}; NoteCountBeforeAfter {}/{}; CandidateRawCursorPrecheckGreen {}; CandidatePartialBackups {}; RendererBeamGroupsChecked {}; RendererBeamMemberPairsChecked {}; RendererInterstitialElements {}; RendererInterstitialDirections {}; RendererFragmentParseFailures {}; RendererContiguousTargetMembers {}; RendererCrossStaffCapabilityGreen {}; WriteProduced {}; WriteValidated {}; MaterializationEquivalenceGreen {}; CandidateFileExactMatch {}; PdfSourceAdopted {}; ProductiveMaterializationGreen {}; PdfHoldRequired {}; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            self.source_stage,
            self.writer_module_path,
            self.locator_resolver_module_path,
            yes_no(self.verifier_token_origin_green),
            self.writer_entry_definitions,
            self.writer_entry_token_parameter_sites,
            self.raw_candidate_writer_entry_points,
            self.token_constructor_sites_in_writer,
            self.token_clone_paths,
            self.authority_writer_call_sites,
            self.detector_writer_call_sites,
            self.planner_writer_call_sites,
            self.direct_legacy_writer_orchestrator_calls,
            yes_no(self.writer_input_authorized_token_only),
            yes_no(self.direct_legacy_writer_bypass_blocked),
            self.literal_measure_selectors,
            self.literal_pitch_selectors,
            self.literal_voice_selectors,
            self.literal_staff_selectors,
            self.locator_identity_access_references,
            self.locator_resolver_musical_selection_references,
            yes_no(self.boundary_invoked),
            yes_no(self.token_handed_to_writer),
            yes_no(self.token_consumed),
            yes_no(self.writer_token_used),
            self.plan_rows_bound,
            yes_no(self.source_measure_found),
            yes_no(self.source_fingerprint_match),
            self.source_events_found,
            yes_no(self.source_event_cursor_match),
            self.main_lane_events,
            self.hidden_carrier_events,
            self.overlap_rest_events,
            self.chord_normalizations,
            self.beam_tags_written,
            self.full_backups_inserted,
            self.partial_backups_inserted,
            yes_no(self.parse_before_ok),
            yes_no(self.parse_after_ok),
            self.note_count_before,
            self.note_count_after,
            yes_no(self.candidate_raw_cursor_precheck_green),
            self.candidate_partial_backups,
            self.renderer_beam_groups_checked,
            self.renderer_beam_member_pairs_checked,
            self.renderer_interstitial_elements,
            self.renderer_interstitial_directions,
            self.renderer_fragment_parse_failures,
            yes_no(self.renderer_contiguous_target_members),
            yes_no(self.renderer_cross_staff_capability_green),
            yes_no(self.write_produced),
            yes_no(self.write_validated),
            yes_no(self.materialization_equivalence_green),
            yes_no(self.candidate_file_exact_match),
            yes_no(self.pdf_source_adopted),
            yes_no(self.productive_materialization_green),
            yes_no(self.pdf_hold_required),
            yes_no(self.write_produced),
            self.verdict,
        )
    }

}

pub(super) fn blocked_authorized_writer_entry(
    source_stage: &str,
) -> AuthorizedWriterEntryBoundaryAudit {
    let mut audit = build_boundary_audit(source_stage);
    audit.verifier_token_origin_green = false;
    audit.verdict = "authorized_writer_not_invoked_no_verifier_token".to_string();
    audit
}

pub(super) fn authorized_reference_writer_entry(
    token: AuthorizedRepairCandidate<StructuralRepairIntent>,
    source_xml: &str,
    source_stage: &str,
) -> (
    String,
    AuthorizedWriterEntryBoundaryAudit,
    Option<ProvenRendererDirectionRelocationPlan>,
    Option<ExactTransformationReceipt>,
) {
    let mut audit = build_boundary_audit(source_stage);
    audit.boundary_invoked = true;
    audit.token_handed_to_writer = true;
    audit.token_consumed = true;
    audit.writer_token_used = true;
    audit.parse_before_ok = Document::parse(source_xml).is_ok();
    audit.note_count_before = source_xml.matches("<note").count();

    let (candidate_kind, locator, plan) = token.into_writer_parts();
    audit.plan_rows_bound = plan.rows.len();
    if candidate_kind != StructuralRepairIntent::CompleteTailBeatBeam
        || !audit.verifier_token_origin_green
        || !audit.writer_input_authorized_token_only
        || !audit.direct_legacy_writer_bypass_blocked
    {
        audit.verdict = "authorized_writer_prerequisite_blocked".to_string();
        return (source_xml.to_string(), audit, None, None);
    }

    let mut direction_relocation_plan = None;
    let (candidate, receipt) = apply_authorized_serialization_plan(
        source_xml,
        &locator,
        &plan,
        source_stage,
        &mut audit,
        &mut direction_relocation_plan,
    );
    audit.parse_after_ok = Document::parse(&candidate).is_ok();
    audit.note_count_after = candidate.matches("<note").count();

    let execution_green = audit.source_measure_found
        && audit.source_fingerprint_match
        && audit.source_events_found == plan.rows.len()
        && audit.source_event_cursor_match
        && audit.main_lane_events == 5
        && audit.hidden_carrier_events == 1
        && audit.overlap_rest_events == 1
        && audit.chord_normalizations == 1
        && audit.beam_tags_written == 7
        && audit.full_backups_inserted == 1
        && audit.partial_backups_inserted == 0
        && audit.parse_before_ok
        && audit.parse_after_ok
        && audit.note_count_before == audit.note_count_after
        && audit.candidate_raw_cursor_precheck_green
        && audit.candidate_partial_backups == 0
        && candidate != source_xml;
    audit.write_produced = execution_green;
    audit.pdf_hold_required = execution_green && !audit.renderer_contiguous_target_members;
    audit.verdict = if execution_green && audit.renderer_contiguous_target_members {
        "authorized_cursor_neutral_render_candidate_written_pending_phase6c_validation".to_string()
    } else if execution_green {
        "authorized_cursor_neutral_render_candidate_written_renderer_contiguity_hold".to_string()
    } else {
        "authorized_cursor_neutral_writer_rejected_by_execution_invariant".to_string()
    };
    if execution_green {
        (candidate, audit, direction_relocation_plan, receipt)
    } else {
        (source_xml.to_string(), audit, None, None)
    }
}

fn build_boundary_audit(source_stage: &str) -> AuthorizedWriterEntryBoundaryAudit {
    let writer_entry_definitions = WRITER_MODULE_SOURCE
        .matches(WRITER_ENTRY_DEFINITION_PATTERN)
        .count();
    let writer_entry_token_parameter_sites = WRITER_MODULE_SOURCE
        .matches(WRITER_ENTRY_TOKEN_PARAMETER_PATTERN)
        .count();
    let raw_candidate_writer_entry_points = WRITER_MODULE_SOURCE
        .matches(RAW_INTENT_PARAMETER_PATTERN)
        .count()
        + WRITER_MODULE_SOURCE
            .matches(RAW_LOCATED_INTENT_PARAMETER_PATTERN)
            .count();
    let token_constructor_sites_in_writer = WRITER_MODULE_SOURCE
        .matches(TOKEN_CONSTRUCTOR_PATTERN)
        .count();
    let token_clone_paths = VERIFIER_MODULE_SOURCE
        .matches(TOKEN_CLONE_DERIVE_PATTERN)
        .count()
        + VERIFIER_MODULE_SOURCE
            .matches(TOKEN_CLONE_CALL_PATTERN)
            .count()
        + WRITER_MODULE_SOURCE
            .matches(TOKEN_CLONE_CALL_PATTERN)
            .count();
    let authority_writer_call_sites = AUTHORITY_MODULE_SOURCE
        .matches(WRITER_ENTRY_NAME_PATTERN)
        .count();
    let detector_writer_call_sites = DETECTOR_SOURCE.matches(WRITER_ENTRY_NAME_PATTERN).count();
    let planner_writer_call_sites = PLANNER_SOURCE.matches(WRITER_ENTRY_NAME_PATTERN).count();
    let direct_legacy_writer_orchestrator_calls = ORCHESTRATOR_SOURCE
        .matches(LEGACY_DIRECT_WRITER_CALL_PATTERN)
        .count();
    let writer_input_authorized_token_only = writer_entry_definitions == 1
        && writer_entry_token_parameter_sites == 1
        && raw_candidate_writer_entry_points == 0
        && token_constructor_sites_in_writer == 0
        && token_clone_paths == 0;
    let locator_identity_access_references = LOCATOR_IDENTITY_ACCESS_PATTERNS
        .iter()
        .map(|pattern| WRITER_MODULE_SOURCE.matches(pattern).count())
        .sum();
    let locator_resolver_musical_selection_references = LOCATOR_RESOLVER_SELECTION_PATTERNS
        .iter()
        .map(|pattern| LOCATOR_RESOLVER_SOURCE.matches(pattern).count())
        .sum();
    let direct_legacy_writer_bypass_blocked = direct_legacy_writer_orchestrator_calls == 0
        && authority_writer_call_sites == 1
        && detector_writer_call_sites == 0
        && planner_writer_call_sites == 0
        && locator_identity_access_references == 0
        && locator_resolver_musical_selection_references == 0;
    let literal_measure_selectors = WRITER_MODULE_SOURCE
        .matches(LITERAL_MEASURE_SELECTOR_PATTERN)
        .count();
    let literal_pitch_selectors = WRITER_MODULE_SOURCE
        .matches(LITERAL_PITCH_STEP_PATTERN)
        .count()
        + WRITER_MODULE_SOURCE
            .matches(LITERAL_PITCH_OCTAVE_PATTERN)
            .count();
    let literal_voice_selectors = WRITER_MODULE_SOURCE
        .matches(LITERAL_VOICE_SELECTOR_PATTERN)
        .count();
    let literal_staff_selectors = WRITER_MODULE_SOURCE
        .matches(LITERAL_STAFF_ONE_SELECTOR_PATTERN)
        .count()
        + WRITER_MODULE_SOURCE
            .matches(LITERAL_STAFF_TWO_SELECTOR_PATTERN)
            .count();

    AuthorizedWriterEntryBoundaryAudit {
        source_stage: source_stage.to_string(),
        writer_module_path: WRITER_MODULE_PATH.to_string(),
        locator_resolver_module_path: LOCATOR_RESOLVER_MODULE_PATH.to_string(),
        verifier_token_origin_green: true,
        writer_entry_definitions,
        writer_entry_token_parameter_sites,
        raw_candidate_writer_entry_points,
        token_constructor_sites_in_writer,
        token_clone_paths,
        authority_writer_call_sites,
        detector_writer_call_sites,
        planner_writer_call_sites,
        direct_legacy_writer_orchestrator_calls,
        writer_input_authorized_token_only,
        direct_legacy_writer_bypass_blocked,
        literal_measure_selectors,
        literal_pitch_selectors,
        literal_voice_selectors,
        literal_staff_selectors,
        locator_identity_access_references,
        locator_resolver_musical_selection_references,
        pdf_source_label: "MusicXML".to_string(),
        ..AuthorizedWriterEntryBoundaryAudit::default()
    }
}

fn apply_authorized_serialization_plan(
    source_xml: &str,
    locator: &super::StructuralLocator,
    plan: &AuthorizedSerializationPlan,
    source_stage: &str,
    audit: &mut AuthorizedWriterEntryBoundaryAudit,
    direction_relocation_plan: &mut Option<ProvenRendererDirectionRelocationPlan>,
) -> (String, Option<ExactTransformationReceipt>) {
    let Some(target) = resolve_authorized_serialization_target(source_xml, locator, plan) else {
        return (source_xml.to_string(), None);
    };
    audit.source_measure_found = true;
    audit.source_fingerprint_match = target.source_fingerprint_match;
    audit.source_events_found = target.source_events_found;
    audit.source_event_cursor_match = target.source_event_cursor_match;
    let (direction_plan_audit, proven_direction_plan) =
        audit_renderer_neutral_direction_placement_plan(&target, source_xml, plan, source_stage);
    audit.renderer_direction_relocation_plan = direction_plan_audit;
    *direction_relocation_plan = proven_direction_plan;

    let mut main_rows = plan
        .rows
        .iter()
        .filter(|row| row.planned_lane == AuthorizedPlannedLane::MainVoiceRun)
        .collect::<Vec<_>>();
    main_rows.sort_by_key(|row| row.planned_xml_order);
    let hidden_rows = plan
        .rows
        .iter()
        .filter(|row| row.planned_lane == AuthorizedPlannedLane::HiddenCarrierAuxiliary)
        .collect::<Vec<_>>();
    let overlap_rows = plan
        .rows
        .iter()
        .filter(|row| row.planned_lane == AuthorizedPlannedLane::OverlapRestAuxiliary)
        .collect::<Vec<_>>();
    audit.main_lane_events = main_rows.len();
    audit.hidden_carrier_events = hidden_rows.len();
    audit.overlap_rest_events = overlap_rows.len();
    if main_rows.len() != 5 || hidden_rows.len() != 1 || overlap_rows.len() != 1 {
        return (source_xml.to_string(), None);
    }

    let hidden_row = hidden_rows[0];
    let Some(hidden_record) = target.record(hidden_row.source_event_ordinal) else {
        return (source_xml.to_string(), None);
    };

    let renderer_contiguity = target.renderer_beam_member_contiguity(source_xml, plan);
    audit.renderer_beam_groups_checked = renderer_contiguity.beam_groups_checked;
    audit.renderer_beam_member_pairs_checked = renderer_contiguity.member_pairs_checked;
    audit.renderer_interstitial_elements = renderer_contiguity.interstitial_elements;
    audit.renderer_interstitial_directions = renderer_contiguity.interstitial_directions;
    audit.renderer_fragment_parse_failures = renderer_contiguity.fragment_parse_failures;
    audit.renderer_contiguous_target_members = renderer_contiguity.green;

    let mut edits = Vec::new();
    let hidden_text = &source_xml[hidden_record.range.clone()];
    edits.push(TextEdit {
        start: hidden_record.range.start,
        end: hidden_record.range.end,
        replacement: String::new(),
    });

    for row in &main_rows {
        let Some(record) = target.record(row.source_event_ordinal) else {
            return (source_xml.to_string(), None);
        };
        let original = &source_xml[record.range.clone()];
        let (rewritten, chord_normalized, beams_written) = rewrite_main_lane_note(original, row);
        audit.chord_normalizations += chord_normalized;
        audit.beam_tags_written += beams_written;
        edits.push(TextEdit {
            start: record.range.start,
            end: record.range.end,
            replacement: rewritten,
        });
    }

    let insertion = format!(
        "\n      <forward><duration>{}</duration><voice>{}</voice><staff>{}</staff></forward>\n{}\n      <backup><duration>{}</duration></backup>",
        hidden_row.planned_cursor_before,
        hidden_record.voice,
        hidden_record.staff,
        hidden_text,
        hidden_row.planned_cursor_after,
    );
    let insertion_backup_end = target.insertion_backup_end();
    edits.push(TextEdit {
        start: insertion_backup_end,
        end: insertion_backup_end,
        replacement: insertion,
    });
    audit.full_backups_inserted = 1;
    audit.partial_backups_inserted = 0;
    let geometry = edits.iter().map(|edit|
        ExactEdit::bound(edit.start, edit.end, edit.replacement.clone())
    ).collect();
    let candidate = apply_text_edits(source_xml, edits);
    let receipt = mint_cursor_neutral_receipt(source_xml, &candidate, geometry);
    let (raw_green, partial_backups) = target.raw_cursor_precheck(&candidate);
    audit.candidate_raw_cursor_precheck_green = raw_green;
    audit.candidate_partial_backups = partial_backups;
    (candidate, receipt)
}

fn mint_cursor_neutral_receipt(
    pre: &str, post: &str, geometry: Vec<ExactEdit>,
) -> Option<ExactTransformationReceipt> {
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        CursorNeutralRenderExactMint { _sealed: () }, pre, post, geometry,
    );
    assert!(pre == post || receipt.is_some(), "#2 changed bytes require its own EXACT receipt");
    receipt
}

fn rewrite_main_lane_note(
    original: &str,
    row: &AuthorizedSerializationPlanRow,
) -> (String, usize, usize) {
    let mut rewritten = remove_beam_elements(original);
    let chord_present = rewritten.contains("<chord");
    let mut chord_normalized = 0_usize;
    if row.planned_chord_member {
        if !chord_present {
            return (original.to_string(), 0, 0);
        }
    } else if chord_present {
        let Some(chord_start) = rewritten.find("<chord") else {
            return (original.to_string(), 0, 0);
        };
        let Some(chord_end_rel) = rewritten[chord_start..].find('>') else {
            return (original.to_string(), 0, 0);
        };
        let chord_end = chord_start + chord_end_rel + 1;
        rewritten.replace_range(chord_start..chord_end, "");
        chord_normalized = 1;
    }

    let Some(note_close) = rewritten.rfind("</note>") else {
        return (original.to_string(), 0, 0);
    };
    let beam_insert = ["<notations", "<lyric", "<play", "<listen"]
        .into_iter()
        .filter_map(|marker| rewritten.find(marker))
        .min()
        .unwrap_or(note_close);
    let beam_insert = rewritten[..beam_insert]
        .rfind('\n')
        .map_or(beam_insert, |line_break| line_break + 1);
    let mut beam_xml = String::new();
    let mut beams_written = 0_usize;
    for (number, state) in [
        (1_u8, row.planned_primary_beam),
        (2_u8, row.planned_secondary_beam),
    ] {
        let Some(state_text) = structural_beam_state_text(state) else {
            continue;
        };
        let _ = write!(
            beam_xml,
            "\n        <beam number=\"{number}\">{state_text}</beam>",
        );
        beams_written += 1;
    }
    rewritten.insert_str(beam_insert, &beam_xml);
    (rewritten, chord_normalized, beams_written)
}

fn remove_beam_elements(original: &str) -> String {
    let mut rewritten = original.to_string();
    while let Some(start) = rewritten.find("<beam") {
        let Some(open_end_rel) = rewritten[start..].find('>') else {
            return original.to_string();
        };
        let content_start = start + open_end_rel + 1;
        let Some(close_start_rel) = rewritten[content_start..].find("</beam>") else {
            return original.to_string();
        };
        let end = content_start + close_start_rel + "</beam>".len();
        rewritten.replace_range(start..end, "");
    }
    rewritten
}

fn structural_beam_state_text(state: StructuralBeamState) -> Option<&'static str> {
    match state {
        StructuralBeamState::Absent => None,
        StructuralBeamState::Begin => Some("begin"),
        StructuralBeamState::Continue => Some("continue"),
        StructuralBeamState::End => Some("end"),
    }
}

fn apply_text_edits(source: &str, mut edits: Vec<TextEdit>) -> String {
    edits.sort_by_key(|edit| (edit.start, edit.end));
    let mut output = String::with_capacity(source.len());
    let mut cursor = 0_usize;
    for edit in edits {
        if edit.start < cursor || edit.start > edit.end || edit.end > source.len() {
            return source.to_string();
        }
        let Some(prefix) = source.get(cursor..edit.start) else {
            return source.to_string();
        };
        output.push_str(prefix);
        output.push_str(&edit.replacement);
        cursor = edit.end;
    }
    let Some(tail) = source.get(cursor..) else {
        return source.to_string();
    };
    output.push_str(tail);
    output
}

pub fn append_authorized_writer_entry_boundary_audit_section(
    out: &mut String,
    audit: &AuthorizedWriterEntryBoundaryAudit,
) {
    out.push_str("\nAUTHORIZED-WRITER-ENTRY-BOUNDARY-AUDIT\n");
    out.push_str("--------------------------------------------\n");
    let _ = writeln!(
        out,
        "Phase: 6C authorized candidate write with validated PDF adoption; SourceStage {}; WriterModule {}; LocatorResolverModule {}; VerifierTokenOriginGreen {}.",
        audit.source_stage,
        audit.writer_module_path,
        audit.locator_resolver_module_path,
        yes_no(audit.verifier_token_origin_green),
    );
    let _ = writeln!(
        out,
        "InputBoundary: WriterEntryDefinitions {}; WriterEntryTokenParameterSites {}; WriterInputAuthorizedTokenOnly {}; RawCandidateWriterEntryPoints {}; TokenConstructorSitesInWriter {}; TokenClonePaths {}; AuthorityWriterCallSites {}.",
        audit.writer_entry_definitions,
        audit.writer_entry_token_parameter_sites,
        yes_no(audit.writer_input_authorized_token_only),
        audit.raw_candidate_writer_entry_points,
        audit.token_constructor_sites_in_writer,
        audit.token_clone_paths,
        audit.authority_writer_call_sites,
    );
    let _ = writeln!(
        out,
        "BypassBoundary: DetectorWriterCallSites {}; PlannerWriterCallSites {}; DirectLegacyWriterOrchestratorCalls {}; DirectLegacyWriterBypassBlocked {}; LiteralSelectors measure/pitch/voice/staff {}/{}/{}/{}; LocatorIdentityAccessReferences {}; LocatorResolverMusicalSelectionReferences {}.",
        audit.detector_writer_call_sites,
        audit.planner_writer_call_sites,
        audit.direct_legacy_writer_orchestrator_calls,
        yes_no(audit.direct_legacy_writer_bypass_blocked),
        audit.literal_measure_selectors,
        audit.literal_pitch_selectors,
        audit.literal_voice_selectors,
        audit.literal_staff_selectors,
        audit.locator_identity_access_references,
        audit.locator_resolver_musical_selection_references,
    );
    let _ = writeln!(
        out,
        "Execution: BoundaryInvoked {}; TokenHandedToWriter {}; TokenConsumed {}; WriterTokenUsed {}; PlanRowsBound {}; SourceMeasureFound {}; SourceFingerprintMatch {}; SourceEventsFound {}; SourceEventCursorMatch {}; Main/Hidden/Rest {}/{}/{}; ChordNormalizations {}; BeamTagsWritten {}; FullBackupsInserted {}; PartialBackupsInserted {}; ParseBeforeAfter {}/{}; NoteCount {}/{}; CandidateRawCursorPrecheckGreen {}; CandidatePartialBackups {}; RendererBeamGroupsChecked {}; RendererBeamMemberPairsChecked {}; RendererInterstitialElements {}; RendererInterstitialDirections {}; RendererFragmentParseFailures {}; RendererContiguousTargetMembers {}; WriteProduced {}.",
        yes_no(audit.boundary_invoked),
        yes_no(audit.token_handed_to_writer),
        yes_no(audit.token_consumed),
        yes_no(audit.writer_token_used),
        audit.plan_rows_bound,
        yes_no(audit.source_measure_found),
        yes_no(audit.source_fingerprint_match),
        audit.source_events_found,
        yes_no(audit.source_event_cursor_match),
        audit.main_lane_events,
        audit.hidden_carrier_events,
        audit.overlap_rest_events,
        audit.chord_normalizations,
        audit.beam_tags_written,
        audit.full_backups_inserted,
        audit.partial_backups_inserted,
        yes_no(audit.parse_before_ok),
        yes_no(audit.parse_after_ok),
        audit.note_count_before,
        audit.note_count_after,
        yes_no(audit.candidate_raw_cursor_precheck_green),
        audit.candidate_partial_backups,
        audit.renderer_beam_groups_checked,
        audit.renderer_beam_member_pairs_checked,
        audit.renderer_interstitial_elements,
        audit.renderer_interstitial_directions,
        audit.renderer_fragment_parse_failures,
        yes_no(audit.renderer_contiguous_target_members),
        yes_no(audit.write_produced),
    );
    let _ = writeln!(
        out,
        "AdoptionTruth: WriteValidated {}; RendererContiguousTargetMembers {}; RendererCrossStaffCapabilityGreen {}; MaterializationEquivalenceGreen {}; CandidateFileExactMatch {}; PdfSourceAdopted {}; ProductiveMaterializationGreen {}; PdfSource {}; PdfHoldRequired {}; MusicXmlWrite nein; MasterApply nein; MusicMutation nein.",
        yes_no(audit.write_validated),
        yes_no(audit.renderer_contiguous_target_members),
        yes_no(audit.renderer_cross_staff_capability_green),
        yes_no(audit.materialization_equivalence_green),
        yes_no(audit.candidate_file_exact_match),
        yes_no(audit.pdf_source_adopted),
        yes_no(audit.productive_materialization_green),
        audit.pdf_source_label,
        yes_no(audit.pdf_hold_required),
    );
    let _ = writeln!(out, "Urteil: {}.", audit.verdict);
    out.push_str("Interpretation: Der Writer erhält genau ein nicht klonbares Verifier-Token mit gebundenem opakem Locator und gebundenem Serialisierungsplan. Er führt keine eigene musikalische Suche aus. In Phase 6C darf der cursorneutrale Renderkandidat nur dann PDF-Quelle werden, wenn globaler Raw-Cursor-Beweis, renderer-seitig zusammenhängende Balkenmitglieder ohne dazwischenliegende XML-Elemente, positiv bewiesene Renderer-Fähigkeit für die konkrete staffübergreifende Balkenklasse, unabhängige tatsächliche Ereignisäquivalenz und exakte Kandidatendateibindung gemeinsam grün sind.\n");
}


#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub(in crate::structural_write_authority) fn executed_cursor_event() -> (String, String, crate::master_state_identity::ExactTransformationReceipt) {
        let pre = super::super::tests::bound_cursor_source();
        let (views, _) = super::super::adapt_structural_snapshot_read_only(&pre, 96, "boundary-fixture");
        let intents = super::super::detect_structural_repair_intents_read_only(&views);
        let detector = super::super::audit_identity_free_detector_module_boundary(views.len(), intents.len(), "boundary-fixture");
        let semantic = super::super::audit_identity_free_semantic_event_equivalence(&intents, views.len(), "boundary-fixture");
        let metamorphic = super::super::audit_identity_free_metamorphic_safety_net(&pre, 96, "boundary-fixture");
        let (_, token) = super::super::authorization_verifier::authorize_repair_candidate_for_writer(
            &intents, &detector, &semantic, &metamorphic, "boundary-fixture");
        let (post, audit, _, receipt) = authorized_reference_writer_entry(
            token.unwrap(), &pre, "boundary-fixture");
        assert!(audit.write_produced());
        (pre, post, receipt.unwrap())
    }



    #[test]
    fn cursor_exact_receipt_rejects_a_combined_direction_post() {
        let pre = "cursor direction";
        assert!(mint_cursor_neutral_receipt(pre, pre, Vec::new()).is_none());
        let own = vec![ExactEdit::bound(0, 6, "neutral".to_string())];
        let post = apply_text_edits(pre, vec![TextEdit { start: 0, end: 6, replacement: "neutral".to_string() }]);
        let receipt = mint_cursor_neutral_receipt(pre, &post, own).unwrap();
        assert_eq!(receipt.realize_post_document(pre).as_deref(), Some(post.as_str()));
        assert!(std::panic::catch_unwind(|| mint_cursor_neutral_receipt(pre,
            "neutral relocated", vec![ExactEdit::bound(0, 6, "neutral".to_string())])).is_err());
    }

    #[test]
    fn writer_entry_signature_accepts_private_token_and_source_data_only() {
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(WRITER_ENTRY_DEFINITION_PATTERN)
                .count(),
            1
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(WRITER_ENTRY_TOKEN_PARAMETER_PATTERN)
                .count(),
            1
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(RAW_INTENT_PARAMETER_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(RAW_LOCATED_INTENT_PARAMETER_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(TOKEN_CONSTRUCTOR_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            LOCATOR_IDENTITY_ACCESS_PATTERNS
                .iter()
                .map(|pattern| WRITER_MODULE_SOURCE.matches(pattern).count())
                .sum::<usize>(),
            0
        );
        assert_eq!(
            LOCATOR_RESOLVER_SELECTION_PATTERNS
                .iter()
                .map(|pattern| LOCATOR_RESOLVER_SOURCE.matches(pattern).count())
                .sum::<usize>(),
            0
        );
        assert_eq!(
            VERIFIER_MODULE_SOURCE
                .matches(TOKEN_CLONE_DERIVE_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            VERIFIER_MODULE_SOURCE
                .matches(TOKEN_CLONE_CALL_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            WRITER_MODULE_SOURCE
                .matches(TOKEN_CLONE_CALL_PATTERN)
                .count(),
            0
        );
    }

    #[test]
    fn production_sources_have_no_direct_legacy_writer_call() {
        assert_eq!(
            ORCHESTRATOR_SOURCE
                .matches(LEGACY_DIRECT_WRITER_CALL_PATTERN)
                .count(),
            0
        );
        assert_eq!(
            DETECTOR_SOURCE.matches(WRITER_ENTRY_NAME_PATTERN).count(),
            0
        );
        assert_eq!(PLANNER_SOURCE.matches(WRITER_ENTRY_NAME_PATTERN).count(), 0);
        assert_eq!(
            AUTHORITY_MODULE_SOURCE
                .matches(WRITER_ENTRY_NAME_PATTERN)
                .count(),
            1
        );
    }


}

#[cfg(test)]
#[path = "ssc_cursor_origin_tests.rs"]
mod ssc_cursor_origin_tests;
