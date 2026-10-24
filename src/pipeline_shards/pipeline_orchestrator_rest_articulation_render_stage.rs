#[derive(Debug)]
struct RestAndArticulationRenderStageOutcome {
    fragmented_rest_chain_transactional_write_trial: RenderXmlPianisticRestRepairTrial,
    piano_final_rest_glyph_canonical_height_write_audit:
        PianoFinalRestGlyphCanonicalHeightWriteAudit,
    single_clear_staccato_render_write_audit: SingleClearStaccatoRenderWriteAudit,
    wav_truth_binding_clear_tiny_accent_render_write_audit:
        AssertiveVisibleArticulationRenderWriteAudit,
    external_notation_feedback_beam_variant_a_tiny_render_write_audit:
        SisterFeedbackBeamVariantATinyRenderWriteAudit,
    repair_loop_invocation_evidence: RestArticulationRepairLoopInvocationEvidence,
}

struct RestAndArticulationRenderStageInputs<'a> {
    render_state: &'a mut structural_write_authority::RenderSovereignState,
    final_render_source_available: bool,
    reference: &'a reference::ReferenceReport,
    render_xml_trial_path: &'a Path,
    pre_preview_render_xml_for_gap: &'a str,
    render_final_chord_structural_plan:
        &'a mut structural_write_authority::CanonicalFinalChordStructuralPlan,
    frozen_serialized_voice_plan: &'a structural_write_authority::FrozenSerializedVoicePlan,
    render_pruned_final_chord_omissions: &'a std::collections::BTreeSet<
        structural_write_authority::RenderPrunedFinalChordOmissionKey,
    >,
}

