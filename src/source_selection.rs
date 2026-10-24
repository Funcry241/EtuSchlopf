//! Canonical identity transport proof at the productive MIDI-seed selection boundary.
//!
//! This module does not choose, rank, weight, move or remove musical notes. It proves that the
//! already-existing MIDI-seed selection preserves CanonicalSourceEvent identity for every retained
//! event and that the removed ordinals are exactly the complement of the retained ordinals. After
//! that immutable SelectionPlan is bound, it may also derive a deterministic technical interval-lane
//! plan; those lanes have no MusicXML voice, tie, chord or writer semantics in r644.

use crate::arrangement::{
    ArrangementMaterialDispositionKind, ArrangementMaterialDispositionRow,
};
use crate::sha256;
use crate::source_event::CanonicalSourceEvent;
use anyhow::anyhow;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RetainedSourceSelectionIdentity {
    pub(crate) source_ordinal: usize,
    pub(crate) pos: i64,
    pub(crate) len: i64,
    pub(crate) midi: i32,
    pub(crate) final_staff: u8,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ExistingSourceSelectionRemovalReason {
    HotspotSpan,
    HotspotJump,
    TrajectoryZigzag,
    ResidualCriticalSpan,
    ResidualCriticalJump,
}

impl ExistingSourceSelectionRemovalReason {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::HotspotSpan => "hotspot_span_reduction",
            Self::HotspotJump => "hotspot_jump_reduction",
            Self::TrajectoryZigzag => "trajectory_zigzag_reduction",
            Self::ResidualCriticalSpan => "residual_critical_span_reduction",
            Self::ResidualCriticalJump => "residual_critical_jump_reduction",
        }
    }

    const fn discriminator(self) -> u8 {
        match self {
            Self::HotspotSpan => 1,
            Self::HotspotJump => 2,
            // Discriminator 3 belonged to the retired soft-register hotspot reason.
            // Keep the remaining historical ledger identities stable.
            Self::TrajectoryZigzag => 4,
            Self::ResidualCriticalSpan => 5,
            Self::ResidualCriticalJump => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RemovedSourceSelectionReason {
    pub(crate) source_ordinal: usize,
    pub(crate) reason: ExistingSourceSelectionRemovalReason,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SourceSelectionDecisionPass {
    InitialPitchStaff,
    PianoHandSplit,
    SmoothFastStaffJump,
    SmoothWideSimultaneity,
    HotspotSpan,
    HotspotJump,
    TrajectoryMoveOutlier,
    TrajectoryZigzag,
    PostTrajectoryWideSimultaneity,
    ResidualRegisterMove,
    ResidualCriticalSpan,
    ResidualCriticalJump,
    PostResidualWideSimultaneity,
}

impl SourceSelectionDecisionPass {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::InitialPitchStaff => "initial_pitch_staff",
            Self::PianoHandSplit => "piano_hand_split",
            Self::SmoothFastStaffJump => "smooth_fast_staff_jump",
            Self::SmoothWideSimultaneity => "smooth_wide_simultaneity",
            Self::HotspotSpan => "hotspot_span",
            Self::HotspotJump => "hotspot_jump",
            Self::TrajectoryMoveOutlier => "trajectory_move_outlier",
            Self::TrajectoryZigzag => "trajectory_zigzag",
            Self::PostTrajectoryWideSimultaneity => "post_trajectory_wide_simultaneity",
            Self::ResidualRegisterMove => "residual_register_move",
            Self::ResidualCriticalSpan => "residual_critical_span",
            Self::ResidualCriticalJump => "residual_critical_jump",
            Self::PostResidualWideSimultaneity => "post_residual_wide_simultaneity",
        }
    }

    const fn discriminator(self) -> u8 {
        match self {
            Self::InitialPitchStaff => 1,
            Self::PianoHandSplit => 2,
            Self::SmoothFastStaffJump => 3,
            Self::SmoothWideSimultaneity => 4,
            Self::HotspotSpan => 5,
            Self::HotspotJump => 6,
            // Discriminator 7 belonged to the retired soft-register hotspot pass.
            // Do not renumber the surviving decision-pass identities.
            Self::TrajectoryMoveOutlier => 8,
            Self::TrajectoryZigzag => 9,
            Self::PostTrajectoryWideSimultaneity => 10,
            Self::ResidualRegisterMove => 11,
            Self::ResidualCriticalSpan => 12,
            Self::ResidualCriticalJump => 13,
            Self::PostResidualWideSimultaneity => 14,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceSelectionDecisionAction {
    InitialStaff,
    StaffMove,
    Remove,
}

impl SourceSelectionDecisionAction {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::InitialStaff => "initial_staff",
            Self::StaffMove => "staff_move",
            Self::Remove => "remove",
        }
    }

    const fn discriminator(self) -> u8 {
        match self {
            Self::InitialStaff => 1,
            Self::StaffMove => 2,
            Self::Remove => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceSelectionDecisionEvidence {
    pub(crate) source_ordinal: usize,
    pub(crate) pass: SourceSelectionDecisionPass,
    pub(crate) pass_iteration: usize,
    pub(crate) action: SourceSelectionDecisionAction,
    pub(crate) pos: i64,
    pub(crate) len: i64,
    pub(crate) midi: i32,
    pub(crate) before_staff: Option<u8>,
    pub(crate) after_staff: Option<u8>,
    pub(crate) witness_a_source_ordinal: Option<usize>,
    pub(crate) witness_b_source_ordinal: Option<usize>,
    pub(crate) observed_primary: i64,
    pub(crate) observed_secondary: i64,
    pub(crate) threshold_primary: i64,
    pub(crate) threshold_secondary: i64,
    pub(crate) focus_score: i64,
    pub(crate) tie_break_code: u8,
    pub(crate) removal_reason: Option<ExistingSourceSelectionRemovalReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceSelectionShadowPlanRow {
    pub(crate) source_ordinal: usize,
    pub(crate) pos: i64,
    pub(crate) len: i64,
    pub(crate) midi: i32,
    pub(crate) retained: bool,
    pub(crate) final_staff: Option<u8>,
    pub(crate) removal_reason: Option<ExistingSourceSelectionRemovalReason>,
    pub(crate) decision_count: usize,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MidiSeedSerializedSourceIdentityRow {
    pub(crate) measure_ordinal: usize,
    pub(crate) raw_note_index: usize,
    pub(crate) source_ordinal: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct MidiSeedSerializedSourceIdentityPlan {
    pub(crate) authority_bound: bool,
    pub(crate) retained_event_count: usize,
    pub(crate) mapped_event_count: usize,
    pub(crate) measure_raw_note_counts: Arc<[usize]>,
    pub(crate) fingerprint_fnv64: u64,
    pub(crate) rows: Arc<[MidiSeedSerializedSourceIdentityRow]>,
}

impl Default for MidiSeedSerializedSourceIdentityPlan {
    fn default() -> Self {
        Self {
            authority_bound: false,
            retained_event_count: 0,
            mapped_event_count: 0,
            measure_raw_note_counts: Arc::from(Vec::<usize>::new()),
            fingerprint_fnv64: 0,
            rows: Arc::from(Vec::<MidiSeedSerializedSourceIdentityRow>::new()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceDurationMaterialDisposition {
    Visible,
    Hidden,
    Restified,
}

impl SourceDurationMaterialDisposition {
    const fn discriminator(self) -> u8 {
        match self {
            Self::Visible => 1,
            Self::Hidden => 2,
            Self::Restified => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceDurationPostLegacyDispositionRow {
    pub(crate) source_ordinal: usize,
    pub(crate) source_pos: i64,
    pub(crate) source_len: i64,
    pub(crate) source_midi: i32,
    pub(crate) selection_staff: u8,
    pub(crate) projected_staff: Option<u8>,
    pub(crate) disposition: SourceDurationMaterialDisposition,
    pub(crate) human_potential_moved: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct SourceDurationPostLegacyDispositionPlan {
    pub(crate) authority_bound: bool,
    pub(crate) retained_event_count: usize,
    pub(crate) visible_pitched_event_count: usize,
    pub(crate) hidden_pitched_event_count: usize,
    pub(crate) restified_pitched_event_count: usize,
    pub(crate) human_potential_moved_event_count: usize,
    pub(crate) projected_staff_change_count: usize,
    pub(crate) raw_note_coverage_exact: bool,
    pub(crate) fingerprint_fnv64: u64,
    pub(crate) rows: Arc<[SourceDurationPostLegacyDispositionRow]>,
}

impl Default for SourceDurationPostLegacyDispositionPlan {
    fn default() -> Self {
        Self {
            authority_bound: false,
            retained_event_count: 0,
            visible_pitched_event_count: 0,
            hidden_pitched_event_count: 0,
            restified_pitched_event_count: 0,
            human_potential_moved_event_count: 0,
            projected_staff_change_count: 0,
            raw_note_coverage_exact: false,
            fingerprint_fnv64: 0,
            rows: Arc::from(Vec::<SourceDurationPostLegacyDispositionRow>::new()),
        }
    }
}


#[derive(Debug, Clone)]
pub(crate) struct SourceSelectionFullCausalProjection {
    pub(crate) canonical_source_event_count: usize,
    pub(crate) retained_event_count: usize,
    pub(crate) removed_event_count: usize,
    pub(crate) initial_staff_assignment_count: usize,
    pub(crate) staff_move_count: usize,
    pub(crate) removal_decision_count: usize,
    pub(crate) canonical_source_retention_ppm: u64,
    pub(crate) event_decision_trace_fingerprint_sha256: String,
    pub(crate) shadow_plan_fingerprint_sha256: String,
    pub(crate) productive_plan_authority: bool,
    pub(crate) productive_plan_equivalence_verified: bool,
    pub(crate) productive_plan_retained_event_count: usize,
    pub(crate) productive_plan_seed_fingerprint_sha256: String,
    pub(crate) midi_seed_serialized_source_identity_plan: MidiSeedSerializedSourceIdentityPlan,
    pub(crate) productive_post_legacy_disposition_plan: SourceDurationPostLegacyDispositionPlan,
    pub(crate) decisions: Arc<[SourceSelectionDecisionEvidence]>,
    pub(crate) shadow_rows: Arc<[SourceSelectionShadowPlanRow]>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct SourceSelectionCandidateEvaluation {
    pub(crate) blocker_count: usize,
    pub(crate) critical_count: usize,
    pub(crate) max_critical_hardness: i64,
    pub(crate) warning_count: usize,
    pub(crate) hint_count: usize,
    pub(crate) arrangement_visible_retention_milli_percent: Option<i64>,
    pub(crate) human_potential_notes_moved: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct SourceSelectionCandidateAuditRow {
    pub(crate) candidate_id: usize,
    pub(crate) origin_stage: String,
    pub(crate) profile_name: String,
    pub(crate) span_limit: i32,
    pub(crate) jump_limit: i32,
    pub(crate) smooth_jumps: bool,
    pub(crate) left_high_soft_limit: i32,
    pub(crate) right_low_soft_limit: i32,
    pub(crate) hotspot_source_issue_count: usize,
    pub(crate) hotspot_group_count_before_truncation: usize,
    pub(crate) hotspot_requested_limit: usize,
    pub(crate) hotspot_truncated_count: usize,
    pub(crate) hotspot_fingerprint_sha256: String,
    pub(crate) canonical_source_retention_ppm: u64,
    pub(crate) shadow_plan_fingerprint_sha256: String,
    pub(crate) evaluation: SourceSelectionCandidateEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceSelectionSolverStageCheckpoint {
    pub(crate) stage: String,
    pub(crate) source_selection_candidate_id: usize,
    pub(crate) current_candidate_iteration: usize,
    pub(crate) current_profile_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceSelectionSolverFrontierRow {
    pub(crate) solver_iteration: usize,
    pub(crate) solver_profile_name: String,
    pub(crate) solver_profile_lineage: String,
    pub(crate) source_selection_candidate_id: usize,
    pub(crate) evaluation: SourceSelectionCandidateEvaluation,
    pub(crate) human_potential_notes_moved: usize,
    pub(crate) blockers: usize,
    pub(crate) warnings: usize,
    pub(crate) hints: usize,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SourceSelectionSolverCausalityProjection {
    pub(crate) unique_source_selection_candidate_count: usize,
    pub(crate) final_source_selection_candidate_id: usize,
    pub(crate) final_source_selection_origin_stage: String,
    pub(crate) candidate_frontier_fingerprint_sha256: String,
    pub(crate) solver_frontier_fingerprint_sha256: String,
    pub(crate) stage_path_fingerprint_sha256: String,
    pub(crate) final_solver_iteration: usize,
    pub(crate) final_solver_profile_name: String,
    pub(crate) final_solver_state_fingerprint_sha256: String,
    pub(crate) candidates: Vec<SourceSelectionCandidateAuditRow>,
    pub(crate) solver_frontier: Vec<SourceSelectionSolverFrontierRow>,
    pub(crate) stage_checkpoints: Vec<SourceSelectionSolverStageCheckpoint>,
}




pub(crate) fn project_source_selection_full_causal_audit(
    canonical_events: &[CanonicalSourceEvent],
    retained_events: &[RetainedSourceSelectionIdentity],
    removed_reasons: &[RemovedSourceSelectionReason],
    decisions: Vec<SourceSelectionDecisionEvidence>,
) -> anyhow::Result<SourceSelectionFullCausalProjection> {
    let mut canonical_by_ordinal = BTreeMap::new();
    for event in canonical_events {
        if canonical_by_ordinal.insert(event.source_ordinal, *event).is_some() {
            return Err(anyhow!(
                "SourceSelectionFullCausalAudit: doppelte CanonicalSourceEvent-Ordinalitaet {}",
                event.source_ordinal
            ));
        }
    }
    for expected in 0..canonical_events.len() {
        if !canonical_by_ordinal.contains_key(&expected) {
            return Err(anyhow!(
                "SourceSelectionFullCausalAudit: fehlende CanonicalSourceEvent-Ordinalitaet {expected}"
            ));
        }
    }

    let mut retained_by_ordinal = BTreeMap::new();
    for retained in retained_events {
        if !matches!(retained.final_staff, 1 | 2) {
            return Err(anyhow!(
                "SourceSelectionFullCausalAudit: ungueltiger finaler Staff {} fuer Source-Ordinalitaet {}",
                retained.final_staff,
                retained.source_ordinal
            ));
        }
        if retained_by_ordinal
            .insert(retained.source_ordinal, *retained)
            .is_some()
        {
            return Err(anyhow!(
                "SourceSelectionFullCausalAudit: doppelte retained Source-Ordinalitaet {}",
                retained.source_ordinal
            ));
        }
        let canonical = canonical_by_ordinal.get(&retained.source_ordinal).ok_or_else(|| {
            anyhow!(
                "SourceSelectionFullCausalAudit: retained Source-Ordinalitaet {} ist nicht kanonisch",
                retained.source_ordinal
            )
        })?;
        if retained.pos != canonical.pos || retained.len != canonical.len || retained.midi != canonical.midi {
            return Err(anyhow!(
                "SourceSelectionFullCausalAudit: retained Source-Ordinalitaet {} veraendert kanonische Identitaet",
                retained.source_ordinal
            ));
        }
    }

    let mut reason_by_ordinal = BTreeMap::new();
    for entry in removed_reasons {
        if reason_by_ordinal.insert(entry.source_ordinal, entry.reason).is_some() {
            return Err(anyhow!(
                "SourceSelectionFullCausalAudit: Source-Ordinalitaet {} besitzt mehrere Removal-Reasons",
                entry.source_ordinal
            ));
        }
    }

    #[derive(Clone, Copy)]
    enum LiveState {
        Active(u8),
        Removed(ExistingSourceSelectionRemovalReason),
    }
    let mut state_by_ordinal: BTreeMap<usize, LiveState> = BTreeMap::new();
    let mut decision_count_by_ordinal: BTreeMap<usize, usize> = BTreeMap::new();
    let mut initial_staff_assignment_count = 0usize;
    let mut staff_move_count = 0usize;
    let mut removal_decision_count = 0usize;
    let mut trace_bytes = b"ETUSCHLOPF_SOURCE_SELECTION_EVENT_DECISION_TRACE_V1\0".to_vec();
    append_usize(&mut trace_bytes, canonical_events.len(), "CanonicalSourceEventCount")?;
    append_usize(&mut trace_bytes, decisions.len(), "DecisionCount")?;

    for decision in &decisions {
        let canonical = canonical_by_ordinal.get(&decision.source_ordinal).ok_or_else(|| {
            anyhow!(
                "SourceSelectionFullCausalAudit: Decision referenziert unbekannte Source-Ordinalitaet {}",
                decision.source_ordinal
            )
        })?;
        if decision.pos != canonical.pos || decision.len != canonical.len || decision.midi != canonical.midi {
            return Err(anyhow!(
                "SourceSelectionFullCausalAudit: Decision fuer Source-Ordinalitaet {} veraendert Position, Dauer oder Pitch",
                decision.source_ordinal
            ));
        }
        for witness in [decision.witness_a_source_ordinal, decision.witness_b_source_ordinal]
            .into_iter()
            .flatten()
        {
            if !canonical_by_ordinal.contains_key(&witness) {
                return Err(anyhow!(
                    "SourceSelectionFullCausalAudit: Decision fuer Source-Ordinalitaet {} referenziert unbekannten Witness {}",
                    decision.source_ordinal,
                    witness
                ));
            }
        }

        match decision.action {
            SourceSelectionDecisionAction::InitialStaff => {
                if decision.pass != SourceSelectionDecisionPass::InitialPitchStaff
                    || decision.before_staff.is_some()
                    || !matches!(decision.after_staff, Some(1 | 2))
                    || decision.removal_reason.is_some()
                {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: ungueltige Initial-Staff-Decision fuer Source-Ordinalitaet {}",
                        decision.source_ordinal
                    ));
                }
                if state_by_ordinal
                    .insert(decision.source_ordinal, LiveState::Active(decision.after_staff.unwrap_or(0)))
                    .is_some()
                {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Source-Ordinalitaet {} besitzt mehrere Initial-Staff-Decisions",
                        decision.source_ordinal
                    ));
                }
                initial_staff_assignment_count += 1;
            }
            SourceSelectionDecisionAction::StaffMove => {
                let Some(LiveState::Active(current_staff)) = state_by_ordinal.get(&decision.source_ordinal).copied() else {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Staff-Move fuer nicht aktives Event {}",
                        decision.source_ordinal
                    ));
                };
                let Some(before_staff) = decision.before_staff else {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Staff-Move ohne Before-Staff fuer {}",
                        decision.source_ordinal
                    ));
                };
                let Some(after_staff) = decision.after_staff else {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Staff-Move ohne After-Staff fuer {}",
                        decision.source_ordinal
                    ));
                };
                if current_staff != before_staff || !matches!(after_staff, 1 | 2) || after_staff == before_staff {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Staff-Move-Statebruch fuer Source-Ordinalitaet {}: live={} before={} after={}",
                        decision.source_ordinal,
                        current_staff,
                        before_staff,
                        after_staff
                    ));
                }
                if decision.removal_reason.is_some() {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Staff-Move fuer {} traegt Removal-Reason",
                        decision.source_ordinal
                    ));
                }
                state_by_ordinal.insert(decision.source_ordinal, LiveState::Active(after_staff));
                staff_move_count += 1;
            }
            SourceSelectionDecisionAction::Remove => {
                let Some(LiveState::Active(current_staff)) = state_by_ordinal.get(&decision.source_ordinal).copied() else {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Remove fuer nicht aktives Event {}",
                        decision.source_ordinal
                    ));
                };
                let reason = decision.removal_reason.ok_or_else(|| {
                    anyhow!(
                        "SourceSelectionFullCausalAudit: Remove fuer {} ohne Removal-Reason",
                        decision.source_ordinal
                    )
                })?;
                if decision.before_staff != Some(current_staff) || decision.after_staff.is_some() {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Remove-Statebruch fuer Source-Ordinalitaet {}",
                        decision.source_ordinal
                    ));
                }
                if reason_by_ordinal.get(&decision.source_ordinal).copied() != Some(reason) {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Remove-Decision und Reason-Ledger widersprechen sich fuer {}",
                        decision.source_ordinal
                    ));
                }
                state_by_ordinal.insert(decision.source_ordinal, LiveState::Removed(reason));
                removal_decision_count += 1;
            }
        }
        *decision_count_by_ordinal.entry(decision.source_ordinal).or_default() += 1;
        append_decision_evidence(&mut trace_bytes, decision)?;
    }

    if initial_staff_assignment_count != canonical_events.len() {
        return Err(anyhow!(
            "SourceSelectionFullCausalAudit: Initial-Staff-Decisions {} != CanonicalSourceEvents {}",
            initial_staff_assignment_count,
            canonical_events.len()
        ));
    }

    let mut shadow_rows = Vec::with_capacity(canonical_events.len());
    let mut plan_bytes = b"ETUSCHLOPF_SOURCE_SELECTION_IMMUTABLE_SHADOW_PLAN_V1\0".to_vec();
    append_usize(&mut plan_bytes, canonical_events.len(), "ShadowPlanEventCount")?;
    for (ordinal, canonical) in &canonical_by_ordinal {
        let state = state_by_ordinal.get(ordinal).copied().ok_or_else(|| {
            anyhow!(
                "SourceSelectionFullCausalAudit: Source-Ordinalitaet {ordinal} besitzt keinen finalen Trace-State"
            )
        })?;
        let row = match state {
            LiveState::Active(final_staff) => {
                let retained = retained_by_ordinal.get(ordinal).ok_or_else(|| {
                    anyhow!(
                        "SourceSelectionFullCausalAudit: Trace behaelt {ordinal}, produktiver Seed aber nicht"
                    )
                })?;
                if retained.final_staff != final_staff || reason_by_ordinal.contains_key(ordinal) {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: finaler Staff/Removal-State widerspricht produktivem Seed fuer {ordinal}"
                    ));
                }
                SourceSelectionShadowPlanRow {
                    source_ordinal: *ordinal,
                    pos: canonical.pos,
                    len: canonical.len,
                    midi: canonical.midi,
                    retained: true,
                    final_staff: Some(final_staff),
                    removal_reason: None,
                    decision_count: decision_count_by_ordinal.get(ordinal).copied().unwrap_or(0),
                }
            }
            LiveState::Removed(reason) => {
                if retained_by_ordinal.contains_key(ordinal) || reason_by_ordinal.get(ordinal).copied() != Some(reason) {
                    return Err(anyhow!(
                        "SourceSelectionFullCausalAudit: Trace entfernt {ordinal}, produktive Partition widerspricht"
                    ));
                }
                SourceSelectionShadowPlanRow {
                    source_ordinal: *ordinal,
                    pos: canonical.pos,
                    len: canonical.len,
                    midi: canonical.midi,
                    retained: false,
                    final_staff: None,
                    removal_reason: Some(reason),
                    decision_count: decision_count_by_ordinal.get(ordinal).copied().unwrap_or(0),
                }
            }
        };
        append_shadow_plan_row(&mut plan_bytes, &row)?;
        shadow_rows.push(row);
    }

    let removed_event_count = canonical_events.len().saturating_sub(retained_events.len());
    if removal_decision_count != removed_event_count || reason_by_ordinal.len() != removed_event_count {
        return Err(anyhow!(
            "SourceSelectionFullCausalAudit: RemovalDecisionCount/ReasonCount/RemovedCount sind nicht identisch: decisions={} reasons={} removed={}",
            removal_decision_count,
            reason_by_ordinal.len(),
            removed_event_count
        ));
    }
    let canonical_source_retention_ppm = if canonical_events.is_empty() {
        0
    } else {
        let retained = u64::try_from(retained_events.len())
            .map_err(|_| anyhow!("RetainedEventCount ueberlaeuft u64"))?;
        let total = u64::try_from(canonical_events.len())
            .map_err(|_| anyhow!("CanonicalSourceEventCount ueberlaeuft u64"))?;
        retained.saturating_mul(1_000_000) / total
    };

    Ok(SourceSelectionFullCausalProjection {
        canonical_source_event_count: canonical_events.len(),
        retained_event_count: retained_events.len(),
        removed_event_count,
        initial_staff_assignment_count,
        staff_move_count,
        removal_decision_count,
        canonical_source_retention_ppm,
        event_decision_trace_fingerprint_sha256: sha256::hex(&trace_bytes),
        shadow_plan_fingerprint_sha256: sha256::hex(&plan_bytes),
        productive_plan_authority: false,
        productive_plan_equivalence_verified: false,
        productive_plan_retained_event_count: 0,
        productive_plan_seed_fingerprint_sha256: String::new(),
        midi_seed_serialized_source_identity_plan:
            MidiSeedSerializedSourceIdentityPlan::default(),
        productive_post_legacy_disposition_plan:
            SourceDurationPostLegacyDispositionPlan::default(),
        decisions: Arc::from(decisions),
        shadow_rows: Arc::from(shadow_rows),
    })
}

