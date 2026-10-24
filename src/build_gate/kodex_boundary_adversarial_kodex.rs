// r450 CompletedCargoTransitionAdversarialSteadyStateClosure: adversarial self-host checks follow the steady-state Cargo contract and no longer require the retired build-owner rename capability.
use super::{parse_feedback_observable_sink_allowlist, parse_kodex_boundary_adversarial_matrix};
use super::feedback_structuralization_kodex::{
    collect_rust_files_for_feedback_confinement,
    expected_feedback_observable_sink_allowlist,
    feedback_compile_time_kodex_owner_paths,
    feedback_path_matches_registered_owner,
    feedback_public_contract_reference_paths,
    find_effectively_public_item,
    normalize_path_for_feedback_gate,
};
use std::{collections::BTreeMap, fs, path::Path};

pub(super) struct KodexBoundaryAdversarialInputs<'a> {
    pub(super) source_root: &'a Path,
    pub(super) kodex_gate_core_source_root: &'a Path,
    pub(super) feedback_gate_text: &'a str,
    pub(super) feedback_transport_text: &'a str,
    pub(super) feedback_observable_sink_allowlist_path: &'a Path,
    pub(super) feedback_observable_sink_allowlist_text: &'a str,
    pub(super) semantic_capability_text: &'a str,
    pub(super) take_change_control_text: &'a str,
    pub(super) take_bootstrap_text: &'a str,
    pub(super) identity_gate_trust_anchor_text: &'a str,
    pub(super) matrix_path: &'a Path,
    pub(super) matrix_text: &'a str,
}

fn adversarial_minimal_projection_contract_is_green(payload: &str) -> bool {
    let mut schema_version = None;
    let mut proof_green = None;
    let mut completeness_complete = None;
    let mut invariants_hold = None;
    let mut writer_eligible = None;
    let mut non_linkable = None;
    let mut field_count = 0usize;
    for field in payload.split("; ") {
        let Some((name, value)) = field.split_once(' ') else {
            return false;
        };
        field_count += 1;
        let assigned = match name {
            "SchemaVersion" => adversarial_assign_once(
                &mut schema_version,
                value.parse::<u32>().ok(),
            ),
            "ProofGreen" => adversarial_assign_once(
                &mut proof_green,
                adversarial_parse_yes_no(value),
            ),
            "CompletenessComplete" => adversarial_assign_once(
                &mut completeness_complete,
                adversarial_parse_yes_no(value),
            ),
            "InvariantsHold" => adversarial_assign_once(
                &mut invariants_hold,
                adversarial_parse_yes_no(value),
            ),
            "WriterEligible" => adversarial_assign_once(
                &mut writer_eligible,
                adversarial_parse_yes_no(value),
            ),
            "NonLinkable" => adversarial_assign_once(
                &mut non_linkable,
                adversarial_parse_yes_no(value),
            ),
            _ => false,
        };
        if !assigned {
            return false;
        }
    }
    field_count == 6
        && schema_version == Some(2)
        && proof_green == Some(true)
        && completeness_complete == Some(true)
        && invariants_hold == Some(true)
        && writer_eligible == Some(false)
        && non_linkable == Some(true)
}

fn adversarial_assign_once<T>(slot: &mut Option<T>, value: Option<T>) -> bool {
    if slot.is_some() {
        return false;
    }
    let Some(value) = value else {
        return false;
    };
    *slot = Some(value);
    true
}

fn adversarial_parse_yes_no(value: &str) -> Option<bool> {
    match value {
        "ja" => Some(true),
        "nein" => Some(false),
        _ => None,
    }
}

