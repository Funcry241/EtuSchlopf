use super::*;

#[test]
fn final_rest_placement_consumes_the_late_beam_voice_relation_once() {
    let scope = 43usize;
    let input = crate::pipeline::actual_beam_owner_capture_tests::input()
        .replace("probe", &scope.to_string());
    let (canonical, voices, source, prepared) = crate::structural_write_authority::
        CanonicalFinalChordStructuralPlan::original_beam_alignment_test_fixture(&input);
    assert!(prepared.changed_voice_elements() > 0);
    let (_, premature, trial) = stage_final_rest_vertical_placement_candidate(source.xml(), true);
    trial.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    assert_eq!(premature.candidates, 0);
    let edits = prepared
        .edits()
        .iter()
        .map(|edit| {
            (
                edit.voice_text_start(),
                edit.voice_text_end(),
                edit.target_label().to_string(),
            )
        })
        .collect();
    let (old_order, old_receipt) =
        crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
            source.xml(),
            edits,
        );
    let old_order = old_order.unwrap();
    assert_eq!(
        old_receipt
            .unwrap()
            .realize_post_document(source.xml())
            .as_deref(),
        Some(old_order.as_str())
    );
    let newly_required = final_rest_structural_targets(&old_order).unwrap().len();
    assert!(
        newly_required > 0,
        "late lane completion exposes stale glyph anchors"
    );

    let mut closure = super::materialize_silent_voice_visible_measure_coverage_for_pdf_source(
        std::path::Path::new("synthetic-lane-completion.render.musicxml"),
        true,
        &canonical,
        &voices,
        &std::collections::BTreeSet::new(),
        &source,
        Some(&prepared),
    );
    let candidate = closure.candidate_xml.take().expect("prepared final source");
    closure
        .receipts
        .discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    assert!(
        final_rest_structural_targets(&candidate)
            .unwrap()
            .is_empty()
    );
    let snapshot = closure.finish.take().unwrap()(&candidate);
    assert!(snapshot.closed, "{}", snapshot.pipeline_line);
    assert_eq!(
        snapshot.primary_beam_voice_partition_changed_voice_elements,
        prepared.changed_voice_elements()
    );
    assert!(snapshot.pipeline_line.contains(&format!(
        "RestVerticalPlacementCandidates {newly_required};"
    )));
    let continued = snapshot.committed_occurrence_continuation.unwrap();
    crate::structural_write_authority::bind_final_pdf_source_occurrence_continuation_to_exact_persisted_readback(
        &continued, &candidate, &canonical, snapshot.committed_voice_plan.as_ref().unwrap(),
        Some(&std::collections::BTreeSet::new()),
    ).unwrap();
    let (again, audit, trial) = stage_final_rest_vertical_placement_candidate(&candidate, true);
    trial.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    assert_eq!(again, candidate);
    assert_eq!(audit.candidates, 0);
}

#[test]
fn final_sink_keeps_clef_and_foreign_rest_stable_after_beam_voice_alignment() {
    use crate::master_state_identity::{MasterLineage, RenderMasterEpoch};
    use crate::structural_write_authority::{
        MasterAdoptedState, RenderSinkKind, RenderSovereignState,
    };
    let scope = 57usize;
    let note = |step: &str, octave: usize, duration: usize, glyph: &str| {
        format!(
            "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>3</voice><type>{glyph}</type><staff>2</staff></note>"
        )
    };
    let first = note("C", 3, 24, "16th");
    let mut tail = String::new();
    for index in 1..16 {
        tail.push_str(&if index == 2 {
            note("C", 3, 24, "16th")
        } else {
            note("B", 5, 24, "16th")
        });
    }
    let input = format!(
        "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{scope}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{}{}</measure><measure number=\"{}\">{first}<backup><duration>24</duration></backup><note><rest/><duration>384</duration><voice>9</voice><type>whole</type><staff>2</staff></note><backup><duration>384</duration></backup><forward><duration>24</duration><voice>3</voice><staff>2</staff></forward>{tail}</measure></part></score-partwise>",
        note("C", 3, 192, "half"),
        note("C", 3, 192, "half"),
        scope + 1,
    );
    let (canonical, voices, source, prepared) = crate::structural_write_authority::
        CanonicalFinalChordStructuralPlan::original_beam_alignment_test_fixture(&input);
    assert!(prepared.changed_voice_elements() > 0);
    // Use real synthetic genesis and the actual FinalNotation sink owner, not
    // candidate bytes presented to the readback as though the sink were a no-op.
    let master = MasterAdoptedState::adopt_genesis_state(
        source.xml().to_owned(),
        MasterLineage::lineage_from_genesis(
            crate::master_state_identity::tests::synthetic_genesis_for_tests(source.xml()),
        ),
    );
    let directory =
        std::env::temp_dir().join(format!("etuschlopf-clef-rest-sink-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let path = directory.join("synthetic-clef-rest.render.musicxml");
    let mut state = RenderSovereignState::begin_project_render(&path, &master);
    let root = state.initial_trial_root(true);
    state
        .adopt_transition(
            root,
            source.xml(),
            RenderTransitionReceipts::new(),
            RenderSinkKind::PrunedPdfTrial,
            |artifact, _, candidate| (std::fs::write(artifact, candidate), None),
        )
        .unwrap();
    let temporal = crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition(
        "MasterXML",
        master.document(),
    )
    .unwrap();
    state.bind_late_master_epoch(RenderMasterEpoch::after_cursor_lane_before_final_chord(
        master.lineage(),
        master.document(),
        &temporal,
        master.document(),
    ));
    let mut preparation = super::materialize_silent_voice_visible_measure_coverage_for_pdf_source(
        &path,
        true,
        &canonical,
        &voices,
        &std::collections::BTreeSet::new(),
        &source,
        Some(&prepared),
    );
    let expected_sink_input = preparation
        .candidate_xml
        .take()
        .expect("prepared final source");
    preparation
        .receipts
        .discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    assert!(
        final_rest_structural_targets(&expected_sink_input)
            .unwrap()
            .is_empty()
    );
    let commit =
        super::super::final_pdf_source_commit::finalize_pdf_source_before_renderer_authorized(
            &mut state,
            &path,
            super::super::final_pdf_source_commit::FinalPdfSourceDisposition::Live,
            super::super::final_pdf_source_commit::FinalPdfSourcePlans {
                chord: &canonical,
                voice: &voices,
            },
            &std::collections::BTreeSet::new(),
            &source,
            Some(&prepared),
        )
        .unwrap();
    assert!(commit.closed(), "{}", commit.pipeline_line());
    let persisted = std::fs::read_to_string(&path).unwrap();
    assert_eq!(
        persisted, expected_sink_input,
        "sink confirms the exact prepared clef/rest fixpoint"
    );
    assert_eq!(commit.committed_xml(), Some(persisted.as_str()));
    assert!(
        final_rest_structural_targets(&persisted)
            .unwrap()
            .is_empty()
    );
    let mut reprojection = persisted.clone();
    super::super::final_pdf_source_commit::primary_clef_probe(&mut reprojection).unwrap();
    assert_eq!(
        reprojection, persisted,
        "actual sink is a final clef fixpoint"
    );
    let _ = std::fs::remove_file(path);
}
