use super::*;
use crate::master_state_identity::tests::{
    permitted_exact_receipt_for_tests, synthetic_exact_receipt_for_tests, synthetic_genesis_for_tests,
};
use crate::master_state_identity::{MasterLineage, RenderTransitionReceipts};
use std::cell::Cell;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

// Complete temporal documents keep the synthetic receipt transitions inside pitch.
const RENDER_ZERO: &str = "<score-partwise><part><measure><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration></note></measure></part></score-partwise>";
const RENDER_ONE: &str = "<score-partwise><part><measure><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration></note></measure></part></score-partwise>";
const RENDER_TWO: &str = "<score-partwise><part><measure><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration></note></measure></part></score-partwise>";
const RENDER_OTHER: &str = "<score-partwise><part><measure><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>F</step><octave>4</octave></pitch><duration>4</duration></note></measure></part></score-partwise>";

static NEXT_TEMP_RENDER: AtomicUsize = AtomicUsize::new(0);

fn test_master(document: &str) -> MasterAdoptedState {
    MasterAdoptedState::adopt_genesis_state(
        document.to_string(),
        MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(document)),
    )
}

#[test]
fn late_master_epoch_refuses_a_proof_checked_on_different_bytes() {
    crate::master_state_identity::reset_render_lineage_measurement();
    let a = "<score-partwise><part><measure><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><forward><duration>4</duration></forward></measure></part></score-partwise>";
    let b = format!("{a}<!-- distinct byte identity -->");
    let master = test_master(&b);
    let proof = crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition("MasterXML", a).unwrap();
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::master_state_identity::RenderMasterEpoch::after_cursor_lane_before_final_chord(
            master.lineage(), master.document(), &proof, master.document(),
        )
    }));
    assert!(refused.is_err(), "checking A cannot authorize binding B");
    crate::master_state_identity::reset_render_lineage_measurement();
}

fn temp_render_path(label: &str) -> PathBuf {
    let ordinal = NEXT_TEMP_RENDER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "etuschlopf-r1230-{label}-{}-{ordinal}.render.musicxml",
        std::process::id()
    ))
}

fn state_with_history_and_late_epoch(label: &str) -> (RenderSovereignState, MasterAdoptedState, PathBuf) {
    let document = RENDER_ZERO;
    let master = test_master(document);
    let path = temp_render_path(label);
    let _ = std::fs::remove_file(&path);
    let mut state = RenderSovereignState::begin_project_render(&path, &master);

    let root = state.initial_trial_root(true);
    state
        .adopt_transition(
            root,
            document,
            RenderTransitionReceipts::new(),
            RenderSinkKind::PrunedPdfTrial,
            |artifact_path, _pre, candidate| {
                (std::fs::write(artifact_path, candidate), None)
            },
        )
        .expect("synthetic Genesis Render adoption");

    state.bind_late_master_epoch(RenderMasterEpoch::after_cursor_lane_before_final_chord(
        master.lineage(),
        master.document(),
        &crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition("MasterXML", master.document()).expect("valid synthetic temporal precondition"),
        master.document(),
    ));

    (state, master, path)
}

#[test]
fn fresh_s4_roots_bind_the_current_render_lineage_position() {
    let (state, _master, path) = state_with_history_and_late_epoch("fresh-s4");

    for branch in [RenderLateBranch::CursorNeutralWriter] {
        let root = state.final_notation_branch_root(branch, false);
        assert!(root.has_render_history(), "S4 preserve existing Render history");
        state.verify_root_is_current(&root, "fresh S4 root");
    }

    let _ = std::fs::remove_file(path);
}

#[test]
fn s4_root_created_before_a_later_commit_is_rejected_before_sink_write() {
    let (mut state, master, path) = state_with_history_and_late_epoch("stale-s4");
    let stale_root = state.final_notation_branch_root(
        RenderLateBranch::CursorNeutralWriter,
        false,
    );

    let pre = state.document().expect("Render history exists").to_string();
    let post = RENDER_ONE;
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(&pre, post)));
    let advance_root = state.require_state_root("advance current Render state");
    state
        .adopt_transition(
            advance_root,
            post,
            receipts,
            RenderSinkKind::MissingRestInsertTrial,
            |artifact_path, _pre, candidate| {
                (std::fs::write(artifact_path, candidate), None)
            },
        )
        .expect("synthetic advancing Render commit");

    let sink_called = Cell::new(false);
    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Result<(), std::io::Error> = state.adopt_final_notation_checkpoint_with_write(
            stale_root,
            master.document(),
            RenderTransitionReceipts::new(),
            |_artifact_path, _candidate| {
                sink_called.set(true);
                Ok(())
            },
        );
    }));

    assert!(rejected.is_err(), "stale S3 root must be rejected");
    assert!(!sink_called.get(), "stale root reached the physical sink");

    // This test intentionally exercises the fatal violation path. Keep the
    // process-local measurement from leaking into later closure tests.
    crate::master_state_identity::reset_render_lineage_measurement();
    let _ = std::fs::remove_file(path);
}

