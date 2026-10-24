use super::*;
use crate::midi_seed::rhythmic_course_creation_tests::{arrangement_fixture, arrangement_pitched_fixture};
use crate::master_state_identity::OwnerReceipts;
use crate::structural_write_authority::{apply_chord_gesture_distribution_batch_authorized, hide_note_print_object_authorized};

/// Receives custody and hides the note of `hidden_source` through the actual row 5
/// owner, because the row 26 owner only relocates hidden notes.
fn received(rows: &[(usize, i64, u8)], hidden_source: usize) -> OwnedRhythmicContinuation {
    let (xml, seed) = arrangement_fixture(rows, 1);
    let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    let state = OwnedRhythmicContinuation::receive(&custody, &xml).unwrap();
    let note = by_source(&state, hidden_source);
    let mut doc = state.document().to_string();
    let (hidden, receipt) = hide_note_print_object_authorized(&mut doc, note.bytes.start, note.bytes.end);
    assert!(hidden.unwrap());
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    receipts.continue_owned_rhythm(&state, &doc).unwrap()
}

fn by_source(state: &OwnedRhythmicContinuation, source: usize) -> Occurrence {
    state.occurrences.iter().find(|o| o.origin.source == Some(source)).unwrap().clone()
}

/// The chord-member clone the ConsensusGesture producer writes: the moved pitch with
/// the anchor's duration, voice and staff.
fn chord_clone(state: &OwnedRhythmicContinuation, moved: &Occurrence, anchor: &Occurrence, pitch: Option<&str>) -> String {
    let moved_text = &state.snapshot[moved.bytes.clone()];
    let anchor_text = &state.snapshot[anchor.bytes.clone()];
    let original = &moved_text[moved_text.find("<pitch>").unwrap()..moved_text.find("</pitch>").unwrap() + "</pitch>".len()];
    format!("<note><chord/>{}<duration>{}</duration><voice>{}</voice><staff>{}</staff></note>",
        pitch.unwrap_or(original), extract_first_i64(anchor_text, "duration").unwrap(),
        extract_first_text(anchor_text, "voice").unwrap(), extract_first_i64(anchor_text, "staff").unwrap())
}

fn relocate(state: &OwnedRhythmicContinuation, moved: &Occurrence, anchor: &Occurrence, clone: String)
    -> (String, OwnerReceipts) {
    let (post, receipt) = apply_chord_gesture_distribution_batch_authorized(state.document(),
        vec![(moved.bytes.start, moved.bytes.end, String::new()), (anchor.bytes.end, anchor.bytes.end, clone)]);
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    (post.unwrap().0, receipts)
}

#[test]
fn captured_relocation_joins_anchor_course_and_keeps_source() {
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let (post, receipts) = relocate(&state, &moved, &anchor, chord_clone(&state, &moved, &anchor, None));
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    next.require_complete_course_context(&post).unwrap();
    let child = next.occurrences.iter().find(|o| o.origin.parent == Some(moved.creation)).unwrap();
    assert_eq!(child.origin.source, Some(11));
    assert_eq!(child.origin.root, moved.origin.root);
    assert_eq!((child.origin.course, child.origin.presentation), (anchor.origin.course, anchor.origin.presentation));
    assert_ne!(child.origin.course, moved.origin.course);
    assert_eq!(child.origin.interval, anchor.origin.interval);
    let relocation = child.origin.relocation.as_ref().unwrap();
    assert_eq!((relocation.from_course, relocation.anchor), (moved.origin.course, anchor.creation));
    assert_eq!(relocation.vacated, moved.origin.interval);
    assert_eq!(next.relocation_vacancy_count(), 0);
}

#[test]
fn vacated_span_is_measured_against_former_course_material_only() {
    // The remaining chord head of the former course represents the vacated span.
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let (post, receipts) = relocate(&state, &moved, &anchor, chord_clone(&state, &moved, &anchor, None));
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    assert_eq!(next.relocation_vacancy_count(), 0);
    assert_eq!(collect_raw_notes(&post).len(), collect_raw_notes(state.document()).len());
    // Without former course material the same span is counted, never filled.
    let mut without_head = next.clone();
    let head = by_source(&next, 13).creation;
    without_head.occurrences.retain(|o| o.creation != head);
    assert_eq!(without_head.relocation_vacancy_count(), 1);
}

