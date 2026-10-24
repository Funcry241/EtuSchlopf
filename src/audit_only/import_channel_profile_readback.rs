//! Audit-only readback of the partwide import-channel profile of the final
//! MusicXML artifacts.
//!
//! The independently documented MuseScore 4.7.4 MusicXML import (first pass:
//! `allocateStaves`, `allocateVoices`, `VoiceOverlapDetector`; second pass:
//! `determineStaffMoveVoice`) reserves at most four voice slots per staff for the
//! whole part. Channels are registered by positive-duration non-chord heads and
//! rests; `print-spacing="no"` material only advances the cursor. A channel whose
//! occurrences on two staves overlap in time within one measure becomes an
//! overlapping channel for the whole part and is reserved separately on every
//! staff, after all ordinary channels. Occurrences whose channel receives no slot
//! are discarded by the consumer.
//!
//! This module recomputes that reservation from the persisted bytes and reports
//! slot use, unreserved channels and the occurrences the consumer would discard.
//! It is evidence for the frozen partwide renderer channel-capacity error class
//! (`docs/selector_derivation_contract_partwide_renderer_channel_capacity.md`).
//! It owns no write path, chooses no channel, staff, hand or voice, and a clean
//! readback is no proof of musical completeness.

use roxmltree::{Document, Node};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

/// Voice slots per staff in the documented consumer profile.
const PROFILE_SLOTS_PER_STAFF: usize = 4;
/// Staves per part handled by the documented consumer profile.
const PROFILE_MAX_STAVES: usize = 4;
const PROFILE_NAME: &str = "musescore_4_7_4_musicxml_pass1_partwide_v1";

#[derive(Debug, Clone, Default)]
struct Channel {
    heads_and_rests: [usize; PROFILE_MAX_STAVES],
    overlapping: bool,
    first_overlap_measure: Option<String>,
    ordinary_staff: Option<Option<usize>>,
    overlapping_staff_slots: [Option<bool>; PROFILE_MAX_STAVES],
}

impl Channel {
    fn total(&self) -> usize {
        self.heads_and_rests.iter().sum()
    }

    fn preferred_staff(&self) -> usize {
        let mut best = 0usize;
        let mut best_count = 0usize;
        for (staff, count) in self.heads_and_rests.iter().enumerate() {
            if *count > best_count {
                best_count = *count;
                best = staff;
            }
        }
        best
    }

    fn mapped_on(&self, staff: usize) -> bool {
        if self.overlapping {
            return self
                .overlapping_staff_slots
                .get(staff)
                .copied()
                .flatten()
                .unwrap_or(false);
        }
        matches!(self.ordinary_staff, Some(Some(_)))
    }
}

#[derive(Debug, Clone)]
struct Occurrence {
    channel: i64,
    staff: usize,
    registering: bool,
    pitched: bool,
    visible: bool,
}

#[derive(Debug, Clone, Default)]
struct PartProfile {
    channels: BTreeMap<i64, Channel>,
    occurrences: Vec<Occurrence>,
    staves_seen: usize,
    peak_concurrent: [usize; PROFILE_MAX_STAVES],
    out_of_profile_staff_occurrences: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ProfileAudit {
    source_read: bool,
    parse_green: bool,
    parts: usize,
    channels: usize,
    overlapping_channels: Vec<String>,
    slot_use: Vec<String>,
    unreserved: Vec<String>,
    lost_heads_and_rests: usize,
    lost_pitched: usize,
    lost_visible_pitched: usize,
    lost_unregistered_members: usize,
    peak_concurrent: Vec<String>,
    out_of_profile_staff_occurrences: usize,
}

fn channel_key(text: Option<&str>) -> i64 {
    let value = text.map(str::trim).unwrap_or("");
    if value.is_empty() {
        return 1;
    }
    match value.parse::<i64>() {
        Ok(number) => number,
        Err(_) => value.chars().next().map_or(0, |character| i64::from(u32::from(character))),
    }
}

fn child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|item| item.has_tag_name(name))
}

fn child_text<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    child(node, name).and_then(|item| item.text())
}

fn child_i64(node: Node<'_, '_>, name: &str) -> Option<i64> {
    child_text(node, name).and_then(|text| text.trim().parse::<i64>().ok())
}

fn overlaps(left: (i64, i64), right: (i64, i64)) -> bool {
    !(left.0 >= right.1 || left.1 <= right.0)
}

