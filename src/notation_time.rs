//! Explicit source-to-notation projection. Exact source events are never mutated.
use crate::midi::DecodedMidiSource;
use crate::midi::source_time::{MetricMeasure, MetricTimeline, SourceTimeDomain};
use anyhow::{Result, anyhow, bail, ensure};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NotationEventKind {
    Regular,
    Grace { host: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NotationEvent {
    pub(crate) source_ordinal: usize,
    pub(crate) pos: i64,
    pub(crate) len: i64,
    pub(crate) kind: NotationEventKind,
}

#[derive(Debug, Clone)]
pub(crate) struct NotationTimePlan {
    pub(crate) source_domain: SourceTimeDomain,
    pub(crate) metric: MetricTimeline,
    pub(crate) events: Vec<NotationEvent>,
    source_events: Vec<crate::source_event::CanonicalSourceEvent>,
}

fn grid(value: i64, divisions: i64) -> Option<i64> {
    let n = i128::from(value) * 96;
    (n % i128::from(divisions) == 0)
        .then(|| i64::try_from(n / i128::from(divisions)).ok())
        .flatten()
}

// Values closed under the existing serializer's six-unit rhythmic grid.
// A locally named dotted 64th (9) can leave an unrepresentable three-unit
// fragment at a later metric split. Refuse that class before output preparation.
fn plain_duration(value: i64) -> bool {
    value >= 6 && value % 6 == 0
}

fn plain_onset(metric: &MetricTimeline, pos: i64) -> bool {
    metric
        .ordinal_at(pos)
        .and_then(|i| metric.measure(i))
        .is_some_and(|bar| pos == bar.start || plain_duration(pos - bar.start))
}

impl NotationTimePlan {
    pub(crate) fn from_source(source: &DecodedMidiSource) -> Result<Self> {
        let d = source.time_domain.divisions;
        let extent = source
            .metric_timeline
            .measure(source.metric_timeline.len() - 1)
            .ok_or_else(|| anyhow!("Leerer Taktverlauf"))?
            .end;
        let mut changes = Vec::new();
        let mut prior = None;
        for segment in source.metric_timeline.segments() {
            ensure!(
                !segment.truncated_last,
                "Taktartwechsel mitten im Takt wird noch nicht unterstuetzt (Takt {})",
                segment.first_ordinal + segment.count
            );
            ensure!(
                segment.beat_type == 4 && matches!(segment.beats, 2..=4),
                "Taktart {}/{} wird noch nicht unterstuetzt",
                segment.beats,
                segment.beat_type
            );
            let meter = (segment.beats, segment.beat_type);
            if prior != Some(meter) {
                changes.push(crate::midi::source_time::MeterChange {
                    tick: u64::try_from(
                        grid(segment.start, d)
                            .ok_or_else(|| anyhow!("Taktgrenze nicht notierbar"))?,
                    )?,
                    beats: segment.beats,
                    beat_type: segment.beat_type,
                });
                prior = Some(meter);
            }
        }
        let metric = MetricTimeline::from_midi(
            SourceTimeDomain {
                source_ppq: 96,
                divisions: 96,
            },
            &changes,
            grid(extent, d).ok_or_else(|| anyhow!("Taktende nicht notierbar"))?,
        )?;
        let n = source.notes.len();
        let mut originals = vec![None; n];
        let mut lanes = BTreeMap::new();
        for entry in &source.source_event_causal_ledger.entries {
            lanes.insert(
                entry.source_ordinal,
                (entry.note_on.track_index, entry.note_on.channel),
            );
        }
        for event in &source.notes {
            ensure!(
                event.source_ordinal < n && originals[event.source_ordinal].is_none(),
                "Quellordinalitaet ist nicht eindeutig"
            );
            originals[event.source_ordinal] = Some(*event);
        }
        let originals = originals
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("Quellordinalitaet ist nicht vollstaendig"))?;
        let mut events = vec![None; n];
        let mut unsupported = Vec::new();
        for event in &originals {
            if let (Some(pos), Some(len)) = (grid(event.pos, d), grid(event.len, d))
                && plain_duration(len)
                && plain_onset(&metric, pos)
            {
                events[event.source_ordinal] = Some(NotationEvent {
                    source_ordinal: event.source_ordinal,
                    pos,
                    len,
                    kind: NotationEventKind::Regular,
                });
                continue;
            }
            unsupported.push(event.source_ordinal);
        }
        let mut proposals = Vec::new();
        for ordinal in &unsupported {
            if events[*ordinal].is_some() {
                continue;
            }
            let v = originals[*ordinal];
            let lane = lanes
                .get(ordinal)
                .ok_or_else(|| anyhow!("Quellspur fehlt"))?;
            let vend = v
                .pos
                .checked_add(v.len)
                .ok_or_else(|| anyhow!("Quellnotenende ueberlaeuft"))?;
            let preds = originals
                .iter()
                .filter(|e| {
                    e.source_ordinal != *ordinal
                        && lanes.get(&e.source_ordinal) == Some(lane)
                        && e.pos.checked_add(e.len) == Some(v.pos)
                })
                .collect::<Vec<_>>();
            let succs = originals
                .iter()
                .filter(|e| {
                    e.source_ordinal != *ordinal
                        && lanes.get(&e.source_ordinal) == Some(lane)
                        && e.pos >= vend
                        && i128::from(e.pos - vend) * 192 < i128::from(d)
                })
                .collect::<Vec<_>>();
            if i128::from(v.len) * 4 >= i128::from(d) || preds.len() != 1 || succs.len() != 1 {
                continue;
            }
            let predecessor = preds[0];
            let host = succs[0];
            if events[predecessor.source_ordinal]
                .is_some_and(|e| e.kind != NotationEventKind::Regular)
            {
                continue;
            }
            if unsupported.iter().any(|i| {
                *i != *ordinal
                    && *i != predecessor.source_ordinal
                    && lanes.get(i) == Some(lane)
                    && (originals[*i].pos == vend
                        || originals[*i].pos.checked_add(originals[*i].len) == Some(v.pos))
            }) {
                continue;
            }
            if originals.iter().any(|e| {
                e.source_ordinal != *ordinal
                    && e.source_ordinal != predecessor.source_ordinal
                    && lanes.get(&e.source_ordinal) == Some(lane)
                    && e.pos.checked_add(e.len) == Some(host.pos)
            }) {
                continue;
            }
            if originals.iter().any(|e| {
                e.source_ordinal != *ordinal
                    && e.source_ordinal != predecessor.source_ordinal
                    && e.source_ordinal != host.source_ordinal
                    && lanes.get(&e.source_ordinal) == Some(lane)
                    && e.pos < host.pos
                    && e.pos
                        .checked_add(e.len)
                        .is_some_and(|end| end > predecessor.pos)
            }) {
                continue;
            }
            let (Some(start), Some(end)) = (grid(predecessor.pos, d), grid(host.pos, d)) else {
                continue;
            };
            if !plain_duration(end - start) || !plain_onset(&metric, start) {
                continue;
            }
            proposals.push((
                *ordinal,
                predecessor.source_ordinal,
                host.source_ordinal,
                start,
                end,
            ));
        }
        // Independent hypotheses are all retained. Shared source/host participants
        // form an explicit conflict group; no first or globally ranked winner.
        let mut participation = BTreeMap::<usize, usize>::new();
        for &(v, predecessor, host, _, _) in &proposals {
            for id in [v, predecessor, host] {
                *participation.entry(id).or_default() += 1;
            }
        }
        for (v, predecessor, host, start, end) in proposals {
            if [v, predecessor, host]
                .iter()
                .any(|id| participation[id] != 1)
            {
                continue;
            }
            events[predecessor] = Some(NotationEvent {
                source_ordinal: predecessor,
                pos: start,
                len: end - start,
                kind: NotationEventKind::Regular,
            });
            events[v] = Some(NotationEvent {
                source_ordinal: v,
                pos: end,
                len: 0,
                kind: NotationEventKind::Grace { host },
            });
        }
        if let Some(i) = events.iter().position(Option::is_none) {
            let e = originals[i];
            let bar = source
                .metric_timeline
                .ordinal_at(e.pos)
                .map_or(0, |b| b + 1);
            bail!(
                "Notationszeit nicht eindeutig darstellbar: Quellnote {}, Takt {}. N-tolen, mehrdeutige und umgekehrt kodierte Vorschlaege werden noch nicht unterstuetzt",
                i,
                bar
            );
        }
        Ok(Self {
            source_domain: source.time_domain,
            metric,
            events: events.into_iter().flatten().collect(),
            source_events: originals,
        })
    }

    // Preserve historical genesis identities for the identity projection. New
    // temporal meaning binds the exact domain, metric and every source relation.
    pub(crate) fn bind_productive_plan(&self, source_plan_sha256: &str) -> String {
        let identity = self.source_domain.divisions == 96
            && self
                .metric
                .segments()
                .all(|s| s.beats == 4 && s.beat_type == 4)
            && self
                .events
                .iter()
                .zip(&self.source_events)
                .all(|(e, source)| {
                    e.kind == NotationEventKind::Regular
                        && e.pos == source.pos
                        && e.len == source.len
                });
        if identity {
            return source_plan_sha256.to_owned();
        }
        let mut bytes = b"ETUSCHLOPF_SOURCE_NOTATION_PLAN_V1\0".to_vec();
        bytes.extend_from_slice(source_plan_sha256.as_bytes());
        bytes.extend_from_slice(&self.source_domain.divisions.to_le_bytes());
        bytes.extend_from_slice(&(self.metric.segments().count() as u64).to_le_bytes());
        for s in self.metric.segments() {
            bytes.extend_from_slice(&s.start.to_le_bytes());
            bytes.extend_from_slice(&s.end.to_le_bytes());
            bytes.extend_from_slice(&s.beats.to_le_bytes());
            bytes.extend_from_slice(&s.beat_type.to_le_bytes());
        }
        bytes.extend_from_slice(&(self.events.len() as u64).to_le_bytes());
        for e in &self.events {
            bytes.extend_from_slice(&(e.source_ordinal as u64).to_le_bytes());
            bytes.extend_from_slice(&e.pos.to_le_bytes());
            bytes.extend_from_slice(&e.len.to_le_bytes());
            match e.kind {
                NotationEventKind::Regular => bytes.push(0),
                NotationEventKind::Grace { host } => {
                    bytes.push(1);
                    bytes.extend_from_slice(&(host as u64).to_le_bytes());
                }
            }
        }
        crate::sha256::hex(&bytes)
    }

    pub(crate) fn source_event(
        &self,
        ordinal: usize,
    ) -> Result<crate::source_event::CanonicalSourceEvent> {
        self.source_events
            .get(ordinal)
            .copied()
            .filter(|e| e.source_ordinal == ordinal)
            .ok_or_else(|| anyhow!("Exact source timing lost source ordinal {ordinal}"))
    }

    pub(crate) fn event(&self, ordinal: usize) -> Result<NotationEvent> {
        self.events
            .get(ordinal)
            .copied()
            .filter(|e| e.source_ordinal == ordinal)
            .ok_or_else(|| anyhow!("Notationszeit: unbekannte Quellordinalitaet {ordinal}"))
    }

    pub(crate) fn measure(&self, ordinal: usize) -> Result<MetricMeasure> {
        self.metric
            .measure(ordinal)
            .ok_or_else(|| anyhow!("Notationszeit: unbekannter Takt"))
    }
}

#[cfg(test)]
#[path = "../tests/unit/notation_time.rs"]
mod tests;