pub(crate) fn bind_productive_source_selection_plan(
    canonical_events: &[CanonicalSourceEvent],
    legacy_retained_events: &[RetainedSourceSelectionIdentity],
    causal: &mut SourceSelectionFullCausalProjection,
) -> anyhow::Result<Vec<RetainedSourceSelectionIdentity>> {
    if causal.productive_plan_authority
        || causal.productive_plan_equivalence_verified
        || causal.productive_plan_retained_event_count != 0
        || !causal.productive_plan_seed_fingerprint_sha256.is_empty()
    {
        return Err(anyhow!(
            "SourceSelectionPlanAuthorityCutover: productive Plan-Authority wurde mehrfach gebunden"
        ));
    }
    if causal.canonical_source_event_count != canonical_events.len()
        || causal.shadow_rows.len() != canonical_events.len()
    {
        return Err(anyhow!(
            "SourceSelectionPlanAuthorityCutover: Plan-/Canonical-Groesse widerspricht r634-CausalProjection"
        ));
    }

    let mut canonical_by_ordinal = BTreeMap::new();
    for event in canonical_events {
        if canonical_by_ordinal.insert(event.source_ordinal, *event).is_some() {
            return Err(anyhow!(
                "SourceSelectionPlanAuthorityCutover: doppelte CanonicalSourceEvent-Ordinalitaet {}",
                event.source_ordinal
            ));
        }
    }
    for expected in 0..canonical_events.len() {
        if !canonical_by_ordinal.contains_key(&expected) {
            return Err(anyhow!(
                "SourceSelectionPlanAuthorityCutover: fehlende CanonicalSourceEvent-Ordinalitaet {expected}"
            ));
        }
    }

    let mut plan_ordinals = BTreeSet::new();
    let mut planned_retained = Vec::with_capacity(causal.retained_event_count);
    for row in causal.shadow_rows.iter() {
        if !plan_ordinals.insert(row.source_ordinal) {
            return Err(anyhow!(
                "SourceSelectionPlanAuthorityCutover: doppelte Plan-Ordinalitaet {}",
                row.source_ordinal
            ));
        }
        let canonical = canonical_by_ordinal.get(&row.source_ordinal).ok_or_else(|| {
            anyhow!(
                "SourceSelectionPlanAuthorityCutover: Plan referenziert unbekannte Source-Ordinalitaet {}",
                row.source_ordinal
            )
        })?;
        if row.pos != canonical.pos || row.len != canonical.len || row.midi != canonical.midi {
            return Err(anyhow!(
                "SourceSelectionPlanAuthorityCutover: Plan veraendert Canonical-Identitaet fuer {}",
                row.source_ordinal
            ));
        }
        if row.decision_count == 0 {
            return Err(anyhow!(
                "SourceSelectionPlanAuthorityCutover: Plan-Zeile {} besitzt keine Decision-Chain",
                row.source_ordinal
            ));
        }

        if row.retained {
            let final_staff = row.final_staff.ok_or_else(|| {
                anyhow!(
                    "SourceSelectionPlanAuthorityCutover: retained Plan-Zeile {} ohne finalen Staff",
                    row.source_ordinal
                )
            })?;
            if !matches!(final_staff, 1 | 2) || row.removal_reason.is_some() {
                return Err(anyhow!(
                    "SourceSelectionPlanAuthorityCutover: retained Plan-Zeile {} besitzt ungueltigen Staff/Removal-State",
                    row.source_ordinal
                ));
            }
            planned_retained.push(RetainedSourceSelectionIdentity {
                source_ordinal: row.source_ordinal,
                pos: row.pos,
                len: row.len,
                midi: row.midi,
                final_staff,
            });
        } else if row.final_staff.is_some() || row.removal_reason.is_none() {
            return Err(anyhow!(
                "SourceSelectionPlanAuthorityCutover: removed Plan-Zeile {} besitzt ungueltigen Staff/Reason-State",
                row.source_ordinal
            ));
        }
    }

    if plan_ordinals.len() != canonical_events.len()
        || planned_retained.len() != causal.retained_event_count
    {
        return Err(anyhow!(
            "SourceSelectionPlanAuthorityCutover: Plan bildet Canonical-/Retention-Partition nicht exakt ab"
        ));
    }

    planned_retained.sort_by_key(|event| (event.pos, event.final_staff, event.midi));
    if planned_retained != legacy_retained_events {
        return Err(anyhow!(
            "SourceSelectionPlanAuthorityCutover: immutable Plan materialisiert nicht byte-semantisch denselben retained Seed wie der r634-Arbeitsbestand"
        ));
    }

    let mut materialized_bytes =
        b"ETUSCHLOPF_SOURCE_SELECTION_PRODUCTIVE_PLAN_SEED_V1\0".to_vec();
    append_usize(
        &mut materialized_bytes,
        planned_retained.len(),
        "ProductivePlanRetainedEventCount",
    )?;
    for event in &planned_retained {
        append_productive_retained_identity(&mut materialized_bytes, event)?;
    }

    causal.productive_plan_authority = true;
    causal.productive_plan_equivalence_verified = true;
    causal.productive_plan_retained_event_count = planned_retained.len();
    causal.productive_plan_seed_fingerprint_sha256 = sha256::hex(&materialized_bytes);
    Ok(planned_retained)
}



