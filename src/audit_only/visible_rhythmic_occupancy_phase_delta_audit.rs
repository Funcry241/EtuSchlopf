use roxmltree::Document;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

use super::notation_voice_causal_lane_pressure_audit::NotationVoiceCanonicalOccurrenceIdentityRow;
use super::notation_voice_pre_serialization_constraint_closure_audit::{
    build_visible_rhythmic_occupancy_phase_snapshot, CanonicalPrimaryBeamOwnerReceipt,
    MaterializationSafetyPrimaryTargetEvidenceRow,
    NotationVoicePreSerializationConstraintClosureAuditProjection,
    PreSerializationAtomicGroupKind, PreSerializationClosedSameLineCandidate,
    PreSerializationClosedSameLineMemberBinding,
    PreSerializationMetricNotationLineCandidate, PreSerializationOccurrenceRow,
    PreSerializationVisibleOccupancyRow, VisibleRhythmicOccupancyAtomKind,
    VisibleRhythmicOccupancyPhaseAtom, VisibleRhythmicOccupancyPhaseSnapshot,
};

pub(crate) const SECTION: &str = "VISIBLE-RHYTHMIC-OCCUPANCY-PHASE-DELTA-AUDIT";
pub(crate) const SECTION_RULE: &str = "---------------------------------------------";

