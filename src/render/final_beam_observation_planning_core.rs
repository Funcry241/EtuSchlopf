// v0.10.1780 FinalBeamLostBeginRecoveryAuthorityRetirement:
// Writer-free typed ownership for final-PDF beam source observation and the
// committed-state lost-begin / retired-HigherBeam regression detectors.
// Productive lost-begin mutation is retired; any reappearance must fail closed
// at the committed artifact boundary instead of being repaired.

#[derive(Debug, Clone, Default)]
struct FinalBeamTimeline {
    expected: i64,
    covered: i64,
    underfill: i64,
    overflow: i64,
}

#[derive(Debug, Clone)]
struct FinalBeamEvent {
    part: String,
    bar: String,
    voice: String,
    staff: String,
    onset: i64,
    duration: i64,
    beat_unit: i64,
    pitch: String,
    rest: bool,
    chord: bool,
    hidden: bool,
    sequence: usize,
    beams: std::collections::BTreeMap<u32, String>,
    range: std::ops::Range<usize>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct FinalBeamSource {
    events: Vec<FinalBeamEvent>,
    timelines: std::collections::BTreeMap<(String, String, String, String), FinalBeamTimeline>,
}

pub(super) fn final_beam_child_text(node: roxmltree::Node<'_, '_>, tag: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == tag)
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub(super) fn final_beam_child_i64(node: roxmltree::Node<'_, '_>, tag: &str) -> i64 {
    final_beam_child_text(node, tag)
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
}

pub(super) fn final_beam_has_child(node: roxmltree::Node<'_, '_>, tag: &str) -> bool {
    node.children()
        .any(|child| child.is_element() && child.tag_name().name() == tag)
}

fn final_beam_pitch_token(note: roxmltree::Node<'_, '_>) -> String {
    if final_beam_has_child(note, "rest") {
        return "rest".to_string();
    }
    let Some(pitch) = note
        .children()
        .find(|child| child.is_element() && child.tag_name().name() == "pitch")
    else {
        return "?".to_string();
    };
    let step = final_beam_child_text(pitch, "step").unwrap_or_else(|| "?".to_string());
    let alter = final_beam_child_text(pitch, "alter").unwrap_or_else(|| "0".to_string());
    let octave = final_beam_child_text(pitch, "octave").unwrap_or_else(|| "?".to_string());
    format!("{step}:{alter}:{octave}")
}

fn final_beam_states(note: roxmltree::Node<'_, '_>) -> std::collections::BTreeMap<u32, String> {
    let mut states = std::collections::BTreeMap::new();
    for beam in note
        .children()
        .filter(|child| child.is_element() && child.tag_name().name() == "beam")
    {
        let level = beam
            .attribute("number")
            .unwrap_or("1")
            .parse::<u32>()
            .unwrap_or(1);
        let state = beam.text().unwrap_or("").trim().to_ascii_lowercase();
        if !state.is_empty() {
            states.insert(level, state);
        }
    }
    states
}

fn final_beam_union_duration(intervals: &[(i64, i64)], expected: i64) -> i64 {
    let mut clipped = intervals
        .iter()
        .filter_map(|(start, end)| {
            let start = (*start).max(0).min(expected);
            let end = (*end).max(0).min(expected);
            (end > start).then_some((start, end))
        })
        .collect::<Vec<_>>();
    clipped.sort_unstable();
    let mut covered = 0_i64;
    let mut current: Option<(i64, i64)> = None;
    for (start, end) in clipped {
        match current {
            None => current = Some((start, end)),
            Some((current_start, current_end)) if start <= current_end => {
                current = Some((current_start, current_end.max(end)));
            }
            Some((current_start, current_end)) => {
                covered += current_end - current_start;
                current = Some((start, end));
            }
        }
    }
    if let Some((start, end)) = current {
        covered += end - start;
    }
    covered
}

pub(super) fn parse_final_beam_source(xml: &str) -> Option<FinalBeamSource> {
    let document = roxmltree::Document::parse(xml).ok()?;
    let root = document.root_element();
    let mut source = FinalBeamSource::default();
    let mut sequence = 0_usize;
    for (part_index, part) in root
        .children()
        .filter(|node| node.is_element() && node.tag_name().name() == "part")
        .enumerate()
    {
        let part_id = part
            .attribute("id")
            .map_or_else(|| format!("P{}", part_index + 1), str::to_string);
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        for measure in part
            .children()
            .filter(|node| node.is_element() && node.tag_name().name() == "measure")
        {
            let bar = measure.attribute("number").unwrap_or("?").to_string();
            if let Some(attributes) = measure
                .children()
                .find(|node| node.is_element() && node.tag_name().name() == "attributes")
            {
                let next_divisions = final_beam_child_i64(attributes, "divisions");
                if next_divisions > 0 {
                    divisions = next_divisions;
                }
                if let Some(time) = attributes
                    .children()
                    .find(|node| node.is_element() && node.tag_name().name() == "time")
                {
                    let next_beats = final_beam_child_i64(time, "beats");
                    let next_beat_type = final_beam_child_i64(time, "beat-type");
                    if next_beats > 0 {
                        beats = next_beats;
                    }
                    if next_beat_type > 0 {
                        beat_type = next_beat_type;
                    }
                }
            }
            let expected = if beat_type > 0 {
                divisions
                    .saturating_mul(beats)
                    .saturating_mul(4)
                    .checked_div(beat_type)
                    .unwrap_or(0)
            } else {
                0
            };
            let mut cursor = 0_i64;
            let mut last_non_chord_onset: Option<i64> = None;
            let mut current_voice: Option<String> = None;
            let mut current_staff: Option<String> = None;
            let mut intervals =
                std::collections::BTreeMap::<(String, String), Vec<(i64, i64)>>::new();
            let mut max_end = std::collections::BTreeMap::<(String, String), i64>::new();
            for child in measure.children().filter(|node| node.is_element()) {
                match child.tag_name().name() {
                    "backup" => {
                        cursor = cursor
                            .saturating_sub(final_beam_child_i64(child, "duration").max(0))
                            .max(0);
                        last_non_chord_onset = None;
                        current_voice = None;
                        current_staff = None;
                    }
                    "forward" => {
                        let duration = final_beam_child_i64(child, "duration").max(0);
                        let voice = final_beam_child_text(child, "voice")
                            .or_else(|| current_voice.clone());
                        let staff = final_beam_child_text(child, "staff")
                            .or_else(|| current_staff.clone());
                        if let (Some(voice), Some(staff)) = (voice.as_ref(), staff.as_ref()) {
                            let lane = (voice.clone(), staff.clone());
                            intervals
                                .entry(lane.clone())
                                .or_default()
                                .push((cursor, cursor.saturating_add(duration)));
                            max_end
                                .entry(lane)
                                .and_modify(|value| {
                                    *value = (*value).max(cursor.saturating_add(duration));
                                })
                                .or_insert(cursor.saturating_add(duration));
                        }
                        cursor = cursor.saturating_add(duration);
                        last_non_chord_onset = None;
                        current_voice = voice;
                        current_staff = staff;
                    }
                    "note" => {
                        let duration = final_beam_child_i64(child, "duration").max(0);
                        let chord = final_beam_has_child(child, "chord");
                        let onset = if chord {
                            last_non_chord_onset.unwrap_or(cursor)
                        } else {
                            cursor
                        };
                        let voice = final_beam_child_text(child, "voice")
                            .or_else(|| current_voice.clone())
                            .unwrap_or_else(|| "1".to_string());
                        let staff = final_beam_child_text(child, "staff")
                            .or_else(|| current_staff.clone())
                            .unwrap_or_else(|| "1".to_string());
                        source.events.push(FinalBeamEvent {
                            part: part_id.clone(),
                            bar: bar.clone(),
                            voice: voice.clone(),
                            staff: staff.clone(),
                            onset,
                            duration,
                            beat_unit: if beat_type == 4 { divisions } else { 0 },
                            pitch: final_beam_pitch_token(child),
                            rest: final_beam_has_child(child, "rest"),
                            chord,
                            hidden: child.attribute("print-object") == Some("no"),
                            sequence,
                            beams: final_beam_states(child),
                            range: child.range(),
                        });
                        sequence += 1;
                        if !chord {
                            let lane = (voice.clone(), staff.clone());
                            intervals
                                .entry(lane.clone())
                                .or_default()
                                .push((onset, onset.saturating_add(duration)));
                            max_end
                                .entry(lane)
                                .and_modify(|value| {
                                    *value = (*value).max(onset.saturating_add(duration));
                                })
                                .or_insert(onset.saturating_add(duration));
                            last_non_chord_onset = Some(onset);
                            if !final_beam_has_child(child, "grace") {
                                cursor = cursor.saturating_add(duration);
                            }
                        }
                        current_voice = Some(voice);
                        current_staff = Some(staff);
                    }
                    _ => {}
                }
            }
            for ((voice, staff), lane_intervals) in intervals {
                let covered = final_beam_union_duration(&lane_intervals, expected);
                let lane_max_end = max_end
                    .get(&(voice.clone(), staff.clone()))
                    .copied()
                    .unwrap_or(0);
                source.timelines.insert(
                    (part_id.clone(), bar.clone(), staff, voice),
                    FinalBeamTimeline {
                        expected,
                        covered,
                        underfill: expected.saturating_sub(covered).max(0),
                        overflow: lane_max_end.saturating_sub(expected).max(0),
                    },
                );
            }
        }
    }
    source.events.sort_by(|left, right| {
        left.part
            .cmp(&right.part)
            .then_with(|| left.bar.cmp(&right.bar))
            .then_with(|| left.voice.cmp(&right.voice))
            .then_with(|| left.onset.cmp(&right.onset))
            .then_with(|| left.sequence.cmp(&right.sequence))
    });
    Some(source)
}

fn final_beam_event_key(event: &FinalBeamEvent) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        event.part,
        event.bar,
        event.voice,
        event.onset,
        event.duration,
        event.pitch,
        event.rest
    )
}

