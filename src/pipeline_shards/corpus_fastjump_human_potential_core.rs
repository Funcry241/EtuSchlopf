
fn parse_bwh_line(text: &str) -> Option<(usize, usize, usize)> {
    for line in text.lines() {
        if line.contains("Blocker:") && line.contains("Warnungen:") && line.contains("Hinweise:") {
            let nums = line.split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();
            if nums.len() >= 3 {
                return Some((nums[0], nums[1], nums[2]));
            }
        }
    }
    None
}
fn parse_canonical_source_retention_percent(text: &str) -> Option<usize> {
    text.lines().find_map(|line| {
        let value = line
            .trim()
            .strip_prefix("CanonicalSourceRetentionPercent:")?
            .trim();
        let integer = value
            .split_once('.')
            .map_or(value, |(head, _)| head)
            .trim();
        let percent = integer.parse::<usize>().ok()?;
        (percent <= 100).then_some(percent)
    })
}

#[cfg(test)]
mod corpus_retention_metric_tests {
    use super::parse_canonical_source_retention_percent;

    #[test]
    fn canonical_source_retention_is_read_from_its_explicit_field() {
        let report = "CanonicalSourceRetentionPercent: 97.612\nArrangementVisibleRetentionPercent: 70.575\nRetentionMetricsAreDistinct: ja\n";
        assert_eq!(parse_canonical_source_retention_percent(report), Some(97));
    }

    #[test]
    fn unrelated_retention_text_cannot_be_misread_as_canonical_source_retention() {
        let report = "Notenerhalt 83%\nArrangementVisibleRetentionPercent: 66.778\nRetentionMetricsAreDistinct: ja\n";
        assert_eq!(parse_canonical_source_retention_percent(report), None);
    }
}
fn resolve_parallel_plan(
    raw_jobs: &str,
    candidate_budget_override: Option<usize>,
) -> anyhow::Result<ParallelPlan> {
    let logical_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .max(1);
    let requested = raw_jobs.trim().to_string();
    let normalized = if requested.is_empty() { "auto" } else { requested.as_str() };
    let workers = match normalized.to_ascii_lowercase().as_str() {
        "auto" => logical_threads.saturating_sub(1).max(1),
        "max" => logical_threads,
        "1" | "single" | "serial" | "sequentiell" => 1,
        other => {
            let parsed = other.parse::<usize>().map_err(|_| {
                anyhow!("Ungültiger Wert für --jobs: '{}'. Erlaubt sind auto, max oder eine Zahl >= 1.", raw_jobs)
            })?;
            if parsed == 0 {
                return Err(anyhow!("Ungültiger Wert für --jobs: 0. Erlaubt sind auto, max oder eine Zahl >= 1."));
            }
            parsed.min(logical_threads.max(1))
        }
    };
    let candidate_budget_workers = resolve_candidate_budget_workers(
        candidate_budget_override,
        workers,
        logical_threads,
    )?;
    let human_potential_budget = human_potential_budget_for_workers(candidate_budget_workers);
    let human_potential_max_moves = human_potential_max_moves_for_workers(candidate_budget_workers);
    Ok(ParallelPlan {
        requested: normalized.to_string(),
        workers,
        candidate_budget_workers,
        logical_threads,
        human_potential_budget,
        human_potential_max_moves,
    })
}

fn resolve_candidate_budget_workers(
    candidate_budget_override: Option<usize>,
    workers: usize,
    logical_threads: usize,
) -> anyhow::Result<usize> {
    if candidate_budget_override.is_some() {
        return Err(anyhow!("--candidate-budget-jobs ist entfallen. --jobs steuert die Parallelitaet; das Suchbudget ist fest 192/5."));
    }
    Ok(workers.max(1).min(logical_threads.max(1)))
}

#[cfg(test)]
mod parallel_plan_budget_tests {
    use super::resolve_candidate_budget_workers;

    #[test]
    fn default_candidate_budget_matches_worker_count() {
        assert_eq!(resolve_candidate_budget_workers(None, 9, 20).unwrap(), 9);
    }

    #[test]
    fn retired_candidate_budget_override_is_explicitly_rejected() {
        assert!(resolve_candidate_budget_workers(Some(19), 9, 20).unwrap_err().to_string().contains("ist entfallen"));
    }

    #[test]
    fn zero_candidate_budget_is_rejected() {
        assert!(resolve_candidate_budget_workers(Some(0), 9, 20).is_err());
    }
}


