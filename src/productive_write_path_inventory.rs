#[derive(Debug, Clone, Copy)]
pub struct StaticProductiveWriteSurface {
    pub path: &'static str,
    pub function: &'static str,
    pub line: usize,
    pub xml_buffer_mutations: usize,
    pub xml_file_writes: usize,
    pub direct_from_real_main: bool,
    pub authority_boundary: bool,
    pub class: &'static str,
    pub evidence: &'static str,
}

include!(env!("ETUSCHLOPF_PRODUCTIVE_WRITE_PATH_INVENTORY_RS"));

pub fn append_productive_write_path_inventory_section(out: &mut String) {
    let mut buffer_surface_functions = 0usize;
    let mut file_surface_functions = 0usize;
    let mut dual_surface_functions = 0usize;
    let mut buffer_operations = 0usize;
    let mut file_write_operations = 0usize;
    let mut authority_surfaces = 0usize;
    let mut legacy_surfaces = 0usize;
    let mut direct_from_real_main = 0usize;
    for surface in PRODUCTIVE_WRITE_PATH_SURFACES {
        if surface.xml_buffer_mutations > 0 {
            buffer_surface_functions += 1;
            buffer_operations += surface.xml_buffer_mutations;
        }
        if surface.xml_file_writes > 0 {
            file_surface_functions += 1;
            file_write_operations += surface.xml_file_writes;
        }
        if surface.xml_buffer_mutations > 0 && surface.xml_file_writes > 0 {
            dual_surface_functions += 1;
        }
        if surface.authority_boundary {
            authority_surfaces += 1;
        } else {
            legacy_surfaces += 1;
        }
        if surface.direct_from_real_main {
            direct_from_real_main += 1;
        }
    }

    out.push_str(&format!(
        "Status: v{} build-time rekursiv ueber src/**/*.rs; projektunabhaengige Detailzeilen sind nicht mehr je Ergebnisbericht dupliziert. Die vollstaendige Flaechenliste bleibt im generierten Build-Inventar gebunden.\n",
        env!("CARGO_PKG_VERSION")
    ));
    out.push_str(&format!(
        "InventorySummary: SourceFiles {}; FunctionsScanned {}; DirectWriteSurfaceFunctions {}; XmlBufferSurfaceFunctions {}; XmlFileWriteSurfaceFunctions {}; DualSurfaceFunctions {}; XmlBufferMutationOperations {}; XmlFileWriteOperations {}.\n",
        PRODUCTIVE_WRITE_PATH_SOURCE_FILES,
        PRODUCTIVE_WRITE_PATH_FUNCTIONS_SCANNED,
        PRODUCTIVE_WRITE_PATH_SURFACES.len(),
        buffer_surface_functions,
        file_surface_functions,
        dual_surface_functions,
        buffer_operations,
        file_write_operations
    ));
    out.push_str(&format!(
        "AuthorityCoverage: CentralAuthoritySurfaces {}; LegacyOutsideAuthoritySurfaces {}; DirectlyReferencedByRealMain {}; SingleMandatoryWriteBoundary {}.\n",
        authority_surfaces,
        legacy_surfaces,
        direct_from_real_main,
        yes_no(legacy_surfaces == 0)
    ));
    out.push_str(&format!(
        "HardBypassShrinkRegister: BaselineExternalSurfaces {}; CurrentExternalSurfaces {}; RetiredExternalSurfaces {}; UnknownExternalSurfaces {}; OperationEscalations {}; DirectMainEscalations {}; DuplicateCurrentSurfaces {}; RegisterSubsetGreen {}; BaselineFingerprint FNV1A64:{:016x}.\n",
        PRODUCTIVE_WRITE_BYPASS_BASELINE_EXTERNAL_SURFACES,
        PRODUCTIVE_WRITE_BYPASS_CURRENT_EXTERNAL_SURFACES,
        PRODUCTIVE_WRITE_BYPASS_RETIRED_EXTERNAL_SURFACES,
        PRODUCTIVE_WRITE_BYPASS_UNKNOWN_EXTERNAL_SURFACES,
        PRODUCTIVE_WRITE_BYPASS_OPERATION_ESCALATIONS,
        PRODUCTIVE_WRITE_BYPASS_DIRECT_MAIN_ESCALATIONS,
        PRODUCTIVE_WRITE_BYPASS_DUPLICATE_CURRENT_SURFACES,
        yes_no(PRODUCTIVE_WRITE_BYPASS_REGISTER_GREEN),
        PRODUCTIVE_WRITE_BYPASS_BASELINE_FNV1A64
    ));
    out.push_str(&format!(
        "InventoryBinding: SurfaceRows {}; SurfaceFingerprint FNV1A64:{:016x}; DetailedRowsProjected nein; DetailedRowsBuildBound ja; NewExternalSurfaceStopsBuild ja; OperationEscalationStopsBuild ja; NewRealMainReachabilityStopsBuild ja.\n",
        PRODUCTIVE_WRITE_PATH_SURFACES.len(),
        productive_write_path_inventory_fingerprint()
    ));
    out.push_str("BoundaryPolicy: XML-nahe String-Mutationen und Dateischreibsenken bleiben build-time vollstaendig inventarisiert. Test-only-Funktionen, reine Reporttextmutation und nicht-XML-Artefaktsenken bleiben ausgeschlossen. RegisteredIdentityDebt 0, UnknownIdentityDebt 0, MusicMutation nein und MasterApply nein bleiben eigenstaendige Gates; konkrete Takte, Projekte, Etuedennamen, Tonhoehen oder Locator erteilen kein Schreibrecht.\n");
}

