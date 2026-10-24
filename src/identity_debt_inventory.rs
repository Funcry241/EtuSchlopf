#[derive(Debug, Clone, Copy)]
pub struct StaticIdentityDebtFinding {
    pub kind: &'static str,
    pub path: &'static str,
    pub function: &'static str,
    pub line: usize,
    pub payload: &'static str,
    pub evidence: &'static str,
    pub owner: &'static str,
    pub decision: &'static str,
}

include!(env!("ETUSCHLOPF_IDENTITY_DEBT_INVENTORY_RS"));

pub fn append_global_identity_debt_inventory_section(out: &mut String) {
    crate::push_static_section_header_v793(
        out,
        "\nGLOBAL-IDENTITY-DEBT-INVENTORY",
        "--------------------------------",
    );
    out.push_str("Status: build-time rekursiv ueber src/**/*.rs und den expliziten Kodex-Gate-Primitiv-Owner; produktive und produktionsnahe Identitaetsabhaengigkeiten werden gegen ein geschlossenes Altlastenregister geprueft. Neue oder still veraenderte Schuld stoppt den Build. Audit-only; keine MusicXML-/Render-/PDF-Aenderung.\n");
    out.push_str(&format!(
        "SourceFiles: {}; RegisteredIdentityDebt: {}; UnknownIdentityDebt: {}; AllowedLocalizationMarkers: {}.\n",
        GLOBAL_IDENTITY_DEBT_SOURCE_FILES,
        GLOBAL_IDENTITY_DEBT_FINDINGS.len(),
        GLOBAL_IDENTITY_DEBT_UNKNOWN,
        GLOBAL_IDENTITY_DEBT_ALLOWED_LOCALIZATIONS
    ));

    let mut absolute_measure = 0usize;
    let mut absolute_measure_dependency = 0usize;
    let mut project_identity = 0usize;
    let mut project_identity_dependency = 0usize;
    let mut identity_named_writer = 0usize;
    let mut feedback_origin_named_writer = 0usize;
    let mut productive_feedback_origin_channel = 0usize;
    let mut productive_identity_channel_predicate = 0usize;
    let mut concrete_pitch_anchor = 0usize;
    for finding in GLOBAL_IDENTITY_DEBT_FINDINGS {
        match finding.kind {
            "ABSOLUTE_MEASURE_SELECTOR" => absolute_measure += 1,
            "ABSOLUTE_MEASURE_DEPENDENCY" => absolute_measure_dependency += 1,
            "PROJECT_IDENTITY_SELECTOR" => project_identity += 1,
            "PROJECT_IDENTITY_DEPENDENCY" => project_identity_dependency += 1,
            "IDENTITY_NAMED_WRITER" => identity_named_writer += 1,
            "FEEDBACK_ORIGIN_NAMED_WRITER" => feedback_origin_named_writer += 1,
            "PRODUCTIVE_FEEDBACK_ORIGIN_CHANNEL" => productive_feedback_origin_channel += 1,
            "PRODUCTIVE_IDENTITY_CHANNEL_PREDICATE" => productive_identity_channel_predicate += 1,
            "CONCRETE_PITCH_ANCHOR" => concrete_pitch_anchor += 1,
            _ => {}
        }
    }
    out.push_str(&format!(
        "DebtClasses: AbsoluteMeasureSelector {}; AbsoluteMeasureDependency {}; ProjectIdentitySelector {}; ProjectIdentityDependency {}; IdentityNamedWriter {}; FeedbackOriginNamedWriter {}; ProductiveFeedbackOriginChannel {}; ProductiveIdentityChannelPredicate {}; ConcretePitchAnchor {}.\n",
        absolute_measure,
        absolute_measure_dependency,
        project_identity,
        project_identity_dependency,
        identity_named_writer,
        feedback_origin_named_writer,
        productive_feedback_origin_channel,
        productive_identity_channel_predicate,
        concrete_pitch_anchor
    ));

    for finding in GLOBAL_IDENTITY_DEBT_FINDINGS {
        out.push_str(&format!(
            "Debt: {} | {}:{} | fn {} | {} | Owner {} | Decision {} | Evidence {}.\n",
            finding.kind,
            finding.path,
            finding.line,
            finding.function,
            finding.payload,
            finding.owner,
            finding.decision,
            finding.evidence
        ));
    }
    out.push_str("BuildGate: lexical_identity_debt=hard_fail; productive_music_identity_provenance=hard_fail; baseline_data_rows=hard_fail; registered_debt_buildable=nein; scanner_and_empty_baseline_fingerprints=pinned.\n");
    out.push_str(&format!(
        "SelfProtection: LexicalIdentityDebtZero {}; ProductiveMusicIdentityProvenanceFirewallGreen {}; CompatibilityZeroDebtFlag {}; ScannerFNV1A64 {:016x}; EmptyBaselineFNV1A64 {:016x}.\n",
        if GLOBAL_LEXICAL_IDENTITY_DEBT_ZERO { "ja" } else { "nein" },
        if PRODUCTIVE_MUSIC_IDENTITY_PROVENANCE_FIREWALL_GREEN { "ja" } else { "nein" },
        if GLOBAL_IDENTITY_ZERO_DEBT_ENFORCED { "ja" } else { "nein" },
        GLOBAL_IDENTITY_SCANNER_FNV1A64,
        GLOBAL_IDENTITY_BASELINE_FNV1A64
    ));
    out.push_str("Interpretation: Der lexikalische Nullschuld-Scanner und die produktive Provenienz-Firewall sind getrennte Beweise. Exakte Etueden-/Taktkoordinaten duerfen read-only Diagnose und Audit lokalisieren; normative Tests unterliegen zusaetzlich der Corpus-Independence-Admission und duerfen daraus keine dauerhafte konkrete Testidentitaet machen. Externe oder rohe Locator duerfen keine produktive musikalische Decision-/Writer-Grenze erreichen. Intern strukturell gebundene Locator bleiben erlaubt.\n");
}

