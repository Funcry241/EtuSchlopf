// r860 KodexDuplicateFullBytePinRetirementAndProofDependencyEssenceClosure: whole-byte identity remains pinned once by identity_gate_trust_anchor; SourceIntegrity retains its unique negative-check contract pin and all semantic owner/coupling checks.
// r378 Pin-Koordination: Warning-Free Kodex-Clippy-Cleanup aktualisiert ausschliesslich die Byte-Pins der mechanisch bereinigten gemeinsamen SHA256- und RustSourceScan-Primitives; Contract-Fingerprint und Gate-Semantik bleiben unveraendert.
use super::rust_source_scan_primitives::fnv1a64;

pub(super) struct SourceIntegrityInputs<'a> {
    pub(super) scanner_text: &'a str,
    pub(super) productive_write_path_scanner_text: &'a str,
    pub(super) semantic_capability_text: &'a str,
    pub(super) take_change_control_text: &'a str,
    pub(super) identity_debt_inventory_text: &'a str,
    pub(super) productive_write_path_inventory_text: &'a str,
    pub(super) global_identity_debt_inventory_test_text: &'a str,
    pub(super) take_bootstrap_text: &'a str,
    pub(super) rust_source_scan_primitives_text: &'a str,
    pub(super) rust_source_fact_primitives_text: &'a str,
    pub(super) rust_source_scan_primitives_contract_text: &'a str,
    pub(super) identity_gate_trust_anchor_text: &'a str,
    pub(super) launcher_text: &'a str,
    pub(super) kodex_gate_core_ownership_text: &'a str,
    pub(super) multi_revision_evasion_text: &'a str,
    pub(super) shared_sha256_text: &'a str,
    pub(super) public_artifact_recovery_text: &'a str,
}

