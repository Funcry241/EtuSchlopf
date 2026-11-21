//! Read-only evidence for deriving written time from a MIDI/WAV performance.
use super::{NotationEvent, NotationEventKind, WrittenHoldProposal};
use crate::midi::source_time::WRITTEN_NOTE_VALUES;
use crate::midi::{DecodedMidiSource, MidiKeySignature};
use anyhow::{Result, ensure};
use std::collections::{BTreeMap, BTreeSet};

const UNIT: i64 = 24;
const VALUES: [i64; 9] = [12, 24, 48, 72, 96, 144, 192, 288, 384];

#[derive(Clone, Copy)]
struct Attack {
    id: usize,
    time: f64,
    hold: f64,
    pitch: u8,
    velocity: u8,
    track: usize,
    channel: u8,
}

pub(super) struct FreePerformance {
    pub(super) beats: u8,
    pub(super) beat_type: u16,
    pub(super) pickup: i64,
    pub(super) key: Option<MidiKeySignature>,
    pub(super) events: Vec<NotationEvent>,
    pub(super) open_durations: BTreeSet<usize>,
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    (values[(values.len() - 1) / 2] + values[values.len() / 2]) / 2.0
}

fn source_attacks(source: &DecodedMidiSource) -> Result<Vec<Attack>> {
    ensure!(
        source.format <= 1 && source.source_ppq > 0,
        "MIDI-Zeitachsen sind unabhaengig"
    );
    ensure!(
        source
            .tempo_changes
            .windows(2)
            .all(|pair| pair[0].0 < pair[1].0)
            && source.tempo_changes.iter().all(|&(_, tempo)| tempo > 0),
        "MIDI-Tempo ist mehrdeutig"
    );
    let seconds = |tick: u64| {
        let mut elapsed = 0.0;
        let mut previous = 0;
        let mut tempo = 500_000;
        for &(change, next_tempo) in source
            .tempo_changes
            .iter()
            .take_while(|entry| entry.0 <= tick)
        {
            elapsed += (change - previous) as f64 * f64::from(tempo);
            previous = change;
            tempo = next_tempo;
        }
        (elapsed + (tick - previous) as f64 * f64::from(tempo))
            / (source.source_ppq as f64 * 1_000_000.0)
    };
    let mut attacks: Vec<_> = source
        .source_event_causal_ledger
        .entries
        .iter()
        .map(|entry| {
            let start = seconds(entry.note_on.tick);
            Attack {
                id: entry.source_ordinal,
                time: start,
                hold: seconds(entry.note_off.tick) - start,
                pitch: entry.note_on.key,
                velocity: source.attack_velocities[entry.source_ordinal],
                track: entry.note_on.track_index,
                channel: entry.note_on.channel,
            }
        })
        .collect();
    attacks.sort_by_key(|attack| attack.id);
    Ok(attacks)
}

struct AttackGroup {
    attacks: Vec<Attack>,
    time: f64,
    pitches: u128,
    bass: f64,
    velocity: f64,
}

fn attack_groups(attacks: &[Attack]) -> (Vec<AttackGroup>, Vec<Attack>) {
    let mut clusters: Vec<Vec<Attack>> = vec![];
    for &attack in attacks {
        if let Some(last) = clusters.last_mut()
            && attack.time - last[0].time <= 0.082
        {
            last.push(attack);
        } else {
            clusters.push(vec![attack]);
        }
    }
    let (played, quiet): (Vec<_>, Vec<_>) = clusters
        .into_iter()
        .partition(|cluster| cluster.iter().any(|attack| attack.velocity > 8));
    let groups = played
        .into_iter()
        .map(|attacks| AttackGroup {
            time: median(attacks.iter().map(|attack| attack.time).collect()),
            pitches: attacks
                .iter()
                .fold(0, |bits, attack| bits | (1_u128 << attack.pitch)),
            bass: f64::from(
                attacks
                    .iter()
                    .map(|attack| attack.pitch)
                    .min()
                    .expect("nonempty attack group"),
            ),
            velocity: attacks
                .iter()
                .map(|attack| f64::from(attack.velocity))
                .sum::<f64>()
                / attacks.len() as f64,
            attacks,
        })
        .collect();
    (groups, quiet.into_iter().flatten().collect())
}

// Key releases near each other support a spread onset. An earlier
// key released at the next attack is instead evidence for separate motion.
fn release_agreement(before: &AttackGroup, after: &AttackGroup, unit: f64) -> f64 {
    let width = unit / 4.0;
    let mut evidence = 0.0_f64;
    for earlier in &before.attacks {
        let release = earlier.time + earlier.hold;
        for later in &after.attacks {
            if (earlier.track, earlier.channel) != (later.track, later.channel)
                || release - later.time < unit / 2.0
                || later.hold < unit / 2.0
            {
                continue;
            }
            let other_release = later.time + later.hold;
            let difference = (release - other_release).abs() / width;
            evidence = evidence.max((-0.5 * difference.powi(2)).exp());
        }
    }
    evidence
}

// A repeated chord change offers rhythmic evidence only when the other
// occurrences agree and the proposed interval fits the local pulse.
fn repeated_transition_jumps(groups: &[AttackGroup], onsets: &[i64]) -> Vec<Option<usize>> {
    let edges = groups.windows(2).map(|pair| (pair[0].pitches, pair[1].pitches))
        .collect::<Vec<_>>();
    let jumps = onsets.windows(2).map(|pair| (pair[1] - pair[0]) as usize).collect::<Vec<_>>();
    let mut occurrences = BTreeMap::<_, Vec<_>>::new();
    for (i, &(before, after)) in edges.iter().enumerate() {
        if before != after && before.count_ones() >= 2 && after.count_ones() >= 2 {
            occurrences.entry((before, after)).or_default().push(i);
        }
    }

    let mut evidence = vec![None; edges.len()];
    for indices in occurrences.values().filter(|indices| indices.len() >= 3) {
        for &edge in indices {
            let mut peers = indices.iter().copied().filter(|&i| i.abs_diff(edge) > 1);
            let Some(first) = peers.next() else { continue };
            let jump = jumps[first];
            if jump == 0 || !peers.next().is_some_and(|i| jumps[i] == jump)
                || peers.any(|i| jumps[i] != jump)
            {
                continue;
            }
            // The next gesture may change pace. Judge entry into the
            // repeated transition using the tempo leading into it.
            let local: Vec<_> = (edge.saturating_sub(3)..edge)
                .filter(|&i| jumps[i] > 0)
                .map(|i| (groups[i + 1].time - groups[i].time) / jumps[i] as f64)
                .collect();
            if local.len() < 2 {
                continue;
            }
            let proposed = (groups[edge + 1].time - groups[edge].time) / jump as f64;
            if (proposed / median(local)).ln().abs() <= 1.4_f64.ln() {
                evidence[edge] = Some(jump);
            }
        }
    }
    evidence
}

type Repeat = Vec<(usize, usize)>;

fn repeated_phrases(groups: &[AttackGroup]) -> Vec<Repeat> {
    let width = groups.len() + 1;
    // Backtracking needs each direction, but scoring reads only the last row.
    let mut previous = vec![0.0; width];
    let mut current = vec![0.0; width];
    let mut back = vec![0_u8; width * width];
    let mut endings = vec![];
    for a in 1..width.saturating_sub(8) {
        current[a + 7] = 0.0;
        for b in a + 8..width {
            let (x, y) = (groups[a - 1].pitches, groups[b - 1].pitches);
            let similarity = f64::from((x & y).count_ones()) / f64::from((x | y).count_ones());
            let mut best = (0.0, 0);
            for (direction, value) in [
                (1, previous[b - 1] + 4.0 * similarity - 2.0),
                (2, previous[b] - 1.6),
                (3, current[b - 1] - 1.6),
            ] {
                if value > best.0 {
                    best = (value, direction);
                }
            }
            current[b] = best.0;
            back[a * width + b] = best.1;
            if best.0 >= 20.0 {
                endings.push((best.0, a, b));
            }
        }
        std::mem::swap(&mut previous, &mut current);
    }
    endings.sort_by(|a, b| {
        b.0.total_cmp(&a.0)
            .then_with(|| b.1.cmp(&a.1))
            .then_with(|| b.2.cmp(&a.2))
    });
    let mut occupied = vec![false; groups.len()];
    let mut repeats = vec![];
    for (_, mut a, mut b) in endings {
        if occupied[a - 1] || occupied[b - 1] {
            continue;
        }
        let mut aligned = vec![];
        while b > a + 4 {
            match back[a * width + b] {
                1 => {
                    if groups[a - 1].pitches == groups[b - 1].pitches {
                        aligned.push((a - 1, b - 1));
                    }
                    a -= 1;
                    b -= 1;
                }
                2 => a -= 1,
                3 => b -= 1,
                _ => break,
            }
        }
        aligned.reverse();
        let mut best = 0..0;
        let mut end = 0;
        for (start, &(_, right)) in aligned.iter().enumerate() {
            end = end.max(start);
            while end < aligned.len() && aligned[end].0 < right {
                end += 1;
            }
            if end - start > best.len() {
                best = start..end;
            }
        }
        let aligned = &aligned[best];
        if aligned.len() >= 12 && aligned.iter().all(|&(a, b)| !occupied[a] && !occupied[b]) {
            for &(a, b) in aligned {
                occupied[a] = true;
                occupied[b] = true;
            }
            repeats.push(aligned.to_vec());
        }
    }
    repeats
}

fn ranks(values: &[f64]) -> Vec<f64> {
    let mut order: Vec<_> = (0..values.len()).collect();
    order.sort_by(|&a, &b| values[a].total_cmp(&values[b]));
    let mut result = vec![0.0; values.len()];
    for (rank, index) in order.into_iter().enumerate() {
        result[index] = (rank as f64 + 0.5) / values.len() as f64 - 0.5;
    }
    result
}

fn observed_accents(groups: &[AttackGroup]) -> Vec<f64> {
    let low = ranks(&groups.iter().map(|g| -g.bass).collect::<Vec<_>>());
    let loud = ranks(&groups.iter().map(|g| g.velocity).collect::<Vec<_>>());
    let dense = ranks(
        &groups
            .iter()
            .map(|g| g.attacks.len() as f64)
            .collect::<Vec<_>>(),
    );
    groups
        .iter()
        .enumerate()
        .map(|(index, group)| {
            let local = &groups[index.saturating_sub(4)..(index + 5).min(groups.len())];
            let contour = (local.iter().map(|g| g.bass).sum::<f64>() / local.len() as f64
                - group.bass)
                / 12.0;
            ((low[index] + loud[index] + dense[index]) / 3.0 + contour) / 2.0
        })
        .collect()
}

#[derive(Clone, Copy)]
struct Meter {
    beats: u8,
    beat_type: u16,
    cycle: usize,
    pulse: usize,
}

const METERS: [Meter; 4] = [
    Meter {
        beats: 2,
        beat_type: 4,
        cycle: 8,
        pulse: 4,
    },
    Meter {
        beats: 3,
        beat_type: 4,
        cycle: 12,
        pulse: 4,
    },
    Meter {
        beats: 4,
        beat_type: 4,
        cycle: 16,
        pulse: 4,
    },
    Meter {
        beats: 6,
        beat_type: 8,
        cycle: 12,
        pulse: 6,
    },
];
const TEMPO_COUNT: usize = 61;

struct RhythmTrace {
    score: f64,
    phase: usize,
    onsets: Vec<i64>,
}

