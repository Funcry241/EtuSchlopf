use super::*;
use crate::structural_write_authority as authority;

impl OwnedRhythmicContinuation {
    pub(crate) fn original_retained_chord_export_for_test(rows: &[(usize, i64, u8, i32)], measures: usize)
        -> (Self, crate::master_state_identity::OriginalFunctionalRespellingChains, usize) {
        let state = Self::synthetic_pitched_received_for_test(rows, measures);
        let mut normalized = normalize_visible_rhythm_musicxml(state.document());
        let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_visible_block_with_births(normalized.visible_rhythm_block_identity.take(),
            normalized.stats.physical_births.pop(), state.document()).unwrap();
        let shifted = owners.continue_owned_rhythm(&state, &normalized.xml).unwrap();
        shifted.require_complete_course_context(&normalized.xml).unwrap();
        assert!(shifted.reparameterized_obligation_count() > 0);
        let (post, receipt, chains) = execute_with_original_chains(&shifted, false);
        let receipt = receipt.unwrap();
        let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
        let tied = capture.rows().iter().filter(|row| row.tied_chain()).count();
        assert!(tied > 0);
        let next = shifted.continue_final_chord(&receipt, capture).unwrap();
        next.require_complete_course_context(&post).unwrap();
        (next, chains, tied)
    }

    pub(crate) fn original_forward_obligations_for_test(partial: bool, copies: usize) -> Self {
        let rows = (0..copies).flat_map(|copy| {
            let mut rows = vec![(801 + copy * 2, copy as i64 * 384, 384, 1, 72)];
            if partial { rows.push((802 + copy * 2, copy as i64 * 384 + 192, 192, 1, 74)); }
            rows
        }).collect::<Vec<_>>();
        let state = received(&rows, copies);
        let state = move_sources_with_original_staff_owner(&state,
            &(0..copies).map(|copy| 801 + copy * 2).collect::<Vec<_>>(), 2);
        let (post, receipt) = execute(&state, false);
        let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_exact(receipt);
        let encoded = owners.continue_owned_rhythm(&state, &post).unwrap();
        assert_eq!(encoded.nonphysical_rest_obligation_count(), copies);
        encoded
    }
    pub(crate) fn original_final_chord_for_test(&self) -> Self {
        let state = noncanonical_original_voices(self);
        let (post, receipt) = execute(&state, false);
        let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_exact(receipt);
        let next = owners.continue_owned_rhythm(&state, &post).unwrap();
        assert!(!next.encoded.is_empty());
        next
    }

    pub(crate) fn original_export_chains_for_test(&self, relabel: bool)
        -> (Self, crate::master_state_identity::OriginalFunctionalRespellingChains, bool) {
        let state = if relabel { noncanonical_original_voices(self) } else { self.clone() };
        let (post, receipt, chains) = execute_with_original_chains(&state, relabel);
        let changed = receipt.is_some();
        let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_exact(receipt);
        let next = owners.continue_owned_rhythm(&state, &post).unwrap();
        (next, chains, changed)
    }

}


fn received(rows: &[(usize, i64, i64, u8, i32)], count: usize) -> OwnedRhythmicContinuation {
    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture(rows, count);
    let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    let state = OwnedRhythmicContinuation::receive(&custody, &xml).unwrap();
    noncanonical_original_voices(&state)
}

pub(super) fn noncanonical_original_voices(state: &OwnedRhythmicContinuation) -> OwnedRhythmicContinuation {
    let doc = roxmltree::Document::parse(state.document()).unwrap();
    let edits = doc.descendants().filter(|node| node.has_tag_name("note") &&
        node.children().any(|child| child.has_tag_name("pitch"))).map(|note| {
        let voice = note.children().find(|child| child.has_tag_name("voice")).unwrap();
        let text = voice.text().unwrap();
        let start = voice.range().start + "<voice>".len();
        (start, start + text.len(), "701".to_owned())
    }).collect();
    let (post, receipt) = authority::apply_atomic_staff_voice_scalar_batch_authorized(state.document(), edits);
    let post = post.unwrap();
    let mut owners = crate::master_state_identity::OwnerReceipts::default();
    owners.push_exact(receipt);
    owners.continue_owned_rhythm(state, &post).unwrap()
}

