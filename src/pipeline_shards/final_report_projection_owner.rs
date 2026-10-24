// Scoped owner for the complete final report projection. This expression is included exactly once
// inside `run_release_report_and_exit_stage`, where all already computed audit evidence remains available without a
// synthetic 260-field transport object. Report mutation is bounded by the shared typed lifecycle:
// DraftReport -> CompactedReport -> FinalizedReport. After compaction, every projection is routed
// through declared section contracts in the byte-preserving ledger; no generic append
// surface and no mutable String escape remain after compaction.
{
fn append_repair_loop_invocation_evidence_body(
    text: &mut String,
    rest_articulation: &RestArticulationRepairLoopInvocationEvidence,
    strict_staff: &StrictStaffOverfullCoverageNormalizationTrial,
) {
    // Stable report-schema tombstones for the retired dedicated ShadowRest owners.
    // These rows preserve the external report contract; they are not runtime evidence carriers.
    text.push_str("ShadowRestPre: invoked 0; candidates 0; written 0.\n");
    text.push_str(&format!(
        "StrictStaffBetweenShadowInvocations: written {}; adopted {}.\n",
        u8::from(strict_staff.written),
        strict_staff.candidate_adopted,
    ));
    text.push_str("ShadowRestPost: invoked 0; candidates 0; written 0.\n");
    text.push_str(&format!(
        "FragmentedRestChain: invoked {}; adopted {}; written {}.\n",
        u8::from(rest_articulation.fragmented_rest_invoked),
        rest_articulation.fragmented_rest_adopted,
        u8::from(rest_articulation.fragmented_rest_written),
    ));
    text.push_str(&format!(
        "StaccatoPrimary: invoked {}; candidates {}; written {}; persisted {}.\n",
        u8::from(rest_articulation.staccato_primary_invoked),
        rest_articulation.staccato_primary_candidates,
        u8::from(rest_articulation.staccato_primary_written),
        u8::from(rest_articulation.staccato_primary_persisted),
    ));
    text.push_str(&format!(
        "StaccatoPrimaryBoundary: write_committed {}; write_persisted {}; postcondition_green {}.\n",
        u8::from(rest_articulation.staccato_primary_write_committed),
        u8::from(rest_articulation.staccato_primary_write_persisted),
        u8::from(!rest_articulation.staccato_primary_write_committed || rest_articulation.staccato_primary_write_persisted),
    ));
    text.push_str("StaccatoPersistenceFallback: retired; productive secondary writer none; immediate primary persistence loss fails closed.\n");
    text.push_str("WriteAllowed: nein. MusicMutation: nein. Purpose: invocation evidence only.\n");
}


    // v0.10.1536: Build only the report state that survives into the public result.
    // Release overrides remain typed decisions and are applied directly to the authoritative
    // result preamble before the current declared projections are attached.
    let strict_staff_candidate_blocks_release = strict_staff_incomplete_blocks_release(
        &visible_lane_completeness_readback_gate_no_mutation,
    );
    let strict_staff_blocks_release =
        strict_staff_candidate_blocks_release && best.final_result.summary.blockers > 0;
    let pre_adoption_release_blocked =
        strict_staff_blocks_release || strict_visible_rest_arithmetic_materializer_blocks;
    let release_blocker_count = usize::from(strict_staff_blocks_release)
        + usize::from(strict_visible_rest_arithmetic_materializer_blocks)
        + usize::from(render_write_pdf_source_adoption_truth_gate_blocks_release);
    let final_release_blocked = release_blocker_count > 0;

    let mut report_preamble = write_text_report(&best.final_result, &report_options);
    if strict_staff_blocks_release {
        report_preamble = apply_strict_staff_incomplete_release_block(
            report_preamble,
            &visible_lane_completeness_readback_gate_no_mutation,
            &strict_staff_local_release_completeness_trial,
        );
    }
    if strict_visible_rest_arithmetic_materializer_blocks {
        report_preamble = apply_strict_visible_rest_arithmetic_materializer_gate_block(
            report_preamble,
            &visible_rest_value_readability_audit_no_mutation,
            &strict_visible_rest_arithmetic_materializer_trial,
        );
    }
    if render_write_pdf_source_adoption_truth_gate_blocks_release {
        report_preamble = apply_render_write_pdf_source_adoption_truth_gate_release_block(
            report_preamble,
            &render_write_pdf_source_adoption_truth_gate_audit,
            pre_adoption_release_blocked,
        );
    }
    let mut console = write_console_summary(&best.final_result, &source.report_out);
    if strict_staff_blocks_release {
        console = apply_strict_staff_incomplete_console_block(console);
    }
    if strict_visible_rest_arithmetic_materializer_blocks {
        console = apply_strict_visible_rest_arithmetic_materializer_gate_console_block(
            console,
            &visible_rest_value_readability_audit_no_mutation,
        );
    }
    if render_write_pdf_source_adoption_truth_gate_blocks_release {
        console = apply_render_write_pdf_source_adoption_truth_gate_console_block(
            console,
            pre_adoption_release_blocked,
        );
    }
    console.push('\n');
    let final_console_done_line = if final_release_blocked {
        console_done_line.replace(
            "Status OK | Blocker 0 |",
            &format!("Status BLOCKIERT | Blocker {release_blocker_count} |"),
        )
    } else {
        console_done_line.clone()
    };
    console.push_str(&final_console_done_line);
    for line in console.lines() {
        capture_console_line(line);
    }

    let clef_readability_shadow_master_course_audit =
        structural_write_authority::derive_clef_readability_shadow_course_audit(best.layout_xml());
    let clef_readability_shadow_render_artifact_course_audit =
        render_musicxml_artifact_for_audit
            .as_deref()
            .map(structural_write_authority::derive_clef_readability_shadow_course_audit);

    let mut text =
        DraftReport::new(report_preamble).compact(compact_current_report_preamble_v0101536);
    // The compact report now retains only the current result preamble. Every later
    // projection is forced through the exact-heading section ledger; retired historical
    // sections cannot recreate release gates by being absent from the standard report.
    text.project_owned_section(
        "final_chord_visible_rest_interval_epoch_witness_audit",
        crate::audit_only::final_chord_visible_rest_interval_epoch_witness_audit::SECTION,
        crate::audit_only::final_chord_visible_rest_interval_epoch_witness_audit::SECTION_RULE,
        |out| {
            use std::fmt::Write as _;

            crate::audit_only::final_chord_visible_rest_interval_epoch_witness_audit::
                append_final_chord_visible_rest_interval_epoch_witness_audit_section(
                    out,
                    &final_export_xml,
                    best.layout_xml(),
                    render_musicxml_artifact_for_audit.as_deref(),
                );
            crate::audit_only::lower_staff_overlapping_rest_bound_dispensability_audit::
                append_lower_staff_overlapping_rest_bound_dispensability_audit(
                    out,
                    &lower_staff_visible_rhythm_rest_lane_trial.duplicate_rest_chosen_evidence,
                    best.layout_xml(),
                    render_musicxml_artifact_for_audit.as_deref(),
                );
            let reanchor_trial =
                &lower_staff_secondary_reanchored_lane_rest_visibility_suppression_trial;
            let _ = writeln!(
                out,
                "ReanchorTrialEvidence: Attempted {}; Written {}; CandidatesSeen {}; CandidateAttempted {}; CandidateAdopted {}; CandidateRejected {}; AffectedMeasures {}; AffectedVisibleRests {}; ForwardSegmentsWritten {}; Verdict {}.",
                u8::from(reanchor_trial.attempted),
                u8::from(reanchor_trial.written),
                reanchor_trial.candidates_seen,
                reanchor_trial.candidate_attempted,
                reanchor_trial.candidate_adopted,
                reanchor_trial.candidate_rejected,
                reanchor_trial.affected_measures,
                reanchor_trial.affected_visible_rests,
                reanchor_trial.forward_segments_written,
                reanchor_trial.verdict,
            );
            for (index, action) in reanchor_trial.actions.iter().enumerate() {
                let _ = writeln!(
                    out,
                    "ReanchorAction#{}: Measure {}; RestFragments {}; Risk {}.",
                    index.saturating_add(1),
                    action.scope,
                    action.old_rest_count,
                    action.risk,
                );
            }
            for line in [
                &behind_bars_rule_check_rest_and_tie_audit.pipeline_line,
                &cross_staff_hand_role_materialization_trial.pipeline_line,
                &lower_staff_visible_rhythm_rest_lane_trial.pipeline_line,
                &cursor_lane_end_invariant_repair_audit.done_fragment,
                &strict_staff_local_release_completeness_trial.pipeline_line,
                &final_visible_rest_role_purge_and_vertical_placement_write_audit.pipeline_line,
                &strict_visible_rest_arithmetic_materializer_trial.pipeline_line,
                &external_feedback_safe_notation_write_audit.pipeline_line,
            ] {
                out.push_str(line);
                out.push('\n');
            }
        },
    )?;
    text.project_owned_section(
        "source_event_causal_ledger_foundation",
        "SOURCE-EVENT-CAUSAL-LEDGER-FOUNDATION",
        "-------------------------------------",
        |out| {
            crate::audit_only::source_event_causal_ledger::append_source_event_causal_ledger_foundation_body(
                out,
                &midi_projection,
                &source_event_causal_projection,
            );
        },
    )?;
    text.project_owned_section(
        "source_selection_full_causal_audit_selection_plan",
        "SOURCE-SELECTION-FULL-CAUSAL-AUDIT-AND-SELECTION-PLAN",
        "--------------------------------------------------------",
        |out| {
            crate::audit_only::source_selection_full_causal_audit_shadow_plan::append_source_selection_full_causal_audit_shadow_plan_body(
                out,
                &best.midi_seed_stats,
            );
        },
    )?;
    project_actual_beam_source_binding_report(&mut text, &best.material_lineage)?;
    text.project_owned_section(
        "candidate_material_lineage_spine",
        "CANDIDATE-MATERIAL-LINEAGE-SPINE",
        "----------------------------------",
        |out| {
            use std::fmt::Write as _;

            append_candidate_material_lineage_spine_body(out, &best.material_lineage);

            let freeze_boundary_xml = best.consensus_relocation_freeze_boundary_xml.as_deref();
            // The existing release stage already rebound the persisted master bytes into best.layout_xml.
            // Do not add a second proof/readback reader here.
            let persisted_master_xml = best.layout_xml();
            out.push_str(&staff_voice_rest_role_counterfactual_decision_audit(
                persisted_master_xml,
                &best.material_lineage,
            ));
            let final_chord_encoding_plan =
                crate::structural_write_authority::plan_final_chord_encoding(&final_export_xml).ok();
            let projected_final_chord_xml = final_chord_encoding_plan.as_ref().and_then(|plan| {
                crate::structural_write_authority::project_final_chord_encoding_plan(
                    &final_export_xml,
                    plan,
                ).0
                .ok()
            });

            let build_note_locator = |xml: &str| {
                let notes = parse_atomic_notes(xml);
                let mut next_raw_by_measure = std::collections::BTreeMap::<usize, usize>::new();
                let mut rows = std::collections::BTreeMap::<
                    (usize, usize),
                    (i64, Option<i32>, usize, usize),
                >::new();
                let mut parse_ok = true;
                for note in &notes {
                    let Ok(measure) = note.measure.parse::<usize>() else {
                        parse_ok = false;
                        continue;
                    };
                    let raw_note_index = next_raw_by_measure.entry(measure).or_default();
                    if rows
                        .insert(
                            (measure, *raw_note_index),
                            (
                                note.start,
                                note.pitch.as_ref().map(|pitch| pitch.midi),
                                note.note_start,
                                note.note_end,
                            ),
                        )
                        .is_some()
                    {
                        parse_ok = false;
                    }
                    *raw_note_index = raw_note_index.saturating_add(1);
                }
                (parse_ok, rows)
            };

            let freeze_locator = freeze_boundary_xml.map(build_note_locator);
            let final_export_locator = build_note_locator(&final_export_xml);
            let projected_locator = projected_final_chord_xml.as_deref().map(build_note_locator);
            let persisted_locator = build_note_locator(persisted_master_xml);
            let projected_notes = projected_final_chord_xml
                .as_deref()
                .map(parse_atomic_notes)
                .unwrap_or_default();

            type FinalChordNoteLocator = std::collections::BTreeMap<
                (usize, usize),
                (i64, Option<i32>, usize, usize),
            >;

            let source_topology_preserved = |
                before: &FinalChordNoteLocator,
                after: &FinalChordNoteLocator,
            | {
                before.len() == after.len()
                    && before.iter().all(|(key, (_, pitch, _, _))| {
                        after
                            .get(key)
                            .is_some_and(|(_, after_pitch, _, _)| after_pitch == pitch)
                    })
            };
            let temporal_topology_preserved = |
                before: &FinalChordNoteLocator,
                after: &FinalChordNoteLocator,
            | {
                before.len() == after.len()
                    && before.iter().all(|(key, (onset, pitch, _, _))| {
                        after.get(key).is_some_and(|(after_onset, after_pitch, _, _)| {
                            after_onset == onset && after_pitch == pitch
                        })
                    })
            };

            let freeze_to_final_export_proven = matches!(
                freeze_locator.as_ref(),
                Some((true, freeze_rows))
                    if final_export_locator.0
                        && source_topology_preserved(freeze_rows, &final_export_locator.1)
            );
            let final_chord_to_persisted_temporal_topology_proven = matches!(
                projected_locator.as_ref(),
                Some((true, projected_rows))
                    if persisted_locator.0
                        && temporal_topology_preserved(projected_rows, &persisted_locator.1)
            );

            // r672 diagnostic only: complete sounding-attack topology at the existing
            // proof frontiers. This intentionally uses only local tuples and the already
            // available AtomicNote parser; no rejected freeze-only type/helper is imported.
            // A sounding attack row matches the freeze semantics: pitched and not a tie-stop.
            let full_sounding_projection = |xml: &str| {
                let mut rows = Vec::<(usize, i64, i32)>::new();
                for note in parse_atomic_notes(xml) {
                    let Some(pitch) = note.pitch.as_ref().map(|pitch| pitch.midi) else {
                        continue;
                    };
                    let Ok(measure) = note.measure.parse::<usize>() else {
                        return None;
                    };
                    let block = xml.get(note.note_start..note.note_end)?;
                    let tie_stop = block.contains("<tie type=\"stop\"")
                        || block.contains("<tie type='stop'");
                    if !tie_stop {
                        rows.push((measure, note.start, pitch));
                    }
                }
                rows.sort_unstable();
                Some(rows)
            };

            let full_projection_fingerprint = |rows: &[(usize, i64, i32)]| {
                let mut state = 0xcbf29ce484222325u64;
                let update = |mut state: u64, bytes: &[u8]| {
                    for byte in bytes {
                        state ^= u64::from(*byte);
                        state = state.wrapping_mul(0x100000001b3);
                    }
                    state
                };
                state = update(
                    state,
                    b"ETUSCHLOPF_FINAL_SOUNDING_ATTACK_PROJECTION_V1\0",
                );
                for (measure, onset, pitch) in rows {
                    state = update(
                        state,
                        &u64::try_from(*measure).unwrap_or(u64::MAX).to_le_bytes(),
                    );
                    state = update(state, &onset.to_le_bytes());
                    state = update(state, &pitch.to_le_bytes());
                }
                state
            };

            let projection_multiset = |rows: &[(usize, i64, i32)]| {
                let mut counts = std::collections::BTreeMap::<(usize, i64, i32), usize>::new();
                for row in rows {
                    *counts.entry(*row).or_default() += 1;
                }
                counts
            };

            let projection_delta_counts = |
                before: &[(usize, i64, i32)],
                after: &[(usize, i64, i32)],
            | {
                let before_counts = projection_multiset(before);
                let after_counts = projection_multiset(after);
                let mut missing_total = 0usize;
                let mut extra_total = 0usize;
                for (row, before_count) in &before_counts {
                    let after_count = after_counts.get(row).copied().unwrap_or_default();
                    missing_total =
                        missing_total.saturating_add(before_count.saturating_sub(after_count));
                }
                for (row, after_count) in &after_counts {
                    let before_count = before_counts.get(row).copied().unwrap_or_default();
                    extra_total =
                        extra_total.saturating_add(after_count.saturating_sub(before_count));
                }
                (missing_total, extra_total)
            };

            let freeze_projection = freeze_boundary_xml.and_then(full_sounding_projection);
            let final_export_projection = full_sounding_projection(&final_export_xml);
            let final_chord_projection = projected_final_chord_xml
                .as_deref()
                .and_then(full_sounding_projection);
            let persisted_projection = full_sounding_projection(persisted_master_xml);

            // r678 outcome projection: the productive r677 cutover already bound the
            // FinalSoundingMaterial freeze in CandidateMaterialState. Project that runtime
            // authority truth without mutating music or re-reading any additional artifact.
            let mut frozen_material_projection = best
                .material_lineage
                .attacks
                .iter()
                .filter(|attack| attack.exists)
                .map(|attack| (attack.measure, attack.onset, attack.pitch))
                .collect::<Vec<_>>();
            frozen_material_projection.sort_unstable();
            let frozen_material_projection_fingerprint =
                full_projection_fingerprint(&frozen_material_projection);
            let freeze_state_seal_consistent =
                best.material_lineage.final_sounding_material_frozen
                    && best.material_lineage.frozen_state_fingerprint_fnv64
                        == best.material_lineage.state_fingerprint_fnv64
                    && best.material_lineage.frozen_active_attack_count
                        == best.material_lineage.active_attack_count
                    && best.material_lineage.frozen_projection_fingerprint_fnv64
                        == frozen_material_projection_fingerprint
                    && best.material_lineage.frozen_active_attack_count
                        == frozen_material_projection.len();
            let frozen_to_final_chord_delta = final_chord_projection
                .as_deref()
                .map(|rows| projection_delta_counts(&frozen_material_projection, rows));
            let frozen_to_persisted_delta = persisted_projection
                .as_deref()
                .map(|rows| projection_delta_counts(&frozen_material_projection, rows));
            let persisted_master_readback_proven = freeze_state_seal_consistent
                && matches!(frozen_to_persisted_delta, Some((0, 0)))
                && persisted_projection.as_deref().is_some_and(|rows| {
                    rows.len() == best.material_lineage.frozen_active_attack_count
                        && full_projection_fingerprint(rows)
                            == best.material_lineage.frozen_projection_fingerprint_fnv64
                });
            let final_chord_projection_matches_frozen = freeze_state_seal_consistent
                && matches!(frozen_to_final_chord_delta, Some((0, 0)));

            let _ = writeln!(
                out,
                "FinalSoundingMaterialFreezeAuthority: {}",
                if best.material_lineage.final_sounding_material_frozen {
                    "bound"
                } else {
                    "none"
                },
            );
            let _ = writeln!(
                out,
                "FinalSoundingMaterialFreezeSeal: state_fingerprint={:016x} frozen_state_fingerprint={:016x} active_attacks={} frozen_active_attacks={} projection_fingerprint={:016x} frozen_projection_fingerprint={:016x} seal_consistent={}",
                best.material_lineage.state_fingerprint_fnv64,
                best.material_lineage.frozen_state_fingerprint_fnv64,
                best.material_lineage.active_attack_count,
                best.material_lineage.frozen_active_attack_count,
                frozen_material_projection_fingerprint,
                best.material_lineage.frozen_projection_fingerprint_fnv64,
                if freeze_state_seal_consistent { "yes" } else { "no" },
            );
            let _ = writeln!(
                out,
                "FinalSoundingMaterialPersistedMasterReadback: {}",
                if persisted_master_readback_proven {
                    "proven"
                } else {
                    "AMBIGUOUS_FAIL_CLOSED"
                },
            );
            let cutover_projection_label = |rows: Option<&[(usize, i64, i32)]>| {
                rows.map_or_else(
                    || "unavailable".to_string(),
                    |rows| {
                        format!(
                            "rows={},fp={:016x}",
                            rows.len(),
                            full_projection_fingerprint(rows)
                        )
                    },
                )
            };
            let cutover_delta_label = |delta: Option<(usize, usize)>| {
                delta.map_or_else(
                    || "AMBIGUOUS_FAIL_CLOSED".to_string(),
                    |(missing, extra)| format!("missing={},extra={}", missing, extra),
                )
            };
            let _ = writeln!(
                out,
                "FinalSoundingMaterialCutoverProjectionAudit: FrozenMaterial=rows={},fp={:016x} FinalChordEncoding={} PersistedMaster={} | FrozenToFinalChord={} FrozenToPersisted={} | final_chord_matches_frozen={} persisted_readback={} ProductMutation=none WriterAuthority=none DecisionAuthority=none",
                frozen_material_projection.len(),
                frozen_material_projection_fingerprint,
                cutover_projection_label(final_chord_projection.as_deref()),
                cutover_projection_label(persisted_projection.as_deref()),
                cutover_delta_label(frozen_to_final_chord_delta),
                cutover_delta_label(frozen_to_persisted_delta),
                if final_chord_projection_matches_frozen { "yes" } else { "no" },
                if persisted_master_readback_proven { "proven" } else { "AMBIGUOUS_FAIL_CLOSED" },
            );

            let freeze_to_final_export_delta = freeze_projection
                .as_deref()
                .zip(final_export_projection.as_deref())
                .map(|(before, after)| projection_delta_counts(before, after));
            let final_export_to_final_chord_delta = final_export_projection
                .as_deref()
                .zip(final_chord_projection.as_deref())
                .map(|(before, after)| projection_delta_counts(before, after));
            let final_chord_to_persisted_delta = final_chord_projection
                .as_deref()
                .zip(persisted_projection.as_deref())
                .map(|(before, after)| projection_delta_counts(before, after));

            let first_divergence = match (
                freeze_to_final_export_delta,
                final_export_to_final_chord_delta,
                final_chord_to_persisted_delta,
            ) {
                (Some((0, 0)), Some((0, 0)), Some((0, 0))) => "none",
                (Some((missing, extra)), _, _) if missing != 0 || extra != 0 => {
                    "FreezeCandidate_to_InitialRenderFinalExport"
                }
                (Some((0, 0)), Some((missing, extra)), _)
                    if missing != 0 || extra != 0 =>
                {
                    "InitialRenderFinalExport_to_FinalChordEncoding"
                }
                (Some((0, 0)), Some((0, 0)), Some((missing, extra)))
                    if missing != 0 || extra != 0 =>
                {
                    "FinalChordEncoding_to_PersistedMaster"
                }
                _ => "AMBIGUOUS_FAIL_CLOSED",
            };

            let fmt_projection = |rows: Option<&[(usize, i64, i32)]>| {
                rows.map_or_else(
                    || "unavailable".to_string(),
                    |rows| {
                        format!(
                            "rows={},fp={:016x}",
                            rows.len(),
                            full_projection_fingerprint(rows)
                        )
                    },
                )
            };
            let fmt_delta = |delta: Option<(usize, usize)>| {
                delta.map_or_else(
                    || "AMBIGUOUS_FAIL_CLOSED".to_string(),
                    |(missing, extra)| format!("missing={},extra={}", missing, extra),
                )
            };

            let _ = writeln!(
                out,
                "FinalSoundingProjectionFullFrontierAudit: FreezeCandidate={} InitialRenderFinalExport={} FinalChordEncoding={} PersistedMaster={} | FreezeToInitial={} InitialToFinalChord={} FinalChordToPersisted={} | first_divergence={} | IdentityClaim=none ProductMutation=none WriterAuthority=none DecisionAuthority=none",
                fmt_projection(freeze_projection.as_deref()),
                fmt_projection(final_export_projection.as_deref()),
                fmt_projection(final_chord_projection.as_deref()),
                fmt_projection(persisted_projection.as_deref()),
                fmt_delta(freeze_to_final_export_delta),
                fmt_delta(final_export_to_final_chord_delta),
                fmt_delta(final_chord_to_persisted_delta),
                first_divergence,
            );

            // r673 diagnostic only: carry every pre-FinalChord sounding source note
            // forward through the already-existing byte-topological carrier. This neither
            // changes FinalChordEncoding nor infers identity backwards from musical fields.
            let source_notes_for_final_chord_audit = parse_atomic_notes(&final_export_xml);
            let mut projected_note_by_start =
                std::collections::BTreeMap::<usize, &AtomicNote>::new();
            let mut projected_note_start_unique = true;
            for note in &projected_notes {
                if projected_note_by_start.insert(note.note_start, note).is_some() {
                    projected_note_start_unique = false;
                }
            }

            let mut sounding_source_rows = 0usize;
            let mut carrier_ok_rows = 0usize;
            let mut carrier_failures =
                std::collections::BTreeMap::<String, usize>::new();
            let mut deleted_source_note_ranges =
                std::collections::BTreeSet::<(usize, usize)>::new();
            let mut onset_changed_rows = 0usize;
            let mut onset_preserved_rows = 0usize;
            let mut pitch_changed_rows = 0usize;
            let mut duration_changed_rows = 0usize;
            let mut projected_tie_stop_rows = 0usize;
            let mut chord_non_to_chord = 0usize;
            let mut chord_chord_to_non = 0usize;
            let mut chord_chord_to_chord = 0usize;
            let mut chord_non_to_non = 0usize;
            let mut carried_changed_before =
                std::collections::BTreeMap::<(usize, i64, i32), usize>::new();
            let mut carried_changed_after =
                std::collections::BTreeMap::<(usize, i64, i32), usize>::new();

            if let Some(plan) = final_chord_encoding_plan.as_ref() {
                for source_note in &source_notes_for_final_chord_audit {
                    let Some(source_pitch) =
                        source_note.pitch.as_ref().map(|pitch| pitch.midi)
                    else {
                        continue;
                    };
                    let Ok(source_measure) = source_note.measure.parse::<usize>() else {
                        *carrier_failures
                            .entry("source_measure_parse_failed".to_string())
                            .or_default() += 1;
                        continue;
                    };
                    let Some(source_block) =
                        final_export_xml.get(source_note.note_start..source_note.note_end)
                    else {
                        *carrier_failures
                            .entry("source_note_block_unavailable".to_string())
                            .or_default() += 1;
                        continue;
                    };
                    let source_tie_stop = source_block.contains("<tie type=\"stop\"")
                        || source_block.contains("<tie type='stop'");
                    if source_tie_stop {
                        continue;
                    }
                    sounding_source_rows = sounding_source_rows.saturating_add(1);

                    let carried_start = match plan.carry_surviving_source_note_start(
                        source_note.note_start,
                        source_note.note_end,
                    ) {
                        Ok(value) => value,
                        Err(reason) => {
                            if reason == "source_note_deleted" {
                                deleted_source_note_ranges
                                    .insert((source_note.note_start, source_note.note_end));
                            }
                            *carrier_failures.entry(reason.to_string()).or_default() += 1;
                            continue;
                        }
                    };
                    if !projected_note_start_unique {
                        *carrier_failures
                            .entry("projected_note_start_not_unique".to_string())
                            .or_default() += 1;
                        continue;
                    }
                    let Some(projected_note) = projected_note_by_start.get(&carried_start).copied()
                    else {
                        *carrier_failures
                            .entry("carried_note_start_not_found".to_string())
                            .or_default() += 1;
                        continue;
                    };
                    let Some(projected_pitch) =
                        projected_note.pitch.as_ref().map(|pitch| pitch.midi)
                    else {
                        *carrier_failures
                            .entry("carried_note_lost_pitch".to_string())
                            .or_default() += 1;
                        continue;
                    };
                    let Ok(projected_measure) = projected_note.measure.parse::<usize>() else {
                        *carrier_failures
                            .entry("projected_measure_parse_failed".to_string())
                            .or_default() += 1;
                        continue;
                    };
                    let Some(projected_xml) = projected_final_chord_xml.as_deref() else {
                        *carrier_failures
                            .entry("projected_final_chord_xml_unavailable".to_string())
                            .or_default() += 1;
                        continue;
                    };
                    let Some(projected_block) =
                        projected_xml.get(projected_note.note_start..projected_note.note_end)
                    else {
                        *carrier_failures
                            .entry("projected_note_block_unavailable".to_string())
                            .or_default() += 1;
                        continue;
                    };
                    let projected_tie_stop = projected_block.contains("<tie type=\"stop\"")
                        || projected_block.contains("<tie type='stop'");
                    if projected_tie_stop {
                        projected_tie_stop_rows = projected_tie_stop_rows.saturating_add(1);
                        continue;
                    }

                    carrier_ok_rows = carrier_ok_rows.saturating_add(1);
                    if projected_pitch != source_pitch || projected_measure != source_measure {
                        pitch_changed_rows = pitch_changed_rows
                            .saturating_add(usize::from(projected_pitch != source_pitch));
                        *carrier_failures
                            .entry("carried_pitch_or_measure_invariant_failed".to_string())
                            .or_default() += 1;
                        continue;
                    }
                    if projected_note.duration != source_note.duration {
                        duration_changed_rows = duration_changed_rows.saturating_add(1);
                    }

                    if projected_note.start == source_note.start {
                        onset_preserved_rows = onset_preserved_rows.saturating_add(1);
                    } else {
                        onset_changed_rows = onset_changed_rows.saturating_add(1);
                        *carried_changed_before
                            .entry((source_measure, source_note.start, source_pitch))
                            .or_default() += 1;
                        *carried_changed_after
                            .entry((projected_measure, projected_note.start, projected_pitch))
                            .or_default() += 1;

                        match (source_note.is_chord, projected_note.is_chord) {
                            (false, true) => {
                                chord_non_to_chord = chord_non_to_chord.saturating_add(1)
                            }
                            (true, false) => {
                                chord_chord_to_non = chord_chord_to_non.saturating_add(1)
                            }
                            (true, true) => {
                                chord_chord_to_chord = chord_chord_to_chord.saturating_add(1)
                            }
                            (false, false) => {
                                chord_non_to_non = chord_non_to_non.saturating_add(1)
                            }
                        }

                        let _ = writeln!(
                            out,
                            "FinalChordForwardSoundingCarrierDelta: source_event_claim=none source_onset={} projected_onset={} pitch={} source_chord={} projected_chord={} source_duration={} projected_duration={} IdentityCarrier=byte_topological_forward_only",
                            source_note.start,
                            projected_note.start,
                            source_pitch,
                            if source_note.is_chord { "yes" } else { "no" },
                            if projected_note.is_chord { "yes" } else { "no" },
                            source_note.duration,
                            projected_note.duration,
                        );
                    }
                }
            } else {
                carrier_failures.insert("final_chord_plan_unavailable".to_string(), 1);
            }

            let observed_missing = final_export_projection
                .as_deref()
                .zip(final_chord_projection.as_deref())
                .map(|(before, after)| {
                    let before_counts = projection_multiset(before);
                    let after_counts = projection_multiset(after);
                    let mut rows = std::collections::BTreeMap::<(usize, i64, i32), usize>::new();
                    for (row, before_count) in before_counts {
                        let after_count = after_counts.get(&row).copied().unwrap_or_default();
                        let missing = before_count.saturating_sub(after_count);
                        if missing != 0 {
                            rows.insert(row, missing);
                        }
                    }
                    rows
                });
            let observed_extra = final_export_projection
                .as_deref()
                .zip(final_chord_projection.as_deref())
                .map(|(before, after)| {
                    let before_counts = projection_multiset(before);
                    let after_counts = projection_multiset(after);
                    let mut rows = std::collections::BTreeMap::<(usize, i64, i32), usize>::new();
                    for (row, after_count) in after_counts {
                        let before_count = before_counts.get(&row).copied().unwrap_or_default();
                        let extra = after_count.saturating_sub(before_count);
                        if extra != 0 {
                            rows.insert(row, extra);
                        }
                    }
                    rows
                });
            let global_delta_exactly_explained = observed_missing
                .as_ref()
                .zip(observed_extra.as_ref())
                .is_some_and(|(missing, extra)| {
                    *missing == carried_changed_before && *extra == carried_changed_after
                });

            let carrier_failure_summary = if carrier_failures.is_empty() {
                "none".to_string()
            } else {
                carrier_failures
                    .iter()
                    .map(|(reason, count)| format!("{}:{}", reason, count))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let _ = writeln!(
                out,
                "FinalChordForwardSoundingCarrierAudit: sounding_source_rows={} carrier_ok_rows={} onset_preserved_rows={} onset_changed_rows={} pitch_changed_rows={} duration_changed_rows={} projected_tie_stop_rows={} chord_state_changed_non_to_chord={} chord_state_changed_chord_to_non={} chord_state_changed_chord_to_chord={} chord_state_changed_non_to_non={} global_26x26_delta_exactly_explained={} carrier_failures={} | FinalChordBehaviorChange=none IdentityClaim=forward_carrier_only ProductMutation=none WriterAuthority=none DecisionAuthority=none",
                sounding_source_rows,
                carrier_ok_rows,
                onset_preserved_rows,
                onset_changed_rows,
                pitch_changed_rows,
                duration_changed_rows,
                projected_tie_stop_rows,
                chord_non_to_chord,
                chord_chord_to_non,
                chord_chord_to_chord,
                chord_non_to_non,
                if global_delta_exactly_explained { "yes" } else { "no" },
                carrier_failure_summary,
            );

            // r674 diagnostic only: consume the transaction-local provenance that
            // FinalChordEncoding already knows at the moment it removes and rematerializes
            // a source note. This is a forward carrier, never a pitch/onset identity match.
            let mut source_note_by_range =
                std::collections::BTreeMap::<(usize, usize), &AtomicNote>::new();
            let mut source_note_range_unique = true;
            for note in &source_notes_for_final_chord_audit {
                if source_note_by_range
                    .insert((note.note_start, note.note_end), note)
                    .is_some()
                {
                    source_note_range_unique = false;
                }
            }

            let mut rematerialization_rows = 0usize;
            let mut rematerialization_onset_changed = 0usize;
            let mut rematerialization_onset_preserved = 0usize;
            let mut rematerialization_tied_anchor_chain = 0usize;
            let mut rematerialization_auxiliary_lane = 0usize;
            let mut rematerialization_source_invariant_failures = 0usize;
            let mut rematerialization_changed_before =
                std::collections::BTreeMap::<(usize, i64, i32), usize>::new();
            let mut rematerialization_changed_after =
                std::collections::BTreeMap::<(usize, i64, i32), usize>::new();
            let mut provenance_source_ranges =
                std::collections::BTreeSet::<(usize, usize)>::new();

            if source_note_range_unique {
                if let Some(plan) = final_chord_encoding_plan.as_ref() {
                    for provenance in plan.forward_materializations() {
                        rematerialization_rows = rematerialization_rows.saturating_add(1);
                        provenance_source_ranges.insert((
                            provenance.source_note_start,
                            provenance.source_note_end,
                        ));
                        match provenance.kind {
                            crate::structural_write_authority::FinalChordForwardMaterializationKind::TiedAnchorChain => {
                                rematerialization_tied_anchor_chain =
                                    rematerialization_tied_anchor_chain.saturating_add(1);
                            }
                            crate::structural_write_authority::FinalChordForwardMaterializationKind::AuxiliaryLane => {
                                rematerialization_auxiliary_lane =
                                    rematerialization_auxiliary_lane.saturating_add(1);
                            }
                        }

                        let Some(source_note) = source_note_by_range
                            .get(&(provenance.source_note_start, provenance.source_note_end))
                            .copied()
                        else {
                            rematerialization_source_invariant_failures =
                                rematerialization_source_invariant_failures.saturating_add(1);
                            continue;
                        };
                        let Some(source_pitch) =
                            source_note.pitch.as_ref().map(|pitch| pitch.midi)
                        else {
                            rematerialization_source_invariant_failures =
                                rematerialization_source_invariant_failures.saturating_add(1);
                            continue;
                        };
                        let Ok(source_measure) = source_note.measure.parse::<usize>() else {
                            rematerialization_source_invariant_failures =
                                rematerialization_source_invariant_failures.saturating_add(1);
                            continue;
                        };
                        let Some(source_block) =
                            final_export_xml.get(source_note.note_start..source_note.note_end)
                        else {
                            rematerialization_source_invariant_failures =
                                rematerialization_source_invariant_failures.saturating_add(1);
                            continue;
                        };
                        let source_tie_stop = source_block.contains("<tie type=\"stop\"")
                            || source_block.contains("<tie type='stop'");
                        if source_tie_stop
                            || source_note.start != provenance.source_onset
                        {
                            rematerialization_source_invariant_failures =
                                rematerialization_source_invariant_failures.saturating_add(1);
                            continue;
                        }

                        if provenance.projected_sounding_onset == provenance.source_onset {
                            rematerialization_onset_preserved =
                                rematerialization_onset_preserved.saturating_add(1);
                        } else {
                            rematerialization_onset_changed =
                                rematerialization_onset_changed.saturating_add(1);
                            *rematerialization_changed_before
                                .entry((
                                    source_measure,
                                    provenance.source_onset,
                                    source_pitch,
                                ))
                                .or_default() += 1;
                            *rematerialization_changed_after
                                .entry((
                                    source_measure,
                                    provenance.projected_sounding_onset,
                                    source_pitch,
                                ))
                                .or_default() += 1;
                        }

                        let _ = writeln!(
                            out,
                            "FinalChordRemovedSourceForwardMaterialization: source_event_claim=none measure={} pitch={} source_onset={} projected_sounding_onset={} generated_fragments={} kind={} source_range={}..{} IdentityCarrier=planner_transaction_forward_only",
                            source_measure,
                            source_pitch,
                            provenance.source_onset,
                            provenance.projected_sounding_onset,
                            provenance.generated_fragment_count,
                            provenance.kind.label(),
                            provenance.source_note_start,
                            provenance.source_note_end,
                        );
                    }
                }
            } else {
                rematerialization_source_invariant_failures =
                    rematerialization_source_invariant_failures.saturating_add(1);
            }

            let removed_sources_exactly_covered =
                source_note_range_unique
                    && provenance_source_ranges == deleted_source_note_ranges;
            let rematerialization_global_delta_exactly_explained = observed_missing
                .as_ref()
                .zip(observed_extra.as_ref())
                .is_some_and(|(missing, extra)| {
                    *missing == rematerialization_changed_before
                        && *extra == rematerialization_changed_after
                });
            let _ = writeln!(
                out,
                "FinalChordRemovedSourceForwardMaterializationAudit: deleted_source_rows={} provenance_rows={} removed_sources_exactly_covered={} onset_changed_rows={} onset_preserved_rows={} tied_anchor_chain_rows={} auxiliary_lane_rows={} source_invariant_failures={} global_frontier_delta_exactly_explained={} | FinalChordBehaviorChange=none IdentityClaim=planner_transaction_forward_only ProductMutation=none WriterAuthority=none DecisionAuthority=none",
                deleted_source_note_ranges.len(),
                rematerialization_rows,
                if removed_sources_exactly_covered { "yes" } else { "no" },
                rematerialization_onset_changed,
                rematerialization_onset_preserved,
                rematerialization_tied_anchor_chain,
                rematerialization_auxiliary_lane,
                rematerialization_source_invariant_failures,
                if rematerialization_global_delta_exactly_explained { "yes" } else { "no" },
            );

            // r675 diagnostic only: bridge the planner's already-forward source-range
            // provenance to the opaque CandidateMaterial identity that was carried from the
            // freeze-boundary candidate into InitialRender by stable measure/raw-note topology.
            // This does not infer identity from pitch/onset: the raw position is accepted only
            // after the complete FreezeCandidate -> InitialRenderFinalExport temporal topology
            // is proven byte-transaction-locally stable.
            let freeze_to_final_export_temporal_topology_proven = matches!(
                freeze_locator.as_ref(),
                Some((true, freeze_rows))
                    if final_export_locator.0
                        && temporal_topology_preserved(freeze_rows, &final_export_locator.1)
            );

            let mut final_export_raw_key_by_range =
                std::collections::BTreeMap::<(usize, usize), (usize, usize)>::new();
            let mut final_export_raw_range_unique = true;
            let mut final_export_next_raw =
                std::collections::BTreeMap::<usize, usize>::new();
            for note in &source_notes_for_final_chord_audit {
                let Ok(measure) = note.measure.parse::<usize>() else {
                    final_export_raw_range_unique = false;
                    continue;
                };
                let raw_note_index = final_export_next_raw.entry(measure).or_default();
                if final_export_raw_key_by_range
                    .insert(
                        (note.note_start, note.note_end),
                        (measure, *raw_note_index),
                    )
                    .is_some()
                {
                    final_export_raw_range_unique = false;
                }
                *raw_note_index = raw_note_index.saturating_add(1);
            }

            let mut material_fragment_by_raw =
                std::collections::BTreeMap::<(usize, usize), &CandidateMaterialFragmentState>::new();
            let mut material_fragment_raw_unique = true;
            for fragment in best.material_lineage.fragments.iter() {
                if material_fragment_by_raw
                    .insert((fragment.measure, fragment.raw_note_index), fragment)
                    .is_some()
                {
                    material_fragment_raw_unique = false;
                }
            }

            let attack_by_source = best
                .material_lineage
                .attacks
                .iter()
                .map(|attack| (attack.source_event_id, attack))
                .collect::<std::collections::BTreeMap<_, _>>();
            let consensus_changed_sources = best
                .material_lineage
                .receipts
                .iter()
                .filter(|receipt| receipt.owner.starts_with("ConsensusGestureProbe/"))
                .flat_map(|receipt| receipt.changed_source_ordinals.iter().copied())
                .map(MaterialSourceEventId)
                .collect::<std::collections::BTreeSet<_>>();

            let mut bridge_rows = 0usize;
            let mut bridge_onset_changed_rows = 0usize;
            let mut bridge_onset_preserved_rows = 0usize;
            let mut bridge_failures = 0usize;
            let mut bridged_source_ids = std::collections::BTreeSet::<MaterialSourceEventId>::new();
            let mut bridged_attack_ids = std::collections::BTreeSet::<SoundingAttackId>::new();
            let mut changed_source_ids = std::collections::BTreeSet::<MaterialSourceEventId>::new();
            let mut consensus_overlap_rows = 0usize;
            let mut cutover_materialized_onset_rows = 0usize;
            let mut duration_values = std::collections::BTreeSet::<i64>::new();

            if freeze_to_final_export_temporal_topology_proven
                && final_export_raw_range_unique
                && material_fragment_raw_unique
            {
                if let Some(plan) = final_chord_encoding_plan.as_ref() {
                    for provenance in plan.forward_materializations() {
                        bridge_rows = bridge_rows.saturating_add(1);
                        let Some(source_note) = source_note_by_range
                            .get(&(provenance.source_note_start, provenance.source_note_end))
                            .copied()
                        else {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        };
                        let Some(source_pitch) =
                            source_note.pitch.as_ref().map(|pitch| pitch.midi)
                        else {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        };
                        let Some((measure, raw_note_index)) = final_export_raw_key_by_range
                            .get(&(provenance.source_note_start, provenance.source_note_end))
                            .copied()
                        else {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        };
                        let Some(fragment) =
                            material_fragment_by_raw.get(&(measure, raw_note_index)).copied()
                        else {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        };
                        let Some(source_event_id) = fragment.source_event_id else {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        };
                        let Some(attack) = attack_by_source.get(&source_event_id).copied() else {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        };
                        let duration = attack.sounding_end.checked_sub(attack.onset);
                        let identity_green = attack.exists
                            && fragment.pitch == Some(source_pitch)
                            && fragment.onset == provenance.source_onset
                            && attack.measure == measure
                            && attack.pitch == source_pitch
                            && attack.onset == provenance.projected_sounding_onset
                            && duration.is_some_and(|value| value > 0);
                        if !identity_green {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        }
                        let duration = duration.expect("checked positive duration");
                        if !bridged_source_ids.insert(source_event_id)
                            || !bridged_attack_ids.insert(attack.attack_id)
                        {
                            bridge_failures = bridge_failures.saturating_add(1);
                            continue;
                        }
                        duration_values.insert(duration);
                        if consensus_changed_sources.contains(&source_event_id) {
                            consensus_overlap_rows = consensus_overlap_rows.saturating_add(1);
                        }
                        if provenance.projected_sounding_onset == provenance.source_onset {
                            bridge_onset_preserved_rows =
                                bridge_onset_preserved_rows.saturating_add(1);
                        } else {
                            bridge_onset_changed_rows =
                                bridge_onset_changed_rows.saturating_add(1);
                            cutover_materialized_onset_rows =
                                cutover_materialized_onset_rows.saturating_add(1);
                            changed_source_ids.insert(source_event_id);
                        }
                        let _ = writeln!(
                            out,
                            "FinalChordSoundingIdentityBridge: SourceEventId={} SoundingAttackId={} measure={} raw_note_index={} pitch={} material_onset={} projected_sounding_onset={} sounding_duration={} kind={} IdentityCarrier=freeze_raw_position_forward_then_planner_source_range_forward",
                            source_event_id.0,
                            attack.attack_id.0,
                            measure,
                            raw_note_index,
                            source_pitch,
                            attack.onset,
                            provenance.projected_sounding_onset,
                            duration,
                            provenance.kind.label(),
                        );
                    }
                } else {
                    bridge_failures = bridge_failures.saturating_add(1);
                }
            } else {
                bridge_failures = bridge_failures.saturating_add(1);
            }

            let final_chord_material_receipt = best
                .material_lineage
                .receipts
                .iter()
                .find(|receipt| receipt.owner == "FinalChordEncodingSoundingMaterialization");
            let final_chord_material_receipt_green =
                final_chord_material_receipt.is_some_and(|receipt| {
                    receipt.active_before == receipt.active_after
                        && receipt.existence_changes == 0
                        && receipt.onset_changes == bridge_onset_changed_rows
                        && receipt.sounding_end_changes == 0
                        && receipt.pitch_changes == 0
                        && receipt.staff_changes == 0
                        && receipt.voice_changes == 0
                        && !receipt.declared.attack_existence
                        && receipt.declared.attack_onset
                        && !receipt.declared.sounding_end
                        && !receipt.declared.pitch
                        && !receipt.declared.staff
                        && !receipt.declared.voice
                });
            let identity_bridge_complete = bridge_failures == 0
                && final_chord_material_receipt_green
                && best.material_lineage.final_sounding_material_frozen
                && freeze_state_seal_consistent
                && persisted_master_readback_proven
                && removed_sources_exactly_covered
                && rematerialization_global_delta_exactly_explained
                && bridge_rows == provenance_source_ranges.len()
                && bridged_source_ids.len() == bridge_rows
                && bridged_attack_ids.len() == bridge_rows
                && bridge_onset_changed_rows
                    .saturating_add(bridge_onset_preserved_rows)
                    == bridge_rows
                && changed_source_ids.len() == bridge_onset_changed_rows;
            let _ = writeln!(
                out,
                "FinalChordSoundingIdentityBridgeAudit: freeze_to_final_export_temporal_topology={} provenance_rows={} bridged_rows={} unique_source_ids={} unique_attack_ids={} onset_changed_rows={} onset_preserved_rows={} cutover_materialized_onset_rows={} consensus_overlap_rows={} positive_duration_values=[{}] failures={} receipt_green={} freeze_authority={} persisted_readback={} identity_bridge_complete={} | IdentityInference=none MaterialStateMutation=already_committed_FinalChordEncodingSoundingMaterialization FinalChordBehaviorChange=none",
                if freeze_to_final_export_temporal_topology_proven { "proven" } else { "AMBIGUOUS_FAIL_CLOSED" },
                bridge_rows,
                bridged_source_ids.len(),
                bridged_source_ids.len(),
                bridged_attack_ids.len(),
                bridge_onset_changed_rows,
                bridge_onset_preserved_rows,
                cutover_materialized_onset_rows,
                consensus_overlap_rows,
                duration_values
                    .iter()
                    .map(i64::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                bridge_failures,
                if final_chord_material_receipt_green { "yes" } else { "no" },
                if best.material_lineage.final_sounding_material_frozen { "bound" } else { "none" },
                if persisted_master_readback_proven { "proven" } else { "AMBIGUOUS_FAIL_CLOSED" },
                if identity_bridge_complete { "yes" } else { "no" },
            );

            let append_projection_delta = |
                out: &mut String,
                edge: &str,
                before: Option<&[(usize, i64, i32)]>,
                after: Option<&[(usize, i64, i32)]>,
            | {
                let (Some(before), Some(after)) = (before, after) else {
                    let _ = writeln!(
                        out,
                        "FinalSoundingProjectionDelta edge={} status=AMBIGUOUS_FAIL_CLOSED",
                        edge
                    );
                    return;
                };
                let before_counts = projection_multiset(before);
                let after_counts = projection_multiset(after);
                let mut ordinal = 0usize;
                for ((measure, onset, pitch), before_count) in &before_counts {
                    let after_count = after_counts
                        .get(&(*measure, *onset, *pitch))
                        .copied()
                        .unwrap_or_default();
                    let missing = before_count.saturating_sub(after_count);
                    if missing != 0 {
                        ordinal = ordinal.saturating_add(1);
                        let _ = writeln!(
                            out,
                            "FinalSoundingProjectionDelta#{} edge={} kind=missing measure={} onset={} pitch={} multiplicity={} IdentityClaim=none",
                            ordinal, edge, measure, onset, pitch, missing,
                        );
                    }
                }
                for ((measure, onset, pitch), after_count) in &after_counts {
                    let before_count = before_counts
                        .get(&(*measure, *onset, *pitch))
                        .copied()
                        .unwrap_or_default();
                    let extra = after_count.saturating_sub(before_count);
                    if extra != 0 {
                        ordinal = ordinal.saturating_add(1);
                        let _ = writeln!(
                            out,
                            "FinalSoundingProjectionDelta#{} edge={} kind=extra measure={} onset={} pitch={} multiplicity={} IdentityClaim=none",
                            ordinal, edge, measure, onset, pitch, extra,
                        );
                    }
                }
                if ordinal == 0 {
                    let _ = writeln!(
                        out,
                        "FinalSoundingProjectionDelta edge={} status=exact_match",
                        edge
                    );
                }
            };

            append_projection_delta(
                out,
                "FreezeCandidate_to_InitialRenderFinalExport",
                freeze_projection.as_deref(),
                final_export_projection.as_deref(),
            );
            append_projection_delta(
                out,
                "InitialRenderFinalExport_to_FinalChordEncoding",
                final_export_projection.as_deref(),
                final_chord_projection.as_deref(),
            );
            append_projection_delta(
                out,
                "FinalChordEncoding_to_PersistedMaster",
                final_chord_projection.as_deref(),
                persisted_projection.as_deref(),
            );

            let _ = writeln!(
                out,
                "ConsensusRelocationLongitudinalIdentityAudit: rows={} freeze_snapshot={} freeze_to_InitialRenderTransactionFinalExport={} FinalChordEncodingCarrier=per_source_surviving_note_only FinalChordToPersistedTemporalTopology={}; selection=observed_ConsensusGestureProbe_AttackOnset_delta_only; ProductMutation=none; WriterAuthority=none; DecisionAuthority=none",
                best.consensus_relocation_longitudinal_trace.len(),
                if freeze_boundary_xml.is_some() { "captured" } else { "AMBIGUOUS_FAIL_CLOSED" },
                if freeze_to_final_export_proven { "proven_transaction_local" } else { "AMBIGUOUS_FAIL_CLOSED" },
                if final_chord_to_persisted_temporal_topology_proven { "proven_ordinal_onset_pitch" } else { "AMBIGUOUS_FAIL_CLOSED" },
            );

            for (index, trace) in best
                .consensus_relocation_longitudinal_trace
                .iter()
                .enumerate()
            {
                let freeze_attack = best.material_lineage.attacks.iter().find(|attack| {
                    attack.source_event_id == trace.source_event_id
                        && attack.attack_id == trace.attack_id
                });
                let source_fragments = best
                    .material_lineage
                    .fragments
                    .iter()
                    .filter(|fragment| {
                        fragment.source_event_id == Some(trace.source_event_id)
                            && fragment.attack_id == Some(trace.attack_id)
                            && fragment.is_sounding_attack_fragment()
                    })
                    .collect::<Vec<_>>();
                let freeze_fragment = if source_fragments.len() == 1 {
                    source_fragments.first().copied()
                } else {
                    None
                };

                let freeze_xml_guarded_onset = freeze_fragment.and_then(|fragment| {
                    freeze_locator.as_ref().and_then(|(parse_ok, rows)| {
                        if !*parse_ok {
                            return None;
                        }
                        rows.get(&(fragment.measure, fragment.raw_note_index)).and_then(
                            |(onset, pitch, _, _)| {
                                if *onset == fragment.onset && *pitch == fragment.pitch {
                                    Some(*onset)
                                } else {
                                    None
                                }
                            },
                        )
                    })
                });

                let final_export_row = if freeze_to_final_export_proven {
                    freeze_fragment.and_then(|fragment| {
                        final_export_locator
                            .1
                            .get(&(fragment.measure, fragment.raw_note_index))
                            .copied()
                    })
                } else {
                    None
                };
                let final_export_onset = final_export_row.map(|row| row.0);

                let (final_chord_carrier_status, final_chord_carried_note) =
                    if let Some((_, source_pitch, source_note_start, source_note_end)) =
                        final_export_row
                    {
                        if let Some(plan) = final_chord_encoding_plan.as_ref() {
                            match plan.carry_surviving_source_note_start(
                                source_note_start,
                                source_note_end,
                            ) {
                                Ok(carried_start) => {
                                    let carried_note = projected_notes.iter().find(|note| {
                                        note.note_start == carried_start
                                            && note.pitch.as_ref().map(|pitch| pitch.midi)
                                                == source_pitch
                                    });
                                    if carried_note.is_some() {
                                        ("carried", carried_note)
                                    } else if projected_notes
                                        .iter()
                                        .any(|note| note.note_start == carried_start)
                                    {
                                        ("carried_start_pitch_invariant_failed", None)
                                    } else {
                                        ("carried_start_missing_projected_note", None)
                                    }
                                }
                                Err(reason) => (reason, None),
                            }
                        } else {
                            ("final_chord_plan_unavailable", None)
                        }
                    } else {
                        ("final_export_source_note_unproven", None)
                    };
                let final_chord_attack_onset = final_chord_carried_note.map(|note| note.start);
                let persisted_master_onset = if final_chord_to_persisted_temporal_topology_proven {
                    final_chord_carried_note.and_then(|carried| {
                        let measure = carried.measure.parse::<usize>().ok()?;
                        let raw_note_index = projected_notes
                            .iter()
                            .filter(|note| note.measure == carried.measure)
                            .position(|note| note.note_start == carried.note_start)?;
                        persisted_locator
                            .1
                            .get(&(measure, raw_note_index))
                            .map(|(onset, _, _, _)| *onset)
                    })
                } else {
                    None
                };

                let freeze_state_onset = freeze_attack.map(|attack| attack.onset);
                let freeze_state_end = freeze_attack.map(|attack| attack.sounding_end);
                let duration_before = trace.sounding_end_before.checked_sub(trace.onset_before);
                let duration_after_owner = trace
                    .sounding_end_after_owner
                    .checked_sub(trace.onset_after_owner);
                let freeze_duration = freeze_attack
                    .and_then(|attack| attack.sounding_end.checked_sub(attack.onset));

                let classification = match (
                    freeze_state_onset,
                    freeze_xml_guarded_onset,
                    final_chord_attack_onset,
                    persisted_master_onset,
                ) {
                    (Some(freeze_state), Some(freeze_xml), Some(final_chord), Some(persisted))
                        if trace.onset_after_owner == freeze_state
                            && freeze_xml == freeze_state
                            && final_chord == freeze_state
                            && persisted == freeze_state =>
                    {
                        "OWNER_ONSET_PRESERVED"
                    }
                    (Some(freeze_state), Some(freeze_xml), Some(final_chord), Some(persisted))
                        if trace.onset_after_owner == freeze_state
                            && freeze_xml == freeze_state
                            && final_chord == persisted
                            && final_chord != freeze_state =>
                    {
                        "REPRESENTATION_ONSET_ONLY"
                    }
                    (Some(freeze_state), Some(freeze_xml), _, Some(persisted))
                        if trace.onset_after_owner == persisted
                            && freeze_xml == persisted
                            && freeze_state != trace.onset_after_owner =>
                    {
                        "OWNER_ONSET_LOST_IN_STATE"
                    }
                    _ => "AMBIGUOUS_FAIL_CLOSED",
                };
                let fmt_opt = |value: Option<i64>| {
                    value.map_or_else(|| "?".to_string(), |value| value.to_string())
                };
                let _ = writeln!(
                    out,
                    "ConsensusRelocationTrace#{} source_event_id={} attack_id={} owner={} measure={}->{} pitch={}->{} before_onset={} after_owner_onset={} after_receipt_state_onset={} freeze_candidate_state_onset={} freeze_candidate_xml_onset={} initial_render_final_export_onset={} final_chord_carrier_status={} final_chord_attack_onset={} persisted_master_onset={} before_end={} after_owner_end={} freeze_state_end={} duration_before={} duration_after_owner={} freeze_duration={} classification={}",
                    index + 1,
                    trace.source_event_id.0,
                    trace.attack_id.0,
                    trace.owner,
                    trace.measure_before,
                    trace.measure_after_owner,
                    trace.pitch_before,
                    trace.pitch_after_owner,
                    trace.onset_before,
                    trace.onset_after_owner,
                    trace.onset_after_owner,
                    fmt_opt(freeze_state_onset),
                    fmt_opt(freeze_xml_guarded_onset),
                    fmt_opt(final_export_onset),
                    final_chord_carrier_status,
                    fmt_opt(final_chord_attack_onset),
                    fmt_opt(persisted_master_onset),
                    trace.sounding_end_before,
                    trace.sounding_end_after_owner,
                    fmt_opt(freeze_state_end),
                    fmt_opt(duration_before),
                    fmt_opt(duration_after_owner),
                    fmt_opt(freeze_duration),
                    classification,
                );
            }
        },
    )?;
    text.project_section(
        "global_identity_debt_inventory",
        "GLOBAL-IDENTITY-DEBT-INVENTORY",
        identity_debt_inventory::append_global_identity_debt_inventory_section,
    )?;
    text.project_bundle(
        "feedback_identity_firewall",
        &[
            ProjectionIdentity::Section("FEEDBACK-IDENTITY-FIREWALL"),
            ProjectionIdentity::Section("KODEX-BOUNDARY-ADVERSARIAL-COMPLETION"),
        ],
        identity_debt_inventory::append_feedback_identity_firewall_section,
    )?;
    let feedback_origin_noninterference_pipeline_audit =
        feedback_structuralization_gate::audit_feedback_origin_noninterference_in_pipeline(
            &source.input_label,
            final_render_snapshot_xml,
        );
    text.project_section(
        "feedback_origin_noninterference_pipeline_proof",
        "FEEDBACK-ORIGIN-NONINTERFERENCE-PIPELINE-PROOF-NO-APPLY",
        |out| {
            feedback_structuralization_gate::append_feedback_origin_noninterference_pipeline_proof_section(
                out,
                &feedback_origin_noninterference_pipeline_audit,
            );
        },
    )?;
    text.project_section(
        "identity_gate_trust_anchor",
        "IDENTITY-GATE-TRUST-ANCHOR",
        identity_gate_trust_anchor::append_identity_gate_trust_anchor_section,
    )?;
    text.project_owned_section(
        "productive_write_path_inventory",
        "PRODUCTIVE-WRITE-PATH-INVENTORY",
        "---------------------------------",
        productive_write_path_inventory::append_productive_write_path_inventory_section,
    )?;
    text.project_section(
        "key_signature_enharmonic_spelling_render_write",
        "TONAL-KEY-SIGNATURE-FUNCTIONAL-RESPELLING-CANONICAL-MASTER-MIRROR",
        |out| {
            append_key_signature_enharmonic_spelling_render_write_section(
                out,
                &key_signature_enharmonic_spelling_render_write_audit,
            );
        },
    )?;
    text.project_section(
        "tonal_segmentation_evidence_audit",
        "TONAL-SEGMENTATION-EVIDENCE-AUDIT-NO-MUTATION",
        |out| append_tonal_segmentation_evidence_audit_no_mutation_section(out, final_render_snapshot_xml),
    )?;
    text.project_section(
        "behind_bars_rule_class_closure",
        "BEHIND-BARS-RULE-CLASS-CLOSURE-NO-MUTATION",
        append_behind_bars_rule_class_closure_no_mutation_section,
    )?;
    text.project_section(
        "external_feedback_closure_render_write",
        "EXTERNAL-FEEDBACK-CLOSURE-RENDER-WRITE",
        |out| {
            append_external_feedback_closure_render_write_section(
                out,
                &external_feedback_closure_render_write_audit,
            );
        },
    )?;
    text.rewrite_document(|mut report_text| {
        normalize_current_facing_report_terms(&mut report_text);
        report_text
    });
    text.project_section(
        "clef_readability_primary_course_postwrite_audit",
        "CLEF-READABILITY-PRIMARY-COURSE-POSTWRITE-AUDIT",
        |out| {
            structural_write_authority::append_clef_readability_shadow_course_audit_section(
                out,
                &clef_readability_shadow_master_course_audit,
                clef_readability_shadow_render_artifact_course_audit.as_ref(),
            );
        },
    )?;
    // Nur die typisierte Materialisierung besitzt eine Ergebnisprojektion.
    text.project_section(
        "typed_lower_clef_surface_materialization",
        "TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION",
        |out| {
            append_typed_lower_clef_surface_materialization_result_section(
                out,
                &lower_clef_surface_materialization_audit,
            );
        },
    )?;
    text.project_section(
        "authorized_writer_entry_boundary_audit",
        "AUTHORIZED-WRITER-ENTRY-BOUNDARY-AUDIT",
        |out| {
            structural_write_authority::append_authorized_writer_entry_boundary_audit_section(
                out,
                &authorized_writer_entry_boundary_audit,
            );
        },
    )?;
    text.project_section(
        "render_write_pdf_source_adoption_truth_gate",
        "RENDER-WRITE-PDF-SOURCE-ADOPTION-TRUTH-GATE",
        |out| {
            append_render_write_pdf_source_adoption_truth_gate_section(
                out,
                &render_write_pdf_source_adoption_truth_gate_audit,
            );
        },
    )?;
    text.project_section(
        "render_candidate_and_selected_source_truth_projection_closure",
        "RENDER-CANDIDATE-AND-SELECTED-SOURCE-TRUTH-PROJECTION-CLOSURE",
        |out| {
            append_render_candidate_and_selected_source_truth_projection_closure(
                out,
                &render_write_pdf_source_adoption_truth_gate_audit,
                &raw_cursor_invariant_hard_gate,
                &final_serialized_musicxml_temporal_artifact_observation,
            );
        },
    )?;
    text.rewrite_document(crate::report_sanitizer::sanitize_behind_bars_active_authority_language_v1284);
    let report_snapshot = text.render_snapshot();
    let active_legacy_authority_markers =
        crate::report_sanitizer::active_legacy_authority_markers_v1284(&report_snapshot);
    if !active_legacy_authority_markers.is_empty() {
        return Err(anyhow!(
            "Aktiver menschlicher Sichtpruefungs-/Reviewvertrag nach Behind-Bars-Sanitizer verblieben: {}",
            active_legacy_authority_markers.join(", ")
        ));
    }
    text.project_owned_section(
        "behind_bars_pdf_readback_authority_truth",
        "BEHIND-BARS-PDF-READBACK-AUTHORITY-TRUTH",
        "---------------------------------------------",
        crate::report_sanitizer::append_behind_bars_pdf_readback_authority_truth_section_v1284,
    )?;
    // Re-attach the projection-only silent-voice outcome before the final locator sanitization.
    text.project_owned_section(
        "silent_voice_coverage_write_outcome",
        "SILENT-VOICE-COVERAGE-WRITE-OUTCOME",
        "-----------------------------------",
        |out| {
            append_silent_voice_coverage_write_outcome_projection(
                out,
                silent_voice_coverage_projection_message.as_deref(),
            );
        },
    )?;
    let original_beam_diagnostics_body =
        crate::audit_only::original_beam_diagnostics::report_body();
    // One formatted diagnostic emission; the per-project result is the durable copy.
    eprint!("{original_beam_diagnostics_body}");
    text.project_owned_section(
        "original_beam_diagnostics",
        "ORIGINAL-BEAM-DIAGNOSTICS",
        "-------------------------",
        |out| out.push_str(&original_beam_diagnostics_body),
    )?;
    let primary_beam_writer_delta_body =
        crate::audit_only::writer_trace::primary_beam_writer_delta_report_body();
    text.project_owned_section(
        "canonical_primary_beam_render_writer_delta_trace",
        "CANONICAL-PRIMARY-BEAM-RENDER-WRITER-DELTA-TRACE",
        "------------------------------------------------",
        |out| out.push_str(&primary_beam_writer_delta_body),
    )?;
    let render_epoch_continuity_body =
        crate::audit_only::writer_trace::render_epoch_continuity_report_body();
    text.project_owned_section(
        "render_epoch_continuity",
        "RENDER-EPOCH-CONTINUITY",
        "-----------------------",
        |out| out.push_str(&render_epoch_continuity_body),
    )?;
    if let Some(writer_trace_body) = crate::audit_only::writer_trace::report_body() {
        text.project_owned_section(
            "writer_trace_optional",
            "WRITER-TRACE-OPTIONAL",
            "=====================",
            |out| out.push_str(&writer_trace_body),
        )?;
    }
    let page_count_preference_shadow_line = {
        let capture = CONSOLE_CAPTURE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        crate::audit_only::page_count_preference_projection::current_process_line(&capture)?
    };
    if let Some(shadow_line) = page_count_preference_shadow_line {
        text.project_owned_section(
            "page_count_preference_shadow_outcome_projection",
            crate::audit_only::page_count_preference_projection::SECTION,
            crate::audit_only::page_count_preference_projection::SECTION_RULE,
            |out| {
                out.push_str("EvidenceOrigin: current_process_console_capture\n");
                out.push_str("PolicyWrite: nein\n");
                out.push_str(&shadow_line);
                out.push('\n');
                out.push_str("ProjectionStatus: persisted_read_only_evidence\n");
            },
        )?;
    }
    let (
        frozen_sounding_voice_binding_complete,
        frozen_sounding_unbound_members,
        frozen_sounding_voice_spans,
    ) = {
        let mut spans = Vec::<(usize, u8, i64, i64, String)>::new();
        let mut unbound_members = 0usize;
        for row in frozen_sounding_temporal_lane_plan.rows.iter() {
            let mut labels = std::collections::BTreeSet::<String>::new();
            for member in row.members.iter() {
                if let Some(label) = committed_voice_plan
                    .as_ref()
                    .and_then(|plan| plan.label_for_source(member.source_event_id.0))
                {
                    labels.insert(label.to_string());
                } else {
                    unbound_members = unbound_members.saturating_add(1);
                }
            }
            let first_measure = row
                .absolute_start
                .div_euclid(FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS);
            let last_measure = row
                .absolute_end
                .saturating_sub(1)
                .div_euclid(FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS);
            for measure_index in first_measure..=last_measure {
                let measure_base = measure_index
                    .saturating_mul(FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS);
                let local_start = row
                    .absolute_start
                    .max(measure_base)
                    .saturating_sub(measure_base);
                let local_end = row
                    .absolute_end
                    .min(measure_base.saturating_add(FROZEN_SOUNDING_TEMPORAL_MEASURE_DIVISIONS))
                    .saturating_sub(measure_base);
                let Ok(measure_number) = usize::try_from(measure_index.saturating_add(1)) else {
                    continue;
                };
                for label in &labels {
                    spans.push((measure_number, row.staff, local_start, local_end, label.clone()));
                }
            }
        }
        spans.sort();
        spans.dedup();
        (
            frozen_sounding_temporal_lane_plan.authority_bound
                && committed_voice_plan.as_ref().is_some_and(|plan| plan.is_bound())
                && unbound_members == 0,
            unbound_members,
            spans,
        )
    };
    text.project_owned_section(
        "staff_local_rest_ownership_audit",
        crate::audit_only::staff_local_rest_ownership_audit::SECTION,
        crate::audit_only::staff_local_rest_ownership_audit::SECTION_RULE,
        |out| {
            crate::audit_only::staff_local_rest_ownership_audit::
                append_staff_local_rest_ownership_audit_section(
                    out,
                    best.layout_xml(),
                    final_render_snapshot_xml,
                    frozen_sounding_voice_binding_complete,
                    frozen_sounding_unbound_members,
                    &frozen_sounding_voice_spans,
                );
        },
    )?;
    text.project_owned_section(
        "final_notation_voice_beam_ownership_audit",
        crate::audit_only::final_notation_voice_beam_ownership_audit::SECTION,
        crate::audit_only::final_notation_voice_beam_ownership_audit::SECTION_RULE,
        |out| {
            crate::audit_only::final_notation_voice_beam_ownership_audit::
                append_final_notation_voice_beam_ownership_audit_section(
                    out,
                    best.layout_xml(),
                    final_render_snapshot_xml,
                );
        },
    )?;
    text.project_owned_section(
        "notation_voice_shadow_constraint_audit",
        crate::audit_only::final_notation_voice_beam_ownership_audit::SHADOW_SECTION,
        crate::audit_only::final_notation_voice_beam_ownership_audit::SHADOW_SECTION_RULE,
        |out| {
            crate::audit_only::final_notation_voice_beam_ownership_audit::
                append_notation_voice_shadow_constraint_audit_section(
                    out,
                    best.layout_xml(),
                    final_render_snapshot_xml,
                );
        },
    )?;
    text.project_owned_section(
        "notation_voice_pre_serialization_constraint_closure_audit",
        crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::SECTION,
        crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::SECTION_RULE,
        |out| notation_voice_pre_serialization_constraint_audit.append_body(out),
    )?;
    text.project_owned_section(
        "visible_rhythmic_occupancy_phase_delta_audit",
        crate::audit_only::visible_rhythmic_occupancy_phase_delta_audit::SECTION,
        crate::audit_only::visible_rhythmic_occupancy_phase_delta_audit::SECTION_RULE,
        |out| {
            crate::audit_only::visible_rhythmic_occupancy_phase_delta_audit::
                append_visible_rhythmic_occupancy_phase_delta_audit_section(
                    out,
                    &notation_voice_pre_serialization_constraint_audit,
                    notation_voice_causal_lane_pressure_audit.occurrence_identity_rows(),
                    final_render_snapshot_xml,
                    &source.pdf_out,
                );
        },
    )?;
    text.project_owned_section(
        "notation_voice_causal_lane_pressure_audit",
        crate::audit_only::notation_voice_causal_lane_pressure_audit::SECTION,
        crate::audit_only::notation_voice_causal_lane_pressure_audit::SECTION_RULE,
        |out| notation_voice_causal_lane_pressure_audit.append_body(out),
    )?;
    text.project_owned_section(
        "repair_loop_invocation_evidence",
        "REPAIR-LOOP-INVOCATION-EVIDENCE",
        "--------------------------------",
        |out| {
            append_repair_loop_invocation_evidence_body(
                out,
                &repair_loop_invocation_evidence,
                &strict_staff_local_release_completeness_trial,
            );
        },
    )?;
    text.project_owned_section(
        "mutation_semantics_authority",
        "MUTATION-SEMANTICS-AUTHORITY",
        "----------------------------",
        |out| {
            report::append_mutation_semantics_authority_body(out);
        },
    )?;
    text.rewrite_document(report::clarify_legacy_music_mutation_report);
    text.rewrite_document(report::sanitize_measure_locator_authority_report);
    text.rewrite_document(report::sanitize_final_locator_suffixes);
    let finalized_report = text.finalize()?;
    let measurement_snapshot_path =
        crate::audit_only::canonical_measurement_snapshot::write_project_snapshot(
            source,
            &best.final_result,
            midi_projection.event_count,
            &best.midi_seed_stats.source_selection_full_causal,
        )?;
    console.push_str("\nCanonicalMeasurementSnapshotV1: ");
    console.push_str(&measurement_snapshot_path.display().to_string());
    (finalized_report, console, final_release_blocked)
}