#[test]
fn missing_capture_stays_unknown_without_refusing_the_owner() {
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let (post, receipt) = apply_chord_gesture_distribution_batch_authorized(state.document(),
        vec![(moved.bytes.start, moved.bytes.end, String::new()),
            (anchor.bytes.end, anchor.bytes.end, chord_clone(&state, &moved, &anchor, None))]);
    let mut receipt = receipt.unwrap();
    assert!(receipt.take_physical_births().is_some());
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(Some(receipt));
    let error = receipts.continue_owned_rhythm(&state, &post.unwrap().0).unwrap_err().to_string();
    assert!(error.contains("Missing actual creator context capture"));
}

#[test]
fn other_edit_shapes_produce_no_capture() {
    // The owner accepts the same edits in another order; the capture only reads
    // strict removal/insertion pairs and therefore stays absent (custody Unknown).
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let clone = chord_clone(&state, &moved, &anchor, None);
    let (post, receipt) = apply_chord_gesture_distribution_batch_authorized(state.document(),
        vec![(anchor.bytes.end, anchor.bytes.end, clone), (moved.bytes.start, moved.bytes.end, String::new())]);
    let mut receipt = receipt.unwrap();
    assert!(receipt.take_physical_births().is_none());
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(Some(receipt));
    assert!(receipts.continue_owned_rhythm(&state, &post.unwrap().0).is_err());
}

#[test]
fn clone_outside_anchor_support_or_without_chord_marker_is_not_continued() {
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let longer = chord_clone(&state, &moved, &anchor, None).replace("<duration>", "<duration>1");
    let (post, receipts) = relocate(&state, &moved, &anchor, longer);
    assert!(receipts.continue_owned_rhythm(&state, &post).unwrap_err().to_string().contains("chord member of its anchor"));
    let plain = chord_clone(&state, &moved, &anchor, None).replace("<chord/>", "");
    let (post, receipt) = apply_chord_gesture_distribution_batch_authorized(state.document(),
        vec![(moved.bytes.start, moved.bytes.end, String::new()), (anchor.bytes.end, anchor.bytes.end, plain)]);
    assert!(post.is_none() && receipt.is_none());
}

#[test]
fn course_change_without_relocation_edge_is_rejected() {
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let (post, receipts) = relocate(&state, &moved, &anchor, chord_clone(&state, &moved, &anchor, None));
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    let mut forged = next.clone();
    let index = forged.occurrences.iter().position(|o| o.origin.parent == Some(moved.creation)).unwrap();
    let creation = forged.occurrences[index].creation;
    forged.occurrences[index].origin.relocation = None;
    forged.history.get_mut(&creation).unwrap().relocation = None;
    assert!(forged.validate_binding(&post).unwrap_err().to_string().contains("changed course"));
}

#[test]
fn sounding_pitch_decides_and_spelling_does_not() {
    assert_eq!(sounding_midi("<pitch><step>B</step><alter>1</alter><octave>3</octave></pitch>"),
        sounding_midi("<pitch><step>C</step><octave>4</octave></pitch>"));
    assert_eq!(sounding_midi("<pitch><step>C</step><alter>-1</alter><octave>4</octave></pitch>"),
        sounding_midi("<pitch><step>B</step><octave>3</octave></pitch>"));
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let moved_text = state.snapshot[moved.bytes.clone()].to_string();
    let midi = sounding_midi(&moved_text).unwrap();
    let other = format!("<pitch><step>C</step><octave>{}</octave></pitch>", (midi + 12) / 12 + 1);
    let (post, receipt) = apply_chord_gesture_distribution_batch_authorized(state.document(),
        vec![(moved.bytes.start, moved.bytes.end, String::new()),
            (anchor.bytes.end, anchor.bytes.end, chord_clone(&state, &moved, &anchor, Some(&other)))]);
    assert!(post.is_none() && receipt.is_none());
}

#[test]
fn equal_executions_receive_distinct_identities() {
    let state = received(&[(11, 0, 1), (13, 0, 1), (21, 0, 2)], 11);
    let (moved, anchor) = (by_source(&state, 11), by_source(&state, 21));
    let run = || {
        let (post, receipts) = relocate(&state, &moved, &anchor, chord_clone(&state, &moved, &anchor, None));
        receipts.continue_owned_rhythm(&state, &post).unwrap()
    };
    let (first, second) = (run(), run());
    assert_eq!(first.document(), second.document());
    assert!(!first.same_identities_as(&second));
}


