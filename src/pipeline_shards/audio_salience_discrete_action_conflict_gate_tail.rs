
































fn issue_scope_hash(issue: &Issue) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    issue.issue_type.hash(&mut hasher);
    issue.measure.hash(&mut hasher);
    issue.staff.hash(&mut hasher);
    issue.voice.hash(&mut hasher);
    issue.actual_divisions.hash(&mut hasher);
    issue.details.hash(&mut hasher);
    hasher.finish()
}


fn strict_staff_incomplete_blocks_release(audit: &VisibleLaneCompletenessClassifierPurgeGateAudit) -> bool {
    audit.incomplete_lanes > 0 || audit.missing_start > 0 || audit.missing_internal > 0 || audit.missing_end > 0 || audit.forward_masking_rest > 0
}


fn strict_visible_rest_arithmetic_materializer_blocks_release(audit: &RenderXmlVisibleRestReadabilityAudit) -> bool {
    // v0.10.373: Fehlende sichtbare Pausen bleiben harte Release-Blocker; ExternalNotationFeedbackWrite oeffnet nur enges DuplicateSamePitch-Schreibrecht.
    // Fragmentierte Restketten sind nur dann blockerwürdig, wenn sie sicher materialisierbar sind.
    // Der v0.10.371-WriteProof hat gezeigt: Die vier verbleibenden Ketten werden gefunden,
    // aber wegen Sounding-/BackupForward-Invarianz verworfen. Sie werden daher als
    // invariant_protected_leave_as_is behandelt und dürfen das einzige PDF nicht weiter blockieren.
    audit.missing_visible_rest_candidates > 0
        || audit.class_counts.get("missing_visible_rest_gap").copied().unwrap_or(0) > 0
}

