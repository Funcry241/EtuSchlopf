use std::fs;
use std::path::Path;

fn read(root: &Path, relative: &str) -> String {
    fs::read_to_string(root.join(relative)).unwrap_or_else(|error| {
        panic!("report-sanitizer source {relative} must be readable: {error}")
    })
}

#[test]
fn score_title_external_metadata_is_opaque_to_internal_report_sanitizers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let musicxml = read(root, "src/musicxml.rs");
    let report = read(root, "src/report.rs");
    let projection = read(
        root,
        "src/pipeline_shards/final_report_projection_owner.rs",
    );
    let sanitizer = read(root, "src/report_sanitizer.rs");
    let former_mixed_owner = read(
        root,
        "src/pipeline_shards/audio_salience_discrete_action_conflict_gate_tail.rs",
    );
    let final_report = read(root, "src/final_report.rs");
    let scope_owner = read(
        root,
        "src/audit_only/report_sanitizer_rewrite_scope_projection.rs",
    );

    assert!(musicxml.contains("first_descendant_text(root, \"movement-title\")"));
    assert!(musicxml.contains("first_descendant_text(root, \"work-title\")"));
    assert!(report.contains("report.score_title.as_ref()"));
    assert!(report.contains("Partitur: {title}"));

    let preamble = projection
        .find("let mut report_preamble = write_text_report")
        .expect("report preamble creation");
    let compact = projection[preamble..]
        .find("DraftReport::new(report_preamble).compact(compact_current_report_preamble_v0101536)")
        .map(|offset| preamble + offset)
        .expect("typed compact report owner");
    let whole_document = projection[compact..]
        .find("sanitize_behind_bars_active_authority_language_v1284")
        .map(|offset| compact + offset)
        .expect("whole-document behind-bars sanitizer call");
    assert!(preamble < compact);
    assert!(compact < whole_document);
    assert!(scope_owner.contains("pub(crate) const EXTERNAL_METADATA_OPAQUE: bool = true;"));
    assert!(scope_owner.contains("ACTIVE_REPORT_REWRITE_SCOPE"));
    assert!(scope_owner.contains("external_metadata_opaque: EXTERNAL_METADATA_OPAQUE"));
    assert!(final_report.contains("external_metadata_opaque"));
    assert!(final_report.contains("sanitize_report_hygiene_polish"));
    assert!(!final_report.contains("fn rewrite_scope_text"));

    assert!(sanitizer.contains("(\"HumanReview\", \"BehindBarsRuleCheck\")"));
    assert!(!former_mixed_owner.contains("fn sanitize_report_hygiene_polish"));
    assert!(!former_mixed_owner.contains("fn sanitize_behind_bars_active_authority_language_v1284"));
    assert!(final_report.contains("ReportFragmentOrigin"));
    assert!(final_report.contains("ExternalMetadata"));
    assert!(final_report.contains("content.starts_with(\"Partitur: \")"));
}
