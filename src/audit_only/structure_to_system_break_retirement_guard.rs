
#[test]
fn phrase_structure_owner_has_no_direct_layout_plan_dependency() {
    let source = include_str!("phrase_boundary_evidence.rs");
    for token in [
        "HorizontalLayoutPlan",
        "starts_new_system",
        "starts_new_page",
        "system_break_slots",
        "page_break_slots",
        "explicit_width_slots",
    ] {
        assert!(
            !source.contains(token),
            "phrase_boundary_evidence must remain layout-decision-independent; found {token:?}"
        );
    }
}
