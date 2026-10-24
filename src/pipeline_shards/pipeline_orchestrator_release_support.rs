#[derive(Debug)]
struct RenderPdfSourceCandidate {
    active: bool,
    label: &'static str,
}


#[derive(Debug)]
struct RenderPdfSourceEvidenceLedger {
    candidates: Vec<RenderPdfSourceCandidate>,
}

impl RenderPdfSourceEvidenceLedger {
    fn new<const N: usize>(candidates: [(bool, &'static str); N]) -> Self {
        Self {
            candidates: candidates
                .into_iter()
                .map(|(active, label)| RenderPdfSourceCandidate { active, label })
                .collect(),
        }
    }

    fn render_requested(&self) -> bool {
        self.candidates.iter().any(|candidate| candidate.active)
    }

    fn selected_label(&self) -> &'static str {
        self.candidates
            .iter()
            .find(|candidate| candidate.active)
            .map_or("MusicXML", |candidate| candidate.label)
    }
}


#[cfg(test)]
mod render_pdf_source_evidence_ledger_tests {
    use super::RenderPdfSourceEvidenceLedger;

    #[test]
    fn first_active_candidate_owns_the_source_label() {
        let ledger = RenderPdfSourceEvidenceLedger::new([
            (false, "first"),
            (true, "second"),
            (true, "third"),
        ]);
        assert!(ledger.render_requested());
        assert_eq!(ledger.selected_label(), "second");
    }

    #[test]
    fn no_candidate_keeps_master_musicxml() {
        let ledger = RenderPdfSourceEvidenceLedger::new([(false, "render")]);
        assert!(!ledger.render_requested());
        assert_eq!(ledger.selected_label(), "MusicXML");
    }

    #[test]
    fn historical_unlabelled_writer_behavior_remains_explicit() {
        let ledger = RenderPdfSourceEvidenceLedger::new([(true, "MusicXML")]);
        assert!(ledger.render_requested());
        assert_eq!(ledger.selected_label(), "MusicXML");
    }
}

/// The Master export carried from the metric stage to the released-Master
/// adoption: the in-memory export realized from the owners' EXACT scripts
/// (#65 -> #59 -> #60) and their move-only receipts in application order.
#[derive(Debug)]
struct CommittedMasterExport {
    document: String,
    final_chord_projection_input_xml: String,
    final_chord_projection_output_xml: String,
    receipts: master_state_identity::OwnerReceipts,
}

impl CommittedMasterExport {
    /// Realizes the exported document from the #65 output by applying the #59
    /// and #60 EXACT scripts in order; any receipt that does not realize from
    /// its pre state is a fatal sovereignty violation.
    fn realize(
        final_chord_encoding_input: &str,
        final_chord_encoding_output: &str,
        final_chord_encoding_receipt: Option<ExactTransformationReceipt>,
        master_export_fifths_receipt: Option<ExactTransformationReceipt>,
        functional_respelling_receipt: Option<ExactTransformationReceipt>,
    ) -> Self {
        let mut document = final_chord_encoding_output.to_string();
        for receipt in [
            master_export_fifths_receipt.as_ref(),
            functional_respelling_receipt.as_ref(),
        ]
        .into_iter()
        .flatten()
        {
            document = receipt.realize_post_document(&document).unwrap_or_else(|| {
                master_state_identity::master_sovereignty_violation(
                    "MasterExport: an EXACT export receipt does not realize from its bound pre state",
                )
            });
        }
        let mut receipts = master_state_identity::OwnerReceipts::default();
        receipts.push_exact(final_chord_encoding_receipt);
        receipts.push_exact(master_export_fifths_receipt);
        receipts.push_exact(functional_respelling_receipt);
        Self {
            document,
            receipts,
            final_chord_projection_input_xml: final_chord_encoding_input.to_string(),
            final_chord_projection_output_xml: final_chord_encoding_output.to_string(),
        }
    }

