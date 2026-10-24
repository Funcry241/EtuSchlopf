use super::*;
use crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture;
use crate::master_state_identity::OwnerReceipts;
use crate::structural_write_authority::{apply_atomic_restification_batch_authorized,
    apply_atomic_staff_voice_scalar_batch_authorized, apply_bound_safe_transfer_authorized};

fn received(rows: &[(usize, i64, u8)], measures: usize) -> OwnedRhythmicContinuation {
    let (xml, seed) = arrangement_fixture(rows, measures);
    let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    OwnedRhythmicContinuation::receive(&custody, &xml).unwrap()
}

#[test]
fn successful_validation_memo_never_accepts_foreign_xml() {
    let state = received(&[(17, 0, 1)], 1);
    assert!(state.validated.0.load(Ordering::Relaxed));
    state.validate_binding(state.document()).unwrap();
    let foreign = format!("{} ", state.document());
    assert!(state.validate_binding(&foreign).unwrap_err().to_string().contains("another document state"));
    state.validate_binding(state.document()).unwrap();
}

#[test]
fn validation_memo_is_empty_on_every_clone_and_preserves_debug_output() {
    let state = received(&[(17, 0, 1)], 1);
    let next = state.clone();
    assert!(!next.validated.0.load(Ordering::Relaxed));
    assert_eq!(format!("{state:?}"), format!("{next:?}"));
    assert_eq!(format!("{state:#?}"), format!("{next:#?}"));
    next.validate_binding(state.document()).unwrap();
    assert!(next.validated.0.load(Ordering::Relaxed));
    assert!(!next.clone().validated.0.load(Ordering::Relaxed));
}

#[test]
fn cloned_missing_physical_coverage_is_rejected_repeatedly_without_caching_failure() {
    let state = received(&[(17, 0, 1)], 1);
    let mut next = state.clone();
    let removed = next.occurrences.pop().unwrap();
    for _ in 0..2 {
        assert!(next.validate_binding(state.document()).is_err());
        assert!(!next.validated.0.load(Ordering::Relaxed));
    }
    next.occurrences.push(removed);
    next.validate_binding(state.document()).unwrap();
    assert!(next.validated.0.load(Ordering::Relaxed));
}

#[test]
fn cloned_changed_ancestry_is_rejected_even_with_identical_xml() {
    let state = received(&[(17, 0, 1)], 1);
    let mut next = state.clone();
    next.history.clear();
    assert_eq!(next.document(), state.document());
    assert!(next.validate_binding(state.document()).is_err());
    assert!(!next.validated.0.load(Ordering::Relaxed));
}

#[test]
fn immutable_validation_remains_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<OwnedRhythmicContinuation>();
}

fn rest(text: &str, hidden: bool) -> String {
    format!("<note{}><rest/><duration>{}</duration><voice>{}</voice><staff>{}</staff></note>",
        if hidden { " print-object=\"no\"" } else { "" },
        extract_first_i64(text, "duration").unwrap(), extract_first_text(text, "voice").unwrap(),
        extract_first_i64(text, "staff").unwrap())
}

#[test]
fn actual_label_owner_cannot_change_issued_identity_source_course_or_presentation() {
    for ids in [[7, 8], [91, 43]] {
        let state = received(&[(ids[0], 0, 1), (ids[1], 0, 1)], 2);
        let mut edits = Vec::new();
        for o in &state.occurrences {
            let text = &state.snapshot[o.bytes.clone()];
            let start = o.bytes.start + text.find("<voice>").unwrap() + "<voice>".len();
            let end = start + text[text.find("<voice>").unwrap() + "<voice>".len()..].find("</voice>").unwrap();
            edits.push((start, end, if o.staff == 1 { "701" } else { "903" }.into()));
        }
        let (post, receipt) = apply_atomic_staff_voice_scalar_batch_authorized(state.document(), edits);
        let post = post.unwrap(); let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
        let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
        next.require_complete_course_context(&post).unwrap(); assert!(next.same_identities_as(&state));
        let coincident = next.occurrences.iter().filter(|o| o.origin.source.is_some()).collect::<Vec<_>>();
        assert_ne!(coincident[0].creation, coincident[1].creation);
        assert_ne!(coincident[0].origin.source, coincident[1].origin.source);
        assert_eq!(coincident[0].origin.interval, coincident[1].origin.interval);
        assert!(next.validate_binding(state.document()).is_err());
    }
}

