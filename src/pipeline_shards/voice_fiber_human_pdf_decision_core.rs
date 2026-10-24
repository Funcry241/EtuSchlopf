#[derive(Debug, Clone, Default)]
struct VoiceFiberBehindBarsDecisionLedgerAudit {
    active: bool,
    checked_issues: usize,
    neutralized: usize,
    kept: usize,
    unresolved: usize,
    pdf_has_pages: bool,
    neutralized_scope_hashes: BTreeSet<u64>,
    rows: Vec<String>,
    done_fragment: String,
    pipeline_line: String,
    verdict: String,
}

fn build_voice_fiber_behind_bars_decision_ledger_no_music_mutation(report: &CheckReport, xml: &str, pdf_pages: Option<usize>) -> VoiceFiberBehindBarsDecisionLedgerAudit {
    let pdf_has_pages = pdf_pages.unwrap_or(0) > 0;
    let notes = parse_atomic_notes(xml);
    let mut audit = VoiceFiberBehindBarsDecisionLedgerAudit {
        active: true,
        pdf_has_pages,
        ..VoiceFiberBehindBarsDecisionLedgerAudit::default()
    };
    let mut issues = report
        .issues
        .iter()
        .filter(|issue| issue.issue_type == "PIANO_FAST_LARGE_JUMP")
        .collect::<Vec<_>>();
    issues.sort_by_key(|issue| {
        (
            measure_sort_key(&issue.measure),
            issue.staff.clone().unwrap_or_default(),
            issue.voice.clone().unwrap_or_default(),
            issue.actual_divisions.unwrap_or(0),
        )
    });
    audit.checked_issues = issues.len();

    for issue in issues.iter().take(24) {
        let scope_hash = issue_scope_hash(issue);
        let issue_staff = issue.staff.clone().unwrap_or_else(|| "?".to_string());
        let issue_voice = issue.voice.clone().unwrap_or_else(|| "?".to_string());
        let Some(identity) = issue.han03_jump_identity.as_ref() else {
            audit.unresolved += 1;
            audit.rows.push(format!(
                "{} | {} | {} | {} | - | - | - | - | - | - | unresolved_keep_warning | Canonical HAN03 JumpIdentity fehlt; keine Neutralisierung",
                scope_hash, issue.measure, issue_staff, issue_voice
            ));
            continue;
        };
        let line_kind = identity.line_kind.clone();
        let from_label = identity.from.pitch_label.clone();
        let to_label = identity.to.pitch_label.clone();
        let pairs = find_jump_issue_note_pairs(&notes, issue);
        let Some((from_note, to_note)) = pairs.first().copied() else {
            audit.unresolved += 1;
            audit.rows.push(format!(
                "{} | {} | {} | {} | {}→{} | - | - | - | - | - | unresolved_keep_warning | A/B nicht im Eventinventar gefunden; keine Neutralisierung",
                scope_hash, issue.measure, issue_staff, issue_voice, from_label, to_label
            ));
            continue;
        };

        let direct_ab = identity.temporal_edge.direct_attack_successor;
        let mut same_staff_voice_notes = notes.iter()
            .filter(|note| note.measure == from_note.measure
                && note.staff == from_note.staff
                && note.voice == from_note.voice
                && note.visible
                && note.pitch.is_some())
            .collect::<Vec<_>>();
        same_staff_voice_notes.sort_by_key(|note| (note.start, note.note_start));
        let prev_same_staff_voice = same_staff_voice_notes
            .iter()
            .copied()
            .rfind(|note| {
                note.start < from_note.start
                    || (note.start == from_note.start
                        && note.note_start < from_note.note_start)
            });
        let next_same_staff_voice = same_staff_voice_notes.iter()
            .copied()
            .find(|note| note.start > to_note.start || (note.start == to_note.start && note.note_start > to_note.note_start));
        let overlap_a_count = notes.iter()
            .filter(|note| note.measure == from_note.measure
                && note.visible
                && note.pitch.is_some()
                && !same_atomic_note(note, from_note)
                && !same_atomic_note(note, to_note)
                && note.start <= from_note.start
                && from_note.start < note.start + note.duration.max(1))
            .count();
        let overlap_b_count = notes.iter()
            .filter(|note| note.measure == to_note.measure
                && note.visible
                && note.pitch.is_some()
                && !same_atomic_note(note, from_note)
                && !same_atomic_note(note, to_note)
                && note.start <= to_note.start
                && to_note.start < note.start + note.duration.max(1))
            .count();
        let same_voice_continuity = direct_ab
            && prev_same_staff_voice.is_some()
            && next_same_staff_voice.is_some();
        let lower_voice_high_fiber = from_note.staff == "2" && line_kind.contains("high");
        let cross_fiber_supported = overlap_a_count > 0 && overlap_b_count > 0;
        let neutralize = pdf_has_pages
            && lower_voice_high_fiber
            && same_voice_continuity
            && cross_fiber_supported;
        let decision = if neutralize {
            audit.neutralized += 1;
            audit.neutralized_scope_hashes.insert(scope_hash);
            "cross_fiber_classifier_warning_neutralized"
        } else {
            audit.kept += 1;
            "keep_warning_no_behind_bars_proof"
        };
        audit.rows.push(format!(
            "{} | {} | {} | {} | {} | {}→{} | DirectAtoB {} | Prev/Next {}/{} | OverlapAtA {} | OverlapAtB {} | {} | {}",
            scope_hash,
            issue.measure,
            from_note.staff,
            from_note.voice,
            line_kind,
            atomic_note_pitch_label(from_note),
            atomic_note_pitch_label(to_note),
            yes_no(direct_ab),
            yes_no(prev_same_staff_voice.is_some()),
            yes_no(next_same_staff_voice.is_some()),
            overlap_a_count,
            overlap_b_count,
            decision,
            if neutralize { "Warnung aus Blockerpfad genommen; keine Note geändert" } else { "Warnung bleibt prüfpflichtig" }
        ));
    }

    audit.verdict = if audit.checked_issues == 0 {
        "voice_fiber_behind_bars_decision_ledger_not_required_no_music_mutation".to_string()
    } else if audit.neutralized == audit.checked_issues && audit.unresolved == 0 {
        "voice_fiber_behind_bars_decision_ledger_all_residuals_neutralized_no_music_mutation".to_string()
    } else if audit.neutralized > 0 {
        "voice_fiber_behind_bars_decision_ledger_partial_neutralization_no_music_mutation".to_string()
    } else {
        "voice_fiber_behind_bars_decision_ledger_no_neutralization_no_music_mutation".to_string()
    };
    audit.done_fragment = format!(
        "VoiceFiberBehindBarsDecisionLedgerNoMusicMutation: active {}, geprüft {}, neutralisiert {}, behalten {}, unresolved {}, PdfHasPages {}, MusicMutation nein, MasterApply nein, Urteil {}",
        yes_no(audit.active),
        audit.checked_issues,
        audit.neutralized,
        audit.kept,
        audit.unresolved,
        yes_no(audit.pdf_has_pages),
        audit.verdict
    );
    audit.pipeline_line = format!(
        "VoiceFiberBehindBarsDecisionLedgerNoMusicMutation v{}: active {}; residuale Voice/Faser-FastJump-Warnungen werden nach maschinellem Behind-Bars-Beweis als CrossFiber-/Classifier-Fälle neutralisiert; geprüft {}, neutralisiert {}, behalten {}; keine Note, kein Staff, keine Voice, kein Pitch, kein Onset, keine Dauer, keine MusicXML-Mutation, kein MasterApply.",
        env!("CARGO_PKG_VERSION"),
        yes_no(audit.active),
        audit.checked_issues,
        audit.neutralized,
        audit.kept
    );
    console_progress(audit.pipeline_line.clone());
    for row in &audit.rows {
        console_progress(format!("VoiceFiberBehindBarsCausality: {row}"));
    }
    audit
}

