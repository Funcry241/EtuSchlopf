






#[derive(Debug, Clone, Default)]
struct BehindBarsRuleCheckPitchSpan {
    scope: usize,
    staff: String,
    voice: String,
    onset: i32,
    duration: i32,
    midi: i32,
    tie_start: bool,
    tie_stop: bool,
}

fn collect_behind_bars_rule_check_pitch_spans(xml: &str) -> Vec<BehindBarsRuleCheckPitchSpan> {
    let mut spans = Vec::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let mut cursor = 0_i32;
        let mut local_scan = 0_usize;
        let mut last_anchor_onset = 0_i32;
        loop {
            let next_note = measure[local_scan..].find("<note").map(|p| local_scan + p);
            let next_backup = measure[local_scan..].find("<backup").map(|p| local_scan + p);
            let next_forward = measure[local_scan..].find("<forward").map(|p| local_scan + p);
            let next = [
                next_note.map(|pos| (pos, "note")),
                next_backup.map(|pos| (pos, "backup")),
                next_forward.map(|pos| (pos, "forward")),
            ]
            .into_iter()
            .flatten()
            .min_by_key(|(pos, _)| *pos);
            let Some((pos, kind)) = next else { break; };
            match kind {
                "note" => {
                    let Some(close_rel) = measure[pos..].find("</note>") else { break; };
                    let end = pos + close_rel + "</note>".len();
                    let note = &measure[pos..end];
                    let duration = extract_first_text_from_fragment(note, "duration")
                        .and_then(|value| value.trim().parse::<i32>().ok())
                        .unwrap_or(0);
                    let is_chord = note.contains("<chord") || note.contains("<chord/>");
                    let onset = if is_chord { last_anchor_onset } else { cursor };
                    if !is_hidden_note(note)
                        && !note.contains("<rest")
                        && !is_chord
                        && let Some(midi) = midi_from_note_fragment(note)
                    {
                        let staff = extract_first_text_from_fragment(note, "staff")
                            .unwrap_or_else(|| "?".to_string());
                        let voice = extract_first_text_from_fragment(note, "voice")
                            .unwrap_or_else(|| "?".to_string());
                        spans.push(BehindBarsRuleCheckPitchSpan {
                            scope: scope_number,
                            staff,
                            voice,
                            onset,
                            duration,
                            midi,
                            tie_start: note.contains("<tie type=\"start\"")
                                || note.contains("<tied type=\"start\""),
                            tie_stop: note.contains("<tie type=\"stop\"")
                                || note.contains("<tied type=\"stop\""),
                        });
                    }
                    if !is_chord {
                        last_anchor_onset = onset;
                        cursor = cursor.saturating_add(duration);
                    }
                    local_scan = end;
                }
                "backup" => {
                    let Some(close_rel) = measure[pos..].find("</backup>") else { break; };
                    let end = pos + close_rel + "</backup>".len();
                    let fragment = &measure[pos..end];
                    let duration = extract_first_text_from_fragment(fragment, "duration")
                        .and_then(|value| value.trim().parse::<i32>().ok())
                        .unwrap_or(0);
                    cursor = cursor.saturating_sub(duration).max(0);
                    last_anchor_onset = cursor;
                    local_scan = end;
                }
                "forward" => {
                    let Some(close_rel) = measure[pos..].find("</forward>") else { break; };
                    let end = pos + close_rel + "</forward>".len();
                    let fragment = &measure[pos..end];
                    let duration = extract_first_text_from_fragment(fragment, "duration")
                        .and_then(|value| value.trim().parse::<i32>().ok())
                        .unwrap_or(0);
                    cursor = cursor.saturating_add(duration);
                    last_anchor_onset = cursor;
                    local_scan = end;
                }
                _ => break,
            }
        }
        scan = measure_end;
    }
    spans
}

