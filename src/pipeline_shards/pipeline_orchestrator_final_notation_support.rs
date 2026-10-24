#[derive(Debug, Clone, Copy)]
struct AuthorizedMaterializationEquivalenceProjection {
    green: bool,
}

impl AuthorizedMaterializationEquivalenceProjection {
    const fn green(&self) -> bool {
        self.green
    }
}

#[derive(Debug)]
struct FinalRenderNotationAuthorityStageOutcome {
    final_render_source_available: bool,
    lower_clef_surface_materialization_audit: LowerClefSurfaceMaterializationAudit,
    authorized_writer_candidate_xml: String,
    authorized_materialization_equivalence_audit:
        AuthorizedMaterializationEquivalenceProjection,
    authorized_writer_entry_boundary_audit:
        structural_write_authority::AuthorizedWriterEntryBoundaryAudit,
    final_pdf_source_occurrence_continuation:
        Option<structural_write_authority::FinalPdfSourceOccurrenceContinuation>,
    renderer_cross_staff_beam_capability_audit:
        structural_write_authority::RendererCrossStaffBeamCapabilityAudit,
}

struct FinalRenderNotationAuthorityStageInputs<'a> {
    render_state: &'a mut structural_write_authority::RenderSovereignState,
    final_render_source_available: bool,
    render_xml_trial_path: &'a Path,
    final_export_xml: &'a str,
    frozen_sounding_temporal_lane_plan: &'a FrozenSoundingTemporalLanePlan,
    render_final_chord_structural_plan:
        &'a mut structural_write_authority::CanonicalFinalChordStructuralPlan,
    frozen_sounding_material_attacks: &'a [CandidateMaterialAttackState],
    render_pruned_final_chord_omissions: &'a std::collections::BTreeSet<
        crate::structural_write_authority::RenderPrunedFinalChordOmissionKey,
    >,
}

