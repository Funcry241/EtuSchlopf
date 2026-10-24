use super::*;
use crate::midi_seed::rhythmic_course_creation_tests::arrangement_pitched_fixture;

fn physical_chord_fixture(split: bool) -> String {
    let mut rows = vec![(101, 0, 1, 60), (102, 0, 1, 64), (103, 0, 1, 67), (104, 0, 1, 71)];
    if !split {
        rows.extend([(105, 96, 1, 72), (106, 192, 1, 74), (107, 288, 1, 76)]);
    }
    let (xml, courses) = arrangement_pitched_fixture(&rows, 1);
    let arranged = arrange_piano_musicxml_with_profile(&xml, ArrangementProfile {
        rhythmic_courses: Some(&courses), preserve_reference: true,
        ..ArrangementProfile::default()
    });
    arranged.stats.rhythmic_courses.as_ref().unwrap().validate_binding(&arranged.xml).unwrap();
    let mut xml = arranged.xml;
    // The real recovery path exposes selected chord tones before its gesture
    // and pitch consumers. Use that owner, preserving all physical tie roles.
    let mut exposed = collect_hidden_chord_tone_recovery_candidates(&xml, 4096)
        .into_iter().filter(|tone| tone.midi != Some(60)).collect::<Vec<_>>();
    exposed.sort_by_key(|tone| std::cmp::Reverse(tone.abs_start));
    for tone in exposed {
        let (changed, receipt) = remove_note_hidden_attrs_in_range(&mut xml, tone.abs_start, tone.abs_end);
        assert!(changed && receipt.is_some());
    }
    xml
}

#[test]
fn actual_split_members_are_protected_before_gauss_and_broken_writes() {
    let xml = physical_chord_fixture(true);
    let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
    let slots = collect_visible_chord_tone_swap_slots(&xml);
    assert!(!hidden.is_empty() && !slots.is_empty());
    assert!(hidden.iter().all(|tone| !single_note_gesture_support_is_independent(&xml[tone.abs_start..tone.abs_end])));
    let (gauss, changes, _, _, carriers, _) = consensus_probe_apply_gauss_gesture_distribution_inner(&xml, &hidden, &slots, false);
    assert_eq!(gauss, xml);
    assert_eq!(changes, 0);
    assert!(carriers.is_empty());
    for mode in 0..3 {
        let (broken, changes, _, carriers, _) = consensus_probe_apply_broken_gesture_distribution(&xml, &hidden, &slots, mode);
        assert_eq!(broken, xml);
        assert_eq!(changes, 0);
        assert!(carriers.is_empty());
    }
}

/// Executes the productive exchange steps through the same frozen owners as the
/// harmonic micro-delta portfolio and returns their receipts in write order.
fn exchange_print_state(xml: &mut String, hidden: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot) -> Option<master_state_identity::OwnerReceipts> {
    let steps = coincident_chord_tone_print_exchange_steps(xml, hidden, slot)?;
    let mut receipts = master_state_identity::OwnerReceipts::default();
    for (start, end, show) in steps {
        let (changed, receipt) = if show {
            crate::structural_write_authority::remove_note_hidden_attrs_in_range_authorized(xml, start, end)
        } else {
            let (changed, receipt) = crate::structural_write_authority::hide_note_print_object_authorized(xml, start, end);
            (changed.ok()?, receipt)
        };
        if !changed || receipt.is_none() {
            return None;
        }
        receipts.push_exact(receipt);
    }
    Some(receipts)
}

#[test]
fn visibility_exchange_refuses_each_tied_shown_tone_without_mutation_or_receipt() {
    let xml = physical_chord_fixture(true);
    let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
    let slots = collect_visible_chord_tone_swap_slots(&xml);
    assert!(!slots.is_empty());
    for slot in slots {
        let mut changed = xml.clone();
        assert!(exchange_print_state(&mut changed, &hidden[0], &slot).is_none());
        assert_eq!(changed, xml);
    }
}

/// Physical XML pitches and print state; this alone does not prove native sound.
fn played_tones(xml: &str) -> Vec<(Option<i32>, bool)> {
    let mut tones = Vec::new();
    let mut scan = 0;
    while let Some(relative) = xml[scan..].find("<note") {
        let start = scan + relative;
        let end = start + xml[start..].find("</note>").unwrap() + "</note>".len();
        let note = &xml[start..end];
        if note.contains("<pitch>") {
            tones.push((note_block_midi(note), !note.contains("print-object=\"no\"")));
        }
        scan = end;
    }
    tones
}

