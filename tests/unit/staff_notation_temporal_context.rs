use super::*;

fn pitch(step: &str, octave: u8, duration: u32, voice: &str, extra: &str) -> String {
    format!(
        "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>2</staff>{extra}</note>"
    )
}

fn clef(sign: &str, line: u8) -> String {
    format!(
        "<attributes><clef number=\"2\"><sign>{sign}</sign><line>{line}</line></clef></attributes>"
    )
}

fn streams(reverse: bool, protected: &str) -> String {
    let high = format!(
        "<note><rest/><duration>3</duration><voice>alpha</voice><staff>1</staff></note>{}",
        pitch("F", 5, 1, "alpha", "")
    );
    let low = match protected {
        "tie" => format!(
            "{}{}{}",
            clef("F", 4),
            pitch("C", 3, 3, "beta", "<tie type=\"start\"/>"),
            pitch("C", 3, 1, "beta", "<tie type=\"stop\"/>")
        ),
        "beam" => format!(
            "{}{}{}",
            clef("F", 4),
            pitch("C", 3, 2, "beta", "<beam number=\"1\">begin</beam>"),
            pitch("C", 3, 2, "beta", "<beam number=\"1\">end</beam>")
        ),
        _ => format!("{}{}", clef("F", 4), pitch("C", 3, 4, "beta", "")),
    };
    let lanes = if reverse {
        format!("{low}<backup><duration>4</duration></backup>{high}")
    } else {
        format!("{high}<backup><duration>4</duration></backup>{low}")
    };
    format!(
        "<score-partwise><part id=\"p\"><measure number=\"seed\"><attributes><divisions>1</divisions></attributes>{}{}</measure><measure number=\"span\">{lanes}</measure></part></score-partwise>",
        clef("G", 2),
        pitch("G", 4, 4, "alpha", "")
    )
}

fn local_part(xml: &str) -> &str {
    let (start, end) = score_part_ranges(xml)[0];
    &xml[start..end]
}

#[test]
fn temporal_source_cost_is_invariant_under_equivalent_stream_order() {
    let mut costs = Vec::new();
    for reversed in [false, true] {
        let xml = streams(reversed, "");
        let part = local_part(&xml);
        let surfaces = scan_lower_clef_surfaces(part).unwrap();
        let notes = scan_lower_pitches(part, None);
        let cost =
            score_readability_course(part, &surfaces, &notes, &vec![true; surfaces.len()]).unwrap();
        assert_eq!(cost.ledger_pressure, 60);
        costs.push(cost);
    }
    assert_eq!(costs[0], costs[1]);
}

#[test]
fn temporal_foreign_tie_blocks_the_same_boundary_in_either_encoding() {
    for reverse in [false, true] {
        let xml = streams(reverse, "tie");
        let part = local_part(&xml);
        let target = note_ranges(part)
            .into_iter()
            .find(|(_, _, n)| n.contains("<octave>5</octave>"))
            .unwrap()
            .0;
        assert!(lower_tie_open_at_position(part, target));
        let surfaces = scan_lower_clef_surfaces(part).unwrap();
        let notes = scan_lower_pitches(part, None);
        let solution = solve_readability_course(part, &surfaces, &notes).unwrap();
        assert!(!solution.insertions.iter().any(|(at, _)| *at == target));
    }
}

#[test]
fn temporal_foreign_complete_beam_blocks_the_same_boundary_in_either_encoding() {
    for reverse in [false, true] {
        let xml = streams(reverse, "beam");
        let part = local_part(&xml);
        let target = note_ranges(part)
            .into_iter()
            .find(|(_, _, n)| n.contains("<octave>5</octave>"))
            .unwrap()
            .0;
        assert!(lower_primary_beam_insertion_blocked(part, target));
    }
}

#[test]
fn temporal_course_materialization_receipt_and_reentry_preserve_note_bytes() {
    let mut courses = Vec::new();
    for reverse in [false, true] {
        let mut xml = streams(reverse, "");
        let before = raw_note_fragments(&xml);
        let (result, receipt) = apply_rustig_clef_course_to_buffer(&mut xml);
        result.unwrap();
        assert!(receipt.is_some());
        assert_eq!(raw_note_fragments(&xml), before);
        let committed = xml.clone();
        apply_rustig_clef_course_to_buffer(&mut xml).0.unwrap();
        assert_eq!(xml, committed);
        let audit = derive_clef_readability_shadow_course_audit(&xml);
        assert_eq!(audit.parts_planned, 1);
        courses.push((
            audit.part_rows[0].source_course.clone(),
            audit.part_rows[0].source_cost,
        ));
    }
    assert_eq!(courses[0], courses[1]);
}

