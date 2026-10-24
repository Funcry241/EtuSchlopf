fn legacy_cursor_batch_oracle(source: &str) -> String {
    let mut current = source.to_string();
    for expected_remaining in [2_usize, 1] {
        let (views, _) = adapt_structural_snapshot_read_only(
            &current, 1, "multiplicity-local-rebind");
        let intents = detect_structural_repair_intents_read_only(&views);
        assert_eq!(intents.len(), expected_remaining);
        let local = vec![intents[0].clone()];
        let detector = audit_identity_free_detector_module_boundary(
            views.len(), 1, "multiplicity-local-rebind");
        let semantic = audit_identity_free_semantic_event_equivalence(
            &local, views.len(), "multiplicity-local-rebind");
        let metamorphic = audit_identity_free_metamorphic_safety_net(
            &current, 1, "multiplicity-local-rebind");
        let (_verifier, token) = authorization_verifier::authorize_repair_candidate_for_writer(
            &local, &detector, &semantic, &metamorphic, "legacy-oracle");
        let (post, audit, _plan, receipt) =
            writer_entry_boundary::authorized_reference_writer_entry(
                token.expect("actual legacy owner authorization"),
                &current, "legacy-oracle");
        assert!(audit.write_produced());
        assert!(receipt.is_some());
        let mut probe = RenderTransitionReceipts::new();
        probe.push_exact(receipt);
        probe.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        current = post;
    }
    let (views, _) = adapt_structural_snapshot_read_only(
        &current, 1, "legacy-oracle-after");
    assert!(detect_structural_repair_intents_read_only(&views).is_empty());
    current
}
