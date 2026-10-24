use super::*;
use crate::arrangement::OwnedRhythmicContinuation;
use crate::master_state_identity::{MasterLineage, OwnerReceipts};
use crate::structural_write_authority::MasterAdoptedState;

fn received_master(rows: &[(usize, i64, u8)], measures: usize)
    -> (MasterAdoptedState, OwnedRhythmicContinuation) {
    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture(rows, measures);
    let arrangement = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        crate::arrangement::ArrangementProfile { rhythmic_courses: Some(&seed),
            preserve_reference: true, ..Default::default() });
    let state = OwnedRhythmicContinuation::receive(
        arrangement.stats.rhythmic_courses.as_ref().unwrap(), &arrangement.xml).unwrap();
    let mut master = MasterAdoptedState::adopt_genesis_state(arrangement.xml,
        MasterLineage::lineage_from_genesis(arrangement.arrangement_source_genesis.unwrap()));
    let before = master.document().to_owned();
    let normalized = crate::arrangement::normalize_visible_rhythm_musicxml(&before);
    let mut receipts = OwnerReceipts::default();
    receipts.push_visible_block_with_births(normalized.visible_rhythm_block_identity,
        normalized.stats.physical_births.first().cloned(), &before).unwrap();
    let state = receipts.continue_owned_rhythm(&state, &normalized.xml).unwrap();
    master.adopt_materialized_document(normalized.xml, receipts, "SyntheticOriginalNormalizer");
    state.require_complete_course_context(master.document()).unwrap();
    master.observe_selected_custody("SyntheticEndpointCleanup", Some(std::sync::Arc::new(state.clone())), None);
    (master, state)
}

fn hidden_tied_master(rows: &[(usize, i64, u8)], measures: usize)
    -> (MasterAdoptedState, OwnedRhythmicContinuation) {
    let (mut master, state) = received_master(rows, measures);
    let before = master.document().to_owned();
    let doc = roxmltree::Document::parse(&before).unwrap();
    let range = doc.descendants().find(|node| node.has_tag_name("note")
        && node.children().any(|child| child.has_tag_name("pitch"))
        && node.descendants().any(|child| child.has_tag_name("tied")))
        .expect("normalizer fixture must contain a real tied fragment").range();
    let mut after = before.clone();
    let (hidden, receipt) = hide_note_print_object_range(&mut after, range.start, range.end);
    assert!(hidden);
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let state = receipts.continue_owned_rhythm(&state, &after).unwrap();
    master.adopt_materialized_document(after, receipts, "SyntheticOriginalVisibilityOwner");
    (master, state)
}

fn sounding_ties(xml: &str) -> Vec<String> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    doc.descendants().filter(|node| node.has_tag_name("tie"))
        .map(|node| xml[node.range()].to_owned()).collect()
}

#[test]
fn actual_orphan_cleanup_reaches_original_master_adoption_with_retained_custody() {
    for rows in [vec![(7, 0, 1)], vec![(91, 0, 1), (53, 384, 2)]] {
        let (mut master, state) = hidden_tied_master(&rows, 2);
        let before = master.document().to_owned();
        let mut after = before.clone();
        let ((_, removed_tied), receipts) = remove_orphan_slur_and_tie_endpoints(&mut after);
        assert!(removed_tied > 0, "actual orphan detector must reach the original writer");
        assert!(receipts.len() >= 2, "endpoint deletion and empty-wrapper removal are separate original receipts");
        assert_ne!(after, before);
        assert_eq!(sounding_ties(&after), sounding_ties(&before));
        let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
        assert!(next.same_identities_as(&state));
        next.require_complete_course_context(&after).unwrap();
        master.adopt_materialized_document(after.clone(), receipts, "SyntheticActualOrphanCleanup");
        assert_eq!(master.document(), after);
        assert!(master.lineage().lineage_continuity_holds(&after));
        let (counts, noop) = remove_orphan_slur_and_tie_endpoints(&mut after);
        assert_eq!(counts, (0, 0)); assert_eq!(noop.len(), 0);
        assert!(noop.continue_owned_rhythm(&next, &after).unwrap().same_identities_as(&next));
    }
}

#[test]
fn valid_cleanup_custody_never_replaces_dropped_original_master_receipts() {
    let (mut master, state) = hidden_tied_master(&[(17, 0, 1)], 1);
    let before = master.document().to_owned(); let mut after = before.clone();
    let ((_, removed), receipts) = remove_orphan_slur_and_tie_endpoints(&mut after);
    assert!(removed > 0);
    receipts.continue_owned_rhythm(&state, &after).unwrap();
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        master.adopt_materialized_document(after, OwnerReceipts::default(), "SyntheticDroppedEndpointReceipts");
    }));
    assert!(refusal.is_err()); assert_eq!(master.document(), before);
    assert!(master.lineage().lineage_continuity_holds(&before));
}

#[test]
fn actual_endpoint_cleanup_noop_preserves_empty_course_context() {
    let (mut master, state) = received_master(&[], 2);
    let before = master.document().to_owned(); let mut after = before.clone();
    let (counts, receipts) = remove_orphan_slur_and_tie_endpoints(&mut after);
    assert_eq!(counts, (0, 0)); assert_eq!(after, before);
    let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
    assert!(next.same_identities_as(&state)); next.require_complete_course_context(&after).unwrap();
    master.adopt_materialized_document(after, receipts, "SyntheticEmptyEndpointCleanup");
    assert!(master.lineage().lineage_continuity_holds(&before));
}
