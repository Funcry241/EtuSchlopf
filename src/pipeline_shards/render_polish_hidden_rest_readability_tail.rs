type PreparedRestTieActions = ((String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize), crate::master_state_identity::RenderTransitionReceipts);

fn render_xml_trial_path_for_source(source: &source::SourcePlan) -> PathBuf {
    source.render_musicxml_out.clone()
}
fn build_retention_first_p2_split_occurrence_receipt(
    source_xml: &str,
    removed_source_note_ranges: &[(usize, usize)],
) -> anyhow::Result<Vec<RetentionFirstP2SplitOccurrenceReceiptRow>> {
    let source_notes = parse_atomic_notes(source_xml);
    let removed_ranges = removed_source_note_ranges
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if removed_ranges.len() != removed_source_note_ranges.len() {
        return Err(anyhow!(
            "RetentionFirstP2SplitOccurrenceContinuation: prune receipt contains duplicate source ranges"
        ));
    }

    let mut next_raw_by_scope = std::collections::BTreeMap::<usize, usize>::new();
    let mut next_pitched_by_scope = std::collections::BTreeMap::<usize, usize>::new();
    let mut consumed_removed_ranges = std::collections::BTreeSet::<(usize, usize)>::new();
    let mut rows = Vec::<RetentionFirstP2SplitOccurrenceReceiptRow>::new();
    for note in &source_notes {
        let scope = note.measure.parse::<usize>().map_err(|_| {
            anyhow!("RetentionFirstP2SplitOccurrenceContinuation: non-numeric split scope")
        })?;
        let raw_note_index = next_raw_by_scope.entry(scope).or_default();
        let current_raw_note_index = *raw_note_index;
        *raw_note_index = raw_note_index.saturating_add(1);

        let source_range = (note.note_start, note.note_end);
        if removed_ranges.contains(&source_range) {
            if note.pitch.is_none() || !consumed_removed_ranges.insert(source_range) {
                return Err(anyhow!(
                    "RetentionFirstP2SplitOccurrenceContinuation: prune receipt removed a non-pitched or duplicate occurrence"
                ));
            }
            continue;
        }
        let Some(pitch) = note.pitch.as_ref().map(|pitch| pitch.midi) else {
            continue;
        };
        let pitched_ordinal = next_pitched_by_scope.entry(scope).or_default();
        rows.push(RetentionFirstP2SplitOccurrenceReceiptRow {
            scope,
            raw_note_index: current_raw_note_index,
            pitched_ordinal: *pitched_ordinal,
            onset: note.start,
            pitch,
            staff: note.staff.clone(),
            chord_syntax: note.is_chord,
        });
        *pitched_ordinal = pitched_ordinal.saturating_add(1);
    }
    if consumed_removed_ranges != removed_ranges {
        return Err(anyhow!(
            "RetentionFirstP2SplitOccurrenceContinuation: prune receipt contains an unrecognized source range"
        ));
    }
    Ok(rows)
}

fn validate_retention_first_p2_split_occurrence_receipt(
    xml: &str,
    rows: &[RetentionFirstP2SplitOccurrenceReceiptRow],
) -> anyhow::Result<()> {
    let (events, backup_forward_ok) = collect_render_xml_note_events(xml);
    if !backup_forward_ok {
        return Err(anyhow!(
            "RetentionFirstP2SplitOccurrenceContinuation: split render timeline is not balanced"
        ));
    }
    let pitched_events = events
        .iter()
        .filter(|event| event.midi.is_some())
        .collect::<Vec<_>>();
    if pitched_events.len() != rows.len() {
        return Err(anyhow!(
            "RetentionFirstP2SplitOccurrenceContinuation: split pitched occurrence count drift expected={} actual={}",
            rows.len(),
            pitched_events.len()
        ));
    }
    let mut expected_ordinal_by_scope = std::collections::BTreeMap::<usize, usize>::new();
    for (event, row) in pitched_events.iter().zip(rows) {
        let ordinal = expected_ordinal_by_scope.entry(row.scope).or_default();
        if row.pitched_ordinal != *ordinal
            || event.scope != row.scope
            || i64::from(event.onset) != row.onset
            || event.midi != Some(row.pitch)
            || event.staff != row.staff
            || event.is_chord != row.chord_syntax
        {
            return Err(anyhow!(
                "RetentionFirstP2SplitOccurrenceContinuation: split occurrence topology drift at scope {} raw {} ordinal {}",
                row.scope,
                row.raw_note_index,
                row.pitched_ordinal
            ));
        }
        *ordinal = ordinal.saturating_add(1);
    }
    Ok(())
}

fn build_render_xml_pruned_pdf_trial(
    master_xml: &str,
    audit: &RenderXmlHiddenMaterialPruneAudit,
    render_xml_path: &Path,
) -> anyhow::Result<(
    RenderXmlPrunedPdfTrial,
    Option<String>,
    crate::master_state_identity::RenderTransitionReceipts,
)> {
    let mut trial = RenderXmlPrunedPdfTrial {
        skipped_risky_timeline_rests: audit.risky_timeline_rests,
        skipped_unknown_hidden_events: audit.unknown_hidden_events + audit.blocked_visible_dependency,
        skipped_hidden_primary_notes: audit.risky_hidden_primary_notes,
        master_visible_signature: audit.visible_signature_before,
        master_backup_forward_ok: audit.backup_forward_balance_ok,
        master_visible_pitched: audit.visible_pitched_notes,
        ..RenderXmlPrunedPdfTrial::default()
    };
    let split_occurrences = build_retention_first_p2_split_occurrence_receipt(master_xml, &[])?;
    validate_retention_first_p2_split_occurrence_receipt(master_xml, &split_occurrences)?;
    trial.p2_split_occurrences = split_occurrences;
    match std::fs::remove_file(render_xml_path) {
        Ok(_) => {},
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {},
        Err(err) => {
            trial.examples.push(format!(
                "Alte Render-XML konnte nicht bereinigt werden: {}: {}.",
                render_xml_path.display(),
                err
            ));
        }
    }
    if audit.safe_hidden_inner_chord_tones == 0 {
        trial.verdict = "render_xml_pruned_pdf_trial_no_safe_candidates".to_string();
        trial.done_fragment = "RenderXmlPrunedPdfTrial: keine sicheren Render-Prune-Kandidaten".to_string();
        trial.pipeline_line = format!(
            "RenderXmlPrunedPdfTrial v{}: keine sichere Render-XML geschrieben; safe_hidden_inner_chord_tone 0; PDF-Quelle bleibt Master-MusicXML.",
            env!("CARGO_PKG_VERSION")
        );
        return Ok((
            trial,
            None,
            crate::master_state_identity::RenderTransitionReceipts::new(),
        ));
    }
    let (render_xml, prune_stats, prune_receipts) =
        prune_safe_hidden_inner_chord_tones_for_render(master_xml);
    let mut render_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    for (receipt, start, end) in prune_receipts {
        render_receipts.push_exact_fragment(Some(receipt), start, end);
    }
    trial.removed_safe_hidden_inner_chord_tones = prune_stats.removed_safe_hidden_inner_chord_tones;
    let (render_events, render_balance_ok) = collect_render_xml_note_events(&render_xml);
    trial.render_backup_forward_ok = render_balance_ok;
    trial.render_visible_signature = render_visible_signature(&render_events, false);
    trial.visible_signature_stable = trial.master_visible_signature == trial.render_visible_signature;
    trial.render_visible_pitched = render_events
        .iter()
        .filter(|event| !event.is_hidden && event.midi.is_some())
        .count();
    trial.render_hidden_pitched = render_events
        .iter()
        .filter(|event| event.is_hidden && event.midi.is_some())
        .count();
    trial.parse_ok = parse_musicxml(&render_xml).is_ok();
    trial.export_semantics_ok = validate_octave_shift_direction_semantics(&render_xml).is_ok();
    let removed_matches_audit = trial.removed_safe_hidden_inner_chord_tones == audit.safe_hidden_inner_chord_tones
        && prune_stats.removed_hidden_pitched_events == audit.safe_hidden_inner_chord_tones;
    let visible_counts_stable = trial.master_visible_pitched == trial.render_visible_pitched;
    let safe_for_render_pdf = audit.visible_signature_stable
        && audit.backup_forward_balance_ok
        && trial.visible_signature_stable
        && trial.render_backup_forward_ok
        && trial.parse_ok
        && trial.export_semantics_ok
        && removed_matches_audit
        && visible_counts_stable
        && trial.skipped_hidden_primary_notes == 0;
    trial.examples.push(format!(
        "Entfernt wurden nur safe_hidden_inner_chord_tone-Events: {}; versteckte Pausen übersprungen {}; unbekannte/blockierte Events übersprungen {}; primäre versteckte Noten übersprungen {}.",
        trial.removed_safe_hidden_inner_chord_tones,
        trial.skipped_risky_timeline_rests,
        trial.skipped_unknown_hidden_events,
        trial.skipped_hidden_primary_notes
    ));
    trial.examples.push(format!(
        "Sichtbare Signatur Master/Render: {}/{}; sichtbare klingende Noten Master/Render: {}/{}.",
        trial.master_visible_signature,
        trial.render_visible_signature,
        trial.master_visible_pitched,
        trial.render_visible_pitched
    ));
    if safe_for_render_pdf {
        crate::audit_only::destructive_edit_attribution::observe_selected_state_successor(
            master_xml,
            &render_xml,
        );
        crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
            "InitialRenderPrunedPdfCandidateBeforeWrite",
            &render_xml,
        );
        let split_occurrences = build_retention_first_p2_split_occurrence_receipt(
            master_xml,
            &prune_stats.removed_source_note_ranges,
        )?;
        validate_retention_first_p2_split_occurrence_receipt(&render_xml, &split_occurrences)?;
        trial.p2_split_occurrences = split_occurrences;
        // Physical materialization is performed by RenderSovereignState only
        // after this complete fragment-receipt chain has replayed successfully.
        trial.render_xml_written = true;
        trial.pdf_source_selected = true;
        trial.verdict = "render_xml_pruned_pdf_trial_ok_pdf_source_render_xml".to_string();
    } else {
        trial.verdict = "render_xml_pruned_pdf_trial_guarded_master_pdf_source".to_string();
    }
    trial.done_fragment = format!(
        "RenderXmlPrunedPdfTrial: Render-XML-Standardausgabe {}, safe Innentöne entfernt {}, PDF-Quelle {}",
        if trial.render_xml_written { "geschrieben" } else { "nicht geschrieben" },
        trial.removed_safe_hidden_inner_chord_tones,
        if trial.pdf_source_selected { "Render-XML" } else { "Master-MusicXML" }
    );
    trial.pipeline_line = format!(
        "RenderXmlPrunedPdfTrial v{}: DualXmlStandardOutput; Master-XML unverändert als Hidden-/Semantik-Variante; Render-XML als Standardausgabe geschrieben {}; entfernt safe_hidden_inner_chord_tone {}; skipped rests/unknown/primary {}/{}/{}; sichtbare Signatur stabil {}; backup-forward Master/Render {}/{}; parse/export {}/{}; PDF-Quelle {}; Urteil {}; keine Pausenmutation, keine Musikmutation, keine PDF-Verbesserungsbehauptung.",
        env!("CARGO_PKG_VERSION"),
        if trial.render_xml_written { "ja" } else { "nein" },
        trial.removed_safe_hidden_inner_chord_tones,
        trial.skipped_risky_timeline_rests,
        trial.skipped_unknown_hidden_events,
        trial.skipped_hidden_primary_notes,
        if trial.visible_signature_stable { "ja" } else { "nein" },
        if trial.master_backup_forward_ok { "ja" } else { "nein" },
        if trial.render_backup_forward_ok { "ja" } else { "nein" },
        if trial.parse_ok { "ja" } else { "nein" },
        if trial.export_semantics_ok { "ja" } else { "nein" },
        if trial.pdf_source_selected { "Render-XML" } else { "Master-MusicXML" },
        trial.verdict
    );
    let candidate = safe_for_render_pdf.then_some(render_xml);
    Ok((trial, candidate, render_receipts))
}
fn prune_safe_hidden_inner_chord_tones_for_render(
    xml: &str,
) -> (
    String,
    RenderXmlPruneStats,
    Vec<(crate::master_state_identity::ExactTransformationReceipt, usize, usize)>,
) {
    let mut out = String::with_capacity(xml.len());
    let mut stats = RenderXmlPruneStats::default();
    let mut receipts = Vec::new();
    let mut scan = 0_usize;
    while let Some((start, end, _scope_number)) = find_next_measure_bounds(xml, scan) {
        out.push_str(&xml[scan..start]);
        let current_start = out.len();
        let ((measure, measure_stats), receipt) =
            prune_safe_hidden_inner_chord_tones_in_measure(&xml[start..end]);
        let current_end = current_start.saturating_add(end.saturating_sub(start));
        if let Some(receipt) = receipt {
            receipts.push((receipt, current_start, current_end));
        }
        out.push_str(&measure);
        stats.removed_safe_hidden_inner_chord_tones +=
            measure_stats.removed_safe_hidden_inner_chord_tones;
        stats.removed_hidden_pitched_events += measure_stats.removed_hidden_pitched_events;
        stats.removed_source_note_ranges.extend(
            measure_stats
                .removed_source_note_ranges
                .iter()
                .map(|(local_start, local_end)| (start.saturating_add(*local_start), start.saturating_add(*local_end))),
        );
        scan = end;
    }
    out.push_str(&xml[scan..]);
    (out, stats, receipts)
}
fn prune_safe_hidden_inner_chord_tones_in_measure(measure: &str) -> ((String, RenderXmlPruneStats), Option<ExactTransformationReceipt>) {
    
    
    let output = {
    let mut out = String::with_capacity(measure.len());
    let mut stats = RenderXmlPruneStats::default();
    let mut cursor = 0_i32;
    let mut scan = 0_usize;
    let mut last_anchor: Option<(i32, RenderXmlAnchorKind)> = None;
    loop {
        let next_note = measure[scan..].find("<note").map(|p| scan + p);
        let next_backup = measure[scan..].find("<backup").map(|p| scan + p);
        let next_forward = measure[scan..].find("<forward").map(|p| scan + p);
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
                let is_rest = note.contains("<rest");
                let midi = midi_from_note_fragment(note);
                let is_hidden = is_hidden_note(note);
                let anchor = if is_chord {
                    last_anchor.map_or(RenderXmlAnchorKind::None, |(_, anchor)| anchor)
                } else {
                    RenderXmlAnchorKind::None
                };
                let safe_hidden_inner = is_hidden
                    && midi.is_some()
                    && is_chord
                    && anchor == RenderXmlAnchorKind::VisiblePitched
                    && !render_note_has_visible_dependency(note);
                out.push_str(&measure[scan..pos]);
                if safe_hidden_inner {
                    stats.removed_safe_hidden_inner_chord_tones += 1;
                    stats.removed_hidden_pitched_events += 1;
                    stats.removed_source_note_ranges.push((pos, end));
                } else {
                    out.push_str(note);
                }
                let current_anchor = render_anchor_kind(is_hidden, is_rest, midi.is_some());
                if !is_chord {
                    last_anchor = Some((cursor, current_anchor));
                    cursor = cursor.saturating_add(duration);
                }
                scan = end;
            }
            "backup" => {
                let Some(close_rel) = measure[pos..].find("</backup>") else { break; };
                let end = pos + close_rel + "</backup>".len();
                out.push_str(&measure[scan..end]);
                let fragment = &measure[pos..end];
                let duration = extract_first_text_from_fragment(fragment, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                cursor = cursor.saturating_sub(duration).max(0);
                last_anchor = None;
                scan = end;
            }
            "forward" => {
                let Some(close_rel) = measure[pos..].find("</forward>") else { break; };
                let end = pos + close_rel + "</forward>".len();
                out.push_str(&measure[scan..end]);
                let fragment = &measure[pos..end];
                let duration = extract_first_text_from_fragment(fragment, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                cursor = cursor.saturating_add(duration);
                last_anchor = None;
                scan = end;
            }
            _ => break,
        }
    }
    out.push_str(&measure[scan..]);
    (out, stats)
};
    let geometry = output.1.removed_source_note_ranges.iter().map(|(start, end)| crate::master_state_identity::ExactEdit::bound(*start, *end, String::new())).collect();
    let receipt = Some(output.0.as_str()).filter(|post| *post != measure).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow22ExactMint { _private: () }, measure, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}
