struct MetricBeamAndFinalNotationStageInputs<'a> {
    initial_render: &'a InitialRenderTransaction,
    render_state: &'a mut structural_write_authority::RenderSovereignState,
    source: &'a source::SourcePlan,
    reference: ReferenceReport,
    no_pdf: bool,
    final_blockers: usize,
    horizontal_layout_seed: &'a HorizontalLayoutSeedPlan,
}

struct MetricBeamAndFinalNotationStageOutcome {
    release_evidence: MetricFinalNotationReleaseEvidence,
    committed_pdf_source_and_renderer_oracle:
        CommittedPdfSourceAndRendererOracleOutcome,
}

// Cross-phase ownership contract: Release/Report receives one explicit evidence
// projection, never the internal Metric/Beam/Register/Notation stage outcomes.
// The productive stage-local values die before this boundary.

struct MetricFinalNotationReleaseEvidence {
    repair_loop_invocation_evidence: RestArticulationRepairLoopInvocationEvidence,
    external_feedback_closure_render_write_audit: ExternalFeedbackClosureRenderWriteAudit,
    key_signature_enharmonic_spelling_render_write_audit:
        KeySignatureEnharmonicSpellingRenderWriteAudit,
    lower_clef_surface_materialization_audit: LowerClefSurfaceMaterializationAudit,
    notation_voice_pre_serialization_constraint_audit:
        crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::
            NotationVoicePreSerializationConstraintClosureAuditProjection,
    notation_voice_causal_lane_pressure_audit:
        crate::audit_only::notation_voice_causal_lane_pressure_audit::
            NotationVoiceCausalLanePressureAuditProjection,
}

