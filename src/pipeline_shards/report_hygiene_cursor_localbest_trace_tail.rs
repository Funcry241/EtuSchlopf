
fn suppress_lower_staff_treble_clef_high_register_hints(report: &mut CheckReport, xml: &str) -> String {
    let left_hand_high_register = crate::rule_id::ValidatorIssueKind::LeftHandHighRegister.as_str();
    let allowed = lower_staff_treble_high_midis_by_measure(xml);
    let before = report.issues.len();
    let hint_before = report.summary.hints;
    report.issues.retain(|issue| {
        if issue.issue_type != left_hand_high_register || issue.staff.as_deref().unwrap_or("") != "2" {
            return true;
        }
        let Some(midi) = issue.actual_divisions else { return true; };
        !allowed
            .get(&issue.measure)
            .is_some_and(|midis| midis.contains(&(midi as i32)))
    });
    refresh_report_summary(report);
    let removed = before.saturating_sub(report.issues.len());
    let hint_after = report.summary.hints;
    if removed == 0 {
        format!(
            "ClefAwareRegisterValidator: keine {left_hand_high_register}-Hinweise durch temporären Violinschlüssel im unteren System zu unterdrücken; finale Freigabe-Hinweise bleiben {hint_after}; keine festen Spezialtakte."
        )
    } else {
        format!(
            "ClefAwareRegisterValidator: {removed} {left_hand_high_register}-Hinweis(e) unterdrückt, weil die hohe Staff-2-Note unter temporärem Violinschlüssel im unteren System steht; finale Freigabe-Hinweise {hint_before}→{hint_after}; keine festen Spezialtakte."
        )
    }
}
fn refresh_report_summary(report: &mut CheckReport) {
    let blockers = report.issues.iter().filter(|i| i.severity == Severity::Blocker).count();
    let warnings = report.issues.iter().filter(|i| i.severity == Severity::Warning).count();
    let hints = report.issues.iter().filter(|i| i.severity == Severity::Hint).count();
    let status = if blockers > 0 {
        ReportStatus::Blocked
    } else if warnings > 0 {
        ReportStatus::Warn
    } else {
        ReportStatus::Ok
    };
    report.summary = ReportSummary { status, blockers, warnings, hints };
}
fn lower_staff_treble_high_midis_by_measure(xml: &str) -> BTreeMap<String, Vec<i32>> {
    let mut allowed = BTreeMap::<String, Vec<i32>>::new();
    let mut state = LowerStaffClefState::Bass;
    let mut scan = 0;
    while let Some((start, end, measure_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[start..end];
        let mut pos = 0;
        while pos < measure.len() {
            let attr = measure[pos..].find("<attributes").map(|offset| pos + offset);
            let note = measure[pos..].find("<note").map(|offset| pos + offset);
            if attr.is_some_and(|offset| note.is_none_or(|note_offset| offset < note_offset)) {
                let start = attr.unwrap_or(pos);
                let Some(close) = measure[start..].find("</attributes>") else { break; };
                pos = start + close + "</attributes>".len();
                state = lower_staff_clef_state_after_attributes(&measure[start..pos], state);
                continue;
            }
            let Some(start) = note else { break; };
            let Some(close) = measure[start..].find("</note>") else { break; };
            pos = start + close + "</note>".len();
            let block = &measure[start..pos];
            if state == LowerStaffClefState::Treble
                && extract_first_text_from_fragment(block, "staff")
                    .as_deref()
                    .unwrap_or("1")
                    .trim()
                    == "2"
                && let Some(midi) = midi_from_note_fragment(block)
                    .filter(|midi| *midi >= LOWER_STAFF_TREBLE_CLEF_FROM_MIDI)
            {
                allowed
                    .entry(measure_number.to_string())
                    .or_default()
                    .push(midi);
            }
        }
        scan = end;
    }
    allowed
}
fn lower_staff_clef_state_after_attributes(fragment: &str, mut state: LowerStaffClefState) -> LowerStaffClefState {
    let mut scan = 0;
    while let Some(offset) = fragment[scan..].find("<clef") {
        let start = scan + offset;
        let Some(close) = fragment[start..].find("</clef>") else { break; };
        scan = start + close + "</clef>".len();
        let clef = &fragment[start..scan];
        if clef.contains("number=\"2\"") || clef.contains("number='2'") {
            state = if clef.contains("<sign>G</sign>") { LowerStaffClefState::Treble }
                else if clef.contains("<sign>F</sign>") { LowerStaffClefState::Bass }
                else { state };
        }
    }
    state
}
fn measure_sort_key(label: &str) -> i64 {
    label.trim().parse::<i64>().unwrap_or(i64::MAX)
}
#[derive(Debug, Clone, Default)]
struct CursorLaneEndInvariantRepairAudit {
    done_fragment: String,
    source_count: usize,
    reanchor_contract_active: bool,
}
fn cursor_lane_temporal_legacy_audit(
    audit: &crate::cursor_lane_temporal_check::CursorLaneTemporalCheckAudit,
) -> CursorLaneEndInvariantRepairAudit {
    CursorLaneEndInvariantRepairAudit {
        source_count: 1,
        done_fragment: format!(
            "CursorLaneReanchorRestVisibilityContract {}: retired; {}",
            audit.source, audit,
        ),
        ..Default::default()
    }
}

fn merge_cursor_lane_end_invariant_repair_audits(audits: Vec<CursorLaneEndInvariantRepairAudit>) -> CursorLaneEndInvariantRepairAudit {
    let source_count = audits.iter().map(|audit| audit.source_count).sum();
    let done_fragment = audits.into_iter().map(|audit| audit.done_fragment).collect::<Vec<_>>().join("; ");
    CursorLaneEndInvariantRepairAudit { source_count, done_fragment, ..Default::default() }
}
#[derive(Debug, Clone, Copy, Default)]
struct SerializedTemporalFindingCounts {
    cursor_overflow: usize,
    note_end_overflow: usize,
    backup_underflow: usize,
    chord_temporal_conflict: usize,
}

impl SerializedTemporalFindingCounts {
    fn from_rows<'a>(rows: impl Iterator<Item = &'a RawCursorInvariantRow>) -> Self {
        let mut counts = Self::default();
        for row in rows {
            counts.cursor_overflow = counts.cursor_overflow.saturating_add(row.overflow_events);
            counts.note_end_overflow = counts
                .note_end_overflow
                .saturating_add(row.note_end_overflow_events);
            counts.backup_underflow = counts
                .backup_underflow
                .saturating_add(row.backup_underflow_events);
            counts.chord_temporal_conflict = counts
                .chord_temporal_conflict
                .saturating_add(row.chord_temporal_conflicts);
        }
        counts
    }
}

