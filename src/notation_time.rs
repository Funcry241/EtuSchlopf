//! Explicit source-to-notation projection. Exact source events are never mutated.
#[path = "inferred_performance_timing.rs"]
mod inferred_performance_timing;
#[path = "free_performance_timing.rs"]
mod free_performance_timing;

use crate::midi::{DecodedMidiSource, MidiKeySignature, decode_standard_midi_bytes};
use crate::midi::source_time::{MetricMeasure, MetricTimeline, SourceTimeDomain, supported_notation_meter};
use anyhow::{Context, Result, anyhow, bail, ensure};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NotationEventKind {
    Regular,
    Grace { host: usize },
    PlaybackOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NotationEvent {
    pub(crate) source_ordinal: usize,
    pub(crate) pos: i64,
    pub(crate) len: i64,
    pub(crate) kind: NotationEventKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WrittenHoldProposal {
    pub(crate) early_end: i64,
    pub(crate) continuation: i64,
    pub(crate) confirmed: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct NotationTimePlan {
    pub(crate) source_domain: SourceTimeDomain,
    pub(crate) metric: MetricTimeline,
    pub(crate) events: Vec<NotationEvent>,
    source_events: Vec<crate::source_event::CanonicalSourceEvent>,
    timing_authority: TimingAuthority,
    open_durations: BTreeSet<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimingAuthority {
    ExactMidi,
    PerformanceAnchors,
    PerformanceNotes,
    InferredPerformance,
    AudioAlignedPerformance,
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

fn validate_performance_meter(beats: u8, beat_type: u16, pickup: i64) -> Result<()> {
    ensure!(supported_notation_meter(beats, beat_type),
        "Taktart {beats}/{beat_type} wird noch nicht unterstuetzt");
    let measure_duration = i64::from(beats) * 384 / i64::from(beat_type);
    ensure!((0..measure_duration).contains(&pickup) && pickup % 6 == 0,
        "Notationszeitkarte: ungueltige Auftaktlaenge");
    Ok(())
}

pub(crate) fn read_midi_with_notation(path: &std::path::Path) -> Result<DecodedMidiSource> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("MIDI-Datei nicht lesbar: {}", path.display()))?;
    let mut source = decode_standard_midi_bytes(&bytes)?;
    let companion = path.with_extension("notation-time");
    let contents = match std::fs::read_to_string(&companion) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(source),
        Err(error) => return Err(error).with_context(|| format!("Notationszeitkarte: {}", companion.display())),
    };
    attach_notation_companion(&mut source, &bytes, &contents)
        .with_context(|| format!("Notationszeitkarte: {}", companion.display()))?;
    Ok(source)
}

fn attach_notation_companion(source: &mut DecodedMidiSource, midi: &[u8], contents: &str) -> Result<()> {
    ensure!(source.timing_markers.is_empty(),
        "Eingebettete und separate Notationszeitkarte konkurrieren");
    let mut lines = contents.lines();
    ensure!(lines.next() == Some("ETUSCHLOPF-NOTATION-TIME-V1"), "Unbekanntes Dateiformat");
    let checksum = crate::sha256::hex(midi);
    ensure!(lines.next().and_then(|line| line.strip_prefix("midi-sha256 ")) == Some(checksum.as_str()),
        "SHA-256 der MIDI stimmt nicht mit der Karte ueberein");
    let key = lines.next().ok_or_else(|| anyhow!("Tonartzeile fehlt"))?;
    match key.split_ascii_whitespace().collect::<Vec<_>>().as_slice() {
        ["key", "midi"] => {}
        ["key", fifths, mode @ ("major" | "minor")] => {
            let fifths = fifths.parse::<i8>()?;
            ensure!((-7..=7).contains(&fifths), "Tonartvorzeichen ausserhalb von -7 bis 7");
            let key = MidiKeySignature { fifths, minor: *mode == "minor" };
            ensure!(source.opening_key_signature.is_none_or(|existing| existing == key),
                "MIDI-Tonart und Notationszeitkarte widersprechen sich");
            source.opening_key_signature = Some(key);
        }
        _ => bail!("Ungueltige Tonartzeile"),
    }
    source.timing_markers = lines.map(|line| {
        let (tick, entry) = line.split_once('\t').ok_or_else(|| anyhow!("Tick und Eintrag fehlen"))?;
        ensure!(entry.starts_with("v2 "), "Separate Karten verwenden notenbezogene v2-Eintraege");
        Ok((tick.parse()?, entry.to_owned()))
    }).collect::<Result<_>>()?;
    ensure!(!source.timing_markers.is_empty(), "Leere Notationszeitkarte");
    Ok(())
}

struct PerformanceTiming {
    beats: u8,
    pickup: i64,
    anchors: Vec<(u64, i64)>,
}

impl PerformanceTiming {
    fn read(markers: &[(u64, String)]) -> Result<Self> {
        let mut ordered: Vec<_> = markers.iter().collect();
        ordered.sort_unstable_by_key(|marker| marker.0);
        let (first_tick, header) = ordered[0];
        let fields: Vec<_> = header.split_ascii_whitespace().collect();
        let (beats, pickup) = match fields.as_slice() {
            ["v1", "meter", beats] => (beats.parse::<u8>()?, 0),
            ["v1", "meter", beats, "pickup", length] => (beats.parse::<u8>()?, length.parse::<i64>()?),
            _ => bail!("Ungueltiger Kopf der MIDI-Notationszeitkarte"),
        };
        validate_performance_meter(beats, 4, pickup)?;
        let mut anchors = vec![(*first_tick, 0)];
        for (tick, marker) in ordered.into_iter().skip(1) {
            let position = marker.strip_prefix("v1 anchor ")
                .ok_or_else(|| anyhow!("Ungueltiger MIDI-Notationszeitanker"))?
                .parse::<i64>()?;
            let (prior_tick, prior_position) = anchors[anchors.len() - 1];
            ensure!(
                *tick > prior_tick && position > prior_position
                    && position - prior_position <= 96 && position % 6 == 0,
                "MIDI-Notationszeitanker muessen streng steigen und hoechstens ein Viertel auseinanderliegen"
            );
            anchors.push((*tick, position));
        }
        ensure!(anchors.len() >= 2, "MIDI-Notationszeitkarte enthaelt keinen Endanker");
        Ok(Self { beats, pickup, anchors })
    }

    fn position(&self, tick: u64) -> Result<i64> {
        let upper = self.anchors.partition_point(|(anchor_tick, _)| *anchor_tick < tick);
        let Some(&(right_tick, right_pos)) = self.anchors.get(upper) else {
            bail!("Quellzeit liegt nach dem letzten Notationszeitanker");
        };
        if right_tick == tick {
            return Ok(right_pos);
        }
        ensure!(upper > 0, "Quellzeit liegt vor dem ersten Notationszeitanker");
        let (left_tick, left_pos) = self.anchors[upper - 1];
        let numerator = i128::from(tick - left_tick) * i128::from(right_pos - left_pos);
        let step = i128::from(right_tick - left_tick) * 6;
        let lower = numerator / step;
        let remainder = numerator % step;
        let nearest = lower + i128::from(remainder * 2 > step);
        ensure!(
            remainder.min(step - remainder) * 4 <= step,
            "Quellzeit liegt ausserhalb des eindeutigen Notationsrasters"
        );
        Ok(left_pos + i64::try_from(nearest * 6)?)
    }
}

fn ordered_source_events(
    source: &DecodedMidiSource,
) -> Result<Vec<crate::source_event::CanonicalSourceEvent>> {
    let mut originals = vec![None; source.notes.len()];
    for event in &source.notes {
        ensure!(
            event.source_ordinal < originals.len() && originals[event.source_ordinal].is_none(),
            "Quellordinalitaet ist nicht eindeutig"
        );
        originals[event.source_ordinal] = Some(*event);
    }
    originals.into_iter().collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow!("Quellordinalitaet ist nicht vollstaendig"))
}

/// One immutable inference shared by the seed workers within the project run.
#[derive(Debug)]
pub(crate) struct PreparedPerformanceTiming {
    path: std::path::PathBuf,
    decoded: DecodedMidiSource,
    plan: std::sync::Arc<NotationTimePlan>,
}

std::thread_local! {
    static PREPARED_TIMING: std::cell::RefCell<Option<std::sync::Arc<PreparedPerformanceTiming>>> =
        const { std::cell::RefCell::new(None) };
}

pub(crate) struct PerformanceTimingScope {
    prior: Option<std::sync::Arc<PreparedPerformanceTiming>>,
    same_thread: std::marker::PhantomData<std::rc::Rc<()>>,
}

impl Drop for PerformanceTimingScope {
    fn drop(&mut self) {
        PREPARED_TIMING.with(|slot| slot.replace(self.prior.take()));
    }
}

impl PreparedPerformanceTiming {
    pub(crate) fn prepare_performance(
        path: &std::path::Path,
        mut decoded: DecodedMidiSource,
        samples: &[f32],
        sample_rate: u32,
    ) -> Result<Self> {
        ensure!(decoded.timing_markers.is_empty(), "Explizite Notationszeitkarte hat Vorrang");
        let inferred = free_performance_timing::infer_performance(&decoded, samples, sample_rate)?;
        if decoded.opening_key_signature.is_none() {
            decoded.opening_key_signature = inferred.key;
        }
        validate_performance_meter(inferred.beats, inferred.beat_type, inferred.pickup)?;
        let originals = ordered_source_events(&decoded)?;
        let extent = inferred.events.iter().map(|note| note.pos + note.len).max().unwrap_or(0);
        let plan = NotationTimePlan {
            source_domain: decoded.time_domain,
            metric: MetricTimeline::from_written_meter(inferred.beats, inferred.beat_type, inferred.pickup, extent)?,
            events: inferred.events,
            source_events: originals,
            timing_authority: TimingAuthority::AudioAlignedPerformance,
            open_durations: inferred.open_durations,
        };
        Ok(Self { path: path.to_path_buf(), decoded, plan: plan.into() })
    }

    pub(crate) fn enter(value: Option<&std::sync::Arc<Self>>) -> PerformanceTimingScope {
        PerformanceTimingScope {
            prior: PREPARED_TIMING.with(|slot| slot.replace(value.cloned())),
            same_thread: std::marker::PhantomData,
        }
    }

    pub(crate) fn for_path(path: &std::path::Path) -> Option<std::sync::Arc<Self>> {
        PREPARED_TIMING.with(|slot| slot.borrow().as_ref().filter(|timing| timing.path == path).cloned())
    }

    pub(crate) fn source_and_plan(&self) -> (&DecodedMidiSource, std::sync::Arc<NotationTimePlan>) {
        (&self.decoded, std::sync::Arc::clone(&self.plan))
    }
}

impl NotationTimePlan {
    pub(crate) fn from_midi(source: &DecodedMidiSource) -> Result<Self> {
        Self::from_source(source).or_else(|exact_error| {
            if !source.timing_markers.is_empty()
                || (!source.time_signatures.is_empty() && source.time_signatures != [(2, 4)])
            {
                return Err(exact_error);
            }
            Self::from_inferred_performance(source)
                .map_err(|error| exact_error.context(format!("Performanceraster: {error}")))
        })
    }

    /// The seed represents the final intervals after the open release proposals
    /// in an audio-aligned performance have received their musical context.
    pub(crate) fn binds_written_intervals(&self) -> bool {
        matches!(self.timing_authority, TimingAuthority::PerformanceNotes | TimingAuthority::AudioAlignedPerformance)
    }

    pub(crate) fn independent_written_intervals(
        &self, source: &DecodedMidiSource,
    ) -> Result<BTreeMap<usize, WrittenHoldProposal>> {
        if self.timing_authority == TimingAuthority::InferredPerformance {
            free_performance_timing::independently_sustained_intervals(source, &self.events)
        } else {
            Ok(BTreeMap::new())
        }
    }

    pub(crate) fn has_open_durations(&self) -> bool {
        !self.open_durations.is_empty()
    }

    pub(crate) fn duration_is_open(&self, id: usize) -> bool {
        self.open_durations.contains(&id)
    }

    /// Close one provisional release in the worker's plan before allocating
    /// voices. Invalid decisions leave both the event and its open status intact.
    pub(crate) fn resolve_duration(&mut self, id: usize, length: i64) -> Result<()> {
        ensure!(self.duration_is_open(id), "Written duration of source {id} is already bound");
        let note = self.event(id)?;
        ensure!(note.kind == NotationEventKind::Regular && length > 0,
            "Invalid written duration for source {id}");
        let end = note.pos.checked_add(length).context("Written release overflow")?;
        ensure!(self.metric.ordinal_at(note.pos).is_some() && self.metric.ordinal_at(end - 1).is_some(),
            "Written duration of source {id} exceeds the metric timeline");
        self.events[id].len = length;
        self.open_durations.remove(&id);
        Ok(())
    }

    pub(crate) fn preserves_attacks(&self) -> bool {
        self.binds_written_intervals() || self.timing_authority == TimingAuthority::InferredPerformance
    }

    fn from_inferred_performance(source: &DecodedMidiSource) -> Result<Self> {
        let originals = ordered_source_events(source)?;
        let inferred = inferred_performance_timing::infer(&originals)?;
        let extent = inferred.events.iter().map(|note| note.pos + note.len).max().unwrap_or(0);
        Ok(Self {
            source_domain: source.time_domain,
            metric: MetricTimeline::from_written_meter(2, 4, inferred.pickup, extent)?,
            events: inferred.events,
            source_events: originals,
            timing_authority: TimingAuthority::InferredPerformance,
            open_durations: BTreeSet::new(),
        })
    }

    pub(crate) fn from_source(source: &DecodedMidiSource) -> Result<Self> {
        if !source.timing_markers.is_empty() {
            if source.timing_markers.iter().any(|(_, marker)| marker.starts_with("v2 ")) {
                return Self::from_notated_performance(source);
            }
            return Self::from_performance(source);
        }
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
                supported_notation_meter(segment.beats, segment.beat_type),
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
        let originals = ordered_source_events(source)?;
        let n = originals.len();
        let mut lanes = BTreeMap::new();
        for entry in &source.source_event_causal_ledger.entries {
            lanes.insert(
                entry.source_ordinal,
                (entry.note_on.track_index, entry.note_on.channel),
            );
        }
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
                "Notationszeit nicht eindeutig darstellbar: Quellnote {}, Takt {}. Die MIDI-Zeit passt weder exakt zum Notationsraster noch zu einem eindeutigen Vorschlag",
                i,
                bar
            );
        }
        Ok(Self {
            source_domain: source.time_domain,
            metric,
            events: events.into_iter().flatten().collect(),
            source_events: originals,
            timing_authority: TimingAuthority::ExactMidi,
            open_durations: BTreeSet::new(),
        })
    }

    fn from_notated_performance(source: &DecodedMidiSource) -> Result<Self> {
        let originals = ordered_source_events(source)?;
        let mut events = vec![None; originals.len()];
        let mut locators = vec![None; originals.len()];
        for entry in &source.source_event_causal_ledger.entries {
            let slot = locators.get_mut(entry.source_ordinal)
                .ok_or_else(|| anyhow!("Notationszeitkarte: Quellordinalitaet ausserhalb der MIDI-Quelle"))?;
            ensure!(slot.replace(entry.note_on).is_none(), "Notationszeitkarte: doppelte Quellordinalitaet");
        }
        let mut meter = None;
        let mut attacks = BTreeSet::new();
        let mut extent = 0;
        for (tick, marker) in &source.timing_markers {
            let fields: Vec<_> = marker.split_ascii_whitespace().collect();
            let (ordinal, channel, key, pos, len, kind) = match fields.as_slice() {
                ["v2", "meter", signature, "pickup", pickup] => {
                    ensure!(*tick == 0 && meter.is_none(), "Notationszeitkarte: genau ein Taktkopf an MIDI-Tick 0 ist erforderlich");
                    let (beats, beat_type) = signature.split_once('/').unwrap_or((signature, "4"));
                    let (beats, beat_type, pickup) = (beats.parse()?, beat_type.parse()?, pickup.parse()?);
                    validate_performance_meter(beats, beat_type, pickup)?;
                    meter = Some((beats, beat_type, pickup));
                    continue;
                }
                ["v2", "note", ordinal, channel, key, pos, len] => {
                    let (pos, len) = (pos.parse::<i64>()?, len.parse::<i64>()?);
                    ensure!(pos >= 0 && pos % 6 == 0 && plain_duration(len),
                        "Notationszeitkarte: Einsatz oder Dauer liegt ausserhalb des Notationsrasters");
                    (ordinal, channel, key, pos, len, NotationEventKind::Regular)
                }
                ["v2", "grace", ordinal, channel, key, host] => {
                    (ordinal, channel, key, 0, 0, NotationEventKind::Grace { host: host.parse()? })
                }
                ["v2", "playback", ordinal, channel, key] => {
                    (ordinal, channel, key, 0, 0, NotationEventKind::PlaybackOnly)
                }
                _ => bail!("Ungueltiger Eintrag der notenbezogenen MIDI-Notationszeitkarte"),
            };
            let ordinal = ordinal.parse::<usize>()?;
            let locator = locators.get(ordinal).copied().flatten()
                .ok_or_else(|| anyhow!("Notationszeitkarte: unbekannte Quellnote {ordinal}"))?;
            ensure!(locator.tick == *tick && locator.channel == channel.parse::<u8>()? && locator.key == key.parse::<u8>()?,
                "Notationszeitkarte: Quellnote {ordinal} stimmt nicht mit MIDI-Tick, Kanal und Ton ueberein");
            if kind == NotationEventKind::Regular {
                ensure!(attacks.insert((pos, locator.channel, locator.key)),
                    "Notationszeitkarte: gleichnamige Anschlaege fallen auf Position {pos} zusammen");
                extent = extent.max(pos.checked_add(len).ok_or_else(|| anyhow!("Notationszeitkarte: Notenende ueberlaeuft"))?);
            }
            ensure!(events[ordinal].replace(NotationEvent { source_ordinal: ordinal, pos, len, kind }).is_none(),
                "Notationszeitkarte: doppelter Eintrag fuer Quellnote {ordinal}");
        }
        let (beats, beat_type, pickup) = meter.ok_or_else(|| anyhow!("Notationszeitkarte: Taktkopf fehlt"))?;
        ensure!(!attacks.is_empty(), "Notationszeitkarte: keine regulaere notierte Note");
        ensure!(source.time_signatures.is_empty() || source.time_signatures == [(beats, beat_type)],
            "MIDI-Taktart und Notationszeitkarte widersprechen sich");
        for ordinal in 0..events.len() {
            let event = events[ordinal].as_ref()
                .ok_or_else(|| anyhow!("Notationszeitkarte: Eintrag fuer Quellnote {ordinal} fehlt"))?;
            if let NotationEventKind::Grace { host } = event.kind {
                let host = events.get(host).and_then(Option::as_ref)
                    .filter(|event| event.kind == NotationEventKind::Regular)
                    .ok_or_else(|| anyhow!("Notationszeitkarte: Vorschlag {ordinal} hat keine regulaere Hauptnote"))?;
                let attack = locators[ordinal].expect("bound grace");
                let host_attack = locators[host.source_ordinal].expect("bound host");
                ensure!(attack.tick < host_attack.tick && (attack.track_index, attack.channel) == (host_attack.track_index, host_attack.channel),
                    "Notationszeitkarte: Vorschlag {ordinal} und Hauptnote sind nicht zeitlich und stimmlich gebunden");
                let host_pos = host.pos;
                events[ordinal].as_mut().expect("bound grace").pos = host_pos;
            }
        }
        let events = events.into_iter().flatten().collect();
        Ok(Self {
            source_domain: source.time_domain,
            metric: MetricTimeline::from_written_meter(beats, beat_type, pickup, extent)?,
            events,
            source_events: originals,
            timing_authority: TimingAuthority::PerformanceNotes,
            open_durations: BTreeSet::new(),
        })
    }

    fn from_performance(source: &DecodedMidiSource) -> Result<Self> {
        let timing = PerformanceTiming::read(&source.timing_markers)?;
        ensure!(
            source.time_signatures.is_empty() || source.time_signatures == [(timing.beats, 4)],
            "MIDI-Taktart und Notationszeitkarte widersprechen sich"
        );
        let originals = ordered_source_events(source)?;
        let mut events = vec![None; originals.len()];
        let mut attacks = BTreeSet::new();
        let mut extent = 0;
        for entry in &source.source_event_causal_ledger.entries {
            let ordinal = entry.source_ordinal;
            let project = |tick| timing.position(tick).map_err(|error| {
                anyhow!("Notationszeitkarte: Quellnote {ordinal}, MIDI-Tick {tick}: {error}")
            });
            let pos = project(entry.note_on.tick)?;
            let end = project(entry.note_off.tick)?;
            ensure!(end > pos, "Notationszeitkarte: Quellnote {ordinal} verliert ihre Dauer");
            ensure!(
                attacks.insert((pos, entry.note_on.channel, entry.note_on.key)),
                "Notationszeitkarte: gleichnamige Anschlaege fallen auf Position {pos} zusammen"
            );
            let event = events.get_mut(ordinal).ok_or_else(|| anyhow!("Quellordinalitaet fehlt"))?;
            ensure!(event.is_none(), "Quellordinalitaet ist nicht eindeutig");
            *event = Some(NotationEvent {
                source_ordinal: ordinal,
                pos,
                len: end - pos,
                kind: NotationEventKind::Regular,
            });
            extent = extent.max(end);
        }
        let events = events.into_iter().collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("Notationszeitkarte: Quellordinalitaet ist nicht vollstaendig"))?;
        Ok(Self {
            source_domain: source.time_domain,
            metric: MetricTimeline::from_written_meter(timing.beats, 4, timing.pickup, extent)?,
            events,
            source_events: originals,
            timing_authority: TimingAuthority::PerformanceAnchors,
            open_durations: BTreeSet::new(),
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
                NotationEventKind::PlaybackOnly => bytes.push(2),
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

    /// Sustains crossing written bar lines require intervals in written units.
    pub(crate) fn written_sustain_interval(&self, ordinal: usize) -> Result<std::ops::Range<i64>> {
        let (onset, length) = if self.timing_authority != TimingAuthority::ExactMidi {
            let note = self.event(ordinal)?;
            ensure!(note.kind != NotationEventKind::PlaybackOnly,
                "Wiedergabeereignis besitzt kein notiertes Dauerintervall");
            (note.pos, note.len)
        } else {
            let note = self.source_event(ordinal)?;
            let onset = grid(note.pos, self.source_domain.divisions)
                .ok_or_else(|| anyhow!("Inexact source onset"))?;
            let length = grid(note.len, self.source_domain.divisions)
                .ok_or_else(|| anyhow!("Inexact source duration"))?;
            (onset, length)
        };
        let end = onset.checked_add(length).ok_or_else(|| anyhow!("Source sustain overflow"))?;
        Ok(onset..end)
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
