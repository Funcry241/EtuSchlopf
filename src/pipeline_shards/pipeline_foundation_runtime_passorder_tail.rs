#[derive(Debug, Clone)]
struct ConsensusRelocationLongitudinalTraceRow {
    owner: String,
    source_event_id: MaterialSourceEventId,
    attack_id: SoundingAttackId,
    measure_before: usize,
    measure_after_owner: usize,
    pitch_before: i32,
    pitch_after_owner: i32,
    onset_before: i64,
    onset_after_owner: i64,
    sounding_end_before: i64,
    sounding_end_after_owner: i64,
}

#[derive(Debug, Clone)] struct SolverCandidate {
iteration: usize, profile_name: String, profile_contract: CandidateProfile, profile_lineage: CandidateLineage, final_result: CheckReport, master_state: structural_write_authority::MasterAdoptedState, midi_seed_stats: MidiSeedStats, midi_seed_genesis: master_state_identity::MasterGenesisIdentity, reference_signature: ReferenceSignature, fidelity: FidelityCheck, arrangement_stats: ArrangementStats, material_lineage: CandidateMaterialLineageSpine, consensus_relocation_longitudinal_trace: Vec<ConsensusRelocationLongitudinalTraceRow>, consensus_relocation_freeze_boundary_xml: Option<String>,
polish_stats: PolishStats, horizontal_layout_seed: HorizontalLayoutSeedPlan, }
impl SolverCandidate {
    /// Read access to the adopted Master document. Changes go only through
    /// the adoption functions of `MasterAdoptedState`.
    fn layout_xml(&self) -> &str {
        self.master_state.document()
    }
}
 #[derive(Debug, Clone)] struct ParallelPlan { requested: String, workers: usize, candidate_budget_workers: usize, logical_threads: usize, human_potential_budget: usize,
