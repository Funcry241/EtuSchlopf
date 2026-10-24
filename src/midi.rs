//! Productive fail-closed Standard MIDI File decoder.
//!
//! Supports SMF formats 0 and 1 with PPQ time division, Running Status, Note-On/Note-Off,
//! tempo and time-signature metadata. Notes are normalized losslessly to EtuSchlopf's canonical
//! 96 divisions. The decoder owns source decoding only; it grants no hand/staff, solver or writer policy.

use crate::source_event::{
    normalize_tick_exact, project_source_event_causality, CanonicalSourceEvent,
    SourceEventCausalEntry, SourceEventCausalLedger, SourceEventLocator,
};
use anyhow::{anyhow, Context};
use std::collections::{BTreeMap, VecDeque};
use std::fs;
use std::path::Path;

const MIDI_HEADER: &[u8; 4] = b"MThd";
const MIDI_TRACK: &[u8; 4] = b"MTrk";

#[derive(Debug, Clone, Copy)]
enum RawNoteEventKind { On, Off }

#[derive(Debug, Clone, Copy)]
struct RawNoteEvent {
    tick: u64,
    track_index: usize,
    event_index: usize,
    channel: u8,
    key: u8,
    kind: RawNoteEventKind,
}

#[derive(Debug, Clone, Copy)]
struct ActiveNote {
    source_ordinal: usize,
    note_on: SourceEventLocator,
}

#[derive(Debug)]
struct ParsedMidi {
    format: u16,
    tracks: u16,
    ppq: i64,
    note_events: Vec<RawNoteEvent>,
    tempo_microseconds_per_quarter: Vec<u32>,
    time_signatures: Vec<(u8, u16)>,
}

#[derive(Debug, Clone)]
pub(crate) struct DecodedMidiSource {
    pub(crate) format: u16,
    pub(crate) tracks: u16,
    pub(crate) source_ppq: i64,
    pub(crate) notes: Vec<CanonicalSourceEvent>,
    pub(crate) source_event_causal_ledger: SourceEventCausalLedger,
    pub(crate) tempo_microseconds_per_quarter: Vec<u32>,
    pub(crate) time_signatures: Vec<(u8, u16)>,
}

pub(crate) fn read_standard_midi(path: &Path) -> anyhow::Result<DecodedMidiSource> {
    let data = fs::read(path)
        .with_context(|| format!("MIDI-Datei nicht lesbar: {}", path.display()))?;
    let parsed = parse_midi(&data)?;
    let (notes, source_event_causal_ledger) = pair_and_normalize_note_events(&parsed)?;
    if notes.is_empty() {
        return Err(anyhow!("MIDI enthält keine lesbaren Notenereignisse"));
    }
    project_source_event_causality(&source_event_causal_ledger, &notes, parsed.ppq)?;
    Ok(DecodedMidiSource {
        format: parsed.format,
        tracks: parsed.tracks,
        source_ppq: parsed.ppq,
        notes,
        source_event_causal_ledger,
        tempo_microseconds_per_quarter: parsed.tempo_microseconds_per_quarter,
        time_signatures: parsed.time_signatures,
    })
}

