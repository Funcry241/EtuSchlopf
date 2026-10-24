

















fn visible_measure_rhythm_invariant_signature(xml: &str, target_measure: usize) -> Option<Vec<String>> {
    let mut scan = 0usize;
    while let Some((measure_start, measure_end, measure_number)) = find_next_measure_bounds(xml, scan) {
        if measure_number == target_measure {
            let measure = &xml[measure_start..measure_end];
            let mut signature = Vec::new();
            let mut cursor_by_voice: BTreeMap<String, i64> = BTreeMap::new();
            let mut last_non_chord_start_by_voice: BTreeMap<String, i64> = BTreeMap::new();
            let mut local_scan = 0usize;
            while let Some(note_rel) = measure[local_scan..].find("<note") {
                let note_start = local_scan + note_rel;
                let Some(note_end_rel) = measure[note_start..].find("</note>") else { break; };
                let note_end = note_start + note_end_rel + "</note>".len();
                let block = &measure[note_start..note_end];
                let duration = tag_text_i64(block, "duration").unwrap_or(0);
                let voice = tag_text(block, "voice").unwrap_or_else(|| "1".to_string());
                let staff = tag_text(block, "staff").unwrap_or_else(|| "1".to_string());
                let is_chord = block.contains("<chord");
                let rest = block.contains("<rest");
                let hidden = block.contains("print-object=\"no\"") || block.contains("print-object='no'");
                let start_tick = if is_chord {
                    *last_non_chord_start_by_voice.get(&voice).unwrap_or(&0)
                } else {
                    *cursor_by_voice.get(&voice).unwrap_or(&0)
                };
                if !is_chord {
                    last_non_chord_start_by_voice.insert(voice.clone(), start_tick);
                    *cursor_by_voice.entry(voice.clone()).or_insert(0) += duration;
                }
                if !hidden {
                    signature.push(format!(
                        "s{}|v{}|t{}|d{}|ch{}|r{}",
                        staff,
                        voice,
                        start_tick,
                        duration,
                        if is_chord { 1 } else { 0 },
                        if rest { 1 } else { 0 }
                    ));
                }
                local_scan = note_end;
            }
            return Some(signature);
        }
        scan = measure_end;
    }
    None
}

fn full_pitch_agnostic_structure_signature(xml: &str) -> Vec<String> {
    let mut signature = Vec::new();
    let mut measure_scan = 0usize;
    while let Some((measure_start, measure_end, measure_number)) = find_next_measure_bounds(xml, measure_scan) {
        let measure = &xml[measure_start..measure_end];
        let mut scan = 0usize;
        while scan < measure.len() {
            let note_pos = measure[scan..].find("<note").map(|v| scan + v);
            let backup_pos = measure[scan..].find("<backup").map(|v| scan + v);
            let forward_pos = measure[scan..].find("<forward").map(|v| scan + v);
            let next = [note_pos, backup_pos, forward_pos].into_iter().flatten().min();
            let Some(pos) = next else { break; };
            if note_pos == Some(pos) {
                let Some(end_rel) = measure[pos..].find("</note>") else { break; };
                let end = pos + end_rel + "</note>".len();
                let block = &measure[pos..end];
                let duration = tag_text_i64(block, "duration").unwrap_or(0);
                let voice = tag_text(block, "voice").unwrap_or_else(|| "1".to_string());
                let staff = tag_text(block, "staff").unwrap_or_else(|| "1".to_string());
                let is_chord = block.contains("<chord");
                let is_rest = block.contains("<rest");
                let hidden = block.contains("print-object=\"no\"") || block.contains("print-object='no'");
                let has_pitch = block.contains("<pitch>");
                signature.push(format!(
                    "m{}|note|s{}|v{}|d{}|ch{}|r{}|h{}|p{}",
                    measure_number, staff, voice, duration,
                    if is_chord { 1 } else { 0 },
                    if is_rest { 1 } else { 0 },
                    if hidden { 1 } else { 0 },
                    if has_pitch { 1 } else { 0 }
                ));
                scan = end;
            } else if backup_pos == Some(pos) {
                let Some(end_rel) = measure[pos..].find("</backup>") else { break; };
                let end = pos + end_rel + "</backup>".len();
                let duration = tag_text_i64(&measure[pos..end], "duration").unwrap_or(0);
                signature.push(format!("m{measure_number}|backup|d{duration}"));
                scan = end;
            } else if forward_pos == Some(pos) {
                let Some(end_rel) = measure[pos..].find("</forward>") else { break; };
                let end = pos + end_rel + "</forward>".len();
                let duration = tag_text_i64(&measure[pos..end], "duration").unwrap_or(0);
                signature.push(format!("m{measure_number}|forward|d{duration}"));
                scan = end;
            } else { break; }
        }
        measure_scan = measure_end;
    }
    signature
}



