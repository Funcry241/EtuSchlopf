//! Read-only phrase-boundary evidence foundation.
//!
//! This owner consumes an already parsed, source-bound atomic projection. It does not parse or
//! mutate MusicXML. It may derive audit-only tonal closure, relative phrase structure and slur
//! intent, but owns no anchor resolver, notation writer or productive slur capability.
//! Observation positions are adjacent true attack points inside one
//! part/staff/voice lane. Chord members are collapsed by onset. Tie-stop continuations are sounding
//! continuity, not new attacks. Rests may support an observation position but can never create one.
//! Canonical cross-measure time is supplied by the existing parsed score metric owner.

use crate::StaticNamedReportLineV766;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

const SAMPLE_LIMIT: usize = 12;
const EXISTING_TONAL_OWNER_PROFILE_GREEN_MARGIN: i64 = 48;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PhraseAtomicObservation {
    pub(crate) source_ordinal: usize,
    pub(crate) part: String,
    pub(crate) measure_ordinal: usize,
    pub(crate) staff: String,
    pub(crate) voice: String,
    pub(crate) start: i64,
    pub(crate) duration: i64,
    pub(crate) canonical_start: i128,
    pub(crate) canonical_duration: i128,
    pub(crate) canonical_measure_start: i128,
    pub(crate) canonical_beat_ticks: i128,
    pub(crate) measure_beats: i64,
    pub(crate) measure_beat_type: i64,
    pub(crate) pitch_midi: Option<i32>,
    pub(crate) visible: bool,
    pub(crate) tie_start: bool,
    pub(crate) tie_stop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LaneKey {
    part: String,
    staff: String,
    voice: String,
}

type ParallelRhythmSignature = [i128; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ParallelMelodicSignature {
    centroid_deltas_x100: [i64; 3],
    chord_members: [usize; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct JointParallelSignature {
    rhythm: ParallelRhythmSignature,
    melodic: ParallelMelodicSignature,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IntrinsicSpanRhythmSignature {
    normalized_internal_iois: Vec<i128>,
    normalized_sounding_durations: Vec<i128>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IntrinsicSpanMelodicSignature {
    centroid_deltas_x100: Vec<i64>,
    chord_members: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IntrinsicSpanParallelSignature {
    rhythm: IntrinsicSpanRhythmSignature,
    melodic: IntrinsicSpanMelodicSignature,
}

type ScorewideTonalContext = (&'static str, i32, bool, i64, i64);

#[derive(Debug, Clone)]
struct PhraseSpanSample {
    lane: LaneKey,
    start: Attack,
    end: Attack,
    attack_count: usize,
    internal_local_groups: usize,
    status: &'static str,
}

#[derive(Debug, Clone)]
struct SlurIntentSample {
    lane: LaneKey,
    start: Attack,
    end: Attack,
    attack_count: usize,
    intent: &'static str,
    reason: &'static str,
}

#[derive(Debug, Clone)]
struct PhraseNotationInternalHierarchyAuditRow {
    lane: LaneKey,
    structural_sample_end: Attack,
    structural_sample_attack_count: usize,
    projection_start: Attack,
    projection_end: Attack,
    projection_attack_count: usize,
    internal_boundary_count: usize,
    internal_structural_joint_count: usize,
    internal_closure_structure_gate_count: usize,
    internal_continuation_supported_count: usize,
    internal_conflict_count: usize,
    internal_unresolved_count: usize,
    internal_unexplained_parallel_divergence_count: usize,
    measure_span_inclusive: usize,
    terminal_role: &'static str,
}

#[derive(Debug, Clone)]
struct PhraseNotationIntrinsicSpanSignatureAuditRow {
    lane: LaneKey,
    projection_start: Attack,
    projection_end: Attack,
    projection_attack_count: usize,
    intrinsic_span_signature: Option<IntrinsicSpanParallelSignature>,
    exact_pending_span_match_count: usize,
    same_lane_exact_pending_span_match_count: usize,
    cross_lane_exact_pending_span_match_count: usize,
}

#[derive(Debug, Clone)]
struct PhraseNotationIntrinsicAllWindowMatchSample {
    lane: LaneKey,
    start: Attack,
    end: Attack,
    start_role: &'static str,
    end_role: &'static str,
    is_pending: bool,
}

#[derive(Debug, Clone)]
struct PhraseNotationIntrinsicAllWindowParallelismAuditRow {
    lane: LaneKey,
    projection_start: Attack,
    projection_end: Attack,
    projection_attack_count: usize,
    intrinsic_span_signature: Option<IntrinsicSpanParallelSignature>,
    exact_all_window_match_count: usize,
    same_lane_exact_all_window_match_count: usize,
    cross_lane_exact_all_window_match_count: usize,
    exact_non_pending_window_match_count: usize,
    match_samples: Vec<PhraseNotationIntrinsicAllWindowMatchSample>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IntrinsicSpanVariationProjection {
    rhythm: IntrinsicSpanRhythmSignature,
    normalized_onset_iois: Vec<i128>,
    centroid_deltas_x100: Vec<i64>,
    contour: Vec<i8>,
    chord_members: Vec<usize>,
}

#[derive(Debug, Clone)]
struct PhraseNotationStructuralVariationMatchSample {
    relaxation: &'static str,
    lane: LaneKey,
    start: Attack,
    end: Attack,
    is_pending: bool,
}

#[derive(Debug, Clone)]
struct PhraseNotationStructuralVariationFactorizationAuditRow {
    lane: LaneKey,
    projection_start: Attack,
    projection_end: Attack,
    projection_attack_count: usize,
    variation_projection: Option<IntrinsicSpanVariationProjection>,
    full_exact_reobserved_match_count: usize,
    texture_relaxed_match_count: usize,
    sounding_duration_relaxed_match_count: usize,
    interval_magnitude_relaxed_match_count: usize,
    rhythm_relaxed_match_count: usize,
    melody_relaxed_match_count: usize,
    match_samples: Vec<PhraseNotationStructuralVariationMatchSample>,
}

#[derive(Debug, Clone)]
struct PhraseNotationHigherLevelUnityAuditRow {
    lane: LaneKey,
    projection_start: Attack,
    projection_end: Attack,
    projection_attack_count: usize,
    internal_boundary_count: usize,
    internal_structural_joint_count: usize,
    internal_continuation_supported_count: usize,
    internal_unresolved_count: usize,
    all_internal_boundaries_continuation_supported: bool,
    span_parallel_signature: Option<Vec<JointParallelSignature>>,
    span_role_sequence: Vec<&'static str>,
    exact_span_parallel_match_count: usize,
    same_lane_exact_span_parallel_match_count: usize,
    cross_lane_exact_span_parallel_match_count: usize,
    exact_span_parallel_same_role_match_count: usize,
    exact_span_parallel_divergent_role_match_count: usize,
    terminal_exact_joint_match_count: usize,
    terminal_same_lane_match_count: usize,
    terminal_cross_lane_match_count: usize,
    terminal_role_divergence_reason: Option<&'static str>,
    terminal_role: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PhraseNotationIntentProjection {
    pub(crate) part: String,
    pub(crate) staff: String,
    pub(crate) voice: String,
    pub(crate) start_measure_ordinal: usize,
    pub(crate) start_onset: i64,
    pub(crate) start_canonical: i128,
    pub(crate) start_pitches: Vec<i32>,
    pub(crate) end_measure_ordinal: usize,
    pub(crate) end_onset: i64,
    pub(crate) end_canonical: i128,
    pub(crate) end_pitches: Vec<i32>,
    pub(crate) attack_count: usize,
}

#[derive(Debug, Clone)]
struct Attack {
    measure_ordinal: usize,
    start: i64,
    duration: i64,
    sounding_end: i64,
    canonical_start: i128,
    canonical_sounding_end: i128,
    canonical_measure_start: i128,
    canonical_beat_ticks: i128,
    measure_beats: i64,
    measure_beat_type: i64,
    pitches: Vec<i32>,
    chord_members: usize,
    tied_continuation_members_at_onset: usize,
}

#[derive(Debug, Clone)]
struct RestObservation {
    canonical_start: i128,
    canonical_end: i128,
}

#[derive(Debug, Clone)]
struct AttackProjectionStats {
    chord_collapsed_members: usize,
    tie_continuation_members_excluded: usize,
    tie_only_onsets_excluded: usize,
    mixed_onsets_with_tie_continuation: usize,
    same_measure_tie_continuation_extensions: usize,
    cross_measure_tie_continuation_extensions: usize,
}

#[derive(Debug, Clone)]
struct BoundaryObservation {
    lane: LaneKey,
    left: Attack,
    right: Attack,
    same_measure: bool,
    temporal_gap: i128,
    explicit_rest_between: bool,
    rhythmic_support: bool,
    rhythmic_oppose: bool,
    continuity_oppose: bool,
    nearest_pitch_distance: Option<i32>,
    register_centroid_delta_x100: Option<i64>,
    metric_measure_offset_ticks: Option<i128>,
    metric_beat_index_zero_based: Option<i64>,
    metric_beat_phase_ticks: Option<i128>,
    metric_on_measure_start: bool,
    metric_on_beat_grid: bool,
    melodic_previous_delta_x100: Option<i64>,
    melodic_following_delta_x100: Option<i64>,
    melodic_previous_interval: Option<i32>,
    melodic_following_interval: Option<i32>,
    melodic_contour_into_boundary: Option<&'static str>,
    melodic_contour_out_of_boundary: Option<&'static str>,
    melodic_boundary_interval_relation: Option<&'static str>,
    melodic_pitch_overlap: bool,
    parallel_rhythm_signature: Option<ParallelRhythmSignature>,
    parallel_melodic_signature: Option<ParallelMelodicSignature>,
    parallel_exact_joint_match_count: usize,
    parallel_same_lane_match_count: usize,
    parallel_cross_lane_match_count: usize,
    parallel_role_divergence_reason: Option<&'static str>,
    novelty_previous_ioi_ticks: Option<i128>,
    novelty_following_ioi_ticks: Option<i128>,
    novelty_rhythmic_context_change: bool,
    novelty_chord_texture_change: bool,
    novelty_active_lanes_before: usize,
    novelty_active_lanes_after: usize,
    novelty_score_active_lane_change: bool,
    novelty_onset_lanes_left: usize,
    novelty_onset_lanes_right: usize,
    novelty_score_onset_multiplicity_change: bool,
    novelty_density_left_beat: usize,
    novelty_density_right_beat: usize,
    novelty_score_density_change: bool,
    tonal_context_known: bool,
    tonal_profile_reliable: bool,
    tonal_tonic_pc: Option<i32>,
    tonal_dominant_pc: Option<i32>,
    tonal_leading_tone_pc: Option<i32>,
    tonal_left_attack_tonic: bool,
    tonal_left_attack_dominant: bool,
    tonal_right_attack_tonic: bool,
    tonal_right_attack_dominant: bool,
    tonal_score_tonic_at_arrival: bool,
    tonal_score_dominant_at_arrival: bool,
    tonal_bass_before_pc: Option<i32>,
    tonal_bass_arrival_pc: Option<i32>,
    tonal_bass_arrival_to_tonic: bool,
    tonal_bass_dominant_to_tonic: bool,
    tonal_lane_leading_tone_resolution: bool,
    tonal_lane_unresolved_leading_tone: bool,
    tonal_closure_support: bool,
    tonal_anti_closure: bool,
    tonal_closure_authorized: bool,
    tonal_anti_closure_authorized: bool,
    phrase_melodic_joint: bool,
    phrase_closure_structure_gate: bool,
    phrase_local_joint: bool,
    phrase_continuation_support: bool,
    phrase_boundary_role: &'static str,
    observed_family_count: usize,
}

#[derive(Debug, Clone)]
struct PhraseBoundaryEvidenceSnapshot {
    project: String,
    source_available: bool,
    source_contract: &'static str,
    canonical_ticks_per_quarter: i64,
    atomic_observations: usize,
    visible_pitched_observations: usize,
    lane_count: usize,
    attack_count: usize,
    chord_collapsed_members: usize,
    tie_start_members_observed: usize,
    tie_continuation_members_excluded: usize,
    tie_only_onsets_excluded: usize,
    mixed_onsets_with_tie_continuation: usize,
    same_measure_tie_continuation_extensions: usize,
    cross_measure_tie_continuation_extensions: usize,
    boundary_count: usize,
    same_measure_boundaries: usize,
    cross_measure_resolved: usize,
    explicit_rest_support: usize,
    positive_gap_support: usize,
    contiguous_opposition: usize,
    overlap_opposition: usize,
    melodic_descriptor_known: usize,
    metric_descriptor_known: usize,
    metric_measure_start: usize,
    metric_on_beat_grid: usize,
    metric_off_beat_grid: usize,
    melodic_context_before_known: usize,
    melodic_context_after_known: usize,
    melodic_contour_continuation_into: usize,
    melodic_contour_reversal_into: usize,
    melodic_contour_continuation_out: usize,
    melodic_contour_reversal_out: usize,
    melodic_boundary_interval_local_peak: usize,
    melodic_boundary_interval_local_valley: usize,
    melodic_pitch_overlap: usize,
    parallel_descriptor_known: usize,
    parallel_joint_repeated: usize,
    parallel_same_lane_repeated: usize,
    parallel_cross_lane_repeated: usize,
    parallel_unique_joint_patterns: usize,
    parallel_max_group_cardinality: usize,
    novelty_descriptor_known: usize,
    novelty_rhythmic_context_known: usize,
    novelty_rhythmic_context_change: usize,
    novelty_chord_texture_change: usize,
    novelty_score_active_lane_change: usize,
    novelty_score_onset_multiplicity_change: usize,
    novelty_score_density_change: usize,
    tonal_profile_name: Option<&'static str>,
    tonal_tonic_pc: Option<i32>,
    tonal_minor: Option<bool>,
    tonal_best_score: Option<i64>,
    tonal_second_score: Option<i64>,
    tonal_margin: Option<i64>,
    tonal_profile_reliable: bool,
    tonal_descriptor_known: usize,
    tonal_left_attack_tonic: usize,
    tonal_left_attack_dominant: usize,
    tonal_right_attack_tonic: usize,
    tonal_right_attack_dominant: usize,
    tonal_score_tonic_at_arrival: usize,
    tonal_score_dominant_at_arrival: usize,
    tonal_bass_arrival_to_tonic: usize,
    tonal_bass_dominant_to_tonic: usize,
    tonal_lane_leading_tone_resolution: usize,
    tonal_lane_unresolved_leading_tone: usize,
    tonal_closure_support: usize,
    tonal_anti_closure: usize,
    tonal_closure_authorized: usize,
    tonal_anti_closure_authorized: usize,
    phrase_closure_supported: usize,
    phrase_structural_joint: usize,
    phrase_continuation_supported: usize,
    phrase_conflict: usize,
    phrase_unresolved: usize,
    score_structure_regions: usize,
    score_closure_regions: usize,
    score_multi_lane_closure_regions: usize,
    score_closure_with_continuing_context: usize,
    level_zero_local_groups: usize,
    level_one_closed_spans: usize,
    level_one_open_tails: usize,
    level_one_spans_with_nested_local_groups: usize,
    slur_intent_allow_structural: usize,
    slur_intent_withhold: usize,
    slur_withhold_single_attack: usize,
    slur_withhold_voice_overlap: usize,
    slur_withhold_internal_conflict: usize,
    slur_withhold_unexplained_parallel_divergence: usize,
    parallel_role_uniform_groups: usize,
    parallel_role_divergent_groups: usize,
    parallel_role_divergent_tonal_context: usize,
    parallel_role_divergent_metric_structural_context: usize,
    parallel_role_divergent_continuity_context: usize,
    parallel_role_divergent_compound_context: usize,
    parallel_role_divergent_unexplained: usize,
    diversity_counts: [usize; 8],
    samples: Vec<BoundaryObservation>,
    phrase_span_samples: Vec<PhraseSpanSample>,
    slur_intent_samples: Vec<SlurIntentSample>,
    pending_visual_notation_intents: Vec<PhraseNotationIntentProjection>,
    notation_internal_hierarchy_audit: Vec<PhraseNotationInternalHierarchyAuditRow>,
    notation_intrinsic_span_signature_audit: Vec<PhraseNotationIntrinsicSpanSignatureAuditRow>,
    notation_intrinsic_all_window_parallelism_audit:
        Vec<PhraseNotationIntrinsicAllWindowParallelismAuditRow>,
    notation_structural_variation_factorization_audit:
        Vec<PhraseNotationStructuralVariationFactorizationAuditRow>,
    notation_higher_level_unity_audit: Vec<PhraseNotationHigherLevelUnityAuditRow>,
}

#[derive(Debug, Clone)]
struct PhraseStructureSummary {
    score_structure_regions: usize,
    score_closure_regions: usize,
    score_multi_lane_closure_regions: usize,
    score_closure_with_continuing_context: usize,
    level_zero_local_groups: usize,
    level_one_closed_spans: usize,
    level_one_open_tails: usize,
    level_one_spans_with_nested_local_groups: usize,
    slur_intent_allow_structural: usize,
    slur_intent_withhold: usize,
    slur_withhold_single_attack: usize,
    slur_withhold_voice_overlap: usize,
    slur_withhold_internal_conflict: usize,
    slur_withhold_unexplained_parallel_divergence: usize,
    phrase_span_samples: Vec<PhraseSpanSample>,
    slur_intent_samples: Vec<SlurIntentSample>,
    pending_visual_notation_intents: Vec<PhraseNotationIntentProjection>,
    notation_internal_hierarchy_audit: Vec<PhraseNotationInternalHierarchyAuditRow>,
    notation_intrinsic_span_signature_audit: Vec<PhraseNotationIntrinsicSpanSignatureAuditRow>,
    notation_intrinsic_all_window_parallelism_audit:
        Vec<PhraseNotationIntrinsicAllWindowParallelismAuditRow>,
    notation_structural_variation_factorization_audit:
        Vec<PhraseNotationStructuralVariationFactorizationAuditRow>,
    notation_higher_level_unity_audit: Vec<PhraseNotationHigherLevelUnityAuditRow>,
}

thread_local! {
    static PHRASE_BOUNDARY_EVIDENCE: RefCell<Option<PhraseBoundaryEvidenceSnapshot>> = const { RefCell::new(None) };
}

pub(crate) fn clear() {
    PHRASE_BOUNDARY_EVIDENCE.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

pub(crate) fn capture(
    project: &str,
    source_contract: &'static str,
    canonical_ticks_per_quarter: i64,
    source_xml: &str,
    observations: Vec<PhraseAtomicObservation>,
) {
    let tonal_context = crate::pipeline::scorewide_tonal_evidence_projection_from_xml(source_xml);
    let snapshot = analyze_with_tonal_context(
        project,
        true,
        source_contract,
        canonical_ticks_per_quarter,
        observations,
        tonal_context,
    );
    PHRASE_BOUNDARY_EVIDENCE.with(|slot| {
        *slot.borrow_mut() = Some(snapshot);
    });
}

pub(crate) fn capture_unavailable(project: &str, source_contract: &'static str) {
    let snapshot = analyze(project, false, source_contract, 0, Vec::new());
    PHRASE_BOUNDARY_EVIDENCE.with(|slot| {
        *slot.borrow_mut() = Some(snapshot);
    });
}

pub(crate) fn append_report_section(out: &mut String) {
    drop(pending_visual_notation_intents());
    PHRASE_BOUNDARY_EVIDENCE.with(|slot| {
        let slot = slot.borrow();
        let Some(snapshot) = slot.as_ref() else {
            return;
        };
        append_snapshot(out, snapshot);
    });
}

pub(crate) fn pending_visual_notation_intents() -> Vec<PhraseNotationIntentProjection> {
    PHRASE_BOUNDARY_EVIDENCE.with(|slot| {
        slot.borrow()
            .as_ref()
            .map(|snapshot| snapshot.pending_visual_notation_intents.clone())
            .unwrap_or_default()
    })
}

fn analyze(
    project: &str,
    source_available: bool,
    source_contract: &'static str,
    canonical_ticks_per_quarter: i64,
    observations: Vec<PhraseAtomicObservation>,
) -> PhraseBoundaryEvidenceSnapshot {
    analyze_with_tonal_context(
        project,
        source_available,
        source_contract,
        canonical_ticks_per_quarter,
        observations,
        None,
    )
}

fn analyze_with_tonal_context(
    project: &str,
    source_available: bool,
    source_contract: &'static str,
    canonical_ticks_per_quarter: i64,
    observations: Vec<PhraseAtomicObservation>,
    tonal_context: Option<ScorewideTonalContext>,
) -> PhraseBoundaryEvidenceSnapshot {
    let atomic_observations = observations.len();
    let visible_pitched_observations = observations
        .iter()
        .filter(|row| row.visible && row.pitch_midi.is_some())
        .count();
    let tie_start_members_observed = observations
        .iter()
        .filter(|row| row.visible && row.pitch_midi.is_some() && row.tie_start)
        .count();

    let mut lane_rows = BTreeMap::<LaneKey, Vec<PhraseAtomicObservation>>::new();
    for observation in observations.into_iter().filter(|row| row.visible && row.duration > 0) {
        let lane = LaneKey {
            part: observation.part.clone(),
            staff: observation.staff.clone(),
            voice: observation.voice.clone(),
        };
        lane_rows.entry(lane).or_default().push(observation);
    }

    let lane_count = lane_rows.len();
    let mut boundaries = Vec::new();
    let mut all_lane_attacks = Vec::<(LaneKey, Attack)>::new();
    let mut lane_attacks_for_structure = BTreeMap::<LaneKey, Vec<Attack>>::new();
    let mut attack_count = 0usize;
    let mut chord_collapsed_members = 0usize;
    let mut tie_continuation_members_excluded = 0usize;
    let mut tie_only_onsets_excluded = 0usize;
    let mut mixed_onsets_with_tie_continuation = 0usize;
    let mut same_measure_tie_continuation_extensions = 0usize;
    let mut cross_measure_tie_continuation_extensions = 0usize;

    for (lane, mut rows) in lane_rows {
        rows.sort_by_key(|row| (row.measure_ordinal, row.start, row.source_ordinal));
        let rests = rows
            .iter()
            .filter(|row| row.pitch_midi.is_none())
            .map(|row| RestObservation {
                canonical_start: row.canonical_start,
                canonical_end: row.canonical_start.saturating_add(row.canonical_duration),
            })
            .collect::<Vec<_>>();
        let (attacks, projection_stats) = project_true_attacks(&rows);
        attack_count = attack_count.saturating_add(attacks.len());
        lane_attacks_for_structure.insert(lane.clone(), attacks.clone());
        all_lane_attacks.extend(
            attacks
                .iter()
                .cloned()
                .map(|attack| (lane.clone(), attack)),
        );
        chord_collapsed_members =
            chord_collapsed_members.saturating_add(projection_stats.chord_collapsed_members);
        tie_continuation_members_excluded = tie_continuation_members_excluded
            .saturating_add(projection_stats.tie_continuation_members_excluded);
        tie_only_onsets_excluded =
            tie_only_onsets_excluded.saturating_add(projection_stats.tie_only_onsets_excluded);
        mixed_onsets_with_tie_continuation = mixed_onsets_with_tie_continuation
            .saturating_add(projection_stats.mixed_onsets_with_tie_continuation);
        same_measure_tie_continuation_extensions = same_measure_tie_continuation_extensions
            .saturating_add(projection_stats.same_measure_tie_continuation_extensions);
        cross_measure_tie_continuation_extensions = cross_measure_tie_continuation_extensions
            .saturating_add(projection_stats.cross_measure_tie_continuation_extensions);

        for (index, pair) in attacks.windows(2).enumerate() {
            let left = pair[0].clone();
            let right = pair[1].clone();
            let previous = index.checked_sub(1).and_then(|previous| attacks.get(previous));
            let following = attacks.get(index + 2);
            let same_measure = left.measure_ordinal == right.measure_ordinal;
            let temporal_gap = right
                .canonical_start
                .saturating_sub(left.canonical_sounding_end);
            let explicit_rest_between = temporal_gap > 0
                && rests.iter().any(|rest| {
                    rest.canonical_start >= left.canonical_sounding_end
                        && rest.canonical_start < right.canonical_start
                        && rest.canonical_end > rest.canonical_start
                });
            let rhythmic_support = temporal_gap > 0 || explicit_rest_between;
            let rhythmic_oppose = temporal_gap == 0;
            let continuity_oppose = temporal_gap <= 0;
            let boundary_pitch_distance = nearest_pitch_distance(&left.pitches, &right.pitches);
            let register_centroid_delta_x100 = centroid_delta_x100(&left, &right);

            let metric_measure_offset_ticks = right
                .canonical_start
                .checked_sub(right.canonical_measure_start)
                .filter(|offset| *offset >= 0);
            let metric_beat_phase_ticks = metric_measure_offset_ticks
                .filter(|_| right.canonical_beat_ticks > 0)
                .map(|offset| offset % right.canonical_beat_ticks);
            let metric_beat_index_zero_based = metric_measure_offset_ticks
                .filter(|_| right.canonical_beat_ticks > 0)
                .and_then(|offset| i64::try_from(offset / right.canonical_beat_ticks).ok())
                .filter(|beat| *beat < right.measure_beats);
            let metric_on_measure_start = metric_measure_offset_ticks == Some(0);
            let metric_on_beat_grid = metric_beat_phase_ticks == Some(0);

            let melodic_previous_delta_x100 =
                previous.and_then(|previous| centroid_delta_x100(previous, &left));
            let melodic_following_delta_x100 =
                following.and_then(|following| centroid_delta_x100(&right, following));
            let melodic_previous_interval =
                previous.and_then(|previous| nearest_pitch_distance(&previous.pitches, &left.pitches));
            let melodic_following_interval = following
                .and_then(|following| nearest_pitch_distance(&right.pitches, &following.pitches));
            let melodic_contour_into_boundary =
                contour_relation(melodic_previous_delta_x100, register_centroid_delta_x100);
            let melodic_contour_out_of_boundary =
                contour_relation(register_centroid_delta_x100, melodic_following_delta_x100);
            let melodic_boundary_interval_relation = interval_relation_to_neighbors(
                boundary_pitch_distance,
                melodic_previous_interval,
                melodic_following_interval,
            );
            let melodic_pitch_overlap = pitch_overlap(&left.pitches, &right.pitches);
            let parallel_rhythm_signature =
                parallel_rhythm_signature(previous, &left, &right, following);
            let parallel_melodic_signature =
                parallel_melodic_signature(previous, &left, &right, following);
            let novelty_previous_ioi_ticks =
                previous.map(|previous| left.canonical_start - previous.canonical_start);
            let novelty_following_ioi_ticks =
                following.map(|following| following.canonical_start - right.canonical_start);
            let novelty_rhythmic_context_change = matches!(
                (novelty_previous_ioi_ticks, novelty_following_ioi_ticks),
                (Some(previous), Some(following)) if previous != following
            );
            let novelty_chord_texture_change = left.chord_members != right.chord_members;
            let metric_known = metric_measure_offset_ticks.is_some()
                && metric_beat_index_zero_based.is_some()
                && metric_beat_phase_ticks.is_some();
            let observed_family_count = 1
                + if boundary_pitch_distance.is_some() { 1 } else { 0 }
                + if metric_known { 1 } else { 0 }
                + 1;

            boundaries.push(BoundaryObservation {
                lane: lane.clone(),
                left,
                right,
                same_measure,
                temporal_gap,
                explicit_rest_between,
                rhythmic_support,
                rhythmic_oppose,
                continuity_oppose,
                nearest_pitch_distance: boundary_pitch_distance,
                register_centroid_delta_x100,
                metric_measure_offset_ticks,
                metric_beat_index_zero_based,
                metric_beat_phase_ticks,
                metric_on_measure_start,
                metric_on_beat_grid,
                melodic_previous_delta_x100,
                melodic_following_delta_x100,
                melodic_previous_interval,
                melodic_following_interval,
                melodic_contour_into_boundary,
                melodic_contour_out_of_boundary,
                melodic_boundary_interval_relation,
                melodic_pitch_overlap,
                parallel_rhythm_signature,
                parallel_melodic_signature,
                parallel_exact_joint_match_count: 0,
                parallel_same_lane_match_count: 0,
                parallel_cross_lane_match_count: 0,
                parallel_role_divergence_reason: None,
                novelty_previous_ioi_ticks,
                novelty_following_ioi_ticks,
                novelty_rhythmic_context_change,
                novelty_chord_texture_change,
                novelty_active_lanes_before: 0,
                novelty_active_lanes_after: 0,
                novelty_score_active_lane_change: false,
                novelty_onset_lanes_left: 0,
                novelty_onset_lanes_right: 0,
                novelty_score_onset_multiplicity_change: false,
                novelty_density_left_beat: 0,
                novelty_density_right_beat: 0,
                novelty_score_density_change: false,
                tonal_context_known: false,
                tonal_profile_reliable: false,
                tonal_tonic_pc: None,
                tonal_dominant_pc: None,
                tonal_leading_tone_pc: None,
                tonal_left_attack_tonic: false,
                tonal_left_attack_dominant: false,
                tonal_right_attack_tonic: false,
                tonal_right_attack_dominant: false,
                tonal_score_tonic_at_arrival: false,
                tonal_score_dominant_at_arrival: false,
                tonal_bass_before_pc: None,
                tonal_bass_arrival_pc: None,
                tonal_bass_arrival_to_tonic: false,
                tonal_bass_dominant_to_tonic: false,
                tonal_lane_leading_tone_resolution: false,
                tonal_lane_unresolved_leading_tone: false,
                tonal_closure_support: false,
                tonal_anti_closure: false,
                tonal_closure_authorized: false,
                tonal_anti_closure_authorized: false,
                phrase_melodic_joint: false,
                phrase_closure_structure_gate: false,
                phrase_local_joint: false,
                phrase_continuation_support: false,
                phrase_boundary_role: "unresolved",
                observed_family_count,
            });
        }
    }

    let mut parallel_groups = BTreeMap::<JointParallelSignature, Vec<usize>>::new();
    for (index, row) in boundaries.iter().enumerate() {
        if let (Some(rhythm), Some(melodic)) =
            (row.parallel_rhythm_signature, row.parallel_melodic_signature)
        {
            parallel_groups
                .entry(JointParallelSignature { rhythm, melodic })
                .or_default()
                .push(index);
        }
    }

    for indices in parallel_groups.values() {
        for index in indices {
            let lane = boundaries[*index].lane.clone();
            let same_lane_matches = indices
                .iter()
                .filter(|other| **other != *index && boundaries[**other].lane == lane)
                .count();
            let exact_joint_matches = indices.len().saturating_sub(1);
            boundaries[*index].parallel_exact_joint_match_count = exact_joint_matches;
            boundaries[*index].parallel_same_lane_match_count = same_lane_matches;
            boundaries[*index].parallel_cross_lane_match_count =
                exact_joint_matches.saturating_sub(same_lane_matches);
            boundaries[*index].observed_family_count =
                boundaries[*index].observed_family_count.saturating_add(1);
        }
    }

    for row in &mut boundaries {
        let right_tick = row.right.canonical_start;
        let before_tick = right_tick.saturating_sub(1);
        row.novelty_active_lanes_before = active_lane_count_at(&all_lane_attacks, before_tick);
        row.novelty_active_lanes_after = active_lane_count_at(&all_lane_attacks, right_tick);
        row.novelty_score_active_lane_change =
            row.novelty_active_lanes_before != row.novelty_active_lanes_after;

        row.novelty_onset_lanes_left =
            onset_lane_count_at(&all_lane_attacks, row.left.canonical_start);
        row.novelty_onset_lanes_right =
            onset_lane_count_at(&all_lane_attacks, row.right.canonical_start);
        row.novelty_score_onset_multiplicity_change =
            row.novelty_onset_lanes_left != row.novelty_onset_lanes_right;

        let beat_ticks = row.right.canonical_beat_ticks.max(1);
        let left_window_start = right_tick.saturating_sub(beat_ticks);
        let right_window_end = right_tick.saturating_add(beat_ticks);
        row.novelty_density_left_beat =
            attack_count_in_window(&all_lane_attacks, left_window_start, right_tick);
        row.novelty_density_right_beat =
            attack_count_in_window(&all_lane_attacks, right_tick, right_window_end);
        row.novelty_score_density_change =
            row.novelty_density_left_beat != row.novelty_density_right_beat;
        row.observed_family_count = row.observed_family_count.saturating_add(1);

        if let Some((_, tonic_pc, _, best_score, second_score)) = tonal_context {
            let tonic_pc = tonic_pc.rem_euclid(12);
            let tonal_margin = best_score.saturating_sub(second_score);
            row.tonal_profile_reliable =
                tonal_margin >= EXISTING_TONAL_OWNER_PROFILE_GREEN_MARGIN;
            let dominant_pc = (tonic_pc + 7).rem_euclid(12);
            let leading_tone_pc = (tonic_pc + 11).rem_euclid(12);
            row.tonal_context_known = true;
            row.tonal_tonic_pc = Some(tonic_pc);
            row.tonal_dominant_pc = Some(dominant_pc);
            row.tonal_leading_tone_pc = Some(leading_tone_pc);
            row.tonal_left_attack_tonic = attack_contains_pitch_class(&row.left, tonic_pc);
            row.tonal_left_attack_dominant = attack_contains_pitch_class(&row.left, dominant_pc);
            row.tonal_right_attack_tonic = attack_contains_pitch_class(&row.right, tonic_pc);
            row.tonal_right_attack_dominant = attack_contains_pitch_class(&row.right, dominant_pc);
            row.tonal_score_tonic_at_arrival =
                active_score_contains_pitch_class_at(&all_lane_attacks, right_tick, tonic_pc);
            row.tonal_score_dominant_at_arrival =
                active_score_contains_pitch_class_at(&all_lane_attacks, right_tick, dominant_pc);
            row.tonal_bass_before_pc = bass_pitch_class_at(&all_lane_attacks, before_tick);
            row.tonal_bass_arrival_pc = bass_pitch_class_at(&all_lane_attacks, right_tick);
            row.tonal_bass_arrival_to_tonic = row.tonal_bass_arrival_pc == Some(tonic_pc)
                && row.tonal_bass_before_pc != Some(tonic_pc);
            row.tonal_bass_dominant_to_tonic = row.tonal_bass_before_pc == Some(dominant_pc)
                && row.tonal_bass_arrival_pc == Some(tonic_pc);
            row.tonal_lane_leading_tone_resolution =
                attack_contains_pitch_class(&row.left, leading_tone_pc)
                    && attack_contains_pitch_class(&row.right, tonic_pc);
            row.tonal_lane_unresolved_leading_tone =
                attack_contains_pitch_class(&row.left, leading_tone_pc)
                    && !attack_contains_pitch_class(&row.right, tonic_pc);
            row.tonal_closure_support = row.tonal_bass_dominant_to_tonic
                || (row.tonal_lane_leading_tone_resolution
                    && row.tonal_score_tonic_at_arrival)
                || (row.tonal_bass_arrival_to_tonic && row.tonal_right_attack_tonic);
            row.tonal_anti_closure = row.tonal_lane_unresolved_leading_tone;
            row.tonal_closure_authorized =
                row.tonal_profile_reliable && row.tonal_closure_support;
            row.tonal_anti_closure_authorized =
                row.tonal_profile_reliable && row.tonal_anti_closure;
            row.observed_family_count = row.observed_family_count.saturating_add(1);
        }

        row.phrase_melodic_joint = row.melodic_boundary_interval_relation == Some("local_peak")
            && matches!(
                (
                    row.melodic_contour_into_boundary,
                    row.melodic_contour_out_of_boundary,
                ),
                (Some("reversal"), _) | (_, Some("reversal"))
            );
        row.phrase_closure_structure_gate =
            (row.rhythmic_support && row.metric_on_beat_grid)
                || (row.novelty_score_active_lane_change && row.phrase_melodic_joint)
                || (row.metric_on_measure_start && row.novelty_rhythmic_context_change);
        row.phrase_local_joint = row.rhythmic_support
            || row.phrase_melodic_joint
            || row.novelty_score_active_lane_change
            || (row.novelty_chord_texture_change && row.novelty_rhythmic_context_change)
            || (row.novelty_score_onset_multiplicity_change
                && row.novelty_rhythmic_context_change);
        row.phrase_continuation_support = row.continuity_oppose
            && row.melodic_contour_into_boundary == Some("continuation")
            && row.melodic_contour_out_of_boundary == Some("continuation")
            && !row.novelty_rhythmic_context_change
            && !row.novelty_chord_texture_change
            && !row.novelty_score_active_lane_change
            && !row.novelty_score_onset_multiplicity_change
            && !row.tonal_closure_authorized;
        row.phrase_boundary_role = if row.tonal_closure_authorized
            && row.tonal_anti_closure_authorized
        {
            "conflict"
        } else if row.tonal_closure_authorized && row.phrase_closure_structure_gate {
            "closure_supported"
        } else if row.phrase_local_joint {
            "structural_joint"
        } else if row.phrase_continuation_support {
            "continuation_supported"
        } else {
            "unresolved"
        };
    }

    let parallel_role_audit =
        annotate_parallel_role_divergence(&mut boundaries, &parallel_groups);
    let phrase_structure = build_phrase_structure(
        &boundaries,
        &lane_attacks_for_structure,
        &all_lane_attacks,
    );

    let boundary_count = boundaries.len();
    let same_measure_boundaries = boundaries.iter().filter(|row| row.same_measure).count();
    let cross_measure_resolved = boundary_count.saturating_sub(same_measure_boundaries);
    let explicit_rest_support = boundaries
        .iter()
        .filter(|row| row.explicit_rest_between)
        .count();
    let positive_gap_support = boundaries
        .iter()
        .filter(|row| row.temporal_gap > 0)
        .count();
    let contiguous_opposition = boundaries
        .iter()
        .filter(|row| row.temporal_gap == 0)
        .count();
    let overlap_opposition = boundaries
        .iter()
        .filter(|row| row.temporal_gap < 0)
        .count();
    let melodic_descriptor_known = boundaries
        .iter()
        .filter(|row| row.nearest_pitch_distance.is_some())
        .count();
    let metric_descriptor_known = boundaries
        .iter()
        .filter(|row| {
            row.metric_measure_offset_ticks.is_some()
                && row.metric_beat_index_zero_based.is_some()
                && row.metric_beat_phase_ticks.is_some()
        })
        .count();
    let metric_measure_start = boundaries
        .iter()
        .filter(|row| row.metric_on_measure_start)
        .count();
    let metric_on_beat_grid = boundaries
        .iter()
        .filter(|row| row.metric_on_beat_grid)
        .count();
    let metric_off_beat_grid = metric_descriptor_known.saturating_sub(metric_on_beat_grid);
    let melodic_context_before_known = boundaries
        .iter()
        .filter(|row| row.melodic_previous_delta_x100.is_some())
        .count();
    let melodic_context_after_known = boundaries
        .iter()
        .filter(|row| row.melodic_following_delta_x100.is_some())
        .count();
    let melodic_contour_continuation_into = boundaries
        .iter()
        .filter(|row| row.melodic_contour_into_boundary == Some("continuation"))
        .count();
    let melodic_contour_reversal_into = boundaries
        .iter()
        .filter(|row| row.melodic_contour_into_boundary == Some("reversal"))
        .count();
    let melodic_contour_continuation_out = boundaries
        .iter()
        .filter(|row| row.melodic_contour_out_of_boundary == Some("continuation"))
        .count();
    let melodic_contour_reversal_out = boundaries
        .iter()
        .filter(|row| row.melodic_contour_out_of_boundary == Some("reversal"))
        .count();
    let melodic_boundary_interval_local_peak = boundaries
        .iter()
        .filter(|row| row.melodic_boundary_interval_relation == Some("local_peak"))
        .count();
    let melodic_boundary_interval_local_valley = boundaries
        .iter()
        .filter(|row| row.melodic_boundary_interval_relation == Some("local_valley"))
        .count();
    let melodic_pitch_overlap = boundaries
        .iter()
        .filter(|row| row.melodic_pitch_overlap)
        .count();
    let parallel_descriptor_known = boundaries
        .iter()
        .filter(|row| {
            row.parallel_rhythm_signature.is_some() && row.parallel_melodic_signature.is_some()
        })
        .count();
    let parallel_joint_repeated = boundaries
        .iter()
        .filter(|row| row.parallel_exact_joint_match_count > 0)
        .count();
    let parallel_same_lane_repeated = boundaries
        .iter()
        .filter(|row| row.parallel_same_lane_match_count > 0)
        .count();
    let parallel_cross_lane_repeated = boundaries
        .iter()
        .filter(|row| row.parallel_cross_lane_match_count > 0)
        .count();
    let parallel_unique_joint_patterns = parallel_groups.len();
    let parallel_max_group_cardinality = parallel_groups
        .values()
        .map(Vec::len)
        .max()
        .unwrap_or(0);
    let novelty_descriptor_known = boundaries.len();
    let novelty_rhythmic_context_known = boundaries
        .iter()
        .filter(|row| {
            row.novelty_previous_ioi_ticks.is_some() && row.novelty_following_ioi_ticks.is_some()
        })
        .count();
    let novelty_rhythmic_context_change = boundaries
        .iter()
        .filter(|row| row.novelty_rhythmic_context_change)
        .count();
    let novelty_chord_texture_change = boundaries
        .iter()
        .filter(|row| row.novelty_chord_texture_change)
        .count();
    let novelty_score_active_lane_change = boundaries
        .iter()
        .filter(|row| row.novelty_score_active_lane_change)
        .count();
    let novelty_score_onset_multiplicity_change = boundaries
        .iter()
        .filter(|row| row.novelty_score_onset_multiplicity_change)
        .count();
    let novelty_score_density_change = boundaries
        .iter()
        .filter(|row| row.novelty_score_density_change)
        .count();
    let tonal_descriptor_known = boundaries.iter().filter(|row| row.tonal_context_known).count();
    let tonal_left_attack_tonic = boundaries.iter().filter(|row| row.tonal_left_attack_tonic).count();
    let tonal_left_attack_dominant = boundaries
        .iter()
        .filter(|row| row.tonal_left_attack_dominant)
        .count();
    let tonal_right_attack_tonic = boundaries.iter().filter(|row| row.tonal_right_attack_tonic).count();
    let tonal_right_attack_dominant = boundaries
        .iter()
        .filter(|row| row.tonal_right_attack_dominant)
        .count();
    let tonal_score_tonic_at_arrival = boundaries
        .iter()
        .filter(|row| row.tonal_score_tonic_at_arrival)
        .count();
    let tonal_score_dominant_at_arrival = boundaries
        .iter()
        .filter(|row| row.tonal_score_dominant_at_arrival)
        .count();
    let tonal_bass_arrival_to_tonic = boundaries
        .iter()
        .filter(|row| row.tonal_bass_arrival_to_tonic)
        .count();
    let tonal_bass_dominant_to_tonic = boundaries
        .iter()
        .filter(|row| row.tonal_bass_dominant_to_tonic)
        .count();
    let tonal_lane_leading_tone_resolution = boundaries
        .iter()
        .filter(|row| row.tonal_lane_leading_tone_resolution)
        .count();
    let tonal_lane_unresolved_leading_tone = boundaries
        .iter()
        .filter(|row| row.tonal_lane_unresolved_leading_tone)
        .count();
    let tonal_closure_support = boundaries.iter().filter(|row| row.tonal_closure_support).count();
    let tonal_anti_closure = boundaries.iter().filter(|row| row.tonal_anti_closure).count();
    let tonal_closure_authorized = boundaries
        .iter()
        .filter(|row| row.tonal_closure_authorized)
        .count();
    let tonal_anti_closure_authorized = boundaries
        .iter()
        .filter(|row| row.tonal_anti_closure_authorized)
        .count();
    let phrase_closure_supported = boundaries
        .iter()
        .filter(|row| row.phrase_boundary_role == "closure_supported")
        .count();
    let phrase_structural_joint = boundaries
        .iter()
        .filter(|row| row.phrase_boundary_role == "structural_joint")
        .count();
    let phrase_continuation_supported = boundaries
        .iter()
        .filter(|row| row.phrase_boundary_role == "continuation_supported")
        .count();
    let phrase_conflict = boundaries
        .iter()
        .filter(|row| row.phrase_boundary_role == "conflict")
        .count();
    let phrase_unresolved = boundaries
        .iter()
        .filter(|row| row.phrase_boundary_role == "unresolved")
        .count();
    let mut diversity_counts = [0usize; 8];
    for row in &boundaries {
        let index = row.observed_family_count.min(7);
        diversity_counts[index] = diversity_counts[index].saturating_add(1);
    }
    let samples = boundaries.iter().take(SAMPLE_LIMIT).cloned().collect();

    PhraseBoundaryEvidenceSnapshot {
        project: project.to_string(),
        source_available,
        source_contract,
        canonical_ticks_per_quarter,
        atomic_observations,
        visible_pitched_observations,
        lane_count,
        attack_count,
        chord_collapsed_members,
        tie_start_members_observed,
        tie_continuation_members_excluded,
        tie_only_onsets_excluded,
        mixed_onsets_with_tie_continuation,
        same_measure_tie_continuation_extensions,
        cross_measure_tie_continuation_extensions,
        boundary_count,
        same_measure_boundaries,
        cross_measure_resolved,
        explicit_rest_support,
        positive_gap_support,
        contiguous_opposition,
        overlap_opposition,
        melodic_descriptor_known,
        metric_descriptor_known,
        metric_measure_start,
        metric_on_beat_grid,
        metric_off_beat_grid,
        melodic_context_before_known,
        melodic_context_after_known,
        melodic_contour_continuation_into,
        melodic_contour_reversal_into,
        melodic_contour_continuation_out,
        melodic_contour_reversal_out,
        melodic_boundary_interval_local_peak,
        melodic_boundary_interval_local_valley,
        melodic_pitch_overlap,
        parallel_descriptor_known,
        parallel_joint_repeated,
        parallel_same_lane_repeated,
        parallel_cross_lane_repeated,
        parallel_unique_joint_patterns,
        parallel_max_group_cardinality,
        novelty_descriptor_known,
        novelty_rhythmic_context_known,
        novelty_rhythmic_context_change,
        novelty_chord_texture_change,
        novelty_score_active_lane_change,
        novelty_score_onset_multiplicity_change,
        novelty_score_density_change,
        tonal_profile_name: tonal_context.map(|context| context.0),
        tonal_tonic_pc: tonal_context.map(|context| context.1.rem_euclid(12)),
        tonal_minor: tonal_context.map(|context| context.2),
        tonal_best_score: tonal_context.map(|context| context.3),
        tonal_second_score: tonal_context.map(|context| context.4),
        tonal_margin: tonal_context.map(|context| context.3.saturating_sub(context.4)),
        tonal_profile_reliable: tonal_context.is_some_and(|context| {
            context.3.saturating_sub(context.4) >= EXISTING_TONAL_OWNER_PROFILE_GREEN_MARGIN
        }),
        tonal_descriptor_known,
        tonal_left_attack_tonic,
        tonal_left_attack_dominant,
        tonal_right_attack_tonic,
        tonal_right_attack_dominant,
        tonal_score_tonic_at_arrival,
        tonal_score_dominant_at_arrival,
        tonal_bass_arrival_to_tonic,
        tonal_bass_dominant_to_tonic,
        tonal_lane_leading_tone_resolution,
        tonal_lane_unresolved_leading_tone,
        tonal_closure_support,
        tonal_anti_closure,
        tonal_closure_authorized,
        tonal_anti_closure_authorized,
        phrase_closure_supported,
        phrase_structural_joint,
        phrase_continuation_supported,
        phrase_conflict,
        phrase_unresolved,
        score_structure_regions: phrase_structure.score_structure_regions,
        score_closure_regions: phrase_structure.score_closure_regions,
        score_multi_lane_closure_regions: phrase_structure.score_multi_lane_closure_regions,
        score_closure_with_continuing_context:
            phrase_structure.score_closure_with_continuing_context,
        level_zero_local_groups: phrase_structure.level_zero_local_groups,
        level_one_closed_spans: phrase_structure.level_one_closed_spans,
        level_one_open_tails: phrase_structure.level_one_open_tails,
        level_one_spans_with_nested_local_groups:
            phrase_structure.level_one_spans_with_nested_local_groups,
        slur_intent_allow_structural: phrase_structure.slur_intent_allow_structural,
        slur_intent_withhold: phrase_structure.slur_intent_withhold,
        slur_withhold_single_attack: phrase_structure.slur_withhold_single_attack,
        slur_withhold_voice_overlap: phrase_structure.slur_withhold_voice_overlap,
        slur_withhold_internal_conflict: phrase_structure.slur_withhold_internal_conflict,
        slur_withhold_unexplained_parallel_divergence:
            phrase_structure.slur_withhold_unexplained_parallel_divergence,
        parallel_role_uniform_groups: parallel_role_audit.uniform_groups,
        parallel_role_divergent_groups: parallel_role_audit.divergent_groups,
        parallel_role_divergent_tonal_context: parallel_role_audit.tonal_context,
        parallel_role_divergent_metric_structural_context:
            parallel_role_audit.metric_structural_context,
        parallel_role_divergent_continuity_context: parallel_role_audit.continuity_context,
        parallel_role_divergent_compound_context: parallel_role_audit.compound_context,
        parallel_role_divergent_unexplained: parallel_role_audit.unexplained,
        diversity_counts,
        samples,
        phrase_span_samples: phrase_structure.phrase_span_samples,
        slur_intent_samples: phrase_structure.slur_intent_samples,
        pending_visual_notation_intents: phrase_structure.pending_visual_notation_intents,
        notation_internal_hierarchy_audit: phrase_structure.notation_internal_hierarchy_audit,
        notation_intrinsic_span_signature_audit:
            phrase_structure.notation_intrinsic_span_signature_audit,
        notation_intrinsic_all_window_parallelism_audit:
            phrase_structure.notation_intrinsic_all_window_parallelism_audit,
        notation_structural_variation_factorization_audit:
            phrase_structure.notation_structural_variation_factorization_audit,
        notation_higher_level_unity_audit: phrase_structure.notation_higher_level_unity_audit,
    }
}

fn attack_contains_pitch_class(attack: &Attack, pitch_class: i32) -> bool {
    attack
        .pitches
        .iter()
        .any(|pitch| pitch.rem_euclid(12) == pitch_class.rem_euclid(12))
}

fn active_score_contains_pitch_class_at(
    attacks: &[(LaneKey, Attack)],
    tick: i128,
    pitch_class: i32,
) -> bool {
    attacks.iter().any(|(_, attack)| {
        attack.canonical_start <= tick
            && tick < attack.canonical_sounding_end
            && attack_contains_pitch_class(attack, pitch_class)
    })
}

fn bass_pitch_class_at(attacks: &[(LaneKey, Attack)], tick: i128) -> Option<i32> {
    attacks
        .iter()
        .filter(|(_, attack)| {
            attack.canonical_start <= tick && tick < attack.canonical_sounding_end
        })
        .flat_map(|(_, attack)| attack.pitches.iter().copied())
        .min()
        .map(|pitch| pitch.rem_euclid(12))
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ParallelRoleAuditSummary {
    uniform_groups: usize,
    divergent_groups: usize,
    tonal_context: usize,
    metric_structural_context: usize,
    continuity_context: usize,
    compound_context: usize,
    unexplained: usize,
}

fn annotate_parallel_role_divergence(
    boundaries: &mut [BoundaryObservation],
    parallel_groups: &BTreeMap<JointParallelSignature, Vec<usize>>,
) -> ParallelRoleAuditSummary {
    let mut summary = ParallelRoleAuditSummary::default();
    for indices in parallel_groups.values().filter(|indices| indices.len() > 1) {
        let Some(first_index) = indices.first().copied() else {
            continue;
        };
        let first_role = boundaries[first_index].phrase_boundary_role;
        if indices
            .iter()
            .all(|index| boundaries[*index].phrase_boundary_role == first_role)
        {
            summary.uniform_groups = summary.uniform_groups.saturating_add(1);
            continue;
        }

        summary.divergent_groups = summary.divergent_groups.saturating_add(1);
        let first = &boundaries[first_index];
        let tonal_varies = indices.iter().skip(1).any(|index| {
            let row = &boundaries[*index];
            row.tonal_closure_authorized != first.tonal_closure_authorized
                || row.tonal_anti_closure_authorized != first.tonal_anti_closure_authorized
        });
        let metric_structural_varies = indices.iter().skip(1).any(|index| {
            let row = &boundaries[*index];
            row.phrase_closure_structure_gate != first.phrase_closure_structure_gate
                || row.phrase_local_joint != first.phrase_local_joint
        });
        let continuity_varies = indices.iter().skip(1).any(|index| {
            boundaries[*index].phrase_continuation_support != first.phrase_continuation_support
        });
        let varying_families = [tonal_varies, metric_structural_varies, continuity_varies]
            .iter()
            .filter(|varies| **varies)
            .count();
        let reason = match varying_families {
            0 => {
                summary.unexplained = summary.unexplained.saturating_add(1);
                "unexplained"
            }
            1 if tonal_varies => {
                summary.tonal_context = summary.tonal_context.saturating_add(1);
                "tonal_context"
            }
            1 if metric_structural_varies => {
                summary.metric_structural_context =
                    summary.metric_structural_context.saturating_add(1);
                "metric_structural_context"
            }
            1 => {
                summary.continuity_context = summary.continuity_context.saturating_add(1);
                "continuity_context"
            }
            _ => {
                summary.compound_context = summary.compound_context.saturating_add(1);
                "compound_context"
            }
        };
        for index in indices {
            boundaries[*index].parallel_role_divergence_reason = Some(reason);
        }
    }
    summary
}

fn build_phrase_structure(
    boundaries: &[BoundaryObservation],
    lane_attacks: &BTreeMap<LaneKey, Vec<Attack>>,
    all_lane_attacks: &[(LaneKey, Attack)],
) -> PhraseStructureSummary {
    let mut score_regions = BTreeMap::<i128, Vec<usize>>::new();
    for (index, row) in boundaries.iter().enumerate() {
        if matches!(
            row.phrase_boundary_role,
            "closure_supported" | "structural_joint" | "conflict"
        ) {
            score_regions
                .entry(row.right.canonical_start)
                .or_default()
                .push(index);
        }
    }

    let mut score_closure_regions = 0usize;
    let mut score_multi_lane_closure_regions = 0usize;
    let mut score_closure_with_continuing_context = 0usize;
    for (tick, indices) in &score_regions {
        let closure_lanes = indices
            .iter()
            .filter(|index| boundaries[**index].phrase_boundary_role == "closure_supported")
            .map(|index| boundaries[*index].lane.clone())
            .collect::<BTreeSet<_>>();
        if closure_lanes.is_empty() {
            continue;
        }
        score_closure_regions = score_closure_regions.saturating_add(1);
        if closure_lanes.len() > 1 {
            score_multi_lane_closure_regions = score_multi_lane_closure_regions.saturating_add(1);
        }
        let continuing_lane = all_lane_attacks.iter().any(|(lane, attack)| {
            !closure_lanes.contains(lane)
                && attack.canonical_start < *tick
                && *tick < attack.canonical_sounding_end
        });
        if continuing_lane {
            score_closure_with_continuing_context =
                score_closure_with_continuing_context.saturating_add(1);
        }
    }

    let mut boundary_indices_by_lane = BTreeMap::<LaneKey, Vec<usize>>::new();
    for (index, row) in boundaries.iter().enumerate() {
        boundary_indices_by_lane
            .entry(row.lane.clone())
            .or_default()
            .push(index);
    }

    let mut level_zero_local_groups = 0usize;
    let mut level_one_closed_spans = 0usize;
    let mut level_one_open_tails = 0usize;
    let mut level_one_spans_with_nested_local_groups = 0usize;
    let mut slur_intent_allow_structural = 0usize;
    let mut slur_intent_withhold = 0usize;
    let mut slur_withhold_single_attack = 0usize;
    let mut slur_withhold_voice_overlap = 0usize;
    let mut slur_withhold_internal_conflict = 0usize;
    let mut slur_withhold_unexplained_parallel_divergence = 0usize;
    let mut phrase_span_samples = Vec::new();
    let mut slur_intent_samples = Vec::new();
    let mut pending_visual_notation_intents = Vec::new();
    let mut notation_internal_hierarchy_audit = Vec::new();
    let mut notation_intrinsic_span_signature_audit = Vec::new();
    let mut notation_structural_variation_factorization_audit = Vec::new();
    let mut notation_higher_level_unity_audit = Vec::new();

    for (lane, attacks) in lane_attacks {
        if attacks.is_empty() {
            continue;
        }
        let indices = boundary_indices_by_lane.get(lane).cloned().unwrap_or_default();
        if indices.len().saturating_add(1) != attacks.len() {
            continue;
        }
        let strong_count = indices
            .iter()
            .filter(|index| boundaries[**index].phrase_closure_structure_gate)
            .count();
        level_zero_local_groups = level_zero_local_groups
            .saturating_add(strong_count)
            .saturating_add(1);

        let mut phrase_start_attack = 0usize;
        let mut previous_strong_boundary: Option<usize> = None;
        for (position, boundary_index) in indices.iter().copied().enumerate() {
            let row = &boundaries[boundary_index];
            if row.phrase_boundary_role == "closure_supported" {
                level_one_closed_spans = level_one_closed_spans.saturating_add(1);
                let internal_local_groups = indices[phrase_start_attack..position]
                    .iter()
                    .filter(|index| boundaries[**index].phrase_closure_structure_gate)
                    .count();
                if internal_local_groups > 0 {
                    level_one_spans_with_nested_local_groups =
                        level_one_spans_with_nested_local_groups.saturating_add(1);
                }
                if phrase_span_samples.len() < SAMPLE_LIMIT {
                    phrase_span_samples.push(PhraseSpanSample {
                        lane: lane.clone(),
                        start: attacks[phrase_start_attack].clone(),
                        end: attacks[position].clone(),
                        attack_count: position.saturating_sub(phrase_start_attack).saturating_add(1),
                        internal_local_groups,
                        status: "closed_by_closure_supported",
                    });
                }

                let local_start_attack = previous_strong_boundary
                    .map_or(0, |previous| previous.saturating_add(1));
                let local_attack_count = position
                    .saturating_sub(local_start_attack)
                    .saturating_add(1);
                let local_internal = &indices[local_start_attack..position];
                let has_overlap = local_internal
                    .iter()
                    .any(|index| boundaries[*index].temporal_gap < 0);
                let has_conflict = local_internal
                    .iter()
                    .any(|index| boundaries[*index].phrase_boundary_role == "conflict");
                let (intent, reason) = if local_attack_count < 2 {
                    slur_intent_withhold = slur_intent_withhold.saturating_add(1);
                    slur_withhold_single_attack = slur_withhold_single_attack.saturating_add(1);
                    ("withhold", "single_attack_local_group")
                } else if has_overlap {
                    slur_intent_withhold = slur_intent_withhold.saturating_add(1);
                    slur_withhold_voice_overlap = slur_withhold_voice_overlap.saturating_add(1);
                    ("withhold", "voice_overlap_inside_local_group")
                } else if has_conflict {
                    slur_intent_withhold = slur_intent_withhold.saturating_add(1);
                    slur_withhold_internal_conflict =
                        slur_withhold_internal_conflict.saturating_add(1);
                    ("withhold", "internal_boundary_conflict")
                } else if row.parallel_role_divergence_reason == Some("unexplained") {
                    slur_intent_withhold = slur_intent_withhold.saturating_add(1);
                    slur_withhold_unexplained_parallel_divergence =
                        slur_withhold_unexplained_parallel_divergence.saturating_add(1);
                    ("withhold", "unexplained_parallel_role_divergence")
                } else {
                    slur_intent_allow_structural = slur_intent_allow_structural.saturating_add(1);
                    pending_visual_notation_intents.push(PhraseNotationIntentProjection {
                        part: lane.part.clone(),
                        staff: lane.staff.clone(),
                        voice: lane.voice.clone(),
                        start_measure_ordinal: attacks[local_start_attack].measure_ordinal,
                        start_onset: attacks[local_start_attack].start,
                        start_canonical: attacks[local_start_attack].canonical_start,
                        start_pitches: attacks[local_start_attack].pitches.clone(),
                        end_measure_ordinal: attacks[position.saturating_add(1)].measure_ordinal,
                        end_onset: attacks[position.saturating_add(1)].start,
                        end_canonical: attacks[position.saturating_add(1)].canonical_start,
                        end_pitches: attacks[position.saturating_add(1)].pitches.clone(),
                        attack_count: local_attack_count.saturating_add(1),
                    });
                    let projection_end_attack = position.saturating_add(1);
                    let projection_attack_count = local_attack_count.saturating_add(1);
                    let internal_rows = &indices[local_start_attack..position];
                    notation_internal_hierarchy_audit.push(
                        PhraseNotationInternalHierarchyAuditRow {
                            lane: lane.clone(),
                            structural_sample_end: attacks[position].clone(),
                            structural_sample_attack_count: local_attack_count,
                            projection_start: attacks[local_start_attack].clone(),
                            projection_end: attacks[projection_end_attack].clone(),
                            projection_attack_count,
                            internal_boundary_count: internal_rows.len(),
                            internal_structural_joint_count: internal_rows
                                .iter()
                                .filter(|index| {
                                    boundaries[**index].phrase_boundary_role == "structural_joint"
                                })
                                .count(),
                            internal_closure_structure_gate_count: internal_rows
                                .iter()
                                .filter(|index| boundaries[**index].phrase_closure_structure_gate)
                                .count(),
                            internal_continuation_supported_count: internal_rows
                                .iter()
                                .filter(|index| {
                                    boundaries[**index].phrase_boundary_role
                                        == "continuation_supported"
                                })
                                .count(),
                            internal_conflict_count: internal_rows
                                .iter()
                                .filter(|index| {
                                    boundaries[**index].phrase_boundary_role == "conflict"
                                })
                                .count(),
                            internal_unresolved_count: internal_rows
                                .iter()
                                .filter(|index| {
                                    boundaries[**index].phrase_boundary_role == "unresolved"
                                })
                                .count(),
                            internal_unexplained_parallel_divergence_count: internal_rows
                                .iter()
                                .filter(|index| {
                                    boundaries[**index].parallel_role_divergence_reason
                                        == Some("unexplained")
                                })
                                .count(),
                            measure_span_inclusive: attacks[projection_end_attack]
                                .measure_ordinal
                                .saturating_sub(attacks[local_start_attack].measure_ordinal)
                                .saturating_add(1),
                            terminal_role: row.phrase_boundary_role,
                        },
                    );
                    let internal_structural_joint_count = internal_rows
                        .iter()
                        .filter(|index| {
                            boundaries[**index].phrase_boundary_role == "structural_joint"
                        })
                        .count();
                    let internal_continuation_supported_count = internal_rows
                        .iter()
                        .filter(|index| {
                            boundaries[**index].phrase_boundary_role == "continuation_supported"
                        })
                        .count();
                    let internal_unresolved_count = internal_rows
                        .iter()
                        .filter(|index| boundaries[**index].phrase_boundary_role == "unresolved")
                        .count();
                    let projection_boundary_rows = &indices[local_start_attack..=position];
                    let span_parallel_signature = projection_boundary_rows
                        .iter()
                        .map(|index| {
                            Some(JointParallelSignature {
                                rhythm: boundaries[*index].parallel_rhythm_signature?,
                                melodic: boundaries[*index].parallel_melodic_signature?,
                            })
                        })
                        .collect::<Option<Vec<_>>>();
                    let span_role_sequence = projection_boundary_rows
                        .iter()
                        .map(|index| boundaries[*index].phrase_boundary_role)
                        .collect::<Vec<_>>();
                    notation_intrinsic_span_signature_audit.push(
                        PhraseNotationIntrinsicSpanSignatureAuditRow {
                            lane: lane.clone(),
                            projection_start: attacks[local_start_attack].clone(),
                            projection_end: attacks[projection_end_attack].clone(),
                            projection_attack_count,
                            intrinsic_span_signature: intrinsic_span_parallel_signature(
                                &attacks[local_start_attack..=projection_end_attack],
                            ),
                            exact_pending_span_match_count: 0,
                            same_lane_exact_pending_span_match_count: 0,
                            cross_lane_exact_pending_span_match_count: 0,
                        },
                    );
                    notation_structural_variation_factorization_audit.push(
                        PhraseNotationStructuralVariationFactorizationAuditRow {
                            lane: lane.clone(),
                            projection_start: attacks[local_start_attack].clone(),
                            projection_end: attacks[projection_end_attack].clone(),
                            projection_attack_count,
                            variation_projection: intrinsic_span_variation_projection(
                                &attacks[local_start_attack..=projection_end_attack],
                            ),
                            full_exact_reobserved_match_count: 0,
                            texture_relaxed_match_count: 0,
                            sounding_duration_relaxed_match_count: 0,
                            interval_magnitude_relaxed_match_count: 0,
                            rhythm_relaxed_match_count: 0,
                            melody_relaxed_match_count: 0,
                            match_samples: Vec::new(),
                        },
                    );
                    notation_higher_level_unity_audit.push(
                        PhraseNotationHigherLevelUnityAuditRow {
                            lane: lane.clone(),
                            projection_start: attacks[local_start_attack].clone(),
                            projection_end: attacks[projection_end_attack].clone(),
                            projection_attack_count,
                            internal_boundary_count: internal_rows.len(),
                            internal_structural_joint_count,
                            internal_continuation_supported_count,
                            internal_unresolved_count,
                            all_internal_boundaries_continuation_supported: !internal_rows.is_empty()
                                && internal_continuation_supported_count == internal_rows.len(),
                            span_parallel_signature,
                            span_role_sequence,
                            exact_span_parallel_match_count: 0,
                            same_lane_exact_span_parallel_match_count: 0,
                            cross_lane_exact_span_parallel_match_count: 0,
                            exact_span_parallel_same_role_match_count: 0,
                            exact_span_parallel_divergent_role_match_count: 0,
                            terminal_exact_joint_match_count: row.parallel_exact_joint_match_count,
                            terminal_same_lane_match_count: row.parallel_same_lane_match_count,
                            terminal_cross_lane_match_count: row.parallel_cross_lane_match_count,
                            terminal_role_divergence_reason: row.parallel_role_divergence_reason,
                            terminal_role: row.phrase_boundary_role,
                        },
                    );
                    ("allow_structural_pending_visual", "closure_ending_local_group")
                };
                if slur_intent_samples.len() < SAMPLE_LIMIT {
                    slur_intent_samples.push(SlurIntentSample {
                        lane: lane.clone(),
                        start: attacks[local_start_attack].clone(),
                        end: attacks[position].clone(),
                        attack_count: local_attack_count,
                        intent,
                        reason,
                    });
                }
                phrase_start_attack = position.saturating_add(1);
            }
            if row.phrase_closure_structure_gate {
                previous_strong_boundary = Some(position);
            }
        }
        if phrase_start_attack < attacks.len() {
            level_one_open_tails = level_one_open_tails.saturating_add(1);
        }
    }

    annotate_phrase_notation_intrinsic_span_signature(
        &mut notation_intrinsic_span_signature_audit,
    );
    let mut notation_intrinsic_all_window_parallelism_audit =
        notation_intrinsic_span_signature_audit
            .iter()
            .map(|row| PhraseNotationIntrinsicAllWindowParallelismAuditRow {
                lane: row.lane.clone(),
                projection_start: row.projection_start.clone(),
                projection_end: row.projection_end.clone(),
                projection_attack_count: row.projection_attack_count,
                intrinsic_span_signature: row.intrinsic_span_signature.clone(),
                exact_all_window_match_count: 0,
                same_lane_exact_all_window_match_count: 0,
                cross_lane_exact_all_window_match_count: 0,
                exact_non_pending_window_match_count: 0,
                match_samples: Vec::new(),
            })
            .collect::<Vec<_>>();
    let boundary_role_by_lane_arrival = boundaries
        .iter()
        .map(|row| {
            (
                (row.lane.clone(), row.right.canonical_start),
                row.phrase_boundary_role,
            )
        })
        .collect::<BTreeMap<_, _>>();
    annotate_phrase_notation_intrinsic_all_window_parallelism(
        &mut notation_intrinsic_all_window_parallelism_audit,
        &notation_intrinsic_span_signature_audit,
        lane_attacks,
        &boundary_role_by_lane_arrival,
    );
    annotate_phrase_notation_structural_variation_factorization(
        &mut notation_structural_variation_factorization_audit,
        &notation_intrinsic_span_signature_audit,
        lane_attacks,
    );
    annotate_phrase_notation_higher_level_unity(&mut notation_higher_level_unity_audit);

    PhraseStructureSummary {
        score_structure_regions: score_regions.len(),
        score_closure_regions,
        score_multi_lane_closure_regions,
        score_closure_with_continuing_context,
        level_zero_local_groups,
        level_one_closed_spans,
        level_one_open_tails,
        level_one_spans_with_nested_local_groups,
        slur_intent_allow_structural,
        slur_intent_withhold,
        slur_withhold_single_attack,
        slur_withhold_voice_overlap,
        slur_withhold_internal_conflict,
        slur_withhold_unexplained_parallel_divergence,
        phrase_span_samples,
        slur_intent_samples,
        pending_visual_notation_intents,
        notation_internal_hierarchy_audit,
        notation_intrinsic_span_signature_audit,
        notation_intrinsic_all_window_parallelism_audit,
        notation_structural_variation_factorization_audit,
        notation_higher_level_unity_audit,
    }
}

fn annotate_phrase_notation_intrinsic_span_signature(
    rows: &mut [PhraseNotationIntrinsicSpanSignatureAuditRow],
) {
    let comparison_rows = rows.to_vec();
    for (index, (row, source)) in rows.iter_mut().zip(comparison_rows.iter()).enumerate() {
        let Some(signature) = source.intrinsic_span_signature.as_ref() else {
            continue;
        };
        let mut exact_matches = 0usize;
        let mut same_lane_matches = 0usize;
        let mut cross_lane_matches = 0usize;
        for (other_index, other) in comparison_rows.iter().enumerate() {
            if other_index == index
                || other.intrinsic_span_signature.as_ref() != Some(signature)
            {
                continue;
            }
            exact_matches = exact_matches.saturating_add(1);
            if other.lane == source.lane {
                same_lane_matches = same_lane_matches.saturating_add(1);
            } else {
                cross_lane_matches = cross_lane_matches.saturating_add(1);
            }
        }
        row.exact_pending_span_match_count = exact_matches;
        row.same_lane_exact_pending_span_match_count = same_lane_matches;
        row.cross_lane_exact_pending_span_match_count = cross_lane_matches;
    }
}

fn annotate_phrase_notation_intrinsic_all_window_parallelism(
    rows: &mut [PhraseNotationIntrinsicAllWindowParallelismAuditRow],
    pending_rows: &[PhraseNotationIntrinsicSpanSignatureAuditRow],
    lane_attacks: &BTreeMap<LaneKey, Vec<Attack>>,
    boundary_role_by_lane_arrival: &BTreeMap<(LaneKey, i128), &'static str>,
) {
    for row in rows {
        let Some(signature) = row.intrinsic_span_signature.as_ref() else {
            continue;
        };
        if row.projection_attack_count < 2 {
            continue;
        }
        for (candidate_lane, attacks) in lane_attacks {
            if attacks.len() < row.projection_attack_count {
                continue;
            }
            for window in attacks.windows(row.projection_attack_count) {
                let Some(start) = window.first() else {
                    continue;
                };
                let Some(end) = window.last() else {
                    continue;
                };
                let is_source_window = candidate_lane == &row.lane
                    && start.canonical_start == row.projection_start.canonical_start
                    && end.canonical_start == row.projection_end.canonical_start;
                if is_source_window
                    || intrinsic_span_parallel_signature(window).as_ref() != Some(signature)
                {
                    continue;
                }

                row.exact_all_window_match_count =
                    row.exact_all_window_match_count.saturating_add(1);
                if candidate_lane == &row.lane {
                    row.same_lane_exact_all_window_match_count =
                        row.same_lane_exact_all_window_match_count.saturating_add(1);
                } else {
                    row.cross_lane_exact_all_window_match_count =
                        row.cross_lane_exact_all_window_match_count.saturating_add(1);
                }

                let is_pending = pending_rows.iter().any(|pending| {
                    &pending.lane == candidate_lane
                        && pending.projection_attack_count == row.projection_attack_count
                        && pending.projection_start.canonical_start == start.canonical_start
                        && pending.projection_end.canonical_start == end.canonical_start
                });
                if !is_pending {
                    row.exact_non_pending_window_match_count =
                        row.exact_non_pending_window_match_count.saturating_add(1);
                }

                if row.match_samples.len() < SAMPLE_LIMIT {
                    let start_role = boundary_role_by_lane_arrival
                        .get(&(candidate_lane.clone(), start.canonical_start))
                        .copied()
                        .unwrap_or("lane_start");
                    let end_role = boundary_role_by_lane_arrival
                        .get(&(candidate_lane.clone(), end.canonical_start))
                        .copied()
                        .unwrap_or("unresolved");
                    row.match_samples
                        .push(PhraseNotationIntrinsicAllWindowMatchSample {
                            lane: candidate_lane.clone(),
                            start: start.clone(),
                            end: end.clone(),
                            start_role,
                            end_role,
                            is_pending,
                        });
                }
            }
        }
    }
}

fn annotate_phrase_notation_structural_variation_factorization(
    rows: &mut [PhraseNotationStructuralVariationFactorizationAuditRow],
    pending_rows: &[PhraseNotationIntrinsicSpanSignatureAuditRow],
    lane_attacks: &BTreeMap<LaneKey, Vec<Attack>>,
) {
    for row in rows {
        let Some(source) = row.variation_projection.as_ref() else {
            continue;
        };
        if row.projection_attack_count < 2 {
            continue;
        }

        for (candidate_lane, attacks) in lane_attacks {
            if attacks.len() < row.projection_attack_count {
                continue;
            }
            for window in attacks.windows(row.projection_attack_count) {
                let Some(start) = window.first() else {
                    continue;
                };
                let Some(end) = window.last() else {
                    continue;
                };
                let is_source_window = candidate_lane == &row.lane
                    && start.canonical_start == row.projection_start.canonical_start
                    && end.canonical_start == row.projection_end.canonical_start;
                if is_source_window {
                    continue;
                }

                let Some(candidate) = intrinsic_span_variation_projection(window) else {
                    continue;
                };
                let full_exact = candidate.rhythm == source.rhythm
                    && candidate.centroid_deltas_x100 == source.centroid_deltas_x100
                    && candidate.chord_members == source.chord_members;
                if full_exact {
                    row.full_exact_reobserved_match_count =
                        row.full_exact_reobserved_match_count.saturating_add(1);
                    continue;
                }

                let is_pending = pending_rows.iter().any(|pending| {
                    &pending.lane == candidate_lane
                        && pending.projection_attack_count == row.projection_attack_count
                        && pending.projection_start.canonical_start == start.canonical_start
                        && pending.projection_end.canonical_start == end.canonical_start
                });

                let texture_relaxed = candidate.rhythm == source.rhythm
                    && candidate.centroid_deltas_x100 == source.centroid_deltas_x100;
                if texture_relaxed {
                    row.texture_relaxed_match_count =
                        row.texture_relaxed_match_count.saturating_add(1);
                    if row.match_samples.len() < SAMPLE_LIMIT {
                        row.match_samples.push(PhraseNotationStructuralVariationMatchSample {
                            relaxation: "texture_only",
                            lane: candidate_lane.clone(),
                            start: start.clone(),
                            end: end.clone(),
                            is_pending,
                        });
                    }
                }

                let sounding_duration_relaxed =
                    candidate.normalized_onset_iois == source.normalized_onset_iois
                        && candidate.centroid_deltas_x100 == source.centroid_deltas_x100
                        && candidate.chord_members == source.chord_members;
                if sounding_duration_relaxed {
                    row.sounding_duration_relaxed_match_count = row
                        .sounding_duration_relaxed_match_count
                        .saturating_add(1);
                    if row.match_samples.len() < SAMPLE_LIMIT {
                        row.match_samples.push(PhraseNotationStructuralVariationMatchSample {
                            relaxation: "sounding_duration_only",
                            lane: candidate_lane.clone(),
                            start: start.clone(),
                            end: end.clone(),
                            is_pending,
                        });
                    }
                }

                let interval_magnitude_relaxed = candidate.rhythm == source.rhythm
                    && candidate.contour == source.contour
                    && candidate.chord_members == source.chord_members;
                if interval_magnitude_relaxed {
                    row.interval_magnitude_relaxed_match_count = row
                        .interval_magnitude_relaxed_match_count
                        .saturating_add(1);
                    if row.match_samples.len() < SAMPLE_LIMIT {
                        row.match_samples.push(PhraseNotationStructuralVariationMatchSample {
                            relaxation: "interval_magnitude_only",
                            lane: candidate_lane.clone(),
                            start: start.clone(),
                            end: end.clone(),
                            is_pending,
                        });
                    }
                }

                let rhythm_relaxed = candidate.centroid_deltas_x100 == source.centroid_deltas_x100
                    && candidate.chord_members == source.chord_members;
                if rhythm_relaxed {
                    row.rhythm_relaxed_match_count =
                        row.rhythm_relaxed_match_count.saturating_add(1);
                    if row.match_samples.len() < SAMPLE_LIMIT {
                        row.match_samples.push(PhraseNotationStructuralVariationMatchSample {
                            relaxation: "rhythm_dimension",
                            lane: candidate_lane.clone(),
                            start: start.clone(),
                            end: end.clone(),
                            is_pending,
                        });
                    }
                }

                let melody_relaxed = candidate.rhythm == source.rhythm;
                if melody_relaxed {
                    row.melody_relaxed_match_count =
                        row.melody_relaxed_match_count.saturating_add(1);
                    if row.match_samples.len() < SAMPLE_LIMIT {
                        row.match_samples.push(PhraseNotationStructuralVariationMatchSample {
                            relaxation: "melody_dimension",
                            lane: candidate_lane.clone(),
                            start: start.clone(),
                            end: end.clone(),
                            is_pending,
                        });
                    }
                }
            }
        }
    }
}

fn annotate_phrase_notation_higher_level_unity(
    rows: &mut [PhraseNotationHigherLevelUnityAuditRow],
) {
    let comparison_rows = rows.to_vec();
    for (index, (row, source)) in rows.iter_mut().zip(comparison_rows.iter()).enumerate() {
        let Some(signature) = source.span_parallel_signature.as_ref() else {
            continue;
        };
        let mut exact_matches = 0usize;
        let mut same_lane_matches = 0usize;
        let mut cross_lane_matches = 0usize;
        let mut same_role_matches = 0usize;
        let mut divergent_role_matches = 0usize;
        for (other_index, other) in comparison_rows.iter().enumerate() {
            if other_index == index
                || other.span_parallel_signature.as_ref() != Some(signature)
            {
                continue;
            }
            exact_matches = exact_matches.saturating_add(1);
            if other.lane == source.lane {
                same_lane_matches = same_lane_matches.saturating_add(1);
            } else {
                cross_lane_matches = cross_lane_matches.saturating_add(1);
            }
            if other.span_role_sequence == source.span_role_sequence {
                same_role_matches = same_role_matches.saturating_add(1);
            } else {
                divergent_role_matches = divergent_role_matches.saturating_add(1);
            }
        }
        row.exact_span_parallel_match_count = exact_matches;
        row.same_lane_exact_span_parallel_match_count = same_lane_matches;
        row.cross_lane_exact_span_parallel_match_count = cross_lane_matches;
        row.exact_span_parallel_same_role_match_count = same_role_matches;
        row.exact_span_parallel_divergent_role_match_count = divergent_role_matches;
    }
}

fn project_true_attacks(
    rows: &[PhraseAtomicObservation],
) -> (Vec<Attack>, AttackProjectionStats) {
    let mut attacks = Vec::<Attack>::new();
    let mut stats = AttackProjectionStats {
        chord_collapsed_members: 0,
        tie_continuation_members_excluded: 0,
        tie_only_onsets_excluded: 0,
        mixed_onsets_with_tie_continuation: 0,
        same_measure_tie_continuation_extensions: 0,
        cross_measure_tie_continuation_extensions: 0,
    };

    let pitched = rows
        .iter()
        .filter(|row| row.pitch_midi.is_some())
        .collect::<Vec<_>>();
    let mut cursor = 0usize;
    while cursor < pitched.len() {
        let measure_ordinal = pitched[cursor].measure_ordinal;
        let start = pitched[cursor].start;
        let canonical_start = pitched[cursor].canonical_start;
        let group_start = cursor;
        cursor = cursor.saturating_add(1);
        while cursor < pitched.len()
            && pitched[cursor].measure_ordinal == measure_ordinal
            && pitched[cursor].start == start
        {
            cursor = cursor.saturating_add(1);
        }
        let group = &pitched[group_start..cursor];
        let tied_continuations = group.iter().filter(|row| row.tie_stop).count();
        stats.tie_continuation_members_excluded = stats
            .tie_continuation_members_excluded
            .saturating_add(tied_continuations);

        let attacked = group
            .iter()
            .copied()
            .filter(|row| !row.tie_stop)
            .collect::<Vec<_>>();
        let sounding_end = group
            .iter()
            .map(|row| row.start.saturating_add(row.duration))
            .max()
            .unwrap_or(start);
        let canonical_sounding_end = group
            .iter()
            .map(|row| row.canonical_start.saturating_add(row.canonical_duration))
            .max()
            .unwrap_or(canonical_start);

        if attacked.is_empty() {
            stats.tie_only_onsets_excluded = stats.tie_only_onsets_excluded.saturating_add(1);
            if let Some(previous) = attacks.last_mut()
                && canonical_sounding_end > previous.canonical_sounding_end
            {
                if previous.measure_ordinal == measure_ordinal {
                    if sounding_end > previous.sounding_end {
                        previous.sounding_end = sounding_end;
                    }
                    stats.same_measure_tie_continuation_extensions = stats
                        .same_measure_tie_continuation_extensions
                        .saturating_add(1);
                } else {
                    stats.cross_measure_tie_continuation_extensions = stats
                        .cross_measure_tie_continuation_extensions
                        .saturating_add(1);
                }
                previous.canonical_sounding_end = canonical_sounding_end;
            }
            continue;
        }

        if tied_continuations > 0 {
            stats.mixed_onsets_with_tie_continuation = stats
                .mixed_onsets_with_tie_continuation
                .saturating_add(1);
        }

        let duration = attacked.iter().map(|row| row.duration).max().unwrap_or(0);
        let pitches = attacked
            .iter()
            .filter_map(|row| row.pitch_midi)
            .collect::<Vec<_>>();
        stats.chord_collapsed_members = stats
            .chord_collapsed_members
            .saturating_add(pitches.len().saturating_sub(1));

        attacks.push(Attack {
            measure_ordinal,
            start,
            duration,
            sounding_end,
            canonical_start,
            canonical_sounding_end,
            canonical_measure_start: attacked[0].canonical_measure_start,
            canonical_beat_ticks: attacked[0].canonical_beat_ticks,
            measure_beats: attacked[0].measure_beats,
            measure_beat_type: attacked[0].measure_beat_type,
            chord_members: pitches.len(),
            pitches,
            tied_continuation_members_at_onset: tied_continuations,
        });
    }

    (attacks, stats)
}

fn nearest_pitch_distance(left: &[i32], right: &[i32]) -> Option<i32> {
    left.iter()
        .flat_map(|left_pitch| {
            right
                .iter()
                .map(move |right_pitch| right_pitch.saturating_sub(*left_pitch).abs())
        })
        .min()
}

fn pitch_centroid_x100(pitches: &[i32]) -> Option<i64> {
    if pitches.is_empty() {
        return None;
    }
    let sum = pitches.iter().map(|pitch| i64::from(*pitch)).sum::<i64>();
    Some(sum.saturating_mul(100) / i64::try_from(pitches.len()).ok()?)
}

fn centroid_delta_x100(left: &Attack, right: &Attack) -> Option<i64> {
    pitch_centroid_x100(&left.pitches)
        .zip(pitch_centroid_x100(&right.pitches))
        .map(|(left_centroid, right_centroid)| right_centroid - left_centroid)
}

fn contour_direction(delta_x100: i64) -> i8 {
    match delta_x100.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn contour_relation(before: Option<i64>, after: Option<i64>) -> Option<&'static str> {
    let before = contour_direction(before?);
    let after = contour_direction(after?);
    Some(match (before, after) {
        (0, 0) => "stable",
        (left, right) if left != 0 && left == right => "continuation",
        (left, right) if left != 0 && right != 0 && left == -right => "reversal",
        _ => "plateau_transition",
    })
}

fn interval_relation_to_neighbors(
    boundary: Option<i32>,
    previous: Option<i32>,
    following: Option<i32>,
) -> Option<&'static str> {
    let boundary = boundary?;
    let previous = previous?;
    let following = following?;
    Some(if boundary > previous && boundary > following {
        "local_peak"
    } else if boundary < previous && boundary < following {
        "local_valley"
    } else {
        "mixed_or_equal"
    })
}

fn pitch_overlap(left: &[i32], right: &[i32]) -> bool {
    left.iter().any(|pitch| right.contains(pitch))
}

fn intrinsic_span_parallel_signature(attacks: &[Attack]) -> Option<IntrinsicSpanParallelSignature> {
    if attacks.len() < 2 {
        return None;
    }

    let mut internal_iois = attacks
        .windows(2)
        .map(|pair| pair[1].canonical_start.checked_sub(pair[0].canonical_start))
        .collect::<Option<Vec<_>>>()?;
    let mut sounding_durations = attacks
        .iter()
        .map(|attack| {
            attack
                .canonical_sounding_end
                .checked_sub(attack.canonical_start)
        })
        .collect::<Option<Vec<_>>>()?;
    if internal_iois.iter().chain(sounding_durations.iter()).any(|value| *value <= 0) {
        return None;
    }
    let divisor = internal_iois
        .iter()
        .chain(sounding_durations.iter())
        .copied()
        .fold(0_i128, phrase_positive_gcd);
    if divisor <= 0 {
        return None;
    }
    for value in &mut internal_iois {
        *value /= divisor;
    }
    for value in &mut sounding_durations {
        *value /= divisor;
    }

    let centroid_deltas_x100 = attacks
        .windows(2)
        .map(|pair| centroid_delta_x100(&pair[0], &pair[1]))
        .collect::<Option<Vec<_>>>()?;
    let chord_members = attacks.iter().map(|attack| attack.chord_members).collect();

    Some(IntrinsicSpanParallelSignature {
        rhythm: IntrinsicSpanRhythmSignature {
            normalized_internal_iois: internal_iois,
            normalized_sounding_durations: sounding_durations,
        },
        melodic: IntrinsicSpanMelodicSignature {
            centroid_deltas_x100,
            chord_members,
        },
    })
}

fn intrinsic_span_variation_projection(
    attacks: &[Attack],
) -> Option<IntrinsicSpanVariationProjection> {
    let full = intrinsic_span_parallel_signature(attacks)?;
    let internal_iois = attacks
        .windows(2)
        .map(|pair| pair[1].canonical_start.checked_sub(pair[0].canonical_start))
        .collect::<Option<Vec<_>>>()?;
    if internal_iois.iter().any(|value| *value <= 0) {
        return None;
    }
    let onset_divisor = internal_iois
        .iter()
        .copied()
        .fold(0_i128, phrase_positive_gcd);
    if onset_divisor <= 0 {
        return None;
    }
    let normalized_onset_iois = internal_iois
        .into_iter()
        .map(|value| value / onset_divisor)
        .collect::<Vec<_>>();
    let centroid_deltas_x100 = full.melodic.centroid_deltas_x100.clone();
    let contour = centroid_deltas_x100
        .iter()
        .map(|delta| delta.signum() as i8)
        .collect::<Vec<_>>();

    Some(IntrinsicSpanVariationProjection {
        rhythm: full.rhythm,
        normalized_onset_iois,
        centroid_deltas_x100,
        contour,
        chord_members: full.melodic.chord_members,
    })
}

fn parallel_rhythm_signature(
    previous: Option<&Attack>,
    left: &Attack,
    right: &Attack,
    following: Option<&Attack>,
) -> Option<ParallelRhythmSignature> {
    let previous = previous?;
    let following = following?;
    let mut values = [
        left.canonical_start.checked_sub(previous.canonical_start)?,
        right.canonical_start.checked_sub(left.canonical_start)?,
        following.canonical_start.checked_sub(right.canonical_start)?,
        left.canonical_sounding_end
            .checked_sub(left.canonical_start)?,
        right
            .canonical_sounding_end
            .checked_sub(right.canonical_start)?,
    ];
    if values.iter().any(|value| *value <= 0) {
        return None;
    }
    let divisor = values
        .iter()
        .copied()
        .fold(0_i128, phrase_positive_gcd);
    if divisor <= 0 {
        return None;
    }
    for value in &mut values {
        *value /= divisor;
    }
    Some(values)
}

fn parallel_melodic_signature(
    previous: Option<&Attack>,
    left: &Attack,
    right: &Attack,
    following: Option<&Attack>,
) -> Option<ParallelMelodicSignature> {
    let previous = previous?;
    let following = following?;
    Some(ParallelMelodicSignature {
        centroid_deltas_x100: [
            centroid_delta_x100(previous, left)?,
            centroid_delta_x100(left, right)?,
            centroid_delta_x100(right, following)?,
        ],
        chord_members: [
            previous.chord_members,
            left.chord_members,
            right.chord_members,
            following.chord_members,
        ],
    })
}

fn phrase_positive_gcd(left: i128, right: i128) -> i128 {
    if left == 0 {
        return right.max(1);
    }
    if right == 0 {
        return left.max(1);
    }
    let mut left = left.max(1);
    let mut right = right.max(1);
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

fn active_lane_count_at(attacks: &[(LaneKey, Attack)], tick: i128) -> usize {
    let mut lanes = BTreeSet::new();
    for (lane, attack) in attacks {
        if attack.canonical_start <= tick && tick < attack.canonical_sounding_end {
            lanes.insert(lane.clone());
        }
    }
    lanes.len()
}

fn onset_lane_count_at(attacks: &[(LaneKey, Attack)], tick: i128) -> usize {
    let mut lanes = BTreeSet::new();
    for (lane, attack) in attacks {
        if attack.canonical_start == tick {
            lanes.insert(lane.clone());
        }
    }
    lanes.len()
}

fn attack_count_in_window(
    attacks: &[(LaneKey, Attack)],
    start_inclusive: i128,
    end_exclusive: i128,
) -> usize {
    attacks
        .iter()
        .filter(|(_, attack)| {
            attack.canonical_start >= start_inclusive && attack.canonical_start < end_exclusive
        })
        .count()
}

fn append_snapshot(out: &mut String, snapshot: &PhraseBoundaryEvidenceSnapshot) {
    out.push_str("\nPHRASE-BOUNDARY-EVIDENCE-FOUNDATION-AUDIT-NO-MUTATION\n");
    out.push_str("----------------------------------------------------------\n");
    out.push_str("Status: audit_only_structure_intent; Evidence -> Structure -> Intent -> Notation; current_stage Structure+Intent; tonal_closure_phrase_structure_slur_intent active.\n");
    out.push_str("ObservationPositionContract: adjacent_true_attacks_within_part_staff_voice_lane; chord_members_collapse_by_onset; tie_stop_continuations_are_not_new_attacks; rests_support_but_never_create_positions.\n");
    out.push_str("EvidenceContract: multidimensional_descriptors; positive_and_negative_evidence_remain_separate; no_scalarization.\n");
    out.push_str("AuthorityContract: PhraseClassification audit_only_qualitative; PhraseSpan audit_only_relative_hierarchy; SlurIntent audit_only_structural; NotationPolicy audit_only_precision_first; AnchorResolver none.\n");
    out.push_str("MutationContract: RenderXmlWrite nein; RenderNotationMutation nein; SoundingEventMutation nein; MasterMusicXmlMutation nein.\n");
    out.push_str("ExternalDataContract: corpora nein; models nein; embeddings nein; learned_weights nein; runtime_lookup_tables nein.\n");
    out.push_format_args(format_args!(
        "Project: {}; SourceContract: {}; SourceAvailable: {}.\n",
        snapshot.project,
        snapshot.source_contract,
        yes_no(snapshot.source_available),
    ));
    out.push_format_args(format_args!(
        "CanonicalTime: TicksPerQuarter {}; MetricOwner existing_musicxml_parse_score_measure_expected; exact_integer_and_full_measure_binding_required; barline_is_not_phrase_evidence.\n",
        snapshot.canonical_ticks_per_quarter,
    ));
    out.push_format_args(format_args!(
        "ProjectionCounts: AtomicObservations {}; VisiblePitchedObservations {}; LaneCount {}; TrueAttacksAfterChordTieProjection {}; ChordMembersCollapsed {}; BoundaryObservations {}.\n",
        snapshot.atomic_observations,
        snapshot.visible_pitched_observations,
        snapshot.lane_count,
        snapshot.attack_count,
        snapshot.chord_collapsed_members,
        snapshot.boundary_count,
    ));
    out.push_format_args(format_args!(
        "TieContinuityEvidence: TieStartMembersObserved {}; TieContinuationMembersExcluded {}; TieOnlyOnsetsExcluded {}; MixedOnsetsWithTieContinuation {}; SameMeasureTieContinuationExtensions {}; CrossMeasureTieContinuationExtensions {}; tie_stop_is_continuity_not_attack.\n",
        snapshot.tie_start_members_observed,
        snapshot.tie_continuation_members_excluded,
        snapshot.tie_only_onsets_excluded,
        snapshot.mixed_onsets_with_tie_continuation,
        snapshot.same_measure_tie_continuation_extensions,
        snapshot.cross_measure_tie_continuation_extensions,
    ));
    out.push_format_args(format_args!(
        "RhythmicEvidence: SameMeasureBoundaries {}; CrossMeasureRhythmResolved {}; CrossMeasureRhythmUnresolved 0; PositiveGapSupport {}; ExplicitRestSupport {}; ContiguousOpposition {}; OverlapOpposition {}.\n",
        snapshot.same_measure_boundaries,
        snapshot.cross_measure_resolved,
        snapshot.positive_gap_support,
        snapshot.explicit_rest_support,
        snapshot.contiguous_opposition,
        snapshot.overlap_opposition,
    ));
    out.push_format_args(format_args!(
        "MetricEvidence: DescriptorKnown {}; RightOnMeasureStart {}; RightOnBeatGrid {}; RightOffBeatGrid {}; descriptor_only; meter_strength_classification none; barline_support none.\n",
        snapshot.metric_descriptor_known,
        snapshot.metric_measure_start,
        snapshot.metric_on_beat_grid,
        snapshot.metric_off_beat_grid,
    ));
    out.push_format_args(format_args!(
        "MelodicEvidence: DescriptorKnown {}; ContextBeforeKnown {}; ContextAfterKnown {}; ContourContinuationInto {}; ContourReversalInto {}; ContourContinuationOut {}; ContourReversalOut {}; BoundaryIntervalLocalPeak {}; BoundaryIntervalLocalValley {}; PitchOverlapAcrossBoundary {}; classification_threshold none; absolute_jump_threshold none; descriptors_are_relative_observations_only.\n",
        snapshot.melodic_descriptor_known,
        snapshot.melodic_context_before_known,
        snapshot.melodic_context_after_known,
        snapshot.melodic_contour_continuation_into,
        snapshot.melodic_contour_reversal_into,
        snapshot.melodic_contour_continuation_out,
        snapshot.melodic_contour_reversal_out,
        snapshot.melodic_boundary_interval_local_peak,
        snapshot.melodic_boundary_interval_local_valley,
        snapshot.melodic_pitch_overlap,
    ));
    out.push_format_args(format_args!(
        "ParallelismEvidence: DescriptorKnown {}; ExactJointPatternRepeated {}; SameLaneExactRepeat {}; CrossLaneExactRepeat {}; UniqueJointPatterns {}; MaxExactGroupCardinality {}; transposition_invariant centroid_delta_shape; augmentation_invariant exact_rhythm_ratio_shape; exact_match_descriptor_only; no_similarity_score.\n",
        snapshot.parallel_descriptor_known,
        snapshot.parallel_joint_repeated,
        snapshot.parallel_same_lane_repeated,
        snapshot.parallel_cross_lane_repeated,
        snapshot.parallel_unique_joint_patterns,
        snapshot.parallel_max_group_cardinality,
    ));
    out.push_format_args(format_args!(
        "StructuralNoveltyEvidence: DescriptorKnown {}; RhythmicContextKnown {}; RhythmicContextChange {}; ChordTextureChange {}; ScoreActiveLaneChange {}; ScoreOnsetMultiplicityChange {}; ScoreOneBeatAttackDensityChange {}; exact_state_change_descriptors_only; no_novelty_score.\n",
        snapshot.novelty_descriptor_known,
        snapshot.novelty_rhythmic_context_known,
        snapshot.novelty_rhythmic_context_change,
        snapshot.novelty_chord_texture_change,
        snapshot.novelty_score_active_lane_change,
        snapshot.novelty_score_onset_multiplicity_change,
        snapshot.novelty_score_density_change,
    ));
    out.push_format_args(format_args!(
        "TonalClosureEvidence: ContextKnown {}; Profile {}; TonicPc {}; Mode {}; BestScore {}; SecondScore {}; Margin {}; ProfileReliable {}; ExistingOwnerProfileGreenMargin {}; LeftAttackTonic {}; LeftAttackDominant {}; RightAttackTonic {}; RightAttackDominant {}; ScoreTonicAtArrival {}; ScoreDominantAtArrival {}; BassArrivalToTonic {}; BassDominantToTonic {}; LaneLeadingToneResolution {}; LaneUnresolvedLeadingTone {}; ClosureSupportRaw {}; AntiClosureRaw {}; ClosureAuthorized {}; AntiClosureAuthorized {}.\n",
        snapshot.tonal_descriptor_known,
        optional_str(snapshot.tonal_profile_name),
        optional_i32(snapshot.tonal_tonic_pc),
        snapshot.tonal_minor.map_or("unresolved", |minor| if minor { "minor" } else { "major" }),
        optional_i64(snapshot.tonal_best_score),
        optional_i64(snapshot.tonal_second_score),
        optional_i64(snapshot.tonal_margin),
        yes_no(snapshot.tonal_profile_reliable),
        EXISTING_TONAL_OWNER_PROFILE_GREEN_MARGIN,
        snapshot.tonal_left_attack_tonic,
        snapshot.tonal_left_attack_dominant,
        snapshot.tonal_right_attack_tonic,
        snapshot.tonal_right_attack_dominant,
        snapshot.tonal_score_tonic_at_arrival,
        snapshot.tonal_score_dominant_at_arrival,
        snapshot.tonal_bass_arrival_to_tonic,
        snapshot.tonal_bass_dominant_to_tonic,
        snapshot.tonal_lane_leading_tone_resolution,
        snapshot.tonal_lane_unresolved_leading_tone,
        snapshot.tonal_closure_support,
        snapshot.tonal_anti_closure,
        snapshot.tonal_closure_authorized,
        snapshot.tonal_anti_closure_authorized,
    ));
    out.push_format_args(format_args!(
        "PhraseStructureAudit: BoundaryRoles closure_supported {} structural_joint {} continuation_supported {} conflict {} unresolved {}; ScoreStructureRegions {}; ScoreClosureRegions {}; MultiLaneClosureRegions {}; ClosureWithContinuingContext {}; GroupingLevel0LocalGroups {}; GroupingLevel1ClosedSpans {}; OpenTails {}; ClosedSpansWithNestedLocalGroups {}; no_majority_vote.\n",
        snapshot.phrase_closure_supported,
        snapshot.phrase_structural_joint,
        snapshot.phrase_continuation_supported,
        snapshot.phrase_conflict,
        snapshot.phrase_unresolved,
        snapshot.score_structure_regions,
        snapshot.score_closure_regions,
        snapshot.score_multi_lane_closure_regions,
        snapshot.score_closure_with_continuing_context,
        snapshot.level_zero_local_groups,
        snapshot.level_one_closed_spans,
        snapshot.level_one_open_tails,
        snapshot.level_one_spans_with_nested_local_groups,
    ));
    out.push_format_args(format_args!(
        "SlurIntentNotationPolicyAudit: AllowStructuralPendingVisual {}; Withhold {}; WithholdSingleAttack {}; WithholdVoiceOverlap {}; WithholdInternalConflict {}; WithholdUnexplainedParallelRoleDivergence {}; DefaultNoSlur ja; GroupingLevelEqualsSlurLevel nein; AnchorResolver none; WriterOpen nein; RenderXmlWrite nein.\n",
        snapshot.slur_intent_allow_structural,
        snapshot.slur_intent_withhold,
        snapshot.slur_withhold_single_attack,
        snapshot.slur_withhold_voice_overlap,
        snapshot.slur_withhold_internal_conflict,
        snapshot.slur_withhold_unexplained_parallel_divergence,
    ));
    let hierarchy_rows_with_internal_structural_joints = snapshot
        .notation_internal_hierarchy_audit
        .iter()
        .filter(|row| row.internal_structural_joint_count > 0)
        .count();
    let hierarchy_internal_structural_joints_total = snapshot
        .notation_internal_hierarchy_audit
        .iter()
        .map(|row| row.internal_structural_joint_count)
        .sum::<usize>();
    let hierarchy_max_internal_structural_joints = snapshot
        .notation_internal_hierarchy_audit
        .iter()
        .map(|row| row.internal_structural_joint_count)
        .max()
        .unwrap_or(0);
    let hierarchy_max_projection_attacks = snapshot
        .notation_internal_hierarchy_audit
        .iter()
        .map(|row| row.projection_attack_count)
        .max()
        .unwrap_or(0);
    let hierarchy_max_measure_span = snapshot
        .notation_internal_hierarchy_audit
        .iter()
        .map(|row| row.measure_span_inclusive)
        .max()
        .unwrap_or(0);
    let hierarchy_projection_adds_closure_arrival = snapshot
        .notation_internal_hierarchy_audit
        .iter()
        .filter(|row| {
            row.projection_attack_count == row.structural_sample_attack_count.saturating_add(1)
                && row.projection_end.canonical_start > row.structural_sample_end.canonical_start
        })
        .count();
    out.push_format_args(format_args!(
        "PhraseNotationInternalHierarchyAudit: PendingVisualRows {}; RowsWithInternalStructuralJoints {}; InternalStructuralJointsTotal {}; MaxInternalStructuralJointsPerProjection {}; MaxProjectionAttacks {}; MaxMeasureSpanInclusive {}; ProjectionAddsClosureArrivalRows {}; SystemSpan unavailable_pre_layout_owner; StaffBoundIdentity ja_current_part_staff_voice_lane; PhraseMarkSemanticCandidate unclassified; LegatoSemanticCandidate unclassified; NotationPolicyDecision unchanged_pending_visual; MutationAuthority none.\n",
        snapshot.notation_internal_hierarchy_audit.len(),
        hierarchy_rows_with_internal_structural_joints,
        hierarchy_internal_structural_joints_total,
        hierarchy_max_internal_structural_joints,
        hierarchy_max_projection_attacks,
        hierarchy_max_measure_span,
        hierarchy_projection_adds_closure_arrival,
    ));
    let intrinsic_rows_with_exact_pending_matches = snapshot
        .notation_intrinsic_span_signature_audit
        .iter()
        .filter(|row| row.exact_pending_span_match_count > 0)
        .count();
    let intrinsic_rows_with_same_lane_pending_matches = snapshot
        .notation_intrinsic_span_signature_audit
        .iter()
        .filter(|row| row.same_lane_exact_pending_span_match_count > 0)
        .count();
    let intrinsic_rows_with_cross_lane_pending_matches = snapshot
        .notation_intrinsic_span_signature_audit
        .iter()
        .filter(|row| row.cross_lane_exact_pending_span_match_count > 0)
        .count();
    let intrinsic_signature_unknown_rows = snapshot
        .notation_intrinsic_span_signature_audit
        .iter()
        .filter(|row| row.intrinsic_span_signature.is_none())
        .count();
    let intrinsic_max_exact_pending_matches = snapshot
        .notation_intrinsic_span_signature_audit
        .iter()
        .map(|row| row.exact_pending_span_match_count)
        .max()
        .unwrap_or(0);
    out.push_format_args(format_args!(
        "PhraseNotationIntrinsicSpanSignatureAudit: PendingVisualRows {}; IntrinsicSignatureKnownRows {}; IntrinsicSignatureUnknownRows {}; RowsWithExactPendingSpanMatches {}; RowsWithSameLaneExactPendingSpanMatches {}; RowsWithCrossLaneExactPendingSpanMatches {}; MaxExactPendingSpanMatches {}; SearchSpace pending_visual_only; OutsideContextUsed nein; AllWindowSearch nein; Similarity none; Threshold none; PositiveUnityDecision unclassified; PhraseMarkSemanticCandidate unclassified; NotationPolicyDecision unchanged_pending_visual; MutationAuthority none.\n",
        snapshot.notation_intrinsic_span_signature_audit.len(),
        snapshot
            .notation_intrinsic_span_signature_audit
            .len()
            .saturating_sub(intrinsic_signature_unknown_rows),
        intrinsic_signature_unknown_rows,
        intrinsic_rows_with_exact_pending_matches,
        intrinsic_rows_with_same_lane_pending_matches,
        intrinsic_rows_with_cross_lane_pending_matches,
        intrinsic_max_exact_pending_matches,
    ));
    let all_window_rows_with_exact_matches = snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .iter()
        .filter(|row| row.exact_all_window_match_count > 0)
        .count();
    let all_window_rows_with_same_lane_matches = snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .iter()
        .filter(|row| row.same_lane_exact_all_window_match_count > 0)
        .count();
    let all_window_rows_with_cross_lane_matches = snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .iter()
        .filter(|row| row.cross_lane_exact_all_window_match_count > 0)
        .count();
    let all_window_rows_with_non_pending_matches = snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .iter()
        .filter(|row| row.exact_non_pending_window_match_count > 0)
        .count();
    let all_window_max_exact_matches = snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .iter()
        .map(|row| row.exact_all_window_match_count)
        .max()
        .unwrap_or(0);
    let all_window_max_non_pending_matches = snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .iter()
        .map(|row| row.exact_non_pending_window_match_count)
        .max()
        .unwrap_or(0);
    out.push_format_args(format_args!(
        "PhraseNotationIntrinsicAllWindowParallelismAudit: PendingVisualRows {}; RowsWithExactAllWindowMatches {}; RowsWithSameLaneExactAllWindowMatches {}; RowsWithCrossLaneExactAllWindowMatches {}; RowsWithExactNonPendingWindowMatches {}; MaxExactAllWindowMatches {}; MaxExactNonPendingWindowMatches {}; SearchSpace all_same_attack_count_windows_in_piece; SourceWindowExcluded ja; OutsideContextUsed nein_signature_only; MatchedWindowBoundaryRoles diagnostic_only; Similarity none; Threshold none; SelectionBiasDecision audit_only; PositiveUnityDecision unclassified; PhraseMarkSemanticCandidate unclassified; NotationPolicyDecision unchanged_pending_visual; MutationAuthority none.\n",
        snapshot.notation_intrinsic_all_window_parallelism_audit.len(),
        all_window_rows_with_exact_matches,
        all_window_rows_with_same_lane_matches,
        all_window_rows_with_cross_lane_matches,
        all_window_rows_with_non_pending_matches,
        all_window_max_exact_matches,
        all_window_max_non_pending_matches,
    ));
    let closure_row_count = snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .len()
        .max(snapshot.notation_intrinsic_span_signature_audit.len())
        .max(snapshot.notation_higher_level_unity_audit.len());
    let mut closure_alignment_mismatches = 0usize;
    let mut closure_legacy_exact_rows = 0usize;
    let mut closure_intrinsic_pending_exact_rows = 0usize;
    let mut closure_intrinsic_all_window_exact_rows = 0usize;
    let mut closure_intrinsic_all_window_zero_rows = 0usize;
    let mut closure_intrinsic_all_window_single_match_rows = 0usize;
    let mut closure_intrinsic_all_window_multiple_match_rows = 0usize;
    let mut closure_cross_lane_intrinsic_rows = 0usize;
    let mut closure_same_and_cross_lane_intrinsic_rows = 0usize;
    let mut closure_legacy_descriptor_unknown_rows = 0usize;
    let mut closure_intrinsic_descriptor_unknown_rows = 0usize;
    let mut closure_legacy_only_rows = 0usize;
    let mut closure_intrinsic_only_rows = 0usize;
    let mut closure_both_legacy_and_intrinsic_rows = 0usize;
    let mut closure_exact_absent_across_models_rows = 0usize;
    let mut closure_selection_bias_exposed_rows = 0usize;
    let mut closure_non_pending_evidence_rows = 0usize;
    let mut closure_total_all_window_matches = 0usize;
    let mut closure_total_non_pending_window_matches = 0usize;
    let mut closure_sampled_match_windows = 0usize;
    let mut closure_sampled_non_pending_match_windows = 0usize;
    let mut closure_sampled_start_lane_start = 0usize;
    let mut closure_sampled_start_closure_supported = 0usize;
    let mut closure_sampled_start_structural_joint = 0usize;
    let mut closure_sampled_start_continuation_supported = 0usize;
    let mut closure_sampled_start_conflict = 0usize;
    let mut closure_sampled_start_unresolved = 0usize;
    let mut closure_sampled_start_other = 0usize;
    let mut closure_sampled_end_lane_start = 0usize;
    let mut closure_sampled_end_closure_supported = 0usize;
    let mut closure_sampled_end_structural_joint = 0usize;
    let mut closure_sampled_end_continuation_supported = 0usize;
    let mut closure_sampled_end_conflict = 0usize;
    let mut closure_sampled_end_unresolved = 0usize;
    let mut closure_sampled_end_other = 0usize;

    for index in 0..closure_row_count {
        let Some(pending_row) = snapshot.notation_intrinsic_span_signature_audit.get(index) else {
            closure_alignment_mismatches = closure_alignment_mismatches.saturating_add(1);
            continue;
        };
        let Some(all_window_row) = snapshot
            .notation_intrinsic_all_window_parallelism_audit
            .get(index)
        else {
            closure_alignment_mismatches = closure_alignment_mismatches.saturating_add(1);
            continue;
        };
        let Some(legacy_row) = snapshot.notation_higher_level_unity_audit.get(index) else {
            closure_alignment_mismatches = closure_alignment_mismatches.saturating_add(1);
            continue;
        };
        let pending_all_window_identity_equal = pending_row.lane == all_window_row.lane
            && pending_row.projection_start.canonical_start
                == all_window_row.projection_start.canonical_start
            && pending_row.projection_end.canonical_start
                == all_window_row.projection_end.canonical_start
            && pending_row.projection_attack_count == all_window_row.projection_attack_count;
        let pending_legacy_identity_equal = pending_row.lane == legacy_row.lane
            && pending_row.projection_start.canonical_start
                == legacy_row.projection_start.canonical_start
            && pending_row.projection_end.canonical_start
                == legacy_row.projection_end.canonical_start
            && pending_row.projection_attack_count == legacy_row.projection_attack_count;
        if !pending_all_window_identity_equal || !pending_legacy_identity_equal {
            closure_alignment_mismatches = closure_alignment_mismatches.saturating_add(1);
            continue;
        }

        let legacy_exact = legacy_row.exact_span_parallel_match_count > 0;
        let intrinsic_pending_exact = pending_row.exact_pending_span_match_count > 0;
        let intrinsic_all_window_exact = all_window_row.exact_all_window_match_count > 0;
        if legacy_row.span_parallel_signature.is_none() {
            closure_legacy_descriptor_unknown_rows =
                closure_legacy_descriptor_unknown_rows.saturating_add(1);
        }
        if pending_row.intrinsic_span_signature.is_none()
            || all_window_row.intrinsic_span_signature.is_none()
        {
            closure_intrinsic_descriptor_unknown_rows =
                closure_intrinsic_descriptor_unknown_rows.saturating_add(1);
        }
        match all_window_row.exact_all_window_match_count {
            0 => {
                closure_intrinsic_all_window_zero_rows =
                    closure_intrinsic_all_window_zero_rows.saturating_add(1);
            }
            1 => {
                closure_intrinsic_all_window_single_match_rows =
                    closure_intrinsic_all_window_single_match_rows.saturating_add(1);
            }
            _ => {
                closure_intrinsic_all_window_multiple_match_rows =
                    closure_intrinsic_all_window_multiple_match_rows.saturating_add(1);
            }
        }
        if all_window_row.cross_lane_exact_all_window_match_count > 0 {
            closure_cross_lane_intrinsic_rows =
                closure_cross_lane_intrinsic_rows.saturating_add(1);
        }
        if all_window_row.same_lane_exact_all_window_match_count > 0
            && all_window_row.cross_lane_exact_all_window_match_count > 0
        {
            closure_same_and_cross_lane_intrinsic_rows =
                closure_same_and_cross_lane_intrinsic_rows.saturating_add(1);
        }
        if legacy_exact {
            closure_legacy_exact_rows = closure_legacy_exact_rows.saturating_add(1);
        }
        if intrinsic_pending_exact {
            closure_intrinsic_pending_exact_rows =
                closure_intrinsic_pending_exact_rows.saturating_add(1);
        }
        if intrinsic_all_window_exact {
            closure_intrinsic_all_window_exact_rows =
                closure_intrinsic_all_window_exact_rows.saturating_add(1);
        }
        match (legacy_exact, intrinsic_all_window_exact) {
            (true, false) => {
                closure_legacy_only_rows = closure_legacy_only_rows.saturating_add(1);
            }
            (false, true) => {
                closure_intrinsic_only_rows = closure_intrinsic_only_rows.saturating_add(1);
            }
            (true, true) => {
                closure_both_legacy_and_intrinsic_rows =
                    closure_both_legacy_and_intrinsic_rows.saturating_add(1);
            }
            (false, false) => {
                closure_exact_absent_across_models_rows =
                    closure_exact_absent_across_models_rows.saturating_add(1);
            }
        }
        if all_window_row.exact_all_window_match_count > pending_row.exact_pending_span_match_count {
            closure_selection_bias_exposed_rows =
                closure_selection_bias_exposed_rows.saturating_add(1);
        }
        if all_window_row.exact_non_pending_window_match_count > 0 {
            closure_non_pending_evidence_rows =
                closure_non_pending_evidence_rows.saturating_add(1);
        }
        closure_total_all_window_matches = closure_total_all_window_matches
            .saturating_add(all_window_row.exact_all_window_match_count);
        closure_total_non_pending_window_matches = closure_total_non_pending_window_matches
            .saturating_add(all_window_row.exact_non_pending_window_match_count);

        for sample in &all_window_row.match_samples {
            closure_sampled_match_windows = closure_sampled_match_windows.saturating_add(1);
            if !sample.is_pending {
                closure_sampled_non_pending_match_windows =
                    closure_sampled_non_pending_match_windows.saturating_add(1);
            }
            match sample.start_role {
                "lane_start" => {
                    closure_sampled_start_lane_start =
                        closure_sampled_start_lane_start.saturating_add(1);
                }
                "closure_supported" => {
                    closure_sampled_start_closure_supported =
                        closure_sampled_start_closure_supported.saturating_add(1);
                }
                "structural_joint" => {
                    closure_sampled_start_structural_joint =
                        closure_sampled_start_structural_joint.saturating_add(1);
                }
                "continuation_supported" => {
                    closure_sampled_start_continuation_supported =
                        closure_sampled_start_continuation_supported.saturating_add(1);
                }
                "conflict" => {
                    closure_sampled_start_conflict =
                        closure_sampled_start_conflict.saturating_add(1);
                }
                "unresolved" => {
                    closure_sampled_start_unresolved =
                        closure_sampled_start_unresolved.saturating_add(1);
                }
                _ => {
                    closure_sampled_start_other =
                        closure_sampled_start_other.saturating_add(1);
                }
            }
            match sample.end_role {
                "lane_start" => {
                    closure_sampled_end_lane_start =
                        closure_sampled_end_lane_start.saturating_add(1);
                }
                "closure_supported" => {
                    closure_sampled_end_closure_supported =
                        closure_sampled_end_closure_supported.saturating_add(1);
                }
                "structural_joint" => {
                    closure_sampled_end_structural_joint =
                        closure_sampled_end_structural_joint.saturating_add(1);
                }
                "continuation_supported" => {
                    closure_sampled_end_continuation_supported =
                        closure_sampled_end_continuation_supported.saturating_add(1);
                }
                "conflict" => {
                    closure_sampled_end_conflict =
                        closure_sampled_end_conflict.saturating_add(1);
                }
                "unresolved" => {
                    closure_sampled_end_unresolved =
                        closure_sampled_end_unresolved.saturating_add(1);
                }
                _ => {
                    closure_sampled_end_other =
                        closure_sampled_end_other.saturating_add(1);
                }
            }
        }
    }
    out.push_format_args(format_args!(
        "PhraseNotationIntrinsicParallelismCorpusClosureAudit: PendingVisualRows {}; RowAlignmentMismatches {}; RowAlignmentConsistent {}; LegacyDescriptorUnknownRows {}; IntrinsicDescriptorUnknownRows {}; LegacyWholeSpanExactRows {}; IntrinsicPendingExactRows {}; IntrinsicAllWindowExactRows {}; IntrinsicAllWindowDistribution zero:{} one:{} multiple:{}; CrossLaneIntrinsicRows {}; SameAndCrossLaneIntrinsicRows {}; LegacyOnlyExactRows {}; IntrinsicOnlyExactRows {}; BothLegacyAndIntrinsicExactRows {}; ExactAbsentAcrossLegacyAndIntrinsicRows {}; SelectionBiasExposedRows {}; RowsWithNonPendingExactEvidence {}; TotalExactAllWindowMatches {}; TotalExactNonPendingWindowMatches {}; SampledMatchedWindows {}; SampledNonPendingMatchedWindows {}; SampledStartRoles lane_start:{} closure_supported:{} structural_joint:{} continuation_supported:{} conflict:{} unresolved:{} other:{}; SampledEndRoles lane_start:{} closure_supported:{} structural_joint:{} continuation_supported:{} conflict:{} unresolved:{} other:{}; RecognitionChanged nein; SignatureChanged nein; SearchSpaceChanged nein; BoundaryRoles diagnostic_only; CorpusDecision deferred_until_all_projects; StructuralVariationDecision not_taken; PositiveUnityDecision unclassified; NotationPolicyDecision unchanged_pending_visual; MutationAuthority none.\n",
        closure_row_count,
        closure_alignment_mismatches,
        yes_no(closure_alignment_mismatches == 0),
        closure_legacy_descriptor_unknown_rows,
        closure_intrinsic_descriptor_unknown_rows,
        closure_legacy_exact_rows,
        closure_intrinsic_pending_exact_rows,
        closure_intrinsic_all_window_exact_rows,
        closure_intrinsic_all_window_zero_rows,
        closure_intrinsic_all_window_single_match_rows,
        closure_intrinsic_all_window_multiple_match_rows,
        closure_cross_lane_intrinsic_rows,
        closure_same_and_cross_lane_intrinsic_rows,
        closure_legacy_only_rows,
        closure_intrinsic_only_rows,
        closure_both_legacy_and_intrinsic_rows,
        closure_exact_absent_across_models_rows,
        closure_selection_bias_exposed_rows,
        closure_non_pending_evidence_rows,
        closure_total_all_window_matches,
        closure_total_non_pending_window_matches,
        closure_sampled_match_windows,
        closure_sampled_non_pending_match_windows,
        closure_sampled_start_lane_start,
        closure_sampled_start_closure_supported,
        closure_sampled_start_structural_joint,
        closure_sampled_start_continuation_supported,
        closure_sampled_start_conflict,
        closure_sampled_start_unresolved,
        closure_sampled_start_other,
        closure_sampled_end_lane_start,
        closure_sampled_end_closure_supported,
        closure_sampled_end_structural_joint,
        closure_sampled_end_continuation_supported,
        closure_sampled_end_conflict,
        closure_sampled_end_unresolved,
        closure_sampled_end_other,
    ));
    let variation_row_count = snapshot
        .notation_structural_variation_factorization_audit
        .len();
    let variation_projection_unknown_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| row.variation_projection.is_none())
        .count();
    let variation_alignment_mismatches = snapshot
        .notation_structural_variation_factorization_audit
        .len()
        .abs_diff(snapshot.notation_intrinsic_all_window_parallelism_audit.len())
        .saturating_add(
            snapshot
                .notation_structural_variation_factorization_audit
                .iter()
                .zip(snapshot.notation_intrinsic_all_window_parallelism_audit.iter())
                .filter(|(variation_row, all_window_row)| {
                    variation_row.lane != all_window_row.lane
                        || variation_row.projection_start.canonical_start
                            != all_window_row.projection_start.canonical_start
                        || variation_row.projection_end.canonical_start
                            != all_window_row.projection_end.canonical_start
                        || variation_row.projection_attack_count
                            != all_window_row.projection_attack_count
                })
                .count(),
        );
    let variation_full_exact_alignment_mismatches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .zip(snapshot.notation_intrinsic_all_window_parallelism_audit.iter())
        .filter(|(variation_row, all_window_row)| {
            variation_row.full_exact_reobserved_match_count
                != all_window_row.exact_all_window_match_count
        })
        .count();
    let variation_full_exact_reobserved_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| row.full_exact_reobserved_match_count > 0)
        .count();
    let variation_texture_relaxed_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| row.texture_relaxed_match_count > 0)
        .count();
    let variation_duration_relaxed_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| row.sounding_duration_relaxed_match_count > 0)
        .count();
    let variation_interval_magnitude_relaxed_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| row.interval_magnitude_relaxed_match_count > 0)
        .count();
    let variation_rhythm_relaxed_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| row.rhythm_relaxed_match_count > 0)
        .count();
    let variation_melody_relaxed_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| row.melody_relaxed_match_count > 0)
        .count();
    let variation_any_relaxed_rows = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .filter(|row| {
            row.texture_relaxed_match_count > 0
                || row.sounding_duration_relaxed_match_count > 0
                || row.interval_magnitude_relaxed_match_count > 0
                || row.rhythm_relaxed_match_count > 0
                || row.melody_relaxed_match_count > 0
        })
        .count();
    let variation_total_full_exact_reobserved_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .map(|row| row.full_exact_reobserved_match_count)
        .sum::<usize>();
    let variation_total_texture_relaxed_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .map(|row| row.texture_relaxed_match_count)
        .sum::<usize>();
    let variation_total_duration_relaxed_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .map(|row| row.sounding_duration_relaxed_match_count)
        .sum::<usize>();
    let variation_total_interval_magnitude_relaxed_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .map(|row| row.interval_magnitude_relaxed_match_count)
        .sum::<usize>();
    let variation_total_rhythm_relaxed_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .map(|row| row.rhythm_relaxed_match_count)
        .sum::<usize>();
    let variation_total_melody_relaxed_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .map(|row| row.melody_relaxed_match_count)
        .sum::<usize>();
    let variation_sampled_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .map(|row| row.match_samples.len())
        .sum::<usize>();
    let variation_sampled_pending_matches = snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .flat_map(|row| row.match_samples.iter())
        .filter(|sample| sample.is_pending)
        .count();
    out.push_format_args(format_args!(
        "PhraseNotationStructuralVariationFactorizationAudit: PendingVisualRows {}; RowAlignmentMismatches {}; RowAlignmentConsistent {}; ProjectionUnknownRows {}; FullExactReobservedRows {}; FullExactAlignmentMismatches {}; RowsWithTextureOnlyRelaxedMatches {}; RowsWithSoundingDurationOnlyRelaxedMatches {}; RowsWithIntervalMagnitudeOnlyRelaxedMatches {}; RowsWithRhythmDimensionRelaxedMatches {}; RowsWithMelodyDimensionRelaxedMatches {}; RowsWithAnyRelaxedProjectionMatch {}; TotalFullExactReobservedMatches {}; TotalTextureOnlyRelaxedMatches {}; TotalSoundingDurationOnlyRelaxedMatches {}; TotalIntervalMagnitudeOnlyRelaxedMatches {}; TotalRhythmDimensionRelaxedMatches {}; TotalMelodyDimensionRelaxedMatches {}; SampledRelaxedMatches {}; SampledPendingRelaxedMatches {}; SearchSpace all_same_attack_count_windows_in_piece; ProjectionSemantics exact_information_drop_only; TranspositionInvariant ja_existing_centroid_deltas; UniformAugmentationDiminutionInvariant ja_existing_full_rhythm; Similarity none; Threshold none; Scalarization none; EditDistance none; StructuralVariationDecision diagnostic_only; PositiveUnityDecision unclassified; NotationPolicyDecision unchanged_pending_visual; MutationAuthority none.\n",
        variation_row_count,
        variation_alignment_mismatches,
        yes_no(variation_alignment_mismatches == 0),
        variation_projection_unknown_rows,
        variation_full_exact_reobserved_rows,
        variation_full_exact_alignment_mismatches,
        variation_texture_relaxed_rows,
        variation_duration_relaxed_rows,
        variation_interval_magnitude_relaxed_rows,
        variation_rhythm_relaxed_rows,
        variation_melody_relaxed_rows,
        variation_any_relaxed_rows,
        variation_total_full_exact_reobserved_matches,
        variation_total_texture_relaxed_matches,
        variation_total_duration_relaxed_matches,
        variation_total_interval_magnitude_relaxed_matches,
        variation_total_rhythm_relaxed_matches,
        variation_total_melody_relaxed_matches,
        variation_sampled_matches,
        variation_sampled_pending_matches,
    ));
    let unity_rows_all_internal_continuation = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.all_internal_boundaries_continuation_supported)
        .count();
    let unity_rows_with_exact_span_parallelism = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.exact_span_parallel_match_count > 0)
        .count();
    let unity_rows_with_same_lane_exact_span_parallelism = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.same_lane_exact_span_parallel_match_count > 0)
        .count();
    let unity_rows_with_cross_lane_exact_span_parallelism = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.cross_lane_exact_span_parallel_match_count > 0)
        .count();
    let unity_rows_with_same_role_exact_span_parallelism = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.exact_span_parallel_same_role_match_count > 0)
        .count();
    let unity_rows_with_divergent_role_exact_span_parallelism = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.exact_span_parallel_divergent_role_match_count > 0)
        .count();
    let unity_rows_with_terminal_exact_joint_repeat = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.terminal_exact_joint_match_count > 0)
        .count();
    let unity_span_descriptor_unknown_rows = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .filter(|row| row.span_parallel_signature.is_none())
        .count();
    let unity_max_exact_span_parallel_matches = snapshot
        .notation_higher_level_unity_audit
        .iter()
        .map(|row| row.exact_span_parallel_match_count)
        .max()
        .unwrap_or(0);
    out.push_format_args(format_args!(
        "PhraseNotationHigherLevelUnityAudit: PendingVisualRows {}; RowsAllInternalContinuationSupported {}; RowsWithExactWholeSpanParallelism {}; RowsWithSameLaneExactWholeSpanParallelism {}; RowsWithCrossLaneExactWholeSpanParallelism {}; RowsWithSameRoleExactWholeSpanParallelism {}; RowsWithDivergentRoleExactWholeSpanParallelism {}; RowsWithTerminalExactJointRepeat {}; SpanParallelDescriptorUnknownRows {}; MaxExactWholeSpanParallelMatches {}; PositiveUnityDecision unclassified; PhraseMarkSemanticCandidate unclassified; NotationPolicyDecision unchanged_pending_visual; Scalarization none; MutationAuthority none.\n",
        snapshot.notation_higher_level_unity_audit.len(),
        unity_rows_all_internal_continuation,
        unity_rows_with_exact_span_parallelism,
        unity_rows_with_same_lane_exact_span_parallelism,
        unity_rows_with_cross_lane_exact_span_parallelism,
        unity_rows_with_same_role_exact_span_parallelism,
        unity_rows_with_divergent_role_exact_span_parallelism,
        unity_rows_with_terminal_exact_joint_repeat,
        unity_span_descriptor_unknown_rows,
        unity_max_exact_span_parallel_matches,
    ));
    out.push_format_args(format_args!(
        "ParallelRoleAudit: ExactRepeatedGroupsUniformRole {}; ExactRepeatedGroupsDivergentRole {}; DivergentTonalContext {}; DivergentMetricStructuralContext {}; DivergentContinuityContext {}; DivergentCompoundContext {}; DivergentUnexplained {}; WriterPreconditionUnexplainedZero {}; diagnostic_only; explained_divergence_is_context_not_write_authority.\n",
        snapshot.parallel_role_uniform_groups,
        snapshot.parallel_role_divergent_groups,
        snapshot.parallel_role_divergent_tonal_context,
        snapshot.parallel_role_divergent_metric_structural_context,
        snapshot.parallel_role_divergent_continuity_context,
        snapshot.parallel_role_divergent_compound_context,
        snapshot.parallel_role_divergent_unexplained,
        yes_no(snapshot.parallel_role_divergent_unexplained == 0),
    ));
    out.push_format_args(format_args!(
        "EvidenceDiversity: families_observed_0 {}; families_observed_1 {}; families_observed_2 {}; families_observed_3 {}; families_observed_4 {}; families_observed_5 {}; families_observed_6 {}; families_observed_7 {}.\n",
        snapshot.diversity_counts[0],
        snapshot.diversity_counts[1],
        snapshot.diversity_counts[2],
        snapshot.diversity_counts[3],
        snapshot.diversity_counts[4],
        snapshot.diversity_counts[5],
        snapshot.diversity_counts[6],
        snapshot.diversity_counts[7],
    ));
    let density_per_100 = snapshot
        .boundary_count
        .saturating_mul(100)
        .checked_div(snapshot.attack_count)
        .unwrap_or(0);
    out.push_format_args(format_args!(
        "ObservationDensityAudit: PositionsPer100TrueAttacks {}; diagnostic_only; target_range none; ground_truth none; PromotedBoundaryCandidates 0; PauseGeneratedPositions 0.\n",
        density_per_100,
    ));
    out.push_str("TiePolicy: tie-stop-only onset is never a new attack; tied sounding continuation extends canonical continuity across measure boundaries when exact metric binding is available.\n");
    out.push_str("CrossMeasurePolicy: canonical cross-measure rhythm is derived from the existing parsed Score::Measure divisions/beats/beat_type/ExpectedDuration owner; no parallel meter truth and no barline boundary verdict.\n");
    out.push_str("MetricPolicy: exact measure offset, beat-grid position and beat phase are descriptors only; same_measure_vs_cross_measure remains context only; barline and downbeat do not create support by themselves.\n");
    out.push_str("MelodicPolicy: contour direction and interval relations are local relative descriptors; no absolute leap threshold, register threshold or melodic boundary verdict exists.\n");
    out.push_str("ParallelismPolicy: repeated local patterns are detected only by exact piece-internal transposition-invariant melodic shape plus normalized rhythmic ratio shape; divergent roles retain named tonal, metric/structural, continuity or compound context reasons; unexplained divergence blocks SlurIntent authorization.\n");
    out.push_str("HigherLevelUnityAuditPolicy: exact whole-span parallelism is derived only among the existing pending-visual spans and only when every boundary from projection start through terminal closure already has the existing exact joint descriptor; the complete ordered descriptor sequence is compared without similarity score, edit distance, threshold or fuzzy matching; matching span role sequences are reported separately from divergent roles; internal continuation support remains an independent named witness; none of these observations grants notation authority.\n");
    out.push_str("IntrinsicSpanSignatureAuditPolicy: exact intrinsic pending-span parallelism uses only attacks from projection start through projection end; rhythm uses internal IOIs plus sounding durations normalized together by one exact positive GCD; melody uses only internal centroid deltas plus the attack chord-member sequence; no previous or following outside attack enters the descriptor; r565 compares only the existing pending-visual spans and deliberately performs no all-window search, no similarity score, no edit distance, no threshold, no fuzzy matching and no notation promotion.\n");
    out.push_str("IntrinsicAllWindowParallelismAuditPolicy: r566 reuses the unchanged r565 intrinsic exact signature and compares each existing pending-visual span against every other same-attack-count contiguous true-attack window in every existing part/staff/voice lane of the same piece; the source window itself is excluded; matching-window start/end boundary roles and pending membership are diagnostic context only and do not enter the signature or grant PhraseMark, SlurIntent, writer or mutation authority; no similarity score, edit distance, threshold, fuzzy matching or notation promotion exists.\n");
    out.push_str("StructuralVariationFactorizationAuditPolicy: r569 does not define musical similarity. It reuses the same pending spans and same-attack-count all-window search, then compares exact projections of the already-observed intrinsic data: texture-only relaxation keeps full rhythm and exact centroid motion while dropping chord-member sequence; sounding-duration-only relaxation keeps independently normalized onset-IOI ratios, exact centroid motion and chord-member sequence while dropping sounding-duration shape; interval-magnitude-only relaxation keeps full rhythm, exact up/same/down contour and chord-member sequence while dropping centroid-delta magnitudes; rhythm-dimension relaxation keeps exact centroid motion plus chord-member sequence; melody-dimension relaxation keeps the full existing rhythm signature. Full-exact matches are reobserved only as an integrity cross-check against r566. No projection match promotes a phrase, slur or notation decision.\n");
    out.push_str("StructuralNoveltyPolicy: local rhythmic context, chord texture and score-wide activity/density state changes are reported independently; no weighted novelty score exists.\n");
    out.push_str("TonalClosurePolicy: reuses only the existing score-wide tonal-evidence owner; raw tonic/dominant, bass-arrival and leading-tone evidence remain explicit even when profile reliability is low; phrase closure authority requires the existing owner profile-green margin criterion; no local-key claim, Roman-numeral analysis or cadence label.\n");
    out.push_str("PhraseStructurePolicy: qualitative conjunctions only; closure requires tonal closure plus an explicit structural gate; structural joints and continuation remain separate; relative grouping levels are audit structure, not notation authority.\n");
    out.push_str("NotationPolicy: precision_first; default_no_slur; only a closure-ending local group may receive allow_structural_pending_visual; visual feasibility and anchor resolution remain mandatory in the later writer take.\n");
    out.push_str("VoicePolicy: no voice collapse; no majority vote; score regions preserve closure lanes and independently continuing context.\n");
    out.push_str("Scalarization: nein; PhraseConfidence none; weighted_magic_score none; probability none; new_numeric_phrase_threshold none; existing_tonal_owner_reliability_gate reused.\n");

    for (index, row) in snapshot.samples.iter().enumerate() {
        out.push_format_args(format_args!(
            "BoundaryObservation[{index:03}]: part {} | staff {} | voice {} | left m_ord {} onset {} dur {} sounding_end {} canonical_start {} canonical_sounding_end {} chord_members {} tied_continuations_at_onset {} | right m_ord {} onset {} dur {} canonical_start {} chord_members {} tied_continuations_at_onset {} | rhythm support {} oppose {} gap_canonical_ticks {} explicit_rest {} | metric context {} meter {}/{} measure_offset_ticks {} beat_index_zero_based {} beat_phase_ticks {} on_measure_start {} on_beat_grid {} | melodic nearest_semitones {} centroid_delta_x100 {} previous_delta_x100 {} following_delta_x100 {} previous_interval {} following_interval {} contour_into {} contour_out {} boundary_interval_relation {} pitch_overlap {} | parallelism rhythm_signature {} melodic_signature {} exact_joint_matches {} same_lane_matches {} cross_lane_matches {} role_divergence_reason {} | novelty previous_ioi_ticks {} following_ioi_ticks {} rhythmic_context_change {} chord_texture_change {} active_lanes {}->{} onset_lanes {}->{} one_beat_attack_density {}->{} score_active_lane_change {} score_onset_multiplicity_change {} score_density_change {} | continuity_oppose {} | tonal profile_reliable {} tonic_pc {} dominant_pc {} leading_tone_pc {} left_tonic {} left_dominant {} right_tonic {} right_dominant {} score_tonic_arrival {} score_dominant_arrival {} bass_before_pc {} bass_arrival_pc {} bass_arrival_to_tonic {} bass_dominant_to_tonic {} leading_tone_resolution {} unresolved_leading_tone {} closure_support_raw {} anti_closure_raw {} closure_authorized {} anti_closure_authorized {} | phrase melodic_joint {} closure_structure_gate {} local_joint {} continuation_support {} role {} | observed_families {}.\n",
            row.lane.part,
            row.lane.staff,
            row.lane.voice,
            row.left.measure_ordinal,
            row.left.start,
            row.left.duration,
            row.left.sounding_end,
            row.left.canonical_start,
            row.left.canonical_sounding_end,
            row.left.chord_members,
            row.left.tied_continuation_members_at_onset,
            row.right.measure_ordinal,
            row.right.start,
            row.right.duration,
            row.right.canonical_start,
            row.right.chord_members,
            row.right.tied_continuation_members_at_onset,
            yes_no(row.rhythmic_support),
            yes_no(row.rhythmic_oppose),
            row.temporal_gap,
            yes_no(row.explicit_rest_between),
            if row.same_measure {
                "same_measure"
            } else {
                "cross_measure_resolved"
            },
            row.right.measure_beats,
            row.right.measure_beat_type,
            optional_i128(row.metric_measure_offset_ticks),
            optional_i64(row.metric_beat_index_zero_based),
            optional_i128(row.metric_beat_phase_ticks),
            yes_no(row.metric_on_measure_start),
            yes_no(row.metric_on_beat_grid),
            optional_i32(row.nearest_pitch_distance),
            optional_i64(row.register_centroid_delta_x100),
            optional_i64(row.melodic_previous_delta_x100),
            optional_i64(row.melodic_following_delta_x100),
            optional_i32(row.melodic_previous_interval),
            optional_i32(row.melodic_following_interval),
            optional_str(row.melodic_contour_into_boundary),
            optional_str(row.melodic_contour_out_of_boundary),
            optional_str(row.melodic_boundary_interval_relation),
            yes_no(row.melodic_pitch_overlap),
            optional_parallel_rhythm_signature(row.parallel_rhythm_signature),
            optional_parallel_melodic_signature(row.parallel_melodic_signature),
            row.parallel_exact_joint_match_count,
            row.parallel_same_lane_match_count,
            row.parallel_cross_lane_match_count,
            optional_str(row.parallel_role_divergence_reason),
            optional_i128(row.novelty_previous_ioi_ticks),
            optional_i128(row.novelty_following_ioi_ticks),
            yes_no(row.novelty_rhythmic_context_change),
            yes_no(row.novelty_chord_texture_change),
            row.novelty_active_lanes_before,
            row.novelty_active_lanes_after,
            row.novelty_onset_lanes_left,
            row.novelty_onset_lanes_right,
            row.novelty_density_left_beat,
            row.novelty_density_right_beat,
            yes_no(row.novelty_score_active_lane_change),
            yes_no(row.novelty_score_onset_multiplicity_change),
            yes_no(row.novelty_score_density_change),
            yes_no(row.continuity_oppose),
            yes_no(row.tonal_profile_reliable),
            optional_i32(row.tonal_tonic_pc),
            optional_i32(row.tonal_dominant_pc),
            optional_i32(row.tonal_leading_tone_pc),
            yes_no(row.tonal_left_attack_tonic),
            yes_no(row.tonal_left_attack_dominant),
            yes_no(row.tonal_right_attack_tonic),
            yes_no(row.tonal_right_attack_dominant),
            yes_no(row.tonal_score_tonic_at_arrival),
            yes_no(row.tonal_score_dominant_at_arrival),
            optional_i32(row.tonal_bass_before_pc),
            optional_i32(row.tonal_bass_arrival_pc),
            yes_no(row.tonal_bass_arrival_to_tonic),
            yes_no(row.tonal_bass_dominant_to_tonic),
            yes_no(row.tonal_lane_leading_tone_resolution),
            yes_no(row.tonal_lane_unresolved_leading_tone),
            yes_no(row.tonal_closure_support),
            yes_no(row.tonal_anti_closure),
            yes_no(row.tonal_closure_authorized),
            yes_no(row.tonal_anti_closure_authorized),
            yes_no(row.phrase_melodic_joint),
            yes_no(row.phrase_closure_structure_gate),
            yes_no(row.phrase_local_joint),
            yes_no(row.phrase_continuation_support),
            row.phrase_boundary_role,
            row.observed_family_count,
        ));
    }

    for (index, span) in snapshot.phrase_span_samples.iter().enumerate() {
        out.push_format_args(format_args!(
            "PhraseSpanSample[{index:03}]: part {} | staff {} | voice {} | grouping_level 1 | start m_ord {} onset {} canonical_start {} | end m_ord {} onset {} canonical_start {} | attacks {} | internal_level0_groups {} | status {} | notation_authority none.\n",
            span.lane.part,
            span.lane.staff,
            span.lane.voice,
            span.start.measure_ordinal,
            span.start.start,
            span.start.canonical_start,
            span.end.measure_ordinal,
            span.end.start,
            span.end.canonical_start,
            span.attack_count,
            span.internal_local_groups,
            span.status,
        ));
    }
    for (index, intent) in snapshot.slur_intent_samples.iter().enumerate() {
        out.push_format_args(format_args!(
            "SlurIntentSample[{index:03}]: part {} | staff {} | voice {} | grouping_level 0 | start m_ord {} onset {} canonical_start {} | end m_ord {} onset {} canonical_start {} | attacks {} | intent {} | reason {} | AnchorResolver none | WriterOpen nein.\n",
            intent.lane.part,
            intent.lane.staff,
            intent.lane.voice,
            intent.start.measure_ordinal,
            intent.start.start,
            intent.start.canonical_start,
            intent.end.measure_ordinal,
            intent.end.start,
            intent.end.canonical_start,
            intent.attack_count,
            intent.intent,
            intent.reason,
        ));
    }
    for (index, row) in snapshot.notation_internal_hierarchy_audit.iter().enumerate() {
        out.push_format_args(format_args!(
            "PhraseNotationInternalHierarchy[{index:03}]: part {} | staff {} | voice {} | projection_start m_ord {} onset {} canonical_start {} | projection_end m_ord {} onset {} canonical_start {} | projection_attacks {} | legacy_structural_sample_end m_ord {} onset {} canonical_start {} | legacy_structural_sample_attacks {} | projection_adds_closure_arrival {} | measure_span_inclusive {} | internal_boundaries {} structural_joint {} closure_structure_gate {} continuation_supported {} conflict {} unresolved {} unexplained_parallel_divergence {} | terminal_role {} | SystemSpan unavailable_pre_layout_owner | StaffBoundIdentity ja | PhraseMarkSemanticCandidate unclassified | LegatoSemanticCandidate unclassified | NotationPolicyDecision unchanged_pending_visual | WriterOpen nein.\n",
            row.lane.part,
            row.lane.staff,
            row.lane.voice,
            row.projection_start.measure_ordinal,
            row.projection_start.start,
            row.projection_start.canonical_start,
            row.projection_end.measure_ordinal,
            row.projection_end.start,
            row.projection_end.canonical_start,
            row.projection_attack_count,
            row.structural_sample_end.measure_ordinal,
            row.structural_sample_end.start,
            row.structural_sample_end.canonical_start,
            row.structural_sample_attack_count,
            yes_no(
                row.projection_attack_count
                    == row.structural_sample_attack_count.saturating_add(1)
                    && row.projection_end.canonical_start > row.structural_sample_end.canonical_start
            ),
            row.measure_span_inclusive,
            row.internal_boundary_count,
            row.internal_structural_joint_count,
            row.internal_closure_structure_gate_count,
            row.internal_continuation_supported_count,
            row.internal_conflict_count,
            row.internal_unresolved_count,
            row.internal_unexplained_parallel_divergence_count,
            row.terminal_role,
        ));
    }
    for (index, row) in snapshot
        .notation_intrinsic_span_signature_audit
        .iter()
        .enumerate()
    {
        out.push_format_args(format_args!(
            "PhraseNotationIntrinsicSpanSignature[{index:03}]: part {} | staff {} | voice {} | projection_start m_ord {} onset {} canonical_start {} | projection_end m_ord {} onset {} canonical_start {} | projection_attacks {} | intrinsic_descriptor {} | exact_pending_matches {} same_lane_pending_matches {} cross_lane_pending_matches {} | OutsideContextUsed nein | SearchSpace pending_visual_only | AllWindowSearch nein | PositiveUnityDecision unclassified | PhraseMarkSemanticCandidate unclassified | NotationPolicyDecision unchanged_pending_visual | WriterOpen nein.\n",
            row.lane.part,
            row.lane.staff,
            row.lane.voice,
            row.projection_start.measure_ordinal,
            row.projection_start.start,
            row.projection_start.canonical_start,
            row.projection_end.measure_ordinal,
            row.projection_end.start,
            row.projection_end.canonical_start,
            row.projection_attack_count,
            if row.intrinsic_span_signature.is_some() {
                "known_exact_intrinsic"
            } else {
                "unavailable"
            },
            row.exact_pending_span_match_count,
            row.same_lane_exact_pending_span_match_count,
            row.cross_lane_exact_pending_span_match_count,
        ));
    }
    for (index, row) in snapshot
        .notation_intrinsic_all_window_parallelism_audit
        .iter()
        .enumerate()
    {
        out.push_format_args(format_args!(
            "PhraseNotationIntrinsicAllWindowParallelism[{index:03}]: part {} | staff {} | voice {} | projection_start m_ord {} onset {} canonical_start {} | projection_end m_ord {} onset {} canonical_start {} | projection_attacks {} | intrinsic_descriptor {} | exact_all_window_matches {} same_lane_all_window_matches {} cross_lane_all_window_matches {} exact_non_pending_window_matches {} | SearchSpace all_same_attack_count_windows_in_piece | SourceWindowExcluded ja | OutsideContextUsed nein_signature_only | SelectionBiasDecision audit_only | PositiveUnityDecision unclassified | PhraseMarkSemanticCandidate unclassified | NotationPolicyDecision unchanged_pending_visual | WriterOpen nein.\n",
            row.lane.part,
            row.lane.staff,
            row.lane.voice,
            row.projection_start.measure_ordinal,
            row.projection_start.start,
            row.projection_start.canonical_start,
            row.projection_end.measure_ordinal,
            row.projection_end.start,
            row.projection_end.canonical_start,
            row.projection_attack_count,
            if row.intrinsic_span_signature.is_some() {
                "known_exact_intrinsic"
            } else {
                "unavailable"
            },
            row.exact_all_window_match_count,
            row.same_lane_exact_all_window_match_count,
            row.cross_lane_exact_all_window_match_count,
            row.exact_non_pending_window_match_count,
        ));
        for (sample_index, sample) in row.match_samples.iter().enumerate() {
            out.push_format_args(format_args!(
                "PhraseNotationIntrinsicAllWindowMatch[{index:03}.{sample_index:02}]: matched_part {} | matched_staff {} | matched_voice {} | matched_start m_ord {} onset {} canonical_start {} | matched_end m_ord {} onset {} canonical_start {} | MatchedWindowStartRole {} | MatchedWindowEndRole {} | MatchedWindowIsPending {} | StartRoleReference boundary_immediately_before_window_start | EndRoleReference boundary_arriving_at_window_end | descriptor_match exact_intrinsic | diagnostic_only.\n",
                sample.lane.part,
                sample.lane.staff,
                sample.lane.voice,
                sample.start.measure_ordinal,
                sample.start.start,
                sample.start.canonical_start,
                sample.end.measure_ordinal,
                sample.end.start,
                sample.end.canonical_start,
                sample.start_role,
                sample.end_role,
                yes_no(sample.is_pending),
            ));
        }
    }
    for (index, row) in snapshot
        .notation_structural_variation_factorization_audit
        .iter()
        .enumerate()
    {
        out.push_format_args(format_args!(
            "PhraseNotationStructuralVariationFactorization[{index:03}]: part {} | staff {} | voice {} | projection_start m_ord {} onset {} canonical_start {} | projection_end m_ord {} onset {} canonical_start {} | projection_attacks {} | variation_projection {} | full_exact_reobserved_matches {} | texture_only_relaxed_matches {} | sounding_duration_only_relaxed_matches {} | interval_magnitude_only_relaxed_matches {} | rhythm_dimension_relaxed_matches {} | melody_dimension_relaxed_matches {} | SearchSpace all_same_attack_count_windows_in_piece | ProjectionSemantics exact_information_drop_only | Similarity none | Threshold none | Scalarization none | StructuralVariationDecision diagnostic_only | PositiveUnityDecision unclassified | NotationPolicyDecision unchanged_pending_visual | WriterOpen nein.\n",
            row.lane.part,
            row.lane.staff,
            row.lane.voice,
            row.projection_start.measure_ordinal,
            row.projection_start.start,
            row.projection_start.canonical_start,
            row.projection_end.measure_ordinal,
            row.projection_end.start,
            row.projection_end.canonical_start,
            row.projection_attack_count,
            if row.variation_projection.is_some() {
                "known_exact_factorization"
            } else {
                "unavailable"
            },
            row.full_exact_reobserved_match_count,
            row.texture_relaxed_match_count,
            row.sounding_duration_relaxed_match_count,
            row.interval_magnitude_relaxed_match_count,
            row.rhythm_relaxed_match_count,
            row.melody_relaxed_match_count,
        ));
        for (sample_index, sample) in row.match_samples.iter().enumerate() {
            out.push_format_args(format_args!(
                "PhraseNotationStructuralVariationMatch[{index:03}.{sample_index:02}]: relaxation {} | matched_part {} | matched_staff {} | matched_voice {} | matched_start m_ord {} onset {} canonical_start {} | matched_end m_ord {} onset {} canonical_start {} | MatchedWindowIsPending {} | exact_projection_match ja | diagnostic_only.\n",
                sample.relaxation,
                sample.lane.part,
                sample.lane.staff,
                sample.lane.voice,
                sample.start.measure_ordinal,
                sample.start.start,
                sample.start.canonical_start,
                sample.end.measure_ordinal,
                sample.end.start,
                sample.end.canonical_start,
                yes_no(sample.is_pending),
            ));
        }
    }
    for (index, row) in snapshot.notation_higher_level_unity_audit.iter().enumerate() {
        out.push_format_args(format_args!(
            "PhraseNotationHigherLevelUnity[{index:03}]: part {} | staff {} | voice {} | projection_start m_ord {} onset {} canonical_start {} | projection_end m_ord {} onset {} canonical_start {} | projection_attacks {} | internal_boundaries {} structural_joint {} continuation_supported {} unresolved {} all_internal_continuation_supported {} | whole_span_parallel_descriptor {} exact_matches {} same_lane_matches {} cross_lane_matches {} same_role_matches {} divergent_role_matches {} | terminal_exact_joint_matches {} terminal_same_lane_matches {} terminal_cross_lane_matches {} terminal_role_divergence_reason {} terminal_role {} | PositiveUnityDecision unclassified | PhraseMarkSemanticCandidate unclassified | NotationPolicyDecision unchanged_pending_visual | WriterOpen nein.\n",
            row.lane.part,
            row.lane.staff,
            row.lane.voice,
            row.projection_start.measure_ordinal,
            row.projection_start.start,
            row.projection_start.canonical_start,
            row.projection_end.measure_ordinal,
            row.projection_end.start,
            row.projection_end.canonical_start,
            row.projection_attack_count,
            row.internal_boundary_count,
            row.internal_structural_joint_count,
            row.internal_continuation_supported_count,
            row.internal_unresolved_count,
            yes_no(row.all_internal_boundaries_continuation_supported),
            if row.span_parallel_signature.is_some() {
                "known_exact_sequence"
            } else {
                "unavailable"
            },
            row.exact_span_parallel_match_count,
            row.same_lane_exact_span_parallel_match_count,
            row.cross_lane_exact_span_parallel_match_count,
            row.exact_span_parallel_same_role_match_count,
            row.exact_span_parallel_divergent_role_match_count,
            row.terminal_exact_joint_match_count,
            row.terminal_same_lane_match_count,
            row.terminal_cross_lane_match_count,
            optional_str(row.terminal_role_divergence_reason),
            row.terminal_role,
        ));
    }
}

