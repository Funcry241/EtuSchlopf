//! Audit-only execution and readback for one qualified external WAV pitch/event sensor candidate.
//!
//! The candidate owns evidence only. Basic Pitch ONNX is executed by a dedicated Rust audit
//! crate. No Python runtime, MIDI transport, CanonicalSourceEvents, staff/hand, phrase, writer
//! or productive source authority is introduced here.

#[cfg(any(windows, test))]
use std::collections::BTreeMap;
#[cfg(any(windows, test))]
use std::fs;
#[cfg(any(windows, test))]
use std::path::Path;
#[cfg(windows)]
use std::process::Command;
#[cfg(windows)]
use std::time::Instant;

const CONTRACT_SCHEMA: &str = "wav_pitch_sensor_qualification_v1";
const SENSOR_IDENTITY: &str = "SpotifyBasicPitchOnnxRust";
const SENSOR_VERSION: &str = "ICASSP2022-ONNX";
#[cfg(any(windows, test))]
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
#[cfg(any(windows, test))]
const MODEL_RELATIVE: &str =
    "target/wav_pitch_external_sensor_audit/runtime/model/basic_pitch_icassp2022_nmp.onnx";
#[cfg(any(windows, test))]
const PIECE_CACHE_IDENTITY_FILE: &str = "EVIDENCE_REUSE_IDENTITY_V1.txt";
#[cfg(any(windows, test))]
const PIECE_CACHE_SCHEMA: &str = "wav_pitch_external_sensor_evidence_reuse_v1";

