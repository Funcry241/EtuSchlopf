//! Parent-bound K2 governance. No runtime authority and no inferred successful evidence.
use super::{
    BASELINE, BASELINE_PATH, PROOF_PATH, REMAINING_PATH, SHADOW_PATH, baseline_ids, code,
    contains_token, fail, item, read, remaining,
};
use crate::rust_source_scan_primitives::{
    collect_rust_files, mask_non_code, normalize_path,
};
use std::path::Path;

#[path = "ssc_governed_change.rs"]
pub(crate) mod governed;
#[path = "cursor_temporal_retirement_kodex.rs"]
pub(crate) mod cursor_temporal;
#[path = "render_checkpoint_prepare_kodex.rs"]
pub(crate) mod checkpoint_prepare;
pub(crate) const PATH: &str = "src/build_gate/ssc_k2_retirement_governance_contract_v1.txt";
const CONTRACT: &str = include_str!("ssc_k2_retirement_governance_contract_v1.txt");
// V2 stays a frozen historical document; the installed proof surface is V3.
const SURFACE_V2: &str = include_str!("ssc_future_type_surface_v2.txt");
const SURFACE: &str = include_str!("ssc_future_type_surface_v3.txt");
// r1278: the containment surface (contract v4 A2) frozen ahead of its runtime take. The
// installed proof is exactly V3 (r1277 runtime) or exactly V4; V4 never returns to V3.
const SURFACE_V4: &str = include_str!("ssc_future_type_surface_v4.txt");
const OBSERVER: &str = include_str!("ssc_observer_future_v2.txt");
const A4: &str = include_str!("ssc_remaining_projection_v2.txt");
const POP_PATH: &str = "src/build_gate/destructive_transformation_population_v1.tsv";
const POP_BASE_PATH: &str = "src/build_gate/ssc_population_prerequisites_r1250_v1.txt";
const POP: &str = include_str!("ssc_population_prerequisites_r1250_v1.txt");
const R57_V2_PATH: &str = "src/build_gate/ssc_render57_clef_batch_atomic_v2.txt";
const R57_BINDING_PATH: &str = "src/build_gate/ssc_render57_runtime_binding_v2.txt";
const R57_CAUSAL_PATH: &str =
    "docs/selector_derivation_contract_lower_staff_clef_original_source_envelope_r1252.md";
const R57_V2: &str = include_str!("ssc_render57_clef_batch_atomic_v2.txt");
const R57_BINDING: &str = include_str!("ssc_render57_runtime_binding_v2.txt");
const R57_CAUSAL: &str = include_str!(
    "../../docs/selector_derivation_contract_lower_staff_clef_original_source_envelope_r1252.md"
);
const R57_WRITER_PATH: &str = "src/structural_write_authority/remaining_layout_writer_family.rs";
const DOCUMENTS: &[(&str, &str)] = &[
    (WAV_COMPLETION_REGISTRY_PATH, super::structural_erasure::WAV_COMPLETION_NODES),
    (WAV_REGISTRY_PATH, super::structural_erasure::WAV_NODES),
    (R57_V2_PATH, R57_V2),
    (R57_BINDING_PATH, R57_BINDING),
    (R57_CAUSAL_PATH, R57_CAUSAL),
    (POP_BASE_PATH, POP),
    (
        COMPLETION_REGISTRY_PATH,
        super::structural_erasure::COMPLETION_NODES,
    ),
    (
        "src/build_gate/ssc_neutral_retirement_nodes_v1.txt",
        include_str!("ssc_neutral_retirement_nodes_v1.txt"),
    ),
    (PATH, CONTRACT),
    (
        "src/build_gate/ssc_s55_a_publication_fixture_v1.txt",
        include_str!("ssc_s55_a_publication_fixture_v1.txt"),
    ),
    (
        "src/build_gate/ssc_carrier_without_grandfathering_fixture_v1.txt",
        include_str!("ssc_carrier_without_grandfathering_fixture_v1.txt"),
    ),
    ("src/build_gate/ssc_future_type_surface_v2.txt", SURFACE_V2),
    ("src/build_gate/ssc_future_type_surface_v3.txt", SURFACE),
    ("src/build_gate/ssc_future_type_surface_v4.txt", SURFACE_V4),
    (
        "src/build_gate/ssc_proof_bundle_preserving_direction_contract_v1.txt",
        include_str!("ssc_proof_bundle_preserving_direction_contract_parent_r1485.txt"),
    ),
    ("src/build_gate/ssc_observer_future_v2.txt", OBSERVER),
    ("src/build_gate/ssc_remaining_projection_v2.txt", A4),
    (
        "src/build_gate/ssc_evidence_proof_contract_v2.txt",
        include_str!("ssc_evidence_proof_contract_v2.txt"),
    ),
    (
        "src/build_gate/ssc_reference_firewall_v2.txt",
        include_str!("ssc_reference_firewall_v2.txt"),
    ),
    (
        "src/build_gate/ssc_containment_governance_contract_v4.txt",
        include_str!("ssc_containment_governance_contract_v4.txt"),
    ),
];
// E1 uses these parent-compiled bytes, not a candidate-updated TA checksum.
const CODE: &[(&str, &[u8])] = &[
    ("src/build_gate/ssc_proof_bundle_preserving_direction_contract_parent_r1485.txt",include_bytes!("ssc_proof_bundle_preserving_direction_contract_parent_r1485.txt")),
    ("src/build_gate/ssc_cursor_temporal_origins_parent_r1444.txt", include_bytes!("ssc_cursor_temporal_origins_parent_r1444.txt")),
    ("src/build_gate/ssc_cursor_temporal_initial_parent_r1444.txt", include_bytes!("ssc_cursor_temporal_initial_parent_r1444.txt")),
    ("src/build_gate/cursor_temporal_retirement_kodex.rs", include_bytes!("cursor_temporal_retirement_kodex.rs")),
    ("src/build_gate/render_checkpoint_prepare_kodex.rs", include_bytes!("render_checkpoint_prepare_kodex.rs")),
    ("src/build_gate/ssc_test_fixture_r1449_render_adopted_state.txt", include_bytes!("ssc_test_fixture_r1449_render_adopted_state.txt")),
    ("src/build_gate/ssc_test_fixture_r1458_initial_render_stage.txt", include_bytes!("ssc_test_fixture_r1458_initial_render_stage.txt")),
    ("src/build_gate/ssc_test_fixture_r1458_checkpoint_owner.txt", include_bytes!("ssc_test_fixture_r1458_checkpoint_owner.txt")),
    (
        "src/build_gate/ssc_k2_retirement_governance.rs",
        include_bytes!("ssc_k2_retirement_governance.rs"),
    ),
    (
        "src/build_gate/ssc_k2_retirement_governance_tests.rs",
        include_bytes!("ssc_k2_retirement_governance_tests.rs"),
    ),
    (
        "src/build_gate/ssc_evidence_proof_shadow_permit_kodex.rs",
        include_bytes!("ssc_evidence_proof_shadow_permit_kodex.rs"),
    ),
    (
        "src/build_gate/ssc_evidence_proof_shadow_permit_kodex_tests.rs",
        include_bytes!("ssc_evidence_proof_shadow_permit_kodex_tests.rs"),
    ),
    (
        "src/build_gate/ssc_structural_erasure.rs",
        include_bytes!("ssc_structural_erasure.rs"),
    ),
    (
        "src/build_gate/ssc_structural_erasure_tests.rs",
        include_bytes!("ssc_structural_erasure_tests.rs"),
    ),
    (
        "src/build_gate/receipt_surface_normalization_kodex.rs",
        include_bytes!("receipt_surface_normalization_kodex.rs"),
    ),
    (
        "src/build_gate/receipt_surface_normalization_kodex_tests.rs",
        include_bytes!("receipt_surface_normalization_kodex_tests.rs"),
    ),
    (
        "src/build_gate/render_sovereignty_kodex.rs",
        include_bytes!("render_sovereignty_kodex.rs"),
    ),
    (
        "src/build_gate/render_sovereignty_kodex_tests.rs",
        include_bytes!("render_sovereignty_kodex_tests.rs"),
    ),
    (
        "src/build_gate/ssc_governed_change.rs",
        include_bytes!("ssc_governed_change.rs"),
    ),
    (
        "src/build_gate/ssc_governed_change_tests.rs",
        include_bytes!("ssc_governed_change_tests.rs"),
    ),
    (
        "src/build_gate/ssc_test_fixture_r1279_ssc_evidence_proof.txt",
        include_bytes!("ssc_test_fixture_r1279_ssc_evidence_proof.txt"),
    ),
    (
        "src/build_gate/ssc_test_fixture_r1279_ssc_shadow_permit.txt",
        include_bytes!("ssc_test_fixture_r1279_ssc_shadow_permit.txt"),
    ),
    (
        "src/build_gate/ssc_test_fixture_r1279_ssc_shadow_measurement.txt",
        include_bytes!("ssc_test_fixture_r1279_ssc_shadow_measurement.txt"),
    ),
    (
        "src/build_gate/ssc_test_fixture_r1279_ssc_shadow_report.txt",
        include_bytes!("ssc_test_fixture_r1279_ssc_shadow_report.txt"),
    ),
    (
        "src/build_gate/ssc_test_fixture_r1279_master_state_identity.txt",
        include_bytes!("ssc_test_fixture_r1279_master_state_identity.txt"),
    ),
    (
        "src/build_gate/ssc_test_fixture_r1279_final_report.txt",
        include_bytes!("ssc_test_fixture_r1279_final_report.txt"),
    ),
    (
        "src/take_change_control.rs",
        include_bytes!("../take_change_control.rs"),
    ),
    (
        "src/build_gate/rust_source_scan_primitives.rs",
        include_bytes!("rust_source_scan_primitives.rs"),
    ),
    (
        "tools/RUST_KODEX_GATE_CORE/src/rust_source_scan_primitives.rs",
        include_bytes!("../../tools/RUST_KODEX_GATE_CORE/src/rust_source_scan_primitives.rs"),
    ),
    (
        "tools/RUST_KODEX_GATE_CORE/src/rust_source_fact_primitives.rs",
        include_bytes!("../../tools/RUST_KODEX_GATE_CORE/src/rust_source_fact_primitives.rs"),
    ),
    ("src/build_gate/ssc_test_fixture_r1485_metronome_tail.txt", include_bytes!("ssc_test_fixture_r1485_metronome_tail.txt")),
];
const UNIQUE_V2: &[&str] = &[
    "SscEvidenceDomain",
    "SscReleasedEventRef",
    "SscEvidenceIntegrityFailure",
    "SscEvidenceIntegrity",
    "SscFamilyScope",
    "SscInvariantCounterexample",
    "SscInvariantAssessment",
    "SscNecessityAssessment",
    "SscPermitVerdictV2",
    "SscClassifiedObservationV2",
    "SscReleasedObservationV2",
    "observe_ssc_event_v2",
    "append_ssc_remaining_projection_v2",
];
const UNIQUE_V1: &[&str] = &[
    "ShadowPermitVerdict",
    "SscShadowObservation",
    "observe_ssc_event",
    "WouldPermit",
    "WouldDeny",
];