#[test]
fn actual_restification_and_normalizer_preserve_parent_course_and_exact_partition() {
    for rows in [[(17, 0, 1), (19, 144, 1)], [(19, 0, 2), (17, 144, 2)]] {
        let state = received(&rows, 1);
        let target = state.occurrences.iter().find(|o| o.origin.source == Some(rows[0].0)).unwrap();
        let (post, receipt) = apply_atomic_restification_batch_authorized(state.document(),
            vec![(target.bytes.start, target.bytes.end, rest(&state.snapshot[target.bytes.clone()], false))]);
        let post = post.unwrap(); let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
        let restified = receipts.continue_owned_rhythm(&state, &post).unwrap();
        restified.require_complete_course_context(&post).unwrap(); assert_eq!(restified.born_from(&state), 1);
        let child = restified.occurrences.iter().find(|o| o.origin.parent == Some(target.creation)).unwrap();
        assert_eq!(child.origin.source, target.origin.source); assert_eq!(child.origin.course, target.origin.course);
        let normalized = crate::arrangement::normalize_visible_rhythm_musicxml(&post);
        assert!(normalized.stats.visible_rhythm_values_split > 0);
        let mut tail = OwnerReceipts::default(); tail.push_visible_block_with_births(
            normalized.visible_rhythm_block_identity, normalized.stats.physical_births.first().cloned(), &post).unwrap();
        receipts.append(tail);
        let next = receipts.continue_owned_rhythm(&state, &normalized.xml).unwrap();
        next.require_complete_course_context(&normalized.xml).unwrap();
        let fragments = next.occurrences.iter().filter(|o| o.origin.source == target.origin.source).collect::<Vec<_>>();
        assert!(fragments.len() > 1);
        assert!(fragments.iter().all(|o| o.origin.parent == Some(child.creation)
            && o.origin.course == target.origin.course && o.origin.presentation == target.origin.presentation));
        assert!(next.same_identities_as(&receipts.continue_owned_rhythm(&state, &normalized.xml).unwrap()));
    }
}

#[test]
fn actual_transfer_keeps_two_distinct_children_and_explicit_staff_segment() {
    let state = received(&[(11, 0, 1), (12, 96, 1), (21, 0, 2), (22, 96, 2)], 1);
    let input = state.occurrences.iter().find(|o| o.origin.source == Some(11)).unwrap();
    let anchor = state.occurrences.iter().find(|o| o.origin.source == Some(21)).unwrap();
    let old = &state.snapshot[input.bytes.clone()];
    let pitch_start = old.find("<pitch>").unwrap();
    let pitch_end = old.find("</pitch>").unwrap() + "</pitch>".len();
    let pitch = &old[pitch_start..pitch_end];
    let transferred = format!("<note><chord/>{pitch}<duration>96</duration><voice>4001</voice><staff>2</staff></note>");
    let (post, receipt) = apply_bound_safe_transfer_authorized(state.document(), input.bytes.start,
        input.bytes.end, anchor.bytes.end, rest(old, false), transferred);
    let post = post.unwrap(); let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap(); next.require_complete_course_context(&post).unwrap();
    let children = next.occurrences.iter().filter(|o| o.origin.parent == Some(input.creation)).collect::<Vec<_>>();
    assert_eq!(children.len(), 2); assert_ne!(children[0].creation, children[1].creation);
    assert!(children.iter().all(|o| o.origin.source == Some(11) && o.origin.course == input.origin.course
        && o.origin.presentation == input.origin.presentation && o.origin.interval == input.origin.interval));
    assert_eq!(children.iter().map(|o| o.staff).collect::<Vec<_>>(), [1, 2]);
}