fn build_render_xml_hidden_rest_dependency_audit(xml: &str) -> RenderXmlHiddenRestDependencyAudit {
    let (base_events, _base_backup_ok) = collect_render_xml_note_events(xml);
    let visible_signature_before = render_visible_signature(&base_events, false);
    let hidden_rest_events = base_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    let visible_pitched = base_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    let visible_rests = base_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    let ((forward_xml, forward_stats), forward_receipt) =
        collapse_hidden_timeline_rests_to_forward(xml);
    let mut probe_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    probe_receipts.push_block(forward_receipt, &forward_xml);
    if !probe_receipts.is_empty() {
        probe_receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe,
        );
    }
    let (forward_events, forward_backup_forward_ok) = collect_render_xml_note_events(&forward_xml);
    let forward_visible_signature = render_visible_signature(&forward_events, false);
    let forward_signature_stable = forward_visible_signature == visible_signature_before;
    let forward_parse_ok = parse_musicxml(&forward_xml).is_ok();
    let forward_visible_pitched = forward_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    let forward_visible_rests = forward_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    let forward_possible = hidden_rest_events > 0
        && forward_stats.changed_events == hidden_rest_events
        && forward_signature_stable
        && forward_backup_forward_ok
        && forward_parse_ok
        && forward_visible_pitched == visible_pitched
        && forward_visible_rests == visible_rests;
    RenderXmlHiddenRestDependencyAudit { forward_possible }
}
fn collapse_hidden_timeline_rests_to_forward(xml: &str) -> ((String, RenderXmlHiddenRestVirtualStats), Option<BlockTransformationReceipt>) {
    
    
    let output = {
    let mut out = String::with_capacity(xml.len());
    let mut stats = RenderXmlHiddenRestVirtualStats::default();
    let mut scan = 0_usize;
    while let Some((start, end, _scope_number)) = find_next_measure_bounds(xml, scan) {
        out.push_str(&xml[scan..start]);
        let (measure, measure_stats) = collapse_hidden_timeline_rests_to_forward_in_measure(&xml[start..end]).0;
        out.push_str(&measure);
        stats.changed_events += measure_stats.changed_events;
        scan = end;
    }
    out.push_str(&xml[scan..]);
    (out, stats)
};
    
    let receipt = Some(output.0.as_str()).filter(|post| *post != xml).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow55BlockMint { _private: () }, xml, post, Some((0, xml.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}
fn collapse_hidden_timeline_rests_to_forward_in_measure(measure: &str) -> ((String, RenderXmlHiddenRestVirtualStats), Option<BlockTransformationReceipt>) {
    
    
    let output = {
    let mut out = String::with_capacity(measure.len());
    let mut stats = RenderXmlHiddenRestVirtualStats::default();
    let mut scan = 0_usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let pos = scan + rel;
        let Some(close_rel) = measure[pos..].find("</note>") else { break; };
        let end = pos + close_rel + "</note>".len();
        let note = &measure[pos..end];
        let is_hidden_rest = is_hidden_note(note) && note.contains("<rest") && !note.contains("<chord") && midi_from_note_fragment(note).is_none();
        out.push_str(&measure[scan..pos]);
        if is_hidden_rest {
            stats.changed_events += 1;
            let duration = extract_first_text_from_fragment(note, "duration").unwrap_or_else(|| "0".to_string());
            out.push_str("<forward><duration>");
            out.push_str(duration.trim());
            out.push_str("</duration></forward>");
        } else {
            out.push_str(note);
        }
        scan = end;
    }
    out.push_str(&measure[scan..]);
    (out, stats)
};
    
    let receipt = Some(output.0.as_str()).filter(|post| *post != measure).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow21BlockMint { _private: () }, measure, post, Some((0, measure.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}
fn build_render_xml_hidden_rest_forward_collapse_trial(
    xml: &str,
    audit: &RenderXmlHiddenRestDependencyAudit,
    _render_xml_path: &Path,
    render_state: &mut structural_write_authority::RenderSovereignState,
    render_root: structural_write_authority::RenderTrialRoot,
) -> anyhow::Result<RenderXmlHiddenRestForwardCollapseTrial> {
    let (before_events, before_backup_ok) = collect_render_xml_note_events(xml);
    let mut trial = RenderXmlHiddenRestForwardCollapseTrial {
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        visible_signature_before: render_visible_signature(&before_events, false),
        backup_forward_before_ok: before_backup_ok,
        ..RenderXmlHiddenRestForwardCollapseTrial::default()
    };
    if !audit.forward_possible || trial.hidden_rests_before == 0 {
        trial.attempted = false;
        trial.hidden_rests_after = trial.hidden_rests_before;
        trial.visible_pitched_after = trial.visible_pitched_before;
        trial.visible_rests_after = trial.visible_rests_before;
        trial.hidden_pitched_after = trial.hidden_pitched_before;
        trial.visible_signature_after = trial.visible_signature_before;
        trial.visible_signature_stable = true;
        trial.backup_forward_after_ok = trial.backup_forward_before_ok;
        trial.parse_ok = parse_musicxml(xml).is_ok();
        trial.export_semantics_ok = false;
        trial.verdict = "hidden_rest_forward_collapse_trial_not_attempted".to_string();
        trial.done_fragment = "RenderXmlHiddenRestForwardCollapseTrial: nicht ausgeführt; Audit gab Forward-Collapse nicht frei".to_string();
        trial.pipeline_line = format!(
            "RenderXmlHiddenRestForwardCollapseTrial v{}: nicht ausgeführt; forward_possible {}, versteckte Pausen {}; Master-XML unverändert, Render-XML unverändert.",
            env!("CARGO_PKG_VERSION"),
            if audit.forward_possible { "ja" } else { "nein" },
            trial.hidden_rests_before
        );
        return Ok(trial);
    }
    trial.attempted = true;
    let ((collapsed_xml, stats), collapse_receipt) = collapse_hidden_timeline_rests_to_forward(xml);
    let mut render_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    render_receipts.push_block(collapse_receipt, &collapsed_xml);
    trial.changed_events = stats.changed_events;
    let (after_events, after_backup_ok) = collect_render_xml_note_events(&collapsed_xml);
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.visible_signature_after = render_visible_signature(&after_events, false);
    trial.visible_signature_stable = trial.visible_signature_after == trial.visible_signature_before;
    trial.backup_forward_after_ok = after_backup_ok;
    trial.parse_ok = parse_musicxml(&collapsed_xml).is_ok();
    let invariants_ok = trial.changed_events == trial.hidden_rests_before
        && trial.hidden_rests_after == 0
        && trial.visible_signature_stable
        && trial.backup_forward_before_ok
        && trial.backup_forward_after_ok
        && trial.parse_ok
        && trial.visible_pitched_after == trial.visible_pitched_before
        && trial.visible_rests_after == trial.visible_rests_before
        && trial.hidden_pitched_after == trial.hidden_pitched_before;
    if invariants_ok {
        let published = render_state.adopt_trial_publication_transition(
            render_root,
            &collapsed_xml,
            render_receipts,
            structural_write_authority::RenderSinkKind::HiddenRestForwardCollapse,
            |artifact_path, source_xml, candidate_xml| {
                let tmp_path = artifact_path.with_file_name(format!(
                    "{}.hidden_rest_forward_trial.tmp",
                    artifact_path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("render.musicxml")
                ));
                crate::structural_write_authority::write_hidden_rest_forward_collapse_trial_authorized(
                    source_xml,
                    candidate_xml,
                    &tmp_path,
                )
                .with_context(|| {
                    format!(
                        "Konnte temporäre Render-XML für Hidden-Rest-Forward-Collapse nicht schreiben: {}",
                        tmp_path.display()
                    )
                })?;
                let export_semantics_ok = validate_musicxml_export_semantics(&tmp_path).is_ok();
                if !export_semantics_ok {
                    std::fs::remove_file(&tmp_path).ok();
                    return Ok::<bool, anyhow::Error>(false);
                }
                std::fs::rename(&tmp_path, artifact_path)
                    .or_else(|_| {
                        std::fs::copy(&tmp_path, artifact_path).map(|_| ())?;
                        std::fs::remove_file(&tmp_path).ok();
                        Ok::<(), std::io::Error>(())
                    })
                    .with_context(|| {
                        format!(
                            "Konnte Render-XML nach Hidden-Rest-Forward-Collapse nicht aktualisieren: {}",
                            artifact_path.display()
                        )
                    })?;
                Ok(true)
            },
        )?;
        trial.export_semantics_ok = published;
        trial.written = published;
        if published {
            crate::audit_only::destructive_edit_attribution::observe_selected_state_successor(
                xml,
                &collapsed_xml,
            );
        }
    } else {
        render_state.decline_transition(render_root, render_receipts);
    }
    trial.verdict = if trial.written {
        "hidden_rest_forward_collapse_trial_written_render_xml_only".to_string()
    } else if invariants_ok {
        "hidden_rest_forward_collapse_trial_blocked_by_export_guard".to_string()
    } else {
        "hidden_rest_forward_collapse_trial_blocked_by_invariance".to_string()
    };
    trial.examples.push(format!(
        "Forward-Collapse ersetzte {} versteckte Timeline-Pausen nur im Render-Pfad durch äquivalente <forward>-Dauern; Master-XML bleibt unverändert.",
        trial.changed_events
    ));
    trial.examples.push(format!(
        "Sichtbare klingende Noten {}/{}, sichtbare Pausen {}/{}, versteckte Pausen {}/{}.",
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.hidden_rests_before,
        trial.hidden_rests_after
    ));
    if !trial.written {
        trial.examples.push("Keine produktive Render-XML-Aktualisierung, weil mindestens ein Guard nicht vollständig grün war.".to_string());
    }
    trial.done_fragment = format!(
        "RenderXmlHiddenRestForwardCollapseTrial: geändert {}, geschrieben {}, Hidden-Rests {}→{}",
        trial.changed_events,
        if trial.written { "ja" } else { "nein" },
        trial.hidden_rests_before,
        trial.hidden_rests_after
    );
    trial.pipeline_line = format!(
        "RenderXmlHiddenRestForwardCollapseTrial v{}: Render-XML-only; changed {}, written {}; hidden rests {}→{}; sichtbar Noten {}/{}; sichtbare Pausen {}/{}; Signatur stabil {}; backup-forward {}/{}; parse/export {}/{}; Master-XML unverändert; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.changed_events,
        if trial.written { "ja" } else { "nein" },
        trial.hidden_rests_before,
        trial.hidden_rests_after,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.visible_rests_before,
        trial.visible_rests_after,
        if trial.visible_signature_stable { "ja" } else { "nein" },
        if trial.backup_forward_before_ok { "ja" } else { "nein" },
        if trial.backup_forward_after_ok { "ja" } else { "nein" },
        if trial.parse_ok { "ja" } else { "nein" },
        if trial.export_semantics_ok { "ja" } else { "nein" },
        trial.verdict
    );
    Ok(trial)
}

fn render_scope_expected_durations(events: &[RenderXmlNoteEvent]) -> BTreeMap<usize, i32> {
    let mut expected = BTreeMap::new();
    for event in events {
        if event.is_hidden || event.is_chord || event.duration <= 0 {
            continue;
        }
        let end = event.onset.saturating_add(event.duration);
        let entry = expected.entry(event.scope).or_insert(0_i32);
        *entry = (*entry).max(end);
    }
    for value in expected.values_mut() {
        if *value <= 0 {
            *value = 384;
        }
    }
    expected
}
fn visible_rest_chain_stats(rests: &[(i32, i32)]) -> (usize, usize) {
    if rests.is_empty() {
        return (0, 0);
    }
    let mut sorted = rests.to_vec();
    sorted.sort_by_key(|(onset, duration)| (*onset, *duration));
    let mut chains = 0_usize;
    let mut max_chain = 0_usize;
    let mut current_len = 0_usize;
    let mut current_start: Option<i32> = None;
    let mut current_end: Option<i32> = None;
    for (onset, duration) in sorted {
        let duration = duration.max(0);
        let end = onset.saturating_add(duration);
        let merge_stays_inside_one_beat = current_start
            .is_none_or(|start| end > start && start.div_euclid(96) == end.saturating_sub(1).div_euclid(96));
        let contiguous = current_end.is_some_and(|old_end| onset == old_end) && merge_stays_inside_one_beat;
        if contiguous {
            current_len += 1;
            current_end = Some(end);
        } else {
            if current_len > 0 {
                max_chain = max_chain.max(current_len);
            }
            chains += 1;
            current_len = 1;
            current_start = Some(onset);
            current_end = Some(end);
        }
    }
    max_chain = max_chain.max(current_len);
    (chains, max_chain)
}
fn count_materializable_rest_consolidation_chains(rests: &[(i32, i32)]) -> usize {
    if rests.len() < 2 {
        return 0;
    }
    let mut sorted = rests.to_vec();
    sorted.sort_by_key(|(onset, duration)| (*onset, *duration));
    let mut count = 0_usize;
    let mut chain: Vec<(i32, i32)> = Vec::new();
    for (onset, duration) in sorted {
        let contiguous = chain
            .last()
            .is_some_and(|(last_onset, last_duration)| last_onset.saturating_add((*last_duration).max(0)) == onset);
        let same_beat_if_merged = if contiguous {
            chain.first().is_some_and(|(first_onset, _)| {
                    let merged_end = onset.saturating_add(duration.max(0));
                    merged_end > *first_onset
                        && first_onset.div_euclid(96) == merged_end.saturating_sub(1).div_euclid(96)
                },
            )
        } else {
            false
        };
        if !chain.is_empty() && (!contiguous || !same_beat_if_merged) {
            let total: i32 = chain.iter().map(|(_, d)| (*d).max(0)).sum();
            if chain.len() >= 2
                && chain.iter().all(|(_, d)| *d <= 96)
                && duration_to_rest_type_and_dots(total).is_some()
            {
                count += 1;
            }
            chain.clear();
        }
        chain.push((onset, duration));
    }
    if !chain.is_empty() {
        let total: i32 = chain.iter().map(|(_, d)| (*d).max(0)).sum();
        if chain.len() >= 2
            && chain.iter().all(|(_, d)| *d <= 96)
            && duration_to_rest_type_and_dots(total).is_some()
        {
            count += 1;
        }
    }
    count
}
fn classify_visible_rest_readability_row(
    row: &RenderXmlVisibleRestScopeAudit,
    materializable_consolidation_chains: usize,
    primary_role_override: Option<bool>,
) -> (String, String, String) {
    let expected = row.expected_duration.max(1);
    let has_missing_rest_gap = row.missing_rest_duration > 0;
    let full_empty_rest = row.visible_pitched == 0
        && row.visible_rests == 1
        && row.rest_duration.saturating_mul(4) >= expected.saturating_mul(3);
    if full_empty_rest && !has_missing_rest_gap {
        return (
            "full_measure_empty_staff_rest_helpful".to_string(),
            "leave_as_is".to_string(),
            "low".to_string(),
        );
    }
    if has_missing_rest_gap && row.visible_pitched > 0 && row.visible_rests == 0 {
        return (
            "missing_visible_rest_gap".to_string(),
            "add_missing_visible_rest_candidate".to_string(),
            "medium".to_string(),
        );
    }
    if has_missing_rest_gap && row.visible_pitched > 0 {
        return (
            "mixed_missing_and_fragmented_rest_ledger".to_string(),
            "rewrite_local_voice_ledger_candidate".to_string(),
            "high".to_string(),
        );
    }
    if row.visible_pitched == 0 && row.visible_rests >= 2 {
        return (
            "aux_voice_nearly_empty".to_string(),
            "rewrite_local_voice_ledger_candidate".to_string(),
            "medium".to_string(),
        );
    }
    if materializable_consolidation_chains > 0 {
        return (
            "fragmented_visible_rest_chain_helpfully_consolidatable".to_string(),
            "consolidate_visible_rests_candidate".to_string(),
            "low".to_string(),
        );
    }
    if row.max_chain_len >= 2 && row.short_rests >= 2 {
        return (
            "fragmented_visible_rest_chain_not_materializable".to_string(),
            "leave_as_is".to_string(),
            "low".to_string(),
        );
    }
    if row.visible_pitched > 0 && row.visible_rests >= 3 && row.rest_note_ratio_milli >= 500 {
        return (
            "aux_voice_rest_orientation_stable".to_string(),
            "leave_as_is".to_string(),
            "low".to_string(),
        );
    }
    if row.visible_rests >= 2 {
        return match primary_role_override {
            Some(false) => (
                "aux_voice_rest_orientation_stable".to_string(),
                "leave_as_is".to_string(),
                "low".to_string(),
            ),
            Some(true) => (
                "main_voice_rest_orientation_stable".to_string(),
                "leave_as_is".to_string(),
                "high".to_string(),
            ),
            None => (
                "voice_rest_orientation_stable".to_string(),
                "leave_as_is".to_string(),
                "low".to_string(),
            ),
        };
    }
    (
        match primary_role_override {
            Some(false) => "aux_voice_rest_orientation_stable",
            Some(true) => "main_voice_rest_orientation_stable",
            None => "voice_rest_orientation_stable",
        }
        .to_string(),
        "leave_as_is".to_string(),
        "low".to_string(),
    )
}
fn build_render_xml_visible_rest_readability_audit_impl(
    xml: &str,
    source_label: &str,
    voice_plan: Option<&crate::structural_write_authority::FrozenSerializedVoicePlan>,
) -> RenderXmlVisibleRestReadabilityAudit {
    let (events, backup_forward_ok) = collect_render_xml_note_events(xml);
    let expected_by_scope = render_scope_expected_durations(&events);
    let mut audit = RenderXmlVisibleRestReadabilityAudit {
        source_label: source_label.to_string(),
        backup_forward_ok,
        parse_ok: parse_musicxml(xml).is_ok(),
        visible_signature: render_visible_signature(&events, false),
        ..RenderXmlVisibleRestReadabilityAudit::default()
    };
    let mut rows: BTreeMap<(usize, String, String), RenderXmlVisibleRestScopeAudit> = BTreeMap::new();
    let mut rest_positions: BTreeMap<(usize, String, String), Vec<(i32, i32)>> = BTreeMap::new();
    for event in &events {
        if event.midi.is_some() && !event.is_hidden {
            audit.visible_pitched += 1;
        }
        if event.is_rest && !event.is_hidden {
            audit.visible_rests += 1;
        }
        if event.midi.is_some() && event.is_hidden {
            audit.hidden_pitched += 1;
        }
        if event.is_rest && event.is_hidden {
            audit.hidden_rests += 1;
        }
        if event.is_hidden || (!event.is_rest && event.midi.is_none()) {
            continue;
        }
        if !(event.is_rest || event.midi.is_some()) {
            continue;
        }
        let key = (event.scope, event.staff.clone(), event.voice.clone());
        let expected_duration = expected_by_scope.get(&event.scope).copied().unwrap_or(384).max(1);
        let row = rows.entry(key.clone()).or_insert_with(|| RenderXmlVisibleRestScopeAudit {
            scope: event.scope,
            staff: event.staff.clone(),
            voice: event.voice.clone(),
            expected_duration,
            ..RenderXmlVisibleRestScopeAudit::default()
        });
        row.expected_duration = row.expected_duration.max(expected_duration);
        if event.is_rest {
            row.visible_rests += 1;
            row.rest_duration = row.rest_duration.saturating_add(event.duration.max(0));
            if event.duration > 0 && event.duration <= expected_duration / 4 {
                row.short_rests += 1;
            }
            rest_positions.entry(key).or_default().push((event.onset, event.duration.max(0)));
        } else if event.midi.is_some() {
            row.visible_pitched += 1;
            if !event.is_chord {
                row.note_duration = row.note_duration.saturating_add(event.duration.max(0));
            }
        }
    }
    let mut staffs_by_scope_voice: BTreeMap<(usize, String), BTreeSet<String>> = BTreeMap::new();
    for (scope, staff, voice) in rows.keys() {
        staffs_by_scope_voice
            .entry((*scope, voice.clone()))
            .or_default()
            .insert(staff.clone());
    }
    let mut top_scopes = Vec::new();
    for (key, mut row) in rows {
        row.ledger_duration = row.note_duration.saturating_add(row.rest_duration);
        row.missing_rest_duration = row.expected_duration.saturating_sub(row.ledger_duration).max(0);
        let cross_staff_same_voice = staffs_by_scope_voice
            .get(&(row.scope, row.voice.clone()))
            .is_some_and(|set| set.len() > 1);
        if cross_staff_same_voice && row.missing_rest_duration > 0 {
            row.missing_rest_duration = 0;
        }
        if row.visible_rests == 0 && row.missing_rest_duration == 0 {
            continue;
        }
        let rests = rest_positions.get(&key).cloned().unwrap_or_default();
        let (rest_chains, max_chain_len) = visible_rest_chain_stats(&rests);
        row.rest_chains = rest_chains;
        row.max_chain_len = max_chain_len;
        row.rest_note_ratio_milli = if row.note_duration > 0 {
            (row.rest_duration as i64 * 1000) / row.note_duration as i64
        } else if row.rest_duration > 0 {
            9999
        } else {
            0
        };
        row.fragmentation_score = (row.short_rests as i64 * 80)
            + (row.rest_chains.saturating_sub(1) as i64 * 60)
            + (row.max_chain_len.saturating_sub(1) as i64 * 45);
        row.pianistic_orientation_score = if row.missing_rest_duration > 0 {
            220 + ((row.missing_rest_duration as i64 * 240) / row.expected_duration.max(1) as i64)
        } else if row.max_chain_len >= 2 && row.short_rests >= 2 {
            180 + row.fragmentation_score
        } else if row.visible_rests > 0 {
            80
        } else {
            0
        };
        let materializable_consolidation_chains = count_materializable_rest_consolidation_chains(&rests);
        let primary_role_override = match voice_plan {
            Some(plan) => match plan.label_is_shadow_rest_primary_exempt(&row.voice) {
                Some(value) => Some(value),
                None => {
                    audit.parse_ok = false;
                    Some(false)
                }
            },
            None => None,
        };
        let (classification, action_candidate, mutation_risk) = classify_visible_rest_readability_row(
            &row,
            materializable_consolidation_chains,
            primary_role_override,
        );
        row.classification = classification;
        row.action_candidate = action_candidate;
        row.mutation_risk = mutation_risk;
        let risk_penalty = match row.mutation_risk.as_str() {
            "low" => 0,
            "medium" => 90,
            "high" => 240,
            _ => 150,
        };
        let action_bonus = match row.action_candidate.as_str() {
            "add_missing_visible_rest_candidate" => 320,
            "rewrite_local_voice_ledger_candidate" => 280,
            "suppress_aux_voice_rests_candidate" => 180,
            "consolidate_visible_rests_candidate" => 200,
            "leave_as_is" => 0,
            _ => 60,
        };
        row.visual_benefit_score = (row.visible_rests as i64 * 100)
            + row.fragmentation_score
            + row.pianistic_orientation_score
            + ((row.rest_duration as i64 * 80) / row.expected_duration.max(1) as i64)
            + ((row.missing_rest_duration as i64 * 180) / row.expected_duration.max(1) as i64)
            + action_bonus;
        row.score = row.visual_benefit_score - risk_penalty;
        *audit.class_counts.entry(row.classification.clone()).or_insert(0) += 1;
        *audit.action_counts.entry(row.action_candidate.clone()).or_insert(0) += 1;
        audit.total_rest_chains += row.rest_chains;
        if row.missing_rest_duration > 0 {
            audit.missing_visible_rest_candidates += 1;
            audit.missing_rest_duration_total = audit.missing_rest_duration_total.saturating_add(row.missing_rest_duration);
        }
        match row.classification.as_str() {
            "fragmented_visible_rest_chain_helpfully_consolidatable" => {
                audit.fragmented_chain_candidates += 1;
                audit.helpful_consolidation_candidates += 1;
            }
            "aux_voice_rest_burden" => audit.aux_voice_burden_candidates += 1,
            "aux_voice_nearly_empty" => audit.nearly_empty_voice_candidates += 1,
            _ => {}
        }
        match row.action_candidate.as_str() {
            "add_missing_visible_rest_candidate" => audit.missing_visible_rest_candidates += 0,
            "consolidate_visible_rests_candidate" => audit.consolidate_candidates += 1,
            "suppress_aux_voice_rests_candidate" => audit.suppress_candidates += 1,
            "rewrite_local_voice_ledger_candidate" => audit.rewrite_candidates += 1,
            "leave_as_is" => audit.leave_as_is += 1,
            _ => audit.blocked_or_unknown += 1,
        }
        top_scopes.push(row);
    }
    audit.checked_voice_scopes = top_scopes.len();
    top_scopes.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.visible_rests.cmp(&a.visible_rests))
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
    });
    top_scopes.truncate(12);
    audit.top_scopes = top_scopes;
    audit.verdict = if audit.parse_ok && audit.backup_forward_ok {
        "notation_rest_readability_planner_ok_no_mutation".to_string()
    } else {
        "notation_rest_readability_planner_blocked_by_parse_or_timeline".to_string()
    };
    audit.done_fragment = format!(
        "RenderXmlPianisticRestReadabilityPlanner: sichtbare Pausen {}, fehlende Pausen-Kandidaten {}, hilfreich zusammenfassbar {}, Rewrite {}, keine Mutation",
        audit.visible_rests,
        audit.missing_visible_rest_candidates,
        audit.helpful_consolidation_candidates,
        audit.rewrite_candidates
    );
    audit.pipeline_line = format!(
        "RenderXmlPianisticRestReadabilityPlanner v{}: audit-only auf {}; sichtbare Pausen {}, sichtbare klingende Noten {}, Voice-Scopes {}, fehlende-Pausen-Kandidaten {} ({} divisions), Restketten {}, hilfreich zusammenfassbar {}, Klassen [{}], Aktionen [{}]; parse/back-forward {}/{}; Signatur {}; Urteil {}; keine Mutation, keine PDF-Verbesserungsbehauptung.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.visible_rests,
        audit.visible_pitched,
        audit.checked_voice_scopes,
        audit.missing_visible_rest_candidates,
        audit.missing_rest_duration_total,
        audit.total_rest_chains,
        audit.helpful_consolidation_candidates,
        count_summary(&audit.class_counts, 8),
        count_summary(&audit.action_counts, 8),
        if audit.parse_ok { "ok" } else { "fail" },
        if audit.backup_forward_ok { "ok" } else { "fail" },
        audit.visible_signature,
        audit.verdict
    );
    audit.examples.push("Audit-only: Pausen werden als pianistische Orientierungssignale im Voice-Ledger geplant; Render-XML und Master-XML bleiben unverändert.".to_string());
    audit.examples.push("Der Planner optimiert nicht auf möglichst wenige Pausen, sondern auf lesbare Zählzeit, Stimme, Einsatz und Handlogik bei identischer klingender Musik.".to_string());
    audit.examples.push("Ein späterer Trial darf fehlende sichtbare Pausen ergänzen oder Pausenketten zusammenfassen, wenn Gesamtdauer, backup/forward, RHY01/RHY08 und final 0/0/0/0 erhalten bleiben.".to_string());
    audit.examples.push("Takt-/Bereichsnummern bleiben reine berechnete Reportdaten; keine konkrete Taktlogik, keine mXX-/Takt-Spezialnamen, keine feste Kandidatenliste.".to_string());
    audit
}
fn build_render_xml_visible_rest_readability_audit(
    xml: &str,
    source_label: &str,
) -> RenderXmlVisibleRestReadabilityAudit {
    build_render_xml_visible_rest_readability_audit_impl(xml, source_label, None)
}

