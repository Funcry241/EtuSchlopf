//! Read-only evidence for deriving written time from a MIDI/WAV performance.
use super::{NotationEvent, NotationEventKind};
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
    pub(super) key: Option<MidiKeySignature>,
    pub(super) events: Vec<NotationEvent>,
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

type Repeat = Vec<(usize, usize)>;

fn repeated_phrases(groups: &[AttackGroup]) -> Vec<Repeat> {
    let width = groups.len() + 1;
    let mut scores = vec![0.0; width * width];
    let mut back = vec![0; width * width];
    let mut endings = vec![];
    for a in 1..width {
        for b in a + 8..width {
            let (x, y) = (groups[a - 1].pitches, groups[b - 1].pitches);
            let similarity = f64::from((x & y).count_ones()) / f64::from((x | y).count_ones());
            let mut best = (0.0, 0);
            for (direction, value) in [
                (1, scores[(a - 1) * width + b - 1] + 4.0 * similarity - 2.0),
                (2, scores[(a - 1) * width + b] - 1.6),
                (3, scores[a * width + b - 1] - 1.6),
            ] {
                if value > best.0 {
                    best = (value, direction);
                }
            }
            scores[a * width + b] = best.0;
            back[a * width + b] = best.1;
            if best.0 >= 20.0 {
                endings.push((best.0, a, b));
            }
        }
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

struct RhythmEvidence<'a> {
    groups: &'a [AttackGroup],
    gaps: Vec<f64>,
    nominal: f64,
    accent: Vec<f64>,
    tempos: [f64; TEMPO_COUNT],
    transition: [[f64; TEMPO_COUNT]; TEMPO_COUNT],
    prior: [f64; TEMPO_COUNT],
}

impl<'a> RhythmEvidence<'a> {
    fn from_observations(groups: &'a [AttackGroup], repetitions: &[Repeat]) -> Self {
        let original: Vec<_> = groups.windows(2).map(|g| g[1].time - g[0].time).collect();
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
        let transition = std::array::from_fn(|a| {
            std::array::from_fn(|b| 100.0 * (tempos[a] / tempos[b]).ln().powi(2))
        });
        let prior = std::array::from_fn(|i| 9.0 * (tempos[i] / nominal).ln().powi(2));
        Self {
            groups,
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
        let max_jump = 16.min(cycle * 2);
        for (index, &gap) in self.gaps.iter().enumerate() {
            for phase in 0..cycle {
                let row = phase * TEMPO_COUNT;
                for tempo in 0..TEMPO_COUNT {
                    let mut best = (f64::INFINITY, 0);
                    for previous in 0..TEMPO_COUNT {
                        let candidate = costs[row + previous] + self.transition[previous][tempo];
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
            for state in 0..states {
                let (phase, tempo) = (state / TEMPO_COUNT, state % TEMPO_COUNT);
                if anchors.get(&(index + 1)).is_some_and(|&p| p != phase) {
                    costs[state] = f64::INFINITY;
                    continue;
                }
                let mut best = (f64::INFINITY, 0, 0);
                for jump in usize::from(!allow_zero)..=max_jump {
                    let previous = (phase + 2 * cycle - jump) % cycle;
                    let from = previous * TEMPO_COUNT + tempo;
                    let complexity = match jump {
                        0 => 0.26,
                        3 => 0.24,
                        1 | 2 | 4 | 6 | 8 | 12 | 16 => 0.0,
                        _ => 1.3,
                    } + jump.saturating_sub(4) as f64 * 0.07;
                    let scale = if jump >= 6 { 0.7 } else { 1.0 };
                    let residual = gap / self.tempos[tempo] - jump as f64 * factors[previous];
                    let candidate = smoothed[from]
                        + scale
                            * (2.5 * residual * residual + self.prior[tempo] + 3.0 * complexity);
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
        let mut anchors: BTreeMap<_, _> = repetitions
            .iter()
            .flat_map(|repeat| {
                let (a, b) = repeat[0];
                [(a, 0), (b, 0)]
            })
            .collect();
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
        for repeat in repetitions {
            let &(a, b) = repeat.last().expect("repetition is nonempty");
            anchors.insert(
                b,
                (refined.phase + refined.onsets[a] as usize) % meter.cycle,
            );
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

fn written_lengths(
    regular: &[Attack],
    positions: &[i64],
    nominal: f64,
    groups: &[AttackGroup],
    repetitions: &[Repeat],
    trace: &RhythmTrace,
    pulse: i64,
) -> Vec<i64> {
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
    let mut expected: Vec<_> = regular
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
            1.2 * ((bound - positions[attack.id] as f64) / UNIT as f64).max(0.6)
        })
        .collect();
    let indices: BTreeMap<_, _> = regular
        .iter()
        .enumerate()
        .map(|(index, a)| (a.id, index))
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
    let costs: Vec<Vec<_>> = regular
        .iter()
        .zip(&expected)
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
    let independent_lengths: Vec<_> = assign_continuations(&costs)
        .into_iter()
        .enumerate()
        .map(|(index, column)| {
            let source = regular[index];
            if let Some(next) = regular.get(column) {
                let gap = positions[next.id] - positions[source.id];
                if VALUES.contains(&gap) {
                    return gap;
                }
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
        &independent_lengths,
    );
    independent_lengths
        .into_iter()
        .zip(shared)
        .map(|(length, shared)| shared.unwrap_or(length))
        .collect()
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
    let scores: Vec<_> = (-200..=200)
        .map(|step| {
            let offset = f64::from(step) * 0.005;
            let value = groups
                .iter()
                .map(|group| envelope(group.time + offset))
                .sum::<f64>()
                / groups.len() as f64;
            (value, offset)
        })
        .collect();
    let &(peak, offset) = scores
        .iter()
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .expect("offset range");
    let floor = median(scores.iter().map(|s| s.0).collect());
    ensure!(
        floor > 0.0 && peak >= 1.5 * floor,
        "WAV bestaetigt die MIDI-Einsaetze nicht eindeutig"
    );
    let last_time = groups.last().expect("nonempty groups").time;
    ensure!(
        groups[0].time + offset >= -time_per_frame
            && last_time + offset <= samples.len() as f64 / f64::from(rate),
        "WAV deckt die MIDI-Einsaetze zeitlich nicht ab"
    );
    for part in groups.chunks(groups.len().div_ceil(3)) {
        let local_support = |shift: f64| {
            part.iter()
                .map(|group| envelope(group.time + shift))
                .sum::<f64>()
                / part.len() as f64
        };
        let unrelated = median(
            scores
                .iter()
                .map(|&(_, shift)| local_support(shift))
                .collect(),
        );
        ensure!(
            unrelated > 0.0 && local_support(offset) >= 1.25 * unrelated,
            "WAV bestaetigt die Zeitzuordnung nicht ueber den Spielverlauf"
        );
    }
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
    let timing = RhythmEvidence::from_observations(&groups, &repetitions);
    let releases = release_alignments(&groups);
    let mut ranked: Vec<_> = METERS
        .into_iter()
        .map(|meter| (meter, timing.read_meter(meter, &repetitions, &releases)))
        .collect();
    ranked.sort_by(|a, b| a.1.score.total_cmp(&b.1.score));
    let (meter, trace) = &ranked[0];
    ensure!(
        trace.score.is_finite() && ranked[1].1.score - trace.score >= 1.0,
        "MIDI-Einsaetze stuetzen keine eindeutige Taktart"
    );
    ensure!(
        source.time_signatures.is_empty()
            || source.time_signatures == [(meter.beats, meter.beat_type)],
        "MIDI-Taktart und abgeleitetes Performanceraster widersprechen sich"
    );
    ensure!(
        trace.phase == 0,
        "Auftakt im freien Performanceraster ist mehrdeutig"
    );
    let (positions, grace) = project_attacks(&groups, trace, timing.nominal, attacks.len());
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
    let lengths = written_lengths(
        &regular,
        &positions,
        timing.nominal,
        &groups,
        &repetitions,
        trace,
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
        key,
        events,
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
