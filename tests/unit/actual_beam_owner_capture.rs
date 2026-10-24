//! Synthetic producer and adoption controls for original beam observations.
use super::*;

pub(crate) fn input() -> String {
    let notes = ["C", "D", "E", "F"].into_iter().map(|step| format!(
        "<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>"
    )).collect::<String>();
    format!(
        "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"probe\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes><note><rest/><duration>288</duration><voice>3</voice><type>half</type><dot/><staff>1</staff></note>{notes}</measure></part></score-partwise>"
    )
}

pub(crate) fn execute(
    before: &str,
) -> (
    String,
    BlockTransformationReceipt,
    std::sync::Arc<ActualCanonicalBeamCapture>,
) {
    let mut after = before.to_string();
    let (receipt, capture) =
        enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after).into_parts_for_test();
    (
        after,
        receipt.expect("actual owner changed the document"),
        capture
            .expect("original owner observation")
            .expect("bound actual decisions"),
    )
}

#[test]
fn actual_owner_carries_both_levels_through_its_rest_completion() {
    let before = input();
    let (after, receipt, capture) = execute(&before);
    assert_ne!(before, after);
    assert!(after.matches("<rest").count() > before.matches("<rest").count());
    assert_ne!(before.find("<pitch>"), after.find("<pitch>"));
    capture.binds_original_receipt(&receipt).unwrap();
    let rows = capture.occurrences(&after).unwrap();
    assert_eq!(rows.len(), 4);
    let groups = rows
        .iter()
        .flat_map(|(_, levels)| {
            levels
                .iter()
                .filter_map(|(level, group, _)| group.map(|group| (*level, group)))
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(groups.len(), 2);
    assert_eq!(
        rows.iter()
            .flat_map(|(_, levels)| levels)
            .filter(|(_, group, _)| group.is_none())
            .count(),
        8
    );
    for (range, levels) in rows {
        let note = roxmltree::Document::parse(&after[range.clone()]).unwrap();
        assert!(
            note.root_element()
                .children()
                .any(|n| n.has_tag_name("pitch"))
        );
        assert_eq!(
            levels
                .iter()
                .map(|(level, _, _)| *level)
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
    }
}

#[test]
fn identical_documents_from_another_execution_do_not_bind() {
    let before = input();
    let (after, first, capture) = execute(&before);
    let (other_after, second, _) = execute(&before);
    assert_eq!(after, other_after);
    assert_ne!(first.ssc_bound_origin().0, second.ssc_bound_origin().0);
    assert!(capture.binds_original_receipt(&second).is_err());
}

#[test]
fn unchanged_note_geometry_cannot_recover_a_stale_document_binding() {
    let (after, _, capture) = execute(&input());
    let stale = after.replace("version=\"4.0\"", "version=\"3.1\"");
    assert_ne!(after, stale);
    assert!(capture.occurrences(&stale).is_err());
}

#[test]
fn missing_actual_carry_retains_original_receipt_as_unknown_observation() {
    let before = input();
    let (after, receipt, _) = execute(&before);
    let identity = *receipt.ssc_bound_origin().0;
    let missing = ActualCanonicalBeamCapture::at_owner(
        &receipt,
        &before,
        &after,
        &ActualBeamNoteMap::default(),
        &ActualBeamNoteMap::default(),
        &[ActualBeamEmission::negative(before.find("<note>").unwrap())],
    );
    assert!(missing.is_err());
    let (retained, observation) =
        crate::master_state_identity::CapturedBlockReceiptInput::observed_beam(receipt, missing)
            .into_parts_for_test();
    assert_eq!(*retained.unwrap().ssc_bound_origin().0, identity);
    assert!(observation.unwrap().is_err());
}

#[test]
fn no_op_does_not_mint_an_owner_event_or_observation() {
    let mut empty = "<score-partwise/>".to_string();
    let (receipt, capture) =
        enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut empty).into_parts_for_test();
    assert!(receipt.is_none());
    assert!(capture.is_none());
}

#[test]
fn original_states_are_carried_per_occurrence_and_level() {
    let (after, _, capture) = execute(&input());
    let rows = capture.occurrences(&after).unwrap();
    for (ordinal, (_, levels)) in rows.iter().enumerate() {
        let expected = if ordinal == 0 {
            "begin"
        } else if ordinal + 1 == rows.len() {
            "end"
        } else {
            "continue"
        };
        for (_, cohort, state) in &levels[..2] {
            assert!(cohort.is_some());
            assert_eq!(state.as_deref(), Some(expected));
        }
        assert!(
            levels[2..]
                .iter()
                .all(|(_, cohort, state)| cohort.is_none() && state.is_none())
        );
    }
}

#[test]
fn meaningful_lyric_whitespace_is_not_discarded_by_the_observer() {
    let note = "<note><pitch><step>C</step><octave>5</octave></pitch><lyric><text> a b </text></lyric></note>";
    let changed = note.replace(" a b ", "a b");
    assert_ne!(
        actual_beam_note_signature(note, true).unwrap(),
        actual_beam_note_signature(&changed, true).unwrap()
    );
    let formatted = note
        .replace("<pitch>", "\n <pitch>")
        .replace("</pitch>", "</pitch>\n ");
    assert_eq!(
        actual_beam_note_signature(note, true).unwrap(),
        actual_beam_note_signature(&formatted, true).unwrap()
    );
}

#[test]
fn altered_owner_plan_cannot_certify_the_actual_written_states() {
    let before = input();
    let mut middle = before.clone();
    let mut beams = enforce_beat_scoped_beaming_in_xml(&mut middle);
    let mut after = middle;
    let rests = enforce_beat_scoped_rest_completion_in_xml(&mut after);
    let receipt = BlockTransformationReceipt::mint_block_receipt(
        BeatScopedBeamingBlockMint { _sealed: () },
        &before,
        &after,
        None,
    )
    .unwrap();
    beams.actual_beam_emissions[0].edits[0].2 = "end".into();
    assert!(
        ActualCanonicalBeamCapture::at_owner(
            &receipt,
            &before,
            &after,
            &beams.actual_note_map,
            &rests.actual_note_map,
            &beams.actual_beam_emissions
        )
        .is_err()
    );
}

#[test]
fn ambiguous_physical_carry_is_not_an_occurrence_identity() {
    let before = input();
    let mut middle = before.clone();
    let mut beams = enforce_beat_scoped_beaming_in_xml(&mut middle);
    let mut after = middle;
    let rests = enforce_beat_scoped_rest_completion_in_xml(&mut after);
    let receipt = BlockTransformationReceipt::mint_block_receipt(
        BeatScopedBeamingBlockMint { _sealed: () },
        &before,
        &after,
        None,
    )
    .unwrap();
    let member = beams.actual_beam_emissions[0].members[0];
    let carried = beams
        .actual_note_map
        .0
        .iter()
        .find(|(input, _)| input.start == member)
        .unwrap()
        .clone();
    beams.actual_note_map.0.push(carried);
    assert!(
        ActualCanonicalBeamCapture::at_owner(
            &receipt,
            &before,
            &after,
            &beams.actual_note_map,
            &rests.actual_note_map,
            &beams.actual_beam_emissions
        )
        .is_err()
    );
}

#[test]
fn receipt_append_retains_the_original_observation() {
    let before = input();
    let mut after = before;
    let input = enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after);
    let mut child = crate::master_state_identity::OwnerReceipts::default();
    child.push_block(input);
    let mut parent = crate::master_state_identity::OwnerReceipts::default();
    parent.append(child);
    let capture = parent
        .observe_actual_beam_continuation(None)
        .unwrap()
        .unwrap();
    assert_eq!(capture.occurrences(&after).unwrap().len(), 4);
}

#[test]
fn exact_original_insertions_shift_ranges_without_recreating_identity() {
    let (after, _, capture) = execute(&input());
    let offset = after.find("<note>").unwrap();
    let post = format!(
        "{}<!-- independent metadata -->{}",
        &after[..offset],
        &after[offset..]
    );
    let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests(
        &after,
        &post,
        offset,
        offset,
        "<!-- independent metadata -->",
    );
    let carried = capture.carry_exact(&receipt).unwrap();
    assert!(carried.occurrences(&after).is_err());
    assert_eq!(carried.execution, capture.execution);
    let rows = carried.occurrences(&post).unwrap();
    assert_eq!(rows.len(), 4);
    assert_eq!(
        rows[0].0.start,
        capture.occurrences(&after).unwrap()[0].0.start + "<!-- independent metadata -->".len()
    );
}

#[test]
fn a_following_note_change_leaves_the_observation_unknown() {
    let (after, _, capture) = execute(&input());
    let offset = after.find("<step>C</step>").unwrap() + "<step>".len();
    let mut post = after.clone();
    post.replace_range(offset..offset + 1, "G");
    let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests(
        &after,
        &post,
        offset,
        offset + 1,
        "G",
    );
    let mut receipts = crate::master_state_identity::OwnerReceipts::default();
    receipts.push_exact(Some(receipt));
    let observed = receipts
        .observe_actual_beam_continuation(Some(&Ok(capture)))
        .unwrap();
    assert!(observed.is_err());
}

#[test]
fn independent_group_duplication_and_relabeling_preserve_local_decisions() {
    let source = input();
    let doc = roxmltree::Document::parse(&source).unwrap();
    let measure = doc
        .descendants()
        .find(|node| node.has_tag_name("measure"))
        .unwrap();
    let body = &source[measure.range()];
    let doubled = source.replacen(
        body,
        &format!("{body}{}", body.replace("probe", "independent")),
        1,
    );
    let (single_post, _, single) = execute(&source);
    let (double_post, _, double) = execute(&doubled);
    let single_rows = single.occurrences(&single_post).unwrap();
    let rows = double.occurrences(&double_post).unwrap();
    assert_eq!(rows.len(), single_rows.len() * 2);
    for block in rows.chunks(single_rows.len()) {
        for ((_, expected), (_, actual)) in single_rows.iter().zip(block) {
            assert_eq!(
                expected
                    .iter()
                    .map(|(level, _, state)| (level, state))
                    .collect::<Vec<_>>(),
                actual
                    .iter()
                    .map(|(level, _, state)| (level, state))
                    .collect::<Vec<_>>()
            );
        }
    }
    let renamed = doubled
        .replace("probe", "renamed")
        .replace("<voice>3</voice>", "<voice>renamed</voice>");
    let (renamed_post, _, renamed_capture) = execute(&renamed);
    assert_eq!(
        renamed_capture.occurrences(&renamed_post).unwrap().len(),
        rows.len()
    );
}

#[test]
fn original_capture_survives_the_actual_sovereign_master_adoption() {
    use crate::master_state_identity::{MasterLineage, OwnerReceipts};
    let xml = input();
    let genesis = crate::master_state_identity::tests::synthetic_genesis_for_tests(&xml);
    let mut state = crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(
        xml,
        MasterLineage::lineage_from_genesis(genesis),
    );
    state.adopt_in_place_transformation("SyntheticActualBeamOwner", |document| {
        let captured = enforce_beat_scoped_beaming_and_rest_completion_in_xml(document);
        let mut receipts = OwnerReceipts::default();
        receipts.push_block(captured);
        ((), receipts)
    });
    let capture = state.actual_beam_capture().unwrap().as_ref().unwrap();
    assert_eq!(capture.occurrences(state.document()).unwrap().len(), 4);
    assert_eq!(state.lineage().committed_event_count(), 1);
    assert!(state.lineage().lineage_continuity_holds(state.document()));
}

fn final_report_with_actual_beam_observation(spine: &CandidateMaterialLineageSpine) -> String {
    let mut text = crate::final_report::DraftReport::new(
        "TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION\n---------------------------------------\nStatus: synthetic.\n\nSILENT-VOICE-COVERAGE-WRITE-OUTCOME\n-----------------------------------\nRendererOutcome: observed_from_existing_writer_audit.\n".into(),
    ).compact(ToOwned::to_owned);
    for _ in 0..2 {
        project_actual_beam_source_binding_report(&mut text, spine).unwrap();
        text.project_owned_section(
            "candidate_material_lineage_spine",
            "CANDIDATE-MATERIAL-LINEAGE-SPINE",
            "----------------------------------",
            |out| append_candidate_material_lineage_spine_body(out, spine),
        )
        .unwrap();
    }
    let report = text.finalize().unwrap();
    let rendered = report.as_str();
    assert_eq!(
        rendered
            .matches("\nACTUAL-CANONICAL-BEAM-SOURCE-BINDING\n")
            .count(),
        1
    );
    assert_eq!(
        rendered
            .matches("\nCANDIDATE-MATERIAL-LINEAGE-SPINE\n")
            .count(),
        1
    );
    rendered.to_owned()
}

#[test]
fn unknown_actual_beam_observation_finalizes_through_declared_report_producers() {
    let report =
        final_report_with_actual_beam_observation(&CandidateMaterialLineageSpine::default());
    assert!(report.contains("ActualCanonicalBeamSourceBinding: Unknown;"));
    assert!(report.contains("ChannelAllocationPermit none; MusicMutation none;"));
}

#[test]
fn captured_actual_beam_observation_finalizes_through_declared_report_producers() {
    let before = input().replace("probe", &83usize.to_string());
    let (after, _, capture) = execute(&before);
    let evidence = parse_atomic_notes(&after)
        .iter()
        .enumerate()
        .filter_map(|(index, note)| {
            let pitch = note.pitch.as_ref()?.midi;
            Some(
                crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                    fragment_id: index + 1,
                    source_event_id: index + 1,
                    attack_id: index + 1,
                    measure: note.measure.parse().unwrap(),
                    raw_note_index: index,
                    onset: note.start,
                    duration: note.duration,
                    pitch,
                    staff: note.staff.parse().unwrap(),
                    chord_syntax: note.is_chord,
                    tie_start: note.tie_start,
                    tie_stop: note.tie_stop,
                },
            )
        })
        .collect::<Vec<_>>();
    let (canonical, _) =
        crate::structural_write_authority::plan_canonical_final_chord_structure_once(
            &after, &evidence,
        )
        .unwrap();
    let mut spine = CandidateMaterialLineageSpine::default();
    spine
        .frozen_sounding_temporal_lane_plan
        .actual_source_beam_binding =
        crate::structural_write_authority::observe_actual_source_beam_binding(
            &canonical,
            &after,
            Some(&Ok(capture)),
        );
    let report = final_report_with_actual_beam_observation(&spine);
    assert!(report.contains(
        "ActualCanonicalBeamSourceBinding: captured_scope_bound; OriginalOccurrences 4;"
    ));
    assert_eq!(
        report
            .matches("ActualCanonicalBeamSourceBindingRow:")
            .count(),
        4
    );
    assert!(!report.contains("ActualCanonicalBeamSourceBinding: Unknown;"));
}

