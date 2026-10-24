#[derive(Debug)]
struct ArrangementSolverStageOutcome {
    best: SolverCandidate,
    next_iteration: usize,
}

struct ArrangementSolverStageInputs<'a> {
    cli: &'a Cli,
    source: &'a source::SourcePlan,
    reference: &'a ReferenceReport,
    options: &'a CheckOptions,
    parallel: &'a ParallelPlan,
    midi_path: &'a Path,
}

struct ArrangementSolverSearchState {
    best: SolverCandidate,
    candidates: Vec<SolverCandidate>,
    next_iteration: usize,
    source_selection_stage_checkpoints: Vec<crate::source_selection::SourceSelectionSolverStageCheckpoint>,
}

fn source_selection_stage_checkpoint(
    stage: &str,
    candidate: &SolverCandidate,
) -> crate::source_selection::SourceSelectionSolverStageCheckpoint {
    crate::source_selection::SourceSelectionSolverStageCheckpoint {
        stage: stage.to_string(),
        source_selection_candidate_id: candidate.midi_seed_stats.source_selection_candidate_id,
        current_candidate_iteration: candidate.iteration,
        current_profile_name: candidate.profile_name.clone(),
    }
}

fn select_initial_arrangement_solver_search_state(
    inputs: &ArrangementSolverStageInputs<'_>,
) -> anyhow::Result<ArrangementSolverSearchState> {
    let cli = inputs.cli;
    let base_profiles = if cli.no_polish {
        vec![CandidateProfile::balanced()]
    } else {
        midi_seed_solver_profiles()
    };
    console_progress(format!("Basiskandidaten: {} Profile", base_profiles.len()));
    let mut candidates = build_candidate_batch(
        "Basiskandidaten",
        &base_profiles,
        1,
        inputs,
        None,
    )?;
    console_progress(format!(
        "Bewertung: {} Kandidaten werden verglichen",
        candidates.len()
    ));
    let best_overall_index = candidates
        .iter()
        .enumerate()
        .min_by_key(|(_, candidate)| candidate_selection_key(candidate))
        .map(|(idx, _)| idx)
        .ok_or_else(|| anyhow!("ArrangementSolver konnte keinen Kandidaten erzeugen"))?;
    let guard_index = candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| !candidate.profile_lineage.contains(CandidateLineageStep::Trajectory))
        .min_by_key(|(_, candidate)| candidate_selection_key(candidate))
        .map(|(idx, _)| idx);
    let chosen_index = match guard_index {
        Some(guard_idx)
            if candidates[best_overall_index].profile_lineage.contains(CandidateLineageStep::Trajectory)
                && !candidate_strictly_beats_guard(
                    &candidates[best_overall_index],
                    &candidates[guard_idx],
                ) =>
        {
            guard_idx
        }
        _ => best_overall_index,
    };
    let best = candidates.remove(chosen_index);
    console_progress(format!(
        "CandidateDecisionAuthorityCutover: initial_selection=non_additive blockers={} critical={} max_critical_hardness={} warnings={} hints={} retention={:?} human_potential_moves={} iteration={}",
        best.final_result.summary.blockers,
        critical_count(&best.final_result),
        max_critical_hardness(&best.final_result),
        best.final_result.summary.warnings,
        best.final_result.summary.hints,
        best.fidelity.retained_note_percent,
        best.arrangement_stats.human_potential_notes_moved,
        best.iteration,
    ));
    let next_iteration = candidates
        .iter()
        .map(|c| c.iteration)
        .chain(std::iter::once(best.iteration))
        .max()
        .unwrap_or(0)
        + 1;
    let source_selection_stage_checkpoints = vec![source_selection_stage_checkpoint(
        "initial_base_selection",
        &best,
    )];
    Ok(ArrangementSolverSearchState {
        best,
        candidates,
        next_iteration,
        source_selection_stage_checkpoints,
    })
}

