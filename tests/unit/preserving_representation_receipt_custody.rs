use super::*;
use crate::arrangement::OwnedRhythmicContinuation;
use crate::master_state_identity::OwnerReceipts;

fn received(rows: &[(usize, i64, u8, i32)], measures: usize) -> OwnedRhythmicContinuation {
    OwnedRhythmicContinuation::synthetic_pitched_received_for_test(rows, measures)
}

fn owners(receipt: Option<PreservingTransformationReceipt>) -> OwnerReceipts {
    let mut receipts = OwnerReceipts::default(); receipts.push_preserving(receipt); receipts
}

#[test]
fn original_polish_receipt_carries_multistep_beam_and_clef_journals_to_adoption() {
    for offset in [0, 384] {
        let state = received(&[(71, offset, 2, 84), (72, offset, 2, 84),
            (73, offset + 24, 2, 86), (74, offset + 48, 2, 88)], 3);
        let result = polish_musicxml(state.document()).unwrap();
        assert_ne!(result.xml, state.document());
        assert!(result.stats.beamed_notes > 0);
        assert!(result.xml.contains("etuschlopf-ledger-bound-clef-readback"));
        let receipts = owners(result.preserving_receipt);
        let next = receipts.continue_owned_rhythm(&state, &result.xml).unwrap();
        next.require_complete_course_context(&result.xml).unwrap();
        assert!(next.same_identities_as(&state));
        assert!(receipts.continue_owned_rhythm(&next, &result.xml).is_err());
    }
}

#[test]
fn missing_polish_journal_and_foreign_final_bytes_remain_unknown() {
    let state = received(&[(71, 0, 1, 60), (72, 24, 1, 64), (73, 48, 1, 67)], 1);
    let result = polish_musicxml(state.document()).unwrap();
    assert_ne!(result.xml, state.document());
    let missing = owners(result.preserving_receipt.map(|r| r.without_recorded_representation_for_test()));
    assert!(missing.continue_owned_rhythm(&state, &result.xml).unwrap_err().to_string()
        .contains("Unsupported course continuation owner boundary"));
    let result = polish_musicxml(state.document()).unwrap();
    let receipts = owners(result.preserving_receipt);
    assert!(receipts.continue_owned_rhythm(&state, &format!("{} ", result.xml)).is_err());
    let foreign = received(&[(71, 0, 1, 61), (72, 24, 1, 65), (73, 48, 1, 68)], 1);
    assert!(receipts.continue_owned_rhythm(&foreign, &result.xml).is_err());
}

#[test]
fn original_partial_beat_owner_carries_duplicate_independent_occurrences() {
    for rows in [vec![(71, 24, 1, 60), (72, 24, 1, 60), (73, 48, 1, 64), (74, 96, 1, 67)],
        vec![(71, 24, 1, 60), (72, 24, 1, 60), (73, 48, 1, 64), (74, 96, 1, 67),
            (81, 408, 2, 48), (82, 408, 2, 48), (83, 432, 2, 52), (84, 480, 2, 55)]] {
        let state = received(&rows, 2);
        let mut after = state.document().to_owned();
        let result = enforce_partial_beat_post_rest_beams_in_xml(&mut after);
        assert!(result.groups > 0 && result.preserving_receipt.is_some());
        let receipts = owners(result.preserving_receipt);
        let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
        next.require_complete_course_context(&after).unwrap();
        assert!(next.same_identities_as(&state));
        let mut again = after.clone();
        let repeated = enforce_partial_beat_post_rest_beams_in_xml(&mut again);
        assert_eq!(again, after); assert!(repeated.preserving_receipt.is_none());
    }
}

#[test]
fn ledger_clef_receipt_preserves_owned_history_and_requires_its_capture() {
    let state = received(&[(71, 0, 2, 84), (72, 24, 2, 86), (73, 48, 2, 88)], 1);
    let (after, receipt) = notation::ledger_bounded_lower_staff_clef_readback(
        state.document(), notation::LowerStaffClefReadbackScope::BeamSafety);
    assert_ne!(after, state.document());
    let next = owners(receipt).continue_owned_rhythm(&state, &after).unwrap();
    assert!(next.same_identities_as(&state)); next.require_complete_course_context(&after).unwrap();
    let (_, receipt) = notation::ledger_bounded_lower_staff_clef_readback(
        state.document(), notation::LowerStaffClefReadbackScope::BeamSafety);
    assert!(owners(receipt.map(|r| r.without_recorded_representation_for_test()))
        .continue_owned_rhythm(&state, &after).is_err());
}

