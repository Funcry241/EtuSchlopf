//! Audit-only WAV observation and frozen legacy source oracle backed by productive MIDI.
//!
//! This module is deliberately outside the productive source/seed path. It may
//! observe exact WAV bytes and canonical MIDI note events for qualification evidence,
//! but owns no pitch, metric, source-event, hand/staff, phrase or writer authority.

use crate::audit_only::wav_signal_decode::decode_wav_sample;
use crate::midi::read_standard_midi;
use crate::source_event::{project_source_events, CANONICAL_DIVISIONS};
#[cfg(test)]
use crate::source_event::CanonicalSourceEvent;
use crate::sha256;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OriginalFrameIndex(pub(crate) u64);

#[derive(Debug, Clone)]
pub(crate) struct WavObservationAudit {
    pub(crate) qualified: bool,
    pub(crate) source_file_sha256: String,
    pub(crate) decoded_signal_fnv64: String,
    pub(crate) audio_format: u16,
    pub(crate) sample_rate: u32,
    pub(crate) channels: u16,
    pub(crate) bits_per_sample: u16,
    pub(crate) block_align: u16,
    pub(crate) original_frame_count: u64,
    pub(crate) first_nonzero_frame: Option<OriginalFrameIndex>,
    pub(crate) last_nonzero_frame: Option<OriginalFrameIndex>,
    pub(crate) decoded_sample_values: u64,
    pub(crate) trailing_partial_data_bytes: usize,
    pub(crate) reason: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct FrozenLegacySourceOracleAudit {
    pub(crate) qualified: bool,
    pub(crate) source_ppq: i64,
    pub(crate) canonical_divisions: i64,
    pub(crate) event_count: usize,
    pub(crate) duplicate_pitch_onset_events: usize,
    pub(crate) terminal_extent_ticks: i64,
    pub(crate) event_fingerprint_sha256: String,
    pub(crate) reason: &'static str,
}

#[derive(Debug, Clone)]
pub(crate) struct WavObservationLegacyOracleQualificationAudit {
    pub(crate) wav: WavObservationAudit,
    pub(crate) legacy: FrozenLegacySourceOracleAudit,
}

pub(crate) fn build_for_score_title(
    score_title: Option<&str>,
) -> Option<WavObservationLegacyOracleQualificationAudit> {
    let title = score_title?.trim();
    if title.is_empty() {
        return None;
    }
    let current = std::env::current_dir().ok()?;
    let wav_path = current.join(format!("{title}.wav"));
    let midi_path = current.join(format!("{title}.mid"));
    if !wav_path.is_file() && !midi_path.is_file() {
        return None;
    }
    Some(WavObservationLegacyOracleQualificationAudit {
        wav: inspect_wav_observation(&wav_path),
        legacy: inspect_frozen_legacy_source_oracle(&midi_path),
    })
}

fn inspect_wav_observation(path: &Path) -> WavObservationAudit {
    let Ok(data) = fs::read(path) else {
        return wav_not_qualified("wav_file_unreadable", &[]);
    };
    inspect_wav_bytes(&data)
}

fn inspect_wav_bytes(data: &[u8]) -> WavObservationAudit {
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return wav_not_qualified("riff_wave_header_missing", data);
    }

    let mut audio_format = None;
    let mut channels = None;
    let mut sample_rate = None;
    let mut bits_per_sample = None;
    let mut block_align = None;
    let mut data_range = None;
    let mut pos = 12usize;

    while pos + 8 <= data.len() {
        let id = &data[pos..pos + 4];
        let Some(size_u32) = read_u32_le(&data[pos + 4..pos + 8]) else {
            return wav_not_qualified("riff_chunk_size_invalid", data);
        };
        let Ok(size) = usize::try_from(size_u32) else {
            return wav_not_qualified("riff_chunk_size_address_overflow", data);
        };
        let body = pos + 8;
        let Some(end) = body.checked_add(size) else {
            return wav_not_qualified("riff_chunk_end_overflow", data);
        };
        if end > data.len() {
            return wav_not_qualified("riff_chunk_truncated", data);
        }

        if id == b"fmt " && size >= 16 {
            audio_format = read_u16_le(&data[body..body + 2]);
            channels = read_u16_le(&data[body + 2..body + 4]);
            sample_rate = read_u32_le(&data[body + 4..body + 8]);
            block_align = read_u16_le(&data[body + 12..body + 14]);
            bits_per_sample = read_u16_le(&data[body + 14..body + 16]);
        } else if id == b"data" {
            data_range = Some((body, end));
        }

        let Some(next) = end.checked_add(size % 2) else {
            return wav_not_qualified("riff_padding_overflow", data);
        };
        if next > data.len() {
            return wav_not_qualified("riff_padding_truncated", data);
        }
        pos = next;
    }

