fn build_render_xml_print_cleanup_one_safe_visual_trial(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> (anyhow::Result<RenderXmlPrintCleanupOneSafeVisualTrial>, Option<BlockTransformationReceipt>) {
    let mut realized_post = None;
    
    let output = (|| {
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut trial = RenderXmlPrintCleanupOneSafeVisualTrial {
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

    if !trial.parse_before_ok || !trial.backup_forward_before_ok {
        trial.verdict = "print_cleanup_blocked_by_input_parse_or_timeline".to_string();
        trial.done_fragment = "RenderXmlPrintCleanupOneSafeVisualTrial: blockiert vor Mutation; keine Änderung".to_string();
        trial.pipeline_line = format!(
            "RenderXmlPrintCleanupOneSafeVisualTrial v{}: blockiert vor Mutation; parse/back-forward {}/{}; keine Render-XML-Änderung.",
            env!("CARGO_PKG_VERSION"),
            if trial.parse_before_ok { "ok" } else { "fail" },
            if trial.backup_forward_before_ok { "ok" } else { "fail" }
        );
        return Ok(trial);
    }

    trial.attempted = true;
    let mut candidate_xml = xml.to_string();
    let mut actions = Vec::<RenderXmlPianisticRestRepairAction>::new();
    let candidate_count = build_rest_consolidation_candidates(xml).len();
    trial.candidates_seen = candidate_count;

    let mut clef_candidate_xml = candidate_xml.clone();
    // Authorization failure aborts this owner execution before any receipt or
    // planned post exists; RenderSovereignState records it as a failed owner.
    let moved_boundary_bass_clefs =
        advance_lower_staff_bass_clef_before_preceding_low_or_rest_events(&mut clef_candidate_xml)?;
    let rest_only_clefs = prune_rest_only_staff2_clef_noops(&mut clef_candidate_xml).0;
    let (empty_clefs, trailing_clefs, redundant_clefs, advance_clefs) = prune_redundant_lower_staff_clefs(&mut clef_candidate_xml).0;
    let clef_cleanup_count = rest_only_clefs + empty_clefs + trailing_clefs + redundant_clefs + advance_clefs + moved_boundary_bass_clefs;
    if clef_cleanup_count > 0 {
        candidate_xml = clef_candidate_xml;
    }
    trial.rest_only_clef_noops_removed = rest_only_clefs;
    trial.empty_clef_ranges_removed = empty_clefs;
    trial.trailing_clefs_removed = trailing_clefs;
    trial.redundant_clefs_removed = redundant_clefs;
    trial.advance_clefs_removed = advance_clefs + moved_boundary_bass_clefs;
    trial.clef_cleanup_candidates = clef_cleanup_count;

    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.visible_signature_after = render_visible_signature(&after_events, false);
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    trial.visible_rest_delta = trial.visible_rests_after as i64 - trial.visible_rests_before as i64;

    let rest_cleanup_count = actions.len();
    let cleanup_count = rest_cleanup_count + trial.clef_cleanup_candidates;
    let invariants_ok = cleanup_count > 0
        && rest_cleanup_count <= 1
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.hidden_pitched_before == trial.hidden_pitched_after
        && trial.hidden_rests_before == trial.hidden_rests_after
        && trial.visible_rest_delta <= 0
        && trial.backup_forward_after_ok
        && trial.parse_after_ok;

    if invariants_ok {
        // #57 is prepared before any productive artifact write. The existing
        // E043 sink is invoked only by RenderSovereignState after this BLOCK
        // receipt has replayed against the selected root.
        let _ = output_path;
        realized_post = Some(candidate_xml.clone());
        trial.written = true;
        trial.safe_candidates_seen = cleanup_count;
        trial.written_candidates = cleanup_count;
        trial.clef_cleanups_written = trial.clef_cleanup_candidates;
        trial.verdict = "notation_surface_safe_render_cleanup_adopted".to_string();
    } else {
        trial.rejected_candidates = cleanup_count;
        trial.verdict = if cleanup_count == 0 {
            "notation_surface_safe_render_cleanup_no_safe_candidate".to_string()
        } else {
            "notation_surface_safe_render_cleanup_rejected_by_invariant_guard".to_string()
        };
        for action in &mut actions {
            action.verdict = "rejected_by_print_cleanup_invariant_guard".to_string();
        }
    }

    trial.actions = actions;
    trial.examples.push("Render-XML-only: Dieser Trial läuft nach dem NotationSurfaceLedger und darf sichere Pausenketten sowie redundante oder zu spät gesetzte Staff-2-Schlüsselzustände bereinigen.".to_string());
    trial.examples.push("Master-XML, klingende Noten, Tonhöhen, Onsets, Dauern, Staffs und Voices klingender Events bleiben unverändert.".to_string());
    trial.examples.push("Verzögerte Bass-Reasserts und Carry-over-Schlüssel sind Cleanup-Kandidaten; Cross-Staff-/Forward-Lücken bleiben ohne Hand-/Voice-Beweis ausgeschlossen.".to_string());

    trial.done_fragment = format!(
        "NotationSurfaceSafeRenderCleanup: Rest-Kandidaten {}, Clef-Kandidaten {}, sicher {}, geschrieben {}, sichtbare Pausen {}→{}, Render-XML-only {}, klingend stabil {}",
        trial.candidates_seen,
        trial.clef_cleanup_candidates,
        trial.safe_candidates_seen,
        trial.written_candidates,
        trial.visible_rests_before,
        trial.visible_rests_after,
        if trial.written { "ja" } else { "nein" },
        if trial.sounding_signature_stable { "ja" } else { "nein" }
    );
    trial.pipeline_line = format!(
        "NotationSurfaceSafeRenderCleanup v{}: Render-XML-only auf {}; Rest-Kandidaten {}, Clef-Kandidaten {} (rest-only {}, empty {}, trailing {}, redundant {}, advance/moved {}), sicher {}, geschrieben {}, verworfen {}; Typ rest_chain_plus_clef_state_cleanup; sichtbare klingende Noten {}→{}; sichtbare Pausen {}→{}; versteckte klingende Noten {}→{}; versteckte Pausen {}→{}; Master-XML unverändert ja; sounding_signature {}/{} stabil {}; visible_rest_delta {}; parse/back-forward nachher {}/{}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.candidates_seen,
        trial.clef_cleanup_candidates,
        trial.rest_only_clef_noops_removed,
        trial.empty_clef_ranges_removed,
        trial.trailing_clefs_removed,
        trial.redundant_clefs_removed,
        trial.advance_clefs_removed,
        trial.safe_candidates_seen,
        trial.written_candidates,
        trial.rejected_candidates,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.visible_rests_before,
        trial.visible_rests_after,
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
        trial.verdict
    );
    Ok(trial)
})();
    
    let receipt = realized_post
        .filter(|post| output.is_ok() && post != xml)
        .map(|post| {
            crate::master_state_identity::BlockTransformationReceipt::mint_file_bound_block_receipt(
                RenderRow57BlockMint { _private: () },
                xml,
                post,
                Some((0, xml.len())),
            )
            .expect("frozen owner changed bytes without receipt")
        });
    (output, receipt)
}

fn collect_forward_spans_for_missing_rest_insert(xml: &str) -> Vec<RenderXmlForwardSpan> {
    let mut spans = Vec::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let mut cursor = 0_i32;
        let mut local_scan = 0_usize;
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
                    if !is_chord {
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
                    local_scan = end;
                }
                "forward" => {
                    let Some(close_rel) = measure[pos..].find("</forward>") else { break; };
                    let end = pos + close_rel + "</forward>".len();
                    let fragment = &measure[pos..end];
                    let duration = extract_first_text_from_fragment(fragment, "duration")
                        .and_then(|value| value.trim().parse::<i32>().ok())
                        .unwrap_or(0);
                    let onset = cursor;
                    spans.push(RenderXmlForwardSpan {
                        scope: scope_number,
                        onset,
                        duration,
                        start: measure_start + pos,
                        end: measure_start + end,
                    });
                    cursor = cursor.saturating_add(duration);
                    local_scan = end;
                }
                _ => break,
            }
        }
        scan = measure_end;
    }
    spans
}

