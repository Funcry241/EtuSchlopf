use super::parse_feedback_observable_sink_allowlist;
use super::rust_source_scan_primitives::mask_non_code;
use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};

const EXPECTED_FEEDBACK_OBSERVABLE_SINK_ALLOWLIST: &[(&str, &str)] = &[
    (
        "src/feedback_structuralization_gate.rs",
        "canonical_result_report_projection",
    ),
    (
        "src/feedback_proof_transport.rs",
        "fail_closed_minimal_transport_parser",
    ),
    (
        "src/pipeline_shards/key_signature_enharmonic_report_core.rs",
        "read_only_release_consumer",
    ),
    (
        "src/build_gate/feedback_structuralization_kodex.rs",
        "compile_time_kodex_policy",
    ),
];

const EXPECTED_COMPILE_TIME_KODEX_POLICY_OWNERS: &[&str] = &[
    "src/kodex_build_orchestrator.rs",
    "src/build_gate/feedback_structuralization_kodex.rs",
    "src/build_gate/kodex_boundary_adversarial_kodex.rs",
    "src/build_gate/source_integrity_kodex.rs",
    "src/build_gate/recovery_transaction_order_kodex.rs",
    "src/build_gate/pinned_policy_kodex.rs",
    "src/build_gate/inventory_kodex.rs",
];

pub(super) fn find_effectively_public_item(source: &str, item: &str) -> Option<usize> {
    let masked = mask_non_code(source);
    let mut line_start = 0usize;
    for line in masked.split_inclusive('\n') {
        let trimmed = line.trim_start_matches([' ', '\t']);
        let indentation = line.len() - trimmed.len();
        if ["pub ", "pub(crate) "].into_iter().any(|visibility| {
            trimmed
                .strip_prefix(visibility)
                .is_some_and(|tail| tail.starts_with(item))
        })
        {
            return Some(line_start + indentation);
        }
        line_start += line.len();
    }
    None
}

pub(super) fn find_effectively_public_line_boundary(source: &str, item: &str) -> Option<usize> {
    let item_start = find_effectively_public_item(source, item)?;
    if item_start == 0 {
        return Some(0);
    }
    (source.as_bytes().get(item_start - 1) == Some(&b'\n')).then_some(item_start - 1)
}

pub(super) fn strip_effective_public_visibility(source: &str) -> Option<&str> {
    source
        .strip_prefix("pub ")
        .or_else(|| source.strip_prefix("pub(crate) "))
}

pub(super) fn expected_feedback_observable_sink_allowlist() -> BTreeMap<String, String> {
    EXPECTED_FEEDBACK_OBSERVABLE_SINK_ALLOWLIST
        .iter()
        .map(|(path, role)| ((*path).to_string(), (*role).to_string()))
        .collect()
}

pub(super) fn feedback_compile_time_kodex_owner_paths(
    _sink_allowlist: &BTreeMap<String, String>,
) -> Vec<String> {
    EXPECTED_COMPILE_TIME_KODEX_POLICY_OWNERS
        .iter()
        .map(|path| normalize_path_for_feedback_gate(Path::new(path)))
        .collect()
}

pub(super) fn feedback_path_matches_registered_owner(path: &str, owners: &[String]) -> bool {
    owners
        .iter()
        .any(|owner| path == owner || path.ends_with(&format!("/{owner}")))
}

pub(super) fn feedback_public_contract_reference_paths(source_root: &Path) -> Result<Vec<String>, String> {
    let project_root = source_root.parent().ok_or_else(|| {
        "FEEDBACK OBSERVABLE SINK SURFACE REGRESSION: source root has no project parent"
            .to_string()
    })?;
    let mut files = Vec::new();
    collect_rust_files_for_feedback_confinement(source_root, &mut files)?;
    let mut observed = Vec::new();
    for path in files {
        let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        if [
            "FeedbackOriginNonInterferencePipelineProof",
            "FeedbackObservableArtifactClosure",
            "parse_feedback_proof_variant_evidence",
        ]
        .iter()
        .any(|marker| text.contains(marker))
        {
            let relative = path.strip_prefix(project_root).map_err(|error| {
                format!(
                    "FEEDBACK OBSERVABLE SINK SURFACE REGRESSION: {} is outside project root: {error}",
                    path.display()
                )
            })?;
            observed.push(normalize_path_for_feedback_gate(relative));
        }
    }
    observed.sort();
    observed.dedup();
    Ok(observed)
}

