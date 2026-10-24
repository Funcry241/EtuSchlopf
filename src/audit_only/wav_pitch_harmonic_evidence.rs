//! Audit-only WAV pitch and harmonic evidence probe.
//!
//! The complete audio-side probe set and per-attack pitch rankings are frozen
//! before the frozen legacy source oracle is loaded from productive MIDI. The oracle is evaluation only.
//! No pitch candidate is promoted into CanonicalSourceEvents.

use crate::audit_only::wav_attack_pulse_metric::{
    build_audio_only_pitch_timing_snapshot, AudioOnlyAttackProbe, AudioOnlyPitchTimingSnapshot,
};
use crate::audit_only::wav_observation_legacy_oracle::{
    load_canonical_legacy_pitch_events_for_score_title, CanonicalLegacyPitchEventAudit,
};
use crate::audit_only::wav_signal_decode::{
    load_decoded_wav_signal_for_score_title, DecodedWavSignalAudit,
};
use std::collections::{BTreeMap, BTreeSet};

const PIANO_MIDI_MIN: i32 = 21;
const PIANO_MIDI_MAX: i32 = 108;
const MAX_PROBE_ATTACKS: usize = 192;
const TOP_EVIDENCE_ROWS: usize = 12;
const EVALUATION_MATCH_MS: u64 = 40;

#[derive(Debug, Clone)]
pub(crate) struct WavPitchHarmonicEvidenceProbeAudit {
    pub(crate) measured: bool,
    pub(crate) reason: &'static str,
    pub(crate) sample_rate: u32,
    pub(crate) quarter_frames: u64,
    pub(crate) audio_attack_candidates: usize,
    pub(crate) probe_attacks: usize,
    pub(crate) dual_probe_attacks: usize,
    pub(crate) positive_rise_pitch_rows: usize,
    pub(crate) top8_harmonic_parent_ratio_median_permille: u64,
    pub(crate) matched_probe_attacks: usize,
    pub(crate) oracle_pitch_instances_evaluated: usize,
    pub(crate) rise_recall_top1: usize,
    pub(crate) rise_recall_top3: usize,
    pub(crate) rise_recall_top5: usize,
    pub(crate) rise_recall_top8: usize,
    pub(crate) presence_recall_top8: usize,
    pub(crate) top8_false_candidates: usize,
    pub(crate) top8_false_harmonic_explainable: usize,
    pub(crate) same_pitch_rearticulation_pairs: usize,
    pub(crate) same_pitch_rearticulation_both_top8: usize,
    pub(crate) external_sensor_frozen_observed: bool,
    pub(crate) complementarity_oracle_pitch_instances: usize,
    pub(crate) external_sensor_recovered_80: usize,
    pub(crate) union_recovered_80: usize,
    pub(crate) external_only_recovered_80: usize,
    pub(crate) physical_only_recovered_80: usize,
    pub(crate) both_recovered_80: usize,
    pub(crate) neither_recovered_80: usize,
    pub(crate) high_oracle_pitch_instances: usize,
    pub(crate) high_external_recovered_80: usize,
    pub(crate) high_physical_recovered_80: usize,
    pub(crate) high_union_recovered_80: usize,
}

#[derive(Debug, Clone, Copy)]
struct PitchEvidence {
    midi: i32,
    rise_power: f64,
    presence_power: f64,
    harmonic_parent_ratio_permille: u64,
}

#[derive(Debug, Clone)]
struct AttackPitchProbe {
    frame: u64,
    dual_family: bool,
    rise_ranked: Vec<PitchEvidence>,
    presence_ranked: Vec<PitchEvidence>,
}