fn render_visible_sounding_signature(events: &[RenderXmlNoteEvent]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for event in events {
        if event.is_hidden || event.midi.is_none() {
            continue;
        }
        event.scope.hash(&mut hasher);
        event.staff.hash(&mut hasher);
        event.voice.hash(&mut hasher);
        event.onset.hash(&mut hasher);
        event.duration.hash(&mut hasher);
        event.is_chord.hash(&mut hasher);
        event.midi.hash(&mut hasher);
    }
    hasher.finish()
}
fn render_hidden_pitched_timeline_signature(events: &[RenderXmlNoteEvent]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for event in events.iter().filter(|event| event.is_hidden && event.midi.is_some()) {
        event.scope.hash(&mut hasher);
        event.staff.hash(&mut hasher);
        event.voice.hash(&mut hasher);
        event.onset.hash(&mut hasher);
        event.duration.hash(&mut hasher);
        event.is_chord.hash(&mut hasher);
        event.midi.hash(&mut hasher);
        event.has_visible_dependency.hash(&mut hasher);
    }
    hasher.finish()
}
fn render_visible_sounding_signature_dedup(events: &[RenderXmlNoteEvent]) -> u64 {
    let mut keys: BTreeSet<(usize, String, i32, i32, i32)> = BTreeSet::new();
    for event in events {
        if event.is_hidden || event.is_rest {
            continue;
        }
        let Some(midi) = event.midi else { continue; };
        keys.insert((event.scope, event.staff.clone(), event.onset, event.duration, midi));
    }
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for key in keys {
        key.hash(&mut hasher);
    }
    hasher.finish()
}
fn replace_xml_tag_text(fragment: &str, tag: &str, value: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let Some(start) = fragment.find(&open) else { return fragment.to_string(); };
    let value_start = start + open.len();
    let Some(end_rel) = fragment[value_start..].find(&close) else { return fragment.to_string(); };
    let value_end = value_start + end_rel;
    let mut out = String::new();
    out.push_str(&fragment[..value_start]);
    out.push_str(value);
    out.push_str(&fragment[value_end..]);
    out
}
fn duration_to_rest_type_and_dots(duration: i32) -> Option<(&'static str, usize)> {
    match duration {
        384 => Some(("whole", 0)),
        288 => Some(("half", 1)),
        192 => Some(("half", 0)),
        144 => Some(("quarter", 1)),
        96 => Some(("quarter", 0)),
        72 => Some(("eighth", 1)),
        48 => Some(("eighth", 0)),
        36 => Some(("16th", 1)),
        24 => Some(("16th", 0)),
        18 => Some(("32nd", 1)),
        12 => Some(("32nd", 0)),
        _ => None,
    }
}

