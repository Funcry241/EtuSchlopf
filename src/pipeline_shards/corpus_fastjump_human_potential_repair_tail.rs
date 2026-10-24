#[derive(Debug, Clone)]
struct ArrangementVariant {
    final_result: CheckReport,
    /// Sovereign Master state of the variant: GENESIS #62 plus the committed
    /// polish/layout (and hint-repair) owner events.
    master_state: structural_write_authority::MasterAdoptedState,
    pre_hint_repair_result: Option<CheckReport>,
    hint_repair_pipeline: Option<String>,
    fidelity: FidelityCheck,
    arrangement_stats: ArrangementStats,
    polish_stats: PolishStats,
    horizontal_layout_seed: HorizontalLayoutSeedPlan,
}

#[cfg(test)]
#[path = "../../tests/unit/arrangement_variant_representation_transport.rs"]
mod arrangement_variant_representation_transport_tests;

fn copy_rejected_human_potential_stats(target: &mut ArrangementStats, source: &ArrangementStats) {
    target.human_potential_budget = source.human_potential_budget;
    target.human_potential_candidates_generated = source.human_potential_candidates_generated;
    target.human_potential_candidates_checked = source.human_potential_candidates_checked;
    target.human_potential_candidates_rejected = source
        .human_potential_candidates_rejected
        .saturating_add(source.human_potential_notes_moved);
    target.human_potential_eligible_measures = source.human_potential_eligible_measures;
    target.human_potential_local_safe_candidates = source.human_potential_local_safe_candidates;
    target.human_potential_no_pitch_band = source.human_potential_no_pitch_band;
    target.human_potential_no_candidates = source.human_potential_no_candidates;
    target.human_potential_reject_duplicate_onset = source.human_potential_reject_duplicate_onset;
    target.human_potential_reject_right_substance = source.human_potential_reject_right_substance;
    target.human_potential_reject_range = source.human_potential_reject_range;
    target.human_potential_reject_jump = source.human_potential_reject_jump;
    target.human_potential_global_guard_rejected = source.human_potential_global_guard_rejected;
    target.human_potential_global_guard_blockers = source.human_potential_global_guard_blockers;
    target.human_potential_global_guard_warnings = source.human_potential_global_guard_warnings;
    target.human_potential_global_guard_hints = source.human_potential_global_guard_hints;
    target.human_potential_global_guard_critical = source.human_potential_global_guard_critical;
    target.human_potential_global_guard_fidelity = source.human_potential_global_guard_fidelity;
    target.human_potential_global_guard_retention = source.human_potential_global_guard_retention;
    target.human_potential_global_candidates_checked = source.human_potential_global_candidates_checked;
    target.human_potential_global_fallback_rank = source.human_potential_global_fallback_rank;
    target.human_potential_candidate_details = source.human_potential_candidate_details.clone();
    target.human_potential_global_blocker_details = source.human_potential_global_blocker_details.clone();
    target.human_potential_accepted_candidate_details = source.human_potential_accepted_candidate_details.clone();
    target.human_potential_rejected_high_potential_details = source.human_potential_rejected_high_potential_details.clone();
    target.human_potential_hint_repair_details = source.human_potential_hint_repair_details.clone();
    target.human_potential_measures = 0;
    target.human_potential_notes_moved = 0;
}

fn evaluate_arrangement_variant(
    profile_name: &str,
    mut arrangement: ArrangementResult,
    reference: &ReferenceReport,
    reference_signature: &ReferenceSignature,
    options: &CheckOptions,
    layout_pages: Option<usize>,
) -> anyhow::Result<ArrangementVariant> {
    if let Some(courses) = &arrangement.stats.rhythmic_courses {
        courses.validate_binding(&arrangement.xml)?;
    }
    let arrangement_source_genesis = arrangement
        .arrangement_source_genesis
        .clone()
        .context("Arrangement-GENESIS fehlt")?;
    arrangement_source_genesis
        .validate_genesis_output::<crate::arrangement::ArrangementGenesisMint>(&arrangement.xml)?;
    let mut master_state = structural_write_authority::MasterAdoptedState::adopt_genesis_state(
        arrangement.xml.clone(),
        crate::master_state_identity::MasterLineage::lineage_from_genesis(arrangement_source_genesis),
    );
    let fidelity = check_reference_fidelity(reference, reference_signature, &arrangement.stats);
    let polish = polish_musicxml(&arrangement.xml)
        .with_context(|| format!("NotationModel/PolishEngine konnte Profil {profile_name} nicht polieren"))?;
    let layout_plan = layout_pages
        .map_or_else(
            || choose_auto_pianist_layout(AutoLayoutInput {
                measures_seen: arrangement.stats.measures_seen,
                notes_seen: arrangement.stats.notes_seen,
                notes_hidden: arrangement.stats.notes_hidden,
                rests_hidden: arrangement.stats.rests_hidden,
                chord_tones_hidden: arrangement.stats.chord_tones_hidden,
                beam_groups: polish.stats.beam_groups,
                beamed_notes: polish.stats.beamed_notes,
                reference_active: reference.guard_active(),
            }),
            LayoutPlan::manual,
        );
    let layout = apply_pianist_layout(&polish.xml, layout_plan.target_pages);
    let _origin = crate::audit_only::panic_origin_observation::enter("ArrangementVariant/PolishAndLayout", &arrangement.xml, Some(&layout.xml));
    if let Some(courses) = &arrangement.stats.rhythmic_courses {
        let polished_courses = courses.continue_representation(&polish.xml, &polish.rhythmic_transport)?;
        let layout_courses = polished_courses.continue_representation(&layout.xml, &layout.rhythmic_transport)?;
        arrangement.stats.owned_rhythm = Some(std::sync::Arc::new(
            crate::arrangement::OwnedRhythmicContinuation::receive(&layout_courses, &layout.xml)?));
        arrangement.stats.rhythmic_courses = Some(std::sync::Arc::new(layout_courses));
    }
    // GENESIS #62 -> PRESERVING E051 (polish) -> PRESERVING E056 (layout);
    // a refused receipt is a fatal sovereignty violation at adoption.
    polish
        .preserving_receipt
        .as_ref()
        .map(|receipt| {
            receipt.validate_preserving_transition::<NotationPolishPreservingMint>(
                &arrangement.xml,
                &polish.xml,
            )
        })
        .transpose()?;
    layout
        .preserving_receipt
        .as_ref()
        .map(|receipt| {
            receipt.validate_preserving_transition::<crate::layout::PianistLayoutPreservingMint>(
                &polish.xml,
                &layout.xml,
            )
        })
        .transpose()?;
    let polish_receipt_present = polish.preserving_receipt.is_some();
    let layout_receipt_present = layout.preserving_receipt.is_some();
    let mut polish_and_layout_receipts = crate::master_state_identity::OwnerReceipts::default();
    polish_and_layout_receipts.push_preserving(polish.preserving_receipt);
    polish_and_layout_receipts.push_preserving(layout.preserving_receipt);
    if let Some(diagnostic) = polish_and_layout_receipts.coverage_diagnostic(
        master_state.document(), &layout.xml,
    ) {
        crate::audit_only::panic_origin_observation::emit_documents("ArrangementVariant/NotationPolish", &arrangement.xml, Some(&polish.xml));
        // Failure evidence only. The existing sovereign adoption below still
        // consumes the original receipts and makes the fatal decision.
        let polish_evidence = crate::arrangement::representation_transport::preserving_stage_evidence::<NotationPolishPreservingMint>(
            &arrangement.xml, &polish.xml, polish_receipt_present);
        let layout_evidence = crate::arrangement::representation_transport::preserving_stage_evidence::<crate::layout::PianistLayoutPreservingMint>(
            &polish.xml, &layout.xml, layout_receipt_present);
        let _ = std::io::Write::write_fmt(&mut std::io::stderr().lock(), format_args!(
            "ETUSCHLOPF: ArrangementVariantReceiptCoverage: {diagnostic}\nETUSCHLOPF: ArrangementVariantReceiptStage: owner=NotationPolish; {polish_evidence}\nETUSCHLOPF: ArrangementVariantReceiptStage: owner=PianistLayout; {layout_evidence}\n"));
    }
    master_state.adopt_materialized_document(
        layout.xml,
        polish_and_layout_receipts,
        "ArrangementVariant/PolishAndLayout",
    );
    let checked_score = parse_musicxml(master_state.document())
        .with_context(|| format!("Polierte/Layout-MusicXML Profil {profile_name} konnte nicht geprüft werden"))?;
    let mut checked = run_checks(&checked_score, options);
    // v0.1.116: CandidateAudit-Basis bleibt unverfälscht. Die globale
    // Notationspolitur wird hier nicht vorweggenommen, damit Baseline und
    // Kandidaten dieselbe Vergleichsbasis behalten. Clef-aware Hints werden nur
    // dort unterdrückt, wo ein temporärer Violinschlüssel bereits im Kandidaten
    // selbst vorhanden ist.
    let _clef_hint_repair =
        suppress_lower_staff_treble_clef_high_register_hints(&mut checked, master_state.document());
    Ok(ArrangementVariant {
        final_result: checked,
        master_state,
        pre_hint_repair_result: None,
        hint_repair_pipeline: None,
        fidelity,
        arrangement_stats: arrangement.stats,
        polish_stats: polish.stats,
        horizontal_layout_seed: layout.horizontal_layout_seed,
    })
}