pub(crate) fn installed(root: &Path) -> bool {
    root.join(PATH).is_file()
}
fn metadata(root: &Path, key: &str) -> Result<String, String> {
    let text = read(root, "ZIP_KIND.txt").or_else(|_| read(root, "root_files/ZIP_KIND.txt"))?;
    let values = text
        .lines()
        .filter_map(|s| s.split_once('='))
        .filter(|(k, _)| *k == key)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    if values.len() != 1 {
        return Err(fail(format!("K2 take metadata {key} missing/duplicate")));
    }
    Ok(values[0].into())
}
fn rust_sources(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut paths = Vec::new();
    for dir in ["src", "tests", "tools"] {
        if root.join(dir).exists() {
            collect_rust_files(&root.join(dir), &mut paths).map_err(fail)?;
        }
    }
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .map(|p| {
            let name = normalize_path(p.strip_prefix(root).map_err(fail)?);
            Ok((name.clone(), read(root, &name)?))
        })
        .collect()
}
fn runtime_sources(root: &Path) -> Result<Vec<(String, String)>, String> {
    Ok(rust_sources(root)?
        .into_iter()
        .filter(|(p, _)| {
            !p.starts_with("src/build_gate/")
                && !matches!(
                    p.as_str(),
                    "src/take_change_control.rs"
                        | "src/identity_gate_trust_anchor.rs"
                        | "src/kodex_build_orchestrator.rs"
                )
                && !p.starts_with("tools/RUST_KODEX_GATE_CORE/")
        })
        .collect())
}
fn absent(sources: &[(String, String)], needles: &[&str]) -> Result<(), String> {
    for (p, s) in sources {
        let masked = mask_non_code(s);
        for token in needles {
            if contains_token(&masked, token) {
                return Err(fail(format!("retired authority persists: {p}::{token}")));
            }
        }
    }
    Ok(())
}
fn status(root: &Path, id: &str) -> Result<String, String> {
    read(root, POP_PATH)?
        .lines()
        .skip(1)
        .find_map(|s| {
            let row = s.split('\t').collect::<Vec<_>>();
            (row.len() == 11 && row[0] == id).then(|| row[9].to_owned())
        })
        .ok_or_else(|| fail("population row missing"))
}
pub(crate) fn validate_population(root: &Path) -> Result<(), String> {
    let actual = read(root, POP_PATH)?;
    let original = POP.lines().collect::<Vec<_>>();
    let rows = actual.lines().collect::<Vec<_>>();
    if original.len() != rows.len() || original.first() != rows.first() {
        return Err(fail("population shape changed"));
    }
    for (old, new) in original.iter().zip(&rows).skip(1) {
        let a = old.split('\t').collect::<Vec<_>>();
        let mut b = new.split('\t').collect::<Vec<_>>();
        if a.len() != 11 || b.len() != 11 {
            return Err(fail("population columns"));
        }
        let target = match a[0] {
            "24" | "28" | "56" => "RETIRED_SSC_K2",
            "55" | "57" | "67" => "READY",
            _ => a[9],
        };
        if b[9] != a[9] && b[9] != target {
            return Err(fail("population status transition forbidden"));
        }
        let changed = b[9] != a[9];
        b[9] = a[9];
        if b != a {
            return Err(fail("only the named prerequisite_state may change"));
        }
        if changed {
            match a[0] {
                "24" | "56" => {
                    if !cursor_temporal::installed(root) { return Err(fail("cursor retirement requires complete checked successor")); }
                    cursor_temporal::validate_tree(root)?;
                }
                "28" => predicate(root, "wav_pending_28")?,
                "55" => validate_s55(root)?,
                "57" => validate_render57_positive(root)?,
                "67" => predicate(root, "deferred_render_67")?,
                _ => return Err(fail("unnamed status transition")),
            }
        }
    }
    Ok(())
}
// The alias registry itself and compiled checks are historical data, not aliases.
fn alias_absent(root: &Path) -> Result<(), String> {
    let names = [
        concat!("Master", "ExactReceipt"),
        concat!("Master", "BlockReceipt"),
        concat!("Master", "PreservingReceipt"),
        concat!("Master", "AdditiveReceipt"),
    ];
    for (p, s) in rust_sources(root)? {
        if CODE.iter().any(|(path, _)| *path == p) || p == "src/identity_gate_trust_anchor.rs" {
            continue;
        }
        // Freeze deliberately remains in scope, including test strings.
        if names.iter().any(|name| s.contains(name)) {
            return Err(fail(format!("legacy alias remains: {p}")));
        }
    }
    Ok(())
}
// The installed writer must carry exactly one bound V2 construction region.
fn validate_row57_v2_installed(writer: &str) -> Result<(), String> {
    if writer.matches(render57_region(R57_V2)?).count() != 1 {
        return Err(fail("#57 installed V2 construction region missing or duplicated"));
    }
    Ok(())
}
// Fixed accepted-r1269 construction bytes, not candidate-controlled anchors.
// Scope: duration producer, caller, expression, tempo writer and additive owner.
const WAV_METRONOME_ITEMS: &[(&str, &str, &str, u64)] = &[
("src/reference.rs", "fn", "score_quarter_signature", 0x7ff5807f81b2bec8),
("src/reference.rs", "fn", "build_reference_signature", 0x6dfc62a162f52740),
("src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs", "fn", "run_master_notation_and_salience_stage", 0x61de3729eba6af62),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "fn", "apply_expression_polish", 0xe0337540292607bb),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "fn", "insert_tempo_words_direction_into_measure", 0x09abc12d2a333319),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "fn", "insert_direction_into_measure", 0x83226b3b2e07b3a1),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "fn", "find_next_measure_bounds", 0xa06b5639d65a219a),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "fn", "extract_attr_from_opening_tag", 0x044d8cb144dd1e85),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "fn", "extract_first_text_from_fragment", 0x1cb3b850faf68fa4),
("src/structural_write_authority/thirty_surface_productive_authority.rs", "fn", "insert_direction_into_measure_authorized", 0xbefd8c54787c6563),
("src/structural_write_authority/thirty_surface_productive_authority.rs", "impl", "DirectionInsertionCapability", 0xf7385c51c649d3ae),
("src/structural_write_authority/thirty_surface_productive_authority.rs", "fn", "structural_measure_locators", 0x128f729a602ba661),
("src/structural_write_authority/thirty_surface_productive_authority.rs", "fn", "first_structural_measure_locator", 0x287c090e131ed0cd),
("src/structural_write_authority/thirty_surface_productive_authority.rs", "impl", "StructuralMeasureLocator", 0xf3ce2f6f1b52d48f),
];
// Contract v4 B4.1: the proof files are bound by content, not by bytes. The token
// ban is only the minimum; inputs, statics and module dependencies are allowlisted
// and the allowlisted modules must themselves be free of WAV/analysis reach.
const PROOF_FORBIDDEN_WORDS: [&str; 9] = [
    "estimated_bpm", "build_reference_signature", "apply_expression_polish",
    "insert_tempo_words_direction_into_measure", "salience", "wav", "tempo", "selector", "ranking",
];
const PROOF_FORBIDDEN_INPUTS: [&str; 22] = [
    "std::fs", "std::env", "std::process", "std::net", "std::time", "std::os", "std::path",
    "std::thread", "::fs::", "std::{", "core::{", "alloc::{", "crate::{", "env!(", "option_env!(",
    "include_bytes!(", "OnceLock", "LazyLock", "lazy_static", "SystemTime", "externcrate", "#[path",
];
// r1278 F1: closed syntax instead of a token blacklist. Every std/core/alloc path is
// spelled out against this list; aliases, re-exports, foreign macros, extern items and
// out-of-line modules are refused, so no alias can hide an input.
const PROOF_ALLOWED_STD: [&str; 19] = [
    "sync", "collections", "ops", "cell", "mem", "ptr", "io", "fmt", "borrow", "cmp", "iter",
    "str", "string", "vec", "boxed", "rc", "convert", "hash", "option",
];
const PROOF_ALLOWED_MACROS: [&str; 16] = [
    "format", "matches", "vec", "assert", "assert_eq", "assert_ne", "debug_assert",
    "debug_assert_eq", "unreachable", "panic", "thread_local", "include_str", "write", "writeln",
    "concat", "record_commit",
];
const PROOF_DEFINED_MACROS: [&str; 1] = ["record_commit"];
// Exact identity functions the proof may call (r1278 F1: no open ssc_* prefix). The
// later names are reserved accessors for containment verdicts; each must exist as a
// plain fn in the identity module before it may be used.
const PROOF_ALLOWED_IDENTITY_FNS: [&str; 15] = [
    "ssc_additive_identity", "ssc_additive_insertion_class", "ssc_additive_origin_fingerprints",
    "ssc_block_origin_fingerprints", "ssc_exact_origin_fingerprints", "ssc_preserving_descriptor",
    "ssc_preserving_origin_fingerprints", "ssc_preserving_descriptor_name",
    "ssc_exact_descriptor_name", "ssc_block_descriptor_name", "ssc_additive_descriptor_name",
    "ssc_exact_descriptor", "ssc_block_descriptor", "ssc_block_bound_span", "ssc_exact_edit_spans",
];
const PROOF_ALLOWED_IDENTITY_TEST_FNS: [&str; 1] = ["synthetic_exact_receipt_for_tests"];
// Inputs the allowlisted identity module itself may never read (transitive step).
const IDENTITY_FORBIDDEN_INPUTS: [&str; 11] = [
    "std::fs", "std::env", "std::process", "std::net", "std::time", "::fs::", "env!(",
    "option_env!(", "include_bytes!(", "include_str!(", "externcrate",
];
const PROOF_ALLOWED_STATICS: [&str; 3] = ["ADOPTED_EVIDENCE", "PENDING_ADDITIVE", "RELEASED_SCOPES"];
const PROOF_ALLOWED_INCLUDES: [&str; 3] = [
    "build_gate/ssc_exit_dependencies_r1238_v1.tsv",
    "build_gate/ssc_exit_remaining_v1.txt",
    "build_gate/ssc_containment_envelopes_v1.tsv",
];
const PROOF_ALLOWED_CRATE_MODULES: [&str; 6] = [
    "master_state_identity", "final_report", "ssc_evidence_proof", "ssc_shadow_permit",
    "ssc_shadow_measurement", "ssc_shadow_report",
];
const PROOF_ALLOWED_IDENTITY_ITEMS: [&str; 10] = [
    "AdditiveInsertionClass", "AdditiveTransformationReceipt", "BlockTransformationReceipt",
    "ExactTransformationReceipt", "MintedIdentity", "PreservingDescriptor",
    "PreservingTransformationReceipt", "SscReceiptOriginAccess", "TransformationDescriptor",
    "AdditiveDescriptor",
];
const PROOF_ALLOWED_EXTERNAL_CRATES: [&str; 2] = ["roxmltree", "flate2"];
const PROOF_DEPENDENCY_ROOTS: [&str; 2] = ["src/master_state_identity", "src/final_report.rs"];
fn cargo_dependencies(root: &Path) -> Result<Vec<String>, String> {
    // A transaction backup keeps root files under root_files/ (like ZIP_KIND.txt above).
    let manifest = read(root, "Cargo.toml").or_else(|_| read(root, "root_files/Cargo.toml"))?;
    let mut in_dependencies = false;
    let mut out = Vec::new();
    for line in manifest.lines().map(str::trim) {
        if line.starts_with('[') {
            in_dependencies = line == "[dependencies]";
            continue;
        }
        if in_dependencies && let Some((name, _)) = line.split_once('=') {
            out.push(name.trim().replace('-', "_"));
        }
    }
    Ok(out)
}
fn segment(text: &str) -> &str {
    let end = text
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(text.len());
    &text[..end]
}
/// Byte offsets of whole-word occurrences of `word` in masked source.
fn word_positions(masked: &str, word: &str) -> Vec<usize> {
    masked
        .match_indices(word)
        .filter(|(start, _)| {
            let before = masked[..*start].chars().next_back();
            let after = masked[start + word.len()..].chars().next();
            before.is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_'))
                && after.is_none_or(|c| !(c.is_ascii_alphanumeric() || c == '_'))
        })
        .map(|(start, _)| start)
        .collect()
}
fn is_public_use(masked: &str, start: usize) -> bool {
    let head = masked[..start].split_whitespace().collect::<String>();
    ["pub", "pub(crate)", "pub(super)", "pub(self)"]
        .iter()
        .any(|visibility| head.ends_with(visibility))
        || head.ends_with(')') && head.rfind("pub(").is_some_and(|open| !head[open..].contains(['{', '}', ';']))
}
/// r1278 F1: use statements carry no alias and no re-export; macros are known.
fn validate_closed_syntax(masked: &str, firewall: &dyn Fn(String) -> String) -> Result<(), String> {
    for root in ["std", "core", "alloc"] {
        for start in word_positions(masked, root) {
            if !masked[start + root.len()..].trim_start().starts_with("::") {
                return Err(firewall(format!("{root} used as a value or alias, not as a spelled-out path")));
            }
            let compact = masked[start..].split_whitespace().collect::<String>();
            let module = segment(&compact[root.len() + 2..]);
            if !PROOF_ALLOWED_STD.contains(&module) {
                return Err(firewall(format!("{root}::{module} is outside the std allowlist")));
            }
            if module == "io" && segment(&compact[root.len() + 6..]) != "Read" {
                return Err(firewall("std::io is limited to std::io::Read".into()));
            }
        }
    }
    for start in word_positions(masked, "use") {
        let statement = &masked[start..start + masked[start..].find(';').unwrap_or(masked.len() - start)];
        if !word_positions(statement, "as").is_empty() {
            return Err(firewall(format!("aliasing use statement {:?}", statement.trim())));
        }
        if is_public_use(masked, start) {
            return Err(firewall(format!("re-export {:?}", statement.trim())));
        }
    }
    if !word_positions(masked, "extern").is_empty() {
        return Err(firewall("extern items are not allowlisted inputs".into()));
    }
    for start in word_positions(masked, "mod") {
        let rest = masked[start + 3..].trim_start();
        let name = segment(rest);
        if rest[name.len()..].trim_start().starts_with(';') {
            return Err(firewall(format!("out-of-line module {name}")));
        }
    }
    for (index, _) in masked.match_indices('!') {
        let after = masked[index + 1..].trim_start();
        if after.starts_with('=') {
            continue;
        }
        let name_start = masked[..index]
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |position| position + 1);
        let name = &masked[name_start..index];
        if name.is_empty() {
            continue;
        }
        if name == "macro_rules" {
            let defined = segment(after);
            if !PROOF_DEFINED_MACROS.contains(&defined) {
                return Err(firewall(format!("macro definition {defined}")));
            }
            continue;
        }
        if after.starts_with(['(', '[', '{']) && !PROOF_ALLOWED_MACROS.contains(&name) {
            return Err(firewall(format!("macro {name}! is not allowlisted")));
        }
    }
    Ok(())
}
fn validate_proof_source(path: &str, source: &str, dependencies: &[String]) -> Result<(), String> {
    let firewall = |what: String| fail(format!("proof firewall: {path}: {what}"));
    let masked = mask_non_code(source);
    validate_closed_syntax(&masked, &firewall)?;
    let lower = masked.to_ascii_lowercase();
    if let Some(word) = PROOF_FORBIDDEN_WORDS.iter().find(|word| lower.contains(*word)) {
        return Err(firewall(format!("WAV/analysis/selector input {word}")));
    }
    let compact = code(source);
    if let Some(input) = PROOF_FORBIDDEN_INPUTS.iter().find(|input| compact.contains(*input)) {
        return Err(firewall(format!("forbidden input or global state {input}")));
    }
    for (start, _) in compact.match_indices("std::io::") {
        if segment(&compact[start + 9..]) != "Read" {
            return Err(firewall("std::io is limited to std::io::Read".into()));
        }
    }
    for (start, _) in compact.match_indices("super::") {
        if !compact[start + 7..].starts_with('*') {
            return Err(firewall("super:: paths other than super::* reach outside the proof module".into()));
        }
    }
    for (start, _) in compact.match_indices("crate::") {
        let tail = &compact[start + 7..];
        let module = segment(tail);
        if !PROOF_ALLOWED_CRATE_MODULES.contains(&module) {
            return Err(firewall(format!("crate module {module:?} is outside the dependency allowlist")));
        }
        let rest = &tail[module.len()..];
        let names = match rest.strip_prefix("::") {
            Some(group) if group.starts_with('{') => group[1..group.find('}').unwrap_or(group.len())]
                .split(',')
                .map(|name| name.split("::").collect::<Vec<_>>())
                .filter(|name| !name[0].is_empty())
                .collect::<Vec<_>>(),
            Some(single) => {
                let end = single
                    .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == ':'))
                    .unwrap_or(single.len());
                vec![single[..end].trim_end_matches(':').split("::").collect::<Vec<_>>()]
            }
            None => vec![],
        };
        for name in names {
            let allowed = match module {
                "master_state_identity" => {
                    PROOF_ALLOWED_IDENTITY_ITEMS.contains(&name[0])
                        || PROOF_ALLOWED_IDENTITY_FNS.contains(&name[0])
                        || (name[0] == "tests"
                            && name.get(1).is_some_and(|f| PROOF_ALLOWED_IDENTITY_TEST_FNS.contains(f)))
                }
                "final_report" => name[0] == "SscObservationReadAccess" && name.len() == 1,
                _ => true,
            };
            let name = name.join("::");
            if !allowed {
                return Err(firewall(format!("crate::{module}::{name} is outside the input allowlist")));
            }
        }
    }
    for dependency in dependencies {
        if PROOF_ALLOWED_EXTERNAL_CRATES.contains(&dependency.as_str()) {
            continue;
        }
        let needle = format!("{dependency}::");
        if masked.match_indices(&needle).any(|(start, _)| {
            start == 0 || !masked.as_bytes()[start - 1].is_ascii_alphanumeric() && masked.as_bytes()[start - 1] != b'_' && masked.as_bytes()[start - 1] != b':'
        }) {
            return Err(firewall(format!("external crate {dependency} is outside the dependency allowlist")));
        }
    }
    for (start, _) in masked.match_indices("include_str!(") {
        let literal = source[start + 13..]
            .split('"')
            .nth(1)
            .ok_or_else(|| firewall("include_str! without a literal path".into()))?;
        if !PROOF_ALLOWED_INCLUDES.contains(&literal) {
            return Err(firewall(format!("include_str!({literal:?}) is not an allowlisted input")));
        }
    }
    // `'static` lifetimes are not items; a static item starts at a token boundary.
    let words = masked
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '\''))
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    for pair in words.windows(2).filter(|pair| pair[0] == "static") {
        if !PROOF_ALLOWED_STATICS.contains(&pair[1]) {
            return Err(firewall(format!("new global state static {}", pair[1])));
        }
    }
    Ok(())
}
fn validate_proof_firewall(root: &Path) -> Result<(), String> {
    let dependencies = cargo_dependencies(root)?;
    // An absent proof sink carries no input; partial cores are rejected by the SSC gate.
    for path in governed::PROOF_FILES.iter().filter(|path| root.join(path).is_file()) {
        validate_proof_source(path, &read(root, path)?, &dependencies)?;
    }
    // Transitive step: the allowlisted modules carry no WAV/analysis reach either.
    let mut modules = Vec::new();
    for dependency_root in PROOF_DEPENDENCY_ROOTS {
        let path = root.join(dependency_root);
        if path.is_dir() {
            collect_rust_files(&path, &mut modules).map_err(fail)?;
        }
        let file = root.join(format!("{dependency_root}.rs"));
        if file.is_file() {
            modules.push(file);
        } else if path.is_file() {
            modules.push(path);
        }
    }
    if modules.is_empty() {
        return Err(fail("proof firewall: allowlisted dependency modules missing"));
    }
    let mut identity_source = String::new();
    for module in modules {
        let name = normalize_path(module.strip_prefix(root).map_err(fail)?);
        let dependency_source = read(root, &name)?;
        let masked = mask_non_code(&dependency_source);
        if name.starts_with("src/master_state_identity") && name != "src/master_state_identity/tests.rs" {
            // r1278 F1: the identity module reads no ambient input and re-exports nothing
            // but its own submodules, so an allowlisted item cannot alias one.
            let compact = masked.split_whitespace().collect::<String>();
            if let Some(input) = IDENTITY_FORBIDDEN_INPUTS.iter().find(|input| compact.contains(*input)) {
                return Err(fail(format!("proof firewall: allowlisted dependency {name} reads {input}")));
            }
            let submodules = word_positions(&masked, "mod")
                .into_iter()
                .map(|start| segment(masked[start + 3..].trim_start()).to_owned())
                .collect::<Vec<_>>();
            for start in word_positions(&masked, "use") {
                let statement = &masked[start..start + masked[start..].find(';').unwrap_or(masked.len() - start)];
                if !word_positions(statement, "as").is_empty() {
                    return Err(fail(format!("proof firewall: allowlisted dependency {name} aliases {:?}", statement.trim())));
                }
                let public = is_public_use(&masked, start);
                let root_name = segment(statement[3..].trim_start());
                if public && !submodules.iter().any(|module| module == root_name) {
                    return Err(fail(format!("proof firewall: allowlisted dependency {name} re-exports {:?}", statement.trim())));
                }
            }
            identity_source.push_str(&masked);
        }
        if name == "src/final_report.rs" && word_positions(&masked, "SscObservationReadAccess").is_empty() {
            return Err(fail("proof firewall: final_report lacks SscObservationReadAccess"));
        }
        // One exact parent-bound, immutable byte-proof edge; not tempo input.
        // Every changed byte falls back to the unchanged general firewall.
        let checked_edge = name == "src/master_state_identity/render_lineage.rs"
            && cursor_temporal::checked_lineage_successor(&dependency_source);
        let dependency_view = if checked_edge {
            masked.replacen("crate::cursor_lane_temporal_check::CheckedCursorTemporalInput", "", 1)
        } else { masked.clone() };
        let lower = dependency_view.to_ascii_lowercase();
        if let Some(word) = PROOF_FORBIDDEN_WORDS.iter().find(|word| lower.contains(*word)) {
            return Err(fail(format!("proof firewall: allowlisted dependency {name} reaches {word}")));
        }
        if lower.contains("crate::reference") {
            return Err(fail(format!("proof firewall: allowlisted dependency {name} reaches crate::reference")));
        }
    }
    // Every allowlisted identity function the proof names is a real fn there.
    for path in governed::PROOF_FILES.iter().filter(|path| root.join(path).is_file()) {
        let compact = code(&read(root, path)?);
        for function in PROOF_ALLOWED_IDENTITY_FNS {
            if contains_token(&compact, function) && word_positions(&identity_source, &format!("fn {function}")).is_empty() {
                return Err(fail(format!("proof firewall: identity accessor {function} is not a plain fn")));
            }
        }
    }
    if !word_positions(&identity_source, "struct SscObservationReadAccess").is_empty() {
        return Err(fail("proof firewall: identity module shadows the report access type"));
    }
    Ok(())
}
const WAV_METRONOME_USES: &[(&str, &str, usize)] = &[
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "estimated_bpm", 2),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "apply_expression_polish", 1),
("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "insert_tempo_words_direction_into_measure", 4),
("src/pipeline_shards/corpus_fastjump_human_potential_core.rs", "build_reference_signature", 2),
("src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs", "estimated_bpm", 3),
("src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs", "apply_expression_polish", 1),
("src/pipeline_shards/pipeline_runtime_progress_console_core.rs", "build_reference_signature", 1),
("src/reference.rs", "estimated_bpm", 3),
("src/reference.rs", "build_reference_signature", 1),
];
fn validate_wav_metronome_positive(
    root: &Path,
    sources: &[(String, String)],
) -> Result<(), String> {
    for (path, kind, name, expected) in WAV_METRONOME_ITEMS {
        let source = read(root, path)?;
        let actual = crate::rust_source_scan_primitives::fnv1a64(item(&source, kind, name)?.as_bytes());
        if actual != *expected {
            return Err(fail(format!("positive WAV metronome construction changed: {path}::{name}")));
        }
    }
    validate_proof_firewall(root)?;
    // Protect actual uses as well as definitions: a new tempo input/caller cannot
    // acquire authority by leaving the sealed producer and writer unchanged.
    let mut actual = std::collections::BTreeMap::new();
    for (path, source) in sources {
        if path.starts_with("tests/") {
            continue;
        }
        let masked = mask_non_code(source);
        for token in ["estimated_bpm", "build_reference_signature", "apply_expression_polish", "insert_tempo_words_direction_into_measure"] {
            let count = masked.split(|c: char| !c.is_alphanumeric() && c != '_')
                .filter(|word| *word == token).count();
            if count != 0 {
                actual.insert((path.as_str(), token), count);
            }
        }
    }
    let expected = WAV_METRONOME_USES.iter().map(|(p, s, n)| ((*p, *s), *n))
        .collect::<std::collections::BTreeMap<_, _>>();
    if actual != expected {
        return Err(fail("positive WAV metronome source/caller scope changed"));
    }
    absent(sources, &[
        "apply_wav_truth_binding_clear_tiny_accent_render_write", "WavTruthAccent",
        "mint_articulation_fragment_receipt", "run_salience_weighted_candidate_gate",
        "run_discrete_action_salience_translator", "run_conflict_staff_split_proof_trial",
    ])?;
    validate_runtime(root)
}