    fn document(&self) -> &str {
        &self.document
    }
}

struct CommittedPdfSourceAndRendererOracleOutcome {
    committed_master_export: CommittedMasterExport,
    render_xml_selected_for_pdf: bool,
    pdf_source_label: &'static str,
    pdf_render_requested: bool,
    committed_source_xml: String,
    committed_initial_horizontal_layout_plan: HorizontalLayoutPlan,
    renderer_feasibility_basis: GlobalHorizontalPageSystemPlanAudit,
    final_pdf_source_commit: Option<render::final_pdf_source_commit::FinalPdfSourceCommit>,
    final_pdf_chord_structural_plan: structural_write_authority::CanonicalFinalChordStructuralPlan,
    final_pdf_pruned_omissions: std::collections::BTreeSet<structural_write_authority::RenderPrunedFinalChordOmissionKey>,
    renderer_feasibility_oracle_outcome: RendererFeasibilityOracleOutcome,
    render_write_pdf_source_adoption_truth_gate_audit:
        RenderWritePdfSourceAdoptionTruthGateAudit,
    render_write_pdf_source_adoption_truth_gate_blocks_release: bool,
    authorized_writer_entry_boundary_audit:
        structural_write_authority::AuthorizedWriterEntryBoundaryAudit,
}

struct RenderPdfSourceSelectionStageInputs<'a> {
    render_state: &'a mut structural_write_authority::RenderSovereignState,
    source: &'a source::SourcePlan,
    render_xml_trial_path: &'a Path,
    master_state: &'a mut structural_write_authority::MasterAdoptedState,
    committed_pdf_source_and_renderer_oracle:
        CommittedPdfSourceAndRendererOracleOutcome,
}

struct RenderPdfSourceSelectionStageOutcome {
    render_xml_selected_for_pdf: bool,
    committed_voice_plan: Option<structural_write_authority::FrozenSerializedVoicePlan>,
    final_serialized_musicxml_temporal_artifact_observation: RawCursorInvariantHardGateAudit,
    pdf_actual_pages: Option<usize>,
    silent_voice_coverage_projection_message: Option<String>,
    render_write_pdf_source_adoption_truth_gate_audit:
        RenderWritePdfSourceAdoptionTruthGateAudit,
    render_write_pdf_source_adoption_truth_gate_blocks_release: bool,
    authorized_writer_entry_boundary_audit:
        structural_write_authority::AuthorizedWriterEntryBoundaryAudit,
}