pub(crate) fn input_with_original_chord() -> String {
    let source = input();
    let start = source.find("<note><pitch>").unwrap();
    let end = start + source[start..].find("</note>").unwrap() + "</note>".len();
    let member = "<note><chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>";
    format!("{}{member}{}", &source[..end], &source[end..])
}

#[test]
fn original_chord_attachment_records_its_actual_head_without_own_beam_inference() {
    let (after, _, capture) = execute(&input_with_original_chord());
    let rows = capture.chord_attachments(&after).unwrap();
    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert!(row.unknown.is_none());
    let head = &capture.occurrences(&after).unwrap()[row.head.unwrap()];
    assert!(after[head.0.clone()].contains("<step>C</step>"));
    let member = &after[row.member.clone()];
    assert!(member.contains("<chord"));
    assert!(!member.contains("<beam"));
    assert!(head.1.iter().any(|(_, group, _)| group.is_some()));
    assert_eq!(capture.occurrences(&after).unwrap().len(), 4);
}

#[test]
fn an_original_cursor_boundary_cannot_borrow_the_previous_head() {
    let before = input_with_original_chord().replace(
        "<note><chord/>",
        "<forward><duration>0</duration></forward><note><chord/>",
    );
    let (after, _, capture) = execute(&before);
    let row = &capture.chord_attachments(&after).unwrap()[0];
    assert!(row.head.is_none());
    assert!(
        row.unknown
            .as_ref()
            .unwrap()
            .contains("No original pitched physical chord head")
    );
}