pub fn append_feedback_identity_firewall_section(out: &mut String) {
    crate::push_static_section_header_v793(
        out,
        "\nFEEDBACK-IDENTITY-FIREWALL",
        "--------------------------",
    );
    out.push_str("FeedbackIntake: Etuede und Takt sind als menschliche Diagnosekoordinaten erlaubt.\n");
    out.push_str("InformationWithholding: Vor Regelbildung werden Projektname, Ursprungstakt und Feedback-Herkunft aus dem produktiven Kandidatenkontext entfernt.\n");
    out.push_str("ProductiveRuleInput: nur strukturelle Merkmale, Invarianten, lokale Nachbarschaft und korpusweit reproduzierbare Evidenz.\n");
    out.push_str("RelocationProof: derselbe Strukturfehler muss nach Projektumbenennung, Taktverschiebung, Herkunftsentfernung und an einem zweiten Vorkommen dieselbe Entscheidung erzeugen; negative Kontrollen muessen abgewiesen werden; sonst WriterOpen nein.\n");
    out.push_str("MaterializedProof: Die detaillierten Nichtbeeinflussungspruefungen laufen ausschliesslich privat und fluechtig im Sicherheitsmodul; der Bericht erhaelt genau eine kanonische Minimalprojektion ohne Variantenlisten, exakte Zaehler oder Struktur-Fingerprints. Gleicher Minimalzustand erzeugt innerhalb derselben Version bytegleiche Abschnittsdaten.\n");
    out.push_str("OriginReadback: Die Ursprungskoordinate darf nach der Entscheidung nur pruefen, ob der generische Detektor den gemeldeten Fall wiederfindet; sie darf den Fall nicht auswaehlen.\n");
    out.push_str("SelfProtection: Der lexikalische Identity-Scanner bleibt als eigener Nullschuld-Beweis aktiv. Zusaetzlich verfolgt die Productive-Music-Identity-Provenance-Firewall externe Runtime-Eingaenge ueber den bestehenden semantischen Callgraph und blockiert deren Schnittmenge mit identitaetssensitiven produktiven Music-/Writer-Pfaden. Rohe oeffentliche Target-/Measure-Locator-Primitiven sind dort gesperrt; verifizierte opake Structural-Locator bleiben zulaessig. Konsole, Debuglog, Fehlerpayload, Dateiname, temporaeres Artefakt, Cache-Schluessel und Manifestkontext sind fuer private Beweisdetails geschlossen.\n");
    out.push_str("WriterOpen nein fuer feedback-spezifische Regeln; MusicMutation nein; MasterApply nein.\n");
    append_kodex_boundary_adversarial_completion_section(out);
}

