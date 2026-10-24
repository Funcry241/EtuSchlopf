#[path = "bridge_voice_plan_guard.rs"]
mod bridge_voice_plan_guard;
use bridge_voice_plan_guard::BoundBridgeVoiceGuard;
#[path = "rest_bridge_extension_course.rs"]
mod rest_bridge_extension_course;
pub(crate) use rest_bridge_extension_course::RestBridgeExtensionOccurrenceEvidence;
use rest_bridge_extension_course::{BoundRestBridgeWithdrawal, RestBridgeExtensionCourse};
#[path = "final_chord_course_capture.rs"]
mod course_capture;
#[path = "original_beam_voice_alignment.rs"]
mod original_beam_voice_alignment;

#[cfg(test)]
use super::authority_primitives::yes_no;
use super::authority_primitives::fingerprint;
use std::io;
use crate::master_state_identity::ExactTransformationReceipt;
use super::{
    LocatedStructuralRepairIntent, StructuralBeamState, StructuralCarrierRole,
    StructuralEvent, StructuralRepairIntent, StructuralStaffRelation,
    StructuralVoiceRelation,
};
use std::fmt::Write as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PlannedLane {
    MainVoiceRun,
    HiddenCarrierAuxiliary,
    OverlapRestAuxiliary,
}

impl PlannedLane {
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AuthorizedPlannedLane {
    MainVoiceRun,
    HiddenCarrierAuxiliary,
    OverlapRestAuxiliary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AuthorizedSerializationPlanRow {
    pub(super) source_event_ordinal: usize,
    pub(super) original_cursor_before: i64,
    pub(super) original_cursor_after: i64,
    pub(super) planned_chord_member: bool,
    pub(super) planned_lane: AuthorizedPlannedLane,
    pub(super) planned_xml_order: usize,
    pub(super) planned_cursor_before: i64,
    pub(super) planned_cursor_after: i64,
    pub(super) planned_primary_beam: StructuralBeamState,
    pub(super) planned_secondary_beam: StructuralBeamState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AuthorizedSerializationPlan {
    pub(super) rows: Vec<AuthorizedSerializationPlanRow>,
    pub(super) full_backups_to_measure_start: usize,
    pub(super) partial_backups_to_nonzero: usize,
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct SerializationPlanRow {
    source_event_ordinal: usize,
    original_onset: i64,
    duration: i64,
    original_cursor_before: i64,
    original_cursor_after: i64,
    voice_relation: StructuralVoiceRelation,
    staff_relation: StructuralStaffRelation,
    carrier_role: StructuralCarrierRole,
    visible: bool,
    rest_like: bool,
    original_chord_member: bool,
    planned_chord_member: bool,
    planned_lane: PlannedLane,
    planned_xml_order: usize,
    planned_cursor_before: i64,
    planned_cursor_after: i64,
    planned_primary_beam: StructuralBeamState,
    planned_secondary_beam: StructuralBeamState,
}

#[derive(Debug, Clone, Default)]
pub struct IdentityFreeSemanticEventEquivalenceAudit {
    #[cfg(test)]
    source_stage: String,
    #[cfg(test)]
    structural_view_count: usize,
    repair_intent_count: usize,
    unique_repair_intent: bool,
    source_identity_discovered: String,
    source_nonce_bound: bool,
    source_event_mapping_complete: bool,
    source_event_ordinals_unique: bool,
    plan_rows: Vec<SerializationPlanRow>,
    main_lane_events: usize,
    auxiliary_lanes: usize,
    full_backups_to_measure_start: usize,
    partial_backups_to_nonzero: usize,
    cursor_neutral_topology_feasible: bool,
    target_members_contiguous: bool,
    hidden_carrier_between_target_members: bool,
    onset_stable: bool,
    duration_stable: bool,
    voice_relation_stable: bool,
    staff_relation_stable: bool,
    visibility_stable: bool,
    rest_role_stable: bool,
    chord_state_stable: bool,
    required_chord_normalizations: usize,
    proven_chord_normalizations: usize,
    unproven_chord_normalizations: usize,
    artificial_chord_additions: usize,
    semantic_complete_event_delta_rows: usize,
    serialization_encoding_delta_rows: usize,
    sounding_identity_delta_rows: usize,
    semantic_event_invariant_green: bool,
    encoding_normalization_proof_green: bool,
    detector_identity_fields_exposed: bool,
    planner_performed_musical_selection: bool,
    writer_token_used: bool,
    write_produced: bool,
    serialization_plan_feasible: bool,
    blocking_invariant: String,
    #[cfg(test)]
    verdict: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetamorphicPlanEvidenceSnapshot {
    pub repair_intent_count: usize,
    pub abstract_plan_fingerprint: String,
    pub semantic_complete_event_delta_rows: usize,
    pub serialization_encoding_delta_rows: usize,
    pub sounding_identity_delta_rows: usize,
    pub required_chord_normalizations: usize,
    pub proven_chord_normalizations: usize,
    pub unproven_chord_normalizations: usize,
    pub artificial_chord_additions: usize,
    pub detector_identity_fields_exposed: bool,
    pub writer_token_used: bool,
    pub write_produced: bool,
    pub serialization_plan_feasible: bool,
    pub blocking_invariant: String,
}

impl IdentityFreeSemanticEventEquivalenceAudit {
    pub fn authorizes_token_origin(&self) -> bool {
        self.unique_repair_intent
            && self.source_event_mapping_complete
            && self.source_event_ordinals_unique
            && self.cursor_neutral_topology_feasible
            && self.partial_backups_to_nonzero == 0
            && self.target_members_contiguous
            && !self.hidden_carrier_between_target_members
            && self.semantic_event_invariant_green
            && self.encoding_normalization_proof_green
            && self.serialization_plan_feasible
            && !self.detector_identity_fields_exposed
            && !self.planner_performed_musical_selection
            && !self.writer_token_used
            && !self.write_produced
    }

    pub(super) fn authorized_serialization_plan(&self) -> Option<AuthorizedSerializationPlan> {
        if !self.authorizes_token_origin() {
            return None;
        }
        let rows = self
            .plan_rows
            .iter()
            .map(|row| AuthorizedSerializationPlanRow {
                source_event_ordinal: row.source_event_ordinal,
                original_cursor_before: row.original_cursor_before,
                original_cursor_after: row.original_cursor_after,
                planned_chord_member: row.planned_chord_member,
                planned_lane: match row.planned_lane {
                    PlannedLane::MainVoiceRun => AuthorizedPlannedLane::MainVoiceRun,
                    PlannedLane::HiddenCarrierAuxiliary => {
                        AuthorizedPlannedLane::HiddenCarrierAuxiliary
                    }
                    PlannedLane::OverlapRestAuxiliary => {
                        AuthorizedPlannedLane::OverlapRestAuxiliary
                    }
                },
                planned_xml_order: row.planned_xml_order,
                planned_cursor_before: row.planned_cursor_before,
                planned_cursor_after: row.planned_cursor_after,
                planned_primary_beam: row.planned_primary_beam,
                planned_secondary_beam: row.planned_secondary_beam,
            })
            .collect();
        Some(AuthorizedSerializationPlan {
            rows,
            full_backups_to_measure_start: self.full_backups_to_measure_start,
            partial_backups_to_nonzero: self.partial_backups_to_nonzero,
        })
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "IdentityFreeSemanticEventEquivalenceAudit v{}: Project {}; SourceStage {}; StructuralViews {}; RepairIntents {}; UniqueIntent {}; SourceIdentityDiscovered {}; SourceNonceBound {}; SourceEventMappingComplete {}; SourceEventOrdinalsUnique {}; PlanRows {}; MainLaneEvents {}; AuxiliaryLanes {}; FullBackupsToMeasureStart {}; PartialBackupsToNonzero {}; CursorNeutralTopologyFeasible {}; TargetMembersContiguous {}; HiddenCarrierBetweenTargetMembers {}; OnsetStable {}; DurationStable {}; VoiceRelationStable {}; StaffRelationStable {}; VisibilityStable {}; RestRoleStable {}; ChordStateStable {}; RequiredChordNormalizations {}; ProvenChordNormalizations {}; UnprovenChordNormalizations {}; ArtificialChordAdditions {}; SemanticCompleteEventDeltaRows {}; SerializationEncodingDeltaRows {}; SoundingIdentityDeltaRows {}; SemanticEventInvariantGreen {}; EncodingNormalizationProofGreen {}; DetectorIdentityFieldsExposed {}; PlannerMusicalSelection {}; WriterTokenUsed {}; SerializationPlanFeasible {}; BlockingInvariant {}; WriteProduced nein; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            self.source_stage,
            self.structural_view_count,
            self.repair_intent_count,
            yes_no(self.unique_repair_intent),
            self.source_identity_discovered,
            yes_no(self.source_nonce_bound),
            yes_no(self.source_event_mapping_complete),
            yes_no(self.source_event_ordinals_unique),
            self.plan_rows.len(),
            self.main_lane_events,
            self.auxiliary_lanes,
            self.full_backups_to_measure_start,
            self.partial_backups_to_nonzero,
            yes_no(self.cursor_neutral_topology_feasible),
            yes_no(self.target_members_contiguous),
            yes_no(self.hidden_carrier_between_target_members),
            yes_no(self.onset_stable),
            yes_no(self.duration_stable),
            yes_no(self.voice_relation_stable),
            yes_no(self.staff_relation_stable),
            yes_no(self.visibility_stable),
            yes_no(self.rest_role_stable),
            yes_no(self.chord_state_stable),
            self.required_chord_normalizations,
            self.proven_chord_normalizations,
            self.unproven_chord_normalizations,
            self.artificial_chord_additions,
            self.semantic_complete_event_delta_rows,
            self.serialization_encoding_delta_rows,
            self.sounding_identity_delta_rows,
            yes_no(self.semantic_event_invariant_green),
            yes_no(self.encoding_normalization_proof_green),
            yes_no(self.detector_identity_fields_exposed),
            yes_no(self.planner_performed_musical_selection),
            yes_no(self.writer_token_used),
            yes_no(self.serialization_plan_feasible),
            self.blocking_invariant,
            self.verdict,
        )
    }


    pub fn metamorphic_evidence_snapshot(&self) -> MetamorphicPlanEvidenceSnapshot {
        let mut abstract_plan_fingerprint = String::new();
        for row in &self.plan_rows {
            let _ = write!(
                abstract_plan_fingerprint,
                "{}:{}:{:?}:{:?}:{:?}:{}:{}:{}>{}:{:?}:{}:{}>{}:{}:{:?}/{:?}|",
                row.original_onset,
                row.duration,
                row.voice_relation,
                row.staff_relation,
                row.carrier_role,
                row.visible,
                row.rest_like,
                row.original_chord_member,
                row.planned_chord_member,
                row.planned_lane,
                row.planned_xml_order,
                row.planned_cursor_before,
                row.planned_cursor_after,
                row.duration,
                row.planned_primary_beam,
                row.planned_secondary_beam,
            );
        }
        MetamorphicPlanEvidenceSnapshot {
            repair_intent_count: self.repair_intent_count,
            abstract_plan_fingerprint,
            semantic_complete_event_delta_rows: self.semantic_complete_event_delta_rows,
            serialization_encoding_delta_rows: self.serialization_encoding_delta_rows,
            sounding_identity_delta_rows: self.sounding_identity_delta_rows,
            required_chord_normalizations: self.required_chord_normalizations,
            proven_chord_normalizations: self.proven_chord_normalizations,
            unproven_chord_normalizations: self.unproven_chord_normalizations,
            artificial_chord_additions: self.artificial_chord_additions,
            detector_identity_fields_exposed: self.detector_identity_fields_exposed,
            writer_token_used: self.writer_token_used,
            write_produced: self.write_produced,
            serialization_plan_feasible: self.serialization_plan_feasible,
            blocking_invariant: self.blocking_invariant.clone(),
        }
    }
}

pub fn audit_identity_free_semantic_event_equivalence(
    intents: &[LocatedStructuralRepairIntent],
    _structural_view_count: usize,
    _source_stage: &str,
) -> IdentityFreeSemanticEventEquivalenceAudit {
    let mut audit = IdentityFreeSemanticEventEquivalenceAudit {
        #[cfg(test)]
        source_stage: _source_stage.to_string(),
        #[cfg(test)]
        structural_view_count: _structural_view_count,
        repair_intent_count: intents.len(),
        unique_repair_intent: intents.len() == 1,
        detector_identity_fields_exposed: false,
        planner_performed_musical_selection: false,
        writer_token_used: false,
        write_produced: false,
        blocking_invariant: "repair_intent_missing_or_ambiguous".to_string(),
        #[cfg(test)]
        verdict: "identity_free_semantic_event_equivalence_not_feasible".to_string(),
        ..IdentityFreeSemanticEventEquivalenceAudit::default()
    };

    let Some(located_intent) = intents.first() else {
        return audit;
    };
    if intents.len() != 1
        || located_intent.intent != StructuralRepairIntent::CompleteTailBeatBeam
    {
        return audit;
    }

    let view = &located_intent.located_view.view;
    let locator = &located_intent.located_view.locator;
    audit.source_identity_discovered = locator.source_identity.clone();
    audit.source_nonce_bound = locator.source_nonce != 0;
    audit.source_event_mapping_complete = view.events().len() == locator.source_events.len();
    if !audit.source_event_mapping_complete {
        audit.blocking_invariant = "source_event_mapping_incomplete".to_string();
        return audit;
    }
    let mut source_ordinals = locator
        .source_events
        .iter()
        .map(|source| source.event_ordinal)
        .collect::<Vec<_>>();
    source_ordinals.sort_unstable();
    source_ordinals.dedup();
    audit.source_event_ordinals_unique = source_ordinals.len() == locator.source_events.len();
    if !audit.source_event_ordinals_unique {
        audit.blocking_invariant = "source_event_ordinals_not_unique".to_string();
        return audit;
    }

    let quarter = i64::from(view.metric_units_per_quarter());
    if quarter <= 0 || quarter % 4 != 0 {
        audit.blocking_invariant = "metric_grid_not_quarter_divisible".to_string();
        return audit;
    }
    let sixteenth = quarter / 4;

    let beat3_first = unique_event_index(view.events(), |event| {
        event_matches(
            event,
            0,
            sixteenth * 3,
            StructuralCarrierRole::Sounding,
            StructuralStaffRelation::SameAsAnchor,
            false,
        )
    });
    let beat3_last = unique_event_index(view.events(), |event| {
        event_matches(
            event,
            sixteenth * 3,
            sixteenth,
            StructuralCarrierRole::Sounding,
            StructuralStaffRelation::SameAsAnchor,
            false,
        )
    });
    let beat4_first = unique_event_index(view.events(), |event| {
        event_matches(
            event,
            quarter,
            sixteenth * 2,
            StructuralCarrierRole::Sounding,
            StructuralStaffRelation::SameAsAnchor,
            false,
        )
    });
    let beat4_cross = unique_event_index(view.events(), |event| {
        event_matches(
            event,
            quarter + sixteenth * 2,
            sixteenth,
            StructuralCarrierRole::Sounding,
            StructuralStaffRelation::DifferentFromAnchor,
            false,
        )
    });
    let beat4_last = unique_event_index(view.events(), |event| {
        event_matches(
            event,
            quarter + sixteenth * 3,
            sixteenth,
            StructuralCarrierRole::Sounding,
            StructuralStaffRelation::SameAsAnchor,
            true,
        )
    });
    let hidden_carrier = unique_event_index(view.events(), |event| {
        event_matches(
            event,
            quarter + sixteenth * 3,
            sixteenth,
            StructuralCarrierRole::HiddenCarrier,
            StructuralStaffRelation::SameAsAnchor,
            false,
        )
    });
    let overlap_rest = unique_event_index(view.events(), |event| {
        event_matches(
            event,
            quarter + sixteenth * 2,
            sixteenth * 2,
            StructuralCarrierRole::Rest,
            StructuralStaffRelation::SameAsAnchor,
            false,
        )
    });

    let (Some(beat3_first), Some(beat3_last), Some(beat4_first), Some(beat4_cross), Some(beat4_last), Some(hidden_carrier), Some(overlap_rest)) = (
        beat3_first,
        beat3_last,
        beat4_first,
        beat4_cross,
        beat4_last,
        hidden_carrier,
        overlap_rest,
    ) else {
        audit.blocking_invariant = "repair_intent_event_roles_not_unique".to_string();
        return audit;
    };

    let planned = [
        PlannedEvent {
            view_index: beat3_first,
            lane: PlannedLane::MainVoiceRun,
            order: 0,
            chord_member: false,
            primary_beam: StructuralBeamState::Begin,
            secondary_beam: StructuralBeamState::Absent,
        },
        PlannedEvent {
            view_index: beat3_last,
            lane: PlannedLane::MainVoiceRun,
            order: 1,
            chord_member: false,
            primary_beam: StructuralBeamState::End,
            secondary_beam: StructuralBeamState::Absent,
        },
        PlannedEvent {
            view_index: beat4_first,
            lane: PlannedLane::MainVoiceRun,
            order: 2,
            chord_member: false,
            primary_beam: StructuralBeamState::Begin,
            secondary_beam: StructuralBeamState::Absent,
        },
        PlannedEvent {
            view_index: beat4_cross,
            lane: PlannedLane::MainVoiceRun,
            order: 3,
            chord_member: false,
            primary_beam: StructuralBeamState::Continue,
            secondary_beam: StructuralBeamState::Begin,
        },
        PlannedEvent {
            view_index: beat4_last,
            lane: PlannedLane::MainVoiceRun,
            order: 4,
            chord_member: false,
            primary_beam: StructuralBeamState::End,
            secondary_beam: StructuralBeamState::End,
        },
        PlannedEvent {
            view_index: hidden_carrier,
            lane: PlannedLane::HiddenCarrierAuxiliary,
            order: 5,
            chord_member: false,
            primary_beam: StructuralBeamState::Absent,
            secondary_beam: StructuralBeamState::Absent,
        },
        PlannedEvent {
            view_index: overlap_rest,
            lane: PlannedLane::OverlapRestAuxiliary,
            order: 6,
            chord_member: false,
            primary_beam: StructuralBeamState::Absent,
            secondary_beam: StructuralBeamState::Absent,
        },
    ];

    audit.plan_rows = planned
        .iter()
        .map(|planned_event| {
            let event = view.events()[planned_event.view_index];
            let source = locator.source_events[planned_event.view_index];
            let absolute_onset = locator.window_start + i64::from(event.onset_in_window);
            SerializationPlanRow {
                source_event_ordinal: source.event_ordinal,
                original_onset: absolute_onset,
                duration: i64::from(event.duration),
                original_cursor_before: source.original_cursor_before,
                original_cursor_after: source.original_cursor_after,
                voice_relation: event.voice_relation,
                staff_relation: event.staff_relation,
                carrier_role: event.carrier_role,
                visible: event.visible,
                rest_like: event.rest_like,
                original_chord_member: event.chord_member,
                planned_chord_member: planned_event.chord_member,
                planned_lane: planned_event.lane,
                planned_xml_order: planned_event.order,
                planned_cursor_before: absolute_onset,
                planned_cursor_after: absolute_onset + i64::from(event.duration),
                planned_primary_beam: planned_event.primary_beam,
                planned_secondary_beam: planned_event.secondary_beam,
            }
        })
        .collect();
    audit.plan_rows.sort_by_key(|row| row.planned_xml_order);

    audit.main_lane_events = audit
        .plan_rows
        .iter()
        .filter(|row| row.planned_lane == PlannedLane::MainVoiceRun)
        .count();
    audit.auxiliary_lanes = 2;
    audit.full_backups_to_measure_start = 2;
    audit.partial_backups_to_nonzero = 0;
    audit.target_members_contiguous = beat4_members_are_contiguous(&audit.plan_rows);
    audit.hidden_carrier_between_target_members = false;
    audit.cursor_neutral_topology_feasible = audit.source_event_mapping_complete
        && audit.main_lane_events == 5
        && audit.auxiliary_lanes == 2
        && audit.full_backups_to_measure_start == 2
        && audit.partial_backups_to_nonzero == 0
        && audit.target_members_contiguous
        && !audit.hidden_carrier_between_target_members
        && auxiliary_lanes_end_at_same_cursor(&audit.plan_rows);

    audit.onset_stable = audit
        .plan_rows
        .iter()
        .all(|row| row.original_onset == row.planned_cursor_before);
    audit.duration_stable = audit.plan_rows.iter().all(|row| {
        row.duration == row.planned_cursor_after.saturating_sub(row.planned_cursor_before)
    });
    audit.voice_relation_stable = audit
        .plan_rows
        .iter()
        .all(|row| row.voice_relation == StructuralVoiceRelation::SameRun);
    audit.staff_relation_stable = true;
    audit.visibility_stable = true;
    audit.rest_role_stable = true;
    audit.chord_state_stable = audit
        .plan_rows
        .iter()
        .all(|row| row.original_chord_member == row.planned_chord_member);

    let encoding_proof = prove_encoding_normalizations(&audit.plan_rows);
    audit.required_chord_normalizations = encoding_proof.required_removals;
    audit.proven_chord_normalizations = encoding_proof.proven_removals;
    audit.unproven_chord_normalizations = encoding_proof.unproven_removals;
    audit.artificial_chord_additions = encoding_proof.artificial_additions;
    audit.encoding_normalization_proof_green = encoding_proof.green;

    let original_semantic_complete = audit
        .plan_rows
        .iter()
        .map(semantic_complete_original_key)
        .collect::<Vec<_>>();
    let planned_semantic_complete = audit
        .plan_rows
        .iter()
        .map(semantic_complete_planned_key)
        .collect::<Vec<_>>();
    audit.semantic_complete_event_delta_rows = sorted_multiset_delta_rows(
        sorted(original_semantic_complete),
        sorted(planned_semantic_complete),
    );

    let original_encoding = audit
        .plan_rows
        .iter()
        .map(serialization_encoding_original_key)
        .collect::<Vec<_>>();
    let planned_encoding = audit
        .plan_rows
        .iter()
        .map(serialization_encoding_planned_key)
        .collect::<Vec<_>>();
    audit.serialization_encoding_delta_rows = sorted_multiset_delta_rows(
        sorted(original_encoding),
        sorted(planned_encoding),
    );

    let original_sounding = audit
        .plan_rows
        .iter()
        .filter(|row| row.carrier_role == StructuralCarrierRole::Sounding)
        .map(sounding_identity_original_key)
        .collect::<Vec<_>>();
    let planned_sounding = audit
        .plan_rows
        .iter()
        .filter(|row| row.carrier_role == StructuralCarrierRole::Sounding)
        .map(sounding_identity_planned_key)
        .collect::<Vec<_>>();
    audit.sounding_identity_delta_rows = sorted_multiset_delta_rows(
        sorted(original_sounding),
        sorted(planned_sounding),
    );

    audit.semantic_event_invariant_green = audit.onset_stable
        && audit.duration_stable
        && audit.voice_relation_stable
        && audit.staff_relation_stable
        && audit.visibility_stable
        && audit.rest_role_stable
        && audit.semantic_complete_event_delta_rows == 0
        && audit.sounding_identity_delta_rows == 0;

    audit.serialization_plan_feasible = audit.source_event_ordinals_unique
        && audit.cursor_neutral_topology_feasible
        && audit.semantic_event_invariant_green
        && audit.encoding_normalization_proof_green;

    if audit.serialization_plan_feasible {
        audit.blocking_invariant = "none".to_string();
        #[cfg(test)]
        {
            audit.verdict =
                "identity_free_semantic_event_equivalence_proven_audit_only".to_string();
        }
    } else if audit.cursor_neutral_topology_feasible
        && audit.semantic_event_invariant_green
        && !audit.encoding_normalization_proof_green
    {
        audit.blocking_invariant = "serialization_encoding_normalization_unproven".to_string();
        #[cfg(test)]
        {
            audit.verdict =
                "identity_free_semantic_event_green_but_encoding_proof_blocked".to_string();
        }
    } else {
        audit.blocking_invariant = "semantic_event_or_cursor_invariants_incomplete".to_string();
        #[cfg(test)]
        {
            audit.verdict = "identity_free_semantic_event_equivalence_not_feasible".to_string();
        }
    }

    audit
}

#[derive(Debug, Clone, Copy)]
struct PlannedEvent {
    view_index: usize,
    lane: PlannedLane,
    order: usize,
    chord_member: bool,
    primary_beam: StructuralBeamState,
    secondary_beam: StructuralBeamState,
}

fn unique_event_index(
    events: &[StructuralEvent],
    predicate: impl Fn(&StructuralEvent) -> bool,
) -> Option<usize> {
    let matches = events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| predicate(event).then_some(index))
        .collect::<Vec<_>>();
    if matches.len() == 1 {
        Some(matches[0])
    } else {
        None
    }
}

fn event_matches(
    event: &StructuralEvent,
    onset: i64,
    duration: i64,
    carrier_role: StructuralCarrierRole,
    staff_relation: StructuralStaffRelation,
    chord_member: bool,
) -> bool {
    i64::from(event.onset_in_window) == onset
        && i64::from(event.duration) == duration
        && event.carrier_role == carrier_role
        && event.staff_relation == staff_relation
        && event.voice_relation == StructuralVoiceRelation::SameRun
        && event.chord_member == chord_member
}

fn beat4_members_are_contiguous(rows: &[SerializationPlanRow]) -> bool {
    let members = rows
        .iter()
        .filter(|row| {
            row.planned_lane == PlannedLane::MainVoiceRun
                && matches!(
                    row.planned_primary_beam,
                    StructuralBeamState::Begin
                        | StructuralBeamState::Continue
                        | StructuralBeamState::End
                )
                && row.planned_xml_order >= 2
        })
        .map(|row| row.planned_xml_order)
        .collect::<Vec<_>>();
    members == vec![2, 3, 4]
}

fn auxiliary_lanes_end_at_same_cursor(rows: &[SerializationPlanRow]) -> bool {
    let mut main_end = None;
    let mut hidden_end = None;
    let mut rest_end = None;
    for row in rows {
        match row.planned_lane {
            PlannedLane::MainVoiceRun => main_end = Some(row.planned_cursor_after),
            PlannedLane::HiddenCarrierAuxiliary => hidden_end = Some(row.planned_cursor_after),
            PlannedLane::OverlapRestAuxiliary => rest_end = Some(row.planned_cursor_after),
        }
    }
    main_end.is_some() && main_end == hidden_end && main_end == rest_end
}

type SemanticCompleteEventKey = (
    i64,
    i64,
    StructuralVoiceRelation,
    StructuralStaffRelation,
    StructuralCarrierRole,
    bool,
    bool,
);

type SerializationEncodingKey = (
    i64,
    i64,
    StructuralVoiceRelation,
    StructuralStaffRelation,
    StructuralCarrierRole,
    bool,
    bool,
    bool,
);

type SoundingIdentityKey = (
    i64,
    i64,
    StructuralVoiceRelation,
    StructuralStaffRelation,
    StructuralCarrierRole,
    bool,
    bool,
);

#[derive(Debug, Clone, Copy, Default)]
struct EncodingNormalizationProof {
    required_removals: usize,
    proven_removals: usize,
    unproven_removals: usize,
    artificial_additions: usize,
    green: bool,
}

fn prove_encoding_normalizations(rows: &[SerializationPlanRow]) -> EncodingNormalizationProof {
    let required_removals = rows
        .iter()
        .filter(|row| row.original_chord_member && !row.planned_chord_member)
        .count();
    let proven_removals = rows
        .iter()
        .filter(|row| row.original_chord_member && !row.planned_chord_member)
        .filter(|row| chord_normalization_is_proven(row, rows))
        .count();
    let artificial_additions = rows
        .iter()
        .filter(|row| !row.original_chord_member && row.planned_chord_member)
        .count();
    let unproven_removals = required_removals.saturating_sub(proven_removals);
    let raw_delta_rows = sorted_multiset_delta_rows(
        sorted(rows.iter().map(serialization_encoding_original_key).collect()),
        sorted(rows.iter().map(serialization_encoding_planned_key).collect()),
    );
    let green = artificial_additions == 0
        && unproven_removals == 0
        && proven_removals == required_removals
        && raw_delta_rows == required_removals.saturating_mul(2);

    EncodingNormalizationProof {
        required_removals,
        proven_removals,
        unproven_removals,
        artificial_additions,
        green,
    }
}

fn chord_normalization_is_proven(
    row: &SerializationPlanRow,
    rows: &[SerializationPlanRow],
) -> bool {
    if !row.original_chord_member
        || row.planned_chord_member
        || row.carrier_role != StructuralCarrierRole::Sounding
        || !row.visible
        || row.rest_like
        || row.planned_lane != PlannedLane::MainVoiceRun
        || row.original_cursor_before != row.original_cursor_after
        || row.original_onset != row.planned_cursor_before
        || row.planned_cursor_after != row.original_onset.saturating_add(row.duration)
    {
        return false;
    }

    rows.iter().any(|carrier| {
        carrier.carrier_role == StructuralCarrierRole::HiddenCarrier
            && !carrier.visible
            && carrier.rest_like
            && !carrier.original_chord_member
            && !carrier.planned_chord_member
            && carrier.voice_relation == row.voice_relation
            && carrier.staff_relation == row.staff_relation
            && carrier.original_onset == row.original_onset
            && carrier.duration == row.duration
            && carrier.original_cursor_before == row.original_onset
            && carrier.original_cursor_after == row.original_cursor_before
            && carrier.planned_lane == PlannedLane::HiddenCarrierAuxiliary
            && carrier.planned_cursor_before == row.original_onset
            && carrier.planned_cursor_after == row.planned_cursor_after
    })
}

fn semantic_complete_original_key(row: &SerializationPlanRow) -> SemanticCompleteEventKey {
    (
        row.original_onset,
        row.duration,
        row.voice_relation,
        row.staff_relation,
        row.carrier_role,
        row.visible,
        row.rest_like,
    )
}

fn semantic_complete_planned_key(row: &SerializationPlanRow) -> SemanticCompleteEventKey {
    (
        row.planned_cursor_before,
        row.duration,
        row.voice_relation,
        row.staff_relation,
        row.carrier_role,
        row.visible,
        row.rest_like,
    )
}

fn serialization_encoding_original_key(row: &SerializationPlanRow) -> SerializationEncodingKey {
    (
        row.original_onset,
        row.duration,
        row.voice_relation,
        row.staff_relation,
        row.carrier_role,
        row.visible,
        row.rest_like,
        row.original_chord_member,
    )
}

fn serialization_encoding_planned_key(row: &SerializationPlanRow) -> SerializationEncodingKey {
    (
        row.planned_cursor_before,
        row.duration,
        row.voice_relation,
        row.staff_relation,
        row.carrier_role,
        row.visible,
        row.rest_like,
        row.planned_chord_member,
    )
}

fn sounding_identity_original_key(row: &SerializationPlanRow) -> SoundingIdentityKey {
    (
        row.original_onset,
        row.duration,
        row.voice_relation,
        row.staff_relation,
        row.carrier_role,
        row.visible,
        row.rest_like,
    )
}

fn sounding_identity_planned_key(row: &SerializationPlanRow) -> SoundingIdentityKey {
    (
        row.planned_cursor_before,
        row.duration,
        row.voice_relation,
        row.staff_relation,
        row.carrier_role,
        row.visible,
        row.rest_like,
    )
}

fn sorted<T: Ord>(mut values: Vec<T>) -> Vec<T> {
    values.sort();
    values
}

fn sorted_multiset_delta_rows<T: Ord>(before: Vec<T>, after: Vec<T>) -> usize {
    let mut before_index = 0_usize;
    let mut after_index = 0_usize;
    let mut delta = 0_usize;
    while before_index < before.len() && after_index < after.len() {
        match before[before_index].cmp(&after[after_index]) {
            std::cmp::Ordering::Less => {
                before_index += 1;
                delta += 1;
            }
            std::cmp::Ordering::Greater => {
                after_index += 1;
                delta += 1;
            }
            std::cmp::Ordering::Equal => {
                before_index += 1;
                after_index += 1;
            }
        }
    }
    delta + before.len().saturating_sub(before_index) + after.len().saturating_sub(after_index)
}



// r462: Final chord encoding is planned here, in the existing writer-free
// serialization-planning owner. Artifact sinks receive only a fully materialized
// candidate; they do not infer pitch/tie/voice/timeline repairs from write history.
#[derive(Debug, Clone)]
struct FinalChordEncodingNote {
    range: std::ops::Range<usize>,
    measure: usize,
    raw_note_index: usize,
    onset: i64,
    duration: i64,
    chord_member: bool,
    rest_like: bool,
    pitch_identity: String,
    pitch_midi: Option<i32>,
    voice: String,
    staff: String,
    tie_start: bool,
    tie_stop: bool,
    anchor_index: Option<usize>,
    orphan_after_forward: bool,
    source_event_id: Option<usize>,
    attack_id: Option<usize>,
    fragment_id: Option<usize>,
}

fn final_chord_physical_host_is_sounding(note: &FinalChordEncodingNote) -> bool {
    !note.rest_like && note.pitch_midi.is_some()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordEncodingEdit {
    start: usize,
    end: usize,
    replacement: String,
    generated_fragments: Vec<FinalChordGeneratedFragmentEditProvenance>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordGeneratedFragmentEditProvenance {
    source_note_start: usize,
    source_note_end: usize,
    fragment_ordinal: usize,
    kind: FinalChordForwardMaterializationKind,
    replacement_note_start: usize,
    replacement_note_end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalChordForwardMaterializationKind {
    TiedAnchorChain,
    AuxiliaryLane,
}

impl FinalChordForwardMaterializationKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::TiedAnchorChain => "tied_anchor_chain",
            Self::AuxiliaryLane => "auxiliary_lane",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalChordForwardMaterializationAuditRow {
    pub(crate) source_note_start: usize,
    pub(crate) source_note_end: usize,
    pub(crate) source_onset: i64,
    pub(crate) projected_sounding_onset: i64,
    pub(crate) generated_fragment_count: usize,
    pub(crate) kind: FinalChordForwardMaterializationKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalChordProjectionMaterializationKind {
    SurvivingSourceNote,
    TiedAnchorChain,
    AuxiliaryLane,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalChordProjectionProvenanceRow {
    pub(crate) source_note_start: usize,
    pub(crate) source_note_end: usize,
    pub(crate) projected_note_start: usize,
    pub(crate) projected_note_end: usize,
    pub(crate) fragment_ordinal: usize,
    pub(crate) kind: FinalChordProjectionMaterializationKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalChordProjectionResult {
    pub(crate) xml: String,
    pub(crate) provenance: Vec<FinalChordProjectionProvenanceRow>,
    // Exact transaction-local transport, not canonical identity or role authority.
    pub(crate) non_pitched_continuations: Vec<FinalChordNonPitchedContinuation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FinalChordNonPitchedCarrier {
    Note,
    Forward,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalChordNonPitchedContinuation {
    source_note_start: usize,
    source_note_end: usize,
    projected_start: usize,
    projected_end: usize,
    fragment_ordinal: usize,
    carrier: FinalChordNonPitchedCarrier,
    // The current input supplies no independently authorized rest-role origin.
    // An XML voice label must never fill this missing binding.
    presentation_role: Option<SerializedVoicePresentationRole>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordCanonicalInputBindingRow {
    fragment_id: usize,
    source_event_id: usize,
    attack_id: usize,
    source_note_start: usize,
    source_note_end: usize,
}

// Private type-state: only the canonical bind can create owner-bearing input provenance.
// Its optional original-course observation carries no role or writer authority.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CanonicalBoundFinalChordEncodingPlan {
    plan: FinalChordEncodingPlan,
    input_bindings: Vec<FinalChordCanonicalInputBindingRow>,
    certified_omitted_fragment_ids: std::collections::BTreeSet<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FinalChordTransactionOutputOccurrenceWitness {
    fragment_id: usize,
    source_event_id: usize,
    attack_id: usize,
    materialization_kind: FinalChordProjectionMaterializationKind,
    fragment_ordinal: usize,
    measure: usize,
    pitched_ordinal: usize,
}

// Transaction-local continuation evidence only. This receipt is neither persisted nor
// part of the semantic fingerprint; it cannot prove standalone persisted-artifact readback.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordTransactionLocalOutputAlphaReceipt {
    rest_bridge_extension_course: Option<RestBridgeExtensionCourse>,
    // Ephemeral occurrence-course identity, issued only by this original
    // projection and cloned by its actual transports. Arc value equality stays
    // neutral in the pre-existing semantic receipt equality below.
    phrase_origin: std::sync::Arc<()>,
    canonical_semantic_fingerprint_fnv64: u64,
    bound_source_fingerprint_fnv64: u64,
    bound_source_len: usize,
    // Drift validation only. This voice-neutral topology fingerprint is not an
    // identity token and does not establish projection-epoch provenance.
    output_topology_fingerprint_fnv64: u64,
    occurrences: Vec<FinalChordTransactionOutputOccurrenceWitness>,
    certified_omitted_fragment_ids: std::collections::BTreeSet<usize>,
    original_beam_voice_groups: Vec<original_beam_voice_alignment::OriginalBeamVoiceGroup>,
}

// Private transaction-local type-state. Production code cannot construct an
// Output-Alpha receipt independently of the canonical-bound projection that emitted it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CanonicalProjectedFinalChord {
    xml: String,
    receipt: FinalChordTransactionLocalOutputAlphaReceipt,
}

// Productive epoch-2 type state. The receipt is created only by the canonical bind +
// FinalChord projection and remains inseparable from the exact projected XML bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalFinalChordRenderOccurrenceContinuation {
    projected: CanonicalProjectedFinalChord,
}

impl CanonicalFinalChordRenderOccurrenceContinuation {
    pub(crate) fn xml(&self) -> &str {
        &self.projected.xml
    }
}

// Productive epoch-3 type state. Identity is inherited from the FinalChord projection.
// The AuthorizedCursorNeutralWriter transition separately proves any permitted encoding-only
// chord normalization before this exact post-writer representation is sealed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalPdfSourceOccurrenceContinuation {
    xml: String,
    // Exact current beam surface carried by the actual private occurrence
    // transports. It is a drift guard, never group identity or music authority.
    beam_surface: OccurrenceBeamSurface,
    receipt: FinalChordTransactionLocalOutputAlphaReceipt,
}


type OccurrenceBeamSurface = std::collections::BTreeMap<(usize, usize), Vec<(u8, String)>>;

fn current_occurrence_beam_surface(xml: &str) -> io::Result<OccurrenceBeamSurface> {
    let mut surface = std::collections::BTreeMap::new();
    for measure in final_chord_parse_binding_measures(xml)? {
        let mut ordinal = 0;
        for note in &measure.notes {
            if note.rest_like || note.pitch_midi.is_none() { continue; }
            let document = roxmltree::Document::parse(&xml[note.range.clone()])
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
            let mut levels = std::collections::BTreeMap::new();
            for beam in document.root_element().children().filter(|node| node.has_tag_name("beam")) {
                let level = beam.attribute("number").unwrap_or("1").parse::<u8>()
                    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
                if levels.insert(level, beam.text().unwrap_or_default().trim().to_owned()).is_some() {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "duplicate beam level in actual occurrence course"));
                }
            }
            surface.insert((measure.measure, ordinal), levels.into_iter().collect());
            ordinal += 1;
        }
    }
    Ok(surface)
}

/// Opaque identity inherited from the actual canonical projection receipt.
/// No XML scalar, musical property or audit row can construct this key.
#[derive(Debug, Clone)]
pub(super) struct PhraseEndpointOccurrence {
    origin: std::sync::Arc<()>,
    fragment_id: usize,
    source_event_id: usize,
    attack_id: usize,
    fragment_ordinal: usize,
}

impl PartialEq for PhraseEndpointOccurrence {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.origin, &other.origin)
            && self.fragment_id == other.fragment_id
            && self.source_event_id == other.source_event_id
            && self.attack_id == other.attack_id
            && self.fragment_ordinal == other.fragment_ordinal
    }
}
impl Eq for PhraseEndpointOccurrence {}

impl FinalPdfSourceOccurrenceContinuation {
    fn from_original_transport(xml: String, receipt: FinalChordTransactionLocalOutputAlphaReceipt) -> io::Result<Self> {
        let mut receipt = receipt;
        if let Some(course) = &mut receipt.rest_bridge_extension_course {
            let before = course.current_xml().to_owned();
            course.through(&before, &xml)?;
        }
        let beam_surface = current_occurrence_beam_surface(&xml)?;
        Ok(Self { xml, beam_surface, receipt })
    }

    fn has_authentic_beam_surface(&self) -> io::Result<bool> {
        Ok(self.beam_surface == current_occurrence_beam_surface(&self.xml)?)
    }

    pub(super) fn bind_phrase_endpoint(
        &self, range: std::ops::Range<usize>,
    ) -> anyhow::Result<PhraseEndpointOccurrence> {
        let positions = self.phrase_occurrence_positions()?;
        let matching = positions.iter().filter(|(_, candidate)| *candidate == range)
            .collect::<Vec<_>>();
        anyhow::ensure!(matching.len() == 1, "phrase endpoint lacks one original occurrence");
        Ok(matching[0].0.clone())
    }

    pub(super) fn carry_phrase_endpoint(
        &self, key: &PhraseEndpointOccurrence,
    ) -> anyhow::Result<std::ops::Range<usize>> {
        let positions = self.phrase_occurrence_positions()?;
        let matching = positions.iter().filter(|(candidate, _)| candidate == key)
            .collect::<Vec<_>>();
        anyhow::ensure!(matching.len() == 1, "phrase endpoint occurrence was lost or duplicated");
        Ok(matching[0].1.clone())
    }

    fn phrase_occurrence_positions(&self) -> anyhow::Result<Vec<(PhraseEndpointOccurrence, std::ops::Range<usize>)>> {
        let document = roxmltree::Document::parse(&self.xml)?;
        let parts = document.root_element().children().filter(|n| n.has_tag_name("part")).collect::<Vec<_>>();
        // The canonical receipt has one piano-part coordinate domain. Do not
        // infer a part discriminator that its original owner did not supply.
        anyhow::ensure!(parts.len() == 1, "phrase occurrence part domain is unsupported");
        let mut result = Vec::new();
        for measure in final_chord_parse_binding_measures(&self.xml)? {
            let mut pitched_ordinal = 0;
            for note in &measure.notes {
                if note.rest_like || note.pitch_midi.is_none() { continue; }
                let rows = self.receipt.occurrences.iter().filter(|row|
                    row.measure == measure.measure && row.pitched_ordinal == pitched_ordinal).collect::<Vec<_>>();
                anyhow::ensure!(rows.len() == 1, "phrase endpoint receipt coverage is not unique");
                let row = rows[0];
                result.push((PhraseEndpointOccurrence { origin: self.receipt.phrase_origin.clone(), fragment_id: row.fragment_id,
                    source_event_id: row.source_event_id, attack_id: row.attack_id,
                    fragment_ordinal: row.fragment_ordinal }, note.range.clone()));
                pitched_ordinal += 1;
            }
        }
        Ok(result)
    }

    pub(crate) fn xml(&self) -> &str {
        &self.xml
    }

    /// Read-only declassification for audit consumers. The private transaction receipt
    /// remains authoritative and opaque; this projection exposes only the already-proven
    /// canonical occurrence identity plus the already-proven materialization class needed to
    /// explain physical-occurrence/attack mismatches without reopening the private receipt.
    /// It grants no mutation or selection authority.
    pub(crate) fn declassified_occurrence_identity_rows(
        &self,
    ) -> Vec<(usize, usize, usize, usize, u8, usize)> {
        let mut rows = self
            .receipt
            .occurrences
            .iter()
            .map(|row| (
                row.source_event_id,
                row.attack_id,
                row.measure,
                row.pitched_ordinal,
                final_chord_projection_materialization_kind_rank(row.materialization_kind),
                row.fragment_ordinal,
            ))
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| (row.2, row.3, row.0, row.1, row.4, row.5));
        rows
    }
}

#[derive(Clone)]
pub(crate) struct FinalChordEncodingPlan {
    source_fingerprint: u64,
    source_len: usize,
    edits: Vec<FinalChordEncodingEdit>,
    forward_materializations: Vec<FinalChordForwardMaterializationAuditRow>,
    visible_rest_interval_projection_authorized: bool,
    original_course_bindings: Option<Result<std::sync::Arc<course_capture::OriginalCourseBindings>, String>>,
    original_respelling_input: Option<crate::master_state_identity::OriginalFunctionalRespellingChains>,
}

impl FinalChordEncodingPlan {
    pub(crate) fn continue_original_respelling_chains(&self,
        before: &str, after: &str, receipt: Option<&ExactTransformationReceipt>,
    ) -> crate::master_state_identity::OriginalFunctionalRespellingChains {
        let received = self.original_respelling_input.clone().unwrap_or_else(||
            crate::master_state_identity::OriginalFunctionalRespellingChains::received(
                Err("Missing canonical original-course input".to_string())));
        let sources = self.original_course_bindings.as_ref()
            .ok_or_else(|| "Missing original canonical respelling binding".to_string())
            .and_then(|binding| binding.as_ref().map_err(Clone::clone))
            .map(|binding| binding.respelling_sources());
        match sources {
            Ok(sources) if before.len() == self.source_len && fingerprint(before) == self.source_fingerprint =>
                received.after_original_final_chord(&FinalChordEncodingExactMint { _sealed: () },
                    before, after, &sources, receipt),
            Ok(_) => crate::master_state_identity::OriginalFunctionalRespellingChains::received(
                Err("Original canonical respelling binding belongs to another state".to_string())),
            Err(error) => crate::master_state_identity::OriginalFunctionalRespellingChains::received(Err(error)),
        }
    }
}

// The observational sidecar is deliberately absent from the original semantic
// equality and Debug representation. It cannot affect existing plan decisions.
impl PartialEq for FinalChordEncodingPlan {
    fn eq(&self, other: &Self) -> bool {
        self.source_fingerprint == other.source_fingerprint && self.source_len == other.source_len
            && self.edits == other.edits && self.forward_materializations == other.forward_materializations
            && self.visible_rest_interval_projection_authorized == other.visible_rest_interval_projection_authorized
    }
}
impl Eq for FinalChordEncodingPlan {}
impl std::fmt::Debug for FinalChordEncodingPlan {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("FinalChordEncodingPlan")
            .field("source_fingerprint", &self.source_fingerprint).field("source_len", &self.source_len)
            .field("edits", &self.edits).field("forward_materializations", &self.forward_materializations)
            .field("visible_rest_interval_projection_authorized", &self.visible_rest_interval_projection_authorized).finish()
    }
}

impl FinalChordEncodingPlan {
    /// Returns the transaction-local forward provenance that the existing planner
    /// already knows when it removes and rematerializes a source note. These rows
    /// never select edits. The projection-provenance closure uses them only as a
    /// fail-closed cardinality/role witness for the fragments emitted by those edits.
    pub(crate) fn forward_materializations(
        &self,
    ) -> &[FinalChordForwardMaterializationAuditRow] {
        &self.forward_materializations
    }

    /// Carries one already-known source note forward through this exact plan only when
    /// FinalChordEncoding keeps that note as one concrete note fragment. The carrier
    /// is byte-topological: it never infers identity from pitch, onset, staff or voice.
    /// Deleted, split/cloned and overlapping cases fail closed. A zero-length insertion at
    /// the exact source-note start is carried as deterministic preceding byte displacement.
    pub(crate) fn carry_surviving_source_note_start(
        &self,
        source_note_start: usize,
        source_note_end: usize,
    ) -> Result<usize, &'static str> {
        if source_note_start >= source_note_end || source_note_end > self.source_len {
            return Err("invalid_source_note_range");
        }

        let mut byte_delta_before = 0_i128;
        let mut exact_replacement: Option<&FinalChordEncodingEdit> = None;
        for edit in &self.edits {
            if edit.start == source_note_start && edit.end == source_note_end {
                if exact_replacement.replace(edit).is_some() {
                    return Err("multiple_exact_replacements");
                }
                continue;
            }

            let overlaps_note = edit.start < source_note_end && edit.end > source_note_start;
            if overlaps_note {
                return Err("overlapping_edit");
            }
            if edit.start == source_note_start && edit.end == edit.start {
                // FinalChordEncoding applies edits from higher byte offsets to lower ones.
                // A zero-length insertion exactly at the known note start is therefore
                // materialized immediately before the surviving source note. It changes
                // only the note's forward byte offset, not its identity or note bytes.
                byte_delta_before += edit.replacement.len() as i128;
                continue;
            }
            if edit.start < source_note_start {
                byte_delta_before += edit.replacement.len() as i128
                    - edit.end.saturating_sub(edit.start) as i128;
            }
        }

        let replacement_note_offset = if let Some(edit) = exact_replacement {
            if edit.replacement.is_empty() {
                return Err("source_note_deleted");
            }
            if edit.replacement.matches("<note").count() != 1 {
                return Err("exact_replacement_not_single_note");
            }
            let Some(offset) = edit.replacement.find("<note") else {
                return Err("exact_replacement_missing_note");
            };
            if !edit.replacement[..offset].trim().is_empty() {
                return Err("exact_replacement_non_note_prefix");
            }
            offset as i128
        } else {
            0
        };

        let carried = source_note_start as i128 + byte_delta_before + replacement_note_offset;
        usize::try_from(carried).map_err(|_| "carried_offset_out_of_range")
    }
}


// r790: canonical FinalChord/SerializedVoice authority cutover.
// The new semantic corridor is intentionally declassified from legacy MusicXML
// voice labels. Opaque source/attack ids are join/provenance keys only; every
// semantic decision below is expressed through explicit temporal/structural facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VoiceFreeFinalChordEvidenceRow {
    pub(crate) fragment_id: usize,
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) measure: usize,
    pub(crate) raw_note_index: usize,
    pub(crate) onset: i64,
    pub(crate) duration: i64,
    pub(crate) pitch: i32,
    pub(crate) staff: u8,
    pub(crate) chord_syntax: bool,
    pub(crate) tie_start: bool,
    pub(crate) tie_stop: bool,
}

pub(crate) type RenderPrunedFinalChordOmissionKey = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FinalChordRenderOccurrenceWitness {
    pub(crate) fragment_id: usize,
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) measure: usize,
    pub(crate) pitched_ordinal: usize,
    pub(crate) representation_duration: Option<i64>,
    pub(crate) sequence_fingerprint_fnv64: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FinalChordRepresentationRole {
    RetainedChordMember,
    TiedRematerializedFragment,
    IndependentAuxiliaryFragment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SerializedVoicePresentationRole {
    UpperPrimary,
    LowerPrimary,
    UpperSecondary,
    LowerSecondary,
    UpperAuxiliary,
    LowerAuxiliary,
}

impl SerializedVoicePresentationRole {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::UpperPrimary => "upper_primary_run",
            Self::LowerPrimary => "lower_primary_run",
            Self::UpperSecondary => "upper_secondary_run",
            Self::LowerSecondary => "lower_secondary_run",
            Self::UpperAuxiliary => "upper_auxiliary_run",
            Self::LowerAuxiliary => "lower_auxiliary_run",
        }
    }

    pub(crate) const fn shadow_rest_primary_exempt(self) -> bool {
        matches!(self, Self::UpperPrimary)
    }

    pub(crate) const fn lower_dual_edge_primary(self) -> bool {
        matches!(self, Self::LowerPrimary)
    }

    const fn canonical_label_preference(self) -> &'static str {
        match self {
            Self::UpperPrimary => "1",
            Self::LowerPrimary => "2",
            Self::UpperSecondary | Self::UpperAuxiliary => "3",
            Self::LowerSecondary | Self::LowerAuxiliary => "4",
        }
    }

    const fn sort_rank(self) -> u8 {
        match self {
            Self::UpperPrimary => 0,
            Self::LowerPrimary => 1,
            Self::UpperSecondary => 2,
            Self::LowerSecondary => 3,
            Self::UpperAuxiliary => 4,
            Self::LowerAuxiliary => 5,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordCanonicalSourceLocator {
    fragment_id: usize,
    source_event_id: usize,
    attack_id: usize,
    measure: usize,
    raw_note_index: usize,
    onset: i64,
    duration: i64,
    pitch_identity: String,
    pitch_midi: i32,
    staff: String,
    chord_syntax: bool,
    tie_start: bool,
    tie_stop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordCanonicalHardSameEdge {
    left_fragment_id: usize,
    right_fragment_id: usize,
    presentation_staff: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordCanonicalFragmentTarget {
    anchor_fragment_id: usize,
    target_onset: i64,
    duration: i64,
    tie_start: bool,
    tie_stop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FinalChordCanonicalActionKind {
    RetainedChordMember {
        canonical_duration: i64,
    },
    TiedAnchorChain {
        fragments: Vec<FinalChordCanonicalFragmentTarget>,
    },
    AuxiliaryLane {
        serialization_run_id: usize,
        canonical_duration: i64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordCanonicalActionRow {
    fragment_id: usize,
    source_event_id: usize,
    attack_id: usize,
    host_source_event_id: Option<usize>,
    locator: FinalChordCanonicalSourceLocator,
    representation_role: FinalChordRepresentationRole,
    kind: FinalChordCanonicalActionKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FinalChordCanonicalMeasureInfo {
    measure: usize,
    divisions: i64,
    expected_duration: i64,
}

// Exact production evidence from the existing FinalChord byte projector. Opaque
// ids establish provenance; they never determine overlap or presentation roles.
#[derive(Debug, Clone)]
struct FinalChordWrittenOccurrence {
    source_event_id: usize,
    fragment_id: usize,
    attack_id: usize,
    measure: usize,
    onset: i64,
    end: i64,
    staff: u8,
    pitch_midi: i32,
    chord_anchor_start: usize,
    chord_anchor_onset: i64,
    chord_anchor_end: i64,
    has_pitched_chord_anchor: bool,
    // Exact projected byte start of this occurrence's own note element. It is
    // carried from the projector provenance row; it is never matched by value.
    projected_note_start: usize,
}

#[derive(Debug, Clone)]
struct FinalChordWrittenProjectionEvidence {
    original_xml: std::sync::Arc<str>,
    xml: std::sync::Arc<str>,
    occurrences: std::sync::Arc<[FinalChordWrittenOccurrence]>,
}

#[derive(Debug, Clone)]
pub(crate) struct CanonicalFinalChordStructuralPlan {
    rest_bridge_main_state: Option<(usize, String)>,
    rest_bridge_extension_course: Option<RestBridgeExtensionCourse>,
    rest_bridge_withdrawal: Option<BoundRestBridgeWithdrawal>,
    bridge_voice_guard: Option<BoundBridgeVoiceGuard>,
    authority_bound: bool,
    original_respelling_input: Option<crate::master_state_identity::OriginalFunctionalRespellingChains>,
    original_beam_voice_groups: Vec<original_beam_voice_alignment::OriginalBeamVoiceGroup>,
    written_projection: Option<FinalChordWrittenProjectionEvidence>,
    source_locators: std::sync::Arc<[FinalChordCanonicalSourceLocator]>,
    render_occurrence_witnesses: std::sync::Arc<[FinalChordRenderOccurrenceWitness]>,
    actions: std::sync::Arc<[FinalChordCanonicalActionRow]>,
    hard_same_edges: std::sync::Arc<[FinalChordCanonicalHardSameEdge]>,
    measures: std::sync::Arc<[FinalChordCanonicalMeasureInfo]>,
    semantic_fingerprint_fnv64: u64,
    unresolved_continuity_decisions: usize,
    unresolved_fragment_roles: usize,
    unresolved_serialization_runs: usize,
    unresolved_presentation_roles: usize,
    unresolved_constraint_edges: usize,
}

impl Default for CanonicalFinalChordStructuralPlan {
    fn default() -> Self {
        Self {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: false,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(Vec::<FinalChordCanonicalSourceLocator>::new()),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(Vec::<FinalChordCanonicalActionRow>::new()),
            hard_same_edges: std::sync::Arc::from(Vec::<FinalChordCanonicalHardSameEdge>::new()),
            measures: std::sync::Arc::from(Vec::<FinalChordCanonicalMeasureInfo>::new()),
            semantic_fingerprint_fnv64: 0,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        }
    }
}

impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn bind_original_respelling_input(&mut self,
        original: crate::master_state_identity::OriginalFunctionalRespellingChains) {
        self.original_respelling_input = Some(original);
    }

    pub(crate) fn validate_written_projection(&self, xml: &str) -> io::Result<()> {
        match &self.written_projection {
            Some(evidence) if self.authority_bound && evidence.xml.as_ref() == xml => Ok(()),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord written production evidence is absent or exact projected bytes drifted",
            )),
        }
    }

    pub(crate) const fn is_bound(&self) -> bool {
        self.authority_bound
    }

    pub(crate) const fn semantic_fingerprint_fnv64(&self) -> u64 {
        self.semantic_fingerprint_fnv64
    }

    pub(crate) fn voice_free_evidence_sufficiency_green(&self) -> bool {
        self.authority_bound
            && self.unresolved_continuity_decisions == 0
            && self.unresolved_fragment_roles == 0
            && self.unresolved_serialization_runs == 0
            && self.unresolved_presentation_roles == 0
            && self.unresolved_constraint_edges == 0
    }

    pub(crate) fn sufficiency_counts(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.unresolved_continuity_decisions,
            self.unresolved_fragment_roles,
            self.unresolved_serialization_runs,
            self.unresolved_presentation_roles,
            self.unresolved_constraint_edges,
        )
    }
}

pub(crate) fn attach_final_chord_render_occurrence_witnesses(
    canonical: &mut CanonicalFinalChordStructuralPlan,
    witnesses: Vec<FinalChordRenderOccurrenceWitness>,
) -> io::Result<()> {
    if !canonical.voice_free_evidence_sufficiency_green() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord render-occurrence continuation requires sealed canonical structure",
        ));
    }
    let locators = canonical
        .source_locators
        .iter()
        .map(|locator| (locator.fragment_id, locator))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut seen_fragments = std::collections::BTreeSet::<usize>::new();
    let mut seen_occurrences = std::collections::BTreeSet::<(usize, usize, u64)>::new();
    let mut sequence_fingerprints = std::collections::BTreeSet::<u64>::new();
    for witness in &witnesses {
        sequence_fingerprints.insert(witness.sequence_fingerprint_fnv64);
        if !seen_fragments.insert(witness.fragment_id) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord render-occurrence continuation has duplicate fragment witness",
            ));
        }
        let locator = locators.get(&witness.fragment_id).copied().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord render-occurrence continuation references unknown canonical fragment",
            )
        })?;
        if !seen_occurrences.insert((
            witness.measure,
            witness.pitched_ordinal,
            witness.sequence_fingerprint_fnv64,
        )) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord render-occurrence continuation has duplicate physical occurrence locator",
            ));
        }
        if witness.representation_duration.is_some_and(|duration| duration <= 0) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord render-occurrence continuation has non-positive representation duration",
            ));
        }
        if locator.source_event_id != witness.source_event_id
            || locator.attack_id != witness.attack_id
            || locator.measure != witness.measure
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord render-occurrence continuation disagrees with canonical source provenance",
            ));
        }
    }
    if sequence_fingerprints.len() > 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord render-occurrence continuation rows disagree on writer sequence fingerprint",
        ));
    }
    canonical.render_occurrence_witnesses = std::sync::Arc::from(witnesses);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FrozenVoiceAttackEvidenceRow {
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) absolute_start: i64,
    pub(crate) absolute_end: i64,
    pub(crate) staff: u8,
    pub(crate) technical_lane: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LegacyVoiceLabelPreferenceRow {
    pub(crate) source_event_id: usize,
    pub(crate) label: String,
}

#[derive(Debug, Clone)]
pub(crate) struct FrozenSerializationVoiceChannelPlan {
    authority_bound: bool,
    source_to_channel: std::collections::BTreeMap<usize, usize>,
    source_to_role: std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    channel_to_role: std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    channel_count: usize,
    semantic_fingerprint_fnv64: u64,
    consumer_allocation: ChannelConsumerAllocation,
    // Sealed by the original sharing projection, never by a legacy label vote.
    label_continuation: Option<FrozenSerializedVoicePlan>,
}

impl FrozenSerializationVoiceChannelPlan {
    pub(crate) fn require_consumer_admission(
        &self,
        canonical: &CanonicalFinalChordStructuralPlan,
        voices: &FrozenSerializedVoicePlan,
    ) -> io::Result<()> {
        if !self.authority_bound || !canonical.is_bound() || !voices.is_bound()
            || self.source_to_channel != voices.source_to_channel
            || self.source_to_role != voices.source_to_role
            || self.channel_to_role != voices.channel_to_role
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                "Serialization consumer admission has missing or foreign bound inputs"));
        }
        let model = canonical.written_projection.as_ref()
            .and_then(|written| channel_consumer_model(written, &self.source_to_channel))
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData,
                "Serialization consumer admission lacks complete written evidence"))?;
        let channels = self.channel_to_role.keys().map(|&channel| (channel, channel)).collect();
        channel_consumer_overflow_with_labels(&model, &channels, &self.channel_to_role, Some(voices))
            .map_err(|staffs| io::Error::new(io::ErrorKind::InvalidData,
                format!("Serialization consumer admission refused unreserved required channels on staffs {staffs:?}; allocation={:?}", self.consumer_allocation)))
    }

    pub(crate) const fn consumer_allocation(&self) -> &ChannelConsumerAllocation {
        &self.consumer_allocation
    }

    pub(crate) fn channel_for_source(&self, source_event_id: usize) -> Option<usize> {
        self.source_to_channel.get(&source_event_id).copied()
    }

    pub(crate) fn role_for_source(
        &self,
        source_event_id: usize,
    ) -> Option<SerializedVoicePresentationRole> {
        self.source_to_role.get(&source_event_id).copied()
    }

    pub(crate) const fn channel_count(&self) -> usize {
        self.channel_count
    }
}

#[derive(Debug, Clone)]
struct CanonicalFrozenRoleAllocation {
    source_roles: std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    channel_roles: std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FrozenSerializedVoicePlan {
    canonical_role_allocation: Option<std::sync::Arc<CanonicalFrozenRoleAllocation>>,
    authority_bound: bool,
    source_to_role: std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    source_to_channel: std::collections::BTreeMap<usize, usize>,
    source_to_label: std::collections::BTreeMap<usize, String>,
    label_to_role: std::collections::BTreeMap<String, SerializedVoicePresentationRole>,
    channel_to_label: std::collections::BTreeMap<usize, String>,
    channel_to_role: std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    semantic_fingerprint_fnv64: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrimaryBeamVoicePartitionRepairCandidate {
    metric_line_id: String,
    members: Vec<(usize, usize)>,
    measure: usize,
    staff: u8,
    canonical_primary_states: Vec<String>,
    target_primary_source_event_id: usize,
    target_primary_attack_id: usize,
    target_channel: usize,
}

impl PrimaryBeamVoicePartitionRepairCandidate {
    pub(crate) fn new(
        metric_line_id: String,
        members: Vec<(usize, usize)>,
        measure: usize,
        staff: u8,
        canonical_primary_states: Vec<String>,
        target_primary_source_attack: (usize, usize),
        target_channel: usize,
    ) -> Self {
        Self {
            metric_line_id,
            members,
            measure,
            staff,
            canonical_primary_states,
            target_primary_source_event_id: target_primary_source_attack.0,
            target_primary_attack_id: target_primary_source_attack.1,
            target_channel,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrimaryBeamVoicePartitionRendererVerificationCohort {
    metric_line_id: String,
    measure: usize,
    staff: u8,
    pitched_ordinals: Vec<usize>,
}

impl PrimaryBeamVoicePartitionRendererVerificationCohort {
    pub(crate) fn metric_line_id(&self) -> &str {
        &self.metric_line_id
    }

    pub(crate) const fn measure(&self) -> usize {
        self.measure
    }

    pub(crate) const fn staff(&self) -> u8 {
        self.staff
    }

    pub(crate) fn pitched_ordinals(&self) -> &[usize] {
        &self.pitched_ordinals
    }
}

pub(crate) fn primary_beam_voice_partition_renderer_cohorts(
    source_continuation: &FinalPdfSourceOccurrenceContinuation,
    candidates: &[PrimaryBeamVoicePartitionRepairCandidate],
) -> Vec<PrimaryBeamVoicePartitionRendererVerificationCohort> {
    let mut out = Vec::new();
    for candidate in candidates {
        let mut pitched_ordinals = Vec::<usize>::new();
        let mut exact = true;
        for &(source_event_id, attack_id) in &candidate.members {
            let matches = source_continuation
                .receipt
                .occurrences
                .iter()
                .filter(|row| {
                    row.source_event_id == source_event_id
                        && row.attack_id == attack_id
                        && row.measure == candidate.measure
                })
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                exact = false;
                break;
            }
            pitched_ordinals.push(matches[0].pitched_ordinal);
        }
        pitched_ordinals.sort_unstable();
        if !exact
            || pitched_ordinals.is_empty()
            || pitched_ordinals.windows(2).any(|pair| pair[0] == pair[1])
        {
            continue;
        }
        out.push(PrimaryBeamVoicePartitionRendererVerificationCohort {
            metric_line_id: candidate.metric_line_id.clone(),
            measure: candidate.measure,
            staff: candidate.staff,
            pitched_ordinals,
        });
    }
    out.extend(original_beam_voice_alignment::renderer_cohorts(source_continuation));
    out.sort_by(|left, right| left.metric_line_id.cmp(&right.metric_line_id));
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrimaryBeamVoicePartitionBoundEdit {
    measure: usize,
    pitched_ordinal: usize,
    voice_text_start: usize,
    voice_text_end: usize,
    expected_old_label: String,
    target_label: String,
}

impl PrimaryBeamVoicePartitionBoundEdit {
    pub(crate) const fn voice_text_start(&self) -> usize {
        self.voice_text_start
    }

    pub(crate) const fn voice_text_end(&self) -> usize {
        self.voice_text_end
    }

    pub(crate) fn target_label(&self) -> &str {
        &self.target_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrimaryBeamVoicePartitionSelectedMember {
    source_event_id: usize,
    attack_id: usize,
    fragment_id: usize,
    target_primary_fragment_id: usize,
    target_primary_source_event_id: usize,
    target_primary_attack_id: usize,
    target_channel: usize,
    target_label: String,
}

pub(crate) struct PrimaryBeamVoicePartitionBindingContext<'a> {
    pub(crate) source_continuation: &'a FinalPdfSourceOccurrenceContinuation,
    pub(crate) voice_plan: &'a FrozenSerializedVoicePlan,
    pub(crate) candidates: &'a [PrimaryBeamVoicePartitionRepairCandidate],
    pub(crate) classification_xml: &'a str,
    pub(crate) renderer_visible_fragments_by_metric_line_id:
        &'a std::collections::BTreeMap<String, usize>,
    pub(crate) materialization_safety_proven_metric_line_ids:
        &'a std::collections::BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct PrimaryBeamVoicePartitionBoundPlan {
    source_fingerprint_fnv64: u64,
    repaired_voice_plan: FrozenSerializedVoicePlan,
    edits: Vec<PrimaryBeamVoicePartitionBoundEdit>,
    selected_members: Vec<PrimaryBeamVoicePartitionSelectedMember>,
    scope_candidates: usize,
    original_group_alignment: Vec<String>,
    historical_phrase_decision_only: bool,
}

impl PrimaryBeamVoicePartitionBoundPlan {
    pub(crate) fn permits_pdf_finalization(&self) -> bool {
        !self.historical_phrase_decision_only
    }

    pub(crate) fn matches_source_xml(&self, source_xml: &str) -> bool {
        self.source_fingerprint_fnv64 == fingerprint(source_xml)
    }

    pub(crate) fn repaired_voice_plan(&self) -> &FrozenSerializedVoicePlan {
        &self.repaired_voice_plan
    }

    pub(crate) fn edits(&self) -> &[PrimaryBeamVoicePartitionBoundEdit] {
        &self.edits
    }

    pub(crate) const fn scope_candidates(&self) -> usize {
        self.scope_candidates
    }

    pub(crate) const fn changed_voice_elements(&self) -> usize {
        self.edits.len()
    }
}

pub(crate) fn rebind_primary_beam_voice_partition_plan_to_current_xml(
    source_xml: &str,
    source_continuation: &FinalPdfSourceOccurrenceContinuation,
    plan: &PrimaryBeamVoicePartitionBoundPlan,
) -> io::Result<PrimaryBeamVoicePartitionBoundPlan> {
    if source_continuation.xml() != source_xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition late rebind received stale exact occurrence continuation",
        ));
    }

    let measures = final_chord_parse_binding_measures(source_xml)?;
    let mut repaired_voice_plan = plan.repaired_voice_plan.clone();
    let mut edits = Vec::<PrimaryBeamVoicePartitionBoundEdit>::new();
    let mut claimed_identities = std::collections::BTreeSet::<(usize, usize, usize)>::new();
    let mut claimed_footprints = std::collections::BTreeSet::<(usize, usize)>::new();

    for selected in &plan.selected_members {
        if !claimed_identities.insert((selected.source_event_id, selected.attack_id, selected.fragment_id)) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition late rebind found overlapping source/attack membership",
            ));
        }

        let target_matches = source_continuation
            .receipt
            .occurrences
            .iter()
            .filter(|row| {
                row.source_event_id == selected.target_primary_source_event_id
                    && row.attack_id == selected.target_primary_attack_id
                    && row.fragment_id == selected.target_primary_fragment_id
            })
            .collect::<Vec<_>>();
        if target_matches.len() != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition late rebind lost exact primary target source/attack occurrence",
            ));
        }
        let target_occurrence = target_matches[0];
        let target_measure = measures
            .iter()
            .find(|measure| measure.measure == target_occurrence.measure)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition late rebind lost target measure",
                )
            })?;
        let target_note = target_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .nth(target_occurrence.pitched_ordinal)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition late rebind lost target pitched occurrence",
                )
            })?;
        let target_label = target_note.voice.trim();
        if target_label != selected.target_label.as_str()
            || repaired_voice_plan
                .channel_to_label
                .get(&selected.target_channel)
                .map(String::as_str)
                != Some(target_label)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition late rebind primary target voice drifted from the sealed target channel",
            ));
        }

        let matches = source_continuation
            .receipt
            .occurrences
            .iter()
            .filter(|row| {
                row.source_event_id == selected.source_event_id
                    && row.attack_id == selected.attack_id
                    && row.fragment_id == selected.fragment_id
            })
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition late rebind lost exact member source/attack occurrence",
            ));
        }
        let occurrence = matches[0];
        if !claimed_footprints.insert((occurrence.measure, occurrence.pitched_ordinal)) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition late rebind produced overlapping physical edit footprints",
            ));
        }
        let measure = measures
            .iter()
            .find(|measure| measure.measure == occurrence.measure)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition late rebind lost selected measure",
                )
            })?;
        let note = measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .nth(occurrence.pitched_ordinal)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition late rebind lost selected pitched occurrence",
                )
            })?;
        let old_label = note.voice.trim().to_string();

        repaired_voice_plan
            .source_to_channel
            .insert(selected.source_event_id, selected.target_channel);
        repaired_voice_plan
            .source_to_label
            .insert(selected.source_event_id, target_label.to_string());

        if old_label == target_label {
            continue;
        }

        let (voice_text_start, voice_text_end) =
            primary_beam_voice_partition_voice_text_range(source_xml, note.range.clone())?;
        if source_xml
            .get(voice_text_start..voice_text_end)
            .map(str::trim)
            != Some(old_label.as_str())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition late rebind voice scalar disagrees with the exact current occurrence",
            ));
        }
        edits.push(PrimaryBeamVoicePartitionBoundEdit {
            measure: occurrence.measure,
            pitched_ordinal: occurrence.pitched_ordinal,
            voice_text_start,
            voice_text_end,
            expected_old_label: old_label,
            target_label: target_label.to_string(),
        });
    }

    edits.sort_by_key(|edit| (edit.voice_text_start, edit.voice_text_end));
    if edits
        .windows(2)
        .any(|pair| pair[0].voice_text_end > pair[1].voice_text_start)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition late rebind produced overlapping scalar edits",
        ));
    }

    if !plan.original_group_alignment.is_empty() {
        original_beam_voice_alignment::validate_late_groups(source_xml, source_continuation, &plan.original_group_alignment)?;
        original_beam_voice_alignment::validate_batch_occupancy(source_xml, &edits)?;
    }
    Ok(PrimaryBeamVoicePartitionBoundPlan {
        source_fingerprint_fnv64: fingerprint(source_xml),
        repaired_voice_plan,
        edits,
        selected_members: plan.selected_members.clone(),
        scope_candidates: plan.scope_candidates,
        original_group_alignment: plan.original_group_alignment.clone(),
        historical_phrase_decision_only: plan.historical_phrase_decision_only,
    })
}

#[path = "actual_source_beam_binding.rs"]
mod actual_source_beam_binding;
pub(crate) use actual_source_beam_binding::{ActualSourceBeamBindingObservation, observe_actual_source_beam_binding};

#[cfg(test)]
pub(crate) fn frozen_serialized_voice_plan_for_rest_role_replay_test(
    primary_label: &str,
    secondary_label: &str,
) -> FrozenSerializedVoicePlan {
    let label_to_role = [
        (
            primary_label.to_string(),
            SerializedVoicePresentationRole::UpperPrimary,
        ),
        (
            secondary_label.to_string(),
            SerializedVoicePresentationRole::UpperSecondary,
        ),
    ]
    .into_iter()
    .collect::<std::collections::BTreeMap<_, _>>();
    let channel_to_label = [
        (1_usize, primary_label.to_string()),
        (2_usize, secondary_label.to_string()),
    ]
    .into_iter()
    .collect::<std::collections::BTreeMap<_, _>>();
    let channel_to_role = [
        (1_usize, SerializedVoicePresentationRole::UpperPrimary),
        (2_usize, SerializedVoicePresentationRole::UpperSecondary),
    ]
    .into_iter()
    .collect::<std::collections::BTreeMap<_, _>>();
    FrozenSerializedVoicePlan {
        canonical_role_allocation: None,
        source_to_role: std::collections::BTreeMap::new(),
        authority_bound: true,
        source_to_channel: std::collections::BTreeMap::new(),
        source_to_label: std::collections::BTreeMap::new(),
        label_to_role,
        channel_to_label,
        channel_to_role,
        semantic_fingerprint_fnv64: 1,
    }
}

impl FrozenSerializedVoicePlan {
    fn has_canonical_role_allocation(&self) -> bool {
        self.canonical_role_allocation.as_ref().is_some_and(|original|
            original.source_roles == self.source_to_role && original.channel_roles == self.channel_to_role)
    }

    /// Canonical pitched-source role, independent of export channels and labels.
    /// This supplies no role for rests or individual notation fragments.
    pub(crate) fn role_for_source(
        &self,
        source_event_id: usize,
    ) -> Option<SerializedVoicePresentationRole> {
        self.source_to_role.get(&source_event_id).copied()
    }

    pub(crate) const fn is_bound(&self) -> bool {
        self.authority_bound
    }

    pub(crate) fn label_for_source(&self, source_event_id: usize) -> Option<&str> {
        self.source_to_label.get(&source_event_id).map(String::as_str)
    }

    pub(crate) fn role_for_label(&self, label: &str) -> Option<SerializedVoicePresentationRole> {
        self.label_to_role.get(label.trim()).copied()
    }

    pub(crate) fn channel_for_label(&self, label: &str) -> Option<usize> {
        let label = label.trim();
        self.channel_to_label
            .iter()
            .find_map(|(&channel, candidate)| (candidate == label).then_some(channel))
    }

    pub(crate) fn label_is_shadow_rest_primary_exempt(&self, label: &str) -> Option<bool> {
        Some(self.role_for_label(label)?.shadow_rest_primary_exempt())
    }

    pub(crate) fn label_is_lower_dual_edge_primary(&self, label: &str) -> Option<bool> {
        Some(self.role_for_label(label)?.lower_dual_edge_primary())
    }

}


#[derive(Debug, Clone, Default)]
pub(crate) struct VoiceAuthorityCutoverMetamorphicAudit {
    label_alpha_cases: usize,
    label_alpha_semantic_deltas: usize,
    legacy_partition_cases: usize,
    legacy_partition_semantic_deltas: usize,
    opaque_identity_rename_cases: usize,
    opaque_identity_semantic_deltas: usize,
    green: bool,
}

impl VoiceAuthorityCutoverMetamorphicAudit {
    pub(crate) const fn green(&self) -> bool {
        self.green
    }

    pub(crate) const fn label_alpha_cases(&self) -> usize {
        self.label_alpha_cases
    }

    pub(crate) const fn label_alpha_semantic_deltas(&self) -> usize {
        self.label_alpha_semantic_deltas
    }

    pub(crate) const fn legacy_partition_cases(&self) -> usize {
        self.legacy_partition_cases
    }

    pub(crate) const fn legacy_partition_semantic_deltas(&self) -> usize {
        self.legacy_partition_semantic_deltas
    }

    pub(crate) const fn opaque_identity_rename_cases(&self) -> usize {
        self.opaque_identity_rename_cases
    }

    pub(crate) const fn opaque_identity_semantic_deltas(&self) -> usize {
        self.opaque_identity_semantic_deltas
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct VoiceLabelBindingMetamorphicAudit {
    cases: usize,
    semantic_role_deltas: usize,
    paired_binding_failures: usize,
    stale_binding_false_accepts: usize,
    green: bool,
}

impl VoiceLabelBindingMetamorphicAudit {
    pub(crate) const fn green(&self) -> bool {
        self.green
    }

    pub(crate) const fn cases(&self) -> usize {
        self.cases
    }

    pub(crate) const fn semantic_role_deltas(&self) -> usize {
        self.semantic_role_deltas
    }

    pub(crate) const fn paired_binding_failures(&self) -> usize {
        self.paired_binding_failures
    }

    pub(crate) const fn stale_binding_false_accepts(&self) -> usize {
        self.stale_binding_false_accepts
    }
}

struct FinalChordCanonicalPlanningContext<'a> {
    evidence_by_key: std::collections::BTreeMap<(usize, usize), &'a VoiceFreeFinalChordEvidenceRow>,
    source_locators: Vec<FinalChordCanonicalSourceLocator>,
    actions: Vec<FinalChordCanonicalActionRow>,
    hard_same_edges: Vec<FinalChordCanonicalHardSameEdge>,
    measures: Vec<FinalChordCanonicalMeasureInfo>,
    next_serialization_run_id: usize,
    unresolved_continuity_decisions: usize,
    unresolved_fragment_roles: usize,
    unresolved_serialization_runs: usize,
    unresolved_presentation_roles: usize,
    unresolved_constraint_edges: usize,
}

impl<'a> FinalChordCanonicalPlanningContext<'a> {
    fn new(evidence: &'a [VoiceFreeFinalChordEvidenceRow]) -> io::Result<Self> {
        let mut evidence_by_key = std::collections::BTreeMap::new();
        let mut source_attack = std::collections::BTreeMap::<usize, usize>::new();
        let mut fragment_ids = std::collections::BTreeSet::<usize>::new();
        for row in evidence {
            if row.fragment_id == 0 || !fragment_ids.insert(row.fragment_id) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VoiceFreeFinalChordEvidence contains invalid/duplicate opaque FragmentId",
                ));
            }
            if !matches!(row.staff, 1 | 2)
                || row.measure == 0
                || row.duration <= 0
                || row.onset < 0
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VoiceFreeFinalChordEvidence contains invalid temporal/staff data",
                ));
            }
            if evidence_by_key.insert((row.measure, row.raw_note_index), row).is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VoiceFreeFinalChordEvidence contains duplicate physical locator",
                ));
            }
            if let Some(previous_attack) = source_attack.insert(row.source_event_id, row.attack_id)
                && previous_attack != row.attack_id
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VoiceFreeFinalChordEvidence maps one SourceEventId to multiple SoundingAttackIds",
                ));
            }
        }
        Ok(Self {
            evidence_by_key,
            source_locators: Vec::new(),
            actions: Vec::new(),
            hard_same_edges: Vec::new(),
            measures: Vec::new(),
            next_serialization_run_id: 1,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        })
    }

    fn alloc_serialization_run(&mut self) -> usize {
        let id = self.next_serialization_run_id;
        self.next_serialization_run_id = self.next_serialization_run_id.saturating_add(1);
        id
    }
}

struct FinalChordAuxiliaryPlanContext<'a> {
    xml: &'a str,
    notes: &'a [FinalChordEncodingNote],
    divisions: i64,
    expected_duration: i64,
    cursor: i64,
    measure_close: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AuxiliaryLaneKey {
    serialization_run_id: usize,
    voice: String,
    staff: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AuxiliaryGroupKey {
    lane: AuxiliaryLaneKey,
    onset: i64,
}

#[derive(Debug, Clone)]
struct AuxiliaryMemberCandidate {
    member_index: usize,
    onset: i64,
    duration: i64,
    revoiced_xml: String,
}

fn final_chord_direct_child_text(
    node: roxmltree::Node<'_, '_>,
    name: &str,
) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.has_tag_name(name))
        .and_then(|child| child.text())
        .map(|text| text.trim().to_string())
}

fn final_chord_direct_child_i64(node: roxmltree::Node<'_, '_>, name: &str) -> Option<i64> {
    final_chord_direct_child_text(node, name)?.parse::<i64>().ok()
}

fn final_chord_direct_has_child(node: roxmltree::Node<'_, '_>, name: &str) -> bool {
    node.children()
        .any(|child| child.is_element() && child.has_tag_name(name))
}

fn final_chord_pitch_identity(node: roxmltree::Node<'_, '_>) -> String {
    let Some(pitch) = node
        .children()
        .find(|child| child.is_element() && child.has_tag_name("pitch"))
    else {
        return String::new();
    };
    format!(
        "{}:{}:{}",
        final_chord_direct_child_text(pitch, "step").unwrap_or_default(),
        final_chord_direct_child_text(pitch, "alter").unwrap_or_else(|| "0".to_string()),
        final_chord_direct_child_text(pitch, "octave").unwrap_or_default(),
    )
}

fn final_chord_pitch_midi(node: roxmltree::Node<'_, '_>) -> Option<i32> {
    let pitch = node
        .children()
        .find(|child| child.is_element() && child.has_tag_name("pitch"))?;
    let step = final_chord_direct_child_text(pitch, "step")?;
    let semitone = match step.as_str() {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => return None,
    };
    let alter = final_chord_direct_child_i64(pitch, "alter").unwrap_or(0);
    let octave = final_chord_direct_child_i64(pitch, "octave")?;
    let midi = (octave + 1)
        .checked_mul(12)?
        .checked_add(i64::from(semitone))?
        .checked_add(alter)?;
    i32::try_from(midi).ok()
}

fn final_chord_exact_type_and_dots(duration: i64, divisions: i64) -> Option<(&'static str, usize)> {
    if duration <= 0 || divisions <= 0 {
        return None;
    }
    let values = [
        ("whole", divisions.saturating_mul(4)),
        ("half", divisions.saturating_mul(2)),
        ("quarter", divisions),
        ("eighth", divisions / 2),
        ("16th", divisions / 4),
        ("32nd", divisions / 8),
        ("64th", divisions / 16),
    ];
    for (note_type, base) in values {
        if base <= 0 {
            continue;
        }
        if duration == base {
            return Some((note_type, 0));
        }
        if duration == base.saturating_mul(3) / 2 {
            return Some((note_type, 1));
        }
        if duration == base.saturating_mul(7) / 4 {
            return Some((note_type, 2));
        }
    }
    None
}

fn final_chord_remove_simple_child_blocks(note: String, tag: &str) -> String {
    let paired_open = format!("<{tag}>");
    let paired_close = format!("</{tag}>");
    let mut current = note;
    while let Some(start) = current.find(&paired_open) {
        let Some(close_rel) = current[start..].find(&paired_close) else {
            break;
        };
        let end = start + close_rel + paired_close.len();
        let mut rebuilt = String::with_capacity(current.len().saturating_sub(end - start));
        rebuilt.push_str(&current[..start]);
        rebuilt.push_str(&current[end..]);
        current = rebuilt;
    }
    let empty_a = format!("<{tag}/>");
    let empty_b = format!("<{tag} />");
    current.replace(&empty_a, "").replace(&empty_b, "")
}

fn final_chord_sync_note_type_and_dots(source: &str, duration: i64, divisions: i64) -> String {
    if source.contains("<time-modification") || source.contains("<grace") {
        return source.to_string();
    }
    let Some((note_type, dots)) = final_chord_exact_type_and_dots(duration, divisions) else {
        return source.to_string();
    };
    let current_type = source.find("<type>").and_then(|start| {
        let text_start = start + "<type>".len();
        let text_end = text_start + source[text_start..].find("</type>")?;
        Some(source[text_start..text_end].trim())
    });
    if current_type == Some(note_type) && source.matches("<dot").count() == dots {
        return source.to_string();
    }
    let without_type = final_chord_remove_simple_child_blocks(source.to_string(), "type");
    let out = final_chord_remove_simple_child_blocks(without_type, "dot");
    let Some(note_close) = out.rfind("</note>") else {
        return source.to_string();
    };
    let insert_at = [
        "<accidental",
        "<time-modification",
        "<stem",
        "<notehead",
        "<staff",
        "<beam",
        "<notations",
        "<lyric",
        "<play",
        "<listen",
    ]
    .into_iter()
    .filter_map(|marker| out.find(marker))
    .min()
    .unwrap_or(note_close);
    let insert_at = out[..insert_at]
        .rfind('\n')
        .map_or(insert_at, |line_break| line_break + 1);
    let mut block = format!("        <type>{note_type}</type>\n");
    for _ in 0..dots {
        block.push_str("        <dot/>\n");
    }
    let mut rebuilt = String::with_capacity(out.len() + block.len());
    rebuilt.push_str(&out[..insert_at]);
    rebuilt.push_str(&block);
    rebuilt.push_str(&out[insert_at..]);
    rebuilt
}

fn final_chord_set_note_duration(source: &str, duration: i64) -> Option<String> {
    let open = "<duration>";
    let close = "</duration>";
    let start = source.find(open)? + open.len();
    let end = start + source[start..].find(close)?;
    Some(format!("{}{}{}", &source[..start], duration, &source[end..]))
}

fn final_chord_set_child_text(source: &str, tag: &str, value: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = source.find(&open)? + open.len();
    let end = start + source[start..].find(&close)?;
    Some(format!("{}{}{}", &source[..start], value, &source[end..]))
}

fn final_chord_add_tie_markup(source: &str, start: bool, stop: bool) -> Option<String> {
    if source.contains("<tie") || source.contains("<notations") {
        return None;
    }
    let duration_end = source.find("</duration>")? + "</duration>".len();
    let close = source.rfind("</note>")?;
    if duration_end > close {
        return None;
    }
    let mut ties = String::new();
    if stop {
        ties.push_str("\n        <tie type=\"stop\"/>");
    }
    if start {
        ties.push_str("\n        <tie type=\"start\"/>");
    }
    let mut notations = String::from("        <notations>\n");
    if stop {
        notations.push_str("          <tied type=\"stop\"/>\n");
    }
    if start {
        notations.push_str("          <tied type=\"start\"/>\n");
    }
    notations.push_str("        </notations>\n");
    let mut rebuilt = String::with_capacity(source.len() + ties.len() + notations.len());
    rebuilt.push_str(&source[..duration_end]);
    rebuilt.push_str(&ties);
    rebuilt.push_str(&source[duration_end..close]);
    rebuilt.push_str(&notations);
    rebuilt.push_str(&source[close..]);
    Some(rebuilt)
}


fn final_chord_same_continuity_witness(
    left: &FinalChordEncodingNote,
    right: &FinalChordEncodingNote,
) -> bool {
    match (
        left.source_event_id,
        right.source_event_id,
        left.attack_id,
        right.attack_id,
    ) {
        (Some(_), Some(_), Some(_), Some(_)) => {
            // r790 declassification boundary: when opaque identities are bound, the
            // caller's explicit tie/pitch/staff/temporal witnesses establish
            // continuation. Legacy voice equality must not participate.
            true
        }
        _ => left.voice == right.voice,
    }
}

fn final_chord_same_pitch_reattack_candidate(
    note: &FinalChordEncodingNote,
    member: &FinalChordEncodingNote,
) -> bool {
    if note.pitch_identity != member.pitch_identity || note.staff != member.staff {
        return false;
    }
    match (note.attack_id, member.attack_id) {
        (Some(left), Some(right)) => left != right,
        _ => note.voice == member.voice,
    }
}

fn final_chord_has_same_pitch_reattack_in_span(
    notes: &[FinalChordEncodingNote],
    member_index: usize,
    span_start: i64,
    span_end: i64,
) -> bool {
    let Some(member) = notes.get(member_index) else {
        return true;
    };
    if member.pitch_identity.is_empty() || span_end <= span_start {
        return true;
    }
    notes.iter().enumerate().any(|(note_index, note)| {
        note_index != member_index
            && !note.rest_like
            && !note.pitch_identity.is_empty()
            && final_chord_same_pitch_reattack_candidate(note, member)
            && note.onset > span_start
            && note.onset < span_end
            && !note.tie_stop
    })
}

fn final_chord_tied_anchor_chain(
    notes: &[FinalChordEncodingNote],
    anchor_index: usize,
    member_duration: i64,
) -> io::Result<Option<Vec<usize>>> {
    let Some(anchor) = notes.get(anchor_index) else {
        return Ok(None);
    };
    if anchor.rest_like || anchor.pitch_identity.is_empty() || !anchor.tie_stop {
        return Ok(None);
    }
    let mut chain = vec![anchor_index];
    let mut current_index = anchor_index;
    loop {
        let Some(current) = notes.get(current_index) else {
            return Ok(None);
        };
        if !current.tie_stop {
            break;
        }
        let candidates = (0..current_index)
            .rev()
            .filter(|&index| {
                let note = &notes[index];
                !note.chord_member
                    && !note.rest_like
                    && note.pitch_identity == current.pitch_identity
                    && note.staff == current.staff
                    && final_chord_same_continuity_witness(note, current)
                    && note.onset.saturating_add(note.duration) == current.onset
                    && note.tie_start
            })
            .collect::<Vec<_>>();
        if candidates.len() > 1
            && current.source_event_id.is_some()
            && current.attack_id.is_some()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "VoiceFreeEvidenceSufficiency: tied-anchor continuation at m{} onset {} pitch {} has {} indistinguishable predecessors",
                    current.measure,
                    current.onset,
                    current.pitch_identity,
                    candidates.len()
                ),
            ));
        }
        let Some(predecessor) = candidates.first().copied() else {
            break;
        };
        chain.insert(0, predecessor);
        current_index = predecessor;
    }
    let total = chain.iter().map(|&index| notes[index].duration).sum::<i64>();
    Ok((total == member_duration && chain.len() >= 2).then_some(chain))
}

fn final_chord_push_edit(
    edits: &mut Vec<FinalChordEncodingEdit>,
    start: usize,
    end: usize,
    replacement: String,
) {
    edits.push(FinalChordEncodingEdit {
        start,
        end,
        replacement,
        generated_fragments: Vec::new(),
    });
}

fn final_chord_push_edit_with_generated_provenance(
    edits: &mut Vec<FinalChordEncodingEdit>,
    start: usize,
    end: usize,
    replacement: String,
    generated_fragments: Vec<FinalChordGeneratedFragmentEditProvenance>,
) {
    edits.push(FinalChordEncodingEdit {
        start,
        end,
        replacement,
        generated_fragments,
    });
}

/// Pre-seal representation decision from complete source/attack/fragment staff evidence.
/// No sealed HARD SAME edge or physical presentation staff is changed here.
fn final_chord_requires_independent_staff_representation(
    xml: &str, member: &FinalChordEncodingNote, notes: &[FinalChordEncodingNote],
) -> bool {
    member.source_event_id.is_some() && member.attack_id.is_some() && member.fragment_id.is_some()
            && member.anchor_index.and_then(|index| notes.get(index)).is_some_and(|anchor| {
                final_chord_physical_host_is_sounding(anchor)
                    && anchor.source_event_id.is_some() && anchor.fragment_id.is_some()
                    && anchor.staff != member.staff
                    && matches!(anchor.staff.as_str(), "1" | "2")
                    && matches!(member.staff.as_str(), "1" | "2")
            })
            && xml.get(member.range.clone()).is_some_and(|source| {
                roxmltree::Document::parse(source).ok().is_some_and(|doc| {
                    doc.root_element().attribute("print-spacing") != Some("no")
                })
            })
}

fn plan_auxiliary_lane(
    context: &FinalChordAuxiliaryPlanContext<'_>,
    removed: &mut std::collections::BTreeSet<usize>,
    normalized_in_place: &mut std::collections::BTreeMap<usize, i64>,
    edits: &mut Vec<FinalChordEncodingEdit>,
    forward_materializations: &mut Vec<FinalChordForwardMaterializationAuditRow>,
    mut canonical_context: Option<&mut FinalChordCanonicalPlanningContext<'_>>,
) -> io::Result<()> {
    if context.expected_duration <= 0 || context.cursor != context.expected_duration {
        return Ok(());
    }
    let notes = context.notes;
    let used_voices = notes
        .iter()
        .map(|note| note.voice.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut lanes = std::collections::BTreeMap::<
        AuxiliaryLaneKey,
        Vec<AuxiliaryMemberCandidate>,
    >::new();
    let mut blocked_groups = std::collections::BTreeSet::<AuxiliaryGroupKey>::new();
    let mut canonical_run_by_staff = std::collections::BTreeMap::<(String, bool), usize>::new();
    let mut staff_separation_runs = std::collections::BTreeSet::<usize>::new();

    for (member_index, member) in notes.iter().enumerate() {
        if removed.contains(&member_index)
            || !member.chord_member
            || member.duration <= 0
        {
            continue;
        }
        // r481: The serialized temporal owner first bounds a chord member by a
        // later same-pitch reattack in the same voice/staff. A note cannot carry
        // temporal extent through its own reattack. If that bounded extent fits
        // beneath the predecessor duration, the chord member stays in place with
        // only its duration representation canonicalized. Otherwise the existing
        // FinalChordEncodingPlan auxiliary lane carries the independent extent.
        // No additional post-serialization mutation stage is introduced.
        // Staff is already carried by declassified production evidence here.
        // A different bound staff cannot share the host's consumer chord object.
        // Decide an existing independent representation before HARD SAME is sealed;
        // never edit an already canonical edge or move either presentation staff.
        let staff_separation_required =
            final_chord_requires_independent_staff_representation(context.xml, member, notes);
        let (anchor_duration, auxiliary_required) = match member.anchor_index {
            Some(anchor_index) => {
                let anchor = &notes[anchor_index];
                (anchor.duration, member.duration > anchor.duration || staff_separation_required)
            }
            None => (0, member.orphan_after_forward),
        };
        if !auxiliary_required {
            continue;
        }

        // A same-pitch attack at exactly the same onset remains ambiguous only when
        // it is foreign to this physical chord group. Same-anchor chord peers are
        // already carried atomically by the existing auxiliary group and retain one
        // generated fragment per source occurrence, so they must not block each other.
        // A tie-stop continuation is not a new attack owner either.
        let foreign_same_pitch_same_onset = !member.pitch_identity.is_empty()
            && notes.iter().enumerate().any(|(note_index, note)| {
                let same_anchor_peer = member.anchor_index.is_some()
                    && note.chord_member
                    && note.anchor_index == member.anchor_index;
                note_index != member_index
                    && !note.rest_like
                    && final_chord_same_pitch_reattack_candidate(note, member)
                    && note.onset == member.onset
                    && !note.tie_stop
                    && !same_anchor_peer
            });
        if foreign_same_pitch_same_onset {
            continue;
        }

        let next_same_pitch_onset = if staff_separation_required || member.pitch_identity.is_empty() {
            None
        } else {
            notes
                .iter()
                .enumerate()
                .filter(|(note_index, note)| {
                    *note_index != member_index
                        && !note.rest_like
                        && final_chord_same_pitch_reattack_candidate(note, member)
                        && note.onset > member.onset
                        && note.onset < member.onset.saturating_add(member.duration)
                        && !note.tie_stop
                })
                .map(|(_, note)| note.onset)
                .min()
        };
        let canonical_duration = next_same_pitch_onset
            .map(|onset| onset.saturating_sub(member.onset))
            .unwrap_or(member.duration)
            .min(member.duration);
        if canonical_duration <= 0 {
            continue;
        }

        let Some(source) = context.xml.get(member.range.clone()) else {
            continue;
        };
        if !staff_separation_required && (source.contains("<tie") || source.contains("<notations")) {
            continue;
        }

        if !staff_separation_required && member.anchor_index.is_some() && canonical_duration <= anchor_duration {
            if canonical_duration < member.duration {
                let Some(retimed) = final_chord_set_note_duration(source, canonical_duration) else {
                    continue;
                };
                let retimed = final_chord_sync_note_type_and_dots(
                    &retimed,
                    canonical_duration,
                    context.divisions,
                );
                final_chord_push_edit(
                    edits,
                    member.range.start,
                    member.range.end,
                    retimed,
                );
                normalized_in_place.insert(member_index, canonical_duration);
            }
            continue;
        }

        let canonical_mode = canonical_context.is_some();
        let auxiliary_voice = if canonical_mode {
            match member.staff.as_str() {
                "1" => "9001",
                "2" => "9002",
                _ => continue,
            }
        } else {
            match member.staff.as_str() {
                "1" => "3",
                "2" => "4",
                _ => continue,
            }
        };
        if (!canonical_mode && used_voices.contains(auxiliary_voice))
            || member.onset < 0
            || member.onset.saturating_add(canonical_duration) > context.expected_duration
        {
            continue;
        }
        let serialization_run_id = if canonical_mode {
            if let Some(existing) = canonical_run_by_staff.get(&(member.staff.clone(), staff_separation_required)).copied() {
                existing
            } else {
                let id = canonical_context
                    .as_deref_mut()
                    .map(FinalChordCanonicalPlanningContext::alloc_serialization_run)
                    .unwrap_or(0);
                canonical_run_by_staff.insert((member.staff.clone(), staff_separation_required), id);
                if staff_separation_required { staff_separation_runs.insert(id); }
                id
            }
        } else {
            0
        };
        let lane_key = AuxiliaryLaneKey {
            serialization_run_id,
            voice: auxiliary_voice.to_string(),
            staff: member.staff.clone(),
        };
        let group_key = AuxiliaryGroupKey {
            lane: lane_key.clone(),
            onset: member.onset,
        };
        let Some(revoiced) = final_chord_set_child_text(source, "voice", auxiliary_voice) else {
            blocked_groups.insert(group_key);
            continue;
        };
        let Some(revoiced) = final_chord_set_note_duration(&revoiced, canonical_duration) else {
            blocked_groups.insert(group_key);
            continue;
        };
        let revoiced =
            final_chord_sync_note_type_and_dots(&revoiced, canonical_duration, context.divisions);
        lanes
            .entry(lane_key)
            .or_default()
            .push(AuxiliaryMemberCandidate {
                member_index,
                onset: member.onset,
                duration: canonical_duration,
                revoiced_xml: revoiced,
            });
    }

    let Some(measure_close) = context.measure_close else {
        return Ok(());
    };
    let mut lane_insertion = String::new();
    let mut lane_insertion_provenance = Vec::<FinalChordGeneratedFragmentEditProvenance>::new();

    // Only an original physical chord cohort is a positive joint witness.
    // Coincident independent cohorts and overlapping extents need distinct runs.
    let mut prepared_lanes = Vec::new();
    for (key, mut members) in lanes {
        members.retain(|member| {
            !blocked_groups.contains(&AuxiliaryGroupKey {
                lane: key.clone(),
                onset: member.onset,
            })
        });
        if let Some(canonical) = canonical_context.as_deref_mut().filter(|_| staff_separation_runs.contains(&key.serialization_run_id)) {
            let mut cohorts = std::collections::BTreeMap::<(i64, usize), Vec<AuxiliaryMemberCandidate>>::new();
            for member in members {
                let cohort = notes[member.member_index].anchor_index.unwrap_or(member.member_index);
                cohorts.entry((member.onset, cohort)).or_default().push(member);
            }
            let mut runs = Vec::<(AuxiliaryLaneKey, i64, Vec<AuxiliaryMemberCandidate>)>::new();
            for ((onset, _), cohort) in cohorts {
                let end = onset.saturating_add(cohort.iter().map(|m| m.duration).max().unwrap_or(0));
                let run_index = if let Some(index) = runs.iter().position(|(_, cursor, _)| *cursor <= onset) {
                    index
                } else {
                    let mut run_key = key.clone();
                    if !runs.is_empty() {
                        run_key.serialization_run_id = canonical.alloc_serialization_run();
                    }
                    runs.push((run_key, 0, Vec::new()));
                    runs.len() - 1
                };
                runs[run_index].1 = end;
                runs[run_index].2.extend(cohort);
            }
            prepared_lanes.extend(runs.into_iter().map(|(key, _, members)| (key, members, true)));
        } else {
            prepared_lanes.push((key, members, false));
        }
    }

    for (key, mut members, staff_separation_run) in prepared_lanes {
        if members.is_empty() {
            continue;
        }
        members.sort_by_key(|member| (member.onset, member.member_index));
        let mut group_ranges = Vec::<std::ops::Range<usize>>::new();
        let mut start = 0_usize;
        while start < members.len() {
            let onset = members[start].onset;
            let mut end = start + 1;
            while end < members.len() && members[end].onset == onset {
                end += 1;
            }
            group_ranges.push(start..end);
            start = end;
        }

        let mut lane_cursor = 0_i64;
        let mut lane_green = true;
        for group in &group_ranges {
            let onset = members[group.start].onset;
            let duration = members[group.start..group.end]
                .iter()
                .map(|member| member.duration)
                .max()
                .unwrap_or(0);
            if onset < lane_cursor || duration <= 0 {
                lane_green = false;
                break;
            }
            lane_cursor = onset.saturating_add(duration);
        }
        if !lane_green || lane_cursor > context.expected_duration {
            continue;
        }

        let mut lane = format!(
            "\n      <backup><duration>{}</duration></backup>",
            context.expected_duration
        );
        let mut lane_provenance = Vec::<FinalChordGeneratedFragmentEditProvenance>::new();
        lane_cursor = 0;
        for group in &group_ranges {
            let group_members = &members[group.start..group.end];
            let onset = group_members[0].onset;
            let anchor_offset = group_members
                .iter()
                .enumerate()
                .max_by_key(|(_, member)| (member.duration, std::cmp::Reverse(member.member_index)))
                .map(|(offset, _)| offset)
                .unwrap_or(0);
            let anchor_duration = group_members[anchor_offset].duration;
            let gap = onset.saturating_sub(lane_cursor);
            if gap > 0 {
                lane.push_str(&format!(
                    "\n      <forward><duration>{gap}</duration><voice>{}</voice><staff>{}</staff></forward>",
                    key.voice, key.staff
                ));
            }

            let mut order = (0..group_members.len()).collect::<Vec<_>>();
            order.sort_by_key(|&offset| {
                (
                    if offset == anchor_offset { 0_usize } else { 1_usize },
                    group_members[offset].member_index,
                )
            });
            for offset in order {
                let member = &group_members[offset];
                let source_member = &notes[member.member_index];
                let note = if offset == anchor_offset {
                    final_chord_remove_simple_child_blocks(member.revoiced_xml.clone(), "chord")
                } else {
                    member.revoiced_xml.clone()
                };
                lane.push('\n');
                let replacement_note_start = lane.len();
                lane.push_str(&note);
                let replacement_note_end = lane.len();
                lane_provenance.push(FinalChordGeneratedFragmentEditProvenance {
                    source_note_start: source_member.range.start,
                    source_note_end: source_member.range.end,
                    fragment_ordinal: 0,
                    kind: FinalChordForwardMaterializationKind::AuxiliaryLane,
                    replacement_note_start,
                    replacement_note_end,
                });
            }
            lane_cursor = onset.saturating_add(anchor_duration);
        }
        let tail = context.expected_duration.saturating_sub(lane_cursor);
        if tail > 0 {
            lane.push_str(&format!(
                "\n      <forward><duration>{tail}</duration><voice>{}</voice><staff>{}</staff></forward>",
                key.voice, key.staff
            ));
        }

        // Bind the actual generated head to its proven original cohort before
        // sealing. Independent runs never acquire a joint edge from coincidence.
        if let Some(canonical) = canonical_context.as_deref_mut().filter(|_| staff_separation_run) {
            for group in &group_ranges {
                let group_members = &members[group.start..group.end];
                let anchor = group_members.iter().max_by_key(|member| {
                    (member.duration, std::cmp::Reverse(member.member_index))
                }).unwrap();
                let anchor_note = &notes[anchor.member_index];
                for member in group_members.iter().filter(|member| member.member_index != anchor.member_index) {
                    let source = &notes[member.member_index];
                    if let (Some(left), Some(right)) = (source.fragment_id, anchor_note.fragment_id) {
                        canonical.hard_same_edges.push(FinalChordCanonicalHardSameEdge {
                            left_fragment_id: left,
                            right_fragment_id: right,
                            presentation_staff: key.staff.parse::<u8>().unwrap_or(0),
                        });
                    } else {
                        canonical.unresolved_constraint_edges = canonical.unresolved_constraint_edges.saturating_add(1);
                    }
                }
            }
        }

        // Commit candidate removal only after the whole lane is proven non-overlapping.
        // A rejected lane therefore leaves every original chord member untouched.
        for member in &members {
            let source_member = &notes[member.member_index];
            removed.insert(member.member_index);
            forward_materializations.push(FinalChordForwardMaterializationAuditRow {
                source_note_start: source_member.range.start,
                source_note_end: source_member.range.end,
                source_onset: source_member.onset,
                projected_sounding_onset: member.onset,
                generated_fragment_count: 1,
                kind: FinalChordForwardMaterializationKind::AuxiliaryLane,
            });
            if let Some(canonical) = canonical_context.as_deref_mut() {
                let Some(source_event_id) = source_member.source_event_id else {
                    canonical.unresolved_fragment_roles = canonical.unresolved_fragment_roles.saturating_add(1);
                    continue;
                };
                let Some(attack_id) = source_member.attack_id else {
                    canonical.unresolved_fragment_roles = canonical.unresolved_fragment_roles.saturating_add(1);
                    continue;
                };
                let Some(fragment_id) = source_member.fragment_id else {
                    canonical.unresolved_fragment_roles = canonical.unresolved_fragment_roles.saturating_add(1);
                    continue;
                };
                let host_source_event_id = source_member
                    .anchor_index
                    .and_then(|index| notes.get(index))
                    .and_then(|note| note.source_event_id);
                canonical.actions.push(FinalChordCanonicalActionRow {
                    fragment_id,
                    source_event_id,
                    attack_id,
                    host_source_event_id,
                    locator: FinalChordCanonicalSourceLocator {
                        fragment_id,
                        source_event_id,
                        attack_id,
                        measure: source_member.measure,
                        raw_note_index: source_member.raw_note_index,
                        onset: source_member.onset,
                        duration: source_member.duration,
                        pitch_identity: source_member.pitch_identity.clone(),
                        pitch_midi: source_member.pitch_midi.unwrap_or(-1),
                        staff: source_member.staff.clone(),
                        chord_syntax: source_member.chord_member,
                        tie_start: source_member.tie_start,
                        tie_stop: source_member.tie_stop,
                    },
                    representation_role: FinalChordRepresentationRole::IndependentAuxiliaryFragment,
                    kind: FinalChordCanonicalActionKind::AuxiliaryLane {
                        serialization_run_id: key.serialization_run_id,
                        canonical_duration: member.duration,
                    },
                });
            }
        }
        let lane_base = lane_insertion.len();
        lane_insertion.push_str(&lane);
        lane_insertion_provenance.extend(lane_provenance.into_iter().map(|mut row| {
            row.replacement_note_start = row.replacement_note_start.saturating_add(lane_base);
            row.replacement_note_end = row.replacement_note_end.saturating_add(lane_base);
            row
        }));
    }

    if !lane_insertion.is_empty() {
        final_chord_push_edit_with_generated_provenance(
            edits,
            measure_close,
            measure_close,
            lane_insertion,
            lane_insertion_provenance,
        );
    }
    Ok(())
}

fn plan_measure_final_chord_encoding(
    xml: &str,
    measure: roxmltree::Node<'_, '_>,
    divisions: i64,
    expected_duration: i64,
    edits: &mut Vec<FinalChordEncodingEdit>,
    forward_materializations: &mut Vec<FinalChordForwardMaterializationAuditRow>,
    mut canonical_context: Option<&mut FinalChordCanonicalPlanningContext<'_>>,
) -> io::Result<()> {
    let measure_ordinal = measure
        .attribute("number")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    if canonical_context.is_some() && measure_ordinal == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "CanonicalFinalChordStructuralPlan requires numeric non-zero measure identity",
        ));
    }
    if let Some(context) = canonical_context.as_deref_mut() {
        context.measures.push(FinalChordCanonicalMeasureInfo {
            measure: measure_ordinal,
            divisions,
            expected_duration,
        });
    }

    let mut cursor = 0_i64;
    let mut last_anchor_onset = None;
    let mut last_anchor_index = None;
    let mut orphan_after_forward = false;
    let mut raw_note_index = 0_usize;
    let mut notes = Vec::<FinalChordEncodingNote>::new();
    for child in measure.children().filter(|node| node.is_element()) {
        if child.has_tag_name("backup") {
            cursor = cursor.saturating_sub(final_chord_direct_child_i64(child, "duration").unwrap_or(0));
            last_anchor_onset = None;
            last_anchor_index = None;
            orphan_after_forward = false;
            continue;
        }
        if child.has_tag_name("forward") {
            cursor = cursor.saturating_add(final_chord_direct_child_i64(child, "duration").unwrap_or(0));
            last_anchor_onset = None;
            last_anchor_index = None;
            orphan_after_forward = true;
            continue;
        }
        if !child.has_tag_name("note") {
            continue;
        }
        let duration = final_chord_direct_child_i64(child, "duration").unwrap_or(0).max(0);
        let chord_member = final_chord_direct_has_child(child, "chord");
        let onset = if chord_member {
            last_anchor_onset.unwrap_or(cursor)
        } else {
            cursor
        };
        let anchor_index = if chord_member { last_anchor_index } else { None };
        let orphan_member_after_forward =
            chord_member && anchor_index.is_none() && orphan_after_forward;
        let rest_like = final_chord_direct_has_child(child, "rest");
        let pitch_identity = final_chord_pitch_identity(child);
        let pitch_midi = final_chord_pitch_midi(child);
        let staff = final_chord_direct_child_text(child, "staff").unwrap_or_else(|| "1".to_string());
        let tie_start = child.children().any(|item| {
            item.is_element()
                && item.has_tag_name("tie")
                && item.attribute("type") == Some("start")
        });
        let tie_stop = child.children().any(|item| {
            item.is_element()
                && item.has_tag_name("tie")
                && item.attribute("type") == Some("stop")
        });
        let mut source_event_id = None;
        let mut attack_id = None;
        let mut fragment_id = None;
        if !rest_like
            && pitch_midi.is_some()
            && let Some(context) = canonical_context.as_deref_mut()
        {
            let evidence = context
                .evidence_by_key
                .get(&(measure_ordinal, raw_note_index))
                .copied()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "VoiceFreeEvidenceSufficiency: pitched fragment m{}#{} has no declassified Source/Attack binding",
                            measure_ordinal, raw_note_index
                        ),
                    )
                })?;
            let staff_u8 = staff.parse::<u8>().ok();
            if evidence.onset != onset
                || evidence.duration != duration
                || Some(evidence.pitch) != pitch_midi
                || Some(evidence.staff) != staff_u8
                || evidence.chord_syntax != chord_member
                || evidence.tie_start != tie_start
                || evidence.tie_stop != tie_stop
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "VoiceFreeEvidenceSufficiency: declassified evidence drift at m{}#{}",
                        measure_ordinal, raw_note_index
                    ),
                ));
            }
            source_event_id = Some(evidence.source_event_id);
            attack_id = Some(evidence.attack_id);
            fragment_id = Some(evidence.fragment_id);
            context.source_locators.push(FinalChordCanonicalSourceLocator {
                fragment_id: evidence.fragment_id,
                source_event_id: evidence.source_event_id,
                attack_id: evidence.attack_id,
                measure: measure_ordinal,
                raw_note_index,
                onset,
                duration,
                pitch_identity: pitch_identity.clone(),
                pitch_midi: final_chord_pitch_midi(child).unwrap_or(-1),
                staff: staff.clone(),
                chord_syntax: chord_member,
                tie_start,
                tie_stop,
            });
        }
        let note_index = notes.len();
        notes.push(FinalChordEncodingNote {
            range: child.range(),
            measure: measure_ordinal,
            raw_note_index,
            onset,
            duration,
            chord_member,
            rest_like,
            pitch_identity,
            pitch_midi,
            voice: final_chord_direct_child_text(child, "voice").unwrap_or_else(|| "1".to_string()),
            staff,
            tie_start,
            tie_stop,
            anchor_index,
            orphan_after_forward: orphan_member_after_forward,
            source_event_id,
            attack_id,
            fragment_id,
        });
        raw_note_index = raw_note_index.saturating_add(1);
        if !chord_member {
            last_anchor_onset = Some(onset);
            last_anchor_index = Some(note_index);
            orphan_after_forward = false;
            cursor = cursor.saturating_add(duration);
        }
    }

    let mut removed = std::collections::BTreeSet::<usize>::new();
    let mut normalized_in_place = std::collections::BTreeMap::<usize, i64>::new();
    #[derive(Debug)]
    struct GeneratedInsertion {
        xml: String,
        source_note_start: usize,
        source_note_end: usize,
        fragment_ordinal: usize,
        kind: FinalChordForwardMaterializationKind,
    }
    let mut insertions =
        std::collections::BTreeMap::<usize, Vec<GeneratedInsertion>>::new();
    for (member_index, member) in notes.iter().enumerate() {
        if !member.chord_member || member.duration <= 0 {
            continue;
        }
        let Some(anchor_index) = member.anchor_index else {
            continue;
        };
        let anchor = &notes[anchor_index];
        // The bound independent staff obligation is planned by the existing
        // auxiliary owner before a new tied-host HARD SAME relation is minted.
        if final_chord_requires_independent_staff_representation(xml, member, &notes) {
            continue;
        }
        if member.duration <= anchor.duration {
            continue;
        }
        let Some(chain) =
            final_chord_tied_anchor_chain(&notes, anchor_index, member.duration)?
        else {
            continue;
        };
        let Some(first_chain_index) = chain.first().copied() else {
            continue;
        };
        let span_start = notes[first_chain_index].onset;
        let span_end = span_start.saturating_add(member.duration);
        if final_chord_has_same_pitch_reattack_in_span(
            &notes,
            member_index,
            span_start,
            span_end,
        ) {
            continue;
        }
        let Some(source) = xml.get(member.range.clone()) else {
            continue;
        };
        if source.contains("<tie") || source.contains("<notations") {
            continue;
        }
        let mut fragments = Vec::with_capacity(chain.len());
        let mut green = true;
        for (ordinal, &chain_index) in chain.iter().enumerate() {
            let duration = notes[chain_index].duration;
            let Some(fragment) = final_chord_set_note_duration(source, duration) else {
                green = false;
                break;
            };
            let fragment = final_chord_sync_note_type_and_dots(&fragment, duration, divisions);
            let Some(fragment) = final_chord_add_tie_markup(
                &fragment,
                ordinal + 1 < chain.len(),
                ordinal > 0,
            ) else {
                green = false;
                break;
            };
            fragments.push((chain_index, fragment));
        }
        if !green {
            continue;
        }
        removed.insert(member_index);
        forward_materializations.push(FinalChordForwardMaterializationAuditRow {
            source_note_start: member.range.start,
            source_note_end: member.range.end,
            source_onset: member.onset,
            projected_sounding_onset: span_start,
            generated_fragment_count: fragments.len(),
            kind: FinalChordForwardMaterializationKind::TiedAnchorChain,
        });
        if let Some(canonical) = canonical_context.as_deref_mut() {
            let (Some(source_event_id), Some(attack_id), Some(fragment_id)) =
                (member.source_event_id, member.attack_id, member.fragment_id)
            else {
                canonical.unresolved_fragment_roles =
                    canonical.unresolved_fragment_roles.saturating_add(1);
                continue;
            };
            let host_source_event_id = notes
                .get(anchor_index)
                .and_then(|note| note.source_event_id);
            let mut targets = Vec::with_capacity(chain.len());
            let mut target_green = true;
            for (ordinal, chain_index) in chain.iter().copied().enumerate() {
                let target = &notes[chain_index];
                let (Some(_anchor_source_event_id), Some(anchor_fragment_id)) =
                    (target.source_event_id, target.fragment_id)
                else {
                    target_green = false;
                    break;
                };
                targets.push(FinalChordCanonicalFragmentTarget {
                    anchor_fragment_id,
                    target_onset: target.onset,
                    duration: target.duration,
                    tie_start: ordinal + 1 < chain.len(),
                    tie_stop: ordinal > 0,
                });
                canonical.hard_same_edges.push(FinalChordCanonicalHardSameEdge {
                    left_fragment_id: fragment_id,
                    right_fragment_id: anchor_fragment_id,
                    presentation_staff: target.staff.parse::<u8>().unwrap_or(0),
                });
            }
            if !target_green || targets.is_empty() {
                canonical.unresolved_constraint_edges =
                    canonical.unresolved_constraint_edges.saturating_add(1);
            } else {
                canonical.actions.push(FinalChordCanonicalActionRow {
                    fragment_id,
                    source_event_id,
                    attack_id,
                    host_source_event_id,
                    locator: FinalChordCanonicalSourceLocator {
                        fragment_id,
                        source_event_id,
                        attack_id,
                        measure: member.measure,
                        raw_note_index: member.raw_note_index,
                        onset: member.onset,
                        duration: member.duration,
                        pitch_identity: member.pitch_identity.clone(),
                        pitch_midi: member.pitch_midi.unwrap_or(-1),
                        staff: member.staff.clone(),
                        chord_syntax: member.chord_member,
                        tie_start: member.tie_start,
                        tie_stop: member.tie_stop,
                    },
                    representation_role:
                        FinalChordRepresentationRole::TiedRematerializedFragment,
                    kind: FinalChordCanonicalActionKind::TiedAnchorChain { fragments: targets },
                });
            }
        }
        for (fragment_ordinal, (chain_index, fragment)) in fragments.into_iter().enumerate() {
            insertions
                .entry(chain_index)
                .or_default()
                .push(GeneratedInsertion {
                    xml: fragment,
                    source_note_start: member.range.start,
                    source_note_end: member.range.end,
                    fragment_ordinal,
                    kind: FinalChordForwardMaterializationKind::TiedAnchorChain,
                });
        }
    }

    let measure_range = measure.range();
    let measure_close = xml
        .get(measure_range.clone())
        .and_then(|text| text.rfind("</measure>"))
        .map(|relative| measure_range.start + relative);
    let auxiliary_context = FinalChordAuxiliaryPlanContext {
        xml,
        notes: &notes,
        divisions,
        expected_duration,
        cursor,
        measure_close,
    };
    plan_auxiliary_lane(
        &auxiliary_context,
        &mut removed,
        &mut normalized_in_place,
        edits,
        forward_materializations,
        canonical_context.as_deref_mut(),
    )?;

    if let Some(canonical) = canonical_context {
        for (member_index, member) in notes.iter().enumerate() {
            if !member.chord_member || member.duration <= 0 || removed.contains(&member_index) {
                continue;
            }
            let (Some(source_event_id), Some(attack_id), Some(fragment_id)) =
                (member.source_event_id, member.attack_id, member.fragment_id)
            else {
                canonical.unresolved_fragment_roles =
                    canonical.unresolved_fragment_roles.saturating_add(1);
                continue;
            };
            let Some(anchor_index) = member.anchor_index else {
                canonical.unresolved_fragment_roles =
                    canonical.unresolved_fragment_roles.saturating_add(1);
                continue;
            };
            let Some(anchor) = notes.get(anchor_index) else {
                canonical.unresolved_fragment_roles =
                    canonical.unresolved_fragment_roles.saturating_add(1);
                continue;
            };
            // A retained <chord/> token only creates semantic HARD SAME when its
            // physical predecessor is itself sounding material.  If the legacy
            // syntax follows a rest and auxiliary rematerialization was rejected,
            // the unique admissible result is to preserve those bytes unchanged;
            // inventing a chord constraint to a non-sounding carrier would turn a
            // deliberate no-op into a false VoiceFreeEvidenceSufficiency failure.
            if !final_chord_physical_host_is_sounding(anchor) {
                continue;
            }
            let (Some(host_source_event_id), Some(host_fragment_id)) =
                (anchor.source_event_id, anchor.fragment_id)
            else {
                canonical.unresolved_constraint_edges =
                    canonical.unresolved_constraint_edges.saturating_add(1);
                continue;
            };
            let presentation_staff = anchor.staff.parse::<u8>().unwrap_or(0);
            if !matches!(presentation_staff, 1 | 2) {
                canonical.unresolved_presentation_roles =
                    canonical.unresolved_presentation_roles.saturating_add(1);
                continue;
            }
            canonical.hard_same_edges.push(FinalChordCanonicalHardSameEdge {
                left_fragment_id: fragment_id,
                right_fragment_id: host_fragment_id,
                presentation_staff,
            });
            canonical.actions.push(FinalChordCanonicalActionRow {
                fragment_id,
                source_event_id,
                attack_id,
                host_source_event_id: Some(host_source_event_id),
                locator: FinalChordCanonicalSourceLocator {
                    fragment_id,
                    source_event_id,
                    attack_id,
                    measure: member.measure,
                    raw_note_index: member.raw_note_index,
                    onset: member.onset,
                    duration: member.duration,
                    pitch_identity: member.pitch_identity.clone(),
                    pitch_midi: member.pitch_midi.unwrap_or(-1),
                    staff: member.staff.clone(),
                    chord_syntax: member.chord_member,
                    tie_start: member.tie_start,
                    tie_stop: member.tie_stop,
                },
                representation_role: FinalChordRepresentationRole::RetainedChordMember,
                kind: FinalChordCanonicalActionKind::RetainedChordMember {
                    canonical_duration: normalized_in_place
                        .get(&member_index)
                        .copied()
                        .unwrap_or(member.duration),
                },
            });
        }
    }

    for &member_index in &removed {
        let member = &notes[member_index];
        final_chord_push_edit(edits, member.range.start, member.range.end, String::new());
    }
    for (anchor_index, fragments) in insertions {
        let anchor = &notes[anchor_index];
        let mut insertion = String::new();
        let mut generated_fragments = Vec::with_capacity(fragments.len());
        for fragment in fragments {
            insertion.push('\n');
            let replacement_note_start = insertion.len();
            insertion.push_str(&fragment.xml);
            let replacement_note_end = insertion.len();
            generated_fragments.push(FinalChordGeneratedFragmentEditProvenance {
                source_note_start: fragment.source_note_start,
                source_note_end: fragment.source_note_end,
                fragment_ordinal: fragment.fragment_ordinal,
                kind: fragment.kind,
                replacement_note_start,
                replacement_note_end,
            });
        }
        final_chord_push_edit_with_generated_provenance(
            edits,
            anchor.range.end,
            anchor.range.end,
            insertion,
            generated_fragments,
        );
    }

    for (note_index, note) in notes.iter().enumerate() {
        if !note.chord_member
            || note.duration <= 0
            || removed.contains(&note_index)
            || normalized_in_place.contains_key(&note_index)
        {
            continue;
        }
        let Some(source) = xml.get(note.range.clone()) else {
            continue;
        };
        let synced = final_chord_sync_note_type_and_dots(source, note.duration, divisions);
        if synced != source {
            final_chord_push_edit(edits, note.range.start, note.range.end, synced);
        }
    }
    Ok(())
}

fn validate_final_chord_musicxml(
    candidate_xml: &str,
) -> io::Result<roxmltree::Document<'_>> {
    if candidate_xml.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "MusicXML is empty"));
    }
    let document = roxmltree::Document::parse(candidate_xml).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("final chord encoding parse failed: {error}"),
        )
    })?;
    if document.root_element().tag_name().name() != "score-partwise" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MusicXML root is not score-partwise",
        ));
    }
    Ok(document)
}


fn final_chord_finalize_bound_plan(
    candidate_xml: &str,
    mut edits: Vec<FinalChordEncodingEdit>,
    mut forward_materializations: Vec<FinalChordForwardMaterializationAuditRow>,
    visible_rest_interval_projection_authorized: bool,
) -> io::Result<FinalChordEncodingPlan> {
    edits.sort_by(|left, right| {
        right
            .start
            .cmp(&left.start)
            .then_with(|| right.end.cmp(&left.end))
    });
    let mut last_start = usize::MAX;
    for edit in &edits {
        if edit.start > edit.end || edit.end > candidate_xml.len() || edit.start > last_start {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "final chord encoding edit ranges overlap or drift",
            ));
        }
        last_start = edit.start;
    }
    forward_materializations.sort_by_key(|row| {
        (
            row.source_note_start,
            row.source_note_end,
            row.projected_sounding_onset,
        )
    });
    Ok(FinalChordEncodingPlan {
        source_fingerprint: fingerprint(candidate_xml),
        source_len: candidate_xml.len(),
        edits,
        forward_materializations,
        visible_rest_interval_projection_authorized,
        original_course_bindings: None,
        original_respelling_input: None,
    })
}

fn final_chord_role_name(role: FinalChordRepresentationRole) -> &'static str {
    match role {
        FinalChordRepresentationRole::RetainedChordMember => "retained_chord_member",
        FinalChordRepresentationRole::TiedRematerializedFragment => "tied_rematerialized_fragment",
        FinalChordRepresentationRole::IndependentAuxiliaryFragment => "independent_auxiliary_fragment",
    }
}

fn final_chord_locator_semantic_key(locator: &FinalChordCanonicalSourceLocator) -> String {
    format!(
        "m{}|o{}|d{}|midi{}|s{}|c{}|ts{}|tp{}",
        locator.measure,
        locator.onset,
        locator.duration,
        locator.pitch_midi,
        locator.staff,
        u8::from(locator.chord_syntax),
        u8::from(locator.tie_start),
        u8::from(locator.tie_stop),
    )
}

/// r790's pre-write semantic-equivalence contract.
///
/// The signature deliberately includes every authority/release-relevant structural
/// property of the canonical plan and deliberately excludes opaque FragmentId /
/// SourceEventId / SoundingAttackId values, SerializationRun ids, channel names, ordering among
/// equivalent rows and later MusicXML voice labels. Two plans are semantically
/// equivalent iff this normalized signature is equal.
fn final_chord_semantic_equivalence_signature(
    context: &FinalChordCanonicalPlanningContext<'_>,
) -> io::Result<String> {
    let mut out = String::from("ETUSCHLOPF_R790_FINAL_CHORD_SEMANTIC_EQUIVALENCE_V1\n");

    let mut locator_keys = context
        .source_locators
        .iter()
        .map(final_chord_locator_semantic_key)
        .collect::<Vec<_>>();
    locator_keys.sort();
    for key in locator_keys {
        let _ = writeln!(out, "L|{key}");
    }

    let locator_by_fragment = context
        .source_locators
        .iter()
        .map(|locator| (locator.fragment_id, final_chord_locator_semantic_key(locator)))
        .collect::<std::collections::BTreeMap<_, _>>();

    let mut action_keys = Vec::<String>::new();
    for action in &context.actions {
        let locator = final_chord_locator_semantic_key(&action.locator);
        let kind = match &action.kind {
            FinalChordCanonicalActionKind::RetainedChordMember {
                canonical_duration,
            } => format!("retained:d{canonical_duration}"),
            FinalChordCanonicalActionKind::TiedAnchorChain { fragments } => {
                let mut rows = fragments
                    .iter()
                    .map(|fragment| {
                        format!(
                            "o{}:d{}:ts{}:tp{}",
                            fragment.target_onset,
                            fragment.duration,
                            u8::from(fragment.tie_start),
                            u8::from(fragment.tie_stop)
                        )
                    })
                    .collect::<Vec<_>>();
                rows.sort();
                format!("tied[{}]", rows.join(","))
            }
            FinalChordCanonicalActionKind::AuxiliaryLane {
                serialization_run_id: _,
                canonical_duration,
            } => format!("aux:d{canonical_duration}"),
        };
        action_keys.push(format!(
            "A|{}|{}|{}",
            final_chord_role_name(action.representation_role),
            locator,
            kind
        ));
    }
    action_keys.sort();
    for key in action_keys {
        let _ = writeln!(out, "{key}");
    }

    let mut same_keys = Vec::<String>::new();
    for edge in &context.hard_same_edges {
        let left = locator_by_fragment.get(&edge.left_fragment_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "semantic equivalence cannot resolve HARD SAME left endpoint",
            )
        })?;
        let right = locator_by_fragment.get(&edge.right_fragment_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "semantic equivalence cannot resolve HARD SAME right endpoint",
            )
        })?;
        let (first, second) = if left <= right { (left, right) } else { (right, left) };
        same_keys.push(format!(
            "SAME|{}|{}|ps{}",
            first, second, edge.presentation_staff
        ));
    }
    same_keys.sort();
    same_keys.dedup();
    for key in same_keys {
        let _ = writeln!(out, "{key}");
    }

    let mut measure_keys = context
        .measures
        .iter()
        .map(|row| format!("M|{}|div{}|dur{}", row.measure, row.divisions, row.expected_duration))
        .collect::<Vec<_>>();
    measure_keys.sort();
    measure_keys.dedup();
    for key in measure_keys {
        let _ = writeln!(out, "{key}");
    }

    let _ = writeln!(
        out,
        "UNRESOLVED|continuity={}|fragment={}|run={}|presentation={}|constraint={}",
        context.unresolved_continuity_decisions,
        context.unresolved_fragment_roles,
        context.unresolved_serialization_runs,
        context.unresolved_presentation_roles,
        context.unresolved_constraint_edges,
    );
    Ok(out)
}

pub(crate) fn plan_canonical_final_chord_structure_once(
    candidate_xml: &str,
    evidence: &[VoiceFreeFinalChordEvidenceRow],
) -> io::Result<(CanonicalFinalChordStructuralPlan, FinalChordEncodingPlan)> {
    let document = validate_final_chord_musicxml(candidate_xml)?;
    if evidence.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "VoiceFreeEvidenceSufficiency: empty declassified pitched evidence",
        ));
    }
    let mut context = FinalChordCanonicalPlanningContext::new(evidence)?;
    let mut edits = Vec::<FinalChordEncodingEdit>::new();
    let mut forward_materializations = Vec::<FinalChordForwardMaterializationAuditRow>::new();
    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
    {
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        for measure in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
        {
            if let Some(attributes) = measure
                .children()
                .find(|node| node.is_element() && node.has_tag_name("attributes"))
            {
                if let Some(value) = final_chord_direct_child_i64(attributes, "divisions") {
                    divisions = value.max(1);
                }
                if let Some(time) = attributes
                    .children()
                    .find(|node| node.is_element() && node.has_tag_name("time"))
                {
                    if let Some(value) = final_chord_direct_child_i64(time, "beats") {
                        beats = value.max(1);
                    }
                    if let Some(value) = final_chord_direct_child_i64(time, "beat-type") {
                        beat_type = value.max(1);
                    }
                }
            }
            let expected_duration = divisions
                .saturating_mul(beats)
                .saturating_mul(4)
                / beat_type.max(1);
            plan_measure_final_chord_encoding(
                candidate_xml,
                measure,
                divisions,
                expected_duration,
                &mut edits,
                &mut forward_materializations,
                Some(&mut context),
            )?;
        }
    }

    if context.source_locators.len() != evidence.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "VoiceFreeEvidenceSufficiency: bound pitched evidence coverage drift expected={} actual={}",
                evidence.len(),
                context.source_locators.len()
            ),
        ));
    }
    if context
        .hard_same_edges
        .iter()
        .any(|edge| !matches!(edge.presentation_staff, 1 | 2))
    {
        context.unresolved_presentation_roles =
            context.unresolved_presentation_roles.saturating_add(1);
    }
    if context.actions.iter().any(|action| {
        matches!(
            &action.kind,
            FinalChordCanonicalActionKind::AuxiliaryLane {
                serialization_run_id,
                ..
            } if *serialization_run_id == 0
        )
    }) {
        context.unresolved_serialization_runs =
            context.unresolved_serialization_runs.saturating_add(1);
    }

    let semantic_signature = final_chord_semantic_equivalence_signature(&context)?;
    let semantic_fingerprint_fnv64 = fingerprint(&semantic_signature);
    let mut plan = CanonicalFinalChordStructuralPlan {
        rest_bridge_main_state: None,
        rest_bridge_extension_course: None,
        rest_bridge_withdrawal: None,
        bridge_voice_guard: None,
        authority_bound: true,
        original_respelling_input: None,
        original_beam_voice_groups: Vec::new(),
        written_projection: None,
        source_locators: std::sync::Arc::from(context.source_locators.clone()),
        render_occurrence_witnesses: std::sync::Arc::from(
            Vec::<FinalChordRenderOccurrenceWitness>::new(),
        ),
        actions: std::sync::Arc::from(context.actions.clone()),
        hard_same_edges: std::sync::Arc::from(context.hard_same_edges.clone()),
        measures: std::sync::Arc::from(context.measures.clone()),
        semantic_fingerprint_fnv64,
        unresolved_continuity_decisions: context.unresolved_continuity_decisions,
        unresolved_fragment_roles: context.unresolved_fragment_roles,
        unresolved_serialization_runs: context.unresolved_serialization_runs,
        unresolved_presentation_roles: context.unresolved_presentation_roles,
        unresolved_constraint_edges: context.unresolved_constraint_edges,
    };
    if !plan.voice_free_evidence_sufficiency_green() {
        let (continuity, fragments, runs, presentation, constraints) = plan.sufficiency_counts();
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "VoiceFreeEvidenceSufficiency blocked canonical plan: continuity={continuity} fragment_roles={fragments} serialization_runs={runs} presentation_roles={presentation} constraint_edges={constraints}"
            ),
        ));
    }

    let bound = final_chord_finalize_bound_plan(
        candidate_xml,
        edits,
        forward_materializations,
        false,
    )?;
    // Reuse the sole byte projector as an explicitly discarded read-only probe.
    // The unchanged render caller materializes the same bound plan; freeze checks
    // exact bytes before this carrier can authorize channel allocation.
    let projection = project_final_chord_encoding_plan_with_provenance(candidate_xml, &bound)?;
    plan.written_projection = Some(final_chord_bind_written_projection(
        candidate_xml,
        &plan,
        projection,
    )?);
    Ok((plan, bound))
}

fn final_chord_bind_written_projection(
    source_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    projection: FinalChordProjectionResult,
) -> io::Result<FinalChordWrittenProjectionEvidence> {
    let invalid = || io::Error::new(
        io::ErrorKind::InvalidData,
        "FinalChord written occurrence production provenance/positive interval coverage drift",
    );
    let original = final_chord_parse_binding_measures(source_xml)?;
    let source_notes = original.iter().flat_map(|measure| measure.notes.iter())
        .filter(|note| !note.rest_like && note.pitch_midi.is_some())
        .map(|note| ((note.measure, note.raw_note_index), note))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut locator_by_range = std::collections::BTreeMap::new();
    for locator in canonical.source_locators.iter() {
        let note = source_notes.get(&(locator.measure, locator.raw_note_index))
            .copied().ok_or_else(invalid)?;
        if note.onset != locator.onset || note.duration != locator.duration
            || note.pitch_identity != locator.pitch_identity || note.staff != locator.staff
            || locator_by_range.insert((note.range.start, note.range.end), locator).is_some()
        {
            return Err(invalid());
        }
    }
    if locator_by_range.len() != source_notes.len() {
        return Err(invalid());
    }
    let projected = final_chord_parse_binding_measures(&projection.xml)?;
    let mut projected_by_range = std::collections::BTreeMap::new();
    for measure in &projected {
        for note in &measure.notes {
            if note.rest_like || note.pitch_midi.is_none() {
                continue;
            }
            // A retained rejected encoding still has exact note provenance.
            // Its rest/orphan host provides no positive synchronous chord proof.
            let anchor = match note.anchor_index {
                Some(index) => measure.notes.get(index).ok_or_else(invalid)?,
                None => note,
            };
            let chord_anchor_end = anchor.onset.checked_add(anchor.duration).ok_or_else(invalid)?;
            let has_pitched_chord_anchor = !anchor.rest_like && anchor.pitch_midi.is_some()
                && !anchor.chord_member && anchor.duration > 0;
            if projected_by_range.insert(
                (note.range.start, note.range.end), (note, anchor.range.start,
                    anchor.onset, chord_anchor_end, has_pitched_chord_anchor),
            ).is_some() {
                return Err(invalid());
            }
        }
    }
    let mut occurrences = Vec::new();
    let mut represented_fragments = std::collections::BTreeSet::new();
    for row in projection.provenance {
        let locator = locator_by_range.get(&(row.source_note_start, row.source_note_end))
            .copied().ok_or_else(invalid)?;
        let (note, chord_anchor_start, chord_anchor_onset, chord_anchor_end,
            has_pitched_chord_anchor) = projected_by_range.remove(
            &(row.projected_note_start, row.projected_note_end),
        ).ok_or_else(invalid)?;
        let staff = note.staff.parse::<u8>().map_err(|_| invalid())?;
        let end = note.onset.checked_add(note.duration).ok_or_else(invalid)?;
        if note.duration <= 0 || !matches!(staff, 1 | 2) {
            return Err(invalid());
        }
        represented_fragments.insert(locator.fragment_id);
        occurrences.push(FinalChordWrittenOccurrence {
            source_event_id: locator.source_event_id,
            fragment_id: locator.fragment_id,
            attack_id: locator.attack_id,
            measure: note.measure,
            onset: note.onset,
            end,
            staff,
            pitch_midi: note.pitch_midi.ok_or_else(invalid)?,
            chord_anchor_start,
            chord_anchor_onset,
            chord_anchor_end,
            has_pitched_chord_anchor,
            projected_note_start: row.projected_note_start,
        });
    }
    let expected_fragments = canonical.source_locators.iter()
        .map(|locator| locator.fragment_id).collect::<std::collections::BTreeSet<_>>();
    if !projected_by_range.is_empty() || represented_fragments != expected_fragments {
        return Err(invalid());
    }
    // Non-pitched carriers have exact physical continuation, but no canonical
    // role origin in this plan. Validate coverage without promoting it to one.
    let expected_non_pitched_sources = original.iter().flat_map(|measure| &measure.notes)
        .filter(|note| note.rest_like || note.pitch_midi.is_none())
        .map(|note| (note.range.start, note.range.end))
        .collect::<std::collections::BTreeSet<_>>();
    let expected_non_pitched_notes = projected.iter().flat_map(|measure| &measure.notes)
        .filter(|note| note.rest_like || note.pitch_midi.is_none())
        .map(|note| (note.range.start, note.range.end))
        .collect::<std::collections::BTreeSet<_>>();
    let mut bound_sources = std::collections::BTreeSet::new();
    let mut bound_notes = std::collections::BTreeSet::new();
    for row in &projection.non_pitched_continuations {
        if row.presentation_role.is_some() || row.projected_start >= row.projected_end {
            return Err(invalid());
        }
        bound_sources.insert((row.source_note_start, row.source_note_end));
        if row.carrier == FinalChordNonPitchedCarrier::Note &&
            !bound_notes.insert((row.projected_start, row.projected_end)) {
            return Err(invalid());
        }
    }
    if bound_sources != expected_non_pitched_sources || bound_notes != expected_non_pitched_notes {
        return Err(invalid());
    }
    Ok(FinalChordWrittenProjectionEvidence {
        original_xml: std::sync::Arc::from(source_xml),
        xml: std::sync::Arc::from(projection.xml),
        occurrences: std::sync::Arc::from(occurrences),
    })
}



fn final_chord_project_ranges(
    source: &str,
    mut ranges: Vec<(usize, usize, String)>,
    drift_message: &'static str,
) -> io::Result<String> {
    ranges.sort_by_key(|row| row.0);
    let replacement_bytes = ranges.iter().map(|row| row.2.len()).sum::<usize>();
    let removed_bytes = ranges
        .iter()
        .map(|row| row.1.saturating_sub(row.0))
        .sum::<usize>();
    let mut out = String::with_capacity(
        source
            .len()
            .saturating_sub(removed_bytes)
            .saturating_add(replacement_bytes),
    );
    let mut cursor = 0usize;
    for (start, end, replacement) in ranges {
        if start < cursor || start > end || end > source.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, drift_message));
        }
        out.push_str(&source[cursor..start]);
        out.push_str(&replacement);
        cursor = end;
    }
    out.push_str(&source[cursor..]);
    Ok(out)
}

fn final_chord_note_voice_labels(candidate_xml: &str) -> io::Result<Vec<String>> {
    let document = roxmltree::Document::parse(candidate_xml).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("voice metamorph parse failed: {error}"),
        )
    })?;
    Ok(document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("note"))
        .map(|note| final_chord_direct_child_text(note, "voice").unwrap_or_else(|| "1".to_string()))
        .collect())
}

fn final_chord_map_note_voice_labels(
    candidate_xml: &str,
    replacement_labels: &[String],
) -> io::Result<String> {
    let document = roxmltree::Document::parse(candidate_xml).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("voice metamorph parse failed: {error}"),
        )
    })?;
    let notes = document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("note"))
        .collect::<Vec<_>>();
    if notes.len() != replacement_labels.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "voice metamorph replacement cardinality drift",
        ));
    }
    let mut edits = Vec::<(usize, usize, String)>::new();
    for (note, replacement) in notes.into_iter().zip(replacement_labels) {
        let voice = note
            .children()
            .find(|child| child.is_element() && child.has_tag_name("voice"))
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "voice metamorph found note without explicit voice",
                )
            })?;
        let range = voice.range();
        edits.push((
            range.start,
            range.end,
            format!("<voice>{replacement}</voice>"),
        ));
    }
    let out = final_chord_project_ranges(
        candidate_xml,
        edits,
        "voice metamorph byte range drift",
    )?;
    validate_final_chord_musicxml(&out)?;
    Ok(out)
}

fn final_chord_all_permutations(values: &[String]) -> Vec<Vec<String>> {
    fn visit(prefix: &mut Vec<String>, rest: &mut Vec<String>, out: &mut Vec<Vec<String>>) {
        if rest.is_empty() {
            out.push(prefix.clone());
            return;
        }
        for index in 0..rest.len() {
            let value = rest.remove(index);
            prefix.push(value.clone());
            visit(prefix, rest, out);
            prefix.pop();
            rest.insert(index, value);
        }
    }
    let mut out = Vec::new();
    let mut prefix = Vec::new();
    let mut rest = values.to_vec();
    visit(&mut prefix, &mut rest, &mut out);
    out
}

fn final_chord_label_alpha_variants(candidate_xml: &str) -> io::Result<Vec<String>> {
    let original = final_chord_note_voice_labels(candidate_xml)?;
    let labels = original.iter().cloned().collect::<std::collections::BTreeSet<_>>()
        .into_iter().collect::<Vec<_>>();
    if labels.len() <= 1 {
        return Ok(vec![candidate_xml.to_string()]);
    }
    let permutations = if labels.len() <= 4 {
        final_chord_all_permutations(&labels)
    } else {
        let identity = labels.clone();
        let mut reverse = labels.clone();
        reverse.reverse();
        let mut swap = labels.clone();
        let last = swap.len() - 1;
        swap.swap(0, last);
        let mut cycle = labels.clone();
        cycle.rotate_left(1);
        vec![identity, reverse, swap, cycle]
    };
    let mut variants = Vec::new();
    for permutation in permutations {
        let mapping = labels
            .iter()
            .cloned()
            .zip(permutation)
            .collect::<std::collections::BTreeMap<_, _>>();
        let replacements = original
            .iter()
            .map(|label| mapping.get(label).cloned().unwrap_or_else(|| label.clone()))
            .collect::<Vec<_>>();
        variants.push(final_chord_map_note_voice_labels(candidate_xml, &replacements)?);
    }
    Ok(variants)
}

fn final_chord_partition_variants(candidate_xml: &str) -> io::Result<Vec<String>> {
    let original = final_chord_note_voice_labels(candidate_xml)?;
    let len = original.len();
    if len == 0 {
        return Ok(Vec::new());
    }
    let all_same = vec!["990001".to_string(); len];
    let all_unique = (0..len)
        .map(|index| format!("991{:06}", index + 1))
        .collect::<Vec<_>>();
    let repartition_a = (0..len)
        .map(|index| format!("99200{}", (index * 2 + 1) % 3))
        .collect::<Vec<_>>();
    let repartition_b = (0..len)
        .map(|index| format!("99300{}", (index * 5 + 2) % 4))
        .collect::<Vec<_>>();
    [all_same, all_unique, repartition_a, repartition_b]
        .into_iter()
        .map(|labels| final_chord_map_note_voice_labels(candidate_xml, &labels))
        .collect()
}

pub(crate) fn audit_voice_cutover_noninterference(
    candidate_xml: &str,
    evidence: &[VoiceFreeFinalChordEvidenceRow],
    baseline: &CanonicalFinalChordStructuralPlan,
) -> io::Result<VoiceAuthorityCutoverMetamorphicAudit> {
    if !baseline.voice_free_evidence_sufficiency_green() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "voice non-interference audit requires green canonical baseline",
        ));
    }
    let baseline_fingerprint = baseline.semantic_fingerprint_fnv64();
    let mut audit = VoiceAuthorityCutoverMetamorphicAudit::default();

    for variant in final_chord_label_alpha_variants(candidate_xml)? {
        audit.label_alpha_cases = audit.label_alpha_cases.saturating_add(1);
        let (plan, _) = plan_canonical_final_chord_structure_once(&variant, evidence)?;
        if plan.semantic_fingerprint_fnv64() != baseline_fingerprint {
            audit.label_alpha_semantic_deltas =
                audit.label_alpha_semantic_deltas.saturating_add(1);
        }
    }

    for variant in final_chord_partition_variants(candidate_xml)? {
        audit.legacy_partition_cases = audit.legacy_partition_cases.saturating_add(1);
        let (plan, _) = plan_canonical_final_chord_structure_once(&variant, evidence)?;
        if plan.semantic_fingerprint_fnv64() != baseline_fingerprint {
            audit.legacy_partition_semantic_deltas =
                audit.legacy_partition_semantic_deltas.saturating_add(1);
        }
    }

    let mut renamed = evidence.to_vec();
    let mut fragment_ids = renamed
        .iter()
        .map(|row| row.fragment_id)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut source_ids = renamed
        .iter()
        .map(|row| row.source_event_id)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut attack_ids = renamed
        .iter()
        .map(|row| row.attack_id)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let fragment_mapping = fragment_ids
        .iter()
        .copied()
        .zip((0..fragment_ids.len()).rev().map(|index| 1_000_000usize.saturating_add(index)))
        .collect::<std::collections::BTreeMap<_, _>>();
    let source_mapping = source_ids
        .iter()
        .copied()
        .zip((0..source_ids.len()).rev().map(|index| 2_000_000usize.saturating_add(index)))
        .collect::<std::collections::BTreeMap<_, _>>();
    let attack_mapping = attack_ids
        .iter()
        .copied()
        .zip((0..attack_ids.len()).rev().map(|index| 3_000_000usize.saturating_add(index)))
        .collect::<std::collections::BTreeMap<_, _>>();
    for row in &mut renamed {
        row.fragment_id = fragment_mapping[&row.fragment_id];
        row.source_event_id = source_mapping[&row.source_event_id];
        row.attack_id = attack_mapping[&row.attack_id];
    }
    fragment_ids.clear();
    source_ids.clear();
    attack_ids.clear();
    audit.opaque_identity_rename_cases = 1;
    let (renamed_plan, _) = plan_canonical_final_chord_structure_once(candidate_xml, &renamed)?;
    if renamed_plan.semantic_fingerprint_fnv64() != baseline_fingerprint {
        audit.opaque_identity_semantic_deltas = 1;
    }

    audit.green = audit.label_alpha_semantic_deltas == 0
        && audit.legacy_partition_semantic_deltas == 0
        && audit.opaque_identity_semantic_deltas == 0;
    Ok(audit)
}

fn final_chord_union_find_root(
    parent: &mut std::collections::BTreeMap<usize, usize>,
    value: usize,
) -> usize {
    let mut current = value;
    while parent.get(&current).copied().unwrap_or(current) != current {
        current = parent.get(&current).copied().unwrap_or(current);
    }
    let root = current;
    current = value;
    while parent.get(&current).copied().unwrap_or(current) != current {
        let next = parent.get(&current).copied().unwrap_or(current);
        parent.insert(current, root);
        current = next;
    }
    root
}

fn final_chord_union_find_join(
    parent: &mut std::collections::BTreeMap<usize, usize>,
    left: usize,
    right: usize,
) {
    parent.entry(left).or_insert(left);
    parent.entry(right).or_insert(right);
    let left_root = final_chord_union_find_root(parent, left);
    let right_root = final_chord_union_find_root(parent, right);
    if left_root != right_root {
        // The numeric SourceEventId is only a deterministic representative choice;
        // it cannot affect any structural role or constraint decision.
        let (keep, replace) = if left_root <= right_root {
            (left_root, right_root)
        } else {
            (right_root, left_root)
        };
        parent.insert(replace, keep);
    }
}

fn final_chord_default_presentation_role(
    staff: u8,
    technical_lane: usize,
) -> io::Result<SerializedVoicePresentationRole> {
    match (staff, technical_lane) {
        (1, 1) => Ok(SerializedVoicePresentationRole::UpperPrimary),
        (2, 1) => Ok(SerializedVoicePresentationRole::LowerPrimary),
        (1, _) => Ok(SerializedVoicePresentationRole::UpperSecondary),
        (2, _) => Ok(SerializedVoicePresentationRole::LowerSecondary),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan received unsupported staff",
        )),
    }
}

fn final_chord_auxiliary_presentation_role(
    staff: u8,
) -> io::Result<SerializedVoicePresentationRole> {
    match staff {
        1 => Ok(SerializedVoicePresentationRole::UpperAuxiliary),
        2 => Ok(SerializedVoicePresentationRole::LowerAuxiliary),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan auxiliary role received unsupported staff",
        )),
    }
}

pub(crate) fn build_frozen_serialization_voice_channels_for_canonical(
    canonical: &CanonicalFinalChordStructuralPlan,
    attacks: &[FrozenVoiceAttackEvidenceRow],
) -> io::Result<FrozenSerializationVoiceChannelPlan> {
    build_frozen_serialization_voice_channels_with_original_beams(canonical, attacks, None, &[])
}

pub(crate) fn build_frozen_serialization_voice_channels_with_original_beams(
    canonical: &CanonicalFinalChordStructuralPlan,
    attacks: &[FrozenVoiceAttackEvidenceRow],
    original_beams: Option<&ActualSourceBeamBindingObservation>,
    legacy_preferences: &[LegacyVoiceLabelPreferenceRow],
) -> io::Result<FrozenSerializationVoiceChannelPlan> {
    if !canonical.voice_free_evidence_sufficiency_green() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan requires a bound VoiceFreeEvidenceSufficiency-green canonical plan",
        ));
    }
    if attacks.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan received no frozen attacks",
        ));
    }

    let mut attack_by_source = std::collections::BTreeMap::<usize, &FrozenVoiceAttackEvidenceRow>::new();
    let mut attack_ids = std::collections::BTreeSet::<usize>::new();
    for attack in attacks {
        if attack.absolute_end <= attack.absolute_start
            || !matches!(attack.staff, 1 | 2)
            || attack.technical_lane == 0
            || attack_by_source.insert(attack.source_event_id, attack).is_some()
            || !attack_ids.insert(attack.attack_id)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FrozenSerializedVoicePlan frozen attack identity/temporal coverage is invalid",
            ));
        }
    }

    for locator in canonical.source_locators.iter() {
        if !attack_by_source.contains_key(&locator.source_event_id) {
            let action = canonical
                .actions
                .iter()
                .find(|action| action.source_event_id == locator.source_event_id);
            let same_measure_pitch_staff = canonical
                .source_locators
                .iter()
                .filter(|candidate| {
                    candidate.fragment_id != locator.fragment_id
                        && candidate.measure == locator.measure
                        && candidate.pitch_midi == locator.pitch_midi
                        && candidate.staff == locator.staff
                })
                .map(|candidate| {
                    format!(
                        "src={} attack={} frag={} raw={} onset={} dur={} tie_start={} tie_stop={} frozen_attack={}",
                        candidate.source_event_id,
                        candidate.attack_id,
                        candidate.fragment_id,
                        candidate.raw_note_index,
                        candidate.onset,
                        candidate.duration,
                        candidate.tie_start,
                        candidate.tie_stop,
                        attack_by_source.contains_key(&candidate.source_event_id),
                    )
                })
                .collect::<Vec<_>>()
                .join(";");
            let adjacent_tie_starts = canonical
                .source_locators
                .iter()
                .filter(|candidate| {
                    candidate.fragment_id != locator.fragment_id
                        && candidate.pitch_midi == locator.pitch_midi
                        && candidate.staff == locator.staff
                        && candidate.tie_start
                        && candidate.measure.abs_diff(locator.measure) <= 1
                })
                .map(|candidate| {
                    format!(
                        "m{}:src={} attack={} frag={} raw={} onset={} end={} frozen_attack={}",
                        candidate.measure,
                        candidate.source_event_id,
                        candidate.attack_id,
                        candidate.fragment_id,
                        candidate.raw_note_index,
                        candidate.onset,
                        candidate.onset.saturating_add(candidate.duration.max(0)),
                        attack_by_source.contains_key(&candidate.source_event_id),
                    )
                })
                .collect::<Vec<_>>()
                .join(";");
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FrozenSerializedVoicePlan: existing pitched canonical carrier SourceEventId {} is absent from Frozen Temporal Truth; fragment_id={} attack_id={} measure={} raw_note_index={} onset={} duration={} pitch={} staff={} chord={} tie_start={} tie_stop={} representation_role={:?} host_source_event_id={:?} action_kind={:?} same_measure_pitch_staff=[{}] adjacent_tie_starts=[{}]",
                    locator.source_event_id,
                    locator.fragment_id,
                    locator.attack_id,
                    locator.measure,
                    locator.raw_note_index,
                    locator.onset,
                    locator.duration,
                    locator.pitch_midi,
                    locator.staff,
                    locator.chord_syntax,
                    locator.tie_start,
                    locator.tie_stop,
                    action.map(|action| action.representation_role),
                    action.and_then(|action| action.host_source_event_id),
                    action.map(|action| &action.kind),
                    same_measure_pitch_staff,
                    adjacent_tie_starts,
                ),
            ));
        }
    }

    let mut parent = attack_by_source
        .keys()
        .copied()
        .map(|source| (source, source))
        .collect::<std::collections::BTreeMap<_, _>>();
    let source_by_fragment = canonical
        .source_locators
        .iter()
        .map(|locator| (locator.fragment_id, locator.source_event_id))
        .collect::<std::collections::BTreeMap<_, _>>();
    if source_by_fragment.len() != canonical.source_locators.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan canonical FragmentId coverage is not unique",
        ));
    }
    for edge in canonical.hard_same_edges.iter() {
        let left_source_event_id = *source_by_fragment.get(&edge.left_fragment_id).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "FrozenSerializedVoicePlan HARD SAME left FragmentId is absent")
        })?;
        let right_source_event_id = *source_by_fragment.get(&edge.right_fragment_id).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "FrozenSerializedVoicePlan HARD SAME right FragmentId is absent")
        })?;
        if !attack_by_source.contains_key(&left_source_event_id)
            || !attack_by_source.contains_key(&right_source_event_id)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FrozenSerializedVoicePlan HARD SAME endpoint is absent from frozen attacks",
            ));
        }
        final_chord_union_find_join(
            &mut parent,
            left_source_event_id,
            right_source_event_id,
        );
    }

    let sources = parent.keys().copied().collect::<Vec<_>>();
    let mut class_by_source = std::collections::BTreeMap::<usize, usize>::new();
    for source in sources {
        let root = final_chord_union_find_root(&mut parent, source);
        class_by_source.insert(source, root);
    }
    let mut members_by_class = std::collections::BTreeMap::<usize, Vec<usize>>::new();
    for (&source, &class) in &class_by_source {
        members_by_class.entry(class).or_default().push(source);
    }

    let auxiliary_sources = canonical
        .actions
        .iter()
        .filter(|action| {
            action.representation_role == FinalChordRepresentationRole::IndependentAuxiliaryFragment
        })
        .map(|action| action.source_event_id)
        .collect::<std::collections::BTreeSet<_>>();
    let mut hard_presentation_staff = std::collections::BTreeMap::<usize, u8>::new();
    for edge in canonical.hard_same_edges.iter() {
        let left_source_event_id = *source_by_fragment.get(&edge.left_fragment_id).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "missing HARD SAME fragment/source bridge")
        })?;
        let class = *class_by_source.get(&left_source_event_id).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "missing HARD SAME class")
        })?;
        if let Some(previous) = hard_presentation_staff.insert(class, edge.presentation_staff)
            && previous != edge.presentation_staff
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VoiceFreeEvidenceSufficiency: one HARD SAME class requires incompatible presentation staffs",
            ));
        }
    }

    let mut class_role = std::collections::BTreeMap::<usize, SerializedVoicePresentationRole>::new();
    for (&class, members) in &members_by_class {
        let contains_auxiliary = members.iter().any(|source| auxiliary_sources.contains(source));
        if contains_auxiliary && members.iter().any(|source| !auxiliary_sources.contains(source)) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VoiceFreeEvidenceSufficiency: auxiliary and retained material collapsed into one HARD SAME class",
            ));
        }
        let role = if contains_auxiliary {
            let staff = members
                .iter()
                .filter_map(|source| attack_by_source.get(source).map(|attack| attack.staff))
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty auxiliary class"))?;
            if members
                .iter()
                .filter_map(|source| attack_by_source.get(source).map(|attack| attack.staff))
                .any(|candidate| candidate != staff)
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VoiceFreeEvidenceSufficiency: one auxiliary run spans incompatible staffs",
                ));
            }
            final_chord_auxiliary_presentation_role(staff)?
        } else if let Some(&presentation_staff) = hard_presentation_staff.get(&class) {
            match presentation_staff {
                1 => SerializedVoicePresentationRole::UpperPrimary,
                2 => SerializedVoicePresentationRole::LowerPrimary,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "VoiceFreeEvidenceSufficiency: invalid HARD SAME presentation staff",
                    ));
                }
            }
        } else {
            let first = attack_by_source
                .get(&members[0])
                .copied()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty voice class"))?;
            final_chord_default_presentation_role(first.staff, first.technical_lane)?
        };
        class_role.insert(class, role);
    }

    // Preserve the accepted class role before assigning any technical channel.
    // This source binding is not a complete rest/notation-occurrence relation.
    let source_to_role = class_by_source.iter()
        .map(|(&source, class)| (source, class_role[class]))
        .collect::<std::collections::BTreeMap<_, _>>();

    let written = canonical.written_projection.as_ref().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan requires complete written production evidence")
    })?;
    if written.occurrences.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan written occurrence evidence is empty"));
    }
    for row in written.occurrences.iter() {
        if row.end <= row.onset
            || source_by_fragment.get(&row.fragment_id) != Some(&row.source_event_id)
            || attack_by_source.get(&row.source_event_id).map(|attack| attack.attack_id)
                != Some(row.attack_id)
        {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                "FrozenSerializedVoicePlan written occurrence identity/interval drift"));
        }
    }
    let represented_fragments = written.occurrences.iter().map(|row| row.fragment_id)
        .collect::<std::collections::BTreeSet<_>>();
    let expected_fragments = source_by_fragment.keys().copied()
        .collect::<std::collections::BTreeSet<_>>();
    if represented_fragments != expected_fragments {
        return Err(io::Error::new(io::ErrorKind::InvalidData,
            "FrozenSerializedVoicePlan written occurrence coverage is incomplete"));
    }
    let mut written_by_measure = std::collections::BTreeMap::<usize, Vec<&FinalChordWrittenOccurrence>>::new();
    for row in written.occurrences.iter() {
        written_by_measure.entry(row.measure).or_default().push(row);
    }
    let mut written_different = std::collections::BTreeSet::<(usize, usize)>::new();
    for occurrences in written_by_measure.values() {
        for (index, left) in occurrences.iter().enumerate() {
            for right in occurrences.iter().skip(index + 1) {
                if left.measure != right.measure || left.onset >= right.end || right.onset >= left.end {
                    continue;
                }
                let left_class = class_by_source[&left.source_event_id];
                let right_class = class_by_source[&right.source_event_id];
                // Geometry alone never grants joint chord authority. Require both
                // the exact projected chord cohort and the canonical HARD SAME class.
                let authorized_chord = left_class == right_class
                    && left.has_pitched_chord_anchor && right.has_pitched_chord_anchor
                    && left.chord_anchor_start == right.chord_anchor_start
                    && left.chord_anchor_onset == right.chord_anchor_onset
                    && left.chord_anchor_end == right.chord_anchor_end
                    && left.onset == left.chord_anchor_onset
                    && right.onset == right.chord_anchor_onset
                    && left.end <= left.chord_anchor_end && right.end <= right.chord_anchor_end;
                if authorized_chord {
                    continue;
                }
                if left_class == right_class {
                    return Err(io::Error::new(io::ErrorKind::InvalidData,
                        format!("FrozenSerializedVoicePlan HARD SAME class has incompatible written obligations: measure={} class={} left=source{} fragment{} written{}..{} anchor{}:{}..{} pitched{} right=source{} fragment{} written{}..{} anchor{}:{}..{} pitched{}",
                            left.measure, left_class, left.source_event_id, left.fragment_id,
                            left.onset, left.end, left.chord_anchor_start, left.chord_anchor_onset,
                            left.chord_anchor_end, left.has_pitched_chord_anchor,
                            right.source_event_id, right.fragment_id, right.onset, right.end,
                            right.chord_anchor_start, right.chord_anchor_onset, right.chord_anchor_end,
                            right.has_pitched_chord_anchor)));
                }
                written_different.insert((left_class.min(right_class), left_class.max(right_class)));
            }
        }
    }
    let classes = members_by_class.keys().copied().collect::<Vec<_>>();
    let mut semantic_different = std::collections::BTreeSet::<(usize, usize)>::new();
    for left_index in 0..classes.len() {
        for right_index in left_index + 1..classes.len() {
            let left_class = classes[left_index];
            let right_class = classes[right_index];
            let overlaps = members_by_class[&left_class].iter().any(|left_source| {
                let left = attack_by_source[left_source];
                members_by_class[&right_class].iter().any(|right_source| {
                    let right = attack_by_source[right_source];
                    left.staff == right.staff
                        && left.absolute_start < right.absolute_end
                        && right.absolute_start < left.absolute_end
                })
            });
            if overlaps {
                semantic_different.insert((left_class.min(right_class), left_class.max(right_class)));
            }
        }
    }

    let mut channel_incompatible = std::collections::BTreeSet::<(usize, usize)>::new();
    for left_index in 0..classes.len() {
        for right_index in left_index + 1..classes.len() {
            let left = classes[left_index];
            let right = classes[right_index];
            if class_role[&left] != class_role[&right] {
                channel_incompatible.insert((left.min(right), left.max(right)));
            }
        }
    }

    for &(left, right) in &semantic_different {
        if left == right {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FrozenSerializedVoicePlan HARD SAME class contains SemanticDifferent conflict",
            ));
        }
    }

    let mut chord_signatures = std::collections::BTreeMap::<(usize, usize), Vec<String>>::new();
    for row in written.occurrences.iter() {
        chord_signatures.entry((row.measure, row.chord_anchor_start)).or_default()
            .push(format!("{}..{}:s{}:p{}:anchor{}..{}:pitched_anchor{}", row.onset, row.end,
                row.staff, row.pitch_midi, row.chord_anchor_onset, row.chord_anchor_end,
                u8::from(row.has_pitched_chord_anchor)));
    }
    for rows in chord_signatures.values_mut() {
        rows.sort();
    }
    let mut written_signatures_by_class = std::collections::BTreeMap::<usize, Vec<String>>::new();
    for row in written.occurrences.iter() {
        written_signatures_by_class.entry(class_by_source[&row.source_event_id]).or_default()
            .push(format!("W|m{}:{}..{}:s{}:p{}:chord=[{}]", row.measure,
                row.onset, row.end, row.staff, row.pitch_midi,
                chord_signatures[&(row.measure, row.chord_anchor_start)].join(",")));
    }
    let mut allocation_signatures = std::collections::BTreeMap::<usize, Vec<String>>::new();
    for row in written.occurrences.iter() {
        allocation_signatures.entry(class_by_source[&row.source_event_id]).or_default()
            .push(format!("W|m{}:{}..{}:s{}", row.measure, row.onset, row.end, row.staff));
    }
    let mut class_signatures = std::collections::BTreeMap::<usize, String>::new();
    for (&class, members) in &members_by_class {
        let mut spans = members.iter().map(|source| {
            let attack = attack_by_source[source];
            format!("S|s{}:{}..{}:lane{}", attack.staff,
                attack.absolute_start, attack.absolute_end, attack.technical_lane)
        }).collect::<Vec<_>>();
        let allocation = allocation_signatures.entry(class).or_default();
        allocation.extend(spans.iter().cloned());
        allocation.sort();
        spans.extend(written_signatures_by_class.remove(&class).unwrap_or_default());
        spans.sort();
        class_signatures.insert(class, format!("CLASS|role={}|{}",
            class_role[&class].label(), spans.join(",")));
    }

    let mut class_order = classes.clone();
    class_order.sort_by_cached_key(|class| {
        let members = &members_by_class[class];
        let min_start = members
            .iter()
            .map(|source| attack_by_source[source].absolute_start)
            .min()
            .unwrap_or(i64::MAX);
        let max_end = members
            .iter()
            .map(|source| attack_by_source[source].absolute_end)
            .max()
            .unwrap_or(i64::MIN);
        let role = class_role[class];
        // Use temporal obligations, not pitch/register, before the final opaque
        // naming tie-break. Equal temporal descriptions have equal external
        // conflict neighborhoods; ids never establish a role or conflict edge.
        (min_start, max_end, role.sort_rank(), allocation_signatures[class].clone(), *class)
    });

    let class_order_for_search = class_order.clone();
    let mut channel_by_class = std::collections::BTreeMap::<usize, usize>::new();
    let mut channel_role = std::collections::BTreeMap::<usize, SerializedVoicePresentationRole>::new();
    let mut next_channel = 1usize;
    for class in class_order {
        let role = class_role[&class];
        let preferred_technical_lane = members_by_class[&class]
            .iter()
            .filter_map(|source| attack_by_source.get(source).map(|attack| attack.technical_lane))
            .min()
            .unwrap_or(usize::MAX);
        let conflicts_with = |other_class: usize| {
            let pair = (class.min(other_class), class.max(other_class));
            semantic_different.contains(&pair) || channel_incompatible.contains(&pair)
                || written_different.contains(&pair)
        };
        let blocked_channels = channel_by_class.iter()
            .filter_map(|(&member, &channel)| conflicts_with(member).then_some(channel))
            .collect::<std::collections::BTreeSet<_>>();
        let mut reusable = channel_by_class
            .iter()
            .filter_map(|(&other_class, &channel)| {
                if channel_role.get(&channel).copied() != Some(role)
                    || blocked_channels.contains(&channel)
                {
                    return None;
                }
                let other_lane = members_by_class[&other_class]
                    .iter()
                    .filter_map(|source| attack_by_source.get(source).map(|attack| attack.technical_lane))
                    .min()
                    .unwrap_or(usize::MAX);
                Some((usize::from(other_lane != preferred_technical_lane), channel))
            })
            .collect::<Vec<_>>();
        reusable.sort_unstable();
        let channel = reusable.first().map(|(_, channel)| *channel).unwrap_or_else(|| {
            let channel = next_channel;
            next_channel = next_channel.saturating_add(1);
            channel_role.insert(channel, role);
            channel
        });
        channel_by_class.insert(class, channel);
    }

    // The greedy assignment above is the first candidate of a deterministic
    // finite alternative. Only if the consumer would leave a required channel unreserved
    // does the search continue, under exactly the same edges and roles.
    let (mut channel_by_class, mut channel_role, mut consumer_allocation) =
        match channel_consumer_model(written, &class_by_source) {
            None => (
                channel_by_class,
                channel_role,
                ChannelConsumerAllocation::Undetermined(
                    "written projection is not readable as complete consumer input",
                ),
            ),
            Some(model) => match channel_consumer_overflow(&model, &channel_by_class, &channel_role) {
                Ok(()) => (channel_by_class, channel_role, ChannelConsumerAllocation::AdmittedFirstCandidate),
                Err(failing_staves) => channel_search_after_consumer_overflow(
                    &ChannelSearchInput {
                        model: &model,
                        class_order: &class_order_for_search,
                        class_role: &class_role,
                        semantic_different: &semantic_different,
                        written_different: &written_different,
                    },
                    &failing_staves,
                    channel_by_class,
                    channel_role,
                ),
            },
        };
    let mut label_continuation = None;
    if let (Some(observation), Some(model)) = (original_beams, channel_consumer_model(written, &class_by_source))
        && channel_consumer_overflow(&model, &channel_by_class, &channel_role).is_err()
    {
        let input = ChannelSearchInput { model: &model, class_order: &class_order_for_search,
            class_role: &class_role, semantic_different: &semantic_different,
            written_different: &written_different };
        let context = cross_role_channel_sharing::SharingContext::new(
            canonical, observation, &class_by_source, &source_to_role, legacy_preferences,
        );
        let eligible = context.eligible_classes();
        match context.search(&input, &channel_by_class, &channel_role, &eligible) {
            Some((shared, roles, moved, baseline_voices)) => {
                consumer_allocation = ChannelConsumerAllocation::AdmittedAfterCrossRoleSharing {
                    channels_before: channel_role.len(), channels_after: roles.len(), shared_classes: moved,
                };
                label_continuation = Some(baseline_voices);
                channel_by_class = shared;
                channel_role = roles;
            }
            None => consumer_allocation = ChannelConsumerAllocation::NoAdmittedCrossRolePartition {
                eligible_classes: eligible.len(), classes: class_by_source.values().copied().collect::<std::collections::BTreeSet<_>>().len(),
            },
        }
    }
    let channel_count = channel_role.len();

    let mut source_to_channel = std::collections::BTreeMap::new();
    for (&source, &class) in &class_by_source {
        source_to_channel.insert(source, channel_by_class[&class]);
    }

    let mut semantic_signature = String::from("ETUSCHLOPF_SERIALIZATION_CHANNEL_WRITTEN_SEMANTICS_V2\n");
    let mut sorted_classes = class_signatures.values().cloned().collect::<Vec<_>>();
    sorted_classes.sort();
    for row in &sorted_classes {
        let _ = writeln!(semantic_signature, "{row}");
    }
    // Semantic names compress the dense role relation without hashing class
    // identities or duplicating long occurrence descriptors for every edge.
    sorted_classes.dedup();
    let semantic_names = sorted_classes.into_iter().enumerate()
        .map(|(index, row)| (row, index)).collect::<std::collections::BTreeMap<_, _>>();
    for (kind, edges) in [
        ("SoundingDifferent", &semantic_different),
        ("RoleIncompatible", &channel_incompatible),
        ("WrittenDifferent", &written_different),
    ] {
        let mut rows = edges.iter().map(|(left, right)| {
            let left = semantic_names[&class_signatures[left]];
            let right = semantic_names[&class_signatures[right]];
            (left.min(right), left.max(right))
        }).collect::<Vec<_>>();
        rows.sort_unstable();
        for (left, right) in rows {
            let _ = writeln!(semantic_signature, "{kind}|{left}|{right}");
        }
    }

    let mut partitions = std::collections::BTreeMap::<usize, Vec<String>>::new();
    for (&class, &channel) in &channel_by_class {
        partitions.entry(channel).or_default().push(class_signatures[&class].clone());
    }
    let mut partitions = partitions.into_values().map(|mut members| {
        members.sort();
        members.join(";")
    }).collect::<Vec<_>>();
    partitions.sort();
    for partition in partitions {
        let _ = writeln!(semantic_signature, "CHANNEL|{partition}");
    }

    Ok(FrozenSerializationVoiceChannelPlan {
        authority_bound: true,
        source_to_channel,
        source_to_role,
        channel_to_role: channel_role,
        channel_count,
        semantic_fingerprint_fnv64: fingerprint(&semantic_signature),
        consumer_allocation,
        label_continuation,
    })
}

// ---------------------------------------------------------------------------
// Consumer admission of the frozen channel partition.
//
// The consumer (MuseScore 4.7.4 MusicXML import) allocates whole-part voice
// labels to at most four voices per staff. `allocateStaves` calls the no-op
// `VoiceDesc::setStaff(-2)` for a regular voice that does not fit, so that
// voice and every smaller regular voice stay unallocated. A partition is
// therefore admitted only if no staff can receive more than four voices.
//
// Inputs are exactly the sealed written projection: pitched notes are bound to
// their class through carried provenance (`projected_note_start`); non-pitched
// carriers keep their unchanged bytes and therefore their written voice label.
// They receive no role. Their label joins the channel that owns the identical
// reserved canonical label (owner chosen from roles only, independent of legacy
// preferences); any other label is counted as a voice of its own.
//
// The later visible-rest projection only splits a rest into parts inside its
// own interval with the same staff and label, so it can remove staff presence
// and cross-staff overlap but never add either. The admission below counts
// every staff on which a voice has a chordrest, which bounds the consumer's
// allocation from above whatever preferred staff or overlap status results.
// ---------------------------------------------------------------------------

const CONSUMER_VOICES_PER_STAFF: usize = 4;

/// Read-only consumer evidence of the channel allocation. It never grants a
/// role, a staff, a label or a writer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ChannelConsumerAllocation {
    AdmittedAfterCrossRoleSharing { channels_before: usize, channels_after: usize, shared_classes: Vec<usize> },
    NoAdmittedCrossRolePartition { eligible_classes: usize, classes: usize },
    /// The first search candidate (the accepted greedy order) is admitted.
    AdmittedFirstCandidate,
    /// The first candidate overflowed a staff; a later candidate is admitted.
    AdmittedAfterSearch {
        channels_before: usize,
        channels_after: usize,
    },
    /// Every partition admissible under the existing edges and roles overflows
    /// this staff. Each clique is a set of classes (opaque root ids) of one
    /// pinned role whose members pairwise require distinct channels.
    SlotObligationWitness {
        staff: u8,
        demand: usize,
        cliques: Vec<(SerializedVoicePresentationRole, Vec<usize>)>,
    },
    /// No admitted candidate was found and no proof applies; the first
    /// candidate is kept unchanged.
    Undetermined(&'static str),
}

impl ChannelConsumerAllocation {
    pub(crate) const fn label(&self) -> &'static str {
        match self {
            Self::AdmittedFirstCandidate => "admitted_first_candidate",
            Self::AdmittedAfterCrossRoleSharing { .. } => "admitted_after_cross_role_sharing",
            Self::NoAdmittedCrossRolePartition { .. } => "no_admitted_cross_role_partition",
            Self::AdmittedAfterSearch { .. } => "admitted_after_search",
            Self::SlotObligationWitness { .. } => "slot_obligation_witness",
            Self::Undetermined(_) => "undetermined",
        }
    }
}

#[derive(Debug, Clone)]
struct ChannelConsumerNote {
    staff: u8,
    chordrest: bool,
    required: bool,
    class: Option<usize>,
    label: String,
}

#[derive(Debug, Clone)]
struct ChannelConsumerModel {
    notes: Vec<ChannelConsumerNote>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ChannelConsumerVoice {
    Channel(usize),
    Free(String),
}

fn channel_consumer_label_key(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let trimmed = if trimmed.is_empty() { "1" } else { trimmed };
    let value = trimmed.parse::<u32>().ok().filter(|value| *value > 0)?;
    let canonical = value.to_string();
    (trimmed == canonical).then_some(canonical)
}

const fn channel_role_staff(role: SerializedVoicePresentationRole) -> u8 {
    match role {
        SerializedVoicePresentationRole::UpperPrimary
        | SerializedVoicePresentationRole::UpperSecondary
        | SerializedVoicePresentationRole::UpperAuxiliary => 1,
        SerializedVoicePresentationRole::LowerPrimary
        | SerializedVoicePresentationRole::LowerSecondary
        | SerializedVoicePresentationRole::LowerAuxiliary => 2,
    }
}

fn channel_consumer_model(
    written: &FinalChordWrittenProjectionEvidence,
    class_by_source: &std::collections::BTreeMap<usize, usize>,
) -> Option<ChannelConsumerModel> {
    let document = roxmltree::Document::parse(&written.xml).ok()?;
    if document.root_element().children().filter(|node| node.has_tag_name("part")).count() != 1 {
        return None;
    }
    let measures = final_chord_parse_binding_measures(&written.xml).ok()?;
    let mut class_by_note = std::collections::BTreeMap::<usize, usize>::new();
    for row in written.occurrences.iter() {
        let class = *class_by_source.get(&row.source_event_id)?;
        if class_by_note.insert(row.projected_note_start, class).is_some() {
            return None;
        }
    }
    let mut notes = Vec::new();
    let mut claimed = 0usize;
    for measure in &measures {
        for note in &measure.notes {
            let fragment = written.xml.get(note.range.clone())?;
            let document = roxmltree::Document::parse(fragment).ok()?;
            let element = document.root_element();
            let participates = element.attribute("print-spacing") != Some("no");
            let grace = element
                .children()
                .any(|child| child.is_element() && child.has_tag_name("grace"));
            let staff = note.staff.trim().parse::<u8>().ok().filter(|staff| matches!(staff, 1 | 2))?;
            let chordrest = participates && !grace && !note.chord_member && note.duration > 0;
            let pitched = !note.rest_like && note.pitch_midi.is_some();
            let class = if pitched {
                claimed = claimed.saturating_add(1);
                Some(class_by_note.get(&note.range.start).copied()?)
            } else {
                None
            };
            notes.push(ChannelConsumerNote {
                staff,
                chordrest,
                required: pitched && participates,
                class,
                label: channel_consumer_label_key(&note.voice)?,
            });
        }
    }
    (claimed == class_by_note.len()).then_some(ChannelConsumerModel { notes })
}

fn channel_reserved_label_owners(
    channel_role: &std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
) -> std::collections::BTreeMap<&'static str, usize> {
    // Mirrors `bind_musicxml_voice_labels`: reserved canonical labels depend on
    // channel roles and channel order only, never on legacy preferences.
    let mut order = channel_role.keys().copied().collect::<Vec<_>>();
    order.sort_by_key(|channel| (channel_role[channel].sort_rank(), *channel));
    let mut owners = std::collections::BTreeMap::new();
    for channel in order {
        owners
            .entry(channel_role[&channel].canonical_label_preference())
            .or_insert(channel);
    }
    owners
}

/// Ok if no staff can receive more voices than the consumer reserves and every
/// required pitched note maps to a reserved voice; otherwise the failing staves.
fn channel_consumer_overflow(
    model: &ChannelConsumerModel,
    channel_by_class: &std::collections::BTreeMap<usize, usize>,
    channel_role: &std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
) -> Result<(), std::collections::BTreeSet<u8>> {
    channel_consumer_overflow_with_labels(model, channel_by_class, channel_role, None)
}

fn channel_consumer_overflow_with_labels(
    model: &ChannelConsumerModel,
    channel_by_class: &std::collections::BTreeMap<usize, usize>,
    channel_role: &std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    voices: Option<&FrozenSerializedVoicePlan>,
) -> Result<(), std::collections::BTreeSet<u8>> {
    let owners = voices.map_or_else(
        || channel_reserved_label_owners(channel_role).into_iter()
            .map(|(label, channel)| (label.to_string(), channel)).collect(),
        |voices| voices.channel_to_label.iter().map(|(&channel, label)| (label.clone(), channel)).collect::<std::collections::BTreeMap<_, _>>(),
    );
    let mut presence = std::collections::BTreeMap::<ChannelConsumerVoice, [bool; 2]>::new();
    let mut pitched_presence = std::collections::BTreeMap::<usize, [bool; 2]>::new();
    let mut failing = std::collections::BTreeSet::<u8>::new();
    let mut resolved = Vec::with_capacity(model.notes.len());
    for note in &model.notes {
        let voice = match note.class {
            Some(class) => match channel_by_class.get(&class) {
                Some(&channel) => ChannelConsumerVoice::Channel(channel),
                None => {
                    failing.insert(note.staff);
                    resolved.push(None);
                    continue;
                }
            },
            None => owners.get(note.label.as_str()).map_or_else(
                || ChannelConsumerVoice::Free(note.label.clone()),
                |&channel| ChannelConsumerVoice::Channel(channel),
            ),
        };
        let staff_index = usize::from(note.staff - 1);
        if note.chordrest {
            presence.entry(voice.clone()).or_default()[staff_index] = true;
            if let ChannelConsumerVoice::Channel(channel) = voice
                && note.class.is_some()
            {
                pitched_presence.entry(channel).or_default()[staff_index] = true;
            }
        }
        resolved.push(Some(voice));
    }
    for staff in [1_u8, 2] {
        let index = usize::from(staff - 1);
        if presence.values().filter(|staves| staves[index]).count() > CONSUMER_VOICES_PER_STAFF {
            failing.insert(staff);
        }
    }
    for (note, voice) in model.notes.iter().zip(&resolved) {
        let Some(ChannelConsumerVoice::Channel(channel)) = voice else {
            continue;
        };
        if !note.required {
            continue;
        }
        let own = pitched_presence.get(channel).copied().unwrap_or_default();
        // Require an own-staff pitched head even if a free rest label later
        // aliases this channel or is split away by an existing notation owner.
        if !own[usize::from(note.staff - 1)] {
            failing.insert(note.staff);
        }
    }
    if failing.is_empty() { Ok(()) } else { Err(failing) }
}

struct ChannelSearchInput<'a> {
    model: &'a ChannelConsumerModel,
    class_order: &'a [usize],
    class_role: &'a std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    semantic_different: &'a std::collections::BTreeSet<(usize, usize)>,
    written_different: &'a std::collections::BTreeSet<(usize, usize)>,
}

impl ChannelSearchInput<'_> {
    fn conflicts(&self, left: usize, right: usize) -> bool {
        let pair = (left.min(right), left.max(right));
        self.semantic_different.contains(&pair) || self.written_different.contains(&pair)
    }
}

struct ChannelRoleColouring {
    role: SerializedVoicePresentationRole,
    classes: Vec<usize>,
    colour: Vec<usize>,
    clique: Vec<usize>,
}

// One deterministic DSATUR pass: each iteration colours one existing class.
// This is finite by graph cardinality; there is no introduced calibration or
// search-budget threshold. Failure of this candidate proves no impossibility.
fn channel_dsatur_candidate(adjacent: &[Vec<bool>]) -> Vec<usize> {
    let count = adjacent.len();
    let neighbours = adjacent.iter().map(|row| (0..count).filter(|&other| row[other]).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut colour = vec![None; count];
    let mut neighbour_colours = vec![std::collections::BTreeSet::new(); count];
    let mut uncoloured_degree = neighbours.iter().map(Vec::len).collect::<Vec<_>>();
    for _ in 0..count {
        let chosen = (0..count).filter(|&vertex| colour[vertex].is_none()).max_by_key(|&vertex| {
            (neighbour_colours[vertex].len(), uncoloured_degree[vertex], std::cmp::Reverse(vertex))
        }).expect("one uncoloured vertex remains in this finite pass");
        let next = (0..=count).find(|candidate| !neighbour_colours[chosen].contains(candidate))
            .expect("a graph of this cardinality always has an available colour");
        colour[chosen] = Some(next);
        for &other in &neighbours[chosen] {
            if colour[other].is_none() {
                neighbour_colours[other].insert(next);
                uncoloured_degree[other] -= 1;
            }
        }
    }
    colour.into_iter().map(|value| value.expect("all classes coloured")).collect()
}

fn channel_greedy_clique(adjacent: &[Vec<bool>], eligible: &[bool]) -> Vec<usize> {
    let count = adjacent.len();
    let mut best = Vec::new();
    for start in (0..count).filter(|&vertex| eligible[vertex]) {
        let mut clique = vec![start];
        for candidate in 0..count {
            if candidate != start
                && eligible[candidate]
                && clique.iter().all(|&member| adjacent[member][candidate])
            {
                clique.push(candidate);
            }
        }
        if clique.len() > best.len() {
            best = clique;
        }
    }
    best.sort_unstable();
    best
}

fn channel_colour_role(
    input: &ChannelSearchInput<'_>,
    role: SerializedVoicePresentationRole,
    greedy_channel_by_class: &std::collections::BTreeMap<usize, usize>,
) -> ChannelRoleColouring {
    let classes = input
        .class_order
        .iter()
        .copied()
        .filter(|class| input.class_role.get(class) == Some(&role))
        .collect::<Vec<_>>();
    let count = classes.len();
    let adjacent = (0..count)
        .map(|left| {
            (0..count)
                .map(|right| left != right && input.conflicts(classes[left], classes[right]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // Greedy colouring renamed by first appearance in the accepted order.
    let mut greedy_names = std::collections::BTreeMap::<usize, usize>::new();
    let greedy = classes
        .iter()
        .map(|class| {
            let channel = greedy_channel_by_class.get(class).copied().unwrap_or(usize::MAX);
            let next = greedy_names.len();
            *greedy_names.entry(channel).or_insert(next)
        })
        .collect::<Vec<_>>();
    let registering = input
        .model
        .notes
        .iter()
        .filter(|note| note.chordrest)
        .filter_map(|note| note.class)
        .collect::<std::collections::BTreeSet<_>>();
    let eligible = classes.iter().map(|class| registering.contains(class)).collect::<Vec<_>>();
    let clique = channel_greedy_clique(&adjacent, &eligible);
    let alternate = channel_dsatur_candidate(&adjacent);
    let alternate_colours = alternate.iter().copied().max().map_or(0, |value| value + 1);
    let colour = if alternate_colours <= greedy_names.len() { alternate } else { greedy };
    ChannelRoleColouring {
        role, clique: clique.iter().map(|&index| classes[index]).collect(),
        colour, classes,
    }

}

fn channel_role_is_pinned(
    input: &ChannelSearchInput<'_>,
    role: SerializedVoicePresentationRole,
    roles_present: &std::collections::BTreeSet<SerializedVoicePresentationRole>,
) -> bool {
    let staff = channel_role_staff(role);
    for note in &input.model.notes {
        if !note.chordrest || note.staff == staff {
            continue;
        }
        match note.class {
            Some(class) => {
                if input.class_role.get(&class) == Some(&role) {
                    return false;
                }
            }
            None => {
                // The owner of a reserved label is a role fact: the lowest sort
                // rank among present roles with that canonical label.
                let owner = roles_present
                    .iter()
                    .copied()
                    .filter(|candidate| candidate.canonical_label_preference() == note.label)
                    .min_by_key(|candidate| candidate.sort_rank());
                if owner.is_none_or(|owner| owner == role) {
                    return false;
                }
            }
        }
    }
    true
}

fn channel_search_after_consumer_overflow(
    input: &ChannelSearchInput<'_>,
    failing_staves: &std::collections::BTreeSet<u8>,
    greedy_channel_by_class: std::collections::BTreeMap<usize, usize>,
    greedy_channel_role: std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
) -> (
    std::collections::BTreeMap<usize, usize>,
    std::collections::BTreeMap<usize, SerializedVoicePresentationRole>,
    ChannelConsumerAllocation,
) {
    let roles_present = input
        .class_role
        .values()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let colourings = roles_present
        .iter()
        .copied()
        .filter(|role| failing_staves.contains(&channel_role_staff(*role)))
        .map(|role| channel_colour_role(input, role, &greedy_channel_by_class))
        .collect::<Vec<_>>();

    // Assemble: recoloured roles take their deterministic candidate colouring; every other
    // class keeps its greedy channel. Channel ids follow first appearance in the
    // accepted class order, which reproduces the greedy numbering when nothing
    // changes.
    let mut key_by_class = std::collections::BTreeMap::<usize, (u8, bool, usize)>::new();
    for colouring in &colourings {
        for (index, &class) in colouring.classes.iter().enumerate() {
            key_by_class.insert(class, (colouring.role.sort_rank(), true, colouring.colour[index]));
        }
    }
    let mut channel_by_key = std::collections::BTreeMap::<(u8, bool, usize), usize>::new();
    let mut channel_by_class = std::collections::BTreeMap::<usize, usize>::new();
    let mut channel_role = std::collections::BTreeMap::<usize, SerializedVoicePresentationRole>::new();
    for &class in input.class_order {
        let role = input.class_role[&class];
        let key = key_by_class
            .get(&class)
            .copied()
            .unwrap_or((role.sort_rank(), false, greedy_channel_by_class[&class]));
        let next = channel_by_key.len() + 1;
        let channel = *channel_by_key.entry(key).or_insert(next);
        channel_role.insert(channel, role);
        channel_by_class.insert(class, channel);
    }
    if channel_consumer_overflow(input.model, &channel_by_class, &channel_role).is_ok() {
        return (
            channel_by_class,
            channel_role.clone(),
            ChannelConsumerAllocation::AdmittedAfterSearch {
                channels_before: greedy_channel_role.len(),
                channels_after: channel_role.len(),
            },
        );
    }

    // No admitted candidate: keep the first candidate and state why.
    let mut verdict = ChannelConsumerAllocation::Undetermined(
        "no admitted partition found and no slot-obligation proof applies",
    );
    for &staff in failing_staves {
        let on_staff = colourings
            .iter()
            .filter(|colouring| channel_role_staff(colouring.role) == staff)
            .collect::<Vec<_>>();
        let pinned = on_staff
            .iter()
            .filter(|colouring| channel_role_is_pinned(input, colouring.role, &roles_present))
            .collect::<Vec<_>>();
        let clique_demand = pinned.iter().map(|colouring| colouring.clique.len()).sum::<usize>();
        if clique_demand > CONSUMER_VOICES_PER_STAFF {
            verdict = ChannelConsumerAllocation::SlotObligationWitness {
                staff,
                demand: clique_demand,
                cliques: pinned
                    .iter()
                    .map(|colouring| (colouring.role, colouring.clique.clone()))
                    .collect(),
            };
            break;
        }
    }
    (greedy_channel_by_class, greedy_channel_role, verdict)
}

#[cfg(test)]
#[path = "../../tests/unit/channel_consumer_allocation.rs"]
mod channel_consumer_allocation_tests;

pub(crate) fn bind_musicxml_voice_labels(
    channels: &FrozenSerializationVoiceChannelPlan,
    legacy_preferences: &[LegacyVoiceLabelPreferenceRow],
) -> io::Result<FrozenSerializedVoicePlan> {
    if !channels.authority_bound || channels.channel_count == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MusicXmlVoiceLabel binding requires bound serialization channels",
        ));
    }
    if channels.source_to_channel.keys().ne(channels.source_to_role.keys()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MusicXmlVoiceLabel binding requires exact canonical source-role coverage",
        ));
    }
    if let Some(before) = &channels.label_continuation {
        let invalid = || io::Error::new(io::ErrorKind::InvalidData,
            "MusicXmlVoiceLabel continuation has missing, foreign or changed channel binding");
        if !before.authority_bound
            || before.source_to_role != channels.source_to_role
            || before.source_to_channel.keys().ne(channels.source_to_channel.keys())
            || before.source_to_label.keys().ne(channels.source_to_channel.keys())
            || before.channel_to_label.keys().ne(before.channel_to_role.keys())
            || channels.channel_count != channels.channel_to_role.len()
            || before.channel_to_label.values().collect::<std::collections::BTreeSet<_>>().len()
                != before.channel_to_label.len()
            || before.label_to_role.len() != before.channel_to_label.len()
        { return Err(invalid()); }
        for (&channel, label) in &before.channel_to_label {
            if label.is_empty() || label.trim() != label
                || before.label_to_role.get(label) != before.channel_to_role.get(&channel)
            { return Err(invalid()); }
        }
        for (&source, &channel) in &before.source_to_channel {
            if before.source_to_label.get(&source) != before.channel_to_label.get(&channel)
                || !before.channel_to_role.contains_key(&channel)
                || (channels.channel_to_role.contains_key(&channel)
                    && channels.source_to_channel.get(&source) != Some(&channel))
            { return Err(invalid()); }
        }
        let mut channel_to_label = std::collections::BTreeMap::new();
        let mut label_to_role = std::collections::BTreeMap::new();
        for (&channel, role) in &channels.channel_to_role {
            if before.channel_to_role.get(&channel) != Some(role)
                || !channels.source_to_channel.values().any(|value| *value == channel)
            { return Err(invalid()); }
            let label = before.channel_to_label.get(&channel).ok_or_else(invalid)?.clone();
            label_to_role.insert(label.clone(), *role);
            channel_to_label.insert(channel, label);
        }
        let source_to_label = channels.source_to_channel.iter().map(|(&source, channel)| {
            channel_to_label.get(channel).cloned().map(|label| (source, label)).ok_or_else(invalid)
        }).collect::<io::Result<std::collections::BTreeMap<_, _>>>()?;
        return Ok(FrozenSerializedVoicePlan {
            canonical_role_allocation: Some(std::sync::Arc::new(CanonicalFrozenRoleAllocation {
                source_roles: channels.source_to_role.clone(), channel_roles: channels.channel_to_role.clone(),
            })),
            authority_bound: true,
            source_to_role: channels.source_to_role.clone(),
            source_to_channel: channels.source_to_channel.clone(),
            source_to_label, label_to_role, channel_to_label,
            channel_to_role: channels.channel_to_role.clone(),
            semantic_fingerprint_fnv64: channels.semantic_fingerprint_fnv64,
        });
    }
    let mut legacy_by_source = std::collections::BTreeMap::<usize, &str>::new();
    for row in legacy_preferences {
        if channels.source_to_channel.contains_key(&row.source_event_id) {
            legacy_by_source.insert(row.source_event_id, row.label.trim());
        }
    }

    let mut channel_members = std::collections::BTreeMap::<usize, Vec<usize>>::new();
    for (&source, &channel) in &channels.source_to_channel {
        channel_members.entry(channel).or_default().push(source);
    }
    let mut channel_order = channels.channel_to_role.keys().copied().collect::<Vec<_>>();
    channel_order.sort_by_key(|channel| (channels.channel_to_role[channel].sort_rank(), *channel));

    let mut reserved_owner = std::collections::BTreeMap::<String, usize>::new();
    for &channel in &channel_order {
        let label = channels.channel_to_role[&channel].canonical_label_preference().to_string();
        reserved_owner.entry(label).or_insert(channel);
    }
    let mut used_labels = std::collections::BTreeSet::<String>::new();
    let mut channel_to_label = std::collections::BTreeMap::<usize, String>::new();
    for channel in channel_order {
        let role = channels.channel_to_role[&channel];
        let canonical = role.canonical_label_preference().to_string();
        let mut legacy_counts = std::collections::BTreeMap::<String, usize>::new();
        for source in channel_members.get(&channel).into_iter().flatten() {
            if let Some(label) = legacy_by_source.get(source)
                && !label.is_empty()
            {
                *legacy_counts.entry((*label).to_string()).or_default() += 1;
            }
        }
        let mut legacy_ranked = legacy_counts.into_iter().collect::<Vec<_>>();
        legacy_ranked.sort_by(|(left_label, left_count), (right_label, right_count)| {
            right_count.cmp(left_count).then_with(|| left_label.cmp(right_label))
        });

        // Legacy labels may influence only this final cosmetic/stability tie-break.
        // Constraint classes, roles and channels are already frozen above.
        let mut candidates = vec![canonical];
        candidates.extend(legacy_ranked.into_iter().map(|(label, _)| label));
        let available = |label: &String| {
            !label.is_empty() && !used_labels.contains(label)
                && reserved_owner.get(label).is_none_or(|owner| *owner == channel)
        };
        let label = if let Some(label) = candidates.into_iter().find(available)
        {
            label
        } else {
            let mut ordinal = 1usize;
            loop {
                let label = ordinal.to_string();
                if available(&label) {
                    break label;
                }
                ordinal = ordinal.saturating_add(1);
            }
        };
        used_labels.insert(label.clone());
        channel_to_label.insert(channel, label);
    }

    let mut source_to_label = std::collections::BTreeMap::new();
    let mut label_to_role = std::collections::BTreeMap::new();
    for (&source, &channel) in &channels.source_to_channel {
        let label = channel_to_label.get(&channel).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "serialization channel has no label")
        })?;
        source_to_label.insert(source, label.clone());
        label_to_role.insert(label.clone(), channels.channel_to_role[&channel]);
    }
    Ok(FrozenSerializedVoicePlan {
        canonical_role_allocation: Some(std::sync::Arc::new(CanonicalFrozenRoleAllocation {
            source_roles: channels.source_to_role.clone(), channel_roles: channels.channel_to_role.clone(),
        })),
        source_to_role: channels.source_to_role.clone(),
        authority_bound: true,
        source_to_channel: channels.source_to_channel.clone(),
        source_to_label,
        label_to_role,
        channel_to_label,
        channel_to_role: channels.channel_to_role.clone(),
        semantic_fingerprint_fnv64: channels.semantic_fingerprint_fnv64,
    })
}

fn final_chord_permute_sealed_voice_labels(
    voice_plan: &FrozenSerializedVoicePlan,
    permutation: &[String],
) -> io::Result<FrozenSerializedVoicePlan> {
    let labels = voice_plan
        .channel_to_label
        .values()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if labels.len() != permutation.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MusicXmlVoiceLabel alpha permutation cardinality drift",
        ));
    }
    let mapping = labels
        .iter()
        .cloned()
        .zip(permutation.iter().cloned())
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut source_to_label = std::collections::BTreeMap::new();
    for (&source, label) in &voice_plan.source_to_label {
        source_to_label.insert(
            source,
            mapping.get(label).cloned().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "sealed source label absent from alpha mapping")
            })?,
        );
    }
    let mut channel_to_label = std::collections::BTreeMap::new();
    let mut label_to_role = std::collections::BTreeMap::new();
    for (&channel, label) in &voice_plan.channel_to_label {
        let permuted = mapping.get(label).cloned().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "sealed channel label absent from alpha mapping")
        })?;
        channel_to_label.insert(channel, permuted.clone());
        label_to_role.insert(permuted, voice_plan.channel_to_role[&channel]);
    }
    Ok(FrozenSerializedVoicePlan {
        canonical_role_allocation: voice_plan.canonical_role_allocation.clone(),
        source_to_role: voice_plan.source_to_role.clone(),
        authority_bound: true,
        source_to_channel: voice_plan.source_to_channel.clone(),
        source_to_label,
        label_to_role,
        channel_to_label,
        channel_to_role: voice_plan.channel_to_role.clone(),
        semantic_fingerprint_fnv64: voice_plan.semantic_fingerprint_fnv64,
    })
}

fn final_chord_map_all_voice_elements(
    candidate_xml: &str,
    mapping: &std::collections::BTreeMap<String, String>,
) -> io::Result<String> {
    let document = roxmltree::Document::parse(candidate_xml).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("MusicXmlVoiceLabel global alpha parse failed: {error}"),
        )
    })?;
    let mut edits = Vec::<(usize, usize, String)>::new();
    for node in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("voice"))
    {
        let label = node.text().unwrap_or("").trim();
        let replacement = mapping.get(label).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "VoiceFreeEvidenceSufficiency: final MusicXML voice label {label:?} has no sealed channel binding"
                ),
            )
        })?;
        let range = node.range();
        edits.push((
            range.start,
            range.end,
            format!("<voice>{replacement}</voice>"),
        ));
    }
    final_chord_project_ranges(
        candidate_xml,
        edits,
        "MusicXmlVoiceLabel global alpha byte range drift",
    )
}

fn audit_musicxml_voice_label_binding_alpha_with_optional_render_omissions(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> io::Result<VoiceLabelBindingMetamorphicAudit> {
    if !canonical.voice_free_evidence_sufficiency_green() || !voice_plan.is_bound() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MusicXmlVoiceLabel alpha audit requires sealed canonical structure and label binding",
        ));
    }
    let labels = voice_plan
        .channel_to_label
        .values()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if labels.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MusicXmlVoiceLabel alpha audit found no sealed labels",
        ));
    }
    let permutations = if labels.len() <= 4 {
        final_chord_all_permutations(&labels)
    } else {
        let identity = labels.clone();
        let mut reverse = labels.clone();
        reverse.reverse();
        let mut swap = labels.clone();
        let last = swap.len().saturating_sub(1);
        swap.swap(0, last);
        let mut cycle = labels.clone();
        cycle.rotate_left(1);
        vec![identity, reverse, swap, cycle]
    };

    let original_bound =
        bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
            candidate_xml,
            canonical,
            voice_plan,
            render_omissions,
        )?;
    let original_projected =
        project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha(
            candidate_xml,
            canonical,
            &original_bound,
        )?;
    let identity_mapping = labels
        .iter()
        .cloned()
        .map(|label| (label.clone(), label))
        .collect::<std::collections::BTreeMap<_, _>>();
    // Preserve the preexisting closed-world check for every serialized <voice>
    // element without using the rewritten bytes as the baseline proof target.
    let _ = final_chord_map_all_voice_elements(&original_projected.xml, &identity_mapping)?;

    // BASELINE is an independent precondition of the alpha experiment. r819 showed
    // that identity itself could fail; stale-negative counters are meaningful only
    // after original projection + original VoicePlan is proven green.
    verify_canonical_final_chord_transaction_local_output_alpha(
        &original_projected.xml,
        canonical,
        voice_plan,
        render_omissions,
        &original_projected.receipt,
        original_bound.plan.source_fingerprint,
        original_bound.plan.source_len,
    )
    .map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChord transaction-local Output-Alpha BASELINE readback failed before label permutation: {error}"
            ),
        )
    })?;

    let mut audit = VoiceLabelBindingMetamorphicAudit::default();
    for permutation in permutations.into_iter().filter(|permutation| permutation != &labels) {
        audit.cases = audit.cases.saturating_add(1);
        let permuted = final_chord_permute_sealed_voice_labels(voice_plan, &permutation)?;
        for (&source, &channel) in &voice_plan.source_to_channel {
            let original_role = voice_plan.channel_to_role.get(&channel).copied();
            let permuted_label = permuted.label_for_source(source);
            let permuted_role = permuted_label.and_then(|label| permuted.role_for_label(label));
            if original_role != permuted_role {
                audit.semantic_role_deltas = audit.semantic_role_deltas.saturating_add(1);
            }
            if voice_plan.role_for_source(source) != permuted.role_for_source(source) {
                audit.semantic_role_deltas = audit.semantic_role_deltas.saturating_add(1);
            }
        }
        let mapping = labels
            .iter()
            .cloned()
            .zip(permutation.iter().cloned())
            .collect::<std::collections::BTreeMap<_, _>>();
        let projected = final_chord_transaction_output_alpha_map(&original_projected, &mapping)?;
        if verify_canonical_final_chord_transaction_local_output_alpha(
            &projected.xml,
            canonical,
            &permuted,
            render_omissions,
            &projected.receipt,
            original_bound.plan.source_fingerprint,
            original_bound.plan.source_len,
        )
        .is_err()
        {
            audit.paired_binding_failures = audit.paired_binding_failures.saturating_add(1);
        }
        let changed = permutation != labels;
        if changed
            && verify_canonical_final_chord_transaction_local_output_alpha(
                &projected.xml,
                canonical,
                voice_plan,
                render_omissions,
                &projected.receipt,
                original_bound.plan.source_fingerprint,
                original_bound.plan.source_len,
            )
            .is_ok()
        {
            audit.stale_binding_false_accepts =
                audit.stale_binding_false_accepts.saturating_add(1);
        }
    }
    audit.green = audit.semantic_role_deltas == 0
        && audit.paired_binding_failures == 0
        && audit.stale_binding_false_accepts == 0;
    Ok(audit)
}


pub(crate) fn audit_musicxml_voice_label_binding_alpha(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
) -> io::Result<VoiceLabelBindingMetamorphicAudit> {
    audit_musicxml_voice_label_binding_alpha_with_optional_render_omissions(
        candidate_xml,
        canonical,
        voice_plan,
        None,
    )
}

pub(crate) fn audit_musicxml_voice_label_binding_alpha_for_render_projection(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: &std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
) -> io::Result<VoiceLabelBindingMetamorphicAudit> {
    audit_musicxml_voice_label_binding_alpha_with_optional_render_omissions(
        candidate_xml,
        canonical,
        voice_plan,
        Some(render_omissions),
    )
}

fn final_chord_declassified_evidence_from_xml(
    candidate_xml: &str,
) -> io::Result<Vec<VoiceFreeFinalChordEvidenceRow>> {
    let measures = final_chord_parse_binding_measures(candidate_xml)?;
    let mut rows = Vec::new();
    let mut next_identity = 1usize;
    let mut next_fragment_id = 1usize;
    for measure in &measures {
        let mut source_by_note = std::collections::BTreeMap::<usize, usize>::new();
        for (index, note) in measure.notes.iter().enumerate() {
            let Some(pitch) = note.pitch_midi else {
                continue;
            };
            let source = if note.tie_stop {
                let predecessors = (0..index)
                    .filter(|other_index| {
                        let other = &measure.notes[*other_index];
                        !other.rest_like
                            && other.pitch_midi == Some(pitch)
                            && other.staff == note.staff
                            && other.tie_start
                            && other.onset.saturating_add(other.duration) == note.onset
                            && source_by_note.contains_key(other_index)
                    })
                    .collect::<Vec<_>>();
                if predecessors.len() == 1 {
                    source_by_note[&predecessors[0]]
                } else {
                    let id = next_identity;
                    next_identity = next_identity.saturating_add(1);
                    id
                }
            } else {
                let id = next_identity;
                next_identity = next_identity.saturating_add(1);
                id
            };
            source_by_note.insert(index, source);
            let staff = note.staff.parse::<u8>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord declassification found non-numeric staff",
                )
            })?;
            rows.push(VoiceFreeFinalChordEvidenceRow {
                fragment_id: next_fragment_id,
                source_event_id: source,
                attack_id: source,
                measure: note.measure,
                raw_note_index: note.raw_note_index,
                onset: note.onset,
                duration: note.duration,
                pitch,
                staff,
                chord_syntax: note.chord_member,
                tie_start: note.tie_start,
                tie_stop: note.tie_stop,
            });
            next_fragment_id = next_fragment_id.saturating_add(1);
        }
    }
    if rows.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord declassification produced no pitched evidence",
        ));
    }
    Ok(rows)
}

pub(crate) fn plan_final_chord_encoding(
    candidate_xml: &str,
) -> io::Result<FinalChordEncodingPlan> {
    let evidence = final_chord_declassified_evidence_from_xml(candidate_xml)?;
    let (canonical, plan) =
        plan_canonical_final_chord_structure_once(candidate_xml, &evidence)?;
    let audit = audit_voice_cutover_noninterference(candidate_xml, &evidence, &canonical)?;
    if !audit.green() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FrozenSerializedVoiceChordConstraintCutover LegacyVoice-NonInterference blocked: label_cases={} label_delta={} partition_cases={} partition_delta={} opaque_id_cases={} opaque_id_delta={}",
                audit.label_alpha_cases(),
                audit.label_alpha_semantic_deltas(),
                audit.legacy_partition_cases(),
                audit.legacy_partition_semantic_deltas(),
                audit.opaque_identity_rename_cases(),
                audit.opaque_identity_semantic_deltas(),
            ),
        ));
    }
    Ok(plan)
}

#[derive(Debug, Clone)]
struct FinalChordBindingMeasure {
    measure: usize,
    divisions: i64,
    expected_duration: i64,
    measure_close: usize,
    notes: Vec<FinalChordEncodingNote>,
}

fn final_chord_parse_binding_measures(candidate_xml: &str) -> io::Result<Vec<FinalChordBindingMeasure>> {
    let document = validate_final_chord_musicxml(candidate_xml)?;
    let mut out = Vec::new();
    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
    {
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        for measure in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
        {
            if let Some(attributes) = measure
                .children()
                .find(|node| node.is_element() && node.has_tag_name("attributes"))
            {
                if let Some(value) = final_chord_direct_child_i64(attributes, "divisions") {
                    divisions = value.max(1);
                }
                if let Some(time) = attributes
                    .children()
                    .find(|node| node.is_element() && node.has_tag_name("time"))
                {
                    if let Some(value) = final_chord_direct_child_i64(time, "beats") {
                        beats = value.max(1);
                    }
                    if let Some(value) = final_chord_direct_child_i64(time, "beat-type") {
                        beat_type = value.max(1);
                    }
                }
            }
            let expected_duration = divisions
                .saturating_mul(beats)
                .saturating_mul(4)
                / beat_type.max(1);
            let measure_ordinal = measure
                .attribute("number")
                .and_then(|value| value.parse::<usize>().ok())
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord physical binding requires numeric measure identity",
                    )
                })?;
            let measure_range = measure.range();
            let measure_close = candidate_xml
                .get(measure_range.clone())
                .and_then(|text| text.rfind("</measure>"))
                .map(|relative| measure_range.start + relative)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord physical binding lost measure close",
                    )
                })?;

            let mut cursor = 0_i64;
            let mut last_anchor_onset = None;
            let mut last_anchor_index = None;
            let mut orphan_after_forward = false;
            let mut raw_note_index = 0usize;
            let mut notes = Vec::new();
            for child in measure.children().filter(|node| node.is_element()) {
                if child.has_tag_name("backup") {
                    cursor = cursor.saturating_sub(
                        final_chord_direct_child_i64(child, "duration").unwrap_or(0),
                    );
                    last_anchor_onset = None;
                    last_anchor_index = None;
                    orphan_after_forward = false;
                    continue;
                }
                if child.has_tag_name("forward") {
                    cursor = cursor.saturating_add(
                        final_chord_direct_child_i64(child, "duration").unwrap_or(0),
                    );
                    last_anchor_onset = None;
                    last_anchor_index = None;
                    orphan_after_forward = true;
                    continue;
                }
                if !child.has_tag_name("note") {
                    continue;
                }
                let duration = final_chord_direct_child_i64(child, "duration").unwrap_or(0).max(0);
                let chord_member = final_chord_direct_has_child(child, "chord");
                let onset = if chord_member {
                    last_anchor_onset.unwrap_or(cursor)
                } else {
                    cursor
                };
                let anchor_index = if chord_member { last_anchor_index } else { None };
                let rest_like = final_chord_direct_has_child(child, "rest");
                let tie_start = child.children().any(|item| {
                    item.is_element()
                        && item.has_tag_name("tie")
                        && item.attribute("type") == Some("start")
                });
                let tie_stop = child.children().any(|item| {
                    item.is_element()
                        && item.has_tag_name("tie")
                        && item.attribute("type") == Some("stop")
                });
                let note_index = notes.len();
                notes.push(FinalChordEncodingNote {
                    range: child.range(),
                    measure: measure_ordinal,
                    raw_note_index,
                    onset,
                    duration,
                    chord_member,
                    rest_like,
                    pitch_identity: final_chord_pitch_identity(child),
                    pitch_midi: final_chord_pitch_midi(child),
                    voice: final_chord_direct_child_text(child, "voice")
                        .unwrap_or_else(|| "1".to_string()),
                    staff: final_chord_direct_child_text(child, "staff")
                        .unwrap_or_else(|| "1".to_string()),
                    tie_start,
                    tie_stop,
                    anchor_index,
                    orphan_after_forward: chord_member
                        && anchor_index.is_none()
                        && orphan_after_forward,
                    source_event_id: None,
                    attack_id: None,
                    fragment_id: None,
                });
                raw_note_index = raw_note_index.saturating_add(1);
                if !chord_member {
                    last_anchor_onset = Some(onset);
                    last_anchor_index = Some(note_index);
                    orphan_after_forward = false;
                    cursor = cursor.saturating_add(duration);
                }
            }
            out.push(FinalChordBindingMeasure {
                measure: measure_ordinal,
                divisions,
                expected_duration,
                measure_close,
                notes,
            });
        }
    }
    Ok(out)
}

fn final_chord_voice_free_pitched_sequence_fingerprint(
    measures: &[FinalChordBindingMeasure],
) -> u64 {
    let mut signature = String::from("ETUSCHLOPF_FINAL_CHORD_RENDER_OCCURRENCE_SEQUENCE_V1\n");
    for measure in measures {
        let mut pitched_ordinal = 0usize;
        for note in &measure.notes {
            if note.rest_like || note.pitch_midi.is_none() {
                continue;
            }
            let _ = writeln!(
                signature,
                "P|m{}|ord{}|o{}|p{}|s{}|c{}",
                measure.measure,
                pitched_ordinal,
                note.onset,
                note.pitch_midi.unwrap_or_default(),
                note.staff,
                u8::from(note.chord_member),
            );
            pitched_ordinal = pitched_ordinal.saturating_add(1);
        }
    }
    fingerprint(&signature)
}

pub(crate) fn locate_final_chord_render_occurrence_by_exact_range(
    candidate_xml: &str,
    note_start: usize,
    note_end: usize,
) -> io::Result<(usize, usize, u64)> {
    let measures = final_chord_parse_binding_measures(candidate_xml)?;
    let fingerprint = final_chord_voice_free_pitched_sequence_fingerprint(&measures);
    for measure in &measures {
        let mut pitched_ordinal = 0usize;
        for note in &measure.notes {
            if note.rest_like || note.pitch_midi.is_none() {
                continue;
            }
            if note.range.start == note_start && note.range.end == note_end {
                return Ok((measure.measure, pitched_ordinal, fingerprint));
            }
            pitched_ordinal = pitched_ordinal.saturating_add(1);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "FinalChord render-occurrence continuation cannot resolve exact generated note range",
    ))
}

fn final_chord_pitched_note_by_measure_ordinal(
    measures: &[FinalChordBindingMeasure],
    measure_number: usize,
    pitched_ordinal: usize,
) -> io::Result<&FinalChordEncodingNote> {
    let measure = measures
        .iter()
        .find(|measure| measure.measure == measure_number)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord occurrence continuation lost witnessed measure",
            )
        })?;
    measure
        .notes
        .iter()
        .filter(|note| !note.rest_like && note.pitch_midi.is_some())
        .nth(pitched_ordinal)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord occurrence continuation lost witnessed pitched ordinal",
            )
        })
}

fn final_chord_render_representation_duration(
    canonical: &CanonicalFinalChordStructuralPlan,
    fragment_id: usize,
    allow_render_occurrence_witness: bool,
) -> Option<i64> {
    if !allow_render_occurrence_witness {
        return None;
    }
    canonical
        .render_occurrence_witnesses
        .iter()
        .find(|witness| witness.fragment_id == fragment_id)
        .and_then(|witness| witness.representation_duration)
}

pub(crate) fn advance_final_chord_render_occurrence_through_staccato_persistence(
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: &std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
    receipt: &super::render_measure_articulation_authority::StaccatoPersistenceOccurrenceReceipt,
    persisted_xml: &str,
) -> io::Result<CanonicalFinalChordStructuralPlan> {
    if receipt.candidate_xml() != persisted_xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord staccato occurrence continuation is not byte-exact to the committed writer candidate",
        ));
    }
    let source_xml = receipt.source_xml();
    let bound = bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
        source_xml,
        canonical,
        voice_plan,
        Some(render_omissions),
    )?;
    let source_measures = final_chord_parse_binding_measures(source_xml)?;
    let persisted_measures = final_chord_parse_binding_measures(persisted_xml)?;
    let source_sequence_fingerprint =
        final_chord_voice_free_pitched_sequence_fingerprint(&source_measures);
    let persisted_sequence_fingerprint =
        final_chord_voice_free_pitched_sequence_fingerprint(&persisted_measures);
    if source_sequence_fingerprint != persisted_sequence_fingerprint {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord staccato occurrence continuation changed pitched occurrence topology",
        ));
    }

    let locator_by_fragment = canonical
        .source_locators
        .iter()
        .map(|locator| (locator.fragment_id, locator))
        .collect::<std::collections::BTreeMap<_, _>>();
    let action_by_fragment = canonical
        .actions
        .iter()
        .map(|action| (action.fragment_id, action))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut witnesses = canonical.render_occurrence_witnesses.to_vec();
    let mut continued_fragments = std::collections::BTreeSet::<usize>::new();

    for &(note_start, note_end) in receipt.pitched_source_ranges() {
        let binding = bound
            .input_bindings
            .iter()
            .find(|row| row.source_note_start == note_start && row.source_note_end == note_end)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord staccato occurrence continuation could not bind a writer-owned pitched source range",
                )
            })?;
        if !continued_fragments.insert(binding.fragment_id) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord staccato occurrence continuation duplicated a canonical fragment",
            ));
        }
        let locator = locator_by_fragment
            .get(&binding.fragment_id)
            .copied()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord staccato occurrence continuation lost canonical source provenance",
                )
            })?;
        match action_by_fragment.get(&binding.fragment_id).map(|row| &row.kind) {
            None | Some(FinalChordCanonicalActionKind::RetainedChordMember { .. }) => {}
            Some(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord staccato occurrence continuation cannot retime a generated/tied representation role",
                ));
            }
        }
        let (measure, pitched_ordinal, observed_source_fingerprint) =
            locate_final_chord_render_occurrence_by_exact_range(
                source_xml,
                note_start,
                note_end,
            )?;
        if observed_source_fingerprint != source_sequence_fingerprint || measure != locator.measure {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord staccato occurrence continuation source occurrence disagrees with canonical provenance",
            ));
        }
        let before_note = final_chord_pitched_note_by_measure_ordinal(
            &source_measures,
            measure,
            pitched_ordinal,
        )?;
        let after_note = final_chord_pitched_note_by_measure_ordinal(
            &persisted_measures,
            measure,
            pitched_ordinal,
        )?;
        if before_note.onset != after_note.onset
            || before_note.pitch_midi != after_note.pitch_midi
            || before_note.staff != after_note.staff
            || before_note.voice != after_note.voice
            || before_note.chord_member != after_note.chord_member
            || before_note.tie_start != after_note.tie_start
            || before_note.tie_stop != after_note.tie_stop
            || after_note.duration <= before_note.duration
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord staccato occurrence continuation found unauthorized pitched-state drift",
            ));
        }
        if binding.source_event_id != locator.source_event_id
            || binding.attack_id != locator.attack_id
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord staccato occurrence continuation source/attack owner drift",
            ));
        }
        let witness = FinalChordRenderOccurrenceWitness {
            fragment_id: binding.fragment_id,
            source_event_id: binding.source_event_id,
            attack_id: binding.attack_id,
            measure,
            pitched_ordinal,
            representation_duration: Some(after_note.duration),
            sequence_fingerprint_fnv64: persisted_sequence_fingerprint,
        };
        if let Some(existing) = witnesses
            .iter_mut()
            .find(|row| row.fragment_id == binding.fragment_id)
        {
            *existing = witness;
        } else {
            witnesses.push(witness);
        }
    }
    if continued_fragments.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord staccato occurrence continuation received no pitched writer receipt",
        ));
    }

    let mut advanced = canonical.clone();
    attach_final_chord_render_occurrence_witnesses(&mut advanced, witnesses)?;
    bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
        persisted_xml,
        &advanced,
        voice_plan,
        Some(render_omissions),
    )?;
    Ok(advanced)
}

fn final_chord_binding_note_matches_locator(
    note: &FinalChordEncodingNote,
    locator: &FinalChordCanonicalSourceLocator,
    ignore_duration: bool,
    ignore_representation_onset: bool,
) -> bool {
    !note.rest_like
        && note.measure == locator.measure
        && (ignore_representation_onset || note.onset == locator.onset)
        && (ignore_duration || note.duration == locator.duration)
        && note.pitch_midi == Some(locator.pitch_midi)
        && note.staff == locator.staff
        && note.chord_member == locator.chord_syntax
        && note.tie_start == locator.tie_start
        && note.tie_stop == locator.tie_stop
}

fn final_chord_find_binding_note<'a>(
    measures: &'a [FinalChordBindingMeasure],
    canonical: &CanonicalFinalChordStructuralPlan,
    locator: &FinalChordCanonicalSourceLocator,
    ignore_duration: bool,
    allow_render_occurrence_witness: bool,
) -> io::Result<&'a FinalChordEncodingNote> {
    let measure = measures
        .iter()
        .find(|measure| measure.measure == locator.measure)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("FinalChord physical binding lost measure {}", locator.measure),
            )
        })?;
    if allow_render_occurrence_witness
        && let Some(witness) = canonical
            .render_occurrence_witnesses
            .iter()
            .find(|witness| witness.fragment_id == locator.fragment_id)
    {
        let current_fingerprint = final_chord_voice_free_pitched_sequence_fingerprint(measures);
        if current_fingerprint != witness.sequence_fingerprint_fnv64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FinalChord render-occurrence continuation drift for fragment {}: expected_fp={:016x} actual_fp={:016x}",
                    locator.fragment_id,
                    witness.sequence_fingerprint_fnv64,
                    current_fingerprint
                ),
            ));
        }
        if witness.measure != locator.measure {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord render-occurrence continuation measure disagrees with canonical locator",
            ));
        }
        let note = measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .nth(witness.pitched_ordinal)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord render-occurrence continuation lost generated pitched ordinal",
                )
            })?;
        let receipt_bound_duration = witness.representation_duration;
        // The writer-sequence fingerprint binds the current physical render onset.
        // The canonical locator keeps semantic onset authority and must not duplicate it here.
        if !final_chord_binding_note_matches_locator(
            note,
            locator,
            ignore_duration || receipt_bound_duration.is_some(),
            true,
        ) || receipt_bound_duration.is_some_and(|duration| note.duration != duration)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FinalChord render-occurrence continuation disagrees with canonical locator fragment={} source={} attack={} m{} ord{}",
                    locator.fragment_id,
                    locator.source_event_id,
                    locator.attack_id,
                    locator.measure,
                    witness.pitched_ordinal
                ),
            ));
        }
        return Ok(note);
    }
    if let Some(note) = measure.notes.get(locator.raw_note_index)
        && final_chord_binding_note_matches_locator(note, locator, ignore_duration, false)
    {
        return Ok(note);
    }
    let matches = measure
        .notes
        .iter()
        .filter(|note| {
            final_chord_binding_note_matches_locator(note, locator, ignore_duration, false)
        })
        .collect::<Vec<_>>();
    let first = matches.first().copied().ok_or_else(|| {
        let raw_candidate = measure.notes.get(locator.raw_note_index).map_or_else(
            || "none".to_string(),
            |note| {
                format!(
                    "raw={} range={}..{} o={} dur={} p={:?} s={} v={} rest={} chord={} tie_start={} tie_stop={}",
                    note.raw_note_index,
                    note.range.start,
                    note.range.end,
                    note.onset,
                    note.duration,
                    note.pitch_midi,
                    note.staff,
                    note.voice,
                    note.rest_like,
                    note.chord_member,
                    note.tie_start,
                    note.tie_stop
                )
            },
        );
        let near_candidates = measure
            .notes
            .iter()
            .filter(|note| {
                !note.rest_like
                    && note.onset == locator.onset
                    && note.pitch_midi == Some(locator.pitch_midi)
            })
            .map(|note| {
                format!(
                    "raw={} range={}..{} dur={} s={} v={} chord={} tie_start={} tie_stop={}",
                    note.raw_note_index,
                    note.range.start,
                    note.range.end,
                    note.duration,
                    note.staff,
                    note.voice,
                    note.chord_member,
                    note.tie_start,
                    note.tie_stop
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChord physical binding cannot resolve canonical locator m{} o{} p{} s{}; locator fragment={} source={} attack={} raw={} dur={} chord={} tie_start={} tie_stop={} ignore_duration={}; raw_candidate=[{}]; near_candidates=[{}]",
                locator.measure,
                locator.onset,
                locator.pitch_midi,
                locator.staff,
                locator.fragment_id,
                locator.source_event_id,
                locator.attack_id,
                locator.raw_note_index,
                locator.duration,
                locator.chord_syntax,
                locator.tie_start,
                locator.tie_stop,
                ignore_duration,
                raw_candidate,
                near_candidates
            ),
        )
    })?;
    if matches.len() > 1 {
        let canonical_peers = canonical
            .source_locators
            .iter()
            .filter(|peer| {
                peer.measure == locator.measure
                    && peer.onset == locator.onset
                    && (ignore_duration || peer.duration == locator.duration)
                    && peer.pitch_midi == locator.pitch_midi
                    && peer.staff == locator.staff
                    && peer.chord_syntax == locator.chord_syntax
                    && peer.tie_start == locator.tie_start
                    && peer.tie_stop == locator.tie_stop
            })
            .map(|peer| {
                format!(
                    "fragment={} source={} attack={} raw={} dur={}",
                    peer.fragment_id,
                    peer.source_event_id,
                    peer.attack_id,
                    peer.raw_note_index,
                    peer.duration
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        let candidate_matches = matches
            .iter()
            .map(|note| {
                let host = note
                    .anchor_index
                    .and_then(|anchor_index| measure.notes.get(anchor_index));
                let host_summary = host.map_or_else(
                    || "host=none".to_string(),
                    |host| {
                        format!(
                            "host_raw={} host_o={} host_dur={} host_p={:?} host_s={} host_v={} host_rest={} host_chord={} host_tie_start={} host_tie_stop={}",
                            host.raw_note_index,
                            host.onset,
                            host.duration,
                            host.pitch_midi,
                            host.staff,
                            host.voice,
                            host.rest_like,
                            host.chord_member,
                            host.tie_start,
                            host.tie_stop
                        )
                    },
                );
                format!(
                    "raw={} range={}..{} dur={} voice={} chord={} tie_start={} tie_stop={} orphan_after_forward={} {}",
                    note.raw_note_index,
                    note.range.start,
                    note.range.end,
                    note.duration,
                    note.voice,
                    note.chord_member,
                    note.tie_start,
                    note.tie_stop,
                    note.orphan_after_forward,
                    host_summary
                )
            })
            .collect::<Vec<_>>()
            .join(" | ");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChordPhysicalBindingAmbiguity: locator fragment={} source={} attack={} m{} raw={} o{} dur={} p{} s{} chord={} tie_start={} tie_stop={} ignore_duration={}; canonical_peers=[{}]; candidate_matches=[{}]",
                locator.fragment_id,
                locator.source_event_id,
                locator.attack_id,
                locator.measure,
                locator.raw_note_index,
                locator.onset,
                locator.duration,
                locator.pitch_midi,
                locator.staff,
                locator.chord_syntax,
                locator.tie_start,
                locator.tie_stop,
                ignore_duration,
                canonical_peers,
                candidate_matches
            ),
        ));
    }
    Ok(first)
}

fn final_chord_measure_info(
    canonical: &CanonicalFinalChordStructuralPlan,
    measure: usize,
) -> io::Result<&FinalChordCanonicalMeasureInfo> {
    canonical
        .measures
        .iter()
        .find(|row| row.measure == measure)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("canonical FinalChord plan has no measure info for {measure}"),
            )
        })
}

fn final_chord_render_pruned_omission_is_certified(
    locator: &FinalChordCanonicalSourceLocator,
    render_omissible_chord_source: bool,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> bool {
    render_omissible_chord_source
        && locator.chord_syntax
        && render_omissions
            .is_some_and(|omissions| omissions.contains(&locator.fragment_id))
}

fn final_chord_render_pruned_tied_chain_is_fully_absent(
    measures: &[FinalChordBindingMeasure],
    locator: &FinalChordCanonicalSourceLocator,
    fragments: &[FinalChordCanonicalFragmentTarget],
) -> bool {
    let Some(measure) = measures.iter().find(|row| row.measure == locator.measure) else {
        return false;
    };
    fragments.iter().all(|fragment| {
        !measure.notes.iter().any(|note| {
            !note.rest_like
                && note.onset == fragment.target_onset
                && note.pitch_midi == Some(locator.pitch_midi)
                && note.staff == locator.staff
        })
    })
}

fn final_chord_record_canonical_input_binding(
    input_bindings: &mut Vec<FinalChordCanonicalInputBindingRow>,
    locator: &FinalChordCanonicalSourceLocator,
    note: &FinalChordEncodingNote,
) -> io::Result<()> {
    if note.rest_like || note.pitch_midi.is_none() || note.range.start >= note.range.end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord canonical input receipt cannot bind a non-pitched/empty occurrence",
        ));
    }
    if input_bindings
        .iter()
        .any(|row| row.fragment_id == locator.fragment_id)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord canonical input receipt has duplicate fragment binding",
        ));
    }
    if input_bindings.iter().any(|row| {
        row.source_note_start == note.range.start && row.source_note_end == note.range.end
    }) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord canonical input receipt has two owners for one physical input occurrence",
        ));
    }
    input_bindings.push(FinalChordCanonicalInputBindingRow {
        fragment_id: locator.fragment_id,
        source_event_id: locator.source_event_id,
        attack_id: locator.attack_id,
        source_note_start: note.range.start,
        source_note_end: note.range.end,
    });
    Ok(())
}

pub(crate) fn bind_canonical_final_chord_encoding_plan(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
) -> io::Result<FinalChordEncodingPlan> {
    Ok(bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
        candidate_xml,
        canonical,
        voice_plan,
        None,
    )?
    .plan)
}

#[cfg(test)]
fn bind_canonical_final_chord_encoding_plan_for_render_projection(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: &std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
) -> io::Result<FinalChordEncodingPlan> {
    Ok(bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
        candidate_xml,
        canonical,
        voice_plan,
        Some(render_omissions),
    )?
    .plan)
}

fn bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> io::Result<CanonicalBoundFinalChordEncodingPlan> {
    if !canonical.voice_free_evidence_sufficiency_green() || !voice_plan.is_bound() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord physical binding requires sealed canonical structure and serialized voice plan",
        ));
    }
    let measures = final_chord_parse_binding_measures(candidate_xml)?;
    let allow_render_occurrence_witness = render_omissions.is_some();
    let action_by_fragment = canonical
        .actions
        .iter()
        .map(|action| (action.fragment_id, action))
        .collect::<std::collections::BTreeMap<_, _>>();
    let locator_by_fragment = canonical
        .source_locators
        .iter()
        .map(|locator| (locator.fragment_id, locator))
        .collect::<std::collections::BTreeMap<_, _>>();

    let mut edits = Vec::<FinalChordEncodingEdit>::new();
    let mut forward_materializations = Vec::<FinalChordForwardMaterializationAuditRow>::new();
    let mut input_bindings = Vec::<FinalChordCanonicalInputBindingRow>::new();
    let mut certified_omitted_fragment_ids = std::collections::BTreeSet::<usize>::new();

    // First bind every source that survives in place. This stage may only select
    // bytes and materialize already-sealed label/retained-duration evidence; it
    // never chooses a role, run or temporal extent.
    for locator in canonical.source_locators.iter() {
        let action = action_by_fragment.get(&locator.fragment_id).copied();
        if matches!(
            action.map(|row| &row.kind),
            Some(FinalChordCanonicalActionKind::TiedAnchorChain { .. })
                | Some(FinalChordCanonicalActionKind::AuxiliaryLane { .. })
        ) {
            continue;
        }
        let ignore_duration = matches!(
            action.map(|row| &row.kind),
            Some(FinalChordCanonicalActionKind::RetainedChordMember { .. })
        );
        if final_chord_render_pruned_omission_is_certified(
            locator,
            matches!(
                action.map(|row| &row.kind),
                Some(FinalChordCanonicalActionKind::RetainedChordMember { .. })
            ),
            render_omissions,
        ) {
            if !certified_omitted_fragment_ids.insert(locator.fragment_id) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord canonical input receipt duplicated certified omission",
                ));
            }
            continue;
        }
        let note = final_chord_find_binding_note(
            &measures,
            canonical,
            locator,
            ignore_duration,
            allow_render_occurrence_witness,
        )?;
        final_chord_record_canonical_input_binding(&mut input_bindings, locator, note)?;
        let label = voice_plan.label_for_source(locator.source_event_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FrozenSerializedVoicePlan has no label for surviving SourceEventId {}",
                    locator.source_event_id
                ),
            )
        })?;
        let source = candidate_xml.get(note.range.clone()).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "bound source note bytes disappeared")
        })?;
        let replaced = final_chord_set_child_text(source, "voice", label).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord physical binding could not set sealed MusicXML voice label",
            )
        })?;
        let replaced = match action.map(|row| &row.kind) {
            Some(FinalChordCanonicalActionKind::RetainedChordMember {
                canonical_duration,
            }) => {
                let representation_duration = final_chord_render_representation_duration(
                    canonical,
                    locator.fragment_id,
                    allow_render_occurrence_witness,
                )
                .unwrap_or(*canonical_duration);
                if representation_duration <= 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "retained canonical chord duration must be positive",
                    ));
                }
                let retimed = final_chord_set_note_duration(&replaced, representation_duration)
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "retained canonical chord duration cannot be materialized",
                        )
                    })?;
                let divisions =
                    final_chord_measure_info(canonical, locator.measure)?.divisions;
                final_chord_sync_note_type_and_dots(
                    &retimed,
                    representation_duration,
                    divisions,
                )
            }
            _ => replaced,
        };
        if replaced != source {
            final_chord_push_edit(&mut edits, note.range.start, note.range.end, replaced);
        }
    }

    // Tied rematerialization is now driven only by the sealed action/fragment list.
    // A Render-only projection may legitimately have pruned the original hidden,
    // untied chord source before this canonical rematerialization boundary. In that
    // one case the whole generated hidden chain must still be absent; partial
    // materialization remains fail-closed.
    for action in canonical.actions.iter() {
        let FinalChordCanonicalActionKind::TiedAnchorChain { fragments } = &action.kind else {
            continue;
        };
        if final_chord_render_pruned_omission_is_certified(
            &action.locator,
            true,
            render_omissions,
        ) {
            if !final_chord_render_pruned_tied_chain_is_fully_absent(
                &measures,
                &action.locator,
                fragments,
            ) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord certified tied omission has a partial generated chain",
                ));
            }
            if !certified_omitted_fragment_ids.insert(action.locator.fragment_id) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord canonical input receipt duplicated certified tied omission",
                ));
            }
            continue;
        }
        let source_note = final_chord_find_binding_note(
            &measures,
            canonical,
            &action.locator,
            false,
            allow_render_occurrence_witness,
        )?;
        final_chord_record_canonical_input_binding(
            &mut input_bindings,
            &action.locator,
            source_note,
        )?;
        let source = candidate_xml.get(source_note.range.clone()).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "tied source bytes disappeared")
        })?;
        let label = voice_plan.label_for_source(action.source_event_id).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "tied source has no sealed voice label")
        })?;
        final_chord_push_edit(
            &mut edits,
            source_note.range.start,
            source_note.range.end,
            String::new(),
        );
        let projected_onset = fragments
            .first()
            .map(|fragment| fragment.target_onset)
            .unwrap_or(action.locator.onset);
        forward_materializations.push(FinalChordForwardMaterializationAuditRow {
            source_note_start: source_note.range.start,
            source_note_end: source_note.range.end,
            source_onset: action.locator.onset,
            projected_sounding_onset: projected_onset,
            generated_fragment_count: fragments.len(),
            kind: FinalChordForwardMaterializationKind::TiedAnchorChain,
        });
        for (fragment_ordinal, fragment) in fragments.iter().enumerate() {
            let anchor_locator = locator_by_fragment
                .get(&fragment.anchor_fragment_id)
                .copied()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "tied canonical fragment lost anchor locator",
                    )
                })?;
            let anchor = final_chord_find_binding_note(
                &measures,
                canonical,
                anchor_locator,
                false,
                allow_render_occurrence_witness,
            )?;
            let with_voice = final_chord_set_child_text(source, "voice", label).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "tied fragment cannot bind voice label")
            })?;
            let with_duration = final_chord_set_note_duration(&with_voice, fragment.duration)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "tied fragment cannot bind duration")
                })?;
            let divisions = final_chord_measure_info(canonical, action.locator.measure)?.divisions;
            let typed = final_chord_sync_note_type_and_dots(&with_duration, fragment.duration, divisions);
            let tied = final_chord_add_tie_markup(&typed, fragment.tie_start, fragment.tie_stop)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "tied fragment cannot bind tie markup")
                })?;
            let replacement = format!("\n{tied}");
            let replacement_note_end = replacement.len();
            final_chord_push_edit_with_generated_provenance(
                &mut edits,
                anchor.range.end,
                anchor.range.end,
                replacement,
                vec![FinalChordGeneratedFragmentEditProvenance {
                    source_note_start: source_note.range.start,
                    source_note_end: source_note.range.end,
                    fragment_ordinal,
                    kind: FinalChordForwardMaterializationKind::TiedAnchorChain,
                    replacement_note_start: 1,
                    replacement_note_end,
                }],
            );
        }
    }

    #[derive(Debug)]
    struct BoundAuxMember<'a> {
        action: &'a FinalChordCanonicalActionRow,
        note: &'a FinalChordEncodingNote,
        xml: String,
        duration: i64,
    }
    let mut aux_runs = std::collections::BTreeMap::<(usize, usize, String, String), Vec<BoundAuxMember<'_>>>::new();
    for action in canonical.actions.iter() {
        let FinalChordCanonicalActionKind::AuxiliaryLane {
            serialization_run_id,
            canonical_duration,
        } = &action.kind
        else {
            continue;
        };
        if final_chord_render_pruned_omission_is_certified(
            &action.locator,
            true,
            render_omissions,
        ) {
            if !certified_omitted_fragment_ids.insert(action.locator.fragment_id) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord canonical input receipt duplicated certified auxiliary omission",
                ));
            }
            continue;
        }
        let note = final_chord_find_binding_note(
            &measures,
            canonical,
            &action.locator,
            false,
            allow_render_occurrence_witness,
        )?;
        final_chord_record_canonical_input_binding(
            &mut input_bindings,
            &action.locator,
            note,
        )?;
        let source = candidate_xml.get(note.range.clone()).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "auxiliary source bytes disappeared")
        })?;
        let label = voice_plan
            .label_for_source(action.source_event_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "auxiliary source has no label"))?
            .to_string();
        let revoiced = final_chord_set_child_text(source, "voice", &label).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "auxiliary source cannot bind voice label")
        })?;
        let retimed = final_chord_set_note_duration(&revoiced, *canonical_duration).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "auxiliary source cannot bind duration")
        })?;
        let divisions = final_chord_measure_info(canonical, action.locator.measure)?.divisions;
        let xml = final_chord_sync_note_type_and_dots(&retimed, *canonical_duration, divisions);
        aux_runs
            .entry((
                action.locator.measure,
                *serialization_run_id,
                label.clone(),
                action.locator.staff.clone(),
            ))
            .or_default()
            .push(BoundAuxMember {
                action,
                note,
                xml,
                duration: *canonical_duration,
            });
    }

    for ((measure_ordinal, _run_id, label, staff), mut members) in aux_runs {
        let measure = measures
            .iter()
            .find(|measure| measure.measure == measure_ordinal)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "auxiliary run lost measure"))?;
        let expected_duration = final_chord_measure_info(canonical, measure_ordinal)?.expected_duration;
        if expected_duration != measure.expected_duration {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "auxiliary physical binding measure duration drift",
            ));
        }
        members.sort_by_key(|member| (member.action.locator.onset, member.action.locator.raw_note_index));
        let mut lane = format!("\n      <backup><duration>{expected_duration}</duration></backup>");
        let mut lane_generated_fragments =
            Vec::<FinalChordGeneratedFragmentEditProvenance>::new();
        let mut lane_cursor = 0_i64;
        let mut start = 0usize;
        while start < members.len() {
            let onset = members[start].action.locator.onset;
            let mut end = start + 1;
            while end < members.len() && members[end].action.locator.onset == onset {
                end += 1;
            }
            let group = &members[start..end];
            let anchor_offset = group
                .iter()
                .enumerate()
                .max_by_key(|(_, member)| (member.duration, std::cmp::Reverse(member.action.locator.raw_note_index)))
                .map(|(offset, _)| offset)
                .unwrap_or(0);
            let gap = onset.saturating_sub(lane_cursor);
            if onset < lane_cursor || gap < 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "sealed auxiliary serialization run overlaps itself",
                ));
            }
            if gap > 0 {
                lane.push_str(&format!(
                    "\n      <forward><duration>{gap}</duration><voice>{label}</voice><staff>{staff}</staff></forward>"
                ));
            }
            let mut order = (0..group.len()).collect::<Vec<_>>();
            order.sort_by_key(|&offset| {
                (
                    usize::from(offset != anchor_offset),
                    group[offset].action.locator.raw_note_index,
                )
            });
            for offset in order {
                let member = &group[offset];
                let note_xml = if offset == anchor_offset {
                    final_chord_remove_simple_child_blocks(member.xml.clone(), "chord")
                } else {
                    member.xml.clone()
                };
                lane.push('\n');
                let replacement_note_start = lane.len();
                lane.push_str(&note_xml);
                let replacement_note_end = lane.len();
                lane_generated_fragments.push(FinalChordGeneratedFragmentEditProvenance {
                    source_note_start: member.note.range.start,
                    source_note_end: member.note.range.end,
                    fragment_ordinal: 0,
                    kind: FinalChordForwardMaterializationKind::AuxiliaryLane,
                    replacement_note_start,
                    replacement_note_end,
                });
            }
            lane_cursor = onset.saturating_add(group[anchor_offset].duration);
            start = end;
        }
        let tail = expected_duration.saturating_sub(lane_cursor);
        if tail < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "sealed auxiliary serialization run exceeds measure duration",
            ));
        }
        if tail > 0 {
            lane.push_str(&format!(
                "\n      <forward><duration>{tail}</duration><voice>{label}</voice><staff>{staff}</staff></forward>"
            ));
        }
        for member in &members {
            final_chord_push_edit(
                &mut edits,
                member.note.range.start,
                member.note.range.end,
                String::new(),
            );
            forward_materializations.push(FinalChordForwardMaterializationAuditRow {
                source_note_start: member.note.range.start,
                source_note_end: member.note.range.end,
                source_onset: member.action.locator.onset,
                projected_sounding_onset: member.action.locator.onset,
                generated_fragment_count: 1,
                kind: FinalChordForwardMaterializationKind::AuxiliaryLane,
            });
        }
        final_chord_push_edit_with_generated_provenance(
            &mut edits,
            measure.measure_close,
            measure.measure_close,
            lane,
            lane_generated_fragments,
        );
    }

    input_bindings.sort_by_key(|row| row.fragment_id);
    let bound_fragment_ids = input_bindings
        .iter()
        .map(|row| row.fragment_id)
        .collect::<std::collections::BTreeSet<_>>();
    if !bound_fragment_ids
        .is_disjoint(&certified_omitted_fragment_ids)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord canonical input receipt marks one fragment both bound and omitted",
        ));
    }
    let expected_fragment_ids = canonical
        .source_locators
        .iter()
        .map(|locator| locator.fragment_id)
        .collect::<std::collections::BTreeSet<_>>();
    let actual_fragment_ids = bound_fragment_ids
        .union(&certified_omitted_fragment_ids)
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if actual_fragment_ids != expected_fragment_ids {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChord canonical input receipt coverage drift expected={} actual={}",
                expected_fragment_ids.len(),
                actual_fragment_ids.len()
            ),
        ));
    }

    let mut plan = final_chord_finalize_bound_plan(
        candidate_xml, edits, forward_materializations, true,
    )?;
    plan.original_respelling_input = canonical.original_respelling_input.clone();
    plan.original_course_bindings = Some(course_capture::OriginalCourseBindings::from_original_bind(
        &input_bindings, canonical,
    ).map(std::sync::Arc::new).map_err(|error| error.to_string()));
    Ok(CanonicalBoundFinalChordEncodingPlan {
        plan, input_bindings, certified_omitted_fragment_ids,
    })
}

fn final_chord_shift_projection_offset(value: usize, delta: i128) -> io::Result<usize> {
    usize::try_from(value as i128 + delta).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord projection provenance offset moved out of range",
        )
    })
}

fn final_chord_non_pitched_continuations(
    source_xml: &str,
    plan: &FinalChordEncodingPlan,
    rest_edits: &[FinalChordEncodingEdit],
    projected_xml: &str,
) -> io::Result<Vec<FinalChordNonPitchedContinuation>> {
    let invalid = || io::Error::new(io::ErrorKind::InvalidData,
        "FinalChord non-pitched continuation exact coverage drift; role origin remains Unknown");
    let projected = roxmltree::Document::parse(projected_xml).map_err(|_| invalid())?;
    let mut unclaimed_notes = projected.descendants()
        .filter(|node| node.has_tag_name("note") &&
            (!node.children().any(|child| child.has_tag_name("pitch")) ||
                node.children().any(|child| child.has_tag_name("rest"))))
        .map(|node| (node.range().start, node.range().end))
        .collect::<std::collections::BTreeMap<_, _>>();
    let projected_forwards = projected.descendants()
        .filter(|node| node.has_tag_name("forward"))
        .map(|node| (node.range().start, node.range().end))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows = Vec::new();
    for measure in final_chord_parse_binding_measures(source_xml)? {
        for note in measure.notes {
            if !note.rest_like && note.pitch_midi.is_some() {
                continue;
            }
            let before_rest_start = plan.carry_surviving_source_note_start(
                note.range.start, note.range.end).map_err(|_| invalid())?;
            let mut delta = 0_i128;
            let mut replacement = None;
            for edit in rest_edits {
                if edit.start == before_rest_start {
                    if replacement.replace(edit).is_some() { return Err(invalid()); }
                } else if edit.end <= before_rest_start {
                    delta += edit.replacement.len() as i128 -
                        edit.end.saturating_sub(edit.start) as i128;
                } else if edit.start < before_rest_start {
                    return Err(invalid());
                }
            }
            let start = final_chord_shift_projection_offset(before_rest_start, delta)?;
            if let Some(edit) = replacement {
                if !note.rest_like { return Err(invalid()); }
                // Read the actual owner's replacement fragments. No time/pitch/voice
                // matching joins a source to an independently parsed output note.
                let wrapped = format!("<continuation>{}</continuation>", edit.replacement);
                let fragments = roxmltree::Document::parse(&wrapped).map_err(|_| invalid())?;
                let mut duration = 0_i64;
                let mut fragment_count = 0;
                for (ordinal, fragment) in fragments.root_element().children()
                    .filter(roxmltree::Node::is_element).enumerate()
                {
                    let carrier = if fragment.has_tag_name("note") &&
                        fragment.children().any(|child| child.has_tag_name("rest")) {
                        FinalChordNonPitchedCarrier::Note
                    } else if fragment.has_tag_name("forward") {
                        FinalChordNonPitchedCarrier::Forward
                    } else { return Err(invalid()); };
                    let fragment_duration = final_chord_direct_child_text(fragment, "duration")
                        .and_then(|value| value.parse::<i64>().ok()).filter(|value| *value > 0)
                        .ok_or_else(invalid)?;
                    duration = duration.checked_add(fragment_duration).ok_or_else(invalid)?;
                    let projected_start = start.checked_add(fragment.range().start -
                        "<continuation>".len()).ok_or_else(invalid)?;
                    let projected_end = projected_start.checked_add(fragment.range().len())
                        .ok_or_else(invalid)?;
                    let actual_end = match carrier {
                        FinalChordNonPitchedCarrier::Note => unclaimed_notes.remove(&projected_start),
                        FinalChordNonPitchedCarrier::Forward => projected_forwards.get(&projected_start).copied(),
                    };
                    if actual_end != Some(projected_end) ||
                        projected_xml.get(projected_start..projected_end) != wrapped.get(fragment.range()) {
                        return Err(invalid());
                    }
                    rows.push(FinalChordNonPitchedContinuation {
                        source_note_start: note.range.start, source_note_end: note.range.end,
                        projected_start, projected_end, fragment_ordinal: ordinal,
                        carrier, presentation_role: None,
                    });
                    fragment_count += 1;
                }
                if fragment_count == 0 || duration != note.duration { return Err(invalid()); }
            } else {
                let end = unclaimed_notes.remove(&start).ok_or_else(invalid)?;
                let expected_fragment = plan.edits.iter()
                    .find(|edit| edit.start == note.range.start && edit.end == note.range.end)
                    .map(|edit| edit.replacement.as_str())
                    .or_else(|| source_xml.get(note.range.clone())).ok_or_else(invalid)?;
                if projected_xml.get(start..end) != Some(expected_fragment) {
                    return Err(invalid());
                }
                rows.push(FinalChordNonPitchedContinuation {
                    source_note_start: note.range.start, source_note_end: note.range.end,
                    projected_start: start, projected_end: end, fragment_ordinal: 0,
                    carrier: FinalChordNonPitchedCarrier::Note, presentation_role: None,
                });
            }
        }
    }
    if !unclaimed_notes.is_empty() { return Err(invalid()); }
    rows.sort_by_key(|row| row.projected_start);
    Ok(rows)
}

pub(crate) fn project_final_chord_encoding_plan_with_provenance(
    candidate_xml: &str,
    plan: &FinalChordEncodingPlan,
) -> io::Result<FinalChordProjectionResult> {
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    let result = project_final_chord_encoding_plan_with_provenance_with_receipts(candidate_xml, plan, &mut receipts);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    result
}

pub(crate) fn project_final_chord_encoding_plan_with_provenance_with_receipts(
    candidate_xml: &str,
    plan: &FinalChordEncodingPlan,
    receipts: &mut crate::master_state_identity::RenderTransitionReceipts,
) -> io::Result<FinalChordProjectionResult> {
    // The ordinary projector remains the sole byte-materialization path. This closure only
    // derives a transaction-local receipt from the already-bound plan; it does not
    // introduce a second XML projector or a second semantic owner.
    //
    // The canonical visible-rest interval projection is a byte-only post-projection of
    // the already-bound FinalChord plan. Provenance therefore has to carry existing
    // pitched-note offsets through those rest-only edits as well. The edits themselves
    // are re-derived from the exact pre-rest projection and remain non-authoritative here.
    let visible_rest_interval_edits = if plan.visible_rest_interval_projection_authorized {
        let mut pre_rest_plan = plan.clone();
        pre_rest_plan.visible_rest_interval_projection_authorized = false;
        let (pre_rest_result, probe_receipt) = project_final_chord_encoding_plan(candidate_xml, &pre_rest_plan);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
        probe.push_exact(probe_receipt);
        probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        let pre_rest_xml = pre_rest_result?;
        final_chord_plan_visible_rest_interval_edits(&pre_rest_xml)?
    } else {
        Vec::new()
    };
    let (result, receipt) = project_final_chord_encoding_plan(candidate_xml, plan);
    receipts.push_exact(receipt);
    let out = result?;
    final_chord_collect_original_projection(candidate_xml, plan, &visible_rest_interval_edits, out)
}

fn final_chord_collect_original_projection(
    candidate_xml: &str, plan: &FinalChordEncodingPlan,
    visible_rest_interval_edits: &[FinalChordEncodingEdit], out: String,
) -> io::Result<FinalChordProjectionResult> {
    let mut generated_provenance = Vec::<FinalChordProjectionProvenanceRow>::new();
    for edit in &plan.edits {
        let delta = edit.replacement.len() as i128
            - edit.end.saturating_sub(edit.start) as i128;
        for row in &mut generated_provenance {
            if row.projected_note_start >= edit.end {
                row.projected_note_start =
                    final_chord_shift_projection_offset(row.projected_note_start, delta)?;
                row.projected_note_end =
                    final_chord_shift_projection_offset(row.projected_note_end, delta)?;
            } else if row.projected_note_end > edit.start {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord generated provenance overlaps a later edit",
                ));
            }
        }
        for fragment in &edit.generated_fragments {
            if fragment.source_note_start >= fragment.source_note_end
                || fragment.source_note_end > plan.source_len
                || fragment.replacement_note_start >= fragment.replacement_note_end
                || fragment.replacement_note_end > edit.replacement.len()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord generated provenance span is invalid",
                ));
            }
            generated_provenance.push(FinalChordProjectionProvenanceRow {
                source_note_start: fragment.source_note_start,
                source_note_end: fragment.source_note_end,
                projected_note_start: edit.start + fragment.replacement_note_start,
                projected_note_end: edit.start + fragment.replacement_note_end,
                fragment_ordinal: fragment.fragment_ordinal,
                kind: match fragment.kind {
                    FinalChordForwardMaterializationKind::TiedAnchorChain => {
                        FinalChordProjectionMaterializationKind::TiedAnchorChain
                    }
                    FinalChordForwardMaterializationKind::AuxiliaryLane => {
                        FinalChordProjectionMaterializationKind::AuxiliaryLane
                    }
                },
            });
        }
    }

    let rematerialized_source_ranges = plan
        .forward_materializations
        .iter()
        .map(|row| (row.source_note_start, row.source_note_end))
        .collect::<std::collections::BTreeSet<_>>();
    let mut provenance = generated_provenance;
    for measure in final_chord_parse_binding_measures(candidate_xml)? {
        for note in measure.notes {
            if note.rest_like || note.pitch_midi.is_none() {
                continue;
            }
            let source_range = (note.range.start, note.range.end);
            if rematerialized_source_ranges.contains(&source_range) {
                continue;
            }
            let projected_note_start = plan
                .carry_surviving_source_note_start(note.range.start, note.range.end)
                .map_err(|reason| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "FinalChordProjectionIdentityTransport: surviving source note cannot be carried: {reason}"
                        ),
                    )
                })?;
            provenance.push(FinalChordProjectionProvenanceRow {
                source_note_start: note.range.start,
                source_note_end: note.range.end,
                projected_note_start,
                projected_note_end: 0,
                fragment_ordinal: 0,
                kind: FinalChordProjectionMaterializationKind::SurvivingSourceNote,
            });
        }
    }

    if !visible_rest_interval_edits.is_empty() {
        for row in &mut provenance {
            let carry_offset = |value: usize| -> io::Result<usize> {
                let mut delta = 0_i128;
                for edit in visible_rest_interval_edits {
                    if edit.end <= value {
                        delta += edit.replacement.len() as i128
                            - edit.end.saturating_sub(edit.start) as i128;
                    } else if edit.start < value && value < edit.end {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "FinalChordProjectionIdentityTransport: pitched provenance overlaps visible-rest interval edit",
                        ));
                    }
                }
                final_chord_shift_projection_offset(value, delta)
            };

            row.projected_note_start = carry_offset(row.projected_note_start)?;
            if row.projected_note_end != 0 {
                row.projected_note_end = carry_offset(row.projected_note_end)?;
            }
        }
    }

    let projected_pitched_ranges = final_chord_parse_binding_measures(&out)?
        .into_iter()
        .flat_map(|measure| measure.notes)
        .filter(|note| !note.rest_like && note.pitch_midi.is_some())
        .map(|note| (note.range.start, note.range.end))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut claimed_projected_starts = std::collections::BTreeSet::<usize>::new();
    for row in &mut provenance {
        let Some(projected_note_end) = projected_pitched_ranges.get(&row.projected_note_start).copied()
        else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FinalChordProjectionIdentityTransport: plan provenance has no projected pitched note at byte {}",
                    row.projected_note_start
                ),
            ));
        };
        if row.projected_note_end != 0 && row.projected_note_end != projected_note_end {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChordProjectionIdentityTransport: generated note span drift",
            ));
        }
        row.projected_note_end = projected_note_end;
        if !claimed_projected_starts.insert(row.projected_note_start) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChordProjectionIdentityTransport: projected pitched note has multiple plan provenances",
            ));
        }
    }
    let projected_starts = projected_pitched_ranges
        .keys()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if claimed_projected_starts != projected_starts {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChordProjectionIdentityTransport: projected pitched coverage drift expected={} actual={}",
                projected_starts.len(),
                claimed_projected_starts.len()
            ),
        ));
    }
    for materialization in &plan.forward_materializations {
        let expected_kind = match materialization.kind {
            FinalChordForwardMaterializationKind::TiedAnchorChain => {
                FinalChordProjectionMaterializationKind::TiedAnchorChain
            }
            FinalChordForwardMaterializationKind::AuxiliaryLane => {
                FinalChordProjectionMaterializationKind::AuxiliaryLane
            }
        };
        let actual_rows = provenance
            .iter()
            .filter(|row| {
                row.kind == expected_kind
                    && row.source_note_start == materialization.source_note_start
                    && row.source_note_end == materialization.source_note_end
            })
            .collect::<Vec<_>>();
        if actual_rows.len() != materialization.generated_fragment_count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FinalChordProjectionIdentityTransport: generated fragment coverage drift expected={} actual={} kind={}",
                    materialization.generated_fragment_count,
                    actual_rows.len(),
                    materialization.kind.label()
                ),
            ));
        }
        let ordinals = actual_rows
            .iter()
            .map(|row| row.fragment_ordinal)
            .collect::<std::collections::BTreeSet<_>>();
        let expected_ordinals = (0..materialization.generated_fragment_count)
            .collect::<std::collections::BTreeSet<_>>();
        if ordinals != expected_ordinals {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FinalChordProjectionIdentityTransport: generated fragment ordinal drift kind={}",
                    materialization.kind.label()
                ),
            ));
        }
    }
    provenance.sort_by_key(|row| row.projected_note_start);
    let non_pitched_continuations = final_chord_non_pitched_continuations(
        candidate_xml, plan, visible_rest_interval_edits, &out)?;
    Ok(FinalChordProjectionResult {
        xml: out,
        provenance,
        non_pitched_continuations,
    })
}


fn final_chord_transaction_output_alpha_topology_fingerprint(
    measures: &[FinalChordBindingMeasure],
) -> u64 {
    let mut signature = String::from("ETUSCHLOPF_FINAL_CHORD_TRANSACTION_OUTPUT_ALPHA_TOPOLOGY_V1\n");
    for measure in measures {
        let mut pitched_ordinal = 0usize;
        for note in &measure.notes {
            if note.rest_like || note.pitch_midi.is_none() {
                continue;
            }
            let _ = writeln!(
                signature,
                "P|m{}|ord{}|o{}|d{}|p{}|s{}|c{}|ts{}|te{}",
                measure.measure,
                pitched_ordinal,
                note.onset,
                note.duration,
                note.pitch_midi.unwrap_or_default(),
                note.staff,
                u8::from(note.chord_member),
                u8::from(note.tie_start),
                u8::from(note.tie_stop),
            );
            pitched_ordinal = pitched_ordinal.saturating_add(1);
        }
    }
    fingerprint(&signature)
}

fn final_chord_projection_materialization_kind_rank(
    kind: FinalChordProjectionMaterializationKind,
) -> u8 {
    match kind {
        FinalChordProjectionMaterializationKind::SurvivingSourceNote => 0,
        FinalChordProjectionMaterializationKind::TiedAnchorChain => 1,
        FinalChordProjectionMaterializationKind::AuxiliaryLane => 2,
    }
}

fn project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    bound: &CanonicalBoundFinalChordEncodingPlan,
) -> io::Result<CanonicalProjectedFinalChord> {
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    let result = project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha_with_receipts(candidate_xml, canonical, bound, &mut receipts);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    result
}

fn project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha_with_receipts(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    bound: &CanonicalBoundFinalChordEncodingPlan,
    receipts: &mut crate::master_state_identity::RenderTransitionReceipts,
) -> io::Result<CanonicalProjectedFinalChord> {
    if !canonical.voice_free_evidence_sufficiency_green() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord transaction-local Output-Alpha projection requires sealed canonical structure",
        ));
    }
    let projection = project_final_chord_encoding_plan_with_provenance_with_receipts(candidate_xml, &bound.plan, receipts)?;

    let mut owner_by_source_range = std::collections::BTreeMap::<
        (usize, usize),
        &FinalChordCanonicalInputBindingRow,
    >::new();
    for row in &bound.input_bindings {
        if row.source_note_start >= row.source_note_end
            || row.source_note_end > bound.plan.source_len
            || owner_by_source_range
                .insert((row.source_note_start, row.source_note_end), row)
                .is_some()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha input receipt has invalid/duplicate source span",
            ));
        }
    }

    let measures = final_chord_parse_binding_measures(&projection.xml)?;
    let output_topology_fingerprint_fnv64 =
        final_chord_transaction_output_alpha_topology_fingerprint(&measures);
    let mut occurrence_by_projected_range = std::collections::BTreeMap::<
        (usize, usize),
        (usize, usize),
    >::new();
    for measure in &measures {
        let mut pitched_ordinal = 0usize;
        for note in &measure.notes {
            if note.rest_like || note.pitch_midi.is_none() {
                continue;
            }
            if occurrence_by_projected_range
                .insert(
                    (note.range.start, note.range.end),
                    (measure.measure, pitched_ordinal),
                )
                .is_some()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha found duplicate projected note span",
                ));
            }
            pitched_ordinal = pitched_ordinal.saturating_add(1);
        }
    }

    let locator_by_fragment = canonical
        .source_locators
        .iter()
        .map(|locator| (locator.fragment_id, locator))
        .collect::<std::collections::BTreeMap<_, _>>();
    let action_by_fragment = canonical
        .actions
        .iter()
        .map(|action| (action.fragment_id, action))
        .collect::<std::collections::BTreeMap<_, _>>();

    let mut occurrences = Vec::<FinalChordTransactionOutputOccurrenceWitness>::new();
    let mut claimed_output_occurrences = std::collections::BTreeSet::<(usize, usize)>::new();
    let mut claimed_occurrence_keys = std::collections::BTreeSet::<(usize, u8, usize)>::new();
    for row in &projection.provenance {
        let Some(owner) = owner_by_source_range
            .get(&(row.source_note_start, row.source_note_end))
            .copied()
        else {
            // The projected XML may contain pitched material outside the canonical
            // FinalChord owner set (for example a physical host). Such notes remain
            // in the topology freshness seal but receive no canonical owner receipt.
            continue;
        };
        let (measure, pitched_ordinal) = occurrence_by_projected_range
            .get(&(row.projected_note_start, row.projected_note_end))
            .copied()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha lost exact projected occurrence",
                )
            })?;
        if !claimed_output_occurrences.insert((measure, pitched_ordinal)) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha assigned two canonical owners to one projected occurrence",
            ));
        }
        let occurrence_key = (
            owner.fragment_id,
            final_chord_projection_materialization_kind_rank(row.kind),
            row.fragment_ordinal,
        );
        if !claimed_occurrence_keys.insert(occurrence_key) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha duplicated canonical output occurrence key",
            ));
        }
        let locator = locator_by_fragment
            .get(&owner.fragment_id)
            .copied()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha input receipt references unknown fragment",
                )
            })?;
        if locator.source_event_id != owner.source_event_id
            || locator.attack_id != owner.attack_id
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha input receipt disagrees with canonical Source/Attack owner",
            ));
        }
        occurrences.push(FinalChordTransactionOutputOccurrenceWitness {
            fragment_id: owner.fragment_id,
            source_event_id: owner.source_event_id,
            attack_id: owner.attack_id,
            materialization_kind: row.kind,
            fragment_ordinal: row.fragment_ordinal,
            measure,
            pitched_ordinal,
        });
    }

    occurrences.sort_by_key(|row| {
        (
            row.fragment_id,
            final_chord_projection_materialization_kind_rank(row.materialization_kind),
            row.fragment_ordinal,
            row.measure,
            row.pitched_ordinal,
        )
    });

    let mut occurrences_by_fragment = std::collections::BTreeMap::<
        usize,
        Vec<&FinalChordTransactionOutputOccurrenceWitness>,
    >::new();
    for row in &occurrences {
        occurrences_by_fragment
            .entry(row.fragment_id)
            .or_default()
            .push(row);
    }
    for rows in occurrences_by_fragment.values_mut() {
        rows.sort_by_key(|row| row.fragment_ordinal);
    }

    for locator in canonical.source_locators.iter() {
        let rows = occurrences_by_fragment
            .get(&locator.fragment_id)
            .map(|rows| rows.as_slice())
            .unwrap_or(&[]);
        let omitted = bound
            .certified_omitted_fragment_ids
            .contains(&locator.fragment_id);
        if omitted {
            if !rows.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha materialized an owner already certified absent",
                ));
            }
            continue;
        }
        let action = action_by_fragment.get(&locator.fragment_id).copied();
        match action.map(|action| &action.kind) {
            Some(FinalChordCanonicalActionKind::TiedAnchorChain { fragments }) => {
                if fragments.is_empty() || rows.len() != fragments.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha tied occurrence cardinality drift",
                    ));
                }
                for (ordinal, row) in rows.iter().enumerate() {
                    if row.materialization_kind
                        != FinalChordProjectionMaterializationKind::TiedAnchorChain
                        || row.fragment_ordinal != ordinal
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "FinalChord transaction-local Output-Alpha tied occurrence provenance drift",
                        ));
                    }
                }
            }
            Some(FinalChordCanonicalActionKind::AuxiliaryLane { .. }) => {
                if rows.len() != 1
                    || rows[0].materialization_kind
                        != FinalChordProjectionMaterializationKind::AuxiliaryLane
                    || rows[0].fragment_ordinal != 0
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha auxiliary occurrence provenance drift",
                    ));
                }
            }
            _ => {
                if rows.len() != 1
                    || rows[0].materialization_kind
                        != FinalChordProjectionMaterializationKind::SurvivingSourceNote
                    || rows[0].fragment_ordinal != 0
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha surviving occurrence provenance drift",
                    ));
                }
            }
        }
    }

    let rest_bridge_extension_course = canonical.rest_bridge_extension_course.as_ref()
        .map(|course| course.projected(candidate_xml, &projection.xml, &occurrences)).transpose()?;
    Ok(CanonicalProjectedFinalChord {
        xml: projection.xml,
        receipt: FinalChordTransactionLocalOutputAlphaReceipt {
            rest_bridge_extension_course,
            phrase_origin: std::sync::Arc::new(()),
            canonical_semantic_fingerprint_fnv64: canonical.semantic_fingerprint_fnv64(),
            bound_source_fingerprint_fnv64: bound.plan.source_fingerprint,
            bound_source_len: bound.plan.source_len,
            output_topology_fingerprint_fnv64,
            occurrences,
            certified_omitted_fragment_ids: bound.certified_omitted_fragment_ids.clone(),
            original_beam_voice_groups: canonical.original_beam_voice_groups.clone(),
        },
    })
}

#[cfg(test)]
pub(crate) fn project_canonical_final_chord_render_occurrence_continuation(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: &std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
) -> io::Result<CanonicalFinalChordRenderOccurrenceContinuation> {
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    let result = project_canonical_final_chord_render_occurrence_continuation_with_receipts(candidate_xml, canonical, voice_plan, render_omissions, &mut receipts);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    result
}

pub(crate) fn project_canonical_final_chord_render_occurrence_continuation_with_receipts(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: &std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
    receipts: &mut crate::master_state_identity::RenderTransitionReceipts,
) -> io::Result<CanonicalFinalChordRenderOccurrenceContinuation> {
    let bound = bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
        candidate_xml,
        canonical,
        voice_plan,
        Some(render_omissions),
    )?;
    let projected =
        project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha_with_receipts(
            candidate_xml,
            canonical,
            &bound,
            receipts,
        )?;
    verify_canonical_final_chord_transaction_local_output_alpha(
        &projected.xml,
        canonical,
        voice_plan,
        Some(render_omissions),
        &projected.receipt,
        bound.plan.source_fingerprint,
        bound.plan.source_len,
    )?;
    Ok(CanonicalFinalChordRenderOccurrenceContinuation { projected })
}

pub(crate) fn advance_canonical_final_chord_render_occurrence_after_final_notation_checkpoint(
    source: CanonicalFinalChordRenderOccurrenceContinuation,
    checkpoint_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: &std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
) -> io::Result<CanonicalFinalChordRenderOccurrenceContinuation> {
    final_chord_verify_pitched_sequence_transition(source.xml(), checkpoint_xml, false).map_err(
        |error| {
            io::Error::new(
                error.kind(),
                format!(
                    "FinalChord render-occurrence continuation cannot cross the FinalNotation checkpoint: {error}"
                ),
            )
        },
    )?;

    let mut projected = source.projected;
    let checkpoint_measures = final_chord_parse_binding_measures(checkpoint_xml)?;
    projected.xml = checkpoint_xml.to_string();
    projected.receipt.output_topology_fingerprint_fnv64 =
        final_chord_transaction_output_alpha_topology_fingerprint(&checkpoint_measures);
    verify_canonical_final_chord_transaction_local_output_alpha(
        &projected.xml,
        canonical,
        voice_plan,
        Some(render_omissions),
        &projected.receipt,
        projected.receipt.bound_source_fingerprint_fnv64,
        projected.receipt.bound_source_len,
    )?;
    Ok(CanonicalFinalChordRenderOccurrenceContinuation { projected })
}

fn final_chord_transaction_output_alpha_map(
    projected: &CanonicalProjectedFinalChord,
    mapping: &std::collections::BTreeMap<String, String>,
) -> io::Result<CanonicalProjectedFinalChord> {
    Ok(CanonicalProjectedFinalChord {
        xml: final_chord_map_all_voice_elements(&projected.xml, mapping)?,
        receipt: projected.receipt.clone(),
    })
}

fn final_chord_transaction_output_occurrence_note<'a>(
    measures: &'a [FinalChordBindingMeasure],
    witness: &FinalChordTransactionOutputOccurrenceWitness,
) -> io::Result<&'a FinalChordEncodingNote> {
    let measure = measures
        .iter()
        .find(|measure| measure.measure == witness.measure)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha lost witnessed measure",
            )
        })?;
    measure
        .notes
        .iter()
        .filter(|note| !note.rest_like && note.pitch_midi.is_some())
        .nth(witness.pitched_ordinal)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha lost witnessed pitched ordinal",
            )
        })
}

fn final_chord_output_alpha_owned_transient_render_orphan(
    note: &FinalChordEncodingNote,
    owner: Option<&FinalChordTransactionOutputOccurrenceWitness>,
    locator_by_fragment: &std::collections::BTreeMap<usize, &FinalChordCanonicalSourceLocator>,
    render_projection: bool,
) -> bool {
    if !render_projection
        || !note.chord_member
        || !note.orphan_after_forward
        || note.anchor_index.is_some()
    {
        return false;
    }
    let Some(owner) = owner else {
        return false;
    };
    if owner.materialization_kind != FinalChordProjectionMaterializationKind::SurvivingSourceNote {
        return false;
    }
    let Some(locator) = locator_by_fragment.get(&owner.fragment_id).copied() else {
        return false;
    };
    owner.source_event_id == locator.source_event_id
        && owner.attack_id == locator.attack_id
        && owner.measure == locator.measure
        && note.measure == owner.measure
        && note.pitch_midi == Some(locator.pitch_midi)
        && note.staff == locator.staff
        && note.chord_member == locator.chord_syntax
        && note.tie_start == locator.tie_start
        && note.tie_stop == locator.tie_stop
}

fn verify_canonical_final_chord_transaction_local_output_alpha(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
    receipt: &FinalChordTransactionLocalOutputAlphaReceipt,
    expected_bound_source_fingerprint_fnv64: u64,
    expected_bound_source_len: usize,
) -> io::Result<()> {
    verify_canonical_final_chord_occurrence_receipt(
        candidate_xml,
        canonical,
        voice_plan,
        render_omissions,
        receipt,
        Some((
            expected_bound_source_fingerprint_fnv64,
            expected_bound_source_len,
        )),
        true,
    )
}

fn verify_canonical_final_chord_occurrence_receipt(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
    receipt: &FinalChordTransactionLocalOutputAlphaReceipt,
    expected_bound_source: Option<(u64, usize)>,
    require_canonical_chord_syntax: bool,
) -> io::Result<()> {
    if !canonical.voice_free_evidence_sufficiency_green() || !voice_plan.is_bound() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord transaction-local Output-Alpha readback requires sealed canonical structure and label binding",
        ));
    }
    if receipt.canonical_semantic_fingerprint_fnv64 != canonical.semantic_fingerprint_fnv64() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord transaction-local Output-Alpha receipt belongs to a different canonical plan",
        ));
    }
    if let Some((expected_bound_source_fingerprint_fnv64, expected_bound_source_len)) =
        expected_bound_source
        && (receipt.bound_source_fingerprint_fnv64 != expected_bound_source_fingerprint_fnv64
            || receipt.bound_source_len != expected_bound_source_len)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord occurrence receipt belongs to a different bound source epoch",
        ));
    }
    let measures = final_chord_parse_binding_measures(candidate_xml)?;
    let output_topology_fingerprint_fnv64 =
        final_chord_transaction_output_alpha_topology_fingerprint(&measures);
    if output_topology_fingerprint_fnv64 != receipt.output_topology_fingerprint_fnv64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord transaction-local Output-Alpha topology drift",
        ));
    }
    let action_by_fragment = canonical
        .actions
        .iter()
        .map(|action| (action.fragment_id, action))
        .collect::<std::collections::BTreeMap<_, _>>();
    let locator_by_fragment = canonical
        .source_locators
        .iter()
        .map(|locator| (locator.fragment_id, locator))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut rows_by_fragment = std::collections::BTreeMap::<
        usize,
        Vec<&FinalChordTransactionOutputOccurrenceWitness>,
    >::new();
    let mut claimed_occurrences = std::collections::BTreeSet::<(usize, usize)>::new();
    let mut claimed_keys = std::collections::BTreeSet::<(usize, u8, usize)>::new();
    for row in &receipt.occurrences {
        let locator = locator_by_fragment
            .get(&row.fragment_id)
            .copied()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha receipt references unknown fragment",
                )
            })?;
        if locator.source_event_id != row.source_event_id || locator.attack_id != row.attack_id {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha receipt disagrees with canonical Source/Attack owner",
            ));
        }
        if !claimed_occurrences.insert((row.measure, row.pitched_ordinal)) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha receipt claims one output occurrence twice",
            ));
        }
        if !claimed_keys.insert((
            row.fragment_id,
            final_chord_projection_materialization_kind_rank(row.materialization_kind),
            row.fragment_ordinal,
        )) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha receipt duplicates canonical occurrence key",
            ));
        }
        let _ = final_chord_transaction_output_occurrence_note(&measures, row)?;
        rows_by_fragment.entry(row.fragment_id).or_default().push(row);
    }
    for rows in rows_by_fragment.values_mut() {
        rows.sort_by_key(|row| row.fragment_ordinal);
    }

    for locator in canonical.source_locators.iter() {
        let action = action_by_fragment.get(&locator.fragment_id).copied();
        let omitted = receipt
            .certified_omitted_fragment_ids
            .contains(&locator.fragment_id);
        if omitted {
            let omission_green = match action.map(|action| &action.kind) {
                Some(FinalChordCanonicalActionKind::TiedAnchorChain { fragments }) => {
                    final_chord_render_pruned_omission_is_certified(
                        locator,
                        true,
                        render_omissions,
                    ) && final_chord_render_pruned_tied_chain_is_fully_absent(
                        &measures,
                        locator,
                        fragments,
                    )
                }
                Some(FinalChordCanonicalActionKind::AuxiliaryLane { .. })
                | Some(FinalChordCanonicalActionKind::RetainedChordMember { .. }) => {
                    final_chord_render_pruned_omission_is_certified(
                        locator,
                        true,
                        render_omissions,
                    )
                }
                _ => final_chord_render_pruned_omission_is_certified(
                    locator,
                    false,
                    render_omissions,
                ),
            };
            if !omission_green || rows_by_fragment.contains_key(&locator.fragment_id) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha certified omission no longer matches current render projection",
                ));
            }
            continue;
        }

        let rows = rows_by_fragment.get(&locator.fragment_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord transaction-local Output-Alpha lost expected canonical output occurrence",
            )
        })?;
        let label = voice_plan
            .label_for_source(locator.source_event_id)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha source lost current sealed voice label",
                )
            })?;
        // A render-occurrence witness is validated against the exact pre-projection pitched
        // sequence during canonical binding. For source occurrences that survive the
        // FinalChord projection, that witness owns physical render onset; the canonical
        // locator continues to own semantic onset. The transaction receipt plus output
        // topology fingerprint proves that the projected occurrence itself did not drift.
        let render_witness_owns_surviving_onset = render_omissions.is_some()
            && canonical.render_occurrence_witnesses.iter().any(|witness| {
                witness.fragment_id == locator.fragment_id
                    && witness.source_event_id == locator.source_event_id
                    && witness.attack_id == locator.attack_id
                    && witness.measure == locator.measure
            });
        match action.map(|action| &action.kind) {
            Some(FinalChordCanonicalActionKind::TiedAnchorChain { fragments }) => {
                if rows.len() != fragments.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha tied readback cardinality drift",
                    ));
                }
                for (ordinal, fragment) in fragments.iter().enumerate() {
                    let witness = rows[ordinal];
                    if witness.materialization_kind
                        != FinalChordProjectionMaterializationKind::TiedAnchorChain
                        || witness.fragment_ordinal != ordinal
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "FinalChord transaction-local Output-Alpha tied readback provenance drift",
                        ));
                    }
                    let note = final_chord_transaction_output_occurrence_note(&measures, witness)?;
                    if note.onset != fragment.target_onset
                        || note.duration != fragment.duration
                        || note.pitch_midi != Some(locator.pitch_midi)
                        || note.staff != locator.staff
                        || note.tie_start != fragment.tie_start
                        || note.tie_stop != fragment.tie_stop
                        || note.voice != label
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "FinalChord transaction-local Output-Alpha tied occurrence disagrees with current representation/voice plan",
                        ));
                    }
                }
            }
            Some(FinalChordCanonicalActionKind::RetainedChordMember {
                canonical_duration,
            }) => {
                if rows.len() != 1
                    || rows[0].materialization_kind
                        != FinalChordProjectionMaterializationKind::SurvivingSourceNote
                    || rows[0].fragment_ordinal != 0
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha retained occurrence provenance drift",
                    ));
                }
                let note = final_chord_transaction_output_occurrence_note(&measures, rows[0])?;
                let representation_duration = final_chord_render_representation_duration(
                    canonical,
                    locator.fragment_id,
                    render_omissions.is_some(),
                )
                .unwrap_or(*canonical_duration);
                if representation_duration <= 0
                    || (!render_witness_owns_surviving_onset && note.onset != locator.onset)
                    || note.duration != representation_duration
                    || note.pitch_midi != Some(locator.pitch_midi)
                    || note.staff != locator.staff
                    || (require_canonical_chord_syntax
                        && note.chord_member != locator.chord_syntax)
                    || note.tie_start != locator.tie_start
                    || note.tie_stop != locator.tie_stop
                    || note.voice != label
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha retained occurrence disagrees with current representation/voice plan",
                    ));
                }
            }
            Some(FinalChordCanonicalActionKind::AuxiliaryLane {
                canonical_duration,
                ..
            }) => {
                if rows.len() != 1
                    || rows[0].materialization_kind
                        != FinalChordProjectionMaterializationKind::AuxiliaryLane
                    || rows[0].fragment_ordinal != 0
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha auxiliary occurrence provenance drift",
                    ));
                }
                let note = final_chord_transaction_output_occurrence_note(&measures, rows[0])?;
                if *canonical_duration <= 0
                    || note.onset != locator.onset
                    || note.duration != *canonical_duration
                    || note.pitch_midi != Some(locator.pitch_midi)
                    || note.staff != locator.staff
                    || note.voice != label
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha auxiliary occurrence disagrees with current representation/voice plan",
                    ));
                }
            }
            _ => {
                if rows.len() != 1
                    || rows[0].materialization_kind
                        != FinalChordProjectionMaterializationKind::SurvivingSourceNote
                    || rows[0].fragment_ordinal != 0
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha surviving occurrence provenance drift",
                    ));
                }
                let note = final_chord_transaction_output_occurrence_note(&measures, rows[0])?;
                let representation_duration = final_chord_render_representation_duration(
                    canonical,
                    locator.fragment_id,
                    render_omissions.is_some(),
                )
                .unwrap_or(locator.duration);
                if (!render_witness_owns_surviving_onset && note.onset != locator.onset)
                    || note.duration != representation_duration
                    || note.pitch_midi != Some(locator.pitch_midi)
                    || note.staff != locator.staff
                    || (require_canonical_chord_syntax
                        && note.chord_member != locator.chord_syntax)
                    || note.tie_start != locator.tie_start
                    || note.tie_stop != locator.tie_stop
                    || note.voice != label
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha surviving occurrence disagrees with current representation/voice plan",
                    ));
                }
            }
        }
    }

    for measure in &measures {
        for (note_index, note) in measure.notes.iter().enumerate() {
            if !note.chord_member || note.rest_like {
                continue;
            }
            let pitched_ordinal_for_index = |target_index: usize| {
                let target = measure.notes.get(target_index)?;
                if target.rest_like || target.pitch_midi.is_none() {
                    return None;
                }
                Some(
                    measure.notes[..=target_index]
                        .iter()
                        .filter(|candidate| {
                            !candidate.rest_like && candidate.pitch_midi.is_some()
                        })
                        .count()
                        .saturating_sub(1),
                )
            };
            let member_pitched_ordinal = pitched_ordinal_for_index(note_index);
            let member_owner = member_pitched_ordinal.and_then(|pitched_ordinal| {
                receipt.occurrences.iter().find(|row| {
                    row.measure == measure.measure && row.pitched_ordinal == pitched_ordinal
                })
            });
            let anchor_index = match note.anchor_index {
                Some(anchor_index) => anchor_index,
                None if final_chord_output_alpha_owned_transient_render_orphan(
                    note,
                    member_owner,
                    &locator_by_fragment,
                    render_omissions.is_some(),
                ) => {
                    // The FinalPdfSource bundle has one later, already-authorized owner for
                    // this exact representation transition: OrphanChordHost replaces the
                    // preceding forward carrier with a receipt-bound hidden rest host and proves the
                    // onset rebase before the artifact sink. Output-Alpha therefore validates
                    // ownership here, but must not demand that post-normalization host early.
                    continue;
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord transaction-local Output-Alpha found unowned chord member without physical host",
                    ));
                }
            };
            let host = measure.notes.get(anchor_index).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha chord host index drift",
                )
            })?;
            if final_chord_physical_host_is_sounding(host) && note.voice != host.voice {
                let host_pitched_ordinal = pitched_ordinal_for_index(anchor_index);
                let host_owner = host_pitched_ordinal.and_then(|pitched_ordinal| {
                    receipt.occurrences.iter().find(|row| {
                        row.measure == measure.measure && row.pitched_ordinal == pitched_ordinal
                    })
                });
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "FinalChord transaction-local Output-Alpha chord voice mismatch: measure={} member_raw_note_index={} member_pitched_ordinal={:?} member_source_event_id={:?} member_attack_id={:?} member_voice={} member_onset={} member_pitch={:?} host_raw_note_index={} host_pitched_ordinal={:?} host_source_event_id={:?} host_attack_id={:?} host_voice={} host_onset={} host_pitch={:?}",
                        measure.measure,
                        note.raw_note_index,
                        member_pitched_ordinal,
                        member_owner.map(|row| row.source_event_id),
                        member_owner.map(|row| row.attack_id),
                        note.voice,
                        note.onset,
                        note.pitch_midi,
                        host.raw_note_index,
                        host_pitched_ordinal,
                        host_owner.map(|row| row.source_event_id),
                        host_owner.map(|row| row.attack_id),
                        host.voice,
                        host.onset,
                        host.pitch_midi,
                    ),
                ));
            }
        }
        for note in &measure.notes {
            if !note.rest_like
                && note.pitch_midi.is_some()
                && voice_plan.role_for_label(&note.voice).is_none()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord transaction-local Output-Alpha found pitched carrier with unsealed current voice label",
                ));
            }
        }
    }
    Ok(())
}

fn final_chord_verify_pitched_sequence_transition(
    before_xml: &str,
    after_xml: &str,
    allow_chord_removal: bool,
) -> io::Result<usize> {
    let before_measures = final_chord_parse_binding_measures(before_xml)?;
    let after_measures = final_chord_parse_binding_measures(after_xml)?;
    if before_measures.len() != after_measures.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord continuation changed measure topology",
        ));
    }

    let mut chord_removals = 0usize;
    for (before_measure, after_measure) in before_measures.iter().zip(after_measures.iter()) {
        if before_measure.measure != after_measure.measure {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord continuation changed measure identity",
            ));
        }
        let before_pitched = before_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();
        let after_pitched = after_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();
        if before_pitched.len() != after_pitched.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FinalChord continuation changed pitched cardinality in measure {}: before={} after={}",
                    before_measure.measure,
                    before_pitched.len(),
                    after_pitched.len(),
                ),
            ));
        }
        for (pitched_ordinal, (before_note, after_note)) in before_pitched
            .iter()
            .zip(after_pitched.iter())
            .enumerate()
        {
            if before_note.onset != after_note.onset
                || before_note.duration != after_note.duration
                || before_note.pitch_midi != after_note.pitch_midi
                || before_note.staff != after_note.staff
                || before_note.voice != after_note.voice
                || before_note.tie_start != after_note.tie_start
                || before_note.tie_stop != after_note.tie_stop
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "FinalChord continuation changed pitched identity/state at m{} ord{}",
                        before_measure.measure, pitched_ordinal,
                    ),
                ));
            }
            if before_note.chord_member != after_note.chord_member {
                if !allow_chord_removal || !before_note.chord_member || after_note.chord_member {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "FinalChord continuation found unauthorized chord-state change at m{} ord{}",
                            before_measure.measure, pitched_ordinal,
                        ),
                    ));
                }
                chord_removals = chord_removals.saturating_add(1);
            }
        }
    }
    Ok(chord_removals)
}

fn final_chord_prove_authorized_cursor_neutral_pitched_identity_transport(
    before_xml: &str,
    after_xml: &str,
    expected_chord_removals: usize,
    materialization_equivalence_green: bool,
) -> io::Result<()> {
    if expected_chord_removals > 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord final-PDF-source continuation received an impossible cursor-neutral chord-normalization count",
        ));
    }
    let write_produced = expected_chord_removals == 1;
    if write_produced && !materialization_equivalence_green {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord final-PDF-source continuation requires green authorized materialization equivalence",
        ));
    }
    if !write_produced {
        if before_xml != after_xml {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord final-PDF-source continuation found representation drift without an authorized writer action",
            ));
        }
        return Ok(());
    }

    let chord_removals =
        final_chord_verify_pitched_sequence_transition(before_xml, after_xml, true)?;
    if chord_removals != expected_chord_removals {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChord final-PDF-source continuation chord-normalization cardinality drift: authorized={} physical={}",
                expected_chord_removals, chord_removals,
            ),
        ));
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub(crate) struct FinalPdfSourceOccurrenceValidationContext<'a> {
    pub(crate) canonical: &'a CanonicalFinalChordStructuralPlan,
    pub(crate) voice_plan: &'a FrozenSerializedVoicePlan,
    pub(crate) render_omissions:
        Option<&'a std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
}

#[derive(Clone, Copy)]
pub(crate) struct OrphanChordHostContinuationEvidence {
    pub(crate) normalization_accepted: bool,
    pub(crate) accepted_chord_notes: usize,
}

pub(crate) fn advance_final_pdf_source_occurrence_after_cursor_commit(
    projection: CanonicalFinalChordRenderOccurrenceContinuation,
    writer_source_xml: &str,
    writer_candidate_xml: &str,
    writer_write_produced: bool,
    materialization_equivalence_green: bool,
    validation: FinalPdfSourceOccurrenceValidationContext<'_>,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    if projection.xml() != writer_source_xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord final-PDF-source continuation writer source is not the exact canonical projected representation",
        ));
    }
    let expected_chord_removals = usize::from(writer_write_produced);
    final_chord_prove_authorized_cursor_neutral_pitched_identity_transport(
        writer_source_xml,
        writer_candidate_xml,
        expected_chord_removals,
        materialization_equivalence_green,
    )?;

    let mut receipt = projection.projected.receipt;
    let after_measures = final_chord_parse_binding_measures(writer_candidate_xml)?;
    receipt.output_topology_fingerprint_fnv64 =
        final_chord_transaction_output_alpha_topology_fingerprint(&after_measures);
    let continuation = FinalPdfSourceOccurrenceContinuation::from_original_transport(writer_candidate_xml.to_string(), receipt)?;
    verify_final_pdf_source_occurrence_continuation(
        continuation.xml(),
        validation.canonical,
        validation.voice_plan,
        validation.render_omissions,
        &continuation,
    )?;
    Ok(continuation)
}

pub(crate) fn verify_final_pdf_source_occurrence_continuation(
    candidate_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
    continuation: &FinalPdfSourceOccurrenceContinuation,
) -> io::Result<()> {
    if !continuation.has_authentic_beam_surface()? || candidate_xml != continuation.xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord final-PDF-source continuation is not bound to these exact representation bytes",
        ));
    }
    if continuation.receipt.original_beam_voice_groups != canonical.original_beam_voice_groups {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "FinalChord original beam origin carrier differs from its canonical epoch"));
    }
    verify_canonical_final_chord_occurrence_receipt(
        candidate_xml,
        canonical,
        voice_plan,
        render_omissions,
        &continuation.receipt,
        None,
        false,
    )
}

fn primary_beam_voice_partition_voice_text_range(
    source_xml: &str,
    note_range: std::ops::Range<usize>,
) -> io::Result<(usize, usize)> {
    let note_text = source_xml.get(note_range.clone()).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition bound note range disappeared",
        )
    })?;
    let open = "<voice>";
    let close = "</voice>";
    let relative_start = note_text.find(open).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition selected note has no explicit voice element",
        )
    })? + open.len();
    let relative_end = relative_start
        + note_text[relative_start..].find(close).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition selected note has unterminated voice element",
            )
        })?;
    Ok((
        note_range.start.saturating_add(relative_start),
        note_range.start.saturating_add(relative_end),
    ))
}

fn primary_beam_voice_partition_note_primary_state(
    source_xml: &str,
    note: &FinalChordEncodingNote,
) -> Option<String> {
    let text = source_xml.get(note.range.clone())?;
    let mut scan = 0usize;
    while let Some(relative) = text.get(scan..)?.find("<beam") {
        let start = scan.saturating_add(relative);
        let open_end = start.saturating_add(text.get(start..)?.find('>')?);
        let open_tag = text.get(start..=open_end)?;
        let primary = !open_tag.contains("number=")
            || open_tag.contains("number=\"1\"")
            || open_tag.contains("number='1'");
        let content_start = open_end.saturating_add(1);
        let close_relative = text.get(content_start..)?.find("</beam>")?;
        let content_end = content_start.saturating_add(close_relative);
        if primary {
            let state = text.get(content_start..content_end)?.trim();
            if !state.is_empty() {
                return Some(state.to_string());
            }
        }
        scan = content_end.saturating_add("</beam>".len());
    }
    None
}

pub(crate) fn bind_primary_beam_voice_partition_plan(
    source_xml: &str,
    context: PrimaryBeamVoicePartitionBindingContext<'_>,
) -> io::Result<PrimaryBeamVoicePartitionBoundPlan> {
    bind_primary_beam_voice_partition_plan_for_purpose(source_xml, context, false)
}

pub(crate) fn bind_historical_metric_phrase_decision_course(
    source_xml: &str, context: PrimaryBeamVoicePartitionBindingContext<'_>,
    geometry: &crate::audit_only::renderer_system_break_readback::HistoricalPhrasePolicyDecisionGeometry,
) -> io::Result<PrimaryBeamVoicePartitionBoundPlan> {
    let candidate_ids=context.candidates.iter().map(|c|c.metric_line_id.as_str()).collect::<std::collections::BTreeSet<_>>();
    let mut observations=std::collections::BTreeMap::new();
    for cohort in primary_beam_voice_partition_renderer_cohorts(context.source_continuation,context.candidates) {
        if !candidate_ids.contains(cohort.metric_line_id()) { continue; }
        if let Some(count)=geometry.metric_decision_fragment_count(cohort.measure(),cohort.staff(),cohort.pitched_ordinals()) {
            observations.insert(cohort.metric_line_id().to_owned(),count);
        }
    }
    // Historical observations cannot reach generic original-group alignment.
    // Its private purpose bit survives rebind and blocks PDF finalization.
    let context = PrimaryBeamVoicePartitionBindingContext {
        renderer_visible_fragments_by_metric_line_id: &observations, ..context
    };
    bind_primary_beam_voice_partition_plan_for_purpose(source_xml,context,true)
}

fn bind_primary_beam_voice_partition_plan_for_purpose(
    source_xml: &str, context: PrimaryBeamVoicePartitionBindingContext<'_>,
    historical_phrase_decision_only: bool,
) -> io::Result<PrimaryBeamVoicePartitionBoundPlan> {
    let PrimaryBeamVoicePartitionBindingContext {
        source_continuation,
        voice_plan,
        candidates,
        classification_xml,
        renderer_visible_fragments_by_metric_line_id,
        materialization_safety_proven_metric_line_ids,
    } = context;
    if source_continuation.xml() != source_xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition binding received stale occurrence continuation",
        ));
    }
    if !voice_plan.is_bound() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition binding requires a bound serialized voice plan",
        ));
    }

    let mut ordered_candidates = candidates.iter().collect::<Vec<_>>();
    ordered_candidates.sort_by(|left, right| left.metric_line_id.cmp(&right.metric_line_id));
    if ordered_candidates
        .windows(2)
        .any(|pair| pair[0].metric_line_id == pair[1].metric_line_id)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition binding received duplicate metric-line identity",
        ));
    }
    for candidate in &ordered_candidates {
        if candidate.metric_line_id.trim().is_empty()
            || candidate.members.is_empty()
            || !matches!(candidate.staff, 1 | 2)
            || candidate.canonical_primary_states.len() != candidate.members.len()
            || !candidate.members.iter().any(|&(source_event_id, attack_id)| {
                source_event_id == candidate.target_primary_source_event_id
                    && attack_id == candidate.target_primary_attack_id
            })
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition binding received malformed metric-line candidate",
            ));
        }
    }

    let source_measures = final_chord_parse_binding_measures(source_xml)?;
    let classification_measures = final_chord_parse_binding_measures(classification_xml)?;
    let mut repaired_voice_plan = voice_plan.clone();
    let mut claimed_identities = std::collections::BTreeSet::<(usize, usize)>::new();
    let mut claimed_footprints = std::collections::BTreeSet::<(usize, usize)>::new();
    let mut edits = Vec::<PrimaryBeamVoicePartitionBoundEdit>::new();
    let mut selected_members = Vec::<PrimaryBeamVoicePartitionSelectedMember>::new();
    let mut scope_candidates = 0usize;

    for candidate in ordered_candidates {
        if !materialization_safety_proven_metric_line_ids.contains(&candidate.metric_line_id) {
            continue;
        }
        let Some(visible_fragments) = renderer_visible_fragments_by_metric_line_id
            .get(&candidate.metric_line_id)
            .copied()
        else {
            continue;
        };
        if visible_fragments <= 1 {
            continue;
        }

        let unique_identities = candidate
            .members
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let unique_sources = candidate
            .members
            .iter()
            .map(|&(source_event_id, _)| source_event_id)
            .collect::<std::collections::BTreeSet<_>>();
        if unique_identities.len() != candidate.members.len()
            || unique_sources.len() != candidate.members.len()
            || unique_sources.len() < 2
        {
            continue;
        }

        let classification_measure = classification_measures
            .iter()
            .find(|measure| measure.measure == candidate.measure);
        let Some(classification_measure) = classification_measure else {
            continue;
        };
        let classification_pitched = classification_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();

        let expected_primary = match candidate.staff {
            1 => SerializedVoicePresentationRole::UpperPrimary,
            2 => SerializedVoicePresentationRole::LowerPrimary,
            _ => unreachable!(),
        };
        let expected_secondary = match candidate.staff {
            1 => SerializedVoicePresentationRole::UpperSecondary,
            2 => SerializedVoicePresentationRole::LowerSecondary,
            _ => unreachable!(),
        };

        let mut observed_primary_states = Vec::<String>::new();
        let mut serialized_voices = std::collections::BTreeSet::<String>::new();
        let mut primary_channels = std::collections::BTreeSet::<usize>::new();
        let mut primary_members = 0usize;
        let mut secondary_members = 0usize;
        let mut current_binding_complete = true;

        for &(source_event_id, attack_id) in &candidate.members {
            let witnesses = source_continuation
                .receipt
                .occurrences
                .iter()
                .filter(|row| {
                    row.source_event_id == source_event_id && row.attack_id == attack_id
                })
                .collect::<Vec<_>>();
            if witnesses.len() != 1 || witnesses[0].measure != candidate.measure {
                current_binding_complete = false;
                break;
            }
            let witness = witnesses[0];
            let Some(note) = classification_pitched.get(witness.pitched_ordinal).copied() else {
                current_binding_complete = false;
                break;
            };
            if note.staff.trim().parse::<u8>().ok() != Some(candidate.staff) {
                current_binding_complete = false;
                break;
            }
            let Some(primary_state) =
                primary_beam_voice_partition_note_primary_state(classification_xml, note)
            else {
                current_binding_complete = false;
                break;
            };
            observed_primary_states.push(primary_state);
            serialized_voices.insert(note.voice.trim().to_string());

            let Some(label) = voice_plan.label_for_source(source_event_id) else {
                current_binding_complete = false;
                break;
            };
            let Some(role) = voice_plan.role_for_source(source_event_id) else {
                current_binding_complete = false;
                break;
            };
            if role == expected_primary {
                primary_members = primary_members.saturating_add(1);
                let Some(channel) = voice_plan.channel_for_label(label) else {
                    current_binding_complete = false;
                    break;
                };
                primary_channels.insert(channel);
            } else if role == expected_secondary {
                secondary_members = secondary_members.saturating_add(1);
            } else {
                current_binding_complete = false;
                break;
            }
        }

        if !current_binding_complete
            || observed_primary_states != candidate.canonical_primary_states
            || serialized_voices.len() <= 1
            || primary_members == 0
            || secondary_members == 0
            || primary_channels.len() != 1
            || !primary_channels.contains(&candidate.target_channel)
        {
            continue;
        }

        let target_role = repaired_voice_plan
            .channel_to_role
            .get(&candidate.target_channel)
            .copied();
        if target_role != Some(expected_primary) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition target channel role drifted from accepted primary relation",
            ));
        }
        let target_label = repaired_voice_plan
            .channel_to_label
            .get(&candidate.target_channel)
            .cloned()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition target channel has no sealed label",
                )
            })?;
        if voice_plan
            .label_for_source(candidate.target_primary_source_event_id)
            != Some(target_label.as_str())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition typed primary target does not carry the sealed target label",
            ));
        }

        for &(source_event_id, attack_id) in &candidate.members {
            if !claimed_identities.insert((source_event_id, attack_id)) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition independently eligible repair candidates overlap in source/attack membership",
                ));
            }
            let witnesses = source_continuation
                .receipt
                .occurrences
                .iter()
                .filter(|row| {
                    row.source_event_id == source_event_id && row.attack_id == attack_id
                })
                .collect::<Vec<_>>();
            if witnesses.len() != 1 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition selected source/attack lost exact current physical occurrence",
                ));
            }
            let witness = witnesses[0];
            if !claimed_footprints.insert((witness.measure, witness.pitched_ordinal)) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition independently eligible repair candidates overlap in physical edit footprint",
                ));
            }
            let measure = source_measures
                .iter()
                .find(|measure| measure.measure == witness.measure)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "PrimaryBeamVoicePartition occurrence measure is absent from current Render XML",
                    )
                })?;
            let note = measure
                .notes
                .iter()
                .filter(|note| !note.rest_like && note.pitch_midi.is_some())
                .nth(witness.pitched_ordinal)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "PrimaryBeamVoicePartition occurrence pitched ordinal is absent from current Render XML",
                    )
                })?;
            let old_label = note.voice.trim().to_string();
            let sealed_old_label = voice_plan.label_for_source(source_event_id).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition selected source lost original sealed label",
                )
            })?;
            if old_label.as_str() != sealed_old_label {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition current physical occurrence disagrees with the sealed pre-FinalNotation voice label",
                ));
            }

            selected_members.push(PrimaryBeamVoicePartitionSelectedMember {
                source_event_id,
                attack_id,
                fragment_id: witness.fragment_id,
                target_primary_fragment_id: source_continuation.receipt.occurrences.iter()
                    .find(|row| row.source_event_id == candidate.target_primary_source_event_id
                        && row.attack_id == candidate.target_primary_attack_id)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "PrimaryBeam target occurrence lost"))?.fragment_id,
                target_primary_source_event_id: candidate.target_primary_source_event_id,
                target_primary_attack_id: candidate.target_primary_attack_id,
                target_channel: candidate.target_channel,
                target_label: target_label.clone(),
            });

            if old_label != target_label {
                let (voice_text_start, voice_text_end) =
                    primary_beam_voice_partition_voice_text_range(source_xml, note.range.clone())?;
                if source_xml
                    .get(voice_text_start..voice_text_end)
                    .map(str::trim)
                    != Some(old_label.as_str())
                {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "PrimaryBeamVoicePartition bound voice range disagrees with the exact current label",
                    ));
                }
                edits.push(PrimaryBeamVoicePartitionBoundEdit {
                    measure: witness.measure,
                    pitched_ordinal: witness.pitched_ordinal,
                    voice_text_start,
                    voice_text_end,
                    expected_old_label: old_label,
                    target_label: target_label.clone(),
                });
            }
            repaired_voice_plan
                .source_to_channel
                .insert(source_event_id, candidate.target_channel);
            repaired_voice_plan
                .source_to_label
                .insert(source_event_id, target_label.clone());
        }
        scope_candidates = scope_candidates.saturating_add(1);
    }

    if historical_phrase_decision_only {
        return Ok(PrimaryBeamVoicePartitionBoundPlan {
            source_fingerprint_fnv64: fingerprint(source_xml), repaired_voice_plan,
            edits, selected_members, scope_candidates, original_group_alignment: Vec::new(),
            historical_phrase_decision_only: true,
        });
    }
    let added = original_beam_voice_alignment::append_proven_alignment(
        original_beam_voice_alignment::AlignmentInput {
            source_xml, source: source_continuation, voices: voice_plan, classification_xml,
            visible_fragments: renderer_visible_fragments_by_metric_line_id,
        },
        original_beam_voice_alignment::AlignmentOutput {
            repaired: &mut repaired_voice_plan, edits: &mut edits, selected: &mut selected_members,
        },
    )?;
    scope_candidates = scope_candidates.saturating_add(added.len());
    eprintln!("OriginalBeamVoiceAlignment: origin_components={}; prepared_components={}; authority=original_owner_and_pre_lane_relation; phase=prepared_not_committed; report_reentry=none",
        source_continuation.receipt.original_beam_voice_groups.len(), added.len());
    if !added.is_empty() {
        original_beam_voice_alignment::validate_batch_occupancy(source_xml, &edits)?;
    }
    edits.sort_by_key(|edit| (edit.voice_text_start, edit.voice_text_end));
    if edits
        .windows(2)
        .any(|pair| pair[0].voice_text_end > pair[1].voice_text_start)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition bound physical voice edits overlap",
        ));
    }

    Ok(PrimaryBeamVoicePartitionBoundPlan {
        source_fingerprint_fnv64: fingerprint(source_xml),
        repaired_voice_plan,
        edits,
        selected_members,
        scope_candidates,
        original_group_alignment: added,
        historical_phrase_decision_only: false,
    })
}

pub(crate) fn transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
    source: &FinalPdfSourceOccurrenceContinuation,
    observed_source_xml: &str,
    target_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    plan: &PrimaryBeamVoicePartitionBoundPlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    if source.xml() != observed_source_xml || !plan.matches_source_xml(observed_source_xml) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition continuation source binding is stale",
        ));
    }

    let before_measures = final_chord_parse_binding_measures(observed_source_xml)?;
    let after_measures = final_chord_parse_binding_measures(target_xml)?;
    if before_measures.len() != after_measures.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition repair changed measure topology",
        ));
    }

    let allowed = plan
        .edits
        .iter()
        .map(|edit| {
            (
                (edit.measure, edit.pitched_ordinal),
                (edit.expected_old_label.as_str(), edit.target_label.as_str()),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut changed = 0usize;

    for (before_measure, after_measure) in before_measures.iter().zip(after_measures.iter()) {
        if before_measure.measure != after_measure.measure {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition repair changed measure identity",
            ));
        }
        let before_pitched = before_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();
        let after_pitched = after_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();
        if before_pitched.len() != after_pitched.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PrimaryBeamVoicePartition repair changed pitched occurrence cardinality",
            ));
        }
        for (pitched_ordinal, (before_note, after_note)) in
            before_pitched.iter().zip(after_pitched.iter()).enumerate()
        {
            if before_note.onset != after_note.onset
                || before_note.duration != after_note.duration
                || before_note.pitch_midi != after_note.pitch_midi
                || before_note.staff != after_note.staff
                || before_note.chord_member != after_note.chord_member
                || before_note.tie_start != after_note.tie_start
                || before_note.tie_stop != after_note.tie_stop
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition repair changed non-voice pitched state",
                ));
            }
            if before_note.voice == after_note.voice {
                continue;
            }
            let Some((expected_old, target)) =
                allowed.get(&(before_measure.measure, pitched_ordinal)).copied()
            else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition changed an unselected occurrence",
                ));
            };
            if before_note.voice.trim() != expected_old || after_note.voice.trim() != target {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "PrimaryBeamVoicePartition changed an occurrence outside its bound voice delta",
                ));
            }
            changed = changed.saturating_add(1);
        }
    }
    if changed != plan.edits.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "PrimaryBeamVoicePartition persisted voice-delta cardinality disagrees with bound plan",
        ));
    }

    let continuation = FinalPdfSourceOccurrenceContinuation::from_original_transport(target_xml.to_string(), source.receipt.clone())?;
    verify_final_pdf_source_occurrence_continuation(
        target_xml,
        canonical,
        &plan.repaired_voice_plan,
        render_omissions,
        &continuation,
    )?;
    Ok(continuation)
}

fn transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write_without_readback(
    source: &FinalPdfSourceOccurrenceContinuation,
    observed_source_xml: &str,
    target_xml: &str,
    boundary: &str,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    if !source.has_authentic_beam_surface()? || source.xml() != observed_source_xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChord final-PDF-source continuation is stale before {boundary}"
            ),
        ));
    }
    final_chord_verify_pitched_sequence_transition(
        observed_source_xml,
        target_xml,
        false,
    )
    .map_err(|error| {
        io::Error::new(
            error.kind(),
            format!(
                "FinalChord final-PDF-source continuation cannot cross {boundary}: {error}"
            ),
        )
    })?;
    FinalPdfSourceOccurrenceContinuation::from_original_transport(target_xml.to_string(), source.receipt.clone())
}

fn transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write(
    source: &FinalPdfSourceOccurrenceContinuation,
    observed_source_xml: &str,
    target_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
    boundary: &str,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    let continuation =
        transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write_without_readback(
            source,
            observed_source_xml,
            target_xml,
            boundary,
        )?;
    verify_final_pdf_source_occurrence_continuation(
        continuation.xml(),
        canonical,
        voice_plan,
        render_omissions,
        &continuation,
    )?;
    Ok(continuation)
}

fn final_chord_prove_orphan_chord_host_onset_rebase(
    before_xml: &str,
    after_xml: &str,
    accepted_chord_notes: usize,
) -> io::Result<Vec<FinalChordBindingMeasure>> {
    let before_measures = final_chord_parse_binding_measures(before_xml)?;
    let after_measures = final_chord_parse_binding_measures(after_xml)?;
    if before_measures.len() != after_measures.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord orphan-chord host continuation changed measure topology",
        ));
    }

    let mut rebased_onsets = 0usize;
    for (before_measure, after_measure) in before_measures.iter().zip(after_measures.iter()) {
        if before_measure.measure != after_measure.measure {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord orphan-chord host continuation changed measure identity",
            ));
        }
        let before_pitched = before_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();
        let after_pitched = after_measure
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();
        if before_pitched.len() != after_pitched.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "FinalChord orphan-chord host continuation changed pitched cardinality in measure {}",
                    before_measure.measure,
                ),
            ));
        }

        for (pitched_ordinal, (before_note, after_note)) in before_pitched
            .iter()
            .zip(after_pitched.iter())
            .enumerate()
        {
            let promoted_orphan_anchor = before_note.onset == after_note.onset
                && before_note.chord_member
                && !after_note.chord_member
                && after_note.anchor_index.is_none()
                && before_note.anchor_index.is_some_and(|anchor_index| {
                    before_measure.notes.get(anchor_index).is_some_and(|host| {
                        host.rest_like
                            && !host.chord_member
                            && host.duration == before_note.duration
                            && host.onset == before_note.onset
                    })
                });
            if before_note.duration != after_note.duration
                || before_note.pitch_midi != after_note.pitch_midi
                || before_note.staff != after_note.staff
                || before_note.voice != after_note.voice
                || before_note.tie_start != after_note.tie_start
                || before_note.tie_stop != after_note.tie_stop
                || (before_note.chord_member != after_note.chord_member
                    && !promoted_orphan_anchor)
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "FinalChord orphan-chord host continuation changed pitched payload/state at m{} ord{}",
                        before_measure.measure, pitched_ordinal,
                    ),
                ));
            }
            if promoted_orphan_anchor {
                rebased_onsets = rebased_onsets.saturating_add(1);
                continue;
            }
            if before_note.onset == after_note.onset {
                continue;
            }
            if !before_note.chord_member
                || !before_note.orphan_after_forward
                || !after_note.chord_member
                || after_note.orphan_after_forward
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "FinalChord orphan-chord host continuation found an unauthorized onset rebase at m{} ord{}",
                        before_measure.measure, pitched_ordinal,
                    ),
                ));
            }
            let Some(anchor_index) = after_note.anchor_index else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord orphan-chord host continuation lost the materialized rest host",
                ));
            };
            let Some(host) = after_measure.notes.get(anchor_index) else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord orphan-chord host continuation rest-host index escaped the measure",
                ));
            };
            if !host.rest_like
                || host.chord_member
                || host.duration <= 0
                || host.onset != after_note.onset
                || before_note.onset != after_note.onset.saturating_add(host.duration)
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "FinalChord orphan-chord host continuation cannot prove the exact forward-to-rest onset rebase at m{} ord{}",
                        before_measure.measure, pitched_ordinal,
                    ),
                ));
            }
            rebased_onsets = rebased_onsets.saturating_add(1);
        }
    }
    if rebased_onsets != accepted_chord_notes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "FinalChord orphan-chord host continuation cardinality drift: audit={} physical={}",
                accepted_chord_notes, rebased_onsets,
            ),
        ));
    }
    Ok(after_measures)
}

fn transport_final_pdf_source_occurrence_continuation_through_orphan_chord_host_normalization(
    source: &FinalPdfSourceOccurrenceContinuation,
    observed_source_xml: &str,
    target_xml: &str,
    evidence: OrphanChordHostContinuationEvidence,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    if source.xml() != observed_source_xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord final-PDF-source continuation is stale before authorized orphan-chord host normalization",
        ));
    }
    if !evidence.normalization_accepted {
        if evidence.accepted_chord_notes != 0 || observed_source_xml != target_xml {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord orphan-chord host continuation found representation drift without an accepted normalization",
            ));
        }
        return Ok(source.clone());
    }
    if evidence.accepted_chord_notes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord orphan-chord host continuation accepted a normalization without chord members",
        ));
    }

    let after_measures = final_chord_prove_orphan_chord_host_onset_rebase(
        observed_source_xml,
        target_xml,
        evidence.accepted_chord_notes,
    )?;
    let mut receipt = source.receipt.clone();
    receipt.output_topology_fingerprint_fnv64 =
        final_chord_transaction_output_alpha_topology_fingerprint(&after_measures);
    FinalPdfSourceOccurrenceContinuation::from_original_transport(target_xml.to_string(), receipt)
}

pub(crate) fn transport_final_pdf_source_occurrence_continuation_through_final_notation_bundle(
    source: &FinalPdfSourceOccurrenceContinuation,
    observed_source_xml: &str,
    silent_voice_xml: &str,
    orphan_chord_host_xml: &str,
    orphan_host_evidence: OrphanChordHostContinuationEvidence,
    target_xml: &str,
    validation: FinalPdfSourceOccurrenceValidationContext<'_>,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    // SilentVoice is identity-preserving for pitched occurrences, but the following
    // already-authorized OrphanChordHost normalizer may still be required to make a
    // transient forward-host representation canonically readable. Carry identity
    // across this internal boundary with its exact local delta proof. Full state
    // validation is coalesced at the bundle end before the side-effecting write.
    let after_silent_voice =
        transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write_without_readback(
            source,
            observed_source_xml,
            silent_voice_xml,
            "authorized silent-voice materialization",
        )?;
    let after_orphan_host =
        transport_final_pdf_source_occurrence_continuation_through_orphan_chord_host_normalization(
            &after_silent_voice,
            silent_voice_xml,
            orphan_chord_host_xml,
            orphan_host_evidence,
        )?;
    transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write(
        &after_orphan_host,
        orphan_chord_host_xml,
        target_xml,
        validation.canonical,
        validation.voice_plan,
        validation.render_omissions,
        "authorized post-orphan final-notation materialization",
    )
}

/// Carry the unchanged pitched image across the existing final clef/rest owners.
/// Each owner supplies its existing receipt; this proves source identity
/// and validates against the already rebound final serialized voice plan.
impl FinalPdfSourceOccurrenceContinuation {
pub(crate) fn through_final_clef_and_rest_placement(
    &self,
    observed_source_xml: &str,
    target_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write(
        self, observed_source_xml, target_xml, canonical, voice_plan,
        render_omissions, "authorized final clef-course and rest-glyph placement",
    )
}
}

#[cfg(test)]
impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn original_beam_alignment_test_fixture(input: &str) -> (
        Self, FrozenSerializedVoicePlan, FinalPdfSourceOccurrenceContinuation,
        PrimaryBeamVoicePartitionBoundPlan,
    ) {
        original_beam_voice_alignment::original_beam_alignment_test_fixture(input)
    }
}

pub(crate) fn transport_final_pdf_source_occurrence_continuation_through_horizontal_layout(
    source: &FinalPdfSourceOccurrenceContinuation,
    observed_source_xml: &str,
    target_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write(
        source,
        observed_source_xml,
        target_xml,
        canonical,
        voice_plan,
        render_omissions,
        "authorized horizontal-layout materialization",
    )
}

pub(crate) fn transport_final_pdf_source_occurrence_continuation_through_committed_cross_staff_beam_projection(
    source: &FinalPdfSourceOccurrenceContinuation,
    observed_source_xml: &str,
    target_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    transport_final_pdf_source_occurrence_continuation_through_pitched_topology_neutral_write(
        source,
        observed_source_xml,
        target_xml,
        canonical,
        voice_plan,
        render_omissions,
        "authorized committed cross-staff beam projection",
    )
}

pub(crate) fn bind_final_pdf_source_occurrence_continuation_to_exact_persisted_readback(
    source: &FinalPdfSourceOccurrenceContinuation,
    persisted_xml: &str,
    canonical: &CanonicalFinalChordStructuralPlan,
    voice_plan: &FrozenSerializedVoicePlan,
    render_omissions: Option<&std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>>,
) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
    if source.xml() != persisted_xml {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord final-PDF-source persisted readback is not byte-exact to the authorized candidate",
        ));
    }
    verify_final_pdf_source_occurrence_continuation(
        persisted_xml,
        canonical,
        voice_plan,
        render_omissions,
        source,
    )?;
    Ok(source.clone())
}


fn final_chord_note_fragment_is_visible(fragment: &str) -> bool {
    !fragment.contains("print-object=\"no\"") && !fragment.contains("print-object='no'")
}

fn final_chord_staff_voice_forward_fragment(
    duration: i64,
    staff: &str,
    voice: &str,
) -> io::Result<String> {
    if duration <= 0 || staff.trim().is_empty() || voice.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord visible-rest interval projection requires positive duration and bound staff/voice",
        ));
    }
    Ok(format!(
        "\n      <forward><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></forward>"
    ))
}

fn final_chord_visible_rest_fragment_from_source(
    source: &str,
    duration: i64,
    divisions: i64,
) -> io::Result<String> {
    if final_chord_exact_type_and_dots(duration, divisions).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord visible-rest interval projection found an unrepresentable rest duration",
        ));
    }
    let retimed = final_chord_set_note_duration(source, duration).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "FinalChord visible-rest interval projection could not retime source rest",
        )
    })?;
    let retimed = retimed
        .replace(" measure=\"yes\"", "")
        .replace(" measure='yes'", "");
    Ok(final_chord_sync_note_type_and_dots(
        &retimed,
        duration,
        divisions,
    ))
}

fn final_chord_plan_visible_rest_interval_edits(
    projected_xml: &str,
) -> io::Result<Vec<FinalChordEncodingEdit>> {
    let measures = final_chord_parse_binding_measures(projected_xml)?;
    let mut edits = Vec::<FinalChordEncodingEdit>::new();

    for measure in &measures {
        let beat_duration = measure.divisions;
        'rest_candidate: for (rest_index, rest) in measure
            .notes
            .iter()
            .enumerate()
            .filter(|(_, note)| {
                note.rest_like
                    && !note.chord_member
                    && note.duration > 0
                    && !note.staff.trim().is_empty()
                    && !note.voice.trim().is_empty()
            })
        {
            // A non-chord note, including a rest, can be the physical MusicXML anchor for
            // following <chord/> members. Replacing such an anchor with a sequence that
            // contains <forward> would reset the parser's anchor state and could move a
            // pitched occurrence even when total cursor duration is conserved. This
            // repair scope therefore stays fail-closed for rest objects that own any
            // currently parsed chord member.
            if measure
                .notes
                .iter()
                .any(|note| note.chord_member && note.anchor_index == Some(rest_index))
            {
                continue;
            }

            let source = projected_xml.get(rest.range.clone()).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord visible-rest interval projection lost source rest range",
                )
            })?;
            if !final_chord_note_fragment_is_visible(source)
                || source.contains("measure=\"yes\"")
                || source.contains("measure='yes'")
            {
                continue;
            }

            let rest_end = rest.onset.checked_add(rest.duration).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord visible-rest interval projection overflowed rest interval",
                )
            })?;

            let ambiguous_rest_overlap = measure.notes.iter().any(|note| {
                if (note.range.start == rest.range.start && note.range.end == rest.range.end)
                    || !note.rest_like
                    || note.chord_member
                    || note.duration <= 0
                    || note.staff != rest.staff
                    || note.voice != rest.voice
                {
                    return false;
                }
                let Some(fragment) = projected_xml.get(note.range.clone()) else {
                    return true;
                };
                final_chord_note_fragment_is_visible(fragment)
                    && rest.onset < note.onset.saturating_add(note.duration)
                    && note.onset < rest_end
            });
            if ambiguous_rest_overlap {
                continue;
            }

            let mut occupied = Vec::<(i64, i64)>::new();
            for note in &measure.notes {
                if note.rest_like
                    || note.duration <= 0
                    || note.pitch_midi.is_none()
                    || note.staff != rest.staff
                    || note.voice != rest.voice
                    || note.onset >= rest_end
                    || note.onset.saturating_add(note.duration) <= rest.onset
                {
                    continue;
                }
                let Some(fragment) = projected_xml.get(note.range.clone()) else {
                    continue;
                };
                if !final_chord_note_fragment_is_visible(fragment) {
                    continue;
                }
                let start = note.onset.max(rest.onset);
                let end = note.onset.saturating_add(note.duration).min(rest_end);
                if end > start {
                    occupied.push((start, end));
                }
            }
            if occupied.is_empty() {
                continue;
            }

            occupied.sort_unstable();
            let mut merged = Vec::<(i64, i64)>::new();
            for (start, end) in occupied {
                if let Some(last) = merged.last_mut()
                    && start <= last.1
                {
                    last.1 = last.1.max(end);
                    continue;
                }
                merged.push((start, end));
            }

            let mut cursor = rest.onset;
            let mut replacement_parts = Vec::<String>::new();
            let mut replacement_duration = 0_i64;
            for (occupied_start, occupied_end) in merged {
                if cursor < occupied_start {
                    let silent_duration = occupied_start.saturating_sub(cursor);
                    let parts = crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                        cursor,
                        silent_duration,
                        measure.expected_duration,
                        beat_duration,
                    );
                    if parts.is_empty() || parts.iter().sum::<i64>() != silent_duration {
                        continue 'rest_candidate;
                    }
                    for part in parts {
                        replacement_parts.push(final_chord_visible_rest_fragment_from_source(
                            source,
                            part,
                            measure.divisions,
                        )?);
                        replacement_duration = replacement_duration.saturating_add(part);
                        cursor = cursor.saturating_add(part);
                    }
                }
                if cursor < occupied_end {
                    let forward_duration = occupied_end.saturating_sub(cursor);
                    replacement_parts.push(final_chord_staff_voice_forward_fragment(
                        forward_duration,
                        &rest.staff,
                        &rest.voice,
                    )?);
                    replacement_duration = replacement_duration.saturating_add(forward_duration);
                    cursor = occupied_end;
                }
            }
            if cursor < rest_end {
                let silent_duration = rest_end.saturating_sub(cursor);
                let parts = crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                    cursor,
                    silent_duration,
                    measure.expected_duration,
                    beat_duration,
                );
                if parts.is_empty() || parts.iter().sum::<i64>() != silent_duration {
                    continue 'rest_candidate;
                }
                for part in parts {
                    replacement_parts.push(final_chord_visible_rest_fragment_from_source(
                        source,
                        part,
                        measure.divisions,
                    )?);
                    replacement_duration = replacement_duration.saturating_add(part);
                    cursor = cursor.saturating_add(part);
                }
            }
            if cursor != rest_end || replacement_duration != rest.duration {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord visible-rest interval projection violated interval conservation",
                ));
            }

            edits.push(FinalChordEncodingEdit {
                start: rest.range.start,
                end: rest.range.end,
                replacement: replacement_parts.join("\n"),
                generated_fragments: Vec::new(),
            });
        }
    }

    edits.sort_by(|left, right| {
        right
            .start
            .cmp(&left.start)
            .then_with(|| right.end.cmp(&left.end))
    });
    for pair in edits.windows(2) {
        if pair[1].end > pair[0].start {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "FinalChord visible-rest interval projection produced overlapping edits",
            ));
        }
    }
    Ok(edits)
}

type FinalChordPitchedTimelineSignatureRow =
    (usize, i64, i64, String, String, String, bool, bool, bool);

fn final_chord_pitched_timeline_signature(
    xml: &str,
) -> io::Result<Vec<FinalChordPitchedTimelineSignatureRow>> {
    let measures = final_chord_parse_binding_measures(xml)?;
    let mut rows = Vec::new();
    for measure in measures {
        for note in measure.notes {
            if note.rest_like || note.pitch_midi.is_none() {
                continue;
            }
            rows.push((
                note.measure,
                note.onset,
                note.duration,
                note.pitch_identity,
                note.staff,
                note.voice,
                note.chord_member,
                note.tie_start,
                note.tie_stop,
            ));
        }
    }
    Ok(rows)
}

/// Sealed EXACT mint capability of population owner #65.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FinalChordEncodingExactMint {
    _sealed: (),
}

pub(crate) fn project_final_chord_encoding_plan(
    candidate_xml: &str,
    plan: &FinalChordEncodingPlan,
) -> (io::Result<String>, Option<ExactTransformationReceipt>) {
    // Two-phase owner: phase 1 applies the bound plan edits to the pre bytes,
    // phase 2 the visible-rest interval edits bound against the phase-1 state.
    // Both scripts are known at origin and composed into one pre-coordinate
    // EXACT script; no post-state diff is taken.
    let mut phase_one_script = Vec::<(usize, usize, String)>::new();
    let mut phase_two_script = Vec::<(usize, usize, String)>::new();
    let mut original_rest_edits = Vec::new();
    let projected = (|| -> io::Result<String> {
        if candidate_xml.len() != plan.source_len || fingerprint(candidate_xml) != plan.source_fingerprint {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "final chord encoding plan is not bound to these source bytes",
            ));
        }

        let mut out = candidate_xml.to_string();
        let mut last_start = usize::MAX;
        for edit in &plan.edits {
            if edit.start > edit.end || edit.end > out.len() || edit.start > last_start {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "final chord encoding edit ranges overlap or drift",
                ));
            }
            let mut rebuilt = String::with_capacity(
                out.len().saturating_sub(edit.end - edit.start) + edit.replacement.len(),
            );
            rebuilt.push_str(&out[..edit.start]);
            rebuilt.push_str(&edit.replacement);
            rebuilt.push_str(&out[edit.end..]);
            out = rebuilt;
            phase_one_script.push((edit.start, edit.end, edit.replacement.clone()));
            last_start = edit.start;
        }
        validate_final_chord_musicxml(&out)?;

        if plan.visible_rest_interval_projection_authorized {
            let pitched_before = final_chord_pitched_timeline_signature(&out)?;
            let rest_edits = final_chord_plan_visible_rest_interval_edits(&out)?;
            original_rest_edits = rest_edits.clone();
            let mut rest_last_start = usize::MAX;
            for edit in rest_edits {
                if edit.start > edit.end || edit.end > out.len() || edit.start > rest_last_start {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "FinalChord visible-rest interval projection edit ranges overlap or drift",
                    ));
                }
                let mut rebuilt = String::with_capacity(
                    out.len().saturating_sub(edit.end - edit.start) + edit.replacement.len(),
                );
                rebuilt.push_str(&out[..edit.start]);
                rebuilt.push_str(&edit.replacement);
                rebuilt.push_str(&out[edit.end..]);
                out = rebuilt;
                rest_last_start = edit.start;
                phase_two_script.push((edit.start, edit.end, edit.replacement));
            }
            validate_final_chord_musicxml(&out)?;
            if pitched_before != final_chord_pitched_timeline_signature(&out)? {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "FinalChord visible-rest interval projection changed pitched timeline",
                ));
            }
        }

        Ok(out)
    })();
    let mut receipt = projected.as_ref().ok().and_then(|post| {
        let composed = crate::master_state_identity::compose_exact_edit_scripts(
            candidate_xml,
            crate::master_state_identity::exact_script_from_descending_application(
                phase_one_script,
            ),
            crate::master_state_identity::exact_script_from_descending_application(
                phase_two_script,
            ),
        )?;
        ExactTransformationReceipt::mint_exact_receipt(
            FinalChordEncodingExactMint { _sealed: () },
            candidate_xml,
            post,
            composed,
        )
    });
    if let (Some(receipt), Ok(post)) = (receipt.as_mut(), projected.as_ref())
        && plan.original_course_bindings.is_some() {
            let capture = course_capture::capture_original_execution(
                &FinalChordEncodingExactMint { _sealed: () }, receipt,
                candidate_xml, post, plan, &original_rest_edits,
            );
            receipt.record_final_chord_capture(capture);
    }
    let _ = projected.as_ref().inspect(|post| {
        crate::audit_only::voice_run_collision_origin::observe_final_chord_projection(
            candidate_xml, post,
        );
    });
    (projected, receipt)
}

#[cfg(test)]
mod tests {

    #[test]
    fn production_shorter_chord_members_keep_exact_head_bound_channel() {
        let scope = 83;
        for staff in [1, 2] {
            for unit in [1, 3, 7] {
                for member_units in [[1, 2], [2, 1], [4, 4]] {
                    let head_duration = 4 * unit;
                    let first_duration = member_units[0] * unit;
                    let second_duration = member_units[1] * unit;
                    let xml = format!(r#"<score-partwise><part id="synthetic"><measure number="{scope}">
                        <attributes><divisions>{unit}</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>
                        <note><pitch><step>C</step><octave>4</octave></pitch><duration>{head_duration}</duration><voice>1</voice><type>whole</type><staff>{staff}</staff></note>
                        <note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>{first_duration}</duration><voice>1</voice><staff>{staff}</staff></note>
                        <note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>{second_duration}</duration><voice>1</voice><staff>{staff}</staff></note>
                        </measure></part></score-partwise>"#);
                    let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
                    let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
                    assert_eq!(canonical.hard_same_edges.len(), 2);
                    let written = canonical.written_projection.as_ref().unwrap();
                    assert_eq!(written.occurrences.len(), 3);
                    for row in written.occurrences.iter() {
                        assert!(row.has_pitched_chord_anchor);
                        assert_eq!(row.chord_anchor_onset, 0);
                        assert_eq!(row.chord_anchor_end, head_duration);
                        assert_eq!(row.chord_anchor_start, written.occurrences[0].chord_anchor_start);
                    }
                    let attacks = evidence.iter().map(|row| FrozenVoiceAttackEvidenceRow {
                        source_event_id: row.source_event_id, attack_id: row.attack_id,
                        absolute_start: row.onset, absolute_end: row.onset + row.duration,
                        staff: row.staff, technical_lane: 1,
                    }).collect::<Vec<_>>();
                    let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks)
                        .expect("Positive canonical members may be shorter than their actual pitched head");
                    assert_eq!(channels.channel_count, 1);
                }
            }
        }
    }

    #[test]
    fn actual_chord_head_bounds_reject_oversized_shifted_or_unknown_members() {
        let ids = [103, 209, 307];
        let (mut canonical, attacks) = synthetic_written_channel_case(
            83, &ids, &[(0, 8), (0, 4), (0, 2)], &[(0, 8), (0, 4), (0, 2)], 1);
        canonical.hard_same_edges = std::sync::Arc::from(vec![
            FinalChordCanonicalHardSameEdge { left_fragment_id: ids[0], right_fragment_id: ids[1], presentation_staff: 1 },
            FinalChordCanonicalHardSameEdge { left_fragment_id: ids[0], right_fragment_id: ids[2], presentation_staff: 1 },
        ]);
        let evidence = canonical.written_projection.as_mut().unwrap();
        let mut rows = evidence.occurrences.to_vec();
        for row in &mut rows {
            row.chord_anchor_start = 0;
            row.chord_anchor_onset = 0;
            row.chord_anchor_end = 8;
        }
        evidence.occurrences = std::sync::Arc::from(rows);
        assert_eq!(build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks)
            .unwrap().channel_count, 1);
        for counterfactual in 0..5 {
            let mut changed = canonical.clone();
            let written = changed.written_projection.as_mut().unwrap();
            let mut rows = written.occurrences.to_vec();
            match counterfactual {
                0 => rows[1].end = 9,
                1 => rows[1].onset = 1,
                2 => rows[1].chord_anchor_end = 4,
                3 => rows[1].has_pitched_chord_anchor = false,
                _ => rows[1].chord_anchor_start = 100,
            }
            written.occurrences = std::sync::Arc::from(rows);
            let error = build_frozen_serialization_voice_channels_for_canonical(&changed, &attacks).unwrap_err();
            let text = error.to_string();
            assert!(text.contains("HARD SAME class has incompatible written obligations"));
            assert!(text.contains("written") && text.contains("anchor") && text.contains("fragment"));
        }
        canonical.hard_same_edges = std::sync::Arc::from([]);
        assert_eq!(build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks)
            .unwrap().channel_count, 3);
    }


    fn synthetic_written_channel_case(
        scope: usize,
        ids: &[usize],
        written_spans: &[(i64, i64)],
        sounding_spans: &[(i64, i64)],
        staff: u8,
    ) -> (CanonicalFinalChordStructuralPlan, Vec<FrozenVoiceAttackEvidenceRow>) {
        assert_eq!(ids.len(), written_spans.len());
        assert_eq!(ids.len(), sounding_spans.len());
        let locators = ids.iter().enumerate().map(|(index, &source)| {
            FinalChordCanonicalSourceLocator {
                fragment_id: source,
                source_event_id: source,
                attack_id: source,
                measure: scope,
                raw_note_index: index,
                onset: written_spans[index].0,
                duration: written_spans[index].1 - written_spans[index].0,
                pitch_identity: format!("synthetic_pitch_{index}"),
                pitch_midi: 60 + index as i32,
                staff: staff.to_string(),
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            }
        }).collect::<Vec<_>>();
        let occurrences = locators.iter().enumerate().map(|(index, locator)| {
            FinalChordWrittenOccurrence {
                source_event_id: locator.source_event_id,
                fragment_id: locator.fragment_id,
                attack_id: locator.attack_id,
                measure: scope,
                onset: written_spans[index].0,
                end: written_spans[index].1,
                staff,
                pitch_midi: locator.pitch_midi,
                chord_anchor_start: index,
                chord_anchor_onset: written_spans[index].0,
                chord_anchor_end: written_spans[index].1,
                has_pitched_chord_anchor: true,
                projected_note_start: index,
            }
        }).collect::<Vec<_>>();
        let attacks = ids.iter().enumerate().map(|(index, &source)| {
            FrozenVoiceAttackEvidenceRow {
                source_event_id: source,
                attack_id: source,
                absolute_start: sounding_spans[index].0,
                absolute_end: sounding_spans[index].1,
                staff,
                technical_lane: 1,
            }
        }).collect();
        (CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            source_locators: std::sync::Arc::from(locators),
            written_projection: Some(FinalChordWrittenProjectionEvidence {
                original_xml: std::sync::Arc::from("synthetic owner-local evidence"),
                xml: std::sync::Arc::from("synthetic owner-local evidence"),
                occurrences: std::sync::Arc::from(occurrences),
            }),
            ..Default::default()
        }, attacks)
    }

    #[test]
    fn written_obligations_override_disjoint_sounding_intervals_in_both_presentations() {
        for staff in [1, 2] {
            let ids = [103, 209];
            let (canonical, attacks) = synthetic_written_channel_case(
                47, &ids, &[(0, 6), (4, 8)], &[(0, 1), (1, 2)], staff);
            let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
            assert_eq!(channels.channel_count, 2);
            assert_ne!(channels.source_to_channel[&ids[0]], channels.source_to_channel[&ids[1]]);
            assert!(channels.channel_to_role.values().all(|role| *role ==
                final_chord_default_presentation_role(staff, 1).unwrap()));
        }
    }

    #[test]
    fn written_endpoint_adjacency_reuses_one_channel() {
        let (canonical, attacks) = synthetic_written_channel_case(
            83, &[103, 209], &[(0, 4), (4, 8)], &[(0, 1), (1, 2)], 1);
        let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        assert_eq!(channels.channel_count, 1);
    }

    #[test]
    fn equal_intervals_require_positive_chord_authority_and_preserve_hard_same() {
        let ids = [103, 209];
        let (mut canonical, attacks) = synthetic_written_channel_case(
            83, &ids, &[(0, 4), (0, 4)], &[(0, 1), (1, 2)], 1);
        let independent = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        assert_eq!(independent.channel_count, 2);
        canonical.hard_same_edges = std::sync::Arc::from(vec![FinalChordCanonicalHardSameEdge {
            left_fragment_id: ids[0], right_fragment_id: ids[1], presentation_staff: 1,
        }]);
        assert!(build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).is_err());
        let evidence = canonical.written_projection.as_mut().unwrap();
        let mut occurrences = evidence.occurrences.to_vec();
        occurrences[1].chord_anchor_start = occurrences[0].chord_anchor_start;
        evidence.occurrences = std::sync::Arc::from(occurrences);
        let legal = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        assert_eq!(legal.channel_count, 1);
        assert_eq!(canonical.hard_same_edges.len(), 1);
        let evidence = canonical.written_projection.as_mut().unwrap();
        let mut occurrences = evidence.occurrences.to_vec();
        occurrences[0].has_pitched_chord_anchor = false;
        evidence.occurrences = std::sync::Arc::from(occurrences);
        assert!(build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).is_err());
    }

    #[test]
    fn channel_reuse_checks_the_member_that_did_not_offer_it() {
        let ids = [103, 209, 307];
        let (canonical, attacks) = synthetic_written_channel_case(
            83, &ids, &[(0, 3), (3, 6), (2, 3)], &[(0, 1), (1, 2), (2, 3)], 1);
        let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        assert_eq!(channels.source_to_channel[&ids[0]], channels.source_to_channel[&ids[1]]);
        assert_ne!(channels.source_to_channel[&ids[0]], channels.source_to_channel[&ids[2]]);
    }

    #[test]
    fn missing_written_provenance_never_falls_back_to_sounding_only() {
        let (mut canonical, attacks) = synthetic_written_channel_case(
            83, &[103, 209], &[(0, 4), (4, 8)], &[(0, 1), (1, 2)], 1);
        let evidence = canonical.written_projection.as_mut().unwrap();
        evidence.occurrences = std::sync::Arc::from(vec![evidence.occurrences[0].clone()]);
        assert!(build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).is_err());
        canonical.written_projection = None;
        assert!(build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).is_err());
    }

    #[test]
    fn source_roles_survive_technical_channel_reassignment() {
        for staff in [1, 2] {
            let ids = [103, 209];
            let (canonical, mut attacks) = synthetic_written_channel_case(
                83, &ids, &[(0, 1), (1, 2)], &[(0, 1), (1, 2)], staff);
            attacks[1].technical_lane = 2;
            let mut channels = build_frozen_serialization_voice_channels_for_canonical(
                &canonical, &attacks).unwrap();
            let roles = ids.map(|source| channels.role_for_source(source).unwrap());
            assert_ne!(roles[0], roles[1]);
            for source in ids {
                assert_eq!(channels.role_for_source(source),
                    channels.channel_to_role.get(&channels.source_to_channel[&source]).copied());
            }
            // This models only a later technical reassignment, not its admission.
            // The productive allocator still rejects role-incompatible reuse.
            channels.source_to_channel = ids.into_iter().map(|source| (source, 823)).collect();
            channels.channel_to_role.clear();
            for (source, role) in ids.into_iter().zip(roles) {
                assert_eq!(channels.role_for_source(source), Some(role));
            }
        }
    }

    #[test]
    fn missing_source_role_never_falls_back_to_a_channel_role() {
        let ids = [103, 209];
        let (canonical, attacks) = synthetic_written_channel_case(
            83, &ids, &[(0, 1), (1, 2)], &[(0, 1), (1, 2)], 1);
        let mut channels = build_frozen_serialization_voice_channels_for_canonical(
            &canonical, &attacks).unwrap();
        assert!(channels.channel_for_source(ids[0]).is_some());
        assert!(channels.role_for_source(ids[0]).is_some());
        channels.source_to_role.remove(&ids[0]);
        assert_eq!(channels.role_for_source(ids[0]), None);
        assert_eq!(channels.role_for_source(701), None);
        assert!(channels.role_for_source(ids[1]).is_some());
    }

    #[test]
    fn canonical_source_roles_survive_identity_time_and_staff_transport() {
        for staff in [1, 2] {
            for scale in [1, 7, 19] {
                let ids = [907, 503];
                let (mut canonical, mut attacks) = synthetic_written_channel_case(
                    311, &ids, &[(0, scale), (scale, 2 * scale)],
                    &[(0, scale), (scale, 2 * scale)], staff);
                attacks[1].technical_lane = 2;
                let expected = ids.map(|source| {
                    let attack = attacks.iter().find(|row| row.source_event_id == source).unwrap();
                    final_chord_default_presentation_role(attack.staff, attack.technical_lane).unwrap()
                });
                let mut locators = canonical.source_locators.to_vec();
                locators.reverse();
                canonical.source_locators = std::sync::Arc::from(locators);
                attacks.reverse();
                let channels = build_frozen_serialization_voice_channels_for_canonical(
                    &canonical, &attacks).unwrap();
                for (source, role) in ids.into_iter().zip(expected) {
                    assert_eq!(channels.role_for_source(source), Some(role));
                }
            }
        }
    }

    #[test]
    fn channel_semantics_survive_opaque_identity_and_input_order_renaming() {
        let first_ids = [103, 209, 307];
        let second_ids = [907, 503, 701];
        let written = [(0, 3), (3, 6), (2, 3)];
        let sounding = [(0, 1), (1, 2), (2, 3)];
        let (first, first_attacks) = synthetic_written_channel_case(83, &first_ids, &written, &sounding, 1);
        let (mut second, mut second_attacks) = synthetic_written_channel_case(83, &second_ids, &written, &sounding, 1);
        let mut rows = second.source_locators.to_vec(); rows.reverse();
        second.source_locators = std::sync::Arc::from(rows);
        let evidence = second.written_projection.as_mut().unwrap();
        let mut rows = evidence.occurrences.to_vec();
        for row in &mut rows { row.chord_anchor_start += 1000; }
        rows.reverse(); evidence.occurrences = std::sync::Arc::from(rows);
        second_attacks.reverse();
        let first = build_frozen_serialization_voice_channels_for_canonical(&first, &first_attacks).unwrap();
        let second = build_frozen_serialization_voice_channels_for_canonical(&second, &second_attacks).unwrap();
        assert_eq!(first.semantic_fingerprint_fnv64, second.semantic_fingerprint_fnv64);
        for (&first_left, &second_left) in first_ids.iter().zip(&second_ids) {
            for (&first_right, &second_right) in first_ids.iter().zip(&second_ids) {
                assert_eq!(first.source_to_channel[&first_left] == first.source_to_channel[&first_right],
                    second.source_to_channel[&second_left] == second.source_to_channel[&second_right]);
            }
        }
    }

    #[test]
    fn independent_candidate_duplication_preserves_existing_local_channel_decisions() {
        let ids = [103, 209];
        let (baseline, baseline_attacks) = synthetic_written_channel_case(
            83, &ids, &[(0, 6), (4, 8)], &[(0, 1), (1, 2)], 1);
        let baseline = build_frozen_serialization_voice_channels_for_canonical(&baseline, &baseline_attacks).unwrap();
        for count in [1, 2, 5] {
            let mut expanded_ids = ids.to_vec();
            let mut written = vec![(0, 6), (4, 8)];
            let mut sounding = vec![(0, 1), (1, 2)];
            for ordinal in 0..count {
                expanded_ids.push(1000 + ordinal);
                let start = 10 + ordinal as i64 * 2;
                written.push((start, start + 1)); sounding.push((start, start + 1));
            }
            let (expanded, attacks) = synthetic_written_channel_case(83, &expanded_ids, &written, &sounding, 1);
            let expanded = build_frozen_serialization_voice_channels_for_canonical(&expanded, &attacks).unwrap();
            for source in ids {
                assert_eq!(baseline.source_to_channel[&source], expanded.source_to_channel[&source]);
            }
        }
    }

    #[test]
    fn supplemental_channels_preserve_reserved_main_and_shared_secondary_labels() {
        use SerializedVoicePresentationRole::*;
        let roles = [UpperPrimary, UpperPrimary, LowerPrimary, UpperSecondary, UpperAuxiliary,
            LowerSecondary, LowerAuxiliary];
        let channels = FrozenSerializationVoiceChannelPlan {
            authority_bound: true,
            source_to_channel: (0..roles.len()).map(|index| (100 + index, index + 1)).collect(),
            source_to_role: roles.into_iter().enumerate().map(|(index, role)| (100 + index, role)).collect(),
            channel_to_role: roles.into_iter().enumerate().map(|(index, role)| (index + 1, role)).collect(),
            channel_count: roles.len(),
            semantic_fingerprint_fnv64: 17,
            consumer_allocation: ChannelConsumerAllocation::Undetermined("synthetic label-binding input"),
            label_continuation: None,
        };
        let plan = bind_musicxml_voice_labels(&channels, &[]).unwrap();
        assert_eq!(plan.channel_to_label[&1], "1");
        assert_eq!(plan.channel_to_label[&3], "2");
        assert_eq!(plan.channel_to_label[&4], "3");
        assert_eq!(plan.channel_to_label[&6], "4");
        assert_eq!(plan.channel_to_label.values().collect::<std::collections::BTreeSet<_>>().len(), roles.len());
        for (channel, role) in channels.channel_to_role {
            assert_eq!(plan.channel_to_role[&channel], role);
            assert_eq!(plan.label_to_role[&plan.channel_to_label[&channel]], role);
        }
    }

    #[test]
    fn written_channel_separation_materializes_labels_without_retiming_or_hand_transfer() {
        let scope = 83;
        let xml = format!(r#"<score-partwise><part id="synthetic"><measure number="{scope}">
            <attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>
            <note><pitch><step>C</step><octave>4</octave></pitch><duration>6</duration><voice>1</voice><type>quarter</type><dot/><staff>1</staff></note>
            <backup><duration>2</duration></backup>
            <note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>
            </measure></part></score-partwise>"#);
        let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical, bound) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
        let before = project_final_chord_encoding_plan_with_provenance(&xml, &bound).unwrap().xml;
        let attacks = evidence.iter().enumerate().map(|(index, row)| FrozenVoiceAttackEvidenceRow {
            source_event_id: row.source_event_id, attack_id: row.attack_id,
            absolute_start: index as i64, absolute_end: index as i64 + 1,
            staff: row.staff, technical_lane: 1,
        }).collect::<Vec<_>>();
        let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        let labels = bind_musicxml_voice_labels(&channels, &[]).unwrap();
        let bound = bind_canonical_final_chord_encoding_plan(&xml, &canonical, &labels).unwrap();
        let after = project_final_chord_encoding_plan_with_provenance(&xml, &bound).unwrap().xml;
        let geometry = |text: &str| final_chord_parse_binding_measures(text).unwrap().into_iter()
            .flat_map(|measure| measure.notes).map(|note| (note.measure, note.onset, note.duration,
                note.pitch_identity, note.staff, note.chord_member, note.tie_start, note.tie_stop))
            .collect::<Vec<_>>();
        assert_eq!(geometry(&before), geometry(&after));
        let voices = final_chord_parse_binding_measures(&after).unwrap().into_iter()
            .flat_map(|measure| measure.notes).map(|note| note.voice)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(voices.len(), 2);
        assert!(labels.channel_to_role.values().all(|role| *role == SerializedVoicePresentationRole::UpperPrimary));
    }

    #[test]
    fn productive_written_projection_seals_exact_bytes_and_complete_origin_provenance() {
        let scope = 83;
        let xml = format!(r#"<score-partwise><part id="synthetic"><measure number="{scope}">
            <attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>
            <note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>
            </measure></part></score-partwise>"#);
        let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
        let (canonical, bound) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
        let mut projection = project_final_chord_encoding_plan_with_provenance(&xml, &bound).unwrap();
        canonical.validate_written_projection(&projection.xml).unwrap();
        assert!(canonical.validate_written_projection(&(projection.xml.clone() + " ")).is_err());
        assert_eq!(canonical.written_projection.as_ref().unwrap().occurrences.len(), evidence.len());
        projection.provenance.clear();
        assert!(final_chord_bind_written_projection(&xml, &canonical, projection).is_err());
    }

    macro_rules! fixture_xml {
        ($($part:expr),+ $(,)?) => { &[$($part),+].concat() };
    }

    const FIXTURE_NOTE_070FEEE235: &str = r#"      <note>
        <rest/>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>"#;
    const FIXTURE_NOTE_1C6F0511B4: &str = r#"      <note print-object="no">
        <chord/>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>"#;
    const FIXTURE_NOTE_395A683D76: &str = r#"      <note>
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>"#;
    const FIXTURE_NOTE_631875EE3D: &str = r#"      <note>
        <rest/>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>"#;
    const FIXTURE_NOTE_64DC3197A6: &str = r#"      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>"#;
    const FIXTURE_NOTE_D42565F8C7: &str = r#"      <note>
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>"#;
    const FIXTURE_NOTE_F714A49732: &str = r#"      <note>
        <rest/>
        <duration>12</duration>
        <voice>1</voice>
        <type>half</type><dot/>
        <staff>1</staff>
      </note>"#;
    const FIXTURE_NOTE_F9745462DA: &str = r#"      <note>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>"#;


    fn baseline_final_chord_canonical_source_locator() -> FinalChordCanonicalSourceLocator {
        FinalChordCanonicalSourceLocator {
            fragment_id: 1,
            source_event_id: 1,
            attack_id: 1,
            measure: 1,
            raw_note_index: 0,
            onset: 0,
            duration: 4,
            pitch_identity: "midi:64".to_string(),
            pitch_midi: 64,
            staff: "1".to_string(),
            chord_syntax: false,
            tie_start: false,
            tie_stop: false,
        }
    }


    use super::*;
    use crate::structural_write_authority::{
        adapt_structural_snapshot_read_only, detect_structural_repair_intents_read_only,
    };

    const SYNTHETIC_SNAPSHOT: &str = include_str!(
        "../../tests/fixtures/synthetic/cross_staff_tail_structural_invariant.xml"
    );

    fn synthetic_audit() -> IdentityFreeSemanticEventEquivalenceAudit {
        let (views, adapter) =
            adapt_structural_snapshot_read_only(SYNTHETIC_SNAPSHOT, 96, "synthetic");
        assert!(adapter.pipeline_line("fixture").contains("ParseOk ja"));
        let intents = detect_structural_repair_intents_read_only(&views);
        audit_identity_free_semantic_event_equivalence(&intents, views.len(), "synthetic")
    }

    fn shadow_rest_voice_plan_for_test() -> FrozenSerializedVoicePlan {
        let label_to_role = [
            ("1".to_string(), SerializedVoicePresentationRole::UpperPrimary),
            ("2".to_string(), SerializedVoicePresentationRole::LowerPrimary),
            ("3".to_string(), SerializedVoicePresentationRole::LowerSecondary),
        ]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
        let channel_to_label = [
            (1_usize, "1".to_string()),
            (2_usize, "2".to_string()),
            (3_usize, "3".to_string()),
        ]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
        let channel_to_role = [
            (1_usize, SerializedVoicePresentationRole::UpperPrimary),
            (2_usize, SerializedVoicePresentationRole::LowerPrimary),
            (3_usize, SerializedVoicePresentationRole::LowerSecondary),
        ]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
        FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: std::collections::BTreeMap::new(),
            authority_bound: true,
            source_to_channel: std::collections::BTreeMap::new(),
            source_to_label: std::collections::BTreeMap::new(),
            label_to_role,
            channel_to_label,
            channel_to_role,
            semantic_fingerprint_fnv64: 1,
        }
    }

    fn shadow_rest_scope_score(pitched_staff: u8) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1"><measure number="1">
    <attributes><divisions>96</divisions><time><beats>1</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>
    <note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>
    <backup><duration>96</duration></backup>
    <note><pitch><step>C</step><octave>5</octave></pitch><duration>96</duration><voice>{}</voice><staff>{}</staff></note>
  </measure></part>
</score-partwise>"#,
            if pitched_staff == 1 { 1 } else { 3 },
            pitched_staff,
        )
    }

    #[test]
    fn frozen_voice_shadow_rest_preserves_r788_same_staff_decision_scope() {
        let xml = shadow_rest_scope_score(1);
        let plan = shadow_rest_voice_plan_for_test();
        assert_eq!(
            crate::pipeline::final_visible_shadow_bookkeeping_regression_candidates(&xml),
            Some(0)
        );
        assert_eq!(
            crate::pipeline::final_visible_shadow_bookkeeping_regression_candidates_with_voice_binding(
                &xml, &plan,
            ),
            Some(0)
        );
        assert_eq!(
            crate::pipeline::final_visible_shadow_bookkeeping_regression_candidate_rows(
                &xml, &plan,
            ),
            Some(Vec::new())
        );
    }

    #[test]
    fn frozen_voice_shadow_rest_count_and_rows_share_one_bound_predicate() {
        let xml = shadow_rest_scope_score(2);
        let plan = shadow_rest_voice_plan_for_test();
        assert_eq!(
            crate::pipeline::final_visible_shadow_bookkeeping_regression_candidates_with_voice_binding(
                &xml, &plan,
            ),
            Some(1)
        );
        let rows = crate::pipeline::final_visible_shadow_bookkeeping_regression_candidate_rows(
            &xml, &plan,
        )
        .expect("rows");
        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains("m1@0+96:s2:v2"));
    }

    #[test]
    fn final_chord_forward_carrier_counts_same_boundary_insertion_before_surviving_note() {
        let plan = FinalChordEncodingPlan {
            source_fingerprint: 0,
            source_len: 64,
            edits: vec![FinalChordEncodingEdit {
                start: 20,
                end: 20,
                replacement: "<inserted/>".to_string(),
                generated_fragments: Vec::new(),
            }],
            forward_materializations: Vec::new(),
            visible_rest_interval_projection_authorized: false,
            original_course_bindings: None,
        original_respelling_input: None,
        };
        assert_eq!(
            plan.carry_surviving_source_note_start(20, 30),
            Ok(20 + "<inserted/>".len())
        );
    }

    #[test]
    fn final_chord_forward_carrier_counts_same_boundary_insertion_with_exact_replacement() {
        let replacement = "<note><duration>24</duration></note>".to_string();
        let prefix = "<inserted/>".to_string();
        let plan = FinalChordEncodingPlan {
            source_fingerprint: 0,
            source_len: 64,
            edits: vec![
                FinalChordEncodingEdit {
                    start: 20,
                    end: 30,
                    replacement,
                    generated_fragments: Vec::new(),
                },
                FinalChordEncodingEdit {
                    start: 20,
                    end: 20,
                    replacement: prefix.clone(),
                    generated_fragments: Vec::new(),
                },
            ],
            forward_materializations: Vec::new(),
            visible_rest_interval_projection_authorized: false,
            original_course_bindings: None,
        original_respelling_input: None,
        };
        assert_eq!(
            plan.carry_surviving_source_note_start(20, 30),
            Ok(20 + prefix.len())
        );
    }

    #[test]
    fn synthetic_structure_proves_semantic_equivalence_and_encoding_normalization() {
        let audit = synthetic_audit();
        assert_eq!(audit.repair_intent_count, 1);
        assert!(audit.unique_repair_intent);
        assert!(audit.source_event_mapping_complete);
        assert!(audit.source_event_ordinals_unique);
        assert_eq!(audit.plan_rows.len(), 7);
        assert_eq!(audit.main_lane_events, 5);
        assert_eq!(audit.auxiliary_lanes, 2);
        assert_eq!(audit.full_backups_to_measure_start, 2);
        assert_eq!(audit.partial_backups_to_nonzero, 0);
        assert!(audit.cursor_neutral_topology_feasible);
        assert!(audit.target_members_contiguous);
        assert!(!audit.hidden_carrier_between_target_members);
        assert_eq!(audit.required_chord_normalizations, 1);
        assert_eq!(audit.proven_chord_normalizations, 1);
        assert_eq!(audit.unproven_chord_normalizations, 0);
        assert_eq!(audit.artificial_chord_additions, 0);
        assert_eq!(audit.semantic_complete_event_delta_rows, 0);
        assert_eq!(audit.serialization_encoding_delta_rows, 2);
        assert_eq!(audit.sounding_identity_delta_rows, 0);
        assert!(audit.semantic_event_invariant_green);
        assert!(audit.encoding_normalization_proof_green);
        assert!(!audit.chord_state_stable);
        assert!(audit.serialization_plan_feasible);
        assert_eq!(audit.blocking_invariant, "none");
        assert!(!audit.write_produced);
        assert!(!audit.writer_token_used);
    }

    #[test]
    fn identity_changes_do_not_change_plan_or_equivalence_proof() {
        let transformed = SYNTHETIC_SNAPSHOT
            .replace("number=\"synthetic\"", "number=\"relocated\"")
            .replace("<voice>2</voice>", "<voice>9</voice>")
            .replace("<staff>1</staff>", "<staff>4</staff>")
            .replace("<staff>2</staff>", "<staff>6</staff>")
            .replace("<step>B</step>", "<step>C</step>")
            .replace("<step>F</step>", "<step>D</step>")
            .replace("<step>A</step>", "<step>E</step>");
        let (original_views, _) =
            adapt_structural_snapshot_read_only(SYNTHETIC_SNAPSHOT, 96, "original");
        let (transformed_views, _) =
            adapt_structural_snapshot_read_only(&transformed, 96, "transformed");
        let original_intents = detect_structural_repair_intents_read_only(&original_views);
        let transformed_intents = detect_structural_repair_intents_read_only(&transformed_views);
        let original = audit_identity_free_semantic_event_equivalence(
            &original_intents,
            original_views.len(),
            "original",
        );
        let transformed = audit_identity_free_semantic_event_equivalence(
            &transformed_intents,
            transformed_views.len(),
            "transformed",
        );
        assert_eq!(original.repair_intent_count, transformed.repair_intent_count);
        assert_eq!(original.plan_rows.len(), transformed.plan_rows.len());
        assert_eq!(original.main_lane_events, transformed.main_lane_events);
        assert_eq!(original.auxiliary_lanes, transformed.auxiliary_lanes);
        assert_eq!(
            original.semantic_complete_event_delta_rows,
            transformed.semantic_complete_event_delta_rows
        );
        assert_eq!(
            original.serialization_encoding_delta_rows,
            transformed.serialization_encoding_delta_rows
        );
        assert_eq!(
            original.proven_chord_normalizations,
            transformed.proven_chord_normalizations
        );
        assert_eq!(
            original.serialization_plan_feasible,
            transformed.serialization_plan_feasible
        );
    }

    #[test]
    fn planner_never_writes_or_issues_authority_token() {
        let audit = synthetic_audit();
        assert!(!audit.detector_identity_fields_exposed);
        assert!(!audit.planner_performed_musical_selection);
        assert!(!audit.writer_token_used);
        assert!(!audit.write_produced);
        assert!(audit.pipeline_line("fixture").contains("RenderXmlWrite nein"));
    }

    #[test]
    fn chord_removal_without_bound_hidden_carrier_is_blocked() {
        let audit = synthetic_audit();
        let mut rows = audit.plan_rows.clone();
        let hidden = rows
            .iter_mut()
            .find(|row| row.carrier_role == StructuralCarrierRole::HiddenCarrier)
            .expect("real plan must contain hidden carrier");
        hidden.carrier_role = StructuralCarrierRole::Rest;
        let proof = prove_encoding_normalizations(&rows);
        assert_eq!(proof.required_removals, 1);
        assert_eq!(proof.proven_removals, 0);
        assert_eq!(proof.unproven_removals, 1);
        assert!(!proof.green);
    }

    #[test]
    fn artificial_chord_addition_is_never_authorized() {
        let audit = synthetic_audit();
        let mut rows = audit.plan_rows.clone();
        let first = rows
            .iter_mut()
            .find(|row| !row.original_chord_member && row.planned_lane == PlannedLane::MainVoiceRun)
            .expect("real plan must contain ordinary main-lane event");
        first.planned_chord_member = true;
        let proof = prove_encoding_normalizations(&rows);
        assert_eq!(proof.artificial_additions, 1);
        assert!(!proof.green);
    }

    #[test]
    fn planned_onset_drift_is_detected_in_sounding_identity() {
        let audit = synthetic_audit();
        let mut rows = audit.plan_rows.clone();
        let target = rows
            .iter_mut()
            .find(|row| row.original_chord_member && !row.planned_chord_member)
            .expect("real plan must contain normalized target");
        target.planned_cursor_before += 24;
        target.planned_cursor_after += 24;
        let original = sorted(
            rows.iter()
                .filter(|row| row.carrier_role == StructuralCarrierRole::Sounding)
                .map(sounding_identity_original_key)
                .collect(),
        );
        let planned = sorted(
            rows.iter()
                .filter(|row| row.carrier_role == StructuralCarrierRole::Sounding)
                .map(sounding_identity_planned_key)
                .collect(),
        );
        assert!(sorted_multiset_delta_rows(original, planned) > 0);
    }

    fn final_chord_score(measure_body: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<score-partwise version="4.0">
  <part-list>
    <score-part id="P1"><part-name>Piano</part-name></score-part>
  </part-list>
  <part id="P1">
    <measure number="1">
      <attributes>
        <divisions>4</divisions>
        <time><beats>4</beats><beat-type>4</beat-type></time>
      </attributes>
{measure_body}
    </measure>
  </part>
</score-partwise>
"#
        )
    }

    #[test]
    fn authorized_cursor_neutral_transport_accepts_only_exact_chord_removal_with_stable_pitched_identity() {
        let before = final_chord_score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        let after = final_chord_score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <backup><duration>4</duration></backup>
      <note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        final_chord_prove_authorized_cursor_neutral_pitched_identity_transport(
            &before,
            &after,
            1,
            true,
        )
        .expect("exact chord normalization should preserve pitched occurrence identity");
    }

    #[test]
    fn authorized_cursor_neutral_transport_rejects_pitched_state_drift_and_unauthorized_noop_drift() {
        let before = final_chord_score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        let pitch_drift = final_chord_score(
            r#"      <note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        assert!(final_chord_prove_authorized_cursor_neutral_pitched_identity_transport(
            &before,
            &pitch_drift,
            1,
            true,
        )
        .is_err());
        assert!(final_chord_prove_authorized_cursor_neutral_pitched_identity_transport(
            &before,
            &pitch_drift,
            0,
            true,
        )
        .is_err());
    }

    #[test]
    fn final_notation_checkpoint_transport_accepts_clef_only_change_and_rejects_note_drift() {
        let before = final_chord_score(
            r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        let after_clef = before.replace(
            "</attributes>",
            "<clef number=\"1\"><sign>G</sign><line>2</line></clef></attributes>",
        );
        final_chord_verify_pitched_sequence_transition(&before, &after_clef, false)
            .expect("clef-only FinalNotation materialization must preserve pitched identity");

        let pitch_drift = after_clef.replace("<step>C</step>", "<step>D</step>");
        assert!(
            final_chord_verify_pitched_sequence_transition(&before, &pitch_drift, false).is_err()
        );
    }

    #[test]
    fn output_alpha_defers_only_receipt_owned_forward_orphan_to_existing_host_normalizer() {
        let note = FinalChordEncodingNote {
            range: 0..1,
            measure: 1,
            raw_note_index: 0,
            onset: 4,
            duration: 4,
            chord_member: true,
            rest_like: false,
            pitch_identity: "midi:64".to_string(),
            pitch_midi: Some(64),
            voice: "1".to_string(),
            staff: "1".to_string(),
            tie_start: false,
            tie_stop: false,
            anchor_index: None,
            orphan_after_forward: true,
            source_event_id: None,
            attack_id: None,
            fragment_id: None,
        };
        let locator = FinalChordCanonicalSourceLocator {
            fragment_id: 7,
            source_event_id: 70,
            attack_id: 70,
            chord_syntax: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let owner = FinalChordTransactionOutputOccurrenceWitness {
            fragment_id: 7,
            source_event_id: 70,
            attack_id: 70,
            materialization_kind: FinalChordProjectionMaterializationKind::SurvivingSourceNote,
            fragment_ordinal: 0,
            measure: 1,
            pitched_ordinal: 0,
        };
        let locators = [(locator.fragment_id, &locator)].into_iter().collect();
        assert!(final_chord_output_alpha_owned_transient_render_orphan(
            &note,
            Some(&owner),
            &locators,
            true,
        ));
        assert!(!final_chord_output_alpha_owned_transient_render_orphan(
            &note,
            None,
            &locators,
            true,
        ));
        assert!(!final_chord_output_alpha_owned_transient_render_orphan(
            &note,
            Some(&owner),
            &locators,
            false,
        ));

        let mut not_after_forward = note.clone();
        not_after_forward.orphan_after_forward = false;
        assert!(!final_chord_output_alpha_owned_transient_render_orphan(
            &not_after_forward,
            Some(&owner),
            &locators,
            true,
        ));

        let mut generated_owner = owner;
        generated_owner.materialization_kind =
            FinalChordProjectionMaterializationKind::AuxiliaryLane;
        assert!(!final_chord_output_alpha_owned_transient_render_orphan(
            &note,
            Some(&generated_owner),
            &locators,
            true,
        ));
    }

    #[test]
    fn orphan_chord_host_transport_accepts_only_exact_forward_to_rest_onset_rebase() {
        let before = final_chord_score(
            r#"      <forward><duration>4</duration></forward>
      <note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        let after = final_chord_score(
            r#"      <note><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        let after_measures = final_chord_prove_orphan_chord_host_onset_rebase(
            &before,
            &after,
            2,
        )
        .expect("exact forward-to-rest host normalization should rebase only the orphan chord onsets");
        let pitched_onsets = after_measures[0]
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .map(|note| note.onset)
            .collect::<Vec<_>>();
        assert_eq!(pitched_onsets, vec![0, 0]);
        assert!(final_chord_prove_orphan_chord_host_onset_rebase(&before, &after, 1).is_err());

        let payload_drift = after.replace("<step>E</step>", "<step>F</step>");
        assert!(final_chord_prove_orphan_chord_host_onset_rebase(
            &before,
            &payload_drift,
            2,
        )
        .is_err());

        let rest_host_before = final_chord_score(
            r#"      <note print-object="no"><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        let promoted_anchor_after = final_chord_score(
            r#"      <note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>
      <note><chord/><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>"#,
        );
        let promoted_measures = final_chord_prove_orphan_chord_host_onset_rebase(
            &rest_host_before,
            &promoted_anchor_after,
            1,
        )
        .expect("exact rest-host to existing pitched-anchor promotion should preserve occurrence identity");
        let promoted_onsets = promoted_measures[0]
            .notes
            .iter()
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .map(|note| note.onset)
            .collect::<Vec<_>>();
        assert_eq!(promoted_onsets, vec![0, 0]);
        assert!(final_chord_prove_orphan_chord_host_onset_rebase(
            &rest_host_before,
            &promoted_anchor_after,
            2,
        )
        .is_err());
    }

    #[test]
    fn p2_render_occurrence_witness_binds_stale_ambiguous_physical_locator_without_inversion() {
        let render_xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_64DC3197A6, r#"
"#, FIXTURE_NOTE_D42565F8C7, r#"
      <note>
        <chord/>
        <pitch><step>G</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_D42565F8C7, r#"
"#, FIXTURE_NOTE_631875EE3D, r#"
      <note>
        <pitch><step>D</step><octave>3</octave></pitch>
        <duration>4</duration>
        <voice>2</voice>
        <type>eighth</type>
        <staff>2</staff>
        <beam number="1">begin</beam>
      </note>"#),
        );
        let locators = vec![
            FinalChordCanonicalSourceLocator {
                pitch_identity: "midi:60".to_string(),
                pitch_midi: 60,
                ..baseline_final_chord_canonical_source_locator()
            },
            FinalChordCanonicalSourceLocator {
                fragment_id: 2,
                source_event_id: 2,
                attack_id: 2,
                raw_note_index: 1,
                chord_syntax: true,
                ..baseline_final_chord_canonical_source_locator()
            },
            FinalChordCanonicalSourceLocator {
                fragment_id: 3,
                source_event_id: 3,
                attack_id: 3,
                raw_note_index: 2,
                onset: 4,
                chord_syntax: true,
                ..baseline_final_chord_canonical_source_locator()
            },
            FinalChordCanonicalSourceLocator {
                fragment_id: 4,
                source_event_id: 4,
                attack_id: 4,
                raw_note_index: 2,
                pitch_identity: "midi:67".to_string(),
                pitch_midi: 67,
                chord_syntax: true,
                ..baseline_final_chord_canonical_source_locator()
            },
            FinalChordCanonicalSourceLocator {
                fragment_id: 5,
                source_event_id: 5,
                attack_id: 5,
                raw_note_index: 5,
                onset: 12,
                pitch_identity: "midi:50".to_string(),
                pitch_midi: 50,
                staff: "2".to_string(),
                ..baseline_final_chord_canonical_source_locator()
            },
        ];
        let base = CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(locators),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(Vec::<FinalChordCanonicalActionRow>::new()),
            hard_same_edges: std::sync::Arc::from(Vec::<FinalChordCanonicalHardSameEdge>::new()),
            measures: std::sync::Arc::from(vec![FinalChordCanonicalMeasureInfo {
                measure: 1,
                divisions: 4,
                expected_duration: 16,
            }]),
            semantic_fingerprint_fnv64: 1,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        };
        let voice_plan = FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: std::collections::BTreeMap::new(),
            authority_bound: true,
            source_to_channel: [(1, 1), (2, 1), (3, 1), (4, 1), (5, 2)]
                .into_iter()
                .collect(),
            source_to_label: [
                (1, "1".to_string()),
                (2, "1".to_string()),
                (3, "1".to_string()),
                (4, "1".to_string()),
                (5, "17".to_string()),
            ]
            .into_iter()
            .collect(),
            label_to_role: [
                ("1".to_string(), SerializedVoicePresentationRole::UpperPrimary),
                ("17".to_string(), SerializedVoicePresentationRole::LowerPrimary),
            ]
            .into_iter()
            .collect(),
            channel_to_label: [(1, "1".to_string()), (2, "17".to_string())]
                .into_iter()
                .collect(),
            channel_to_role: [
                (1, SerializedVoicePresentationRole::UpperPrimary),
                (2, SerializedVoicePresentationRole::LowerPrimary),
            ]
            .into_iter()
            .collect(),
            semantic_fingerprint_fnv64: 1,
        };

        assert!(
            bind_canonical_final_chord_encoding_plan(&render_xml, &base, &voice_plan).is_err(),
            "stale duplicate physical locator must remain ambiguous without writer continuation"
        );

        let measures = final_chord_parse_binding_measures(&render_xml).expect("parse render");
        let sequence_fingerprint_fnv64 =
            final_chord_voice_free_pitched_sequence_fingerprint(&measures);
        let mut canonical = base;
        attach_final_chord_render_occurrence_witnesses(
            &mut canonical,
            vec![FinalChordRenderOccurrenceWitness {
                fragment_id: 3,
                source_event_id: 3,
                attack_id: 3,
                measure: 1,
                pitched_ordinal: 3,
                representation_duration: None,
                sequence_fingerprint_fnv64,
            }],
        )
        .expect("attach writer continuation");

        assert!(
            bind_canonical_final_chord_encoding_plan(&render_xml, &canonical, &voice_plan).is_err(),
            "master/normal binding must never consume render-only writer continuation"
        );
        let render_omissions =
            std::collections::BTreeSet::<RenderPrunedFinalChordOmissionKey>::new();
        let plan = bind_canonical_final_chord_encoding_plan_for_render_projection(
            &render_xml,
            &canonical,
            &voice_plan,
            &render_omissions,
        )
        .expect("render projection must bind exact generated occurrence from writer continuation");
        let projected =
            project_final_chord_encoding_plan(&render_xml, &plan).0.expect("project labels");
        let projected_measures =
            final_chord_parse_binding_measures(&projected).expect("parse projected");
        let e_chords = projected_measures[0]
            .notes
            .iter()
            .filter(|note| note.chord_member && note.pitch_midi == Some(64))
            .collect::<Vec<_>>();
        assert_eq!(e_chords.len(), 2);
        assert_eq!(e_chords[0].voice, "1");
        assert_eq!(e_chords[1].voice, "1");

        let alpha = audit_musicxml_voice_label_binding_alpha_for_render_projection(
            &render_xml,
            &canonical,
            &voice_plan,
            &render_omissions,
        )
        .expect("render output alpha must compose writer bind with alpha-stable projected occurrence readback");
        assert!(alpha.green());
        assert_eq!(alpha.cases(), 1);
        assert_eq!(alpha.semantic_role_deltas(), 0);
        assert_eq!(alpha.paired_binding_failures(), 0);
        assert_eq!(alpha.stale_binding_false_accepts(), 0);

        let bound =
            bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
                &render_xml,
                &canonical,
                &voice_plan,
                Some(&render_omissions),
            )
            .expect("render canonical-bound transaction plan");
        let projected =
            project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha(
                &render_xml,
                &canonical,
                &bound,
            )
            .expect("transaction-local canonical projection");
        let drifted = projected.xml.replacen("<step>D</step>", "<step>F</step>", 1);
        assert!(
            verify_canonical_final_chord_transaction_local_output_alpha(
                &drifted,
                &canonical,
                &voice_plan,
                Some(&render_omissions),
                &projected.receipt,
                bound.plan.source_fingerprint,
                bound.plan.source_len,
            )
            .is_err(),
            "post-projection topology drift must not be self-certified by the receipt"
        );

        let mut wrong_epoch = projected.receipt.clone();
        wrong_epoch.bound_source_fingerprint_fnv64 ^= 1;
        assert!(
            verify_canonical_final_chord_transaction_local_output_alpha(
                &projected.xml,
                &canonical,
                &voice_plan,
                Some(&render_omissions),
                &wrong_epoch,
                bound.plan.source_fingerprint,
                bound.plan.source_len,
            )
            .is_err(),
            "a receipt claiming a different bound-source epoch must fail even when output topology matches"
        );

        let parsed = final_chord_parse_binding_measures(&projected.xml)
            .expect("projected output for duplicate-occurrence regression");
        let duplicate_range = parsed
            .iter()
            .flat_map(|measure| measure.notes.iter())
            .find(|note| !note.rest_like && note.pitch_midi == Some(50))
            .expect("independent D occurrence")
            .range
            .clone();
        let duplicate_xml = projected
            .xml
            .get(duplicate_range)
            .expect("duplicate occurrence bytes")
            .to_string();
        let measure_close = projected
            .xml
            .find("</measure>")
            .expect("measure close for duplicate-occurrence regression");
        let duplicated = format!(
            "{}\n{}{}",
            &projected.xml[..measure_close],
            duplicate_xml,
            &projected.xml[measure_close..]
        );
        assert!(
            verify_canonical_final_chord_transaction_local_output_alpha(
                &duplicated,
                &canonical,
                &voice_plan,
                Some(&render_omissions),
                &projected.receipt,
                bound.plan.source_fingerprint,
                bound.plan.source_len,
            )
            .is_err(),
            "an additional unclaimed pitched occurrence must remain blocking"
        );
    }


    #[test]
    fn staccato_persistence_receipt_advances_render_duration_without_relaxing_master_binding() {
        let anchor = r#"<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>"#;
        let member = r#"<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>"#;
        let target_rest = r#"<note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>"#;
        let trailing_rest_a = r#"<note><rest/><duration>192</duration><voice>1</voice><type>half</type><staff>1</staff></note>"#;
        let trailing_rest_b = r#"<note><rest/><duration>144</duration><voice>1</voice><type>quarter</type><dot/><staff>1</staff></note>"#;
        let source_xml = final_chord_score(&format!(
            "{anchor}\n{member}\n{target_rest}\n{trailing_rest_a}\n{trailing_rest_b}"
        ))
        .replace("<divisions>4</divisions>", "<divisions>96</divisions>");
        let anchor_start = source_xml.find(anchor).expect("anchor range");
        let anchor_end = anchor_start + anchor.len();
        let member_start = source_xml.find(member).expect("member range");
        let member_end = member_start + member.len();
        let rest_start = source_xml.find(target_rest).expect("rest range");
        let rest_end = rest_start + target_rest.len();

        let anchor_locator = FinalChordCanonicalSourceLocator {
            duration: 24,
            pitch_identity: "midi:60".to_string(),
            pitch_midi: 60,
            ..baseline_final_chord_canonical_source_locator()
        };
        let member_locator = FinalChordCanonicalSourceLocator {
            fragment_id: 2,
            source_event_id: 2,
            attack_id: 2,
            raw_note_index: 1,
            duration: 24,
            chord_syntax: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let canonical = CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(vec![
                anchor_locator.clone(),
                member_locator.clone(),
            ]),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(vec![FinalChordCanonicalActionRow {
                fragment_id: 2,
                source_event_id: 2,
                attack_id: 2,
                host_source_event_id: Some(1),
                locator: member_locator,
                representation_role: FinalChordRepresentationRole::RetainedChordMember,
                kind: FinalChordCanonicalActionKind::RetainedChordMember {
                    canonical_duration: 24,
                },
            }]),
            hard_same_edges: std::sync::Arc::from(vec![FinalChordCanonicalHardSameEdge {
                left_fragment_id: 2,
                right_fragment_id: 1,
                presentation_staff: 1,
            }]),
            measures: std::sync::Arc::from(vec![FinalChordCanonicalMeasureInfo {
                measure: 1,
                divisions: 96,
                expected_duration: 384,
            }]),
            semantic_fingerprint_fnv64: 1,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        };
        let voice_plan = FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: std::collections::BTreeMap::new(),
            authority_bound: true,
            source_to_channel: [(1, 1), (2, 1)].into_iter().collect(),
            source_to_label: [(1, "1".to_string()), (2, "1".to_string())]
                .into_iter()
                .collect(),
            label_to_role: [(
                "1".to_string(),
                SerializedVoicePresentationRole::UpperPrimary,
            )]
            .into_iter()
            .collect(),
            channel_to_label: [(1, "1".to_string())].into_iter().collect(),
            channel_to_role: [(1, SerializedVoicePresentationRole::UpperPrimary)]
                .into_iter()
                .collect(),
            semantic_fingerprint_fnv64: 1,
        };
        let render_omissions =
            std::collections::BTreeSet::<RenderPrunedFinalChordOmissionKey>::new();

        bind_canonical_final_chord_encoding_plan_for_render_projection(
            &source_xml,
            &canonical,
            &voice_plan,
            &render_omissions,
        )
        .expect("pre-write canonical render binding");

        let anchor_after =
            crate::structural_write_authority::set_pitched_note_duration_type_add_staccato_authorized(
                anchor, 48,
            )
            .expect("anchor retime");
        let member_after =
            crate::structural_write_authority::ensure_atomic_move_child_text_value_authorized(
                member, "duration", "48",
            );
        let member_after =
            crate::structural_write_authority::sync_atomic_move_note_type_and_dots_authorized(
                &member_after,
            );
        let (persisted_xml, receipt) =
            crate::structural_write_authority::apply_bound_staccato_persistence_batch_authorized(
                &source_xml,
                vec![
                    (anchor_start, anchor_end, anchor_after),
                    (member_start, member_end, member_after),
                    (rest_start, rest_end, String::new()),
                ],
            )
            .expect("authorized staccato persistence batch");

        assert!(
            bind_canonical_final_chord_encoding_plan_for_render_projection(
                &persisted_xml,
                &canonical,
                &voice_plan,
                &render_omissions,
            )
            .is_err(),
            "the representation-duration drift must fail before receipt-bound continuation"
        );

        let advanced = advance_final_chord_render_occurrence_through_staccato_persistence(
            &canonical,
            &voice_plan,
            &render_omissions,
            &receipt,
            &persisted_xml,
        )
        .expect("receipt-bound staccato occurrence continuation");

        assert!(
            bind_canonical_final_chord_encoding_plan(&persisted_xml, &advanced, &voice_plan)
                .is_err(),
            "master/normal binding must never consume render-only duration continuation"
        );
        let alpha = audit_musicxml_voice_label_binding_alpha_for_render_projection(
            &persisted_xml,
            &advanced,
            &voice_plan,
            &render_omissions,
        )
        .expect("render Output-Alpha after receipt-bound duration continuation");
        assert!(alpha.green());

        let bound = bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
            &persisted_xml,
            &advanced,
            &voice_plan,
            Some(&render_omissions),
        )
        .expect("advanced render binding");
        let projected =
            project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha(
                &persisted_xml,
                &advanced,
                &bound,
            )
            .expect("receipt-aware render projection");
        let projected_measures =
            final_chord_parse_binding_measures(&projected.xml).expect("parse projected render");
        let chord = projected_measures[0]
            .notes
            .iter()
            .find(|note| note.chord_member && note.pitch_midi == Some(64))
            .expect("retained chord member");
        let anchor = projected_measures[0]
            .notes
            .iter()
            .find(|note| !note.chord_member && note.pitch_midi == Some(60))
            .expect("staccato anchor");
        assert_eq!(anchor.duration, 48);
        assert_eq!(chord.duration, 48);
    }

    #[test]
    fn transaction_output_alpha_carries_tied_one_to_many_occurrences() {
        let render_xml = final_chord_score(
            fixture_xml!(r#"      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <tie type="start"/>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
        <notations><tied type="start"/></notations>
      </note>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <tie type="stop"/>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
        <notations><tied type="stop"/></notations>
      </note>
"#, FIXTURE_NOTE_395A683D76, r#"
"#, FIXTURE_NOTE_631875EE3D),
        );
        let first_anchor = FinalChordCanonicalSourceLocator {
            pitch_identity: "midi:60".to_string(),
            pitch_midi: 60,
            tie_start: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let second_anchor = FinalChordCanonicalSourceLocator {
            fragment_id: 3,
            source_event_id: 3,
            attack_id: 3,
            raw_note_index: 1,
            onset: 4,
            pitch_identity: "midi:60".to_string(),
            pitch_midi: 60,
            tie_stop: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let tied_source = FinalChordCanonicalSourceLocator {
            fragment_id: 2,
            source_event_id: 2,
            attack_id: 2,
            raw_note_index: 2,
            onset: 4,
            duration: 8,
            chord_syntax: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let canonical = CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(vec![
                first_anchor,
                tied_source.clone(),
                second_anchor,
            ]),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(vec![FinalChordCanonicalActionRow {
                fragment_id: 2,
                source_event_id: 2,
                attack_id: 2,
                host_source_event_id: Some(3),
                locator: tied_source,
                representation_role: FinalChordRepresentationRole::TiedRematerializedFragment,
                kind: FinalChordCanonicalActionKind::TiedAnchorChain {
                    fragments: vec![
                        FinalChordCanonicalFragmentTarget {
                            anchor_fragment_id: 1,
                            target_onset: 0,
                            duration: 4,
                            tie_start: true,
                            tie_stop: false,
                        },
                        FinalChordCanonicalFragmentTarget {
                            anchor_fragment_id: 3,
                            target_onset: 4,
                            duration: 4,
                            tie_start: false,
                            tie_stop: true,
                        },
                    ],
                },
            }]),
            hard_same_edges: std::sync::Arc::from(Vec::<FinalChordCanonicalHardSameEdge>::new()),
            measures: std::sync::Arc::from(vec![FinalChordCanonicalMeasureInfo {
                measure: 1,
                divisions: 4,
                expected_duration: 16,
            }]),
            semantic_fingerprint_fnv64: 41,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        };
        let voice_plan = FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: std::collections::BTreeMap::new(),
            authority_bound: true,
            source_to_channel: [(1, 1), (2, 1), (3, 1)].into_iter().collect(),
            source_to_label: [
                (1, "1".to_string()),
                (2, "1".to_string()),
                (3, "1".to_string()),
            ]
            .into_iter()
            .collect(),
            label_to_role: [(
                "1".to_string(),
                SerializedVoicePresentationRole::UpperPrimary,
            )]
            .into_iter()
            .collect(),
            channel_to_label: [(1, "1".to_string())].into_iter().collect(),
            channel_to_role: [(1, SerializedVoicePresentationRole::UpperPrimary)]
                .into_iter()
                .collect(),
            semantic_fingerprint_fnv64: 41,
        };

        let alpha = audit_musicxml_voice_label_binding_alpha(&render_xml, &canonical, &voice_plan)
            .expect("transaction-local Output-Alpha must carry one canonical owner to two tied output occurrences");
        assert!(alpha.green());
        assert_eq!(alpha.paired_binding_failures(), 0);
        assert_eq!(alpha.stale_binding_false_accepts(), 0);
    }

    #[test]
    fn transaction_output_alpha_carries_auxiliary_lane_occurrence_across_label_length_change() {
        let render_xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_64DC3197A6, r#"
"#, FIXTURE_NOTE_D42565F8C7, r#"
"#, FIXTURE_NOTE_F714A49732),
        );
        let anchor = FinalChordCanonicalSourceLocator {
            pitch_identity: "midi:60".to_string(),
            pitch_midi: 60,
            ..baseline_final_chord_canonical_source_locator()
        };
        let auxiliary = FinalChordCanonicalSourceLocator {
            fragment_id: 2,
            source_event_id: 2,
            attack_id: 2,
            raw_note_index: 1,
            chord_syntax: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let canonical = CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(vec![anchor, auxiliary.clone()]),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(vec![FinalChordCanonicalActionRow {
                fragment_id: 2,
                source_event_id: 2,
                attack_id: 2,
                host_source_event_id: None,
                locator: auxiliary,
                representation_role: FinalChordRepresentationRole::IndependentAuxiliaryFragment,
                kind: FinalChordCanonicalActionKind::AuxiliaryLane {
                    serialization_run_id: 1,
                    canonical_duration: 4,
                },
            }]),
            hard_same_edges: std::sync::Arc::from(Vec::<FinalChordCanonicalHardSameEdge>::new()),
            measures: std::sync::Arc::from(vec![FinalChordCanonicalMeasureInfo {
                measure: 1,
                divisions: 4,
                expected_duration: 16,
            }]),
            semantic_fingerprint_fnv64: 42,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        };
        let voice_plan = FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: std::collections::BTreeMap::new(),
            authority_bound: true,
            source_to_channel: [(1, 1), (2, 2)].into_iter().collect(),
            source_to_label: [(1, "1".to_string()), (2, "17".to_string())]
                .into_iter()
                .collect(),
            label_to_role: [
                ("1".to_string(), SerializedVoicePresentationRole::UpperPrimary),
                ("17".to_string(), SerializedVoicePresentationRole::UpperAuxiliary),
            ]
            .into_iter()
            .collect(),
            channel_to_label: [(1, "1".to_string()), (2, "17".to_string())]
                .into_iter()
                .collect(),
            channel_to_role: [
                (1, SerializedVoicePresentationRole::UpperPrimary),
                (2, SerializedVoicePresentationRole::UpperAuxiliary),
            ]
            .into_iter()
            .collect(),
            semantic_fingerprint_fnv64: 42,
        };

        let alpha = audit_musicxml_voice_label_binding_alpha(&render_xml, &canonical, &voice_plan)
            .expect("transaction-local Output-Alpha must carry auxiliary occurrence across 1<->17 label permutation");
        assert!(alpha.green());
        assert_eq!(alpha.cases(), 1);
        assert_eq!(alpha.paired_binding_failures(), 0);
        assert_eq!(alpha.stale_binding_false_accepts(), 0);
    }

    fn render_prune_binding_fixture(
    ) -> (
        String,
        CanonicalFinalChordStructuralPlan,
        FrozenSerializedVoicePlan,
        std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
    ) {
        let render_xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_64DC3197A6, r#"
"#, FIXTURE_NOTE_D42565F8C7, r#"
"#, FIXTURE_NOTE_F714A49732),
        );
        let anchor = FinalChordCanonicalSourceLocator {
            pitch_identity: "midi:60".to_string(),
            pitch_midi: 60,
            ..baseline_final_chord_canonical_source_locator()
        };
        let chord = FinalChordCanonicalSourceLocator {
            fragment_id: 2,
            source_event_id: 2,
            attack_id: 2,
            raw_note_index: 1,
            chord_syntax: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let survivor_chord = FinalChordCanonicalSourceLocator {
            fragment_id: 3,
            source_event_id: 3,
            attack_id: 3,
            raw_note_index: 2,
            chord_syntax: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let canonical = CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(vec![
                anchor,
                chord.clone(),
                survivor_chord.clone(),
            ]),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(vec![
                FinalChordCanonicalActionRow {
                    fragment_id: 2,
                    source_event_id: 2,
                    attack_id: 2,
                    host_source_event_id: Some(1),
                    locator: chord,
                    representation_role: FinalChordRepresentationRole::RetainedChordMember,
                    kind: FinalChordCanonicalActionKind::RetainedChordMember {
                        canonical_duration: 4,
                    },
                },
                FinalChordCanonicalActionRow {
                    fragment_id: 3,
                    source_event_id: 3,
                    attack_id: 3,
                    host_source_event_id: Some(1),
                    locator: survivor_chord,
                    representation_role: FinalChordRepresentationRole::RetainedChordMember,
                    kind: FinalChordCanonicalActionKind::RetainedChordMember {
                        canonical_duration: 4,
                    },
                },
            ]),
            hard_same_edges: std::sync::Arc::from(vec![
                FinalChordCanonicalHardSameEdge {
                    left_fragment_id: 2,
                    right_fragment_id: 1,
                    presentation_staff: 1,
                },
                FinalChordCanonicalHardSameEdge {
                    left_fragment_id: 3,
                    right_fragment_id: 1,
                    presentation_staff: 1,
                },
            ]),
            measures: std::sync::Arc::from(vec![FinalChordCanonicalMeasureInfo {
                measure: 1,
                divisions: 4,
                expected_duration: 16,
            }]),
            semantic_fingerprint_fnv64: 1,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        };
        let voice_plan = FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: std::collections::BTreeMap::new(),
            authority_bound: true,
            source_to_channel: [(1, 1), (2, 1), (3, 1)].into_iter().collect(),
            source_to_label: [
                (1, "1".to_string()),
                (2, "1".to_string()),
                (3, "1".to_string()),
            ]
            .into_iter()
            .collect(),
            label_to_role: [(
                "1".to_string(),
                SerializedVoicePresentationRole::UpperPrimary,
            )]
            .into_iter()
            .collect(),
            channel_to_label: [(1, "1".to_string())].into_iter().collect(),
            channel_to_role: [(1, SerializedVoicePresentationRole::UpperPrimary)]
                .into_iter()
                .collect(),
            semantic_fingerprint_fnv64: 1,
        };
        let omissions = [2].into_iter().collect();
        (render_xml, canonical, voice_plan, omissions)
    }

    #[test]
    fn render_pruned_retained_chord_omission_is_projection_only() {
        let (render_xml, canonical, voice_plan, omissions) = render_prune_binding_fixture();

        assert!(
            bind_canonical_final_chord_encoding_plan(&render_xml, &canonical, &voice_plan)
                .is_err()
        );

        let plan = bind_canonical_final_chord_encoding_plan_for_render_projection(
            &render_xml,
            &canonical,
            &voice_plan,
            &omissions,
        )
        .expect("certified render omission must bind");
        let projected =
            project_final_chord_encoding_plan(&render_xml, &plan).0.expect("render projection");
        assert_eq!(projected, render_xml);
    }

    #[test]
    fn render_pruned_retained_chord_omission_requires_exact_certificate() {
        let (render_xml, canonical, voice_plan, mut omissions) = render_prune_binding_fixture();
        omissions.clear();
        omissions.insert(999);

        assert!(
            bind_canonical_final_chord_encoding_plan_for_render_projection(
                &render_xml,
                &canonical,
                &voice_plan,
                &omissions,
            )
            .is_err()
        );
    }

    #[test]
    fn render_pruned_auxiliary_lane_omission_is_projection_only() {
        let (render_xml, mut canonical, voice_plan, omissions) = render_prune_binding_fixture();
        let chord = canonical
            .source_locators
            .iter()
            .find(|locator| locator.fragment_id == 2)
            .cloned()
            .expect("auxiliary fixture chord locator");
        canonical.actions = std::sync::Arc::from(vec![FinalChordCanonicalActionRow {
            fragment_id: 2,
            source_event_id: 2,
            attack_id: 2,
            host_source_event_id: Some(1),
            locator: chord,
            representation_role: FinalChordRepresentationRole::IndependentAuxiliaryFragment,
            kind: FinalChordCanonicalActionKind::AuxiliaryLane {
                serialization_run_id: 1,
                canonical_duration: 4,
            },
        }]);

        assert!(
            bind_canonical_final_chord_encoding_plan(&render_xml, &canonical, &voice_plan).is_err(),
            "auxiliary source omission must remain render-projection-only"
        );

        let plan = bind_canonical_final_chord_encoding_plan_for_render_projection(
            &render_xml,
            &canonical,
            &voice_plan,
            &omissions,
        )
        .expect("certified auxiliary render omission must bind");
        let projected =
            project_final_chord_encoding_plan(&render_xml, &plan).0.expect("render projection");
        assert_eq!(projected, render_xml);
        let alpha = audit_musicxml_voice_label_binding_alpha_for_render_projection(
            &render_xml,
            &canonical,
            &voice_plan,
            &omissions,
        )
        .expect("transaction-local Output-Alpha must preserve certified auxiliary omission");
        assert!(alpha.green());
    }

    #[test]
    fn render_pruned_auxiliary_lane_omission_requires_exact_certificate() {
        let (render_xml, mut canonical, voice_plan, mut omissions) =
            render_prune_binding_fixture();
        let chord = canonical
            .source_locators
            .iter()
            .find(|locator| locator.fragment_id == 2)
            .cloned()
            .expect("auxiliary fixture chord locator");
        canonical.actions = std::sync::Arc::from(vec![FinalChordCanonicalActionRow {
            fragment_id: 2,
            source_event_id: 2,
            attack_id: 2,
            host_source_event_id: Some(1),
            locator: chord,
            representation_role: FinalChordRepresentationRole::IndependentAuxiliaryFragment,
            kind: FinalChordCanonicalActionKind::AuxiliaryLane {
                serialization_run_id: 1,
                canonical_duration: 4,
            },
        }]);
        omissions.clear();
        omissions.insert(999);

        assert!(
            bind_canonical_final_chord_encoding_plan_for_render_projection(
                &render_xml,
                &canonical,
                &voice_plan,
                &omissions,
            )
            .is_err()
        );
    }

    fn render_pruned_tied_chain_fixture(
        partial_hidden_fragment: bool,
    ) -> (
        String,
        CanonicalFinalChordStructuralPlan,
        FrozenSerializedVoicePlan,
        std::collections::BTreeSet<RenderPrunedFinalChordOmissionKey>,
    ) {
        let hidden_partial = if partial_hidden_fragment {
            r#"      <note print-object="no" print-spacing="no">
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>4</duration>
        <tie type="start"/>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
        <notations><tied type="start"/></notations>
      </note>
"#
        } else {
            ""
        };
        let render_xml = final_chord_score(&format!(
            r#"      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>
{hidden_partial}      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>
      <note>
        <rest/>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>"#,
        ));

        let first_anchor = FinalChordCanonicalSourceLocator {
            pitch_identity: "midi:60".to_string(),
            pitch_midi: 60,
            ..baseline_final_chord_canonical_source_locator()
        };
        let tied_source = FinalChordCanonicalSourceLocator {
            fragment_id: 2,
            source_event_id: 2,
            attack_id: 2,
            raw_note_index: 2,
            onset: 4,
            duration: 8,
            chord_syntax: true,
            ..baseline_final_chord_canonical_source_locator()
        };
        let second_anchor = FinalChordCanonicalSourceLocator {
            fragment_id: 3,
            source_event_id: 3,
            attack_id: 3,
            raw_note_index: 1,
            onset: 4,
            pitch_identity: "midi:62".to_string(),
            pitch_midi: 62,
            ..baseline_final_chord_canonical_source_locator()
        };
        let fragments = vec![
            FinalChordCanonicalFragmentTarget {
                anchor_fragment_id: 1,
                target_onset: 0,
                duration: 4,
                tie_start: true,
                tie_stop: false,
            },
            FinalChordCanonicalFragmentTarget {
                anchor_fragment_id: 3,
                target_onset: 4,
                duration: 4,
                tie_start: false,
                tie_stop: true,
            },
        ];
        let canonical = CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(vec![
                first_anchor,
                tied_source.clone(),
                second_anchor,
            ]),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(vec![FinalChordCanonicalActionRow {
                fragment_id: 2,
                source_event_id: 2,
                attack_id: 2,
                host_source_event_id: Some(3),
                locator: tied_source,
                representation_role: FinalChordRepresentationRole::TiedRematerializedFragment,
                kind: FinalChordCanonicalActionKind::TiedAnchorChain { fragments },
            }]),
            hard_same_edges: std::sync::Arc::from(vec![
                FinalChordCanonicalHardSameEdge {
                    left_fragment_id: 2,
                    right_fragment_id: 1,
                    presentation_staff: 1,
                },
                FinalChordCanonicalHardSameEdge {
                    left_fragment_id: 2,
                    right_fragment_id: 3,
                    presentation_staff: 1,
                },
            ]),
            measures: std::sync::Arc::from(vec![FinalChordCanonicalMeasureInfo {
                measure: 1,
                divisions: 4,
                expected_duration: 16,
            }]),
            semantic_fingerprint_fnv64: 2,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        };
        let voice_plan = FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: std::collections::BTreeMap::new(),
            authority_bound: true,
            source_to_channel: [(1, 1), (2, 1), (3, 1)].into_iter().collect(),
            source_to_label: [
                (1, "1".to_string()),
                (2, "1".to_string()),
                (3, "1".to_string()),
            ]
            .into_iter()
            .collect(),
            label_to_role: [(
                "1".to_string(),
                SerializedVoicePresentationRole::UpperPrimary,
            )]
            .into_iter()
            .collect(),
            channel_to_label: [(1, "1".to_string())].into_iter().collect(),
            channel_to_role: [(1, SerializedVoicePresentationRole::UpperPrimary)]
                .into_iter()
                .collect(),
            semantic_fingerprint_fnv64: 2,
        };
        let omissions = [2].into_iter().collect();
        (render_xml, canonical, voice_plan, omissions)
    }

    #[test]
    fn render_pruned_tied_source_omission_is_projection_only_when_whole_chain_is_absent() {
        let (render_xml, canonical, voice_plan, omissions) =
            render_pruned_tied_chain_fixture(false);

        assert!(
            bind_canonical_final_chord_encoding_plan(&render_xml, &canonical, &voice_plan)
                .is_err()
        );

        let plan = bind_canonical_final_chord_encoding_plan_for_render_projection(
            &render_xml,
            &canonical,
            &voice_plan,
            &omissions,
        )
        .expect("certified pruned tied source must bind as absent render projection");
        let projected =
            project_final_chord_encoding_plan(&render_xml, &plan).0.expect("render projection");
        assert_eq!(projected, render_xml);
        let alpha = audit_musicxml_voice_label_binding_alpha_for_render_projection(
            &render_xml,
            &canonical,
            &voice_plan,
            &omissions,
        )
        .expect("transaction-local Output-Alpha must preserve whole certified tied omission");
        assert!(alpha.green());
    }

    #[test]
    fn render_pruned_tied_source_omission_rejects_partial_generated_chain() {
        let (render_xml, canonical, voice_plan, omissions) =
            render_pruned_tied_chain_fixture(true);

        assert!(
            bind_canonical_final_chord_encoding_plan_for_render_projection(
                &render_xml,
                &canonical,
                &voice_plan,
                &omissions,
            )
            .is_err()
        );
    }

    const FINAL_CHORD_TEST_TEMP_UPPER_AUX_VOICE: &str = "9001";

    fn final_chord_temp_upper_aux_voice_count(xml: &str) -> usize {
        let needle = format!("<voice>{FINAL_CHORD_TEST_TEMP_UPPER_AUX_VOICE}</voice>");
        xml.matches(needle.as_str()).count()
    }

    #[test]
    fn final_chord_auxiliary_multi_member_group_is_atomic_and_simultaneous() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_070FEEE235, r#"
"#, FIXTURE_NOTE_1C6F0511B4, r#"
      <note print-object="no">
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_F714A49732),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(final_chord_temp_upper_aux_voice_count(&out), 3);
        assert_eq!(out.matches("<backup>").count(), 1);
        assert_eq!(out.matches("<chord/>").count(), 1);
        assert_eq!(out.matches("<step>C</step>").count(), 1);
        assert_eq!(out.matches("<step>E</step>").count(), 1);
    }

    #[test]
    fn final_chord_auxiliary_six_member_group_advances_cursor_once() {
        let mut body = String::from(
            fixture_xml!(FIXTURE_NOTE_070FEEE235, r#"
"#),
        );
        for step in ["C", "D", "E", "F", "G", "A"] {
            body.push_str(&format!(
                r#"      <note print-object="no">
        <chord/>
        <pitch><step>{step}</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>
"#
            ));
        }
        body.push_str(
            fixture_xml!(FIXTURE_NOTE_F714A49732),
        );
        let xml = final_chord_score(&body);
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(out.matches("<chord/>").count(), 5);
        assert_eq!(out.matches("<backup>").count(), 1);
        assert_eq!(final_chord_temp_upper_aux_voice_count(&out), 7);
        for step in ["C", "D", "E", "F", "G", "A"] {
            let expected_pitch = format!("<step>{step}</step>");
            assert_eq!(out.matches(expected_pitch.as_str()).count(), 1);
        }
    }

    #[test]
    fn final_chord_rejected_auxiliary_lane_keeps_original_members() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_070FEEE235, r#"
"#, FIXTURE_NOTE_1C6F0511B4, r#"
"#, FIXTURE_NOTE_070FEEE235, r#"
      <note print-object="no">
        <chord/>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_631875EE3D),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        assert!(plan.edits.is_empty());
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(out, xml);
        assert_eq!(out.matches("<chord/>").count(), 2);
        assert_eq!(final_chord_temp_upper_aux_voice_count(&out), 0);
    }

    #[test]
    fn final_chord_visible_long_member_uses_existing_auxiliary_lane() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_64DC3197A6, r#"
"#, FIXTURE_NOTE_395A683D76, r#"
      <note>
        <pitch><step>D</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_631875EE3D),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let projection = project_final_chord_encoding_plan_with_provenance(&xml, &plan)
            .expect("materialize with provenance");
        let auxiliary_rows = projection
            .provenance
            .iter()
            .filter(|row| row.kind == FinalChordProjectionMaterializationKind::AuxiliaryLane)
            .collect::<Vec<_>>();
        assert_eq!(auxiliary_rows.len(), 1);
        assert_eq!(auxiliary_rows[0].fragment_ordinal, 0);
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(projection.xml, out);
        assert_eq!(out.matches("<step>E</step>").count(), 1);
        assert_eq!(out.matches("<chord/>").count(), 0);
        assert!(out.contains("<backup><duration>16</duration></backup>"));
        assert!(final_chord_temp_upper_aux_voice_count(&out) > 0);
        let second_plan = plan_final_chord_encoding(&out).expect("second plan");
        let twice = project_final_chord_encoding_plan(&out, &second_plan).0.expect("second");
        assert_eq!(twice, out);
    }

    #[test]
    fn final_chord_long_member_is_bounded_by_next_same_pitch_reattack() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_64DC3197A6, r#"
      <note print-object="no" print-spacing="no">
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_F9745462DA, r#"
"#, FIXTURE_NOTE_631875EE3D),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(out.matches("<step>E</step>").count(), 2);
        assert_eq!(out.matches("<chord/>").count(), 1);
        assert_eq!(out.matches("<duration>4</duration>").count(), 3);
        assert_eq!(final_chord_temp_upper_aux_voice_count(&out), 0);
        let second_plan = plan_final_chord_encoding(&out).expect("second plan");
        let twice = project_final_chord_encoding_plan(&out, &second_plan).0.expect("second");
        assert_eq!(twice, out);
    }

    #[test]
    fn final_chord_same_anchor_same_pitch_peers_share_existing_auxiliary_lane() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_64DC3197A6, r#"
"#, FIXTURE_NOTE_395A683D76, r#"
      <note print-object="no" print-spacing="no">
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_F714A49732),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let projection = project_final_chord_encoding_plan_with_provenance(&xml, &plan)
            .expect("materialize with provenance");
        let auxiliary_rows = projection
            .provenance
            .iter()
            .filter(|row| row.kind == FinalChordProjectionMaterializationKind::AuxiliaryLane)
            .count();
        assert_eq!(auxiliary_rows, 2);
        let out = projection.xml;
        assert_eq!(out.matches("<step>E</step>").count(), 2);
        assert_eq!(out.matches("<chord/>").count(), 1);
        // The helper counts both generated note voices plus the lane-tail <forward>
        // transport that carries the same technical voice to the measure end.
        assert_eq!(final_chord_temp_upper_aux_voice_count(&out), 3);
        let second_plan = plan_final_chord_encoding(&out).expect("second plan");
        let twice = project_final_chord_encoding_plan(&out, &second_plan).0.expect("second");
        assert_eq!(twice, out);
    }

    #[test]
    fn final_chord_same_onset_same_pitch_owner_remains_fail_closed() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_F9745462DA, r#"
      <backup><duration>4</duration></backup>
      <note>
        <pitch><step>C</step><octave>3</octave></pitch>
        <duration>4</duration>
        <voice>2</voice>
        <type>eighth</type>
        <staff>2</staff>
        <beam number="1">continue</beam>
      </note>
"#, FIXTURE_NOTE_395A683D76, r#"
      <forward><duration>12</duration></forward>"#),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        assert!(plan.edits.is_empty());
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(out, xml);
    }

    #[test]
    fn final_chord_same_onset_tie_stop_does_not_block_auxiliary_lane() {
        let xml = final_chord_score(
            r#"      <note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><tie type="start"/><voice>1</voice><type>quarter</type><staff>1</staff><notations><tied type="start"/></notations></note>
      <note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><tie type="stop"/><voice>1</voice><type>quarter</type><staff>1</staff><notations><tied type="stop"/></notations></note>
      <backup><duration>4</duration></backup>
      <note><pitch><step>B</step><octave>2</octave></pitch><duration>4</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>
      <note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>8</duration><voice>1</voice><type>half</type><staff>1</staff></note>
      <note><rest/><duration>8</duration><voice>2</voice><type>half</type><staff>2</staff></note>"#,
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(out.matches("<chord/>").count(), 0);
        assert_eq!(final_chord_temp_upper_aux_voice_count(&out), 3);
        let second_plan = plan_final_chord_encoding(&out).expect("second plan");
        let twice = project_final_chord_encoding_plan(&out, &second_plan).0.expect("second");
        assert_eq!(twice, out);
    }

    #[test]
    fn final_chord_long_member_is_split_over_tied_anchor_chain() {
        let xml = final_chord_score(
            fixture_xml!(r#"      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <tie type="start"/>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>
      <note>
        <pitch><step>C</step><octave>4</octave></pitch>
        <duration>4</duration>
        <tie type="stop"/>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_395A683D76, r#"
"#, FIXTURE_NOTE_631875EE3D),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let projection = project_final_chord_encoding_plan_with_provenance(&xml, &plan)
            .expect("materialize with provenance");
        let tied_rows = projection
            .provenance
            .iter()
            .filter(|row| row.kind == FinalChordProjectionMaterializationKind::TiedAnchorChain)
            .collect::<Vec<_>>();
        assert_eq!(tied_rows.len(), 2);
        assert_eq!(tied_rows[0].source_note_start, tied_rows[1].source_note_start);
        assert_eq!(
            tied_rows
                .iter()
                .map(|row| row.fragment_ordinal)
                .collect::<std::collections::BTreeSet<_>>(),
            [0_usize, 1_usize].into_iter().collect()
        );
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(projection.xml, out);
        assert_eq!(out.matches("<step>E</step>").count(), 2);
        assert_eq!(out.matches("<duration>4</duration>").count(), 4);
        assert!(out.contains("<tied type=\"start\"/>"));
        assert!(out.contains("<tied type=\"stop\"/>"));
    }

    #[test]
    fn final_chord_projection_provenance_keeps_representation_collision_sources_distinct() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_64DC3197A6, r#"
      <backup><duration>4</duration></backup>
"#, FIXTURE_NOTE_64DC3197A6, r#"
      <forward><duration>12</duration></forward>"#),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let projection = project_final_chord_encoding_plan_with_provenance(&xml, &plan)
            .expect("collision-safe provenance");
        let pitched = projection
            .provenance
            .iter()
            .filter(|row| row.kind == FinalChordProjectionMaterializationKind::SurvivingSourceNote)
            .collect::<Vec<_>>();
        assert_eq!(pitched.len(), 2);
        assert_ne!(pitched[0].source_note_start, pitched[1].source_note_start);
        assert_ne!(pitched[0].projected_note_start, pitched[1].projected_note_start);
    }

    #[test]
    fn final_chord_orphan_after_forward_six_member_group_moves_atomically() {
        let mut body = String::from(
            fixture_xml!(FIXTURE_NOTE_070FEEE235, r#"
      <forward><duration>4</duration></forward>
"#),
        );
        for step in ["C", "D", "E", "F", "G", "A"] {
            body.push_str(&format!(
                r#"      <note print-object="no">
        <chord/>
        <pitch><step>{step}</step><octave>4</octave></pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <staff>1</staff>
      </note>
"#
            ));
        }
        body.push_str(
            fixture_xml!(r#"      <forward><duration>4</duration></forward>
"#, FIXTURE_NOTE_070FEEE235),
        );
        let xml = final_chord_score(&body);
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(final_chord_temp_upper_aux_voice_count(&out), 8);
        assert_eq!(out.matches("<chord/>").count(), 5);
        assert_eq!(out.matches("<backup>").count(), 1);
        let expected_forward = format!(
            "<forward><duration>8</duration><voice>{FINAL_CHORD_TEST_TEMP_UPPER_AUX_VOICE}</voice><staff>1</staff></forward>"
        );
        assert!(out.contains(expected_forward.as_str()));
        for step in ["C", "D", "E", "F", "G", "A"] {
            let expected_pitch = format!("<step>{step}</step>");
            assert_eq!(out.matches(expected_pitch.as_str()).count(), 1);
        }
    }

    #[test]
    fn final_chord_tied_split_stays_unchanged_across_same_pitch_reattack() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_070FEEE235, r#"
"#, FIXTURE_NOTE_F9745462DA, r#"
"#, FIXTURE_NOTE_631875EE3D, r#"
      <backup><duration>16</duration></backup>
      <note>
        <pitch><step>C</step><octave>3</octave></pitch>
        <duration>4</duration>
        <tie type="start"/>
        <voice>2</voice>
        <type>eighth</type>
        <staff>2</staff>
        <beam number="1">end</beam>
      </note>
      <note>
        <pitch><step>C</step><octave>3</octave></pitch>
        <duration>4</duration>
        <tie type="stop"/>
        <voice>2</voice>
        <type>quarter</type>
        <staff>2</staff>
      </note>
"#, FIXTURE_NOTE_395A683D76, r#"
      <note>
        <rest/>
        <duration>8</duration>
        <voice>2</voice>
        <type>half</type>
        <staff>2</staff>
      </note>"#),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        assert!(plan.edits.is_empty());
        let out = project_final_chord_encoding_plan(&xml, &plan).0.expect("materialize");
        assert_eq!(out, xml);
        assert_eq!(out.matches("<step>E</step>").count(), 2);
    }

    #[test]
    fn final_chord_plan_is_source_bound_and_idempotent() {
        let xml = final_chord_score(
            fixture_xml!(FIXTURE_NOTE_070FEEE235, r#"
"#, FIXTURE_NOTE_1C6F0511B4, r#"
      <note print-object="no">
        <chord/>
        <pitch><step>E</step><octave>4</octave></pitch>
        <duration>8</duration>
        <voice>1</voice>
        <type>half</type>
        <staff>1</staff>
      </note>
"#, FIXTURE_NOTE_F714A49732),
        );
        let plan = plan_final_chord_encoding(&xml).expect("plan");
        assert!(project_final_chord_encoding_plan(&(xml.clone() + "\n"), &plan).0.is_err());
        let once = project_final_chord_encoding_plan(&xml, &plan).0.expect("first");
        let second_plan = plan_final_chord_encoding(&once).expect("second plan");
        let twice = project_final_chord_encoding_plan(&once, &second_plan).0.expect("second");
        assert_eq!(twice, once);
    }


    fn primary_beam_voice_partition_binding_fixture() -> (
        String,
        CanonicalFinalChordStructuralPlan,
        FrozenSerializedVoicePlan,
        FinalPdfSourceOccurrenceContinuation,
        Vec<PrimaryBeamVoicePartitionRepairCandidate>,
    ) {
        let xml = final_chord_score(
            r#"      <note>
        <pitch><step>C</step><octave>3</octave></pitch>
        <duration>4</duration>
        <voice>2</voice>
        <type>eighth</type>
        <staff>2</staff>
        <beam number="1">begin</beam>
      </note>
      <note>
        <pitch><step>D</step><octave>3</octave></pitch>
        <duration>4</duration>
        <voice>4</voice>
        <type>eighth</type>
        <staff>2</staff>
        <beam number="1">continue</beam>
      </note>
      <note>
        <pitch><step>E</step><octave>3</octave></pitch>
        <duration>4</duration>
        <voice>2</voice>
        <type>eighth</type>
        <staff>2</staff>
        <beam number="1">end</beam>
      </note>
      <forward><duration>4</duration></forward>"#,
        );
        let source_locators = vec![
            FinalChordCanonicalSourceLocator {
                pitch_identity: "midi:48".to_string(),
                pitch_midi: 48,
                staff: "2".to_string(),
                ..baseline_final_chord_canonical_source_locator()
            },
            FinalChordCanonicalSourceLocator {
                fragment_id: 2,
                source_event_id: 2,
                attack_id: 2,
                raw_note_index: 1,
                onset: 4,
                pitch_identity: "midi:50".to_string(),
                pitch_midi: 50,
                staff: "2".to_string(),
                ..baseline_final_chord_canonical_source_locator()
            },
            FinalChordCanonicalSourceLocator {
                fragment_id: 3,
                source_event_id: 3,
                attack_id: 3,
                raw_note_index: 2,
                onset: 8,
                pitch_identity: "midi:52".to_string(),
                pitch_midi: 52,
                staff: "2".to_string(),
                ..baseline_final_chord_canonical_source_locator()
            },
        ];
        let canonical = CanonicalFinalChordStructuralPlan {
            bridge_voice_guard: None,
            rest_bridge_main_state: None,
            rest_bridge_extension_course: None,
            rest_bridge_withdrawal: None,
            authority_bound: true,
            original_respelling_input: None,
            original_beam_voice_groups: Vec::new(),
            written_projection: None,
            source_locators: std::sync::Arc::from(source_locators),
            render_occurrence_witnesses: std::sync::Arc::from(
                Vec::<FinalChordRenderOccurrenceWitness>::new(),
            ),
            actions: std::sync::Arc::from(Vec::<FinalChordCanonicalActionRow>::new()),
            hard_same_edges: std::sync::Arc::from(Vec::<FinalChordCanonicalHardSameEdge>::new()),
            measures: std::sync::Arc::from(vec![FinalChordCanonicalMeasureInfo {
                measure: 1,
                divisions: 4,
                expected_duration: 16,
            }]),
            semantic_fingerprint_fnv64: 73,
            unresolved_continuity_decisions: 0,
            unresolved_fragment_roles: 0,
            unresolved_serialization_runs: 0,
            unresolved_presentation_roles: 0,
            unresolved_constraint_edges: 0,
        };
        let voice_plan = FrozenSerializedVoicePlan {
            canonical_role_allocation: None,
            source_to_role: [
                (1, SerializedVoicePresentationRole::LowerPrimary),
                (2, SerializedVoicePresentationRole::LowerSecondary),
                (3, SerializedVoicePresentationRole::LowerPrimary),
            ].into_iter().collect(),
            authority_bound: true,
            source_to_channel: [(1, 2), (2, 4), (3, 2)].into_iter().collect(),
            source_to_label: [
                (1, "2".to_string()),
                (2, "4".to_string()),
                (3, "2".to_string()),
            ]
            .into_iter()
            .collect(),
            label_to_role: [
                ("2".to_string(), SerializedVoicePresentationRole::LowerPrimary),
                ("4".to_string(), SerializedVoicePresentationRole::LowerSecondary),
            ]
            .into_iter()
            .collect(),
            channel_to_label: [(2, "2".to_string()), (4, "4".to_string())]
                .into_iter()
                .collect(),
            channel_to_role: [
                (2, SerializedVoicePresentationRole::LowerPrimary),
                (4, SerializedVoicePresentationRole::LowerSecondary),
            ]
            .into_iter()
            .collect(),
            semantic_fingerprint_fnv64: 73,
        };
        let render_omissions = std::collections::BTreeSet::new();
        let projected = project_canonical_final_chord_render_occurrence_continuation(
            &xml,
            &canonical,
            &voice_plan,
            &render_omissions,
        )
        .expect("fixture projection");
        let continuation = FinalPdfSourceOccurrenceContinuation::from_original_transport(projected.xml().to_string(), projected.projected.receipt.clone()).unwrap();
        let candidates = vec![PrimaryBeamVoicePartitionRepairCandidate::new(
            "metric-a".to_string(),
            vec![(1, 1), (2, 2), (3, 3)],
            1,
            2,
            vec!["begin".to_string(), "continue".to_string(), "end".to_string()],
            (1, 1),
            2,
        )];
        (
            continuation.xml().to_string(),
            canonical,
            voice_plan,
            continuation,
            candidates,
        )
    }

    #[test]
    fn primary_beam_voice_partition_binding_rebinds_only_causally_selected_voice_elements() {
        let (xml, canonical, voice_plan, continuation, candidates) =
            primary_beam_voice_partition_binding_fixture();
        let safety = ["metric-a".to_string()]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        let fragments = [("metric-a".to_string(), 2usize)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>();
        let render_omissions = std::collections::BTreeSet::new();
        let plan = bind_primary_beam_voice_partition_plan(
            &xml,
            PrimaryBeamVoicePartitionBindingContext {
                source_continuation: &continuation,
                voice_plan: &voice_plan,
                candidates: &candidates,
                classification_xml: &xml,
                renderer_visible_fragments_by_metric_line_id: &fragments,
                materialization_safety_proven_metric_line_ids: &safety,
            },
        )
        .expect("bound scoped voice rebinding");

        assert_eq!(plan.scope_candidates(), 1);
        assert_eq!(plan.changed_voice_elements(), 1);
        assert_eq!(plan.repaired_voice_plan().label_for_source(2), Some("2"));

        let rebound =
            rebind_primary_beam_voice_partition_plan_to_current_xml(&xml, &continuation, &plan)
                .expect("exact source/attack late rebind");
        assert_eq!(rebound.changed_voice_elements(), 1);
        assert_eq!(rebound.edits()[0].expected_old_label.as_str(), "4");
        assert_eq!(rebound.edits()[0].target_label(), "2");

        let mut candidate = xml.clone();
        for edit in rebound.edits().iter().rev() {
            candidate.replace_range(
                edit.voice_text_start()..edit.voice_text_end(),
                edit.target_label(),
            );
        }
        assert_eq!(candidate, xml.replacen("<voice>4</voice>", "<voice>2</voice>", 1));
        let transported =
            transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
                &continuation,
                &xml,
                &candidate,
                &canonical,
                &rebound,
                Some(&render_omissions),
            )
            .expect("voice-only continuation");
        assert_eq!(transported.xml(), candidate);
    }

    #[test]
    fn primary_beam_voice_partition_independent_candidate_duplication_preserves_existing_decision() {
        let (xml, _canonical, voice_plan, continuation, mut candidates) =
            primary_beam_voice_partition_binding_fixture();
        let safety = ["metric-a".to_string()]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        let fragments = [("metric-a".to_string(), 2usize)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>();
        let baseline = bind_primary_beam_voice_partition_plan(
            &xml,
            PrimaryBeamVoicePartitionBindingContext {
                source_continuation: &continuation,
                voice_plan: &voice_plan,
                candidates: &candidates,
                classification_xml: &xml,
                renderer_visible_fragments_by_metric_line_id: &fragments,
                materialization_safety_proven_metric_line_ids: &safety,
            },
        )
        .expect("baseline selected candidate");

        candidates.push(PrimaryBeamVoicePartitionRepairCandidate::new(
            "metric-z".to_string(),
            vec![(101, 101), (102, 102)],
            9,
            1,
            vec!["begin".to_string(), "end".to_string()],
            (101, 101),
            1,
        ));
        let duplicated_population = bind_primary_beam_voice_partition_plan(
            &xml,
            PrimaryBeamVoicePartitionBindingContext {
                source_continuation: &continuation,
                voice_plan: &voice_plan,
                candidates: &candidates,
                classification_xml: &xml,
                renderer_visible_fragments_by_metric_line_id: &fragments,
                materialization_safety_proven_metric_line_ids: &safety,
            },
        )
        .expect("independent non-selected candidate must not affect selected plan");

        assert_eq!(baseline.edits(), duplicated_population.edits());
        assert_eq!(baseline.scope_candidates(), duplicated_population.scope_candidates());
    }

    #[test]
    fn primary_beam_voice_partition_without_safety_proof_has_no_write_scope() {
        let (xml, _canonical, voice_plan, continuation, candidates) =
            primary_beam_voice_partition_binding_fixture();
        let no_safety = std::collections::BTreeSet::new();
        let fragments = [("metric-a".to_string(), 2usize)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>();
        let plan = bind_primary_beam_voice_partition_plan(
            &xml,
            PrimaryBeamVoicePartitionBindingContext {
                source_continuation: &continuation,
                voice_plan: &voice_plan,
                candidates: &candidates,
                classification_xml: &xml,
                renderer_visible_fragments_by_metric_line_id: &fragments,
                materialization_safety_proven_metric_line_ids: &no_safety,
            },
        )
        .expect("safety-blocked population remains a valid no-write plan");
        assert_eq!(plan.scope_candidates(), 0);
        assert_eq!(plan.changed_voice_elements(), 0);
    }

    #[test]
    fn primary_beam_voice_partition_owner_state_mismatch_is_not_selected() {
        let (xml, _canonical, voice_plan, continuation, mut candidates) =
            primary_beam_voice_partition_binding_fixture();
        candidates[0].canonical_primary_states =
            vec!["begin".to_string(), "begin".to_string(), "end".to_string()];
        let safety = ["metric-a".to_string()]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        let fragments = [("metric-a".to_string(), 2usize)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>();
        let plan = bind_primary_beam_voice_partition_plan(
            &xml,
            PrimaryBeamVoicePartitionBindingContext {
                source_continuation: &continuation,
                voice_plan: &voice_plan,
                candidates: &candidates,
                classification_xml: &xml,
                renderer_visible_fragments_by_metric_line_id: &fragments,
                materialization_safety_proven_metric_line_ids: &safety,
            },
        )
        .expect("owner mismatch remains fail-closed no-write");
        assert_eq!(plan.scope_candidates(), 0);
        assert_eq!(plan.changed_voice_elements(), 0);
    }

    #[test]
    fn primary_beam_voice_partition_renderer_preserved_candidate_is_not_selected() {
        let (xml, _canonical, voice_plan, continuation, candidates) =
            primary_beam_voice_partition_binding_fixture();
        let safety = ["metric-a".to_string()]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        let fragments = [("metric-a".to_string(), 1usize)]
            .into_iter()
            .collect::<std::collections::BTreeMap<_, _>>();
        let plan = bind_primary_beam_voice_partition_plan(
            &xml,
            PrimaryBeamVoicePartitionBindingContext {
                source_continuation: &continuation,
                voice_plan: &voice_plan,
                candidates: &candidates,
                classification_xml: &xml,
                renderer_visible_fragments_by_metric_line_id: &fragments,
                materialization_safety_proven_metric_line_ids: &safety,
            },
        )
        .expect("renderer-preserved population remains a valid no-write plan");
        assert_eq!(plan.scope_candidates(), 0);
        assert_eq!(plan.changed_voice_elements(), 0);
    }

    fn non_pitched_continuation_score(body: &str, measure: usize) -> String {
        format!(r#"<score-partwise><part id="P1"><measure number="{measure}"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>"#)
    }

    fn non_pitched_continuation_projection(xml: &str, rest_projection: bool) -> FinalChordProjectionResult {
        let plan = final_chord_finalize_bound_plan(xml, Vec::new(), Vec::new(), rest_projection).unwrap();
        // This runs the sole productive byte projector and its real receipt path.
        project_final_chord_encoding_plan_with_provenance(xml, &plan).unwrap()
    }

    #[test]
    fn non_pitched_continuation_preserves_distinct_equal_and_hidden_carriers() {
        let note = r#"<note print-object="no"><rest/><duration>24</duration><voice>73</voice><staff>1</staff></note>"#;
        let unpitched = r#"<note><unpitched><display-step>C</display-step><display-octave>4</display-octave></unpitched><duration>24</duration><voice>73</voice><staff>1</staff></note>"#;
        let xml = non_pitched_continuation_score(&(note.repeat(2) + unpitched), 509);
        let projection = non_pitched_continuation_projection(&xml, true);
        assert_eq!(projection.xml, xml);
        assert!(projection.provenance.is_empty());
        let rows = projection.non_pitched_continuations;
        assert_eq!(rows.len(), 3);
        assert_ne!(rows[0].source_note_start, rows[1].source_note_start);
        assert_ne!(rows[0].projected_start, rows[1].projected_start);
        assert!(rows.iter().all(|row| row.presentation_role.is_none()));
        for row in rows {
            assert_eq!(&xml[row.source_note_start..row.source_note_end],
                &projection.xml[row.projected_start..row.projected_end]);
        }
    }

    #[test]
    fn non_pitched_continuation_carries_split_rest_and_forward_from_one_source() {
        for (measure, voice, staff, scale) in [(17, "2", "2", 1), (701, "voice-x", "1", 1)] {
            let body = format!(r#"<note><pitch><step>C</step><octave>4</octave></pitch><duration>{}</duration><voice>{voice}</voice><staff>{staff}</staff></note><backup><duration>{}</duration></backup><note><rest/><duration>{}</duration><voice>{voice}</voice><staff>{staff}</staff></note>"#, 24 * scale, 24 * scale, 96 * scale);
            let xml = non_pitched_continuation_score(&body, measure)
                .replace("<divisions>96</divisions>", &format!("<divisions>{}</divisions>", 96 * scale));
            let projection = non_pitched_continuation_projection(&xml, true);
            assert_eq!(projection.provenance.len(), 1);
            let rows = &projection.non_pitched_continuations;
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0].carrier, FinalChordNonPitchedCarrier::Forward);
            assert_eq!(rows[1].carrier, FinalChordNonPitchedCarrier::Note);
            assert_eq!(rows[0].source_note_start, rows[1].source_note_start);
            assert_eq!(rows[0].source_note_end, rows[1].source_note_end);
            assert_eq!((rows[0].fragment_ordinal, rows[1].fragment_ordinal), (0, 1));
            assert!(rows.iter().all(|row| row.presentation_role.is_none()));
        }
    }

    #[test]
    fn non_pitched_continuation_duplicate_split_candidates_keep_separate_origins() {
        let body = r#"<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note><backup><duration>24</duration></backup><note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>"#;
        let single = non_pitched_continuation_projection(&non_pitched_continuation_score(body, 611), true);
        let duplicate = non_pitched_continuation_projection(&non_pitched_continuation_score(&body.repeat(2), 719), true);
        assert_eq!(duplicate.non_pitched_continuations.len(), 2 * single.non_pitched_continuations.len());
        let origins = duplicate.non_pitched_continuations.iter()
            .map(|row| (row.source_note_start, row.source_note_end))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(origins.len(), 2);
        for origin in origins {
            let fragments = duplicate.non_pitched_continuations.iter()
                .filter(|row| (row.source_note_start, row.source_note_end) == origin)
                .map(|row| (row.fragment_ordinal, row.carrier, row.presentation_role))
                .collect::<Vec<_>>();
            let original_fragments = single.non_pitched_continuations.iter()
                .map(|row| (row.fragment_ordinal, row.carrier, row.presentation_role))
                .collect::<Vec<_>>();
            assert_eq!(fragments, original_fragments);
        }
    }

    #[test]
    fn non_pitched_continuation_transport_is_independent_of_time_unit_scale() {
        for scale in [1, 7, 19] {
            let body = format!(r#"<note><rest/><duration>{}</duration><voice>opaque</voice><staff>2</staff></note>"#, 96 * scale);
            let xml = non_pitched_continuation_score(&body, 887)
                .replace("<divisions>96</divisions>", &format!("<divisions>{}</divisions>", 96 * scale));
            // Test transport independently of the accepted rest selector, whose
            // own authority is deliberately limited to normalized divisions=96.
            let projection = non_pitched_continuation_projection(&xml, false);
            assert_eq!(projection.xml, xml);
            assert_eq!(projection.non_pitched_continuations.len(), 1);
            assert_eq!(projection.non_pitched_continuations[0].fragment_ordinal, 0);
            assert!(projection.non_pitched_continuations[0].presentation_role.is_none());
        }
    }

    #[test]
    fn non_pitched_continuation_protects_actual_rest_chord_host() {
        let xml = non_pitched_continuation_score(r#"<note><rest/><duration>48</duration><voice>opaque</voice><staff>2</staff></note><note print-object="no"><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>48</duration><voice>opaque</voice><staff>2</staff></note>"#, 303);
        let projection = non_pitched_continuation_projection(&xml, true);
        assert_eq!(projection.xml, xml);
        assert_eq!(projection.provenance.len(), 1);
        assert_eq!(projection.non_pitched_continuations.len(), 1);
        assert_eq!(projection.non_pitched_continuations[0].carrier, FinalChordNonPitchedCarrier::Note);
        assert!(projection.non_pitched_continuations[0].presentation_role.is_none());
    }

    #[test]
    fn non_pitched_continuation_rejects_missing_extra_or_stale_output() {
        let xml = non_pitched_continuation_score(r#"<note><rest/><duration>96</duration><voice>1</voice><staff>1</staff></note>"#, 901);
        let plan = final_chord_finalize_bound_plan(&xml, Vec::new(), Vec::new(), false).unwrap();
        let missing = non_pitched_continuation_score("<forward><duration>96</duration></forward>", 901);
        assert!(final_chord_non_pitched_continuations(&xml, &plan, &[], &missing).is_err());
        let extra = xml.replace("</measure>", "<note><rest/><duration>24</duration></note></measure>");
        assert!(final_chord_non_pitched_continuations(&xml, &plan, &[], &extra).is_err());
        let same_length_stale = xml.replace("<voice>1</voice>", "<voice>9</voice>");
        assert!(final_chord_non_pitched_continuations(&xml, &plan, &[], &same_length_stale).is_err());
        assert!(project_final_chord_encoding_plan_with_provenance(&extra, &plan).is_err());
    }

    #[test]
    fn non_pitched_continuation_transports_through_prior_pitched_byte_edit() {
        let xml = non_pitched_continuation_score(r#"<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note><note><rest/><duration>72</duration><voice>primary-looking</voice><staff>1</staff></note>"#, 613);
        let measures = final_chord_parse_binding_measures(&xml).unwrap();
        let pitched = &measures[0].notes[0];
        let mut edits = Vec::new();
        let replacement = xml[pitched.range.clone()].replace("<voice>1</voice>", "<voice>arbitrary-export-label</voice>");
        final_chord_push_edit(&mut edits, pitched.range.start, pitched.range.end, replacement);
        let plan = final_chord_finalize_bound_plan(&xml, edits, Vec::new(), true).unwrap();
        let projection = project_final_chord_encoding_plan_with_provenance(&xml, &plan).unwrap();
        let row = &projection.non_pitched_continuations[0];
        assert!(row.projected_start > row.source_note_start);
        assert_eq!(&xml[row.source_note_start..row.source_note_end],
            &projection.xml[row.projected_start..row.projected_end]);
        assert!(row.presentation_role.is_none());
    }

    #[test]
    fn final_chord_visible_rest_interval_projection_is_final_voice_local_relocatable_and_additive() {
        fn synthetic_score(rows: &[(usize, &str)]) -> String {
            let mut measures = String::new();
            for (measure, body) in rows {
                measures.push_str(&format!(
                    r#"    <measure number="{measure}">
      <attributes>
        <divisions>96</divisions>
        <time><beats>4</beats><beat-type>4</beat-type></time>
      </attributes>
{body}
    </measure>
"#
                ));
            }
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
{measures}  </part>
</score-partwise>
"#
            )
        }

        let same_voice = r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>"#;
        let cross_voice = same_voice.replacen("<voice>2</voice>", "<voice>1</voice>", 1);
        let hidden_pitch =
            same_voice.replacen("<note><pitch>", "<note print-object=\"no\"><pitch>", 1);

        let one = synthetic_score(&[(71, same_voice)]);
        let one_edits = final_chord_plan_visible_rest_interval_edits(&one)
            .expect("synthetic same-final-lane rest projection must plan");
        assert_eq!(one_edits.len(), 1);
        assert!(one_edits[0]
            .replacement
            .contains("<forward><duration>24</duration><voice>2</voice><staff>2</staff></forward>"));
        assert!(one_edits[0].replacement.contains("<duration>72</duration>"));

        assert!(final_chord_plan_visible_rest_interval_edits(&synthetic_score(&[(72, cross_voice.as_str())]))
            .expect("cross-voice synthetic score must parse")
            .is_empty());
        assert!(final_chord_plan_visible_rest_interval_edits(&synthetic_score(&[(73, hidden_pitch.as_str())]))
            .expect("hidden-pitch synthetic score must parse")
            .is_empty());

        let duplicated = synthetic_score(&[(74, same_voice), (75, same_voice)]);
        let duplicated_edits = final_chord_plan_visible_rest_interval_edits(&duplicated)
            .expect("independent synthetic candidates must remain additive");
        assert_eq!(duplicated_edits.len(), 2);

        let anchored_rest = r#"      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
      <backup><duration>24</duration></backup>
      <note><rest/><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff></note>
      <note print-object="no" print-spacing="no"><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff></note>"#;
        let anchored = synthetic_score(&[(76, anchored_rest)]);
        assert!(
            final_chord_plan_visible_rest_interval_edits(&anchored)
                .expect("rest-backed chord anchor synthetic score must parse")
                .is_empty()
        );
        let anchored_plan = final_chord_finalize_bound_plan(
            &anchored,
            Vec::new(),
            Vec::new(),
            true,
        )
        .expect("synthetic anchored-rest canonical plan");
        assert_eq!(
            project_final_chord_encoding_plan(&anchored, &anchored_plan).0
                .expect("rest-backed chord anchor must remain byte-stable"),
            anchored
        );

        let early_plan = final_chord_finalize_bound_plan(
            &one,
            Vec::new(),
            Vec::new(),
            false,
        )
        .expect("synthetic early plan");
        assert_eq!(
            project_final_chord_encoding_plan(&one, &early_plan).0
                .expect("early noncanonical projection must remain rest-neutral"),
            one
        );

        let canonical_plan = final_chord_finalize_bound_plan(
            &one,
            Vec::new(),
            Vec::new(),
            true,
        )
        .expect("synthetic canonical plan");
        let projected = project_final_chord_encoding_plan(&one, &canonical_plan).0
            .expect("canonical final-voice projection must reconcile rest interval");
        assert!(projected.contains("<duration>72</duration>"));
        assert_eq!(
            final_chord_pitched_timeline_signature(&one).expect("before pitched signature"),
            final_chord_pitched_timeline_signature(&projected).expect("after pitched signature")
        );

        // #65 two-phase EXACT receipt: phase 1 (bound plan edits) and phase 2
        // (rest-interval edits bound on the intermediate state) compose into one
        // pre-coordinate script that realizes the projection byte-exactly.
        let (phase_two_only, phase_two_receipt) =
            project_final_chord_encoding_plan(&one, &canonical_plan);
        let phase_two_only = phase_two_only.expect("phase-2-only projection");
        let phase_two_receipt = phase_two_receipt.expect("phase-2-only #65 receipt");
        assert_eq!(phase_two_receipt.descriptor_row(), 65);
        assert_eq!(phase_two_receipt.realize_post_document(&one).as_deref(), Some(phase_two_only.as_str()));
        let part_name = one.find("Piano").expect("part name");
        let rest_tag = one.find("<note><rest/>").expect("rest note");
        for phase_one_edits in [
            vec![(part_name, part_name + "Piano".len(), "Klavier".to_string())],
            vec![
                (part_name, part_name + "Piano".len(), "Klavier".to_string()),
                (rest_tag, rest_tag + "<note>".len(), "<note >".to_string()),
            ],
            vec![(rest_tag, rest_tag + "<note>".len(), "<note >".to_string())],
        ] {
            let edits = phase_one_edits
                .into_iter()
                .map(|(start, end, replacement)| FinalChordEncodingEdit {
                    start,
                    end,
                    replacement,
                    generated_fragments: Vec::new(),
                })
                .collect::<Vec<_>>();
            for rest_projection in [false, true] {
                let plan = final_chord_finalize_bound_plan(&one, edits.clone(), Vec::new(), rest_projection)
                    .expect("synthetic two-phase plan");
                let (projected, receipt) = project_final_chord_encoding_plan(&one, &plan);
                let projected = projected.expect("two-phase projection");
                let receipt = receipt.expect("two-phase #65 receipt");
                assert_eq!(receipt.realize_post_document(&one).as_deref(), Some(projected.as_str()));
                assert_eq!(projected.contains("<duration>72</duration>"), rest_projection);
            }
        }
        let (unchanged, no_receipt) = project_final_chord_encoding_plan(&anchored, &anchored_plan);
        assert_eq!(unchanged.expect("no-op projection"), anchored);
        assert!(no_receipt.is_none());
        assert!(project_final_chord_encoding_plan(&(one.clone() + " "), &canonical_plan).1.is_none());

        let rest_before_pitch = r#"      <note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>
      <backup><duration>96</duration></backup>
      <note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>"#;
        let rest_before_pitch_score = synthetic_score(&[(77, rest_before_pitch)]);
        let rest_before_pitch_plan = final_chord_finalize_bound_plan(
            &rest_before_pitch_score,
            Vec::new(),
            Vec::new(),
            true,
        )
        .expect("synthetic rest-before-pitch canonical plan");
        let projection_with_provenance = project_final_chord_encoding_plan_with_provenance(
            &rest_before_pitch_score,
            &rest_before_pitch_plan,
        )
        .expect("rest-only byte delta must preserve pitched provenance transport");
        assert_eq!(projection_with_provenance.provenance.len(), 1);
        let projected_notes = final_chord_parse_binding_measures(&projection_with_provenance.xml)
            .expect("projected provenance score must parse")
            .into_iter()
            .flat_map(|measure| measure.notes)
            .filter(|note| !note.rest_like && note.pitch_midi.is_some())
            .collect::<Vec<_>>();
        assert_eq!(projected_notes.len(), 1);
        assert_eq!(
            projection_with_provenance.provenance[0].projected_note_start,
            projected_notes[0].range.start
        );
        assert_eq!(
            projection_with_provenance.provenance[0].projected_note_end,
            projected_notes[0].range.end
        );
    }

    // Synthetic retained-occurrence identity through the real binder and Output-Alpha.
    #[test]
    fn no_write_retained_twins_require_carried_occurrence_identity() {
        let (render, mut canonical, voice_plan, _) = render_prune_binding_fixture();
        let twins = render.replacen(FIXTURE_NOTE_D42565F8C7,
            &format!("{FIXTURE_NOTE_D42565F8C7}\n{FIXTURE_NOTE_D42565F8C7}"), 1);
        for locator in std::sync::Arc::make_mut(&mut canonical.source_locators) {
            if locator.fragment_id != 1 { locator.raw_note_index += 17; }
        }
        for action in std::sync::Arc::make_mut(&mut canonical.actions) {
            action.locator.raw_note_index += 17;
        }
        let omissions = std::collections::BTreeSet::new();
        let error = audit_musicxml_voice_label_binding_alpha_for_render_projection(
            &twins, &canonical, &voice_plan, &omissions).unwrap_err();
        assert!(error.to_string().contains("FinalChordPhysicalBindingAmbiguity"), "{error}");
        let measures = final_chord_parse_binding_measures(&twins).unwrap();
        let fingerprint = final_chord_voice_free_pitched_sequence_fingerprint(&measures);
        let witnesses = (1..=3).map(|id| FinalChordRenderOccurrenceWitness {
            fragment_id:id, source_event_id:id, attack_id:id, measure:1,
            pitched_ordinal:id-1, representation_duration:None,
            sequence_fingerprint_fnv64:fingerprint,
        }).collect::<Vec<_>>();
        attach_final_chord_render_occurrence_witnesses(&mut canonical, witnesses.clone()).unwrap();
        let alpha = audit_musicxml_voice_label_binding_alpha_for_render_projection(
            &twins, &canonical, &voice_plan, &omissions).expect("carried identity resolves existing twins");
        assert!(alpha.green());
        let bound = bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(
            &twins, &canonical, &voice_plan, Some(&omissions)).unwrap();
        assert_eq!(bound.input_bindings.len(), 3);
        let drifted = twins.replacen("<step>E</step>", "<step>F</step>", 1);
        assert!(audit_musicxml_voice_label_binding_alpha_for_render_projection(
            &drifted, &canonical, &voice_plan, &omissions).is_err());
        let mut duplicate = witnesses.clone(); duplicate.push(witnesses[1].clone());
        assert!(attach_final_chord_render_occurrence_witnesses(&mut canonical, duplicate).is_err());
        let mut wrong_origin = witnesses; wrong_origin[1].source_event_id += 91;
        assert!(attach_final_chord_render_occurrence_witnesses(&mut canonical, wrong_origin).is_err());
    }

    #[test]
    fn serialized_source_roles_survive_binding_alpha_and_channel_table_removal() {
        for staff in [1, 2] {
            for scale in [1, 7] {
                let ids = [907, 503];
                let (canonical, mut attacks) = synthetic_written_channel_case(
                    311, &ids, &[(0, scale), (scale, 2 * scale)],
                    &[(0, scale), (scale, 2 * scale)], staff);
                attacks[1].technical_lane = 2;
                let channels = build_frozen_serialization_voice_channels_for_canonical(
                    &canonical, &attacks).unwrap();
                let roles = ids.map(|source| channels.role_for_source(source).unwrap());
                let bound = bind_musicxml_voice_labels(&channels, &[]).unwrap();
                for (source, role) in ids.into_iter().zip(roles) {
                    assert_eq!(bound.role_for_source(source), Some(role));
                    assert_eq!(bound.role_for_source(source),
                        bound.label_for_source(source).and_then(|label| bound.role_for_label(label)));
                }
                let labels = bound.channel_to_label.values().cloned()
                    .collect::<std::collections::BTreeSet<_>>();
                let permutation = labels.into_iter().rev().collect::<Vec<_>>();
                let mut renamed = final_chord_permute_sealed_voice_labels(&bound, &permutation).unwrap();
                assert_eq!(renamed.source_to_role, bound.source_to_role);
                // Counterfactual readback only: no mixed-channel allocation is admitted.
                renamed.channel_to_role.clear();
                renamed.label_to_role.clear();
                for (source, role) in ids.into_iter().zip(roles) {
                    assert_eq!(renamed.role_for_source(source), Some(role));
                }
                assert_eq!(renamed.role_for_source(701), None);
            }
        }
    }

    #[test]
    fn serialized_source_role_binding_rejects_missing_and_unexpected_sources() {
        let (canonical, attacks) = synthetic_written_channel_case(
            311, &[907, 503], &[(0, 1), (1, 2)], &[(0, 1), (1, 2)], 1);
        let channels = build_frozen_serialization_voice_channels_for_canonical(
            &canonical, &attacks).unwrap();
        let mut missing = channels.clone();
        missing.source_to_role.remove(&907);
        assert!(bind_musicxml_voice_labels(&missing, &[]).unwrap_err().to_string()
            .contains("exact canonical source-role coverage"));
        let mut unexpected = channels;
        unexpected.source_to_role.insert(701, SerializedVoicePresentationRole::UpperPrimary);
        assert!(bind_musicxml_voice_labels(&unexpected, &[]).is_err());
    }

    fn source_role_beam_binding_probe(
        clear_label_roles: bool, missing_source_role: bool,
    ) -> PrimaryBeamVoicePartitionBoundPlan {
        let (xml, _, mut voice_plan, continuation, candidates) =
            primary_beam_voice_partition_binding_fixture();
        if clear_label_roles { voice_plan.label_to_role.clear(); }
        if missing_source_role { voice_plan.source_to_role.remove(&2); }
        let safety = ["metric-a".to_string()].into_iter().collect();
        let fragments = [("metric-a".to_string(), 2usize)].into_iter().collect();
        bind_primary_beam_voice_partition_plan(&xml, PrimaryBeamVoicePartitionBindingContext {
            source_continuation: &continuation, voice_plan: &voice_plan,
            candidates: &candidates, classification_xml: &xml,
            renderer_visible_fragments_by_metric_line_id: &fragments,
            materialization_safety_proven_metric_line_ids: &safety,
        }).unwrap()
    }

    #[test]
    fn primary_beam_binding_uses_carried_source_roles_and_preserves_them_on_rebind() {
        let ordinary = source_role_beam_binding_probe(false, false);
        let without_label_roles = source_role_beam_binding_probe(true, false);
        assert_eq!(ordinary.scope_candidates(), 1);
        assert_eq!(ordinary.changed_voice_elements(), 1);
        assert_eq!(without_label_roles.scope_candidates(), ordinary.scope_candidates());
        assert_eq!(without_label_roles.changed_voice_elements(), ordinary.changed_voice_elements());
        assert_eq!(without_label_roles.repaired_voice_plan().role_for_source(2),
            Some(SerializedVoicePresentationRole::LowerSecondary));
        assert_eq!(without_label_roles.repaired_voice_plan().label_for_source(2), Some("2"));
        assert_eq!(ordinary.repaired_voice_plan().source_to_role,
            without_label_roles.repaired_voice_plan().source_to_role);
        for (left, right) in ordinary.edits().iter().zip(without_label_roles.edits()) {
            assert_eq!((left.voice_text_start(), left.voice_text_end(), left.target_label()),
                (right.voice_text_start(), right.voice_text_end(), right.target_label()));
        }
    }

    #[test]
    fn primary_beam_binding_never_recovers_missing_source_role_from_label() {
        let plan = source_role_beam_binding_probe(false, true);
        assert_eq!(plan.scope_candidates(), 0);
        assert_eq!(plan.changed_voice_elements(), 0);
        assert_eq!(plan.repaired_voice_plan().role_for_source(2), None);
        assert_eq!(plan.repaired_voice_plan().role_for_label("4"),
            Some(SerializedVoicePresentationRole::LowerSecondary));
    }

}

#[cfg(test)]
#[path = "../../tests/unit/cross_staff_auxiliary_projection.rs"]
mod cross_staff_auxiliary_projection_tests;

#[path = "cross_role_channel_sharing.rs"]
mod cross_role_channel_sharing;

#[cfg(test)]
#[path = "../../tests/unit/final_chord_original_compact_capture.rs"]
mod original_compact_course_capture_tests;
