use roxmltree::Document;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub(crate) const SECTION: &str = "STAFF-LOCAL-REST-OWNERSHIP-AUDIT";
pub(crate) const SECTION_RULE: &str = "--------------------------------";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RestOwnershipEvent {
    part: String,
    measure: String,
    staff: String,
    voice: String,
    onset: i64,
    duration: i64,
    visible: bool,
    is_rest: bool,
    is_chord: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct StaffLocalRestOwnershipAudit {
    master_parse_ok: bool,
    render_parse_ok: bool,
    exact_cross_voice_duplicate_groups: usize,
    exact_cross_voice_duplicate_excess_rests: usize,
    same_lane_rest_pitch_overlap_pairs: usize,
    master_hidden_to_render_visible_exact: usize,
    frozen_sounding_voice_binding_complete: bool,
    frozen_sounding_voice_spans: usize,
    frozen_sounding_unbound_members: usize,
    frozen_same_voice_rest_conflict_rests: usize,
    frozen_same_voice_rest_overlap_pairs: usize,
    duplicate_groups_with_frozen_sounding_conflict: usize,
    duplicate_groups_awaiting_negative_ownership: usize,
    rows: Vec<String>,
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
}

fn collect_events(xml: &str) -> Option<Vec<RestOwnershipEvent>> {
    let document = Document::parse(xml).ok()?;
    let mut events = Vec::new();

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
                .unwrap_or_else(|| (measure_index + 1).to_string());
            let mut cursor = 0_i64;
            let mut last_note_onset = 0_i64;

            for node in measure.children().filter(|node| node.is_element()) {
                if node.has_tag_name("backup") {
                    cursor = cursor.saturating_sub(child_duration(node).max(0));
                    continue;
                }
                if node.has_tag_name("forward") {
                    cursor = cursor.saturating_add(child_duration(node).max(0));
                    continue;
                }
                if !node.has_tag_name("note") {
                    continue;
                }

                let duration = child_duration(node).max(0);
                let is_chord = node
                    .children()
                    .any(|child| child.is_element() && child.has_tag_name("chord"));
                let onset = if is_chord { last_note_onset } else { cursor };
                if !is_chord {
                    last_note_onset = onset;
                }
                let is_rest = node
                    .children()
                    .any(|child| child.is_element() && child.has_tag_name("rest"));
                let visible = node.attribute("print-object") != Some("no");
                let staff = child_text(node, "staff").unwrap_or_else(|| "1".to_string());
                let voice = child_text(node, "voice").unwrap_or_else(|| "1".to_string());

                events.push(RestOwnershipEvent {
                    part: part_id.clone(),
                    measure: measure_id.clone(),
                    staff,
                    voice,
                    onset,
                    duration,
                    visible,
                    is_rest,
                    is_chord,
                });

                if !is_chord {
                    cursor = cursor.saturating_add(duration);
                }
            }
        }
    }

    Some(events)
}

fn intervals_overlap(
    left_onset: i64,
    left_duration: i64,
    right_onset: i64,
    right_duration: i64,
) -> bool {
    left_duration > 0
        && right_duration > 0
        && left_onset < right_onset.saturating_add(right_duration)
        && right_onset < left_onset.saturating_add(left_duration)
}

