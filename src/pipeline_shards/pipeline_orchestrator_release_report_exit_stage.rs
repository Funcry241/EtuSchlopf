include!("horizontal_layout_final_notation_snapshot.rs");
include!("horizontal_layout_plan_contract_projection.rs");

struct ReleaseReportAndExitStageInputs<'a> {
    cli: &'a Cli,
    source: &'a source::SourcePlan,
    best: SolverCandidate,
    initial_render: InitialRenderTransaction,
    render_state: structural_write_authority::RenderSovereignState,
    metric_beam_and_final_notation: MetricBeamAndFinalNotationStageOutcome,
    midi_projection: crate::source_event::CanonicalSourceEventProjection,
    source_event_causal_projection: crate::source_event::SourceEventCausalLedgerProjection,
}

fn run_release_report_and_exit_stage(
    inputs: ReleaseReportAndExitStageInputs<'_>,
) -> anyhow::Result<i32> {
    master_state_identity::record_render_stage_boundary(master_state_identity::RenderStageBoundary::Release);
    let ReleaseReportAndExitStageInputs {
        cli,
        source,
        mut best,
        initial_render,
        mut render_state,
        metric_beam_and_final_notation,
        midi_projection,
        source_event_causal_projection,
    } = inputs;
    // r1291: wall time per release sub-stage on stderr only (console log).
    let timing_label = cli.input.as_ref().map(|input| input.display().to_string()).unwrap_or_default();
    let mut stage_clock = Instant::now();
    let MetricBeamAndFinalNotationStageOutcome {
        release_evidence,
        committed_pdf_source_and_renderer_oracle,
    } = metric_beam_and_final_notation;
    let MetricFinalNotationReleaseEvidence {
        repair_loop_invocation_evidence,
        external_feedback_closure_render_write_audit,
        key_signature_enharmonic_spelling_render_write_audit,
        lower_clef_surface_materialization_audit,
        notation_voice_pre_serialization_constraint_audit,
        notation_voice_causal_lane_pressure_audit,
    } = release_evidence;
    let RenderPdfSourceSelectionStageOutcome {
        render_xml_selected_for_pdf,
        committed_voice_plan,
        final_serialized_musicxml_temporal_artifact_observation,
        pdf_actual_pages,
        silent_voice_coverage_projection_message,
        render_write_pdf_source_adoption_truth_gate_audit,
        render_write_pdf_source_adoption_truth_gate_blocks_release,
        authorized_writer_entry_boundary_audit,
    } = run_render_pdf_source_selection_stage(RenderPdfSourceSelectionStageInputs {
        render_state: &mut render_state,
        source,
        render_xml_trial_path: &initial_render.render_xml_trial_path,
        master_state: &mut best.master_state,
        committed_pdf_source_and_renderer_oracle,
    })?;
    stage_clock = pipeline_stage_timing(&timing_label, "ReleaseReportAndExit/RenderPdfSourceSelection", stage_clock);
    // The released Master state was adopted with its full transformation
    // lineage inside the release support; the Master file is only an artifact.
    let _ =
        validate_musicxml_export_semantics_from_xml(&source.musicxml_out, best.layout_xml())?;
    let InitialRenderTransaction {
        render_xml_trial_path,
        final_export_xml,
        behind_bars_rule_check_rest_and_tie_audit,
        cross_staff_hand_role_materialization_trial,
        lower_staff_visible_rhythm_rest_lane_trial,
        cursor_lane_end_invariant_repair_audit,
        p2_assertive_return_write_audit,
        strict_staff_local_release_completeness_trial,
        final_visible_rest_role_purge_and_vertical_placement_write_audit,
        strict_visible_rest_arithmetic_materializer_trial,
        external_feedback_safe_notation_write_audit,
        frozen_sounding_temporal_lane_plan,
        lower_staff_secondary_reanchored_lane_rest_visibility_suppression_trial,
        ..
    } = initial_render;
    let PostRenderAuditAssemblyStageOutcome {
        mut best,
        final_export_xml,
        render_musicxml_artifact_for_audit,
        final_render_xml_for_performance_audit,
        visible_lane_completeness_readback_gate_no_mutation,
        visible_rest_value_readability_audit_no_mutation,
        strict_visible_rest_arithmetic_materializer_blocks,
    } = run_post_render_audit_assembly_stage(PostRenderAuditAssemblyStageInputs {
        render_xml_trial_path: &render_xml_trial_path,
        render_state: &render_state,
        final_export_xml,
        render_xml_selected_for_pdf,
        pdf_actual_pages,
        best,
    })?;
    stage_clock = pipeline_stage_timing(&timing_label, "ReleaseReportAndExit/PostRenderAuditAssembly", stage_clock);
    master_state_identity::record_final_master_lineage(
        best.master_state.lineage(),
        best.layout_xml(),
    );
    let final_render_snapshot_xml = final_render_xml_for_performance_audit
        .as_deref()
        .unwrap_or(&final_export_xml);
    let raw_cursor_invariant_hard_gate = build_raw_cursor_invariant_hard_gate_audit(
        &final_export_xml,
        render_musicxml_artifact_for_audit.as_deref(),
    );
    let _ = apply_raw_cursor_invariant_hard_gate_to_report(
        &mut best.final_result,
        &raw_cursor_invariant_hard_gate,
    );
    if raw_cursor_invariant_hard_gate.hard_violations == 0 {
        let before_raw_cursor_issues = best.final_result.issues.len();
        best.final_result
            .issues
            .retain(|issue| issue.issue_type != "RAW_CURSOR_INVARIANT_HARD_GATE");
        let removed_raw_cursor_issues =
            before_raw_cursor_issues.saturating_sub(best.final_result.issues.len());
        if removed_raw_cursor_issues > 0 {
            refresh_report_summary(&mut best.final_result);
        }
    }
    let effective_retention_text = if p2_assertive_return_write_audit.actual_pitch_retention > 0 {
        format!(
            "{}%",
            p2_assertive_return_write_audit.actual_pitch_retention
        )
    } else {
        best.fidelity
            .retained_note_percent
            .map_or_else(|| "unbekannt".to_string(), |value| format!("{value:.0}%"))
    };
    let status_line_context = OrchestratorStatusLineContext {
        status: best.final_result.summary.status.to_string(),
        blockers: best.final_result.summary.blockers,
        warnings: best.final_result.summary.warnings,
        hints: best.final_result.summary.hints,
        retention: effective_retention_text.clone(),
    };
    let console_done_line = orchestrator_console_done_line(&status_line_context);
    let pdf_file_fingerprint = std::fs::read(&source.pdf_out)
        .ok()
        .map(|bytes| PdfFileFingerprintReport {
            bytes: bytes.len() as u64,
            sha256: sha256::hex(&bytes),
        });
    let render_closure = render_state.close_required_render_product(
        &best.master_state, source, render_xml_selected_for_pdf,
    )?;
    debug_assert!(render_closure.closed);
    let report_options = ReportOptions {
        top: cli.top,
        pdf_file_fingerprint,
        musicxml_artifact_locators: Some(crate::report::MusicXmlArtifactLocatorReport {
            master: source.musicxml_out.clone(),
            render: source.render_musicxml_out.clone(),
        }),
    };
    // The included final-report projection owner is compiled without a local lint suppression.
    stage_clock = pipeline_stage_timing(&timing_label, "ReleaseReportAndExit/ReleaseGatesAndOptions", stage_clock);
    let (finalized_report, console, final_release_blocked) =
        include!("final_report_projection_owner.rs");
    let _ = pipeline_stage_timing(&timing_label, "ReleaseReportAndExit/FinalReportProjection", stage_clock);
    console_artifact_line(
        ConsoleArtifactLine::Report,
        format!("schreibe Ergebnisbericht {}", source.report_out.display()),
    );
    let artifact_progress_started = Instant::now();
    console_artifact_progress(
        ConsoleArtifactLine::Report,
        "Report schreiben",
        1,
        1,
        artifact_progress_started,
    );
    crate::structural_write_authority::write_result_report_output_authorized(
        &source.report_out,
        &finalized_report,
    )
    .with_context(|| {
        format!(
            "Konnte Ergebnisdatei nicht schreiben: {}",
            source.report_out.display()
        )
    })?;
    // v0.10.1547: FinalReportContract already proves every required typed
    // section exactly once before the report authority writes the file.
    println!("{console}");
    Ok(if final_release_blocked {
        2
    } else {
        best.final_result.exit_code()
    })
}
