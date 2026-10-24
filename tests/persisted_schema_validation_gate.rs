//! Executable regression for the deliberate Gate-A / Gate-B separation.
//!
//! MusicXML 4.0 XSD validity does not imply EtuSchlopf's declared product version. The product
//! policy remains owned by EtuSchlopf while SchemaSchlopf owns only XSD conformance.

use roxmltree::Document;
use schemaschlopf::{MUSICXML40_SCHEMA_DIR, compile_bound_instance_validator};
use std::path::Path;

const EXPECTED_PRODUCT_VERSION: &str = "4.0";

const VALID_DECLARED_31: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="3.1">
  <part-list>
    <score-part id="P1"><part-name>Piano</part-name></score-part>
  </part-list>
  <part id="P1"><measure number="1"/></part>
</score-partwise>"#;

const VALID_DECLARED_40: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Piano</part-name></score-part>
  </part-list>
  <part id="P1"><measure number="1"/></part>
</score-partwise>"#;

fn declared_product_version(bytes: &[u8]) -> Option<String> {
    let xml = std::str::from_utf8(bytes).ok()?;
    let document = Document::parse(xml).ok()?;
    let root = document.root_element();
    if root.tag_name().name() != "score-partwise" {
        return None;
    }
    root.attribute("version").map(str::to_owned)
}

#[test]
fn xsd40_validity_does_not_imply_declared_product_version40() {
    let validator = compile_bound_instance_validator(Path::new(MUSICXML40_SCHEMA_DIR))
        .expect("bound MusicXML 4.0 profile must compile");
    validator
        .validate_bytes(VALID_DECLARED_31)
        .expect("4.0 XSD may accept a 3.1 declaration");
    assert_ne!(
        declared_product_version(VALID_DECLARED_31).as_deref(),
        Some(EXPECTED_PRODUCT_VERSION)
    );
}

#[test]
fn declared40_and_xsd40_are_separate_green_facts() {
    assert_eq!(
        declared_product_version(VALID_DECLARED_40).as_deref(),
        Some(EXPECTED_PRODUCT_VERSION)
    );
    let validator = compile_bound_instance_validator(Path::new(MUSICXML40_SCHEMA_DIR))
        .expect("bound MusicXML 4.0 profile must compile");
    let report = validator
        .validate_bytes(VALID_DECLARED_40)
        .expect("valid MusicXML 4.0 fixture");
    assert!(report.instance_validation_qualified());
}
