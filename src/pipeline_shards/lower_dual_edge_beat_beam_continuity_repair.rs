

fn lower_dual_edge_beat_structural_group_with_voice_binding(
    infos: &[PartialBeatNoteInfo],
    raw_notes: &[PartialBeatRawNote],
    offsets: &[i64; 4],
    sixteenth: i64,
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> Option<[usize; 4]> {
    let mut indices = [0_usize; 4];
    for (slot, offset) in offsets.iter().enumerate() {
        let matches = infos
            .iter()
            .filter(|info| {
                info.staff == "2"
                    && voice_plan.label_is_lower_dual_edge_primary(&info.voice) == Some(true)
                    && !info.is_chord
                    && info.start == *offset
                    && info.duration == sixteenth
                    && info.note_type == "16th"
            })
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return None;
        }
        let info = matches[0];
        raw_notes.iter().find(|raw| raw.start == info.index)?;
        if slot == 2 {
            if !info.is_rest {
                return None;
            }
        } else if info.is_rest {
            return None;
        }
        indices[slot] = info.index;
    }
    Some(indices)
}

pub(crate) fn lower_dual_edge_beat_regression_candidates_with_voice_binding(
    xml: &str,
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> Option<usize> {
    if !voice_plan.is_bound() {
        return None;
    }
    parse_musicxml(xml).ok()?;
    let spans = partial_beat_measure_spans(xml);
    let mut inherited_divisions = 1_i64;
    let mut candidates = 0_usize;
    for span in spans {
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let beat_unit = inherited_divisions;
        let sixteenth = (beat_unit / 4).max(1);
        let first_offsets = [0_i64, sixteenth, sixteenth * 2, sixteenth * 3];
        let last_offsets = [
            beat_unit * 3,
            beat_unit * 3 + sixteenth,
            beat_unit * 3 + sixteenth * 2,
            beat_unit * 3 + sixteenth * 3,
        ];
        for offsets in [&first_offsets, &last_offsets] {
            let Some(group) = lower_dual_edge_beat_structural_group_with_voice_binding(
                &infos,
                &raw_notes,
                offsets,
                sixteenth,
                voice_plan,
            ) else {
                continue;
            };
            if !lower_dual_edge_beat_group_matches_plan(&infos, &raw_notes, &group) {
                candidates = candidates.saturating_add(1);
            }
        }
    }
    Some(candidates)
}







fn lower_dual_edge_beat_after_plan(
    infos: &[PartialBeatNoteInfo],
    indices: &[usize; 4],
) -> Option<(structural_write_authority::BeamPlan, [usize; 3])> {
    // r331: the retired LowerDualEdge readback projects the living shared
    // metric BeamHierarchy DecisionOwner instead of preserving the historical
    // rest-carrier state.  The rest at slot 2 remains structural gap evidence
    // and therefore has no BeamPlan event ordinal.
    let sounding_indices = [indices[0], indices[1], indices[3]];
    let sounding_infos = sounding_indices
        .iter()
        .map(|index| infos.iter().find(|info| info.index == *index))
        .collect::<Option<Vec<_>>>()?;
    let plan = structural_metric_beam_hierarchy_plan(&sounding_infos, 2)?.into_plan();
    Some((plan, sounding_indices))
}

fn lower_dual_edge_beat_group_matches_plan(
    infos: &[PartialBeatNoteInfo],
    raw_notes: &[PartialBeatRawNote],
    indices: &[usize; 4],
) -> bool {
    let Some((plan, sounding_indices)) = lower_dual_edge_beat_after_plan(infos, indices) else {
        return false;
    };
    let sounding_green = sounding_indices
        .iter()
        .enumerate()
        .all(|(event_ordinal, index)| {
            let Some(raw) = raw_notes.iter().find(|raw| raw.start == *index) else {
                return false;
            };
            lower_dual_edge_beat_beam_states(&raw.text)
                == beam_plan_states_for_event(&plan, event_ordinal)
        });
    if !sounding_green {
        return false;
    }
    let Some(rest_raw) = raw_notes.iter().find(|raw| raw.start == indices[2]) else {
        return false;
    };
    lower_dual_edge_beat_beam_states(&rest_raw.text).is_empty()
}

fn beam_plan_states_for_event(
    plan: &structural_write_authority::BeamPlan,
    event_ordinal: usize,
) -> std::collections::BTreeMap<u8, String> {
    plan.edits()
        .iter()
        .copied()
        .filter(|edit| edit.event_ordinal() == event_ordinal)
        .map(|edit| (edit.level(), edit.state().as_musicxml_text().to_string()))
        .collect()
}

fn lower_dual_edge_beat_beam_states(note_text: &str) -> std::collections::BTreeMap<u8, String> {
    let mut states = std::collections::BTreeMap::new();
    let mut scan = 0_usize;
    while let Some(rel_start) = note_text[scan..].find("<beam") {
        let start = scan + rel_start;
        let Some(open_end_rel) = note_text[start..].find('>') else {
            break;
        };
        let open_end = start + open_end_rel;
        let open_tag = &note_text[start..=open_end];
        let Some(number_pos) = open_tag.find("number=\"") else {
            scan = open_end + 1;
            continue;
        };
        let number_start = number_pos + "number=\"".len();
        let Some(number_end_rel) = open_tag[number_start..].find('"') else {
            scan = open_end + 1;
            continue;
        };
        let number = open_tag[number_start..number_start + number_end_rel]
            .trim()
            .parse::<u8>()
            .ok();
        let content_start = open_end + 1;
        let Some(close_rel) = note_text[content_start..].find("</beam>") else {
            break;
        };
        let content_end = content_start + close_rel;
        if let Some(level) = number {
            states.insert(level, note_text[content_start..content_end].trim().to_string());
        }
        scan = content_end + "</beam>".len();
    }
    states
}





#[cfg(test)]
fn lower_dual_edge_beat_structural_group(
    infos: &[PartialBeatNoteInfo],
    raw_notes: &[PartialBeatRawNote],
    offsets: &[i64; 4],
    sixteenth: i64,
) -> Option<[usize; 4]> {
    let mut indices = [0_usize; 4];
    for (slot, offset) in offsets.iter().enumerate() {
        let matches = infos
            .iter()
            .filter(|info| {
                info.staff == "2"
                    && info.voice == "2"
                    && !info.is_chord
                    && info.start == *offset
                    && info.duration == sixteenth
                    && info.note_type == "16th"
            })
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return None;
        }
        let info = matches[0];
        raw_notes.iter().find(|raw| raw.start == info.index)?;
        if slot == 2 {
            if !info.is_rest {
                return None;
            }
        } else if info.is_rest {
            return None;
        }
        indices[slot] = info.index;
    }
    Some(indices)
}

