//! Audit-only execution and readback for one qualified external WAV pitch/event sensor candidate.
//!
//! The candidate owns evidence only. Basic Pitch ONNX is executed by a dedicated Rust audit
//! crate. No Python runtime, MIDI transport, CanonicalSourceEvents, staff/hand, phrase, writer
//! or productive source authority is introduced here.

#[cfg(any(windows, test))]
use std::collections::BTreeMap;
#[cfg(windows)]
use std::fs;
#[cfg(windows)]
use std::path::Path;
#[cfg(windows)]
use std::process::Command;
#[cfg(windows)]
use std::time::Instant;

const CONTRACT_SCHEMA: &str = "wav_pitch_sensor_qualification_v1";
const SENSOR_IDENTITY: &str = "SpotifyBasicPitchOnnxRust";
const SENSOR_VERSION: &str = "ICASSP2022-ONNX";
#[cfg(windows)]
const TOOL_SOURCE_RELATIVE: &str = "tools/wav_pitch_external_sensor_audit";
#[cfg(windows)]
const TOOL_TARGET_SOURCE_RELATIVE: &str =
    "target/wav_pitch_external_sensor_audit/rust_tool_src";
#[cfg(windows)]
const TOOL_CARGO_TARGET_RELATIVE: &str = "target/wav_pitch_external_sensor_audit/cargo_target";
#[cfg(windows)]
const TOOL_RUNTIME_RELATIVE: &str = "target/wav_pitch_external_sensor_audit/runtime";
#[cfg(windows)]
const LATEST_KV_RELATIVE: &str = "target/wav_pitch_external_sensor_audit/latest_audit_kv.txt";
#[cfg(windows)]
const SENSOR_EXE_RELATIVE: &str =
    "target/wav_pitch_external_sensor_audit/cargo_target/release/etuschlopf_wav_pitch_external_sensor_audit.exe";
#[cfg(windows)]
const MODEL_RELATIVE: &str =
    "target/wav_pitch_external_sensor_audit/runtime/model/basic_pitch_icassp2022_nmp.onnx";
#[cfg(windows)]
const PIECE_CACHE_IDENTITY_FILE: &str = "EVIDENCE_REUSE_IDENTITY_V1.txt";
#[cfg(windows)]
const PIECE_CACHE_SCHEMA: &str = "wav_pitch_external_sensor_evidence_reuse_v1";

#[derive(Debug, Clone)]
pub(crate) struct WavPitchExternalSensorBenchmarkAudit {
    pub(crate) observed: bool,
    pub(crate) reason: String,
    pub(crate) contract_schema: String,
    pub(crate) authority: String,
    pub(crate) sensor_identity: String,
    pub(crate) sensor_version: String,
    pub(crate) package_fingerprint_sha256: String,
    pub(crate) model_fingerprint_sha256: String,
    pub(crate) feature_schema: String,
    pub(crate) source_wav_sha256: String,
    pub(crate) freeze_sha256: String,
    pub(crate) sensor_events: usize,
    pub(crate) oracle_events: usize,
    pub(crate) precision_80_permille: u64,
    pub(crate) recall_80_permille: u64,
    pub(crate) f1_80_permille: u64,
    pub(crate) chord_tone_recall_80_permille: u64,
    pub(crate) rearticulation_both_80_permille: u64,
    pub(crate) multiplicity_recall_80_permille: u64,
    pub(crate) bass_recall_80_permille: u64,
    pub(crate) high_recall_80_permille: u64,
    pub(crate) false_sensor_events_80: usize,
    pub(crate) harmonic_explainable_false_80: usize,
    pub(crate) deterministic_repeat: String,
    pub(crate) windows_offline_runtime: String,
    pub(crate) setup_network_used: String,
    pub(crate) original_frame_mapping: String,
    pub(crate) midi_authority: String,
    pub(crate) canonical_source_event_promotion: String,
}

