use super::take_change_control;
use std::{
    env,
    fs,
    path::{Path, PathBuf},
};


pub(super) fn manifest_dir() -> Result<PathBuf, String> {
    env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .map_err(|error| error.to_string())
}

pub(super) fn read_cargo_version_for_gate(path: &Path) -> Result<String, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("Cargo.toml fuer Take-Change-Control nicht lesbar: {error}"))?;
    let package_section = text
        .split("[package]")
        .nth(1)
        .and_then(|tail| tail.split("\n[").next())
        .ok_or_else(|| "Cargo.toml [package] fehlt".to_string())?;
    for line in package_section.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("version") {
            let Some((_, rhs)) = value.split_once('=') else {
                continue;
            };
            let version = rhs.trim().trim_matches('"');
            if !version.is_empty() {
                return Ok(version.to_string());
            }
        }
    }
    Err("Cargo.toml package.version fehlt".to_string())
}

pub(super) struct KodexBuildContext {
    pub(super) manifest_dir: PathBuf,
    pub(super) source_root: PathBuf,
    pub(super) kodex_gate_core_source_root: PathBuf,
    pub(super) baseline_path: PathBuf,
    pub(super) feedback_gate_path: PathBuf,
    pub(super) feedback_admission_path: PathBuf,
    pub(super) feedback_observable_sink_allowlist_path: PathBuf,
    pub(super) kodex_boundary_adversarial_matrix_path: PathBuf,
    pub(super) write_bypass_baseline_path: PathBuf,
    pub(super) thirty_surface_batch_manifest_path: PathBuf,

    pub(super) scanner_text: String,
    pub(super) baseline_text: String,
    pub(super) feedback_gate_text: String,
    pub(super) feedback_transport_text: String,
    pub(super) feedback_admission_text: String,
    pub(super) feedback_observable_sink_allowlist_text: String,
    pub(super) kodex_boundary_adversarial_matrix_text: String,
    pub(super) identity_gate_trust_anchor_text: String,
    pub(super) take_change_control_text: String,
    pub(super) semantic_capability_text: String,
    pub(super) kodex_capability_retirements_text: String,
    pub(super) multi_revision_evasion_text: String,
    pub(super) shared_sha256_text: String,
    pub(super) rust_source_scan_primitives_text: String,
    pub(super) rust_source_fact_primitives_text: String,
    pub(super) rust_source_scan_primitives_contract_text: String,
    pub(super) productive_write_path_scanner_text: String,
    pub(super) identity_debt_inventory_text: String,
    pub(super) productive_write_path_inventory_text: String,
    pub(super) global_identity_debt_inventory_test_text: String,
    pub(super) take_bootstrap_text: String,
    pub(super) launcher_text: String,
    pub(super) launcher_build_control_text: String,
    pub(super) launcher_assistant_export_text: String,
    pub(super) launcher_neuro_evidence_export_text: String,
    pub(super) public_artifact_recovery_text: String,
    pub(super) formal_green_contract_text: String,
    pub(super) run_cmd_text: String,
    pub(super) kodex_gate_core_ownership_text: String,
}

