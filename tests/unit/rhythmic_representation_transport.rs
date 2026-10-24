use super::*;

fn preserving_evidence(before: &str, after: &str, present: bool) -> String {
    preserving_stage_evidence::<crate::pipeline::NotationPolishPreservingMint>(before, after, present)
}

fn document(duration: i64, label: &str) -> String {
    format!("<score-partwise><part id=\"P1\"><measure number=\"{label}\"><note><rest/><duration>{duration}</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>")
}

#[test]
fn preserving_evidence_distinguishes_noop_missing_and_present_receipts() {
    let before = document(24, "alpha");
    let after = before.replace("</note>", "<beam number=\"1\">begin</beam></note>");
    let noop = preserving_evidence(&before, &before, false);
    assert!(noop.contains("changed=false; receipt_present=false"));
    let missing = preserving_evidence(&before, &after, false);
    assert!(missing.contains("changed=true; receipt_present=false"));
    assert!(missing.contains("projection_equal=true"));
    assert!(preserving_evidence(&before, &after, true).contains("receipt_present=true"));
}

#[test]
fn preserving_evidence_exposes_actual_conservation_difference_and_parse_errors() {
    let before = document(24, "alpha");
    let after = document(48, "alpha");
    let evidence = preserving_evidence(&before, &after, false);
    assert!(evidence.contains("projection_difference_line=2"));
    assert!(evidence.contains("N|rest|24|"));
    assert!(evidence.contains("N|rest|48|"));
    for (pre, post) in [("<broken>", before.as_str()), (before.as_str(), "<broken>")] {
        let evidence = preserving_evidence(pre, post, false);
        assert!(evidence.contains("parse_error=Some("));
        assert!(evidence.contains("projection_available=false"));
        assert!(evidence.contains("source_context="));
    }
}

#[test]
fn preserving_evidence_remains_independent_of_duplicate_observations_and_labels() {
    let before = document(24, "alpha");
    let after = document(48, "alpha");
    let evidence = preserving_evidence(&before, &after, false);
    let unrelated = document(96, "unrelated");
    preserving_evidence(&unrelated, &unrelated, false);
    assert_eq!(evidence, preserving_evidence(&before, &after, false));
    let renamed = preserving_evidence(&document(24, "other_label"), &document(48, "other_label"), false);
    assert_eq!(evidence.split("projection_difference_line=").last(),
        renamed.split("projection_difference_line=").last());
}

fn transport(before: &str, after: &str, edits: Vec<(Range<usize>, String)>) -> RhythmicRepresentationTransport {
    RhythmicRepresentationTransport { steps: vec![RepresentationStep::recorded(before, after, edits)] }
}

#[test]
fn missing_actual_operation_cannot_transport_a_changed_state() {
    assert!(transport("abc", "axbc", vec![]).carry("abc", "axbc", std::iter::once(1..2).collect()).is_err());
}

#[test]
fn occurrence_boundary_crossing_is_not_a_note_replacement() {
    assert!(transport("abcd", "ad", vec![(1..3, String::new())])
        .carry("abcd", "ad", std::iter::once(0..2).collect()).is_err());
    assert!(transport("abcd", "abxcd", vec![(2..2, "x".into())])
        .carry("abcd", "abxcd", std::iter::once(1..3).collect()).is_err());
}

#[test]
fn overlapping_operations_and_stale_intermediate_states_fail_closed() {
    assert!(transport("abcd", "xd", vec![(0..2, "x".into()), (1..3, String::new())])
        .carry("abcd", "xd", std::iter::once(3..4).collect()).is_err());
    let chain = RhythmicRepresentationTransport { steps: vec![
        RepresentationStep::recorded("abc", "xabc", vec![(0..0, "x".into())]),
        RepresentationStep::recorded("yabc", "yabc", vec![])] };
    assert!(chain.carry("abc", "yabc", std::iter::once(1..2).collect()).is_err());
}

