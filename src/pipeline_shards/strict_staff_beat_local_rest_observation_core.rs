#[derive(Debug, Clone)]
struct BeatLocalRestBackupObservation {
    start: usize,
    end: usize,
    duration: i32,
    cursor_before: i32,
    rest_lane_after: bool,
    fragment: String,
}

#[derive(Debug, Clone)]
struct BeatLocalVoiceRestMeasureObservation {
    measure_end: usize,
    expected: i32,
    beat: i32,
    final_cursor: i32,
    has_cross_staff_visible_voice: bool,
    fragments: Vec<RenderXmlTimelineFragmentSpan>,
    backups: Vec<BeatLocalRestBackupObservation>,
}

fn collect_beat_local_voice_rest_measure_observations(
    xml: &str,
) -> Vec<BeatLocalVoiceRestMeasureObservation> {
    let expected_by_scope = render_scope_metric_expected_duration_map(xml);
    let mut observations = Vec::new();
    let mut scan = 0_usize;

    while let Some((measure_start, measure_end, scope_number)) =
        find_next_measure_bounds(xml, scan)
    {
        let measure = &xml[measure_start..measure_end];
        let expected = expected_by_scope
            .get(&scope_number)
            .copied()
            .unwrap_or(384)
            .max(1);
        let beat = if expected % 4 == 0 {
            (expected / 4).max(1)
        } else {
            96
        };
        let fragments =
            collect_render_timeline_fragments_in_segment(measure, measure_start, scope_number);

        let mut staffs_by_voice: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for frag in fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && !frag.is_chord && !frag.is_hidden)
        {
            let staff = frag.staff.trim();
            if staff == "1" || staff == "2" {
                staffs_by_voice
                    .entry(frag.voice.clone())
                    .or_default()
                    .insert(frag.staff.clone());
            }
        }
        let has_cross_staff_visible_voice = staffs_by_voice
            .values()
            .any(|staffs| staffs.contains("1") && staffs.contains("2"));

        let mut cursor = 0_i32;
        let mut local_scan = 0_usize;
        let mut backups = Vec::new();
        loop {
            let next_note = measure[local_scan..]
                .find("<note")
                .map(|position| local_scan + position);
            let next_backup = measure[local_scan..]
                .find("<backup")
                .map(|position| local_scan + position);
            let next_forward = measure[local_scan..]
                .find("<forward")
                .map(|position| local_scan + position);
            let next = [
                next_note.map(|position| (position, "note")),
                next_backup.map(|position| (position, "backup")),
                next_forward.map(|position| (position, "forward")),
            ]
            .into_iter()
            .flatten()
            .min_by_key(|(position, _)| *position);

            let Some((position, kind)) = next else {
                break;
            };
            match kind {
                "note" => {
                    let Some(close_relative) = measure[position..].find("</note>") else {
                        break;
                    };
                    let end = position + close_relative + "</note>".len();
                    let note = &measure[position..end];
                    let duration = extract_first_text_from_fragment(note, "duration")
                        .and_then(|value| value.trim().parse::<i32>().ok())
                        .unwrap_or(0);
                    let is_chord = note.contains("<chord") || note.contains("<chord/>");
                    if !is_chord {
                        cursor = cursor.saturating_add(duration);
                    }
                    local_scan = end;
                }
                "backup" => {
                    let Some(close_relative) = measure[position..].find("</backup>") else {
                        break;
                    };
                    let end = position + close_relative + "</backup>".len();
                    let backup = &measure[position..end];
                    let duration = extract_first_text_from_fragment(backup, "duration")
                        .and_then(|value| value.trim().parse::<i32>().ok())
                        .unwrap_or(0);
                    let cursor_before = cursor.max(0);
                    let lookahead_end = measure.len().min(end.saturating_add(1800));
                    let lookahead = &measure[end..lookahead_end];
                    backups.push(BeatLocalRestBackupObservation {
                        start: measure_start + position,
                        end: measure_start + end,
                        duration,
                        cursor_before,
                        rest_lane_after: lookahead.contains("<rest")
                            && !lookahead.contains("<pitch>"),
                        fragment: backup.to_string(),
                    });
                    cursor = cursor.saturating_sub(duration).max(0);
                    local_scan = end;
                }
                "forward" => {
                    let Some(close_relative) = measure[position..].find("</forward>") else {
                        break;
                    };
                    let end = position + close_relative + "</forward>".len();
                    let forward = &measure[position..end];
                    let duration = extract_first_text_from_fragment(forward, "duration")
                        .and_then(|value| value.trim().parse::<i32>().ok())
                        .unwrap_or(0);
                    cursor = cursor.saturating_add(duration);
                    local_scan = end;
                }
                _ => break,
            }
        }

        observations.push(BeatLocalVoiceRestMeasureObservation {
            measure_end,
            expected,
            beat,
            final_cursor: cursor.max(0),
            has_cross_staff_visible_voice,
            fragments,
            backups,
        });
        scan = measure_end;
    }

    observations
}