#[test]
fn independent_equal_owner_executions_have_distinct_children_and_no_foreign_context_bridge() {
    let state = received(&[(71, 0, 1), (72, 0, 1)], 1);
    let input = &state.occurrences[0];
    let run = || {
        let (post, receipt) = apply_atomic_restification_batch_authorized(state.document(),
            vec![(input.bytes.start, input.bytes.end, rest(&state.snapshot[input.bytes.clone()], false))]);
        let post = post.unwrap(); let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
        receipts.continue_owned_rhythm(&state, &post).unwrap()
    };
    let first = run(); let second = run();
    assert_eq!(first.document(), second.document()); assert!(!first.same_identities_as(&second));
    assert!(first.same_identities_as(&first.clone()));
    let mut missing = first.clone(); missing.occurrences.pop(); assert!(missing.validate_binding(first.document()).is_err());
    let mut duplicate = first.clone(); duplicate.occurrences[1].creation = duplicate.occurrences[0].creation;
    assert!(duplicate.validate_binding(first.document()).is_err());
    let mut forged = first.clone(); forged.occurrences[0].origin.course = second.occurrences[1].origin.course;
    forged.occurrences[0].origin.source = Some(9981);
    assert!(forged.validate_binding(first.document()).is_err());
}

#[test]
fn missing_birth_capture_and_scalar_staff_change_cannot_reconstruct_context() {
    let state = received(&[(51, 0, 1)], 1); let input = &state.occurrences[0];
    let (post, mut receipt) = apply_atomic_restification_batch_authorized(state.document(),
        vec![(input.bytes.start, input.bytes.end, rest(&state.snapshot[input.bytes.clone()], false))]);
    receipt.as_mut().unwrap().take_physical_births().unwrap();
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    assert!(receipts.continue_owned_rhythm(&state, &post.unwrap()).is_err());
    let text = &state.snapshot[input.bytes.clone()]; let start = input.bytes.start + text.find("<staff>").unwrap() + 7;
    let (post, receipt) = apply_atomic_staff_voice_scalar_batch_authorized(state.document(), vec![(start, start + 1, "2".into())]);
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let error = receipts.continue_owned_rhythm(&state, &post.unwrap()).unwrap_err().to_string();
    assert!(error.contains("OwnerStep=1 PopulationRow=40"));
    assert!(error.contains("Retained occurrence changed staff without a course transition"));
    state.require_complete_course_context(state.document()).unwrap();
}

#[test]
fn actual_gap_births_do_not_license_label_derived_silence_or_shift_received_obligations() {
    let state = received(&[(81, 0, 1), (82, 144, 1)], 1);
    let input = state.occurrences.iter().find(|o| o.origin.source == Some(82)).unwrap();
    let (post, receipt) = apply_atomic_restification_batch_authorized(state.document(),
        vec![(input.bytes.start, input.bytes.end, rest(&state.snapshot[input.bytes.clone()], true))]);
    let post = post.unwrap(); let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let restified = receipts.continue_owned_rhythm(&state, &post).unwrap();
    let normalized = crate::arrangement::normalize_visible_rhythm_musicxml(&post);
    assert!(normalized.stats.visible_rhythm_rests_inserted > 0);
    let births = &normalized.stats.physical_births[0];
    assert!(births.records().iter().any(|b| b.kind == PhysicalBirthKind::GapRest && b.input.is_none()));
    // Legacy insertion moved an existing physical rest. The stronger custody
    // refuses this context even though the ordinary BLOCK receipt still exists.
    assert!(restified.continue_visible(births).is_err());
    restified.require_complete_course_context(&post).unwrap();
}

#[test]
fn actual_arrangement_ancestry_survives_later_restification_without_boundary_roots() {
    let (xml, seed) = arrangement_fixture(&[(91, 24, 1), (92, 24, 1)], 2);
    let arranged = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        ArrangementProfile { rhythmic_courses: Some(&seed), preserve_reference: true, ..Default::default() });
    let courses = arranged.stats.rhythmic_courses.as_ref().unwrap();
    let state = OwnedRhythmicContinuation::receive(courses, &arranged.xml).unwrap();
    let input = state.occurrences.iter().find(|o| o.origin.source.is_some() && o.origin.parent.is_some()).unwrap();
    assert_ne!(input.origin.root, input.creation);
    let (post, receipt) = apply_atomic_restification_batch_authorized(state.document(),
        vec![(input.bytes.start, input.bytes.end, format!(" \n{}\n ", rest(&state.snapshot[input.bytes.clone()], false)))]);
    let post = post.unwrap(); let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    next.require_complete_course_context(&post).unwrap();
    let child = next.occurrences.iter().find(|o| o.origin.parent == Some(input.creation)).unwrap();
    assert_eq!(child.origin.root, input.origin.root);
    assert_eq!(child.origin.source, input.origin.source);
    assert_eq!(next.history.get(&input.creation), state.history.get(&input.creation));
    assert!(next.history.contains_key(&input.origin.root));
    let mut forged = next.clone();
    forged.occurrences.iter_mut().find(|o| o.creation == child.creation).unwrap().origin.root = input.creation;
    assert!(forged.validate_binding(&post).is_err());
}