fn move_sources_with_original_staff_owner(state: &OwnedRhythmicContinuation,
    sources: &[usize], target_staff: u8) -> OwnedRhythmicContinuation {
    let mut state = state.clone();
    for source in sources {
        let old = state.occurrences.iter().find(|old| old.origin.source == Some(*source)).unwrap();
        let text = &state.snapshot[old.bytes.clone()];
        let value_range = |tag: &str| {
            let open = format!("<{tag}>");
            let start = old.bytes.start + text.find(&open).unwrap() + open.len();
            let end = start + state.snapshot[start..].find('<').unwrap();
            Some((start, end))
        };
        let voice = extract_first_text(text, "voice").unwrap();
        let (post, receipt) = authority::apply_bound_staff_voice_move_authorized(state.document(),
            value_range("voice"), value_range("staff"), old.bytes.end,
            voice.to_owned(), target_staff.to_string());
        let post = post.unwrap();
        assert!(receipt.as_ref().unwrap().staff_transition().is_some());
        let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_exact(receipt);
        state = owners.continue_owned_rhythm(&state, &post).unwrap();
    }
    state
}

pub(super) fn execute(state: &OwnedRhythmicContinuation, renamed: bool)
    -> (String, Option<ExactTransformationReceipt>) {
    let (post, receipt, _) = execute_with_original_chains(state, renamed);
    (post, receipt)
}

fn execute_with_original_chains(state: &OwnedRhythmicContinuation, renamed: bool)
    -> (String, Option<ExactTransformationReceipt>, crate::master_state_identity::OriginalFunctionalRespellingChains) {
    let physical = physical_inventory(state.document()).unwrap();
    let doc = roxmltree::Document::parse(state.document()).unwrap();
    let mut evidence = Vec::new();
    let mut attacks = BTreeMap::new();
    let mut offset = 0;
    let mut rhythm = RhythmContext::default();
    for span in find_measure_spans(state.document()) {
        let text = &state.document()[span.start..span.end];
        rhythm = rhythm.update_from_measure(text);
        let scope = doc.descendants().find(|node| node.has_tag_name("measure")
            && node.range().start == span.start).unwrap().attribute("number").unwrap().parse().unwrap();
        for (ordinal, note) in physical.iter().filter(|note|
            note.bytes.start >= span.start && note.bytes.end <= span.end).enumerate() {
            let Some(pitch) = sounding_midi(&note.text) else { continue; };
            let old = state.occurrences.iter().find(|old| old.bytes == note.bytes)
                .or_else(|| state.encoded.iter().find(|encoded| encoded.note_ranges().any(|bytes| *bytes == note.bytes))
                    .map(|encoded| &encoded.received)).unwrap();
            let source = old.origin.source.unwrap();
            evidence.push(authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: evidence.len() + 1, source_event_id: source, attack_id: source,
                measure: scope, raw_note_index: ordinal, onset: note.interval.start - offset,
                duration: note.interval.end - note.interval.start, pitch: pitch as i32, staff: note.staff,
                chord_syntax: note.text.contains("<chord"),
                tie_start: marks_named(&note.text, &["tie"]).contains(&"tie:Some(\"start\")".to_string()),
                tie_stop: marks_named(&note.text, &["tie"]).contains(&"tie:Some(\"stop\")".to_string()),
            });
            attacks.entry(source).and_modify(|row: &mut authority::FrozenVoiceAttackEvidenceRow| {
                row.absolute_start = row.absolute_start.min(old.origin.interval.start);
                row.absolute_end = row.absolute_end.max(old.origin.interval.end);
            }).or_insert(authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: source, attack_id: source, absolute_start: old.origin.interval.start,
                absolute_end: old.origin.interval.end, staff: note.staff, technical_lane: 1,
            });
        }
        offset += rhythm.expected_duration();
    }
    let (mut canonical, _) = authority::plan_canonical_final_chord_structure_once(state.document(), &evidence).unwrap();
    canonical.bind_original_respelling_input(crate::master_state_identity::OriginalFunctionalRespellingChains::received(
        Ok(std::sync::Arc::new(state.clone()))));
    let channels = authority::build_frozen_serialization_voice_channels_for_canonical(
        &canonical, &attacks.into_values().collect::<Vec<_>>()).unwrap();
    let preferences = if renamed { evidence.iter().map(|row| authority::LegacyVoiceLabelPreferenceRow {
        source_event_id: row.source_event_id, label: format!("lane-α-{}", row.staff),
    }).collect() } else { Vec::new() };
    let voices = authority::bind_musicxml_voice_labels(&channels, &preferences).unwrap();
    let plan = authority::bind_canonical_final_chord_encoding_plan(state.document(), &canonical, &voices).unwrap();
    let (post, receipt) = authority::project_final_chord_encoding_plan(state.document(), &plan);
    let post = post.unwrap();
    let chains = plan.continue_original_respelling_chains(state.document(), &post, receipt.as_ref());
    (post, receipt, chains)
}