fn issue_rule_id_fragment(issue: &Issue) -> String {
    let mut ids = blocker_rule_ids_for_issue(issue);
    if ids.is_empty() { ids.push(format!("UNMAPPED:{}", issue.issue_type)); }
    ids.sort();
    ids.dedup();
    ids.join("+")
}

fn validator_issue_delta_signature(issue: &Issue) -> String {
    format!(
        "{}|{}|m{}|s{}|v{}|exp{}|act{}|delta{}|{}",
        issue_rule_id_fragment(issue),
        issue.issue_type,
        issue.measure,
        issue.staff.clone().unwrap_or_else(|| "?".to_string()),
        issue.voice.clone().unwrap_or_else(|| "?".to_string()),
        issue.expected_divisions.clone().unwrap_or_else(|| "?".to_string()),
        issue.actual_divisions.map_or_else(|| "?".to_string(), |v| v.to_string()),
        issue.delta_divisions.clone().unwrap_or_else(|| "?".to_string()),
        issue.details
    )
}

fn issue_is_rhy01_rhy08(issue: &Issue) -> bool {
    blocker_rule_ids_for_issue(issue).iter().any(|id| id == "RHY01" || id == "RHY08")
}








fn validator_rhy_issue_signatures(report: &CheckReport) -> BTreeSet<String> {
    report
        .issues
        .iter()
        .filter(|issue| issue.severity == Severity::Blocker && issue_is_rhy01_rhy08(issue))
        .map(validator_issue_delta_signature)
        .collect::<BTreeSet<_>>()
}



#[derive(Debug, Clone)]
struct CandidatePathLedgerValidation {
    normalized_report: CheckReport,
    raw_rhy_signatures: BTreeSet<String>,
}

fn normalize_candidate_report_against_noop(
    mut report: CheckReport,
    noop_raw_rhy_signatures: &BTreeSet<String>,
) -> (CheckReport, usize) {
    let before = report.issues.len();
    report.issues.retain(|issue| {
        if issue.severity == Severity::Blocker && issue_is_rhy01_rhy08(issue) {
            let sig = validator_issue_delta_signature(issue);
            !noop_raw_rhy_signatures.contains(&sig)
        } else {
            true
        }
    });
    refresh_report_summary(&mut report);
    let after = report.issues.len();
    (report, before.saturating_sub(after))
}

fn validate_candidate_path_equivalent(
    xml: &str,
    options: &CheckOptions,
    noop_raw_rhy_signatures: &BTreeSet<String>,
) -> anyhow::Result<CandidatePathLedgerValidation> {
    let parsed = parse_musicxml(xml)?;
    let mut raw_report = run_checks(&parsed, options);
    let _ = suppress_lower_staff_treble_clef_high_register_hints(&mut raw_report, xml);
    let raw_rhy_signatures = validator_rhy_issue_signatures(&raw_report);
    let (normalized_report, _) = normalize_candidate_report_against_noop(raw_report.clone(), noop_raw_rhy_signatures);
    Ok(CandidatePathLedgerValidation {
        normalized_report,
        raw_rhy_signatures,
    })
}