fn push_unique_limited(target: &mut Vec<String>, value: String, limit: usize) {
    if target.len() >= limit {
        return;
    }
    if !target.iter().any(|existing| existing == &value) {
        target.push(value);
    }
}
fn collect_human_potential_targets(
    stats: &ArrangementStats,
    limit: usize,
) -> Vec<crate::arrangement::HumanPotentialStructuralTarget> {
    let mut targets = stats.human_potential_structural_targets.clone();
    targets.sort_by_key(|target| (
        std::cmp::Reverse(target.score()),
        target.measure_ordinal(),
        target.candidate_rank(),
    ));
    targets.dedup_by_key(|target| (target.measure_ordinal(), target.candidate_rank()));
    targets.truncate(limit);
    targets
}
fn compact_for_report(value: &str, max_len: usize) -> String {
    let collapsed = value.split_whitespace().collect::<Vec<_>>().join(" " );
    if collapsed.chars().count() <= max_len {
        collapsed
    } else {
        let mut out = collapsed.chars().take(max_len.saturating_sub(1)).collect::<String>();
        out.push('…');
        out
    }
}
fn hint_issue_fingerprint(issue: &Issue) -> String {
    format!(
        "{}|m{}|staff{}|voice{}|actual{}|{}",
        issue.issue_type,
        issue.measure,
        issue.staff.as_deref().unwrap_or("?"),
        issue.voice.as_deref().unwrap_or("?"),
        issue.actual_divisions.map_or_else(|| "?".to_string(), |v| v.to_string()),
        compact_for_report(&issue.details, 80)
    )
}
fn added_hint_issues(baseline: &CheckReport, candidate: &CheckReport) -> Vec<Issue> {
    let mut remaining_baseline: BTreeMap<String, usize> = BTreeMap::new();
    for issue in baseline.issues.iter().filter(|issue| issue.severity == Severity::Hint) {
        *remaining_baseline.entry(hint_issue_fingerprint(issue)).or_insert(0) += 1;
    }
    let mut added = Vec::new();
    for issue in candidate.issues.iter().filter(|issue| issue.severity == Severity::Hint) {
        let key = hint_issue_fingerprint(issue);
        if let Some(count) = remaining_baseline.get_mut(&key)
            && *count > 0
        {
            *count -= 1;
            continue;
        }
        added.push(issue.clone());
    }
    added
}
fn hint_type_summary(issues: &[Issue]) -> String {
    if issues.is_empty() {
        return "[]".to_string();
    }
    let mut by_type: BTreeMap<String, usize> = BTreeMap::new();
    for issue in issues {
        *by_type.entry(issue.issue_type.clone()).or_insert(0) += 1;
    }
    format!(
        "[{}]",
        by_type
            .iter()
            .map(|(issue_type, count)| format!("{issue_type} +{count}"))
            .collect::<Vec<_>>()
            .join(", ")
    )
}
fn repairable_hint_pass(issue: &Issue) -> Option<&'static str> {
    if issue.issue_type == "LEFT_HAND_HIGH_REGISTER"
        && issue.staff.as_deref().unwrap_or("").trim() == "2"
        && issue.actual_divisions.is_some_and(|midi| midi as i32 >= LOWER_STAFF_TREBLE_CLEF_FROM_MIDI)
    {
        return Some("clef_aware_lower_staff_high_notes");
    }
    None
}
fn hint_repair_pass_summary(issues: &[Issue]) -> String {
    let mut passes = BTreeSet::new();
    for issue in issues {
        if let Some(pass) = repairable_hint_pass(issue) {
            passes.insert(pass.to_string());
        }
    }
    if !passes.is_empty() {
        passes.insert("octave_placement_after_restaff".to_string());
        passes.insert("visible_rhythm_recheck".to_string());
    }
    if passes.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", passes.into_iter().collect::<Vec<_>>().join(", "))
    }
}
fn hint_repairability_summary(issues: &[Issue], repairable: bool) -> String {
    let mut by_type: BTreeMap<String, usize> = BTreeMap::new();
    for issue in issues {
        let is_repairable = repairable_hint_pass(issue).is_some();
        if is_repairable == repairable {
            *by_type.entry(issue.issue_type.clone()).or_insert(0) += 1;
        }
    }
    if by_type.is_empty() {
        return "[]".to_string();
    }
    format!(
        "[{}]",
        by_type
            .iter()
            .map(|(issue_type, count)| format!("{issue_type} {count}"))
            .collect::<Vec<_>>()
            .join(", ")
    )
}
fn hint_is_local_to_target(issue: &Issue, target: crate::arrangement::HumanPotentialStructuralTarget) -> bool {
    let Some(issue_measure) = issue.measure.parse::<isize>().ok() else {
        return false;
    };
    let target_measure = target.measure_ordinal() as isize;
    (issue_measure - target_measure).abs() <= 1
}
fn hint_locality_summary(issues: &[Issue], target: crate::arrangement::HumanPotentialStructuralTarget) -> String {
    let local = issues.iter().filter(|issue| hint_is_local_to_target(issue, target)).count();
    let nonlocal = issues.len().saturating_sub(local);
    format!("local {local} nonlocal {nonlocal}")
}
fn added_hints_are_locally_repairable(issues: &[Issue], target: crate::arrangement::HumanPotentialStructuralTarget) -> bool {
    !issues.is_empty()
        && issues.iter().all(|issue| hint_is_local_to_target(issue, target))
        && issues.iter().all(|issue| repairable_hint_pass(issue).is_some())
}
fn hint_location_summary(issues: &[Issue], limit: usize) -> String {
    if issues.is_empty() {
        return "[]".to_string();
    }
    let mut locations = issues
        .iter()
        .take(limit)
        .map(|issue| {
            format!(
                "scope=<computed_area> staff{} voice{} midi{} {}",
                issue.staff.as_deref().unwrap_or("?"),
                issue.voice.as_deref().unwrap_or("?"),
                issue.actual_divisions.map_or_else(|| "?".to_string(), |v| v.to_string()),
                compact_for_report(&issue.message_for_ai, 56)
            )
        })
        .collect::<Vec<_>>();
    if issues.len() > limit {
        locations.push(format!("… +{} weitere", issues.len() - limit));
    }
    format!("[{}]", locations.join(" | "))
}
fn record_human_potential_hint_repair_attempt(
    stats: &mut ArrangementStats,
    attempt_rank: usize,
    target: crate::arrangement::HumanPotentialStructuralTarget,
    candidate: &ArrangementVariant,
    baseline: &ArrangementVariant,
    result: &str,
) {
    let Some(before) = candidate.pre_hint_repair_result.as_ref() else { return; };
    let added_before = added_hint_issues(&baseline.final_result, before);
    let added_after = added_hint_issues(&baseline.final_result, &candidate.final_result);
    let repair_applied = candidate.hint_repair_pipeline.is_some()
        || before.summary.hints != candidate.final_result.summary.hints
        || !added_before.is_empty();
    let detail = format!(
        "trial_rank={} target_scope=<computed_area> candidate_rank={} score={} moved_notes={} hints_before={} baseline_hints={} hints_after={} repair_applied={} repair_passes={} added_hint_types={} added_hint_locations={} added_hint_count_by_type={} repairable_hint_types={} hint_locality={} unrepaired_hint_types={} unrepairable_hint_types={} result={} trace={}",
        attempt_rank,
        target.candidate_rank(),
        target.score(),
        candidate.arrangement_stats.human_potential_notes_moved,
        before.summary.hints,
        baseline.final_result.summary.hints,
        candidate.final_result.summary.hints,
        if repair_applied { "true" } else { "false" },
        hint_repair_pass_summary(&added_before),
        hint_type_summary(&added_before),
        hint_location_summary(&added_before, 6),
        hint_type_summary(&added_before),
        hint_repairability_summary(&added_before, true),
        hint_locality_summary(&added_before, target),
        hint_type_summary(&added_after),
        hint_repairability_summary(&added_after, false),
        result,
        candidate
            .hint_repair_pipeline
            .as_ref()
            .map_or_else(|| "keine lokale Notationsänderung".to_string(), |line| compact_for_report(line, 180)),
    );
    push_unique_limited(&mut stats.human_potential_hint_repair_details, detail, 16);
}
fn record_human_potential_global_blocker_details(
    stats: &mut ArrangementStats,
    attempt_rank: usize,
    target: crate::arrangement::HumanPotentialStructuralTarget,
    candidate: &ArrangementVariant,
) {
    let blockers = candidate
        .final_result
        .issues
        .iter()
        .filter(|issue| issue.severity == Severity::Blocker)
        .collect::<Vec<_>>();
    if blockers.is_empty() {
        return;
    }
    let mut by_type: BTreeMap<String, usize> = BTreeMap::new();
    for issue in &blockers {
        *by_type.entry(issue.issue_type.clone()).or_insert(0) += 1;
    }
    let types = by_type
        .iter()
        .map(|(name, count)| format!("{name} {count}"))
        .collect::<Vec<_>>()
        .join(", " );
    if let Some(first) = blockers.first() {
        let expected = first.expected_divisions.clone().unwrap_or_else(|| "?".to_string());
        let actual = first.actual_divisions.map_or_else(|| "?".to_string(), |v| v.to_string());
        let delta = first.delta_divisions.clone().unwrap_or_else(|| "?".to_string());
        let detail = format!(
            "trial_rank={} target_scope=<computed_area> candidate_rank={} score={} moved_notes={} blockers{} [{}]; first_blocker={} blocker_scope=<computed_area> staff{} voice{} expected{} actual{} delta{} details={} ai={}",
            attempt_rank,
            target.candidate_rank(),
            target.score(),
            candidate.arrangement_stats.human_potential_notes_moved,
            blockers.len(),
            types,
            first.issue_type,
            first.staff.as_deref().unwrap_or("?"),
            first.voice.as_deref().unwrap_or("?"),
            expected,
            actual,
            delta,
            compact_for_report(&first.details, 96),
            compact_for_report(&first.message_for_ai, 96),
        );
        push_unique_limited(&mut stats.human_potential_global_blocker_details, detail, 16);
    }
}
fn describe_human_potential_reject_reason(reason: HumanPotentialGlobalGuardReject) -> &'static str {
    match reason {
        HumanPotentialGlobalGuardReject::NoMove => "NoMove",
        HumanPotentialGlobalGuardReject::Blockers => "Blocker",
        HumanPotentialGlobalGuardReject::Warnings => "Warnungen",
        HumanPotentialGlobalGuardReject::Hints => "Hinweise",
        HumanPotentialGlobalGuardReject::Critical => "Kritisch",
        HumanPotentialGlobalGuardReject::Fidelity => "WAV",
        HumanPotentialGlobalGuardReject::Retention => "Notenerhalt",
    }
}
fn record_human_potential_rejected_high_potential(
    stats: &mut ArrangementStats,
    attempt_rank: usize,
    target: crate::arrangement::HumanPotentialStructuralTarget,
    reason: HumanPotentialGlobalGuardReject,
    candidate: &ArrangementVariant,
    baseline: &ArrangementVariant,
) {
    let added_hints = if reason == HumanPotentialGlobalGuardReject::Hints {
        added_hint_issues(&baseline.final_result, &candidate.final_result)
    } else {
        Vec::new()
    };
    let hint_suffix = if reason == HumanPotentialGlobalGuardReject::Hints {
        format!(
            " added_hint_types={} added_hint_locations={} added_hint_count_by_type={} repairable_hint_types={} hint_locality={} unrepairable_hint_types={}",
            hint_type_summary(&added_hints),
            hint_location_summary(&added_hints, 4),
            hint_type_summary(&added_hints),
            hint_repairability_summary(&added_hints, true),
            hint_locality_summary(&added_hints, target),
            hint_repairability_summary(&added_hints, false)
        )
    } else {
        String::new()
    };
    let detail = format!(
        "trial_rank={} target_scope=<computed_area> candidate_rank={} score={} moved_notes={} reason={} candidate_warnings={} baseline_warnings={} candidate_hints={} baseline_hints={} candidate_critical={} baseline_critical={} candidate_retention={} baseline_retention={} wav_rank={} baseline_wav_rank={}{}",
        attempt_rank,
        target.candidate_rank(),
        target.score(),
        candidate.arrangement_stats.human_potential_notes_moved,
        describe_human_potential_reject_reason(reason),
        candidate.final_result.summary.warnings,
        baseline.final_result.summary.warnings,
        candidate.final_result.summary.hints,
        baseline.final_result.summary.hints,
        critical_count(&candidate.final_result),
        critical_count(&baseline.final_result),
        candidate.fidelity.retained_note_percent.map_or_else(|| "?".to_string(), |v| format!("{v:.2}")),
        baseline.fidelity.retained_note_percent.map_or_else(|| "?".to_string(), |v| format!("{v:.2}")),
        fidelity_rank(&candidate.fidelity),
        fidelity_rank(&baseline.fidelity),
        hint_suffix,
    );
    push_unique_limited(&mut stats.human_potential_rejected_high_potential_details, detail, 16);
}
fn record_human_potential_accepted_candidate(
    stats: &mut ArrangementStats,
    attempt_rank: usize,
    target: crate::arrangement::HumanPotentialStructuralTarget,
    candidate: &ArrangementVariant,
) {
    let detail = format!(
        "trial_rank={} target_scope=<computed_area> candidate_rank={} score={} moved_notes={} accepted_scopes={} warnings={} hints={} critical={} retention={} mutation=restaff_existing_note_only",
        attempt_rank,
        target.candidate_rank(),
        target.score(),
        candidate.arrangement_stats.human_potential_notes_moved,
        candidate.arrangement_stats.human_potential_measures,
        candidate.final_result.summary.warnings,
        candidate.final_result.summary.hints,
        critical_count(&candidate.final_result),
        candidate.fidelity.retained_note_percent.map_or_else(|| "?".to_string(), |v| format!("{v:.2}")),
    );
    push_unique_limited(&mut stats.human_potential_accepted_candidate_details, detail, 8);
}