// Repetition supplies a correspondence between positions within the bar.
// The independently fitted course supplies the phase of those positions.
fn relative_repeat_anchors(
    meter: Meter, trace: &RhythmTrace, repetitions: &[Repeat],
) -> BTreeMap<usize, usize> {
    let phase_at = |index| (trace.phase + trace.onsets[index] as usize) % meter.cycle;
    repetitions.iter().flat_map(|repeat| {
        let first = repeat[0];
        let last = *repeat.last().expect("repetition is nonempty");
        [first, last].into_iter().flat_map(move |(a, b)| {
            let phase = phase_at(a);
            [(a, phase), (b, phase)]
        })
    }).collect()
}

struct RhythmEvidence<'a> {
    groups: &'a [AttackGroup],
    shared_pulse: bool,
    gaps: Vec<f64>,
    nominal: f64,
    accent: Vec<f64>,
    tempos: [f64; TEMPO_COUNT],
    transition: [[f64; TEMPO_COUNT]; TEMPO_COUNT],
    prior: [f64; TEMPO_COUNT],
}

impl<'a> RhythmEvidence<'a> {
    fn from_observations(
        groups: &'a [AttackGroup], repetitions: &[Repeat], nominal_unit: Option<f64>,
    ) -> Self {
        let original: Vec<_> = groups.windows(2).map(|g| g[1].time - g[0].time).collect();
        let nominal = nominal_unit.unwrap_or_else(|| {
            let mut nominal = 0.14;
            let mut error = f64::INFINITY;
            for step in 0..=80 {
                let candidate = 0.14 + f64::from(step) * 0.002;
                let residual = median(
                    original
                        .iter()
                        .map(|gap| {
                            let steps = gap / candidate;
                            (steps - steps.round().clamp(1.0, 12.0)).abs()
                        })
                        .collect(),
                );
                if residual < error {
                    nominal = candidate;
                    error = residual;
                }
            }
            nominal
        });
        let mut gaps = original.clone();
        for repetition in repetitions {
            let consecutive: Vec<_> = repetition
                .windows(2)
                .filter_map(|pairs| {
                    let ((a, b), (c, d)) = (pairs[0], pairs[1]);
                    (c == a + 1 && d == b + 1).then_some((a, b))
                })
                .collect();
            if consecutive.is_empty() {
                continue;
            }
            let scale = median(
                consecutive
                    .iter()
                    .map(|&(a, b)| original[a] / original[b])
                    .collect(),
            )
            .sqrt();
            for (a, b) in consecutive {
                let common = (original[a] * original[b]).sqrt();
                gaps[a] = scale * common;
                gaps[b] = common / scale;
            }
        }
        let tempos =
            std::array::from_fn(|i| nominal * 0.55 * (1.8_f64 / 0.55).powf(i as f64 / 60.0));
        let transition = std::array::from_fn(|to| {
            std::array::from_fn(|from| 100.0 * (tempos[from] / tempos[to]).ln().powi(2))
        });
        let prior = std::array::from_fn(|i| 9.0 * (tempos[i] / nominal).ln().powi(2));
        Self {
            groups,
            shared_pulse: nominal_unit.is_some(),
            gaps,
            nominal,
            accent: observed_accents(groups),
            tempos,
            transition,
            prior,
        }
    }

    fn trace(
        &self,
        meter: Meter,
        anchors: &BTreeMap<usize, usize>,
        factors: &[f64],
    ) -> RhythmTrace {
        let cycle = meter.cycle;
        let states = cycle * TEMPO_COUNT;
        let accent: Vec<_> = (0..cycle)
            .map(|p| {
                if p == 0 {
                    1.0
                } else if p % meter.pulse == 0 {
                    0.55
                } else if p % 2 == 0 {
                    0.0
                } else {
                    -0.25
                }
            })
            .collect();
        let mut costs: Vec<_> = (0..states)
            .map(|s| {
                let phase = s / TEMPO_COUNT;
                if anchors.get(&0).is_some_and(|&p| p != phase) {
                    f64::INFINITY
                } else {
                    -self.accent[0] * accent[phase] * 0.7
                }
            })
            .collect();
        let mut smoothed = vec![0.0; states];
        let mut tempo_from = vec![0; states];
        let mut back = vec![0_u16; self.gaps.len() * states];
        // These alternatives depend on the meter and its phase factors only.
        let transitions: Vec<Vec<_>> = (0..cycle).map(|phase| {
            (0..=16.min(cycle * 2)).map(|jump| {
                let previous = (phase + 2 * cycle - jump) % cycle;
                let complexity = match jump {
                    0 => 0.26,
                    3 => 0.24,
                    1 | 2 | 4 | 6 | 8 | 12 | 16 => 0.0,
                    _ => 1.3,
                } + jump.saturating_sub(4) as f64 * 0.07;
                let scale = if jump >= 6 { 0.7 } else { 1.0 };
                (previous * TEMPO_COUNT, jump as f64 * factors[previous], scale, 3.0 * complexity)
            }).collect()
        }).collect();
        for (index, &gap) in self.gaps.iter().enumerate() {
            for phase in 0..cycle {
                let row = phase * TEMPO_COUNT;
                for tempo in 0..TEMPO_COUNT {
                    let mut best = (f64::INFINITY, 0);
                    for previous in 0..TEMPO_COUNT {
                        let candidate = costs[row + previous] + self.transition[tempo][previous];
                        if candidate < best.0 {
                            best = (candidate, previous);
                        }
                    }
                    smoothed[row + tempo] = best.0;
                    tempo_from[row + tempo] = best.1;
                }
            }
            let allow_zero = gap <= self.nominal * 0.85
                && self.groups[index].pitches & self.groups[index + 1].pitches == 0;
            let shared_release = if self.shared_pulse && allow_zero {
                release_agreement(&self.groups[index], &self.groups[index + 1], self.nominal)
            } else {
                0.0
            };
            let required_phase = anchors.get(&(index + 1));
            for state in 0..states {
                let (phase, tempo) = (state / TEMPO_COUNT, state % TEMPO_COUNT);
                if required_phase.is_some_and(|&p| p != phase) {
                    costs[state] = f64::INFINITY;
                    continue;
                }
                let mut best = (f64::INFINITY, 0, 0);
                let performed = gap / self.tempos[tempo];
                for (jump, &(row, expected, scale, penalty)) in transitions[phase].iter().enumerate().skip(usize::from(!allow_zero)) {
                    let from = row + tempo;
                    let residual = performed - expected;
                    let spread_credit = if jump == 0 { shared_release } else { 0.0 };
                    let candidate = smoothed[from] - spread_credit
                        + scale
                            * (2.5 * residual * residual + self.prior[tempo] + penalty);
                    if candidate < best.0 {
                        best = (candidate, jump, tempo_from[from]);
                    }
                }
                costs[state] = best.0 - self.accent[index + 1] * accent[phase] * 0.7;
                back[index * states + state] = (best.1 * TEMPO_COUNT + best.2) as u16;
            }
        }
        let (mut state, &score) = costs
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.total_cmp(b.1))
            .expect("meter has states");
        let mut jumps = Vec::with_capacity(self.gaps.len());
        for row in back.chunks_exact(states).rev() {
            let code = usize::from(row[state]);
            let jump = code / TEMPO_COUNT;
            let phase = (state / TEMPO_COUNT + 2 * cycle - jump) % cycle;
            state = phase * TEMPO_COUNT + code % TEMPO_COUNT;
            jumps.push(jump);
        }
        let mut onsets = vec![0];
        for jump in jumps.into_iter().rev() {
            onsets.push(onsets.last().expect("initial onset") + jump as i64);
        }
        RhythmTrace {
            score,
            phase: state / TEMPO_COUNT,
            onsets,
        }
    }

    fn read_meter(
        &self,
        meter: Meter,
        repetitions: &[Repeat],
        confirmed_ends: &[(usize, usize)],
    ) -> RhythmTrace {
        let mut factors = vec![1.0; meter.cycle];
        let mut anchors = BTreeMap::new();
        if !self.shared_pulse {
            anchors.extend(repetitions.iter().flat_map(|repeat| {
                let (a, b) = repeat[0];
                [(a, 0), (b, 0)]
            }));
        }
        let initial = self.trace(meter, &anchors, &factors);
        let mut units = vec![vec![]; meter.cycle];
        for (i, pair) in initial.onsets.windows(2).enumerate() {
            if pair[1] > pair[0] {
                units[(initial.phase + pair[0] as usize) % meter.cycle]
                    .push(self.gaps[i] / (pair[1] - pair[0]) as f64);
            }
        }
        let baseline_values: Vec<_> = units.iter().flatten().copied().collect();
        if !baseline_values.is_empty() {
            let baseline = median(baseline_values);
            for (factor, values) in factors.iter_mut().zip(units) {
                let n = values.len() as f64;
                if n > 0.0 {
                    *factor = ((median(values) / baseline).clamp(0.75, 1.25) * n + 8.0) / (n + 8.0);
                }
            }
        }
        let refined = self.trace(meter, &anchors, &factors);
        if self.shared_pulse {
            anchors = relative_repeat_anchors(meter, &refined, repetitions);
        } else {
            for repeat in repetitions {
                let &(a, b) = repeat.last().expect("repetition is nonempty");
                anchors.insert(
                    b,
                    (refined.phase + refined.onsets[a] as usize) % meter.cycle,
                );
            }
        }
        let mut final_trace = self.trace(meter, &anchors, &factors);
        let crossings = confirmed_ends
            .iter()
            .filter(|&&(a, b)| {
                let start = final_trace.phase + final_trace.onsets[a] as usize;
                let end = final_trace.phase + final_trace.onsets[b] as usize;
                start < end
                    && !start.is_multiple_of(meter.pulse)
                    && start / meter.pulse != (end - 1) / meter.pulse
            })
            .count();
        final_trace.score += crossings as f64 * 0.22;
        final_trace
    }
}

// The rhythm trace proposes the pickup. Require the same downbeat in
// successive regions of independently observed simultaneous chord changes.
fn inferred_pickup(
    meter: Meter, trace: &RhythmTrace, groups: &[AttackGroup],
) -> Result<i64> {
    if trace.phase == 0 {
        return Ok(0);
    }
    let pickup = (meter.cycle - trace.phase) % meter.cycle;
    let edges = groups.windows(2).zip(trace.onsets.iter().skip(1));
    let mut votes = [vec![0; meter.cycle], vec![0; meter.cycle], vec![0; meter.cycle]];
    for (index, (pair, &onset)) in edges.enumerate() {
        let a = pair[0].pitches;
        let b = pair[1].pitches;
        if a.count_ones() < 2 || b.count_ones() < 2
            || a.trailing_zeros() % 12 == b.trailing_zeros() % 12
        {
            continue;
        }
        let region = index * votes.len() / (groups.len() - 1);
        votes[region][onset as usize % meter.cycle] += 1;
    }
    ensure!(
        votes.iter().all(|region| {
            region[pickup] >= 3
                && region.iter().enumerate().all(|(phase, &count)| {
                    phase == pickup || region[pickup] >= count + 2
                })
        }),
        "Auftakt im freien Performanceraster ist mehrdeutig"
    );
    Ok(pickup as i64 * UNIT)
}

