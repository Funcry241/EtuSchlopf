const REPLACEMENT_AUTHORITY: &str = include_str!(
    "../src/structural_write_authority/strict_staff_visible_lane_authority.rs",
);
const PROJECT_TRY: &str = include_str!(
    "../src/pipeline_shards/pipeline_orchestrator_report_compaction.rs",
);
const CORPUS_TRY: &str = include_str!(
    "../src/pipeline_shards/rest_glyph_retention_batch_corpus_tail.rs",
);
const VALIDATOR_AUTHORITY: &str = include_str!("../src/rule_id.rs");

#[test]
fn rust_2024_expr_fragments_are_used_at_the_three_surviving_macro_boundaries() {
    assert!(PROJECT_TRY.contains("($expression:expr)"));
    assert!(CORPUS_TRY.contains("($expression:expr)"));
    assert!(VALIDATOR_AUTHORITY.contains("rule_id: $rule_id:expr,"));
    assert!(VALIDATOR_AUTHORITY.contains("actual_unit: $actual_unit:expr,"));
    assert!(VALIDATOR_AUTHORITY.contains(
        "runtime_severity_profile: $runtime_severity_profile:expr,",
    ));
    assert!(VALIDATOR_AUTHORITY.contains("priority_profile: $priority_profile:expr,"));
    assert!(VALIDATOR_AUTHORITY.contains("repair_hint: $repair_hint:expr $(,)?"));
}

#[test]
fn replacement_authority_uses_a_real_rust_2024_const_expression_probe() {
    assert!(REPLACEMENT_AUTHORITY.contains(
        "const { VisibleLaneEffect::BackupAnchor }",
    ));
}