// Search semantics are independent of available hardware. Worker counts only
// schedule the fixed search; these are the accepted wide-search values.
fn human_potential_budget_for_workers(_workers: usize) -> usize { 192 }
fn human_potential_max_moves_for_workers(_workers: usize) -> usize { 5 }
#[cfg(test)]
mod fixed_search_budget_tests {
    use super::*;
    #[test]
    fn search_extent_is_identical_across_worker_counts() {
        for workers in [0, 1, 2, 4, 11, 12, 19, 32, 128] {
            assert_eq!((human_potential_budget_for_workers(workers), human_potential_max_moves_for_workers(workers)), (192, 5));
        }
    }
}
#[derive(Debug)]
struct PreparedSolverSeed {
    iteration: usize,
    profile: CandidateProfile,
    xml: String,
    midi_seed_stats: MidiSeedStats,
    master_genesis: master_state_identity::MasterGenesisIdentity,
}
fn parallel_indexed_results<T, F>(
    total: usize,
    workers: usize,
    evaluate: F,
) -> Vec<(usize, anyhow::Result<T>)>
where
    T: Send,
    F: Fn(usize) -> anyhow::Result<T> + Sync,
{
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<(usize, anyhow::Result<T>)>> =
        Mutex::new(Vec::with_capacity(total));
    thread::scope(|scope| {
        for _ in 0..workers.max(1) {
            let next = &next;
            let results = &results;
            let evaluate = &evaluate;
            scope.spawn(move || loop {
                let index = next.fetch_add(1, Ordering::Relaxed);
                if index >= total {
                    break;
                }
                let result = evaluate(index);
                results
                    .lock()
                    .expect("ParallelIndex Mutex vergiftet")
                    .push((index, result));
            });
        }
    });
    results.into_inner().expect("ParallelIndex Mutex vergiftet")
}
fn prepare_solver_seed(
    origin_stage: &str,
    iteration: usize,
    profile: CandidateProfile,
    midi_path: &Path,
    input_label: &str,
    hotspots: Option<&HotspotPlan>,
) -> anyhow::Result<PreparedSolverSeed> {
    let (xml, mut midi_seed_stats) = midi_to_musicxml_seed_with_profile_and_hotspots(
        midi_path,
        input_label,
        profile.clone(),
        hotspots,
    )
    .with_context(|| {
        format!(
            "MIDI konnte im Solver-Profil {} nicht übersetzt werden",
            profile.display_name()
        )
    })?;
    midi_seed_stats.source_selection_candidate_id = iteration;
    midi_seed_stats.source_selection_origin_stage = origin_stage.to_string();
    midi_seed_stats.rhythmic_courses.validate_seed_binding(&xml)?;
    let profile_contract = format!("{profile:?}");
    let master_genesis = midi_seed_stats.master_genesis.take().context(
        "MIDI-Seed-GENESIS #68 wurde vom MIDI-Seed-Erzeuger nicht gepraegt",
    )?;
    master_genesis.validate_genesis_binding::<crate::midi_seed::MidiSeedGenesisMint>(
        &profile_contract,
        &midi_seed_genesis_input(&midi_seed_stats),
        &xml,
    )?;
    Ok(PreparedSolverSeed {
        iteration,
        profile,
        xml,
        midi_seed_stats,
        master_genesis,
    })
}
fn midi_seed_genesis_input(
    midi_seed_stats: &MidiSeedStats,
) -> master_state_identity::GenesisInputBinding {
    master_state_identity::GenesisInputBinding::ProductiveSourcePlanSha256(
        midi_seed_stats
            .source_selection_full_causal
            .productive_plan_seed_fingerprint_sha256
            .clone(),
    )
}
fn build_candidate_batch(
    label: &str,
    profiles: &[CandidateProfile],
    start_iteration: usize,
    inputs: &ArrangementSolverStageInputs<'_>,
    hotspots: Option<&HotspotPlan>,
) -> anyhow::Result<Vec<SolverCandidate>> {
    let parallel = inputs.parallel;
    let midi_path = inputs.midi_path;
    let input_label = inputs.source.input_label.as_str();
    let reference = inputs.reference;
    let options = inputs.options;
    let layout_pages = inputs.cli.layout_pages;
    let no_polish = inputs.cli.no_polish;
    let total = profiles.len();
    if total == 0 {
        return Ok(Vec::new());
    }
    let worker_count = parallel.workers.min(total).max(1);
    let started = Instant::now();
    report_progress(label, 0, total, worker_count, started);
    if worker_count > 1 {
        console_progress(format!(
            "{label}: bereite {total} Kandidaten parallel mit {worker_count} Workern vor"
        ));
    }

    // r215: Seed-equivalence reuse. Seed-Erzeugung ist billig und bleibt fuer
    // jedes Profil erhalten. Die teure Candidate-Finalisierung wird dagegen
    // nur fuer byteidentisch verschiedene Seed-MusicXML einmal ausgefuehrt.
    // Die exakte XML-Gleichheit ist die Autoritaet; der FNV-Wert dient nur als
    // Bucket und kann bei einer Kollision niemals Gleichheit vortaeuschen.
    let mut prepared_results = parallel_indexed_results(total, worker_count, |profile_index| {
        if should_report_candidate_progress(profile_index, total) {
            console_progress(format!(
                "{}: bereite Kandidat {}/{} ({}) vor",
                label,
                profile_index + 1,
                total,
                profiles[profile_index].display_name()
            ));
        }
        let seed = prepare_solver_seed(
            label,
            start_iteration + profile_index,
            profiles[profile_index].clone(),
            midi_path,
            input_label,
            hotspots,
        )
        .with_context(|| format!("{} Seed {} fehlgeschlagen", label, profile_index + 1))?;
        Ok(seed)
    });
    prepared_results.sort_by_key(|(index, _)| *index);
    let mut prepared_seeds = Vec::with_capacity(total);
    for (index, result) in prepared_results {
        prepared_seeds.push(
            result.with_context(|| format!("{} Seed {} fehlgeschlagen", label, index + 1))?,
        );
    }

    let mut representatives = Vec::with_capacity(total);
    let mut unique_indices = Vec::new();
    let mut fingerprint_buckets: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for index in 0..total {
        let mut fingerprint = 0xcbf29ce484222325_u64;
        for byte in prepared_seeds[index].xml.as_bytes() {
            fingerprint ^= u64::from(*byte);
            fingerprint = fingerprint.wrapping_mul(0x100000001b3);
        }
        let representative = fingerprint_buckets
            .get(&fingerprint)
            .and_then(|bucket| {
                bucket.iter().copied().find(|existing| {
                    prepared_seeds[*existing].xml.as_bytes()
                        == prepared_seeds[index].xml.as_bytes()
                })
            });
        let representative = representative.unwrap_or_else(|| {
            fingerprint_buckets
                .entry(fingerprint)
                .or_default()
                .push(index);
            unique_indices.push(index);
            index
        });
        representatives.push(representative);
    }

    let unique_total = unique_indices.len();
    if unique_total < total {
        console_progress(format!(
            "{label}: ExactSeedReuse {} von {} Kandidaten teilen byteidentische Seeds; {} eindeutige Finalisierungen",
            total - unique_total,
            total,
            unique_total
        ));
    }

    let unique_worker_count = parallel.workers.min(unique_total).max(1);
    let completed = AtomicUsize::new(0);
    let mut unique_results = parallel_indexed_results(
        unique_total,
        unique_worker_count,
        |unique_position| {
            let seed_index = unique_indices[unique_position];
            let result = finish_solver_candidate_from_seed(
                &prepared_seeds[seed_index],
                parallel,
                reference,
                options,
                layout_pages,
                no_polish,
            );
            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            if should_report_progress_tick(done, unique_total) {
                report_progress(
                    label,
                    done,
                    unique_total,
                    unique_worker_count,
                    started,
                );
            }
            result
                .with_context(|| format!("{} Kandidat {} fehlgeschlagen", label, seed_index + 1))
                .map(|candidate| (seed_index, candidate))
        },
    );
    unique_results.sort_by_key(|(index, _)| *index);
    let mut finished_by_seed = BTreeMap::new();
    for (_unique_position, result) in unique_results {
        let (seed_index, candidate) = result?;
        finished_by_seed.insert(seed_index, candidate);
    }

    let mut candidates = Vec::with_capacity(total);
    for index in 0..total {
        let representative = representatives[index];
        let mut candidate = finished_by_seed
            .get(&representative)
            .with_context(|| {
                format!(
                    "{} Kandidat {} besitzt keine finalisierte Seed-Repraesentation",
                    label,
                    index + 1
                )
            })?
            .clone();
        let seed = &prepared_seeds[index];
        candidate.midi_seed_genesis = seed.master_genesis.clone();
        let seed_profile_contract = format!("{:?}", seed.profile);
        candidate
            .midi_seed_genesis
            .validate_genesis_binding::<crate::midi_seed::MidiSeedGenesisMint>(
                &seed_profile_contract,
                &midi_seed_genesis_input(&seed.midi_seed_stats),
                &seed.xml,
            )?;
        candidate.iteration = seed.iteration;
        candidate.profile_name = seed.profile.display_name().to_string();
        candidate.profile_contract = seed.profile.clone();
        candidate.profile_lineage = CandidateLineage::from_profile(&seed.profile);
        let source_selection_candidate_evaluation =
            candidate.midi_seed_stats.source_selection_candidate_evaluation.clone();
        candidate.midi_seed_stats = seed.midi_seed_stats.clone();
        candidate.material_lineage = CandidateMaterialLineageSpine::default();
        candidate.consensus_relocation_longitudinal_trace.clear();
        candidate.consensus_relocation_freeze_boundary_xml = None;
        if !candidate
            .arrangement_stats
            .material_disposition_rows
            .is_empty()
        {
            crate::source_selection::bind_productive_post_legacy_selection_disposition_plan(
                &mut candidate.midi_seed_stats.source_selection_full_causal,
                &candidate.arrangement_stats.material_disposition_rows,
            )?;
            bind_candidate_material_lineage_from_post_legacy(&mut candidate)?;
        }
        candidate.midi_seed_stats.source_selection_candidate_evaluation =
            source_selection_candidate_evaluation;
        candidates.push(candidate);
    }
    report_progress(label, total, total, worker_count, started);
    Ok(candidates)
}
fn should_report_candidate_progress(profile_idx: usize, total: usize) -> bool {
    profile_idx == 0 || profile_idx + 1 == total || (profile_idx + 1).checked_rem(25) == Some(0)
}
fn has_non_repairable_preledger_blockers(report: &CheckReport) -> bool {
    // v0.1.68: Der rohe MIDI-Seed darf sichtbare Rhythmusfehler enthalten,
    // weil genau diese erst im VisibleRhythmLedger normalisiert werden.
    // Harte Strukturfehler wie Cursor-Unterlauf bleiben dagegen sofortige
    // Blocker.
    report.issues.iter().any(|issue| {
        issue.severity == Severity::Blocker && !is_repairable_visible_rhythm_issue(&issue.issue_type)
    })
}
fn is_repairable_visible_rhythm_issue(issue_type: &str) -> bool {
    issue_type.starts_with("VISIBLE_") || issue_type == "RHYTHM_LEDGER_HIDDEN_TIMELINE_EVENT"
}
fn finish_solver_candidate_from_seed(
    seed: &PreparedSolverSeed,
    parallel: &ParallelPlan,
    reference: &ReferenceReport,
    options: &CheckOptions,
    layout_pages: Option<usize>,
    no_polish: bool,
) -> anyhow::Result<SolverCandidate> {
    let iteration = seed.iteration;
    let profile = &seed.profile;
    let xml = &seed.xml;
    let midi_seed_stats = seed.midi_seed_stats.clone();
    let profile_contract = format!("{profile:?}");
    seed.master_genesis
        .validate_genesis_binding::<crate::midi_seed::MidiSeedGenesisMint>(
            &profile_contract,
            &midi_seed_genesis_input(&midi_seed_stats),
            xml,
        )?;
    let score = parse_musicxml(xml)
        .with_context(|| format!("Interne MIDI-Notationsbasis Profil {} konnte nicht geprüft werden", profile.display_name()))?;
    let first_result = run_checks(&score, options);
    let reference_signature = build_reference_signature(reference, &score);
    if no_polish || has_non_repairable_preledger_blockers(&first_result) {
        let mut midi_seed_stats = midi_seed_stats;
        midi_seed_stats.source_selection_candidate_evaluation =
            source_selection_candidate_evaluation(
                &first_result,
                None,
                ArrangementStats::default().human_potential_notes_moved,
            );
        return Ok(SolverCandidate {
            iteration,
            profile_name: profile.display_name().to_string(),
            profile_contract: (*profile).clone(),
            profile_lineage: CandidateLineage::from_profile(profile),
            final_result: first_result,
            // MIDI-only candidate: its document is the seed output, so its
            // adopted state starts at the seed's own GENESIS #68.
            master_state: structural_write_authority::MasterAdoptedState::adopt_genesis_state(
                (*xml).clone(),
                master_state_identity::MasterLineage::lineage_from_genesis(
                    seed.master_genesis.clone(),
                ),
            ),
            midi_seed_stats,
            midi_seed_genesis: seed.master_genesis.clone(),
            reference_signature,
            fidelity: check_reference_fidelity(reference, &build_reference_signature(reference, &score), &ArrangementStats::default()),
            arrangement_stats: ArrangementStats::default(),
            material_lineage: CandidateMaterialLineageSpine::default(),
            consensus_relocation_longitudinal_trace: Vec::new(),
            consensus_relocation_freeze_boundary_xml: None,
            polish_stats: PolishStats::default(),
            horizontal_layout_seed: HorizontalLayoutSeedPlan::default(),
        });
    }
    let baseline_arrangement = arrange_piano_musicxml_with_profile(xml, ArrangementProfile {
        rhythmic_courses: Some(&midi_seed_stats.rhythmic_courses),
        preserve_reference: reference.guard_active(),
        human_potential_enabled: false,
        ..ArrangementProfile::default()
    });
    let mut chosen_variant = evaluate_arrangement_variant(
        profile.display_name(),
        baseline_arrangement,
        reference,
        &reference_signature,
        options,
        layout_pages,
    )?;
    if reference.guard_active() {
        // v0.1.116: CandidateAuditLocalHintRepair.
        // Erst wird HumanPotential breit und rein diagnostisch entdeckt. Alle
        // generisch starken Kandidaten bleiben im Audit sichtbar; eine frühe
        // sichere Kleinübernahme beendet die Prüfung nicht mehr. Danach wird
        // jeder Kandidat auf derselben Baseline geprüft. Scheitert er nur an
        // lokalen, reparierbaren Notationshinweisen, bekommt er eine lokale
        // RepairTrial-Zwischenstufe und anschließend denselben strengen Guard.
        let max_global_attempts = parallel.human_potential_budget.clamp(1, 24);
        let diagnostic_arrangement = arrange_piano_musicxml_with_profile(xml, ArrangementProfile {
        rhythmic_courses: Some(&midi_seed_stats.rhythmic_courses),
            preserve_reference: true,
            human_potential_enabled: true,
            human_potential_candidate_budget: parallel.human_potential_budget,
            human_potential_max_moves: parallel.human_potential_max_moves,
            human_potential_structural_target: None,
        });
        let mut diagnostic_stats = diagnostic_arrangement.stats;
        let targets = collect_human_potential_targets(&diagnostic_stats, max_global_attempts);
        let mut accepted_human_variant: Option<ArrangementVariant> = None;
        let mut global_checked = 0usize;
        for target in targets.iter().copied() {
            let human_arrangement = arrange_piano_musicxml_with_profile(xml, ArrangementProfile {
        rhythmic_courses: Some(&midi_seed_stats.rhythmic_courses),
                preserve_reference: true,
                human_potential_enabled: true,
                human_potential_candidate_budget: parallel.human_potential_budget,
                human_potential_max_moves: parallel.human_potential_max_moves,
                human_potential_structural_target: Some(target),
            });
            if human_arrangement.stats.human_potential_candidates_checked == 0
                || human_arrangement.stats.human_potential_notes_moved == 0
            {
                continue;
            }
            global_checked += 1;
            let attempt_idx = global_checked.saturating_sub(1);
            let human_variant = evaluate_arrangement_variant(
                profile.display_name(),
                human_arrangement,
                reference,
                &reference_signature,
                options,
                layout_pages,
            )?;
            diagnostic_stats.human_potential_global_candidates_checked = global_checked;
            let mut decision_variant = human_variant;
            let mut repair_logged = false;
            let mut reject_reason = human_potential_variant_reject_reason(&decision_variant, &chosen_variant);
            if reject_reason == Some(HumanPotentialGlobalGuardReject::Hints) {
                let added_hints = added_hint_issues(&chosen_variant.final_result, &decision_variant.final_result);
                if added_hints_are_locally_repairable(&added_hints, target) {
                    let repaired_variant = evaluate_human_potential_local_hint_repair(
                        profile.display_name(),
                        &decision_variant,
                        target,
                        options,
                    )?;
                    let repaired_reject = human_potential_variant_reject_reason(&repaired_variant, &chosen_variant);
                    let repair_result = repaired_reject
                        .map_or_else(|| "accepted".to_string(), |reason| format!("rejected_{}", describe_human_potential_reject_reason(reason)));
                    record_human_potential_hint_repair_attempt(
                        &mut diagnostic_stats,
                        attempt_idx,
                        target,
                        &repaired_variant,
                        &chosen_variant,
                        &repair_result,
                    );
                    repair_logged = true;
                    decision_variant = repaired_variant;
                    reject_reason = repaired_reject;
                }
            }
            if let Some(reason) = reject_reason {
                if reason == HumanPotentialGlobalGuardReject::Blockers {
                    record_human_potential_global_blocker_details(&mut diagnostic_stats, attempt_idx, target, &decision_variant);
                }
                record_human_potential_rejected_high_potential(
                    &mut diagnostic_stats,
                    attempt_idx,
                    target,
                    reason,
                    &decision_variant,
                    &chosen_variant,
                );
                if decision_variant.pre_hint_repair_result.is_some() && !repair_logged {
                    record_human_potential_hint_repair_attempt(
                        &mut diagnostic_stats,
                        attempt_idx,
                        target,
                        &decision_variant,
                        &chosen_variant,
                        &format!("rejected_{}", describe_human_potential_reject_reason(reason)),
                    );
                }
                mark_human_potential_global_guard_reject(&mut diagnostic_stats, reason);
                continue;
            }
            record_human_potential_accepted_candidate(
                &mut diagnostic_stats,
                attempt_idx,
                target,
                &decision_variant,
            );
            if decision_variant.pre_hint_repair_result.is_some() && !repair_logged {
                record_human_potential_hint_repair_attempt(
                    &mut diagnostic_stats,
                    attempt_idx,
                    target,
                    &decision_variant,
                    &chosen_variant,
                    "accepted",
                );
            }
            if accepted_human_variant.is_none() {
                accepted_human_variant = Some(decision_variant);
            }
        }
        if let Some(mut human_variant) = accepted_human_variant {
            let accepted_measures = human_variant.arrangement_stats.human_potential_measures;
            let accepted_notes = human_variant.arrangement_stats.human_potential_notes_moved;
            human_variant.arrangement_stats.human_potential_budget = diagnostic_stats.human_potential_budget;
            human_variant.arrangement_stats.human_potential_candidates_generated = diagnostic_stats.human_potential_candidates_generated;
            human_variant.arrangement_stats.human_potential_candidates_checked = diagnostic_stats.human_potential_candidates_checked;
            human_variant.arrangement_stats.human_potential_candidates_rejected = diagnostic_stats.human_potential_candidates_rejected;
            human_variant.arrangement_stats.human_potential_eligible_measures = diagnostic_stats.human_potential_eligible_measures;
            human_variant.arrangement_stats.human_potential_local_safe_candidates = diagnostic_stats.human_potential_local_safe_candidates;
            human_variant.arrangement_stats.human_potential_no_pitch_band = diagnostic_stats.human_potential_no_pitch_band;
            human_variant.arrangement_stats.human_potential_no_candidates = diagnostic_stats.human_potential_no_candidates;
            human_variant.arrangement_stats.human_potential_reject_duplicate_onset = diagnostic_stats.human_potential_reject_duplicate_onset;
            human_variant.arrangement_stats.human_potential_reject_right_substance = diagnostic_stats.human_potential_reject_right_substance;
            human_variant.arrangement_stats.human_potential_reject_range = diagnostic_stats.human_potential_reject_range;
            human_variant.arrangement_stats.human_potential_reject_jump = diagnostic_stats.human_potential_reject_jump;
            human_variant.arrangement_stats.human_potential_measures = accepted_measures;
            human_variant.arrangement_stats.human_potential_notes_moved = accepted_notes;
            human_variant.arrangement_stats.human_potential_global_candidates_checked = global_checked;
            human_variant.arrangement_stats.human_potential_global_guard_rejected = diagnostic_stats.human_potential_global_guard_rejected;
            human_variant.arrangement_stats.human_potential_global_guard_blockers = diagnostic_stats.human_potential_global_guard_blockers;
            human_variant.arrangement_stats.human_potential_global_guard_warnings = diagnostic_stats.human_potential_global_guard_warnings;
            human_variant.arrangement_stats.human_potential_global_guard_hints = diagnostic_stats.human_potential_global_guard_hints;
            human_variant.arrangement_stats.human_potential_global_guard_critical = diagnostic_stats.human_potential_global_guard_critical;
            human_variant.arrangement_stats.human_potential_global_guard_fidelity = diagnostic_stats.human_potential_global_guard_fidelity;
            human_variant.arrangement_stats.human_potential_global_guard_retention = diagnostic_stats.human_potential_global_guard_retention;
            human_variant.arrangement_stats.human_potential_candidate_details = diagnostic_stats.human_potential_candidate_details.clone();
            human_variant.arrangement_stats.human_potential_global_blocker_details = diagnostic_stats.human_potential_global_blocker_details.clone();
            human_variant.arrangement_stats.human_potential_accepted_candidate_details = diagnostic_stats.human_potential_accepted_candidate_details.clone();
            human_variant.arrangement_stats.human_potential_rejected_high_potential_details = diagnostic_stats.human_potential_rejected_high_potential_details.clone();
            human_variant.arrangement_stats.human_potential_hint_repair_details = diagnostic_stats.human_potential_hint_repair_details.clone();
            chosen_variant = human_variant;
        } else {
            copy_rejected_human_potential_stats(&mut chosen_variant.arrangement_stats, &diagnostic_stats);
        }
    }
    let mut midi_seed_stats = midi_seed_stats;
    midi_seed_stats.source_selection_candidate_evaluation = source_selection_candidate_evaluation(
        &chosen_variant.final_result,
        chosen_variant.fidelity.retained_note_percent,
        chosen_variant.arrangement_stats.human_potential_notes_moved,
    );
    let candidate = SolverCandidate {
        iteration,
        profile_name: profile.display_name().to_string(),
        profile_contract: (*profile).clone(),
        profile_lineage: CandidateLineage::from_profile(profile),
        final_result: chosen_variant.final_result,
        // Arranged candidate: its adopted state starts at its own arrangement
        // GENESIS #62, not at a synthetic continuation of the MIDI seed.
        master_state: chosen_variant.master_state,
        midi_seed_stats,
        midi_seed_genesis: seed.master_genesis.clone(),
        reference_signature,
        fidelity: chosen_variant.fidelity,
        arrangement_stats: chosen_variant.arrangement_stats,
        material_lineage: CandidateMaterialLineageSpine::default(),
        consensus_relocation_longitudinal_trace: Vec::new(),
        consensus_relocation_freeze_boundary_xml: None,
        polish_stats: chosen_variant.polish_stats,
        horizontal_layout_seed: chosen_variant.horizontal_layout_seed,
    };
    Ok(candidate)
}
