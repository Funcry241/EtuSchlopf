//! Audit-only WAV attack, pulse and metric reconstruction.
//!
//! Audio-only hypotheses are generated first. The frozen legacy source oracle is loaded from MIDI
//! only after hypothesis generation and is used exclusively for evaluation.
//! This module owns no productive source, pitch, staff/hand, phrase or writer authority.

use crate::audit_only::wav_observation_legacy_oracle::{
    load_canonical_legacy_unique_onsets_for_score_title,
};
use crate::audit_only::wav_signal_decode::{
    load_decoded_wav_signal_for_score_title, DecodedWavSignalAudit,
};
use std::collections::{BTreeMap, BTreeSet};

const CANONICAL_DIVISIONS: i64 = 96;
const SIXTEENTHS_PER_QUARTER: u64 = 4;
const MAX_LOCAL_PEAKS_PER_SECOND_PER_FAMILY: usize = 12;
const MIN_PEAK_CAPACITY_PER_FAMILY: usize = 64;
const MAX_PULSE_PAIR_DISTANCE: usize = 8;
const MAX_TEMPO_FAMILIES: usize = 6;
const MAX_PHASE_FAMILIES_PER_TEMPO: usize = 2;
const MAX_AUDIO_HYPOTHESES: usize = 96;
const READY_MATCH_40_PERCENT: usize = 75;
const READY_MATCH_80_PERCENT: usize = 90;
const READY_MEDIAN_MS: u64 = 25;
const READY_P90_MS: u64 = 80;
const PERFORMANCE_MAP_WINDOW_QUARTERS: u64 = 4;
const PERFORMANCE_MAP_MIN_ANCHORS_PER_WINDOW: usize = 3;

#[derive(Debug, Clone)]
pub(crate) struct WavAttackPulseMetricAudit {
    pub(crate) qualified: bool,
    pub(crate) reason: &'static str,
    pub(crate) sample_rate: u32,
    pub(crate) hop_frames: u64,
    pub(crate) envelope_bins: usize,
    pub(crate) energy_family_peaks: usize,
    pub(crate) transient_family_peaks: usize,
    pub(crate) attack_candidates: usize,
    pub(crate) dual_family_attack_candidates: usize,
    pub(crate) pulse_attack_candidates: usize,
    pub(crate) audio_hypothesis_count: usize,
    pub(crate) audio_top_hypotheses: Vec<AudioMetricHypothesisSummary>,
    pub(crate) audio_tempo_families: Vec<AudioTempoFamilySummary>,
    pub(crate) zero_origin_grid_families: Vec<AudioZeroOriginGridFamilySummary>,
    pub(crate) oracle_unique_onsets: usize,
    pub(crate) best_evaluations: Vec<MetricHypothesisEvaluation>,
    pub(crate) audio_identifiability_evaluations: Vec<MetricHypothesisEvaluation>,
    pub(crate) audio_identifiable: bool,
    pub(crate) performance_time_map: Option<AudioPerformanceTimeMapSummary>,
    pub(crate) performance_time_map_evaluation: Option<PerformanceTimeMapEvaluation>,
    pub(crate) performance_time_map_identifiable: bool,
}


