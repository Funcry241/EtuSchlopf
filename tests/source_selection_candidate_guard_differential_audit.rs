use std::fs;
use std::path::Path;

#[test]
fn hand_rhythm_primary_repair_precedes_local_best_refinement() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let stage = fs::read_to_string(
        root.join("src/pipeline_shards/pipeline_orchestrator_arrangement_solver_stage.rs"),
    )
    .expect("arrangement solver stage");

    let guided = stage
        .find("fn run_arrangement_solver_guided_search_stage(")
        .expect("guided source-selection search stage");
    let primary = stage[guided..]
        .find("run_corpus_driven_hand_rhythm_primary_repair(")
        .map(|offset| guided + offset)
        .expect("hand/rhythm primary repair call");
    let refinement = stage[guided..]
        .find("LocalBestRefinement")
        .map(|offset| guided + offset)
        .expect("local best refinement stage");

    assert!(primary < refinement);
}

#[test]
fn source_selection_guard_transition_contract_preserves_issue_first_authority() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let solver = fs::read_to_string(
        root.join("src/pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs"),
    )
    .expect("candidate guard source");

    let key_start = solver
        .find("struct CandidateDecisionKey")
        .expect("current CandidateDecisionKey authority");
    let end = solver[key_start..]
        .find("fn count_summary(")
        .map(|offset| key_start + offset)
        .expect("non-additive candidate guard end");
    let decision = &solver[key_start..end];

    let blocker = decision.find("blockers:").expect("blocker key");
    let critical = decision
        .find("critical_count:")
        .expect("critical-count key");
    let hardness = decision
        .find("max_critical_hardness:")
        .expect("critical-hardness key");
    let warnings = decision.find("warnings:").expect("warning key");
    let hints = decision.find("hints:").expect("hint key");
    let retention = decision
        .find("retention_rank:")
        .expect("arrangement-retention key");
    let human_moves = decision
        .find("human_potential_notes_moved:")
        .expect("human-moves tie-break");

    assert!(blocker < critical);
    assert!(critical < hardness);
    assert!(hardness < warnings);
    assert!(warnings < hints);
    assert!(hints < retention);
    assert!(retention < human_moves);
    assert!(decision.contains("retained_note_percent"));
    assert!(!decision.contains("candidate.score"));
    assert!(!decision.contains("canonical_source_retention"));
    assert!(!decision.contains("canonical_source_retention_ppm"));

    let guard_start = decision
        .find("fn candidate_strictly_beats_guard(")
        .expect("candidate guard");
    let guard = &decision[guard_start..];
    assert!(guard.contains("candidate_decision_key(candidate) < candidate_decision_key(guard)"));
    assert!(!guard.contains("candidate.score"));
}

#[test]
fn fast_large_jump_critical_boundary_remains_twenty_four_semitones() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let solver = fs::read_to_string(
        root.join("src/pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs"),
    )
    .expect("pianistic criticality source");

    assert!(solver.contains(
        "\"PIANO_FAST_LARGE_JUMP\" => issue.actual_divisions.unwrap_or(0) >= 24"
    ));
}