#[derive(Debug)]
struct GeneratedRestDurationReceipt {
    receipt: BlockTransformationReceipt,
    pre_fragment: String,
    post_fragment: String,
    located: usize,
}

fn collect_visible_rest_spans_for_repair(xml: &str) -> Vec<RenderXmlRestFragmentSpan> {
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
                    if !is_chord {
                        last_anchor_onset = onset;
                        cursor = cursor.saturating_add(duration);
                    }
                    let is_rest = note.contains("<rest");
                    let is_hidden = is_hidden_note(note);
                    let safe_fragment = is_rest
                        && !is_hidden
                        && !is_chord
                        && duration > 0
                        && !note.contains("<time-modification")
                        && !render_note_has_visible_dependency(note);
                    if safe_fragment {
                        let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "?".to_string());
                        let voice = extract_first_text_from_fragment(note, "voice").unwrap_or_else(|| "?".to_string());
                        spans.push(RenderXmlRestFragmentSpan {
                            scope: scope_number,
                            staff,
                            voice,
                            onset,
                            duration,
                            start: measure_start + pos,
                            end: measure_start + end,
                            fragment: note.to_string(),
                        });
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
#[derive(Debug, Clone)]
struct CarrierPreservingInitialRestProjectionCandidate {
    rest: RenderXmlRestConsolidationCandidate,
    parts: Vec<i32>,
    insertion_pos: usize,
}

fn hidden_chord_carrier_group_end(
    xml: &str,
    rest_end: usize,
    limit: usize,
) -> Option<usize> {
    let mut cursor = rest_end;
    let mut seen = false;
    loop {
        while cursor < limit && xml.as_bytes()[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor >= limit || !xml[cursor..limit].starts_with("<note") {
            break;
        }
        let end = cursor + xml[cursor..limit].find("</note>")? + "</note>".len();
        let note = &xml[cursor..end];
        if !(note.contains("<chord") || note.contains("<chord/>")) {
            break;
        }
        if !is_hidden_note(note) || midi_from_note_fragment(note).is_none() {
            return None;
        }
        seen = true;
        cursor = end;
    }
    seen.then_some(cursor)
}

#[cfg(test)]
fn hide_rest_as_technical_carrier(fragment: &str) -> Option<String> {
    if !fragment.contains("<rest") || fragment.contains("<pitch>") {
        return None;
    }
    let normalized_print_object = fragment
        .replace("print-object=\"yes\"", "print-object=\"no\"")
        .replace("print-object='yes'", "print-object='no'");
    let (hidden, receipt) = hide_note_fragment_print_object_no(&normalized_print_object);
    let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
    probe.push_generated_block(receipt, &normalized_print_object, &hidden, None);
    probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    if hidden.contains("print-spacing=\"yes\"") {
        Some(hidden.replace("print-spacing=\"yes\"", "print-spacing=\"no\""))
    } else if hidden.contains("print-spacing='yes'") {
        Some(hidden.replace("print-spacing='yes'", "print-spacing='no'"))
    } else if hidden.contains("print-spacing=\"no\"") || hidden.contains("print-spacing='no'") {
        Some(hidden)
    } else {
        Some(hidden.replacen("<note", "<note print-spacing=\"no\"", 1))
    }
}

fn build_carrier_preserving_initial_rest_projection_candidates(
    xml: &str,
) -> Vec<CarrierPreservingInitialRestProjectionCandidate> {
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let (events, _) = collect_render_xml_note_events(xml);
    let mut by_lane: BTreeMap<(usize, String, String), Vec<RenderXmlRestFragmentSpan>> =
        BTreeMap::new();
    for span in collect_visible_rest_spans_for_repair(xml) {
        by_lane
            .entry((span.scope, span.staff.clone(), span.voice.clone()))
            .or_default()
            .push(span);
    }

    let mut out = Vec::new();
    for ((scope, staff, voice), mut spans) in by_lane {
        let Some((expected, beat_unit)) = metric_by_scope.get(&scope).copied() else {
            continue;
        };
        spans.sort_by_key(|span| (span.onset, span.start));
        let Some(first) = spans.first() else {
            continue;
        };
        if first.onset != 0 {
            continue;
        }

        let mut chain = vec![first.clone()];
        for span in spans.iter().skip(1) {
            let contiguous = chain
                .last()
                .is_some_and(|last| last.onset.saturating_add(last.duration.max(0)) == span.onset);
            if contiguous {
                chain.push(span.clone());
            } else {
                break;
            }
        }
        if chain.len() < 2 {
            continue;
        }

        let total_duration: i32 = chain.iter().map(|span| span.duration.max(0)).sum();
        if total_duration <= 0 || total_duration >= expected {
            continue;
        }

        // Visible rest notation represents voice silence, not a technical host gap.
        // Cross-staff pitched events of the same voice therefore also end the silent span.
        let first_visible_pitch = events
            .iter()
            .filter(|event| {
                event.scope == scope
                    && event.voice == voice
                    && !event.is_hidden
                    && !event.is_chord
                    && event.midi.is_some()
            })
            .map(|event| event.onset)
            .min();
        if first_visible_pitch != Some(total_duration) {
            continue;
        }

        let Some((_, measure_end, _)) = find_measure_bounds_containing_pos(xml, first.start) else {
            continue;
        };
        let mut has_carrier = false;
        let mut insertion_pos = chain.last().map_or(first.end, |span| span.end);
        let mut carrier_safe = true;
        for span in &chain {
            if !note_is_followed_by_chord_note(xml, span.end, measure_end) {
                continue;
            }
            has_carrier = true;
            let Some(group_end) = hidden_chord_carrier_group_end(xml, span.end, measure_end) else {
                carrier_safe = false;
                break;
            };
            if chain
                .last()
                .is_some_and(|last| last.start == span.start && last.end == span.end)
            {
                insertion_pos = group_end;
            }
        }
        if !has_carrier
            || !carrier_safe
            || xml[first.start..insertion_pos].contains("<backup")
            || xml[first.start..insertion_pos].contains("<forward")
        {
            continue;
        }

        let Some(decision) = crate::model::metric_rest_notation::decide_rest_span_consolidation(
            crate::model::metric_rest_notation::RestSpanConsolidationFacts {
                start: 0,
                duration: i64::from(total_duration),
                expected_measure_duration: i64::from(expected),
                beat_duration: i64::from(beat_unit),
                source_fragment_count: chain.len(),
                has_chord_anchor_dependency: true,
                all_source_fragments_short: chain.iter().all(|span| span.duration <= beat_unit),
            },
            crate::model::metric_rest_notation::RestSpanConsolidationProjection::
                CarrierPreservingMetricCompaction,
        ) else {
            continue;
        };
        let parts = decision
            .parts
            .into_iter()
            .map(i32::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default();
        if parts.len() <= 1
            || parts
                .iter()
                .any(|duration| duration_to_rest_type_and_dots(*duration).is_none())
        {
            continue;
        }

        out.push(CarrierPreservingInitialRestProjectionCandidate {
            rest: RenderXmlRestConsolidationCandidate {
                scope,
                staff,
                voice,
                spans: chain,
                total_duration,
                first_onset: 0,
                score: i64::from(total_duration),
            },
            parts,
            insertion_pos,
        });
    }
    out.sort_by(|a, b| {
        b.rest
            .spans
            .len()
            .cmp(&a.rest.spans.len())
            .then_with(|| a.rest.scope.cmp(&b.rest.scope))
    });
    out
}

fn carrier_projection_invariants_hold(
    before: &str,
    after: &str,
    candidate: &CarrierPreservingInitialRestProjectionCandidate,
) -> bool {
    let (before_events, before_balance) = collect_render_xml_note_events(before);
    let (after_events, after_balance) = collect_render_xml_note_events(after);
    let count = |events: &[RenderXmlNoteEvent], hidden: bool, rest: bool, pitched: bool| {
        events
            .iter()
            .filter(|event| {
                event.is_hidden == hidden
                    && event.is_rest == rest
                    && event.midi.is_some() == pitched
            })
            .count()
    };
    let removed_visible_rests = candidate
        .rest
        .spans
        .len()
        .saturating_sub(candidate.parts.len());
    before_balance
        && after_balance
        && parse_musicxml(after).is_ok()
        && render_visible_sounding_signature(&before_events)
            == render_visible_sounding_signature(&after_events)
        && render_hidden_pitched_timeline_signature(&before_events)
            == render_hidden_pitched_timeline_signature(&after_events)
        && count(&before_events, false, false, true) == count(&after_events, false, false, true)
        && count(&before_events, true, false, true) == count(&after_events, true, false, true)
        && count(&after_events, false, true, false).saturating_add(removed_visible_rests)
            == count(&before_events, false, true, false)
        && count(&after_events, true, true, false)
            == count(&before_events, true, true, false)
                .saturating_add(candidate.rest.spans.len())
}

fn build_rest_consolidation_candidates(xml: &str) -> Vec<RenderXmlRestConsolidationCandidate> {
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let spans = collect_visible_rest_spans_for_repair(xml);
    let mut by_scope_voice: BTreeMap<(usize, String, String), Vec<RenderXmlRestFragmentSpan>> = BTreeMap::new();
    for span in spans {
        by_scope_voice
            .entry((span.scope, span.staff.clone(), span.voice.clone()))
            .or_default()
            .push(span);
    }
    let mut candidates = Vec::new();
    for ((scope, _staff, _voice), mut group) in by_scope_voice {
        let Some((expected, beat_unit)) = metric_by_scope.get(&scope).copied() else {
            continue;
        };
        group.sort_by_key(|span| (span.onset, span.duration, span.start));
        let mut chain: Vec<RenderXmlRestFragmentSpan> = Vec::new();
        for span in group {
            let contiguous = chain
                .last()
                .is_some_and(|last| last.onset.saturating_add(last.duration) == span.onset);
            let same_beat_if_merged = if contiguous {
                chain
                    .first()
                    .is_none_or(|first| {
                            let merged_end = span.onset.saturating_add(span.duration.max(0));
                            merged_end > first.onset
                                && first.onset.div_euclid(beat_unit)
                                    == merged_end.saturating_sub(1).div_euclid(beat_unit)
                        },
                    )
            } else {
                false
            };
            if !chain.is_empty() && (!contiguous || !same_beat_if_merged) {
                push_rest_consolidation_candidate(
                    &mut candidates,
                    &chain,
                    expected,
                    beat_unit,
                );
                chain.clear();
            }
            chain.push(span);
        }
        push_rest_consolidation_candidate(
            &mut candidates,
            &chain,
            expected,
            beat_unit,
        );
    }
    candidates.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| b.total_duration.cmp(&a.total_duration))
            .then_with(|| b.spans.len().cmp(&a.spans.len()))
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
    });
    candidates
}
fn push_rest_consolidation_candidate(
    out: &mut Vec<RenderXmlRestConsolidationCandidate>,
    chain: &[RenderXmlRestFragmentSpan],
    expected: i32,
    beat_unit: i32,
) {
    if chain.len() < 2 {
        return;
    }
    let total_duration: i32 = chain.iter().map(|span| span.duration.max(0)).sum();
    let Some(first) = chain.first() else { return; };
    let all_short = chain.iter().all(|span| span.duration <= 96);
    if crate::model::metric_rest_notation::decide_rest_span_consolidation(
        crate::model::metric_rest_notation::RestSpanConsolidationFacts {
            start: i64::from(first.onset),
            duration: i64::from(total_duration),
            expected_measure_duration: i64::from(expected),
            beat_duration: i64::from(beat_unit),
            source_fragment_count: chain.len(),
            has_chord_anchor_dependency: false,
            all_source_fragments_short: all_short,
        },
        crate::model::metric_rest_notation::RestSpanConsolidationProjection::SinglePartShortBeatLocal,
    )
    .is_none()
        || duration_to_rest_type_and_dots(total_duration).is_none()
    {
        return;
    }
    let beat_aligned_bonus = if first.onset % 96 == 0 && total_duration % 96 == 0 { 120 } else { 0 };
    out.push(RenderXmlRestConsolidationCandidate {
        scope: first.scope,
        staff: first.staff.clone(),
        voice: first.voice.clone(),
        spans: chain.to_vec(),
        total_duration,
        first_onset: first.onset,
        score: (chain.len() as i64 * 100) + (total_duration as i64 / 2) + beat_aligned_bonus,
    });
}

#[cfg(test)]
fn apply_pianistic_rest_consolidation(
    xml: &str,
    max_actions: usize,
) -> (String, Vec<RenderXmlPianisticRestRepairAction>, usize) {
    let (out, actions, seen, receipts) = apply_pianistic_rest_consolidation_prepared(xml, max_actions);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    (out, actions, seen)
}