pub(crate) fn build_for_score_title(
    score_title: Option<&str>,
) -> Option<WavPitchExternalSensorBenchmarkAudit> {
    let title = score_title?.trim();
    if title.is_empty() {
        return None;
    }
    let current = std::env::current_dir().ok()?;
    if !current.join(format!("{title}.wav")).is_file() {
        return None;
    }

    #[cfg(not(windows))]
    {
        Some(not_observed("windows_runtime_required"))
    }

    #[cfg(windows)]
    {
        let benchmark_started = Instant::now();
        // The benchmark is rendered while the current result report is still being built.
        // Therefore it must not depend on <title>.result.txt. Reuse the already-qualified
        // WAV-only timing owner in memory. No FLP bytes are opened in this owner.
        let signal = match crate::audit_only::wav_signal_decode::load_decoded_wav_signal_for_score_title(
            Some(title),
        ) {
            Ok(signal) => signal,
            Err(reason) => return Some(not_observed(reason)),
        };
        let timing = match crate::audit_only::wav_attack_pulse_metric::build_audio_only_pitch_timing_snapshot(
            &signal,
        ) {
            Ok(timing) => timing,
            Err(reason) => return Some(not_observed(reason)),
        };
        let wav_bytes = match fs::read(current.join(format!("{title}.wav"))) {
            Ok(bytes) => bytes,
            Err(_) => return Some(not_observed("wav_file_unreadable")),
        };
        let expected_wav_sha256 = crate::sha256::hex(&wav_bytes);

        let tool_source = current.join(TOOL_SOURCE_RELATIVE);
        if !tool_source.join("Cargo.toml").is_file()
            || !tool_source.join("src/main.rs").is_file()
        {
            return Some(not_observed("rust_onnx_benchmark_source_missing"));
        }

        let target_source = current.join(TOOL_TARGET_SOURCE_RELATIVE);
        let cargo_target = current.join(TOOL_CARGO_TARGET_RELATIVE);
        let runtime = current.join(TOOL_RUNTIME_RELATIVE);
        if fs::create_dir_all(&target_source).is_err()
            || fs::create_dir_all(&cargo_target).is_err()
            || fs::create_dir_all(&runtime).is_err()
        {
            return Some(not_observed("rust_onnx_target_boundary_unavailable"));
        }

        // Retire any earlier Python-sidecar runtime from r157-r161. target/ is explicitly
        // derived state, so this does not mutate the managed source tree.
        let _ = fs::remove_dir_all(runtime.join("venv"));
        let _ = fs::remove_dir_all(current.join("target/wav_pitch_external_sensor_audit/python"));

        let mut source_changed = false;
        for relative in ["Cargo.toml", "BASIC_PITCH_NOTICE.md", "src/main.rs"] {
            match sync_file_if_changed(&tool_source.join(relative), &target_source.join(relative)) {
                Ok(changed) => source_changed |= changed,
                Err(_) => return Some(not_observed("rust_onnx_tool_source_sync_failed")),
            }
        }
        match sync_file_if_changed(
            &current.join("src/midi.rs"),
            &target_source.join("src/shared_midi.rs"),
        ) {
            Ok(changed) => source_changed |= changed,
            Err(_) => return Some(not_observed("rust_onnx_shared_midi_source_sync_failed")),
        }
        match sync_file_if_changed(
            &current.join("src/source_event.rs"),
            &target_source.join("src/shared_source_event.rs"),
        ) {
            Ok(changed) => source_changed |= changed,
            Err(_) => {
                return Some(not_observed(
                    "rust_onnx_shared_source_event_source_sync_failed",
                ));
            }
        }

        let executable = current.join(SENSOR_EXE_RELATIVE);
        let needs_build = source_changed || !executable.is_file();
        let cargo_bootstrap_mode = if needs_build {
            if !build_rust_audit_tool(&target_source, &cargo_target) {
                return Some(not_observed("rust_onnx_sensor_build_failed"));
            }
            "build_or_dependency_bootstrap_possible"
        } else {
            "cached_binary_no_cargo_process"
        };
        if !executable.is_file() {
            return Some(not_observed("rust_onnx_sensor_binary_missing_after_build"));
        }

        let pieces = current.join("target/wav_pitch_external_sensor_audit/pieces");
        let piece_output = pieces.join(title);
        if fs::create_dir_all(&piece_output).is_err() {
            return Some(not_observed("rust_onnx_piece_output_unavailable"));
        }
        let latest = current.join(LATEST_KV_RELATIVE);
        let _ = fs::remove_file(&latest);
        let evidence_started = Instant::now();

        if try_reuse_piece_evidence(
            &current,
            title,
            &piece_output,
            &latest,
            timing.quarter_frames,
            &expected_wav_sha256,
        ) {
            eprintln!(
                "ETUSCHLOPF: EvidenceRuntimeReuse: sensor=BasicPitch cache=hit cache_phase_ms={} total_benchmark_ms={}",
                evidence_started.elapsed().as_millis(),
                benchmark_started.elapsed().as_millis()
            );
            return Some(read_kv_audit(&latest));
        }

        let _ = fs::remove_dir_all(&piece_output);
        if fs::create_dir_all(&piece_output).is_err() {
            return Some(not_observed("rust_onnx_piece_output_unavailable"));
        }
        let status = Command::new(&executable)
            .arg("--root")
            .arg(&current)
            .arg("--project")
            .arg(title)
            .arg("--output")
            .arg(&piece_output)
            .arg("--runtime")
            .arg(&runtime)
            .arg("--quarter-frames")
            .arg(timing.quarter_frames.to_string())
            .arg("--expected-wav-sha256")
            .arg(&expected_wav_sha256)
            .arg("--cargo-bootstrap-mode")
            .arg(cargo_bootstrap_mode)
            .status();
        let Ok(status) = status else {
            return Some(not_observed("rust_onnx_sensor_process_start_failed"));
        };
        if !status.success() {
            return Some(not_observed("rust_onnx_sensor_benchmark_process_failed"));
        }
        let audit = read_kv_audit(&latest);
        if audit.observed {
            let _ = write_piece_cache_identity(
                &current,
                title,
                &piece_output,
                timing.quarter_frames,
                &expected_wav_sha256,
                &audit,
            );
        }
        eprintln!(
            "ETUSCHLOPF: EvidenceRuntimeReuse: sensor=BasicPitch cache=miss cache_phase_ms={} total_benchmark_ms={}",
            evidence_started.elapsed().as_millis(),
            benchmark_started.elapsed().as_millis()
        );
        Some(audit)
    }
}

