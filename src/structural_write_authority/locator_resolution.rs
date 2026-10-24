//! Opaque source-location resolution for an already authorized serialization plan.
//!
//! This module owns all access to absolute part/measure identity and source-event
//! ordinals. The writer receives only resolved byte ranges and cursor facts; it
//! cannot inspect or branch on measure, pitch, voice or staff identity.

use super::serialization_plan::{AuthorizedPlannedLane, AuthorizedSerializationPlan};
use super::{StructuralBeamState, StructuralLocator};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Debug, Clone)]
pub(super) struct ResolvedSourceNoteRecord {
    pub(super) range: Range<usize>,
    pub(super) cursor_before: i64,
    pub(super) cursor_after: i64,
    pub(super) voice: String,
    pub(super) staff: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RendererBeamMemberContiguityEvidence {
    pub(super) beam_groups_checked: usize,
    pub(super) member_pairs_checked: usize,
    pub(super) interstitial_elements: usize,
    pub(super) interstitial_directions: usize,
    pub(super) fragment_parse_failures: usize,
    pub(super) green: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RendererInterstitialElementKind {
    Direction,
    Other,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct RendererInterstitialObstacleView {
    pub(super) group_ordinal: usize,
    pub(super) pair_ordinal: usize,
    pub(super) element_ordinal: usize,
    pub(super) kind: RendererInterstitialElementKind,
    pub(super) group_start_cursor: i64,
    pub(super) group_end_cursor: i64,
    pub(super) left_cursor_after: i64,
    pub(super) right_cursor_before: i64,
    pub(super) element_cursor: i64,
    pub(super) current_offset: i64,
    pub(super) offset_explicit: bool,
    pub(super) effective_cursor: i64,
    pub(super) direction_type_element_count: usize,
    pub(super) dynamics_mark_count: usize,
    pub(super) has_sound: bool,
    pub(super) has_timing_spanner: bool,
    pub(super) has_words_or_metronome: bool,
    pub(super) has_default_x: bool,
    pub(super) staff_matches_left_member: bool,
    pub(super) staff_matches_right_member: bool,
    pub(super) payload_without_offset_fingerprint: u64,
}

#[derive(Debug, Clone, Default)]
pub(super) struct RendererInterstitialObstacleSnapshot {
    pub(super) beam_groups_checked: usize,
    pub(super) member_pairs_checked: usize,
    pub(super) fragment_parse_failures: usize,
    pub(super) obstacles: Vec<RendererInterstitialObstacleView>,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedSerializationTarget {
    records: BTreeMap<usize, ResolvedSourceNoteRecord>,
    insertion_backup_end: usize,
    part_ordinal: usize,
    measure_ordinal: usize,
    pub(super) source_fingerprint_match: bool,
    pub(super) source_events_found: usize,
    pub(super) source_event_cursor_match: bool,
}

impl ResolvedSerializationTarget {
    pub(super) fn record(&self, ordinal: usize) -> Option<&ResolvedSourceNoteRecord> {
        self.records.get(&ordinal)
    }

    pub(super) fn insertion_backup_end(&self) -> usize {
        self.insertion_backup_end
    }

    pub(super) fn raw_cursor_precheck(&self, xml: &str) -> (bool, usize) {
        target_measure_raw_cursor_precheck(xml, self.part_ordinal, self.measure_ordinal)
    }

    pub(super) fn renderer_beam_member_contiguity(
        &self,
        source_xml: &str,
        plan: &AuthorizedSerializationPlan,
    ) -> RendererBeamMemberContiguityEvidence {
        renderer_beam_member_contiguity_evidence(source_xml, &self.records, plan)
    }

    pub(super) fn renderer_interstitial_obstacle_snapshot(
        &self,
        source_xml: &str,
        plan: &AuthorizedSerializationPlan,
    ) -> RendererInterstitialObstacleSnapshot {
        renderer_interstitial_obstacle_snapshot(source_xml, &self.records, plan)
    }
}

pub(super) fn resolve_authorized_serialization_target(
    source_xml: &str,
    locator: &StructuralLocator,
    plan: &AuthorizedSerializationPlan,
) -> Option<ResolvedSerializationTarget> {
    let source_fingerprint_match =
        locator.source_fingerprint == stable_xml_snapshot_fingerprint(source_xml);
    if !source_fingerprint_match {
        return None;
    }
    let document = Document::parse(source_xml).ok()?;
    let measure = locate_measure(&document, locator.part_ordinal, locator.measure_ordinal)?;
    let records = collect_source_note_records(&document, source_xml);
    let target_records = plan
        .rows
        .iter()
        .filter_map(|row| records.get(&row.source_event_ordinal))
        .collect::<Vec<_>>();
    let source_events_found = target_records.len();
    if source_events_found != plan.rows.len()
        || target_records.iter().any(|record| {
            record.range.start < measure.range().start || record.range.end > measure.range().end
        })
    {
        return None;
    }
    let source_event_cursor_match = plan.rows.iter().all(|row| {
        records
            .get(&row.source_event_ordinal)
            .is_some_and(|record| {
                    record.cursor_before == row.original_cursor_before
                        && record.cursor_after == row.original_cursor_after
                },
            )
    });
    if !source_event_cursor_match {
        return None;
    }

    let last_main_end = plan
        .rows
        .iter()
        .filter(|row| row.planned_lane == AuthorizedPlannedLane::MainVoiceRun)
        .filter_map(|row| records.get(&row.source_event_ordinal))
        .map(|record| record.range.end)
        .max()?;
    let overlap_record = plan
        .rows
        .iter()
        .find(|row| row.planned_lane == AuthorizedPlannedLane::OverlapRestAuxiliary)
        .and_then(|row| records.get(&row.source_event_ordinal))?;
    let insertion_backup_end = measure
        .children()
        .filter(Node::is_element)
        .filter(|child| child.has_tag_name("backup"))
        .find_map(|backup| {
            let range = backup.range();
            (range.start > last_main_end && range.end < overlap_record.range.start)
                .then_some(range.end)
        })?;

    Some(ResolvedSerializationTarget {
        records,
        insertion_backup_end,
        part_ordinal: locator.part_ordinal,
        measure_ordinal: locator.measure_ordinal,
        source_fingerprint_match,
        source_events_found,
        source_event_cursor_match,
    })
}

fn renderer_beam_member_contiguity_evidence(
    source_xml: &str,
    records: &BTreeMap<usize, ResolvedSourceNoteRecord>,
    plan: &AuthorizedSerializationPlan,
) -> RendererBeamMemberContiguityEvidence {
    let groups = planned_primary_beam_groups(plan);

    let removed_ranges = plan
        .rows
        .iter()
        .filter(|row| row.planned_lane == AuthorizedPlannedLane::HiddenCarrierAuxiliary)
        .filter_map(|row| records.get(&row.source_event_ordinal))
        .map(|record| record.range.clone())
        .collect::<Vec<_>>();

    let mut evidence = RendererBeamMemberContiguityEvidence {
        beam_groups_checked: groups.len(),
        ..RendererBeamMemberContiguityEvidence::default()
    };
    for group in groups {
        for pair in group.windows(2) {
            evidence.member_pairs_checked += 1;
            let Some(left) = records.get(&pair[0]) else {
                evidence.fragment_parse_failures += 1;
                continue;
            };
            let Some(right) = records.get(&pair[1]) else {
                evidence.fragment_parse_failures += 1;
                continue;
            };
            if left.range.end > right.range.start || right.range.start > source_xml.len() {
                evidence.fragment_parse_failures += 1;
                continue;
            }
            let fragment = fragment_without_removed_ranges(
                source_xml,
                left.range.end,
                right.range.start,
                &removed_ranges,
            );
            let wrapped = format!("<root>{fragment}</root>");
            let Ok(document) = Document::parse(&wrapped) else {
                evidence.fragment_parse_failures += 1;
                continue;
            };
            for child in document.root_element().children().filter(Node::is_element) {
                evidence.interstitial_elements += 1;
                if child.has_tag_name("direction") {
                    evidence.interstitial_directions += 1;
                }
            }
        }
    }
    evidence.green = evidence.beam_groups_checked > 0
        && evidence.member_pairs_checked > 0
        && evidence.interstitial_elements == 0
        && evidence.fragment_parse_failures == 0;
    evidence
}

fn planned_primary_beam_groups(plan: &AuthorizedSerializationPlan) -> Vec<Vec<usize>> {
    let mut rows = plan
        .rows
        .iter()
        .filter(|row| row.planned_lane == AuthorizedPlannedLane::MainVoiceRun)
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.planned_xml_order);

    let mut groups = Vec::<Vec<usize>>::new();
    let mut current = Vec::<usize>::new();
    for row in rows {
        match row.planned_primary_beam {
            StructuralBeamState::Begin => {
                current.clear();
                current.push(row.source_event_ordinal);
            }
            StructuralBeamState::Continue => {
                if !current.is_empty() {
                    current.push(row.source_event_ordinal);
                }
            }
            StructuralBeamState::End => {
                if !current.is_empty() {
                    current.push(row.source_event_ordinal);
                    groups.push(std::mem::take(&mut current));
                }
            }
            StructuralBeamState::Absent => current.clear(),
        }
    }
    groups
}

fn renderer_interstitial_obstacle_snapshot(
    source_xml: &str,
    records: &BTreeMap<usize, ResolvedSourceNoteRecord>,
    plan: &AuthorizedSerializationPlan,
) -> RendererInterstitialObstacleSnapshot {
    let groups = planned_primary_beam_groups(plan);
    let removed_ranges = plan
        .rows
        .iter()
        .filter(|row| row.planned_lane == AuthorizedPlannedLane::HiddenCarrierAuxiliary)
        .filter_map(|row| records.get(&row.source_event_ordinal))
        .map(|record| record.range.clone())
        .collect::<Vec<_>>();

    let mut snapshot = RendererInterstitialObstacleSnapshot {
        beam_groups_checked: groups.len(),
        ..RendererInterstitialObstacleSnapshot::default()
    };

    for (group_ordinal, group) in groups.iter().enumerate() {
        let Some(first) = group.first().and_then(|ordinal| records.get(ordinal)) else {
            snapshot.fragment_parse_failures += 1;
            continue;
        };
        let Some(last) = group.last().and_then(|ordinal| records.get(ordinal)) else {
            snapshot.fragment_parse_failures += 1;
            continue;
        };
        for (pair_ordinal, pair) in group.windows(2).enumerate() {
            snapshot.member_pairs_checked += 1;
            let Some(left) = records.get(&pair[0]) else {
                snapshot.fragment_parse_failures += 1;
                continue;
            };
            let Some(right) = records.get(&pair[1]) else {
                snapshot.fragment_parse_failures += 1;
                continue;
            };
            if left.range.end > right.range.start || right.range.start > source_xml.len() {
                snapshot.fragment_parse_failures += 1;
                continue;
            }
            let fragment = fragment_without_removed_ranges(
                source_xml,
                left.range.end,
                right.range.start,
                &removed_ranges,
            );
            let wrapped = format!("<root>{fragment}</root>");
            let Ok(document) = Document::parse(&wrapped) else {
                snapshot.fragment_parse_failures += 1;
                continue;
            };
            for (element_ordinal, child) in document
                .root_element()
                .children()
                .filter(Node::is_element)
                .enumerate()
            {
                let kind = if child.has_tag_name("direction") {
                    RendererInterstitialElementKind::Direction
                } else {
                    RendererInterstitialElementKind::Other
                };
                let current_offset = if child.has_tag_name("direction") {
                    direct_child_i64(child, "offset").unwrap_or(0)
                } else {
                    0
                };
                let offset_explicit = child.has_tag_name("direction")
                    && child
                        .children()
                        .any(|node| node.is_element() && node.has_tag_name("offset"));
                let direction_type_element_count = child
                    .children()
                    .filter(|node| node.is_element() && node.has_tag_name("direction-type"))
                    .flat_map(|node| node.children().filter(Node::is_element))
                    .count();
                let dynamics_mark_count = child
                    .descendants()
                    .filter(|node| {
                        node.is_element()
                            && node
                                .parent()
                                .is_some_and(|parent| {
                                        parent.is_element() && parent.has_tag_name("dynamics")
                                    },
                                )
                    })
                    .count();
                let has_sound = child
                    .children()
                    .any(|node| node.is_element() && node.has_tag_name("sound"));
                let has_timing_spanner = child.descendants().any(|node| {
                    node.is_element()
                        && matches!(
                            node.tag_name().name(),
                            "wedge" | "octave-shift" | "pedal" | "dashes" | "bracket"
                        )
                });
                let has_words_or_metronome = child.descendants().any(|node| {
                    node.is_element()
                        && matches!(
                            node.tag_name().name(),
                            "words"
                                | "metronome"
                                | "rehearsal"
                                | "segno"
                                | "coda"
                                | "other-direction"
                        )
                });
                let has_default_x = child
                    .descendants()
                    .any(|node| node.is_element() && node.attribute("default-x").is_some());
                let direction_staff = if child.has_tag_name("direction") {
                    direct_child_text(child, "staff")
                } else {
                    None
                };
                let raw = wrapped.get(child.range()).unwrap_or_default();
                let payload_without_offset_fingerprint = if child.has_tag_name("direction") {
                    stable_xml_snapshot_fingerprint(&direction_without_offset(raw))
                } else {
                    0
                };
                snapshot.obstacles.push(RendererInterstitialObstacleView {
                    group_ordinal,
                    pair_ordinal,
                    element_ordinal,
                    kind,
                    group_start_cursor: first.cursor_before,
                    group_end_cursor: last.cursor_after,
                    left_cursor_after: left.cursor_after,
                    right_cursor_before: right.cursor_before,
                    element_cursor: left.cursor_after,
                    current_offset,
                    offset_explicit,
                    effective_cursor: left.cursor_after.saturating_add(current_offset),
                    direction_type_element_count,
                    dynamics_mark_count,
                    has_sound,
                    has_timing_spanner,
                    has_words_or_metronome,
                    has_default_x,
                    staff_matches_left_member: direction_staff
                        .as_deref()
                        .is_some_and(|staff| staff == left.staff.as_str()),
                    staff_matches_right_member: direction_staff
                        .as_deref()
                        .is_some_and(|staff| staff == right.staff.as_str()),
                    payload_without_offset_fingerprint,
                });
            }
        }
    }
    snapshot
}

pub(super) fn direction_without_offset(direction_xml: &str) -> String {
    let mut out = direction_xml.to_string();
    while let Some(start) = out.find("<offset") {
        let Some(open_end_rel) = out[start..].find('>') else {
            break;
        };
        let content_start = start + open_end_rel + 1;
        let Some(close_rel) = out[content_start..].find("</offset>") else {
            break;
        };
        let end = content_start + close_rel + "</offset>".len();
        out.replace_range(start..end, "");
    }
    out
}

fn fragment_without_removed_ranges(
    source_xml: &str,
    start: usize,
    end: usize,
    removed_ranges: &[Range<usize>],
) -> String {
    let mut ranges = removed_ranges
        .iter()
        .filter(|range| range.start >= start && range.end <= end)
        .cloned()
        .collect::<Vec<_>>();
    ranges.sort_by_key(|range| (range.start, range.end));

    let mut fragment = String::new();
    let mut cursor = start;
    for range in ranges {
        if range.start < cursor || range.end > end {
            continue;
        }
        if let Some(text) = source_xml.get(cursor..range.start) {
            fragment.push_str(text);
        }
        cursor = range.end;
    }
    if let Some(text) = source_xml.get(cursor..end) {
        fragment.push_str(text);
    }
    fragment
}

fn collect_source_note_records(
    document: &Document<'_>,
    source_xml: &str,
) -> BTreeMap<usize, ResolvedSourceNoteRecord> {
    let mut out = BTreeMap::new();
    let mut ordinal = 0_usize;
    for (_part_ordinal, _measure_ordinal, measure) in all_measures(document) {
        let mut cursor = 0_i64;
        for child in measure.children().filter(Node::is_element) {
            if child.has_tag_name("backup") {
                cursor -= direct_child_i64(child, "duration").unwrap_or(0);
                continue;
            }
            if child.has_tag_name("forward") {
                cursor += direct_child_i64(child, "duration").unwrap_or(0);
                continue;
            }
            if !child.has_tag_name("note") {
                continue;
            }
            let cursor_before = cursor;
            let duration = direct_child_i64(child, "duration").unwrap_or(0).max(0);
            let chord = child
                .children()
                .any(|node| node.is_element() && node.has_tag_name("chord"));
            if !chord {
                cursor += duration;
            }
            let range = child.range();
            if range.end <= source_xml.len() {
                out.insert(
                    ordinal,
                    ResolvedSourceNoteRecord {
                        range,
                        cursor_before,
                        cursor_after: cursor,
                        voice: direct_child_text(child, "voice").unwrap_or_else(|| "1".to_string()),
                        staff: direct_child_text(child, "staff").unwrap_or_else(|| "1".to_string()),
                    },
                );
            }
            ordinal += 1;
        }
    }
    out
}

fn all_measures<'a, 'input>(
    document: &'a Document<'input>,
) -> Vec<(usize, usize, Node<'a, 'input>)> {
    let root = document.root_element();
    if root.has_tag_name("measure") {
        return vec![(0, 0, root)];
    }
    let parts = root
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
        .collect::<Vec<_>>();
    if parts.is_empty() {
        return root
            .descendants()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
            .enumerate()
            .map(|(measure_ordinal, measure)| (0, measure_ordinal, measure))
            .collect();
    }
    let mut out = Vec::new();
    for (part_ordinal, part) in parts.into_iter().enumerate() {
        for (measure_ordinal, measure) in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
            .enumerate()
        {
            out.push((part_ordinal, measure_ordinal, measure));
        }
    }
    out
}

fn locate_measure<'a, 'input>(
    document: &'a Document<'input>,
    part_ordinal: usize,
    measure_ordinal: usize,
) -> Option<Node<'a, 'input>> {
    all_measures(document)
        .into_iter()
        .find(|(part, measure, _)| *part == part_ordinal && *measure == measure_ordinal)
        .map(|(_, _, node)| node)
}

fn target_measure_raw_cursor_precheck(
    xml: &str,
    part_ordinal: usize,
    measure_ordinal: usize,
) -> (bool, usize) {
    let Ok(document) = Document::parse(xml) else {
        return (false, 0);
    };
    let Some(measure) = locate_measure(&document, part_ordinal, measure_ordinal) else {
        return (false, 0);
    };
    let Some(expected_duration) =
        inherited_measure_duration(&document, part_ordinal, measure_ordinal)
    else {
        return (false, 0);
    };
    let mut cursor = 0_i64;
    let mut cursor_max = 0_i64;
    let mut partial_backups = 0_usize;
    let mut post_expected_events = 0_usize;
    let mut hard = false;
    for child in measure.children().filter(Node::is_element) {
        let cursor_before = cursor;
        if child.has_tag_name("backup") {
            let duration = direct_child_i64(child, "duration").unwrap_or(0);
            cursor -= duration;
            if cursor != 0 {
                partial_backups += 1;
            }
            if duration <= 0 || cursor < 0 {
                hard = true;
            }
        } else if child.has_tag_name("forward") {
            let duration = direct_child_i64(child, "duration").unwrap_or(0);
            cursor += duration;
            if duration <= 0 {
                hard = true;
            }
            if cursor > expected_duration || cursor_before >= expected_duration {
                post_expected_events += 1;
            }
        } else if child.has_tag_name("note") {
            let chord = child
                .children()
                .any(|node| node.is_element() && node.has_tag_name("chord"));
            if !chord {
                let duration = direct_child_i64(child, "duration").unwrap_or(0).max(0);
                cursor += duration;
                if cursor > expected_duration || cursor_before >= expected_duration {
                    post_expected_events += 1;
                }
            }
        }
        cursor_max = cursor_max.max(cursor);
    }
    (
        !hard
            && partial_backups == 0
            && post_expected_events == 0
            && cursor_max <= expected_duration
            && cursor == expected_duration,
        partial_backups,
    )
}

fn inherited_measure_duration(
    document: &Document<'_>,
    target_part_ordinal: usize,
    target_measure_ordinal: usize,
) -> Option<i64> {
    let mut divisions = 1_i64;
    let mut beats = 4_i64;
    let mut beat_type = 4_i64;
    for (part_ordinal, measure_ordinal, measure) in all_measures(document) {
        if part_ordinal != target_part_ordinal {
            continue;
        }
        if let Some(value) = measure
            .descendants()
            .find(|node| node.is_element() && node.has_tag_name("divisions"))
            .and_then(|node| node.text())
            .and_then(|text| text.trim().parse::<i64>().ok())
        {
            divisions = value.max(1);
        }
        if let Some(value) = measure
            .descendants()
            .find(|node| node.is_element() && node.has_tag_name("beats"))
            .and_then(|node| node.text())
            .and_then(|text| text.trim().parse::<i64>().ok())
        {
            beats = value.max(1);
        }
        if let Some(value) = measure
            .descendants()
            .find(|node| node.is_element() && node.has_tag_name("beat-type"))
            .and_then(|node| node.text())
            .and_then(|text| text.trim().parse::<i64>().ok())
        {
            beat_type = value.max(1);
        }
        if measure_ordinal == target_measure_ordinal {
            return Some(divisions.saturating_mul(beats).saturating_mul(4) / beat_type);
        }
    }
    None
}

pub(super) fn direct_child_text(node: Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(ToOwned::to_owned)
}

pub(super) fn direct_child_i64(node: Node<'_, '_>, name: &str) -> Option<i64> {
    direct_child_text(node, name)?.parse().ok()
}

pub(super) fn stable_xml_snapshot_fingerprint(text: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}