#[test]
fn byte_round_trip_with_verified_receipts_commits_both_owner_events() {
    let (mut state, _master, path) = state_with_history_and_late_epoch("byte-round-trip-lineage");
    let pre = state.document().expect("Render history exists").to_string();
    let middle = RENDER_ONE;
    let before = state
        .adopted
        .as_ref()
        .expect("Render history exists")
        .lineage
        .committed_element_count();

    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(&pre, middle)));
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(middle, &pre)));

    let root = state.require_state_root("byte round-trip owner chain");
    state
        .adopt_transition(
            root,
            &pre,
            receipts,
            RenderSinkKind::MissingRestInsertTrial,
            |artifact_path, _pre, candidate| {
                (std::fs::write(artifact_path, candidate), None)
            },
        )
        .expect("byte round-trip Render adoption");

    assert_eq!(state.document(), Some(pre.as_str()));
    let after = state
        .adopted
        .as_ref()
        .expect("Render history remains")
        .lineage
        .committed_element_count();
    assert_eq!(
        after,
        before + 2,
        "both verified owner events must commit exactly once"
    );

    let _ = std::fs::remove_file(path);
}

#[test]
fn empty_receipt_chain_with_unchanged_bytes_does_not_add_lineage_event() {
    let (mut state, _master, path) = state_with_history_and_late_epoch("empty-no-op-lineage");
    let pre = state.document().expect("Render history exists").to_string();
    let before = state
        .adopted
        .as_ref()
        .expect("Render history exists")
        .lineage
        .committed_element_count();

    let root = state.require_state_root("empty unchanged Render transition");
    state
        .adopt_transition(
            root,
            &pre,
            RenderTransitionReceipts::new(),
            RenderSinkKind::MissingRestInsertTrial,
            |artifact_path, _pre, candidate| {
                (std::fs::write(artifact_path, candidate), None)
            },
        )
        .expect("unchanged empty Render adoption");

    assert_eq!(state.document(), Some(pre.as_str()));
    let after = state
        .adopted
        .as_ref()
        .expect("Render history remains")
        .lineage
        .committed_element_count();
    assert_eq!(
        after,
        before,
        "an empty unchanged transition must remain commit-free"
    );

    let _ = std::fs::remove_file(path);
}

#[test]
fn foreign_exact_instruction_source_is_rejected_before_sink_and_commit() {
    let (pre, cursor_post, _cursor_receipt, _direction_post, _direction_receipt) =
        crate::structural_write_authority::executed_cursor_then_direction_events_for_render_lineage_test();
    let master = test_master(&pre);
    let path = temp_render_path("foreign-instruction-source");
    let _ = std::fs::remove_file(&path);
    let mut state = RenderSovereignState::begin_project_render(&path, &master);

    let initial_root = state.initial_trial_root(true);
    state
        .adopt_transition(
            initial_root,
            &pre,
            RenderTransitionReceipts::new(),
            RenderSinkKind::PrunedPdfTrial,
            |artifact_path, _pre, candidate| {
                (std::fs::write(artifact_path, candidate), None)
            },
        )
        .expect("synthetic initial Render adoption");

    let before_events = state
        .adopted
        .as_ref()
        .expect("Render history exists")
        .lineage
        .committed_element_count();
    let foreign = synthetic_exact_receipt_for_tests(
        &pre,
        &cursor_post,
        0,
        pre.len(),
        &cursor_post,
    );
    assert!(!foreign.minted_by::<crate::structural_write_authority::CursorNeutralRenderExactMint>());
    assert_eq!(
        foreign.realize_post_document(&pre).as_deref(),
        Some(cursor_post.as_str())
    );

    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_instruction_source_exact(Some(foreign), &pre, &cursor_post);
    let root = state.require_state_root("foreign instruction source");
    let sink_called = Cell::new(false);
    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Result<(), std::io::Error> = state.adopt_transition(
            root,
            &pre,
            receipts,
            RenderSinkKind::MissingRestInsertTrial,
            |_artifact_path, _pre, _candidate| {
                sink_called.set(true);
                (Ok(()), None)
            },
        );
    }));

    assert!(rejected.is_err(), "foreign EXACT owner must be rejected");
    assert!(!sink_called.get(), "foreign InstructionSource reached the physical sink");
    assert_eq!(state.document(), Some(pre.as_str()));
    assert_eq!(
        state
            .adopted
            .as_ref()
            .expect("Render history remains")
            .lineage
            .committed_element_count(),
        before_events,
        "foreign InstructionSource must not mutate lineage"
    );

    crate::master_state_identity::reset_render_lineage_measurement();
    let _ = std::fs::remove_file(path);
}