type AtomSignature = (usize, u8, i64, i64, VisibleRhythmicOccupancyAtomKind, Vec<i32>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderMetricEvent {
    measure: usize,
    staff: u8,
    onset: i64,
    duration: i64,
    pitch_midi: Option<i32>,
    visible: bool,
    is_rest: bool,
    is_chord: bool,
    voice: String,
    beam1: Option<String>,
    pitched_ordinal: Option<usize>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct VisibleRhythmicOccupancyPhaseDeltaAudit {
    render_parse_ok: bool,
    s1_metric_parse_ok: bool,
    s0_visible_atoms: usize,
    s1_visible_atoms: usize,
    s0_incomplete_groups: usize,
    s1_incomplete_groups: usize,
    shared_pitched_occupancy_signatures: usize,
    shared_duplicate_rest_conflict_geometries: usize,
    shared_rest_pitched_conflict_geometries: usize,
    s1_new_duplicate_rest_conflict_geometries: usize,
    s1_new_rest_pitched_conflict_geometries_at_s0_hidden_rest_signature: usize,
    s1_unbound_rest_pitched_conflicts: usize,
    s1_visible_at_s0_hidden_signatures: usize,
    blocked_incomplete_chord_bindings: usize,
    metric_lines_total: usize,
    metric_lines_exact_render_binding: usize,
    metric_lines_unresolved_render_binding: usize,
    metric_lines_cross_voice: usize,
    metric_lines_beamless: usize,
    metric_lines_multiple_beam_fragments: usize,
    rows: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct FrozenPrimaryBeamSerializationVoicePartitionClassification {
    render_parse_ok: bool,
    exact_closed_same_line_metric_bindings: usize,
    exact_closed_same_line_render_structural_bindings: usize,
    persisted_render_cross_voice_structural_observations: usize,
    persisted_render_cross_voice_single_beam_fragment_observations: usize,
    persisted_render_cross_voice_multiple_beam_fragment_observations: usize,
    persisted_render_occurrence_identity_legacy_exact_bindings: usize,
    persisted_render_occurrence_identity_source_attack_bridge_bindings: usize,
    persisted_render_occurrence_identity_local_bijection_bindings: usize,
    persisted_render_occurrence_identity_exact_bindings: usize,
    persisted_render_occurrence_identity_unresolved: usize,
    class_members_proven: usize,
    class_members_unresolved: usize,
    proven_metric_line_ids: BTreeSet<String>,
    rows: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct PrimaryBeamSerializationVoicePartitionRepairScopeAudit {
    class_members_proven: usize,
    class_members_unresolved: usize,
    materialization_safety_proven_candidates: usize,
    repair_scope_eligible: usize,
    blocked_materialization_safety: usize,
    eligible_metric_line_ids: BTreeSet<String>,
    rows: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAudit {
    render_parse_ok: bool,
    eligible_metric_lines: usize,
    metric_candidate_bindings: usize,
    target_evidence_bindings: usize,
    target_evidence_unresolved: usize,
    member_occurrences_bound: usize,
    member_occurrences_unresolved: usize,
    existing_target_voice_bindings: usize,
    proposed_voice_scalar_changes: usize,
    already_target_voice_scalars: usize,
    source_overlap_conflicts: usize,
    physical_edit_footprint_conflicts: usize,
    replay_ready_metric_lines: usize,
    rows: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct RendererVisiblePrimaryBeamGeometryAudit {
    pdf_readback_ok: bool,
    measure_count: usize,
    cross_voice_observations: usize,
    occurrence_identity_bound: usize,
    occurrence_identity_unresolved: usize,
    renderer_geometry_bound: usize,
    continuity_preserved: usize,
    continuity_fragmented: usize,
    geometry_unavailable: usize,
    bound_measure_staff_cells: usize,
    unresolved_measure_staff_cells: usize,
    visible_fragments_by_metric_line_id: BTreeMap<String, usize>,
    rows: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct CanonicalPrimaryBeamAuthorityBindingAudit {
    render_parse_ok: bool,
    cross_voice_observations: usize,
    occurrence_identity_bound: usize,
    occurrence_identity_unresolved: usize,
    pre_final_chord_exact_encoding_witnesses: usize,
    pre_final_chord_encoding_witness_missing: usize,
    typed_owner_receipts_available: usize,
    owner_receipt_cardinality_unresolved: usize,
    owner_receipt_event_count_mismatch: usize,
    owner_receipt_primary_state_mismatch: usize,
    canonical_authority_bindings_proven: usize,
    canonical_authority_bindings_unresolved: usize,
    proven_metric_line_ids: BTreeSet<String>,
    rows: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct CanonicalPrimaryBeamOwnerReceiptTransportAudit {
    render_parse_ok: bool,
    cross_voice_observations: usize,
    occurrence_identity_bound: usize,
    occurrence_identity_unresolved: usize,
    pre_final_chord_exact_encoding_witnesses: usize,
    pre_final_chord_encoding_witness_missing: usize,
    typed_current_canonical_owner_receipts: usize,
    receipt_event_count_mismatch: usize,
    receipt_transport_proven: usize,
    receipt_transport_unresolved: usize,
    rows: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BeamOccurrenceIdentityClosureProof {
    LegacyExactLocator,
    SourceAttackBridge,
    LocalSourceAttackBijection,
}

impl BeamOccurrenceIdentityClosureProof {
    const fn label(self) -> &'static str {
        match self {
            Self::LegacyExactLocator => "legacy_exact_measure_pitched_ordinal_binding",
            Self::SourceAttackBridge => "exact_source_attack_bridge_to_canonical_s1",
            Self::LocalSourceAttackBijection => {
                "unique_local_source_attack_geometry_bijection_to_canonical_s1"
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct BeamOccurrenceCrossPhaseIdentityAudit {
    cross_voice_observations: usize,
    legacy_exact_locator_bindings: usize,
    s0_source_attack_missing: usize,
    s0_source_attack_multiplicity: usize,
    canonical_source_attack_missing: usize,
    canonical_source_attack_multiplicity: usize,
    source_attack_bridge_exact_same_locator: usize,
    source_attack_bridge_exact_locator_shift: usize,
    canonical_to_s1_locator_unresolved: usize,
    canonical_to_s1_geometry_mismatch: usize,
    unresolved_after_source_attack_bridge: usize,
    rows: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct BeamOccurrenceSourceAttackMultiplicityLocalBijectionAudit {
    blocked_cross_voice_observations: usize,
    local_identity_groups: usize,
    proven_unique_local_bijections: usize,
    blocked_cardinality_mismatch: usize,
    blocked_member_s0_locator: usize,
    blocked_canonical_s1_locator: usize,
    blocked_no_perfect_bijection: usize,
    blocked_ambiguous_perfect_bijection: usize,
    blocked_member_not_in_selected_s1: usize,
    observations_proven_after_local_bijection: usize,
    observations_unresolved_after_local_bijection: usize,
    rows: Vec<String>,
}

fn child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.has_tag_name(name))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn child_duration(node: roxmltree::Node<'_, '_>) -> i64 {
    child_text(node, "duration")
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
}

fn primary_beam_state(node: roxmltree::Node<'_, '_>) -> Option<String> {
    node.children()
        .filter(|child| child.is_element() && child.has_tag_name("beam"))
        .find(|beam| beam.attribute("number").unwrap_or("1") == "1")
        .and_then(|beam| beam.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
}

fn pitch_midi(node: roxmltree::Node<'_, '_>) -> Option<i32> {
    let pitch = node
        .children()
        .find(|child| child.is_element() && child.has_tag_name("pitch"))?;
    let step = child_text(pitch, "step")?;
    let octave = child_text(pitch, "octave")?.parse::<i32>().ok()?;
    let alter = child_text(pitch, "alter")
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(0);
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
    Some((octave + 1).saturating_mul(12).saturating_add(semitone + alter))
}

fn parse_render_phase(
    xml: &str,
) -> Option<(Vec<PreSerializationVisibleOccupancyRow>, Vec<RenderMetricEvent>)> {
    let document = Document::parse(xml).ok()?;
    let mut occupancy_rows = Vec::new();
    let mut metric_events = Vec::new();
    let mut next_group_id = 1usize;

    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("part"))
    {
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("measure"))
            .enumerate()
        {
            let measure_number = measure
                .attribute("number")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(measure_index.saturating_add(1));
            let mut cursor = 0_i64;
            let mut last_note_onset = 0_i64;
            let mut current_anchor_group_id = None::<usize>;
            let mut current_anchor_kind = None::<PreSerializationAtomicGroupKind>;
            let mut current_orphan_chord_group_id = None::<usize>;
            let mut pitched_ordinal = 0usize;

            for node in measure.children().filter(|node| node.is_element()) {
                if node.has_tag_name("backup") {
                    cursor = cursor.saturating_sub(child_duration(node).max(0));
                    continue;
                }
                if node.has_tag_name("forward") {
                    cursor = cursor.saturating_add(child_duration(node).max(0));
                    continue;
                }
                if !node.has_tag_name("note") {
                    continue;
                }

                let duration = child_duration(node).max(0);
                let is_chord = node
                    .children()
                    .any(|child| child.is_element() && child.has_tag_name("chord"));
                let onset = if is_chord { last_note_onset } else { cursor };
                if !is_chord {
                    last_note_onset = onset;
                }
                let is_rest = node
                    .children()
                    .any(|child| child.is_element() && child.has_tag_name("rest"));
                let visible = node.attribute("print-object") != Some("no");
                let pitch = pitch_midi(node);
                let event_pitched_ordinal = pitch.map(|_| {
                    let ordinal = pitched_ordinal;
                    pitched_ordinal = pitched_ordinal.saturating_add(1);
                    ordinal
                });
                let staff = child_text(node, "staff")
                    .and_then(|value| value.parse::<u8>().ok())
                    .unwrap_or(1);
                let voice = child_text(node, "voice").unwrap_or_else(|| "1".to_string());

                let (atomic_group_id, atomic_group_kind) = if !is_chord {
                    let group = next_group_id;
                    next_group_id = next_group_id.saturating_add(1);
                    let kind = if is_rest {
                        if visible {
                            PreSerializationAtomicGroupKind::VisibleRest
                        } else {
                            PreSerializationAtomicGroupKind::HiddenRest
                        }
                    } else if pitch.is_some() {
                        if visible {
                            PreSerializationAtomicGroupKind::VisiblePitched
                        } else {
                            PreSerializationAtomicGroupKind::HiddenPitched
                        }
                    } else {
                        PreSerializationAtomicGroupKind::Other
                    };
                    current_anchor_group_id = Some(group);
                    current_anchor_kind = Some(kind);
                    current_orphan_chord_group_id = None;
                    (group, kind)
                } else if let (Some(group), Some(kind)) =
                    (current_anchor_group_id, current_anchor_kind)
                {
                    if matches!(
                        kind,
                        PreSerializationAtomicGroupKind::VisiblePitched
                            | PreSerializationAtomicGroupKind::HiddenPitched
                    ) {
                        (group, kind)
                    } else {
                        let group = match current_orphan_chord_group_id {
                            Some(group) => group,
                            None => {
                                let group = next_group_id;
                                next_group_id = next_group_id.saturating_add(1);
                                current_orphan_chord_group_id = Some(group);
                                group
                            }
                        };
                        (
                            group,
                            PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor,
                        )
                    }
                } else {
                    let group = match current_orphan_chord_group_id {
                        Some(group) => group,
                        None => {
                            let group = next_group_id;
                            next_group_id = next_group_id.saturating_add(1);
                            current_orphan_chord_group_id = Some(group);
                            group
                        }
                    };
                    (
                        group,
                        PreSerializationAtomicGroupKind::ChordWithoutPitchedAnchor,
                    )
                };

                occupancy_rows.push(PreSerializationVisibleOccupancyRow {
                    measure: measure_number,
                    staff,
                    represented_onset: onset,
                    represented_duration: duration,
                    visible,
                    is_rest,
                    is_chord_member: is_chord,
                    pitch_midi: pitch,
                    source_event_id: None,
                    attack_id: None,
                    atomic_group_id,
                    atomic_group_kind,
                });
                metric_events.push(RenderMetricEvent {
                    measure: measure_number,
                    staff,
                    onset,
                    duration,
                    pitch_midi: pitch,
                    visible,
                    is_rest,
                    is_chord,
                    voice,
                    beam1: primary_beam_state(node),
                    pitched_ordinal: event_pitched_ordinal,
                });

                if !is_chord {
                    cursor = cursor.saturating_add(duration);
                }
            }
        }
    }

    Some((occupancy_rows, metric_events))
}

fn atom_signature(atom: &VisibleRhythmicOccupancyPhaseAtom) -> AtomSignature {
    (
        atom.measure,
        atom.staff,
        atom.onset,
        atom.duration,
        atom.kind,
        atom.pitches.clone(),
    )
}

fn beam_fragment_count(events: &[&RenderMetricEvent]) -> usize {
    let mut fragments = 0usize;
    let mut active = false;
    for event in events {
        match event.beam1.as_deref() {
            Some("begin") => {
                fragments = fragments.saturating_add(1);
                active = true;
            }
            Some("continue") | Some("forward hook") | Some("backward hook") => {
                if !active {
                    fragments = fragments.saturating_add(1);
                    active = true;
                }
            }
            Some("end") => {
                if !active {
                    fragments = fragments.saturating_add(1);
                }
                active = false;
            }
            Some(_) => {
                if !active {
                    fragments = fragments.saturating_add(1);
                    active = true;
                }
            }
            None => active = false,
        }
    }
    fragments
}

fn closed_same_line_candidate_matches_metric_line(
    closed: &PreSerializationClosedSameLineCandidate,
    metric: &PreSerializationMetricNotationLineCandidate,
) -> bool {
    closed.staff == metric.staff
        && closed.source_event_ids == metric.source_event_ids
        && closed.members == metric.members
}

fn frozen_class_render_structural_binding<'a>(
    metric: &PreSerializationMetricNotationLineCandidate,
    render_events: &'a [RenderMetricEvent],
) -> Option<Vec<&'a RenderMetricEvent>> {
    let mut expected = metric
        .members
        .iter()
        .map(|member| (member.represented_onset, member.represented_duration))
        .collect::<Vec<_>>();
    expected.sort();
    let mut selected = render_events
        .iter()
        .filter(|event| {
            event.measure == metric.measure
                && event.staff == metric.staff
                && event.visible
                && !event.is_rest
                && !event.is_chord
                && event.duration > 0
                && event.onset >= metric.interval_start
                && event.onset.saturating_add(event.duration) <= metric.interval_end
        })
        .collect::<Vec<_>>();
    selected.sort_by_key(|event| (event.onset, event.duration, event.voice.clone()));
    let observed = selected
        .iter()
        .map(|event| (event.onset, event.duration))
        .collect::<Vec<_>>();
    (observed == expected).then_some(selected)
}

fn frozen_class_persisted_occurrence_binding(
    metric: &PreSerializationMetricNotationLineCandidate,
    selected: &[&RenderMetricEvent],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
) -> bool {
    if selected.len() != metric.members.len() || selected.is_empty() {
        return false;
    }
    let mut claimed_members = BTreeSet::<(usize, usize)>::new();
    for event in selected {
        let Some(pitched_ordinal) = event.pitched_ordinal else {
            return false;
        };
        let mut matching_members = metric.members.iter().filter(|member| {
            member.measure == event.measure
                && member.staff == event.staff
                && member.represented_onset == event.onset
                && member.represented_duration == event.duration
                && member.pitched_ordinal == pitched_ordinal
        });
        let Some(member) = matching_members.next() else {
            return false;
        };
        if matching_members.next().is_some() {
            return false;
        }
        let mut matching_occurrences = occurrence_rows.iter().filter(|row| {
            row.measure == event.measure && row.pitched_ordinal == pitched_ordinal
        });
        let Some(occurrence) = matching_occurrences.next() else {
            return false;
        };
        if matching_occurrences.next().is_some() {
            return false;
        }
        if occurrence.source_event_id != member.source_event_id
            || occurrence.attack_id != member.attack_id
        {
            return false;
        }
        if !claimed_members.insert((member.source_event_id, member.attack_id)) {
            return false;
        }
    }
    claimed_members.len() == metric.members.len()
}

fn build_beam_occurrence_cross_phase_identity_audit(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    canonical_occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    render_xml: &str,
) -> BeamOccurrenceCrossPhaseIdentityAudit {
    let Some((_, render_events)) = parse_render_phase(render_xml) else {
        return BeamOccurrenceCrossPhaseIdentityAudit::default();
    };
    let mut audit = BeamOccurrenceCrossPhaseIdentityAudit::default();
    let mut s0_by_identity = BTreeMap::<(usize, usize), Vec<&PreSerializationOccurrenceRow>>::new();
    for row in s0_occurrence_rows {
        s0_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }
    let mut canonical_by_identity = BTreeMap::<
        (usize, usize),
        Vec<&NotationVoiceCanonicalOccurrenceIdentityRow>,
    >::new();
    for row in canonical_occurrence_rows {
        canonical_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }

    for metric in metric_candidates {
        let closed_matches = closed_candidates
            .iter()
            .filter(|closed| closed_same_line_candidate_matches_metric_line(closed, metric))
            .count();
        if closed_matches != 1 {
            continue;
        }
        let Some(selected) = frozen_class_render_structural_binding(metric, &render_events) else {
            continue;
        };
        let voices = selected
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        if voices.len() <= 1 {
            continue;
        }
        audit.cross_voice_observations = audit.cross_voice_observations.saturating_add(1);

        if frozen_class_persisted_occurrence_binding(metric, &selected, canonical_occurrence_rows) {
            audit.legacy_exact_locator_bindings =
                audit.legacy_exact_locator_bindings.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} class=LEGACY_EXACT_MEASURE_PITCHED_ORDINAL_BINDING source_attack_bridge=not_needed presentation_voices={}",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                voices.into_iter().collect::<Vec<_>>().join(","),
            ));
            continue;
        }

        let mut s0_missing = false;
        let mut s0_multiplicity = false;
        let mut canonical_missing = false;
        let mut canonical_multiplicity = false;
        let mut locator_shift = false;
        let mut s1_locator_unresolved = false;
        let mut s1_geometry_mismatch = false;
        let mut member_rows = Vec::<String>::new();

        for member in &metric.members {
            let identity = (member.source_event_id, member.attack_id);
            let s0_matches = match s0_by_identity.get(&identity) {
                Some(rows) => rows.as_slice(),
                None => &[],
            };
            let canonical_matches = match canonical_by_identity.get(&identity) {
                Some(rows) => rows.as_slice(),
                None => &[],
            };
            if s0_matches.is_empty() {
                s0_missing = true;
            } else if s0_matches.len() > 1 {
                s0_multiplicity = true;
            }
            if canonical_matches.is_empty() {
                canonical_missing = true;
            } else if canonical_matches.len() > 1 {
                canonical_multiplicity = true;
            }

            if s0_matches.len() != 1 || canonical_matches.len() != 1 {
                member_rows.push(format!(
                    "{}:{}:s0_count={}:canonical_count={}",
                    member.source_event_id,
                    member.attack_id,
                    s0_matches.len(),
                    canonical_matches.len(),
                ));
                continue;
            }

            let s0 = s0_matches[0];
            let canonical = canonical_matches[0];
            if s0.measure != canonical.measure || s0.pitched_ordinal != canonical.pitched_ordinal {
                locator_shift = true;
            }
            let matching_s1 = selected
                .iter()
                .filter(|event| {
                    event.measure == canonical.measure
                        && event.pitched_ordinal == Some(canonical.pitched_ordinal)
                })
                .copied()
                .collect::<Vec<_>>();
            if matching_s1.len() != 1 {
                s1_locator_unresolved = true;
                member_rows.push(format!(
                    "{}:{}:s0={}:{}:canonical={}:{}:kind={}:fragment={}:s1_count={}",
                    member.source_event_id,
                    member.attack_id,
                    s0.measure,
                    s0.pitched_ordinal,
                    canonical.measure,
                    canonical.pitched_ordinal,
                    canonical.materialization_kind_rank,
                    canonical.fragment_ordinal,
                    matching_s1.len(),
                ));
                continue;
            }
            let s1 = matching_s1[0];
            let geometry_exact = s1.staff == s0.staff
                && s1.onset == s0.represented_onset
                && s1.duration == s0.represented_duration
                && s1.pitch_midi == Some(s0.pitch_midi);
            if !geometry_exact {
                s1_geometry_mismatch = true;
            }
            member_rows.push(format!(
                "{}:{}:s0={}:{}:canonical={}:{}:kind={}:fragment={}:s1_geometry={}",
                member.source_event_id,
                member.attack_id,
                s0.measure,
                s0.pitched_ordinal,
                canonical.measure,
                canonical.pitched_ordinal,
                canonical.materialization_kind_rank,
                canonical.fragment_ordinal,
                if geometry_exact { "exact" } else { "mismatch" },
            ));
        }

        let class = if s0_missing {
            audit.s0_source_attack_missing = audit.s0_source_attack_missing.saturating_add(1);
            audit.unresolved_after_source_attack_bridge =
                audit.unresolved_after_source_attack_bridge.saturating_add(1);
            "BLOCKED_S0_SOURCE_ATTACK_MISSING"
        } else if s0_multiplicity {
            audit.s0_source_attack_multiplicity =
                audit.s0_source_attack_multiplicity.saturating_add(1);
            audit.unresolved_after_source_attack_bridge =
                audit.unresolved_after_source_attack_bridge.saturating_add(1);
            "BLOCKED_S0_SOURCE_ATTACK_MULTIPLICITY"
        } else if canonical_missing {
            audit.canonical_source_attack_missing =
                audit.canonical_source_attack_missing.saturating_add(1);
            audit.unresolved_after_source_attack_bridge =
                audit.unresolved_after_source_attack_bridge.saturating_add(1);
            "BLOCKED_CANONICAL_SOURCE_ATTACK_MISSING"
        } else if canonical_multiplicity {
            audit.canonical_source_attack_multiplicity =
                audit.canonical_source_attack_multiplicity.saturating_add(1);
            audit.unresolved_after_source_attack_bridge =
                audit.unresolved_after_source_attack_bridge.saturating_add(1);
            "BLOCKED_CANONICAL_SOURCE_ATTACK_MULTIPLICITY"
        } else if s1_locator_unresolved {
            audit.canonical_to_s1_locator_unresolved =
                audit.canonical_to_s1_locator_unresolved.saturating_add(1);
            audit.unresolved_after_source_attack_bridge =
                audit.unresolved_after_source_attack_bridge.saturating_add(1);
            "BLOCKED_CANONICAL_TO_S1_LOCATOR"
        } else if s1_geometry_mismatch {
            audit.canonical_to_s1_geometry_mismatch =
                audit.canonical_to_s1_geometry_mismatch.saturating_add(1);
            audit.unresolved_after_source_attack_bridge =
                audit.unresolved_after_source_attack_bridge.saturating_add(1);
            "BLOCKED_CANONICAL_TO_S1_GEOMETRY_MISMATCH"
        } else if locator_shift {
            audit.source_attack_bridge_exact_locator_shift = audit
                .source_attack_bridge_exact_locator_shift
                .saturating_add(1);
            "PROVEN_SOURCE_ATTACK_BRIDGE_WITH_LOCATOR_SHIFT"
        } else {
            audit.source_attack_bridge_exact_same_locator = audit
                .source_attack_bridge_exact_same_locator
                .saturating_add(1);
            "PROVEN_SOURCE_ATTACK_BRIDGE_SAME_LOCATOR"
        };

        audit.rows.push(format!(
            "metric_line_id={} measure={} staff={} interval={}-{} class={} legacy_binding=unresolved presentation_voices={} members=[{}]",
            metric.metric_line_id,
            metric.measure,
            metric.staff,
            metric.interval_start,
            metric.interval_end,
            class,
            voices.into_iter().collect::<Vec<_>>().join(","),
            member_rows.join("|"),
        ));
    }
    audit.rows.sort();
    audit.rows.dedup();
    audit
}

fn occurrence_geometry_matches_render_event(
    s0: &PreSerializationOccurrenceRow,
    s1: &RenderMetricEvent,
) -> bool {
    s1.staff == s0.staff
        && s1.onset == s0.represented_onset
        && s1.duration == s0.represented_duration
        && s1.pitch_midi == Some(s0.pitch_midi)
}

fn count_local_perfect_bijections(edges: &[Vec<usize>]) -> (usize, Option<Vec<usize>>) {
    fn visit(
        row: usize,
        edges: &[Vec<usize>],
        used: &mut [bool],
        assignment: &mut [usize],
        count: &mut usize,
        first: &mut Option<Vec<usize>>,
    ) {
        if *count >= 2 {
            return;
        }
        if row == edges.len() {
            *count = count.saturating_add(1);
            if first.is_none() {
                *first = Some(assignment.to_vec());
            }
            return;
        }
        for &column in &edges[row] {
            if column >= used.len() || used[column] {
                continue;
            }
            used[column] = true;
            assignment[row] = column;
            visit(row + 1, edges, used, assignment, count, first);
            used[column] = false;
            if *count >= 2 {
                return;
            }
        }
    }

    if edges.is_empty() {
        return (0, None);
    }
    let columns = edges.len();
    if edges.iter().any(|row| row.is_empty() || row.iter().any(|column| *column >= columns)) {
        return (0, None);
    }
    let mut used = vec![false; columns];
    let mut assignment = vec![usize::MAX; edges.len()];
    let mut count = 0usize;
    let mut first = None;
    visit(
        0,
        edges,
        &mut used,
        &mut assignment,
        &mut count,
        &mut first,
    );
    let unique_assignment = if count == 1 { first } else { None };
    (count, unique_assignment)
}

fn beam_occurrence_identity_closure_binding(
    metric: &PreSerializationMetricNotationLineCandidate,
    selected: &[&RenderMetricEvent],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    canonical_occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    render_events: &[RenderMetricEvent],
) -> Option<BeamOccurrenceIdentityClosureProof> {
    if frozen_class_persisted_occurrence_binding(metric, selected, canonical_occurrence_rows) {
        return Some(BeamOccurrenceIdentityClosureProof::LegacyExactLocator);
    }

    let mut used_source_attack_bridge = false;
    let mut used_local_bijection = false;
    let mut processed_repeated_identities = BTreeSet::<(usize, usize)>::new();

    for member in &metric.members {
        let identity = (member.source_event_id, member.attack_id);
        let s0_matches = s0_occurrence_rows
            .iter()
            .filter(|row| (row.source_event_id, row.attack_id) == identity)
            .collect::<Vec<_>>();
        let canonical_matches = canonical_occurrence_rows
            .iter()
            .filter(|row| (row.source_event_id, row.attack_id) == identity)
            .collect::<Vec<_>>();

        if s0_matches.len() == 1 && canonical_matches.len() == 1 {
            let s0 = s0_matches[0];
            if s0.measure != member.measure
                || s0.pitched_ordinal != member.pitched_ordinal
                || s0.staff != member.staff
                || s0.represented_onset != member.represented_onset
                || s0.represented_duration != member.represented_duration
            {
                return None;
            }
            let canonical = canonical_matches[0];
            let matching_selected = selected
                .iter()
                .filter(|event| {
                    event.measure == canonical.measure
                        && event.pitched_ordinal == Some(canonical.pitched_ordinal)
                })
                .copied()
                .collect::<Vec<_>>();
            if matching_selected.len() != 1
                || !occurrence_geometry_matches_render_event(s0, matching_selected[0])
            {
                return None;
            }
            used_source_attack_bridge = true;
            continue;
        }

        if s0_matches.len() <= 1 || s0_matches.len() != canonical_matches.len() {
            return None;
        }
        if !processed_repeated_identities.insert(identity) {
            return None;
        }
        let metric_members = metric
            .members
            .iter()
            .filter(|candidate| {
                candidate.source_event_id == identity.0 && candidate.attack_id == identity.1
            })
            .collect::<Vec<_>>();
        if metric_members.len() != 1 {
            return None;
        }
        let metric_member = metric_members[0];
        let member_s0_indices = s0_matches
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                row.measure == metric_member.measure
                    && row.pitched_ordinal == metric_member.pitched_ordinal
                    && row.staff == metric_member.staff
                    && row.represented_onset == metric_member.represented_onset
                    && row.represented_duration == metric_member.represented_duration
            })
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        if member_s0_indices.len() != 1 {
            return None;
        }

        let mut canonical_s1 = Vec::<&RenderMetricEvent>::new();
        for canonical in &canonical_matches {
            let matches = render_events
                .iter()
                .filter(|event| {
                    event.measure == canonical.measure
                        && event.pitched_ordinal == Some(canonical.pitched_ordinal)
                })
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return None;
            }
            canonical_s1.push(matches[0]);
        }
        let edges = s0_matches
            .iter()
            .map(|s0| {
                canonical_s1
                    .iter()
                    .enumerate()
                    .filter(|(_, s1)| occurrence_geometry_matches_render_event(s0, s1))
                    .map(|(index, _)| index)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let (perfect_matchings, unique_assignment) = count_local_perfect_bijections(&edges);
        if perfect_matchings != 1 {
            return None;
        }
        let assignment = unique_assignment?;
        let canonical_index = assignment[member_s0_indices[0]];
        let canonical = canonical_matches[canonical_index];
        let selected_matches = selected
            .iter()
            .filter(|event| {
                event.measure == canonical.measure
                    && event.pitched_ordinal == Some(canonical.pitched_ordinal)
            })
            .count();
        if selected_matches != 1 {
            return None;
        }
        used_local_bijection = true;
    }

    if used_local_bijection {
        Some(BeamOccurrenceIdentityClosureProof::LocalSourceAttackBijection)
    } else if used_source_attack_bridge {
        Some(BeamOccurrenceIdentityClosureProof::SourceAttackBridge)
    } else {
        None
    }
}

fn build_beam_occurrence_source_attack_multiplicity_local_bijection_audit(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    canonical_occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    render_xml: &str,
) -> BeamOccurrenceSourceAttackMultiplicityLocalBijectionAudit {
    let Some((_, render_events)) = parse_render_phase(render_xml) else {
        return BeamOccurrenceSourceAttackMultiplicityLocalBijectionAudit::default();
    };
    let mut audit = BeamOccurrenceSourceAttackMultiplicityLocalBijectionAudit::default();
    let mut s0_by_identity = BTreeMap::<(usize, usize), Vec<&PreSerializationOccurrenceRow>>::new();
    for row in s0_occurrence_rows {
        s0_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }
    let mut canonical_by_identity = BTreeMap::<
        (usize, usize),
        Vec<&NotationVoiceCanonicalOccurrenceIdentityRow>,
    >::new();
    for row in canonical_occurrence_rows {
        canonical_by_identity
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }

    for metric in metric_candidates {
        let closed_matches = closed_candidates
            .iter()
            .filter(|closed| closed_same_line_candidate_matches_metric_line(closed, metric))
            .count();
        if closed_matches != 1 {
            continue;
        }
        let Some(selected) = frozen_class_render_structural_binding(metric, &render_events) else {
            continue;
        };
        let voices = selected
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        if voices.len() <= 1
            || frozen_class_persisted_occurrence_binding(metric, &selected, canonical_occurrence_rows)
        {
            continue;
        }

        let repeated_identities = metric
            .members
            .iter()
            .map(|member| (member.source_event_id, member.attack_id))
            .filter(|identity| s0_by_identity.get(identity).is_some_and(|rows| rows.len() > 1))
            .collect::<BTreeSet<_>>();
        if repeated_identities.is_empty() {
            continue;
        }
        audit.blocked_cross_voice_observations =
            audit.blocked_cross_voice_observations.saturating_add(1);

        let mut observation_proven = true;
        let mut group_rows = Vec::<String>::new();
        for identity in repeated_identities {
            audit.local_identity_groups = audit.local_identity_groups.saturating_add(1);
            let s0_matches = s0_by_identity
                .get(&identity)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let canonical_matches = canonical_by_identity
                .get(&identity)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            if s0_matches.len() != canonical_matches.len() || s0_matches.len() <= 1 {
                audit.blocked_cardinality_mismatch =
                    audit.blocked_cardinality_mismatch.saturating_add(1);
                observation_proven = false;
                group_rows.push(format!(
                    "{}:{}:state=BLOCKED_CARDINALITY_MISMATCH:s0_count={}:canonical_count={}",
                    identity.0,
                    identity.1,
                    s0_matches.len(),
                    canonical_matches.len(),
                ));
                continue;
            }

            let metric_members = metric
                .members
                .iter()
                .filter(|member| {
                    member.source_event_id == identity.0 && member.attack_id == identity.1
                })
                .collect::<Vec<_>>();
            if metric_members.len() != 1 {
                audit.blocked_member_s0_locator =
                    audit.blocked_member_s0_locator.saturating_add(1);
                observation_proven = false;
                group_rows.push(format!(
                    "{}:{}:state=BLOCKED_MEMBER_S0_LOCATOR:metric_member_count={}",
                    identity.0,
                    identity.1,
                    metric_members.len(),
                ));
                continue;
            }
            let metric_member = metric_members[0];
            let member_s0_indices = s0_matches
                .iter()
                .enumerate()
                .filter(|(_, row)| {
                    row.measure == metric_member.measure
                        && row.pitched_ordinal == metric_member.pitched_ordinal
                })
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            if member_s0_indices.len() != 1 {
                audit.blocked_member_s0_locator =
                    audit.blocked_member_s0_locator.saturating_add(1);
                observation_proven = false;
                group_rows.push(format!(
                    "{}:{}:state=BLOCKED_MEMBER_S0_LOCATOR:member={}:{}:s0_locator_count={}",
                    identity.0,
                    identity.1,
                    metric_member.measure,
                    metric_member.pitched_ordinal,
                    member_s0_indices.len(),
                ));
                continue;
            }

            let mut canonical_s1 = Vec::<&RenderMetricEvent>::new();
            let mut canonical_locator_ok = true;
            for canonical in canonical_matches {
                let matches = render_events
                    .iter()
                    .filter(|event| {
                        event.measure == canonical.measure
                            && event.pitched_ordinal == Some(canonical.pitched_ordinal)
                    })
                    .collect::<Vec<_>>();
                if matches.len() != 1 {
                    canonical_locator_ok = false;
                    break;
                }
                canonical_s1.push(matches[0]);
            }
            if !canonical_locator_ok {
                audit.blocked_canonical_s1_locator =
                    audit.blocked_canonical_s1_locator.saturating_add(1);
                observation_proven = false;
                group_rows.push(format!(
                    "{}:{}:state=BLOCKED_CANONICAL_S1_LOCATOR:s0_count={}:canonical_count={}",
                    identity.0,
                    identity.1,
                    s0_matches.len(),
                    canonical_matches.len(),
                ));
                continue;
            }

            let edges = s0_matches
                .iter()
                .map(|s0| {
                    canonical_s1
                        .iter()
                        .enumerate()
                        .filter(|(_, s1)| occurrence_geometry_matches_render_event(s0, s1))
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let (perfect_matchings, unique_assignment) = count_local_perfect_bijections(&edges);
            if perfect_matchings == 0 {
                audit.blocked_no_perfect_bijection =
                    audit.blocked_no_perfect_bijection.saturating_add(1);
                observation_proven = false;
                group_rows.push(format!(
                    "{}:{}:state=BLOCKED_NO_PERFECT_BIJECTION:s0_count={}:canonical_count={}:edges={:?}",
                    identity.0,
                    identity.1,
                    s0_matches.len(),
                    canonical_matches.len(),
                    edges,
                ));
                continue;
            }
            if perfect_matchings > 1 {
                audit.blocked_ambiguous_perfect_bijection = audit
                    .blocked_ambiguous_perfect_bijection
                    .saturating_add(1);
                observation_proven = false;
                group_rows.push(format!(
                    "{}:{}:state=BLOCKED_AMBIGUOUS_PERFECT_BIJECTION:s0_count={}:canonical_count={}:perfect_matchings=at_least_2",
                    identity.0,
                    identity.1,
                    s0_matches.len(),
                    canonical_matches.len(),
                ));
                continue;
            }

            let assignment = unique_assignment.expect("one perfect matching has one assignment");
            let member_s0_index = member_s0_indices[0];
            let canonical_index = assignment[member_s0_index];
            let canonical = canonical_matches[canonical_index];
            let selected_matches = selected
                .iter()
                .filter(|event| {
                    event.measure == canonical.measure
                        && event.pitched_ordinal == Some(canonical.pitched_ordinal)
                })
                .count();
            if selected_matches != 1 {
                audit.blocked_member_not_in_selected_s1 =
                    audit.blocked_member_not_in_selected_s1.saturating_add(1);
                observation_proven = false;
                group_rows.push(format!(
                    "{}:{}:state=BLOCKED_MEMBER_NOT_IN_SELECTED_S1:member_s0={}:{}:canonical={}:{}:selected_count={}",
                    identity.0,
                    identity.1,
                    s0_matches[member_s0_index].measure,
                    s0_matches[member_s0_index].pitched_ordinal,
                    canonical.measure,
                    canonical.pitched_ordinal,
                    selected_matches,
                ));
                continue;
            }

            audit.proven_unique_local_bijections =
                audit.proven_unique_local_bijections.saturating_add(1);
            group_rows.push(format!(
                "{}:{}:state=PROVEN_UNIQUE_LOCAL_BIJECTION:s0_count={}:canonical_count={}:member_s0={}:{}:canonical={}:{}:kind={}:fragment={}",
                identity.0,
                identity.1,
                s0_matches.len(),
                canonical_matches.len(),
                s0_matches[member_s0_index].measure,
                s0_matches[member_s0_index].pitched_ordinal,
                canonical.measure,
                canonical.pitched_ordinal,
                canonical.materialization_kind_rank,
                canonical.fragment_ordinal,
            ));
        }

        if observation_proven {
            audit.observations_proven_after_local_bijection = audit
                .observations_proven_after_local_bijection
                .saturating_add(1);
        } else {
            audit.observations_unresolved_after_local_bijection = audit
                .observations_unresolved_after_local_bijection
                .saturating_add(1);
        }
        audit.rows.push(format!(
            "metric_line_id={} measure={} staff={} interval={}-{} state={} presentation_voices={} groups=[{}]",
            metric.metric_line_id,
            metric.measure,
            metric.staff,
            metric.interval_start,
            metric.interval_end,
            if observation_proven {
                "PROVEN_ALL_LOCAL_SOURCE_ATTACK_BIJECTIONS"
            } else {
                "BLOCKED_LOCAL_SOURCE_ATTACK_BIJECTION"
            },
            voices.into_iter().collect::<Vec<_>>().join(","),
            group_rows.join("|"),
        ));
    }
    audit.rows.sort();
    audit.rows.dedup();
    audit
}

#[cfg(test)]
fn build_frozen_primary_beam_serialization_voice_partition_classification(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    render_xml: &str,
) -> FrozenPrimaryBeamSerializationVoicePartitionClassification {
    build_frozen_primary_beam_serialization_voice_partition_classification_with_authority(
        metric_candidates,
        closed_candidates,
        s0_occurrence_rows,
        occurrence_rows,
        &BTreeSet::new(),
        &BTreeMap::new(),
        render_xml,
    )
}

fn build_renderer_visible_primary_beam_geometry_audit(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    render_xml: &str,
    rendered_pdf: &Path,
) -> RendererVisiblePrimaryBeamGeometryAudit {
    let Some((_, render_events)) = parse_render_phase(render_xml) else {
        return RendererVisiblePrimaryBeamGeometryAudit::default();
    };
    let geometry = match super::renderer_system_break_readback::primary_beam_geometry_readback_xml_pdf(
        render_xml,
        rendered_pdf,
    ) {
        Ok(geometry) => geometry,
        Err(error) => {
            return RendererVisiblePrimaryBeamGeometryAudit {
                rows: vec![format!(
                    "state=UNAVAILABLE_PDF_VECTOR_READBACK reason={}",
                    error.to_string().replace(['\n', '\r'], " ")
                )],
                ..RendererVisiblePrimaryBeamGeometryAudit::default()
            };
        }
    };
    let mut audit = RendererVisiblePrimaryBeamGeometryAudit {
        pdf_readback_ok: true,
        measure_count: geometry.measure_count,
        bound_measure_staff_cells: geometry.bound_measure_staff_cells,
        unresolved_measure_staff_cells: geometry.unresolved_measure_staff_cells,
        ..RendererVisiblePrimaryBeamGeometryAudit::default()
    };

    for metric in metric_candidates {
        let closed_matches = closed_candidates
            .iter()
            .filter(|closed| closed_same_line_candidate_matches_metric_line(closed, metric))
            .count();
        if closed_matches != 1 {
            continue;
        }
        let Some(selected) = frozen_class_render_structural_binding(metric, &render_events) else {
            continue;
        };
        let voices = selected
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        if voices.len() <= 1 {
            continue;
        }
        audit.cross_voice_observations = audit.cross_voice_observations.saturating_add(1);
        let occurrence_identity_proof = beam_occurrence_identity_closure_binding(
            metric,
            &selected,
            s0_occurrence_rows,
            occurrence_rows,
            &render_events,
        );
        let Some(occurrence_identity_proof) = occurrence_identity_proof else {
            audit.occurrence_identity_unresolved = audit.occurrence_identity_unresolved.saturating_add(1);
            audit.geometry_unavailable = audit.geometry_unavailable.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} state=UNAVAILABLE_OCCURRENCE_IDENTITY",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
            ));
            continue;
        };
        audit.occurrence_identity_bound = audit.occurrence_identity_bound.saturating_add(1);
        let pitched_ordinals = selected
            .iter()
            .map(|event| event.pitched_ordinal)
            .collect::<Option<Vec<_>>>();
        let Some(pitched_ordinals) = pitched_ordinals else {
            audit.geometry_unavailable = audit.geometry_unavailable.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} identity={} state=UNAVAILABLE_PITCHED_ORDINAL",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                occurrence_identity_proof.label(),
            ));
            continue;
        };
        let Some(cohort_geometry) = geometry.cohort_geometry(metric.measure, metric.staff, &pitched_ordinals) else {
            audit.geometry_unavailable = audit.geometry_unavailable.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} identity={} pitched_ordinals={} state=UNAVAILABLE_PDF_STEM_OR_BEAM_BINDING",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                occurrence_identity_proof.label(),
                pitched_ordinals.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(","),
            ));
            continue;
        };
        audit.renderer_geometry_bound = audit.renderer_geometry_bound.saturating_add(1);
        audit.visible_fragments_by_metric_line_id.insert(
            metric.metric_line_id.clone(),
            cohort_geometry.visible_fragments,
        );
        let state = if cohort_geometry.visible_fragments == 1 {
            audit.continuity_preserved = audit.continuity_preserved.saturating_add(1);
            "VISIBLE_PRIMARY_BEAM_CONTINUITY_PRESERVED"
        } else {
            audit.continuity_fragmented = audit.continuity_fragmented.saturating_add(1);
            "VISIBLE_PRIMARY_BEAM_CONTINUITY_FRAGMENTED"
        };
        audit.rows.push(format!(
            "metric_line_id={} measure={} staff={} interval={}-{} identity={} presentation_voices={} pitched_ordinals={} connected_adjacent_pairs={} visible_primary_beam_fragments={} state={}",
            metric.metric_line_id,
            metric.measure,
            metric.staff,
            metric.interval_start,
            metric.interval_end,
            occurrence_identity_proof.label(),
            voices.into_iter().collect::<Vec<_>>().join(","),
            pitched_ordinals.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(","),
            cohort_geometry.connected_adjacent_pairs,
            cohort_geometry.visible_fragments,
            state,
        ));
    }
    audit.rows.sort();
    audit.rows.dedup();
    audit
}