fn final_pdf_source_higher_beam_regression_key(event: &FinalBeamEvent) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        event.part,
        event.bar,
        event.staff,
        event.voice,
        event.onset,
        event.duration,
        event.pitch,
        event.rest
    )
}

fn final_pdf_source_higher_beam_regression_group_key(event: &FinalBeamEvent) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        event.part,
        event.bar,
        event.staff,
        event.voice,
        event.onset.div_euclid(event.beat_unit.max(1))
    )
}

pub(super) fn final_pdf_source_higher_beam_regression_candidates(
    master_xml: &str,
    candidate_xml: &str,
) -> Option<usize> {
    let master = parse_final_beam_source(master_xml)?;
    let candidate = parse_final_beam_source(candidate_xml)?;
    let mut master_rows: std::collections::BTreeMap<String, Vec<&FinalBeamEvent>> =
        std::collections::BTreeMap::new();
    let mut candidate_rows: std::collections::BTreeMap<String, Vec<&FinalBeamEvent>> =
        std::collections::BTreeMap::new();
    let mut candidate_groups: std::collections::BTreeMap<String, Vec<&FinalBeamEvent>> =
        std::collections::BTreeMap::new();
    for event in &master.events {
        master_rows
            .entry(final_pdf_source_higher_beam_regression_key(event))
            .or_default()
            .push(event);
    }
    for event in &candidate.events {
        candidate_rows
            .entry(final_pdf_source_higher_beam_regression_key(event))
            .or_default()
            .push(event);
        if !event.hidden
            && !event.rest
            && !event.chord
            && event.duration > 0
            && event.beat_unit > 0
        {
            candidate_groups
                .entry(final_pdf_source_higher_beam_regression_group_key(event))
                .or_default()
                .push(event);
        }
    }
    let primary = ["begin", "continue", "continue", "end"];
    let secondary_master = ["begin", "continue", "continue", "end"];
    let secondary_missing_begin = [None, Some("continue"), Some("continue"), Some("end")];
    let expected_primary = primary.map(Some);
    let expected_secondary_master = secondary_master.map(Some);
    let mut candidates = 0_usize;
    for mut group in candidate_groups.into_values() {
        group.sort_by_key(|event| (event.onset, event.sequence));
        let Some(first) = group.first().copied() else {
            continue;
        };
        let beat_start = first.onset.div_euclid(first.beat_unit) * first.beat_unit;
        let beat_end = beat_start.saturating_add(first.beat_unit);
        let complete_four_sixteenths = group.len() == 4
            && group
                .iter()
                .all(|event| event.duration.saturating_mul(4) == event.beat_unit)
            && group
                .first()
                .is_some_and(|event| event.onset == beat_start)
            && group.last().is_some_and(|event| {
                event.onset.saturating_add(event.duration) == beat_end
            })
            && group.windows(2).all(|pair| {
                pair[0].onset.saturating_add(pair[0].duration) == pair[1].onset
            });
        if !complete_four_sixteenths {
            continue;
        }
        let mut master_group = Vec::with_capacity(group.len());
        let mut unique = true;
        for candidate_event in &group {
            let key = final_pdf_source_higher_beam_regression_key(candidate_event);
            let Some(candidate_matches) = candidate_rows.get(&key) else {
                unique = false;
                break;
            };
            let Some(master_matches) = master_rows.get(&key) else {
                unique = false;
                break;
            };
            if candidate_matches.len() != 1 || master_matches.len() != 1 {
                unique = false;
                break;
            }
            master_group.push(master_matches[0]);
        }
        if !unique {
            continue;
        }
        let candidate_primary = group
            .iter()
            .map(|event| event.beams.get(&1).map(String::as_str))
            .collect::<Vec<_>>();
        let master_primary = master_group
            .iter()
            .map(|event| event.beams.get(&1).map(String::as_str))
            .collect::<Vec<_>>();
        let candidate_secondary = group
            .iter()
            .map(|event| event.beams.get(&2).map(String::as_str))
            .collect::<Vec<_>>();
        let master_secondary = master_group
            .iter()
            .map(|event| event.beams.get(&2).map(String::as_str))
            .collect::<Vec<_>>();
        if candidate_primary.as_slice() == expected_primary.as_slice()
            && master_primary.as_slice() == expected_primary.as_slice()
            && candidate_secondary.as_slice() == secondary_missing_begin.as_slice()
            && master_secondary.as_slice() == expected_secondary_master.as_slice()
        {
            candidates = candidates.saturating_add(1);
        }
    }
    Some(candidates)
}