fn parse_midi(data: &[u8]) -> anyhow::Result<ParsedMidi> {
    if data.len() < 14 || data.get(0..4) != Some(&MIDI_HEADER[..]) {
        return Err(anyhow!("Standard-MIDI-Header MThd fehlt"));
    }
    let header_len = usize::try_from(read_u32_be(data, 4)?)
        .map_err(|_| anyhow!("MIDI-Headerlänge überläuft den Adressraum"))?;
    if header_len < 6 {
        return Err(anyhow!("MIDI-Header ist kürzer als 6 Bytes"));
    }
    let header_end = 8usize
        .checked_add(header_len)
        .ok_or_else(|| anyhow!("MIDI-Headerende überläuft den Adressraum"))?;
    if header_end > data.len() {
        return Err(anyhow!("MIDI-Header ist abgeschnitten"));
    }
    let format = read_u16_be(data, 8)?;
    if format > 1 {
        return Err(anyhow!(
            "MIDI-Format {format} ist für den eindeutigen Quellvergleich nicht zugelassen"
        ));
    }
    let tracks = read_u16_be(data, 10)?;
    if tracks == 0 {
        return Err(anyhow!("MIDI enthält keine Tracks"));
    }
    if format == 0 && tracks != 1 {
        return Err(anyhow!("MIDI-Format 0 muss exakt einen Track enthalten"));
    }
    let division = read_u16_be(data, 12)?;
    if division & 0x8000 != 0 {
        return Err(anyhow!("SMPTE-Zeitteilung ist für den Quellvergleich nicht zugelassen"));
    }
    let ppq = i64::from(division);
    if ppq <= 0 {
        return Err(anyhow!("MIDI-PPQ muss größer als 0 sein"));
    }

    let mut pos = header_end;
    let mut note_events = Vec::new();
    let mut tempo_microseconds_per_quarter = Vec::new();
    let mut time_signatures = Vec::new();
    for track_index in 0..usize::from(tracks) {
        if data.get(pos..pos + 4) != Some(&MIDI_TRACK[..]) {
            return Err(anyhow!("MIDI-Track {track_index}: MTrk-Kennung fehlt"));
        }
        let track_len = usize::try_from(read_u32_be(data, pos + 4)?)
            .map_err(|_| anyhow!("MIDI-Tracklänge überläuft den Adressraum"))?;
        let body = pos + 8;
        let end = body
            .checked_add(track_len)
            .ok_or_else(|| anyhow!("MIDI-Trackende überläuft den Adressraum"))?;
        if end > data.len() {
            return Err(anyhow!("MIDI-Track {track_index} ist abgeschnitten"));
        }
        parse_track(
            &data[body..end],
            track_index,
            &mut note_events,
            &mut tempo_microseconds_per_quarter,
            &mut time_signatures,
        )?;
        pos = end;
    }
    if pos != data.len() {
        return Err(anyhow!(
            "MIDI enthält {} nicht deklarierte Restbytes",
            data.len().saturating_sub(pos)
        ));
    }
    tempo_microseconds_per_quarter.sort_unstable();
    tempo_microseconds_per_quarter.dedup();
    time_signatures.sort_unstable();
    time_signatures.dedup();

    Ok(ParsedMidi {
        format,
        tracks,
        ppq,
        note_events,
        tempo_microseconds_per_quarter,
        time_signatures,
    })
}

