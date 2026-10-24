//! Audit-only renderer-neutral relocation plan for interstitial directions.
//!
//! The detector and musical planner are already finished when this module runs.
//! It receives only a fingerprint-bound resolved target plus the authorized
//! serialization plan. It may classify an already observed XML obstruction and
//! calculate an equivalent zero-duration placement, but it cannot choose a new
//! musical candidate and it never writes XML.

#[cfg(test)]
use super::authority_primitives::yes_no;
use super::locator_resolution::{
    stable_xml_snapshot_fingerprint, RendererInterstitialElementKind, ResolvedSerializationTarget,
};
use super::serialization_plan::AuthorizedSerializationPlan;

const PLANNER_MODULE_SOURCE: &str = include_str!("renderer_direction_relocation_plan.rs");
#[cfg(test)]
const PLANNER_MODULE_PATH: &str =
    "src/structural_write_authority/renderer_direction_relocation_plan.rs";
const ABSOLUTE_IDENTITY_PATTERNS: &[&str] = &[
    concat!("Etüde Nr. ", "6"),
    concat!("\"", "3", "3", "\""),
    concat!("<st", "ep>"),
    concat!("<oct", "ave>"),
    concat!("<voi", "ce>", "2", "</voi", "ce>"),
    concat!("<sta", "ff>", "1", "</sta", "ff>"),
    concat!("<sta", "ff>", "2", "</sta", "ff>"),
];
const MUSICAL_SELECTION_PATTERNS: &[&str] = &[
    concat!("detect_structural_", "repair_intents"),
    concat!("classify_tail_", "beam_state"),
    concat!("Structural", "View"),
    concat!("LocatedStructural", "RepairIntent"),
    concat!("StructuralRepair", "Intent::"),
];
const LOCATOR_IDENTITY_ACCESS_PATTERNS: &[&str] = &[
    concat!("locator.", "part_ordinal"),
    concat!("locator.", "measure_ordinal"),
    concat!("locator.", "source_identity"),
    concat!("locator.", "source_events"),
    concat!("locator.", "source_fingerprint"),
];

#[derive(Debug, Clone, Default)]
pub struct RendererNeutralDirectionPlacementPlanAudit {
    #[cfg(test)]
    source_stage: String,
    #[cfg(test)]
    planner_module_path: String,
    #[cfg(test)]
    beam_groups_checked: usize,
    #[cfg(test)]
    member_pairs_checked: usize,
    #[cfg(test)]
    fragment_parse_failures: usize,
    #[cfg(test)]
    interstitial_elements: usize,
    #[cfg(test)]
    interstitial_directions: usize,
    #[cfg(test)]
    interstitial_other_elements: usize,
    unique_direction_obstacle: bool,
    direction_group_ordinal: usize,
    direction_pair_ordinal: usize,
    direction_element_ordinal: usize,
    group_start_cursor: i64,
    group_end_cursor: i64,
    source_element_cursor: i64,
    source_offset: i64,
    source_offset_explicit: bool,
    source_effective_cursor: i64,
    proposed_host_cursor: i64,
    proposed_offset: i64,
    proposed_offset_nonnegative: bool,
    proposed_offset_within_group: bool,
    effective_cursor_stable: bool,
    direction_type_element_count: usize,
    dynamics_mark_count: usize,
    instantaneous_dynamics_only: bool,
    sound_absent: bool,
    timing_spanner_absent: bool,
    words_or_metronome_absent: bool,
    default_x_absent: bool,
    staff_relation_to_right_member_stable: bool,
    payload_without_offset_fingerprint_bound: bool,
    source_snapshot_fingerprint_bound: bool,
    authorization_binding_ready: bool,
    predicted_interstitial_elements_after_relocation: usize,
    predicted_renderer_contiguity_green: bool,
    #[cfg(test)]
    absolute_identity_references: usize,
    #[cfg(test)]
    musical_selection_references: usize,
    #[cfg(test)]
    locator_identity_access_references: usize,
    planner_identity_free_boundary_green: bool,
    relocation_plan_feasible: bool,
    #[cfg(test)]
    authorization_attempted: bool,
    #[cfg(test)]
    writer_token_used: bool,
    #[cfg(test)]
    write_produced: bool,
    #[cfg(test)]
    verdict: String,
}

