// Report-contract integration test: layout witness renewal must retain both
// actual writer-result rows and their interpretation by the unchanged manifest.
fn writer_report_fixture(forwards: usize) -> String {
    let version = env!("CARGO_PKG_VERSION");
    let written = if forwards > 0 { "ja" } else { "nein" };
    format!("ETUSCHLOPF-ERGEBNIS\nWerkzeug: etuschlopf {version}\nErgebnis: OK\n\
SilentVoiceVisibleMeasureCoverageWriterStage v{version}: Attempted ja; SelectedRenderSource ja; WriteEnabled ja; Candidates {forwards}; MeasuresWritten {forwards}; ForwardNodesReplaced {forwards}; ForwardDurationReplaced {forwards}; BeamLostBeginNotesWritten 0; SourceHigherBeamNotesRestored 0; RestVerticalPlacementCandidates 0; RestVerticalPlacementAnchorsWritten 0; RestVerticalPlacementCandidateRowsAfter 0; LedgerBoundedClefBoundariesAccepted 0; LedgerBoundedClefBoundariesWritten 0; PrimaryClefProjectionChanged nein; VisibleRestsBeforeAfter 0/{forwards}; NoteCountBeforeAfter 0/{forwards}; PitchedEventMultisetStable ja; CandidateRowsAfter 0; ExactCandidateDelta ja; RenderXmlWrite {written}; NotationMutation {written}; LegacyMusicMutation {written}; MasterApply nein; Urteil test.\n\
FinalBeamLostBeginRenderWriterStage v{version}: Attempted ja; SelectedRenderSource ja; WriteEnabled ja; Candidates 0; NotesWritten 0; CandidateRowsAfter 0; RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein; MasterApply nein; Urteil test.\n\
FinalRestVerticalPlacementRenderWriterStage v{version}: Attempted ja; SelectedRenderSource ja; WriteEnabled ja; Candidates 0; AnchorsWritten 0; CandidateRowsAfter 0; RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein; MasterApply nein; Urteil test.\n")
}

#[test]
fn layout_report_renewals_preserve_writer_outcome_for_manifest() {
    for forwards in [0usize, 3] {
        let report = writer_report_fixture(forwards);
        let mut source = FinalPdfSourceCommit {
            pipeline_line: report.clone(), closed: true,
            committed_xml: Some("witnessed".to_string()),
            committed_occurrence_continuation: None, committed_voice_plan: None,
            phrase_occurrence_course: None,
            primary_beam_voice_partition_changed_voice_elements: 0,
        };
        for _ in 0..3 {
            source.pipeline_line = source.horizontal_layout_pipeline_line();
            assert!(source.pipeline_line().starts_with(&report));
            assert_eq!(source.pipeline_line().matches("SilentVoiceVisibleMeasureCoverageWriterStage ").count(), 1);
            let evidence = crate::corpus_run_manifest::project_evidence(
                "LayoutReportTransport", Path::new("LayoutReportTransport.result.txt"),
                source.pipeline_line(), Some(0), true);
            assert!(evidence.writer_outcome_observed);
            assert_eq!(evidence.forward_nodes_replaced, forwards);
            assert_eq!(evidence.routes_written, forwards);
            assert_eq!(evidence.render_xml_write, forwards > 0);
            assert_eq!(source.committed_xml(), Some("witnessed"));
        }
    }
}

#[test]
fn layout_report_renewal_does_not_invent_missing_writer_evidence() {
    let source = FinalPdfSourceCommit {
        pipeline_line: String::new(), closed: true, committed_xml: None,
        committed_occurrence_continuation: None, committed_voice_plan: None,
        phrase_occurrence_course: None,
            primary_beam_voice_partition_changed_voice_elements: 0,
    };
    let report = source.horizontal_layout_pipeline_line();
    let evidence = crate::corpus_run_manifest::project_evidence(
        "MissingWriterEvidence", Path::new("MissingWriterEvidence.result.txt"),
        &report, Some(0), true);
    assert!(!evidence.writer_outcome_observed);
}