fn append_kodex_boundary_adversarial_completion_section(out: &mut String) {
    crate::push_static_section_header_v793(
        out,
        "\nKODEX-BOUNDARY-ADVERSARIAL-COMPLETION",
        "------------------------------------------",
    );
    out.push_str("Status: letzte planmaessige abstrakte Sicherheitsabschlusspruefung; keine neue produktive Architektur und keine Musikmutation.\n");
    out.push_str(&format!(
        "AdversarialMatrix: Blocked {}/{}; UnknownAttackDenied ja; KodexFreezeReady {}; WriterEligible nein; MusicMutation nein; MasterApply nein.\n",
        env!("ETUSCHLOPF_KODEX_ADVERSARIAL_BLOCKED"),
        env!("ETUSCHLOPF_KODEX_ADVERSARIAL_TOTAL"),
        if env!("ETUSCHLOPF_KODEX_FREEZE_READY") == "1" {
            "ja"
        } else {
            "nein"
        }
    ));
    for (attack, result) in [
        ("PrivateDebugOrErrorLeak", "blocked"),
        ("AdditionalPublicSink", "blocked"),
        ("UnknownProjectionField", "blocked"),
        ("DuplicateProjection", "blocked"),
        ("RetiredDetailSection", "blocked"),
        ("PrivateValueInArtifactIdentity", "blocked"),
        ("KodexToMusicWriter", "blocked"),
        ("KodexToCalibration", "blocked"),
        ("RemovedKodexNegativeCheck", "blocked"),
        ("InconsistentTrustAnchor", "blocked"),
        ("UnknownSourcePath", "blocked"),
        ("FailedReexecRollback", "blocked"),
        ("ExternalRuntimeLocatorToMusicWriter", "blocked"),
        ("ExternalLocatorWrapperChain", "blocked"),
        ("ExternalLocatorStructField", "blocked"),
        ("ExternalLocatorClosureCapture", "blocked"),
        ("RawPublicTargetMeasure", "blocked"),
        ("NewProductiveRepairAuthority", "blocked"),
        ("NewDeadCodeSuppression", "blocked"),
        ("DeadCodeSuppressionScopeWidening", "blocked"),
        ("CompilerDeadCodeLintWeakening", "blocked"),
    ] {
        out.push_str(&format!("Attack: {attack} | Result {result}.\n"));
    }
    out.push_str("FreezePolicy: Nach gruenem realem Zielsystemlauf keine weitere abstrakte Sicherheitsversion ohne konkretes Gegenbeispiel, neue Capability-Klasse oder neue Writerfamilie.\n");
}
#[cfg(test)]
#[path = "build_gate/rust_source_scan_primitives.rs"]
pub mod rust_source_scan_primitives;

#[cfg(test)]
#[allow(dead_code)]
#[path = "build_gate/identity_debt_scan.rs"]
mod identity_debt_scan_test_support;