#[test]
fn an_original_unowned_pitched_head_stays_unknown() {
    let source = input().replace("<rest/>", "<pitch><step>G</step><octave>4</octave></pitch>");
    let end = source.find("</note>").unwrap() + "</note>".len();
    let member = "<note><chord/><pitch><step>A</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>";
    let before = format!("{}{member}{}", &source[..end], &source[end..]);
    let (after, _, capture) = execute(&before);
    let row = &capture.chord_attachments(&after).unwrap()[0];
    assert!(row.head.is_none());
    assert!(
        row.unknown
            .as_ref()
            .unwrap()
            .contains("no captured owner beam decision")
    );
}

#[test]
fn original_attachment_ranges_follow_the_original_insertion_receipt() {
    let (after, _, capture) = execute(&input_with_original_chord());
    let before_member = capture.chord_attachments(&after).unwrap()[0].member.clone();
    let offset = after.find("<note>").unwrap();
    let insertion = "<!-- original operation -->";
    let post = format!("{}{insertion}{}", &after[..offset], &after[offset..]);
    let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests(
        &after, &post, offset, offset, insertion,
    );
    let carried = capture.carry_exact(&receipt).unwrap();
    let row = &carried.chord_attachments(&post).unwrap()[0];
    assert_eq!(row.member.start, before_member.start + insertion.len());
    assert_eq!(&post[row.member.clone()], &after[before_member]);
    assert!(carried.chord_attachments(&after).is_err());
}