fn run_arrangement_solver_guided_search_stage(
    inputs: &ArrangementSolverStageInputs<'_>,
    state: &mut ArrangementSolverSearchState,
) -> anyhow::Result<()> {
    let cli = inputs.cli;
    let options = inputs.options;
    let parallel = inputs.parallel;
    let ArrangementSolverSearchState {
        best,
        candidates,
        next_iteration,
        ..
    } = state;
    if !cli.no_polish {
        run_corpus_driven_hand_rhythm_primary_repair(
            best,
            candidates,
            next_iteration,
            options,
            parallel,
        )?;
    }
    // v0.1.50: LocalBestRefinement. Statt das Zusatzbudget breit in schlechte
    // Varianten zu streuen, wird um die aktuell gewählte Guard-/Best-Fassung
    // lokal weitergesucht. Die Profile bleiben generisch und werden aus dem
    // Gewinnerprofil abgeleitet; die Hotspots kommen weiterhin aus dem Report.
    if !cli.no_polish && best.final_result.summary.blockers == 0 {
        let refine_plan = collect_hotspots(&best.final_result, 18);
        let refine_profiles = local_best_refinement_profiles(&best.profile_contract);
        if !refine_plan.is_empty() && !refine_profiles.is_empty() {
            console_progress(format!(
                "LocalBestRefinement: {} Hotspots in {} Takten; Feinsuche prüft {} Kandidaten",
                refine_plan.len(),
                refine_plan.measure_count(),
                refine_profiles.len()
            ));
            let next_iteration = candidates
                .iter()
                .map(|c| c.iteration)
                .chain(std::iter::once(best.iteration))
                .max()
                .unwrap_or(0)
                + 1;
            let refine_candidates = build_candidate_batch(
                "LocalBestRefinement",
                &refine_profiles,
                next_iteration,
                inputs,
                Some(&refine_plan),
            )?;
            for candidate in refine_candidates {
                if candidate_strictly_beats_guard(&candidate, best) {
                    let previous_best = std::mem::replace(best, candidate);
                    candidates.push(previous_best);
                } else {
                    candidates.push(candidate);
                }
            }
        }
    }
    // v0.1.48: echter ResidualCriticalPass-Nachbrenner.
    // Der erste ResidualCriticalPass war nur Teil der allgemeinen Profilmatrix.
    // Jetzt wird nach dem BestKnownGuard die konkret gewählte Fassung erneut
    // geprüft, ihre verbleibenden kritischen Befunde werden generisch gesammelt,
    // und nur dann wird ein Nachbrenner-Kandidat übernommen, wenn er die
    // Guard-Fassung objektiv schlägt. Keine fest codierten Takte, keine
    // projektspezifischen Sonderfälle.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && critical_count(&best.final_result) > 0
    {
        let residual_plan = collect_hotspots(&best.final_result, 18);
        if !residual_plan.is_empty() {
            let residual_profiles = residual_afterburner_profiles();
            console_progress(format!(
                "ResidualCriticalPass: {} Rest-Hotspots in {} Takten; Nachbrenner prüft {} Kandidaten",
                residual_plan.len(),
                residual_plan.measure_count(),
                residual_profiles.len()
            ));
            let next_iteration = candidates
                .iter()
                .map(|c| c.iteration)
                .chain(std::iter::once(best.iteration))
                .max()
                .unwrap_or(0)
                + 1;
            let residual_candidates = build_candidate_batch(
                "ResidualCriticalPass",
                &residual_profiles,
                next_iteration,
                inputs,
                Some(&residual_plan),
            )?;
            for candidate in residual_candidates {
                if candidate_strictly_beats_guard(&candidate, best) {
                    let previous_best = std::mem::replace(best, candidate);
                    candidates.push(previous_best);
                } else {
                    candidates.push(candidate);
                }
            }
        }
    }
    // v0.1.50: TargetedCriticalPass. Nach dem generischen Nachbrenner wird
    // nicht mehr breit gesucht, sondern ausschließlich auf verbliebene kritische
    // Pianistik-Befunde gezielt. Übernommen wird nur eine objektive Verbesserung
    // gegenüber der aktuellen Fassung.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && critical_count(&best.final_result) > 0
    {
        let targeted_plan = collect_critical_hotspots(&best.final_result, 24);
        let targeted_profiles = targeted_critical_profiles();
        if !targeted_plan.is_empty() && !targeted_profiles.is_empty() {
            console_progress(format!(
                "TargetedCriticalPass: {} kritische Hotspots in {} Takten; prüft {} Kandidaten",
                targeted_plan.len(),
                targeted_plan.measure_count(),
                targeted_profiles.len()
            ));
            let next_iteration = candidates
                .iter()
                .map(|c| c.iteration)
                .chain(std::iter::once(best.iteration))
                .max()
                .unwrap_or(0)
                + 1;
            let targeted_candidates = build_candidate_batch(
                "TargetedCriticalPass",
                &targeted_profiles,
                next_iteration,
                inputs,
                Some(&targeted_plan),
            )?;
            for candidate in targeted_candidates {
                if candidate_strictly_beats_guard(&candidate, best) {
                    let previous_best = std::mem::replace(best, candidate);
                    candidates.push(previous_best);
                } else {
                    candidates.push(candidate);
                }
            }
        }
    }
    // v0.1.51: CriticalMicroRepair. Nach den groben Kandidaten wird nicht
    // weiter breit gesucht, sondern sehr klein um die verbliebenen kritischen
    // Einzelbefunde herum repariert. Die Profile sind absichtlich konservativ:
    // kleine Hotspot-Kappen, bevorzugte Umverteilung, keine Takt-Sonderfaelle.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && critical_count(&best.final_result) > 0
    {
        let micro_plan = collect_critical_hotspots(&best.final_result, 12);
        let micro_profiles = critical_micro_repair_profiles(&best.profile_contract);
        if !micro_plan.is_empty() && !micro_profiles.is_empty() {
            console_progress(format!(
                "CriticalMicroRepair: {} kritische Hotspots in {} Takten; prüft {} Mikrokandidaten",
                micro_plan.len(),
                micro_plan.measure_count(),
                micro_profiles.len()
            ));
            let next_iteration = candidates
                .iter()
                .map(|c| c.iteration)
                .chain(std::iter::once(best.iteration))
                .max()
                .unwrap_or(0)
                + 1;
            let micro_candidates = build_candidate_batch(
                "CriticalMicroRepair",
                &micro_profiles,
                next_iteration,
                inputs,
                Some(&micro_plan),
            )?;
            for candidate in micro_candidates {
                if candidate_strictly_beats_guard(&candidate, best) {
                    let previous_best = std::mem::replace(best, candidate);
                    candidates.push(previous_best);
                } else {
                    candidates.push(candidate);
                }
            }
        }
    }
    Ok(())
}

