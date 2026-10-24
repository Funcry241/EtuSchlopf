// Integration test for a grandfathered production entry, kept in its private lexical scope.
    #[test]
    fn sovereign_lower_clef_commit_uses_real_owner_and_exact_readback() {
        let ordinal = 17_usize;
        let xml = format!("<score-partwise><part id=\"P1\">{}</part></score-partwise>", overlap_measure(&ordinal.to_string()));
        let path = temporary_render_path("sovereign-clef");
        let mut state = sovereign_fixture(&xml, &path);
        let root = state.final_notation_branch_root(crate::structural_write_authority::RenderLateBranch::LowerClefSurface, false);
        let audit = materialize_lower_clef_surface_decision(resolve(&xml), &xml, &path, Some((&mut state, root))).unwrap();
        assert!(audit.written);
        let actual = std::fs::read_to_string(&path).unwrap();
        assert_eq!(state.document(), Some(actual.as_str()));
        assert_ne!(actual, xml);
        assert!(state.has_history());
        let _ = std::fs::remove_file(path);
    }
