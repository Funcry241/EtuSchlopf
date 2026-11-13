use super::*;

fn pitched(step: &str, octave: u8, duration: i64, voice: &str, staff: u8, extra: &str) -> String {
    format!(
        "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff>{extra}</note>"
    )
}
fn rest(duration: i64) -> String {
    format!("<note><rest/><duration>{duration}</duration><voice>2</voice><staff>2</staff></note>")
}
fn measure(body: &str) -> String {
    format!("<measure number=\"SyntheticMeasure\">{body}</measure>")
}
fn run(xml: &str, context: RhythmContext, enabled: bool) -> String {
    let mut stats = VisibleLedgerStats::default();
    let (out, origins) = if enabled {
        normalize_visible_note_values(xml, context, &mut stats, None)
    } else {
        r1537_normalize_visible_note_values(xml, context, &mut stats, None)
    };
    assert_eq!(collect_raw_notes(&out).len(), origins.len());
    out
}
fn compare_excluded(xml: &str, context: RhythmContext) {
    assert_eq!(run(xml, context, true), run(xml, context, false));
}
fn sound(xml: &str) -> Vec<(i32, String, u8, i64, i64)> {
    let raw = collect_raw_notes(xml);
    let mut active = BTreeMap::<(i32, String, u8), (i64, i64)>::new();
    let mut events = Vec::new();
    for meta in collect_note_metas(xml, &raw) {
        let Some(pitch) = meta.midi else {
            continue;
        };
        let key = (
            pitch,
            meta.voice,
            meta.staff.number().parse::<u8>().unwrap(),
        );
        let doc = roxmltree::Document::parse(&raw[meta.raw_idx].text).unwrap();
        let has = |kind| {
            doc.root_element()
                .children()
                .any(|n| n.has_tag_name("tie") && n.attribute("type") == Some(kind))
        };
        let interval = if has("stop") {
            let previous = active.remove(&key).expect("matched tie");
            assert_eq!(previous.1, meta.start);
            (previous.0, meta.start + meta.duration)
        } else {
            (meta.start, meta.start + meta.duration)
        };
        if has("start") {
            assert!(active.insert(key, interval).is_none());
        } else {
            events.push((key.0, key.1, key.2, interval.0, interval.1));
        }
    }
    assert!(active.is_empty());
    events.sort();
    events
}
fn assert_preserved(xml: &str, out: &str) {
    assert_eq!(sound(xml), sound(out));
}

