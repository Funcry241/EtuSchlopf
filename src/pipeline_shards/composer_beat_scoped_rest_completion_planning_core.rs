// Pure beat-scoped rest-completion planning. No XML materialization, writer, filesystem sink, or report projection.

#[derive(Debug, Clone, PartialEq, Eq)]
struct BeatScopedRestCompletionGapPlan {
    insert_before: usize,
    staff: String,
    voice: String,
    gap_start: i64,
    gap_end: i64,
}

fn plan_visible_voice_rest_completion_gaps(
    note_infos: &[PartialBeatNoteInfo],
    measure_duration: i64,
) -> (Vec<BeatScopedRestCompletionGapPlan>, BTreeSet<String>) {
    let mut pitched_by_key: BTreeMap<(String, String), Vec<&PartialBeatNoteInfo>> = BTreeMap::new();
    for info in note_infos {
        if !info.is_hidden && !info.is_rest && !info.is_chord && info.duration > 0 {
            pitched_by_key
                .entry((info.staff.clone(), info.voice.clone()))
                .or_default()
                .push(info);
        }
    }

    let mut plans = Vec::new();
    let mut cross_staff_suppressed_staffs = BTreeSet::new();
    for ((staff, voice), mut notes) in pitched_by_key {
        notes.sort_by_key(|info| (info.start, info.index));
        for pair in notes.windows(2) {
            let left = pair[0];
            let right = pair[1];
            let gap_start = (left.start + left.duration).max(0);
            let gap_end = right.start.min(measure_duration);
            if gap_end <= gap_start {
                continue;
            }
            // A staff-local visual gap is not a proven voice silence while the
            // same voice carries pitched material on another staff (or in a hidden/
            // chord anchor) inside that span. Treating such cross-staff motion as
            // silence would insert cursor-advancing rests into an already occupied
            // MusicXML lane and can push the measure beyond its metric duration.
            let same_voice_pitched_material_in_gap = note_infos.iter().any(|candidate| {
                candidate.voice == voice
                    && !candidate.is_rest
                    && candidate.duration > 0
                    && candidate.start < gap_end
                    && candidate.start.saturating_add(candidate.duration) > gap_start
            });
            if same_voice_pitched_material_in_gap {
                cross_staff_suppressed_staffs.insert(staff.clone());
                continue;
            }
            if existing_visible_rests_cover_range(
                note_infos,
                &staff,
                &voice,
                gap_start,
                gap_end,
            ) {
                continue;
            }
            plans.push(BeatScopedRestCompletionGapPlan {
                insert_before: right.index,
                staff: staff.clone(),
                voice: voice.clone(),
                gap_start,
                gap_end,
            });
        }
    }
    (plans, cross_staff_suppressed_staffs)
}


fn plan_exact_metric_visible_rest_durations(
    start: i64,
    duration: i64,
    measure_duration: i64,
    beat_unit: i64,
) -> Option<Vec<i64>> {
    if start == 0 && duration == measure_duration {
        return None;
    }
    let durations = crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
        start,
        duration,
        measure_duration,
        beat_unit,
    );
    if durations.is_empty() || durations.iter().copied().sum::<i64>() != duration {
        None
    } else {
        Some(durations)
    }
}

fn existing_visible_rests_cover_range(
    note_infos: &[PartialBeatNoteInfo],
    staff: &str,
    voice: &str,
    start: i64,
    end: i64,
) -> bool {
    let mut rests = note_infos
        .iter()
        .filter(|info| {
            info.staff == staff
                && info.voice == voice
                && info.is_rest
                && !info.is_hidden
                && !info.is_chord
                && info.duration > 0
                && info.start >= start
                && info.start < end
        })
        .collect::<Vec<_>>();
    rests.sort_by_key(|info| (info.start, info.index));
    let mut cursor = start;
    for rest in rests {
        if rest.start != cursor {
            return false;
        }
        cursor = rest.start + rest.duration;
        if cursor > end {
            return false;
        }
    }
    cursor == end
}