fn actual_staff_move(state: &OwnedRhythmicContinuation, index: usize, staff: u8, voice: &str)
    -> (String, ExactTransformationReceipt) {
    let occurrence = &state.occurrences[index]; let note = &state.snapshot[occurrence.bytes.clone()];
    let value = |tag: &str| {
        let open = format!("<{tag}>"); let local = note.find(&open)? + open.len();
        let start = occurrence.bytes.start + local;
        Some((start, start + note[local..].find('<')?))
    };
    let (post, receipt) = crate::structural_write_authority::apply_bound_staff_voice_move_authorized(
        state.document(), value("voice"), value("staff"), occurrence.bytes.end, voice.into(), staff.to_string());
    (post.unwrap(), receipt.unwrap())
}

#[test]
fn actual_staff_owner_transports_existing_source_identity_without_new_ancestry() {
    for (source, staff, voice) in [(71, 1, "90001"), (17, 2, "3")] {
        let state = received(&[(source, 0, staff), (source + 1, 0, staff)], 1);
        let index = state.occurrences.iter().position(|o| o.origin.source == Some(source)).unwrap();
        let (post, receipt) = actual_staff_move(&state, index, 3 - staff, voice);
        let mut receipts = OwnerReceipts::default(); receipts.push_exact(Some(receipt));
        let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
        next.require_complete_course_context(&post).unwrap();
        assert!(next.same_identities_as(&state)); assert_eq!(next.history, state.history);
        assert_eq!(next.born_from(&state), 0);
        assert_eq!(next.occurrences[index].staff, 3 - staff);
        assert_eq!(next.occurrences[index].origin, state.occurrences[index].origin);
        assert_eq!(next.occurrences.iter().filter(|o| o.origin.source.is_some()).count(), 2);
        assert!(next.validate_binding(state.document()).is_err());
    }
}

#[test]
fn staff_move_capture_is_execution_bound_missing_capture_is_unknown_without_owner_refusal() {
    let state = received(&[(71, 0, 1)], 1);
    let index = state.occurrences.iter().position(|o| o.origin.source == Some(71)).unwrap();
    let (post, mut receipt) = actual_staff_move(&state, index, 2, "90001");
    let capture = receipt.take_staff_transition().unwrap();
    let (_, foreign) = actual_staff_move(&state, index, 2, "90002");
    assert!(capture.validate_exact_execution(&foreign).is_err());
    let (_, equal_execution) = actual_staff_move(&state, index, 2, "90001");
    assert!(capture.validate_exact_execution(&equal_execution).is_err());
    assert_eq!(receipt.realize_post_document(state.document()).as_deref(), Some(post.as_str()));
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(Some(receipt));
    let error = receipts.continue_owned_rhythm(&state, &post).unwrap_err().to_string();
    assert!(error.contains("PopulationRow=4") && error.contains("without a course transition"));
}

#[test]
fn actual_staff_owner_cannot_assign_source_context_to_a_generated_rest() {
    let state = received(&[(71, 0, 1)], 1);
    let index = state.occurrences.iter().position(|o| o.origin.source.is_none()).unwrap();
    let (post, receipt) = actual_staff_move(&state, index, 3 - state.occurrences[index].staff, "901");
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(Some(receipt));
    assert!(receipts.continue_owned_rhythm(&state, &post).is_err());
}

