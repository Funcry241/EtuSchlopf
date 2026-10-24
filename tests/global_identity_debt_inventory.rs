#[path = "../src/build_gate/rust_source_scan_primitives.rs"]
mod rust_source_scan_primitives;
#[allow(dead_code)]
#[path = "../src/build_gate/identity_debt_scan.rs"]
mod identity_debt_scan;

use identity_debt_scan::scan_source_text;

#[test]
fn injected_absolute_measure_writer_is_detected() {
    let source = r#"
fn apply_demo_render_write(xml: &str) -> String {
    let target_measure = "42";
    if target_measure == "42" { xml.to_string() } else { String::new() }
}
"#;
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.iter().any(|finding| {
        finding.kind == "ABSOLUTE_MEASURE_SELECTOR"
            && finding.function == "apply_demo_render_write"
            && finding.payload == "measures=42"
    }));
}

#[test]
fn test_only_localization_is_not_productive_debt() {
    // Productive-scanner scope only. Corpus-independent normative test admission
    // is enforced separately by TakeChangeControl and receives no permission from here.
    let source = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn apply_demo_render_write() {
        let target_measure = "42";
        assert_eq!(target_measure, "42");
    }
}
"#;
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.is_empty());
    assert!(scan.allowed_localizations > 0);
}

#[test]
fn identity_named_writer_is_detected_without_project_literal() {
    let source = r"
fn apply_etude6_demo_render_write(xml: &str) -> String {
    xml.to_string()
}
";
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.iter().any(|finding| {
        finding.kind == "IDENTITY_NAMED_WRITER"
            && finding.function == "apply_etude6_demo_render_write"
            && finding.payload == "symbol=etude6"
    }));
}

#[test]
fn identity_scope_dependency_is_bound_to_the_writer() {
    let source = r#"
fn is_etude6_demo_scope(label: &str) -> bool {
    label.contains("6")
}

fn apply_demo_render_write(xml: &str, label: &str) -> String {
    if is_etude6_demo_scope(label) { xml.to_string() } else { String::new() }
}
"#;
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.iter().any(|finding| {
        finding.kind == "PROJECT_IDENTITY_SELECTOR"
            && finding.function == "is_etude6_demo_scope"
            && finding.payload == "projects=etude6"
    }));
    assert!(scan.findings.iter().any(|finding| {
        finding.kind == "PROJECT_IDENTITY_DEPENDENCY"
            && finding.function == "apply_demo_render_write"
            && finding.payload == "selectors=is_etude6_demo_scope"
    }));
}


#[test]
fn absolute_measure_in_write_dependency_is_detected() {
    let source = r#"
fn allowed_delta_is_only_allowed_xml_delta(xml: &str) -> bool {
    find_measure_span_by_number(xml, "12").is_some()
}
"#;
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.iter().any(|finding| {
        finding.kind == "ABSOLUTE_MEASURE_DEPENDENCY"
            && finding.function == "allowed_delta_is_only_allowed_xml_delta"
            && finding.payload == "measures=12"
    }));
}

#[test]
fn audit_only_absolute_measure_is_allowed_localization() {
    let source = r#"
fn append_demo_audit_no_mutation_section() {
    let target_measure = "42";
    assert_eq!(target_measure, "42");
}
"#;
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.is_empty());
    assert!(scan.allowed_localizations > 0);
}

#[test]
fn generically_named_project_selector_cannot_hide_a_literal_identity() {
    let source = r#"
fn is_target(label: &str) -> bool {
    label == "Etüde Nr. 6"
}
"#;
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.iter().any(|finding| {
        finding.kind == "PROJECT_IDENTITY_SELECTOR"
            && finding.function == "is_target"
            && finding.payload == "projects=etude6"
    }));
}


#[test]
fn audit_name_does_not_hide_a_direct_mutation_surface() {
    let source = r#"
fn append_demo_audit_section(xml: &mut String) {
    let target_measure = "42";
    if target_measure == "42" {
        xml.replace_range(0..0, "");
    }
}
"#;
    let scan = scan_source_text("src/demo.rs", source);
    assert!(scan.findings.iter().any(|finding| {
        finding.kind == "ABSOLUTE_MEASURE_DEPENDENCY"
            && finding.function == "append_demo_audit_section"
            && finding.payload == "measures=42"
    }));
}
