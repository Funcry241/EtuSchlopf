// v0.10.1176 RelaxedClosureAndRetiredRuntimePrune:
// A render-side write is only productively materialized when the complete
// success proof is green. Missing renderer capability still blocks the success
// claim, but no longer blocks an otherwise safe PDF release. Release authority
// remains hard only for selected render input with broken cursor, contiguity,
// event equivalence or file binding. This changes no MusicXML and no PDF.

#[derive(Debug, Clone, Default)]
struct RenderWritePdfSourceAdoptionTruthGateAudit {
    writer_label: String,
    pdf_requested: bool,
    write_produced: bool,
    raw_cursor_guard_checked: bool,
    raw_cursor_green: bool,
    raw_cursor_measures: usize,
    raw_cursor_hard_violations: usize,
    raw_cursor_max_overflow: i64,
    raw_cursor_partial_backups: usize,
    raw_cursor_post_expected_events: usize,
    renderer_contiguity_green: bool,
    renderer_capability_green: bool,
    materialization_equivalence_green: bool,
    candidate_file_exact_match: bool,
    pdf_source_selected: bool,
    pdf_source_adopted: bool,
    pdf_source_label: String,
    write_validated: bool,
    productive_materialization_green: bool,
    success_claim_blocked: bool,
    hard_safety_green: bool,
    blocks_release: bool,
    verdict: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct RenderXmlPdfSourceRawCursorGuardEvidence {
    checked: bool,
    green: bool,
    measures: usize,
    hard_violations: usize,
    max_overflow: i64,
    partial_backups: usize,
    post_expected_events: usize,
}

#[allow(clippy::too_many_arguments)]
fn build_render_write_pdf_source_adoption_truth_gate(
    writer_label: &str,
    pdf_requested: bool,
    write_produced: bool,
    raw_cursor: RenderXmlPdfSourceRawCursorGuardEvidence,
    renderer_contiguity_green: bool,
    renderer_capability_green: bool,
    materialization_equivalence_green: bool,
    candidate_file_exact_match: bool,
    pdf_source_selected: bool,
    pdf_source_label: &str,
) -> RenderWritePdfSourceAdoptionTruthGateAudit {
    let write_validated = write_produced
        && raw_cursor.checked
        && raw_cursor.green
        && renderer_contiguity_green
        && renderer_capability_green
        && materialization_equivalence_green
        && candidate_file_exact_match;
    // Selecting Render-MusicXML as review input is not yet adoption as a
    // successful writer delta. Adoption additionally requires validation.
    let pdf_source_adopted = pdf_source_selected && write_validated;
    // Productive materialization is a positive claim, never an N/A shortcut.
    // It requires the complete chain: produced, validated and adopted.
    let productive_materialization_green = write_produced && write_validated && pdf_source_adopted;
    let success_claim_blocked = write_produced && pdf_requested && !productive_materialization_green;
    let hard_safety_green = raw_cursor.checked
        && raw_cursor.green
        && renderer_contiguity_green
        && materialization_equivalence_green
        && candidate_file_exact_match;
    // Renderer capability is a success/adoption proof, not a structural safety
    // invariant. A selected render source blocks release only when one of the
    // hard invariants is red. An unselected candidate cannot block the product.
    let blocks_release = write_produced
        && pdf_requested
        && pdf_source_selected
        && !hard_safety_green;
    let verdict = if !write_produced {
        "render_write_pdf_source_adoption_not_applicable_no_write"
    } else if !pdf_requested {
        "render_write_pdf_source_adoption_not_applicable_no_pdf"
    } else if !raw_cursor.checked {
        "render_write_produced_but_validation_missing"
    } else if !raw_cursor.green {
        "render_write_produced_but_quarantined_raw_cursor_red"
    } else if !renderer_contiguity_green {
        "render_write_produced_but_renderer_contiguity_red"
    } else if !renderer_capability_green {
        "render_write_xml_green_but_renderer_cross_staff_capability_unproven"
    } else if !materialization_equivalence_green {
        "render_write_produced_but_materialization_equivalence_red"
    } else if !candidate_file_exact_match {
        "render_write_produced_but_candidate_file_binding_mismatch"
    } else if !pdf_source_selected {
        "render_write_validated_but_pdf_source_not_adopted"
    } else {
        "render_write_validated_and_pdf_source_adopted"
    }
    .to_string();

    RenderWritePdfSourceAdoptionTruthGateAudit {
        writer_label: writer_label.to_string(),
        pdf_requested,
        write_produced,
        raw_cursor_guard_checked: raw_cursor.checked,
        raw_cursor_green: raw_cursor.green,
        raw_cursor_measures: raw_cursor.measures,
        raw_cursor_hard_violations: raw_cursor.hard_violations,
        raw_cursor_max_overflow: raw_cursor.max_overflow,
        raw_cursor_partial_backups: raw_cursor.partial_backups,
        raw_cursor_post_expected_events: raw_cursor.post_expected_events,
        renderer_contiguity_green,
        renderer_capability_green,
        materialization_equivalence_green,
        candidate_file_exact_match,
        pdf_source_selected,
        pdf_source_adopted,
        pdf_source_label: pdf_source_label.to_string(),
        write_validated,
        productive_materialization_green,
        success_claim_blocked,
        hard_safety_green,
        blocks_release,
        verdict,
    }
}

fn append_render_write_pdf_source_adoption_truth_gate_section(
    out: &mut String,
    audit: &RenderWritePdfSourceAdoptionTruthGateAudit,
) {
    out.push_str("\nRENDER-WRITE-PDF-SOURCE-ADOPTION-TRUTH-GATE\n");
    out.push_str("------------------------------------------------\n");
    out.push_format_args(format_args!(
        "Kurz: Writer {}; PdfRequested {}; WriteProduced {}; WriteValidated {}; RendererContiguousTargetMembers {}; RendererCrossStaffCapabilityGreen {}; MaterializationEquivalenceGreen {}; CandidateFileExactMatch {}; PdfSourceSelected {}; PdfSourceAdopted {}; ProductiveMaterializationGreen {}; SuccessClaimBlocked {}; HardSafetyGreen {}; ReleaseBlocker {}; Urteil {}.\n",
        audit.writer_label,
        yes_no(audit.pdf_requested),
        yes_no(audit.write_produced),
        yes_no(audit.write_validated),
        yes_no(audit.renderer_contiguity_green),
        yes_no(audit.renderer_capability_green),
        yes_no(audit.materialization_equivalence_green),
        yes_no(audit.candidate_file_exact_match),
        yes_no(audit.pdf_source_selected),
        yes_no(audit.pdf_source_adopted),
        yes_no(audit.productive_materialization_green),
        yes_no(audit.success_claim_blocked),
        yes_no(audit.hard_safety_green),
        yes_no(audit.blocks_release),
        audit.verdict,
    ));
    out.push_format_args(format_args!(
        "RawCursorGuard: checked {}; green {}; Measures {}; HardViolations {}; MaxOverflow {}; PartialBackups {}; PostExpectedEvents {}.\n",
        yes_no(audit.raw_cursor_guard_checked),
        yes_no(audit.raw_cursor_green),
        audit.raw_cursor_measures,
        audit.raw_cursor_hard_violations,
        audit.raw_cursor_max_overflow,
        audit.raw_cursor_partial_backups,
        audit.raw_cursor_post_expected_events,
    ));
    out.push_format_args(format_args!(
        "PostWriteBinding: RendererContiguousTargetMembers {}; RendererCrossStaffCapabilityGreen {}; MaterializationEquivalenceGreen {}; CandidateFileExactMatch {}.\n",
        yes_no(audit.renderer_contiguity_green),
        yes_no(audit.renderer_capability_green),
        yes_no(audit.materialization_equivalence_green),
        yes_no(audit.candidate_file_exact_match),
    ));
    out.push_format_args(format_args!(
        "PdfSource: selected {}; adopted_as_success {}; label {}.\n",
        yes_no(audit.pdf_source_selected),
        yes_no(audit.pdf_source_adopted),
        audit.pdf_source_label,
    ));
    out.push_static_named_report_line_v766(
        "Wahrheitsregel",
        " Written bedeutet nur, dass ein Render-XML-Kandidat erzeugt wurde. Validated und PdfSourceAdopted bleiben strenge Voraussetzungen fuer die positive Erfolgsbehauptung. Fuer die Produktfreigabe sind dagegen nur harte Sicherheitsinvarianten veto-berechtigt: Raw-Cursor, renderer-seitige Kontiguitaet, Ereignisaequivalenz und exakte Dateibindung der tatsaechlich gewaehlten PDF-Quelle.",
    );
    out.push_static_named_report_line_v766(
        "Entspannte Autoritaet",
        " Ein fehlender positiver Renderer-Faehigkeitsbeweis verhindert weiterhin die Behauptung eines erfolgreichen Writer-Deltas, ist aber bei sonst gruenen Invarianten nur noch ein Hinweis fuer den Behind-Bars-Evidenzstatus und kein eigener roter Exit.",
    );
    out.push_static_named_report_line_v766(
        "Keine Mutation",
        " Dieses Gate schreibt weder MusicXML noch Render-MusicXML oder PDF. Es korrigiert ausschliesslich die Freigabe- und Erfolgswahrheit nach der finalen PDF-Quellenentscheidung.",
    );
}

fn append_render_candidate_and_selected_source_truth_projection_closure(
    out: &mut String,
    rejected_render_candidate: &RenderWritePdfSourceAdoptionTruthGateAudit,
    selected_pdf_source_raw_cursor: &RawCursorInvariantHardGateAudit,
    final_serialized_musicxml_temporal_artifact_observation: &RawCursorInvariantHardGateAudit,
) {
    let selected_source_label = if rejected_render_candidate.pdf_source_label == "MusicXML" {
        "Master-MusicXML"
    } else {
        rejected_render_candidate.pdf_source_label.as_str()
    };
    out.push_str("\nRENDER-CANDIDATE-AND-SELECTED-SOURCE-TRUTH-PROJECTION-CLOSURE\n");
    out.push_str("-----------------------------------------------------------------\n");
    out.push_format_args(format_args!(
        "Status: v{} Wahrheitsabschluss; Renderkandidat und tatsaechlich gewaehlte PDF-Quelle sind getrennte, bindbare Beweisobjekte.\n",
        env!("CARGO_PKG_VERSION")
    ));
    if rejected_render_candidate.write_produced
        && rejected_render_candidate.write_validated
        && rejected_render_candidate.pdf_source_adopted
    {
        out.push_format_args(format_args!(
            "RenderCandidateRawCursor: Writer {}; WriteProduced {}; Checked {}; Green {}; HardViolations {}; PartialBackups {}; Quarantined nein; RendererContiguousTargetMembers {}; RendererCrossStaffCapabilityGreen {}; MaterializationEquivalenceGreen {}; CandidateFileExactMatch {}; PdfSourceSelected {}; PdfSourceAdopted {}; ProductiveMaterializationGreen {}.\n",
            rejected_render_candidate.writer_label,
            yes_no(rejected_render_candidate.write_produced),
            yes_no(rejected_render_candidate.raw_cursor_guard_checked),
            yes_no(rejected_render_candidate.raw_cursor_green),
            rejected_render_candidate.raw_cursor_hard_violations,
            rejected_render_candidate.raw_cursor_partial_backups,
            yes_no(rejected_render_candidate.renderer_contiguity_green),
            yes_no(rejected_render_candidate.renderer_capability_green),
            yes_no(rejected_render_candidate.materialization_equivalence_green),
            yes_no(rejected_render_candidate.candidate_file_exact_match),
            yes_no(rejected_render_candidate.pdf_source_selected),
            yes_no(rejected_render_candidate.pdf_source_adopted),
            yes_no(rejected_render_candidate.productive_materialization_green),
        ));
    } else if rejected_render_candidate.write_produced {
        out.push_format_args(format_args!(
            "RejectedRenderCandidateRawCursor: Writer {}; WriteProduced {}; Checked {}; Green {}; HardViolations {}; PartialBackups {}; Quarantined {}; RendererContiguousTargetMembers {}; RendererCrossStaffCapabilityGreen {}; MaterializationEquivalenceGreen {}; CandidateFileExactMatch {}; PdfSourceSelected {}; PdfSourceAdopted {}; ProductiveMaterializationGreen {}.\n",
            rejected_render_candidate.writer_label,
            yes_no(rejected_render_candidate.write_produced),
            yes_no(rejected_render_candidate.raw_cursor_guard_checked),
            yes_no(rejected_render_candidate.raw_cursor_green),
            rejected_render_candidate.raw_cursor_hard_violations,
            rejected_render_candidate.raw_cursor_partial_backups,
            yes_no(!rejected_render_candidate.hard_safety_green),
            yes_no(rejected_render_candidate.renderer_contiguity_green),
            yes_no(rejected_render_candidate.renderer_capability_green),
            yes_no(rejected_render_candidate.materialization_equivalence_green),
            yes_no(rejected_render_candidate.candidate_file_exact_match),
            yes_no(rejected_render_candidate.pdf_source_selected),
            yes_no(rejected_render_candidate.pdf_source_adopted),
            yes_no(rejected_render_candidate.productive_materialization_green),
        ));
    } else {
        out.push_format_args(format_args!(
            "RenderCandidateRawCursor: Writer {}; WriteProduced nein; Checked {}; CandidateAbsent ja; Quarantined nein; PdfSourceAdopted nein; ProductiveMaterializationGreen nein.\n",
            rejected_render_candidate.writer_label,
            yes_no(rejected_render_candidate.raw_cursor_guard_checked),
        ));
    }
    out.push_format_args(format_args!(
        "FinalSerializedMusicXmlTemporalArtifactObservation: SnapshotAuthority FinalNotationLayoutSnapshotSet; MasterPresent yes; MasterParseOk {}; RenderPresent {}; RenderParseOk {}; Master CursorOverflow {} NoteEndOverflow {} BackupUnderflow {} ChordTemporalConflict {}; Render CursorOverflow {} NoteEndOverflow {} BackupUnderflow {} ChordTemporalConflict {}; PdfSourceSelectionIndependent yes; ObservationGreen {}; NewReleaseVeto no; ExistingRawCursorGateUnchanged yes; RepairAuthority none.\n",
        yes_no(final_serialized_musicxml_temporal_artifact_observation.master_parse_ok),
        yes_no(final_serialized_musicxml_temporal_artifact_observation.render_present),
        yes_no(final_serialized_musicxml_temporal_artifact_observation.render_parse_ok),
        final_serialized_musicxml_temporal_artifact_observation.master_findings.cursor_overflow,
        final_serialized_musicxml_temporal_artifact_observation.master_findings.note_end_overflow,
        final_serialized_musicxml_temporal_artifact_observation.master_findings.backup_underflow,
        final_serialized_musicxml_temporal_artifact_observation
            .master_findings
            .chord_temporal_conflict,
        final_serialized_musicxml_temporal_artifact_observation.render_findings.cursor_overflow,
        final_serialized_musicxml_temporal_artifact_observation.render_findings.note_end_overflow,
        final_serialized_musicxml_temporal_artifact_observation.render_findings.backup_underflow,
        final_serialized_musicxml_temporal_artifact_observation
            .render_findings
            .chord_temporal_conflict,
        yes_no(final_serialized_musicxml_temporal_artifact_observation.temporal_observation_green),
    ));
    out.push_format_args(format_args!(
        "SelectedPdfSourceRawCursor: Source {}; HardViolations {}; MasterHard {}; SelectedRenderHard {}; PartialBackups {}; Green {}.\n",
        selected_source_label,
        selected_pdf_source_raw_cursor.hard_violations,
        selected_pdf_source_raw_cursor.master_hard_violations,
        selected_pdf_source_raw_cursor.render_hard_violations,
        selected_pdf_source_raw_cursor.partial_backup_count,
        yes_no(selected_pdf_source_raw_cursor.hard_violations == 0),
    ));
    out.push_format_args(format_args!(
        "TruthProjection: RenderCandidate und SelectedPdfSource nicht vermischt; SelectedPdfSource {}; SelectedPdfInputUsesWriterDelta {}; WriterDeltaAdoptedAsSuccess {}; ReleaseBlocker {}; MusicXmlWrite nein; RenderXmlWrite nein; PdfWrite nein; MusicMutation nein; Urteil {}.\n",
        selected_source_label,
        yes_no(rejected_render_candidate.pdf_source_selected),
        yes_no(rejected_render_candidate.pdf_source_adopted),
        yes_no(rejected_render_candidate.blocks_release),
        if rejected_render_candidate.blocks_release {
            "render_candidate_hard_safety_red_release_blocked"
        } else if rejected_render_candidate.pdf_source_adopted {
            "render_candidate_adopted_selected_pdf_source_truth_consistent"
        } else if rejected_render_candidate.success_claim_blocked {
            "render_success_unproven_release_advisory"
        } else {
            "render_candidate_and_selected_pdf_source_truth_consistent"
        }
    ));
}

fn increment_blocker_count_tokens(text: String) -> String {
    fn increment_pattern(mut text: String, prefix: &str, suffix: &str) -> String {
        let mut search_from = 0usize;
        while let Some(relative_start) = text[search_from..].find(prefix) {
            let number_start = search_from + relative_start + prefix.len();
            let Some(relative_end) = text[number_start..].find(suffix) else {
                break;
            };
            let number_end = number_start + relative_end;
            let token = &text[number_start..number_end];
            let Ok(value) = token.parse::<usize>() else {
                search_from = number_end + suffix.len();
                continue;
            };
            let replacement = (value + 1).to_string();
            text.replace_range(number_start..number_end, &replacement);
            search_from = number_start + replacement.len() + suffix.len();
        }
        text
    }

    let text = increment_pattern(text, "Blocker: ", " |");
    increment_pattern(text, "Status BLOCKIERT | Blocker ", " |")
}

fn apply_render_write_pdf_source_adoption_truth_gate_release_block(
    mut text: String,
    audit: &RenderWritePdfSourceAdoptionTruthGateAudit,
    already_blocked: bool,
) -> String {
    if !audit.blocks_release {
        return text;
    }

    if already_blocked {
        text = text.replacen(
            "Ergebnis: BLOCKIERT:",
            "Ergebnis: BLOCKIERT: RenderWritePdfSourceAdoptionTruthGate +",
            1,
        );
        text = increment_blocker_count_tokens(text);
    } else {
        text = text.replacen(
            "Ergebnis: OK",
            "Ergebnis: BLOCKIERT: RenderWritePdfSourceAdoptionTruthGate",
            1,
        );
        text = text.replace("Blocker: 0 |", "Blocker: 1 |");
        text = text.replace("Status OK | Blocker 0 |", "Status BLOCKIERT | Blocker 1 |");
    }
    text
}

fn apply_render_write_pdf_source_adoption_truth_gate_console_block(
    mut console: String,
    already_blocked: bool,
) -> String {
    if already_blocked {
        console = console.replacen(
            "ETUSCHLOPF: BLOCKIERT:",
            "ETUSCHLOPF: BLOCKIERT: RenderWritePdfSourceAdoptionTruthGate +",
            1,
        );
        console = increment_blocker_count_tokens(console);
    } else {
        console = console.replacen(
            "ETUSCHLOPF: OK",
            "ETUSCHLOPF: BLOCKIERT: RenderWritePdfSourceAdoptionTruthGate",
            1,
        );
        console = console.replace("Blocker: 0 |", "Blocker: 1 |");
    }
    console
}

#[cfg(test)]
mod render_write_pdf_source_adoption_truth_gate_tests {
    use super::*;

