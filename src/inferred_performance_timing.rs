//! Evidence for a steady subdivision under local tempo variation.
use super::{NotationEvent, NotationEventKind};
use crate::source_event::CanonicalSourceEvent;
use anyhow::{Result, ensure};
use std::collections::BTreeSet;

const SUBDIVISION: i64 = 24;

pub(super) struct InferredTiming {
    pub(super) pickup: i64,
    pub(super) events: Vec<NotationEvent>,
}

fn median(values: &mut [i64]) -> i64 {
    values.sort_unstable();
    values[values.len() / 2]
}

fn ratio(value: i64, pulse: i64) -> f64 {
    value as f64 / pulse as f64
}

pub(super) fn infer(notes: &[CanonicalSourceEvent]) -> Result<InferredTiming> {
    let mut order = notes.iter().collect::<Vec<_>>();
    order.sort_by_key(|note| (note.pos, note.source_ordinal));
    let mut gaps = order.windows(2).map(|pair| pair[1].pos - pair[0].pos)
        .filter(|&gap| gap > 0).collect::<Vec<_>>();
    ensure!(gaps.len() >= 32, "Zu wenige Einsätze für ein belastbares Performanceraster");
    gaps.sort_unstable();
    // The upper third excludes most rolled/splayed chord intervals.
    let upper_third = gaps.len() * 7 / 10;
    let pulse = median(&mut gaps[upper_third..]);

    let mut clusters: Vec<Vec<&CanonicalSourceEvent>> = Vec::new();
    for note in order {
        if let Some(last) = clusters.last_mut()
            && ratio(note.pos - last[0].pos, pulse) <= 0.25
        {
            last.push(note);
        } else {
            clusters.push(vec![note]);
        }
    }
    let mut groups: Vec<Vec<&CanonicalSourceEvent>> = Vec::new();
    let mut last_cluster_start = None;
    for cluster in clusters {
        let start = cluster[0].pos;
        if let Some(previous) = last_cluster_start
            && ratio(start - previous, pulse) < 0.52
        {
            groups.last_mut().expect("prior cluster creates a group").extend(cluster);
        } else {
            groups.push(cluster);
        }
        last_cluster_start = Some(start);
    }
    ensure!(groups.len() >= 32, "Zu wenige getrennte Einsatzgruppen");
    let gaps = groups.windows(2).map(|pair| pair[1][0].pos - pair[0][0].pos).collect::<Vec<_>>();
    let stable = gaps.iter().filter(|&&gap| (0.68..=1.62).contains(&ratio(gap, pulse))).count();
    ensure!(stable * 100 >= gaps.len() * 94, "Kein durchgehendes binäres Performanceraster");

    // A repeating bass accent supplies the quarter phase. A quarter pulse
    // alone does not prove a bar period: this narrow model writes 2/4.
    let mut bass = [(0_i64, 0_i64); 4];
    for (index, group) in groups.iter().enumerate().skip(1) {
        let lowest = group.iter().map(|note| note.midi).min().expect("nonempty group");
        bass[index % 4].0 += i64::from(lowest);
        bass[index % 4].1 += 1;
    }
    let mut phases = [0, 1, 2, 3];
    phases.sort_by_key(|&phase| bass[phase].0 * 100 / bass[phase].1);
    let pitch_mean = |phase: usize| bass[phase].0 as f64 / bass[phase].1 as f64;
    ensure!(pitch_mean(phases[1]) - pitch_mean(phases[0]) >= 4.0,
        "Bassakzente stützen keine eindeutige Viertelphase");

    let opening = groups[0].len() == 1 && groups[1].len() > 1 && phases[0] == 1
        && ratio(gaps[0], pulse) >= 1.35;
    ensure!(opening || phases[0] == 0, "Beginn des Performancerasters ist mehrdeutig");
    let pickup = if opening { 2 * SUBDIVISION } else { 0 };
    let mut events = Vec::with_capacity(notes.len());
    let mut attacks = BTreeSet::new();
    for (index, group) in groups.iter().enumerate() {
        let pos = index as i64 * SUBDIVISION + if index > 0 { pickup / 2 } else { 0 };
        let mut neighbours = gaps[index.saturating_sub(4)..(index + 4).min(gaps.len())]
            .iter().copied().filter(|gap| (0.72..=1.55).contains(&ratio(*gap, pulse)))
            .collect::<Vec<_>>();
        let local = if neighbours.is_empty() { pulse } else { median(&mut neighbours) };
        let hosts = grace_hosts(group, pulse);
        for (ordinal, note) in group.iter().enumerate() {
            let (len, kind) = if let Some(host) = hosts[ordinal] {
                (0, NotationEventKind::Grace { host })
            } else {
                ensure!(attacks.insert((pos, note.midi)),
                    "Gleichnamige Anschläge fallen im Performanceraster zusammen");
                let units = (ratio(note.len, local) * 1.10).round().clamp(1.0, 32.0) as i64;
                let duration = if opening && index == 0 {
                    pickup
                } else if units == 3 && (
                    index % 4 == phases[0] && is_quarter_release_candidate(&groups[index..], note)
                    || index % 4 == (phases[0] + 2) % 4
                        && is_offbeat_melody_quarter_candidate(&groups[index..], note, pulse)
                )
                {
                    4 * SUBDIVISION
                } else if (2..=3).contains(&units) && index > 0
                    && is_overlapping_bass_figure_candidate(&groups[index - 1..], note)
                {
                    SUBDIVISION
                } else if units == 2 && index >= 4 && index % 4 == phases[0]
                    && is_continuing_bass_quarter_candidate(&groups[index - 4..], note)
                {
                    4 * SUBDIVISION
                } else if units == 2 && index >= 2
                    && is_repeated_chord_release_candidate(&groups[index - 2..], note)
                {
                    SUBDIVISION
                } else {
                    units * SUBDIVISION
                };
                (duration, NotationEventKind::Regular)
            };
            events.push(NotationEvent { source_ordinal: note.source_ordinal, pos, len, kind });
        }
    }
    events.sort_by_key(|event| event.source_ordinal);
    Ok(InferredTiming { pickup, events })
}