#[test]
fn arbitrary_pitch_staff_voice_and_measure_number() {
    for (step, octave) in [("C", 3), ("G", 4), ("B", 2)] {
        for staff in [1, 2] {
            for voice in ["1", "2", "17"] {
                let xml = measure(&pitched(step, octave, 384, voice, staff, ""));
                let out = run(&xml, RhythmContext::default(), true);
                assert_eq!(collect_raw_notes(&out).len(), 1);
                assert!(out.contains("<type>whole</type>"));
                assert_preserved(&xml, &out);
            }
        }
    }
}
#[test]
fn other_staff_independent_melody_is_preserved() {
    let melody = (0..4)
        .map(|_| pitched("G", 4, 96, "1", 1, ""))
        .collect::<String>();
    let xml = measure(&format!(
        "{melody}<backup><duration>384</duration></backup>{}",
        pitched("C", 3, 384, "2", 2, "")
    ));
    let out = run(&xml, RhythmContext::default(), true);
    assert_eq!(collect_raw_notes(&out).len(), 5);
    assert_preserved(&xml, &out);
}
#[test]
fn independent_repeated_attacks_are_not_merged() {
    let xml = measure(&pitched("C", 3, 96, "2", 2, "").repeat(4));
    compare_excluded(&xml, RhythmContext::default());
    let out = run(&xml, RhythmContext::default(), true);
    assert_eq!(sound(&out).len(), 4);
    assert_preserved(&xml, &out);
}
#[test]
fn existing_tied_quarters_are_not_merged() {
    let mut body = String::new();
    for i in 0..4 {
        let extra = format!(
            "{}{}",
            if i > 0 { "<tie type=\"stop\"/>" } else { "" },
            if i < 3 { "<tie type=\"start\"/>" } else { "" }
        );
        body.push_str(&pitched("C", 3, 96, "2", 2, &extra));
    }
    let xml = measure(&body);
    compare_excluded(&xml, RhythmContext::default());
    assert_preserved(&xml, &run(&xml, RhythmContext::default(), true));
}
#[test]
fn delayed_entry_after_rest_keeps_legacy_notation() {
    let xml = measure(&format!("{}{}", rest(96), pitched("C", 3, 288, "2", 2, "")));
    compare_excluded(&xml, RhythmContext::default());
    assert_preserved(&xml, &run(&xml, RhythmContext::default(), true));
}
#[test]
fn interruption_by_rest_preserves_two_attacks() {
    let xml = measure(&format!(
        "{}{}{}",
        pitched("C", 3, 192, "2", 2, ""),
        rest(96),
        pitched("C", 3, 96, "2", 2, "")
    ));
    let out = run(&xml, RhythmContext::default(), true);
    // Preserve the original half, while the
    // separate post-rest attack remains independently represented.
    assert_eq!(half_bar_pitch_count(&out, 48), 2);
    assert_eq!(sound(&out).len(), 2);
    assert_preserved(&xml, &out);
}
#[test]
fn pickup_and_implicit_measure_are_excluded() {
    let short = measure(&pitched("C", 3, 96, "2", 2, ""));
    compare_excluded(&short, RhythmContext::default());
    let implicit = measure(&pitched("C", 3, 384, "2", 2, "")).replace(
        "number=\"SyntheticMeasure\"",
        "number=\"0\" implicit=\"yes\"",
    );
    compare_excluded(&implicit, RhythmContext::default());
}
#[test]
fn other_meters_are_excluded() {
    for (beats, beat_type, duration) in [(3, 4, 288), (2, 2, 384), (6, 8, 288)] {
        compare_excluded(
            &measure(&pitched("C", 3, duration, "2", 2, "")),
            RhythmContext {
                divisions: 96,
                beats,
                beat_type,
                physical_duration: None,
            },
        );
    }
}
#[test]
fn equal_and_unequal_chord_members_are_excluded() {
    for duration in [192, 384] {
        let member = pitched("E", 3, duration, "2", 2, "").replace("<note>", "<note><chord/>");
        compare_excluded(
            &measure(&format!("{}{member}", pitched("C", 3, 384, "2", 2, ""))),
            RhythmContext::default(),
        );
    }
}
#[test]
fn additional_voice_in_same_staff_is_excluded() {
    let xml = measure(&format!(
        "{}<backup><duration>384</duration></backup>{}",
        pitched("C", 3, 384, "2", 2, ""),
        pitched("G", 3, 384, "3", 2, "")
    ));
    compare_excluded(&xml, RhythmContext::default());
    assert_preserved(&xml, &run(&xml, RhythmContext::default(), true));
}
#[test]
fn incoming_outgoing_and_through_ties_are_excluded() {
    for ties in [
        "<tie type=\"stop\"/>",
        "<tie type=\"start\"/>",
        "<tie type=\"stop\"/><tie type=\"start\"/>",
    ] {
        compare_excluded(
            &measure(&pitched("C", 3, 384, "2", 2, ties)),
            RhythmContext::default(),
        );
    }
}
#[test]
fn articulation_slur_and_lyric_are_excluded() {
    for extra in [
        "<notations><articulations><accent/></articulations></notations>",
        "<notations><slur type=\"start\" number=\"1\"/></notations>",
        "<lyric><text>La</text></lyric>",
    ] {
        compare_excluded(
            &measure(&pitched("C", 3, 384, "2", 2, extra)),
            RhythmContext::default(),
        );
    }
}
fn pedals(xml: &str) -> Vec<(String, i64)> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let mut cursor = 0;
    let mut result = Vec::new();
    for node in doc.root_element().children().filter(|n| n.is_element()) {
        if node.has_tag_name("note") && !node.children().any(|n| n.has_tag_name("chord")) {
            cursor += node
                .children()
                .find(|n| n.has_tag_name("duration"))
                .unwrap()
                .text()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        } else if node.has_tag_name("direction") {
            let offset = node
                .children()
                .find(|n| n.has_tag_name("offset"))
                .and_then(|n| n.text())
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap();
            for pedal in node.descendants().filter(|n| n.has_tag_name("pedal")) {
                result.push((pedal.attribute("type").unwrap().to_owned(), cursor + offset));
            }
        }
    }
    result
}
#[test]
fn independent_pedal_changes_keep_exact_positions() {
    let direction = |kind, offset| {
        format!(
            "<direction><direction-type><pedal type=\"{kind}\"/></direction-type><offset>{offset}</offset><staff>2</staff></direction>"
        )
    };
    let xml = measure(&format!(
        "{}{}{}{}",
        direction("start", 0),
        pitched("C", 3, 384, "2", 2, ""),
        direction("change", -192),
        direction("stop", 0)
    ));
    let out = run(&xml, RhythmContext::default(), true);
    assert_eq!(collect_raw_notes(&out).len(), 1);
    assert_preserved(&xml, &out);
    assert_eq!(pedals(&xml), pedals(&out));
    assert_eq!(
        pedals(&out),
        vec![
            ("start".into(), 0),
            ("change".into(), 192),
            ("stop".into(), 384)
        ]
    );
}
#[test]
fn hidden_note_and_unsupported_divisions_are_excluded() {
    let hidden =
        measure(&pitched("C", 3, 384, "2", 2, "")).replace("<note>", "<note print-object=\"no\">");
    compare_excluded(&hidden, RhythmContext::default());
    compare_excluded(
        &measure(&pitched("C", 3, 768, "2", 2, "")),
        RhythmContext {
            divisions: 192,
            beats: 4,
            beat_type: 4,
            physical_duration: None,
        },
    );
}
#[test]
fn overfilled_measure_is_excluded() {
    compare_excluded(
        &measure(&format!(
            "{}<forward><duration>96</duration></forward>{}",
            pitched("C", 3, 384, "2", 2, ""),
            pitched("E", 4, 96, "1", 1, "")
        )),
        RhythmContext::default(),
    );
}
#[test]
fn actual_source_bound_arrangement_accepts_singleton_and_keeps_chord_support() {
    use crate::midi_seed::rhythmic_course_creation_tests::arrangement_pitched_fixture;
    for rows in [
        vec![(101, 0, 2, 48)],
        vec![(101, 0, 2, 48), (102, 0, 2, 52)],
    ] {
        let (xml, seed) = arrangement_pitched_fixture(&rows, 1);
        let result = arrange_piano_musicxml_with_profile(
            &xml,
            ArrangementProfile {
                rhythmic_courses: Some(&seed),
                preserve_reference: true,
                ..Default::default()
            },
        );
        result
            .stats
            .rhythmic_courses
            .as_ref()
            .unwrap()
            .validate_binding(&result.xml)
            .unwrap();
        let notes = collect_raw_notes(&result.xml);
        let metas = collect_note_metas(&result.xml, &notes);
        let bass = metas.iter().filter(|m| m.midi == Some(48)).count();
        assert_eq!(bass, if rows.len() == 1 { 1 } else { 4 });
    }
}