fn missing_rest_visible_template(staff: &str, voice: &str, duration: i32) -> Option<String> {
    let (rest_type, dots) = duration_to_rest_type_and_dots(duration)?;
    let mut out = String::new();
    out.push_str("      <note>\n");
    out.push_str("        <rest/>\n");
    out.push_str(&format!("        <duration>{duration}</duration>\n"));
    out.push_str(&format!("        <voice>{voice}</voice>\n"));
    out.push_str(&format!("        <type>{rest_type}</type>\n"));
    for _ in 0..dots {
        out.push_str("        <dot/>\n");
    }
    out.push_str(&format!("        <staff>{staff}</staff>\n"));
    out.push_str("      </note>");
    Some(out)
}

fn missing_rest_visible_sequence_template(
    staff: &str,
    voice: &str,
    onset: i32,
    duration: i32,
    expected: i32,
    beat_unit: i32,
) -> Option<String> {
    // Fehlende sichtbare Zeit wird aus bestehender Forward-Zeit materialisiert;
    // die Dauerzerlegung gehoert dabei derselben zentralen Partial-Rest-Policy
    // wie dem Master-VisibleRhythmLedger.
    let parts = crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
        i64::from(onset),
        i64::from(duration),
        i64::from(expected),
        i64::from(beat_unit),
    );
    if parts.is_empty() {
        return None;
    }
    let mut notes = Vec::new();
    for part in parts {
        notes.push(missing_rest_visible_template(staff, voice, i32::try_from(part).ok()?)?);
    }
    Some(notes.join("\n"))
}