// Exact one-way admission for the independent P2 no-op identity continuation.
// Runtime stays unchanged in the named Kodex revision that installs these rules.
const R57_NOOP_STAGE_PATH: &str = "src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs";
const R57_NOOP_STAGE_BEFORE: u64 = 0xd6e77ec59a12d30f;
const R57_NOOP_STAGE_AFTER: u64 = 0xd122561046e72a9c;
const R57_NOOP_STAGE_EDITS: &[(&str, &str)] = &[
    ("    if transaction.p2_assertive_return_write_audit.render_written\n        && !transaction\n", "    if transaction.final_render_source_available\n        && !transaction\n"),
    ("            \"RetentionFirstP2OccurrenceContinuation: productive P2 write did not produce a total retained+generated occurrence receipt\"\n", "            \"RetentionFirstP2OccurrenceContinuation: P2 boundary did not produce a total retained+generated occurrence receipt\"\n"),
    ("    if transaction.p2_assertive_return_write_audit.render_written {\n", "    if transaction.final_render_source_available {\n        // Continue a participating P2 boundary even without a new P2 write.\n"),
];

// Independent exact admission; this Kodex revision changes no runtime bytes.
const R57_MATERIAL_STAGE_AFTER: u64 = 0x41cb4d9c8b7fb327;
const R57_MATERIAL_STAGE_EDITS: &[(&str, &str)] = &[
    ("        fragment,\n    };\n    cache.insert(source_range, binding.clone());\n    Ok(binding)\n}\n\nstruct InitialRenderTransaction {\n    render_xml_trial_path: PathBuf,\n    render_xml_pruned_pdf_trial: RenderXmlPrunedPdfTrial,\n    behind_bars_rule_check_rest_and_tie_audit: BehindBarsRuleCheckRestAndTieAudit,\n", "        fragment,\n    };\n    cache.insert(source_range, binding.clone());\n    Ok(binding)\n}\n\n// Representation provenance binds occurrences, not necessarily new attacks.\n// A tie continuation retains its existing attack identity at a later fragment\n// onset. Only an actual onset change enters the attack translation owner.\nfn final_chord_forward_attack_translation(\n    binding: &FinalChordSourceBindingRow,\n    attack: &CandidateMaterialAttackState,\n    source_onset: i64,\n    target_onset: i64,\n) -> anyhow::Result<Option<CandidateMaterialForwardOnsetTranslation>> {\n    let fragment = &binding.fragment;\n    if binding.source_onset != source_onset\n        || fragment.measure != binding.measure\n        || fragment.raw_note_index != binding.raw_note_index\n        || fragment.onset != source_onset\n        || fragment.duration <= 0\n        || fragment.pitch != binding.source_pitch\n        || fragment.pitch != Some(attack.pitch)\n        || fragment.source_event_id != Some(attack.source_event_id)\n        || fragment.attack_id != Some(attack.attack_id)\n        || !attack.exists\n    {\n        return Err(anyhow!(\n            \"FinalChordEncodingSoundingMaterialization: bound occurrence identity disagrees with CandidateMaterialState for SourceEventId {}\",\n            attack.source_event_id.0\n        ));\n    }\n    if source_onset == target_onset && fragment.tie_stop {\n        return Ok(None);\n    }\n    if attack.measure != binding.measure || attack.onset != source_onset {\n        return Err(anyhow!(\n            \"FinalChordEncodingSoundingMaterialization: forward identity bridge disagrees with CandidateMaterialState for SourceEventId {}; fragment_id={} fragment_measure={} attack_measure={} source_onset={} attack_onset={} target_onset={} tie_stop={}\",\n            attack.source_event_id.0, binding.fragment_id, binding.measure, attack.measure,\n            source_onset, attack.onset, target_onset, fragment.tie_stop,\n        ));\n    }\n    if source_onset == target_onset {\n        return Ok(None);\n    }\n    Ok(Some(CandidateMaterialForwardOnsetTranslation {\n        source_event_id: attack.source_event_id,\n        attack_id: attack.attack_id,\n        target_onset,\n    }))\n}\n\n#[cfg(test)]\n#[path = \"../../tests/unit/final_chord_material_identity_continuation.rs\"]\nmod final_chord_material_identity_continuation_tests;\n\nstruct InitialRenderTransaction {\n    render_xml_trial_path: PathBuf,\n    render_xml_pruned_pdf_trial: RenderXmlPrunedPdfTrial,\n    behind_bars_rule_check_rest_and_tie_audit: BehindBarsRuleCheckRestAndTieAudit,\n"),
    ("        let attack = attack_by_source.get(&source_event_id).cloned().ok_or_else(|| {\n            anyhow!(\n                \"FinalChordEncodingSoundingMaterialization: provenance SourceEventId has no SoundingAttack\"\n            )\n        })?;\n        if !attack.exists\n            || attack.measure != binding.measure\n            || attack.pitch != source_pitch\n            || attack.onset != provenance.source_onset\n        {\n            return Err(anyhow!(\n                \"FinalChordEncodingSoundingMaterialization: forward identity bridge disagrees with CandidateMaterialState for SourceEventId {}\",\n                source_event_id.0\n            ));\n        }\n        if !seen_source_ids.insert(source_event_id) || !seen_attack_ids.insert(attack.attack_id) {\n            return Err(anyhow!(\n                \"FinalChordEncodingSoundingMaterialization: planner provenance is not one-to-one in opaque identity\"\n            ));\n        }\n        final_chord_onset_translations.push(CandidateMaterialForwardOnsetTranslation {\n            source_event_id,\n            attack_id: attack.attack_id,\n            target_onset: provenance.projected_sounding_onset,\n        });\n    }\n\n    reconcile_candidate_material_forward_onset_translations(\n        &mut best,\n        \"FinalChordEncodingSoundingMaterialization\",\n", "        let attack = attack_by_source.get(&source_event_id).cloned().ok_or_else(|| {\n            anyhow!(\n                \"FinalChordEncodingSoundingMaterialization: provenance SourceEventId has no SoundingAttack\"\n            )\n        })?;\n        let Some(translation) = final_chord_forward_attack_translation(\n            &binding, &attack, provenance.source_onset, provenance.projected_sounding_onset,\n        )? else {\n            continue;\n        };\n        if !seen_source_ids.insert(source_event_id) || !seen_attack_ids.insert(attack.attack_id) {\n            return Err(anyhow!(\n                \"FinalChordEncodingSoundingMaterialization: planner provenance is not one-to-one in opaque identity\"\n            ));\n        }\n        final_chord_onset_translations.push(translation);\n    }\n\n    reconcile_candidate_material_forward_onset_translations(\n        &mut best,\n        \"FinalChordEncodingSoundingMaterialization\",\n"),
];
fn render57_material_stage_successor(source: &str) -> Result<String, String> {
    if crate::rust_source_scan_primitives::fnv1a64(source.as_bytes()) != R57_NOOP_STAGE_AFTER {
        return Err(fail("#57 material identity transition requires the accepted P2 continuation state"));
    }
    let mut expected = source.to_string();
    for &(before, after) in R57_MATERIAL_STAGE_EDITS {
        if expected.matches(before).count() != 1 || expected.contains(after) {
            return Err(fail("#57 material identity edit binding missing or ambiguous"));
        }
        expected = expected.replacen(before, after, 1);
    }
    if crate::rust_source_scan_primitives::fnv1a64(expected.as_bytes()) != R57_MATERIAL_STAGE_AFTER {
        return Err(fail("#57 material identity complete successor binding drift"));
    }
    Ok(expected)
}