#[test]
fn own_genesis_root_adopts_but_foreign_genesis_root_is_rejected_before_sink() {
    let document = RENDER_ZERO;
    let master = test_master(document);
    let own_path = temp_render_path("own-genesis-root");
    let foreign_path = temp_render_path("foreign-genesis-root");
    let _ = std::fs::remove_file(&own_path);
    let _ = std::fs::remove_file(&foreign_path);

    let mut own = RenderSovereignState::begin_project_render(&own_path, &master);
    let own_root = own.initial_trial_root(true);
    own.adopt_transition(
        own_root,
        document,
        RenderTransitionReceipts::new(),
        RenderSinkKind::PrunedPdfTrial,
        |artifact_path, _pre, candidate| (std::fs::write(artifact_path, candidate), None),
    )
    .expect("own G1 root must adopt");

    let foreign_issuer = RenderSovereignState::begin_project_render(&foreign_path, &master);
    let foreign_root = foreign_issuer.initial_trial_root(true);
    let mut target = RenderSovereignState::begin_project_render(&foreign_path, &master);
    let sink_called = Cell::new(false);
    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Result<(), std::io::Error> = target.adopt_transition(
            foreign_root,
            document,
            RenderTransitionReceipts::new(),
            RenderSinkKind::PrunedPdfTrial,
            |_artifact_path, _pre, _candidate| {
                sink_called.set(true);
                (Ok::<(), std::io::Error>(()), None)
            },
        );
    }));

    assert!(rejected.is_err(), "Genesis root from another sovereign state must be rejected");
    assert!(!sink_called.get(), "foreign Genesis root reached the physical sink");
    assert!(!target.has_history(), "foreign Genesis root must not create target history");

    crate::master_state_identity::reset_render_lineage_measurement();
    let _ = std::fs::remove_file(own_path);
    let _ = std::fs::remove_file(foreign_path);
}

#[test]
fn path_bound_transition_verifies_receipts_before_productive_write() {
    let (mut state, _master, path) = state_with_history_and_late_epoch("path-bound-preverify-positive");
    let pre = state.document().expect("Render history exists").to_string();
    let post = RENDER_ONE.to_string();
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(&pre, &post)));
    let root = state.require_state_root("path-bound positive");
    let writes = Cell::new(0usize);

    let value = state
        .adopt_path_bound_transition(
            root,
            RenderSinkKind::PrintCleanupOneSafeVisual,
            move |_pre| RenderPathBoundOutcome {
                value: 7usize,
                planned_post: Some(post.clone()),
                receipts,
                failed: false,
            },
            |artifact_path, _pre, candidate| {
                writes.set(writes.get() + 1);
                std::fs::write(artifact_path, candidate)
            },
        )
        .expect("verified path-bound transition");

    assert_eq!(value, 7);
    assert_eq!(writes.get(), 1, "verified path-bound sink writes exactly once");
    assert_eq!(state.document(), Some(RENDER_ONE));
    let _ = std::fs::remove_file(path);
}

#[test]
fn path_bound_receipt_mismatch_is_rejected_before_productive_write() {
    let (mut state, _master, path) = state_with_history_and_late_epoch("path-bound-preverify-negative");
    let pre = state.document().expect("Render history exists").to_string();
    let planned = RENDER_ONE.to_string();
    let receipt_post = RENDER_TWO;
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(&pre, receipt_post)));
    let root = state.require_state_root("path-bound mismatched receipt");
    let sink_called = Cell::new(false);
    let before = state.document().expect("Render history exists").to_string();

    let rejected = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Result<usize, std::io::Error> = state.adopt_path_bound_transition(
            root,
            RenderSinkKind::PrintCleanupOneSafeVisual,
            move |_pre| RenderPathBoundOutcome {
                value: 9usize,
                planned_post: Some(planned.clone()),
                receipts,
                failed: false,
            },
            |_artifact_path, _pre, _candidate| {
                sink_called.set(true);
                Ok(())
            },
        );
    }));

    assert!(rejected.is_err(), "receipt mismatch must fail closed");
    assert!(!sink_called.get(), "unverified path-bound bytes reached the productive sink");
    assert_eq!(state.document(), Some(before.as_str()));
    assert_eq!(std::fs::read_to_string(&path).ok().as_deref(), Some(before.as_str()));

    crate::master_state_identity::reset_render_lineage_measurement();
    let _ = std::fs::remove_file(path);
}

fn closure_failure_count(state: &RenderSovereignState, metric: RenderClosureMetric) -> usize {
    let verdict = record_final_render_lineage(
        state
            .adopted
            .as_ref()
            .map(|adopted| (&adopted.lineage, adopted.document.as_str())),
        true,
    );
    verdict
        .failures
        .iter()
        .find(|(candidate, _)| *candidate == metric)
        .map_or(0, |(_, value)| *value)
}

#[test]
fn normal_write_error_blocks_later_render_closure() {
    let (mut state, _master, path) = state_with_history_and_late_epoch("normal-write-error-closure");
    crate::master_state_identity::reset_render_lineage_measurement();
    let pre = state.document().expect("Render history exists").to_string();
    let post = RENDER_ONE;
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(&pre, post)));
    let root = state.require_state_root("normal failing write");

    let result: Result<(), std::io::Error> = state.adopt_transition(
        root,
        post,
        receipts,
        RenderSinkKind::MissingRestInsertTrial,
        |_artifact_path, _pre, _candidate| {
            (Err(std::io::Error::other("synthetic sink failure")), None)
        },
    );

    assert!(result.is_err());
    assert_eq!(state.document(), Some(pre.as_str()));
    assert!(
        closure_failure_count(&state, RenderClosureMetric::CandidateCoverageFailures) > 0,
        "handled write failure must remain visible to closure"
    );
    crate::master_state_identity::reset_render_lineage_measurement();
    let _ = std::fs::remove_file(path);
}