#[derive(Debug)]
pub(super) struct ProvenRendererDirectionRelocationPlan {
    source_snapshot_fingerprint: u64,
    authorized_beam_group_ordinal: usize,
    authorized_member_pair_ordinal: usize,
    unique_obstacle_element_ordinal: usize,
    group_start_cursor: i64,
    group_end_cursor: i64,
    source_element_cursor: i64,
    source_offset: i64,
    source_offset_explicit: bool,
    source_effective_cursor: i64,
    proposed_host_cursor: i64,
    proposed_offset: i64,
    direction_type_element_count: usize,
    dynamics_mark_count: usize,
    staff_relation_to_right_member_stable: bool,
    payload_without_offset_fingerprint: u64,
    sound_absent: bool,
    timing_spanner_absent: bool,
    words_or_metronome_absent: bool,
    default_x_absent: bool,
    expected_interstitial_elements_after_relocation: usize,
    predicted_renderer_contiguity_green: bool,
}

#[derive(Debug)]
pub(super) struct RendererDirectionRelocationAuthorizationParts {
    pub(super) source_snapshot_fingerprint: u64,
    pub(super) authorized_beam_group_ordinal: usize,
    pub(super) authorized_member_pair_ordinal: usize,
    pub(super) unique_obstacle_element_ordinal: usize,
    pub(super) group_start_cursor: i64,
    pub(super) group_end_cursor: i64,
    pub(super) source_element_cursor: i64,
    pub(super) source_offset: i64,
    pub(super) source_offset_explicit: bool,
    pub(super) source_effective_cursor: i64,
    pub(super) proposed_host_cursor: i64,
    pub(super) proposed_offset: i64,
    pub(super) direction_type_element_count: usize,
    pub(super) dynamics_mark_count: usize,
    pub(super) staff_relation_to_right_member_stable: bool,
    pub(super) payload_without_offset_fingerprint: u64,
    pub(super) sound_absent: bool,
    pub(super) timing_spanner_absent: bool,
    pub(super) words_or_metronome_absent: bool,
    pub(super) default_x_absent: bool,
    pub(super) expected_interstitial_elements_after_relocation: usize,
    pub(super) predicted_renderer_contiguity_green: bool,
}

impl ProvenRendererDirectionRelocationPlan {
    pub(super) fn into_authorization_parts(self) -> RendererDirectionRelocationAuthorizationParts {
        RendererDirectionRelocationAuthorizationParts {
            source_snapshot_fingerprint: self.source_snapshot_fingerprint,
            authorized_beam_group_ordinal: self.authorized_beam_group_ordinal,
            authorized_member_pair_ordinal: self.authorized_member_pair_ordinal,
            unique_obstacle_element_ordinal: self.unique_obstacle_element_ordinal,
            group_start_cursor: self.group_start_cursor,
            group_end_cursor: self.group_end_cursor,
            source_element_cursor: self.source_element_cursor,
            source_offset: self.source_offset,
            source_offset_explicit: self.source_offset_explicit,
            source_effective_cursor: self.source_effective_cursor,
            proposed_host_cursor: self.proposed_host_cursor,
            proposed_offset: self.proposed_offset,
            direction_type_element_count: self.direction_type_element_count,
            dynamics_mark_count: self.dynamics_mark_count,
            staff_relation_to_right_member_stable: self.staff_relation_to_right_member_stable,
            payload_without_offset_fingerprint: self.payload_without_offset_fingerprint,
            sound_absent: self.sound_absent,
            timing_spanner_absent: self.timing_spanner_absent,
            words_or_metronome_absent: self.words_or_metronome_absent,
            default_x_absent: self.default_x_absent,
            expected_interstitial_elements_after_relocation: self
                .expected_interstitial_elements_after_relocation,
            predicted_renderer_contiguity_green: self.predicted_renderer_contiguity_green,
        }
    }
}

