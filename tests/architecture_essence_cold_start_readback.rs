use std::fs;
use std::path::{Path, PathBuf};

const START_HERE: &str = "docs/assistant_memory/000_start_here.md";
const BOOTSTRAP: &str = "docs/assistant_memory/architecture_essence_bootstrap_v0.json";
const ESSENCE: &str = "docs/assistant_memory/020_architecture_essence_v0.md";
const AIC: &str = "docs/assistant_memory/architecture_inventory_contract_v1.json";
const AIF: &str = "docs/assistant_memory/architecture_inventory_assistant_fast_path_v1.json";
const KNOWN_FAILURE_PATTERNS: &str = "docs/assistant_memory/010_known_failure_patterns.md";
const HISTORICAL_EXTERNAL_MEMORY: &str = "docs/assistant_external_memory.md";
const READBACK_CONTRACT: &str = "docs/assistant_memory/030_cold_start_readback_contract.md";
const READBACK_TEST: &str = "tests/architecture_essence_cold_start_readback.rs";

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(root: &Path, relative: &str) -> String {
    fs::read_to_string(root.join(relative))
        .unwrap_or_else(|error| panic!("cold-start source {relative} must be readable: {error}"))
}

fn assert_repository_path_exists(root: &Path, relative: &str) {
    assert!(
        root.join(relative).exists(),
        "cold-start reference must resolve inside current repository: {relative}"
    );
}

#[test]
fn architecture_essence_bootstrap_resolves_without_granting_authority() {
    let root = project_root();
    let bootstrap = read(&root, BOOTSTRAP);

    for relative in [
        START_HERE,
        ESSENCE,
        AIC,
        AIF,
        KNOWN_FAILURE_PATTERNS,
        HISTORICAL_EXTERNAL_MEMORY,
        READBACK_CONTRACT,
        READBACK_TEST,
    ] {
        assert_repository_path_exists(&root, relative);
        assert!(
            bootstrap.contains(&format!("\"{relative}\"")),
            "bootstrap must point to repository source {relative}"
        );
    }

    assert!(bootstrap.contains("\"class\": \"advisory_only\""));
    assert!(bootstrap.contains("\"normative\": false"));
    assert!(bootstrap.contains("\"productive_authority\": []"));
    assert!(bootstrap.contains(
        "Never infer permission from this bootstrap or from the essence."
    ));
    assert!(bootstrap.contains(
        "The JSON is an index/bootstrap only. Full rule rationale"
    ));
    assert!(bootstrap.contains("\"class\": \"read_only_regression\""));
    assert!(bootstrap.contains("\"grants_authority\": false"));
    assert!(bootstrap.contains("\"retirement_preflight\""));
    assert!(bootstrap.contains("productive_reachability_absent_or_cutover_proven"));
    assert!(bootstrap.contains(
        "apparent_rollout_switch_checked_for_independent_single_owner_literal_or_source_witness_role"
    ));
    assert!(bootstrap.contains(
        "provenance_carries_known_identity_readback_proves_persisted_state"
    ));
    assert!(bootstrap.contains("\"source_snapshot_vs_take_tree\""));
}

#[test]
fn architecture_essence_rule_index_resolves_exactly_e00_through_e31() {
    let root = project_root();
    let bootstrap = read(&root, BOOTSTRAP);
    let essence = read(&root, ESSENCE);

    let headings = essence
        .lines()
        .filter(|line| {
            let bytes = line.as_bytes();
            bytes.len() >= 5
                && line.starts_with("# E")
                && bytes[3].is_ascii_digit()
                && bytes[4].is_ascii_digit()
        })
        .collect::<Vec<_>>();
    // Symbol name is retained for semantic-capability continuity; the live contract now extends through E32.
    assert_eq!(headings.len(), 33, "Essence v0 must contain exactly E00-E32");

    for number in 0..=32 {
        let id = format!("E{number:02}");
        let heading_prefix = format!("# {id} – ");
        assert_eq!(
            headings
                .iter()
                .filter(|heading| heading.starts_with(&heading_prefix))
                .count(),
            1,
            "{id} must resolve to exactly one Essence heading"
        );
        assert_eq!(
            bootstrap.matches(&format!("\"id\": \"{id}\"")).count(),
            1,
            "{id} must occur exactly once in bootstrap rules_index"
        );
        assert!(
            bootstrap.contains(&format!("\"source_heading\": \"# {id} – \"")),
            "{id} bootstrap entry must resolve by heading prefix"
        );
    }
}

#[test]
fn architecture_essence_discovery_hints_are_live_but_non_authoritative() {
    let root = project_root();
    let bootstrap = read(&root, BOOTSTRAP);

    let discovery_paths = [
        "src/take_change_control.rs",
        "src/build_gate/semantic_capability_diff.rs",
        "src/build_gate/productive_write_path_scan.rs",
        "src/structural_write_authority/",
        "src/identity_gate_trust_anchor.rs",
        "src/build_gate/formal_green_contract.rs",
        "src/build_gate/managed_source_tree_definition.rs",
    ];

    for relative in discovery_paths {
        assert_repository_path_exists(&root, relative);
        assert!(
            bootstrap.contains(&format!("\"{relative}\"")),
            "discovery hint must remain represented in bootstrap: {relative}"
        );
    }

    assert!(bootstrap.contains(
        "Discovery hints only. The current repository source decides whether a path still owns the named contract"
    ));
}