#[derive(Debug, Clone)]
pub(crate) struct WavPitchExternalSensorBenchmarkAudit {
    pub(crate) observed: bool,
    pub(crate) frozen_projection: Option<String>,
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

pub(crate) fn build_for_report(
    wav: &super::ReportWav<'_>,
) -> Option<WavPitchExternalSensorBenchmarkAudit> {
    let title = wav.title?.trim();
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
        let captured = match wav.captured() {
            Ok(captured) => captured,
            Err(reason) => return Some(not_observed(reason)),
        };
        let timing = match wav.pitch_timing() {
            Ok(timing) => timing,
            Err(reason) => return Some(not_observed(reason)),
        };
        let expected_wav_sha256 = captured.sha256.as_str();

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
        for (tool_source, relative) in [
            (&tool_source, "Cargo.toml"),
            (&tool_source, "BASIC_PITCH_NOTICE.md"),
            (&tool_source, "src/main.rs"),
            (&current, "src/source_time.rs"),
            (&current, "src/notation_time.rs"),
            (&current, "src/inferred_performance_timing.rs"),
        ] {
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

        if let Some(audit) = try_reuse_piece_evidence(
            &current,
            title,
            &piece_output,
            &latest,
            timing.quarter_frames,
            expected_wav_sha256,
        ) {
            eprintln!(
                "ETUSCHLOPF: EvidenceRuntimeReuse: sensor=BasicPitch cache=hit cache_phase_ms={} total_benchmark_ms={}",
                evidence_started.elapsed().as_millis(),
                benchmark_started.elapsed().as_millis()
            );
            return Some(audit);
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
            .arg(expected_wav_sha256)
            .arg("--cargo-bootstrap-mode")
            .arg(cargo_bootstrap_mode)
            .status();
        let Ok(status) = status else {
            return Some(not_observed("rust_onnx_sensor_process_start_failed"));
        };
        if !status.success() {
            return Some(not_observed("rust_onnx_sensor_benchmark_process_failed"));
        }
        let audit = read_fresh_piece_evidence(
            &current, title, &piece_output, timing.quarter_frames, expected_wav_sha256,
        );
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

#[cfg(any(windows, test))]
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
    for relative in [
        "src/midi.rs", "src/source_event.rs", "src/source_time.rs", "src/notation_time.rs",
        "src/inferred_performance_timing.rs",
    ] {
        let source = fs::read(root.join(relative)).map_err(|_| "sensor_shared_source_unreadable")?;
        bytes.extend_from_slice(relative.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(&source);
        bytes.push(0);
    }
    Ok(crate::sha256::hex(&bytes))
}

#[cfg(any(windows, test))]
fn file_sha256(path: &Path) -> Result<String, &'static str> {
    fs::read(path)
        .map(|bytes| crate::sha256::hex(&bytes))
        .map_err(|_| "cache_identity_input_unreadable")
}

#[cfg(any(windows, test))]
fn read_fresh_piece_evidence(
    root: &Path,
    title: &str,
    piece_output: &Path,
    quarter_frames: u64,
    expected_wav_sha256: &str,
) -> WavPitchExternalSensorBenchmarkAudit {
    let Ok(text) = fs::read_to_string(piece_output.join("audit_kv.txt")) else {
        return not_observed("benchmark_readback_missing");
    };
    let values = parse_kv(&text);
    let mut audit = audit_from_values(&values);
    if !audit.observed {
        return audit;
    }
    if !matches_report(&values, title, quarter_frames, expected_wav_sha256) {
        return not_observed("benchmark_provenance_mismatch");
    }
    if let Ok(identity) = piece_cache_identity(
        root, title, quarter_frames, expected_wav_sha256, &audit,
    ) {
        audit.frozen_projection = read_bound_projection(
            piece_output, &values,
        );
        if audit.frozen_projection.is_some() {
            let _ = write_piece_cache_identity(piece_output, &identity);
        }
    }
    audit
}

#[cfg(any(windows, test))]
fn try_reuse_piece_evidence(
    root: &Path,
    title: &str,
    piece_output: &Path,
    latest: &Path,
    quarter_frames: u64,
    expected_wav_sha256: &str,
) -> Option<WavPitchExternalSensorBenchmarkAudit> {
    let identity_path = piece_output.join(PIECE_CACHE_IDENTITY_FILE);
    let audit_path = piece_output.join("audit_kv.txt");
    let identity_text = fs::read_to_string(&identity_path).ok()?;
    let identity = parse_kv(&identity_text);
    if identity.get("schema").map(String::as_str) != Some(PIECE_CACHE_SCHEMA)
        || identity.get("project").map(String::as_str) != Some(title)
        || identity.get("source_wav_sha256").map(String::as_str) != Some(expected_wav_sha256)
        || get_u64(&identity, "quarter_frames") != quarter_frames
    {
        return None;
    }

    let midi_path = root.join(format!("{title}.mid"));
    let current_midi_sha256 = file_sha256(&midi_path).ok()?;
    if identity.get("source_midi_sha256").map(String::as_str)
        != Some(current_midi_sha256.as_str())
    {
        return None;
    }
    let current_package_sha256 = current_sensor_package_fingerprint(root).ok()?;
    if identity
        .get("package_fingerprint_sha256")
        .map(String::as_str)
        != Some(current_package_sha256.as_str())
    {
        return None;
    }
    let model_path = root.join(MODEL_RELATIVE);
    let current_model_sha256 = file_sha256(&model_path).ok()?;
    if identity.get("model_fingerprint_sha256").map(String::as_str)
        != Some(current_model_sha256.as_str())
    {
        return None;
    }

    let audit_text = fs::read_to_string(&audit_path).ok()?;
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
            return None;
        }
    }
    let mut result = audit_from_values(&audit);
    if !result.observed || !matches_report(&audit, title, quarter_frames, expected_wav_sha256) {
        return None;
    }
    result.frozen_projection = Some(read_bound_projection(
        piece_output, &audit,
    )?);
    fs::write(latest, audit_text.as_bytes()).ok()?;
    Some(result)
}

#[cfg(any(windows, test))]
fn matches_report(
    audit: &BTreeMap<String, String>,
    title: &str,
    quarter_frames: u64,
    expected_wav_sha256: &str,
) -> bool {
    audit.get("project").is_some_and(|project| project == title)
        && audit.get("audio_pulse_quarter_frames").and_then(|frames| frames.parse().ok())
            == Some(quarter_frames)
        && audit.get("source_wav_sha256").is_some_and(|sha| sha == expected_wav_sha256)
}

#[cfg(any(windows, test))]
fn read_bound_projection(
    piece_output: &Path,
    audit: &BTreeMap<String, String>,
) -> Option<String> {
    let manifest = fs::read_to_string(piece_output.join("sensor_freeze_manifest.txt")).ok()?;
    let fingerprint = crate::sha256::hex(manifest.as_bytes());
    let marker = fs::read_to_string(piece_output.join("SENSOR_OUTPUT_FROZEN.sha256")).ok()?;
    if audit.get("freeze_sha256")? != &fingerprint || marker.trim() != fingerprint {
        return None;
    }
    let freeze = parse_kv(&manifest);
    for key in [
        "contract_schema", "sensor_identity", "sensor_version", "package_fingerprint_sha256",
        "model_fingerprint_sha256", "feature_schema", "source_wav_sha256",
    ] {
        if audit.get(key)? != freeze.get(key)? {
            return None;
        }
    }
    // Hash and return the same bytes; the report never reopens this path.
    let projection = fs::read_to_string(piece_output.join("sensor_events.frozen.txt")).ok()?;
    (freeze.get("frozen_event_projection_sha256")? == &crate::sha256::hex(projection.as_bytes()))
        .then_some(projection)
}

#[cfg(any(windows, test))]
fn piece_cache_identity(
    root: &Path,
    title: &str,
    quarter_frames: u64,
    expected_wav_sha256: &str,
    audit: &WavPitchExternalSensorBenchmarkAudit,
) -> Result<String, &'static str> {
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
    Ok(text)
}