fn build_frozen_primary_beam_serialization_voice_partition_classification_with_authority(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    authority_bound_metric_line_ids: &BTreeSet<String>,
    renderer_visible_fragments_by_metric_line_id: &BTreeMap<String, usize>,
    render_xml: &str,
) -> FrozenPrimaryBeamSerializationVoicePartitionClassification {
    let Some((_, render_events)) = parse_render_phase(render_xml) else {
        return FrozenPrimaryBeamSerializationVoicePartitionClassification::default();
    };
    let mut classification = FrozenPrimaryBeamSerializationVoicePartitionClassification {
        render_parse_ok: true,
        ..FrozenPrimaryBeamSerializationVoicePartitionClassification::default()
    };
    for metric in metric_candidates {
        let closed_matches = closed_candidates
            .iter()
            .filter(|closed| closed_same_line_candidate_matches_metric_line(closed, metric))
            .count();
        if closed_matches != 1 {
            continue;
        }
        classification.exact_closed_same_line_metric_bindings = classification
            .exact_closed_same_line_metric_bindings
            .saturating_add(1);
        let Some(selected) = frozen_class_render_structural_binding(metric, &render_events) else {
            continue;
        };
        classification.exact_closed_same_line_render_structural_bindings = classification
            .exact_closed_same_line_render_structural_bindings
            .saturating_add(1);
        let voices = selected
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        if voices.len() <= 1 {
            continue;
        }
        classification.persisted_render_cross_voice_structural_observations = classification
            .persisted_render_cross_voice_structural_observations
            .saturating_add(1);
        let occurrence_identity_proof = beam_occurrence_identity_closure_binding(
            metric,
            &selected,
            s0_occurrence_rows,
            occurrence_rows,
            &render_events,
        );
        match occurrence_identity_proof {
            Some(BeamOccurrenceIdentityClosureProof::LegacyExactLocator) => {
                classification.persisted_render_occurrence_identity_legacy_exact_bindings =
                    classification
                        .persisted_render_occurrence_identity_legacy_exact_bindings
                        .saturating_add(1);
                classification.persisted_render_occurrence_identity_exact_bindings =
                    classification
                        .persisted_render_occurrence_identity_exact_bindings
                        .saturating_add(1);
            }
            Some(BeamOccurrenceIdentityClosureProof::SourceAttackBridge) => {
                classification.persisted_render_occurrence_identity_source_attack_bridge_bindings =
                    classification
                        .persisted_render_occurrence_identity_source_attack_bridge_bindings
                        .saturating_add(1);
                classification.persisted_render_occurrence_identity_exact_bindings =
                    classification
                        .persisted_render_occurrence_identity_exact_bindings
                        .saturating_add(1);
            }
            Some(BeamOccurrenceIdentityClosureProof::LocalSourceAttackBijection) => {
                classification.persisted_render_occurrence_identity_local_bijection_bindings =
                    classification
                        .persisted_render_occurrence_identity_local_bijection_bindings
                        .saturating_add(1);
                classification.persisted_render_occurrence_identity_exact_bindings =
                    classification
                        .persisted_render_occurrence_identity_exact_bindings
                        .saturating_add(1);
            }
            None => {
                classification.persisted_render_occurrence_identity_unresolved = classification
                    .persisted_render_occurrence_identity_unresolved
                    .saturating_add(1);
            }
        }
        let beam_fragments = beam_fragment_count(&selected);
        if beam_fragments == 1 {
            classification.persisted_render_cross_voice_single_beam_fragment_observations =
                classification
                    .persisted_render_cross_voice_single_beam_fragment_observations
                    .saturating_add(1);
        } else if beam_fragments > 1 {
            classification.persisted_render_cross_voice_multiple_beam_fragment_observations =
                classification
                    .persisted_render_cross_voice_multiple_beam_fragment_observations
                    .saturating_add(1);
        }
        let authority_bound = authority_bound_metric_line_ids.contains(&metric.metric_line_id);
        let authority_binding = if authority_bound {
            "current_canonical_owner_receipt_bound_exact_occurrence_cohort"
        } else {
            "unavailable_current_projection_has_encoding_witness_not_historical_owner"
        };
        let visible_fragments = renderer_visible_fragments_by_metric_line_id
            .get(&metric.metric_line_id)
            .copied();
        let (renderer_geometry, class_membership) = match (occurrence_identity_proof, visible_fragments) {
            (None, _) => {
                classification.class_members_unresolved = classification.class_members_unresolved.saturating_add(1);
                ("unavailable_occurrence_identity".to_string(), "UNRESOLVED_MISSING_EXACT_OCCURRENCE_IDENTITY")
            }
            (Some(_), None) => {
                classification.class_members_unresolved = classification.class_members_unresolved.saturating_add(1);
                ("unavailable".to_string(), "UNRESOLVED_RENDERER_GEOMETRY")
            }
            (Some(_), Some(1)) => (
                "available_continuous_single_fragment".to_string(),
                "NOT_MEMBER_RENDERER_VISIBLE_CONTINUITY_PRESERVED",
            ),
            (Some(_), Some(fragment_count)) if authority_bound => {
                classification.class_members_proven = classification.class_members_proven.saturating_add(1);
                classification
                    .proven_metric_line_ids
                    .insert(metric.metric_line_id.clone());
                (format!("available_fragmented_{fragment_count}"), "PROVEN_FROZEN_CLASS_MEMBER")
            }
            (Some(_), Some(fragment_count)) => {
                classification.class_members_unresolved = classification.class_members_unresolved.saturating_add(1);
                (format!("available_fragmented_{fragment_count}"), "UNRESOLVED_MISSING_CANONICAL_PRIMARY_BEAM_AUTHORITY")
            }
        };
        classification.rows.push(format!(
            "metric_line_id={} measure={} staff={} interval={}-{} pre_serialization_cohort_binding=exact final_render_binding={} presentation_voices={} persisted_xml_primary_beam_fragments={} canonical_primary_beam_authority_binding={} semantic_same_line_closure=present_pre_serialization renderer_visible_primary_beam_geometry_readback={} class_membership={}",
            metric.metric_line_id,
            metric.measure,
            metric.staff,
            metric.interval_start,
            metric.interval_end,
            occurrence_identity_proof
                .map(BeamOccurrenceIdentityClosureProof::label)
                .unwrap_or("unresolved_persisted_occurrence_identity"),
            voices.into_iter().collect::<Vec<_>>().join(","),
            beam_fragments,
            authority_binding,
            renderer_geometry,
            class_membership,
        ));
    }
    classification.rows.sort();
    classification.rows.dedup();
    classification
}

fn build_primary_beam_serialization_voice_partition_repair_scope_audit(
    frozen_class: &FrozenPrimaryBeamSerializationVoicePartitionClassification,
    materialization_safety_proven_metric_line_ids: &BTreeSet<String>,
) -> PrimaryBeamSerializationVoicePartitionRepairScopeAudit {
    let mut audit = PrimaryBeamSerializationVoicePartitionRepairScopeAudit {
        class_members_proven: frozen_class.class_members_proven,
        class_members_unresolved: frozen_class.class_members_unresolved,
        materialization_safety_proven_candidates:
            materialization_safety_proven_metric_line_ids.len(),
        ..PrimaryBeamSerializationVoicePartitionRepairScopeAudit::default()
    };

    for metric_line_id in &frozen_class.proven_metric_line_ids {
        if materialization_safety_proven_metric_line_ids.contains(metric_line_id) {
            audit.repair_scope_eligible = audit.repair_scope_eligible.saturating_add(1);
            audit.eligible_metric_line_ids.insert(metric_line_id.clone());
            audit.rows.push(format!(
                "metric_line_id={} class_membership=PROVEN_FROZEN_CLASS_MEMBER materialization_safety=proven state=REPAIR_SCOPE_ELIGIBLE_ANALYSIS_ONLY",
                metric_line_id
            ));
        } else {
            audit.blocked_materialization_safety =
                audit.blocked_materialization_safety.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} class_membership=PROVEN_FROZEN_CLASS_MEMBER materialization_safety=not_proven state=BLOCKED_REPAIR_SCOPE_MATERIALIZATION_SAFETY",
                metric_line_id
            ));
        }
    }
    audit.rows.sort();
    audit.rows.dedup();
    audit
}

fn late_rebind_replay_member_event<'a>(
    member: &PreSerializationClosedSameLineMemberBinding,
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    canonical_occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    render_events: &'a [RenderMetricEvent],
) -> Option<&'a RenderMetricEvent> {
    let s0_matches = s0_occurrence_rows
        .iter()
        .filter(|row| {
            row.source_event_id == member.source_event_id
                && row.attack_id == member.attack_id
                && row.measure == member.measure
                && row.pitched_ordinal == member.pitched_ordinal
                && row.staff == member.staff
                && row.represented_onset == member.represented_onset
                && row.represented_duration == member.represented_duration
        })
        .collect::<Vec<_>>();
    if s0_matches.len() != 1 {
        return None;
    }
    let s0 = s0_matches[0];

    let mut matches = Vec::<&RenderMetricEvent>::new();
    for canonical in canonical_occurrence_rows.iter().filter(|row| {
        row.source_event_id == member.source_event_id && row.attack_id == member.attack_id
    }) {
        let physical = render_events
            .iter()
            .filter(|event| {
                event.measure == canonical.measure
                    && event.pitched_ordinal == Some(canonical.pitched_ordinal)
                    && occurrence_geometry_matches_render_event(s0, event)
            })
            .collect::<Vec<_>>();
        if physical.len() == 1 {
            matches.push(physical[0]);
        }
    }
    matches.sort_by_key(|event| (event.measure, event.pitched_ordinal));
    matches.dedup_by_key(|event| (event.measure, event.pitched_ordinal));
    if matches.len() == 1 {
        Some(matches[0])
    } else {
        None
    }
}

