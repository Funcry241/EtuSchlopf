use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

use super::final_notation_voice_beam_ownership_audit::{
    voice_free_primary_beam_chain_evidence, VoiceFreePrimaryBeamChainEvidence,
};

pub(crate) const SECTION: &str = "NOTATION-VOICE-CAUSAL-LANE-PRESSURE-AUDIT";
pub(crate) const SECTION_RULE: &str = "-----------------------------------------";

const OCCURRENCE_CORRESPONDENCE_RECEIPT_SCHEMA_V2: &str = concat!(
    "semantic_occurrence_correspondence_receipt_v2\n",
    "artifact_binding=render_xml_sha256\n",
    "identity=source_event_id,sounding_attack_id\n",
    "physical_locator=render_bound_locator_sha256\n",
    "physical_locator_derivation=sha256(domain,render_xml_sha256,measure,pitched_ordinal)\n",
    "physical_locator_domain=semantic_occurrence_render_physical_locator_v1\n",
    "materialization=kind_rank,fragment_ordinal\n",
    "row_order=physical_locator_sha256,source_event_id,attack_id,kind_rank,fragment_ordinal\n",
    "identity_inference=forbidden\n",
    "authority=audit_only_no_selector_no_writer_no_release_veto\n",
);

const OCCURRENCE_CORRESPONDENCE_PHYSICAL_LOCATOR_DOMAIN_V1: &str =
    "semantic_occurrence_render_physical_locator_v1";

#[derive(Debug, Clone, PartialEq, Eq)]
struct PersistedOccurrenceCorrespondenceReceiptRow {
    source_event_id: usize,
    attack_id: usize,
    physical_locator_sha256: String,
    materialization_kind_rank: u8,
    fragment_ordinal: usize,
}

fn occurrence_correspondence_physical_locator_sha256(
    render_sha256: &str,
    measure: usize,
    pitched_ordinal: usize,
) -> String {
    let mut payload = String::new();
    let _ = writeln!(
        payload,
        "domain={OCCURRENCE_CORRESPONDENCE_PHYSICAL_LOCATOR_DOMAIN_V1}"
    );
    let _ = writeln!(payload, "render_xml_sha256={render_sha256}");
    let _ = writeln!(payload, "measure={measure}");
    let _ = writeln!(payload, "pitched_ordinal={pitched_ordinal}");
    crate::sha256::hex(payload.as_bytes())
}