fn source_identity_transport_fnv1a64_update(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub(crate) fn bind_midi_seed_serialized_source_identity_plan(
    causal: &mut SourceSelectionFullCausalProjection,
    measure_raw_note_counts: Vec<usize>,
    mut rows: Vec<MidiSeedSerializedSourceIdentityRow>,
) -> anyhow::Result<()> {
    if !causal.productive_plan_authority || !causal.productive_plan_equivalence_verified {
        return Err(anyhow!(
            "MidiSeedSerializedSourceIdentityPlan: SelectionPlan-Authority ist nicht gebunden"
        ));
    }
    let plan = &causal.midi_seed_serialized_source_identity_plan;
    if plan.authority_bound
        || plan.retained_event_count != 0
        || plan.mapped_event_count != 0
        || !plan.rows.is_empty()
    {
        return Err(anyhow!(
            "MidiSeedSerializedSourceIdentityPlan: Authority wurde mehrfach gebunden"
        ));
    }

    rows.sort_by_key(|row| {
        (
            row.measure_ordinal,
            row.raw_note_index,
            row.source_ordinal,
        )
    });
    let mut ordinals = BTreeSet::new();
    let mut raw_keys = BTreeSet::new();
    let mut fingerprint = 0xcbf29ce484222325u64;
    for row in &rows {
        if row.measure_ordinal == 0
            || row.measure_ordinal > measure_raw_note_counts.len()
        {
            return Err(anyhow!(
                "MidiSeedSerializedSourceIdentityPlan: ungueltiger Takt {} fuer SourceOrdinal {}",
                row.measure_ordinal,
                row.source_ordinal
            ));
        }
        if row.raw_note_index >= measure_raw_note_counts[row.measure_ordinal - 1] {
            return Err(anyhow!(
                "MidiSeedSerializedSourceIdentityPlan: raw_note_index {} ausserhalb Takt {}",
                row.raw_note_index,
                row.measure_ordinal
            ));
        }
        if !ordinals.insert(row.source_ordinal) {
            return Err(anyhow!(
                "MidiSeedSerializedSourceIdentityPlan: doppelte SourceOrdinal {}",
                row.source_ordinal
            ));
        }
        if !raw_keys.insert((row.measure_ordinal, row.raw_note_index)) {
            return Err(anyhow!(
                "MidiSeedSerializedSourceIdentityPlan: doppelte RawNote-Position Takt {} Index {}",
                row.measure_ordinal,
                row.raw_note_index
            ));
        }
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &u64::try_from(row.measure_ordinal)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &u64::try_from(row.raw_note_index)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &u64::try_from(row.source_ordinal)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
    }

    if rows.len() != causal.productive_plan_retained_event_count
        || ordinals.len() != causal.productive_plan_retained_event_count
    {
        return Err(anyhow!(
            "MidiSeedSerializedSourceIdentityPlan: Coverage {} widerspricht retained {}",
            rows.len(),
            causal.productive_plan_retained_event_count
        ));
    }

    causal.midi_seed_serialized_source_identity_plan =
        MidiSeedSerializedSourceIdentityPlan {
            authority_bound: true,
            retained_event_count: causal.productive_plan_retained_event_count,
            mapped_event_count: rows.len(),
            measure_raw_note_counts: Arc::from(measure_raw_note_counts),
            fingerprint_fnv64: fingerprint,
            rows: Arc::from(rows),
        };
    Ok(())
}

pub(crate) fn bind_productive_post_legacy_selection_disposition_plan(
    causal: &mut SourceSelectionFullCausalProjection,
    disposition_rows: &[ArrangementMaterialDispositionRow],
) -> anyhow::Result<()> {
    if !causal.midi_seed_serialized_source_identity_plan.authority_bound {
        return Err(anyhow!(
            "PostLegacySelectionDispositionPlan: MidiSeed-Serialisierungsidentitaet ist nicht gebunden"
        ));
    }
    let existing = &causal.productive_post_legacy_disposition_plan;
    if existing.authority_bound
        || existing.retained_event_count != 0
        || !existing.rows.is_empty()
    {
        return Err(anyhow!(
            "PostLegacySelectionDispositionPlan: Authority wurde mehrfach gebunden"
        ));
    }

    let serialization = &causal.midi_seed_serialized_source_identity_plan;
    let mut observed = BTreeMap::<(usize, usize), ArrangementMaterialDispositionRow>::new();
    for row in disposition_rows {
        let key = (row.measure_ordinal, row.raw_note_index);
        if observed.insert(key, *row).is_some() {
            return Err(anyhow!(
                "PostLegacySelectionDispositionPlan: doppelte Arrangement-Disposition Takt {} Index {}",
                row.measure_ordinal,
                row.raw_note_index
            ));
        }
    }

    let expected_raw_note_count = serialization
        .measure_raw_note_counts
        .iter()
        .copied()
        .sum::<usize>();
    let raw_note_coverage_exact = observed.len() == expected_raw_note_count;
    if !raw_note_coverage_exact {
        return Err(anyhow!(
            "PostLegacySelectionDispositionPlan: Arrangement-RawNote-Coverage {} widerspricht Serialisierung {}",
            observed.len(),
            expected_raw_note_count
        ));
    }

    let retained_by_ordinal = causal
        .shadow_rows
        .iter()
        .filter(|row| row.retained)
        .map(|row| (row.source_ordinal, row))
        .collect::<BTreeMap<_, _>>();
    let mut output_rows =
        Vec::<SourceDurationPostLegacyDispositionRow>::with_capacity(serialization.rows.len());
    let mut visible_pitched_event_count = 0usize;
    let mut hidden_pitched_event_count = 0usize;
    let mut restified_pitched_event_count = 0usize;
    let mut human_potential_moved_event_count = 0usize;
    let mut projected_staff_change_count = 0usize;
    let mut fingerprint = 0xcbf29ce484222325u64;

    for identity in serialization.rows.iter() {
        let source = retained_by_ordinal
            .get(&identity.source_ordinal)
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "PostLegacySelectionDispositionPlan: SourceOrdinal {} ist nicht retained",
                    identity.source_ordinal
                )
            })?;
        let observed_row = observed
            .get(&(identity.measure_ordinal, identity.raw_note_index))
            .copied()
            .ok_or_else(|| {
                anyhow!(
                    "PostLegacySelectionDispositionPlan: fehlende Arrangement-Disposition fuer Takt {} Index {}",
                    identity.measure_ordinal,
                    identity.raw_note_index
                )
            })?;

        let (disposition, projected_staff) = match observed_row.disposition {
            ArrangementMaterialDispositionKind::VisiblePitched => {
                visible_pitched_event_count =
                    visible_pitched_event_count.saturating_add(1);
                (
                    SourceDurationMaterialDisposition::Visible,
                    Some(observed_row.projected_staff),
                )
            }
            ArrangementMaterialDispositionKind::HiddenPitched => {
                hidden_pitched_event_count =
                    hidden_pitched_event_count.saturating_add(1);
                (
                    SourceDurationMaterialDisposition::Hidden,
                    Some(observed_row.projected_staff),
                )
            }
            ArrangementMaterialDispositionKind::RestifiedPitched => {
                restified_pitched_event_count =
                    restified_pitched_event_count.saturating_add(1);
                (
                    SourceDurationMaterialDisposition::Restified,
                    None,
                )
            }
            ArrangementMaterialDispositionKind::RestOrOther => {
                return Err(anyhow!(
                    "PostLegacySelectionDispositionPlan: SourceOrdinal {} zeigt auf Rest/sonstigen RawNote-Traeger",
                    identity.source_ordinal
                ));
            }
        };

        if observed_row.human_potential_moved {
            human_potential_moved_event_count =
                human_potential_moved_event_count.saturating_add(1);
        }
        if let Some(staff) = projected_staff {
            let selection_staff = source.final_staff.ok_or_else(|| {
                anyhow!(
                    "PostLegacySelectionDispositionPlan: retained SourceOrdinal {} ohne Selection-Staff",
                    source.source_ordinal
                )
            })?;
            if staff != selection_staff {
                projected_staff_change_count =
                    projected_staff_change_count.saturating_add(1);
            }
        }

        let selection_staff = source.final_staff.ok_or_else(|| {
            anyhow!(
                "PostLegacySelectionDispositionPlan: retained SourceOrdinal {} ohne Selection-Staff",
                source.source_ordinal
            )
        })?;
        let output = SourceDurationPostLegacyDispositionRow {
            source_ordinal: source.source_ordinal,
            source_pos: source.pos,
            source_len: source.len,
            source_midi: source.midi,
            selection_staff,
            projected_staff,
            disposition,
            human_potential_moved: observed_row.human_potential_moved,
        };

        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &u64::try_from(output.source_ordinal)
                .unwrap_or(u64::MAX)
                .to_le_bytes(),
        );
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &output.source_pos.to_le_bytes(),
        );
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &output.source_len.to_le_bytes(),
        );
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &output.source_midi.to_le_bytes(),
        );
        fingerprint =
            source_identity_transport_fnv1a64_update(fingerprint, &[output.selection_staff]);
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &[output.projected_staff.unwrap_or(0)],
        );
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &[output.disposition.discriminator()],
        );
        fingerprint = source_identity_transport_fnv1a64_update(
            fingerprint,
            &[u8::from(output.human_potential_moved)],
        );
        output_rows.push(output);
    }

    output_rows.sort_by_key(|row| row.source_ordinal);
    if output_rows.len() != causal.productive_plan_retained_event_count
        || visible_pitched_event_count
            .saturating_add(hidden_pitched_event_count)
            .saturating_add(restified_pitched_event_count)
            != causal.productive_plan_retained_event_count
    {
        return Err(anyhow!(
            "PostLegacySelectionDispositionPlan: Event-Coverage ist nicht exakt"
        ));
    }

    causal.productive_post_legacy_disposition_plan =
        SourceDurationPostLegacyDispositionPlan {
            authority_bound: true,
            retained_event_count: causal.productive_plan_retained_event_count,
            visible_pitched_event_count,
            hidden_pitched_event_count,
            restified_pitched_event_count,
            human_potential_moved_event_count,
            projected_staff_change_count,
            raw_note_coverage_exact,
            fingerprint_fnv64: fingerprint,
            rows: Arc::from(output_rows),
        };
    Ok(())
}