#[test]
fn visibility_exchange_keeps_every_played_tone_and_moves_only_the_print_state() {
    let xml = coincident_chord_measure(3);
    let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
    let slots = collect_visible_chord_tone_swap_slots(&xml);
    assert!(!hidden.is_empty() && !slots.is_empty());
    let mut changed = xml.clone();
    let receipts = exchange_print_state(&mut changed, &hidden[0], &slots[0]).unwrap();
    assert_eq!(receipts.len(), 2);
    parse_musicxml(&changed).unwrap();
    let (before, after) = (played_tones(&xml), played_tones(&changed));
    // Same played tones in the same order: nothing lost, nothing doubled.
    assert_eq!(before.iter().map(|t| t.0).collect::<Vec<_>>(), after.iter().map(|t| t.0).collect::<Vec<_>>());
    let flipped = before.iter().zip(&after).filter(|(b, a)| b.1 != a.1).map(|(b, a)| (b.0, a.1)).collect::<Vec<_>>();
    assert_eq!(flipped.len(), 2);
    assert!(flipped.contains(&(hidden[0].midi, true)) && flipped.contains(&(slots[0].midi, false)));
    assert!(visibility_exchange_only(&xml, &changed, slots[0].measure));
}

#[test]
fn tied_concealed_tone_is_not_shown_by_the_exchange() {
    let xml = physical_chord_fixture(false);
    let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
    assert!(!hidden.is_empty());
    let close = hidden[0].abs_end - "</note>".len();
    let mut tied = xml.clone();
    tied.insert_str(close, "<tie type=\"start\"/>");
    let hidden = collect_hidden_chord_tone_recovery_candidates(&tied, 4096);
    let slots = collect_visible_chord_tone_swap_slots(&tied);
    let mut changed = tied.clone();
    assert!(exchange_print_state(&mut changed, &hidden[0], &slots[0]).is_none());
    assert_eq!(changed, tied);
}

#[test]
fn unsafe_visibility_exchange_refuses_without_changing_selected_course_custody() {
    let rows = [(101, 0, 1, 60), (102, 0, 1, 64), (103, 0, 1, 67), (104, 0, 1, 71),
        (105, 96, 1, 72), (106, 192, 1, 74), (107, 288, 1, 76)];
    let (seed_xml, courses) = arrangement_pitched_fixture(&rows, 1);
    let arranged = arrange_piano_musicxml_with_profile(&seed_xml, ArrangementProfile {
        rhythmic_courses: Some(&courses), preserve_reference: true,
        ..ArrangementProfile::default()
    });
    let custody = arranged.stats.rhythmic_courses.as_ref().unwrap();
    let mut xml = arranged.xml.clone();
    let received = crate::arrangement::OwnedRhythmicContinuation::receive(custody, &xml).unwrap();
    let mut exposed = collect_hidden_chord_tone_recovery_candidates(&xml, 4096)
        .into_iter().filter(|tone| tone.midi != Some(60)).collect::<Vec<_>>();
    exposed.sort_by_key(|tone| std::cmp::Reverse(tone.abs_start));
    let mut exposure = master_state_identity::OwnerReceipts::default();
    for tone in exposed {
        let (changed, receipt) = remove_note_hidden_attrs_in_range(&mut xml, tone.abs_start, tone.abs_end);
        assert!(changed);
        exposure.push_exact(receipt);
    }
    let state = exposure.continue_owned_rhythm(&received, &xml).unwrap();
    let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
    let slots = collect_visible_chord_tone_swap_slots(&xml);
    let mut post = xml.clone();
    assert!(exchange_print_state(&mut post, &hidden[0], &slots[0]).is_none());
    assert_eq!(post, xml);
    state.require_complete_course_context(&post).unwrap();
    assert!(state.same_identities_as(&received));
}

#[test]
fn independent_sources_still_reach_both_gesture_writers() {
    let xml = physical_chord_fixture(false);
    let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
    let slots = collect_visible_gesture_attack_slots(&xml);
    let (gauss, changes, _, _, carriers, _) = consensus_probe_apply_gauss_gesture_distribution_inner(&xml, &hidden, &slots, false);
    assert!(changes > 0 && !carriers.is_empty());
    assert_ne!(gauss, xml);
    parse_musicxml(&gauss).unwrap();
    let (broken, changes, _, carriers, _) = consensus_probe_apply_broken_gesture_distribution(&xml, &hidden, &slots, 0);
    assert!(changes > 0 && !carriers.is_empty());
    assert_ne!(broken, xml);
    parse_musicxml(&broken).unwrap();
}