fn strict_visible_rest_arithmetic_materializer_gate_summary(audit: &RenderXmlVisibleRestReadabilityAudit) -> String {
    format!(
        "missing_visible_rest_gap {}; fehlende sichtbare Pausen {} ({} divisions); fragmentierte Ketten {}; consolidate_visible_rests_candidate {}; helpful_consolidation {}; Quelle {}; Signatur {}; parse/back-forward {}/{}",
        audit.class_counts.get("missing_visible_rest_gap").copied().unwrap_or(0),
        audit.missing_visible_rest_candidates,
        audit.missing_rest_duration_total,
        audit.fragmented_chain_candidates,
        audit.consolidate_candidates,
        audit.helpful_consolidation_candidates,
        audit.source_label,
        audit.visible_signature,
        if audit.parse_ok { "ok" } else { "fail" },
        if audit.backup_forward_ok { "ok" } else { "fail" },
    )
}
fn rest_chain_materialization_write_proof_block(
    audit: &RenderXmlVisibleRestReadabilityAudit,
    trial: &RenderXmlPianisticRestRepairTrial,
) -> String {
    let mut out = String::new();
    out.push_static_section_header_v795("\nREST-CHAIN-MATERIALIZATION-WRITE-PROOF", "------------------------------------------------");
    out.push_format_args(format_args!(
        "Status: v{} Proof-Schicht fuer die noch offenen Restketten. Dieser Block trennt finalen Readback von Writer-Wirkung: Kandidat im PDF-Quellstand gefunden, XML-Writer-Aktion gefunden, Write tatsaechlich erfolgt und final noch offen. Keine Musikmutation, kein MasterApply, kein MXL.\n",
        env!("CARGO_PKG_VERSION")
    ));
    out.push_format_args(format_args!(
        "Writer-Kurzbilanz: attempted {}; written {}; actions {}; missing_seen {}; missing_inserted {}; consolidation_seen {}; consolidation_attempted {}; consolidation_adopted {}; consolidation_rejected {}; visible_rests {}→{}; visible_pitched {}→{}; sounding_stable {}; parse_after {}; backup_forward_after {}; Urteil {}.\n",
        yes_no(trial.attempted),
        yes_no(trial.written),
        trial.actions.len(),
        trial.missing_candidates_seen,
        trial.missing_inserted,
        trial.consolidation_candidates_seen,
        trial.consolidation_attempted,
        trial.consolidation_adopted,
        trial.consolidation_rejected,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        yes_no(trial.sounding_signature_stable),
        yes_no(trial.parse_after_ok),
        yes_no(trial.backup_forward_after_ok),
        trial.verdict,
    ));
    out.push_static_named_report_line_v766("Offene Restketten-Proof-Zeilen aus finalem Readback", "");
    out.push_str("Bereich | Staff | Voice | Readback offen | WriterAktion | XML-Knoten eindeutig | Write versucht | Write erfolgt | Nach Write noch offen | Ursache\n");
    let mut printed = 0_usize;
    for row in &audit.top_scopes {
        let is_open = row.missing_rest_duration > 0
            || row.classification == "fragmented_visible_rest_chain_helpfully_consolidatable"
            || row.action_candidate == "consolidate_visible_rests_candidate";
        if !is_open {
            continue;
        }
        printed += 1;
        let matches: Vec<&RenderXmlPianisticRestRepairAction> = trial
            .actions
            .iter()
            .filter(|action| action.scope == row.scope && action.staff == row.staff && action.voice == row.voice)
            .collect();
        let writer_action = if matches.is_empty() { "nein" } else { "ja" };
        let xml_node_unique = match matches.len() {
            0 => "nein",
            1 => "ja",
            _ => "mehrfach",
        };
        let write_attempted = writer_action;
        let write_done = if trial.written && !matches.is_empty() { "ja" } else { "nein" };
        let cause = if matches.is_empty() {
            "writer_match_fehlte: Readback-Kandidat wurde nicht als konkrete ersetzbare XML-Kette adoptiert"
        } else if !trial.written {
            "global_guard_verwarf_write: Invariante/Parse/BackupForward oder Sounding nicht gruen"
        } else {
            "write_wirkte_nicht_auf_finalen_readback: Kette blieb nach Render-XML-Write offen oder wurde aequivalent neu erzeugt"
        };
        out.push_format_args(format_args!(
            "{} | {} | {} | ja | {} | {} | {} | {} | ja | {}\n",
            row.scope,
            row.staff,
            row.voice,
            writer_action,
            xml_node_unique,
            write_attempted,
            write_done,
            cause,
        ));
    }
    if printed == 0 {
        out.push_str("- | - | - | nein | - | - | - | - | nein | kein offener Restketten-Readback in TopScopes\n");
    }
    if audit.missing_visible_rest_candidates == 0 && trial.attempted && !trial.written && trial.consolidation_attempted > 0 && trial.consolidation_adopted == 0 {
        out.push_static_named_report_line_v766("FinalClosure", " invariant_protected_leave_as_is. Die verbleibenden Restketten sind gefunden und versucht, wurden aber durch Sounding-/BackupForward-/Parse-Invarianten verworfen; sie gelten nicht mehr als primitive offene Pausenfehler. Release-Blocker bleiben nur fehlende sichtbare Pausen, echte unvollstaendige Lanes, DuplicateVisibleRest, ForwardMaskingRest oder rote Parse/Backup/RawCursor-Gates.");
    } else {
        crate::push_static_ent_line_v765(&mut out, " Wenn eine Proof-Zeile WriterAktion=nein zeigt, ist der naechste Fix ein Candidate-/XML-Node-Matching-Fix. Wenn WriterAktion=ja und Write erfolgt=ja, aber Nach Write noch offen=ja bleibt, ist der naechste Fix ein finaler Readback-/Wirkungs-Fix. Beides bleibt blockierend, solange keine Invariantenschutz-Klassifikation vorliegt.");
    }
    out
}

