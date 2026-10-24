use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use tract_onnx::prelude::*;

mod sha256 {
    use sha2::{Digest, Sha256};

    pub(crate) fn hex(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let digest = hasher.finalize();
        let mut out = String::with_capacity(digest.len() * 2);
        for byte in digest {
            use std::fmt::Write as _;
            let _ = write!(out, "{byte:02x}");
        }
        out
    }
}

#[path = "shared_source_event.rs"]
mod source_event;

#[path = "shared_midi.rs"]
mod midi;

const CONTRACT_SCHEMA: &str = "wav_pitch_sensor_qualification_v1";
const SENSOR_IDENTITY: &str = "SpotifyBasicPitchOnnxRust";
const SENSOR_VERSION: &str = "ICASSP2022-ONNX";
const FEATURE_SCHEMA: &str = "basic_pitch_icassp2022_onnx_rust_note_event_original_frame_v2";
const DERIVED_VIEW_IDENTITY: &str = "stereo_pcm24_44100_to_mono_f32_fir127_decimate2_rust_v1";
const ORIGINAL_FRAME_MAPPING: &str = "basic_pitch_kept_window_frame_to_original_44100_frame_v1";
const MODEL_SHA256: &str = "2c3c1d144bfa61ad236e92e169c13535c880469a12a047d4e73451f2c059a0ec";
const MODEL_BYTES: usize = 230_444;
const MODEL_URL_PRIMARY: &str = "https://raw.githubusercontent.com/spotify/basic-pitch/main/basic_pitch/saved_models/icassp_2022/nmp.onnx";
const MODEL_URL_FALLBACK: &str = "https://huggingface.co/AEmotionStudio/basic-pitch-onnx-models/resolve/main/nmp.onnx?download=true";
const NOTE_OUTPUT_NODE: &str = "StatefulPartitionedCall:1";
const ONSET_OUTPUT_NODE: &str = "StatefulPartitionedCall:2";
const CONTOUR_OUTPUT_NODE: &str = "StatefulPartitionedCall:0";

const SOURCE_SAMPLE_RATE: u64 = 44_100;
const MODEL_SAMPLE_RATE: usize = 22_050;
const AUDIO_N_SAMPLES: usize = 43_844;
const FFT_HOP: usize = 256;
const ANNOT_N_FRAMES: usize = 172;
const OVERLAPPING_FRAMES: usize = 30;
const OVERLAP_SAMPLES: usize = OVERLAPPING_FRAMES * FFT_HOP;
const PREFIX_SAMPLES: usize = OVERLAP_SAMPLES / 2;
const HOP_SIZE: usize = AUDIO_N_SAMPLES - OVERLAP_SAMPLES;
const TRIM_FRAMES: usize = OVERLAPPING_FRAMES / 2;
const KEPT_FRAMES: usize = ANNOT_N_FRAMES - OVERLAPPING_FRAMES;
const PITCH_BINS: usize = 88;
const MIDI_OFFSET: u8 = 21;
const ONSET_THRESHOLD: f32 = 0.5;
const FRAME_THRESHOLD: f32 = 0.3;
const MIN_NOTE_LEN: usize = 11;
const ENERGY_TOLERANCE: usize = 11;
const CANONICAL_DIVISIONS: u64 = 96;
const HARMONIC_INTERVALS: [u8; 7] = [12, 19, 24, 28, 31, 34, 36];

#[derive(Debug)]
struct Args {
    root: PathBuf,
    project: String,
    output: PathBuf,
    runtime: PathBuf,
    quarter_frames: u64,
    expected_wav_sha256: String,
    cargo_bootstrap_mode: String,
}

#[derive(Debug, Clone)]
struct OracleEvent {
    ordinal: usize,
    pos: u64,
    length: u64,
    midi: u8,
}

#[derive(Debug, Clone)]
struct SensorEvent {
    ordinal: usize,
    midi: u8,
    start_idx: usize,
    end_idx: usize,
    amplitude: f32,
    onset_frame: u64,
    offset_frame: u64,
}

#[derive(Debug)]
struct Activations {
    frames: Vec<f32>,
    onsets: Vec<f32>,
    frame_source: Vec<u64>,
    n_frames: usize,
}

#[derive(Debug, Clone, Copy)]
struct MatchPair {
    oracle: usize,
    sensor: usize,
}

#[derive(Debug, Clone, Copy)]
struct Prf {
    precision: f64,
    recall: f64,
    f1: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapCell {
    amp_bits: u32,
    flat: usize,
}

impl HeapCell {
    fn new(amp: f32, flat: usize) -> Self {
        Self {
            amp_bits: amp.to_bits(),
            flat,
        }
    }

    fn amp(self) -> f32 {
        f32::from_bits(self.amp_bits)
    }
}

impl Ord for HeapCell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amp()
            .total_cmp(&other.amp())
            .then_with(|| other.flat.cmp(&self.flat))
    }
}