fn build_missing_rest_insert_candidates(xml: &str, blocked_examples: &mut Vec<String>) -> (Vec<RenderXmlMissingRestInsertCandidate>, usize) {
    let (events, _backup_ok) = collect_render_xml_note_events(xml);
    let forwards = collect_forward_spans_for_missing_rest_insert(xml);
    let expected_by_scope = render_scope_expected_durations(&events);
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let mut by_scope_staff_voice: BTreeMap<(usize, String, String), Vec<RenderXmlNoteEvent>> = BTreeMap::new();
    let mut staffs_by_scope_voice: BTreeMap<(usize, String), BTreeSet<String>> = BTreeMap::new();
    for event in &events {
        if event.is_hidden || event.is_chord {
            continue;
        }
        if !(event.is_rest || event.midi.is_some()) {
            continue;
        }
        by_scope_staff_voice
            .entry((event.scope, event.staff.clone(), event.voice.clone()))
            .or_default()
            .push(event.clone());
        staffs_by_scope_voice
            .entry((event.scope, event.voice.clone()))
            .or_default()
            .insert(event.staff.clone());
    }

    let mut candidates = Vec::new();
    let mut missing_rows_seen = 0_usize;
    for ((scope, staff, voice), mut group) in by_scope_staff_voice {
        let expected = expected_by_scope.get(&scope).copied().unwrap_or(384).max(1);
        let visible_pitched = group.iter().filter(|event| event.midi.is_some()).count();
        let visible_rests = group.iter().filter(|event| event.is_rest).count();
        if visible_pitched == 0 || visible_rests > 0 {
            continue;
        }
        let ledger_duration: i32 = group.iter().map(|event| event.duration.max(0)).sum();
        if ledger_duration >= expected {
            continue;
        }
        missing_rows_seen += 1;
        let staff_count = staffs_by_scope_voice
            .get(&(scope, voice.clone()))
            .map_or(1, |set| set.len());
        if staff_count > 1 {
            if blocked_examples.len() < 4 {
                blocked_examples.push(format!(
                    "scope {scope} staff {staff} voice {voice} blockiert: gleiche Voice nutzt mehrere Systeme; fehlende Pausen brauchen hier Voice-/Hand-Ledger statt Einzel-Rest-Einfügung."
                ));
            }
            continue;
        }
        group.sort_by_key(|event| (event.onset, event.duration));
        let mut cursor = 0_i32;
        let mut gaps = Vec::<(i32, i32)>::new();
        for event in &group {
            if event.onset > cursor {
                gaps.push((cursor, event.onset.saturating_sub(cursor)));
            }
            cursor = cursor.max(event.onset.saturating_add(event.duration.max(0)));
        }
        if cursor < expected {
            gaps.push((cursor, expected.saturating_sub(cursor)));
        }
        let mut safe_gap_found = false;
        for (gap_onset, gap_duration) in gaps {
            if gap_duration <= 0 {
                continue;
            }
            let Some((metric_expected, beat_unit)) = metric_by_scope.get(&scope).copied() else {
                continue;
            };
            if crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                i64::from(gap_onset),
                i64::from(gap_duration),
                i64::from(metric_expected),
                i64::from(beat_unit),
            )
            .is_empty()
            {
                continue;
            }
            if gap_onset % 24 != 0 || gap_duration % 24 != 0 {
                continue;
            }
            let Some(forward) = forwards.iter().find(|span| span.scope == scope && span.onset == gap_onset && span.duration == gap_duration) else {
                continue;
            };
            safe_gap_found = true;
            let beat_aligned_bonus = if gap_onset % 96 == 0 && gap_duration % 96 == 0 { 200 } else { 0 };
            candidates.push(RenderXmlMissingRestInsertCandidate {
                scope,
                staff: staff.clone(),
                voice: voice.clone(),
                onset: gap_onset,
                duration: gap_duration,
                forward_start: forward.start,
                forward_end: forward.end,
                score: 1000 + beat_aligned_bonus + gap_duration as i64,
            });
        }
        if !safe_gap_found && blocked_examples.len() < 4 {
            blocked_examples.push(format!(
                "scope {scope} staff {staff} voice {voice} blockiert: fehlende Dauer ist nicht als einzelne passende forward-Zeit mit sicherem Resttyp materialisierbar."
            ));
        }
    }
    candidates.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.duration.cmp(&a.duration))
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
            .then_with(|| a.onset.cmp(&b.onset))
    });
    (candidates, missing_rows_seen)
}



