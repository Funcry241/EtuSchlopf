// Canonical SWA boundary tests; included in the original private helper scope.
#[cfg(test)]
mod r1230_pruned_pdf_render_sovereignty_tests {
    use super::*;
    use crate::master_state_identity::tests::synthetic_genesis_for_tests;
    use crate::master_state_identity::MasterLineage;
    use crate::structural_write_authority::{
        MasterAdoptedState, RenderSinkKind, RenderSovereignState,
    };

    fn measure_with_safe_hidden_inner_chord(ordinal: usize) -> String {
        format!(
            "<measure number=\"{ordinal}\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><note print-object=\"no\" print-spacing=\"no\"><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note></measure>"
        )
    }

    #[test]
    fn ir1_fragment_receipts_replay_across_multiple_length_changing_measures_before_sink() {
        let master_xml = format!(
            "<score-partwise><part id=\"P1\">{}{}</part></score-partwise>",
            measure_with_safe_hidden_inner_chord(10),
            measure_with_safe_hidden_inner_chord(20),
        );
        let master = MasterAdoptedState::adopt_genesis_state(
            master_xml.clone(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(&master_xml)),
        );
        let path = std::env::temp_dir().join(format!(
            "etuschlopf-r1230-ir1-fragment-replay-{}.render.musicxml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let audit = build_render_xml_hidden_material_prune_audit(&master_xml);
        let (trial, candidate, receipts) =
            build_render_xml_pruned_pdf_trial(&master_xml, &audit, &path)
                .expect("IR1 candidate must build");
        assert!(trial.render_xml_written);
        assert_eq!(receipts.len(), 2, "one #22 fragment receipt per changed measure");
        let candidate = candidate.expect("safe IR1 candidate");

        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(true);
        state
            .adopt_transition(
                root,
                &candidate,
                receipts,
                RenderSinkKind::PrunedPdfTrial,
                |artifact_path, _pre, verified_candidate| {
                    (std::fs::write(artifact_path, verified_candidate), None)
                },
            )
            .expect("fragment replay must verify before the sink write");

        assert_eq!(state.document(), Some(candidate.as_str()));
        assert_eq!(std::fs::read_to_string(&path).ok().as_deref(), Some(candidate.as_str()));
        let _ = std::fs::remove_file(path);
    }
    #[test]
    fn ir3_generated_row17_is_consumed_by_row34_before_pianistic_rest_sink_write() {
        let ordinal = 31_usize;
        let master_xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><rest/><duration>48</duration><voice>1</voice><type>eighth</type><staff>1</staff></note><note><rest/><duration>48</duration><voice>1</voice><type>eighth</type><staff>1</staff></note><note><pitch><step>C</step><octave>4</octave></pitch><duration>288</duration><voice>1</voice><type>half</type><dot/><staff>1</staff></note></measure></part></score-partwise>"
        );
        let master = MasterAdoptedState::adopt_genesis_state(
            master_xml.clone(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(&master_xml)),
        );
        let path = std::env::temp_dir().join(format!(
            "etuschlopf-r1230-ir3-generated-consumer-{}.render.musicxml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let planner = build_render_xml_visible_rest_readability_audit(
            &master_xml,
            "r1230 IR3 generated-consumer regression",
        );
        let (trial, candidate, receipts) =
            build_render_xml_pianistic_rest_repair_trial(&master_xml, &planner, &path)
                .expect("IR3 candidate must build");
        assert!(trial.written);
        assert_eq!(
            receipts.len(),
            2,
            "IR3 must carry one #17 Generated receipt followed by the consuming #34 EXACT receipt"
        );
        let candidate = candidate.expect("safe IR3 candidate");
        assert_eq!(candidate.matches("<rest").count(), 1);
        assert!(candidate.contains("<duration>96</duration>"));

        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(false);
        state
            .adopt_transition(
                root,
                &candidate,
                receipts,
                RenderSinkKind::PianisticRestRepairTrial,
                |artifact_path, _source_xml, verified_candidate| {
                    (std::fs::write(artifact_path, verified_candidate), None)
                },
            )
            .expect("#17 Generated must be consumed by #34 before the IR3 sink write");

        assert_eq!(state.document(), Some(candidate.as_str()));
        assert_eq!(
            std::fs::read_to_string(&path).ok().as_deref(),
            Some(candidate.as_str())
        );
        let _ = std::fs::remove_file(path);
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/render_authority/ir5_row69_exact_replays_before_missing_rest_sink_write.rs"));

}
