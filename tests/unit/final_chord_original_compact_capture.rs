use super::*;

#[test]
fn original_adjacent_emissions_survive_exact_script_coalescence() {
    for scope in [83, 1083] {
        for copies in [1, 2] {
            for voice in ["original", "lane-α"] {
                let mut evidence = Vec::new();
                let mut attacks = Vec::new();
                let mut body = String::new();
                for copy in 0..copies {
                    body.push_str(&format!("<measure number=\"{}\"><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>", scope + copy));
                    for (ordinal, step) in ["C", "D"].iter().enumerate() {
                        let source = 601 + copy * 2 + ordinal;
                        body.push_str(&format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>1</duration><voice>{voice}</voice><type>quarter</type><staff>1</staff></note>"));
                        evidence.push(VoiceFreeFinalChordEvidenceRow {
                            fragment_id: source, source_event_id: source, attack_id: source,
                            measure: scope + copy, raw_note_index: ordinal, onset: ordinal as i64,
                            duration: 1, pitch: 60 + ordinal as i32 * 2, staff: 1,
                            chord_syntax: false, tie_start: false, tie_stop: false,
                        });
                        attacks.push(FrozenVoiceAttackEvidenceRow {
                            source_event_id: source, attack_id: source,
                            absolute_start: (copy * 4 + ordinal) as i64,
                            absolute_end: (copy * 4 + ordinal + 1) as i64,
                            staff: 1, technical_lane: 1,
                        });
                    }
                    body.push_str(&format!("<note><rest/><duration>2</duration><voice>{voice}</voice><type>half</type><staff>1</staff></note></measure>"));
                }
                let before = format!("<score-partwise><part id=\"source\">{body}</part></score-partwise>");
                let (canonical, _) = plan_canonical_final_chord_structure_once(&before, &evidence).unwrap();
                let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
                let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
                let plan = bind_canonical_final_chord_encoding_plan(&before, &canonical, &voices).unwrap();
                let mut unrecorded = plan.clone(); unrecorded.original_course_bindings = None;
                assert_eq!(plan, unrecorded);
                assert_eq!(format!("{plan:?}"), format!("{unrecorded:?}"));
                let (post, receipt) = project_final_chord_encoding_plan(&before, &plan);
                let post = post.unwrap(); let receipt = receipt.unwrap();
                let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
                assert_eq!(capture.validate_execution(&receipt, &before).unwrap(), post);
                assert_eq!(capture.rows().len(), copies * 3);
                let first = capture.rows()[0].input().clone();
                assert!(receipt.operation_readback(&before).unwrap().carry_range(first).is_err(),
                    "the original composed script must exercise the real geometric gap");
                let output = roxmltree::Document::parse(&post).unwrap();
                let ranges = output.descendants().filter(|node| node.has_tag_name("note"))
                    .map(|node| node.range()).collect::<Vec<_>>();
                for row in capture.rows() {
                    assert_eq!(row.outputs().len(), 1);
                    assert!(ranges.contains(&row.outputs()[0].0));
                }
            }
        }
    }
}

#[test]
fn original_tied_emitter_certifies_a_total_correspondence_of_the_original_domain() {
    for scope in [83, 1083] {
        let note = |step: &str, duration: i64, chord: bool, tie: &str| {
            let chord = if chord { "<chord/>" } else { "" };
            let tie = if tie.is_empty() { String::new() } else { format!("<tie type=\"{tie}\"/>") };
            format!("<note>{chord}<pitch><step>{step}</step><octave>4</octave></pitch><duration>{duration}</duration>{tie}<voice>source-α</voice><type>quarter</type><staff>1</staff></note>")
        };
        let body = format!("{}{}{}<note><rest/><duration>8</duration><voice>source-α</voice><type>half</type><staff>1</staff></note>",
            note("C", 4, false, "start"), note("C", 4, false, "stop"), note("E", 8, true, ""));
        let before = format!("<score-partwise><part id=\"source\"><measure number=\"{scope}\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>");
        let evidence = [(901, 911, 0, 0, 4, 60, false, true, false),
            (902, 911, 1, 4, 4, 60, false, false, true),
            (903, 912, 2, 4, 8, 64, true, false, false)].into_iter().map(|
                (fragment_id, source_event_id, raw_note_index, onset, duration, pitch, chord_syntax, tie_start, tie_stop)|
            VoiceFreeFinalChordEvidenceRow { fragment_id, source_event_id, attack_id: source_event_id,
                measure: scope, raw_note_index, onset, duration, pitch, staff: 1, chord_syntax, tie_start, tie_stop }).collect::<Vec<_>>();
        let (canonical, _) = plan_canonical_final_chord_structure_once(&before, &evidence).unwrap();
        let attacks = [911, 912].into_iter().map(|source_event_id| FrozenVoiceAttackEvidenceRow {
            source_event_id, attack_id: source_event_id, absolute_start: 0, absolute_end: 8, staff: 1, technical_lane: 1,
        }).collect::<Vec<_>>();
        let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
        let plan = bind_canonical_final_chord_encoding_plan(&before, &canonical, &voices).unwrap();
        let (post, receipt) = project_final_chord_encoding_plan(&before, &plan);
        let post = post.unwrap(); let receipt = receipt.unwrap();
        let capture = receipt.final_chord_capture().unwrap().as_ref().unwrap();
        let rows = capture.rows().iter().filter(|row| row.tied_chain()).collect::<Vec<_>>();
        assert_eq!(rows.len(), 1); assert_eq!(rows[0].outputs().len(), 2);
        assert_eq!(rows[0].original_offset(), 4);
        let notes = final_chord_parse_binding_measures(&post).unwrap().into_iter().flat_map(|scope| scope.notes)
            .map(|note| ((note.range.start, note.range.end), note)).collect::<std::collections::BTreeMap<_, _>>();
        let mut spans = rows[0].outputs().iter().map(|(bytes, cursor)| {
            assert!(!cursor); let note = &notes[&(bytes.start, bytes.end)];
            assert_eq!(note.pitch_midi, Some(64));
            note.onset + rows[0].original_offset()..note.onset + note.duration + rows[0].original_offset()
        }).collect::<Vec<_>>();
        spans.sort_by_key(|span| span.start);
        assert_eq!(spans, vec![4..8, 8..12]);
        assert!(notes[&(rows[0].outputs()[0].0.start, rows[0].outputs()[0].0.end)].tie_start);
        assert!(notes[&(rows[0].outputs()[1].0.start, rows[0].outputs()[1].0.end)].tie_stop);
    }
}