// Read-only original #56 evidence borrow at the existing final-sounding freeze.
// This admits one complete RefactorOnly successor, not a mutable file surface.
const R57_CURSOR_COURSE_STAGE_AFTER: u64 = 0xc93fea242d1e03d5;
#[cfg(test)]
const R57_CURSOR_COURSE_STAGE_BEFORE_TEXT: &str = "        &pre_serialization_occurrences,\n        &pre_serialization_occupancy,\n    )?;\n    crate::structural_write_authority::attach_final_chord_render_occurrence_witnesses(";
#[cfg(test)]
const R57_CURSOR_COURSE_STAGE_AFTER_TEXT: &str = "        &pre_serialization_occurrences,\n        &pre_serialization_occupancy,\n        (\n            transaction.master_export_block_receipt.as_ref(),\n            &transaction.cursor_lane_end_invariant_repair_audit,\n        ),\n    )?;\n    crate::structural_write_authority::attach_final_chord_render_occurrence_witnesses(";

#[cfg(test)]
fn render57_cursor_course_stage_successor(source: &str) -> Result<String, String> {
    if crate::rust_source_scan_primitives::fnv1a64(source.as_bytes()) != R57_MATERIAL_STAGE_AFTER {
        return Err(fail("original cursor evidence borrow requires the exact accepted material stage"));
    }
    if source.matches(R57_CURSOR_COURSE_STAGE_BEFORE_TEXT).count() != 1
        || source.contains(R57_CURSOR_COURSE_STAGE_AFTER_TEXT)
    {
        return Err(fail("original cursor evidence borrow binding missing or ambiguous"));
    }
    let expected = source.replacen(R57_CURSOR_COURSE_STAGE_BEFORE_TEXT,
        R57_CURSOR_COURSE_STAGE_AFTER_TEXT, 1);
    if crate::rust_source_scan_primitives::fnv1a64(expected.as_bytes()) != R57_CURSOR_COURSE_STAGE_AFTER {
        return Err(fail("original cursor evidence borrow complete successor drift"));
    }
    Ok(expected)
}