    let (
        Some(audio_format),
        Some(channels),
        Some(sample_rate),
        Some(bits_per_sample),
        Some(block_align),
        Some((data_start, data_end)),
    ) = (
        audio_format,
        channels,
        sample_rate,
        bits_per_sample,
        block_align,
        data_range,
    )
    else {
        return wav_not_qualified("wav_required_chunks_missing", data);
    };

    if channels == 0 || sample_rate == 0 || bits_per_sample == 0 || block_align == 0 {
        return wav_not_qualified("wav_format_zero_value", data);
    }

    let bytes_per_sample = usize::from(bits_per_sample).div_ceil(8);
    let minimum_block_align = bytes_per_sample.saturating_mul(usize::from(channels));
    if bytes_per_sample == 0 || usize::from(block_align) < minimum_block_align {
        return wav_not_qualified("wav_block_align_incompatible", data);
    }

    let payload = &data[data_start..data_end];
    let frame_bytes = usize::from(block_align);
    let frame_count = payload.len() / frame_bytes;
    let trailing_partial_data_bytes = payload.len() % frame_bytes;
    let Ok(original_frame_count) = u64::try_from(frame_count) else {
        return wav_not_qualified("wav_frame_count_overflow", data);
    };

    let mut decoded_hash = Fnv64::new();
    let mut first_nonzero_frame = None;
    let mut last_nonzero_frame = None;
    let mut decoded_sample_values = 0u64;

    for (frame_index, frame) in payload.chunks_exact(frame_bytes).enumerate() {
        let Ok(frame_index) = u64::try_from(frame_index) else {
            return wav_not_qualified("wav_frame_index_overflow", data);
        };
        let mut frame_nonzero = false;
        for channel_index in 0..usize::from(channels) {
            let offset = channel_index.saturating_mul(bytes_per_sample);
            let Some(sample_end) = offset.checked_add(bytes_per_sample) else {
                return wav_not_qualified("wav_sample_offset_overflow", data);
            };
            if sample_end > frame.len() {
                return wav_not_qualified("wav_sample_outside_frame", data);
            }
            let Some(value) = decode_wav_sample(
                &frame[offset..sample_end],
                bits_per_sample,
                audio_format,
            ) else {
                return wav_not_qualified("wav_sample_format_unsupported", data);
            };
            decoded_hash.update(&value.to_bits().to_le_bytes());
            decoded_sample_values = decoded_sample_values.saturating_add(1);
            if value != 0.0 {
                frame_nonzero = true;
            }
        }
        if frame_nonzero {
            let index = OriginalFrameIndex(frame_index);
            first_nonzero_frame.get_or_insert(index);
            last_nonzero_frame = Some(index);
        }
    }

    WavObservationAudit {
        qualified: trailing_partial_data_bytes == 0,
        source_file_sha256: sha256::hex(data),
        decoded_signal_fnv64: format!("fnv64:{:016x}", decoded_hash.finish()),
        audio_format,
        sample_rate,
        channels,
        bits_per_sample,
        block_align,
        original_frame_count,
        first_nonzero_frame,
        last_nonzero_frame,
        decoded_sample_values,
        trailing_partial_data_bytes,
        reason: if trailing_partial_data_bytes == 0 {
            "wav_observation_qualified"
        } else {
            "wav_data_partial_frame"
        },
    }
}

fn wav_not_qualified(reason: &'static str, file_bytes: &[u8]) -> WavObservationAudit {
    WavObservationAudit {
        qualified: false,
        source_file_sha256: sha256::hex(file_bytes),
        decoded_signal_fnv64: "fnv64:unavailable".to_string(),
        audio_format: 0,
        sample_rate: 0,
        channels: 0,
        bits_per_sample: 0,
        block_align: 0,
        original_frame_count: 0,
        first_nonzero_frame: None,
        last_nonzero_frame: None,
        decoded_sample_values: 0,
        trailing_partial_data_bytes: 0,
        reason,
    }
}

pub(crate) fn inspect_frozen_legacy_source_oracle(
    path: &Path,
) -> FrozenLegacySourceOracleAudit {
    let decoded = match read_standard_midi(path) {
        Ok(decoded) => decoded,
        Err(_) => return legacy_not_qualified("midi_file_unreadable", 0),
    };
    let source_ppq = decoded.source_ppq.max(1);
    let projection = project_source_events(&decoded.notes);

    FrozenLegacySourceOracleAudit {
        qualified: !decoded.notes.is_empty(),
        source_ppq,
        canonical_divisions: CANONICAL_DIVISIONS,
        event_count: projection.event_count,
        duplicate_pitch_onset_events: projection.duplicate_pitch_onset_events,
        terminal_extent_ticks: projection.terminal_extent_ticks,
        event_fingerprint_sha256: projection.event_fingerprint_sha256,
        reason: if decoded.notes.is_empty() {
            "midi_no_source_events"
        } else {
            "frozen_legacy_source_oracle_qualified"
        },
    }
}

