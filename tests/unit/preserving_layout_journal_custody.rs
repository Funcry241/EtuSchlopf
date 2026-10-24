use super::*;
use crate::master_state_identity::OwnerReceipts;
use crate::structural_write_authority::{ensure_local_staff_distance_authorized, ensure_visual_layout_defaults_authorized};

fn received(rows: &[(usize, i64, u8)], measures: usize) -> OwnedRhythmicContinuation {
    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture(rows, measures);
    let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    OwnedRhythmicContinuation::receive(&custody, &xml).unwrap()
}

fn defaults(state: &OwnedRhythmicContinuation) -> (String, OwnerReceipts) {
    let mut post = state.document().to_string();
    let (written, receipt) = ensure_visual_layout_defaults_authorized(&mut post, 90, 70);
    assert!(written.unwrap());
    let mut receipts = OwnerReceipts::default();
    receipts.push_preserving(receipt);
    (post, receipts)
}

fn distance(state: &OwnedRhythmicContinuation, nth: usize, value: usize) -> (String, OwnerReceipts) {
    let mut post = state.document().to_string();
    let start = post.match_indices("<measure ").nth(nth).unwrap().0;
    let (written, receipt) = ensure_local_staff_distance_authorized(&mut post, start, value);
    assert!(written.unwrap());
    let mut receipts = OwnerReceipts::default();
    receipts.push_preserving(receipt);
    (post, receipts)
}

#[test]
fn recorded_layout_writes_carry_equal_coincident_notes() {
    let state = received(&[(7, 0, 1), (8, 0, 1), (9, 0, 2)], 2);
    let (first, receipts) = defaults(&state);
    let a = receipts.continue_owned_rhythm(&state, &first).unwrap();
    a.require_complete_course_context(&first).unwrap();
    let (second, receipts) = distance(&a, 1, 120);
    let b = receipts.continue_owned_rhythm(&a, &second).unwrap();
    let (third, receipts) = distance(&b, 1, 160);
    let c = receipts.continue_owned_rhythm(&b, &third).unwrap();
    c.require_complete_course_context(&third).unwrap();
    assert!(c.same_identities_as(&state));
    let coincident = c.occurrences.iter().filter(|o| o.origin.source.is_some() && o.staff == 1).collect::<Vec<_>>();
    assert_ne!(coincident[0].creation, coincident[1].creation);
}

#[test]
fn layout_journal_is_bound_to_its_original_state() {
    let state = received(&[(7, 0, 1)], 1);
    let (post, receipts) = defaults(&state);
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    assert!(receipts.continue_owned_rhythm(&next, &post).is_err());
}

#[test]
fn unjournaled_preserving_receipt_keeps_the_unsupported_boundary() {
    let state = received(&[(7, 0, 1)], 1);
    let mut post = state.document().to_string();
    let (_, receipt) = ensure_visual_layout_defaults_authorized(&mut post, 90, 70);
    let mut receipts = OwnerReceipts::default();
    receipts.push_preserving(receipt.map(|receipt| receipt.without_layout_journal_for_test()));
    let error = receipts.continue_owned_rhythm(&state, &post).unwrap_err().to_string();
    assert!(error.contains("Unsupported course continuation owner boundary"), "{error}");
}

#[test]
fn layout_journal_that_rewrites_a_note_is_refused_even_when_it_replays() {
    let state = received(&[(7, 0, 1)], 1);
    let before = state.document().to_string();
    let mut after = before.clone();
    let (_, receipt) = ensure_visual_layout_defaults_authorized(&mut after, 90, 70);
    let receipt = receipt.unwrap().without_layout_journal_for_test();
    let insertion = after.len() - before.len();
    let offset = (0..before.len()).find(|&i| before.as_bytes()[i] != after.as_bytes()[i]).unwrap();
    let note_start = before.find("<note").unwrap();
    let note_end = note_start + before[note_start..].find("</note>").unwrap() + "</note>".len();
    assert!(offset < note_start);
    let edits = vec![(offset..offset, after[offset..offset + insertion].to_string()),
        (note_start..note_end, before[note_start..note_end].to_string())];
    let step = crate::arrangement::representation_transport::RepresentationStep::recorded(&before, &after, edits);
    let error = crate::arrangement::representation_transport::RhythmicRepresentationTransport::preserving_layout(step, &receipt)
        .unwrap_err().to_string();
    assert!(error.contains("rhythmic element"), "{error}");
}
