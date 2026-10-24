use roxmltree::Document;
use std::collections::BTreeSet;
use std::fmt::Write as _;

#[derive(Debug, Clone, PartialEq, Eq)]
struct RestCarrierWitness {
    measure: usize,
    staff: String,
    voice: String,
    onset: i64,
    duration: i64,
    overlap_pitch: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntervalKind {
    Rest,
    Pitched,
    Forward,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TimelineInterval {
    part: String,
    measure: usize,
    staff: String,
    voice: String,
    start: i64,
    end: i64,
    kind: IntervalKind,
    visible: bool,
}

fn child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.has_tag_name(name))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn child_duration(node: roxmltree::Node<'_, '_>) -> i64 {
    child_text(node, "duration")
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
        .max(0)
}

fn note_is_rest(node: roxmltree::Node<'_, '_>) -> bool {
    node.children()
        .any(|child| child.is_element() && child.has_tag_name("rest"))
}

fn note_has_pitch(node: roxmltree::Node<'_, '_>) -> bool {
    node.children()
        .any(|child| child.is_element() && child.has_tag_name("pitch"))
}

fn note_is_chord(node: roxmltree::Node<'_, '_>) -> bool {
    node.children()
        .any(|child| child.is_element() && child.has_tag_name("chord"))
}

fn collect_timeline(xml: &str) -> Option<Vec<TimelineInterval>> {
    let document = Document::parse(xml).ok()?;
    let mut intervals = Vec::<TimelineInterval>::new();

    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
    {
        let part_id = part.attribute("id").unwrap_or("-").to_string();
        for (ordinal, measure) in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
            .enumerate()
        {
            let measure_id = measure
                .attribute("number")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(ordinal.saturating_add(1));
            let mut cursor = 0_i64;
            let mut last_anchor_onset = None::<i64>;

            for node in measure.children().filter(|node| node.is_element()) {
                if node.has_tag_name("backup") {
                    cursor = cursor.saturating_sub(child_duration(node));
                    last_anchor_onset = None;
                    continue;
                }
                if node.has_tag_name("forward") {
                    let duration = child_duration(node);
                    if duration > 0 {
                        intervals.push(TimelineInterval {
                            part: part_id.clone(),
                            measure: measure_id,
                            staff: child_text(node, "staff").unwrap_or_else(|| "1".to_string()),
                            voice: child_text(node, "voice").unwrap_or_else(|| "1".to_string()),
                            start: cursor,
                            end: cursor.saturating_add(duration),
                            kind: IntervalKind::Forward,
                            visible: true,
                        });
                    }
                    cursor = cursor.saturating_add(duration);
                    last_anchor_onset = None;
                    continue;
                }
                if !node.has_tag_name("note") {
                    continue;
                }

                let duration = child_duration(node);
                let chord = note_is_chord(node);
                let onset = if chord {
                    last_anchor_onset.unwrap_or(cursor)
                } else {
                    cursor
                };
                let rest = note_is_rest(node);
                let kind = if rest {
                    Some(IntervalKind::Rest)
                } else if note_has_pitch(node) {
                    Some(IntervalKind::Pitched)
                } else {
                    None
                };
                if let Some(kind) = kind
                    && duration > 0
                {
                    intervals.push(TimelineInterval {
                        part: part_id.clone(),
                        measure: measure_id,
                        staff: child_text(node, "staff").unwrap_or_else(|| "1".to_string()),
                        voice: child_text(node, "voice").unwrap_or_else(|| "1".to_string()),
                        start: onset,
                        end: onset.saturating_add(duration),
                        kind,
                        visible: node.attribute("print-object") != Some("no"),
                    });
                }
                if !chord {
                    last_anchor_onset = Some(onset);
                    cursor = cursor.saturating_add(duration);
                }
            }
        }
    }

    Some(intervals)
}

fn parse_chosen_witnesses(text: &str) -> Vec<RestCarrierWitness> {
    text.split(';')
        .filter_map(|row| {
            let tokens = row.split_whitespace().collect::<Vec<_>>();
            Some(RestCarrierWitness {
                measure: tokens.get(1)?.parse().ok()?,
                staff: tokens.get(3)?.to_string(),
                voice: tokens.get(5)?.to_string(),
                onset: tokens.get(7)?.parse().ok()?,
                duration: tokens.get(9)?.parse::<i64>().ok()?.max(0),
                overlap_pitch: tokens.get(13).is_some_and(|value| *value == "ja"),
            })
        })
        .filter(|row| row.duration > 0)
        .collect()
}

fn overlaps(start: i64, end: i64, other_start: i64, other_end: i64) -> bool {
    start < other_end && other_start < end
}

fn merged_coverage(
    intervals: &[TimelineInterval],
    witness: &RestCarrierWitness,
    part: &str,
    same_lane: bool,
) -> Vec<(i64, i64)> {
    let end = witness.onset.saturating_add(witness.duration);
    let mut rows = intervals
        .iter()
        .filter(|row| {
            row.part == part
                && row.measure == witness.measure
                && row.staff == witness.staff
                && row.kind == IntervalKind::Pitched
                && row.visible
                && (if same_lane {
                    row.voice == witness.voice
                } else {
                    row.voice != witness.voice
                })
                && overlaps(row.start, row.end, witness.onset, end)
        })
        .map(|row| (row.start.max(witness.onset), row.end.min(end)))
        .filter(|(start, end)| end > start)
        .collect::<Vec<_>>();
    rows.sort_unstable();

    let mut merged = Vec::<(i64, i64)>::new();
    for (start, end) in rows {
        if let Some(last) = merged.last_mut()
            && start <= last.1
        {
            last.1 = last.1.max(end);
            continue;
        }
        merged.push((start, end));
    }
    merged
}

fn complement(start: i64, end: i64, coverage: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut out = Vec::<(i64, i64)>::new();
    let mut cursor = start;
    for &(covered_start, covered_end) in coverage {
        if cursor < covered_start {
            out.push((cursor, covered_start));
        }
        cursor = cursor.max(covered_end);
    }
    if cursor < end {
        out.push((cursor, end));
    }
    out
}

fn format_intervals(intervals: &[(i64, i64)]) -> String {
    if intervals.is_empty() {
        return "none".to_string();
    }
    intervals
        .iter()
        .map(|(start, end)| format!("{start}..{end}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn witness_parts(
    intervals: &[TimelineInterval],
    witness: &RestCarrierWitness,
) -> BTreeSet<String> {
    let end = witness.onset.saturating_add(witness.duration);
    intervals
        .iter()
        .filter(|row| {
            row.measure == witness.measure
                && row.staff == witness.staff
                && row.voice == witness.voice
                && overlaps(row.start, row.end, witness.onset, end)
        })
        .map(|row| row.part.clone())
        .collect()
}

pub(crate) fn append_lower_staff_overlapping_rest_bound_dispensability_audit(
    out: &mut String,
    chosen_evidence: &str,
    final_master_xml: &str,
    final_render_xml: Option<&str>,
) {
    let witnesses = parse_chosen_witnesses(chosen_evidence);
    let master = collect_timeline(final_master_xml);
    let render = final_render_xml.and_then(collect_timeline);

    if witnesses.is_empty() {
        out.push_str("LowerStaffOverlappingRestBoundDispensabilityAudit: chosen_witnesses none; Authority none; Status NOT_APPLICABLE.\n");
        return;
    }
    let Some(master) = master else {
        out.push_str("LowerStaffOverlappingRestBoundDispensabilityAudit: final_master_parse failed; Authority none; Status UNPROVEN.\n");
        return;
    };

    for (index, witness) in witnesses.iter().enumerate() {
        let end = witness.onset.saturating_add(witness.duration);
        let mut parts = witness_parts(&master, witness);
        if let Some(render_rows) = render.as_ref() {
            parts.extend(witness_parts(render_rows, witness));
        }
        if parts.is_empty() {
            let _ = writeln!(
                out,
                "LowerStaffOverlappingRestBoundDispensabilityAudit#{}: Measure {}; Staff {}; Voice {}; R {}..{}; TriggerOverlapPitch {}; BoundPart none; PositiveTechnicalRole none_observed_at_writer; Authority none; Status UNPROVEN.",
                index.saturating_add(1),
                witness.measure,
                witness.staff,
                witness.voice,
                witness.onset,
                end,
                if witness.overlap_pitch { "ja" } else { "nein" },
            );
            continue;
        }

        for part in parts {
            let master_other = merged_coverage(&master, witness, &part, false);
            let master_same = merged_coverage(&master, witness, &part, true);
            let master_other_uncovered = complement(witness.onset, end, &master_other);
            let master_same_uncovered = complement(witness.onset, end, &master_same);
            let (render_other, render_same) = render.as_ref().map_or_else(
                || (Vec::new(), Vec::new()),
                |rows| {
                    (
                        merged_coverage(rows, witness, &part, false),
                        merged_coverage(rows, witness, &part, true),
                    )
                },
            );
            let _ = writeln!(
                out,
                "LowerStaffOverlappingRestBoundDispensabilityAudit#{}: Measure {}; Part {}; Staff {}; Voice {}; R {}..{}; TriggerOverlapPitch {}; FinalMasterOtherVoiceCoverage [{}]; FinalMasterOtherVoiceUncovered [{}]; FinalMasterSameLaneCoverage [{}]; FinalMasterSameLaneUncovered [{}]; FinalRenderOtherVoiceCoverage [{}]; FinalRenderSameLaneCoverage [{}]; PositiveTechnicalRole none_observed_at_writer; CoverageAuthority none; Status UNPROVEN.",
                index.saturating_add(1),
                witness.measure,
                part,
                witness.staff,
                witness.voice,
                witness.onset,
                end,
                if witness.overlap_pitch { "ja" } else { "nein" },
                format_intervals(&master_other),
                format_intervals(&master_other_uncovered),
                format_intervals(&master_same),
                format_intervals(&master_same_uncovered),
                format_intervals(&render_other),
                format_intervals(&render_same),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        append_lower_staff_overlapping_rest_bound_dispensability_audit, complement,
        parse_chosen_witnesses,
    };

    #[test]
    fn chosen_evidence_is_parsed_without_corpus_identity() {
        let rows = parse_chosen_witnesses(
            "marker 7 staff 2 voice 3 onset 10 duration 100 overlap_rest_or_same_beat nein overlap_pitch ja measure_local_aux_rest nein",
        );
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].measure, 7);
        assert_eq!(rows[0].onset, 10);
        assert_eq!(rows[0].duration, 100);
        assert!(rows[0].overlap_pitch);
    }

    #[test]
    fn complement_keeps_each_uncovered_subinterval() {
        assert_eq!(complement(10, 110, &[(10, 40), (70, 80)]), vec![(40, 70), (80, 110)]);
    }

    #[test]
    fn complete_foreign_lane_sound_remains_non_authoritative() {
        let measure_id = 7;
        let master = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{measure_id}\"><note><pitch><step>C</step><octave>4</octave></pitch><duration>100</duration><voice>9</voice><staff>2</staff></note><backup><duration>100</duration></backup><forward><duration>100</duration><voice>3</voice><staff>2</staff></forward></measure></part></score-partwise>"
        );
        let mut out = String::new();
        append_lower_staff_overlapping_rest_bound_dispensability_audit(
            &mut out,
            "marker 7 staff 2 voice 3 onset 0 duration 100 overlap_rest_or_same_beat nein overlap_pitch ja measure_local_aux_rest nein",
            &master,
            Some(&master),
        );
        assert!(out.contains("FinalMasterOtherVoiceCoverage [0..100]"));
        assert!(out.contains("CoverageAuthority none"));
        assert!(out.contains("Status UNPROVEN"));
    }
}