fn build_staff_local_rest_ownership_audit(
    master_xml: &str,
    render_xml: &str,
    frozen_sounding_voice_binding_complete: bool,
    frozen_sounding_unbound_members: usize,
    frozen_sounding_voice_spans: &[(usize, u8, i64, i64, String)],
) -> StaffLocalRestOwnershipAudit {
    let Some(master_events) = collect_events(master_xml) else {
        return StaffLocalRestOwnershipAudit::default();
    };
    let Some(render_events) = collect_events(render_xml) else {
        return StaffLocalRestOwnershipAudit {
            master_parse_ok: true,
            ..StaffLocalRestOwnershipAudit::default()
        };
    };

    let mut audit = StaffLocalRestOwnershipAudit {
        master_parse_ok: true,
        render_parse_ok: true,
        frozen_sounding_voice_binding_complete,
        frozen_sounding_voice_spans: frozen_sounding_voice_spans.len(),
        frozen_sounding_unbound_members,
        ..StaffLocalRestOwnershipAudit::default()
    };

    let frozen_overlap = |measure: &str, staff: &str, voice: &str, onset: i64, duration: i64| {
        let Some(measure_number) = measure
            .parse::<usize>()
            .ok()
            .filter(|value| *value > 0)
        else {
            return false;
        };
        let Some(staff_number) = staff.parse::<u8>().ok() else {
            return false;
        };
        let local_end = onset.saturating_add(duration.max(0));
        frozen_sounding_voice_spans.iter().any(
            |(span_measure, span_staff, span_start, span_end, span_voice)| {
                *span_measure == measure_number
                    && *span_staff == staff_number
                    && span_voice.trim() == voice.trim()
                    && onset < *span_end
                    && *span_start < local_end
            },
        )
    };

    let mut exact_visible_rest_groups = BTreeMap::<
        (String, String, String, i64, i64),
        BTreeSet<String>,
    >::new();
    for event in render_events.iter().filter(|event| {
        event.visible && event.is_rest && !event.is_chord && event.duration > 0
    }) {
        exact_visible_rest_groups
            .entry((
                event.part.clone(),
                event.measure.clone(),
                event.staff.clone(),
                event.onset,
                event.duration,
            ))
            .or_default()
            .insert(event.voice.clone());
    }
    for ((part, measure, staff, onset, duration), voices) in exact_visible_rest_groups {
        if voices.len() < 2 {
            continue;
        }
        audit.exact_cross_voice_duplicate_groups =
            audit.exact_cross_voice_duplicate_groups.saturating_add(1);
        audit.exact_cross_voice_duplicate_excess_rests = audit
            .exact_cross_voice_duplicate_excess_rests
            .saturating_add(voices.len().saturating_sub(1));
        if frozen_sounding_voice_binding_complete {
            let frozen_conflict = voices
                .iter()
                .any(|voice| frozen_overlap(&measure, &staff, voice, onset, duration));
            let frozen_class = if frozen_conflict {
                audit.duplicate_groups_with_frozen_sounding_conflict = audit
                    .duplicate_groups_with_frozen_sounding_conflict
                    .saturating_add(1);
                "POSITIVE_SOUNDING_CONFLICT"
            } else {
                audit.duplicate_groups_awaiting_negative_ownership = audit
                    .duplicate_groups_awaiting_negative_ownership
                    .saturating_add(1);
                "NEGATIVE_OWNERSHIP_UNBOUND"
            };
            audit.rows.push(format!(
                concat!(
                    "FROZEN_DUPLICATE_REST_GROUP_CLASS part={} measure={} staff={} ",
                    "interval={}-{} class={}"
                ),
                part,
                measure,
                staff,
                onset,
                onset.saturating_add(duration),
                frozen_class,
            ));
        }
        audit.rows.push(format!(
            "DUPLICATE_VISIBLE_REST_OWNERSHIP part={} measure={} staff={} interval={}-{} voices={}",
            part,
            measure,
            staff,
            onset,
            onset.saturating_add(duration),
            voices.into_iter().collect::<Vec<_>>().join(",")
        ));
    }

    let visible_rests = render_events.iter().filter(|event| {
        event.visible && event.is_rest && !event.is_chord && event.duration > 0
    });
    for rest in visible_rests {
        for pitched in render_events.iter().filter(|event| {
            event.visible
                && !event.is_rest
                && !event.is_chord
                && event.duration > 0
                && event.part == rest.part
                && event.measure == rest.measure
                && event.staff == rest.staff
                && event.voice == rest.voice
        }) {
            if !intervals_overlap(rest.onset, rest.duration, pitched.onset, pitched.duration) {
                continue;
            }
            audit.same_lane_rest_pitch_overlap_pairs =
                audit.same_lane_rest_pitch_overlap_pairs.saturating_add(1);
            audit.rows.push(format!(
                concat!(
                "SAME_LANE_REST_PITCH_OVERLAP part={} measure={} staff={} voice={} ",
                "rest={}-{} pitch={}-{}"
            ),
                rest.part,
                rest.measure,
                rest.staff,
                rest.voice,
                rest.onset,
                rest.onset.saturating_add(rest.duration),
                pitched.onset,
                pitched.onset.saturating_add(pitched.duration)
            ));
        }
    }

    if frozen_sounding_voice_binding_complete {
        let mut conflict_rests = BTreeSet::<(String, String, String, String, i64, i64)>::new();
        for rest in render_events.iter().filter(|event| {
            event.visible && event.is_rest && !event.is_chord && event.duration > 0
        }) {
            let Some(measure_number) = rest
                .measure
                .parse::<usize>()
                .ok()
                .filter(|value| *value > 0)
            else {
                continue;
            };
            let Some(staff_number) = rest.staff.parse::<u8>().ok() else {
                continue;
            };
            let local_end = rest.onset.saturating_add(rest.duration.max(0));
            for (span_measure, span_staff, span_start, span_end, span_voice) in
                frozen_sounding_voice_spans
            {
                if *span_measure != measure_number
                    || *span_staff != staff_number
                    || span_voice.trim() != rest.voice.trim()
                    || rest.onset >= *span_end
                    || *span_start >= local_end
                {
                    continue;
                }
                audit.frozen_same_voice_rest_overlap_pairs = audit
                    .frozen_same_voice_rest_overlap_pairs
                    .saturating_add(1);
                conflict_rests.insert((
                    rest.part.clone(),
                    rest.measure.clone(),
                    rest.staff.clone(),
                    rest.voice.clone(),
                    rest.onset,
                    rest.duration,
                ));
                audit.rows.push(format!(
                    concat!(
                        "FROZEN_SOUNDING_SAME_VOICE_REST_CONFLICT part={} measure={} staff={} voice={} ",
                        "rest={}-{} frozen={}-{}"
                    ),
                    rest.part,
                    rest.measure,
                    rest.staff,
                    rest.voice,
                    rest.onset,
                    rest.onset.saturating_add(rest.duration),
                    span_start,
                    span_end,
                ));
            }
        }
        audit.frozen_same_voice_rest_conflict_rests = conflict_rests.len();
    }

    let master_hidden_rests = master_events
        .iter()
        .filter(|event| !event.visible && event.is_rest && !event.is_chord && event.duration > 0)
        .map(|event| {
            (
                event.part.clone(),
                event.measure.clone(),
                event.staff.clone(),
                event.voice.clone(),
                event.onset,
                event.duration,
            )
        })
        .collect::<BTreeSet<_>>();
    let render_visible_rests = render_events
        .iter()
        .filter(|event| event.visible && event.is_rest && !event.is_chord && event.duration > 0)
        .map(|event| {
            (
                event.part.clone(),
                event.measure.clone(),
                event.staff.clone(),
                event.voice.clone(),
                event.onset,
                event.duration,
            )
        })
        .collect::<BTreeSet<_>>();
    for (part, measure, staff, voice, onset, duration) in
        master_hidden_rests.intersection(&render_visible_rests)
    {
        audit.master_hidden_to_render_visible_exact =
            audit.master_hidden_to_render_visible_exact.saturating_add(1);
        audit.rows.push(format!(
            concat!(
                "MASTER_HIDDEN_TO_RENDER_VISIBLE_EXACT part={} measure={} staff={} voice={} ",
                "interval={}-{}"
            ),
            part,
            measure,
            staff,
            voice,
            onset,
            onset.saturating_add(*duration)
        ));
    }

    audit.rows.sort();
    audit.rows.dedup();
    audit
}