fn apply_voice_fiber_behind_bars_decision_ledger_to_report(report: &mut CheckReport, audit: &VoiceFiberBehindBarsDecisionLedgerAudit) -> String {
    if !audit.active {
        refresh_report_summary(report);
        return format!(
            "VoiceFiberBehindBarsDecisionLedgerApply: Ledger inaktiv; Validator bleibt {}/{}/{}; MusicMutation nein; MasterApply nein.",
            report.summary.blockers,
            report.summary.warnings,
            report.summary.hints
        );
    }
    if audit.neutralized_scope_hashes.is_empty() {
        refresh_report_summary(report);
        return format!(
            "VoiceFiberBehindBarsDecisionLedgerApply: keine neutralisierte residuale FastJump-Warnung; Validator bleibt {}/{}/{}; MusicMutation nein; MasterApply nein.",
            report.summary.blockers,
            report.summary.warnings,
            report.summary.hints
        );
    }
    let before_blockers = report.summary.blockers;
    let before_warnings = report.summary.warnings;
    let before_hints = report.summary.hints;
    let before_len = report.issues.len();
    report.issues.retain(|issue| {
        !(issue.issue_type == "PIANO_FAST_LARGE_JUMP" && audit.neutralized_scope_hashes.contains(&issue_scope_hash(issue)))
    });
    refresh_report_summary(report);
    let removed = before_len.saturating_sub(report.issues.len());
    format!(
        "VoiceFiberBehindBarsDecisionLedgerApply: {} FastJump-Warnung(en) nach maschinellem Behind-Bars-Beweis als CrossFiber-/Classifier-Warnung neutralisiert; Validator {}/{}/{} -> {}/{}/{}; MusicMutation nein; MusicXML unverändert; MasterApply nein.",
        removed,
        before_blockers,
        before_warnings,
        before_hints,
        report.summary.blockers,
        report.summary.warnings,
        report.summary.hints
    )
}