fn render57_noop_stage_successor(source: &str) -> Result<String, String> {
    if crate::rust_source_scan_primitives::fnv1a64(source.as_bytes()) != R57_NOOP_STAGE_BEFORE {
        return Err(fail("#57 no-op stage transition requires the exact accepted before state"));
    }
    let mut expected = source.to_string();
    for &(before, after) in R57_NOOP_STAGE_EDITS {
        if expected.matches(before).count() != 1 || expected.contains(after) {
            return Err(fail("#57 no-op stage transition edit binding missing or ambiguous"));
        }
        expected = expected.replacen(before, after, 1);
    }
    if crate::rust_source_scan_primitives::fnv1a64(expected.as_bytes()) != R57_NOOP_STAGE_AFTER {
        return Err(fail("#57 no-op stage transition post binding drift"));
    }
    Ok(expected)
}

fn validate_render57_noop_stage_transition(parent: &Path, child: &Path, class: &str) -> Result<(), String> {
    if status(parent, "57")? != "READY" && status(child, "57")? != "READY" {
        return Ok(());
    }
    let before = read(parent, R57_NOOP_STAGE_PATH)?;
    let after = read(child, R57_NOOP_STAGE_PATH)?;
    let before_hash = crate::rust_source_scan_primitives::fnv1a64(before.as_bytes());
    let after_hash = crate::rust_source_scan_primitives::fnv1a64(after.as_bytes());
    if checkpoint_prepare::applies(parent, child) {
        return checkpoint_prepare::validate_transition(parent, child, class);
    }
    if cursor_temporal::stage_successor(before.as_bytes()) || cursor_temporal::stage_successor(after.as_bytes()) {
        if cursor_temporal::stage_successor(before.as_bytes()) {
            return if before==after { Ok(()) } else { Err(fail("checked cursor stage cannot revert or drift")) };
        }
        if before_hash != R57_MATERIAL_STAGE_AFTER || class != "MusicMutation" {
            return Err(fail("checked cursor stage requires exact material parent and MusicMutation"));
        }
        cursor_temporal::validate_tree(child)?;
        return Ok(());
    }
    if before_hash == R57_CURSOR_COURSE_STAGE_AFTER || after_hash == R57_CURSOR_COURSE_STAGE_AFTER {
        return Err(fail("unactivated original cursor evidence borrow retired by temporal-check contract"));
    }
    if before_hash == R57_MATERIAL_STAGE_AFTER || after_hash == R57_MATERIAL_STAGE_AFTER {
        if before_hash == R57_MATERIAL_STAGE_AFTER {
            if after_hash != before_hash {
                return Err(fail("#57 material identity continuation cannot revert or drift"));
            }
            return Ok(());
        }
        if class != "MusicMutation" || before_hash != R57_NOOP_STAGE_AFTER
            || after != render57_material_stage_successor(&before)?
        {
            return Err(fail("#57 material identity transition requires the exact complete MusicMutation successor"));
        }
        return Ok(());
    }
    if ![R57_NOOP_STAGE_BEFORE, R57_NOOP_STAGE_AFTER].contains(&before_hash)
        || ![R57_NOOP_STAGE_BEFORE, R57_NOOP_STAGE_AFTER].contains(&after_hash)
    {
        return Err(fail("#57 no-op stage transition contains an unadmitted state"));
    }
    if before == after {
        return Ok(());
    }
    if before_hash == R57_NOOP_STAGE_AFTER {
        return Err(fail("#57 no-op stage transition cannot revert"));
    }
    if class != "MusicMutation" {
        return Err(fail("#57 no-op stage transition requires MusicMutation"));
    }
    if after != render57_noop_stage_successor(&before)? {
        return Err(fail("#57 no-op stage transition requires the exact complete successor"));
    }
    Ok(())
}

// Accepted r1271 runtime and independent through-tests; fixed compiled literals.
// This discharges the deferred construction prerequisite, not SSC proof debt.
const R57_POSITIVE_FILES: &[(&str, u64)] = &[
    ("src/structural_write_authority/remaining_layout_writer_family.rs", 0xa508b9ab6d210241),
    ("src/pipeline_shards/notation_endpoint_layout_core.rs", 0xdee2bc3ee4911734),
    ("src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs", 0x87ee9a605fd9783c),
    (R57_NOOP_STAGE_PATH, R57_NOOP_STAGE_BEFORE),
    ("src/structural_write_authority/master_adoption/render_adopted_state.rs", 0x51997746bec0b556),
    ("src/structural_write_authority/verified_render_candidate_sink_authority.rs", 0x3006a9194cf07cfe),
    ("src/structural_write_authority/row57_prepare_boundary_tests.rs", 0x447ad1468112a7c7),
    ("tests/support/render_authority/row57_prepares_file_bound_post_without_writing_before_adoption.rs", 0x9f01d122a8fcff16),
    ("tests/support/render_authority/row57_local_clef_scope_through_trial.rs", 0x861b6cdd726d2976),
];
// Contract v4 B4.1: #57 binds its accepted construction items in the identity file,
// not the whole file; SSC work elsewhere in that file stays possible.
const R57_POSITIVE_ITEMS: &[(&str, &str, &str, u64)] = &[
    ("src/master_state_identity.rs", "const", "RENDER_ROW_57_BLOCK", 0xb3336919b2fa2c15),
    ("src/master_state_identity.rs", "struct", "BlockTransformationReceipt", 0xec0b40541e58d5ae),
    // r1278 F6: the whole receipt impl, so the delegated mint, the post split and the
    // block transition check cannot drift behind an unchanged wrapper.
    ("src/master_state_identity.rs", "impl", "BlockTransformationReceipt", 0x669e110149ca2403),
];
fn validate_render57_positive(root: &Path) -> Result<(), String> {
    for (path, kind, name, expected) in R57_POSITIVE_ITEMS {
        let source = read(root, path)?;
        let actual = crate::rust_source_scan_primitives::fnv1a64(item(&source, kind, name)?.as_bytes());
        if actual != *expected {
            return Err(fail(format!(
                "#57 accepted construction item changed: {path}::{name} ({actual:#018x})"
            )));
        }
    }
    validate_row57_v2_installed(&read(root, R57_WRITER_PATH)?)?;
    if status(root, "57")? != "READY" {
        return Err(fail("#57 population governance not READY"));
    }
    for (path, expected) in R57_POSITIVE_FILES {
        let bytes = std::fs::read(root.join(path)).map_err(fail)?;
        let actual = crate::rust_source_scan_primitives::fnv1a64(&bytes);
        let exact_noop_successor = *path == R57_NOOP_STAGE_PATH && (actual == R57_NOOP_STAGE_AFTER
            || actual == R57_MATERIAL_STAGE_AFTER || cursor_temporal::stage_successor(&bytes));
        let exact_checked_test_successor = *path == "tests/support/render_authority/row57_local_clef_scope_through_trial.rs" && cursor_temporal::row57_test_successor(&bytes);
        let exact_prepared_adoption = *path == "src/structural_write_authority/master_adoption/render_adopted_state.rs" && (checkpoint_prepare::render_successor(&bytes) || (actual != *expected && checkpoint_prepare::writer_retirement_state(root)?));
        if actual != *expected && !exact_noop_successor && !exact_checked_test_successor && !exact_prepared_adoption {
            return Err(fail(format!("#57 accepted construction/adoption/test binding changed: {path}")));
        }
    }
    validate_runtime(root)
}