fn final_beam_exact_visible_identity(
    master: &FinalBeamSource,
    render: &FinalBeamSource,
    master_event: &FinalBeamEvent,
    render_event: &FinalBeamEvent,
) -> bool {
    if master_event.hidden
        || render_event.hidden
        || final_beam_event_key(master_event) != final_beam_event_key(render_event)
    {
        return false;
    }
    let key = final_beam_event_key(master_event);
    master
        .events
        .iter()
        .filter(|event| !event.hidden && final_beam_event_key(event) == key)
        .count()
        == 1
        && render
            .events
            .iter()
            .filter(|event| !event.hidden && final_beam_event_key(event) == key)
            .count()
            == 1
}

fn final_beam_level_open_before(
    source: &FinalBeamSource,
    target: &FinalBeamEvent,
    level: u32,
) -> bool {
    let mut rows = source
        .events
        .iter()
        .filter(|event| {
            event.part == target.part
                && event.bar == target.bar
                && event.voice == target.voice
                && event.staff == target.staff
                && event.onset < target.onset
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.onset
            .cmp(&right.onset)
            .then_with(|| left.sequence.cmp(&right.sequence))
    });
    let mut open = false;
    let mut index = 0_usize;
    while index < rows.len() {
        let onset = rows[index].onset;
        let mut states = std::collections::BTreeSet::new();
        while index < rows.len() && rows[index].onset == onset {
            if let Some(state) = rows[index].beams.get(&level) {
                states.insert(state.as_str());
            }
            index += 1;
        }
        if states.len() > 1 {
            return true;
        }
        match states.iter().next().copied() {
            Some("begin") => open = true,
            Some("end") => open = false,
            _ => {}
        }
    }
    open
}