// Normalize the number of bass changes by the available chord transitions
// at that position. One busy subdivision should not win merely by providing
// more opportunities. Each part of the recording must support the same phase.
fn shared_pulse_pickup(meter: Meter, onsets: &[i64], groups: &[AttackGroup]) -> Option<usize> {
    #[derive(Clone, Copy, Default)]
    struct ChordTransitions {
        observed: usize,
        changed: usize,
    }

    let mut regions = std::array::from_fn::<_, 3, _>(|_| {
        vec![ChordTransitions::default(); meter.cycle]
    });
    for (index, (pair, &onset)) in groups.windows(2).zip(onsets.iter().skip(1)).enumerate() {
        let before = pair[0].pitches;
        let after = pair[1].pitches;
        if before.count_ones() < 2 || after.count_ones() < 2 {
            continue;
        }
        let region = index * regions.len() / (groups.len() - 1);
        let evidence = &mut regions[region][onset as usize % meter.cycle];
        evidence.observed += 1;
        evidence.changed += usize::from(before.trailing_zeros() % 12 != after.trailing_zeros() % 12);
    }

    (0..meter.cycle).find(|&phase| {
        regions.iter().all(|region| {
            let preferred = region[phase];
            preferred.changed >= 3 && region.iter().enumerate().all(|(other_phase, other)| {
                other_phase == phase || other.changed < 3
                    // Add one changed and one unchanged observation to each
                    // side so a small perfect sample cannot dominate by itself.
                    || (preferred.changed + 1) * (other.observed + 2)
                        > (other.changed + 1) * (preferred.observed + 2)
            })
        })
    })
}

// In uneven rhythms a common pulse can be observed between attacks
// separated by either two or three intervening onset intervals.
fn common_pulse(groups: &[AttackGroup]) -> Option<f64> {
    const SECTIONS: usize = 3;
    if groups.len() < SECTIONS * 4 {
        return None;
    }
    let mut peaks = Vec::with_capacity(SECTIONS);
    for section in 0..SECTIONS {
        let region = &groups[section * groups.len() / SECTIONS..(section + 1) * groups.len() / SECTIONS];
        let gaps: Vec<_> = region.windows(2).map(|g| g[1].time - g[0].time).collect();
        let step = median(gaps);
        if !step.is_finite() || step <= 0.0 {
            return None;
        }
        let bandwidth = step / 8.0;
        let spans = [2, 3].map(|count| {
            region.windows(count + 1).map(|g| g[count].time - g[0].time).collect::<Vec<_>>()
        });
        let consensus = |period: f64| {
            spans.iter().map(|observed| {
                observed.iter().map(|duration| {
                    (-0.5 * ((duration - period) / bandwidth).powi(2)).exp()
                }).sum::<f64>() / observed.len() as f64
            }).fold(f64::INFINITY, f64::min)
        };
        let (period, support) = (64..=320).map(|n| {
            let period = f64::from(n) / 64.0 * step;
            (period, consensus(period))
        }).max_by(|a, b| a.1.total_cmp(&b.1))?;
        if support < 0.10 {
            return None;
        }
        peaks.push(period);
    }
    let typical = median(peaks.clone());
    peaks.iter().all(|peak| (peak / typical - 1.0).abs() <= 0.1).then_some(typical)
}

// Compare the written spans with the trend in the incoming repeated
// material. The measured interval under consideration is held out.
fn incoming_pulse_supports(
    groups: &[AttackGroup], onsets: &[i64], edge: usize, proposed: i64,
) -> bool {
    let previous = onsets[edge + 1] - onsets[edge];
    if previous <= 0 || proposed <= 0 || edge < 4 {
        return false;
    }
    let elapsed = groups[edge + 1].time - groups[edge].time;
    let proposed_pulse = (elapsed / proposed as f64).ln();
    let previous_pulse = (elapsed / previous as f64).ln();
    for width in 4..=6.min(edge) {
        let center = (width - 1) as f64 / 2.0;
        let mut sum = 0.0;
        let mut covariance = 0.0;
        let mut variance = 0.0;
        for (index, group) in (edge - width..edge).enumerate() {
            let span = onsets[group + 1] - onsets[group];
            if span <= 0 {
                return false;
            }
            let pulse = ((groups[group + 1].time - groups[group].time) / span as f64).ln();
            let offset = index as f64 - center;
            sum += pulse;
            covariance += offset * pulse;
            variance += offset * offset;
        }
        let prediction = sum / width as f64 + covariance / variance * (width as f64 - center);
        let proposed_error = (proposed_pulse - prediction).abs();
        let previous_error = (previous_pulse - prediction).abs();
        if !(proposed_error < previous_error && proposed_error <= 1.4_f64.ln()) {
            return false;
        }
    }
    true
}

// An independently supported change of written span shifts the following
// attacks together. Keep their intervals and the recurring phrase anchors.
fn corroborated_onsets(
    meter: Meter, trace: &RhythmTrace, groups: &[AttackGroup], repetitions: &[Repeat],
    releases: &[(usize, usize)],
) -> Option<Vec<i64>> {
    let transition_disagreements = |onsets: &[i64]| {
        repeated_transition_jumps(groups, onsets).into_iter()
            .zip(onsets.windows(2)).enumerate()
            .filter_map(|(index, (expected, positions))| {
                let actual = positions[1] - positions[0];
                expected.filter(|&span| span as i64 != actual)
                    .map(|span| (index, span as i64))
            })
            .collect::<Vec<_>>()
    };
    let old = &trace.onsets;
    let disagreements = transition_disagreements(old);
    if disagreements.is_empty() {
        return Some(old.clone());
    }
    let mut onsets = old.clone();
    for (edge, span) in disagreements {
        if !incoming_pulse_supports(groups, old, edge, span) {
            return None;
        }
        let delta = span - (old[edge + 1] - old[edge]);
        for onset in &mut onsets[edge + 1..] {
            *onset += delta;
        }
    }
    for repeat in repetitions {
        let first = repeat[0];
        let last = *repeat.last().expect("repetition is nonempty");
        for (a, b) in [first, last] {
            if (onsets[a] - onsets[b]).rem_euclid(meter.cycle as i64)
                != (old[a] - old[b]).rem_euclid(meter.cycle as i64)
            {
                return None;
            }
        }
    }
    let crosses = |course: &[i64], a, b| {
        let start = trace.phase as i64 + course[a];
        let end = trace.phase as i64 + course[b];
        let pulse = meter.pulse as i64;
        start < end && start % pulse != 0 && start / pulse != (end - 1) / pulse
    };
    for &(a, b) in releases {
        if crosses(&onsets, a, b) && !crosses(old, a, b) {
            return None;
        }
    }
    transition_disagreements(&onsets).is_empty().then_some(onsets)
}

fn shared_pulse_reading(
    groups: &[AttackGroup], repetitions: &[Repeat], releases: &[(usize, usize)],
) -> Result<(Meter, RhythmTrace, f64, i64)> {
    let pulse = common_pulse(groups)
        .ok_or_else(|| anyhow::anyhow!("Kein wiederkehrender gemeinsamer Puls belegt"))?;
    let mut accepted: Option<(Meter, RhythmTrace, f64, i64)> = None;
    for scale in [1.0, 0.96, 1.04] {
        let mut ranked = METERS.map(|meter| {
            let unit = pulse * scale / meter.pulse as f64;
            let timing = RhythmEvidence::from_observations(groups, repetitions, Some(unit));
            (meter, timing.read_meter(meter, repetitions, releases), unit)
        });
        ranked.sort_by(|a, b| a.1.score.total_cmp(&b.1.score));
        let [(meter, mut trace, unit), alternative, ..] = ranked;
        ensure!(trace.score.is_finite() && alternative.1.score - trace.score >= 1.0,
            "Gemeinsamer Puls stuetzt keine eindeutige Taktart");
        trace.onsets = corroborated_onsets(meter, &trace, groups, repetitions, releases)
            .ok_or_else(|| anyhow::anyhow!("Gemeinsamer Puls widerspricht rhythmisch belegten Akkordwechseln"))?;
        let pickup = shared_pulse_pickup(meter, &trace.onsets, groups)
            .ok_or_else(|| anyhow::anyhow!("Taktart aus gemeinsamem Puls ohne bestaetigte Taktphase"))?;
        ensure!(
            (trace.phase + pickup).is_multiple_of(meter.cycle),
            "Taktart aus gemeinsamem Puls widerspricht der harmonisch belegten Taktphase"
        );
        let pickup = pickup as i64 * UNIT;
        if let Some((prior_meter, prior_trace, _, prior_pickup)) = &accepted {
            ensure!((meter.beats, meter.beat_type, pickup)
                    == (prior_meter.beats, prior_meter.beat_type, *prior_pickup)
                && trace.onsets == prior_trace.onsets,
                "Gemeinsamer Puls stuetzt keinen stabilen Verlauf der Notationseinsaetze");
        } else {
            accepted = Some((meter, trace, unit, pickup));
        }
    }
    Ok(accepted.expect("central pulse reading"))
}

fn release_alignments(groups: &[AttackGroup]) -> Vec<(usize, usize)> {
    let mut pairs = vec![];
    for (index, group) in groups.iter().enumerate() {
        for attack in &group.attacks {
            let endpoint = attack.time + attack.hold;
            let next = groups.partition_point(|g| g.time < endpoint);
            if let Some(best) = [next.saturating_sub(1), next]
                .into_iter()
                .filter(|&j| j < groups.len())
                .min_by(|&a, &b| {
                    (groups[a].time - endpoint)
                        .abs()
                        .total_cmp(&(groups[b].time - endpoint).abs())
                })
                && best > index
                && (groups[best].time - endpoint).abs() < 0.045
            {
                pairs.push((index, best));
            }
        }
    }
    pairs
}

fn project_attacks(
    groups: &[AttackGroup],
    trace: &RhythmTrace,
    nominal: f64,
    count: usize,
) -> (Vec<i64>, Vec<Option<usize>>) {
    let mut positions = vec![0; count];
    let mut grace = vec![None; count];
    let mut lanes: BTreeMap<_, Vec<Attack>> = BTreeMap::default();
    for (group, &position) in groups.iter().zip(&trace.onsets) {
        for &attack in &group.attacks {
            positions[attack.id] = position * UNIT;
            lanes.entry(position).or_default().push(attack);
        }
    }
    let mut lanes = lanes.iter().peekable();
    while let Some((&position, attacks)) = lanes.next() {
        let hosts: Vec<_> = attacks
            .iter()
            .rev()
            .filter_map(|host| {
                let approach = grace_approach(*host, attacks, nominal);
                (!approach.is_empty()).then_some((host, approach))
            })
            .collect();
        for attack in attacks {
            let mut owners = hosts
                .iter()
                .filter(|(_, approach)| approach.contains(&attack.id));
            let Some((host, _)) = owners.next() else {
                continue;
            };
            if owners.next().is_none()
                && !hosts
                    .iter()
                    .any(|(_, approach)| approach.contains(&host.id))
            {
                grace[attack.id] = Some(host.id);
            }
        }
        // The half-step is available only inside this occupied interval. Use
        // release and short successive motion to distinguish it from a spread
        // chord. Additional simultaneous heads travel with their attack.
        if lanes
            .peek()
            .is_none_or(|&(&next, _)| (next - position) * UNIT > UNIT / 2)
        {
            let regular = attacks
                .iter()
                .filter(|a| grace[a.id].is_none())
                .collect::<Vec<_>>();
            if let Some(later) = half_step_attack(&regular, nominal) {
                for attack in later {
                    positions[attack.id] += UNIT / 2;
                }
            }
        }
    }
    (positions, grace)
}

