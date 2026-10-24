use super::*;
use crate::structural_write_authority::apply_editorial_voice_complete_rewrite_authorized;

fn fixture(staff: u8, measures: usize) -> OwnedRhythmicContinuation {
    OwnedRhythmicContinuation::synthetic_pitched_received_for_test(
        &[(907, 24, staff, 65), (503, 96, staff, 69)], measures)
}

fn rewrite(state: &OwnedRhythmicContinuation, staff: u8, measure: usize, voice: &str)
    -> (String, ExactTransformationReceipt) {
    let physical = physical_inventory(state.document()).unwrap();
    let start = measure as i64 * 384;
    let inputs = physical.iter().filter(|p| p.staff == staff && p.interval.start >= start
        && p.interval.end <= start + 384).map(|p| (p.bytes.start, p.bytes.end)).collect::<Vec<_>>();
    let insertion = inputs.iter().map(|p| p.0).min().unwrap();
    let replacement = format!("<note><rest/><duration>384</duration><voice>{voice}</voice><type>whole</type><staff>{staff}</staff></note>");
    let (after, receipt) = apply_editorial_voice_complete_rewrite_authorized(
        state.document(), inputs, insertion, replacement);
    (after.unwrap(), receipt.unwrap())
}

#[test]
fn actual_editorial_owner_captures_every_parent_and_retains_original_roots() {
    for staff in [1, 2] {
        let initial = fixture(staff, 1);
        let roots = initial.history.keys().copied().collect::<BTreeSet<_>>();
        let (after, mut receipt) = rewrite(&initial, staff, 0, "29");
        let capture = receipt.take_physical_births().expect("actual emitter N:1 capture");
        assert_eq!(capture.consolidations().len(), 1);
        assert!(capture.consolidations()[0].inputs.len() > 1);
        let next = initial.continue_exact(&receipt, Some(&capture)).unwrap();
        next.require_complete_course_context(&after).unwrap();
        assert!(roots.iter().all(|id| next.history.contains_key(id)));
        assert_eq!(next.consolidations.len(), 1);
        let support = next.support_by_root().unwrap();
        assert!(roots.iter().all(|id| support.get(id).is_some_and(|v| !v.is_empty())));
        let created = next.occurrences.iter().find(|o| o.origin.birth == Some(PhysicalBirthKind::RestConsolidated)).unwrap();
        assert!(created.origin.source.is_none());
        assert!(created.origin.course.is_some());
        assert_eq!(created.origin.presentation, Some(SeedCoursePresentation::Primary));
        assert!(!roots.contains(&created.creation));
    }
}

#[test]
fn original_receipt_without_creator_capture_cannot_reconstruct_the_merge() {
    let initial = fixture(1, 1);
    let (_, mut receipt) = rewrite(&initial, 1, 0, "29");
    assert!(receipt.take_physical_births().is_some());
    assert!(initial.continue_exact(&receipt, None).is_err());
}

#[test]
fn nested_consolidations_preserve_all_prior_received_support() {
    let initial = fixture(1, 1);
    let (first_xml, first) = rewrite(&initial, 1, 0, "29");
    let next = first_owner_continuation(&initial, first, &first_xml);
    let (last_xml, last) = rewrite(&next, 1, 0, "41");
    let last = first_owner_continuation(&next, last, &last_xml);
    assert_eq!(last.consolidations.len(), 2);
    last.require_complete_course_context(&last_xml).unwrap();
    assert!(initial.history.keys().all(|id| last.support_by_root().unwrap().contains_key(id)));
}

fn first_owner_continuation(initial: &OwnedRhythmicContinuation,
    mut receipt: ExactTransformationReceipt, after: &str) -> OwnedRhythmicContinuation {
    let capture = receipt.take_physical_births().unwrap();
    let next = initial.continue_exact(&receipt, Some(&capture)).unwrap();
    next.validate_binding(after).unwrap(); next
}

#[test]
fn separate_measure_consolidations_preserve_existing_local_decisions() {
    let initial = fixture(1, 2);
    let (a, receipt) = rewrite(&initial, 1, 0, "29");
    let first = first_owner_continuation(&initial, receipt, &a);
    let first_root = *first.consolidations.keys().next().unwrap();
    let first_origin = first.history[&first_root].clone();
    let (b, receipt) = rewrite(&first, 1, 1, "41");
    let both = first_owner_continuation(&first, receipt, &b);
    both.require_complete_course_context(&b).unwrap();
    assert_eq!(both.history[&first_root], first_origin);
    assert_eq!(first.support_by_root().unwrap()[&first_root], both.support_by_root().unwrap()[&first_root]);
}

#[test]
fn capture_from_another_owner_execution_is_not_a_merge_witness() {
    let initial = fixture(1, 1);
    let (_, mut receipt_a) = rewrite(&initial, 1, 0, "29");
    let (_, receipt_b) = rewrite(&initial, 1, 0, "41");
    let capture = receipt_a.take_physical_births().unwrap();
    assert!(initial.continue_exact(&receipt_b, Some(&capture)).is_err());
}