impl RendererNeutralDirectionPlacementPlanAudit {
    #[cfg(test)]
    pub fn feasible(&self) -> bool {
        self.relocation_plan_feasible
    }

    #[cfg(test)]
    pub fn pipeline_line(&self, project_label: &str) -> String {
        format!(
            "RendererNeutralDirectionPlacementPlanAudit v{}: Project {}; SourceStage {}; PlannerModule {}; BeamGroupsChecked {}; MemberPairsChecked {}; FragmentParseFailures {}; InterstitialElements {}; InterstitialDirections {}; InterstitialOtherElements {}; UniqueDirectionObstacle {}; DirectionGroupOrdinal {}; DirectionPairOrdinal {}; DirectionElementOrdinal {}; GroupCursor {}->{}; SourceElementCursor {}; SourceOffset {}; SourceOffsetExplicit {}; SourceEffectiveCursor {}; ProposedHostCursor {}; ProposedOffset {}; ProposedOffsetNonnegative {}; ProposedOffsetWithinGroup {}; EffectiveCursorStable {}; DirectionTypeElements {}; DynamicsMarks {}; InstantaneousDynamicsOnly {}; SoundAbsent {}; TimingSpannerAbsent {}; WordsOrMetronomeAbsent {}; DefaultXAbsent {}; StaffRelationToRightMemberStable {}; PayloadWithoutOffsetFingerprintBound {}; SourceSnapshotFingerprintBound {}; AuthorizationBindingReady {}; PredictedInterstitialElementsAfterRelocation {}; PredictedRendererContiguityGreen {}; AbsoluteIdentityReferences {}; MusicalSelectionReferences {}; LocatorIdentityAccessReferences {}; PlannerIdentityFreeBoundaryGreen {}; RelocationPlanFeasible {}; AuthorizationAttempted {}; WriterTokenUsed {}; WriteProduced {}; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            project_label,
            self.source_stage,
            self.planner_module_path,
            self.beam_groups_checked,
            self.member_pairs_checked,
            self.fragment_parse_failures,
            self.interstitial_elements,
            self.interstitial_directions,
            self.interstitial_other_elements,
            yes_no(self.unique_direction_obstacle),
            self.direction_group_ordinal,
            self.direction_pair_ordinal,
            self.direction_element_ordinal,
            self.group_start_cursor,
            self.group_end_cursor,
            self.source_element_cursor,
            self.source_offset,
            yes_no(self.source_offset_explicit),
            self.source_effective_cursor,
            self.proposed_host_cursor,
            self.proposed_offset,
            yes_no(self.proposed_offset_nonnegative),
            yes_no(self.proposed_offset_within_group),
            yes_no(self.effective_cursor_stable),
            self.direction_type_element_count,
            self.dynamics_mark_count,
            yes_no(self.instantaneous_dynamics_only),
            yes_no(self.sound_absent),
            yes_no(self.timing_spanner_absent),
            yes_no(self.words_or_metronome_absent),
            yes_no(self.default_x_absent),
            yes_no(self.staff_relation_to_right_member_stable),
            yes_no(self.payload_without_offset_fingerprint_bound),
            yes_no(self.source_snapshot_fingerprint_bound),
            yes_no(self.authorization_binding_ready),
            self.predicted_interstitial_elements_after_relocation,
            yes_no(self.predicted_renderer_contiguity_green),
            self.absolute_identity_references,
            self.musical_selection_references,
            self.locator_identity_access_references,
            yes_no(self.planner_identity_free_boundary_green),
            yes_no(self.relocation_plan_feasible),
            yes_no(self.authorization_attempted),
            yes_no(self.writer_token_used),
            yes_no(self.write_produced),
            self.verdict,
        )
    }
}

