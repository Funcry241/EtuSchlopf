// Canonical SWA boundary tests; included in the original private helper scope.
#[cfg(test)]
mod r1230_ir17_ir18_prepared_chain_tests {
    use super::*;
    use crate::master_state_identity::{MasterLineage, RenderDiscardReason, RenderTransitionReceipts};
    use crate::master_state_identity::tests::synthetic_genesis_for_tests;
    use crate::structural_write_authority::{MasterAdoptedState, RenderSinkKind, RenderSovereignState};

    fn missing_measure_at(ordinal: usize) -> String {
        format!("<measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>288</duration></forward><backup><duration>384</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>384</duration><voice>2</voice><type>whole</type><staff>2</staff></note></measure>")
    }
    fn duplicate_measure_at(ordinal: usize) -> String {
        format!("<measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note><note><chord/><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note></measure>")
    }
    fn gap_score_at(ordinal: usize) -> String {
        format!("<score-partwise><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>288</duration></forward></measure></part></score-partwise>")
    }
    fn whole_note_at(ordinal: usize) -> String {
        format!("<score-partwise><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note></measure></part></score-partwise>")
    }

    fn rest(duration: i32) -> String {
        format!("<note><rest/><duration>{duration}</duration><voice> 1 </voice><type>16th</type><staff> 1 </staff></note>")
    }
    fn measure(number: usize) -> String {
        format!("<measure number=\"{number}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{}{}<note><pitch><step>C</step><octave>5</octave></pitch><duration>336</duration><voice>1</voice><type>half</type><staff>1</staff></note></measure>", rest(24), rest(24))
    }
    fn source() -> String {
        format!("<score-partwise><part id=\"P1\">{}{}</part></score-partwise>", measure(10), measure(20))
    }
    fn master(xml: &str) -> MasterAdoptedState {
        MasterAdoptedState::adopt_genesis_state(xml.to_owned(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(xml)))
    }
    fn path(case: &str) -> PathBuf {
        std::env::temp_dir().join(format!("r1230-ir1718-{case}-{}.musicxml", std::process::id()))
    }

    #[test]
    fn ir17_real_generated_fragments_and_single_typed_e022_commits() {
        let a = source();
        let (legacy_c, _, _, generated, consumer) = apply_pianistic_rest_consolidation_with_receipts(&a, 256);
        let mut oracle = RenderTransitionReceipts::new();
        for generated in generated {
            oracle.push_generated_block(Some(generated.receipt), &generated.pre_fragment,
                &generated.post_fragment, Some(generated.located));
        }
        oracle.push_exact(consumer);
        oracle.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        let (c, actions, _, chain) = apply_pianistic_rest_consolidation_prepared(&a, 256);
        assert_eq!(actions.len(), 2);
        assert_eq!(c, legacy_c);
        assert_eq!(chain.len(), 3, "two located Generated #17 and actual #34");
        let p = path("ir17");
        let _ = std::fs::remove_file(&p);
        let m = master(&a);
        let mut state = RenderSovereignState::begin_project_render(&p, &m);
        let root = state.initial_trial_root(false);
        state.adopt_transition(root, &c, chain, RenderSinkKind::StrictVisibleRestArithmeticTrial,
            |p, pre, post| (crate::structural_write_authority::write_strict_visible_rest_arithmetic_trial_authorized(pre, post, p), None)
        ).unwrap();
        assert_eq!(state.document(), Some(c.as_str()));
        assert_eq!(std::fs::read_to_string(&p).unwrap(), c);
        std::fs::remove_file(p).unwrap();
    }

    #[test]
    fn ir18_two_accepted_transactions_replay_in_order_and_typed_e025_commits() {
        let a = source();
        let (c, actions, _, chain) = apply_structural_transactional_rest_coalesce_render_write_prepared(&a, 4);
        assert_eq!(actions.iter().filter(|a| a.verdict == "adopted_transactionally").count(), 2);
        assert_eq!(chain.len(), 4, "two actual Generated #17→#37 transactions");
        let p = path("ir18");
        let _ = std::fs::remove_file(&p);
        let m = master(&a);
        let mut state = RenderSovereignState::begin_project_render(&p, &m);
        let root = state.initial_trial_root(false);
        state.adopt_transition(root, &c, chain, RenderSinkKind::ExternalNotationFeedbackAudit,
            |p, _, post| (crate::structural_write_authority::write_external_notation_feedback_audit_candidate_authorized(p, post), None)
        ).unwrap();
        assert_eq!(state.document(), Some(c.as_str()));
        assert_eq!(std::fs::read_to_string(&p).unwrap(), c);
        std::fs::remove_file(p).unwrap();
    }