#[test]
fn failed_path_bound_preparation_blocks_closure_without_writing() {
    let (mut state, _master, path) = state_with_history_and_late_epoch("path-bound-failed-closure");
    crate::master_state_identity::reset_render_lineage_measurement();
    let pre = state.document().expect("Render history exists").to_string();
    let root = state.require_state_root("failed path-bound preparation");
    let sink_called = Cell::new(false);

    let result: Result<usize, std::io::Error> = state.adopt_path_bound_transition(
        root,
        RenderSinkKind::PrintCleanupOneSafeVisual,
        |_pre| RenderPathBoundOutcome {
            value: 11usize,
            planned_post: None,
            receipts: RenderTransitionReceipts::new(),
            failed: true,
        },
        |_artifact_path, _pre, _candidate| {
            sink_called.set(true);
            Ok(())
        },
    );

    assert_eq!(result.expect("legacy result is returned"), 11);
    assert!(!sink_called.get(), "failed preparation must not write");
    assert_eq!(state.document(), Some(pre.as_str()));
    assert!(
        closure_failure_count(&state, RenderClosureMetric::CandidateCoverageFailures) > 0,
        "handled path-bound failure must remain visible to closure"
    );
    crate::master_state_identity::reset_render_lineage_measurement();
    let _ = std::fs::remove_file(path);
}

#[test]
fn verified_trial_publication_decline_keeps_state_unchanged_without_pending_chain() {
    let pre = RENDER_ZERO;
    let post = RENDER_ONE;
    let master = test_master(pre);
    let path = temp_render_path("trial-publication-decline");
    let _ = std::fs::remove_file(&path);
    let mut state = RenderSovereignState::begin_project_render(&path, &master);
    let root = state.initial_trial_root(false);
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_exact(Some(permitted_exact_receipt_for_tests(pre, post)));
    let publish_called = Cell::new(false);

    let published: bool = state
        .adopt_trial_publication_transition(
            root,
            post,
            receipts,
            RenderSinkKind::HiddenRestForwardCollapse,
            |_artifact_path, source, candidate| {
                publish_called.set(true);
                assert_eq!(source, pre);
                assert_eq!(candidate, post);
                Ok::<bool, std::io::Error>(false)
            },
        )
        .expect("a verified losing trial is not a write failure");

    assert!(publish_called.get());
    assert!(!published);
    assert!(!state.has_history());
    assert_eq!(state.document(), None);
    assert!(!path.exists());
}

#[test]
fn pdf_shadow_checkpoint_preserves_main_state_and_history() {
    for ordinal in [11usize, 97] {
        let document = format!("<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>1</divisions><time><beats>1</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>quarter</type></note></measure></part></score-partwise>");
        let before = document.as_str();
        let master = test_master(before);
        let path = temp_render_path("pdf-shadow-main");
        let shadow = temp_render_path("pdf-shadow-only");
        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(true);
        state.adopt_transition(root, before, RenderTransitionReceipts::new(), RenderSinkKind::PrunedPdfTrial,
            |path, _pre, post| (std::fs::write(path, post), None)).unwrap();
        let position = state.adopted.as_ref().unwrap().lineage.position();
        let checkpoint = state.last_checkpoint_58_basis().map(str::to_string);
        let start = before.find("Piano").unwrap();
        let mut candidate = before.to_string();
        candidate.replace_range(start..start + 5, "Pianx");
        let mut chain = RenderTransitionReceipts::new();
        chain.push_exact(Some(synthetic_exact_receipt_for_tests(before, &candidate, start, start + 5, "Pianx")));
        let root = state.require_state_root("PDF Shadow");
        let a = state.materialize_shadow_final_notation_checkpoint(root, &candidate, chain, &shadow).unwrap();
        assert_eq!(std::fs::read_to_string(&shadow).unwrap(), a);
        assert_eq!(state.document(), Some(before));
        assert_eq!(state.adopted.as_ref().unwrap().lineage.position(), position);
        assert_eq!(state.last_checkpoint_58_basis().map(str::to_string), checkpoint);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
        let _ = std::fs::remove_file(shadow);
        let _ = std::fs::remove_file(path);
    }
}

#[test]
fn pdf_shadow_write_failure_does_not_mutate_main() {
    let (state, _master, path) = state_with_history_and_late_epoch("pdf-shadow-failed");
    let missing_parent = temp_render_path("nonexistent-pdf-shadow-parent");
    let _ = std::fs::remove_dir_all(&missing_parent);
    let shadow = missing_parent.join("trial.render.musicxml");
    let position = state.adopted.as_ref().unwrap().lineage.position();
    let root = state.require_state_root("PDF Shadow failure");
    let result = state.materialize_shadow_final_notation_checkpoint(root, RENDER_ZERO, RenderTransitionReceipts::new(), &shadow);
    assert!(result.is_err());
    assert_eq!(state.document(), Some(RENDER_ZERO));
    assert_eq!(state.adopted.as_ref().unwrap().lineage.position(), position);
    assert!(state.last_checkpoint_58_basis().is_none());
    let _ = std::fs::remove_file(path);
}