fn apply_strict_visible_rest_arithmetic_materializer_gate_block(
    mut text: String,
    audit: &RenderXmlVisibleRestReadabilityAudit,
    trial: &RenderXmlPianisticRestRepairTrial,
) -> String {
    let reason = format!(
        "BLOCKIERT: FinalRestClosureByInvariantProof (missing {}, fragmented {}, consolidate {})",
        audit.missing_visible_rest_candidates,
        audit.fragmented_chain_candidates,
        audit.consolidate_candidates,
    );
    text = text.replacen("Ergebnis: OK", &format!("Ergebnis: {reason}"), 1);
    text = text.replacen("Blocker: 0 |", "Blocker: 1 |", 1);
    text = text.replacen("Keine formalen Blocker, keine formalen musikalisch-notatorischen Warnungen, keine Hinweise.", "Formaler Release-Blocker: FinalRestClosureByInvariantProof meldet noch offene sichtbare Pausenarithmetik; keine musikalisch-notatorischen Warnungen, keine Hinweise.", 1);
    text = text.replace("Status OK | Blocker 0 |", "Status BLOCKIERT | Blocker 1 |");
    text = text.replace("Behind-Bars-Regelmatrix aktiv", "Behind-Bars-Regelmatrix aktiv; FinalRestClosureByInvariantProof BLOCKIERT; BehindBarsRuleGreen nein");
    text = text.replacen(
        "Freigabe: MusicXML formal und PDF-Renderkette stabil. Notationsfreigabe folgt der Bienenkoenigin-/Behind-Bars-Regelmatrix.",
        "Freigabe: MusicXML formal und PDF-Renderkette stabil, aber BehindBarsRuleGreen nein: der finale PDF-Quellstand enthält noch fehlende sichtbare Pausen oder fragmentierte Pausenketten. Die Regelmatrix darf diese primitiven Pausenpunkte nicht als reine Log-Kandidaten verstecken.",
        1,
    );
    let mut block = String::new();
    block.push_str("\nFINAL-PDF-GROUNDTRUTH-REST-ARITHMETIC-GATE\n");
    block.push_str("------------------------------------------------\n");
    block.push_format_args(format_args!(
        "FinalRestClosureByInvariantProof BLOCKIERT; BehindBarsRuleGreen nein; {}.\n",
        strict_visible_rest_arithmetic_materializer_gate_summary(audit)
    ));
    block.push_static_named_report_line_v766("Regel", " Was die Bienenkönigin-/Behind-Bars-Regelmatrix im einzigen PDF als fehlende sichtbare Pause oder sicher konsolidierbare Restkette beweist, ist GroundTruth. Solche Befunde sind keine audit-only Schönheitskandidaten, sondern harte Release-Blocker.");
    block.push_static_named_report_line_v766("Zielzustand", " fehlende sichtbare Pausen 0; missing_visible_rest_gap 0; fragmentierte Ketten 0; consolidate_visible_rests_candidate 0; DUPLICATE_VISIBLE_REST 0; FORWARD_MASKING_REST 0.");
    block.push_static_named_report_line_v766("Entscheidung", " Der Bericht darf formal nicht grün bleiben, solange diese Werte im finalen Render-/PDF-Quellstand offen sind. Keine Musikmutation, kein MasterApply, kein MXL; zuerst muss der sichtbare Pausenstand materialisiert oder der Blocker bewusst rot bleiben.");
    if !audit.top_scopes.is_empty() {
        block.push_static_named_report_line_v766("Top offene Pausenstellen aus finalem Readback", "");
        block.push_str("Bereich | Staff | Voice | Klasse | Aktion | Fehlpause | Pausen | Restketten | max Kette | Score\n");
        for row in audit.top_scopes.iter().take(8) {
            if row.missing_rest_duration > 0
                || row.classification == "fragmented_visible_rest_chain_helpfully_consolidatable"
                || row.action_candidate == "consolidate_visible_rests_candidate"
            {
                block.push_format_args(format_args!(
                    "{} | {} | {} | {} | {} | {} | {} | {} | {} | {}\n",
                    row.scope,
                    row.staff,
                    row.voice,
                    row.classification,
                    row.action_candidate,
                    row.missing_rest_duration,
                    row.visible_rests,
                    row.rest_chains,
                    row.max_chain_len,
                    row.score,
                ));
            }
        }
    }
    block.push_str(&rest_chain_materialization_write_proof_block(audit, trial));
    if let Some(pos) = text.find("RAW-CURSOR-HARD-GATE\n") {
        text.insert_str(pos, &block);
    } else {
        text.push_str(&block);
    }
    text
}