fn legacy_not_qualified(reason: &'static str, source_ppq: i64) -> FrozenLegacySourceOracleAudit {
    FrozenLegacySourceOracleAudit {
        qualified: false,
        source_ppq,
        canonical_divisions: CANONICAL_DIVISIONS,
        event_count: 0,
        duplicate_pitch_onset_events: 0,
        terminal_extent_ticks: 0,
        event_fingerprint_sha256: "unavailable".to_string(),
        reason,
    }
}

fn read_u16_le(bytes: &[u8]) -> Option<u16> {
    (bytes.len() >= 2).then(|| u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    (bytes.len() >= 4).then(|| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

struct Fnv64(u64);

impl Fnv64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    fn new() -> Self {
        Self(Self::OFFSET)
    }

    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(Self::PRIME);
        }
    }

    fn finish(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalLegacyPitchEventAudit {
    pub(crate) source_ordinal: usize,
    pub(crate) onset_ticks: i64,
    pub(crate) midi: i32,
}

pub(crate) fn load_canonical_legacy_pitch_events_for_score_title(
    score_title: Option<&str>,
) -> Result<Vec<CanonicalLegacyPitchEventAudit>, &'static str> {
    let title = score_title.ok_or("score_title_missing")?.trim();
    if title.is_empty() {
        return Err("score_title_empty");
    }
    let current = std::env::current_dir().map_err(|_| "current_dir_unavailable")?;
    let path = current.join(format!("{title}.mid"));
    let decoded = read_standard_midi(&path).map_err(|_| "midi_file_unreadable")?;
    Ok(decoded
        .notes
        .into_iter()
        .map(|event| CanonicalLegacyPitchEventAudit {
            source_ordinal: event.source_ordinal,
            onset_ticks: event.pos,
            midi: event.midi,
        })
        .collect())
}

pub(crate) fn load_canonical_legacy_unique_onsets_for_score_title(
    score_title: Option<&str>,
) -> Result<Vec<i64>, &'static str> {
    let title = score_title.ok_or("score_title_missing")?.trim();
    if title.is_empty() {
        return Err("score_title_empty");
    }
    let current = std::env::current_dir().map_err(|_| "current_dir_unavailable")?;
    let path = current.join(format!("{title}.mid"));
    let decoded = read_standard_midi(&path).map_err(|_| "midi_file_unreadable")?;
    let mut onsets: Vec<i64> = decoded.notes.into_iter().map(|event| event.pos).collect();
    onsets.sort_unstable();
    onsets.dedup();
    if onsets.is_empty() {
        return Err("midi_no_source_onsets");
    }
    Ok(onsets)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pcm16_stereo_wav(frames: &[[i16; 2]]) -> Vec<u8> {
        let data_size = frames.len() * 4;
        let mut out = Vec::new();
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&(36u32 + u32::try_from(data_size).unwrap()).to_le_bytes());
        out.extend_from_slice(b"WAVEfmt ");
        out.extend_from_slice(&16u32.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&2u16.to_le_bytes());
        out.extend_from_slice(&44_100u32.to_le_bytes());
        out.extend_from_slice(&(44_100u32 * 4).to_le_bytes());
        out.extend_from_slice(&4u16.to_le_bytes());
        out.extend_from_slice(&16u16.to_le_bytes());
        out.extend_from_slice(b"data");
        out.extend_from_slice(&u32::try_from(data_size).unwrap().to_le_bytes());
        for [left, right] in frames {
            out.extend_from_slice(&left.to_le_bytes());
            out.extend_from_slice(&right.to_le_bytes());
        }
        out
    }

    #[test]
    fn wav_observation_preserves_original_frame_domain_and_channels() {
        let bytes = pcm16_stereo_wav(&[[0, 0], [1000, 0], [0, -1000], [0, 0]]);
        let audit = inspect_wav_bytes(&bytes);
        assert!(audit.qualified);
        assert_eq!(audit.sample_rate, 44_100);
        assert_eq!(audit.channels, 2);
        assert_eq!(audit.original_frame_count, 4);
        assert_eq!(audit.decoded_sample_values, 8);
        assert_eq!(audit.first_nonzero_frame, Some(OriginalFrameIndex(1)));
        assert_eq!(audit.last_nonzero_frame, Some(OriginalFrameIndex(2)));
    }

    #[test]
    fn legacy_event_projection_preserves_multiplicity() {
        let events = vec![
            CanonicalSourceEvent {
                source_ordinal: 0,
                pos: 96,
                len: 24,
                midi: 60,
            },
            CanonicalSourceEvent {
                source_ordinal: 1,
                pos: 96,
                len: 24,
                midi: 60,
            },
        ];
        let projection = project_source_events(&events);
        assert_eq!(projection.duplicate_pitch_onset_events, 1);
        assert_eq!(projection.terminal_extent_ticks, 120);
        assert_ne!(projection.event_fingerprint_sha256, "unavailable");
    }
}