fn shared_anchor_received(second_midi: i32) -> OwnedRhythmicContinuation {
    let (xml, seed) = arrangement_pitched_fixture(&[(11, 0, 1, 64), (13, 0, 1, second_midi),
        (15, 0, 1, 72), (21, 0, 2, 48)], 1);
    let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    let mut state = OwnedRhythmicContinuation::receive(&custody, &xml).unwrap();
    for source in [11, 13] {
        let note = by_source(&state, source);
        let mut doc = state.document().to_string();
        let (hidden, receipt) = hide_note_print_object_authorized(&mut doc, note.bytes.start, note.bytes.end);
        assert!(hidden.unwrap());
        let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
        state = receipts.continue_owned_rhythm(&state, &doc).unwrap();
    }
    state
}

fn assert_shared_anchor_capture(second_midi: i32) {
    let state = shared_anchor_received(second_midi);
    let anchor = by_source(&state, 21);
    let first = by_source(&state, 11);
    let second = by_source(&state, 13);
    let mut edits = Vec::new();
    for moved in [&first, &second] {
        edits.push((moved.bytes.start, moved.bytes.end, String::new()));
        edits.push((anchor.bytes.end, anchor.bytes.end, chord_clone(&state, moved, &anchor, None)));
    }
    let (post, receipt) = apply_chord_gesture_distribution_batch_authorized(state.document(), edits);
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let post = post.unwrap().0;
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    next.require_complete_course_context(&post).unwrap();
    for moved in [&first, &second] {
        let child = next.occurrences.iter().find(|o| o.origin.parent == Some(moved.creation)).unwrap();
        assert_eq!(child.origin.source, moved.origin.source);
        assert_eq!(child.origin.root, moved.origin.root);
        assert_eq!(child.origin.course, anchor.origin.course);
        assert_eq!(child.origin.interval, anchor.origin.interval);
        assert_eq!(child.origin.relocation.as_ref().unwrap().anchor, anchor.creation);
        assert_eq!(sounding_midi(&next.snapshot[child.bytes.clone()]), sounding_midi(&state.snapshot[moved.bytes.clone()]));
    }
    let first_child = by_source(&next, 11);
    let second_child = by_source(&next, 13);
    assert!(second_child.bytes.start < first_child.bytes.start);
    assert_ne!(first_child.creation, second_child.creation);
    assert_eq!(collect_raw_notes(&post).len(), collect_raw_notes(state.document()).len());
}

#[test]
fn distinct_pitch_insertions_at_shared_anchor_keep_their_own_sources() {
    assert_shared_anchor_capture(67);
}

#[test]
fn equal_pitch_insertions_at_shared_anchor_keep_distinct_identities() {
    assert_shared_anchor_capture(64);
}

#[test]
fn notehead_markup_does_not_replace_the_captured_anchor_note() {
    // This probes physical capture directly; the existing seed remains sealed.
    let state = shared_anchor_received(67);
    let moved = by_source(&state, 11);
    let anchor = by_source(&state, 21);
    let old_anchor = &state.snapshot[anchor.bytes.clone()];
    let marked_anchor = old_anchor.replace("</note>", "<notehead>diamond</notehead></note>");
    let mut before = state.document().to_string();
    before.replace_range(anchor.bytes.clone(), &marked_anchor);
    let insert_at = anchor.bytes.start + marked_anchor.len();
    let (post, receipt) = apply_chord_gesture_distribution_batch_authorized(&before,
        vec![(moved.bytes.start, moved.bytes.end, String::new()),
            (insert_at, insert_at, chord_clone(&state, &moved, &anchor, None))]);
    let post = post.unwrap().0;
    let births = receipt.unwrap().take_physical_births().unwrap();
    births.validate_binding(&before, &post).unwrap();
    assert_eq!(births.records().len(), 1);
    assert_eq!(births.records()[0].anchor, Some(anchor.bytes.start..insert_at));
    assert_eq!(&before[births.records()[0].anchor.clone().unwrap()], marked_anchor);
    assert_eq!(sounding_midi(&post[births.records()[0].output.clone()]),
        sounding_midi(&before[moved.bytes.clone()]));
}