#[test]
fn boundary_insertions_and_exact_replacements_carry_each_independent_occurrence() {
    let proof = transport("aabb", "xaaBBBz", vec![(0..0, "x".into()), (2..4, "BBB".into()), (4..4, "z".into())]);
    assert_eq!(proof.carry("aabb", "xaaBBBz", vec![0..2, 2..4]).unwrap(), vec![1..3, 3..6]);
}

#[test]
fn original_octave_receipt_refuses_missing_stale_foreign_and_rhythmic_journals() {
    let before = document(24, "renamed");
    let doc = roxmltree::Document::parse(&before).unwrap();
    let span = doc.descendants().find(|n| n.has_tag_name("measure")).unwrap().range();
    let anchor = before.find("<note").unwrap();
    let direction = "<direction><direction-type><octave-shift type=\"up\" size=\"8\"/></direction-type></direction>";
    let replacement = format!("{}{}{}", &before[span.start..anchor], direction, &before[anchor..span.end]);
    let (after, receipt) = crate::structural_write_authority::replace_bound_octave_measure_authorized(
        &before, span.start, span.end, replacement);
    let after = after.unwrap(); let receipt = receipt.unwrap();
    let journal = || RepresentationStep::recorded(&before, &after, vec![(anchor..anchor, direction.into())]);
    let valid = RhythmicRepresentationTransport::octave_notation(journal(), &receipt).unwrap();
    let note_end = before.find("</note>").unwrap() + 7;
    let ranges = valid.carry(&before, &after, std::iter::once(anchor..note_end).collect()).unwrap();
    assert_eq!(&before[anchor..note_end], &after[ranges[0].clone()]);
    let foreign = format!("{after} ");
    let refusal = valid.carry(&before, &foreign, std::iter::once(anchor..note_end).collect()).unwrap_err().to_string();
    assert!(refusal.starts_with("Foreign representation post-state; "));
    assert!(refusal.contains(&format!("expected_sha256={}", crate::sha256::hex(after.as_bytes()))));
    assert!(refusal.contains(&format!("observed_sha256={}", crate::sha256::hex(foreign.as_bytes()))));
    assert_eq!(valid.carry(&before, &after, std::iter::once(anchor..note_end).collect()).unwrap(), ranges);

    for wrong in [
        RepresentationStep::recorded(&before, &after, vec![]),
        RepresentationStep::recorded(&format!("{before} "), &after, vec![(anchor..anchor, direction.into())]),
        RepresentationStep::recorded(&before, &format!("{after} "), vec![(anchor..anchor, direction.into())]),
        RepresentationStep::recorded(&before, &after, vec![(0..0, direction.into())]),
    ] { assert!(RhythmicRepresentationTransport::octave_notation(wrong, &receipt).is_err()); }
    let altered = before[span.clone()].replace("</note>", "<type>quarter</type></note>");
    let (changed, original) = crate::structural_write_authority::replace_bound_octave_measure_authorized(
        &before, span.start, span.end, altered);
    let changed = changed.unwrap();
    let note = &before[anchor..note_end];
    assert!(RhythmicRepresentationTransport::octave_notation(
        RepresentationStep::recorded(&before, &changed, vec![(anchor..note_end,
            note.replace("</note>", "<type>quarter</type></note>"))]), &original.unwrap()).is_err());
}

#[test]
fn local_operation_enclosure_requires_exact_span_and_unchanged_exterior() {
    let local = || RepresentationStep::recorded("abc", "axbc", vec![(1..1, "x".into())]);
    let lifted = local().enclosed("!abc?", "!axbc?", 1..4).unwrap();
    assert_eq!(lifted.carry(2..3).unwrap(), 3..4);
    assert!(local().enclosed("!abc?", "!axbc!", 1..4).is_err());
    assert!(local().enclosed("!abc?", "!axbc?", 0..3).is_err());
}