pub(crate) fn runtime_present_for_current_project() -> bool {
    #[cfg(not(windows))]
    {
        false
    }
    #[cfg(windows)]
    {
        let Ok(current) = std::env::current_dir() else {
            return false;
        };
        current.join(SENSOR_EXE_RELATIVE).is_file() && current.join(MODEL_RELATIVE).is_file()
    }
}

#[cfg(windows)]
fn current_sensor_package_fingerprint(root: &Path) -> Result<String, &'static str> {
    let package_root = root.join(TOOL_SOURCE_RELATIVE);
    let mut bytes = Vec::<u8>::new();
    for relative in ["Cargo.toml", "BASIC_PITCH_NOTICE.md", "src/main.rs"] {
        let path = package_root.join(relative);
        let source = fs::read(path).map_err(|_| "sensor_package_source_unreadable")?;
        bytes.extend_from_slice(relative.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&source);
        bytes.push(0);
    }
    for relative in ["src/midi.rs", "src/source_event.rs"] {
        let source = fs::read(root.join(relative)).map_err(|_| "sensor_shared_source_unreadable")?;
        bytes.extend_from_slice(relative.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&source);
        bytes.push(0);
    }
    Ok(crate::sha256::hex(&bytes))
}

#[cfg(windows)]
fn file_sha256(path: &Path) -> Result<String, &'static str> {
    fs::read(path)
        .map(|bytes| crate::sha256::hex(&bytes))
        .map_err(|_| "cache_identity_input_unreadable")
}

