use super::*;
// r1280: frozen fixtures (state r1279) instead of live runtime or governed data files,
// so a GovernedChange take that legitimately changes those files cannot break these
// parent-compiled tests, and the tests keep judging the same inputs.
const FIXTURE_ENVELOPES: &str = "schema=ssc_containment_envelopes_v1\n";
const FIXTURE_EXTENSIONS: &str = "schema=ssc_surface_extensions_v1\n";
const FIXTURE_WIRING: &str = "schema=ssc_verdict_wiring_v1\n";

#[test]
fn wav_completion_registry_is_mandatory_and_revision_alone_grants_no_exception() {
    let t = Tree::new();
    std::fs::remove_file(t.0.join(WAV_COMPLETION_REGISTRY_PATH)).unwrap();
    assert!(validate_tree(&t.0).unwrap_err().contains(WAV_COMPLETION_REGISTRY_PATH));
    t.put("ZIP_KIND.txt", "revision=r1257\n");
    assert!(validate_tree(&t.0).unwrap_err().contains(WAV_COMPLETION_REGISTRY_PATH));
    let c = Tree::new();
    std::fs::remove_file(c.0.join(WAV_COMPLETION_REGISTRY_PATH)).unwrap();
    assert!(validate_transition(&Tree::new().0, &c.0, "KodexOnly").is_err());
}

#[test]
fn wav_completion_helpers_bind_only_their_declared_obligation() {
    let t = Tree::new();
    let path = "src/pipeline_shards/audio_salience_discrete_translator_core.rs";
    let symbol = "run_salience_weighted_candidate_gate";
    let row = super::super::structural_erasure::WAV_COMPLETION_NODES
        .lines().map(|s| s.split('|').collect::<Vec<_>>())
        .find(|r| r.len() == 7 && r[1].ends_with("_salience_gate_row_class_measures")).unwrap();
    let bytes = (0..row[6].len()).step_by(2)
        .map(|i| u8::from_str_radix(&row[6][i..i+2], 16).unwrap()).collect::<Vec<_>>();
    t.bytes(path, &bytes);
    let bound = format!("{path}::{symbol}");
    validate_obligation_symbol_binding(&t.0, "wav_salience_master_adoptions", &bound).unwrap();
    assert!(validate_obligation_symbol_binding(&t.0, "wav_accent_productive", &bound).is_err());
    assert!(validate_obligation_symbol_binding(&t.0, "wav_salience_master_adoptions,wav_pending_28", &bound).is_err());
    assert!(validate_obligation_symbol_binding(&t.0, "wav_salience_master_adoptions", &format!("{path}::unregistered_helper")).is_err());
}

#[test]
fn wav_registry_cannot_disappear_from_a_later_tree() {
    let t = Tree::new();
    std::fs::remove_file(t.0.join(WAV_REGISTRY_PATH)).unwrap();
    assert!(validate_tree(&t.0).unwrap_err().contains(WAV_REGISTRY_PATH));
    t.put("ZIP_KIND.txt", "revision=r1256\n");
    assert!(validate_tree(&t.0).unwrap_err().contains(WAV_REGISTRY_PATH));
}

#[test]
fn private_wav_nodes_bind_only_their_registered_obligation() {
    let t = Tree::new();
    let path = "src/pipeline_shards/sheaf_noether_harmonic_micro_tail.rs";
    let symbol = "AudioSalienceAudit";
    let registry = super::super::structural_erasure::WAV_NODES;
    let row = registry.lines().map(|s| s.split('|').collect::<Vec<_>>()).find(|r| r.len() == 7 && r[4] == symbol).unwrap();
    let bytes = (0..row[6].len()).step_by(2).map(|i| u8::from_str_radix(&row[6][i..i+2], 16).unwrap()).collect::<Vec<_>>();
    t.bytes(path, &bytes);
    let bound = format!("{path}::{symbol}");
    validate_obligation_symbol_binding(&t.0, "wav_salience_master_adoptions", &bound).unwrap();
    assert!(validate_obligation_symbol_binding(&t.0, "wav_accent_productive", &bound).is_err());
    assert!(validate_obligation_symbol_binding(&t.0, "wav_salience_master_adoptions,wav_metronome", &bound).is_err());
}
use std::sync::atomic::{AtomicU64, Ordering};
const INSTALLED_WRITER: &str =
    include_str!("../structural_write_authority/remaining_layout_writer_family.rs");
