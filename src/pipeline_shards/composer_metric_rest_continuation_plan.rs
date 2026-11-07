/// Alternative metric plans for extending a contiguous written run across
/// represented rest time. Their note ranges bind only to this input document.
fn metric_rest_continuation_plans<'a>(
    infos: &'a [PartialBeatNoteInfo],
    beat_unit: i64,
    clef_boundaries: &BTreeMap<String, Vec<i64>>,
) -> Vec<(Vec<&'a PartialBeatNoteInfo>, structural_write_authority::BeamPlan)> {
    let mut strands = BTreeMap::<_, Vec<_>>::new();
    for note in infos.iter().filter(|note| !note.is_rest && !note.is_chord && note.duration > 0) {
        strands.entry((&note.staff, &note.voice, note.start.div_euclid(beat_unit)))
            .or_default().push(note);
    }

    let mut plans = Vec::new();
    for ((staff, voice, beat), mut notes) in strands {
        notes.sort_by_key(|note| (note.start, note.index));
        let beat_start = beat * beat_unit;
        let crossing_pitch = infos.iter().any(|note| !note.is_rest && !note.is_chord
            && &note.staff == staff && &note.voice == voice
            && note.start < beat_start && note.start + note.duration > beat_start);
        if crossing_pitch || notes.windows(2).any(|pair| pair[0].start + pair[0].duration > pair[1].start) {
            continue;
        }
        let boundaries = clef_boundaries.get(staff).map_or(&[][..], Vec::as_slice);
        let parts = notes.chunk_by(|left, right| {
            let end = left.start + left.duration;
            partial_beat_is_beam_candidate(left) && partial_beat_is_beam_candidate(right)
                && !crosses_metric_beam_clef_boundary(boundaries, left.start, right.start)
                && (end == right.start || (end < right.start && visible_rests_cover_gap(infos, left, right)))
        });
        for run in parts {
            if !beat_group_stays_inside_one_beat(run, beat_unit) {
                continue;
            }
            let contiguous = |pair: &[&PartialBeatNoteInfo]| pair[0].start + pair[0].duration == pair[1].start;
            if !run.windows(2).any(contiguous) || run.windows(2).all(contiguous) {
                continue;
            }
            if let Some(decision) = structural_metric_beam_hierarchy_plan(run, 4) {
                plans.push((run.to_vec(), decision.into_plan()));
            }
        }
    }
    plans
}

fn observe_metric_rest_continuation(
    xml: &str,
    guard: &structural_write_authority::CanonicalFinalChordStructuralPlan,
) {
    let mut inherited_divisions = 1;
    let mut groups = 0;
    let mut rest_edges = 0;
    for span in partial_beat_measure_spans(xml) {
        let measure = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure, "divisions")
            .unwrap_or(inherited_divisions).max(1);
        let Some(boundaries) = metric_beam_clef_boundaries(measure) else { continue; };
        let raw_notes = partial_beat_raw_notes(measure);
        let infos = partial_beat_note_infos(measure, &raw_notes);
        let ranges = raw_notes.iter().map(|note| (note.start, (span.start + note.start, span.start + note.end)))
            .collect::<BTreeMap<_, _>>();
        let measure_label = partial_beat_measure_number(measure);
        for (run, plan) in metric_rest_continuation_plans(&infos, inherited_divisions, &boundaries) {
            let Some(members) = run.iter().map(|note| ranges.get(&note.index).copied())
                .collect::<Option<Vec<_>>>() else { continue; };
            let voice_binding = match guard.bridge_voice_admission(xml, &members) {
                Some(true) => "SealedCommon",
                Some(false) => "SealedSplit",
                None => "Unknown",
            };
            let mut prior = BTreeMap::<u8, usize>::new();
            groups += 1;
            for edit in plan.edits() {
                let level = edit.level();
                let ordinal = edit.event_ordinal();
                let marker = edit.state().as_musicxml_text();
                if marker != "begin"
                    && let Some(&left) = prior.get(&level)
                    && run[left].start + run[left].duration < run[ordinal].start
                {
                    rest_edges += 1;
                    eprintln!("MetricRestContinuationPlanEdge: measure={measure_label:?}; level={level}; left_range={:?}; right_range={:?}; voice_binding={voice_binding}; authority=none",
                        members[left], members[ordinal]);
                }
                if marker == "end" {
                    prior.remove(&level);
                } else {
                    prior.insert(level, ordinal);
                }
            }
        }
    }
    eprintln!("MetricRestContinuationPlan: scope=early_e050_input; groups={groups}; rest_edges={rest_edges}; authority=none");
}

#[cfg(test)]
mod metric_rest_continuation_tests {
    use super::*;

    #[test]
    fn rest_continuation_requires_written_coverage_and_keeps_metric_boundaries() {
        let pitched = |step| format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>24</duration><voice>synthetic</voice><type>16th</type><staff>1</staff></note>");
        let visible = "<note><rest/><duration>24</duration><voice>synthetic</voice><type>16th</type><staff>1</staff></note>";
        let hidden = visible.replacen("<note>", "<note print-object=\"no\">", 1);
        let forward = "<forward><duration>24</duration></forward>";
        let attributes = r#"<attributes><divisions>96</divisions><clef number="1"><sign>G</sign><line>2</line></clef></attributes>"#;
        let clef_change = r#"<attributes><clef number="1"><sign>F</sign><line>4</line></clef></attributes>"#;
        for (gap, clef, offset, expected) in [
            (visible, "", "", true),
            (hidden.as_str(), "", "", false),
            (forward, "", "", false),
            (visible, clef_change, "", false),
            (visible, "", forward, false),
        ] {
            for label in ["arbitrary", "renamed"] {
                let measure = format!(r#"<measure number="{label}">{attributes}{offset}{}{}{gap}{clef}{}</measure>"#,
                    pitched("C"), pitched("D"), pitched("E"));
                let raw = partial_beat_raw_notes(&measure);
                let infos = partial_beat_note_infos(&measure, &raw);
                let boundaries = metric_beam_clef_boundaries(&measure).unwrap();
                let plans = metric_rest_continuation_plans(&infos, 96, &boundaries);
                if expected {
                    let [(run, plan)] = plans.as_slice() else { panic!("expected one continued run"); };
                    assert_eq!(run.iter().map(|note| note.start).collect::<Vec<_>>(), [0, 24, 72]);
                    for level in [1, 2] {
                        assert_eq!(plan.edits().iter().filter(|edit| edit.level() == level)
                            .map(|edit| edit.state().as_musicxml_text()).collect::<Vec<_>>(),
                            ["begin", "continue", "end"]);
                    }
                } else {
                    assert!(plans.is_empty(), "{measure}");
                }
            }
        }
    }
}
