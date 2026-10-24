#[derive(Debug, Clone, Default)]
struct WavTruthArticulationRow {
    scope: usize,
    onset: i64,
    wav_energy_ratio_milli: i64,
    wav_attack_ratio_milli: i64,
    wav_confidence_milli: i64,
    truth_class: String,
}
#[derive(Debug, Clone, Default)]
struct WavTruthArticulationGateAudit {
    done_fragment: String,
    pipeline_line: String,
    wav_path: String,
    wav_decoded: bool,
    wav_windows: usize,
    candidate_count: usize,
    wav_checked_candidates: usize,
    wav_reference_energy_milli: i64,
    wav_reference_attack_milli: i64,
    wav_reference_flux_milli: i64,
    wav_confirmed_accent: usize,
    wav_confirmed_tenuto: usize,
    wav_confirmed_staccato: usize,
    wav_suggested_accent: usize,
    wav_suggested_tenuto: usize,
    wav_suggested_staccato: usize,
    score_wav_match: usize,
    score_wav_mismatch: usize,
    wav_truth_strong: usize,
    wav_truth_medium: usize,
    wav_truth_blocked: usize,
    wav_rejected_score_only: usize,
    wav_review: usize,
    write_ready: usize,
    verdict: String,
    rows: Vec<WavTruthArticulationRow>,}