#[cfg(windows)]
fn try_reuse_piece_evidence(
    root: &Path,
    title: &str,
    piece_output: &Path,
    latest: &Path,
    quarter_frames: u64,
    expected_wav_sha256: &str,
) -> bool {
    let identity_path = piece_output.join(PIECE_CACHE_IDENTITY_FILE);
    let audit_path = piece_output.join("audit_kv.txt");
    let freeze_manifest_path = piece_output.join("sensor_freeze_manifest.txt");
    let freeze_sha_path = piece_output.join("SENSOR_OUTPUT_FROZEN.sha256");
    let projection_path = piece_output.join("sensor_events.frozen.txt");
    let Ok(identity_text) = fs::read_to_string(&identity_path) else {
        return false;
    };
    let identity = parse_kv(&identity_text);
    if identity.get("schema").map(String::as_str) != Some(PIECE_CACHE_SCHEMA)
        || identity.get("project").map(String::as_str) != Some(title)
        || identity.get("source_wav_sha256").map(String::as_str) != Some(expected_wav_sha256)
        || get_u64(&identity, "quarter_frames") != quarter_frames
    {
        return false;
    }

    let midi_path = root.join(format!("{title}.mid"));
    let Ok(current_midi_sha256) = file_sha256(&midi_path) else {
        return false;
    };
    if identity.get("source_midi_sha256").map(String::as_str)
        != Some(current_midi_sha256.as_str())
    {
        return false;
    }
    let Ok(current_package_sha256) = current_sensor_package_fingerprint(root) else {
        return false;
    };
    if identity
        .get("package_fingerprint_sha256")
        .map(String::as_str)
        != Some(current_package_sha256.as_str())
    {
        return false;
    }
    let model_path = root.join(MODEL_RELATIVE);
    let Ok(current_model_sha256) = file_sha256(&model_path) else {
        return false;
    };
    if identity.get("model_fingerprint_sha256").map(String::as_str)
        != Some(current_model_sha256.as_str())
    {
        return false;
    }

    let Ok(audit_text) = fs::read_to_string(&audit_path) else {
        return false;
    };
    let audit = parse_kv(&audit_text);
    for key in [
        "package_fingerprint_sha256",
        "model_fingerprint_sha256",
        "feature_schema",
        "source_wav_sha256",
        "freeze_sha256",
        "sensor_version",
    ] {
        if identity.get(key) != audit.get(key) {
            return false;
        }
    }
    if audit.get("status").map(String::as_str) != Some("observed")
        || audit.get("contract_schema").map(String::as_str) != Some(CONTRACT_SCHEMA)
        || audit.get("sensor_identity").map(String::as_str) != Some(SENSOR_IDENTITY)
        || audit.get("sensor_version").map(String::as_str) != Some(SENSOR_VERSION)
    {
        return false;
    }

    let Ok(freeze_manifest) = fs::read(&freeze_manifest_path) else {
        return false;
    };
    let freeze_sha256 = crate::sha256::hex(&freeze_manifest);
    if identity.get("freeze_sha256").map(String::as_str) != Some(freeze_sha256.as_str()) {
        return false;
    }
    let Ok(freeze_sha_text) = fs::read_to_string(&freeze_sha_path) else {
        return false;
    };
    if freeze_sha_text.trim() != freeze_sha256 {
        return false;
    }
    let freeze_values = parse_kv(&String::from_utf8_lossy(&freeze_manifest));
    for key in [
        "package_fingerprint_sha256",
        "model_fingerprint_sha256",
        "feature_schema",
        "source_wav_sha256",
        "sensor_version",
    ] {
        if identity.get(key) != freeze_values.get(key) {
            return false;
        }
    }
    let Ok(projection) = fs::read(&projection_path) else {
        return false;
    };
    let projection_sha256 = crate::sha256::hex(&projection);
    if freeze_values
        .get("frozen_event_projection_sha256")
        .map(String::as_str)
        != Some(projection_sha256.as_str())
    {
        return false;
    }

    fs::write(latest, audit_text.as_bytes()).is_ok()
}

