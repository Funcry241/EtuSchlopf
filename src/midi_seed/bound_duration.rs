use super::{
    DIVISIONS, MetricTimeline, MidiSeedNote, NotationEventKind, NotationTimePlan,
    PlannedSeedOccurrence, SeedPrimaryCourse, midi_to_pitch, type_and_dots_for_duration,
};
use crate::midi::source_time::WRITTEN_NOTE_VALUES;
use anyhow::{Context, Result, anyhow, ensure};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

pub(super) fn plan(
    sources: &[MidiSeedNote],
    metric: &MetricTimeline,
    courses: &[SeedPrimaryCourse],
    time: &NotationTimePlan,
) -> Result<Vec<PlannedSeedOccurrence>> {
    let mut graces = BTreeMap::<_, Vec<_>>::new();
    let mut attacks = BTreeMap::<_, Vec<_>>::new();
    for note in sources {
        let event = time.event(note.source_ordinal)?;
        ensure!(
            (event.pos, event.len) == (note.pos, note.len),
            "Bound note {}: seed time differs from the notation plan",
            note.source_ordinal
        );
        match event.kind {
            NotationEventKind::Regular => attacks
                .entry((note.staff, note.pos))
                .or_default()
                .push(note),
            NotationEventKind::Grace { host } => graces.entry(host).or_default().push(note),
            NotationEventKind::PlaybackOnly => {
                return Err(anyhow!(
                    "Playback-only source {} entered the written plan",
                    note.source_ordinal
                ));
            }
        }
    }

    let mut by_measure = BTreeMap::<_, Vec<_>>::new();
    let mut lane_ends = [Vec::<i64>::new(), Vec::new()];
    for ((staff, start), mut notes) in attacks {
        // Keep the outer note as the primary head where intervals permit it.
        // Prefer a grace host within a chord so its approach stays on that lane.
        notes.sort_by_key(|note| {
            (
                !graces.contains_key(&note.source_ordinal),
                if staff == 1 { -note.midi } else { note.midi },
                note.source_ordinal,
            )
        });
        let course = courses
            .iter()
            .find(|course| course.staff == staff)
            .context("Missing primary course for a bound interval")?;
        let lanes = &mut lane_ends[usize::from(staff - 1)];
        let mut chords = Vec::<(Vec<&MidiSeedNote>, BTreeSet<i32>)>::new();
        for note in notes {
            let chord_target = chords.iter_mut().find(|(members, pitches)| {
                members[0].len == note.len
                    && !pitches.contains(&note.midi)
                    && !graces.contains_key(&note.source_ordinal)
            });
            if let Some((members, pitches)) = chord_target {
                members.push(note);
                pitches.insert(note.midi);
            } else {
                chords.push((vec![note], BTreeSet::from([note.midi])));
            }
        }

        for (members, _) in chords {
            let end = start
                .checked_add(members[0].len)
                .context("Bound release overflow")?;
            let lane = lanes
                .iter()
                .position(|release| *release <= start)
                .unwrap_or(lanes.len());
            if lane == lanes.len() {
                lanes.push(end);
            } else {
                lanes[lane] = end;
            }
            let voice = (lane != 0).then_some(2 * lane + usize::from(staff));
            let head = members[0];
            if let Some(approach) = graces.remove(&head.source_ordinal) {
                let bar = metric
                    .ordinal_at(start)
                    .context("Grace outside bound meter")?;
                for grace in approach {
                    ensure!(
                        grace.staff == staff && grace.pos == start,
                        "Grace {} lost its bound host",
                        grace.source_ordinal
                    );
                    by_measure
                        .entry((bar, staff, lane))
                        .or_default()
                        .push(PlannedSeedOccurrence {
                            course: course.creation,
                            interval: start..start,
                            source: Some(grace.source_ordinal),
                            pitch: Some(grace.midi),
                            chord: false,
                            independent: lane != 0,
                            opposite_stem: lane != 0,
                            source_voice: voice,
                            tie_stop: false,
                            tie_start: false,
                        });
                }
            }
            let mut cursor = start;
            while cursor < end {
                let (bar, fragment) = next_fragment(metric, cursor, end).with_context(|| {
                    format!(
                        "Bound source {} requires {start}..{end}",
                        head.source_ordinal
                    )
                })?;
                for (member, note) in members.iter().enumerate() {
                    by_measure
                        .entry((bar, staff, lane))
                        .or_default()
                        .push(PlannedSeedOccurrence {
                            course: course.creation,
                            interval: fragment.clone(),
                            source: Some(note.source_ordinal),
                            pitch: Some(note.midi),
                            chord: member != 0,
                            independent: lane != 0,
                            opposite_stem: lane != 0,
                            source_voice: voice,
                            tie_stop: cursor != start,
                            tie_start: fragment.end != end,
                        });
                }
                cursor = fragment.end;
            }
        }
    }
    ensure!(
        graces.is_empty(),
        "Bound grace host did not enter the written plan"
    );

    let mut planned = Vec::new();
    for bar_index in 0..metric.len() {
        let bar = metric
            .measure(bar_index)
            .context("Missing bound notation measure")?;
        for course in courses {
            let lanes = lane_ends[usize::from(course.staff - 1)].len().max(1);
            for lane in 0..lanes {
                let notes = by_measure.remove(&(bar_index, course.staff, lane));
                if lane != 0 && notes.is_none() {
                    continue;
                }
                let mut cursor = bar.start;
                let voice = (lane != 0).then_some(2 * lane + usize::from(course.staff));
                for note in notes.into_iter().flatten() {
                    if !note.chord {
                        append_silence(
                            &mut planned,
                            course,
                            metric,
                            cursor..note.interval.start,
                            voice,
                        )?;
                        cursor = note.interval.end;
                    }
                    planned.push(note);
                }
                append_silence(&mut planned, course, metric, cursor..bar.end, voice)?;
            }
        }
    }
    ensure!(
        by_measure.is_empty(),
        "Bound fragments escaped the seed measures"
    );
    validate_partition(sources, &planned)?;
    Ok(planned)
}