fn predicate(root: &Path, id: &str) -> Result<(), String> {
    let sources = runtime_sources(root)?;
    match id {
        "deferred_render_57" => validate_render57_positive(root),
        "deferred_render_67" => {
            if status(root, "67")? != "READY" {
                return Err(fail("#67 not READY"));
            }
            let k = &crate::take_change_control::render_sovereignty_kodex::validate_row67_source(
                &read(
                    root,
                    "src/render/silent_voice_visible_measure_coverage_render_writer.rs",
                )?,
            )?;
            if *k != crate::take_change_control::render_sovereignty_kodex::Row67State::Relocated {
                return Err(fail("#67 not Relocated"));
            }
            // The full K-R gate checks exact target-use count, mint, geometry and pins.
            crate::take_change_control::render_sovereignty_kodex::validate_current_tree(root)
                .map(|_| ())
        }
        "wav_pending_28" => {
            if status(root, "28")? != "RETIRED_SSC_K2" {
                return Err(fail("#28 not retired status"));
            }
            absent(
                &sources,
                &[
                    "apply_bound_articulation_batch_authorized",
                    "RenderRow28ExactMint",
                    "ARTICULATION_BATCH_RENDER_EXACT",
                    "ArticulationBatch",
                ],
            )?;
            predicate(root, "wav_accent_productive")
        }
        "wav_accent_productive" => absent(
            &sources,
            &[
                "apply_wav_truth_binding_clear_tiny_accent_render_write",
                "WavTruthAccent",
                "mint_articulation_fragment_receipt",
                "ArticulationFragmentAdditiveMint",
                "E029_FRAGMENT_ADDITIVE",
                "ArticulationFragment",
                "apply_bound_articulation_batch_authorized",
            ],
        ),
        "wav_salience_master_adoptions" => absent(
            &sources,
            &[
                "run_salience_weighted_candidate_gate",
                "run_discrete_action_salience_translator",
                "run_conflict_staff_split_proof_trial",
            ],
        ),
        "wav_metronome" => validate_wav_metronome_positive(root, &sources),
        "e031_tenuto" => absent(
            &sources,
            &[
                "add_tenuto_to_note_fragment_authorized",
                "TenutoFragment",
                "mint_tenuto_fragment_receipt",
                "TenutoFragmentAdditiveMint",
                "E031_FRAGMENT_ADDITIVE",
                "close_structural_accent_tenuto",
            ],
        ),
        "ra5_accent_reserve" => absent(
            &sources,
            &[
                "apply_accent_reserve_top_six_micro_write_candidate",
                "write_accent_reserve_checkpoint_authorized",
                "AccentReserve",
            ],
        ),
        "master_receipt_legacy_alias_contract" => alias_absent(root),
        "audit_carrier_grandfathering" => {
            let source = read(root, "src/build_gate/destructive_carrier_effect_scan.rs")?;
            if contains_token(&mask_non_code(&source), "NON_PRODUCTIVE_REBUILD_PREFIXES")
                || source.contains("src/audit")
            {
                return Err(fail("audit prefix grandfathering remains"));
            }
            if super::structural_erasure::tokens(&source)?
                != super::structural_erasure::tokens(include_str!(
                    "ssc_carrier_without_grandfathering_fixture_v1.txt"
                ))?
            {
                return Err(fail("general carrier inventory exact template changed"));
            }
            Ok(())
        }
        _ => {
            let (owner, variant) = match id {
                "macro_hand_role" => (
                    "apply_hand_role_rest_materialization_authorized",
                    "HandRoleRestToForward",
                ),
                "macro_cross_staff" => (
                    "apply_cross_staff_voice_ledger_authorized",
                    "CrossStaffVoiceLedger",
                ),
                "macro_terminal_shadow" => (
                    "apply_terminal_shadow_rest_forward_authorized",
                    "TerminalShadowRestToForward",
                ),
                "macro_same_pitch_tie" => {
                    ("apply_same_pitch_tie_merge_authorized", "SamePitchTieMerge")
                }
                "macro_lower_rhythm_lane" => (
                    "apply_lower_visible_rhythm_rest_lane_authorized",
                    "LowerVisibleRhythmRestLane",
                ),
                _ => return Err(fail("unknown K2 obligation")),
            };
            absent(&sources, &[owner, variant])
        }
    }
}
fn validate_s55(root: &Path) -> Result<(), String> {
    let path = "src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs";
    let source = read(root, path)?;
    let masked = mask_non_code(&source);
    if contains_token(&masked, "rename") || contains_token(&masked, "copy") {
        return Err(fail("#55 raw rename/copy remains in publication file"));
    }
    let function = item(
        &source,
        "fn",
        "build_render_xml_hidden_rest_forward_collapse_trial",
    )?;
    let compact = code(function);
    for marker in [
        "RenderSinkKind::HiddenRestForwardCollapse",
        "adopt_trial_publication_transition(",
        "validate_musicxml_export_semantics(&tmp_path)",
        "write_hidden_rest_forward_collapse_trial_authorized(source_xml,candidate_xml,&tmp_path,)",
        "write_hidden_rest_forward_collapse_trial_authorized(source_xml,candidate_xml,artifact_path,)",
    ] {
        if !compact.contains(marker) {
            return Err(fail(format!("#55 S55-A binding missing: {marker}")));
        }
    }
    let expected = item(
        include_str!("ssc_s55_a_publication_fixture_v1.txt"),
        "fn",
        "build_render_xml_hidden_rest_forward_collapse_trial",
    )?;
    if super::structural_erasure::tokens(function)? != super::structural_erasure::tokens(expected)?
    {
        return Err(fail("#55 S55-A exact callback/control flow differs"));
    }
    Ok(())
}
fn shape_items(source: &str) -> Vec<(&str, &str)> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let line = line.strip_prefix("pub(crate) ").unwrap_or(line);
            let mut words = line.split_whitespace();
            let kind = words.next()?;
            if !matches!(kind, "enum" | "struct") {
                return None;
            }
            let name = words.next()?.split('<').next()?;
            Some((kind, name))
        })
        .collect()
}
pub(crate) fn is_v2(root: &Path) -> Result<bool, String> {
    if !root.join(PROOF_PATH).is_file() {
        return Ok(false);
    }
    let proof = read(root, PROOF_PATH)?;
    Ok(contains_token(
        &mask_non_code(&proof),
        "SscReleasedObservationV2",
    ))
}
// The proof must match the complete installed surface.
#[cfg(test)]
fn surface_mismatch(proof: &str, surface: &str) -> Result<Option<String>, String> {
    surface_mismatch_extended(proof, surface, &[])
}
// Contract v4 surface_extend: the installed V3 surface plus the governed, append-only
// extensions (variants or named fields appended at the end of an existing item).
fn surface_mismatch_extended(
    proof: &str,
    surface: &str,
    extensions: &[(String, String)],
) -> Result<Option<String>, String> {
    let items = shape_items(surface);
    for (owner, _) in extensions {
        if !items.iter().any(|(kind, name)| *name == owner.as_str() && matches!(*kind, "enum" | "struct")) {
            return Ok(Some(owner.clone()));
        }
    }
    for (kind, name) in items {
        let mut expected = super::structural_erasure::tokens(item(surface, kind, name)?)?;
        let close = expected.pop();
        if close.as_deref() != Some("}") {
            return Ok(Some(name.to_owned()));
        }
        for (_, source) in extensions.iter().filter(|(owner, _)| owner.as_str() == name) {
            let mut variant = super::structural_erasure::tokens(source)?;
            if variant.last().map(String::as_str) != Some(",") {
                variant.push(",".to_owned());
            }
            expected.extend(variant);
        }
        expected.push("}".to_owned());
        if super::structural_erasure::tokens(item(proof, kind, name)?)? != expected {
            return Ok(Some(name.to_owned()));
        }
    }
    Ok(None)
}
// Contract v4 B4.1 / r1278 F2: the observer stays the single verdict entry with an exact,
// parent-compiled signature. Only the V2 form and the V4 containment form exist; a new
// parameter, type or generic is never a sealed input, whatever its name.
const OBSERVER_V4_HEAD: &str = "fn observe_ssc_event_v2<'event>(
    transformation: &'event SscTransformationEvidence<'event>,
    invariant: SscInvariantAssessment<'event>,
    necessity: SscNecessityAssessment<'event>,
    containment: SscContainmentAssessment<'event>,
) -> Result<
    SscClassifiedObservationV2<'event>,
    SscEvidenceIntegrityFailure,
>";
fn observer_head(source: &str) -> Result<Vec<String>, String> {
    let observer = item(source, "fn", "observe_ssc_event_v2")?;
    let masked = mask_non_code(observer);
    let open = masked.find('{').ok_or_else(|| fail("observer body missing"))?;
    super::structural_erasure::tokens(&observer[..open])
}
fn validate_observer_inputs(proof: &str) -> Result<(), String> {
    let head = observer_head(proof)?;
    if head != observer_head(OBSERVER)? && head != super::structural_erasure::tokens(OBSERVER_V4_HEAD)? {
        return Err(fail(
            "proof firewall: observer signature is neither the V2 nor the V4 containment form; inputs are sealed",
        ));
    }
    Ok(())
}
// Reserved enforcement entry of contract v4 A7; it may only be wired at adoption boundaries.
const VERDICT_ENTRY: &str = "ssc_adoption_verdict";
/// Exactly the frozen V4 containment surface (plus governed unit variants).
fn is_surface_v4(root: &Path, proof: &str) -> Result<bool, String> {
    if item(proof, "enum", "SscContainmentVerdict").is_err() {
        return Ok(false);
    }
    let extensions = governed::extensions(root)?;
    if let Some(name) = surface_mismatch_extended(proof, SURFACE_V4, &extensions)? {
        return Err(fail(format!("V4 exact surface mismatch {name}")));
    }
    Ok(true)
}
pub(crate) fn validate_runtime(root: &Path) -> Result<(), String> {
    let v2 = is_v2(root)?;
    let sources = runtime_sources(root)?;
    if !v2 {
        return Err(fail("pre-V2 SSC proof surface is older than the epoch base"));
    }
    let proof = read(root, PROOF_PATH)?;
    super::forbid_tokens(
        &proof,
        &[
            "audit_only",
            "audits",
            "feedback_proof_transport",
            "FeedbackPipelineProof",
            "ProofMath",
            "WavSalience",
            "wav_truth_articulation",
            "ValidatorCounter",
            "Deserialize",
            "FromStr",
            "mint_fresh",
            "opaque_value",
            "transmute",
            "unsafe",
        ],
        "V2 origin proof cannot be reconstructed from diagnostics or surrogate identity",
    )?;
    let compact = code(&proof);
    for marker in [
        "implFrom<",
        "implTryFrom<",
        "implDefaultforSsc",
        "pubtypeSsc",
        "pub(crate)typeSsc",
    ] {
        if compact.contains(marker) {
            return Err(fail("V2 conversion/alias proof factory forbidden"));
        }
    }
    let masked_proof = mask_non_code(&proof);
    for (start, _) in masked_proof.match_indices("pub") {
        let tail = &masked_proof[start..];
        let Some(open) = tail.find('{') else {
            continue;
        };
        let head = &tail[..open];
        if head.contains("fn ")
            && !head.contains("fn observe_ssc_event_v2")
            && head.contains("->")
            && shape_items(SURFACE_V4)
                .iter()
                .any(|(_, name)| contains_token(head, name))
        {
            return Err(fail(
                "V2 constructive carrier factory/getter cannot be publicly exported",
            ));
        }
    }
    let v4 = is_surface_v4(root, &proof)?;
    if !v4 {
        let extensions = governed::extensions(root)?;
        if let Some(name) = surface_mismatch_extended(&proof, SURFACE, &extensions)? {
            return Err(fail(format!("V3 exact surface mismatch {name}")));
        }
    }
    validate_observer_inputs(&proof)?;
    if !v4 && observer_head(&proof)? != observer_head(OBSERVER)? {
        return Err(fail("proof firewall: V4 observer requires the frozen V4 containment surface"));
    }
    validate_proof_firewall(root)?;
    let mut wired = std::collections::BTreeMap::<String, Vec<String>>::new();
    for target in governed::wiring(root)? {
        if let Some((path, function)) = target.rsplit_once("::") {
            wired.entry(path.to_owned()).or_default().push(function.to_owned());
        }
    }
    let report = read(root, "src/ssc_shadow_report.rs")?;
    if super::structural_erasure::tokens(item(
        &report,
        "fn",
        "append_ssc_remaining_projection_v2",
    )?)? != super::structural_erasure::tokens(item(
        A4,
        "fn",
        "append_ssc_remaining_projection_v2",
    )?)? {
        return Err(fail("V2 missing exact A4"));
    }
    if !code(item(&report, "fn", "append_ssc_shadow_observation")?)
        .contains("append_ssc_remaining_projection_v2(out)")
    {
        return Err(fail("A4 not invoked by report"));
    }
    for (p, s) in &sources {
        let masked = mask_non_code(s);
        for name in UNIQUE_V1 {
            if contains_token(&masked, name) {
                return Err(fail(format!("V1 remains in V2: {p}::{name}")));
            }
        }
        if !matches!(
            p.as_str(),
            PROOF_PATH
                | SHADOW_PATH
                | "src/ssc_shadow_measurement.rs"
                | "src/ssc_shadow_report.rs"
                | "src/final_report.rs"
        ) {
            // r1278 F4: a wired adoption function may name the verdict; the rest of its
            // file may not.
            let mut masked = masked;
            for function in wired.get(p.as_str()).into_iter().flatten() {
                let bound = item(s, "fn", function)?;
                let start = bound.as_ptr() as usize - s.as_ptr() as usize;
                masked.replace_range(start..start + bound.len(), &" ".repeat(bound.len()));
            }
            let protected = shape_items(SURFACE_V4)
                .into_iter()
                .map(|(_, name)| name)
                .chain(UNIQUE_V2.iter().copied())
                .chain([VERDICT_ENTRY]);
            for name in protected {
                if contains_token(&masked, name) {
                    return Err(fail(format!("V2 escapes observation: {p}::{name}")));
                }
            }
        }
    }
    Ok(())
}
const NEUTRAL_REGISTRY_PATH: &str = "src/build_gate/ssc_neutral_retirement_nodes_v1.txt";
const COMPLETION_REGISTRY_PATH: &str = "src/build_gate/ssc_retirement_completion_nodes_v2.txt";
const WAV_COMPLETION_REGISTRY_PATH: &str = "src/build_gate/ssc_wav_retirement_completion_nodes_v2.txt";
const WAV_REGISTRY_PATH: &str = "src/build_gate/ssc_wav_retirement_nodes_v1.txt";