#[test]
fn recorded_block_cannot_change_complete_note_notation_even_with_equal_duration() {
    let state = crate::arrangement::OwnedRhythmicContinuation::synthetic_received_for_test();
    let before = state.document();
    let start = before.find("<note").unwrap();
    let end = start + before[start..].find("</note>").unwrap() + 7;
    let replacement = before[start..end].replace("</note>", "<notations><articulations><staccato/></articulations></notations></note>");
    let mut after = before.to_owned(); after.replace_range(start..end, &replacement);
    let transport = transport(before, &after, vec![(start..end, replacement)]);
    assert!(state.continue_recorded_block(&transport).is_err());
}

#[test]
fn original_multistep_endpoints_require_every_intermediate_operation() {
    let valid = RhythmicRepresentationTransport { steps: vec![
        RepresentationStep::recorded("abcd", "axbcd", vec![(1..1, "x".into())]),
        RepresentationStep::recorded("axbcd", "axbCcd", vec![(3..3, "C".into())])] };
    assert_eq!(valid.recorded_documents().unwrap(), ("abcd", "axbCcd"));
    let mut stale = valid.clone();
    stale.steps[1].before = "aybcd".into();
    assert!(stale.recorded_documents().unwrap_err().to_string().contains("Discontinuous"));
    let mut missing = valid;
    missing.steps[1].edits.clear();
    assert!(missing.recorded_documents().unwrap_err().to_string().contains("do not reproduce"));
    assert!(RhythmicRepresentationTransport { steps: vec![] }.recorded_documents().is_err());
}


fn original_encoded_state(copies: usize) -> crate::arrangement::OwnedRhythmicContinuation {
    let rows = (0..copies).flat_map(|copy| [
        (1101 + copy * 2, copy as i64 * 384, 96, 1, 72),
        (1102 + copy * 2, copy as i64 * 384 + 96, 288, 1, 74),
    ]).collect::<Vec<_>>();
    crate::arrangement::OwnedRhythmicContinuation::synthetic_source_received_for_test(&rows, copies)
        .original_final_chord_for_test()
}

#[test]
fn encoded_occurrences_cannot_evade_original_representation_and_block_checks() {
    for copies in [1, 2] {
        let state = original_encoded_state(copies);
        let before = state.document();
        let doc = roxmltree::Document::parse(before).unwrap();
        let note = doc.descendants().find(|node| node.has_tag_name("note") &&
            node.children().any(|child| child.has_tag_name("pitch"))).unwrap();
        let step = note.descendants().find(|node| node.has_tag_name("step")).unwrap();
        let value = step.children().find(roxmltree::Node::is_text).unwrap().range();
        let mut after = before.to_owned();
        after.replace_range(value.clone(), "D");
        // Deliberately unissued negative journals: neither grants an owner permit.
        let invalid = transport(before, &after, vec![(value, "D".into())]);
        assert!(state.continue_representation(&after, &invalid).unwrap_err().to_string()
            .contains("occurrence boundary"));

        assert!(note.attribute("print-object").is_none());
        let input = note.range();
        let replacement = format!("<note print-object=\"yes\"{}", &before[input.clone()]["<note".len()..]);
        let mut after = before.to_owned(); after.replace_range(input.clone(), &replacement);
        let invalid = transport(before, &after, vec![(input, replacement)]);
        assert!(state.continue_recorded_block(&invalid).unwrap_err().to_string()
            .contains("changed a complete note"));
    }
}

#[test]
fn actual_layout_noop_keeps_encoded_original_ranges_without_issuing_a_receipt() {
    for copies in [1, 2] {
        let state = original_encoded_state(copies);
        let layout = crate::layout::apply_pianist_layout(state.document(), 1);
        assert_eq!(layout.xml, state.document());
        assert!(layout.preserving_receipt.is_none());
        let next = state.continue_representation(&layout.xml, &layout.rhythmic_transport).unwrap();
        next.require_complete_course_context(&layout.xml).unwrap();
        assert!(next.same_identities_as(&state));
    }
}