fn next_fragment(metric: &MetricTimeline, cursor: i64, end: i64) -> Result<(usize, Range<i64>)> {
    let ordinal = metric
        .ordinal_at(cursor)
        .context("Fragment starts outside bound meter")?;
    let bar = metric
        .measure(ordinal)
        .context("Missing bound fragment measure")?;
    let remaining = end.min(bar.end) - cursor;
    let beat = DIVISIONS * 4 / i64::from(bar.beat_type);
    let capacity = if WRITTEN_NOTE_VALUES.contains(&remaining) {
        remaining
    } else {
        remaining.min(beat - (cursor - bar.start) % beat)
    };
    let length = WRITTEN_NOTE_VALUES
        .into_iter()
        .find(|length| *length <= capacity)
        .ok_or_else(|| anyhow!("No exact seed value at {cursor} before {end}"))?;
    Ok((ordinal, cursor..cursor + length))
}

fn append_silence(
    planned: &mut Vec<PlannedSeedOccurrence>,
    course: &SeedPrimaryCourse,
    metric: &MetricTimeline,
    gap: Range<i64>,
    voice: Option<usize>,
) -> Result<()> {
    ensure!(
        gap.start <= gap.end,
        "Overlapping bound seed fragments in a cursor lane"
    );
    let mut cursor = gap.start;
    while cursor < gap.end {
        let (_, interval) = next_fragment(metric, cursor, gap.end)?;
        cursor = interval.end;
        planned.push(PlannedSeedOccurrence {
            course: course.creation,
            interval,
            source: None,
            pitch: None,
            chord: false,
            independent: voice.is_some(),
            opposite_stem: false,
            source_voice: voice,
            tie_stop: false,
            tie_start: false,
        });
    }
    Ok(())
}

