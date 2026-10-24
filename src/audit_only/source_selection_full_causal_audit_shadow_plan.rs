//! Audit-only projection for SourceSelection causality and the immutable SelectionPlan authority.
//!
//! This module itself has no writer authority. It serializes the already-observed event decisions,
//! canonical source retention, hotspot evidence, solver-candidate lineage and the r635 productive
//! plan-materialization binding. The productive authority lives in the SourceSelection boundary,
//! not in this report projection.

use crate::midi_seed::MidiSeedStats;
use std::fmt::Write as _;

fn option_usize(value: Option<usize>) -> String {
    value.map_or_else(|| "none".to_string(), |value| value.to_string())
}

fn option_u8(value: Option<u8>) -> String {
    value.map_or_else(|| "none".to_string(), |value| value.to_string())
}

pub(crate) fn append_source_selection_full_causal_audit_shadow_plan_body(
    out: &mut String,
    stats: &MidiSeedStats,
) {
    let causal = &stats.source_selection_full_causal;
    let solver = &stats.source_selection_solver_causality;
    let eval = &stats.source_selection_candidate_evaluation;
    let hotspot = &stats.hotspot_plan_audit;

    writeln!(out, "SchemaVersion: 1").expect("String write cannot fail");
    writeln!(out, "Boundary: CanonicalSourceEvent_to_existing_SourceSelection_to_solver_winner")
        .expect("String write cannot fail");
    writeln!(out, "AuditOnly: ja").expect("String write cannot fail");
    writeln!(out, "SelectionPlanAuthority: productive_seed_materialization")
        .expect("String write cannot fail");
    writeln!(
        out,
        "SelectionPlanAuthorityBound: {}",
        if causal.productive_plan_authority { "ja" } else { "nein" }
    )
    .expect("String write cannot fail");
    writeln!(out, "LegacyMutableWorkingSetMaterializationAuthority: none")
        .expect("String write cannot fail");
    writeln!(out, "ReportWriterAuthority: none").expect("String write cannot fail");
    writeln!(out, "MusicMutation: nein").expect("String write cannot fail");
    writeln!(out, "SelectionRulesChanged: nein").expect("String write cannot fail");
    writeln!(out, "SelectionOrderChanged: nein").expect("String write cannot fail");
    writeln!(out, "RankingChanged: nein").expect("String write cannot fail");
    writeln!(out, "WeightingChanged: nein").expect("String write cannot fail");
    writeln!(out, "ThresholdsChanged: nein").expect("String write cannot fail");
    writeln!(out, "PersistentEventIdentity: source_ordinal").expect("String write cannot fail");
    writeln!(out, "MutableVecIndexAsIdentity: nein").expect("String write cannot fail");
    writeln!(out, "CanonicalSourceEvents: {}", causal.canonical_source_event_count)
        .expect("String write cannot fail");
    writeln!(out, "CanonicalSourceRetained: {}", causal.retained_event_count)
        .expect("String write cannot fail");
    writeln!(out, "CanonicalSourceRemoved: {}", causal.removed_event_count)
        .expect("String write cannot fail");
    writeln!(out, "CanonicalSourceRetentionPPM: {}", causal.canonical_source_retention_ppm)
        .expect("String write cannot fail");
    writeln!(
        out,
        "CanonicalSourceRetentionPercent: {:.3}",
        causal.canonical_source_retention_ppm as f64 / 10_000.0
    )
    .expect("String write cannot fail");
    match eval.arrangement_visible_retention_milli_percent {
        Some(value) => writeln!(
            out,
            "ArrangementVisibleRetentionPercent: {:.3}",
            value as f64 / 1_000.0
        )
        .expect("String write cannot fail"),
        None => writeln!(out, "ArrangementVisibleRetentionPercent: none")
            .expect("String write cannot fail"),
    }
    writeln!(out, "RetentionMetricsAreDistinct: ja").expect("String write cannot fail");
    writeln!(out, "InitialStaffAssignments: {}", causal.initial_staff_assignment_count)
        .expect("String write cannot fail");
    writeln!(out, "StaffMoves: {}", causal.staff_move_count).expect("String write cannot fail");
    writeln!(out, "RemovalDecisions: {}", causal.removal_decision_count)
        .expect("String write cannot fail");
    writeln!(
        out,
        "EventDecisionTraceFingerprintSHA256: {}",
        causal.event_decision_trace_fingerprint_sha256
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "ImmutableShadowPlanFingerprintSHA256: {}",
        causal.shadow_plan_fingerprint_sha256
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "ImmutableSelectionPlanFingerprintSHA256: {}",
        causal.shadow_plan_fingerprint_sha256
    )
    .expect("String write cannot fail");
    writeln!(out, "ShadowPlanEquivalentToProductiveSelection: ja")
        .expect("String write cannot fail");
    writeln!(
        out,
        "ProductivePlanMaterializationEquivalentToLegacyWorkingSet: {}",
        if causal.productive_plan_equivalence_verified { "ja" } else { "nein" }
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "ProductivePlanRetainedEvents: {}",
        causal.productive_plan_retained_event_count
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "ProductivePlanSeedFingerprintSHA256: {}",
        causal.productive_plan_seed_fingerprint_sha256
    )
    .expect("String write cannot fail");

    let serialization_identity = &causal.midi_seed_serialized_source_identity_plan;
    writeln!(
        out,
        "MidiSeedSerializedSourceIdentityPlanAuthority: {}",
        if serialization_identity.authority_bound {
            "bound"
        } else {
            "none"
        }
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "MidiSeedSerializedSourceIdentityPlanCoverage: retained={} mapped={} measures={} fingerprint=fnv64:{:016x}",
        serialization_identity.retained_event_count,
        serialization_identity.mapped_event_count,
        serialization_identity.measure_raw_note_counts.len(),
        serialization_identity.fingerprint_fnv64,
    )
    .expect("String write cannot fail");

    let disposition_plan = &causal.productive_post_legacy_disposition_plan;
    writeln!(
        out,
        "SourceDurationPostLegacySelectionDispositionPlanAuthority: {}",
        if disposition_plan.authority_bound {
            "bound"
        } else {
            "none"
        }
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "SourceDurationPostLegacySelectionDispositionPlanCoverage: retained={} visible_pitched={} hidden_pitched={} restified_pitched={} raw_note_coverage_exact={}",
        disposition_plan.retained_event_count,
        disposition_plan.visible_pitched_event_count,
        disposition_plan.hidden_pitched_event_count,
        disposition_plan.restified_pitched_event_count,
        if disposition_plan.raw_note_coverage_exact {
            "ja"
        } else {
            "nein"
        },
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "SourceDurationPostLegacySelectionStaffProjection: human_potential_moved={} projected_staff_changes={}",
        disposition_plan.human_potential_moved_event_count,
        disposition_plan.projected_staff_change_count,
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "SourceDurationPostLegacySelectionDispositionPlanFingerprintFNV64: fnv64:{:016x}",
        disposition_plan.fingerprint_fnv64,
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "SourceDurationPostLegacySelectionDispositionMusicMutation: nein"
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "SourceDurationPostLegacySelectionDispositionWriterAuthority: none"
    )
    .expect("String write cannot fail");
    writeln!(
        out,
        "SourceDurationPostLegacySelectionDispositionDecision: {}",
        if disposition_plan.authority_bound
            && disposition_plan.raw_note_coverage_exact
            && serialization_identity.authority_bound
            && serialization_identity.mapped_event_count
                == disposition_plan.retained_event_count
        {
            "productive_source_identity_survives_legacy_material_decision_into_candidate_material_lineage"
        } else {
            "precutover_blocked_disposition_identity_not_bound"
        }
    )
    .expect("String write cannot fail");

    writeln!(out, "HotspotPlanCriticalOnly: {}", if hotspot.critical_only { "ja" } else { "nein" })
        .expect("String write cannot fail");
    writeln!(out, "HotspotSourceIssues: {}", hotspot.source_issue_count)
        .expect("String write cannot fail");
    writeln!(out, "HotspotGroupsBeforeTruncation: {}", hotspot.group_count_before_truncation)
        .expect("String write cannot fail");
    writeln!(out, "HotspotRequestedLimit: {}", hotspot.requested_limit)
        .expect("String write cannot fail");
    writeln!(out, "HotspotTruncatedGroups: {}", hotspot.truncated_count)
        .expect("String write cannot fail");
    writeln!(out, "HotspotPlanFingerprintSHA256: {}", hotspot.fingerprint_sha256)
        .expect("String write cannot fail");
    for row in &hotspot.evidence {
        writeln!(
            out,
            "HotspotEvidence issue={} measure={} staff={} base={} critical_bonus={} hardness={} contribution={} included={}",
            row.issue_type,
            row.measure,
            option_u8(row.staff),
            row.base_weight,
            row.critical_bonus,
            row.hardness,
            row.contribution,
            if row.included_after_truncation { "ja" } else { "nein" },
        )
        .expect("String write cannot fail");
    }

    for row in causal.shadow_rows.iter() {
        writeln!(
            out,
            "SelectionPlan source_ordinal={} pos={} len={} midi={} retained={} final_staff={} reason={} decisions={}",
            row.source_ordinal,
            row.pos,
            row.len,
            row.midi,
            if row.retained { "ja" } else { "nein" },
            option_u8(row.final_staff),
            row.removal_reason.map_or("none", |reason| reason.as_str()),
            row.decision_count,
        )
        .expect("String write cannot fail");
    }

    for decision in causal.decisions.iter() {
        writeln!(
            out,
            "EventDecision source_ordinal={} pass={} iteration={} action={} before_staff={} after_staff={} witness_a={} witness_b={} observed_primary={} observed_secondary={} threshold_primary={} threshold_secondary={} focus_score={} tie_break={} reason={}",
            decision.source_ordinal,
            decision.pass.as_str(),
            decision.pass_iteration,
            decision.action.as_str(),
            option_u8(decision.before_staff),
            option_u8(decision.after_staff),
            option_usize(decision.witness_a_source_ordinal),
            option_usize(decision.witness_b_source_ordinal),
            decision.observed_primary,
            decision.observed_secondary,
            decision.threshold_primary,
            decision.threshold_secondary,
            decision.focus_score,
            decision.tie_break_code,
            decision.removal_reason.map_or("none", |reason| reason.as_str()),
        )
        .expect("String write cannot fail");
    }

    writeln!(out, "SourceSelectionCandidates: {}", solver.unique_source_selection_candidate_count)
        .expect("String write cannot fail");
    writeln!(out, "FinalSourceSelectionCandidateID: {}", solver.final_source_selection_candidate_id)
        .expect("String write cannot fail");
    writeln!(out, "FinalSourceSelectionOriginStage: {}", solver.final_source_selection_origin_stage)
        .expect("String write cannot fail");
    writeln!(out, "CandidateFrontierFingerprintSHA256: {}", solver.candidate_frontier_fingerprint_sha256)
        .expect("String write cannot fail");
    writeln!(out, "SolverFrontierStates: {}", solver.solver_frontier.len())
        .expect("String write cannot fail");
    writeln!(out, "SolverFrontierScope: final_decision_pool_plus_winner")
        .expect("String write cannot fail");
    writeln!(out, "ProbeOnlyCandidatesOutsideDecisionFrontierPossible: ja")
        .expect("String write cannot fail");
    writeln!(out, "FinalSolverIteration: {}", solver.final_solver_iteration)
        .expect("String write cannot fail");
    writeln!(out, "FinalSolverProfileName: {}", solver.final_solver_profile_name)
        .expect("String write cannot fail");
    writeln!(
        out,
        "FinalSolverStateFingerprintSHA256: {}",
        solver.final_solver_state_fingerprint_sha256
    )
    .expect("String write cannot fail");
    writeln!(out, "SolverIterationIsImmutableIdentity: nein")
        .expect("String write cannot fail");
    writeln!(out, "SolverStateIdentity: full_frontier_row_fingerprint")
        .expect("String write cannot fail");
    writeln!(out, "SolverFrontierFingerprintSHA256: {}", solver.solver_frontier_fingerprint_sha256)
        .expect("String write cannot fail");
    writeln!(out, "SolverStagePathFingerprintSHA256: {}", solver.stage_path_fingerprint_sha256)
        .expect("String write cannot fail");
    for checkpoint in &solver.stage_checkpoints {
        writeln!(
            out,
            "SolverCheckpoint stage={} source_candidate_id={} current_iteration={} profile={}",
            checkpoint.stage,
            checkpoint.source_selection_candidate_id,
            checkpoint.current_candidate_iteration,
            checkpoint.current_profile_name,
        )
        .expect("String write cannot fail");
    }
    for row in &solver.solver_frontier {
        writeln!(
            out,
            "SolverCandidate iteration={} state_fp={} profile={} lineage={} source_candidate_id={} raw_blockers={} raw_critical_count={} raw_max_critical_hardness={} raw_warnings={} raw_hints={} arrangement_retention_milli={} blockers={} warnings={} hints={} human_moves={}",
            row.solver_iteration,
            crate::source_selection::solver_frontier_state_fingerprint_sha256(row),
            row.solver_profile_name,
            row.solver_profile_lineage,
            row.source_selection_candidate_id,
            row.evaluation.blocker_count,
            row.evaluation.critical_count,
            row.evaluation.max_critical_hardness,
            row.evaluation.warning_count,
            row.evaluation.hint_count,
            row.evaluation
                .arrangement_visible_retention_milli_percent
                .map_or_else(|| "none".to_string(), |value| value.to_string()),
            row.blockers,
            row.warnings,
            row.hints,
            row.human_potential_notes_moved,
        )
        .expect("String write cannot fail");
    }
    for row in &solver.candidates {
        let evaluation = &row.evaluation;
        writeln!(
            out,
            "SourceCandidate id={} origin={} profile={} span={} jump={} smooth={} left_high={} right_low={} hotspot_issues={} hotspot_groups={} hotspot_limit={} hotspot_truncated={} hotspot_fp={} canonical_retention_ppm={} shadow_fp={} raw_blockers={} raw_critical_count={} raw_max_critical_hardness={} raw_warnings={} raw_hints={} arrangement_retention_milli={} human_moves={}",
            row.candidate_id,
            row.origin_stage,
            row.profile_name,
            row.span_limit,
            row.jump_limit,
            if row.smooth_jumps { "ja" } else { "nein" },
            row.left_high_soft_limit,
            row.right_low_soft_limit,
            row.hotspot_source_issue_count,
            row.hotspot_group_count_before_truncation,
            row.hotspot_requested_limit,
            row.hotspot_truncated_count,
            row.hotspot_fingerprint_sha256,
            row.canonical_source_retention_ppm,
            row.shadow_plan_fingerprint_sha256,
            evaluation.blocker_count,
            evaluation.critical_count,
            evaluation.max_critical_hardness,
            evaluation.warning_count,
            evaluation.hint_count,
            evaluation.arrangement_visible_retention_milli_percent.map_or_else(|| "none".to_string(), |value| value.to_string()),
            evaluation.human_potential_notes_moved,
        )
        .expect("String write cannot fail");
    }
}