fn run_arrangement_solver_atomic_playability_repair_stage(
    inputs: &ArrangementSolverStageInputs<'_>,
    state: &mut ArrangementSolverSearchState,
) -> anyhow::Result<()> {
    let cli = inputs.cli;
    let options = inputs.options;
    let parallel = inputs.parallel;
    let ArrangementSolverSearchState {
        best,
        candidates,
        next_iteration,
        ..
    } = state;
    // v0.1.53: SafeAtomicRepair. v0.1.52 hat gezeigt, dass einzelne
    // Staff-Umhängungen musikalisch bessere Kandidaten finden, aber formale
    // Blocker erzeugen. Diese Phase ergänzt deshalb notationssichere Varianten:
    // versteckte Ersetzungs-Pausen, optionale Zielstaff-Kopien als Akkordton
    // und eine lokale Note-Normalisierung.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && critical_count(&best.final_result) > 0
    {
        run_atomic_critical_repair(
            best,
            candidates,
            next_iteration,
            options,
            parallel,
        )?;
        reconcile_candidate_material_lineage(
            best,
            "AtomicCriticalRepair",
            CandidateMaterialDeclaredEffects::candidate_material(),
        )?;
    }
    // v0.1.54: StrictWarningCleaner. Wenn der Nutzer explizit keine Warnungen
    // mehr will, werden nach der kritischen Reparatur auch normale pianistische
    // Warnungen einzelbefundbasiert und notationssicher bereinigt. Der Pass
    // akzeptiert nur blockerfreie Patches, die die Warnungszahl wirklich senken.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && best.final_result.summary.warnings > 0
    {
        run_warning_elimination_repair(
            best,
            candidates,
            next_iteration,
            options,
            parallel,
        )?;
        reconcile_candidate_material_lineage(
            best,
            "WarningEliminationRepair",
            CandidateMaterialDeclaredEffects::candidate_material(),
        )?;
    }
    // v0.1.55: FinalCriticalTaktRepair. Der breite StrictWarningCleaner räumt
    // normale Warnungen ab. Falls danach noch genau einzelne kritische Reste
    // übrig bleiben, werden ausschließlich die betroffenen Takte als lokale
    // Ereignisgruppen bearbeitet: Einzelnoten, Endpunktpaare und hohe Staff-2-
    // Cluster werden blockerfrei geprüft. Das Zusatzbudget bleibt eng geführt.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && best.final_result.summary.warnings > 0
    {
        run_final_critical_takt_repair(
            best,
            candidates,
            next_iteration,
            options,
            parallel,
        )?;
        reconcile_candidate_material_lineage(
            best,
            "FinalCriticalTaktRepair",
            CandidateMaterialDeclaredEffects::candidate_material(),
        )?;
    }
    // v0.1.142: FinalPlayabilityCleaner. Nach der Validator-Härtung entstehen
    // neue, fachlich präzisere Voice-/Außenstimmen-Sprungbefunde. Dieser Pass
    // darf keine festen Takte oder Tonhöhen kennen; er leitet alle Ziele aus
    // den aktuellen Validator-Issues ab und übernimmt nur Kandidaten, die die
    // globale Bewertung verbessern.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && best.final_result.summary.warnings > 0
    {
        run_final_playability_cleaner(
            best,
            candidates,
            next_iteration,
            options,
            parallel,
        )?;
        reconcile_candidate_material_lineage(
            best,
            "FinalPlayabilityCleaner",
            CandidateMaterialDeclaredEffects::candidate_material(),
        )?;
    }
    // v0.1.143: FinalPlayabilityPolish. Falls nach dem generischen Cleaner
    // nur noch wenige nichtkritische Spielbarkeitswarnungen übrig bleiben,
    // werden zusammengesetzte Endpunkt-/Spannweitenvarianten geprüft. Auch
    // dieser Pass kennt keine festen Takte, keine hartcodierten Tonhöhen und
    // keine projektbezogenen Sonderfälle: alle Ziele stammen aus aktuellen Validator-
    // Befunden.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && best.final_result.summary.warnings > 0
    {
        run_final_playability_polish(
            best,
            candidates,
            next_iteration,
            options,
            parallel,
        )?;
        reconcile_candidate_material_lineage(
            best,
            "FinalPlayabilityPolish",
            CandidateMaterialDeclaredEffects::candidate_material(),
        )?;
    }
    // v0.1.144: GlobalHandTrajectorySolver. Falls lokale Restpolitur nicht
    // genügt, werden handkohärente Mehrnoten-/Zeitfenster-Varianten geprüft.
    // Staff ist dabei nur Notationssystem, nicht blind gleich Hand. Keine
    // festen Takte, keine hartcodierten Tonhöhen, keine Regel-Lockerung.
    if !cli.no_polish
        && best.final_result.summary.blockers == 0
        && best.final_result.summary.warnings > 0
    {
        run_global_hand_trajectory_solver(
            best,
            candidates,
            next_iteration,
            options,
            parallel,
        )?;
        reconcile_candidate_material_lineage(
            best,
            "GlobalHandTrajectorySolver",
            CandidateMaterialDeclaredEffects::candidate_material(),
        )?;
    }
    Ok(())
}