#[test]
fn real_original_final_chord_owner_carries_received_obligations_for_every_copy() {
    for shift in [0, 384] {
        for copies in [1, 2] {
            for renamed in [false, true] {
                let rows = (0..copies).flat_map(|copy| [
                    (101 + copy * 3, shift + copy as i64 * 384, 96, 1, 72),
                    (102 + copy * 3, shift + copy as i64 * 384 + 96, 96, 1, 74),
                    (103 + copy * 3, shift + copy as i64 * 384, 192, 2, 48),
                ]).collect::<Vec<_>>();
                let state = received(&rows, copies + usize::from(shift > 0));
                let (post, receipt) = execute(&state, renamed);
                let mut owners = crate::master_state_identity::OwnerReceipts::default();
                owners.push_exact(receipt);
                let next = owners.continue_owned_rhythm(&state, &post).unwrap();
                next.require_complete_course_context(&post).unwrap();
                assert!(next.same_identities_as(&state));
                assert_eq!(next.initial_custody.snapshot, state.initial_custody.snapshot);
            }
        }
    }
}

#[test]
fn final_chord_original_execution_cannot_be_replaced_by_equal_bytes() {
    let state = received(&[(211, 0, 96, 1, 72), (212, 96, 96, 1, 74)], 1);
    let (post, receipt) = execute(&state, true);
    let receipt = receipt.unwrap();
    let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
    state.continue_final_chord(&receipt, capture).unwrap();
    let (other_post, other_receipt) = execute(&state, true);
    assert_eq!(post, other_post);
    assert!(state.continue_final_chord(&other_receipt.unwrap(), capture).is_err());
    assert!(capture.validate_execution(&receipt, &(state.document().to_owned() + " ")).is_err());
}

#[test]
fn original_auxiliary_materialization_retains_its_received_course() {
    for copies in [1, 2] {
        let rows = (0..copies).flat_map(|copy| [
            (301 + copy * 3, copy as i64 * 384, 384, 2, 48),
            (302 + copy * 3, copy as i64 * 384, 384, 2, 52),
            (303 + copy * 3, copy as i64 * 384, 384, 1, 72),
        ]).collect::<Vec<_>>();
        let state = received(&rows, copies);
        let mut edits = Vec::new();
        for old in state.occurrences.iter().filter(|old| old.origin.source.is_some_and(|source|
            (0..copies).any(|copy| source == 302 + copy * 3))) {
            let text = &state.snapshot[old.bytes.clone()];
            assert!(text.contains("<chord"));
            let start = old.bytes.start + text.find("<staff>").unwrap() + "<staff>".len();
            let end = start + text[text.find("<staff>").unwrap() + "<staff>".len()..].find("</staff>").unwrap();
            edits.push((start, end, "1".to_owned()));
        }
        let (post, receipt) = authority::apply_atomic_staff_voice_scalar_batch_authorized(state.document(), edits);
        let post = post.unwrap(); let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_exact(receipt);
        assert!(owners.continue_owned_rhythm(&state, &post).unwrap_err().to_string()
            .contains("staff without a course transition"));
        let state = move_sources_with_original_staff_owner(&state,
            &(0..copies).map(|copy| 302 + copy * 3).collect::<Vec<_>>(), 1);
        let (post, receipt) = execute(&state, true);
        let receipt = receipt.unwrap();
        let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
        assert_eq!(capture.rows().iter().filter(|row| row.auxiliary()).count(), copies);
        let next = state.continue_final_chord(&receipt, capture).unwrap();
        next.require_complete_course_context(&post).unwrap();
        assert!(next.same_identities_as(&state));
        assert_eq!(next.coverage.len(), state.coverage.len());
    }
}

#[test]
fn changed_missing_or_permuted_original_maps_are_not_a_course_permit() {
    let state = received(&[(501, 0, 96, 1, 72), (502, 96, 96, 1, 74)], 1);
    let (_, receipt) = execute(&state, true);
    let receipt = receipt.unwrap();
    let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
    let original = capture.rows().to_vec();
    let mut missing = original.clone(); missing.pop();
    let mut duplicate = original.clone(); duplicate.push(original[0].clone());
    let mut permuted = original.clone();
    let pitched = original.iter().enumerate().filter(|(_, row)| row.source.is_some()).map(|(index, _)| index).collect::<Vec<_>>();
    let first = permuted[pitched[0]].outputs.clone();
    permuted[pitched[0]].outputs = permuted[pitched[1]].outputs.clone();
    permuted[pitched[1]].outputs = first;
    let mut foreign_source = original.clone(); foreign_source[pitched[0]].source = original[pitched[1]].source;
    for rows in [missing, duplicate, permuted, foreign_source] {
        let changed = capture.altered_rows_for_test(rows);
        assert!(state.continue_final_chord(&receipt, &changed).unwrap_err().to_string()
            .contains("emission map changed"));
    }
}