#[test]
fn original_layout_deletion_journal_binds_its_receipt_without_inventing_custody() {
    let state = received(&[(71, 0, 1, 60)], 2);
    let before = state.document().replacen("<attributes>", "<print new-page=\"yes\"/><attributes>", 1);
    let layout = crate::layout::apply_pianist_layout(&before, 2);
    assert_ne!(layout.xml, before);
    let (original_pre, original_post) = layout.rhythmic_transport.recorded_documents().unwrap();
    layout.preserving_receipt.as_ref().unwrap().binds_recorded_representation(original_pre, original_post).unwrap();
    // The independently edited document is not rebound to the received origins.
    assert!(owners(layout.preserving_receipt).continue_owned_rhythm(&state, &layout.xml).is_err());
}

// Test-only access to the real private pipeline owner, with no product API.
pub(crate) fn original_polish_document_and_receipt(xml: &str) -> (String, Option<PreservingTransformationReceipt>) {
    let result = polish_musicxml(xml).unwrap(); (result.xml, result.preserving_receipt)
}

#[test]
fn e053_beat_scoped_owner_continues_selected_course_custody() {
    for (copies, offset) in [(1usize, 0usize), (2, 0), (1, 384), (2, 384)] {
        let rows = (0..copies).flat_map(|copy| (0..16).map(move |i|
            (100 + offset + copy * 100 + i, offset as i64 + copy as i64 * 384 + i as i64 * 24,
             1, 72 + (i % 4) as i32))).collect::<Vec<_>>();
        let state = received(&rows, copies + offset / 384);
        let mut after = state.document().to_owned();
        let input = enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after);
        assert_ne!(after, state.document(), "fixture must make BLOCK #53 write");
        let mut receipts = OwnerReceipts::default();
        receipts.push_block(input);
        let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
        next.require_complete_course_context(&after).unwrap();
        assert!(next.same_identities_as(&state));
        assert!(receipts.continue_owned_rhythm(&state, &format!("{after} ")).is_err());
    }
}

#[test]
fn e053_without_its_emitter_carry_stays_unknown() {
    let rows = (0..16).map(|i| (100 + i, i as i64 * 24, 1, 72 + (i % 4) as i32)).collect::<Vec<_>>();
    let state = received(&rows, 1);
    let mut after = state.document().to_owned();
    let (receipt, _) = enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after).into_parts_for_test();
    let mut receipts = OwnerReceipts::default();
    receipts.push_block(crate::master_state_identity::CapturedBlockReceiptInput::from(receipt));
    assert!(receipts.continue_owned_rhythm(&state, &after).unwrap_err().to_string()
        .contains("Unsupported course continuation BLOCK owner"));
}

#[test]
fn e024_clef_course_continues_selected_course_custody() {
    for (copies, offset) in [(1usize, 0usize), (2, 0), (1, 384), (2, 384)] {
        let rows = (0..copies).flat_map(|copy| (0..18).map(move |i|
            (200 + offset + copy * 100 + i, offset as i64 + copy as i64 * 1152 + i as i64 * 64,
             2, 83))).collect::<Vec<_>>();
        let state = received(&rows, copies * 3 + offset / 384);
        let mut after = state.document().to_owned();
        let (result, receipt) = crate::structural_write_authority::apply_rustig_clef_course_to_buffer(&mut after);
        result.unwrap();
        assert_ne!(after, state.document(), "fixture must materialize a clef course");
        let next = owners(receipt).continue_owned_rhythm(&state, &after).unwrap();
        next.require_complete_course_context(&after).unwrap();
        assert!(next.same_identities_as(&state));
        let mut again = state.document().to_owned();
        let (_, replay) = crate::structural_write_authority::apply_rustig_clef_course_to_buffer(&mut again);
        assert_eq!(again, after);
        let missing = owners(replay.map(|r| r.without_recorded_representation_for_test()));
        assert!(missing.continue_owned_rhythm(&state, &after).is_err());
    }
}
