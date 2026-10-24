use super::*;
use crate::arrangement::OwnedRhythmicContinuation;
use crate::master_state_identity::OriginalFunctionalRespellingChains;
use std::sync::Arc;

fn normalized(rows: &[(usize, i64, i64, u8, i32)], measures: usize) -> OwnedRhythmicContinuation {
    let state = OwnedRhythmicContinuation::synthetic_source_received_for_test(rows, measures);
    let normalized = crate::arrangement::normalize_visible_rhythm_musicxml(state.document());
    let mut owners = crate::master_state_identity::OwnerReceipts::default();
    owners.push_visible_block_with_births(normalized.visible_rhythm_block_identity,
        normalized.stats.physical_births.first().cloned(), state.document()).unwrap();
    owners.continue_owned_rhythm(&state, &normalized.xml).unwrap()
}

fn original_export(state: &OwnedRhythmicContinuation, chains: OriginalFunctionalRespellingChains,
    reference: &str) -> (Result<String, String>, Option<ExactTransformationReceipt>) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!("original-complete-export-{}-{}.musicxml",
        std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    let operation = crate::structural_write_authority::FunctionalMasterRespellingOperation::new(
        canonical_master_functional_respelling_from_render,
    ).bind_tonal_frame(Some(TonalFrameFact::derive_from_render_snapshot(reference).unwrap()))
        .bind_original_chains(chains);
    let (written, outer, inner) = crate::structural_write_authority::write_master_musicxml_output_authorized(
        &path, state.document(), operation);
    let result = written.map_err(|error| error.to_string()).and_then(|()| {
        let mut post = state.document().to_owned();
        for receipt in [outer.as_ref(), inner.as_ref()].into_iter().flatten() {
            post = receipt.realize_post_document(&post)
                .ok_or_else(|| "original export receipt failed replay".to_string())?;
        }
        Ok(post)
    });
    if result.is_err() { assert!(!path.exists(), "refused original evidence must stop before the Master sink"); }
    let _ = std::fs::remove_file(path);
    if let Ok(post) = &result {
        let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_exact(outer); owners.push_exact(inner);
        let next = owners.continue_owned_rhythm(state, post).unwrap();
        next.require_complete_course_context(post).unwrap();
        assert!(next.same_identities_as(state));
        return (result, None);
    }
    (result, inner)
}

fn reference_with_opening(state: &OwnedRhythmicContinuation, fifths: i32) -> String {
    let text = state.document();
    let doc = roxmltree::Document::parse(text).unwrap();
    let key = doc.descendants().find(|node| node.has_tag_name("fifths")).unwrap();
    format!("{}<fifths>{fifths}</fifths>{}", &text[..key.range().start], &text[key.range().end..])
}

#[test]
fn original_canonical_carrier_crosses_real_key_edits_for_each_independent_copy() {
    for shift in [0, 384] {
        for copies in [1, 2] {
            let rows = (0..copies).flat_map(|copy| {
                let start = shift + copy as i64 * 384;
                [(301 + copy * 3, start, 96, 1, 70),
                 (302 + copy * 3, start + 96, 192, 1, 62),
                 (303 + copy * 3, start + 288, 96, 1, 63)]
            }).collect::<Vec<_>>();
            let state = normalized(&rows, copies + usize::from(shift > 0));
            let (state, chains, changed) = state.original_export_chains_for_test(true);
            assert!(changed, "exercise the actual canonical emitter");
            let reference = reference_with_opening(&state, 6);
            let (post, _) = original_export(&state, chains, &reference);
            let post = post.unwrap();
            let leading = parse_atomic_notes(&post).into_iter().filter(|note|
                note.pitch.as_ref().is_some_and(|pitch| pitch.midi == 62)).collect::<Vec<_>>();
            assert_eq!(leading.len(), copies * 2);
            let first = leading[0].pitch.as_ref().unwrap();
            assert!(leading.iter().all(|note| {
                let pitch = note.pitch.as_ref().unwrap();
                (pitch.step.as_str(), pitch.alter, pitch.octave) == (first.step.as_str(), first.alter, first.octave)
            }));
        }
    }
}

#[test]
fn original_local_key_insertions_keep_complete_chain_membership() {
    let state = normalized(&[(411, 384, 96, 1, 70), (412, 480, 192, 1, 62), (413, 672, 96, 1, 63)], 2);
    let (state, chains, _) = state.original_export_chains_for_test(true);
    let reference = rewrite_key_signature_for_measure_plain(state.document(), 2, 6);
    let (post, _) = original_export(&state, chains, &reference);
    let post = post.unwrap();
    assert_eq!(functional_explicit_key_signatures(&post), functional_explicit_key_signatures(&reference));
}

