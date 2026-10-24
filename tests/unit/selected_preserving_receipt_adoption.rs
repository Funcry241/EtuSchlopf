use super::*;
use crate::master_state_identity::{MasterLineage, OwnerReceipts};

#[test]
fn actual_final_layout_receipt_reaches_selected_master_adoption_and_keeps_noop() {
    use crate::structural_write_authority as authority;
    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_pitched_fixture(
        &[(1701, 0, 1, 72), (1702, 96, 1, 74), (1703, 384, 2, 48)], 2);
    let arranged = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        crate::arrangement::ArrangementProfile { rhythmic_courses: Some(&seed),
            preserve_reference: true, ..Default::default() });
    let custody = crate::arrangement::OwnedRhythmicContinuation::receive(
        arranged.stats.rhythmic_courses.as_ref().unwrap(), &arranged.xml).unwrap();
    let mut master = MasterAdoptedState::adopt_genesis_state(arranged.xml,
        MasterLineage::lineage_from_genesis(arranged.arrangement_source_genesis.unwrap()));
    master.observe_selected_custody("SyntheticOriginalE007", Some(std::sync::Arc::new(custody.clone())), None);
    let doc = roxmltree::Document::parse(master.document()).unwrap();
    let directives = doc.descendants().filter(|node| node.has_tag_name("measure"))
        .map(|_| authority::HorizontalLayoutMaterializationDirective {
            explicit_width: Some("910".to_owned()),
            system_break: authority::HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }).collect::<Vec<_>>();
    let path = std::env::temp_dir().join(format!("actual-e007-master-adoption-{}.musicxml", std::process::id()));
    std::fs::write(&path, master.document()).unwrap();
    let (written, receipt) = authority::materialize_final_render_notation_with_phrase_projection_authorized(
        &path, &directives, &[]);
    assert!(written.unwrap() > 0); assert!(receipt.is_some());
    let before = master.document().to_owned();
    master.adopt_path_bound_preserving_transition(receipt, "SyntheticOriginalE007Adoption");
    assert_ne!(master.document(), before);
    let selected = master.selected_custody.as_ref().unwrap();
    assert!(selected.unknown.is_none(), "{:?}", selected.unknown);
    let next = selected.bound.as_ref().unwrap();
    next.require_complete_course_context(master.document()).unwrap();
    assert!(next.same_identities_as(&custody));
    assert!(master.lineage().lineage_continuity_holds(master.document()));
    let head = master.document().to_owned();
    let (written, receipt) = authority::materialize_final_render_notation_with_phrase_projection_authorized(
        &path, &directives, &[]);
    assert_eq!(written.unwrap(), 0); assert!(receipt.is_none());
    master.adopt_path_bound_preserving_transition(receipt, "SyntheticOriginalE007Noop");
    assert_eq!(master.document(), head);
    assert!(master.selected_custody.as_ref().unwrap().unknown.is_none());
    assert!(master.lineage().lineage_continuity_holds(master.document()));
    std::fs::remove_file(path).unwrap();
}
    #[cfg(test)]
    #[test]
    fn actual_preserving_journal_reaches_selected_master_custody_without_replacement_receipts() {
        let rows = [(71, 0, 2, 84), (72, 0, 2, 84), (73, 24, 2, 86), (74, 48, 2, 88)];
        let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_pitched_fixture(&rows, 2);
        let arranged = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
            crate::arrangement::ArrangementProfile { rhythmic_courses: Some(&seed),
                preserve_reference: true, ..Default::default() });
        let custody = crate::arrangement::OwnedRhythmicContinuation::receive(
            arranged.stats.rhythmic_courses.as_ref().unwrap(), &arranged.xml).unwrap();
        let mut master = MasterAdoptedState::adopt_genesis_state(arranged.xml.clone(),
            MasterLineage::lineage_from_genesis(arranged.arrangement_source_genesis.unwrap()));
        master.observe_selected_custody("SyntheticPreservingJournal", Some(std::sync::Arc::new(custody.clone())), None);
        let (post, receipt) = crate::pipeline::preserving_representation_receipt_custody_tests::original_polish_document_and_receipt(&arranged.xml);
        assert_ne!(post, arranged.xml);
        let mut receipts = OwnerReceipts::default(); receipts.push_preserving(receipt);
        master.adopt_materialized_document(post.clone(), receipts, "SyntheticPolishAdoption");
        let carried = master.selected_custody.as_ref().unwrap();
        assert!(carried.unknown.is_none(), "{:?}", carried.unknown);
        let next = carried.bound.as_ref().unwrap();
        next.require_complete_course_context(master.document()).unwrap();
        assert!(next.same_identities_as(&custody));
        assert!(master.lineage().lineage_continuity_holds(master.document()));
    }
