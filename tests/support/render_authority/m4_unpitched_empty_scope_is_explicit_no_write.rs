// Integration test for a grandfathered production entry, kept in its private lexical scope.
    #[test]
    fn m4_unpitched_empty_scope_is_explicit_no_write() {
        let source = "<score-partwise><part id=\"P1\"/></score-partwise>";
        let (state, path) = state_for(source);
        let ((post, audit), receipt) = apply_key_signature_enharmonic_spelling_render_write(
            state.require_state_root("test M4").document(), "M4 test", &path,
        );
        assert!(!audit.written);
        assert_eq!(post, source);
        let mut chain = RenderTransitionReceipts::new();
        chain.push_block(receipt, &post);
        assert!(chain.is_empty());
        chain.discard_unadopted(RenderDiscardReason::LosingCandidate);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), source);
        std::fs::remove_file(path).unwrap();
    }