fn build_behind_bars_rule_check_same_pitch_rows(xml: &str) -> Vec<BehindBarsRuleCheckSamePitchRow> {
    let mut spans = collect_behind_bars_rule_check_pitch_spans(xml);
    spans.sort_by_key(|span| (span.scope, span.staff.clone(), span.voice.clone(), span.onset, span.duration));
    let mut rows = Vec::new();
    for pair in spans.windows(2) {
        let a = &pair[0];
        let b = &pair[1];
        if a.scope != b.scope || a.staff != b.staff || a.voice != b.voice || a.midi != b.midi {
            continue;
        }
        if a.onset.saturating_add(a.duration) != b.onset {
            continue;
        }
        let has_tie_boundary = a.tie_start || a.tie_stop || b.tie_start || b.tie_stop;
        let (classification, _recommendation) = if a.tie_start && b.tie_stop && !b.tie_start {
            (
                "same_pitch_tie_ends_before_immediate_reattack".to_string(),
                "prüfen_ob_midi_quelle_oder_wav_evidenz_neue_attacke_beweist_sonst_längere_note_oder_durchbindung".to_string(),
            )
        } else if has_tie_boundary {
            (
                "same_pitch_tie_boundary".to_string(),
                "tie_kette_auf_saubere_endpunkte_und_reartikulation_pruefen".to_string(),
            )
        } else {
            (
                "same_pitch_direct_reattack".to_string(),
                "nur_behalten_wenn_attack_oder_rollenwechsel_belegt".to_string(),
            )
        };
        rows.push(BehindBarsRuleCheckSamePitchRow {
            scope: a.scope,
            staff: a.staff.clone(),
            voice: a.voice.clone(),
            first_onset: a.onset,
            has_tie_boundary,
            classification,
        });
    }
    rows.sort_by(|a, b| {
        let arisk = if a.classification == "same_pitch_tie_ends_before_immediate_reattack" { 2 } else if a.has_tie_boundary { 1 } else { 0 };
        let brisk = if b.classification == "same_pitch_tie_ends_before_immediate_reattack" { 2 } else if b.has_tie_boundary { 1 } else { 0 };
        brisk.cmp(&arisk)
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
            .then_with(|| a.first_onset.cmp(&b.first_onset))
    });
    rows
}