fn run_final_render_notation_authority_stage(
    inputs: FinalRenderNotationAuthorityStageInputs<'_>,
) -> anyhow::Result<FinalRenderNotationAuthorityStageOutcome> {
    let FinalRenderNotationAuthorityStageInputs {
        render_state,
        mut final_render_source_available,
        render_xml_trial_path,
        final_export_xml: _final_export_xml,
        frozen_sounding_temporal_lane_plan,
        render_final_chord_structural_plan,
        frozen_sounding_material_attacks,
        render_pruned_final_chord_omissions,
    } = inputs;
    let _frozen_sounding_temporal_materialization_transport = (
        frozen_sounding_temporal_lane_plan,
        frozen_sounding_material_attacks,
    );
    crate::master_state_identity::record_render_stage_boundary(crate::master_state_identity::RenderStageBoundary::FinalNotation);

    // Der echte Snapshot unmittelbar vor der Lower-Clef-Freigabe wird genau einmal
    // scoreweit beobachtet. RustigClefSurfaceDecision klassifiziert die qualifizierten
    // typisierten Surface-Plaene; Kandidaten-XML, Quellbindung, Klang- und NoteCount-
    // Beweis sowie finaler Sink gehoeren ausschliesslich der Materialisierungs-Authority.
    let lower_clef_root = render_state.final_notation_branch_root(structural_write_authority::RenderLateBranch::LowerClefSurface, final_render_source_available);
    let lower_clef_pair_prewrite_xml = lower_clef_root.document().to_string();
    let rustig_clef_surface_decision =
        derive_lower_clef_rustig_surface_decision(&lower_clef_pair_prewrite_xml);
    let lower_clef_surface_materialization_audit = materialize_lower_clef_surface_decision(
        rustig_clef_surface_decision,
        &lower_clef_pair_prewrite_xml,
        render_xml_trial_path,
        Some((&mut *render_state, lower_clef_root)),
    )?;
    render_final_chord_structural_plan.continue_rest_bridge_extensions_at_state(
        &lower_clef_pair_prewrite_xml, render_state.document())?;
    // v0.10.1818: the retired late PartialBeatPostRest recovery/audit graph has
    // no pre-commit execution here. The persisted committed-state detector remains
    // the sole fail-closed regression authority for that historical domain.
    // LowerDualEdge bleibt hier reine historische Regressionsevidenz. Die lebende
    let mut final_chord_render_occurrence_continuation: Option<
        structural_write_authority::CanonicalFinalChordRenderOccurrenceContinuation,
    > = None;
    // v0.10.1718: Spaete Render-Notation-Writes duerfen eine zuvor beobachtete
    // Legacy-Clef-Oberflaeche transportieren, besitzen aber keine Clef-Authority.
    // Deshalb wird derselbe bestehende FinalNotation-Checkpoint genau an der
    // letzten Render-Notationsgrenze vor Structural Authority/PDF-Bindung erneut
    // auf den realen Snapshot angewandt. Planner und Policy bleiben writer-frei;
    // es entsteht weder ein neuer Writer noch ein Repair-Pass.
    if final_render_source_available {
        let final_notation_root = render_state.require_state_root("FinalNotation B3");
        let final_clef_course_source_xml = final_notation_root.document().to_string();
        let mut final_notation_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        let mut final_chord_projection_input_xml = String::new();
        let preparation = (|| -> anyhow::Result<_> {
        // r347: Der historische FinalRender-MasterMeasure-Fallback ist pensioniert.
        // Dieser Checkpoint darf keinen roten Render-Takt mehr durch die komplette
        // Timeline des Master-Takts ersetzen. Die bestehende identity-freie
        // Crossing-Rest-Authority bleibt als legitime spaete Projektion erhalten;
        // danach beobachten die bereits vorhandenen committed/final RawCursor-Gates
        // den realen Renderstand read-only und blockieren fail-closed, falls er rot ist.
        // Damit existiert genau eine produktive CursorLane-Reparaturgrenze und keine
        // zweite Master-basierte Recovery-Authority.
        let ((crossing_rest_closed_render_xml, _crossing_rest_closed_render_measures), crossing_receipt) =
            close_beat_scoped_rest_completion_crossing_rests_in_xml(
                &final_clef_course_source_xml,
            );
        final_notation_receipts.push_block(crossing_receipt, &crossing_rest_closed_render_xml);
        render_final_chord_structural_plan.continue_rest_bridge_extensions(
            &final_clef_course_source_xml, &crossing_rest_closed_render_xml)?;
        let final_chord_structural_plan = &*render_final_chord_structural_plan;
        let frozen_serialized_voice_plan =
            &frozen_sounding_temporal_lane_plan.frozen_serialized_voice_plan;
        let output_alpha =
            structural_write_authority::audit_musicxml_voice_label_binding_alpha_for_render_projection(
                &crossing_rest_closed_render_xml,
                final_chord_structural_plan,
                frozen_serialized_voice_plan,
                render_pruned_final_chord_omissions,
            )
            .with_context(|| {
                "FrozenSerializedVoiceChordConstraintCutover konnte Output-Label-Alpha vor Render-Materialisierung nicht beweisen"
            })?;
        if !output_alpha.green() {
            anyhow::bail!(
                "FrozenSerializedVoiceChordConstraintCutover Output-Alpha blockiert Render-Materialisierung: cases={} semantic_role_delta={} paired_binding_failures={} stale_binding_false_accepts={}",
                output_alpha.cases(),
                output_alpha.semantic_role_deltas(),
                output_alpha.paired_binding_failures(),
                output_alpha.stale_binding_false_accepts(),
            );
        }
        final_chord_projection_input_xml = crossing_rest_closed_render_xml.clone();
        let projected_continuation =
            structural_write_authority::project_canonical_final_chord_render_occurrence_continuation_with_receipts(
                &crossing_rest_closed_render_xml,
                final_chord_structural_plan,
                frozen_serialized_voice_plan,
                render_pruned_final_chord_omissions,
                &mut final_notation_receipts,
            )
            .with_context(|| {
                "FinalChordEncodingPlan konnte den finalen Render-Kandidaten nicht mit transaction-lokaler Occurrence-Continuation materialisieren"
            })?;
        // r1144: close the accepted final-visible-rest-role / final-voice-binding
        // ordering class without widening writer reachability to a noncanonical
        // pipeline writer. Selection is pure; materialization remains exclusively
        // inside the existing StructuralWriteAuthority RestVisibility capability.
        let bound_shadow_before =
            final_visible_shadow_bookkeeping_regression_candidates_with_voice_binding(
                projected_continuation.xml(),
                frozen_serialized_voice_plan,
            )
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "FinalVisibleRestRoleFrozenVoiceBindingClosure konnte den gebundenen Kandidaten vor Materialisierung nicht klassifizieren"
                )
            })?;
        let bound_shadow_replacements =
            final_visible_shadow_bookkeeping_replacements_with_voice_binding(
                projected_continuation.xml(),
                frozen_serialized_voice_plan,
            )
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "FinalVisibleRestRoleFrozenVoiceBindingClosure konnte keinen relocatable Replacement-Plan ableiten"
                )
            })?;
        if bound_shadow_replacements.len() != bound_shadow_before {
            anyhow::bail!(
                "FinalVisibleRestRoleFrozenVoiceBindingClosure lokale Selektionsbilanz divergiert: candidates={} replacements={}",
                bound_shadow_before,
                bound_shadow_replacements.len(),
            );
        }

        let final_notation_candidate_xml = if bound_shadow_replacements.is_empty() {
            projected_continuation.xml().to_string()
        } else {
            let (result, receipt) = crate::structural_write_authority::apply_final_visible_rest_role_rewrite_authorized(
                projected_continuation.xml(),
                bound_shadow_replacements,
            );
            final_notation_receipts.push_exact(receipt);
            result.ok_or_else(|| {
                anyhow::anyhow!(
                    "FinalVisibleRestRoleFrozenVoiceBindingClosure StructuralWriteAuthority hat den gebundenen Rest-Delta nicht autorisiert"
                )
            })?
        };

        let bound_shadow_after =
            final_visible_shadow_bookkeeping_regression_candidates_with_voice_binding(
                &final_notation_candidate_xml,
                frozen_serialized_voice_plan,
            )
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "FinalVisibleRestRoleFrozenVoiceBindingClosure konnte den gebundenen Kandidaten nach Materialisierung nicht klassifizieren"
                )
            })?;
        let (bound_before_events, bound_before_backup_forward_ok) =
            collect_render_xml_note_events(projected_continuation.xml());
        let (bound_after_events, bound_after_backup_forward_ok) =
            collect_render_xml_note_events(&final_notation_candidate_xml);
        let bound_before_signature = render_visible_sounding_signature(&bound_before_events);
        let bound_after_signature = render_visible_sounding_signature(&bound_after_events);
        let bound_visible_pitched_before = bound_before_events
            .iter()
            .filter(|event| !event.is_hidden && event.midi.is_some())
            .count();
        let bound_visible_pitched_after = bound_after_events
            .iter()
            .filter(|event| !event.is_hidden && event.midi.is_some())
            .count();
        let bound_hidden_pitched_before = bound_before_events
            .iter()
            .filter(|event| event.is_hidden && event.midi.is_some())
            .count();
        let bound_hidden_pitched_after = bound_after_events
            .iter()
            .filter(|event| event.is_hidden && event.midi.is_some())
            .count();
        let bound_visible_rests_before = bound_before_events
            .iter()
            .filter(|event| !event.is_hidden && event.is_rest)
            .count();
        let bound_visible_rests_after = bound_after_events
            .iter()
            .filter(|event| !event.is_hidden && event.is_rest)
            .count();
        let bound_raw_cursor_after_ok = scan_raw_cursor_xml(
            "FinalVisibleRestRoleFrozenVoiceBindingClosure",
            &final_notation_candidate_xml,
        )
        .into_iter()
        .all(|row| {
            !row.hard_violation
                && row.partial_backup_count == 0
                && row.post_expected_events == 0
        });
        let bound_invariants_green = parse_musicxml(&final_notation_candidate_xml).is_ok()
            && bound_before_backup_forward_ok
            && bound_after_backup_forward_ok
            && bound_raw_cursor_after_ok
            && bound_before_signature == bound_after_signature
            && bound_visible_pitched_before == bound_visible_pitched_after
            && bound_hidden_pitched_before == bound_hidden_pitched_after
            && bound_visible_rests_after <= bound_visible_rests_before
            && bound_shadow_after == 0;
        if !bound_invariants_green {
            anyhow::bail!(
                "FinalVisibleRestRoleFrozenVoiceBindingClosure Invarianten blockieren FinalNotation: shadow {}->{}, visible_pitched {}->{}, hidden_pitched {}->{}, visible_rests {}->{}, parse={}, backup_forward={}/{}, raw_cursor={}, sounding_stable={}",
                bound_shadow_before,
                bound_shadow_after,
                bound_visible_pitched_before,
                bound_visible_pitched_after,
                bound_hidden_pitched_before,
                bound_hidden_pitched_after,
                bound_visible_rests_before,
                bound_visible_rests_after,
                yes_no(parse_musicxml(&final_notation_candidate_xml).is_ok()),
                yes_no(bound_before_backup_forward_ok),
                yes_no(bound_after_backup_forward_ok),
                yes_no(bound_raw_cursor_after_ok),
                yes_no(bound_before_signature == bound_after_signature),
            );
        }
        console_progress(format!(
            "FinalVisibleRestRoleFrozenVoiceBindingClosure: candidates {}→{}; StructuralWriteAuthority {}; pitched stable; cursor green",
            bound_shadow_before,
            bound_shadow_after,
            if bound_shadow_before > 0 { "materialized" } else { "not_required" },
        ));

        Ok((projected_continuation, final_notation_candidate_xml))
        })();
        let (projected_continuation, final_notation_candidate_xml) = match preparation {
            Ok(value) => value,
            Err(error) => { final_notation_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate); crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures); return Err(error); }
        };
        render_state.adopt_final_notation_checkpoint(final_notation_root, &final_notation_candidate_xml, final_notation_receipts)?;
        let final_notation_checkpoint_xml = render_state.document().expect("committed FinalNotation state").to_string();
        crate::audit_only::voice_run_collision_origin::observe_final_chord_adoption(
            "FinalNotationReceiptChainCommitted",
            &final_chord_projection_input_xml,
            projected_continuation.xml(),
            &final_notation_checkpoint_xml,
        );
        let checkpoint_continuation =
            structural_write_authority::advance_canonical_final_chord_render_occurrence_after_final_notation_checkpoint(
                projected_continuation,
                &final_notation_checkpoint_xml,
                render_final_chord_structural_plan,
                &frozen_sounding_temporal_lane_plan.frozen_serialized_voice_plan,
                render_pruned_final_chord_omissions,
            )
            .with_context(|| {
                "FinalNotation-Checkpoint konnte die FinalChord-Occurrence-Continuation nicht kausal ueber die ClefReadability-Materialisierung transportieren"
            })?;
        final_chord_render_occurrence_continuation = Some(checkpoint_continuation);
    }
    // v0.10.1147 Phase 1: derselbe echte Vor-Operator-Snapshot wird zusaetzlich
    // read-only in identitaetsfreie StructuralViews projiziert. Der Adapter
    // erhaelt XML-Identitaeten nur hinter dem opaken Locator; der Detector sieht
    // ausschliesslich relative Metrik, Rollen, Staff-Relationen und Balkenzustaende.
    // Noch kein Writer akzeptiert oder verwendet das neue Autorisierungstoken.
    let cursor_root = render_state.final_notation_branch_root(structural_write_authority::RenderLateBranch::CursorNeutralWriter, final_render_source_available);
    let structural_authority_source_xml = cursor_root.document().to_string();
    let historical_checkpoint = render_state.last_checkpoint_58_basis().map(str::to_owned);
    let (structural_snapshot_views, _) =
        structural_write_authority::adapt_structural_snapshot_read_only(
            &structural_authority_source_xml,
            1,
            "echter Vor-Operator-Snapshot nach Rhythmus-Regressionen",
        );
    // v0.10.1151 Phase 2B: Der identitaetsfreie RepairIntent wird audit-only
    // auf semantische Ereignisgleichheit und explizit bewiesene XML-Encoding-
    // Normalisierung geprueft. Der Planner trifft keine neue musikalische Auswahl
    // und erzeugt weiterhin weder Token noch Write.
    let structural_repair_intents =
        structural_write_authority::detect_structural_repair_intents_read_only(
            &structural_snapshot_views,
        );
    // v0.10.1153 Phase 4: Der musikalische Detector liegt nun in einem eigenen
    // Rust-Modul. Seine Import- und Typgrenze wird audit-only gegen Raw-XML,
    // Locator und absolute Quellidentitäten geprüft. Dieser Grenzaudit erzeugt selbst kein Token.
    let identity_free_detector_module_boundary_audit:
        structural_write_authority::IdentityFreeDetectorModuleBoundaryAudit =
        structural_write_authority::audit_identity_free_detector_module_boundary(
            structural_snapshot_views.len(),
            structural_repair_intents.len(),
            "echter Vor-Operator-Snapshot mit echter Detector-Modulgrenze",
        );
    let identity_free_semantic_event_equivalence_audit:
        structural_write_authority::IdentityFreeSemanticEventEquivalenceAudit =
        structural_write_authority::audit_identity_free_semantic_event_equivalence(
            &structural_repair_intents,
            structural_snapshot_views.len(),
            "echter Vor-Operator-Snapshot nach identitaetsfreier Strukturdetektion",
        );
    // v0.10.1152 Phase 3: Das reale Vor-Writer-XML wird audit-only durch fünf
    // positive Identitäts-/Reihenfolgemetamorphosen und sechs negative, musikalisch
    // ähnliche Strukturvarianten geführt. Vollständiges GenericityEvidence wird
    // berechnet. Der Harness selbst autorisiert nicht; erst der nachfolgende unabhängige Verifier darf einmalig ein Token erzeugen.
    let identity_free_metamorphic_safety_net_audit:
        structural_write_authority::IdentityFreeMetamorphicSafetyNetAudit =
        structural_write_authority::audit_identity_free_metamorphic_safety_net(
            &structural_authority_source_xml,
            1,
            "echter Vor-Operator-Snapshot mit positiven und negativen Metamorphosen",
        );
    // v0.10.1160 renderer-contiguity truth hold: Die parent-owned Authority erzeugt aus der
    // vollständigen Evidenz genau ein privates, nicht klonbares Token, bindet
    // den bewiesenen Serialisierungsplan hinein und verschiebt es genau einmal
    // in den Writer. Der Writer erzeugt einen cursorneutralen Renderkandidaten.
    // Ein unabhängiger Post-Write-Verifier prüft anschließend das tatsächlich
    // erzeugte semantische und klingende Ereignis-Multiset, bevor exakt diese
    // Kandidatendatei als PDF-Quelle freigegeben werden darf.
    let prepared = structural_write_authority::execute_authorized_cursor_neutral_writer_candidate(&structural_authority_source_xml,
&structural_repair_intents,
&identity_free_detector_module_boundary_audit,
&identity_free_semantic_event_equivalence_audit,
&identity_free_metamorphic_safety_net_audit,
crate::structural_write_authority::CursorNeutralExecutionStages { verifier_source_stage: "echter Vor-Operator-Snapshot mit unabhaengiger Token-Autorisierung und Handoff", writer_source_stage: "echter Vor-Operator-Snapshot mit tokenpflichtigem cursorneutralem Writer Phase 6C", historical_checkpoint_58: historical_checkpoint.as_deref() })?;
    let _receipt_iteration_counts = (prepared.accepted_iterations, prepared.accepted_direction_iterations);
    let _token_issuance_audit = &prepared.verifier_audit;
    let structural_write_authority::PreparedCursorNeutralWriterCandidate {
        candidate_xml: authorized_writer_candidate_xml, writer_audit: authorized_writer_entry_boundary_audit,
        receipts: cursor_receipts, basis: cursor_basis, verified_direction, ..
    } = prepared;
    let authorized_materialization_equivalence_audit =
        structural_write_authority::audit_authorized_materialization_equivalence(
            &structural_authority_source_xml,
            &authorized_writer_candidate_xml,
            "tatsaechlich geschriebener AuthorizedCursorNeutralWriter-Kandidat gegen gebundenen Vor-Writer-Snapshot",
            historical_checkpoint.as_deref(), verified_direction.as_ref(),
        );
    let continuation_result = (|| -> anyhow::Result<_> {
        Ok(match final_chord_render_occurrence_continuation {
            Some(projection) => Some(
                structural_write_authority::advance_final_pdf_source_occurrence_after_cursor_commit(
                    projection,
                    &structural_authority_source_xml,
                    &authorized_writer_candidate_xml,
                    authorized_writer_entry_boundary_audit.write_produced(),
                    authorized_materialization_equivalence_audit.green(),
                    structural_write_authority::FinalPdfSourceOccurrenceValidationContext {
                        canonical: render_final_chord_structural_plan,
                        voice_plan: &frozen_sounding_temporal_lane_plan
                            .frozen_serialized_voice_plan,
                        render_omissions: Some(render_pruned_final_chord_omissions),
                    },
                )
                .with_context(|| {
                    "AuthorizedCursorNeutralWriter konnte die FinalChord-Occurrence-Continuation nicht kausal in die FinalPdfSource-Epoche transportieren"
                })?,
            ),
            None if authorized_writer_entry_boundary_audit.write_produced() => {
                anyhow::bail!(
                    "AuthorizedCursorNeutralWriter erzeugte einen Kandidaten ohne vorherige FinalChord-Occurrence-Continuation"
                );
            }
            None => None,
        })
    })();
    let final_pdf_source_occurrence_continuation = match continuation_result {
        Ok(value) => value,
        Err(error) => { cursor_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate); crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures); return Err(error); }
    };
    // Historische Renderer-Proben sind abgeschlossen und aus der Laufzeit entfernt.
    // Nur die bereits nach Behind Bars geprüfte Evidenzzahl wird an die aktive
    // Capability-Bewertung übergeben; es werden keine Hilfsartefakte gesucht,
    // gelöscht, erzeugt oder exportiert.
    const REVIEWED_ISOLATED_VISIBLE_BEAM_ROWS: usize = 5;
    const CONTEXT_EQUIVALENT_POSITIVE_ROWS: usize = 0;
    // XML-Grammatik, isolierte Renderer-Grundfaehigkeit und produktive
    // Kontextgleichheit bleiben drei getrennte Wahrheiten. Der Volltakt-Write
    // bleibt blockiert, bis mindestens eine kontextaequivalente Probe sichtbar
    // denselben Balken materialisiert.
    let renderer_cross_staff_beam_capability_audit:
        structural_write_authority::RendererCrossStaffBeamCapabilityAudit =
        structural_write_authority::audit_renderer_cross_staff_beam_capability(
            &authorized_writer_candidate_xml,
            "tatsaechlich geschriebener Direction-Relocation-Kandidat nach positiver Isolationssichtpruefung und vor Zwei-Takt-Systemumbruch-Sichtpruefung",
            REVIEWED_ISOLATED_VISIBLE_BEAM_ROWS,
            CONTEXT_EQUIVALENT_POSITIVE_ROWS,
        );
    if authorized_writer_entry_boundary_audit.write_produced() {
        if !authorized_materialization_equivalence_audit.green() {
            cursor_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
            crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures);
            anyhow::bail!("B4 final materialization proof rejected");
        }
        let root = match cursor_basis {
            structural_write_authority::CursorNeutralPreparedBasis::Source => cursor_root,
            structural_write_authority::CursorNeutralPreparedBasis::HistoricalCheckpoint58 => match render_state.historical_checkpoint_58_root() {
                Some(root) => root,
                None => { cursor_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate); crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures); anyhow::bail!("B4 historical checkpoint stale"); }
            },
        };
        render_state.adopt_transition(root, &authorized_writer_candidate_xml, cursor_receipts,
            structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::DirectionRelocation),
            |path, _pre, post| structural_write_authority::write_direction_relocation_checkpoint_authorized(path, post))?;
        final_render_source_available = true;
    }
    else { cursor_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate); }
    let authorized_materialization_equivalence_audit =
        AuthorizedMaterializationEquivalenceProjection {
            green: authorized_materialization_equivalence_audit.green(),
        };
    Ok(FinalRenderNotationAuthorityStageOutcome {
        final_render_source_available,
        lower_clef_surface_materialization_audit,
        authorized_writer_candidate_xml,
        authorized_materialization_equivalence_audit,
        authorized_writer_entry_boundary_audit,
        final_pdf_source_occurrence_continuation,
        renderer_cross_staff_beam_capability_audit,
    })
}