fn grace_approach(host: Attack, attacks: &[Attack], nominal: f64) -> Vec<usize> {
    if host.hold < nominal {
        return Vec::new();
    }
    let mut approach = Vec::new();
    let mut next = host;
    for &attack in attacks.iter().rev() {
        let gap = next.time - attack.time;
        if !(0.0..=0.56 * nominal).contains(&gap)
            || gap == 0.0
            || host.time - attack.time > nominal
            || attack.hold >= 0.8 * nominal
            || 2.0 * attack.hold >= host.hold
            || !(1..=2).contains(&attack.pitch.abs_diff(next.pitch))
            || (attack.track, attack.channel) != (host.track, host.channel)
        {
            continue;
        }
        approach.push(attack.id);
        next = attack;
    }
    approach
}

fn half_step_attack<'a>(attacks: &[&'a Attack], nominal: f64) -> Option<Vec<&'a Attack>> {
    let first = attacks.first()?;
    let split = attacks.partition_point(|attack| attack.time - first.time < nominal * 0.25);
    let (early, late) = attacks.split_at(split);
    let late_start = late.first()?.time;
    if !(0.32 * nominal..=0.72 * nominal).contains(&(late_start - first.time))
        || late
            .iter()
            .any(|attack| attack.time - late_start >= nominal * 0.25 || attack.hold >= nominal)
        || !early.iter().any(|attack| {
            attack.hold < nominal
                && late.iter().any(|next| {
                    (attack.track, attack.channel) == (next.track, next.channel)
                        && attack.pitch.abs_diff(next.pitch) <= 3
                })
        })
    {
        return None;
    }
    Some(late.to_vec())
}

/// Finds minimum-cost nonoverlapping lane continuations. The additional
/// columns allow a release without consuming another attack.
fn assign_continuations(costs: &[Vec<f64>]) -> Vec<usize> {
    let n = costs.len();
    let m = n * 2;
    let mut row_price = vec![0.0; n + 1];
    let mut column_price = vec![0.0; m + 1];
    let mut owner = vec![0; m + 1];
    let mut parent = vec![0; m + 1];
    for row in 1..=n {
        owner[0] = row;
        let mut current = 0;
        let mut used = vec![false; m + 1];
        let mut best_reaching = vec![f64::INFINITY; m + 1];
        loop {
            used[current] = true;
            let route = owner[current];
            let mut step = (f64::INFINITY, 0);
            for column in 1..=m {
                if used[column] {
                    continue;
                }
                let cost = if column <= n {
                    costs[route - 1][column - 1]
                } else {
                    1.7
                };
                let reduced = cost - row_price[route] - column_price[column];
                if reduced < best_reaching[column] {
                    best_reaching[column] = reduced;
                    parent[column] = current;
                }
                if best_reaching[column] < step.0 {
                    step = (best_reaching[column], column);
                }
            }
            for column in 0..=m {
                if used[column] {
                    row_price[owner[column]] += step.0;
                    column_price[column] -= step.0;
                } else {
                    best_reaching[column] -= step.0;
                }
            }
            current = step.1;
            if owner[current] == 0 {
                break;
            }
        }
        while current != 0 {
            let previous = parent[current];
            owner[current] = owner[previous];
            current = previous;
        }
    }
    let mut columns = vec![n; n];
    for (index, &row) in owner.iter().enumerate().skip(1) {
        if row != 0 {
            columns[row - 1] = index - 1;
        }
    }
    columns
}

fn measured_durations(
    regular: &[Attack], positions: &[i64], nominal: f64, pulse: i64,
) -> Vec<f64> {
    let mut at_position: BTreeMap<i64, Vec<f64>> = BTreeMap::default();
    for attack in regular {
        at_position
            .entry(positions[attack.id])
            .or_default()
            .push(attack.time);
    }
    let mut map: Vec<_> = at_position
        .into_iter()
        .map(|(position, times)| (position, median(times)))
        .collect();
    let &(last, end) = map.last().expect("regular attacks");
    // Carry the approach tempo through the final release, which is no longer bracketed by attacks.
    let terminal_intervals: Vec<_> = map
        .windows(2)
        .rev()
        .take(4)
        .filter_map(|pair| {
            let elapsed = pair[1].1 - pair[0].1;
            (elapsed > 0.0).then_some(elapsed * UNIT as f64 / (pair[1].0 - pair[0].0) as f64)
        })
        .collect();
    let tail_tempo = map
        .iter()
        .find(|&&(position, time)| position == last - pulse && time < end)
        .map(|&(_, time)| (end - time) * UNIT as f64 / pulse as f64)
        .unwrap_or_else(|| {
            if terminal_intervals.is_empty() {
                nominal
            } else {
                median(terminal_intervals)
            }
        });
    map.push((last + 8 * UNIT, end + 8.0 * tail_tempo));
    regular
        .iter()
        .map(|attack| {
            let release = attack.time + attack.hold;
            let right = map
                .partition_point(|&(_, time)| time < release)
                .clamp(1, map.len() - 1);
            let (low, start) = map[right - 1];
            let (high, end) = map[right];
            let fraction = ((release - start) / (end - start)).clamp(0.0, 1.0);
            let bound = low as f64 + (high - low) as f64 * fraction;
            (bound - positions[attack.id] as f64) / UNIT as f64
        })
        .collect()
}

fn preferred_continuations(regular: &[Attack], positions: &[i64], expected: &[f64]) -> Vec<usize> {
    let costs: Vec<Vec<_>> = regular
        .iter()
        .zip(expected)
        .map(|(attack, &observed)| {
            regular
                .iter()
                .map(|next| {
                    let distance = positions[next.id] - positions[attack.id];
                    let pitch_gap = attack.pitch.abs_diff(next.pitch);
                    if !(1..=16 * UNIT).contains(&distance) || pitch_gap > 16 {
                        return 20.0;
                    }
                    let gap = distance as f64 / UNIT as f64;
                    let penalty = if VALUES.contains(&distance) {
                        0.0
                    } else {
                        0.15
                    };
                    let brief = if gap < 1.0 && observed >= 0.8 {
                        0.25
                    } else {
                        0.0
                    };
                    1.1 * ((gap.max(0.25) + 0.25) / (observed + 0.25)).log2().abs()
                        + f64::from(pitch_gap) * 0.065
                        + (gap - 8.0).max(0.0) * 0.08
                        + penalty
                        + brief
                })
                .collect()
        })
        .collect();
    assign_continuations(&costs)
}

/// On a separately established grid, certify a held interval only when its
/// assigned successor and the independent key-release evidence agree.
pub(super) fn independently_sustained_intervals(
    source: &DecodedMidiSource, events: &[NotationEvent],
) -> Result<BTreeMap<usize, WrittenHoldProposal>> {
    if events.len() > 1024 {
        return Ok(BTreeMap::new());
    }
    let attacks = source_attacks(source)?;
    Ok(sustained_intervals(&attacks, events))
}

fn sustained_intervals(attacks: &[Attack], events: &[NotationEvent]) -> BTreeMap<usize, WrittenHoldProposal> {
    let mut regular: Vec<_> = attacks.iter().copied()
        .filter(|attack| events[attack.id].kind == NotationEventKind::Regular)
        .collect();
    let positions: Vec<_> = events.iter().map(|event| event.pos).collect();
    regular.sort_by_key(|attack| (positions[attack.id], attack.pitch, attack.id));
    if regular.is_empty() {
        return BTreeMap::new();
    }
    let measured = measured_durations(&regular, &positions, 0.0, 4 * UNIT);
    let expected: Vec<_> = measured.iter().map(|length| 1.2 * length.max(0.6)).collect();
    let successors: Vec<_> = preferred_continuations(&regular, &positions, &expected)
        .into_iter()
        .zip(&regular)
        .map(|(index, attack)| {
            regular.get(index).filter(|next| {
                (attack.track, attack.channel) == (next.track, next.channel)
                    && VALUES.contains(&(positions[next.id] - positions[attack.id]))
            }).map(|_| index)
        })
        .collect();
    let mut predecessors = vec![None; regular.len()];
    for (index, &next) in successors.iter().enumerate() {
        if let Some(next) = next {
            predecessors[next] = Some(index);
        }
    }
    let mut latest_attacks = BTreeMap::<i64, f64>::new();
    for attack in &regular {
        latest_attacks.entry(positions[attack.id])
            .and_modify(|latest| *latest = latest.max(attack.time))
            .or_insert(attack.time);
    }
    let mut intervals = BTreeMap::new();
    for (index, &attack) in regular.iter().enumerate() {
        let Some(successor) = successors[index] else { continue };
        let start = positions[attack.id];
        let end = positions[regular[successor].id];
        let duration = end - start;
        let between: Vec<_> = regular.iter().enumerate().filter(|(_, other)| {
            let onset = positions[other.id];
            start < onset && onset < end && (attack.track, attack.channel) == (other.track, other.channel)
        }).collect();
        let Some((_, first)) = between.first() else { continue };
        let cut = positions[first.id];
        let release = attack.time + attack.hold;
        let past_midpoint = |earlier: i64| {
            latest_time_at(&latest_attacks, earlier + (end - earlier) / 2)
                .is_some_and(|time| release > time)
        };
        // Compare the selected value with the nearest shorter written value,
        // including dotted values. An established pulse in the same strand
        // can resolve the ambiguity when the performer released early.
        let shorter = WRITTEN_NOTE_VALUES.iter().copied()
            .find(|&value| value < duration)
            .map_or(cut, |value| cut.max(start + value));
        let shares_pulse = predecessors[index]
            .is_some_and(|before| start - positions[regular[before].id] == duration)
            && successors[successor]
                .is_some_and(|after| positions[regular[after].id] - end == duration);
        let crosses_independent_strand = shares_pulse
            && latest_time_at(&latest_attacks, cut).is_some_and(|time| release > time)
            && between.iter().any(|(inner, _)| successors[*inner]
                .is_some_and(|next| positions[regular[next].id] < end));
        let candidate_has_support = past_midpoint(cut) || crosses_independent_strand;
        if start + events[attack.id].len > cut
            && candidate_has_support
            && between.iter().all(|(_, other)| other.pitch != attack.pitch)
        {
            intervals.insert(attack.id, WrittenHoldProposal {
                early_end: cut, continuation: end,
                confirmed: past_midpoint(shorter) || shares_pulse,
            });
        }
    }
    intervals
}

fn latest_time_at(latest_attacks: &BTreeMap<i64, f64>, position: i64) -> Option<f64> {
    if let Some(&time) = latest_attacks.get(&position) {
        return Some(time);
    }
    let (&left_pos, &left_time) = latest_attacks.range(..position).next_back()?;
    let (&right_pos, &right_time) = latest_attacks.range(position..).next()?;
    let fraction = (position - left_pos) as f64 / (right_pos - left_pos) as f64;
    Some(left_time + fraction * (right_time - left_time))
}