// A metrically aligned outer line can outlast the played release. Require
// the accompaniment to continue through the last subdivision and the outer
// line to return at the next quarter boundary.
fn is_quarter_release_candidate(
    groups: &[Vec<&CanonicalSourceEvent>],
    note: &CanonicalSourceEvent,
) -> bool {
    let [opening, first, second, third, boundary, ..] = groups else { return false; };
    let release = note.pos + note.len;
    if release <= third[0].pos || release >= boundary[0].pos {
        return false;
    }

    let away_from_line = |other: &&CanonicalSourceEvent, direction| {
        direction * (note.midi - other.midi) >= 3
    };
    [-1, 1].into_iter().any(|direction| {
        opening.iter().any(|other| away_from_line(other, direction))
            && opening.iter().all(|other| direction * (note.midi - other.midi) >= 0)
            && [first, second, third].into_iter().flatten().all(|other| away_from_line(other, direction))
            && first.iter().any(|attack| third.iter().any(|repeat| attack.midi == repeat.midi))
            && boundary.iter().any(|next| {
                note.midi.abs_diff(next.midi) <= 7 && !away_from_line(next, direction)
            })
    })
}

// A melody starting on the offbeat can continue across the beat
// boundary while the accompaniment alternates underneath it.
fn is_offbeat_melody_quarter_candidate(
    groups: &[Vec<&CanonicalSourceEvent>],
    note: &CanonicalSourceEvent,
    pulse: i64,
) -> bool {
    let [opening, first, second, third, boundary, ..] = groups else { return false; };
    let release = note.pos + note.len;
    if release <= third[0].pos || release >= boundary[0].pos {
        return false;
    }

    let below = |attack: &&CanonicalSourceEvent| attack.midi <= note.midi - 2;
    if !opening.iter().any(below)
        || !opening.iter().chain([first, second, third].into_iter().flatten())
            .all(|attack| attack.source_ordinal == note.source_ordinal || below(attack))
        || !first.iter().all(|attack| third.iter().any(|repeat| repeat.midi == attack.midi))
        || !third.iter().all(|attack| first.iter().any(|repeat| repeat.midi == attack.midi))
        || !second.iter().all(|attack| first.iter().all(|other| other.midi != attack.midi))
    {
        return false;
    }

    boundary.iter().zip(grace_hosts(boundary, pulse))
        .filter(|(_, host)| host.is_none())
        .map(|(attack, _)| attack.midi)
        .max()
        .is_some_and(|pitch| note.midi.abs_diff(pitch) <= 2)
}