#[test]
fn a_changed_member_invalidates_attachment_observation_without_losing_head_capture() {
    let (after, _, capture) = execute(&input_with_original_chord());
    let member = capture.chord_attachments(&after).unwrap()[0].member.clone();
    let offset = member.start + after[member].find("<step>A</step>").unwrap() + "<step>".len();
    let mut post = after.clone();
    post.replace_range(offset..offset + 1, "B");
    let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests(
        &after,
        &post,
        offset,
        offset + 1,
        "B",
    );
    let carried = capture.carry_exact(&receipt).unwrap();
    assert_eq!(carried.occurrences(&post).unwrap().len(), 4);
    assert!(carried.chord_attachments(&post).is_err());
    assert_eq!(capture.execution, carried.execution);
}

#[test]
fn independently_duplicated_chords_keep_separate_original_heads() {
    let source = input_with_original_chord();
    let document = roxmltree::Document::parse(&source).unwrap();
    let measure = document
        .descendants()
        .find(|n| n.has_tag_name("measure"))
        .unwrap();
    let body = &source[measure.range()];
    let doubled = source.replacen(
        body,
        &format!("{body}{}", body.replace("probe", "independent")),
        1,
    );
    let (after, _, capture) = execute(&doubled);
    let rows = capture.chord_attachments(&after).unwrap();
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].head, rows[1].head);
    let original = capture.occurrences(&after).unwrap();
    for row in rows {
        let head = &original[row.head.unwrap()].0;
        assert!(head.end <= row.member.start);
        assert!(after[head.clone()].contains("<step>C</step>"));
    }
}

