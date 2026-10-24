//! Audit-only WAV signal decoding shared by current audio evidence probes.
//!
//! This module decodes RIFF/WAVE sample data only. It owns no FLP legacy oracle,
//! pitch, metric, source-event, hand/staff, phrase or writer authority.

use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub(super) struct DecodedWavSignalAudit {
    pub(super) sample_rate: u32,
    pub(super) channels: u16,
    pub(super) original_frame_count: u64,
    pub(super) interleaved_samples: Vec<f32>,
}

pub(super) fn load_decoded_wav_signal_for_score_title(
    score_title: Option<&str>,
) -> Result<DecodedWavSignalAudit, &'static str> {
    let title = score_title.ok_or("score_title_missing")?.trim();
    if title.is_empty() {
        return Err("score_title_empty");
    }
    let current = std::env::current_dir().map_err(|_| "current_dir_unavailable")?;
    let path = current.join(format!("{title}.wav"));
    load_decoded_wav_signal(&path)
}

fn load_decoded_wav_signal(path: &Path) -> Result<DecodedWavSignalAudit, &'static str> {
    let data = fs::read(path).map_err(|_| "wav_file_unreadable")?;
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return Err("riff_wave_header_missing");
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
        let size_u32 = read_u32_le(&data[pos + 4..pos + 8]).ok_or("riff_chunk_size_invalid")?;
        let size = usize::try_from(size_u32).map_err(|_| "riff_chunk_size_address_overflow")?;
        let body = pos.checked_add(8).ok_or("riff_chunk_body_overflow")?;
        let end = body.checked_add(size).ok_or("riff_chunk_end_overflow")?;
        if end > data.len() {
            return Err("riff_chunk_truncated");
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
        pos = end
            .checked_add(size % 2)
            .ok_or("riff_padding_overflow")?;
        if pos > data.len() {
            return Err("riff_padding_truncated");
        }
    }

    let (audio_format, channels, sample_rate, bits_per_sample, block_align, data_range) = (
        audio_format.ok_or("wav_format_missing")?,
        channels.ok_or("wav_channels_missing")?,
        sample_rate.ok_or("wav_sample_rate_missing")?,
        bits_per_sample.ok_or("wav_bits_missing")?,
        block_align.ok_or("wav_block_align_missing")?,
        data_range.ok_or("wav_data_missing")?,
    );
    if channels == 0 || sample_rate == 0 || bits_per_sample == 0 || block_align == 0 {
        return Err("wav_format_zero_value");
    }

    let bytes_per_sample = usize::from(bits_per_sample).div_ceil(8);
    let frame_bytes = usize::from(block_align);
    let minimum_block_align = bytes_per_sample.saturating_mul(usize::from(channels));
    if bytes_per_sample == 0 || frame_bytes < minimum_block_align {
        return Err("wav_block_align_incompatible");
    }

    let payload = &data[data_range.0..data_range.1];
    if payload.len() % frame_bytes != 0 {
        return Err("wav_data_partial_frame");
    }
    let frame_count = payload.len() / frame_bytes;
    let sample_capacity = frame_count
        .checked_mul(usize::from(channels))
        .ok_or("wav_sample_capacity_overflow")?;
    let mut interleaved_samples = Vec::with_capacity(sample_capacity);
    for frame in payload.chunks_exact(frame_bytes) {
        for channel_index in 0..usize::from(channels) {
            let offset = channel_index
                .checked_mul(bytes_per_sample)
                .ok_or("wav_sample_offset_overflow")?;
            let sample_end = offset
                .checked_add(bytes_per_sample)
                .ok_or("wav_sample_offset_overflow")?;
            if sample_end > frame.len() {
                return Err("wav_sample_outside_frame");
            }
            let value = decode_wav_sample(
                &frame[offset..sample_end],
                bits_per_sample,
                audio_format,
            )
            .ok_or("wav_sample_format_unsupported")?;
            interleaved_samples.push(value);
        }
    }
    let original_frame_count = u64::try_from(frame_count).map_err(|_| "wav_frame_count_overflow")?;
    Ok(DecodedWavSignalAudit {
        sample_rate,
        channels,
        original_frame_count,
        interleaved_samples,
    })
}

pub(super) fn decode_wav_sample(bytes: &[u8], bits: u16, format: u16) -> Option<f32> {
    if format == 3 && bits == 32 && bytes.len() >= 4 {
        return Some(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]).clamp(-1.0, 1.0));
    }
    if format != 1 {
        return None;
    }
    match bits {
        8 if !bytes.is_empty() => Some((f32::from(bytes[0]) - 128.0) / 128.0),
        16 if bytes.len() >= 2 => {
            let value = f32::from(i16::from_le_bytes([bytes[0], bytes[1]]));
            Some((value / 32768.0).clamp(-1.0, 1.0))
        }
        24 if bytes.len() >= 3 => {
            let mut value = i32::from(bytes[0])
                | (i32::from(bytes[1]) << 8)
                | (i32::from(bytes[2]) << 16);
            if value & 0x0080_0000 != 0 {
                value |= !0x00ff_ffff;
            }
            Some(((value as f32) / 8_388_608.0).clamp(-1.0, 1.0))
        }
        32 if bytes.len() >= 4 => {
            let value = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f32;
            Some((value / 2_147_483_648.0).clamp(-1.0, 1.0))
        }
        _ => None,
    }
}

fn read_u16_le(bytes: &[u8]) -> Option<u16> {
    (bytes.len() >= 2).then(|| u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    (bytes.len() >= 4).then(|| u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

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
    fn decoded_signal_preserves_frame_domain_channels_and_samples() {
        let bytes = pcm16_stereo_wav(&[[0, 0], [1000, -1000], [0, 0]]);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("etuschlopf_wav_signal_decode_{nonce}.wav"));
        fs::write(&path, bytes).expect("write wav fixture");
        let signal = load_decoded_wav_signal(&path).expect("decode wav fixture");
        let _ = fs::remove_file(path);

        assert_eq!(signal.sample_rate, 44_100);
        assert_eq!(signal.channels, 2);
        assert_eq!(signal.original_frame_count, 3);
        assert_eq!(signal.interleaved_samples.len(), 6);
        assert_eq!(signal.interleaved_samples[0], 0.0);
        assert!(signal.interleaved_samples[2] > 0.0);
        assert!(signal.interleaved_samples[3] < 0.0);
    }
}