fn productive_write_path_inventory_fingerprint() -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for surface in PRODUCTIVE_WRITE_PATH_SURFACES {
        hash_fingerprint_field(&mut hash, surface.path.as_bytes());
        hash_fingerprint_field(&mut hash, surface.function.as_bytes());
        hash_fingerprint_u64(&mut hash, surface.line as u64);
        hash_fingerprint_u64(&mut hash, surface.xml_buffer_mutations as u64);
        hash_fingerprint_u64(&mut hash, surface.xml_file_writes as u64);
        hash_fingerprint_field(&mut hash, &[u8::from(surface.direct_from_real_main)]);
        hash_fingerprint_field(&mut hash, &[u8::from(surface.authority_boundary)]);
        hash_fingerprint_field(&mut hash, surface.class.as_bytes());
        hash_fingerprint_field(&mut hash, surface.evidence.as_bytes());
    }
    hash_fingerprint_u64(&mut hash, PRODUCTIVE_WRITE_PATH_SOURCE_FILES as u64);
    hash_fingerprint_u64(&mut hash, PRODUCTIVE_WRITE_PATH_FUNCTIONS_SCANNED as u64);
    hash_fingerprint_u64(
        &mut hash,
        PRODUCTIVE_WRITE_BYPASS_BASELINE_FNV1A64,
    );
    hash
}

fn hash_fingerprint_u64(hash: &mut u64, value: u64) {
    hash_fingerprint_field(hash, &value.to_le_bytes());
}