fn source_tree_contains_marker_outside_compile_time_kodex_policy(
    source_root: &Path,
    marker: &str,
) -> Result<bool, String> {
    let mut files = Vec::new();
    collect_rust_files_for_feedback_confinement(source_root, &mut files)?;
    let compile_time_kodex_owner_paths =
        feedback_compile_time_kodex_owner_paths(&expected_feedback_observable_sink_allowlist());
    for path in files {
        let normalized = normalize_path_for_feedback_gate(&path);
        if feedback_path_matches_registered_owner(
            &normalized,
            &compile_time_kodex_owner_paths,
        ) {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        if text.contains(marker) {
            return Ok(true);
        }
    }
    Ok(false)
}


pub(super) fn validate_kodex_boundary_adversarial_completion(
    inputs: KodexBoundaryAdversarialInputs<'_>,
) -> Result<usize, String> {
    let KodexBoundaryAdversarialInputs {
        source_root,
        kodex_gate_core_source_root,
        feedback_gate_text,
        feedback_transport_text,
        feedback_observable_sink_allowlist_path,
        feedback_observable_sink_allowlist_text,
        semantic_capability_text,
        take_change_control_text,
        take_bootstrap_text,
        identity_gate_trust_anchor_text,
        matrix_path,
        matrix_text,
    } = inputs;
    let observed = parse_kodex_boundary_adversarial_matrix(matrix_path, matrix_text)?;
    let dead_code_detection_integrity_text = fs::read_to_string(
        source_root.join("build_gate/dead_code_detection_integrity.rs"),
    )
    .map_err(|error| {
        format!(
            "DEAD-CODE ADVERSARIAL POLICY OWNER not readable: {error}"
        )
    })?;
    let expected = BTreeMap::from([
        (
            "additional_public_sink".to_string(),
            ("exact_sink_allowlist".to_string(), "blocked".to_string()),
        ),
        (
            "duplicate_projection".to_string(),
            ("closed_projection_parser".to_string(), "blocked".to_string()),
        ),
        (
            "failed_reexec_rollback".to_string(),
            ("transactional_rollback".to_string(), "blocked".to_string()),
        ),
        (
            "inconsistent_trust_anchor".to_string(),
            ("pinned_fingerprint_chain".to_string(), "blocked".to_string()),
        ),
        (
            "private_debug_or_error_leak".to_string(),
            ("private_detail_confinement".to_string(), "blocked".to_string()),
        ),
        (
            "private_value_in_artifact_identity".to_string(),
            ("controlled_sink_denial".to_string(), "blocked".to_string()),
        ),
        (
            "removed_kodex_negative_check".to_string(),
            ("capability_continuity".to_string(), "blocked".to_string()),
        ),
        (
            "retired_detail_section".to_string(),
            ("legacy_sink_marker_denial".to_string(), "blocked".to_string()),
        ),
        (
            "kodex_to_calibration".to_string(),
            ("change_class_separation".to_string(), "blocked".to_string()),
        ),
        (
            "kodex_to_music_writer".to_string(),
            ("semantic_writer_reachability".to_string(), "blocked".to_string()),
        ),
        (
            "unknown_projection_field".to_string(),
            ("closed_projection_parser".to_string(), "blocked".to_string()),
        ),
        (
            "unknown_source_path".to_string(),
            ("semantic_unknown_fail_closed".to_string(), "blocked".to_string()),
        ),
        (
            "external_runtime_locator_to_music_writer".to_string(),
            ("productive_identity_provenance".to_string(), "blocked".to_string()),
        ),
        (
            "external_locator_wrapper_chain".to_string(),
            ("productive_identity_provenance".to_string(), "blocked".to_string()),
        ),
        (
            "external_locator_struct_field".to_string(),
            ("productive_identity_provenance".to_string(), "blocked".to_string()),
        ),
        (
            "external_locator_closure_capture".to_string(),
            ("productive_identity_provenance".to_string(), "blocked".to_string()),
        ),
        (
            "raw_public_target_measure".to_string(),
            ("opaque_structural_locator_boundary".to_string(), "blocked".to_string()),
        ),
        (
            "new_productive_repair_authority".to_string(),
            ("no_new_repair_authority".to_string(), "blocked".to_string()),
        ),
        (
            "new_dead_code_suppression".to_string(),
            (
                "dead_code_suppression_monotonicity".to_string(),
                "blocked".to_string(),
            ),
        ),
        (
            "dead_code_suppression_scope_widening".to_string(),
            (
                "dead_code_suppression_identity".to_string(),
                "blocked".to_string(),
            ),
        ),
        (
            "compiler_dead_code_lint_weakening".to_string(),
            (
                "compiler_lint_session_integrity".to_string(),
                "blocked".to_string(),
            ),
        ),
    ]);
    if observed != expected {
        return Err(format!(
            "KODEX BOUNDARY ADVERSARIAL MATRIX REGRESSION: expected exact closed matrix {expected:?}, got {observed:?}. Missing, additional, renamed or reassigned attacks fail closed."
        ));
    }

    let gate_production = feedback_gate_text
        .split("\n#[cfg(test)]")
        .next()
        .unwrap_or(feedback_gate_text);
    let transport_production = feedback_transport_text
        .split("\n#[cfg(test)]")
        .next()
        .unwrap_or(feedback_transport_text);
    for forbidden_debug_or_error_surface in [
        "{:?}",
        "dbg!",
        "println!",
        "eprintln!",
        "panic!",
        "write_all(",
        "File::",
        "OpenOptions",
    ] {
        if gate_production.contains(forbidden_debug_or_error_surface)
            || transport_production.contains(forbidden_debug_or_error_surface)
        {
            return Err(format!(
                "ADVERSARIAL PRIVATE DEBUG/ERROR LEAK NOT BLOCKED: production feedback kodex surface contains {forbidden_debug_or_error_surface:?}."
            ));
        }
    }
    for private_item in [
        "struct FeedbackOriginNonInterferencePipelineAudit",
        "struct IdentityWithheldPipelineCandidateSet",
        "struct PipelineNonInterferenceVariantRow",
    ] {
        if find_effectively_public_item(feedback_gate_text, private_item).is_some() {
            return Err(format!(
                "ADVERSARIAL PRIVATE TYPE ESCAPE NOT BLOCKED: {private_item:?} is effectively public."
            ));
        }
    }

    let exact_sinks = parse_feedback_observable_sink_allowlist(
        feedback_observable_sink_allowlist_path,
        feedback_observable_sink_allowlist_text,
    )?;
    let expected_exact_sinks = expected_feedback_observable_sink_allowlist();
    if exact_sinks != expected_exact_sinks {
        return Err(
            "ADVERSARIAL ADDITIONAL PUBLIC SINK PROBE FAILED: baseline allowlist no longer equals the exact closed registered sink set."
                .to_string(),
        );
    }
    const INJECTED_SINK_PATH: &str = "src/adversarial_extra_sink.rs";
    const INJECTED_SINK_ROLE: &str = "public_debug_sink";
    let mut injected_sink_text = feedback_observable_sink_allowlist_text.to_string();
    injected_sink_text.push_str(&format!(
        "{INJECTED_SINK_PATH}\t{INJECTED_SINK_ROLE}\n"
    ));
    let injected_sinks = parse_feedback_observable_sink_allowlist(
        feedback_observable_sink_allowlist_path,
        &injected_sink_text,
    )?;
    let mut expected_injected_sinks = exact_sinks.clone();
    if expected_injected_sinks
        .insert(INJECTED_SINK_PATH.to_string(), INJECTED_SINK_ROLE.to_string())
        .is_some()
        || injected_sinks != expected_injected_sinks
    {
        return Err(
            "ADVERSARIAL ADDITIONAL PUBLIC SINK PROBE FAILED: exact allowlist comparison did not distinguish one injected unknown sink from the closed baseline set."
                .to_string(),
        );
    }
    if feedback_public_contract_reference_paths(source_root)?
        != exact_sinks.keys().cloned().collect::<Vec<_>>()
    {
        return Err(
            "ADVERSARIAL ADDITIONAL PUBLIC SINK PROBE FAILED: source references do not equal the exact registered sink set."
                .to_string(),
        );
    }

    const GREEN: &str = "SchemaVersion 2; ProofGreen ja; CompletenessComplete ja; InvariantsHold ja; WriterEligible nein; NonLinkable ja";
    if !adversarial_minimal_projection_contract_is_green(GREEN)
        || adversarial_minimal_projection_contract_is_green(
            "SchemaVersion 2; ProofGreen ja; CompletenessComplete ja; InvariantsHold ja; WriterEligible nein; NonLinkable ja; ExtraField ja",
        )
        || adversarial_minimal_projection_contract_is_green(
            "SchemaVersion 2; ProofGreen ja; ProofGreen ja; CompletenessComplete ja; InvariantsHold ja; WriterEligible nein; NonLinkable ja",
        )
        || adversarial_minimal_projection_contract_is_green(
            "SchemaVersion 2; ProofGreen ja; CompletenessComplete ja; WriterEligible nein; NonLinkable ja",
        )
    {
        return Err(
            "ADVERSARIAL MINIMAL PROJECTION PARSER PROBE FAILED: unknown, duplicate or missing fields were not rejected."
                .to_string(),
        );
    }

    for retired_marker in [
        "FEEDBACK-STRUCTURALIZATION-AND-RELOCATION-PROOF-GATE",
        "PositiveVariant | DecisionStable | StructuralFingerprintStable | OriginVisibleToRule",
        "NegativeControl: Rejected {}/{}",
        "Admission: BaselineDecision {:?}",
    ] {
        if source_tree_contains_marker_outside_compile_time_kodex_policy(source_root, retired_marker)?
            || source_tree_contains_marker_outside_compile_time_kodex_policy(
                kodex_gate_core_source_root,
                retired_marker,
            )?
        {
            return Err(format!(
                "ADVERSARIAL RETIRED DETAIL SECTION NOT BLOCKED: managed Rust sources reintroduced {retired_marker:?}."
            ));
        }
    }

    for required_negative_capability in [
        "fn explicit_named_counts_survive_without_locator_syntax()",
        "fn sanitized_legacy_ratio_is_reconstructed_from_variant_rows()",
        "fn unstable_variant_keeps_transport_closed()",
        "fn missing_evidence_is_fail_closed()",
        "fn false_origin_does_not_change_multiset_or_fingerprint()",
        "fn typed_projection_contains_only_identity_free_proof_state()",
    ] {
        if !feedback_gate_text.contains(required_negative_capability)
            && !feedback_transport_text.contains(required_negative_capability)
        {
            return Err(format!(
                "ADVERSARIAL NEGATIVE-CAPABILITY CONTINUITY FAILURE: required kodex check {required_negative_capability:?} is missing."
            ));
        }
    }

    for required_semantic_defense in [
        "fn kodex_only_rejects_indirect_music_writer_reachability()",
        "fn kodex_only_rejects_music_threshold_change()",
        "fn removed_kodex_check_is_rejected_even_for_kodex_only()",
        "fn kodex_negative_check_relocation_accepts_exact_path_move()",
        "fn kodex_negative_check_relocation_is_rejected_without_explicit_marker()",
        "fn kodex_negative_check_relocation_rejects_body_change()",
        "fn kodex_negative_check_relocation_rejects_signature_change()",
        "fn kodex_negative_check_relocation_rejects_symbol_change()",
        "fn kodex_negative_check_relocation_rejects_lost_execution_anchor()",
        "fn kodex_negative_check_relocation_rejects_extra_negative_check()",
        "fn kodex_negative_check_module_relocation_accepts_private_to_parent_module_move()",
        "fn kodex_negative_check_module_relocation_is_rejected_without_explicit_marker()",
        "fn kodex_negative_check_module_relocation_rejects_pub_crate_target()",
        "fn kodex_negative_check_module_relocation_rejects_body_change()",
        "fn kodex_negative_check_module_relocation_rejects_missing_parent_binding()",
        "fn kodex_negative_check_module_relocation_rejects_commented_path_binding()",
        "kodex_execution_anchor_contract",
        "validate_kodex_negative_check_relocation",
        "validate_kodex_negative_check_module_relocation",
        "validate_parent_module_binding",
        "removed negative/kodex check",
        "KodexCapabilityRetirement",
        "fn exact_new_kodex_retirement_authorizes_only_bound_negative_check()",
        "fn kodex_retirement_is_append_only_and_fingerprint_bound()",
        "Writer-Faehigkeit und Schwellenwertaenderung",
        "geaenderter Pfad {path} erzeugt keine bekannte Capability; fail-closed",
        "record.domain != Domain::Kodex",
        "fn productive_identity_firewall_rejects_direct_variable_measure_selector()",
        "fn productive_identity_firewall_rejects_external_locator_through_harmless_wrapper()",
        "fn productive_identity_firewall_rejects_tuple_transport()",
        "fn productive_identity_firewall_rejects_external_locator_via_struct_field()",
        "fn productive_identity_firewall_rejects_closure_capture()",
        "fn productive_identity_firewall_rejects_raw_public_target_measure()",
        "fn productive_identity_firewall_rejects_raw_public_target_parameter()",
        "fn productive_identity_firewall_allows_private_structural_measure_and_audit_locator()",
        "fn productive_identity_firewall_allows_opaque_structural_locator_boundary()",
        "fn productive_identity_firewall_allows_non_identity_runtime_control()",
        "validate_productive_music_identity_provenance_inventory(&incoming)?;",
        "validate_no_new_productive_repair_recovery_authority(",
        "exact_byte_identical_refactor_rust_owner_relocation",
        "fn refactor_exact_rust_owner_relocation_preserves_existing_writer_reachability()",
        "fn refactor_rust_owner_relocation_rejects_nonidentical_writer_body()",
        "fn music_mutation_rejects_new_productive_repair_authority()",
        "NO-NEW-REPAIR-AUTHORITY KODEX GATE",
        "fn music_mutation_rejects_renamed_late_writer_layer_outside_structural_authority()",
        "new_writer_caller_outside_structural_write_authority",
        "fn music_mutation_allows_new_canonical_structural_writer_owner()",
    ] {
        if !semantic_capability_text.contains(required_semantic_defense) {
            return Err(format!(
                "ADVERSARIAL SEMANTIC CHANGE-CLASS DEFENSE MISSING: {required_semantic_defense:?}."
            ));
        }
    }

    for required_transactional_defense in [
        "validate_complete_incoming_tree",
        "verify_installed_tree_matches_source",
        "validate_pending_transaction_semantics",
        "kodex_negative_check_relocation",
        "closed_exact_negative_check_relocation_v1",
        "manifest_accepts_closed_kodex_negative_check_relocation_marker",
        "manifest_rejects_unknown_kodex_negative_check_relocation_value",
        "kodex_negative_check_module_relocation",
        "closed_private_to_parent_module_relocation_v1",
        "manifest_accepts_closed_kodex_negative_check_module_relocation_marker",
        "manifest_rejects_unknown_kodex_negative_check_module_relocation_value",
        "manifest_rejects_unknown_kodex_build_owner_rename_value",
        "fn retired_one_shot_transition_markers_are_rejected()",
        "fn cargo_manifest_steady_state_allows_only_package_version_change()",
        "fn watch_cargo_steady_state_rejects_manifest_and_lock_changes()",
        "validate_cargo_manifest_change_contract",
        "validate_watch_cargo_manifest_change_contract",
    ] {
        if !take_change_control_text.contains(required_transactional_defense) {
            return Err(format!(
                "ADVERSARIAL TRANSACTION DEFENSE MISSING: take change control lacks {required_transactional_defense:?}."
            ));
        }
    }
    for required_rollback_defense in [
        "TAKE-TRANSACTION ROLLBACK",
        "transactional_apply_can_rollback_exactly",
        "transaction.rollback(&project_root)?;",
        "transaction.commit()?;",
        "TAKE-TRANSACTION COMMIT: Reexec 0",
    ] {
        if !take_bootstrap_text.contains(required_rollback_defense) {
            return Err(format!(
                "ADVERSARIAL FAILED-REEXEC ROLLBACK DEFENSE MISSING: bootstrap lacks {required_rollback_defense:?}."
            ));
        }
    }

    for required_dead_code_defense in [
        "fn new_dead_code_suppression_is_rejected_by_monotonic_identity()",
        "fn dead_code_suppression_scope_widening_changes_identity()",
        "fn cfg_attr_and_unused_group_are_protected_suppressions()",
        "fn compiler_dead_code_lint_weakening_is_rejected()",
        "fn root_package_lint_authority_cannot_be_lowered()",
        "validate_transition",
        "validate_installed",
        "CARGO_ENCODED_RUSTFLAGS",
        "dead_code_suppression_debt_ceiling.tsv",
    ] {
        if !dead_code_detection_integrity_text.contains(required_dead_code_defense) {
            return Err(format!(
                "ADVERSARIAL DEAD-CODE DETECTION DEFENSE MISSING: policy owner lacks {required_dead_code_defense:?}."
            ));
        }
    }

    for required_trust_chain in [
        "KODEX_BOUNDARY_ADVERSARIAL_MATRIX_BYTES",
        "KODEX_BOUNDARY_ADVERSARIAL_MATRIX_FNV1A64",
        "SOURCE_INTEGRITY_KODEX_FNV1A64",
        "RECOVERY_TRANSACTION_ORDER_KODEX_FNV1A64",
        "PINNED_POLICY_KODEX_FNV1A64",
        "INVENTORY_KODEX_FNV1A64",
        "MANAGED_SOURCE_TREE_DEFINITION_FNV1A64",
        "MULTI_REVISION_EVASION_FNV1A64",
        "SHARED_SHA256_FNV1A64",
        "RUST_SOURCE_SCAN_PRIMITIVES_FNV1A64",
        "SEMANTIC_CAPABILITY_DIFF_FNV1A64",
        "KODEX_CAPABILITY_RETIREMENTS_FNV1A64",
        "PRODUCTIVE_WRITE_PATH_SCANNER_FNV1A64",
        "THIRTY_SURFACE_BATCH_SCAN_FNV1A64",
        "KODEX_GATE_CORE_OWNERSHIP_FNV1A64",
        "DEAD_CODE_DETECTION_INTEGRITY_FNV1A64",
        "DEAD_CODE_SUPPRESSION_DEBT_CEILING_FNV1A64",
        "assert!(KODEX_BOUNDARY_ADVERSARIAL_MATRIX_FNV1A64 ==",
        "assert!(SOURCE_INTEGRITY_KODEX_FNV1A64 ==",
        "assert!(RECOVERY_TRANSACTION_ORDER_KODEX_FNV1A64 ==",
        "assert!(PINNED_POLICY_KODEX_FNV1A64 ==",
        "assert!(INVENTORY_KODEX_FNV1A64 ==",
        "assert!(MANAGED_SOURCE_TREE_DEFINITION_FNV1A64 ==",
        "assert!(SHARED_SHA256_FNV1A64 ==",
        "assert!(RUST_SOURCE_SCAN_PRIMITIVES_FNV1A64 ==",
        "assert!(TAKE_CHANGE_CONTROL_FNV1A64 ==",
        "assert!(SEMANTIC_CAPABILITY_DIFF_FNV1A64 ==",
        "assert!(KODEX_CAPABILITY_RETIREMENTS_FNV1A64 ==",
        "assert!(PRODUCTIVE_WRITE_PATH_SCANNER_FNV1A64 ==",
        "assert!(KODEX_GATE_CORE_OWNERSHIP_FNV1A64 ==",
        "assert!(DEAD_CODE_DETECTION_INTEGRITY_FNV1A64 ==",
        "assert!(DEAD_CODE_SUPPRESSION_DEBT_CEILING_FNV1A64 ==",
    ] {
        if !identity_gate_trust_anchor_text.contains(required_trust_chain) {
            return Err(format!(
                "ADVERSARIAL TRUST-ANCHOR DEFENSE MISSING: trust anchor lacks {required_trust_chain:?}."
            ));
        }
    }

    Ok(expected.len())
}