#[test]
fn missing_duplicate_cyclic_or_changed_context_history_fails_closed() {
    let initial = fixture(1, 1);
    let (after, receipt) = rewrite(&initial, 1, 0, "29");
    let next = first_owner_continuation(&initial, receipt, &after);
    let root = *next.consolidations.keys().next().unwrap();
    let parent = next.consolidations[&root].parents[0].creation;
    let mut missing = next.clone(); missing.history.remove(&parent);
    assert!(missing.validate_binding(&after).is_err());
    let mut source_drift = next.clone();
    source_drift.history.get_mut(&parent).unwrap().source = Some(701);
    assert!(source_drift.validate_binding(&after).is_err());
    let mut duplicate = next.clone();
    let edge = duplicate.consolidations[&root].parents[0].clone();
    duplicate.consolidations.get_mut(&root).unwrap().parents.push(edge);
    assert!(duplicate.validate_binding(&after).is_err());
    let mut cyclic = next.clone();
    let edge = &mut cyclic.consolidations.get_mut(&root).unwrap().parents[0];
    edge.creation = root; edge.root = root;
    assert!(cyclic.validate_binding(&after).is_err());
    let mut context = next.clone();
    context.history.get_mut(&root).unwrap().course = None;
    context.occurrences.iter_mut().find(|o| o.creation == root).unwrap().origin.course = None;
    assert!(context.validate_binding(&after).is_err());
}

#[test]
fn incomplete_parent_union_cannot_authorize_a_full_measure_rest() {
    let initial = fixture(1, 1);
    let physical = physical_inventory(initial.document()).unwrap();
    let p = physical.iter().find(|p| p.staff == 1 && p.text.contains("<pitch>")).unwrap();
    let replacement = "<note><rest/><duration>384</duration><voice>29</voice><staff>1</staff></note>";
    let (after, mut receipt) = apply_editorial_voice_complete_rewrite_authorized(initial.document(),
        vec![(p.bytes.start, p.bytes.end)], p.bytes.start, replacement.into());
    assert!(after.is_some());
    let mut receipt = receipt.take().unwrap();
    let capture = receipt.take_physical_births().unwrap();
    assert!(initial.continue_exact(&receipt, Some(&capture)).is_err());
}

#[test]
fn rest_consolidation_cannot_transfer_staff_or_collapse_independent_hands() {
    let initial = fixture(1, 1);
    let physical = physical_inventory(initial.document()).unwrap();
    let inputs = physical.iter().map(|p| (p.bytes.start, p.bytes.end)).collect::<Vec<_>>();
    let insertion = inputs.iter().map(|p| p.0).min().unwrap();
    let (_, mut receipt) = apply_editorial_voice_complete_rewrite_authorized(initial.document(), inputs,
        insertion, "<note><rest/><duration>384</duration><voice>29</voice><staff>1</staff></note>".into());
    let mut receipt = receipt.take().unwrap(); let capture = receipt.take_physical_births().unwrap();
    assert!(initial.continue_exact(&receipt, Some(&capture)).is_err());
}

#[test]
fn exact_union_uses_actual_support_not_interval_hulls_or_unit_sizes() {
    for scale in [1, 7, 29] {
        exact_interval_union([2*scale..4*scale, 4*scale..8*scale], &(2*scale..8*scale)).unwrap();
        exact_interval_union([2*scale..6*scale, 4*scale..8*scale], &(2*scale..8*scale)).unwrap();
        assert!(exact_interval_union([2*scale..4*scale, 5*scale..8*scale], &(2*scale..8*scale)).is_err());
        assert!(exact_interval_union(std::iter::once(2*scale..4*scale), &(2*scale..8*scale)).is_err());
        assert!(exact_interval_union(std::iter::once(2*scale..9*scale), &(2*scale..8*scale)).is_err());
    }
}

#[test]
fn independent_received_courses_do_not_merge_by_staff_or_role_equality() {
    let a = fixture(1, 1); let b = fixture(1, 1);
    let a = &a.occurrences.iter().find(|o| o.staff == 1).unwrap().origin;
    let b = &b.occurrences.iter().find(|o| o.staff == 1).unwrap().origin;
    assert_eq!(a.presentation, b.presentation);
    assert_ne!(a.course, b.course);
    assert_eq!(common_context([a, b]), (None, None));
    assert_eq!(common_context([a, a]), (a.course, a.presentation));
}

#[test]
fn identical_bytes_from_another_receipt_cannot_replay_a_creation_event() {
    let initial = fixture(1, 1);
    let (a, mut receipt_a) = rewrite(&initial, 1, 0, "29");
    let (b, receipt_b) = rewrite(&initial, 1, 0, "29");
    assert_eq!(a, b);
    assert_ne!(receipt_a.ssc_bound_origin().0, receipt_b.ssc_bound_origin().0);
    let capture = receipt_a.take_physical_births().unwrap();
    assert!(initial.continue_exact(&receipt_b, Some(&capture)).is_err());
}