fn hide_note_fragment_print_object_no(fragment: &str) -> (String, Option<BlockTransformationReceipt>) {
    
    
    let output = (|| {
    if fragment.contains("print-object=\"no\"") || fragment.contains("print-object='no'") {
        return fragment.to_string();
    }
    let Some(note_pos) = fragment.find("<note") else { return fragment.to_string(); };
    let Some(close_rel) = fragment[note_pos..].find('>') else { return fragment.to_string(); };
    let open_end = note_pos + close_rel;
    let mut out = String::new();
    out.push_str(&fragment[..open_end]);
    out.push_str(" print-object=\"no\"");
    out.push_str(&fragment[open_end..]);
    out
})();
    
    let receipt = Some(output.as_str()).filter(|post| *post != fragment).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow20BlockMint { _private: () }, fragment, post, Some((0, fragment.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}
fn note_is_followed_by_chord_note(xml: &str, note_end: usize, limit: usize) -> bool {
    if note_end >= xml.len() || note_end >= limit { return false; }
    let mut i = note_end;
    while i < limit {
        let Some(ch) = xml[i..].chars().next() else { break; };
        if ch.is_whitespace() { i += ch.len_utf8(); continue; }
        break;
    }
    if i >= limit || !xml[i..limit].starts_with("<note") { return false; }
    let Some(close_rel) = xml[i..limit].find("</note>") else { return false; };
    let note = &xml[i..i + close_rel + "</note>".len()];
    note.contains("<chord") || note.contains("<chord/>")
}
fn replace_forward_fragment_duration(fragment: &str, new_duration: i32) -> String {
    if new_duration <= 0 {
        String::new()
    } else {
        replace_xml_tag_text(fragment, "duration", &new_duration.to_string())
    }
}
fn find_measure_bounds_containing_pos(xml: &str, pos: usize) -> Option<(usize, usize, usize)> {
    let mut scan = 0_usize;
    while let Some((start, end, scope)) = find_next_measure_bounds(xml, scan) {
        if start <= pos && pos < end {
            return Some((start, end, scope));
        }
        scan = end;
    }
    None
}
fn fragmented_rest_chain_final_guard(trial: &RenderXmlPianisticRestRepairTrial) -> bool {
    trial.consolidation_adopted > 0
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.backup_forward_after_ok
        && trial.parse_after_ok
}

fn build_fragmented_rest_chain_transactional_write_trial(
    xml: &str,
    source_label: &str,
    _output_path: &Path,
) -> anyhow::Result<(RenderXmlPianisticRestRepairTrial, Option<String>, crate::master_state_identity::RenderTransitionReceipts)> {
    use crate::master_state_identity::{RenderDiscardReason, RenderTransitionReceipts};
    let mut main_chain = RenderTransitionReceipts::new();
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut trial = RenderXmlPianisticRestRepairTrial {
        source_label: source_label.to_string(),
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        backup_forward_before_ok: before_backup_forward_ok,
        parse_before_ok: parse_musicxml(xml).is_ok(),
        ..RenderXmlPianisticRestRepairTrial::default()
    };
    if !trial.parse_before_ok || !trial.backup_forward_before_ok {
        trial.verdict = "fragmented_rest_chain_transactional_write_blocked_by_input_guard".to_string();
        trial.done_fragment = "FragmentedRestChainTransactionalWrite: blockiert vor Mutation; Eingangs-Parse/BackupForward nicht gruen".to_string();
        trial.pipeline_line = format!(
            "FragmentedRestChainTransactionalWrite v{}: blockiert vor Mutation; parse/back-forward vorab {}/{}; RenderXmlWrite nein; MusicMutation nein.",
            env!("CARGO_PKG_VERSION"),
            yes_no(trial.parse_before_ok),
            yes_no(trial.backup_forward_before_ok)
        );
        return Ok((trial, None, main_chain));
    }
    trial.attempted = true;
    let mut current_xml = xml.to_string();
    let mut actions: Vec<RenderXmlPianisticRestRepairAction> = Vec::new();
    let carrier_seen =
        build_carrier_preserving_initial_rest_projection_candidates(&current_xml).len();
    trial.consolidation_candidates_seen = carrier_seen
        .saturating_add(build_rest_consolidation_candidates(&current_xml).len());
    let mut processed_carriers = BTreeSet::<(usize, String, String, i32)>::new();
    for _ in 0..8 {
        let Some(candidate) =
            build_carrier_preserving_initial_rest_projection_candidates(&current_xml)
                .into_iter()
                .find(|candidate| {
                    !processed_carriers.contains(&(
                        candidate.rest.scope,
                        candidate.rest.staff.clone(),
                        candidate.rest.voice.clone(),
                        candidate.rest.total_duration,
                    ))
                })
        else {
            break;
        };
        processed_carriers.insert((
            candidate.rest.scope,
            candidate.rest.staff.clone(),
            candidate.rest.voice.clone(),
            candidate.rest.total_duration,
        ));
        trial.consolidation_attempted += 1;
        let (projected, local_chain) = match apply_layer_balanced_single_rest_consolidation_candidate_xml(
            &current_xml,
            &candidate.rest,
            Some((&candidate.parts, candidate.insertion_pos)),
        ) { Ok(value) => value, Err(error) => { main_chain.discard_unadopted(RenderDiscardReason::LosingCandidate); crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures); return Err(error); } };
        let adopted = projected
            .as_ref()
            .is_some_and(|after| carrier_projection_invariants_hold(&current_xml, after, &candidate));
        if adopted {
            current_xml = projected.expect("checked carrier projection");
            main_chain.append(local_chain);
            trial.consolidation_adopted += 1;
        } else {
            local_chain.discard_unadopted(RenderDiscardReason::LosingCandidate);
            trial.consolidation_rejected += 1;
        }
        actions.push(RenderXmlPianisticRestRepairAction {
            scope: candidate.rest.scope,
            staff: candidate.rest.staff,
            voice: candidate.rest.voice,
            verdict: if adopted {
                "adopted_carrier_preserving_visible_rest_projection"
            } else {
                "rejected_carrier_projection_invariant_guard"
            }
            .to_string(),
        });
    }
    let mut processed_keys: BTreeSet<(usize, String, String, i32, i32)> = BTreeSet::new();
    for _ in 0..16 {
        let candidates = build_rest_consolidation_candidates(&current_xml);
        let Some(candidate) = candidates.into_iter().find(|candidate| {
            let key = (
                candidate.scope,
                candidate.staff.clone(),
                candidate.voice.clone(),
                candidate.first_onset,
                candidate.total_duration,
            );
            !processed_keys.contains(&key)
        }) else { break; };
        let key = (
            candidate.scope,
            candidate.staff.clone(),
            candidate.voice.clone(),
            candidate.first_onset,
            candidate.total_duration,
        );
        processed_keys.insert(key);
        trial.consolidation_attempted += 1;

        // Candidate detection stays policy-free: chord-anchored rest spans remain
        // observable evidence. This transactional writer must nevertheless ask
        // the existing RestSpanConsolidation decision owner before it may
        // materialize a SinglePartShortBeatLocal projection.
        let chord_anchor_dependency = candidate.spans.iter().any(|span| {
            find_measure_bounds_containing_pos(&current_xml, span.start).is_some_and(
                |(_, measure_end, _)| {
                    note_is_followed_by_chord_note(&current_xml, span.end, measure_end)
                },
            )
        });
        let metric_by_scope = render_scope_metric_expected_and_beat_map(&current_xml);
        let Some((expected, beat_unit)) = metric_by_scope.get(&candidate.scope).copied() else {
            trial.consolidation_rejected += 1;
            actions.push(RenderXmlPianisticRestRepairAction {
                scope: candidate.scope,
                staff: candidate.staff.clone(),
                voice: candidate.voice.clone(),
                verdict: "rejected_missing_metric_basis".to_string(),
            });
            continue;
        };
        let all_short = candidate.spans.iter().all(|span| span.duration <= 96);
        if crate::model::metric_rest_notation::decide_rest_span_consolidation(
            crate::model::metric_rest_notation::RestSpanConsolidationFacts {
                start: i64::from(candidate.first_onset),
                duration: i64::from(candidate.total_duration),
                expected_measure_duration: i64::from(expected),
                beat_duration: i64::from(beat_unit),
                source_fragment_count: candidate.spans.len(),
                has_chord_anchor_dependency: chord_anchor_dependency,
                all_source_fragments_short: all_short,
            },
            crate::model::metric_rest_notation::RestSpanConsolidationProjection::SinglePartShortBeatLocal,
        )
        .is_none()
        {
            trial.consolidation_rejected += 1;
            actions.push(RenderXmlPianisticRestRepairAction {
                scope: candidate.scope,
                staff: candidate.staff.clone(),
                voice: candidate.voice.clone(),
                verdict: if chord_anchor_dependency {
                    "rejected_chord_anchor_dependency"
                } else {
                    "rejected_by_rest_span_decision_owner"
                }
                .to_string(),
            });
            continue;
        }

        let (local_before_events, local_before_backup_ok) = collect_render_xml_note_events(&current_xml);
        let local_before_visible_pitched = local_before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
        let local_before_hidden_pitched = local_before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
        let local_before_visible_rests = local_before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
        let local_before_sounding = render_visible_sounding_signature(&local_before_events);
        let (candidate_post, local_chain) = match apply_layer_balanced_single_rest_consolidation_candidate_xml(&current_xml, &candidate, None) { Ok(value) => value, Err(error) => { main_chain.discard_unadopted(RenderDiscardReason::LosingCandidate); crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures); return Err(error); } };
        let Some(candidate_xml) = candidate_post else {
            local_chain.discard_unadopted(RenderDiscardReason::LosingCandidate);
            actions.push(RenderXmlPianisticRestRepairAction {
                scope: candidate.scope,
                staff: candidate.staff.clone(),
                voice: candidate.voice.clone(),
                verdict: "rejected_xml_layer_balance_locator_failed".to_string(),
            });
            trial.consolidation_rejected += 1;
            continue;
        };
        let (local_after_events, local_after_backup_ok) = collect_render_xml_note_events(&candidate_xml);
        let local_after_visible_pitched = local_after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
        let local_after_hidden_pitched = local_after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
        let local_after_visible_rests = local_after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
        let local_after_sounding = render_visible_sounding_signature(&local_after_events);
        let expected_delta = candidate.spans.len().saturating_sub(1);
        let invariant_ok = local_before_backup_ok
            && local_after_backup_ok
            && parse_musicxml(&candidate_xml).is_ok()
            && local_before_visible_pitched == local_after_visible_pitched
            && local_before_hidden_pitched == local_after_hidden_pitched
            && local_before_sounding == local_after_sounding
            && local_after_visible_rests.saturating_add(expected_delta) == local_before_visible_rests;
        if invariant_ok {
            current_xml = candidate_xml;
            main_chain.append(local_chain);
            trial.consolidation_adopted += 1;
        } else {
            local_chain.discard_unadopted(RenderDiscardReason::LosingCandidate);
            trial.consolidation_rejected += 1;
        }
        actions.push(RenderXmlPianisticRestRepairAction {
            scope: candidate.scope,
            staff: candidate.staff.clone(),
            voice: candidate.voice.clone(),
            verdict: if invariant_ok { "adopted_transactionally" } else { "rejected_by_transactional_invariant_guard" }.to_string(),
        });
    }
    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&current_xml);
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.visible_signature_after = render_visible_signature(&after_events, false);
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.parse_after_ok = parse_musicxml(&current_xml).is_ok();
    trial.visible_rest_delta = trial.visible_rests_after as i64 - trial.visible_rests_before as i64;
    let remaining = build_carrier_preserving_initial_rest_projection_candidates(&current_xml)
        .len()
        .saturating_add(build_rest_consolidation_candidates(&current_xml).len());
    let final_ok = fragmented_rest_chain_final_guard(&trial);
    if final_ok {
        trial.verdict = if remaining == 0 {
            "fragmented_rest_chain_transactional_write_green_render_xml_only".to_string()
        } else {
            "fragmented_rest_chain_transactional_write_partial_green_render_xml_only".to_string()
        };
    } else {
        trial.verdict = if trial.consolidation_attempted == 0 {
            "fragmented_rest_chain_transactional_write_no_candidate".to_string()
        } else {
            "fragmented_rest_chain_transactional_write_rejected_by_final_guard".to_string()
        };
    }
    trial.actions = actions;
    trial.examples.push("Jede Kette wird einzeln transaktional geprüft; eine rote Kette blockiert nicht automatisch die anderen.".to_string());
    trial.examples.push("CarrierProjection: hidden Chord-Anker behalten Timeline und Payload; die sichtbare metrische Restprojektion wird davon cursor-neutral getrennt.".to_string());
    trial.examples.push("Layer-Balance: Die Zielpause wird verlängert und die zugehörige Forward-Zeit derselben XML-Zeitspur gekürzt; die zu ersetzende sichtbare Restspur wird Forward oder bei Chord-Abhängigkeit unsichtbarer Anker.".to_string());
    trial.examples.push("Keine Guard-Lockerung: Parse, BackupForward und Sounding-Signatur muessen pro Transaktion und final gruen bleiben.".to_string());
    trial.done_fragment = format!(
        "FragmentedRestChainTransactionalWrite: Kandidaten {}, versucht {}, übernommen {}, verworfen {}, Restketten verbleibend {}, sichtbare Pausen {}→{}, parse/back-forward/sounding {}/{}/{}; RenderXmlWrite {}; MusicMutation nein",
        trial.consolidation_candidates_seen,
        trial.consolidation_attempted,
        trial.consolidation_adopted,
        trial.consolidation_rejected,
        remaining,
        trial.visible_rests_before,
        trial.visible_rests_after,
        yes_no(trial.parse_after_ok),
        yes_no(trial.backup_forward_after_ok),
        yes_no(trial.sounding_signature_stable),
        yes_no(trial.written),
    );
    trial.pipeline_line = format!(
        "FragmentedRestChainTransactionalWrite v{}: Source {}; Kandidaten {}, versucht {}, adopted {}, rejected {}, remaining {}; visible_rests {}→{}; visible_pitched {}→{}; hidden_pitched {}→{}; hidden_rests {}→{}; parse/back-forward/sounding {}/{}/{}; written {}; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.consolidation_candidates_seen,
        trial.consolidation_attempted,
        trial.consolidation_adopted,
        trial.consolidation_rejected,
        remaining,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.hidden_pitched_before,
        trial.hidden_pitched_after,
        trial.hidden_rests_before,
        trial.hidden_rests_after,
        yes_no(trial.parse_after_ok),
        yes_no(trial.backup_forward_after_ok),
        yes_no(trial.sounding_signature_stable),
        yes_no(trial.written),
        trial.verdict,
    );
    if final_ok { Ok((trial, Some(current_xml), main_chain)) } else {
        main_chain.discard_unadopted(RenderDiscardReason::LosingCandidate);
        Ok((trial, None, RenderTransitionReceipts::new()))
    }
}
#[cfg(test)]
fn apply_structural_transactional_rest_coalesce_render_write(xml: &str, max_transactions: usize) -> (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize) {
    let (out, actions, seen, receipts) = apply_structural_transactional_rest_coalesce_render_write_prepared(xml, max_transactions);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    (out, actions, seen)
}

fn collect_render_timeline_fragments_in_segment(segment: &str, abs_start: usize, scope_number: usize) -> Vec<RenderXmlTimelineFragmentSpan> {
    let mut fragments = Vec::new();
    let mut scan = 0_usize;
    let mut cursor = 0_i32;
    let mut last_anchor_onset = 0_i32;
    loop {
        let next_note = segment[scan..].find("<note").map(|p| scan + p);
        let next_backup = segment[scan..].find("<backup").map(|p| scan + p);
        let next_forward = segment[scan..].find("<forward").map(|p| scan + p);
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
                let Some(close_rel) = segment[pos..].find("</note>") else { break; };
                let end = pos + close_rel + "</note>".len();
                let note = &segment[pos..end];
                let duration = extract_first_text_from_fragment(note, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "?".to_string());
                let voice = extract_first_text_from_fragment(note, "voice").unwrap_or_else(|| "?".to_string());
                let is_chord = note.contains("<chord") || note.contains("<chord/>");
                let onset = if is_chord { last_anchor_onset } else { cursor };
                if !is_chord {
                    last_anchor_onset = onset;
                    cursor = cursor.saturating_add(duration);
                }
                let is_rest = note.contains("<rest");
                let is_hidden = is_hidden_note(note);
                let midi = midi_from_note_fragment(note);
                let has_visible_dependency = render_note_has_visible_dependency(note);
                fragments.push(RenderXmlTimelineFragmentSpan {
                    scope: scope_number,
                    start: abs_start + pos,
                    end: abs_start + end,
                    onset,
                    duration,
                    staff,
                    voice,
                    is_note: true,
                    is_forward: false,
                    is_rest,
                    is_hidden,
                    is_chord,
                    is_visible_rest: is_rest && !is_hidden && !is_chord,
                    is_visible_pitched: midi.is_some() && !is_hidden,
                    has_visible_dependency,
                    fragment: note.to_string(),
                });
                scan = end;
            }
            "backup" => {
                let Some(close_rel) = segment[pos..].find("</backup>") else { break; };
                let end = pos + close_rel + "</backup>".len();
                let backup = &segment[pos..end];
                let duration = extract_first_text_from_fragment(backup, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                cursor = cursor.saturating_sub(duration).max(0);
                scan = end;
            }
            "forward" => {
                let Some(close_rel) = segment[pos..].find("</forward>") else { break; };
                let end = pos + close_rel + "</forward>".len();
                let forward = &segment[pos..end];
                let duration = extract_first_text_from_fragment(forward, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                let onset = cursor;
                cursor = cursor.saturating_add(duration);
                fragments.push(RenderXmlTimelineFragmentSpan {
                    scope: scope_number,
                    start: abs_start + pos,
                    end: abs_start + end,
                    onset,
                    duration,
                    staff: "?".to_string(),
                    voice: "?".to_string(),
                    is_note: false,
                    is_forward: true,
                    is_rest: false,
                    is_hidden: false,
                    is_chord: false,
                    is_visible_rest: false,
                    is_visible_pitched: false,
                    has_visible_dependency: false,
                    fragment: forward.to_string(),
                });
                scan = end;
            }
            _ => break,
        }
    }
    fragments
}
fn render_forward_fragment(duration: i32) -> String {
    format!("\n      <forward><duration>{}</duration></forward>", duration.max(0))
}
fn render_staff_voice_forward_fragment(duration: i32, staff: &str, voice: &str) -> String {
    let d = duration.max(0);
    let v = voice.trim();
    let st = staff.trim();
    if v.is_empty() && st.is_empty() {
        return render_forward_fragment(d);
    }
    let mut out = format!("\n      <forward><duration>{d}</duration>");
    if !v.is_empty() {
        out.push_str(&format!("<voice>{v}</voice>"));
    }
    if !st.is_empty() {
        out.push_str(&format!("<staff>{st}</staff>"));
    }
    out.push_str("</forward>");
    out
}
fn render_visible_rest_note_fragment(duration: i32, staff: &str, voice: &str) -> Option<String> {
    let d = duration.max(0);
    let (rest_type, dots) = duration_to_rest_type_and_dots(d)?;
    let mut out = String::new();
    out.push_str("\n      <note>");
    out.push_str("\n        <rest/>");
    out.push_str(&format!("\n        <duration>{d}</duration>"));
    let v = voice.trim();
    if !v.is_empty() {
        out.push_str(&format!("\n        <voice>{v}</voice>"));
    }
    out.push_str(&format!("\n        <type>{rest_type}</type>"));
    for _ in 0..dots {
        out.push_str("\n        <dot/>");
    }
    let st = staff.trim();
    if !st.is_empty() {
        out.push_str(&format!("\n        <staff>{st}</staff>"));
    }
    out.push_str("\n      </note>");
    Some(out)
}
fn beat_index_for_duration_position(onset: i32) -> i32 {
    onset.max(0) / 96
}
fn duration_span_is_inside_single_beat(onset: i32, duration: i32) -> bool {
    if duration <= 0 {
        return false;
    }
    let end_exclusive = onset.saturating_add(duration);
    beat_index_for_duration_position(onset) == beat_index_for_duration_position(end_exclusive.saturating_sub(1))
}
#[derive(Debug, Clone, Default)]
struct RestOnlyStaffEmptyMeasureReadabilityOutcome {
    xml: String,
    measure_rest_candidates_seen: usize,
    measure_rest_attempted: usize,
    auxiliary_lane_candidates_seen: usize,
    auxiliary_lane_attempted: usize,
    hidden_rests_consumed: usize,
    visible_rests_consumed: usize,
    forwards_consumed: usize,
    authority_rejected: bool,
}


