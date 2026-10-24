//! Identity-free musical write authorization contract.
//!
//! v0.10.1245 preserves the v0.10.1242 zero-external-surface cutover, deletes the retired cross-staff direct writer outside this authority, and keeps all productive MusicXML writes behind typed capabilities. No public raw XML/file writer is introduced.
//!
//! v0.10.1230 closes the reportish-name scanner blind spot and immediately centralizes the two newly visible XML audit sinks plus the clef-context XML delta. The typed, source-bound capability cutover and the 78-surface external register remain closed; no writer receives musical selection authority.
//!
//! v0.10.1225 preserves the first family-batched shrink and closes its compile integration: the four remaining
//! direct write surfaces in notation_endpoint_layout_core move behind four
//! separate source-bound capabilities. The external bypass register falls from
//! 112 to 108 without introducing a generic raw-XML writer.
//!
//! v0.10.1223 performs the eighth post-register shrink: insertion of the final
//! right light-heavy barline moves behind one source-bound central capability.
//! The external bypass register falls from 113 to 112 surfaces without changing
//! notation or PDF.
//!
//! v0.10.1222 performs the seventh post-register shrink: global visual layout
//! defaults insertion moves behind one source-bound central capability. The
//! external bypass register falls from 114 to 113 surfaces without changing
//! notation or PDF.
//!
//! v0.10.1221 performs the sixth post-register shrink: existing staff-distance
//! raising and local staff-layout insertion become one source-bound central
//! transaction. The external bypass register falls from 116 to 114 surfaces
//! without changing notation or PDF.
//!
//! v0.10.1220 performs the fifth post-register shrink: removal of existing
//! layout-break prints and insertion of one page break become one source-bound
//! central transaction. The external bypass register falls from 118 to 116
//! surfaces without changing notation or PDF.
//!
//! v0.10.1219 performs the fourth post-register shrink: minimum measure-width
//! mutation moves behind a source-bound central writer. The external bypass
//! register falls from 119 to 118 surfaces without changing notation or PDF.
//!
//! v0.10.1218 performs the third post-register shrink: insertion of a bare
//! `new-system` print moves behind a source-bound central writer. The external
//! bypass register falls from 120 to 119 surfaces without changing notation or
//! PDF output.
//!
//! v0.10.1217 performs the second post-register shrink: the final-short-system
//! `new-system` neutralization moves behind a source-bound central writer. The
//! external bypass register therefore falls from 121 to 120 surfaces without
//! changing notation or PDF output.
//!
//! v0.10.1216 performed the first post-register shrink: redundant bare
//! system-break suppression moved behind a source-bound central writer. The
//! external bypass register fell from 122 to 121 surfaces without changing
//! notation or PDF output.
//!
//! v0.10.1597 keeps the productive Gould authority unchanged and replaces
//! its local event/view/state/write-intent quartet with the shared identity-free
//! BeamTimeline and BeamPlan model. The verifier token now carries the plan
//! directly; legacy parity, the token-only writer and the sole render sink stay
//! unchanged.
//!
//! v0.10.1215 keeps the v0.10.1212 Gould cutover unchanged while a separate
//! build gate freezes all remaining external write surfaces as a hard,
//! only-shrinking bypass register.
//!
//! v0.10.1212 performs the first productive Gould central-authority cutover.
//! The general metric 16th-eighth-16th detector remains identity-free, the
//! independent verifier issues move-only source-bound capabilities, and only a
//! central candidate with exact legacy-shadow parity can reach the typed render
//! sink. The legacy algorithm remains available solely as an isolated veto
//! reference and has no productive file-write authority.
//!
//! The wider structural-authority modules remain at their previously documented
//! proof stages. Absolute project, measure, pitch, voice and staff identities
//! stay outside detector-facing structural views and cannot create write rights.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructuralBeamState {
    Absent,
    Begin,
    Continue,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructuralStaffRelation {
    SameAsAnchor,
    DifferentFromAnchor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructuralVoiceRelation {
    SameRun,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StructuralCarrierRole {
    Sounding,
    Rest,
    HiddenCarrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructuralEvent {
    /// Relative position from the start of the two-quarter structural window.
    pub onset_in_window: i32,
    pub duration: i32,
    pub primary_beam: StructuralBeamState,
    pub secondary_beam: StructuralBeamState,
    pub staff_relation: StructuralStaffRelation,
    pub voice_relation: StructuralVoiceRelation,
    pub carrier_role: StructuralCarrierRole,
    pub visible: bool,
    pub rest_like: bool,
    pub chord_member: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralView {
    metric_units_per_quarter: i32,
    events: Vec<StructuralEvent>,
}

impl StructuralView {
    pub fn new(metric_units_per_quarter: i32, events: Vec<StructuralEvent>) -> Self {
        Self {
            metric_units_per_quarter: metric_units_per_quarter.max(1),
            events,
        }
    }

    pub fn metric_units_per_quarter(&self) -> i32 {
        self.metric_units_per_quarter
    }

    pub fn events(&self) -> &[StructuralEvent] {
        &self.events
    }

    #[cfg(test)]
    pub fn fingerprint(&self) -> (i32, Vec<StructuralEvent>) {
        (self.metric_units_per_quarter, self.events.clone())
    }
}

/// Opaque bridge back to source XML.  The detector never receives this value.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuralRepairIntent {
    CompleteTailBeatBeam,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedStructuralRepairIntent {
    intent: StructuralRepairIntent,
    located_view: LocatedStructuralView,
}

impl LocatedStructuralRepairIntent {
    fn new(intent: StructuralRepairIntent, located_view: LocatedStructuralView) -> Self {
        Self {
            intent,
            located_view,
        }
    }

    fn into_parts(self) -> (StructuralRepairIntent, LocatedStructuralView) {
        (self.intent, self.located_view)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StructuralSourceEventLocator {
    event_ordinal: usize,
    original_cursor_before: i64,
    original_cursor_after: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralLocator {
    source_nonce: u64,
    source_fingerprint: u64,
    part_ordinal: usize,
    measure_ordinal: usize,
    source_identity: String,
    window_start: i64,
    source_events: Vec<StructuralSourceEventLocator>,
}

impl StructuralLocator {
    #[cfg(test)]
    pub fn from_orchestrator_nonce(nonce: u64) -> Self {
        Self {
            source_nonce: nonce,
            source_fingerprint: 0,
            part_ordinal: 0,
            measure_ordinal: 0,
            source_identity: String::new(),
            window_start: 0,
            source_events: Vec::new(),
        }
    }

    fn from_snapshot(
        source_nonce: u64,
        source_fingerprint: u64,
        part_ordinal: usize,
        measure_ordinal: usize,
        source_identity: String,
        window_start: i64,
        source_events: Vec<StructuralSourceEventLocator>,
    ) -> Self {
        Self {
            source_nonce,
            source_fingerprint,
            part_ordinal,
            measure_ordinal,
            source_identity,
            window_start,
            source_events,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocatedStructuralView {
    view: StructuralView,
    locator: StructuralLocator,
}

impl LocatedStructuralView {
    pub fn new(view: StructuralView, locator: StructuralLocator) -> Self {
        Self { view, locator }
    }

    /// The detector-facing API deliberately exposes only the identity-free view.
    pub fn detector_view(&self) -> &StructuralView {
        &self.view
    }

    fn into_parts(self) -> (StructuralView, StructuralLocator) {
        (self.view, self.locator)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericityEvidence {
    pub real_pipeline_snapshot_passed: bool,
    pub metamorphic_positive_variants: usize,
    pub metamorphic_negative_variants: usize,
    pub positive_delta_kind_stable: bool,
    pub negatives_rejected: bool,
    pub independent_candidate_duplication_invariant: bool,
    /// Observation only. Scorewide multiplicity is never write authority.
    pub candidate_count: usize,
}

impl GenericityEvidence {
    fn authorizes_write(self) -> bool {
        self.real_pipeline_snapshot_passed
            && self.metamorphic_positive_variants > 0
            && self.metamorphic_negative_variants > 0
            && self.positive_delta_kind_stable
            && self.negatives_rejected
            && self.independent_candidate_duplication_invariant
    }
}

pub(crate) const PHRASE_SLUR_NUMBER_LEVEL_MAXIMUM: usize = 16;
const _: usize = PHRASE_SLUR_NUMBER_LEVEL_MAXIMUM;

mod authority_primitives;
mod beam_plan;
mod authorization_verifier;
mod detector;
mod detector_boundary_audit;
mod final_barline;
mod remaining_layout_writer_family;
pub(crate) use remaining_layout_writer_family::{AllowedRangeRemovalExactMint, NoteVisibilityExactMint};
pub(crate) use final_barline::FinalBarlineAdditiveMint;
pub(crate) use thirty_surface_productive_authority::{
    DirectionInsertionAdditiveMint, HiddenAttributeRemovalExactMint,

};
#[cfg(test)]
pub(crate) use thirty_surface_productive_authority::{apply_harmonic_pitch_swap_authorized, HarmonicPitchSwapExactMint};
mod thirty_surface_productive_authority;
mod scanner_truth_closure;
mod destructive_edit_attribution;
mod rest_voice_family_authority;
mod strict_staff_visible_lane_authority;
mod staff_voice_atomic_authority;
pub(crate) use staff_voice_atomic_authority::{RestificationBatchExactMint, StaffVoiceScalarBatchExactMint};
mod render_measure_articulation_authority;
pub(crate) use render_measure_articulation_authority::MissingRestRenderExactMint;
pub(crate) use render_measure_articulation_authority::{ClefMeasureBlockMint, OctaveMeasureBlockMint};
mod verified_render_candidate_sink_authority;
pub mod clef_surface_materialization;
mod remaining_atomic_patch_authority;
pub(crate) use remaining_atomic_patch_authority::{SafeTransferExactMint, StaffVoiceMoveExactMint};
mod retention_chord_authority;
mod bounded_semantic_rewrite_authority;
pub(crate) use bounded_semantic_rewrite_authority::{EditorialVoiceCompleteRewriteExactMint, VoiceOverfullCropExactMint};
mod final_buffer_surface_authority;
pub(crate) use final_buffer_surface_authority::{
    ChordGestureDistributionExactMint, VisibilityStaffDistributionExactMint,
};
mod orchestrator_artifact_sink_authority;
pub(crate) use orchestrator_artifact_sink_authority::{
    bind_tonal_frame_fact_from_render_snapshot, MasterExportFifthsExactMint,
};
mod rustig_clef_planning;
pub(crate) use rustig_clef_planning::RustigClefCoursePreservingMint;
mod locator_resolution;
mod horizontal_layout_plan_materialization;
mod final_notation_phrase_projection_transaction;
pub(crate) use final_notation_phrase_projection_transaction::FinalLayoutPhrasePreservingMint;
pub(crate) use final_notation_phrase_projection_transaction::FinalLayoutCourseTransport;
mod phrase_notation_policy;
mod materialization_verifier;
mod local_staff_distance_transaction;
pub(crate) use local_staff_distance_transaction::LocalStaffDistancePreservingMint;
mod visual_layout_defaults;
pub(crate) use visual_layout_defaults::VisualLayoutDefaultsPreservingMint;
mod metamorphic_harness;
mod renderer_cross_staff_beam_capability_audit;
mod renderer_direction_relocation_authorization;
mod renderer_direction_relocation_plan;
mod renderer_direction_relocation_post_write_verifier;
mod renderer_direction_relocation_writer_boundary;
pub(crate) use renderer_direction_relocation_writer_boundary::DirectionRelocationExactMint;
#[cfg(test)]
pub(crate) use renderer_direction_relocation_writer_boundary::executed_cursor_then_direction_events_for_render_lineage_test;
mod serialization_plan;
pub(crate) use serialization_plan::{ActualSourceBeamBindingObservation, observe_actual_source_beam_binding};

pub(crate) use serialization_plan::FinalChordEncodingExactMint;
mod snapshot_adapter;
mod writer_entry_boundary;
pub(crate) use writer_entry_boundary::CursorNeutralRenderExactMint;
mod master_adoption;

pub(crate) use master_adoption::{
    adopt_final_layout_phrase_projection,
    apply_block_master_xml_request, MasterAdoptedState, MasterAdoptionCommitBoundary,
    MasterXmlReplacementRequest, RenderAdoptionCommitBoundary, RenderCheckpointSink,
    RenderLateBranch, RenderPathBoundOutcome, RenderSinkKind, RenderSovereignState,
    RenderTrialRoot, DurationPromotion, mutate_selected_duration_fragments,
};

pub use authorization_verifier::IndependentVerifierTokenIssuanceAudit;
pub use detector_boundary_audit::{
    audit_identity_free_detector_module_boundary, IdentityFreeDetectorModuleBoundaryAudit,
};
pub use beam_plan::BeamPlan;
pub use final_barline::ensure_final_barline_authorized;
pub use thirty_surface_productive_authority::{

    
    add_tie_markup_authorized,
    apply_standard_rest_display_pitch_authorized,
    ensure_arrangement_child_text_value_authorized,
    ensure_atomic_move_child_text_value_authorized,
    first_structural_measure_locator,
    fractional_structural_measure_locator,
    insert_direction_into_measure_authorized,
    StructuralMeasureLocator,
    remove_note_hidden_attrs_in_range_authorized,
    remove_note_start_attribute_authorized,

    replace_or_insert_note_tag_authorized,
    restify_atomic_move_note_authorized,
    restify_rhythm_ledger_note_authorized,
    set_pitched_note_duration_type_add_staccato_authorized,
    set_rest_note_duration_type_authorized,
    sync_arrangement_note_type_and_dots_authorized,
    sync_atomic_move_note_type_and_dots_authorized,
    write_missing_rest_trial_xml_authorized,
};
pub use scanner_truth_closure::{
    write_external_notation_feedback_audit_candidate_authorized,
    write_behind_bars_rest_tie_audit_candidate_authorized,
};
pub use rest_voice_family_authority::{
    apply_final_rest_glyph_height_authorized,
    apply_final_visible_rest_role_rewrite_authorized,
    apply_layer_balanced_rest_consolidation_authorized,
    apply_lower_staff_rest_chain_compaction_authorized,
    apply_pianistic_rest_consolidation_authorized,
    apply_rest_only_staff_readability_authorized,
    apply_secondary_reanchored_rest_visibility_authorized,
    apply_single_rest_consolidation_authorized,
    apply_strict_staff_gap_materialization_authorized,
    write_fragmented_rest_chain_trial_authorized,
    write_hidden_rest_forward_collapse_trial_authorized,
    write_lower_staff_visible_rhythm_rest_lane_trial_authorized,
    write_pianistic_rest_repair_trial_authorized,
    write_pruned_pdf_render_trial_authorized,
    write_secondary_reanchored_rest_trial_authorized,
    write_strict_staff_release_trial_authorized,
    write_strict_visible_rest_arithmetic_trial_authorized,
};
pub use strict_staff_visible_lane_authority::{
    write_cross_staff_hand_role_trial_authorized,
    apply_backup_anchor_rewrite_authorized,
    apply_duplicate_same_pitch_suppression_authorized,
    apply_empty_voice_rest_suppression_authorized,
    apply_hidden_rest_visibility_rewrite_authorized,
    apply_lower_visible_rest_suppression_authorized,
    apply_rest_beat_materialization_authorized,
    apply_upper_duplicate_rest_suppression_authorized,
    apply_upper_initial_rest_coalescing_authorized,
    insert_missing_staff_edge_rest_authorized,
};
pub use staff_voice_atomic_authority::{
    apply_atomic_restification_batch_authorized,
    apply_atomic_staff_voice_scalar_batch_authorized,
};
pub use render_measure_articulation_authority::{

    apply_bound_missing_rest_batch_authorized,
    apply_bound_staccato_persistence_batch_authorized,
    StaccatoPersistenceOccurrenceReceipt,
    replace_bound_clef_measure_authorized,
    replace_bound_octave_measure_authorized,
};
pub use verified_render_candidate_sink_authority::{
    write_single_staccato_candidate_authorized,
    write_visual_cleanup_candidate_authorized,
};
pub(crate) use verified_render_candidate_sink_authority::{
    write_renderer_feasibility_shadow_candidate_authorized,
};
pub use clef_surface_materialization::{
    inspect_clef_surface_span,
    project_clef_surface_plan,
    write_clef_surface_materialization_authorized,
};
pub use rustig_clef_planning::{
    append_clef_readability_shadow_course_audit_section,
    apply_rustig_clef_course_to_buffer,
    derive_clef_readability_shadow_course_audit,
    derive_rustig_surface_clef_decision,
    is_rustig_surface_course_candidate,
    RustigClefSurfaceDecision,
};
pub use remaining_atomic_patch_authority::{
    apply_bound_safe_transfer_authorized,
    apply_bound_staff_voice_move_authorized,
};
pub use retention_chord_authority::ensure_bound_chord_marker_authorized;
pub use bounded_semantic_rewrite_authority::{
    apply_editorial_voice_complete_rewrite_authorized,
    apply_voice_overfull_crop_edits_authorized,
    remove_first_bound_accent_authorized,
    replace_bound_condensation_rest_authorized,
};
pub use orchestrator_artifact_sink_authority::{
    
    FunctionalMasterRespellingOperation,
    validate_terminal_render_temporal_postcondition_authorized,
    write_clef_island_decomposition_checkpoint_authorized,
    write_cursor_lane_repair_checkpoint_authorized,
    write_direction_relocation_checkpoint_authorized,
    write_duplicate_rest_rewrite_checkpoint_authorized,
    write_external_feedback_closure_checkpoint_authorized,
    write_final_piano_rest_glyph_checkpoint_authorized,
    write_key_signature_spelling_checkpoint_authorized,
    write_master_musicxml_output_authorized,
    write_result_report_output_authorized,
    write_retention_return_checkpoint_authorized,
    write_sister_feedback_beam_checkpoint_authorized,
    write_visible_rest_role_checkpoint_authorized,

};
pub use final_buffer_surface_authority::{
    apply_chord_gesture_distribution_batch_authorized,
    apply_visibility_staff_distribution_batch_authorized,
};
pub use remaining_layout_writer_family::{
    apply_lower_staff_clef_moves_authorized,
    hide_note_print_object_authorized,
    remove_allowed_xml_ranges_authorized,
    LowerStaffClefMoveRequest,
};
pub(crate) use horizontal_layout_plan_materialization::{
    HorizontalLayoutMaterializationDirective,
    HorizontalSystemBreakMaterialization,
};
pub(crate) use final_notation_phrase_projection_transaction::
    materialize_final_render_notation_with_phrase_projection_authorized;
pub use local_staff_distance_transaction::ensure_local_staff_distance_authorized;
pub use visual_layout_defaults::ensure_visual_layout_defaults_authorized;
pub use materialization_verifier::audit_authorized_materialization_equivalence;
pub use metamorphic_harness::{
    audit_identity_free_metamorphic_safety_net, IdentityFreeMetamorphicSafetyNetAudit,
};
pub use renderer_cross_staff_beam_capability_audit::{
    audit_renderer_cross_staff_beam_capability, RendererCrossStaffBeamCapabilityAudit,
};
pub use serialization_plan::{
    audit_identity_free_semantic_event_equivalence, IdentityFreeSemanticEventEquivalenceAudit,
};
pub(crate) use serialization_plan::{
    attach_final_chord_render_occurrence_witnesses,
    audit_musicxml_voice_label_binding_alpha,
    audit_musicxml_voice_label_binding_alpha_for_render_projection,
    bind_canonical_final_chord_encoding_plan,
    bind_musicxml_voice_labels, build_frozen_serialization_voice_channels_for_canonical,
    build_frozen_serialization_voice_channels_with_original_beams,
    locate_final_chord_render_occurrence_by_exact_range,
    plan_canonical_final_chord_structure_once, plan_final_chord_encoding,
    project_final_chord_encoding_plan,
    project_final_chord_encoding_plan_with_provenance,
    project_canonical_final_chord_render_occurrence_continuation_with_receipts,
    advance_canonical_final_chord_render_occurrence_after_final_notation_checkpoint,
    advance_final_chord_render_occurrence_through_staccato_persistence,
    advance_final_pdf_source_occurrence_after_cursor_commit,
    transport_final_pdf_source_occurrence_continuation_through_final_notation_bundle,
    transport_final_pdf_source_occurrence_continuation_through_horizontal_layout,
    transport_final_pdf_source_occurrence_continuation_through_committed_cross_staff_beam_projection,
    bind_final_pdf_source_occurrence_continuation_to_exact_persisted_readback,
    CanonicalFinalChordRenderOccurrenceContinuation, CanonicalFinalChordStructuralPlan,
    RestBridgeExtensionOccurrenceEvidence,
    FinalPdfSourceOccurrenceContinuation, FinalPdfSourceOccurrenceValidationContext,
    FinalChordRenderOccurrenceWitness, OrphanChordHostContinuationEvidence,
    FinalChordForwardMaterializationKind,
    FrozenSerializationVoiceChannelPlan, FrozenSerializedVoicePlan,
    FrozenVoiceAttackEvidenceRow, LegacyVoiceLabelPreferenceRow,
    SerializedVoicePresentationRole,
    RenderPrunedFinalChordOmissionKey, VoiceFreeFinalChordEvidenceRow,
    PrimaryBeamVoicePartitionRepairCandidate,
    primary_beam_voice_partition_renderer_cohorts,
    PrimaryBeamVoicePartitionBindingContext,
    PrimaryBeamVoicePartitionBoundPlan,
    bind_primary_beam_voice_partition_plan,
    bind_historical_metric_phrase_decision_course,
    rebind_primary_beam_voice_partition_plan_to_current_xml,
    transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition,
};
#[cfg(test)]
pub(crate) use serialization_plan::frozen_serialized_voice_plan_for_rest_role_replay_test;

pub use snapshot_adapter::{
    adapt_structural_snapshot_read_only, detect_structural_repair_intents_read_only,
};
pub use writer_entry_boundary::{
    append_authorized_writer_entry_boundary_audit_section, AuthorizedWriterEntryBoundaryAudit,
};

pub(crate) enum CursorNeutralPreparedBasis { Source, HistoricalCheckpoint58 }
pub(crate) struct PreparedCursorNeutralWriterCandidate {
    pub(crate) candidate_xml: String,
    pub(crate) verifier_audit: IndependentVerifierTokenIssuanceAudit,
    pub(crate) writer_audit: AuthorizedWriterEntryBoundaryAudit,
    pub(crate) basis: CursorNeutralPreparedBasis,
    pub(crate) receipts: crate::master_state_identity::RenderTransitionReceipts,
    pub(crate) verified_direction: Option<orchestrator_artifact_sink_authority::VerifiedDirectionOnlyCandidateSnapshot>,
    pub(crate) accepted_iterations: usize,
    pub(crate) accepted_direction_iterations: usize,
}
#[derive(Debug)]
pub(crate) struct CursorNeutralPreparationError {
    pub(crate) message: &'static str,
    pub(crate) accepted_iterations: usize,
    pub(crate) accepted_direction_iterations: usize,
}
impl std::fmt::Display for CursorNeutralPreparationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{} (accepted iterations {}; accepted direction iterations {})", self.message, self.accepted_iterations, self.accepted_direction_iterations) }
}
impl std::error::Error for CursorNeutralPreparationError {}
// A rejected attempt has no accepted head and cannot replace the winner's witness.
fn advance_verified_direction_for_accepted_head(
    witness: &mut Option<orchestrator_artifact_sink_authority::VerifiedDirectionOnlyCandidateSnapshot>,
    historical: Option<&str>, accepted_head: Option<&str>, direction_proven: bool,
) {
    let Some(head) = accepted_head else { return; };
    if direction_proven {
        *witness = historical.and_then(|base|
            orchestrator_artifact_sink_authority::verify_direction_only_candidate(base, head));
    } else if !historical.zip(witness.as_ref()).is_some_and(|(base, bound)| bound.matches(base, head)) {
        *witness = None;
    }
}

struct CursorIterationHead<'a> {
    chain: &'a mut crate::master_state_identity::RenderTransitionReceipts,
    head: &'a mut Option<String>,
    basis: &'a mut CursorNeutralPreparedBasis,
}

fn retain_accepted_cursor_iteration(
    state: CursorIterationHead<'_>,
    iteration_source: &str, cursor_post: &str, actual_final: &str,
    historical: Option<&str>, cursor: Option<crate::master_state_identity::ExactTransformationReceipt>,
    direction: Option<crate::master_state_identity::ExactTransformationReceipt>,
) -> Result<(), &'static str> {
    let CursorIterationHead { chain, head, basis } = state;
    let has_direction = direction.is_some();
    let cursor_binds = cursor.as_ref().map(|r| r.minted_by::<CursorNeutralRenderExactMint>() && r.realize_post_document(iteration_source).as_deref() == Some(cursor_post)).unwrap_or(iteration_source == cursor_post);
    let direction_binds = direction.as_ref().map(|r| r.minted_by::<DirectionRelocationExactMint>() && historical.is_some_and(|h| r.realize_post_document(h).as_deref() == Some(actual_final))).unwrap_or(cursor_post == actual_final);
    if has_direction { chain.push_instruction_source_exact(cursor, iteration_source, cursor_post); chain.push_exact(direction); }
    else { chain.push_exact(cursor); }
    if !cursor_binds || !direction_binds { return Err("B4 accepted local owner receipt does not bind its actual bytes"); }
    let pre = if has_direction { historical.ok_or("B4 DirectionRelocation without HistoricalCheckpoint58")? } else { iteration_source };
    if let Some(previous) = head.as_deref() {
        if previous != pre { return Err("B4 accepted local event does not continue the prepared document head"); }
    } else { *basis = if has_direction { CursorNeutralPreparedBasis::HistoricalCheckpoint58 } else { CursorNeutralPreparedBasis::Source }; }
    *head = Some(actual_final.to_string());
    Ok(())
}

pub(crate) struct CursorNeutralExecutionStages<'a> {
    pub(crate) verifier_source_stage: &'a str,
    pub(crate) writer_source_stage: &'a str,
    pub(crate) historical_checkpoint_58: Option<&'a str>,
}

pub(crate) fn execute_authorized_cursor_neutral_writer_candidate(
    source_xml: &str,
    repair_intents: &[LocatedStructuralRepairIntent],
    detector_boundary_audit: &IdentityFreeDetectorModuleBoundaryAudit,
    semantic_event_audit: &IdentityFreeSemanticEventEquivalenceAudit,
    metamorphic_audit: &IdentityFreeMetamorphicSafetyNetAudit,
    stages: CursorNeutralExecutionStages<'_>,
) -> Result<PreparedCursorNeutralWriterCandidate, CursorNeutralPreparationError> {
    let CursorNeutralExecutionStages { verifier_source_stage, writer_source_stage, historical_checkpoint_58 } = stages;
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    let mut head: Option<String> = None;
    let mut basis = CursorNeutralPreparedBasis::Source;
    let mut verified_direction = None;
    let mut accepted_iterations = 0;
    let mut accepted_direction_iterations = 0;
    let batch_mode = repair_intents.len() > 1;
    let mut current_xml = source_xml.to_string();
    let mut last_verifier_audit =
        authorization_verifier::IndependentVerifierTokenIssuanceAudit::default();
    let mut last_writer_audit =
        writer_entry_boundary::blocked_authorized_writer_entry(writer_source_stage);
    // Preserve the most conservative renderer-contiguity outcome across the
    // whole independent-candidate batch. A later green local candidate must
    // never erase an earlier PDF hold.
    let mut held_writer_audit: Option<
        writer_entry_boundary::AuthorizedWriterEntryBoundaryAudit,
    > = None;

    loop {
        let candidate_rows = if batch_mode {
            // Scorewide multiplicity is observation only. Re-read the current
            // XML after every committed local write so no stale locator can
            // authorize another independent candidate.
            let (current_views, _) = adapt_structural_snapshot_read_only(
                &current_xml,
                1,
                "multiplicity-local-rebind",
            );
            let current_intents =
                detect_structural_repair_intents_read_only(&current_views);
            if current_intents.is_empty() {
                break;
            }
            let current_metamorphic_audit = audit_identity_free_metamorphic_safety_net(
                &current_xml,
                1,
                "multiplicity-local-rebind",
            );

            current_intents
                .iter()
                .filter(|located_intent| {
                    let locator = &located_intent.located_view.locator;
                    current_intents
                        .iter()
                        .filter(|other| {
                            let other_locator = &other.located_view.locator;
                            locator.source_events.iter().any(|left| {
                                other_locator.source_events.iter().any(|right| {
                                    left.event_ordinal == right.event_ordinal
                                })
                            })
                        })
                        .count()
                        == 1
                })
                .map(|located_intent| {
                    let local_intents = vec![located_intent.clone()];
                    let local_detector_boundary =
                        audit_identity_free_detector_module_boundary(
                            current_views.len(),
                            1,
                            "multiplicity-local-rebind",
                        );
                    let local_semantic_event_audit =
                        audit_identity_free_semantic_event_equivalence(
                            &local_intents,
                            current_views.len(),
                            "multiplicity-local-rebind",
                        );
                    (
                        local_intents,
                        local_detector_boundary,
                        local_semantic_event_audit,
                        current_metamorphic_audit.clone(),
                    )
                })
                .collect::<Vec<_>>()
        } else {
            vec![(
                repair_intents.to_vec(),
                (*detector_boundary_audit).clone(),
                (*semantic_event_audit).clone(),
                (*metamorphic_audit).clone(),
            )]
        };

        if candidate_rows.is_empty() {
            break;
        }

        let mut progress = false;
        for (
            local_intents,
            local_detector_boundary,
            local_semantic_event_audit,
            local_metamorphic_audit,
        ) in candidate_rows
        {
            let (mut verifier_audit, token) =
                authorization_verifier::authorize_repair_candidate_for_writer(
                    &local_intents,
                    &local_detector_boundary,
                    &local_semantic_event_audit,
                    &local_metamorphic_audit,
                    verifier_source_stage,
                );
            let Some(token) = token else {
                last_verifier_audit = verifier_audit;
                if batch_mode {
                    continue;
                }

                let mut writer_audit =
                    writer_entry_boundary::blocked_authorized_writer_entry(writer_source_stage);
                let (relocation_authorization, _token, contract) =
                    renderer_direction_relocation_authorization::authorize_renderer_direction_relocation_for_writer(
                        None,
                        &current_xml,
                        &current_xml,
                        false,
                        "kein autorisierter Hauptwriter-Kandidat fuer Direction-Autorisierung",
                    );
                let relocation_writer_audit =
                    renderer_direction_relocation_writer_boundary::RendererDirectionRelocationWriterAudit::default();
                let relocation_post_write =
                    renderer_direction_relocation_post_write_verifier::audit_renderer_direction_relocation_post_write(
                        &current_xml,
                        &current_xml,
                        contract.as_ref(),
                        &relocation_writer_audit,
                        "kein Direction-Write ohne autorisierten Hauptwriter-Kandidaten",
                    );
                writer_audit.bind_renderer_direction_relocation_authorization(
                    relocation_authorization,
                );
                writer_audit.bind_renderer_direction_relocation_post_write(
                    relocation_post_write,
                );
                return Ok(PreparedCursorNeutralWriterCandidate { candidate_xml: current_xml, verifier_audit: last_verifier_audit, writer_audit, receipts, basis, verified_direction, accepted_iterations, accepted_direction_iterations });
            };

            let (
                pre_relocation_candidate_xml,
                mut writer_audit,
                proven_direction_relocation_plan,
                cursor_neutral_receipt,
            ) = writer_entry_boundary::authorized_reference_writer_entry(
                token,
                &current_xml,
                writer_source_stage,
            );
            let (
                mut relocation_authorization,
                relocation_token,
                verification_contract,
            ) = renderer_direction_relocation_authorization::authorize_renderer_direction_relocation_for_writer(
                proven_direction_relocation_plan,
                &current_xml,
                &pre_relocation_candidate_xml,
                writer_audit.write_produced(),
                "exakter cursorneutraler Renderkandidat vor Direction-Writer-Handoff",
            );

            let (
                final_candidate_xml,
                relocation_shadow_candidate_xml,
                relocation_writer_audit,
                direction_relocation_receipt,
            ) = if let (Some(relocation_token), Some(historical)) = (relocation_token, historical_checkpoint_58) {
                renderer_direction_relocation_writer_boundary::authorized_renderer_direction_relocation_writer_entry(
                    relocation_token,
                    &pre_relocation_candidate_xml,
                    historical,
                    "tokenpflichtiger renderer-neutraler Direction-Writer Phase 6F mit threadlokal gebundenem typisiertem Render-Checkpoint-Sink",
                )
            } else {
                (
                    pre_relocation_candidate_xml.clone(),
                    pre_relocation_candidate_xml.clone(),
                    renderer_direction_relocation_writer_boundary::RendererDirectionRelocationWriterAudit::default(),
                    None,
                )
            };
            relocation_authorization.bind_writer_outcome(
                relocation_writer_audit.token_handed_to_writer,
                relocation_writer_audit.token_consumed,
                relocation_writer_audit.writer_token_used,
                relocation_writer_audit.write_produced,
            );
            let relocation_post_write =
                renderer_direction_relocation_post_write_verifier::audit_renderer_direction_relocation_post_write(
                    &pre_relocation_candidate_xml,
                    &relocation_shadow_candidate_xml,
                    verification_contract.as_ref(),
                    &relocation_writer_audit,
                    "tatsaechlich geschriebener Direction-Relocation-Kandidat gegen exakten Vor-Writer-Snapshot",
                );
            let accepted_direction_proven = relocation_writer_audit.write_produced
                && relocation_post_write.green();
            writer_audit.bind_renderer_direction_relocation_authorization(
                relocation_authorization,
            );
            writer_audit.bind_renderer_direction_relocation_post_write(
                relocation_post_write,
            );
            verifier_audit.bind_writer_outcome(
                writer_audit.token_consumed(),
                writer_audit.write_produced(),
            );

            if writer_audit.write_produced() {
                accepted_iterations += 1;
                accepted_direction_iterations += usize::from(direction_relocation_receipt.is_some());
                if let Err(message) = retain_accepted_cursor_iteration(
                    CursorIterationHead { chain: &mut receipts, head: &mut head, basis: &mut basis },
&current_xml,
&pre_relocation_candidate_xml,
&final_candidate_xml,
historical_checkpoint_58,
cursor_neutral_receipt,
direction_relocation_receipt) {
                    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
                    crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures);
                    return Err(CursorNeutralPreparationError { message, accepted_iterations, accepted_direction_iterations });
                }
                advance_verified_direction_for_accepted_head(
                    &mut verified_direction, historical_checkpoint_58,
                    Some(&final_candidate_xml), accepted_direction_proven,
                );
            } else {
                let mut rejected = crate::master_state_identity::RenderTransitionReceipts::new();
                rejected.push_exact(cursor_neutral_receipt);
                rejected.push_exact(direction_relocation_receipt);
                rejected.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
            }
            if !batch_mode {
                return Ok(PreparedCursorNeutralWriterCandidate { candidate_xml: final_candidate_xml, verifier_audit, writer_audit, receipts, basis, verified_direction, accepted_iterations, accepted_direction_iterations });
            }

            let wrote = writer_audit.write_produced();
            let renderer_contiguity_green = writer_audit.renderer_contiguity_green();
            last_verifier_audit = verifier_audit;
            if wrote && !renderer_contiguity_green && held_writer_audit.is_none() {
                held_writer_audit = Some(writer_audit);
            } else {
                last_writer_audit = writer_audit;
            }
            if wrote {
                current_xml = final_candidate_xml;
                progress = true;
                break;
            }
        }

        if !batch_mode || !progress {
            break;
        }
    }

    let batch_writer_audit = held_writer_audit.unwrap_or(last_writer_audit);
    Ok(PreparedCursorNeutralWriterCandidate { candidate_xml: current_xml, verifier_audit: last_verifier_audit, writer_audit: batch_writer_audit, receipts, basis, verified_direction, accepted_iterations, accepted_direction_iterations })
}

// Private owner capability values for the accepted r1226 closed fixtures.
pub(crate) struct VisibleLane41RenderExactMint { _private: () }
const VISIBLE_LANE_41_RENDER_EXACT_MINT: VisibleLane41RenderExactMint = VisibleLane41RenderExactMint { _private: () };
use VISIBLE_LANE_41_RENDER_EXACT_MINT as VisibleLane41RenderExactMint;
pub(crate) struct VisibleLane42RenderExactMint { _private: () }
const VISIBLE_LANE_42_RENDER_EXACT_MINT: VisibleLane42RenderExactMint = VisibleLane42RenderExactMint { _private: () };
use VISIBLE_LANE_42_RENDER_EXACT_MINT as VisibleLane42RenderExactMint;
pub(crate) struct VisibleLane43RenderExactMint { _private: () }
const VISIBLE_LANE_43_RENDER_EXACT_MINT: VisibleLane43RenderExactMint = VisibleLane43RenderExactMint { _private: () };
use VISIBLE_LANE_43_RENDER_EXACT_MINT as VisibleLane43RenderExactMint;
pub(crate) struct VisibleLane44RenderExactMint { _private: () }
const VISIBLE_LANE_44_RENDER_EXACT_MINT: VisibleLane44RenderExactMint = VisibleLane44RenderExactMint { _private: () };
use VISIBLE_LANE_44_RENDER_EXACT_MINT as VisibleLane44RenderExactMint;
pub(crate) struct VisibleLane45RenderExactMint { _private: () }
const VISIBLE_LANE_45_RENDER_EXACT_MINT: VisibleLane45RenderExactMint = VisibleLane45RenderExactMint { _private: () };
use VISIBLE_LANE_45_RENDER_EXACT_MINT as VisibleLane45RenderExactMint;
pub(crate) struct VisibleLane46RenderExactMint { _private: () }
const VISIBLE_LANE_46_RENDER_EXACT_MINT: VisibleLane46RenderExactMint = VisibleLane46RenderExactMint { _private: () };
use VISIBLE_LANE_46_RENDER_EXACT_MINT as VisibleLane46RenderExactMint;
pub(crate) struct VisibleLane47RenderExactMint { _private: () }
const VISIBLE_LANE_47_RENDER_EXACT_MINT: VisibleLane47RenderExactMint = VisibleLane47RenderExactMint { _private: () };
use VISIBLE_LANE_47_RENDER_EXACT_MINT as VisibleLane47RenderExactMint;
pub(crate) struct VisibleLane48RenderExactMint { _private: () }
const VISIBLE_LANE_48_RENDER_EXACT_MINT: VisibleLane48RenderExactMint = VisibleLane48RenderExactMint { _private: () };
use VISIBLE_LANE_48_RENDER_EXACT_MINT as VisibleLane48RenderExactMint;






pub(crate) struct TieFragmentAdditiveMint { _private: () }
const TIE_FRAGMENT_ADDITIVE_MINT: TieFragmentAdditiveMint = TieFragmentAdditiveMint { _private: () };
use TIE_FRAGMENT_ADDITIVE_MINT as TieFragmentAdditiveMint;
pub(crate) use retention_chord_authority::RenderRow7BlockMint;
pub(crate) use thirty_surface_productive_authority::RenderRow17BlockMint;

pub(crate) use rest_voice_family_authority::RenderRow30ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow31ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow32ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow33ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow34ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow35ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow36ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow37ExactMint;
pub(crate) use rest_voice_family_authority::RenderRow38ExactMint;
pub(crate) use strict_staff_visible_lane_authority::RenderRow51ExactMint;
pub(crate) use orchestrator_artifact_sink_authority::RenderRow58ExactMint;
pub(crate) use clef_surface_materialization::RenderRow61BlockMint;

mod fragment_receipt_geometry;
pub(crate) use fragment_receipt_geometry::bound_tie_fragment_insertions;

#[cfg(test)]
mod tests {
    use super::*;

    const SYNTHETIC_SNAPSHOT: &str = include_str!(
        "../../tests/fixtures/synthetic/cross_staff_tail_structural_invariant.xml"
    );


    fn execute_authorized_cursor_neutral_writer_candidate(
        source_xml: &str, intents: &[LocatedStructuralRepairIntent], detector: &IdentityFreeDetectorModuleBoundaryAudit,
        semantic: &IdentityFreeSemanticEventEquivalenceAudit, metamorphic: &IdentityFreeMetamorphicSafetyNetAudit,
        verifier_stage: &str, writer_stage: &str,
    ) -> (String, IndependentVerifierTokenIssuanceAudit, AuthorizedWriterEntryBoundaryAudit) {
        let prepared = super::execute_authorized_cursor_neutral_writer_candidate(source_xml,
intents,
detector,
semantic,
metamorphic,
crate::structural_write_authority::CursorNeutralExecutionStages { verifier_source_stage: verifier_stage, writer_source_stage: writer_stage, historical_checkpoint_58: None }).expect("cursor fixture");
        prepared.receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        (prepared.candidate_xml, prepared.verifier_audit, prepared.writer_audit)
    }
    fn audit_authorized_materialization_equivalence(source: &str, candidate: &str, stage: &str) -> materialization_verifier::AuthorizedMaterializationEquivalenceAudit {
        super::audit_authorized_materialization_equivalence(source, candidate, stage, None, None)
    }

    pub(super) fn bound_cursor_source() -> String {
        bound_cursor_source_at(11)
    }

    pub(super) fn bound_cursor_source_at(ordinal: u32) -> String {
        let measure = SYNTHETIC_SNAPSHOT.replacen(
            "number=\"synthetic\"", &format!("number=\"{ordinal}\""), 1,
        );
        let measure = measure.replacen('>',
            "><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>", 1);
        format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>fixture</part-name></score-part></part-list><part id=\"P1\">{measure}</part></score-partwise>")
    }

    #[test]
    fn cursor_event_receipt_ends_before_direction_composition() {
        let (pre, post, receipt) = writer_entry_boundary::tests::executed_cursor_event();
        assert!(receipt.minted_by::<CursorNeutralRenderExactMint>());
        assert!(!receipt.minted_by::<DirectionRelocationExactMint>());
        assert_eq!(receipt.realize_post_document(&pre).as_deref(), Some(post.as_str()));
        assert!(!post.contains("<offset>48</offset>"));
        let (views, _) = adapt_structural_snapshot_read_only(&pre, 96, "boundary-fixture");
        let intents = detect_structural_repair_intents_read_only(&views);
        let detector = audit_identity_free_detector_module_boundary(views.len(), intents.len(), "boundary-fixture");
        let semantic = audit_identity_free_semantic_event_equivalence(&intents, views.len(), "boundary-fixture");
        let metamorphic = audit_identity_free_metamorphic_safety_net(&pre, 96, "boundary-fixture");
        let (legacy, _, _) = execute_authorized_cursor_neutral_writer_candidate(
            &pre, &intents, &detector, &semantic, &metamorphic, "boundary-fixture", "boundary-fixture");
        assert_eq!(legacy, post);
        assert!(receipt.realize_post_document(&format!("{pre} ")).is_none());
        let (no_views, _) = adapt_structural_snapshot_read_only(&post, 96, "boundary-noop");
        assert!(detect_structural_repair_intents_read_only(&no_views).is_empty());
    }

    fn identity_free_view() -> StructuralView {
        StructuralView::new(
            96,
            vec![
                StructuralEvent {
                    onset_in_window: 0,
                    duration: 24,
                    primary_beam: StructuralBeamState::Begin,
                    secondary_beam: StructuralBeamState::Absent,
                    staff_relation: StructuralStaffRelation::SameAsAnchor,
                    voice_relation: StructuralVoiceRelation::SameRun,
                    carrier_role: StructuralCarrierRole::Sounding,
                    visible: true,
                    rest_like: false,
                    chord_member: false,
                },
                StructuralEvent {
                    onset_in_window: 24,
                    duration: 24,
                    primary_beam: StructuralBeamState::End,
                    secondary_beam: StructuralBeamState::Absent,
                    staff_relation: StructuralStaffRelation::DifferentFromAnchor,
                    voice_relation: StructuralVoiceRelation::SameRun,
                    carrier_role: StructuralCarrierRole::Sounding,
                    visible: true,
                    rest_like: false,
                    chord_member: false,
                },
            ],
        )
    }

    #[test]
    fn detector_view_is_independent_of_source_locator_identity() {
        let first = LocatedStructuralView::new(
            identity_free_view(),
            StructuralLocator::from_orchestrator_nonce(33),
        );
        let second = LocatedStructuralView::new(
            identity_free_view(),
            StructuralLocator::from_orchestrator_nonce(7),
        );

        assert_eq!(
            first.detector_view().fingerprint(),
            second.detector_view().fingerprint()
        );
        assert_eq!(first.detector_view().events().len(), 2);
        assert_eq!(first.detector_view().metric_units_per_quarter(), 96);
    }

    #[test]
    fn wrong_note_beam_sink_is_blocked_after_token_consumption() {
        // The production writer always receives a full MusicXML document with
        // inherited divisions/time context. The synthetic isolated-measure
        // fixture does not contain that context, so bind it explicitly for the
        // raw-cursor proof without changing any musical event.
        let cursor_bound_snapshot = bound_cursor_source();
        let (views, _adapter) =
            adapt_structural_snapshot_read_only(&cursor_bound_snapshot, 96, "writer-fixture");
        let intents = detect_structural_repair_intents_read_only(&views);
        let detector_boundary = audit_identity_free_detector_module_boundary(
            views.len(),
            intents.len(),
            "writer-fixture",
        );
        let semantic =
            audit_identity_free_semantic_event_equivalence(&intents, views.len(), "writer-fixture");
        let metamorphic = audit_identity_free_metamorphic_safety_net(
            &cursor_bound_snapshot,
            96,
            "writer-fixture",
        );

        // The historical sink has a different note/staff relation, so the
        // authorized Direction target cannot compose onto that snapshot.
        let wrong_note_sink = cursor_bound_snapshot.replace("<staff>1</staff>", "<staff>2</staff>");
        let prepared = super::execute_authorized_cursor_neutral_writer_candidate(&cursor_bound_snapshot,
&intents,
&detector_boundary,
&semantic,
&metamorphic,
crate::structural_write_authority::CursorNeutralExecutionStages { verifier_source_stage: "writer-fixture", writer_source_stage: "writer-fixture", historical_checkpoint_58: Some(&wrong_note_sink) }).expect("wrong-note sink fixture");
        prepared.receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        let (candidate, verifier, writer) =
            (prepared.candidate_xml, prepared.verifier_audit, prepared.writer_audit);

        assert_ne!(candidate, cursor_bound_snapshot);
        assert!(roxmltree::Document::parse(&candidate).is_ok());
        assert!(verifier.token_origin_proof_green());
        assert!(verifier
            .pipeline_line("fixture")
            .contains("TokenHandedToWriter ja; TokenConsumed ja"));
        assert!(writer.write_produced());
        assert!(writer.requires_pdf_hold());
        assert!(!writer.renderer_contiguity_green());
        let equivalence = audit_authorized_materialization_equivalence(
            &cursor_bound_snapshot,
            &candidate,
            "writer-fixture",
        );
        assert!(equivalence.green());
        let writer_pipeline = writer.pipeline_line("fixture");
        assert!(writer_pipeline.contains("CandidateRawCursorPrecheckGreen ja"));
        assert!(writer_pipeline.contains("RendererBeamGroupsChecked 2"));
        assert!(writer_pipeline.contains("RendererBeamMemberPairsChecked 3"));
        assert!(writer_pipeline.contains("RendererInterstitialElements 0"));
        assert!(writer_pipeline.contains("RendererInterstitialDirections 0"));
        assert!(writer_pipeline.contains("RendererFragmentParseFailures 0"));
        assert!(writer_pipeline.contains("RendererContiguousTargetMembers nein"));
        let direction_plan = writer.renderer_direction_relocation_plan();
        assert!(direction_plan.feasible());
        let direction_pipeline = direction_plan.pipeline_line("fixture");
        assert!(direction_pipeline.contains("SourceEffectiveCursor 336"));
        assert!(direction_pipeline.contains("ProposedHostCursor 288"));
        assert!(direction_pipeline.contains("ProposedOffset 48"));
        assert!(direction_pipeline.contains("InstantaneousDynamicsOnly ja"));
        assert!(direction_pipeline.contains("RelocationPlanFeasible ja"));
        let direction_authorization = writer.renderer_direction_relocation_authorization();
        assert!(!direction_authorization.green());
        let authorization_pipeline = direction_authorization.pipeline_line("fixture");
        assert!(authorization_pipeline.contains("AuthorizationGranted 1"));
        assert!(authorization_pipeline.contains("TokensIssued 1"));
        assert!(authorization_pipeline.contains("WriterInputAuthorizedTokenOnly ja"));
        assert!(authorization_pipeline.contains("AuthorityWriterCallSites 1"));
        assert!(authorization_pipeline.contains("TokenHandedToWriter ja"));
        assert!(authorization_pipeline.contains("TokenConsumed ja"));
        assert!(authorization_pipeline.contains("TokensDroppedWithoutWriter 0"));
        assert!(authorization_pipeline.contains("WriteProduced nein"));
        let post_write = writer.renderer_direction_relocation_post_write();
        assert!(!post_write.green());
        let post_write_pipeline = post_write.pipeline_line("fixture");
        assert!(post_write_pipeline.contains("ExpectedPostSnapshotFingerprintMatch nein"));
        assert!(post_write_pipeline.contains("UniqueRelocatedDirection nein"));
        assert!(!candidate.contains("<offset>48</offset>"));
        assert!(!candidate.contains("<backup><duration>24</duration></backup>"));

        let measure_with_context = |ordinal| {
            let source = bound_cursor_source_at(ordinal);
            let start = source.find("<measure").expect("synthetic measure start");
            let end = source.find("</measure>").expect("synthetic measure end") + "</measure>".len();
            source[start..end].to_string()
        };
        let first_measure = measure_with_context(11);
        let second_measure = measure_with_context(12);
        let duplicated_snapshot = format!(
            "<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>fixture</part-name></score-part></part-list><part id=\"P1\">{first_measure}{second_measure}</part></score-partwise>"
        );
        let (duplicated_views, _) = adapt_structural_snapshot_read_only(
            &duplicated_snapshot,
            96,
            "duplication-writer-fixture",
        );
        let duplicated_intents =
            detect_structural_repair_intents_read_only(&duplicated_views);
        assert_eq!(duplicated_intents.len(), 2);
        let duplicated_detector_boundary = audit_identity_free_detector_module_boundary(
            duplicated_views.len(),
            duplicated_intents.len(),
            "duplication-writer-fixture",
        );
        let duplicated_semantic = audit_identity_free_semantic_event_equivalence(
            &duplicated_intents,
            duplicated_views.len(),
            "duplication-writer-fixture",
        );
        let duplicated_metamorphic = audit_identity_free_metamorphic_safety_net(
            &duplicated_snapshot,
            96,
            "duplication-writer-fixture",
        );
        assert_eq!(
            duplicated_metamorphic.authorization_evidence().candidate_count,
            2
        );
        assert!(
            duplicated_metamorphic
                .authorization_evidence()
                .independent_candidate_duplication_invariant
        );

        let (duplicated_candidate, duplicated_verifier, duplicated_writer) =
            execute_authorized_cursor_neutral_writer_candidate(
                &duplicated_snapshot,
                &duplicated_intents,
                &duplicated_detector_boundary,
                &duplicated_semantic,
                &duplicated_metamorphic,
                "duplication-writer-fixture",
                "duplication-writer-fixture",
            );
        assert_ne!(duplicated_candidate, duplicated_snapshot);
        assert!(duplicated_verifier.token_origin_proof_green());
        assert!(duplicated_writer.write_produced());
        let (remaining_views, _) = adapt_structural_snapshot_read_only(
            &duplicated_candidate,
            96,
            "duplication-writer-fixture-post",
        );
        let remaining_intents =
            detect_structural_repair_intents_read_only(&remaining_views);
        assert_eq!(remaining_intents.len(), 0);
        assert!(
            audit_authorized_materialization_equivalence(
                &duplicated_snapshot,
                &duplicated_candidate,
                "duplication-writer-fixture",
            )
            .green()
        );
    }

    #[test]
    fn renderer_direction_relocation_plan_rejects_timing_spanner() {
        let unsafe_snapshot = SYNTHETIC_SNAPSHOT.replace("<mf/>", "<wedge type=\"crescendo\"/>");
        let (views, _adapter) =
            adapt_structural_snapshot_read_only(&unsafe_snapshot, 96, "writer-fixture");
        let intents = detect_structural_repair_intents_read_only(&views);
        let detector_boundary = audit_identity_free_detector_module_boundary(
            views.len(),
            intents.len(),
            "writer-fixture",
        );
        let semantic =
            audit_identity_free_semantic_event_equivalence(&intents, views.len(), "writer-fixture");
        let metamorphic =
            audit_identity_free_metamorphic_safety_net(&unsafe_snapshot, 96, "writer-fixture");

        let (_candidate, _verifier, writer) = execute_authorized_cursor_neutral_writer_candidate(
            &unsafe_snapshot,
            &intents,
            &detector_boundary,
            &semantic,
            &metamorphic,
            "writer-fixture",
            "writer-fixture",
        );

        let direction_plan = writer.renderer_direction_relocation_plan();
        assert!(!direction_plan.feasible());
        let direction_pipeline = direction_plan.pipeline_line("fixture");
        assert!(direction_pipeline.contains("TimingSpannerAbsent nein"));
        assert!(direction_pipeline.contains("InstantaneousDynamicsOnly nein"));
        assert!(direction_pipeline.contains("RelocationPlanFeasible nein"));
        let direction_authorization = writer.renderer_direction_relocation_authorization();
        assert!(!direction_authorization.green());
        assert!(direction_authorization
            .pipeline_line("fixture")
            .contains("AuthorizationGranted 0"));
    }

    #[test]
    fn renderer_direction_relocation_plan_rejects_default_x_override() {
        let unsafe_snapshot = SYNTHETIC_SNAPSHOT.replace("<mf/>", "<mf default-x=\"12\"/>");
        let (views, _adapter) =
            adapt_structural_snapshot_read_only(&unsafe_snapshot, 96, "writer-fixture");
        let intents = detect_structural_repair_intents_read_only(&views);
        let detector_boundary = audit_identity_free_detector_module_boundary(
            views.len(),
            intents.len(),
            "writer-fixture",
        );
        let semantic =
            audit_identity_free_semantic_event_equivalence(&intents, views.len(), "writer-fixture");
        let metamorphic =
            audit_identity_free_metamorphic_safety_net(&unsafe_snapshot, 96, "writer-fixture");

        let (_candidate, _verifier, writer) = execute_authorized_cursor_neutral_writer_candidate(
            &unsafe_snapshot,
            &intents,
            &detector_boundary,
            &semantic,
            &metamorphic,
            "writer-fixture",
            "writer-fixture",
        );

        let direction_plan = writer.renderer_direction_relocation_plan();
        assert!(!direction_plan.feasible());
        let direction_pipeline = direction_plan.pipeline_line("fixture");
        assert!(direction_pipeline.contains("DefaultXAbsent nein"));
        assert!(direction_pipeline.contains("InstantaneousDynamicsOnly nein"));
        assert!(direction_pipeline.contains("RelocationPlanFeasible nein"));
    }
}

#[cfg(test)]
mod raw_splice_receipt_tests {
#[test]
fn raw_splice_helper_and_real_receipt_match_legacy_bytes() {
    // The frozen helper admits only its definition and its guarded production
    // call in src. Compile the actual helper in a temporary test program rather
    // than adding an extra productive source reference to that owner.
    fn item(source: &str, anchor: &str) -> String {
        let start = source.find(anchor).unwrap();
        let open = start + source[start..].find('{').unwrap();
        let mut depth = 1;
        let mut end = open + 1;
        for byte in source[open + 1..].bytes() {
            match byte { b'{' => depth += 1, b'}' => depth -= 1, _ => {} }
            end += 1;
            if depth == 0 { break; }
        }
        source[start..end].to_string()
    }
    let owner = include_str!("../render/silent_voice_visible_measure_coverage_render_writer.rs");
    // This harness executes the actual EXACT core and frozen #67 owner. Render
    // lineage is an adoption layer, and E057's owner-specific PRESERVING binding
    // calls a separate musical projection; neither participates in this event.
    // Omit those unrelated bindings rather than supplying mock implementations.
    let identity_source = include_str!("../master_state_identity.rs");
    let identity_core = identity_source.split_once("\nmod render_lineage;")
        .expect("real receipt core precedes Render adoption module").0;
    let e057_sealed = item(identity_core,
        "impl sealed::MintOwner for crate::pipeline::PartialBeatLaneTransferPreservingMint");
    let e057_preserving = item(identity_core,
        "impl PreservingMintOwner for crate::pipeline::PartialBeatLaneTransferPreservingMint");
    // The new course reader is independent of this raw-splice receipt test.
    // Omit it too; no stand-in custody implementation or receipt is supplied.
    let course_reader = item(identity_core, "pub(crate) fn continue_owned_rhythm(");
    let identity = identity_core.replace(&course_reader, "").replace(&e057_sealed, "")
        .replace(&e057_preserving, "");
    let helper = item(owner, "fn same_lane_strict_interior_raw_splice(");
    let factory = item(owner, "fn mint_same_lane_raw_splice_receipt(");
    let marker = item(owner, "pub(crate) struct SameLaneRawSpliceExactMint");
    let mut modules = std::collections::BTreeMap::<String, Vec<String>>::new();
    for line in identity.lines().filter(|line| line.starts_with("bind_mint_owner!(")) {
        let path = line.split(',').nth(1).unwrap().trim();
        let parts = path.split("::").collect::<Vec<_>>();
        if parts[2] != "SameLaneRawSpliceExactMint" {
            modules.entry(parts[1].to_string()).or_default().push(format!("pub(crate) struct {};", parts[2]));
        }
    }
    modules.entry("structural_write_authority".to_string()).or_default()
        .push("pub(crate) struct MasterAdoptionCommitBoundary;".to_string());
    let fragment_geometry = include_str!("fragment_receipt_geometry.rs");
    modules.entry("structural_write_authority".to_string()).or_default().extend([

        
        "pub(crate) struct TieFragmentAdditiveMint;".to_string(),
        format!("mod fragment_receipt_geometry {{ {fragment_geometry} }} pub(crate) use fragment_receipt_geometry::{{bound_tie_fragment_insertions}};"),
    ]);
    // The receipt's typed alternative owns the real immutable E007 data.
    // Include those actual types/read methods, not a substitute proof or an
    // unrelated productive layout writer, in this original #67 event probe.
    let horizontal_source = include_str!("horizontal_layout_plan_materialization.rs");
    let layout_copies = item(horizontal_source,
        "#[derive(Debug)]\npub(super) struct OriginalHorizontalLayoutCopies");
    let layout_copy_reader = item(horizontal_source, "impl OriginalHorizontalLayoutCopies");
    let phrase_source = include_str!("final_notation_phrase_projection_transaction.rs");
    let phrase_insertion = item(phrase_source,
        "#[derive(Debug, Clone, PartialEq, Eq)]\nstruct PhraseSlurInsertion");
    let layout_transport = item(phrase_source,
        "#[derive(Debug)]\npub(crate) struct FinalLayoutCourseTransport");
    let layout_transport_reader = item(phrase_source, "impl FinalLayoutCourseTransport");
    modules.entry("structural_write_authority".to_string()).or_default().push(format!(
        "mod horizontal_layout_plan_materialization {{ use anyhow::Result; {layout_copies} {layout_copy_reader} }}\n\
         mod final_notation_phrase_projection_transaction {{ use anyhow::Result; {phrase_insertion} {layout_transport} {layout_transport_reader} }}\n\
         pub(crate) use final_notation_phrase_projection_transaction::FinalLayoutCourseTransport;"));
    // The receipt companion owns the real observational capture type. Compile
    // that dependency and its original data/plan types rather than a mock.
    let beam_observation = include_str!("../pipeline_shards/composer_metric_beam_observation_core.rs");
    let beam_data = [item(beam_observation, "struct PartialBeatRawNote {"),
        item(beam_observation, "struct PartialBeatNoteInfo {")].join("\n");
    let beam_capture = include_str!("../pipeline_shards/actual_beam_capture_core.rs")
        .split_once("\n#[cfg(test)]").expect("capture tests follow implementation").0
        .replace("//!", "//");
    let beam_plan = include_str!("beam_plan.rs").replace("//!", "//");
    modules.entry("structural_write_authority".to_string()).or_default().push(format!(
        "mod beam_plan {{ {beam_plan} }} pub(crate) use beam_plan::BeamPlan;"));
    let pipeline = modules.remove("pipeline").unwrap().join("\n");
    // Compile the actual new receipt dependency, including its actual creation
    // identity and physical reader, rather than a stand-in evidence type.
    let arrangement_source = include_str!("../arrangement.rs");
    let raw_note = item(arrangement_source, "struct RawNote {");
    let raw_reader = item(arrangement_source, "fn collect_raw_notes(");
    let physical_births = include_str!("../physical_rhythmic_births.rs");
    // Compile the actual notation journal as well; the receipt container owns
    // this dependency. The excluded course reader still has no mock substitute.
    let representation_transport = include_str!("../rhythmic_representation_transport.rs");
    modules.entry("arrangement".to_string()).or_default().extend([
        "use std::collections::BTreeSet; pub(crate) struct ArrangementRhythmicMint;".to_string(),
        format!("{raw_note}\n{raw_reader}\npub(crate) mod physical_births {{ {physical_births} }}"),
        format!("pub(crate) mod representation_transport {{ {representation_transport} }}"),
    ]);
    let seed_source = include_str!("../midi_seed.rs");
    let identity_start = seed_source.find("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]\npub(crate) struct RhythmicCreationIdentity").unwrap();
    let identity_end = identity_start + seed_source[identity_start..].find("\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub(crate) enum SeedCoursePresentation").unwrap();
    modules.entry("midi_seed".to_string()).or_default().push(format!(
        "use anyhow::anyhow; {}", &seed_source[identity_start..identity_end]));
    let mut program = "#![allow(dead_code)]\n".to_string();
    for (module, declarations) in modules {
        program.push_str(&format!("mod {module} {{ {} }}\n", declarations.join("\n")));
    }
    program.push_str(&format!("mod master_state_identity {{ {identity} }}\n"));
    // Compile the actual refusal formatter and SHA reader used by transport.
    program.push_str(&format!("mod sha256 {{ {} }}\n", include_str!("../build_gate/sha256.rs")));
    program.push_str(&format!("mod audit_only {{ pub(crate) mod panic_origin_observation {{ {} }} }}\n", include_str!("../audit_only/panic_origin_observation.rs")));

    // Retain the real observation dependency of the original receipt core.
    // This is not a mock proof or replacement receipt implementation.
    let evidence = include_str!("../ssc_evidence_proof.rs");
    let report_access = item(include_str!("../final_report.rs"),
        "pub(crate) struct SscObservationReadAccess");
    program.push_str(&format!("mod final_report {{ {report_access} }}\n"));
    program.push_str(&format!("mod ssc_evidence_proof {{ {evidence} }}\n"));
    program.push_str(&format!("mod pipeline {{ {pipeline}\n use crate::master_state_identity::{{ExactEdit, ExactTransformationReceipt, BlockTransformationReceipt}}; use crate::structural_write_authority; {beam_data} {beam_capture} {marker} {factory} {helper}\n"));
    program.push_str(r#"
        pub fn check() {
            for (pre, start, end, replacement) in [
                ("abcd", 1, 3, "X"), ("abcd", 1, 3, "bc"),
                ("abcd", 0, 0, "before"), ("abcd", 4, 4, "after"),
                ("abcd", 0, 4, ""), ("Î±Î²Î³", 2, 4, "Î´"),
            ] {
                let mut legacy = String::with_capacity(pre.len() + replacement.len());
                legacy.push_str(&pre[..start]);
                legacy.push_str(replacement);
                legacy.push_str(&pre[end..]);
                let (post, receipt) = same_lane_strict_interior_raw_splice(pre, start, end, replacement);
                assert_eq!(post, legacy);
                assert_eq!(receipt.is_some(), pre != post);
                if let Some(receipt) = receipt {
                    assert!(receipt.minted_by::<SameLaneRawSpliceExactMint>());
                    assert_eq!(receipt.realize_post_document(pre).as_deref(), Some(post.as_str()));
                    assert!(receipt.realize_post_document(&format!("{pre} ")).is_none());
                }
            }
            for (start, end, replacement) in [(2, 3, "X"), (1, 2, "X"), (1, 3, "Y")] {
                assert!(std::panic::catch_unwind(|| mint_same_lane_raw_splice_receipt(
                    "abcd", start, end, replacement, "aXd")).is_err());
            }
        }
    }
    fn main() { pipeline::check(); }
    "#);
    let dir = std::env::temp_dir().join(format!("raw-splice-event-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    // The proof module reads its frozen envelope table (contract v4) next to itself.
    std::fs::create_dir_all(dir.join("build_gate")).unwrap();
    std::fs::write(dir.join("build_gate/ssc_containment_envelopes_v1.tsv"), include_str!("../build_gate/ssc_containment_envelopes_v1.tsv")).unwrap();
    let source = dir.join("raw_splice_event.rs");
    std::fs::write(&source, program).unwrap();
    let executable = dir.join(if cfg!(windows) { "raw_splice_event.exe" } else { "raw_splice_event" });
    let deps = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let anyhow = std::fs::read_dir(&deps).unwrap().filter_map(Result::ok)
        .map(|entry| entry.path()).find(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("libanyhow-") && name.ends_with(".rlib")
        }).unwrap();
    let roxmltree = std::fs::read_dir(&deps).unwrap().filter_map(Result::ok)
        .map(|entry| entry.path()).find(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("libroxmltree-") && name.ends_with(".rlib")
        }).unwrap();
    let flate2 = std::fs::read_dir(&deps).unwrap().filter_map(Result::ok)
        .map(|entry| entry.path()).find(|path| {
            let name = path.file_name().unwrap().to_string_lossy();
            name.starts_with("libflate2-") && name.ends_with(".rlib")
        }).unwrap();
    let result = std::process::Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .arg("--edition=2024").arg(&source).arg("-L").arg(format!("dependency={}", deps.display()))
        .arg("--extern").arg(format!("anyhow={}", anyhow.display()))
        .arg("--extern").arg(format!("roxmltree={}", roxmltree.display()))
        .arg("--extern").arg(format!("flate2={}", flate2.display())).arg("-o").arg(&executable)
        .output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let executed = std::process::Command::new(&executable).output().unwrap();
    assert!(executed.status.success(), "{}", String::from_utf8_lossy(&executed.stderr));
    std::fs::remove_dir_all(dir).unwrap();
}

}

#[cfg(test)]
mod render_cursor_continuity_tests {
use super::*;
use crate::master_state_identity::{RenderTransitionReceipts, RenderDiscardReason};
fn two_cursor_measure_source(distinct_directions: bool, ordinal: u32) -> String {
    let one = super::tests::bound_cursor_source_at(ordinal);
    let document = roxmltree::Document::parse(&one).unwrap();
    let measure = document.descendants()
        .find(|node| node.has_tag_name("measure")).unwrap();
    let range = measure.range();
    let raw = &one[range.clone()];
    let first = raw.to_string();
    let second = raw.replacen(
        &format!("number=\"{ordinal}\""),
        &format!("number=\"{}\"", ordinal + 1),
        1,
    );
    let second = if distinct_directions {
        assert!(second.contains("<mf/>"));
        second.replacen("<mf/>", "<f/>", 1)
    } else { second };
    format!("{}{}{}{}", &one[..range.start], first, second, &one[range.end..])
}

fn prepare_cursor_fixture(
    source: &str,
    historical: Option<&str>,
) -> Result<PreparedCursorNeutralWriterCandidate, CursorNeutralPreparationError> {
    let (views, _) = adapt_structural_snapshot_read_only(source, 96, "b4-fixture");
    let intents = detect_structural_repair_intents_read_only(&views);
    assert_eq!(intents.len(), 2);
    let detector = audit_identity_free_detector_module_boundary(
        views.len(), intents.len(), "b4-fixture");
    let semantic = audit_identity_free_semantic_event_equivalence(
        &intents, views.len(), "b4-fixture");
    let metamorphic = audit_identity_free_metamorphic_safety_net(
        source, 96, "b4-fixture");
    execute_authorized_cursor_neutral_writer_candidate(source,
&intents,
&detector,
&semantic,
&metamorphic,
crate::structural_write_authority::CursorNeutralExecutionStages { verifier_source_stage: "b4-fixture", writer_source_stage: "b4-fixture", historical_checkpoint_58: historical })
}

// UnabhÃ¤ngige spezifische Byteoracle fÃ¼r den bestehenden Zwei-Measure-
// #2-only-Legacyzweig. Ruft die tatsÃ¤chlichen unverÃ¤nderten Owner auf,
// nicht den neuen Orchestrator/Collector. Auswahl ist fÃ¼r diese Fixture
// eindeutig; die Assertions verhindern einen still anders verlaufenden Test.
include!("../../tests/support/render_authority/legacy_cursor_batch_oracle.rs");

#[test]
fn two_real_cursor_iterations_keep_continuous_receipts() {
    for ordinal in [11_u32, 103] {
        let source = two_cursor_measure_source(false, ordinal);
        // No historical sink: the actual Direction writer cannot compose; existing
        // #2 normalizations still execute. Both are real accepted local events.
        let prepared = prepare_cursor_fixture(&source, None).unwrap();
        assert_eq!(prepared.accepted_iterations, 2);
        assert_eq!(prepared.accepted_direction_iterations, 0);
        assert_eq!(prepared.receipts.len(), 2);
        assert!(matches!(prepared.basis, CursorNeutralPreparedBasis::Source));
        let (views, _) = adapt_structural_snapshot_read_only(
            &prepared.candidate_xml, 96, "b4-fixture-after");
        assert!(detect_structural_repair_intents_read_only(&views).is_empty());
        assert!(audit_authorized_materialization_equivalence(
            &source, &prepared.candidate_xml, "b4-fixture", None, None).green());
        // Compare against the existing legacy two-accepted fixture/oracle, not a
        // newly synthesized final result. The independent oracle executes the
        // same existing owners with the fixture-specific unambiguous legacy winners.
        assert_eq!(prepared.candidate_xml, legacy_cursor_batch_oracle(&source));
        prepared.receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);

    }
}

#[test]
fn two_real_direction_iterations_fail_before_any_checkpoint() {
    for ordinal in [11_u32, 103] {
        let source = two_cursor_measure_source(true, ordinal);
        // Distinct canonical payloads make each source Direction unique in H.
        // Both existing local token-authorized Direction writers must execute.
        let error = match prepare_cursor_fixture(&source, Some(&source)) {
            Ok(prepared) => {
                prepared.receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
                panic!("two H-based advancing Direction events must not be adoptable");
            }
            Err(error) => error,
        };
        assert!(error.accepted_iterations >= 2);
        assert_eq!(error.accepted_direction_iterations, 2);
        assert_eq!(error.message,
            "B4 accepted local event does not continue the prepared document head");
        // Owner counts increment only after real write_produced+real Some(receipt).
        // Run under the shared serial closure harness: after this call open chains=0,
        // discarded receipts=accepted_iterations+accepted_direction_iterations,
        // CandidateCoverageFailures increased by exactly 1. An intervening real
        // #2-only iteration is preserved/discarded too; it must not be omitted.

    }
}
fn direction_witness_fixture() -> (String, String) {
    let base = r#"<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="x"><direction><direction-type><dynamics><mf/></dynamics></direction-type><staff>1</staff></direction><note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>"#.to_string();
    let winner = base.replace("</direction-type><staff>", "</direction-type><offset>24</offset><staff>");
    (base, winner)
}

#[test]
fn rejected_attempt_keeps_the_actual_accepted_direction_witness() {
    let (historical, winner) = direction_witness_fixture();
    let mut witness = None;
    advance_verified_direction_for_accepted_head(&mut witness, Some(&historical), Some(&winner), true);
    assert!(witness.as_ref().is_some_and(|w| w.matches(&historical, &winner)));
    advance_verified_direction_for_accepted_head(&mut witness, Some(&historical), None, false);
    assert!(witness.as_ref().is_some_and(|w| w.matches(&historical, &winner)));
}

#[test]
fn accepted_cursor_head_change_clears_stale_direction_witness() {
    let (historical, winner) = direction_witness_fixture();
    let mut witness = None;
    advance_verified_direction_for_accepted_head(&mut witness, Some(&historical), Some(&winner), true);
    // A byte-identical accepted head retains its still exact witness.
    advance_verified_direction_for_accepted_head(&mut witness, Some(&historical), Some(&winner), false);
    assert!(witness.is_some());
    let later_cursor_head = winner.replace("<voice>1</voice>", "<voice>2</voice>");
    advance_verified_direction_for_accepted_head(&mut witness, Some(&historical), Some(&later_cursor_head), false);
    assert!(witness.is_none());
}

}

#[cfg(test)]
mod ssc_shadow_observation_tests;
#[cfg(test)]
mod ssc_proof_bundle_tests;


pub(crate) use final_notation_phrase_projection_transaction::{ApprovedPhraseOccurrenceCourse, BoundPhraseOccurrenceCourse, approve_phrase_occurrence_course};
pub(crate) use master_adoption::adopt_final_layout_phrase_projection_with_course;