pub(super) fn validate_source_integrity_kodex(
    inputs: SourceIntegrityInputs<'_>,
) -> Result<(), String> {
    let SourceIntegrityInputs {
        scanner_text,
        productive_write_path_scanner_text,
        semantic_capability_text,
        take_change_control_text,
        identity_debt_inventory_text,
        productive_write_path_inventory_text,
        global_identity_debt_inventory_test_text,
        take_bootstrap_text,
        rust_source_scan_primitives_text,
        rust_source_fact_primitives_text,
        rust_source_scan_primitives_contract_text,
        identity_gate_trust_anchor_text,
        launcher_text,
        kodex_gate_core_ownership_text,
        multi_revision_evasion_text,
        shared_sha256_text,
        public_artifact_recovery_text,
    } = inputs;

    // Exact source-byte identity for the live Kodex owners below is already
    // pinned by identity_gate_trust_anchor. SourceIntegrity owns only the
    // additional path-stable negative-check contract that is not represented there.
    const PINNED_RUST_SOURCE_SCAN_PRIMITIVES_CONTRACT_FNV1A64: u64 = 0xf5d4_9eab_bcf5_b1bb;
    let rust_source_scan_primitives_contract_fingerprint =
        fnv1a64(rust_source_scan_primitives_contract_text.as_bytes());

    if !multi_revision_evasion_text.contains("use crate::sha256;")
        || !multi_revision_evasion_text.contains("sha256::hex(&bytes)")
        || multi_revision_evasion_text.contains("mod sha256;")
        || multi_revision_evasion_text.contains("fn ledger_sha256")
        || !public_artifact_recovery_text.contains("use crate::sha256;")
        || !public_artifact_recovery_text.contains("sha256::hex(&pdf)")
        || !public_artifact_recovery_text.contains("sha256::hex(bytes)")
        || public_artifact_recovery_text.contains("mod sha256;")
        || public_artifact_recovery_text.contains("fn sha256_hex")
        || public_artifact_recovery_text.contains("fn sha256(input")
        || !take_bootstrap_text.contains("#[path = \"../build_gate/sha256.rs\"]\nmod sha256;")
        || take_bootstrap_text.matches("mod sha256;").count() != 1
        || !launcher_text.contains("#[path = \"../build_gate/sha256.rs\"]\nmod sha256;")
        || launcher_text.matches("mod sha256;").count() != 1
        || !shared_sha256_text.contains("fn known_vectors_cover_empty_short_padding_and_multiple_blocks")
        || !shared_sha256_text.contains("cf5b16a778af8380036ce59e7b0492370b249b11e8f07a51afac45037afee9d1")
    {
        return Err(
            "SHARED SHA-256 OWNER BINDING REGRESSION: ledger and recovery must use the one pinned primitive, retain no private implementation and keep empty, short, padding and multi-block vectors."
                .to_string(),
        );
    }
    if rust_source_scan_primitives_contract_fingerprint
        != PINNED_RUST_SOURCE_SCAN_PRIMITIVES_CONTRACT_FNV1A64
    {
        return Err(format!(
            "RUST SOURCE SCAN NEGATIVE-CHECK CONTRACT CHANGED: expected FNV1A64 {PINNED_RUST_SOURCE_SCAN_PRIMITIVES_CONTRACT_FNV1A64:016x}, got {rust_source_scan_primitives_contract_fingerprint:016x}. Existing negative kodex checks remain path-bound until an explicit relocation policy exists."
        ));
    }
    if !scanner_text.contains("use super::rust_source_scan_primitives::{")
        || !scanner_text.contains("collect_rust_files, line_number_at, line_starts, mask_non_code, normalize_path,")
        || !scanner_text.contains("token_boundary_after, token_boundary_before,")
        || scanner_text.contains("fn mask_non_code(")
        || scanner_text.contains("fn collect_rust_files(")
        || scanner_text.contains("fn normalize_path(")
        || scanner_text.contains("fn line_starts(")
        || scanner_text.contains("fn line_number_at(")
        || scanner_text.contains("fn token_boundary_before(")
        || scanner_text.contains("fn token_boundary_after(")
        || !productive_write_path_scanner_text
            .contains("use super::rust_source_scan_primitives::{")
        || !productive_write_path_scanner_text.contains(
            "use crate::identity_debt_inventory::rust_source_scan_primitives::{",
        )
        || !productive_write_path_scanner_text.contains("collect_rust_files, fnv1a64_update, line_number_at, line_starts, mask_non_code,")
        || !productive_write_path_scanner_text.contains("normalize_path, token_boundary_after, token_boundary_before, FNV1A64_OFFSET_BASIS,")
        || productive_write_path_scanner_text.contains("fn mask_non_code(")
        || productive_write_path_scanner_text.contains("fn collect_rust_files(")
        || productive_write_path_scanner_text.contains("fn normalize_path(")
        || productive_write_path_scanner_text.contains("fn line_starts(")
        || productive_write_path_scanner_text.contains("fn line_number_at(")
        || productive_write_path_scanner_text.contains("fn token_boundary_before(")
        || productive_write_path_scanner_text.contains("fn token_boundary_after(")
        || productive_write_path_scanner_text.contains("0xcbf29ce484222325")
        || productive_write_path_scanner_text.contains("0x100000001b3")
        || !semantic_capability_text.contains(
            "use crate::rust_source_scan_primitives::{fnv1a64, mask_non_code};",
        )
        || semantic_capability_text.contains("fn mask_non_code(")
        || semantic_capability_text.contains("fn fnv1a64(")
        || !take_change_control_text.contains(
            "use crate::rust_source_scan_primitives::fnv1a64;",
        )
        || !take_change_control_text.contains(
            "#[path = \"build_gate/semantic_capability_diff.rs\"]\nmod semantic_capability_diff;",
        )
        || take_change_control_text.contains("fn rust_code_without_comments_and_literals(")
        || take_change_control_text.contains("pub(crate) const fn fnv1a64(")
        || take_change_control_text.contains("pub const fn fnv1a64(")
        || !identity_debt_inventory_text.contains(
            "#[path = \"build_gate/rust_source_scan_primitives.rs\"]",
        )
        || !identity_debt_inventory_text.contains("identity_debt_scan_test_support")
        || !identity_debt_inventory_text.contains(
            "fn injected_absolute_measure_writer_is_detected()",
        )
        || !identity_debt_inventory_text.contains(
            "fn audit_localization_cannot_call_mutation_authority()",
        )
        || productive_write_path_inventory_text.contains("mod rust_source_scan_primitives;")
        || !global_identity_debt_inventory_test_text.contains(
            "#[path = \"../src/build_gate/rust_source_scan_primitives.rs\"]\nmod rust_source_scan_primitives;",
        )
        || !global_identity_debt_inventory_test_text.contains(
            "#[path = \"../src/build_gate/identity_debt_scan.rs\"]\nmod identity_debt_scan;",
        )
        || global_identity_debt_inventory_test_text
            .matches("mod rust_source_scan_primitives;")
            .count()
            != 1
        || !take_bootstrap_text.contains(
            "#[path = \"../build_gate/rust_source_scan_primitives.rs\"]\nmod rust_source_scan_primitives;",
        )
        || take_bootstrap_text
            .matches("mod rust_source_scan_primitives;")
            .count()
            != 1
        || !take_bootstrap_text.contains("rust_source_scan_primitives::fnv1a64(&bytes)")
        || rust_source_scan_primitives_text.contains("#[cfg(test)]")
        || rust_source_scan_primitives_text.contains("pub fn collect_rust_files(")
        || rust_source_scan_primitives_text.contains("pub fn normalize_path(")
        || rust_source_scan_primitives_text.contains("pub fn line_starts(")
        || rust_source_scan_primitives_text.contains("pub fn line_number_at(")
        || rust_source_scan_primitives_text.contains("pub fn token_boundary_before(")
        || rust_source_scan_primitives_text.contains("pub fn token_boundary_after(")
        || !rust_source_fact_primitives_text.contains("pub fn collect_rust_files(")
        || !rust_source_fact_primitives_text.contains("pub fn normalize_path(")
        || !rust_source_fact_primitives_text.contains("pub fn line_starts(")
        || !rust_source_fact_primitives_text.contains("pub fn line_number_at(")
        || !rust_source_fact_primitives_text.contains("pub fn token_boundary_before(")
        || !rust_source_fact_primitives_text.contains("pub fn token_boundary_after(")
        || rust_source_fact_primitives_text.contains("fn mask_non_code(")
        || rust_source_fact_primitives_text.contains("fn fnv1a64(")
        || rust_source_scan_primitives_text.contains(
            "fn character_literals_are_blank_but_lifetimes_and_labels_remain",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "#[path = \"../../tools/RUST_KODEX_GATE_CORE/src/rust_source_scan_primitives.rs\"]",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "#[path = \"../../tools/RUST_KODEX_GATE_CORE/src/rust_source_fact_primitives.rs\"]",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "fn character_literals_are_blank_but_lifetimes_and_labels_remain",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "fn nested_comments_strings_and_raw_strings_are_blank",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "fn fnv1a64_known_vectors_and_incremental_update_match",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "pub use implementation::*;",
        )
        || rust_source_scan_primitives_contract_text.contains("pub use implementation::{")
        || !rust_source_scan_primitives_contract_text.contains(
            "pub use source_facts::{",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "collect_rust_files, line_number_at, line_starts, normalize_path, token_boundary_after,",
        )
        || !rust_source_scan_primitives_contract_text.contains(
            "fn policy_free_source_facts_preserve_existing_scanner_semantics",
        )
    {
        return Err(
            "RUST SOURCE FACT OWNER BINDING REGRESSION: the tools owner must contain the only policy-free masking, hashing and source-fact implementations in separate compile units, the path-stable negative-check facade must project the byte-pinned base owner as one complete import surface, all existing path-bound negative checks must remain executable at their authorized source paths, and the take bootstrap must consume source facts only through DeadCodeDetectionIntegrity."
                .to_string(),
        );
    }
    if !identity_gate_trust_anchor_text.contains(
        "include_bytes!(\"../tools/RUST_KODEX_GATE_CORE/src/rust_source_scan_primitives.rs\")",
    )
        || !identity_gate_trust_anchor_text.contains(
            "include_bytes!(\"../tools/RUST_KODEX_GATE_CORE/src/rust_source_fact_primitives.rs\")",
        )
        || !launcher_text.contains("root.join(\"tools/RUST_KODEX_GATE_CORE/src\")")
        || !kodex_gate_core_ownership_text.contains("new_package=false")
        || !kodex_gate_core_ownership_text.contains(
            "owned_mechanics=rust_source_masking_fnv1a64_and_policy_free_source_facts_v1",
        )
        || !kodex_gate_core_ownership_text.contains(
            "source_fact_mechanics=recursive_rs_collection,path_normalization,line_start_index,line_number_lookup,token_boundaries",
        )
        || !kodex_gate_core_ownership_text.contains(
            "implementation_path=tools/RUST_KODEX_GATE_CORE/src/rust_source_scan_primitives.rs",
        )
        || !kodex_gate_core_ownership_text.contains(
            "source_fact_implementation_path=tools/RUST_KODEX_GATE_CORE/src/rust_source_fact_primitives.rs",
        )
        || !kodex_gate_core_ownership_text.contains(
            "negative_check_contract_path=src/build_gate/rust_source_scan_primitives.rs",
        )
        || !kodex_gate_core_ownership_text.contains("negative_check_identity=path_stable")
        || !take_change_control_text.contains(
            "#[path = \"build_gate/dead_code_detection_integrity.rs\"]\nmod dead_code_detection_integrity;",
        )
        || !take_change_control_text.contains(
            "validate_dead_code_detection_integrity_transition",
        )
        || !take_change_control_text.contains(
            "validate_dead_code_detection_integrity_installed",
        )
        || !identity_gate_trust_anchor_text.contains(
            "include_bytes!(\"build_gate/dead_code_detection_integrity.rs\")",
        )
        || !identity_gate_trust_anchor_text.contains(
            "include_bytes!(\"build_gate/dead_code_suppression_debt_ceiling.tsv\")",
        )
        || !kodex_gate_core_ownership_text.contains(
            "src/build_gate/dead_code_detection_integrity.rs",
        )
        || !kodex_gate_core_ownership_text.contains(
            "cross_crate_compilation=build_script_base_plus_source_facts,take_bootstrap_base_plus_source_facts_for_dead_code_integrity,main_test_base_plus_source_facts,global_identity_debt_inventory_test_base_plus_source_facts",
        )
    {
        return Err(
            "KODEX GATE CORE OWNERSHIP REGRESSION: implementation ownership, path-stable negative checks, trust anchor and build signature must remain jointly bound without a new Cargo boundary."
                .to_string(),
        );
    }

    Ok(())
}