fn close_ir6_duration_promotions(
    promotions: Vec<crate::structural_write_authority::DurationPromotion>,
) {
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    for promotion in promotions {
        receipts.push_block_fragment(
            Some(promotion.receipt),
            promotion.start,
            promotion.end,
            &promotion.post,
        );
    }
    receipts.discard_unadopted(
        crate::master_state_identity::RenderDiscardReason::LosingCandidate,
    );
}
fn apply_rest_only_staff_empty_measure_readability(
    xml: &str,
    max_actions: usize,
) -> anyhow::Result<(
    RestOnlyStaffEmptyMeasureReadabilityOutcome,
    crate::master_state_identity::RenderTransitionReceipts,
)> {
    let mut outcome = RestOnlyStaffEmptyMeasureReadabilityOutcome {
        xml: xml.to_string(),
        ..RestOnlyStaffEmptyMeasureReadabilityOutcome::default()
    };
    let mut selected_replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut duration_promotions = Vec::<crate::structural_write_authority::DurationPromotion>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut auxiliary_lane_slots_reserved = 0_usize;
    let metric_expected_by_scope = render_scope_metric_expected_duration_map(xml);
    let mut scan = 0_usize;

    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let (measure_events, measure_ok) = collect_render_measure_note_events(measure, scope_number);
        if !measure_ok {
            scan = measure_end;
            continue;
        }
        let mut staff_visible_pitched: BTreeMap<String, usize> = BTreeMap::new();
        let mut pitched_staffs_by_voice: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for event in &measure_events {
            if event.midi.is_some() {
                pitched_staffs_by_voice
                    .entry(event.voice.clone())
                    .or_default()
                    .insert(event.staff.clone());
                if !event.is_hidden {
                    *staff_visible_pitched.entry(event.staff.clone()).or_default() += 1;
                }
            }
        }

        let measure_close = measure.rfind("</measure>").unwrap_or(measure.len());
        let measure_body_start = measure.find('>').map_or(0, |index| index + 1);
        let measure_expected = metric_expected_by_scope
            .get(&scope_number)
            .copied()
            .or_else(|| infer_measure_duration_from_measure_xml(measure))
            .unwrap_or(384)
            .max(1);
        let mut segments = Vec::<(usize, usize, i32)>::new();
        let mut next_segment_start = measure_body_start;
        let mut local = measure_body_start;
        let mut next_expected = measure_expected;
        loop {
            let next_backup = measure[local..measure_close]
                .find("<backup")
                .map(|position| local + position);
            let segment_end = next_backup.unwrap_or(measure_close);
            if segment_end > next_segment_start {
                segments.push((next_segment_start, segment_end, next_expected));
            }
            let Some(backup_start) = next_backup else {
                break;
            };
            let Some(backup_close_rel) = measure[backup_start..measure_close].find("</backup>") else {
                break;
            };
            let backup_end = backup_start + backup_close_rel + "</backup>".len();
            let backup_fragment = &measure[backup_start..backup_end];
            next_expected = extract_first_text_from_fragment(backup_fragment, "duration")
                .and_then(|value| value.trim().parse::<i32>().ok())
                .unwrap_or(measure_expected)
                .max(1);
            next_segment_start = backup_end;
            local = backup_end;
        }

        for (segment_start, segment_end, expected) in segments {
            if expected <= 0 || duration_to_rest_type_and_dots(expected).is_none() {
                continue;
            }
            let segment = &measure[segment_start..segment_end];
            let fragments = collect_render_timeline_fragments_in_segment(
                segment,
                measure_start + segment_start,
                scope_number,
            );
            if fragments.is_empty() || fragments.iter().any(|frag| frag.scope != scope_number) {
                continue;
            }
            let timeline_duration: i32 = fragments
                .iter()
                .filter(|frag| (frag.is_note && !frag.is_chord) || frag.is_forward)
                .map(|frag| frag.duration.max(0))
                .sum();
            if timeline_duration != expected {
                continue;
            }
            let visible_rests = fragments
                .iter()
                .filter(|frag| frag.is_visible_rest)
                .collect::<Vec<_>>();
            if visible_rests.is_empty() || fragments.iter().any(|frag| frag.is_visible_pitched) {
                continue;
            }
            let unsafe_rest = fragments
                .iter()
                .filter(|frag| frag.is_note && frag.is_rest)
                .any(|frag| frag.has_visible_dependency || frag.duration <= 0);
            let hidden_pitched = fragments
                .iter()
                .any(|frag| frag.is_note && frag.is_hidden && !frag.is_rest);
            let non_rest_notes = fragments.iter().any(|frag| frag.is_note && !frag.is_rest);
            if unsafe_rest || hidden_pitched || non_rest_notes {
                continue;
            }
            let Some(first_rest) = visible_rests.first().copied() else {
                continue;
            };
            let same_staff_voice = fragments
                .iter()
                .filter(|frag| frag.is_note && frag.is_rest)
                .all(|frag| frag.staff == first_rest.staff && frag.voice == first_rest.voice);
            if !same_staff_voice {
                continue;
            }
            let staff_has_visible_pitched = staff_visible_pitched
                .get(&first_rest.staff)
                .copied()
                .unwrap_or(0)
                > 0;
            let same_voice_is_cross_staff = pitched_staffs_by_voice
                .get(&first_rest.voice)
                .is_some_and(|staffs| {
                    staffs
                        .iter()
                        .any(|staff| staff.trim() != first_rest.staff.trim())
                });

            if !staff_has_visible_pitched && same_voice_is_cross_staff {
                continue;
            }

            if !staff_has_visible_pitched {
                outcome.measure_rest_candidates_seen =
                    outcome.measure_rest_candidates_seen.saturating_add(1);
                if outcome.measure_rest_attempted
                    + outcome.auxiliary_lane_attempted
                    + auxiliary_lane_slots_reserved
                    >= max_actions
                {
                    continue;
                }
                let timeline_fragments = fragments
                    .iter()
                    .filter(|frag| (frag.is_note && !frag.is_chord) || frag.is_forward)
                    .collect::<Vec<_>>();
                let already_canonical = timeline_fragments.len() == 1
                    && first_rest.duration == expected
                    && first_rest.fragment.contains("measure=\"yes\"");
                if already_canonical {
                    continue;
                }
                let (owner_post, owner_receipt) =
                    set_rest_note_duration_type_with_receipt(&first_rest.fragment, expected);
                let Some(owner_post) = owner_post else {
                    continue;
                };
                let mut full_rest = owner_post.clone();
                if full_rest.contains("<rest/>") {
                    full_rest = full_rest.replace("<rest/>", "<rest measure=\"yes\"/>");
                } else if full_rest.contains("<rest />") {
                    full_rest = full_rest.replace("<rest />", "<rest measure=\"yes\"/>");
                } else if full_rest.contains("<rest>") {
                    full_rest = full_rest.replacen("<rest>", "<rest measure=\"yes\">", 1);
                }
                let local_promotion = owner_receipt.map(|receipt| {
                    crate::structural_write_authority::DurationPromotion::new(
                        first_rest.start,
                        first_rest.end,
                        owner_post,
                        receipt,
                    )
                });
                let mut replacements = vec![(first_rest.start, first_rest.end, full_rest)];
                let mut hidden_rests_consumed = 0_usize;
                let mut visible_rests_consumed = 1_usize;
                let mut forwards_consumed = 0_usize;
                for frag in timeline_fragments {
                    if frag.start == first_rest.start && frag.end == first_rest.end {
                        continue;
                    }
                    if frag.is_forward {
                        forwards_consumed = forwards_consumed.saturating_add(1);
                    } else if frag.is_note && frag.is_rest {
                        if frag.is_hidden {
                            hidden_rests_consumed = hidden_rests_consumed.saturating_add(1);
                        } else {
                            visible_rests_consumed = visible_rests_consumed.saturating_add(1);
                        }
                    }
                    replacements.push((frag.start, frag.end, String::new()));
                }
                let overlaps = replacements.iter().any(|(start, end, _)| {
                    occupied
                        .iter()
                        .any(|(used_start, used_end)| *start < *used_end && *end > *used_start)
                });
                if overlaps {
                    if let Some(promotion) = local_promotion {
                        close_ir6_duration_promotions(vec![promotion]);
                    }
                    continue;
                }
                for (start, end, _) in &replacements {
                    occupied.push((*start, *end));
                }
                selected_replacements.extend(replacements);
                if let Some(promotion) = local_promotion {
                    duration_promotions.push(promotion);
                }
                outcome.measure_rest_attempted = outcome.measure_rest_attempted.saturating_add(1);
                outcome.hidden_rests_consumed = outcome
                    .hidden_rests_consumed
                    .saturating_add(hidden_rests_consumed);
                outcome.visible_rests_consumed = outcome
                    .visible_rests_consumed
                    .saturating_add(visible_rests_consumed);
                outcome.forwards_consumed =
                    outcome.forwards_consumed.saturating_add(forwards_consumed);
            } else {
                outcome.auxiliary_lane_candidates_seen =
                    outcome.auxiliary_lane_candidates_seen.saturating_add(1);
                if outcome.measure_rest_attempted
                    + outcome.auxiliary_lane_attempted
                    + auxiliary_lane_slots_reserved
                    >= max_actions
                    || visible_rests.len() != 1
                {
                    continue;
                }
                let overlaps = occupied.iter().any(|(used_start, used_end)| {
                    first_rest.start < *used_end && first_rest.end > *used_start
                });
                if overlaps {
                    continue;
                }
                occupied.push((first_rest.start, first_rest.end));
                auxiliary_lane_slots_reserved =
                    auxiliary_lane_slots_reserved.saturating_add(1);
            }
        }
        scan = measure_end;
    }

    if selected_replacements.is_empty() {
        return Ok((outcome, crate::master_state_identity::RenderTransitionReceipts::new()));
    }
    let (draft, rebased_replacements, mut receipts) =
        crate::structural_write_authority::mutate_selected_duration_fragments(
            xml,
            duration_promotions,
            selected_replacements,
        )?;
    let (candidate, consumer_receipt) =
        crate::structural_write_authority::apply_rest_only_staff_readability_authorized(
            &draft,
            rebased_replacements,
        );
    match candidate {
        Some(candidate_xml) => {
            receipts.push_exact(consumer_receipt);
            outcome.xml = candidate_xml;
        }
        None => {
            receipts.discard_unadopted(
                crate::master_state_identity::RenderDiscardReason::LosingCandidate,
            );
            receipts = crate::master_state_identity::RenderTransitionReceipts::new();
            outcome.authority_rejected = true;
            // Versuche bleiben als geplante, aber abgelehnte Kandidaten sichtbar.
            // Die "consumed"-Zaehler bezeichnen nur tatsaechlich autorisierte
            // Entfernung und bleiben bei Authority-Ablehnung deshalb null.
            outcome.hidden_rests_consumed = 0;
            outcome.visible_rests_consumed = 0;
            outcome.forwards_consumed = 0;
        }
    }
    Ok((outcome, receipts))
}