fn parse_track(
    track: &[u8],
    track_index: usize,
    note_events: &mut Vec<RawNoteEvent>,
    tempos: &mut Vec<u32>,
    time_signatures: &mut Vec<(u8, u16)>,
) -> anyhow::Result<()> {
    let mut pos = 0usize;
    let mut tick = 0u64;
    let mut running_status = None;
    let mut event_index = 0usize;
    let mut end_of_track_seen = false;

    while pos < track.len() {
        let delta = u64::from(read_variable_length(track, &mut pos)?);
        tick = tick
            .checked_add(delta)
            .ok_or_else(|| anyhow!("MIDI-Track {track_index}: Tickposition überläuft u64"))?;
        let first = *track
            .get(pos)
            .ok_or_else(|| anyhow!("MIDI-Track {track_index}: Eventstatus fehlt"))?;
        let status = if first & 0x80 != 0 {
            pos += 1;
            if first < 0xf0 {
                running_status = Some(first);
            } else {
                running_status = None;
            }
            first
        } else {
            running_status.ok_or_else(|| {
                anyhow!("MIDI-Track {track_index}: Running Status ohne Vorgänger")
            })?
        };

        match status {
            0x80..=0xef => {
                let message = status & 0xf0;
                let channel = status & 0x0f;
                let data_len = if matches!(message, 0xc0 | 0xd0) { 1 } else { 2 };
                let first_data = take_byte(track, &mut pos, track_index)?;
                let second_data = if data_len == 2 {
                    Some(take_byte(track, &mut pos, track_index)?)
                } else {
                    None
                };
                if first_data & 0x80 != 0 || second_data.is_some_and(|value| value & 0x80 != 0) {
                    return Err(anyhow!(
                        "MIDI-Track {track_index}: Kanalnachricht enthält ungültiges Datenbyte"
                    ));
                }
                match message {
                    0x80 => note_events.push(RawNoteEvent {
                        tick,
                        track_index,
                        event_index,
                        channel,
                        key: first_data,
                        kind: RawNoteEventKind::Off,
                    }),
                    0x90 => {
                        let velocity = second_data.unwrap_or(0);
                        note_events.push(RawNoteEvent {
                            tick,
                            track_index,
                            event_index,
                            channel,
                            key: first_data,
                            kind: if velocity == 0 {
                                RawNoteEventKind::Off
                            } else {
                                RawNoteEventKind::On
                            },
                        });
                    }
                    _ => {}
                }
            }
            0xff => {
                let meta_type = take_byte(track, &mut pos, track_index)?;
                let size = usize::try_from(read_variable_length(track, &mut pos)?)
                    .map_err(|_| anyhow!("MIDI-Track {track_index}: Meta-Länge überläuft"))?;
                let payload = take_slice(track, &mut pos, size, track_index)?;
                match meta_type {
                    0x2f => {
                        if !payload.is_empty() {
                            return Err(anyhow!(
                                "MIDI-Track {track_index}: End-of-Track muss Länge 0 haben"
                            ));
                        }
                        end_of_track_seen = true;
                        if pos != track.len() {
                            return Err(anyhow!(
                                "MIDI-Track {track_index}: Daten nach End-of-Track"
                            ));
                        }
                    }
                    0x51 => {
                        if payload.len() != 3 {
                            return Err(anyhow!(
                                "MIDI-Track {track_index}: Tempo-Metaereignis muss Länge 3 haben"
                            ));
                        }
                        tempos.push(
                            (u32::from(payload[0]) << 16)
                                | (u32::from(payload[1]) << 8)
                                | u32::from(payload[2]),
                        );
                    }
                    0x58 => {
                        if payload.len() != 4 {
                            return Err(anyhow!(
                                "MIDI-Track {track_index}: Taktart-Metaereignis muss Länge 4 haben"
                            ));
                        }
                        let denominator = 1u16
                            .checked_shl(u32::from(payload[1]))
                            .ok_or_else(|| {
                                anyhow!("MIDI-Track {track_index}: Taktart-Nenner überläuft")
                            })?;
                        time_signatures.push((payload[0], denominator));
                    }
                    _ => {}
                }
            }
            0xf0 | 0xf7 => {
                let size = usize::try_from(read_variable_length(track, &mut pos)?)
                    .map_err(|_| anyhow!("MIDI-Track {track_index}: SysEx-Länge überläuft"))?;
                take_slice(track, &mut pos, size, track_index)?;
            }
            _ => {
                return Err(anyhow!(
                    "MIDI-Track {track_index}: nicht unterstützter Systemstatus 0x{status:02x}"
                ));
            }
        }
        event_index = event_index.saturating_add(1);
    }
    if !end_of_track_seen {
        return Err(anyhow!("MIDI-Track {track_index}: End-of-Track fehlt"));
    }
    Ok(())
}

