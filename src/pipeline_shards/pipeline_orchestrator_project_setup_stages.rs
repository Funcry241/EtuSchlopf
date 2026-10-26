struct ProjectRunPreparationStageInputs<'a> {
    cli: &'a Cli,
}

struct ProjectRunPreparationStageOutcome {
    source: source::SourcePlan,
    writer_trace_session: crate::audit_only::writer_trace::WriterTraceSession,
    reference: ReferenceReport,
    options: CheckOptions,
    parallel: ParallelPlan,
    midi_path: PathBuf,
    midi_projection: crate::source_event::CanonicalSourceEventProjection,
    source_event_causal_projection: crate::source_event::SourceEventCausalLedgerProjection,
}

fn run_project_run_preparation_stage(
    inputs: ProjectRunPreparationStageInputs<'_>,
) -> anyhow::Result<ProjectRunPreparationStageOutcome> {
    let ProjectRunPreparationStageInputs { cli } = inputs;
    let source = resolve_source_plan(
        cli.input.as_deref(),
        cli.out.as_deref(),
        cli.musicxml_out.as_deref(),
        cli.pdf_out.as_deref(),
    )?;
    let parallel = resolve_parallel_plan(&cli.jobs, cli.candidate_budget_jobs)?;
    let decoded_midi = validate_project_input_files(&source, cli.reference_wav.as_deref())?;
    let reference = inspect_reference_files(
        &source.expected_midi,
        cli.reference_wav.as_deref().or(Some(source.expected_wav.as_path())),
    );
    let midi_projection = crate::source_event::project_source_events(&decoded_midi.notes);
    let source_event_causal_projection = crate::source_event::project_source_event_causality(
        &decoded_midi.source_event_causal_ledger,
        &decoded_midi.notes,
        decoded_midi.source_ppq,
    )?;
    console_progress(format!(
        "MIDI produktiv; Format {}; Tracks {}; PPQ {}; Tempo-Metadaten {}; Taktarten {}; Events {}; Endposition {}; Fingerprint {}; RawNoteEvents {}; CausalPairs {}; CrossTrackPairs {}; RawPairFingerprint {}; CausalFingerprint {}; PairingReplay ja",
        decoded_midi.format,
        decoded_midi.tracks,
        decoded_midi.source_ppq,
        decoded_midi.tempo_microseconds_per_quarter.len(),
        decoded_midi.time_signatures.len(),
        midi_projection.event_count,
        midi_projection.terminal_extent_ticks,
        midi_projection.event_fingerprint_sha256,
        source_event_causal_projection.raw_note_event_count,
        source_event_causal_projection.entry_count,
        source_event_causal_projection.cross_track_pair_count,
        source_event_causal_projection.raw_pair_fingerprint_sha256,
        source_event_causal_projection.causal_fingerprint_sha256,
    ));
    let options = CheckOptions {
        strict: !cli.lenient_partial_voices,
    };
    let _writer_trace_session = crate::audit_only::writer_trace::start(
        cli.writer_trace.as_deref(),
        &source.input_label,
        Some(&source.render_musicxml_out),
    )
    .map_err(|reason| anyhow!("Writer-Trace konnte nicht initialisiert werden: {reason}"))?;
    purge_previous_report_outputs(&source)?;
    let midi_path = source.expected_midi.clone();
    console_progress(format!(
        "ARBEITET - Projekt '{}' wird aus MIDI/WAV analysiert",
        source.input_label
    ));
    console_artifact_legend();
    console_artifact_line(
        ConsoleArtifactLine::MasterMusicXml,
        format!(
            "Standardausgabe Master-MusicXML: {}",
            source.musicxml_out.display()
        ),
    );
    console_artifact_line(
        ConsoleArtifactLine::PreRenderXml,
        format!(
            "Pre-Render-XML-Zwischenstand: {}",
            source.render_musicxml_out.display()
        ),
    );
    console_artifact_line(
        ConsoleArtifactLine::PostRenderXml,
        "Post-Render-XML ist die finale PDF-Quelle nach renderseitigen Writes/Readbacks",
    );
    console_artifact_line(
        ConsoleArtifactLine::Pdf,
        format!("PDF-Ausgabe: {}", source.pdf_out.display()),
    );
    console_artifact_line(
        ConsoleArtifactLine::Report,
        format!("Ergebnisbericht: {}", source.report_out.display()),
    );
    console_artifact_line(ConsoleArtifactLine::BehindBarsRuleCheck, "Bienenkoenigin-/Behind-Bars-Regelmatrix bleibt eigenes Notationsgate und wird nicht durch Maschinen-OK ersetzt");
    console_progress(format!(
        "Parallelität: --jobs {} => {} Worker (System meldet {} logische Threads); CandidateBudgetBasis {} Worker; HumanPotential-Budget {} / max {} Stützen",
        parallel.requested,
        parallel.workers,
        parallel.logical_threads,
        parallel.candidate_budget_workers,
        parallel.human_potential_budget,
        parallel.human_potential_max_moves
    ));

    Ok(ProjectRunPreparationStageOutcome {
        source,
        writer_trace_session: _writer_trace_session,
        reference,
        options,
        parallel,
        midi_path,
        midi_projection,
        source_event_causal_projection,
    })
}