fn evaluate_human_potential_local_hint_repair(
    profile_name: &str,
    candidate: &ArrangementVariant,
    target: crate::arrangement::HumanPotentialStructuralTarget,
    options: &CheckOptions,
) -> anyhow::Result<ArrangementVariant> {
    if let Some(courses) = &candidate.arrangement_stats.rhythmic_courses {
        courses.validate_binding(candidate.master_state.document())?;
    }
    // v0.1.116: LocalRepairTrial. Die Reparatur wird nur als zweite Stufe für
    // einen bereits erzeugten HumanPotential-Kandidaten versucht. Sie ändert
    // keine Tonhöhen, Dauern, Noten oder backup/forward-Strukturen; sie lässt
    // lediglich die generische Notationshygiene über das Kandidaten-XML laufen
    // und wird anschließend wieder vollständig geprüft.
    let before = candidate.final_result.clone();
    // The repaired variant clones the committed history of its parent variant
    // and adopts the clef-policy owners' BLOCK #49 receipts on top of it.
    let mut master_state = candidate.master_state.clone();
    let clef_repair = master_state.adopt_in_place_transformation(
        "HumanPotentialLocalHintRepair/ClefPolicy",
        |xml| {
            let mut clef_repair = apply_lower_staff_clef_readability_policy(xml);
            let mut clef_policy_receipts = crate::master_state_identity::OwnerReceipts::default();
            clef_policy_receipts.push_blocks(std::mem::take(&mut clef_repair.block_receipts));
            (clef_repair, clef_policy_receipts)
        },
    );
    let repaired_xml = master_state.document();
    let mut arrangement_stats = candidate.arrangement_stats.clone();
    if let Some(courses) = &arrangement_stats.rhythmic_courses {
        let transport = clef_repair.rhythmic_transport.as_ref().map_err(|e| anyhow!("{e}"))?;
        let continued = courses.continue_representation(repaired_xml, transport)?;
        arrangement_stats.rhythmic_courses = Some(std::sync::Arc::new(continued));
        if let Some(owned) = &arrangement_stats.owned_rhythm {
            arrangement_stats.owned_rhythm = Some(std::sync::Arc::new(
                owned.continue_representation(repaired_xml, transport)?));
        }
    }
    let checked_score = parse_musicxml(repaired_xml)
        .with_context(|| format!("LocalRepairTrial-MusicXML Profil {profile_name} konnte nicht geprüft werden"))?;
    let mut checked = run_checks(&checked_score, options);
    let clef_hint_repair = suppress_lower_staff_treble_clef_high_register_hints(&mut checked, repaired_xml);
    let trace = format!(
        "HumanPotentialLocalRepairTrial: candidate_measure={} candidate_rank={} strikt regelbasiert; Hinweise {}→{}, Warnungen {}→{}, Blocker {}→{}; Registernotation bleibt bis K00 schreibfrei; {}; {}; keine feste Taktlogik, keine Ton-/Dauer-/Noten-/backup-/forward-Änderung.",
        target.measure_ordinal(),
        target.candidate_rank(),
        before.summary.hints,
        checked.summary.hints,
        before.summary.warnings,
        checked.summary.warnings,
        before.summary.blockers,
        checked.summary.blockers,
        clef_repair.done_fragment,
        clef_hint_repair
    );

    Ok(ArrangementVariant {
        final_result: checked,
        master_state,
        pre_hint_repair_result: Some(before),
        hint_repair_pipeline: Some(trace),
        fidelity: candidate.fidelity.clone(),
        arrangement_stats,
        polish_stats: candidate.polish_stats.clone(),
        horizontal_layout_seed: candidate.horizontal_layout_seed.clone(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HumanPotentialGlobalGuardReject {
    NoMove,
    Blockers,
    Warnings,
    Hints,
    Critical,
    Fidelity,
    Retention,
}

fn human_potential_variant_reject_reason(candidate: &ArrangementVariant, baseline: &ArrangementVariant) -> Option<HumanPotentialGlobalGuardReject> {
    if candidate.arrangement_stats.human_potential_notes_moved == 0 {
        return Some(HumanPotentialGlobalGuardReject::NoMove);
    }
    if candidate.final_result.summary.blockers > baseline.final_result.summary.blockers {
        return Some(HumanPotentialGlobalGuardReject::Blockers);
    }
    if candidate.final_result.summary.warnings > baseline.final_result.summary.warnings {
        return Some(HumanPotentialGlobalGuardReject::Warnings);
    }
    if candidate.final_result.summary.hints > baseline.final_result.summary.hints {
        return Some(HumanPotentialGlobalGuardReject::Hints);
    }
    if critical_count(&candidate.final_result) > critical_count(&baseline.final_result) {
        return Some(HumanPotentialGlobalGuardReject::Critical);
    }
    if max_critical_hardness(&candidate.final_result)
        > max_critical_hardness(&baseline.final_result)
    {
        return Some(HumanPotentialGlobalGuardReject::Critical);
    }
    if fidelity_rank(&candidate.fidelity) > fidelity_rank(&baseline.fidelity) {
        return Some(HumanPotentialGlobalGuardReject::Fidelity);
    }
    match (candidate.fidelity.retained_note_percent, baseline.fidelity.retained_note_percent) {
        (Some(candidate_retained), Some(baseline_retained)) if candidate_retained + 0.01 < baseline_retained => return Some(HumanPotentialGlobalGuardReject::Retention),
        _ => {}
    }
    None
}

fn mark_human_potential_global_guard_reject(stats: &mut ArrangementStats, reason: HumanPotentialGlobalGuardReject) {
    if reason == HumanPotentialGlobalGuardReject::NoMove {
        return;
    }
    stats.human_potential_global_guard_rejected += 1;
    match reason {
        HumanPotentialGlobalGuardReject::NoMove => {}
        HumanPotentialGlobalGuardReject::Blockers => stats.human_potential_global_guard_blockers += 1,
        HumanPotentialGlobalGuardReject::Warnings => stats.human_potential_global_guard_warnings += 1,
        HumanPotentialGlobalGuardReject::Hints => stats.human_potential_global_guard_hints += 1,
        HumanPotentialGlobalGuardReject::Critical => stats.human_potential_global_guard_critical += 1,
        HumanPotentialGlobalGuardReject::Fidelity => stats.human_potential_global_guard_fidelity += 1,
        HumanPotentialGlobalGuardReject::Retention => stats.human_potential_global_guard_retention += 1,
    }
}

fn fidelity_rank(fidelity: &FidelityCheck) -> usize {
    match fidelity.verdict {
        reference::FidelityVerdict::Inactive => 0,
        reference::FidelityVerdict::Plausible => 0,
        reference::FidelityVerdict::Check => 1,
        reference::FidelityVerdict::Critical => 2,
    }
}


#[derive(Debug, Clone)]
struct AtomicNote {
    measure: String,
    run: usize,
    staff: String,
    voice: String,
    start: i64,
    duration: i64,
    pitch: Option<Pitch>,
    visible: bool,
    note_start: usize,
    note_end: usize,
    staff_value_range: Option<(usize, usize)>,
    voice_value_range: Option<(usize, usize)>,
    pitch_block_range: Option<(usize, usize)>,
    is_chord: bool,
    has_staccato: bool,
    tie_start: bool,
    tie_stop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AtomicPatchAction {
    MoveStaff,
    RestifyHidden,
    SafeTransferChord,
}

#[derive(Debug, Clone)]
struct AtomicPatch {
    name: String,
    note_start: usize,
    note_end: usize,
    target_staff: String,
    target_voice: String,
    staff_value_range: Option<(usize, usize)>,
    voice_value_range: Option<(usize, usize)>,
    action: AtomicPatchAction,
    insert_after: Option<usize>,
    anchor_duration: Option<i64>,
    material_transfer_carrier: Option<CandidateMaterialRepresentationTransferCarrier>,
}

#[derive(Debug, Clone)]
struct RestifyChoice {
    name: String,
    ranges: Vec<(usize, usize)>,
}

/// Patched document together with the receipts its owners minted while the
/// patch was applied. Provenance only; it never decides adoption.
struct AtomicPatchedDocument {
    xml: String,
    lineage_receipts: crate::master_state_identity::OwnerReceipts,
}

/// One patched candidate input. Move-only: its owner receipts are consumed by
/// exactly one candidate adoption.
#[derive(Debug)]
struct PatchedCandidateInput {
    source_index: usize,
    name: String,
    xml: String,
    material_declared_effects: Option<CandidateMaterialDeclaredEffects>,
    material_transfer_carriers: Vec<CandidateMaterialRepresentationTransferCarrier>,
    lineage_receipts: crate::master_state_identity::OwnerReceipts,
}


fn run_corpus_driven_hand_rhythm_primary_repair(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
) -> anyhow::Result<()> {
    let phase_started = Instant::now();

    // Korpus-Rückführung v0.10.90: Die über mehrere Stücke beobachtete
    // Primärursache ist nicht Pitch, sondern erst Voice-/Rhythmus-Überfüllung
    // und danach Handkurve. Deshalb läuft dieser Pass bewusst vor allen
    // blockerfreien Pitch-/Micro-Spuren. Zielwahl ist ausschließlich aus den
    // aktuellen Validator-Issues abgeleitet.
    for round in 0..10 {
        if overfull_issue_count(&best.final_result) == 0 {
            break;
        }
        let variants = corpus_overfull_repair_variants(best.layout_xml(), &best.final_result, 96);
        if variants.is_empty() {
            break;
        }
        console_progress(format!(
            "CorpusDrivenHandRhythmPrimaryRepair: Overfull-Runde {} prüft {} Voice-/Rhythmusvarianten",
            round + 1,
            variants.len()
        ));
        let inputs = final_variants_to_candidate_inputs(variants, best.layout_xml());
        let evaluated = evaluate_patched_candidates_parallel(
            "CorpusDrivenHandRhythmPrimaryRepair/Overfull",
            inputs,
            best,
            next_iteration,
            options,
            parallel,
            phase_started,
        )?;

        let mut round_best: Option<SolverCandidate> = None;
        let mut rejected = Vec::new();
        for candidate in evaluated {
            let beats_current = corpus_primary_candidate_improves(&candidate, best);
            let beats_round_best = round_best
                .as_ref()
                .is_none_or(|current| corpus_primary_candidate_improves(&candidate, current));
            if beats_current && beats_round_best {
                if let Some(previous) = round_best.replace(candidate) {
                    rejected.push(previous);
                }
            } else {
                rejected.push(candidate);
            }
        }
        candidates.extend(rejected);
        let Some(accepted) = round_best else { break; };
        let previous_best = std::mem::replace(best, accepted);
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        candidates.push(previous_best);
    }

    // Erst wenn die harten Voice-/Rhythmus-Blocker weg sind, werden schnelle
    // große Sprünge als Handkurvenproblem angefasst. Das nutzt vorhandene
    // Warnungs-Patches, nimmt aber nur Verbesserungen an, die keine neuen
    // Blocker erzeugen und die Korpus-Priorität respektieren.
    if best.final_result.summary.blockers == 0 {
        for round in 0..8 {
            if best.final_result.summary.warnings == 0 || critical_count(&best.final_result) == 0 {
                break;
            }
            let variants = corpus_hand_rhythm_repair_variants(best.layout_xml(), &best.final_result, 96);
            if variants.is_empty() {
                break;
            }
            console_progress(format!(
                "CorpusDrivenHandRhythmPrimaryRepair: Hand/Rhythmus-Runde {} prüft {} handkurvennahe Varianten",
                round + 1,
                variants.len()
            ));
            let inputs = final_variants_to_candidate_inputs(variants, best.layout_xml());
            let evaluated = evaluate_patched_candidates_parallel(
                "CorpusDrivenHandRhythmPrimaryRepair/Hand",
                inputs,
                best,
                next_iteration,
                options,
                parallel,
                phase_started,
            )?;

            let mut round_best: Option<SolverCandidate> = None;
            let mut rejected = Vec::new();
            for candidate in evaluated {
                let beats_current = corpus_primary_candidate_improves(&candidate, best);
                let beats_round_best = round_best
                    .as_ref()
                    .is_none_or(|current| corpus_primary_candidate_improves(&candidate, current));
                if beats_current && beats_round_best {
                    if let Some(previous) = round_best.replace(candidate) {
                        rejected.push(previous);
                    }
                } else {
                    rejected.push(candidate);
                }
            }
            candidates.extend(rejected);
            let Some(accepted) = round_best else { break; };
            let previous_best = std::mem::replace(best, accepted);
            crate::audit_only::destructive_edit_attribution::observe_selected_state(
                best.layout_xml(),
            );
            candidates.push(previous_best);
        }
    }
    Ok(())
}

fn overfull_issue_count(report: &CheckReport) -> usize {
    report.issues.iter().filter(|issue| is_voice_overfull_issue(issue)).count()
}

fn is_voice_overfull_issue(issue: &Issue) -> bool {
    matches!(issue.issue_type.as_str(), "VOICE_DURATION_OVERFULL" | "VISIBLE_VOICE_DURATION_OVERFULL" | "MEASURE_CURSOR_OVERFULL")
}

fn corpus_primary_candidate_improves(candidate: &SolverCandidate, current: &SolverCandidate) -> bool {
    let cand_overfull = overfull_issue_count(&candidate.final_result);
    let curr_overfull = overfull_issue_count(&current.final_result);
    if cand_overfull != curr_overfull {
        return cand_overfull < curr_overfull;
    }
    if candidate.final_result.summary.blockers != current.final_result.summary.blockers {
        return candidate.final_result.summary.blockers < current.final_result.summary.blockers;
    }
    let cand_critical = critical_count(&candidate.final_result);
    let curr_critical = critical_count(&current.final_result);
    if cand_critical != curr_critical {
        return cand_critical < curr_critical && candidate.final_result.summary.blockers <= current.final_result.summary.blockers;
    }
    if candidate.final_result.summary.warnings != current.final_result.summary.warnings {
        return candidate.final_result.summary.warnings < current.final_result.summary.warnings
            && candidate.final_result.summary.blockers <= current.final_result.summary.blockers;
    }
    candidate_selection_key(candidate) < candidate_selection_key(current)
}

fn corpus_overfull_repair_variants(xml: &str, report: &CheckReport, limit: usize) -> Vec<PatchVariant> {
    let mut variants = Vec::new();
    let mut seen = BTreeSet::new();
    let issues = report.issues.iter().filter(|issue| is_voice_overfull_issue(issue)).collect::<Vec<_>>();
    for (idx, issue) in issues.iter().take(limit).enumerate() {
        let patched = apply_corpus_voice_overfull_crop(xml, &[*issue]);
        push_corpus_variant(&mut variants, &mut seen, format!("corpus_rhy_overfull_crop_{:03}", idx + 1), xml, patched, limit);
    }
    if issues.len() > 1 {
        let patched = apply_corpus_voice_overfull_crop(xml, &issues);
        push_corpus_variant(&mut variants, &mut seen, "corpus_rhy_overfull_crop_all".to_string(), xml, patched, limit);
    }
    variants
}

fn corpus_hand_rhythm_repair_variants(xml: &str, report: &CheckReport, limit: usize) -> Vec<PatchVariant> {
    let mut variants = Vec::new();
    let mut seen = BTreeSet::new();
    let notes = parse_atomic_notes(xml);
    for issue in report.issues.iter().filter(|issue| issue.issue_type == "PIANO_FAST_LARGE_JUMP").take(24) {
        for patch in warning_repair_patches_for_issue(xml, &notes, issue, 8) {
            let (patched, patch_receipt) = apply_atomic_patch(xml, &patch);
                    let patched = PatchedXml::with_exact(patched, patch_receipt);
            push_corpus_variant(&mut variants, &mut seen, format!("corpus_han_rhy_{}", patch.name), xml, patched, limit);
            if variants.len() >= limit { return variants; }
        }
    }
    variants
}

fn warning_repair_patches_for_issue(_xml: &str, notes: &[AtomicNote], issue: &Issue, limit: usize) -> Vec<AtomicPatch> {
    let mut patches = Vec::new();
    for patch in warning_repair_patches(_xml, &CheckReport {
        tool: String::new(),
        version: String::new(),
        score_title: None,
        summary: ReportSummary { status: ReportStatus::Blocked, blockers: 0, warnings: 1, hints: 0 },
        issues: vec![issue.clone()],
    }, limit.saturating_mul(2)) {
        patches.push(patch);
        if patches.len() >= limit { break; }
    }
    if patches.is_empty() && issue.issue_type == "PIANO_FAST_LARGE_JUMP" {
        for choice in restify_choices_for_issue(notes, issue).into_iter().take(limit) {
            if let Some((start, end)) = choice.ranges.first().copied() {
                patches.push(AtomicPatch {
                    name: format!("restify_{}", choice.name),
                    note_start: start,
                    note_end: end,
                    target_staff: String::new(),
                    target_voice: String::new(),
                    staff_value_range: None,
                    voice_value_range: None,
                    action: AtomicPatchAction::RestifyHidden,
                    insert_after: None,
                    anchor_duration: None,
                    material_transfer_carrier: None,
                });
            }
        }
    }
    patches
}

fn push_corpus_variant(
    variants: &mut Vec<PatchVariant>,
    seen: &mut BTreeSet<String>,
    name: String,
    base_xml: &str,
    patched: PatchedXml,
    limit: usize,
) {
    if variants.len() >= limit || patched.xml == base_xml {
        return;
    }
    if seen.insert(patched.xml.clone()) {
        variants.push(PatchVariant { name, patched });
    }
}

fn apply_corpus_voice_overfull_crop(xml: &str, issues: &[&Issue]) -> PatchedXml {
    let notes = parse_atomic_notes(xml);
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    for issue in issues {
        let Some(staff) = issue.staff.as_deref() else { continue; };
        let Some(voice) = issue.voice.as_deref() else { continue; };
        let Some(expected) = parse_expected_divisions_value(issue.expected_divisions.as_deref()) else { continue; };
        if expected <= 0 { continue; }
        for note in notes.iter().filter(|note| note.measure == issue.measure && note.staff == staff && note.voice == voice && note.duration > 0) {
            let end = note.start + note.duration;
            if note.start >= expected {
                replacements.push((note.note_start, note.note_end, String::new()));
            } else if end > expected {
                let new_duration = (expected - note.start).max(0);
                if let Some((start, end)) = child_value_range(&xml[note.note_start..note.note_end], note.note_start, "duration") {
                    replacements.push((start, end, new_duration.to_string()));
                }
            }
        }
    }
    if replacements.is_empty() {
        return PatchedXml::with_exact(xml.to_string(), None);
    }
    replacements.sort_by_key(|a| std::cmp::Reverse((a.0, a.1)));
    replacements.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    let (cropped, crop_receipt) =
        crate::structural_write_authority::apply_voice_overfull_crop_edits_authorized(
            xml,
            replacements,
        );
    let out = cropped.expect("source-bound corpus voice crop authorization");
    // Nach Dauer-Crops wird die sichtbare Rhythmik erneut normalisiert, damit
    // Notenwerte, Pausen und Typ/Dot-Notation wieder zusammenpassen.
    let normalized_result = normalize_visible_rhythm_musicxml(&out);
    let normalized = normalized_result.xml;
    crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
        &out,
        &normalized,
    );
    // Lineage: EXACT #25 (crop) -> BLOCK #52 (visible-rhythm normalization).
    let mut lineage_receipts = master_state_identity::OwnerReceipts::default();
    lineage_receipts.push_exact(crop_receipt);
    lineage_receipts.push_visible_block_with_births(normalized_result.visible_rhythm_block_identity,
        normalized_result.stats.physical_births.into_iter().next(), &out)
        .expect("Crop must carry the normalizer's original creation capture");
    PatchedXml {
        xml: normalized,
        lineage_receipts,
    }
}

fn parse_expected_divisions_value(value: Option<&str>) -> Option<i64> {
    let value = value?;
    if let Some((num, den)) = value.split_once('/') {
        let numerator = num.trim().parse::<i64>().ok()?;
        let denominator = den.split_whitespace().next()?.trim().parse::<i64>().ok()?.max(1);
        return Some(numerator / denominator);
    }
    value
        .split(|ch: char| !ch.is_ascii_digit() && ch != '-')
        .filter(|part| !part.is_empty())
        .find_map(|part| part.parse::<i64>().ok())
}

fn run_atomic_critical_repair(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
) -> anyhow::Result<()> {
    let phase_started = Instant::now();

    for round in 0..12 {
        if critical_count(&best.final_result) == 0 {
            break;
        }
        let patches = atomic_repair_patches(best.layout_xml(), &best.final_result, 96);
        if patches.is_empty() {
            break;
        }
        let patch_count = patches.len();
        console_progress(format!("SafeAtomicRepair: Runde {} prüft {} Einzelpatches", round + 1, patch_count));

        let inputs = atomic_patches_to_candidate_inputs(patches, best.layout_xml());
        let evaluated = evaluate_patched_candidates_parallel(
            "SafeAtomicRepair",
            inputs,
            best,
            next_iteration,
            options,
            parallel,
            phase_started,
        )?;

        let mut round_best: Option<SolverCandidate> = None;
        let mut round_rejected = Vec::new();
        for candidate in evaluated {
            let beats_current = atomic_candidate_improves(&candidate, best);
            let beats_round_best = round_best
                .as_ref()
                .is_none_or(|current| atomic_candidate_improves(&candidate, current));
            if beats_current && beats_round_best {
                if let Some(previous_round_best) = round_best.replace(candidate) {
                    round_rejected.push(previous_round_best);
                }
            } else {
                round_rejected.push(candidate);
            }
        }

        candidates.extend(round_rejected);
        let Some(accepted_candidate) = round_best else { break; };
        let previous_best = std::mem::replace(best, accepted_candidate);
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        candidates.push(previous_best);
    }
    Ok(())
}

fn atomic_patches_to_candidate_inputs(patches: Vec<AtomicPatch>, base_xml: &str) -> Vec<PatchedCandidateInput> {
    let mut inputs = Vec::new();
    for (source_index, patch) in patches.into_iter().enumerate() {
        let (patched_xml, patch_receipt) = apply_atomic_patch(base_xml, &patch);
        let mut lineage_receipts = crate::master_state_identity::OwnerReceipts::default();
        lineage_receipts.push_exact(patch_receipt);
        if patched_xml == base_xml {
            continue;
        }
        let material_declared_effects = match patch.action {
            AtomicPatchAction::MoveStaff => CandidateMaterialDeclaredEffects::staff_relocation(),
            AtomicPatchAction::SafeTransferChord => {
                CandidateMaterialDeclaredEffects::staff_voice_relocation()
            }
            AtomicPatchAction::RestifyHidden => {
                CandidateMaterialDeclaredEffects::attack_existence_only()
            }
        };
        inputs.push(PatchedCandidateInput {
            source_index,
            name: patch.name,
            xml: patched_xml,
            material_declared_effects: Some(material_declared_effects),
            material_transfer_carriers: patch.material_transfer_carrier.into_iter().collect(),
            lineage_receipts,
        });
    }
    inputs
}

fn final_variants_to_candidate_inputs(variants: Vec<PatchVariant>, base_xml: &str) -> Vec<PatchedCandidateInput> {
    let mut inputs = Vec::new();
    for (source_index, PatchVariant { name, patched }) in variants.into_iter().enumerate() {
        if patched.xml == base_xml {
            continue;
        }
        inputs.push(PatchedCandidateInput {
            source_index,
            name,
            xml: patched.xml,
            material_declared_effects: None,
            material_transfer_carriers: Vec::new(),
            lineage_receipts: patched.lineage_receipts,
        });
    }
    inputs
}
























fn han_repair_candidate_risk(cand: &HiddenChordToneRecoveryCandidate) -> i64 {
    let midi = cand.midi.unwrap_or(64);
    let mut risk = 0_i64;
    if cand.staff == 1 && midi < 60 {
        risk += (60 - midi) as i64 * 8;
    }
    if cand.staff == 2 && midi > 72 {
        risk += (midi - 72) as i64 * 8;
    }
    if midi <= 42 || midi >= 90 {
        risk += 90;
    }
    if cand.duration < 24 {
        risk += 12;
    }
    risk
}








fn hand_plan_candidate_risk(cand: &HiddenChordToneRecoveryCandidate) -> i64 {
    let midi = cand.midi.unwrap_or(64);
    let mut risk = han_repair_candidate_risk(cand) / 2;
    if cand.staff == 1 {
        if midi < 60 { risk += (60 - midi) as i64 * 10; }
        if midi > 96 { risk += (midi - 96) as i64 * 6; }
    } else {
        if midi > 76 { risk += (midi - 76) as i64 * 11; }
        if midi < 36 { risk += (36 - midi) as i64 * 8; }
    }
    if cand.duration < 24 { risk += 18; }
    if cand.start_tick % 24 != 0 { risk += 10; }
    risk
}

















fn evaluate_patched_candidates_parallel(
    label: &str,
    inputs: Vec<PatchedCandidateInput>,
    base: &SolverCandidate,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
    phase_started: Instant,
) -> anyhow::Result<Vec<SolverCandidate>> {
    if inputs.is_empty() {
        return Ok(Vec::new());
    }

    let original_total = inputs.len();
    let iteration_start = *next_iteration;
    *next_iteration += original_total;

    let mut skipped_preexisting_transfer_targets = 0usize;
    let mut eligible_inputs = Vec::<(usize, PatchedCandidateInput)>::with_capacity(original_total);
    for (evaluation_index, input) in inputs.into_iter().enumerate() {
        if base.material_lineage.authority_bound && !input.material_transfer_carriers.is_empty() {
            match preflight_candidate_material_representation_transfers(
                base.layout_xml(),
                base,
                &input.material_transfer_carriers,
            )
            .with_context(|| {
                format!(
                    "{label} Patch {} RepresentationTransfer-Eligibility konnte nicht geprüft werden",
                    input.source_index + 1
                )
            })? {
                CandidateMaterialRepresentationTransferPreflight::Eligible(_) => {}
                CandidateMaterialRepresentationTransferPreflight::PreexistingTarget => {
                    skipped_preexisting_transfer_targets += 1;
                    continue;
                }
            }
        }
        eligible_inputs.push((evaluation_index, input));
    }

    if skipped_preexisting_transfer_targets > 0 {
        console_progress(format!(
            "{label}: {skipped_preexisting_transfer_targets} Kandidaten wegen bereits vorhandenem RepresentationTransfer-Ziel fail-closed verworfen"
        ));
    }
    if eligible_inputs.is_empty() {
        return Ok(Vec::new());
    }

    let total = eligible_inputs.len();
    let worker_count = parallel.workers.min(total).max(1);

    if worker_count == 1 {
        let mut out: Vec<(usize, SolverCandidate)> = Vec::with_capacity(total);
        report_progress(label, 0, total, worker_count, phase_started);
        for (eligible_index, (evaluation_index, input)) in eligible_inputs.into_iter().enumerate() {
            let PatchedCandidateInput {
                source_index,
                name,
                xml,
                material_declared_effects,
                material_transfer_carriers,
                lineage_receipts,
            } = input;
            let candidate = build_atomic_candidate(
                iteration_start + evaluation_index,
                &name,
                AtomicPatchedDocument { xml, lineage_receipts },
                material_declared_effects,
                &material_transfer_carriers,
                base,
                options,
            )?;
            let done = eligible_index + 1;
            if should_report_progress_tick(done, total) {
                report_progress(label, done, total, worker_count, phase_started);
            }
            out.push((source_index, candidate));
        }
        out.sort_by_key(|(source_index, _)| *source_index);
        return Ok(out.into_iter().map(|(_, candidate)| candidate).collect());
    }

    console_progress(format!(
        "{label}: prüfe {total} zulässige Kandidaten parallel mit {worker_count} Workern"
    ));
    report_progress(label, 0, total, worker_count, phase_started);

    // Each eligible input is moved into exactly one worker evaluation, so its
    // owner receipts are consumed once and never cloned.
    let eligible_inputs = eligible_inputs
        .into_iter()
        .map(|(evaluation_index, input)| (evaluation_index, Mutex::new(Some(input))))
        .collect::<Vec<_>>();
    let next = AtomicUsize::new(0);
    let completed = AtomicUsize::new(0);
    let results: Mutex<Vec<(usize, anyhow::Result<SolverCandidate>)>> =
        Mutex::new(Vec::with_capacity(total));

    thread::scope(|scope| {
        for _ in 0..worker_count {
            let next = &next;
            let completed = &completed;
            let results = &results;
            let eligible_inputs = &eligible_inputs;
            scope.spawn(move || {
                loop {
                    let eligible_index = next.fetch_add(1, Ordering::Relaxed);
                    if eligible_index >= eligible_inputs.len() {
                        break;
                    }
                    let (evaluation_index, input_slot) = &eligible_inputs[eligible_index];
                    let PatchedCandidateInput {
                        source_index,
                        name,
                        xml,
                        material_declared_effects,
                        material_transfer_carriers,
                        lineage_receipts,
                    } = input_slot
                        .lock()
                        .expect("ParallelPatchEvaluator Eingabe-Mutex vergiftet")
                        .take()
                        .expect("ParallelPatchEvaluator: jede Eingabe wird genau einmal bewertet");
                    let result = build_atomic_candidate(
                        iteration_start + *evaluation_index,
                        &name,
                        AtomicPatchedDocument {
                            xml,
                            lineage_receipts,
                        },
                        material_declared_effects,
                        &material_transfer_carriers,
                        base,
                        options,
                    );
                    results
                        .lock()
                        .expect("ParallelPatchEvaluator Mutex vergiftet")
                        .push((source_index, result));
                    let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
                    if should_report_progress_tick(done, total) {
                        report_progress(label, done, total, worker_count, phase_started);
                    }
                }
            });
        }
    });

    let mut results = results
        .into_inner()
        .expect("ParallelPatchEvaluator Mutex vergiftet");
    results.sort_by_key(|(source_index, _)| *source_index);
    let mut out = Vec::with_capacity(results.len());
    for (source_index, result) in results {
        out.push(
            result.with_context(|| format!("{} Patch {} fehlgeschlagen", label, source_index + 1))?,
        );
    }
    Ok(out)
}

fn atomic_candidate_improves(candidate: &SolverCandidate, current: &SolverCandidate) -> bool {
    if candidate.final_result.summary.blockers != current.final_result.summary.blockers {
        return candidate.final_result.summary.blockers < current.final_result.summary.blockers;
    }
    let candidate_critical = critical_count(&candidate.final_result);
    let current_critical = critical_count(&current.final_result);
    if candidate_critical < current_critical {
        return candidate.final_result.summary.warnings <= current.final_result.summary.warnings + 8;
    }
    if candidate_critical > current_critical {
        return false;
    }

    let candidate_max = max_critical_hardness(&candidate.final_result);
    let current_max = max_critical_hardness(&current.final_result);
    if candidate_max < current_max {
        return candidate.final_result.summary.warnings <= current.final_result.summary.warnings + 2;
    }
    if candidate_max > current_max {
        return false;
    }

    candidate.final_result.summary.warnings < current.final_result.summary.warnings
}

// Observation only. Caller supplies the exact current candidate and boundary;
// the batch child forwards stderr even when its stdout projection is filtered.
// Observation handoff only: original Master adoption still consumes its receipts
// at each caller. No stale predecessor snapshot survives a changed document.
fn continue_candidate_owned_rhythm(base: &SolverCandidate, candidate: &mut SolverCandidate,
    receipts: &master_state_identity::OwnerReceipts, after: &str, boundary: &str) {
    let continued = base.arrangement_stats.owned_rhythm.as_ref()
        .ok_or_else(|| anyhow!("{}", base.arrangement_stats.owned_rhythm_unknown.as_deref()
            .unwrap_or("Missing base owned rhythm")))
        .and_then(|state| receipts.continue_owned_rhythm(state, after)
            .map_err(|error| anyhow!("{boundary}/OriginalOwners: {error}")));
    match continued {
        Ok(state) => {
            candidate.arrangement_stats.owned_rhythm_unknown = state.require_complete_course_context(after).err().map(|e| e.to_string());
            candidate.arrangement_stats.owned_rhythm = Some(std::sync::Arc::new(state));
        },
        Err(error) => {
            candidate.arrangement_stats.owned_rhythm = None;
            candidate.arrangement_stats.owned_rhythm_unknown = Some(error.to_string());
        },
    }
}

fn owned_rhythm_diagnostic_line(
    project: &str,
    candidate: &SolverCandidate,
) -> String {
    let status = match &candidate.arrangement_stats.owned_rhythm {
        Some(owned) => match owned.require_complete_course_context(candidate.layout_xml()) {
            Ok(()) => "complete_received_course_context".to_owned(),
            Err(error) => format!("Unknown: {error}"),
        },
        None => format!("Unknown: {}", candidate.arrangement_stats.owned_rhythm_unknown.as_deref()
            .unwrap_or("no received context")),
    };
    format!(
        "ETUSCHLOPF: OwnedRhythmicContinuation: project={project:?}; boundary=ArrangementSolverFinal; context={status:?}; CanonicalPitchedRoleClosure=none; ChannelAllocationPermit=none; MusicMutation=none")
}

fn build_atomic_candidate(
    iteration: usize,
    patch_name: &str,
    patched: AtomicPatchedDocument,
    material_declared_effects: Option<CandidateMaterialDeclaredEffects>,
    material_transfer_carriers: &[CandidateMaterialRepresentationTransferCarrier],
    base: &SolverCandidate,
    options: &CheckOptions,
) -> anyhow::Result<SolverCandidate> {
    let AtomicPatchedDocument {
        xml,
        lineage_receipts,
    } = patched;
    // v0.1.68: Atomic-/Warning-/Final-Patches dürfen neue Rest-/Dauerprobleme
    // erzeugen. Deshalb vor der harten Prüfung erneut durch den
    // VisibleRhythmLedger normalisieren.
    //
    // r687: SafeAtomic-VisibleRhythm-ForwardCarrier-Closure. Der produktive AtomicPatch und
    // die nachgeschaltete VisibleRhythm-Normalisierung sind zwei verschiedene
    // Writer-Grenzen. Die Material-Lineage wird deshalb zuerst gegen das rohe
    // Patch-XML geschlossen und erst danach representation-only gegen das
    // normalisierte XML. So beweist ein Move-/Transfer-Carrier ausschließlich
    // den AtomicPatch und wird nicht für zusätzliche Ledger-Deltas missbraucht.
    let mut normalized = normalize_visible_rhythm_musicxml(&xml);
    // Original owner capture, independent of CandidateMaterial matching.
    // These identities supply no missing course, role or adoption authority.
    let patch_births = lineage_receipts.physical_birth_readback(base.layout_xml(), &xml);
    let continued_rhythm = base.arrangement_stats.owned_rhythm.as_ref()
        .ok_or_else(|| anyhow!("{}", base.arrangement_stats.owned_rhythm_unknown.as_deref()
            .unwrap_or("No received owned rhythm at AtomicPatch input")))
        .and_then(|state| { state.validate_binding(base.layout_xml())?;
            lineage_receipts.continue_owned_rhythm(state, &xml)
                .map_err(|error| anyhow!("AtomicPatch/OriginalOwners: {error}")) })
        .and_then(|state| match normalized.stats.physical_births.first() {
            Some(births) => state.continue_visible(births)
                .map_err(|error| anyhow!("AtomicPatch/VisibleRhythm: {error}")),
            None => { state.validate_binding(&normalized.xml)?; Ok(state) },
        });
    for births in &normalized.stats.physical_births { births.validate_binding(&xml, &normalized.xml)?; }
    // BLOCK #52 exists exactly when the normalizer changed the document.
    let visible_rhythm_receipt = normalized.visible_rhythm_block_identity.take();
    match visible_rhythm_receipt.as_ref() {
        Some(receipt) => receipt
            .validate_block_transition::<crate::arrangement::VisibleRhythmBlockMint>(
                &xml,
                &normalized.xml,
            )?,
        None => anyhow::ensure!(
            normalized.xml == xml,
            "VisibleRhythm BLOCK #52 receipt missing for a changed document"
        ),
    }
    crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
        &xml,
        &normalized.xml,
    );
    let checked_score = parse_musicxml(&normalized.xml)
        .with_context(|| format!("SafeAtomicRepair-Patch {patch_name} konnte nicht geprüft werden"))?;
    let checked = run_checks(&checked_score, options);
    let mut candidate = base.clone();
    match continued_rhythm {
        Ok(state) => {
            candidate.arrangement_stats.owned_rhythm_unknown = state.require_complete_course_context(&normalized.xml).err().map(|e| e.to_string());
            candidate.arrangement_stats.owned_rhythm = Some(std::sync::Arc::new(state));
        },
        Err(error) => {
            candidate.arrangement_stats.owned_rhythm = None;
            candidate.arrangement_stats.owned_rhythm_unknown = Some(error.to_string());
        },
    }
    match patch_births {
        Ok(births) => candidate.arrangement_stats.physical_births.extend(births),
        Err(_) => candidate.arrangement_stats.physical_birth_capture_unknown += 1,
    }
    candidate.arrangement_stats.physical_births.extend(normalized.stats.physical_births.iter().cloned());
    // Lineage: base state -> the patch owners' receipts -> BLOCK #52. Missing
    // coverage is a fatal sovereignty violation at adoption.
    let mut visible_rhythm_receipts = crate::master_state_identity::OwnerReceipts::default();
    visible_rhythm_receipts.push_block(visible_rhythm_receipt);
    let mut visible_rhythm_receipts = Some(visible_rhythm_receipts);
    if let Some(diagnostic) = lineage_receipts.coverage_diagnostic(base.layout_xml(), &xml) {
        // Best-effort failure output only. Adoption below still decides and
        // aborts on missing coverage; no candidate is discarded or substituted.
        let _ = std::io::Write::write_fmt(
            &mut std::io::stderr().lock(),
            format_args!(
                "ETUSCHLOPF: AtomicPatchReceiptCoverage: iteration={iteration}; patch={patch_name:?}; {diagnostic}\n",
            ),
        );
    }
    candidate.iteration = iteration;
    candidate.profile_name = patch_name.to_string();
    candidate.profile_contract = candidate.profile_contract.without_projected_parameters();
    candidate.profile_lineage = candidate.profile_lineage.with_hand_repaired();
    candidate.final_result = checked;
    candidate.master_state.adopt_materialized_document(
        xml.clone(),
        lineage_receipts,
        "SafeAtomicCandidateProducer/Patch",
    );

    if let Some(declared) = material_declared_effects {
        let reconcile_result = if material_transfer_carriers.is_empty() {
            reconcile_candidate_material_lineage(
                &mut candidate,
                "SafeAtomicCandidateProducer",
                declared,
            )
        } else {
            reconcile_candidate_material_lineage_with_representation_transfers(
                base.layout_xml(),
                &mut candidate,
                "SafeAtomicTransferCandidateProducer",
                declared,
                material_transfer_carriers,
            )
        };
        reconcile_result.with_context(|| {
            let last_receipt_owner = base
                .material_lineage
                .receipts
                .last()
                .map(|receipt| receipt.owner.as_str())
                .unwrap_or("none");
            format!(
                "SafeAtomicCandidateProducerLineageAudit patch_name={} base_iteration={} base_profile_name={} base_profile_lineage={:?} source_selection_candidate_id={} source_selection_origin_stage={} carrier_count={} base_material_fragments={} base_receipt_count={} base_last_receipt_owner={} base_state_fp={:016x}",
                patch_name,
                base.iteration,
                base.profile_name,
                base.profile_lineage,
                base.midi_seed_stats.source_selection_candidate_id,
                base.midi_seed_stats.source_selection_origin_stage,
                material_transfer_carriers.len(),
                base.material_lineage.fragments.len(),
                base.material_lineage.receipts.len(),
                last_receipt_owner,
                base.material_lineage.state_fingerprint_fnv64
            )
        })?;

        if candidate.layout_xml() != normalized.xml {
            candidate.master_state.adopt_materialized_document(
                normalized.xml.clone(),
                visible_rhythm_receipts.take().unwrap_or_default(),
                "SafeAtomicCandidateProducer/VisibleRhythm",
            );
            reconcile_candidate_material_lineage_with_visible_rhythm_forward_rows(
                &xml,
                &mut candidate,
                "SafeAtomicVisibleRhythmNormalizer",
                &normalized.stats.representation_lineage_rows,
                &normalized.stats.representation_post_raw_note_counts,
            )
            .with_context(|| {
                format!(
                    "SafeAtomicVisibleRhythmOwnerBoundaryAudit patch_name={} notes_normalized={} rests_inserted={} values_split={} measures_completed={} representation_lineage_rows={}",
                    patch_name,
                    normalized.stats.visible_notes_normalized,
                    normalized.stats.visible_rhythm_rests_inserted,
                    normalized.stats.visible_rhythm_values_split,
                    normalized.stats.visible_rhythm_measures_completed,
                    normalized.stats.representation_lineage_rows.len()
                )
            })?;
        } else {
            candidate.master_state.adopt_materialized_document(
                normalized.xml.clone(),
                visible_rhythm_receipts.take().unwrap_or_default(),
                "SafeAtomicCandidateProducer/VisibleRhythm",
            );
        }
    } else {
        if !material_transfer_carriers.is_empty() {
            return Err(anyhow!(
                "SafeAtomicCandidateProducer: representation transfer carrier has no declared material effects"
            ));
        }
        candidate.master_state.adopt_materialized_document(
                normalized.xml.clone(),
                visible_rhythm_receipts.take().unwrap_or_default(),
                "SafeAtomicCandidateProducer/VisibleRhythm",
            );
    }

    candidate.arrangement_stats.visible_notes_normalized += normalized.stats.visible_notes_normalized;
    candidate.arrangement_stats.visible_rhythm_rests_inserted += normalized.stats.visible_rhythm_rests_inserted;
    candidate.arrangement_stats.visible_rhythm_values_split += normalized.stats.visible_rhythm_values_split;
    candidate.arrangement_stats.visible_rhythm_measures_completed += normalized.stats.visible_rhythm_measures_completed;
    candidate.midi_seed_stats.solver_profile = patch_name.to_string();
    Ok(candidate)
}

fn build_editorial_voice_trial_candidate(
    _iteration: usize,
    patch_name: &str,
    patched: PatchedXml,
    base: &SolverCandidate,
    options: &CheckOptions,
) -> anyhow::Result<(CheckReport, structural_write_authority::MasterXmlReplacementRequest)> {
    let PatchedXml {
        xml,
        lineage_receipts,
    } = patched;
    // v0.1.188: NormalizedFullRewriteAcceptanceTrial. Editorial-Voice-Trials
    // bleiben isoliert; Full-Rewrite-Kandidaten werden final-clean normalisiert und dürfen bei engem Gate einmalig übernommen werden. Der eigentliche Frozen-Baseline-
    // Vergleich wird im Scope-Trace vorgenommen: out-of-scope Rhythmus-/
    // Ledger-Befunde, die nicht im Zielbereich liegen und aus dem
    // Vollstimmen-Rewrite stammen, werden nicht als musikalischer
    // Ziel-Ablehnungsgrund gewertet.
    let checked_score = parse_musicxml(&xml)
        .with_context(|| format!("EditorialVoice-IsolationTrial {patch_name} konnte nicht geprüft werden"))?;
    let checked = run_checks(&checked_score, options);
    let request = structural_write_authority::MasterXmlReplacementRequest::bind_with_owner_receipts(
        base.layout_xml(),
        xml,
        lineage_receipts,
    );
    Ok((checked, request))
}


fn run_warning_elimination_repair(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
) -> anyhow::Result<()> {
    let phase_started = Instant::now();

    // Runden bleiben deterministisch seriell; die Kandidaten innerhalb einer
    // Runde werden ab v0.1.80 parallel bewertet.
    for round in 0..24 {
        if best.final_result.summary.warnings == 0 {
            break;
        }
        let patches = warning_repair_patches(best.layout_xml(), &best.final_result, 256);
        if patches.is_empty() {
            break;
        }
        let patch_count = patches.len();
        console_progress(format!("StrictWarningCleaner: Runde {} prüft {} Einzelpatches", round + 1, patch_count));

        let inputs = atomic_patches_to_candidate_inputs(patches, best.layout_xml());
        let evaluated = evaluate_patched_candidates_parallel(
            "StrictWarningCleaner",
            inputs,
            best,
            next_iteration,
            options,
            parallel,
            phase_started,
        )?;

        let mut round_best: Option<SolverCandidate> = None;
        let mut round_rejected = Vec::new();
        for candidate in evaluated {
            let beats_current = decide_late_playability_best_candidate_adoption(
                LatePlayabilityBestCandidateSelectionProjection::StrictWarningCleaner,
                late_playability_best_candidate_decision_facts(&candidate),
                late_playability_best_candidate_decision_facts(best),
            );
            let beats_round_best = round_best
                .as_ref()
                .is_none_or(|current| decide_late_playability_best_candidate_adoption(
                    LatePlayabilityBestCandidateSelectionProjection::StrictWarningCleaner,
                    late_playability_best_candidate_decision_facts(&candidate),
                    late_playability_best_candidate_decision_facts(current),
                ));
            if beats_current && beats_round_best {
                if let Some(previous_round_best) = round_best.replace(candidate) {
                    round_rejected.push(previous_round_best);
                }
            } else {
                round_rejected.push(candidate);
            }
        }

        candidates.extend(round_rejected);
        let Some(accepted_candidate) = round_best else { break; };
        let previous_best = std::mem::replace(best, accepted_candidate);
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        candidates.push(previous_best);
    }
    Ok(())
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LatePlayabilityBestCandidateSelectionProjection {
    StrictWarningCleaner,
    FinalCriticalTaktRepair,
    FinalPlayabilityCleaner,
    FinalPlayabilityPolish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LatePlayabilityBestCandidateDecisionFacts {
    blockers: usize,
    critical: usize,
    warnings: usize,
    max_critical_hardness: i64,
    playability_warning_load: i64,
    max_playability_warning_hardness: i64,
    decision_key: CandidateDecisionKey,
}

fn late_playability_best_candidate_decision_facts(
    candidate: &SolverCandidate,
) -> LatePlayabilityBestCandidateDecisionFacts {
    LatePlayabilityBestCandidateDecisionFacts {
        blockers: candidate.final_result.summary.blockers,
        critical: critical_count(&candidate.final_result),
        warnings: candidate.final_result.summary.warnings,
        max_critical_hardness: max_critical_hardness(&candidate.final_result),
        playability_warning_load: playability_warning_load(&candidate.final_result),
        max_playability_warning_hardness: max_playability_warning_hardness(
            &candidate.final_result,
        ),
        decision_key: candidate_decision_key(candidate),
    }
}

fn decide_late_playability_best_candidate_adoption(
    projection: LatePlayabilityBestCandidateSelectionProjection,
    candidate: LatePlayabilityBestCandidateDecisionFacts,
    current: LatePlayabilityBestCandidateDecisionFacts,
) -> bool {
    if candidate.blockers != current.blockers {
        return candidate.blockers < current.blockers;
    }
    if candidate.blockers > 0 {
        return false;
    }

    match projection {
        LatePlayabilityBestCandidateSelectionProjection::StrictWarningCleaner => {
            if candidate.critical > current.critical {
                return false;
            }
            if candidate.warnings < current.warnings {
                return true;
            }
            if candidate.warnings > current.warnings {
                return false;
            }
            if candidate.critical < current.critical {
                return true;
            }
            candidate.max_critical_hardness < current.max_critical_hardness
        }
        LatePlayabilityBestCandidateSelectionProjection::FinalCriticalTaktRepair => {
            if candidate.critical < current.critical {
                return candidate.warnings <= current.warnings + 4;
            }
            if candidate.critical > current.critical {
                return false;
            }
            if candidate.warnings < current.warnings {
                return true;
            }
            if candidate.warnings > current.warnings {
                return false;
            }
            candidate.max_critical_hardness < current.max_critical_hardness
        }
        LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityCleaner => {
            if candidate.critical < current.critical {
                return candidate.warnings <= current.warnings + 6;
            }
            if candidate.critical > current.critical {
                return false;
            }
            if candidate.warnings < current.warnings {
                return true;
            }
            if candidate.warnings > current.warnings {
                return false;
            }
            if candidate.max_critical_hardness < current.max_critical_hardness {
                return true;
            }
            if candidate.max_critical_hardness > current.max_critical_hardness {
                return false;
            }
            candidate.decision_key < current.decision_key
        }
        LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityPolish => {
            if candidate.critical != current.critical {
                return candidate.critical < current.critical;
            }
            if candidate.warnings != current.warnings {
                return candidate.warnings < current.warnings;
            }
            if candidate.playability_warning_load != current.playability_warning_load {
                return candidate.playability_warning_load < current.playability_warning_load;
            }
            if candidate.max_playability_warning_hardness
                != current.max_playability_warning_hardness
            {
                return candidate.max_playability_warning_hardness
                    < current.max_playability_warning_hardness;
            }
            candidate.decision_key < current.decision_key
        }
    }
}

#[cfg(test)]
mod late_playability_best_candidate_selection_decision_owner_tests {
    use super::{
        CandidateDecisionKey,
        LatePlayabilityBestCandidateDecisionFacts,
        LatePlayabilityBestCandidateSelectionProjection,
        decide_late_playability_best_candidate_adoption,
    };

    fn facts(
        critical: usize,
        warnings: usize,
    ) -> LatePlayabilityBestCandidateDecisionFacts {
        LatePlayabilityBestCandidateDecisionFacts {
            blockers: 0,
            critical,
            warnings,
            max_critical_hardness: 24,
            playability_warning_load: 2_400,
            max_playability_warning_hardness: 24,
            decision_key: CandidateDecisionKey {
                blockers: 0,
                critical_count: critical,
                max_critical_hardness: 24,
                warnings,
                hints: 0,
                retention_rank: std::cmp::Reverse(70_000),
                human_potential_notes_moved: 0,
            },
        }
    }

    #[test]
    fn strict_warning_projection_keeps_zero_warning_slack() {
        let current = facts(1, 1);
        let candidate = facts(0, 2);
        assert!(!decide_late_playability_best_candidate_adoption(
            LatePlayabilityBestCandidateSelectionProjection::StrictWarningCleaner,
            candidate,
            current,
        ));
    }

    #[test]
    fn final_critical_projection_pins_warning_slack_four() {
        let current = facts(1, 2);
        let mut candidate = facts(0, 6);
        assert!(decide_late_playability_best_candidate_adoption(
            LatePlayabilityBestCandidateSelectionProjection::FinalCriticalTaktRepair,
            candidate,
            current,
        ));
        candidate.warnings = 7;
        assert!(!decide_late_playability_best_candidate_adoption(
            LatePlayabilityBestCandidateSelectionProjection::FinalCriticalTaktRepair,
            candidate,
            current,
        ));
    }

    #[test]
    fn final_playability_projection_pins_warning_slack_six() {
        let current = facts(1, 2);
        let mut candidate = facts(0, 8);
        assert!(decide_late_playability_best_candidate_adoption(
            LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityCleaner,
            candidate,
            current,
        ));
        candidate.warnings = 9;
        assert!(!decide_late_playability_best_candidate_adoption(
            LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityCleaner,
            candidate,
            current,
        ));
    }

    #[test]
    fn polish_projection_keeps_playability_facts_before_non_additive_fallback() {
        let current = facts(1, 1);

        let mut candidate = facts(0, 99);
        assert!(decide_late_playability_best_candidate_adoption(
            LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityPolish,
            candidate,
            current,
        ));

        candidate = current;
        candidate.playability_warning_load -= 1;
        candidate.decision_key.hints += 1;
        assert!(decide_late_playability_best_candidate_adoption(
            LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityPolish,
            candidate,
            current,
        ));
    }
}

fn run_final_critical_takt_repair(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
) -> anyhow::Result<()> {
    let phase_started = Instant::now();

    for round in 0..16 {
        if best.final_result.summary.warnings == 0 {
            break;
        }
        let variants = final_critical_takt_variants(best.layout_xml(), &best.final_result, 512);
        if variants.is_empty() {
            break;
        }
        let variant_count = variants.len();
        console_progress(format!("FinalCriticalTaktRepair: Runde {} prüft {} Taktvarianten", round + 1, variant_count));

        let inputs = final_variants_to_candidate_inputs(variants, best.layout_xml());
        let evaluated = evaluate_patched_candidates_parallel(
            "FinalCriticalTaktRepair",
            inputs,
            best,
            next_iteration,
            options,
            parallel,
            phase_started,
        )?;

        let mut round_best: Option<SolverCandidate> = None;
        let mut round_rejected = Vec::new();
        for candidate in evaluated {
            let beats_current = decide_late_playability_best_candidate_adoption(
                LatePlayabilityBestCandidateSelectionProjection::FinalCriticalTaktRepair,
                late_playability_best_candidate_decision_facts(&candidate),
                late_playability_best_candidate_decision_facts(best),
            );
            let beats_round_best = round_best
                .as_ref()
                .is_none_or(|current| decide_late_playability_best_candidate_adoption(
                    LatePlayabilityBestCandidateSelectionProjection::FinalCriticalTaktRepair,
                    late_playability_best_candidate_decision_facts(&candidate),
                    late_playability_best_candidate_decision_facts(current),
                ));
            if beats_current && beats_round_best {
                if let Some(previous_round_best) = round_best.replace(candidate) {
                    round_rejected.push(previous_round_best);
                }
            } else {
                round_rejected.push(candidate);
            }
        }

        candidates.extend(round_rejected);
        let Some(accepted_candidate) = round_best else { break; };
        let previous_best = std::mem::replace(best, accepted_candidate);
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        candidates.push(previous_best);
    }
    Ok(())
}


fn run_final_playability_cleaner(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
) -> anyhow::Result<()> {
    let phase_started = Instant::now();

    for round in 0..18 {
        if best.final_result.summary.warnings == 0 {
            break;
        }
        let variants = final_playability_variants(best.layout_xml(), &best.final_result, 768);
        if variants.is_empty() {
            break;
        }
        let variant_count = variants.len();
        console_progress(format!("FinalPlayabilityCleaner: Runde {} prüft {} generische Spielbarkeitsvarianten", round + 1, variant_count));

        let inputs = final_variants_to_candidate_inputs(variants, best.layout_xml());
        let evaluated = evaluate_patched_candidates_parallel(
            "FinalPlayabilityCleaner",
            inputs,
            best,
            next_iteration,
            options,
            parallel,
            phase_started,
        )?;

        let mut round_best: Option<SolverCandidate> = None;
        let mut round_rejected = Vec::new();
        for candidate in evaluated {
            let beats_current = decide_late_playability_best_candidate_adoption(
                LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityCleaner,
                late_playability_best_candidate_decision_facts(&candidate),
                late_playability_best_candidate_decision_facts(best),
            );
            let beats_round_best = round_best
                .as_ref()
                .is_none_or(|current| decide_late_playability_best_candidate_adoption(
                    LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityCleaner,
                    late_playability_best_candidate_decision_facts(&candidate),
                    late_playability_best_candidate_decision_facts(current),
                ));
            if beats_current && beats_round_best {
                if let Some(previous_round_best) = round_best.replace(candidate) {
                    round_rejected.push(previous_round_best);
                }
            } else {
                round_rejected.push(candidate);
            }
        }

        candidates.extend(round_rejected);
        let Some(accepted_candidate) = round_best else { break; };
        let previous_best = std::mem::replace(best, accepted_candidate);
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        candidates.push(previous_best);
    }
    Ok(())
}

fn run_final_playability_polish(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
) -> anyhow::Result<()> {
    let phase_started = Instant::now();

    for round in 0..10 {
        if best.final_result.summary.warnings == 0 {
            break;
        }
        let variants = final_playability_polish_variants(best.layout_xml(), &best.final_result, 2048);
        if variants.is_empty() {
            break;
        }
        let variant_count = variants.len();
        console_progress(format!(
            "FinalPlayabilityPolish: Runde {} prüft {} generische Restspielbarkeitsvarianten",
            round + 1,
            variant_count
        ));

        let inputs = final_variants_to_candidate_inputs(variants, best.layout_xml());
        let evaluated = evaluate_patched_candidates_parallel(
            "FinalPlayabilityPolish",
            inputs,
            best,
            next_iteration,
            options,
            parallel,
            phase_started,
        )?;

        let mut round_best: Option<SolverCandidate> = None;
        let mut round_rejected = Vec::new();
        for candidate in evaluated {
            let beats_current = decide_late_playability_best_candidate_adoption(
                LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityPolish,
                late_playability_best_candidate_decision_facts(&candidate),
                late_playability_best_candidate_decision_facts(best),
            );
            let beats_round_best = round_best
                .as_ref()
                .is_none_or(|current| decide_late_playability_best_candidate_adoption(
                    LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityPolish,
                    late_playability_best_candidate_decision_facts(&candidate),
                    late_playability_best_candidate_decision_facts(current),
                ));
            if beats_current && beats_round_best {
                if let Some(previous_round_best) = round_best.replace(candidate) {
                    round_rejected.push(previous_round_best);
                }
            } else {
                round_rejected.push(candidate);
            }
        }

        candidates.extend(round_rejected);
        let Some(accepted_candidate) = round_best else { break; };
        let previous_best = std::mem::replace(best, accepted_candidate);
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        candidates.push(previous_best);
    }
    Ok(())
}

fn run_global_hand_trajectory_solver(
    best: &mut SolverCandidate,
    candidates: &mut Vec<SolverCandidate>,
    next_iteration: &mut usize,
    options: &CheckOptions,
    parallel: &ParallelPlan,
) -> anyhow::Result<()> {
    let phase_started = Instant::now();

    for round in 0..8 {
        if best.final_result.summary.warnings == 0 {
            break;
        }
        let variants = global_hand_trajectory_variants(best.layout_xml(), &best.final_result, 4096);
        if variants.is_empty() {
            break;
        }
        let variant_count = variants.len();
        console_progress(format!(
            "GlobalHandTrajectorySolver: Runde {} prüft {} ergonomische Handverlaufsvarianten",
            round + 1,
            variant_count
        ));

        let inputs = final_variants_to_candidate_inputs(variants, best.layout_xml());
        let evaluated = evaluate_patched_candidates_parallel(
            "GlobalHandTrajectorySolver",
            inputs,
            best,
            next_iteration,
            options,
            parallel,
            phase_started,
        )?;

        let mut round_best: Option<SolverCandidate> = None;
        let mut round_rejected = Vec::new();
        for candidate in evaluated {
            let beats_current = decide_late_playability_best_candidate_adoption(
                LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityPolish,
                late_playability_best_candidate_decision_facts(&candidate),
                late_playability_best_candidate_decision_facts(best),
            );
            let beats_round_best = round_best
                .as_ref()
                .is_none_or(|current| decide_late_playability_best_candidate_adoption(
                    LatePlayabilityBestCandidateSelectionProjection::FinalPlayabilityPolish,
                    late_playability_best_candidate_decision_facts(&candidate),
                    late_playability_best_candidate_decision_facts(current),
                ));
            if beats_current && beats_round_best {
                if let Some(previous_round_best) = round_best.replace(candidate) {
                    round_rejected.push(previous_round_best);
                }
            } else {
                round_rejected.push(candidate);
            }
        }

        candidates.extend(round_rejected);
        let Some(accepted_candidate) = round_best else { break; };
        let previous_best = std::mem::replace(best, accepted_candidate);
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        candidates.push(previous_best);
    }
    Ok(())
}

fn final_playability_variants(xml: &str, report: &CheckReport, limit: usize) -> Vec<PatchVariant> {
    let notes = parse_atomic_notes(xml);
    let mut variants: Vec<PatchVariant> = Vec::new();
    let mut issues: Vec<&Issue> = report
        .issues
        .iter()
        .filter(|issue| issue.severity == Severity::Warning)
        .filter(|issue| matches!(issue.issue_type.as_str(), "PIANO_FAST_LARGE_JUMP" | "PIANO_HAND_SPAN_LARGE" | "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF"))
        .collect();
    issues.sort_by_key(|issue| {
        let critical_rank = if is_critical_piano_issue(issue) { 0 } else { 1 };
        let hardness = -issue.actual_divisions.unwrap_or(0);
        (critical_rank, hardness, issue.measure.clone(), issue.staff.clone().unwrap_or_default())
    });

    for (issue_index, issue) in issues.into_iter().enumerate() {
        let patch_prefix = format!("final_playability_{:03}", issue_index + 1);
        match issue.issue_type.as_str() {
            "PIANO_FAST_LARGE_JUMP" => {
                for patch in jump_atomic_patches(&notes, issue) {
                    let (patched, patch_receipt) = apply_atomic_patch(xml, &patch);
                    let patched = PatchedXml::with_exact(patched, patch_receipt);
                    push_final_variant(&mut variants, patch.name, patched, limit);
                }
                if let Some((from_note, to_note)) = find_jump_pair_notes(&notes, issue) {
                    let from_name = from_note.pitch.as_ref().map_or_else(|| "from".to_string(), |p| sanitize_patch_label(&p.label));
                    let to_name = to_note.pitch.as_ref().map_or_else(|| "to".to_string(), |p| sanitize_patch_label(&p.label));
                    push_restify_variant(
                        xml,
                        &mut variants,
                        &format!("{patch_prefix}_hide_to_{to_name}"),
                        &[to_note],
                        limit,
                    );
                    push_restify_variant(
                        xml,
                        &mut variants,
                        &format!("{patch_prefix}_hide_from_{from_name}"),
                        &[from_note],
                        limit,
                    );
                    push_restify_variant(
                        xml,
                        &mut variants,
                        &format!("{patch_prefix}_hide_pair_{from_name}_{to_name}"),
                        &[from_note, to_note],
                        limit,
                    );
                }
            }
            "PIANO_HAND_SPAN_LARGE" => {
                for patch in span_atomic_patches(&notes, issue) {
                    let (patched, patch_receipt) = apply_atomic_patch(xml, &patch);
                    let patched = PatchedXml::with_exact(patched, patch_receipt);
                    push_final_variant(&mut variants, patch.name, patched, limit);
                }
            }
            "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => {
                for patch in overload_atomic_patches(&notes, issue) {
                    let (patched, patch_receipt) = apply_atomic_patch(xml, &patch);
                    let patched = PatchedXml::with_exact(patched, patch_receipt);
                    push_final_variant(&mut variants, patch.name, patched, limit);
                }
            }
            _ => {}
        }
        if variants.len() >= limit {
            break;
        }
    }

    variants.truncate(limit);
    variants
}

fn final_playability_polish_variants(xml: &str, report: &CheckReport, limit: usize) -> Vec<PatchVariant> {
    let notes = parse_atomic_notes(xml);
    let mut variants = final_playability_variants(xml, report, limit / 2);

    let mut issues: Vec<&Issue> = report
        .issues
        .iter()
        .filter(|issue| issue.severity == Severity::Warning)
        .filter(|issue| matches!(issue.issue_type.as_str(), "PIANO_FAST_LARGE_JUMP" | "PIANO_HAND_SPAN_LARGE" | "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF"))
        .collect();
    issues.sort_by_key(|issue| {
        let critical_rank = if is_critical_piano_issue(issue) { 0 } else { 1 };
        let hardness = -issue.actual_divisions.unwrap_or(0);
        (critical_rank, hardness, issue.measure.clone(), issue.staff.clone().unwrap_or_default())
    });

    let mut choice_groups: Vec<Vec<RestifyChoice>> = Vec::new();
    for issue in issues.into_iter().take(6) {
        let choices = restify_choices_for_issue(&notes, issue);
        if !choices.is_empty() {
            choice_groups.push(choices);
        }
    }

    push_compound_restify_variants(xml, &mut variants, &choice_groups, limit);
    variants.truncate(limit);
    variants
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct MoveChoice {
    name: String,
    targets: Vec<MoveTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MoveTarget {
    note_start: usize,
    note_end: usize,
    staff_value_range: Option<(usize, usize)>,
    voice_value_range: Option<(usize, usize)>,
    target_staff: String,
    target_voice: String,
}

fn global_hand_trajectory_variants(xml: &str, report: &CheckReport, limit: usize) -> Vec<PatchVariant> {
    let notes = parse_atomic_notes(xml);
    let mut variants = final_playability_polish_variants(xml, report, limit / 4);

    let mut issues: Vec<&Issue> = report
        .issues
        .iter()
        .filter(|issue| issue.severity == Severity::Warning)
        .filter(|issue| matches!(issue.issue_type.as_str(), "PIANO_FAST_LARGE_JUMP" | "PIANO_HAND_SPAN_LARGE" | "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF"))
        .collect();
    issues.sort_by_key(|issue| {
        let critical_rank = if is_critical_piano_issue(issue) { 0 } else { 1 };
        let hardness = -issue.actual_divisions.unwrap_or(0);
        (critical_rank, hardness, issue.measure.clone(), issue.staff.clone().unwrap_or_default())
    });

    let mut move_groups: Vec<Vec<MoveChoice>> = Vec::new();
    let mut restify_groups: Vec<Vec<RestifyChoice>> = Vec::new();

    for issue in issues.into_iter().take(8) {
        let move_choices = move_choices_for_trajectory_issue(&notes, issue);
        if !move_choices.is_empty() {
            move_groups.push(move_choices);
        }
        let restify_choices = restify_choices_for_issue(&notes, issue);
        if !restify_choices.is_empty() {
            restify_groups.push(restify_choices);
        }
    }

    push_compound_move_variants(xml, &mut variants, &move_groups, limit);
    push_mixed_move_restify_variants(xml, &mut variants, &move_groups, &restify_groups, limit);
    variants.truncate(limit);
    variants
}

fn move_choices_for_trajectory_issue(notes: &[AtomicNote], issue: &Issue) -> Vec<MoveChoice> {
    match issue.issue_type.as_str() {
        "PIANO_FAST_LARGE_JUMP" => {
            let mut out = Vec::new();
            for (from_note, to_note) in find_jump_issue_note_pairs(notes, issue).into_iter().take(4) {
                out.extend(move_choices_for_jump_pair(notes, issue, from_note, to_note));
            }
            dedupe_move_choices(out, 12)
        }
        "PIANO_HAND_SPAN_LARGE" => {
            let mut out = Vec::new();
            for (low, high) in find_span_issue_note_pairs(notes, issue).into_iter().take(4) {
                out.extend(move_choices_for_span_pair(notes, issue, low, high));
            }
            dedupe_move_choices(out, 12)
        }
        "PIANO_TOO_MANY_SIMULTANEOUS_NOTES_ONE_STAFF" => {
            let Some(staff) = issue.staff.as_deref() else { return Vec::new(); };
            let mut out = Vec::new();
            let mut by_start: BTreeMap<i64, Vec<&AtomicNote>> = BTreeMap::new();
            for note in notes.iter().filter(|note| note.measure == issue.measure.as_str() && note.staff == staff && note.visible && note.pitch.is_some()) {
                by_start.entry(note.start).or_default().push(note);
            }
            for (_start, group) in by_start {
                if group.len() < 5 { continue; }
                out.extend(move_choice_for_pitch_band(&group, "overload_low_band", true));
                out.extend(move_choice_for_pitch_band(&group, "overload_high_band", false));
            }
            dedupe_move_choices(out, 8)
        }
        _ => Vec::new(),
    }
}

fn move_choices_for_jump_pair<'a>(notes: &'a [AtomicNote], issue: &Issue, from_note: &'a AtomicNote, to_note: &'a AtomicNote) -> Vec<MoveChoice> {
    let mut out = Vec::new();
    let start_min = from_note.start.min(to_note.start).saturating_sub(96);
    let start_max = from_note.start.max(to_note.start) + 96;
    let voice_scope = issue.voice.as_deref();
    let staff = issue.staff.as_deref().unwrap_or(from_note.staff.as_str());
    let mut window = notes
        .iter()
        .filter(|note| {
            note.measure == issue.measure.as_str()
                && note.staff == staff
                && note.visible
                && !note.is_chord
                && note.pitch.is_some()
                && note.start >= start_min
                && note.start <= start_max
                && voice_scope.is_none_or(|v| note.voice == v)
        })
        .collect::<Vec<_>>();
    window.sort_by_key(|note| (note.start, note.pitch.as_ref().map_or(0, |p| p.midi)));

    let Some(from_pitch) = from_note.pitch.as_ref() else { return out; };
    let Some(to_pitch) = to_note.pitch.as_ref() else { return out; };
    let low_midi = from_pitch.midi.min(to_pitch.midi);
    let high_midi = from_pitch.midi.max(to_pitch.midi);
    let mid_midi = (low_midi + high_midi) / 2;

    let low_cluster = window.iter().copied().filter(|note| note.pitch.as_ref().is_some_and(|p| p.midi <= mid_midi)).collect::<Vec<_>>();
    let high_cluster = window.iter().copied().filter(|note| note.pitch.as_ref().is_some_and(|p| p.midi > mid_midi)).collect::<Vec<_>>();
    out.extend(move_cluster_choice(&low_cluster, "trajectory_low_cluster", "2"));
    out.extend(move_cluster_choice(&high_cluster, "trajectory_high_cluster", "1"));

    let endpoint_group = vec![from_note, to_note];
    out.extend(move_cluster_choice(&endpoint_group, "trajectory_endpoints_to_lower", "2"));
    out.extend(move_cluster_choice(&endpoint_group, "trajectory_endpoints_to_upper", "1"));

    if from_pitch.midi <= 60 || to_pitch.midi <= 60 {
        let low_context = window.iter().copied().filter(|note| note.pitch.as_ref().is_some_and(|p| p.midi <= 66)).collect::<Vec<_>>();
        out.extend(move_cluster_choice(&low_context, "trajectory_low_context", "2"));
    }
    if from_pitch.midi >= 60 || to_pitch.midi >= 60 {
        let high_context = window.iter().copied().filter(|note| note.pitch.as_ref().is_some_and(|p| p.midi >= 60)).collect::<Vec<_>>();
        out.extend(move_cluster_choice(&high_context, "trajectory_high_context", "1"));
    }

    dedupe_move_choices(out, 12)
}

fn move_choices_for_span_pair<'a>(notes: &'a [AtomicNote], issue: &Issue, low_note: &'a AtomicNote, high_note: &'a AtomicNote) -> Vec<MoveChoice> {
    let mut out = Vec::new();
    let staff = issue.staff.as_deref().unwrap_or(low_note.staff.as_str());
    let start = low_note.start;
    let group = notes
        .iter()
        .filter(|note| note.measure == issue.measure.as_str() && note.staff == staff && note.visible && note.pitch.is_some() && note.start == start)
        .collect::<Vec<_>>();
    out.extend(move_cluster_choice(&[low_note], "trajectory_span_low_to_lower", "2"));
    out.extend(move_cluster_choice(&[high_note], "trajectory_span_high_to_upper", "1"));
    out.extend(move_choice_for_pitch_band(&group, "trajectory_span_low_band", true));
    out.extend(move_choice_for_pitch_band(&group, "trajectory_span_high_band", false));
    dedupe_move_choices(out, 10)
}

fn move_choice_for_pitch_band(notes: &[&AtomicNote], name: &str, low_band: bool) -> Vec<MoveChoice> {
    if notes.len() < 2 { return Vec::new(); }
    let mut sorted = notes.to_vec();
    sorted.sort_by_key(|note| note.pitch.as_ref().map_or(0, |p| p.midi));
    let split = sorted.len() / 2;
    let selected = if low_band { &sorted[..split.max(1)] } else { &sorted[split..] };
    let target_staff = if low_band { "2" } else { "1" };
    move_cluster_choice(selected, name, target_staff)
}