fn written_lengths(
    regular: &[Attack],
    positions: &[i64],
    nominal: f64,
    groups: &[AttackGroup],
    repetitions: &[Repeat],
    trace: &RhythmTrace,
    pulse: i64,
) -> (Vec<i64>, BTreeSet<usize>) {
    let measured = measured_durations(regular, positions, nominal, pulse);
    let indices: BTreeMap<_, _> = regular
        .iter()
        .enumerate()
        .map(|(index, a)| (a.id, index))
        .collect();
    let mut expected: Vec<_> = measured
        .iter()
        .map(|&duration| 1.2 * duration.max(0.6))
        .collect();
    let unpaired = expected.clone();
    for alignment in repetitions {
        for triplet in alignment.windows(3) {
            let ((pa, pb), (a, b), (na, nb)) = (triplet[0], triplet[1], triplet[2]);
            if a != pa + 1
                || b != pb + 1
                || na != a + 1
                || nb != b + 1
                || trace.onsets[a] - trace.onsets[pa] != trace.onsets[b] - trace.onsets[pb]
                || trace.onsets[na] - trace.onsets[a] != trace.onsets[nb] - trace.onsets[b]
            {
                continue;
            }
            for x in &groups[a].attacks {
                let Some(y) = groups[b].attacks.iter().find(|y| y.pitch == x.pitch) else {
                    continue;
                };
                if positions[x.id] % UNIT != positions[y.id] % UNIT {
                    continue;
                }
                let (Some(&left), Some(&right)) = (indices.get(&x.id), indices.get(&y.id)) else {
                    continue;
                };
                let longer = unpaired[left].max(unpaired[right]);
                let shorter = unpaired[left].min(unpaired[right]);
                if longer <= 3.0 * shorter {
                    let shared = 0.25 * shorter + 0.75 * longer;
                    expected[left] = shared;
                    expected[right] = shared;
                }
            }
        }
    }
    let successors = preferred_continuations(regular, positions, &expected);
    let connected: Vec<_> = regular
        .iter()
        .zip(successors)
        .map(|(attack, target)| {
            let next = regular.get(target)?;
            let length = positions[next.id] - positions[attack.id];
            VALUES.contains(&length).then_some(WrittenSuccessor {
                length,
                same_source: (attack.track, attack.channel) == (next.track, next.channel),
            })
        })
        .collect();
    let chord_ends = unconnected_chord_ends(regular, positions, &connected, &measured, pulse);
    let mut lengths: Vec<_> = connected
        .iter()
        .enumerate()
        .map(|(index, successor)| {
            if let Some(next) = successor {
                return next.length;
            }
            let observed = expected[index];
            let error = |length| {
                let value = length as f64 / UNIT as f64;
                let brief = if value < 1.0 && observed >= 0.8 {
                    0.30
                } else {
                    0.0
                };
                ((value + 0.1) / (observed + 0.1)).ln().abs() + brief
            };
            *VALUES
                .iter()
                .min_by(|&&a, &&b| error(a).total_cmp(&error(b)))
                .expect("supported values")
        })
        .collect();
    let shared = shared_written_releases(
        groups,
        repetitions,
        trace,
        positions,
        &indices,
        &lengths,
    );
    let mut open_durations = BTreeSet::new();
    for (index, length) in lengths.iter_mut().enumerate() {
        if let Some(end) = shared[index].or(chord_ends[index]) {
            *length = end;
        } else if connected[index].is_none() {
            // Release rounding supplies a proposal. Staff and voice context may
            // still establish a different end before the seed receives it.
            open_durations.insert(regular[index].id);
        }
    }
    (lengths, open_durations)
}

#[derive(Clone, Copy)]
struct WrittenSuccessor {
    length: i64,
    same_source: bool,
}

impl WrittenSuccessor {
    fn local_length(self) -> Option<i64> {
        self.same_source.then_some(self.length)
    }
}

/// A dying voice can share a chord release even when the following chord has
/// fewer notes. Only independent, assigned successors can witness its end.
fn unconnected_chord_ends(
    regular: &[Attack],
    positions: &[i64],
    connected: &[Option<WrittenSuccessor>],
    measured: &[f64],
    pulse: i64,
) -> Vec<Option<i64>> {
    use std::ops::Bound::Excluded;

    let mut chords = BTreeMap::<_, BTreeMap<i64, Vec<usize>>>::new();
    for (index, attack) in regular.iter().enumerate() {
        chords
            .entry((attack.track, attack.channel))
            .or_default()
            .entry(positions[attack.id])
            .or_default()
            .push(index);
    }
    let mut shared = vec![None; regular.len()];
    for timeline in chords.values() {
        for (&start, members) in timeline {
            if members.len() < 2 {
                continue;
            }
            if members.iter().all(|&i| connected[i].is_none()) {
                if let Some(length) =
                    pulsed_release_before_rest(timeline, start, pulse, connected, measured)
                {
                    for &i in members {
                        shared[i] = Some(length);
                    }
                }
                continue;
            }
            let (first, last) = members
                .iter()
                .map(|&i| regular[i].time)
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), t| {
                    (low.min(t), high.max(t))
                });
            for &i in members.iter().filter(|&&i| connected[i].is_none()) {
                let attack = regular[i];
                let release = attack.time + attack.hold;
                if release <= last {
                    continue;
                }
                let mut witnesses = members.iter().filter_map(|&j| {
                    let other = regular[j];
                    let end = other.time + other.hold;
                    (end > last && (release - end).abs() <= last - first)
                        .then(|| connected[j].and_then(WrittenSuccessor::local_length))
                        .flatten()
                });
                let Some(length) = witnesses.next() else {
                    continue;
                };
                if witnesses.all(|other| other == length)
                    && !timeline.range((Excluded(start), Excluded(start + length)))
                        .flat_map(|(_, chord)| chord)
                        .any(|&j| regular[j].pitch == attack.pitch)
                {
                    shared[i] = Some(length);
                }
            }
        }
    }
    shared
}

/// Two preceding pulses establish a local subdivision only when every voice of
/// both chord attacks independently reaches the immediately following onset.
/// If that subdivision falls silent, a comparable articulation supports the
/// same written value followed by a rest. New articulation is independent.
fn pulsed_release_before_rest(
    timeline: &BTreeMap<i64, Vec<usize>>,
    position: i64,
    pulse: i64,
    connected: &[Option<WrittenSuccessor>],
    measured: &[f64],
) -> Option<i64> {
    use std::ops::Bound::{Excluded, Unbounded};

    let next_onset = |start| {
        timeline
            .range((Excluded(start), Unbounded))
            .next()
            .map(|(&p, _)| p)
    };
    let first = position - 2 * pulse;
    let length = next_onset(first)? - first;
    if length >= pulse || next_onset(position)? <= position + length {
        return None;
    }
    let mut shortest = f64::INFINITY;
    let mut longest = f64::NEG_INFINITY;
    for start in [first, position - pulse] {
        let chord = timeline.get(&start)?;
        if chord.len() < 2
            || next_onset(start)? != start + length
            || chord
                .iter()
                .any(|&i| connected[i].and_then(WrittenSuccessor::local_length) != Some(length))
        {
            return None;
        }
        for &i in chord {
            shortest = shortest.min(measured[i]);
            longest = longest.max(measured[i]);
        }
    }
    timeline[&position]
        .iter()
        .all(|&i| (shortest..=longest).contains(&measured[i]))
        .then_some(length)
}

fn shared_written_releases(
    groups: &[AttackGroup],
    repetitions: &[Repeat],
    trace: &RhythmTrace,
    positions: &[i64],
    indices: &BTreeMap<usize, usize>,
    lengths: &[i64],
) -> Vec<Option<i64>> {
    let chord = |group_index: usize| {
        let group = &groups[group_index];
        // A release cluster is credible when it is at least as tight as the
        // simultaneous attack and lies entirely after that attack.
        let first = group.attacks.first()?.time;
        let last = group.attacks.last()?.time;
        let (earliest, latest) = group
            .attacks
            .iter()
            .map(|a| a.time + a.hold)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), time| {
                (lo.min(time), hi.max(time))
            });
        if group.attacks.len() < 2 || earliest <= last || latest - earliest > last - first {
            return None;
        }
        let position = trace.onsets[group_index] * UNIT;
        group
            .attacks
            .iter()
            .map(|attack| {
                (positions[attack.id] == position)
                    .then(|| indices.get(&attack.id).copied())
                    .flatten()
            })
            .collect::<Option<Vec<_>>>()
    };
    let mut shared = vec![None; lengths.len()];
    for alignment in repetitions {
        for (index, &(a, b)) in alignment.iter().enumerate() {
            let same_gap = |(x, y): (usize, usize), (u, v): (usize, usize)| {
                x + 1 == u
                    && y + 1 == v
                    && trace.onsets[u] - trace.onsets[x] == trace.onsets[v] - trace.onsets[y]
            };
            let aligned_neighbor = index
                .checked_sub(1)
                .is_some_and(|prior| same_gap(alignment[prior], (a, b)))
                || alignment
                    .get(index + 1)
                    .is_some_and(|&next| same_gap((a, b), next));
            if !aligned_neighbor {
                continue;
            }
            let (Some(left), Some(right)) = (chord(a), chord(b)) else {
                continue;
            };
            let range = |members: &[usize]| {
                members
                    .iter()
                    .fold((i64::MAX, i64::MIN), |(low, high), &i| {
                        (low.min(lengths[i]), high.max(lengths[i]))
                    })
            };
            let (left_min, left_max) = range(&left);
            let (right_min, right_max) = range(&right);
            let (target, first, length) =
                if left_min == left_max && right_min < left_min && left_min < right_max {
                    (b, right, left_min)
                } else if right_min == right_max && left_min < right_min && right_min < left_max {
                    (a, left, right_min)
                } else {
                    continue;
                };
            let end = trace.onsets[target] * UNIT + length;
            // A shared written end may span intervening voices, but it must
            // never run through a later onset of one of the same keys.
            if groups[target + 1..]
                .iter()
                .take_while(|group| group.attacks.iter().any(|a| positions[a.id] < end))
                .any(|group| {
                    group.attacks.iter().any(|attack| {
                        indices.contains_key(&attack.id)
                            && positions[attack.id] < end
                            && groups[target].attacks.iter().any(|held| {
                                held.pitch == attack.pitch && held.channel == attack.channel
                            })
                    })
                })
            {
                continue;
            }
            for i in first {
                shared[i] = Some(length);
            }
        }
    }
    shared
}

fn opening_key(regular: &[Attack], positions: &[i64], lengths: &[i64]) -> Option<MidiKeySignature> {
    let major: [f64; 12] = [
        6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88,
    ];
    let minor: [f64; 12] = [
        6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17,
    ];
    let last = regular.iter().map(|attack| positions[attack.id]).max()?;
    let mut classes = [0.0; 12];
    let mut final_chord = [false; 12];
    for (attack, &length) in regular.iter().zip(lengths) {
        let pitch_class = usize::from(attack.pitch % 12);
        classes[pitch_class] += length as f64;
        if positions[attack.id] == last {
            final_chord[pitch_class] = true;
        }
    }
    let mean = classes.iter().sum::<f64>() / 12.0;
    let mut best = None;
    for (minor, template, third, fifths) in [
        (false, major, 4, [0, -5, 2, -3, 4, -1, -6, 1, -4, 3, -2, 5]),
        (true, minor, 3, [-3, 4, -1, -6, 1, -4, 3, -2, 5, 0, -5, 2]),
    ] {
        let center = template.iter().sum::<f64>() / 12.0;
        let scale = template
            .iter()
            .map(|v| (v - center).powi(2))
            .sum::<f64>()
            .sqrt();
        for tonic in 0..12 {
            if final_chord[tonic] && final_chord[(tonic + third) % 12] {
                let score = template
                    .iter()
                    .enumerate()
                    .map(|(class, weight)| {
                        (classes[(class + tonic) % 12] - mean) * (weight - center)
                    })
                    .sum::<f64>()
                    / scale;
                if best.is_none_or(|(prior, _)| score > prior) {
                    best = Some((
                        score,
                        MidiKeySignature {
                            fifths: fifths[tonic],
                            minor,
                        },
                    ));
                }
            }
        }
    }
    best.map(|(_, key)| key)
}

