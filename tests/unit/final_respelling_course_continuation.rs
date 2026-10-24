use super::*;

#[test]
fn original_respelling_receipts_preserve_received_midi_and_complete_chains() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let mut observed = 0;
    let mut complete_chain_repairs = 0;
    for copies in [1, 2] {
        // The last case is D-sharp minor: its original context rule changes
        // the member adjacent to the tonic; the original component must now be complete.
        for (fifths, previous, leading, tonic) in [
            (-5, 67, 66, 61), (-2, 67, 66, 70), (2, 67, 66, 62),
            (5, 67, 66, 71), (6, 70, 62, 63),
        ] {
            for normalized in [false, true] {
                let rows = (0..copies).flat_map(|copy| {
                    let shift = copy as i64 * 384;
                    [
                        (701 + copy * 3, shift, 96, 1, previous),
                        (702 + copy * 3, shift + 96, 192, 1, leading),
                        (703 + copy * 3, shift + 288, 96, 1, tonic),
                    ]
                }).collect::<Vec<_>>();
                let state = crate::arrangement::OwnedRhythmicContinuation::synthetic_source_received_for_test(&rows, copies);
                let state = if normalized {
                    let normalized = crate::arrangement::normalize_visible_rhythm_musicxml(state.document());
                    let mut owners = crate::master_state_identity::OwnerReceipts::default();
                    owners.push_visible_block_with_births(normalized.visible_rhythm_block_identity,
                        normalized.stats.physical_births.first().cloned(), state.document()).unwrap();
                    owners.continue_owned_rhythm(&state, &normalized.xml).unwrap()
                } else { state };
                let (state, original_chains, _) = state.original_export_chains_for_test(true);
                let before = state.document();
                let doc = roxmltree::Document::parse(before).unwrap();
                let opening = doc.descendants().find(|node| node.has_tag_name("fifths")).unwrap();
                let mut reference = before.to_owned();
                reference.replace_range(opening.range(), &format!("<fifths>{fifths}</fifths>"));
                let operation = crate::structural_write_authority::FunctionalMasterRespellingOperation::new(
                    canonical_master_functional_respelling_from_render,
                ).bind_tonal_frame(Some(TonalFrameFact::derive_from_render_snapshot(&reference).unwrap()))
                    .bind_original_chains(original_chains);
                let path = std::env::temp_dir().join(format!("original-final-export-{}-{}.musicxml",
                    std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
                let (written, outer, inner) = crate::structural_write_authority::write_master_musicxml_output_authorized(
                    &path, before, operation,
                );
                written.unwrap();
                let after = std::fs::read_to_string(&path).unwrap();
                std::fs::remove_file(path).unwrap();
                assert_eq!(outer.as_ref().unwrap().descriptor_row(), 59);
                let intermediate = outer.as_ref().unwrap().realize_post_document(before).unwrap();
                if let Some(receipt) = &inner {
                    observed += 1;
                    assert_eq!(receipt.descriptor_row(), 60);
                    assert_eq!(receipt.realize_post_document(&intermediate).as_deref(), Some(after.as_str()));
                } else { assert_eq!(intermediate, after); }
                let parsed = roxmltree::Document::parse(&after).unwrap();
                let tied = parsed.descendants().filter(|node| node.has_tag_name("note") &&
                    node.children().any(|child| child.has_tag_name("tie")) &&
                    parse_pitch_from_note_block(&after[node.range()]).map(|pitch| pitch.midi) == Some(leading)).collect::<Vec<_>>();
                if normalized { assert_eq!(tied.len(), copies * 2, "the actual normalizer must issue the original complete chains"); }
                let spellings = tied.iter().map(|node| node.children().find(|child| child.has_tag_name("pitch"))
                    .map(|pitch| &after[pitch.range()])).collect::<Vec<_>>();
                let complete = spellings.windows(2).all(|pair| pair[0] == pair[1]);
                assert!(complete, "the original owner must now spell complete original chains");
                complete_chain_repairs += usize::from(normalized && fifths == 6);
                let mut owners = crate::master_state_identity::OwnerReceipts::default();
                owners.push_exact(outer);
                owners.push_exact(inner);
                let next = owners.continue_owned_rhythm(&state, &after).unwrap();
                next.require_complete_course_context(&after).unwrap();
                assert!(next.same_identities_as(&state));
            }
        }
    }
    assert!(observed > 0, "exercise actual original respelling receipts");
    assert!(complete_chain_repairs > 0, "exercise the actual original context-dependent complete-chain repair");
}