impl KodexBuildContext {
    pub(super) fn load(manifest_dir: PathBuf) -> Result<Self, String> {
        let source_root = manifest_dir.join("src");
        let kodex_gate_core_source_root =
            manifest_dir.join("tools/RUST_KODEX_GATE_CORE/src");
        let baseline_path = manifest_dir.join("src/build_gate/identity_debt_baseline.tsv");
        let identity_scanner_path = manifest_dir.join("src/build_gate/identity_debt_scan.rs");
        let feedback_gate_path = manifest_dir.join("src/feedback_structuralization_gate.rs");
        let feedback_transport_path = manifest_dir.join("src/feedback_proof_transport.rs");
        let feedback_admission_path =
            manifest_dir.join("src/build_gate/feedback_rule_admission.tsv");
        let feedback_observable_sink_allowlist_path =
            manifest_dir.join("src/build_gate/feedback_observable_sink_allowlist.tsv");
        let kodex_boundary_adversarial_matrix_path =
            manifest_dir.join("src/build_gate/kodex_boundary_adversarial_matrix.tsv");
        let identity_gate_trust_anchor_path =
            manifest_dir.join("src/identity_gate_trust_anchor.rs");
        let take_change_control_path = manifest_dir.join("src/take_change_control.rs");
        let managed_source_tree_definition_path =
            manifest_dir.join("src/build_gate/managed_source_tree_definition.rs");
        let semantic_capability_path =
            manifest_dir.join("src/build_gate/semantic_capability_diff.rs");
        let kodex_capability_retirements_path =
            manifest_dir.join("src/build_gate/kodex_capability_retirements.txt");
        let multi_revision_evasion_path =
            manifest_dir.join("src/build_gate/multi_revision_evasion.rs");
        let shared_sha256_path = manifest_dir.join("src/build_gate/sha256.rs");
        let rust_source_scan_primitives_path =
            manifest_dir.join("tools/RUST_KODEX_GATE_CORE/src/rust_source_scan_primitives.rs");
        let rust_source_fact_primitives_path =
            manifest_dir.join("tools/RUST_KODEX_GATE_CORE/src/rust_source_fact_primitives.rs");
        let rust_source_scan_primitives_contract_path =
            manifest_dir.join("src/build_gate/rust_source_scan_primitives.rs");
        let productive_write_path_scanner_path =
            manifest_dir.join("src/build_gate/productive_write_path_scan.rs");
        let identity_debt_inventory_path = manifest_dir.join("src/identity_debt_inventory.rs");
        let productive_write_path_inventory_path =
            manifest_dir.join("src/productive_write_path_inventory.rs");
        let global_identity_debt_inventory_test_path =
            manifest_dir.join("tests/global_identity_debt_inventory.rs");
        let take_bootstrap_path = manifest_dir.join("src/bin/etuschlopf_take_bootstrap.rs");
        let launcher_path = manifest_dir.join("src/bin/etuschlopf_launcher.rs");
        let launcher_build_control_path =
            manifest_dir.join("src/bin/etuschlopf_launcher/launcher_build_control.rs");
        let launcher_assistant_export_path =
            manifest_dir.join("src/bin/etuschlopf_launcher/assistant_export.rs");
        let launcher_neuro_evidence_export_path =
            manifest_dir.join("src/bin/etuschlopf_launcher/neuro_evidence_export.rs");
        let public_artifact_recovery_path =
            manifest_dir.join("src/kodex_control_plane/public_artifact_recovery.rs");
        let formal_green_contract_path =
            manifest_dir.join("src/build_gate/formal_green_contract.rs");
        let run_cmd_path = manifest_dir.join("run.cmd");
        let take_change_manifest_path =
            manifest_dir.join(take_change_control::CHANGE_MANIFEST_RELATIVE_PATH);
        let write_bypass_baseline_path =
            manifest_dir.join("src/build_gate/productive_write_bypass_baseline.tsv");
        let thirty_surface_batch_manifest_path =
            manifest_dir.join("src/build_gate/thirty_surface_batch_manifest.tsv");
        let thirty_surface_batch_scanner_path =
            manifest_dir.join("src/build_gate/thirty_surface_batch_scan.rs");
        let kodex_gate_core_ownership_path =
            manifest_dir.join("tools/RUST_KODEX_GATE_CORE/OWNERSHIP.txt");

        for path in [
            &baseline_path,
            &identity_scanner_path,
            &feedback_gate_path,
            &feedback_transport_path,
            &feedback_admission_path,
            &feedback_observable_sink_allowlist_path,
            &kodex_boundary_adversarial_matrix_path,
            &identity_gate_trust_anchor_path,
            &take_change_control_path,
            &managed_source_tree_definition_path,
            &semantic_capability_path,
            &multi_revision_evasion_path,
            &shared_sha256_path,
            &rust_source_scan_primitives_path,
            &rust_source_fact_primitives_path,
            &rust_source_scan_primitives_contract_path,
            &productive_write_path_scanner_path,
            &identity_debt_inventory_path,
            &productive_write_path_inventory_path,
            &global_identity_debt_inventory_test_path,
            &take_bootstrap_path,
            &launcher_path,
            &launcher_assistant_export_path,
            &launcher_neuro_evidence_export_path,
            &public_artifact_recovery_path,
            &formal_green_contract_path,
            &run_cmd_path,
            &take_change_manifest_path,
            &write_bypass_baseline_path,
            &thirty_surface_batch_manifest_path,
            &thirty_surface_batch_scanner_path,
            &kodex_gate_core_ownership_path,
        ] {
            println!("cargo:rerun-if-changed={}", path.display());
        }
        emit_recursive_rerun(&source_root)?;
        emit_recursive_rerun(&kodex_gate_core_source_root)?;

        let scanner_text = read(&identity_scanner_path)?;
        let baseline_text = read(&baseline_path)?;
        let feedback_gate_text = read(&feedback_gate_path)?;
        let feedback_transport_text = read(&feedback_transport_path)?;
        let feedback_admission_text = read(&feedback_admission_path)?;
        let feedback_observable_sink_allowlist_text =
            read(&feedback_observable_sink_allowlist_path)?;
        let kodex_boundary_adversarial_matrix_text =
            read(&kodex_boundary_adversarial_matrix_path)?;
        let identity_gate_trust_anchor_text = read(&identity_gate_trust_anchor_path)?;
        let take_change_control_text = read(&take_change_control_path)?;
        let semantic_capability_text = read(&semantic_capability_path)?;
        let kodex_capability_retirements_text = read(&kodex_capability_retirements_path)?;
        let multi_revision_evasion_text = read(&multi_revision_evasion_path)?;
        let shared_sha256_text = read(&shared_sha256_path)?;
        let rust_source_scan_primitives_text = read(&rust_source_scan_primitives_path)?;
        let rust_source_fact_primitives_text = read(&rust_source_fact_primitives_path)?;
        let rust_source_scan_primitives_contract_text =
            read(&rust_source_scan_primitives_contract_path)?;
        let productive_write_path_scanner_text = read(&productive_write_path_scanner_path)?;
        let identity_debt_inventory_text = read(&identity_debt_inventory_path)?;
        let productive_write_path_inventory_text = read(&productive_write_path_inventory_path)?;
        let global_identity_debt_inventory_test_text =
            read(&global_identity_debt_inventory_test_path)?;
        let take_bootstrap_text = read(&take_bootstrap_path)?;
        let launcher_text = read(&launcher_path)?;
        let launcher_build_control_text = read(&launcher_build_control_path)?;
        let launcher_assistant_export_text = read(&launcher_assistant_export_path)?;
        let launcher_neuro_evidence_export_text = read(&launcher_neuro_evidence_export_path)?;
        let public_artifact_recovery_text = read(&public_artifact_recovery_path)?;
        let formal_green_contract_text = read(&formal_green_contract_path)?;
        let run_cmd_text = read(&run_cmd_path)?;
        let kodex_gate_core_ownership_text = read(&kodex_gate_core_ownership_path)?;

        Ok(Self {
            manifest_dir,
            source_root,
            kodex_gate_core_source_root,
            baseline_path,
            feedback_gate_path,
            feedback_admission_path,
            feedback_observable_sink_allowlist_path,
            kodex_boundary_adversarial_matrix_path,
            write_bypass_baseline_path,
            thirty_surface_batch_manifest_path,
            scanner_text,
            baseline_text,
            feedback_gate_text,
            feedback_transport_text,
            feedback_admission_text,
            feedback_observable_sink_allowlist_text,
            kodex_boundary_adversarial_matrix_text,
            identity_gate_trust_anchor_text,
            take_change_control_text,
            semantic_capability_text,
            kodex_capability_retirements_text,
            multi_revision_evasion_text,
            shared_sha256_text,
            rust_source_scan_primitives_text,
            rust_source_fact_primitives_text,
            rust_source_scan_primitives_contract_text,
            productive_write_path_scanner_text,
            identity_debt_inventory_text,
            productive_write_path_inventory_text,
            global_identity_debt_inventory_test_text,
            take_bootstrap_text,
            launcher_text,
            launcher_build_control_text,
            launcher_assistant_export_text,
            launcher_neuro_evidence_export_text,
            public_artifact_recovery_text,
            formal_green_contract_text,
            run_cmd_text,
            kodex_gate_core_ownership_text,
        })
    }
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| error.to_string())
}

fn emit_recursive_rerun(path: &Path) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let child = entry.path();
        if child.is_dir() {
            emit_recursive_rerun(&child)?;
        } else if child.extension().and_then(|value| value.to_str()) == Some("rs") {
            println!("cargo:rerun-if-changed={}", child.display());
        }
    }
    Ok(())
}