fn validate_audio(
    groups: &[AttackGroup],
    quiet: &[Attack],
    samples: &[f32],
    rate: u32,
) -> Result<()> {
    ensure!(
        rate >= 1_000 && !samples.is_empty(),
        "WAV hat keine auswertbare Zeitbasis"
    );
    let hop = (rate / 200).max(1) as usize;
    let time_per_frame = hop as f64 / f64::from(rate);
    let coefficients = [250.0, 700.0, 1_800.0, 4_500.0]
        .map(|f| 1.0 - (-std::f64::consts::TAU * f / f64::from(rate)).exp());
    let mut filters = [0.0; 4];
    let mut previous_levels = [0.0; 4];
    let mut flux = Vec::with_capacity(samples.len().div_ceil(hop));
    for frame in samples.chunks(hop) {
        let mut bands = [0.0; 4];
        for &sample in frame {
            ensure!(
                sample.is_finite(),
                "WAV enthaelt nicht endliche Samplewerte"
            );
            let sample = f64::from(sample);
            for (filter, coefficient) in filters.iter_mut().zip(coefficients) {
                *filter += (sample - *filter) * coefficient;
            }
            for (index, energy) in bands.iter_mut().enumerate() {
                let band = if index < 3 {
                    filters[index + 1] - filters[index]
                } else {
                    sample - filters[index]
                };
                *energy += band * band;
            }
        }
        let levels = bands.map(|sum| (1.0 + 1_000.0 * (sum / frame.len() as f64).sqrt()).ln());
        flux.push(
            levels
                .iter()
                .zip(previous_levels)
                .map(|(&a, b)| (a - b).max(0.0))
                .sum::<f64>(),
        );
        previous_levels = levels;
    }
    let mut prefix = vec![0.0];
    for &value in &flux {
        prefix.push(prefix.last().expect("prefix starts at zero") + value);
    }
    let count = flux.len();
    for (i, value) in flux.iter_mut().enumerate() {
        let start = i.saturating_sub(45);
        let end = (i + 46).min(count);
        *value /= ((prefix[end] - prefix[start]) / (end - start) as f64).max(0.025);
    }
    let envelope = |seconds: f64| {
        let center = (seconds / time_per_frame).round() as isize;
        (-2..=2)
            .filter_map(|offset| {
                usize::try_from(center + offset)
                    .ok()
                    .and_then(|frame| flux.get(frame).copied())
            })
            .fold(0.0, f64::max)
    };
    let offset = matching_audio_offset(
        groups,
        &envelope,
        time_per_frame,
        samples.len() as f64 / f64::from(rate),
    )?;
    let typical = median(groups.iter().map(|g| envelope(g.time + offset)).collect());
    for attack in quiet {
        let nearby = (-4..=4)
            .map(|step| envelope(attack.time + offset + f64::from(step) * 0.005))
            .fold(0.0, f64::max);
        ensure!(
            nearby < typical,
            "Leiser MIDI-Anschlag {} besitzt akustische Einsatz-Evidenz",
            attack.id
        );
    }
    Ok(())
}

fn matching_audio_offset(
    groups: &[AttackGroup],
    envelope: &impl Fn(f64) -> f64,
    frame_seconds: f64,
    audio_seconds: f64,
) -> Result<f64> {
    const STEP_SECONDS: f64 = 0.005;
    const RANGE_STEPS: i32 = 200;
    const LOCAL_TOLERANCE_STEPS: i32 = 8;
    const DISTINCT_OFFSET_SECONDS: f64 = 0.082;
    let first = groups[0].time;
    let last = groups[groups.len() - 1].time;
    let score = |part: &[AttackGroup], offset: f64| {
        part.iter()
            .map(|group| envelope(group.time + offset))
            .sum::<f64>()
            / part.len() as f64
    };
    let covers_attacks =
        |offset| first + offset >= -frame_seconds && last + offset <= audio_seconds;
    let steps = -RANGE_STEPS..=RANGE_STEPS;
    let mut scores: Vec<_> = steps
        .clone()
        .map(|step| {
            let offset = f64::from(step) * STEP_SECONDS;
            (score(groups, offset), offset)
        })
        .collect();
    let peak = |scores: &[(f64, f64)], require_coverage: bool| {
        scores
            .iter()
            .copied()
            .filter(|&(_, offset)| !require_coverage || covers_attacks(offset))
            .max_by(|a, b| a.0.total_cmp(&b.0))
    };
    let has_global_support = |height, scores: &[(f64, f64)]| {
        let floor = median(scores.iter().map(|s| s.0).collect());
        floor > 0.0 && height >= 1.5 * floor
    };
    let has_local_support = |offset, scores: &[(f64, f64)], tolerance: i32| {
        groups.chunks(groups.len().div_ceil(3)).all(|part| {
            let unrelated = median(
                scores
                    .iter()
                    .map(|&(_, shift)| score(part, shift))
                    .collect(),
            );
            let nearby = (-tolerance..=tolerance)
                .map(|step| score(part, offset + f64::from(step) * STEP_SECONDS))
                .fold(0.0, f64::max);
            unrelated > 0.0 && nearby >= 1.25 * unrelated
        })
    };
    let (height, offset) = peak(&scores, false).expect("unshifted audio search");
    let original_failure = if !has_global_support(height, &scores) {
        "WAV bestaetigt die MIDI-Einsaetze nicht eindeutig"
    } else if !covers_attacks(offset) {
        "WAV deckt die MIDI-Einsaetze zeitlich nicht ab"
    } else if !has_local_support(offset, &scores, 0) {
        "WAV bestaetigt die Zeitzuordnung nicht ueber den Spielverlauf"
    } else {
        return Ok(offset);
    };
    if first > f64::from(RANGE_STEPS) * STEP_SECONDS {
        // An audio export can cut its leading silence independently of the MIDI
        // time origin. Test that origin separately and require a distinct peak.
        scores.extend(steps.filter_map(|step| {
            let offset = f64::from(step) * STEP_SECONDS - first;
            (offset < -f64::from(RANGE_STEPS) * STEP_SECONDS)
                .then(|| (score(groups, offset), offset))
        }));
        if let Some((height, offset)) = peak(&scores, true)
            && has_global_support(height, &scores)
            && scores.iter().all(|&(alternative, other)| {
                (offset - other).abs() <= DISTINCT_OFFSET_SECONDS || height >= 1.15 * alternative
            })
            && has_local_support(offset, &scores, LOCAL_TOLERANCE_STEPS)
        {
            return Ok(offset);
        }
    }
    anyhow::bail!("{original_failure}")
}

pub(super) fn infer_performance(
    source: &DecodedMidiSource,
    samples: &[f32],
    sample_rate: u32,
) -> Result<FreePerformance> {
    ensure!(
        source.notes.len() <= 1024,
        "Freies Performanceraster ist fuer diese Eingabegroesse nicht belegt"
    );
    let attacks = source_attacks(source)?;
    let (groups, quiet) = attack_groups(&attacks);
    ensure!(
        groups.len() >= 32,
        "Zu wenige getrennte Einsatzgruppen fuer freies Performanceraster"
    );
    validate_audio(&groups, &quiet, samples, sample_rate)?;
    let repetitions = repeated_phrases(&groups);
    ensure!(
        !repetitions.is_empty(),
        "Taktphase ohne wiederkehrende Anschlagstruktur mehrdeutig"
    );
    let timing = RhythmEvidence::from_observations(&groups, &repetitions, None);
    let releases = release_alignments(&groups);
    let mut ranked = METERS.map(|meter| (meter, timing.read_meter(meter, &repetitions, &releases)));
    ranked.sort_by(|a, b| a.1.score.total_cmp(&b.1.score));
    let [(meter, trace), alternative, ..] = ranked;
    let (meter, trace, nominal, pickup) = if trace.score.is_finite()
        && alternative.1.score - trace.score >= 1.0
    {
        let pickup = inferred_pickup(meter, &trace, &groups)?;
        (meter, trace, timing.nominal, pickup)
    } else {
        shared_pulse_reading(&groups, &repetitions, &releases)
            .map_err(|error| error.context("MIDI-Einsaetze stuetzen keine eindeutige Taktart"))?
    };
    ensure!(
        source.time_signatures.is_empty()
            || source.time_signatures == [(meter.beats, meter.beat_type)],
        "MIDI-Taktart und abgeleitetes Performanceraster widersprechen sich"
    );
    let (positions, grace) = project_attacks(&groups, &trace, nominal, attacks.len());
    let mut unique = BTreeSet::default();
    let mut regular: Vec<_> = groups
        .iter()
        .flat_map(|g| &g.attacks)
        .copied()
        .filter(|attack| grace[attack.id].is_none())
        .collect();
    ensure!(
        !regular.is_empty()
            && regular
                .iter()
                .all(|a| unique.insert((positions[a.id], a.channel, a.pitch))),
        "Gleichnamige Anschlaege fallen im Performanceraster zusammen"
    );
    regular.sort_by_key(|a| (positions[a.id], a.pitch, a.id));
    let (lengths, open_durations) = written_lengths(
        &regular,
        &positions,
        nominal,
        &groups,
        &repetitions,
        &trace,
        meter.pulse as i64 * UNIT,
    );
    let key = opening_key(&regular, &positions, &lengths);
    let mut events: Vec<_> = regular
        .iter()
        .zip(lengths)
        .map(|(a, len)| NotationEvent {
            source_ordinal: a.id,
            pos: positions[a.id],
            len,
            kind: NotationEventKind::Regular,
        })
        .collect();
    for (id, host) in grace
        .into_iter()
        .enumerate()
        .filter_map(|(id, host)| Some((id, host?)))
    {
        ensure!(
            events
                .iter()
                .any(|event| event.source_ordinal == host
                    && event.kind == NotationEventKind::Regular),
            "Vorschlag ohne regulaere Hauptnote"
        );
        events.push(NotationEvent {
            source_ordinal: id,
            pos: positions[host],
            len: 0,
            kind: NotationEventKind::Grace { host },
        });
    }
    for attack in quiet {
        events.push(NotationEvent {
            source_ordinal: attack.id,
            pos: 0,
            len: 0,
            kind: NotationEventKind::PlaybackOnly,
        });
    }
    events.sort_by_key(|event| event.source_ordinal);
    ensure!(
        events
            .iter()
            .enumerate()
            .all(|(i, event)| event.source_ordinal == i)
            && events.len() == attacks.len(),
        "Abgeleitete Notationszeit deckt die Quellnoten nicht vollstaendig"
    );
    Ok(FreePerformance {
        beats: meter.beats,
        beat_type: meter.beat_type,
        pickup,
        key,
        events,
        open_durations,
    })
}

#[cfg(test)]
mod repeated_ending_release_tests {
    use super::*;

    #[test]
    fn approach_is_independent_of_unrelated_chord_members() {
        let note = |id, time, hold, pitch| Attack {
            id,
            time,
            hold,
            pitch,
            velocity: 80,
            track: 0,
            channel: 0,
        };
        let approach = note(0, 0.0, 0.3, 62);
        let host = note(1, 0.4, 2.0, 64);
        let bass = note(2, 0.008, 2.3, 47);
        assert_eq!(grace_approach(host, &[approach, host], 1.0), vec![0]);
        assert_eq!(grace_approach(host, &[approach, bass, host], 1.0), vec![0]);
        assert!(grace_approach(note(1, 0.4, 0.5, 64), &[approach, host], 1.0).is_empty());
        let later = note(3, 0.5, 0.6, 64);
        let early = note(0, 0.0, 0.6, 62);
        let displaced = half_step_attack(&[&early, &bass, &later], 1.0).unwrap();
        assert_eq!(
            displaced.iter().map(|note| note.id).collect::<Vec<_>>(),
            vec![3]
        );
        let rolled = note(3, 0.5, 1.5, 64);
        assert!(half_step_attack(&[&early, &bass, &rolled], 1.0).is_none());
    }