fn pair_and_normalize_note_events(
    parsed: &ParsedMidi,
) -> anyhow::Result<(Vec<CanonicalSourceEvent>, SourceEventCausalLedger)> {
    let mut ordered = parsed.note_events.clone();
    ordered.sort_by_key(|event| (event.tick, event.track_index, event.event_index));
    let mut active: BTreeMap<(u8, u8), VecDeque<ActiveNote>> = BTreeMap::new();
    let mut completed = Vec::new();
    let mut causal_entries = Vec::new();
    let mut next_source_ordinal = 0usize;

    for event in ordered {
        let locator = SourceEventLocator {
            tick: event.tick,
            track_index: event.track_index,
            event_index: event.event_index,
            channel: event.channel,
            key: event.key,
        };
        match event.kind {
            RawNoteEventKind::On => {
                active
                    .entry((event.channel, event.key))
                    .or_default()
                    .push_back(ActiveNote {
                        source_ordinal: next_source_ordinal,
                        note_on: locator,
                    });
                next_source_ordinal = next_source_ordinal.saturating_add(1);
            }
            RawNoteEventKind::Off => {
                let queue = active.get_mut(&(event.channel, event.key)).ok_or_else(|| {
                    anyhow!(
                        "MIDI Note-Off ohne Note-On: Kanal {} Ton {} Tick {}",
                        event.channel,
                        event.key,
                        event.tick
                    )
                })?;
                let started = queue.pop_front().ok_or_else(|| {
                    anyhow!(
                        "MIDI Note-Off ohne offene Note: Kanal {} Ton {} Tick {}",
                        event.channel,
                        event.key,
                        event.tick
                    )
                })?;
                if event.tick <= started.note_on.tick {
                    return Err(anyhow!(
                        "MIDI-Note ohne positive Dauer: Kanal {} Ton {} Tick {}",
                        event.channel,
                        event.key,
                        event.tick
                    ));
                }
                let start = i64::try_from(started.note_on.tick)
                    .map_err(|_| anyhow!("MIDI-Notenstart überläuft i64"))?;
                let length = i64::try_from(event.tick - started.note_on.tick)
                    .map_err(|_| anyhow!("MIDI-Notendauer überläuft i64"))?;
                let canonical = CanonicalSourceEvent {
                    source_ordinal: started.source_ordinal,
                    pos: normalize_tick_exact(start, parsed.ppq)?,
                    len: normalize_tick_exact(length, parsed.ppq)?,
                    midi: i32::from(event.key),
                };
                completed.push(canonical);
                causal_entries.push(SourceEventCausalEntry {
                    source_ordinal: started.source_ordinal,
                    note_on: started.note_on,
                    note_off: locator,
                    canonical,
                });
            }
        }
    }

    let unclosed = active.values().map(VecDeque::len).sum::<usize>();
    if unclosed != 0 {
        return Err(anyhow!("MIDI enthält {unclosed} nicht geschlossene Note-On-Ereignisse"));
    }
    Ok((
        completed,
        SourceEventCausalLedger {
            raw_note_event_count: parsed.note_events.len(),
            entries: causal_entries,
        },
    ))
}

fn read_variable_length(data: &[u8], pos: &mut usize) -> anyhow::Result<u32> {
    let mut value = 0u32;
    for _ in 0..4 {
        let byte = *data
            .get(*pos)
            .ok_or_else(|| anyhow!("MIDI Variable-Length-Wert ist abgeschnitten"))?;
        *pos += 1;
        value = value
            .checked_shl(7)
            .and_then(|shifted| shifted.checked_add(u32::from(byte & 0x7f)))
            .ok_or_else(|| anyhow!("MIDI Variable-Length-Wert überläuft"))?;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(anyhow!("MIDI Variable-Length-Wert ist länger als 4 Bytes"))
}

fn take_byte(data: &[u8], pos: &mut usize, track_index: usize) -> anyhow::Result<u8> {
    let byte = *data
        .get(*pos)
        .ok_or_else(|| anyhow!("MIDI-Track {track_index}: Datenbyte fehlt"))?;
    *pos += 1;
    Ok(byte)
}

fn take_slice<'a>(
    data: &'a [u8],
    pos: &mut usize,
    size: usize,
    track_index: usize,
) -> anyhow::Result<&'a [u8]> {
    let end = pos
        .checked_add(size)
        .ok_or_else(|| anyhow!("MIDI-Track {track_index}: Datenbereich überläuft"))?;
    let payload = data
        .get(*pos..end)
        .ok_or_else(|| anyhow!("MIDI-Track {track_index}: Ereignisdaten sind abgeschnitten"))?;
    *pos = end;
    Ok(payload)
}