#[cfg(test)]
pub(crate) fn lower_dual_edge_beat_regression_candidates(xml: &str) -> Option<usize> {
    parse_musicxml(xml).ok()?;
    let spans = partial_beat_measure_spans(xml);
    let mut inherited_divisions = 1_i64;
    let mut candidates = 0_usize;
    for span in spans {
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let beat_unit = inherited_divisions;
        let sixteenth = (beat_unit / 4).max(1);
        let first_offsets = [0_i64, sixteenth, sixteenth * 2, sixteenth * 3];
        let last_offsets = [
            beat_unit * 3,
            beat_unit * 3 + sixteenth,
            beat_unit * 3 + sixteenth * 2,
            beat_unit * 3 + sixteenth * 3,
        ];
        for offsets in [&first_offsets, &last_offsets] {
            let Some(group) = lower_dual_edge_beat_structural_group(
                &infos,
                &raw_notes,
                offsets,
                sixteenth,
            ) else {
                continue;
            };
            if !lower_dual_edge_beat_group_matches_plan(&infos, &raw_notes, &group) {
                candidates = candidates.saturating_add(1);
            }
        }
    }
    Some(candidates)
}

#[cfg(test)]
mod lower_dual_edge_recovery_retirement_tests {
    use super::lower_dual_edge_beat_regression_candidates;

    fn score(second_level1: &str, second_level2: &str, rest_beams: &str) -> String {
        format!(
            r#"<?xml version="1.0"?><score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="1"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff><beam number="1">begin</beam><beam number="2">begin</beam></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff><beam number="1">{}</beam><beam number="2">{}</beam></note><note><rest/><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff>{}</note><note><pitch><step>E</step><octave>3</octave></pitch><duration>1</duration><type>16th</type><voice>2</voice><staff>2</staff><beam number="1">end</beam><beam number="2">end</beam></note></measure></part></score-partwise>"#,
            second_level1, second_level2, rest_beams
        )
    }

    #[test]
    fn canonical_lower_rest_bridge_edge_is_not_a_regression() {
        let xml = score("continue", "continue", "");
        assert_eq!(
            lower_dual_edge_beat_regression_candidates(&xml),
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
            lower_dual_edge_beat_regression_candidates(&legacy_rest_carrier),
            Some(1)
        );
    }

    #[test]
    fn level_two_duplicate_begin_remains_visible_after_primary_masking() {
        let xml = score("continue", "begin", "");
        assert_eq!(
            lower_dual_edge_beat_regression_candidates(&xml),
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
            lower_dual_edge_beat_regression_candidates(&missing_begin),
            Some(1)
        );
    }
}

#[cfg(test)]
mod lower_dual_edge_multimeasure_detector_tests {
    use super::lower_dual_edge_beat_regression_candidates;
fn note(staff: &str, voice: &str, rest: bool, beams: (&str, &str)) -> String {
        let body = if rest {
            "<rest/>".to_string()
        } else {
            "<pitch><step>C</step><octave>4</octave></pitch>".to_string()
        };
        let beam_rows = if rest {
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
            lower.push_str(&note("2", "2", false, ("begin", "begin")));
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
            assert_eq!(lower_dual_edge_beat_regression_candidates(
                &exact_structural_measure(ordinal, "continue")), Some(0));
            assert_eq!(lower_dual_edge_beat_regression_candidates(
                &exact_structural_measure(ordinal, "begin")), Some(2));
        }
        let first = 11_usize;
        let second = 43_usize;
        let prefix = first - 1;
        let measures = [structural_measure_fragment(first, "begin"),
            structural_measure_fragment(second, "begin")].join("");
        let combined = format!("<score-partwise><part id=\"P1\"><measure number=\"{prefix}\"><attributes><divisions>96</divisions></attributes><forward><duration>384</duration></forward></measure>{measures}</part></score-partwise>");
        assert_eq!(lower_dual_edge_beat_regression_candidates(&combined), Some(4));
    }
}