pub(super) fn validate_partition(
    sources: &[MidiSeedNote],
    planned: &[PlannedSeedOccurrence],
) -> Result<()> {
    let mut fragments = BTreeMap::<_, Vec<_>>::new();
    for fragment in planned {
        if let Some(source) = fragment.source {
            fragments.entry(source).or_default().push(fragment);
        }
    }
    for source in sources {
        let parts = fragments.remove(&source.source_ordinal).unwrap_or_default();
        let end = source.pos + source.len;
        let mut cursor = source.pos;
        let valid = !parts.is_empty()
            && (source.len != 0 || parts.len() == 1)
            && parts.iter().all(|part| {
                let contiguous = part.interval.start == cursor
                    && part.interval.end <= end
                    && (part.interval.start < part.interval.end || source.len == 0)
                    && part.pitch == Some(source.midi)
                    && part.source_voice == parts[0].source_voice
                    && part.tie_stop == (cursor != source.pos)
                    && part.tie_start == (part.interval.end != end);
                cursor = part.interval.end;
                contiguous
            })
            && cursor == end;
        ensure!(
            valid,
            "Bound source {} requires {}..{}; seed fragments {:?}",
            source.source_ordinal,
            source.pos,
            end,
            parts.iter().map(|part| &part.interval).collect::<Vec<_>>()
        );
    }
    ensure!(
        fragments.is_empty(),
        "Foreign source in bound seed fragments"
    );
    Ok(())
}

pub(super) fn verify_written_intervals(
    xml: &str,
    metric: &MetricTimeline,
    planned: &[PlannedSeedOccurrence],
    bound_sections: &BTreeSet<(usize, u8)>,
) -> Result<()> {
    let document = roxmltree::Document::parse(xml)?;
    let mut fragments = planned.iter();
    for (ordinal, measure) in document
        .descendants()
        .filter(|node| node.has_tag_name("measure"))
        .enumerate()
    {
        let bar = metric
            .measure(ordinal)
            .context("Unexpected emitted bound measure")?;
        let mut cursor = bar.start;
        let mut head = cursor;
        for node in measure.children().filter(|child| child.is_element()) {
            let name = node.tag_name().name();
            if !matches!(name, "forward" | "backup" | "note") {
                continue;
            }
            let child_text = |tag| {
                node.children()
                    .find(|child| child.has_tag_name(tag))
                    .and_then(|child| child.text())
            };
            let duration = child_text("duration")
                .map(str::parse::<i64>)
                .transpose()?
                .unwrap_or(0);
            if name != "note" {
                cursor += if name == "forward" {
                    duration
                } else {
                    -duration
                };
                ensure!(
                    (bar.start..=bar.end).contains(&cursor),
                    "Bound seed cursor escaped its measure"
                );
                continue;
            }
            let fragment = fragments
                .next()
                .context("Extra emitted bound seed fragment")?;
            let chord = node.children().any(|child| child.has_tag_name("chord"));
            if !chord {
                head = cursor;
                cursor += duration;
            }
            let staff = child_text("staff").and_then(|value| value.parse::<u8>().ok())
                .context("Missing emitted seed staff")?;
            if !bound_sections.contains(&(ordinal, staff)) {
                continue;
            }
            let tied = |kind| {
                node.children()
                    .any(|child| child.has_tag_name("tie") && child.attribute("type") == Some(kind))
            };
            let pitch_matches = if let Some(midi) = fragment.pitch {
                let (step, alter, octave) = midi_to_pitch(midi);
                let pitch = node.children().find(|child| child.has_tag_name("pitch"));
                let field = |tag| {
                    pitch
                        .and_then(|pitch| pitch.children().find(|child| child.has_tag_name(tag)))
                        .and_then(|child| child.text())
                };
                field("step") == Some(step)
                    && field("alter").unwrap_or("0").parse() == Ok(alter)
                    && field("octave").and_then(|value| value.parse().ok()) == Some(octave)
            } else {
                node.children().any(|child| child.has_tag_name("rest"))
            };
            let (note_type, dots) = type_and_dots_for_duration(duration);
            ensure!(
                (head..head + duration) == fragment.interval
                    && chord == fragment.chord
                    && tied("start") == fragment.tie_start
                    && tied("stop") == fragment.tie_stop
                    && pitch_matches
                    && (duration == 0
                        || (WRITTEN_NOTE_VALUES.contains(&duration)
                            && child_text("type") == Some(note_type)
                            && node
                                .children()
                                .filter(|child| child.has_tag_name("dot"))
                                .count()
                                == dots)),
                "Bound source {:?}: expected fragment {:?}, emitted {}..{}",
                fragment.source,
                fragment.interval,
                head,
                head + duration
            );
        }
    }
    ensure!(
        fragments.next().is_none(),
        "Missing emitted bound seed fragments"
    );
    Ok(())
}
