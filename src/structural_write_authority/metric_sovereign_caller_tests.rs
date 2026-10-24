// Canonical SWA boundary tests; included in the original private helper scope.
#[cfg(test)]
mod metric_sovereign_caller_tests {
    use super::*;
    use crate::master_state_identity::{MasterLineage, RenderDiscardReason, RenderTransitionReceipts};
    use crate::master_state_identity::tests::synthetic_genesis_for_tests;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_METRIC_PATH: AtomicUsize = AtomicUsize::new(0);

    fn metric_xml(staffs: &[u8], ordinal: usize) -> String {
        let notes = staffs.iter().map(|staff| format!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>{staff}</staff></note>"
        )).collect::<String>();
        format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>{}</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{notes}</measure></part></score-partwise>", staffs.len()/4)
    }
    fn state_for(xml: &str) -> (structural_write_authority::RenderSovereignState, std::path::PathBuf) {
        let id = NEXT_METRIC_PATH.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("r1230-metric-{}-{id}.render.musicxml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let master = structural_write_authority::MasterAdoptedState::adopt_genesis_state(
            xml.to_owned(), MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(xml)),
        );
        let mut state = structural_write_authority::RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(true);
        state.adopt_transition(root, xml, RenderTransitionReceipts::new(),
            structural_write_authority::RenderSinkKind::PrunedPdfTrial,
            |path, _pre, post| (std::fs::write(path, post), None),
        ).unwrap();
        (state, path)
    }
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/render_authority/m4_unpitched_empty_scope_is_explicit_no_write.rs"));
    #[test]
    fn m5_actual_owner_bad_readback_keeps_prior_state() {
        let source = metric_xml(&[1, 2, 1, 2], 29);
        let (mut state, path) = state_for(&source);
        let root = state.require_state_root("test M5");
        let mut post = root.document().to_owned();
        let (_, receipt) = close_committed_cross_staff_beam_gestures(&mut post);
        assert!(receipt.is_some());
        let mut chain = RenderTransitionReceipts::new();
        chain.push_preserving(receipt, &post);
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = state.adopt_transition(root, &post, chain,
                structural_write_authority::RenderSinkKind::Checkpoint(structural_write_authority::RenderCheckpointSink::ExternalFeedbackClosure),
                |path, _pre, post| {
                    let (result, receipt) = structural_write_authority::write_external_feedback_closure_checkpoint_authorized(path, post);
                    if result.is_ok() { std::fs::write(path, "<foreign/>").unwrap(); }
                    (result, receipt)
                },
            );
        }));
        assert!(failed.is_err());
        assert_eq!(state.document(), Some(source.as_str()));
        std::fs::remove_file(path).unwrap();
    }
}