#[derive(Debug, Clone, Default)]
struct RawCursorInvariantHardGateAudit {
    hard_violations: usize,
    master_hard_violations: usize,
    render_hard_violations: usize,
    partial_backup_count: usize,
    master_parse_ok: bool,
    render_present: bool,
    render_parse_ok: bool,
    temporal_observation_green: bool,
    master_findings: SerializedTemporalFindingCounts,
    render_findings: SerializedTemporalFindingCounts,
    rows: Vec<RawCursorInvariantRow>,
}

fn build_raw_cursor_invariant_hard_gate_audit(
    master_xml: &str,
    render_xml: Option<&str>,
) -> RawCursorInvariantHardGateAudit {
    let master = observe_serialized_temporal_xml("MasterXML", master_xml);
    let render = render_xml.map(|xml| observe_serialized_temporal_xml("RenderXML", xml));
    let master_findings = SerializedTemporalFindingCounts::from_rows(master.rows.iter());
    let render_findings = SerializedTemporalFindingCounts::from_rows(
        render.iter().flat_map(|observation| observation.rows.iter()),
    );
    let master_parse_ok = master.parse_ok && master.score_partwise;
    let render_parse_ok = render
        .as_ref()
        .is_some_and(|observation| observation.parse_ok && observation.score_partwise);
    let temporal_observation_green = master_parse_ok
        && (render.is_none() || render_parse_ok)
        && master.rows.iter().all(|row| !row.temporal_violation)
        && render
            .iter()
            .flat_map(|observation| observation.rows.iter())
            .all(|row| !row.temporal_violation);
    let master_artifact_red = if master.parse_ok && master.score_partwise { 0 } else { 1 };
    let render_artifact_red = render.as_ref().map_or(0, |observation| {
        if observation.parse_ok && observation.score_partwise {
            0
        } else {
            1
        }
    });
    let partial_backup_count = master
        .rows
        .iter()
        .chain(
            render
                .iter()
                .flat_map(|observation| observation.rows.iter()),
        )
        .map(|row| row.partial_backup_count)
        .sum();
    let mut rows = master
        .rows
        .into_iter()
        .chain(
            render
                .as_ref()
                .into_iter()
                .flat_map(|observation| observation.rows.iter().cloned()),
        )
        .filter(|row| row.hard_violation)
        .collect::<Vec<_>>();
    let score = |row: &RawCursorInvariantRow| {
        (row.cursor_max - row.expected_duration).max(0) * 4
            + (row.cursor_final - row.expected_duration).abs() * 2
            + row.partial_backup_count as i64 * 500
            + row.post_expected_events as i64 * 80
            + if row.source == "MasterXML" { 40 } else { 0 }
    };
    rows.sort_by(|a, b| {
        score(b)
            .cmp(&score(a))
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.source.cmp(&b.source))
    });
    let master_hard_violations = rows
        .iter()
        .filter(|row| row.source == "MasterXML")
        .count()
        .saturating_add(master_artifact_red);
    let render_hard_violations = rows
        .iter()
        .filter(|row| row.source == "RenderXML")
        .count()
        .saturating_add(render_artifact_red);
    RawCursorInvariantHardGateAudit {
        hard_violations: master_hard_violations.saturating_add(render_hard_violations),
        master_hard_violations,
        render_hard_violations,
        partial_backup_count,
        master_parse_ok,
        render_present: render.is_some(),
        render_parse_ok,
        temporal_observation_green,
        master_findings,
        render_findings,
        rows,
    }
}