fn append_occurrence_correspondence_receipt(
    out: &mut String,
    render_xml: &str,
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
) {
    let schema_sha256 = crate::sha256::hex(OCCURRENCE_CORRESPONDENCE_RECEIPT_SCHEMA_V2.as_bytes());
    let render_sha256 = crate::sha256::hex(render_xml.as_bytes());
    let mut rows = occurrence_rows
        .iter()
        .map(|row| PersistedOccurrenceCorrespondenceReceiptRow {
            source_event_id: row.source_event_id,
            attack_id: row.attack_id,
            physical_locator_sha256: occurrence_correspondence_physical_locator_sha256(
                &render_sha256,
                row.measure,
                row.pitched_ordinal,
            ),
            materialization_kind_rank: row.materialization_kind_rank,
            fragment_ordinal: row.fragment_ordinal,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        (
            &left.physical_locator_sha256,
            left.source_event_id,
            left.attack_id,
            left.materialization_kind_rank,
            left.fragment_ordinal,
        )
            .cmp(&(
                &right.physical_locator_sha256,
                right.source_event_id,
                right.attack_id,
                right.materialization_kind_rank,
                right.fragment_ordinal,
            ))
    });

    let mut payload = String::new();
    let _ = writeln!(payload, "schema_sha256={schema_sha256}");
    let _ = writeln!(payload, "render_xml_sha256={render_sha256}");
    for row in &rows {
        let _ = writeln!(
            payload,
            "{}|{}|{}|{}|{}",
            row.source_event_id,
            row.attack_id,
            row.physical_locator_sha256,
            row.materialization_kind_rank,
            row.fragment_ordinal,
        );
    }
    let receipt_sha256 = crate::sha256::hex(payload.as_bytes());

    let _ = writeln!(out, "OccurrenceCorrespondenceReceipt: persisted_result_projection_v2");
    let _ = writeln!(out, "OccurrenceCorrespondenceReceiptAuthority: audit_only_no_selector_no_writer_no_release_veto");
    let _ = writeln!(out, "OccurrenceCorrespondenceReceiptIdentityInference: forbidden");
    let _ = writeln!(
        out,
        "OccurrenceCorrespondenceReceiptPhysicalLocatorDomain: {OCCURRENCE_CORRESPONDENCE_PHYSICAL_LOCATOR_DOMAIN_V1}"
    );
    let _ = writeln!(out, "OccurrenceCorrespondenceReceiptSchemaSha256: {schema_sha256}");
    let _ = writeln!(out, "OccurrenceCorrespondenceReceiptRenderXmlSha256: {render_sha256}");
    let _ = writeln!(out, "OccurrenceCorrespondenceReceiptRows: {}", rows.len());
    let _ = writeln!(out, "OccurrenceCorrespondenceReceiptSha256: {receipt_sha256}");
    for row in rows {
        let _ = writeln!(
            out,
            "OccurrenceCorrespondenceRow: source_event_id={} attack_id={} physical_locator_sha256={} materialization_kind_rank={} fragment_ordinal={}",
            row.source_event_id,
            row.attack_id,
            row.physical_locator_sha256,
            row.materialization_kind_rank,
            row.fragment_ordinal,
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NotationVoiceCausalSourceEvidenceRow {
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) measure: usize,
    pub(crate) onset: i64,
    pub(crate) pitch: i32,
    pub(crate) staff: u8,
    pub(crate) absolute_start: i64,
    pub(crate) absolute_end: i64,
    pub(crate) technical_lane: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NotationVoiceCanonicalOccurrenceIdentityRow {
    pub(crate) source_event_id: usize,
    pub(crate) attack_id: usize,
    pub(crate) measure: usize,
    pub(crate) pitched_ordinal: usize,
    pub(crate) materialization_kind_rank: u8,
    pub(crate) fragment_ordinal: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NotationVoiceCausalLanePressureAuditProjection {
    body: String,
    occurrence_identity_rows: Vec<NotationVoiceCanonicalOccurrenceIdentityRow>,
}

impl NotationVoiceCausalLanePressureAuditProjection {
    pub(crate) fn append_body(&self, out: &mut String) {
        out.push_str(&self.body);
    }

    pub(crate) fn occurrence_identity_rows(
        &self,
    ) -> &[NotationVoiceCanonicalOccurrenceIdentityRow] {
        &self.occurrence_identity_rows
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NotationVoiceCausalDecision {
    ProvenSameNotationVoice,
    SingleTechnicalLane,
    UnresolvedNonContiguousNotation,
    UnresolvedCanonicalOccurrenceIdentity,
    UnresolvedSourceAttackIdentity,
    UnresolvedOccurrenceAttackMismatch,
    UnresolvedLanePressureWitness,
    UnresolvedExternalLanePressure,
}

impl NotationVoiceCausalDecision {
    const fn label(self) -> &'static str {
        match self {
            Self::ProvenSameNotationVoice => "PROVEN_SAME_NOTATION_VOICE",
            Self::SingleTechnicalLane => "NOT_APPLICABLE_SINGLE_TECHNICAL_LANE",
            Self::UnresolvedNonContiguousNotation => "UNRESOLVED_NONCONTIGUOUS_NOTATION",
            Self::UnresolvedCanonicalOccurrenceIdentity => {
                "UNRESOLVED_CANONICAL_OCCURRENCE_IDENTITY"
            }
            Self::UnresolvedSourceAttackIdentity => "UNRESOLVED_SOURCE_ATTACK_IDENTITY",
            Self::UnresolvedOccurrenceAttackMismatch => {
                "UNRESOLVED_OCCURRENCE_ATTACK_MISMATCH"
            }
            Self::UnresolvedLanePressureWitness => "UNRESOLVED_LANE_PRESSURE_WITNESS",
            Self::UnresolvedExternalLanePressure => "UNRESOLVED_EXTERNAL_LANE_PRESSURE",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NotationVoiceOccurrenceAttackMismatchEvidence {
    materialization_kind_ranks: Vec<u8>,
    fragment_ordinals: Vec<usize>,
    mismatch_count: usize,
    measure_delta: bool,
    onset_delta: bool,
    staff_delta: bool,
    pitch_delta: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NotationVoiceCausalMemberEvidence {
    pitched_ordinal: usize,
    physical_onset: i64,
    duration: i64,
    pitch_midi: i32,
    current_voice: String,
    source_event_id: usize,
    attack_id: usize,
    semantic_onset: i64,
    technical_lane: usize,
    materialization_kind_rank: u8,
    fragment_ordinal: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NotationVoiceCausalChainClassification {
    decision: NotationVoiceCausalDecision,
    source_event_ids: Vec<usize>,
    attack_ids: Vec<usize>,
    technical_lanes: Vec<usize>,
    lane_pressure_witnesses: usize,
    external_blocker_source_ids: Vec<usize>,
    occurrence_attack_mismatch: Option<NotationVoiceOccurrenceAttackMismatchEvidence>,
    member_evidence: Vec<NotationVoiceCausalMemberEvidence>,
}

type SourceAttackKey = (usize, usize);
type SourceAttackIndex<'a> =
    HashMap<SourceAttackKey, Vec<&'a NotationVoiceCausalSourceEvidenceRow>>;
type OccurrenceKey = (usize, usize);
type OccurrenceIdentityIndex<'a> = HashMap<
    OccurrenceKey,
    Vec<&'a NotationVoiceCanonicalOccurrenceIdentityRow>,
>;

fn build_source_attack_index(
    source_rows: &[NotationVoiceCausalSourceEvidenceRow],
) -> SourceAttackIndex<'_> {
    let mut index: SourceAttackIndex<'_> = HashMap::new();
    for row in source_rows {
        index
            .entry((row.source_event_id, row.attack_id))
            .or_default()
            .push(row);
    }
    index
}

fn build_occurrence_identity_index(
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
) -> OccurrenceIdentityIndex<'_> {
    let mut index: OccurrenceIdentityIndex<'_> = HashMap::new();
    for row in occurrence_rows {
        index
            .entry((row.measure, row.pitched_ordinal))
            .or_default()
            .push(row);
    }
    index
}

fn materialization_kind_label(rank: u8) -> &'static str {
    match rank {
        0 => "surviving_source_note",
        1 => "tied_anchor_chain",
        2 => "auxiliary_lane",
        _ => "unknown",
    }
}

fn classify_chain(
    chain: &VoiceFreePrimaryBeamChainEvidence,
    source_rows: &[NotationVoiceCausalSourceEvidenceRow],
    source_index: &SourceAttackIndex<'_>,
    occurrence_index: &OccurrenceIdentityIndex<'_>,
) -> NotationVoiceCausalChainClassification {
    let unresolved = |decision| NotationVoiceCausalChainClassification {
        decision,
        source_event_ids: Vec::new(),
        attack_ids: Vec::new(),
        technical_lanes: Vec::new(),
        lane_pressure_witnesses: 0,
        external_blocker_source_ids: Vec::new(),
        occurrence_attack_mismatch: None,
        member_evidence: Vec::new(),
    };
    if chain.members.len() < 2
        || chain.members.windows(2).any(|pair| {
            pair[0].measure != pair[1].measure
                || pair[0].staff != pair[1].staff
                || pair[0].onset.saturating_add(pair[0].duration) != pair[1].onset
        })
    {
        return unresolved(NotationVoiceCausalDecision::UnresolvedNonContiguousNotation);
    }

    let mut mapped = Vec::<&NotationVoiceCausalSourceEvidenceRow>::with_capacity(chain.members.len());
    let mut mismatch_kind_ranks = BTreeSet::<u8>::new();
    let mut mismatch_fragment_ordinals = BTreeSet::<usize>::new();
    let mut mismatch_count = 0usize;
    let mut mismatch_measure = false;
    let mut mismatch_onset = false;
    let mut mismatch_staff = false;
    let mut mismatch_pitch = false;
    for member in &chain.members {
        let Some(occurrence_candidates) =
            occurrence_index.get(&(member.measure, member.pitched_ordinal))
        else {
            return unresolved(
                NotationVoiceCausalDecision::UnresolvedCanonicalOccurrenceIdentity,
            );
        };
        if occurrence_candidates.len() != 1 {
            return unresolved(
                NotationVoiceCausalDecision::UnresolvedCanonicalOccurrenceIdentity,
            );
        }
        let occurrence = occurrence_candidates[0];
        let Some(source_candidates) =
            source_index.get(&(occurrence.source_event_id, occurrence.attack_id))
        else {
            return unresolved(NotationVoiceCausalDecision::UnresolvedSourceAttackIdentity);
        };
        if source_candidates.len() != 1 {
            return unresolved(NotationVoiceCausalDecision::UnresolvedSourceAttackIdentity);
        }
        let source = source_candidates[0];
        let measure_delta = source.measure != member.measure;
        let onset_delta = source.onset != member.onset;
        let staff_delta = source.staff != member.staff;
        let pitch_delta = source.pitch != member.pitch_midi;
        if measure_delta || onset_delta || staff_delta || pitch_delta {
            mismatch_count = mismatch_count.saturating_add(1);
            mismatch_kind_ranks.insert(occurrence.materialization_kind_rank);
            mismatch_fragment_ordinals.insert(occurrence.fragment_ordinal);
            mismatch_measure |= measure_delta;
            mismatch_onset |= onset_delta;
            mismatch_staff |= staff_delta;
            mismatch_pitch |= pitch_delta;
        }
        mapped.push(source);
    }

    let member_evidence = chain
        .members
        .iter()
        .zip(mapped.iter().copied())
        .map(|(member, source)| {
            let occurrence = occurrence_index
                .get(&(member.measure, member.pitched_ordinal))
                .and_then(|rows| rows.first())
                .copied();
            NotationVoiceCausalMemberEvidence {
                pitched_ordinal: member.pitched_ordinal,
                physical_onset: member.onset,
                duration: member.duration,
                pitch_midi: member.pitch_midi,
                current_voice: member.current_voice.clone(),
                source_event_id: source.source_event_id,
                attack_id: source.attack_id,
                semantic_onset: source.onset,
                technical_lane: source.technical_lane,
                materialization_kind_rank: occurrence
                    .map_or(u8::MAX, |row| row.materialization_kind_rank),
                fragment_ordinal: occurrence.map_or(usize::MAX, |row| row.fragment_ordinal),
            }
        })
        .collect::<Vec<_>>();

    if mismatch_count > 0 {
        let source_event_ids = mapped
            .iter()
            .map(|row| row.source_event_id)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let attack_ids = mapped
            .iter()
            .map(|row| row.attack_id)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let technical_lanes = mapped
            .iter()
            .map(|row| row.technical_lane)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        return NotationVoiceCausalChainClassification {
            decision: NotationVoiceCausalDecision::UnresolvedOccurrenceAttackMismatch,
            source_event_ids,
            attack_ids,
            technical_lanes,
            lane_pressure_witnesses: 0,
            external_blocker_source_ids: Vec::new(),
            occurrence_attack_mismatch: Some(NotationVoiceOccurrenceAttackMismatchEvidence {
                materialization_kind_ranks: mismatch_kind_ranks.into_iter().collect(),
                fragment_ordinals: mismatch_fragment_ordinals.into_iter().collect(),
                mismatch_count,
                measure_delta: mismatch_measure,
                onset_delta: mismatch_onset,
                staff_delta: mismatch_staff,
                pitch_delta: mismatch_pitch,
            }),
            member_evidence,
        };
    }

    let unique_mapped = mapped
        .iter()
        .copied()
        .map(|row| ((row.source_event_id, row.attack_id), row))
        .collect::<BTreeMap<_, _>>();
    let owner_keys = unique_mapped.keys().copied().collect::<BTreeSet<_>>();
    let source_event_ids = owner_keys
        .iter()
        .map(|(source_event_id, _)| *source_event_id)
        .collect::<BTreeSet<_>>();
    let attack_ids = owner_keys
        .iter()
        .map(|(_, attack_id)| *attack_id)
        .collect::<BTreeSet<_>>();
    let technical_lanes = unique_mapped
        .values()
        .map(|row| row.technical_lane)
        .collect::<BTreeSet<_>>();
    if technical_lanes.len() < 2 {
        return NotationVoiceCausalChainClassification {
            decision: NotationVoiceCausalDecision::SingleTechnicalLane,
            source_event_ids: source_event_ids.into_iter().collect(),
            attack_ids: attack_ids.into_iter().collect(),
            technical_lanes: technical_lanes.into_iter().collect(),
            lane_pressure_witnesses: 0,
            external_blocker_source_ids: Vec::new(),
            occurrence_attack_mismatch: None,
            member_evidence,
        };
    }

    let mut lane_pressure_witnesses = 0usize;
    let mut external_blockers = BTreeSet::<usize>::new();
    for current in unique_mapped
        .values()
        .copied()
        .filter(|row| row.technical_lane > 1)
    {
        for lower_lane in 1..current.technical_lane {
            let mut blocker_found = false;
            for blocker in source_rows.iter().filter(|row| {
                row.staff == current.staff
                    && row.technical_lane == lower_lane
                    && row.absolute_start <= current.absolute_start
                    && current.absolute_start < row.absolute_end
            }) {
                blocker_found = true;
                if !owner_keys.contains(&(blocker.source_event_id, blocker.attack_id)) {
                    external_blockers.insert(blocker.source_event_id);
                }
            }
            if !blocker_found {
                return NotationVoiceCausalChainClassification {
                    decision: NotationVoiceCausalDecision::UnresolvedLanePressureWitness,
                    source_event_ids: source_event_ids.iter().copied().collect(),
                    attack_ids: attack_ids.iter().copied().collect(),
                    technical_lanes: technical_lanes.iter().copied().collect(),
                    lane_pressure_witnesses,
                    external_blocker_source_ids: external_blockers.into_iter().collect(),
                    occurrence_attack_mismatch: None,
                    member_evidence,
                };
            }
            lane_pressure_witnesses = lane_pressure_witnesses.saturating_add(1);
        }
    }

    if !external_blockers.is_empty() {
        return NotationVoiceCausalChainClassification {
            decision: NotationVoiceCausalDecision::UnresolvedExternalLanePressure,
            source_event_ids: source_event_ids.into_iter().collect(),
            attack_ids: attack_ids.into_iter().collect(),
            technical_lanes: technical_lanes.into_iter().collect(),
            lane_pressure_witnesses,
            external_blocker_source_ids: external_blockers.into_iter().collect(),
            occurrence_attack_mismatch: None,
            member_evidence,
        };
    }

    NotationVoiceCausalChainClassification {
        decision: NotationVoiceCausalDecision::ProvenSameNotationVoice,
        source_event_ids: source_event_ids.into_iter().collect(),
        attack_ids: attack_ids.into_iter().collect(),
        technical_lanes: technical_lanes.into_iter().collect(),
        lane_pressure_witnesses,
        external_blocker_source_ids: Vec::new(),
        occurrence_attack_mismatch: None,
        member_evidence,
    }
}

pub(crate) fn build_notation_voice_causal_lane_pressure_audit_projection(
    render_xml: &str,
    source_rows: &[NotationVoiceCausalSourceEvidenceRow],
    occurrence_rows: &[NotationVoiceCanonicalOccurrenceIdentityRow],
    frozen_lane_plan_bound: bool,
) -> NotationVoiceCausalLanePressureAuditProjection {
    let mut out = String::new();
    append_occurrence_correspondence_receipt(&mut out, render_xml, occurrence_rows);
    let Some(chains) = voice_free_primary_beam_chain_evidence(render_xml) else {
        let _ = writeln!(out, "ProjectionOnly: true");
        let _ = writeln!(out, "Authority: none");
        let _ = writeln!(out, "MusicMutation: nein");
        let _ = writeln!(out, "ParseRender: fail");
        let _ = writeln!(out, "Verdict: UNAVAILABLE_PARSE_FAILURE");
        return NotationVoiceCausalLanePressureAuditProjection {
            body: out,
            occurrence_identity_rows: occurrence_rows.to_vec(),
        };
    };

    let source_index = build_source_attack_index(source_rows);
    let occurrence_index = build_occurrence_identity_index(occurrence_rows);
    let mut multi_lane_chains = 0usize;
    let mut single_lane_chains = 0usize;
    let mut proven = 0usize;
    let mut unresolved_occurrence_identity = 0usize;
    let mut unresolved_source_attack = 0usize;
    let mut unresolved_occurrence_attack_mismatch = 0usize;
    let mut mismatch_surviving_source_note = 0usize;
    let mut mismatch_tied_anchor_chain = 0usize;
    let mut mismatch_auxiliary_lane = 0usize;
    let mut mismatch_multi_materialization = 0usize;
    let mut mismatch_positive_fragment_ordinal = 0usize;
    let mut mismatch_measure_delta = 0usize;
    let mut mismatch_onset_delta = 0usize;
    let mut mismatch_staff_delta = 0usize;
    let mut mismatch_pitch_delta = 0usize;
    let mut mismatch_onset_only = 0usize;
    let mut mismatch_single_dimension_other = 0usize;
    let mut mismatch_multiple_dimensions = 0usize;
    let mut unresolved_pressure = 0usize;
    let mut unresolved_external = 0usize;
    let mut unresolved_noncontiguous = 0usize;
    let mut proven_current_cross_voice = 0usize;
    let mut rows = Vec::<String>::new();

    for chain in &chains {
        let classification = classify_chain(
            chain,
            source_rows,
            &source_index,
            &occurrence_index,
        );
        let current_voices = chain
            .members
            .iter()
            .map(|member| member.current_voice.clone())
            .collect::<BTreeSet<_>>();
        match classification.decision {
            NotationVoiceCausalDecision::ProvenSameNotationVoice => {
                multi_lane_chains = multi_lane_chains.saturating_add(1);
                proven = proven.saturating_add(1);
                if current_voices.len() > 1 {
                    proven_current_cross_voice = proven_current_cross_voice.saturating_add(1);
                }
            }
            NotationVoiceCausalDecision::SingleTechnicalLane => {
                single_lane_chains = single_lane_chains.saturating_add(1);
                continue;
            }
            NotationVoiceCausalDecision::UnresolvedNonContiguousNotation => {
                unresolved_noncontiguous = unresolved_noncontiguous.saturating_add(1);
            }
            NotationVoiceCausalDecision::UnresolvedCanonicalOccurrenceIdentity => {
                unresolved_occurrence_identity = unresolved_occurrence_identity.saturating_add(1);
            }
            NotationVoiceCausalDecision::UnresolvedSourceAttackIdentity => {
                unresolved_source_attack = unresolved_source_attack.saturating_add(1);
            }
            NotationVoiceCausalDecision::UnresolvedOccurrenceAttackMismatch => {
                unresolved_occurrence_attack_mismatch =
                    unresolved_occurrence_attack_mismatch.saturating_add(1);
                if let Some(mismatch) = classification.occurrence_attack_mismatch.as_ref() {
                    if mismatch.materialization_kind_ranks.contains(&0) {
                        mismatch_surviving_source_note =
                            mismatch_surviving_source_note.saturating_add(1);
                    }
                    if mismatch.materialization_kind_ranks.contains(&1) {
                        mismatch_tied_anchor_chain =
                            mismatch_tied_anchor_chain.saturating_add(1);
                    }
                    if mismatch.materialization_kind_ranks.contains(&2) {
                        mismatch_auxiliary_lane = mismatch_auxiliary_lane.saturating_add(1);
                    }
                    if mismatch.materialization_kind_ranks.len() > 1 {
                        mismatch_multi_materialization =
                            mismatch_multi_materialization.saturating_add(1);
                    }
                    if mismatch.fragment_ordinals.iter().any(|ordinal| *ordinal > 0) {
                        mismatch_positive_fragment_ordinal =
                            mismatch_positive_fragment_ordinal.saturating_add(1);
                    }
                    if mismatch.measure_delta {
                        mismatch_measure_delta = mismatch_measure_delta.saturating_add(1);
                    }
                    if mismatch.onset_delta {
                        mismatch_onset_delta = mismatch_onset_delta.saturating_add(1);
                    }
                    if mismatch.staff_delta {
                        mismatch_staff_delta = mismatch_staff_delta.saturating_add(1);
                    }
                    if mismatch.pitch_delta {
                        mismatch_pitch_delta = mismatch_pitch_delta.saturating_add(1);
                    }
                    let dimension_count = [
                        mismatch.measure_delta,
                        mismatch.onset_delta,
                        mismatch.staff_delta,
                        mismatch.pitch_delta,
                    ]
                    .into_iter()
                    .filter(|value| *value)
                    .count();
                    if dimension_count == 1 && mismatch.onset_delta {
                        mismatch_onset_only = mismatch_onset_only.saturating_add(1);
                    } else if dimension_count == 1 {
                        mismatch_single_dimension_other =
                            mismatch_single_dimension_other.saturating_add(1);
                    } else if dimension_count > 1 {
                        mismatch_multiple_dimensions =
                            mismatch_multiple_dimensions.saturating_add(1);
                    }
                }
            }
            NotationVoiceCausalDecision::UnresolvedLanePressureWitness => {
                multi_lane_chains = multi_lane_chains.saturating_add(1);
                unresolved_pressure = unresolved_pressure.saturating_add(1);
            }
            NotationVoiceCausalDecision::UnresolvedExternalLanePressure => {
                multi_lane_chains = multi_lane_chains.saturating_add(1);
                unresolved_external = unresolved_external.saturating_add(1);
            }
        }

        let first = chain.members.first();
        let last = chain.members.last();
        if let (Some(first), Some(last)) = (first, last) {
            let stable_chain_id = if classification.member_evidence.is_empty() {
                "unbound".to_string()
            } else {
                classification
                    .member_evidence
                    .iter()
                    .map(|member| format!("{}:{}", member.source_event_id, member.attack_id))
                    .collect::<Vec<_>>()
                    .join("-")
            };
            let member_rows = classification
                .member_evidence
                .iter()
                .map(|member| {
                    format!(
                        "ord{}@r{}+{}:midi{}:v{}=>s{}:a{}@s{}:lane{}:kind{}:frag{}",
                        member.pitched_ordinal,
                        member.physical_onset,
                        member.duration,
                        member.pitch_midi,
                        member.current_voice,
                        member.source_event_id,
                        member.attack_id,
                        member.semantic_onset,
                        member.technical_lane,
                        materialization_kind_label(member.materialization_kind_rank),
                        member.fragment_ordinal,
                    )
                })
                .collect::<Vec<_>>()
                .join("|");
            let time_provenance = if classification
                .member_evidence
                .iter()
                .all(|member| member.physical_onset == member.semantic_onset)
            {
                "equal_semantic_and_physical_onset"
            } else {
                "representation_time_divergence_requires_authorized_origin_proof"
            };
            let analysis_status = match classification.decision {
                NotationVoiceCausalDecision::ProvenSameNotationVoice => {
                    "BLOCKED_INCOMPLETE_EVIDENCE_OCCUPANCY_COMPONENT_AND_MATERIALIZATION_PROOF"
                }
                _ => "BLOCKED_EXISTING_CAUSAL_CLASSIFICATION",
            };
            rows.push(format!(
                "CAUSAL_NOTATION_VOICE proof_chain_id={} part={} measure={} staff={} interval={}-{} current_voices={} technical_lanes={} decision={} source_ids=[{}] attack_ids=[{}] lane_pressure_witnesses={} external_blockers=[{}] mismatch_kinds=[{}] mismatch_fragment_ordinals=[{}] mismatch_count={} mismatch_delta={} identity_mapping={} time_provenance={} conflict_coverage=beam_local_lane_pressure_only materialization_contract=not_yet_proven analysis_status={} members=[{}] join=final_chord_occurrence_measure_pitched_ordinal_to_source_attack",
                stable_chain_id,
                first.part,
                first.measure,
                first.staff,
                first.onset,
                last.onset.saturating_add(last.duration),
                current_voices.into_iter().collect::<Vec<_>>().join(","),
                classification
                    .technical_lanes
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                classification.decision.label(),
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
                classification.lane_pressure_witnesses,
                classification
                    .external_blocker_source_ids
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                classification
                    .occurrence_attack_mismatch
                    .as_ref()
                    .map(|mismatch| mismatch.materialization_kind_ranks.iter().map(|rank| materialization_kind_label(*rank)).collect::<Vec<_>>().join(","))
                    .unwrap_or_default(),
                classification
                    .occurrence_attack_mismatch
                    .as_ref()
                    .map(|mismatch| mismatch.fragment_ordinals.iter().map(usize::to_string).collect::<Vec<_>>().join(","))
                    .unwrap_or_default(),
                classification
                    .occurrence_attack_mismatch
                    .as_ref()
                    .map_or(0, |mismatch| mismatch.mismatch_count),
                classification
                    .occurrence_attack_mismatch
                    .as_ref()
                    .map(|mismatch| {
                        let mut deltas = Vec::<&str>::new();
                        if mismatch.measure_delta { deltas.push("measure"); }
                        if mismatch.onset_delta { deltas.push("onset"); }
                        if mismatch.staff_delta { deltas.push("staff"); }
                        if mismatch.pitch_delta { deltas.push("pitch"); }
                        deltas.join(",")
                    })
                    .unwrap_or_default(),
                if classification.member_evidence.len() == chain.members.len() { "total_unique" } else { "incomplete" },
                time_provenance,
                analysis_status,
                member_rows,
            ));
        }
    }
    rows.sort();
    rows.dedup();

    let unresolved_total = unresolved_occurrence_identity
        .saturating_add(unresolved_source_attack)
        .saturating_add(unresolved_occurrence_attack_mismatch)
        .saturating_add(unresolved_pressure)
        .saturating_add(unresolved_external)
        .saturating_add(unresolved_noncontiguous);
    let verdict = if !frozen_lane_plan_bound {
        "UNAVAILABLE_FROZEN_LANE_PLAN_NOT_BOUND"
    } else if occurrence_rows.is_empty() {
        "UNAVAILABLE_CANONICAL_OCCURRENCE_IDENTITY"
    } else if proven > 0 && unresolved_total == 0 {
        "CAUSAL_PROOF_AVAILABLE"
    } else if proven > 0 {
        "CAUSAL_PROOF_AVAILABLE_WITH_UNRESOLVED"
    } else if unresolved_total > 0 {
        "NO_CAUSAL_PROOF_UNRESOLVED"
    } else {
        "NO_MULTI_LANE_PRIMARY_BEAM_CASE"
    };

    let _ = writeln!(out, "ProjectionOnly: true");
    let _ = writeln!(out, "Authority: none");
    let _ = writeln!(out, "MusicMutation: nein");
    let _ = writeln!(out, "RuleChange: nein");
    let _ = writeln!(out, "CurrentVoiceDecisionAuthority: none");
    let _ = writeln!(out, "TechnicalLaneRole: causal_evidence_only");
    let _ = writeln!(out, "PrimaryBeamDecisionAuthority: evidence_only");
    let _ = writeln!(
        out,
        "JoinIdentity: FinalChord transaction-local occurrence receipt (measure,pitched_ordinal)->(SourceEventId,SoundingAttackId,MaterializationKind,FragmentOrdinal); current_voice excluded"
    );
    let _ = writeln!(out, "AuditSourcePhase: final_pdf_source_occurrence_continuation_pre_horizontal_layout");
    let _ = writeln!(out, "FinalNotationOccupancy: exact_occurrence_continuation_note_duration");
    let _ = writeln!(out, "LanePressureWitness: lower_lane_active_at_attack_start");
    let _ = writeln!(
        out,
        "SelfInducedPressureRule: every_required_lower_lane_blocker_belongs_to_same_source_attack_set"
    );
    let _ = writeln!(out, "AnalysisContract: S0_evidence_only_no_writer_authority");
    let _ = writeln!(out, "CanonicalMusicPreservedTarget: pitch_semantic_onset_semantic_duration_strictly_unchanged");
    let _ = writeln!(out, "StableProofChainIdentity: ordered_SourceEventId_SoundingAttackId_members_only");
    let _ = writeln!(out, "EvidenceBinding: current_render_occurrence_receipt_separate_from_stable_chain_identity");
    let _ = writeln!(out, "ConflictCoverageStatus: INCOMPLETE_BEAM_LOCAL_ONLY");
    let _ = writeln!(out, "MaterializationSafetyStatus: NOT_PROVEN");
    let _ = writeln!(out, "CatalogRole: acceptance_oracle_only_not_selector");
    let _ = writeln!(out, "CanonicalOccurrenceIdentityRows: {}", occurrence_rows.len());
    let _ = writeln!(out, "PrimaryBeamChainsObserved: {}", chains.len());
    let _ = writeln!(out, "SingleTechnicalLaneChains: {single_lane_chains}");
    let _ = writeln!(out, "MultiTechnicalLaneChains: {multi_lane_chains}");
    let _ = writeln!(out, "ProvenSameNotationVoice: {proven}");
    let _ = writeln!(out, "ProvenCurrentCrossVoice: {proven_current_cross_voice}");
    let _ = writeln!(
        out,
        "UnresolvedCanonicalOccurrenceIdentity: {unresolved_occurrence_identity}"
    );
    let _ = writeln!(out, "UnresolvedSourceAttackIdentity: {unresolved_source_attack}");
    let _ = writeln!(
        out,
        "UnresolvedOccurrenceAttackMismatch: {unresolved_occurrence_attack_mismatch}"
    );
    let _ = writeln!(out, "MismatchSurvivingSourceNoteChains: {mismatch_surviving_source_note}");
    let _ = writeln!(out, "MismatchTiedAnchorChainChains: {mismatch_tied_anchor_chain}");
    let _ = writeln!(out, "MismatchAuxiliaryLaneChains: {mismatch_auxiliary_lane}");
    let _ = writeln!(out, "MismatchMultiMaterializationChains: {mismatch_multi_materialization}");
    let _ = writeln!(out, "MismatchPositiveFragmentOrdinalChains: {mismatch_positive_fragment_ordinal}");
    let _ = writeln!(out, "MismatchMeasureDeltaChains: {mismatch_measure_delta}");
    let _ = writeln!(out, "MismatchOnsetDeltaChains: {mismatch_onset_delta}");
    let _ = writeln!(out, "MismatchStaffDeltaChains: {mismatch_staff_delta}");
    let _ = writeln!(out, "MismatchPitchDeltaChains: {mismatch_pitch_delta}");
    let _ = writeln!(out, "MismatchOnsetOnlyChains: {mismatch_onset_only}");
    let _ = writeln!(out, "MismatchSingleDimensionOtherChains: {mismatch_single_dimension_other}");
    let _ = writeln!(out, "MismatchMultipleDimensionsChains: {mismatch_multiple_dimensions}");
    let _ = writeln!(out, "UnresolvedLanePressureWitness: {unresolved_pressure}");
    let _ = writeln!(out, "UnresolvedExternalLanePressure: {unresolved_external}");
    let _ = writeln!(out, "UnresolvedNoncontiguousNotation: {unresolved_noncontiguous}");
    let _ = writeln!(out, "Verdict: {verdict}");
    if rows.is_empty() {
        let _ = writeln!(out, "Rows: none");
    } else {
        for row in rows {
            let _ = writeln!(out, "Row: {row}");
        }
    }

    NotationVoiceCausalLanePressureAuditProjection {
        body: out,
        occurrence_identity_rows: occurrence_rows.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit_only::final_notation_voice_beam_ownership_audit::{
        VoiceFreePrimaryBeamChainEvidence, VoiceFreePrimaryBeamMemberEvidence,
    };

    #[test]
    fn occurrence_correspondence_receipt_survives_report_locator_sanitization_without_identity_inference() {
        let source_event_id = 7usize.saturating_mul(3);
        let attack_id = source_event_id.saturating_add(1);
        let scope = attack_id.saturating_add(1);
        let pitched_ordinal = scope.saturating_add(1);
        let row = NotationVoiceCanonicalOccurrenceIdentityRow {
            source_event_id,
            attack_id,
            measure: scope,
            pitched_ordinal,
            materialization_kind_rank: 0,
            fragment_ordinal: 0,
        };
        let render_xml = "<score-partwise version=\"4.0\"></score-partwise>";
        let render_sha256 = crate::sha256::hex(render_xml.as_bytes());
        let physical_locator_sha256 = occurrence_correspondence_physical_locator_sha256(
            &render_sha256,
            scope,
            pitched_ordinal,
        );
        let mut out = String::new();
        append_occurrence_correspondence_receipt(&mut out, render_xml, &[row]);

        assert!(out.contains("OccurrenceCorrespondenceReceipt: persisted_result_projection_v2"));
        assert!(out.contains(&format!(
            "OccurrenceCorrespondenceReceiptRenderXmlSha256: {render_sha256}"
        )));
        assert!(out.contains(&format!(
            "source_event_id={source_event_id} attack_id={attack_id} physical_locator_sha256={physical_locator_sha256}"
        )));
        assert!(!out.contains(" measure="));
        assert!(!out.contains(" pitched_ordinal="));
        assert!(out.contains("OccurrenceCorrespondenceReceiptIdentityInference: forbidden"));

        let sanitized = crate::report::sanitize_measure_locator_authority_report(out.clone());
        assert_eq!(
            sanitized, out,
            "persisted occurrence receipt must survive the canonical locator sanitizer byte-exactly"
        );
    }


    fn member(
        pitched_ordinal: usize,
        onset: i64,
        duration: i64,
        pitch_midi: i32,
        voice: &str,
    ) -> VoiceFreePrimaryBeamMemberEvidence {
        VoiceFreePrimaryBeamMemberEvidence {
            part: "P1".to_string(),
            measure: 1,
            staff: 1,
            onset,
            duration,
            pitch_midi,
            pitched_ordinal,
            current_voice: voice.to_string(),
        }
    }

    fn source(
        source_event_id: usize,
        attack_id: usize,
        onset: i64,
        pitch: i32,
        absolute_start: i64,
        absolute_end: i64,
        technical_lane: usize,
    ) -> NotationVoiceCausalSourceEvidenceRow {
        NotationVoiceCausalSourceEvidenceRow {
            source_event_id,
            attack_id,
            measure: 1,
            onset,
            pitch,
            staff: 1,
            absolute_start,
            absolute_end,
            technical_lane,
        }
    }

    fn occurrence(
        source_event_id: usize,
        attack_id: usize,
        pitched_ordinal: usize,
    ) -> NotationVoiceCanonicalOccurrenceIdentityRow {
        occurrence_with_materialization(source_event_id, attack_id, pitched_ordinal, 0, 0)
    }

    fn occurrence_with_materialization(
        source_event_id: usize,
        attack_id: usize,
        pitched_ordinal: usize,
        materialization_kind_rank: u8,
        fragment_ordinal: usize,
    ) -> NotationVoiceCanonicalOccurrenceIdentityRow {
        NotationVoiceCanonicalOccurrenceIdentityRow {
            source_event_id,
            attack_id,
            measure: 1,
            pitched_ordinal,
            materialization_kind_rank,
            fragment_ordinal,
        }
    }

    #[test]
    fn canonical_occurrence_identity_disambiguates_same_surface_locator() {
        let chain = VoiceFreePrimaryBeamChainEvidence {
            members: vec![
                member(0, 0, 48, 60, "1"),
                member(1, 48, 48, 62, "3"),
            ],
        };
        let sources = vec![
            source(1, 101, 0, 60, 0, 60, 1),
            source(99, 199, 0, 60, 0, 40, 3),
            source(2, 102, 48, 62, 48, 96, 2),
        ];
        let occurrences = vec![occurrence(1, 101, 0), occurrence(2, 102, 1)];
        let source_index = build_source_attack_index(&sources);
        let occurrence_index = build_occurrence_identity_index(&occurrences);
        let result = classify_chain(
            &chain,
            &sources,
            &source_index,
            &occurrence_index,
        );
        assert_eq!(
            result.decision,
            NotationVoiceCausalDecision::ProvenSameNotationVoice
        );
        assert_eq!(result.lane_pressure_witnesses, 1);
        assert!(result.external_blocker_source_ids.is_empty());
    }

    #[test]
    fn external_lower_lane_blocker_keeps_case_unresolved() {
        let chain = VoiceFreePrimaryBeamChainEvidence {
            members: vec![
                member(0, 48, 24, 60, "1"),
                member(1, 72, 24, 62, "3"),
            ],
        };
        let sources = vec![
            source(9, 109, 0, 50, 0, 90, 1),
            source(1, 101, 48, 60, 48, 70, 2),
            source(2, 102, 72, 62, 72, 96, 1),
        ];
        let occurrences = vec![occurrence(1, 101, 0), occurrence(2, 102, 1)];
        let source_index = build_source_attack_index(&sources);
        let occurrence_index = build_occurrence_identity_index(&occurrences);
        let result = classify_chain(
            &chain,
            &sources,
            &source_index,
            &occurrence_index,
        );
        assert_eq!(
            result.decision,
            NotationVoiceCausalDecision::UnresolvedExternalLanePressure
        );
        assert_eq!(result.external_blocker_source_ids, vec![9]);
    }

    #[test]
    fn rematerialized_non_attack_occurrence_fails_closed() {
        let chain = VoiceFreePrimaryBeamChainEvidence {
            members: vec![
                member(0, 0, 48, 60, "1"),
                member(1, 48, 48, 62, "3"),
            ],
        };
        let sources = vec![
            source(1, 101, 0, 60, 0, 60, 1),
            source(2, 102, 24, 62, 24, 96, 2),
        ];
        let occurrences = vec![
            occurrence(1, 101, 0),
            occurrence_with_materialization(2, 102, 1, 1, 1),
        ];
        let source_index = build_source_attack_index(&sources);
        let occurrence_index = build_occurrence_identity_index(&occurrences);
        let result = classify_chain(
            &chain,
            &sources,
            &source_index,
            &occurrence_index,
        );
        assert_eq!(
            result.decision,
            NotationVoiceCausalDecision::UnresolvedOccurrenceAttackMismatch
        );
        let mismatch = result
            .occurrence_attack_mismatch
            .expect("mismatch evidence");
        assert_eq!(mismatch.materialization_kind_ranks, vec![1]);
        assert_eq!(mismatch.fragment_ordinals, vec![1]);
        assert_eq!(mismatch.mismatch_count, 1);
        assert!(mismatch.onset_delta);
        assert!(!mismatch.measure_delta);
        assert!(!mismatch.staff_delta);
        assert!(!mismatch.pitch_delta);
    }

    #[test]
    fn mismatch_report_separates_materialization_and_delta_dimensions() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>48</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>48</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note></measure></part></score-partwise>";
        let sources = vec![
            source(1, 101, 0, 60, 0, 60, 1),
            source(2, 102, 24, 62, 24, 96, 2),
        ];
        let occurrences = vec![
            occurrence(1, 101, 0),
            occurrence_with_materialization(2, 102, 1, 1, 2),
        ];
        let projection = build_notation_voice_causal_lane_pressure_audit_projection(
            xml,
            &sources,
            &occurrences,
            true,
        );
        let mut body = String::new();
        projection.append_body(&mut body);
        assert!(body.contains("UnresolvedOccurrenceAttackMismatch: 1"));
        assert!(body.contains("MismatchTiedAnchorChainChains: 1"));
        assert!(body.contains("MismatchPositiveFragmentOrdinalChains: 1"));
        assert!(body.contains("MismatchOnsetOnlyChains: 1"));
        assert!(body.contains("mismatch_kinds=[tied_anchor_chain]"));
        assert!(body.contains("mismatch_fragment_ordinals=[2]"));
        assert!(body.contains("mismatch_delta=onset"));
    }

    #[test]
    fn surviving_source_note_pitch_mismatch_stays_distinct_from_tie_fragment() {
        let chain = VoiceFreePrimaryBeamChainEvidence {
            members: vec![
                member(0, 0, 48, 60, "1"),
                member(1, 48, 48, 63, "3"),
            ],
        };
        let sources = vec![
            source(1, 101, 0, 60, 0, 60, 1),
            source(2, 102, 48, 62, 48, 96, 2),
        ];
        let occurrences = vec![occurrence(1, 101, 0), occurrence(2, 102, 1)];
        let source_index = build_source_attack_index(&sources);
        let occurrence_index = build_occurrence_identity_index(&occurrences);
        let result = classify_chain(&chain, &sources, &source_index, &occurrence_index);
        let mismatch = result
            .occurrence_attack_mismatch
            .expect("mismatch evidence");
        assert_eq!(mismatch.materialization_kind_ranks, vec![0]);
        assert_eq!(mismatch.fragment_ordinals, vec![0]);
        assert!(mismatch.pitch_delta);
        assert!(!mismatch.onset_delta);
    }

    #[test]
    fn current_voice_label_permutation_does_not_change_causal_decision() {
        let left = VoiceFreePrimaryBeamChainEvidence {
            members: vec![
                member(0, 0, 48, 60, "1"),
                member(1, 48, 48, 62, "3"),
            ],
        };
        let right = VoiceFreePrimaryBeamChainEvidence {
            members: vec![
                member(0, 0, 48, 60, "17"),
                member(1, 48, 48, 62, "2"),
            ],
        };
        let sources = vec![
            source(1, 101, 0, 60, 0, 60, 1),
            source(2, 102, 48, 62, 48, 96, 2),
        ];
        let occurrences = vec![occurrence(1, 101, 0), occurrence(2, 102, 1)];
        let source_index = build_source_attack_index(&sources);
        let occurrence_index = build_occurrence_identity_index(&occurrences);
        assert_eq!(
            classify_chain(&left, &sources, &source_index, &occurrence_index).decision,
            classify_chain(&right, &sources, &source_index, &occurrence_index).decision,
        );
    }
    #[test]
    fn proof_row_binds_stable_members_without_voice_in_chain_identity() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>48</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>48</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note></measure></part></score-partwise>";
        let sources = vec![
            source(1, 101, 0, 60, 0, 60, 1),
            source(2, 102, 48, 62, 48, 96, 2),
        ];
        let occurrences = vec![occurrence(1, 101, 0), occurrence(2, 102, 1)];
        let projection = build_notation_voice_causal_lane_pressure_audit_projection(
            xml, &sources, &occurrences, true,
        );
        let mut body = String::new();
        projection.append_body(&mut body);
        assert!(body.contains("proof_chain_id=1:101-2:102"));
        assert!(body.contains("measure=1 staff=1 interval=0-96"));
        assert!(body.contains("identity_mapping=total_unique"));
        assert!(body.contains("time_provenance=equal_semantic_and_physical_onset"));
        assert!(body.contains("analysis_status=BLOCKED_INCOMPLETE_EVIDENCE_OCCUPANCY_COMPONENT_AND_MATERIALIZATION_PROOF"));
    }

    #[test]
    fn onset_divergence_is_reported_as_missing_authorized_origin_proof() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>48</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>48</duration><voice>3</voice><staff>1</staff><beam number=\"1\">end</beam></note></measure></part></score-partwise>";
        let sources = vec![
            source(1, 101, 0, 60, 0, 60, 1),
            source(2, 102, 24, 62, 24, 96, 2),
        ];
        let occurrences = vec![occurrence(1, 101, 0), occurrence(2, 102, 1)];
        let projection = build_notation_voice_causal_lane_pressure_audit_projection(
            xml, &sources, &occurrences, true,
        );
        let mut body = String::new();
        projection.append_body(&mut body);
        assert!(body.contains("time_provenance=representation_time_divergence_requires_authorized_origin_proof"));
        assert!(body.contains("mismatch_delta=onset"));
    }

}