#[cfg(test)]
mod beat_scoped_rest_completion_planning_boundary_tests {
    use super::{
        add_visible_voice_rest_completion, beat_scoped_rest_completion_cursor_state,
        beat_scoped_rest_completion_pitched_timeline,
        close_beat_scoped_rest_completion_crossing_rest_suffix,
        close_beat_scoped_rest_completion_crossing_rests_in_xml, partial_beat_note_infos,
        partial_beat_raw_notes, patch_beat_scoped_beaming_in_measure,
        patch_beat_scoped_rest_completion_in_measure, plan_exact_metric_visible_rest_durations,
        plan_visible_voice_rest_completion_gaps, BeatScopedRestCompletionGapPlan,
        BeatScopedRestPatchStats,
    };

    #[test]
    fn exact_metric_rest_plan_preserves_the_requested_cursor_duration() {
        let durations = plan_exact_metric_visible_rest_durations(0, 336, 384, 96)
            .expect("normalized metric plan");
        assert_eq!(durations, vec![192, 96, 48]);
        assert_eq!(durations.iter().copied().sum::<i64>(), 336);
        assert!(plan_exact_metric_visible_rest_durations(0, 168, 192, 48).is_none());
    }

    #[test]
    fn complete_measure_rest_stays_with_the_existing_full_measure_owner() {
        assert!(plan_exact_metric_visible_rest_durations(0, 384, 384, 96).is_none());

        let measure = "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes>\n<note><rest/><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note>\n</measure>";
        let (patched, stats) = patch_beat_scoped_rest_completion_in_measure(measure, 96);
        assert_eq!(patched, measure);
        assert_eq!(stats.rest_notes_inserted, 0);
        assert_eq!(stats.rests_rewritten, 0);
    }

    #[test]
    fn final_projection_crossing_rest_closure_is_identity_free_and_idempotent() {
        let measure = "<measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>\n<note><pitch><step>C</step><octave>5</octave></pitch><duration>240</duration><voice>1</voice><staff>1</staff></note>\n<note><rest/><duration>48</duration><type>eighth</type><voice>1</voice><staff>2</staff></note>\n<note><rest/><duration>24</duration><type>16th</type><voice>1</voice><staff>2</staff></note>\n<note><rest/><duration>96</duration><type>quarter</type><voice>1</voice><staff>2</staff></note>\n<backup><duration>384</duration></backup>\n<forward><duration>96</duration></forward>\n<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><staff>2</staff></note>\n<forward><duration>240</duration></forward>\n</measure>";
        let before = beat_scoped_rest_completion_cursor_state(measure, 384);
        assert_eq!(before.cursor_max, 408);
        assert_eq!(before.partial_backup_count, 1);

        let (closed, closed_measures) =
            close_beat_scoped_rest_completion_crossing_rests_in_xml(measure).0;
        assert_eq!(closed_measures, 1);
        assert!(beat_scoped_rest_completion_cursor_state(&closed, 384).is_green(384));
        assert!(close_beat_scoped_rest_completion_crossing_rest_suffix(&closed, 384, 96)
            .is_none());

        let (second, second_closed_measures) =
            close_beat_scoped_rest_completion_crossing_rests_in_xml(&closed).0;
        assert_eq!(second_closed_measures, 0);
        assert_eq!(second, closed);
    }

    #[test]
    fn metric_rest_materialization_uses_one_half_rest_for_an_aligned_two_beat_gap() {
        let plans = vec![BeatScopedRestCompletionGapPlan {
            insert_before: 42,
            staff: "1".to_string(),
            voice: "1".to_string(),
            gap_start: 0,
            gap_end: 192,
        }];
        let mut inserts = std::collections::BTreeMap::new();
        let mut staffs = std::collections::BTreeSet::new();
        let mut stats = BeatScopedRestPatchStats::default();
        add_visible_voice_rest_completion(
            &plans,
            384,
            96,
            &mut inserts,
            &mut staffs,
            &mut stats,
        );
        let created = inserts.get(&42).expect("one materialization anchor");
        assert_eq!(created.len(), 1);
        assert!(created[0].contains("<duration>192</duration>"));
        assert!(created[0].contains("<type>half</type>"));
        assert_eq!(stats.rest_notes_inserted, 1);
    }

