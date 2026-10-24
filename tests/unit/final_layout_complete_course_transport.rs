use super::*;
use crate::master_state_identity::OwnerReceipts;
use crate::structural_write_authority as authority;

fn received(copies: usize, shift: i64) -> OwnedRhythmicContinuation {
    let rows = (0..copies).flat_map(|copy| {
        let start = shift + copy as i64 * 384;
        [(701 + copy * 3, start, 96, 1, 60),
         (702 + copy * 3, start + 96, 96, 1, 62),
         (703 + copy * 3, start, 192, 2, 48)]
    }).collect::<Vec<_>>();
    OwnedRhythmicContinuation::synthetic_source_received_for_test(&rows, copies + usize::from(shift > 0))
}

fn layout(state: &OwnedRhythmicContinuation) -> (String, OwnerReceipts) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!("original-final-layout-{}-{}.musicxml",
        std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    std::fs::write(&path, state.document()).unwrap();
    let directives = find_measure_spans(state.document()).iter().enumerate().map(|(index, _)|
        authority::HorizontalLayoutMaterializationDirective {
            explicit_width: Some((890 + index).to_string()),
            system_break: authority::HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }).collect::<Vec<_>>();
    let (written, receipt) = authority::materialize_final_render_notation_with_phrase_projection_authorized(
        &path, &directives, &[]);
    assert!(written.unwrap() > 0);
    let (receipt, post) = receipt.unwrap().split_realized_post_document();
    let post = post.unwrap();
    std::fs::remove_file(path).unwrap();
    let mut owners = OwnerReceipts::default(); owners.push_preserving(Some(receipt));
    (post, owners)
}

#[test]
fn actual_path_bound_layout_carries_every_original_copy_after_encoding() {
    for shift in [0, 384] {
        for copies in [1, 2] {
            let original = received(copies, shift);
            for state in [original.clone(), original.original_final_chord_for_test()] {
                let (post, owners) = layout(&state);
                let next = owners.continue_owned_rhythm(&state, &post).unwrap();
                next.require_complete_course_context(&post).unwrap();
                assert!(next.same_identities_as(&state));
                assert_eq!(next.nonphysical_rest_obligation_count(), state.nonphysical_rest_obligation_count());
                let foreign = received(copies, shift);
                // Text equality alone is not source identity: this comparison
                // deliberately does not grant the foreign state's identities.
                assert!(!foreign.same_identities_as(&state));
                assert!(owners.continue_owned_rhythm(&next, &post).is_err());
                assert!(owners.continue_owned_rhythm(&state, &(post + " ")).is_err());
            }
        }
    }
}

#[test]
fn actual_note_free_additive_carries_encoded_complete_ranges() {
    for partial in [false, true] {
    let encoded = OwnedRhythmicContinuation::original_forward_obligations_for_test(partial, 2);
    assert!(!encoded.encoded.is_empty());
    let mut post = encoded.document().to_owned();
    let (_, receipt) = authority::ensure_final_barline_authorized(&mut post).unwrap();
    assert!(receipt.is_some());
    let mut owners = OwnerReceipts::default(); owners.push_additive(receipt);
    let next = owners.continue_owned_rhythm(&encoded, &post).unwrap();
    assert!(next.same_identities_as(&encoded));
    next.require_complete_course_context(&post).unwrap();
    let (layout_post, layouts) = layout(&next);
    let last = layouts.continue_owned_rhythm(&next, &layout_post).unwrap();
    assert!(last.same_identities_as(&encoded));
    assert!(owners.continue_owned_rhythm(&next, &post).is_err());
    assert_eq!(last.nonphysical_rest_obligation_count(), 2);
    let doc = roxmltree::Document::parse(last.document()).unwrap();
    let forward = doc.descendants().find(|node| node.has_tag_name("forward")).unwrap().range();
    let mut missing = last.document().to_owned(); missing.replace_range(forward, "");
    assert!(last.validate_binding(&missing).is_err());
    }
}