fn peak_concurrency(intervals: &BTreeMap<i64, Vec<(i64, i64)>>) -> usize {
    let mut points = intervals
        .values()
        .flatten()
        .map(|interval| interval.0)
        .collect::<Vec<_>>();
    points.sort_unstable();
    points.dedup();
    let mut peak = 0usize;
    for point in points {
        let active = intervals
            .values()
            .filter(|list| list.iter().any(|interval| interval.0 <= point && point < interval.1))
            .count();
        peak = peak.max(active);
    }
    peak
}

fn read_measure(profile: &mut PartProfile, measure: Node<'_, '_>, label: &str, divisions: &mut i64) {
    // Cursor in units of 1 / (4 * divisions); rescaled if divisions change.
    let mut cursor = 0_i64;
    let mut last_head_start = 0_i64;
    let mut per_channel: BTreeMap<i64, [Vec<(i64, i64)>; PROFILE_MAX_STAVES]> = BTreeMap::new();
    for element in measure.children().filter(Node::is_element) {
        match element.tag_name().name() {
            "attributes" => {
                if let Some(value) = child_i64(element, "divisions").filter(|value| *value > 0) {
                    if value != *divisions {
                        cursor = cursor * value / *divisions;
                    }
                    *divisions = value;
                }
            }
            "backup" => {
                let amount = child_i64(element, "duration").unwrap_or(0).max(0);
                cursor = (cursor - amount).max(0);
            }
            "forward" => {
                cursor += child_i64(element, "duration").unwrap_or(0).max(0);
            }
            "note" => {
                let duration = child_i64(element, "duration").unwrap_or(0).max(0);
                let chord = child(element, "chord").is_some();
                let grace = child(element, "grace").is_some();
                let cue = child(element, "cue").is_some();
                let rest = child(element, "rest").is_some();
                let cursor_only = element.attribute("print-spacing") == Some("no");
                if cursor_only {
                    if !chord && !grace {
                        cursor += duration;
                    }
                    continue;
                }
                if cue {
                    continue;
                }
                let staff_number = child_i64(element, "staff").unwrap_or(1).max(1);
                let staff = usize::try_from(staff_number - 1).unwrap_or(0);
                profile.staves_seen = profile.staves_seen.max(staff + 1);
                let channel = channel_key(child_text(element, "voice"));
                let start = if chord { last_head_start } else { cursor };
                let registering = !chord && !grace && duration > 0;
                if staff >= PROFILE_MAX_STAVES {
                    profile.out_of_profile_staff_occurrences += 1;
                } else if registering {
                    let entry = profile.channels.entry(channel).or_default();
                    entry.heads_and_rests[staff] += 1;
                    per_channel.entry(channel).or_default()[staff].push((start, start + duration));
                }
                profile.occurrences.push(Occurrence {
                    channel,
                    staff,
                    registering,
                    pitched: !rest,
                    visible: element.attribute("print-object") != Some("no"),
                });
                if !chord && !grace {
                    last_head_start = cursor;
                    cursor += duration;
                }
            }
            _ => {}
        }
    }
    for (channel, staves) in &per_channel {
        let mut crossing = false;
        for left in 0..PROFILE_MAX_STAVES {
            for right in (left + 1)..PROFILE_MAX_STAVES {
                crossing |= staves[left]
                    .iter()
                    .any(|a| staves[right].iter().any(|b| overlaps(*a, *b)));
            }
        }
        if crossing && let Some(entry) = profile.channels.get_mut(channel) {
            if entry.first_overlap_measure.is_none() {
                entry.first_overlap_measure = Some(label.to_string());
            }
            entry.overlapping = true;
        }
    }
    for staff in 0..PROFILE_MAX_STAVES {
        let lists = per_channel
            .iter()
            .map(|(channel, staves)| (*channel, staves[staff].clone()))
            .collect::<BTreeMap<_, _>>();
        profile.peak_concurrent[staff] = profile.peak_concurrent[staff].max(peak_concurrency(&lists));
    }
}