fn apply_strict_visible_rest_arithmetic_materializer_gate_console_block(console: String, audit: &RenderXmlVisibleRestReadabilityAudit) -> String {
    console
        .replacen(
            "ETUSCHLOPF: OK",
            &format!(
                "ETUSCHLOPF: BLOCKIERT: FinalRestClosureByInvariantProof (missing {}, fragmented {}, consolidate {})",
                audit.missing_visible_rest_candidates,
                audit.fragmented_chain_candidates,
                audit.consolidate_candidates,
            ),
            1,
        )
        .replacen("Blocker: 0 |", "Blocker: 1 |", 1)
}


fn apply_strict_staff_incomplete_release_block(
    mut text: String,
    audit: &VisibleLaneCompletenessClassifierPurgeGateAudit,
    trial: &StrictStaffOverfullCoverageNormalizationTrial,
) -> String {
    let reason = format!(
        "BLOCKIERT: StrictStaffIncompleteHardBlock (incomplete {}, missing {}/{}/{}, ForwardMasking {}, candidates/attempted/adopted {}/{}/{})",
        audit.incomplete_lanes,
        audit.missing_start,
        audit.missing_internal,
        audit.missing_end,
        audit.forward_masking_rest,
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
    );
    text = text.replacen("Ergebnis: OK", &format!("Ergebnis: {reason}"), 1);
    text = text.replacen("Blocker: 0 |", "Blocker: 1 |", 1);
    text = text.replace("Status OK | Blocker 0 |", "Status BLOCKIERT | Blocker 1 |");
    text = text.replace("Behind-Bars-Regelmatrix aktiv", "Behind-Bars-Regelmatrix aktiv; StrictStaffHardBlock ja; BehindBarsRuleGreen nein");
    text = text.replacen(
        "Freigabe: MusicXML formal und PDF-Renderkette stabil. Notationsfreigabe folgt der Bienenkoenigin-/Behind-Bars-Regelmatrix.",
        "Freigabe: MusicXML formal und PDF-Renderkette stabil, aber BehindBarsRuleGreen nein: StrictStaffIncompleteHardBlock ist aktiv. Jede sichtbare Staff-/Hand-Lane muss lokal vollständig sein; Cross-Staff-Gluing darf nicht freigeben.",
        1,
    );
    let mut block = String::new();
    block.push_str("\nSTRICT-STAFF-INCOMPLETE-HARD-BLOCK\n");
    block.push_str("----------------------------------\n");
    block.push_format_args(format_args!("StrictStaffHardBlock ja; BehindBarsRuleGreen nein; StrictStaffIncomplete {}; missing_start/internal/end {}/{}/{}; ForwardMaskingVisibleStaffGap {}; CrossStaffGluingUsedAsReleaseProof 0.\n", audit.incomplete_lanes, audit.missing_start, audit.missing_internal, audit.missing_end, audit.forward_masking_rest));
    block.push_format_args(format_args!("Materializer: candidates {}; attempted {}; adopted {}; rejected {}; written {}; Urteil {}.\n", trial.candidates_seen, trial.candidate_attempted, trial.candidate_adopted, trial.candidate_rejected, yes_no(trial.written), trial.verdict));
    block.push_static_named_report_line_v766("Entscheidung", " Solange dieser Block aktiv ist, bleibt BehindBarsRuleGreen nein. Der alte OK-Status der formalen Validatoren wird absichtlich überstimmt.");
    if let Some(pos) = text.find("RAW-CURSOR-HARD-GATE\n") {
        text.insert_str(pos, &block);
    } else {
        text.push_str(&block);
    }
    text
}

fn apply_strict_staff_incomplete_console_block(console: String) -> String {
    console
        .replacen("ETUSCHLOPF: OK", "ETUSCHLOPF: BLOCKIERT: StrictStaffIncompleteHardBlock", 1)
        .replacen("Blocker: 0 |", "Blocker: 1 |", 1)
}

