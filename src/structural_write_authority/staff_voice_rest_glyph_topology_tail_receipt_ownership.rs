// Canonical structural mutation ownership.
// Included in the pipeline lexical namespace to retain private selection facts.
// The complete preparation and mutation implementations live here; no forwarding facade.

fn apply_final_visible_rest_role_purge_and_vertical_placement_write_with_receipts(xml: &str, source_label: &str) -> (String, UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixAudit, crate::master_state_identity::RenderTransitionReceipts) {
    // v0.10.383: fix the v0.10.381/382 no-candidate classifier path.
    // The previous implementation used collect_render_timeline_fragments_in_segment() over a full
    // measure. That scanner is useful for contiguous local segments, but it can miss MusicXML lanes
    // after backup/forward interleaving and therefore classified almost every visible rest as
    // REAL_HAND_REST.  The role purge now uses the same atomic note onset model as the earlier
    // RestStack readback.  This keeps the classifier and the proof ledger on the same data basis.
    let mut audit = UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixAudit {
        source_label: source_label.to_string(),
        candidate_rewrite_path_enabled: true,
        ..UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixAudit::default()
    };

    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let before_signature = render_visible_sounding_signature(&before_events);
    audit.visible_rests_before = before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    audit.visible_pitched_before = before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    audit.hidden_pitched_before = before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    audit.hidden_rests_before = before_events.iter().filter(|event| event.is_hidden && event.is_rest).count();

    let notes = parse_atomic_notes(xml);
    let mut scope_set = BTreeSet::new();
    for note in notes.iter().filter(|note| note.visible) {
        scope_set.insert(note.measure.clone());
    }

    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut real_hand_rest_rows = 0usize;
    let mut independent_countervoice_rows = 0usize;
    let mut mixed_review_rows = 0usize;

    for rest in notes.iter().filter(|note| note.visible && note.pitch.is_none() && note.duration > 0 && !note.is_chord) {
        let role_observation = observe_atomic_visible_rest_role(rest, &notes);

        if role_observation.pitched_overlap_count == 0 {
            if role_observation.rest_overlap_count == 0 {
                real_hand_rest_rows = real_hand_rest_rows.saturating_add(1);
            } else {
                mixed_review_rows = mixed_review_rows.saturating_add(1);
            }
            continue;
        }

        if rest.voice.trim() == "1" {
            audit.keep_voice1_rows = audit.keep_voice1_rows.saturating_add(1);
            independent_countervoice_rows = independent_countervoice_rows.saturating_add(1);
            continue;
        }

        let is_shadow_candidate =
            atomic_visible_rest_is_shadow_bookkeeping_candidate(rest, role_observation);
        if !is_shadow_candidate {
            audit.protected_only_rows = audit.protected_only_rows.saturating_add(1);
            mixed_review_rows = mixed_review_rows.saturating_add(1);
            continue;
        }
        if occupied.iter().any(|(used_start, used_end)| rest.note_start < *used_end && rest.note_end > *used_start) {
            continue;
        }

        // r1118: once this canonical role owner has positively classified a rest as
        // AUXILIARY_SHADOW_BOOKKEEPING_REST_LANE, the retired visible carrier must
        // become identity-neutral cursor time. Keeping voice/staff on the forward
        // would let the later SilentVoice coverage planner reconstruct the technical
        // lane as a new visible silent voice.
        let replacement = render_forward_fragment(rest.duration as i32);
        replacements.push((rest.note_start, rest.note_end, replacement));
        occupied.push((rest.note_start, rest.note_end));
        audit.forward_duration = audit.forward_duration.saturating_add(rest.duration.max(0) as i32);
        if role_observation.has_real_voice1_rest_overlap {
            audit.split_guard_rows = audit.split_guard_rows.saturating_add(1);
        } else {
            audit.full_forward_rows = audit.full_forward_rows.saturating_add(1);
        }
        audit.rows.push(UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixRow {
            staff: rest.staff.clone(),
            voice: rest.voice.clone(),
            onset: rest.start,
            duration: rest.duration,
            pitched_overlap_count: role_observation.pitched_overlap_count,
            same_voice_pitched_overlap_count: role_observation.same_voice_pitched_overlap_count,
            pitched_overlap_coverage: role_observation.pitched_overlap_coverage,
            pitched_overlap_union_coverage: role_observation.pitched_overlap_union_coverage,
            pitched_geometry_union_coverage: role_observation.pitched_geometry_union_coverage,
            pitched_overlap_multiplicity_peak: role_observation.pitched_overlap_multiplicity_peak,
            has_real_voice1_rest_overlap: role_observation.has_real_voice1_rest_overlap,
            role_decision: "AUXILIARY_SHADOW_BOOKKEEPING_REST_LANE".to_string(),
        });
    }


    let (candidate_xml, receipt) = crate::structural_write_authority::apply_final_visible_rest_role_rewrite_authorized(
        xml,
        replacements,
    );
    let candidate_xml = candidate_xml.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);

    audit.scopes_checked = scope_set.len();
    audit.candidate_rows = audit.rows.iter().filter(|row| row.role_decision.contains("AUXILIARY_SHADOW_BOOKKEEPING")).count();
    audit.real_hand_rest_rows = real_hand_rest_rows;
    audit.independent_countervoice_rows = independent_countervoice_rows;
    audit.mixed_review_rows = mixed_review_rows;
    audit.non_focus_shadow_rows = audit.rows.iter().filter(|row| row.staff != "1" && row.role_decision.contains("AUXILIARY_SHADOW")).count();
    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    let after_signature = render_visible_sounding_signature(&after_events);
    audit.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    audit.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    audit.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    audit.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    audit.sounding_stable = before_signature == after_signature;
    audit.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    audit.backup_forward_after_ok = before_backup_forward_ok && after_backup_forward_ok;
    audit.raw_cursor_after_ok = scan_raw_cursor_xml("RenderXML", &candidate_xml)
        .into_iter()
        .all(|row| !row.hard_violation && row.partial_backup_count == 0 && row.post_expected_events == 0);

    let changed = candidate_xml != xml;
    let invariants_ok = changed
        && audit.candidate_rewrite_path_enabled
        && audit.sounding_stable
        && audit.parse_after_ok
        && audit.backup_forward_after_ok
        && audit.raw_cursor_after_ok
        && audit.visible_pitched_before == audit.visible_pitched_after
        && audit.hidden_pitched_before == audit.hidden_pitched_after
        && audit.visible_rests_after <= audit.visible_rests_before;
    audit.written = invariants_ok;
    audit.verdict = if audit.written {
        "whole_rest_display_coordinate_readback_truth_green_render_xml_only".to_string()
    } else if !changed {
        "whole_rest_display_coordinate_readback_truth_no_candidate".to_string()
    } else {
        "atomic_visible_rest_role_classifier_fix_rejected_by_invariant_guard".to_string()
    };
    audit.done_fragment = format!(
        "WholeRestDisplayCoordinateReadbackTruthFix: echte Handpausen {}, echte Gegenstimmen {}, Schatten-/Hilfspausen {}, MixedReview {}, sichtbare Pausen {}→{}, KlangMutation nein",
        audit.real_hand_rest_rows,
        audit.independent_countervoice_rows,
        audit.candidate_rows,
        audit.mixed_review_rows,
        audit.visible_rests_before,
        audit.visible_rests_after
    );
    let candidate_evidence = audit
        .rows
        .iter()
        .map(|row| {
            format!(
                "staff {} voice {} onset {} duration {} pitched_overlap_count {} same_voice_pitched_overlap_count {} pitched_overlap_coverage {} pitched_overlap_union_coverage {} pitched_geometry_union_coverage {} overlap_multiplicity_peak {} real_voice1_rest_overlap {} role {}",
                row.staff,
                row.voice,
                row.onset,
                row.duration,
                row.pitched_overlap_count,
                row.same_voice_pitched_overlap_count,
                row.pitched_overlap_coverage,
                row.pitched_overlap_union_coverage,
                row.pitched_geometry_union_coverage,
                row.pitched_overlap_multiplicity_peak,
                yes_no(row.has_real_voice1_rest_overlap),
                row.role_decision,
            )
        })
        .collect::<Vec<_>>()
        .join(" | ");
    audit.pipeline_line = format!(
        "WholeRestDisplayCoordinateReadbackTruthFix v{}: Source {}; ScopesChecked {}; RoleLedger REAL_HAND_REST {}; INDEPENDENT_COUNTERVOICE {}; AUX_SHADOW_BOOKKEEPING {}; MIXED_REVIEW_PROTECTED {}; FullForwardRows {}; SplitGuardRows {}; RealVoice1KeepRows {}; ForwardDuration {}; CandidateEvidence [{}]; visible_rests {}→{}; visible_pitched {}→{}; hidden_pitched {}→{}; hidden_rests {}→{}; ClassifierBasis atomic_note_onset; SoundingStable {}; parse/back-forward/raw-cursor {}/{}/{}; CandidateRewritePathEnabled yes; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite source-selected; MasterApply nein; P3P4 nein; MXL nein; MusicMutation nein; NotationRuleChange ja; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.scopes_checked,
        audit.real_hand_rest_rows,
        audit.independent_countervoice_rows,
        audit.candidate_rows,
        audit.mixed_review_rows,
        audit.full_forward_rows,
        audit.split_guard_rows,
        audit.keep_voice1_rows,
        audit.forward_duration,
        candidate_evidence,
        audit.visible_rests_before,
        audit.visible_rests_after,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.hidden_pitched_before,
        audit.hidden_pitched_after,
        audit.hidden_rests_before,
        audit.hidden_rests_after,
        yes_no(audit.sounding_stable),
        if audit.parse_after_ok { "ok" } else { "fail" },
        if audit.backup_forward_after_ok { "ok" } else { "fail" },
        if audit.raw_cursor_after_ok { "ok" } else { "fail" },
        yes_no(audit.written),
        audit.verdict
    );

    if audit.written { (candidate_xml, audit, receipts) } else {
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        (xml.to_string(), audit, crate::master_state_identity::RenderTransitionReceipts::new())
    }
}