fn hash_fingerprint_field(hash: &mut u64, bytes: &[u8]) {
    for byte in (bytes.len() as u64).to_le_bytes().iter().chain(bytes) {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

#[cfg(test)]
#[allow(dead_code)]
#[path = "build_gate/productive_write_path_scan.rs"]
mod productive_write_path_scan_test_support;

#[cfg(test)]
mod productive_write_path_inventory_tests {
    use super::productive_write_path_scan_test_support::{
        parse_productive_write_bypass_baseline,
        productive_write_bypass_baseline_fingerprint,
        scan_productive_write_paths_text,
        validate_productive_write_bypass_register,
    };

    #[test]
    fn direct_xml_buffer_mutation_is_detected() {
        let source = r#"
fn apply_demo_render_write(xml: &str) -> String {
    let mut candidate_xml = xml.to_string();
    candidate_xml.replace_range(0..0, "<note/>");
    candidate_xml
}
"#;
        let scan = scan_productive_write_paths_text("src/demo.rs", source);
        assert_eq!(scan.surfaces.len(), 1);
        assert_eq!(scan.surfaces[0].xml_buffer_mutations, 1);
        assert!(!scan.surfaces[0].authority_boundary);
    }

    #[test]
    fn central_authority_surface_is_classified_separately() {
        let source = r#"
fn rewrite_main_lane_note(xml: &str) -> String {
    let mut rewritten = xml.to_string();
    rewritten.insert_str(0, "<note/>");
    rewritten
}
"#;
        let scan = scan_productive_write_paths_text(
            "src/structural_write_authority/writer_entry_boundary.rs",
            source,
        );
        assert_eq!(scan.surfaces.len(), 1);
        assert!(scan.surfaces[0].authority_boundary);
        assert_eq!(
            scan.surfaces[0].class,
            "CENTRAL_AUTHORITY_XML_BUFFER_SURFACE"
        );
    }

    #[test]
    fn pure_report_text_replacement_is_not_a_music_write_surface() {
        let source = r#"
fn sanitize_report_text(text: &str) -> String {
    let mut text = text.to_string();
    text.replace_range(0..0, "status");
    text
}
"#;
        let scan = scan_productive_write_paths_text("src/report.rs", source);
        assert!(scan.surfaces.is_empty());
    }

    #[test]
    fn parser_cleanup_is_not_counted_as_productive_write() {
        let source = r#"
fn parse_attr(xml: &str) -> String {
    let mut cleaned = xml.to_string();
    cleaned.replace_range(0..0, "");
    cleaned
}
"#;
        let scan = scan_productive_write_paths_text("src/layout.rs", source);
        assert!(scan.surfaces.is_empty());
    }

    #[test]
    fn xml_file_sink_is_detected() {
        let source = r"
fn build_render_xml_trial(output_path: &std::path::Path, candidate_xml: &str) {
    std::fs::write(output_path, candidate_xml).unwrap();
}
";
        let scan = scan_productive_write_paths_text("src/demo.rs", source);
        assert_eq!(scan.surfaces.len(), 1);
        assert_eq!(scan.surfaces[0].xml_file_writes, 1);
    }

    #[test]
    fn audit_named_xml_file_sink_is_detected() {
        let source = r"
fn build_behind_bars_review_rest_and_tie_audit(
    output_path: &std::path::Path,
    candidate_xml: &str,
) {
    std::fs::write(output_path, candidate_xml).unwrap();
}
";
        let scan = scan_productive_write_paths_text("src/demo.rs", source);
        assert_eq!(scan.surfaces.len(), 1);
        assert_eq!(scan.surfaces[0].xml_file_writes, 1);
    }

    #[test]
    fn context_named_xml_delta_buffer_mutation_is_detected() {
        let source = r#"
fn build_clef_context_hysteresis_surface_delta(xml: &str) -> String {
    let mut out = xml.to_string();
    out.replace_range(0..0, "<clef/>");
    out
}
"#;
        let scan = scan_productive_write_paths_text("src/demo.rs", source);
        assert_eq!(scan.surfaces.len(), 1);
        assert_eq!(scan.surfaces[0].xml_buffer_mutations, 1);
    }

    #[test]
    fn test_only_mutation_is_excluded() {
        let source = r#"
#[cfg(test)]
mod tests {
    #[test]
    fn mutates_xml_fixture() {
        let mut candidate_xml = String::new();
        candidate_xml.insert_str(0, "<note/>");
    }
}
"#;
        let scan = scan_productive_write_paths_text("src/demo.rs", source);
        assert!(scan.surfaces.is_empty());
    }

    #[test]
    fn final_report_projects_write_inventory_after_compaction() {
        let orchestrator = include_str!(
            "pipeline_shards/final_report_projection_owner.rs"
        );
        let consolidate = orchestrator
            .find("compact(compact_current_report_preamble_v0101536)")
            .expect("direct current report projection call");
        let projection = orchestrator
            .find("productive_write_path_inventory::append_productive_write_path_inventory_section,")
            .expect("productive write path report projection");
        assert!(projection > consolidate);
        assert_eq!(
            orchestrator
                .matches("productive_write_path_inventory::append_productive_write_path_inventory_section,")
                .count(),
            1
        );
    }

    #[test]
    fn current_inventory_is_nonempty_and_mandatory_boundary_is_closed() {
        let mut report = String::new();
        super::append_productive_write_path_inventory_section(&mut report);
        assert!(report.contains("SingleMandatoryWriteBoundary ja"));
        assert!(report.contains("LegacyOutsideAuthoritySurfaces"));
        assert!(report.contains("HardBypassShrinkRegister"));
        assert!(report.contains("NewExternalSurfaceStopsBuild ja"));
        assert!(report.contains("DetailedRowsProjected nein"));
        assert!(!report.contains("WriteSurface:"));
        assert_ne!(super::productive_write_path_inventory_fingerprint(), 0);
        assert_eq!(super::PRODUCTIVE_WRITE_BYPASS_BASELINE_EXTERNAL_SURFACES, 1);
        assert_eq!(super::PRODUCTIVE_WRITE_BYPASS_CURRENT_EXTERNAL_SURFACES, 0);
        assert_eq!(super::PRODUCTIVE_WRITE_BYPASS_RETIRED_EXTERNAL_SURFACES, 1);
        assert_eq!(super::PRODUCTIVE_WRITE_BYPASS_UNKNOWN_EXTERNAL_SURFACES, 0);
        const { assert!(super::PRODUCTIVE_WRITE_BYPASS_REGISTER_GREEN); }
        assert!(!super::PRODUCTIVE_WRITE_PATH_SURFACES.is_empty());
    }

    #[test]
    fn post_cutover_build_gate_requires_zero_external_musicxml_surfaces() {
        let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut kodex_sources = vec![(
            "src/kodex_build_orchestrator.rs".to_string(),
            include_str!("kodex_build_orchestrator.rs").to_string(),
        )];
        let build_gate_dir = manifest_dir.join("src/build_gate");
        for entry in std::fs::read_dir(&build_gate_dir).expect("read src/build_gate") {
            let entry = entry.expect("read src/build_gate entry");
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("rs") {
                continue;
            }
            let relative = path
                .strip_prefix(manifest_dir)
                .expect("build-gate path under manifest root")
                .to_string_lossy()
                .replace('\\', "/");
            let text = std::fs::read_to_string(&path).expect("read build-gate source");
            kodex_sources.push((relative, text));
        }

        let owners = kodex_sources
            .iter()
            .filter(|(_, source)| {
                source.contains("REQUIRE_ZERO_EXTERNAL_MUSICXML_SURFACES: bool = true")
                    && source.contains("SINGLE MANDATORY WRITE BOUNDARY REGRESSION")
            })
            .map(|(path, _)| path.clone())
            .collect::<Vec<_>>();

        assert_eq!(
            owners.len(),
            1,
            "zero-external MusicXML kodex contract must have exactly one source owner, got {owners:?}"
        );
    }

    fn frozen_baseline() -> Vec<super::productive_write_path_scan_test_support::ProductiveWriteBypassBaselineRow> {
        parse_productive_write_bypass_baseline(include_str!(
            "build_gate/productive_write_bypass_baseline.tsv"
        ))
        .expect("frozen productive write bypass baseline")
    }

    #[test]
    fn frozen_bypass_baseline_has_expected_size_and_fingerprint() {
        let text = include_str!("build_gate/productive_write_bypass_baseline.tsv");
        assert!(
            !text.contains("<locator>"),
            "the executable build baseline must contain exact Rust function names, not report-redacted placeholders"
        );
        assert_eq!(frozen_baseline().len(), 1);
        assert_eq!(
            productive_write_bypass_baseline_fingerprint(text),
            0xe81d_f25a_184e_d737
        );
    }

    #[test]
    fn new_external_write_surface_is_rejected_by_register() {
        let scan = scan_productive_write_paths_text(
            "src/new_external.rs",
            r#"
fn new_external_render_writer(xml: &str) -> String {
    let mut candidate_xml = xml.to_string();
    candidate_xml.insert_str(0, "<note/>");
    candidate_xml
}
"#,
        );
        let validation = validate_productive_write_bypass_register(
            &scan.surfaces,
            &frozen_baseline(),
        );
        assert!(!validation.is_green());
        assert_eq!(validation.unknown_external_surfaces.len(), 1);
    }

    #[test]
    fn existing_surface_operation_escalation_is_rejected() {
        let baseline = parse_productive_write_bypass_baseline(
            "src/demo.rs\tknown_writer\t1\t0\tnein\towner\tmigrate\n",
        )
        .expect("test baseline");
        let scan = scan_productive_write_paths_text(
            "src/demo.rs",
            r#"
fn known_writer(xml: &str) -> String {
    let mut candidate_xml = xml.to_string();
    candidate_xml.insert_str(0, "<note/>");
    candidate_xml.insert_str(0, "<rest/>");
    candidate_xml
}
"#,
        );
        let validation = validate_productive_write_bypass_register(&scan.surfaces, &baseline);
        assert!(!validation.is_green());
        assert_eq!(validation.operation_escalations.len(), 1);
    }

    #[test]
    fn new_real_main_reachability_is_rejected() {
        let baseline = parse_productive_write_bypass_baseline(
            "src/demo.rs\tknown_writer\t1\t0\tnein\towner\tmigrate\n",
        )
        .expect("test baseline");
        let scan = scan_productive_write_paths_text(
            "src/demo.rs",
            r#"
fn known_writer(xml: &str) -> String {
    let mut candidate_xml = xml.to_string();
    candidate_xml.insert_str(0, "<note/>");
    candidate_xml
}

fn real_main() {
    let _ = known_writer("<score-partwise/>");
}
"#,
        );
        let validation = validate_productive_write_bypass_register(&scan.surfaces, &baseline);
        assert!(!validation.is_green());
        assert_eq!(validation.direct_main_escalations.len(), 1);
    }

    #[test]
    fn pure_external_surface_removal_is_allowed() {
        let baseline = parse_productive_write_bypass_baseline(
            "src/demo.rs\tretired_writer\t1\t0\tnein\towner\tmigrate\n",
        )
        .expect("test baseline");
        let validation = validate_productive_write_bypass_register(&[], &baseline);
        assert!(validation.is_green());
        assert_eq!(validation.retired_external_surfaces, 1);
        assert_eq!(validation.current_external_surfaces, 0);
    }

    #[test]
    fn new_central_authority_surface_does_not_expand_external_register() {
        let scan = scan_productive_write_paths_text(
            "src/structural_write_authority/new_writer.rs",
            r#"
fn central_writer(xml: &str) -> String {
    let mut candidate_xml = xml.to_string();
    candidate_xml.insert_str(0, "<note/>");
    candidate_xml
}
"#,
        );
        let validation = validate_productive_write_bypass_register(
            &scan.surfaces,
            &frozen_baseline(),
        );
        assert!(validation.is_green());
        assert_eq!(validation.current_external_surfaces, 0);
        assert_eq!(validation.retired_external_surfaces, 1);
    }
}