#[cfg(test)]
mod global_identity_debt_inventory_tests {
    use super::identity_debt_scan_test_support::scan_source_text;

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
        // Productive-scanner scope only: this does not grant normative test admission; TakeChangeControl enforces Corpus Independence separately.
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
    fn sealed_capability_breaks_the_writer_identity_dependency() {
        let source = r#"
    fn is_etude6_demo_scope(label: &str) -> bool {
        label.contains("6")
    }

    struct DemoCapability(());

    fn resolve_demo_capability(label: &str) -> Option<DemoCapability> {
        is_etude6_demo_scope(label).then_some(DemoCapability(()))
    }

    fn apply_demo_render_write(xml: &str, capability: Option<DemoCapability>) -> String {
        if capability.is_some() { xml.to_string() } else { String::new() }
    }
    "#;
        let scan = scan_source_text("src/demo.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "PROJECT_IDENTITY_SELECTOR"
                && finding.function == "is_etude6_demo_scope"
        }));
        assert!(!scan.findings.iter().any(|finding| {
            finding.kind == "PROJECT_IDENTITY_DEPENDENCY"
                && finding.function == "apply_demo_render_write"
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
    fn final_report_projects_inventory_after_compaction() {
        let owner = include_str!("pipeline_shards/final_report_projection_owner.rs");
        let compact = owner
            .find("DraftReport::new(report_preamble).compact(compact_current_report_preamble_v0101536)")
            .expect("typed compact report owner");
        let projection = owner
            .find("identity_debt_inventory::append_global_identity_debt_inventory_section,")
            .expect("identity debt report projection");
        assert!(projection > compact);
        assert_eq!(
            owner
                .matches("identity_debt_inventory::append_global_identity_debt_inventory_section,")
                .count(),
            1
        );
    }

    #[test]
    fn feedback_identity_firewall_section_is_visible() {
        let mut report = String::new();
        super::append_feedback_identity_firewall_section(&mut report);
        assert!(report.contains("FEEDBACK-IDENTITY-FIREWALL"));
        assert!(report.contains("InformationWithholding"));
        assert!(report.contains("RelocationProof"));
        assert!(report.contains("WriterOpen nein"));
        assert!(report.contains("KODEX-BOUNDARY-ADVERSARIAL-COMPLETION"));
        assert!(report.contains("AdversarialMatrix: Blocked 21/21"));
        assert_eq!(report.matches("Attack: ").count(), 21);
        assert!(report.contains("KodexFreezeReady ja"));
    }

    #[test]
    fn inventory_section_contains_closed_gate_summary() {
        let mut report = String::new();
        super::append_global_identity_debt_inventory_section(&mut report);
        assert!(report.contains("GLOBAL-IDENTITY-DEBT-INVENTORY"));
        assert!(report.contains("RegisteredIdentityDebt: 0"));
        assert!(report.contains("UnknownIdentityDebt: 0"));
        assert!(report.contains("productive_music_identity_provenance=hard_fail"));
    }

    #[test]
    fn feedback_named_writer_is_detected() {
        let source = r"
    fn apply_user_feedback_render_write(xml: &str) -> String {
        xml.to_string()
    }
    ";
        let scan = scan_source_text("src/demo.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "FEEDBACK_ORIGIN_NAMED_WRITER"
                && finding.function == "apply_user_feedback_render_write"
        }));
    }

    #[test]
    fn feedback_origin_channel_cannot_enter_writer() {
        let source = r"
    fn apply_demo_render_write(xml: &str, feedback_measure: usize) -> String {
        if feedback_measure > 0 { xml.to_string() } else { String::new() }
    }
    ";
        let scan = scan_source_text("src/demo.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "PRODUCTIVE_FEEDBACK_ORIGIN_CHANNEL"
                && finding.function == "apply_demo_render_write"
        }));
    }

    #[test]
    fn generic_structural_writer_without_origin_identity_is_allowed() {
        let source = r"
    fn apply_structural_rest_bridge_render_write(xml: &str, candidates: usize) -> String {
        if candidates == 1 { xml.to_string() } else { String::new() }
    }
    ";
        let scan = scan_source_text("src/demo.rs", source);
        assert!(scan.findings.is_empty());
    }

    #[test]
    fn audit_localization_cannot_call_mutation_authority() {
        let source = r"
    fn append_feedback_audit_section(xml: &mut String) {
        let target_measure = 42;
        if target_measure == 42 {
            apply_demo_authorized(xml);
        }
    }
    ";
        let scan = scan_source_text("src/demo.rs", source);
        assert!(scan.findings.iter().any(|finding| {
            finding.kind == "ABSOLUTE_MEASURE_DEPENDENCY"
                && finding.function == "append_feedback_audit_section"
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
}