/// Mirrors `allocateStaves` and `allocateVoices` of the documented profile.
fn reserve_slots(profile: &mut PartProfile) {
    let mut used = [0usize; PROFILE_MAX_STAVES];
    let keys = profile.channels.keys().copied().collect::<Vec<_>>();
    for _ in 0..keys.len() {
        let mut best: Option<(i64, usize)> = None;
        for key in &keys {
            let channel = &profile.channels[key];
            if !channel.overlapping
                && channel.ordinary_staff.is_none()
                && channel.total() > best.map_or(0, |(_, count)| count)
            {
                best = Some((*key, channel.total()));
            }
        }
        let Some((key, _)) = best else { continue };
        if key <= 0 {
            continue;
        }
        let channel = profile.channels.get_mut(&key).expect("channel key present");
        let staff = channel.preferred_staff();
        if used[staff] < PROFILE_SLOTS_PER_STAFF {
            channel.ordinary_staff = Some(Some(staff));
            used[staff] += 1;
        }
        // VoiceDesc::setStaff(-2) ignores the negative argument. The first
        // overflowing ordinary channel stays eligible and is selected again
        // in every remaining iteration, starving smaller channels on any staff.
    }
    for (staff, used_slots) in used.iter_mut().enumerate() {
        for _ in 0..keys.len() {
            let mut best: Option<(i64, usize)> = None;
            for key in &keys {
                let channel = &profile.channels[key];
                let count = channel.heads_and_rests[staff];
                if channel.overlapping
                    && channel.overlapping_staff_slots[staff].is_none()
                    && count > best.map_or(0, |(_, value)| value)
                {
                    best = Some((*key, count));
                }
            }
            let Some((key, _)) = best else { continue };
            if key <= 0 {
                continue;
            }
            let channel = profile.channels.get_mut(&key).expect("channel key present");
            if *used_slots < PROFILE_SLOTS_PER_STAFF {
                channel.overlapping_staff_slots[staff] = Some(true);
                *used_slots += 1;
            } else {
                channel.overlapping_staff_slots[staff] = Some(false);
            }
        }
    }
}

fn inspect_xml(xml: &str) -> ProfileAudit {
    let mut audit = ProfileAudit {
        source_read: true,
        ..ProfileAudit::default()
    };
    let Ok(document) = Document::parse(xml) else {
        return audit;
    };
    audit.parse_green = true;
    for (part_index, part) in document
        .descendants()
        .filter(|node| node.has_tag_name("part"))
        .enumerate()
    {
        audit.parts += 1;
        let mut profile = PartProfile::default();
        let mut divisions = 1_i64;
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.has_tag_name("measure"))
            .enumerate()
        {
            let label = measure
                .attribute("number")
                .map_or_else(|| (measure_index + 1).to_string(), str::to_string);
            read_measure(&mut profile, measure, &label, &mut divisions);
        }
        reserve_slots(&mut profile);
        let part_ordinal = part_index + 1;
        audit.channels += profile.channels.len();
        audit.out_of_profile_staff_occurrences += profile.out_of_profile_staff_occurrences;
        let mut used = [0usize; PROFILE_MAX_STAVES];
        for (key, channel) in &profile.channels {
            if channel.overlapping {
                audit.overlapping_channels.push(format!(
                    "P{part_ordinal}:v{key}@m{}",
                    channel.first_overlap_measure.as_deref().unwrap_or("?")
                ));
                for (staff, (slot, used_slots)) in channel
                    .overlapping_staff_slots
                    .iter()
                    .zip(used.iter_mut())
                    .enumerate()
                {
                    match slot {
                        Some(true) => *used_slots += 1,
                        Some(false) => audit
                            .unreserved
                            .push(format!("P{part_ordinal}:v{key}@St{}", staff + 1)),
                        None => {}
                    }
                }
            } else {
                match channel.ordinary_staff {
                    Some(Some(staff)) => used[staff] += 1,
                    _ => audit.unreserved.push(format!(
                        "P{part_ordinal}:v{key}@St{}",
                        channel.preferred_staff() + 1
                    )),
                }
            }
        }
        let staves = profile.staves_seen.clamp(1, PROFILE_MAX_STAVES);
        for (staff, (used_slots, peak)) in used
            .iter()
            .zip(profile.peak_concurrent.iter())
            .enumerate()
            .take(staves)
        {
            audit
                .slot_use
                .push(format!("P{part_ordinal}St{}={used_slots}/{PROFILE_SLOTS_PER_STAFF}", staff + 1));
            audit
                .peak_concurrent
                .push(format!("P{part_ordinal}St{}={peak}", staff + 1));
        }
        for occurrence in &profile.occurrences {
            let mapped = occurrence.staff < PROFILE_MAX_STAVES
                && profile
                    .channels
                    .get(&occurrence.channel)
                    .is_some_and(|channel| channel.mapped_on(occurrence.staff));
            if mapped {
                continue;
            }
            if !profile.channels.contains_key(&occurrence.channel) {
                audit.lost_unregistered_members += 1;
            }
            if occurrence.registering {
                audit.lost_heads_and_rests += 1;
            }
            if occurrence.pitched {
                audit.lost_pitched += 1;
                if occurrence.visible {
                    audit.lost_visible_pitched += 1;
                }
            }
        }
    }
    audit
}

