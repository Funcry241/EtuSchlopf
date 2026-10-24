//! Canonical source-note event identity shared by productive source decoders and transition audits.
//!
//! This owner defines only the lossless 96-division event representation, its historical
//! ordinal-sensitive identity projection and the read-only causal proof from raw MIDI pairs to
//! canonical source events. It owns no selection, ranking, hand/staff, solver, phrase or writer
//! policy.

use crate::sha256;
use anyhow::{anyhow, Context};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const CANONICAL_DIVISIONS: i64 = 96;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalSourceEvent {
    pub(crate) source_ordinal: usize,
    pub(crate) pos: i64,
    pub(crate) len: i64,
    pub(crate) midi: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceEventLocator {
    pub(crate) tick: u64,
    pub(crate) track_index: usize,
    pub(crate) event_index: usize,
    pub(crate) channel: u8,
    pub(crate) key: u8,
}

impl SourceEventLocator {
    fn order_key(self) -> (u64, usize, usize) {
        (self.tick, self.track_index, self.event_index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceEventCausalEntry {
    pub(crate) source_ordinal: usize,
    pub(crate) note_on: SourceEventLocator,
    pub(crate) note_off: SourceEventLocator,
    pub(crate) canonical: CanonicalSourceEvent,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SourceEventCausalLedger {
    pub(crate) raw_note_event_count: usize,
    pub(crate) entries: Vec<SourceEventCausalEntry>,
}

#[derive(Debug, Clone)]
pub(crate) struct CanonicalSourceEventProjection {
    pub(crate) event_count: usize,
    pub(crate) duplicate_pitch_onset_events: usize,
    pub(crate) terminal_extent_ticks: i64,
    pub(crate) event_fingerprint_sha256: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SourceEventCausalLedgerProjection {
    pub(crate) raw_note_event_count: usize,
    pub(crate) entry_count: usize,
    pub(crate) cross_track_pair_count: usize,
    pub(crate) raw_pair_fingerprint_sha256: String,
    pub(crate) causal_fingerprint_sha256: String,
}

pub(crate) fn project_source_events(
    events: &[CanonicalSourceEvent],
) -> CanonicalSourceEventProjection {
    let mut ordered = events.to_vec();
    ordered.sort_by_key(|event| (event.pos, event.midi, event.source_ordinal));
    let mut canonical = Vec::with_capacity(ordered.len().saturating_mul(28));
    let mut duplicate_pitch_onset_events = 0usize;
    let mut previous_pitch_onset = None;
    let mut terminal_extent_ticks = 0i64;

    for event in &ordered {
        let key = (event.pos, event.midi);
        if previous_pitch_onset == Some(key) {
            duplicate_pitch_onset_events = duplicate_pitch_onset_events.saturating_add(1);
        }
        previous_pitch_onset = Some(key);
        canonical.extend_from_slice(
            &u64::try_from(event.source_ordinal)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        canonical.extend_from_slice(&event.pos.to_le_bytes());
        canonical.extend_from_slice(&event.len.to_le_bytes());
        canonical.extend_from_slice(&event.midi.to_le_bytes());
        terminal_extent_ticks =
            terminal_extent_ticks.max(event.pos.saturating_add(event.len.max(1)));
    }

    CanonicalSourceEventProjection {
        event_count: ordered.len(),
        duplicate_pitch_onset_events,
        terminal_extent_ticks,
        event_fingerprint_sha256: sha256::hex(&canonical),
    }
}

pub(crate) fn normalize_tick_exact(value: i64, source_ppq: i64) -> anyhow::Result<i64> {
    let numerator = i128::from(value) * i128::from(CANONICAL_DIVISIONS);
    let denominator = i128::from(source_ppq);
    if denominator <= 0 || numerator % denominator != 0 {
        return Err(anyhow!(
            "MIDI-Tick {value} bei PPQ {source_ppq} ist nicht verlustfrei auf {CANONICAL_DIVISIONS} Divisions abbildbar"
        ));
    }
    i64::try_from(numerator / denominator)
        .map_err(|_| anyhow!("normalisierte MIDI-Tickposition überläuft i64"))
}

pub(crate) fn project_source_event_causality(
    ledger: &SourceEventCausalLedger,
    canonical_events: &[CanonicalSourceEvent],
    source_ppq: i64,
) -> anyhow::Result<SourceEventCausalLedgerProjection> {
    if source_ppq <= 0 {
        return Err(anyhow!(
            "SourceEventCausalLedger: source_ppq muss größer als 0 sein"
        ));
    }
    let expected_raw_note_event_count = ledger
        .entries
        .len()
        .checked_mul(2)
        .ok_or_else(|| anyhow!("SourceEventCausalLedger: Rohereigniszahl überläuft usize"))?;
    if ledger.raw_note_event_count != expected_raw_note_event_count {
        return Err(anyhow!(
            "SourceEventCausalLedger: Rohereignisse {} ungleich zwei je Ledger-Eintrag {}",
            ledger.raw_note_event_count,
            expected_raw_note_event_count
        ));
    }
    if ledger.entries.len() != canonical_events.len() {
        return Err(anyhow!(
            "SourceEventCausalLedger: Ledger-Einträge {} ungleich CanonicalSourceEvents {}",
            ledger.entries.len(),
            canonical_events.len()
        ));
    }

    let mut canonical_by_ordinal = BTreeMap::new();
    for event in canonical_events {
        if canonical_by_ordinal
            .insert(event.source_ordinal, *event)
            .is_some()
        {
            return Err(anyhow!(
                "SourceEventCausalLedger: doppelte CanonicalSourceEvent-Ordinalität {}",
                event.source_ordinal
            ));
        }
    }
    for expected in 0..canonical_events.len() {
        if !canonical_by_ordinal.contains_key(&expected) {
            return Err(anyhow!(
                "SourceEventCausalLedger: fehlende CanonicalSourceEvent-Ordinalität {expected}"
            ));
        }
    }

    let mut entries = ledger.entries.clone();
    entries.sort_by_key(|entry| entry.source_ordinal);
    for (expected, entry) in entries.iter().enumerate() {
        if entry.source_ordinal != expected {
            return Err(anyhow!(
                "SourceEventCausalLedger: Ledger-Ordinalität {} statt erwarteter {}",
                entry.source_ordinal,
                expected
            ));
        }
    }

    let mut note_on_order = entries.iter().collect::<Vec<_>>();
    note_on_order.sort_by_key(|entry| entry.note_on.order_key());
    for (expected, entry) in note_on_order.into_iter().enumerate() {
        if entry.source_ordinal != expected {
            return Err(anyhow!(
                "SourceEventCausalLedger: Source-Ordinalität {} folgt nicht der globalen Note-On-Reihenfolge {}",
                entry.source_ordinal,
                expected
            ));
        }
    }

    let mut raw_locators = BTreeSet::new();
    let mut pairs_by_channel_key: BTreeMap<(u8, u8), Vec<&SourceEventCausalEntry>> =
        BTreeMap::new();
    let mut cross_track_pair_count = 0usize;
    let mut raw_bytes = Vec::new();
    raw_bytes.extend_from_slice(b"ETUSCHLOPF_SOURCE_EVENT_RAW_PAIR_LEDGER_V1\0");
    raw_bytes.extend_from_slice(&source_ppq.to_le_bytes());
    append_usize(
        &mut raw_bytes,
        ledger.raw_note_event_count,
        "Rohereigniszahl",
    )?;
    append_usize(&mut raw_bytes, entries.len(), "Ledger-Eintragzahl")?;
    let mut causal_bytes = Vec::new();
    causal_bytes.extend_from_slice(b"ETUSCHLOPF_SOURCE_EVENT_CAUSAL_LEDGER_V1\0");
    causal_bytes.extend_from_slice(&source_ppq.to_le_bytes());
    causal_bytes.extend_from_slice(&CANONICAL_DIVISIONS.to_le_bytes());
    append_usize(
        &mut causal_bytes,
        ledger.raw_note_event_count,
        "Rohereigniszahl",
    )?;
    append_usize(&mut causal_bytes, entries.len(), "Ledger-Eintragzahl")?;

    for entry in &entries {
        let canonical = canonical_by_ordinal
            .get(&entry.source_ordinal)
            .with_context(|| {
                format!(
                    "SourceEventCausalLedger: CanonicalSourceEvent {} fehlt",
                    entry.source_ordinal
                )
            })?;
        if entry.canonical != *canonical {
            return Err(anyhow!(
                "SourceEventCausalLedger: eingebettetes CanonicalSourceEvent {} weicht vom produktiven Ereignis ab",
                entry.source_ordinal
            ));
        }
        if entry.note_on.channel != entry.note_off.channel
            || entry.note_on.key != entry.note_off.key
        {
            return Err(anyhow!(
                "SourceEventCausalLedger: Note-On/Note-Off-Paar {} stimmt in Kanal oder Ton nicht überein",
                entry.source_ordinal
            ));
        }
        if entry.note_off.tick <= entry.note_on.tick {
            return Err(anyhow!(
                "SourceEventCausalLedger: Paar {} besitzt keine positive Rohdauer",
                entry.source_ordinal
            ));
        }
        if i32::from(entry.note_on.key) != canonical.midi {
            return Err(anyhow!(
                "SourceEventCausalLedger: Rohpitch und CanonicalSourceEvent {} stimmen nicht überein",
                entry.source_ordinal
            ));
        }

        let raw_start = i64::try_from(entry.note_on.tick)
            .map_err(|_| anyhow!("SourceEventCausalLedger: Rohstart überläuft i64"))?;
        let raw_length = i64::try_from(entry.note_off.tick - entry.note_on.tick)
            .map_err(|_| anyhow!("SourceEventCausalLedger: Rohdauer überläuft i64"))?;
        let normalized_start = normalize_tick_exact(raw_start, source_ppq)?;
        let normalized_length = normalize_tick_exact(raw_length, source_ppq)?;
        if normalized_start != canonical.pos || normalized_length != canonical.len {
            return Err(anyhow!(
                "SourceEventCausalLedger: exakte Normalisierung von Paar {} reproduziert CanonicalSourceEvent nicht",
                entry.source_ordinal
            ));
        }

        for locator in [entry.note_on, entry.note_off] {
            if !raw_locators.insert(locator.order_key()) {
                return Err(anyhow!(
                    "SourceEventCausalLedger: Rohereignis-Locator {:?} wird mehrfach verwendet",
                    locator.order_key()
                ));
            }
        }
        if entry.note_on.track_index != entry.note_off.track_index {
            cross_track_pair_count = cross_track_pair_count.saturating_add(1);
        }
        pairs_by_channel_key
            .entry((entry.note_on.channel, entry.note_on.key))
            .or_default()
            .push(entry);

        append_raw_pair_bytes(&mut raw_bytes, entry)?;
        append_raw_pair_bytes(&mut causal_bytes, entry)?;
        append_canonical_event_bytes(&mut causal_bytes, canonical)?;
    }

    for ((channel, key), pairs) in &mut pairs_by_channel_key {
        pairs.sort_by_key(|entry| entry.note_on.order_key());
        for pair in pairs.windows(2) {
            if pair[0].note_off.order_key() >= pair[1].note_off.order_key() {
                return Err(anyhow!(
                    "SourceEventCausalLedger: FIFO-Replay scheitert für Kanal {} Ton {} zwischen Ordinalität {} und {}",
                    channel,
                    key,
                    pair[0].source_ordinal,
                    pair[1].source_ordinal
                ));
            }
        }
    }

    Ok(SourceEventCausalLedgerProjection {
        raw_note_event_count: ledger.raw_note_event_count,
        entry_count: entries.len(),
        cross_track_pair_count,
        raw_pair_fingerprint_sha256: sha256::hex(&raw_bytes),
        causal_fingerprint_sha256: sha256::hex(&causal_bytes),
    })
}

fn append_raw_pair_bytes(
    out: &mut Vec<u8>,
    entry: &SourceEventCausalEntry,
) -> anyhow::Result<()> {
    append_usize(out, entry.source_ordinal, "SourceEvent-Ordinalität")?;
    append_locator_bytes(out, entry.note_on)?;
    append_locator_bytes(out, entry.note_off)?;
    Ok(())
}

fn append_locator_bytes(out: &mut Vec<u8>, locator: SourceEventLocator) -> anyhow::Result<()> {
    out.extend_from_slice(&locator.tick.to_le_bytes());
    append_usize(out, locator.track_index, "MIDI-Trackindex")?;
    append_usize(out, locator.event_index, "MIDI-Ereignisindex")?;
    out.push(locator.channel);
    out.push(locator.key);
    Ok(())
}

fn append_canonical_event_bytes(
    out: &mut Vec<u8>,
    event: &CanonicalSourceEvent,
) -> anyhow::Result<()> {
    append_usize(out, event.source_ordinal, "CanonicalSourceEvent-Ordinalität")?;
    out.extend_from_slice(&event.pos.to_le_bytes());
    out.extend_from_slice(&event.len.to_le_bytes());
    out.extend_from_slice(&event.midi.to_le_bytes());
    Ok(())
}

fn append_usize(out: &mut Vec<u8>, value: usize, label: &str) -> anyhow::Result<()> {
    let value = u64::try_from(value).map_err(|_| anyhow!("{label} überläuft u64"))?;
    out.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locator(
        tick: u64,
        track_index: usize,
        event_index: usize,
        channel: u8,
        key: u8,
    ) -> SourceEventLocator {
        SourceEventLocator {
            tick,
            track_index,
            event_index,
            channel,
            key,
        }
    }

    #[test]
    fn projection_preserves_historical_ordinal_sensitive_contract() {
        let events = [
            CanonicalSourceEvent {
                source_ordinal: 1,
                pos: 0,
                len: 96,
                midi: 60,
            },
            CanonicalSourceEvent {
                source_ordinal: 0,
                pos: 0,
                len: 48,
                midi: 60,
            },
            CanonicalSourceEvent {
                source_ordinal: 2,
                pos: 96,
                len: 96,
                midi: 64,
            },
        ];
        let projection = project_source_events(&events);
        assert_eq!(projection.event_count, 3);
        assert_eq!(projection.duplicate_pitch_onset_events, 1);
        assert_eq!(projection.terminal_extent_ticks, 192);
        assert_eq!(
            projection.event_fingerprint_sha256,
            "882e0d808b8a2815afd6f5a63c1af54a43e34dc43610a736c234e96a803b878f"
        );
    }

    #[test]
    fn causal_projection_replays_raw_pairing_and_preserves_duplicate_identity() {
        let canonical_events = [
            CanonicalSourceEvent {
                source_ordinal: 0,
                pos: 0,
                len: 48,
                midi: 60,
            },
            CanonicalSourceEvent {
                source_ordinal: 1,
                pos: 0,
                len: 96,
                midi: 60,
            },
            CanonicalSourceEvent {
                source_ordinal: 2,
                pos: 96,
                len: 96,
                midi: 64,
            },
        ];
        let ledger = SourceEventCausalLedger {
            raw_note_event_count: 6,
            entries: vec![
                SourceEventCausalEntry {
                    source_ordinal: 2,
                    note_on: locator(96, 0, 8, 0, 64),
                    note_off: locator(192, 1, 5, 0, 64),
                    canonical: canonical_events[2],
                },
                SourceEventCausalEntry {
                    source_ordinal: 0,
                    note_on: locator(0, 0, 2, 0, 60),
                    note_off: locator(48, 0, 4, 0, 60),
                    canonical: canonical_events[0],
                },
                SourceEventCausalEntry {
                    source_ordinal: 1,
                    note_on: locator(0, 0, 3, 0, 60),
                    note_off: locator(96, 0, 6, 0, 60),
                    canonical: canonical_events[1],
                },
            ],
        };

        let projection = project_source_event_causality(&ledger, &canonical_events, 96)
            .expect("causal pairing must replay exactly");
        assert_eq!(projection.raw_note_event_count, 6);
        assert_eq!(projection.entry_count, 3);
        assert_eq!(projection.cross_track_pair_count, 1);
        assert_eq!(
            projection.raw_pair_fingerprint_sha256,
            "bbc13e5b5e3571cd6b85c7d4005a3af454c5925d5319b23e8490560637588291"
        );
        assert_eq!(
            projection.causal_fingerprint_sha256,
            "105b34c2deac1a34b2f34b12beb20227e1ca6a4523b0d47802df86585b25f08d"
        );
    }

    #[test]
    fn causal_projection_rejects_non_fifo_duplicate_pairing() {
        let canonical_events = [
            CanonicalSourceEvent {
                source_ordinal: 0,
                pos: 0,
                len: 96,
                midi: 60,
            },
            CanonicalSourceEvent {
                source_ordinal: 1,
                pos: 0,
                len: 48,
                midi: 60,
            },
        ];
        let ledger = SourceEventCausalLedger {
            raw_note_event_count: 4,
            entries: vec![
                SourceEventCausalEntry {
                    source_ordinal: 0,
                    note_on: locator(0, 0, 0, 0, 60),
                    note_off: locator(96, 0, 3, 0, 60),
                    canonical: canonical_events[0],
                },
                SourceEventCausalEntry {
                    source_ordinal: 1,
                    note_on: locator(0, 0, 1, 0, 60),
                    note_off: locator(48, 0, 2, 0, 60),
                    canonical: canonical_events[1],
                },
            ],
        };

        let error = project_source_event_causality(&ledger, &canonical_events, 96)
            .expect_err("crossed duplicate pairing must fail FIFO replay");
        assert!(error.to_string().contains("FIFO-Replay"));
    }
}