#[cfg(any(windows, test))]
fn write_piece_cache_identity(piece_output: &Path, text: &str) -> Result<(), &'static str> {
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
        frozen_projection: None,
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

#[cfg(any(windows, test))]
fn audit_from_values(values: &BTreeMap<String, String>) -> WavPitchExternalSensorBenchmarkAudit {
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
        frozen_projection: None,
        reason: "external_sensor_piece_evidence_observed".to_string(),
        contract_schema: get(values, "contract_schema"),
        authority: get(values, "authority"),
        sensor_identity: get(values, "sensor_identity"),
        sensor_version: get(values, "sensor_version"),
        package_fingerprint_sha256: get(values, "package_fingerprint_sha256"),
        model_fingerprint_sha256: get(values, "model_fingerprint_sha256"),
        feature_schema: get(values, "feature_schema"),
        source_wav_sha256: get(values, "source_wav_sha256"),
        freeze_sha256: get(values, "freeze_sha256"),
        sensor_events: get_usize(values, "sensor_events"),
        oracle_events: get_usize(values, "oracle_events"),
        precision_80_permille: get_u64(values, "precision_80_permille"),
        recall_80_permille: get_u64(values, "recall_80_permille"),
        f1_80_permille: get_u64(values, "f1_80_permille"),
        chord_tone_recall_80_permille: get_u64(values, "chord_tone_recall_80_permille"),
        rearticulation_both_80_permille: get_u64(values, "rearticulation_both_80_permille"),
        multiplicity_recall_80_permille: get_u64(values, "multiplicity_recall_80_permille"),
        bass_recall_80_permille: get_u64(values, "bass_recall_80_permille"),
        high_recall_80_permille: get_u64(values, "high_recall_80_permille"),
        false_sensor_events_80: get_usize(values, "false_sensor_events_80"),
        harmonic_explainable_false_80: get_usize(values, "harmonic_explainable_false_80"),
        deterministic_repeat: get(values, "deterministic_repeat"),
        windows_offline_runtime: get(values, "windows_offline_runtime"),
        setup_network_used: get(values, "setup_network_used"),
        original_frame_mapping: get(values, "original_frame_mapping"),
        midi_authority: get(values, "midi_authority"),
        canonical_source_event_promotion: get(values, "canonical_source_event_promotion"),
    }
}