    fn raw(green: bool) -> RenderXmlPdfSourceRawCursorGuardEvidence {
        RenderXmlPdfSourceRawCursorGuardEvidence {
            checked: true,
            green,
            measures: 44,
            hard_violations: if green { 0 } else { 1 },
            max_overflow: 0,
            partial_backups: if green { 0 } else { 1 },
            post_expected_events: 0,
        }
    }

    #[test]
    fn write_is_green_only_after_validation_and_pdf_adoption() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(true),
            true,
            true,
            true,
            true,
            true,
            "Render-XML",
        );
        assert!(audit.write_validated);
        assert!(audit.pdf_source_adopted);
        assert!(audit.productive_materialization_green);
        assert!(!audit.blocks_release);
    }

    #[test]
    fn unselected_quarantined_candidate_does_not_block_release() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(false),
            true,
            true,
            true,
            true,
            false,
            "MusicXML",
        );
        assert!(audit.write_produced);
        assert!(!audit.write_validated);
        assert!(!audit.pdf_source_adopted);
        assert!(!audit.productive_materialization_green);
        assert!(audit.success_claim_blocked);
        assert!(!audit.blocks_release);
        assert_eq!(
            audit.verdict,
            "render_write_produced_but_quarantined_raw_cursor_red"
        );
    }

    #[test]
    fn selected_quarantined_candidate_still_blocks_release() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(false),
            true,
            true,
            true,
            true,
            true,
            "Render-MusicXML",
        );
        assert!(audit.success_claim_blocked);
        assert!(!audit.hard_safety_green);
        assert!(audit.blocks_release);
    }

    #[test]
    fn no_write_is_nonmaterialized_without_becoming_a_blocker() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            false,
            RenderXmlPdfSourceRawCursorGuardEvidence::default(),
            true,
            true,
            false,
            false,
            false,
            "MusicXML",
        );
        assert!(!audit.productive_materialization_green);
        assert!(!audit.blocks_release);
        assert_eq!(
            audit.verdict,
            "render_write_pdf_source_adoption_not_applicable_no_write"
        );
    }

    #[test]
    fn no_pdf_request_is_not_productive_materialization() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            false,
            true,
            raw(true),
            true,
            true,
            true,
            true,
            false,
            "MusicXML",
        );
        assert!(audit.write_produced);
        assert!(audit.write_validated);
        assert!(!audit.productive_materialization_green);
        assert!(!audit.blocks_release);
        assert_eq!(
            audit.verdict,
            "render_write_pdf_source_adoption_not_applicable_no_pdf"
        );
    }

    #[test]
    fn renderer_contiguity_failure_blocks_adoption() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(true),
            false,
            true,
            true,
            true,
            true,
            "Render-MusicXML",
        );
        assert!(!audit.write_validated);
        assert!(!audit.pdf_source_adopted);
        assert!(!audit.productive_materialization_green);
        assert!(!audit.hard_safety_green);
        assert!(audit.blocks_release);
        assert_eq!(
            audit.verdict,
            "render_write_produced_but_renderer_contiguity_red"
        );
    }

    #[test]
    fn renderer_capability_without_roundtrip_blocks_success_claim() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(true),
            true,
            false,
            true,
            true,
            true,
            "Render-MusicXML",
        );
        assert!(audit.write_produced);
        assert!(!audit.write_validated);
        assert!(audit.pdf_source_selected);
        assert!(!audit.pdf_source_adopted);
        assert!(!audit.productive_materialization_green);
        assert!(audit.success_claim_blocked);
        assert!(audit.hard_safety_green);
        assert!(!audit.blocks_release);
        assert_eq!(
            audit.verdict,
            "render_write_xml_green_but_renderer_cross_staff_capability_unproven"
        );
    }

    #[test]
    fn semantic_equivalence_failure_blocks_adoption() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(true),
            true,
            true,
            false,
            true,
            true,
            "Render-MusicXML",
        );
        assert!(!audit.write_validated);
        assert!(!audit.hard_safety_green);
        assert!(audit.blocks_release);
        assert_eq!(
            audit.verdict,
            "render_write_produced_but_materialization_equivalence_red"
        );
    }

    #[test]
    fn candidate_file_mismatch_blocks_adoption() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(true),
            true,
            true,
            true,
            false,
            true,
            "Render-MusicXML",
        );
        assert!(!audit.write_validated);
        assert!(!audit.hard_safety_green);
        assert!(audit.blocks_release);
        assert_eq!(
            audit.verdict,
            "render_write_produced_but_candidate_file_binding_mismatch"
        );
    }

    #[test]
    fn truth_projection_names_rejected_candidate_and_selected_source_separately() {
        let rejected = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(false),
            true,
            true,
            true,
            true,
            false,
            "MusicXML",
        );
        let selected = RawCursorInvariantHardGateAudit {
            hard_violations: 0,
            master_hard_violations: 0,
            render_hard_violations: 0,
            partial_backup_count: 0,
            ..RawCursorInvariantHardGateAudit::default()
        };
        let mut report = String::new();
        let final_serialized = RawCursorInvariantHardGateAudit {
            master_parse_ok: true,
            master_findings: SerializedTemporalFindingCounts {
                chord_temporal_conflict: 7,
                ..SerializedTemporalFindingCounts::default()
            },
            ..RawCursorInvariantHardGateAudit::default()
        };
        append_render_candidate_and_selected_source_truth_projection_closure(
            &mut report,
            &rejected,
            &selected,
            &final_serialized,
        );
        assert!(report.contains("RejectedRenderCandidateRawCursor"));
        assert!(report.contains("Green nein"));
        assert!(report.contains("SelectedPdfSourceRawCursor: Source Master-MusicXML"));
        assert!(
            report.contains("SelectedPdfSourceRawCursor: Source Master-MusicXML; HardViolations 0")
        );
        assert!(report.contains("SelectedPdfInputUsesWriterDelta nein"));
        assert!(report.contains("WriterDeltaAdoptedAsSuccess nein"));
        assert!(report.contains("nicht vermischt"));
        assert!(report.contains(
            "FinalSerializedMusicXmlTemporalArtifactObservation: SnapshotAuthority FinalNotationLayoutSnapshotSet"
        ));
        assert!(report.contains(
            "Master CursorOverflow 0 NoteEndOverflow 0 BackupUnderflow 0 ChordTemporalConflict 7"
        ));
        assert!(report.contains("NewReleaseVeto no"));
        assert!(report.contains("ExistingRawCursorGateUnchanged yes"));
    }

    #[test]
    fn truth_projection_names_adopted_candidate_as_selected_render_source() {
        let adopted = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(true),
            true,
            true,
            true,
            true,
            true,
            "Render-MusicXML nach AuthorizedCursorNeutralWriter",
        );
        let selected = RawCursorInvariantHardGateAudit {
            hard_violations: 0,
            master_hard_violations: 0,
            render_hard_violations: 0,
            partial_backup_count: 0,
            ..RawCursorInvariantHardGateAudit::default()
        };
        let mut report = String::new();
        let final_serialized = RawCursorInvariantHardGateAudit::default();
        append_render_candidate_and_selected_source_truth_projection_closure(
            &mut report,
            &adopted,
            &selected,
            &final_serialized,
        );
        assert!(report.contains("RenderCandidateRawCursor"));
        assert!(!report.contains("RejectedRenderCandidateRawCursor"));
        assert!(report.contains("SelectedPdfInputUsesWriterDelta ja"));
        assert!(report.contains("WriterDeltaAdoptedAsSuccess ja"));
        assert!(report.contains("ProductiveMaterializationGreen ja"));
    }

    #[test]
    fn second_release_blocker_is_counted_separately() {
        let audit = build_render_write_pdf_source_adoption_truth_gate(
            "ReferenceWriter",
            true,
            true,
            raw(false),
            true,
            true,
            true,
            true,
            true,
            "Render-MusicXML",
        );
        let report = "Ergebnis: BLOCKIERT: Existing\nBlocker: 3 | Warnungen: 0\nStatus BLOCKIERT | Blocker 3 |".to_string();
        let report =
            apply_render_write_pdf_source_adoption_truth_gate_release_block(report, &audit, true);
        assert!(report.contains("Blocker: 4 |"));
        assert!(report.contains("Status BLOCKIERT | Blocker 4 |"));
    }
}