#[test]
fn start_here_preserves_repository_first_cold_start_order() {
    let root = project_root();
    let start_here = read(&root, START_HERE);

    let bootstrap_at = start_here.find(BOOTSTRAP).expect("bootstrap path in start-here");
    let essence_at = start_here.find(ESSENCE).expect("essence path in start-here");
    let aic_at = start_here.find(AIC).expect("AIC path in start-here");
    let aif_at = start_here.find(AIF).expect("AIF path in start-here");
    let history_at = start_here
        .find(HISTORICAL_EXTERNAL_MEMORY)
        .expect("historical memory path in start-here");

    assert!(bootstrap_at < essence_at);
    assert!(essence_at < aic_at);
    assert!(aic_at < aif_at);
    assert!(aif_at < history_at);
    assert!(start_here.contains("Die Essenz ist **advisory only**."));
    assert!(start_here.contains(
        "kanonischen Contracts, Authorities und Gates aus dem aktuellen Source-Baum"
    ));
}

#[test]
fn architecture_inventory_contract_v1_is_resolvable_advisory_and_complete_in_shape() {
    let root = project_root();
    let bootstrap = read(&root, BOOTSTRAP);
    let aic = read(&root, AIC);

    assert_repository_path_exists(&root, AIC);
    assert!(bootstrap.contains(AIC));
    assert!(bootstrap.contains("\"contract_version\": \"AIC-1\""));

    assert!(aic.contains("\"contract_version\": \"AIC-1\""));
    assert!(aic.contains("\"class\": \"advisory_only\""));
    assert!(aic.contains("\"normative\": false"));
    assert!(aic.contains("\"productive_authority\": []"));
    assert!(aic.contains("\"runtime_implementation\": \"none\""));
    assert!(aic.contains("\"extraction_basis\": \"rust_source_ast\""));
    assert!(aic.contains("\"regex_only_extraction_allowed\": false"));
    assert!(aic.contains("\"one_symbol_one_row\": true"));
    assert!(aic.contains("\"default_unreviewed_state\": \"OFFEN\""));
    assert!(aic.contains("Gesamt = A + B + C + N + OFFEN"));
    assert!(aic.contains("src/**/*.rs"));
    assert!(aic.contains("tests/**/*.rs"));
    assert!(aic.contains("tools/qualified_components/**"));
    assert_eq!(
        aic.matches("\"position\": ").count(),
        22,
        "AIC-1 must define exactly the frozen 22 inventory columns"
    );
    assert!(aic.contains("\"table_start_row\": 8"));
    assert!(aic.contains("\"sheet\": \"Inventur\""));
    assert!(aic.contains("\"table_name\": \"EtuSchlopfInventur\""));
}

#[test]
fn architecture_inventory_assistant_fast_path_v1_is_resolvable_advisory_and_semantics_preserving() {
    let root = project_root();
    let bootstrap = read(&root, BOOTSTRAP);
    let aif = read(&root, AIF);

    assert_repository_path_exists(&root, AIF);
    assert!(bootstrap.contains(AIF));
    assert!(bootstrap.contains("\"profile_version\": \"AIF-1\""));

    assert!(aif.contains("\"profile_version\": \"AIF-1\""));
    assert!(aif.contains("\"class\": \"advisory_only\""));
    assert!(aif.contains("\"normative\": false"));
    assert!(aif.contains("\"productive_authority\": []"));
    assert!(aif.contains("\"runtime_implementation\": \"none\""));
    assert!(aif.contains("\"required_contract_version\": \"AIC-1\""));
    assert!(aif.contains("\"may_change_aic_semantics\": false"));
    assert!(aif.contains("\"key\": \"source_snapshot_sha256\""));
    assert!(aif.contains("single_rust_syntax_pass_build_symbol_universe_and_repository_token_call_index"));
    assert!(aif.contains("A_B_C_N_OFFEN_classification"));
    assert!(aif.contains("\"whole_repository_rescan_per_symbol\": false"));
    assert!(aif.contains("\"second_full_source_pass_for_call_index\": false"));
    assert!(aif.contains("\"old_xlsx_as_cache_or_truth\": false"));
    assert!(aif.contains("\"log_reconstruction_of_native_metrics\": false"));
    assert!(aif.contains("\"preferred_export_count\": 1"));
}

#[test]
fn early_take_tree_integrity_precedes_expensive_post_patch_proofs() {
    let root = project_root();
    let bootstrap = read(&root, BOOTSTRAP);
    let essence = read(&root, ESSENCE);

    assert!(essence.contains("# E31 – Billige fail-closed Strukturchecks vor teuren Beweisen"));
    assert!(essence.contains("unknown_top_level_paths = 0"));
    assert!(essence.contains(
        "Ein Pfad wird niemals nur deshalb erlaubt, um einen Candidate durch das Gate zu bringen"
    ));
    assert!(essence.contains("Provenienz traegt Identitaet; Readback beweist Zustand."));
    assert!(essence.contains("produktive Reachability"));
    assert!(essence.contains("Single-Owner-/Contract-Witness"));
    assert!(essence.contains(
        "Source-Snapshot und ein eingehender Take-Baum sind getrennte Artefakte"
    ));

    let tree_at = bootstrap
        .find("take_tree_integrity_allowed_top_level_roles_and_unknown_zero")
        .expect("early Take-Tree-Integrity step in bootstrap");
    let diff_at = bootstrap
        .find("final_actual_diff_vs_take_declaration")
        .expect("diff/declaration step in bootstrap");
    let semantic_at = bootstrap
        .find("capability_semantic_diff")
        .expect("semantic capability step in bootstrap");
    let call_graph_at = bootstrap
        .find("productive_call_graph")
        .expect("productive call graph step in bootstrap");
    let build_at = bootstrap
        .find("strict_clippy_build_tests")
        .expect("build/clippy step in bootstrap");

    assert!(tree_at < diff_at);
    assert!(diff_at < semantic_at);
    assert!(semantic_at < call_graph_at);
    assert!(call_graph_at < build_at);
}