fn run_arrangement_solver_substance_gesture_stage(
    inputs: &ArrangementSolverStageInputs<'_>,
    state: &mut ArrangementSolverSearchState,
) -> anyhow::Result<()> {
    let options = inputs.options;
    let parallel = inputs.parallel;
    let ArrangementSolverSearchState {
        best,
        candidates,
        next_iteration,
        ..
    } = state;
    run_substance_guard(best, candidates, parallel);
    reconcile_candidate_material_lineage(
        best,
        "SubstanceGuard",
        CandidateMaterialDeclaredEffects::candidate_material(),
    )?;
    reconcile_candidate_material_lineage(
        best,
        "ComposerSubstancePass",
        CandidateMaterialDeclaredEffects::candidate_material(),
    )?;
    reconcile_candidate_material_lineage(
        best,
        "NearMissRepairPass",
        CandidateMaterialDeclaredEffects::candidate_material(),
    )?;
    reconcile_candidate_material_lineage(
        best,
        "HandTrajectoryV2SubstanceBridge",
        CandidateMaterialDeclaredEffects::candidate_material(),
    )?;
    let _ = run_consensus_gesture_probe(
        best,
        options,
        next_iteration,
    )?;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "ArrangementAfterConsensusGestureProbe",
        best.layout_xml(),
    );
    reconcile_candidate_material_lineage(
        best,
        "ConsensusGestureProbe/ExitInvariant",
        CandidateMaterialDeclaredEffects::representation_only(),
    )?;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "ArrangementAfterConsensusGestureReconcile",
        best.layout_xml(),
    );
    Ok(())
}