#[test]
fn cursor_only_overfill_is_excluded() {
    compare_excluded(
        &measure(&format!(
            "{}<forward><duration>96</duration></forward>",
            pitched("C", 3, 384, "2", 2, "")
        )),
        RhythmContext::default(),
    );
}

// Test-only differential oracle: verbatim r1537 normalizer, renamed below.
// Original function SHA256: 5b815bab1489a4c393dd137fe32f884452e4e69c54f50c5f400282c1fee54763
fn r1537_normalize_visible_note_values(
    measure_text: &str,
    rhythm_context: RhythmContext,
    stats: &mut VisibleLedgerStats,
    mut course_measure: Option<&mut CourseMeasure>,
) -> (String, Vec<VisibleLedgerNormalizedOrigin>) {
    let beat_duration = rhythm_context.beat_duration();
    let raw_notes = collect_raw_notes(measure_text);
    if raw_notes.is_empty() {
        return (measure_text.to_string(), Vec::new());
    }
    let metas = collect_note_metas(measure_text, &raw_notes);
    let mut meta_by_raw: BTreeMap<usize, NoteMeta> = BTreeMap::new();
    for meta in metas {
        meta_by_raw.insert(meta.raw_idx, meta);
    }
    let mut out = String::with_capacity(measure_text.len() + 256);
    let mut origins = Vec::<VisibleLedgerNormalizedOrigin>::new();
    let mut last = 0_usize;
    let mut cohort_end = 0;
    for (idx, note) in raw_notes.iter().enumerate() {
        if idx < cohort_end {
            continue;
        }
        out.push_str(&measure_text[last..note.start]);
        let visible = is_note_block_visible(&note.text);
        let is_chord = note.text.contains("<chord") || note.text.contains("<chord/");
        let is_rest = note.text.contains("<rest") || note.text.contains("<rest/");
        let duration = extract_first_i64(&note.text, "duration")
            .unwrap_or(0)
            .max(0);
        let start = meta_by_raw.get(&idx).map_or(0, |m| m.start);
        if visible && !is_chord && duration > 0 {
            let staff = extract_first_text(&note.text, "staff")
                .and_then(|s| match s.trim() {
                    "1" => Some(PianoStaff::Right),
                    "2" => Some(PianoStaff::Left),
                    _ => None,
                })
                .or_else(|| staff_from_pitch(&note.text))
                .unwrap_or(PianoStaff::Right);
            let voice = extract_first_text(&note.text, "voice")
                .unwrap_or_else(|| staff.default_voice().to_string());
            let parts = if is_rest {
                crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                    start,
                    duration,
                    rhythm_context.expected_duration(),
                    beat_duration,
                )
            } else {
                split_duration_for_metric_notation(start, duration, beat_duration.max(1))
            };
            if is_rest && parts.is_empty() {
                if let Some(courses) = course_measure.as_deref_mut() {
                    courses
                        .push_normalized(idx, &note.text)
                        .expect("Unsplit rest custody");
                }
                if stats.capture_births {
                    stats
                        .births
                        .capture_continuation(note.start..note.end, out.len(), &note.text);
                }
                out.push_str(&note.text);
                origins.push(VisibleLedgerNormalizedOrigin {
                    pre_raw_idx: idx,
                    fragment_ordinal: 0,
                });
                last = note.end;
                continue;
            }
            let must_split = parts.len() > 1 || !is_exact_notation_duration(duration);
            if must_split {
                if let Some(courses) = course_measure.as_deref_mut() {
                    let mut end = idx + 1;
                    while end < raw_notes.len() && meta_by_raw.get(&end).is_some_and(|m| m.is_chord)
                    {
                        end += 1;
                    }
                    if end > idx + 1 && parts.len() > 1 {
                        // This is one physical host transaction, not a scorewide
                        // winner. Origin support is checked before realization.
                        courses
                            .cohort_support(idx, end, &parts)
                            .expect("Complete source-bound chord cohort before split");
                        for member in idx + 1..end {
                            assert!(
                                measure_text[raw_notes[member - 1].end..raw_notes[member].start]
                                    .trim()
                                    .is_empty(),
                                "Chord cohort crosses a cursor operation or independent XML object"
                            );
                        }
                        let replacements = raw_notes[idx..end]
                            .iter()
                            .enumerate()
                            .map(|(member, raw)| {
                                if member == 0 && is_rest {
                                    make_visible_rest_sequence_from_parts(
                                        &parts,
                                        staff,
                                        voice.trim(),
                                    )
                                } else {
                                    split_visible_pitched_note_with_parts(&raw.text, &parts)
                                }
                            })
                            .collect::<Vec<_>>();
                        let blocks = replacements
                            .iter()
                            .map(|text| collect_raw_notes(text))
                            .collect::<Vec<_>>();
                        assert!(
                            blocks.iter().all(|notes| notes.len() == parts.len()),
                            "Incomplete physical cohort split"
                        );
                        courses
                            .cohort_normalized(idx, &replacements)
                            .expect("Actual cohort split origins");
                        for fragment_ordinal in 0..parts.len() {
                            for (member, notes) in blocks.iter().enumerate() {
                                out.push_str(&notes[fragment_ordinal].text);
                                out.push('\n');
                                origins.push(VisibleLedgerNormalizedOrigin {
                                    pre_raw_idx: idx + member,
                                    fragment_ordinal,
                                });
                            }
                        }
                        stats.values_split += end - idx;
                        stats.chord_cohorts_source_bound_split += 1;
                        last = raw_notes[end - 1].end;
                        cohort_end = end;
                        continue;
                    }
                }
                let replacement = if is_rest {
                    make_visible_rest_sequence_from_parts(&parts, staff, voice.trim())
                } else {
                    split_visible_pitched_note_with_parts(&note.text, &parts)
                };
                if let Some(courses) = course_measure.as_deref_mut() {
                    courses
                        .push_normalized(idx, &replacement)
                        .expect("Actual split course ancestry");
                }
                if stats.capture_births && course_measure.is_none() && parts.len() > 1 {
                    stats.births.capture_visible_sequence(
                        &VisibleRhythmBlockMint { _sealed: () },
                        Some(note.start..note.end),
                        PhysicalBirthKind::MetricFragment,
                        out.len(),
                        &replacement,
                    );
                }
                if stats.capture_births && parts.len() <= 1 {
                    stats.births.capture_continuation(
                        note.start..note.end,
                        out.len(),
                        &replacement,
                    );
                }
                out.push_str(&replacement);
                for fragment_ordinal in 0..parts.len() {
                    origins.push(VisibleLedgerNormalizedOrigin {
                        pre_raw_idx: idx,
                        fragment_ordinal,
                    });
                }
                stats.values_split += 1;
                last = note.end;
                continue;
            }
        }
        let replacement = if visible && duration > 0 {
            let synced = sync_note_type_and_dots(&note.text);
            if synced != note.text {
                stats.notes_normalized += 1;
            }
            synced
        } else {
            note.text.clone()
        };
        if let Some(courses) = course_measure.as_deref_mut() {
            courses
                .push_normalized(idx, &replacement)
                .expect("Unsplit occurrence custody");
        }
        if stats.capture_births {
            stats
                .births
                .capture_continuation(note.start..note.end, out.len(), &replacement);
        }
        out.push_str(&replacement);
        origins.push(VisibleLedgerNormalizedOrigin {
            pre_raw_idx: idx,
            fragment_ordinal: 0,
        });
        last = note.end;
    }
    out.push_str(&measure_text[last..]);
    (out, origins)
}