#[cfg(windows)]
fn write_piece_cache_identity(
    root: &Path,
    title: &str,
    piece_output: &Path,
    quarter_frames: u64,
    expected_wav_sha256: &str,
    audit: &WavPitchExternalSensorBenchmarkAudit,
) -> Result<(), &'static str> {
    let source_midi_sha256 = file_sha256(&root.join(format!("{title}.mid")))?;
    let current_package_sha256 = current_sensor_package_fingerprint(root)?;
    if current_package_sha256 != audit.package_fingerprint_sha256 {
        return Err("cache_package_identity_mismatch");
    }
    let current_model_sha256 = file_sha256(&root.join(MODEL_RELATIVE))?;
    if current_model_sha256 != audit.model_fingerprint_sha256 {
        return Err("cache_model_identity_mismatch");
    }
    let text = format!(
        concat!(
            "schema={}\n",
            "project={}\n",
            "source_wav_sha256={}\n",
            "source_midi_sha256={}\n",
            "quarter_frames={}\n",
            "package_fingerprint_sha256={}\n",
            "model_fingerprint_sha256={}\n",
            "feature_schema={}\n",
            "sensor_version={}\n",
            "freeze_sha256={}\n"
        ),
        PIECE_CACHE_SCHEMA,
        title,
        expected_wav_sha256,
        source_midi_sha256,
        quarter_frames,
        audit.package_fingerprint_sha256,
        audit.model_fingerprint_sha256,
        audit.feature_schema,
        audit.sensor_version,
        audit.freeze_sha256,
    );
    let destination = piece_output.join(PIECE_CACHE_IDENTITY_FILE);
    let partial = piece_output.join(format!("{PIECE_CACHE_IDENTITY_FILE}.partial"));
    fs::write(&partial, text.as_bytes()).map_err(|_| "cache_identity_write_failed")?;
    let _ = fs::remove_file(&destination);
    fs::rename(partial, destination).map_err(|_| "cache_identity_commit_failed")
}