    #[test]
    fn a_repeated_chord_uses_common_release_evidence_and_preserves_independent_releases() {
        let attack = |id, time, release, pitch| Attack {
            id,
            time,
            hold: release - time,
            pitch,
            velocity: 80,
            track: 0,
            channel: 0,
        };
        let group = |attacks: Vec<Attack>| AttackGroup {
            time: attacks[0].time,
            pitches: attacks
                .iter()
                .fold(0, |bits, note| bits | (1_u128 << note.pitch)),
            bass: f64::from(attacks.iter().map(|note| note.pitch).min().unwrap()),
            velocity: 80.0,
            attacks,
        };
        let trace = RhythmTrace {
            score: 0.0,
            phase: 0,
            onsets: vec![0, 6, 12, 18],
        };
        let positions = [0, 144, 144, 144, 288, 432, 432, 432];
        let indices = (0..positions.len()).map(|id| (id, id)).collect();
        let repeat = vec![(0, 2), (1, 3)];
        let original = [144, 48, 144, 48, 144, 96, 96, 96];
        for (middle_release, expected) in [(1.59, 96), (1.9, 144)] {
            let groups = [
                group(vec![attack(0, 0.0, 0.83, 62)]),
                group(vec![
                    attack(1, 1.00, 1.58, 55),
                    attack(2, 1.03, middle_release, 59),
                    attack(3, 1.06, 1.60, 67),
                ]),
                group(vec![attack(4, 2.0, 2.83, 62)]),
                group(vec![
                    attack(5, 3.00, 3.70, 55),
                    attack(6, 3.03, 3.71, 59),
                    attack(7, 3.06, 3.72, 67),
                ]),
            ];
            let shared = shared_written_releases(
                &groups,
                std::slice::from_ref(&repeat),
                &trace,
                &positions,
                &indices,
                &original,
            );
            let lengths: Vec<_> = original
                .into_iter()
                .zip(shared)
                .map(|(value, candidate)| candidate.unwrap_or(value))
                .collect();
            assert_eq!(lengths[2], expected);
            let expected_outer = if expected == 96 { 96 } else { 48 };
            assert_eq!(lengths[1], expected_outer);
            assert_eq!(lengths[3], expected_outer);
            assert_eq!(lengths[4..], original[4..]);
        }
    }
}

#[cfg(test)]
mod audio_offset_tests {
    use super::*;

    fn groups(start: f64) -> Vec<AttackGroup> {
        (0..36)
            .scan(start, |time, index| {
                let group = AttackGroup {
                    attacks: vec![],
                    time: *time,
                    pitches: 1,
                    bass: 60.0,
                    velocity: 80.0,
                };
                *time += 0.24 + f64::from((index * index + 7 * index) % 13) / 40.0;
                Some(group)
            })
            .collect()
    }

    #[test]
    fn trimmed_audio_origin_needs_one_peak_supported_across_the_recording() {
        let groups = groups(4.0);
        let times: Vec<_> = groups.iter().map(|group| group.time - 3.98).collect();
        let duration = times[times.len() - 1] + 0.8;
        let impulse = |time: f64, offset: f64, onsets: &[f64]| {
            onsets
                .iter()
                .any(|onset| (time - onset - offset).abs() < 0.009)
        };
        let envelope = |time| 0.2 + f64::from(impulse(time, 0.0, &times));
        let offset = matching_audio_offset(&groups, &envelope, 0.005, duration).unwrap();
        assert!((offset + 3.98).abs() <= 0.01);

        let ambiguous =
            |time| 0.2 + f64::from(impulse(time, 0.0, &times) || impulse(time, 0.20, &times));
        assert!(matching_audio_offset(&groups, &ambiguous, 0.005, duration).is_err());

        let spliced = |time| {
            0.2 + f64::from(impulse(time, 0.0, &times[..24]) || impulse(time, 0.18, &times[24..]))
        };
        assert!(matching_audio_offset(&groups, &spliced, 0.005, duration).is_err());
        assert!(matching_audio_offset(&groups, &envelope, 0.005, duration - 1.5).is_err());
    }

    #[test]
    fn shared_origin_keeps_the_existing_alignment() {
        let groups = groups(0.5);
        let envelope = |time: f64| {
            0.2 + f64::from(groups.iter().any(|g| (time - g.time - 0.04).abs() < 0.012))
        };
        let duration = groups[groups.len() - 1].time + 0.5;
        let offset = matching_audio_offset(&groups, &envelope, 0.005, duration).unwrap();
        assert!((offset - 0.04).abs() <= 0.015);
    }
}

#[cfg(test)]
mod pickup_tests {
    use super::*;

    #[test]
    fn pickup_requires_phase_agreement_across_the_performance() {
        // Build three regions with recurring downbeat chord changes
        // and chord changes on varying offbeats between them.
        let mut groups = Vec::new();
        let mut trace = RhythmTrace { score: 0.0, phase: 4, onsets: Vec::new() };
        for i in 0..18_i64 {
            let bass = if i % 2 == 0 { 53 } else { 60 };
            groups.push(AttackGroup {
                attacks: vec![],
                time: i as f64,
                pitches: (1_u128 << bass) | (1_u128 << (bass + 7)),
                bass: f64::from(bass),
                velocity: 80.0,
            });
            trace.onsets.push(if i == 0 { 0 } else if i % 2 == 1 {
                12 * (i / 2) + 8
            } else {
                12 * (i / 2) + i % 3
            });
        }
        let meter = METERS[3];
        assert_eq!(inferred_pickup(meter, &trace, &groups).unwrap(), 8 * UNIT);

        let last_region = trace.onsets.len() * 2 / 3;
        for onset in &mut trace.onsets[last_region..] {
            *onset += 1;
        }
        assert!(inferred_pickup(meter, &trace, &groups).is_err());
        trace.phase = 0;
        assert_eq!(inferred_pickup(meter, &trace, &groups).unwrap(), 0);
    }

    #[test]
    fn shared_pulse_pickup_compares_opportunities_in_each_region() {
        let meter = METERS[3];
        let mut groups = Vec::new();
        let mut onsets = Vec::new();
        let mut bass = 48_u32;
        let add = |tick, change, groups: &mut Vec<AttackGroup>, onsets: &mut Vec<i64>, bass: &mut u32| {
            if change {
                *bass = if *bass == 48 { 53 } else { 48 };
            }
            groups.push(AttackGroup {
                attacks: vec![],
                time: tick as f64,
                pitches: (1_u128 << *bass) | (1_u128 << (*bass + 7)),
                bass: f64::from(*bass),
                velocity: 70.0,
            });
            onsets.push(tick);
        };
        add(0, false, &mut groups, &mut onsets, &mut bass);
        for region in 0..3 {
            for bar in 0..6 {
                let start = (region * 6 + bar) * meter.cycle as i64;
                add(start + 1, bar < 3, &mut groups, &mut onsets, &mut bass);
                if bar < 3 || (region == 2 && bar == 3) {
                    add(start + 7, bar < 3, &mut groups, &mut onsets, &mut bass);
                } else {
                    add(start + 9, false, &mut groups, &mut onsets, &mut bass);
                }
            }
        }
        assert_eq!(shared_pulse_pickup(meter, &onsets, &groups), Some(7));

        // The same number of opportunities on both phases removes the
        // distinction in the final region, so phase must stay unresolved.
        for onset in onsets.iter_mut().skip(25) {
            if *onset % meter.cycle as i64 == 9 {
                *onset -= 2;
            }
        }
        assert_eq!(shared_pulse_pickup(meter, &onsets, &groups), None);
    }

    #[test]
    fn common_pulse_needs_support_from_the_whole_performance() {
        let positions = [0.0, 0.2, 0.6, 0.8, 1.0];
        let groups = |speeds: [f64; 3]| {
            speeds.into_iter().enumerate().flat_map(|(section, scale)| {
                (0..6).flat_map(move |bar| positions.map(move |offset| {
                    let time = (section as f64 * 30.0) + (f64::from(bar) * 1.2 + offset) * scale;
                    AttackGroup {
                        attacks: vec![],
                        time,
                        pitches: 1,
                        bass: 60.0,
                        velocity: 80.0,
                    }
                }))
            }).collect::<Vec<_>>()
        };
        let steady = common_pulse(&groups([1.0, 1.02, 0.98])).unwrap();
        assert!((steady - 0.60).abs() < 0.025);
        assert!(common_pulse(&groups([1.0, 1.0, 1.4])).is_none());
    }
}

#[cfg(test)]
mod split_attack_release_tests {
    use super::*;

    fn attack(id: usize, on: f64, off: f64, pitch: u8, channel: u8) -> AttackGroup {
        AttackGroup {
            attacks: vec![Attack { id, time: on, hold: off - on, pitch, velocity: 80, track: 0, channel }],
            time: on,
            pitches: 1_u128 << pitch,
            bass: f64::from(pitch),
            velocity: 80.0,
        }
    }

    #[test]
    fn shared_releases_support_spread_chords_over_quick_melodic_steps() {
        for (origin, scale, transpose) in [(0.0, 1.0, 0), (4.2, 0.67, 7), (1.0, 2.0, 12)] {
            let group = |id, on, off, pitch: u8, channel| {
                attack(id, origin + on * scale, origin + off * scale, pitch + transpose, channel)
            };
            let early = group(0, 0.0, 0.39, 49, 0);
            let spread = group(1, 0.1, 0.4, 56, 0);
            let longer_voice = group(1, 0.1, 0.65, 56, 0);
            let other_channel = group(1, 0.1, 0.4, 56, 1);
            let short = group(0, 0.0, 0.095, 49, 0);
            assert!(release_agreement(&early, &spread, 0.2 * scale) > 0.95);
            assert!(release_agreement(&early, &longer_voice, 0.2 * scale) < 0.001);
            assert_eq!(release_agreement(&early, &other_channel, 0.2 * scale), 0.0);
            assert_eq!(release_agreement(&short, &spread, 0.2 * scale), 0.0);
        }
    }
}


#[cfg(test)]
mod repeated_transition_tests {
    use super::*;

    #[test]
    fn recurring_chord_change_needs_agreeing_peers_and_local_tempo_support() {
        let pitch_sets = [[45, 57], [47, 59], [43, 55]];
        let mut groups = [0.0, 0.4, 0.8, 1.2, 1.6, 2.0, 2.4, 2.82]
            .into_iter()
            .enumerate()
            .map(|(i, time)| AttackGroup {
                time,
                attacks: vec![],
                pitches: pitch_sets[i % 3].iter().fold(0, |bits, pitch| bits | (1 << pitch)),
                bass: f64::from(pitch_sets[i % 3][0]),
                velocity: 80.0,
            })
            .collect::<Vec<_>>();
        let onsets = [0, 2, 4, 6, 8, 10, 12, 15];
        let evidence = repeated_transition_jumps(&groups, &onsets);
        assert_eq!(evidence[6], Some(2));
        assert_eq!(evidence[0], None);
        assert_eq!(evidence[3], None);
        assert_eq!(evidence[2], None);

        groups[7].time = 3.2;
        assert_eq!(repeated_transition_jumps(&groups, &onsets)[6], None);

        groups[7].time = 2.82;
        let conflicting_peers = [0, 2, 4, 6, 9, 11, 13, 16];
        assert_eq!(repeated_transition_jumps(&groups, &conflicting_peers)[6], None);
    }
}

