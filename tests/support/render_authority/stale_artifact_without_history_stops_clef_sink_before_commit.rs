// Integration test for a grandfathered production entry, kept in its private lexical scope.
    #[test]
    fn stale_artifact_without_history_stops_clef_sink_before_commit() {
        let ordinal = 17_usize;
        let xml = format!("<score-partwise><part id=\"P1\">{}</part></score-partwise>", overlap_measure(&ordinal.to_string()));
        let path = temporary_render_path("stale-sovereign-clef");
        let mut state = sovereign_fixture(&xml, &path);
        std::fs::write(&path, "stale artifact").unwrap();
        let root = state.final_notation_branch_root(crate::structural_write_authority::RenderLateBranch::LowerClefSurface, false);
        let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            materialize_lower_clef_surface_decision(resolve(&xml), &xml, &path, Some((&mut state, root)))
        }));
        assert!(rejected.is_err());
        assert!(state.document().is_none());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "stale artifact");
        let _ = std::fs::remove_file(path);
    }