pub(super) fn collect_rust_files_for_feedback_confinement(
    path: &Path,
    out: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let child = entry.path();
        if child.is_dir() {
            collect_rust_files_for_feedback_confinement(&child, out)?;
        } else if child.extension().and_then(|value| value.to_str()) == Some("rs") {
            out.push(child);
        }
    }
    Ok(())
}

pub(super) fn normalize_path_for_feedback_gate(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}


pub(super) fn validate_feedback_structuralization_boundary(
    source_root: &Path,
    feedback_gate_path: &Path,
    feedback_gate_text: &str,
    feedback_admission_path: &Path,
    feedback_admission_text: &str,
    feedback_observable_sink_allowlist_path: &Path,
    feedback_observable_sink_allowlist_text: &str,
) -> Result<(), String> {
    let admission_rows = feedback_admission_text
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("rule_id\t")
        })
        .count();
    if admission_rows != 0 {
        return Err(format!(
            "FEEDBACK RULE ADMISSION MUST REMAIN EMPTY: {} contains {} data rows. This audit-only version cannot admit a feedback-derived writer.",
            feedback_admission_path.display(),
            admission_rows
        ));
    }

    let sink_allowlist = parse_feedback_observable_sink_allowlist(
        feedback_observable_sink_allowlist_path,
        feedback_observable_sink_allowlist_text,
    )?;
    let expected_sink_allowlist = expected_feedback_observable_sink_allowlist();
    if sink_allowlist != expected_sink_allowlist {
        return Err(format!(
            "FEEDBACK OBSERVABLE SINK ALLOWLIST REGRESSION: expected exact closed register {expected_sink_allowlist:?}, got {sink_allowlist:?}. Unknown, missing or reassigned sinks fail closed."
        ));
    }
    let observed_sink_paths = feedback_public_contract_reference_paths(source_root)?;
    let expected_sink_paths = expected_sink_allowlist.keys().cloned().collect::<Vec<_>>();
    if observed_sink_paths != expected_sink_paths {
        return Err(format!(
            "FEEDBACK OBSERVABLE SINK SURFACE REGRESSION: registered paths {expected_sink_paths:?}, observed public-contract reference paths {observed_sink_paths:?}."
        ));
    }

    let transport_path = source_root.join("feedback_proof_transport.rs");
    let transport_text =
        fs::read_to_string(&transport_path).map_err(|error| error.to_string())?;
    for required_transport_contract in [
        "Kurz: FeedbackOriginNonInterferencePipelineProof:",
        "SchemaVersion",
        "ProofGreen",
        "CompletenessComplete",
        "InvariantsHold",
        "WriterEligible",
        "NonLinkable",
        "field_count != 6",
        "schema_version != Some(2)",
        "transport_green: total == 1 && stable == 1",
    ] {
        if !transport_text.contains(required_transport_contract) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROOF TRANSPORT REGRESSION: required contract {required_transport_contract:?} is missing."
            ));
        }
    }
    for forbidden_transport_contract in [
        "StableVariantCount",
        "TotalVariantCount",
        "parse_variant_table",
        "CandidateSetFingerprint",
        "CandidateOccurrences",
        "UniqueSignatures",
        "Variant | CoordinatePresent",
    ] {
        if transport_text.contains(forbidden_transport_contract) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROOF TRANSPORT REGRESSION: legacy linkable contract {forbidden_transport_contract:?} is still present."
            ));
        }
    }
    let transport_production = transport_text
        .split("\n#[cfg(test)]")
        .next()
        .unwrap_or(transport_text.as_str());
    for forbidden_transport_sink in [
        "println!",
        "eprintln!",
        "dbg!",
        "panic!",
        "std::fs",
        "File::",
        "OpenOptions",
        "std::process",
        "Command::",
        "std::env::",
        "PathBuf",
        ".push_str(",
        "writeln!(",
    ] {
        if transport_production.contains(forbidden_transport_sink) {
            return Err(format!(
                "FEEDBACK MINIMAL TRANSPORT ROLE REGRESSION: {} contains controlled-sink primitive {forbidden_transport_sink:?}.",
                transport_path.display()
            ));
        }
    }

    let read_only_consumer_path =
        source_root.join("pipeline_shards/key_signature_enharmonic_report_core.rs");
    let read_only_consumer_text =
        fs::read_to_string(&read_only_consumer_path).map_err(|error| error.to_string())?;
    let consumer_start = read_only_consumer_text
        .find("fn feedback_noninterference_summary_line(")
        .ok_or_else(|| {
            format!(
                "FEEDBACK READ-ONLY CONSUMER REGRESSION: {} has no minimal summary reader.",
                read_only_consumer_path.display()
            )
        })?;
    let consumer_tail = &read_only_consumer_text[consumer_start..];
    let consumer_end = consumer_tail
        .find("\nfn parse_tonal_segmentation_summary_value(")
        .ok_or_else(|| {
            format!(
                "FEEDBACK READ-ONLY CONSUMER REGRESSION: {} has no closed reader boundary.",
                read_only_consumer_path.display()
            )
        })?;
    let consumer_block = &consumer_tail[..consumer_end];
    for forbidden_consumer_sink in [
        "println!",
        "eprintln!",
        "dbg!",
        "panic!",
        "std::fs",
        "File::",
        "OpenOptions",
        "std::process",
        "Command::",
        "PathBuf",
        ".push_str(",
        "writeln!(",
        "write_all(",
    ] {
        if consumer_block.contains(forbidden_consumer_sink) {
            return Err(format!(
                "FEEDBACK READ-ONLY CONSUMER ROLE REGRESSION: {} contains controlled-sink primitive {forbidden_consumer_sink:?}.",
                read_only_consumer_path.display()
            ));
        }
    }

    let private_coordinate_declaration_seen = feedback_gate_text
        .lines()
        .any(|line| line.trim_start().starts_with("struct DiagnosticCoordinate"));
    if find_effectively_public_item(feedback_gate_text, "struct DiagnosticCoordinate").is_some() {
        return Err(
            "FEEDBACK ORIGIN PRIVACY REGRESSION: DiagnosticCoordinate must remain private to feedback_structuralization_gate.rs."
                .to_string(),
        );
    }
    if !private_coordinate_declaration_seen {
        return Err(
            "FEEDBACK ORIGIN PRIVACY REGRESSION: private DiagnosticCoordinate declaration is missing."
                .to_string(),
        );
    }
    if feedback_gate_text.contains("FeedbackRuleWriteCapability")
        || feedback_gate_text.contains("FeedbackWriteCapability")
        || feedback_gate_text.contains("issue_feedback_writer")
    {
        return Err(
            "FEEDBACK WRITE CAPABILITY REGRESSION: v0.10.1293 defines no feedback-derived write token or issuance path."
                .to_string(),
        );
    }
    // The retired detailed structuralization audit used to contribute six legacy
    // relocation/control names here.  r783 closes that stale proof dependency:
    // only the still-live FeedbackOriginNonInterferencePipelineAudit contract is
    // required after the legacy private-projection island has been retired.
    for required in [
        "candidate_set_without_project_identity",
        "candidate_set_without_measure_identity",
        "origin_coordinate_removed",
        "candidate_set_with_false_origin_coordinate",
        "false_project_and_measure_identity",
        "nearest_origin_tiebreaker_defined",
        "report_data_reentry_defined",
        "project_or_path_identity_in_rule_hash",
        "pipeline_detector_accepts_origin",
    ] {
        if !feedback_gate_text.contains(required) {
            return Err(format!(
                "FEEDBACK NONINTERFERENCE PROOF REGRESSION: required live proof contract {required:?} is missing."
            ));
        }
    }

    for required_contract in [
        "fn detect_pipeline_observation_bundle(xml: &str)",
        "fn build_identity_withheld_pipeline_candidate_set(",
        "observations: &[IdentityWithheldPipelineObservation]",
        "fn post_selection_origin_confirmation(",
        "coordinate: Option<&DiagnosticCoordinate>",
        "origin_can_select_candidate = false",
        "nearest_origin_tiebreaker_defined = false",
        "report_data_reentry_defined = false",
        "project_or_path_identity_in_rule_hash = false",
        "pipeline_detector_accepts_origin = false",
    ] {
        if !feedback_gate_text.contains(required_contract) {
            return Err(format!(
                "FEEDBACK NONINTERFERENCE CONTRACT REGRESSION: required source contract {required_contract:?} is missing."
            ));
        }
    }
    for forbidden_contract in [
        "fn detect_pipeline_observation_bundle(xml: &str, coordinate",
        "fn build_identity_withheld_pipeline_candidate_set(\n    coordinate",
        "origin_can_select_candidate = true",
        "nearest_origin_tiebreaker_defined = true",
        "report_data_reentry_defined = true",
        "project_or_path_identity_in_rule_hash = true",
        "pipeline_detector_accepts_origin = true",
    ] {
        if feedback_gate_text.contains(forbidden_contract) {
            return Err(format!(
                "FEEDBACK INFORMATION BACKFLOW REGRESSION: forbidden contract {forbidden_contract:?} is present."
            ));
        }
    }

    let production_gate_text = feedback_gate_text
        .split("\n#[cfg(test)]")
        .next()
        .unwrap_or(feedback_gate_text);
    for forbidden_direct_sink in [
        "println!",
        "eprintln!",
        "dbg!",
        "panic!",
        "std::fs",
        "File::",
        "OpenOptions",
        "std::process",
        "Command::",
        "std::env::",
        "PathBuf",
        "create_dir",
        "write_all",
        "set_var",
    ] {
        if production_gate_text.contains(forbidden_direct_sink) {
            return Err(format!(
                "FEEDBACK OBSERVABLE SURFACE REGRESSION: private kodex module contains direct controlled-sink primitive {forbidden_direct_sink:?}."
            ));
        }
    }

    let projection_start = find_effectively_public_item(
        feedback_gate_text,
        "struct MinimalNonLinkableProofProjection",
    )
    .ok_or_else(|| {
        "MINIMAL NON-LINKABLE PROJECTION REGRESSION: public proof projection is missing."
            .to_string()
    })?;
    let projection_tail = &feedback_gate_text[projection_start..];
    let projection_end = projection_tail
        .find("\n}\n")
        .map(|index| index + 3)
        .ok_or_else(|| {
            "MINIMAL NON-LINKABLE PROJECTION REGRESSION: projection block is malformed."
                .to_string()
        })?;
    let projection_block = &projection_tail[..projection_end];
    let projection_contract = strip_effective_public_visibility(projection_block).ok_or_else(|| {
        "MINIMAL NON-LINKABLE PROJECTION REGRESSION: projection is not effectively public."
            .to_string()
    })?;
    let expected_projection_contract = "struct MinimalNonLinkableProofProjection {\n    schema_version: u32,\n    decision: MinimalProofDecision,\n    completeness: MinimalProofCompleteness,\n    invariants: MinimalProofInvariantStatus,\n    write_permission: MinimalProofWritePermission,\n    non_linkability: MinimalProofNonLinkability,\n}\n";
    if projection_contract != expected_projection_contract {
        return Err(format!(
            "MINIMAL NON-LINKABLE PROJECTION FIELD REGRESSION: expected exact closed field contract, got {projection_contract:?}. Unknown or reordered fields fail closed."
        ));
    }

    for exact_enum_contract in [
        "enum MinimalProofDecision {\n    Green,\n    Incomplete,\n}",
        "enum MinimalProofCompleteness {\n    Complete,\n    Incomplete,\n}",
        "enum MinimalProofInvariantStatus {\n    Holds,\n    Violated,\n}",
        "enum MinimalProofWritePermission {\n    Denied,\n}",
        "enum MinimalProofNonLinkability {\n    Enforced,\n}",
    ] {
        if !feedback_gate_text.contains(exact_enum_contract) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROJECTION ENUM REGRESSION: exact closed enum {exact_enum_contract:?} is missing."
            ));
        }
    }

    let projection_block_lower = projection_block.to_ascii_lowercase();
    for forbidden_projection_token in [
        "candidate",
        "fingerprint",
        "pattern",
        "project_",
        "measure",
        "pitch",
        "staff",
        "voice",
        "path",
        "coordinate",
        "count",
        "string",
        "vec<",
        "pathbuf",
        "serialize",
        "hash",
    ] {
        if projection_block_lower.contains(forbidden_projection_token) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROJECTION PRIVACY REGRESSION: forbidden token {forbidden_projection_token:?} escaped into the public projection."
            ));
        }
    }

    if find_effectively_public_item(
        feedback_gate_text,
        "fn audit_feedback_origin_noninterference_in_pipeline(",
    )
    .is_none()
    {
        return Err(
            "MINIMAL NON-LINKABLE PROJECTION REGRESSION: public audit entry is missing."
                .to_string(),
        );
    }

    for required_projection_contract in [
        "fn build_feedback_origin_noninterference_internal_audit(",
        ") -> FeedbackOriginNonInterferencePipelineAudit {",
        ") -> MinimalNonLinkableProofProjection {",
        "fn minimal_non_linkable_projection(",
        "fn minimal_non_linkable_projection_line(",
        "fn canonical_feedback_public_artifact_bytes(",
        "Kurz: FeedbackObservableArtifactClosure: SchemaVersion 1;",
        "AllowedSink ResultReportMinimalProjection",
        "OtherControlledSinksDenied ja",
        "UnknownSinkDenied ja",
        "DeniedControlledSinks: Console; DebugLog; ErrorPayload; FileName;",
        "TemporaryArtifact; CacheKey; ManifestContext.",
        "fn legacy_feedback_report_sink_marker_present(",
        "fn public_feedback_report_sink_is_closed_before_append(",
        "fn public_feedback_report_sink_is_closed_after_append(",
        "Kurz: FeedbackOriginNonInterferencePipelineProof: SchemaVersion {}; ProofGreen {}; CompletenessComplete {}; InvariantsHold {}; WriterEligible {}; NonLinkable {}.",
        "projection: &MinimalNonLinkableProofProjection,",
    ] {
        if !feedback_gate_text.contains(required_projection_contract) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROJECTION CONTRACT REGRESSION: required source contract {required_projection_contract:?} is missing."
            ));
        }
    }

    for private_item in [
        "struct FeedbackOriginNonInterferencePipelineAudit",
        "fn build_feedback_origin_noninterference_internal_audit",
        "fn minimal_non_linkable_projection_line",
    ] {
        if find_effectively_public_item(feedback_gate_text, private_item).is_some() {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROJECTION CONFINEMENT REGRESSION: private item {private_item:?} became effectively public."
            ));
        }
    }

    let renderer_start = feedback_gate_text
        .find("fn minimal_non_linkable_projection_line(")
        .ok_or_else(|| {
            "MINIMAL NON-LINKABLE PROJECTION RENDERER REGRESSION: renderer is missing."
                .to_string()
        })?;
    let renderer_tail = &feedback_gate_text[renderer_start..];
    let renderer_end = find_effectively_public_line_boundary(
        renderer_tail,
        "fn append_feedback_origin_noninterference_pipeline_proof_section",
    )
    .ok_or_else(|| {
            "MINIMAL NON-LINKABLE PROJECTION RENDERER REGRESSION: renderer boundary is missing."
                .to_string()
        })?;
    let renderer_block = &renderer_tail[..renderer_end];
    for forbidden_renderer_payload in [
        "FeedbackOriginNonInterferencePipelineAudit",
        "build_feedback_origin_noninterference_internal_audit",
        "variant_rows",
        "candidate_",
        "fingerprint",
        "localized_readback",
        "measure_number",
        "project_label",
        "std::fs",
        "File::",
        "OpenOptions",
        "println!",
        "eprintln!",
        "dbg!",
        "std::process",
        "Command::",
        "std::env",
        "PathBuf",
        "serde",
    ] {
        if renderer_block.contains(forbidden_renderer_payload) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROJECTION RENDERER REGRESSION: private or persistent payload {forbidden_renderer_payload:?} is reachable from the renderer."
            ));
        }
    }

    let append_start = find_effectively_public_item(
        feedback_gate_text,
        "fn append_feedback_origin_noninterference_pipeline_proof_section(",
    )
    .ok_or_else(|| {
            "MINIMAL NON-LINKABLE PROJECTION WRITER REGRESSION: public section writer is missing."
                .to_string()
        })?;
    let append_tail = &feedback_gate_text[append_start..];
    let append_end = append_tail.find("\n#[cfg(test)]").ok_or_else(|| {
        "MINIMAL NON-LINKABLE PROJECTION WRITER REGRESSION: writer block boundary is missing."
            .to_string()
    })?;
    let append_block = &append_tail[..append_end];
    for forbidden_writer_payload in [
        "FeedbackOriginNonInterferencePipelineAudit",
        "variant_rows",
        "candidate_occurrences",
        "candidate_set_fingerprint",
        "unique_structural_signatures",
        "baseline_candidate_set_fingerprint",
        "measure_number",
        "project_label",
        "localized_readback",
        "RuntimeEvidence:",
        "Variant | CoordinatePresent",
    ] {
        if append_block.contains(forbidden_writer_payload) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROJECTION WRITER REGRESSION: private payload {forbidden_writer_payload:?} is reachable from the artifact writer."
            ));
        }
    }

    for retired_legacy_surface in [
        "enum StructuralDefectFamily",
        "struct StructuralObservation",
        "struct IdentityWithheldStructuralClaim",
        "enum StructuralDecision",
        "struct OriginReadback",
        "struct PositiveRelocationRow",
        "struct NegativeControlRow",
        "struct FeedbackStructuralizationRelocationAudit",
        "fn diagnostic_coordinate_is_private_in_source(",
        "fn structuralize_observation(",
        "fn structural_fingerprint(",
        "fn evaluate_identity_withheld_claim(",
        "fn base_observation(",
        "fn positive_row(",
        "fn negative_row(",
        "fn audit_feedback_structuralization_and_relocation_boundary(",
        "fn append_feedback_structuralization_and_relocation_proof_section(",
    ] {
        if feedback_gate_text.contains(retired_legacy_surface) {
            return Err(format!(
                "LEGACY FEEDBACK STRUCTURALIZATION RETIREMENT REGRESSION: retired surface {retired_legacy_surface:?} returned."
            ));
        }
    }

    for forbidden_legacy_output in [
        "TypedProofProjection:",
        "RuntimeEvidence: MeasuresScanned",
        "Variant | CoordinatePresent | CandidateSetStable",
        "CandidateSetFingerprint {:016x}",
        "CandidateOccurrences {}; UniqueSignatures {}",
        "SerializationOwner noch_nicht_geoeffnet",
        "FEEDBACK-STRUCTURALIZATION-AND-RELOCATION-PROOF-GATE",
        "PositiveVariant | DecisionStable | StructuralFingerprintStable | OriginVisibleToRule",
        "NegativeVariant | Rejected",
        "RelocationProof: PositiveVariants {}/{}",
        "NegativeControl: Rejected {}/{}",
        "Admission: BaselineDecision {:?}",
    ] {
        if feedback_gate_text.contains(forbidden_legacy_output) {
            return Err(format!(
                "MINIMAL NON-LINKABLE PROJECTION OUTPUT REGRESSION: legacy linkable output {forbidden_legacy_output:?} remains present."
            ));
        }
    }

    let mut files = Vec::new();
    collect_rust_files_for_feedback_confinement(source_root, &mut files)?;
    let normalized_gate = normalize_path_for_feedback_gate(feedback_gate_path);
    let compile_time_kodex_owner_paths =
        feedback_compile_time_kodex_owner_paths(&expected_sink_allowlist);
    for path in files {
        let normalized = normalize_path_for_feedback_gate(&path);
        if normalized == normalized_gate
            || feedback_path_matches_registered_owner(
                &normalized,
                &compile_time_kodex_owner_paths,
            )
        {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        if text.contains("DiagnosticCoordinate") {
            return Err(format!(
                "FEEDBACK ORIGIN TYPE ESCAPED ITS MODULE: {} references DiagnosticCoordinate. Productive code may receive only identity-withheld structural claims.",
                path.display()
            ));
        }
        for private_symbol in [
            "FeedbackOriginNonInterferencePipelineAudit",
            "IdentityWithheldPipelineCandidateSet",
            "PipelineNonInterferenceVariantRow",
            "build_feedback_origin_noninterference_internal_audit",
            "minimal_non_linkable_projection_line",
        ] {
            if text.contains(private_symbol) {
                return Err(format!(
                    "MINIMAL NON-LINKABLE PROJECTION CONFINEMENT REGRESSION: {} references private symbol {private_symbol:?}.",
                    path.display()
                ));
            }
        }
        for forbidden_parallel_feedback_sink in [
            "FEEDBACK-STRUCTURALIZATION-AND-RELOCATION-PROOF-GATE",
            "PositiveVariant | DecisionStable | StructuralFingerprintStable | OriginVisibleToRule",
            "NegativeVariant | Rejected",
            "RelocationProof: PositiveVariants {}/{}",
            "NegativeControl: Rejected {}/{}",
            "Admission: BaselineDecision {:?}",
        ] {
            if text.contains(forbidden_parallel_feedback_sink) {
                return Err(format!(
                    "LEGACY FEEDBACK REPORT SINK ESCAPED CENTRAL GATE: {} contains forbidden public sink marker {forbidden_parallel_feedback_sink:?}.",
                    path.display()
                ));
            }
        }
        if text.contains("FeedbackRuleWriteCapability")
            || text.contains("FeedbackWriteCapability")
            || text.contains("issue_feedback_writer")
        {
            return Err(format!(
                "FEEDBACK WRITE CAPABILITY OUTSIDE CENTRAL GATE: {} contains a forbidden feedback-derived write-token symbol.",
                path.display()
            ));
        }
    }
    Ok(())
}
