use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use super::final_notation_voice_beam_ownership_audit::{
    voice_free_primary_beam_chain_evidence, VoiceFreePrimaryBeamChainEvidence,
};

pub(crate) const SECTION: &str = "NOTATION-VOICE-PRE-SERIALIZATION-CONSTRAINT-CLOSURE-AUDIT";
pub(crate) const SECTION_RULE: &str = "----------------------------------------------------------";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreSerializationSourceAttackRow {
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) measure: usize,
    pub(crate) semantic_onset: i64,
    pub(crate) semantic_duration: i64,
    pub(crate) pitch_midi: i32,
    pub(crate) staff: u8,
    pub(crate) absolute_start: i64,
    pub(crate) absolute_end: i64,
    pub(crate) technical_lane: usize,
    pub(crate) pre_lane_voice: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreSerializationOccurrenceRow {
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) measure: usize,
    pub(crate) pitched_ordinal: usize,
    pub(crate) represented_onset: i64,
    pub(crate) represented_duration: i64,
    pub(crate) pitch_midi: i32,
    pub(crate) staff: u8,
    pub(crate) current_voice: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PreSerializationAtomicGroupKind {
    VisiblePitched,
    HiddenPitched,
    VisibleRest,
    HiddenRest,
    Other,
    ChordWithoutPitchedAnchor,
}