fn optional_parallel_rhythm_signature(value: Option<ParallelRhythmSignature>) -> String {
    value.map_or_else(
        || "unresolved".to_string(),
        |value| {
            format!(
                "{}/{}/{}/{}/{}",
                value[0], value[1], value[2], value[3], value[4]
            )
        },
    )
}

fn optional_parallel_melodic_signature(value: Option<ParallelMelodicSignature>) -> String {
    value.map_or_else(
        || "unresolved".to_string(),
        |value| {
            format!(
                "{}/{}/{}|{}/{}/{}/{}",
                value.centroid_deltas_x100[0],
                value.centroid_deltas_x100[1],
                value.centroid_deltas_x100[2],
                value.chord_members[0],
                value.chord_members[1],
                value.chord_members[2],
                value.chord_members[3]
            )
        },
    )
}

fn optional_i128(value: Option<i128>) -> String {
    value.map_or_else(|| "unresolved".to_string(), |value| value.to_string())
}

fn optional_str(value: Option<&'static str>) -> &'static str {
    value.unwrap_or("unresolved")
}

fn optional_i64(value: Option<i64>) -> String {
    value.map_or_else(|| "unresolved".to_string(), |value| value.to_string())
}

fn optional_i32(value: Option<i32>) -> String {
    value.map_or_else(|| "unresolved".to_string(), |value| value.to_string())
}

fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

#[cfg(test)]
mod tests {
    use super::{
        analyze, analyze_with_tonal_context, append_snapshot,
        annotate_phrase_notation_higher_level_unity,
        annotate_phrase_notation_intrinsic_all_window_parallelism,
        annotate_phrase_notation_intrinsic_span_signature,
        annotate_phrase_notation_structural_variation_factorization, intrinsic_span_parallel_signature,
        intrinsic_span_variation_projection, Attack, JointParallelSignature, LaneKey,
        ParallelMelodicSignature, PhraseAtomicObservation, PhraseNotationHigherLevelUnityAuditRow,
        PhraseNotationIntrinsicAllWindowParallelismAuditRow,
        PhraseNotationIntrinsicSpanSignatureAuditRow,
        PhraseNotationStructuralVariationFactorizationAuditRow,
    };
    use std::collections::BTreeMap;

    fn note(
        ordinal: usize,
        measure: usize,
        start: i64,
        duration: i64,
        midi: Option<i32>,
    ) -> PhraseAtomicObservation {
        const MEASURE_TICKS: i128 = 16;
        PhraseAtomicObservation {
            source_ordinal: ordinal,
            part: "P1".to_string(),
            measure_ordinal: measure,
            staff: "1".to_string(),
            voice: "1".to_string(),
            start,
            duration,
            canonical_start: i128::try_from(measure.saturating_sub(1)).unwrap_or(0)
                * MEASURE_TICKS
                + i128::from(start),
            canonical_duration: i128::from(duration),
            canonical_measure_start: i128::try_from(measure.saturating_sub(1)).unwrap_or(0)
                * MEASURE_TICKS,
            canonical_beat_ticks: 4,
            measure_beats: 4,
            measure_beat_type: 4,
            pitch_midi: midi,
            visible: true,
            tie_start: false,
            tie_stop: false,
        }
    }

    fn with_tie(mut row: PhraseAtomicObservation, start: bool, stop: bool) -> PhraseAtomicObservation {
        row.tie_start = start;
        row.tie_stop = stop;
        row
    }

    #[test]
    fn rests_support_but_do_not_create_candidate_positions() {
        let without_rest = analyze(
            "test",
            true,
            "test",
            4,
            vec![note(0, 1, 0, 4, Some(60)), note(1, 1, 8, 4, Some(62))],
        );
        let with_rest = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(60)),
                note(1, 1, 4, 4, None),
                note(2, 1, 8, 4, Some(62)),
            ],
        );
        assert_eq!(without_rest.boundary_count, 1);
        assert_eq!(with_rest.boundary_count, 1);
        assert_eq!(without_rest.explicit_rest_support, 0);
        assert_eq!(with_rest.explicit_rest_support, 1);
    }

    #[test]
    fn exact_continuation_is_negative_boundary_evidence_not_a_phrase_verdict() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![note(0, 1, 0, 4, Some(60)), note(1, 1, 4, 4, Some(62))],
        );
        assert_eq!(snapshot.boundary_count, 1);
        assert_eq!(snapshot.contiguous_opposition, 1);
        assert_eq!(snapshot.positive_gap_support, 0);
        assert!(snapshot.samples[0].continuity_oppose);
    }

    #[test]
    fn tie_stop_only_onset_is_continuation_not_attack_and_extends_sounding_end() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                with_tie(note(0, 1, 0, 4, Some(60)), true, false),
                with_tie(note(1, 1, 4, 4, Some(60)), false, true),
                note(2, 1, 8, 4, Some(62)),
            ],
        );
        assert_eq!(snapshot.attack_count, 2);
        assert_eq!(snapshot.tie_continuation_members_excluded, 1);
        assert_eq!(snapshot.tie_only_onsets_excluded, 1);
        assert_eq!(snapshot.same_measure_tie_continuation_extensions, 1);
        assert_eq!(snapshot.cross_measure_tie_continuation_extensions, 0);
        assert_eq!(snapshot.boundary_count, 1);
        assert_eq!(snapshot.positive_gap_support, 0);
        assert_eq!(snapshot.contiguous_opposition, 1);
        assert_eq!(snapshot.samples[0].temporal_gap, 0);
    }

    #[test]
    fn cross_measure_tie_stop_extends_canonical_continuity_without_new_attack() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                with_tie(note(0, 1, 12, 4, Some(60)), true, false),
                with_tie(note(1, 2, 0, 4, Some(60)), false, true),
                note(2, 2, 4, 4, Some(62)),
            ],
        );
        assert_eq!(snapshot.attack_count, 2);
        assert_eq!(snapshot.tie_only_onsets_excluded, 1);
        assert_eq!(snapshot.same_measure_tie_continuation_extensions, 0);
        assert_eq!(snapshot.cross_measure_tie_continuation_extensions, 1);
        assert_eq!(snapshot.cross_measure_resolved, 1);
        assert_eq!(snapshot.contiguous_opposition, 1);
        assert_eq!(snapshot.samples[0].temporal_gap, 0);
    }

    #[test]
    fn mixed_tied_and_new_pitch_onset_remains_one_true_attack() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                with_tie(note(0, 1, 0, 4, Some(60)), true, false),
                with_tie(note(1, 1, 4, 4, Some(60)), false, true),
                note(2, 1, 4, 4, Some(64)),
                note(3, 1, 8, 4, Some(67)),
            ],
        );
        assert_eq!(snapshot.attack_count, 3);
        assert_eq!(snapshot.tie_continuation_members_excluded, 1);
        assert_eq!(snapshot.tie_only_onsets_excluded, 0);
        assert_eq!(snapshot.mixed_onsets_with_tie_continuation, 1);
        assert_eq!(snapshot.samples[0].nearest_pitch_distance, Some(4));
    }

    #[test]
    fn cross_measure_exact_continuation_is_resolved_without_barline_support() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![note(0, 1, 12, 4, Some(60)), note(1, 2, 0, 4, Some(62))],
        );
        assert_eq!(snapshot.boundary_count, 1);
        assert_eq!(snapshot.cross_measure_resolved, 1);
        assert_eq!(snapshot.positive_gap_support, 0);
        assert_eq!(snapshot.contiguous_opposition, 1);
        assert_eq!(snapshot.samples[0].temporal_gap, 0);
        assert!(!snapshot.samples[0].rhythmic_support);
    }

    #[test]
    fn cross_measure_gap_is_evidence_only_not_a_phrase_verdict() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![note(0, 1, 8, 4, Some(60)), note(1, 2, 4, 4, Some(62))],
        );
        assert_eq!(snapshot.boundary_count, 1);
        assert_eq!(snapshot.cross_measure_resolved, 1);
        assert_eq!(snapshot.positive_gap_support, 1);
        assert_eq!(snapshot.samples[0].temporal_gap, 8);
        assert!(snapshot.samples[0].rhythmic_support);
    }

    #[test]
    fn lanes_are_never_collapsed_or_majority_voted() {
        let mut second_lane = note(2, 1, 0, 4, Some(48));
        second_lane.staff = "2".to_string();
        let mut second_lane_next = note(3, 1, 4, 4, Some(50));
        second_lane_next.staff = "2".to_string();
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(60)),
                note(1, 1, 4, 4, Some(62)),
                second_lane,
                second_lane_next,
            ],
        );
        assert_eq!(snapshot.boundary_count, 2);
        let staffs = snapshot
            .samples
            .iter()
            .map(|row| row.lane.staff.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(staffs, std::collections::BTreeSet::from(["1", "2"]));
    }

    #[test]
    fn metric_descriptors_use_exact_right_attack_position_without_barline_support() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(60)),
                note(1, 1, 4, 4, Some(62)),
                note(2, 2, 0, 4, Some(64)),
            ],
        );
        assert_eq!(snapshot.metric_descriptor_known, 2);
        assert_eq!(snapshot.metric_on_beat_grid, 2);
        assert_eq!(snapshot.metric_measure_start, 1);
        assert_eq!(snapshot.samples[1].metric_measure_offset_ticks, Some(0));
        assert_eq!(snapshot.samples[1].metric_beat_index_zero_based, Some(0));
        assert_eq!(snapshot.samples[1].metric_beat_phase_ticks, Some(0));
        assert_eq!(snapshot.positive_gap_support, 1);
    }

    #[test]
    fn offbeat_metric_position_is_descriptor_only() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![note(0, 1, 0, 2, Some(60)), note(1, 1, 2, 2, Some(62))],
        );
        assert_eq!(snapshot.metric_descriptor_known, 1);
        assert_eq!(snapshot.metric_on_beat_grid, 0);
        assert_eq!(snapshot.metric_off_beat_grid, 1);
        assert_eq!(snapshot.samples[0].metric_beat_phase_ticks, Some(2));
        assert_eq!(snapshot.samples[0].metric_beat_index_zero_based, Some(0));
        assert_eq!(snapshot.samples[0].temporal_gap, 0);
    }

    #[test]
    fn melodic_contour_relations_are_relative_and_threshold_free() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(60)),
                note(1, 1, 4, 4, Some(62)),
                note(2, 1, 8, 4, Some(67)),
                note(3, 1, 12, 4, Some(65)),
            ],
        );
        assert_eq!(snapshot.melodic_context_before_known, 2);
        assert_eq!(snapshot.melodic_context_after_known, 2);
        assert_eq!(snapshot.samples[1].melodic_contour_into_boundary, Some("continuation"));
        assert_eq!(snapshot.samples[1].melodic_contour_out_of_boundary, Some("reversal"));
        assert_eq!(snapshot.samples[1].melodic_boundary_interval_relation, Some("local_peak"));
        assert_eq!(snapshot.melodic_boundary_interval_local_peak, 1);
    }

    #[test]
    fn melodic_pitch_overlap_is_observed_without_promoting_a_boundary() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![note(0, 1, 0, 4, Some(60)), note(1, 1, 4, 4, Some(60))],
        );
        assert!(snapshot.samples[0].melodic_pitch_overlap);
        assert_eq!(snapshot.melodic_pitch_overlap, 1);
        assert_eq!(snapshot.samples[0].nearest_pitch_distance, Some(0));
    }

    #[test]
    fn exact_piece_internal_parallelism_is_transposition_and_augmentation_invariant() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 2, Some(60)),
                note(1, 1, 2, 2, Some(62)),
                note(2, 1, 4, 2, Some(64)),
                note(3, 1, 6, 2, Some(65)),
                note(4, 1, 20, 4, Some(72)),
                note(5, 1, 24, 4, Some(74)),
                note(6, 1, 28, 4, Some(76)),
                note(7, 1, 32, 4, Some(77)),
            ],
        );
        assert!(snapshot.parallel_descriptor_known >= 4);
        assert!(snapshot.parallel_joint_repeated >= 2);
        assert!(snapshot.parallel_max_group_cardinality >= 2);
        assert!(snapshot
            .samples
            .iter()
            .any(|row| row.parallel_exact_joint_match_count > 0));
    }

    #[test]
    fn exact_parallel_role_divergence_is_explained_by_tonal_context() {
        let snapshot = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 2, Some(64)),
                note(1, 1, 4, 2, Some(67)),
                note(2, 1, 8, 2, Some(72)),
                note(3, 1, 12, 2, Some(74)),
                note(4, 2, 0, 2, Some(66)),
                note(5, 2, 4, 2, Some(69)),
                note(6, 2, 8, 2, Some(74)),
                note(7, 2, 12, 2, Some(76)),
            ],
            Some(("C-Dur/C-major", 0, false, 140, 80)),
        );
        assert!(snapshot.parallel_role_divergent_groups >= 1);
        assert!(snapshot.parallel_role_divergent_tonal_context >= 1);
        assert_eq!(snapshot.parallel_role_divergent_unexplained, 0);
        assert!(snapshot.samples.iter().any(|row| {
            row.parallel_role_divergence_reason == Some("tonal_context")
        }));
    }

    #[test]
    fn structural_novelty_reports_score_activity_change_without_boundary_verdict() {
        let mut second_voice = note(4, 1, 0, 8, Some(48));
        second_voice.voice = "2".to_string();
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(60)),
                note(1, 1, 4, 4, Some(62)),
                note(2, 1, 8, 4, Some(64)),
                note(3, 1, 12, 4, Some(65)),
                second_voice,
            ],
        );
        let activity_change = snapshot
            .samples
            .iter()
            .find(|row| row.right.canonical_start == 8)
            .expect("boundary at tick 8");
        assert_eq!(activity_change.novelty_active_lanes_before, 2);
        assert_eq!(activity_change.novelty_active_lanes_after, 1);
        assert!(activity_change.novelty_score_active_lane_change);
        assert!(snapshot.novelty_score_active_lane_change >= 1);
    }

    #[test]
    fn tonal_closure_and_structural_gate_create_audit_only_phrase_structure() {
        let snapshot = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(64)),
                note(1, 1, 4, 2, Some(67)),
                note(2, 1, 8, 4, Some(60)),
            ],
            Some(("C-Dur/C-major", 0, false, 140, 80)),
        );
        assert_eq!(snapshot.tonal_descriptor_known, 2);
        assert_eq!(snapshot.tonal_closure_support, 1);
        assert_eq!(snapshot.phrase_closure_supported, 1);
        assert_eq!(snapshot.level_one_closed_spans, 1);
        assert_eq!(snapshot.slur_intent_allow_structural, 1);
        assert_eq!(snapshot.slur_intent_withhold, 0);
        assert_eq!(snapshot.pending_visual_notation_intents.len(), 1);
        let intent = &snapshot.pending_visual_notation_intents[0];
        assert_eq!(intent.part, "P1");
        assert_eq!(intent.staff, "1");
        assert_eq!(intent.voice, "1");
        assert_eq!(intent.start_measure_ordinal, 1);
        assert_eq!(intent.start_onset, 0);
        assert_eq!(intent.end_measure_ordinal, 1);
        assert_eq!(intent.end_onset, 8);
        assert_eq!(intent.attack_count, 3);
        assert_eq!(snapshot.samples[1].phrase_boundary_role, "closure_supported");
        assert_eq!(snapshot.notation_internal_hierarchy_audit.len(), 1);
        let hierarchy = &snapshot.notation_internal_hierarchy_audit[0];
        assert_eq!(hierarchy.structural_sample_attack_count, 2);
        assert_eq!(hierarchy.projection_attack_count, 3);
        assert_eq!(hierarchy.internal_boundary_count, 1);
        assert_eq!(hierarchy.internal_structural_joint_count, 0);
        assert_eq!(hierarchy.internal_closure_structure_gate_count, 0);
        assert_eq!(hierarchy.terminal_role, "closure_supported");
        assert_eq!(snapshot.notation_higher_level_unity_audit.len(), 1);
        let unity = &snapshot.notation_higher_level_unity_audit[0];
        assert_eq!(unity.projection_attack_count, intent.attack_count);
        assert_eq!(unity.terminal_role, "closure_supported");
    }

    #[test]
    fn notation_hierarchy_audit_exposes_internal_structural_joint_without_changing_intent() {
        let snapshot = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 2, Some(64)),
                note(1, 1, 3, 1, Some(66)),
                note(2, 1, 4, 2, Some(71)),
                note(3, 1, 8, 4, Some(60)),
            ],
            Some(("C-Dur/C-major", 0, false, 140, 80)),
        );
        assert_eq!(snapshot.slur_intent_allow_structural, 1);
        assert_eq!(snapshot.pending_visual_notation_intents.len(), 1);
        assert_eq!(snapshot.notation_internal_hierarchy_audit.len(), 1);
        let hierarchy = &snapshot.notation_internal_hierarchy_audit[0];
        assert_eq!(hierarchy.structural_sample_attack_count, 3);
        assert_eq!(hierarchy.projection_attack_count, 4);
        assert_eq!(hierarchy.internal_boundary_count, 2);
        assert!(hierarchy.internal_structural_joint_count >= 1);
        assert_eq!(hierarchy.internal_closure_structure_gate_count, 0);
        assert_eq!(hierarchy.measure_span_inclusive, 1);
        assert_eq!(hierarchy.terminal_role, "closure_supported");
    }

    #[test]
    fn low_margin_tonal_profile_preserves_raw_evidence_without_phrase_authority() {
        let snapshot = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(64)),
                note(1, 1, 4, 2, Some(67)),
                note(2, 1, 8, 4, Some(60)),
            ],
            Some(("C-Dur/C-major", 0, false, 100, 80)),
        );
        assert!(!snapshot.tonal_profile_reliable);
        assert_eq!(snapshot.tonal_closure_support, 1);
        assert_eq!(snapshot.tonal_closure_authorized, 0);
        assert_eq!(snapshot.phrase_closure_supported, 0);
        assert_eq!(snapshot.level_one_closed_spans, 0);
        assert_eq!(snapshot.slur_intent_allow_structural, 0);
    }

    #[test]
    fn tonal_profile_reliability_matches_existing_owner_margin_boundary() {
        let rows = vec![
            note(0, 1, 0, 4, Some(64)),
            note(1, 1, 4, 2, Some(67)),
            note(2, 1, 8, 4, Some(60)),
        ];
        let at_boundary = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            rows.clone(),
            Some(("C-Dur/C-major", 0, false, 128, 80)),
        );
        let below_boundary = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            rows,
            Some(("C-Dur/C-major", 0, false, 127, 80)),
        );
        assert!(at_boundary.tonal_profile_reliable);
        assert!(!below_boundary.tonal_profile_reliable);
    }

    #[test]
    fn tonal_arrival_without_independent_structure_does_not_become_phrase_closure() {
        let snapshot = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            vec![note(0, 1, 0, 4, Some(67)), note(1, 1, 4, 4, Some(60))],
            Some(("C-Dur/C-major", 0, false, 140, 80)),
        );
        assert_eq!(snapshot.tonal_closure_support, 1);
        assert_eq!(snapshot.phrase_closure_supported, 0);
        assert_eq!(snapshot.level_one_closed_spans, 0);
        assert_eq!(snapshot.slur_intent_allow_structural, 0);
        assert_eq!(snapshot.samples[0].phrase_boundary_role, "unresolved");
    }

    #[test]
    fn unresolved_leading_tone_remains_explicit_conflict_across_score_closure() {
        let mut upper_left = note(0, 1, 0, 4, Some(59));
        upper_left.staff = "1".to_string();
        upper_left.voice = "1".to_string();
        let mut upper_right = note(1, 1, 4, 4, Some(62));
        upper_right.staff = "1".to_string();
        upper_right.voice = "1".to_string();
        let mut lower_left = note(2, 1, 0, 4, Some(43));
        lower_left.staff = "2".to_string();
        lower_left.voice = "2".to_string();
        let mut lower_right = note(3, 1, 4, 4, Some(48));
        lower_right.staff = "2".to_string();
        lower_right.voice = "2".to_string();
        let snapshot = analyze_with_tonal_context(
            "test",
            true,
            "test",
            4,
            vec![upper_left, upper_right, lower_left, lower_right],
            Some(("C-Dur/C-major", 0, false, 140, 80)),
        );
        assert!(snapshot.tonal_bass_dominant_to_tonic >= 1);
        assert!(snapshot.tonal_lane_unresolved_leading_tone >= 1);
        assert!(snapshot.phrase_conflict >= 1);
        assert!(snapshot
            .samples
            .iter()
            .any(|row| row.phrase_boundary_role == "conflict"));
    }

    fn intrinsic_test_attack(
        canonical_start: i128,
        canonical_duration: i128,
        pitches: Vec<i32>,
    ) -> Attack {
        Attack {
            measure_ordinal: 1,
            start: i64::try_from(canonical_start).unwrap_or(0),
            duration: i64::try_from(canonical_duration).unwrap_or(0),
            sounding_end: i64::try_from(canonical_start.saturating_add(canonical_duration))
                .unwrap_or(0),
            canonical_start,
            canonical_sounding_end: canonical_start.saturating_add(canonical_duration),
            canonical_measure_start: 0,
            canonical_beat_ticks: 4,
            measure_beats: 4,
            measure_beat_type: 4,
            chord_members: pitches.len(),
            pitches,
            tied_continuation_members_at_onset: 0,
        }
    }

    #[test]
    fn intrinsic_span_signature_ignores_outside_context_and_preserves_exact_invariances() {
        let base = vec![
            intrinsic_test_attack(10, 2, vec![60]),
            intrinsic_test_attack(14, 2, vec![62]),
            intrinsic_test_attack(18, 4, vec![64, 67]),
        ];
        let augmented_transposed = vec![
            intrinsic_test_attack(100, 4, vec![72]),
            intrinsic_test_attack(108, 4, vec![74]),
            intrinsic_test_attack(116, 8, vec![76, 79]),
        ];
        assert_eq!(
            intrinsic_span_parallel_signature(&base),
            intrinsic_span_parallel_signature(&augmented_transposed)
        );

        let rhythm_changed = vec![
            intrinsic_test_attack(10, 2, vec![60]),
            intrinsic_test_attack(13, 2, vec![62]),
            intrinsic_test_attack(18, 4, vec![64, 67]),
        ];
        assert_ne!(
            intrinsic_span_parallel_signature(&base),
            intrinsic_span_parallel_signature(&rhythm_changed)
        );
    }

    fn intrinsic_test_row(
        staff: &str,
        attacks: &[Attack],
    ) -> PhraseNotationIntrinsicSpanSignatureAuditRow {
        PhraseNotationIntrinsicSpanSignatureAuditRow {
            lane: LaneKey {
                part: "P1".to_string(),
                staff: staff.to_string(),
                voice: "1".to_string(),
            },
            projection_start: attacks[0].clone(),
            projection_end: attacks[attacks.len() - 1].clone(),
            projection_attack_count: attacks.len(),
            intrinsic_span_signature: intrinsic_span_parallel_signature(attacks),
            exact_pending_span_match_count: 0,
            same_lane_exact_pending_span_match_count: 0,
            cross_lane_exact_pending_span_match_count: 0,
        }
    }

    #[test]
    fn intrinsic_span_signature_audit_counts_pending_matches_without_promoting_them() {
        let base = vec![
            intrinsic_test_attack(0, 2, vec![60]),
            intrinsic_test_attack(4, 2, vec![62]),
            intrinsic_test_attack(8, 4, vec![64]),
        ];
        let same_shape = vec![
            intrinsic_test_attack(20, 4, vec![65]),
            intrinsic_test_attack(28, 4, vec![67]),
            intrinsic_test_attack(36, 8, vec![69]),
        ];
        let different = vec![
            intrinsic_test_attack(40, 2, vec![60]),
            intrinsic_test_attack(44, 2, vec![63]),
            intrinsic_test_attack(48, 4, vec![64]),
        ];
        let mut rows = vec![
            intrinsic_test_row("1", &base),
            intrinsic_test_row("1", &same_shape),
            intrinsic_test_row("2", &same_shape),
            intrinsic_test_row("1", &different),
        ];
        annotate_phrase_notation_intrinsic_span_signature(&mut rows);

        assert_eq!(rows[0].exact_pending_span_match_count, 2);
        assert_eq!(rows[0].same_lane_exact_pending_span_match_count, 1);
        assert_eq!(rows[0].cross_lane_exact_pending_span_match_count, 1);
        assert_eq!(rows[3].exact_pending_span_match_count, 0);
    }

    #[test]
    fn intrinsic_all_window_audit_finds_non_pending_same_lane_match_without_promotion() {
        let base = vec![
            intrinsic_test_attack(0, 2, vec![60]),
            intrinsic_test_attack(4, 2, vec![62]),
            intrinsic_test_attack(8, 4, vec![64]),
        ];
        let repeated = vec![
            intrinsic_test_attack(20, 4, vec![65]),
            intrinsic_test_attack(28, 4, vec![67]),
            intrinsic_test_attack(36, 8, vec![69]),
        ];
        let pending = intrinsic_test_row("1", &base);
        let lane = pending.lane.clone();
        let mut all_attacks = base.clone();
        all_attacks.extend(repeated.clone());
        let lane_attacks = BTreeMap::from([(lane.clone(), all_attacks)]);
        let boundary_roles = BTreeMap::from([
            ((lane.clone(), 20), "structural_joint"),
            ((lane.clone(), 36), "closure_supported"),
        ]);
        let mut rows = vec![PhraseNotationIntrinsicAllWindowParallelismAuditRow {
            lane: lane.clone(),
            projection_start: base[0].clone(),
            projection_end: base[2].clone(),
            projection_attack_count: base.len(),
            intrinsic_span_signature: intrinsic_span_parallel_signature(&base),
            exact_all_window_match_count: 0,
            same_lane_exact_all_window_match_count: 0,
            cross_lane_exact_all_window_match_count: 0,
            exact_non_pending_window_match_count: 0,
            match_samples: Vec::new(),
        }];

        annotate_phrase_notation_intrinsic_all_window_parallelism(
            &mut rows,
            &[pending],
            &lane_attacks,
            &boundary_roles,
        );

        assert_eq!(rows[0].exact_all_window_match_count, 1);
        assert_eq!(rows[0].same_lane_exact_all_window_match_count, 1);
        assert_eq!(rows[0].cross_lane_exact_all_window_match_count, 0);
        assert_eq!(rows[0].exact_non_pending_window_match_count, 1);
        assert_eq!(rows[0].match_samples.len(), 1);
        assert_eq!(rows[0].match_samples[0].start_role, "structural_joint");
        assert_eq!(rows[0].match_samples[0].end_role, "closure_supported");
        assert!(!rows[0].match_samples[0].is_pending);
    }

    #[test]
    fn structural_variation_factorization_is_exact_projection_only_and_non_promoting() {
        let base = vec![
            intrinsic_test_attack(0, 2, vec![60]),
            intrinsic_test_attack(4, 2, vec![62]),
            intrinsic_test_attack(8, 4, vec![64]),
        ];
        let texture_changed = vec![
            intrinsic_test_attack(20, 2, vec![59, 61]),
            intrinsic_test_attack(24, 2, vec![61, 63]),
            intrinsic_test_attack(28, 4, vec![63, 65]),
        ];
        let sounding_duration_changed = vec![
            intrinsic_test_attack(40, 1, vec![70]),
            intrinsic_test_attack(44, 3, vec![72]),
            intrinsic_test_attack(48, 1, vec![74]),
        ];
        let interval_magnitude_changed = vec![
            intrinsic_test_attack(60, 2, vec![70]),
            intrinsic_test_attack(64, 2, vec![73]),
            intrinsic_test_attack(68, 4, vec![76]),
        ];
        let rhythm_changed = vec![
            intrinsic_test_attack(80, 2, vec![80]),
            intrinsic_test_attack(83, 2, vec![82]),
            intrinsic_test_attack(88, 4, vec![84]),
        ];
        let pending = intrinsic_test_row("1", &base);
        let source_lane = pending.lane.clone();
        let lane = |staff: &str| LaneKey {
            part: "P1".to_string(),
            staff: staff.to_string(),
            voice: "1".to_string(),
        };
        let lane_attacks = BTreeMap::from([
            (source_lane.clone(), base.clone()),
            (lane("2"), texture_changed),
            (lane("3"), sounding_duration_changed),
            (lane("4"), interval_magnitude_changed),
            (lane("5"), rhythm_changed),
        ]);
        let mut rows = vec![PhraseNotationStructuralVariationFactorizationAuditRow {
            lane: source_lane,
            projection_start: base[0].clone(),
            projection_end: base[2].clone(),
            projection_attack_count: base.len(),
            variation_projection: intrinsic_span_variation_projection(&base),
            full_exact_reobserved_match_count: 0,
            texture_relaxed_match_count: 0,
            sounding_duration_relaxed_match_count: 0,
            interval_magnitude_relaxed_match_count: 0,
            rhythm_relaxed_match_count: 0,
            melody_relaxed_match_count: 0,
            match_samples: Vec::new(),
        }];

        annotate_phrase_notation_structural_variation_factorization(
            &mut rows,
            &[pending],
            &lane_attacks,
        );

        assert_eq!(rows[0].full_exact_reobserved_match_count, 0);
        assert!(rows[0].texture_relaxed_match_count >= 1);
        assert!(rows[0].sounding_duration_relaxed_match_count >= 1);
        assert!(rows[0].interval_magnitude_relaxed_match_count >= 1);
        assert!(rows[0].rhythm_relaxed_match_count >= 1);
        assert!(rows[0].melody_relaxed_match_count >= 2);
        assert!(!rows[0].match_samples.is_empty());
    }

    #[test]
    fn intrinsic_parallelism_corpus_closure_is_report_only_and_alignment_explicit() {
        let snapshot = analyze(
            "test",
            true,
            "test",
            4,
            vec![
                note(0, 1, 0, 4, Some(60)),
                note(1, 1, 4, 4, Some(62)),
                note(2, 1, 8, 4, Some(64)),
            ],
        );
        let mut report = String::new();
        append_snapshot(&mut report, &snapshot);
        assert!(report.contains("PhraseNotationIntrinsicParallelismCorpusClosureAudit:"));
        assert!(report.contains("PhraseNotationStructuralVariationFactorizationAudit:"));
        assert!(report.contains("ProjectionSemantics exact_information_drop_only;"));
        assert!(report.contains("RowAlignmentMismatches 0; RowAlignmentConsistent ja;"));
        assert!(report.contains("RecognitionChanged nein; SignatureChanged nein; SearchSpaceChanged nein;"));
        assert!(report.contains("CorpusDecision deferred_until_all_projects;"));
        assert!(report.contains("NotationPolicyDecision unchanged_pending_visual; MutationAuthority none."));
    }

    fn higher_level_unity_test_attack(canonical_start: i128) -> Attack {
        Attack {
            measure_ordinal: 1,
            start: i64::try_from(canonical_start).unwrap_or(0),
            duration: 1,
            sounding_end: i64::try_from(canonical_start.saturating_add(1)).unwrap_or(0),
            canonical_start,
            canonical_sounding_end: canonical_start.saturating_add(1),
            canonical_measure_start: 0,
            canonical_beat_ticks: 4,
            measure_beats: 4,
            measure_beat_type: 4,
            pitches: vec![60],
            chord_members: 1,
            tied_continuation_members_at_onset: 0,
        }
    }

    fn higher_level_unity_test_row(
        staff: &str,
        roles: Vec<&'static str>,
        melodic_seed: i64,
    ) -> PhraseNotationHigherLevelUnityAuditRow {
        let signature = vec![JointParallelSignature {
            rhythm: [1, 1, 1, 1, 1],
            melodic: ParallelMelodicSignature {
                centroid_deltas_x100: [melodic_seed, melodic_seed, melodic_seed],
                chord_members: [1, 1, 1, 1],
            },
        }];
        PhraseNotationHigherLevelUnityAuditRow {
            lane: LaneKey {
                part: "P1".to_string(),
                staff: staff.to_string(),
                voice: "1".to_string(),
            },
            projection_start: higher_level_unity_test_attack(0),
            projection_end: higher_level_unity_test_attack(4),
            projection_attack_count: 2,
            internal_boundary_count: 1,
            internal_structural_joint_count: 0,
            internal_continuation_supported_count: 0,
            internal_unresolved_count: 1,
            all_internal_boundaries_continuation_supported: false,
            span_parallel_signature: Some(signature),
            span_role_sequence: roles,
            exact_span_parallel_match_count: 0,
            same_lane_exact_span_parallel_match_count: 0,
            cross_lane_exact_span_parallel_match_count: 0,
            exact_span_parallel_same_role_match_count: 0,
            exact_span_parallel_divergent_role_match_count: 0,
            terminal_exact_joint_match_count: 0,
            terminal_same_lane_match_count: 0,
            terminal_cross_lane_match_count: 0,
            terminal_role_divergence_reason: None,
            terminal_role: "closure_supported",
        }
    }

    #[test]
    fn higher_level_unity_audit_keeps_exact_span_parallelism_and_role_agreement_separate() {
        let mut rows = vec![
            higher_level_unity_test_row(
                "1",
                vec!["unresolved", "closure_supported"],
                100,
            ),
            higher_level_unity_test_row(
                "1",
                vec!["unresolved", "closure_supported"],
                100,
            ),
            higher_level_unity_test_row(
                "2",
                vec!["structural_joint", "closure_supported"],
                100,
            ),
            higher_level_unity_test_row(
                "1",
                vec!["unresolved", "closure_supported"],
                200,
            ),
        ];
        annotate_phrase_notation_higher_level_unity(&mut rows);

        assert_eq!(rows[0].exact_span_parallel_match_count, 2);
        assert_eq!(rows[0].same_lane_exact_span_parallel_match_count, 1);
        assert_eq!(rows[0].cross_lane_exact_span_parallel_match_count, 1);
        assert_eq!(rows[0].exact_span_parallel_same_role_match_count, 1);
        assert_eq!(rows[0].exact_span_parallel_divergent_role_match_count, 1);
        assert_eq!(rows[2].exact_span_parallel_match_count, 2);
        assert_eq!(rows[2].same_lane_exact_span_parallel_match_count, 0);
        assert_eq!(rows[2].cross_lane_exact_span_parallel_match_count, 2);
        assert_eq!(rows[2].exact_span_parallel_same_role_match_count, 0);
        assert_eq!(rows[2].exact_span_parallel_divergent_role_match_count, 2);
        assert_eq!(rows[3].exact_span_parallel_match_count, 0);
    }

    #[test]
    fn analysis_is_deterministic_for_identical_projection() {
        let rows = vec![
            note(0, 1, 0, 4, Some(60)),
            note(1, 1, 4, 4, Some(62)),
            note(2, 1, 12, 8, Some(67)),
        ];
        let first = analyze("test", true, "test", 4, rows.clone());
        let second = analyze("test", true, "test", 4, rows);
        assert_eq!(first.boundary_count, second.boundary_count);
        assert_eq!(first.attack_count, second.attack_count);
        assert_eq!(
            first.tie_continuation_members_excluded,
            second.tie_continuation_members_excluded
        );
        assert_eq!(first.positive_gap_support, second.positive_gap_support);
        assert_eq!(first.contiguous_opposition, second.contiguous_opposition);
        assert_eq!(first.metric_descriptor_known, second.metric_descriptor_known);
        assert_eq!(first.metric_on_beat_grid, second.metric_on_beat_grid);
        assert_eq!(first.melodic_contour_reversal_into, second.melodic_contour_reversal_into);
        assert_eq!(first.parallel_joint_repeated, second.parallel_joint_repeated);
        assert_eq!(
            first.novelty_score_active_lane_change,
            second.novelty_score_active_lane_change
        );
        assert_eq!(first.diversity_counts, second.diversity_counts);
        assert_eq!(first.samples[0].temporal_gap, second.samples[0].temporal_gap);
    }
}