struct Tree(std::path::PathBuf);
impl Tree {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let t = Self(std::env::temp_dir().join(format!(
            "ssc-k2-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )));
        for (p, s) in DOCUMENTS {
            t.put(p, s);
        }
        for (p, b) in CODE {
            t.bytes(p, b);
        }
        t.put(super::super::CONTRACT_PATH, super::super::CONTRACT);
        t.put(BASELINE_PATH, BASELINE);
        t.put(REMAINING_PATH, super::super::INITIAL_REMAINING);
        t.put(POP_PATH, POP);
        t.put(R57_WRITER_PATH, INSTALLED_WRITER);
        t.put(governed::ENVELOPES_PATH, FIXTURE_ENVELOPES);
        t.put(governed::EXTENSIONS_PATH, FIXTURE_EXTENSIONS);
        t.put(governed::WIRING_PATH, FIXTURE_WIRING);
        t.put("src/master_state_identity.rs", include_str!("ssc_test_fixture_r1279_master_state_identity.txt"));
        t.put("src/final_report.rs", include_str!("ssc_test_fixture_r1279_final_report.txt"));
        t.put("Cargo.toml", include_str!("../../Cargo.toml"));
        complete_v2(&t);
        t.put("ZIP_KIND.txt","kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r2002\nparent_revision=r2001\nname=SyntheticEpochTree\n");
        t
    }
    fn bytes(&self, p: &str, b: &[u8]) {
        let path = self.0.join(p);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b).unwrap();
    }
    fn put(&self, p: &str, s: &str) {
        self.bytes(p, s.as_bytes());
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn closed_install_epochs_reject_parents_older_than_the_epoch_base() {
    let p = Tree::new();
    let c = Tree::new();
    validate_transition(&p.0, &c.0, "KodexOnly").unwrap();
    std::fs::remove_file(p.0.join(PATH)).unwrap();
    p.put("ZIP_KIND.txt", "revision=r1241\n");
    assert!(validate_transition(&p.0, &c.0, "KodexOnly").unwrap_err().contains("epoch base"));
    let p = Tree::new();
    std::fs::remove_file(p.0.join(PROOF_PATH)).unwrap();
    std::fs::remove_file(p.0.join(SHADOW_PATH)).unwrap();
    assert!(validate_tree(&p.0).unwrap_err().contains("epoch base"));
}
#[test]
fn undeclared_checker_change_blocks_two_hop_weakening() {
    let p = Tree::new();
    let c = Tree::new();
    c.put(
        "src/build_gate/ssc_k2_retirement_governance.rs",
        "fn validate_tree(){ /* weakened */ }",
    );
    assert!(
        validate_transition(&p.0, &c.0, "KodexOnly")
            .unwrap_err()
            .contains("checker change")
    );
}
#[test]
fn governance_and_documents_cannot_disappear_or_change() {
    let p = Tree::new();
    let c = Tree::new();
    std::fs::remove_file(c.0.join(PATH)).unwrap();
    assert!(validate_transition(&p.0, &c.0, "KodexOnly").is_err());
    let c = Tree::new();
    c.put(
        "src/build_gate/ssc_evidence_proof_contract_v2.txt",
        "weakened",
    );
    assert!(validate_tree(&c.0).is_err());
}
#[test]
fn baseline_alias_registry_does_not_make_resolution_impossible() {
    let t = Tree::new();
    alias_absent(&t.0).unwrap();
    t.put(
        "src/build_gate/destructive_population_freeze_gate.rs",
        concat!("const OLD: &str = \"Master", "ExactReceipt\";"),
    );
    assert!(alias_absent(&t.0).is_err());
}
#[test]
fn scope_empty_owner_cannot_close_publication() {
    let t = Tree::new();
    t.put(
        "src/pipeline_shards/render_polish_hidden_rest_readability_tail.rs",
        "fn collapse_hidden_timeline_rests_to_forward(){} fn publish(){std::fs::rename(a,b);}",
    );
    assert!(validate_s55(&t.0).is_err());
}
#[test]
fn id_removal_requires_actual_parent_v2_not_a_status_claim() {
    let p = Tree::new();
    let c = Tree::new();
    c.put(
        REMAINING_PATH,
        &super::super::INITIAL_REMAINING.replace("open=deferred_render_57\n", ""),
    );
    assert!(
        validate_transition(&p.0, &c.0, "KodexOnly")
            .unwrap_err()
            .contains("population governance")
    );
}
#[test]
fn row57_ready_and_non_status_population_edits_are_rejected() {
    let t = Tree::new();
    let mut s = POP.to_owned();
    let row = POP.lines().find(|s| s.starts_with("57\t")).unwrap();
    s = s.replace(row, &row.replace("DEFERRED_RENDER_PREREQUISITE", "READY"));
    t.put(POP_PATH, &s);
    assert!(validate_population(&t.0).is_err());
    t.put(POP_PATH, &POP.replacen("DESTRUCTIVE", "ALTERED", 1));
    assert!(validate_population(&t.0).is_err());
}
#[test]
fn pre_v2_or_partial_proof_surfaces_are_rejected() {
    let t = Tree::new();
    t.put(PROOF_PATH, "struct SscInvariantCounterexample {}");
    assert!(validate_runtime(&t.0).unwrap_err().contains("epoch base"));
}

fn complete_v2(t: &Tree) {
    t.put(PROOF_PATH, &format!("{SURFACE}\n{OBSERVER}"));
    t.put(SHADOW_PATH, "// V2 observer is in the proof core.\n");
    t.put("src/ssc_shadow_report.rs", &format!("{A4}\npub(crate) fn append_ssc_shadow_observation(out: &mut String) {{ append_ssc_remaining_projection_v2(out); }}"));
}
#[test]
fn complete_v2_is_admitted_as_a_future_form() {
    let t = Tree::new();
    complete_v2(&t);
    super::super::validate_current_tree(&t.0).unwrap();
}
#[test]
fn v2_cannot_revert_and_a4_literal_changes_are_rejected() {
    let p = Tree::new();
    let c = Tree::new();
    std::fs::remove_file(c.0.join(PROOF_PATH)).unwrap();
    assert!(
        validate_transition(&p.0, &c.0, "RefactorOnly")
            .unwrap_err()
            .contains("epoch base")
    );
    p.put("src/ssc_shadow_report.rs", &format!("{}\npub(crate) fn append_ssc_shadow_observation(out: &mut String) {{ append_ssc_remaining_projection_v2(out); }}",A4.replace("retirement={}","retirement=none{}")));
    assert!(validate_runtime(&p.0).is_err());
}

#[test]
fn real_row28_mint_and_descriptor_block_retirement() {
    let t = Tree::new();
    let row = POP.lines().find(|s| s.starts_with("28\t")).unwrap();
    let cols = row.split('\t').collect::<Vec<_>>();
    t.put(
        POP_PATH,
        &POP.replace(row, &row.replace(cols[9], "RETIRED_SSC_K2")),
    );
    predicate(&t.0, "wav_pending_28").unwrap();
    for symbol in [
        "RenderRow28ExactMint",
        "ARTICULATION_BATCH_RENDER_EXACT",
        "ArticulationBatch",
    ] {
        t.put("src/leftover.rs", &format!("struct {symbol};"));
        assert!(predicate(&t.0, "wav_pending_28").is_err(), "{symbol}");
    }
}
#[test]
fn physical_anchor_loss_needs_scope_even_while_id_remains_open() {
    let p = Tree::new();
    let c = Tree::new();
    let path = "src/pipeline_shards/noether_register_articulation_readback_tail.rs";
    p.put(
        path,
        "fn apply_accent_reserve_top_six_micro_write_candidate(){} fn live(){}",
    );
    c.put(path, "fn live(){}");
    c.put("src/build_gate/take_change_class.txt", "schema_version=1\n");
    assert!(
        validate_anchor_scope(&p.0, &c.0, "MusicMutation")
            .unwrap_err()
            .contains("bound SSC retirement contract")
    );
    let doc = "docs/authority_retirement_contract_neutral.md";
    c.put(
        "src/build_gate/take_change_class.txt",
        &format!("authority_retirement_contract={doc}\n"),
    );
    c.put(doc,&format!("SscExitObligations: e031_tenuto\nRetiredAuthoritySymbols: {path}::apply_accent_reserve_top_six_micro_write_candidate\n"));
    assert!(validate_anchor_scope(&p.0, &c.0, "MusicMutation").is_err());
    c.put(doc,&format!("SscExitObligations: ra5_accent_reserve\nRetiredAuthoritySymbols: {path}::apply_accent_reserve_top_six_micro_write_candidate\n"));
    validate_anchor_scope(&p.0, &c.0, "MusicMutation").unwrap();
}
#[test]
fn test_only_witness_scope_is_explicit_and_cannot_cover_productive_loss() {
    let p = Tree::new();
    let c = Tree::new();
    let path = "src/structural_write_authority/strict_staff_visible_lane_authority.rs";
    p.put(
        path,
        "#[cfg(test)] fn apply_terminal_shadow_rest_forward_authorized(){}",
    );
    c.put(path, "");
    let doc = "docs/ssc_retirement_scope_test.md";
    c.put(
        "src/build_gate/take_change_class.txt",
        &format!("change=added|{doc}|fnv64:0000000000000001\n"),
    );
    c.put(doc,&format!("SscNonProductiveOnly: true\nSscExitObligations: macro_terminal_shadow\nRetiredAuthoritySymbols: {path}::apply_terminal_shadow_rest_forward_authorized\n"));
    validate_anchor_scope(&p.0, &c.0, "RefactorOnly").unwrap();
    assert!(validate_anchor_scope(&p.0, &c.0, "MusicMutation").is_err());
    p.put("src/master_state_identity.rs", "struct TenutoFragment;");
    assert!(validate_anchor_scope(&p.0, &c.0, "RefactorOnly").is_err());
}

#[test]
fn registries_have_no_historical_absence_exception() {
    for path in [NEUTRAL_REGISTRY_PATH, COMPLETION_REGISTRY_PATH] {
        let t = Tree::new();
        std::fs::remove_file(t.0.join(path)).unwrap();
        for metadata in [
            "revision=r1243\nname=AssistantHandoffHygieneClosure\n",
            "revision=r1244\nname=SscK2GovernanceRevisionKodex\n",
        ] {
            t.put("ZIP_KIND.txt", metadata);
            assert!(validate_tree(&t.0).is_err(), "{path}");
        }
        t.put(path, "altered");
        assert!(
            validate_tree(&t.0)
                .unwrap_err()
                .contains("compiled K2 document changed")
        );
    }
}
#[test]
fn exact_function_name_count_does_not_confuse_probe_and_authorized_owner() {
    let t = super::super::structural_erasure::tokens(
        "fn probe() {} fn probe_authorized() {} // fn probe()\n",
    )
    .unwrap();
    assert_eq!(
        t.windows(2)
            .filter(|w| w[0] == "fn" && w[1] == "probe")
            .count(),
        1
    );
}

#[test]
fn row57_installed_v2_does_not_grant_population_discharge() {
    let t = Tree::new();
    validate_row57_v2_installed(INSTALLED_WRITER).unwrap();
    assert!(
        predicate(&t.0, "deferred_render_57")
            .unwrap_err()
            .contains("population")
    );
    let region = render57_region(R57_V2).unwrap();
    for forged in [
        INSTALLED_WRITER.replace(region, ""),
        format!("{INSTALLED_WRITER}\n{region}\n"),
    ] {
        t.put(R57_WRITER_PATH, &forged);
        assert!(predicate(&t.0, "deferred_render_57").unwrap_err().contains("V2"));
    }
}

#[test]
fn population_baseline_is_independent_and_requires_real_ready_proof() {
    let t = Tree::new();
    let row = POP.lines().find(|row| row.starts_with("67\t")).unwrap();
    let mut columns = row.split('\t').collect::<Vec<_>>();
    columns[9] = "READY";
    let ready = POP.replace(row, &columns.join("\t"));
    // Without full K-R source proof even this otherwise legal status is rejected.
    t.put(POP_PATH, &ready);
    // Real parseable legacy shape: fail for absent relocation, not a read error.
    t.put("src/render/silent_voice_visible_measure_coverage_render_writer.rs", concat!(
        "fn materialize_silent_voice_visible_measure_coverage_for_pdf_source() { ",
        "if let Some((edit_start, edit_end, replacement)) = same_lane_strict_interior_edit { ",
        "let mut candidate = String::with_capacity(final_candidate.len() + replacement.len()); ",
        "candidate.push_str(&final_candidate[..edit_start]); candidate.push_str(&replacement); ",
        "candidate.push_str(&final_candidate[edit_end..]); } }"
    ));
    assert!(
        validate_population(&t.0)
            .unwrap_err()
            .contains("#67 not Relocated")
    );
    // Historical parent bytes remain valid under the fixed original baseline.
    t.put(POP_PATH, POP);
    validate_population(&t.0).unwrap();
    t.put(POP_BASE_PATH, &ready);
    assert!(
        validate_tree(&t.0)
            .unwrap_err()
            .contains("compiled K2 document changed")
    );
}

#[test]
fn population_transition_rejects_ready_and_retired_rollbacks() {
    for (id, target) in [("67", "READY"), ("28", "RETIRED_SSC_K2")] {
        let row = POP
            .lines()
            .find(|s| s.starts_with(&format!("{id}\t")))
            .unwrap();
        let mut columns = row.split('\t').collect::<Vec<_>>();
        columns[9] = target;
        let advanced = POP.replace(row, &columns.join("\t"));
        validate_population_no_revert(POP, &advanced).unwrap();
        assert!(
            validate_population_no_revert(&advanced, POP)
                .unwrap_err()
                .contains("population cannot revert")
        );
        // The transition entry point must reject before any runtime source proof.
        let p = Tree::new();
        let c = Tree::new();
        p.put(POP_PATH, &advanced);
        assert!(
            validate_transition(&p.0, &c.0, "KodexOnly")
                .unwrap_err()
                .contains("population cannot revert")
        );
    }
}

#[test]
fn render57_v2_contract_group_has_no_absence_exception() {
    for path in [R57_V2_PATH, R57_BINDING_PATH, R57_CAUSAL_PATH, POP_BASE_PATH] {
        let t = Tree::new();
        std::fs::remove_file(t.0.join(path)).unwrap();
        t.put("ZIP_KIND.txt", "revision=r1251\nname=SscK2GovernanceRevisionKodex\n");
        assert!(validate_tree(&t.0).is_err(), "{path}");
    }
}

#[test]
fn render57_real_installed_writer_is_one_exact_v2_construction() {
    let region = render57_region(R57_V2).unwrap();
    assert_eq!(INSTALLED_WRITER.matches(region).count(), 1);
    assert_eq!(
        item(INSTALLED_WRITER, "fn", "verify_range").unwrap(),
        item(R57_V2, "fn", "verify_range").unwrap()
    );
}

#[test]
fn render57_installed_v2_writer_is_frozen_for_every_class() {
    let p = Tree::new();
    let c = Tree::new();
    for class in ["KodexOnly", "RefactorOnly", "MusicMutation"] {
        validate_render57_writer_transition(&p.0, &c.0).unwrap();
        validate_transition(&p.0, &c.0, class).unwrap_or_else(|error| panic!("{class}: {error}"));
    }
    let region = render57_region(R57_V2).unwrap();
    for forged in [
        format!("{INSTALLED_WRITER}\n"),
        INSTALLED_WRITER.replace(
            "pub fn apply_lower_staff_clef_moves_authorized",
            "fn apply_lower_staff_clef_moves_authorized",
        ),
        INSTALLED_WRITER.replace(
            "verify_clef_move_envelope(source_xml, request.insert_at, request.old_start)?;",
            "",
        ),
        INSTALLED_WRITER.replace(
            "assert_eq!(production.matches(\"xml.insert_str(\").count(), 0);",
            "assert_eq!(production.matches(\"xml.insert_str(\").count(), 1);",
        ),
        format!("{INSTALLED_WRITER}\nfn second_writer() {{}}\n"),
    ] {
        c.put(R57_WRITER_PATH, &forged);
        assert!(
            validate_render57_writer_transition(&p.0, &c.0)
                .unwrap_err()
                .contains("frozen")
        );
    }
    // A parent without the installed V2 construction is older than the epoch base.
    p.put(R57_WRITER_PATH, &INSTALLED_WRITER.replace(region, ""));
    c.put(R57_WRITER_PATH, &INSTALLED_WRITER.replace(region, ""));
    assert!(validate_render57_writer_transition(&p.0, &c.0).unwrap_err().contains("V2"));
}


fn positive_metronome_tree() -> Tree {
    let t = Tree::new();
    t.put("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", include_str!("ssc_test_fixture_r1485_metronome_tail.txt") );
    t.put("src/pipeline_shards/corpus_fastjump_human_potential_core.rs", include_str!("../pipeline_shards/corpus_fastjump_human_potential_core.rs") );
    t.put("src/pipeline_shards/pipeline_orchestrator_project_setup_stages.rs", include_str!("../pipeline_shards/pipeline_orchestrator_project_setup_stages.rs") );
    t.put("src/pipeline_shards/pipeline_runtime_progress_console_core.rs", include_str!("../pipeline_shards/pipeline_runtime_progress_console_core.rs") );
    t.put("src/reference.rs", include_str!("../reference.rs") );
    t.put("src/ssc_evidence_proof.rs", include_str!("ssc_test_fixture_r1279_ssc_evidence_proof.txt") );
    t.put("src/ssc_shadow_measurement.rs", include_str!("ssc_test_fixture_r1279_ssc_shadow_measurement.txt") );
    t.put("src/ssc_shadow_permit.rs", include_str!("ssc_test_fixture_r1279_ssc_shadow_permit.txt") );
    t.put("src/ssc_shadow_report.rs", include_str!("ssc_test_fixture_r1279_ssc_shadow_report.txt") );
    t.put("src/structural_write_authority/thirty_surface_productive_authority.rs", include_str!("../structural_write_authority/thirty_surface_productive_authority.rs") );
    t
}
#[test]
fn positive_metronome_is_reachable_with_estimate_present_and_open_removal() {
    let t = positive_metronome_tree();
    predicate(&t.0, "wav_metronome").unwrap();
    t.put(REMAINING_PATH, &super::super::INITIAL_REMAINING.replace("open=wav_metronome\n", ""));
    validate_tree(&t.0).unwrap();
}
#[test]
fn positive_metronome_rejects_formula_rounding_clamp_and_duplicate_direction_changes() {
    let mutations = [
        ("src/reference.rs", "total_quarters * 60.0 / seconds", "total_quarters * 61.0 / seconds"),
        ("src/reference.rs", "quarters += expected_divisions / divisions", "quarters += expected_divisions * divisions"),
        ("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "bpm.round()", "bpm.floor()"),
        ("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "clamp(40.0, 220.0)", "clamp(41.0, 220.0)"),
        ("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "clamp(40.0, 220.0)", "clamp(40.0, 221.0)"),
        ("src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs", "let (inserted, receipt) = insert_direction_into_measure(xml, measure, &direction);", "let (inserted, receipt) = insert_direction_into_measure(xml, measure, &direction); let duplicate = direction.replace(\"above\", \"below\"); let _ = insert_direction_into_measure(xml, measure, &duplicate);"),
    ];
    for (path, before, after) in mutations {
        let t = positive_metronome_tree();
        let source = read(&t.0, path).unwrap();
        assert!(source.contains(before));
        t.put(path, &source.replacen(before, after, 1));
        assert!(predicate(&t.0, "wav_metronome").is_err(), "{path}: {before}");
    }
}
#[test]
fn positive_metronome_rejects_missing_input_and_additive_owner_drift() {
    let t = positive_metronome_tree();
    std::fs::remove_file(t.0.join("src/reference.rs")).unwrap();
    assert!(predicate(&t.0, "wav_metronome").is_err());
    let t = positive_metronome_tree();
    let path = "src/structural_write_authority/thirty_surface_productive_authority.rs";
    let text = read(&t.0, path).unwrap();
    assert!(text.contains("measure_numbers.sort_unstable();"));
    t.put(path, &text.replace("measure_numbers.sort_unstable();", "measure_numbers.reverse();"));
    assert!(predicate(&t.0, "wav_metronome").is_err());
}
#[test]
fn positive_metronome_rejects_second_source_and_wav_as_proof_input() {
    let t = positive_metronome_tree();
    t.put("src/synthetic_second_tempo.rs", "fn second_tempo() { apply_expression_polish(); }\n");
    assert!(predicate(&t.0, "wav_metronome").unwrap_err().contains("source/caller scope"));
    for path in governed::PROOF_FILES {
        let t = positive_metronome_tree();
        let text = read(&t.0, path).unwrap();
        t.put(path, &(text + "\nfn wav_proof_source() { let estimated_bpm = 120.0; }\n"));
        assert!(predicate(&t.0, "wav_metronome").unwrap_err().contains("proof firewall"));
    }
}
#[test]
fn positive_metronome_rejects_retired_writers_and_unknown_closed_construction() {
    for symbol in ["WavTruthAccent", "run_salience_weighted_candidate_gate", "mint_articulation_fragment_receipt"] {
        let t = positive_metronome_tree();
        t.put("src/synthetic_retired_writer.rs", &format!("fn {symbol}() {{}}\n"));
        assert!(predicate(&t.0, "wav_metronome").is_err());
    }
    let t = Tree::new();
    t.put(REMAINING_PATH, &super::super::INITIAL_REMAINING.replace("open=wav_metronome\n", ""));
    assert!(validate_tree(&t.0).is_err());
}

fn positive_render57_tree() -> Tree {
    let t = Tree::new();
    t.put("src/structural_write_authority/remaining_layout_writer_family.rs", include_str!("../../src/structural_write_authority/remaining_layout_writer_family.rs"));
    t.put("src/pipeline_shards/notation_endpoint_layout_core.rs", include_str!("../../src/pipeline_shards/notation_endpoint_layout_core.rs"));
    t.put("src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs", include_str!("../../src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs"));
    t.put("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs", include_str!("ssc_test_fixture_r1458_initial_render_stage.txt"));
    // Historical #57 transition fixtures stay in their fixed pre-checkpoint58 epoch.
    t.put("src/structural_write_authority/master_adoption/render_adopted_state.rs", include_str!("ssc_test_fixture_r1449_render_adopted_state.txt"));
    t.put("src/structural_write_authority/verified_render_candidate_sink_authority.rs", include_str!("../../src/structural_write_authority/verified_render_candidate_sink_authority.rs"));
    t.put("src/master_state_identity.rs", include_str!("ssc_test_fixture_r1279_master_state_identity.txt"));
    // r1286: the live identity file is judged together with its own installed A7 wiring,
    // so a wired adoption boundary is never mistaken for an observation escape.
    t.put(governed::WIRING_PATH, include_str!("ssc_verdict_wiring_v1.txt"));
    t.put("src/structural_write_authority/row57_prepare_boundary_tests.rs", include_str!("../../src/structural_write_authority/row57_prepare_boundary_tests.rs"));
    t.put("tests/support/render_authority/row57_prepares_file_bound_post_without_writing_before_adoption.rs", include_str!("../../tests/support/render_authority/row57_prepares_file_bound_post_without_writing_before_adoption.rs"));
    t.put("tests/support/render_authority/row57_local_clef_scope_through_trial.rs", include_str!("../../tests/support/render_authority/row57_local_clef_scope_through_trial.rs"));
    let population = read(&t.0, POP_PATH).unwrap();
    let ready = population.lines().map(|line| {
        let mut row = line.split('\t').collect::<Vec<_>>();
        if row[0] == "57" { row[9] = "READY"; }
        row.join("\t")
    }).collect::<Vec<_>>().join("\n") + "\n";
    t.put(POP_PATH, &ready);
    t
}
#[test]
fn render57_positive_requires_ready_and_the_complete_accepted_construction() {
    let t = positive_render57_tree();
    validate_render57_positive(&t.0).unwrap();
    validate_population(&t.0).unwrap();
    let pop = read(&t.0, POP_PATH).unwrap();
    t.put(POP_PATH, &pop.replace("\tREADY\tDEFERRED_RENDER_PREREQUISITE: contains clef", "\tDEFERRED_RENDER_PREREQUISITE\tDEFERRED_RENDER_PREREQUISITE: contains clef"));
    assert!(validate_render57_positive(&t.0).unwrap_err().contains("not READY"));
}
#[test]
fn render57_missing_or_changed_runtime_and_tests_reject_even_with_ready() {
    for (path, _) in R57_POSITIVE_FILES {
        let t = positive_render57_tree();
        let original = read(&t.0, path).unwrap();
        t.put(path, &(original.clone() + "\n// synthetic changed binding\n"));
        assert!(validate_render57_positive(&t.0).is_err(), "{path}");
        std::fs::remove_file(t.0.join(path)).unwrap();
        assert!(validate_render57_positive(&t.0).is_err(), "{path}");
    }
}
#[test]
fn render57_changed_safety_planner_and_owner_error_transport_reject() {
    for (path, before, after) in [
        ("src/pipeline_shards/notation_endpoint_layout_core.rs", "if has_low_or_rest_before && !high_before", "if has_low_or_rest_before"),
        ("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs", "let failed = trial_result.is_err();", "let failed = false;"),
        ("src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs", "mint_file_bound_block_receipt", "synthetic_unbound_receipt"),
    ] {
        let t = positive_render57_tree();
        let source = read(&t.0, path).unwrap();
        assert!(source.contains(before));
        t.put(path, &source.replacen(before, after, 1));
        assert!(validate_render57_positive(&t.0).is_err());
    }
}
#[test]
fn render57_positive_population_cannot_revert_or_change_identity() {
    let t = positive_render57_tree();
    let ready = read(&t.0, POP_PATH).unwrap();
    assert!(validate_population_no_revert(&ready, POP).is_err());
    t.put(POP_PATH, &ready.replacen("BLOCK_DOCUMENT", "EXACT", 1));
    assert!(validate_population(&t.0).is_err());
}

#[test]
fn proof_bundle_rejects_any_other_proof_or_identity_bytes() {
    // Contract v4 B4.1: proof drift is a parent-relative transition question (proof_rule),
    // never an unreviewed acceptance; the content firewall holds on every tree.
    let p = positive_metronome_tree();
    let c = positive_metronome_tree();
    predicate(&c.0, "wav_metronome").unwrap();
    let text = read(&c.0, "src/ssc_evidence_proof.rs").unwrap();
    c.put("src/ssc_evidence_proof.rs", &(text + "\n// unreviewed drift\n"));
    predicate(&c.0, "wav_metronome").unwrap();
    for class in ["RefactorOnly", "KodexOnly", "MusicMutation"] {
        assert!(governed::validate_transition(&p.0, &c.0, class, false).unwrap_err().contains("proof_rule"));
    }
    // #57: drift outside the bound construction items is not #57 authority; drift inside is.
    let t = positive_render57_tree();
    validate_render57_positive(&t.0).unwrap();
    let text = read(&t.0, "src/master_state_identity.rs").unwrap();
    t.put("src/master_state_identity.rs", &(text.clone() + "\n// unrelated drift\n"));
    validate_render57_positive(&t.0).unwrap();
    for (_, kind, name, _) in R57_POSITIVE_ITEMS {
        let t = positive_render57_tree();
        let bound = item(&text, kind, name).unwrap();
        let changed = bound.replacen('{', "{ /* drift */", 1);
        t.put("src/master_state_identity.rs", &text.replacen(bound, &changed, 1));
        assert!(validate_render57_positive(&t.0).unwrap_err().contains(name), "{name}");
    }
}
#[test]
fn installed_proof_and_identity_bindings_admit_exactly_one_value_each() {
    // No whole-file proof or identity hash remains; #57 binds three exact items, among
    // them the complete BlockTransformationReceipt impl (r1278 F6).
    assert!(R57_POSITIVE_FILES.iter().all(|(path, _)| *path != "src/master_state_identity.rs"));
    assert_eq!(R57_POSITIVE_ITEMS.len(), 3);
    let names = R57_POSITIVE_ITEMS.iter().map(|i| (i.1, i.2)).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&("impl", "BlockTransformationReceipt")));
    let real = include_str!("../master_state_identity.rs");
    for (_, kind, name, expected) in R57_POSITIVE_ITEMS {
        assert_eq!(crate::rust_source_scan_primitives::fnv1a64(item(real, kind, name).unwrap().as_bytes()), *expected);
    }
}
#[test]
fn installed_surface_is_complete_v3_and_rejects_v2_hybrid_or_widened_variants() {
    assert_eq!(super::surface_mismatch(SURFACE, SURFACE).unwrap(), None);
    assert!(super::surface_mismatch(super::SURFACE_V2, SURFACE).unwrap().is_some(), "historical V2 is no longer accepted");
    let hybrid = SURFACE.replacen("    BrokenBeamChain {\n        note: usize,\n    },\n", "", 1);
    assert_ne!(hybrid, SURFACE);
    assert!(super::surface_mismatch(&hybrid, SURFACE).unwrap().is_some());
    let widened = SURFACE.replacen("    Preserving,\n", "    Preserving,\n    Exempt,\n", 1);
    assert!(super::surface_mismatch(&widened, SURFACE).unwrap().is_some());
}
#[test]
fn historical_parent_documents_are_never_absent_or_changed() {
    // PATH is the K2 installation marker; its loss is rejected by validate_transition.
    for (path, _) in DOCUMENTS.iter().filter(|(path, _)| *path != PATH) {
        let t = Tree::new();
        validate_tree(&t.0).unwrap();
        std::fs::remove_file(t.0.join(path)).unwrap();
        assert!(validate_tree(&t.0).is_err(), "{path}");
    }
    let child = Tree::new();
    std::fs::remove_file(child.0.join(PATH)).unwrap();
    assert!(validate_transition(&Tree::new().0, &child.0, "KodexOnly").unwrap_err().contains("cannot disappear"));
}

#[test]
fn proof_firewall_rejects_inputs_globals_and_dependencies_beyond_the_token_ban() {
    validate_runtime(&positive_metronome_tree().0).unwrap();
    let injections = [
        "use std::fs;\n",
        "fn x() -> String { std::env::var(\"X\").unwrap_or_default() }\n",
        "static NEW_PROOF_STATE: u8 = 0;\n",
        "static mut COUNTER: u8 = 0;\n",
        "const C: &str = include_str!(\"../Cargo.toml\");\n",
        "const C: &[u8] = include_bytes!(\"build_gate/ssc_exit_remaining_v1.txt\");\n",
        "fn x() { let _ = crate::reference::score_quarter_signature; }\n",
        "fn x() { let _ = crate::pipeline::run; }\n",
        "use crate::{final_report};\n",
        "fn x() { let _ = crate::final_report::append_final_report; }\n",
        "fn x() { let _ = crate::master_state_identity::master_sovereignty_violation; }\n",
        "fn x() { let _ = super::reference::y; }\n",
        "fn x() { let _ = anyhow::Error::msg(\"x\"); }\n",
        "fn x() { let _ = zip::ZipArchive::<u8>::new; }\n",
        "fn x() { let _ = std::time::Instant::now(); }\n",
        "fn x() { let _ = std::io::stdin(); }\n",
        "fn x() { let estimated_bpm = 120.0; let _ = estimated_bpm; }\n",
    ];
    for path in governed::PROOF_FILES {
        for injection in injections {
            let t = positive_metronome_tree();
            let text = read(&t.0, path).unwrap();
            t.put(path, &(text + "\n" + injection));
            assert!(
                validate_runtime(&t.0).unwrap_err().contains("proof firewall"),
                "{path}: {injection}"
            );
        }
    }
}
#[test]
fn proof_firewall_checks_allowlisted_dependency_modules_transitively() {
    for (path, injection) in [
        ("src/final_report.rs", "fn reach() { let _ = crate::reference::build_reference_signature; }\n"),
        ("src/master_state_identity.rs", "fn reach() { let estimated_bpm = 1.0; let _ = estimated_bpm; }\n"),
        ("src/master_state_identity.rs", "fn reach() { let _ = crate::salience_gate::x; }\n"),
        ("src/master_state_identity/render_lineage.rs", "fn reach() { let _ = crate::wav_truth::x; }\n"),
    ] {
        let t = positive_metronome_tree();
        let text = read(&t.0, path).unwrap_or_default();
        t.put(path, &(text + injection));
        assert!(validate_runtime(&t.0).unwrap_err().contains("proof firewall"), "{path}");
    }
}
#[test]
fn observer_accepts_only_sealed_ssc_inputs() {
    let t = positive_metronome_tree();
    let text = read(&t.0, PROOF_PATH).unwrap();
    let anchor = "    necessity: SscNecessityAssessment<'event>,\n) -> Result<";
    assert_eq!(text.matches(anchor).count(), 1);
    let head = "    necessity: SscNecessityAssessment<'event>,\n";
    for injected in ["    hint: f64,\n", "    xml: &str,\n", "    signature: crate::master_state_identity::MintedIdentity,\n"] {
        t.put(PROOF_PATH, &text.replacen(anchor, &format!("{head}{injected}) -> Result<"), 1));
        let result = validate_runtime(&t.0);
        assert!(result.as_ref().is_err_and(|e| e.contains("proof firewall")), "{injected}: {result:?}");
    }
}
#[test]
fn surface_extensions_are_exact_appends_of_installed_v3_items() {
    let ext = vec![("SscProofFamily".to_string(), "Containment".to_string())];
    let proof = SURFACE.replacen("    Preserving,\n}", "    Preserving,\n    Containment,\n}", 1);
    assert_ne!(proof, SURFACE);
    assert_eq!(surface_mismatch_extended(&proof, SURFACE, &ext).unwrap(), None);
    assert!(surface_mismatch_extended(&proof, SURFACE, &[]).unwrap().is_some(), "undeclared variant");
    assert!(surface_mismatch_extended(SURFACE, SURFACE, &ext).unwrap().is_some(), "declared but absent");
    let middle = SURFACE.replacen("    Additive,\n    Preserving,\n}", "    Additive,\n    Containment,\n    Preserving,\n}", 1);
    assert_ne!(middle, SURFACE);
    assert!(surface_mismatch_extended(&middle, SURFACE, &ext).unwrap().is_some(), "not an append");
    let unknown = vec![("SscInventedType".to_string(), "X".to_string())];
    assert!(surface_mismatch_extended(SURFACE, SURFACE, &unknown).unwrap().is_some());
    // Installed through the governed file, the extended proof is the valid surface.
    let t = positive_metronome_tree();
    let real = read(&t.0, PROOF_PATH).unwrap();
    let extended = real.replacen("    Preserving,\n}", "    Preserving,\n    Containment,\n}", 1);
    assert_ne!(extended, real);
    t.put(PROOF_PATH, &extended);
    assert!(validate_runtime(&t.0).unwrap_err().contains("surface mismatch"));
    t.put(governed::EXTENSIONS_PATH, &format!("{}variant=SscProofFamily|Containment\n", FIXTURE_EXTENSIONS));
    validate_runtime(&t.0).unwrap();
}
#[test]
fn verdict_tokens_escape_observation_only_through_declared_wiring() {
    let t = positive_metronome_tree();
    let path = "src/master_state_identity.rs";
    let text = read(&t.0, path).unwrap();
    let anchor = "fn commit(self, domain: crate::ssc_evidence_proof::SscAdoptionDomain) -> CommittedTransformationEvent {";
    assert_eq!(text.matches(anchor).count(), 1);
    let gated = text.replacen(anchor, &format!("{anchor}\n        let _gate = crate::ssc_evidence_proof::ssc_adoption_verdict;"), 1);
    t.put(path, &gated);
    assert!(validate_runtime(&t.0).unwrap_err().contains("escapes observation"));
    t.put(governed::WIRING_PATH, &format!("{}wiring={path}::commit\n", FIXTURE_WIRING));
    validate_runtime(&t.0).unwrap();
    // r1278 F4 (Codex counterexample): the same verdict in another function of the wired
    // file is still an escape.
    for token in ["ssc_adoption_verdict", "SscPermitVerdictV2", "SscContainmentVerdict"] {
        t.put(path, &format!("{gated}\nfn some_mint_or_helper() {{ let _ = crate::ssc_evidence_proof::{token}; }}\n"));
        assert!(validate_runtime(&t.0).unwrap_err().contains("escapes observation"), "{token}");
    }
    t.put(path, &gated);
    let other = "src/structural_write_authority/synthetic_boundary.rs";
    t.put(other, "fn commit() { let _ = crate::ssc_evidence_proof::ssc_adoption_verdict; }\n");
    assert!(validate_runtime(&t.0).unwrap_err().contains("escapes observation"));
}
#[test]
fn governed_transition_runs_after_the_parent_k2_rules_for_every_class() {
    let p = Tree::new();
    let c = Tree::new();
    let text = read(&c.0, PROOF_PATH).unwrap();
    c.put(PROOF_PATH, &(text + "\n// drift\n"));
    for class in ["RefactorOnly", "MusicMutation", "KodexOnly"] {
        assert!(validate_transition(&p.0, &c.0, class).unwrap_err().contains("proof_rule"), "{class}");
    }
}
#[test]
fn proof_firewall_reads_the_transaction_backup_layout_of_an_accepted_parent() {
    let t = positive_metronome_tree();
    validate_runtime(&t.0).unwrap();
    let manifest = read(&t.0, "Cargo.toml").unwrap();
    std::fs::remove_file(t.0.join("Cargo.toml")).unwrap();
    assert!(validate_runtime(&t.0).is_err());
    t.put("root_files/Cargo.toml", &manifest);
    validate_runtime(&t.0).unwrap();
    assert!(cargo_dependencies(&t.0).unwrap().iter().any(|d| d == "roxmltree"));
}

#[test]
fn proof_firewall_refuses_aliases_reexports_foreign_macros_and_hidden_modules() {
    // r1278 F1 (Codex counterexamples and their neighbours).
    let injections = [
        "use std as ambient;\nfn x() -> bool { ambient::env::var(\"SSC_EXTERNAL_INPUT\").is_ok() }\n",
        "use std::env as ambient;\n",
        "use core as ambient;\n",
        "use std::sync::Arc as Shared;\n",
        "pub(crate) use std::sync::Arc;\n",
        "pub use crate::master_state_identity::MintedIdentity;\n",
        "fn x() -> bool { crate::master_state_identity::ssc_external::var(\"X\").is_ok() }\n",
        "fn x() { let _ = crate::master_state_identity::tests::read_sources; }\n",
        "macro_rules! sneaky { () => {} }\n",
        "fn x() { let _ = foreign_input!(); }\n",
        "extern \"C\" { fn getenv(name: *const u8) -> *const u8; }\n",
        "mod hidden;\n",
        "fn x() { let _ = std::net::TcpStream::connect; }\n",
        "fn x() { let _ = std::path::Path::new(\"x\"); }\n",
        "fn x() { let _ = std::process::id(); }\n",
    ];
    for path in governed::PROOF_FILES {
        for injection in injections {
            let t = positive_metronome_tree();
            let text = read(&t.0, path).unwrap();
            t.put(path, &(text + "\n" + injection));
            let result = validate_runtime(&t.0);
            assert!(result.as_ref().is_err_and(|e| e.contains("proof firewall")), "{path}: {injection}: {result:?}");
        }
    }
}
#[test]
fn identity_dependency_cannot_reexport_or_read_an_ambient_input() {
    for injection in [
        "pub(crate) use std::env as ssc_external;\n",
        "pub(crate) use std::fs;\n",
        "pub(crate) fn ssc_block_descriptor() -> bool { std::env::var(\"X\").is_ok() }\n",
        "use std::sync::Mutex as Guard;\n",
    ] {
        let t = positive_metronome_tree();
        let text = read(&t.0, "src/master_state_identity.rs").unwrap();
        t.put("src/master_state_identity.rs", &(text + injection));
        let result = validate_runtime(&t.0);
        assert!(result.as_ref().is_err_and(|e| e.contains("proof firewall")), "{injection}: {result:?}");
    }
    // A reserved accessor name used by the proof must exist as a plain identity fn.
    let t = positive_metronome_tree();
    let proof = read(&t.0, PROOF_PATH).unwrap();
    t.put(PROOF_PATH, &(proof + "\nfn x() { let _ = crate::master_state_identity::ssc_block_descriptor; }\n"));
    assert!(validate_runtime(&t.0).unwrap_err().contains("not a plain fn"));
}
#[test]
fn observer_signature_is_exactly_v2_or_v4_containment() {
    // r1278 F2 (Codex counterexample): an Ssc-named struct with a public bool is no seal.
    let t = positive_metronome_tree();
    let text = read(&t.0, PROOF_PATH).unwrap();
    let anchor = "    necessity: SscNecessityAssessment<'event>,\n) -> Result<";
    let head = "    necessity: SscNecessityAssessment<'event>,\n";
    for injected in [
        "    ambient: SscAmbientInput,\n",
        "    containment: SscContainmentAssessment<'event>,\n    ambient: SscAmbientInput,\n",
    ] {
        t.put(PROOF_PATH, &format!("{}\npub(crate) struct SscAmbientInput {{\n    pub(crate) accept: bool,\n}}\n", text.replacen(anchor, &format!("{head}{injected}) -> Result<"), 1)));
        assert!(validate_runtime(&t.0).unwrap_err().contains("proof firewall"), "{injected}");
    }
    let generic = text.replacen("fn observe_ssc_event_v2<'event>(", "fn observe_ssc_event_v2<'event, SscInput>(", 1);
    t.put(PROOF_PATH, &generic);
    assert!(validate_runtime(&t.0).unwrap_err().contains("proof firewall"));
    // The V4 head alone, without the frozen V4 surface, is rejected.
    t.put(PROOF_PATH, &text.replacen(anchor, &format!("{head}    containment: SscContainmentAssessment<'event>,\n) -> Result<"), 1));
    assert!(validate_runtime(&t.0).unwrap_err().contains("V4"));
}
fn v4_proof(v3: &str) -> String {
    let old = item(v3, "struct", "SscClassifiedObservationV2").unwrap();
    let new = item(SURFACE_V4, "struct", "SscClassifiedObservationV2").unwrap();
    let start = SURFACE_V4.find("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub(crate) enum SscEnvelopeKind").unwrap();
    let anchor = "    necessity: SscNecessityAssessment<'event>,\n) -> Result<";
    let observer = "    necessity: SscNecessityAssessment<'event>,\n    containment: SscContainmentAssessment<'event>,\n) -> Result<";
    format!("{}\n{}", v3.replacen(old, new, 1).replacen(anchor, observer, 1), &SURFACE_V4[start..])
}
#[test]
fn frozen_v4_containment_surface_is_admitted_exactly_and_never_reverts() {
    let t = positive_metronome_tree();
    let v3 = read(&t.0, PROOF_PATH).unwrap();
    let v4 = v4_proof(&v3);
    t.put(PROOF_PATH, &v4);
    validate_runtime(&t.0).unwrap();
    // Any drift of a V4 item is a mismatch; extensions stay bare unit variants.
    for (before, after) in [
        ("    ContainmentDenied,\n", "    ContainmentDenied,\n    Exempt,\n"),
        ("    checked_bytes: usize,\n", "    checked_bytes: usize,\n    accept: bool,\n"),
        ("    descriptor: &'static str,\n", "    descriptor: &'static str,\n    pub(crate) ambient: bool,\n"),
    ] {
        assert!(v4.contains(before));
        t.put(PROOF_PATH, &v4.replacen(before, after, 1));
        assert!(validate_runtime(&t.0).unwrap_err().contains("surface mismatch"), "{after}");
    }
    // V4 -> V3 is a regression.
    let p = Tree::new();
    let c = Tree::new();
    let parent_v4 = v4_proof(&read(&p.0, PROOF_PATH).unwrap());
    p.put(PROOF_PATH, &parent_v4);
    validate_tree(&p.0).unwrap();
    assert!(validate_transition(&p.0, &c.0, "KodexOnly").unwrap_err().contains("cannot return to V3"));
}
#[test]
fn render57_impl_pin_detects_helper_drift_behind_an_unchanged_wrapper() {
    // r1278 F6 (Codex counterexample): dropping the post fingerprint filter in the split
    // helper leaves the wrapper unchanged but breaks the bound impl.
    let t = positive_render57_tree();
    validate_render57_positive(&t.0).unwrap();
    let text = read(&t.0, "src/master_state_identity.rs").unwrap();
    let block = item(&text, "impl", "BlockTransformationReceipt").unwrap();
    let filter = ".filter(|document| fingerprint(document) == self.post_fingerprint)";
    assert!(block.contains(filter));
    let changed = block.replacen(filter, "", 1);
    t.put("src/master_state_identity.rs", &text.replacen(block, &changed, 1));
    assert!(validate_render57_positive(&t.0).unwrap_err().contains("BlockTransformationReceipt"));
    for helper in ["fn mint_block_receipt", "fn validate_block_transition", "fn split_realized_post_document"] {
        assert!(block.contains(helper), "{helper}");
    }
}

// r1288 contract v4 closure: synthetic sources only, never live runtime files.
fn closure_tree() -> Tree {
    let t = Tree::new();
    t.put(EXIT_CLOSURE_PATH, EXIT_CLOSURE);
    t.put(REMAINING_PATH, "schema=ssc_exit_remaining_v1\nexit_claim=not_closed\n");
    t.put(governed::WIRING_PATH, &format!("{FIXTURE_WIRING}wiring=src/master_state_identity.rs::commit_owner_receipts\nwiring=src/master_state_identity/render_lineage.rs::commit\n"));
    t.put("src/master_state_identity.rs", "fn commit_owner_receipts() { if let Err(r) = crate::ssc_evidence_proof::ssc_adoption_verdict(a, b, c) { stop(r) } }\n");
    t.put("src/master_state_identity/render_lineage.rs", "fn commit() { if let Err(r) = crate::ssc_evidence_proof::ssc_adoption_verdict(a, b, c) { stop(r) } }\n");
    t.put("src/ssc_shadow_report.rs", "fn header() -> &'static str { \"PermitEnforcement: active\\n\" }\n");
    t.put(PROOF_PATH, "fn ssc_adoption_verdict() { classify_bound_origin(a); adoption_decision(&c) }\nfn adoption_decision(c: &C) -> R { if permitted == 1 { ok() } else { err() } }\n");
    t
}
#[test]
fn exit_closure_holds_only_with_empty_remaining_set_and_enforcing_a7_boundaries() {
    let t = closure_tree();
    validate_exit_closure(&t.0).unwrap();
    let absent = Tree::new();
    validate_exit_closure(&absent.0).unwrap();
    let cases: [(&str, String); 7] = [
        (EXIT_CLOSURE_PATH, EXIT_CLOSURE.replace("withdrawal=forbidden", "withdrawal=allowed")),
        (REMAINING_PATH, "schema=ssc_exit_remaining_v1\nexit_claim=not_closed\nopen=wav_metronome\n".to_owned()),
        (governed::WIRING_PATH, format!("{FIXTURE_WIRING}wiring=src/master_state_identity.rs::commit_owner_receipts\n")),
        ("src/master_state_identity/render_lineage.rs", "fn commit() { observe_only(a, b, c) }\n".to_owned()),
        ("src/ssc_shadow_report.rs", "fn header() -> &'static str { \"PermitEnforcement: none\\n\" }\n".to_owned()),
        (PROOF_PATH, "fn ssc_adoption_verdict() { classify_bound_origin(a); adoption_decision(&c) }\nfn adoption_decision(c: &C) -> R { ok() }\n".to_owned()),
        (PROOF_PATH, "fn ssc_adoption_verdict() { adoption_decision(&c) }\nfn adoption_decision(c: &C) -> R { if permitted == 1 { ok() } else { err() } }\n".to_owned()),
    ];
    for (path, text) in cases {
        let t = closure_tree();
        t.put(path, &text);
        assert!(validate_exit_closure(&t.0).unwrap_err().contains("SSC exit closure"), "{path}");
    }
}
#[test]
fn exit_closure_can_never_be_withdrawn() {
    let parent = closure_tree();
    let child = closure_tree();
    validate_exit_closure_transition(&parent.0, &child.0).unwrap();
    std::fs::remove_file(child.0.join(EXIT_CLOSURE_PATH)).unwrap();
    assert!(validate_exit_closure_transition(&parent.0, &child.0).unwrap_err().contains("cannot be withdrawn"));
    let open = Tree::new();
    validate_exit_closure_transition(&open.0, &child.0).unwrap();
}

// A test-only fixture for both admitted runtime epochs. The checker never
// reconstructs its parent; only this synthetic tree setup reverses fixed edits.
fn render57_noop_stage_before_fixture() -> String {
    let live = include_str!("ssc_cursor_temporal_initial_parent_r1444.txt");
    let mut normalized = live.to_string();
    if crate::rust_source_scan_primitives::fnv1a64(normalized.as_bytes()) == R57_CURSOR_COURSE_STAGE_AFTER {
        assert_eq!(normalized.matches(R57_CURSOR_COURSE_STAGE_AFTER_TEXT).count(), 1);
        normalized = normalized.replacen(R57_CURSOR_COURSE_STAGE_AFTER_TEXT,
            R57_CURSOR_COURSE_STAGE_BEFORE_TEXT, 1);
    }
    if crate::rust_source_scan_primitives::fnv1a64(normalized.as_bytes()) == R57_MATERIAL_STAGE_AFTER {
        for &(old, new) in R57_MATERIAL_STAGE_EDITS.iter().rev() {
            assert_eq!(normalized.matches(new).count(), 1);
            normalized = normalized.replacen(new, old, 1);
        }
    }
    let live = normalized.as_str();
    let hash = crate::rust_source_scan_primitives::fnv1a64(live.as_bytes());
    if hash == R57_NOOP_STAGE_BEFORE {
        return live.to_string();
    }
    assert_eq!(hash, R57_NOOP_STAGE_AFTER, "unknown live stage fixture");
    let mut before = live.to_string();
    for &(old, new) in R57_NOOP_STAGE_EDITS.iter().rev() {
        assert_eq!(before.matches(new).count(), 1);
        before = before.replacen(new, old, 1);
    }
    assert_eq!(crate::rust_source_scan_primitives::fnv1a64(before.as_bytes()), R57_NOOP_STAGE_BEFORE);
    before
}

#[test]
fn render57_noop_stage_admits_only_the_complete_one_way_music_transition() {
    let before = render57_noop_stage_before_fixture();
    let after = render57_noop_stage_successor(&before).unwrap();
    let p = positive_render57_tree();
    let c = positive_render57_tree();
    p.put(R57_NOOP_STAGE_PATH, &before);
    c.put(R57_NOOP_STAGE_PATH, &after);
    validate_render57_positive(&p.0).unwrap();
    validate_render57_positive(&c.0).unwrap();
    validate_render57_noop_stage_transition(&p.0, &c.0, "MusicMutation").unwrap();
    // Exercise the actual complete transition owner, not just the local predicate.
    validate_transition(&p.0, &c.0, "MusicMutation").unwrap();
    for class in ["KodexOnly", "RefactorOnly", "AuditOnly", "GovernedChange", "MechanicalOnly", "ValidationOnly", "OutcomeProjection", "CalibrationOnly"] {
        assert!(validate_render57_noop_stage_transition(&p.0, &c.0, class).is_err(), "{class}");
    }
    for class in ["MusicMutation", "KodexOnly", "GovernedChange"] {
        assert!(validate_render57_noop_stage_transition(&c.0, &p.0, class).unwrap_err().contains("cannot revert"));
        validate_render57_noop_stage_transition(&p.0, &p.0, class).unwrap();
        validate_render57_noop_stage_transition(&c.0, &c.0, class).unwrap();
    }
}

#[test]
fn render57_noop_stage_rejects_every_partial_edit_and_extra_byte() {
    let before = render57_noop_stage_before_fixture();
    let after = render57_noop_stage_successor(&before).unwrap();
    let p = positive_render57_tree();
    p.put(R57_NOOP_STAGE_PATH, &before);
    for mask in 0..(1usize << R57_NOOP_STAGE_EDITS.len()) {
        let mut partial = before.clone();
        for (index, &(old, new)) in R57_NOOP_STAGE_EDITS.iter().enumerate() {
            if mask & (1 << index) != 0 { partial = partial.replacen(old, new, 1); }
        }
        let c = positive_render57_tree();
        c.put(R57_NOOP_STAGE_PATH, &partial);
        let result = validate_render57_noop_stage_transition(&p.0, &c.0, "MusicMutation");
        if mask == 0 || mask + 1 == (1usize << R57_NOOP_STAGE_EDITS.len()) {
            result.unwrap();
        } else {
            assert!(result.is_err(), "partial mask {mask}");
            assert!(validate_render57_positive(&c.0).is_err());
        }
    }
    for source in [before, after] {
        let c = positive_render57_tree();
        c.put(R57_NOOP_STAGE_PATH, &(source + "\n// extra synthetic byte\n"));
        assert!(validate_render57_positive(&c.0).is_err());
        assert!(validate_render57_noop_stage_transition(&p.0, &c.0, "MusicMutation").is_err());
    }
}

#[test]
fn render57_noop_stage_keeps_trial_receipt_sink_and_error_mutations_red_in_both_epochs() {
    let before = render57_noop_stage_before_fixture();
    let after = render57_noop_stage_successor(&before).unwrap();
    for source in [before, after] {
        for (old, new) in [
            ("let failed = trial_result.is_err();", "let failed = false;"),
            ("receipts.push_block(Some(receipt), &post);", "// synthetic omitted receipt"),
            ("adopt_path_bound_transition(", "synthetic_unbound_adoption("),
            ("write_visual_cleanup_candidate_authorized(", "synthetic_other_sink("),
        ] {
            assert!(source.contains(old));
            let c = positive_render57_tree();
            c.put(R57_NOOP_STAGE_PATH, &source.replacen(old, new, 1));
            assert!(validate_render57_positive(&c.0).is_err(), "{old}");
        }
    }
}

#[test]
fn render57_noop_stage_successor_cannot_be_derived_from_an_unknown_parent() {
    let before = render57_noop_stage_before_fixture();
    let after = render57_noop_stage_successor(&before).unwrap();
    assert!(render57_noop_stage_successor(&after).is_err());
    assert!(render57_noop_stage_successor(&(before + "\n")).is_err());
}

#[test]
fn render57_noop_stage_rejects_the_unconditional_no_render_boundary_successor() {
    let before = render57_noop_stage_before_fixture();
    let after = render57_noop_stage_successor(&before).unwrap();
    let p = positive_render57_tree();
    p.put(R57_NOOP_STAGE_PATH, &before);
    // This was the earlier uninstalled successor: it incorrectly required a
    // P2 receipt even when the producer was skipped without a Render source.
    let unconditional = after
        .replace("    if transaction.final_render_source_available\n        && !transaction\n", "    if !transaction\n")
        .replace("    if transaction.final_render_source_available {\n        // Continue a participating P2 boundary even without a new P2 write.\n", "    {\n        // Split identity exists for both the unpruned and selected-prune source.\n        // Its continuation is independent of whether P2 inserted a new copy.\n");
    assert_eq!(crate::rust_source_scan_primitives::fnv1a64(unconditional.as_bytes()), 0x9e277f212f6585e9);
    let c = positive_render57_tree();
    c.put(R57_NOOP_STAGE_PATH, &unconditional);
    assert!(validate_render57_positive(&c.0).is_err());
    assert!(validate_render57_noop_stage_transition(&p.0, &c.0, "MusicMutation").is_err());
    assert!(validate_transition(&p.0, &c.0, "MusicMutation").is_err());
}

#[test]
fn render57_material_identity_admission_is_exact_one_way_and_independent() {
    let before = render57_noop_stage_successor(&render57_noop_stage_before_fixture()).unwrap();
    let after = render57_material_stage_successor(&before).unwrap();
    let p = positive_render57_tree(); let c = positive_render57_tree();
    p.put(R57_NOOP_STAGE_PATH, &before); c.put(R57_NOOP_STAGE_PATH, &after);
    validate_render57_positive(&p.0).unwrap(); validate_render57_positive(&c.0).unwrap();
    validate_transition(&p.0, &c.0, "MusicMutation").unwrap();
    for class in ["KodexOnly", "RefactorOnly", "AuditOnly", "GovernedChange", "MechanicalOnly", "ValidationOnly", "OutcomeProjection", "CalibrationOnly"] {
        assert!(validate_render57_noop_stage_transition(&p.0, &c.0, class).is_err(), "{class}");
    }
    assert!(validate_render57_noop_stage_transition(&c.0, &p.0, "MusicMutation").is_err());
    validate_render57_noop_stage_transition(&c.0, &c.0, "MusicMutation").unwrap();
}
#[test]
fn render57_material_identity_admission_rejects_partial_delta_and_protected_drift() {
    let before = render57_noop_stage_successor(&render57_noop_stage_before_fixture()).unwrap();
    let after = render57_material_stage_successor(&before).unwrap();
    let p = positive_render57_tree(); p.put(R57_NOOP_STAGE_PATH, &before);
    for mask in 1..(1usize << R57_MATERIAL_STAGE_EDITS.len())-1 {
        let mut partial = before.clone();
        for (index, &(old, new)) in R57_MATERIAL_STAGE_EDITS.iter().enumerate() {
            if mask & (1 << index) != 0 { partial = partial.replacen(old, new, 1); }
        }
        let c = positive_render57_tree(); c.put(R57_NOOP_STAGE_PATH, &partial);
        assert!(validate_render57_positive(&c.0).is_err());
        assert!(validate_render57_noop_stage_transition(&p.0, &c.0, "MusicMutation").is_err());
    }
    for (old, new) in [
        ("let failed = trial_result.is_err();", "let failed = false;"),
        ("receipts.push_block(Some(receipt), &post);", "// omitted receipt"),
        ("adopt_path_bound_transition(", "unbound_adoption("),
        ("write_visual_cleanup_candidate_authorized(", "unbound_sink("),
        ("if transaction.final_render_source_available {", "if false {"),
    ] {
        assert!(after.contains(old));
        let c = positive_render57_tree(); c.put(R57_NOOP_STAGE_PATH, &after.replacen(old,new,1));
        assert!(validate_render57_positive(&c.0).is_err());
        assert!(validate_render57_noop_stage_transition(&p.0,&c.0,"MusicMutation").is_err());
    }
    let c = positive_render57_tree(); c.put(R57_NOOP_STAGE_PATH, &(after+"\n// extra byte\n"));
    assert!(validate_render57_positive(&c.0).is_err());
}

#[test]
// Historical test identity retained; the previously unactivated positive route is now closed.
fn original_cursor_evidence_borrow_admits_only_complete_one_way_refactor() {
    let material = render57_material_stage_successor(
        &render57_noop_stage_successor(&render57_noop_stage_before_fixture()).unwrap()).unwrap();
    let after = render57_cursor_course_stage_successor(&material).unwrap();
    let parent = positive_render57_tree(); let child = positive_render57_tree();
    parent.put(R57_NOOP_STAGE_PATH, &material); child.put(R57_NOOP_STAGE_PATH, &after);
    validate_render57_positive(&parent.0).unwrap(); assert!(validate_render57_positive(&child.0).is_err());
    assert!(validate_transition(&parent.0, &child.0, "RefactorOnly").is_err());
    for class in ["KodexOnly", "MusicMutation", "AuditOnly", "GovernedChange", "MechanicalOnly", "ValidationOnly", "OutcomeProjection", "CalibrationOnly"] {
        assert!(validate_render57_noop_stage_transition(&parent.0, &child.0, class).is_err(), "{class}");
    }
    assert!(validate_render57_noop_stage_transition(&child.0, &parent.0, "RefactorOnly").is_err());
    assert!(validate_render57_noop_stage_transition(&child.0, &child.0, "RefactorOnly").is_err());
    let earlier = positive_render57_tree();
    earlier.put(R57_NOOP_STAGE_PATH, &render57_noop_stage_before_fixture());
    assert!(validate_render57_noop_stage_transition(&earlier.0, &child.0, "RefactorOnly").is_err());
}

#[test]
fn original_cursor_evidence_borrow_refuses_missing_foreign_mutable_or_extra_code() {
    let material = render57_material_stage_successor(
        &render57_noop_stage_successor(&render57_noop_stage_before_fixture()).unwrap()).unwrap();
    let after = render57_cursor_course_stage_successor(&material).unwrap();
    let parent = positive_render57_tree(); parent.put(R57_NOOP_STAGE_PATH, &material);
    for (old,new) in [
        ("transaction.master_export_block_receipt.as_ref(),", "None,"),
        ("&transaction.cursor_lane_end_invariant_repair_audit,", "&Default::default(),"),
        ("transaction.master_export_block_receipt.as_ref(),", "transaction.master_export_block_receipt.as_mut(),"),
        ("&transaction.cursor_lane_end_invariant_repair_audit,", "&mut transaction.cursor_lane_end_invariant_repair_audit,"),
    ] {
        assert_eq!(after.matches(old).count(),1);
        let child = positive_render57_tree(); child.put(R57_NOOP_STAGE_PATH, &after.replacen(old,new,1));
        assert!(validate_render57_positive(&child.0).is_err());
        assert!(validate_transition(&parent.0,&child.0,"RefactorOnly").is_err());
    }
    let child = positive_render57_tree(); child.put(R57_NOOP_STAGE_PATH, &(after+"\n// extra byte\n"));
    assert!(validate_render57_positive(&child.0).is_err());
    assert!(validate_transition(&parent.0,&child.0,"RefactorOnly").is_err());
}

#[test]
fn original_cursor_evidence_borrow_preserves_every_other_construction_pin() {
    let material = render57_material_stage_successor(
        &render57_noop_stage_successor(&render57_noop_stage_before_fixture()).unwrap()).unwrap();
    let after = render57_cursor_course_stage_successor(&material).unwrap();
    for (path, _) in R57_POSITIVE_FILES.iter().filter(|(p,_)| *p != R57_NOOP_STAGE_PATH) {
        let tree = positive_render57_tree(); tree.put(R57_NOOP_STAGE_PATH, &after);
        let original = std::fs::read_to_string(tree.0.join(path)).unwrap();
        tree.put(path, &(original+"\n// forbidden pin drift\n"));
        assert!(validate_render57_positive(&tree.0).is_err(),"{path}");
    }
    let tree = positive_render57_tree(); tree.put(R57_NOOP_STAGE_PATH,&after);
    let identity = std::fs::read_to_string(tree.0.join("src/master_state_identity.rs")).unwrap();
    let old="realized_post_document: Option<String>,";
    assert!(identity.contains(old));
    tree.put("src/master_state_identity.rs",&identity.replacen(old,
        "realized_post_document: Option<String>,\n    forbidden_observation: bool,",1));
    assert!(validate_render57_positive(&tree.0).is_err());
}