#[test]
fn actual_gesture_move_cannot_reconstruct_source_identity_from_equal_pitch() {
    let state = received(&[(71, 0, 1), (72, 96, 1), (81, 0, 2), (82, 96, 2)], 1);
    let input = state.occurrences.iter().find(|o| o.origin.source == Some(71)).unwrap();
    let anchor = state.occurrences.iter().find(|o| o.origin.source == Some(81)).unwrap();
    let text = &state.snapshot[input.bytes.clone()];
    let pitch = &text[text.find("<pitch>").unwrap()..text.find("</pitch>").unwrap() + "</pitch>".len()];
    let hidden = format!("<note print-object=\"no\"><chord/>{pitch}<duration>96</duration><voice>901</voice><staff>2</staff></note>");
    let (post, receipt) = apply_bound_safe_transfer_authorized(state.document(), input.bytes.start,
        input.bytes.end, anchor.bytes.end, rest(text, false), hidden);
    let post = post.unwrap(); let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let state = receipts.continue_owned_rhythm(&state, &post).unwrap();
    let input = state.occurrences.iter().find(|o| o.origin.source == Some(71)
        && state.snapshot[o.bytes.clone()].contains("<pitch>")).unwrap();
    let anchor = state.occurrences.iter().find(|o| o.origin.source == Some(82)).unwrap();
    let moved = state.snapshot[input.bytes.clone()].replacen(" print-object=\"no\"", "", 1);
    let (post, receipt) = crate::structural_write_authority::apply_chord_gesture_distribution_batch_authorized(
        state.document(), vec![(input.bytes.start, input.bytes.end, String::new()), (anchor.bytes.end, anchor.bytes.end, moved)]);
    let (post, _) = post.unwrap(); let receipt = receipt.unwrap();
    assert_eq!(receipt.realize_post_document(state.document()).as_deref(), Some(post.as_str()));
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(Some(receipt));
    let error = receipts.continue_owned_rhythm(&state, &post).unwrap_err().to_string();
    assert!(error.contains("PopulationRow=26"));
}


#[test]
fn original_additive_captures_carry_equal_notes_in_order_without_consuming_adoption_evidence() {
    use crate::structural_write_authority::{ensure_final_barline_authorized,
        first_structural_measure_locator, insert_direction_into_measure_authorized, MasterAdoptedState};
    use crate::master_state_identity::{MasterLineage, tests::synthetic_genesis_for_tests};
    for ids in [[73, 74], [511, 302]] {
        let state = received(&[(ids[0], 0, 1), (ids[1], 0, 1)], 2);
        let mut master = MasterAdoptedState::adopt_genesis_state(state.document().to_string(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(state.document())));
        master.observe_selected_custody("synthetic", Some(Arc::new(state.clone())), None);
        let mut post = state.document().to_string();
        let locator = first_structural_measure_locator(&post).unwrap();
        let (written, direction) = insert_direction_into_measure_authorized(&mut post, &locator,
            "<direction placement=\"above\"><direction-type><words>synthetic</words></direction-type></direction>");
        assert_eq!(written, 1);
        let (_, barline) = ensure_final_barline_authorized(&mut post).unwrap();
        let mut receipts = OwnerReceipts::default();
        receipts.push_additive(direction); receipts.push_additive(barline);
        let first = receipts.continue_owned_rhythm(&state, &post).unwrap();
        let second = receipts.continue_owned_rhythm(&state, &post).unwrap();
        first.require_complete_course_context(&post).unwrap();
        assert!(first.same_identities_as(&state)); assert!(first.same_identities_as(&second));
        let coincident = first.occurrences.iter().filter(|o| o.origin.source.is_some()).collect::<Vec<_>>();
        assert_ne!(coincident[0].creation, coincident[1].creation);
        assert_eq!(coincident[0].origin.interval, coincident[1].origin.interval);
        assert!(receipts.continue_owned_rhythm(&first, &post).is_err());
        master.adopt_materialized_document(post.clone(), receipts, "SyntheticAdditiveChain");
        assert_eq!(master.document(), post);
        assert!(master.lineage().lineage_continuity_holds(&post));
    }
}