pub(crate) fn project_source_selection_solver_causality(
    mut candidates: Vec<SourceSelectionCandidateAuditRow>,
    mut solver_frontier: Vec<SourceSelectionSolverFrontierRow>,
    stage_checkpoints: Vec<SourceSelectionSolverStageCheckpoint>,
    final_source_selection_candidate_id: usize,
    final_solver_row: SourceSelectionSolverFrontierRow,
) -> anyhow::Result<SourceSelectionSolverCausalityProjection> {
    candidates.sort_by_key(|row| row.candidate_id);
    let mut ids = BTreeSet::new();
    for row in &candidates {
        if !ids.insert(row.candidate_id) {
            return Err(anyhow!(
                "SourceSelectionSolverCausality: doppelte SourceSelectionCandidate-ID {}",
                row.candidate_id
            ));
        }
    }
    let final_row = candidates
        .iter()
        .find(|row| row.candidate_id == final_source_selection_candidate_id)
        .ok_or_else(|| {
            anyhow!(
                "SourceSelectionSolverCausality: finaler SourceSelectionCandidate {} fehlt im Frontier",
                final_source_selection_candidate_id
            )
        })?;
    for checkpoint in &stage_checkpoints {
        if !ids.contains(&checkpoint.source_selection_candidate_id) {
            return Err(anyhow!(
                "SourceSelectionSolverCausality: Stage {} referenziert unbekannten SourceSelectionCandidate {}",
                checkpoint.stage,
                checkpoint.source_selection_candidate_id
            ));
        }
    }

    solver_frontier.sort_by(|left, right| {
        left.solver_iteration
            .cmp(&right.solver_iteration)
            .then_with(|| left.solver_profile_name.cmp(&right.solver_profile_name))
            .then_with(|| left.solver_profile_lineage.cmp(&right.solver_profile_lineage))
            .then_with(|| {
                left.source_selection_candidate_id
                    .cmp(&right.source_selection_candidate_id)
            })
            .then_with(|| {
                left.human_potential_notes_moved
                    .cmp(&right.human_potential_notes_moved)
            })
            .then_with(|| left.blockers.cmp(&right.blockers))
            .then_with(|| left.warnings.cmp(&right.warnings))
            .then_with(|| left.hints.cmp(&right.hints))
    });
    solver_frontier.dedup();

    for row in &solver_frontier {
        if !ids.contains(&row.source_selection_candidate_id) {
            return Err(anyhow!(
                "SourceSelectionSolverCausality: Solver-Zustand iteration={} state_fp={} referenziert unbekannten SourceSelectionCandidate {}",
                row.solver_iteration,
                solver_frontier_state_fingerprint_sha256(row),
                row.source_selection_candidate_id
            ));
        }
    }
    let final_solver_state_fingerprint_sha256 =
        solver_frontier_state_fingerprint_sha256(&final_solver_row);
    if final_solver_row.source_selection_candidate_id != final_source_selection_candidate_id {
        return Err(anyhow!(
            "SourceSelectionSolverCausality: finaler Solver-Zustand iteration={} state_fp={} bindet SourceSelectionCandidate {}, erwartet {}",
            final_solver_row.solver_iteration,
            final_solver_state_fingerprint_sha256,
            final_solver_row.source_selection_candidate_id,
            final_source_selection_candidate_id
        ));
    }
    if !solver_frontier.iter().any(|row| row == &final_solver_row) {
        return Err(anyhow!(
            "SourceSelectionSolverCausality: finaler Solver-Zustand iteration={} state_fp={} fehlt im Frontier",
            final_solver_row.solver_iteration,
            final_solver_state_fingerprint_sha256
        ));
    }

    let mut frontier_bytes = b"ETUSCHLOPF_SOURCE_SELECTION_CANDIDATE_FRONTIER_V2\0".to_vec();
    append_usize(&mut frontier_bytes, candidates.len(), "SourceSelectionCandidateCount")?;
    for row in &candidates {
        append_candidate_audit_row(&mut frontier_bytes, row)?;
    }
    let mut solver_frontier_bytes = b"ETUSCHLOPF_SOURCE_SELECTION_SOLVER_FRONTIER_V2\0".to_vec();
    append_usize(
        &mut solver_frontier_bytes,
        solver_frontier.len(),
        "SolverFrontierCandidateCount",
    )?;
    for row in &solver_frontier {
        append_solver_frontier_row(&mut solver_frontier_bytes, row)?;
    }
    let mut path_bytes = b"ETUSCHLOPF_SOURCE_SELECTION_SOLVER_STAGE_PATH_V1\0".to_vec();
    append_usize(&mut path_bytes, stage_checkpoints.len(), "SolverStageCheckpointCount")?;
    for checkpoint in &stage_checkpoints {
        append_str(&mut path_bytes, &checkpoint.stage, "SolverStage")?;
        append_usize(
            &mut path_bytes,
            checkpoint.source_selection_candidate_id,
            "StageSourceSelectionCandidateId",
        )?;
        append_usize(
            &mut path_bytes,
            checkpoint.current_candidate_iteration,
            "StageCurrentCandidateIteration",
        )?;
        append_str(&mut path_bytes, &checkpoint.current_profile_name, "StageCurrentProfileName")?;
    }

    Ok(SourceSelectionSolverCausalityProjection {
        unique_source_selection_candidate_count: candidates.len(),
        final_source_selection_candidate_id,
        final_source_selection_origin_stage: final_row.origin_stage.clone(),
        candidate_frontier_fingerprint_sha256: sha256::hex(&frontier_bytes),
        solver_frontier_fingerprint_sha256: sha256::hex(&solver_frontier_bytes),
        stage_path_fingerprint_sha256: sha256::hex(&path_bytes),
        final_solver_iteration: final_solver_row.solver_iteration,
        final_solver_profile_name: final_solver_row.solver_profile_name.clone(),
        final_solver_state_fingerprint_sha256,
        candidates,
        solver_frontier,
        stage_checkpoints,
    })
}

