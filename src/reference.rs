// v0.10.725 FoundationIoRealModuleSplitNoBehaviorChange.
// Real crate-root module split from the former mechanical IO grouping.
// Preserves original module path `reference` and behavior; no wrapper/include shell.
// --- mechanical module boundary: former src/reference.rs ---
use std::fs;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone, Default)]
pub struct ReferenceReport {
    pub wav: Option<WavReference>,
}
#[derive(Debug, Clone)]
pub struct WavReference {
    pub path: PathBuf,
    pub seconds: Option<f64>,
}
impl ReferenceReport {
    pub fn guard_active(&self) -> bool {
        self.wav.is_some()
    }
}
pub fn inspect_reference_files(
    input: &Path,
    explicit_wav: Option<&Path>,
) -> ReferenceReport {
    let dirs = candidate_dirs(input);
    let wav_path = resolve_reference_file(&dirs, explicit_wav, "wav");
    ReferenceReport {
        wav: wav_path.map(|path| inspect_wav(path.as_path())),
    }
}
fn candidate_dirs(input: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(parent) = input.parent()
        && !parent.as_os_str().is_empty()
    {
        dirs.push(parent.to_path_buf());
    }
    if let Ok(current) = std::env::current_dir()
        && !dirs.iter().any(|dir| same_pathish(dir, &current))
    {
        dirs.push(current);
    }
    dirs
}
fn same_pathish(a: &Path, b: &Path) -> bool {
    let a_text = a.to_string_lossy().replace('\\', "/").to_ascii_lowercase();
    let b_text = b.to_string_lossy().replace('\\', "/").to_ascii_lowercase();
    a_text == b_text
}
fn resolve_reference_file(
    dirs: &[PathBuf],
    explicit: Option<&Path>,
    extension: &str,
) -> Option<PathBuf> {
    if let Some(path) = explicit {
        return path.exists().then(|| path.to_path_buf());
    }
    let mut found = Vec::new();
    for dir in dirs {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
            {
                found.push(path);
            }
        }
    }
    found.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    found.dedup_by(|a, b| same_pathish(a.as_path(), b.as_path()));
    (found.len() == 1).then(|| found.remove(0))
}
fn inspect_wav(path: &Path) -> WavReference {
    let data = fs::read(path).unwrap_or_default();
    let mut sample_rate = None;
    let mut channels = None;
    let mut bits_per_sample = None;
    let mut data_bytes = None;
    if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WAVE" {
        let mut pos = 12usize;
        while pos + 8 <= data.len() {
            let id = &data[pos..pos + 4];
            let size = read_u32_le(&data[pos + 4..pos + 8]).unwrap_or(0) as usize;
            let body = pos + 8;
            if body + size > data.len() {
                break;
            }
            if id == b"fmt " && size >= 16 {
                channels = read_u16_le(&data[body + 2..body + 4]);
                sample_rate = read_u32_le(&data[body + 4..body + 8]);
                bits_per_sample = read_u16_le(&data[body + 14..body + 16]);
            } else if id == b"data" {
                data_bytes = Some(size as u64);
            }
            pos = body + size + (size % 2);
        }
    }
    let seconds = match (data_bytes, sample_rate, channels, bits_per_sample) {
        (Some(bytes), Some(rate), Some(ch), Some(bits)) if rate > 0 && ch > 0 && bits > 0 => {
            let bytes_per_second = rate as f64 * ch as f64 * bits as f64 / 8.0;
            if bytes_per_second > 0.0 {
                Some(bytes as f64 / bytes_per_second)
            } else {
                None
            }
        }
        _ => None,
    };
    WavReference { path: path.to_path_buf(), seconds }
}
fn read_u16_le(bytes: &[u8]) -> Option<u16> {
    if bytes.len() < 2 {
        return None;
    }
    Some(u16::from_le_bytes([bytes[0], bytes[1]]))
}
fn read_u32_le(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < 4 {
        return None;
    }
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}
use crate::arrangement::ArrangementStats;
use crate::model::Score;
#[derive(Debug, Clone)]
pub struct ReferenceSignature {
    pub active: bool,
    pub wav_authoritative: bool,
    pub measures: usize,
    pub estimated_bpm: Option<f64>,
}
#[derive(Debug, Clone)]
pub struct FidelityCheck {
    pub retained_note_percent: Option<f64>,
    pub verdict: FidelityVerdict,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FidelityVerdict {
    Inactive,
    Plausible,
    Check,
    Critical,
}
pub fn build_reference_signature(reference: &ReferenceReport, score: &Score) -> ReferenceSignature {
    let (measures, total_quarters) = score_quarter_signature(score);
    let wav_seconds = reference.wav.as_ref().and_then(|wav| wav.seconds);
    let estimated_bpm = match (wav_seconds, total_quarters > 0.0) {
        (Some(seconds), true) if seconds > 0.0 => Some(total_quarters * 60.0 / seconds),
        _ => None,
    };
    ReferenceSignature {
        active: reference.guard_active(),
        wav_authoritative: reference.wav.is_some(),
        measures,
        estimated_bpm,
    }
}
pub fn check_reference_fidelity(
    reference: &ReferenceReport,
    signature: &ReferenceSignature,
    arrangement: &ArrangementStats,
) -> FidelityCheck {
    if !reference.guard_active() || !signature.active {
        return FidelityCheck {
            retained_note_percent: None,
            verdict: FidelityVerdict::Inactive,
        };
    }
    let retained_note_percent = if arrangement.notes_seen > 0 {
        let kept = arrangement.notes_seen.saturating_sub(arrangement.notes_hidden) as f64;
        Some(kept * 100.0 / arrangement.notes_seen as f64)
    } else {
        None
    };
    let verdict = if !signature.wav_authoritative {
        FidelityVerdict::Check
    } else {
        match retained_note_percent {
            Some(value) if value < 55.0 => FidelityVerdict::Critical,
            Some(value) if value < 68.0 => FidelityVerdict::Check,
            Some(_) => FidelityVerdict::Plausible,
            None => FidelityVerdict::Check,
        }
    };
    FidelityCheck {
        retained_note_percent,
        verdict,
    }
}
fn score_quarter_signature(score: &Score) -> (usize, f64) {
    let Some(part) = score.parts.first() else {
        return (0, 0.0);
    };
    let mut quarters = 0.0;
    for measure in &part.measures {
        let divisions = measure.divisions.max(1) as f64;
        let expected_divisions = measure.expected.numerator as f64 / measure.expected.denominator as f64;
        quarters += expected_divisions / divisions;
    }
    (part.measures.len(), quarters)
}
