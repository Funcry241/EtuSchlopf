// Committed rest-bridge readback observes written continuity on one bound lane.
// It does not require a bridge where the score explicitly closes the beam.

pub(crate) fn lower_dual_edge_beat_regression_candidates_with_voice_binding(
    xml: &str,
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> Option<usize> {
    if !voice_plan.is_bound() {
        return None;
    }
    count_written_rest_beam_errors(xml, |voice| voice_plan.role_for_label(voice).is_some())
}

fn count_written_rest_beam_errors(
    xml: &str,
    bound_voice: impl Fn(&str) -> bool,
) -> Option<usize> {
    parse_musicxml(xml).ok()?;
    let mut candidates = 0_usize;
    for span in partial_beat_measure_spans(xml) {
        let measure = &xml[span.start..span.end];
        let raw_notes = partial_beat_raw_notes(measure);
        let infos = partial_beat_note_infos(measure, &raw_notes);
        if !infos.iter().any(|note| note.is_rest && !note.is_hidden) {
            continue;
        }
        let boundaries = metric_beam_clef_boundaries(measure)?;
        let mut lanes = BTreeMap::<_, Vec<_>>::new();
        for (info, raw) in infos.iter().zip(&raw_notes) {
            if !info.is_chord && info.duration > 0 && bound_voice(&info.voice) {
                lanes.entry((info.staff.as_str(), info.voice.as_str()))
                    .or_default().push((info, raw));
            }
        }
        for ((staff, _), notes) in &mut lanes {
            notes.sort_by_key(|(note, _)| (note.start, note.index));
            for interval in notes.chunk_by(|(left, _), (right, _)| {
                left.start + left.duration == right.start && left.index < right.index
            }) {
                if interval.len() < 3 {
                    continue;
                }
                let cuts = boundaries.get(*staff).map_or(&[][..], Vec::as_slice);
                candidates += count_rest_bridge_beam_gaps(interval, cuts)?;
            }
        }
    }
    Some(candidates)
}

fn count_rest_bridge_beam_gaps(
    interval: &[(&PartialBeatNoteInfo, &PartialBeatRawNote)],
    clef_cuts: &[i64],
) -> Option<usize> {
    let mut candidates = 0;
    let mut left = None;
    for (right_index, (right, right_raw)) in interval.iter().copied().enumerate() {
        if !partial_beat_is_beam_candidate(right) {
            if !right.is_rest || right.is_hidden {
                left = None;
            }
            continue;
        }
        let previous = left.replace(right_index);
        let Some(left_index) = previous.filter(|index| index + 1 < right_index) else {
            continue;
        };
        let (left, _) = interval[left_index];
        let right_beams = partial_beat_observed_beam_states(&right_raw.text)?;
        let rests = &interval[left_index + 1..right_index];
        let left_beams = partial_beat_observed_beam_states(&interval[left_index].1.text)?;
        let mut rest_beam_levels = BTreeSet::new();
        for (_, raw) in rests {
            rest_beam_levels.extend(
                partial_beat_observed_beam_states(&raw.text)?.into_iter().map(|(level, _)| level),
            );
        }
        let cut = crosses_metric_beam_clef_boundary(clef_cuts, left.start, right.start);
        let mut malformed = false;
        // The rest-bridge owner emits only the two lower beam levels.
        for level in 1..=2 {
            let incoming = right_beams.iter().any(|(number, state)| {
                *number == level && matches!(state.as_str(), "continue" | "end")
            });
            let outgoing = left_beams.iter().any(|(number, state)| {
                *number == level && matches!(state.as_str(), "begin" | "continue")
            });
            if incoming {
                malformed |= cut || !outgoing || rest_beam_levels.contains(&level);
                let mut begin = None;
                for (note, raw) in interval[..=left_index].iter().rev() {
                    if note.is_rest && !note.is_hidden {
                        continue;
                    }
                    let beams = partial_beat_observed_beam_states(&raw.text)?;
                    match beams.iter().find(|(number, _)| *number == level)
                        .map(|(_, state)| state.as_str())
                    {
                        Some("continue") => {}
                        Some("begin") => {
                            begin = Some(begin.is_none());
                            if begin == Some(false) {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                malformed |= begin != Some(true);
            } else if outgoing {
                malformed = true;
            } else if cut {
                let previous_pitch = left_index.checked_sub(1)
                    .and_then(|index| interval.get(index))
                    .filter(|(note, _)| partial_beat_is_beam_candidate(note));
                let unfinished_preceding_beam = if let Some((_, raw)) = previous_pitch {
                    partial_beat_observed_beam_states(&raw.text)?.iter().any(|(number, state)| {
                        *number == level && matches!(state.as_str(), "begin" | "continue")
                    }) && !left_beams.iter().any(|(number, _)| *number == level)
                } else {
                    false
                };
                malformed |= unfinished_preceding_beam;
            }
        }
        candidates += usize::from(malformed);
    }
    Some(candidates)
}

#[cfg(test)]
fn read_fixture_rest_beam_errors(xml: &str) -> Option<usize> {
    count_written_rest_beam_errors(xml, |_| true)
}

#[cfg(test)]
mod lower_dual_edge_recovery_retirement_tests {
    use super::read_fixture_rest_beam_errors;

    pub(super) fn score(second_level1: &str, second_level2: &str, rest_beams: &str) -> String {
        format!(
            r#"<?xml version="1.0"?><score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="1"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff><beam number="1">begin</beam><beam number="2">begin</beam></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff><beam number="1">{}</beam><beam number="2">{}</beam></note><note><rest/><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff>{}</note><note><pitch><step>E</step><octave>3</octave></pitch><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff><beam number="1">end</beam><beam number="2">end</beam></note></measure></part></score-partwise>"#,
            second_level1, second_level2, rest_beams
        )
    }

    #[test]
    fn rest_bridge_accepts_balanced_written_beams() {
        let xml = score("continue", "continue", "");
        assert_eq!(
            read_fixture_rest_beam_errors(&xml),
            Some(0)
        );

        // Historical counterexample A remains an explicit negative case:
        // the old rest-carrier projection must be rejected by committed readback.
        let legacy_rest_carrier = score(
            "continue",
            "continue",
            r#"<beam number="1">continue</beam><beam number="2">continue</beam>"#,
        );
        assert_eq!(
            read_fixture_rest_beam_errors(&legacy_rest_carrier),
            Some(1)
        );
    }

    #[test]
    fn closed_beams_allow_a_split_even_without_a_clef_boundary() {
        let before = score("end", "end", "").replace(
            "</time>",
            r#"</time><clef number="2"><sign>F</sign><line>4</line></clef>"#,
        );
        let terminal_beams = r#"<beam number="1">end</beam><beam number="2">end</beam>"#;
        let (prefix, suffix) = before.rsplit_once(terminal_beams).unwrap();
        let split = format!("{prefix}{suffix}");
        assert_eq!(read_fixture_rest_beam_errors(&split), Some(0));

        let clef = r#"<attributes><clef number="2"><sign>G</sign><line>2</line></clef></attributes>"#;
        let bounded = split.replace("<note><rest/>", &format!("{clef}<note><rest/>"));
        assert_eq!(read_fixture_rest_beam_errors(&bounded), Some(0));
        let other_staff = bounded.replace(r#"<clef number="2">"#, r#"<clef number="1">"#);
        assert_eq!(read_fixture_rest_beam_errors(&other_staff), Some(0));
        let incomplete = bounded.replacen(terminal_beams, r#"<beam number="1">end</beam>"#, 1);
        assert_eq!(read_fixture_rest_beam_errors(&incomplete), Some(1));
    }

    #[test]
    fn a_written_beam_can_continue_across_multiple_rest_gaps() {
        let second_head = "<note><pitch><step>D</step>";
        let xml = score("continue", "continue", "").replace(
            second_head,
            &format!("<note><rest/><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff></note>{second_head}"),
        );
        assert_eq!(read_fixture_rest_beam_errors(&xml), Some(0));
    }

    #[test]
    fn level_two_duplicate_begin_remains_visible_after_primary_masking() {
        let xml = score("continue", "begin", "");
        assert_eq!(
            read_fixture_rest_beam_errors(&xml),
            Some(1)
        );

        // Historical counterexample B is still fail-closed: projecting the new
        // rest-free canonical state must not make a lost primary begin invisible.
        let missing_begin = score("continue", "continue", "").replacen(
            r#"<beam number="1">begin</beam>"#,
            "",
            1,
        );
        assert_eq!(
            read_fixture_rest_beam_errors(&missing_begin),
            Some(1)
        );
    }
}

#[cfg(test)]
mod lower_dual_edge_multimeasure_detector_tests {
    use super::read_fixture_rest_beam_errors;
fn note(staff: &str, voice: &str, rest: bool, beams: (&str, &str)) -> String {
        let body = if rest {
            "<rest/>".to_string()
        } else {
            "<pitch><step>C</step><octave>4</octave></pitch>".to_string()
        };
        let beam_rows = if rest || beams.0.is_empty() {
            String::new()
        } else {
            format!(
                "<beam number=\"1\">{}</beam><beam number=\"2\">{}</beam>",
                beams.0, beams.1
            )
        };
        format!(
            "<note>{body}<duration>24</duration><type>16th</type><voice>{voice}</voice><staff>{staff}</staff>{beam_rows}</note>"
        )
    }
fn exact_structural_measure(measure_number: usize, second_state: &str) -> String {
        let mut lower = String::new();
        let states = [
            ("begin", "begin"),
            (second_state, second_state),
            ("continue", "continue"),
            ("end", "end"),
        ];
        for (slot, state) in states.iter().enumerate() {
            lower.push_str(&note("2", "2", slot == 2, *state));
        }
        for _ in 0..8 {
            lower.push_str(&note("2", "2", false, ("", "")));
        }
        for (slot, state) in states.iter().enumerate() {
            lower.push_str(&note("2", "2", slot == 2, *state));
        }
        let previous_measure = measure_number.saturating_sub(1);
        format!(
            "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{previous_measure}\"><attributes><divisions>96</divisions></attributes><forward><duration>384</duration></forward></measure><measure number=\"{measure_number}\">{lower}</measure></part></score-partwise>"
        )
    }
fn structural_measure_fragment(measure_number: usize, second_state: &str) -> String {
        let full = exact_structural_measure(measure_number, second_state);
        let marker = format!("<measure number=\"{measure_number}\">");
        let start = full.find(&marker).expect("target measure");
        let end = full[start..]
            .find("</measure>")
            .map(|offset| start + offset + "</measure>".len())
            .expect("measure end");
        full[start..end].to_string()
    }
    #[test]
    fn detector_retains_green_double_begin_and_multimeasure_counts() {
        for ordinal in [11_usize, 43] {
            assert_eq!(read_fixture_rest_beam_errors(
                &exact_structural_measure(ordinal, "continue")), Some(0));
            assert_eq!(read_fixture_rest_beam_errors(
                &exact_structural_measure(ordinal, "begin")), Some(2));
        }
        let first = 11_usize;
        let second = 43_usize;
        let prefix = first - 1;
        let measures = [structural_measure_fragment(first, "begin"),
            structural_measure_fragment(second, "begin")].join("");
        let combined = format!("<score-partwise><part id=\"P1\"><measure number=\"{prefix}\"><attributes><divisions>96</divisions></attributes><forward><duration>384</duration></forward></measure>{measures}</part></score-partwise>");
        assert_eq!(read_fixture_rest_beam_errors(&combined), Some(4));
    }
}

#[cfg(test)]
#[path = "../../tests/unit/written_rest_beam_readback.rs"]
mod written_rest_beam_readback_tests;