#[test]
fn temporal_independent_part_duplication_preserves_each_local_course() {
    let xml = streams(false, "");
    let part = local_part(&xml).to_string();
    let other = part
        .replace("id=\"p\"", "id=\"other\"")
        .replace("alpha", "upper")
        .replace("beta", "lower");
    let mut single = xml;
    apply_rustig_clef_course_to_buffer(&mut single).0.unwrap();
    let expected = local_part(&single).to_string();
    let mut combined = format!("<score-partwise>{part}{other}</score-partwise>");
    apply_rustig_clef_course_to_buffer(&mut combined).0.unwrap();
    let ranges = score_part_ranges(&combined);
    assert_eq!(&combined[ranges[0].0..ranges[0].1], expected);
    assert_eq!(
        &combined[ranges[1].0..ranges[1].1],
        expected
            .replace("id=\"p\"", "id=\"other\"")
            .replace("alpha", "upper")
            .replace("beta", "lower")
    );
}

#[test]
fn temporal_octave_offset_applies_before_an_earlier_serialized_head() {
    let high = pitch("F", 5, 4, "a", "");
    let direction = "<direction><direction-type><octave-shift type=\"down\" size=\"8\"/></direction-type><offset>2</offset><staff>2</staff></direction>";
    let xml = format!(
        "<part id=\"p\"><measure><attributes><divisions>1</divisions></attributes>{}{}{high}<backup><duration>6</duration></backup>{direction}{}</measure></part>",
        clef("F", 4),
        pitch("C", 3, 2, "b", ""),
        pitch("C", 3, 4, "c", "")
    );
    let time = TemporalPart::observe(&xml).unwrap();
    let high = note_ranges(&xml)
        .into_iter()
        .find(|(_, _, n)| n.contains("<octave>5</octave>"))
        .unwrap()
        .0;
    assert_eq!(time.shift(high), Some(-7));
}

#[test]
fn temporal_duration_normalization_and_chord_attachment_are_exact() {
    let xml = format!(
        "<part id=\"p\"><measure><attributes><divisions>2</divisions></attributes>{}{}{}</measure><measure><attributes><divisions>3</divisions></attributes>{}</measure></part>",
        clef("F", 4),
        pitch("C", 3, 2, "a", ""),
        pitch("E", 3, 2, "a", "").replacen("<note>", "<note><chord/>", 1),
        pitch("G", 3, 3, "a", "")
    );
    let time = TemporalPart::observe(&xml).unwrap();
    let notes = note_ranges(&xml);
    assert_eq!(time.at(notes[0].0), Some(0));
    assert_eq!(time.at(notes[1].0), Some(0));
    assert_eq!(time.at(notes[2].0), Some(6));
}

#[test]
fn temporal_unknown_context_grants_no_course() {
    for fragment in [
        "<backup><duration>1</duration></backup>".to_string(),
        pitch("C", 3, 1, "a", "").replacen("<note>", "<note><chord/>", 1),
        "<note><grace/><pitch><step>C</step><octave>4</octave></pitch></note>".to_string(),
        format!(
            "{}<backup><duration>1</duration></backup>{}<backup><duration>0</duration></backup>{}",
            pitch("C", 3, 1, "a", ""),
            clef("F", 4),
            clef("G", 2)
        ),
    ] {
        let xml = format!("<part id=\"p\"><measure>{fragment}</measure></part>");
        assert!(TemporalPart::observe(&xml).is_none());
        let plan = derive_clef_readability_primary_course_plan(&xml);
        assert!(plan.removals.is_empty() && plan.insertions.is_empty());
    }
}

#[test]
fn temporal_overlapping_heads_in_one_voice_grant_no_plan() {
    let xml = format!(
        "<part><measure>{}{}<backup><duration>1</duration></backup>{}</measure></part>",
        clef("F", 4),
        pitch("C", 5, 2, "shared", ""),
        pitch("E", 5, 1, "shared", "")
    );
    let time = TemporalPart::observe(&xml).unwrap();
    let notes = note_ranges(&xml);
    assert!(!time.origin_supported(notes[0].0));
    assert!(!time.origin_supported(notes[1].0));
    let plan = derive_clef_readability_primary_course_plan(&xml);
    assert!(plan.insertions.is_empty());
}

fn nonoverlapping_reordered_tail(reverse: bool, voice: &str) -> String {
    let first = format!("{}{}", pitch("B", 4, 1, voice, "").repeat(5),
        pitch("C", 3, 1, voice, "").repeat(5));
    let low = pitch("C", 3, 1, voice, "");
    let tail = if reverse {
        format!("<forward><duration>2</duration></forward>{low}<backup><duration>3</duration></backup>{low}")
    } else {
        format!("{low}<forward><duration>1</duration></forward>{low}")
    };
    format!("<score-partwise><part id=\"local\"><measure number=\"passage\"><attributes><divisions>1</divisions></attributes>{}{first}</measure><measure number=\"tail\">{tail}</measure></part></score-partwise>", clef("F", 4))
}