pub(crate) fn append_staff_local_rest_ownership_audit_section(
    out: &mut String,
    master_xml: &str,
    render_xml: &str,
    frozen_sounding_voice_binding_complete: bool,
    frozen_sounding_unbound_members: usize,
    frozen_sounding_voice_spans: &[(usize, u8, i64, i64, String)],
) {
    let audit = build_staff_local_rest_ownership_audit(
        master_xml,
        render_xml,
        frozen_sounding_voice_binding_complete,
        frozen_sounding_unbound_members,
        frozen_sounding_voice_spans,
    );
    let obstruction_count = audit
        .exact_cross_voice_duplicate_groups
        .saturating_add(audit.same_lane_rest_pitch_overlap_pairs)
        .saturating_add(audit.frozen_same_voice_rest_conflict_rests);
    let verdict = if !audit.master_parse_ok || !audit.render_parse_ok {
        "UNAVAILABLE_PARSE_FAILURE"
    } else if obstruction_count > 0 {
        "REST_OWNERSHIP_OBSTRUCTION_PRESENT"
    } else if audit.master_hidden_to_render_visible_exact > 0 {
        "VISIBILITY_PROVENANCE_REVIEW"
    } else {
        "CLEAN"
    };

    let _ = writeln!(out, "ProjectionOnly: true");
    let _ = writeln!(out, "Authority: none");
    let _ = writeln!(out, "MusicMutation: nein");
    let _ = writeln!(out, "RuleChange: nein");
    let _ = writeln!(out, "LocatorRowsAreDiagnosticOnly: yes");
    let _ = writeln!(
        out,
        "ParseMaster/Render: {}/{}",
        if audit.master_parse_ok { "ok" } else { "fail" },
        if audit.render_parse_ok { "ok" } else { "fail" }
    );
    let _ = writeln!(
        out,
        "ExactCrossVoiceDuplicateVisibleRestGroups: {}",
        audit.exact_cross_voice_duplicate_groups
    );
    let _ = writeln!(
        out,
        "ExactCrossVoiceDuplicateExcessRests: {}",
        audit.exact_cross_voice_duplicate_excess_rests
    );
    let _ = writeln!(
        out,
        "SameLaneVisibleRestPitchOverlapPairs: {}",
        audit.same_lane_rest_pitch_overlap_pairs
    );
    let _ = writeln!(
        out,
        "MasterHiddenToRenderVisibleExact: {}",
        audit.master_hidden_to_render_visible_exact
    );
    let _ = writeln!(
        out,
        "FrozenSoundingVoiceBindingComplete: {}",
        if audit.frozen_sounding_voice_binding_complete {
            "yes"
        } else {
            "no"
        }
    );
    let _ = writeln!(
        out,
        "FrozenSoundingSerializedVoiceSegments: {}",
        audit.frozen_sounding_voice_spans
    );
    let _ = writeln!(
        out,
        "FrozenSoundingUnboundMembers: {}",
        audit.frozen_sounding_unbound_members
    );
    let _ = writeln!(
        out,
        "FrozenSameVoiceVisibleRestConflictRests: {}",
        audit.frozen_same_voice_rest_conflict_rests
    );
    let _ = writeln!(
        out,
        "FrozenSameVoiceVisibleRestOverlapPairs: {}",
        audit.frozen_same_voice_rest_overlap_pairs
    );
    let _ = writeln!(
        out,
        "DuplicateGroupsWithFrozenSoundingConflict: {}",
        audit.duplicate_groups_with_frozen_sounding_conflict
    );
    let _ = writeln!(
        out,
        "DuplicateGroupsAwaitingNegativeOwnership: {}",
        audit.duplicate_groups_awaiting_negative_ownership
    );
    let _ = writeln!(
        out,
        "FrozenRestOwnershipShadowPlan: positive_sounding_veto_bound; negative_rest_obligation_unbound; writer_authority_none"
    );
    let _ = writeln!(out, "Verdict: {verdict}");
    if audit.rows.is_empty() {
        let _ = writeln!(out, "Rows: none");
    } else {
        for row in audit.rows {
            let _ = writeln!(out, "Row: {row}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(body: &str) -> String {
        format!(
            "<score-partwise><part id=\"P1\"><measure number=\"2\">{body}</measure></part></score-partwise>"
        )
    }

    #[test]
    fn finds_exact_cross_voice_duplicate_visible_rest_groups() {
        let master = score(
            "<note><rest/><duration>96</duration><voice>1</voice><staff>1</staff></note>",
        );
        let render = score(concat!(
            "<note><rest/><duration>96</duration><voice>1</voice><staff>1</staff></note>",
            "<backup><duration>96</duration></backup>",
            "<note><rest/><duration>96</duration><voice>2</voice><staff>1</staff></note>"
        ));
        let audit = build_staff_local_rest_ownership_audit(&master, &render, true, 0, &[]);
        assert_eq!(audit.exact_cross_voice_duplicate_groups, 1);
        assert_eq!(audit.exact_cross_voice_duplicate_excess_rests, 1);
        assert_eq!(audit.duplicate_groups_awaiting_negative_ownership, 1);
    }

    #[test]
    fn finds_visible_rest_pitch_overlap_in_the_same_lane() {
        let master = score(
            "<note><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>",
        );
        let render = score(concat!(
            "<note><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>",
            "<backup><duration>48</duration></backup>",
            "<forward><duration>24</duration></forward>",
            "<note><pitch><step>C</step><octave>4</octave></pitch>",
            "<duration>24</duration><voice>1</voice><staff>1</staff></note>"
        ));
        let audit = build_staff_local_rest_ownership_audit(&master, &render, false, 0, &[]);
        assert_eq!(audit.same_lane_rest_pitch_overlap_pairs, 1);
    }

    #[test]
    fn frozen_sounding_truth_binds_same_voice_rest_veto_without_cross_voice_false_positive() {
        let master = score(
            "<note><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>",
        );
        let render = score(
            "<note><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>",
        );
        let spans = vec![
            (2, 1, 24, 48, "1".to_string()),
            (2, 1, 0, 48, "2".to_string()),
        ];
        let audit = build_staff_local_rest_ownership_audit(&master, &render, true, 0, &spans);
        assert_eq!(audit.frozen_same_voice_rest_conflict_rests, 1);
        assert_eq!(audit.frozen_same_voice_rest_overlap_pairs, 1);
    }

    #[test]
    fn distinguishes_exact_master_hidden_to_render_visible_promotion() {
        let master = score(concat!(
            "<note print-object=\"no\" print-spacing=\"no\"><rest/>",
            "<duration>48</duration><voice>1</voice><staff>1</staff></note>"
        ));
        let render = score(
            "<note><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>",
        );
        let audit = build_staff_local_rest_ownership_audit(&master, &render, false, 0, &[]);
        assert_eq!(audit.master_hidden_to_render_visible_exact, 1);
    }
}