fn append_decision_evidence(
    out: &mut Vec<u8>,
    decision: &SourceSelectionDecisionEvidence,
) -> anyhow::Result<()> {
    append_usize(out, decision.source_ordinal, "DecisionSourceOrdinal")?;
    out.push(decision.pass.discriminator());
    append_usize(out, decision.pass_iteration, "DecisionPassIteration")?;
    out.push(decision.action.discriminator());
    out.extend_from_slice(&decision.pos.to_le_bytes());
    out.extend_from_slice(&decision.len.to_le_bytes());
    out.extend_from_slice(&decision.midi.to_le_bytes());
    append_opt_u8(out, decision.before_staff);
    append_opt_u8(out, decision.after_staff);
    append_opt_usize(out, decision.witness_a_source_ordinal, "DecisionWitnessA")?;
    append_opt_usize(out, decision.witness_b_source_ordinal, "DecisionWitnessB")?;
    out.extend_from_slice(&decision.observed_primary.to_le_bytes());
    out.extend_from_slice(&decision.observed_secondary.to_le_bytes());
    out.extend_from_slice(&decision.threshold_primary.to_le_bytes());
    out.extend_from_slice(&decision.threshold_secondary.to_le_bytes());
    out.extend_from_slice(&decision.focus_score.to_le_bytes());
    out.push(decision.tie_break_code);
    out.push(decision.removal_reason.map_or(0, ExistingSourceSelectionRemovalReason::discriminator));
    Ok(())
}

