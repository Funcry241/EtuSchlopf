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
    assert_eq!(row.original_head.as_ref(), Some(&head.0));
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
    assert!(row.original_head.is_none());
    assert!(row.original_head_absent);
    assert!(row.original_rest_host.is_none(), "a cursor reset cannot borrow a preceding rest host");
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
    let physical_head = row.original_head.as_ref().expect("original non-beamed chord head");
    assert!(after[physical_head.clone()].contains("<step>G</step>"));
    assert!(!after[physical_head.clone()].contains("<beam"));
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
    let before_head = capture.chord_attachments(&after).unwrap()[0].original_head.clone().unwrap();
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
    let head = row.original_head.as_ref().unwrap();
    assert_eq!(head.start, before_head.start + insertion.len());
    assert_eq!(&post[head.clone()], &after[before_head]);
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
    assert_ne!(rows[0].original_head, rows[1].original_head);
    let original = capture.occurrences(&after).unwrap();
    for row in rows {
        let head = &original[row.head.unwrap()].0;
        assert_eq!(row.original_head.as_ref(), Some(head));
        assert!(head.end <= row.member.start);
        assert!(after[head.clone()].contains("<step>C</step>"));
    }
}

// Pure operation-observation controls. They issue no receipt and invoke no
// music writer; they do not claim an executed ActualCanonicalBeamCapture.
fn chord_observation_source() -> String {
    "<score-partwise><part id=\"synthetic\"><measure number=\"arbitrary\"><note><pitch><step>C</step><octave>3</octave></pitch><duration>4</duration><voice>renamed</voice><staff>2</staff></note><note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>renamed</voice><staff>1</staff></note></measure></part></score-partwise>".into()
}

#[test]
fn pure_chord_observation_keeps_physical_membership_separate_from_beam_permission() {
    let source = chord_observation_source();
    let notes = partial_beat_raw_notes(&source);
    let map = ActualBeamNoteMap::unchanged(&notes);
    let rows = capture_actual_chord_beam_attachments(&source, &source, &map, &map, &[]).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].original_head, Some(notes[0].start..notes[0].end));
    assert_eq!(rows[0].member, notes[1].start..notes[1].end);
    assert!(rows[0].head.is_none());
    assert!(rows[0].unknown.is_some());
}

#[test]
fn pure_chord_observation_refuses_a_changed_uncaptured_physical_head() {
    let source = chord_observation_source();
    let notes = partial_beat_raw_notes(&source);
    let map = ActualBeamNoteMap::unchanged(&notes);
    let changed = source.replacen("<step>C</step>", "<step>D</step>", 1);
    assert_eq!(source.len(), changed.len());
    assert!(capture_actual_chord_beam_attachments(&source, &changed, &map, &map, &[]).is_err());
}

#[test]
fn pure_chord_observation_does_not_guess_an_ambiguous_physical_head() {
    let source = chord_observation_source();
    let notes = partial_beat_raw_notes(&source);
    let mut beam = ActualBeamNoteMap::unchanged(&notes);
    beam.0.push(beam.0[0].clone());
    let rest = ActualBeamNoteMap::unchanged(&notes);
    let rows = capture_actual_chord_beam_attachments(&source, &source, &beam, &rest, &[]).unwrap();
    assert!(rows[0].original_head.is_none());
    assert!(!rows[0].original_head_absent);
    assert!(rows[0].head.is_none());
    assert!(rows[0].unknown.is_some());
}

#[test]
fn original_rest_host_observation_requires_the_exact_emitted_operation() {
    let source = chord_observation_source().replace(
        "<pitch><step>C</step><octave>3</octave></pitch>", "<rest/>");
    let notes = partial_beat_raw_notes(&source);
    let map = ActualBeamNoteMap::unchanged(&notes);
    let rows = capture_actual_chord_beam_attachments(&source, &source, &map, &map, &[]).unwrap();
    assert_eq!(rows[0].original_rest_host, Some(notes[0].start..notes[0].end));
    assert!(rows[0].original_head.is_none());
    assert!(rows[0].original_head_absent);
    assert!(rows[0].head.is_none());
    let changed = source.replacen("<duration>4</duration>", "<duration>5</duration>", 1);
    assert!(capture_actual_chord_beam_attachments(&source, &changed, &map, &map, &[]).is_err());
    let mut ambiguous = map.clone();
    ambiguous.0.push(ambiguous.0[0].clone());
    assert!(capture_actual_chord_beam_attachments(&source, &source, &ambiguous, &map, &[]).is_err());
    let document = roxmltree::Document::parse(&source).unwrap();
    let measure = document.descendants().find(|node| node.has_tag_name("measure")).unwrap();
    let body = &source[measure.range()];
    let doubled = source.replacen(body, &format!("{body}{}", body.replace("arbitrary", "relocated")), 1);
    let notes = partial_beat_raw_notes(&doubled);
    let map = ActualBeamNoteMap::unchanged(&notes);
    let rows = capture_actual_chord_beam_attachments(&doubled, &doubled, &map, &map, &[]).unwrap();
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].original_rest_host, rows[1].original_rest_host);
    for row in rows {
        assert!(doubled[row.original_rest_host.unwrap()].contains("<rest"));
        assert!(row.head.is_none());
    }
}