#[cfg(test)]
mod relative_repeat_anchor_tests {
    use super::*;

    #[test]
    fn corresponding_phrase_positions_follow_the_clock_even_away_from_downbeats() {
        let meter = METERS[3];
        let repeats = [vec![(1, 4), (2, 5), (3, 6)]];
        let onsets = vec![0, 2, 5, 7, 15, 18, 20];
        for phase in [0, 2, 6] {
            let trace = RhythmTrace { score: 0.0, phase, onsets: onsets.clone() };
            let anchors = relative_repeat_anchors(meter, &trace, &repeats);
            assert_eq!(
                anchors,
                BTreeMap::from([(1, phase + 2), (3, (phase + 7) % meter.cycle),
                               (4, phase + 2), (6, (phase + 7) % meter.cycle)])
            );
        }
    }
}


#[cfg(test)]
mod corroborated_onsets_tests {
    use super::*;

    #[test]
    fn repeated_span_needs_incoming_tempo_and_preserves_relative_phrase_positions() {
        let meter = METERS[3];
        let mut groups: Vec<_> = (0..11).map(|index| {
            let bass = if index == 10 { 41 } else if index % 2 == 0 { 45 } else { 48 };
            AttackGroup {
                time: index as f64 * 0.4,
                attacks: vec![],
                pitches: (1_u128 << bass) | (1_u128 << (bass + 7)),
                bass: f64::from(bass),
                velocity: 75.0,
            }
        }).collect();
        let trace = RhythmTrace {
            score: 0.0,
            phase: 0,
            onsets: vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 20, 22],
        };
        let internal = [vec![(0, 4), (2, 6)]];
        assert_eq!(
            corroborated_onsets(meter, &trace, &groups, &internal, &[]).unwrap(),
            [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20],
        );
        let crosses_adjustment = [vec![(0, 6), (3, 9)]];
        assert!(corroborated_onsets(meter, &trace, &groups, &crosses_adjustment, &[]).is_none());

        // If the played gap genuinely matches the longer span, agreement
        // among earlier chords is insufficient evidence to shorten it.
        groups[9].time += 0.4;
        groups[10].time += 0.4;
        assert!(!incoming_pulse_supports(&groups, &trace.onsets, 8, 2));
        assert_eq!(corroborated_onsets(meter, &trace, &groups, &[], &[]).unwrap(), trace.onsets);
    }
}

#[cfg(test)]
mod unconnected_chord_end_tests {
    use super::*;

    fn local(length: i64) -> Option<WrittenSuccessor> {
        Some(WrittenSuccessor {
            length,
            same_source: true,
        })
    }

    #[test]
    fn coincident_releases_need_an_unambiguous_successor_without_reattack() {
        let attack = |id, time: f64, end: f64, pitch| Attack {
            id,
            time,
            hold: end - time,
            pitch,
            velocity: 70,
            track: 1,
            channel: 0,
        };
        let notes = [
            attack(0, 0.0, 0.52, 42),
            attack(1, 0.02, 0.53, 53),
            attack(2, 0.04, 0.51, 60),
            attack(3, 0.02, 0.75, 67),
        ];
        let positions = [0; 4];
        let connected = [local(48), None, local(48), None];
        assert_eq!(
            unconnected_chord_ends(&notes, &positions, &connected, &[2.0; 4], 48),
            [None, Some(48), None, None]
        );
        assert_eq!(
            unconnected_chord_ends(
                &notes,
                &positions,
                &[local(48), None, local(72), None],
                &[2.0; 4],
                48
            ),
            [None; 4]
        );
        let remote = Some(WrittenSuccessor {
            length: 48,
            same_source: false,
        });
        assert_eq!(
            unconnected_chord_ends(
                &notes,
                &positions,
                &[remote, None, remote, None],
                &[2.0; 4],
                48
            ),
            [None; 4]
        );
        let mut repeated = notes.to_vec();
        repeated.push(attack(4, 0.6, 1.1, 53));
        assert_eq!(
            unconnected_chord_ends(
                &repeated,
                &[0, 0, 0, 0, 24],
                &[local(48), None, local(48), None, None],
                &[2.0; 5],
                48
            ),
            [None; 5]
        );
        let mut independent = notes;
        independent[1].channel = 1;
        assert_eq!(
            unconnected_chord_ends(&independent, &positions, &connected, &[2.0; 4], 48),
            [None; 4]
        );
    }

    #[test]
    fn a_predicted_chord_release_requires_matching_local_pulses_and_articulation() {
        let timeline = BTreeMap::from([
            (12, vec![0, 1]),
            (36, vec![2]),
            (84, vec![3, 4]),
            (108, vec![5]),
            (156, vec![6, 7]),
            (228, vec![8]),
        ]);
        let mut connected = [
            local(24),
            local(24),
            None,
            local(24),
            local(24),
            None,
            None,
            None,
            None,
        ];
        let mut measured = [0.8, 1.0, 1.4, 0.7, 0.9, 1.4, 0.83, 0.9, 1.5];
        assert_eq!(
            pulsed_release_before_rest(&timeline, 156, 72, &connected, &measured),
            Some(24)
        );
        // An independent distant occurrence has no vote in this phrase.
        let mut more = timeline.clone();
        more.extend([(420, vec![2, 5]), (444, vec![8])]);
        assert_eq!(
            pulsed_release_before_rest(&more, 156, 72, &connected, &measured),
            Some(24)
        );
        // The current keys are released distinctly earlier. Preserve the rest.
        measured[6] = 0.25;
        assert_eq!(
            pulsed_release_before_rest(&timeline, 156, 72, &connected, &measured),
            None
        );
        // Two durations below the rounding floor remain distinct evidence.
        let short_gates = [0.15, 0.35, 0.1, 0.25, 0.3, 0.1, 0.1, 0.2, 0.2];
        assert_eq!(
            pulsed_release_before_rest(&timeline, 156, 72, &connected, &short_gates),
            None
        );
        measured[6] = 0.83;
        connected[3] = local(48);
        assert_eq!(
            pulsed_release_before_rest(&timeline, 156, 72, &connected, &measured),
            None
        );
        connected[3] = local(24);
        let mut interrupted = timeline;
        interrupted.insert(168, vec![8]);
        assert_eq!(
            pulsed_release_before_rest(&interrupted, 156, 72, &connected, &measured),
            None
        );
    }
}

#[cfg(test)]
mod independent_hold_tests {
    use super::*;

    fn confirmed_holds(attacks: &[Attack], events: &[NotationEvent]) -> BTreeMap<usize, (i64, i64)> {
        sustained_intervals(attacks, events).into_iter()
            .filter(|(_, proposal)| proposal.confirmed)
            .map(|(id, proposal)| (id, (proposal.early_end, proposal.continuation)))
            .collect()
    }

    #[test]
    fn midpoint_uses_the_latest_plausible_time_in_a_spread_chord() {
        let latest = BTreeMap::from([(0, 0.0), (24, 0.28), (48, 0.54), (96, 1.02)]);
        assert!((latest_time_at(&latest, 36).unwrap() - 0.41).abs() < 1.0e-12);
        assert_eq!(latest_time_at(&latest, 48), Some(0.54));
        assert_eq!(latest_time_at(&latest, 120), None);
    }

    #[test]
    fn early_release_needs_the_same_written_pulse_on_both_sides() {
        let events: Vec<_> = (0..=12).map(|id| NotationEvent {
            source_ordinal: id, pos: id as i64 * UNIT,
            len: if id % 4 == 0 { 4 * UNIT } else { UNIT },
            kind: NotationEventKind::Regular,
        }).collect();
        let attacks: Vec<_> = events.iter().map(|event| {
            let id = event.source_ordinal;
            let in_bass = id % 4 == 0;
            Attack {
                id, time: id as f64 * 0.25,
                hold: if in_bass { 0.82 } else { 0.22 },
                pitch: if in_bass { 43 } else { 69 }, velocity: 64, track: 0, channel: 0,
            }
        }).collect();
        let result = confirmed_holds(&attacks, &events);
        assert!(!result.contains_key(&0));
        assert_eq!(result.get(&4), Some(&(5 * UNIT, 8 * UNIT)));
        assert!(!result.contains_key(&8));
    }

    #[test]
    fn short_articulation_requires_continuation_of_another_strand_inside_the_note() {
        let mut events: Vec<_> = (0..=12).map(|id| NotationEvent {
            source_ordinal: id, pos: id as i64 * UNIT,
            len: if id % 4 == 0 { 4 * UNIT } else { UNIT },
            kind: NotationEventKind::Regular,
        }).collect();
        let attacks: Vec<_> = events.iter().map(|event| {
            let id = event.source_ordinal;
            let in_bass = id % 4 == 0;
            Attack {
                id, time: id as f64 * 0.25,
                hold: if id == 4 { 0.52 } else if in_bass { 0.92 } else { 0.22 },
                pitch: if in_bass { 43 } else { 69 }, velocity: 64, track: 0, channel: 0,
            }
        }).collect();
        assert_eq!(confirmed_holds(&attacks, &events).get(&4), Some(&(5 * UNIT, 8 * UNIT)));
        events[6].kind = NotationEventKind::PlaybackOnly;
        events[7].kind = NotationEventKind::PlaybackOnly;
        assert!(!confirmed_holds(&attacks, &events).contains_key(&4));
    }

    #[test]
    fn a_held_line_outlasts_independent_attacks_but_a_short_key_does_not() {
        for (offset, transpose, scale) in [(0, 0, 1.0), (192, 7, 0.75), (384, 12, 1.5)] {
            let pitches = [43, 67, 70, 68, 45, 72, 69, 71, 47];
            let mut events = Vec::new();
            let mut attacks = Vec::new();
            for (id, pitch) in pitches.into_iter().enumerate() {
                events.push(NotationEvent {
                    source_ordinal: id, pos: offset + id as i64 * UNIT,
                    len: if id % 4 == 0 { 4 * UNIT } else { UNIT },
                    kind: NotationEventKind::Regular,
                });
                attacks.push(Attack {
                    id, time: 2.0 + id as f64 * 0.25 * scale,
                    hold: if id % 4 == 0 { 0.92 * scale } else { 0.22 * scale },
                    pitch: pitch + transpose, velocity: 64, track: 0, channel: 0,
                });
            }
            let result = confirmed_holds(&attacks, &events);
            assert_eq!(result.get(&0), Some(&(offset + UNIT, offset + 4 * UNIT)));
            assert_eq!(result.get(&4), Some(&(offset + 5 * UNIT, offset + 8 * UNIT)));

            attacks[0].hold = 0.23 * scale;
            let result = confirmed_holds(&attacks, &events);
            assert!(!result.contains_key(&0));
            assert_eq!(result.get(&4), Some(&(offset + 5 * UNIT, offset + 8 * UNIT)));

            attacks[0].hold = 0.92 * scale;
            attacks[3].pitch = attacks[0].pitch;
            assert_ne!(confirmed_holds(&attacks, &events).get(&0),
                       Some(&(offset + UNIT, offset + 4 * UNIT)));
        }
    }
}