fn apply_raw_cursor_invariant_hard_gate_to_report(
    report: &mut CheckReport,
    audit: &RawCursorInvariantHardGateAudit,
) -> String {
    if audit.hard_violations == 0 {
        return "SelectedPdfSourceRawCursorInvariantHardGate: gruen; kein Blocker injiziert."
            .to_string();
    }
    if audit.master_hard_violations == 0 {
        let before = report.issues.len();
        report
            .issues
            .retain(|issue| issue.issue_type != "RAW_CURSOR_INVARIANT_HARD_GATE");
        let removed = before.saturating_sub(report.issues.len());
        if removed > 0 {
            refresh_report_summary(report);
        }
        return format!(
            "SelectedPdfSourceRawCursorInvariantHardGate: Render-only rot quarantiniert; RenderHard {}; stale Issues {removed} entfernt.",
            audit.render_hard_violations
        );
    }
    let kind = crate::rule_id::ValidatorIssueKind::RawCursorInvariantHardGate;
    let issue_type = kind.as_str();
    if !report.issues.iter().any(|issue| issue.issue_type == issue_type) {
        let authority = crate::rule_id::validator_issue_authority(kind);
        let rule = crate::central_rulebook::rule_by_id(authority.rule_id.as_str());
        let first = audit.rows.first();
        let source = first.map_or_else(
            || {
                if !audit.master_parse_ok {
                    "MasterXML".to_string()
                } else {
                    "RenderXML".to_string()
                }
            },
            |row| row.source.clone(),
        );
        let measure = first.map_or_else(|| "0".to_string(), |row| row.measure.to_string());
        let expected = first.map_or(0, |row| row.expected_duration);
        let actual = first.map_or(0, |row| row.cursor_max.max(row.max_note_end));
        let details = first.map_or_else(
            || format!("{source}: persisted artifact parse/score-partwise replay unsupported"),
            |row| {
                format!(
                    "{} measure {}: expected {}; cursor min/max/final {}/{}/{}; max-note-end {}; first_bad {}; trace {}",
                    row.source,
                    row.measure,
                    row.expected_duration,
                    row.cursor_min,
                    row.cursor_max,
                    row.cursor_final,
                    row.max_note_end,
                    row.first_bad,
                    row.trace,
                )
            },
        );
        report.issues.push(Issue {
            severity: crate::rule_id::runtime_severity_for_issue_kind(kind, true),
            issue_type: issue_type.to_string(),
            rule_id: authority.rule_id.as_str().to_string(),
            rule_severity: rule.map_or_else(
                || "UNMAPPED".to_string(),
                |entry| entry.severity.to_string(),
            ),
            rule_weight: rule.map_or(0, |entry| entry.weight),
            part_id: "P1".to_string(),
            part_name: source,
            measure,
            staff: None,
            voice: None,
            expected_divisions: Some(format!("{expected} divisions")),
            actual_divisions: Some(actual),
            actual_unit: authority.actual_unit.map(str::to_string),
            delta_divisions: Some(format!(
                "Master CursorOverflow {} NoteEndOverflow {} BackupUnderflow {} ChordTemporalConflict {}; Render CursorOverflow {} NoteEndOverflow {} BackupUnderflow {} ChordTemporalConflict {}",
                audit.master_findings.cursor_overflow,
                audit.master_findings.note_end_overflow,
                audit.master_findings.backup_underflow,
                audit.master_findings.chord_temporal_conflict,
                audit.render_findings.cursor_overflow,
                audit.render_findings.note_end_overflow,
                audit.render_findings.backup_underflow,
                audit.render_findings.chord_temporal_conflict,
            )),
            han03_jump_identity: None,
            details,
            message_for_ai: "Roher MusicXML-Cursor verletzt die Endinvariante; finales OK bleibt gesperrt.".to_string(),
        });
        refresh_report_summary(report);
    }
    format!(
        "RawCursorInvariantHardGate: Blocker injiziert; MasterHard {}; RenderHard {}; RenderPresent {}; erste rote Quelle {}.",
        audit.master_hard_violations,
        audit.render_hard_violations,
        yes_no(audit.render_present),
        audit.rows.first().map_or("parse", |row| row.source.as_str()),
    )
}
fn parse_musicxml_current_meter(measure: &str, prev_divisions: i64, prev_beats: i64, prev_beat_type: i64) -> (i64, i64, i64, i64) {
    let divisions = tag_text_i64(measure, "divisions").filter(|v| *v > 0).unwrap_or(prev_divisions.max(1));
    let beats = tag_text_i64(measure, "beats").filter(|v| *v > 0).unwrap_or(prev_beats.max(4));
    let beat_type = tag_text_i64(measure, "beat-type").filter(|v| *v > 0).unwrap_or(prev_beat_type.max(4));
    let expected = divisions * beats * 4 / beat_type;
    (divisions, beats, beat_type, expected.max(1))
}