fn build_render_xml_pianistic_rest_repair_trial(
    xml: &str,
    planner: &RenderXmlVisibleRestReadabilityAudit,
    _output_path: &Path,
) -> anyhow::Result<(
    RenderXmlPianisticRestRepairTrial,
    Option<String>,
    crate::master_state_identity::RenderTransitionReceipts,
)> {
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut trial = RenderXmlPianisticRestRepairTrial {
        source_label: planner.source_label.clone(),
        missing_candidates_seen: planner.missing_visible_rest_candidates,
        consolidation_candidates_seen: planner.consolidate_candidates,
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        backup_forward_before_ok: before_backup_forward_ok,
        parse_before_ok: parse_musicxml(xml).is_ok(),
        ..RenderXmlPianisticRestRepairTrial::default()
    };
    if !trial.parse_before_ok || !trial.backup_forward_before_ok {
        trial.verdict = "pianistic_rest_repair_blocked_by_input_parse_or_timeline".to_string();
        trial.done_fragment = "RenderXmlPianisticRestRepairTrial: blockiert vor Mutation; keine Änderung".to_string();
        trial.pipeline_line = format!(
            "RenderXmlPianisticRestRepairTrial v{}: blockiert vor Mutation; parse/back-forward {}/{}; keine Render-XML-Änderung.",
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
    let (
        candidate_xml,
        mut actions,
        merge_candidate_count,
        generated_receipts,
        consumer_receipt,
    ) = apply_pianistic_rest_consolidation_with_receipts(xml, 64);
    let has_consumer_receipt = consumer_receipt.is_some();
    let mut render_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    for generated in generated_receipts {
        render_receipts.push_generated_block(
            Some(generated.receipt),
            &generated.pre_fragment,
            &generated.post_fragment,
            Some(generated.located),
        );
    }
    render_receipts.push_exact(consumer_receipt);
    trial.consolidation_candidates_seen = merge_candidate_count;
    trial.consolidation_attempted = actions.len();
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
    let invariants_ok = trial.consolidation_attempted > 0
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.hidden_pitched_before == trial.hidden_pitched_after
        && trial.hidden_rests_before == trial.hidden_rests_after
        && trial.backup_forward_after_ok
        && trial.parse_after_ok
        && has_consumer_receipt;
    if invariants_ok {
        // Physical materialization is performed only after #17 Generated and
        // the consuming #34 EXACT receipt replay as one sovereign transition.
        trial.written = true;
        trial.consolidation_adopted = trial.consolidation_attempted;
        trial.verdict = "pianistic_rest_consolidation_adopted_render_xml_only".to_string();
        for action in &mut actions {
            action.verdict = "adopted".to_string();
        }
    } else {
        trial.consolidation_rejected = trial.consolidation_attempted;
        trial.verdict = if trial.consolidation_attempted == 0 {
            "pianistic_rest_repair_trial_no_safe_candidate".to_string()
        } else {
            "pianistic_rest_repair_trial_rejected_by_invariant_guard".to_string()
        };
        for action in &mut actions {
            action.verdict = "rejected_by_global_invariant_guard".to_string();
        }
    }
    trial.actions = actions;
    trial.examples.push("Render-XML-only: Der Trial verändert höchstens sichtbare Restketten; Master-XML, klingende Noten, Tonhöhen, Onsets, Dauern, Staffs und Voices klingender Events bleiben unverändert.".to_string());
    trial.examples.push("Fehlende sichtbare Pausen werden nicht mehr als bloße Log-Kandidaten behandelt: vorhandene forward-Zeit darf als sichtbare Pause materialisiert werden, wenn die klingende Signatur stabil bleibt; sichtbare Pausenketten gleicher Staff/Voice müssen generisch zusammengefasst werden, wenn dies ohne CrossBeatMerge sicher ist.".to_string());
    trial.examples.push("Ganztaktstille und rest-only Hilfsstimmen werden hier nicht mehr entschieden; ihre einzige produktive Owner-Grenze ist BehindBarsRestAndTieAudit.".to_string());
    trial.examples.push("Praktische Notensatzwirkung: benachbarte äquivalente Pausen derselben Stimme werden zählzeitfreundlicher notiert, z. B. zwei 16tel-Pausen als eine 8tel-Pause; das ist keine musikalische Änderung.".to_string());
    trial.examples.push("Keine konkrete Taktlogik: boundary-/rest-lane-artige Fälle fallen nur dann darunter, wenn die Staff-/Voice-/Rest-only-Invarianten generisch erfüllt sind; Bereichsnummern in der Tabelle sind berechnete Ergebnisdaten.".to_string());
    trial.done_fragment = format!(
        "PianisticRestConsolidation: Rest-Aktionen {}/{}, Fehlpausen eingefügt {}, Render-XML-only {}, klingend stabil {}",
        trial.consolidation_adopted,
        trial.consolidation_attempted,
        trial.missing_inserted,
        if trial.written { "geschrieben" } else { "nicht geschrieben" },
        if trial.sounding_signature_stable { "ja" } else { "nein" }
    );
    trial.pipeline_line = format!(
        "PianisticRestConsolidation v{}: Render-XML-only auf {}; fehlende sichtbare Pausen gesehen {}, eingefügt {}; Rest-/Pausenkandidaten {}, versucht {}, übernommen {}; sichtbare Pausen {}→{}; sichtbare klingende Noten {}→{}; versteckte klingende Noten {}→{}; versteckte Pausen {}→{}; sounding_signature {}/{} stabil {}; visible_rest_delta {}; parse/back-forward nachher {}/{}; geschrieben {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.missing_candidates_seen,
        trial.missing_inserted,
        trial.consolidation_candidates_seen,
        trial.consolidation_attempted,
        trial.consolidation_adopted,
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
fn infer_measure_duration_from_measure_xml(measure: &str) -> Option<i32> {
    let (events, _ok) = collect_render_measure_note_events(measure, 0);
    let max_event_end = events
        .iter()
        .filter(|event| !event.is_chord)
        .map(|event| event.onset.saturating_add(event.duration.max(0)))
        .max()
        .unwrap_or(0);
    if max_event_end > 0 {
        Some(max_event_end)
    } else {
        let divisions = extract_first_text_from_fragment(measure, "divisions")
            .and_then(|value| value.trim().parse::<i32>().ok())
            .unwrap_or(96);
        let beats = extract_first_text_from_fragment(measure, "beats")
            .and_then(|value| value.trim().parse::<i32>().ok())
            .unwrap_or(4);
        let beat_type = extract_first_text_from_fragment(measure, "beat-type")
            .and_then(|value| value.trim().parse::<i32>().ok())
            .unwrap_or(4);
        let duration = beats.saturating_mul(divisions).saturating_mul(4).checked_div(beat_type.max(1)).unwrap_or(0);
        if duration > 0 { Some(duration) } else { None }
    }
}
#[cfg(test)]
fn rest_fragment_with_staff_voice_duration(template: &str, duration: i32, staff: &str, voice: &str) -> Option<String> {
    let (out, receipt) = set_rest_note_duration_type_with_receipt(template, duration);
    let mut out = out?;
    let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
    probe.push_generated_block(receipt, template, &out, None);
    probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    out = replace_xml_tag_text(&out, "staff", staff);
    out = replace_xml_tag_text(&out, "voice", voice);
    Some(out)
}
fn apply_behind_bars_rule_check_lower_staff_rest_compaction(
    xml: &str,
    max_actions: usize,
) -> anyhow::Result<PreparedRestTieActions> {
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let spans = collect_visible_rest_spans_for_repair(xml);
    let mut by_scope_voice: BTreeMap<(usize, String, String), Vec<RenderXmlRestFragmentSpan>> = BTreeMap::new();
    for span in spans.into_iter().filter(|span| span.staff.trim() == "2") {
        by_scope_voice
            .entry((span.scope, span.staff.clone(), span.voice.clone()))
            .or_default()
            .push(span);
    }
    let mut chains: Vec<Vec<RenderXmlRestFragmentSpan>> = Vec::new();
    for ((_scope, _staff, _voice), mut group) in by_scope_voice {
        group.sort_by_key(|span| (span.onset, span.start));
        let mut chain: Vec<RenderXmlRestFragmentSpan> = Vec::new();
        for span in group {
            let contiguous = chain
                .last()
                .is_some_and(|last| last.onset.saturating_add(last.duration) == span.onset);
            if !chain.is_empty() && !contiguous {
                if chain.len() >= 2 {
                    chains.push(chain.clone());
                }
                chain.clear();
            }
            chain.push(span);
        }
        if chain.len() >= 2 {
            chains.push(chain);
        }
    }
    let candidates_seen = chains.len();
    chains.sort_by(|a, b| {
        let adur: i32 = a.iter().map(|s| s.duration.max(0)).sum();
        let bdur: i32 = b.iter().map(|s| s.duration.max(0)).sum();
        b.len()
            .cmp(&a.len())
            .then_with(|| bdur.cmp(&adur))
            .then_with(|| a.first().map_or(0, |s| s.scope).cmp(&b.first().map_or(0, |s| s.scope)))
            .then_with(|| a.first().map_or(0, |s| s.onset).cmp(&b.first().map_or(0, |s| s.onset)))
    });
    let mut selected_replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut duration_promotions = Vec::<crate::structural_write_authority::DurationPromotion>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut actions = Vec::new();
    for chain in chains {
        if actions.len() >= max_actions {
            break;
        }
        let Some(first) = chain.first() else { continue; };
        let total_duration: i32 = chain.iter().map(|span| span.duration.max(0)).sum();
        let Some((expected, beat_unit)) = metric_by_scope.get(&first.scope).copied() else {
            continue;
        };
        let overlaps = chain.iter().any(|span| {
            occupied.iter().any(|(used_start, used_end)| span.start < *used_end && span.end > *used_start)
        });
        if overlaps {
            continue;
        }
        let chord_anchor_dependency = chain.iter().any(|span| {
            find_measure_bounds_containing_pos(xml, span.start)
                .is_some_and(|(_, measure_end, _)| {
                    note_is_followed_by_chord_note(xml, span.end, measure_end)
                })
        });
        let Some(decision) = crate::model::metric_rest_notation::decide_rest_span_consolidation(
            crate::model::metric_rest_notation::RestSpanConsolidationFacts {
                start: i64::from(first.onset),
                duration: i64::from(total_duration),
                expected_measure_duration: i64::from(expected),
                beat_duration: i64::from(beat_unit),
                source_fragment_count: chain.len(),
                has_chord_anchor_dependency: chord_anchor_dependency,
                all_source_fragments_short: false,
            },
            crate::model::metric_rest_notation::RestSpanConsolidationProjection::MetricCompaction,
        ) else {
            continue;
        };
        let new_parts = decision
            .parts
            .into_iter()
            .map(i32::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default();
        let all_supported = new_parts.iter().all(|part| duration_to_rest_type_and_dots(*part).is_some());
        if !all_supported {
            continue;
        }
        let mut replacements = Vec::<(usize, usize, String)>::new();
        let mut local_promotions = Vec::<crate::structural_write_authority::DurationPromotion>::new();
        let mut build_ok = true;
        for (idx, span) in chain.iter().enumerate() {
            if let Some(duration) = new_parts.get(idx).copied() {
                let (owner_post, owner_receipt) =
                    set_rest_note_duration_type_with_receipt(&span.fragment, duration);
                let Some(owner_post) = owner_post else {
                    build_ok = false;
                    break;
                };
                let mut fragment = replace_xml_tag_text(&owner_post, "staff", &span.staff);
                fragment = replace_xml_tag_text(&fragment, "voice", &span.voice);
                if let Some(receipt) = owner_receipt {
                    local_promotions.push(
                        crate::structural_write_authority::DurationPromotion::new(
                            span.start,
                            span.end,
                            owner_post,
                            receipt,
                        ),
                    );
                }
                replacements.push((span.start, span.end, fragment));
            } else {
                replacements.push((span.start, span.end, String::new()));
            }
        }
        if !build_ok {
            close_ir6_duration_promotions(local_promotions);
            continue;
        }
        for span in &chain {
            occupied.push((span.start, span.end));
        }
        selected_replacements.extend(replacements);
        duration_promotions.extend(local_promotions);
        actions.push(BehindBarsRuleCheckRestAndTieAction {
            action_kind: "lower_staff_visible_rest_chain_compaction".to_string(),
            old_rest_count: chain.len(),
            new_rest_count: new_parts.len(),
            verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
        });
    }
    if selected_replacements.is_empty() {
        return Ok(((
            xml.to_string(),
            actions,
            candidates_seen,
        ), crate::master_state_identity::RenderTransitionReceipts::new()));
    }
    let (draft, rebased_replacements, mut receipts) =
        crate::structural_write_authority::mutate_selected_duration_fragments(
            xml,
            duration_promotions,
            selected_replacements,
        )?;
    let (out, consumer_receipt) =
        crate::structural_write_authority::apply_lower_staff_rest_chain_compaction_authorized(
            &draft,
            rebased_replacements,
        );
    let Some(out) = out else {
        receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::LosingCandidate,
        );
        return Ok(((
            xml.to_string(),
            actions,
            candidates_seen,
        ), crate::master_state_identity::RenderTransitionReceipts::new()));
    };
    receipts.push_exact(consumer_receipt);
    Ok(((out, actions, candidates_seen), receipts))
}
fn render_scope_metric_expected_duration_map(xml: &str) -> BTreeMap<usize, i32> {
    let mut out = BTreeMap::new();
    let mut scan = 0_usize;
    let mut divisions = 96_i32;
    let mut beats = 4_i32;
    let mut beat_type = 4_i32;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        if let Some(value) = extract_first_text_from_fragment(measure, "divisions")
            .and_then(|value| value.trim().parse::<i32>().ok())
            && value > 0
        {
            divisions = value;
        }
        if let Some(value) = extract_first_text_from_fragment(measure, "beats")
            .and_then(|value| value.trim().parse::<i32>().ok())
            && value > 0
        {
            beats = value;
        }
        if let Some(value) = extract_first_text_from_fragment(measure, "beat-type")
            .and_then(|value| value.trim().parse::<i32>().ok())
            && value > 0
        {
            beat_type = value;
        }
        let metric = beats
            .saturating_mul(divisions)
            .saturating_mul(4)
            .checked_div(beat_type.max(1))
            .unwrap_or(384)
            .max(1);
        out.insert(scope_number, metric);
        scan = measure_end;
    }
    out
}
fn render_scope_metric_expected_and_beat_map(xml: &str) -> BTreeMap<usize, (i32, i32)> {
    let mut out = BTreeMap::new();
    let mut scan = 0_usize;
    let mut divisions = 96_i32;
    let mut beats = 4_i32;
    let mut beat_type = 4_i32;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        if let Some(value) = extract_first_text_from_fragment(measure, "divisions")
            .and_then(|value| value.trim().parse::<i32>().ok())
            && value > 0
        {
            divisions = value;
        }
        if let Some(value) = extract_first_text_from_fragment(measure, "beats")
            .and_then(|value| value.trim().parse::<i32>().ok())
            && value > 0
        {
            beats = value;
        }
        if let Some(value) = extract_first_text_from_fragment(measure, "beat-type")
            .and_then(|value| value.trim().parse::<i32>().ok())
            && value > 0
        {
            beat_type = value;
        }
        let expected = beats
            .saturating_mul(divisions)
            .saturating_mul(4)
            .checked_div(beat_type.max(1))
            .unwrap_or(384)
            .max(1);
        let beat_unit = divisions
            .saturating_mul(4)
            .checked_div(beat_type.max(1))
            .unwrap_or(divisions)
            .max(1);
        out.insert(scope_number, (expected, beat_unit));
        scan = measure_end;
    }
    out
}
fn format_visible_lane_gaps(gaps: &[(i32, i32, &'static str)], limit: usize) -> String {
    if gaps.is_empty() {
        return "-".to_string();
    }
    let mut parts = Vec::new();
    for (idx, (start, end, kind)) in gaps.iter().enumerate() {
        if idx >= limit {
            parts.push(format!("+{}more", gaps.len().saturating_sub(limit)));
            break;
        }
        parts.push(format!("{kind}:{start}-{end}"));
    }
    parts.join(",")
}

fn strict_staff_gap_parts(
    gaps: &[(i32, i32, &'static str)],
    expected: i32,
    beat_unit: i32,
) -> Vec<(i32, i32)> {
    let mut parts = Vec::new();
    for (start, end, _kind) in gaps.iter().copied() {
        let duration = end.saturating_sub(start);
        if duration <= 0 {
            continue;
        }
        let planned = crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
            i64::from(start),
            i64::from(duration),
            i64::from(expected),
            i64::from(beat_unit),
        );
        if planned.is_empty() {
            return Vec::new();
        }
        let mut cursor = start;
        for part in planned {
            let Ok(part) = i32::try_from(part) else {
                return Vec::new();
            };
            if part <= 0 || duration_to_rest_type_and_dots(part).is_none() {
                return Vec::new();
            }
            parts.push((cursor, part));
            cursor = cursor.saturating_add(part);
        }
        if cursor != end {
            return Vec::new();
        }
    }
    parts
}
fn strict_staff_merge_visible_intervals(mut intervals: Vec<(i32, i32)>, expected: i32) -> Vec<(i32, i32)> {
    intervals.sort_by_key(|(start, duration)| (*start, *duration));
    let mut merged: Vec<(i32, i32)> = Vec::new();
    for (start, duration) in intervals {
        if duration <= 0 || start >= expected { continue; }
        let s0 = start.max(0);
        let e0 = start.saturating_add(duration).min(expected);
        if e0 <= s0 { continue; }
        if let Some(last) = merged.last_mut() {
            let last_end = last.0.saturating_add(last.1);
            if s0 <= last_end {
                last.1 = last_end.max(e0).saturating_sub(last.0);
                continue;
            }
        }
        merged.push((s0, e0.saturating_sub(s0)));
    }
    merged
}
fn strict_staff_gaps_from_merged_intervals(merged: &[(i32, i32)], expected: i32) -> Vec<(i32, i32, &'static str)> {
    let mut gaps = Vec::new();
    let mut cursor = 0_i32;
    for (start, duration) in merged.iter().copied() {
        if start > cursor {
            let kind = if cursor == 0 { "start" } else { "internal" };
            gaps.push((cursor, start, kind));
        }
        cursor = cursor.max(start.saturating_add(duration));
    }
    if cursor < expected {
        gaps.push((cursor, expected, "end"));
    }
    gaps
}

#[cfg(test)]
mod strict_staff_metric_gap_parts_tests {
    use super::*;

    #[test]
    fn aligned_two_beat_gap_uses_central_half_rest_plan() {
        let parts = strict_staff_gap_parts(&[(0, 288, "start")], 384, 96);
        assert_eq!(parts, vec![(0, 192), (192, 96)]);
    }

    #[test]
    fn weak_beat_gap_does_not_cross_the_half_measure_boundary() {
        let parts = strict_staff_gap_parts(&[(96, 288, "internal")], 384, 96);
        assert_eq!(parts, vec![(96, 96), (192, 96)]);
    }

    #[test]
    fn noncanonical_time_basis_fails_closed() {
        let parts = strict_staff_gap_parts(&[(0, 96, "start")], 192, 48);
        assert!(parts.is_empty());
    }
}

#[cfg(test)]
mod canonical_measure_rest_single_owner_architecture_tests {
    use super::*;


    fn quarter_rest(voice: &str, staff: &str, hidden: bool) -> String {
        let hidden_attr = if hidden { " print-object=\"no\"" } else { "" };
        format!(
            "<note{hidden_attr}><rest/><duration>96</duration><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff></note>"
        )
    }

    #[test]
    fn complete_initial_lane_silence_becomes_one_measure_rest() {
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{}{}{}{}<backup><duration>384</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>384</duration><voice>2</voice><type>whole</type><staff>2</staff></note></measure></part></score-partwise>",
            quarter_rest("1", "1", false),
            quarter_rest("1", "1", true),
            quarter_rest("1", "1", true),
            quarter_rest("1", "1", true),
        );
        let (outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&xml, 8).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!outcome.authority_rejected);
        assert_eq!(outcome.measure_rest_attempted, 1);
        assert_eq!(outcome.hidden_rests_consumed, 3);
        assert_eq!(outcome.xml.matches("<rest measure=\"yes\"").count(), 1);
        assert_eq!(outcome.xml.matches("print-object=\"no\"").count(), 0);
    }

    #[test]
    fn quarter_rest_plus_forward_coverage_becomes_one_measure_rest() {
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note><backup><duration>384</duration></backup>{}<forward><duration>96</duration></forward><forward><duration>96</duration></forward><forward><duration>96</duration></forward></measure></part></score-partwise>",
            quarter_rest("2", "2", false),
        );
        let (outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&xml, 8).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!outcome.authority_rejected);
        assert_eq!(outcome.measure_rest_attempted, 1);
        assert_eq!(outcome.forwards_consumed, 3);
        assert_eq!(outcome.xml.matches("<rest measure=\"yes\"").count(), 1);
        assert_eq!(outcome.xml.matches("<forward>").count(), 0);

        // Regression: the first lane of a later measure may consist of one
        // visible rest plus forward carriers before any backup. Its expected
        // duration must come from inherited metric scope truth, not from the
        // note-event endpoint of that carrier form. This stays inside the
        // already existing test entry and therefore adds no writer caller.
        let inherited_metric_xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note></measure><measure number=\"\
2\">{}<forward><duration>96</duration></forward><forward><duration>96</duration></forward><forward><duration>96</duration></forward><backup><duration>384</duration></backup>{}<forward><duration>96</duration></forward><forward><duration>96</duration></forward><forward><duration>96</duration></forward></measure></part></score-partwise>",
            quarter_rest("1", "1", false),
            quarter_rest("2", "2", false),
        );
        let (inherited_outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&inherited_metric_xml, 8).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!inherited_outcome.authority_rejected);
        assert_eq!(inherited_outcome.measure_rest_attempted, 2);
        assert_eq!(inherited_outcome.forwards_consumed, 6);
        assert_eq!(
            inherited_outcome.xml.matches("<rest measure=\"yes\"").count(),
            2
        );
        assert_eq!(inherited_outcome.xml.matches("<forward>").count(), 0);

        // r1118 regression: visible staff activity elsewhere in the measure is
        // observation only. It does not prove that a visible rest in a separate
        // rest-only serialization segment is a dispensable auxiliary carrier.
        let synthetic_scope_a = 1_usize;
        let synthetic_scope_b = 2_usize;
        let partially_active_staff_xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note><note><pitch><step>E</step><octave>3</octave></pitch><duration>192</duration><voice>2</voice><type>half</type><staff>2</staff></note><backup><duration>384</duration></backup><forward><duration>96</duration></forward>{}<forward><duration>96</duration></forward><forward><duration>96</duration></forward></measure></part></score-partwise>",
            synthetic_scope_a,
            quarter_rest("4", "2", false),
        );
        let (partially_active_outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&partially_active_staff_xml, 8).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!partially_active_outcome.authority_rejected);
        assert_eq!(partially_active_outcome.auxiliary_lane_candidates_seen, 1);
        assert_eq!(partially_active_outcome.auxiliary_lane_attempted, 0);
        assert_eq!(partially_active_outcome.visible_rests_consumed, 0);
        assert_eq!(partially_active_outcome.xml, partially_active_staff_xml);

        // The same obligation is metric-relative, not a 384-tick special case.
        let three_four_xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{}\"><attributes><divisions>48</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>144</duration><voice>2</voice><type>half</type><dot/><staff>2</staff></note><backup><duration>144</duration></backup><forward><duration>48</duration></forward><note><rest/><duration>48</duration><voice>7</voice><type>quarter</type><staff>2</staff></note><forward><duration>48</duration></forward></measure></part></score-partwise>",
            synthetic_scope_a,
        );
        let (three_four_outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&three_four_xml, 8).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!three_four_outcome.authority_rejected);
        assert_eq!(three_four_outcome.auxiliary_lane_candidates_seen, 1);
        assert_eq!(three_four_outcome.auxiliary_lane_attempted, 0);
        assert_eq!(three_four_outcome.xml, three_four_xml);

        // Suppressing the unsafe write must not free an old action slot and
        // thereby enable a later unrelated measure-rest write in the same scan.
        let action_slot_parity_xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{}\"><attributes><divisions>48</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>144</duration><voice>2</voice><type>half</type><dot/><staff>2</staff></note><backup><duration>144</duration></backup><forward><duration>48</duration></forward><note><rest/><duration>48</duration><voice>7</voice><type>quarter</type><staff>2</staff></note><forward><duration>48</duration></forward></measure><measure number=\"{}\"><note><rest/><duration>48</duration><voice>3</voice><type>quarter</type><staff>1</staff></note><forward><duration>48</duration></forward><forward><duration>48</duration></forward></measure></part></score-partwise>",
            synthetic_scope_a,
            synthetic_scope_b,
        );
        let (action_slot_parity_outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&action_slot_parity_xml, 1).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!action_slot_parity_outcome.authority_rejected);
        assert_eq!(action_slot_parity_outcome.auxiliary_lane_candidates_seen, 1);
        assert_eq!(action_slot_parity_outcome.auxiliary_lane_attempted, 0);
        assert_eq!(action_slot_parity_outcome.measure_rest_attempted, 0);
        assert_eq!(action_slot_parity_outcome.xml, action_slot_parity_xml);

        // Independent duplication may increase observation counts, but must not
        // turn either local Keep decision into scorewide write authority.
        let duplicated_keep_xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{}\"><attributes><divisions>48</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>144</duration><voice>2</voice><type>half</type><dot/><staff>2</staff></note><backup><duration>144</duration></backup><forward><duration>48</duration></forward><note><rest/><duration>48</duration><voice>7</voice><type>quarter</type><staff>2</staff></note><forward><duration>48</duration></forward></measure><measure number=\"{}\"><note><pitch><step>D</step><octave>3</octave></pitch><duration>144</duration><voice>2</voice><type>half</type><dot/><staff>2</staff></note><backup><duration>144</duration></backup><forward><duration>48</duration></forward><note><rest/><duration>48</duration><voice>9</voice><type>quarter</type><staff>2</staff></note><forward><duration>48</duration></forward></measure></part></score-partwise>",
            synthetic_scope_a,
            synthetic_scope_b,
        );
        let (duplicated_keep_outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&duplicated_keep_xml, 8).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!duplicated_keep_outcome.authority_rejected);
        assert_eq!(duplicated_keep_outcome.auxiliary_lane_candidates_seen, 2);
        assert_eq!(duplicated_keep_outcome.auxiliary_lane_attempted, 0);
        assert_eq!(duplicated_keep_outcome.xml, duplicated_keep_xml);
    }

    #[test]
    fn cross_staff_voice_is_not_collapsed_to_measure_rest() {
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note><backup><duration>384</duration></backup>{}{}{}{}</measure></part></score-partwise>",
            quarter_rest("1", "2", false),
            quarter_rest("1", "2", true),
            quarter_rest("1", "2", true),
            quarter_rest("1", "2", true),
        );
        let (outcome, receipts) = apply_rest_only_staff_empty_measure_readability(&xml, 8).expect("prepare");
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(!outcome.authority_rejected);
        assert_eq!(outcome.measure_rest_attempted, 0);
        assert!(!outcome.xml.contains("<rest measure=\"yes\""));
        assert_eq!(outcome.xml, xml);

        let alternating_cross_staff_xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>2</staff></note><note><pitch><step>D</step><octave>5</octave></pitch><duration>192</duration><voice>1</voice><type>half</type><staff>1</staff></note><note><pitch><step>E</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>2</staff></note></measure></part></score-partwise>";
        let strict_readback = build_strict_staff_local_release_completeness_readback(
            alternating_cross_staff_xml,
            "alternating cross-staff regression",
        );
        assert_eq!(strict_readback.incomplete_lanes, 0);
        assert_eq!(strict_readback.cross_staff_bookkeeping_lanes, 2);
        let final_readback = build_visible_lane_completeness_readback_gate_no_mutation(
            alternating_cross_staff_xml,
            "alternating cross-staff regression",
        );
        assert_eq!(final_readback.incomplete_lanes, 0);
        assert_eq!(final_readback.cross_staff_bookkeeping_lanes, 2);
        let (strict_xml, strict_actions, strict_seen, strict_trace) =
            apply_strict_staff_local_release_completeness_materialization(
                alternating_cross_staff_xml,
                8,
            );
        assert_eq!(strict_seen, 0);
        assert!(strict_actions.is_empty());
        assert_eq!(strict_xml, alternating_cross_staff_xml);
        assert_eq!(
            strict_trace
                .iter()
                .filter(|row| row.contains("no_overlay_same_voice_other_staff_pitch_coverage"))
                .count(),
            2
        );
    }

    #[test]
    fn hidden_chord_carriers_can_preserve_topology_while_visible_initial_rests_compact() {
        let rest = |duration, note_type| {
            format!(
                "<note><rest/><duration>{duration}</duration><voice>1</voice><type>{note_type}</type><staff>1</staff></note>"
            )
        };
        let chord = |step, duration, note_type| {
            format!(
                "<note print-object=\"no\" print-spacing=\"no\"><chord/><pitch><step>{step}</step><octave>6</octave></pitch><duration>{duration}</duration><voice>1</voice><type>{note_type}</type><staff>1</staff></note>"
            )
        };
        let note = |step| {
            format!(
                "<note><pitch><step>{step}</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>"
            )
        };
        let measure_body = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            rest(24, "16th"),
            rest(24, "16th"),
            rest(48, "eighth"),
            chord("D", 48, "eighth"),
            rest(24, "16th"),
            rest(24, "16th"),
            chord("F", 24, "16th"),
            rest(24, "16th"),
            rest(24, "16th"),
            rest(24, "16th"),
            chord("C", 24, "16th"),
            ["D", "C", "B", "A", "G", "F", "E"]
                .into_iter()
                .map(note)
                .collect::<String>()
        );
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{measure_body}</measure></part></score-partwise>"
        );
        let candidates = build_carrier_preserving_initial_rest_projection_candidates(&xml);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].parts, vec![192, 24]);
        assert_eq!(candidates[0].rest.total_duration, 216);
        assert_eq!(candidates[0].rest.spans.len(), 8);
        assert!(candidates[0].insertion_pos > candidates[0].rest.spans.last().expect("last span").end);
    }

}