#[test]
fn original_rest_forward_and_residual_rest_keep_their_own_full_obligation() {
    for partial in [false, true] {
        for copies in [1, 2] {
            let rows = (0..copies).flat_map(|copy| {
                let mut rows = vec![(801 + copy * 2, copy as i64 * 384, 384, 1, 72)];
                if partial { rows.push((802 + copy * 2, copy as i64 * 384 + 192, 192, 1, 74)); }
                rows
            }).collect::<Vec<_>>();
            let state = received(&rows, copies);
            let edits = state.occurrences.iter().filter(|old| old.origin.source.is_some_and(|source|
                (0..copies).any(|copy| source == 801 + copy * 2))).map(|old| {
                let text = &state.snapshot[old.bytes.clone()];
                let start = old.bytes.start + text.find("<staff>").unwrap() + "<staff>".len();
                let end = start + text[text.find("<staff>").unwrap() + "<staff>".len()..].find("</staff>").unwrap();
                (start, end, "2".to_owned())
            }).collect();
            let (post, receipt) = authority::apply_atomic_staff_voice_scalar_batch_authorized(state.document(), edits);
            let post = post.unwrap(); let mut owners = crate::master_state_identity::OwnerReceipts::default();
            owners.push_exact(receipt);
            assert!(owners.continue_owned_rhythm(&state, &post).unwrap_err().to_string()
                .contains("staff without a course transition"));
            let state = move_sources_with_original_staff_owner(&state,
                &(0..copies).map(|copy| 801 + copy * 2).collect::<Vec<_>>(), 2);
            let (post, receipt) = execute(&state, false);
            let receipt = receipt.unwrap();
            let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
            let cursor_rows = capture.rows().iter().filter(|row| row.outputs().iter().any(|(_, cursor)| *cursor)).collect::<Vec<_>>();
            assert_eq!(cursor_rows.len(), copies);
            assert!(cursor_rows.iter().all(|row| row.source().is_none()));
            if partial { assert!(cursor_rows.iter().all(|row| row.outputs().iter().any(|(_, cursor)| !*cursor))); }
            let next = state.continue_final_chord(&receipt, capture).unwrap();
            next.require_complete_course_context(&post).unwrap();
            assert!(next.same_identities_as(&state));
            assert_eq!(next.nonphysical_rest_obligation_count(), copies);
            assert_eq!(next.reparameterized_obligation_count(), 0);
            for encoded in &next.encoded {
                let old = state.occurrences.iter().find(|old| old.creation == encoded.received.creation).unwrap();
                assert_eq!(old.origin, encoded.received.origin);
            }
        }
    }
}

#[test]
fn original_execution_and_ordinals_survive_a_later_real_owner() {
    let state = received(&[(901, 0, 96, 1, 72), (902, 96, 96, 1, 74)], 1);
    let (post, receipt) = execute(&state, false);
    let receipt = receipt.unwrap();
    let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
    let encoded = state.continue_final_chord(&receipt, capture).unwrap();
    let next = noncanonical_original_voices(&encoded);
    assert!(next.same_identities_as(&state));
    next.require_complete_course_context(next.document()).unwrap();
    assert_eq!(encoded.encoded.len(), next.encoded.len());
    for (before, after) in encoded.encoded.iter().zip(&next.encoded) {
        assert!(before.original_emissions.iter().flatten().all(|emission|
            emission.execution == EncodingExecution::FinalChord(*receipt.ssc_bound_origin().0)));
        assert_eq!(before.original_emissions, after.original_emissions);
        assert_eq!(before.emission_count, after.emission_count);
        assert_eq!(before.fragments.iter().map(|fragment| fragment.ordinal).collect::<Vec<_>>(),
            after.fragments.iter().map(|fragment| fragment.ordinal).collect::<Vec<_>>());
    }
    assert_ne!(post, next.document());
}


