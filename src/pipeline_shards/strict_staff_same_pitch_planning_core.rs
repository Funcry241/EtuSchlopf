// Writer-free visible-pitch observation and duplicate-suppression planning.

type StrictStaffDuplicateSamePitchCandidate = (
    usize,
    String,
    String,
    i32,
    i32,
    i32,
    Vec<RenderXmlTimelineFragmentSpan>,
);

#[derive(Debug, Clone)]
struct StrictStaffVisiblePitchObservation {
    span: RenderXmlTimelineFragmentSpan,
    midi: i32,
}

#[derive(Debug, Clone)]
struct StrictStaffVisiblePitchMeasureObservation {
    scope: usize,
    notes: Vec<StrictStaffVisiblePitchObservation>,
}

#[derive(Debug, Clone, Copy)]
enum StrictStaffSamePitchGrouping {
    StaffAcrossVoices,
    ExactVoice,
}

#[derive(Debug, Default)]
struct StrictStaffSamePitchSuppressionPlan {
    candidates_seen: usize,
    replacements: Vec<(usize, usize, String)>,
    actions: Vec<BehindBarsRuleCheckRestAndTieAction>,
}

fn collect_strict_staff_visible_pitch_observations(
    xml: &str,
) -> Vec<StrictStaffVisiblePitchMeasureObservation> {
    let mut measures = Vec::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) =
        find_next_measure_bounds(xml, scan)
    {
        let measure = &xml[measure_start..measure_end];
        let mut notes = Vec::new();
        for span in collect_render_timeline_fragments_in_segment(
            measure,
            measure_start,
            scope_number,
        )
        .into_iter()
        .filter(|span| span.is_visible_pitched && span.duration > 0 && !span.is_hidden)
        {
            let Some(midi) = midi_from_note_fragment(&span.fragment) else {
                continue;
            };
            notes.push(StrictStaffVisiblePitchObservation { span, midi });
        }
        measures.push(StrictStaffVisiblePitchMeasureObservation {
            scope: scope_number,
            notes,
        });
        scan = measure_end;
    }
    measures
}

fn hide_visible_duplicate_note_fragment(fragment: &str) -> String {
    if fragment.contains("print-object=\"no\"") {
        return fragment.to_string();
    }
    if fragment.starts_with("<note>") || fragment.starts_with("      <note>") {
        fragment.replacen("<note>", "<note print-object=\"no\" print-spacing=\"no\">", 1)
    } else {
        fragment.replacen("<note", "<note print-object=\"no\" print-spacing=\"no\"", 1)
    }
}

fn plan_strict_staff_duplicate_same_pitch_suppression(
    xml: &str,
    max_actions: usize,
    grouping: StrictStaffSamePitchGrouping,
    action_kind: &'static str,
) -> StrictStaffSamePitchSuppressionPlan {
    let mut candidates: Vec<StrictStaffDuplicateSamePitchCandidate> = Vec::new();
    for measure in collect_strict_staff_visible_pitch_observations(xml) {
        let mut by_note: BTreeMap<
            (String, String, i32, i32, i32),
            Vec<RenderXmlTimelineFragmentSpan>,
        > = BTreeMap::new();
        for observation in measure.notes {
            let span = observation.span;
            let grouping_voice = match grouping {
                StrictStaffSamePitchGrouping::StaffAcrossVoices => String::new(),
                StrictStaffSamePitchGrouping::ExactVoice => span.voice.clone(),
            };
            by_note
                .entry((
                    span.staff.clone(),
                    grouping_voice,
                    span.onset,
                    span.duration,
                    observation.midi,
                ))
                .or_default()
                .push(span);
        }
        for ((staff, voice, onset, duration, midi), mut group) in by_note {
            if group.len() < 2 {
                continue;
            }
            group.sort_by_key(|span| {
                (
                    if span.is_chord { 1 } else { 0 },
                    span.voice.clone(),
                    span.start,
                )
            });
            candidates.push((
                measure.scope,
                staff,
                voice,
                onset,
                duration,
                midi,
                group,
            ));
        }
    }

    let candidates_seen = candidates.len();
    candidates.sort_by_key(
        |(scope, staff, voice, onset, _duration, midi, _group)| {
            (
                *scope,
                staff.clone(),
                voice.clone(),
                *onset,
                *midi,
            )
        },
    );

    let mut replacements = Vec::new();
    let mut actions = Vec::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    for (_, _, _, _, _, _, group) in candidates {
        if actions.len() >= max_actions {
            break;
        }
        let mut hidden_count = 0_usize;
        for span in group.iter().skip(1) {
            if occupied.iter().any(|(used_start, used_end)| {
                span.start < *used_end && span.end > *used_start
            }) {
                continue;
            }
            occupied.push((span.start, span.end));
            replacements.push((
                span.start,
                span.end,
                hide_visible_duplicate_note_fragment(&span.fragment),
            ));
            hidden_count += 1;
        }
        if hidden_count > 0 {
            actions.push(BehindBarsRuleCheckRestAndTieAction {
                action_kind: action_kind.to_string(),
                old_rest_count: group.len(),
                new_rest_count: group.len().saturating_sub(hidden_count),
                verdict: "adopted_if_dedup_sounding_invariant_holds".to_string(),
            });
        }
    }
    replacements.sort_by_key(|left| std::cmp::Reverse(left.0));

    StrictStaffSamePitchSuppressionPlan {
        candidates_seen,
        replacements,
        actions,
    }
}