#[cfg(windows)]
fn sync_file_if_changed(source: &Path, destination: &Path) -> std::io::Result<bool> {
    let incoming = fs::read(source)?;
    if destination.is_file() && fs::read(destination)? == incoming {
        return Ok(false);
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(destination, incoming)?;
    Ok(true)
}

#[cfg(windows)]
fn build_rust_audit_tool(target_source: &Path, cargo_target: &Path) -> bool {
    let manifest = target_source.join("Cargo.toml");
    let lock = target_source.join("Cargo.lock");
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--quiet")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&manifest)
        .env("CARGO_TARGET_DIR", cargo_target)
        .env("CARGO_INCREMENTAL", "0");
    if lock.is_file() {
        command.arg("--locked");
    }
    matches!(command.status(), Ok(status) if status.success())
}

fn not_observed(reason: &str) -> WavPitchExternalSensorBenchmarkAudit {
    WavPitchExternalSensorBenchmarkAudit {
        observed: false,
        reason: reason.to_string(),
        contract_schema: CONTRACT_SCHEMA.to_string(),
        authority: "evidence_only".to_string(),
        sensor_identity: SENSOR_IDENTITY.to_string(),
        sensor_version: SENSOR_VERSION.to_string(),
        package_fingerprint_sha256: "none".to_string(),
        model_fingerprint_sha256: "none".to_string(),
        feature_schema: "none".to_string(),
        source_wav_sha256: "none".to_string(),
        freeze_sha256: "none".to_string(),
        sensor_events: 0,
        oracle_events: 0,
        precision_80_permille: 0,
        recall_80_permille: 0,
        f1_80_permille: 0,
        chord_tone_recall_80_permille: 0,
        rearticulation_both_80_permille: 0,
        multiplicity_recall_80_permille: 0,
        bass_recall_80_permille: 0,
        high_recall_80_permille: 0,
        false_sensor_events_80: 0,
        harmonic_explainable_false_80: 0,
        deterministic_repeat: "not_probed".to_string(),
        windows_offline_runtime: "not_qualified".to_string(),
        setup_network_used: "unknown".to_string(),
        original_frame_mapping: "none".to_string(),
        midi_authority: "none_evaluation_only_post_freeze".to_string(),
        canonical_source_event_promotion: "none".to_string(),
    }
}

#[cfg(windows)]
fn read_kv_audit(path: &Path) -> WavPitchExternalSensorBenchmarkAudit {
    let Ok(text) = fs::read_to_string(path) else {
        return not_observed("benchmark_readback_missing");
    };
    let values = parse_kv(&text);
    if values.get("status").map(String::as_str) != Some("observed") {
        return not_observed("benchmark_readback_status_not_observed");
    }
    if values.get("contract_schema").map(String::as_str) != Some(CONTRACT_SCHEMA)
        || values.get("sensor_identity").map(String::as_str) != Some(SENSOR_IDENTITY)
        || values.get("sensor_version").map(String::as_str) != Some(SENSOR_VERSION)
    {
        return not_observed("benchmark_provenance_mismatch");
    }

    WavPitchExternalSensorBenchmarkAudit {
        observed: true,
        reason: "external_sensor_piece_evidence_observed".to_string(),
        contract_schema: get(&values, "contract_schema"),
        authority: get(&values, "authority"),
        sensor_identity: get(&values, "sensor_identity"),
        sensor_version: get(&values, "sensor_version"),
        package_fingerprint_sha256: get(&values, "package_fingerprint_sha256"),
        model_fingerprint_sha256: get(&values, "model_fingerprint_sha256"),
        feature_schema: get(&values, "feature_schema"),
        source_wav_sha256: get(&values, "source_wav_sha256"),
        freeze_sha256: get(&values, "freeze_sha256"),
        sensor_events: get_usize(&values, "sensor_events"),
        oracle_events: get_usize(&values, "oracle_events"),
        precision_80_permille: get_u64(&values, "precision_80_permille"),
        recall_80_permille: get_u64(&values, "recall_80_permille"),
        f1_80_permille: get_u64(&values, "f1_80_permille"),
        chord_tone_recall_80_permille: get_u64(&values, "chord_tone_recall_80_permille"),
        rearticulation_both_80_permille: get_u64(&values, "rearticulation_both_80_permille"),
        multiplicity_recall_80_permille: get_u64(&values, "multiplicity_recall_80_permille"),
        bass_recall_80_permille: get_u64(&values, "bass_recall_80_permille"),
        high_recall_80_permille: get_u64(&values, "high_recall_80_permille"),
        false_sensor_events_80: get_usize(&values, "false_sensor_events_80"),
        harmonic_explainable_false_80: get_usize(&values, "harmonic_explainable_false_80"),
        deterministic_repeat: get(&values, "deterministic_repeat"),
        windows_offline_runtime: get(&values, "windows_offline_runtime"),
        setup_network_used: get(&values, "setup_network_used"),
        original_frame_mapping: get(&values, "original_frame_mapping"),
        midi_authority: get(&values, "midi_authority"),
        canonical_source_event_promotion: get(&values, "canonical_source_event_promotion"),
    }
}

#[cfg(any(windows, test))]
fn parse_kv(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
}

#[cfg(windows)]
fn get(values: &BTreeMap<String, String>, key: &str) -> String {
    values
        .get(key)
        .cloned()
        .unwrap_or_else(|| "missing".to_string())
}

#[cfg(any(windows, test))]
fn get_u64(values: &BTreeMap<String, String>, key: &str) -> u64 {
    values
        .get(key)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
}

#[cfg(windows)]
fn get_usize(values: &BTreeMap<String, String>, key: &str) -> usize {
    values
        .get(key)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kv_readback_preserves_rust_evidence_only_authority() {
        let text = concat!(
            "status=observed\n",
            "contract_schema=wav_pitch_sensor_qualification_v1\n",
            "authority=evidence_only\n",
            "sensor_identity=SpotifyBasicPitchOnnxRust\n",
            "sensor_version=ICASSP2022-ONNX\n",
            "precision_80_permille=930\n",
        );
        let values = parse_kv(text);
        assert_eq!(
            values.get("authority").map(String::as_str),
            Some("evidence_only")
        );
        assert_eq!(get_u64(&values, "precision_80_permille"), 930);
    }

    #[test]
    fn missing_runtime_has_no_midi_or_productive_sensor_authority() {
        let audit = not_observed("test");
        assert!(!audit.observed);
        assert_eq!(audit.midi_authority, "none_evaluation_only_post_freeze");
        assert_eq!(audit.canonical_source_event_promotion, "none");
    }
}
