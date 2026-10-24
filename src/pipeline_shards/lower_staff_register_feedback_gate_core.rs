fn build_lower_staff_visible_rhythm_rest_lane_trial(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(
    UpperStaffLocalVisualRestLedgerProofTrial,
    Option<String>,
    crate::master_state_identity::RenderTransitionReceipts,
)> {
    let _ = output_path;
    let mut accepted_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut all_pitched_timeline_before = before_events
        .iter()
        .filter_map(|event| {
            event.midi.map(|midi| {
                (
                    event.scope,
                    event.staff.clone(),
                    event.voice.clone(),
                    event.onset,
                    event.duration,
                    event.is_hidden,
                    event.is_chord,
                    midi,
                )
            })
        })
        .collect::<Vec<_>>();
    all_pitched_timeline_before.sort();
    let mut trial = UpperStaffLocalVisualRestLedgerProofTrial {
        source_label: source_label.to_string(),
        parse_before_ok: parse_musicxml(xml).is_ok(),
        backup_forward_before_ok: before_backup_forward_ok,
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        upper_visual_local_ledger_before: upper_staff_local_visual_rest_ledger(xml, 12),
        ..UpperStaffLocalVisualRestLedgerProofTrial::default()
    };
    let (visual_collision_before, visual_collision_scopes_before) = count_lower_staff_visible_rest_collisions(xml);
    trial.visual_rest_collision_before = visual_collision_before;
    let mut candidate_xml = xml.to_string();
    if trial.parse_before_ok && trial.backup_forward_before_ok {
        trial.attempted = true;
        trial.candidates_seen = 0;
        trial.candidate_attempted = 0;

        let (
            dedup_xml,
            dedup_seen,
            dedup_suppressed,
            dedup_duration,
            dedup_scopes,
            dedup_chosen_evidence,
        ) = apply_lower_staff_overlapping_visible_rest_suppression(&candidate_xml, 16);
        trial.duplicate_rest_candidates_seen = dedup_seen;
        trial.duplicate_rest_suppressed = dedup_suppressed;
        trial.duplicate_rest_duration_to_forward = dedup_duration;
        trial.duplicate_rest_scopes = dedup_scopes;
        trial.duplicate_rest_chosen_evidence = dedup_chosen_evidence;
        if dedup_suppressed > 0 {
            candidate_xml = dedup_xml;
        }

        let ((
            anchor_xml,
            anchor_seen,
            anchor_suppressed,
            anchor_duration,
            anchor_scopes,
            anchor_chosen_evidence,
        ), anchor_receipts) = apply_lower_staff_visible_rest_lane_anchor_suppression(&candidate_xml, 24);
        trial.visible_rest_lane_anchor_candidates_seen = anchor_seen;
        trial.visible_rest_lane_anchor_suppressed = anchor_suppressed;
        trial.visible_rest_lane_anchor_duration_to_forward = anchor_duration;
        trial.visible_rest_lane_anchor_scopes = anchor_scopes;
        trial.visible_rest_lane_anchor_chosen_evidence = anchor_chosen_evidence;
        if anchor_suppressed > 0 {
            candidate_xml = anchor_xml;
            accepted_receipts.append(anchor_receipts);
        } else {
            anchor_receipts.discard_unadopted(
                crate::master_state_identity::RenderDiscardReason::LosingCandidate,
            );
        }

        let ((
            upper_xml,
            upper_initial_seen,
            upper_initial_written,
            upper_initial_scopes,
            upper_initial_duration,
            upper_coalesce_seen,
            upper_coalesced,
            upper_coalesce_scopes,
            upper_coalesce_duration,
            upper_cross_beat_blocked,
            upper_cross_beat_scopes,
        ), upper_receipts) = match apply_upper_staff_initial_rest_and_beat_local_coalescing(&candidate_xml, 8) {
            Ok(prepared) => prepared,
            Err(error) => {
                accepted_receipts.discard_unadopted(
                    crate::master_state_identity::RenderDiscardReason::LosingCandidate,
                );
                return Err(error);
            }
        };
        trial.upper_visual_initial_rest_candidates_seen = upper_initial_seen;
        trial.upper_visual_initial_rest_materialized = upper_initial_written;
        trial.upper_visual_initial_rest_scopes = upper_initial_scopes;
        trial.upper_visual_initial_rest_duration = upper_initial_duration;
        trial.upper_visual_rest_coalescing_candidates_seen = upper_coalesce_seen;
        trial.upper_visual_rest_coalesced = upper_coalesced;
        trial.upper_visual_rest_coalescing_scopes = upper_coalesce_scopes;
        trial.upper_visual_rest_coalescing_duration = upper_coalesce_duration;
        trial.upper_rest_cross_beat_blocked = upper_cross_beat_blocked;
        trial.upper_rest_cross_beat_scopes = upper_cross_beat_scopes;
        trial.upper_beat_boundary_invariant = true;
        if upper_initial_written > 0 || upper_coalesced > 0 {
            candidate_xml = upper_xml;
            accepted_receipts.append(upper_receipts);
        } else {
            upper_receipts.discard_unadopted(
                crate::master_state_identity::RenderDiscardReason::LosingCandidate,
            );
        }

        let ((upper_dedup_xml, upper_dup_seen, upper_dup_suppressed, upper_dup_duration, upper_dup_scopes, upper_dup_interval_proof), upper_dedup_receipts) =
            apply_upper_staff_duplicate_visible_rest_suppression_with_receipts(&candidate_xml, 16);
        trial.upper_visual_duplicate_rest_candidates_seen = upper_dup_seen;
        trial.upper_visual_duplicate_rest_suppressed = upper_dup_suppressed;
        trial.upper_visual_duplicate_rest_duration_to_forward = upper_dup_duration;
        trial.upper_visual_duplicate_rest_scopes = upper_dup_scopes;
        trial.upper_visual_duplicate_rest_interval_proof = upper_dup_interval_proof;
        if upper_dup_suppressed > 0 {
            candidate_xml = upper_dedup_xml;
            accepted_receipts.append(upper_dedup_receipts);
        } else {
            upper_dedup_receipts.discard_unadopted(
                crate::master_state_identity::RenderDiscardReason::LosingCandidate,
            );
        }
    }

    trial.upper_visual_local_ledger_after = upper_staff_local_visual_rest_ledger(&candidate_xml, 12);
    trial.candidate_attempted = trial.actions.len()
        + trial.duplicate_rest_suppressed
        + trial.visible_rest_lane_anchor_suppressed
        + trial.upper_visual_initial_rest_materialized
        + trial.upper_visual_rest_coalesced
        + trial.upper_visual_duplicate_rest_suppressed;

    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    let mut all_pitched_timeline_after = after_events
        .iter()
        .filter_map(|event| {
            event.midi.map(|midi| {
                (
                    event.scope,
                    event.staff.clone(),
                    event.voice.clone(),
                    event.onset,
                    event.duration,
                    event.is_hidden,
                    event.is_chord,
                    midi,
                )
            })
        })
        .collect::<Vec<_>>();
    all_pitched_timeline_after.sort();
    let all_pitched_timeline_stable = all_pitched_timeline_before == all_pitched_timeline_after;
    trial.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    trial.visible_rest_delta = trial.visible_rests_after as i64 - trial.visible_rests_before as i64;
    let (visual_collision_after, visual_collision_scopes_after) = count_lower_staff_visible_rest_collisions(&candidate_xml);
    trial.visual_rest_collision_after = visual_collision_after;
    trial.visual_rest_collision_green = trial.visual_rest_collision_before > trial.visual_rest_collision_after;
    trial.visual_rest_collision_proof = format!(
        "VisualRestCollision staff2 sichtbar {}→{}; before_scopes {}; after_scopes {}; LowerDuplicateRestSuppressed {}; LowerDuplicateRestChosenEvidence [{}]; LowerVisibleRestLaneAnchorSuppressed {}; UpperDuplicateRestSuppressed {}; UpperDuplicateRestScopes {}; UpperDuplicateRestIntervalProof {}; UpperLocalLedgerBefore [{}]; UpperLocalLedgerAfter [{}]; AllPitchedTimelineStable {}; Guard same-staff/rest-overlap, sichtbarer Rest-Lane-Anker oder identische obere sichtbare Rest-Dubletten; Render-XML-only, alle pitched Onsets/Dauern/Staff/Voice inkl. hidden stabil required",
        trial.visual_rest_collision_before,
        trial.visual_rest_collision_after,
        visual_collision_scopes_before,
        visual_collision_scopes_after,
        trial.duplicate_rest_suppressed,
        trial.duplicate_rest_chosen_evidence,
        trial.visible_rest_lane_anchor_suppressed,
        trial.upper_visual_duplicate_rest_suppressed,
        trial.upper_visual_duplicate_rest_scopes,
        trial.upper_visual_duplicate_rest_interval_proof,
        trial.upper_visual_local_ledger_before,
        trial.upper_visual_local_ledger_after,
        if all_pitched_timeline_stable { "ja" } else { "nein" }
    );

    if let Some(action) = trial.actions.first() {
        trial.selected_scope = action.scope;
        trial.selected_source_voice = action.source_voice.clone();
        trial.selected_rest_voice = action.rest_voice.clone();
        trial.lower_staff_notes = action.lower_staff_notes;
        trial.staff_switches = action.staff_switches;
        trial.rest_segments_written = action.rest_parts.split(';').filter(|part| !part.trim().is_empty() && *part != "-").count();
        trial.forward_segments_written = action.forward_parts.split(';').filter(|part| !part.trim().is_empty() && *part != "-").count();
        trial.rest_duration_written = action.rest_duration;
        trial.forward_duration_written = action.forward_duration;
    }

    let render_notation_delta_present = (trial.candidate_attempted > 0
        && trial.visible_rest_delta > 0
        && trial.rest_duration_written > 0
        && trial.rest_duration_written.saturating_add(trial.forward_duration_written) > 0)
        || (trial.duplicate_rest_suppressed > 0
            && trial.visible_rests_after <= trial.visible_rests_before
            && trial.visual_rest_collision_green)
        || (trial.visible_rest_lane_anchor_suppressed > 0
            && trial.visible_rests_after < trial.visible_rests_before)
        || (trial.upper_visual_initial_rest_materialized > 0
            && trial.visible_rests_after >= trial.visible_rests_before
            && trial.upper_visual_initial_rest_duration > 0)
        || (trial.upper_visual_rest_coalesced > 0
            && trial.visible_rests_after < trial.visible_rests_before
            && trial.upper_beat_boundary_invariant)
        || (trial.upper_visual_duplicate_rest_suppressed > 0
            && trial.visible_rests_after < trial.visible_rests_before
            && trial.upper_visual_duplicate_rest_duration_to_forward > 0);
    let invariant_ok = render_notation_delta_present
        && trial.parse_after_ok
        && trial.backup_forward_after_ok
        && trial.sounding_signature_stable
        && all_pitched_timeline_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.hidden_pitched_before == trial.hidden_pitched_after
        && trial.hidden_rests_before == trial.hidden_rests_after;

    if invariant_ok {
        // Pure preparation. The sovereign stage verifies the complete owner chain
        // before the single E022 transport sink is allowed to write.
        trial.written = true;
        trial.candidate_adopted = trial.candidate_attempted;
        trial.verdict = if trial.upper_visual_duplicate_rest_suppressed > 0
            || trial.upper_visual_initial_rest_materialized > 0
            || trial.upper_visual_rest_coalesced > 0 {
            "upper_staff_local_visual_rest_ledger_proof_written_render_xml_only".to_string()
        } else {
            "lower_staff_visual_rest_collision_proof_written_render_xml_only".to_string()
        };
    } else {
        trial.candidate_rejected = trial.candidate_attempted;
        trial.verdict = if trial.candidate_attempted == 0 {
            "lower_staff_visible_rhythm_no_safe_rest_lane_candidate".to_string()
        } else {
            "lower_staff_visual_rest_collision_proof_rejected_by_invariant_guard".to_string()
        };
    }

    trial.examples.push("Historische PDF-Evidenz: Unten wirkt die Zeitachse unvollständig; der Operator erzeugt daraus keine feste Taktlogik, sondern sucht generisch Cross-Staff-Voices mit sparse Staff-2-Noten und ohne sichtbare Staff-2-Pausenführung.".to_string());
    trial.examples.push("Die sichtbare Rest-Lane entsteht nur aus bereits vorhandener Forward-Zeit im Render-XML. Master-XML, klingende Noten, Pitch, Onset und Dauer bleiben unverändert.".to_string());
    trial.examples.push("Die Staff-2-Rests werden nur in die Lücken zwischen vorhandenen unteren Noten gesetzt; sehr kleine Restfragmente unter einer Achtel werden als Forward-Zeit beruhigt, damit die RestLane nicht kleinteilig wirkt.".to_string());
    trial.examples.push("Offensichtlich übereinander liegende untere Pausen werden generisch als überlappende Staff-2-Rest-Lanes erkannt und nur in Forward-Zeit überführt, wenn klingende Noten unverändert bleiben und backup/forward stabil bleibt.".to_string());
    trial.done_fragment = format!(
        "UpperStaffLocalVisualRestLedgerProof: gesehen {}, versucht {}, übernommen {}, Scope {}, Voice {}->{}, lower_notes {}, Restsegmente {}, Forwardsegmente {}, DuplicateRestCandidates {}, DuplicateRestSuppressed {}, DuplicateRestScopes {}, DuplicateRestChosenEvidence [{}], VisibleRestLaneAnchorCandidates {}, VisibleRestLaneAnchorSuppressed {}, VisibleRestLaneAnchorScopes {}, VisibleRestLaneAnchorChosenEvidence [{}], UpperVisualInitialRest {}/{} scopes {} duration {}, UpperVisualRestCoalescing {}/{} scopes {} duration {}, UpperVisualDuplicateRest {}/{} scopes {} duration {} proof {}, CrossBeatMergeBlocked {} scopes {}, BeatBoundaryInvariant {}, VisualRestCollision {}→{} green {}, sichtbare Pausen {}→{}, klingend stabil {}, geschrieben {}",
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
        trial.selected_scope,
        trial.selected_source_voice,
        trial.selected_rest_voice,
        trial.lower_staff_notes,
        trial.rest_segments_written,
        trial.forward_segments_written,
        trial.duplicate_rest_candidates_seen,
        trial.duplicate_rest_suppressed,
        trial.duplicate_rest_scopes,
        trial.duplicate_rest_chosen_evidence,
        trial.visible_rest_lane_anchor_candidates_seen,
        trial.visible_rest_lane_anchor_suppressed,
        trial.visible_rest_lane_anchor_scopes,
        trial.visible_rest_lane_anchor_chosen_evidence,
        trial.upper_visual_initial_rest_candidates_seen,
        trial.upper_visual_initial_rest_materialized,
        trial.upper_visual_initial_rest_scopes,
        trial.upper_visual_initial_rest_duration,
        trial.upper_visual_rest_coalescing_candidates_seen,
        trial.upper_visual_rest_coalesced,
        trial.upper_visual_rest_coalescing_scopes,
        trial.upper_visual_rest_coalescing_duration,
        trial.upper_visual_duplicate_rest_candidates_seen,
        trial.upper_visual_duplicate_rest_suppressed,
        trial.upper_visual_duplicate_rest_scopes,
        trial.upper_visual_duplicate_rest_duration_to_forward,
        trial.upper_visual_duplicate_rest_interval_proof,
        trial.upper_rest_cross_beat_blocked,
        trial.upper_rest_cross_beat_scopes,
        if trial.upper_beat_boundary_invariant { "ja" } else { "nein" },
        trial.visual_rest_collision_before,
        trial.visual_rest_collision_after,
        if trial.visual_rest_collision_green { "ja" } else { "nein" },
        trial.visible_rests_before,
        trial.visible_rests_after,
        if trial.sounding_signature_stable { "ja" } else { "nein" },
        if trial.written { "ja" } else { "nein" }
    );
    trial.pipeline_line = format!(
        "UpperStaffLocalVisualRestLedgerProof v{}: Render-XML-only auf {}; seen/attempted/adopted/rejected {}/{}/{}/{}; selected_scope {}; source_voice {}; rest_voice {}; lower_notes {}; rest_segments {}; forward_segments {}; rest_duration {}; forward_duration {}; duplicate_rest_candidates {}; duplicate_rest_suppressed {}; duplicate_rest_scopes {}; duplicate_rest_duration_to_forward {}; duplicate_rest_chosen_evidence [{}]; visible_rest_lane_anchor_candidates {}; visible_rest_lane_anchor_suppressed {}; visible_rest_lane_anchor_scopes {}; visible_rest_lane_anchor_duration_to_forward {}; visible_rest_lane_anchor_chosen_evidence [{}]; upper_visual_initial_rest_candidates {}; upper_visual_initial_rest_materialized {}; upper_visual_initial_rest_scopes {}; upper_visual_initial_rest_duration {}; upper_visual_rest_coalescing_candidates {}; upper_visual_rest_coalesced {}; upper_visual_rest_coalescing_scopes {}; upper_visual_rest_coalescing_duration {}; upper_visual_duplicate_rest_candidates {}; upper_visual_duplicate_rest_suppressed {}; upper_visual_duplicate_rest_scopes {}; upper_visual_duplicate_rest_duration_to_forward {}; upper_visual_duplicate_rest_interval_proof {}; upper_local_ledger_before [{}]; upper_local_ledger_after [{}]; cross_beat_merge_blocked {}; cross_beat_merge_scopes {}; beat_boundary_invariant {}; visual_rest_collision {}→{} green {}; proof {}; sichtbare Pausen {}→{}; sichtbare klingende Noten {}→{}; hidden klingend {}→{}; hidden Pausen {}→{}; sounding_signature {}/{} stabil {}; visible_rest_delta {}; parse/back-forward nachher {}/{}; geschrieben {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
        trial.candidate_rejected,
        trial.selected_scope,
        trial.selected_source_voice,
        trial.selected_rest_voice,
        trial.lower_staff_notes,
        trial.rest_segments_written,
        trial.forward_segments_written,
        trial.rest_duration_written,
        trial.forward_duration_written,
        trial.duplicate_rest_candidates_seen,
        trial.duplicate_rest_suppressed,
        trial.duplicate_rest_scopes,
        trial.duplicate_rest_duration_to_forward,
        trial.duplicate_rest_chosen_evidence,
        trial.visible_rest_lane_anchor_candidates_seen,
        trial.visible_rest_lane_anchor_suppressed,
        trial.visible_rest_lane_anchor_scopes,
        trial.visible_rest_lane_anchor_duration_to_forward,
        trial.visible_rest_lane_anchor_chosen_evidence,
        trial.upper_visual_initial_rest_candidates_seen,
        trial.upper_visual_initial_rest_materialized,
        trial.upper_visual_initial_rest_scopes,
        trial.upper_visual_initial_rest_duration,
        trial.upper_visual_rest_coalescing_candidates_seen,
        trial.upper_visual_rest_coalesced,
        trial.upper_visual_rest_coalescing_scopes,
        trial.upper_visual_rest_coalescing_duration,
        trial.upper_visual_duplicate_rest_candidates_seen,
        trial.upper_visual_duplicate_rest_suppressed,
        trial.upper_visual_duplicate_rest_scopes,
        trial.upper_visual_duplicate_rest_duration_to_forward,
        trial.upper_visual_duplicate_rest_interval_proof,
        trial.upper_visual_local_ledger_before,
        trial.upper_visual_local_ledger_after,
        trial.upper_rest_cross_beat_blocked,
        trial.upper_rest_cross_beat_scopes,
        if trial.upper_beat_boundary_invariant { "ja" } else { "nein" },
        trial.visual_rest_collision_before,
        trial.visual_rest_collision_after,
        if trial.visual_rest_collision_green { "ja" } else { "nein" },
        trial.visual_rest_collision_proof,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.hidden_pitched_before,
        trial.hidden_pitched_after,
        trial.hidden_rests_before,
        trial.hidden_rests_after,
        trial.sounding_signature_before,
        trial.sounding_signature_after,
        if trial.sounding_signature_stable { "ja" } else { "nein" },
        trial.visible_rest_delta,
        if trial.parse_after_ok { "ok" } else { "fail" },
        if trial.backup_forward_after_ok { "ok" } else { "fail" },
        if trial.written { "ja" } else { "nein" },
        trial.verdict
    );
    let candidate = if invariant_ok {
        Some(candidate_xml)
    } else {
        accepted_receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::LosingCandidate,
        );
        accepted_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        None
    };
    Ok((trial, candidate, accepted_receipts))
}