fn build_primary_beam_serialization_voice_partition_late_rebind_evidence_replay_audit(
    repair_scope: &PrimaryBeamSerializationVoicePartitionRepairScopeAudit,
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    canonical_occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    target_evidence_rows: &[MaterializationSafetyPrimaryTargetEvidenceRow],
    render_xml: &str,
) -> PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAudit {
    let Some((_, render_events)) = parse_render_phase(render_xml) else {
        return PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAudit {
            eligible_metric_lines: repair_scope.eligible_metric_line_ids.len(),
            ..PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAudit::default()
        };
    };

    let mut audit = PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAudit {
        render_parse_ok: true,
        eligible_metric_lines: repair_scope.eligible_metric_line_ids.len(),
        ..PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAudit::default()
    };
    let mut claimed_sources = BTreeSet::<usize>::new();
    let mut claimed_footprints = BTreeSet::<(usize, usize)>::new();

    for metric_line_id in &repair_scope.eligible_metric_line_ids {
        let metric_matches = metric_candidates
            .iter()
            .filter(|candidate| candidate.metric_line_id == *metric_line_id)
            .collect::<Vec<_>>();
        if metric_matches.len() != 1 {
            audit.rows.push(format!(
                "metric_line_id={} metric_candidate_binding=count_{} state=BLOCKED_REPLAY_METRIC_CANDIDATE_BINDING",
                metric_line_id,
                metric_matches.len()
            ));
            continue;
        }
        audit.metric_candidate_bindings = audit.metric_candidate_bindings.saturating_add(1);
        let metric = metric_matches[0];

        let target_matches = target_evidence_rows
            .iter()
            .filter(|row| row.metric_line_id == *metric_line_id)
            .collect::<Vec<_>>();
        if target_matches.len() != 1 {
            audit.target_evidence_unresolved = audit.target_evidence_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} target_evidence_binding=count_{} state=BLOCKED_REPLAY_PRIMARY_TARGET_EVIDENCE",
                metric_line_id,
                target_matches.len()
            ));
            continue;
        }
        audit.target_evidence_bindings = audit.target_evidence_bindings.saturating_add(1);
        let target_evidence = target_matches[0];

        let target_members = metric
            .members
            .iter()
            .filter(|member| {
                member.source_event_id == target_evidence.target_primary_source_event_id
                    && member.attack_id == target_evidence.target_primary_attack_id
            })
            .collect::<Vec<_>>();
        if target_members.len() != 1 {
            audit.target_evidence_unresolved = audit.target_evidence_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} target_source={} target_attack={} target_channel={} target_member_binding=count_{} state=BLOCKED_REPLAY_PRIMARY_TARGET_MEMBER",
                metric_line_id,
                target_evidence.target_primary_source_event_id,
                target_evidence.target_primary_attack_id,
                target_evidence.target_channel,
                target_members.len()
            ));
            continue;
        }

        let Some(target_event) = late_rebind_replay_member_event(
            target_members[0],
            s0_occurrence_rows,
            canonical_occurrence_rows,
            &render_events,
        ) else {
            audit.member_occurrences_unresolved =
                audit.member_occurrences_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} target_source={} target_attack={} target_channel={} state=BLOCKED_REPLAY_PRIMARY_TARGET_PHYSICAL_OCCURRENCE",
                metric_line_id,
                target_evidence.target_primary_source_event_id,
                target_evidence.target_primary_attack_id,
                target_evidence.target_channel,
            ));
            continue;
        };
        let target_voice = target_event.voice.clone();

        let mut line_failures = Vec::<String>::new();
        let mut current_voices = BTreeSet::<String>::new();
        let mut proposed = Vec::<String>::new();
        let mut line_member_bound = 0usize;
        let mut line_changes = 0usize;
        let mut line_already_target = 0usize;
        let mut line_sources = Vec::<usize>::new();
        let mut line_footprints = Vec::<(usize, usize)>::new();

        for member in &metric.members {
            let Some(event) = late_rebind_replay_member_event(
                member,
                s0_occurrence_rows,
                canonical_occurrence_rows,
                &render_events,
            ) else {
                audit.member_occurrences_unresolved =
                    audit.member_occurrences_unresolved.saturating_add(1);
                line_failures.push(format!(
                    "unresolved_occurrence:{}:{}",
                    member.source_event_id, member.attack_id
                ));
                continue;
            };
            let Some(pitched_ordinal) = event.pitched_ordinal else {
                audit.member_occurrences_unresolved =
                    audit.member_occurrences_unresolved.saturating_add(1);
                line_failures.push(format!(
                    "missing_pitched_ordinal:{}:{}",
                    member.source_event_id, member.attack_id
                ));
                continue;
            };
            line_member_bound = line_member_bound.saturating_add(1);
            audit.member_occurrences_bound = audit.member_occurrences_bound.saturating_add(1);
            current_voices.insert(event.voice.clone());
            line_sources.push(member.source_event_id);
            line_footprints.push((event.measure, pitched_ordinal));
            if event.voice == target_voice {
                line_already_target = line_already_target.saturating_add(1);
            } else {
                line_changes = line_changes.saturating_add(1);
            }
            proposed.push(format!(
                "{}:{}:{}:{}->{}",
                member.source_event_id,
                event.measure,
                pitched_ordinal,
                event.voice,
                target_voice
            ));
        }

        let target_voice_exists = current_voices.contains(&target_voice);
        if target_voice_exists {
            audit.existing_target_voice_bindings =
                audit.existing_target_voice_bindings.saturating_add(1);
        } else {
            line_failures.push("target_voice_not_in_current_metric_line".to_string());
        }
        if line_member_bound != metric.members.len() {
            line_failures.push(format!(
                "member_binding_cardinality:{}:{}",
                line_member_bound,
                metric.members.len()
            ));
        }

        for source in &line_sources {
            if !claimed_sources.insert(*source) {
                audit.source_overlap_conflicts = audit.source_overlap_conflicts.saturating_add(1);
                line_failures.push(format!("source_overlap:{source}"));
            }
        }
        for footprint in &line_footprints {
            if !claimed_footprints.insert(*footprint) {
                audit.physical_edit_footprint_conflicts =
                    audit.physical_edit_footprint_conflicts.saturating_add(1);
                line_failures.push(format!(
                    "physical_edit_footprint_overlap:{}:{}",
                    footprint.0, footprint.1
                ));
            }
        }

        line_failures.sort();
        line_failures.dedup();
        audit.proposed_voice_scalar_changes =
            audit.proposed_voice_scalar_changes.saturating_add(line_changes);
        audit.already_target_voice_scalars = audit
            .already_target_voice_scalars
            .saturating_add(line_already_target);
        let state = if line_failures.is_empty() {
            audit.replay_ready_metric_lines = audit.replay_ready_metric_lines.saturating_add(1);
            "LATE_REBIND_EVIDENCE_REPLAY_READY_ANALYSIS_ONLY"
        } else {
            "BLOCKED_LATE_REBIND_EVIDENCE_REPLAY"
        };
        audit.rows.push(format!(
            "metric_line_id={} target_source={} target_attack={} target_channel={} target_voice={} current_voices=[{}] members_bound={}/{} proposed_voice_scalar_changes={} already_target={} proposed=[{}] failures=[{}] state={}",
            metric_line_id,
            target_evidence.target_primary_source_event_id,
            target_evidence.target_primary_attack_id,
            target_evidence.target_channel,
            target_voice,
            current_voices.into_iter().collect::<Vec<_>>().join(","),
            line_member_bound,
            metric.members.len(),
            line_changes,
            line_already_target,
            proposed.join("|"),
            line_failures.join("|"),
            state,
        ));
    }

    audit.rows.sort();
    audit.rows.dedup();
    audit
}

#[cfg(test)]
fn build_canonical_primary_beam_authority_binding_audit(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    pre_final_primary_beam_exact_proof_chain_ids: &BTreeSet<String>,
    render_xml: &str,
) -> CanonicalPrimaryBeamAuthorityBindingAudit {
    build_canonical_primary_beam_authority_binding_audit_with_owner_receipts(
        metric_candidates,
        closed_candidates,
        s0_occurrence_rows,
        occurrence_rows,
        pre_final_primary_beam_exact_proof_chain_ids,
        &[],
        render_xml,
    )
}

fn build_canonical_primary_beam_authority_binding_audit_with_owner_receipts(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    pre_final_primary_beam_exact_proof_chain_ids: &BTreeSet<String>,
    owner_receipts: &[CanonicalPrimaryBeamOwnerReceipt],
    render_xml: &str,
) -> CanonicalPrimaryBeamAuthorityBindingAudit {
    let Some((_, render_events)) = parse_render_phase(render_xml) else {
        return CanonicalPrimaryBeamAuthorityBindingAudit::default();
    };
    let mut audit = CanonicalPrimaryBeamAuthorityBindingAudit {
        render_parse_ok: true,
        ..CanonicalPrimaryBeamAuthorityBindingAudit::default()
    };

    for metric in metric_candidates {
        let closed_matches = closed_candidates
            .iter()
            .filter(|closed| closed_same_line_candidate_matches_metric_line(closed, metric))
            .collect::<Vec<_>>();
        if closed_matches.len() != 1 {
            continue;
        }
        let Some(selected) = frozen_class_render_structural_binding(metric, &render_events) else {
            continue;
        };
        let voices = selected
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        if voices.len() <= 1 {
            continue;
        }
        audit.cross_voice_observations = audit.cross_voice_observations.saturating_add(1);

        let identity_proof = beam_occurrence_identity_closure_binding(
            metric,
            &selected,
            s0_occurrence_rows,
            occurrence_rows,
            &render_events,
        );
        let identity_label = identity_proof
            .map(BeamOccurrenceIdentityClosureProof::label)
            .unwrap_or("unresolved_persisted_occurrence_identity");
        if identity_proof.is_none() {
            audit.occurrence_identity_unresolved =
                audit.occurrence_identity_unresolved.saturating_add(1);
            audit.canonical_authority_bindings_unresolved =
                audit.canonical_authority_bindings_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=not_evaluated typed_historical_owner_receipt=unavailable state=BLOCKED_OCCURRENCE_IDENTITY_BEFORE_BEAM_AUTHORITY_BINDING",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
            ));
            continue;
        }
        audit.occurrence_identity_bound = audit.occurrence_identity_bound.saturating_add(1);

        let closed = closed_matches[0];
        let pre_final_exact = pre_final_primary_beam_exact_proof_chain_ids
            .contains(&closed.proof_chain_id);
        if pre_final_exact {
            audit.pre_final_chord_exact_encoding_witnesses = audit
                .pre_final_chord_exact_encoding_witnesses
                .saturating_add(1);
        } else {
            audit.pre_final_chord_encoding_witness_missing = audit
                .pre_final_chord_encoding_witness_missing
                .saturating_add(1);
        }

        if !pre_final_exact {
            audit.canonical_authority_bindings_unresolved = audit
                .canonical_authority_bindings_unresolved
                .saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=missing current_canonical_owner_receipt=not_evaluated historical_owner_causality=not_claimed state=BLOCKED_NO_PRE_FINAL_ENCODING_WITNESS",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
            ));
            continue;
        }

        let matching_receipts = owner_receipts
            .iter()
            .filter(|receipt| receipt.proof_chain_id == closed.proof_chain_id)
            .collect::<Vec<_>>();
        if matching_receipts.len() != 1 {
            audit.owner_receipt_cardinality_unresolved = audit
                .owner_receipt_cardinality_unresolved
                .saturating_add(1);
            audit.canonical_authority_bindings_unresolved = audit
                .canonical_authority_bindings_unresolved
                .saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=exact current_canonical_owner_receipt_count={} historical_owner_causality=not_claimed state=BLOCKED_CURRENT_CANONICAL_OWNER_RECEIPT_CARDINALITY",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
                matching_receipts.len(),
            ));
            continue;
        }

        let receipt = matching_receipts[0];
        audit.typed_owner_receipts_available = audit.typed_owner_receipts_available.saturating_add(1);
        if receipt.event_count != closed.members.len() || receipt.event_count != selected.len() {
            audit.owner_receipt_event_count_mismatch = audit
                .owner_receipt_event_count_mismatch
                .saturating_add(1);
            audit.canonical_authority_bindings_unresolved = audit
                .canonical_authority_bindings_unresolved
                .saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=exact current_canonical_owner_receipt=present owner={} receipt_events={} closed_events={} persisted_events={} historical_owner_causality=not_claimed state=BLOCKED_CURRENT_CANONICAL_OWNER_RECEIPT_EVENT_COUNT_MISMATCH",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
                receipt.owner_kind,
                receipt.event_count,
                closed.members.len(),
                selected.len(),
            ));
            continue;
        }

        let persisted_primary_states = selected
            .iter()
            .map(|event| event.beam1.as_deref())
            .collect::<Option<Vec<_>>>();
        let states_match = persisted_primary_states.as_ref().is_some_and(|states| {
            states.len() == receipt.primary_states.len()
                && states
                    .iter()
                    .zip(&receipt.primary_states)
                    .all(|(observed, intended)| *observed == *intended)
        });
        if !states_match {
            audit.owner_receipt_primary_state_mismatch = audit
                .owner_receipt_primary_state_mismatch
                .saturating_add(1);
            audit.canonical_authority_bindings_unresolved = audit
                .canonical_authority_bindings_unresolved
                .saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=exact current_canonical_owner_receipt=present owner={} intended_primary_states=[{}] persisted_primary_states=[{}] historical_owner_causality=not_claimed state=BLOCKED_CURRENT_CANONICAL_OWNER_RECEIPT_PRIMARY_STATE_MISMATCH",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
                receipt.owner_kind,
                receipt.primary_states.join(","),
                persisted_primary_states
                    .map(|states| states.join(","))
                    .unwrap_or_else(|| "missing".to_string()),
            ));
            continue;
        }

        audit.canonical_authority_bindings_proven = audit
            .canonical_authority_bindings_proven
            .saturating_add(1);
        audit.proven_metric_line_ids.insert(metric.metric_line_id.clone());
        audit.rows.push(format!(
            "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=exact current_canonical_owner_receipt=present owner={} owner_source_fnv64={:016x} plan_fingerprint_fnv64={:016x} intended_primary_states=[{}] persisted_primary_states=exact historical_owner_causality=not_claimed state=CANONICAL_PRIMARY_BEAM_AUTHORITY_BOUND_CURRENT_OWNER",
            metric.metric_line_id,
            metric.measure,
            metric.staff,
            metric.interval_start,
            metric.interval_end,
            identity_label,
            receipt.owner_kind,
            receipt.owner_source_fnv64,
            receipt.plan_fingerprint_fnv64,
            receipt.primary_states.join(","),
        ));
    }

    audit.rows.sort();
    audit.rows.dedup();
    audit
}

fn build_canonical_primary_beam_owner_receipt_transport_audit(
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    closed_candidates: &[PreSerializationClosedSameLineCandidate],
    s0_occurrence_rows: &[PreSerializationOccurrenceRow],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    pre_final_primary_beam_exact_proof_chain_ids: &BTreeSet<String>,
    owner_receipts: &[CanonicalPrimaryBeamOwnerReceipt],
    render_xml: &str,
) -> CanonicalPrimaryBeamOwnerReceiptTransportAudit {
    let Some((_, render_events)) = parse_render_phase(render_xml) else {
        return CanonicalPrimaryBeamOwnerReceiptTransportAudit::default();
    };
    let mut audit = CanonicalPrimaryBeamOwnerReceiptTransportAudit {
        render_parse_ok: true,
        ..CanonicalPrimaryBeamOwnerReceiptTransportAudit::default()
    };

    for metric in metric_candidates {
        let closed_matches = closed_candidates
            .iter()
            .filter(|closed| closed_same_line_candidate_matches_metric_line(closed, metric))
            .collect::<Vec<_>>();
        if closed_matches.len() != 1 {
            continue;
        }
        let Some(selected) = frozen_class_render_structural_binding(metric, &render_events) else {
            continue;
        };
        let voices = selected
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        if voices.len() <= 1 {
            continue;
        }
        audit.cross_voice_observations = audit.cross_voice_observations.saturating_add(1);

        let identity_proof = beam_occurrence_identity_closure_binding(
            metric,
            &selected,
            s0_occurrence_rows,
            occurrence_rows,
            &render_events,
        );
        let identity_label = identity_proof
            .map(BeamOccurrenceIdentityClosureProof::label)
            .unwrap_or("unresolved_persisted_occurrence_identity");
        if identity_proof.is_none() {
            audit.occurrence_identity_unresolved =
                audit.occurrence_identity_unresolved.saturating_add(1);
            audit.receipt_transport_unresolved =
                audit.receipt_transport_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=not_evaluated current_canonical_owner_receipt=not_evaluated historical_owner_causality=not_claimed state=BLOCKED_OCCURRENCE_IDENTITY_BEFORE_OWNER_RECEIPT_TRANSPORT",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
            ));
            continue;
        }
        audit.occurrence_identity_bound = audit.occurrence_identity_bound.saturating_add(1);

        let closed = closed_matches[0];
        let pre_final_exact = pre_final_primary_beam_exact_proof_chain_ids
            .contains(&closed.proof_chain_id);
        if pre_final_exact {
            audit.pre_final_chord_exact_encoding_witnesses = audit
                .pre_final_chord_exact_encoding_witnesses
                .saturating_add(1);
        } else {
            audit.pre_final_chord_encoding_witness_missing = audit
                .pre_final_chord_encoding_witness_missing
                .saturating_add(1);
            audit.receipt_transport_unresolved =
                audit.receipt_transport_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=missing current_canonical_owner_receipt=not_evaluated historical_owner_causality=not_claimed state=BLOCKED_PRE_FINAL_ENCODING_BEFORE_OWNER_RECEIPT_TRANSPORT",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
            ));
            continue;
        }

        let matching_receipts = owner_receipts
            .iter()
            .filter(|receipt| receipt.proof_chain_id == closed.proof_chain_id)
            .collect::<Vec<_>>();
        if matching_receipts.len() != 1 {
            audit.receipt_transport_unresolved =
                audit.receipt_transport_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=exact current_canonical_owner_receipt={} historical_owner_causality=not_claimed state=BLOCKED_CURRENT_CANONICAL_OWNER_RECEIPT_CARDINALITY",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
                matching_receipts.len(),
            ));
            continue;
        }
        let receipt = matching_receipts[0];
        audit.typed_current_canonical_owner_receipts = audit
            .typed_current_canonical_owner_receipts
            .saturating_add(1);
        if receipt.event_count != closed.members.len() || receipt.event_count != selected.len() {
            audit.receipt_event_count_mismatch =
                audit.receipt_event_count_mismatch.saturating_add(1);
            audit.receipt_transport_unresolved =
                audit.receipt_transport_unresolved.saturating_add(1);
            audit.rows.push(format!(
                "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=exact current_canonical_owner_receipt=present owner={} owner_source_fnv64={:016x} receipt_events={} closed_events={} persisted_events={} historical_owner_causality=not_claimed state=BLOCKED_OWNER_RECEIPT_EVENT_COUNT_MISMATCH",
                metric.metric_line_id,
                metric.measure,
                metric.staff,
                metric.interval_start,
                metric.interval_end,
                identity_label,
                receipt.owner_kind,
                receipt.owner_source_fnv64,
                receipt.event_count,
                closed.members.len(),
                selected.len(),
            ));
            continue;
        }

        audit.receipt_transport_proven = audit.receipt_transport_proven.saturating_add(1);
        audit.rows.push(format!(
            "metric_line_id={} measure={} staff={} interval={}-{} occurrence_identity={} pre_final_chord_encoding_witness=exact current_canonical_owner_receipt=present owner={} owner_source_fnv64={:016x} plan_fingerprint_fnv64={:016x} primary_states=[{}] event_count={} historical_owner_causality=not_claimed state=CURRENT_CANONICAL_OWNER_RECEIPT_TRANSPORT_PROVEN",
            metric.metric_line_id,
            metric.measure,
            metric.staff,
            metric.interval_start,
            metric.interval_end,
            identity_label,
            receipt.owner_kind,
            receipt.owner_source_fnv64,
            receipt.plan_fingerprint_fnv64,
            receipt.primary_states.join(","),
            receipt.event_count,
        ));
    }

    audit.rows.sort();
    audit.rows.dedup();
    audit
}

fn append_metric_line_delta(
    audit: &mut VisibleRhythmicOccupancyPhaseDeltaAudit,
    candidates: &[PreSerializationMetricNotationLineCandidate],
    render_events: &[RenderMetricEvent],
) {
    audit.metric_lines_total = candidates.len();
    for candidate in candidates {
        let mut expected = candidate
            .members
            .iter()
            .map(|member| (member.represented_onset, member.represented_duration))
            .collect::<Vec<_>>();
        expected.sort();
        let mut selected = render_events
            .iter()
            .filter(|event| {
                event.measure == candidate.measure
                    && event.staff == candidate.staff
                    && event.visible
                    && !event.is_rest
                    && !event.is_chord
                    && event.duration > 0
                    && event.onset >= candidate.interval_start
                    && event.onset.saturating_add(event.duration) <= candidate.interval_end
            })
            .collect::<Vec<_>>();
        selected.sort_by_key(|event| (event.onset, event.duration, event.voice.clone()));
        let observed = selected
            .iter()
            .map(|event| (event.onset, event.duration))
            .collect::<Vec<_>>();
        if observed != expected {
            audit.metric_lines_unresolved_render_binding = audit
                .metric_lines_unresolved_render_binding
                .saturating_add(1);
            audit.rows.push(format!(
                "METRIC_LINE_PHASE_BINDING metric_line_id={} measure={} staff={} interval={}-{} status=UNRESOLVED_RENDER_STRUCTURAL_BINDING expected_members={} observed_members={}",
                candidate.metric_line_id,
                candidate.measure,
                candidate.staff,
                candidate.interval_start,
                candidate.interval_end,
                expected.len(),
                observed.len(),
            ));
            continue;
        }

        audit.metric_lines_exact_render_binding =
            audit.metric_lines_exact_render_binding.saturating_add(1);
        let voices = selected
            .iter()
            .map(|event| event.voice.clone())
            .collect::<BTreeSet<_>>();
        let beam_fragments = beam_fragment_count(&selected);
        if voices.len() > 1 {
            audit.metric_lines_cross_voice = audit.metric_lines_cross_voice.saturating_add(1);
        }
        if beam_fragments == 0 {
            audit.metric_lines_beamless = audit.metric_lines_beamless.saturating_add(1);
        }
        if beam_fragments > 1 {
            audit.metric_lines_multiple_beam_fragments =
                audit.metric_lines_multiple_beam_fragments.saturating_add(1);
        }
        if voices.len() > 1 || beam_fragments != 1 {
            audit.rows.push(format!(
                "METRIC_LINE_PHASE_BINDING metric_line_id={} measure={} staff={} interval={}-{} status=STRUCTURALLY_BOUND_PRESENTATION_DIAGNOSTIC_ONLY presentation_voices={} primary_beam_fragments={} selection_voice_labels=excluded selection_beams=excluded",
                candidate.metric_line_id,
                candidate.measure,
                candidate.staff,
                candidate.interval_start,
                candidate.interval_end,
                voices.into_iter().collect::<Vec<_>>().join(","),
                beam_fragments,
            ));
        }
    }
}