#[derive(Debug, Clone)]
struct ProjectedOracleOnset {
    onset_ticks: i64,
    frame: u64,
    pitches: Vec<i32>,
    source_ordinals: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct FrozenExternalSensorEvent {
    midi: i32,
    onset_frame: u64,
}

pub(crate) fn build_for_score_title(
    score_title: Option<&str>,
) -> Option<WavPitchHarmonicEvidenceProbeAudit> {
    let signal = match load_decoded_wav_signal_for_score_title(score_title) {
        Ok(signal) => signal,
        Err(reason) => return Some(not_measured(reason)),
    };
    let timing = match build_audio_only_pitch_timing_snapshot(&signal) {
        Ok(timing) => timing,
        Err(reason) => return Some(not_measured(reason)),
    };

    // Hard anti-leakage boundary: probe attacks and all 88-key pitch evidence
    // rows are frozen from WAV only before the frozen legacy source oracle is loaded.
    let selected_attacks = select_probe_attacks(&timing.attacks, MAX_PROBE_ATTACKS);
    let probes: Vec<AttackPitchProbe> = selected_attacks
        .iter()
        .filter_map(|attack| build_pitch_probe(&signal, *attack))
        .collect();

    // Cross-view ordering remains fail-closed: the external sensor projection was already
    // frozen by the preceding report section without source-oracle access. Read only that immutable
    // projection here, still before loading any legacy oracle evidence.
    let external_sensor_events = load_frozen_external_sensor_events(score_title);

    let legacy_events = match load_canonical_legacy_pitch_events_for_score_title(score_title) {
        Ok(events) => events,
        Err(reason) => {
            let mut audit = audio_only_summary(&timing, &probes);
            audit.reason = reason;
            return Some(audit);
        }
    };

    let projected_oracle = project_oracle_onsets(&timing, &legacy_events);
    let mut audit = evaluate_probes(
        &timing,
        &probes,
        &projected_oracle,
        external_sensor_events.as_deref(),
    );
    audit.measured = audit.probe_attacks > 0 && audit.oracle_pitch_instances_evaluated > 0;
    audit.reason = if audit.measured {
        "wav_pitch_harmonic_evidence_probe_measured"
    } else {
        "wav_pitch_harmonic_evidence_probe_insufficient_overlap"
    };
    Some(audit)
}

fn not_measured(reason: &'static str) -> WavPitchHarmonicEvidenceProbeAudit {
    WavPitchHarmonicEvidenceProbeAudit {
        measured: false,
        reason,
        sample_rate: 0,
        quarter_frames: 0,
        audio_attack_candidates: 0,
        probe_attacks: 0,
        dual_probe_attacks: 0,
        positive_rise_pitch_rows: 0,
        top8_harmonic_parent_ratio_median_permille: 0,
        matched_probe_attacks: 0,
        oracle_pitch_instances_evaluated: 0,
        rise_recall_top1: 0,
        rise_recall_top3: 0,
        rise_recall_top5: 0,
        rise_recall_top8: 0,
        presence_recall_top8: 0,
        top8_false_candidates: 0,
        top8_false_harmonic_explainable: 0,
        same_pitch_rearticulation_pairs: 0,
        same_pitch_rearticulation_both_top8: 0,
        external_sensor_frozen_observed: false,
        complementarity_oracle_pitch_instances: 0,
        external_sensor_recovered_80: 0,
        union_recovered_80: 0,
        external_only_recovered_80: 0,
        physical_only_recovered_80: 0,
        both_recovered_80: 0,
        neither_recovered_80: 0,
        high_oracle_pitch_instances: 0,
        high_external_recovered_80: 0,
        high_physical_recovered_80: 0,
        high_union_recovered_80: 0,
    }
}

fn audio_only_summary(
    timing: &AudioOnlyPitchTimingSnapshot,
    probes: &[AttackPitchProbe],
) -> WavPitchHarmonicEvidenceProbeAudit {
    let mut parent_ratios = Vec::new();
    let mut positive_rise_pitch_rows = 0usize;
    for probe in probes {
        for row in &probe.rise_ranked {
            if row.rise_power > 0.0 {
                positive_rise_pitch_rows = positive_rise_pitch_rows.saturating_add(1);
            }
        }
        parent_ratios.extend(
            probe
                .rise_ranked
                .iter()
                .take(8)
                .map(|row| row.harmonic_parent_ratio_permille),
        );
    }
    parent_ratios.sort_unstable();
    WavPitchHarmonicEvidenceProbeAudit {
        measured: false,
        reason: "legacy_pitch_oracle_unavailable",
        sample_rate: timing.sample_rate,
        quarter_frames: timing.quarter_frames,
        audio_attack_candidates: timing.attacks.len(),
        probe_attacks: probes.len(),
        dual_probe_attacks: probes.iter().filter(|probe| probe.dual_family).count(),
        positive_rise_pitch_rows,
        top8_harmonic_parent_ratio_median_permille: percentile(&parent_ratios, 50),
        matched_probe_attacks: 0,
        oracle_pitch_instances_evaluated: 0,
        rise_recall_top1: 0,
        rise_recall_top3: 0,
        rise_recall_top5: 0,
        rise_recall_top8: 0,
        presence_recall_top8: 0,
        top8_false_candidates: 0,
        top8_false_harmonic_explainable: 0,
        same_pitch_rearticulation_pairs: 0,
        same_pitch_rearticulation_both_top8: 0,
        external_sensor_frozen_observed: false,
        complementarity_oracle_pitch_instances: 0,
        external_sensor_recovered_80: 0,
        union_recovered_80: 0,
        external_only_recovered_80: 0,
        physical_only_recovered_80: 0,
        both_recovered_80: 0,
        neither_recovered_80: 0,
        high_oracle_pitch_instances: 0,
        high_external_recovered_80: 0,
        high_physical_recovered_80: 0,
        high_union_recovered_80: 0,
    }
}

fn select_probe_attacks(
    attacks: &[AudioOnlyAttackProbe],
    maximum: usize,
) -> Vec<AudioOnlyAttackProbe> {
    if attacks.len() <= maximum {
        return attacks.to_vec();
    }
    if maximum == 0 {
        return Vec::new();
    }

    let mut selected = Vec::with_capacity(maximum);
    for bucket in 0..maximum {
        let start = bucket.saturating_mul(attacks.len()) / maximum;
        let mut end = bucket.saturating_add(1).saturating_mul(attacks.len()) / maximum;
        end = end.max(start.saturating_add(1)).min(attacks.len());
        let slice = &attacks[start..end];
        let center_frame = slice
            .first()
            .zip(slice.last())
            .map_or(0, |(first, last)| first.frame.saturating_add(last.frame) / 2);
        let chosen = slice
            .iter()
            .copied()
            .min_by_key(|attack| {
                (
                    !attack.dual_family,
                    attack.frame.abs_diff(center_frame),
                    attack.frame,
                )
            })
            .unwrap_or(slice[0]);
        selected.push(chosen);
    }
    selected.sort_by_key(|attack| attack.frame);
    selected.dedup_by_key(|attack| attack.frame);
    selected
}

fn build_pitch_probe(
    signal: &DecodedWavSignalAudit,
    attack: AudioOnlyAttackProbe,
) -> Option<AttackPitchProbe> {
    if signal.sample_rate == 0 || signal.channels == 0 {
        return None;
    }

    let mut rows = Vec::with_capacity(
        usize::try_from(PIANO_MIDI_MAX - PIANO_MIDI_MIN + 1).unwrap_or(88),
    );
    for midi in PIANO_MIDI_MIN..=PIANO_MIDI_MAX {
        let frequency = midi_frequency_hz(midi);
        let window_frames = pitch_window_frames(signal.sample_rate, frequency);
        let gap_frames = u64::from(signal.sample_rate) / 200; // 5 ms
        let pre_end = attack.frame.saturating_sub(gap_frames);
        if pre_end < window_frames {
            continue;
        }
        let pre_start = pre_end.saturating_sub(window_frames);
        let post_start = attack.frame;
        let post_end = post_start.saturating_add(window_frames);
        if post_end > signal.original_frame_count {
            continue;
        }

        let pre_power = goertzel_power(signal, pre_start, window_frames, frequency);
        let post_power = goertzel_power(signal, post_start, window_frames, frequency);
        let rise_power = (post_power - pre_power).max(0.0);
        rows.push(PitchEvidence {
            midi,
            rise_power,
            presence_power: post_power,
            harmonic_parent_ratio_permille: 0,
        });
    }
    if rows.is_empty() {
        return None;
    }

    let rise_by_midi: BTreeMap<i32, f64> =
        rows.iter().map(|row| (row.midi, row.rise_power)).collect();
    for row in &mut rows {
        let parent_rise = [12, 19, 24]
            .into_iter()
            .filter_map(|offset| rise_by_midi.get(&row.midi.saturating_sub(offset)).copied())
            .fold(0.0f64, f64::max);
        let denominator = row.rise_power + parent_rise;
        row.harmonic_parent_ratio_permille = if denominator > 0.0 {
            ((parent_rise / denominator) * 1_000.0)
                .round()
                .clamp(0.0, 1_000.0) as u64
        } else {
            0
        };
    }

    let mut rise_ranked = rows.clone();
    rise_ranked.sort_by(|left, right| {
        right
            .rise_power
            .total_cmp(&left.rise_power)
            .then_with(|| right.presence_power.total_cmp(&left.presence_power))
            .then_with(|| left.midi.cmp(&right.midi))
    });
    rise_ranked.truncate(TOP_EVIDENCE_ROWS.min(rise_ranked.len()));

    let mut presence_ranked = rows;
    presence_ranked.sort_by(|left, right| {
        right
            .presence_power
            .total_cmp(&left.presence_power)
            .then_with(|| right.rise_power.total_cmp(&left.rise_power))
            .then_with(|| left.midi.cmp(&right.midi))
    });
    presence_ranked.truncate(TOP_EVIDENCE_ROWS.min(presence_ranked.len()));

    Some(AttackPitchProbe {
        frame: attack.frame,
        dual_family: attack.dual_family,
        rise_ranked,
        presence_ranked,
    })
}


fn load_frozen_external_sensor_events(
    score_title: Option<&str>,
) -> Option<Vec<FrozenExternalSensorEvent>> {
    let title = score_title?.trim();
    if title.is_empty() {
        return None;
    }
    let current = std::env::current_dir().ok()?;
    let piece_dir = current
        .join("target/wav_pitch_external_sensor_audit/pieces")
        .join(title);
    if !piece_dir.join("SENSOR_OUTPUT_FROZEN.sha256").is_file() {
        return None;
    }
    let text = std::fs::read_to_string(piece_dir.join("sensor_events.frozen.txt")).ok()?;
    Some(parse_frozen_external_sensor_events(&text))
}

fn parse_frozen_external_sensor_events(text: &str) -> Vec<FrozenExternalSensorEvent> {
    let mut events = Vec::new();
    for line in text.lines() {
        if line.is_empty() || line.starts_with("schema=") {
            continue;
        }
        let mut fields = line.split('|');
        let _ordinal = fields.next();
        let Some(midi) = fields.next().and_then(|value| value.parse::<i32>().ok()) else {
            continue;
        };
        let Some(onset_frame) = fields.next().and_then(|value| value.parse::<u64>().ok()) else {
            continue;
        };
        events.push(FrozenExternalSensorEvent { midi, onset_frame });
    }
    events.sort_by_key(|event| (event.onset_frame, event.midi));
    events
}

fn midi_frequency_hz(midi: i32) -> f64 {
    440.0 * 2.0f64.powf(f64::from(midi - 69) / 12.0)
}

fn pitch_window_frames(sample_rate: u32, frequency: f64) -> u64 {
    let minimum = u64::from(sample_rate).saturating_mul(40) / 1_000;
    let maximum = u64::from(sample_rate).saturating_mul(180) / 1_000;
    let cycle_based = if frequency > 0.0 {
        ((f64::from(sample_rate) * 4.0) / frequency).round().max(1.0) as u64
    } else {
        minimum
    };
    cycle_based.max(minimum).min(maximum).max(256)
}

fn goertzel_power(
    signal: &DecodedWavSignalAudit,
    start_frame: u64,
    frame_count: u64,
    frequency: f64,
) -> f64 {
    let channels = usize::from(signal.channels).max(1);
    let Ok(start) = usize::try_from(start_frame) else {
        return 0.0;
    };
    let Ok(count) = usize::try_from(frame_count) else {
        return 0.0;
    };
    if count == 0 {
        return 0.0;
    }
    let end = start.saturating_add(count);
    let total_frames = signal.interleaved_samples.len() / channels;
    if end > total_frames || signal.sample_rate == 0 {
        return 0.0;
    }

    let omega = 2.0 * std::f64::consts::PI * frequency / f64::from(signal.sample_rate);
    let coefficient = 2.0 * omega.cos();
    let mut s_prev = 0.0f64;
    let mut s_prev2 = 0.0f64;

    for frame in start..end {
        let base = frame.saturating_mul(channels);
        let mut mono = 0.0f64;
        for channel in 0..channels {
            mono += f64::from(signal.interleaved_samples[base + channel]);
        }
        mono /= channels as f64;
        let s = mono + coefficient * s_prev - s_prev2;
        s_prev2 = s_prev;
        s_prev = s;
    }

    let raw = s_prev2 * s_prev2 + s_prev * s_prev - coefficient * s_prev * s_prev2;
    let normalization = (count as f64).powi(2).max(1.0);
    (raw / normalization).max(0.0)
}

fn project_oracle_onsets(
    timing: &AudioOnlyPitchTimingSnapshot,
    events: &[CanonicalLegacyPitchEventAudit],
) -> Vec<ProjectedOracleOnset> {
    let mut grouped = BTreeMap::<i64, (Vec<i32>, Vec<usize>)>::new();
    for event in events {
        let entry = grouped.entry(event.onset_ticks).or_default();
        entry.0.push(event.midi);
        entry.1.push(event.source_ordinal);
    }

    let mut out = Vec::with_capacity(grouped.len());
    for (onset_ticks, (mut pitches, mut source_ordinals)) in grouped {
        let projected = timing.project_canonical_tick(onset_ticks);
        if projected < 0 {
            continue;
        }
        let Ok(frame) = u64::try_from(projected) else {
            continue;
        };
        if frame >= timing.original_frame_count {
            continue;
        }
        pitches.sort_unstable();
        source_ordinals.sort_unstable();
        out.push(ProjectedOracleOnset {
            onset_ticks,
            frame,
            pitches,
            source_ordinals,
        });
    }
    out.sort_by_key(|row| (row.frame, row.onset_ticks));
    out
}

fn evaluate_probes(
    timing: &AudioOnlyPitchTimingSnapshot,
    probes: &[AttackPitchProbe],
    oracle: &[ProjectedOracleOnset],
    external_sensor_events: Option<&[FrozenExternalSensorEvent]>,
) -> WavPitchHarmonicEvidenceProbeAudit {
    let mut audit = audio_only_summary(timing, probes);
    audit.external_sensor_frozen_observed = external_sensor_events.is_some();
    let tolerance = milliseconds_to_frames(timing.sample_rate, EVALUATION_MATCH_MS);
    let external_tolerance = milliseconds_to_frames(timing.sample_rate, 80);
    let mut matched_rows: Vec<(i64, BTreeSet<i32>, BTreeSet<i32>)> = Vec::new();

    for probe in probes {
        let Some(row) = oracle
            .iter()
            .min_by_key(|row| row.frame.abs_diff(probe.frame))
        else {
            continue;
        };
        if row.frame.abs_diff(probe.frame) > tolerance {
            continue;
        }
        audit.matched_probe_attacks = audit.matched_probe_attacks.saturating_add(1);

        // Preserve source multiplicity in the denominator even though one
        // spectral pitch row cannot represent two physically identical
        // same-pitch/same-onset source records as separate sensor values.
        audit.oracle_pitch_instances_evaluated = audit
            .oracle_pitch_instances_evaluated
            .saturating_add(row.source_ordinals.len());

        let oracle_unique: BTreeSet<i32> = row.pitches.iter().copied().collect();
        let rise_top8: BTreeSet<i32> = probe
            .rise_ranked
            .iter()
            .take(8)
            .map(|evidence| evidence.midi)
            .collect();
        let presence_top8: BTreeSet<i32> = probe
            .presence_ranked
            .iter()
            .take(8)
            .map(|evidence| evidence.midi)
            .collect();

        for &midi in &row.pitches {
            let rise_rank = probe
                .rise_ranked
                .iter()
                .position(|evidence| evidence.midi == midi)
                .map(|index| index.saturating_add(1))
                .unwrap_or(usize::MAX);
            if rise_rank <= 1 {
                audit.rise_recall_top1 = audit.rise_recall_top1.saturating_add(1);
            }
            if rise_rank <= 3 {
                audit.rise_recall_top3 = audit.rise_recall_top3.saturating_add(1);
            }
            if rise_rank <= 5 {
                audit.rise_recall_top5 = audit.rise_recall_top5.saturating_add(1);
            }
            if rise_rank <= 8 {
                audit.rise_recall_top8 = audit.rise_recall_top8.saturating_add(1);
            }
            if presence_top8.contains(&midi) {
                audit.presence_recall_top8 = audit.presence_recall_top8.saturating_add(1);
            }

            if let Some(external) = external_sensor_events {
                let physical_recovered = rise_rank <= 8;
                let external_recovered = external.iter().any(|event| {
                    event.midi == midi && event.onset_frame.abs_diff(row.frame) <= external_tolerance
                });
                audit.complementarity_oracle_pitch_instances =
                    audit.complementarity_oracle_pitch_instances.saturating_add(1);
                if external_recovered {
                    audit.external_sensor_recovered_80 =
                        audit.external_sensor_recovered_80.saturating_add(1);
                }
                if external_recovered || physical_recovered {
                    audit.union_recovered_80 = audit.union_recovered_80.saturating_add(1);
                }
                match (external_recovered, physical_recovered) {
                    (true, true) => {
                        audit.both_recovered_80 = audit.both_recovered_80.saturating_add(1);
                    }
                    (true, false) => {
                        audit.external_only_recovered_80 =
                            audit.external_only_recovered_80.saturating_add(1);
                    }
                    (false, true) => {
                        audit.physical_only_recovered_80 =
                            audit.physical_only_recovered_80.saturating_add(1);
                    }
                    (false, false) => {
                        audit.neither_recovered_80 =
                            audit.neither_recovered_80.saturating_add(1);
                    }
                }

                if midi >= 84 {
                    audit.high_oracle_pitch_instances =
                        audit.high_oracle_pitch_instances.saturating_add(1);
                    if external_recovered {
                        audit.high_external_recovered_80 =
                            audit.high_external_recovered_80.saturating_add(1);
                    }
                    if physical_recovered {
                        audit.high_physical_recovered_80 =
                            audit.high_physical_recovered_80.saturating_add(1);
                    }
                    if external_recovered || physical_recovered {
                        audit.high_union_recovered_80 =
                            audit.high_union_recovered_80.saturating_add(1);
                    }
                }
            }
        }

        for candidate in rise_top8.difference(&oracle_unique) {
            audit.top8_false_candidates = audit.top8_false_candidates.saturating_add(1);
            if oracle_unique
                .iter()
                .any(|source| harmonic_semitone_relation(*source, *candidate))
            {
                audit.top8_false_harmonic_explainable =
                    audit.top8_false_harmonic_explainable.saturating_add(1);
            }
        }
        matched_rows.push((row.onset_ticks, oracle_unique, rise_top8));
    }

    matched_rows.sort_by_key(|row| row.0);
    let mut previous_by_pitch = BTreeMap::<i32, (i64, bool)>::new();
    for (onset, oracle_pitches, top8) in matched_rows {
        for midi in oracle_pitches {
            if let Some((previous_onset, previous_recovered)) = previous_by_pitch.get(&midi).copied()
                && onset > previous_onset
                && onset.saturating_sub(previous_onset) <= 96
            {
                audit.same_pitch_rearticulation_pairs =
                    audit.same_pitch_rearticulation_pairs.saturating_add(1);
                if previous_recovered && top8.contains(&midi) {
                    audit.same_pitch_rearticulation_both_top8 =
                        audit.same_pitch_rearticulation_both_top8.saturating_add(1);
                }
            }
            previous_by_pitch.insert(midi, (onset, top8.contains(&midi)));
        }
    }

    audit
}

fn harmonic_semitone_relation(source_midi: i32, candidate_midi: i32) -> bool {
    if candidate_midi <= source_midi {
        return false;
    }
    matches!(
        candidate_midi - source_midi,
        12 | 19 | 24 | 28 | 31 | 34 | 36
    )
}

fn milliseconds_to_frames(sample_rate: u32, milliseconds: u64) -> u64 {
    u64::from(sample_rate)
        .saturating_mul(milliseconds)
        .saturating_add(500)
        / 1_000
}

fn percentile(sorted: &[u64], value: usize) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let index = sorted
        .len()
        .saturating_sub(1)
        .saturating_mul(value.min(100))
        / 100;
    sorted[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_selection_is_bounded_and_time_ordered() {
        let attacks: Vec<AudioOnlyAttackProbe> = (0..1_000)
            .map(|index| AudioOnlyAttackProbe {
                frame: index * 100,
                dual_family: index % 3 == 0,
            })
            .collect();
        let selected = select_probe_attacks(&attacks, 192);
        assert!(selected.len() <= 192);
        assert!(selected.windows(2).all(|pair| pair[0].frame < pair[1].frame));
    }

    #[test]
    fn harmonic_relation_covers_low_partial_family_only_upward() {
        assert!(harmonic_semitone_relation(60, 72));
        assert!(harmonic_semitone_relation(60, 79));
        assert!(harmonic_semitone_relation(60, 84));
        assert!(!harmonic_semitone_relation(72, 60));
        assert!(!harmonic_semitone_relation(60, 61));
    }

    #[test]
    fn midi_frequency_has_a4_at_440_hz() {
        assert!((midi_frequency_hz(69) - 440.0).abs() < 0.000_001);
    }

    #[test]
    fn frozen_external_projection_parser_keeps_pitch_and_original_frame() {
        let events = parse_frozen_external_sensor_events(
            "schema=basic_pitch_rust_event_projection_v1\n0|60|44100|45000|00000000|1|2\n1|84|88200|89000|00000000|3|4\n",
        );
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].midi, 60);
        assert_eq!(events[0].onset_frame, 44_100);
        assert_eq!(events[1].midi, 84);
        assert_eq!(events[1].onset_frame, 88_200);
    }
}