fn append_shadow_plan_row(
    out: &mut Vec<u8>,
    row: &SourceSelectionShadowPlanRow,
) -> anyhow::Result<()> {
    append_usize(out, row.source_ordinal, "ShadowPlanSourceOrdinal")?;
    out.extend_from_slice(&row.pos.to_le_bytes());
    out.extend_from_slice(&row.len.to_le_bytes());
    out.extend_from_slice(&row.midi.to_le_bytes());
    out.push(u8::from(row.retained));
    append_opt_u8(out, row.final_staff);
    out.push(row.removal_reason.map_or(0, ExistingSourceSelectionRemovalReason::discriminator));
    append_usize(out, row.decision_count, "ShadowPlanDecisionCount")?;
    Ok(())
}

fn append_candidate_audit_row(
    out: &mut Vec<u8>,
    row: &SourceSelectionCandidateAuditRow,
) -> anyhow::Result<()> {
    append_usize(out, row.candidate_id, "SourceSelectionCandidateId")?;
    append_str(out, &row.origin_stage, "SourceSelectionOriginStage")?;
    append_str(out, &row.profile_name, "SourceSelectionProfileName")?;
    out.extend_from_slice(&row.span_limit.to_le_bytes());
    out.extend_from_slice(&row.jump_limit.to_le_bytes());
    out.push(u8::from(row.smooth_jumps));
    out.extend_from_slice(&row.left_high_soft_limit.to_le_bytes());
    out.extend_from_slice(&row.right_low_soft_limit.to_le_bytes());
    append_usize(out, row.hotspot_source_issue_count, "HotspotSourceIssueCount")?;
    append_usize(
        out,
        row.hotspot_group_count_before_truncation,
        "HotspotGroupCountBeforeTruncation",
    )?;
    append_usize(out, row.hotspot_requested_limit, "HotspotRequestedLimit")?;
    append_usize(out, row.hotspot_truncated_count, "HotspotTruncatedCount")?;
    append_str(out, &row.hotspot_fingerprint_sha256, "HotspotFingerprint")?;
    out.extend_from_slice(&row.canonical_source_retention_ppm.to_le_bytes());
    append_str(out, &row.shadow_plan_fingerprint_sha256, "ShadowPlanFingerprint")?;
    let evaluation = &row.evaluation;
    append_usize(out, evaluation.blocker_count, "EvaluationBlockerCount")?;
    append_usize(out, evaluation.critical_count, "EvaluationCriticalCount")?;
    out.extend_from_slice(&evaluation.max_critical_hardness.to_le_bytes());
    append_usize(out, evaluation.warning_count, "EvaluationWarningCount")?;
    append_usize(out, evaluation.hint_count, "EvaluationHintCount")?;
    match evaluation.arrangement_visible_retention_milli_percent {
        Some(value) => {
            out.push(1);
            out.extend_from_slice(&value.to_le_bytes());
        }
        None => out.push(0),
    }
    append_usize(
        out,
        evaluation.human_potential_notes_moved,
        "HumanPotentialNotesMoved",
    )?;
    Ok(())
}