#[cfg(test)]
mod final_serialized_musicxml_temporal_artifact_observation_tests {
    use super::*;

    fn score(body: &str) -> String {
        format!(
            r#"<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="1"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>"#
        )
    }

    #[test]
    fn existing_unselected_render_artifact_remains_observable_without_new_hard_violation() {
        let master = score("<note><duration>384</duration><voice>1</voice><staff>1</staff></note>");
        let render = score(
            "<forward><duration>300</duration></forward><note><duration>84</duration><voice>1</voice><staff>1</staff></note><note><chord/><duration>100</duration><voice>1</voice><staff>1</staff></note>",
        );
        let audit = build_raw_cursor_invariant_hard_gate_audit(&master, Some(&render));
        assert_eq!(audit.master_hard_violations, 0);
        assert_eq!(audit.render_hard_violations, 0);
        assert_eq!(audit.render_findings.note_end_overflow, 1);
        assert_eq!(audit.render_findings.chord_temporal_conflict, 1);
        assert!(!audit.temporal_observation_green);
        assert_eq!(audit.hard_violations, 0);
    }

    #[test]
    fn absent_render_is_reported_as_absent_without_making_master_observation_red() {
        let master = score(
            "<note><duration>384</duration><voice>1</voice><staff>1</staff></note>",
        );
        let audit = build_raw_cursor_invariant_hard_gate_audit(&master, None);
        assert!(audit.master_parse_ok);
        assert!(!audit.render_present);
        assert!(!audit.render_parse_ok);
        assert!(audit.temporal_observation_green);
        assert_eq!(audit.hard_violations, 0);
    }
}