fn run_render_pdf_source_selection_stage(
    inputs: RenderPdfSourceSelectionStageInputs<'_>,
) -> anyhow::Result<RenderPdfSourceSelectionStageOutcome> {
    let RenderPdfSourceSelectionStageInputs {
        render_state,
        source,
        render_xml_trial_path,
        master_state,
        committed_pdf_source_and_renderer_oracle,
    } = inputs;
    let CommittedPdfSourceAndRendererOracleOutcome {
        committed_master_export,
        render_xml_selected_for_pdf,
        pdf_source_label,
        pdf_render_requested,
        committed_source_xml,
        committed_initial_horizontal_layout_plan,
        renderer_feasibility_basis,
        mut final_pdf_source_commit,
        final_pdf_chord_structural_plan,
        final_pdf_pruned_omissions,
        renderer_feasibility_oracle_outcome,
        render_write_pdf_source_adoption_truth_gate_audit,
        render_write_pdf_source_adoption_truth_gate_blocks_release,
        authorized_writer_entry_boundary_audit,
    } = committed_pdf_source_and_renderer_oracle;
    let pdf_source_path: &Path = if render_xml_selected_for_pdf {
        render_xml_trial_path
    } else {
        source.musicxml_out.as_path()
    };
    let committed_voice_plan = final_pdf_source_commit
        .as_ref()
        .and_then(|commit| commit.committed_voice_plan().cloned());

    // Final-notation horizontal-layout planning cutover. The committed-source
    // initial plan was built exactly once in the Metric/FinalNotation owner,
    // before the renderer-feasibility oracle. Reuse that typed plan token here
    // instead of recomputing the same Joint-Planner evidence in the release owner.
    let selected_final_notation_xml_before_layout = committed_source_xml;
    let initial_final_horizontal_layout_plan = committed_initial_horizontal_layout_plan;
    let final_horizontal_layout_plan = select_renderer_feasibility_productive_plan(
        &initial_final_horizontal_layout_plan,
        &renderer_feasibility_oracle_outcome,
        &selected_final_notation_xml_before_layout,
    )?;
    if !final_horizontal_layout_plan.provenance_complete() {
        anyhow::bail!(
            "FinalNotationHorizontalLayoutPlanningCutover: finaler Horizontal-Layout-Plan besitzt unvollstaendige Decision-Provenienz"
        );
    }
    console_progress(
        renderer_feasibility_basis
            .final_selected_plan_demand_binding_projection(&final_horizontal_layout_plan)?,
    );

    let final_horizontal_layout_materialization =
        final_horizontal_layout_plan.materialization_directives();
    // Released-Master adoption: the export events #65 -> #59 -> #60 after the read-only temporal check are
    // committed into the sovereign Master state together with the realized
    // export document. No file is read back as Master state.
    let CommittedMasterExport {
        document: master_export_document,
        receipts: master_export_receipts,
        final_chord_projection_input_xml,
        final_chord_projection_output_xml,
    } = committed_master_export;
    let mut released_master_export_receipts = master_state_identity::OwnerReceipts::default();
    released_master_export_receipts.append(master_export_receipts);
    master_state.adopt_materialized_document(
        master_export_document,
        released_master_export_receipts,
        "ReleasedMaster/FinalExport",
    );
    crate::audit_only::voice_run_collision_origin::observe_final_chord_adoption(
        "ReleasedMasterFinalExportReceiptChainCommitted",
        &final_chord_projection_input_xml,
        &final_chord_projection_output_xml,
        master_state.document(),
    );
    // E007 is the final Master owner (PRESERVING) with an empty phrase-intent
    // list; it calls the E011 sink internally and carries its own verified
    // post bytes in its receipt.
    let (final_master_layout, final_master_layout_receipt) =
        structural_write_authority::materialize_final_render_notation_with_phrase_projection_authorized(
            &source.musicxml_out,
            &final_horizontal_layout_materialization,
            &[],
        );
    let _ = final_master_layout?;
    master_state.adopt_path_bound_preserving_transition(
        final_master_layout_receipt,
        "ReleasedMaster/FinalLayoutPhrase",
    );
    if render_state.document().is_some() {
        let intents = crate::audit_only::phrase_boundary_evidence::pending_visual_notation_intents();
        if let Some(commit) = final_pdf_source_commit.as_ref() {
            commit.adopt_layout_and_phrase_course(render_state, &final_horizontal_layout_materialization, &intents)?;
        } else {
            structural_write_authority::adopt_final_layout_phrase_projection(
                render_state, &final_horizontal_layout_materialization, &intents,
            )?;
        }
    }

    // E007 may legitimately advance beyond the last #58. The committed PDF
    // proof is rebound to that witnessed sovereign post, never a later file read.
    if render_xml_selected_for_pdf {
        let prior_commit = final_pdf_source_commit.as_ref().with_context(||
            "Release E007 requires the previously committed final PDF source")?;
        let voice_plan = committed_voice_plan.as_ref().with_context(||
            "Release E007 requires the previously committed frozen voice plan")?;
        let renewed_commit = render::final_pdf_source_commit::
            preserve_pdf_source_commit_through_horizontal_layout_authorized(
                render_xml_trial_path,
                prior_commit,
                &selected_final_notation_xml_before_layout,
                render_state.require_document("Release E007 final layout post"),
                &final_pdf_chord_structural_plan,
                voice_plan,
                &final_pdf_pruned_omissions,
            ).with_context(|| "Release E007 PDF source proof continuation")?;
        final_pdf_source_commit = Some(renewed_commit);
    }

    // The Master file is an artifact and integrity witness only: its bytes
    // must equal the sovereign Master document byte for byte.
    let released_master_artifact =
        std::fs::read_to_string(&source.musicxml_out).with_context(|| {
            format!(
                "FinalNotationLayoutSnapshotClosure konnte kanonische Master-MusicXML nicht lesen: {}",
                source.musicxml_out.display()
            )
        })?;
    master_state.witness_released_artifact(&released_master_artifact);

    // After the single final materialization, freeze the sovereign Master state
    // and the persisted render notation before MuseScore.
    let final_notation_layout_snapshots = FinalNotationLayoutSnapshotSet::capture(
        master_state.clone(),
        render_state.document().map(str::to_owned),
    );
    let selected_final_notation_snapshot = final_notation_layout_snapshots
        .selected(render_xml_selected_for_pdf)
        .cloned()
        .with_context(|| {
            "FinalNotationLayoutSnapshotClosure: Render-Quelle wurde ausgewaehlt, aber kein gebundener finaler Render-Snapshot existiert"
        })?;
    let selected_final_notation_read_before_renderer =
        std::fs::read_to_string(pdf_source_path).with_context(|| {
            format!(
                "FinalNotationLayoutSnapshotClosure konnte ausgewaehlte finale Notationsquelle nicht lesen: {}",
                pdf_source_path.display()
            )
        });
    if render_xml_selected_for_pdf {
        master_state_identity::record_render_artifact_read(
            master_state_identity::RenderArtifactReadPurpose::WitnessAgainstSovereignState,
            selected_final_notation_read_before_renderer.as_ref().ok().map(String::as_str),
            render_state.require_document("Release renderer witness"),
            "Release renderer before_renderer",
        );
    }
    let selected_final_notation_xml_before_renderer = selected_final_notation_read_before_renderer?;
    if !selected_final_notation_snapshot.matches_exact(&selected_final_notation_xml_before_renderer) {
        anyhow::bail!(
            "FinalNotationLayoutSnapshotClosure: finale Notationsquelle stimmt unmittelbar nach SnapshotClose nicht ueberein; source={} fingerprint={:016x}",
            if render_xml_selected_for_pdf {
                "render"
            } else {
                "master"
            },
            selected_final_notation_snapshot.fingerprint(),
        );
    }

    let horizontal_layout_plan_contract = HorizontalLayoutPlanContractProjection::capture(
        &selected_final_notation_xml_before_renderer,
        selected_final_notation_snapshot.fingerprint(),
    )?;
    if !horizontal_layout_plan_contract.matches_snapshot_bound_source(
        &selected_final_notation_xml_before_renderer,
        selected_final_notation_snapshot.fingerprint(),
    )? {
        anyhow::bail!(
            "HorizontalLayoutPlanContractProjection: SnapshotClose konnte nicht exakt in den read-only Planvertrag projiziert werden"
        );
    }
    if !final_horizontal_layout_plan.matches_materialized(
        &selected_final_notation_xml_before_renderer,
    )? {
        anyhow::bail!(
            "FinalNotationHorizontalLayoutPlanningCutover: finaler Decision-Plan weicht von der einmalig materialisierten snapshot-gebundenen Notationsquelle ab"
        );
    }
    if !horizontal_layout_plan_contract.matches_decision_plan(&final_horizontal_layout_plan) {
        anyhow::bail!(
            "FinalNotationHorizontalLayoutPlanningCutover: finaler Decision-Plan stimmt nicht mit dem snapshot-gebundenen Planvertrag ueberein"
        );
    }

    let mut pdf_actual_pages: Option<usize> = None;
    let silent_voice_coverage_projection_message = if pdf_render_requested {
        if render_xml_selected_for_pdf {
            console_artifact_line(
                ConsoleArtifactLine::PostRenderXml,
                format!("PDF-Quelle ist Post-Render-XML: {pdf_source_label}"),
            );
            let artifact_progress_started = Instant::now();
            console_artifact_progress(
                ConsoleArtifactLine::PostRenderXml,
                "Post-Render-XML PDF-Quelle",
                1,
                1,
                artifact_progress_started,
            );
        } else {
            console_artifact_line(
                ConsoleArtifactLine::MasterMusicXml,
                format!("PDF-Quelle ist Master-MusicXML: {pdf_source_label}"),
            );
            let artifact_progress_started = Instant::now();
            console_artifact_progress(
                ConsoleArtifactLine::MasterMusicXml,
                "Master-PDF-Quelle",
                1,
                1,
                artifact_progress_started,
            );
        }
        console_artifact_line(
            ConsoleArtifactLine::Pdf,
            format!(
                "PDF-Renderer schreibt {} aus {}",
                source.pdf_out.display(),
                pdf_source_label
            ),
        );
        let artifact_progress_started = Instant::now();
        console_artifact_progress(
            ConsoleArtifactLine::Pdf,
            "PDF schreiben",
            1,
            1,
            artifact_progress_started,
        );
        let final_pdf_source_commit = final_pdf_source_commit.as_ref().with_context(|| {
            "FinalNotationLayoutSnapshotClosure requires the existing final PDF-source commit before rendering"
        })?;
        let render = render_pdf_if_possible(
            pdf_source_path,
            &source.pdf_out,
            final_pdf_source_commit,
        );
        match render.status {
            RenderStatus::Written | RenderStatus::WrittenButNonZeroExit => {
                pdf_actual_pages = count_pdf_pages(&source.pdf_out);
            }
            RenderStatus::NoRenderer | RenderStatus::RendererFailed => {}
        };
        let projection_message = render.message.clone();
        Some(projection_message)
    } else {
        None
    };

    let selected_final_notation_read_after_renderer =
        std::fs::read_to_string(pdf_source_path).with_context(|| {
            format!(
                "FinalNotationLayoutSnapshotClosure konnte ausgewaehlte finale Notationsquelle nach Renderer nicht lesen: {}",
                pdf_source_path.display()
            )
        });
    if render_xml_selected_for_pdf {
        master_state_identity::record_render_artifact_read(
            master_state_identity::RenderArtifactReadPurpose::WitnessAgainstSovereignState,
            selected_final_notation_read_after_renderer.as_ref().ok().map(String::as_str),
            render_state.require_document("Release renderer witness"),
            "Release renderer after_renderer",
        );
    }
    let selected_final_notation_xml_after_renderer = selected_final_notation_read_after_renderer?;
    if !selected_final_notation_snapshot.matches_exact(&selected_final_notation_xml_after_renderer) {
        anyhow::bail!(
            "FinalNotationLayoutSnapshotClosure: finale Notationsquelle wurde nach SnapshotClose mutiert; source={} fingerprint={:016x}",
            if render_xml_selected_for_pdf {
                "render"
            } else {
                "master"
            },
            selected_final_notation_snapshot.fingerprint(),
        );
    }

    if !horizontal_layout_plan_contract.matches_snapshot_bound_source(
        &selected_final_notation_xml_after_renderer,
        selected_final_notation_snapshot.fingerprint(),
    )? {
        anyhow::bail!(
            "HorizontalLayoutPlanContractProjection: horizontale Direktiven weichen nach PlanContractClose vom snapshot-gebundenen Vertrag ab"
        );
    }

    // This observation has no release authority. It reads the exact persisted
    // master/render bytes frozen by FinalNotationLayoutSnapshotSet, while the
    // historical raw-cursor release gate remains bound to its existing inputs.
    let final_serialized_musicxml_temporal_artifact_observation =
        build_raw_cursor_invariant_hard_gate_audit(
            final_notation_layout_snapshots
                .released_master_state()
                .document(),
            final_notation_layout_snapshots
                .render
                .as_ref()
                .map(|snapshot| snapshot.xml.as_str()),
        );

    Ok(RenderPdfSourceSelectionStageOutcome {
        render_xml_selected_for_pdf,
        committed_voice_plan,
        final_serialized_musicxml_temporal_artifact_observation,
        pdf_actual_pages,
        silent_voice_coverage_projection_message,
        render_write_pdf_source_adoption_truth_gate_audit,
        render_write_pdf_source_adoption_truth_gate_blocks_release,
        authorized_writer_entry_boundary_audit,
    })
}