fn finish_arrangement_solver_stage(
    mut state: ArrangementSolverSearchState,
    project: &str,
) -> anyhow::Result<ArrangementSolverStageOutcome> {
    state.source_selection_stage_checkpoints.push(source_selection_stage_checkpoint(
        "final_solver_selection",
        &state.best,
    ));

    let mut by_source_candidate = BTreeMap::<usize, crate::source_selection::SourceSelectionCandidateAuditRow>::new();
    for candidate in state.candidates.iter().chain(std::iter::once(&state.best)) {
        let stats = &candidate.midi_seed_stats;
        let row = crate::source_selection::SourceSelectionCandidateAuditRow {
            candidate_id: stats.source_selection_candidate_id,
            origin_stage: stats.source_selection_origin_stage.clone(),
            profile_name: stats.source_selection_profile_name.clone(),
            span_limit: stats.profile_span_limit,
            jump_limit: stats.profile_jump_limit,
            smooth_jumps: stats.profile_smooth_jumps,
            left_high_soft_limit: stats.profile_left_high_soft_limit,
            right_low_soft_limit: stats.profile_right_low_soft_limit,
            hotspot_source_issue_count: stats.hotspot_plan_audit.source_issue_count,
            hotspot_group_count_before_truncation: stats.hotspot_plan_audit.group_count_before_truncation,
            hotspot_requested_limit: stats.hotspot_plan_audit.requested_limit,
            hotspot_truncated_count: stats.hotspot_plan_audit.truncated_count,
            hotspot_fingerprint_sha256: stats.hotspot_plan_audit.fingerprint_sha256.clone(),
            canonical_source_retention_ppm: stats.source_selection_full_causal.canonical_source_retention_ppm,
            shadow_plan_fingerprint_sha256: stats.source_selection_full_causal.shadow_plan_fingerprint_sha256.clone(),
            evaluation: stats.source_selection_candidate_evaluation.clone(),
        };
        if let Some(existing) = by_source_candidate.get(&row.candidate_id) {
            if existing.origin_stage != row.origin_stage
                || existing.profile_name != row.profile_name
                || existing.span_limit != row.span_limit
                || existing.jump_limit != row.jump_limit
                || existing.smooth_jumps != row.smooth_jumps
                || existing.left_high_soft_limit != row.left_high_soft_limit
                || existing.right_low_soft_limit != row.right_low_soft_limit
                || existing.hotspot_source_issue_count != row.hotspot_source_issue_count
                || existing.hotspot_group_count_before_truncation != row.hotspot_group_count_before_truncation
                || existing.hotspot_requested_limit != row.hotspot_requested_limit
                || existing.hotspot_truncated_count != row.hotspot_truncated_count
                || existing.hotspot_fingerprint_sha256 != row.hotspot_fingerprint_sha256
                || existing.shadow_plan_fingerprint_sha256 != row.shadow_plan_fingerprint_sha256
                || existing.canonical_source_retention_ppm != row.canonical_source_retention_ppm
                || existing.evaluation != row.evaluation
            {
                return Err(anyhow!(
                    "SourceSelectionSolverCausality: Candidate-ID {} besitzt widerspruechliche immutable Auditdaten",
                    row.candidate_id
                ));
            }
        } else {
            by_source_candidate.insert(row.candidate_id, row);
        }
    }

    let solver_frontier_row = |candidate: &SolverCandidate| {
        let evaluation = source_selection_candidate_evaluation(
            &candidate.final_result,
            candidate.fidelity.retained_note_percent,
            candidate.arrangement_stats.human_potential_notes_moved,
        );
        crate::source_selection::SourceSelectionSolverFrontierRow {
            solver_iteration: candidate.iteration,
            solver_profile_name: candidate.profile_name.clone(),
            solver_profile_lineage: format!("{:?}", candidate.profile_lineage),
            source_selection_candidate_id: candidate.midi_seed_stats.source_selection_candidate_id,
            evaluation,
            human_potential_notes_moved: candidate.arrangement_stats.human_potential_notes_moved,
            blockers: candidate.final_result.summary.blockers,
            warnings: candidate.final_result.summary.warnings,
            hints: candidate.final_result.summary.hints,
        }
    };
    let final_solver_row = solver_frontier_row(&state.best);
    let mut solver_frontier = state
        .candidates
        .iter()
        .map(solver_frontier_row)
        .collect::<Vec<_>>();
    solver_frontier.push(final_solver_row.clone());
    let final_source_selection_candidate_id =
        state.best.midi_seed_stats.source_selection_candidate_id;
    state.best.midi_seed_stats.source_selection_solver_causality =
        crate::source_selection::project_source_selection_solver_causality(
            by_source_candidate.into_values().collect(),
            solver_frontier,
            state.source_selection_stage_checkpoints.clone(),
            final_source_selection_candidate_id,
            final_solver_row,
        )?;

    console_progress(format!(
        "CandidateDecisionAuthorityCutover: final_selection=non_additive iteration={} source_candidate_id={} blockers={} critical={} max_critical_hardness={} warnings={} hints={} retention={:?} human_potential_moves={}",
        state.best.iteration,
        state.best.midi_seed_stats.source_selection_candidate_id,
        state.best.final_result.summary.blockers,
        critical_count(&state.best.final_result),
        max_critical_hardness(&state.best.final_result),
        state.best.final_result.summary.warnings,
        state.best.final_result.summary.hints,
        state.best.fidelity.retained_note_percent,
        state.best.arrangement_stats.human_potential_notes_moved,
    ));
    // Best-effort read-only output. Batch children preserve stderr in the
    // exported console; a diagnostic I/O failure cannot change music or adoption.
    let _ = std::io::Write::write_fmt(&mut std::io::stderr().lock(),
        format_args!("{}\n", owned_rhythm_diagnostic_line(project, &state.best)));
    state.best.master_state.observe_selected_custody(project,
        state.best.arrangement_stats.owned_rhythm.clone(),
        state.best.arrangement_stats.owned_rhythm_unknown.clone());
    for issue in &state.best.final_result.issues {
        let severity = match issue.severity {
            Severity::Blocker => "BLOCKER",
            Severity::Warning => "WARNUNG",
            Severity::Hint => "HINWEIS",
        };
        let actual_unit = match issue.actual_unit.as_deref() {
            Some("semitones") => "Halbtöne",
            Some("notes") => "Noten",
            Some("midi") => "MIDI",
            _ => "divisions",
        };
        let expected = issue.expected_divisions.as_deref().unwrap_or("-");
        let actual = issue.actual_divisions.map_or_else(
            || "-".to_string(),
            |value| format!("{value} {actual_unit}"),
        );
        let delta = issue.delta_divisions.as_deref().unwrap_or("-");
        let details = issue.details.replace(['\r', '\n'], " ").replace('|', "/");
        console_progress(format!(
            "FinalSolverIssueCausality: severity={severity} authority={}:{} measure={} part={} staff={} voice={} rule_severity={} rule_weight={} expected={} actual={} delta={} details={}",
            issue.rule_id,
            issue.issue_type,
            issue.measure,
            issue.part_id,
            issue.staff.as_deref().unwrap_or("-"),
            issue.voice.as_deref().unwrap_or("-"),
            issue.rule_severity,
            issue.rule_weight,
            expected,
            actual,
            delta,
            details,
        ));
    }
    Ok(ArrangementSolverStageOutcome {
        best: state.best,
        next_iteration: state.next_iteration,
    })
}