fn build_phase_delta_audit(
    s0: &VisibleRhythmicOccupancyPhaseSnapshot,
    metric_candidates: &[PreSerializationMetricNotationLineCandidate],
    render_xml: &str,
) -> VisibleRhythmicOccupancyPhaseDeltaAudit {
    let Some((render_rows, render_metric_events)) = parse_render_phase(render_xml) else {
        return VisibleRhythmicOccupancyPhaseDeltaAudit::default();
    };
    let s1 = build_visible_rhythmic_occupancy_phase_snapshot(render_xml, &render_rows);
    let mut audit = VisibleRhythmicOccupancyPhaseDeltaAudit {
        render_parse_ok: true,
        s1_metric_parse_ok: s1.metric_parse_ok,
        s0_visible_atoms: s0.visible_atoms.len(),
        s1_visible_atoms: s1.visible_atoms.len(),
        s0_incomplete_groups: s0.incomplete_groups.len(),
        s1_incomplete_groups: s1.incomplete_groups.len(),
        ..VisibleRhythmicOccupancyPhaseDeltaAudit::default()
    };

    let s0_visible = s0
        .visible_atoms
        .iter()
        .map(atom_signature)
        .collect::<BTreeSet<_>>();
    let s1_visible = s1
        .visible_atoms
        .iter()
        .map(atom_signature)
        .collect::<BTreeSet<_>>();
    let s0_hidden = s0
        .hidden_atoms
        .iter()
        .map(atom_signature)
        .collect::<BTreeSet<_>>();

    audit.shared_pitched_occupancy_signatures = s0_visible
        .intersection(&s1_visible)
        .filter(|signature| signature.4.is_pitched())
        .count();
    audit.shared_duplicate_rest_conflict_geometries = s0
        .duplicate_rest_conflicts
        .intersection(&s1.duplicate_rest_conflicts)
        .count();
    audit.shared_rest_pitched_conflict_geometries = s0
        .rest_pitched_conflicts
        .intersection(&s1.rest_pitched_conflicts)
        .count();

    for conflict in s0
        .rest_pitched_conflicts
        .intersection(&s1.rest_pitched_conflicts)
    {
        audit.rows.push(format!(
            "S0_S1_SHARED_REST_PITCH_CONFLICT_GEOMETRY measure={} staff={} rest_interval={}-{} pitched_interval={}-{} pitches=[{}]",
            conflict.measure,
            conflict.staff,
            conflict.rest_onset,
            conflict.rest_onset.saturating_add(conflict.rest_duration),
            conflict.pitched_onset,
            conflict.pitched_onset.saturating_add(conflict.pitched_duration),
            conflict.pitches.iter().map(i32::to_string).collect::<Vec<_>>().join(","),
        ));
    }
    for conflict in s1
        .duplicate_rest_conflicts
        .difference(&s0.duplicate_rest_conflicts)
    {
        audit.s1_new_duplicate_rest_conflict_geometries =
            audit.s1_new_duplicate_rest_conflict_geometries.saturating_add(1);
        audit.rows.push(format!(
            "S1_NEW_DUPLICATE_REST_CONFLICT_GEOMETRY measure={} staff={} interval={}-{}",
            conflict.measure,
            conflict.staff,
            conflict.onset,
            conflict.onset.saturating_add(conflict.duration),
        ));
    }
    for conflict in s1
        .rest_pitched_conflicts
        .difference(&s0.rest_pitched_conflicts)
    {
        let rest_signature = (
            conflict.measure,
            conflict.staff,
            conflict.rest_onset,
            conflict.rest_duration,
            VisibleRhythmicOccupancyAtomKind::Rest,
            Vec::new(),
        );
        if s0_hidden.contains(&rest_signature) && !s0_visible.contains(&rest_signature) {
            audit.s1_new_rest_pitched_conflict_geometries_at_s0_hidden_rest_signature = audit
                .s1_new_rest_pitched_conflict_geometries_at_s0_hidden_rest_signature
                .saturating_add(1);
            audit.rows.push(format!(
                "S1_NEW_REST_PITCH_CONFLICT_GEOMETRY_AT_S0_HIDDEN_REST_SIGNATURE measure={} staff={} rest_interval={}-{} pitched_interval={}-{} pitches=[{}] relation=S1_visible_rest_signature_matches_S0_hidden_rest_signature_no_event_identity",
                conflict.measure,
                conflict.staff,
                conflict.rest_onset,
                conflict.rest_onset.saturating_add(conflict.rest_duration),
                conflict.pitched_onset,
                conflict.pitched_onset.saturating_add(conflict.pitched_duration),
                conflict.pitches.iter().map(i32::to_string).collect::<Vec<_>>().join(","),
            ));
        } else {
            audit.s1_unbound_rest_pitched_conflicts =
                audit.s1_unbound_rest_pitched_conflicts.saturating_add(1);
        }
    }
    for promoted in s0_hidden
        .intersection(&s1_visible)
        .filter(|signature| !s0_visible.contains(*signature))
    {
        audit.s1_visible_at_s0_hidden_signatures =
            audit.s1_visible_at_s0_hidden_signatures.saturating_add(1);
        audit.rows.push(format!(
            "S1_VISIBLE_AT_S0_HIDDEN_SIGNATURE measure={} staff={} interval={}-{} kind={} pitches=[{}]",
            promoted.0,
            promoted.1,
            promoted.2,
            promoted.2.saturating_add(promoted.3),
            promoted.4.label(),
            promoted.5.iter().map(i32::to_string).collect::<Vec<_>>().join(","),
        ));
    }
    for row in &s0.incomplete_groups {
        if row.contains("pitched_atom_not_anchor_closed") || row.contains("orphan_chord=yes") {
            audit.blocked_incomplete_chord_bindings =
                audit.blocked_incomplete_chord_bindings.saturating_add(1);
            audit.rows.push(format!("BLOCKED_INCOMPLETE_CHORD_BINDING {row}"));
        }
    }

    append_metric_line_delta(&mut audit, metric_candidates, &render_metric_events);
    audit.rows.sort();
    audit.rows.dedup();
    audit
}