#[derive(Debug, Clone, Default)]
struct TieMergeAccountingAndTrueTransactionalRestCoalesceAudit {
    done_fragment: String,
    pipeline_line: String,
    source_label: String,
    attempted: bool,
    written: bool,
    duplicate_candidates_seen: usize,
    duplicate_attempted: usize,
    duplicate_adopted: usize,
    duplicate_rejected: usize,
    tie_merge_candidates_seen: usize,
    tie_merge_attempted: usize,
    tie_merge_adopted: usize,
    tie_merge_rejected: usize,
    tie_merge_equivalent_contractions: usize,
    retention_neutralized_pitched_delta: usize,
    transactional_rest_coalesce_candidates: usize,
    transactional_rest_coalesce_attempted: usize,
    transactional_rest_coalesce_adopted: usize,
    transactional_rest_coalesce_rejected: usize,
    generic_shadow_rest_candidates: usize,
    generic_shadow_rest_attempted: usize,
    generic_shadow_rest_adopted: usize,
    generic_shadow_rest_rejected: usize,
    visible_pitched_before: usize,
    visible_pitched_after: usize,
    hidden_pitched_before: usize,
    hidden_pitched_after: usize,
    visible_rests_before: usize,
    visible_rests_after: usize,
    sounding_signature_before: u64,
    sounding_signature_after: u64,
    dedup_sounding_signature_before: u64,
    dedup_sounding_signature_after: u64,
    parse_after_ok: bool,
    backup_forward_after_ok: bool,
    raw_cursor_after_ok: bool,
    music_mutation: bool,
    notation_rule_change: bool,
    verdict: String,
    actions: Vec<BehindBarsRuleCheckRestAndTieAction>,
    duplicate_rows_after: Vec<BehindBarsRuleCheckDuplicateStemRow>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderXmlAnchorKind {
    None,
    VisiblePitched,
    VisibleRest,
    HiddenPitched,
    HiddenRest,
}
#[derive(Debug, Clone, Default)]
struct RenderXmlScopeAggregate {
    visible_pitched: usize,
    visible_rests: usize,
    hidden_pitched: usize,
    hidden_rests: usize,
    safe_inner: usize,
    risky_rests: usize,
    blocked: usize,
    unknown: usize,
}
#[derive(Debug, Clone, Default)]
struct VisibleLaneCompletenessClassifierPurgeGateAudit {
    source_label: String,
    measures_checked: usize,
    lanes_checked: usize,
    independent_lanes: usize,
    auxiliary_lanes: usize,
    chord_shadow_lanes: usize,
    cross_staff_bookkeeping_lanes: usize,
    mixed_review_lanes: usize,
    complete_lanes: usize,
    incomplete_lanes: usize,
    missing_start: usize,
    missing_internal: usize,
    missing_end: usize,
    forward_masking_rest: usize,
    duplicate_visible_rests: usize,
    cross_beat_merge_blocked: usize,
    raw_time_ok: bool,
    verdict: String,
    done_fragment: String,
    pipeline_line: String,
    rows: Vec<VisibleLaneCompletenessRow>,}
#[derive(Debug, Clone, Default)]
struct VisibleLaneCompletenessRow {
    scope: usize,
    staff: String,
    voice: String,
    duplicates: usize,
    crossbeat_blocked: usize,
    verdict: String,}
#[derive(Debug, Clone, Default)]
struct StrictStaffOverfullCoverageNormalizationTrial {
    source_label: String,
    written: bool,
    candidates_seen: usize,
    candidate_attempted: usize,
    candidate_adopted: usize,
    candidate_rejected: usize,
    strict_incomplete_before: usize,
    strict_incomplete_after: usize,
    strict_missing_start_before: usize,
    strict_missing_internal_before: usize,
    strict_missing_end_before: usize,
    strict_missing_start_after: usize,
    strict_missing_internal_after: usize,
    strict_missing_end_after: usize,
    cross_staff_gluing_used_as_release_proof: usize,
    transient_wav_contradictions: usize,
    wav_gate_mode: String,
    visible_pitched_before: usize,
    visible_pitched_after: usize,
    visible_rests_before: usize,
    visible_rests_after: usize,
    hidden_pitched_before: usize,
    hidden_pitched_after: usize,
    hidden_rests_before: usize,
    hidden_rests_after: usize,
    sounding_signature_before: u64,
    sounding_signature_after: u64,
    sounding_signature_stable: bool,
    parse_before_ok: bool,
    parse_after_ok: bool,
    backup_forward_before_ok: bool,
    backup_forward_after_ok: bool,
    raw_cursor_after_ok: bool,
    verdict: String,
    done_fragment: String,
    pipeline_line: String,
    actions: Vec<BehindBarsRuleCheckRestAndTieAction>,
    rows_after: Vec<VisibleLaneCompletenessRow>,
    examples: Vec<String>,
}
#[derive(Debug, Clone)]
struct StrictStaffVisibleMeasureCandidate {
    score: i64,
    scope: usize,
    staff: String,
    voice: String,
    replacements: Vec<(usize, usize, String)>,
    overlaps_other_staff_pitched_evidence: bool,
    action: BehindBarsRuleCheckRestAndTieAction,
}




#[derive(Debug, Clone)]
struct HiddenChordToneRecoveryCandidate {
    measure: usize,
    staff: usize,
    start_tick: i64,
    duration: i64,
    midi: Option<i32>,
    voice: String,
    pitch_xml: String,
    abs_start: usize,
    abs_end: usize,
    score: i64,
}
#[derive(Debug, Clone)]
struct VisibleChordToneSwapSlot {
    measure: usize,
    staff: usize,
    start_tick: i64,
    duration: i64,
    midi: Option<i32>,
    voice: String,
    note_start: usize,
    note_end: usize,
}

#[derive(Debug, Clone, Default)]
struct EditorialRoleAudit {
    visible_voices: usize,
    editorial_weak_voices: usize,
    rest_heavy_voices: usize,
    hidden_should_sound: usize,
}
#[derive(Debug, Clone, Default)]
struct EditorialRoleMeasureAudit {
    visible_notes: usize,
    hidden_should_sound: usize,
    voices: usize,
    weak_voices: usize,
    rest_heavy_voices: usize,
    verdict: String,
}
#[derive(Debug, Clone, Default)]
struct EditorialVoiceAudit {
    staff: String,
    voice: String,
    note_count: usize,
    rest_count: usize,
    note_duration: i64,
    rest_duration: i64,
    chord_notes: usize,
    midi_min: Option<i32>,
    midi_max: Option<i32>,
    role: String,
    grade: String,
    reason: String,
}
#[derive(Debug, Clone, Default)]
struct EditorialVoiceDecisionStats {checked: usize,
    accepted: usize,
    kept: usize,
    hidden_variants: usize,
    integrated_variants: usize,
    complete_rewrite_variants: usize,
    rejected_freigabe_guard: usize,
    rejected_role_balance: usize,
    rejected_hidden_substance: usize,
    rejected_visible_loss: usize,
    rejected_isolation_guard: usize,
    frozen_baseline_reclassified: usize,
    full_rewrite_acceptance_gate_checked: usize,
    full_rewrite_acceptance_gate_delta_clean: usize,
    full_rewrite_acceptance_gate_final_clean: usize,
    full_rewrite_acceptance_gate_role_better: usize,
    full_rewrite_acceptance_gate_accepted: usize,
    final_clean_normalizer_checked: usize,
    final_clean_normalizer_normalized_clean: usize,
    final_clean_normalizer_would_accept: usize,
    final_clean_normalizer_raw_blockers: usize,
    final_clean_normalizer_raw_warnings: usize,
    final_clean_normalizer_raw_hints: usize,
    final_clean_normalizer_raw_critical: usize,
    final_clean_normalizer_baseline_subtracted: usize,
    final_clean_normalizer_normalized_blockers: usize,
    final_clean_normalizer_normalized_warnings: usize,
    final_clean_normalizer_normalized_hints: usize,
    final_clean_normalizer_normalized_critical: usize,
    final_clean_normalizer_examples: Vec<String>,
    reject_examples: Vec<String>,
    rewrite_trace_examples: Vec<String>,
    rewrite_trace_blocker_rules: BTreeMap<String, usize>,
    rewrite_trace_out_of_scope_rules: BTreeMap<String, usize>,
    rewrite_trace_in_scope_blockers: usize,
    rewrite_trace_out_of_scope_blockers: usize,
    rewrite_trace_delta_new_blockers: usize,
    rewrite_trace_baseline_old_blockers: usize,
    rewrite_trace_scope_mismatch: usize,
    rewrite_trace_underfull: usize,
    rewrite_trace_overfull: usize,
    rewrite_trace_voice_conflict: usize,
    end_weak_voices: usize,
    end_rest_heavy_voices: usize,
    end_visible_notes: usize,
    end_hidden_should_sound: usize,
    post_acceptance_audit_changed_targets: usize,
    post_acceptance_audit_examples: Vec<String>,}
#[derive(Debug, Clone, Default)]
struct EditorialVoiceDecisionTarget {
    measure: String,
    staff: String,
    voice: String,
    target_voice: Option<String>,
    grade: String,
    note_count: usize,
    rest_count: usize,
    note_ranges: Vec<(usize, usize)>,
    rest_ranges: Vec<(usize, usize)>,
}










#[derive(Debug, Clone, Default)]
struct CondensationPitchAnchor {
    midi: i32,
    step: String,
    alter: i32,
    octave: i32,
}
#[derive(Debug, Clone, Default)]
struct WavFunctionGapStaffMeasureStats {
    staff1_visible_pitched: usize,
    staff2_visible_pitched: usize,
    staff2_visible_rests: usize,
    staff2_hidden_pitched: usize,
}

















#[derive(Debug, Clone, Default)]
struct ConsensusGestureProbeStats {checked_cells: usize,
    variants_checked: usize,
    accepted: bool,
    accepted_notes: usize,
    start_visible_notes: usize,
    end_visible_notes: usize,
    rejected_blocker: usize,
    rejected_warning: usize,
    rejected_hint: usize,
    rejected_critical: usize,
    rejected_not_improved: usize,
    rejected_parse: usize,
    accepted_name: Option<String>,
    accepted_iteration: Option<usize>,
    accepted_details: Vec<String>,
    rejected_rule_hist: BTreeMap<String, usize>,
    han02_repair_variants_checked: usize,
    han02_repair_accepted: bool,
    han02_repair_details: Vec<String>,
    broken_gesture_variants_checked: usize,
    broken_gesture_accepted: bool,
    broken_gesture_details: Vec<String>,
    gauss_gesture_variants_checked: usize,
    gauss_gesture_accepted: bool,
    gauss_gesture_details: Vec<String>,
    gauss_gesture_best_residual: i64,
    gauss_input_fit_points: usize,
    gauss_candidate_raw: usize,
    gauss_candidate_after_topology: usize,
    gauss_candidate_after_rhythm: usize,
    gauss_candidate_after_hand: usize,
    gauss_candidate_after_godel: usize,
    gauss_reject_records: Vec<GaussGestureRejectRecord>,
    gauss_chain_variants_checked: usize,
    gauss_chain_accepted: bool,
    gauss_chain_best_name: Option<String>,
    gauss_chain_best_gain: i64,
    gauss_chain_best_residual: i64,
    gauss_chain_details: Vec<String>,
    gauss_chain_reject_records: Vec<GaussGestureRejectRecord>,
    hybrid_acceptance_ledger: bool,
    hybrid_profile_name: Option<String>,
    hybrid_gauss_chain_net_notes: usize,
    hybrid_broken_gesture_net_notes: usize,
    hybrid_final_visible_gain: usize,
    hybrid_anchor_detail_lines: usize,
    hybrid_neighbor_detail_lines: usize,
    gauss_tail_chain_variants_checked: usize,
    gauss_tail_chain_accepted: bool,
    gauss_tail_chain_name: Option<String>,
    gauss_tail_chain_net_notes: usize,
    gauss_tail_chain_details: Vec<String>,
    economy_constrained_tail_search: bool,
    economy_constrained_tail_checked: usize,
    economy_constrained_tail_floor_gain: usize,
    economy_constrained_tail_best_gain: usize,
    economy_constrained_tail_best_residual: i64,
    economy_constrained_tail_accepted: bool,
    economy_tail_residual_reduction: bool,
    economy_tail_residual_before: i64,
    economy_tail_residual_after: i64,
    economy_tail_residual_savings: i64,
    economy_tail_residual_strategy: String,
    examples: Vec<String>,
}
#[derive(Debug, Clone, Default)]
struct GaussGestureRejectRecord {
    candidate_name: String,
    measure: usize,
    half: usize,
    take: usize,
    residual: i64,
    visible_gain: i64,
    validator_blockers: usize,
    validator_warnings: usize,
    validator_hints: usize,
    validator_critical: usize,
    godel_certified: bool,
}