fn run_arrangement_solver_stage(
    inputs: ArrangementSolverStageInputs<'_>,
) -> anyhow::Result<ArrangementSolverStageOutcome> {
    // r1291: wall time per solver sub-stage on stderr only (console log).
    let timing_label = inputs.cli.input.as_ref().map(|input| input.display().to_string()).unwrap_or_default();
    let mut stage_clock = std::time::Instant::now();
    let mut search_state = select_initial_arrangement_solver_search_state(&inputs)?;
    stage_clock = pipeline_stage_timing(&timing_label, "ArrangementSolver/InitialSearchState", stage_clock);
    run_arrangement_solver_guided_search_stage(&inputs, &mut search_state)?;
    reconcile_candidate_material_lineage(
        &mut search_state.best,
        "GuidedSearch",
        CandidateMaterialDeclaredEffects::candidate_material(),
    )?;
    search_state.source_selection_stage_checkpoints.push(source_selection_stage_checkpoint(
        "guided_source_selection_search_complete",
        &search_state.best,
    ));
    stage_clock = pipeline_stage_timing(&timing_label, "ArrangementSolver/GuidedSearch", stage_clock);
    run_arrangement_solver_atomic_playability_repair_stage(&inputs, &mut search_state)?;
    search_state.source_selection_stage_checkpoints.push(source_selection_stage_checkpoint(
        "atomic_playability_repair_complete",
        &search_state.best,
    ));
    stage_clock = pipeline_stage_timing(&timing_label, "ArrangementSolver/AtomicPlayabilityRepair", stage_clock);
    run_arrangement_solver_substance_gesture_stage(&inputs, &mut search_state)?;
    search_state.source_selection_stage_checkpoints.push(source_selection_stage_checkpoint(
        "substance_gesture_stage_complete",
        &search_state.best,
    ));
    let _ = pipeline_stage_timing(&timing_label, "ArrangementSolver/SubstanceGesture", stage_clock);
    finish_arrangement_solver_stage(search_state, &inputs.source.input_label)
}