#[cfg(test)]
mod retention_first_p2_split_receipt_tests {
    use super::*;

    #[test]
    fn prune_receipt_preserves_surviving_occurrence_order_across_visibility_only_drift() {
        let master_xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><note print-object=\"no\" print-spacing=\"no\"><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note></measure></part></score-partwise>";
        let source_notes = parse_atomic_notes(master_xml);
        assert_eq!(source_notes.len(), 3);

        let (render_xml, prune_stats, _receipts) =
            prune_safe_hidden_inner_chord_tones_for_render(master_xml);
        assert_eq!(prune_stats.removed_source_note_ranges.len(), 1);
        assert_eq!(
            prune_stats.removed_source_note_ranges[0],
            (source_notes[1].note_start, source_notes[1].note_end)
        );

        let rows = build_retention_first_p2_split_occurrence_receipt(
            master_xml,
            &prune_stats.removed_source_note_ranges,
        )
        .expect("split receipt must describe surviving pitched occurrences");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].raw_note_index, 0);
        assert_eq!(rows[1].raw_note_index, 2);
        assert_eq!(rows[0].pitched_ordinal, 0);
        assert_eq!(rows[1].pitched_ordinal, 1);
        validate_retention_first_p2_split_occurrence_receipt(&render_xml, &rows)
            .expect("pruned render must match its split receipt");

        let visibility_only_render = render_xml.replace(
            "<note><chord/><pitch><step>G</step>",
            "<note print-object=\"no\"><chord/><pitch><step>G</step>",
        );
        validate_retention_first_p2_split_occurrence_receipt(&visibility_only_render, &rows)
            .expect("visibility-only drift must preserve occurrence topology");

        let duplicated_render = visibility_only_render.replace(
            "</measure>",
            "<note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note></measure>",
        );
        assert!(
            validate_retention_first_p2_split_occurrence_receipt(&duplicated_render, &rows).is_err(),
            "pitched occurrence topology drift must fail closed"
        );
    }
}

pub(crate) struct RenderRow20BlockMint { _private: () }

pub(crate) struct RenderRow21BlockMint { _private: () }

pub(crate) struct RenderRow22ExactMint { _private: () }

pub(crate) struct RenderRow55BlockMint { _private: () }

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/pruned_pdf_render_sovereignty_tests.rs"));

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/initial_render_prepared_chain_tests.rs"));

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/rest_arithmetic_sovereign_tests.rs"));


// Receipt-bearing mutation preparation belongs to the canonical structural owner.
include!("../structural_write_authority/render_polish_hidden_rest_readability_tail_receipt_ownership.rs");
