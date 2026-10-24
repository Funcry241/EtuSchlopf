use std::fs;
use std::path::Path;

#[test]
fn immutable_selection_plan_is_the_productive_seed_materialization_source() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let midi_seed = fs::read_to_string(root.join("src/midi_seed.rs")).expect("midi seed source");
    let selection =
        fs::read_to_string(root.join("src/source_selection.rs")).expect("selection source");

    let audit_pos = midi_seed
        .find("project_source_selection_full_causal_audit(")
        .expect("r634 causal plan projection");
    let bind_pos = midi_seed
        .find("bind_productive_source_selection_plan(")
        .expect("r635 productive plan binding");
    let rebuild_pos = midi_seed
        .find("notes = productive_plan_retained_identities")
        .expect("productive seed rebuild from immutable plan");
    let stats_pos = midi_seed.find("let stats = MidiSeedStats").expect("seed stats");
    let write_pos = midi_seed.find("build_musicxml(title, &notes").expect("MusicXML seed build");

    assert!(audit_pos < bind_pos);
    assert!(bind_pos < rebuild_pos);
    assert!(rebuild_pos < stats_pos);
    assert!(stats_pos < write_pos);

    assert!(selection.contains("SourceSelectionPlanAuthorityCutover"));
    assert!(selection.contains("planned_retained != legacy_retained_events"));
    assert!(selection.contains("productive_plan_equivalence_verified = true"));
    assert!(selection.contains("productive_plan_authority = true"));
}

#[test]
fn authority_cutover_does_not_turn_canonical_retention_into_selection_policy() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let selection =
        fs::read_to_string(root.join("src/source_selection.rs")).expect("selection source");
    let pipeline = fs::read_to_string(
        root.join("src/pipeline_shards/pipeline_orchestrator_arrangement_solver_stage.rs"),
    )
    .expect("solver source");
    let report = fs::read_to_string(
        root.join("src/audit_only/source_selection_full_causal_audit_shadow_plan.rs"),
    )
    .expect("selection audit report");

    assert!(selection.contains("canonical_source_retention_ppm"));
    assert!(report.contains("RetentionMetricsAreDistinct: ja"));
    assert!(report.contains("SelectionRulesChanged: nein"));
    assert!(report.contains("RankingChanged: nein"));
    assert!(report.contains("WeightingChanged: nein"));
    assert!(report.contains("ThresholdsChanged: nein"));

    assert!(!pipeline.contains("canonical_source_retention_ppm <"));
    assert!(!pipeline.contains("canonical_source_retention_ppm >"));
    assert!(!pipeline.contains("canonical_source_retention_ppm.cmp"));
}