#[test]
fn pdf_shadow_cannot_publish_to_main_path() {
    let (state, _master, path) = state_with_history_and_late_epoch("pdf-shadow-same-path");
    let position = state.adopted.as_ref().unwrap().lineage.position();
    let root = state.require_state_root("PDF Shadow same path");
    let result = state.materialize_shadow_final_notation_checkpoint(root, RENDER_ZERO, RenderTransitionReceipts::new(), &path);
    assert!(result.is_err());
    assert_eq!(state.document(), Some(RENDER_ZERO));
    assert_eq!(state.adopted.as_ref().unwrap().lineage.position(), position);
    let _ = std::fs::remove_file(path);
}

#[test]
fn pdf_shadow_wrong_readback_closes_branch_and_keeps_main() {
    for ordinal in [11usize, 97] {
        let document = format!("<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>1</divisions><time><beats>1</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>quarter</type></note></measure></part></score-partwise>");
        let before = document.as_str();
        let master = test_master(before);
        let path = temp_render_path("pdf-shadow-wrong-main");
        let shadow = temp_render_path("pdf-shadow-wrong-readback");
        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(true);
        state.adopt_transition(root, before, RenderTransitionReceipts::new(), RenderSinkKind::PrunedPdfTrial,
            |path, _pre, post| (std::fs::write(path, post), None)).unwrap();
        let position = state.adopted.as_ref().unwrap().lineage.position();
        let readback_called = Cell::new(false);
        let root = state.require_state_root("PDF Shadow wrong readback");
        let result = state.materialize_shadow_final_notation_checkpoint_with_readback(
            root, before, RenderTransitionReceipts::new(), &shadow,
            |_path| { readback_called.set(true); Ok("foreign B".to_string()) });
        assert!(readback_called.get(), "actual typed #58 must precede the wrong-readback witness");
        assert!(result.is_err());
        assert_eq!(state.document(), Some(before));
        assert_eq!(state.adopted.as_ref().unwrap().lineage.position(), position);
        assert!(state.last_checkpoint_58_basis().is_none());
        let _ = std::fs::remove_file(shadow);
        let _ = std::fs::remove_file(path);
    }
}