pub(crate) fn append_visible_rhythmic_occupancy_phase_delta_audit_section(
    out: &mut String,
    s0_projection: &NotationVoicePreSerializationConstraintClosureAuditProjection,
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    render_xml: &str,
    rendered_pdf: &Path,
) {
    let audit = build_phase_delta_audit(
        s0_projection.visible_rhythmic_occupancy_phase_snapshot(),
        s0_projection.metric_line_candidates(),
        render_xml,
    );
    let canonical_primary_beam_authority =
        build_canonical_primary_beam_authority_binding_audit_with_owner_receipts(
            s0_projection.metric_line_candidates(),
            s0_projection.closed_candidates(),
            s0_projection.occurrence_rows(),
            occurrence_rows,
            s0_projection.pre_final_primary_beam_exact_proof_chain_ids(),
            s0_projection.canonical_primary_beam_owner_receipts(),
            render_xml,
        );
    let renderer_primary_beam_geometry = build_renderer_visible_primary_beam_geometry_audit(
        s0_projection.metric_line_candidates(),
        s0_projection.closed_candidates(),
        s0_projection.occurrence_rows(),
        occurrence_rows,
        render_xml,
        rendered_pdf,
    );
    let frozen_class =
        build_frozen_primary_beam_serialization_voice_partition_classification_with_authority(
            s0_projection.metric_line_candidates(),
            s0_projection.closed_candidates(),
            s0_projection.occurrence_rows(),
            occurrence_rows,
            &canonical_primary_beam_authority.proven_metric_line_ids,
            &renderer_primary_beam_geometry.visible_fragments_by_metric_line_id,
            render_xml,
        );
    let repair_scope = build_primary_beam_serialization_voice_partition_repair_scope_audit(
        &frozen_class,
        s0_projection.materialization_safety_proven_metric_line_ids(),
    );
    let late_rebind_replay =
        build_primary_beam_serialization_voice_partition_late_rebind_evidence_replay_audit(
            &repair_scope,
            s0_projection.metric_line_candidates(),
            s0_projection.occurrence_rows(),
            occurrence_rows,
            s0_projection.materialization_safety_primary_target_evidence_rows(),
            render_xml,
        );
    let cross_phase_identity = build_beam_occurrence_cross_phase_identity_audit(
        s0_projection.metric_line_candidates(),
        s0_projection.closed_candidates(),
        s0_projection.occurrence_rows(),
        occurrence_rows,
        render_xml,
    );
    let local_bijection = build_beam_occurrence_source_attack_multiplicity_local_bijection_audit(
        s0_projection.metric_line_candidates(),
        s0_projection.closed_candidates(),
        s0_projection.occurrence_rows(),
        occurrence_rows,
        render_xml,
    );
    let canonical_primary_beam_owner_receipt_transport =
        build_canonical_primary_beam_owner_receipt_transport_audit(
            s0_projection.metric_line_candidates(),
            s0_projection.closed_candidates(),
            s0_projection.occurrence_rows(),
            occurrence_rows,
            s0_projection.pre_final_primary_beam_exact_proof_chain_ids(),
            s0_projection.canonical_primary_beam_owner_receipts(),
            render_xml,
        );
    let verdict = if !audit.render_parse_ok || !audit.s1_metric_parse_ok {
        "UNAVAILABLE_PARSE_FAILURE"
    } else if audit.s0_incomplete_groups > 0 || audit.s1_incomplete_groups > 0 {
        "PHASE_DELTA_AVAILABLE_WITH_INCOMPLETE_ATOMIC_BINDINGS"
    } else {
        "PHASE_DELTA_CLOSED_ANALYSIS_ONLY"
    };

    let _ = writeln!(out, "ProjectionOnly: true");
    let _ = writeln!(out, "Authority: none");
    let _ = writeln!(out, "MusicMutation: nein");
    let _ = writeln!(out, "RuleChange: nein");
    let _ = writeln!(out, "WriterAuthority: none");
    let _ = writeln!(out, "S0: canonical_final_chord_projection_before_serialized_voice_channel_plan");
    let _ = writeln!(out, "S1: persisted_final_render_musicxml");
    let _ = writeln!(out, "PhaseBindingSelectionVoiceLabels: excluded");
    let _ = writeln!(out, "PhaseBindingSelectionBeams: excluded");
    let _ = writeln!(out, "MetricLineSelection: S0_measure_staff_interval_and_ordered_onset_duration_only");
    let _ = writeln!(out, "MetricLineVoiceBeamRole: diagnostic_after_structural_binding_only");
    let _ = writeln!(
        out,
        "MeasurementVoiceInvariant: contract_tested_voice_relabeling_cannot_change_occupancy_or_conflict_geometry"
    );
    let _ = writeln!(
        out,
        "PhaseEventIdentityBinding: unavailable_persisted_render_has_no_source_event_or_attack_identity"
    );
    let _ = writeln!(out, "EventLevelVisibilityPromotionClaim: none");
    let _ = writeln!(out, "RenderParse: {}", if audit.render_parse_ok { "ok" } else { "fail" });
    let _ = writeln!(out, "S1MetricParse: {}", if audit.s1_metric_parse_ok { "ok" } else { "fail" });
    let _ = writeln!(out, "S0VisibleAtoms: {}", audit.s0_visible_atoms);
    let _ = writeln!(out, "S1VisibleAtoms: {}", audit.s1_visible_atoms);
    let _ = writeln!(out, "S0IncompleteAtomicGroups: {}", audit.s0_incomplete_groups);
    let _ = writeln!(out, "S1IncompleteAtomicGroups: {}", audit.s1_incomplete_groups);
    let _ = writeln!(
        out,
        "SharedPitchedOccupancySignatures: {}",
        audit.shared_pitched_occupancy_signatures
    );
    let _ = writeln!(
        out,
        "SharedDuplicateRestConflictGeometries: {}",
        audit.shared_duplicate_rest_conflict_geometries
    );
    let _ = writeln!(
        out,
        "SharedRestPitchedConflictGeometries: {}",
        audit.shared_rest_pitched_conflict_geometries
    );
    let _ = writeln!(
        out,
        "S1NewDuplicateRestConflictGeometries: {}",
        audit.s1_new_duplicate_rest_conflict_geometries
    );
    let _ = writeln!(
        out,
        "S1NewRestPitchedConflictGeometriesAtS0HiddenRestSignature: {}",
        audit.s1_new_rest_pitched_conflict_geometries_at_s0_hidden_rest_signature
    );
    let _ = writeln!(
        out,
        "S1UnboundRestPitchedConflictsDiagnosticOnly: {}",
        audit.s1_unbound_rest_pitched_conflicts
    );
    let _ = writeln!(
        out,
        "S1VisibleAtS0HiddenSignatures: {}",
        audit.s1_visible_at_s0_hidden_signatures
    );
    let _ = writeln!(out, "BlockedIncompleteChordBindings: {}", audit.blocked_incomplete_chord_bindings);
    let _ = writeln!(out, "MetricNotationLinesS0: {}", audit.metric_lines_total);
    let _ = writeln!(out, "MetricNotationLinesExactRenderStructuralBinding: {}", audit.metric_lines_exact_render_binding);
    let _ = writeln!(out, "MetricNotationLinesUnresolvedRenderStructuralBinding: {}", audit.metric_lines_unresolved_render_binding);
    let _ = writeln!(out, "MetricNotationLinesRenderCrossVoice: {}", audit.metric_lines_cross_voice);
    let _ = writeln!(out, "MetricNotationLinesRenderBeamless: {}", audit.metric_lines_beamless);
    let _ = writeln!(out, "MetricNotationLinesRenderMultipleBeamFragments: {}", audit.metric_lines_multiple_beam_fragments);
    let _ = writeln!(out, "Verdict: {verdict}");
    if audit.rows.is_empty() {
        let _ = writeln!(out, "Rows: none");
    } else {
        for row in audit.rows {
            let _ = writeln!(out, "Row: {row}");
        }
    }

    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityAudit: read_only");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityAuthority: none");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityRuleChange: nein");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityProductiveSelectionAuthority: none");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityPurpose: explain_S0_to_canonical_receipt_to_persisted_S1_identity_without_changing_frozen_error_class");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityS0: pre_serialization_occurrence_rows");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityBridge: SourceEventId+SoundingAttackId");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityCanonical: final_chord_transaction_occurrence_receipt_measure_pitched_ordinal_materialization_kind_fragment_ordinal");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseIdentityS1: persisted_render_measure_pitched_ordinal_plus_geometry");
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseCrossVoiceObservations: {}", cross_phase_identity.cross_voice_observations);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseLegacyExactLocatorBindings: {}", cross_phase_identity.legacy_exact_locator_bindings);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseS0SourceAttackMissing: {}", cross_phase_identity.s0_source_attack_missing);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseS0SourceAttackMultiplicity: {}", cross_phase_identity.s0_source_attack_multiplicity);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseCanonicalSourceAttackMissing: {}", cross_phase_identity.canonical_source_attack_missing);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseCanonicalSourceAttackMultiplicity: {}", cross_phase_identity.canonical_source_attack_multiplicity);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseSourceAttackBridgeExactSameLocator: {}", cross_phase_identity.source_attack_bridge_exact_same_locator);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseSourceAttackBridgeExactLocatorShift: {}", cross_phase_identity.source_attack_bridge_exact_locator_shift);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseCanonicalToS1LocatorUnresolved: {}", cross_phase_identity.canonical_to_s1_locator_unresolved);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseCanonicalToS1GeometryMismatch: {}", cross_phase_identity.canonical_to_s1_geometry_mismatch);
    let _ = writeln!(out, "BeamOccurrenceCrossPhaseUnresolvedAfterSourceAttackBridge: {}", cross_phase_identity.unresolved_after_source_attack_bridge);
    if cross_phase_identity.rows.is_empty() {
        let _ = writeln!(out, "BeamOccurrenceCrossPhaseRows: none");
    } else {
        for row in cross_phase_identity.rows {
            let _ = writeln!(out, "BeamOccurrenceCrossPhaseRow: {row}");
        }
    }

    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionAudit: read_only");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionAuthority: none");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionRuleChange: nein");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionProductiveSelectionAuthority: none");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionConflictGroup: exact_SourceEventId_plus_SoundingAttackId_occurrence_group");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionEdge: exact_staff_onset_duration_pitch_geometry_via_canonical_S1_locator");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionRanking: none");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionTieBreak: none");
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionBlockedCrossVoiceObservations: {}", local_bijection.blocked_cross_voice_observations);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionGroups: {}", local_bijection.local_identity_groups);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionProvenUnique: {}", local_bijection.proven_unique_local_bijections);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionBlockedCardinalityMismatch: {}", local_bijection.blocked_cardinality_mismatch);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionBlockedMemberS0Locator: {}", local_bijection.blocked_member_s0_locator);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionBlockedCanonicalS1Locator: {}", local_bijection.blocked_canonical_s1_locator);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionBlockedNoPerfectBijection: {}", local_bijection.blocked_no_perfect_bijection);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionBlockedAmbiguousPerfectBijection: {}", local_bijection.blocked_ambiguous_perfect_bijection);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionBlockedMemberNotInSelectedS1: {}", local_bijection.blocked_member_not_in_selected_s1);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionObservationsProven: {}", local_bijection.observations_proven_after_local_bijection);
    let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionObservationsUnresolved: {}", local_bijection.observations_unresolved_after_local_bijection);
    if local_bijection.rows.is_empty() {
        let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionRows: none");
    } else {
        for row in local_bijection.rows {
            let _ = writeln!(out, "BeamOccurrenceSourceAttackMultiplicityLocalBijectionRow: {row}");
        }
    }

    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportAudit: read_only");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportAuthority: none");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportRuleChange: nein");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportProductiveSelector: none");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportOwnerSemantics: current_canonical_source_bound_primary_level_one_plan");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportHistoricalCausalityClaim: none");
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportCrossVoiceObservations: {}", canonical_primary_beam_owner_receipt_transport.cross_voice_observations);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportOccurrenceIdentityBound: {}", canonical_primary_beam_owner_receipt_transport.occurrence_identity_bound);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportOccurrenceIdentityUnresolved: {}", canonical_primary_beam_owner_receipt_transport.occurrence_identity_unresolved);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportPreFinalChordExactEncodingWitnesses: {}", canonical_primary_beam_owner_receipt_transport.pre_final_chord_exact_encoding_witnesses);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportPreFinalChordEncodingWitnessMissing: {}", canonical_primary_beam_owner_receipt_transport.pre_final_chord_encoding_witness_missing);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportTypedCurrentCanonicalOwnerReceipts: {}", canonical_primary_beam_owner_receipt_transport.typed_current_canonical_owner_receipts);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportEventCountMismatch: {}", canonical_primary_beam_owner_receipt_transport.receipt_event_count_mismatch);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportProven: {}", canonical_primary_beam_owner_receipt_transport.receipt_transport_proven);
    let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportUnresolved: {}", canonical_primary_beam_owner_receipt_transport.receipt_transport_unresolved);
    let _ = writeln!(
        out,
        "CanonicalPrimaryBeamOwnerReceiptTransportVerdict: {}",
        if !canonical_primary_beam_owner_receipt_transport.render_parse_ok {
            "UNAVAILABLE_RENDER_PARSE_FAILURE"
        } else if canonical_primary_beam_owner_receipt_transport.receipt_transport_unresolved == 0
            && canonical_primary_beam_owner_receipt_transport.cross_voice_observations > 0
        {
            "CURRENT_CANONICAL_OWNER_RECEIPT_TRANSPORT_PROVEN_ALL_CROSS_VOICE_HISTORICAL_CAUSALITY_NOT_CLAIMED"
        } else {
            "CURRENT_CANONICAL_OWNER_RECEIPT_TRANSPORT_INCOMPLETE_FAIL_CLOSED"
        }
    );
    if canonical_primary_beam_owner_receipt_transport.rows.is_empty() {
        let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportRows: none");
    } else {
        for row in canonical_primary_beam_owner_receipt_transport.rows {
            let _ = writeln!(out, "CanonicalPrimaryBeamOwnerReceiptTransportRow: {row}");
        }
    }

    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingAudit: read_only");
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingAuthority: none");
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingRuleChange: nein");
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingProductiveSelector: none");
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingEncodingAloneIsAuthority: no");
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingRequiredReceipt: typed_existing_canonical_beam_owner_receipt_bound_to_exact_occurrence_cohort");
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingCrossVoiceObservations: {}", canonical_primary_beam_authority.cross_voice_observations);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingOccurrenceIdentityBound: {}", canonical_primary_beam_authority.occurrence_identity_bound);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingOccurrenceIdentityUnresolved: {}", canonical_primary_beam_authority.occurrence_identity_unresolved);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingPreFinalChordExactEncodingWitnesses: {}", canonical_primary_beam_authority.pre_final_chord_exact_encoding_witnesses);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingPreFinalChordEncodingWitnessMissing: {}", canonical_primary_beam_authority.pre_final_chord_encoding_witness_missing);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingTypedOwnerReceiptsAvailable: {}", canonical_primary_beam_authority.typed_owner_receipts_available);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingOwnerReceiptCardinalityUnresolved: {}", canonical_primary_beam_authority.owner_receipt_cardinality_unresolved);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingOwnerReceiptEventCountMismatch: {}", canonical_primary_beam_authority.owner_receipt_event_count_mismatch);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingOwnerReceiptPrimaryStateMismatch: {}", canonical_primary_beam_authority.owner_receipt_primary_state_mismatch);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingHistoricalOwnerCausalityClaim: none");
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingProven: {}", canonical_primary_beam_authority.canonical_authority_bindings_proven);
    let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingUnresolved: {}", canonical_primary_beam_authority.canonical_authority_bindings_unresolved);
    let _ = writeln!(
        out,
        "CanonicalPrimaryBeamAuthorityBindingVerdict: {}",
        if !canonical_primary_beam_authority.render_parse_ok {
            "UNAVAILABLE_RENDER_PARSE_FAILURE"
        } else if canonical_primary_beam_authority.canonical_authority_bindings_unresolved == 0
            && canonical_primary_beam_authority.cross_voice_observations > 0
        {
            "CANONICAL_PRIMARY_BEAM_AUTHORITY_BOUND_FOR_ALL_CROSS_VOICE_OBSERVATIONS"
        } else if canonical_primary_beam_authority.pre_final_chord_exact_encoding_witnesses > 0 {
            "AUTHORITY_PROVENANCE_RECEIPT_REQUIRED_ENCODING_CONTINUITY_ALONE_IS_NOT_AUTHORITY"
        } else {
            "AUTHORITY_PROVENANCE_AND_PRE_FINAL_ENCODING_EVIDENCE_UNRESOLVED"
        }
    );
    if canonical_primary_beam_authority.rows.is_empty() {
        let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingRows: none");
    } else {
        for row in canonical_primary_beam_authority.rows {
            let _ = writeln!(out, "CanonicalPrimaryBeamAuthorityBindingRow: {row}");
        }
    }

    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryAudit: read_only");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryAuthority: none");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryRuleChange: nein");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryProductiveSelector: none");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometrySource: final_musescore_pdf_vector_paths");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryXmlSubstitution: forbidden");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryOcr: none");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometrySecondRendererRun: none");
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryPdfReadback: {}", if renderer_primary_beam_geometry.pdf_readback_ok { "ok" } else { "unavailable" });
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryMeasureCount: {}", renderer_primary_beam_geometry.measure_count);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryCrossVoiceObservations: {}", renderer_primary_beam_geometry.cross_voice_observations);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryOccurrenceIdentityBound: {}", renderer_primary_beam_geometry.occurrence_identity_bound);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryOccurrenceIdentityUnresolved: {}", renderer_primary_beam_geometry.occurrence_identity_unresolved);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryBound: {}", renderer_primary_beam_geometry.renderer_geometry_bound);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryContinuityPreserved: {}", renderer_primary_beam_geometry.continuity_preserved);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryContinuityFragmented: {}", renderer_primary_beam_geometry.continuity_fragmented);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryUnavailable: {}", renderer_primary_beam_geometry.geometry_unavailable);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryBoundMeasureStaffCells: {}", renderer_primary_beam_geometry.bound_measure_staff_cells);
    let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryUnresolvedMeasureStaffCells: {}", renderer_primary_beam_geometry.unresolved_measure_staff_cells);
    let _ = writeln!(
        out,
        "RendererVisiblePrimaryBeamGeometryVerdict: {}",
        if !renderer_primary_beam_geometry.pdf_readback_ok {
            "UNAVAILABLE_PDF_VECTOR_READBACK"
        } else if renderer_primary_beam_geometry.geometry_unavailable == 0
            && renderer_primary_beam_geometry.cross_voice_observations > 0
        {
            "EXACT_VECTOR_GEOMETRY_BOUND_FOR_ALL_CROSS_VOICE_OBSERVATIONS"
        } else if renderer_primary_beam_geometry.renderer_geometry_bound > 0 {
            "PARTIAL_VECTOR_GEOMETRY_AVAILABLE_FAIL_CLOSED"
        } else {
            "UNAVAILABLE_EXACT_VECTOR_BINDING"
        }
    );
    if renderer_primary_beam_geometry.rows.is_empty() {
        let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryRows: none");
    } else {
        for row in &renderer_primary_beam_geometry.rows {
            let _ = writeln!(out, "RendererVisiblePrimaryBeamGeometryRow: {row}");
        }
    }

    let frozen_class_verdict = if !frozen_class.render_parse_ok {
        "UNAVAILABLE_PARSE_FAILURE"
    } else if frozen_class.persisted_render_occurrence_identity_unresolved > 0 {
        "CLASSIFICATION_BLOCKED_MISSING_EXACT_PERSISTED_OCCURRENCE_IDENTITY"
    } else if renderer_primary_beam_geometry.geometry_unavailable > 0 {
        "CLASSIFICATION_BLOCKED_MISSING_RENDERER_BEAM_GEOMETRY"
    } else if canonical_primary_beam_authority.canonical_authority_bindings_unresolved > 0 {
        "CLASSIFICATION_BLOCKED_MISSING_CANONICAL_PRIMARY_BEAM_AUTHORITY"
    } else if frozen_class.class_members_unresolved > 0 {
        "CLASSIFICATION_BLOCKED_UNRESOLVED_FROZEN_CLASS_EVIDENCE"
    } else if frozen_class.class_members_proven > 0 {
        "CLASSIFICATION_PROVEN_RENDERER_VISIBLE_PRIMARY_BEAM_FRAGMENTATION"
    } else {
        "NO_RENDERER_VISIBLE_PRIMARY_BEAM_FRAGMENTATION_OBSERVED"
    };
    let _ = writeln!(out, "FrozenErrorClassContract: docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition.md");
    let _ = writeln!(out, "FrozenErrorClassDefinitionModified: no");
    let _ = writeln!(out, "FrozenErrorClassClassificationPurpose: measurement_only");
    let _ = writeln!(out, "FrozenErrorClassRepairAuthority: none");
    let _ = writeln!(out, "FrozenErrorClassNonCausalContextNarrowing: none");
    let _ = writeln!(out, "FrozenErrorClassExactClosedSameLineMetricBindings: {}", frozen_class.exact_closed_same_line_metric_bindings);
    let _ = writeln!(out, "FrozenErrorClassExactClosedSameLineRenderStructuralBindings: {}", frozen_class.exact_closed_same_line_render_structural_bindings);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderCrossVoiceStructuralObservations: {}", frozen_class.persisted_render_cross_voice_structural_observations);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderCrossVoiceSingleBeamFragmentObservations: {}", frozen_class.persisted_render_cross_voice_single_beam_fragment_observations);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderCrossVoiceMultipleBeamFragmentObservations: {}", frozen_class.persisted_render_cross_voice_multiple_beam_fragment_observations);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderOccurrenceIdentityClosure: legacy_exact_locator_then_exact_source_attack_bridge_then_unique_local_source_attack_geometry_bijection_fail_closed");
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderOccurrenceIdentityLegacyExactBindings: {}", frozen_class.persisted_render_occurrence_identity_legacy_exact_bindings);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderOccurrenceIdentitySourceAttackBridgeBindings: {}", frozen_class.persisted_render_occurrence_identity_source_attack_bridge_bindings);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderOccurrenceIdentityLocalBijectionBindings: {}", frozen_class.persisted_render_occurrence_identity_local_bijection_bindings);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderOccurrenceIdentityExactBindings: {}", frozen_class.persisted_render_occurrence_identity_exact_bindings);
    let _ = writeln!(out, "FrozenErrorClassPersistedRenderOccurrenceIdentityUnresolved: {}", frozen_class.persisted_render_occurrence_identity_unresolved);
    let _ = writeln!(
        out,
        "FrozenErrorClassPersistedRenderOccurrenceIdentity: {}",
        if frozen_class.persisted_render_cross_voice_structural_observations > 0
            && frozen_class.persisted_render_occurrence_identity_unresolved == 0
        {
            "available_exact_for_all_cross_voice_observations"
        } else if frozen_class.persisted_render_occurrence_identity_exact_bindings > 0 {
            "partially_available_fail_closed"
        } else {
            "unavailable"
        }
    );
    let _ = writeln!(
        out,
        "FrozenErrorClassCanonicalPrimaryBeamAuthorityBinding: {}",
        if canonical_primary_beam_authority.cross_voice_observations > 0
            && canonical_primary_beam_authority.canonical_authority_bindings_unresolved == 0
        {
            "available_current_canonical_owner_receipt_bound_to_all_cross_voice_occurrences"
        } else if canonical_primary_beam_authority.canonical_authority_bindings_proven > 0 {
            "partially_available_fail_closed"
        } else {
            "unavailable_current_projection_has_encoding_witness_not_historical_owner"
        }
    );
    let _ = writeln!(
        out,
        "FrozenErrorClassRendererVisiblePrimaryBeamGeometryReadback: {}",
        if renderer_primary_beam_geometry.cross_voice_observations > 0
            && renderer_primary_beam_geometry.geometry_unavailable == 0
        {
            "available_exact_vector_readback_all_exact_occurrences"
        } else if renderer_primary_beam_geometry.renderer_geometry_bound > 0 {
            "partially_available_fail_closed"
        } else {
            "unavailable"
        }
    );
    let _ = writeln!(out, "FrozenErrorClassMembersProven: {}", frozen_class.class_members_proven);
    let _ = writeln!(out, "FrozenErrorClassMembersUnresolved: {}", frozen_class.class_members_unresolved);
    let _ = writeln!(out, "FrozenErrorClassVerdict: {frozen_class_verdict}");
    if frozen_class.rows.is_empty() {
        let _ = writeln!(out, "FrozenErrorClassRows: none");
    } else {
        for row in frozen_class.rows {
            let _ = writeln!(out, "FrozenErrorClassRow: {row}");
        }
    }

    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeAudit: read_only"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeAuthority: none"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeMusicMutation: nein"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeRuleChange: ja_repair_scope_definition_only"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeParentErrorClassContract: docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition.md"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeSelection: proven_frozen_class_member_intersection_materialization_safety_conjunction_proven"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeMetricLineIdRole: opaque_join_key_only_not_causal_feature"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeNonMemberPolicy: renderer_continuity_preserved_is_never_selected"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeUnresolvedPolicy: fail_closed_no_selection"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeClassMembersProven: {}",
        repair_scope.class_members_proven
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeClassMembersUnresolved: {}",
        repair_scope.class_members_unresolved
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeMaterializationSafetyProvenCandidates: {}",
        repair_scope.materialization_safety_proven_candidates
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeEligible: {}",
        repair_scope.repair_scope_eligible
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeBlockedMaterializationSafety: {}",
        repair_scope.blocked_materialization_safety
    );
    let repair_scope_verdict = if repair_scope.class_members_proven == 0 {
        "NO_PROVEN_FROZEN_CLASS_MEMBER"
    } else if repair_scope.repair_scope_eligible == 0 {
        "NO_SAFETY_CLOSED_REPAIR_SCOPE"
    } else if repair_scope.blocked_materialization_safety == 0 {
        "REPAIR_SCOPE_PROVEN_FOR_ALL_PROVEN_CLASS_MEMBERS_ANALYSIS_ONLY"
    } else {
        "REPAIR_SCOPE_PROVEN_FOR_SAFETY_SUBSET_BLOCKED_CASES_REMAIN"
    };
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionRepairScopeVerdict: {repair_scope_verdict}"
    );
    if repair_scope.rows.is_empty() {
        let _ = writeln!(out, "PrimaryBeamSerializationVoicePartitionRepairScopeRows: none");
    } else {
        for row in repair_scope.rows {
            let _ = writeln!(
                out,
                "PrimaryBeamSerializationVoicePartitionRepairScopeRow: {row}"
            );
        }
    }

    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAudit: read_only"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAuthority: none"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayMusicMutation: nein"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayRuleChange: nein_existing_r1061_scope_consumed_only"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayInputScope: exact_r1061_repair_scope_eligible_metric_line_ids"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayTargetAuthority: existing_primary_channel_anchor_from_materialization_safety_conjunction"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayPhysicalBinding: source_attack_to_canonical_occurrence_to_persisted_render_voice_scalar"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayWriter: none_dry_run_only"
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayRenderParseOk: {}",
        if late_rebind_replay.render_parse_ok { "yes" } else { "no" }
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayEligibleMetricLines: {}",
        late_rebind_replay.eligible_metric_lines
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayMetricCandidateBindings: {}",
        late_rebind_replay.metric_candidate_bindings
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayTargetEvidenceBindings: {}",
        late_rebind_replay.target_evidence_bindings
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayTargetEvidenceUnresolved: {}",
        late_rebind_replay.target_evidence_unresolved
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayMemberOccurrencesBound: {}",
        late_rebind_replay.member_occurrences_bound
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayMemberOccurrencesUnresolved: {}",
        late_rebind_replay.member_occurrences_unresolved
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayExistingTargetVoiceBindings: {}",
        late_rebind_replay.existing_target_voice_bindings
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayProposedVoiceScalarChanges: {}",
        late_rebind_replay.proposed_voice_scalar_changes
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayAlreadyTargetVoiceScalars: {}",
        late_rebind_replay.already_target_voice_scalars
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplaySourceOverlapConflicts: {}",
        late_rebind_replay.source_overlap_conflicts
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayPhysicalEditFootprintConflicts: {}",
        late_rebind_replay.physical_edit_footprint_conflicts
    );
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayReadyMetricLines: {}",
        late_rebind_replay.replay_ready_metric_lines
    );
    let late_rebind_replay_verdict = if late_rebind_replay.eligible_metric_lines == 0 {
        "NO_REPAIR_SCOPE_ELIGIBLE_METRIC_LINES"
    } else if late_rebind_replay.render_parse_ok
        && late_rebind_replay.metric_candidate_bindings == late_rebind_replay.eligible_metric_lines
        && late_rebind_replay.target_evidence_bindings == late_rebind_replay.eligible_metric_lines
        && late_rebind_replay.target_evidence_unresolved == 0
        && late_rebind_replay.member_occurrences_unresolved == 0
        && late_rebind_replay.existing_target_voice_bindings
            == late_rebind_replay.eligible_metric_lines
        && late_rebind_replay.source_overlap_conflicts == 0
        && late_rebind_replay.physical_edit_footprint_conflicts == 0
        && late_rebind_replay.replay_ready_metric_lines == late_rebind_replay.eligible_metric_lines
    {
        "LATE_REBIND_EVIDENCE_REPLAY_PROVEN_FOR_COMPLETE_R1061_SCOPE_ANALYSIS_ONLY"
    } else {
        "LATE_REBIND_EVIDENCE_REPLAY_BLOCKED_FAIL_CLOSED"
    };
    let _ = writeln!(
        out,
        "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayVerdict: {late_rebind_replay_verdict}"
    );
    if late_rebind_replay.rows.is_empty() {
        let _ = writeln!(
            out,
            "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayRows: none"
        );
    } else {
        for row in late_rebind_replay.rows {
            let _ = writeln!(
                out,
                "PrimaryBeamSerializationVoicePartitionLateRebindEvidenceReplayRow: {row}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit_only::notation_voice_pre_serialization_constraint_closure_audit::{
        VisibleRhythmicOccupancyDuplicateRestConflict,
        VisibleRhythmicOccupancyRestPitchedConflict,
    };

    fn score(body: &str) -> String {
        format!(
            "<score-partwise><part id=\"P1\"><measure number=\"1\"><attributes><divisions>24</divisions></attributes>{body}</measure></part></score-partwise>"
        )
    }

    fn cross_phase_member(
        source_event_id: usize,
        attack_id: usize,
        pitched_ordinal: usize,
        onset: i64,
    ) -> super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
        super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id,
            attack_id,
            measure: 1,
            pitched_ordinal,
            represented_onset: onset,
            represented_duration: 12,
            staff: 1,
        }
    }

    fn cross_phase_s0_occurrence(
        source_event_id: usize,
        attack_id: usize,
        pitched_ordinal: usize,
        onset: i64,
    ) -> PreSerializationOccurrenceRow {
        PreSerializationOccurrenceRow {
            source_event_id,
            attack_id,
            measure: 1,
            pitched_ordinal,
            represented_onset: onset,
            represented_duration: 12,
            pitch_midi: if source_event_id == 1 { 60 } else { 62 },
            staff: 1,
            current_voice: "1".to_string(),
        }
    }

    fn cross_phase_canonical_occurrence(
        source_event_id: usize,
        attack_id: usize,
        pitched_ordinal: usize,
    ) -> NotationVoiceCanonicalOccurrenceIdentityRow {
        NotationVoiceCanonicalOccurrenceIdentityRow {
            source_event_id,
            attack_id,
            measure: 1,
            pitched_ordinal,
            materialization_kind_rank: 0,
            fragment_ordinal: 0,
        }
    }

    fn cross_phase_metric_and_closed(
    ) -> (
        PreSerializationMetricNotationLineCandidate,
        PreSerializationClosedSameLineCandidate,
    ) {
        let members = vec![
            cross_phase_member(1, 101, 0, 0),
            cross_phase_member(2, 102, 1, 12),
        ];
        (
            PreSerializationMetricNotationLineCandidate {
                metric_line_id: "metric-cross-phase".to_string(),
                source_event_ids: vec![1, 2],
                members: members.clone(),
                measure: 1,
                staff: 1,
                beat: 0,
                interval_start: 0,
                interval_end: 24,
            },
            PreSerializationClosedSameLineCandidate {
                proof_chain_id: "1:101-2:102".to_string(),
                source_event_ids: vec![1, 2],
                members,
                staff: 1,
            },
        )
    }

    fn cross_phase_render() -> String {
        score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note>"
        ))
    }

    fn cross_phase_render_with_duplicate_second_source() -> String {
        score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff></note>"
        ))
    }

    #[test]
    fn current_canonical_owner_receipt_transport_binds_exact_cross_voice_cohort() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let pre_final = BTreeSet::from([closed.proof_chain_id.clone()]);
        let receipts = vec![CanonicalPrimaryBeamOwnerReceipt {
            proof_chain_id: closed.proof_chain_id.clone(),
            owner_kind: "structural_metric_beam_hierarchy_plan",
            owner_source_fnv64: 0xb304_f23e_30b1_6e79,
            event_count: 2,
            plan_fingerprint_fnv64: 7,
            primary_states: vec!["begin", "end"],
        }];
        let audit = build_canonical_primary_beam_owner_receipt_transport_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &pre_final,
            &receipts,
            &cross_phase_render(),
        );
        assert_eq!(audit.cross_voice_observations, 1);
        assert_eq!(audit.typed_current_canonical_owner_receipts, 1);
        assert_eq!(audit.receipt_transport_proven, 1);
        assert_eq!(audit.receipt_transport_unresolved, 0);
        assert!(audit.rows[0].contains("CURRENT_CANONICAL_OWNER_RECEIPT_TRANSPORT_PROVEN"));
    }

    #[test]
    fn current_canonical_owner_receipt_transport_fails_closed_on_event_count_mismatch() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let pre_final = BTreeSet::from([closed.proof_chain_id.clone()]);
        let receipts = vec![CanonicalPrimaryBeamOwnerReceipt {
            proof_chain_id: closed.proof_chain_id.clone(),
            owner_kind: "structural_metric_beam_hierarchy_plan",
            owner_source_fnv64: 0xb304_f23e_30b1_6e79,
            event_count: 3,
            plan_fingerprint_fnv64: 7,
            primary_states: vec!["begin", "continue", "end"],
        }];
        let audit = build_canonical_primary_beam_owner_receipt_transport_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &pre_final,
            &receipts,
            &cross_phase_render(),
        );
        assert_eq!(audit.receipt_event_count_mismatch, 1);
        assert_eq!(audit.receipt_transport_proven, 0);
        assert_eq!(audit.receipt_transport_unresolved, 1);
        assert!(audit.rows[0].contains("BLOCKED_OWNER_RECEIPT_EVENT_COUNT_MISMATCH"));
    }

    #[test]
    fn canonical_beam_authority_closure_binds_current_owner_receipt_and_persisted_primary_states() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let metric_line_id = metric.metric_line_id.clone();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let pre_final = BTreeSet::from([closed.proof_chain_id.clone()]);
        let receipts = vec![CanonicalPrimaryBeamOwnerReceipt {
            proof_chain_id: closed.proof_chain_id.clone(),
            owner_kind: "structural_metric_beam_hierarchy_plan",
            owner_source_fnv64: 0xd1bb5c6255558e59,
            event_count: 2,
            plan_fingerprint_fnv64: 7,
            primary_states: vec!["begin", "end"],
        }];
        let audit = build_canonical_primary_beam_authority_binding_audit_with_owner_receipts(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &pre_final,
            &receipts,
            &cross_phase_render(),
        );
        assert_eq!(audit.cross_voice_observations, 1);
        assert_eq!(audit.typed_owner_receipts_available, 1);
        assert_eq!(audit.owner_receipt_cardinality_unresolved, 0);
        assert_eq!(audit.owner_receipt_event_count_mismatch, 0);
        assert_eq!(audit.owner_receipt_primary_state_mismatch, 0);
        assert_eq!(audit.canonical_authority_bindings_proven, 1);
        assert_eq!(audit.canonical_authority_bindings_unresolved, 0);
        assert!(audit.proven_metric_line_ids.contains(&metric_line_id));
        assert!(audit.rows[0].contains("CANONICAL_PRIMARY_BEAM_AUTHORITY_BOUND_CURRENT_OWNER"));
        assert!(audit.rows[0].contains("historical_owner_causality=not_claimed"));
    }

    #[test]
    fn canonical_beam_authority_closure_fails_closed_on_primary_state_mismatch() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let pre_final = BTreeSet::from([closed.proof_chain_id.clone()]);
        let receipts = vec![CanonicalPrimaryBeamOwnerReceipt {
            proof_chain_id: closed.proof_chain_id.clone(),
            owner_kind: "structural_metric_beam_hierarchy_plan",
            owner_source_fnv64: 0xd1bb5c6255558e59,
            event_count: 2,
            plan_fingerprint_fnv64: 7,
            primary_states: vec!["begin", "continue"],
        }];
        let audit = build_canonical_primary_beam_authority_binding_audit_with_owner_receipts(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &pre_final,
            &receipts,
            &cross_phase_render(),
        );
        assert_eq!(audit.typed_owner_receipts_available, 1);
        assert_eq!(audit.owner_receipt_primary_state_mismatch, 1);
        assert_eq!(audit.canonical_authority_bindings_proven, 0);
        assert_eq!(audit.canonical_authority_bindings_unresolved, 1);
        assert!(audit.proven_metric_line_ids.is_empty());
        assert!(audit.rows[0].contains("BLOCKED_CURRENT_CANONICAL_OWNER_RECEIPT_PRIMARY_STATE_MISMATCH"));
    }

    #[test]
    fn canonical_beam_authority_audit_keeps_exact_pre_final_encoding_fail_closed_without_owner_receipt() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let pre_final = BTreeSet::from([closed.proof_chain_id.clone()]);
        let audit = build_canonical_primary_beam_authority_binding_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &pre_final,
            &cross_phase_render(),
        );
        assert_eq!(audit.cross_voice_observations, 1);
        assert_eq!(audit.occurrence_identity_bound, 1);
        assert_eq!(audit.pre_final_chord_exact_encoding_witnesses, 1);
        assert_eq!(audit.typed_owner_receipts_available, 0);
        assert_eq!(audit.canonical_authority_bindings_proven, 0);
        assert_eq!(audit.canonical_authority_bindings_unresolved, 1);
        assert!(audit.rows[0].contains("BLOCKED_CURRENT_CANONICAL_OWNER_RECEIPT_CARDINALITY"));
    }

    #[test]
    fn canonical_beam_authority_audit_reports_missing_pre_final_witness_without_inventing_authority() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let audit = build_canonical_primary_beam_authority_binding_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &BTreeSet::new(),
            &cross_phase_render(),
        );
        assert_eq!(audit.pre_final_chord_exact_encoding_witnesses, 0);
        assert_eq!(audit.pre_final_chord_encoding_witness_missing, 1);
        assert_eq!(audit.canonical_authority_bindings_proven, 0);
        assert_eq!(audit.canonical_authority_bindings_unresolved, 1);
        assert!(audit.rows[0].contains("BLOCKED_NO_PRE_FINAL_ENCODING_WITNESS"));
    }

    #[test]
    fn canonical_beam_authority_audit_requires_occurrence_identity_before_owner_binding() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let pre_final = BTreeSet::from([closed.proof_chain_id.clone()]);
        let audit = build_canonical_primary_beam_authority_binding_audit(
            &[metric],
            &[closed],
            &[],
            &[],
            &pre_final,
            &cross_phase_render(),
        );
        assert_eq!(audit.cross_voice_observations, 1);
        assert_eq!(audit.occurrence_identity_bound, 0);
        assert_eq!(audit.occurrence_identity_unresolved, 1);
        assert_eq!(audit.pre_final_chord_exact_encoding_witnesses, 0);
        assert_eq!(audit.canonical_authority_bindings_proven, 0);
        assert_eq!(audit.canonical_authority_bindings_unresolved, 1);
        assert!(audit.rows[0].contains("BLOCKED_OCCURRENCE_IDENTITY_BEFORE_BEAM_AUTHORITY_BINDING"));
    }

    #[test]
    fn local_bijection_recovers_repeated_source_attack_by_exact_geometry() {
        let (mut metric, mut closed) = cross_phase_metric_and_closed();
        metric.members[0].pitched_ordinal = 7;
        metric.members[1].pitched_ordinal = 8;
        closed.members[0].pitched_ordinal = 7;
        closed.members[1].pitched_ordinal = 8;
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 7, 0),
            cross_phase_s0_occurrence(2, 102, 8, 12),
            cross_phase_s0_occurrence(2, 102, 9, 24),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
            cross_phase_canonical_occurrence(2, 102, 2),
        ];
        let audit = build_beam_occurrence_source_attack_multiplicity_local_bijection_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &cross_phase_render_with_duplicate_second_source(),
        );
        assert_eq!(audit.blocked_cross_voice_observations, 1);
        assert_eq!(audit.local_identity_groups, 1);
        assert_eq!(audit.proven_unique_local_bijections, 1);
        assert_eq!(audit.observations_proven_after_local_bijection, 1);
        assert_eq!(audit.observations_unresolved_after_local_bijection, 0);
    }

    #[test]
    fn local_bijection_counter_reports_one_perfect_matching() {
        let (count, assignment) = count_local_perfect_bijections(&[vec![0], vec![1]]);
        assert_eq!(count, 1);
        assert_eq!(assignment, Some(vec![0, 1]));
    }

    #[test]
    fn local_bijection_counter_keeps_ambiguous_matching_fail_closed() {
        let (count, assignment) =
            count_local_perfect_bijections(&[vec![0, 1], vec![0, 1]]);
        assert_eq!(count, 2);
        assert_eq!(assignment, None);
    }

    #[test]
    fn local_bijection_counter_keeps_missing_matching_fail_closed() {
        let (count, assignment) = count_local_perfect_bijections(&[vec![0], Vec::new()]);
        assert_eq!(count, 0);
        assert_eq!(assignment, None);
    }

    #[test]
    fn occurrence_identity_closure_promotes_unique_source_attack_bridge() {
        let (mut metric, mut closed) = cross_phase_metric_and_closed();
        metric.members[0].pitched_ordinal = 7;
        metric.members[1].pitched_ordinal = 8;
        closed.members[0].pitched_ordinal = 7;
        closed.members[1].pitched_ordinal = 8;
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 7, 0),
            cross_phase_s0_occurrence(2, 102, 8, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let classification =
            build_frozen_primary_beam_serialization_voice_partition_classification(
                &[metric],
                &[closed],
                &s0,
                &canonical,
                &cross_phase_render(),
            );
        assert_eq!(
            classification.persisted_render_occurrence_identity_legacy_exact_bindings,
            0
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_source_attack_bridge_bindings,
            1
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_local_bijection_bindings,
            0
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_exact_bindings,
            1
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_unresolved,
            0
        );
    }

    #[test]
    fn occurrence_identity_closure_promotes_unique_local_bijection() {
        let (mut metric, mut closed) = cross_phase_metric_and_closed();
        metric.members[0].pitched_ordinal = 7;
        metric.members[1].pitched_ordinal = 8;
        closed.members[0].pitched_ordinal = 7;
        closed.members[1].pitched_ordinal = 8;
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 7, 0),
            cross_phase_s0_occurrence(2, 102, 8, 12),
            cross_phase_s0_occurrence(2, 102, 9, 24),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
            cross_phase_canonical_occurrence(2, 102, 2),
        ];
        let classification =
            build_frozen_primary_beam_serialization_voice_partition_classification(
                &[metric],
                &[closed],
                &s0,
                &canonical,
                &cross_phase_render_with_duplicate_second_source(),
            );
        assert_eq!(
            classification.persisted_render_occurrence_identity_legacy_exact_bindings,
            0
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_source_attack_bridge_bindings,
            0
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_local_bijection_bindings,
            1
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_exact_bindings,
            1
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_unresolved,
            0
        );
    }

    #[test]
    fn occurrence_identity_closure_keeps_ambiguous_local_bijection_fail_closed() {
        let (mut metric, _) = cross_phase_metric_and_closed();
        metric.members[0].pitched_ordinal = 7;
        metric.members[1].pitched_ordinal = 8;
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 7, 0),
            cross_phase_s0_occurrence(2, 102, 8, 12),
            cross_phase_s0_occurrence(2, 102, 9, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
            cross_phase_canonical_occurrence(2, 102, 2),
        ];
        let render_events = vec![
            RenderMetricEvent {
                measure: 1,
                staff: 1,
                onset: 0,
                duration: 12,
                pitch_midi: Some(60),
                visible: true,
                is_rest: false,
                is_chord: false,
                voice: "1".to_string(),
                beam1: Some("begin".to_string()),
                pitched_ordinal: Some(0),
            },
            RenderMetricEvent {
                measure: 1,
                staff: 1,
                onset: 12,
                duration: 12,
                pitch_midi: Some(62),
                visible: true,
                is_rest: false,
                is_chord: false,
                voice: "3".to_string(),
                beam1: Some("end".to_string()),
                pitched_ordinal: Some(1),
            },
            RenderMetricEvent {
                measure: 1,
                staff: 1,
                onset: 12,
                duration: 12,
                pitch_midi: Some(62),
                visible: true,
                is_rest: false,
                is_chord: false,
                voice: "1".to_string(),
                beam1: None,
                pitched_ordinal: Some(2),
            },
        ];
        let selected = vec![&render_events[0], &render_events[1]];
        assert_eq!(
            beam_occurrence_identity_closure_binding(
                &metric,
                &selected,
                &s0,
                &canonical,
                &render_events,
            ),
            None
        );
    }

    #[test]
    fn cross_phase_identity_keeps_legacy_exact_locator_binding_separate() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let audit = build_beam_occurrence_cross_phase_identity_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &cross_phase_render(),
        );
        assert_eq!(audit.cross_voice_observations, 1);
        assert_eq!(audit.legacy_exact_locator_bindings, 1);
        assert_eq!(audit.unresolved_after_source_attack_bridge, 0);
    }

    #[test]
    fn cross_phase_identity_recovers_unique_source_attack_locator_shift() {
        let (mut metric, mut closed) = cross_phase_metric_and_closed();
        metric.members[0].pitched_ordinal = 7;
        metric.members[1].pitched_ordinal = 8;
        closed.members[0].pitched_ordinal = 7;
        closed.members[1].pitched_ordinal = 8;
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 7, 0),
            cross_phase_s0_occurrence(2, 102, 8, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let audit = build_beam_occurrence_cross_phase_identity_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &cross_phase_render(),
        );
        assert_eq!(audit.legacy_exact_locator_bindings, 0);
        assert_eq!(audit.source_attack_bridge_exact_locator_shift, 1);
        assert_eq!(audit.unresolved_after_source_attack_bridge, 0);
    }

    #[test]
    fn cross_phase_identity_keeps_s0_source_attack_multiplicity_fail_closed() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(1, 101, 4, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 1),
            cross_phase_canonical_occurrence(2, 102, 0),
        ];
        let audit = build_beam_occurrence_cross_phase_identity_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &cross_phase_render(),
        );
        assert_eq!(audit.s0_source_attack_multiplicity, 1);
        assert_eq!(audit.unresolved_after_source_attack_bridge, 1);
    }

    #[test]
    fn cross_phase_identity_rejects_source_attack_bridge_geometry_swap() {
        let (metric, closed) = cross_phase_metric_and_closed();
        let s0 = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical = vec![
            cross_phase_canonical_occurrence(1, 101, 1),
            cross_phase_canonical_occurrence(2, 102, 0),
        ];
        let audit = build_beam_occurrence_cross_phase_identity_audit(
            &[metric],
            &[closed],
            &s0,
            &canonical,
            &cross_phase_render(),
        );
        assert_eq!(audit.canonical_to_s1_geometry_mismatch, 1);
        assert_eq!(audit.unresolved_after_source_attack_bridge, 1);
    }

    #[test]
    fn phase_delta_detects_s1_new_duplicate_rest_geometry_voice_free() {
        let s0 = VisibleRhythmicOccupancyPhaseSnapshot::default();
        let render = score(concat!(
            "<note><rest/><duration>24</duration><voice>1</voice><staff>1</staff></note>",
            "<backup><duration>24</duration></backup>",
            "<note><rest/><duration>24</duration><voice>7</voice><staff>1</staff></note>"
        ));
        let audit = build_phase_delta_audit(&s0, &[], &render);
        assert_eq!(audit.s1_new_duplicate_rest_conflict_geometries, 1);
        assert!(
            audit
                .rows
                .iter()
                .any(|row| row.starts_with("S1_NEW_DUPLICATE_REST_CONFLICT_GEOMETRY"))
        );
    }

    #[test]
    fn phase_delta_separates_shared_and_s1_new_rest_pitch_geometry() {
        let preserved = VisibleRhythmicOccupancyRestPitchedConflict {
            measure: 1,
            staff: 1,
            rest_onset: 0,
            rest_duration: 24,
            pitched_onset: 12,
            pitched_duration: 12,
            pitches: vec![60],
        };
        let mut s0 = VisibleRhythmicOccupancyPhaseSnapshot::default();
        s0.rest_pitched_conflicts.insert(preserved);
        let render = score(concat!(
            "<note><rest/><duration>24</duration><voice>1</voice><staff>1</staff></note>",
            "<backup><duration>24</duration></backup><forward><duration>12</duration></forward>",
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>9</voice><staff>1</staff></note>"
        ));
        let audit = build_phase_delta_audit(&s0, &[], &render);
        assert_eq!(audit.shared_rest_pitched_conflict_geometries, 1);
        assert_eq!(audit.s1_new_rest_pitched_conflict_geometries_at_s0_hidden_rest_signature, 0);
    }

    #[test]
    fn s1_new_rest_pitch_geometry_can_be_related_to_s0_hidden_rest_signature() {
        let mut s0 = VisibleRhythmicOccupancyPhaseSnapshot::default();
        s0.hidden_atoms.push(VisibleRhythmicOccupancyPhaseAtom {
            measure: 1,
            staff: 2,
            onset: 0,
            duration: 24,
            kind: VisibleRhythmicOccupancyAtomKind::Rest,
            pitches: Vec::new(),
        });
        let render = score(concat!(
            "<note><rest/><duration>24</duration><voice>2</voice><staff>2</staff></note>",
            "<backup><duration>24</duration></backup>",
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>8</voice><staff>2</staff></note>"
        ));
        let audit = build_phase_delta_audit(&s0, &[], &render);
        assert_eq!(audit.s1_new_rest_pitched_conflict_geometries_at_s0_hidden_rest_signature, 1);
        assert_eq!(audit.s1_unbound_rest_pitched_conflicts, 0);
    }

    #[test]
    fn unbound_s1_rest_pitch_is_diagnostic_not_event_provenance() {
        let s0 = VisibleRhythmicOccupancyPhaseSnapshot::default();
        let render = score(concat!(
            "<note><rest/><duration>24</duration><voice>2</voice><staff>2</staff></note>",
            "<backup><duration>24</duration></backup>",
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>8</voice><staff>2</staff></note>"
        ));
        let audit = build_phase_delta_audit(&s0, &[], &render);
        assert_eq!(audit.s1_new_rest_pitched_conflict_geometries_at_s0_hidden_rest_signature, 0);
        assert_eq!(audit.s1_unbound_rest_pitched_conflicts, 1);
    }

    #[test]
    fn phase_delta_detects_s1_visible_at_s0_hidden_signature_without_event_identity() {
        let mut s0 = VisibleRhythmicOccupancyPhaseSnapshot::default();
        s0.hidden_atoms.push(VisibleRhythmicOccupancyPhaseAtom {
            measure: 1,
            staff: 2,
            onset: 0,
            duration: 24,
            kind: VisibleRhythmicOccupancyAtomKind::Rest,
            pitches: Vec::new(),
        });
        let render = score(
            "<note><rest/><duration>24</duration><voice>4</voice><staff>2</staff></note>",
        );
        let audit = build_phase_delta_audit(&s0, &[], &render);
        assert_eq!(audit.s1_visible_at_s0_hidden_signatures, 1);
    }

    #[test]
    fn metric_line_is_bound_without_using_voice_or_beam_as_selector() {
        let s0 = VisibleRhythmicOccupancyPhaseSnapshot::default();
        let candidate = PreSerializationMetricNotationLineCandidate {
            metric_line_id: "metric-line-test".to_string(),
            source_event_ids: vec![1, 2],
            members: vec![
                super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
                    source_event_id: 1,
                    attack_id: 1,
                    measure: 1,
                    pitched_ordinal: 0,
                    represented_onset: 0,
                    represented_duration: 12,
                    staff: 1,
                },
                super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
                    source_event_id: 2,
                    attack_id: 2,
                    measure: 1,
                    pitched_ordinal: 1,
                    represented_onset: 12,
                    represented_duration: 12,
                    staff: 1,
                },
            ],
            measure: 1,
            staff: 1,
            beat: 0,
            interval_start: 0,
            interval_end: 24,
        };
        let render = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note>"
        ));
        let audit = build_phase_delta_audit(&s0, &[candidate], &render);
        assert_eq!(audit.metric_lines_exact_render_binding, 1);
        assert_eq!(audit.metric_lines_cross_voice, 1);
        assert_eq!(audit.metric_lines_multiple_beam_fragments, 0);
    }

    #[test]
    fn incomplete_orphan_chord_remains_blocked_in_s0() {
        let mut s0 = VisibleRhythmicOccupancyPhaseSnapshot::default();
        s0.incomplete_groups.push(
            "VISIBLE_RHYTHMIC_OCCUPANCY_INCOMPLETE group=7 measure=30 staff=1 reason=pitched_atom_not_anchor_closed visible_pitched_members=2 visible_anchors=0 orphan_chord=yes".to_string(),
        );
        let audit = build_phase_delta_audit(&s0, &[], &score(""));
        assert_eq!(audit.blocked_incomplete_chord_bindings, 1);
        assert!(audit.rows.iter().any(|row| row.starts_with("BLOCKED_INCOMPLETE_CHORD_BINDING")));
    }

    #[test]
    fn voice_relabeling_cannot_change_voice_free_occupancy_or_conflict_geometry() {
        let separated = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>",
            "<backup><duration>24</duration></backup>",
            "<note><rest/><duration>24</duration><voice>2</voice><staff>1</staff></note>",
            "<backup><duration>24</duration></backup>",
            "<note><rest/><duration>24</duration><voice>3</voice><staff>1</staff></note>"
        ));
        let merged = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>9</voice><staff>1</staff></note>",
            "<backup><duration>24</duration></backup>",
            "<note><rest/><duration>24</duration><voice>9</voice><staff>1</staff></note>",
            "<backup><duration>24</duration></backup>",
            "<note><rest/><duration>24</duration><voice>9</voice><staff>1</staff></note>"
        ));

        let (separated_rows, _) = parse_render_phase(&separated).expect("separated parse");
        let (merged_rows, _) = parse_render_phase(&merged).expect("merged parse");
        let separated_snapshot =
            build_visible_rhythmic_occupancy_phase_snapshot(&separated, &separated_rows);
        let merged_snapshot =
            build_visible_rhythmic_occupancy_phase_snapshot(&merged, &merged_rows);

        assert_eq!(separated_snapshot, merged_snapshot);
        assert_eq!(separated_snapshot.duplicate_rest_conflicts.len(), 1);
        assert_eq!(separated_snapshot.rest_pitched_conflicts.len(), 1);
    }

    #[test]
    fn frozen_class_cross_voice_structural_observation_remains_unresolved_without_renderer_geometry() {
        let member_a = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 1,
            attack_id: 1,
            measure: 1,
            pitched_ordinal: 0,
            represented_onset: 0,
            represented_duration: 12,
            staff: 1,
        };
        let member_b = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 2,
            attack_id: 2,
            measure: 1,
            pitched_ordinal: 1,
            represented_onset: 12,
            represented_duration: 12,
            staff: 1,
        };
        let metric = PreSerializationMetricNotationLineCandidate {
            metric_line_id: "metric-frozen-class".to_string(),
            source_event_ids: vec![1, 2],
            members: vec![member_a.clone(), member_b.clone()],
            measure: 1,
            staff: 1,
            beat: 0,
            interval_start: 0,
            interval_end: 24,
        };
        let closed = PreSerializationClosedSameLineCandidate {
            proof_chain_id: "1:1-2:2".to_string(),
            source_event_ids: vec![1, 2],
            members: vec![member_a, member_b],
            staff: 1,
        };
        let render = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note>"
        ));
        let classification =
            build_frozen_primary_beam_serialization_voice_partition_classification(
                &[metric],
                &[closed],
                &[],
                &[],
                &render,
            );
        assert_eq!(classification.exact_closed_same_line_metric_bindings, 1);
        assert_eq!(
            classification.persisted_render_cross_voice_structural_observations,
            1
        );
        assert_eq!(
            classification.persisted_render_cross_voice_single_beam_fragment_observations,
            1
        );
        assert_eq!(classification.class_members_proven, 0);
        assert_eq!(classification.class_members_unresolved, 1);
    }

    #[test]
    fn frozen_class_rebinds_exact_persisted_occurrence_identity_by_physical_pitched_ordinal() {
        let member_a = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 1,
            attack_id: 101,
            measure: 1,
            pitched_ordinal: 0,
            represented_onset: 0,
            represented_duration: 12,
            staff: 1,
        };
        let member_b = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 3,
            attack_id: 103,
            measure: 1,
            pitched_ordinal: 2,
            represented_onset: 24,
            represented_duration: 12,
            staff: 1,
        };
        let metric = PreSerializationMetricNotationLineCandidate {
            metric_line_id: "metric-persisted-occurrence".to_string(),
            source_event_ids: vec![1, 3],
            members: vec![member_a.clone(), member_b.clone()],
            measure: 1,
            staff: 1,
            beat: 0,
            interval_start: 0,
            interval_end: 36,
        };
        let closed = PreSerializationClosedSameLineCandidate {
            proof_chain_id: "1:101-3:103".to_string(),
            source_event_ids: vec![1, 3],
            members: vec![member_a, member_b],
            staff: 1,
        };
        let occurrences = vec![
            NotationVoiceCanonicalOccurrenceIdentityRow {
                source_event_id: 1,
                attack_id: 101,
                measure: 1,
                pitched_ordinal: 0,
                materialization_kind_rank: 0,
                fragment_ordinal: 0,
            },
            NotationVoiceCanonicalOccurrenceIdentityRow {
                source_event_id: 2,
                attack_id: 102,
                measure: 1,
                pitched_ordinal: 1,
                materialization_kind_rank: 0,
                fragment_ordinal: 0,
            },
            NotationVoiceCanonicalOccurrenceIdentityRow {
                source_event_id: 3,
                attack_id: 103,
                measure: 1,
                pitched_ordinal: 2,
                materialization_kind_rank: 0,
                fragment_ordinal: 0,
            },
        ];
        let render = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note print-object=\"no\"><pitch><step>E</step><octave>4</octave></pitch><duration>12</duration><voice>2</voice><staff>1</staff></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note>"
        ));
        let classification =
            build_frozen_primary_beam_serialization_voice_partition_classification(
                &[metric],
                &[closed],
                &[],
                &occurrences,
                &render,
            );
        assert_eq!(
            classification.persisted_render_cross_voice_structural_observations,
            1
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_exact_bindings,
            1
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_unresolved,
            0
        );
        assert_eq!(classification.class_members_proven, 0);
        assert_eq!(classification.class_members_unresolved, 1);
    }

    #[test]
    fn frozen_class_duplicate_persisted_occurrence_identity_fails_closed() {
        let member_a = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 1,
            attack_id: 101,
            measure: 1,
            pitched_ordinal: 0,
            represented_onset: 0,
            represented_duration: 12,
            staff: 1,
        };
        let member_b = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 2,
            attack_id: 102,
            measure: 1,
            pitched_ordinal: 1,
            represented_onset: 12,
            represented_duration: 12,
            staff: 1,
        };
        let metric = PreSerializationMetricNotationLineCandidate {
            metric_line_id: "metric-duplicate-occurrence".to_string(),
            source_event_ids: vec![1, 2],
            members: vec![member_a.clone(), member_b.clone()],
            measure: 1,
            staff: 1,
            beat: 0,
            interval_start: 0,
            interval_end: 24,
        };
        let closed = PreSerializationClosedSameLineCandidate {
            proof_chain_id: "1:101-2:102".to_string(),
            source_event_ids: vec![1, 2],
            members: vec![member_a, member_b],
            staff: 1,
        };
        let occurrences = vec![
            NotationVoiceCanonicalOccurrenceIdentityRow {
                source_event_id: 1,
                attack_id: 101,
                measure: 1,
                pitched_ordinal: 0,
                materialization_kind_rank: 0,
                fragment_ordinal: 0,
            },
            NotationVoiceCanonicalOccurrenceIdentityRow {
                source_event_id: 99,
                attack_id: 999,
                measure: 1,
                pitched_ordinal: 0,
                materialization_kind_rank: 0,
                fragment_ordinal: 0,
            },
            NotationVoiceCanonicalOccurrenceIdentityRow {
                source_event_id: 2,
                attack_id: 102,
                measure: 1,
                pitched_ordinal: 1,
                materialization_kind_rank: 0,
                fragment_ordinal: 0,
            },
        ];
        let render = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note>"
        ));
        let classification =
            build_frozen_primary_beam_serialization_voice_partition_classification(
                &[metric],
                &[closed],
                &[],
                &occurrences,
                &render,
            );
        assert_eq!(
            classification.persisted_render_occurrence_identity_exact_bindings,
            0
        );
        assert_eq!(
            classification.persisted_render_occurrence_identity_unresolved,
            1
        );
        assert_eq!(classification.class_members_proven, 0);
        assert_eq!(classification.class_members_unresolved, 1);
    }

    #[test]
    fn frozen_class_single_voice_structural_observation_is_not_promoted_to_partition_precursor() {
        let member_a = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 1,
            attack_id: 1,
            measure: 1,
            pitched_ordinal: 0,
            represented_onset: 0,
            represented_duration: 12,
            staff: 1,
        };
        let member_b = super::super::notation_voice_pre_serialization_constraint_closure_audit::PreSerializationClosedSameLineMemberBinding {
            source_event_id: 2,
            attack_id: 2,
            measure: 1,
            pitched_ordinal: 1,
            represented_onset: 12,
            represented_duration: 12,
            staff: 1,
        };
        let metric = PreSerializationMetricNotationLineCandidate {
            metric_line_id: "metric-single-voice".to_string(),
            source_event_ids: vec![1, 2],
            members: vec![member_a.clone(), member_b.clone()],
            measure: 1,
            staff: 1,
            beat: 0,
            interval_start: 0,
            interval_end: 24,
        };
        let closed = PreSerializationClosedSameLineCandidate {
            proof_chain_id: "1:1-2:2".to_string(),
            source_event_ids: vec![1, 2],
            members: vec![member_a, member_b],
            staff: 1,
        };
        let render = score(concat!(
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>12</duration><voice>1</voice><staff>1</staff><beam number=\"1\">end</beam></note>"
        ));
        let classification =
            build_frozen_primary_beam_serialization_voice_partition_classification(
                &[metric],
                &[closed],
                &[],
                &[],
                &render,
            );
        assert_eq!(classification.exact_closed_same_line_metric_bindings, 1);
        assert_eq!(
            classification.persisted_render_cross_voice_structural_observations,
            0
        );
        assert_eq!(classification.class_members_proven, 0);
        assert_eq!(classification.class_members_unresolved, 0);
    }

    #[test]
    fn repair_scope_requires_proven_class_membership_and_materialization_safety() {
        let frozen = FrozenPrimaryBeamSerializationVoicePartitionClassification {
            class_members_proven: 2,
            class_members_unresolved: 1,
            proven_metric_line_ids: ["metric-a".to_string(), "metric-b".to_string()]
                .into_iter()
                .collect(),
            ..FrozenPrimaryBeamSerializationVoicePartitionClassification::default()
        };
        let safety = ["metric-a".to_string(), "metric-c".to_string()]
            .into_iter()
            .collect();
        let scope = build_primary_beam_serialization_voice_partition_repair_scope_audit(
            &frozen,
            &safety,
        );
        assert_eq!(scope.repair_scope_eligible, 1);
        assert_eq!(scope.blocked_materialization_safety, 1);
        assert_eq!(scope.class_members_unresolved, 1);
        assert_eq!(scope.materialization_safety_proven_candidates, 2);
        assert!(scope.eligible_metric_line_ids.contains("metric-a"));
        assert!(!scope.eligible_metric_line_ids.contains("metric-b"));
        assert!(!scope.eligible_metric_line_ids.contains("metric-c"));
    }

    #[test]
    fn late_rebind_evidence_replay_binds_existing_primary_target_without_writing() {
        let (metric, _) = cross_phase_metric_and_closed();
        let metric_id = metric.metric_line_id.clone();
        let frozen = FrozenPrimaryBeamSerializationVoicePartitionClassification {
            class_members_proven: 1,
            proven_metric_line_ids: [metric_id.clone()].into_iter().collect(),
            ..FrozenPrimaryBeamSerializationVoicePartitionClassification::default()
        };
        let safety = [metric_id.clone()].into_iter().collect();
        let scope = build_primary_beam_serialization_voice_partition_repair_scope_audit(
            &frozen,
            &safety,
        );
        let s0_occurrences = vec![
            cross_phase_s0_occurrence(1, 101, 0, 0),
            cross_phase_s0_occurrence(2, 102, 1, 12),
        ];
        let canonical_occurrences = vec![
            cross_phase_canonical_occurrence(1, 101, 0),
            cross_phase_canonical_occurrence(2, 102, 1),
        ];
        let target_evidence = vec![MaterializationSafetyPrimaryTargetEvidenceRow {
            metric_line_id: metric_id,
            target_primary_source_event_id: 1,
            target_primary_attack_id: 101,
            target_channel: 1,
        }];
        let replay =
            build_primary_beam_serialization_voice_partition_late_rebind_evidence_replay_audit(
                &scope,
                &[metric],
                &s0_occurrences,
                &canonical_occurrences,
                &target_evidence,
                &cross_phase_render(),
            );
        assert!(replay.render_parse_ok);
        assert_eq!(replay.eligible_metric_lines, 1);
        assert_eq!(replay.target_evidence_bindings, 1);
        assert_eq!(replay.member_occurrences_bound, 2);
        assert_eq!(replay.member_occurrences_unresolved, 0);
        assert_eq!(replay.existing_target_voice_bindings, 1);
        assert_eq!(replay.proposed_voice_scalar_changes, 1);
        assert_eq!(replay.already_target_voice_scalars, 1);
        assert_eq!(replay.source_overlap_conflicts, 0);
        assert_eq!(replay.physical_edit_footprint_conflicts, 0);
        assert_eq!(replay.replay_ready_metric_lines, 1);
        assert!(replay.rows[0].contains("1:1:0:1->1"));
        assert!(replay.rows[0].contains("2:1:1:3->1"));
    }

    #[test]
    fn repair_scope_independent_candidate_duplication_preserves_existing_decision() {
        let singleton = FrozenPrimaryBeamSerializationVoicePartitionClassification {
            class_members_proven: 1,
            proven_metric_line_ids: ["metric-a".to_string()].into_iter().collect(),
            ..FrozenPrimaryBeamSerializationVoicePartitionClassification::default()
        };
        let singleton_safety = ["metric-a".to_string()].into_iter().collect();
        let singleton_scope = build_primary_beam_serialization_voice_partition_repair_scope_audit(
            &singleton,
            &singleton_safety,
        );

        let duplicated = FrozenPrimaryBeamSerializationVoicePartitionClassification {
            class_members_proven: 2,
            proven_metric_line_ids: ["metric-a".to_string(), "metric-b".to_string()]
                .into_iter()
                .collect(),
            ..FrozenPrimaryBeamSerializationVoicePartitionClassification::default()
        };
        let duplicated_safety = ["metric-a".to_string(), "metric-b".to_string()]
            .into_iter()
            .collect();
        let duplicated_scope = build_primary_beam_serialization_voice_partition_repair_scope_audit(
            &duplicated,
            &duplicated_safety,
        );

        assert!(singleton_scope.eligible_metric_line_ids.contains("metric-a"));
        assert!(duplicated_scope.eligible_metric_line_ids.contains("metric-a"));
        assert_eq!(singleton_scope.repair_scope_eligible, 1);
        assert_eq!(duplicated_scope.repair_scope_eligible, 2);
    }

    #[test]
    fn fixture_type_remains_orderable_for_duplicate_rest_delta() {
        let mut set = BTreeSet::new();
        set.insert(VisibleRhythmicOccupancyDuplicateRestConflict {
            measure: 1,
            staff: 1,
            onset: 0,
            duration: 24,
        });
        assert_eq!(set.len(), 1);
    }
}