fn read_u16_be(data: &[u8], offset: usize) -> anyhow::Result<u16> {
    let bytes = data
        .get(offset..offset + 2)
        .ok_or_else(|| anyhow!("MIDI u16 an Offset {offset} ist abgeschnitten"))?;
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn read_u32_be(data: &[u8], offset: usize) -> anyhow::Result<u32> {
    let bytes = data
        .get(offset..offset + 4)
        .ok_or_else(|| anyhow!("MIDI u32 an Offset {offset} ist abgeschnitten"))?;
    Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_event::{project_source_event_causality, project_source_events};

    fn variable_length(mut value: u32) -> Vec<u8> {
        let mut bytes = [0u8; 4];
        let mut index = 3usize;
        bytes[index] = u8::try_from(value & 0x7f).unwrap();
        while {
            value >>= 7;
            value != 0
        } {
            index -= 1;
            bytes[index] = u8::try_from(value & 0x7f).unwrap() | 0x80;
        }
        bytes[index..].to_vec()
    }

    fn midi_with_track(track: &[u8]) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(b"MThd");
        data.extend_from_slice(&6u32.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&1u16.to_be_bytes());
        data.extend_from_slice(&96u16.to_be_bytes());
        data.extend_from_slice(b"MTrk");
        data.extend_from_slice(&u32::try_from(track.len()).unwrap().to_be_bytes());
        data.extend_from_slice(track);
        data
    }

    #[test]
    fn decoder_preserves_ordinal_duplicate_duration_and_metadata_identity() {
        let mut track = Vec::new();
        track.extend_from_slice(&[0x00, 0xff, 0x51, 0x03, 0x07, 0x0a, 0xe2]);
        track.extend_from_slice(&[0x00, 0xff, 0x58, 0x04, 0x04, 0x02, 0x18, 0x08]);
        track.extend_from_slice(&[0x00, 0x90, 60, 100]);
        track.extend_from_slice(&[0x00, 60, 90]);
        track.extend_from_slice(&variable_length(48));
        track.extend_from_slice(&[0x80, 60, 0]);
        track.extend_from_slice(&variable_length(48));
        track.extend_from_slice(&[60, 0]);
        track.extend_from_slice(&[0x00, 0x90, 64, 80]);
        track.extend_from_slice(&variable_length(96));
        track.extend_from_slice(&[0x90, 64, 0]);
        track.extend_from_slice(&[0x00, 0xff, 0x2f, 0x00]);

        let parsed = parse_midi(&midi_with_track(&track)).expect("valid MIDI");
        let (events, causal_ledger) =
            pair_and_normalize_note_events(&parsed).expect("paired notes");
        let projection = project_source_events(&events);
        let causal_projection =
            project_source_event_causality(&causal_ledger, &events, parsed.ppq)
                .expect("causal ledger must replay");
        assert_eq!(parsed.tempo_microseconds_per_quarter, vec![461_538]);
        assert_eq!(parsed.time_signatures, vec![(4, 4)]);
        assert_eq!(projection.event_count, 3);
        assert_eq!(causal_projection.raw_note_event_count, 6);
        assert_eq!(causal_projection.entry_count, 3);
        assert_eq!(causal_projection.cross_track_pair_count, 0);
        assert_eq!(projection.duplicate_pitch_onset_events, 1);
        assert_eq!(projection.terminal_extent_ticks, 192);
        assert_eq!(
            projection.event_fingerprint_sha256,
            "882e0d808b8a2815afd6f5a63c1af54a43e34dc43610a736c234e96a803b878f"
        );
    }

    #[test]
    fn decoder_rejects_unclosed_note() {
        let track = [0x00, 0x90, 60, 100, 0x00, 0xff, 0x2f, 0x00];
        let parsed = parse_midi(&midi_with_track(&track)).expect("structurally valid MIDI");
        let error = pair_and_normalize_note_events(&parsed).expect_err("note must remain open");
        assert!(error.to_string().contains("nicht geschlossene"));
    }

    #[test]
    fn decoder_rejects_lossy_ppq_normalization() {
        let parsed = ParsedMidi {
            format: 0,
            tracks: 1,
            ppq: 100,
            note_events: vec![
                RawNoteEvent {
                    tick: 1,
                    track_index: 0,
                    event_index: 0,
                    channel: 0,
                    key: 60,
                    kind: RawNoteEventKind::On,
                },
                RawNoteEvent {
                    tick: 51,
                    track_index: 0,
                    event_index: 1,
                    channel: 0,
                    key: 60,
                    kind: RawNoteEventKind::Off,
                },
            ],
            tempo_microseconds_per_quarter: Vec::new(),
            time_signatures: Vec::new(),
        };
        let error = pair_and_normalize_note_events(&parsed)
            .expect_err("lossy canonical tick projection must fail closed");
        assert!(error.to_string().contains("nicht verlustfrei"));
    }

    #[test]
    fn decoder_rejects_trailing_undeclared_bytes() {
        let track = [0x00, 0xff, 0x2f, 0x00];
        let mut data = midi_with_track(&track);
        data.push(0);
        let error = parse_midi(&data).expect_err("trailing byte must fail closed");
        assert!(error.to_string().contains("Restbytes"));
    }
}