fn build_behind_bars_rule_check_rest_and_tie_audit(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(
    BehindBarsRuleCheckRestAndTieAudit,
    Option<String>,
    crate::master_state_identity::RenderTransitionReceipts,
)> {
    let _ = output_path;
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut audit = BehindBarsRuleCheckRestAndTieAudit {
        source_label: source_label.to_string(),
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        backup_forward_before_ok: before_backup_forward_ok,
        parse_before_ok: parse_musicxml(xml).is_ok(),
        ..Default::default()
    };
    let mut accepted_receipts = crate::master_state_identity::RenderTransitionReceipts::new();

    // v0.10.1740 r01: BehindBarsRestAndTieAudit bleibt der einzige physische
    // Render-Writer, aber seine zwei semantisch verschiedenen Transaktionen
    // werden getrennt abgenommen. Legacy-Behind-Bars-Reparaturen duerfen eine
    // sichere Ganztaktpause nicht mehr blockieren und umgekehrt.
    let mut legacy_candidate_xml = xml.to_string();
    let mut actions = Vec::new();
    if audit.parse_before_ok && audit.backup_forward_before_ok {
        audit.attempted = true;
        let ((chain_xml, mut chain_actions, chain_seen), chain_receipts) =
            apply_behind_bars_rule_check_lower_staff_rest_compaction(&legacy_candidate_xml, 16)?;
        accepted_receipts.append(chain_receipts);
        audit.lower_staff_rest_chain_candidates_seen = chain_seen;
        audit.lower_staff_rest_chain_attempted = chain_actions.len();
        legacy_candidate_xml = chain_xml;
        actions.append(&mut chain_actions);

        let ((beat_xml, mut beat_actions, beat_seen), beat_receipts) =
            match apply_structural_rest_beat_materialization(&legacy_candidate_xml, 48) {
                Ok(prepared) => prepared,
                Err(error) => {
                    accepted_receipts.discard_unadopted(
                        crate::master_state_identity::RenderDiscardReason::LosingCandidate,
                    );
                    return Err(error);
                }
            };
        accepted_receipts.append(beat_receipts);
        audit.external_feedback_beat_rest_ledger_candidates_seen = beat_seen;
        audit.external_feedback_beat_rest_ledger_attempted = beat_actions.len();
        legacy_candidate_xml = beat_xml;
        actions.append(&mut beat_actions);

        let ((rewrite_xml, mut rewrite_actions, rewrite_seen), rewrite_receipts) =
            apply_beat_local_voice_rest_rewrite(&legacy_candidate_xml, 12);
        accepted_receipts.append(rewrite_receipts);
        audit.external_feedback_beat_rest_ledger_candidates_seen = audit
            .external_feedback_beat_rest_ledger_candidates_seen
            .saturating_add(rewrite_seen);
        audit.external_feedback_beat_rest_ledger_attempted = audit
            .external_feedback_beat_rest_ledger_attempted
            .saturating_add(rewrite_actions.len());
        legacy_candidate_xml = rewrite_xml;
        actions.append(&mut rewrite_actions);

        let ((suppressed_xml, mut suppress_actions, suppress_seen), suppress_receipts) =
            apply_behind_bars_rule_check_spurious_empty_voice_rest_suppression(
                &legacy_candidate_xml,
                8,
            );
        accepted_receipts.append(suppress_receipts);
        audit.spurious_empty_voice_rest_candidates_seen = suppress_seen;
        audit.spurious_empty_voice_rest_attempted = suppress_actions.len();
        legacy_candidate_xml = suppressed_xml;
        actions.append(&mut suppress_actions);

        let ((duplicate_xml, mut duplicate_actions, duplicate_seen), duplicate_receipts) =
            apply_duplicate_stem_same_pitch_render_suppression(&legacy_candidate_xml, 8);
        accepted_receipts.append(duplicate_receipts);
        audit.duplicate_stem_same_pitch_candidates = duplicate_seen;
        audit.duplicate_stem_same_pitch_attempted = duplicate_actions.len();
        legacy_candidate_xml = duplicate_xml;
        actions.append(&mut duplicate_actions);
    }

    let legacy_attempted_total = audit.lower_staff_rest_chain_attempted
        + audit.external_feedback_beat_rest_ledger_attempted
        + audit.spurious_empty_voice_rest_attempted
        + audit.cross_staff_voice_ledger_attempted
        + audit.duplicate_stem_same_pitch_attempted;
    let (legacy_after_events, legacy_backup_forward_after_ok) =
        collect_render_xml_note_events(&legacy_candidate_xml);
    let legacy_sounding_stable = audit.sounding_signature_before
        == render_visible_sounding_signature(&legacy_after_events);
    let legacy_dedup_sounding_stable = render_visible_sounding_signature_dedup(&before_events)
        == render_visible_sounding_signature_dedup(&legacy_after_events);
    let legacy_visible_pitched_after = legacy_after_events
        .iter()
        .filter(|event| !event.is_hidden && event.midi.is_some())
        .count();
    let legacy_hidden_pitched_after = legacy_after_events
        .iter()
        .filter(|event| event.is_hidden && event.midi.is_some())
        .count();
    let legacy_hidden_rests_after = legacy_after_events
        .iter()
        .filter(|event| event.is_hidden && event.is_rest)
        .count();
    let legacy_pitched_invariant_ok = if audit.duplicate_stem_same_pitch_attempted > 0 {
        audit.visible_pitched_before
            == legacy_visible_pitched_after
                .saturating_add(audit.duplicate_stem_same_pitch_attempted)
            && legacy_hidden_pitched_after
                == audit
                    .hidden_pitched_before
                    .saturating_add(audit.duplicate_stem_same_pitch_attempted)
            && legacy_dedup_sounding_stable
    } else {
        legacy_sounding_stable
            && audit.visible_pitched_before == legacy_visible_pitched_after
            && audit.hidden_pitched_before == legacy_hidden_pitched_after
    };
    let legacy_hidden_rest_visibility_rewrites = actions
        .iter()
        .filter(|action| {
            action.action_kind == "beat_local_voice_rest_rewrite_hidden_rest_visibility"
        })
        .count();
    let legacy_hidden_rest_invariant_ok = audit.hidden_rests_before == legacy_hidden_rests_after
        || (legacy_hidden_rest_visibility_rewrites > 0
            && audit.hidden_rests_before
                == legacy_hidden_rests_after
                    .saturating_add(legacy_hidden_rest_visibility_rewrites));
    let legacy_transaction_adopted = legacy_attempted_total > 0
        && legacy_candidate_xml.as_str() != xml
        && legacy_pitched_invariant_ok
        && legacy_hidden_rest_invariant_ok
        && parse_musicxml(&legacy_candidate_xml).is_ok()
        && legacy_backup_forward_after_ok;

    if !legacy_transaction_adopted {
        accepted_receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::LosingCandidate,
        );
        accepted_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    }
    let mut candidate_xml = if legacy_transaction_adopted {
        legacy_candidate_xml
    } else {
        xml.to_string()
    };

    // Rest-only-/Ganztakt-Transaktion: dieselbe vorhandene Rest-Authority und
    // derselbe physische Behind-Bars-Writer, aber eine eigene lokale Abnahme.
    // Dadurch kann eine vollstaendige stille Lane die Render-Quelle legitimieren,
    // selbst wenn eine andere Behind-Bars-Reparatur verworfen wurde.
    let (measure_rest_outcome, rest_only_receipts) =
        if audit.parse_before_ok && audit.backup_forward_before_ok {
            match apply_rest_only_staff_empty_measure_readability(&candidate_xml, 64) {
                Ok(prepared) => prepared,
                Err(error) => {
                    accepted_receipts.discard_unadopted(
                        crate::master_state_identity::RenderDiscardReason::LosingCandidate,
                    );
                    return Err(error);
                }
            }
        } else {
            (
                Default::default(),
                crate::master_state_identity::RenderTransitionReceipts::new(),
            )
        };
    audit.complete_measure_rest_candidates_seen = measure_rest_outcome.measure_rest_candidates_seen;
    audit.complete_measure_rest_attempted = measure_rest_outcome.measure_rest_attempted;
    audit.rest_only_auxiliary_lane_candidates_seen = measure_rest_outcome.auxiliary_lane_candidates_seen;
    audit.rest_only_auxiliary_lane_attempted = measure_rest_outcome.auxiliary_lane_attempted;

    let rest_only_attempted_total = audit.complete_measure_rest_attempted
        + audit.rest_only_auxiliary_lane_attempted;
    let rest_base_xml = candidate_xml.clone();
    let (rest_before_events, rest_backup_forward_before_ok) =
        collect_render_xml_note_events(&rest_base_xml);
    let (rest_after_events, rest_backup_forward_after_ok) =
        collect_render_xml_note_events(&measure_rest_outcome.xml);
    let rest_before_sounding = render_visible_sounding_signature(&rest_before_events);
    let rest_after_sounding = render_visible_sounding_signature(&rest_after_events);
    let rest_visible_pitched_before = rest_before_events
        .iter()
        .filter(|event| !event.is_hidden && event.midi.is_some())
        .count();
    let rest_visible_pitched_after = rest_after_events
        .iter()
        .filter(|event| !event.is_hidden && event.midi.is_some())
        .count();
    let rest_hidden_pitched_before = rest_before_events
        .iter()
        .filter(|event| event.is_hidden && event.midi.is_some())
        .count();
    let rest_hidden_pitched_after = rest_after_events
        .iter()
        .filter(|event| event.is_hidden && event.midi.is_some())
        .count();
    let rest_visible_rests_before = rest_before_events
        .iter()
        .filter(|event| !event.is_hidden && event.is_rest)
        .count();
    let rest_visible_rests_after = rest_after_events
        .iter()
        .filter(|event| !event.is_hidden && event.is_rest)
        .count();
    let rest_hidden_rests_before = rest_before_events
        .iter()
        .filter(|event| event.is_hidden && event.is_rest)
        .count();
    let rest_hidden_rests_after = rest_after_events
        .iter()
        .filter(|event| event.is_hidden && event.is_rest)
        .count();
    let expected_visible_rest_reduction = measure_rest_outcome
        .visible_rests_consumed
        .saturating_sub(audit.complete_measure_rest_attempted);
    let rest_visible_rest_invariant_ok = rest_visible_rests_before
        == rest_visible_rests_after.saturating_add(expected_visible_rest_reduction);
    let rest_hidden_rest_invariant_ok = rest_hidden_rests_before
        == rest_hidden_rests_after.saturating_add(measure_rest_outcome.hidden_rests_consumed);
    let rest_only_transaction_adopted = rest_only_attempted_total > 0
        && !measure_rest_outcome.authority_rejected
        && measure_rest_outcome.xml != rest_base_xml
        && rest_backup_forward_before_ok
        && rest_backup_forward_after_ok
        && parse_musicxml(&measure_rest_outcome.xml).is_ok()
        && rest_before_sounding == rest_after_sounding
        && rest_visible_pitched_before == rest_visible_pitched_after
        && rest_hidden_pitched_before == rest_hidden_pitched_after
        && rest_visible_rest_invariant_ok
        && rest_hidden_rest_invariant_ok;
    if rest_only_transaction_adopted {
        candidate_xml = measure_rest_outcome.xml.clone();
        accepted_receipts.append(rest_only_receipts);
    } else {
        rest_only_receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::LosingCandidate,
        );
    }

    let same_pitch_rows = build_behind_bars_rule_check_same_pitch_rows(&candidate_xml);
    audit.same_pitch_reattack_candidates = same_pitch_rows.len();
    audit.same_pitch_tie_boundary_candidates = same_pitch_rows
        .iter()
        .filter(|row| row.has_tie_boundary)
        .count();
    audit.same_pitch_rows = same_pitch_rows.into_iter().take(12).collect();
    let duplicate_stem_rows = build_behind_bars_rule_check_duplicate_stem_rows(&candidate_xml);
    audit.duplicate_stem_same_pitch_candidates = audit
        .duplicate_stem_same_pitch_candidates
        .max(duplicate_stem_rows.len());
    audit.duplicate_stem_same_pitch_rows = duplicate_stem_rows.into_iter().take(12).collect();

    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    audit.visible_pitched_after = after_events
        .iter()
        .filter(|event| !event.is_hidden && event.midi.is_some())
        .count();
    audit.visible_rests_after = after_events
        .iter()
        .filter(|event| !event.is_hidden && event.is_rest)
        .count();
    audit.hidden_pitched_after = after_events
        .iter()
        .filter(|event| event.is_hidden && event.midi.is_some())
        .count();
    audit.hidden_rests_after = after_events
        .iter()
        .filter(|event| event.is_hidden && event.is_rest)
        .count();
    audit.sounding_signature_after = render_visible_sounding_signature(&after_events);
    audit.sounding_signature_stable = audit.sounding_signature_before == audit.sounding_signature_after;
    audit.backup_forward_after_ok = after_backup_forward_ok;
    audit.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    audit.visible_rest_delta = audit.visible_rests_after as i64 - audit.visible_rests_before as i64;

    let attempted_total = legacy_attempted_total + rest_only_attempted_total;
    let accepted_any = legacy_transaction_adopted || rest_only_transaction_adopted;
    let final_dedup_sounding_stable = render_visible_sounding_signature_dedup(&before_events)
        == render_visible_sounding_signature_dedup(&after_events);
    let final_pitched_invariant_ok = if legacy_transaction_adopted
        && audit.duplicate_stem_same_pitch_attempted > 0
    {
        audit.visible_pitched_before
            == audit
                .visible_pitched_after
                .saturating_add(audit.duplicate_stem_same_pitch_attempted)
            && audit.hidden_pitched_after
                == audit
                    .hidden_pitched_before
                    .saturating_add(audit.duplicate_stem_same_pitch_attempted)
            && final_dedup_sounding_stable
    } else {
        audit.sounding_signature_stable
            && audit.visible_pitched_before == audit.visible_pitched_after
            && audit.hidden_pitched_before == audit.hidden_pitched_after
    };
    let expected_final_hidden_rest_reduction = (if legacy_transaction_adopted {
        legacy_hidden_rest_visibility_rewrites
    } else {
        0
    })
    .saturating_add(if rest_only_transaction_adopted {
        measure_rest_outcome.hidden_rests_consumed
    } else {
        0
    });
    let final_hidden_rest_invariant_ok = audit.hidden_rests_before == audit.hidden_rests_after
        || (expected_final_hidden_rest_reduction > 0
            && audit.hidden_rests_before
                == audit
                    .hidden_rests_after
                    .saturating_add(expected_final_hidden_rest_reduction));
    let final_invariants_ok = attempted_total > 0
        && accepted_any
        && candidate_xml.as_str() != xml
        && final_pitched_invariant_ok
        && final_hidden_rest_invariant_ok
        && audit.parse_after_ok
        && audit.backup_forward_after_ok;

    if final_invariants_ok {
        // Pure preparation.  E025 is invoked by the sovereign Stage caller only
        // after every selected owner receipt has replayed against this root.
        audit.written = true;
        if legacy_transaction_adopted {
            audit.lower_staff_rest_chain_adopted = audit.lower_staff_rest_chain_attempted;
            audit.external_feedback_beat_rest_ledger_adopted =
                audit.external_feedback_beat_rest_ledger_attempted;
            audit.spurious_empty_voice_rest_adopted = audit.spurious_empty_voice_rest_attempted;
            audit.cross_staff_voice_ledger_adopted = audit.cross_staff_voice_ledger_attempted;
            audit.duplicate_stem_same_pitch_adopted = audit.duplicate_stem_same_pitch_attempted;
            for action in &mut actions {
                action.verdict = "adopted".to_string();
            }
        } else {
            audit.lower_staff_rest_chain_rejected = audit.lower_staff_rest_chain_attempted;
            audit.external_feedback_beat_rest_ledger_rejected =
                audit.external_feedback_beat_rest_ledger_attempted;
            audit.spurious_empty_voice_rest_rejected = audit.spurious_empty_voice_rest_attempted;
            audit.cross_staff_voice_ledger_rejected = audit.cross_staff_voice_ledger_attempted;
            audit.duplicate_stem_same_pitch_rejected = audit.duplicate_stem_same_pitch_attempted;
            for action in &mut actions {
                action.verdict = "rejected_by_legacy_invariant_guard".to_string();
            }
        }
        if rest_only_transaction_adopted {
            audit.complete_measure_rest_adopted = audit.complete_measure_rest_attempted;
            audit.rest_only_auxiliary_lane_adopted = audit.rest_only_auxiliary_lane_attempted;
            audit.complete_measure_rest_hidden_rests_consumed =
                measure_rest_outcome.hidden_rests_consumed;
            audit.complete_measure_rest_visible_rests_consumed =
                measure_rest_outcome.visible_rests_consumed;
            audit.complete_measure_rest_forwards_consumed = measure_rest_outcome.forwards_consumed;
        } else {
            audit.complete_measure_rest_rejected = audit.complete_measure_rest_attempted;
            audit.rest_only_auxiliary_lane_rejected = audit.rest_only_auxiliary_lane_attempted;
        }
        audit.verdict = match (legacy_transaction_adopted, rest_only_transaction_adopted) {
            (true, true) => "behind_bars_legacy_plus_canonical_measure_rest_adopted_single_writer",
            (true, false) => "behind_bars_legacy_adopted_measure_rest_rejected_single_writer",
            (false, true) => "canonical_measure_rest_adopted_independently_single_writer",
            (false, false) => "behind_bars_review_no_adopted_transaction",
        }
        .to_string();
    } else {
        audit.lower_staff_rest_chain_rejected = audit.lower_staff_rest_chain_attempted;
        audit.external_feedback_beat_rest_ledger_rejected =
            audit.external_feedback_beat_rest_ledger_attempted;
        audit.spurious_empty_voice_rest_rejected = audit.spurious_empty_voice_rest_attempted;
        audit.cross_staff_voice_ledger_rejected = audit.cross_staff_voice_ledger_attempted;
        audit.duplicate_stem_same_pitch_rejected = audit.duplicate_stem_same_pitch_attempted;
        audit.complete_measure_rest_rejected = audit.complete_measure_rest_attempted;
        audit.rest_only_auxiliary_lane_rejected = audit.rest_only_auxiliary_lane_attempted;
        audit.verdict = if attempted_total == 0 {
            "behind_bars_review_no_safe_rest_cleanup_candidate_tie_audit_only".to_string()
        } else if accepted_any {
            "behind_bars_review_local_transaction_rejected_by_final_invariant_guard".to_string()
        } else {
            "behind_bars_review_all_local_transactions_rejected".to_string()
        };
        for action in &mut actions {
            action.verdict = "rejected_by_final_invariant_guard".to_string();
        }
    }
    audit.actions = actions;
    audit.examples.push("Die Behind-Bars-Regelmatrix ist keine Takt-Sonderlogik: Untere-Staff-Restketten werden nur nach Staff/Voice/Onset/Dauer und globaler Signatur geprüft.".to_string());
    audit.examples.push("Restketten dürfen auf mehrere größere Restwerte reduziert werden, wenn ein einzelner Gesamtwert nicht notierbar ist; z. B. wird aus vier Fragmenten nicht blind ein Ganzpausen-Symbol, sondern eine metrisch gültige Sequenz.".to_string());
    audit.examples.push("UpperStaffRestLaneVisibleProofRule/CrossStaffEmptyRestLaneToForwardRule: Leere Zusatzstimmen-Pausen werden nur in Forward-Zeit überführt, wenn derselbe Staff im selben Bereich sichtbares klingendes Material in einer anderen Stimme trägt oder dieselbe Voice cross-staff sichtbar weiterläuft; das gilt staff-basiert für obere und untere Hand, ohne Takt-Sonderlogik.".to_string());
    audit.examples.push("CrossStaffVoiceLedgerMaterializer: Cross-Staff-Stimmen werden nur renderseitig mit vollständiger 4x16tel-Beam-Buchhaltung materialisiert, wenn eine komplette Voice denselben Takt über Staff 1+2 trägt und ein leeres Ledger bereits als Backup/Forward existiert.".to_string());
    audit.examples.push("SamePitchTie/Reattack bleibt audit-only: Eine scheinbar zerstückelte gleiche Tonhöhe wird nur markiert; ohne Reattack-Evidenz aus MIDI-Quelle oder WAV wird sie nicht automatisch zusammengezogen.".to_string());
    audit.examples.push("BeatLocalVoiceRestRewrite: Pausen werden aus Staff-/Voice-/Beat-Belegung und nicht aus kosmetischer Verschiebung abgeleitet; fehlende Staff-Edge-Pausen, kurze Backup-Anker und versteckte Endpausen bleiben Render-XML-only und metrisch gebunden.".to_string());
    audit.examples.push("CanonicalMeasureRestSingleOwner: Vollstaendige klanglose Single-Staff-Taktabdeckung wird hier einmalig zu genau einer measure=yes-Pause kanonisiert; redundante sichtbare/versteckte technische Rest-Carrier und Forward-Zeit werden nur innerhalb derselben Lane konsumiert. Cross-Staff-Voices und semantisch annotierte Pausen bleiben geschlossen.".to_string());
    audit.examples.push(format!(
        "CanonicalMeasureRestSingleOwner outcome: measure-rest adopted/rejected {}/{}, auxiliary-lane adopted/rejected {}/{}.",
        audit.complete_measure_rest_adopted,
        audit.complete_measure_rest_rejected,
        audit.rest_only_auxiliary_lane_adopted,
        audit.rest_only_auxiliary_lane_rejected
    ));
    audit.examples.push("DuplicateStemSamePitchGuard bleibt als Behind Bars-Gate sichtbar: gleiche Tonhöhe am selben Onset in mehreren Stimmen wird nicht blind gelöscht, sondern als Doppelhals-/Rollenfrage reportet.".to_string());

    audit.done_fragment = format!(
        "BeatLocalVoiceRestRewrite/BehindBarsRestAndTieAudit: Restketten {}/{}, BeatRestLedger {}/{}, leere Zusatzpausen/obere Füllpausen {}/{}, CrossStaffVoiceLedger {}/{}, CanonicalMeasureRest {}/{}, AuxiliaryLane {}/{}, DuplicateStemSuppress {}/{}, SamePitch/Reattack {}, DuplicateStem {}, Render-XML-only {}, klingend stabil {}",
        audit.lower_staff_rest_chain_adopted,
        audit.lower_staff_rest_chain_attempted,
        audit.external_feedback_beat_rest_ledger_adopted,
        audit.external_feedback_beat_rest_ledger_attempted,
        audit.spurious_empty_voice_rest_adopted,
        audit.spurious_empty_voice_rest_attempted,
        audit.cross_staff_voice_ledger_adopted,
        audit.cross_staff_voice_ledger_attempted,
        audit.complete_measure_rest_adopted,
        audit.complete_measure_rest_attempted,
        audit.rest_only_auxiliary_lane_adopted,
        audit.rest_only_auxiliary_lane_attempted,
        audit.duplicate_stem_same_pitch_adopted,
        audit.duplicate_stem_same_pitch_attempted,
        audit.same_pitch_reattack_candidates,
        audit.duplicate_stem_same_pitch_candidates,
        if audit.written { "geschrieben" } else { "nicht geschrieben" },
        if audit.sounding_signature_stable { "ja" } else { "nein" }
    );
    audit.pipeline_line = format!(
        "BeatLocalVoiceRestRewrite v{} mit BeatLocalVoiceOccupancyLedger + RestBeatAnchorPlacement + MissingStaffEdgeRest + HiddenRestVisibilityProof + RestCoalescingWithinBeat + CrossStaffVoiceLedgerMaterializer + DuplicateStemSamePitchGuard: Render-XML-only auf {}; lower_staff_rest_chain seen/attempted/adopted {}/{}/{}; external_feedback_beat_rest_ledger seen/attempted/adopted {}/{}/{}; empty_or_upper_rest_lane seen/attempted/adopted {}/{}/{}; cross_staff_voice_ledger seen/attempted/adopted {}/{}/{}; canonical_measure_rest seen/attempted/adopted {}/{}/{} hidden_rests_consumed {} visible_rests_consumed {} forwards_consumed {}; auxiliary_lane seen/attempted/adopted {}/{}/{}; same_pitch_reattack {}, tie_boundary {}, duplicate_stem {}, duplicate_stem_suppressed {}/{}; sichtbare Pausen {}→{}; sichtbare klingende Noten {}→{}; hidden klingend {}→{}; hidden Pausen {}→{}; sounding_signature {}/{} stabil {}; visible_rest_delta {}; parse/back-forward nachher {}/{}; geschrieben {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.lower_staff_rest_chain_candidates_seen,
        audit.lower_staff_rest_chain_attempted,
        audit.lower_staff_rest_chain_adopted,
        audit.external_feedback_beat_rest_ledger_candidates_seen,
        audit.external_feedback_beat_rest_ledger_attempted,
        audit.external_feedback_beat_rest_ledger_adopted,
        audit.spurious_empty_voice_rest_candidates_seen,
        audit.spurious_empty_voice_rest_attempted,
        audit.spurious_empty_voice_rest_adopted,
        audit.cross_staff_voice_ledger_candidates_seen,
        audit.cross_staff_voice_ledger_attempted,
        audit.cross_staff_voice_ledger_adopted,
        audit.complete_measure_rest_candidates_seen,
        audit.complete_measure_rest_attempted,
        audit.complete_measure_rest_adopted,
        audit.complete_measure_rest_hidden_rests_consumed,
        audit.complete_measure_rest_visible_rests_consumed,
        audit.complete_measure_rest_forwards_consumed,
        audit.rest_only_auxiliary_lane_candidates_seen,
        audit.rest_only_auxiliary_lane_attempted,
        audit.rest_only_auxiliary_lane_adopted,
        audit.same_pitch_reattack_candidates,
        audit.same_pitch_tie_boundary_candidates,
        audit.duplicate_stem_same_pitch_candidates,
        audit.duplicate_stem_same_pitch_adopted,
        audit.duplicate_stem_same_pitch_attempted,
        audit.visible_rests_before,
        audit.visible_rests_after,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.hidden_pitched_before,
        audit.hidden_pitched_after,
        audit.hidden_rests_before,
        audit.hidden_rests_after,
        audit.sounding_signature_before,
        audit.sounding_signature_after,
        if audit.sounding_signature_stable { "ja" } else { "nein" },
        audit.visible_rest_delta,
        if audit.parse_after_ok { "ok" } else { "fail" },
        if audit.backup_forward_after_ok { "ok" } else { "fail" },
        if audit.written { "ja" } else { "nein" },
        audit.verdict
    );
    let candidate = if final_invariants_ok {
        Some(candidate_xml)
    } else {
        accepted_receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::LosingCandidate,
        );
        accepted_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        None
    };
    Ok((audit, candidate, accepted_receipts))
}
fn collect_visible_staff2_note_infos(measure: &str) -> Vec<(usize, i32)> {
    let mut out = Vec::new();
    let mut scan = 0usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else { break; };
        let end = start + end_rel + "</note>".len();
        let note = &measure[start..end];
        let open_end = note.find('>').unwrap_or(0);
        let open_tag = &note[..=open_end.min(note.len().saturating_sub(1))];
        let visible = !(open_tag.contains("print-object=\"no\"") || open_tag.contains("print-object='no'"));
        let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
        if visible
            && staff.trim() == "2"
            && let Some(midi) = midi_from_note_fragment(note)
        {
            out.push((start, midi));
        }
        scan = end;
    }
    out
}

fn count_visible_staff2_rests_before(measure: &str, local_limit: usize) -> usize {
    let mut count = 0usize;
    let mut scan = 0usize;
    let limit = local_limit.min(measure.len());
    while let Some(rel) = measure[scan..limit].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else { break; };
        let end = start + end_rel + "</note>".len();
        if end > limit { break; }
        let note = &measure[start..end];
        let visible = !is_hidden_note(note);
        let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
        if visible && staff.trim() == "2" && note.contains("<rest") {
            count += 1;
        }
        scan = end;
    }
    count
}
