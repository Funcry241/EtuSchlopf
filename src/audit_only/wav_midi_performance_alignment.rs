//! Diagnostic correspondence between raw MIDI Note-ons and original WAV frames.

use std::path::Path;

use crate::midi::DecodedMidiSource;

use super::{wav_attack_pulse_metric::original_audio_attack_frames, wav_signal_decode::load_decoded_wav_signal};

const DEFAULT_MICROSECONDS_PER_QUARTER: u32 = 500_000;
const OFFSET_SEARCH_SECONDS: i64 = 10;
const OFFSET_STEP_MILLISECONDS: i64 = 10;
const ATTACK_TOLERANCE_MILLISECONDS: i64 = 30;
const MINIMUM_ATTACKS: usize = 12;

pub(crate) fn report(midi: &DecodedMidiSource, wav_path: &Path) -> String {
    let evidence = measure(midi, wav_path);
    let detail = match evidence {
        Ok((alignment, attack_count, sample_rate)) => format!(
            "Zeitzuordnung gestützt · WAV-Frame ≈ MIDI-Frame {:+} bei {} Hz · {}/{} Anschläge · beste getrennte Alternative {}/{}",
            alignment.offset, sample_rate, alignment.matched, attack_count, alignment.alternative, attack_count,
        ),
        Err(reason) => format!("Zeitzuordnung offen · {reason}"),
    };
    format!("MIDI/WAV-Diagnose · {detail}")
}

fn measure(midi: &DecodedMidiSource, wav_path: &Path) -> Result<(Alignment, usize, u32), &'static str> {
    if midi.format > 1 {
        return Err("MIDI enthält unabhängige Zeitachsen");
    }
    let signal = load_decoded_wav_signal(wav_path)?;
    let mut ticks: Vec<_> = midi.source_event_causal_ledger.entries.iter().map(|entry| entry.note_on.tick).collect();
    ticks.sort_unstable();
    ticks.dedup();
    let midi_frames = midi_attack_frames(&ticks, &midi.tempo_changes, midi.source_ppq, signal.sample_rate)?;
    let audio_frames: Vec<_> = original_audio_attack_frames(&signal)
        .into_iter()
        .map(|frame| i64::try_from(frame).map_err(|_| "WAV-Frame außerhalb des Zahlenbereichs"))
        .collect::<Result<_, _>>()?;
    let alignment = select_alignment(&midi_frames, &audio_frames, signal.sample_rate)?;
    Ok((alignment, midi_frames.len(), signal.sample_rate))
}

fn midi_attack_frames(
    ticks: &[u64],
    tempo_changes: &[(u64, u32)],
    ppq: i64,
    sample_rate: u32,
) -> Result<Vec<i64>, &'static str> {
    if ppq <= 0 || sample_rate == 0 || tempo_changes.iter().any(|&(_, tempo)| tempo == 0) {
        return Err("Ungültige MIDI-Tempo- oder WAV-Zeitbasis");
    }
    if tempo_changes.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err("MIDI-Tempo am selben Tick ist mehrdeutig");
    }
    let denominator = i128::from(ppq) * 1_000_000;
    let mut elapsed = 0_i128;
    let mut previous_tick = 0;
    let mut tempo = DEFAULT_MICROSECONDS_PER_QUARTER;
    let mut changes = tempo_changes.iter().copied().peekable();
    ticks.iter().map(|&tick| {
        while let Some((change_tick, next_tempo)) = changes.next_if(|&(change_tick, _)| change_tick <= tick) {
            elapsed += i128::from(change_tick - previous_tick) * i128::from(tempo);
            previous_tick = change_tick;
            tempo = next_tempo;
        }
        let attack_time = elapsed + i128::from(tick - previous_tick) * i128::from(tempo);
        let frame = (attack_time * i128::from(sample_rate) + denominator / 2) / denominator;
        i64::try_from(frame).map_err(|_| "MIDI-Spielzeit außerhalb des WAV-Frame-Zahlenbereichs")
    }).collect()
}

#[derive(Debug)]
struct Alignment {
    offset: i64,
    matched: usize,
    alternative: usize,
}