fn append_solver_frontier_row(
    out: &mut Vec<u8>,
    row: &SourceSelectionSolverFrontierRow,
) -> anyhow::Result<()> {
    append_usize(out, row.solver_iteration, "SolverIteration")?;
    append_str(out, &row.solver_profile_name, "SolverProfileName")?;
    append_str(out, &row.solver_profile_lineage, "SolverProfileLineage")?;
    append_usize(
        out,
        row.source_selection_candidate_id,
        "SolverSourceSelectionCandidateId",
    )?;
    append_usize(
        out,
        row.evaluation.blocker_count,
        "SolverEvaluationBlockerCount",
    )?;
    append_usize(
        out,
        row.evaluation.critical_count,
        "SolverEvaluationCriticalCount",
    )?;
    out.extend_from_slice(&row.evaluation.max_critical_hardness.to_le_bytes());
    append_usize(
        out,
        row.evaluation.warning_count,
        "SolverEvaluationWarningCount",
    )?;
    append_usize(
        out,
        row.evaluation.hint_count,
        "SolverEvaluationHintCount",
    )?;
    match row.evaluation.arrangement_visible_retention_milli_percent {
        Some(value) => {
            out.push(1);
            out.extend_from_slice(&value.to_le_bytes());
        }
        None => out.push(0),
    }
    append_usize(
        out,
        row.evaluation.human_potential_notes_moved,
        "SolverEvaluationHumanPotentialNotesMoved",
    )?;
    append_usize(
        out,
        row.human_potential_notes_moved,
        "SolverHumanPotentialNotesMoved",
    )?;
    append_usize(out, row.blockers, "SolverBlockers")?;
    append_usize(out, row.warnings, "SolverWarnings")?;
    append_usize(out, row.hints, "SolverHints")?;
    Ok(())
}

pub(crate) fn solver_frontier_state_fingerprint_sha256(
    row: &SourceSelectionSolverFrontierRow,
) -> String {
    let mut bytes = b"ETUSCHLOPF_SOURCE_SELECTION_SOLVER_STATE_V3\0".to_vec();
    append_solver_frontier_row(&mut bytes, row)
        .expect("solver frontier row values must fit deterministic audit encoding");
    sha256::hex(&bytes)
}

fn append_opt_u8(out: &mut Vec<u8>, value: Option<u8>) {
    match value {
        Some(value) => {
            out.push(1);
            out.push(value);
        }
        None => out.push(0),
    }
}

fn append_opt_usize(out: &mut Vec<u8>, value: Option<usize>, label: &str) -> anyhow::Result<()> {
    match value {
        Some(value) => {
            out.push(1);
            append_usize(out, value, label)?;
        }
        None => out.push(0),
    }
    Ok(())
}

fn append_str(out: &mut Vec<u8>, value: &str, label: &str) -> anyhow::Result<()> {
    append_usize(out, value.len(), label)?;
    out.extend_from_slice(value.as_bytes());
    Ok(())
}


fn append_retained_identity(
    out: &mut Vec<u8>,
    event: &RetainedSourceSelectionIdentity,
) -> anyhow::Result<()> {
    append_usize(out, event.source_ordinal, "RetainedSourceEvent-Ordinalitaet")?;
    out.extend_from_slice(&event.pos.to_le_bytes());
    out.extend_from_slice(&event.len.to_le_bytes());
    out.extend_from_slice(&event.midi.to_le_bytes());
    Ok(())
}

fn append_productive_retained_identity(
    out: &mut Vec<u8>,
    event: &RetainedSourceSelectionIdentity,
) -> anyhow::Result<()> {
    append_retained_identity(out, event)?;
    out.push(event.final_staff);
    Ok(())
}