#[test]
fn independent_candidates_in_distinct_staffs_are_not_ranked_or_count_limited() {
    for reversed in [false, true] {
        let a = pitched("C", 3, 384, "2", 2, "");
        let b = pitched("G", 4, 384, "17", 1, "");
        let xml = measure(&if reversed {
            format!("{b}<backup><duration>384</duration></backup>{a}")
        } else {
            format!("{a}<backup><duration>384</duration></backup>{b}")
        });
        let out = run(&xml, RhythmContext::default(), true);
        assert_eq!(out.matches("<type>whole</type>").count(), 2);
        assert_preserved(&xml, &out);
    }
}
#[test]
fn independent_candidates_in_distinct_measures_and_renamed_labels_are_preserved() {
    for labels in [("1", "2"), ("900", "Renamed")] {
        let a = measure(&pitched("C", 3, 384, "2", 2, "")).replace(
            "number=\"SyntheticMeasure\"",
            &format!("number=\"{}\"", labels.0),
        );
        let b = measure(&pitched("G", 3, 384, "2", 2, "")).replace(
            "number=\"SyntheticMeasure\"",
            &format!("number=\"{}\"", labels.1),
        );
        let xml = format!("<score-partwise><part id=\"P1\">{a}{b}</part></score-partwise>");
        let result = normalize_visible_rhythm_musicxml(&xml);
        assert_eq!(result.xml.matches("<type>whole</type>").count(), 2);
    }
}