#[test]
fn temporal_disjoint_tail_order_preserves_course_receipt_and_reentry() {
    let mut expected = None;
    for voice in ["lane", "renamed"] {
        for reverse in [false, true] {
            let mut xml = nonoverlapping_reordered_tail(reverse, voice);
            let before = raw_note_fragments(&xml);
            let part = local_part(&xml);
            let time = TemporalPart::observe(part).unwrap();
            assert!(note_ranges(part).iter().all(|(start, _, _)| time.origin_supported(*start)));
            let surfaces = scan_lower_clef_surfaces(part).unwrap();
            let notes = scan_lower_pitches(part, None);
            let plan = solve_readability_course(part, &surfaces, &notes).unwrap();
            assert_eq!(plan.cost.ledger_pressure, 0);
            assert_eq!(plan.cost.clef_transitions, 2);
            let (result, receipt) = apply_rustig_clef_course_to_buffer(&mut xml);
            result.unwrap();
            assert!(receipt.is_some());
            assert_eq!(raw_note_fragments(&xml), before);
            let committed = xml.clone();
            apply_rustig_clef_course_to_buffer(&mut xml).0.unwrap();
            assert_eq!(xml, committed);
            let audit = derive_clef_readability_shadow_course_audit(&xml);
            let actual = (audit.part_rows[0].source_course.clone(), audit.part_rows[0].source_cost);
            if let Some(ref baseline) = expected { assert_eq!(&actual, baseline); }
            else { expected = Some(actual); }
        }
    }
}

#[test]
fn temporal_reordered_tie_and_beam_origins_keep_their_source_locators() {
    for (start, stop) in [("<tie type=\"start\"/>", "<tie type=\"stop\"/>"),
        ("<beam number=\"1\">begin</beam>", "<beam number=\"1\">end</beam>")] {
        let first = pitch("B", 4, 1, "lane", start);
        let last = pitch("B", 4, 1, "lane", stop);
        for reverse in [false, true] {
            let sequence = if reverse {
                format!("<forward><duration>1</duration></forward>{last}<backup><duration>2</duration></backup>{first}")
            } else { format!("{first}{last}") };
            let xml = format!("<part><measure>{}{sequence}</measure></part>", clef("F", 4));
            let time = TemporalPart::observe(&xml).unwrap();
            let notes = scan_lower_pitches(&xml, None);
            assert_eq!(notes.len(), 2);
            assert_eq!(notes[0].at, 0);
            assert_eq!(notes[1].at, 1);
            assert_eq!(notes[1].clef_transition_anchor, Some(notes[0].start));
            assert_eq!(notes[1].anchor_at, Some(0));
            assert_eq!(time.at(notes[0].start), Some(0));
            assert_eq!(time.at(notes[1].start), Some(1));
            assert_eq!(notes[0].start > notes[1].start, reverse);
        }
    }
}

#[test]
fn temporal_reordered_part_duplication_keeps_each_independent_course() {
    let xml = nonoverlapping_reordered_tail(true, "lane");
    let part = local_part(&xml).to_string();
    let other = part.replace("local", "other").replace("lane", "second");
    let mut single = xml;
    apply_rustig_clef_course_to_buffer(&mut single).0.unwrap();
    let expected = local_part(&single);
    let mut both = format!("<score-partwise>{part}{other}</score-partwise>");
    apply_rustig_clef_course_to_buffer(&mut both).0.unwrap();
    let ranges = score_part_ranges(&both);
    assert_eq!(&both[ranges[0].0..ranges[0].1], expected);
    assert_eq!(&both[ranges[1].0..ranges[1].1], expected.replace("local", "other").replace("lane", "second"));
}

#[test]
fn temporal_ordered_equal_time_initial_override_retains_a_valid_course() {
    let xml = format!(
        "<part><measure>{}{}{}{}</measure></part>",
        clef("F", 4),
        clef("G", 2),
        pitch("C", 5, 1, "a", ""),
        pitch("D", 5, 1, "a", "")
    );
    let surfaces = scan_lower_clef_surfaces(&xml).unwrap();
    assert_eq!(surfaces.len(), 2);
    let notes = scan_lower_pitches(&xml, None);
    assert_eq!(
        score_readability_course(&xml, &surfaces, &notes, &[true, true])
            .unwrap()
            .ledger_pressure,
        0
    );
}

#[test]
fn temporal_hidden_cursor_overlay_does_not_replace_visible_voice_origins() {
    let visible = pitch("C", 3, 2, "shared", "");
    let hidden = "<note print-object=\"no\"><rest/><duration>2</duration><voice>shared</voice><staff>2</staff></note>";
    let xml = format!(
        "<part><measure>{visible}<backup><duration>2</duration></backup>{hidden}</measure></part>"
    );
    assert!(TemporalPart::observe(&xml).is_some());
}