human_potential_max_moves: usize, }
#[derive(Debug, Clone, Default)] struct RenderXmlHiddenMaterialPruneAudit { done_fragment: String, pipeline_line: String, visible_pitched_notes: usize, hidden_pitched_notes: usize,
hidden_inner_chord_tones: usize, hidden_primary_notes: usize, hidden_rest_events: usize, hidden_events_total: usize, safe_hidden_inner_chord_tones: usize, risky_hidden_primary_notes: usize, risky_timeline_rests: usize,
safe_rest_only_aux_voice_events: usize, blocked_visible_dependency: usize, unknown_hidden_events: usize, hidden_chord_on_visible_pitched_anchor: usize, hidden_chord_on_rest_anchor: usize, hidden_chord_without_visible_anchor: usize,
visible_signature_before: u64, visible_signature_after_virtual_prune: u64, visible_signature_stable: bool, backup_forward_balance_ok: bool, top_scopes: Vec<RenderXmlHiddenScopeAudit>, examples: Vec<String>, verdict: String, }
#[derive(Debug, Clone, PartialEq, Eq)] struct RetentionFirstP2SplitOccurrenceReceiptRow { scope: usize, raw_note_index: usize, pitched_ordinal: usize, onset: i64, pitch: i32, staff: String, chord_syntax: bool, }
#[derive(Debug, Clone, Default)] struct RenderXmlPrunedPdfTrial { done_fragment: String, pipeline_line: String, render_xml_written: bool, pdf_source_selected: bool, removed_safe_hidden_inner_chord_tones: usize, p2_split_occurrences: Vec<RetentionFirstP2SplitOccurrenceReceiptRow>,
skipped_risky_timeline_rests: usize, skipped_unknown_hidden_events: usize, skipped_hidden_primary_notes: usize, master_visible_signature: u64, render_visible_signature: u64, visible_signature_stable: bool, master_backup_forward_ok: bool,
render_backup_forward_ok: bool, master_visible_pitched: usize, render_visible_pitched: usize, render_hidden_pitched: usize, parse_ok: bool, export_semantics_ok: bool, verdict: String, examples: Vec<String>, }
#[derive(Debug, Clone, Default)] struct RenderXmlHiddenRestDependencyAudit { forward_possible: bool, } #[derive(Debug, Clone, Default)] struct RenderXmlHiddenRestForwardCollapseTrial { done_fragment: String, pipeline_line: String, attempted: bool, written: bool, changed_events: usize, hidden_rests_before: usize, hidden_rests_after: usize, visible_pitched_before: usize, visible_pitched_after: usize, visible_rests_before: usize,
visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize, visible_signature_before: u64, visible_signature_after: u64, visible_signature_stable: bool, backup_forward_before_ok: bool,
backup_forward_after_ok: bool, parse_ok: bool, export_semantics_ok: bool, verdict: String, examples: Vec<String>, }   #[derive(Debug, Clone, Default)]
struct RenderXmlVisibleRestReadabilityAudit { done_fragment: String, pipeline_line: String, source_label: String, checked_voice_scopes: usize, visible_pitched: usize, visible_rests: usize, hidden_pitched: usize, hidden_rests: usize,
total_rest_chains: usize, fragmented_chain_candidates: usize, aux_voice_burden_candidates: usize, nearly_empty_voice_candidates: usize, missing_visible_rest_candidates: usize, missing_rest_duration_total: i32,
helpful_consolidation_candidates: usize, consolidate_candidates: usize, suppress_candidates: usize, rewrite_candidates: usize, leave_as_is: usize, blocked_or_unknown: usize, visible_signature: u64, backup_forward_ok: bool, parse_ok: bool,
verdict: String, class_counts: BTreeMap<String, usize>, action_counts: BTreeMap<String, usize>, top_scopes: Vec<RenderXmlVisibleRestScopeAudit>, examples: Vec<String>, } #[derive(Debug, Clone, Default)]
struct RenderXmlVisibleRestScopeAudit { scope: usize, staff: String, voice: String, visible_pitched: usize, visible_rests: usize, note_duration: i32, rest_duration: i32, expected_duration: i32, ledger_duration: i32,
missing_rest_duration: i32, short_rests: usize, rest_chains: usize, max_chain_len: usize, rest_note_ratio_milli: i64, fragmentation_score: i64, pianistic_orientation_score: i64, visual_benefit_score: i64, mutation_risk: String,
classification: String, action_candidate: String, score: i64, } #[derive(Debug, Clone, Default)] struct RenderXmlPianisticRestRepairTrial { done_fragment: String, pipeline_line: String, source_label: String, attempted: bool, written: bool,
missing_candidates_seen: usize, missing_inserted: usize, consolidation_candidates_seen: usize, consolidation_attempted: usize, consolidation_adopted: usize, consolidation_rejected: usize, visible_pitched_before: usize,
visible_pitched_after: usize, visible_rests_before: usize, visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize, hidden_rests_before: usize, hidden_rests_after: usize, sounding_signature_before: u64,
sounding_signature_after: u64, visible_signature_after: u64, sounding_signature_stable: bool, visible_rest_delta: i64, backup_forward_before_ok: bool, backup_forward_after_ok: bool, parse_before_ok: bool,
parse_after_ok: bool, verdict: String, actions: Vec<RenderXmlPianisticRestRepairAction>, examples: Vec<String>, } #[derive(Debug, Clone, Default)] struct RenderXmlPianisticRestRepairAction {
scope: usize, staff: String, voice: String, verdict: String, } #[derive(Debug, Clone, Default)] struct RenderXmlPrintCleanupOneSafeVisualTrial { done_fragment: String, pipeline_line: String,
source_label: String, attempted: bool, written: bool, candidates_seen: usize, safe_candidates_seen: usize, written_candidates: usize, rejected_candidates: usize, clef_cleanup_candidates: usize, clef_cleanups_written: usize,
rest_only_clef_noops_removed: usize, empty_clef_ranges_removed: usize, trailing_clefs_removed: usize, redundant_clefs_removed: usize, advance_clefs_removed: usize, visible_pitched_before: usize, visible_pitched_after: usize,
visible_rests_before: usize, visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize, hidden_rests_before: usize, hidden_rests_after: usize, sounding_signature_before: u64, sounding_signature_after: u64, visible_signature_after: u64, sounding_signature_stable: bool, visible_rest_delta: i64, backup_forward_before_ok: bool, backup_forward_after_ok: bool, parse_before_ok: bool, parse_after_ok: bool, verdict: String, actions: Vec<RenderXmlPianisticRestRepairAction>, examples: Vec<String>, } #[derive(Debug, Clone, Default)] struct RenderXmlRestFragmentSpan { scope: usize, staff: String, voice: String, onset: i32,
duration: i32, start: usize, end: usize, fragment: String, } #[derive(Debug, Clone, Default)] struct RenderXmlTimelineFragmentSpan { scope: usize, start: usize, end: usize, onset: i32, duration: i32, staff: String, voice: String,
is_note: bool, is_forward: bool, is_rest: bool, is_hidden: bool, is_chord: bool, is_visible_rest: bool, is_visible_pitched: bool, has_visible_dependency: bool, fragment: String, } #[derive(Debug, Clone, Default)]
struct RenderXmlPianisticMissingRestInsertTrial { done_fragment: String, pipeline_line: String, source_label: String, attempted: bool, written: bool, missing_candidates_seen: usize, safe_candidates_seen: usize, insert_attempted: usize,
insert_adopted: usize, insert_rejected: usize, visible_pitched_before: usize, visible_pitched_after: usize, visible_rests_before: usize, visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize,
hidden_rests_before: usize, hidden_rests_after: usize, sounding_signature_before: u64, sounding_signature_after: u64, visible_signature_after: u64, sounding_signature_stable: bool, visible_rest_delta: i64,
backup_forward_before_ok: bool, backup_forward_after_ok: bool, parse_before_ok: bool, parse_after_ok: bool, verdict: String, actions: Vec<RenderXmlPianisticMissingRestInsertAction>, blocked_examples: Vec<String>,
examples: Vec<String>, } #[derive(Debug, Clone, Default)] struct RenderXmlPianisticMissingRestInsertAction {
scope: usize, staff: String, voice: String, } #[derive(Debug, Clone, Default)] struct RenderXmlForwardSpan { scope: usize, onset: i32, duration: i32, start: usize, end: usize, } #[derive(Debug, Clone, Default)]
struct RenderXmlMissingRestInsertCandidate { scope: usize, staff: String, voice: String, onset: i32, duration: i32, forward_start: usize, forward_end: usize, score: i64, }
#[derive(Debug, Clone, Default)] struct BehindBarsRuleCheckRestAndTieAudit {
done_fragment: String, pipeline_line: String, source_label: String, attempted: bool, written: bool, lower_staff_rest_chain_candidates_seen: usize, lower_staff_rest_chain_attempted: usize,
lower_staff_rest_chain_adopted: usize, lower_staff_rest_chain_rejected: usize, spurious_empty_voice_rest_candidates_seen: usize, spurious_empty_voice_rest_attempted: usize, spurious_empty_voice_rest_adopted: usize,
spurious_empty_voice_rest_rejected: usize, cross_staff_voice_ledger_candidates_seen: usize, cross_staff_voice_ledger_attempted: usize, cross_staff_voice_ledger_adopted: usize, cross_staff_voice_ledger_rejected: usize,
external_feedback_beat_rest_ledger_candidates_seen: usize, external_feedback_beat_rest_ledger_attempted: usize, external_feedback_beat_rest_ledger_adopted: usize, external_feedback_beat_rest_ledger_rejected: usize,
duplicate_stem_same_pitch_attempted: usize, duplicate_stem_same_pitch_adopted: usize, duplicate_stem_same_pitch_rejected: usize, same_pitch_reattack_candidates: usize, same_pitch_tie_boundary_candidates: usize,
complete_measure_rest_candidates_seen: usize, complete_measure_rest_attempted: usize, complete_measure_rest_adopted: usize, complete_measure_rest_rejected: usize, complete_measure_rest_hidden_rests_consumed: usize, complete_measure_rest_visible_rests_consumed: usize, complete_measure_rest_forwards_consumed: usize,
rest_only_auxiliary_lane_candidates_seen: usize, rest_only_auxiliary_lane_attempted: usize, rest_only_auxiliary_lane_adopted: usize, rest_only_auxiliary_lane_rejected: usize,
duplicate_stem_same_pitch_candidates: usize, duplicate_stem_same_pitch_rows: Vec<BehindBarsRuleCheckDuplicateStemRow>, visible_pitched_before: usize, visible_pitched_after: usize, visible_rests_before: usize, visible_rests_after: usize,
hidden_pitched_before: usize, hidden_pitched_after: usize, hidden_rests_before: usize, hidden_rests_after: usize, sounding_signature_before: u64, sounding_signature_after: u64, sounding_signature_stable: bool, visible_rest_delta: i64,
parse_before_ok: bool, parse_after_ok: bool, backup_forward_before_ok: bool, backup_forward_after_ok: bool, verdict: String, actions: Vec<BehindBarsRuleCheckRestAndTieAction>, same_pitch_rows: Vec<BehindBarsRuleCheckSamePitchRow>,
examples: Vec<String>, } #[derive(Debug, Clone, Default)] struct BehindBarsRuleCheckRestAndTieAction { action_kind: String, old_rest_count: usize, new_rest_count: usize, verdict: String, } #[derive(Debug, Clone, Default)] struct BehindBarsRuleCheckSamePitchRow { scope: usize, staff: String,
voice: String, first_onset: i32, has_tie_boundary: bool, classification: String, } #[derive(Debug, Clone, Default)] struct BehindBarsRuleCheckDuplicateStemRow { scope: usize,
staff: String, midi: i32, onset: i32, } #[derive(Debug, Clone, Default)] struct CrossStaffHandRoleMaterializationTrial { done_fragment: String,
pipeline_line: String, source_label: String, attempted: bool, candidates_seen: usize, candidate_attempted: usize, candidate_adopted: usize, candidate_rejected: usize, written: bool, visible_pitched_before: usize,
visible_pitched_after: usize, visible_rests_before: usize, visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize, hidden_rests_before: usize, hidden_rests_after: usize, sounding_signature_before: u64,
sounding_signature_after: u64, visible_signature_after: u64, sounding_signature_stable: bool, parse_before_ok: bool, parse_after_ok: bool, backup_forward_before_ok: bool, backup_forward_after_ok: bool, visible_rest_delta: i64,
verdict: String, actions: Vec<CrossStaffHandRoleMaterializationAction>, examples: Vec<String>, } #[derive(Debug, Clone, Default)] struct CrossStaffHandRoleMaterializationAction {
verdict: String, } #[derive(Debug, Clone, Default)] struct UpperStaffLocalVisualRestLedgerProofTrial { done_fragment: String, pipeline_line: String, source_label: String, attempted: bool, candidates_seen: usize,
candidate_attempted: usize, candidate_adopted: usize, candidate_rejected: usize, written: bool, selected_scope: usize, selected_source_voice: String, selected_rest_voice: String, lower_staff_notes: usize, staff_switches: usize,
rest_segments_written: usize, forward_segments_written: usize, rest_duration_written: i32, forward_duration_written: i32, duplicate_rest_candidates_seen: usize, duplicate_rest_suppressed: usize, duplicate_rest_duration_to_forward: i32,
duplicate_rest_scopes: String, duplicate_rest_chosen_evidence: String, visible_rest_lane_anchor_candidates_seen: usize, visible_rest_lane_anchor_suppressed: usize, visible_rest_lane_anchor_duration_to_forward: i32, visible_rest_lane_anchor_scopes: String, visible_rest_lane_anchor_chosen_evidence: String,
upper_visual_initial_rest_candidates_seen: usize, upper_visual_initial_rest_materialized: usize, upper_visual_initial_rest_scopes: String, upper_visual_initial_rest_duration: i32, upper_visual_rest_coalescing_candidates_seen: usize,
upper_visual_rest_coalesced: usize, upper_visual_rest_coalescing_scopes: String, upper_visual_rest_coalescing_duration: i32, upper_visual_duplicate_rest_candidates_seen: usize, upper_visual_duplicate_rest_suppressed: usize,
upper_visual_duplicate_rest_scopes: String, upper_visual_duplicate_rest_duration_to_forward: i32, upper_visual_duplicate_rest_interval_proof: String, upper_visual_local_ledger_before: String, upper_visual_local_ledger_after: String,
upper_rest_cross_beat_blocked: usize, upper_rest_cross_beat_scopes: String, upper_beat_boundary_invariant: bool, visual_rest_collision_before: usize, visual_rest_collision_after: usize, visual_rest_collision_green: bool,
visual_rest_collision_proof: String, visible_pitched_before: usize, visible_pitched_after: usize, visible_rests_before: usize, visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize,
hidden_rests_before: usize, hidden_rests_after: usize, sounding_signature_before: u64, sounding_signature_after: u64, sounding_signature_stable: bool, visible_rest_delta: i64, parse_before_ok: bool, parse_after_ok: bool,
backup_forward_before_ok: bool, backup_forward_after_ok: bool, verdict: String, actions: Vec<UpperStaffLocalVisualRestLedgerProofAction>, examples: Vec<String>, } #[derive(Debug, Clone, Default)]
struct UpperStaffLocalVisualRestLedgerProofAction { scope: usize, source_voice: String, rest_voice: String, lower_staff_notes: usize, staff_switches: usize, rest_parts: String, forward_parts: String, rest_duration: i32,
forward_duration: i32, } #[derive(Debug, Clone, Default)] struct LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionTrial { done_fragment: String, pipeline_line: String, source_label: String, attempted: bool,
written: bool, candidates_seen: usize, candidate_attempted: usize, candidate_adopted: usize, candidate_rejected: usize, affected_measures: usize, affected_visible_rests: usize, forward_segments_written: usize, visible_pitched_before: usize,
visible_pitched_after: usize, visible_rests_before: usize, visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize, hidden_rests_before: usize, hidden_rests_after: usize, sounding_signature_before: u64,
sounding_signature_after: u64, sounding_signature_stable: bool, parse_before_ok: bool, parse_after_ok: bool, backup_forward_before_ok: bool, backup_forward_after_ok: bool, raw_cursor_after_ok: bool, visible_rest_delta: i64, verdict: String,
actions: Vec<LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionAction>, examples: Vec<String>, } #[derive(Debug, Clone, Default)] struct LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionAction {
scope: usize, old_rest_count: usize, risk: String, } #[derive(Debug, Clone, Default)] struct RenderXmlRestConsolidationCandidate {
scope: usize, staff: String, voice: String, spans: Vec<RenderXmlRestFragmentSpan>, total_duration: i32, first_onset: i32, score: i64, } #[derive(Debug, Clone, Default)] struct RenderXmlHiddenRestVirtualStats {
changed_events: usize, } #[derive(Debug, Clone, Default)] struct RenderXmlPruneStats { removed_safe_hidden_inner_chord_tones: usize, removed_hidden_pitched_events: usize, removed_source_note_ranges: Vec<(usize, usize)>, } #[derive(Debug, Clone, Default)]
struct RenderXmlHiddenScopeAudit { scope: usize, staff: String, voice: String, hidden_pitched: usize, hidden_rests: usize, score: i64, } #[derive(Debug, Clone)] struct RenderXmlNoteEvent { scope: usize, staff: String, voice: String,
onset: i32, duration: i32, is_hidden: bool, is_rest: bool, is_chord: bool, midi: Option<i32>, anchor: RenderXmlAnchorKind, has_visible_dependency: bool, }
#[derive(Debug, Clone, Default)] struct StaccatoProxyCandidateRow { scope: usize, staff: String, voice: String, midi: i32, note_onset: i32, note_duration: i32, rest_duration: i32, total_duration: i32, beat_unit: i32, }
#[derive(Debug, Clone, Default)] struct PianistHelpfulArticulationRow { scope: usize, staff: String, voice: String, onset: i32, midi: i32, duration: i32, class_name: String, placement: String, placement_risk: String,
score: i64, } #[derive(Debug, Clone, Default)] struct PianistHelpfulArticulationAndNotationAudit { rows: Vec<PianistHelpfulArticulationRow>, } #[derive(Debug, Clone, Default)]
struct SingleClearStaccatoRenderWriteAudit { done_fragment: String, pipeline_line: String, output_path: String, written: bool, clear_candidate_count: usize, target_matched: usize,
new_staccato_marks: usize, unexpected_staccato_marks: usize, target_scope: usize, target_staff: String, target_voice: String, target_onset: i32, target_midi: i32, target_note_duration: i32, target_rest_duration: i32,
target_total_duration: i32, target_chord_member_midis: Vec<i32>, chord_cohort_readback: bool, placement_binding: String, placement_readback: String, stem_like_risk_target: usize, rest_lane_risk_target: usize, lower_staff_writes: usize, visible_pitched_before: usize, visible_pitched_after: usize,
visible_rests_before: usize, visible_rests_after: usize, hidden_pitched_before: usize, hidden_pitched_after: usize, hidden_rests_before: usize, hidden_rests_after: usize, existing_staccato_before: usize, existing_staccato_after: usize,
parse_after_ok: bool, backup_forward_after_ok: bool, staccato_equivalence_stable: bool, write_allowed: bool, render_xml_write: bool, music_mutation: bool, verdict: String, actions: Vec<BehindBarsRuleCheckRestAndTieAction>, }
#[derive(Debug, Clone, Default)] struct AssertiveVisibleArticulationRow { scope: usize, staff: String, voice: String, onset: i64, midi: i32, duration: i64, mark: String, 
score: i64, } #[derive(Debug, Clone, Default)] struct AssertiveVisibleArticulationRenderWriteAudit { written: bool, }