fn build_render_xml_pianistic_missing_rest_insert_trial(
    xml: &str,
    source_label: &str,
    _output_path: &Path,
) -> anyhow::Result<(
    RenderXmlPianisticMissingRestInsertTrial,
    Option<String>,
    crate::master_state_identity::RenderTransitionReceipts,
)> {
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut trial = RenderXmlPianisticMissingRestInsertTrial {
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

    if !trial.parse_before_ok || !trial.backup_forward_before_ok {
        trial.verdict = "pianistic_missing_rest_insert_blocked_by_input_parse_or_timeline".to_string();
        trial.done_fragment = "RenderXmlPianisticMissingRestInsertTrial: blockiert vor Mutation; keine Änderung".to_string();
        trial.pipeline_line = format!(
            "RenderXmlPianisticMissingRestInsertTrial v{}: blockiert vor Mutation; parse/back-forward {}/{}; keine Render-XML-Änderung.",
            env!("CARGO_PKG_VERSION"),
            if trial.parse_before_ok { "ok" } else { "fail" },
            if trial.backup_forward_before_ok { "ok" } else { "fail" }
        );
        return Ok((
            trial,
            None,
            crate::master_state_identity::RenderTransitionReceipts::new(),
        ));
    }

    trial.attempted = true;
    let mut blocked_examples = Vec::new();
    let (candidate_xml, actions, safe_count, missing_seen, missing_rest_receipt) =
        apply_pianistic_missing_rest_insert_with_receipt(xml, 8, &mut blocked_examples);
    let has_missing_rest_receipt = missing_rest_receipt.is_some();
    let mut render_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    render_receipts.push_exact(missing_rest_receipt);
    trial.blocked_examples = blocked_examples;
    trial.missing_candidates_seen = missing_seen;
    trial.safe_candidates_seen = safe_count;
    trial.insert_attempted = actions.len();

    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.visible_signature_after = render_visible_signature(&after_events, false);
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    trial.visible_rest_delta = trial.visible_rests_after as i64 - trial.visible_rests_before as i64;

    let invariants_ok = trial.insert_attempted > 0
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.hidden_pitched_before == trial.hidden_pitched_after
        && trial.hidden_rests_before == trial.hidden_rests_after
        && trial.visible_rests_after >= trial.visible_rests_before
        && trial.backup_forward_after_ok
        && trial.parse_after_ok
        && has_missing_rest_receipt;

    if invariants_ok {
        // The #69 EXACT receipt is replayed by RenderSovereignState before the
        // existing missing-rest sink may materialize the candidate.
        trial.written = true;
        trial.insert_adopted = trial.insert_attempted;
        trial.verdict = "pianistic_missing_rest_insert_trial_adopted_render_xml_only".to_string();
    } else {
        trial.insert_rejected = trial.insert_attempted;
        trial.verdict = if trial.insert_attempted == 0 {
            "pianistic_missing_rest_insert_trial_no_extremely_safe_candidate".to_string()
        } else {
            "pianistic_missing_rest_insert_trial_rejected_by_invariant_guard".to_string()
        };
    }
    trial.actions = actions;
    trial.examples.push("Render-XML-only: Der Trial ersetzt nur vorhandene forward-Zeit durch sichtbare Pausen; klingende Events bleiben unverändert.".to_string());
    trial.examples.push("Extrem sichere Einfügung heißt: einzelner notierbarer Resttyp, passende forward-Zeit, keine cross-staff Voice-Verkürzung, parse/back-forward stabil und klingende Signatur unverändert.".to_string());
    trial.examples.push("Keine suppress-Logik, kein Voice-Rewrite, keine neue Note, keine konkrete Bereichlogik; Bereichsnummern sind berechnete Ergebnisdaten.".to_string());

    trial.done_fragment = format!(
        "RenderXmlPianisticMissingRestInsertTrial: Fehlpausen-Kandidaten {}, extrem sicher {}, eingefügt {}, Render-XML-only {}, klingend stabil {}",
        trial.missing_candidates_seen,
        trial.safe_candidates_seen,
        trial.insert_adopted,
        if trial.written { "geschrieben" } else { "nicht geschrieben" },
        if trial.sounding_signature_stable { "ja" } else { "nein" }
    );
    trial.pipeline_line = format!(
        "RenderXmlPianisticMissingRestInsertTrial v{}: Render-XML-only auf {}; fehlende sichtbare Pausen Kandidaten {}, extrem sichere forward-Einfügekandidaten {}, versucht {}, übernommen {}; sichtbare Pausen {}→{}; sichtbare klingende Noten {}→{}; versteckte klingende Noten {}→{}; versteckte Pausen {}→{}; sounding_signature {}/{} stabil {}; visible_rest_delta {}; parse/back-forward nachher {}/{}; geschrieben {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.missing_candidates_seen,
        trial.safe_candidates_seen,
        trial.insert_attempted,
        trial.insert_adopted,
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
    let candidate = invariants_ok.then_some(candidate_xml);
    Ok((trial, candidate, render_receipts))
}


pub(crate) struct RenderRow57BlockMint { _private: () }

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/row57_prepare_boundary_tests.rs"));


// Receipt-bearing mutation preparation belongs to the canonical structural owner.
include!("../structural_write_authority/human_pdf_pianistic_cross_staff_ledger_tail_receipt_ownership.rs");