fn beam_only_input_at(measure: usize) -> String {
    let notes = (0..16).map(|i| format!(
        "<note><pitch><step>{}</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>",
        ["C", "D", "E", "F"][i % 4]
    )).collect::<String>();
    format!(
        "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{measure}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes>{notes}</measure></part></score-partwise>"
    )
}

fn beam_only_input() -> String { beam_only_input_at(83) }

#[test]
fn e053_beam_only_execution_carries_every_complete_note_once() {
    for scope in [83usize, 1083] {
        let before = beam_only_input_at(scope);
        let mut after = before.clone();
        let input = enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after);
        assert_ne!(before, after, "the owner wrote beams");
        assert_eq!(before.matches("<rest").count(), after.matches("<rest").count());
        let mut receipts = crate::master_state_identity::OwnerReceipts::default();
        receipts.push_block(input);
        let births = receipts.physical_birth_readback(&before, &after).unwrap();
        assert_eq!(births.len(), 1);
    }
}

#[test]
fn e053_rest_birth_without_capture_refuses_the_carry() {
    let before = input();
    let mut after = before.clone();
    let (receipt, capture) = enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after).into_parts_for_test();
    assert!(after.matches("<rest").count() > before.matches("<rest").count());
    let (receipt, capture) = (receipt.unwrap(), capture.unwrap().unwrap());
    assert!(capture.course_carry_births(&receipt).is_err(), "an uncaptured rest birth must stay Unknown");
    let mut receipts = crate::master_state_identity::OwnerReceipts::default();
    receipts.push_block(crate::master_state_identity::CapturedBlockReceiptInput::from(Some(receipt)));
    assert!(receipts.physical_birth_readback(&before, &after).is_err());
}