// Successive beats can carry the same bass-and-accompaniment texture
// even when the performer releases one bass note early.
fn is_continuing_bass_quarter_candidate(
    groups: &[Vec<&CanonicalSourceEvent>],
    note: &CanonicalSourceEvent,
) -> bool {
    let [prior, _, _, prior_last, current, first, second, third, next, ..] = groups else {
        return false;
    };
    let release = note.pos + note.len;
    if release <= second[0].pos || release >= third[0].pos {
        return false;
    }

    let separated_bass = |group: &[&CanonicalSourceEvent], bass: &CanonicalSourceEvent| {
        group.len() > 1 && group.iter().all(|other| {
            other.source_ordinal == bass.source_ordinal || other.midi - bass.midi >= 3
        })
    };
    if !separated_bass(current, note) {
        return false;
    }
    let previous_bass = prior.iter().min_by_key(|attack| attack.midi).expect("nonempty group");
    separated_bass(prior, previous_bass)
        && previous_bass.pos + previous_bass.len > prior_last[0].pos
        && previous_bass.pos + previous_bass.len < current[0].pos
        && note.midi.abs_diff(previous_bass.midi) <= 7
        && [first, second, third].into_iter().flatten().all(|attack| attack.midi - note.midi >= 3)
        && first.iter().any(|attack| third.iter().any(|repeat| attack.midi == repeat.midi))
        && next.iter().any(|attack| attack.midi <= note.midi && attack.midi.abs_diff(note.midi) <= 7)
}

// A repeated upper member can articulate a moving bass figure. The
// independent note release must not turn that moving bass into a held voice.
fn is_overlapping_bass_figure_candidate(
    groups: &[Vec<&CanonicalSourceEvent>],
    note: &CanonicalSourceEvent,
) -> bool {
    let [before, current, next, after, ..] = groups else { return false; };
    let lowest = |group: &[&CanonicalSourceEvent]| {
        group.iter().map(|attack| attack.midi).min().expect("nonempty group")
    };
    let intervening = lowest(next);
    let returning = lowest(after);
    if note.midi != lowest(current) || intervening != lowest(before)
        || note.midi >= intervening || returning >= intervening
        || note.pos + note.len <= next[0].pos
    {
        return false;
    }

    let repeats_above_bass = |first: &[&CanonicalSourceEvent], second: &[&CanonicalSourceEvent]| {
        let lower_bound = lowest(first).max(lowest(second));
        first.iter().filter(|attack| attack.midi > lower_bound)
            .any(|attack| second.iter().any(|other| other.midi == attack.midi))
    };
    repeats_above_bass(current, after)
        && (returning != note.midi || after.iter().any(|attack| attack.midi == note.midi
            && attack.pos >= note.pos + note.len))
}

// A repeated chord can change one member at the next subdivision while
// the previous key is still held. Require the chord to be established
// before the overlap and the same member to return immediately afterward.
fn is_repeated_chord_release_candidate(
    groups: &[Vec<&CanonicalSourceEvent>],
    note: &CanonicalSourceEvent,
) -> bool {
    let [earlier, prior, current, next, returning, ..] = groups else { return false; };
    let release = note.pos + note.len;
    if release <= next[0].pos || release >= returning[0].pos {
        return false;
    }

    let has_pitch = |group: &[&CanonicalSourceEvent], midi| {
        group.iter().any(|attack| attack.midi == midi)
    };
    if has_pitch(next, note.midi) {
        return false;
    }
    let repeats = |midi| [earlier, prior, returning].into_iter().all(|group| has_pitch(group, midi));
    repeats(note.midi)
        && current.iter().filter(|attack| repeats(attack.midi) && has_pitch(next, attack.midi))
            .count() >= 2
}

fn grace_hosts(group: &[&CanonicalSourceEvent], pulse: i64) -> Vec<Option<usize>> {
    let mut hosts = vec![None; group.len()];
    for (index, note) in group.iter().enumerate().rev() {
        if ratio(note.len, pulse) >= 0.80 {
            continue;
        }
        if let Some((host, candidate)) = group.iter().enumerate().skip(index + 1).find(|(_, other)| {
            let delay = ratio(other.pos - note.pos, pulse);
            (0.10..=0.56).contains(&delay)
                && (1..=2).contains(&other.midi.abs_diff(note.midi))
                && other.pos + other.len > note.pos + note.len
        }) {
            hosts[index] = Some(hosts[host].unwrap_or(candidate.source_ordinal));
        }
    }
    hosts
}