    #[test]
    fn ir17_full_builder_transports_productive_max64_between_real_rounds() {
        for base in [17_usize, 911_usize] {
            let missing_measure = missing_measure_at(base + 513);
            // Existing max_actions=256 leaves one real independent consolidation
            // for the second #34 batch; #69 runs between the batches unchanged.
            let measures = (base..base + 257).map(measure).collect::<String>();
            let a = format!("<score-partwise><part id=\"P1\">{measures}{missing_measure}</part></score-partwise>");
            assert_eq!(find_next_measure_bounds(&a, 0).expect("relocated numeric scope").2, base);
            let p = path("full-ir17");
            let _ = std::fs::remove_file(&p);
            let (trial, candidate, chain) = build_strict_visible_rest_arithmetic_materializer_trial_prepared(&a, "test", &p).unwrap();
            assert!(trial.written);
            assert!(trial.missing_inserted > 0);
            assert!(!p.exists(), "prepare is pure");
            assert_eq!(chain.len(), 260, "256 Generated #17 + #34, productive #69, then one Generated #17 + #34");
            assert_eq!(trial.consolidation_attempted, 257);
            let c = candidate.unwrap();
            assert!(!c.contains("<forward><duration>288</duration></forward>"));
            let m = master(&a);
            let mut state = RenderSovereignState::begin_project_render(&p, &m);
            let root = state.initial_trial_root(false);
            state.adopt_transition(root, &c, chain, RenderSinkKind::StrictVisibleRestArithmeticTrial,
                |p, pre, post| (crate::structural_write_authority::write_strict_visible_rest_arithmetic_trial_authorized(pre, post, p), None)
            ).unwrap();
            assert_eq!(state.document(), Some(c.as_str()));
            std::fs::remove_file(p).unwrap();
        }
    }

    #[test]
    fn ir18_full_builder_commits_42_before_two_fragment_consumer_transactions() {
        for base in [17_usize, 911_usize] {
            let duplicate_measure = duplicate_measure_at(base + 513);
            let a = format!("<score-partwise><part id=\"P1\">{duplicate_measure}{}{}</part></score-partwise>", measure(base), measure(base + 1));
            assert_eq!(find_next_measure_bounds(&a, 0).expect("relocated numeric scope").2, base + 513);
            let p = path("full-ir18");
            let _ = std::fs::remove_file(&p);
            let (legacy_output, audit, candidate, chain) = build_external_notation_feedback_safe_write_audit_prepared(&a, "test", &p).unwrap();
            assert!(audit.written);
            assert_eq!(audit.duplicate_adopted, 1);
            assert_eq!(audit.transactional_rest_coalesce_adopted, 2);
            assert_eq!(chain.len(), 5, "#42 then two Generated #17→#37 pairs");
            assert!(!p.exists(), "prepare is pure");
            let c = candidate.unwrap();
            assert_eq!(legacy_output, c);
            let m = master(&a);
            let mut state = RenderSovereignState::begin_project_render(&p, &m);
            let root = state.initial_trial_root(false);
            state.adopt_transition(root, &c, chain, RenderSinkKind::ExternalNotationFeedbackAudit,
                |p, _, post| (crate::structural_write_authority::write_external_notation_feedback_audit_candidate_authorized(p, post), None)
            ).unwrap();
            assert_eq!(state.document(), Some(c.as_str()));
            assert_eq!(std::fs::read_to_string(&p).unwrap(), c);
            std::fs::remove_file(p).unwrap();
        }
    }

    #[test]
    fn max0_missing_probes_are_disposed_outside_the_main_chain() {
        let a = source();
        let mut examples = Vec::new();
        let (post, actions, _, _, receipt) = apply_pianistic_missing_rest_insert_with_receipt(&a, 0, &mut examples);
        let mut probe = RenderTransitionReceipts::new();
        probe.push_exact(receipt);
        assert_eq!(post, a);
        assert!(actions.is_empty());
        assert!(probe.is_empty());
        probe.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn pure_ir17_builder_does_not_touch_a_real_existing_sink() {
        let a = source();
        let p = path("pure");
        std::fs::write(&p, "sentinel").unwrap();
        let (_, _, chain) = build_strict_visible_rest_arithmetic_materializer_trial_prepared(&a, "test", &p).unwrap();
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "sentinel");
        chain.discard_unadopted(RenderDiscardReason::LosingCandidate);
        std::fs::remove_file(p).unwrap();
    }

