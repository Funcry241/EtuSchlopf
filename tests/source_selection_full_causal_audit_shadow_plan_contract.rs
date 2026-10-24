use std::fs;
use std::path::Path;

#[test]
fn full_causal_audit_is_source_ordinal_bound_and_reports_productive_plan_authority() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let midi_seed = fs::read_to_string(root.join("src/midi_seed.rs")).expect("midi seed source");
    let causal = fs::read_to_string(root.join("src/source_selection.rs")).expect("causal owner");
    let report = fs::read_to_string(
        root.join("src/audit_only/source_selection_full_causal_audit_shadow_plan.rs"),
    )
    .expect("audit projection");
    let final_report = fs::read_to_string(
        root.join("src/pipeline_shards/final_report_projection_owner.rs"),
    )
    .expect("final report projection owner");

    for pass in [
        "InitialPitchStaff",
        "PianoHandSplit",
        "SmoothFastStaffJump",
        "SmoothWideSimultaneity",
        "HotspotSpan",
        "HotspotJump",
        "TrajectoryMoveOutlier",
        "TrajectoryZigzag",
        "PostTrajectoryWideSimultaneity",
        "ResidualRegisterMove",
        "ResidualCriticalSpan",
        "ResidualCriticalJump",
        "PostResidualWideSimultaneity",
    ] {
        assert!(midi_seed.contains(pass), "missing event-decision pass {pass}");
    }
    assert!(!midi_seed.contains("HotspotRegisterOutlier"));
    assert!(!causal.contains("HotspotRegisterOutlier"));
    assert!(causal.contains("PersistentEventIdentity") || report.contains("PersistentEventIdentity: source_ordinal"));
    assert!(causal.contains("project_source_selection_full_causal_audit"));
    assert!(causal.contains("SourceSelectionShadowPlanRow"));
    assert!(midi_seed.contains("source_selection_profile_name"));
    assert!(causal.contains("SourceSelectionSolverFrontierRow"));
    assert!(report.contains("SolverFrontierFingerprintSHA256"));
    assert!(report.contains("SolverCandidate iteration="));
    assert!(report.contains("SelectionPlanAuthority: productive_seed_materialization"));
    assert!(report.contains("SelectionPlanAuthorityBound:"));
    assert!(report.contains("LegacyMutableWorkingSetMaterializationAuthority: none"));
    assert!(report.contains("ReportWriterAuthority: none"));
    assert!(report.contains("ProductivePlanMaterializationEquivalentToLegacyWorkingSet:"));
    assert!(report.contains("MusicMutation: nein"));
    assert!(final_report.contains("SOURCE-SELECTION-FULL-CAUSAL-AUDIT-AND-SELECTION-PLAN"));
    assert!(!report.contains("MasterApply"));
}

#[test]
fn canonical_and_arrangement_retention_are_explicitly_distinct() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let report = fs::read_to_string(
        root.join("src/audit_only/source_selection_full_causal_audit_shadow_plan.rs"),
    )
    .expect("audit projection");
    assert!(report.contains("CanonicalSourceRetentionPercent"));
    assert!(report.contains("ArrangementVisibleRetentionPercent"));
    assert!(report.contains("RetentionMetricsAreDistinct: ja"));
}