#[test]
fn original_retained_chord_copy_composes_with_actual_canonical_tied_encoding() {
    for copies in [1, 2] {
        let rows = (0..copies).flat_map(|copy| [(7101 + copy * 2, copy as i64 * 384 + 24, 1, 72),
            (7102 + copy * 2, copy as i64 * 384 + 24, 1, 76)]).collect::<Vec<_>>();
        let state = OwnedRhythmicContinuation::synthetic_pitched_received_for_test(&rows, copies);
        let mut normalized = normalize_visible_rhythm_musicxml(state.document());
        let capture = normalized.stats.physical_births.pop();
        let mut owners = crate::master_state_identity::OwnerReceipts::default();
        owners.push_visible_block_with_births(normalized.visible_rhythm_block_identity.take(),
            capture, state.document()).unwrap();
        let shifted = owners.continue_owned_rhythm(&state, &normalized.xml).unwrap();
        shifted.require_complete_course_context(&normalized.xml).unwrap();
        assert_eq!(shifted.reparameterized_obligation_count(), copies);
        assert!(shifted.final_chord_execution.is_none());
        let (post, receipt, _) = execute_with_original_chains(&shifted, false);
        let receipt = receipt.unwrap();
        let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
        assert_eq!(capture.rows().iter().filter(|row| row.tied_chain()).count(), copies);
        let next = shifted.continue_final_chord(&receipt, capture).unwrap();
        next.require_complete_course_context(&post).unwrap();
        assert_eq!(next.reparameterized_obligation_count(), 0);
        assert!(next.final_chord_execution.is_some());
        assert!(next.same_identities_as(&shifted));
        let (later, later_receipt) = execute(&next, true);
        if let Some(later_receipt) = later_receipt {
            let later_capture = later_receipt.final_chord_capture().unwrap().as_ref().unwrap();
            assert!(next.continue_final_chord(&later_receipt, later_capture).is_err());
        } else { assert_eq!(later, next.document()); }
    }
}

#[test]
fn retained_copy_requires_the_original_visible_execution_even_for_equal_bytes() {
    let state = OwnedRhythmicContinuation::synthetic_pitched_received_for_test(
        &[(9101, 24, 1, 72), (9102, 24, 1, 76)], 1);
    let first = normalize_visible_rhythm_musicxml(state.document());
    let second = normalize_visible_rhythm_musicxml(state.document());
    assert_eq!(first.xml, second.xml);
    let capture = first.stats.physical_births.first().unwrap().clone();
    let foreign = second.visible_rhythm_block_identity.unwrap();
    let mut owners = crate::master_state_identity::OwnerReceipts::default();
    assert!(owners.push_visible_block_with_births(Some(foreign), Some(capture.clone()), state.document())
        .unwrap_err().to_string().contains("Foreign original visible emission execution"));
    let original = first.visible_rhythm_block_identity.unwrap();
    owners.push_visible_block_with_births(Some(original), Some(capture.clone()), state.document()).unwrap();
    let shifted = owners.continue_owned_rhythm(&state, &first.xml).unwrap();
    assert!(shifted.continue_visible(&capture).is_err());
    assert!(capture.block_post_document(&normalize_visible_rhythm_musicxml(state.document())
        .visible_rhythm_block_identity.unwrap(), state.document()).is_err());
}

#[test]
fn composed_origin_images_reject_gaps_aliases_overflow_and_changed_emissions() {
    let copy = OriginImage::new(24..384, 288..648).unwrap();
    let first = copy.compose(&OriginImage::translated(24..288, 264).unwrap()).unwrap().unwrap();
    let last = copy.compose(&OriginImage::translated(288..384, 264).unwrap()).unwrap().unwrap();
    assert_eq!(first.origin, 24..288);assert_eq!(first.image, 24..288);
    assert_eq!(last.origin, 288..384);assert_eq!(last.image, 288..384);
    exact_partition(vec![last.origin.clone(), first.origin.clone()], &(24..384)).unwrap();
    assert!(exact_partition(vec![first.origin.clone(), first.origin], &(24..384)).is_err());
    assert!(exact_partition(vec![last.origin], &(24..384)).is_err());
    assert!(OriginImage::new(24..384, 288..647).is_err());
    assert!(OriginImage::translated((i64::MAX - 1)..i64::MAX, 1).is_err());
    let (state, _, _) = OwnedRhythmicContinuation::original_retained_chord_export_for_test(
        &[(9201, 24, 1, 72), (9202, 24, 1, 76)], 1);
    let mut altered = state.clone();altered.final_chord_execution = None;
    assert!(altered.validate_binding(altered.document()).is_err());
    let mut altered = state.clone();altered.encoded[0].fragments[0].correspondence.origin.start += 1;
    assert!(altered.validate_binding(altered.document()).is_err());
    let mut altered = state.clone();altered.encoded[0].fragments[0].original_emissions.clear();
    assert!(altered.validate_binding(altered.document()).is_err());
    let mut altered = state.clone();altered.encoded[0].fragments[0].ordinal += 1;
    assert!(altered.validate_binding(altered.document()).is_err());
}
