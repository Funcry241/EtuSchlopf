fn hotspot_plan_audit_fingerprint(
    critical_only: bool,
    limit: usize,
    entries: &[HotspotFocus],
    evidence: &[HotspotEvidenceAudit],
) -> String {
    let mut bytes = b"ETUSCHLOPF_SOURCE_SELECTION_HOTSPOT_PLAN_AUDIT_V1\0".to_vec();
    bytes.push(u8::from(critical_only));
    bytes.extend_from_slice(&(limit as u64).to_le_bytes());
    bytes.extend_from_slice(&(entries.len() as u64).to_le_bytes());
    for entry in entries {
        bytes.extend_from_slice(&(entry.measure as u64).to_le_bytes());
        bytes.push(entry.staff.unwrap_or(0));
        bytes.extend_from_slice(&entry.score.to_le_bytes());
    }
    bytes.extend_from_slice(&(evidence.len() as u64).to_le_bytes());
    for row in evidence {
        bytes.extend_from_slice(&(row.issue_type.len() as u64).to_le_bytes());
        bytes.extend_from_slice(row.issue_type.as_bytes());
        bytes.extend_from_slice(&(row.measure as u64).to_le_bytes());
        bytes.push(row.staff.unwrap_or(0));
        bytes.extend_from_slice(&row.base_weight.to_le_bytes());
        bytes.extend_from_slice(&row.critical_bonus.to_le_bytes());
        bytes.extend_from_slice(&row.hardness.to_le_bytes());
        bytes.extend_from_slice(&row.contribution.to_le_bytes());
        bytes.push(u8::from(row.included_after_truncation));
    }
    sha256::hex(&bytes)
}

fn collect_hotspot_plan(
    report: &CheckReport,
    limit: usize,
    critical_only: bool,
) -> HotspotPlan {
    let mut grouped: BTreeMap<(usize, Option<u8>), i64> = BTreeMap::new();
    let mut raw_evidence = Vec::new();
    for issue in &report.issues {
        if critical_only && !is_critical_piano_issue(issue) {
            continue;
        }
        let Some(measure) = issue.measure.trim().parse::<usize>().ok() else {
            continue;
        };
        let staff = issue
            .staff
            .as_deref()
            .and_then(|value| value.trim().parse::<u8>().ok());
        let (base_weight, critical_bonus, hardness) = if critical_only {
            let base_weight = match issue.issue_type.as_str() {
                "PIANO_HAND_SPAN_LARGE" => 220,
                "PIANO_FAST_LARGE_JUMP" => 205,
                "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => 240,
                _ => 80,
            };
            (base_weight, 0, issue.actual_divisions.unwrap_or(0).clamp(0, 72))
        } else {
            let base_weight = match issue.issue_type.as_str() {
                "PIANO_HAND_SPAN_LARGE" => 120,
                "PIANO_FAST_LARGE_JUMP" => 105,
                "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => 90,
                "LEFT_HAND_HIGH_REGISTER" => 25,
                _ => 0,
            };
            if base_weight == 0 {
                continue;
            }
            (
                base_weight,
                if is_critical_piano_issue(issue) { 80 } else { 0 },
                issue.actual_divisions.unwrap_or(0).clamp(0, 48),
            )
        };
        let contribution = base_weight + critical_bonus + hardness;
        *grouped.entry((measure, staff)).or_default() += contribution;
        raw_evidence.push(HotspotEvidenceAudit {
            issue_type: issue.issue_type.clone(),
            measure,
            staff,
            base_weight,
            critical_bonus,
            hardness,
            contribution,
            included_after_truncation: false,
        });
    }

    let group_count_before_truncation = grouped.len();
    let mut entries = grouped
        .into_iter()
        .map(|((measure, staff), score)| HotspotFocus { measure, staff, score })
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.measure.cmp(&b.measure)));
    entries.truncate(limit);
    let retained_groups = entries
        .iter()
        .map(|entry| (entry.measure, entry.staff))
        .collect::<BTreeSet<_>>();
    for row in &mut raw_evidence {
        row.included_after_truncation = retained_groups.contains(&(row.measure, row.staff));
    }
    let fingerprint_sha256 =
        hotspot_plan_audit_fingerprint(critical_only, limit, &entries, &raw_evidence);
    HotspotPlan {
        entries,
        audit: HotspotPlanAuditProjection {
            critical_only,
            source_issue_count: raw_evidence.len(),
            group_count_before_truncation,
            requested_limit: limit,
            truncated_count: group_count_before_truncation.saturating_sub(retained_groups.len()),
            fingerprint_sha256,
            evidence: raw_evidence,
        },
    }
}