#[test]
fn temporal_unsupported_input_retains_parent_postwrite_protection() {
    let source = format!(
        "<score-partwise><part><measure>{}{}{}{}{}</measure></part></score-partwise>",
        clef("F", 4),
        pitch("C", 3, 1, "a", "<beam number=\"1\">begin</beam>"),
        clef("G", 2),
        pitch("G", 3, 1, "a", "<beam number=\"1\">end</beam>"),
        "<note><grace/><pitch><step>C</step><octave>3</octave></pitch><voice>a</voice><staff>2</staff></note>"
    );
    assert!(TemporalPart::observe(&source).is_none());
    let mut candidate = source.clone();
    let (result, receipt) = apply_rustig_clef_course_to_buffer(&mut candidate);
    assert!(result.is_err());
    assert!(receipt.is_none());
    assert_eq!(candidate, source);
}

#[test]
fn temporal_chord_clock_does_not_mint_a_missing_structural_attachment() {
    let source = format!(
        "<part><measure>{}{}{}{}{}</measure></part>",
        clef("F", 4),
        pitch("C", 3, 1, "a", ""),
        clef("G", 2),
        pitch("E", 3, 1, "a", "").replacen("<note>", "<note><chord/>", 1),
        pitch("G", 3, 1, "a", "")
    );
    let time = TemporalPart::observe(&source).unwrap();
    let notes = note_ranges(&source);
    assert_eq!(time.at(notes[0].0), time.at(notes[1].0));
    assert_eq!(time.beam_inside(notes[1].0), None);
    let pitched = scan_lower_pitches(&source, None);
    assert_eq!(pitched[1].clef_transition_anchor, None);
}

#[test]
fn temporal_scalar_whitespace_matches_the_existing_observation_semantics() {
    let xml = streams(false, "tie")
        .replace("<staff>2</staff>", "<staff> 2 </staff>")
        .replace("<voice>alpha</voice>", "<voice> alpha </voice>")
        .replace("<voice>beta</voice>", "<voice> beta </voice>")
        .replace("<divisions>1</divisions>", "<divisions> 1 </divisions>")
        .replace("<duration>", "<duration> ")
        .replace("</duration>", " </duration>");
    let part = local_part(&xml);
    let notes = scan_lower_pitches(part, None);
    let surfaces = scan_lower_clef_surfaces(part).unwrap();
    assert_eq!(
        score_readability_course(part, &surfaces, &notes, &vec![true; surfaces.len()])
            .unwrap()
            .ledger_pressure,
        60
    );
    let target = note_ranges(part)
        .into_iter()
        .find(|(_, _, note)| note.contains("<step>F</step>"))
        .unwrap()
        .0;
    assert!(lower_tie_open_at_position(part, target));
}

#[test]
fn temporal_present_empty_offset_is_unresolved_instead_of_default_zero() {
    let xml = format!(
        "<part><measure><direction><direction-type><octave-shift type=\"down\" size=\"8\"/></direction-type><offset/><staff>2</staff></direction>{}{}</measure></part>",
        clef("F", 4),
        pitch("C", 3, 1, "a", "")
    );
    assert!(TemporalPart::observe(&xml).is_none());
    let plan = derive_clef_readability_primary_course_plan(&xml);
    assert!(plan.insertions.is_empty() && plan.removals.is_empty());
}

#[test]
fn temporal_initial_override_does_not_make_later_streams_new_defaults() {
    let xml = format!(
        "<part><measure>{}<backup><duration>0</duration></backup>{}<backup><duration>0</duration></backup>{}{}</measure></part>",
        clef("F", 4),
        clef("G", 2),
        clef("F", 4),
        pitch("C", 3, 1, "a", "")
    );
    assert!(TemporalPart::observe(&xml).is_none());
}

#[test]
fn temporal_anchor_diagnostics_observe_the_final_effective_anchor() {
    let xml = format!(
        "<part><measure>{}{}<backup><duration>1</duration></backup>{}</measure></part>",
        clef("F", 4),
        pitch("C", 5, 2, "a", "<beam number=\"1\">begin</beam>"),
        pitch("E", 5, 1, "a", "<beam number=\"1\">end</beam>")
    );
    let mut rows = Vec::new();
    let notes = scan_lower_pitches(&xml, Some(&mut rows));
    assert_eq!(rows.len(), 2);
    for row in rows {
        let actual = notes.iter().find(|note| note.start == row.start).unwrap();
        assert_eq!(row.effective_anchor, actual.clef_transition_anchor);
        assert_eq!(row.effective_anchor, None);
    }
}
