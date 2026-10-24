//! Shared read-only replay for persisted MusicXML temporal structure.
//!
//! The replay owns both the serial MusicXML cursor and the temporal extent of
//! notes/chords.  It never selects an artifact, repairs XML, or writes files.

use roxmltree::Node;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SerializedTemporalReplayRow {
    pub(crate) source: String,
    pub(crate) measure: usize,
    pub(crate) expected_duration: i64,
    pub(crate) cursor_min: i64,
    pub(crate) cursor_max: i64,
    pub(crate) cursor_final: i64,
    pub(crate) max_note_end: i64,
    pub(crate) partial_backup_count: usize,
    pub(crate) overflow_events: usize,
    pub(crate) note_end_overflow_events: usize,
    pub(crate) backup_underflow_events: usize,
    pub(crate) chord_temporal_conflicts: usize,
    pub(crate) chord_duration_conflicts: usize,
    pub(crate) chord_conflicts_after_forward: usize,
    pub(crate) chord_conflicts_after_backup: usize,
    pub(crate) post_expected_events: usize,
    pub(crate) first_bad: String,
    pub(crate) trace: String,
    pub(crate) temporal_violation: bool,
    pub(crate) hard_violation: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SerializedTemporalReplayObservation {
    pub(crate) parse_ok: bool,
    pub(crate) score_partwise: bool,
    pub(crate) rows: Vec<SerializedTemporalReplayRow>,
}

#[derive(Debug, Clone, Copy)]
struct ChordAnchor {
    onset: i64,
    voice: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnchorInvalidation {
    Backup,
    Forward,
}

fn direct_child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
}

fn direct_child_i64(node: Node<'_, '_>, name: &str) -> Option<i64> {
    direct_child(node, name)
        .and_then(|child| child.text())
        .and_then(|text| text.trim().parse::<i64>().ok())
}

fn measure_number(node: Node<'_, '_>, fallback: usize) -> usize {
    node.attribute("number")
        .and_then(|number| number.trim().parse::<usize>().ok())
        .unwrap_or(fallback)
}

fn current_meter(
    measure: Node<'_, '_>,
    divisions: &mut i64,
    beats: &mut i64,
    beat_type: &mut i64,
) -> i64 {
    for attributes in measure
        .children()
        .filter(|child| child.is_element() && child.tag_name().name() == "attributes")
    {
        if let Some(value) = direct_child_i64(attributes, "divisions").filter(|value| *value > 0)
        {
            *divisions = value;
        }
        for time in attributes
            .children()
            .filter(|child| child.is_element() && child.tag_name().name() == "time")
        {
            if let Some(value) = direct_child_i64(time, "beats").filter(|value| *value > 0) {
                *beats = value;
            }
            if let Some(value) =
                direct_child_i64(time, "beat-type").filter(|value| *value > 0)
            {
                *beat_type = value;
            }
        }
    }
    (*divisions)
        .saturating_mul(*beats)
        .saturating_mul(4)
        .checked_div((*beat_type).max(1))
        .unwrap_or(1)
        .max(1)
}

fn first_bad_reason(row: &SerializedTemporalReplayRow) -> String {
    let mut reasons = Vec::<&str>::new();
    if row.overflow_events > 0 {
        reasons.push("CursorOverflow");
    }
    if row.note_end_overflow_events > 0 {
        reasons.push("NoteEndOverflow");
    }
    if row.backup_underflow_events > 0 {
        reasons.push("BackupUnderflow");
    }
    if row.chord_temporal_conflicts > 0 {
        reasons.push("ChordTemporalConflict");
    }
    if row.partial_backup_count > 0 {
        reasons.push("BackupToNonzeroLegacy");
    }
    if row.cursor_final != row.expected_duration {
        reasons.push("FinalCursorMismatch");
    }
    if reasons.is_empty() {
        "clean".to_string()
    } else {
        reasons.join("+")
    }
}

fn replay_measure_node(
    source: &str,
    measure_number: usize,
    measure: Node<'_, '_>,
    expected_duration: i64,
) -> SerializedTemporalReplayRow {
    let mut cursor = 0_i64;
    let mut cursor_min = 0_i64;
    let mut cursor_max = 0_i64;
    let mut max_note_end = 0_i64;
    let mut partial_backup_count = 0_usize;
    let mut overflow_events = 0_usize;
    let mut note_end_overflow_events = 0_usize;
    let mut backup_underflow_events = 0_usize;
    let mut chord_temporal_conflicts = 0_usize;
    let mut chord_duration_conflicts = 0_usize;
    let mut chord_conflicts_after_forward = 0_usize;
    let mut chord_conflicts_after_backup = 0_usize;
    let mut post_expected_events = 0_usize;
    let mut anchor = None::<ChordAnchor>;
    let mut last_note_duration = None::<i64>;
    let mut anchor_invalidation = None::<AnchorInvalidation>;
    let mut trace = Vec::<String>::new();

    for event in measure.children().filter(|node| node.is_element()) {
        match event.tag_name().name() {
            "note" => {
                let duration = direct_child_i64(event, "duration").unwrap_or(0).max(0);
                let staff = direct_child_i64(event, "staff");
                let voice = direct_child_i64(event, "voice");
                let chord = direct_child(event, "chord").is_some();
                let rest = direct_child(event, "rest").is_some();
                let hidden = event.attribute("print-object") == Some("no")
                    || event.attribute("print-spacing") == Some("no");

                if chord {
                    let valid_anchor = anchor.filter(|candidate| {
                        candidate
                            .voice
                            .is_none_or(|value| voice.is_none_or(|voice| voice == value))
                    });
                    let duration_conflict = last_note_duration
                        .is_some_and(|anchor_duration| duration > anchor_duration);
                    let structural_conflict = valid_anchor.is_none() || rest;
                    if duration_conflict {
                        chord_duration_conflicts = chord_duration_conflicts.saturating_add(1);
                    }
                    if structural_conflict || duration_conflict {
                        chord_temporal_conflicts = chord_temporal_conflicts.saturating_add(1);
                        if duration_conflict {
                            match anchor_invalidation {
                                Some(AnchorInvalidation::Forward) => {
                                    chord_conflicts_after_forward =
                                        chord_conflicts_after_forward.saturating_add(1);
                                }
                                Some(AnchorInvalidation::Backup) => {
                                    chord_conflicts_after_backup =
                                        chord_conflicts_after_backup.saturating_add(1);
                                }
                                None => {}
                            }
                        }
                        if trace.len() < 10 {
                            trace.push(format!(
                                "ChordTemporalConflict@{cursor}+{duration} s{}v{}",
                                staff.map_or_else(|| "?".to_string(), |value| value.to_string()),
                                voice.map_or_else(|| "?".to_string(), |value| value.to_string()),
                            ));
                        }
                    }
                    if let Some(candidate) = valid_anchor {
                        let note_end = candidate.onset.saturating_add(duration);
                        max_note_end = max_note_end.max(note_end);
                        if note_end > expected_duration {
                            note_end_overflow_events = note_end_overflow_events.saturating_add(1);
                            if trace.len() < 10 {
                                trace.push(format!(
                                    "NoteEndOverflow:chord@{}+{}>{}",
                                    candidate.onset, duration, note_end
                                ));
                            }
                        }
                    }
                    last_note_duration = Some(duration);
                    anchor_invalidation = None;
                } else {
                    let onset = cursor;
                    let note_end = onset.saturating_add(duration);
                    max_note_end = max_note_end.max(note_end);
                    if note_end > expected_duration {
                        note_end_overflow_events = note_end_overflow_events.saturating_add(1);
                    }
                    cursor = note_end;
                    cursor_min = cursor_min.min(cursor);
                    cursor_max = cursor_max.max(cursor);
                    if cursor > expected_duration {
                        overflow_events = overflow_events.saturating_add(1);
                    }
                    if onset >= expected_duration || note_end > expected_duration {
                        post_expected_events = post_expected_events.saturating_add(1);
                        if trace.len() < 10 {
                            trace.push(format!(
                                "N@{onset}+{duration}>{note_end} s{}v{} {}{}",
                                staff.map_or_else(|| "?".to_string(), |value| value.to_string()),
                                voice.map_or_else(|| "?".to_string(), |value| value.to_string()),
                                if rest { "rest" } else { "note" },
                                if hidden { ":hidden" } else { ":visible" },
                            ));
                        }
                    }
                    anchor = Some(ChordAnchor { onset, voice });
                    last_note_duration = Some(duration);
                    anchor_invalidation = None;
                }
            }
            "backup" => {
                let duration = direct_child_i64(event, "duration").unwrap_or(0).max(0);
                let before = cursor;
                let after = cursor.saturating_sub(duration);
                if after != 0 {
                    partial_backup_count = partial_backup_count.saturating_add(1);
                }
                if after < 0 {
                    backup_underflow_events = backup_underflow_events.saturating_add(1);
                    if trace.len() < 10 {
                        trace.push(format!("BackupUnderflow@{before}-{duration}>{after}"));
                    }
                } else if after != 0 && trace.len() < 10 {
                    trace.push(format!("B@{before}-{duration}>{after}"));
                }
                cursor = after;
                cursor_min = cursor_min.min(cursor);
                anchor = None;
                anchor_invalidation = Some(AnchorInvalidation::Backup);
            }
            "forward" => {
                let duration = direct_child_i64(event, "duration").unwrap_or(0).max(0);
                let before = cursor;
                cursor = cursor.saturating_add(duration);
                cursor_min = cursor_min.min(cursor);
                cursor_max = cursor_max.max(cursor);
                if cursor > expected_duration {
                    overflow_events = overflow_events.saturating_add(1);
                }
                if before >= expected_duration || cursor > expected_duration {
                    post_expected_events = post_expected_events.saturating_add(1);
                    if trace.len() < 10 {
                        trace.push(format!("F@{before}+{duration}>{cursor}"));
                    }
                }
                anchor = None;
                anchor_invalidation = Some(AnchorInvalidation::Forward);
            }
            _ => {}
        }
    }

    let final_cursor_mismatch_temporal = measure.attribute("implicit") != Some("yes");
    let mut row = SerializedTemporalReplayRow {
        source: source.to_string(),
        measure: measure_number,
        expected_duration,
        cursor_min,
        cursor_max,
        cursor_final: cursor,
        max_note_end,
        partial_backup_count,
        overflow_events,
        note_end_overflow_events,
        backup_underflow_events,
        chord_temporal_conflicts,
        chord_duration_conflicts,
        chord_conflicts_after_forward,
        chord_conflicts_after_backup,
        post_expected_events,
        first_bad: String::new(),
        trace: if trace.is_empty() {
            "-".to_string()
        } else {
            trace.join(" ; ")
        },
        temporal_violation: false,
        hard_violation: false,
    };
    row.first_bad = first_bad_reason(&row);
    row.temporal_violation = row.overflow_events > 0
        || row.note_end_overflow_events > 0
        || row.backup_underflow_events > 0
        || row.chord_temporal_conflicts > 0
        || (final_cursor_mismatch_temporal && row.cursor_final != row.expected_duration);
    // Keep the productive r476 raw-cursor contract unchanged. The corrected
    // temporal interpretation above is observation-only in r477.
    row.hard_violation = row.cursor_min < 0
        || row.cursor_max > row.expected_duration
        || row.cursor_final != row.expected_duration
        || row.partial_backup_count > 0
        || row.overflow_events > 0
        || row.post_expected_events > 0
        || row.backup_underflow_events > 0;
    row
}

pub(crate) fn observe_serialized_temporal_xml(
    source: &str,
    xml: &str,
) -> SerializedTemporalReplayObservation {
    let Ok(document) = roxmltree::Document::parse(xml) else {
        return SerializedTemporalReplayObservation::default();
    };
    let root = document.root_element();
    let score_partwise = root.tag_name().name() == "score-partwise";
    if !score_partwise {
        return SerializedTemporalReplayObservation {
            parse_ok: true,
            score_partwise: false,
            rows: Vec::new(),
        };
    }

    let mut rows = Vec::<SerializedTemporalReplayRow>::new();
    for part in root
        .children()
        .filter(|node| node.is_element() && node.tag_name().name() == "part")
    {
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        for (index, measure) in part
            .children()
            .filter(|node| node.is_element() && node.tag_name().name() == "measure")
            .enumerate()
        {
            let expected = current_meter(measure, &mut divisions, &mut beats, &mut beat_type);
            rows.push(replay_measure_node(
                source,
                measure_number(measure, index.saturating_add(1)),
                measure,
                expected,
            ));
        }
    }
    SerializedTemporalReplayObservation {
        parse_ok: true,
        score_partwise,
        rows,
    }
}

pub(crate) fn scan_raw_cursor_xml(source: &str, xml: &str) -> Vec<SerializedTemporalReplayRow> {
    let observation = observe_serialized_temporal_xml(source, xml);
    if observation.parse_ok && observation.score_partwise {
        observation.rows
    } else {
        vec![SerializedTemporalReplayRow {
            source: source.to_string(),
            first_bad: if observation.parse_ok {
                "UnsupportedRoot".to_string()
            } else {
                "ParseFailure".to_string()
            },
            trace: "serialized_temporal_replay_unsupported".to_string(),
            temporal_violation: true,
            hard_violation: true,
            ..SerializedTemporalReplayRow::default()
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::{observe_serialized_temporal_xml, scan_raw_cursor_xml};

    fn score(body: &str, implicit: bool) -> String {
        let implicit = if implicit { " implicit=\"yes\"" } else { "" };
        format!(
            r#"<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="1"{implicit}><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>"#
        )
    }

    fn replay(body: &str) -> super::SerializedTemporalReplayRow {
        observe_serialized_temporal_xml("Synthetic", &score(body, false))
            .rows
            .into_iter()
            .next()
            .expect("synthetic score must contain one measure")
    }

    #[test]
    fn masked_cursor_overflow_remains_red_after_repositioning() {
        let row = replay(
            "<note><duration>408</duration><voice>1</voice><staff>1</staff></note><backup><duration>384</duration></backup><forward><duration>360</duration></forward>",
        );
        assert_eq!(row.cursor_max, 408);
        assert_eq!(row.cursor_final, 384);
        assert_eq!(row.overflow_events, 1);
        assert!(row.first_bad.contains("CursorOverflow"));
    }

    #[test]
    fn longer_chord_member_is_a_temporal_conflict() {
        let row = replay(
            "<note><duration>72</duration><voice>1</voice><staff>1</staff></note><note><chord/><duration>192</duration><voice>1</voice><staff>1</staff></note><forward><duration>312</duration></forward>",
        );
        assert_eq!(row.chord_temporal_conflicts, 1);
        assert_eq!(row.chord_duration_conflicts, 1);
        assert!(row.first_bad.contains("ChordTemporalConflict"));
        assert!(row.temporal_violation);
        assert!(!row.hard_violation);
    }

    #[test]
    fn chord_extent_can_overflow_without_serial_cursor_overflow() {
        let row = replay(
            "<forward><duration>300</duration></forward><note><duration>84</duration><voice>1</voice><staff>1</staff></note><note><chord/><duration>100</duration><voice>1</voice><staff>1</staff></note>",
        );
        assert_eq!(row.cursor_max, 384);
        assert_eq!(row.overflow_events, 0);
        assert_eq!(row.note_end_overflow_events, 1);
        assert_eq!(row.max_note_end, 400);
        assert!(row.temporal_violation);
        assert!(!row.hard_violation);
    }

    #[test]
    fn real_backup_underflow_is_red() {
        let row = replay(
            "<backup><duration>24</duration></backup><note><duration>408</duration><voice>1</voice><staff>1</staff></note>",
        );
        assert_eq!(row.backup_underflow_events, 1);
        assert!(row.first_bad.contains("BackupUnderflow"));
    }

    #[test]
    fn complex_partial_backups_are_temporally_valid_but_keep_the_legacy_gate_unchanged() {
        let row = replay(
            "<note><duration>192</duration><voice>1</voice><staff>1</staff></note><backup><duration>96</duration></backup><note><duration>96</duration><voice>2</voice><staff>1</staff></note><backup><duration>192</duration></backup><note><duration>384</duration><voice>1</voice><staff>2</staff></note>",
        );
        assert_eq!(row.partial_backup_count, 1);
        assert_eq!(row.backup_underflow_events, 0);
        assert!(!row.temporal_violation, "{}", row.first_bad);
        assert!(row.hard_violation, "r476 productive gate must remain unchanged");
    }

    #[test]
    fn implicit_incomplete_measure_is_temporally_valid_without_relaxing_the_legacy_gate() {
        let observation = observe_serialized_temporal_xml(
            "Synthetic",
            &score("<note><duration>96</duration></note>", true),
        );
        assert!(!observation.rows[0].temporal_violation);
        assert!(observation.rows[0].hard_violation);
    }

    #[test]
    fn raw_cursor_consumers_remain_fail_closed_for_malformed_xml() {
        let rows = scan_raw_cursor_xml("Synthetic", "<score-partwise>");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].first_bad, "ParseFailure");
        assert!(rows[0].hard_violation);
    }
}