// Source-bound regressions for the aligned half-bar preservation policy.
fn half_bar_owned(rows: &[(usize, i64, u8, i32)], measures: usize) -> (String, String) {
    use crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture;
    let lengths = rows.iter().map(|&(source, start, staff, pitch)| {
        let end = rows.iter().filter(|r| r.2 == staff && r.1 > start && r.1 < (start / 384 + 1) * 384)
            .map(|r| r.1).min().unwrap_or((start / 384 + 1) * 384);
        (source, start, end - start, staff, pitch)
    }).collect::<Vec<_>>();
    let (input, seed) = arrangement_source_lengths_fixture(&lengths, measures);
    let received = ArrangementRhythmicCourses::receive(&seed, &input).unwrap();
    let old = received.source_origin_support_rows();
    let result = arrange_piano_musicxml_with_profile(&input, ArrangementProfile {
        rhythmic_courses: Some(&seed), preserve_reference: true, ..Default::default()
    });
    let custody = result.stats.rhythmic_courses.as_ref().unwrap();
    custody.validate_binding(&result.xml).unwrap();
    let new = custody.source_origin_support_rows();
    for (source, root, course, start, end) in &old {
        let mut children = new.iter().filter(|r| r.0 == *source && r.1 == *root && r.2 == *course).collect::<Vec<_>>();
        children.sort_by_key(|r| r.3);
        let mut cursor = *start;
        for row in children { assert_eq!(cursor, row.3); assert!(row.4 > row.3); cursor = row.4; }
        assert_eq!(cursor, *end, "source support preserved");
    }
    assert!(new.iter().all(|r| old.iter().any(|o| o.0 == r.0 && o.1 == r.1 && o.2 == r.2)));
    assert_eq!(sound(&input), sound(&result.xml));
    let again = normalize_visible_rhythm_musicxml(&result.xml);
    assert_eq!(sound(&result.xml), sound(&again.xml));
    assert_eq!(collect_raw_notes(&result.xml).len(), collect_raw_notes(&again.xml).len());
    let duration = extract_first_i64(&result.xml, "duration").unwrap();
    let invalid_support = result.xml.replacen(
        &format!("<duration>{duration}</duration>"),
        &format!("<duration>{}</duration>", duration + 1), 1,
    );
    assert!(custody.validate_binding(&invalid_support).is_err());
    (input, result.xml)
}
fn half_bar_pitch_count(xml: &str, pitch: i32) -> usize {
    collect_note_metas(xml, &collect_raw_notes(xml)).iter().filter(|n| n.midi == Some(pitch)).count()
}
#[test]
fn half_bar_owned_aligned_durations() {
    for staff in [1, 2] { for start in [0, 192] { for duration in [144, 192] {
        let mut rows = vec![(102, start, staff, 48)];
        if start > 0 { rows.push((101, 0, staff, 55)); }
        if start + duration < 384 { rows.push((103, start + duration, staff, 57)); }
        let (input, output) = half_bar_owned(&rows, 1);
        assert_eq!(half_bar_pitch_count(&input, 48), 1);
        assert_eq!(half_bar_pitch_count(&output, 48), 1);
    } } }
}
#[test]
fn half_bar_owned_independent_same_pitch_attacks() {
    let (_, output) = half_bar_owned(&[(101, 0, 2, 55), (102, 192, 2, 48), (103, 288, 2, 48)], 1);
    assert_eq!(half_bar_pitch_count(&output, 48), 2);
}
#[test]
fn source_bound_aligned_chords_keep_their_half_note_heads() {
    use crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture;
    let rows = [
        (101, 0, 192, 2, 48), (102, 0, 192, 2, 52), (103, 192, 192, 2, 55),
        (201, 576, 192, 1, 67), (202, 576, 192, 1, 71),
    ];
    let (input, seed) = arrangement_source_lengths_fixture(&rows, 2);
    let result = arrange_piano_musicxml_with_profile(&input, ArrangementProfile {
        rhythmic_courses: Some(&seed),
        preserve_reference: true,
        ..Default::default()
    });
    result.stats.rhythmic_courses.as_ref().unwrap().validate_binding(&result.xml).unwrap();
    assert_preserved(&input, &result.xml);
    for (_, _, _, _, pitch) in rows {
        assert_eq!(half_bar_pitch_count(&result.xml, pitch), 1);
    }
    let notes = collect_raw_notes(&result.xml);
    for meta in collect_note_metas(&result.xml, &notes).into_iter().filter(|meta| meta.midi.is_some()) {
        assert!(notes[meta.raw_idx].text.contains("<type>half</type>"));
        assert!(!notes[meta.raw_idx].text.contains("<tie"));
    }
}
#[test]
fn source_bound_quarters_cross_weak_beats_but_expose_the_half_bar() {
    use crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture;
    let rows = [
        (101, 48, 96, 2, 48), (102, 144, 96, 2, 50),
        (103, 240, 96, 2, 52), (104, 336, 48, 2, 55),
    ];
    let (input, seed) = arrangement_source_lengths_fixture(&rows, 1);
    let result = arrange_piano_musicxml_with_profile(&input, ArrangementProfile {
        rhythmic_courses: Some(&seed),
        preserve_reference: true,
        ..Default::default()
    });
    result.stats.rhythmic_courses.as_ref().unwrap().validate_binding(&result.xml).unwrap();
    assert_preserved(&input, &result.xml);
    for (pitch, count) in [(48, 1), (50, 2), (52, 1)] {
        assert_eq!(half_bar_pitch_count(&result.xml, pitch), count);
    }
}

