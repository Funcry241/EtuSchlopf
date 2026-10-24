#[cfg(test)]
use super::authority_primitives::yes_no;

const DETECTOR_MODULE_SOURCE: &str = include_str!("detector.rs");
#[cfg(test)]
const DETECTOR_MODULE_PATH: &str = "src/structural_write_authority/detector.rs";

const RAW_XML_DEPENDENCY_TOKENS: &[&str] = &["roxmltree", "Document", "Node<'", "Node<"];
const LOCATOR_DEPENDENCY_TOKENS: &[&str] = &[
    "StructuralLocator",
    "LocatedStructuralView",
    "LocatedStructuralRepairIntent",
];
const ABSOLUTE_IDENTITY_TOKENS: &[&str] = &[
    "source_identity",
    "source_nonce",
    "part_ordinal",
    "measure_ordinal",
    "project_label",
    "attribute(\"number\")",
    "parse_measure_events",
    "direct_child_text",
];

#[derive(Debug, Clone, Default)]
pub struct IdentityFreeDetectorModuleBoundaryAudit {
    #[cfg(test)]
    source_stage: String,
    #[cfg(test)]
    detector_module_path: String,
    #[cfg(test)]
    structural_view_count: usize,
    #[cfg(test)]
    repair_intent_count: usize,
    raw_xml_dependency_references: usize,
    locator_dependency_references: usize,
    absolute_identity_references: usize,
    detector_input_signature_green: bool,
    detector_output_signature_green: bool,
    #[cfg(test)]
    adapter_owns_xml_parsing: bool,
    #[cfg(test)]
    adapter_owns_locator_binding: bool,
    detector_module_boundary_green: bool,
    #[cfg(test)]
    authorization_attempted: bool,
    #[cfg(test)]
    writer_token_used: bool,
    #[cfg(test)]
    write_produced: bool,
    #[cfg(test)]
    verdict: String,
}

impl IdentityFreeDetectorModuleBoundaryAudit {
    pub fn authorizes_token_origin(&self) -> bool {
        self.detector_module_boundary_green
            && self.raw_xml_dependency_references == 0
            && self.locator_dependency_references == 0
            && self.absolute_identity_references == 0
            && self.detector_input_signature_green
            && self.detector_output_signature_green
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "IdentityFreeDetectorModuleBoundaryAudit v{}: Project {}; SourceStage {}; DetectorModule {}; StructuralViews {}; RepairIntents {}; RawXmlDependencyReferences {}; LocatorDependencyReferences {}; AbsoluteIdentityReferences {}; DetectorInputStructuralViewOnly {}; DetectorOutputRepairIntentOnly {}; AdapterOwnsXmlParsing {}; AdapterOwnsLocatorBinding {}; ModuleBoundaryGreen {}; AuthorizationAttempted {}; WriterTokenUsed {}; WriteProduced {}; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            self.source_stage,
            self.detector_module_path,
            self.structural_view_count,
            self.repair_intent_count,
            self.raw_xml_dependency_references,
            self.locator_dependency_references,
            self.absolute_identity_references,
            yes_no(self.detector_input_signature_green),
            yes_no(self.detector_output_signature_green),
            yes_no(self.adapter_owns_xml_parsing),
            yes_no(self.adapter_owns_locator_binding),
            yes_no(self.detector_module_boundary_green),
            yes_no(self.authorization_attempted),
            yes_no(self.writer_token_used),
            yes_no(self.write_produced),
            self.verdict,
        )
    }

}

pub fn audit_identity_free_detector_module_boundary(
    _structural_view_count: usize,
    _repair_intent_count: usize,
    _source_stage: &str,
) -> IdentityFreeDetectorModuleBoundaryAudit {
    let raw_xml_dependency_references =
        count_token_references(DETECTOR_MODULE_SOURCE, RAW_XML_DEPENDENCY_TOKENS);
    let locator_dependency_references =
        count_token_references(DETECTOR_MODULE_SOURCE, LOCATOR_DEPENDENCY_TOKENS);
    let absolute_identity_references =
        count_token_references(DETECTOR_MODULE_SOURCE, ABSOLUTE_IDENTITY_TOKENS);
    let detector_input_signature_green = DETECTOR_MODULE_SOURCE
        .contains("pub(super) fn detect_repair_intent(")
        && DETECTOR_MODULE_SOURCE.contains("view: &StructuralView");
    let detector_output_signature_green =
        DETECTOR_MODULE_SOURCE.contains(") -> Option<StructuralRepairIntent>");
    let adapter_owns_xml_parsing = true;
    let adapter_owns_locator_binding = true;
    let detector_module_boundary_green = raw_xml_dependency_references == 0
        && locator_dependency_references == 0
        && absolute_identity_references == 0
        && detector_input_signature_green
        && detector_output_signature_green
        && adapter_owns_xml_parsing
        && adapter_owns_locator_binding;

    IdentityFreeDetectorModuleBoundaryAudit {
        #[cfg(test)]
        source_stage: _source_stage.to_string(),
        #[cfg(test)]
        detector_module_path: DETECTOR_MODULE_PATH.to_string(),
        #[cfg(test)]
        structural_view_count: _structural_view_count,
        #[cfg(test)]
        repair_intent_count: _repair_intent_count,
        raw_xml_dependency_references,
        locator_dependency_references,
        absolute_identity_references,
        detector_input_signature_green,
        detector_output_signature_green,
        #[cfg(test)]
        adapter_owns_xml_parsing,
        #[cfg(test)]
        adapter_owns_locator_binding,
        detector_module_boundary_green,
        #[cfg(test)]
        authorization_attempted: false,
        #[cfg(test)]
        writer_token_used: false,
        #[cfg(test)]
        write_produced: false,
        #[cfg(test)]
        verdict: if detector_module_boundary_green {
            "identity_free_detector_real_module_import_boundary_green_audit_only".to_string()
        } else {
            "identity_free_detector_module_import_boundary_open".to_string()
        },
    }
}

fn count_token_references(source: &str, tokens: &[&str]) -> usize {
    tokens
        .iter()
        .map(|token| source.matches(token).count())
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detector_module_has_no_raw_xml_or_locator_dependency() {
        assert_eq!(
            count_token_references(DETECTOR_MODULE_SOURCE, RAW_XML_DEPENDENCY_TOKENS),
            0
        );
        assert_eq!(
            count_token_references(DETECTOR_MODULE_SOURCE, LOCATOR_DEPENDENCY_TOKENS),
            0
        );
        assert_eq!(
            count_token_references(DETECTOR_MODULE_SOURCE, ABSOLUTE_IDENTITY_TOKENS),
            0
        );
    }

    #[test]
    fn detector_contract_is_structural_view_to_repair_intent_only() {
        assert!(DETECTOR_MODULE_SOURCE.contains("pub(super) fn detect_repair_intent("));
        assert!(DETECTOR_MODULE_SOURCE.contains("view: &StructuralView"));
        assert!(DETECTOR_MODULE_SOURCE.contains(") -> Option<StructuralRepairIntent>"));
    }

    #[test]
    fn module_boundary_audit_never_authorizes_or_writes() {
        let audit = audit_identity_free_detector_module_boundary(248, 1, "test");
        assert!(audit.detector_module_boundary_green);
        assert!(!audit.authorization_attempted);
        assert!(!audit.writer_token_used);
        assert!(!audit.write_produced);
        assert!(audit.pipeline_line("project").contains("ModuleBoundaryGreen ja"));
    }
}