fn complete_render_stages_for_test() {
    use crate::master_state_identity::{record_render_stage_boundary, RenderStageBoundary as S};
    for stage in [S::InitialRender, S::RestArticulation, S::MetricBeam,
                  S::FinalNotation, S::PdfSourceSelection, S::Release] {
        record_render_stage_boundary(stage);
    }
}
fn current_exact_chain(state: &RenderSovereignState, next: &str) -> RenderTransitionReceipts {
    let pre = state.document().expect("history");
    let mut chain = RenderTransitionReceipts::new();
    chain.push_exact(Some(permitted_exact_receipt_for_tests(pre, next)));
    chain
}
fn closure_panics(state: &RenderSovereignState) -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| state.close_render_sovereignty())).is_err()
}
fn cleanup_closure_fixture(path: PathBuf) {
    let _ = std::fs::remove_file(path);
    crate::master_state_identity::reset_render_lineage_measurement();
}
fn closed_history_fixture(label: &str) -> (RenderSovereignState, MasterAdoptedState, PathBuf) {
    crate::master_state_identity::reset_render_lineage_measurement();
    state_with_history_and_late_epoch(label)
}
#[test]
fn normal_render_closes_after_all_stages_and_exact_artifact() {
    let (state, _master, path) = closed_history_fixture("normal-close");
    complete_render_stages_for_test();
    assert!(state.close_render_sovereignty().closed);
    cleanup_closure_fixture(path);
}
#[test]
fn no_pdf_still_closes_existing_render_history() {
    let (state, _master, path) = closed_history_fixture("no-pdf-close");
    complete_render_stages_for_test(); // logical PDF stage, zero PDF writes
    assert!(state.close_render_sovereignty().closed);
    cleanup_closure_fixture(path);
}
#[test]
fn no_render_no_history_absent_artifact_closes() {
    crate::master_state_identity::reset_render_lineage_measurement();
    let master = test_master(RENDER_ZERO);
    let path = temp_render_path("no-render-close");
    let _ = std::fs::remove_file(&path);
    let state = RenderSovereignState::begin_project_render(&path, &master);
    complete_render_stages_for_test();
    assert!(state.close_render_sovereignty().closed);
    cleanup_closure_fixture(path);
}
#[test]
fn no_history_stale_artifact_never_becomes_state() {
    crate::master_state_identity::reset_render_lineage_measurement();
    let master = test_master(RENDER_ZERO);
    let path = temp_render_path("no-history-stale");
    std::fs::write(&path, RENDER_OTHER).unwrap();
    let state = RenderSovereignState::begin_project_render(&path, &master);
    complete_render_stages_for_test();
    assert!(closure_panics(&state)); assert_eq!(state.document(), None);
    cleanup_closure_fixture(path);
}
#[test]
fn existing_state_wrong_artifact_fails_closed() {
    let (state, _master, path) = closed_history_fixture("wrong-final");
    std::fs::write(&path, RENDER_OTHER).unwrap();
    complete_render_stages_for_test(); assert!(closure_panics(&state));
    assert_eq!(state.document(), Some(RENDER_ZERO));
    cleanup_closure_fixture(path);
}
#[test]
fn existing_state_missing_artifact_fails_closed() {
    let (state, _master, path) = closed_history_fixture("missing-final");
    std::fs::remove_file(&path).unwrap();
    complete_render_stages_for_test(); assert!(closure_panics(&state));
    cleanup_closure_fixture(path);
}
#[test]
fn handled_write_error_is_sticky_at_final_closure() {
    let (mut state, _master, path) = closed_history_fixture("handled-write");
    let root = state.require_state_root("attempt");
    let chain = current_exact_chain(&state, RENDER_ONE);
    let result = state.adopt_transition(root, RENDER_ONE, chain,
        RenderSinkKind::MissingRestInsertTrial,
        |_,_,_| (Err(std::io::Error::other("injected sink failure")), None));
    assert!(result.is_err());
    complete_render_stages_for_test(); assert!(closure_panics(&state));
    assert_eq!(state.document(), Some(RENDER_ZERO));
    cleanup_closure_fixture(path);
}
#[test]
fn handled_wrong_readback_stays_sticky_even_if_artifact_is_restored() {
    let (mut state, _master, path) = closed_history_fixture("handled-readback");
    let root = state.require_state_root("attempt");
    let chain = current_exact_chain(&state, RENDER_ONE);
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.adopt_transition(root, RENDER_ONE, chain,
            RenderSinkKind::MissingRestInsertTrial,
            |path,_,_| (std::fs::write(path, RENDER_OTHER), None))
    }));
    assert!(caught.is_err());
    std::fs::write(&path, RENDER_ZERO).unwrap();
    complete_render_stages_for_test(); assert!(closure_panics(&state));
    cleanup_closure_fixture(path);
}
#[test]
fn live_pending_chain_blocks_closure_before_drop() {
    let (state, _master, path) = closed_history_fixture("live-pending");
    let chain = current_exact_chain(&state, RENDER_ONE);
    complete_render_stages_for_test(); assert!(closure_panics(&state));
    chain.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
    cleanup_closure_fixture(path);
}
#[test]
fn losing_trial_discard_allows_closure_without_state_mutation() {
    let (state, _master, path) = closed_history_fixture("losing-discard");
    current_exact_chain(&state, RENDER_ONE).discard_unadopted(
        crate::master_state_identity::RenderDiscardReason::LosingCandidate);
    complete_render_stages_for_test(); assert!(state.close_render_sovereignty().closed);
    assert_eq!(state.document(), Some(RENDER_ZERO));
    cleanup_closure_fixture(path);
}
#[test]
fn successful_shadow_receipts_discard_without_main_events() {
    let (state, _master, path) = closed_history_fixture("shadow-discard");
    let before = state.adopted.as_ref().unwrap().lineage.committed_element_count();
    let shadow = current_exact_chain(&state, RENDER_ONE);
    shadow.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    complete_render_stages_for_test(); assert!(state.close_render_sovereignty().closed);
    assert_eq!(state.adopted.as_ref().unwrap().lineage.committed_element_count(), before);
    cleanup_closure_fixture(path);
}
#[test]
fn legitimate_post_checkpoint_event_may_advance_final_state() {
    let (mut state, _master, path) = closed_history_fixture("after-checkpoint");
    let root = state.require_state_root("checkpoint sink-only");
    state.adopt_final_notation_checkpoint(root, RENDER_ZERO, RenderTransitionReceipts::new()).unwrap();
    let root = state.require_state_root("later legitimate owner");
    let chain = current_exact_chain(&state, RENDER_ONE);
    // Core sequence test uses synthetic EXACT; actual E007 PR test below required.
    state.adopt_transition(root, RENDER_ONE, chain,
        RenderSinkKind::FinalLayoutPhraseProjection,
        |path,_,post| (std::fs::write(path, post), None)).unwrap();
    assert_eq!(state.last_checkpoint_58_recorded_document(), Some(RENDER_ZERO));
    assert!(state.last_checkpoint_58_basis().is_none());
    assert!(state.historical_checkpoint_58_root().is_none());
    assert_eq!(state.document(), Some(RENDER_ONE));
    complete_render_stages_for_test(); assert!(state.close_render_sovereignty().closed);
    cleanup_closure_fixture(path);
}
#[test]
fn stale_b4_historical_root_never_reaches_physical_sink() {
    let (mut state, _master, path) = closed_history_fixture("stale-historical");
    let root = state.require_state_root("checkpoint");
    state.adopt_final_notation_checkpoint(root, RENDER_ZERO, RenderTransitionReceipts::new()).unwrap();
    let stale = state.historical_checkpoint_58_root().unwrap();
    let root = state.require_state_root("advance");
    let chain = current_exact_chain(&state, RENDER_ONE);
    state.adopt_transition(root, RENDER_ONE, chain, RenderSinkKind::MissingRestInsertTrial,
        |path,_,post| (std::fs::write(path, post), None)).unwrap();
    let called = Cell::new(false);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.adopt_transition(stale, RENDER_ZERO, RenderTransitionReceipts::new(),
            RenderSinkKind::Checkpoint(RenderCheckpointSink::DirectionRelocation),
            |_,_,_| { called.set(true); (Ok::<(),std::io::Error>(()), None) })
    }));
    assert!(result.is_err()); assert!(!called.get());
    cleanup_closure_fixture(path);
}