#[test]
fn original_missing_and_stale_course_evidence_refuses_before_the_master_sink() {
    let state = normalized(&[(511, 0, 96, 1, 70), (512, 96, 192, 1, 62), (513, 288, 96, 1, 63)], 1);
    let (encoded, chains, _) = state.original_export_chains_for_test(true);
    let reference = reference_with_opening(&encoded, 6);
    let missing = OriginalFunctionalRespellingChains::received(Err("absent original evidence".to_string()));
    let (result, receipt) = original_export(&encoded, missing, &reference);
    assert!(result.unwrap_err().contains("absent original evidence")); assert!(receipt.is_none());
    let stale = OriginalFunctionalRespellingChains::received(Ok(Arc::new(state)));
    let (result, receipt) = original_export(&encoded, stale, &reference);
    assert!(result.is_err()); assert!(receipt.is_none());
    assert!(original_export(&encoded, chains, &reference).0.is_ok());
}

#[test]
fn original_conflicts_and_outside_tonal_components_do_not_expand_authority() {
    let state = normalized(&[(611, 24, 360, 1, 62)], 1);
    let (state, chains, _) = state.original_export_chains_for_test(true);
    let chains = chains.after_original_master_export(state.document(), state.document(), None);
    let notes = parse_atomic_notes(state.document()).into_iter().filter(|note|
        note.pitch.as_ref().is_some_and(|pitch| pitch.midi == 62)).collect::<Vec<_>>();
    assert!(notes.len() >= 2);
    let plan = |note: &AtomicNote, step, alter| FunctionalRespellingPlan {
        note_start: note.note_start, note_end: note.note_end, step, alter, octave: 4,
        kind: FunctionalRespellingKind::MinorLeading,
    };
    let conflict = original_chain_functional_respelling_plans(state.document(), state.document(), &[], &chains,
        vec![plan(&notes[0], "D", 0), plan(notes.last().unwrap(), "C", 2)], None);
    assert!(conflict.unwrap_err().contains("conflicting"));
    let last = notes.last().unwrap();
    let boundary = functional_measure_ordinal(last) + 1;
    assert!(notes.iter().all(|note| functional_measure_ordinal(note) < boundary));
    let outside = original_chain_functional_respelling_plans(state.document(), state.document(), &[], &chains,
        vec![plan(last, "C", 2)], Some(boundary));
    assert!(outside.unwrap_err().contains("tonal scope"));
}

#[test]
fn original_same_key_chord_conflicts_are_refused_without_merging_sources() {
    let state = OwnedRhythmicContinuation::synthetic_source_received_for_test(
        &[(651, 0, 384, 1, 62), (652, 0, 384, 1, 62)], 1);
    let (state, chains, _) = state.original_export_chains_for_test(true);
    let chains = chains.after_original_master_export(state.document(), state.document(), None);
    let notes = parse_atomic_notes(state.document()).into_iter().filter(|note|
        note.pitch.as_ref().is_some_and(|pitch| pitch.midi == 62)).collect::<Vec<_>>();
    assert_eq!(notes.len(), 2);
    assert!(notes[1].is_chord);
    let one = FunctionalRespellingPlan { note_start: notes[0].note_start, note_end: notes[0].note_end,
        step: "C", alter: 2, octave: 4, kind: FunctionalRespellingKind::MinorLeading };
    let conflict = original_chain_functional_respelling_plans(state.document(), state.document(), &[], &chains,
        vec![one.clone()], None);
    assert!(conflict.unwrap_err().contains("same-key chord"));
    let other = FunctionalRespellingPlan { note_start: notes[1].note_start, note_end: notes[1].note_end, ..one.clone() };
    assert!(original_chain_functional_respelling_plans(state.document(), state.document(), &[], &chains,
        vec![one, other], None).is_ok());
}

