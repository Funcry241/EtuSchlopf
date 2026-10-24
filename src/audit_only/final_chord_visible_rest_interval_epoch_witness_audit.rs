use roxmltree::Document;
use std::fmt::Write as _;

pub(crate) const SECTION: &str = "FINAL-CHORD-VISIBLE-REST-INTERVAL-EPOCH-WITNESS-AUDIT";
pub(crate) const SECTION_RULE: &str = "------------------------------------------------------";

const LOCATOR_DOMAIN: &str = "final_chord_visible_rest_interval_epoch_witness_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntervalKind {
    Rest,
    Pitched,
    Forward,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TimelineInterval {
    part: String,
    measure: String,
    staff: String,
    voice: String,
    start: i64,
    end: i64,
    kind: IntervalKind,
    visible: bool,
    owns_chord_member: bool,
    whole_measure_rest: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EpochWitness {
    locator_sha256: String,
    part: String,
    measure: String,
    staff: String,
    voice: String,
    start: i64,
    end: i64,
    source_prebind_occupied: Vec<(i64, i64)>,
    master_pitched_occupied: Vec<(i64, i64)>,
    expected_silent: Vec<(i64, i64)>,
    prebind_occupancy_stable: bool,
    master_forward: Vec<(i64, i64)>,
    master_rest: Vec<(i64, i64)>,
    render_forward: Vec<(i64, i64)>,
    render_rest: Vec<(i64, i64)>,
    master_matches_expected_split: bool,
    render_whole_source_rest_forward: bool,
    render_available: bool,
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

fn note_is_whole_measure_rest(node: roxmltree::Node<'_, '_>) -> bool {
    node.children().any(|child| {
        child.is_element()
            && child.has_tag_name("rest")
            && child.attribute("measure") == Some("yes")
    })
}

fn collect_timeline(xml: &str) -> Option<Vec<TimelineInterval>> {
    let document = Document::parse(xml).ok()?;
    let mut intervals = Vec::<TimelineInterval>::new();

    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
    {
        let part_id = part.attribute("id").unwrap_or("-").to_string();
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
            .enumerate()
        {
            let measure_id = measure
                .attribute("number")
                .map(str::to_string)
                .unwrap_or_else(|| measure_index.saturating_add(1).to_string());
            let mut cursor = 0_i64;
            let mut last_anchor_onset = None::<i64>;
            let mut last_anchor_interval = None::<usize>;

            for node in measure.children().filter(|node| node.is_element()) {
                if node.has_tag_name("backup") {
                    cursor = cursor.saturating_sub(child_duration(node));
                    last_anchor_onset = None;
                    last_anchor_interval = None;
                    continue;
                }
                if node.has_tag_name("forward") {
                    let duration = child_duration(node);
                    if duration > 0 {
                        intervals.push(TimelineInterval {
                            part: part_id.clone(),
                            measure: measure_id.clone(),
                            staff: child_text(node, "staff").unwrap_or_else(|| "1".to_string()),
                            voice: child_text(node, "voice").unwrap_or_else(|| "1".to_string()),
                            start: cursor,
                            end: cursor.saturating_add(duration),
                            kind: IntervalKind::Forward,
                            visible: true,
                            owns_chord_member: false,
                            whole_measure_rest: false,
                        });
                    }
                    cursor = cursor.saturating_add(duration);
                    last_anchor_onset = None;
                    last_anchor_interval = None;
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
                if chord
                    && let Some(anchor_index) = last_anchor_interval
                    && let Some(anchor) = intervals.get_mut(anchor_index)
                {
                    anchor.owns_chord_member = true;
                }

                let rest = note_is_rest(node);
                let pitched = !rest && note_has_pitch(node);
                let kind = if rest {
                    Some(IntervalKind::Rest)
                } else if pitched {
                    Some(IntervalKind::Pitched)
                } else {
                    None
                };
                let mut pushed_index = None;
                if let Some(kind) = kind
                    && duration > 0
                {
                    let index = intervals.len();
                    intervals.push(TimelineInterval {
                        part: part_id.clone(),
                        measure: measure_id.clone(),
                        staff: child_text(node, "staff").unwrap_or_else(|| "1".to_string()),
                        voice: child_text(node, "voice").unwrap_or_else(|| "1".to_string()),
                        start: onset,
                        end: onset.saturating_add(duration),
                        kind,
                        visible: node.attribute("print-object") != Some("no"),
                        owns_chord_member: false,
                        whole_measure_rest: rest && note_is_whole_measure_rest(node),
                    });
                    pushed_index = Some(index);
                }

                if !chord {
                    last_anchor_onset = Some(onset);
                    last_anchor_interval = pushed_index;
                    cursor = cursor.saturating_add(duration);
                }
            }
        }
    }

    Some(intervals)
}

fn overlaps(left: (i64, i64), right: (i64, i64)) -> bool {
    left.0 < right.1 && right.0 < left.1
}

fn merged_clipped_intervals(
    intervals: &[TimelineInterval],
    reference: &TimelineInterval,
    kind: IntervalKind,
    require_visible: bool,
) -> Vec<(i64, i64)> {
    let mut rows = intervals
        .iter()
        .filter(|row| {
            row.part == reference.part
                && row.measure == reference.measure
                && row.staff == reference.staff
                && row.voice == reference.voice
                && row.kind == kind
                && (!require_visible || row.visible)
                && overlaps((row.start, row.end), (reference.start, reference.end))
        })
        .map(|row| (row.start.max(reference.start), row.end.min(reference.end)))
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

fn complement_interval(
    start: i64,
    end: i64,
    occupied: &[(i64, i64)],
) -> Vec<(i64, i64)> {
    let mut out = Vec::<(i64, i64)>::new();
    let mut cursor = start;
    for &(occupied_start, occupied_end) in occupied {
        if cursor < occupied_start {
            out.push((cursor, occupied_start));
        }
        cursor = cursor.max(occupied_end);
    }
    if cursor < end {
        out.push((cursor, end));
    }
    out
}

fn interval_duration(intervals: &[(i64, i64)]) -> i64 {
    intervals
        .iter()
        .map(|(start, end)| end.saturating_sub(*start))
        .sum()
}

fn has_ambiguous_visible_rest_overlap(
    source: &[TimelineInterval],
    reference: &TimelineInterval,
) -> bool {
    source.iter().any(|row| {
        !std::ptr::eq(row, reference)
            && row.kind == IntervalKind::Rest
            && row.visible
            && row.part == reference.part
            && row.measure == reference.measure
            && row.staff == reference.staff
            && row.voice == reference.voice
            && overlaps((row.start, row.end), (reference.start, reference.end))
    })
}

fn locator_sha256(source_sha256: &str, rest: &TimelineInterval) -> String {
    let payload = format!(
        "{LOCATOR_DOMAIN}\nsource_sha256={source_sha256}\npart={}\nmeasure={}\nstaff={}\nvoice={}\nstart={}\nend={}\n",
        rest.part, rest.measure, rest.staff, rest.voice, rest.start, rest.end,
    );
    crate::sha256::hex(payload.as_bytes())
}

fn build_epoch_witnesses(
    pre_final_chord_xml: &str,
    final_master_xml: &str,
    final_render_xml: Option<&str>,
) -> Option<Vec<EpochWitness>> {
    let source = collect_timeline(pre_final_chord_xml)?;
    let master = collect_timeline(final_master_xml)?;
    let render = final_render_xml.and_then(collect_timeline);
    let source_sha256 = crate::sha256::hex(pre_final_chord_xml.as_bytes());
    let mut witnesses = Vec::<EpochWitness>::new();

    for rest in source.iter().filter(|row| {
        row.kind == IntervalKind::Rest
            && row.visible
            && !row.owns_chord_member
            && !row.whole_measure_rest
            && row.end > row.start
    }) {
        if has_ambiguous_visible_rest_overlap(&source, rest) {
            continue;
        }
        let source_prebind_occupied = merged_clipped_intervals(
            &source,
            rest,
            IntervalKind::Pitched,
            true,
        );
        let master_pitched_occupied = merged_clipped_intervals(
            &master,
            rest,
            IntervalKind::Pitched,
            true,
        );
        let occupied_duration = interval_duration(&master_pitched_occupied);
        let rest_duration = rest.end.saturating_sub(rest.start);
        if occupied_duration <= 0 || occupied_duration >= rest_duration {
            continue;
        }
        let expected_silent =
            complement_interval(rest.start, rest.end, &master_pitched_occupied);
        let prebind_occupancy_stable = source_prebind_occupied == master_pitched_occupied;
        let master_forward = merged_clipped_intervals(
            &master,
            rest,
            IntervalKind::Forward,
            false,
        );
        let master_rest = merged_clipped_intervals(
            &master,
            rest,
            IntervalKind::Rest,
            true,
        );
        let render_forward = render.as_deref().map_or_else(Vec::new, |rows| {
            merged_clipped_intervals(rows, rest, IntervalKind::Forward, false)
        });
        let render_rest = render.as_deref().map_or_else(Vec::new, |rows| {
            merged_clipped_intervals(rows, rest, IntervalKind::Rest, true)
        });
        let master_matches_expected_split =
            master_forward == master_pitched_occupied && master_rest == expected_silent;
        let render_available = render.is_some();
        let render_whole_source_rest_forward = render_available
            && render_forward == vec![(rest.start, rest.end)]
            && render_rest.is_empty();

        witnesses.push(EpochWitness {
            locator_sha256: locator_sha256(&source_sha256, rest),
            part: rest.part.clone(),
            measure: rest.measure.clone(),
            staff: rest.staff.clone(),
            voice: rest.voice.clone(),
            start: rest.start,
            end: rest.end,
            source_prebind_occupied,
            master_pitched_occupied,
            expected_silent,
            prebind_occupancy_stable,
            master_forward,
            master_rest,
            render_forward,
            render_rest,
            master_matches_expected_split,
            render_whole_source_rest_forward,
            render_available,
        });
    }

    witnesses.sort_by(|left, right| {
        left.locator_sha256
            .cmp(&right.locator_sha256)
            .then_with(|| left.start.cmp(&right.start))
            .then_with(|| left.end.cmp(&right.end))
    });
    Some(witnesses)
}

fn interval_list_label(intervals: &[(i64, i64)]) -> String {
    if intervals.is_empty() {
        return "none".to_string();
    }
    intervals
        .iter()
        .map(|(start, end)| format!("{start}..{end}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) fn append_final_chord_visible_rest_interval_epoch_witness_audit_section(
    out: &mut String,
    pre_final_chord_xml: &str,
    final_master_xml: &str,
    final_render_xml: Option<&str>,
) {
    let source_sha256 = crate::sha256::hex(pre_final_chord_xml.as_bytes());
    let master_sha256 = crate::sha256::hex(final_master_xml.as_bytes());
    let render_sha256 = final_render_xml
        .map(|xml| crate::sha256::hex(xml.as_bytes()))
        .unwrap_or_else(|| "UNAVAILABLE".to_string());
    let _ = writeln!(
        out,
        "EpochBinding: PreFinalChordMasterSource exact; FinalMaster persisted_final; FinalRender {}; RenderPreFinalChordInput UNAVAILABLE_AT_REPORT_BOUNDARY.",
        if final_render_xml.is_some() { "persisted_final" } else { "UNAVAILABLE" }
    );
    let _ = writeln!(
        out,
        "EpochSha256: PreFinalChordMasterSource {source_sha256}; FinalMaster {master_sha256}; FinalRender {render_sha256}."
    );

    let Some(witnesses) = build_epoch_witnesses(
        pre_final_chord_xml,
        final_master_xml,
        final_render_xml,
    ) else {
        out.push_str("ParseStatus: UNAVAILABLE; no epoch witness classification emitted.\n");
        out.push_str("Authority: audit_only_read_only_no_mutation_no_release_veto; MusicMutation nein.\n");
        return;
    };

    let master_matches = witnesses
        .iter()
        .filter(|row| row.master_matches_expected_split)
        .count();
    let render_whole = witnesses
        .iter()
        .filter(|row| row.render_whole_source_rest_forward)
        .count();
    let divergent = witnesses
        .iter()
        .filter(|row| {
            row.master_matches_expected_split && row.render_whole_source_rest_forward
        })
        .count();
    let _ = writeln!(
        out,
        "Summary: SourceRestCarriersWithFinalMasterPartialSameLaneVisibleOccupancy {}; FinalMasterMatchesRIntersectNForwardAndRMinusNRest {}; FinalRenderAvailable {}; FinalRenderWholeRForward {}; JointEpochWitnesses {}.",
        witnesses.len(),
        master_matches,
        if final_render_xml.is_some() { "yes" } else { "no" },
        render_whole,
        divergent,
    );

    for (index, witness) in witnesses.iter().enumerate() {
        let classification = if !witness.render_available {
            if witness.master_matches_expected_split {
                "MASTER_EXPECTED_SPLIT_RENDER_UNAVAILABLE"
            } else {
                "SOURCE_PARTIAL_OCCUPANCY_RENDER_UNAVAILABLE"
            }
        } else {
            match (
                witness.master_matches_expected_split,
                witness.render_whole_source_rest_forward,
            ) {
                (true, true) => "MASTER_EXPECTED_SPLIT_RENDER_WHOLE_R_FORWARD",
                (true, false) => "MASTER_EXPECTED_SPLIT_RENDER_OTHER",
                (false, true) => "MASTER_OTHER_RENDER_WHOLE_R_FORWARD",
                (false, false) => "SOURCE_PARTIAL_OCCUPANCY_ONLY",
            }
        };
        let _ = writeln!(
            out,
            "Witness#{}: PhysicalLocatorSha256 {}; Part {}; Measure {}; Staff {}; Voice {}; R {}..{}; PreBindSourcePitchedN {}; FinalMasterPitchedN {}; ExpectedRMinusN {}; PreBindOccupancyStable {}; FinalMasterForward {}; FinalMasterVisibleRest {}; FinalRenderForward {}; FinalRenderVisibleRest {}; MasterSplitMatch {}; RenderWholeRForward {}; Classification {}.",
            index.saturating_add(1),
            witness.locator_sha256,
            witness.part,
            witness.measure,
            witness.staff,
            witness.voice,
            witness.start,
            witness.end,
            interval_list_label(&witness.source_prebind_occupied),
            interval_list_label(&witness.master_pitched_occupied),
            interval_list_label(&witness.expected_silent),
            if witness.prebind_occupancy_stable { "yes" } else { "no" },
            interval_list_label(&witness.master_forward),
            interval_list_label(&witness.master_rest),
            interval_list_label(&witness.render_forward),
            interval_list_label(&witness.render_rest),
            if witness.master_matches_expected_split { "yes" } else { "no" },
            if witness.render_whole_source_rest_forward { "yes" } else { "no" },
            classification,
        );
    }

    out.push_str("InterpretationBoundary: a joint witness proves that the larger visible rest carrier R existed in the real pre-FinalChord Master source and that the two final persisted artifacts realize the same interval differently; it does not identify the Render pre-FinalChord input or prove which Render writer created the whole-R forward.\n");
    out.push_str("Authority: audit_only_read_only_no_mutation_no_release_veto; MusicMutation nein.\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(body: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
    <measure>
      <attributes><divisions>96</divisions></attributes>
{body}
    </measure>
  </part>
</score-partwise>
"#
        )
    }

    #[test]
    fn detects_partial_same_lane_occupancy_master_split_and_render_whole_forward() {
        let source = score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>"#,
        );
        let master = score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <forward><duration>24</duration><voice>2</voice><staff>2</staff></forward>
      <note><rest/><duration>72</duration><voice>2</voice><staff>2</staff></note>"#,
        );
        let render = score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <forward><duration>96</duration><voice>2</voice><staff>2</staff></forward>"#,
        );

        let witnesses = build_epoch_witnesses(&source, &master, Some(&render))
            .expect("synthetic epoch witness score must parse");
        assert_eq!(witnesses.len(), 1);
        assert_eq!(witnesses[0].source_prebind_occupied, vec![(0, 24)]);
        assert_eq!(witnesses[0].master_pitched_occupied, vec![(0, 24)]);
        assert_eq!(witnesses[0].expected_silent, vec![(24, 96)]);
        assert!(witnesses[0].prebind_occupancy_stable);
        assert!(witnesses[0].master_matches_expected_split);
        assert!(witnesses[0].render_whole_source_rest_forward);
    }

    #[test]
    fn final_master_bound_voice_can_supply_n_without_claiming_prebind_stability() {
        let source = score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>"#,
        );
        let master = score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <forward><duration>24</duration><voice>2</voice><staff>2</staff></forward>
      <note><rest/><duration>72</duration><voice>2</voice><staff>2</staff></note>"#,
        );
        let render = score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <forward><duration>96</duration><voice>2</voice><staff>2</staff></forward>"#,
        );

        let witnesses = build_epoch_witnesses(&source, &master, Some(&render))
            .expect("synthetic bound-voice epoch witness score must parse");
        assert_eq!(witnesses.len(), 1);
        assert!(witnesses[0].source_prebind_occupied.is_empty());
        assert_eq!(witnesses[0].master_pitched_occupied, vec![(0, 24)]);
        assert!(!witnesses[0].prebind_occupancy_stable);
        assert!(witnesses[0].master_matches_expected_split);
        assert!(witnesses[0].render_whole_source_rest_forward);
    }

    #[test]
    fn cross_voice_pitch_does_not_create_same_lane_source_witness() {
        let source = score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>"#,
        );
        let witnesses = build_epoch_witnesses(&source, &source, Some(&source))
            .expect("synthetic cross-voice score must parse");
        assert!(witnesses.is_empty());
    }
}