fn select_alignment(midi: &[i64], audio: &[i64], sample_rate: u32) -> Result<Alignment, &'static str> {
    if midi.len().min(audio.len()) < MINIMUM_ATTACKS {
        return Err("Zu wenige Anschläge");
    }
    let rate = i64::from(sample_rate);
    let tolerance = (rate * ATTACK_TOLERANCE_MILLISECONDS / 1_000).max(1);
    let step = (rate * OFFSET_STEP_MILLISECONDS / 1_000).max(1);
    let steps = rate * OFFSET_SEARCH_SECONDS / step;
    let counts: Vec<_> = (-steps..=steps).map(|index| {
        let offset = index * step;
        (offset, matching_attacks(midi, audio, offset, tolerance))
    }).collect();
    let &(offset, matched) = counts.iter()
        .max_by_key(|&&(offset, matched)| (matched, std::cmp::Reverse(offset.abs())))
        .ok_or("Kein Zeitversatz geprüft")?;
    if matched < midi.len().saturating_mul(3).div_ceil(4) {
        return Err("Zu geringe gemeinsame Anschlagsdeckung");
    }
    let alternative = counts.iter()
        .filter(|&&(other_offset, _)| other_offset.abs_diff(offset) > (tolerance * 2).unsigned_abs())
        .map(|&(_, count)| count)
        .max().unwrap_or(0);
    if matched - alternative < midi.len().div_ceil(10) {
        return Err("Mehrere getrennte Zeitversätze passen ähnlich gut");
    }
    // A dense passage must not hide a mismatch elsewhere in the performance.
    let start = midi[0];
    let third = (midi[midi.len() - 1] - start) / 3;
    let middle = midi.partition_point(|&frame| frame < start + third);
    let end = midi.partition_point(|&frame| frame < start + third * 2);
    if [&midi[..middle], &midi[middle..end], &midi[end..]].into_iter().any(|part| {
        part.len() < 3 || matching_attacks(part, audio, offset, tolerance) < part.len().saturating_mul(2).div_ceil(3)
    }) {
        return Err("Zeitversatz wird nicht über den Spielverlauf gestützt");
    }
    Ok(Alignment { offset, matched, alternative })
}

fn matching_attacks(midi: &[i64], audio: &[i64], offset: i64, tolerance: i64) -> usize {
    let mut next_audio = 0;
    let mut matched = 0;
    for &frame in midi {
        let target = frame.saturating_add(offset);
        let lower = target.saturating_sub(tolerance);
        let upper = target.saturating_add(tolerance);
        while audio.get(next_audio).is_some_and(|&attack| attack < lower) {
            next_audio += 1;
        }
        if audio.get(next_audio).is_some_and(|&attack| attack <= upper) {
            matched += 1;
            next_audio += 1;
        }
    }
    matched
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATTACKS: [i64; 15] = [
        2200, 2445, 2890, 3315, 3940, 4300, 4780, 5295, 5510, 5920, 6430, 6750, 7080, 7465, 7750,
    ];

    #[test]
    fn tempo_map_integrates_original_ticks_and_rejects_conflicts() {
        assert_eq!(
            midi_attack_frames(&[0, 240, 480, 720, 960, 1440], &[(480, 600_000), (960, 400_000)], 480, 44_100),
            Ok(vec![0, 11025, 22050, 35280, 48510, 66150]),
        );
        assert!(midi_attack_frames(&[0], &[(0, 400_000), (0, 500_000)], 480, 1_000).is_err());
    }

    #[test]
    fn irregular_attacks_support_a_shared_original_frame_offset() {
        for offset in [-1_350, 1_350] {
            let audio = ATTACKS.map(|frame| frame + offset);
            let alignment = select_alignment(&ATTACKS, &audio, 1_000).unwrap();
            assert!(alignment.offset.abs_diff(offset) <= 30);
            assert_eq!(alignment.matched, ATTACKS.len());
        }
    }

    #[test]
    fn independent_copies_remain_ambiguous() {
        let mut audio = Vec::from(ATTACKS);
        audio.extend(ATTACKS.map(|frame| frame + 8_000));
        assert!(select_alignment(&ATTACKS, &audio, 1_000).is_err());
    }

    #[test]
    fn agreement_early_in_the_playback_cannot_cover_late_drift() {
        let mut audio = ATTACKS;
        for (index, frame) in audio.iter_mut().skip(11).enumerate() {
            *frame += 100 * (i64::try_from(index).unwrap() + 1);
        }
        assert!(select_alignment(&ATTACKS, &audio, 1_000).is_err());
        assert_eq!(matching_attacks(&[98, 102], &[100], 0, 5), 1);
    }
}