#[test]
fn additive_transport_rejects_stale_input_and_foreign_endpoint() {
    use crate::structural_write_authority::ensure_final_barline_authorized;
    let state = received(&[(83, 0, 1)], 1);
    let mut post = state.document().to_string();
    let (_, receipt) = ensure_final_barline_authorized(&mut post).unwrap();
    let receipt = receipt.unwrap();
    let next = state.continue_additive(&receipt).unwrap();
    assert!(next.continue_additive(&receipt).is_err());
    let mut receipts = OwnerReceipts::default(); receipts.push_additive(Some(receipt));
    assert!(receipts.continue_owned_rhythm(&state, state.document()).is_err());
}


#[test]
fn additive_readback_requires_original_capture_and_never_consumes_it() {
    let state = received(&[(89, 0, 1)], 1);
    let mut xml = state.document().to_string();
    let (_, receipt) = crate::structural_write_authority::ensure_final_barline_authorized(&mut xml).unwrap();
    let receipt = receipt.unwrap();
    let first = crate::master_state_identity::additive_original_documents(&receipt).unwrap();
    assert_eq!(crate::master_state_identity::additive_original_documents(&receipt).unwrap(), first);
    crate::master_state_identity::tests::forget_additive_origin_for_tests(&receipt);
    assert!(state.continue_additive(&receipt).unwrap_err().to_string().contains("Missing original additive capture"));
}

#[test]
fn original_additive_chain_refuses_reversed_receipt_order() {
    use crate::structural_write_authority::{ensure_final_barline_authorized,
        first_structural_measure_locator, insert_direction_into_measure_authorized};
    let state = received(&[(97, 0, 1)], 1);
    let mut post = state.document().to_string();
    let locator = first_structural_measure_locator(&post).unwrap();
    let (written, direction) = insert_direction_into_measure_authorized(&mut post, &locator,
        "<direction><direction-type><words>independent</words></direction-type></direction>");
    assert_eq!(written, 1);
    let (_, barline) = ensure_final_barline_authorized(&mut post).unwrap();
    let mut reversed = OwnerReceipts::default();
    reversed.push_additive(barline); reversed.push_additive(direction);
    let error = reversed.continue_owned_rhythm(&state, &post).unwrap_err().to_string();
    assert!(error.contains("OwnerStep=1") && error.contains("another state"));
}

#[test]
fn actual_pitch_change_then_reversal_cannot_hide_an_unbound_intermediate() {
    let state = received(&[(101, 0, 1)], 1);
    let mut post = state.document().to_string();
    let start = post.find("<pitch>").unwrap();
    let end = post.find("</pitch>").unwrap() + "</pitch>".len();
    let original = post[start..end].to_string();
    let replacement = "<pitch><step>D</step><octave>6</octave></pitch>";
    let (changed, first) = crate::structural_write_authority::apply_harmonic_pitch_swap_authorized(
        &mut post, start, end, replacement);
    assert!(changed && first.is_some());
    let (changed, second) = crate::structural_write_authority::apply_harmonic_pitch_swap_authorized(
        &mut post, start, start + replacement.len(), &original);
    assert!(changed && second.is_some());
    assert_eq!(post, state.document());
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(first); receipts.push_exact(second);
    let error = receipts.continue_owned_rhythm(&state, &post).unwrap_err().to_string();
    assert!(error.contains("OwnerStep=1") && error.contains("PopulationRow=8"));
}


#[test]
fn original_custody_must_be_valid_before_owned_state_can_be_received() {
    let (xml, seed) = arrangement_fixture(&[(41, 0, 1), (73, 96, 2)], 2);
    let valid = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    OwnedRhythmicContinuation::receive(&valid, &xml).unwrap().validate_binding(&xml).unwrap();

    let mut wrong_document = valid.clone();
    wrong_document.snapshot.push(' ');
    assert!(OwnedRhythmicContinuation::receive(&wrong_document, &xml).is_err());

    let mut missing_ancestry = valid.clone();
    missing_ancestry.history.clear();
    assert!(OwnedRhythmicContinuation::receive(&missing_ancestry, &xml).is_err());

    let mut duplicate_course = valid.clone();
    duplicate_course.coverage.push(duplicate_course.coverage[0].clone());
    assert!(OwnedRhythmicContinuation::receive(&duplicate_course, &xml).is_err());

    let mut incomplete_support = valid.clone();
    incomplete_support.occurrences.pop();
    assert!(OwnedRhythmicContinuation::receive(&incomplete_support, &xml).is_err());
}