fn render57_region(fixture: &str) -> Result<&str, String> {
    fixture
        .split_once("\nfn verify_range(")
        .map(|(region, _)| region.trim_end())
        .ok_or_else(|| fail("render57 fixture region boundary missing"))
}
// Epoch base: every admissible parent already carries the installed V2 writer.
// It stays byte-identical until a named governance revision defines a later contract.
fn validate_render57_writer_transition(parent: &Path, child: &Path) -> Result<(), String> {
    let old = read(parent, R57_WRITER_PATH)?;
    validate_row57_v2_installed(&old)?;
    if read(child, R57_WRITER_PATH)? != old {
        return Err(fail(
            "installed #57 V2 writer is frozen until a named governance revision defines a later construction contract",
        ));
    }
    Ok(())
}
fn validate_population_no_revert(old: &str, new: &str) -> Result<(), String> {
    if old.lines().count() != new.lines().count() {
        return Err(fail("population shape changed"));
    }
    for (old, new) in old.lines().zip(new.lines()).skip(1) {
        let o = old.split('\t').collect::<Vec<_>>();
        let n = new.split('\t').collect::<Vec<_>>();
        if o.len() != 11 || n.len() != 11 || o[0] != n[0] {
            return Err(fail("population malformed"));
        }
        if o[9] != n[9]
            && n[9]
                == POP
                    .lines()
                    .find_map(|s| {
                        let r = s.split('\t').collect::<Vec<_>>();
                        (r.len() == 11 && r[0] == n[0]).then(|| r[9])
                    })
                    .unwrap_or("")
        {
            return Err(fail("population cannot revert"));
        }
    }
    Ok(())
}

// r1288 contract v4 closure (Teil C, Take 6). The SSC exit is closed by this compiled record,
// never by a manifest line. The record is valid only while the r1238 remaining set is empty,
// both A7 adoption boundaries are wired and call the adoption verdict, the report states
// active enforcement and the decision stays a single Permit over the report's classifier.
// The runtime-pinned A4 projection keeps its frozen schema token `exit_claim=not_closed`.
pub(crate) const EXIT_CLOSURE_PATH: &str = "src/build_gate/ssc_exit_closure_v1.txt";
const EXIT_CLOSURE: &str = include_str!("ssc_exit_closure_v1.txt");
const A7_ENFORCED_BOUNDARIES: [&str; 2] = [
    "src/master_state_identity.rs::commit_owner_receipts",
    "src/master_state_identity/render_lineage.rs::commit",
];
pub(crate) fn validate_exit_closure(root: &Path) -> Result<(), String> {
    if !root.join(EXIT_CLOSURE_PATH).is_file() {
        return Ok(());
    }
    if read(root, EXIT_CLOSURE_PATH)? != EXIT_CLOSURE {
        return Err(fail("SSC exit closure record changed"));
    }
    if !remaining(&read(root, REMAINING_PATH)?)?.is_empty() {
        return Err(fail("SSC exit closure requires an empty remaining set"));
    }
    let wired = governed::wiring(root)?;
    for target in A7_ENFORCED_BOUNDARIES {
        let Some((path, function)) = target.rsplit_once("::") else {
            return Err(fail("SSC exit closure boundary malformed"));
        };
        if !wired.contains(target) || !code(item(&read(root, path)?, "fn", function)?).contains("ssc_adoption_verdict(") {
            return Err(fail(format!("SSC exit closure requires the enforcing A7 boundary {target}")));
        }
    }
    let report = read(root, "src/ssc_shadow_report.rs")?;
    if !report.contains("PermitEnforcement: active") || report.contains("PermitEnforcement: none") {
        return Err(fail("SSC exit closure requires PermitEnforcement: active"));
    }
    let proof = read(root, PROOF_PATH)?;
    let entry = code(item(&proof, "fn", "ssc_adoption_verdict")?);
    let decision = code(item(&proof, "fn", "adoption_decision")?);
    if !entry.contains("classify_bound_origin(") || !entry.contains("adoption_decision(") || !decision.contains("permitted==1") {
        return Err(fail("SSC exit closure requires the single-Permit decision over the report classifier"));
    }
    Ok(())
}
fn validate_exit_closure_transition(parent: &Path, child: &Path) -> Result<(), String> {
    if parent.join(EXIT_CLOSURE_PATH).is_file() && !child.join(EXIT_CLOSURE_PATH).is_file() {
        return Err(fail("SSC exit closure cannot be withdrawn"));
    }
    validate_exit_closure(child)
}
pub(crate) fn validate_tree(root: &Path) -> Result<(), String> {
    if !installed(root) {
        return Ok(());
    }
    let writer_retired=checkpoint_prepare::writer_retirement_state(root)?;
    for (p, s) in DOCUMENTS {
        // The only contract successor is part of the complete source/data tuple;
        // historical proof contracts and all other documents remain unchanged.
        if writer_retired && *p=="src/build_gate/ssc_proof_bundle_preserving_direction_contract_v1.txt" { continue; }
        if read(root, p)? != *s {
            return Err(fail(format!("compiled K2 document changed {p}")));
        }
    }
    if read(root, BASELINE_PATH)? != BASELINE {
        return Err(fail("K2 baseline changed"));
    }
    governed::validate_tree(root)?;
    cursor_temporal::validate_tree(root)?;
    checkpoint_prepare::validate_tree(root)?;
    let baseline = baseline_ids(BASELINE)?;
    let open = remaining(&read(root, REMAINING_PATH)?)?;
    if !open.is_subset(&baseline) {
        return Err(fail("K2 unknown open ID"));
    }
    validate_runtime(root)?;
    for id in baseline.difference(&open) {
        predicate(root, id)?;
    }
    validate_exit_closure(root)?;
    validate_population(root)
}
fn named_revision(parent: &Path, child: &Path, class: &str) -> Result<bool, String> {
    Ok(class == "KodexOnly"
        && metadata(child, "name")? == "SscK2GovernanceRevisionKodex"
        && metadata(child, "parent_revision")? == metadata(parent, "revision")?
        && metadata(child, "revision")? != metadata(parent, "revision")?)
}
pub(crate) fn validate_transition(parent: &Path, child: &Path, class: &str) -> Result<(), String> {
    let before = installed(parent);
    let after = installed(child);
    if before && !after {
        return Err(fail("K2 governance cannot disappear"));
    }
    if !after {
        return Ok(());
    }
    if !before {
        return Err(fail(
            "K2 installation epoch is closed; a parent without K2 is older than the epoch base",
        ));
    }
    if read(child, WAV_COMPLETION_REGISTRY_PATH)? != super::structural_erasure::WAV_COMPLETION_NODES {
        return Err(fail("incoming WAV completion registry missing or changed"));
    }
    if read(child, WAV_REGISTRY_PATH)? != super::structural_erasure::WAV_NODES {
        return Err(fail("incoming WAV registry missing or changed"));
    }
    {
        let revision = named_revision(parent, child, class)?;
        if !revision {
            for (p, b) in CODE {
                if std::fs::read(child.join(p)).map_err(fail)? != *b {
                    return Err(fail(format!(
                        "K2 checker change requires named governance revision: {p}"
                    )));
                }
            }
        }
        let a = read(parent, POP_PATH)?;
        let b = read(child, POP_PATH)?;
        validate_population_no_revert(&a, &b)?;
    }
    cursor_temporal::validate_transition(parent, child, class)?;
    checkpoint_prepare::validate_transition(parent, child, class)?;
    validate_render57_writer_transition(parent, child)?;
    validate_render57_noop_stage_transition(parent, child, class)?;
    validate_exit_closure_transition(parent, child)?;
    validate_anchor_scope(parent, child, class)?;
    validate_tree(child)?;
    if item(&read(parent, PROOF_PATH)?, "enum", "SscContainmentVerdict").is_ok()
        && item(&read(child, PROOF_PATH)?, "enum", "SscContainmentVerdict").is_err()
    {
        return Err(fail("installed V4 containment surface cannot return to V3"));
    }
    // Contract v4 B3.2: GovernedChange kinds are admitted by these parent-compiled rules;
    // a child validator never admits itself (CODE stays frozen above).
    governed::validate_transition(parent, child, class, named_revision(parent, child, class)?)
}
#[cfg(test)]
#[path = "ssc_k2_retirement_governance_tests.rs"]
mod tests;