/// Pure partwide consumer-profile proof for a proposed in-memory voice image.
/// Every registering head/rest and unregistered chord resource is re-evaluated.
pub(crate) fn partwide_import_channels_admissible(xml: &str) -> bool {
    profile_admissible(&inspect_xml(xml))
}

// One predicate for the consumer check and the report status; the report must never
// call a profile admissible that the consumer check rejects.
fn profile_admissible(audit: &ProfileAudit) -> bool {
    audit.parse_green && audit.parts > 0 && audit.unreserved.is_empty()
        && audit.lost_heads_and_rests == 0 && audit.lost_pitched == 0
        && audit.lost_unregistered_members == 0
        && audit.out_of_profile_staff_occurrences == 0
}

fn profile_status(audit: &ProfileAudit) -> &'static str {
    if !audit.parse_green || audit.parts == 0 {
        "unknown"
    } else if profile_admissible(audit) {
        "admissible"
    } else {
        "exhausted"
    }
}

fn inspect_path(path: &Path) -> ProfileAudit {
    let Ok(xml) = fs::read_to_string(path) else {
        return ProfileAudit::default();
    };
    inspect_xml(&xml)
}

fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

fn list(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(",")
    }
}

fn append_source(out: &mut String, label: &str, audit: &ProfileAudit) {
    let status = profile_status(audit);
    let _ = writeln!(
        out,
        "ImportChannelProfileReadback: Source {label}; Profile {PROFILE_NAME}; SourceRead {}; ParseGreen {}; Parts {}; Channels {}; SlotUse {}; OverlappingChannels {}; UnreservedChannels {}; LostHeadsAndRests {}; LostPitchedOccurrences {}; LostVisiblePitchedOccurrences {}; LostUnregisteredMembers {}; OutOfProfileStaffOccurrences {}; PeakConcurrentChannels {}; ProfileStatus {status}.",
        yes_no(audit.source_read),
        yes_no(audit.parse_green),
        audit.parts,
        audit.channels,
        list(&audit.slot_use),
        list(&audit.overlapping_channels),
        list(&audit.unreserved),
        audit.lost_heads_and_rests,
        audit.lost_pitched,
        audit.lost_visible_pitched,
        audit.lost_unregistered_members,
        audit.out_of_profile_staff_occurrences,
        list(&audit.peak_concurrent),
    );
}