pub(super) fn audit_renderer_neutral_direction_placement_plan(
    target: &ResolvedSerializationTarget,
    source_xml: &str,
    plan: &AuthorizedSerializationPlan,
    _source_stage: &str,
) -> (
    RendererNeutralDirectionPlacementPlanAudit,
    Option<ProvenRendererDirectionRelocationPlan>,
) {
    let snapshot = target.renderer_interstitial_obstacle_snapshot(source_xml, plan);
    let source_snapshot_fingerprint = stable_xml_snapshot_fingerprint(source_xml);
    let absolute_identity_references = ABSOLUTE_IDENTITY_PATTERNS
        .iter()
        .map(|pattern| PLANNER_MODULE_SOURCE.matches(pattern).count())
        .sum::<usize>();
    let musical_selection_references = MUSICAL_SELECTION_PATTERNS
        .iter()
        .map(|pattern| PLANNER_MODULE_SOURCE.matches(pattern).count())
        .sum::<usize>();
    let locator_identity_access_references = LOCATOR_IDENTITY_ACCESS_PATTERNS
        .iter()
        .map(|pattern| PLANNER_MODULE_SOURCE.matches(pattern).count())
        .sum::<usize>();
    let planner_identity_free_boundary_green = absolute_identity_references == 0
        && musical_selection_references == 0
        && locator_identity_access_references == 0;

    let interstitial_directions = snapshot
        .obstacles
        .iter()
        .filter(|obstacle| obstacle.kind == RendererInterstitialElementKind::Direction)
        .count();
    let interstitial_other_elements = snapshot
        .obstacles
        .len()
        .saturating_sub(interstitial_directions);
    let unique_direction_obstacle = snapshot.obstacles.len() == 1
        && interstitial_directions == 1
        && interstitial_other_elements == 0;

    let mut audit = RendererNeutralDirectionPlacementPlanAudit {
        #[cfg(test)]
        source_stage: _source_stage.to_string(),
        #[cfg(test)]
        planner_module_path: PLANNER_MODULE_PATH.to_string(),
        #[cfg(test)]
        beam_groups_checked: snapshot.beam_groups_checked,
        #[cfg(test)]
        member_pairs_checked: snapshot.member_pairs_checked,
        #[cfg(test)]
        fragment_parse_failures: snapshot.fragment_parse_failures,
        #[cfg(test)]
        interstitial_elements: snapshot.obstacles.len(),
        #[cfg(test)]
        interstitial_directions,
        #[cfg(test)]
        interstitial_other_elements,
        unique_direction_obstacle,
        #[cfg(test)]
        absolute_identity_references,
        #[cfg(test)]
        musical_selection_references,
        #[cfg(test)]
        locator_identity_access_references,
        planner_identity_free_boundary_green,
        source_snapshot_fingerprint_bound: source_snapshot_fingerprint != 0,
        #[cfg(test)]
        verdict: "renderer_neutral_direction_placement_plan_not_feasible".to_string(),
        ..RendererNeutralDirectionPlacementPlanAudit::default()
    };

    let Some(obstacle) = snapshot.obstacles.first().copied() else {
        return (audit, None);
    };
    if !unique_direction_obstacle
        || obstacle.kind != RendererInterstitialElementKind::Direction
        || snapshot.fragment_parse_failures != 0
    {
        return (audit, None);
    }

    audit.direction_group_ordinal = obstacle.group_ordinal;
    audit.direction_pair_ordinal = obstacle.pair_ordinal;
    audit.direction_element_ordinal = obstacle.element_ordinal;
    audit.group_start_cursor = obstacle.group_start_cursor;
    audit.group_end_cursor = obstacle.group_end_cursor;
    audit.source_element_cursor = obstacle.element_cursor;
    audit.source_offset = obstacle.current_offset;
    audit.source_offset_explicit = obstacle.offset_explicit;
    audit.source_effective_cursor = obstacle.effective_cursor;
    audit.proposed_host_cursor = obstacle.group_start_cursor;
    audit.proposed_offset = obstacle
        .effective_cursor
        .saturating_sub(obstacle.group_start_cursor);
    audit.proposed_offset_nonnegative = audit.proposed_offset >= 0;
    audit.proposed_offset_within_group = audit.proposed_offset >= 0
        && obstacle
            .group_start_cursor
            .saturating_add(audit.proposed_offset)
            < obstacle.group_end_cursor;
    audit.effective_cursor_stable = obstacle
        .group_start_cursor
        .saturating_add(audit.proposed_offset)
        == obstacle.effective_cursor;
    audit.direction_type_element_count = obstacle.direction_type_element_count;
    audit.dynamics_mark_count = obstacle.dynamics_mark_count;
    audit.sound_absent = !obstacle.has_sound;
    audit.timing_spanner_absent = !obstacle.has_timing_spanner;
    audit.words_or_metronome_absent = !obstacle.has_words_or_metronome;
    audit.default_x_absent = !obstacle.has_default_x;
    audit.instantaneous_dynamics_only = obstacle.direction_type_element_count == 1
        && obstacle.dynamics_mark_count == 1
        && audit.sound_absent
        && audit.timing_spanner_absent
        && audit.words_or_metronome_absent
        && audit.default_x_absent;
    audit.staff_relation_to_right_member_stable =
        obstacle.staff_matches_right_member && !obstacle.staff_matches_left_member;
    audit.payload_without_offset_fingerprint_bound =
        obstacle.payload_without_offset_fingerprint != 0;
    audit.predicted_interstitial_elements_after_relocation = 0;
    audit.predicted_renderer_contiguity_green = snapshot.beam_groups_checked > 0
        && snapshot.member_pairs_checked > 0
        && snapshot.fragment_parse_failures == 0
        && audit.predicted_interstitial_elements_after_relocation == 0;
    audit.relocation_plan_feasible = audit.unique_direction_obstacle
        && audit.planner_identity_free_boundary_green
        && obstacle.left_cursor_after == obstacle.right_cursor_before
        && obstacle.element_cursor == obstacle.right_cursor_before
        && obstacle.effective_cursor >= obstacle.group_start_cursor
        && obstacle.effective_cursor <= obstacle.group_end_cursor
        && audit.proposed_offset_nonnegative
        && audit.proposed_offset_within_group
        && audit.effective_cursor_stable
        && audit.instantaneous_dynamics_only
        && audit.staff_relation_to_right_member_stable
        && audit.payload_without_offset_fingerprint_bound
        && audit.source_snapshot_fingerprint_bound
        && audit.predicted_renderer_contiguity_green;
    audit.authorization_binding_ready = audit.relocation_plan_feasible;
    #[cfg(test)]
    {
        audit.verdict = if audit.relocation_plan_feasible {
            "renderer_neutral_direction_placement_plan_proven_audit_only".to_string()
        } else {
            "renderer_neutral_direction_placement_plan_blocked_by_invariant".to_string()
        };
    }
    let proven_plan =
        audit
            .relocation_plan_feasible
            .then_some(ProvenRendererDirectionRelocationPlan {
                source_snapshot_fingerprint,
                authorized_beam_group_ordinal: obstacle.group_ordinal,
                authorized_member_pair_ordinal: obstacle.pair_ordinal,
                unique_obstacle_element_ordinal: obstacle.element_ordinal,
                group_start_cursor: obstacle.group_start_cursor,
                group_end_cursor: obstacle.group_end_cursor,
                source_element_cursor: obstacle.element_cursor,
                source_offset: obstacle.current_offset,
                source_offset_explicit: obstacle.offset_explicit,
                source_effective_cursor: obstacle.effective_cursor,
                proposed_host_cursor: audit.proposed_host_cursor,
                proposed_offset: audit.proposed_offset,
                direction_type_element_count: obstacle.direction_type_element_count,
                dynamics_mark_count: obstacle.dynamics_mark_count,
                staff_relation_to_right_member_stable: audit.staff_relation_to_right_member_stable,
                payload_without_offset_fingerprint: obstacle.payload_without_offset_fingerprint,
                sound_absent: audit.sound_absent,
                timing_spanner_absent: audit.timing_spanner_absent,
                words_or_metronome_absent: audit.words_or_metronome_absent,
                default_x_absent: audit.default_x_absent,
                expected_interstitial_elements_after_relocation: audit
                    .predicted_interstitial_elements_after_relocation,
                predicted_renderer_contiguity_green: audit.predicted_renderer_contiguity_green,
            });
    (audit, proven_plan)
}