    #[test]
    fn ir17_write_failure_does_not_adopt_prepared_events() {
        let a = source();
        let (c, _, _, chain) = apply_pianistic_rest_consolidation_prepared(&a, 256);
        let p = path("writefail");
        let _ = std::fs::remove_file(&p);
        let m = master(&a);
        let mut state = RenderSovereignState::begin_project_render(&p, &m);
        let root = state.initial_trial_root(false);
        let result = state.adopt_transition(root, &c, chain, RenderSinkKind::StrictVisibleRestArithmeticTrial,
            |_, _, _| (Err::<(), _>(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "injected sink error")), None)
        );
        assert!(result.is_err());
        assert_eq!(state.document(), None);
        assert!(!p.exists());
    }

    #[test]
    fn ir18_wrong_final_readback_never_becomes_state() {
        let a = source();
        let (c, _, _, chain) = apply_structural_transactional_rest_coalesce_render_write_prepared(&a, 4);
        let p = path("readbackfail");
        let _ = std::fs::remove_file(&p);
        let m = master(&a);
        let mut state = RenderSovereignState::begin_project_render(&p, &m);
        let root = state.initial_trial_root(false);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            state.adopt_transition(root, &c, chain, RenderSinkKind::ExternalNotationFeedbackAudit,
                |p, _, _| (std::fs::write(p, &a), None)
            )
        }));
        assert!(result.is_err(), "wrong readback triggers sovereignty_violation panic");
        assert_eq!(state.document(), None);
        assert_eq!(std::fs::read_to_string(&p).unwrap(), a);
        std::fs::remove_file(p).unwrap();
    }
    #[test]
    fn pure_late_initial_builders_reject_invalid_input_without_io_or_receipts() {
        let p = path("invalid");
        std::fs::write(&p, "unchanged sink").unwrap();
        let (trial, candidate, receipts) = build_lower_staff_secondary_reanchored_lane_rest_visibility_suppression_trial_prepared(
            "not XML", "test", &p,
        ).unwrap();
        assert!(!trial.written);
        assert!(candidate.is_none());
        assert!(receipts.is_empty());
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        let (trial, candidate, receipts) = build_strict_staff_local_release_completeness_with_transient_wav_gate_trial_prepared(
            "not XML", "test", &p,
        ).unwrap();
        assert!(!trial.written);
        assert!(candidate.is_none());
        assert!(receipts.is_empty());
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        let (trial, candidate, receipts) = build_strict_visible_rest_arithmetic_materializer_trial_prepared(
            "not XML", "test", &p,
        ).unwrap();
        assert!(!trial.written);
        assert!(candidate.is_none());
        assert!(receipts.is_empty());
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        assert_eq!(std::fs::read_to_string(&p).unwrap(), "unchanged sink");
        std::fs::remove_file(p).unwrap();
    }

    #[test]
    fn valid_whole_note_noop_has_no_rest_receipts_or_sink_post() {
        for base in [17_usize, 911_usize] {
            let a = whole_note_at(base);
            assert_eq!(find_next_measure_bounds(&a, 0).expect("relocated numeric scope").2, base);
            let p = path("noop");
            let _ = std::fs::remove_file(&p);
            let (trial, candidate, receipts) = build_strict_visible_rest_arithmetic_materializer_trial_prepared(&a, "test", &p).unwrap();
            assert!(!trial.written);
            assert!(candidate.is_none());
            assert!(receipts.is_empty());
            receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
            let (post, audit, candidate, receipts) = build_external_notation_feedback_safe_write_audit_prepared(&a, "test", &p).unwrap();
            assert_eq!(post, a);
            assert!(!audit.written);
            assert!(candidate.is_none());
            assert!(receipts.is_empty());
            receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
            assert!(!p.exists());
        }
    }

    #[test]
    fn generated_consumer_chain_fails_before_sink_on_wrong_endbytes() {
        let a = source();
        let (_post, _, _, receipts) = apply_pianistic_rest_consolidation_prepared(&a, 256);
        let p = path("wrong-candidate");
        let _ = std::fs::remove_file(&p);
        let m = master(&a);
        let mut state = RenderSovereignState::begin_project_render(&p, &m);
        let root = state.initial_trial_root(false);
        let called = std::cell::Cell::new(false);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            state.adopt_transition(root, &a, receipts, RenderSinkKind::StrictVisibleRestArithmeticTrial,
                |p, _, post| { called.set(true); (std::fs::write(p, post), None) })
        }));
        assert!(result.is_err());
        assert!(!called.get());
        assert_eq!(state.document(), None);
        assert!(!p.exists());
    }

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/render_authority/ir15_gap_owner_return_preserves_legacy_bytes_and_replays_real_receipt.rs"));

}