#[test]
fn e053_carry_refuses_missing_duplicate_and_foreign_mappings() {
    let before = beam_only_input();
    let (after, receipt, _) = execute(&before);
    let pairs = partial_beat_raw_notes(&before).into_iter()
        .zip(partial_beat_raw_notes(&after))
        .map(|(a, b)| (a.start..a.end, b.start..b.end))
        .collect::<Vec<_>>();
    let carry = |carried: Vec<(std::ops::Range<usize>, std::ops::Range<usize>)>, pre: &str, post: &str|
        crate::arrangement::physical_births::PhysicalRhythmicBirths::beat_scoped_carry(
            &receipt, pre, post, carried);
    carry(pairs.clone(), &before, &after).unwrap();
    carry(pairs[1..].to_vec(), &before, &after).unwrap_err();
    let mut duplicated = pairs.clone();
    duplicated[1] = duplicated[0].clone();
    carry(duplicated, &before, &after).unwrap_err();
    let mut shifted = pairs.clone();
    shifted[0].1 = shifted[0].1.start + 1..shifted[0].1.end;
    carry(shifted, &before, &after).unwrap_err();
    carry(pairs, &after, &before).unwrap_err();
}

#[test]
fn course_carry_from_equal_bytes_still_requires_the_original_execution() {
    let before = beam_only_input();
    let (after, receipt, capture) = execute(&before);
    capture.course_carry_births(&receipt).unwrap();
    let (other_after, other_receipt, _) = execute(&before);
    assert_eq!(after, other_after);
    assert!(capture.course_carry_births(&other_receipt).is_err());
}