fn ssc_render_report() -> String {
    let report="TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION\n----------------------------------------\nStatus: present.\nSILENT-VOICE-COVERAGE-WRITE-OUTCOME\n--------------------------------\nStatus: present.\nRendererOutcome: observed_from_existing_writer_audit.\n";
    crate::final_report::DraftReport::new(report.to_owned()).compact(str::to_owned).finalize().unwrap().as_str().to_owned()
}
#[test]
fn ssc_render_owner_commit_is_observed_without_changing_adoption_or_bytes() {
    crate::master_state_identity::reset_master_lineage_measurement();
    let (mut state, _master, path)=state_with_history_and_late_epoch("ssc-render-commit");
    let pre=state.document().unwrap().to_owned();let post=RENDER_ONE;
    let mut chain=RenderTransitionReceipts::new();chain.push_exact(Some(permitted_exact_receipt_for_tests(&pre, post)));
    let root=state.require_state_root("SSC read-only observation");
    state.adopt_transition(root,post,chain,RenderSinkKind::MissingRestInsertTrial,|path,_,candidate|(std::fs::write(path,candidate),None)).unwrap();
    record_final_render_lineage(state.adopted.as_ref().map(|state| (&state.lineage, state.document.as_str())), true);
    let report=ssc_render_report();for line in ["MasterOwnerEvents: 0","RenderOwnerEvents: 1","ReleasedOwnerEvents: 1","ClassifiedOwnerEvents: 1","V4Permitted: 1","InvariantDenied: 0","MissingEvidence: 0","PermitEnforcement: active"] { assert!(report.contains(line),"{line}: {report}"); }
    assert_eq!(state.document(),Some(post));assert_eq!(std::fs::read_to_string(&path).unwrap(),post);let _=std::fs::remove_file(path);
}
#[test]
fn ssc_losing_render_trial_is_not_a_shadow_adopted_event() {
    crate::master_state_identity::reset_master_lineage_measurement();
    let (mut state,_master,path)=state_with_history_and_late_epoch("ssc-render-decline");let pre=state.document().unwrap().to_owned();let post=RENDER_ONE;
    let mut chain=RenderTransitionReceipts::new();chain.push_exact(Some(permitted_exact_receipt_for_tests(&pre, post)));let root=state.require_state_root("SSC declined trial");
    let published=state.adopt_trial_publication_transition(root,post,chain,RenderSinkKind::MissingRestInsertTrial,|_,_,_|Ok::<_,std::io::Error>(false)).unwrap();assert!(!published);
    assert!(!ssc_render_report().contains("SSC-SHADOW-PERMIT"));assert_eq!(state.document(),Some(pre.as_str()));let _=std::fs::remove_file(path);
}


#[cfg(test)]
mod checkpoint58_preparation_tests {
use super::*;

fn checkpoint_owner_document(label: &str) -> String {
    format!(r#"<score-partwise version="4.0"><part-list><score-part id="p"><part-name>{label}</part-name></score-part></part-list><part id="p"><measure><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number="1"><sign>G</sign><line>2</line></clef><clef number="2"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>2</duration><voice>2</voice><type>half</type><staff>2</staff></note><attributes><clef number="2"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>2</duration><voice>2</voice><type>half</type><staff>2</staff></note></measure></part></score-partwise>"#)
}

fn checkpoint_owner_state(document: &str, label: &str) -> (RenderSovereignState, PathBuf) {
    crate::master_state_identity::reset_render_lineage_measurement();
    let master = test_master(document);
    let path = temp_render_path(label);
    let mut state = RenderSovereignState::begin_project_render(&path, &master);
    let root = state.initial_trial_root(true);
    state.adopt_transition(root, document, RenderTransitionReceipts::new(),
        RenderSinkKind::PrunedPdfTrial,
        |path, _, xml| (std::fs::write(path, xml), None)).unwrap();
    (state, path)
}

#[test]
fn prepared_checkpoint_replays_real_owner_before_main_write_and_matches_legacy() {
    for label in ["alpha", "renamed independent"] {
        let before = checkpoint_owner_document(label);
        let (mut state, path) = checkpoint_owner_state(&before, "prepared-main");
        let legacy = temp_render_path("legacy-equivalence");
        let (result, receipt) = crate::structural_write_authority::orchestrator_artifact_sink_authority::legacy_final_notation_checkpoint_for_tests(&legacy, &before);
        result.unwrap();
        let legacy_post = std::fs::read_to_string(&legacy).unwrap();
        let mut old = RenderTransitionReceipts::new();old.push_exact(receipt);
        old.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert_ne!(legacy_post, before, "synthetic source must exercise real additional owner edits");
        let root = state.require_state_root("prepared main");
        let called = Cell::new(false);
        state.adopt_final_notation_checkpoint_with_write(root, &before, RenderTransitionReceipts::new(),
            |artifact, verified| {
                assert_eq!(std::fs::read_to_string(artifact).unwrap(), before);
                assert_eq!(verified, legacy_post);
                called.set(true);
                crate::structural_write_authority::orchestrator_artifact_sink_authority::write_prepared_final_notation_checkpoint(&COMMIT_BOUNDARY, artifact, verified)
            }).unwrap();
        assert!(called.get());
        assert_eq!(state.document(), Some(legacy_post.as_str()));
        assert_eq!(state.last_checkpoint_58_basis(), Some(legacy_post.as_str()));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), legacy_post);
        std::fs::remove_file(path).unwrap();std::fs::remove_file(legacy).unwrap();
    }
    crate::master_state_identity::reset_render_lineage_measurement();
}

