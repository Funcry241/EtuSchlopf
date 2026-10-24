use super::*;
use crate::master_state_identity::OwnerReceipts;
use crate::structural_write_authority::remove_allowed_xml_ranges_authorized;

/// Received single course, split by the actual visible-rhythm normalizer into
/// tied fragments that carry both a sounding `<tie>` and a notation `<tied>`.
fn tied_fragments() -> OwnedRhythmicContinuation {
    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture(&[(7, 0, 1)], 1);
    let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    let state = OwnedRhythmicContinuation::receive(&custody, &xml).unwrap();
    let normalized = crate::arrangement::normalize_visible_rhythm_musicxml(&xml);
    let mut receipts = OwnerReceipts::default();
    receipts.push_visible_block_with_births(normalized.visible_rhythm_block_identity,
        normalized.stats.physical_births.first().cloned(), &xml).unwrap();
    let split = receipts.continue_owned_rhythm(&state, &normalized.xml).unwrap();
    split.require_complete_course_context(&normalized.xml).unwrap();
    split
}

fn nth_range(xml: &str, needle: &str, nth: usize) -> (usize, usize) {
    let start = xml.match_indices(needle).nth(nth).unwrap().0;
    (start, start + needle.len())
}

fn removed(state: &OwnedRhythmicContinuation, ranges: &[(usize, usize)]) -> (String, OwnerReceipts) {
    let mut post = state.document().to_string();
    let (changed, receipt) = remove_allowed_xml_ranges_authorized(&mut post, ranges);
    assert_eq!(changed.unwrap(), ranges.len());
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(receipt);
    (post, receipts)
}

#[test]
fn orphan_tied_endpoint_cleanup_keeps_identity_and_sounding_tie() {
    let state = tied_fragments();
    let (post, receipts) = removed(&state, &[nth_range(state.document(), "<tied type=\"start\"/>", 0)]);
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    next.require_complete_course_context(&post).unwrap();
    assert!(next.same_identities_as(&state));
    assert_eq!(post.matches("<tie type=").count(), state.document().matches("<tie type=").count());
}

#[test]
fn chained_endpoint_and_empty_notations_cleanup_is_carried() {
    let state = tied_fragments();
    let doc = state.document();
    let (post, receipts) = removed(&state, &[nth_range(doc, "<tied type=\"stop\"/>", 2), nth_range(doc, "<tied type=\"start\"/>", 0)]);
    let middle = receipts.continue_owned_rhythm(&state, &post).unwrap();
    let wrapper_start = post.find("<notations>").unwrap();
    let wrapper_end = wrapper_start + post[wrapper_start..].find("</notations>").unwrap() + "</notations>".len();
    assert!(post[wrapper_start..wrapper_end]["<notations>".len()..wrapper_end - wrapper_start - "</notations>".len()].trim().is_empty());
    let (last, tail) = removed(&middle, &[(wrapper_start, wrapper_end)]);
    let next = tail.continue_owned_rhythm(&middle, &last).unwrap();
    next.require_complete_course_context(&last).unwrap();
    assert!(next.same_identities_as(&state));
}

#[test]
fn removing_a_sounding_tie_is_not_notation_cleanup() {
    let state = tied_fragments();
    let (post, receipts) = removed(&state, &[nth_range(state.document(), "<tie type=\"start\"/>", 0)]);
    let error = receipts.continue_owned_rhythm(&state, &post).unwrap_err().to_string();
    assert!(error.contains("PopulationRow=6") && error.contains("tie disposition"), "{error}");
}

#[test]
fn endpoint_cleanup_receipt_is_bound_to_its_original_state() {
    let state = tied_fragments();
    let (post, receipts) = removed(&state, &[nth_range(state.document(), "<tied type=\"start\"/>", 0)]);
    let next = receipts.continue_owned_rhythm(&state, &post).unwrap();
    assert!(receipts.continue_owned_rhythm(&next, &post).is_err());
}