#[derive(Debug, Clone)]
pub(crate) struct AudioMetricHypothesisSummary {
    pub(crate) audio_rank: usize,
    pub(crate) tempo_family_rank: usize,
    pub(crate) phase_family_rank: usize,
    pub(crate) quarter_frames: u64,
    pub(crate) bpm_milli: u64,
    pub(crate) origin_frame: i64,
    pub(crate) tempo_votes: usize,
    pub(crate) phase_votes: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct AudioTempoFamilySummary {
    pub(crate) tempo_family_rank: usize,
    pub(crate) quarter_frames: u64,
    pub(crate) bpm_milli: u64,
    pub(crate) tempo_votes: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AudioZeroOriginGridFamilySummary {
    pub(crate) grid_rank: usize,
    pub(crate) coarse_tempo_family_rank: usize,
    pub(crate) coarse_quarter_frames: u64,
    pub(crate) refined_quarter_frames: u64,
    pub(crate) bpm_milli: u64,
    pub(crate) tempo_votes: usize,
    pub(crate) grid_alignment_permille: u64,
    pub(crate) dual_grid_alignment_permille: u64,
    pub(crate) median_grid_residual_permille: u64,
    pub(crate) first_nonzero_frame: Option<u64>,
    pub(crate) first_nonzero_grid_residual_frames: u64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AudioPerformanceTimeMapSummary {
    pub(crate) quarter_frames: u64,
    pub(crate) window_quarters: u64,
    pub(crate) total_windows: usize,
    pub(crate) supported_windows: usize,
    pub(crate) dual_supported_windows: usize,
    pub(crate) node_count: usize,
    pub(crate) monotone: bool,
    pub(crate) median_abs_offset_frames: u64,
    pub(crate) p90_abs_offset_frames: u64,
    pub(crate) max_abs_offset_frames: u64,
    pub(crate) first_offset_frames: i64,
    pub(crate) last_offset_frames: i64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PerformanceTimeMapEvaluation {
    pub(crate) match_40: usize,
    pub(crate) match_80: usize,
    pub(crate) ordered_match_40: usize,
    pub(crate) ordered_match_80: usize,
    pub(crate) median_residual_frames: u64,
    pub(crate) p90_residual_frames: u64,
    pub(crate) projected_span_frames: u64,
    pub(crate) pulse_attack_span_frames: u64,
    pub(crate) span_ratio_permille: u64,
    pub(crate) oracle_unique_onsets: usize,
    pub(crate) ready: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct MetricHypothesisEvaluation {
    pub(crate) audio_rank: usize,
    pub(crate) tempo_family_rank: usize,
    pub(crate) phase_family_rank: usize,
    pub(crate) quarter_frames: u64,
    pub(crate) bpm_milli: u64,
    pub(crate) origin_frame: i64,
    pub(crate) tempo_votes: usize,
    pub(crate) phase_votes: usize,
    pub(crate) match_10: usize,
    pub(crate) match_20: usize,
    pub(crate) match_40: usize,
    pub(crate) match_80: usize,
    pub(crate) median_residual_frames: u64,
    pub(crate) p90_residual_frames: u64,
    pub(crate) attack_explained_40: usize,
    pub(crate) ordered_match_40: usize,
    pub(crate) ordered_match_80: usize,
    pub(crate) projected_legacy_span_frames: u64,
    pub(crate) pulse_attack_span_frames: u64,
    pub(crate) span_ratio_permille: u64,
    pub(crate) span_coherent: bool,
    pub(crate) oracle_unique_onsets: usize,
    pub(crate) attack_candidates: usize,
    pub(crate) ready: bool,
}

#[derive(Debug, Clone, Copy)]
struct EnvelopeBin {
    energy: f64,
    transient: f64,
}

#[derive(Debug, Clone, Copy)]
struct Peak {
    bin: usize,
    value: f64,
}

#[derive(Debug, Clone, Copy)]
struct AttackCandidate {
    frame: u64,
    energy_family: bool,
    transient_family: bool,
}

#[derive(Debug, Clone, Copy)]
struct AudioMetricHypothesis {
    audio_rank: usize,
    tempo_family_rank: usize,
    phase_family_rank: usize,
    quarter_frames: u64,
    origin_frame: i64,
    tempo_votes: usize,
    phase_votes: usize,
}

#[derive(Debug, Clone, Copy)]
struct ZeroOriginGridScore {
    quarter_frames: u64,
    grid_alignment_permille: u64,
    dual_grid_alignment_permille: u64,
    median_grid_residual_permille: u64,
    first_nonzero_grid_residual_frames: u64,
}

#[derive(Debug, Clone, Copy)]
struct PerformanceTimeMapNode {
    nominal_frame: u64,
    offset_frames: i64,
}

#[derive(Debug, Clone)]
struct AudioPerformanceTimeMap {
    summary: AudioPerformanceTimeMapSummary,
    nodes: Vec<PerformanceTimeMapNode>,
}


#[derive(Debug, Clone, Copy)]
pub(crate) struct AudioOnlyAttackProbe {
    pub(crate) frame: u64,
    pub(crate) dual_family: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct AudioOnlyPitchTimingSnapshot {
    pub(crate) sample_rate: u32,
    pub(crate) quarter_frames: u64,
    pub(crate) original_frame_count: u64,
    pub(crate) attacks: Vec<AudioOnlyAttackProbe>,
    performance_time_map: Option<AudioPerformanceTimeMap>,
}

impl AudioOnlyPitchTimingSnapshot {
    pub(crate) fn project_canonical_tick(&self, tick: i64) -> i64 {
        if let Some(map) = self.performance_time_map.as_ref() {
            return project_tick_with_performance_time_map(map, tick);
        }
        let numerator = i128::from(tick).saturating_mul(i128::from(self.quarter_frames));
        let adjusted = if numerator >= 0 {
            numerator.saturating_add(i128::from(CANONICAL_DIVISIONS / 2))
        } else {
            numerator.saturating_sub(i128::from(CANONICAL_DIVISIONS / 2))
        };
        i64::try_from(adjusted / i128::from(CANONICAL_DIVISIONS)).unwrap_or_else(|_| {
            if adjusted.is_negative() {
                i64::MIN
            } else {
                i64::MAX
            }
        })
    }
}

pub(super) fn build_audio_only_pitch_timing_snapshot(
    signal: &DecodedWavSignalAudit,
) -> Result<AudioOnlyPitchTimingSnapshot, &'static str> {
    let audio = build_audio_hypotheses(signal);
    let zero_origin_grid_families = build_zero_origin_grid_families(&audio, signal);
    let Some(family) = zero_origin_grid_families.first().copied() else {
        return Err("zero_origin_grid_unavailable");
    };
    let performance_time_map = build_audio_performance_time_map(
        &audio.pulse_attacks,
        family.refined_quarter_frames,
        signal.original_frame_count,
    );
    let attacks = audio
        .pulse_attacks
        .iter()
        .map(|attack| AudioOnlyAttackProbe {
            frame: attack.frame,
            dual_family: attack.energy_family && attack.transient_family,
        })
        .collect();
    Ok(AudioOnlyPitchTimingSnapshot {
        sample_rate: signal.sample_rate,
        quarter_frames: family.refined_quarter_frames,
        original_frame_count: signal.original_frame_count,
        attacks,
        performance_time_map,
    })
}

pub(crate) fn build_for_score_title(score_title: Option<&str>) -> Option<WavAttackPulseMetricAudit> {
    let signal = match load_decoded_wav_signal_for_score_title(score_title) {
        Ok(signal) => signal,
        Err(reason) => return Some(not_qualified(reason)),
    };

    // Hard anti-leakage boundary: all audio-side hypotheses, including the
    // zero-origin grid refinement and local PerformanceTimeMap, are fully
    // frozen before the source oracle is loaded.
    let audio = build_audio_hypotheses(&signal);
    let zero_origin_grid_families = build_zero_origin_grid_families(&audio, &signal);
    let audio_identifiability_hypotheses =
        zero_origin_grid_hypotheses(&zero_origin_grid_families);
    let performance_time_map = zero_origin_grid_families
        .first()
        .and_then(|family| {
            build_audio_performance_time_map(
                &audio.pulse_attacks,
                family.refined_quarter_frames,
                signal.original_frame_count,
            )
        });

    let legacy_onsets = match load_canonical_legacy_unique_onsets_for_score_title(score_title) {
        Ok(onsets) => onsets,
        Err(reason) => {
            let mut audit = audio.into_audit(
                Vec::new(),
                0,
                zero_origin_grid_families,
                Vec::new(),
                false,
            );
            audit.performance_time_map =
                performance_time_map.as_ref().map(|map| map.summary);
            audit.qualified = false;
            audit.reason = reason;
            return Some(audit);
        }
    };

    let mut evaluations = evaluate_hypotheses(
        &audio.hypotheses,
        &audio.pulse_attacks,
        &legacy_onsets,
        signal.sample_rate,
        signal.original_frame_count,
    );
    evaluations.sort_by(|left, right| {
        right
            .span_coherent
            .cmp(&left.span_coherent)
            .then_with(|| {
                left.span_ratio_permille
                    .abs_diff(1_000)
                    .cmp(&right.span_ratio_permille.abs_diff(1_000))
            })
            .then_with(|| right.ordered_match_40.cmp(&left.ordered_match_40))
            .then_with(|| right.ordered_match_80.cmp(&left.ordered_match_80))
            .then_with(|| right.match_40.cmp(&left.match_40))
            .then_with(|| right.match_80.cmp(&left.match_80))
            .then_with(|| left.p90_residual_frames.cmp(&right.p90_residual_frames))
            .then_with(|| left.median_residual_frames.cmp(&right.median_residual_frames))
            .then_with(|| left.audio_rank.cmp(&right.audio_rank))
    });
    evaluations.truncate(3);

    // This evaluation preserves the already-frozen audio grid rank. The oracle
    // is not allowed to reorder or select these candidates.
    let audio_identifiability_evaluations = evaluate_hypotheses(
        &audio_identifiability_hypotheses,
        &audio.pulse_attacks,
        &legacy_onsets,
        signal.sample_rate,
        signal.original_frame_count,
    );
    let audio_identifiable = audio_identifiability_evaluations
        .first()
        .is_some_and(|evaluation| evaluation.ready);
    let performance_time_map_evaluation = performance_time_map.as_ref().map(|map| {
        evaluate_performance_time_map(
            map,
            &audio.pulse_attacks,
            &legacy_onsets,
            signal.sample_rate,
            signal.original_frame_count,
        )
    });
    let performance_time_map_identifiable = performance_time_map_evaluation
        .as_ref()
        .is_some_and(|evaluation| evaluation.ready);

    let best_ready = evaluations.first().is_some_and(|evaluation| evaluation.ready);
    let qualified = !audio.hypotheses.is_empty() && audio.pulse_attacks.len() >= 8 && best_ready;
    let reason = if audio.hypotheses.is_empty() {
        "no_audio_metric_hypotheses"
    } else if audio.pulse_attacks.len() < 8 {
        "insufficient_attack_evidence"
    } else if qualified {
        "wav_metric_oracle_discoverability_ready"
    } else {
        "wav_metric_oracle_discoverability_not_ready"
    };

    let mut audit = audio
        .into_audit(
            evaluations,
            legacy_onsets.len(),
            zero_origin_grid_families,
            audio_identifiability_evaluations,
            audio_identifiable,
        )
        .with_gate(qualified, reason);
    audit.performance_time_map = performance_time_map.as_ref().map(|map| map.summary);
    audit.performance_time_map_evaluation = performance_time_map_evaluation;
    audit.performance_time_map_identifiable = performance_time_map_identifiable;
    Some(audit)
}


fn not_qualified(reason: &'static str) -> WavAttackPulseMetricAudit {
    WavAttackPulseMetricAudit {
        qualified: false,
        reason,
        sample_rate: 0,
        hop_frames: 0,
        envelope_bins: 0,
        energy_family_peaks: 0,
        transient_family_peaks: 0,
        attack_candidates: 0,
        dual_family_attack_candidates: 0,
        pulse_attack_candidates: 0,
        audio_hypothesis_count: 0,
        audio_top_hypotheses: Vec::new(),
        audio_tempo_families: Vec::new(),
        zero_origin_grid_families: Vec::new(),
        oracle_unique_onsets: 0,
        best_evaluations: Vec::new(),
        audio_identifiability_evaluations: Vec::new(),
        audio_identifiable: false,
        performance_time_map: None,
        performance_time_map_evaluation: None,
        performance_time_map_identifiable: false,
    }
}

#[derive(Debug, Clone)]
struct AudioHypothesisBuild {
    sample_rate: u32,
    hop_frames: u64,
    envelope_bins: usize,
    energy_family_peaks: usize,
    transient_family_peaks: usize,
    attack_candidates: Vec<AttackCandidate>,
    pulse_attacks: Vec<AttackCandidate>,
    hypotheses: Vec<AudioMetricHypothesis>,
}

impl AudioHypothesisBuild {
    fn into_audit(
        self,
        best_evaluations: Vec<MetricHypothesisEvaluation>,
        oracle_unique_onsets: usize,
        zero_origin_grid_families: Vec<AudioZeroOriginGridFamilySummary>,
        audio_identifiability_evaluations: Vec<MetricHypothesisEvaluation>,
        audio_identifiable: bool,
    ) -> WavAttackPulseMetricAudit {
        let audio_top_hypotheses = self
            .hypotheses
            .iter()
            .take(3)
            .map(|hypothesis| summarize_hypothesis(*hypothesis, self.sample_rate))
            .collect();
        let mut seen_tempo_families = BTreeSet::new();
        let audio_tempo_families = self
            .hypotheses
            .iter()
            .filter(|hypothesis| seen_tempo_families.insert(hypothesis.tempo_family_rank))
            .take(3)
            .map(|hypothesis| AudioTempoFamilySummary {
                tempo_family_rank: hypothesis.tempo_family_rank,
                quarter_frames: hypothesis.quarter_frames,
                bpm_milli: bpm_milli(self.sample_rate, hypothesis.quarter_frames),
                tempo_votes: hypothesis.tempo_votes,
            })
            .collect();
        let dual_family_attack_candidates = self
            .attack_candidates
            .iter()
            .filter(|candidate| candidate.energy_family && candidate.transient_family)
            .count();
        WavAttackPulseMetricAudit {
            qualified: false,
            reason: "wav_metric_oracle_discoverability_not_ready",
            sample_rate: self.sample_rate,
            hop_frames: self.hop_frames,
            envelope_bins: self.envelope_bins,
            energy_family_peaks: self.energy_family_peaks,
            transient_family_peaks: self.transient_family_peaks,
            attack_candidates: self.attack_candidates.len(),
            dual_family_attack_candidates,
            pulse_attack_candidates: self.pulse_attacks.len(),
            audio_hypothesis_count: self.hypotheses.len(),
            audio_top_hypotheses,
            audio_tempo_families,
            zero_origin_grid_families,
            oracle_unique_onsets,
            best_evaluations,
            audio_identifiability_evaluations,
            audio_identifiable,
            performance_time_map: None,
            performance_time_map_evaluation: None,
            performance_time_map_identifiable: false,
        }
    }
}

impl WavAttackPulseMetricAudit {
    fn with_gate(mut self, qualified: bool, reason: &'static str) -> Self {
        self.qualified = qualified;
        self.reason = reason;
        self
    }
}


fn build_zero_origin_grid_families(
    audio: &AudioHypothesisBuild,
    signal: &DecodedWavSignalAudit,
) -> Vec<AudioZeroOriginGridFamilySummary> {
    if audio.pulse_attacks.len() < 4 || audio.sample_rate == 0 {
        return Vec::new();
    }

    let mut coarse_families: BTreeMap<usize, (u64, usize)> = BTreeMap::new();
    for hypothesis in &audio.hypotheses {
        coarse_families
            .entry(hypothesis.tempo_family_rank)
            .or_insert((hypothesis.quarter_frames, hypothesis.tempo_votes));
    }

    let first_nonzero = first_nonzero_frame(signal);
    let search_radius = audio.hop_frames.saturating_mul(2).max(1);
    let min_quarter = u64::from(audio.sample_rate) / 4;
    let max_quarter = u64::from(audio.sample_rate).saturating_mul(3) / 2;
    let mut families = Vec::new();

    for (coarse_rank, (coarse_quarter, tempo_votes)) in coarse_families {
        let lower = coarse_quarter
            .saturating_sub(search_radius)
            .max(min_quarter.max(4));
        let upper = coarse_quarter
            .saturating_add(search_radius)
            .min(max_quarter.max(lower));
        let mut best: Option<ZeroOriginGridScore> = None;

        for quarter_frames in lower..=upper {
            let candidate = zero_origin_grid_score(
                &audio.pulse_attacks,
                quarter_frames,
                first_nonzero,
            );
            let replace = best.is_none_or(|current| {
                candidate.grid_alignment_permille > current.grid_alignment_permille
                    || (candidate.grid_alignment_permille == current.grid_alignment_permille
                        && candidate.dual_grid_alignment_permille
                            > current.dual_grid_alignment_permille)
                    || (candidate.grid_alignment_permille == current.grid_alignment_permille
                        && candidate.dual_grid_alignment_permille
                            == current.dual_grid_alignment_permille
                        && candidate.median_grid_residual_permille
                            < current.median_grid_residual_permille)
                    || (candidate.grid_alignment_permille == current.grid_alignment_permille
                        && candidate.dual_grid_alignment_permille
                            == current.dual_grid_alignment_permille
                        && candidate.median_grid_residual_permille
                            == current.median_grid_residual_permille
                        && candidate
                            .quarter_frames
                            .abs_diff(coarse_quarter)
                            < current.quarter_frames.abs_diff(coarse_quarter))
            });
            if replace {
                best = Some(candidate);
            }
        }

        if let Some(best) = best {
            families.push(AudioZeroOriginGridFamilySummary {
                grid_rank: 0,
                coarse_tempo_family_rank: coarse_rank,
                coarse_quarter_frames: coarse_quarter,
                refined_quarter_frames: best.quarter_frames,
                bpm_milli: bpm_milli(audio.sample_rate, best.quarter_frames),
                tempo_votes,
                grid_alignment_permille: best.grid_alignment_permille,
                dual_grid_alignment_permille: best.dual_grid_alignment_permille,
                median_grid_residual_permille: best.median_grid_residual_permille,
                first_nonzero_frame: first_nonzero,
                first_nonzero_grid_residual_frames: best.first_nonzero_grid_residual_frames,
            });
        }
    }

    families.sort_by(|left, right| {
        right
            .grid_alignment_permille
            .cmp(&left.grid_alignment_permille)
            .then_with(|| {
                right
                    .dual_grid_alignment_permille
                    .cmp(&left.dual_grid_alignment_permille)
            })
            .then_with(|| {
                left.median_grid_residual_permille
                    .cmp(&right.median_grid_residual_permille)
            })
            .then_with(|| right.tempo_votes.cmp(&left.tempo_votes))
            .then_with(|| {
                left.coarse_tempo_family_rank
                    .cmp(&right.coarse_tempo_family_rank)
            })
    });
    for (index, family) in families.iter_mut().enumerate() {
        family.grid_rank = index.saturating_add(1);
    }
    families.truncate(3);
    families
}

fn zero_origin_grid_hypotheses(
    families: &[AudioZeroOriginGridFamilySummary],
) -> Vec<AudioMetricHypothesis> {
    families
        .iter()
        .map(|family| AudioMetricHypothesis {
            audio_rank: family.grid_rank,
            tempo_family_rank: family.coarse_tempo_family_rank,
            phase_family_rank: 0,
            quarter_frames: family.refined_quarter_frames,
            origin_frame: 0,
            tempo_votes: family.tempo_votes,
            phase_votes: usize::try_from(family.grid_alignment_permille).unwrap_or(usize::MAX),
        })
        .collect()
}


fn build_audio_performance_time_map(
    attacks: &[AttackCandidate],
    quarter_frames: u64,
    original_frame_count: u64,
) -> Option<AudioPerformanceTimeMap> {
    if quarter_frames < SIXTEENTHS_PER_QUARTER || attacks.len() < 8 || original_frame_count == 0 {
        return None;
    }

    let window_frames = quarter_frames.saturating_mul(PERFORMANCE_MAP_WINDOW_QUARTERS);
    if window_frames == 0 {
        return None;
    }

    let total_windows_u64 = original_frame_count.div_ceil(window_frames);
    let total_windows = usize::try_from(total_windows_u64).unwrap_or(usize::MAX);
    let mut candidate_nodes = Vec::new();
    let mut supported_windows = 0usize;
    let mut dual_supported_windows = 0usize;
    let mut previous_offset = 0i64;
    candidate_nodes.push(PerformanceTimeMapNode {
        nominal_frame: 0,
        offset_frames: 0,
    });

    for window_index in 0..total_windows_u64 {
        let window_start = window_index.saturating_mul(window_frames);
        let window_end = window_start
            .saturating_add(window_frames)
            .min(original_frame_count);
        if window_end <= window_start {
            continue;
        }

        let mut all_residuals = Vec::new();
        let mut dual_residuals = Vec::new();
        for attack in attacks.iter().filter(|attack| {
            attack.frame >= window_start && attack.frame < window_end
        }) {
            let residual = signed_sixteenth_grid_residual(attack.frame, quarter_frames);
            all_residuals.push(residual);
            if attack.energy_family && attack.transient_family {
                dual_residuals.push(residual);
            }
        }

        let (selected, dual_supported) =
            if dual_residuals.len() >= PERFORMANCE_MAP_MIN_ANCHORS_PER_WINDOW {
                (&mut dual_residuals, true)
            } else if all_residuals.len() >= PERFORMANCE_MAP_MIN_ANCHORS_PER_WINDOW {
                (&mut all_residuals, false)
            } else {
                continue;
            };
        selected.sort_unstable();
        let local_residual = signed_percentile(selected, 50);
        let sixteenth_frames = quarter_frames / SIXTEENTHS_PER_QUARTER;
        let unwrapped = unwrap_residual_near(
            local_residual,
            previous_offset,
            i64::try_from(sixteenth_frames.max(1)).unwrap_or(i64::MAX),
        );
        let nominal_frame = window_start.saturating_add(window_end.saturating_sub(window_start) / 2);
        candidate_nodes.push(PerformanceTimeMapNode {
            nominal_frame,
            offset_frames: unwrapped,
        });
        previous_offset = unwrapped;
        supported_windows = supported_windows.saturating_add(1);
        if dual_supported {
            dual_supported_windows = dual_supported_windows.saturating_add(1);
        }
    }

    if candidate_nodes.len() < 3 {
        return None;
    }

    let smoothed = smooth_performance_time_map_nodes(&candidate_nodes);
    let mut nodes = Vec::with_capacity(smoothed.len());
    let mut previous_mapped = None::<i128>;
    for node in smoothed {
        let mapped = i128::from(node.nominal_frame).saturating_add(i128::from(node.offset_frames));
        if mapped < 0 {
            continue;
        }
        if previous_mapped.is_some_and(|previous| mapped <= previous) {
            continue;
        }
        previous_mapped = Some(mapped);
        nodes.push(node);
    }

    if nodes.len() < 3 {
        return None;
    }

    let mut abs_offsets: Vec<u64> = nodes
        .iter()
        .skip(1)
        .map(|node| node.offset_frames.unsigned_abs())
        .collect();
    abs_offsets.sort_unstable();
    let median_abs_offset_frames = percentile(&abs_offsets, 50);
    let p90_abs_offset_frames = percentile(&abs_offsets, 90);
    let max_abs_offset_frames = abs_offsets.last().copied().unwrap_or(0);
    let first_offset_frames = nodes.first().map_or(0, |node| node.offset_frames);
    let last_offset_frames = nodes.last().map_or(0, |node| node.offset_frames);

    Some(AudioPerformanceTimeMap {
        summary: AudioPerformanceTimeMapSummary {
            quarter_frames,
            window_quarters: PERFORMANCE_MAP_WINDOW_QUARTERS,
            total_windows,
            supported_windows,
            dual_supported_windows,
            node_count: nodes.len(),
            monotone: true,
            median_abs_offset_frames,
            p90_abs_offset_frames,
            max_abs_offset_frames,
            first_offset_frames,
            last_offset_frames,
        },
        nodes,
    })
}

fn signed_sixteenth_grid_residual(frame: u64, quarter_frames: u64) -> i64 {
    if quarter_frames == 0 {
        return 0;
    }
    let scaled = i128::from(frame).saturating_mul(i128::from(SIXTEENTHS_PER_QUARTER));
    let quarter = i128::from(quarter_frames);
    let grid_index = scaled.saturating_add(quarter / 2) / quarter;
    let target = grid_index.saturating_mul(quarter);
    let diff_scaled = scaled.saturating_sub(target);
    let rounded = if diff_scaled >= 0 {
        diff_scaled.saturating_add(2) / 4
    } else {
        diff_scaled.saturating_sub(2) / 4
    };
    i64::try_from(rounded).unwrap_or_else(|_| {
        if rounded.is_negative() {
            i64::MIN
        } else {
            i64::MAX
        }
    })
}

fn unwrap_residual_near(residual: i64, previous: i64, period: i64) -> i64 {
    if period <= 0 {
        return residual;
    }
    let base_k = previous.saturating_sub(residual) / period;
    [base_k.saturating_sub(1), base_k, base_k.saturating_add(1)]
        .into_iter()
        .map(|k| residual.saturating_add(k.saturating_mul(period)))
        .min_by_key(|candidate| candidate.abs_diff(previous))
        .unwrap_or(residual)
}

fn signed_percentile(sorted: &[i64], percentile_value: usize) -> i64 {
    if sorted.is_empty() {
        return 0;
    }
    let index = (sorted.len().saturating_sub(1))
        .saturating_mul(percentile_value.min(100))
        / 100;
    sorted[index]
}

fn smooth_performance_time_map_nodes(
    nodes: &[PerformanceTimeMapNode],
) -> Vec<PerformanceTimeMapNode> {
    if nodes.len() <= 3 {
        return nodes.to_vec();
    }
    let mut out = Vec::with_capacity(nodes.len());
    out.push(nodes[0]);
    for index in 1..nodes.len().saturating_sub(1) {
        let mut offsets = [
            nodes[index - 1].offset_frames,
            nodes[index].offset_frames,
            nodes[index + 1].offset_frames,
        ];
        offsets.sort_unstable();
        out.push(PerformanceTimeMapNode {
            nominal_frame: nodes[index].nominal_frame,
            offset_frames: offsets[1],
        });
    }
    if let Some(last) = nodes.last().copied() {
        out.push(last);
    }
    out
}

fn performance_time_map_offset(map: &AudioPerformanceTimeMap, nominal_frame: u64) -> i64 {
    if map.nodes.is_empty() {
        return 0;
    }
    match map
        .nodes
        .binary_search_by_key(&nominal_frame, |node| node.nominal_frame)
    {
        Ok(index) => map.nodes[index].offset_frames,
        Err(0) => map.nodes[0].offset_frames,
        Err(index) if index >= map.nodes.len() => map
            .nodes
            .last()
            .map_or(0, |node| node.offset_frames),
        Err(index) => {
            let left = map.nodes[index - 1];
            let right = map.nodes[index];
            let span = right.nominal_frame.saturating_sub(left.nominal_frame).max(1);
            let position = nominal_frame.saturating_sub(left.nominal_frame);
            let delta = i128::from(right.offset_frames)
                .saturating_sub(i128::from(left.offset_frames));
            let interpolated = i128::from(left.offset_frames).saturating_add(
                delta.saturating_mul(i128::from(position)) / i128::from(span),
            );
            i64::try_from(interpolated).unwrap_or_else(|_| {
                if interpolated.is_negative() {
                    i64::MIN
                } else {
                    i64::MAX
                }
            })
        }
    }
}

fn project_tick_with_performance_time_map(
    map: &AudioPerformanceTimeMap,
    tick: i64,
) -> i64 {
    let numerator = i128::from(tick).saturating_mul(i128::from(map.summary.quarter_frames));
    let adjusted = if numerator >= 0 {
        numerator.saturating_add(i128::from(CANONICAL_DIVISIONS / 2))
    } else {
        numerator.saturating_sub(i128::from(CANONICAL_DIVISIONS / 2))
    };
    let nominal = adjusted / i128::from(CANONICAL_DIVISIONS);
    let nominal_i64 = i64::try_from(nominal).unwrap_or_else(|_| {
        if nominal.is_negative() {
            i64::MIN
        } else {
            i64::MAX
        }
    });
    if nominal_i64 < 0 {
        return nominal_i64;
    }
    let nominal_u64 = u64::try_from(nominal_i64).unwrap_or(u64::MAX);
    nominal_i64.saturating_add(performance_time_map_offset(map, nominal_u64))
}

fn evaluate_performance_time_map(
    map: &AudioPerformanceTimeMap,
    attacks: &[AttackCandidate],
    legacy_onsets: &[i64],
    sample_rate: u32,
    original_frame_count: u64,
) -> PerformanceTimeMapEvaluation {
    let attack_frames: Vec<u64> = attacks.iter().map(|candidate| candidate.frame).collect();
    let t40 = milliseconds_to_frames(sample_rate, 40);
    let t80 = milliseconds_to_frames(sample_rate, 80);
    let mut residuals = Vec::with_capacity(legacy_onsets.len());
    let mut predicted_frames = Vec::with_capacity(legacy_onsets.len());
    let mut match_40 = 0usize;
    let mut match_80 = 0usize;

    for &onset in legacy_onsets {
        let predicted = project_tick_with_performance_time_map(map, onset);
        let residual = if predicted < 0 {
            original_frame_count
        } else {
            let predicted = u64::try_from(predicted).unwrap_or(u64::MAX);
            if predicted >= original_frame_count {
                original_frame_count
            } else {
                predicted_frames.push(predicted);
                nearest_distance(&attack_frames, predicted).unwrap_or(original_frame_count)
            }
        };
        residuals.push(residual);
        if residual <= t40 {
            match_40 = match_40.saturating_add(1);
        }
        if residual <= t80 {
            match_80 = match_80.saturating_add(1);
        }
    }

    residuals.sort_unstable();
    predicted_frames.sort_unstable();
    let median_residual_frames = percentile(&residuals, 50);
    let p90_residual_frames = percentile(&residuals, 90);
    let ordered_match_40 = ordered_unique_match_count(&predicted_frames, &attack_frames, t40);
    let ordered_match_80 = ordered_unique_match_count(&predicted_frames, &attack_frames, t80);
    let projected_span_frames = projected_frames_span(&predicted_frames);
    let pulse_attack_span_frames = projected_frames_span(&attack_frames);
    let span_ratio_permille = ratio_permille(projected_span_frames, pulse_attack_span_frames);
    let onset_count = legacy_onsets.len();
    let span_coherent = (800..=1_250).contains(&span_ratio_permille);
    let ready = onset_count >= 8
        && span_coherent
        && percent_at_least(match_40, onset_count, READY_MATCH_40_PERCENT)
        && percent_at_least(match_80, onset_count, READY_MATCH_80_PERCENT)
        && frames_to_milliseconds(sample_rate, median_residual_frames) <= READY_MEDIAN_MS
        && frames_to_milliseconds(sample_rate, p90_residual_frames) <= READY_P90_MS;

    PerformanceTimeMapEvaluation {
        match_40,
        match_80,
        ordered_match_40,
        ordered_match_80,
        median_residual_frames,
        p90_residual_frames,
        projected_span_frames,
        pulse_attack_span_frames,
        span_ratio_permille,
        oracle_unique_onsets: onset_count,
        ready,
    }
}

fn first_nonzero_frame(signal: &DecodedWavSignalAudit) -> Option<u64> {
    let channels = usize::from(signal.channels).max(1);
    signal
        .interleaved_samples
        .chunks_exact(channels)
        .position(|frame| frame.iter().any(|sample| *sample != 0.0))
        .and_then(|index| u64::try_from(index).ok())
}

fn zero_origin_grid_score(
    attacks: &[AttackCandidate],
    quarter_frames: u64,
    first_nonzero: Option<u64>,
) -> ZeroOriginGridScore {
    if attacks.is_empty() || quarter_frames < 4 {
        return ZeroOriginGridScore {
            quarter_frames,
            grid_alignment_permille: 0,
            dual_grid_alignment_permille: 0,
            median_grid_residual_permille: u64::MAX,
            first_nonzero_grid_residual_frames: u64::MAX,
        };
    }

    let mut normalized_residuals = Vec::with_capacity(attacks.len());
    let mut alignment_sum = 0u128;
    let mut dual_alignment_sum = 0u128;
    let mut dual_count = 0u64;

    for attack in attacks {
        let residual_frames = sixteenth_grid_distance(attack.frame, quarter_frames);
        let normalized = normalized_grid_residual_permille(residual_frames, quarter_frames);
        normalized_residuals.push(normalized);
        let alignment = 1_000u64.saturating_sub(normalized);
        alignment_sum = alignment_sum.saturating_add(u128::from(alignment));
        if attack.energy_family && attack.transient_family {
            dual_alignment_sum = dual_alignment_sum.saturating_add(u128::from(alignment));
            dual_count = dual_count.saturating_add(1);
        }
    }

    normalized_residuals.sort_unstable();
    let count = u128::try_from(attacks.len()).unwrap_or(u128::MAX).max(1);
    let grid_alignment_permille =
        u64::try_from((alignment_sum.saturating_add(count / 2)) / count).unwrap_or(u64::MAX);
    let dual_grid_alignment_permille = if dual_count == 0 {
        grid_alignment_permille
    } else {
        let denominator = u128::from(dual_count);
        u64::try_from(
            (dual_alignment_sum.saturating_add(denominator / 2)) / denominator,
        )
        .unwrap_or(u64::MAX)
    };
    let first_nonzero_grid_residual_frames = first_nonzero
        .map(|frame| sixteenth_grid_distance(frame, quarter_frames))
        .unwrap_or(u64::MAX);

    ZeroOriginGridScore {
        quarter_frames,
        grid_alignment_permille,
        dual_grid_alignment_permille,
        median_grid_residual_permille: percentile(&normalized_residuals, 50),
        first_nonzero_grid_residual_frames,
    }
}

fn normalized_grid_residual_permille(residual_frames: u64, quarter_frames: u64) -> u64 {
    if quarter_frames == 0 {
        return u64::MAX;
    }
    residual_frames
        .saturating_mul(8_000)
        .saturating_add(quarter_frames / 2)
        / quarter_frames
}

fn sixteenth_grid_distance(frame: u64, quarter_frames: u64) -> u64 {
    if quarter_frames == 0 {
        return u64::MAX;
    }
    let scaled_frame = u128::from(frame).saturating_mul(u128::from(SIXTEENTHS_PER_QUARTER));
    let quarter = u128::from(quarter_frames);
    let grid_index = scaled_frame.saturating_add(quarter / 2) / quarter;
    let target_scaled = grid_index.saturating_mul(quarter);
    let diff_scaled = scaled_frame.abs_diff(target_scaled);
    u64::try_from(diff_scaled.saturating_add(2) / 4).unwrap_or(u64::MAX)
}

fn build_audio_hypotheses(signal: &DecodedWavSignalAudit) -> AudioHypothesisBuild {
    let sample_rate = signal.sample_rate;
    let hop_frames = u64::from((sample_rate / 200).max(1));
    let envelopes = build_envelopes(signal, usize::try_from(hop_frames).unwrap_or(1));
    let duration_seconds = if sample_rate == 0 {
        0usize
    } else {
        usize::try_from(signal.original_frame_count / u64::from(sample_rate)).unwrap_or(0)
    };
    let family_capacity = duration_seconds
        .saturating_mul(MAX_LOCAL_PEAKS_PER_SECOND_PER_FAMILY)
        .max(MIN_PEAK_CAPACITY_PER_FAMILY);

    let energy_rise = positive_rise(&envelopes, |bin| bin.energy);
    let transient_rise = positive_rise(&envelopes, |bin| bin.transient);
    let energy_peaks = strongest_local_peaks(&energy_rise, family_capacity);
    let transient_peaks = strongest_local_peaks(&transient_rise, family_capacity);
    let attack_candidates = merge_peak_families(&energy_peaks, &transient_peaks, hop_frames);
    let pulse_attacks = collapse_for_pulse(&attack_candidates, hop_frames.saturating_mul(3));
    let hypotheses = build_metric_hypotheses(&pulse_attacks, sample_rate, hop_frames);

    AudioHypothesisBuild {
        sample_rate,
        hop_frames,
        envelope_bins: envelopes.len(),
        energy_family_peaks: energy_peaks.len(),
        transient_family_peaks: transient_peaks.len(),
        attack_candidates,
        pulse_attacks,
        hypotheses,
    }
}

fn build_envelopes(signal: &DecodedWavSignalAudit, hop_frames: usize) -> Vec<EnvelopeBin> {
    let channels = usize::from(signal.channels).max(1);
    let frame_count = signal.interleaved_samples.len() / channels;
    let mut out = Vec::with_capacity(frame_count.div_ceil(hop_frames));
    let mut previous = vec![0.0f32; channels];
    let mut frame_start = 0usize;

    while frame_start < frame_count {
        let frame_end = frame_start.saturating_add(hop_frames).min(frame_count);
        let mut energy_sum = 0.0f64;
        let mut transient_sum = 0.0f64;
        let mut sample_count = 0usize;
        for frame_index in frame_start..frame_end {
            let base = frame_index.saturating_mul(channels);
            for (channel, previous_sample) in previous.iter_mut().enumerate().take(channels) {
                let sample = signal.interleaved_samples[base + channel];
                energy_sum += f64::from(sample.abs());
                transient_sum += f64::from((sample - *previous_sample).abs());
                *previous_sample = sample;
                sample_count = sample_count.saturating_add(1);
            }
        }
        let denominator = sample_count.max(1) as f64;
        out.push(EnvelopeBin {
            energy: energy_sum / denominator,
            transient: transient_sum / denominator,
        });
        frame_start = frame_end;
    }
    out
}

fn positive_rise<F>(bins: &[EnvelopeBin], selector: F) -> Vec<f64>
where
    F: Fn(EnvelopeBin) -> f64,
{
    let mut out = vec![0.0f64; bins.len()];
    for index in 1..bins.len() {
        out[index] = (selector(bins[index]) - selector(bins[index - 1])).max(0.0);
    }
    out
}

fn strongest_local_peaks(values: &[f64], capacity: usize) -> Vec<Peak> {
    if values.len() < 5 {
        return Vec::new();
    }
    let mut peaks = Vec::new();
    for index in 2..values.len() - 2 {
        let value = values[index];
        if value <= 0.0 {
            continue;
        }
        if value >= values[index - 1]
            && value >= values[index - 2]
            && value > values[index + 1]
            && value >= values[index + 2]
        {
            peaks.push(Peak { bin: index, value });
        }
    }
    peaks.sort_by(|left, right| {
        right
            .value
            .total_cmp(&left.value)
            .then_with(|| left.bin.cmp(&right.bin))
    });
    peaks.truncate(capacity.min(peaks.len()));
    peaks.sort_by_key(|peak| peak.bin);
    peaks
}

fn merge_peak_families(
    energy_peaks: &[Peak],
    transient_peaks: &[Peak],
    hop_frames: u64,
) -> Vec<AttackCandidate> {
    let energy_bins: BTreeSet<usize> = energy_peaks.iter().map(|peak| peak.bin).collect();
    let transient_bins: BTreeSet<usize> = transient_peaks.iter().map(|peak| peak.bin).collect();
    let all_bins: BTreeSet<usize> = energy_bins.union(&transient_bins).copied().collect();
    all_bins
        .into_iter()
        .map(|bin| AttackCandidate {
            frame: u64::try_from(bin)
                .unwrap_or(u64::MAX)
                .saturating_mul(hop_frames)
                .saturating_add(hop_frames / 2),
            energy_family: has_neighboring_bin(&energy_bins, bin),
            transient_family: has_neighboring_bin(&transient_bins, bin),
        })
        .collect()
}

fn has_neighboring_bin(set: &BTreeSet<usize>, bin: usize) -> bool {
    set.contains(&bin)
        || bin.checked_sub(1).is_some_and(|neighbor| set.contains(&neighbor))
        || bin.checked_add(1).is_some_and(|neighbor| set.contains(&neighbor))
}

fn collapse_for_pulse(candidates: &[AttackCandidate], radius_frames: u64) -> Vec<AttackCandidate> {
    if candidates.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut cluster = Vec::new();
    for candidate in candidates {
        if cluster
            .last()
            .is_some_and(|previous: &&AttackCandidate| candidate.frame.saturating_sub(previous.frame) <= radius_frames)
        {
            cluster.push(candidate);
        } else {
            if let Some(selected) = select_cluster_candidate(&cluster) {
                out.push(selected);
            }
            cluster.clear();
            cluster.push(candidate);
        }
    }
    if let Some(selected) = select_cluster_candidate(&cluster) {
        out.push(selected);
    }
    out
}

fn select_cluster_candidate(cluster: &[&AttackCandidate]) -> Option<AttackCandidate> {
    cluster
        .iter()
        .copied()
        .copied()
        .min_by_key(|candidate| {
            let family_priority = match (candidate.energy_family, candidate.transient_family) {
                (true, true) => 0u8,
                (false, true) => 1,
                (true, false) => 2,
                (false, false) => 3,
            };
            (family_priority, candidate.frame)
        })
}

fn build_metric_hypotheses(
    attacks: &[AttackCandidate],
    sample_rate: u32,
    hop_frames: u64,
) -> Vec<AudioMetricHypothesis> {
    if attacks.len() < 4 || sample_rate == 0 {
        return Vec::new();
    }
    let min_quarter = u64::from(sample_rate) / 4; // 240 BPM
    let max_quarter = u64::from(sample_rate).saturating_mul(3) / 2; // 40 BPM
    let min_pair = u64::from(sample_rate) / 20; // 50 ms
    let max_pair = u64::from(sample_rate).saturating_mul(2); // 2 s
    let tempo_bin = hop_frames.saturating_mul(2).max(1);
    let factors: &[(u64, u64)] = &[(1, 4), (1, 2), (1, 1), (2, 1), (4, 1), (8, 1)];
    let mut votes: BTreeMap<u64, usize> = BTreeMap::new();

    for index in 0..attacks.len() {
        let end = attacks.len().min(index.saturating_add(MAX_PULSE_PAIR_DISTANCE + 1));
        for next in index + 1..end {
            let diff = attacks[next].frame.saturating_sub(attacks[index].frame);
            if diff < min_pair || diff > max_pair {
                continue;
            }
            for &(numerator, denominator) in factors {
                let scaled = diff.saturating_mul(numerator) / denominator;
                if scaled < min_quarter || scaled > max_quarter {
                    continue;
                }
                let bin = scaled / tempo_bin;
                let entry = votes.entry(bin).or_default();
                *entry = entry.saturating_add(1);
            }
        }
    }

    let mut tempo_families: Vec<(u64, usize)> = votes.into_iter().collect();
    tempo_families.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    tempo_families.truncate(MAX_TEMPO_FAMILIES);

    let first_attack = attacks.first().map_or(0, |candidate| candidate.frame);
    let mut hypotheses = Vec::new();
    for (tempo_index, (tempo_bin_index, tempo_votes)) in tempo_families.into_iter().enumerate() {
        let quarter_frames = tempo_bin_index
            .saturating_mul(tempo_bin)
            .saturating_add(tempo_bin / 2)
            .max(min_quarter);
        let subdivision = (quarter_frames / SIXTEENTHS_PER_QUARTER).max(1);
        let phase_bin_width = hop_frames.max(1).min(subdivision);
        let mut phase_votes_map: BTreeMap<u64, usize> = BTreeMap::new();
        for attack in attacks {
            let remainder = attack.frame % subdivision;
            let phase_bin = remainder / phase_bin_width;
            let entry = phase_votes_map.entry(phase_bin).or_default();
            *entry = entry.saturating_add(1);
        }
        let mut phase_families: Vec<(u64, usize)> = phase_votes_map.into_iter().collect();
        phase_families.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        phase_families.truncate(MAX_PHASE_FAMILIES_PER_TEMPO);

        for (phase_index, (phase_bin, phase_votes)) in phase_families.into_iter().enumerate() {
            let phase = phase_bin
                .saturating_mul(phase_bin_width)
                .saturating_add(phase_bin_width / 2)
                % subdivision;
            let lower_bound = -i64::try_from(quarter_frames.saturating_mul(4)).unwrap_or(i64::MAX);
            let upper_bound = i64::try_from(first_attack).unwrap_or(i64::MAX);
            let subdivision_i64 = i64::try_from(subdivision).unwrap_or(i64::MAX).max(1);
            let phase_i64 = i64::try_from(phase).unwrap_or(0);
            let mut origin = phase_i64;
            while origin > upper_bound {
                origin = origin.saturating_sub(subdivision_i64);
            }
            while origin.saturating_sub(subdivision_i64) >= lower_bound {
                origin = origin.saturating_sub(subdivision_i64);
            }
            while origin <= upper_bound && hypotheses.len() < MAX_AUDIO_HYPOTHESES {
                hypotheses.push(AudioMetricHypothesis {
                    audio_rank: 0,
                    tempo_family_rank: tempo_index.saturating_add(1),
                    phase_family_rank: phase_index.saturating_add(1),
                    quarter_frames,
                    origin_frame: origin,
                    tempo_votes,
                    phase_votes,
                });
                origin = origin.saturating_add(subdivision_i64);
            }
        }
        if hypotheses.len() >= MAX_AUDIO_HYPOTHESES {
            break;
        }
    }

    hypotheses.sort_by(|left, right| {
        right
            .tempo_votes
            .cmp(&left.tempo_votes)
            .then_with(|| right.phase_votes.cmp(&left.phase_votes))
            .then_with(|| left.tempo_family_rank.cmp(&right.tempo_family_rank))
            .then_with(|| left.phase_family_rank.cmp(&right.phase_family_rank))
            .then_with(|| left.quarter_frames.cmp(&right.quarter_frames))
            .then_with(|| left.origin_frame.cmp(&right.origin_frame))
    });
    for (index, hypothesis) in hypotheses.iter_mut().enumerate() {
        hypothesis.audio_rank = index.saturating_add(1);
    }
    hypotheses
}

fn evaluate_hypotheses(
    hypotheses: &[AudioMetricHypothesis],
    attacks: &[AttackCandidate],
    legacy_onsets: &[i64],
    sample_rate: u32,
    original_frame_count: u64,
) -> Vec<MetricHypothesisEvaluation> {
    let attack_frames: Vec<u64> = attacks.iter().map(|candidate| candidate.frame).collect();
    hypotheses
        .iter()
        .map(|hypothesis| {
            evaluate_hypothesis(
                *hypothesis,
                &attack_frames,
                legacy_onsets,
                sample_rate,
                original_frame_count,
            )
        })
        .collect()
}

fn evaluate_hypothesis(
    hypothesis: AudioMetricHypothesis,
    attack_frames: &[u64],
    legacy_onsets: &[i64],
    sample_rate: u32,
    original_frame_count: u64,
) -> MetricHypothesisEvaluation {
    let t10 = milliseconds_to_frames(sample_rate, 10);
    let t20 = milliseconds_to_frames(sample_rate, 20);
    let t40 = milliseconds_to_frames(sample_rate, 40);
    let t80 = milliseconds_to_frames(sample_rate, 80);
    let mut residuals = Vec::with_capacity(legacy_onsets.len());
    let mut predicted_frames = Vec::with_capacity(legacy_onsets.len());
    let mut match_10 = 0usize;
    let mut match_20 = 0usize;
    let mut match_40 = 0usize;
    let mut match_80 = 0usize;

    for &onset in legacy_onsets {
        let predicted = project_tick_to_frame(hypothesis, onset);
        let residual = if predicted < 0 {
            original_frame_count
        } else {
            let predicted = u64::try_from(predicted).unwrap_or(u64::MAX);
            if predicted >= original_frame_count {
                original_frame_count
            } else {
                predicted_frames.push(predicted);
                nearest_distance(attack_frames, predicted).unwrap_or(original_frame_count)
            }
        };
        residuals.push(residual);
        if residual <= t10 {
            match_10 = match_10.saturating_add(1);
        }
        if residual <= t20 {
            match_20 = match_20.saturating_add(1);
        }
        if residual <= t40 {
            match_40 = match_40.saturating_add(1);
        }
        if residual <= t80 {
            match_80 = match_80.saturating_add(1);
        }
    }

    residuals.sort_unstable();
    predicted_frames.sort_unstable();
    let median_residual_frames = percentile(&residuals, 50);
    let p90_residual_frames = percentile(&residuals, 90);
    let attack_explained_40 = attack_frames
        .iter()
        .filter(|&&frame| nearest_distance(&predicted_frames, frame).is_some_and(|distance| distance <= t40))
        .count();
    let onset_count = legacy_onsets.len();
    let ordered_match_40 = ordered_unique_match_count(&predicted_frames, attack_frames, t40);
    let ordered_match_80 = ordered_unique_match_count(&predicted_frames, attack_frames, t80);
    let projected_legacy_span_frames = legacy_metric_span_frames(legacy_onsets, hypothesis.quarter_frames);
    let pulse_attack_span_frames = projected_frames_span(attack_frames);
    let span_ratio_permille = ratio_permille(projected_legacy_span_frames, pulse_attack_span_frames);
    // Broad diagnostic coherence bound only. It rejects globally compressed/expanded
    // timelines while leaving local performance timing and terminal-tail uncertainty intact.
    let span_coherent = (800..=1_250).contains(&span_ratio_permille);
    let ready = onset_count >= 8
        && span_coherent
        && percent_at_least(match_40, onset_count, READY_MATCH_40_PERCENT)
        && percent_at_least(match_80, onset_count, READY_MATCH_80_PERCENT)
        && frames_to_milliseconds(sample_rate, median_residual_frames) <= READY_MEDIAN_MS
        && frames_to_milliseconds(sample_rate, p90_residual_frames) <= READY_P90_MS;
    let bpm_milli = bpm_milli(sample_rate, hypothesis.quarter_frames);

    MetricHypothesisEvaluation {
        audio_rank: hypothesis.audio_rank,
        tempo_family_rank: hypothesis.tempo_family_rank,
        phase_family_rank: hypothesis.phase_family_rank,
        quarter_frames: hypothesis.quarter_frames,
        bpm_milli,
        origin_frame: hypothesis.origin_frame,
        tempo_votes: hypothesis.tempo_votes,
        phase_votes: hypothesis.phase_votes,
        match_10,
        match_20,
        match_40,
        match_80,
        median_residual_frames,
        p90_residual_frames,
        attack_explained_40,
        ordered_match_40,
        ordered_match_80,
        projected_legacy_span_frames,
        pulse_attack_span_frames,
        span_ratio_permille,
        span_coherent,
        oracle_unique_onsets: onset_count,
        attack_candidates: attack_frames.len(),
        ready,
    }
}

fn project_tick_to_frame(hypothesis: AudioMetricHypothesis, tick: i64) -> i64 {
    let numerator = i128::from(tick).saturating_mul(i128::from(hypothesis.quarter_frames));
    let adjusted = if numerator >= 0 {
        numerator.saturating_add(i128::from(CANONICAL_DIVISIONS / 2))
    } else {
        numerator.saturating_sub(i128::from(CANONICAL_DIVISIONS / 2))
    };
    let rounded = adjusted / i128::from(CANONICAL_DIVISIONS);
    let projected = i128::from(hypothesis.origin_frame).saturating_add(rounded);
    i64::try_from(projected).unwrap_or_else(|_| if projected.is_negative() { i64::MIN } else { i64::MAX })
}

fn nearest_distance(sorted: &[u64], value: u64) -> Option<u64> {
    if sorted.is_empty() {
        return None;
    }
    match sorted.binary_search(&value) {
        Ok(_) => Some(0),
        Err(index) => {
            let left = index.checked_sub(1).and_then(|pos| sorted.get(pos)).copied();
            let right = sorted.get(index).copied();
            match (left, right) {
                (Some(left), Some(right)) => Some(value.abs_diff(left).min(value.abs_diff(right))),
                (Some(left), None) => Some(value.abs_diff(left)),
                (None, Some(right)) => Some(value.abs_diff(right)),
                (None, None) => None,
            }
        }
    }
}

fn bpm_milli(sample_rate: u32, quarter_frames: u64) -> u64 {
    if quarter_frames == 0 {
        return 0;
    }
    u64::from(sample_rate)
        .saturating_mul(60_000)
        .saturating_add(quarter_frames / 2)
        / quarter_frames
}

fn summarize_hypothesis(
    hypothesis: AudioMetricHypothesis,
    sample_rate: u32,
) -> AudioMetricHypothesisSummary {
    AudioMetricHypothesisSummary {
        audio_rank: hypothesis.audio_rank,
        tempo_family_rank: hypothesis.tempo_family_rank,
        phase_family_rank: hypothesis.phase_family_rank,
        quarter_frames: hypothesis.quarter_frames,
        bpm_milli: bpm_milli(sample_rate, hypothesis.quarter_frames),
        origin_frame: hypothesis.origin_frame,
        tempo_votes: hypothesis.tempo_votes,
        phase_votes: hypothesis.phase_votes,
    }
}

fn legacy_metric_span_frames(legacy_onsets: &[i64], quarter_frames: u64) -> u64 {
    let Some(first) = legacy_onsets.first().copied() else {
        return 0;
    };
    let Some(last) = legacy_onsets.last().copied() else {
        return 0;
    };
    let tick_span = last.abs_diff(first);
    tick_span
        .saturating_mul(quarter_frames)
        .saturating_add(u64::try_from(CANONICAL_DIVISIONS / 2).unwrap_or(0))
        / u64::try_from(CANONICAL_DIVISIONS).unwrap_or(1)
}

fn projected_frames_span(sorted: &[u64]) -> u64 {
    match (sorted.first(), sorted.last()) {
        (Some(first), Some(last)) => last.saturating_sub(*first),
        _ => 0,
    }
}

fn ratio_permille(numerator: u64, denominator: u64) -> u64 {
    if denominator == 0 {
        return u64::MAX;
    }
    numerator
        .saturating_mul(1_000)
        .saturating_add(denominator / 2)
        / denominator
}

fn ordered_unique_match_count(predicted: &[u64], observed: &[u64], tolerance: u64) -> usize {
    if predicted.is_empty() || observed.is_empty() {
        return 0;
    }
    let mut observed_index = 0usize;
    let mut matched = 0usize;
    for &target in predicted {
        let lower = target.saturating_sub(tolerance);
        while observed_index < observed.len() && observed[observed_index] < lower {
            observed_index = observed_index.saturating_add(1);
        }
        if observed_index >= observed.len() {
            break;
        }
        if observed[observed_index].abs_diff(target) <= tolerance {
            matched = matched.saturating_add(1);
            observed_index = observed_index.saturating_add(1);
        }
    }
    matched
}

fn percentile(sorted: &[u64], percentile: usize) -> u64 {
    if sorted.is_empty() {
        return u64::MAX;
    }
    let index = (sorted.len().saturating_sub(1))
        .saturating_mul(percentile.min(100))
        / 100;
    sorted[index]
}

fn milliseconds_to_frames(sample_rate: u32, milliseconds: u64) -> u64 {
    u64::from(sample_rate)
        .saturating_mul(milliseconds)
        .saturating_add(500)
        / 1_000
}

pub(crate) fn frames_to_milliseconds(sample_rate: u32, frames: u64) -> u64 {
    if sample_rate == 0 {
        return u64::MAX;
    }
    frames
        .saturating_mul(1_000)
        .saturating_add(u64::from(sample_rate) / 2)
        / u64::from(sample_rate)
}

fn percent_at_least(count: usize, total: usize, required_percent: usize) -> bool {
    if total == 0 {
        return false;
    }
    count.saturating_mul(100) >= total.saturating_mul(required_percent)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synthetic_pulse_signal() -> DecodedWavSignalAudit {
        let sample_rate = 1_000u32;
        let frame_count = 6_000usize;
        let mut samples = vec![0.0f32; frame_count];
        for frame in (500..5_501).step_by(125) {
            samples[frame] = 1.0;
            if frame + 1 < samples.len() {
                samples[frame + 1] = -0.5;
            }
        }
        DecodedWavSignalAudit {
            sample_rate,
            channels: 1,
            original_frame_count: u64::try_from(frame_count).unwrap(),
            interleaved_samples: samples,
        }
    }

    #[test]
    fn audio_hypotheses_are_built_without_legacy_oracle_input() {
        let signal = synthetic_pulse_signal();
        let build = build_audio_hypotheses(&signal);
        assert!(!build.pulse_attacks.is_empty());
        assert!(!build.hypotheses.is_empty());
        assert!(build.hypotheses.iter().any(|hypothesis| {
            hypothesis.quarter_frames.abs_diff(500) <= build.hop_frames.saturating_mul(4)
        }));
    }

    #[test]
    fn oracle_evaluation_is_separate_and_can_recognize_aligned_grid() {
        let attacks: Vec<AttackCandidate> = (500u64..=5_500)
            .step_by(125)
            .map(|frame| AttackCandidate {
                frame,
                energy_family: true,
                transient_family: true,
            })
            .collect();
        let onsets: Vec<i64> = (96i64..=1_056).step_by(24).collect();
        let hypothesis = AudioMetricHypothesis {
            audio_rank: 1,
            tempo_family_rank: 1,
            phase_family_rank: 1,
            quarter_frames: 500,
            origin_frame: 0,
            tempo_votes: 10,
            phase_votes: 10,
        };
        let evaluation = evaluate_hypothesis(hypothesis, &attacks.iter().map(|a| a.frame).collect::<Vec<_>>(), &onsets, 1_000, 6_000);
        assert_eq!(evaluation.match_10, onsets.len());
        assert!(evaluation.span_coherent);
        assert_eq!(evaluation.ordered_match_40, onsets.len());
        assert!(evaluation.ready);
    }

    #[test]
    fn globally_compressed_timeline_is_not_ready_even_when_local_attacks_are_dense() {
        let attacks: Vec<u64> = (500u64..=5_500).step_by(125).collect();
        let onsets: Vec<i64> = (96i64..=1_056).step_by(24).collect();
        let hypothesis = AudioMetricHypothesis {
            audio_rank: 1,
            tempo_family_rank: 1,
            phase_family_rank: 1,
            quarter_frames: 300,
            origin_frame: 200,
            tempo_votes: 10,
            phase_votes: 10,
        };
        let evaluation = evaluate_hypothesis(hypothesis, &attacks, &onsets, 1_000, 6_000);
        assert!(!evaluation.span_coherent);
        assert!(!evaluation.ready);
    }
    #[test]
    fn zero_origin_grid_refinement_recovers_unbinned_quarter() {
        let attacks: Vec<AttackCandidate> = (0u64..=5_000)
            .step_by(125)
            .map(|frame| AttackCandidate {
                frame,
                energy_family: true,
                transient_family: true,
            })
            .collect();
        let signal = DecodedWavSignalAudit {
            sample_rate: 1_000,
            channels: 1,
            original_frame_count: 6_000,
            interleaved_samples: vec![0.0; 6_000],
        };
        let audio = AudioHypothesisBuild {
            sample_rate: 1_000,
            hop_frames: 5,
            envelope_bins: 0,
            energy_family_peaks: attacks.len(),
            transient_family_peaks: attacks.len(),
            attack_candidates: attacks.clone(),
            pulse_attacks: attacks,
            hypotheses: vec![AudioMetricHypothesis {
                audio_rank: 1,
                tempo_family_rank: 1,
                phase_family_rank: 1,
                quarter_frames: 510,
                origin_frame: 0,
                tempo_votes: 100,
                phase_votes: 100,
            }],
        };
        let families = build_zero_origin_grid_families(&audio, &signal);
        assert_eq!(families.len(), 1);
        assert_eq!(families[0].refined_quarter_frames, 500);
        assert_eq!(families[0].grid_alignment_permille, 1_000);
    }

    #[test]
    fn zero_origin_grid_ranking_can_override_nearby_raw_vote_alias() {
        let attacks: Vec<AttackCandidate> = (0u64..=5_000)
            .step_by(125)
            .map(|frame| AttackCandidate {
                frame,
                energy_family: true,
                transient_family: true,
            })
            .collect();
        let signal = DecodedWavSignalAudit {
            sample_rate: 1_000,
            channels: 1,
            original_frame_count: 6_000,
            interleaved_samples: vec![0.0; 6_000],
        };
        let audio = AudioHypothesisBuild {
            sample_rate: 1_000,
            hop_frames: 5,
            envelope_bins: 0,
            energy_family_peaks: attacks.len(),
            transient_family_peaks: attacks.len(),
            attack_candidates: attacks.clone(),
            pulse_attacks: attacks,
            hypotheses: vec![
                AudioMetricHypothesis {
                    audio_rank: 1,
                    tempo_family_rank: 1,
                    phase_family_rank: 1,
                    quarter_frames: 385,
                    origin_frame: 0,
                    tempo_votes: 101,
                    phase_votes: 100,
                },
                AudioMetricHypothesis {
                    audio_rank: 2,
                    tempo_family_rank: 2,
                    phase_family_rank: 1,
                    quarter_frames: 510,
                    origin_frame: 0,
                    tempo_votes: 100,
                    phase_votes: 100,
                },
            ],
        };
        let families = build_zero_origin_grid_families(&audio, &signal);
        assert_eq!(families.len(), 2);
        assert_eq!(families[0].coarse_tempo_family_rank, 2);
        assert_eq!(families[0].refined_quarter_frames, 500);
    }

    #[test]
    fn zero_origin_grid_hypotheses_keep_project_origin_fixed() {
        let families = vec![AudioZeroOriginGridFamilySummary {
            grid_rank: 1,
            coarse_tempo_family_rank: 2,
            coarse_quarter_frames: 510,
            refined_quarter_frames: 500,
            bpm_milli: 120_000,
            tempo_votes: 10,
            grid_alignment_permille: 900,
            dual_grid_alignment_permille: 910,
            median_grid_residual_permille: 100,
            first_nonzero_frame: Some(500),
            first_nonzero_grid_residual_frames: 0,
        }];
        let hypotheses = zero_origin_grid_hypotheses(&families);
        assert_eq!(hypotheses.len(), 1);
        assert_eq!(hypotheses[0].origin_frame, 0);
        assert_eq!(hypotheses[0].audio_rank, 1);
        assert_eq!(hypotheses[0].quarter_frames, 500);
    }


    #[test]
    fn performance_time_map_is_audio_only_and_tracks_gradual_timing_residual() {
        let attacks: Vec<AttackCandidate> = (0u64..=5_000)
            .step_by(125)
            .map(|nominal| AttackCandidate {
                frame: nominal.saturating_add(nominal / 50),
                energy_family: true,
                transient_family: true,
            })
            .collect();
        let map = build_audio_performance_time_map(&attacks, 500, 6_200)
            .expect("audio-only performance map");
        assert!(map.summary.monotone);
        assert!(map.summary.node_count >= 3);
        assert_eq!(map.nodes[0].nominal_frame, 0);
        assert_eq!(map.nodes[0].offset_frames, 0);

        let onsets: Vec<i64> = (0i64..=960).step_by(24).collect();
        let evaluation =
            evaluate_performance_time_map(&map, &attacks, &onsets, 1_000, 6_200);
        assert!(evaluation.ready);
        assert!(frames_to_milliseconds(1_000, evaluation.p90_residual_frames) <= 80);
    }

    #[test]
    fn performance_time_map_projection_preserves_monotone_metric_order() {
        let attacks: Vec<AttackCandidate> = (0u64..=8_000)
            .step_by(125)
            .map(|nominal| AttackCandidate {
                frame: nominal.saturating_add(nominal / 80),
                energy_family: true,
                transient_family: true,
            })
            .collect();
        let map = build_audio_performance_time_map(&attacks, 500, 8_500)
            .expect("audio-only performance map");
        let mut previous = i64::MIN;
        for tick in (0i64..=1_536).step_by(24) {
            let projected = project_tick_with_performance_time_map(&map, tick);
            assert!(projected > previous);
            previous = projected;
        }
    }

}