#[test]
fn a_concealed_tone_is_shown_at_most_once() {
    let xml = coincident_chord_measure(3);
    let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
    let slots = collect_visible_chord_tone_swap_slots(&xml);
    assert!(slots.len() >= 2);
    let mut once = xml.clone();
    assert!(exchange_print_state(&mut once, &hidden[0], &slots[0]).is_some());
    let shown_again = once.clone();
    // The tone is printed now; a second exchange over stale ranges cannot reuse it.
    assert!(exchange_print_state(&mut once, &hidden[0], &slots[1]).is_none());
    assert_eq!(once, shown_again);
}

#[test]
fn support_independence_ignores_visibility_labels_pitch_and_note_location() {
    for tie in ["<tie type=\"start\"/>", "<tie type='stop'/>", "<notations><tied type=\"stop\"/></notations>"] {
        for label in ["1", "23"] {
            let note = format!("<note print-object=\"no\"><pitch><step>D</step><octave>6</octave></pitch><voice>{label}</voice>{tie}</note>");
            assert!(!single_note_gesture_support_is_independent(&note));
        }
    }
    assert!(single_note_gesture_support_is_independent("<note><pitch><step>C</step><octave>3</octave></pitch></note>"));
}

fn coincident_chord_measure(number: usize) -> String {
    let note = |step: &str, octave: i32, chord: bool, hidden: bool| format!(
        "<note{}>{}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>",
        if hidden { " print-object=\"no\" print-spacing=\"no\"" } else { " print-spacing=\"yes\"" },
        if chord { "<chord/>" } else { "" });
    let notes = [note("C", 4, false, false), note("E", 4, true, false), note("G", 4, true, true),
        note("D", 4, false, false), note("F", 4, true, false), note("A", 4, true, true),
        note("E", 4, false, false), note("G", 4, true, false), note("B", 4, true, true),
        note("F", 4, false, false), note("A", 4, true, false), note("C", 5, true, true)].concat();
    format!("<?xml version=\"1.0\"?><score-partwise><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{number}\"><attributes><divisions>96</divisions><key><fifths>0</fifths></key><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{notes}</measure></part></score-partwise>")
}

#[test]
fn every_harmonic_micro_node_shows_instead_of_overwriting() {
    // Relocated measure numbers must not change the exchange (metamorphic check).
    for number in [3, 41] {
    let xml = coincident_chord_measure(number);
    let graph = build_harmonic_micro_delta_graph(&xml, 24);
    assert_eq!(graph.nodes.len(), 4);
    for node in graph.ordered_micro_variants() {
        let (before, after) = (played_tones(&xml), played_tones(&node.patched_xml));
        assert_eq!(before.iter().map(|t| t.0).collect::<Vec<_>>(), after.iter().map(|t| t.0).collect::<Vec<_>>());
        assert!(after.contains(&(Some(node.to_midi), true)));
        assert!(after.contains(&(node.slot.midi, false)));
        assert!(visibility_exchange_only(&xml, &node.patched_xml, node.slot.measure));
    }
    }
}

#[test]
fn native_unsafe_spacing_refuses_before_either_visibility_writer() {
    for number in [3, 41] {
        for spacing in ["", " print-spacing=\"no\"", " data-print-spacing=\"yes\""] {
            let xml = coincident_chord_measure(number).replace(" print-spacing=\"yes\"", spacing);
            let hidden = collect_hidden_chord_tone_recovery_candidates(&xml, 4096);
            let slots = collect_visible_chord_tone_swap_slots(&xml);
            assert!(!hidden.is_empty() && !slots.is_empty());
            for slot in &slots {
                let mut post = xml.clone();
                assert!(exchange_print_state(&mut post, &hidden[0], slot).is_none());
                assert_eq!(post, xml);
            }
            assert!(build_harmonic_micro_delta_graph(&xml, 24).nodes.is_empty());
        }
    }
}

#[test]
fn native_preserving_spacing_supports_both_quote_styles() {
    for spacing in [" print-spacing=\"yes\"", " print-spacing='yes'"] {
        let xml = coincident_chord_measure(41).replace(" print-spacing=\"yes\"", spacing);
        let graph = build_harmonic_micro_delta_graph(&xml, 24);
        assert_eq!(graph.nodes.len(), 4);
        for node in graph.ordered_micro_variants() {
            let root = roxmltree::Document::parse(&node.patched_xml).unwrap();
            for note in root.descendants().filter(|n| n.has_tag_name("note")) {
                if note.attribute("print-object") == Some("no") && note.attribute("print-spacing") == Some("yes") {
                    assert!(note.children().any(|n| n.has_tag_name("pitch")));
                }
            }
        }
    }
}