#[test]
fn prepared_checkpoint_missing_or_duplicate_receipt_refuses_before_sink() {
    for duplicate in [false, true] {
        let before = checkpoint_owner_document("missing-chain");
        let (mut state, path) = checkpoint_owner_state(&before, "refused-main");
        let after = if duplicate { before.clone() } else { before.replace("missing-chain", "unproved edit") };
        let root = state.require_state_root("refused prepared owner");
        let position = state.adopted.as_ref().unwrap().lineage.position();
        let mut receipts = RenderTransitionReceipts::new();
        if duplicate {
            let (prepared, receipt) = crate::structural_write_authority::orchestrator_artifact_sink_authority::prepare_final_notation_checkpoint(&path, &before);
            assert!(prepared.is_ok());receipts.push_exact(receipt);
        }
        let called = Cell::new(false);
        let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            state.adopt_final_notation_checkpoint_with_write(root, &after, receipts,
                |_, _| { called.set(true); Ok(()) })
        }));
        assert!(refused.is_err());assert!(!called.get());
        assert_eq!(state.document(), Some(before.as_str()));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
        assert_eq!(state.adopted.as_ref().unwrap().lineage.position(), position);
        assert!(state.last_checkpoint_58_recorded_document().is_none());
        std::fs::remove_file(path).unwrap();
    }
    crate::master_state_identity::reset_render_lineage_measurement();
}

#[test]
fn prepared_checkpoint_shadow_real_owner_preserves_main() {
    let before = checkpoint_owner_document("shadow owner");
    let (state, path) = checkpoint_owner_state(&before, "prepared-shadow-main");
    let shadow = temp_render_path("prepared-shadow");
    let position = state.adopted.as_ref().unwrap().lineage.position();
    let root = state.require_state_root("shadow original owner");
    let post = state.materialize_shadow_final_notation_checkpoint(root, &before, RenderTransitionReceipts::new(), &shadow).unwrap();
    assert_ne!(post, before);
    assert_eq!(std::fs::read_to_string(&shadow).unwrap(), post);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
    assert_eq!(state.document(), Some(before.as_str()));
    assert_eq!(state.adopted.as_ref().unwrap().lineage.position(), position);
    assert!(state.last_checkpoint_58_recorded_document().is_none());
    std::fs::remove_file(path).unwrap();std::fs::remove_file(shadow).unwrap();
    crate::master_state_identity::reset_render_lineage_measurement();
}

#[test]
fn final_notation_cannot_use_unprepared_generic_callback() {
    let before = checkpoint_owner_document("closed callback");
    let (mut state, path) = checkpoint_owner_state(&before, "unprepared-callback");
    let root = state.require_state_root("unprepared");
    let called = Cell::new(false);
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.adopt_transition(root, &before, RenderTransitionReceipts::new(),
            RenderSinkKind::Checkpoint(RenderCheckpointSink::FinalNotation),
            |_, _, _| { called.set(true); (Ok::<(),std::io::Error>(()), None) })
    }));
    assert!(refused.is_err());assert!(!called.get());
    assert_eq!(state.document(), Some(before.as_str()));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
    std::fs::remove_file(path).unwrap();
    crate::master_state_identity::reset_render_lineage_measurement();
}


#[test]
fn prepared_checkpoint_wrong_readback_cannot_commit() {
    let before = checkpoint_owner_document("wrong readback");
    let (mut state, path) = checkpoint_owner_state(&before, "prepared-readback");
    let position = state.adopted.as_ref().unwrap().lineage.position();
    let root = state.require_state_root("prepared wrong readback");
    let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.adopt_final_notation_checkpoint_with_write(root, &before, RenderTransitionReceipts::new(),
            |artifact, _| std::fs::write(artifact, "foreign persisted bytes"))
    }));
    assert!(refused.is_err());
    assert_eq!(state.document(), Some(before.as_str()));
    assert_eq!(state.adopted.as_ref().unwrap().lineage.position(), position);
    assert!(state.last_checkpoint_58_recorded_document().is_none());
    std::fs::remove_file(path).unwrap();
    crate::master_state_identity::reset_render_lineage_measurement();
}

}