fn collect_critical_hotspots(report: &CheckReport, limit: usize) -> HotspotPlan {
    collect_hotspot_plan(report, limit, true)
}

fn collect_hotspots(report: &CheckReport, limit: usize) -> HotspotPlan {
    collect_hotspot_plan(report, limit, false)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CandidateDecisionKey {
    blockers: usize,
    critical_count: usize,
    max_critical_hardness: i64,
    warnings: usize,
    hints: usize,
    retention_rank: std::cmp::Reverse<i64>,
    human_potential_notes_moved: usize,
}

fn candidate_decision_key_from_facts(
    report: &CheckReport,
    retained_note_percent: Option<f64>,
    human_potential_notes_moved: usize,
) -> CandidateDecisionKey {
    CandidateDecisionKey {
        blockers: report.summary.blockers,
        critical_count: critical_count(report),
        max_critical_hardness: max_critical_hardness(report),
        warnings: report.summary.warnings,
        hints: report.summary.hints,
        retention_rank: std::cmp::Reverse(
            retained_note_percent
                .map(|value| (value * 1_000.0).round() as i64)
                .unwrap_or(i64::MIN),
        ),
        human_potential_notes_moved,
    }
}

fn candidate_decision_key(candidate: &SolverCandidate) -> CandidateDecisionKey {
    candidate_decision_key_from_facts(
        &candidate.final_result,
        candidate.fidelity.retained_note_percent,
        candidate.arrangement_stats.human_potential_notes_moved,
    )
}

fn candidate_selection_key(candidate: &SolverCandidate) -> (CandidateDecisionKey, usize) {
    (candidate_decision_key(candidate), candidate.iteration)
}

fn candidate_strictly_beats_guard(candidate: &SolverCandidate, guard: &SolverCandidate) -> bool {
    candidate_decision_key(candidate) < candidate_decision_key(guard)
}

#[cfg(test)]
mod candidate_non_additive_decision_key_tests {
    use super::CandidateDecisionKey;
    use std::cmp::Reverse;

    fn key(retention: i64, hints: usize, moved: usize) -> CandidateDecisionKey {
        CandidateDecisionKey {
            blockers: 0,
            critical_count: 0,
            max_critical_hardness: 0,
            warnings: 0,
            hints,
            retention_rank: Reverse(retention),
            human_potential_notes_moved: moved,
        }
    }

    #[test]
    fn higher_retention_wins_only_after_issue_facts_tie() {
        assert!(key(80_000, 0, 0) < key(70_000, 0, 0));
        assert!(key(100_000, 1, 0) > key(70_000, 0, 0));
    }

    #[test]
    fn fewer_interventions_break_an_otherwise_exact_tie() {
        assert!(key(80_000, 0, 1) < key(80_000, 0, 2));
    }
}
 


fn count_summary(map: &BTreeMap<String, usize>, limit: usize) -> String { if map.is_empty() { return "keine".to_string(); } map.iter() .take(limit) .map(|(name, count)| format!("{name} {count}")) .collect::<Vec<_>>() .join(", ") }
  
        


fn warning_rule_ids_for_issue(issue: &Issue) -> Vec<String> { let mut ids = vec![issue.rule_id.clone()]; if ids.is_empty() { ids.extend(infer_warning_rule_ids_from_text(&issue.details));
ids.extend(infer_warning_rule_ids_from_text(&issue.message_for_ai)); } ids.sort(); ids.dedup(); ids }  fn infer_warning_rule_ids_from_text(warning: &str) -> Vec<String> { let w = warning.to_lowercase(); let mut ids: Vec<String> = Vec::new(); if w.contains("sprung") || w.contains("jump") {
ids.push("HAN03".to_string()); }
if w.contains("griff") || w.contains("span") || w.contains("weite") { ids.push("HAN02".to_string()); }
if w.contains("dicht") || w.contains("unspielbar") || w.contains("playability") {
ids.push("HAN06".to_string()); }
if w.contains("layout") || w.contains("übervoll") || w.contains("uebervoll") || w.contains("system") { ids.push("LAY01".to_string()); }
if w.contains("schlussseite") || w.contains("verwaist") || w.contains("ending") { ids.push("LAY03".to_string()); }
if w.contains("schutzraum") || w.contains("spacing") || w.contains("octave line") { ids.push("LAY07".to_string()); }
if w.contains("hilfslinie") || w.contains("ledger") { ids.push("REG05".to_string()); }
if w.contains("register") || w.contains("hohe töne") || w.contains("hohe toene") { ids.push("REG01".to_string()); }
if w.contains("schlüssel") || w.contains("schluessel") || w.contains("clef") { ids.push("REG07".to_string()); }
if w.contains("oktav") || w.contains("8va") || w.contains("8vb") || w.contains("15ma") || w.contains("15mb") {
ids.push("OCT04".to_string()); }
if w.contains("oktavierungsbereich") || w.contains("octave span") { ids.push("OCT05".to_string()); }
if w.contains("balk") || w.contains("beam") { ids.push("BEA04".to_string()); }
if w.contains("synthetische balk") || w.contains("synthetic beam") { ids.push("BEA02".to_string()); }
if w.contains("stimmenkreuz") || w.contains("cross") { ids.push("BEA06".to_string()); }
if w.contains("pause") && w.contains("verdeckt") {
ids.push("VOI05".to_string()); }
if w.contains("stimmensprung") || w.contains("voice jump") { ids.push("VOI03".to_string()); }
if w.contains("wav") || w.contains("referenz") || w.contains("reference") { ids.push("REF04".to_string()); }
if w.contains("tempo") || w.contains("raster") || w.contains("dauer") || w.contains("duration") { ids.push("REF03".to_string()); }
if w.contains("rhythm") || w.contains("rhythmus") || w.contains("zählzeit") || w.contains("zaehlzeit") {
ids.push("RHY07".to_string()); } ids.sort(); ids.dedup(); ids } 
  
fn tag_text(block: &str, tag: &str) -> Option<String> { let open = format!("<{tag}>"); let close = format!("</{tag}>"); let start = block.find(&open)? + open.len(); let end = block[start..].find(&close)? + start;
Some(block[start..end].trim().to_string()) } fn tag_text_i64(block: &str, tag: &str) -> Option<i64> { tag_text(block, tag)?.parse::<i64>().ok() } fn note_block_midi(block: &str) -> Option<i32> { let pitch_start = block.find("<pitch>")?;
let pitch_end = block[pitch_start..].find("</pitch>")? + pitch_start + "</pitch>".len(); let pitch = &block[pitch_start..pitch_end]; let step = tag_text(pitch, "step")?;
let alter = tag_text(pitch, "alter").and_then(|v| v.parse::<i32>().ok()).unwrap_or(0); let octave = tag_text(pitch, "octave")?.parse::<i32>().ok()?; let base = match step.as_str() { "C" => 0, "D" => 2, "E" => 4, "F" => 5, "G" => 7,
"A" => 9, "B" => 11, _ => return None, }; Some((octave + 1) * 12 + base + alter) }
  
     
 #[derive(Debug, Clone)]
struct SubstanceGuardEval { index: usize, compatible: bool, visible: usize, hints: usize, decision_key: CandidateDecisionKey, } fn evaluate_substance_guard_candidates( candidates: &[SolverCandidate], best: &SolverCandidate,
parallel: &ParallelPlan, phase_started: Instant, ) -> Vec<SubstanceGuardEval> { let total_candidates = candidates.len(); if total_candidates == 0 { return Vec::new(); } let worker_count = parallel.workers.min(total_candidates).max(1);
report_progress("SubstanzGuard", 0, total_candidates, worker_count, phase_started); let evaluate_one = |index: usize, candidate: &SolverCandidate| { let compatible = candidate.final_result.summary.blockers == 0
&& candidate.final_result.summary.warnings == 0; let hints = candidate.final_result.summary.hints; let visible = if compatible && hints <= best.final_result.summary.hints + 4 { visible_pitched_note_count(candidate.layout_xml()) } else { 0 };
SubstanceGuardEval { index, compatible, visible, hints, decision_key: candidate_decision_key(candidate), } }; if worker_count == 1 {
let mut out = Vec::with_capacity(total_candidates); for (index, candidate) in candidates.iter().enumerate() { let eval = evaluate_one(index, candidate); let done = index + 1; if should_report_progress_tick(done, total_candidates) {
report_progress("SubstanzGuard", done, total_candidates, worker_count, phase_started); } out.push(eval); } return out; } console_progress(format!( "SubstanzGuard: analysiere {total_candidates} Kandidaten parallel mit {worker_count} Workern" )); let next = AtomicUsize::new(0); let completed = AtomicUsize::new(0); let results: Mutex<Vec<SubstanceGuardEval>> = Mutex::new(Vec::with_capacity(total_candidates)); thread::scope(|scope| { for _ in 0..worker_count {
let next = &next; let completed = &completed; let results = &results; let evaluate_one = &evaluate_one; scope.spawn(move || { loop { let index = next.fetch_add(1, Ordering::Relaxed); if index >= candidates.len() { break; }
let eval = evaluate_one(index, &candidates[index]); results.lock().expect("SubstanzGuard Mutex vergiftet").push(eval); let done = completed.fetch_add(1, Ordering::Relaxed) + 1; if should_report_progress_tick(done, total_candidates) {
report_progress("SubstanzGuard", done, total_candidates, worker_count, phase_started); } } }); } }); let mut out = results.into_inner().expect("SubstanzGuard Mutex vergiftet"); out.sort_by_key(|eval| eval.index); out }
fn run_substance_guard(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    parallel: &ParallelPlan,
) {
    let phase_started = Instant::now();
    let start_visible_notes = visible_pitched_note_count(best.layout_xml());
    if best.final_result.summary.blockers != 0 || best.final_result.summary.warnings != 0 {
        return;
    }
    let mut best_index: Option<usize> = None;
    let mut best_visible = start_visible_notes;
    let mut best_hints = best.final_result.summary.hints;
    let mut best_decision_key = candidate_decision_key(best);
    for eval in evaluate_substance_guard_candidates(candidates, best, parallel, phase_started) {
        if !eval.compatible || eval.hints > best.final_result.summary.hints + 4 {
            continue;
        }
        let better = eval.visible > best_visible
            || (eval.visible == best_visible && eval.hints < best_hints)
            || (eval.visible == best_visible
                && eval.hints == best_hints
                && eval.decision_key < best_decision_key);
        if better {
            best_index = Some(eval.index);
            best_visible = eval.visible;
            best_hints = eval.hints;
            best_decision_key = eval.decision_key;
        }
    }
    if let Some(index) = best_index.filter(|_| best_visible > start_visible_notes) {
        let chosen = &candidates[index];
        console_progress(format!(
            "SubstanceGuardCandidateAudit: pool_index={index} iteration={} profile_name={} profile_lineage={:?} source_selection_candidate_id={} source_selection_origin_stage={} best_visible_before={} chosen_visible_after={} visible_gain={}",
            chosen.iteration,
            chosen.profile_name,
            chosen.profile_lineage,
            chosen.midi_seed_stats.source_selection_candidate_id,
            chosen.midi_seed_stats.source_selection_origin_stage,
            start_visible_notes,
            best_visible,
            best_visible.saturating_sub(start_visible_notes)
        ));
        match audit_candidate_material_lineage_gap(
            chosen,
            CandidateMaterialDeclaredEffects::candidate_material(),
        ) {
            Ok(audit) => {
                console_progress(format!(
                    "SubstanceGuardCandidateLineageAudit: authority_bound={} material_fragments={} xml_atomic_notes={} failure_measure={:?} failure_onset={:?} unmatched_new_at_failure={} unmatched_old_at_failure={} receipt_count={} state_fp={:016x} mapping_status={}",
                    audit.authority_bound,
                    audit.material_fragment_count,
                    audit.xml_atomic_note_count,
                    audit.failure_measure,
                    audit.failure_onset,
                    audit.unmatched_new_fragment_count,
                    audit.unmatched_old_fragment_count,
                    audit.receipt_count,
                    audit.state_fingerprint_fnv64,
                    audit.mapping_status
                ));
                console_progress(format!(
                    "SubstanceGuardCandidateReceiptAudit: last_receipt={}",
                    audit.last_receipt.as_deref().unwrap_or("none")
                ));
                for fragment in &audit.unmatched_pitched_fragments {
                    console_progress(format!(
                        "SubstanceGuardCandidateUnmatchedPitched: measure={} onset={} duration={} pitch={:?} staff={} voice={} raw_note_index={} chord={} rest={} hidden={} tie_start={} tie_stop={}",
                        fragment.measure,
                        fragment.onset,
                        fragment.duration,
                        fragment.pitch,
                        fragment.staff,
                        fragment.voice,
                        fragment.raw_note_index,
                        fragment.is_chord,
                        fragment.pitch.is_none(),
                        !fragment.visible,
                        fragment.tie_start,
                        fragment.tie_stop
                    ));
                }
            }
            Err(error) => console_progress(format!(
                "SubstanceGuardCandidateLineageAudit: diagnostic_unavailable error={error:#}"
            )),
        }
        let chosen = candidates.remove(index);
        let previous = std::mem::replace(best, chosen);
        candidates.push(previous);
    }
}
fn visible_pitched_note_count(xml: &str) -> usize { let mut count = 0_usize; let mut pos = 0_usize; while let Some(relative_start) = xml[pos..].find("<note") { let start = pos + relative_start;
let Some(relative_end) = xml[start..].find("</note>") else { break; }; let end = start + relative_end + "</note>".len(); let block = &xml[start..end]; if block.contains("<pitch>") && !block.contains("print-object=\"no\"")
&& !block.contains("print-object='no'") { count += 1; } pos = end; } count } fn source_selection_candidate_evaluation(
    report: &CheckReport,
    retained_note_percent: Option<f64>,
    human_potential_notes_moved: usize,
) -> crate::source_selection::SourceSelectionCandidateEvaluation {
    crate::source_selection::SourceSelectionCandidateEvaluation {
        blocker_count: report.summary.blockers,
        critical_count: critical_count(report),
        max_critical_hardness: max_critical_hardness(report),
        warning_count: report.summary.warnings,
        hint_count: report.summary.hints,
        arrangement_visible_retention_milli_percent: retained_note_percent.map(|value| (value * 1_000.0).round() as i64),
        human_potential_notes_moved,
    }
}
fn max_critical_hardness(report: &CheckReport) -> i64 { report.issues .iter() .filter(|issue| is_critical_piano_issue(issue)) .filter_map(|issue| issue.actual_divisions) .max() .unwrap_or(0) }
fn max_playability_warning_hardness(report: &CheckReport) -> i64 { report .issues .iter()
.filter(|issue| matches!(issue.issue_type.as_str(), "PIANO_HAND_SPAN_LARGE" | "PIANO_FAST_LARGE_JUMP" | "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF")) .filter_map(|issue| issue.actual_divisions) .max() .unwrap_or(0) }
fn playability_warning_load(report: &CheckReport) -> i64 { report .issues .iter() .filter(|issue| matches!(issue.issue_type.as_str(), "PIANO_HAND_SPAN_LARGE" | "PIANO_FAST_LARGE_JUMP" | "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF"))
.map(|issue| { let hardness = issue.actual_divisions.unwrap_or(0).max(1); let weight = match issue.issue_type.as_str() { "PIANO_HAND_SPAN_LARGE" => 130, "PIANO_FAST_LARGE_JUMP" => 110, "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => 160,
_ => 1, }; hardness * weight }) .sum() } fn critical_count(report: &CheckReport) -> usize {
report.issues.iter().filter(|issue| is_critical_piano_issue(issue)).count() } fn is_critical_piano_issue(issue: &Issue) -> bool { match issue.issue_type.as_str() { "PIANO_HAND_SPAN_LARGE" => issue.actual_divisions.unwrap_or(0) >= 18,
"PIANO_FAST_LARGE_JUMP" => issue.actual_divisions.unwrap_or(0) >= 24, "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => true, _ => false, } }  