fn run_metric_beam_and_final_notation_stage(
    inputs: MetricBeamAndFinalNotationStageInputs<'_>,
) -> anyhow::Result<MetricBeamAndFinalNotationStageOutcome> {
    let MetricBeamAndFinalNotationStageInputs {
        initial_render,
        render_state,
        source,
        reference,
        no_pdf,
        final_blockers,
        horizontal_layout_seed,
    } = inputs;
    let receipt_checkpoint_metric_entry = render_state.document()
        .unwrap_or(&initial_render.pre_preview_render_xml_for_gap).to_owned();
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MetricFinalNotationEntryRender",
        &receipt_checkpoint_metric_entry,
    );
    let render_pruned_final_chord_omissions =
        initial_render.render_pruned_final_chord_omissions.clone();
    let mut render_final_chord_structural_plan = initial_render
        .frozen_sounding_temporal_lane_plan
        .final_chord_structural_plan
        .clone();
    // Cross-file interface closure: RestAndArticulationRenderStageInputs is physically owned by the dedicated private include.
    let rest_and_articulation =
        run_rest_and_articulation_render_stage(RestAndArticulationRenderStageInputs {
            render_state: &mut *render_state,
            final_render_source_available: initial_render.final_render_source_available,
            reference: &reference,
            render_xml_trial_path: &initial_render.render_xml_trial_path,
            pre_preview_render_xml_for_gap: &initial_render.pre_preview_render_xml_for_gap,
            render_final_chord_structural_plan: &mut render_final_chord_structural_plan,
            frozen_serialized_voice_plan: &initial_render
                .frozen_sounding_temporal_lane_plan
                .frozen_serialized_voice_plan,
            render_pruned_final_chord_omissions: &render_pruned_final_chord_omissions,
        })?;
    crate::master_state_identity::record_render_stage_boundary(
        crate::master_state_identity::RenderStageBoundary::MetricBeam,
    );
    let receipt_checkpoint_after_rest = render_state.document()
        .unwrap_or(&initial_render.pre_preview_render_xml_for_gap).to_owned();
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MetricAfterRestArticulationRenderStage",
        &receipt_checkpoint_after_rest,
    );
    if initial_render.final_render_source_available {
        let whole_rest_truth_source_xml = render_state.require_state_root("Metric WholeRest truth").document().to_owned();
        let whole_rest_truth =
            verify_whole_rest_display_coordinate_readback_truth(&whole_rest_truth_source_xml);
        let readback_green = whole_rest_truth.residual_candidates == 0;
        if !readback_green {
            anyhow::bail!(
                "WholeRestDisplayCoordinateReadbackVerifier: {} residuale Ganztaktpausen-Kandidaten (measure=yes {}, fehlende Koordinaten {}); der typisierte produktive Writer ist nicht vollstaendig und der Lauf bleibt fail-closed",
                whole_rest_truth.residual_candidates,
                whole_rest_truth.measure_yes_rows,
                whole_rest_truth.missing_coordinate_rows
            );
        }
    }
    let mut bridge_transition_pre = render_state.document().map(str::to_owned);
    let mut external_feedback_closure_render_write_audit = ExternalFeedbackClosureRenderWriteAudit::default();
    let key_signature_enharmonic_spelling_render_write_audit = if initial_render.final_render_source_available {
        let key_signature_root = render_state.require_state_root("M4 KeySignatureSpelling");
        let key_signature_source_xml = key_signature_root.document().to_owned();
        let ((key_signature_after_xml, audit), key_signature_receipt) = apply_key_signature_enharmonic_spelling_render_write(
            &key_signature_source_xml,
            "finale Render-MusicXML vor KeySignatureEnharmonicSpelling",
            &initial_render.render_xml_trial_path,
        );
        let mut key_signature_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        key_signature_receipts.push_block(key_signature_receipt, &key_signature_after_xml);
        if audit.written {
            render_state.adopt_transition(
                key_signature_root, &key_signature_after_xml, key_signature_receipts,
                structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::KeySignatureSpelling),
                |path, _pre, post| structural_write_authority::write_key_signature_spelling_checkpoint_authorized(path, post),
            ).with_context(|| "M4 KeySignatureSpelling sovereign adoption")?;
        } else {
            key_signature_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        }
        audit
    } else {
        KeySignatureEnharmonicSpellingRenderWriteAudit {
            source_label: "keine finale Render-XML-Quelle".to_string(),
            output_path: initial_render.render_xml_trial_path.display().to_string(),
            evidence_intake_open: false,
            verdict: "key_signature_enharmonic_spelling_skipped_no_render_source".to_string(),
            ..KeySignatureEnharmonicSpellingRenderWriteAudit::default()
        }
    };
    if let Some(pre) = bridge_transition_pre.take() {
        let post = render_state.require_document("Metric M4 committed");
        render_final_chord_structural_plan.continue_rest_bridge_extensions(&pre, post)?;
    }
    let receipt_checkpoint_before_final_notation = render_state.document()
        .unwrap_or(&initial_render.pre_preview_render_xml_for_gap).to_owned();
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MetricBeforeFinalRenderNotationAuthority",
        &receipt_checkpoint_before_final_notation,
    );
    let final_notation_authority = run_final_render_notation_authority_stage(
        FinalRenderNotationAuthorityStageInputs {
            render_state: &mut *render_state,
            final_render_source_available: initial_render.final_render_source_available,
            render_xml_trial_path: &initial_render.render_xml_trial_path,
            final_export_xml: &initial_render.final_export_xml,
            frozen_sounding_temporal_lane_plan:
                &initial_render.frozen_sounding_temporal_lane_plan,
            render_final_chord_structural_plan: &mut render_final_chord_structural_plan,
            frozen_sounding_material_attacks:
                &initial_render.frozen_sounding_material_attacks,
            render_pruned_final_chord_omissions:
                &render_pruned_final_chord_omissions,
        },
    )?;
    let receipt_checkpoint_after_final_notation = render_state.document()
        .unwrap_or(&initial_render.pre_preview_render_xml_for_gap).to_owned();
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MetricAfterFinalRenderNotationAuthority",
        &receipt_checkpoint_after_final_notation,
    );
    let mut render_pdf_source_occurrence_continuation = final_notation_authority
        .final_pdf_source_occurrence_continuation
        .clone();
    // r342: CrossStaffHandProjectionReadability must consume the committed Staff/Voice
    // render state. r341 proved that an earlier projection can be overwritten by the
    // final notation authority even though the shared BeamHierarchy decision is sound.
    if initial_render.final_render_source_available {
        let committed_cross_staff_root = render_state.require_state_root("M5 post-FinalNotation");
        let committed_source_xml = committed_cross_staff_root.document().to_owned();
        let note_count_before = committed_source_xml.matches("<note").count();
        external_feedback_closure_render_write_audit.attempted = true;
        external_feedback_closure_render_write_audit.note_count_before = note_count_before;
        external_feedback_closure_render_write_audit.note_count_after = note_count_before;
        external_feedback_closure_render_write_audit.parse_after_ok = parse_musicxml(&committed_source_xml).is_ok();
        external_feedback_closure_render_write_audit.sounding_stable = true;
        external_feedback_closure_render_write_audit.verdict = "committed_cross_staff_beam_projection_no_change".to_owned();
        let (before_events, backup_forward_before_ok) =
            collect_render_xml_note_events(&committed_source_xml);
        let before_signature = render_visible_sounding_signature(&before_events);
        let mut committed_after_xml = committed_source_xml.clone();
        let (committed_cross_staff_stats, committed_cross_staff_receipt) =
            close_committed_cross_staff_beam_gestures(&mut committed_after_xml);
        let mut committed_cross_staff_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        committed_cross_staff_receipts.push_preserving(committed_cross_staff_receipt, &committed_after_xml);
        let changed = committed_after_xml != committed_source_xml;
        if changed {
            let (after_events, backup_forward_after_ok) =
                collect_render_xml_note_events(&committed_after_xml);
            let after_signature = render_visible_sounding_signature(&after_events);
            let invariant_green = parse_musicxml(&committed_source_xml).is_ok()
                && parse_musicxml(&committed_after_xml).is_ok()
                && backup_forward_before_ok
                && backup_forward_after_ok
                && before_signature == after_signature
                && note_count_before == committed_after_xml.matches("<note").count();
            if !invariant_green {
                crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures);
                committed_cross_staff_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
                anyhow::bail!(
                    "CommittedCrossStaffBeamProjection: finaler Staff/Voice-Snapshot verletzt Parse/Cursor/Sounding/NoteCount-Invariante; kein Write"
                );
            }
            let transported_occurrence_continuation_result = render_pdf_source_occurrence_continuation
                .as_ref()
                .map(|continuation| {
                    structural_write_authority::transport_final_pdf_source_occurrence_continuation_through_committed_cross_staff_beam_projection(
                        continuation,
                        &committed_source_xml,
                        &committed_after_xml,
                        &render_final_chord_structural_plan,
                        &initial_render.frozen_sounding_temporal_lane_plan.frozen_serialized_voice_plan,
                        Some(&render_pruned_final_chord_omissions),
                    )
                })
                .transpose()
                .with_context(|| {
                    "CommittedCrossStaffBeamProjection durfte die FinalChord-Occurrence-Continuation nicht verÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¤ndern"
                });
            let transported_occurrence_continuation = match transported_occurrence_continuation_result {
                Ok(continuation) => continuation,
                Err(error) => {
                    crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures);
                    committed_cross_staff_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
                    return Err(error);
                }
            };
            if render_pdf_source_occurrence_continuation.is_none() {
                crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures);
                committed_cross_staff_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
                anyhow::bail!(
                    "CommittedCrossStaffBeamProjection kann ohne FinalChord-Occurrence-Continuation nicht kausal fortsetzen"
                );
            }
            render_state.adopt_transition(
                committed_cross_staff_root, &committed_after_xml, committed_cross_staff_receipts,
                structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::ExternalFeedbackClosure),
                |path, _pre, post| structural_write_authority::write_external_feedback_closure_checkpoint_authorized(path, post),
            ).with_context(|| "M5 CommittedCrossStaffBeamProjection sovereign adoption")?;
            render_pdf_source_occurrence_continuation = transported_occurrence_continuation;
            external_feedback_closure_render_write_audit.written = true;
            external_feedback_closure_render_write_audit.parse_after_ok = true;
            external_feedback_closure_render_write_audit.sounding_stable = true;
            external_feedback_closure_render_write_audit.note_count_after = note_count_before;
            external_feedback_closure_render_write_audit.verdict =
                "external_feedback_closure_written_render_xml_only".to_string();
            external_feedback_closure_render_write_audit.done_fragment.push_str(&format!(
                "; CommittedCrossStaffVoiceBeat {} Gruppen / {} Notenprojektionen nach finaler Staff/Voice-Authority",
                committed_cross_staff_stats.cross_staff_groups,
                committed_cross_staff_stats.rows_written,
            ));
            external_feedback_closure_render_write_audit.pipeline_line.push_str(&format!(
                " CommittedCrossStaffBeamProjection: Groups {}; Rows {}; BeamLevel1Rows {}; BeamLevel2Rows {}; FinalStaffVoiceSnapshot ja;",
                committed_cross_staff_stats.cross_staff_groups,
                committed_cross_staff_stats.rows_written,
                committed_cross_staff_stats.level1_rows,
                committed_cross_staff_stats.level2_rows,
            ));
        } else {
            committed_cross_staff_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        }
        external_feedback_closure_render_write_audit.cross_staff_beam_groups +=
            committed_cross_staff_stats.cross_staff_groups;
        external_feedback_closure_render_write_audit.beam_rows_written +=
            committed_cross_staff_stats.rows_written;
        external_feedback_closure_render_write_audit.beam_level1_rows +=
            committed_cross_staff_stats.level1_rows;
        external_feedback_closure_render_write_audit.beam_level2_rows +=
            committed_cross_staff_stats.level2_rows;
    }
    external_feedback_closure_render_write_audit.done_fragment = format!(
        "CommittedCrossStaffBeamProjection: {} Gruppen; {} Notenprojektionen; RenderXmlWrite {}",
        external_feedback_closure_render_write_audit.cross_staff_beam_groups,
        external_feedback_closure_render_write_audit.beam_rows_written,
        yes_no(external_feedback_closure_render_write_audit.written),
    );
    external_feedback_closure_render_write_audit.pipeline_line = format!(
        "CommittedCrossStaffBeamProjection: Attempted {}; Written {}; Groups {}; Rows {};",
        yes_no(external_feedback_closure_render_write_audit.attempted),
        yes_no(external_feedback_closure_render_write_audit.written),
        external_feedback_closure_render_write_audit.cross_staff_beam_groups,
        external_feedback_closure_render_write_audit.beam_rows_written,
    );
    let notation_voice_causal_lane_pressure_audit = {
        use crate::audit_only::notation_voice_causal_lane_pressure_audit::{
            build_notation_voice_causal_lane_pressure_audit_projection,
            NotationVoiceCanonicalOccurrenceIdentityRow,
            NotationVoiceCausalSourceEvidenceRow,
        };

        let attack_by_source = initial_render
            .frozen_sounding_material_attacks
            .iter()
            .filter(|attack| attack.exists)
            .map(|attack| (attack.source_event_id, attack))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut source_rows = Vec::<NotationVoiceCausalSourceEvidenceRow>::new();
        for lane_row in initial_render
            .frozen_sounding_temporal_lane_plan
            .rows
            .iter()
        {
            for member in lane_row.members.iter() {
                let Some(attack) = attack_by_source.get(&member.source_event_id).copied() else {
                    continue;
                };
                if attack.attack_id != member.attack_id {
                    continue;
                }
                let Ok(staff) = attack.staff.parse::<u8>() else {
                    continue;
                };
                if staff != lane_row.staff || !matches!(staff, 1 | 2) {
                    continue;
                }
                source_rows.push(NotationVoiceCausalSourceEvidenceRow {
                    source_event_id: member.source_event_id.0,
                    attack_id: member.attack_id.0,
                    measure: attack.measure,
                    onset: attack.onset,
                    pitch: attack.pitch,
                    staff,
                    absolute_start: lane_row.absolute_start,
                    absolute_end: lane_row.absolute_end,
                    technical_lane: lane_row.lane,
                });
            }
        }

        let occurrence_rows = render_pdf_source_occurrence_continuation
            .as_ref()
            .map(|continuation| {
                continuation
                    .declassified_occurrence_identity_rows()
                    .into_iter()
                    .map(|(
                        source_event_id,
                        attack_id,
                        measure,
                        pitched_ordinal,
                        materialization_kind_rank,
                        fragment_ordinal,
                    )| {
                        NotationVoiceCanonicalOccurrenceIdentityRow {
                            source_event_id,
                            attack_id,
                            measure,
                            pitched_ordinal,
                            materialization_kind_rank,
                            fragment_ordinal,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let audit_xml = render_pdf_source_occurrence_continuation
            .as_ref()
            .map_or_else(
                || {
                    render_state.document().unwrap_or(&initial_render.pre_preview_render_xml_for_gap).to_string()
                },
                |continuation| continuation.xml().to_string(),
            );
        build_notation_voice_causal_lane_pressure_audit_projection(
            &audit_xml,
            &source_rows,
            &occurrence_rows,
            initial_render
                .frozen_sounding_temporal_lane_plan
                .authority_bound,
        )
    };

    console_artifact_line(
        ConsoleArtifactLine::MasterMusicXml,
        format!(
            "schreibe kanonische Master-MusicXML {}",
            source.musicxml_out.display()
        ),
    );
    let artifact_progress_started = Instant::now();
    console_artifact_progress(
        ConsoleArtifactLine::MasterMusicXml,
        "Master-MusicXML schreiben",
        1,
        1,
        artifact_progress_started,
    );
    // TonalFrameFact: the only Render information #59/#60 consume, bound to
    // the Render snapshot this stage reads. The retired thread-local view is
    // compared once inside the binding; any divergence is fatal.
    let master_export_tonal_frame =
        structural_write_authority::bind_tonal_frame_fact_from_render_snapshot(
            render_state.document().map(str::to_owned),
        )
        .with_context(|| {
            "TonalFrameFact konnte nicht fail-closed an den Render-Snapshot gebunden werden"
        })?;
    let functional_master_respelling =
        structural_write_authority::FunctionalMasterRespellingOperation::new(
            canonical_master_functional_respelling_from_render,
        )
        .bind_tonal_frame(master_export_tonal_frame);
    let master_voice_label_alpha_audit =
        structural_write_authority::audit_musicxml_voice_label_binding_alpha(
            &initial_render.final_export_xml,
            &initial_render.frozen_sounding_temporal_lane_plan.final_chord_structural_plan,
            &initial_render.frozen_sounding_temporal_lane_plan.frozen_serialized_voice_plan,
        )
        .with_context(|| {
            "FrozenSerializedVoiceChordConstraintCutover Master-Label-Alpha-Gate konnte nicht ausgewertet werden"
        })?;
    if !master_voice_label_alpha_audit.green() {
        anyhow::bail!(
            "FrozenSerializedVoiceChordConstraintCutover Master-Label-Alpha-Gate blockiert: cases={} semantic_role_delta={} paired_binding_failures={} stale_binding_false_accepts={}",
            master_voice_label_alpha_audit.cases(),
            master_voice_label_alpha_audit.semantic_role_deltas(),
            master_voice_label_alpha_audit.paired_binding_failures(),
            master_voice_label_alpha_audit.stale_binding_false_accepts(),
        );
    }
    let final_chord_encoding_plan =
        structural_write_authority::bind_canonical_final_chord_encoding_plan(
            &initial_render.final_export_xml,
            &initial_render.frozen_sounding_temporal_lane_plan.final_chord_structural_plan,
            &initial_render.frozen_sounding_temporal_lane_plan.frozen_serialized_voice_plan,
        )
        .with_context(|| {
            "FrozenSerializedVoiceChordConstraintCutover konnte den Master-Kandidaten nicht an den kanonischen Plan binden"
        })?;
    let (final_chord_encoding_candidate, final_chord_encoding_receipt) =
        structural_write_authority::project_final_chord_encoding_plan(
            &initial_render.final_export_xml,
            &final_chord_encoding_plan,
        );
    let final_chord_encoding_candidate = final_chord_encoding_candidate.with_context(|| {
        "FinalChordEncodingPlan konnte den Master-Kandidaten nicht materialisieren"
    })?;
    let original_chains = final_chord_encoding_plan.continue_original_respelling_chains(
        &initial_render.final_export_xml, &final_chord_encoding_candidate,
        final_chord_encoding_receipt.as_ref(),
    );
    let functional_master_respelling = functional_master_respelling.bind_original_chains(original_chains);
    let (master_written, master_export_fifths_receipt, functional_respelling_receipt) =
        structural_write_authority::write_master_musicxml_output_authorized(
            &source.musicxml_out,
            &final_chord_encoding_candidate,
            functional_master_respelling,
        );
    master_written.with_context(|| {
        format!(
            "Konnte MusicXML-Ausgabe nicht schreiben: {}",
            source.musicxml_out.display()
        )
    })?;
    let committed_master_export = CommittedMasterExport::realize(
        &initial_render.final_export_xml,
        &final_chord_encoding_candidate,
        final_chord_encoding_receipt,
        master_export_fifths_receipt,
        functional_respelling_receipt,
    );
    let committed_master_source =
        render::final_pdf_source_commit::bind_committed_master_source_before_renderer_authorized(
            &source.musicxml_out,
        );
    let committed_master_xml = committed_master_source
        .committed_xml()
        .map(str::to_string)
        .with_context(|| {
            format!(
                "RendererFeasibilityCommittedSourcePhase konnte committed Master-Quelle nicht binden: {}",
                source.musicxml_out.display()
            )
        })?;
    let _ = validate_musicxml_export_semantics_from_xml(
        &source.musicxml_out,
        &committed_master_xml,
    )?;
    // The written Master file is an artifact witness only: its bytes must equal
    // the in-memory export realized from the owners' EXACT scripts.
    if committed_master_xml != committed_master_export.document() {
        master_state_identity::master_sovereignty_violation(
            "MasterExport: written Master artifact bytes differ from the realized owner export",
        );
    }

    // The existing phrase owner receives the same freshly committed Master
    // bytes as before, now before technical Render channel closure. In a cold
    // project this artifact does not exist at InitialRender entry.
    let phrase_capture_start = std::time::Instant::now();
    capture_phrase_boundary_evidence_foundation(source, &committed_master_xml);
    let _ = pipeline_stage_timing(&source.input_label, "PhraseBoundaryEvidenceCapture", phrase_capture_start);

    let render_pdf_source_evidence_ledger = RenderPdfSourceEvidenceLedger::new([
        (
            final_notation_authority
                .authorized_writer_entry_boundary_audit
                .write_produced(),
            "Render-MusicXML nach AuthorizedCursorNeutralWriter",
        ),
        (
            final_notation_authority
                .lower_clef_surface_materialization_audit
                .written,
            "Render-XML nach TypedLowerClefSurfaceMaterialization",
        ),
        (
            rest_and_articulation
                .piano_final_rest_glyph_canonical_height_write_audit
                .written,
            "Render-XML nach PianoFinalRestGlyphCanonicalHeightRenderWrite",
        ),
        (
            rest_and_articulation
                .fragmented_rest_chain_transactional_write_trial
                .written,
            "Render-XML nach FragmentedRestChainTransactionalWrite",
        ),
        (
            initial_render
                .final_visible_rest_role_purge_and_vertical_placement_write_audit
                .written,
            "Render-XML nach RestGlyphVisualEnvelopeStandardFix",
        ),
        (
            initial_render
                .strict_visible_rest_arithmetic_materializer_trial
                .written,
            "Render-XML nach FinalRestClosureByInvariantProof",
        ),
        (
            initial_render
                .strict_staff_local_release_completeness_trial
                .written,
            "Render-XML nach StrictStaffOverfullCoverageNormalization",
        ),
        (
            rest_and_articulation
                .wav_truth_binding_clear_tiny_accent_render_write_audit
                .written,
            "Render-XML nach WavTruthBindingClearAccentRenderWrite",
        ),
        (
            external_feedback_closure_render_write_audit.written,
            "Render-XML nach ExternalFeedbackClosureRenderWrite",
        ),
        (
            rest_and_articulation
                .external_notation_feedback_beam_variant_a_tiny_render_write_audit
                .written,
            "Render-XML nach ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback",
        ),
        (
            rest_and_articulation
                .single_clear_staccato_render_write_audit
                .written,
            "Render-XML nach SingleClearStaccatoRenderWriteWithPlacementReadback",
        ),
        (
            initial_render.external_feedback_safe_notation_write_audit.written,
            "Render-XML nach TieMergeAccountingAndTrueTransactionalRestCoalesce",
        ),
        (
            initial_render.upper_staff_final_duplicate_rest_rewrite_written,
            "Render-XML nach UpperStaffDuplicateRestIntervalForwardRewriteProof",
        ),
        (
            initial_render.cursor_lane_end_invariant_repair_audit.reanchor_contract_active
                && final_notation_authority.final_render_source_available,
            "Render-XML nach CursorLaneReanchorRestVisibilityContract",
        ),
        (
            initial_render
                .lower_staff_secondary_reanchored_lane_rest_visibility_suppression_trial
                .written,
            "Render-XML nach ReanchorRestVisibilitySafetyNet",
        ),
        (
            initial_render
                .lower_staff_visible_rhythm_rest_lane_trial
                .written,
            "Render-XML nach UpperStaffDuplicateRestIntervalForwardRewriteProof",
        ),
        (
            initial_render.cross_staff_hand_role_materialization_trial.written,
            "Render-XML nach CrossStaffHandRoleMaterializationTrial",
        ),
        (
            initial_render.behind_bars_rule_check_rest_and_tie_audit.written,
            "Render-XML nach BehindBarsRestAndTieAudit",
        ),
        (
            initial_render.render_xml_pruned_pdf_trial.pdf_source_selected,
            "Render-XML",
        ),
        (
            key_signature_enharmonic_spelling_render_write_audit.written,
            "MusicXML",
        ),
    ]);
    // Boundary eligibility: only these two Rest/Articulation values survive their
    // final productive consumer. They remain local variables until the single
    // MetricFinalNotationReleaseEvidence contract is assembled at stage exit.
    let RestAndArticulationRenderStageOutcome {
        repair_loop_invocation_evidence,
        ..
    } = rest_and_articulation;
    let render_xml_candidate = render_state.document().map(str::to_string);
    // r48 committed-plan single-owner runtime closure: keep PDF-source adoption inside the already-live
    // Metric/FinalNotation owner. Do not introduce a new writer-reachable
    // helper outside StructuralWriteAuthority. The decision consumes only the
    // in-memory snapshot and existing writer-boundary evidence.
    let mut authorized_writer_entry_boundary_audit = final_notation_authority
        .authorized_writer_entry_boundary_audit
        .clone();
    let mut render_xml_selected_for_pdf = render_pdf_source_evidence_ledger.render_requested();
    let mut render_xml_pdf_source_raw_cursor_evidence =
        RenderXmlPdfSourceRawCursorGuardEvidence::default();
    let mut authorized_writer_candidate_file_exact_match = false;
    let mut render_occurrence_continuation_file_exact_match = false;
    if render_xml_selected_for_pdf {
        if let Some(render_xml_candidate) = render_xml_candidate.as_deref() {
            authorized_writer_candidate_file_exact_match =
                authorized_writer_entry_boundary_audit.write_produced()
                    && render_xml_candidate
                        == final_notation_authority.authorized_writer_candidate_xml.as_str();
            render_occurrence_continuation_file_exact_match =
                render_pdf_source_occurrence_continuation
                    .as_ref()
                    .is_some_and(|continuation| continuation.xml() == render_xml_candidate);
            let render_raw_rows = scan_raw_cursor_xml("RenderXML", render_xml_candidate);
            let render_hard = render_raw_rows
                .iter()
                .filter(|row| row.hard_violation)
                .count();
            let render_max_overflow = render_raw_rows
                .iter()
                .map(|row| (row.cursor_max - row.expected_duration).max(0))
                .max()
                .unwrap_or(0);
            let render_partial_backups: usize = render_raw_rows
                .iter()
                .map(|row| row.partial_backup_count)
                .sum();
            let render_post_expected: usize = render_raw_rows
                .iter()
                .map(|row| row.post_expected_events)
                .sum();
            let render_raw_green = render_hard == 0
                && render_max_overflow == 0
                && render_partial_backups == 0
                && render_post_expected == 0;
            render_xml_pdf_source_raw_cursor_evidence =
                RenderXmlPdfSourceRawCursorGuardEvidence {
                    checked: true,
                    green: render_raw_green,
                    measures: render_raw_rows.len(),
                    hard_violations: render_hard,
                    max_overflow: render_max_overflow,
                    partial_backups: render_partial_backups,
                    post_expected_events: render_post_expected,
                };
            if !render_raw_green {
                render_xml_selected_for_pdf = false;
            }
        } else {
            render_xml_selected_for_pdf = false;
            render_xml_pdf_source_raw_cursor_evidence =
                RenderXmlPdfSourceRawCursorGuardEvidence {
                    checked: true,
                    green: false,
                    ..RenderXmlPdfSourceRawCursorGuardEvidence::default()
                };
        }
    }

    let semantic_lower_clef_pdf_force_reason =
        semantic_lower_clef_pdf_render_input_force_reason(
            final_notation_authority
                .lower_clef_surface_materialization_audit
                .written,
            &source.musicxml_out,
            &initial_render.render_xml_trial_path,
        );
    if semantic_lower_clef_pdf_force_reason.is_some() {
        render_xml_selected_for_pdf = true;
    }

    if authorized_writer_entry_boundary_audit.write_produced() {
        let authorized_writer_adoption_release_green = render_xml_pdf_source_raw_cursor_evidence
            .checked
            && render_xml_pdf_source_raw_cursor_evidence.green
            && authorized_writer_entry_boundary_audit.renderer_contiguity_green()
            && final_notation_authority
                .authorized_materialization_equivalence_audit
                .green()
            && authorized_writer_candidate_file_exact_match
            && render_occurrence_continuation_file_exact_match;
        render_xml_selected_for_pdf = authorized_writer_adoption_release_green;
    }

    let pdf_source_label = if render_xml_selected_for_pdf {
        render_pdf_source_evidence_ledger.selected_label()
    } else {
        "MusicXML"
    };
    let authorized_writer_pdf_source_selected =
        authorized_writer_entry_boundary_audit.write_produced() && render_xml_selected_for_pdf;
    authorized_writer_entry_boundary_audit.bind_pdf_source_adoption_truth(
        render_xml_pdf_source_raw_cursor_evidence.checked
            && render_xml_pdf_source_raw_cursor_evidence.green,
        final_notation_authority
            .renderer_cross_staff_beam_capability_audit
            .green(),
        final_notation_authority
            .authorized_materialization_equivalence_audit
            .green(),
        authorized_writer_candidate_file_exact_match,
        authorized_writer_pdf_source_selected,
        pdf_source_label,
    );
    let render_write_pdf_source_adoption_truth_gate_audit =
        build_render_write_pdf_source_adoption_truth_gate(
            "AuthorizedCursorNeutralWriter",
            !no_pdf,
            authorized_writer_entry_boundary_audit.write_produced(),
            render_xml_pdf_source_raw_cursor_evidence,
            authorized_writer_entry_boundary_audit.renderer_contiguity_green(),
            final_notation_authority
                .renderer_cross_staff_beam_capability_audit
                .green(),
            final_notation_authority
                .authorized_materialization_equivalence_audit
                .green(),
            authorized_writer_candidate_file_exact_match,
            authorized_writer_pdf_source_selected,
            pdf_source_label,
        );
    let render_write_pdf_source_adoption_truth_gate_blocks_release =
        render_write_pdf_source_adoption_truth_gate_audit.blocks_release;

    crate::master_state_identity::record_render_stage_boundary(
        crate::master_state_identity::RenderStageBoundary::PdfSourceSelection);
    let pdf_render_requested = final_blockers == 0 && !no_pdf;
    let mut render_pdf_source_voice_plan = initial_render
        .frozen_sounding_temporal_lane_plan
        .frozen_serialized_voice_plan
        .clone();
    let mut approved_phrase_occurrence_course = None;
    let mut primary_beam_voice_partition_plan:
        Option<structural_write_authority::PrimaryBeamVoicePartitionBoundPlan> = None;

    if pdf_render_requested && render_xml_selected_for_pdf {
        let baseline_occurrence_continuation = render_pdf_source_occurrence_continuation
            .as_ref()
            .with_context(|| {
                "PrimaryBeamVoicePartition baseline lost FinalChord occurrence continuation"
            })?;
        let baseline_render_xml = baseline_occurrence_continuation.xml().to_string();

        let project_root_hint = source
            .pdf_out
            .parent()
            .filter(|path| !path.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."));
        let project_root = std::fs::canonicalize(project_root_hint).with_context(|| {
            "PrimaryBeamVoicePartition baseline could not canonicalize project root"
        })?;
        let shadow_project_slot = source
            .input_label
            .chars()
            .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
            .collect::<String>();
        let shadow_dir = project_root
            .join("target")
            .join("etuschlopf_renderer_feasibility_shadow")
            .join(if shadow_project_slot.is_empty() {
                "project__primary_beam_voice_partition_scope".to_string()
            } else {
                format!("{shadow_project_slot}__primary_beam_voice_partition_scope")
            });
        let shadow_musicxml = shadow_dir.join("constraint_candidate.render.musicxml");
        let shadow_pdf = shadow_dir.join("constraint_candidate.pdf");
        std::fs::create_dir_all(&shadow_dir).with_context(|| {
            format!(
                "PrimaryBeamVoicePartition baseline could not create shadow directory: {}",
                shadow_dir.display()
            )
        })?;
        structural_write_authority::write_renderer_feasibility_shadow_candidate_authorized(
            &shadow_musicxml,
            &baseline_render_xml,
        )?;
        let shadow_occurrence_continuation=baseline_occurrence_continuation.with_rest_bridge_shadow_diagnostics();
        let shadow_final_notation_commit =
            render::final_pdf_source_commit::finalize_pdf_source_before_renderer_authorized(&mut *render_state,
&shadow_musicxml,
render::final_pdf_source_commit::FinalPdfSourceDisposition::Shadow,
render::final_pdf_source_commit::FinalPdfSourcePlans { chord: &render_final_chord_structural_plan, voice: &render_pdf_source_voice_plan },
&render_pruned_final_chord_omissions,
&shadow_occurrence_continuation,
None)?;
        let shadow_final_notation_xml = shadow_final_notation_commit
            .committed_xml()
            .with_context(|| {
                "PrimaryBeamVoicePartition baseline FinalPdfSourceCommit remained open"
            })?
            .to_string();
        let (shadow_horizontal_layout_plan, _) = build_final_notation_plan(
            &shadow_final_notation_xml,
            horizontal_layout_seed,
            FinalNotationPlanEvidenceProjection::RunLocalProbeSilent,
        )?;
        if !shadow_horizontal_layout_plan.provenance_complete() {
            anyhow::bail!(
                "PrimaryBeamVoicePartition baseline HorizontalLayout plan has incomplete provenance"
            );
        }
        let shadow_directives = shadow_horizontal_layout_plan.materialization_directives();
        let (shadow_layout_result, shadow_layout_receipt) = structural_write_authority::materialize_final_render_notation_with_phrase_projection_authorized(
            &shadow_musicxml, &shadow_directives, &[]);
        let shadow_layout_post = render::final_pdf_source_commit::discard_shadow_layout_receipt_post(
            &shadow_final_notation_xml, shadow_layout_receipt)?;
        shadow_layout_result.inspect_err(|_error| {
            crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures); })?;
        let shadow_commit =
            render::final_pdf_source_commit::preserve_pdf_source_commit_through_horizontal_layout_authorized(
                &shadow_musicxml,
                &shadow_final_notation_commit,
                &shadow_final_notation_xml,
                &shadow_layout_post,
                &render_final_chord_structural_plan,
                &render_pdf_source_voice_plan,
                &render_pruned_final_chord_omissions,
            )?;
        let shadow_committed_xml = shadow_commit
            .committed_xml()
            .with_context(|| {
                "PrimaryBeamVoicePartition baseline lost committed shadow source"
            })?
            .to_string();
        let shadow_render = render_pdf_if_possible(&shadow_musicxml, &shadow_pdf, &shadow_commit);
        if !matches!(shadow_render.status, RenderStatus::Written) {
            anyhow::bail!(
                "PrimaryBeamVoicePartition baseline shadow renderer not GREEN: {}",
                shadow_render.message
            );
        }

        let scope_projection = &initial_render
            .frozen_sounding_temporal_lane_plan
            .notation_voice_pre_serialization_constraint_audit;
        let repair_candidates = scope_projection
            .metric_line_candidates()
            .iter()
            .filter_map(|candidate| {
                let closed_matches = scope_projection
                    .closed_candidates()
                    .iter()
                    .filter(|closed| {
                        closed.staff == candidate.staff
                            && closed.source_event_ids == candidate.source_event_ids
                            && closed.members == candidate.members
                    })
                    .collect::<Vec<_>>();
                if closed_matches.len() != 1 {
                    return None;
                }
                let closed = closed_matches[0];
                if !scope_projection
                    .pre_final_primary_beam_exact_proof_chain_ids()
                    .contains(&closed.proof_chain_id)
                {
                    return None;
                }
                let receipts = scope_projection
                    .canonical_primary_beam_owner_receipts()
                    .iter()
                    .filter(|receipt| receipt.proof_chain_id == closed.proof_chain_id)
                    .collect::<Vec<_>>();
                if receipts.len() != 1 {
                    return None;
                }
                let receipt = receipts[0];
                if receipt.event_count != candidate.members.len()
                    || receipt.primary_states.len() != candidate.members.len()
                {
                    return None;
                }
                let target_evidence = scope_projection
                    .materialization_safety_primary_target_evidence_rows()
                    .iter()
                    .filter(|row| row.metric_line_id == candidate.metric_line_id)
                    .collect::<Vec<_>>();
                if target_evidence.len() != 1 {
                    return None;
                }
                let target_evidence = target_evidence[0];
                Some(structural_write_authority::PrimaryBeamVoicePartitionRepairCandidate::new(
                    candidate.metric_line_id.clone(),
                    candidate
                        .members
                        .iter()
                        .map(|member| (member.source_event_id, member.attack_id))
                        .collect(),
                    candidate.measure,
                    candidate.staff,
                    receipt
                        .primary_states
                        .iter()
                        .map(|state| (*state).to_string())
                        .collect(),
                    (
                        target_evidence.target_primary_source_event_id,
                        target_evidence.target_primary_attack_id,
                    ),
                    target_evidence.target_channel,
                ))
            })
            .collect::<Vec<_>>();

        let renderer_geometry = crate::audit_only::renderer_system_break_readback::
            primary_beam_geometry_readback_xml_pdf(&shadow_committed_xml, &shadow_pdf)?;
        let mut visible_fragments_by_metric_line_id =
            std::collections::BTreeMap::<String, usize>::new();
        for cohort in structural_write_authority::primary_beam_voice_partition_renderer_cohorts(
            baseline_occurrence_continuation,
            &repair_candidates,
        ) {
            let unusual = crate::audit_only::original_beam_diagnostics::record(
                crate::audit_only::original_beam_diagnostics::Observation::CohortProject,
            );
            if unusual || crate::audit_only::original_beam_diagnostics::detail_requested_for_measure(cohort.measure()) {
                eprintln!("OriginalBeamCohortProject: project={}; group={}; origin=actual_owner_continuation; authority=none", source.input_label, cohort.metric_line_id());
            }
            if let Some(geometry) = renderer_geometry.cohort_geometry(
                cohort.measure(),
                cohort.staff(),
                cohort.pitched_ordinals(),
            ) {
                visible_fragments_by_metric_line_id
                    .insert(cohort.metric_line_id().to_string(), geometry.visible_fragments);
            }
        }

        let bound_plan = structural_write_authority::bind_primary_beam_voice_partition_plan(
            &baseline_render_xml,
            structural_write_authority::PrimaryBeamVoicePartitionBindingContext {
                source_continuation: baseline_occurrence_continuation,
                voice_plan: &render_pdf_source_voice_plan,
                candidates: &repair_candidates,
                classification_xml: &shadow_committed_xml,
                renderer_visible_fragments_by_metric_line_id:
                    &visible_fragments_by_metric_line_id,
                materialization_safety_proven_metric_line_ids: scope_projection
                    .materialization_safety_proven_metric_line_ids(),
            },
        )?;
        let historical_phrase_geometry = crate::audit_only::renderer_system_break_readback::
            historical_phrase_policy_decision_geometry(&baseline_render_xml,&shadow_pdf)?;
        let historical_phrase_course = structural_write_authority::bind_historical_metric_phrase_decision_course(
            &baseline_render_xml,
            structural_write_authority::PrimaryBeamVoicePartitionBindingContext {
                source_continuation: baseline_occurrence_continuation,
                voice_plan: &render_pdf_source_voice_plan, candidates: &repair_candidates,
                classification_xml: &shadow_committed_xml,
                renderer_visible_fragments_by_metric_line_id: &std::collections::BTreeMap::new(),
                materialization_safety_proven_metric_line_ids: scope_projection.materialization_safety_proven_metric_line_ids(),
            }, &historical_phrase_geometry)?;
        approved_phrase_occurrence_course = Some(shadow_commit.approve_phrase_course_after_historical_metric_partition(
            &crate::audit_only::phrase_boundary_evidence::pending_visual_notation_intents(),
            &historical_phrase_course, &render_final_chord_structural_plan, &render_pruned_final_chord_omissions)?);
        if bound_plan.scope_candidates() > 0 {
            primary_beam_voice_partition_plan = Some(bound_plan);
        }
    }

    // Retract only the real E050 operation whose carried connection would be
    // separated by this already-bound simultaneous assignment. The existing
    // E050 owner and its typed sink still own every beam write.
    // r1484: provisional bridges are decided here even without a late plan; if
    // their custody never reached this continuation the run fails visibly.
    anyhow::ensure!(render_pdf_source_occurrence_continuation.is_some()
        || !render_final_chord_structural_plan.has_provisional_rest_bridges(),
        "Provisional rest bridge reached no withdrawal decision");
    let provisional_without_plan = primary_beam_voice_partition_plan.is_none()
        && render_pdf_source_occurrence_continuation.as_ref()
            .is_some_and(|course| course.has_provisional_rest_bridges());
    if let (Some(plan_or_fallback), Some(course)) = (
        primary_beam_voice_partition_plan.as_ref().map(Some).or(provisional_without_plan.then_some(None)),
        render_pdf_source_occurrence_continuation.as_ref())
    {
        let guard = match plan_or_fallback {
            Some(plan) => render_final_chord_structural_plan.bind_rest_bridge_withdrawal(course, plan)?,
            None => render_final_chord_structural_plan.bind_provisional_rest_bridge_fallback(course)?,
        };
        let root = render_state.require_state_root("Bound rest bridge withdrawal");
        anyhow::ensure!(root.document() == course.xml(), "Bridge withdrawal Main/source mismatch");
        let ((post, audit), receipt, pending) = apply_structural_rest_bridge_beam_variant_a_render_write(
            course.xml(), "carried E050 connection before bound final voice assignment",
            &initial_render.render_xml_trial_path, &guard);
        anyhow::ensure!(pending.is_none(), "Bridge withdrawal may not mint replacement extension evidence");
        if audit.written {
            let carried = course.through_rest_bridge_withdrawal(&post, &guard,
                receipt.as_ref().context("Bridge withdrawal lacks owner receipt")?)?;
            let mut chain = crate::master_state_identity::RenderTransitionReceipts::new();
            chain.push_preserving(receipt, &post);
            render_state.adopt_transition(root, &post, chain,
                structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::SisterFeedbackBeam),
                |path, _pre, verified| structural_write_authority::write_sister_feedback_beam_checkpoint_authorized(path, verified))?;
            render_pdf_source_occurrence_continuation = Some(carried);
        } else {
            let mut chain = crate::master_state_identity::RenderTransitionReceipts::new();
            chain.push_preserving(receipt, &post);
            chain.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        }
    }

    // Boundary eligibility: the complete FinalRenderNotationAuthorityStageOutcome
    // remains local through its writer/source-adoption consumers. Only the remaining
    // live audits survive until the single release-evidence contract is assembled.
    let FinalRenderNotationAuthorityStageOutcome {
        final_render_source_available: terminal_render_source_available,
        lower_clef_surface_materialization_audit,
        ..
    } = final_notation_authority;
    let pdf_source_path = if render_xml_selected_for_pdf {
        initial_render.render_xml_trial_path.as_path()
    } else {
        source.musicxml_out.as_path()
    };
    let mut final_pdf_source_commit = if pdf_render_requested {
        if render_xml_selected_for_pdf {
            let render_occurrence_continuation = render_pdf_source_occurrence_continuation
                .as_ref()
                .with_context(|| {
                    "FinalPdfSourceCommit selected Render source lost FinalChord occurrence continuation"
                })?;
            Some(
                render::final_pdf_source_commit::finalize_pdf_source_before_renderer_authorized(&mut *render_state,
pdf_source_path,
render::final_pdf_source_commit::FinalPdfSourceDisposition::Live,
render::final_pdf_source_commit::FinalPdfSourcePlans { chord: &render_final_chord_structural_plan, voice: &render_pdf_source_voice_plan },
&render_pruned_final_chord_omissions,
render_occurrence_continuation,
primary_beam_voice_partition_plan.as_ref())?,
            )
        } else {
            Some(committed_master_source)
        }
    } else {
        None
    };
    if let (Some(commit), Some(course)) = (final_pdf_source_commit.as_mut(), approved_phrase_occurrence_course.as_ref()) {
        commit.continue_phrase_course(course)?;
    }
    if let Some(commit) = final_pdf_source_commit.as_ref() {
        if let Some(committed_voice_plan) = commit.committed_voice_plan() {
            render_pdf_source_voice_plan = committed_voice_plan.clone();
        }
        let changed = commit.primary_beam_voice_partition_changed_voice_elements();
        if primary_beam_voice_partition_plan.is_none() && changed != 0 {
            anyhow::bail!(
                "PrimaryBeamVoicePartition existing writer root wrote without a bound plan"
            );
        }
    }
    // r490: The terminal temporal replay remains the existing read-only
    // checkpoint-bound proof. Its productive call edge stays in this owner, but
    // it now runs only after the existing FinalPdfSourceCommit had the chance to
    // canonicalize a selected Render source and register that exact committed
    // checkpoint. If Render is not selected (or PDF is not requested), the same
    // proof still observes the last committed Render checkpoint and remains
    // fail-closed rather than manufacturing a new truth in Release/Report code.
    if terminal_render_source_available {
        structural_write_authority::validate_terminal_render_temporal_postcondition_authorized(
            &initial_render.render_xml_trial_path,
            render_state.document().ok_or_else(|| anyhow::anyhow!("terminal render has no sovereign state"))?,
        )
        .with_context(|| {
            format!(
                "Finale Render-MusicXML verletzt nach FinalPdfSourceCommit die terminale serialisierte Zeitinvariante: {}",
                initial_render.render_xml_trial_path.display()
            )
        })?;
    }
    let committed_source_xml = if let Some(final_commit) = final_pdf_source_commit.as_ref() {
        final_commit
            .committed_xml()
            .map(str::to_string)
            .with_context(|| {
                format!(
                    "RendererFeasibilityCommittedSourcePhase konnte committed PDF-Quelle nicht binden: {}",
                    pdf_source_path.display()
                )
            })?
    } else if render_xml_selected_for_pdf {
        render_xml_candidate.with_context(|| {
            format!(
                "RendererFeasibilityCommittedSourcePhase konnte ausgewaehlte Render-Quelle nicht binden: {}",
                initial_render.render_xml_trial_path.display()
            )
        })?
    } else {
        committed_master_xml.clone()
    };
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "MetricCommittedPdfSource",
        &committed_source_xml,
    );

    // Build the committed-source Joint Plan exactly once. It is the immutable
    // typed plan token shared by the veto-only oracle and the later release stage.
    // This avoids a second planner execution and therefore duplicate planner evidence.
    let (committed_initial_horizontal_layout_plan, renderer_feasibility_basis) =
        build_final_notation_plan(
            &committed_source_xml,
            horizontal_layout_seed,
            FinalNotationPlanEvidenceProjection::AuthoritativeConsole,
        )?;
    if !committed_initial_horizontal_layout_plan.provenance_complete() {
        anyhow::bail!(
            "RendererFeasibilityCommittedSourcePhase: initialer Joint-Plan besitzt unvollstaendige Provenienz"
        );
    }

    let renderer_feasibility_oracle_outcome = if pdf_render_requested {
        let oracle_result = (|| -> anyhow::Result<RendererFeasibilityOracleOutcome> {
            const MAX_ORACLE_ROUNDS: usize = 8;
            let source_fingerprint =
                renderer_feasibility_source_fingerprint(&committed_source_xml);
            let initial_plan = committed_initial_horizontal_layout_plan.clone();

            let project_root_hint = source
                .pdf_out
                .parent()
                .filter(|path| !path.as_os_str().is_empty())
                .unwrap_or_else(|| std::path::Path::new("."));
            let project_root = std::fs::canonicalize(project_root_hint).with_context(|| {
                "RendererFeasibilityCommittedSourceOracle konnte Projektwurzel nicht kanonisieren"
            })?;
            let shadow_project_slot = source
                .input_label
                .chars()
                .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
                .collect::<String>();
            let shadow_dir = project_root
                .join("target")
                .join("etuschlopf_renderer_feasibility_shadow")
                .join(if shadow_project_slot.is_empty() {
                    "project".to_string()
                } else {
                    shadow_project_slot
                });
            let shadow_render_musicxml =
                shadow_dir.join("constraint_candidate.render.musicxml");
            let shadow_master_musicxml = shadow_dir.join("constraint_candidate.musicxml");
            let shadow_musicxml = if render_xml_selected_for_pdf {
                shadow_render_musicxml.as_path()
            } else {
                shadow_master_musicxml.as_path()
            };
            let shadow_pdf = shadow_dir.join("constraint_candidate.pdf");

            let mut candidate_plan = initial_plan.clone();
            let mut page_rejections = RendererPartitionRejections::new(&committed_source_xml);
            let mut no_goods = std::collections::BTreeSet::<(usize, usize)>::new();
            for round in 1..=MAX_ORACLE_ROUNDS {
                eprintln!(
                    "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=begin | SourceFingerprint={:016x} | PlanPageStartsAndSystemLengths={:?} | SystemNoGoods={:?} | RejectedPartitions={:?} | ProjectionOnly=true | Authority=none",
                    source.input_label, round, source_fingerprint,
                    authoritative_page_shape(&candidate_plan).map(|shape| (shape.page_starts, shape.page_system_lengths)), no_goods, page_rejections.partitions,
                );
                if render_xml_selected_for_pdf {
                    structural_write_authority::write_renderer_feasibility_shadow_candidate_authorized(
                        &shadow_master_musicxml,
                        &committed_master_xml,
                    )?;
                }

                structural_write_authority::write_renderer_feasibility_shadow_candidate_authorized(
                    shadow_musicxml,
                    &committed_source_xml,
                )?;
                let shadow_directives = candidate_plan.materialization_directives();
                let (shadow_layout_result, shadow_layout_receipt) = structural_write_authority::materialize_final_render_notation_with_phrase_projection_authorized(
                    shadow_musicxml, &shadow_directives, &[]);
                let shadow_layout_post = render::final_pdf_source_commit::discard_shadow_layout_receipt_post(
                    &committed_source_xml, shadow_layout_receipt)?;
                shadow_layout_result.inspect_err(|_error| {
            crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures); })?;

                let shadow_commit = if render_xml_selected_for_pdf {
                    let committed_source = final_pdf_source_commit.as_ref().with_context(|| {
                        "RendererFeasibilityCommittedSourceOracle verlor den committed FinalPdfSource-Zustand"
                    })?;
                    render::final_pdf_source_commit::
                        preserve_pdf_source_commit_through_horizontal_layout_authorized(
                            shadow_musicxml,
                            committed_source,
                            &committed_source_xml,
                            &shadow_layout_post,
                            &render_final_chord_structural_plan,
                            &render_pdf_source_voice_plan,
                            &render_pruned_final_chord_omissions,
                        )
                        .with_context(|| {
                            "RendererFeasibilityCommittedSourceOracle konnte den bereits bewiesenen FinalPdfSource-Zustand nicht durch HorizontalLayout erhalten"
                        })?
                } else {
                    render::final_pdf_source_commit::bind_committed_master_source_before_renderer_authorized(
                        shadow_musicxml,
                    )
                };
                let shadow_committed_xml = shadow_commit
                    .committed_xml()
                    .with_context(|| {
                        "RendererFeasibilityCommittedSourceOracle konnte committed Shadow-Quelle nicht binden"
                    })?;

                if !candidate_plan.matches_materialized(shadow_committed_xml)? {
                    anyhow::bail!(
                        "RendererFeasibilityCommittedSourceOracle: committed Shadow-Quelle driftet vom Candidate-Plan"
                    );
                }

                let shadow_render = render_pdf_if_possible(
                    shadow_musicxml,
                    &shadow_pdf,
                    &shadow_commit,
                );
                eprintln!(
                    "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=renderer_returned | Status={:?} | Message={:?} | ProjectionOnly=true | Authority=none",
                    source.input_label, round, shadow_render.status, shadow_render.message,
                );
                if !matches!(shadow_render.status, RenderStatus::Written) {
                    anyhow::bail!(
                        "RendererFeasibilityCommittedSourceOracle: Shadow-Renderer nicht GREEN: {}",
                        shadow_render.message
                    );
                }
                let shadow_observation =
                    crate::audit_only::renderer_system_break_readback::audit_paths(
                        shadow_musicxml,
                        &shadow_pdf,
                    ).inspect_err(|error| {
                        eprintln!(
                            "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=readback_error | Reason={:?} | ProjectionOnly=true | Authority=none",
                            source.input_label, round, error.to_string(),
                        );
                    })?;
                eprintln!(
                    "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=readback | Observation={:?} | ProjectionOnly=true | Authority=none",
                    source.input_label, round, shadow_observation,
                );

                if !page_rejections.partitions.is_empty() || !no_goods.is_empty() {
                    page_rejections.bind(&committed_source_xml, shadow_render.probe_identity.as_ref())?;
                    page_rejections.validate_horizontal_context(&initial_plan, &candidate_plan)?;
                }
                if shadow_observation.inserted_system_breaks.is_empty()
                    && shadow_observation.collapsed_system_breaks.is_empty()
                    && shadow_observation.inserted_page_breaks.is_empty()
                    && shadow_observation.collapsed_page_breaks.is_empty()
                {
                    eprintln!(
                        "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=confirmed | Reason=all_four_boundary_differences_empty | ProjectionOnly=true | Authority=none",
                        source.input_label, round,
                    );
                    return Ok(RendererFeasibilityOracleOutcome::Confirmed {
                        plan: candidate_plan,
                        rounds: round,
                        no_goods,
                        source_fingerprint,
                    });
                }
                if !shadow_observation.collapsed_system_breaks.is_empty()
                    || !shadow_observation.collapsed_page_breaks.is_empty()
                {
                    eprintln!(
                        "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=rejected | Reason=collapsed_boundaries_unsupported | ProjectionOnly=true | Authority=none",
                        source.input_label, round,
                    );
                    anyhow::bail!(
                        "RendererFeasibilityCommittedSourceOracle: unsupported Renderer-Drift; collapsedSystems={:?} insertedPages={:?} collapsedPages={:?}",
                        shadow_observation.collapsed_system_breaks,
                        shadow_observation.inserted_page_breaks,
                        shadow_observation.collapsed_page_breaks,
                    );
                }

                let before = no_goods.len();
                let page_before = page_rejections.partitions.len();
                if !shadow_observation.inserted_page_breaks.is_empty() {
                    eprintln!(
                        "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=rejected | Reason=inserted_pages_exclude_exact_complete_partition | ProjectionOnly=true | Authority=none",
                        source.input_label, round,
                    );
                    page_rejections.bind(&committed_source_xml, shadow_render.probe_identity.as_ref())?;
                    page_rejections.record(&candidate_plan, &shadow_observation)?;
                }
                if !shadow_observation.inserted_system_breaks.is_empty() {
                    page_rejections.bind(&committed_source_xml, shadow_render.probe_identity.as_ref())?;
                    let ranges = page_rejections.system_ranges(&initial_plan, &candidate_plan, &shadow_observation)?;
                    eprintln!(
                        "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=rejected | Reason=bound_horizontal_internal_subdivision | SystemRanges={:?} | ProjectionOnly=true | Authority=none",
                        source.input_label, round, ranges,
                    );
                    no_goods.extend(ranges);
                }
                if no_goods.len() == before && page_rejections.partitions.len() == page_before {
                    anyhow::bail!(
                        "RendererFeasibilityCommittedSourceOracle: Renderer-Rejection erzeugt kein neues run-lokales No-Good"
                    );
                }

                let candidate = build_renderer_feasibility_frontier_candidate(
                    &RendererFeasibilityFrontierRequest {
                        measure_demands: &renderer_feasibility_basis.selected_measure_demand_evidence,
                        measure_clearance_edges:
                            &renderer_feasibility_basis.selected_measure_clearance_edge_evidence,
                        feasible_page_counts: &renderer_feasibility_basis.feasible_page_counts,
                        preferred_pages: renderer_feasibility_basis.preferred_pages,
                        accepted_system_capacity: renderer_feasibility_basis.accepted_system_capacity,
                        max_system_measures: renderer_feasibility_basis.max_allowed_system_measures,
                        vertical_max_systems_per_page: renderer_feasibility_basis.vertical_max_systems_per_page,
                        reference_systems: renderer_feasibility_basis.selected_systems,
                        no_goods: &no_goods,
                        rejected_partitions: &page_rejections.partitions,
                    },
                )?;
                candidate_plan = promote_renderer_feasibility_constraint_candidate(
                    &initial_plan,
                    &candidate,
                    &renderer_feasibility_basis,
                    &no_goods,
                )?;
                page_rejections.validate(&candidate_plan)?;
                eprintln!(
                    "ETUSCHLOPF: RendererFeasibilityRound: Project={:?} | Round={} | Phase=next_plan_ready | NextPlanPageStartsAndSystemLengths={:?} | SystemNoGoods={:?} | RejectedPartitions={:?} | ProjectionOnly=true | Authority=none",
                    source.input_label, round, authoritative_page_shape(&candidate_plan).map(|shape| (shape.page_starts, shape.page_system_lengths)),
                    no_goods, page_rejections.partitions,
                );
            }
            anyhow::bail!(
                "RendererFeasibilityCommittedSourceOracle: keine Renderer-bestaetigte Joint-Plan-Konvergenz innerhalb von {} Runden",
                MAX_ORACLE_ROUNDS
            )
        })();
        match oracle_result {
            Ok(outcome) => outcome,
            Err(error) => RendererFeasibilityOracleOutcome::Rejected {
                reason: error.to_string().replace(['\r', '\n'], " "),
            },
        }
    } else {
        RendererFeasibilityOracleOutcome::NotRequired
    };

    match &renderer_feasibility_oracle_outcome {
        RendererFeasibilityOracleOutcome::NotRequired => console_progress(
            "HorizontalLayoutRendererFeasibilityCommittedSourceOracle: Status NOT_REQUIRED | Reason pdf_not_requested | SourcePhase MetricFinalNotation_after_FinalPdfSourceCommit_before_productive_layout | ProductiveAdoptionEnabled ja | PolicyWrite nein",
        ),
        RendererFeasibilityOracleOutcome::Confirmed {
            rounds,
            no_goods,
            source_fingerprint,
            plan,
        } => {
            let learned_no_goods = if no_goods.is_empty() {
                "none".to_string()
            } else {
                no_goods
                    .iter()
                    .map(|(start, end)| format!("{start}-{end}"))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            console_progress(format!(
                "HorizontalLayoutRendererFeasibilityCommittedSourceOracle: Status RENDERER_CONFIRMED | Rounds {} | LearnedNoGoods {} | SourceFingerprint {:016x} | CandidatePages {} | CandidateSystems {} | SourcePhase MetricFinalNotation_after_FinalPdfSourceCommit_before_productive_layout | RendererAuthority feasibility_veto_only | RunLocalNoGoods ja | ProductiveAdoptionEnabled ja | PolicyWrite ja",
                rounds,
                learned_no_goods,
                source_fingerprint,
                if plan.slots.is_empty() {
                    0
                } else {
                    1 + plan
                        .slots
                        .iter()
                        .skip(1)
                        .filter(|slot| slot.directives.starts_new_page)
                        .count()
                },
                if plan.slots.is_empty() {
                    0
                } else {
                    1 + plan
                        .slots
                        .iter()
                        .skip(1)
                        .filter(|slot| {
                            slot.directives.starts_new_system
                                || slot.directives.starts_new_page
                        })
                        .count()
                },
            ));
        }
        RendererFeasibilityOracleOutcome::Rejected { reason } => console_progress(format!(
            "HorizontalLayoutRendererFeasibilityCommittedSourceOracle: Status UNAVAILABLE | Reason {} | SourcePhase MetricFinalNotation_after_FinalPdfSourceCommit_before_productive_layout | RendererAuthority feasibility_veto_only | RunLocalNoGoods ja | ProductiveAdoptionEnabled ja | PolicyWrite nein",
            reason,
        )),
    }

    let committed_pdf_source_and_renderer_oracle =
        CommittedPdfSourceAndRendererOracleOutcome {
            committed_master_export,
            render_xml_selected_for_pdf,
            pdf_source_label,
            pdf_render_requested,
            committed_source_xml,
            committed_initial_horizontal_layout_plan,
            renderer_feasibility_basis,
            final_pdf_source_commit,
            final_pdf_chord_structural_plan: render_final_chord_structural_plan.clone(),
            final_pdf_pruned_omissions: render_pruned_final_chord_omissions.clone(),
            renderer_feasibility_oracle_outcome,
            render_write_pdf_source_adoption_truth_gate_audit,
            render_write_pdf_source_adoption_truth_gate_blocks_release,
            authorized_writer_entry_boundary_audit,
        };

    let release_evidence = MetricFinalNotationReleaseEvidence {
        repair_loop_invocation_evidence,
        external_feedback_closure_render_write_audit,
        key_signature_enharmonic_spelling_render_write_audit,
        lower_clef_surface_materialization_audit,
        notation_voice_pre_serialization_constraint_audit: initial_render
            .frozen_sounding_temporal_lane_plan
            .notation_voice_pre_serialization_constraint_audit
            .clone(),
        notation_voice_causal_lane_pressure_audit,
    };

    Ok(MetricBeamAndFinalNotationStageOutcome {
        release_evidence,
        committed_pdf_source_and_renderer_oracle,
    })
}

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/metric_sovereign_caller_tests.rs"));