#[cfg(any(windows, test))]
fn parse_kv(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
}

#[cfg(any(windows, test))]
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

#[cfg(any(windows, test))]
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
    fn fresh_and_reused_sensor_projections_belong_to_one_recording() {
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!("report_sensor_evidence_{nonce}"));
        let piece = root.join("piece");
        fs::create_dir_all(&piece).unwrap();
        for name in ["Cargo.toml", "BASIC_PITCH_NOTICE.md", "src/main.rs"] {
            let path = root.join(TOOL_SOURCE_RELATIVE).join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, name).unwrap();
        }
        for name in [
            "src/midi.rs", "src/source_event.rs", "src/source_time.rs", "src/notation_time.rs",
            "src/inferred_performance_timing.rs", MODEL_RELATIVE, "synthetic.mid",
        ] {
            let path = root.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, name).unwrap();
        }
        let recording = crate::sha256::hex(b"one synthetic recording");
        let package = current_sensor_package_fingerprint(&root).unwrap();
        let model = file_sha256(&root.join(MODEL_RELATIVE)).unwrap();
        let projection = "schema=basic_pitch_rust_event_projection_v1\n0|60|1|2|0|1|2\n";
        let projection_hash = crate::sha256::hex(projection.as_bytes());
        let provenance = format!(
            "contract_schema={CONTRACT_SCHEMA}\nsensor_identity={SENSOR_IDENTITY}\nsensor_version={SENSOR_VERSION}\npackage_fingerprint_sha256={package}\nmodel_fingerprint_sha256={model}\nfeature_schema=synthetic\nsource_wav_sha256={recording}\n"
        );
        let manifest = format!("{provenance}frozen_event_projection_sha256={projection_hash}\n");
        let freeze = crate::sha256::hex(manifest.as_bytes());
        fs::write(piece.join("sensor_freeze_manifest.txt"), manifest).unwrap();
        fs::write(piece.join("SENSOR_OUTPUT_FROZEN.sha256"), &freeze).unwrap();
        let path = piece.join("sensor_events.frozen.txt");
        fs::write(&path, projection).unwrap();
        fs::write(piece.join("audit_kv.txt"), format!(
            "status=observed\nproject=synthetic\naudio_pulse_quarter_frames=31\n{provenance}freeze_sha256={freeze}\n"
        )).unwrap();
        let latest = root.join("latest_audit_kv.txt");
        fs::write(&latest, "status=another_project").unwrap();
        let fresh = read_fresh_piece_evidence(&root, "synthetic", &piece, 31, &recording);
        assert!(fresh.observed);
        assert_eq!(fresh.frozen_projection.as_deref(), Some(projection));
        let warm = |sha: &str, frames| {
            try_reuse_piece_evidence(&root, "synthetic", &piece, &latest, frames, sha)
        };
        let hit = warm(&recording, 31).unwrap();
        assert_eq!(hit.frozen_projection, fresh.frozen_projection);
        let wrong = read_fresh_piece_evidence(&root, "synthetic", &piece, 31, "different");
        assert!(!wrong.observed);
        assert_eq!(wrong.reason, "benchmark_provenance_mismatch");
        assert!(wrong.frozen_projection.is_none());
        assert!(warm("different", 31).is_none());
        assert!(warm(&recording, 30).is_none());
        fs::write(&path, "other bytes").unwrap();
        assert!(warm(&recording, 31).is_none());
        let own_audit = read_fresh_piece_evidence(&root, "synthetic", &piece, 31, &recording);
        assert!(own_audit.observed);
        assert!(own_audit.frozen_projection.is_none());
        assert_eq!(hit.frozen_projection.as_deref(), Some(projection));
        fs::write(&path, projection).unwrap();
        fs::write(root.join("synthetic.mid"), "changed").unwrap();
        assert!(warm(&recording, 31).is_none());
        fs::remove_dir_all(root).unwrap();
    }

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