struct MasterNotationAndSalienceStageInputs<'a> {
    best: SolverCandidate,
    next_iteration: usize,
    options: &'a CheckOptions,
}

struct MasterNotationAndSalienceStageOutcome {
    best: SolverCandidate,
}

fn run_master_notation_and_salience_stage(
    inputs: MasterNotationAndSalienceStageInputs<'_>,
) -> anyhow::Result<MasterNotationAndSalienceStageOutcome> {
    let MasterNotationAndSalienceStageInputs {
        mut best,
        mut next_iteration,
        options,
    } = inputs;
    console_progress(
        "NotationPolish: Schlussstrich/Spielanweisungen/Oktavierung/Layout werden geschrieben",
    );
    let receipt_before_master_notation_prelude = best.layout_xml().to_string();
    let estimated_bpm = best.reference_signature.estimated_bpm;
    // ADDITIVE E004 (final barline) followed by ADDITIVE E036 directions.
    best.master_state.adopt_in_place_transformation(
        "MasterNotationPrelude/FinalBarlineAndExpression",
        |xml| {
            let mut receipts = master_state_identity::OwnerReceipts::default();
            receipts.push_additive(ensure_final_barline(xml));
            let (_expression_summary, expression_receipts) =
                apply_expression_polish(xml, estimated_bpm);
            receipts.append(expression_receipts);
            ((), receipts)
        },
    );
    crate::audit_only::destructive_edit_attribution::observe_selected_state_successor(
        &receipt_before_master_notation_prelude,
        best.layout_xml(),
    );
    crate::registernotation::loesche();
    crate::registernotation::beobachte_vor_k00(best.layout_xml());
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterBeforeOctaveNotationPolicy",
        best.layout_xml(),
    );
    // BLOCK #50 per bound octave measure replacement.
    best.master_state.adopt_in_place_transformation("MasterNotationPrelude/OctaveNotationPolicy", |xml| {
        ((), apply_octave_notation_policy(xml).block_receipts)
    });
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterOctaveNotationPolicy",
        best.layout_xml(),
    );
    best.master_state.adopt_in_place_transformation("MasterNotationPrelude/LowerStaffClefReadability", |xml| {
        let lower_clef_policy = apply_lower_staff_clef_readability_policy(xml);
        let mut lower_clef_policy_receipts = master_state_identity::OwnerReceipts::default();
        lower_clef_policy_receipts.push_blocks(lower_clef_policy.block_receipts);
        ((), lower_clef_policy_receipts)
    });
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterLowerStaffClefReadability",
        best.layout_xml(),
    );
    let _ = suppress_lower_staff_treble_clef_high_register_hints(
        &mut best.final_result,
        best.master_state.document(),
    );
    let mut horizontal_layout_plan_builder =
        HorizontalLayoutPlanBuilder::from_seed(
            &best.horizontal_layout_seed,
            best.layout_xml(),
        )?;
    let receipt_before_final_notation_polish = best.layout_xml().to_string();
    // EXACT #5 (note visibility) and EXACT #6 (allowed range removal).
    best.master_state.adopt_in_place_transformation("MasterNotationPrelude/FinalNotationPolishGuard", |xml| {
        ((), apply_final_notation_polish_guard(xml))
    });
    crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
        &receipt_before_final_notation_polish,
        best.layout_xml(),
    );
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterFinalNotationPolishGuard",
        best.layout_xml(),
    );
    if !horizontal_layout_plan_builder.working_xml_is_compatible(best.layout_xml())? {
        anyhow::bail!(
            "HorizontalLayoutPlanMaterializationClosure: FinalNotationPolishGuard erzeugte unautorisierten horizontalen XML-Zwischenzustand"
        );
    }
    let receipt_before_final_export_normalization = best.layout_xml().to_string();
    // PRESERVING E055 (generated octave word labels).
    let _ = best
        .master_state
        .adopt_in_place_transformation("MasterNotationPrelude/FinalExportNormalization", |xml| {
            let (normalized, receipt) = normalize_final_export_xml(xml);
            let mut receipts = master_state_identity::OwnerReceipts::default();
            receipts.push_preserving(receipt);
            (normalized, receipts)
        })?;
    crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
        &receipt_before_final_export_normalization,
        best.layout_xml(),
    );
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterFinalExportNormalization",
        best.layout_xml(),
    );
    if !horizontal_layout_plan_builder.working_xml_is_compatible(best.layout_xml())? {
        anyhow::bail!(
            "HorizontalLayoutPlanMaterializationClosure: Normalisierung veraenderte horizontalen XML-Zwischenzustand ausserhalb des Plan-Owners"
        );
    }
    let receipt_before_visual_layout_pressure = best.layout_xml().to_string();
    // PRESERVING E044 (visual layout defaults).
    best.master_state
        .adopt_in_place_transformation("MasterNotationPrelude/VisualLayoutPressure", |xml| {
            apply_visual_layout_pressure_pass(xml, &mut horizontal_layout_plan_builder)
        })?;
    crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
        &receipt_before_visual_layout_pressure,
        best.layout_xml(),
    );
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterVisualLayoutPressure",
        best.layout_xml(),
    );
    if !horizontal_layout_plan_builder.working_xml_is_compatible(best.layout_xml())? {
        anyhow::bail!(
            "HorizontalLayoutPlanMaterializationClosure: VisualLayoutPressure schrieb horizontale Geometrie vor PlanClose"
        );
    }
    let receipt_before_pianistic_readability = best.layout_xml().to_string();
    // PRESERVING E013 (local staff distance) per readability target.
    best.master_state.adopt_in_place_transformation("MasterNotationPrelude/PianisticReadability", |xml| {
        ((), apply_pianistic_readability_polish(xml, &mut horizontal_layout_plan_builder))
    });
    crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
        &receipt_before_pianistic_readability,
        best.layout_xml(),
    );
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterPianisticReadability",
        best.layout_xml(),
    );
    if !horizontal_layout_plan_builder.working_xml_is_compatible(best.layout_xml())? {
        anyhow::bail!(
            "HorizontalLayoutPlanMaterializationClosure: PianisticReadability schrieb horizontale Geometrie vor PlanClose"
        );
    }
    // The early builder remains only as a live XML-polish guard and as the
    // planned-system-state input for PianisticReadability. It is deliberately
    // not closed into a carried comparison plan; authoritative horizontal-layout
    // planning happens once on the committed final-notation source.
    reconcile_candidate_material_lineage(
        &mut best,
        "MasterNotationRepresentationPrelude",
        CandidateMaterialDeclaredEffects::representation_only(),
    )?;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterBeforeEditorialVoiceDecision",
        best.layout_xml(),
    );
    let _ = run_editorial_voice_decision_pass(&mut best, options)?;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterEditorialVoiceDecisionBeforeReconcile",
        best.layout_xml(),
    );
    reconcile_candidate_material_lineage(
        &mut best,
        "EditorialVoiceDecision",
        CandidateMaterialDeclaredEffects::candidate_material(),
    )?;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterEditorialVoiceDecision",
        best.layout_xml(),
    );
    console_progress("NotationPolish: portable MusicXML-Anreicherungen validieren");
    run_certified_materializing_local_rewrite_portfolio(
        &mut best,
        options,
        &mut next_iteration,
    )?;
    reconcile_candidate_material_lineage(
        &mut best,
        "CertifiedMaterializingLocalRewritePortfolio",
        CandidateMaterialDeclaredEffects::pitch_staff_voice(),
    )?;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterAfterCertifiedMaterializingLocalRewritePortfolio",
        best.layout_xml(),
    );
    mark_candidate_material_freeze_boundary_candidate(&mut best)?;
    // r667 diagnostic only: retain the exact XML bytes at the already-existing r666
    // freeze-boundary candidate so the later report cannot confuse them with persisted master.
    best.consensus_relocation_freeze_boundary_xml = Some(best.layout_xml().to_string());
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MasterFreezeBoundaryCandidate",
        best.layout_xml(),
    );

    Ok(MasterNotationAndSalienceStageOutcome { best })
}

fn validate_project_input_files(source: &source::SourcePlan, explicit_wav: Option<&Path>) -> anyhow::Result<crate::midi::DecodedMidiSource> {
    let wav = explicit_wav.unwrap_or(&source.expected_wav);
    crate::source::validate_file_targets(
        &[source.expected_midi.clone(), wav.to_path_buf()],
        &source.output_paths(),
    )?;
    let decoded_midi = crate::midi::read_standard_midi(&source.expected_midi)
        .with_context(|| format!("MIDI ungueltig: {}. Bitte eine gueltige Standard-MIDI-Datei verwenden.", source.expected_midi.display()))?;
    if decode_wav_mono_samples(wav).is_none() {
        return Err(anyhow!("WAV kann nicht als Audiodatei gelesen werden: {}. Bitte eine unterstuetzte, nicht leere WAV-Datei bereitstellen.", wav.display()));
    }
    Ok(decoded_midi)
}