fn run_rest_and_articulation_render_stage(
    inputs: RestAndArticulationRenderStageInputs<'_>,
) -> anyhow::Result<RestAndArticulationRenderStageOutcome> {
    let RestAndArticulationRenderStageInputs {
        render_state,
        final_render_source_available,
        reference,
        render_xml_trial_path,
        pre_preview_render_xml_for_gap: _pre_preview_render_xml_for_gap,
        render_final_chord_structural_plan,
        frozen_serialized_voice_plan,
        render_pruned_final_chord_omissions,
    } = inputs;
    use crate::master_state_identity::{RenderDiscardReason, RenderTransitionReceipts};
    crate::master_state_identity::record_render_stage_boundary(crate::master_state_identity::RenderStageBoundary::RestArticulation);
    let mut repair_loop_invocation_evidence = RestArticulationRepairLoopInvocationEvidence {
        fragmented_rest_invoked: final_render_source_available,
        staccato_primary_invoked: final_render_source_available,
        ..RestArticulationRepairLoopInvocationEvidence::default()
    };
    let fragmented_rest_chain_transactional_write_trial = if final_render_source_available {
        let root = render_state.require_state_root("RestArticulation current State");
        let (mut trial, post, receipts) = build_fragmented_rest_chain_transactional_write_trial(
            root.document(),
            "Render-XML nach WholeRestDisplayCoordinateReadbackTruthFix vor FragmentedRestChainTransactionalWrite",
            render_xml_trial_path,
        )?;
        if let Some(post) = post {
            render_state.adopt_transition(root, &post, receipts,
                structural_write_authority::RenderSinkKind::FragmentedRestChainTrial,
                |path, pre, verified| (structural_write_authority::write_fragmented_rest_chain_trial_authorized(pre, verified, path), None))?;
            trial.written = true;
            trial.done_fragment = trial.done_fragment.replace("RenderXmlWrite nein", "RenderXmlWrite ja");
            trial.pipeline_line = trial.pipeline_line.replace("written nein", "written ja");
            let lane_root = render_state.require_state_root("RestArticulation current State");
            let ((lane_post, lane_stats), lane_receipt) = human_bound_cursor_lane_owner_transfer_xml_with_receipt(lane_root.document());
            let mut lane_chain = RenderTransitionReceipts::new();
            lane_chain.push_preserving(lane_receipt, &lane_post);
            if lane_stats.written && lane_post != lane_root.document() {
                render_state.adopt_transition(lane_root, &lane_post, lane_chain,
                    structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::CursorLaneRepair),
                    |path, _pre, verified| structural_write_authority::write_cursor_lane_repair_checkpoint_authorized(path, verified))?;
            } else { lane_chain.discard_unadopted(RenderDiscardReason::LosingCandidate); }
        } else { receipts.discard_unadopted(RenderDiscardReason::LosingCandidate); }
        trial
    } else {
        RenderXmlPianisticRestRepairTrial {
            source_label: "keine finale Render-XML-Quelle".to_string(),
            verdict: "fragmented_rest_chain_transactional_write_skipped_no_render_source".to_string(),
            done_fragment: "FragmentedRestChainTransactionalWrite: uebersprungen; keine Render-XML-Quelle".to_string(),
            pipeline_line: format!("FragmentedRestChainTransactionalWrite v{}: uebersprungen; keine finale Render-XML-Quelle; kein Write, keine Musikmutation.", env!("CARGO_PKG_VERSION")),
            ..RenderXmlPianisticRestRepairTrial::default()
        }
    };
    repair_loop_invocation_evidence.fragmented_rest_adopted =
        fragmented_rest_chain_transactional_write_trial.consolidation_adopted;
    repair_loop_invocation_evidence.fragmented_rest_written =
        fragmented_rest_chain_transactional_write_trial.written;
    let piano_final_rest_glyph_canonical_height_write_audit = if final_render_source_available {
        let piano_final_rest_source_xml = render_state.require_document("RestArticulation source").to_owned();
        let root = render_state.require_state_root("RestArticulation current State");
        let (piano_final_rest_after_xml, audit, receipts) = build_piano_final_rest_glyph_canonical_height_write(
            &piano_final_rest_source_xml,
            "finale Render-MusicXML nach FragmentedRestChainTransactionalWrite; Klavierfassung eine sichtbare Pausenhoehe je Glyph",
            render_xml_trial_path,
        )?;
        if audit.written {
            render_state.adopt_transition(root, &piano_final_rest_after_xml, receipts,
                structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::FinalPianoRestGlyph),
                |path, _pre, verified| structural_write_authority::write_final_piano_rest_glyph_checkpoint_authorized(path, verified))?;
        } else { receipts.discard_unadopted(RenderDiscardReason::LosingCandidate); }
        audit
    } else {
        PianoFinalRestGlyphCanonicalHeightWriteAudit {
            source_label: "keine finale Render-XML-Quelle".to_string(),
            output_path: render_xml_trial_path.display().to_string(),
            verdict: "piano_final_rest_glyph_canonical_height_skipped_no_render_source".to_string(),
            done_fragment: "PianoFinalRestGlyphCanonicalHeightRenderWrite: uebersprungen; keine Render-XML-Quelle".to_string(),
            pipeline_line: format!("PianoFinalRestGlyphCanonicalHeightRenderWrite v{}: uebersprungen; keine finale Render-XML-Quelle; RenderXmlWrite nein; MusicMutation nein.", env!("CARGO_PKG_VERSION")),
            ..PianoFinalRestGlyphCanonicalHeightWriteAudit::default()
        }
    };
    let single_clear_staccato_render_write_audit = if final_render_source_available {
        let single_staccato_source_xml = render_state.require_document("RestArticulation source").to_owned();
        let (single_staccato_after_xml, mut audit, occurrence_receipt) =
            apply_single_clear_staccato_render_write_with_placement_readback(
                &single_staccato_source_xml,
                "Render-XML nach Rest-/Pausen-Notation-Writes vor SingleClearStaccatoRenderWrite",
                render_xml_trial_path,
            )?;
        repair_loop_invocation_evidence.staccato_primary_candidates = audit.clear_candidate_count;
        repair_loop_invocation_evidence.staccato_primary_written = audit.written;
        let final_artifact_xml = single_staccato_after_xml;
        let staccato_primary_persisted = audit.target_scope > 0
            && single_clear_staccato_target_persisted_in_final_xml(&final_artifact_xml, &audit);
        repair_loop_invocation_evidence.staccato_primary_persisted = staccato_primary_persisted;
        let staccato_primary_write_committed = audit.written && audit.render_xml_write;
        repair_loop_invocation_evidence.staccato_primary_write_committed =
            staccato_primary_write_committed;
        repair_loop_invocation_evidence.staccato_primary_write_persisted =
            staccato_primary_write_committed && staccato_primary_persisted;
        if staccato_primary_write_committed && !staccato_primary_persisted {
            anyhow::bail!(
                "SingleClearStaccatoPrimaryBoundaryPostcondition: committed primary write is not present in immediate exact-readback artifact: {}",
                render_xml_trial_path.display()
            );
        }
        match occurrence_receipt.as_ref() {
            Some(receipt) if staccato_primary_write_committed && staccato_primary_persisted => {
                let advanced_render_final_chord_structural_plan =
                    structural_write_authority::advance_final_chord_render_occurrence_through_staccato_persistence(
                        &*render_final_chord_structural_plan,
                        frozen_serialized_voice_plan,
                        render_pruned_final_chord_omissions,
                        receipt,
                        &final_artifact_xml,
                    )
                    .with_context(|| {
                        "SingleClearStaccatoPrimaryBoundary konnte die bekannte FinalChord-Identitaet nicht kausal in die retimete Render-Repraesentation fortsetzen"
                    })?;
                *render_final_chord_structural_plan =
                    advanced_render_final_chord_structural_plan;
            }
            Some(_) => {
                anyhow::bail!(
                    "SingleClearStaccatoPrimaryBoundary erzeugte Occurrence-Provenienz ohne committed persisted writer state"
                );
            }
            None if staccato_primary_write_committed => {
                anyhow::bail!(
                    "SingleClearStaccatoPrimaryBoundary committed einen Write ohne Occurrence-Provenienz"
                );
            }
            None => {}
        }
        refresh_single_clear_staccato_audit_from_primary_boundary_artifact(
            &mut audit,
            &final_artifact_xml,
            "unmittelbarer Primary-Boundary-Readback nach SingleClearStaccatoRenderWrite",
        );
        audit
    } else {
        SingleClearStaccatoRenderWriteAudit {
            output_path: render_xml_trial_path.display().to_string(),
            verdict: "single_clear_staccato_skipped_no_render_source".to_string(),
            done_fragment: "SingleClearStaccatoRenderWriteWithPlacementReadback: uebersprungen; keine Render-XML-Quelle; RenderXmlWrite nein; MusicMutation nein".to_string(),
            pipeline_line: format!("SingleClearStaccatoRenderWriteWithPlacementReadback v{}: uebersprungen; keine finale Render-XML-Quelle; RenderXmlWrite nein; MusicMutation nein.", env!("CARGO_PKG_VERSION")),
            ..SingleClearStaccatoRenderWriteAudit::default()
        }
    };
    let _wav_truth_articulation_gate_no_mutation = if final_render_source_available {
        let wav_truth_articulation_xml = render_state.require_document("RestArticulation source").to_owned();
        build_wav_truth_articulation_gate_no_mutation(
            reference,
            &wav_truth_articulation_xml,
            "finale Render-MusicXML vor WavTruthArticulationFeatureLedgerMaterializedNoMutation",
        )
    } else {
        WavTruthArticulationGateAudit {
            done_fragment: "WavTruthArticulationFeatureLedgerMaterializedNoMutation: uebersprungen; keine finale Render-XML-Quelle; WriteReady 0; RenderXmlWrite nein; MusicMutation nein".to_string(),
            pipeline_line: format!("WavTruthArticulationFeatureLedgerMaterializedNoMutation v{}: uebersprungen; keine finale Render-XML-Quelle; WriteReady 0; RenderXmlWrite nein; MusicMutation nein.", env!("CARGO_PKG_VERSION")),
            verdict: "wav_truth_articulation_skipped_no_render_source".to_string(),
            ..WavTruthArticulationGateAudit::default()
        }
    };
    let wav_truth_binding_clear_tiny_accent_render_write_audit = AssertiveVisibleArticulationRenderWriteAudit::default();
    let external_notation_feedback_beam_variant_a_tiny_render_write_audit =
        if final_render_source_available {
            let beam_variant_a_source_xml = render_state.require_document("RestArticulation source").to_owned();
            let root = render_state.require_state_root("RestArticulation current State");
            let mut voice_guard = render_final_chord_structural_plan.bind_bridge_voice_guard(
                &beam_variant_a_source_xml, frozen_serialized_voice_plan, render_pruned_final_chord_omissions,
            ).context("Bridge voice-plan binding to current Render input")?;
            voice_guard.bind_rest_bridge_main_state(render_state);
            let ((beam_variant_a_after_xml, audit), receipt, extension) = apply_structural_rest_bridge_beam_variant_a_render_write(
            &beam_variant_a_source_xml,
            "finale Render-MusicXML nach Artikulations-/Staccato-Readback vor ExternalNotationFeedbackBeamVariantA",
            render_xml_trial_path,
            &voice_guard,
        );
            if audit.written {
                let mut receipts = RenderTransitionReceipts::new();
                receipts.push_preserving(receipt, &beam_variant_a_after_xml);
                render_state.adopt_transition(root, &beam_variant_a_after_xml, receipts,
                    structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::SisterFeedbackBeam),
                    |path, _pre, verified| structural_write_authority::write_sister_feedback_beam_checkpoint_authorized(path, verified))?;
                render_final_chord_structural_plan.adopt_rest_bridge_extension_output(extension, render_state)?;
            } else {
                let mut receipts = RenderTransitionReceipts::new();
                receipts.push_preserving(receipt, &beam_variant_a_after_xml);
                receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
            }
            audit
        } else {
            SisterFeedbackBeamVariantATinyRenderWriteAudit {
            source_label: "keine finale Render-XML-Quelle".to_string(),
            output_path: render_xml_trial_path.display().to_string(),
            attempted: false,
            variant_b_blocked: true,
            verdict: "external_notation_feedback_beam_variant_a_skipped_no_render_source".to_string(),
            done_fragment: "ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback: uebersprungen; keine finale Render-XML-Quelle; RenderXmlWrite nein; MusicMutation nein".to_string(),
            pipeline_line: format!("ExternalNotationFeedbackBeamVariantATinyRenderWriteWithPdfReadback v{}: uebersprungen; keine finale Render-XML-Quelle; RenderXmlWrite nein; MusicMutation nein.", env!("CARGO_PKG_VERSION")),
            ..SisterFeedbackBeamVariantATinyRenderWriteAudit::default()
        }
        };

    Ok(RestAndArticulationRenderStageOutcome {
        fragmented_rest_chain_transactional_write_trial,
        piano_final_rest_glyph_canonical_height_write_audit,
        single_clear_staccato_render_write_audit,
        wav_truth_binding_clear_tiny_accent_render_write_audit,
        external_notation_feedback_beam_variant_a_tiny_render_write_audit,
        repair_loop_invocation_evidence,
    })
}