#[derive(Debug)]
struct PostRenderAuditAssemblyStageOutcome {
    best: SolverCandidate,
    final_export_xml: String,
    render_musicxml_artifact_for_audit: Option<String>,
    final_render_xml_for_performance_audit: Option<String>,
    visible_lane_completeness_readback_gate_no_mutation:
        VisibleLaneCompletenessClassifierPurgeGateAudit,
    visible_rest_value_readability_audit_no_mutation: RenderXmlVisibleRestReadabilityAudit,
    strict_visible_rest_arithmetic_materializer_blocks: bool,
}

struct PostRenderAuditAssemblyStageInputs<'a> {
    render_xml_trial_path: &'a Path,
    render_state: &'a structural_write_authority::RenderSovereignState,
    final_export_xml: String,
    render_xml_selected_for_pdf: bool,
    pdf_actual_pages: Option<usize>,
    best: SolverCandidate,
}

fn run_post_render_audit_assembly_stage(
    inputs: PostRenderAuditAssemblyStageInputs<'_>,
) -> anyhow::Result<PostRenderAuditAssemblyStageOutcome> {
    let PostRenderAuditAssemblyStageInputs {
        render_xml_trial_path,
        render_state,
        final_export_xml,
        render_xml_selected_for_pdf,
        pdf_actual_pages,
        mut best,
    } = inputs;
    let render_musicxml_artifact_for_audit = render_state.document().map(str::to_owned);
    let mut final_render_xml_for_performance_audit: Option<String> = None;
    if render_xml_selected_for_pdf {
        let render_guard_source = render_musicxml_artifact_for_audit
            .as_ref()
            .cloned()
            .with_context(|| {
                format!(
                    "Konnte Render-MusicXML für ExportGuard nicht lesen: {}",
                    render_xml_trial_path.display()
                )
            })?;
        let _ = validate_musicxml_export_semantics_from_xml(
            render_xml_trial_path,
            &render_guard_source,
        )?;
        final_render_xml_for_performance_audit = Some(render_guard_source);
    }
    let final_render_snapshot_xml = final_render_xml_for_performance_audit
        .as_deref()
        .unwrap_or(&final_export_xml);
    let final_render_snapshot_source_label = if final_render_xml_for_performance_audit.is_some() {
        "finale Render-MusicXML nach allen Writes"
    } else {
        "finale Master-MusicXML ohne Render-XML"
    };
    let visible_lane_completeness_readback_gate_no_mutation =
        build_visible_lane_completeness_readback_gate_no_mutation(
            final_render_snapshot_xml,
            final_render_snapshot_source_label,
        );
    let mut visible_rest_value_readability_audit_no_mutation =
        build_render_xml_visible_rest_readability_audit(
            final_render_snapshot_xml,
            final_render_snapshot_source_label,
        );
    visible_rest_value_readability_audit_no_mutation.done_fragment = format!(
        "VisibleRestValueReadabilityAuditNoMutation: sichtbare Pausenwerte {}; Voice-Scopes {}; fehlende-Pausen-Kandidaten {}; fragmentierte Ketten {}; ganztaktige/Rest-only-Pruefung integriert; WriteAllowed nein; MusicMutation nein",
        visible_rest_value_readability_audit_no_mutation.visible_rests,
        visible_rest_value_readability_audit_no_mutation.checked_voice_scopes,
        visible_rest_value_readability_audit_no_mutation.missing_visible_rest_candidates,
        visible_rest_value_readability_audit_no_mutation.fragmented_chain_candidates
    );
    visible_rest_value_readability_audit_no_mutation.pipeline_line = format!(
        "VisibleRestValueReadabilityAuditNoMutation v{}: finaler audit-only Readback aus {}; sichtbare Pausenwerte {}, sichtbare klingende Noten {}, Voice-Scopes {}, fehlende-Pausen-Kandidaten {} ({} divisions), Restketten {}, fragmentierte Ketten {}, hilfreich zusammenfassbar {}, Rewrite-Kandidaten {}, Klassen [{}], Aktionen [{}]; parse/back-forward {}/{}; Signatur {}; WriteAllowed nein; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        visible_rest_value_readability_audit_no_mutation.source_label,
        visible_rest_value_readability_audit_no_mutation.visible_rests,
        visible_rest_value_readability_audit_no_mutation.visible_pitched,
        visible_rest_value_readability_audit_no_mutation.checked_voice_scopes,
        visible_rest_value_readability_audit_no_mutation.missing_visible_rest_candidates,
        visible_rest_value_readability_audit_no_mutation.missing_rest_duration_total,
        visible_rest_value_readability_audit_no_mutation.total_rest_chains,
        visible_rest_value_readability_audit_no_mutation.fragmented_chain_candidates,
        visible_rest_value_readability_audit_no_mutation.helpful_consolidation_candidates,
        visible_rest_value_readability_audit_no_mutation.rewrite_candidates,
        count_summary(&visible_rest_value_readability_audit_no_mutation.class_counts, 8),
        count_summary(&visible_rest_value_readability_audit_no_mutation.action_counts, 8),
        if visible_rest_value_readability_audit_no_mutation.parse_ok { "ok" } else { "fail" },
        if visible_rest_value_readability_audit_no_mutation.backup_forward_ok { "ok" } else { "fail" },
        visible_rest_value_readability_audit_no_mutation.visible_signature,
        visible_rest_value_readability_audit_no_mutation.verdict
    );
    let strict_visible_rest_arithmetic_materializer_candidate_blocks =
        strict_visible_rest_arithmetic_materializer_blocks_release(
            &visible_rest_value_readability_audit_no_mutation,
        );
    let strict_visible_rest_visible_lane_candidate_blocks = strict_staff_incomplete_blocks_release(
        &visible_lane_completeness_readback_gate_no_mutation,
    );
    let strict_visible_rest_arithmetic_materializer_blocks =
        strict_visible_rest_arithmetic_materializer_candidate_blocks
            && strict_visible_rest_visible_lane_candidate_blocks
            && best.final_result.summary.blockers > 0;
    let voice_fiber_behind_bars_decision_ledger_no_music_mutation =
        build_voice_fiber_behind_bars_decision_ledger_no_music_mutation(
            &best.final_result,
            best.layout_xml(),
            pdf_actual_pages,
        );
    let _ =
        apply_voice_fiber_behind_bars_decision_ledger_to_report(
            &mut best.final_result,
            &voice_fiber_behind_bars_decision_ledger_no_music_mutation,
        );
    Ok(PostRenderAuditAssemblyStageOutcome {
        best,
        final_export_xml,
        render_musicxml_artifact_for_audit,
        final_render_xml_for_performance_audit,
        visible_lane_completeness_readback_gate_no_mutation,
        visible_rest_value_readability_audit_no_mutation,
        strict_visible_rest_arithmetic_materializer_blocks,
    })
}