impl PreSerializationAtomicGroupKind {
    const fn has_pitched_anchor(self) -> bool {
        matches!(self, Self::VisiblePitched | Self::HiddenPitched)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PreSerializationAtomicGroupTracker {
    next_group_id: usize,
    current_measure: Option<usize>,
    current_anchor_group_id: Option<usize>,
    current_anchor_kind: Option<PreSerializationAtomicGroupKind>,
    current_orphan_chord_group_id: Option<usize>,
}

impl Default for PreSerializationAtomicGroupTracker {
    fn default() -> Self {
        Self {
            next_group_id: 1,
            current_measure: None,
            current_anchor_group_id: None,
            current_anchor_kind: None,
            current_orphan_chord_group_id: None,
        }
    }
}

impl PreSerializationAtomicGroupTracker {
    fn allocate_group(&mut self) -> usize {
        let group = self.next_group_id;
        self.next_group_id = self.next_group_id.saturating_add(1);
        group
    }

    pub(crate) fn assign(
        &mut self,
        measure: usize,
        visible: bool,
        is_rest: bool,
        is_chord: bool,
        has_pitch: bool,
    ) -> (usize, PreSerializationAtomicGroupKind) {
        if self.current_measure != Some(measure) {
            self.current_measure = Some(measure);
            self.current_anchor_group_id = None;
            self.current_anchor_kind = None;
            self.current_orphan_chord_group_id = None;
        }

        if !is_chord {
            let group = self.allocate_group();
            let kind = if is_rest {
                if visible {
                    PreSerializationAtomicGroupKind::VisibleRest
                } else {
                    PreSerializationAtomicGroupKind::HiddenRest
                }
            } else if has_pitch {
                if visible {
                    PreSerializationAtomicGroupKind::VisiblePitched
                } else {
                    PreSerializationAtomicGroupKind::HiddenPitched
                }
            } else {
                PreSerializationAtomicGroupKind::Other
            };
            self.current_anchor_group_id = Some(group);
            self.current_anchor_kind = Some(kind);
            self.current_orphan_chord_group_id = None;
            return (group, kind);
        }

        if let (Some(group), Some(kind)) =
            (self.current_anchor_group_id, self.current_anchor_kind)
            && kind.has_pitched_anchor()
        {
            return (group, kind);
        }

        let group = match self.current_orphan_chord_group_id {
            Some(group) => group,
            None => {
                let group = self.allocate_group();
                self.current_orphan_chord_group_id = Some(group);
                group
            }
        };
        (
            group,
            PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreSerializationVisibleOccupancyRow {
    pub(crate) measure: usize,
    pub(crate) staff: u8,
    pub(crate) represented_onset: i64,
    pub(crate) represented_duration: i64,
    pub(crate) visible: bool,
    pub(crate) is_rest: bool,
    pub(crate) is_chord_member: bool,
    pub(crate) pitch_midi: Option<i32>,
    pub(crate) source_event_id: Option<usize>,
    pub(crate) attack_id: Option<usize>,
    pub(crate) atomic_group_id: usize,
    pub(crate) atomic_group_kind: PreSerializationAtomicGroupKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreSerializationClosedSameLineMemberBinding {
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) measure: usize,
    pub(crate) pitched_ordinal: usize,
    pub(crate) represented_onset: i64,
    pub(crate) represented_duration: i64,
    pub(crate) staff: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreSerializationClosedSameLineCandidate {
    pub(crate) proof_chain_id: String,
    pub(crate) source_event_ids: Vec<usize>,
    pub(crate) members: Vec<PreSerializationClosedSameLineMemberBinding>,
    pub(crate) staff: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalPrimaryBeamOwnerReceipt {
    pub(crate) proof_chain_id: String,
    pub(crate) owner_kind: &'static str,
    pub(crate) owner_source_fnv64: u64,
    pub(crate) event_count: usize,
    pub(crate) plan_fingerprint_fnv64: u64,
    pub(crate) primary_states: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreSerializationMetricNotationLineCandidate {
    pub(crate) metric_line_id: String,
    pub(crate) source_event_ids: Vec<usize>,
    pub(crate) members: Vec<PreSerializationClosedSameLineMemberBinding>,
    pub(crate) measure: usize,
    pub(crate) staff: u8,
    pub(crate) beat: i64,
    pub(crate) interval_start: i64,
    pub(crate) interval_end: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaterializationSafetyPrimaryTargetEvidenceRow {
    pub(crate) metric_line_id: String,
    pub(crate) target_primary_source_event_id: usize,
    pub(crate) target_primary_attack_id: usize,
    pub(crate) target_channel: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VisibleRhythmicOccupancyPhaseAtom {
    pub(crate) measure: usize,
    pub(crate) staff: u8,
    pub(crate) onset: i64,
    pub(crate) duration: i64,
    pub(crate) kind: VisibleRhythmicOccupancyAtomKind,
    pub(crate) pitches: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct VisibleRhythmicOccupancyDuplicateRestConflict {
    pub(crate) measure: usize,
    pub(crate) staff: u8,
    pub(crate) onset: i64,
    pub(crate) duration: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct VisibleRhythmicOccupancyRestPitchedConflict {
    pub(crate) measure: usize,
    pub(crate) staff: u8,
    pub(crate) rest_onset: i64,
    pub(crate) rest_duration: i64,
    pub(crate) pitched_onset: i64,
    pub(crate) pitched_duration: i64,
    pub(crate) pitches: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct VisibleRhythmicOccupancyPhaseSnapshot {
    pub(crate) metric_parse_ok: bool,
    pub(crate) visible_atoms: Vec<VisibleRhythmicOccupancyPhaseAtom>,
    pub(crate) hidden_atoms: Vec<VisibleRhythmicOccupancyPhaseAtom>,
    pub(crate) incomplete_groups: Vec<String>,
    pub(crate) incomplete_group_ids: BTreeSet<usize>,
    pub(crate) duplicate_rest_conflicts: BTreeSet<VisibleRhythmicOccupancyDuplicateRestConflict>,
    pub(crate) rest_pitched_conflicts: BTreeSet<VisibleRhythmicOccupancyRestPitchedConflict>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NotationVoicePreSerializationConstraintClosureAuditProjection {
    body: String,
    closed_candidates: Vec<PreSerializationClosedSameLineCandidate>,
    metric_line_candidates: Vec<PreSerializationMetricNotationLineCandidate>,
    occurrence_rows: Vec<PreSerializationOccurrenceRow>,
    pre_final_primary_beam_exact_proof_chain_ids: BTreeSet<String>,
    canonical_primary_beam_owner_receipts: Vec<CanonicalPrimaryBeamOwnerReceipt>,
    materialization_safety_proven_metric_line_ids: BTreeSet<String>,
    materialization_safety_primary_target_evidence_rows:
        Vec<MaterializationSafetyPrimaryTargetEvidenceRow>,
    visible_rhythmic_occupancy_phase_snapshot: VisibleRhythmicOccupancyPhaseSnapshot,
}

impl NotationVoicePreSerializationConstraintClosureAuditProjection {
    pub(crate) fn append_body(&self, out: &mut String) {
        out.push_str(&self.body);
    }

    pub(crate) fn closed_candidates(&self) -> &[PreSerializationClosedSameLineCandidate] {
        &self.closed_candidates
    }

    pub(crate) fn metric_line_candidates(&self) -> &[PreSerializationMetricNotationLineCandidate] {
        &self.metric_line_candidates
    }

    pub(crate) fn occurrence_rows(&self) -> &[PreSerializationOccurrenceRow] {
        &self.occurrence_rows
    }

    pub(crate) fn pre_final_primary_beam_exact_proof_chain_ids(&self) -> &BTreeSet<String> {
        &self.pre_final_primary_beam_exact_proof_chain_ids
    }

    pub(crate) fn canonical_primary_beam_owner_receipts(
        &self,
    ) -> &[CanonicalPrimaryBeamOwnerReceipt] {
        &self.canonical_primary_beam_owner_receipts
    }

    pub(crate) fn materialization_safety_proven_metric_line_ids(&self) -> &BTreeSet<String> {
        &self.materialization_safety_proven_metric_line_ids
    }

    pub(crate) fn materialization_safety_primary_target_evidence_rows(
        &self,
    ) -> &[MaterializationSafetyPrimaryTargetEvidenceRow] {
        &self.materialization_safety_primary_target_evidence_rows
    }

    pub(crate) fn visible_rhythmic_occupancy_phase_snapshot(
        &self,
    ) -> &VisibleRhythmicOccupancyPhaseSnapshot {
        &self.visible_rhythmic_occupancy_phase_snapshot
    }

    pub(crate) fn append_counterfactual_body(&mut self, body: &str) {
        self.body.push_str(body);
    }
}

impl Default for NotationVoicePreSerializationConstraintClosureAuditProjection {
    fn default() -> Self {
        Self {
            body: String::from(
                "ProjectionOnly: true\nAuthority: none\nStatus: UNAVAILABLE_NOT_COMPUTED\n",
            ),
            closed_candidates: Vec::new(),
            metric_line_candidates: Vec::new(),
            occurrence_rows: Vec::new(),
            pre_final_primary_beam_exact_proof_chain_ids: BTreeSet::new(),
            canonical_primary_beam_owner_receipts: Vec::new(),
            materialization_safety_proven_metric_line_ids: BTreeSet::new(),
            materialization_safety_primary_target_evidence_rows: Vec::new(),
            visible_rhythmic_occupancy_phase_snapshot:
                VisibleRhythmicOccupancyPhaseSnapshot::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChainStatus {
    CandidateClosedExceptBeamProvenanceAndMaterialization,
    NotApplicableSingleTechnicalLane,
    BlockedNoncontiguousRepresentation,
    BlockedOccurrenceIdentity,
    BlockedSourceAttackIdentity,
    BlockedNonInjectiveIdentity,
    BlockedSemanticTimeDivergence,
    BlockedLanePressureWitness,
    BlockedExternalLanePressure,
    BlockedIndependentPitchedOccupancy,
    BlockedRestOccupancyOwnership,
}

impl ChainStatus {
    const fn label(self) -> &'static str {
        match self {
            Self::CandidateClosedExceptBeamProvenanceAndMaterialization => {
                "BLOCKED_INCOMPLETE_BEAM_WITNESS_PROVENANCE_AND_MATERIALIZATION"
            }
            Self::NotApplicableSingleTechnicalLane => "NOT_APPLICABLE_SINGLE_TECHNICAL_LANE",
            Self::BlockedNoncontiguousRepresentation => {
                "BLOCKED_INCOMPLETE_NONCONTIGUOUS_REPRESENTATION"
            }
            Self::BlockedOccurrenceIdentity => "BLOCKED_INCOMPLETE_OCCURRENCE_IDENTITY",
            Self::BlockedSourceAttackIdentity => "BLOCKED_INCOMPLETE_SOURCE_ATTACK_IDENTITY",
            Self::BlockedNonInjectiveIdentity => "BLOCKED_CONTRADICTION_NONINJECTIVE_IDENTITY",
            Self::BlockedSemanticTimeDivergence => {
                "BLOCKED_INCOMPLETE_SEMANTIC_REPRESENTATION_TIME_ORIGIN"
            }
            Self::BlockedLanePressureWitness => "BLOCKED_INCOMPLETE_LANE_PRESSURE_WITNESS",
            Self::BlockedExternalLanePressure => "BLOCKED_INCOMPLETE_EXTERNAL_LANE_PRESSURE",
            Self::BlockedIndependentPitchedOccupancy => {
                "BLOCKED_CONTRADICTION_INDEPENDENT_PITCHED_OCCUPANCY"
            }
            Self::BlockedRestOccupancyOwnership => {
                "BLOCKED_INCOMPLETE_REST_OCCUPANCY_OWNERSHIP"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LanePressureEdge {
    blocker_source_event_id: usize,
    blocker_attack_id: usize,
    blocker_lane: usize,
    blocker_absolute_end: i64,
    blocked_source_event_id: usize,
    blocked_attack_id: usize,
    blocked_lane: usize,
    blocked_absolute_start: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClassifiedChain {
    status: ChainStatus,
    source_event_ids: Vec<usize>,
    attack_ids: Vec<usize>,
    technical_lanes: Vec<usize>,
    current_voices: Vec<String>,
    lane_pressure_edges: Vec<LanePressureEdge>,
    external_lane_pressure_source_ids: Vec<usize>,
    external_pitched_atomic_groups: Vec<usize>,
    rest_atomic_groups: Vec<usize>,
    chord_extension_atomic_groups: Vec<usize>,
    semantic_durations: Vec<i64>,
    semantic_shape: String,
}

type OccurrenceKey = (usize, usize);
type SourceAttackKey = (usize, usize);

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut state = 0xcbf29ce484222325u64;
    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100000001b3);
    }
    state
}

const CANONICAL_PRIMARY_BEAM_OWNER_KIND: &str = "structural_metric_beam_hierarchy_plan";
const CANONICAL_PRIMARY_BEAM_OWNER_SOURCE_FNV64_CURRENT: u64 = 0x5fdeb910c8bffee5;
const CANONICAL_PRIMARY_BEAM_OWNER_SOURCE: &str =
    include_str!("../pipeline_shards/composer_metric_beam_planning_core.rs");

fn canonical_primary_beam_owner_source_is_bound() -> bool {
    let source = CANONICAL_PRIMARY_BEAM_OWNER_SOURCE;
    fnv1a64(source.as_bytes()) == CANONICAL_PRIMARY_BEAM_OWNER_SOURCE_FNV64_CURRENT
        && source.contains("fn structural_metric_beam_hierarchy_plan(")
        && source.contains("let primary = (0..group.len()).collect::<Vec<_>>();")
        && source.contains("let mut level_groups = vec![(primary, 1_u8)];")
        && source.contains("BeamPlan::from_level_groups(group.len(), &borrowed_groups)")
}

fn current_canonical_primary_beam_states(event_count: usize) -> Option<Vec<&'static str>> {
    if event_count < 2 {
        return None;
    }
    let mut states = vec!["continue"; event_count];
    states[0] = "begin";
    states[event_count - 1] = "end";
    Some(states)
}

fn primary_beam_plan_fingerprint(primary_states: &[&str]) -> u64 {
    let mut material = String::new();
    for (event_ordinal, state) in primary_states.iter().enumerate() {
        let _ = write!(material, "{event_ordinal}:1:{state};");
    }
    fnv1a64(material.as_bytes())
}

fn build_canonical_primary_beam_owner_receipt(
    proof_chain_id: &str,
    event_count: usize,
) -> Option<CanonicalPrimaryBeamOwnerReceipt> {
    if !canonical_primary_beam_owner_source_is_bound() {
        return None;
    }
    let primary_states = current_canonical_primary_beam_states(event_count)?;
    let plan_fingerprint_fnv64 = primary_beam_plan_fingerprint(&primary_states);
    Some(CanonicalPrimaryBeamOwnerReceipt {
        proof_chain_id: proof_chain_id.to_string(),
        owner_kind: CANONICAL_PRIMARY_BEAM_OWNER_KIND,
        owner_source_fnv64: CANONICAL_PRIMARY_BEAM_OWNER_SOURCE_FNV64_CURRENT,
        event_count,
        plan_fingerprint_fnv64,
        primary_states,
    })
}

fn overlaps(left_start: i64, left_duration: i64, right_start: i64, right_duration: i64) -> bool {
    if left_duration <= 0 || right_duration <= 0 {
        return false;
    }
    let left_end = left_start.saturating_add(left_duration);
    let right_end = right_start.saturating_add(right_duration);
    left_start < right_end && right_start < left_end
}

fn classify_chain(
    chain: &VoiceFreePrimaryBeamChainEvidence,
    source_index: &BTreeMap<SourceAttackKey, Vec<&PreSerializationSourceAttackRow>>,
    occurrence_index: &BTreeMap<OccurrenceKey, Vec<&PreSerializationOccurrenceRow>>,
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
) -> ClassifiedChain {
    let empty = |status| ClassifiedChain {
        status,
        source_event_ids: Vec::new(),
        attack_ids: Vec::new(),
        technical_lanes: Vec::new(),
        current_voices: Vec::new(),
        lane_pressure_edges: Vec::new(),
        external_lane_pressure_source_ids: Vec::new(),
        external_pitched_atomic_groups: Vec::new(),
        rest_atomic_groups: Vec::new(),
        chord_extension_atomic_groups: Vec::new(),
        semantic_durations: Vec::new(),
        semantic_shape: String::new(),
    };

    if chain.members.len() < 2
        || chain.members.windows(2).any(|pair| {
            pair[0].measure != pair[1].measure
                || pair[0].staff != pair[1].staff
                || pair[0].onset.saturating_add(pair[0].duration) != pair[1].onset
        })
    {
        return empty(ChainStatus::BlockedNoncontiguousRepresentation);
    }

    let mut occurrences = Vec::<&PreSerializationOccurrenceRow>::with_capacity(chain.members.len());
    for member in &chain.members {
        let Some(rows) = occurrence_index.get(&(member.measure, member.pitched_ordinal)) else {
            return empty(ChainStatus::BlockedOccurrenceIdentity);
        };
        if rows.len() != 1 {
            return empty(ChainStatus::BlockedOccurrenceIdentity);
        }
        let occurrence = rows[0];
        if occurrence.represented_onset != member.onset
            || occurrence.represented_duration != member.duration
            || occurrence.pitch_midi != member.pitch_midi
            || occurrence.staff != member.staff
        {
            return empty(ChainStatus::BlockedOccurrenceIdentity);
        }
        occurrences.push(occurrence);
    }

    let mut sources = Vec::<&PreSerializationSourceAttackRow>::with_capacity(occurrences.len());
    for occurrence in &occurrences {
        let Some(rows) = source_index.get(&(occurrence.source_event_id, occurrence.attack_id)) else {
            return empty(ChainStatus::BlockedSourceAttackIdentity);
        };
        if rows.len() != 1 {
            return empty(ChainStatus::BlockedSourceAttackIdentity);
        }
        let source = rows[0];
        if source.measure != occurrence.measure
            || source.staff != occurrence.staff
            || source.pitch_midi != occurrence.pitch_midi
        {
            return empty(ChainStatus::BlockedSourceAttackIdentity);
        }
        sources.push(source);
    }

    let unique_source_attack = sources
        .iter()
        .map(|source| (source.source_event_id, source.attack_id))
        .collect::<BTreeSet<_>>();
    if unique_source_attack.len() != sources.len() {
        return empty(ChainStatus::BlockedNonInjectiveIdentity);
    }

    let source_event_ids = sources.iter().map(|row| row.source_event_id).collect::<Vec<_>>();
    let attack_ids = sources.iter().map(|row| row.attack_id).collect::<Vec<_>>();
    let technical_lanes = sources
        .iter()
        .map(|row| row.technical_lane)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let current_voices = occurrences
        .iter()
        .map(|row| row.current_voice.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let semantic_durations = sources
        .iter()
        .map(|row| row.semantic_duration)
        .collect::<Vec<_>>();
    let semantic_shape = chain
        .members
        .iter()
        .zip(sources.iter().copied())
        .map(|(member, source)| {
            format!(
                "m{}:s{}:{}+{}:p{}:sd{}:lane{}",
                member.measure,
                member.staff,
                member.onset,
                member.duration,
                member.pitch_midi,
                source.semantic_duration,
                source.technical_lane,
            )
        })
        .collect::<Vec<_>>()
        .join("|");

    if technical_lanes.len() <= 1 {
        return ClassifiedChain {
            status: ChainStatus::NotApplicableSingleTechnicalLane,
            source_event_ids,
            attack_ids,
            technical_lanes,
            current_voices,
            lane_pressure_edges: Vec::new(),
            external_lane_pressure_source_ids: Vec::new(),
            external_pitched_atomic_groups: Vec::new(),
            rest_atomic_groups: Vec::new(),
            chord_extension_atomic_groups: Vec::new(),
            semantic_durations,
            semantic_shape,
        };
    }

    if occurrences
        .iter()
        .zip(sources.iter())
        .any(|(occurrence, source)| occurrence.represented_onset != source.semantic_onset)
    {
        return ClassifiedChain {
            status: ChainStatus::BlockedSemanticTimeDivergence,
            source_event_ids,
            attack_ids,
            technical_lanes,
            current_voices,
            lane_pressure_edges: Vec::new(),
            external_lane_pressure_source_ids: Vec::new(),
            external_pitched_atomic_groups: Vec::new(),
            rest_atomic_groups: Vec::new(),
            chord_extension_atomic_groups: Vec::new(),
            semantic_durations,
            semantic_shape,
        };
    }

    let chain_identity = sources
        .iter()
        .map(|source| (source.source_event_id, source.attack_id))
        .collect::<BTreeSet<_>>();
    let all_sources = source_index
        .values()
        .flat_map(|rows| rows.iter().copied())
        .collect::<Vec<_>>();
    let mut lane_pressure_edges = Vec::<LanePressureEdge>::new();
    let mut external_lane_pressure_source_ids = BTreeSet::<usize>::new();
    for source in &sources {
        if source.technical_lane <= 1 {
            continue;
        }
        for required_lane in 1..source.technical_lane {
            let blockers = all_sources
                .iter()
                .copied()
                .filter(|candidate| {
                    candidate.staff == source.staff
                        && candidate.technical_lane == required_lane
                        && candidate.absolute_start <= source.absolute_start
                        && source.absolute_start < candidate.absolute_end
                })
                .collect::<Vec<_>>();
            if blockers.is_empty() {
                return ClassifiedChain {
                    status: ChainStatus::BlockedLanePressureWitness,
                    source_event_ids,
                    attack_ids,
                    technical_lanes,
                    current_voices,
                    lane_pressure_edges,
                    external_lane_pressure_source_ids: external_lane_pressure_source_ids
                        .into_iter()
                        .collect(),
                    external_pitched_atomic_groups: Vec::new(),
                    rest_atomic_groups: Vec::new(),
                    chord_extension_atomic_groups: Vec::new(),
                    semantic_durations,
                    semantic_shape,
                };
            }
            for blocker in blockers {
                if !chain_identity.contains(&(blocker.source_event_id, blocker.attack_id)) {
                    external_lane_pressure_source_ids.insert(blocker.source_event_id);
                }
                lane_pressure_edges.push(LanePressureEdge {
                    blocker_source_event_id: blocker.source_event_id,
                    blocker_attack_id: blocker.attack_id,
                    blocker_lane: blocker.technical_lane,
                    blocker_absolute_end: blocker.absolute_end,
                    blocked_source_event_id: source.source_event_id,
                    blocked_attack_id: source.attack_id,
                    blocked_lane: source.technical_lane,
                    blocked_absolute_start: source.absolute_start,
                });
            }
        }
    }
    lane_pressure_edges.sort_by_key(|edge| {
        (
            edge.blocked_absolute_start,
            edge.blocked_lane,
            edge.blocked_source_event_id,
            edge.blocker_lane,
            edge.blocker_source_event_id,
        )
    });
    lane_pressure_edges.dedup();
    if !external_lane_pressure_source_ids.is_empty() {
        return ClassifiedChain {
            status: ChainStatus::BlockedExternalLanePressure,
            source_event_ids,
            attack_ids,
            technical_lanes,
            current_voices,
            lane_pressure_edges,
            external_lane_pressure_source_ids: external_lane_pressure_source_ids
                .into_iter()
                .collect(),
            external_pitched_atomic_groups: Vec::new(),
            rest_atomic_groups: Vec::new(),
            chord_extension_atomic_groups: Vec::new(),
            semantic_durations,
            semantic_shape,
        };
    }

    let first = &chain.members[0];
    let last = &chain.members[chain.members.len() - 1];
    let interval_start = first.onset;
    let interval_duration = last
        .onset
        .saturating_add(last.duration)
        .saturating_sub(interval_start);
    let mut occupancy_groups = BTreeMap::<usize, Vec<&PreSerializationVisibleOccupancyRow>>::new();
    for row in occupancy_rows.iter().filter(|row| {
        row.visible
            && row.measure == first.measure
            && row.staff == first.staff
            && overlaps(
                row.represented_onset,
                row.represented_duration,
                interval_start,
                interval_duration,
            )
    }) {
        occupancy_groups.entry(row.atomic_group_id).or_default().push(row);
    }

    let mut external_pitched_atomic_groups = BTreeSet::<usize>::new();
    let mut rest_atomic_groups = BTreeSet::<usize>::new();
    let mut chord_extension_atomic_groups = BTreeSet::<usize>::new();
    for (&group_id, rows) in &occupancy_groups {
        if rows.iter().any(|row| row.is_rest) {
            rest_atomic_groups.insert(group_id);
        }
        let pitched = rows
            .iter()
            .copied()
            .filter(|row| !row.is_rest && row.pitch_midi.is_some())
            .collect::<Vec<_>>();
        if pitched.is_empty() {
            continue;
        }
        let group_identities = pitched
            .iter()
            .filter_map(|row| Some((row.source_event_id?, row.attack_id?)))
            .collect::<BTreeSet<_>>();
        let touches_chain = group_identities
            .iter()
            .any(|identity| chain_identity.contains(identity));
        if touches_chain {
            if pitched.len() > 1 && pitched.iter().any(|row| row.is_chord_member) {
                chord_extension_atomic_groups.insert(group_id);
            }
        } else {
            external_pitched_atomic_groups.insert(group_id);
        }
    }

    let status = if !external_pitched_atomic_groups.is_empty() {
        ChainStatus::BlockedIndependentPitchedOccupancy
    } else if !rest_atomic_groups.is_empty() {
        ChainStatus::BlockedRestOccupancyOwnership
    } else {
        ChainStatus::CandidateClosedExceptBeamProvenanceAndMaterialization
    };

    ClassifiedChain {
        status,
        source_event_ids,
        attack_ids,
        technical_lanes,
        current_voices,
        lane_pressure_edges,
        external_lane_pressure_source_ids: Vec::new(),
        external_pitched_atomic_groups: external_pitched_atomic_groups.into_iter().collect(),
        rest_atomic_groups: rest_atomic_groups.into_iter().collect(),
        chord_extension_atomic_groups: chord_extension_atomic_groups.into_iter().collect(),
        semantic_durations,
        semantic_shape,
    }
}

type VoiceFreeBeamChainShape = Vec<(usize, u8, i64, i64, i32)>;

fn voice_free_beam_chain_shape(chain: &VoiceFreePrimaryBeamChainEvidence) -> VoiceFreeBeamChainShape {
    chain
        .members
        .iter()
        .map(|member| {
            (
                member.measure,
                member.staff,
                member.onset,
                member.duration,
                member.pitch_midi,
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct MetricNotationAtom {
    atomic_group_id: usize,
    measure: usize,
    staff: u8,
    onset: i64,
    duration: i64,
    pitch_midi: i32,
    member: PreSerializationClosedSameLineMemberBinding,
}

#[derive(Debug, Default)]
struct MetricNotationLineAudit {
    parse_ok: bool,
    short_atomic_groups: usize,
    multi_member_atomic_groups_excluded: usize,
    metric_scopes_with_multiple_atoms: usize,
    noncontiguous_scopes: usize,
    single_lane_scopes: usize,
    blocked_identity: usize,
    blocked_semantic_time: usize,
    blocked_lane_pressure: usize,
    blocked_external_lane_pressure: usize,
    blocked_external_pitched_occupancy: usize,
    blocked_rest_occupancy: usize,
    closed_candidates: Vec<PreSerializationMetricNotationLineCandidate>,
    rows: Vec<String>,
}

fn measure_divisions_by_number(xml: &str) -> Option<BTreeMap<usize, i64>> {
    let document = roxmltree::Document::parse(xml).ok()?;
    let mut out = BTreeMap::<usize, i64>::new();
    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
    {
        let mut inherited_divisions = 1_i64;
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
            .enumerate()
        {
            if let Some(divisions) = measure
                .descendants()
                .find(|node| node.is_element() && node.has_tag_name("divisions"))
                .and_then(|node| node.text())
                .and_then(|value| value.trim().parse::<i64>().ok())
            {
                inherited_divisions = divisions.max(1);
            }
            let measure_number = measure
                .attribute("number")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(measure_index.saturating_add(1));
            if let Some(previous) = out.insert(measure_number, inherited_divisions)
                && previous != inherited_divisions
            {
                return None;
            }
        }
    }
    (!out.is_empty()).then_some(out)
}

fn build_metric_notation_line_audit(
    projected_final_chord_xml: &str,
    source_rows: &[PreSerializationSourceAttackRow],
    occurrence_rows: &[PreSerializationOccurrenceRow],
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
) -> MetricNotationLineAudit {
    let Some(divisions_by_measure) = measure_divisions_by_number(projected_final_chord_xml) else {
        return MetricNotationLineAudit {
            parse_ok: false,
            ..MetricNotationLineAudit::default()
        };
    };

    let mut audit = MetricNotationLineAudit {
        parse_ok: true,
        ..MetricNotationLineAudit::default()
    };
    let mut source_index = BTreeMap::<SourceAttackKey, Vec<&PreSerializationSourceAttackRow>>::new();
    for row in source_rows {
        source_index
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }
    let all_sources = source_rows.iter().collect::<Vec<_>>();

    let mut occupancy_groups = BTreeMap::<usize, Vec<&PreSerializationVisibleOccupancyRow>>::new();
    for row in occupancy_rows {
        occupancy_groups.entry(row.atomic_group_id).or_default().push(row);
    }

    let mut atoms = Vec::<MetricNotationAtom>::new();
    for (&group_id, rows) in &occupancy_groups {
        let visible_pitched = rows
            .iter()
            .copied()
            .filter(|row| row.visible && !row.is_rest && row.pitch_midi.is_some())
            .collect::<Vec<_>>();
        if visible_pitched.is_empty() {
            continue;
        }
        let identities = visible_pitched
            .iter()
            .filter_map(|row| Some((row.source_event_id?, row.attack_id?)))
            .collect::<BTreeSet<_>>();
        if visible_pitched.len() != 1
            || identities.len() != 1
            || visible_pitched.iter().any(|row| {
                row.is_chord_member
                    || row.atomic_group_kind
                        == PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor
            })
        {
            audit.multi_member_atomic_groups_excluded =
                audit.multi_member_atomic_groups_excluded.saturating_add(1);
            continue;
        }
        let row = visible_pitched[0];
        let Some(source_event_id) = row.source_event_id else {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        };
        let Some(attack_id) = row.attack_id else {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        };
        let Some(pitch_midi) = row.pitch_midi else {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        };
        let matching_occurrences = occurrence_rows
            .iter()
            .filter(|occurrence| {
                occurrence.source_event_id == source_event_id
                    && occurrence.attack_id == attack_id
                    && occurrence.measure == row.measure
                    && occurrence.staff == row.staff
                    && occurrence.represented_onset == row.represented_onset
                    && occurrence.represented_duration == row.represented_duration
                    && occurrence.pitch_midi == pitch_midi
            })
            .collect::<Vec<_>>();
        if matching_occurrences.len() != 1 {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        }
        let occurrence = matching_occurrences[0];
        let Some(&divisions) = divisions_by_measure.get(&row.measure) else {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        };
        if row.represented_duration <= 0 || row.represented_duration > divisions.max(1) / 2 {
            continue;
        }
        let beat = row.represented_onset.div_euclid(divisions.max(1));
        let beat_start = beat.saturating_mul(divisions.max(1));
        let beat_end = beat_start.saturating_add(divisions.max(1));
        if row.represented_onset < beat_start
            || row
                .represented_onset
                .saturating_add(row.represented_duration)
                > beat_end
        {
            continue;
        }
        audit.short_atomic_groups = audit.short_atomic_groups.saturating_add(1);
        atoms.push(MetricNotationAtom {
            atomic_group_id: group_id,
            measure: row.measure,
            staff: row.staff,
            onset: row.represented_onset,
            duration: row.represented_duration,
            pitch_midi,
            member: PreSerializationClosedSameLineMemberBinding {
                source_event_id,
                attack_id,
                measure: row.measure,
                pitched_ordinal: occurrence.pitched_ordinal,
                represented_onset: row.represented_onset,
                represented_duration: row.represented_duration,
                staff: row.staff,
            },
        });
    }

    let mut by_scope = BTreeMap::<(usize, u8, i64), Vec<MetricNotationAtom>>::new();
    for atom in atoms {
        let Some(&divisions) = divisions_by_measure.get(&atom.measure) else {
            continue;
        };
        let beat = atom.onset.div_euclid(divisions.max(1));
        by_scope
            .entry((atom.measure, atom.staff, beat))
            .or_default()
            .push(atom);
    }

    for ((measure, staff, beat), mut group) in by_scope {
        if group.len() < 2 {
            continue;
        }
        audit.metric_scopes_with_multiple_atoms =
            audit.metric_scopes_with_multiple_atoms.saturating_add(1);
        group.sort_by_key(|atom| {
            (
                atom.onset,
                atom.duration,
                atom.member.source_event_id,
                atom.member.attack_id,
            )
        });
        if group
            .windows(2)
            .any(|pair| pair[0].onset == pair[1].onset
                || pair[0].onset.saturating_add(pair[0].duration) != pair[1].onset)
        {
            audit.noncontiguous_scopes = audit.noncontiguous_scopes.saturating_add(1);
            continue;
        }
        let Some(&divisions) = divisions_by_measure.get(&measure) else {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        };
        let beat_start = beat.saturating_mul(divisions.max(1));
        let beat_end = beat_start.saturating_add(divisions.max(1));
        let interval_start = group[0].onset;
        let interval_end = group
            .last()
            .map(|atom| atom.onset.saturating_add(atom.duration))
            .unwrap_or(interval_start);
        if interval_start < beat_start || interval_end > beat_end {
            audit.noncontiguous_scopes = audit.noncontiguous_scopes.saturating_add(1);
            continue;
        }

        let mut sources = Vec::<&PreSerializationSourceAttackRow>::new();
        let mut identity_failure = false;
        for atom in &group {
            let identity = (atom.member.source_event_id, atom.member.attack_id);
            let Some(rows) = source_index.get(&identity) else {
                identity_failure = true;
                break;
            };
            if rows.len() != 1 {
                identity_failure = true;
                break;
            }
            let source = rows[0];
            if source.measure != measure
                || source.staff != staff
                || source.pitch_midi != atom.pitch_midi
            {
                identity_failure = true;
                break;
            }
            sources.push(source);
        }
        if identity_failure {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        }
        let identities = sources
            .iter()
            .map(|source| (source.source_event_id, source.attack_id))
            .collect::<BTreeSet<_>>();
        if identities.len() != sources.len() {
            audit.blocked_identity = audit.blocked_identity.saturating_add(1);
            continue;
        }
        let technical_lanes = sources
            .iter()
            .map(|source| source.technical_lane)
            .collect::<BTreeSet<_>>();
        if technical_lanes.len() <= 1 {
            audit.single_lane_scopes = audit.single_lane_scopes.saturating_add(1);
            continue;
        }
        if group
            .iter()
            .zip(&sources)
            .any(|(atom, source)| atom.onset != source.semantic_onset)
        {
            audit.blocked_semantic_time = audit.blocked_semantic_time.saturating_add(1);
            continue;
        }

        let candidate_group_ids = group
            .iter()
            .map(|atom| atom.atomic_group_id)
            .collect::<BTreeSet<_>>();
        let interval_duration = interval_end.saturating_sub(interval_start);
        let mut has_external_pitched = false;
        let mut has_rest = false;
        for (&group_id, rows) in &occupancy_groups {
            if candidate_group_ids.contains(&group_id) {
                continue;
            }
            let touches_interval = rows.iter().any(|row| {
                row.visible
                    && row.measure == measure
                    && row.staff == staff
                    && overlaps(
                        row.represented_onset,
                        row.represented_duration,
                        interval_start,
                        interval_duration,
                    )
            });
            if !touches_interval {
                continue;
            }
            if rows.iter().any(|row| row.visible && row.is_rest) {
                has_rest = true;
            }
            if rows
                .iter()
                .any(|row| row.visible && !row.is_rest && row.pitch_midi.is_some())
            {
                has_external_pitched = true;
            }
        }
        if has_external_pitched {
            audit.blocked_external_pitched_occupancy =
                audit.blocked_external_pitched_occupancy.saturating_add(1);
            continue;
        }
        if has_rest {
            audit.blocked_rest_occupancy = audit.blocked_rest_occupancy.saturating_add(1);
            continue;
        }

        let mut lane_pressure_edges = Vec::<LanePressureEdge>::new();
        let mut pressure_missing = false;
        let mut external_pressure = BTreeSet::<usize>::new();
        for source in &sources {
            if source.technical_lane <= 1 {
                continue;
            }
            for required_lane in 1..source.technical_lane {
                let blockers = all_sources
                    .iter()
                    .copied()
                    .filter(|candidate| {
                        candidate.staff == source.staff
                            && candidate.technical_lane == required_lane
                            && candidate.absolute_start <= source.absolute_start
                            && source.absolute_start < candidate.absolute_end
                    })
                    .collect::<Vec<_>>();
                if blockers.is_empty() {
                    pressure_missing = true;
                    continue;
                }
                for blocker in blockers {
                    if !identities.contains(&(blocker.source_event_id, blocker.attack_id)) {
                        external_pressure.insert(blocker.source_event_id);
                    }
                    lane_pressure_edges.push(LanePressureEdge {
                        blocker_source_event_id: blocker.source_event_id,
                        blocker_attack_id: blocker.attack_id,
                        blocker_lane: blocker.technical_lane,
                        blocker_absolute_end: blocker.absolute_end,
                        blocked_source_event_id: source.source_event_id,
                        blocked_attack_id: source.attack_id,
                        blocked_lane: source.technical_lane,
                        blocked_absolute_start: source.absolute_start,
                    });
                }
            }
        }
        if pressure_missing || lane_pressure_edges.is_empty() {
            audit.blocked_lane_pressure = audit.blocked_lane_pressure.saturating_add(1);
            continue;
        }
        if !external_pressure.is_empty() {
            audit.blocked_external_lane_pressure =
                audit.blocked_external_lane_pressure.saturating_add(1);
            continue;
        }
        lane_pressure_edges.sort_by_key(|edge| {
            (
                edge.blocked_absolute_start,
                edge.blocked_lane,
                edge.blocked_source_event_id,
                edge.blocker_lane,
                edge.blocker_source_event_id,
            )
        });
        lane_pressure_edges.dedup();

        let source_event_ids = sources
            .iter()
            .map(|source| source.source_event_id)
            .collect::<Vec<_>>();
        let id_material = sources
            .iter()
            .map(|source| format!("{}:{}", source.source_event_id, source.attack_id))
            .collect::<Vec<_>>()
            .join("|");
        let metric_line_id = format!("metric-{:016x}", fnv1a64(id_material.as_bytes()));
        let lane_edges = lane_pressure_edges
            .iter()
            .map(|edge| {
                format!(
                    "{}:{}:lane{}:end{}>{}:{}:lane{}:start{}",
                    edge.blocker_source_event_id,
                    edge.blocker_attack_id,
                    edge.blocker_lane,
                    edge.blocker_absolute_end,
                    edge.blocked_source_event_id,
                    edge.blocked_attack_id,
                    edge.blocked_lane,
                    edge.blocked_absolute_start,
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        audit.rows.push(format!(
            "METRIC_NOTATION_LINE metric_line_id={} measure={} staff={} beat={} interval={}-{} source_ids=[{}] attack_ids=[{}] technical_lanes={} lane_pressure_edges=[{}] selection_voice_labels=ignored selection_beams=ignored atomic_chord_policy=multi_visible_member_atom_blocked_for_first_writer_domain status=ELIGIBLE_ANALYSIS_ONLY",
            metric_line_id,
            measure,
            staff,
            beat,
            interval_start,
            interval_end,
            source_event_ids
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            sources
                .iter()
                .map(|source| source.attack_id.to_string())
                .collect::<Vec<_>>()
                .join(","),
            technical_lanes
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            lane_edges,
        ));
        audit.closed_candidates.push(PreSerializationMetricNotationLineCandidate {
            metric_line_id,
            source_event_ids,
            members: group.iter().map(|atom| atom.member.clone()).collect(),
            measure,
            staff,
            beat,
            interval_start,
            interval_end,
        });
    }

    audit.rows.sort();
    audit.rows.dedup();
    audit
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum VisibleRhythmicOccupancyAtomKind {
    PitchedNote,
    PitchedChord,
    Rest,
}

impl VisibleRhythmicOccupancyAtomKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::PitchedNote => "PITCHED_NOTE",
            Self::PitchedChord => "PITCHED_CHORD",
            Self::Rest => "REST",
        }
    }

    pub(crate) const fn is_pitched(self) -> bool {
        matches!(self, Self::PitchedNote | Self::PitchedChord)
    }
}

#[derive(Debug, Clone)]
struct VisibleRhythmicOccupancyAtom {
    atomic_group_id: usize,
    measure: usize,
    staff: u8,
    onset: i64,
    duration: i64,
    kind: VisibleRhythmicOccupancyAtomKind,
    source_attack_ids: Vec<(usize, usize)>,
    pitches: Vec<i32>,
}

#[derive(Debug, Default)]
struct VisibleRhythmicOccupancyClosureAudit {
    metric_parse_ok: bool,
    visible_atomic_groups: usize,
    hidden_only_atomic_groups: usize,
    incomplete_atomic_groups: usize,
    pitched_note_atoms: usize,
    pitched_chord_atoms: usize,
    rest_atoms: usize,
    duplicate_rest_pairs: usize,
    rest_pitched_overlap_pairs: usize,
    independent_pitched_overlap_pairs: usize,
    metric_scopes_with_multiple_atoms: usize,
    metric_scopes_contiguous_without_overlap: usize,
    metric_scopes_with_rest: usize,
    metric_scopes_with_visible_overlap: usize,
    rows: Vec<String>,
    phase_snapshot: VisibleRhythmicOccupancyPhaseSnapshot,
}

fn same_visible_interval(
    rows: &[&PreSerializationVisibleOccupancyRow],
    onset: i64,
    duration: i64,
) -> bool {
    rows.iter()
        .all(|row| row.represented_onset == onset && row.represented_duration == duration)
}

fn build_visible_rhythmic_occupancy_closure_audit(
    projected_final_chord_xml: &str,
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
    require_pitched_identity: bool,
) -> VisibleRhythmicOccupancyClosureAudit {
    let divisions_by_measure = measure_divisions_by_number(projected_final_chord_xml);
    let mut audit = VisibleRhythmicOccupancyClosureAudit {
        metric_parse_ok: divisions_by_measure.is_some(),
        ..VisibleRhythmicOccupancyClosureAudit::default()
    };
    audit.phase_snapshot.metric_parse_ok = audit.metric_parse_ok;

    let mut grouped = BTreeMap::<usize, Vec<&PreSerializationVisibleOccupancyRow>>::new();
    for row in occupancy_rows {
        grouped.entry(row.atomic_group_id).or_default().push(row);
    }

    let mut atoms = Vec::<VisibleRhythmicOccupancyAtom>::new();
    for (&group_id, rows) in &grouped {
        let visible = rows
            .iter()
            .copied()
            .filter(|row| row.visible)
            .collect::<Vec<_>>();
        if visible.is_empty() {
            audit.hidden_only_atomic_groups = audit.hidden_only_atomic_groups.saturating_add(1);
            continue;
        }
        audit.visible_atomic_groups = audit.visible_atomic_groups.saturating_add(1);

        let locations = visible
            .iter()
            .map(|row| (row.measure, row.staff))
            .collect::<BTreeSet<_>>();
        if locations.len() != 1 || visible.iter().any(|row| row.represented_duration <= 0) {
            audit.incomplete_atomic_groups = audit.incomplete_atomic_groups.saturating_add(1);
            audit.phase_snapshot.incomplete_group_ids.insert(group_id);
            audit.rows.push(format!(
                "VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE group={} reason=location_or_duration",
                group_id
            ));
            continue;
        }
        let (measure, staff) = *locations
            .iter()
            .next()
            .expect("visible occupancy location set cannot be empty");
        let visible_rests = visible
            .iter()
            .copied()
            .filter(|row| row.is_rest)
            .collect::<Vec<_>>();
        let visible_pitched = visible
            .iter()
            .copied()
            .filter(|row| !row.is_rest && row.pitch_midi.is_some())
            .collect::<Vec<_>>();
        let visible_other = visible
            .len()
            .saturating_sub(visible_rests.len())
            .saturating_sub(visible_pitched.len());

        if (!visible_rests.is_empty() && !visible_pitched.is_empty()) || visible_other > 0 {
            audit.incomplete_atomic_groups = audit.incomplete_atomic_groups.saturating_add(1);
            audit.phase_snapshot.incomplete_group_ids.insert(group_id);
            audit.rows.push(format!(
                "VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE group={} measure={} staff={} reason=mixed_visible_kinds",
                group_id, measure, staff
            ));
            continue;
        }

        if !visible_rests.is_empty() {
            let first = visible_rests[0];
            if visible_rests.len() != visible.len()
                || !same_visible_interval(
                    &visible_rests,
                    first.represented_onset,
                    first.represented_duration,
                )
            {
                audit.incomplete_atomic_groups = audit.incomplete_atomic_groups.saturating_add(1);
                audit.phase_snapshot.incomplete_group_ids.insert(group_id);
                audit.rows.push(format!(
                    "VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE group={} measure={} staff={} reason=non_atomic_rest_group",
                    group_id, measure, staff
                ));
                continue;
            }
            audit.rest_atoms = audit.rest_atoms.saturating_add(1);
            atoms.push(VisibleRhythmicOccupancyAtom {
                atomic_group_id: group_id,
                measure,
                staff,
                onset: first.represented_onset,
                duration: first.represented_duration,
                kind: VisibleRhythmicOccupancyAtomKind::Rest,
                source_attack_ids: Vec::new(),
                pitches: Vec::new(),
            });
            continue;
        }

        if visible_pitched.is_empty() {
            audit.incomplete_atomic_groups = audit.incomplete_atomic_groups.saturating_add(1);
            audit.phase_snapshot.incomplete_group_ids.insert(group_id);
            audit.rows.push(format!(
                "VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE group={} measure={} staff={} reason=no_visible_rhythmic_kind",
                group_id, measure, staff
            ));
            continue;
        }

        let first = visible_pitched[0];
        let anchor_count = visible_pitched
            .iter()
            .filter(|row| !row.is_chord_member)
            .count();
        let orphan_chord = visible_pitched.iter().any(|row| {
            row.atomic_group_kind == PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor
        });
        if visible_pitched.len() != visible.len()
            || anchor_count != 1
            || orphan_chord
            || !same_visible_interval(
                &visible_pitched,
                first.represented_onset,
                first.represented_duration,
            )
        {
            audit.incomplete_atomic_groups = audit.incomplete_atomic_groups.saturating_add(1);
            audit.phase_snapshot.incomplete_group_ids.insert(group_id);
            audit.rows.push(format!(
                "VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE group={} measure={} staff={} reason=pitched_atom_not_anchor_closed visible_pitched_members={} visible_anchors={} orphan_chord={}",
                group_id,
                measure,
                staff,
                visible_pitched.len(),
                anchor_count,
                if orphan_chord { "yes" } else { "no" },
            ));
            continue;
        }

        let mut source_attack_ids = Vec::<(usize, usize)>::new();
        let mut pitches = Vec::<i32>::new();
        let mut identity_complete = true;
        for row in &visible_pitched {
            if let Some(pitch) = row.pitch_midi {
                pitches.push(pitch);
            } else {
                identity_complete = false;
            }
            if let (Some(source_event_id), Some(attack_id)) = (row.source_event_id, row.attack_id) {
                source_attack_ids.push((source_event_id, attack_id));
            } else {
                identity_complete = false;
            }
        }
        if require_pitched_identity && !identity_complete {
            audit.incomplete_atomic_groups = audit.incomplete_atomic_groups.saturating_add(1);
            audit.phase_snapshot.incomplete_group_ids.insert(group_id);
            audit.rows.push(format!(
                "VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE group={} measure={} staff={} reason=incomplete_pitched_identity",
                group_id, measure, staff
            ));
            continue;
        }
        source_attack_ids.sort();
        source_attack_ids.dedup();
        pitches.sort();
        pitches.dedup();
        let kind = if visible_pitched.len() == 1 {
            audit.pitched_note_atoms = audit.pitched_note_atoms.saturating_add(1);
            VisibleRhythmicOccupancyAtomKind::PitchedNote
        } else {
            audit.pitched_chord_atoms = audit.pitched_chord_atoms.saturating_add(1);
            VisibleRhythmicOccupancyAtomKind::PitchedChord
        };
        atoms.push(VisibleRhythmicOccupancyAtom {
            atomic_group_id: group_id,
            measure,
            staff,
            onset: first.represented_onset,
            duration: first.represented_duration,
            kind,
            source_attack_ids,
            pitches,
        });
    }

    atoms.sort_by_key(|atom| {
        (
            atom.measure,
            atom.staff,
            atom.onset,
            atom.duration,
            atom.atomic_group_id,
        )
    });
    audit.phase_snapshot.visible_atoms = atoms
        .iter()
        .map(|atom| VisibleRhythmicOccupancyPhaseAtom {
            measure: atom.measure,
            staff: atom.staff,
            onset: atom.onset,
            duration: atom.duration,
            kind: atom.kind,
            pitches: atom.pitches.clone(),
        })
        .collect();

    for atom in &atoms {
        let source_attacks = atom
            .source_attack_ids
            .iter()
            .map(|(source, attack)| format!("{}:{}", source, attack))
            .collect::<Vec<_>>()
            .join(",");
        let pitches = atom
            .pitches
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        audit.rows.push(format!(
            "VISIBLE_RHYTHMIC_OCCUPANCY_ATOM group={} measure={} staff={} interval={}-{} kind={} source_attacks=[{}] pitches=[{}]",
            atom.atomic_group_id,
            atom.measure,
            atom.staff,
            atom.onset,
            atom.onset.saturating_add(atom.duration),
            atom.kind.label(),
            source_attacks,
            pitches,
        ));
    }

    let mut atoms_by_measure_staff =
        BTreeMap::<(usize, u8), Vec<&VisibleRhythmicOccupancyAtom>>::new();
    for atom in &atoms {
        atoms_by_measure_staff
            .entry((atom.measure, atom.staff))
            .or_default()
            .push(atom);
    }
    for ((_measure, _staff), scope_atoms) in atoms_by_measure_staff {
        for left_index in 0..scope_atoms.len() {
            for right_index in left_index + 1..scope_atoms.len() {
                let left = scope_atoms[left_index];
                let right = scope_atoms[right_index];
                if left.kind == VisibleRhythmicOccupancyAtomKind::Rest
                    && right.kind == VisibleRhythmicOccupancyAtomKind::Rest
                    && left.onset == right.onset
                    && left.duration == right.duration
                {
                    audit.duplicate_rest_pairs = audit.duplicate_rest_pairs.saturating_add(1);
                    audit.phase_snapshot.duplicate_rest_conflicts.insert(
                        VisibleRhythmicOccupancyDuplicateRestConflict {
                            measure: left.measure,
                            staff: left.staff,
                            onset: left.onset,
                            duration: left.duration,
                        },
                    );
                    audit.rows.push(format!(
                        "VISIBLE_RHYTHMIC_OCCUPANCY_CONFLICT kind=DUPLICATE_REST_OCCUPANCY left_group={} right_group={} measure={} staff={} interval={}-{} status=REST_OWNERSHIP_CONFLICT_ANALYSIS_ONLY",
                        left.atomic_group_id,
                        right.atomic_group_id,
                        left.measure,
                        left.staff,
                        left.onset,
                        left.onset.saturating_add(left.duration),
                    ));
                    continue;
                }
                if !overlaps(left.onset, left.duration, right.onset, right.duration) {
                    continue;
                }
                let left_pitched = left.kind.is_pitched();
                let right_pitched = right.kind.is_pitched();
                if left_pitched != right_pitched {
                    let (rest, pitched) =
                        if left_pitched { (right, left) } else { (left, right) };
                    audit.rest_pitched_overlap_pairs =
                        audit.rest_pitched_overlap_pairs.saturating_add(1);
                    audit.phase_snapshot.rest_pitched_conflicts.insert(
                        VisibleRhythmicOccupancyRestPitchedConflict {
                            measure: rest.measure,
                            staff: rest.staff,
                            rest_onset: rest.onset,
                            rest_duration: rest.duration,
                            pitched_onset: pitched.onset,
                            pitched_duration: pitched.duration,
                            pitches: pitched.pitches.clone(),
                        },
                    );
                    audit.rows.push(format!(
                        "VISIBLE_RHYTHMIC_OCCUPANCY_CONFLICT kind=REST_PITCHED_OVERLAP rest_group={} pitched_group={} measure={} staff={} rest_interval={}-{} pitched_interval={}-{} status=REST_OWNERSHIP_CONFLICT_ANALYSIS_ONLY",
                        rest.atomic_group_id,
                        pitched.atomic_group_id,
                        rest.measure,
                        rest.staff,
                        rest.onset,
                        rest.onset.saturating_add(rest.duration),
                        pitched.onset,
                        pitched.onset.saturating_add(pitched.duration),
                    ));
                    continue;
                }
                if require_pitched_identity && left_pitched && right_pitched {
                    let left_ids = left
                        .source_attack_ids
                        .iter()
                        .copied()
                        .collect::<BTreeSet<_>>();
                    let right_ids = right
                        .source_attack_ids
                        .iter()
                        .copied()
                        .collect::<BTreeSet<_>>();
                    if left_ids.iter().all(|identity| !right_ids.contains(identity)) {
                        audit.independent_pitched_overlap_pairs =
                            audit.independent_pitched_overlap_pairs.saturating_add(1);
                        audit.rows.push(format!(
                            "VISIBLE_RHYTHMIC_OCCUPANCY_CONFLICT kind=INDEPENDENT_PITCHED_OVERLAP left_group={} right_group={} measure={} staff={} left_interval={}-{} right_interval={}-{} status=HARD_DIFFERENT_CANDIDATE_ANALYSIS_ONLY",
                            left.atomic_group_id,
                            right.atomic_group_id,
                            left.measure,
                            left.staff,
                            left.onset,
                            left.onset.saturating_add(left.duration),
                            right.onset,
                            right.onset.saturating_add(right.duration),
                        ));
                    }
                }
            }
        }
    }

    if let Some(divisions_by_measure) = divisions_by_measure {
        let mut scopes = BTreeMap::<(usize, u8, i64), Vec<&VisibleRhythmicOccupancyAtom>>::new();
        for atom in &atoms {
            let Some(&divisions) = divisions_by_measure.get(&atom.measure) else {
                continue;
            };
            let divisions = divisions.max(1);
            let beat = atom.onset.div_euclid(divisions);
            let beat_start = beat.saturating_mul(divisions);
            let beat_end = beat_start.saturating_add(divisions);
            if atom.onset < beat_start || atom.onset.saturating_add(atom.duration) > beat_end {
                continue;
            }
            scopes
                .entry((atom.measure, atom.staff, beat))
                .or_default()
                .push(atom);
        }
        for ((_measure, _staff, _beat), mut scope) in scopes {
            if scope.len() < 2 {
                continue;
            }
            audit.metric_scopes_with_multiple_atoms =
                audit.metric_scopes_with_multiple_atoms.saturating_add(1);
            scope.sort_by_key(|atom| (atom.onset, atom.duration, atom.atomic_group_id));
            if scope
                .iter()
                .any(|atom| atom.kind == VisibleRhythmicOccupancyAtomKind::Rest)
            {
                audit.metric_scopes_with_rest = audit.metric_scopes_with_rest.saturating_add(1);
            }
            let mut has_overlap = false;
            for left_index in 0..scope.len() {
                for right_index in left_index.saturating_add(1)..scope.len() {
                    if overlaps(
                        scope[left_index].onset,
                        scope[left_index].duration,
                        scope[right_index].onset,
                        scope[right_index].duration,
                    ) {
                        has_overlap = true;
                    }
                }
            }
            if has_overlap {
                audit.metric_scopes_with_visible_overlap =
                    audit.metric_scopes_with_visible_overlap.saturating_add(1);
            } else if scope.windows(2).all(|pair| {
                pair[0].onset.saturating_add(pair[0].duration) == pair[1].onset
            }) {
                audit.metric_scopes_contiguous_without_overlap =
                    audit.metric_scopes_contiguous_without_overlap.saturating_add(1);
            }
        }
    }

    audit.phase_snapshot.incomplete_groups = audit
        .rows
        .iter()
        .filter(|row| row.starts_with("VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE "))
        .cloned()
        .collect();
    for rows in grouped.values() {
        if rows.is_empty() || rows.iter().any(|row| row.visible) {
            continue;
        }
        let first = rows[0];
        if first.represented_duration <= 0
            || rows.iter().any(|row| {
                row.measure != first.measure
                    || row.staff != first.staff
                    || row.represented_onset != first.represented_onset
                    || row.represented_duration != first.represented_duration
            })
        {
            continue;
        }
        let all_rest = rows.iter().all(|row| row.is_rest);
        let all_pitched = rows
            .iter()
            .all(|row| !row.is_rest && row.pitch_midi.is_some());
        let kind = if all_rest {
            Some(VisibleRhythmicOccupancyAtomKind::Rest)
        } else if all_pitched {
            let anchors = rows.iter().filter(|row| !row.is_chord_member).count();
            let orphan = rows.iter().any(|row| {
                row.atomic_group_kind
                    == PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor
            });
            if anchors != 1 || orphan {
                None
            } else if rows.len() == 1 {
                Some(VisibleRhythmicOccupancyAtomKind::PitchedNote)
            } else {
                Some(VisibleRhythmicOccupancyAtomKind::PitchedChord)
            }
        } else {
            None
        };
        let Some(kind) = kind else {
            continue;
        };
        let mut pitches = rows.iter().filter_map(|row| row.pitch_midi).collect::<Vec<_>>();
        pitches.sort();
        pitches.dedup();
        audit.phase_snapshot.hidden_atoms.push(VisibleRhythmicOccupancyPhaseAtom {
            measure: first.measure,
            staff: first.staff,
            onset: first.represented_onset,
            duration: first.represented_duration,
            kind,
            pitches,
        });
    }
    audit.phase_snapshot.hidden_atoms.sort_by_key(|atom| {
        (atom.measure, atom.staff, atom.onset, atom.duration, atom.kind, atom.pitches.clone())
    });
    audit.rows.sort();
    audit.rows.dedup();
    audit
}

pub(crate) fn build_visible_rhythmic_occupancy_phase_snapshot(
    xml: &str,
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
) -> VisibleRhythmicOccupancyPhaseSnapshot {
    build_visible_rhythmic_occupancy_closure_audit(xml, occupancy_rows, false).phase_snapshot
}

#[cfg(test)]
pub(crate) fn build_notation_voice_pre_serialization_constraint_closure_audit_projection(
    projected_final_chord_xml: &str,
    source_rows: &[PreSerializationSourceAttackRow],
    occurrence_rows: &[PreSerializationOccurrenceRow],
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
) -> NotationVoicePreSerializationConstraintClosureAuditProjection {
    build_notation_voice_pre_serialization_constraint_closure_audit_projection_with_beam_origin(
        projected_final_chord_xml,
        None,
        source_rows,
        occurrence_rows,
        occupancy_rows,
    )
}

pub(crate) fn build_notation_voice_pre_serialization_constraint_closure_audit_projection_with_beam_origin(
    projected_final_chord_xml: &str,
    pre_final_chord_xml: Option<&str>,
    source_rows: &[PreSerializationSourceAttackRow],
    occurrence_rows: &[PreSerializationOccurrenceRow],
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
) -> NotationVoicePreSerializationConstraintClosureAuditProjection {
    let mut source_index = BTreeMap::<SourceAttackKey, Vec<&PreSerializationSourceAttackRow>>::new();
    for row in source_rows {
        source_index
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }
    let mut occurrence_index = BTreeMap::<OccurrenceKey, Vec<&PreSerializationOccurrenceRow>>::new();
    for row in occurrence_rows {
        occurrence_index
            .entry((row.measure, row.pitched_ordinal))
            .or_default()
            .push(row);
    }

    let mut occupancy_groups_all =
        BTreeMap::<usize, Vec<&PreSerializationVisibleOccupancyRow>>::new();
    for row in occupancy_rows {
        occupancy_groups_all
            .entry(row.atomic_group_id)
            .or_default()
            .push(row);
    }
    let hidden_chord_without_pitched_anchor_members = occupancy_rows
        .iter()
        .filter(|row| {
            row.is_chord_member
                && !row.visible
                && row.atomic_group_kind
                    == PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor
        })
        .count();
    let visible_chord_without_pitched_anchor_groups = occupancy_groups_all
        .iter()
        .filter_map(|(&group_id, group_rows)| {
            group_rows
                .iter()
                .any(|row| {
                    row.is_chord_member
                        && row.visible
                        && row.pitch_midi.is_some()
                        && row.atomic_group_kind
                            == PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor
                })
                .then_some(group_id)
        })
        .collect::<BTreeSet<_>>();
    let visible_rest_groups = occupancy_groups_all
        .iter()
        .filter_map(|(&group_id, group_rows)| {
            group_rows
                .iter()
                .any(|row| row.visible && row.is_rest)
                .then_some(group_id)
        })
        .collect::<BTreeSet<_>>();
    let mut adapter_evidence_rows = Vec::<String>::new();
    for group_id in &visible_chord_without_pitched_anchor_groups {
        let group_rows = occupancy_groups_all
            .get(group_id)
            .expect("visible orphan chord group must exist");
        let visible_pitched_rows = group_rows
            .iter()
            .copied()
            .filter(|row| row.visible && row.pitch_midi.is_some())
            .collect::<Vec<_>>();
        let representative = visible_pitched_rows[0];
        let interval_start = visible_pitched_rows
            .iter()
            .map(|row| row.represented_onset)
            .min()
            .unwrap_or(representative.represented_onset);
        let interval_end = visible_pitched_rows
            .iter()
            .map(|row| {
                row.represented_onset
                    .saturating_add(row.represented_duration)
            })
            .max()
            .unwrap_or_else(|| {
                representative
                    .represented_onset
                    .saturating_add(representative.represented_duration)
            });
        let pitches = visible_pitched_rows
            .iter()
            .filter_map(|row| row.pitch_midi)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|pitch| pitch.to_string())
            .collect::<Vec<_>>()
            .join(",");
        adapter_evidence_rows.push(format!(
            "VISIBLE_CHORD_WITHOUT_PITCHED_ANCHOR group={} measure={} staff={} interval={}-{} pitches=[{}]",
            group_id,
            representative.measure,
            representative.staff,
            interval_start,
            interval_end,
            pitches,
        ));
    }
    let mut visible_rest_orphan_chord_overlap_pairs = BTreeSet::<(usize, usize)>::new();
    for rest_group_id in &visible_rest_groups {
        let Some(rest_rows) = occupancy_groups_all.get(rest_group_id) else {
            continue;
        };
        let Some(rest) = rest_rows.iter().copied().find(|row| row.visible && row.is_rest) else {
            continue;
        };
        for chord_group_id in &visible_chord_without_pitched_anchor_groups {
            let Some(chord_rows) = occupancy_groups_all.get(chord_group_id) else {
                continue;
            };
            let visible_chord_rows = chord_rows
                .iter()
                .copied()
                .filter(|row| row.visible && row.pitch_midi.is_some())
                .collect::<Vec<_>>();
            let Some(chord) = visible_chord_rows.first().copied() else {
                continue;
            };
            let chord_interval_start = visible_chord_rows
                .iter()
                .map(|row| row.represented_onset)
                .min()
                .unwrap_or(chord.represented_onset);
            let chord_interval_end = visible_chord_rows
                .iter()
                .map(|row| {
                    row.represented_onset
                        .saturating_add(row.represented_duration)
                })
                .max()
                .unwrap_or_else(|| {
                    chord
                        .represented_onset
                        .saturating_add(chord.represented_duration)
                });
            let chord_interval_duration =
                chord_interval_end.saturating_sub(chord_interval_start);
            if rest.measure == chord.measure
                && rest.staff == chord.staff
                && overlaps(
                    rest.represented_onset,
                    rest.represented_duration,
                    chord_interval_start,
                    chord_interval_duration,
                )
            {
                visible_rest_orphan_chord_overlap_pairs
                    .insert((*rest_group_id, *chord_group_id));
                adapter_evidence_rows.push(format!(
                    "VISIBLE_REST_ORPHAN_CHORD_OVERLAP rest_group={} chord_group={} measure={} staff={} rest_interval={}-{} chord_interval={}-{}",
                    rest_group_id,
                    chord_group_id,
                    rest.measure,
                    rest.staff,
                    rest.represented_onset,
                    rest.represented_onset.saturating_add(rest.represented_duration),
                    chord_interval_start,
                    chord_interval_end,
                ));
            }
        }
    }
    adapter_evidence_rows.sort();
    adapter_evidence_rows.dedup();

    let chains = voice_free_primary_beam_chain_evidence(projected_final_chord_xml)
        .unwrap_or_default();
    let pre_final_chord_chain_shapes = pre_final_chord_xml
        .and_then(voice_free_primary_beam_chain_evidence)
        .map(|chains| {
            chains
                .iter()
                .map(voice_free_beam_chain_shape)
                .collect::<BTreeSet<_>>()
        });
    let mut rows = Vec::<String>::new();
    let mut semantic_shapes = Vec::<String>::new();
    let mut closed_candidates = Vec::<PreSerializationClosedSameLineCandidate>::new();
    let mut pre_final_primary_beam_exact_proof_chain_ids = BTreeSet::<String>::new();
    let mut canonical_primary_beam_owner_receipts = Vec::<CanonicalPrimaryBeamOwnerReceipt>::new();
    let mut beam_witness_pre_final_exact = 0usize;
    let mut beam_witness_pre_final_missing = 0usize;
    let mut multi_lane = 0usize;
    let mut candidate_closed_except_beam = 0usize;
    let mut single_lane = 0usize;
    let mut blocked_noncontiguous = 0usize;
    let mut blocked_occurrence_identity = 0usize;
    let mut blocked_source_attack_identity = 0usize;
    let mut blocked_noninjective_identity = 0usize;
    let mut blocked_time = 0usize;
    let mut blocked_pressure = 0usize;
    let mut blocked_external_pressure = 0usize;
    let mut blocked_pitched_occupancy = 0usize;
    let mut blocked_rest_occupancy = 0usize;

    for chain in &chains {
        let classification = classify_chain(chain, &source_index, &occurrence_index, occupancy_rows);
        if classification.technical_lanes.len() > 1 {
            multi_lane = multi_lane.saturating_add(1);
        }
        match classification.status {
            ChainStatus::CandidateClosedExceptBeamProvenanceAndMaterialization => {
                candidate_closed_except_beam = candidate_closed_except_beam.saturating_add(1);
            }
            ChainStatus::NotApplicableSingleTechnicalLane => {
                single_lane = single_lane.saturating_add(1);
            }
            ChainStatus::BlockedNoncontiguousRepresentation => {
                blocked_noncontiguous = blocked_noncontiguous.saturating_add(1);
            }
            ChainStatus::BlockedOccurrenceIdentity => {
                blocked_occurrence_identity = blocked_occurrence_identity.saturating_add(1);
            }
            ChainStatus::BlockedSourceAttackIdentity => {
                blocked_source_attack_identity = blocked_source_attack_identity.saturating_add(1);
            }
            ChainStatus::BlockedNonInjectiveIdentity => {
                blocked_noninjective_identity = blocked_noninjective_identity.saturating_add(1);
            }
            ChainStatus::BlockedSemanticTimeDivergence => {
                blocked_time = blocked_time.saturating_add(1);
            }
            ChainStatus::BlockedLanePressureWitness => {
                blocked_pressure = blocked_pressure.saturating_add(1);
            }
            ChainStatus::BlockedExternalLanePressure => {
                blocked_external_pressure = blocked_external_pressure.saturating_add(1);
            }
            ChainStatus::BlockedIndependentPitchedOccupancy => {
                blocked_pitched_occupancy = blocked_pitched_occupancy.saturating_add(1);
            }
            ChainStatus::BlockedRestOccupancyOwnership => {
                blocked_rest_occupancy = blocked_rest_occupancy.saturating_add(1);
            }
        }
        if !classification.semantic_shape.is_empty() {
            semantic_shapes.push(classification.semantic_shape.clone());
        }
        if classification.source_event_ids.is_empty() || chain.members.is_empty() {
            continue;
        }
        let first = &chain.members[0];
        let last = &chain.members[chain.members.len() - 1];
        let proof_chain_id = classification
            .source_event_ids
            .iter()
            .zip(&classification.attack_ids)
            .map(|(source, attack)| format!("{source}:{attack}"))
            .collect::<Vec<_>>()
            .join("-");
        let beam_witness_pre_final_chord_exact = pre_final_chord_chain_shapes
            .as_ref()
            .map(|shapes| shapes.contains(&voice_free_beam_chain_shape(chain)));
        match beam_witness_pre_final_chord_exact {
            Some(true) => {
                beam_witness_pre_final_exact = beam_witness_pre_final_exact.saturating_add(1);
            }
            Some(false) => {
                beam_witness_pre_final_missing = beam_witness_pre_final_missing.saturating_add(1);
            }
            None => {}
        }
        if classification.status
            == ChainStatus::CandidateClosedExceptBeamProvenanceAndMaterialization
        {
            if beam_witness_pre_final_chord_exact == Some(true) {
                pre_final_primary_beam_exact_proof_chain_ids.insert(proof_chain_id.clone());
                if let Some(receipt) = build_canonical_primary_beam_owner_receipt(
                    &proof_chain_id,
                    chain.members.len(),
                ) {
                    canonical_primary_beam_owner_receipts.push(receipt);
                }
            }
            let members = chain
                .members
                .iter()
                .zip(classification.source_event_ids.iter())
                .zip(classification.attack_ids.iter())
                .map(|((member, source_event_id), attack_id)| {
                    PreSerializationClosedSameLineMemberBinding {
                        source_event_id: *source_event_id,
                        attack_id: *attack_id,
                        measure: member.measure,
                        pitched_ordinal: member.pitched_ordinal,
                        represented_onset: member.onset,
                        represented_duration: member.duration,
                        staff: member.staff,
                    }
                })
                .collect::<Vec<_>>();
            closed_candidates.push(PreSerializationClosedSameLineCandidate {
                proof_chain_id: proof_chain_id.clone(),
                source_event_ids: classification.source_event_ids.clone(),
                members,
                staff: first.staff,
            });
        }
        let lane_edges = classification
            .lane_pressure_edges
            .iter()
            .map(|edge| {
                format!(
                    "{}:{}:lane{}:end{}>{}:{}:lane{}:start{}",
                    edge.blocker_source_event_id,
                    edge.blocker_attack_id,
                    edge.blocker_lane,
                    edge.blocker_absolute_end,
                    edge.blocked_source_event_id,
                    edge.blocked_attack_id,
                    edge.blocked_lane,
                    edge.blocked_absolute_start,
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let justified_role_candidate = if classification.status
            == ChainStatus::CandidateClosedExceptBeamProvenanceAndMaterialization
        {
            if first.staff == 1 {
                "upper_primary_candidate"
            } else {
                "lower_primary_candidate"
            }
        } else {
            "none"
        };
        rows.push(format!(
            "PRE_SERIALIZATION_NOTATION_VOICE proof_chain_id={} measure={} staff={} interval={}-{} current_voices_diagnostic_only={} technical_lanes={} source_ids=[{}] attack_ids=[{}] semantic_durations=[{}] identity_mapping=total_unique_injective time_relation={} lane_pressure_edges=[{}] external_lane_pressure_sources=[{}] external_pitched_atomic_groups=[{}] rest_atomic_groups=[{}] chord_extension_atomic_groups=[{}] beam_witness=present_existing_encoding beam_witness_pre_final_chord_exact={} beam_witness_historical_owner=unproven justified_role_candidate={} role_reason=unique_visible_staff_line_with_self_induced_higher_lanes_only_if_conflict_checks_green status={}",
            proof_chain_id,
            first.measure,
            first.staff,
            first.onset,
            last.onset.saturating_add(last.duration),
            classification.current_voices.join(","),
            classification
                .technical_lanes
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            classification
                .source_event_ids
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            classification
                .attack_ids
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            classification
                .semantic_durations
                .iter()
                .map(i64::to_string)
                .collect::<Vec<_>>()
                .join(","),
            if classification.status == ChainStatus::BlockedSemanticTimeDivergence {
                "divergent_requires_authorized_origin_proof"
            } else {
                "semantic_onset_equals_representation_onset_for_mapped_members"
            },
            lane_edges,
            classification
                .external_lane_pressure_source_ids
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            classification
                .external_pitched_atomic_groups
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            classification
                .rest_atomic_groups
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            classification
                .chord_extension_atomic_groups
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            match beam_witness_pre_final_chord_exact {
                Some(true) => "yes",
                Some(false) => "no",
                None => "unavailable",
            },
            justified_role_candidate,
            classification.status.label(),
        ));
    }

    let visible_rhythmic_occupancy_audit = build_visible_rhythmic_occupancy_closure_audit(
        projected_final_chord_xml,
        occupancy_rows,
        true,
    );
    let metric_line_audit = build_metric_notation_line_audit(
        projected_final_chord_xml,
        source_rows,
        occurrence_rows,
        occupancy_rows,
    );
    let beam_candidate_source_sets = closed_candidates
        .iter()
        .map(|candidate| candidate.source_event_ids.iter().copied().collect::<BTreeSet<_>>())
        .collect::<BTreeSet<_>>();
    let metric_candidate_source_sets = metric_line_audit
        .closed_candidates
        .iter()
        .map(|candidate| candidate.source_event_ids.iter().copied().collect::<BTreeSet<_>>())
        .collect::<BTreeSet<_>>();
    let metric_candidates_with_exact_beam_candidate = metric_candidate_source_sets
        .intersection(&beam_candidate_source_sets)
        .count();

    rows.sort();
    rows.dedup();
    semantic_shapes.sort();
    semantic_shapes.dedup();
    let semantic_fingerprint_fnv64 = fnv1a64(semantic_shapes.join("\n").as_bytes());
    let blocked_total = blocked_noncontiguous
        .saturating_add(blocked_occurrence_identity)
        .saturating_add(blocked_source_attack_identity)
        .saturating_add(blocked_noninjective_identity)
        .saturating_add(blocked_time)
        .saturating_add(blocked_pressure)
        .saturating_add(blocked_external_pressure)
        .saturating_add(blocked_pitched_occupancy)
        .saturating_add(blocked_rest_occupancy);
    let verdict = if chains.is_empty() {
        "NO_PRIMARY_BEAM_CHAIN_EVIDENCE"
    } else if candidate_closed_except_beam > 0 && blocked_total == 0 {
        "PRE_SERIALIZATION_CONSTRAINTS_GREEN_BEAM_PROVENANCE_AND_MATERIALIZATION_OPEN"
    } else if candidate_closed_except_beam > 0 {
        "PRE_SERIALIZATION_CANDIDATES_WITH_BLOCKED_CASES"
    } else if blocked_total > 0 {
        "NO_CLOSED_CANDIDATE_BLOCKED_EVIDENCE"
    } else {
        "NO_MULTI_LANE_CANDIDATE"
    };

    let mut out = String::new();
    let _ = writeln!(out, "ProjectionOnly: true");
    let _ = writeln!(out, "Authority: none");
    let _ = writeln!(out, "MusicMutation: nein");
    let _ = writeln!(out, "RuleChange: nein");
    let _ = writeln!(out, "AuditSourcePhase: canonical_final_chord_projection_after_sounding_freeze_before_serialized_voice_channel_plan");
    let _ = writeln!(out, "CurrentVoiceLabels: diagnostic_only_excluded_from_decisions_and_semantic_fingerprint");
    let _ = writeln!(out, "StableProofChainIdentity: ordered_SourceEventId_SoundingAttackId_members_only");
    let _ = writeln!(out, "OccurrenceIdentity: transaction_bound_final_chord_projection_provenance_to_measure_pitched_ordinal");
    let _ = writeln!(out, "IdentityRequirement: total_unique_injective_on_occurrence_attack_groups");
    let _ = writeln!(out, "CanonicalMusicInvariant: pitch_semantic_onset_semantic_duration_strictly_unchanged");
    let _ = writeln!(out, "VisibleOccupancy: complete_visible_same_staff_interval_note_and_rest_rows_with_atomic_chord_groups");
    let _ = writeln!(out, "ConflictCoverage: same_staff_visible_interval_complete_cross_staff_not_claimed");
    let _ = writeln!(out, "ChordOccupancy: visible_pitched_anchor_plus_chord_members_atomic_group; chord_without_pitched_anchor_preserved_as_separate_evidence_group");
    let _ = writeln!(out, "HiddenChordCarrierPolicy: hidden_chord_without_pitched_anchor_is_not_visible_occupancy_authority_and_never_adapter_fatal");
    let _ = writeln!(out, "VisibleChordWithoutPitchedAnchorGroups: {}", visible_chord_without_pitched_anchor_groups.len());
    let _ = writeln!(out, "HiddenChordWithoutPitchedAnchorMembers: {hidden_chord_without_pitched_anchor_members}");
    let _ = writeln!(out, "VisibleRestOrphanChordOverlapEvidencePairs: {}", visible_rest_orphan_chord_overlap_pairs.len());
    let _ = writeln!(out, "RestOccupancy: visible_overlap_blocks_until_ownership_is_independently_proven");
    let _ = writeln!(out, "VisibleRhythmicOccupancyProjectionOnly: true");
    let _ = writeln!(out, "VisibleRhythmicOccupancyAuthority: none");
    let _ = writeln!(out, "VisibleRhythmicOccupancySelectionVoiceLabels: excluded");
    let _ = writeln!(out, "VisibleRhythmicOccupancySelectionBeams: excluded");
    let _ = writeln!(out, "VisibleRhythmicOccupancySelectionSoundingLanes: excluded");
    let _ = writeln!(out, "VisibleRhythmicOccupancyAtomicChordPolicy: one_visible_pitched_anchor_plus_visible_chord_members_is_one_atom");
    let _ = writeln!(out, "VisibleRhythmicOccupancyRestConflictSemantics: duplicate_or_note_overlap_is_REST_OWNERSHIP_CONFLICT_not_HARD_DIFFERENT");
    let _ = writeln!(out, "VisibleRhythmicOccupancyPitchedOverlapSemantics: disjoint_visible_pitched_atoms_overlap_is_HARD_DIFFERENT_CANDIDATE_analysis_only");
    let _ = writeln!(out, "VisibleRhythmicOccupancyMetricParse: {}", if visible_rhythmic_occupancy_audit.metric_parse_ok { "ok" } else { "unavailable" });
    let _ = writeln!(out, "VisibleRhythmicOccupancyVisibleAtomicGroups: {}", visible_rhythmic_occupancy_audit.visible_atomic_groups);
    let _ = writeln!(out, "VisibleRhythmicOccupancyHiddenOnlyAtomicGroups: {}", visible_rhythmic_occupancy_audit.hidden_only_atomic_groups);
    let _ = writeln!(out, "VisibleRhythmicOccupancyIncompleteAtomicGroups: {}", visible_rhythmic_occupancy_audit.incomplete_atomic_groups);
    let _ = writeln!(out, "VisibleRhythmicOccupancyPitchedNoteAtoms: {}", visible_rhythmic_occupancy_audit.pitched_note_atoms);
    let _ = writeln!(out, "VisibleRhythmicOccupancyPitchedChordAtoms: {}", visible_rhythmic_occupancy_audit.pitched_chord_atoms);
    let _ = writeln!(out, "VisibleRhythmicOccupancyRestAtoms: {}", visible_rhythmic_occupancy_audit.rest_atoms);
    let _ = writeln!(out, "VisibleRhythmicOccupancyDuplicateRestPairs: {}", visible_rhythmic_occupancy_audit.duplicate_rest_pairs);
    let _ = writeln!(out, "VisibleRhythmicOccupancyRestPitchedOverlapPairs: {}", visible_rhythmic_occupancy_audit.rest_pitched_overlap_pairs);
    let _ = writeln!(out, "VisibleRhythmicOccupancyIndependentPitchedOverlapPairs: {}", visible_rhythmic_occupancy_audit.independent_pitched_overlap_pairs);
    let _ = writeln!(out, "VisibleRhythmicOccupancyMetricScopesWithMultipleAtoms: {}", visible_rhythmic_occupancy_audit.metric_scopes_with_multiple_atoms);
    let _ = writeln!(out, "VisibleRhythmicOccupancyMetricScopesContiguousWithoutOverlap: {}", visible_rhythmic_occupancy_audit.metric_scopes_contiguous_without_overlap);
    let _ = writeln!(out, "VisibleRhythmicOccupancyMetricScopesWithRest: {}", visible_rhythmic_occupancy_audit.metric_scopes_with_rest);
    let _ = writeln!(out, "VisibleRhythmicOccupancyMetricScopesWithVisibleOverlap: {}", visible_rhythmic_occupancy_audit.metric_scopes_with_visible_overlap);
    let visible_rhythmic_occupancy_verdict = if visible_rhythmic_occupancy_audit.incomplete_atomic_groups == 0 {
        "VOICE_FREE_VISIBLE_RHYTHMIC_OCCUPANCY_CLOSURE_GREEN_ANALYSIS_ONLY"
    } else {
        "VISIBLE_RHYTHMIC_OCCUPANCY_AVAILABLE_WITH_INCOMPLETE_ATOMIC_GROUPS"
    };
    let _ = writeln!(out, "VisibleRhythmicOccupancyVerdict: {visible_rhythmic_occupancy_verdict}");
    let _ = writeln!(out, "LanePressureWitness: explicit_lower_lane_blocker_edges");
    let _ = writeln!(out, "BeamWitnessAuthority: evidence_only_existing_encoding_current_voice_labels_not_authority");
    let _ = writeln!(out, "BeamWitnessPreFinalChordExactMatches: {beam_witness_pre_final_exact}");
    let _ = writeln!(out, "BeamWitnessPreFinalChordMissing: {beam_witness_pre_final_missing}");
    let _ = writeln!(out, "BeamWitnessHistoricalOwner: unproven_even_when_pre_final_chord_exact");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransport: read_only_current_canonical_owner_source_bound");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptOwnerKind: {CANONICAL_PRIMARY_BEAM_OWNER_KIND}");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptOwnerSourceFnv64: {CANONICAL_PRIMARY_BEAM_OWNER_SOURCE_FNV64_CURRENT:016x}");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptOwnerSourceBound: {}", if canonical_primary_beam_owner_source_is_bound() { "yes" } else { "no" });
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptCount: {}", canonical_primary_beam_owner_receipts.len());
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptHistoricalCausalityClaim: none");
    let _ = writeln!(out, "PresentationRoleAuthority: none_candidate_reason_only_not_members0_or_lane_fallback");
    let _ = writeln!(out, "MaterializationAuthority: none");
    let _ = writeln!(out, "CatalogRole: acceptance_oracle_only_not_selector");
    let _ = writeln!(out, "PrimaryBeamChainsObserved: {}", chains.len());
    let _ = writeln!(out, "SingleTechnicalLaneChains: {single_lane}");
    let _ = writeln!(out, "MultiLaneChainsObserved: {multi_lane}");
    let _ = writeln!(out, "CandidateClosedExceptBeamProvenanceAndMaterialization: {candidate_closed_except_beam}");
    let _ = writeln!(out, "BlockedNoncontiguousRepresentation: {blocked_noncontiguous}");
    let _ = writeln!(out, "BlockedOccurrenceIdentity: {blocked_occurrence_identity}");
    let _ = writeln!(out, "BlockedSourceAttackIdentity: {blocked_source_attack_identity}");
    let _ = writeln!(out, "BlockedNonInjectiveIdentity: {blocked_noninjective_identity}");
    let _ = writeln!(out, "BlockedSemanticRepresentationTimeOrigin: {blocked_time}");
    let _ = writeln!(out, "BlockedLanePressureWitness: {blocked_pressure}");
    let _ = writeln!(out, "BlockedExternalLanePressure: {blocked_external_pressure}");
    let _ = writeln!(out, "BlockedIndependentPitchedOccupancy: {blocked_pitched_occupancy}");
    let _ = writeln!(out, "BlockedRestOccupancyOwnership: {blocked_rest_occupancy}");
    let _ = writeln!(out, "MetricNotationLineProjectionOnly: true");
    let _ = writeln!(out, "MetricNotationLineAuthority: none");
    let _ = writeln!(out, "MetricNotationLineSelectionVoiceLabels: excluded");
    let _ = writeln!(out, "MetricNotationLineSelectionBeams: excluded");
    let _ = writeln!(out, "MetricNotationLineMetricAuthority: MusicXML_divisions_measure_staff_beat_plus_visible_atomic_occupancy");
    let _ = writeln!(out, "MetricNotationLineParse: {}", if metric_line_audit.parse_ok { "ok" } else { "unavailable" });
    let _ = writeln!(out, "MetricNotationLineShortAtomicGroups: {}", metric_line_audit.short_atomic_groups);
    let _ = writeln!(out, "MetricNotationLineMultiMemberAtomicGroupsExcluded: {}", metric_line_audit.multi_member_atomic_groups_excluded);
    let _ = writeln!(out, "MetricNotationLineScopesWithMultipleAtoms: {}", metric_line_audit.metric_scopes_with_multiple_atoms);
    let _ = writeln!(out, "MetricNotationLineNoncontiguousScopes: {}", metric_line_audit.noncontiguous_scopes);
    let _ = writeln!(out, "MetricNotationLineSingleLaneScopes: {}", metric_line_audit.single_lane_scopes);
    let _ = writeln!(out, "MetricNotationLineBlockedIdentity: {}", metric_line_audit.blocked_identity);
    let _ = writeln!(out, "MetricNotationLineBlockedSemanticTime: {}", metric_line_audit.blocked_semantic_time);
    let _ = writeln!(out, "MetricNotationLineBlockedLanePressure: {}", metric_line_audit.blocked_lane_pressure);
    let _ = writeln!(out, "MetricNotationLineBlockedExternalLanePressure: {}", metric_line_audit.blocked_external_lane_pressure);
    let _ = writeln!(out, "MetricNotationLineBlockedExternalPitchedOccupancy: {}", metric_line_audit.blocked_external_pitched_occupancy);
    let _ = writeln!(out, "MetricNotationLineBlockedRestOccupancy: {}", metric_line_audit.blocked_rest_occupancy);
    let _ = writeln!(out, "MetricNotationLineEligibleAnalysisOnly: {}", metric_line_audit.closed_candidates.len());
    let _ = writeln!(out, "MetricNotationLineExactBeamCandidateMatchesDiagnosticOnly: {metric_candidates_with_exact_beam_candidate}");
    let _ = writeln!(out, "MetricNotationLineCatalogRole: none_selector_independent");
    let _ = writeln!(out, "SemanticPlanFingerprintFnv64: {semantic_fingerprint_fnv64:016x}");
    let _ = writeln!(out, "Verdict: {verdict}");
    if adapter_evidence_rows.is_empty() {
        let _ = writeln!(out, "OccupancyAdapterEvidence: none");
    } else {
        for row in adapter_evidence_rows {
            let _ = writeln!(out, "OccupancyAdapterEvidence: {row}");
        }
    }
    if visible_rhythmic_occupancy_audit.rows.is_empty() {
        let _ = writeln!(out, "VisibleRhythmicOccupancyRows: none");
    } else {
        for row in &visible_rhythmic_occupancy_audit.rows {
            let _ = writeln!(out, "VisibleRhythmicOccupancyRow: {row}");
        }
    }
    if metric_line_audit.rows.is_empty() {
        let _ = writeln!(out, "MetricNotationLineRows: none");
    } else {
        for row in &metric_line_audit.rows {
            let _ = writeln!(out, "MetricNotationLineRow: {row}");
        }
    }
    if rows.is_empty() {
        let _ = writeln!(out, "Rows: none");
    } else {
        for row in rows {
            let _ = writeln!(out, "Row: {row}");
        }
    }

    NotationVoicePreSerializationConstraintClosureAuditProjection {
        body: out,
        closed_candidates,
        metric_line_candidates: metric_line_audit.closed_candidates,
        occurrence_rows: occurrence_rows.to_vec(),
        pre_final_primary_beam_exact_proof_chain_ids,
        canonical_primary_beam_owner_receipts,
        materialization_safety_proven_metric_line_ids: BTreeSet::new(),
        materialization_safety_primary_target_evidence_rows: Vec::new(),
        visible_rhythmic_occupancy_phase_snapshot:
            visible_rhythmic_occupancy_audit.phase_snapshot,
    }
}


fn source_partition_signature(
    plan: &crate::structural_write_authority::FrozenSerializationVoiceChannelPlan,
    source_ids: &[usize],
) -> Vec<Vec<usize>> {
    let mut by_channel = BTreeMap::<usize, Vec<usize>>::new();
    for source in source_ids {
        if let Some(channel) = plan.channel_for_source(*source) {
            by_channel.entry(channel).or_default().push(*source);
        }
    }
    let mut groups = by_channel.into_values().collect::<Vec<_>>();
    for group in &mut groups {
        group.sort_unstable();
    }
    groups.sort();
    groups
}

fn source_role_signature(
    plan: &crate::structural_write_authority::FrozenSerializationVoiceChannelPlan,
    source_ids: &[usize],
) -> Vec<(usize, &'static str)> {
    let mut rows = source_ids
        .iter()
        .filter_map(|source| {
            plan.role_for_source(*source)
                .map(|role| (*source, role.label()))
        })
        .collect::<Vec<_>>();
    rows.sort_unstable();
    rows
}

/// Read-only readback for the voice timeline collision class: two physical
/// occurrences bound to the same frozen serialization channel, in the same
/// measure and on the same represented staff, whose represented intervals
/// overlap without sharing onset and duration. Each pair shows the frozen
/// material view (staff, sounding interval, technical lane, role) next to the
/// represented view, so the view in which the overlap is invisible to the
/// channel plan can be read off. Writes nothing and changes no plan.
fn notated_channel_overlap_readback(
    source_rows: &[PreSerializationSourceAttackRow],
    occurrence_rows: &[PreSerializationOccurrenceRow],
    channel_of: impl Fn(usize) -> Option<(usize, &'static str)>,
) -> String {
    const ROW_LIMIT: usize = 40;
    let material = source_rows
        .iter()
        .map(|row| ((row.source_event_id, row.attack_id), row))
        .collect::<BTreeMap<_, _>>();
    let mut groups = BTreeMap::<(usize, u8, usize), Vec<&PreSerializationOccurrenceRow>>::new();
    let mut unbound = 0usize;
    for row in occurrence_rows {
        if let Some((channel, _)) = channel_of(row.source_event_id) {
            groups.entry((row.measure, row.staff, channel)).or_default().push(row);
        } else {
            unbound += 1;
        }
    }
    let describe = |row: &PreSerializationOccurrenceRow| {
        let role = channel_of(row.source_event_id).map_or("none", |(_, role)| role);
        match material.get(&(row.source_event_id, row.attack_id)) {
            Some(source) => format!(
                "src={} pitch={} represented=m{}:{}/{}:staff{} material=staff{}:{}/{} sounding_abs={}..{} lane={} role={}",
                row.source_event_id,
                row.pitch_midi,
                row.measure,
                row.represented_onset,
                row.represented_duration,
                row.staff,
                source.staff,
                source.semantic_onset,
                source.semantic_duration,
                source.absolute_start,
                source.absolute_end,
                source.technical_lane,
                role,
            ),
            None => format!(
                "src={} pitch={} represented=m{}:{}/{}:staff{} material=absent role={}",
                row.source_event_id,
                row.pitch_midi,
                row.measure,
                row.represented_onset,
                row.represented_duration,
                row.staff,
                role,
            ),
        }
    };
    let mut pairs = 0usize;
    let mut staff_view_differs = 0usize;
    let mut sounding_disjoint = 0usize;
    let mut sounding_overlap = 0usize;
    let mut material_absent = 0usize;
    let mut rows = Vec::<String>::new();
    for (&(_, _, channel), members) in &groups {
        for (left_index, left) in members.iter().enumerate() {
            for right in &members[left_index + 1..] {
                let left_end = left.represented_onset + left.represented_duration;
                let right_end = right.represented_onset + right.represented_duration;
                let overlaps = left.represented_onset < right_end && right.represented_onset < left_end;
                let coincident = left.represented_onset == right.represented_onset
                    && left.represented_duration == right.represented_duration;
                if left.source_event_id == right.source_event_id || !overlaps || coincident {
                    continue;
                }
                pairs += 1;
                let left_material = material.get(&(left.source_event_id, left.attack_id));
                let right_material = material.get(&(right.source_event_id, right.attack_id));
                let view = match (left_material, right_material) {
                    (Some(l), Some(r)) => {
                        if l.staff != left.staff || r.staff != right.staff {
                            staff_view_differs += 1;
                            "staff_view_differs"
                        } else if l.absolute_start < r.absolute_end && r.absolute_start < l.absolute_end {
                            sounding_overlap += 1;
                            "sounding_overlap_same_staff"
                        } else {
                            sounding_disjoint += 1;
                            "sounding_intervals_disjoint"
                        }
                    }
                    _ => {
                        material_absent += 1;
                        "material_row_absent"
                    }
                };
                if rows.len() < ROW_LIMIT {
                    rows.push(format!(
                        "NotatedChannelOverlap: channel={channel} view={view} left[{}] right[{}]",
                        describe(left),
                        describe(right),
                    ));
                }
            }
        }
    }
    let mut out = String::new();
    let _ = writeln!(
        out,
        "NotatedChannelOverlapReadback: read_only; Pairs {pairs}; StaffViewDiffers {staff_view_differs}; SoundingIntervalsDisjoint {sounding_disjoint}; SoundingOverlapSameStaff {sounding_overlap}; MaterialRowAbsent {material_absent}; UnboundOccurrences {unbound}; RowsShown {}.",
        rows.len()
    );
    for row in rows {
        let _ = writeln!(out, "{row}");
    }
    out
}

pub(crate) fn append_notation_voice_materialization_counterfactual_audit(
    projection: &mut NotationVoicePreSerializationConstraintClosureAuditProjection,
    canonical: &crate::structural_write_authority::CanonicalFinalChordStructuralPlan,
    source_rows: &[PreSerializationSourceAttackRow],
    occurrence_rows: &[PreSerializationOccurrenceRow],
    frozen_attacks: &[crate::structural_write_authority::FrozenVoiceAttackEvidenceRow],
    baseline_channels: &crate::structural_write_authority::FrozenSerializationVoiceChannelPlan,
) {
    let candidates = projection.closed_candidates().to_vec();
    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(out, "MaterializationCounterfactual: read_only");
    let _ = writeln!(out, "CounterfactualProjectionOnly: true");
    let _ = writeln!(out, "Authority: none");
    let _ = writeln!(out, "MusicMutation: nein");
    let _ = writeln!(out, "RuleChange: nein");
    let _ = writeln!(out, "CounterfactualScope: candidate_local_exact_occurrence_binding_then_source_disjoint_clean_batch");
    let _ = writeln!(out, "CounterfactualInputDelta: candidate_sources_only technical_lane=>1 and sounding_absolute_interval=>bound_representation_absolute_interval");
    let _ = writeln!(out, "CounterfactualInvariant: source_event_id_attack_id_staff_pitch_and_semantic_source_rows_unchanged");
    let _ = writeln!(out, "OccurrenceMultiplicityPolicy: semantic_SourceAttack_may_have_multiple_physical_occurrences; candidate_is_locally_blocked_unless_each_member_SourceAttack_has_exactly_one_physical_occurrence");
    let _ = writeln!(out, "CandidateBindingAuthority: closed_candidate_measure_pitched_ordinal_binding_not_global_SourceAttack_to_occurrence_uniqueness");
    let _ = writeln!(out, "CounterfactualPurpose: locate_current_lane_role_and_sounding_overlap_authority_without_writing_music");
    out.push_str(&notated_channel_overlap_readback(source_rows, occurrence_rows, |source| {
        baseline_channels.channel_for_source(source).map(|channel| {
            let role = baseline_channels
                .role_for_source(source)
                .map_or("none", |role| role.label());
            (channel, role)
        })
    }));

    if candidates.is_empty() {
        let _ = writeln!(out, "Candidates: 0");
        let _ = writeln!(out, "Verdict: NO_CLOSED_PRE_SERIALIZATION_CANDIDATES");
        projection.append_counterfactual_body(&out);
        return;
    }

    type IdentityKey = (usize, usize);
    type LocatorKey = (usize, usize);

    let mut source_by_identity = BTreeMap::<IdentityKey, Vec<&PreSerializationSourceAttackRow>>::new();
    for row in source_rows {
        source_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }
    let mut occurrence_by_identity =
        BTreeMap::<IdentityKey, Vec<&PreSerializationOccurrenceRow>>::new();
    let mut occurrence_by_locator =
        BTreeMap::<LocatorKey, Vec<&PreSerializationOccurrenceRow>>::new();
    for row in occurrence_rows {
        occurrence_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
        occurrence_by_locator
            .entry((row.measure, row.pitched_ordinal))
            .or_default()
            .push(row);
    }
    let mut frozen_count_by_identity = BTreeMap::<IdentityKey, usize>::new();
    for attack in frozen_attacks {
        *frozen_count_by_identity
            .entry((attack.source_event_id, attack.attack_id))
            .or_default() += 1;
    }

    let all_source_ids = frozen_attacks
        .iter()
        .map(|attack| attack.source_event_id)
        .collect::<BTreeSet<_>>();

    let mut current_split_candidates = 0usize;
    let mut unique_binding_candidates = 0usize;
    let mut multi_occurrence_blocked = 0usize;
    let mut other_input_blocked = 0usize;
    let mut planner_rejected = 0usize;
    let mut target_realized = 0usize;
    let mut target_not_realized = 0usize;
    let mut local_external_partition_preserved_all = true;
    let mut local_external_roles_preserved_all = true;
    let mut rows = Vec::<String>::new();
    let mut locally_green = Vec::<PreSerializationClosedSameLineCandidate>::new();

    for candidate in &candidates {
        let candidate_source_ids = candidate
            .source_event_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let source_id_unique = candidate_source_ids.len() == candidate.source_event_ids.len();
        let external_source_ids = all_source_ids
            .difference(&candidate_source_ids)
            .copied()
            .collect::<Vec<_>>();
        let current_channels = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_channels.channel_for_source(*source))
            .collect::<BTreeSet<_>>();
        let current_roles = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_channels.role_for_source(*source))
            .map(|role| role.label())
            .collect::<BTreeSet<_>>();
        if current_channels.len() > 1 || current_roles.len() > 1 {
            current_split_candidates = current_split_candidates.saturating_add(1);
        }

        let mut input_failures = Vec::<String>::new();
        let mut has_multi_occurrence = false;
        if !source_id_unique {
            input_failures.push("duplicate_source_event_id_within_candidate".to_string());
        }
        for member in &candidate.members {
            let identity = (member.source_event_id, member.attack_id);
            match source_by_identity.get(&identity) {
                Some(rows) if rows.len() == 1 => {
                    let source = rows[0];
                    if source.staff != member.staff {
                        input_failures.push(format!(
                            "source_staff_mismatch:{}:{}",
                            identity.0, identity.1
                        ));
                    }
                }
                Some(rows) => input_failures.push(format!(
                    "source_identity_multiplicity:{}:{}:{}",
                    identity.0,
                    identity.1,
                    rows.len()
                )),
                None => input_failures.push(format!(
                    "missing_source:{}:{}",
                    identity.0, identity.1
                )),
            }

            let locator = (member.measure, member.pitched_ordinal);
            match occurrence_by_locator.get(&locator) {
                Some(rows) if rows.len() == 1 => {
                    let occurrence = rows[0];
                    if occurrence.source_event_id != member.source_event_id
                        || occurrence.attack_id != member.attack_id
                        || occurrence.staff != member.staff
                        || occurrence.represented_onset != member.represented_onset
                        || occurrence.represented_duration != member.represented_duration
                    {
                        input_failures.push(format!(
                            "bound_occurrence_mismatch:{}:{}:{}:{}",
                            member.measure,
                            member.pitched_ordinal,
                            member.source_event_id,
                            member.attack_id
                        ));
                    }
                }
                Some(rows) => input_failures.push(format!(
                    "locator_multiplicity:{}:{}:{}",
                    member.measure,
                    member.pitched_ordinal,
                    rows.len()
                )),
                None => input_failures.push(format!(
                    "missing_bound_occurrence:{}:{}",
                    member.measure, member.pitched_ordinal
                )),
            }

            match occurrence_by_identity.get(&identity) {
                Some(rows) if rows.len() == 1 => {}
                Some(rows) => {
                    has_multi_occurrence = true;
                    input_failures.push(format!(
                        "multi_occurrence_representation:{}:{}:{}",
                        identity.0,
                        identity.1,
                        rows.len()
                    ));
                }
                None => input_failures.push(format!(
                    "missing_occurrence_identity:{}:{}",
                    identity.0, identity.1
                )),
            }

            match frozen_count_by_identity.get(&identity).copied() {
                Some(1) => {}
                Some(count) => input_failures.push(format!(
                    "frozen_attack_multiplicity:{}:{}:{}",
                    identity.0, identity.1, count
                )),
                None => input_failures.push(format!(
                    "missing_frozen_attack:{}:{}",
                    identity.0, identity.1
                )),
            }
        }
        input_failures.sort();
        input_failures.dedup();

        if !input_failures.is_empty() {
            if has_multi_occurrence {
                multi_occurrence_blocked = multi_occurrence_blocked.saturating_add(1);
            } else {
                other_input_blocked = other_input_blocked.saturating_add(1);
            }
            rows.push(format!(
                "COUNTERFACTUAL_NOTATION_VOICE proof_chain_id={} staff={} occurrence_binding={} failures=[{}] status={}",
                candidate.proof_chain_id,
                candidate.staff,
                if has_multi_occurrence {
                    "multi_occurrence_representation"
                } else {
                    "incomplete"
                },
                input_failures.join("|"),
                if has_multi_occurrence {
                    "BLOCKED_INCOMPLETE_MULTI_OCCURRENCE_REPRESENTATION_BINDING"
                } else {
                    "BLOCKED_INCOMPLETE_COUNTERFACTUAL_INPUT_BINDING"
                },
            ));
            continue;
        }
        unique_binding_candidates = unique_binding_candidates.saturating_add(1);

        let mut shadow_attacks = frozen_attacks.to_vec();
        let candidate_keys = candidate
            .members
            .iter()
            .map(|member| (member.source_event_id, member.attack_id))
            .collect::<BTreeSet<_>>();
        for attack in &mut shadow_attacks {
            let identity = (attack.source_event_id, attack.attack_id);
            if !candidate_keys.contains(&identity) {
                continue;
            }
            let source = source_by_identity
                .get(&identity)
                .and_then(|rows| rows.first())
                .copied()
                .expect("counterfactual source binding validated");
            let member = candidate
                .members
                .iter()
                .find(|member| {
                    member.source_event_id == identity.0 && member.attack_id == identity.1
                })
                .expect("counterfactual candidate member binding validated");
            let occurrence = occurrence_by_locator
                .get(&(member.measure, member.pitched_ordinal))
                .and_then(|rows| rows.first())
                .copied()
                .expect("counterfactual occurrence locator validated");
            let measure_base = source.absolute_start.saturating_sub(source.semantic_onset);
            let represented_absolute_start =
                measure_base.saturating_add(occurrence.represented_onset);
            let represented_absolute_end = represented_absolute_start
                .saturating_add(occurrence.represented_duration);
            attack.absolute_start = represented_absolute_start;
            attack.absolute_end = represented_absolute_end;
            attack.technical_lane = 1;
        }

        let shadow_channels = match crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
            canonical,
            &shadow_attacks,
        ) {
            Ok(plan) => plan,
            Err(error) => {
                planner_rejected = planner_rejected.saturating_add(1);
                rows.push(format!(
                    "COUNTERFACTUAL_NOTATION_VOICE proof_chain_id={} staff={} occurrence_binding=unique_exact planner_error={} status=BLOCKED_COUNTERFACTUAL_PLANNER_REJECTED",
                    candidate.proof_chain_id,
                    candidate.staff,
                    error,
                ));
                continue;
            }
        };

        let shadow_candidate_channels = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| shadow_channels.channel_for_source(*source))
            .collect::<BTreeSet<_>>();
        let shadow_roles = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| shadow_channels.role_for_source(*source))
            .map(|role| role.label())
            .collect::<BTreeSet<_>>();
        let expected_role = match candidate.staff {
            1 => Some(
                crate::structural_write_authority::SerializedVoicePresentationRole::UpperPrimary,
            ),
            2 => Some(
                crate::structural_write_authority::SerializedVoicePresentationRole::LowerPrimary,
            ),
            _ => None,
        };
        let expected_role_label = expected_role
            .map(crate::structural_write_authority::SerializedVoicePresentationRole::label)
            .unwrap_or("unsupported_staff");
        let realized = shadow_candidate_channels.len() == 1
            && shadow_roles.len() == 1
            && shadow_roles.contains(expected_role_label);
        if realized {
            target_realized = target_realized.saturating_add(1);
        } else {
            target_not_realized = target_not_realized.saturating_add(1);
        }

        let baseline_external_partition =
            source_partition_signature(baseline_channels, &external_source_ids);
        let shadow_external_partition =
            source_partition_signature(&shadow_channels, &external_source_ids);
        let external_partition_preserved =
            baseline_external_partition == shadow_external_partition;
        let external_roles_preserved = source_role_signature(baseline_channels, &external_source_ids)
            == source_role_signature(&shadow_channels, &external_source_ids);
        local_external_partition_preserved_all &= external_partition_preserved;
        local_external_roles_preserved_all &= external_roles_preserved;

        let current_channel_text = current_channels
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let shadow_channel_text = shadow_candidate_channels
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let status = if realized && external_partition_preserved && external_roles_preserved {
            locally_green.push(candidate.clone());
            "COUNTERFACTUAL_TARGET_RELATION_REALIZED_EXTERNAL_RELATIONS_PRESERVED"
        } else if realized {
            "BLOCKED_COUNTERFACTUAL_EXTERNAL_RELATION_CHANGE"
        } else {
            "BLOCKED_COUNTERFACTUAL_TARGET_RELATION_NOT_REALIZED"
        };
        rows.push(format!(
            "COUNTERFACTUAL_NOTATION_VOICE proof_chain_id={} staff={} sources=[{}] occurrence_binding=unique_exact current_channels=[{}] current_roles=[{}] shadow_channels=[{}] shadow_roles=[{}] target_role={} target_relation_realized={} external_partition_preserved={} external_roles_preserved={} status={}",
            candidate.proof_chain_id,
            candidate.staff,
            candidate
                .source_event_ids
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            current_channel_text,
            current_roles.iter().copied().collect::<Vec<_>>().join(","),
            shadow_channel_text,
            shadow_roles.iter().copied().collect::<Vec<_>>().join(","),
            expected_role_label,
            if realized { "yes" } else { "no" },
            if external_partition_preserved { "yes" } else { "no" },
            if external_roles_preserved { "yes" } else { "no" },
            status,
        ));
    }

    let mut green_source_counts = BTreeMap::<usize, usize>::new();
    for candidate in &locally_green {
        for source in candidate.source_event_ids.iter().copied().collect::<BTreeSet<_>>() {
            *green_source_counts.entry(source).or_default() += 1;
        }
    }
    let batch_candidates = locally_green
        .iter()
        .filter(|candidate| {
            candidate
                .source_event_ids
                .iter()
                .all(|source| green_source_counts.get(source).copied() == Some(1))
        })
        .cloned()
        .collect::<Vec<_>>();
    let batch_overlap_excluded = locally_green.len().saturating_sub(batch_candidates.len());

    let mut batch_target_realized = 0usize;
    let mut batch_target_not_realized = 0usize;
    let mut batch_external_partition_preserved = true;
    let mut batch_external_roles_preserved = true;
    let batch_verdict = if batch_candidates.is_empty() {
        "NO_BATCH_ELIGIBLE_SOURCE_DISJOINT_CANDIDATES".to_string()
    } else {
        let batch_source_ids = batch_candidates
            .iter()
            .flat_map(|candidate| candidate.source_event_ids.iter().copied())
            .collect::<BTreeSet<_>>();
        let batch_external_source_ids = all_source_ids
            .difference(&batch_source_ids)
            .copied()
            .collect::<Vec<_>>();
        let mut batch_shadow_attacks = frozen_attacks.to_vec();
        for candidate in &batch_candidates {
            let candidate_keys = candidate
                .members
                .iter()
                .map(|member| (member.source_event_id, member.attack_id))
                .collect::<BTreeSet<_>>();
            for attack in &mut batch_shadow_attacks {
                let identity = (attack.source_event_id, attack.attack_id);
                if !candidate_keys.contains(&identity) {
                    continue;
                }
                let source = source_by_identity
                    .get(&identity)
                    .and_then(|rows| rows.first())
                    .copied()
                    .expect("batch source binding inherited from local green candidate");
                let member = candidate
                    .members
                    .iter()
                    .find(|member| {
                        member.source_event_id == identity.0 && member.attack_id == identity.1
                    })
                    .expect("batch member binding inherited from local green candidate");
                let occurrence = occurrence_by_locator
                    .get(&(member.measure, member.pitched_ordinal))
                    .and_then(|rows| rows.first())
                    .copied()
                    .expect("batch occurrence binding inherited from local green candidate");
                let measure_base = source.absolute_start.saturating_sub(source.semantic_onset);
                let represented_absolute_start =
                    measure_base.saturating_add(occurrence.represented_onset);
                attack.absolute_start = represented_absolute_start;
                attack.absolute_end = represented_absolute_start
                    .saturating_add(occurrence.represented_duration);
                attack.technical_lane = 1;
            }
        }

        match crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
            canonical,
            &batch_shadow_attacks,
        ) {
            Ok(batch_channels) => {
                for candidate in &batch_candidates {
                    let channels = candidate
                        .source_event_ids
                        .iter()
                        .filter_map(|source| batch_channels.channel_for_source(*source))
                        .collect::<BTreeSet<_>>();
                    let roles = candidate
                        .source_event_ids
                        .iter()
                        .filter_map(|source| batch_channels.role_for_source(*source))
                        .map(|role| role.label())
                        .collect::<BTreeSet<_>>();
                    let expected_role_label = match candidate.staff {
                        1 => crate::structural_write_authority::SerializedVoicePresentationRole::UpperPrimary.label(),
                        2 => crate::structural_write_authority::SerializedVoicePresentationRole::LowerPrimary.label(),
                        _ => "unsupported_staff",
                    };
                    if channels.len() == 1
                        && roles.len() == 1
                        && roles.contains(expected_role_label)
                    {
                        batch_target_realized = batch_target_realized.saturating_add(1);
                    } else {
                        batch_target_not_realized = batch_target_not_realized.saturating_add(1);
                    }
                }
                batch_external_partition_preserved =
                    source_partition_signature(baseline_channels, &batch_external_source_ids)
                        == source_partition_signature(&batch_channels, &batch_external_source_ids);
                batch_external_roles_preserved =
                    source_role_signature(baseline_channels, &batch_external_source_ids)
                        == source_role_signature(&batch_channels, &batch_external_source_ids);
                if batch_target_not_realized == 0
                    && batch_external_partition_preserved
                    && batch_external_roles_preserved
                {
                    "BATCH_COUNTERFACTUAL_TARGET_RELATIONS_REALIZED_EXTERNAL_RELATIONS_PRESERVED"
                        .to_string()
                } else if batch_target_not_realized == 0 {
                    "BATCH_COUNTERFACTUAL_TARGET_RELATIONS_REALIZED_WITH_EXTERNAL_EFFECTS"
                        .to_string()
                } else {
                    "BATCH_COUNTERFACTUAL_TARGET_RELATIONS_NOT_FULLY_REALIZED".to_string()
                }
            }
            Err(error) => format!("BLOCKED_BATCH_COUNTERFACTUAL_PLANNER_REJECTED:{error}"),
        }
    };

    rows.sort();
    let _ = writeln!(out, "Candidates: {}", candidates.len());
    let _ = writeln!(out, "BaselineChannelCount: {}", baseline_channels.channel_count());
    let _ = writeln!(out, "CurrentSplitCandidates: {current_split_candidates}");
    let _ = writeln!(out, "CandidateCounterfactualUniqueBinding: {unique_binding_candidates}");
    let _ = writeln!(out, "CandidateCounterfactualMultiOccurrenceBlocked: {multi_occurrence_blocked}");
    let _ = writeln!(out, "CandidateCounterfactualOtherInputBlocked: {other_input_blocked}");
    let _ = writeln!(out, "CandidateCounterfactualPlannerRejected: {planner_rejected}");
    let _ = writeln!(out, "CounterfactualTargetRealized: {target_realized}");
    let _ = writeln!(out, "CounterfactualTargetNotRealized: {target_not_realized}");
    let _ = writeln!(
        out,
        "ExternalPartitionPreserved: {}",
        if local_external_partition_preserved_all { "yes" } else { "no" }
    );
    let _ = writeln!(
        out,
        "ExternalPresentationRolesPreserved: {}",
        if local_external_roles_preserved_all { "yes" } else { "no" }
    );
    let _ = writeln!(out, "LocallyGreenCandidates: {}", locally_green.len());
    let _ = writeln!(out, "BatchSourceDisjointCandidates: {}", batch_candidates.len());
    let _ = writeln!(out, "BatchSourceOverlapExcludedCandidates: {batch_overlap_excluded}");
    let _ = writeln!(out, "BatchTargetRealized: {batch_target_realized}");
    let _ = writeln!(out, "BatchTargetNotRealized: {batch_target_not_realized}");
    let _ = writeln!(
        out,
        "BatchExternalPartitionPreserved: {}",
        if batch_external_partition_preserved { "yes" } else { "no" }
    );
    let _ = writeln!(
        out,
        "BatchExternalPresentationRolesPreserved: {}",
        if batch_external_roles_preserved { "yes" } else { "no" }
    );
    let _ = writeln!(out, "BatchVerdict: {batch_verdict}");

    let overall_verdict = if locally_green.is_empty() {
        if multi_occurrence_blocked > 0 && other_input_blocked == 0 && planner_rejected == 0 {
            "BLOCKED_INCOMPLETE_MULTI_OCCURRENCE_REPRESENTATION_BINDING"
        } else {
            "NO_LOCALLY_GREEN_COUNTERFACTUAL_CANDIDATE"
        }
    } else if target_not_realized == 0
        && local_external_partition_preserved_all
        && local_external_roles_preserved_all
    {
        "LOCAL_COUNTERFACTUAL_TARGET_RELATIONS_REALIZED_EXTERNAL_RELATIONS_PRESERVED"
    } else {
        "LOCAL_COUNTERFACTUAL_HAS_BLOCKED_OR_EXTERNAL_EFFECT_CASES"
    };
    let _ = writeln!(out, "Verdict: {overall_verdict}");
    for row in rows {
        let _ = writeln!(out, "Row: {row}");
    }
    projection.append_counterfactual_body(&out);
}

fn visible_pitched_channel_conflicts(
    source_to_channel: &BTreeMap<usize, usize>,
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
) -> BTreeSet<(usize, u8, usize, usize)> {
    let mut grouped = BTreeMap::<(usize, usize, u8, usize), (i64, i64)>::new();
    for row in occupancy_rows.iter().filter(|row| {
        row.visible && !row.is_rest && row.pitch_midi.is_some() && row.source_event_id.is_some()
    }) {
        let Some(source_event_id) = row.source_event_id else {
            continue;
        };
        let Some(&channel) = source_to_channel.get(&source_event_id) else {
            continue;
        };
        let key = (channel, row.measure, row.staff, row.atomic_group_id);
        let start = row.represented_onset;
        let end = row
            .represented_onset
            .saturating_add(row.represented_duration);
        grouped
            .entry(key)
            .and_modify(|span| {
                span.0 = span.0.min(start);
                span.1 = span.1.max(end);
            })
            .or_insert((start, end));
    }

    let mut by_scope = BTreeMap::<(usize, usize, u8), Vec<(usize, i64, i64)>>::new();
    for ((channel, measure, staff, group_id), (start, end)) in grouped {
        by_scope
            .entry((channel, measure, staff))
            .or_default()
            .push((group_id, start, end));
    }
    let mut conflicts = BTreeSet::new();
    for ((_channel, measure, staff), mut groups) in by_scope {
        groups.sort_by_key(|row| (row.1, row.2, row.0));
        for left_index in 0..groups.len() {
            for right_index in left_index + 1..groups.len() {
                let left = groups[left_index];
                let right = groups[right_index];
                if left.1 < right.2 && right.1 < left.2 {
                    conflicts.insert((
                        measure,
                        staff,
                        left.0.min(right.0),
                        left.0.max(right.0),
                    ));
                }
            }
        }
    }
    conflicts
}

pub(crate) fn append_notation_voice_metric_line_baseline_overlay_audit(
    projection: &mut NotationVoicePreSerializationConstraintClosureAuditProjection,
    source_rows: &[PreSerializationSourceAttackRow],
    occurrence_rows: &[PreSerializationOccurrenceRow],
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
    baseline_channels: &crate::structural_write_authority::FrozenSerializationVoiceChannelPlan,
) {
    let candidates = projection.metric_line_candidates().to_vec();
    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(out, "MetricLineBaselineOverlayCounterfactual: read_only");
    let _ = writeln!(out, "MetricLineOverlayProjectionOnly: true");
    let _ = writeln!(out, "MetricLineOverlayAuthority: none");
    let _ = writeln!(out, "MetricLineOverlayMusicMutation: nein");
    let _ = writeln!(out, "MetricLineOverlayRuleChange: nein");
    let _ = writeln!(out, "MetricLineOverlaySelection: beam_free_voice_label_free_metric_notation_line_only");
    let _ = writeln!(out, "MetricLineOverlayTransformation: preserve_baseline_channel_plan_and_rebind_only_candidate_sources_to_one_existing_same_staff_primary_channel");
    let _ = writeln!(out, "MetricLineOverlayNonCandidateInvariant: source_to_channel_and_presentation_role_exactly_unchanged");
    let _ = writeln!(out, "MetricLineOverlayMultiplicityPolicy: candidate_source_attack_must_have_exactly_one_physical_occurrence_before_source_level_channel_overlay");
    let _ = writeln!(out, "MetricLineOverlayPurpose: test_minimal_baseline_preserving_materialization_without_global_channel_replanning");

    if candidates.is_empty() {
        let _ = writeln!(out, "MetricLineOverlayCandidates: 0");
        let _ = writeln!(out, "MetricLineOverlayVerdict: NO_BEAM_FREE_METRIC_LINE_CANDIDATES");
        projection.append_counterfactual_body(&out);
        return;
    }

    let all_source_ids = source_rows
        .iter()
        .map(|row| row.source_event_id)
        .collect::<BTreeSet<_>>();
    let mut baseline_source_to_channel = BTreeMap::<usize, usize>::new();
    let mut baseline_missing_sources = Vec::<usize>::new();
    for source in &all_source_ids {
        if let Some(channel) = baseline_channels.channel_for_source(*source) {
            baseline_source_to_channel.insert(*source, channel);
        } else {
            baseline_missing_sources.push(*source);
        }
    }
    if !baseline_missing_sources.is_empty() {
        let _ = writeln!(
            out,
            "MetricLineOverlayInputFailure: baseline_missing_sources=[{}]",
            baseline_missing_sources
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        );
        let _ = writeln!(out, "MetricLineOverlayVerdict: BLOCKED_INCOMPLETE_BASELINE_CHANNEL_BINDING");
        projection.append_counterfactual_body(&out);
        return;
    }

    let mut occurrence_count_by_identity = BTreeMap::<(usize, usize), usize>::new();
    for occurrence in occurrence_rows {
        *occurrence_count_by_identity
            .entry((occurrence.source_event_id, occurrence.attack_id))
            .or_default() += 1;
    }
    let baseline_conflicts =
        visible_pitched_channel_conflicts(&baseline_source_to_channel, occupancy_rows);

    let mut rows = Vec::<String>::new();
    let mut locally_green = Vec::<(PreSerializationMetricNotationLineCandidate, usize)>::new();
    let mut current_split_candidates = 0usize;
    let mut already_realized_candidates = 0usize;
    let mut multi_occurrence_blocked = 0usize;
    let mut occurrence_binding_blocked = 0usize;
    let mut role_anchor_blocked = 0usize;
    let mut target_realized = 0usize;
    let mut external_binding_preserved = 0usize;
    let mut new_visible_conflict_blocked = 0usize;

    for candidate in &candidates {
        let candidate_sources = candidate
            .source_event_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut failures = Vec::<String>::new();
        let mut has_multi_occurrence = false;
        if candidate_sources.len() != candidate.source_event_ids.len() {
            failures.push("duplicate_source_event_id_within_metric_line".to_string());
        }
        for member in &candidate.members {
            let count = occurrence_count_by_identity
                .get(&(member.source_event_id, member.attack_id))
                .copied()
                .unwrap_or(0);
            if count != 1 {
                has_multi_occurrence |= count > 1;
                failures.push(format!(
                    "occurrence_multiplicity:{}:{}:{}",
                    member.source_event_id, member.attack_id, count
                ));
            }
        }
        if !failures.is_empty() {
            if has_multi_occurrence {
                multi_occurrence_blocked = multi_occurrence_blocked.saturating_add(1);
            } else {
                occurrence_binding_blocked = occurrence_binding_blocked.saturating_add(1);
            }
            rows.push(format!(
                "METRIC_LINE_OVERLAY metric_line_id={} measure={} staff={} beat={} interval={}-{} status={} failures=[{}]",
                candidate.metric_line_id,
                candidate.measure,
                candidate.staff,
                candidate.beat,
                candidate.interval_start,
                candidate.interval_end,
                if has_multi_occurrence {
                    "BLOCKED_INCOMPLETE_MULTI_OCCURRENCE_REPRESENTATION_BINDING"
                } else {
                    "BLOCKED_INCOMPLETE_OCCURRENCE_BINDING"
                },
                failures.join("|")
            ));
            continue;
        }

        let expected_primary = match candidate.staff {
            1 => crate::structural_write_authority::SerializedVoicePresentationRole::UpperPrimary,
            2 => crate::structural_write_authority::SerializedVoicePresentationRole::LowerPrimary,
            _ => {
                role_anchor_blocked = role_anchor_blocked.saturating_add(1);
                rows.push(format!(
                    "METRIC_LINE_OVERLAY metric_line_id={} measure={} staff={} beat={} status=BLOCKED_UNSUPPORTED_STAFF",
                    candidate.metric_line_id, candidate.measure, candidate.staff, candidate.beat
                ));
                continue;
            }
        };
        let expected_secondary = match candidate.staff {
            1 => crate::structural_write_authority::SerializedVoicePresentationRole::UpperSecondary,
            2 => crate::structural_write_authority::SerializedVoicePresentationRole::LowerSecondary,
            _ => unreachable!("staff checked above"),
        };

        let current_channels = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_channels.channel_for_source(*source))
            .collect::<BTreeSet<_>>();
        let current_roles = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_channels.role_for_source(*source))
            .collect::<BTreeSet<_>>();
        if current_channels.len() > 1 || current_roles.len() > 1 {
            current_split_candidates = current_split_candidates.saturating_add(1);
        }
        if current_channels.len() == 1
            && current_roles.len() == 1
            && current_roles.contains(&expected_primary)
        {
            already_realized_candidates = already_realized_candidates.saturating_add(1);
        }
        if current_roles
            .iter()
            .any(|role| *role != expected_primary && *role != expected_secondary)
        {
            role_anchor_blocked = role_anchor_blocked.saturating_add(1);
            rows.push(format!(
                "METRIC_LINE_OVERLAY metric_line_id={} measure={} staff={} beat={} interval={}-{} current_channels=[{}] current_roles=[{}] status=BLOCKED_CONTRADICTION_NON_PRIMARY_SECONDARY_ROLE",
                candidate.metric_line_id,
                candidate.measure,
                candidate.staff,
                candidate.beat,
                candidate.interval_start,
                candidate.interval_end,
                current_channels.iter().map(usize::to_string).collect::<Vec<_>>().join(","),
                current_roles.iter().map(|role| role.label()).collect::<Vec<_>>().join(","),
            ));
            continue;
        }
        let primary_target_channels = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| {
                if baseline_channels.role_for_source(*source) == Some(expected_primary) {
                    baseline_channels.channel_for_source(*source)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>();
        if primary_target_channels.len() != 1 {
            role_anchor_blocked = role_anchor_blocked.saturating_add(1);
            rows.push(format!(
                "METRIC_LINE_OVERLAY metric_line_id={} measure={} staff={} beat={} interval={}-{} primary_target_channels=[{}] status=BLOCKED_INCOMPLETE_UNIQUE_EXISTING_PRIMARY_CHANNEL_ANCHOR",
                candidate.metric_line_id,
                candidate.measure,
                candidate.staff,
                candidate.beat,
                candidate.interval_start,
                candidate.interval_end,
                primary_target_channels.iter().map(usize::to_string).collect::<Vec<_>>().join(","),
            ));
            continue;
        }
        let target_channel = *primary_target_channels
            .iter()
            .next()
            .expect("one primary target channel");

        let mut overlay = baseline_source_to_channel.clone();
        for source in &candidate.source_event_ids {
            overlay.insert(*source, target_channel);
        }
        let external_sources = all_source_ids
            .difference(&candidate_sources)
            .copied()
            .collect::<Vec<_>>();
        let external_preserved = external_sources
            .iter()
            .all(|source| overlay.get(source) == baseline_source_to_channel.get(source));
        if external_preserved {
            external_binding_preserved = external_binding_preserved.saturating_add(1);
        }
        let overlay_candidate_channels = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| overlay.get(source).copied())
            .collect::<BTreeSet<_>>();
        let realized = overlay_candidate_channels.len() == 1
            && overlay_candidate_channels.contains(&target_channel);
        if realized {
            target_realized = target_realized.saturating_add(1);
        }

        let overlay_conflicts = visible_pitched_channel_conflicts(&overlay, occupancy_rows);
        let new_conflicts = overlay_conflicts
            .difference(&baseline_conflicts)
            .copied()
            .collect::<Vec<_>>();
        if !new_conflicts.is_empty() {
            new_visible_conflict_blocked = new_visible_conflict_blocked.saturating_add(1);
            rows.push(format!(
                "METRIC_LINE_OVERLAY metric_line_id={} measure={} staff={} beat={} interval={}-{} target_channel={} target_relation_realized={} external_bindings_preserved={} new_visible_channel_conflicts=[{}] status=BLOCKED_COUNTERFACTUAL_NEW_VISIBLE_CHANNEL_CONFLICT",
                candidate.metric_line_id,
                candidate.measure,
                candidate.staff,
                candidate.beat,
                candidate.interval_start,
                candidate.interval_end,
                target_channel,
                if realized { "yes" } else { "no" },
                if external_preserved { "yes" } else { "no" },
                new_conflicts
                    .iter()
                    .map(|(measure, staff, left, right)| format!("m{}:s{}:{}-{}", measure, staff, left, right))
                    .collect::<Vec<_>>()
                    .join("|")
            ));
            continue;
        }

        let status = if realized && external_preserved {
            locally_green.push((candidate.clone(), target_channel));
            "OVERLAY_TARGET_RELATION_REALIZED_BASELINE_EXTERNAL_BINDINGS_PRESERVED"
        } else if !realized {
            "BLOCKED_COUNTERFACTUAL_TARGET_RELATION_NOT_REALIZED"
        } else {
            "BLOCKED_COUNTERFACTUAL_EXTERNAL_BINDING_CHANGE"
        };
        rows.push(format!(
            "METRIC_LINE_OVERLAY metric_line_id={} measure={} staff={} beat={} interval={}-{} sources=[{}] current_channels=[{}] current_roles=[{}] target_channel={} target_role={} overlay_channels=[{}] target_relation_realized={} external_bindings_preserved={} new_visible_channel_conflicts=0 status={}",
            candidate.metric_line_id,
            candidate.measure,
            candidate.staff,
            candidate.beat,
            candidate.interval_start,
            candidate.interval_end,
            candidate.source_event_ids.iter().map(usize::to_string).collect::<Vec<_>>().join(","),
            current_channels.iter().map(usize::to_string).collect::<Vec<_>>().join(","),
            current_roles.iter().map(|role| role.label()).collect::<Vec<_>>().join(","),
            target_channel,
            expected_primary.label(),
            overlay_candidate_channels.iter().map(usize::to_string).collect::<Vec<_>>().join(","),
            if realized { "yes" } else { "no" },
            if external_preserved { "yes" } else { "no" },
            status,
        ));
    }

    let mut source_candidate_count = BTreeMap::<usize, usize>::new();
    for (candidate, _) in &locally_green {
        for source in candidate.source_event_ids.iter().copied().collect::<BTreeSet<_>>() {
            *source_candidate_count.entry(source).or_default() += 1;
        }
    }
    let batch_candidates = locally_green
        .iter()
        .filter(|(candidate, _)| {
            candidate
                .source_event_ids
                .iter()
                .all(|source| source_candidate_count.get(source).copied() == Some(1))
        })
        .cloned()
        .collect::<Vec<_>>();
    let batch_overlap_excluded = locally_green.len().saturating_sub(batch_candidates.len());
    let mut batch_overlay = baseline_source_to_channel.clone();
    let mut batch_candidate_sources = BTreeSet::<usize>::new();
    for (candidate, target_channel) in &batch_candidates {
        for source in &candidate.source_event_ids {
            batch_candidate_sources.insert(*source);
            batch_overlay.insert(*source, *target_channel);
        }
    }
    let batch_external_sources = all_source_ids
        .difference(&batch_candidate_sources)
        .copied()
        .collect::<Vec<_>>();
    let batch_external_preserved = batch_external_sources
        .iter()
        .all(|source| batch_overlay.get(source) == baseline_source_to_channel.get(source));
    let batch_new_conflicts = visible_pitched_channel_conflicts(&batch_overlay, occupancy_rows)
        .difference(&baseline_conflicts)
        .copied()
        .collect::<Vec<_>>();
    let mut batch_target_realized = 0usize;
    for (candidate, target_channel) in &batch_candidates {
        if candidate
            .source_event_ids
            .iter()
            .all(|source| batch_overlay.get(source).copied() == Some(*target_channel))
        {
            batch_target_realized = batch_target_realized.saturating_add(1);
        }
    }
    let batch_target_not_realized = batch_candidates.len().saturating_sub(batch_target_realized);
    let batch_verdict = if batch_candidates.is_empty() {
        "NO_BATCH_ELIGIBLE_SOURCE_DISJOINT_METRIC_LINES"
    } else if batch_target_not_realized == 0
        && batch_external_preserved
        && batch_new_conflicts.is_empty()
    {
        "BASELINE_OVERLAY_BATCH_TARGET_RELATIONS_REALIZED_EXTERNAL_BINDINGS_PRESERVED_NO_NEW_VISIBLE_CHANNEL_CONFLICT"
    } else {
        "BASELINE_OVERLAY_BATCH_BLOCKED"
    };

    rows.sort();
    rows.dedup();
    let _ = writeln!(out, "MetricLineOverlayCandidates: {}", candidates.len());
    let _ = writeln!(out, "MetricLineOverlayCurrentSplitCandidates: {current_split_candidates}");
    let _ = writeln!(out, "MetricLineOverlayAlreadyPrimaryCandidates: {already_realized_candidates}");
    let _ = writeln!(out, "MetricLineOverlayMultiOccurrenceBlocked: {multi_occurrence_blocked}");
    let _ = writeln!(out, "MetricLineOverlayOccurrenceBindingBlocked: {occurrence_binding_blocked}");
    let _ = writeln!(out, "MetricLineOverlayRoleAnchorBlocked: {role_anchor_blocked}");
    let _ = writeln!(out, "MetricLineOverlayNewVisibleConflictBlocked: {new_visible_conflict_blocked}");
    let _ = writeln!(out, "MetricLineOverlayTargetRealized: {target_realized}");
    let _ = writeln!(out, "MetricLineOverlayExternalBindingPreserved: {external_binding_preserved}");
    let _ = writeln!(out, "MetricLineOverlayLocallyGreen: {}", locally_green.len());
    let _ = writeln!(out, "MetricLineOverlayBatchSourceDisjointCandidates: {}", batch_candidates.len());
    let _ = writeln!(out, "MetricLineOverlayBatchSourceOverlapExcluded: {batch_overlap_excluded}");
    let _ = writeln!(out, "MetricLineOverlayBatchTargetRealized: {batch_target_realized}");
    let _ = writeln!(out, "MetricLineOverlayBatchTargetNotRealized: {batch_target_not_realized}");
    let _ = writeln!(out, "MetricLineOverlayBatchExternalBindingsPreserved: {}", if batch_external_preserved { "yes" } else { "no" });
    let _ = writeln!(out, "MetricLineOverlayBatchNewVisibleChannelConflicts: {}", batch_new_conflicts.len());
    let _ = writeln!(out, "MetricLineOverlayBatchVerdict: {batch_verdict}");
    let overall_verdict = if locally_green.is_empty() {
        "NO_LOCALLY_GREEN_BEAM_FREE_METRIC_LINE_OVERLAY"
    } else if batch_verdict
        == "BASELINE_OVERLAY_BATCH_TARGET_RELATIONS_REALIZED_EXTERNAL_BINDINGS_PRESERVED_NO_NEW_VISIBLE_CHANNEL_CONFLICT"
    {
        "BEAM_FREE_METRIC_LINE_BASELINE_OVERLAY_COUNTERFACTUAL_GREEN"
    } else {
        "BEAM_FREE_METRIC_LINE_LOCAL_GREEN_BATCH_OPEN"
    };
    let _ = writeln!(out, "MetricLineOverlayVerdict: {overall_verdict}");
    for row in rows {
        let _ = writeln!(out, "MetricLineOverlayRow: {row}");
    }
    projection.append_counterfactual_body(&out);

    let materialization_safety_baseline_bindings = source_rows
        .iter()
        .filter_map(|row| {
            let channel = baseline_channels.channel_for_source(row.source_event_id)?;
            let role = baseline_channels.role_for_source(row.source_event_id)?;
            Some(MaterializationSafetyBaselineBindingRow {
                source_event_id: row.source_event_id,
                channel,
                role_label: role.label().to_string(),
            })
        })
        .collect::<Vec<_>>();
    append_notation_voice_materialization_safety_conjunction_audit(
        projection,
        source_rows,
        occurrence_rows,
        occupancy_rows,
        &materialization_safety_baseline_bindings,
    );
}

pub(crate) fn append_notation_voice_upstream_lineage_preservation_audit(
    projection: &mut NotationVoicePreSerializationConstraintClosureAuditProjection,
    source_rows: &[PreSerializationSourceAttackRow],
    baseline_channels: &crate::structural_write_authority::FrozenSerializationVoiceChannelPlan,
) {
    let candidates = projection.metric_line_candidates().to_vec();
    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(out, "UpstreamLineagePreservationAudit: read_only");
    let _ = writeln!(out, "UpstreamLineageProjectionOnly: true");
    let _ = writeln!(out, "UpstreamLineageAuthority: none");
    let _ = writeln!(out, "UpstreamLineageMusicMutation: nein");
    let _ = writeln!(out, "UpstreamLineageRuleChange: nein");
    let _ = writeln!(out, "UpstreamVoiceCarrier: CandidateMaterialAttackState.voice");
    let _ = writeln!(out, "UpstreamVoiceCaptureOrder: CandidateMaterialLineage_before_FrozenSoundingTemporalLanePlan_before_FrozenSerializationVoiceChannelPlan");
    let _ = writeln!(out, "UpstreamVoiceSelectionRole: preservation_precondition_only_not_MetricNotationLine_selector");
    let _ = writeln!(out, "UpstreamVoiceInterpretation: prove_existing_pre_lane_representation_voice_uniformity_not_infer_new_notation_voice");
    let _ = writeln!(out, "RequiredCompanionProof: r1006_metric_line_baseline_overlay_counterfactual_remains_separate_and_required");

    if candidates.is_empty() {
        let _ = writeln!(out, "UpstreamLineageCandidates: 0");
        let _ = writeln!(out, "UpstreamLineageVerdict: NO_BEAM_FREE_METRIC_LINE_CANDIDATES");
        projection.append_counterfactual_body(&out);
        return;
    }

    let mut source_by_identity =
        BTreeMap::<(usize, usize), Vec<&PreSerializationSourceAttackRow>>::new();
    for row in source_rows {
        source_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }

    let mut uniform = 0usize;
    let mut ambiguous = 0usize;
    let mut incomplete = 0usize;
    let mut uniform_current_split = 0usize;
    let mut uniform_already_single_channel = 0usize;
    let mut rows = Vec::<String>::new();

    for candidate in &candidates {
        let mut failures = Vec::<String>::new();
        let mut voices = BTreeSet::<String>::new();
        let mut member_voices = Vec::<String>::new();
        for member in &candidate.members {
            let identity = (member.source_event_id, member.attack_id);
            match source_by_identity.get(&identity) {
                Some(matches) if matches.len() == 1 => {
                    let source = matches[0];
                    if source.staff != candidate.staff {
                        failures.push(format!(
                            "source_staff_mismatch:{}:{}:{}:{}",
                            member.source_event_id, member.attack_id, source.staff, candidate.staff
                        ));
                        continue;
                    }
                    if source.pre_lane_voice.is_empty() {
                        failures.push(format!(
                            "empty_pre_lane_voice:{}:{}",
                            member.source_event_id, member.attack_id
                        ));
                        continue;
                    }
                    voices.insert(source.pre_lane_voice.clone());
                    member_voices.push(format!(
                        "{}:{}:{}",
                        member.source_event_id, member.attack_id, source.pre_lane_voice
                    ));
                }
                Some(matches) => failures.push(format!(
                    "source_identity_multiplicity:{}:{}:{}",
                    member.source_event_id,
                    member.attack_id,
                    matches.len()
                )),
                None => failures.push(format!(
                    "missing_source_identity:{}:{}",
                    member.source_event_id, member.attack_id
                )),
            }
        }

        let current_channels = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_channels.channel_for_source(*source))
            .collect::<BTreeSet<_>>();
        let current_roles = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_channels.role_for_source(*source))
            .map(|role| role.label())
            .collect::<BTreeSet<_>>();

        let status = if !failures.is_empty() || member_voices.len() != candidate.members.len() {
            incomplete = incomplete.saturating_add(1);
            "BLOCKED_INCOMPLETE_UPSTREAM_VOICE_BINDING"
        } else if voices.len() != 1 {
            ambiguous = ambiguous.saturating_add(1);
            "BLOCKED_UPSTREAM_VOICE_AMBIGUITY"
        } else {
            uniform = uniform.saturating_add(1);
            if current_channels.len() > 1 || current_roles.len() > 1 {
                uniform_current_split = uniform_current_split.saturating_add(1);
            } else {
                uniform_already_single_channel = uniform_already_single_channel.saturating_add(1);
            }
            "UPSTREAM_LINEAGE_PRESERVATION_PROVEN_ANALYSIS_ONLY"
        };

        rows.push(format!(
            "UPSTREAM_LINEAGE metric_line_id={} measure={} staff={} beat={} source_ids=[{}] member_pre_lane_voices=[{}] distinct_pre_lane_voices=[{}] upstream_voice_uniform={} current_channels=[{}] current_roles=[{}] failures=[{}] status={}",
            candidate.metric_line_id,
            candidate.measure,
            candidate.staff,
            candidate.beat,
            candidate
                .source_event_ids
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            member_voices.join("|"),
            voices.iter().cloned().collect::<Vec<_>>().join(","),
            if failures.is_empty() && voices.len() == 1 { "yes" } else { "no" },
            current_channels
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            current_roles.iter().copied().collect::<Vec<_>>().join(","),
            failures.join("|"),
            status,
        ));
    }

    rows.sort();
    rows.dedup();
    let _ = writeln!(out, "UpstreamLineageCandidates: {}", candidates.len());
    let _ = writeln!(out, "UpstreamVoiceUniformCandidates: {uniform}");
    let _ = writeln!(out, "UpstreamVoiceAmbiguousCandidates: {ambiguous}");
    let _ = writeln!(out, "UpstreamVoiceIncompleteCandidates: {incomplete}");
    let _ = writeln!(out, "UpstreamVoiceUniformCurrentSplitCandidates: {uniform_current_split}");
    let _ = writeln!(out, "UpstreamVoiceUniformAlreadySingleChannelCandidates: {uniform_already_single_channel}");
    let verdict = if uniform == 0 {
        if ambiguous > 0 && incomplete == 0 {
            "NO_UPSTREAM_UNIFORM_CANDIDATE_AMBIGUITY_BLOCKS_ALL"
        } else {
            "NO_UPSTREAM_UNIFORM_CANDIDATE_INCOMPLETE_OR_AMBIGUOUS"
        }
    } else if ambiguous == 0 && incomplete == 0 {
        "UPSTREAM_LINEAGE_PRESERVATION_PROVEN_FOR_ALL_METRIC_LINES_ANALYSIS_ONLY"
    } else {
        "UPSTREAM_LINEAGE_PRESERVATION_PROVEN_FOR_SUBSET_BLOCKED_CASES_REMAIN"
    };
    let _ = writeln!(out, "UpstreamLineageVerdict: {verdict}");
    for row in rows {
        let _ = writeln!(out, "UpstreamLineageRow: {row}");
    }
    projection.append_counterfactual_body(&out);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MaterializationSafetyBaselineBindingRow {
    pub(crate) source_event_id: usize,
    pub(crate) channel: usize,
    pub(crate) role_label: String,
}

fn append_notation_voice_materialization_safety_conjunction_audit(
    projection: &mut NotationVoicePreSerializationConstraintClosureAuditProjection,
    source_rows: &[PreSerializationSourceAttackRow],
    occurrence_rows: &[PreSerializationOccurrenceRow],
    occupancy_rows: &[PreSerializationVisibleOccupancyRow],
    baseline_bindings: &[MaterializationSafetyBaselineBindingRow],
) {
    let candidates = projection.metric_line_candidates().to_vec();
    let mut out = String::new();
    let _ = writeln!(out);
    let _ = writeln!(out, "MaterializationSafetyConjunctionAudit: read_only");
    let _ = writeln!(out, "MaterializationSafetyProjectionOnly: true");
    let _ = writeln!(out, "MaterializationSafetyAuthority: none");
    let _ = writeln!(out, "MaterializationSafetyMusicMutation: nein");
    let _ = writeln!(out, "MaterializationSafetyRuleChange: nein");
    let _ = writeln!(
        out,
        "MaterializationSafetyInputBoundary: audit_visible_primitive_channel_role_projection"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyTarget: existing_same_staff_primary_channel_only"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetySelection: beam_free_voice_label_free_MetricNotationLine"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyRequiredConjunction: exact_unique_occurrence_binding+uniform_pre_lane_voice+complete_same_staff_visible_occupancy+unique_existing_primary_channel+baseline_overlay_target_realized+no_new_visible_channel_conflict"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyNonCandidateInvariant: all_non_candidate_source_channel_bindings_unchanged_by_construction"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyInterpretation: proves_S0_channel_rebinding_is_locally_safe_analysis_only; does_not_authorize_writer_or_claim_PDF_improvement"
    );

    if candidates.is_empty() {
        let _ = writeln!(out, "MaterializationSafetyCandidates: 0");
        let _ = writeln!(
            out,
            "MaterializationSafetyVerdict: NO_BEAM_FREE_METRIC_LINE_CANDIDATES"
        );
        projection.append_counterfactual_body(&out);
        return;
    }

    let mut source_by_identity =
        BTreeMap::<(usize, usize), Vec<&PreSerializationSourceAttackRow>>::new();
    for row in source_rows {
        source_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }

    let mut occurrence_by_identity =
        BTreeMap::<(usize, usize), Vec<&PreSerializationOccurrenceRow>>::new();
    let mut occurrence_by_locator =
        BTreeMap::<(usize, usize), Vec<&PreSerializationOccurrenceRow>>::new();
    for row in occurrence_rows {
        occurrence_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
        occurrence_by_locator
            .entry((row.measure, row.pitched_ordinal))
            .or_default()
            .push(row);
    }

    let all_source_ids = source_rows
        .iter()
        .map(|row| row.source_event_id)
        .collect::<BTreeSet<_>>();
    let mut baseline_rows_by_source =
        BTreeMap::<usize, Vec<&MaterializationSafetyBaselineBindingRow>>::new();
    for row in baseline_bindings {
        baseline_rows_by_source
            .entry(row.source_event_id)
            .or_default()
            .push(row);
    }
    let mut baseline_source_to_channel = BTreeMap::<usize, usize>::new();
    let mut baseline_source_to_role = BTreeMap::<usize, String>::new();
    let mut baseline_missing_or_ambiguous = Vec::<usize>::new();
    for source in &all_source_ids {
        match baseline_rows_by_source.get(source) {
            Some(rows) if rows.len() == 1 => {
                baseline_source_to_channel.insert(*source, rows[0].channel);
                baseline_source_to_role.insert(*source, rows[0].role_label.clone());
            }
            _ => baseline_missing_or_ambiguous.push(*source),
        }
    }
    if !baseline_missing_or_ambiguous.is_empty() {
        let _ = writeln!(
            out,
            "MaterializationSafetyBaselineBindingMissingOrAmbiguousSources: [{}]",
            baseline_missing_or_ambiguous
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",")
        );
        let _ = writeln!(
            out,
            "MaterializationSafetyVerdict: BLOCKED_INCOMPLETE_BASELINE_CHANNEL_BINDING"
        );
        projection.append_counterfactual_body(&out);
        return;
    }

    let baseline_conflicts =
        visible_pitched_channel_conflicts(&baseline_source_to_channel, occupancy_rows);

    let mut current_split = 0usize;
    let mut already_realized = 0usize;
    let mut safe = 0usize;
    let mut blocked_occurrence = 0usize;
    let mut blocked_upstream = 0usize;
    let mut blocked_occupancy = 0usize;
    let mut blocked_target = 0usize;
    let mut blocked_conflict = 0usize;
    let mut rows = Vec::<String>::new();

    for candidate in &candidates {
        let candidate_sources = candidate
            .source_event_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut failures = Vec::<String>::new();

        let mut pre_lane_voices = BTreeSet::<String>::new();
        for member in &candidate.members {
            let identity = (member.source_event_id, member.attack_id);
            match source_by_identity.get(&identity) {
                Some(matches) if matches.len() == 1 => {
                    let source = matches[0];
                    if source.staff != member.staff {
                        failures.push(format!(
                            "source_staff_mismatch:{}:{}",
                            member.source_event_id, member.attack_id
                        ));
                    }
                    pre_lane_voices.insert(source.pre_lane_voice.clone());
                }
                Some(matches) => failures.push(format!(
                    "source_identity_multiplicity:{}:{}:{}",
                    member.source_event_id,
                    member.attack_id,
                    matches.len()
                )),
                None => failures.push(format!(
                    "missing_source_identity:{}:{}",
                    member.source_event_id, member.attack_id
                )),
            }

            match occurrence_by_identity.get(&identity) {
                Some(matches) if matches.len() == 1 => {}
                Some(matches) => failures.push(format!(
                    "occurrence_identity_multiplicity:{}:{}:{}",
                    member.source_event_id,
                    member.attack_id,
                    matches.len()
                )),
                None => failures.push(format!(
                    "missing_occurrence_identity:{}:{}",
                    member.source_event_id, member.attack_id
                )),
            }

            let locator = (member.measure, member.pitched_ordinal);
            match occurrence_by_locator.get(&locator) {
                Some(matches)
                    if matches.len() == 1
                        && matches[0].source_event_id == member.source_event_id
                        && matches[0].attack_id == member.attack_id
                        && matches[0].staff == member.staff
                        && matches[0].represented_onset == member.represented_onset
                        && matches[0].represented_duration == member.represented_duration => {}
                Some(matches) => failures.push(format!(
                    "occurrence_locator_not_exact:{}:{}:{}",
                    member.measure,
                    member.pitched_ordinal,
                    matches.len()
                )),
                None => failures.push(format!(
                    "missing_occurrence_locator:{}:{}",
                    member.measure, member.pitched_ordinal
                )),
            }
        }

        let exact_occurrence_binding = !failures.iter().any(|failure| {
            failure.starts_with("source_")
                || failure.starts_with("missing_source")
                || failure.starts_with("occurrence_")
                || failure.starts_with("missing_occurrence")
        });
        if !exact_occurrence_binding {
            blocked_occurrence = blocked_occurrence.saturating_add(1);
        }

        let upstream_uniform = pre_lane_voices.len() == 1
            && candidate
                .members
                .iter()
                .all(|member| source_by_identity.contains_key(&(member.source_event_id, member.attack_id)));
        if !upstream_uniform {
            failures.push("upstream_pre_lane_voice_not_uniform".to_string());
            blocked_upstream = blocked_upstream.saturating_add(1);
        }

        let mut external_visible_occupancy = Vec::<String>::new();
        for row in occupancy_rows.iter().filter(|row| {
            row.visible && row.measure == candidate.measure && row.staff == candidate.staff
        }) {
            let row_start = row.represented_onset;
            let row_end = row
                .represented_onset
                .saturating_add(row.represented_duration);
            let overlaps =
                candidate.interval_start < row_end && row_start < candidate.interval_end;
            if !overlaps {
                continue;
            }

            if row.is_rest {
                external_visible_occupancy.push(format!(
                    "rest:{}-{}",
                    row_start, row_end
                ));
                continue;
            }

            match row.source_event_id {
                Some(source) if candidate_sources.contains(&source) => {}
                Some(source) => external_visible_occupancy.push(format!(
                    "pitched_source:{}:{}-{}",
                    source, row_start, row_end
                )),
                None => external_visible_occupancy.push(format!(
                    "unbound_pitched:{}-{}",
                    row_start, row_end
                )),
            }
        }
        external_visible_occupancy.sort();
        external_visible_occupancy.dedup();
        let occupancy_complete = external_visible_occupancy.is_empty();
        if !occupancy_complete {
            failures.push(format!(
                "external_visible_occupancy:[{}]",
                external_visible_occupancy.join("|")
            ));
            blocked_occupancy = blocked_occupancy.saturating_add(1);
        }

        let current_channels = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_source_to_channel.get(source).copied())
            .collect::<BTreeSet<_>>();
        let current_roles = candidate
            .source_event_ids
            .iter()
            .filter_map(|source| baseline_source_to_role.get(source).cloned())
            .collect::<BTreeSet<_>>();

        let (expected_primary, expected_secondary) = match candidate.staff {
            1 => ("upper_primary_run", "upper_secondary_run"),
            2 => ("lower_primary_run", "lower_secondary_run"),
            _ => {
                failures.push("unsupported_staff".to_string());
                ("unsupported_primary", "unsupported_secondary")
            }
        };

        let primary_target_sources = candidate
            .source_event_ids
            .iter()
            .copied()
            .filter(|source| {
                baseline_source_to_role
                    .get(source)
                    .is_some_and(|role| role.as_str() == expected_primary)
            })
            .collect::<BTreeSet<_>>();

        if current_roles
            .iter()
            .any(|role| role.as_str() != expected_primary && role.as_str() != expected_secondary)
        {
            failures.push("non_primary_secondary_role_present".to_string());
        }

        let primary_target_channels = candidate
            .source_event_ids
            .iter()
            .filter(|source| {
                baseline_source_to_role
                    .get(source)
                    .is_some_and(|role| role.as_str() == expected_primary)
            })
            .filter_map(|source| baseline_source_to_channel.get(source).copied())
            .collect::<BTreeSet<_>>();

        let target_channel = if primary_target_channels.len() == 1 {
            primary_target_channels.iter().next().copied()
        } else {
            None
        };
        if target_channel.is_none() {
            failures.push(format!(
                "unique_primary_channel_not_proven:{}",
                primary_target_channels
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            blocked_target = blocked_target.saturating_add(1);
        }

        let is_current_split = current_channels.len() > 1 || current_roles.len() > 1;
        if is_current_split {
            current_split = current_split.saturating_add(1);
        }

        let mut target_realized = false;
        let mut no_new_conflict = false;
        let mut overlay_channels = BTreeSet::<usize>::new();
        if let Some(target_channel) = target_channel {
            let mut overlay = baseline_source_to_channel.clone();
            for source in &candidate.source_event_ids {
                overlay.insert(*source, target_channel);
            }
            overlay_channels = candidate
                .source_event_ids
                .iter()
                .filter_map(|source| overlay.get(source).copied())
                .collect::<BTreeSet<_>>();
            target_realized =
                overlay_channels.len() == 1 && overlay_channels.contains(&target_channel);
            let overlay_conflicts = visible_pitched_channel_conflicts(&overlay, occupancy_rows);
            let new_conflicts = overlay_conflicts
                .difference(&baseline_conflicts)
                .copied()
                .collect::<Vec<_>>();
            no_new_conflict = new_conflicts.is_empty();
            if !no_new_conflict {
                failures.push(format!(
                    "new_visible_channel_conflicts:[{}]",
                    new_conflicts
                        .iter()
                        .map(|(measure, staff, left, right)| {
                            format!("m{}:s{}:{}-{}", measure, staff, left, right)
                        })
                        .collect::<Vec<_>>()
                        .join("|")
                ));
                blocked_conflict = blocked_conflict.saturating_add(1);
            }
            if !target_realized {
                failures.push("overlay_target_relation_not_realized".to_string());
            }
        }

        failures.sort();
        failures.dedup();

        let status = if !is_current_split
            && exact_occurrence_binding
            && upstream_uniform
            && occupancy_complete
            && target_realized
            && no_new_conflict
            && failures.is_empty()
        {
            already_realized = already_realized.saturating_add(1);
            "ALREADY_SINGLE_PRIMARY_RELATION_NO_WRITE_NEEDED"
        } else if is_current_split
            && exact_occurrence_binding
            && upstream_uniform
            && occupancy_complete
            && target_realized
            && no_new_conflict
            && failures.is_empty()
        {
            safe = safe.saturating_add(1);
            projection
                .materialization_safety_proven_metric_line_ids
                .insert(candidate.metric_line_id.clone());
            if let (Some(target_channel), Some(target_primary_source_event_id)) =
                (target_channel, primary_target_sources.iter().next().copied())
                && let Some(target_primary_attack_id) = candidate
                    .members
                    .iter()
                    .find(|member| member.source_event_id == target_primary_source_event_id)
                    .map(|member| member.attack_id)
            {
                projection
                    .materialization_safety_primary_target_evidence_rows
                    .push(MaterializationSafetyPrimaryTargetEvidenceRow {
                        metric_line_id: candidate.metric_line_id.clone(),
                        target_primary_source_event_id,
                        target_primary_attack_id,
                        target_channel,
                    });
            }
            "MATERIALIZATION_SAFETY_CONJUNCTION_PROVEN_ANALYSIS_ONLY"
        } else {
            "BLOCKED_MATERIALIZATION_SAFETY_CONJUNCTION"
        };

        rows.push(format!(
            "MATERIALIZATION_SAFETY metric_line_id={} measure={} staff={} beat={} interval={}-{} source_ids=[{}] pre_lane_voices=[{}] current_channels=[{}] current_roles=[{}] target_channel={} overlay_channels=[{}] exact_occurrence_binding={} upstream_uniform={} visible_occupancy_complete={} no_new_visible_channel_conflict={} failures=[{}] status={}",
            candidate.metric_line_id,
            candidate.measure,
            candidate.staff,
            candidate.beat,
            candidate.interval_start,
            candidate.interval_end,
            candidate
                .source_event_ids
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            pre_lane_voices.iter().cloned().collect::<Vec<_>>().join(","),
            current_channels
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            current_roles.iter().cloned().collect::<Vec<_>>().join(","),
            target_channel.map_or_else(|| "none".to_string(), |value| value.to_string()),
            overlay_channels
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
            if exact_occurrence_binding { "yes" } else { "no" },
            if upstream_uniform { "yes" } else { "no" },
            if occupancy_complete { "yes" } else { "no" },
            if no_new_conflict { "yes" } else { "no" },
            failures.join("|"),
            status,
        ));
    }

    rows.sort();
    rows.dedup();
    let blocked = candidates
        .len()
        .saturating_sub(safe)
        .saturating_sub(already_realized);
    let _ = writeln!(out, "MaterializationSafetyCandidates: {}", candidates.len());
    let _ = writeln!(out, "MaterializationSafetyCurrentSplitCandidates: {current_split}");
    let _ = writeln!(out, "MaterializationSafetyAlreadyRealized: {already_realized}");
    let _ = writeln!(out, "MaterializationSafetyProvenAnalysisOnly: {safe}");
    let _ = writeln!(out, "MaterializationSafetyBlocked: {blocked}");
    let _ = writeln!(
        out,
        "MaterializationSafetyBlockedOccurrenceBinding: {blocked_occurrence}"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyBlockedUpstreamLineage: {blocked_upstream}"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyBlockedVisibleOccupancy: {blocked_occupancy}"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyBlockedTargetChannel: {blocked_target}"
    );
    let _ = writeln!(
        out,
        "MaterializationSafetyBlockedNewVisibleConflict: {blocked_conflict}"
    );
    let verdict = if safe > 0 {
        if blocked == 0 {
            "MATERIALIZATION_SAFETY_CONJUNCTION_PROVEN_FOR_ALL_CURRENT_SPLIT_METRIC_LINES_ANALYSIS_ONLY"
        } else {
            "MATERIALIZATION_SAFETY_CONJUNCTION_PROVEN_FOR_SUBSET_BLOCKED_CASES_REMAIN"
        }
    } else if already_realized == candidates.len() {
        "NO_CURRENT_SPLIT_METRIC_LINE_REQUIRES_MATERIALIZATION"
    } else {
        "NO_MATERIALIZATION_SAFETY_CANDIDATE_PROVEN"
    };
    let _ = writeln!(out, "MaterializationSafetyVerdict: {verdict}");
    for row in rows {
        let _ = writeln!(out, "MaterializationSafetyRow: {row}");
    }
    projection.append_counterfactual_body(&out);
}

#[cfg(test)]
mod tests {
    fn baseline_pre_serialization_visible_occupancy_row() -> PreSerializationVisibleOccupancyRow {
        PreSerializationVisibleOccupancyRow {
            measure: 1,
            staff: 1,
            represented_onset: 0,
            represented_duration: 24,
            visible: true,
            is_rest: false,
            is_chord_member: false,
            pitch_midi: None,
            source_event_id: None,
            attack_id: None,
            atomic_group_id: 1,
            atomic_group_kind: PreSerializationAtomicGroupKind::VisiblePitched,
        }
    }


    use super::*;

    fn source(
        source_event_id: usize,
        onset: i64,
        duration: i64,
        lane: usize,
    ) -> PreSerializationSourceAttackRow {
        PreSerializationSourceAttackRow {
            source_event_id,
            attack_id: source_event_id + 100,
            measure: 1,
            semantic_onset: onset,
            semantic_duration: duration,
            pitch_midi: 60 + source_event_id as i32,
            staff: 1,
            absolute_start: onset,
            absolute_end: onset + duration,
            technical_lane: lane,
            pre_lane_voice: "1".to_string(),
        }
    }

    fn occurrence(
        source_event_id: usize,
        pitched_ordinal: usize,
        onset: i64,
        voice: &str,
        _group: usize,
    ) -> PreSerializationOccurrenceRow {
        PreSerializationOccurrenceRow {
            source_event_id,
            attack_id: source_event_id + 100,
            measure: 1,
            pitched_ordinal,
            represented_onset: onset,
            represented_duration: 24,
            pitch_midi: 60 + source_event_id as i32,
            staff: 1,
            current_voice: voice.to_string(),
        }
    }

    fn occupancy(row: &PreSerializationOccurrenceRow) -> PreSerializationVisibleOccupancyRow {
        PreSerializationVisibleOccupancyRow {
            measure: row.measure,
            staff: row.staff,
            represented_onset: row.represented_onset,
            represented_duration: row.represented_duration,
            pitch_midi: Some(row.pitch_midi),
            source_event_id: Some(row.source_event_id),
            attack_id: Some(row.attack_id),
            atomic_group_id: row.pitched_ordinal.saturating_add(1),
            ..baseline_pre_serialization_visible_occupancy_row()
        }
    }

    fn xml(voices: [&str; 4]) -> String {
        format!(
            r#"<score-partwise version="4.0"><part id="P1"><measure number="1"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>{}</voice><staff>1</staff><beam number="1">begin</beam></note>
<note><pitch><step>C</step><alter>1</alter><octave>4</octave></pitch><duration>24</duration><voice>{}</voice><staff>1</staff><beam number="1">continue</beam></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>{}</voice><staff>1</staff><beam number="1">continue</beam></note>
<note><pitch><step>D</step><alter>1</alter><octave>4</octave></pitch><duration>24</duration><voice>{}</voice><staff>1</staff><beam number="1">end</beam></note>
</measure></part></score-partwise>"#,
            voices[0], voices[1], voices[2], voices[3]
        )
    }

    fn clean_fixture(voices: [&str; 4]) -> (
        String,
        Vec<PreSerializationSourceAttackRow>,
        Vec<PreSerializationOccurrenceRow>,
        Vec<PreSerializationVisibleOccupancyRow>,
    ) {
        let source_rows = vec![
            source(0, 0, 48, 1),
            source(1, 24, 24, 2),
            source(2, 48, 48, 1),
            source(3, 72, 24, 2),
        ];
        let occurrence_rows = vec![
            occurrence(0, 0, 0, voices[0], 1),
            occurrence(1, 1, 24, voices[1], 2),
            occurrence(2, 2, 48, voices[2], 3),
            occurrence(3, 3, 72, voices[3], 4),
        ];
        let occupancy_rows = occurrence_rows.iter().map(occupancy).collect();
        (xml(voices), source_rows, occurrence_rows, occupancy_rows)
    }

    #[test]
    fn current_canonical_primary_beam_owner_receipt_is_source_bound_and_primary_only() {
        let receipt = build_canonical_primary_beam_owner_receipt("1:101-2:102-3:103", 3)
            .expect("source-bound current canonical owner receipt");
        assert_eq!(receipt.owner_kind, CANONICAL_PRIMARY_BEAM_OWNER_KIND);
        assert_eq!(
            receipt.owner_source_fnv64,
            CANONICAL_PRIMARY_BEAM_OWNER_SOURCE_FNV64_CURRENT,
        );
        assert_eq!(receipt.event_count, 3);
        assert_eq!(receipt.primary_states, vec!["begin", "continue", "end"]);
        assert_eq!(receipt.plan_fingerprint_fnv64, 0x27a3_8b22_98a8_f831);
        assert!(canonical_primary_beam_owner_source_is_bound());
    }

    #[test]
    fn canonical_primary_beam_owner_receipt_fails_closed_for_single_event() {
        assert!(build_canonical_primary_beam_owner_receipt("1:101", 1).is_none());
    }

    #[test]
    fn semantic_fingerprint_is_independent_of_current_voice_labels() {
        let (xml_a, source_rows, occurrence_a, occupancy_a) = clean_fixture(["1", "3", "1", "3"]);
        let projection_a = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml_a,
            &source_rows,
            &occurrence_a,
            &occupancy_a,
        );
        let (xml_b, _, occurrence_b, occupancy_b) = clean_fixture(["71", "72", "73", "74"]);
        let projection_b = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml_b,
            &source_rows,
            &occurrence_b,
            &occupancy_b,
        );
        let fingerprint_a = projection_a
            .body
            .lines()
            .find(|line| line.starts_with("SemanticPlanFingerprintFnv64: "))
            .expect("semantic fingerprint A");
        let fingerprint_b = projection_b
            .body
            .lines()
            .find(|line| line.starts_with("SemanticPlanFingerprintFnv64: "))
            .expect("semantic fingerprint B");
        assert_eq!(fingerprint_a, fingerprint_b);
        assert!(projection_a
            .body
            .contains("BLOCKED_INCOMPLETE_BEAM_WITNESS_PROVENANCE_AND_MATERIALIZATION"));
    }

    #[test]
    fn beam_witness_reports_exact_pre_final_chord_voice_free_origin_match() {
        let (xml, source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        let projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection_with_beam_origin(
                &xml,
                Some(&xml),
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        assert!(projection
            .body
            .contains("BeamWitnessPreFinalChordExactMatches: 1"));
        assert!(projection
            .body
            .contains("beam_witness_pre_final_chord_exact=yes"));
        assert!(projection
            .body
            .contains("BeamWitnessHistoricalOwner: unproven_even_when_pre_final_chord_exact"));
        assert_eq!(
            projection.pre_final_primary_beam_exact_proof_chain_ids().len(),
            1
        );
    }

    #[test]
    fn materialization_counterfactual_localizes_lane_and_sounding_overlap_authority_without_write() {
        let (xml, source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        let evidence = source_rows
            .iter()
            .enumerate()
            .map(|(index, row)| crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: index.saturating_add(1),
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                measure: row.measure,
                raw_note_index: index,
                onset: row.semantic_onset,
                duration: 24,
                pitch: row.pitch_midi,
                staff: row.staff,
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            })
            .collect::<Vec<_>>();
        let (canonical, _) =
            crate::structural_write_authority::plan_canonical_final_chord_structure_once(
                &xml,
                &evidence,
            )
            .expect("canonical FinalChord plan");
        let frozen_attacks = source_rows
            .iter()
            .map(|row| crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                absolute_start: row.absolute_start,
                absolute_end: row.absolute_end,
                staff: row.staff,
                technical_lane: row.technical_lane,
            })
            .collect::<Vec<_>>();
        let baseline =
            crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
                &canonical,
                &frozen_attacks,
            )
            .expect("baseline voice channels");
        let mut projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection_with_beam_origin(
                &xml,
                Some(&xml),
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        append_notation_voice_materialization_counterfactual_audit(
            &mut projection,
            &canonical,
            &source_rows,
            &occurrence_rows,
            &frozen_attacks,
            &baseline,
        );
        assert!(projection
            .body
            .contains("CurrentSplitCandidates: 1"));
        assert!(projection
            .body
            .contains("CounterfactualTargetRealized: 1"));
        assert!(projection
            .body
            .contains("target_relation_realized=yes"));
        assert!(projection
            .body
            .contains("ExternalPartitionPreserved: yes"));
        assert!(projection
            .body
            .contains("ExternalPresentationRolesPreserved: yes"));
        assert!(projection
            .body
            .contains("BatchSourceDisjointCandidates: 1"));
        assert!(projection.body.contains(
            "BatchVerdict: BATCH_COUNTERFACTUAL_TARGET_RELATIONS_REALIZED_EXTERNAL_RELATIONS_PRESERVED"
        ));
    }

    #[test]
    fn notated_channel_overlap_readback_names_the_view_that_hides_the_overlap() {
        let (_, source_rows, occurrence_rows, _) = clean_fixture(["1", "3", "1", "3"]);
        // Lane 1 sources share one channel, lane 2 sources another.
        let channel_of = |source: usize| match source {
            0 | 2 => Some((1, "upper_primary_run")),
            1 | 3 => Some((2, "upper_secondary_run")),
            _ => None,
        };

        let clean = notated_channel_overlap_readback(&source_rows, &occurrence_rows, channel_of);
        assert!(clean.starts_with("NotatedChannelOverlapReadback: read_only; Pairs 0;"));

        let mut longer = occurrence_rows.clone();
        longer[0].represented_duration = 72;
        let disjoint = notated_channel_overlap_readback(&source_rows, &longer, channel_of);
        assert!(disjoint.contains("Pairs 1; StaffViewDiffers 0; SoundingIntervalsDisjoint 1; SoundingOverlapSameStaff 0;"));
        assert!(disjoint.contains("channel=1 view=sounding_intervals_disjoint"));
        assert!(disjoint.contains("sounding_abs=0..48 lane=1 role=upper_primary_run"));

        let mut coincident = occurrence_rows.clone();
        coincident[2].represented_onset = 0;
        let same_event = notated_channel_overlap_readback(&source_rows, &coincident, channel_of);
        assert!(same_event.contains("Pairs 0;"));

        let mut moved = longer;
        moved[0].staff = 2;
        moved[2].staff = 2;
        let staff = notated_channel_overlap_readback(&source_rows, &moved, channel_of);
        assert!(staff.contains("Pairs 1; StaffViewDiffers 1;"));
        assert!(staff.contains("represented=m1:0/72:staff2 material=staff1:0/48"));

        let mut overlapping = occurrence_rows;
        overlapping[0].represented_duration = 72;
        let mut sounding = source_rows;
        sounding[0].absolute_end = 72;
        let same = notated_channel_overlap_readback(&sounding, &overlapping, channel_of);
        assert!(same.contains("SoundingOverlapSameStaff 1;"));
    }

    #[test]
    fn counterfactual_localizes_multi_occurrence_representation_without_global_abort() {
        let (xml, source_rows, mut occurrence_rows, occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        let mut duplicate = occurrence_rows[0].clone();
        duplicate.measure = 2;
        duplicate.pitched_ordinal = 0;
        occurrence_rows.push(duplicate);

        let evidence = source_rows
            .iter()
            .enumerate()
            .map(|(index, row)| crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: index.saturating_add(1),
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                measure: row.measure,
                raw_note_index: index,
                onset: row.semantic_onset,
                duration: 24,
                pitch: row.pitch_midi,
                staff: row.staff,
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            })
            .collect::<Vec<_>>();
        let (canonical, _) =
            crate::structural_write_authority::plan_canonical_final_chord_structure_once(
                &xml,
                &evidence,
            )
            .expect("canonical FinalChord plan");
        let frozen_attacks = source_rows
            .iter()
            .map(|row| crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                absolute_start: row.absolute_start,
                absolute_end: row.absolute_end,
                staff: row.staff,
                technical_lane: row.technical_lane,
            })
            .collect::<Vec<_>>();
        let baseline =
            crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
                &canonical,
                &frozen_attacks,
            )
            .expect("baseline voice channels");
        let mut projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection_with_beam_origin(
                &xml,
                Some(&xml),
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        append_notation_voice_materialization_counterfactual_audit(
            &mut projection,
            &canonical,
            &source_rows,
            &occurrence_rows,
            &frozen_attacks,
            &baseline,
        );
        assert!(projection
            .body
            .contains("CandidateCounterfactualMultiOccurrenceBlocked: 1"));
        assert!(projection.body.contains(
            "status=BLOCKED_INCOMPLETE_MULTI_OCCURRENCE_REPRESENTATION_BINDING"
        ));
        assert!(!projection.body.contains("CounterfactualInputFailures:"));
    }

    #[test]
    fn explicit_lane_pressure_edges_are_projected_for_self_induced_overlap() {
        let (xml, source_rows, occurrence_rows, occupancy_rows) = clean_fixture(["1", "3", "1", "3"]);
        let projection = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
        );
        assert!(projection.body.contains("0:100:lane1:end48>1:101:lane2:start24"));
        assert!(projection.body.contains("2:102:lane1:end96>3:103:lane2:start72"));
        assert!(projection.body.contains("CandidateClosedExceptBeamProvenanceAndMaterialization: 1"));
    }

    #[test]
    fn visible_rest_overlap_blocks_without_becoming_hard_different() {
        let (xml, source_rows, occurrence_rows, mut occupancy_rows) = clean_fixture(["1", "3", "1", "3"]);
        occupancy_rows.push(PreSerializationVisibleOccupancyRow {
            represented_onset: 24,
            is_rest: true,
            atomic_group_id: 90,
            atomic_group_kind: PreSerializationAtomicGroupKind::VisibleRest,
            ..baseline_pre_serialization_visible_occupancy_row()
        });
        let projection = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
        );
        assert!(projection.body.contains("BLOCKED_INCOMPLETE_REST_OCCUPANCY_OWNERSHIP"));
        assert!(!projection.body.contains("status=BLOCKED_CONTRADICTION_INDEPENDENT_PITCHED_OCCUPANCY"));
    }
    #[test]
    fn chord_extension_sharing_chain_anchor_is_atomic_not_independent_occupancy() {
        let (xml, source_rows, occurrence_rows, mut occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        occupancy_rows.push(PreSerializationVisibleOccupancyRow {
            is_chord_member: true,
            pitch_midi: Some(72),
            ..baseline_pre_serialization_visible_occupancy_row()
        });
        let projection = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
        );
        assert!(projection.body.contains("chord_extension_atomic_groups=[1]"));
        assert!(!projection
            .body
            .contains("status=BLOCKED_CONTRADICTION_INDEPENDENT_PITCHED_OCCUPANCY"));
        assert!(projection
            .body
            .contains("BLOCKED_INCOMPLETE_BEAM_WITNESS_PROVENANCE_AND_MATERIALIZATION"));
    }

    #[test]
    fn semantic_fingerprint_is_independent_of_opaque_source_and_attack_id_names() {
        let (xml, source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        let projection_a = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
        );

        let mut renamed_sources = source_rows.clone();
        for row in &mut renamed_sources {
            row.source_event_id = row.source_event_id.saturating_add(1000);
            row.attack_id = row.attack_id.saturating_add(2000);
        }
        let mut renamed_occurrences = occurrence_rows.clone();
        for row in &mut renamed_occurrences {
            row.source_event_id = row.source_event_id.saturating_add(1000);
            row.attack_id = row.attack_id.saturating_add(2000);
        }
        let mut renamed_occupancy = occupancy_rows.clone();
        for row in &mut renamed_occupancy {
            row.source_event_id = row.source_event_id.map(|id| id.saturating_add(1000));
            row.attack_id = row.attack_id.map(|id| id.saturating_add(2000));
        }
        let projection_b = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml,
            &renamed_sources,
            &renamed_occurrences,
            &renamed_occupancy,
        );

        let fingerprint_a = projection_a
            .body
            .lines()
            .find(|line| line.starts_with("SemanticPlanFingerprintFnv64: "))
            .expect("semantic fingerprint A");
        let fingerprint_b = projection_b
            .body
            .lines()
            .find(|line| line.starts_with("SemanticPlanFingerprintFnv64: "))
            .expect("semantic fingerprint B");
        assert_eq!(fingerprint_a, fingerprint_b);
        assert!(projection_a.body.contains("proof_chain_id=0:100-1:101-2:102-3:103"));
        assert!(projection_b
            .body
            .contains("proof_chain_id=1000:2100-1001:2101-1002:2102-1003:2103"));
    }

    #[test]
    fn hidden_chord_after_rest_is_preserved_as_hidden_technical_carrier() {
        let mut tracker = PreSerializationAtomicGroupTracker::default();
        let (rest_group, rest_kind) = tracker.assign(1, true, true, false, false);
        let (chord_group, chord_kind) = tracker.assign(1, false, false, true, true);

        assert_eq!(rest_kind, PreSerializationAtomicGroupKind::VisibleRest);
        assert_eq!(
            chord_kind,
            PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor
        );
        assert_ne!(rest_group, chord_group);
    }

    #[test]
    fn visible_chord_after_rest_is_separate_evidence_and_retains_rest_overlap() {
        let mut tracker = PreSerializationAtomicGroupTracker::default();
        let (visible_rest_group, visible_rest_kind) =
            tracker.assign(30, true, true, false, false);
        let (hidden_rest_group, hidden_rest_kind) =
            tracker.assign(30, false, true, false, false);
        let (chord_group, chord_kind) = tracker.assign(30, true, false, true, true);
        let (chord_group_2, chord_kind_2) = tracker.assign(30, true, false, true, true);

        assert_ne!(visible_rest_group, hidden_rest_group);
        assert_ne!(hidden_rest_group, chord_group);
        assert_eq!(chord_group, chord_group_2);
        assert_eq!(visible_rest_kind, PreSerializationAtomicGroupKind::VisibleRest);
        assert_eq!(hidden_rest_kind, PreSerializationAtomicGroupKind::HiddenRest);
        assert_eq!(
            chord_kind,
            PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor
        );
        assert_eq!(chord_kind, chord_kind_2);

        let occupancy_rows = vec![
            PreSerializationVisibleOccupancyRow {
                measure: 30,
                staff: 2,
                represented_onset: 288,
                is_rest: true,
                atomic_group_id: visible_rest_group,
                atomic_group_kind: visible_rest_kind,
                ..baseline_pre_serialization_visible_occupancy_row()
            },
            PreSerializationVisibleOccupancyRow {
                measure: 30,
                staff: 2,
                represented_onset: 288,
                visible: false,
                is_rest: true,
                atomic_group_id: hidden_rest_group,
                atomic_group_kind: hidden_rest_kind,
                ..baseline_pre_serialization_visible_occupancy_row()
            },
            PreSerializationVisibleOccupancyRow {
                measure: 30,
                staff: 2,
                represented_onset: 288,
                is_chord_member: true,
                pitch_midi: Some(66),
                source_event_id: Some(901),
                attack_id: Some(1901),
                atomic_group_id: chord_group,
                atomic_group_kind: chord_kind,
                ..baseline_pre_serialization_visible_occupancy_row()
            },
            PreSerializationVisibleOccupancyRow {
                measure: 30,
                staff: 2,
                represented_onset: 288,
                is_chord_member: true,
                pitch_midi: Some(63),
                source_event_id: Some(902),
                attack_id: Some(1902),
                atomic_group_id: chord_group_2,
                atomic_group_kind: chord_kind_2,
                ..baseline_pre_serialization_visible_occupancy_row()
            },
        ];
        let projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection(
                "<score-partwise version=\"4.0\"></score-partwise>",
                &[],
                &[],
                &occupancy_rows,
            );
        assert!(projection
            .body
            .contains("VisibleChordWithoutPitchedAnchorGroups: 1"));
        assert!(projection
            .body
            .contains("VisibleRestOrphanChordOverlapEvidencePairs: 1"));
        assert!(projection
            .body
            .contains("VISIBLE_CHORD_WITHOUT_PITCHED_ANCHOR"));
        assert!(projection
            .body
            .contains("VISIBLE_REST_ORPHAN_CHORD_OVERLAP"));
    }

    #[test]
    fn normal_visible_chord_reuses_visible_pitched_anchor_atomic_group() {
        let mut tracker = PreSerializationAtomicGroupTracker::default();
        let (anchor_group, anchor_kind) = tracker.assign(1, true, false, false, true);
        let (chord_group, chord_kind) = tracker.assign(1, true, false, true, true);

        assert_eq!(anchor_group, chord_group);
        assert_eq!(
            anchor_kind,
            PreSerializationAtomicGroupKind::VisiblePitched
        );
        assert_eq!(anchor_kind, chord_kind);
    }

    #[test]
    fn metric_notation_line_selection_is_independent_of_voice_labels_and_beams() {
        let (xml_with_beams, source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["71", "72", "73", "74"]);
        let xml_without_beams = xml_with_beams
            .replace("<beam number=\"1\">begin</beam>", "")
            .replace("<beam number=\"1\">continue</beam>", "")
            .replace("<beam number=\"1\">end</beam>", "");
        let projection = build_notation_voice_pre_serialization_constraint_closure_audit_projection(
            &xml_without_beams,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
        );
        assert!(projection
            .body
            .contains("MetricNotationLineSelectionVoiceLabels: excluded"));
        assert!(projection
            .body
            .contains("MetricNotationLineSelectionBeams: excluded"));
        assert!(projection
            .body
            .contains("MetricNotationLineEligibleAnalysisOnly: 1"));
        assert!(projection
            .body
            .contains("selection_voice_labels=ignored selection_beams=ignored"));
        assert_eq!(projection.metric_line_candidates().len(), 1);
    }

    #[test]
    fn metric_line_baseline_overlay_rebinds_only_candidate_sources_without_global_replan() {
        let (xml, source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        let evidence = source_rows
            .iter()
            .enumerate()
            .map(|(index, row)| crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: index.saturating_add(1),
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                measure: row.measure,
                raw_note_index: index,
                onset: row.semantic_onset,
                duration: 24,
                pitch: row.pitch_midi,
                staff: row.staff,
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            })
            .collect::<Vec<_>>();
        let (canonical, _) =
            crate::structural_write_authority::plan_canonical_final_chord_structure_once(
                &xml,
                &evidence,
            )
            .expect("canonical FinalChord plan");
        let frozen_attacks = source_rows
            .iter()
            .map(|row| crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                absolute_start: row.absolute_start,
                absolute_end: row.absolute_end,
                staff: row.staff,
                technical_lane: row.technical_lane,
            })
            .collect::<Vec<_>>();
        let baseline =
            crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
                &canonical,
                &frozen_attacks,
            )
            .expect("baseline voice channels");
        let mut projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection(
                &xml,
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        append_notation_voice_metric_line_baseline_overlay_audit(
            &mut projection,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
            &baseline,
        );
        assert!(projection
            .body
            .contains("MetricLineOverlayCandidates: 1"));
        assert!(projection
            .body
            .contains("MetricLineOverlayTargetRealized: 1"));
        assert!(projection
            .body
            .contains("MetricLineOverlayExternalBindingPreserved: 1"));
        assert!(projection.body.contains(
            "MetricLineOverlayBatchVerdict: BASELINE_OVERLAY_BATCH_TARGET_RELATIONS_REALIZED_EXTERNAL_BINDINGS_PRESERVED_NO_NEW_VISIBLE_CHANNEL_CONFLICT"
        ));
        assert!(projection.body.contains(
            "MetricLineOverlayVerdict: BEAM_FREE_METRIC_LINE_BASELINE_OVERLAY_COUNTERFACTUAL_GREEN"
        ));
    }

    #[test]
    fn upstream_lineage_preservation_accepts_uniform_pre_lane_voice_without_using_current_voice_labels() {
        let (xml, source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["71", "72", "73", "74"]);
        let evidence = source_rows
            .iter()
            .enumerate()
            .map(|(index, row)| crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: index.saturating_add(1),
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                measure: row.measure,
                raw_note_index: index,
                onset: row.semantic_onset,
                duration: 24,
                pitch: row.pitch_midi,
                staff: row.staff,
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            })
            .collect::<Vec<_>>();
        let (canonical, _) =
            crate::structural_write_authority::plan_canonical_final_chord_structure_once(
                &xml,
                &evidence,
            )
            .expect("canonical FinalChord plan");
        let frozen_attacks = source_rows
            .iter()
            .map(|row| crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                absolute_start: row.absolute_start,
                absolute_end: row.absolute_end,
                staff: row.staff,
                technical_lane: row.technical_lane,
            })
            .collect::<Vec<_>>();
        let baseline =
            crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
                &canonical,
                &frozen_attacks,
            )
            .expect("baseline voice channels");
        let mut projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection(
                &xml,
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        append_notation_voice_upstream_lineage_preservation_audit(
            &mut projection,
            &source_rows,
            &baseline,
        );
        assert_eq!(projection.metric_line_candidates().len(), 1);
        assert!(projection
            .body
            .contains("UpstreamVoiceUniformCandidates: 1"));
        assert!(projection
            .body
            .contains("UpstreamVoiceAmbiguousCandidates: 0"));
        assert!(projection
            .body
            .contains("UpstreamVoiceUniformCurrentSplitCandidates: 1"));
        assert!(projection
            .body
            .contains("upstream_voice_uniform=yes"));
        assert!(projection
            .body
            .contains("status=UPSTREAM_LINEAGE_PRESERVATION_PROVEN_ANALYSIS_ONLY"));
    }

    #[test]
    fn upstream_lineage_preservation_blocks_alternating_pre_lane_voice_hocket() {
        let (xml, mut source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["71", "72", "73", "74"]);
        for (row, voice) in source_rows
            .iter_mut()
            .zip(["1", "2", "1", "2"])
        {
            row.pre_lane_voice = voice.to_string();
        }
        let evidence = source_rows
            .iter()
            .enumerate()
            .map(|(index, row)| crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: index.saturating_add(1),
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                measure: row.measure,
                raw_note_index: index,
                onset: row.semantic_onset,
                duration: 24,
                pitch: row.pitch_midi,
                staff: row.staff,
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            })
            .collect::<Vec<_>>();
        let (canonical, _) =
            crate::structural_write_authority::plan_canonical_final_chord_structure_once(
                &xml,
                &evidence,
            )
            .expect("canonical FinalChord plan");
        let frozen_attacks = source_rows
            .iter()
            .map(|row| crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: row.source_event_id,
                attack_id: row.attack_id,
                absolute_start: row.absolute_start,
                absolute_end: row.absolute_end,
                staff: row.staff,
                technical_lane: row.technical_lane,
            })
            .collect::<Vec<_>>();
        let baseline =
            crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
                &canonical,
                &frozen_attacks,
            )
            .expect("baseline voice channels");
        let mut projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection(
                &xml,
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        assert_eq!(projection.metric_line_candidates().len(), 1);
        append_notation_voice_upstream_lineage_preservation_audit(
            &mut projection,
            &source_rows,
            &baseline,
        );
        assert!(projection
            .body
            .contains("UpstreamVoiceUniformCandidates: 0"));
        assert!(projection
            .body
            .contains("UpstreamVoiceAmbiguousCandidates: 1"));
        assert!(projection
            .body
            .contains("upstream_voice_uniform=no"));
        assert!(projection
            .body
            .contains("status=BLOCKED_UPSTREAM_VOICE_AMBIGUITY"));
    }

    #[test]
    fn materialization_safety_conjunction_proves_uniform_split_metric_line_without_write() {
        let (xml, source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        let baseline = vec![
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 0,
                channel: 1,
                role_label: "upper_primary_run".to_string(),
            },
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 1,
                channel: 2,
                role_label: "upper_secondary_run".to_string(),
            },
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 2,
                channel: 1,
                role_label: "upper_primary_run".to_string(),
            },
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 3,
                channel: 2,
                role_label: "upper_secondary_run".to_string(),
            },
        ];
        let mut projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection(
                &xml,
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        append_notation_voice_materialization_safety_conjunction_audit(
            &mut projection,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
            &baseline,
        );
        assert!(projection.body.contains("MaterializationSafetyCandidates: 1"));
        assert!(projection
            .body
            .contains("MaterializationSafetyCurrentSplitCandidates: 1"));
        assert!(projection
            .body
            .contains("MaterializationSafetyProvenAnalysisOnly: 1"));
        assert!(projection.body.contains(
            "status=MATERIALIZATION_SAFETY_CONJUNCTION_PROVEN_ANALYSIS_ONLY"
        ));
        assert!(projection.body.contains("MaterializationSafetyAuthority: none"));
        assert!(projection.body.contains("MaterializationSafetyMusicMutation: nein"));
        assert_eq!(
            projection
                .materialization_safety_primary_target_evidence_rows()
                .len(),
            1
        );
        assert_eq!(
            projection.materialization_safety_primary_target_evidence_rows()[0]
                .target_primary_source_event_id,
            0
        );
        assert_eq!(
            projection.materialization_safety_primary_target_evidence_rows()[0]
                .target_primary_attack_id,
            100
        );
        assert_eq!(
            projection.materialization_safety_primary_target_evidence_rows()[0].target_channel,
            1
        );
    }

    #[test]
    fn materialization_safety_conjunction_blocks_nonuniform_upstream_lineage() {
        let (xml, mut source_rows, occurrence_rows, occupancy_rows) =
            clean_fixture(["1", "3", "1", "3"]);
        source_rows[1].pre_lane_voice = "2".to_string();
        let baseline = vec![
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 0,
                channel: 1,
                role_label: "upper_primary_run".to_string(),
            },
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 1,
                channel: 2,
                role_label: "upper_secondary_run".to_string(),
            },
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 2,
                channel: 1,
                role_label: "upper_primary_run".to_string(),
            },
            MaterializationSafetyBaselineBindingRow {
                source_event_id: 3,
                channel: 2,
                role_label: "upper_secondary_run".to_string(),
            },
        ];
        let mut projection =
            build_notation_voice_pre_serialization_constraint_closure_audit_projection(
                &xml,
                &source_rows,
                &occurrence_rows,
                &occupancy_rows,
            );
        append_notation_voice_materialization_safety_conjunction_audit(
            &mut projection,
            &source_rows,
            &occurrence_rows,
            &occupancy_rows,
            &baseline,
        );
        assert!(projection
            .body
            .contains("MaterializationSafetyProvenAnalysisOnly: 0"));
        assert!(projection
            .body
            .contains("MaterializationSafetyBlockedUpstreamLineage: 1"));
        assert!(projection.body.contains("upstream_pre_lane_voice_not_uniform"));
        assert!(projection
            .body
            .contains("status=BLOCKED_MATERIALIZATION_SAFETY_CONJUNCTION"));
    }

    #[test]
    fn visible_rhythmic_occupancy_treats_visible_chord_as_one_atomic_pitched_occupancy() {
        let xml = "<score-partwise version=\"4.0\"><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions></attributes></measure></part></score-partwise>";
        let occupancy_rows = vec![
            PreSerializationVisibleOccupancyRow {
                pitch_midi: Some(60),
                source_event_id: Some(1),
                attack_id: Some(101),
                ..baseline_pre_serialization_visible_occupancy_row()
            },
            PreSerializationVisibleOccupancyRow {
                is_chord_member: true,
                pitch_midi: Some(64),
                source_event_id: Some(2),
                attack_id: Some(102),
                ..baseline_pre_serialization_visible_occupancy_row()
            },
        ];
        let audit = build_visible_rhythmic_occupancy_closure_audit(xml, &occupancy_rows, true);
        assert!(audit.metric_parse_ok);
        assert_eq!(audit.visible_atomic_groups, 1);
        assert_eq!(audit.pitched_note_atoms, 0);
        assert_eq!(audit.pitched_chord_atoms, 1);
        assert_eq!(audit.incomplete_atomic_groups, 0);
        assert_eq!(audit.independent_pitched_overlap_pairs, 0);
        assert!(audit.rows.iter().any(|row| {
            row.contains("kind=PITCHED_CHORD") && row.contains("pitches=[60,64]")
        }));
    }

    #[test]
    fn visible_rhythmic_occupancy_separates_duplicate_rest_and_rest_pitched_conflicts() {
        let xml = "<score-partwise version=\"4.0\"><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions></attributes></measure></part></score-partwise>";
        let occupancy_rows = vec![
            PreSerializationVisibleOccupancyRow {
                represented_duration: 48,
                is_rest: true,
                atomic_group_kind: PreSerializationAtomicGroupKind::VisibleRest,
                ..baseline_pre_serialization_visible_occupancy_row()
            },
            PreSerializationVisibleOccupancyRow {
                represented_duration: 48,
                is_rest: true,
                atomic_group_id: 2,
                atomic_group_kind: PreSerializationAtomicGroupKind::VisibleRest,
                ..baseline_pre_serialization_visible_occupancy_row()
            },
            PreSerializationVisibleOccupancyRow {
                represented_onset: 24,
                pitch_midi: Some(67),
                source_event_id: Some(3),
                attack_id: Some(103),
                atomic_group_id: 3,
                ..baseline_pre_serialization_visible_occupancy_row()
            },
        ];
        let audit = build_visible_rhythmic_occupancy_closure_audit(xml, &occupancy_rows, true);
        assert!(audit.metric_parse_ok);
        assert_eq!(audit.rest_atoms, 2);
        assert_eq!(audit.pitched_note_atoms, 1);
        assert_eq!(audit.duplicate_rest_pairs, 1);
        assert_eq!(audit.rest_pitched_overlap_pairs, 2);
        assert_eq!(audit.independent_pitched_overlap_pairs, 0);
        assert!(audit.rows.iter().any(|row| row.contains("kind=DUPLICATE_REST_OCCUPANCY")));
        assert!(audit.rows.iter().any(|row| row.contains("kind=REST_PITCHED_OVERLAP")));
    }

}
