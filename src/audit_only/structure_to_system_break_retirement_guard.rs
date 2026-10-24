//! Test-only closure guard for the retired Structure -> SystemBreak research branch.
//!
//! The accepted r178 corpus result established no directional musical preference.
//! r179 removed the experimental projection; r180 removes the retained r176 inventory.
//! Reintroduction must therefore be explicit and must first remove/update this guard.

#[test]
fn retired_structure_to_system_break_interfaces_are_absent_from_active_rust_sources() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let guard_name = "structure_to_system_break_retirement_guard.rs";
    let forbidden = [
        "structure_layout_capability_gap",
        "structure_to_layout_counterfactual",
        "StructureLayoutCapabilityGap",
        "StructureToLayoutCounterfactual",
        "PhraseStructureLayoutCapabilityProjection",
        "structure_layout_capability_projection",
        "StructureToSystemBreakPreference",
    ];

    let mut stack = vec![root];
    while let Some(path) = stack.pop() {
        let entries = std::fs::read_dir(&path)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
        for entry in entries {
            let entry = entry.unwrap_or_else(|error| panic!("cannot read source entry: {error}"));
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs")
                || path.file_name().and_then(|name| name.to_str()) == Some(guard_name)
            {
                continue;
            }
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
            for token in forbidden {
                assert!(
                    !source.contains(token),
                    "retired Structure -> SystemBreak interface token {token:?} returned in {}",
                    path.display()
                );
            }
        }
    }
}

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