impl PartialOrd for HeapCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("[WavPitchSensorRust] FEHLER: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = parse_args()?;
    fs::create_dir_all(&args.output)
        .with_context(|| format!("Output-Verzeichnis nicht anlegbar: {}", args.output.display()))?;
    fs::create_dir_all(&args.runtime)
        .with_context(|| format!("Runtime-Verzeichnis nicht anlegbar: {}", args.runtime.display()))?;

    let wav = args.root.join(format!("{}.wav", args.project));
    let midi_path = args.root.join(format!("{}.mid", args.project));
    if !wav.is_file() {
        bail!("WAV fehlt: {}", wav.display());
    }

    let wav_bytes = fs::read(&wav).with_context(|| format!("WAV nicht lesbar: {}", wav.display()))?;
    let wav_sha = sha256_bytes(&wav_bytes);
    if wav_sha != args.expected_wav_sha256 {
        bail!(
            "WAV-Fingerprint stimmt nicht: erwartet {}, erhalten {}",
            args.expected_wav_sha256,
            wav_sha
        );
    }

    let model_dir = args.runtime.join("model");
    fs::create_dir_all(&model_dir)?;
    let model_path = model_dir.join("basic_pitch_icassp2022_nmp.onnx");
    let model_downloaded = ensure_pinned_model(&model_path)?;
    let package_fingerprint = package_fingerprint(&args.root)?;

    // STAGE 1: Sensor-only. No MIDI source-oracle content is opened before the freeze below.
    let mono_44100 = decode_pcm24_stereo_44100(&wav_bytes)?;
    let source_frame_count = mono_44100.len() as u64;
    let mono_22050 = decimate_2_fir127(&mono_44100);
    if mono_22050.len() < MODEL_SAMPLE_RATE / 2 {
        bail!("WAV ist für den Sensor unerwartet kurz");
    }

    eprintln!(
        "[WavPitchSensorRust] Basic Pitch ONNX · Rust/tract · {} · {} Frames",
        args.project, source_frame_count
    );
    let activations = infer_basic_pitch(&model_path, &mono_22050, source_frame_count)?;
    let mut events = decode_polyphonic_events(&activations)?;
    normalize_event_order(&mut events);
    let projection = event_projection_bytes(&events);
    let projection_sha = sha256_bytes(&projection);
    fs::write(args.output.join("sensor_events.frozen.txt"), &projection)?;

    let freeze_manifest = format!(
        concat!(
            "contract_schema={}\n",
            "authority=evidence_only\n",
            "sensor_identity={}\n",
            "sensor_version={}\n",
            "package_fingerprint_sha256={}\n",
            "model_fingerprint_sha256={}\n",
            "feature_schema={}\n",
            "source_wav_sha256={}\n",
            "derived_view_identity={}\n",
            "original_frame_mapping={}\n",
            "frozen_event_projection_sha256={}\n",
            "frozen_event_count={}\n",
            "midi_authority=none_evaluation_only_post_freeze\n",
            "canonical_source_event_promotion=none\n"
        ),
        CONTRACT_SCHEMA,
        SENSOR_IDENTITY,
        SENSOR_VERSION,
        package_fingerprint,
        MODEL_SHA256,
        FEATURE_SCHEMA,
        wav_sha,
        DERIVED_VIEW_IDENTITY,
        ORIGINAL_FRAME_MAPPING,
        projection_sha,
        events.len(),
    );
    let freeze_sha = sha256_bytes(freeze_manifest.as_bytes());
    fs::write(args.output.join("sensor_freeze_manifest.txt"), freeze_manifest.as_bytes())?;
    fs::write(
        args.output.join("SENSOR_OUTPUT_FROZEN.sha256"),
        format!("{freeze_sha}\n").as_bytes(),
    )?;

    let deterministic_repeat = if args.project == "Etüde Nr. 6" {
        let repeat = infer_basic_pitch(&model_path, &mono_22050, source_frame_count)?;
        let mut repeat_events = decode_polyphonic_events(&repeat)?;
        normalize_event_order(&mut repeat_events);
        if event_projection_bytes(&repeat_events) == projection {
            "yes"
        } else {
            "no"
        }
    } else {
        "not_probed"
    };

    // STAGE 2: Only after SENSOR_OUTPUT_FROZEN may the FrozenLegacySourceOracle be read or
    // even required as an input. Its MIDI source must never influence the sensor projection above.
    if !midi_path.is_file() {
        bail!(
            "MIDI-Oracle fehlt nach Sensor-Freeze: {}",
            midi_path.display()
        );
    }
    let decoded_midi = midi::read_standard_midi(&midi_path)?;
    eprintln!(
        "[WavPitchSensorRust] MIDI-Oracle nach Freeze · Format {} · Tracks {} · PPQ {} · Tempo-Metadaten {} · Taktarten {}",
        decoded_midi.format,
        decoded_midi.tracks,
        decoded_midi.source_ppq,
        decoded_midi.tempo_microseconds_per_quarter.len(),
        decoded_midi.time_signatures.len(),
    );
    let oracle = decoded_midi
        .notes
        .into_iter()
        .map(|event| {
            Ok(OracleEvent {
                ordinal: event.source_ordinal,
                pos: u64::try_from(event.pos).context("MIDI-Oracle-Position ist negativ")?,
                length: u64::try_from(event.len).context("MIDI-Oracle-Dauer ist negativ")?,
                midi: u8::try_from(event.midi).context("MIDI-Oracle-Ton liegt außerhalb u8")?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let m80 = evaluate_matches(&oracle, &events, args.quarter_frames, 80);
    let chord_recall = chord_tone_recall(&oracle, &events, args.quarter_frames, 80);
    let rearticulation = rearticulation_recall(&oracle, &events, args.quarter_frames, 80);
    let multiplicity = multiplicity_recall(&oracle, &events, args.quarter_frames, 80);
    let bass = subset_prf(&oracle, &events, args.quarter_frames, 80, |m| m <= 47);
    let high = subset_prf(&oracle, &events, args.quarter_frames, 80, |m| m >= 84);
    let (false_sensor_events, harmonic_false) =
        false_event_harmonic_count(&oracle, &events, args.quarter_frames, 80);

    let setup_network_used = if model_downloaded {
        format!("model_bootstrap_yes;cargo_{}", args.cargo_bootstrap_mode)
    } else {
        format!("model_bootstrap_no;cargo_{}", args.cargo_bootstrap_mode)
    };
    let windows_offline_runtime = if deterministic_repeat == "no" {
        "not_qualified_nondeterministic"
    } else {
        "qualified_rust_onnx_after_pinned_bootstrap"
    };

    let kv = format!(
        concat!(
            "schema=etuschlopf_wav_pitch_external_sensor_piece_benchmark_v2\n",
            "status=observed\n",
            "project={}\n",
            "contract_schema={}\n",
            "authority=evidence_only\n",
            "sensor_identity={}\n",
            "sensor_version={}\n",
            "package_fingerprint_sha256={}\n",
            "model_fingerprint_sha256={}\n",
            "feature_schema={}\n",
            "source_wav_sha256={}\n",
            "freeze_sha256={}\n",
            "sensor_events={}\n",
            "oracle_events={}\n",
            "precision_80_permille={}\n",
            "recall_80_permille={}\n",
            "f1_80_permille={}\n",
            "chord_tone_recall_80_permille={}\n",
            "rearticulation_both_80_permille={}\n",
            "multiplicity_recall_80_permille={}\n",
            "bass_recall_80_permille={}\n",
            "high_recall_80_permille={}\n",
            "false_sensor_events_80={}\n",
            "harmonic_explainable_false_80={}\n",
            "deterministic_repeat={}\n",
            "windows_offline_runtime={}\n",
            "setup_network_used={}\n",
            "original_frame_mapping={}\n",
            "midi_authority=none_evaluation_only_post_freeze\n",
            "canonical_source_event_promotion=none\n"
        ),
        args.project,
        CONTRACT_SCHEMA,
        SENSOR_IDENTITY,
        SENSOR_VERSION,
        package_fingerprint,
        MODEL_SHA256,
        FEATURE_SCHEMA,
        wav_sha,
        freeze_sha,
        events.len(),
        oracle.len(),
        permille(m80.precision),
        permille(m80.recall),
        permille(m80.f1),
        permille(chord_recall),
        permille(rearticulation),
        permille(multiplicity),
        permille(bass.recall),
        permille(high.recall),
        false_sensor_events,
        harmonic_false,
        deterministic_repeat,
        windows_offline_runtime,
        setup_network_used,
        ORIGINAL_FRAME_MAPPING,
    );
    fs::write(args.output.join("audit_kv.txt"), kv.as_bytes())?;
    let latest = args
        .output
        .parent()
        .and_then(Path::parent)
        .context("Audit-Output besitzt keinen Runtime-Parent")?
        .join("latest_audit_kv.txt");
    fs::write(&latest, kv.as_bytes())?;

    let summary = format!(
        concat!(
            "ETUSCHLOPF WAV PITCH EXTERNAL SENSOR · RUST/ONNX\n",
            "================================================\n",
            "Project: {}\n",
            "Sensor: {} {}\n",
            "Authority: evidence_only; MIDI evaluation-only post-freeze; no source-event promotion\n",
            "SensorOutputFrozen: {}\n",
            "SensorEvents: {}\n",
            "OracleEvents: {}\n",
            "PitchOnset80 Precision: {:.2}%\n",
            "PitchOnset80 Recall: {:.2}%\n",
            "PitchOnset80 F1: {:.2}%\n",
            "ChordToneRecall80: {:.2}%\n",
            "SamePitchRearticulationBoth80: {:.2}%\n",
            "MultiplicityRecall80: {:.2}%\n",
            "BassRecall80: {:.2}%\n",
            "HighRecall80: {:.2}%\n",
            "DeterministicRepeat: {}\n"
        ),
        args.project,
        SENSOR_IDENTITY,
        SENSOR_VERSION,
        freeze_sha,
        events.len(),
        oracle.len(),
        m80.precision * 100.0,
        m80.recall * 100.0,
        m80.f1 * 100.0,
        chord_recall * 100.0,
        rearticulation * 100.0,
        multiplicity * 100.0,
        bass.recall * 100.0,
        high.recall * 100.0,
        deterministic_repeat,
    );
    fs::write(args.output.join("SUMMARY.txt"), summary.as_bytes())?;
    eprintln!("{summary}");
    Ok(())
}

fn parse_args() -> Result<Args> {
    let mut values = BTreeMap::<String, OsString>::new();
    let mut it = env::args_os().skip(1);
    while let Some(raw) = it.next() {
        let key = raw.to_string_lossy().to_string();
        if !key.starts_with("--") {
            bail!("Unerwartetes Argument: {key}");
        }
        let value = it.next().with_context(|| format!("Wert fehlt für {key}"))?;
        values.insert(key, value);
    }
    let path = |key: &str| -> Result<PathBuf> {
        Ok(PathBuf::from(
            values.get(key).with_context(|| format!("Argument fehlt: {key}"))?,
        ))
    };
    let text = |key: &str| -> Result<String> {
        Ok(values
            .get(key)
            .with_context(|| format!("Argument fehlt: {key}"))?
            .to_string_lossy()
            .to_string())
    };
    let quarter_frames = text("--quarter-frames")?
        .parse::<u64>()
        .context("--quarter-frames ist keine Zahl")?;
    if quarter_frames == 0 {
        bail!("--quarter-frames darf nicht 0 sein");
    }
    let expected_wav_sha256 = text("--expected-wav-sha256")?.to_ascii_lowercase();
    if expected_wav_sha256.len() != 64
        || !expected_wav_sha256.bytes().all(|b| b.is_ascii_hexdigit())
    {
        bail!("--expected-wav-sha256 ist ungültig");
    }
    Ok(Args {
        root: path("--root")?,
        project: text("--project")?,
        output: path("--output")?,
        runtime: path("--runtime")?,
        quarter_frames,
        expected_wav_sha256,
        cargo_bootstrap_mode: text("--cargo-bootstrap-mode")?,
    })
}

fn ensure_pinned_model(path: &Path) -> Result<bool> {
    if path.is_file() {
        verify_model(path)?;
        return Ok(false);
    }
    eprintln!("[WavPitchSensorRust] Gepinntes Basic-Pitch-Modell wird einmalig bereitgestellt ...");
    let mut last_error = None;
    for url in [MODEL_URL_PRIMARY, MODEL_URL_FALLBACK] {
        match minreq::get(url).with_timeout(60).send() {
            Ok(response) if response.status_code == 200 => {
                let bytes = response.into_bytes();
                if bytes.len() != MODEL_BYTES || sha256_bytes(&bytes) != MODEL_SHA256 {
                    last_error = Some(anyhow::anyhow!(
                        "Modell-Fingerprint/Größe von {url} stimmt nicht"
                    ));
                    continue;
                }
                let tmp = path.with_extension("onnx.partial");
                fs::write(&tmp, &bytes)?;
                fs::rename(&tmp, path)?;
                verify_model(path)?;
                return Ok(true);
            }
            Ok(response) => {
                last_error = Some(anyhow::anyhow!(
                    "Modell-Download {url} lieferte HTTP {}",
                    response.status_code
                ));
            }
            Err(error) => last_error = Some(anyhow::anyhow!(error)),
        }
    }
    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Modell-Download fehlgeschlagen")))
}

fn verify_model(path: &Path) -> Result<()> {
    let bytes = fs::read(path)?;
    if bytes.len() != MODEL_BYTES {
        bail!(
            "Basic-Pitch-Modellgröße ungültig: erwartet {}, erhalten {}",
            MODEL_BYTES,
            bytes.len()
        );
    }
    let sha = sha256_bytes(&bytes);
    if sha != MODEL_SHA256 {
        bail!("Basic-Pitch-Modell-Fingerprint ungültig: {sha}");
    }
    Ok(())
}

fn package_fingerprint(root: &Path) -> Result<String> {
    // Runtime identity must be relocatable. A cached audit binary may legitimately move with
    // an EtuSchlopf project directory, so no compile-time CARGO_MANIFEST_DIR may participate
    // in the runtime contract. The launcher already rebuilds whenever this source surface
    // differs from the staged copy; hashing the current project source therefore binds the
    // executed binary to the same portable audit package without an absolute-path dependency.
    let package_root = root.join("tools/wav_pitch_external_sensor_audit");
    let mut hasher = Sha256::new();
    for relative in ["Cargo.toml", "BASIC_PITCH_NOTICE.md", "src/main.rs"] {
        let path = package_root.join(relative);
        if !path.is_file() {
            bail!("Audit-Quelloberfläche fehlt: {}", path.display());
        }
        hasher.update(relative.as_bytes());
        hasher.update([0]);
        hasher.update(fs::read(&path)?);
        hasher.update([0]);
    }
    for relative in ["src/midi.rs", "src/source_event.rs"] {
        let shared_source = root.join(relative);
        if !shared_source.is_file() {
            bail!("Gemeinsame SourceEvent-Quelle fehlt: {}", shared_source.display());
        }
        hasher.update(relative.as_bytes());
        hasher.update([0]);
        hasher.update(fs::read(&shared_source)?);
        hasher.update([0]);
    }
    Ok(hex_lower(&hasher.finalize()))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex_lower(&hasher.finalize())
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn decode_pcm24_stereo_44100(bytes: &[u8]) -> Result<Vec<f32>> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        bail!("WAV ist kein RIFF/WAVE");
    }
    let mut cursor = 12usize;
    let mut fmt = None::<(u16, u16, u32, u16, u16)>;
    let mut data = None::<&[u8]>;
    while cursor + 8 <= bytes.len() {
        let id = &bytes[cursor..cursor + 4];
        let size = u32::from_le_bytes(bytes[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
        cursor += 8;
        if cursor + size > bytes.len() {
            bail!("WAV-Chunk ist abgeschnitten");
        }
        let payload = &bytes[cursor..cursor + size];
        if id == b"fmt " {
            if payload.len() < 16 {
                bail!("WAV fmt-Chunk ist zu kurz");
            }
            fmt = Some((
                u16::from_le_bytes(payload[0..2].try_into().unwrap()),
                u16::from_le_bytes(payload[2..4].try_into().unwrap()),
                u32::from_le_bytes(payload[4..8].try_into().unwrap()),
                u16::from_le_bytes(payload[12..14].try_into().unwrap()),
                u16::from_le_bytes(payload[14..16].try_into().unwrap()),
            ));
        } else if id == b"data" {
            data = Some(payload);
        }
        cursor += size + (size & 1);
    }
    let (format, channels, sample_rate, block_align, bits) = fmt.context("WAV fmt-Chunk fehlt")?;
    if format != 1 || channels != 2 || sample_rate != 44_100 || block_align != 6 || bits != 24 {
        bail!(
            "WAV-Sensorvertrag verletzt: erwartet PCM 44.1kHz Stereo 24-bit, erhalten format={format} channels={channels} rate={sample_rate} align={block_align} bits={bits}"
        );
    }
    let data = data.context("WAV data-Chunk fehlt")?;
    if data.len() % 6 != 0 {
        bail!("WAV data-Chunk ist nicht frame-aligned");
    }
    let mut mono = Vec::with_capacity(data.len() / 6);
    for frame in data.chunks_exact(6) {
        let left = pcm24(frame[0], frame[1], frame[2]);
        let right = pcm24(frame[3], frame[4], frame[5]);
        mono.push((left + right) * 0.5);
    }
    Ok(mono)
}

fn pcm24(a: u8, b: u8, c: u8) -> f32 {
    let mut value = (a as i32) | ((b as i32) << 8) | ((c as i32) << 16);
    if value & 0x0080_0000 != 0 {
        value |= !0x00ff_ffff;
    }
    value as f32 / 8_388_608.0
}

fn decimate_2_fir127(input: &[f32]) -> Vec<f32> {
    // Long deterministic low-pass before exact 2:1 decimation. The source corpus is fixed
    // at 44.1 kHz, so no generic resampler or platform-specific DSP runtime is required.
    const TAPS: usize = 127;
    let center = (TAPS / 2) as isize;
    let cutoff = 0.24f64;
    let mut taps = [0.0f64; TAPS];
    let mut sum = 0.0;
    for (n, tap) in taps.iter_mut().enumerate() {
        let k = n as isize - center;
        let ideal = if k == 0 {
            2.0 * cutoff
        } else {
            (2.0 * std::f64::consts::PI * cutoff * k as f64).sin()
                / (std::f64::consts::PI * k as f64)
        };
        let window = 0.54
            - 0.46
                * (2.0 * std::f64::consts::PI * n as f64 / (TAPS as f64 - 1.0)).cos();
        *tap = ideal * window;
        sum += *tap;
    }
    for tap in &mut taps {
        *tap /= sum;
    }
    let out_len = input.len().div_ceil(2);
    let mut output = Vec::with_capacity(out_len);
    for out_index in 0..out_len {
        let source_center = (out_index * 2) as isize;
        let mut value = 0.0f64;
        for (n, tap) in taps.iter().enumerate() {
            let source = source_center + n as isize - center;
            if source >= 0 && (source as usize) < input.len() {
                value += input[source as usize] as f64 * *tap;
            }
        }
        output.push(value as f32);
    }
    output
}

fn infer_basic_pitch(model_path: &Path, audio: &[f32], source_frames: u64) -> Result<Activations> {
    let mut parsed = tract_onnx::onnx()
        .model_for_path(model_path)
        .with_context(|| format!("ONNX-Modell nicht ladbar: {}", model_path.display()))?;
    // The exact ONNX bytes are pinned above. r166 proved that tract does not expose
    // the TensorFlow/ONNX input export name as an outlet label. The model has exactly
    // one input, so bind it positionally. Outputs are different: note and onset share
    // the same shape and must never be guessed by position. tract preserves imported
    // framework node names and can bind model outputs by those node names.
    let input_outlets = parsed.input_outlets()?;
    if input_outlets.len() != 1 {
        bail!(
            "Basic-Pitch ONNX Input-Topologie unerwartet: erwartet 1 Eingang, erhalten {}",
            input_outlets.len()
        );
    }
    let original_outputs = parsed.output_outlets()?.to_vec();
    if original_outputs.len() != 3 {
        bail!(
            "Basic-Pitch ONNX Output-Topologie unerwartet: erwartet 3 Ausgänge, erhalten {}",
            original_outputs.len()
        );
    }
    let original_output_nodes = original_outputs
        .iter()
        .map(|outlet| parsed.node(outlet.node).name.clone())
        .collect::<Vec<_>>();
    parsed
        .set_output_names([NOTE_OUTPUT_NODE, ONSET_OUTPUT_NODE, CONTOUR_OUTPUT_NODE])
        .with_context(|| {
            format!(
                "Basic-Pitch ONNX Output-Nodes nicht bindbar; vorhanden: {:?}",
                original_output_nodes
            )
        })?;
    let model = parsed
        .with_input_fact(
            0,
            InferenceFact::dt_shape(f32::datum_type(), tvec!(1, AUDIO_N_SAMPLES, 1)),
        )?
        .into_optimized()?
        .into_runnable()?;

    let padded_len = PREFIX_SAMPLES + audio.len();
    let expected_frames = ((audio.len() as f64 / HOP_SIZE as f64) * KEPT_FRAMES as f64) as usize;
    let mut frames = Vec::with_capacity(expected_frames * PITCH_BINS);
    let mut onsets = Vec::with_capacity(expected_frames * PITCH_BINS);
    let mut frame_source = Vec::with_capacity(expected_frames);
    let mut start = 0usize;

    while start < padded_len && frame_source.len() < expected_frames {
        let mut window = vec![0.0f32; AUDIO_N_SAMPLES];
        for (offset, sample) in window.iter_mut().enumerate() {
            let padded_index = start + offset;
            if padded_index >= PREFIX_SAMPLES {
                let source = padded_index - PREFIX_SAMPLES;
                if let Some(value) = audio.get(source) {
                    *sample = *value;
                }
            }
        }
        let input: Tensor = tract_ndarray::Array3::<f32>::from_shape_vec(
            (1, AUDIO_N_SAMPLES, 1),
            window,
        )?
        .into();
        let outputs = model.run(tvec!(input.into()))?;
        if outputs.len() != 3 {
            bail!("Basic-Pitch ONNX lieferte {} statt 3 Ausgänge", outputs.len());
        }
        let note_view = outputs[0].to_array_view::<f32>()?;
        let onset_view = outputs[1].to_array_view::<f32>()?;
        let contour_view = outputs[2].to_array_view::<f32>()?;
        if note_view.shape() != [1, ANNOT_N_FRAMES, PITCH_BINS]
            || onset_view.shape() != [1, ANNOT_N_FRAMES, PITCH_BINS]
            || contour_view.shape() != [1, ANNOT_N_FRAMES, PITCH_BINS * 3]
        {
            bail!(
                "Basic-Pitch ONNX Output-Shape unerwartet: note={:?} onset={:?} contour={:?}",
                note_view.shape(),
                onset_view.shape(),
                contour_view.shape()
            );
        }
        let note_flat: Vec<f32> = note_view.iter().copied().collect();
        let onset_flat: Vec<f32> = onset_view.iter().copied().collect();
        for j in TRIM_FRAMES..(ANNOT_N_FRAMES - TRIM_FRAMES) {
            if frame_source.len() >= expected_frames {
                break;
            }
            let base = j * PITCH_BINS;
            frames.extend_from_slice(&note_flat[base..base + PITCH_BINS]);
            onsets.extend_from_slice(&onset_flat[base..base + PITCH_BINS]);
            let model_sample = start + j * FFT_HOP;
            let original_22050 = model_sample.saturating_sub(PREFIX_SAMPLES);
            let source = (original_22050 as u64)
                .saturating_mul(2)
                .min(source_frames.saturating_sub(1));
            frame_source.push(source);
        }
        start = start.saturating_add(HOP_SIZE);
    }
    if frame_source.len() != expected_frames {
        bail!(
            "Basic-Pitch Window-Unwrap unvollständig: erwartet {} Frames, erhalten {}",
            expected_frames,
            frame_source.len()
        );
    }
    Ok(Activations {
        frames,
        onsets,
        frame_source,
        n_frames: expected_frames,
    })
}

fn decode_polyphonic_events(a: &Activations) -> Result<Vec<SensorEvent>> {
    if a.frames.len() != a.n_frames * PITCH_BINS
        || a.onsets.len() != a.n_frames * PITCH_BINS
        || a.frame_source.len() != a.n_frames
    {
        bail!("Basic-Pitch Aktivierungsdimensionen inkonsistent");
    }
    if a.n_frames < 3 {
        return Ok(Vec::new());
    }
    let inferred_onsets = inferred_onsets(&a.onsets, &a.frames, a.n_frames);
    let mut remaining = a.frames.clone();
    let mut peaks = Vec::<(usize, usize)>::new();
    for t in 1..a.n_frames - 1 {
        for p in 0..PITCH_BINS {
            let value = inferred_onsets[idx(t, p)];
            if value >= ONSET_THRESHOLD
                && value > inferred_onsets[idx(t - 1, p)]
                && value > inferred_onsets[idx(t + 1, p)]
            {
                peaks.push((t, p));
            }
        }
    }
    peaks.sort_unstable();
    peaks.reverse();

    let mut events = Vec::<SensorEvent>::new();
    for (start, pitch) in peaks {
        if start >= a.n_frames - 1 {
            continue;
        }
        let mut i = start + 1;
        let mut k = 0usize;
        while i < a.n_frames - 1 && k < ENERGY_TOLERANCE {
            if remaining[idx(i, pitch)] < FRAME_THRESHOLD {
                k += 1;
            } else {
                k = 0;
            }
            i += 1;
        }
        i = i.saturating_sub(k);
        if i.saturating_sub(start) <= MIN_NOTE_LEN {
            continue;
        }
        clear_pitch_band(&mut remaining, a.n_frames, start, i, pitch);
        push_event(&mut events, a, start, i, pitch);
    }

    let mut heap = BinaryHeap::<HeapCell>::new();
    for (flat, amp) in remaining.iter().copied().enumerate() {
        if amp > FRAME_THRESHOLD {
            heap.push(HeapCell::new(amp, flat));
        }
    }
    while let Some(cell) = heap.pop() {
        let flat = cell.flat;
        if remaining[flat] <= FRAME_THRESHOLD {
            continue;
        }
        let mid = flat / PITCH_BINS;
        let pitch = flat % PITCH_BINS;
        remaining[flat] = 0.0;

        let mut i = mid + 1;
        let mut k = 0usize;
        while i < a.n_frames - 1 && k < ENERGY_TOLERANCE {
            if remaining[idx(i, pitch)] < FRAME_THRESHOLD {
                k += 1;
            } else {
                k = 0;
            }
            clear_pitch_cell_and_neighbors(&mut remaining, i, pitch);
            i += 1;
        }
        let end = i.saturating_sub(1 + k);

        let mut back = mid as isize - 1;
        let mut back_k = 0usize;
        while back > 0 && back_k < ENERGY_TOLERANCE {
            let t = back as usize;
            if remaining[idx(t, pitch)] < FRAME_THRESHOLD {
                back_k += 1;
            } else {
                back_k = 0;
            }
            clear_pitch_cell_and_neighbors(&mut remaining, t, pitch);
            back -= 1;
        }
        let start = (back + 1 + back_k as isize).max(0) as usize;
        if end.saturating_sub(start) <= MIN_NOTE_LEN {
            continue;
        }
        push_event(&mut events, a, start, end, pitch);
    }
    Ok(events)
}

fn inferred_onsets(onsets: &[f32], frames: &[f32], n_frames: usize) -> Vec<f32> {
    let mut diff = vec![0.0f32; onsets.len()];
    for t in 0..n_frames {
        for p in 0..PITCH_BINS {
            let current = frames[idx(t, p)];
            let d1 = current - if t >= 1 { frames[idx(t - 1, p)] } else { 0.0 };
            let d2 = current - if t >= 2 { frames[idx(t - 2, p)] } else { 0.0 };
            let mut value = d1.min(d2).max(0.0);
            if t < 2 {
                value = 0.0;
            }
            diff[idx(t, p)] = value;
        }
    }
    let onset_max = onsets.iter().copied().fold(0.0f32, f32::max);
    let diff_max = diff.iter().copied().fold(0.0f32, f32::max);
    if diff_max > 0.0 {
        let scale = onset_max / diff_max;
        for value in &mut diff {
            *value *= scale;
        }
    }
    onsets
        .iter()
        .zip(diff)
        .map(|(a, b)| a.max(b))
        .collect()
}

fn idx(time: usize, pitch: usize) -> usize {
    time * PITCH_BINS + pitch
}

fn clear_pitch_band(remaining: &mut [f32], n_frames: usize, start: usize, end: usize, pitch: usize) {
    for t in start..end.min(n_frames) {
        clear_pitch_cell_and_neighbors(remaining, t, pitch);
    }
}

fn clear_pitch_cell_and_neighbors(remaining: &mut [f32], time: usize, pitch: usize) {
    remaining[idx(time, pitch)] = 0.0;
    if pitch + 1 < PITCH_BINS {
        remaining[idx(time, pitch + 1)] = 0.0;
    }
    if pitch > 0 {
        remaining[idx(time, pitch - 1)] = 0.0;
    }
}

fn push_event(events: &mut Vec<SensorEvent>, a: &Activations, start: usize, end: usize, pitch: usize) {
    if start >= a.n_frames || end <= start {
        return;
    }
    let end_clamped = end.min(a.n_frames - 1);
    let mut sum = 0.0f64;
    let mut count = 0usize;
    for t in start..end.min(a.n_frames) {
        sum += a.frames[idx(t, pitch)] as f64;
        count += 1;
    }
    let amplitude = if count == 0 { 0.0 } else { (sum / count as f64) as f32 };
    events.push(SensorEvent {
        ordinal: 0,
        midi: MIDI_OFFSET + pitch as u8,
        start_idx: start,
        end_idx: end,
        amplitude,
        onset_frame: a.frame_source[start],
        offset_frame: a.frame_source[end_clamped],
    });
}

fn normalize_event_order(events: &mut [SensorEvent]) {
    events.sort_by(|a, b| {
        a.onset_frame
            .cmp(&b.onset_frame)
            .then_with(|| a.midi.cmp(&b.midi))
            .then_with(|| a.offset_frame.cmp(&b.offset_frame))
            .then_with(|| a.amplitude.total_cmp(&b.amplitude))
    });
    for (ordinal, event) in events.iter_mut().enumerate() {
        event.ordinal = ordinal;
    }
}

fn event_projection_bytes(events: &[SensorEvent]) -> Vec<u8> {
    let mut text = String::new();
    text.push_str("schema=basic_pitch_rust_event_projection_v1\n");
    for event in events {
        use std::fmt::Write as _;
        let _ = writeln!(
            text,
            "{}|{}|{}|{}|{:08x}|{}|{}",
            event.ordinal,
            event.midi,
            event.onset_frame,
            event.offset_frame,
            event.amplitude.to_bits(),
            event.start_idx,
            event.end_idx,
        );
    }
    text.into_bytes()
}

fn oracle_source_frame(pos: u64, quarter_frames: u64) -> u64 {
    (pos * quarter_frames + CANONICAL_DIVISIONS / 2) / CANONICAL_DIVISIONS
}

fn tolerance_frames(ms: u64) -> u64 {
    (ms * SOURCE_SAMPLE_RATE + 500) / 1000
}

fn match_events(
    oracle: &[OracleEvent],
    sensor: &[SensorEvent],
    quarter_frames: u64,
    tolerance_ms: u64,
) -> Vec<MatchPair> {
    let tolerance = tolerance_frames(tolerance_ms);
    let mut by_pitch_o = BTreeMap::<u8, Vec<(usize, u64)>>::new();
    let mut by_pitch_s = BTreeMap::<u8, Vec<(usize, u64)>>::new();
    for event in oracle {
        by_pitch_o
            .entry(event.midi)
            .or_default()
            .push((event.ordinal, oracle_source_frame(event.pos, quarter_frames)));
    }
    for event in sensor {
        by_pitch_s
            .entry(event.midi)
            .or_default()
            .push((event.ordinal, event.onset_frame));
    }
    let mut pitches = BTreeMap::<u8, ()>::new();
    for pitch in by_pitch_o.keys().chain(by_pitch_s.keys()) {
        pitches.insert(*pitch, ());
    }
    let mut out = Vec::new();
    for pitch in pitches.keys() {
        let empty = Vec::new();
        let o = by_pitch_o.get(pitch).unwrap_or(&empty);
        let s = by_pitch_s.get(pitch).unwrap_or(&empty);
        out.extend(match_same_pitch(o, s, tolerance));
    }
    out.sort_by_key(|pair| (pair.oracle, pair.sensor));
    out
}

fn match_same_pitch(oracle: &[(usize, u64)], sensor: &[(usize, u64)], tolerance: u64) -> Vec<MatchPair> {
    let n = oracle.len();
    let m = sensor.len();
    let width = m + 1;
    let mut count = vec![0u32; (n + 1) * (m + 1)];
    let mut residual = vec![0u64; (n + 1) * (m + 1)];
    let mut action = vec![0u8; (n + 1) * (m + 1)];
    for i in 1..=n {
        action[i * width] = 1;
    }
    for j in 1..=m {
        action[j] = 2;
    }
    for i in 1..=n {
        for j in 1..=m {
            let cur = i * width + j;
            let up = (i - 1) * width + j;
            let left = i * width + j - 1;
            let diag = (i - 1) * width + j - 1;
            let mut best_count = count[up];
            let mut best_residual = residual[up];
            let mut best_action = 1u8;
            if better_pair(count[left], residual[left], best_count, best_residual) {
                best_count = count[left];
                best_residual = residual[left];
                best_action = 2;
            }
            let delta = oracle[i - 1].1.abs_diff(sensor[j - 1].1);
            if delta <= tolerance {
                let candidate_count = count[diag] + 1;
                let candidate_residual = residual[diag] + delta;
                if better_pair(candidate_count, candidate_residual, best_count, best_residual) {
                    best_count = candidate_count;
                    best_residual = candidate_residual;
                    best_action = 3;
                }
            }
            count[cur] = best_count;
            residual[cur] = best_residual;
            action[cur] = best_action;
        }
    }
    let mut pairs = Vec::new();
    let mut i = n;
    let mut j = m;
    while i > 0 || j > 0 {
        let act = action[i * width + j];
        match act {
            3 => {
                pairs.push(MatchPair {
                    oracle: oracle[i - 1].0,
                    sensor: sensor[j - 1].0,
                });
                i -= 1;
                j -= 1;
            }
            1 if i > 0 => i -= 1,
            2 if j > 0 => j -= 1,
            _ if i > 0 => i -= 1,
            _ if j > 0 => j -= 1,
            _ => break,
        }
    }
    pairs.reverse();
    pairs
}

fn better_pair(candidate_count: u32, candidate_residual: u64, best_count: u32, best_residual: u64) -> bool {
    candidate_count > best_count || (candidate_count == best_count && candidate_residual < best_residual)
}

fn evaluate_matches(oracle: &[OracleEvent], sensor: &[SensorEvent], quarter_frames: u64, tolerance_ms: u64) -> Prf {
    let matches = match_events(oracle, sensor, quarter_frames, tolerance_ms).len();
    prf(matches, oracle.len(), sensor.len())
}

fn prf(matches: usize, oracle_count: usize, sensor_count: usize) -> Prf {
    let recall = if oracle_count == 0 { 1.0 } else { matches as f64 / oracle_count as f64 };
    let precision = if sensor_count == 0 {
        if oracle_count == 0 { 1.0 } else { 0.0 }
    } else {
        matches as f64 / sensor_count as f64
    };
    let f1 = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    };
    Prf {
        precision,
        recall,
        f1,
    }
}

fn subset_prf<F: Fn(u8) -> bool>(
    oracle: &[OracleEvent],
    sensor: &[SensorEvent],
    quarter_frames: u64,
    tolerance_ms: u64,
    predicate: F,
) -> Prf {
    let mut o = Vec::new();
    let mut s = Vec::new();
    for event in oracle.iter().filter(|e| predicate(e.midi)) {
        let mut event = event.clone();
        event.ordinal = o.len();
        o.push(event);
    }
    for event in sensor.iter().filter(|e| predicate(e.midi)) {
        let mut event = event.clone();
        event.ordinal = s.len();
        s.push(event);
    }
    evaluate_matches(&o, &s, quarter_frames, tolerance_ms)
}

fn chord_tone_recall(oracle: &[OracleEvent], sensor: &[SensorEvent], quarter_frames: u64, tolerance_ms: u64) -> f64 {
    let tolerance = tolerance_frames(tolerance_ms);
    let mut groups = BTreeMap::<u64, Vec<&OracleEvent>>::new();
    for event in oracle {
        groups.entry(event.pos).or_default().push(event);
    }
    let mut total = 0usize;
    let mut recovered = 0usize;
    for (pos, group) in groups.into_iter().filter(|(_, group)| group.len() >= 2) {
        let center = oracle_source_frame(pos, quarter_frames);
        let mut want = [0usize; 128];
        let mut got = [0usize; 128];
        for event in group {
            want[event.midi as usize] += 1;
            total += 1;
        }
        for event in sensor {
            if event.onset_frame.abs_diff(center) <= tolerance {
                got[event.midi as usize] += 1;
            }
        }
        for pitch in 0..128 {
            recovered += want[pitch].min(got[pitch]);
        }
    }
    if total == 0 { 1.0 } else { recovered as f64 / total as f64 }
}

fn rearticulation_recall(
    oracle: &[OracleEvent],
    sensor: &[SensorEvent],
    quarter_frames: u64,
    tolerance_ms: u64,
) -> f64 {
    let matched: HashSet<usize> = match_events(oracle, sensor, quarter_frames, tolerance_ms)
        .into_iter()
        .map(|pair| pair.oracle)
        .collect();
    let mut by_pitch = BTreeMap::<u8, Vec<&OracleEvent>>::new();
    for event in oracle {
        by_pitch.entry(event.midi).or_default().push(event);
    }
    let mut pairs = 0usize;
    let mut both = 0usize;
    for rows in by_pitch.values_mut() {
        rows.sort_by_key(|event| (event.pos, event.ordinal));
        for window in rows.windows(2) {
            let previous = window[0];
            let current = window[1];
            if current.pos > previous.pos && current.pos - previous.pos <= CANONICAL_DIVISIONS {
                pairs += 1;
                if matched.contains(&previous.ordinal) && matched.contains(&current.ordinal) {
                    both += 1;
                }
            }
        }
    }
    if pairs == 0 { 1.0 } else { both as f64 / pairs as f64 }
}

fn multiplicity_recall(
    oracle: &[OracleEvent],
    sensor: &[SensorEvent],
    quarter_frames: u64,
    tolerance_ms: u64,
) -> f64 {
    let tolerance = tolerance_frames(tolerance_ms);
    let mut groups = BTreeMap::<(u64, u8), usize>::new();
    for event in oracle {
        *groups.entry((event.pos, event.midi)).or_default() += 1;
    }
    let duplicates: Vec<_> = groups.into_iter().filter(|(_, count)| *count > 1).collect();
    let total: usize = duplicates.iter().map(|(_, count)| *count).sum();
    let mut recovered = 0usize;
    for ((pos, midi), count) in duplicates {
        let center = oracle_source_frame(pos, quarter_frames);
        let got = sensor
            .iter()
            .filter(|event| event.midi == midi && event.onset_frame.abs_diff(center) <= tolerance)
            .count();
        recovered += count.min(got);
    }
    if total == 0 { 1.0 } else { recovered as f64 / total as f64 }
}

fn false_event_harmonic_count(
    oracle: &[OracleEvent],
    sensor: &[SensorEvent],
    quarter_frames: u64,
    tolerance_ms: u64,
) -> (usize, usize) {
    let tolerance = tolerance_frames(tolerance_ms);
    let matched_sensor: HashSet<usize> = match_events(oracle, sensor, quarter_frames, tolerance_ms)
        .into_iter()
        .map(|pair| pair.sensor)
        .collect();
    let oracle_times: Vec<_> = oracle
        .iter()
        .map(|event| (event.midi, oracle_source_frame(event.pos, quarter_frames)))
        .collect();
    let mut false_count = 0usize;
    let mut harmonic = 0usize;
    for event in sensor.iter().filter(|event| !matched_sensor.contains(&event.ordinal)) {
        false_count += 1;
        if oracle_times.iter().any(|(pitch, frame)| {
            event.onset_frame.abs_diff(*frame) <= tolerance
                && event.midi > *pitch
                && HARMONIC_INTERVALS.contains(&(event.midi - *pitch))
        }) {
            harmonic += 1;
        }
    }
    (false_count, harmonic)
}

fn permille(value: f64) -> u64 {
    (value.clamp(0.0, 1.0) * 1000.0).round() as u64
}