pub(crate) fn append_report_section(out: &mut String, master: Option<&Path>, render: Option<&Path>) {
    let (Some(master), Some(render)) = (master, render) else {
        return;
    };
    out.push_str("IMPORT-CHANNEL-PROFILE-READBACK\n");
    out.push_str("===============================\n");
    out.push_str("ChangeClass: AuditOnly; EvidenceSource final Master- and Render-MusicXML; MusicMutation nein; MasterApply nein; WriteAuthority none; ErrorClass partwide_renderer_channel_capacity frozen in r1326.\n");
    append_source(out, "master", &inspect_path(master));
    append_source(out, "render", &inspect_path(render));
    out.push_str("Reading: the profile reserves at most four slots per staff for the whole part. Ordinary channels are reserved on their preferred staff in descending head/rest count; the first ordinary overflow remains unassigned and prevents lower-priority ordinary channels on any staff from being visited; a channel whose occurrences on two staves overlap in time within one measure becomes overlapping for the whole part and is reserved per staff only after all ordinary channels. UnreservedChannels lists channels without a slot; the consumer discards their occurrences. LostPitchedOccurrences counts pitched heads and chord members on unreserved or unregistered channels; LostVisiblePitchedOccurrences excludes print-object no. ProfileStatus exhausted means the consumer would drop material, with or without a fatal import error. PeakConcurrentChannels is the largest number of channels with simultaneously active registering occurrences on one staff in any measure.\n");
    out.push_str("InterpretationBoundary: readback only; recomputes a documented consumer profile from persisted bytes, moves no note, chooses no channel, staff, hand or voice, owns no MusicXML write path and is no proof of musical completeness.\n\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(staff: u8, voice: &str, duration: u32, extra: &str) -> String {
        format!(
            "<note{extra}><pitch><step>C</step><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>"
        )
    }

    fn rest(staff: u8, voice: &str, duration: u32) -> String {
        format!("<note><rest/><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>")
    }

    fn chord_member(staff: u8, voice: &str, duration: u32) -> String {
        format!(
            "<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>"
        )
    }

    fn backup(duration: u32) -> String {
        format!("<backup><duration>{duration}</duration></backup>")
    }

    fn score(measures: &[String], divisions: u32) -> String {
        let mut body = String::new();
        for (index, content) in measures.iter().enumerate() {
            let attributes = if index == 0 {
                format!("<attributes><divisions>{divisions}</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>")
            } else {
                String::new()
            };
            body.push_str(&format!("<measure>{attributes}{content}</measure>"));
        }
        format!("<score-partwise><part id=\"P1\">{body}</part></score-partwise>")
    }

    /// Synthetic counterexample of the frozen class: four ordinary upper
    /// channels fill the upper slots; one channel crosses with a lower rest
    /// that overlaps its upper head and becomes overlapping.
    fn exhausted_score(scale: u32, label_map: impl Fn(u32) -> String) -> String {
        let q = scale;
        let first = [
            note(1, &label_map(1), q, ""),
            chord_member(1, &label_map(1), q),
            rest(1, &label_map(1), 3 * q),
            backup(4 * q),
            note(2, &label_map(2), 4 * q, ""),
            backup(4 * q),
            rest(2, &label_map(1), q),
        ]
        .concat();
        let mut second = note(2, &label_map(2), 4 * q, "");
        for (index, label) in [3_u32, 5, 7, 9].into_iter().enumerate() {
            let before = q * u32::try_from(index).expect("small index");
            let after = 3 * q - before;
            second.push_str(&backup(4 * q));
            if before > 0 {
                second.push_str(&format!("<forward><duration>{before}</duration></forward>"));
            }
            second.push_str(&note(1, &label_map(label), q, ""));
            if after > 0 {
                second.push_str(&format!("<forward><duration>{after}</duration></forward>"));
            }
        }
        score(&[first, second], scale)
    }

    #[test]
    fn crossing_rest_makes_channel_overlapping_and_unreserved() {
        let audit = inspect_xml(&exhausted_score(24, |value| value.to_string()));
        assert!(audit.parse_green);
        assert_eq!(audit.overlapping_channels.len(), 1);
        assert!(audit.overlapping_channels[0].starts_with("P1:v1@"));
        assert!(audit.unreserved.iter().any(|item| item == "P1:v1@St1"));
        assert!(audit.lost_pitched >= 2);
        assert_eq!(audit.lost_visible_pitched, audit.lost_pitched);
    }

    #[test]
    fn label_renaming_and_duration_scaling_preserve_the_readback() {
        let plain = inspect_xml(&exhausted_score(24, |value| value.to_string()));
        let renamed = inspect_xml(&exhausted_score(37, |value| (17 + 5 * value).to_string()));
        assert_eq!(plain.lost_pitched, renamed.lost_pitched);
        assert_eq!(plain.lost_heads_and_rests, renamed.lost_heads_and_rests);
        assert_eq!(plain.overlapping_channels.len(), renamed.overlapping_channels.len());
        assert_eq!(plain.unreserved.len(), renamed.unreserved.len());
    }

    #[test]
    fn same_material_without_crossing_rest_is_admissible() {
        let q = 24;
        let first = [
            note(1, "1", q, ""),
            chord_member(1, "1", q),
            rest(1, "1", 3 * q),
            backup(4 * q),
            note(2, "2", 4 * q, ""),
        ]
        .concat();
        let second = [note(1, "1", 4 * q, ""), backup(4 * q), note(2, "2", 4 * q, "")].concat();
        let audit = inspect_xml(&score(&[first, second], q));
        assert!(audit.overlapping_channels.is_empty());
        assert!(audit.unreserved.is_empty());
        assert_eq!(audit.lost_pitched, 0);
        assert_eq!(audit.lost_heads_and_rests, 0);
    }

    #[test]
    fn cursor_only_material_registers_no_channel() {
        let q = 24;
        let content = [
            note(1, "1", 4 * q, ""),
            backup(4 * q),
            note(1, "9", 4 * q, " print-spacing=\"no\" print-object=\"no\""),
            backup(4 * q),
            note(2, "2", 4 * q, ""),
        ]
        .concat();
        let audit = inspect_xml(&score(&[content], q));
        assert_eq!(audit.channels, 2);
        assert_eq!(audit.lost_pitched, 0);
    }

    #[test]
    fn fifth_ordinary_channel_on_one_staff_is_unreserved() {
        let q = 60;
        let mut content = String::new();
        for (index, label) in ["1", "3", "5", "7", "9"].iter().enumerate() {
            if index > 0 {
                content.push_str(&backup(4 * q));
            }
            let duration = 4 * q;
            let repeat = 5 - u32::try_from(index).expect("small index");
            for _ in 0..repeat {
                content.push_str(&note(1, label, duration / repeat, ""));
            }
        }
        content.push_str(&backup(4 * q));
        content.push_str(&note(2, "2", 4 * q, ""));
        let audit = inspect_xml(&score(&[content], q));
        assert_eq!(audit.unreserved, vec!["P1:v9@St1".to_string()]);
        assert_eq!(audit.lost_pitched, 1);
        assert_eq!(audit.lost_heads_and_rests, 1);
    }

    #[test]
    fn unregistered_chord_member_channel_is_counted_separately() {
        let q = 24;
        let content = [
            note(1, "1", 4 * q, ""),
            chord_member(1, "4", 4 * q),
            backup(4 * q),
            note(2, "2", 4 * q, ""),
        ]
        .concat();
        let audit = inspect_xml(&score(&[content], q));
        assert_eq!(audit.lost_unregistered_members, 1);
        assert_eq!(audit.lost_pitched, 1);
        assert!(audit.unreserved.is_empty());
        // Material is lost, so the report status agrees with the consumer check.
        assert_eq!(profile_status(&audit), "exhausted");
        let mut out = String::new();
        append_source(&mut out, "probe", &audit);
        assert!(out.contains("ProfileStatus exhausted."));
    }

    #[test]
    fn report_status_matches_the_consumer_check() {
        let q = 24;
        let clean = score(&[[note(1, "1", 4 * q, ""), backup(4 * q), note(2, "2", 4 * q, "")].concat()], q);
        let clean_audit = inspect_xml(&clean);
        assert!(partwide_import_channels_admissible(&clean));
        assert_eq!(profile_status(&clean_audit), "admissible");
        let unread = ProfileAudit::default();
        assert_eq!(profile_status(&unread), "unknown");
        let no_parts = inspect_xml("<score-partwise version=\"4.0\"></score-partwise>");
        assert!(!partwide_import_channels_admissible("<score-partwise version=\"4.0\"></score-partwise>"));
        assert_ne!(profile_status(&no_parts), "admissible");
        assert!(!partwide_import_channels_admissible("<score-partwise"));
        assert_eq!(profile_status(&inspect_xml("<score-partwise")), "unknown");
    }


    fn ordinary_priority_score(staff: u8, scale: u32, rename: impl Fn(u32) -> String,
        rows: &[(bool, u32, u32)]) -> String {
        // Every channel spans one complete synthetic measure. Counts determine
        // the consumer priority; channel names never choose a musical role.
        let whole = 1680 * scale;
        let mut content = String::new();
        for (index, &(own_staff, label, count)) in rows.iter().enumerate() {
            if index > 0 { content.push_str(&backup(whole)); }
            assert_eq!(whole % count, 0);
            for _ in 0..count {
                content.push_str(&note(if own_staff { staff } else { 3 - staff },
                    &rename(label), whole / count, ""));
            }
        }
        score(&[content], whole / 4)
    }

    #[test]
    fn ordinary_overflow_starves_smaller_channels_on_both_staves() {
        for staff in [1, 2] {
            for scale in [1, 11] {
                for shift in [0, 37] {
                    let xml = ordinary_priority_score(staff, scale,
                        |label| (5 * label + shift).to_string(),
                        &[(true, 1, 8), (true, 3, 7), (true, 5, 6),
                          (true, 7, 5), (true, 9, 4), (false, 2, 3), (false, 4, 2)]);
                    let audit = inspect_xml(&xml);
                    assert!(audit.parse_green);
                    assert_eq!(audit.unreserved.len(), 3);
                    assert_eq!(audit.lost_heads_and_rests, 9);
                    assert_eq!(audit.lost_pitched, 9);
                    assert_eq!(audit.lost_visible_pitched, 9);
                    assert_eq!(audit.slot_use, vec![
                        format!("P1St1={}/4", if staff == 1 { 4 } else { 0 }),
                        format!("P1St2={}/4", if staff == 2 { 4 } else { 0 }),
                    ]);
                }
            }
        }
    }

    #[test]
    fn equal_priority_key_is_visited_before_the_first_overflow() {
        let xml = ordinary_priority_score(1, 1, |label| label.to_string(),
            &[(true, 1, 8), (true, 3, 7), (true, 5, 6), (true, 7, 5),
              (true, 9, 4), (false, 2, 4), (false, 4, 3)]);
        let audit = inspect_xml(&xml);
        assert_eq!(audit.unreserved, vec!["P1:v4@St2", "P1:v9@St1"]);
        assert_eq!(audit.lost_pitched, 7);
        assert_eq!(audit.slot_use, vec!["P1St1=4/4", "P1St2=1/4"]);
    }

    #[test]
    fn removing_the_overflow_leaves_smaller_other_staff_channels_admissible() {
        let xml = ordinary_priority_score(1, 1, |label| label.to_string(),
            &[(true, 1, 8), (true, 3, 7), (true, 5, 6), (true, 7, 5),
              (false, 2, 3), (false, 4, 2)]);
        let audit = inspect_xml(&xml);
        assert!(audit.unreserved.is_empty());
        assert_eq!(audit.lost_pitched, 0);
        assert_eq!(audit.slot_use, vec!["P1St1=4/4", "P1St2=2/4"]);
    }

    #[test]
    fn ordinary_starvation_does_not_skip_the_separate_overlapping_phase() {
        let mut profile = PartProfile::default();
        for (key, count) in [(1, 8), (3, 7), (5, 6), (7, 5), (9, 4)] {
            profile.channels.insert(key, Channel {
                heads_and_rests: [count, 0, 0, 0], ..Channel::default()
            });
        }
        profile.channels.insert(2, Channel {
            heads_and_rests: [0, 3, 0, 0], ..Channel::default()
        });
        profile.channels.insert(11, Channel {
            heads_and_rests: [1, 1, 0, 0], overlapping: true, ..Channel::default()
        });
        reserve_slots(&mut profile);
        assert!(profile.channels[&9].ordinary_staff.is_none());
        assert!(profile.channels[&2].ordinary_staff.is_none());
        assert!(!profile.channels[&11].mapped_on(0));
        assert!(profile.channels[&11].mapped_on(1));
    }

    #[test]
    fn ordinary_overflow_is_independent_between_parts() {
        let one = ordinary_priority_score(1, 1, |label| label.to_string(),
            &[(true, 1, 8), (true, 3, 7), (true, 5, 6), (true, 7, 5),
              (true, 9, 4), (false, 2, 3), (false, 4, 2)]);
        let single = inspect_xml(&one);
        let part = one.strip_prefix("<score-partwise>").unwrap()
            .strip_suffix("</score-partwise>").unwrap();
        let duplicate = format!("<score-partwise>{part}{}</score-partwise>",
            part.replacen("id=\"P1\"", "id=\"P2\"", 1));
        let doubled = inspect_xml(&duplicate);
        assert_eq!(doubled.parts, 2);
        assert_eq!(doubled.lost_pitched, 2 * single.lost_pitched);
        assert_eq!(doubled.unreserved.len(), 2 * single.unreserved.len());
        assert_eq!(doubled.channels, 2 * single.channels);
    }

    #[test]
    fn report_section_names_status_and_boundary() {
        let mut out = String::new();
        append_source(&mut out, "render", &inspect_xml(&exhausted_score(24, |value| value.to_string())));
        assert!(out.contains("ImportChannelProfileReadback: Source render;"));
        assert!(out.contains("ProfileStatus exhausted."));
        let mut clean = String::new();
        append_source(&mut clean, "render", &ProfileAudit::default());
        assert!(clean.contains("ProfileStatus unknown."));
    }
}