#[test]
fn permuted_emitter_carry_cannot_continue_received_occurrence_support() {
    let rows = (0..16).map(|i| (100 + i, i as i64 * 24, 1, 72 + (i % 4) as i32)).collect::<Vec<_>>();
    let state = crate::arrangement::OwnedRhythmicContinuation::synthetic_pitched_received_for_test(&rows, 1);
    let (after, receipt, capture) = execute(state.document());
    let mut tampered = (*capture).clone();
    let mut pairs = tampered.course_carry.as_ref().clone();
    let pitched = pairs.iter().enumerate().filter(|(_, (input, _))|
        state.document()[input.clone()].contains("<pitch>")).map(|(i, _)| i).collect::<Vec<_>>();
    let (a, b) = (pitched[0], pitched[1]);
    let output = pairs[a].1.clone();
    pairs[a].1 = pairs[b].1.clone();
    pairs[b].1 = output;
    tampered.course_carry = std::sync::Arc::new(pairs);
    let mut receipts = crate::master_state_identity::OwnerReceipts::default();
    receipts.push_block(crate::master_state_identity::CapturedBlockReceiptInput::observed_beam(
        receipt, Ok(tampered)));
    let error = receipts.continue_owned_rhythm(&state, &after).unwrap_err().to_string();
    assert!(error.contains("Retained occurrence changed licensed support"), "{error}");
}


#[test]
fn original_visibility_only_rest_emission_retains_complete_carry() {
    for label in [83usize, 1083] {
        for voice in ["lane", "renamed-lane"] {
            let pitch = |step: &str, staff: usize| format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>96</duration><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff></note>");
            let body = format!("<attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{}{}{}<forward><duration>96</duration></forward><backup><duration>384</duration></backup><note><rest/><duration>384</duration><voice>aux</voice><type>whole</type><staff>1</staff></note>",pitch("C",1),pitch("D",2),pitch("E",1));
            for copies in [1,2] {
                let measures=(0..copies).map(|i|format!("<measure number=\"{}\">{body}</measure>",label+i)).collect::<String>();
                let before=format!("<score-partwise><part id=\"P1\">{measures}</part></score-partwise>");
                let mut after=before.clone();
                let input=enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after);
                let expected=before.replace("<note><rest/>","<note print-object=\"no\" print-spacing=\"no\"><rest/>");
                assert_eq!(after,expected,"only the original auxiliary-rest visibility write occurred");
                assert_eq!(partial_beat_raw_notes(&before).len(),4*copies);
                assert_eq!(partial_beat_raw_notes(&after).len(),4*copies);
                let mut receipts=crate::master_state_identity::OwnerReceipts::default();
                receipts.push_block(input);
                let births=receipts.physical_birth_readback(&before,&after).expect("original visibility write carries every complete note");
                assert_eq!(births.len(),1);
                assert!(receipts.physical_birth_readback(&(before.clone()+" "),&after).is_err());
            }
        }
    }
}

#[test]
fn original_uncaptured_rest_split_keeps_its_refusal_after_receipt_append() {
    let state=crate::arrangement::OwnedRhythmicContinuation::synthetic_pitched_received_for_test(
        &[(71,288,1,72),(72,312,1,74),(73,336,1,76),(74,360,1,77)],1);
    let before=state.document();
    let mut after=before.to_owned();
    let input=enforce_beat_scoped_beaming_and_rest_completion_in_xml(&mut after);
    assert!(partial_beat_raw_notes(&after).len()>partial_beat_raw_notes(before).len());
    let mut inner=crate::master_state_identity::OwnerReceipts::default();
    inner.push_block(input);
    let mut receipts=crate::master_state_identity::OwnerReceipts::default();
    receipts.append(inner);
    let error=receipts.continue_owned_rhythm(&state,&after).unwrap_err().to_string();
    assert!(error.contains("Original BLOCK course capture refused"),"{error}");
    assert!(error.contains("Original complete-note carry"),"{error}");
    assert!(error.contains("retain every complete note exactly once"),"{error}");
    state.require_complete_course_context(before).unwrap();
}

/// Synthetic producer-side notation relation for exact owner fixtures.
pub(crate) fn notation_voice_bindings(
    original: &str, rows: impl IntoIterator<Item = ((usize, usize), String)>,
) -> OriginalNotationVoiceBindings {
    OriginalNotationVoiceBindings { original: std::sync::Arc::from(original), voices: rows.into_iter().collect() }
}