pub(crate) fn validate_removed_edges(
    parent: &Path,
    child: &Path,
    symbols: &str,
    edges: &str,
) -> Result<(), String> {
    let mut retired = std::collections::BTreeSet::new();
    let parent_sources = runtime_sources(parent)?;
    let child_sources = runtime_sources(child)?;
    let mut definitions = std::collections::BTreeMap::<String, usize>::new();
    for (_, source) in &parent_sources {
        let tokens = super::structural_erasure::tokens(source)?;
        for pair in tokens.windows(2).filter(|pair| pair[0] == "fn") {
            *definitions.entry(pair[1].clone()).or_default() += 1;
        }
    }
    for bound in symbols.split(',').map(str::trim) {
        let (path, name) = bound
            .rsplit_once("::")
            .ok_or_else(|| fail("retired authority requires path::function"))?;
        if !path.starts_with("src/")
            || !path.ends_with(".rs")
            || path.contains("..")
            || name.is_empty()
        {
            return Err(fail("invalid retired function path"));
        }
        let source = read(parent, path)?;
        let is_function = item(&source, "fn", name).is_ok();
        if !is_function && !super::structural_erasure::declared_closed_symbol(parent, path, name)? {
            return Err(fail(
                "retired symbol is not a unique function or compiled closed item",
            ));
        }
        if is_function && definitions.get(name).copied().unwrap_or(0) != 1 {
            return Err(fail("ambiguous retired function target"));
        }
        absent(&child_sources, &[name])?;
        retired.insert(bound);
    }
    for edge in edges.split(',').map(str::trim) {
        let (caller, callee) = edge
            .split_once("->")
            .ok_or_else(|| fail("retired edge requires caller->callee"))?;
        if !retired.contains(callee.trim()) {
            return Err(fail("edge does not bind a retired function"));
        }
        let (path, name) = caller
            .trim()
            .rsplit_once("::")
            .ok_or_else(|| fail("edge caller requires path::function"))?;
        if !path.starts_with("src/") || !path.ends_with(".rs") || path.contains("..") {
            return Err(fail("invalid edge caller path"));
        }
        let source = read(parent, path)?;
        let body = item(&source, "fn", name)?;
        let target = callee
            .trim()
            .rsplit("::")
            .next()
            .ok_or_else(|| fail("callee missing"))?;
        if !contains_token(&mask_non_code(body), target) {
            return Err(fail("declared caller edge absent in parent"));
        }
    }
    Ok(())
}

// These are physical source triggers, independent of the OPEN projection.
// Historical dependency rows remain immutable; their old proxy anchors are not authority.
fn retirement_symbols(id: &str) -> Result<Vec<&'static str>, String> {
    Ok(match id {
        "wav_pending_28" => vec![
            "apply_bound_articulation_batch_authorized",
            "RenderRow28ExactMint",
            "ARTICULATION_BATCH_RENDER_EXACT",
            "ArticulationBatch",
        ],
        "wav_accent_productive" => vec![
            "apply_wav_truth_binding_clear_tiny_accent_render_write",
            "WavTruthAccent",
            "mint_articulation_fragment_receipt",
            "ArticulationFragmentAdditiveMint",
            "E029_FRAGMENT_ADDITIVE",
            "ArticulationFragment",
        ],
        "wav_salience_master_adoptions" => vec![
            "run_salience_weighted_candidate_gate",
            "run_discrete_action_salience_translator",
            "run_conflict_staff_split_proof_trial",
        ],
        "wav_metronome" => vec!["estimated_bpm"],
        "e031_tenuto" => vec![
            "add_tenuto_to_note_fragment_authorized",
            "TenutoFragment",
            "mint_tenuto_fragment_receipt",
            "TenutoFragmentAdditiveMint",
            "TENUTO_FRAGMENT_ADDITIVE_MINT",
            "E031_FRAGMENT_ADDITIVE",
            "close_structural_accent_tenuto",
            "ExternalFeedbackArticulationClosureStats",
        ],
        "ra5_accent_reserve" => vec![
            "apply_accent_reserve_top_six_micro_write_candidate",
            "write_accent_reserve_checkpoint_authorized",
            "AccentReserve",
        ],
        "macro_hand_role" => vec![
            "apply_hand_role_rest_materialization_authorized",
            "HandRoleRestToForward",
        ],
        "macro_cross_staff" => vec![
            "apply_cross_staff_voice_ledger_authorized",
            "CrossStaffVoiceLedger",
        ],
        "macro_terminal_shadow" => vec![
            "apply_terminal_shadow_rest_forward_authorized",
            "TerminalShadowRestToForward",
        ],
        "macro_same_pitch_tie" => {
            vec!["apply_same_pitch_tie_merge_authorized", "SamePitchTieMerge"]
        }
        "macro_lower_rhythm_lane" => vec![
            "apply_lower_visible_rhythm_rest_lane_authorized",
            "LowerVisibleRhythmRestLane",
        ],
        "master_receipt_legacy_alias_contract" => vec![
            concat!("Master", "ExactReceipt"),
            concat!("Master", "BlockReceipt"),
            concat!("Master", "PreservingReceipt"),
            concat!("Master", "AdditiveReceipt"),
        ],
        "audit_carrier_grandfathering" => vec!["NON_PRODUCTIVE_REBUILD_PREFIXES"],
        "deferred_render_57" | "deferred_render_67" => vec![],
        _ => return Err(fail("unknown retirement obligation")),
    })
}
fn scope_value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    let values = text
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter(|(k, _)| k.trim() == key)
        .map(|(_, v)| v.trim())
        .collect::<Vec<_>>();
    if values.len() != 1 || values[0].is_empty() {
        return Err(fail(format!("retirement scope {key} missing/duplicate")));
    }
    Ok(values[0])
}
pub(crate) fn validate_obligation_symbol_binding(
    parent: &Path,
    ids: &str,
    symbols: &str,
) -> Result<(), String> {
    let ids = ids
        .split(',')
        .map(str::trim)
        .collect::<std::collections::BTreeSet<_>>();
    if ids.contains("") {
        return Err(fail("empty retirement obligation"));
    }
    let mut bound = std::collections::BTreeSet::new();
    for entry in symbols.split(',').map(str::trim) {
        let (path, name) = entry
            .rsplit_once("::")
            .ok_or_else(|| fail("retired symbol must bind path::name"))?;
        if !path.starts_with("src/") || !path.ends_with(".rs") || path.contains("..") {
            return Err(fail("invalid retired symbol path"));
        }
        let source = read(parent, path)?;
        let historical_alias = ids.contains("master_receipt_legacy_alias_contract")
            && retirement_symbols("master_receipt_legacy_alias_contract")?.contains(&name)
            && path == "src/build_gate/destructive_population_freeze_gate.rs";
        if !contains_token(&mask_non_code(&source), name)
            && !(historical_alias && source.contains(name))
            && !super::structural_erasure::declared_closed_symbol(parent, path, name)?
        {
            return Err(fail("retired symbol absent from bound parent source"));
        }
        let matches = ids
            .iter()
            .filter_map(|id| {
                retirement_symbols(id)
                    .ok()
                    .filter(|names| names.contains(&name))
                    .map(|_| *id)
            })
            .collect::<Vec<_>>();
        // Auxiliary sealed nodes also bind exactly one obligation in the compiled registry.
        let registry = include_str!("ssc_neutral_retirement_nodes_v1.txt");
        let matches = if matches.is_empty() {
            registry
                .lines()
                .chain(super::structural_erasure::COMPLETION_NODES.lines())
                .chain(super::structural_erasure::WAV_NODES.lines())
                .chain(super::structural_erasure::WAV_COMPLETION_NODES.lines())
                .filter_map(|line| {
                    let row = line.split('|').collect::<Vec<_>>();
                    ((row.len() == 7 || row.len() == 8)
                        && matches!(row[0], "node" | "replacement")
                        && row[3] == path
                        && row[4] == name
                        && ids.contains(row[2]))
                    .then(|| row[2])
                })
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            matches
        };
        if matches.len() != 1 {
            return Err(fail(
                "retired symbol does not bind exactly one declared obligation",
            ));
        }
        bound.insert(matches[0]);
    }
    if bound != ids {
        return Err(fail("retirement obligation lacks a bound parent symbol"));
    }
    Ok(())
}
fn validate_anchor_scope(parent: &Path, child: &Path, class: &str) -> Result<(), String> {
    let mut lost = std::collections::BTreeSet::new();
    let mut parent_sources = runtime_sources(parent)?;
    parent_sources.push((
        "src/build_gate/destructive_carrier_effect_scan.rs".into(),
        read(parent, "src/build_gate/destructive_carrier_effect_scan.rs").unwrap_or_default(),
    ));
    parent_sources.push((
        "src/build_gate/destructive_population_freeze_gate.rs".into(),
        read(
            parent,
            "src/build_gate/destructive_population_freeze_gate.rs",
        )
        .unwrap_or_default(),
    ));
    for id in baseline_ids(BASELINE)? {
        let names = retirement_symbols(&id)?;
        for (path, old) in &parent_sources {
            let new = std::fs::read_to_string(child.join(path)).unwrap_or_default();
            if old == &new {
                continue;
            }
            let old_raw = old;
            let new_raw = &new;
            let old = mask_non_code(old);
            let new = mask_non_code(&new);
            for name in &names {
                let count = |s: &str| {
                    s.split(|c: char| !c.is_alphanumeric() && c != '_')
                        .filter(|s| s == name)
                        .count()
                };
                if count(&old) > count(&new) {
                    lost.insert(id.clone());
                }
            }
            // Alias retirement intentionally includes historical diagnostic strings.
            if id == "master_receipt_legacy_alias_contract"
                && names
                    .iter()
                    .any(|name| old_raw.matches(name).count() > new_raw.matches(name).count())
            {
                lost.insert(id.clone());
            }
        }
    }
    if lost.is_empty() {
        return Ok(());
    }
    let manifest = read(child, "src/build_gate/take_change_class.txt")?;
    let contract = manifest
        .lines()
        .filter_map(|line| line.strip_prefix("authority_retirement_contract="))
        .collect::<Vec<_>>();
    let text = if contract.len() == 1 {
        read(child, contract[0])?
    } else {
        // Test-only physical retirement has no productive capability loss. It still
        // needs a manifest-bound scope; it never removes an OPEN ID by itself.
        if !matches!(class, "RefactorOnly" | "KodexOnly")
            || lost.iter().any(|id| {
                !id.starts_with("macro_")
                    && !matches!(
                        id.as_str(),
                        "master_receipt_legacy_alias_contract" | "audit_carrier_grandfathering"
                    )
            })
        {
            return Err(fail(
                "physical SSC anchor loss requires a bound SSC retirement contract, even while OPEN",
            ));
        }
        let scopes = manifest
            .lines()
            .filter_map(|line| line.strip_prefix("change="))
            .filter_map(|line| {
                let row = line.split('|').collect::<Vec<_>>();
                (row.len() == 3
                    && matches!(row[0], "added" | "modified")
                    && row[1].starts_with("docs/ssc_retirement_scope_")
                    && row[1].ends_with(".md"))
                .then(|| row[1])
            })
            .collect::<Vec<_>>();
        if scopes.len() != 1 {
            return Err(fail("test witness retirement requires one declared scope"));
        }
        let text = read(child, scopes[0])?;
        if scope_value(&text, "SscNonProductiveOnly")? != "true" {
            return Err(fail("test witness scope must be explicitly nonproductive"));
        }
        text
    };
    let ids = scope_value(&text, "SscExitObligations")?;
    let symbols = scope_value(&text, "RetiredAuthoritySymbols")?;
    validate_obligation_symbol_binding(parent, ids, symbols)?;
    let declared = ids
        .split(',')
        .map(str::trim)
        .collect::<std::collections::BTreeSet<_>>();
    if lost.iter().any(|id| !declared.contains(id.as_str())) {
        return Err(fail("physical SSC anchor loss is undeclared"));
    }
    Ok(())
}

