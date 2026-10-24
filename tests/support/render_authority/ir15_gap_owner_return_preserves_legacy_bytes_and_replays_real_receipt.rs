// Integration test for a grandfathered production entry, kept in its private lexical scope.
    #[test]
    fn ir15_gap_owner_return_preserves_legacy_bytes_and_replays_real_receipt() {
        for base in [17_usize, 911_usize] {
            let a = gap_score_at(base);
            let (oracle, _, _, _) = apply_strict_staff_local_release_completeness_materialization(&a, 32);
            let (candidate, actions, _, _, receipts) = apply_strict_staff_local_release_completeness_materialization_with_receipts(&a, 32);
            assert_eq!(candidate, oracle);
            assert!(!actions.is_empty());
            assert!(!receipts.is_empty());
            assert_eq!(find_next_measure_bounds(&a, 0).expect("relocated numeric scope").2, base);
            let p = path("ir15-gap");
            let _ = std::fs::remove_file(&p);
            let m = master(&a);
            let mut state = RenderSovereignState::begin_project_render(&p, &m);
            let root = state.initial_trial_root(false);
            state.adopt_transition(root, &candidate, receipts, RenderSinkKind::StrictStaffReleaseTrial,
                |p, pre, post| (crate::structural_write_authority::write_strict_staff_release_trial_authorized(pre, post, p), None)
            ).unwrap();
            assert_eq!(state.document(), Some(candidate.as_str()));
            std::fs::remove_file(p).unwrap();
        }
    }