fn final_beam_all_levels_begin(
    observed_beams: &std::collections::BTreeMap<u32, String>,
) -> bool {
    !observed_beams.is_empty()
        && observed_beams.iter().all(|(level, state)| {
            *level > 0 && u8::try_from(*level).is_ok() && state.as_str() == "begin"
        })
}

fn count_final_beam_lost_begin_candidates(
    master: &FinalBeamSource,
    render: &FinalBeamSource,
) -> usize {
    let mut ranges = std::collections::BTreeSet::new();
    for master_event in &master.events {
        if master_event.hidden
            || master_event.rest
            || !final_beam_all_levels_begin(&master_event.beams)
        {
            continue;
        }
        let key = final_beam_event_key(master_event);
        let render_matches = render
            .events
            .iter()
            .filter(|event| !event.hidden && final_beam_event_key(event) == key)
            .collect::<Vec<_>>();
        if render_matches.len() != 1 {
            continue;
        }
        let render_event = render_matches[0];
        let timeline = render.timelines.get(&(
            render_event.part.clone(),
            render_event.bar.clone(),
            render_event.staff.clone(),
            render_event.voice.clone(),
        ));
        if render_event.rest
            || render_event.staff != master_event.staff
            || !render_event.beams.is_empty()
            || timeline.is_none_or(|lane| {
                lane.expected <= 0
                    || lane.covered != lane.expected
                    || lane.underfill > 0
                    || lane.overflow > 0
            })
            || !final_beam_exact_visible_identity(master, render, master_event, render_event)
            || master_event
                .beams
                .keys()
                .any(|level| final_beam_level_open_before(render, render_event, *level))
        {
            continue;
        }

        let mut following_master = master
            .events
            .iter()
            .filter(|event| {
                event.part == master_event.part
                    && event.bar == master_event.bar
                    && event.voice == master_event.voice
                    && event.staff == master_event.staff
                    && event.onset > master_event.onset
                    && !event.beams.is_empty()
            })
            .collect::<Vec<_>>();
        following_master.sort_by(|left, right| {
            left.onset
                .cmp(&right.onset)
                .then_with(|| left.sequence.cmp(&right.sequence))
        });

        let mut open_levels = master_event
            .beams
            .keys()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let mut expected_onset = master_event.onset.saturating_add(master_event.duration);
        let mut group_end_onset = None;
        for follower in following_master {
            if follower.hidden
                || follower.rest
                || follower.onset != expected_onset
                || follower
                    .beams
                    .values()
                    .any(|state| state.as_str() == "begin")
                || open_levels.iter().any(|level| {
                    !matches!(
                        follower.beams.get(level).map(String::as_str),
                        Some("continue" | "end")
                    )
                })
            {
                break;
            }
            let follower_key = final_beam_event_key(follower);
            let matching_render = render
                .events
                .iter()
                .filter(|event| !event.hidden && final_beam_event_key(event) == follower_key)
                .collect::<Vec<_>>();
            if matching_render.len() != 1
                || matching_render[0].staff != follower.staff
                || matching_render[0].beams != follower.beams
                || !final_beam_exact_visible_identity(
                    master,
                    render,
                    follower,
                    matching_render[0],
                )
            {
                break;
            }
            expected_onset = follower.onset.saturating_add(follower.duration);
            open_levels.retain(|level| {
                follower.beams.get(level).map(String::as_str) != Some("end")
            });
            if open_levels.is_empty() {
                group_end_onset = Some(follower.onset);
                break;
            }
        }
        let Some(group_end_onset) = group_end_onset else {
            continue;
        };
        if master.events.iter().any(|event| {
            event.part == master_event.part
                && event.bar == master_event.bar
                && event.voice == master_event.voice
                && event.staff == master_event.staff
                && event.onset >= master_event.onset
                && event.onset <= group_end_onset
                && event.rest
                && !event.beams.is_empty()
        }) || render.events.iter().any(|event| {
            event.part == render_event.part
                && event.bar == render_event.bar
                && event.voice == render_event.voice
                && event.staff == render_event.staff
                && event.onset >= render_event.onset
                && event.onset <= group_end_onset
                && event.rest
                && !event.beams.is_empty()
        }) {
            continue;
        }
        ranges.insert((render_event.range.start, render_event.range.end));
    }
    ranges.len()
}

pub(super) fn final_pdf_source_lost_begin_regression_candidates(
    master_xml: &str,
    candidate_xml: &str,
) -> Option<usize> {
    let master = parse_final_beam_source(master_xml)?;
    let candidate = parse_final_beam_source(candidate_xml)?;
    Some(count_final_beam_lost_begin_candidates(&master, &candidate))
}