#[test]
fn source_bound_held_halves_tie_only_the_short_remainder() {
    use crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture;
    let rows = [
        (101, 0, 240, 2, 48), (102, 240, 144, 2, 50),
        (201, 384, 216, 1, 67), (202, 600, 168, 1, 69),
    ];
    let (input, seed) = arrangement_source_lengths_fixture(&rows, 2);
    let result = arrange_piano_musicxml_with_profile(&input, ArrangementProfile {
        rhythmic_courses: Some(&seed),
        preserve_reference: true,
        ..Default::default()
    });
    result.stats.rhythmic_courses.as_ref().unwrap().validate_binding(&result.xml).unwrap();
    assert_preserved(&input, &result.xml);
    for (pitch, tail) in [(48, 48), (67, 24)] {
        let parts = find_measure_spans(&result.xml).iter().flat_map(|span| {
            let measure = &result.xml[span.start..span.end];
            let notes = collect_raw_notes(measure);
            collect_note_metas(measure, &notes).into_iter().filter(|meta| meta.midi == Some(pitch))
                .map(|meta| (meta.start, meta.duration, notes[meta.raw_idx].text.clone()))
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);
        assert_eq!((parts[0].0, parts[0].1), (0, 192));
        assert_eq!((parts[1].0, parts[1].1), (192, tail));
        assert!(parts[0].2.contains("<type>half</type>") && parts[0].2.contains("<tie type=\"start\"/>"));
        assert!(parts[1].2.contains("<tie type=\"stop\"/>"));
    }
}

#[test]
fn half_bar_owned_duplication_is_independent() {
    let (_, output) = half_bar_owned(&[(101, 0, 1, 55), (102, 192, 1, 48), (103, 0, 2, 57), (104, 192, 2, 60),
        (201, 384, 1, 55), (202, 576, 1, 48), (203, 384, 2, 57), (204, 576, 2, 60)], 2);
    for pitch in [48, 60] { assert_eq!(half_bar_pitch_count(&output,pitch), 2); }
}
#[test]
fn half_bar_counterexamples_keep_parent_output() {
    for (start, duration) in [(48,96),(72,96),(144,96),(96,192),(168,144)] {
        let xml = measure(&format!("{}{}{}", rest(start), pitched("C",3,duration,"2",2,""),rest(384-start-duration)));
        compare_excluded(&xml,RhythmContext::default());
        assert_eq!(sound(&xml),sound(&run(&xml,RhythmContext::default(),true)));
    }
    for extra in ["<notations><articulations><accent/></articulations></notations>",
        "<notations><slur type=\"start\" number=\"1\"/></notations>",
        "<lyric><text>synthetic</text></lyric>", "<time-modification><actual-notes>3</actual-notes><normal-notes>2</normal-notes></time-modification>",
        "<tie type=\"start\"/>"] {
        let xml=measure(&format!("{}{}",rest(192),pitched("C",3,192,"2",2,extra)));
        compare_excluded(&xml,RhythmContext::default());
    }
    let xml=measure(&format!("{}{}{}",rest(192),pitched("C",3,192,"2",2,""),pitched("E",3,192,"2",2,"<chord/>")));
    compare_excluded(&xml,RhythmContext::default());
}
#[test]
fn half_bar_alternative_meters_and_bounds_keep_parent() {
    let xml=measure(&format!("{}{}",rest(96),pitched("C",3,192,"2",2,"")));
    compare_excluded(&xml,RhythmContext {divisions:96,beats:3,beat_type:4,physical_duration:None});
    let xml=measure(&format!("{}{}<forward><duration>24</duration></forward>",rest(192),pitched("C",3,192,"2",2,"")));
    compare_excluded(&xml,RhythmContext::default());
}

#[test]
fn half_bar_hidden_chord_and_overlap_keep_parent_output() {
    let host = pitched("C",3,192,"2",2,"");
    let hidden = pitched("E",3,192,"2",2,"<chord/>").replacen("<note>","<note print-object=\"no\">",1);
    compare_excluded(&measure(&format!("{}{host}{hidden}",rest(192))),RhythmContext::default());
    let overlap=measure(&format!("{}{host}<backup><duration>192</duration></backup>{}",rest(192),pitched("E",3,192,"other",2,"")));
    compare_excluded(&overlap,RhythmContext::default());
}
#[test]
fn half_bar_independent_pedal_offsets_keep_absolute_time() {
    let direction="<direction><direction-type><pedal type=\"start\"/></direction-type><offset>-48</offset><staff>2</staff></direction>";
    let xml=measure(&format!("{}{}{direction}",rest(192),pitched("C",3,192,"2",2,"")));
    let output=run(&xml,RhythmContext::default(),true);
    assert!(output.contains(direction));
    assert_eq!(sound(&xml),sound(&output));
    assert_eq!(half_bar_pitch_count(&output,48),1);
}