#[test]
fn original_noop_and_internal_hidden_chord_members_remain_distinct() {
    let ordinary = normalized(&[(711, 0, 96, 1, 70), (712, 96, 192, 1, 62), (713, 288, 96, 1, 63)], 1);
    let (state, chains, changed) = ordinary.original_export_chains_for_test(false);
    assert!(!changed, "exercise a real canonical no-op, not an absent capture of a write");
    let reference = reference_with_opening(&state, 6);
    assert!(original_export(&state, chains, &reference).0.is_ok());

    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture(
        &[(721, 0, 384, 1, 66), (722, 0, 384, 1, 67), (723, 384, 384, 1, 63)], 2);
    let arranged = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        crate::arrangement::ArrangementProfile { rhythmic_courses: Some(&seed), preserve_reference: true,
            ..crate::arrangement::ArrangementProfile::default() });
    let mut state = OwnedRhythmicContinuation::receive(arranged.stats.rhythmic_courses.as_ref().unwrap(), &arranged.xml).unwrap();
    let notes = parse_atomic_notes(state.document());
    let mut hidden = notes.iter().filter(|note| !note.visible
        && note.pitch.as_ref().is_some_and(|pitch| pitch.midi == 66)).count();
    assert!(notes.iter().any(|note| state.document()[note.note_start..note.note_end].contains("<chord")));
    for note in notes.iter().rev().filter(|note| note.visible
        && note.pitch.as_ref().is_some_and(|pitch| pitch.midi == 66)) {
        let mut xml = state.document().to_owned();
        let (written, receipt) = crate::structural_write_authority::hide_note_print_object_authorized(
            &mut xml, note.note_start, note.note_end);
        assert!(written.unwrap());
        let mut owners = crate::master_state_identity::OwnerReceipts::default(); owners.push_exact(receipt);
        state = owners.continue_owned_rhythm(&state, &xml).unwrap(); hidden += 1;
    }
    assert!(hidden >= 3, "exercise an original internal tie occurrence");
    let (state, chains, _) = state.original_export_chains_for_test(true);
    let reference = reference_with_opening(&state, -5);
    let post = original_export(&state, chains, &reference).0.unwrap();
    let carried = parse_atomic_notes(&post).into_iter().filter(|note|
        note.pitch.as_ref().is_some_and(|pitch| pitch.midi == 66)).collect::<Vec<_>>();
    assert!(carried.len() >= 3);
    assert!(carried.iter().all(|note| !note.visible && note.pitch.as_ref()
        .is_some_and(|pitch| pitch.step == "G" && pitch.alter == -1)));
}


#[test]
fn actual_retained_copy_and_canonical_tied_chain_cross_export_and_final_layout() {
    use crate::structural_write_authority as authority;
    let mut respellings = 0;
    for (case, pitches) in [[72, 76], [65, 69], [62, 70]].into_iter().enumerate() {
        for copies in [1, 2] {
            let rows = (0..copies).flat_map(|copy| {
                let shift = copy as i64 * 384;
                [(8101 + copy * 3, shift + 24, 1, pitches[0]),
                 (8102 + copy * 3, shift + 24, 1, pitches[1]),
                 (8103 + copy * 3, shift + 288, 1, 63)]
            }).collect::<Vec<_>>();
            let (state, chains, tied) = OwnedRhythmicContinuation::original_retained_chord_export_for_test(&rows, copies);
            assert!(tied >= copies);
            let reference = reference_with_opening(&state, 6);
            let path = std::env::temp_dir().join(format!("original-tied-export-layout-{}-{case}-{copies}.musicxml", std::process::id()));
            let operation = authority::FunctionalMasterRespellingOperation::new(canonical_master_functional_respelling_from_render)
                .bind_tonal_frame(Some(TonalFrameFact::derive_from_render_snapshot(&reference).unwrap()))
                .bind_original_chains(chains);
            let (written, outer, inner) = authority::write_master_musicxml_output_authorized(&path, state.document(), operation);
            written.unwrap();
            assert!(outer.is_some());
            respellings += usize::from(inner.is_some());
            let after = std::fs::read_to_string(&path).unwrap();
            let mut owners = crate::master_state_identity::OwnerReceipts::default();
            owners.push_exact(outer); owners.push_exact(inner);
            let exported = owners.continue_owned_rhythm(&state, &after).unwrap();
            exported.require_complete_course_context(&after).unwrap();
            assert!(exported.same_identities_as(&state));
            let directives = (0..copies).map(|_| authority::HorizontalLayoutMaterializationDirective {
                explicit_width: Some("970".to_owned()),
                system_break: authority::HorizontalSystemBreakMaterialization::None,
                starts_new_page: false,
            }).collect::<Vec<_>>();
            let (written, receipt) = authority::materialize_final_render_notation_with_phrase_projection_authorized(&path, &directives, &[]);
            assert!(written.unwrap() > 0); assert!(receipt.is_some());
            let laid_out = std::fs::read_to_string(&path).unwrap();
            let mut owners = crate::master_state_identity::OwnerReceipts::default();owners.push_preserving(receipt);
            let next = owners.continue_owned_rhythm(&exported, &laid_out).unwrap();
            next.require_complete_course_context(&laid_out).unwrap();
            assert!(next.same_identities_as(&state));
            std::fs::remove_file(path).unwrap();
        }
    }
    println!("OriginalRetainedCopyCanonicalTiedExportLayoutCases=6;ActualInnerRespellings={respellings}");
    assert!(respellings > 0);
}