fn append_usize(out: &mut Vec<u8>, value: usize, label: &str) -> anyhow::Result<()> {
    let value = u64::try_from(value).map_err(|_| anyhow!("{label} ueberlaeuft u64"))?;
    out.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(source_ordinal: usize, pos: i64, len: i64, midi: i32) -> CanonicalSourceEvent {
        CanonicalSourceEvent {
            source_ordinal,
            pos,
            len,
            midi,
        }
    }






    #[test]
    fn full_causal_shadow_plan_replays_exact_productive_partition() {
        let canonical = [event(0, 0, 48, 55), event(1, 96, 48, 67)];
        let retained_events = [RetainedSourceSelectionIdentity {
            source_ordinal: 1,
            pos: 96,
            len: 48,
            midi: 67,
            final_staff: 2,
        }];
        let reasons = [RemovedSourceSelectionReason {
            source_ordinal: 0,
            reason: ExistingSourceSelectionRemovalReason::HotspotJump,
        }];
        let decisions = vec![
            SourceSelectionDecisionEvidence {
                source_ordinal: 0,
                pass: SourceSelectionDecisionPass::InitialPitchStaff,
                pass_iteration: 0,
                action: SourceSelectionDecisionAction::InitialStaff,
                pos: 0,
                len: 48,
                midi: 55,
                before_staff: None,
                after_staff: Some(2),
                witness_a_source_ordinal: None,
                witness_b_source_ordinal: None,
                observed_primary: 55,
                observed_secondary: 0,
                threshold_primary: 60,
                threshold_secondary: 0,
                focus_score: 0,
                tie_break_code: 0,
                removal_reason: None,
            },
            SourceSelectionDecisionEvidence {
                source_ordinal: 1,
                pass: SourceSelectionDecisionPass::InitialPitchStaff,
                pass_iteration: 0,
                action: SourceSelectionDecisionAction::InitialStaff,
                pos: 96,
                len: 48,
                midi: 67,
                before_staff: None,
                after_staff: Some(1),
                witness_a_source_ordinal: None,
                witness_b_source_ordinal: None,
                observed_primary: 67,
                observed_secondary: 0,
                threshold_primary: 60,
                threshold_secondary: 0,
                focus_score: 0,
                tie_break_code: 1,
                removal_reason: None,
            },
            SourceSelectionDecisionEvidence {
                source_ordinal: 1,
                pass: SourceSelectionDecisionPass::PianoHandSplit,
                pass_iteration: 0,
                action: SourceSelectionDecisionAction::StaffMove,
                pos: 96,
                len: 48,
                midi: 67,
                before_staff: Some(1),
                after_staff: Some(2),
                witness_a_source_ordinal: Some(0),
                witness_b_source_ordinal: Some(1),
                observed_primary: 12,
                observed_secondary: 1,
                threshold_primary: 10,
                threshold_secondary: 2,
                focus_score: 0,
                tie_break_code: 1,
                removal_reason: None,
            },
            SourceSelectionDecisionEvidence {
                source_ordinal: 0,
                pass: SourceSelectionDecisionPass::HotspotJump,
                pass_iteration: 0,
                action: SourceSelectionDecisionAction::Remove,
                pos: 0,
                len: 48,
                midi: 55,
                before_staff: Some(2),
                after_staff: None,
                witness_a_source_ordinal: Some(0),
                witness_b_source_ordinal: Some(1),
                observed_primary: 12,
                observed_secondary: 96,
                threshold_primary: 10,
                threshold_secondary: 144,
                focus_score: 100,
                tie_break_code: 1,
                removal_reason: Some(ExistingSourceSelectionRemovalReason::HotspotJump),
            },
        ];
        let projection = project_source_selection_full_causal_audit(
            &canonical,
            &retained_events,
            &reasons,
            decisions,
        )
        .expect("full causal replay");
        assert_eq!(projection.canonical_source_retention_ppm, 500_000);
        assert_eq!(projection.staff_move_count, 1);
        assert_eq!(projection.removal_decision_count, 1);
        assert_eq!(projection.shadow_rows.len(), 2);
        assert!(!projection.shadow_rows[0].retained);
        assert_eq!(projection.shadow_rows[1].final_staff, Some(2));

        let source_candidate = SourceSelectionCandidateAuditRow {
            candidate_id: 7,
            origin_stage: "test".to_string(),
            profile_name: "test".to_string(),
            span_limit: 12,
            jump_limit: 18,
            smooth_jumps: true,
            left_high_soft_limit: 72,
            right_low_soft_limit: 48,
            hotspot_source_issue_count: 0,
            hotspot_group_count_before_truncation: 0,
            hotspot_requested_limit: 0,
            hotspot_truncated_count: 0,
            hotspot_fingerprint_sha256: "hotspot".to_string(),
            canonical_source_retention_ppm: 1_000_000,
            shadow_plan_fingerprint_sha256: "shadow".to_string(),
            evaluation: SourceSelectionCandidateEvaluation::default(),
        };
        let frontier_row = SourceSelectionSolverFrontierRow {
            solver_iteration: 773,
            solver_profile_name: "winner".to_string(),
            solver_profile_lineage: "[]".to_string(),
            source_selection_candidate_id: 7,
            evaluation: SourceSelectionCandidateEvaluation::default(),
            human_potential_notes_moved: 0,
            blockers: 0,
            warnings: 0,
            hints: 0,
        };
        let mut later_state_same_iteration = frontier_row.clone();
        later_state_same_iteration.hints = 1;
        later_state_same_iteration.solver_profile_name = "winner_after_in_place_stage".to_string();

        let solver_projection = project_source_selection_solver_causality(
            vec![source_candidate.clone()],
            vec![
                frontier_row.clone(),
                frontier_row.clone(),
                later_state_same_iteration.clone(),
            ],
            Vec::new(),
            7,
            later_state_same_iteration.clone(),
        )
        .expect("same iteration may carry multiple distinct solver states");
        assert_eq!(solver_projection.solver_frontier.len(), 2);
        assert_eq!(solver_projection.final_solver_iteration, 773);
        assert_eq!(
            solver_projection.final_solver_state_fingerprint_sha256,
            solver_frontier_state_fingerprint_sha256(&later_state_same_iteration)
        );

        let unknown_final_state = SourceSelectionSolverFrontierRow {
            solver_profile_name: "not_in_frontier".to_string(),
            ..later_state_same_iteration
        };
        let error = project_source_selection_solver_causality(
            vec![source_candidate],
            vec![frontier_row],
            Vec::new(),
            7,
            unknown_final_state,
        )
        .expect_err("exact final solver state must be present in decision frontier");
        assert!(error.to_string().contains("finaler Solver-Zustand"));
    }

    #[test]
    fn full_causal_shadow_plan_rejects_staff_state_break() {
        let canonical = [event(0, 0, 48, 67)];
        let retained_events = [RetainedSourceSelectionIdentity {
            source_ordinal: 0,
            pos: 0,
            len: 48,
            midi: 67,
            final_staff: 2,
        }];
        let decisions = vec![
            SourceSelectionDecisionEvidence {
                source_ordinal: 0,
                pass: SourceSelectionDecisionPass::InitialPitchStaff,
                pass_iteration: 0,
                action: SourceSelectionDecisionAction::InitialStaff,
                pos: 0,
                len: 48,
                midi: 67,
                before_staff: None,
                after_staff: Some(1),
                witness_a_source_ordinal: None,
                witness_b_source_ordinal: None,
                observed_primary: 67,
                observed_secondary: 0,
                threshold_primary: 60,
                threshold_secondary: 0,
                focus_score: 0,
                tie_break_code: 1,
                removal_reason: None,
            },
            SourceSelectionDecisionEvidence {
                source_ordinal: 0,
                pass: SourceSelectionDecisionPass::PianoHandSplit,
                pass_iteration: 0,
                action: SourceSelectionDecisionAction::StaffMove,
                pos: 0,
                len: 48,
                midi: 67,
                before_staff: Some(2),
                after_staff: Some(1),
                witness_a_source_ordinal: None,
                witness_b_source_ordinal: None,
                observed_primary: 0,
                observed_secondary: 0,
                threshold_primary: 0,
                threshold_secondary: 0,
                focus_score: 0,
                tie_break_code: 0,
                removal_reason: None,
            },
        ];
        let error = project_source_selection_full_causal_audit(
            &canonical,
            &retained_events,
            &[],
            decisions,
        )
        .expect_err("staff state break must fail closed");
        assert!(error.to_string().contains("Staff-Move-Statebruch"));
    }



}
