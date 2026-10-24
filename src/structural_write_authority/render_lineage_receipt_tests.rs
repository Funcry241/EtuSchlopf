use super::*;

fn exact(
    pre: &str,
    post: &str,
    start: usize,
    end: usize,
    replacement: &str,
) -> ExactTransformationReceipt {
    crate::master_state_identity::tests::synthetic_exact_receipt_for_tests(
        pre,
        post,
        start,
        end,
        replacement,
    )
}

#[test]
fn generated_fragment_is_consumed_by_the_following_bound_exact_replacement() {
    for located in [Some(0), None] {
        let mut receipts = RenderTransitionReceipts::new();
        receipts.push_generated_exact(
            Some(exact("A", "B", 0, 1, "B")),
            "A",
            "B",
            located,
        );
        receipts.push_exact(Some(exact("A|X", "B|X", 0, 1, "B")));

        let (steps, mut obligation) = receipts.take_steps();
        let (post, _) = replay_steps("A|X", &steps)
            .expect("the following EXACT replacement consumes the generated fragment");
        obligation.close_successfully();
        assert_eq!(post, "B|X");
    }
}

#[test]
fn generated_fragment_is_not_consumed_by_identical_bytes_elsewhere() {
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_generated_exact(
        Some(exact("A", "B", 0, 1, "B")),
        "A",
        "B",
        Some(0),
    );
    receipts.push_exact(Some(exact("A|B", "C|B", 0, 1, "C")));

    let (steps, mut obligation) = receipts.take_steps();
    let error = replay_steps("A|B", &steps)
        .expect_err("a pre-existing B elsewhere must not consume the generated A -> B");
    obligation.close_successfully();
    assert!(
        error.contains("bound replacement geometry"),
        "unexpected replay failure: {error}"
    );
}

#[test]
fn instruction_source_accepts_only_the_cursor_neutral_owner_in_direction_chain() {
    let (pre, cursor_post, cursor_receipt, direction_post, direction_receipt) =
        crate::structural_write_authority::executed_cursor_then_direction_events_for_render_lineage_test();

    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_instruction_source_exact(Some(cursor_receipt), &pre, &cursor_post);
    receipts.push_exact(Some(direction_receipt));

    let (steps, mut obligation) = receipts.take_steps();
    let (post, traces) = replay_steps(&pre, &steps)
        .expect("the genuine #2 receipt remains a separate instruction event before DirectionRelocation");
    obligation.close_successfully();
    assert_eq!(post, direction_post);
    assert_eq!(traces.len(), 2);
    assert_eq!(traces[0].placement, StepPlacement::InstructionSource);
    assert_eq!(traces[0].doc_pre, traces[0].doc_post);
    assert_eq!(traces[1].placement, StepPlacement::Document);
    assert_ne!(traces[1].doc_pre, traces[1].doc_post);
}

#[derive(Clone, Copy)]
struct EnclosingBlockProbe;
impl sealed::MintOwner for EnclosingBlockProbe {}
impl BlockMintOwner for EnclosingBlockProbe {
    const DESCRIPTOR: &'static TransformationDescriptor =
        <crate::pipeline::BeatScopedBeamingBlockMint as BlockMintOwner>::DESCRIPTOR;
}

fn block(pre: &str, post: &str) -> BlockTransformationReceipt {
    BlockTransformationReceipt::mint_block_receipt(EnclosingBlockProbe, pre, post, None)
        .expect("synthetic enclosing BLOCK receipt")
}

#[test]
fn enclosing_accepts_generated_step_only_after_its_consumer_is_inside_the_enclosure() {
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(exact("A", "B", 0, 1, "B")));
    receipts.push_generated_exact(Some(exact("B", "C", 0, 1, "C")), "B", "C", Some(0));
    receipts.push_exact(Some(exact("B", "C", 0, 1, "C")));
    receipts.push_enclosing_block(Some(block("A", "C")), 0);

    let (steps, mut obligation) = receipts.take_steps();
    let (post, _) = replay_steps("A", &steps)
        .expect("the generated owner and its consumer are both inside the enclosure");
    obligation.close_successfully();
    assert_eq!(post, "C");
}

#[test]
fn enclosing_rejects_generated_step_whose_consumer_is_outside_the_enclosure() {
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(exact("A", "B", 0, 1, "B")));
    receipts.push_generated_exact(Some(exact("B", "C", 0, 1, "C")), "B", "C", Some(0));
    receipts.push_enclosing_block(Some(block("A", "B")), 0);
    receipts.push_exact(Some(exact("B", "C", 0, 1, "C")));

    let (steps, mut obligation) = receipts.take_steps();
    let error = replay_steps("A", &steps)
        .expect_err("an enclosing owner must not close over a still-pending generated step");
    obligation.close_successfully();
    assert!(
        error.contains("before its consuming owner"),
        "unexpected replay failure: {error}"
    );
}

fn record_complete_render_stage_sequence() {
    for boundary in RenderStageBoundary::ORDER {
        record_render_stage_boundary(boundary);
    }
}

#[test]
fn explicitly_discarded_receipt_chain_does_not_block_closure() {
    reset_render_lineage_measurement();
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(exact("A", "B", 0, 1, "B")));
    receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
    record_complete_render_stage_sequence();

    let verdict = record_final_render_lineage(None, true);
    assert!(verdict.closed, "explicit discard closes the live receipt obligation: {}", verdict.detail);
    reset_render_lineage_measurement();
}

#[test]
fn live_receipt_chain_blocks_closure_before_drop() {
    reset_render_lineage_measurement();
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(exact("A", "B", 0, 1, "B")));
    record_complete_render_stage_sequence();

    let verdict = record_final_render_lineage(None, true);
    assert!(!verdict.closed, "a live receipt obligation must already keep closure open");
    assert_eq!(
        verdict
            .failures
            .iter()
            .find(|(metric, _)| *metric == RenderClosureMetric::LineagePending)
            .map(|(_, value)| *value),
        Some(1)
    );

    receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
    reset_render_lineage_measurement();
}