#[test]
fn original_rest_host_range_follows_receipts_and_rejects_changed_support() {
    let host = "<note><rest/><duration>384</duration><voice>4</voice><type>whole</type><staff>2</staff></note>";
    let member = "<note print-object=\"no\"><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>384</duration><voice>4</voice><type>whole</type><staff>2</staff></note>";
    let source = input().replace("</attributes>",
        &format!("</attributes>{host}{member}<backup><duration>384</duration></backup>"));
    let (after, _, capture) = execute(&source);
    let row = &capture.chord_attachments(&after).unwrap()[0];
    let original = row.original_rest_host.clone().unwrap();
    let emissions = partial_beat_raw_notes(&after).into_iter()
        .map(|note| note.start..note.end).filter(|range| *range != original).collect::<Vec<_>>();
    let unsupported = capture.carry_final_chord(&after, emissions.iter().map(|range| (range, range))).unwrap();
    let incomplete = &unsupported.chord_attachments(&after).unwrap()[0];
    assert!(incomplete.original_rest_host.is_none());
    assert!(incomplete.original_head_absent);
    assert_eq!(incomplete.member, row.member, "missing support cannot erase its surviving member's negative evidence");
    let at = original.start;
    let insertion = "<!-- carried original rest host -->";
    let post = format!("{}{insertion}{}", &after[..at], &after[at..]);
    use crate::arrangement::representation_transport::{RepresentationStep, RhythmicRepresentationTransport};
    let owner = PartialBeatBeamPreservingMint { _sealed: () };
    let transport = RhythmicRepresentationTransport::partial_beam(&owner,
        RepresentationStep::recorded(&after, &post, vec![(at..at, insertion.to_string())]));
    let recorded = capture.carry_recorded(&transport).unwrap();
    let recorded_host = recorded.chord_attachments(&post).unwrap()[0].original_rest_host.clone().unwrap();
    assert_eq!(&post[recorded_host], &after[original.clone()]);
    let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests(
        &after, &post, at, at, insertion);
    let carried = capture.carry_exact(&receipt).unwrap();
    let rest = carried.chord_attachments(&post).unwrap()[0].original_rest_host.clone().unwrap();
    assert_eq!(rest.start, original.start + insertion.len());
    assert_eq!(&post[rest.clone()], &after[original]);
    let at = rest.start + post[rest].find("384").unwrap();
    let changed = format!("{}383{}", &post[..at], &post[at + 3..]);
    let receipt = crate::master_state_identity::tests::synthetic_exact_receipt_for_tests(
        &post, &changed, at, at + 3, "383");
    let rejected = carried.carry_exact(&receipt).unwrap();
    assert!(rejected.chord_attachments(&changed).is_err());
    assert_eq!(rejected.occurrences(&changed).unwrap().len(),
        carried.occurrences(&post).unwrap().len());
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
                assert_eq!(after,before,"unproven auxiliary-rest elision is an exact no-op");
                assert_eq!(partial_beat_raw_notes(&before).len(),4*copies);
                assert_eq!(partial_beat_raw_notes(&after).len(),4*copies);
                let mut receipts=crate::master_state_identity::OwnerReceipts::default();
                receipts.push_block(input);
                let births=receipts.physical_birth_readback(&before,&after).expect("unchanged original notes require no fabricated birth capture");
                assert_eq!(receipts.len(),0);
                assert!(births.is_empty());
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

#[test]
fn rest_bridge_carry_preserves_independent_complete_beam_groups() {
    use crate::arrangement::representation_transport::{RepresentationStep, RhythmicRepresentationTransport};

    let (before, _, capture) = execute(&beam_only_input());
    let old_rows = capture.occurrences(&before).unwrap();
    let range = old_rows[0].0.clone();
    let replacement = partial_beat_strip_beam_tags(&before[range.clone()]);
    let after = [&before[..range.start], &replacement, &before[range.end..]].concat();
    let edits = vec![(range, replacement)];
    let owner = PartialBeatBeamPreservingMint { _sealed: () };
    let transport = RhythmicRepresentationTransport::partial_beam(
        &owner, RepresentationStep::recorded(&before, &after, edits.clone()),
    );
    let receipt = realize_metric_rest_bridge_receipt(&before, &after, edits);

    assert!(capture.carry_recorded(&transport).is_err());
    let step = transport.rest_bridge_step(&receipt).unwrap();
    let edited = &step.original_operations()[0].0;
    let retained = capture.retain_unaffected(|range| edited.start < range.end && range.start < edited.end);
    let (readback, _) = capture.rest_bridge_readback(&retained, step, &transport, "E053MetricRestBridgeReplay").unwrap();
    assert!(readback.lines().any(|line| line.contains("edited=true; capture=retired;")
        && line.contains("before={1: \"begin\"") && line.contains("after={};")), "{readback}");
    assert!(readback.contains("operations=1; retired_occurrences=4; unknown=0"), "{readback}");
    assert!(readback.contains("present=4; missing=2; unknown=0"), "{readback}");
    let carried = capture.carry_rest_bridge(&receipt, &transport).unwrap();
    assert_eq!(carried.bridge_edges.len(), 4);
    for edge in &carried.bridge_edges {
        assert_ne!(edge.original_left, old_rows[0].0);
        assert_ne!(edge.original_right, old_rows[0].0);
        assert_eq!(&before[edge.original_left.clone()], &after[edge.left.clone()]);
        assert_eq!(&before[edge.original_right.clone()], &after[edge.right.clone()]);
    }

    let document = roxmltree::Document::parse(&after).unwrap();
    let ranges = document.descendants().filter(|node| node.has_tag_name("note"))
        .map(|node| node.range()).collect::<Vec<_>>();
    let missing = carried.bridge_edges[0].left.clone();
    let projected = carried.carry_final_chord(&after,
        ranges.iter().filter(|range| **range != missing).map(|range| (range, range))).unwrap();
    assert_eq!(projected.bridge_edges.len(), 2);
    assert!(projected.bridge_edges.iter().all(|edge| edge.left != missing && edge.right != missing));

    let remaining = carried.occurrences(&after).unwrap();
    assert_eq!(remaining.len(), 12);
    for ((old_range, old_levels), (new_range, new_levels)) in old_rows[4..].iter().zip(remaining) {
        assert_eq!(old_levels, new_levels);
        assert_eq!(&before[old_range.clone()], &after[new_range.clone()]);
    }
}

#[test]
fn final_chord_carry_retires_only_batches_with_unproved_emissions() {
    let (before, _, capture) = execute(&beam_only_input());
    let rows = capture.occurrences(&before).unwrap();
    let first = rows[0].0.clone();
    let changed = before[first.clone()].replace("<step>C</step>", "<step>D</step>");
    assert_ne!(changed, before[first.clone()]);
    let after = [&before[..first.start], &changed, &before[first.end..]].concat();

    let changed_note = capture.carry_final_chord(&after, rows.iter().map(|(range, _)| (range, range))).unwrap();
    let split_note = capture.carry_final_chord(&before, rows[1..].iter().map(|(range, _)| (range, range))).unwrap();
    assert_eq!(changed_note.occurrences(&after).unwrap(), &rows[4..]);
    assert_eq!(split_note.occurrences(&before).unwrap(), &rows[4..]);
}

#[test]
fn rest_edge_readback_separates_written_extension_from_metric_owner_plan() {
    use crate::arrangement::representation_transport::{RepresentationStep, RhythmicRepresentationTransport};

    let input = beam_only_input();
    let third = partial_beat_raw_notes(&input)[2].clone();
    let rest = "<note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>";
    let input = [&input[..third.start], rest, &input[third.end..]].concat();
    let (before, _, capture) = execute(&input);
    let notes = partial_beat_raw_notes(&before);
    let left = &notes[1];
    let right = &notes[3];
    let left_after = partial_beat_rewrite_owned_beam_levels(&left.text,
        &[(1, "continue"), (2, "continue")], 2).unwrap();
    let right_after = partial_beat_rewrite_owned_beam_levels(&right.text,
        &[(1, "end"), (2, "end")], 2).unwrap();
    let after = [&before[..left.start], &left_after, &before[left.end..right.start],
        &right_after, &before[right.end..]].concat();
    let edits = vec![(left.start..left.end, left_after), (right.start..right.end, right_after)];
    let owner = PartialBeatBeamPreservingMint { _sealed: () };
    let transport = RhythmicRepresentationTransport::partial_beam(
        &owner, RepresentationStep::recorded(&before, &after, edits.clone()),
    );
    let receipt = realize_metric_rest_bridge_receipt(&before, &after, edits);
    let step = transport.rest_bridge_step(&receipt).unwrap();
    let retained = capture.retain_unaffected(|range|
        step.original_operations().iter().any(|(edit, _)| edit == range));
    let (report, _) = capture.rest_bridge_readback(&retained, step, &transport, "E053MetricRestBridgeReplay").unwrap();
    let bridges = report.lines().filter(|line| line.starts_with("MetricBeamRestEdge:")).collect::<Vec<_>>();
    assert_eq!(bridges.len(), 2, "{report}");
    assert!(bridges.iter().all(|line| line.contains("before_e050=Missing; metric_plan=OutsideCapturedPlan")), "{report}");
}
