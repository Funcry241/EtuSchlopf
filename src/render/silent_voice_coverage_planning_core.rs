// v0.10.1706 SilentVoiceCoveragePlanningCoreExtraction:
// Pure candidate planning for silent-voice visible measure coverage and
// structural cross-staff technical-rest suppression. The core owns no
// Structural Write Authority and performs no filesystem write.

use super::super::{
    collect_render_timeline_fragments_in_segment, find_next_measure_bounds,
    infer_measure_duration_from_measure_xml, parse_musicxml_current_meter,
    render_scope_metric_expected_duration_map, RenderXmlTimelineFragmentSpan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SilentCoverageCarrierKind {
    VisibleRest,
    HiddenRest,
    Pitched,
    Forward,
}

#[derive(Debug, Clone)]
struct SilentCoverageCarrier {
    start: usize,
    end: usize,
    onset: i64,
    duration: i64,
    kind: SilentCoverageCarrierKind,
}

#[derive(Debug, Clone, Default)]
struct SilentCoverageLane {
    voice: String,
    staffs: std::collections::BTreeSet<String>,
    carriers: Vec<SilentCoverageCarrier>,
}

#[derive(Debug, Clone, Default)]
struct SilentCoverageReplacement {
    start: usize,
    end: usize,
    xml: String,
}

fn silent_coverage_direct_child_text(fragment: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = fragment.find(&open)? + open.len();
    let end = fragment[start..].find(&close)? + start;
    Some(fragment[start..end].trim().to_string())
}

fn silent_coverage_direct_child_i64(fragment: &str, tag: &str) -> i64 {
    silent_coverage_direct_child_text(fragment, tag)
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
        .max(0)
}

fn silent_coverage_union_duration(
    carriers: &[SilentCoverageCarrier],
    expected: i64,
    predicate: impl Fn(&SilentCoverageCarrier) -> bool,
) -> i64 {
    let mut intervals = carriers
        .iter()
        .filter(|carrier| predicate(carrier))
        .filter_map(|carrier| {
            let start = carrier.onset.max(0).min(expected);
            let end = carrier
                .onset
                .saturating_add(carrier.duration)
                .max(0)
                .min(expected);
            (end > start).then_some((start, end))
        })
        .collect::<Vec<_>>();
    intervals.sort_unstable();
    let mut total = 0_i64;
    let mut current: Option<(i64, i64)> = None;
    for (start, end) in intervals {
        match current {
            None => current = Some((start, end)),
            Some((current_start, current_end)) if start <= current_end => {
                current = Some((current_start, current_end.max(end)));
            }
            Some((current_start, current_end)) => {
                total += current_end - current_start;
                current = Some((start, end));
            }
        }
    }
    if let Some((start, end)) = current {
        total += end - start;
    }
    total
}

fn silent_coverage_fragment_union_duration(
    fragments: &[RenderXmlTimelineFragmentSpan],
    start: i32,
    end: i32,
    predicate: impl Fn(&RenderXmlTimelineFragmentSpan) -> bool,
) -> i32 {
    let mut intervals = fragments
        .iter()
        .filter(|fragment| predicate(fragment))
        .filter_map(|fragment| {
            let fragment_start = fragment.onset.max(start);
            let fragment_end = fragment
                .onset
                .saturating_add(fragment.duration.max(0))
                .min(end);
            (fragment_end > fragment_start).then_some((fragment_start, fragment_end))
        })
        .collect::<Vec<_>>();
    intervals.sort_unstable();
    let mut total = 0_i32;
    let mut current: Option<(i32, i32)> = None;
    for (interval_start, interval_end) in intervals {
        match current {
            None => current = Some((interval_start, interval_end)),
            Some((current_start, current_end)) if interval_start <= current_end => {
                current = Some((current_start, current_end.max(interval_end)));
            }
            Some((current_start, current_end)) => {
                total = total.saturating_add(current_end.saturating_sub(current_start));
                current = Some((interval_start, interval_end));
            }
        }
    }
    if let Some((current_start, current_end)) = current {
        total = total.saturating_add(current_end.saturating_sub(current_start));
    }
    total
}

fn silent_coverage_line_indent(xml: &str, position: usize) -> &str {
    let line_start = xml[..position].rfind('\n').map_or(0, |index| index + 1);
    let prefix = &xml[line_start..position];
    let indent_len = prefix
        .bytes()
        .take_while(|byte| matches!(*byte, b' ' | b'\t'))
        .count();
    &prefix[..indent_len]
}

fn silent_coverage_rest_type(duration: i64, divisions: i64) -> Option<(&'static str, usize)> {
    if duration <= 0 || divisions <= 0 {
        return None;
    }
    let bases = [
        ("whole", divisions.saturating_mul(4)),
        ("half", divisions.saturating_mul(2)),
        ("quarter", divisions),
        ("eighth", divisions / 2),
        ("16th", divisions / 4),
        ("32nd", divisions / 8),
        ("64th", divisions / 16),
    ];
    for (name, base) in bases {
        if base <= 0 {
            continue;
        }
        if duration == base {
            return Some((name, 0));
        }
        if base % 2 == 0 && duration == base.saturating_mul(3) / 2 {
            return Some((name, 1));
        }
        if base % 4 == 0 && duration == base.saturating_mul(7) / 4 {
            return Some((name, 2));
        }
    }
    None
}

fn silent_coverage_visible_rest_fragment(
    xml: &str,
    start: usize,
    duration: i64,
    divisions: i64,
    voice: &str,
    staff: &str,
) -> Option<String> {
    let (note_type, dots) = silent_coverage_rest_type(duration, divisions)?;
    let indent = silent_coverage_line_indent(xml, start);
    let child = format!("{indent}  ");
    let mut out = String::new();
    out.push_str(indent);
    out.push_str("<note>\n");
    out.push_str(&child);
    out.push_str("<rest/>\n");
    out.push_str(&child);
    out.push_str(&format!("<duration>{duration}</duration>\n"));
    out.push_str(&child);
    out.push_str(&format!("<voice>{voice}</voice>\n"));
    out.push_str(&child);
    out.push_str(&format!("<type>{note_type}</type>\n"));
    for _ in 0..dots {
        out.push_str(&child);
        out.push_str("<dot/>\n");
    }
    out.push_str(&child);
    out.push_str(&format!("<staff>{staff}</staff>\n"));
    out.push_str(indent);
    out.push_str("</note>");
    Some(out)
}

fn silent_coverage_flush_pending(
    lanes: &mut std::collections::BTreeMap<String, SilentCoverageLane>,
    pending: &mut Vec<SilentCoverageCarrier>,
    voice: &str,
    staff: &str,
) {
    if pending.is_empty() {
        return;
    }
    let lane = lanes
        .entry(voice.to_string())
        .or_insert_with(|| SilentCoverageLane {
            voice: voice.to_string(),
            ..SilentCoverageLane::default()
        });
    lane.staffs.insert(staff.to_string());
    for carrier in pending.drain(..) {
        lane.carriers.push(carrier);
    }
}

fn silent_coverage_measure_replacements(
    xml: &str,
    measure_start: usize,
    measure_end: usize,
    divisions: i64,
    expected: i64,
) -> (Vec<SilentCoverageReplacement>, usize) {
    let measure = &xml[measure_start..measure_end];
    let mut cursor = 0_i64;
    let mut current_voice: Option<String> = None;
    let mut current_staff: Option<String> = None;
    let mut pending = Vec::<SilentCoverageCarrier>::new();
    let mut lanes = std::collections::BTreeMap::<String, SilentCoverageLane>::new();
    let mut scan = 0_usize;

    loop {
        let next_note = measure[scan..].find("<note").map(|position| scan + position);
        let next_backup = measure[scan..]
            .find("<backup")
            .map(|position| scan + position);
        let next_forward = measure[scan..]
            .find("<forward")
            .map(|position| scan + position);
        let Some((position, kind)) = [
            next_note.map(|value| (value, "note")),
            next_backup.map(|value| (value, "backup")),
            next_forward.map(|value| (value, "forward")),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(position, _)| *position)
        else {
            break;
        };
        let close = match kind {
            "note" => "</note>",
            "backup" => "</backup>",
            "forward" => "</forward>",
            _ => break,
        };
        let Some(close_relative) = measure[position..].find(close) else {
            break;
        };
        let end = position + close_relative + close.len();
        let fragment = &measure[position..end];
        let duration = silent_coverage_direct_child_i64(fragment, "duration");

        match kind {
            "backup" => {
                if let (Some(voice), Some(staff)) =
                    (current_voice.as_deref(), current_staff.as_deref())
                {
                    silent_coverage_flush_pending(&mut lanes, &mut pending, voice, staff);
                } else {
                    pending.clear();
                }
                cursor = cursor.saturating_sub(duration).max(0);
                current_voice = None;
                current_staff = None;
            }
            "forward" => {
                let explicit_voice = silent_coverage_direct_child_text(fragment, "voice");
                let explicit_staff = silent_coverage_direct_child_text(fragment, "staff");
                let resolved_voice = explicit_voice.or_else(|| current_voice.clone());
                let resolved_staff = explicit_staff.or_else(|| current_staff.clone());
                let carrier = SilentCoverageCarrier {
                    start: measure_start + position,
                    end: measure_start + end,
                    onset: cursor,
                    duration,
                    kind: SilentCoverageCarrierKind::Forward,
                };
                if let (Some(voice), Some(staff)) = (resolved_voice, resolved_staff) {
                    silent_coverage_flush_pending(&mut lanes, &mut pending, &voice, &staff);
                    let lane = lanes
                        .entry(voice.clone())
                        .or_insert_with(|| SilentCoverageLane {
                            voice: voice.clone(),
                            ..SilentCoverageLane::default()
                        });
                    lane.staffs.insert(staff.clone());
                    lane.carriers.push(carrier);
                    current_voice = Some(voice);
                    current_staff = Some(staff);
                } else {
                    pending.push(carrier);
                }
                cursor = cursor.saturating_add(duration);
            }
            "note" => {
                let voice = silent_coverage_direct_child_text(fragment, "voice")
                    .or_else(|| current_voice.clone())
                    .unwrap_or_else(|| "1".to_string());
                let staff = silent_coverage_direct_child_text(fragment, "staff")
                    .or_else(|| current_staff.clone())
                    .unwrap_or_else(|| "1".to_string());
                silent_coverage_flush_pending(&mut lanes, &mut pending, &voice, &staff);
                let is_chord = fragment.contains("<chord");
                let is_rest = fragment.contains("<rest");
                let hidden = fragment.contains("print-object=\"no\"")
                    || fragment.contains("print-object='no'");
                let kind = if !is_rest {
                    SilentCoverageCarrierKind::Pitched
                } else if hidden {
                    SilentCoverageCarrierKind::HiddenRest
                } else {
                    SilentCoverageCarrierKind::VisibleRest
                };
                let lane = lanes
                    .entry(voice.clone())
                    .or_insert_with(|| SilentCoverageLane {
                        voice: voice.clone(),
                        ..SilentCoverageLane::default()
                    });
                lane.staffs.insert(staff.clone());
                lane.carriers.push(SilentCoverageCarrier {
                    start: measure_start + position,
                    end: measure_start + end,
                    onset: cursor,
                    duration,
                    kind,
                });
                if !is_chord {
                    cursor = cursor.saturating_add(duration);
                }
                current_voice = Some(voice);
                current_staff = Some(staff);
            }
            _ => {}
        }
        scan = end;
    }

    if let (Some(voice), Some(staff)) = (current_voice.as_deref(), current_staff.as_deref()) {
        silent_coverage_flush_pending(&mut lanes, &mut pending, voice, staff);
    }

    let mut replacements = Vec::<SilentCoverageReplacement>::new();
    let mut candidate_lanes = 0_usize;
    for lane in lanes.values() {
        if lane.voice == "?" || lane.staffs.len() != 1 || expected <= 0 {
            continue;
        }
        let pitched = lane
            .carriers
            .iter()
            .any(|carrier| carrier.kind == SilentCoverageCarrierKind::Pitched);
        let hidden_rest_duration = silent_coverage_union_duration(
            &lane.carriers,
            expected,
            |carrier| carrier.kind == SilentCoverageCarrierKind::HiddenRest,
        );
        let visible_rest_duration = silent_coverage_union_duration(
            &lane.carriers,
            expected,
            |carrier| carrier.kind == SilentCoverageCarrierKind::VisibleRest,
        );
        let forward_duration = silent_coverage_union_duration(
            &lane.carriers,
            expected,
            |carrier| carrier.kind == SilentCoverageCarrierKind::Forward,
        );
        let timeline_duration =
            silent_coverage_union_duration(&lane.carriers, expected, |_| true);
        let max_end = lane
            .carriers
            .iter()
            .map(|carrier| carrier.onset.saturating_add(carrier.duration))
            .max()
            .unwrap_or(0);
        if pitched
            || hidden_rest_duration != 0
            || forward_duration <= 0
            || visible_rest_duration >= expected
            || timeline_duration != expected
            || max_end > expected
        {
            continue;
        }
        let staff = lane.staffs.iter().next().cloned().unwrap_or_default();
        let cross_staff_pitched_voice_present = lanes.values().any(|other| {
            other.staffs.len() > 1
                && other
                    .carriers
                    .iter()
                    .any(|carrier| carrier.kind == SilentCoverageCarrierKind::Pitched)
        });
        let same_staff_other_voice_pitched = lanes.values().any(|other| {
            other.voice != lane.voice
                && other.staffs.contains(&staff)
                && other
                    .carriers
                    .iter()
                    .any(|carrier| carrier.kind == SilentCoverageCarrierKind::Pitched)
        });
        if cross_staff_pitched_voice_present && same_staff_other_voice_pitched {
            continue;
        }
        let forward_carriers = lane
            .carriers
            .iter()
            .filter(|carrier| carrier.kind == SilentCoverageCarrierKind::Forward)
            .collect::<Vec<_>>();
        let mut lane_replacements = Vec::<SilentCoverageReplacement>::new();
        let mut representable = true;
        for carrier in forward_carriers {
            let Some(rest_xml) = silent_coverage_visible_rest_fragment(
                xml,
                carrier.start,
                carrier.duration,
                divisions,
                &lane.voice,
                &staff,
            ) else {
                representable = false;
                break;
            };
            lane_replacements.push(SilentCoverageReplacement {
                start: carrier.start,
                end: carrier.end,
                xml: rest_xml,
            });
        }
        if representable && !lane_replacements.is_empty() {
            candidate_lanes += 1;
            replacements.extend(lane_replacements);
        }
    }
    (replacements, candidate_lanes)
}

fn collect_silent_coverage_replacements(
    xml: &str,
) -> (Vec<SilentCoverageReplacement>, usize, i64) {
    let mut replacements = Vec::<SilentCoverageReplacement>::new();
    let mut candidates = 0_usize;
    let mut duration = 0_i64;
    let mut scan = 0_usize;
    let mut divisions = 1_i64;
    let mut beats = 4_i64;
    let mut beat_type = 4_i64;
    while let Some((start, end, _)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[start..end];
        let (next_divisions, next_beats, next_beat_type, expected) =
            parse_musicxml_current_meter(measure, divisions, beats, beat_type);
        divisions = next_divisions;
        beats = next_beats;
        beat_type = next_beat_type;
        let (mut measure_replacements, measure_candidates) =
            silent_coverage_measure_replacements(xml, start, end, divisions, expected);
        duration += measure_replacements
            .iter()
            .map(|replacement| {
                xml.get(replacement.start..replacement.end)
                    .map_or(0, |fragment| silent_coverage_direct_child_i64(fragment, "duration"))
            })
            .sum::<i64>();
        candidates += measure_candidates;
        replacements.append(&mut measure_replacements);
        scan = end;
    }
    replacements.sort_by_key(|left| std::cmp::Reverse(left.start));
    (replacements, candidates, duration)
}

#[derive(Debug, Clone, Default)]
pub(super) struct SilentVoiceCoveragePlan {
    pub(super) replacements: Vec<(usize, usize, String)>,
    pub(super) candidates: usize,
    pub(super) forward_nodes_planned: usize,
    pub(super) forward_duration_planned: i64,
    pub(super) cross_staff_rest_suppressions: usize,
}

pub(super) fn silent_coverage_candidate_rows(xml: &str) -> usize {
    collect_silent_coverage_replacements(xml).1
}

pub(super) fn plan_silent_voice_visible_measure_coverage(
    xml: &str,
    allow_cross_staff_suppression: bool,
) -> SilentVoiceCoveragePlan {
    let (mut replacements, candidates, forward_duration_planned) =
        collect_silent_coverage_replacements(xml);
    let forward_nodes_planned = replacements.len();
    let mut cross_staff_rest_suppressions = 0_usize;

    if allow_cross_staff_suppression {
        let expected_by_scope = render_scope_metric_expected_duration_map(xml);
        let mut selected_ranges = std::collections::BTreeSet::<(usize, usize)>::new();
        let mut scan = 0_usize;
        while let Some((measure_start, measure_end, scope_number)) =
            find_next_measure_bounds(xml, scan)
        {
            let measure = &xml[measure_start..measure_end];
            let expected = expected_by_scope
                .get(&scope_number)
                .copied()
                .or_else(|| infer_measure_duration_from_measure_xml(measure))
                .unwrap_or(384)
                .max(1);
            let fragments = collect_render_timeline_fragments_in_segment(
                measure,
                measure_start,
                scope_number,
            );
            let mut pitched_staffs_by_voice = std::collections::BTreeMap::<
                String,
                std::collections::BTreeSet<String>,
            >::new();
            let mut pitched_by_staff_voice = std::collections::BTreeMap::<
                (String, String),
                Vec<&RenderXmlTimelineFragmentSpan>,
            >::new();
            let mut rests_by_staff_voice = std::collections::BTreeMap::<
                (String, String),
                Vec<&RenderXmlTimelineFragmentSpan>,
            >::new();
            for fragment in &fragments {
                if fragment.is_visible_pitched
                    && !fragment.is_hidden
                    && !fragment.is_chord
                    && fragment.duration > 0
                {
                    pitched_staffs_by_voice
                        .entry(fragment.voice.clone())
                        .or_default()
                        .insert(fragment.staff.clone());
                    pitched_by_staff_voice
                        .entry((fragment.staff.clone(), fragment.voice.clone()))
                        .or_default()
                        .push(fragment);
                } else if fragment.is_visible_rest
                    && !fragment.is_hidden
                    && !fragment.is_chord
                    && fragment.duration > 0
                {
                    rests_by_staff_voice
                        .entry((fragment.staff.clone(), fragment.voice.clone()))
                        .or_default()
                        .push(fragment);
                }
            }
            if !pitched_staffs_by_voice
                .values()
                .any(|staffs| staffs.len() > 1)
            {
                scan = measure_end;
                continue;
            }

            for ((staff, voice), rests) in &rests_by_staff_voice {
                for rest in rests {
                    let start = rest.onset.max(0);
                    let end = rest
                        .onset
                        .saturating_add(rest.duration.max(0))
                        .min(expected);
                    let duration = end.saturating_sub(start);
                    if duration <= 0 {
                        continue;
                    }
                    let opposite_pitched_coverage = silent_coverage_fragment_union_duration(
                        &fragments,
                        start,
                        end,
                        |fragment| {
                            fragment.is_visible_pitched
                                && !fragment.is_hidden
                                && !fragment.is_chord
                                && fragment.voice == *voice
                                && fragment.staff.trim() != staff.trim()
                        },
                    );
                    let opposite_visible_voice_coverage =
                        silent_coverage_fragment_union_duration(
                            &fragments,
                            start,
                            end,
                            |fragment| {
                                (fragment.is_visible_pitched || fragment.is_visible_rest)
                                    && !fragment.is_hidden
                                    && !fragment.is_chord
                                    && fragment.voice == *voice
                                    && fragment.staff.trim() != staff.trim()
                            },
                        );
                    // A cross-staff voice may begin with a real visible rest on the
                    // opposite staff and continue there with a sounding event while a
                    // longer technical projection-rest remains on this staff.  Treat the
                    // complete opposite visible voice timeline as coverage only when the
                    // interval contains pitched evidence as well; rest-only overlap must
                    // never erase both representations of genuine silence.
                    let fully_pitched_covered = opposite_pitched_coverage >= duration;
                    let mixed_visible_voice_covered = opposite_pitched_coverage > 0
                        && opposite_visible_voice_coverage >= duration;
                    if (fully_pitched_covered || mixed_visible_voice_covered)
                        && selected_ranges.insert((rest.start, rest.end))
                    {
                        replacements.push(SilentCoverageReplacement {
                            start: rest.start,
                            end: rest.end,
                            xml: format!(
                                "\n      <forward><duration>{}</duration></forward>",
                                rest.duration.max(0)
                            ),
                        });
                        cross_staff_rest_suppressions =
                            cross_staff_rest_suppressions.saturating_add(1);
                    }
                }
            }

            for ((staff, voice), mut rests) in rests_by_staff_voice {
                if pitched_by_staff_voice.contains_key(&(staff.clone(), voice.clone()))
                    || !pitched_by_staff_voice
                        .keys()
                        .any(|(pitched_staff, pitched_voice)| {
                            pitched_staff == &staff && pitched_voice != &voice
                        })
                {
                    continue;
                }
                rests.sort_by_key(|rest| (rest.onset, rest.duration, rest.start));
                let mut covered_until = 0_i32;
                for rest in &rests {
                    let start = rest.onset.max(0);
                    let end = rest
                        .onset
                        .saturating_add(rest.duration.max(0))
                        .min(expected);
                    if start > covered_until {
                        break;
                    }
                    covered_until = covered_until.max(end);
                }
                if covered_until < expected {
                    continue;
                }
                for rest in rests {
                    if selected_ranges.insert((rest.start, rest.end)) {
                        replacements.push(SilentCoverageReplacement {
                            start: rest.start,
                            end: rest.end,
                            xml: format!(
                                "\n      <forward><duration>{}</duration></forward>",
                                rest.duration.max(0)
                            ),
                        });
                        cross_staff_rest_suppressions =
                            cross_staff_rest_suppressions.saturating_add(1);
                    }
                }
            }
            scan = measure_end;
        }
        replacements.sort_by_key(|left| std::cmp::Reverse(left.start));
    }

    SilentVoiceCoveragePlan {
        replacements: replacements
            .into_iter()
            .map(|replacement| (replacement.start, replacement.end, replacement.xml))
            .collect(),
        candidates,
        forward_nodes_planned,
        forward_duration_planned,
        cross_staff_rest_suppressions,
    }
}

#[cfg(test)]
mod cross_staff_visible_voice_rest_suppression_tests {
    use super::plan_silent_voice_visible_measure_coverage;

    #[test]
    fn suppresses_only_longer_cross_staff_technical_rest_when_opposite_visible_voice_covers_it() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes><note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note><note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note><note><pitch><step>E</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note><note><pitch><step>F</step><octave>5</octave></pitch><duration>288</duration><voice>1</voice><type>half</type><dot/><staff>1</staff></note><backup><duration>384</duration></backup><note><rest/><duration>48</duration><voice>1</voice><type>eighth</type><staff>2</staff></note><forward><duration>336</duration><voice>1</voice><staff>2</staff></forward></measure></part></score-partwise>";

        let plan = plan_silent_voice_visible_measure_coverage(xml, true);

        assert_eq!(plan.candidates, 0);
        assert_eq!(plan.forward_nodes_planned, 0);
        assert_eq!(plan.cross_staff_rest_suppressions, 1);
        assert_eq!(plan.replacements.len(), 1);
        assert!(plan
            .replacements
            .iter()
            .any(|(_, _, xml)| xml.contains("<forward><duration>48</duration></forward>")));
    }
}
