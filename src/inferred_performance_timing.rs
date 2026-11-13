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
                let duration = if opening && index == 0 { pickup } else { units * SUBDIVISION };
                (duration, NotationEventKind::Regular)
            };
            events.push(NotationEvent { source_ordinal: note.source_ordinal, pos, len, kind });
        }
    }
    events.sort_by_key(|event| event.source_ordinal);
    Ok(InferredTiming { pickup, events })
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