    #[test]
    fn beam_only_stage_never_creates_a_rest_note() {
        let measure = "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes>\n<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff></note>\n<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff></note>\n</measure>";
        let (patched, stats) = patch_beat_scoped_beaming_in_measure(measure, 96);
        assert_eq!(stats.groups, 1);
        assert!(!patched.contains("<rest"));
        assert!(patched.contains("<beam number=\"1\">begin</beam>"));
        assert!(patched.contains("<beam number=\"1\">end</beam>"));
    }

    #[test]
    fn rest_only_stage_preserves_existing_beam_tags_when_no_rest_write_is_needed() {
        let measure = "<measure number=\"\
1\"><attributes><divisions>96</divisions></attributes>\n<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>\n<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><type>16th</type><voice>1</voice><staff>1</staff><beam number=\"1\">end</beam></note>\n</measure>";
        let (patched, stats) = patch_beat_scoped_rest_completion_in_measure(measure, 96);
        assert_eq!(patched, measure);
        assert_eq!(stats.rest_notes_inserted, 0);
        assert_eq!(stats.rests_rewritten, 0);
    }

    #[test]
    fn existing_visible_rest_coverage_preserves_the_current_no_gap_decision() {
        let measure = "<measure number=\"\
1\"><attributes><divisions>24</divisions></attributes>\n<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>\n<note><rest/><duration>24</duration><voice>1</voice><staff>1</staff></note>\n<note><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>\n</measure>";
        let raw = partial_beat_raw_notes(measure);
        let observed = partial_beat_note_infos(measure, &raw);
        let (plans, cross_staff_suppressed_staffs) =
            plan_visible_voice_rest_completion_gaps(&observed, 96);
        assert!(plans.is_empty());
        assert!(cross_staff_suppressed_staffs.is_empty());
    }

    #[test]
    fn cross_staff_same_voice_material_blocks_false_gap_completion_and_preserves_cursor() {
        let measure = "<measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>\n<note><pitch><step>C</step><octave>5</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note>\n<note><pitch><step>E</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>2</staff></note>\n<note><pitch><step>G</step><octave>5</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note>\n<note><pitch><step>B</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>2</staff></note>\n</measure>";
        let raw = partial_beat_raw_notes(measure);
        let observed = partial_beat_note_infos(measure, &raw);
        let (plans, cross_staff_suppressed_staffs) =
            plan_visible_voice_rest_completion_gaps(&observed, 384);
        assert!(plans.is_empty());
        assert_eq!(
            cross_staff_suppressed_staffs,
            ["1".to_string(), "2".to_string()].into_iter().collect()
        );

        let before = beat_scoped_rest_completion_cursor_state(measure, 384);
        assert!(before.is_green(384));
        let (patched, stats) = patch_beat_scoped_rest_completion_in_measure(measure, 96);
        assert_eq!(patched, measure);
        assert_eq!(stats.rest_notes_inserted, 0);
        assert!(beat_scoped_rest_completion_cursor_state(&patched, 384).is_green(384));
    }
    #[test]
    fn cross_staff_suppressed_gap_keeps_aux_shadow_rest_hidden_without_rest_insertion() {
        let measure = "<measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>\n<note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>2</staff></note>\n<note><pitch><step>E</step><octave>5</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note>\n<note><pitch><step>G</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>2</staff></note>\n<note><pitch><step>B</step><octave>5</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note>\n<backup><duration>384</duration></backup>\n<note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>\n<note print-object=\"no\" print-spacing=\"no\"><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>\n<note print-object=\"no\" print-spacing=\"no\"><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>\n<note print-object=\"no\" print-spacing=\"no\"><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>\n</measure>";

        let before_pitched = beat_scoped_rest_completion_pitched_timeline(measure);
        let (patched, stats) = patch_beat_scoped_rest_completion_in_measure(measure, 96);
        assert_eq!(stats.rest_notes_inserted, 0);
        assert_eq!(stats.rest_only_aux_rests_hidden, 1);
        assert_eq!(
            patched.matches("<note print-object=\"no\" print-spacing=\"no\"><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>").count(),
            4
        );
        assert_eq!(
            before_pitched,
            beat_scoped_rest_completion_pitched_timeline(&patched)
        );
        assert!(beat_scoped_rest_completion_cursor_state(&patched, 384).is_green(384));
    }

}
