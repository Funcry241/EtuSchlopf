//! Original positive beam ownership survives a purely technical voice partition.
//! No XML/report observation can construct the private origin carrier.
use super::*;
use crate::audit_only::original_beam_diagnostics::{self as beam_diagnostics, Observation};
use std::collections::{BTreeMap, BTreeSet};

type OriginalCapturedMember<'a> = (
    &'a FinalChordEncodingNote,
    (usize, usize),
    String,
    usize,
    crate::pipeline::ActualBeamLevels,
);
type ChannelConflict = (usize, usize, usize);
type ChannelSafety = (BTreeSet<ChannelConflict>, BTreeSet<String>);

type OriginalBeamAuditLocation = (usize,String,i64,i64,usize,String);
type OriginalBeamAuditLocations = BTreeMap<(usize,usize),Vec<OriginalBeamAuditLocation>>;
type OriginalBeamOccurrenceKey = (usize, usize, usize);
type ClosedBridgeOrigins = Vec<(usize, BTreeSet<OriginalBeamOccurrenceKey>)>;
type BoundNote<'a> = (&'a FinalChordEncodingNote, &'a FinalChordTransactionOutputOccurrenceWitness);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OriginalBeamGraceHost {
    grace: OriginalBeamOccurrenceKey,
    host: OriginalBeamOccurrenceKey,
}

fn occurrence_key(row: &FinalChordTransactionOutputOccurrenceWitness) -> OriginalBeamOccurrenceKey {
    (row.source_event_id, row.attack_id, row.fragment_id)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OriginalPositiveBeamOrigin {
    id: String,
    measure: usize,
    members: BTreeSet<(usize, usize, usize)>,
    states: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OriginalBeamVoiceGroup {
    positive_origins: Vec<OriginalPositiveBeamOrigin>,
    id: String,
    members: Vec<(usize, usize)>,
    states: Vec<String>,
    fragments: Vec<usize>,
    levels: Vec<crate::pipeline::ActualBeamLevels>,
    measure: usize,
    staff: u8,
    semantic_voice: String,
    original_offsets: Vec<usize>,
    primary_group_count: usize,
    // Canonical original source identity, not additional beam membership.
    // Shared unchanged across the real owner's groups and their transports.
    source_resources: std::sync::Arc<[FinalChordCanonicalSourceLocator]>,
    notation_voices: std::sync::Arc<BTreeMap<(usize, usize), String>>,
}

impl CanonicalFinalChordStructuralPlan {
    pub(super) fn allocation_original_beam_groups(&self) -> impl Iterator<Item = &OriginalBeamVoiceGroup> {
        self.original_beam_voice_groups.iter().filter(|group| group.is_measure_local())
    }

    // This is inherited producer evidence, never inferred from current XML
    // labels, acoustic lanes, roles or a report. Every current source fragment
    // must retain a binding to the same independently captured notation voice.
    pub(super) fn proves_common_original_notation_voice(&self, sources: &BTreeSet<usize>) -> bool {
        let Some(group) = self.allocation_original_beam_groups().next() else { return false; };
        if sources.is_empty() || !std::sync::Arc::ptr_eq(&group.source_resources, &self.source_locators)
            || self.original_beam_voice_groups.iter().any(|other|
                !std::sync::Arc::ptr_eq(&group.notation_voices, &other.notation_voices)) {
            return false;
        }
        let mut represented = BTreeSet::new();
        let mut common: Option<&str> = None;
        for locator in self.source_locators.iter().filter(|row| sources.contains(&row.source_event_id)) {
            let Some(voice) = group.notation_voices.get(&(locator.source_event_id, locator.attack_id)) else {
                return false;
            };
            if voice.trim().is_empty() || common.is_some_and(|current| current != voice) { return false; }
            common = Some(voice);
            represented.insert(locator.source_event_id);
        }
        common.is_some() && represented == *sources
    }

    // MusicXML channel names are reusable across independent written frames.
    // A whole technical channel is not a musical line. Establish origin in
    // the sealed component's written footprint; every physical conservation
    // check still covers the entire proposal and all source resources.
    pub(super) fn proves_original_notation_voice_for_component_reuse(
        &self,
        component: &BTreeSet<usize>,
        destination: &BTreeSet<usize>,
    ) -> bool {
        let footprint = self.source_locators.iter()
            .filter(|row| component.contains(&row.source_event_id))
            .map(|row| row.measure).collect::<BTreeSet<_>>();
        let mut relevant = self.source_locators.iter()
            .filter(|row| destination.contains(&row.source_event_id) && footprint.contains(&row.measure))
            .map(|row| row.source_event_id).collect::<BTreeSet<_>>();
        if relevant.is_empty() { return false; }
        relevant.extend(component);
        self.proves_common_original_notation_voice(&relevant)
    }

    pub(crate) fn bind_original_beam_voice_groups(
        &mut self,
        original: &str,
        captured: Option<
            &Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>,
        >,
        notation: &crate::pipeline::OriginalNotationVoiceBindings,
    ) {
        self.original_beam_voice_groups.clear();
        let Some(Ok(captured)) = captured else {
            return;
        };
        let Some(written) = self.written_projection.as_ref() else {
            return;
        };
        if !self.voice_free_evidence_sufficiency_green()
            || written.original_xml.as_ref() != original
        {
            return;
        }
        if roxmltree::Document::parse(original)
            .ok()
            .is_none_or(|doc| doc.descendants().filter(|n| n.has_tag_name("part")).count() != 1)
        {
            return;
        }
        let Ok(rows) = captured.occurrences(original) else {
            return;
        };
        let Ok(measures) = final_chord_parse_binding_measures(original) else {
            return;
        };
        let mut groups = BTreeMap::<(usize, usize), Vec<OriginalCapturedMember<'_>>>::new();
        let mut incomplete = BTreeSet::new();
        // Read-only census includes original groups that cannot become a
        // productive source-closed component. They must not vanish from the
        // final native review merely because no writer capability is minted.
        let mut original_counts = BTreeMap::<(usize,usize),usize>::new();
        let mut original_locations = OriginalBeamAuditLocations::new();
        for (range, levels) in rows {
            let Some((_, Some(group), Some(state))) =
                levels.iter().find(|(level, _, _)| *level == 1)
            else {
                continue;
            };
            *original_counts.entry(*group).or_default() += 1;
            let matches = measures
                .iter()
                .flat_map(|m| &m.notes)
                .filter(|note| note.range == *range)
                .collect::<Vec<_>>();
            let [note] = matches.as_slice() else {
                incomplete.insert(*group);
                continue;
            };
            original_locations.entry(*group).or_default().push((note.measure,
                note.staff.clone(),note.onset,note.duration,note.raw_note_index,note.pitch_identity.clone()));
            let locators = self
                .source_locators
                .iter()
                .filter(|locator| {
                    locator.measure == note.measure && locator.raw_note_index == note.raw_note_index
                })
                .collect::<Vec<_>>();
            let [locator] = locators.as_slice() else {
                incomplete.insert(*group);
                continue;
            };
            groups.entry(*group).or_default().push((
                note,
                (locator.source_event_id, locator.attack_id),
                state.clone(),
                locator.fragment_id,
                levels.clone(),
            ));
        }
        // This independent pre-lane witness is captured once at its actual owner.
        // Missing entries never acquire a semantic voice from later XML labels.
        let notation_voices: std::sync::Arc<BTreeMap<(usize, usize), String>> = std::sync::Arc::new(self.source_locators.iter().filter_map(|row|
            notation.original_voice(original, row.source_event_id, row.attack_id)
                .map(|voice| ((row.source_event_id, row.attack_id), voice.to_owned()))).collect());
        for (identity, mut rows) in groups {
            if incomplete.contains(&identity) {
                continue;
            }
            rows.sort_by_key(|(note, _, _, _, _)| note.range.start);
            let Some((first, _, _, _, _)) = rows.first() else {
                continue;
            };
            let Some(staff) = first
                .staff
                .parse::<u8>()
                .ok()
                .filter(|s| matches!(s, 1 | 2))
            else {
                continue;
            };
            let members = rows
                .iter()
                .map(|(_, member, _, _, _)| *member)
                .collect::<Vec<_>>();
            let states = rows
                .iter()
                .map(|(_, _, state, _, _)| state.clone())
                .collect::<Vec<_>>();
            let fragments: Vec<usize> = rows
                .iter()
                .map(|(_, _, _, fragment, _)| *fragment)
                .collect();
            let levels = rows
                .iter()
                .map(|(_, _, _, _, levels)| levels.clone())
                .collect();
            let Some(semantic_voice) =
                notation.original_voice(original, members[0].0, members[0].1)
            else {
                continue;
            };
            if !closed_primary(&states)
                || members.iter().any(|&(source, attack)| {
                    notation.original_voice(original, source, attack) != Some(semantic_voice)
                })
                || fragments.iter().collect::<BTreeSet<_>>().len() != fragments.len()
                || rows.iter().any(|(note, _, _, _, _)| {
                    note.measure != first.measure
                        || note.staff != first.staff
                        || note.chord_member
                        || note.rest_like
                        || note.duration <= 0
                })
            {
                continue;
            }
            let id = format!(
                "original-beam-{:016x}-{}-{}",
                self.semantic_fingerprint_fnv64, identity.0, identity.1
            );
            self.original_beam_voice_groups
                .push(OriginalBeamVoiceGroup {
                    positive_origins: vec![OriginalPositiveBeamOrigin {
                        id: id.clone(),
                        measure: first.measure,
                        members: members.iter().zip(&fragments).map(|(&(source,attack),&fragment)|(source,attack,fragment)).collect(),
                        states: states.clone(),
                    }],
                    id,
                    members,
                    states,
                    fragments,
                    levels,
                    measure: first.measure,
                    staff,
                    semantic_voice: semantic_voice.to_owned(),
                    original_offsets: rows
                        .iter()
                        .map(|(note, _, _, _, _)| note.range.start)
                        .collect(),
                    primary_group_count: 1,
                    source_resources: self.source_locators.clone(),
                    notation_voices: notation_voices.clone(),
                });
        }
        let bound_seeds = self.original_beam_voice_groups.iter().map(|group|group.id.clone()).collect::<BTreeSet<_>>();
        self.original_beam_voice_groups =
            close_shared_sources(std::mem::take(&mut self.original_beam_voice_groups));
        let retained = self.original_beam_voice_groups.iter()
            .flat_map(|group| group.positive_origins.iter().map(|origin| origin.id.as_str()))
            .collect::<BTreeSet<_>>();
        for ((owner_group,owner_piece),count) in original_counts {
            let id=format!("original-beam-{:016x}-{owner_group}-{owner_piece}",self.semantic_fingerprint_fnv64);
            let locations = original_locations.get(&(owner_group,owner_piece));
            let unusual = beam_diagnostics::record(Observation::Census {
                bound_seed: bound_seeds.contains(&id),
                retained_component: retained.contains(id.as_str()),
                source_locations_present: locations.is_some(),
            });
            if unusual || locations.is_some_and(|rows| rows.iter().any(|row| beam_diagnostics::detail_requested_for_measure(row.0))) {
                eprintln!("OriginalBeamOriginCensus: group={id}; original_members={count}; bound_seed={}; retained_component={}; source_locations={:?}; origin=actual_original_owner; authority=none",bound_seeds.contains(&id),retained.contains(id.as_str()),locations);
            }
        }
    }
}

impl OriginalBeamVoiceGroup {
    pub(super) fn is_measure_local(&self) -> bool {
        self.positive_origins.iter().all(|origin| origin.measure == self.measure)
    }

    fn observed_fragments(
        &self,
        visible_fragments: &BTreeMap<String, usize>,
        source: &FinalPdfSourceOccurrenceContinuation,
        current: &[FinalChordBindingMeasure],
    ) -> Option<usize> {
        if self.is_measure_local() {
            return visible_fragments.get(&self.id).copied();
        }
        let rows = exact_members(source, self)?;
        let mut total = 0usize;
        for origin in &self.positive_origins {
            let fragments = *visible_fragments.get(&origin.id)?;
            if fragments == 0 || fragments > origin.members.len() {
                return None;
            }
            if fragments > 1 {
                let mut members: Vec<_> = rows.iter()
                    .filter(|row| origin.members.contains(&occurrence_key(row)))
                    .collect();
                // This is the same order sent to renderer cohort geometry.
                members.sort_unstable_by_key(|row| row.pitched_ordinal);
                let labels = members.iter().map(|row| {
                    pitched_note(current, row.measure, row.pitched_ordinal).map(|note| note.voice.trim())
                }).collect::<Option<Vec<_>>>()?;
                if !proves_missing_channel_boundary(&labels, fragments) {
                    return None;
                }
            }
            total = total.checked_add(fragments)?;
        }
        Some(total)
    }

    /// Sealed original-owner membership and the independently carried pre-lane
    /// notation relation. Coordinates and current labels mint no new identity.
    pub(super) fn allocation_sources(&self) -> BTreeSet<usize> {
        self.members.iter().map(|&(source, _)| source).collect()
    }

    pub(super) fn closed_bridge_origins(&self, authored: &BTreeSet<OriginalBeamOccurrenceKey>) -> ClosedBridgeOrigins {
        if !self.is_measure_local() {
            return Vec::new();
        }
        self.positive_origins.iter().enumerate().filter(|(_,origin)|
            closed_primary(&origin.states) && origin.members.is_subset(authored))
            .map(|(index,origin)|(index,origin.members.clone())).collect()
    }
    pub(super) fn owns_complete_sustain_beam_tail(
        &self,
        tail: &BTreeMap<OriginalBeamOccurrenceKey, BTreeMap<u8, String>>,
    ) -> bool {
        self.is_measure_local() && self.positive_origins.iter().any(|origin| {
            origin.members.len() == tail.len() && closed_primary(&origin.states)
                && origin.members.iter().all(|key| {
                    let Some(observed) = tail.get(key) else { return false; };
                    self.members.iter().zip(&self.fragments).zip(&self.levels)
                        .find(|(((source, attack), fragment), _)| (*source, *attack, **fragment) == *key)
                        .is_some_and(|(_, levels)| {
                            levels.iter().filter_map(|(level, _, state)| {
                                state.clone().map(|value| (*level, value))
                            }).collect::<BTreeMap<_, _>>() == *observed
                        })
                })
        })
    }

    pub(super) fn bridge_component_occurrences(&self) -> BTreeSet<(usize,usize,usize)> {
        self.members.iter().zip(&self.fragments).map(|(&(source,attack),&fragment)|(source,attack,fragment)).collect()
    }

}

// The renderer counts one extra fragment for each missing adjacent
// connection. If all missing edges could be within a single label on each
// side, the observation has not proved any break at a label boundary.
fn proves_missing_channel_boundary(labels: &[&str], visible_fragments: usize) -> bool {
    let same_channel_edges = labels.windows(2).filter(|pair| pair[0] == pair[1]).count();
    visible_fragments <= labels.len()
        && visible_fragments.checked_sub(1).is_some_and(|missing| missing > same_channel_edges)
}

fn close_shared_sources(mut pending: Vec<OriginalBeamVoiceGroup>) -> Vec<OriginalBeamVoiceGroup> {
    let mut result = Vec::new();
    while let Some(first) = pending.pop() {
        let mut component = vec![first];
        loop {
            let sources = component
                .iter()
                .flat_map(|group| group.members.iter().copied())
                .collect::<BTreeSet<_>>();
            let mut changed = false;
            for index in (0..pending.len()).rev() {
                if pending[index]
                    .members
                    .iter()
                    .any(|member| sources.contains(member))
                {
                    component.push(pending.remove(index));
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let first = &component[0];
        // A held source can connect independent local groups across a bar.
        // Only the shared channel decision spans the bar, never a beam edge.
        if component.iter().any(|group| {
            group.staff != first.staff || group.semantic_voice != first.semantic_voice
        }) {
            continue;
        }
        let mut rows = component
            .iter()
            .flat_map(|group| {
                (0..group.members.len()).map(move |index| {
                    (
                        group.original_offsets[index],
                        group.members[index],
                        group.fragments[index],
                        group.states[index].clone(),
                        group.levels[index].clone(),
                    )
                })
            })
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| row.0);
        if rows.iter().map(|row| row.2).collect::<BTreeSet<_>>().len() != rows.len() {
            continue;
        }
        let mut ids = component
            .iter()
            .map(|group| group.id.clone())
            .collect::<Vec<_>>();
        ids.sort();
        result.push(OriginalBeamVoiceGroup {
            positive_origins: component.iter().flat_map(|group|group.positive_origins.clone()).collect(),
            id: ids.join("+"),
            members: rows.iter().map(|row| row.1).collect(),
            fragments: rows.iter().map(|row| row.2).collect(),
            states: rows.iter().map(|row| row.3.clone()).collect(),
            levels: rows.iter().map(|row| row.4.clone()).collect(),
            original_offsets: rows.iter().map(|row| row.0).collect(),
            measure: first.measure,
            staff: first.staff,
            semantic_voice: first.semantic_voice.clone(),
            source_resources: first.source_resources.clone(),
            notation_voices: first.notation_voices.clone(),
            primary_group_count: component
                .iter()
                .map(|group| group.primary_group_count)
                .sum(),
        });
    }
    result.sort_by(|left, right| left.id.cmp(&right.id));
    result
}

fn closed_primary(states: &[String]) -> bool {
    states.len() >= 2
        && states.first().is_some_and(|s| s == "begin")
        && states.last().is_some_and(|s| s == "end")
        && states[1..states.len() - 1].iter().all(|s| s == "continue")
}

fn exact_members<'a>(
    source: &'a FinalPdfSourceOccurrenceContinuation,
    group: &OriginalBeamVoiceGroup,
) -> Option<Vec<&'a FinalChordTransactionOutputOccurrenceWitness>> {
    let mut result = Vec::new();
    let mut measure_by_member = BTreeMap::new();
    for origin in &group.positive_origins {
        for &identity in &origin.members {
            if measure_by_member.insert(identity, origin.measure).is_some() {
                return None;
            }
        }
    }
    for (index, &(source_id, attack_id)) in group.members.iter().enumerate() {
        let matches = source
            .receipt
            .occurrences
            .iter()
            .filter(|row| {
                row.source_event_id == source_id
                    && row.attack_id == attack_id
                    && row.fragment_id == group.fragments[index]
            })
            .collect::<Vec<_>>();
        // A source with generated/tied copies needs a separate whole-source
        // transport; a conveniently chosen occurrence cannot authorize it.
        let [row] = matches.as_slice() else {
            return None;
        };
        if measure_by_member.get(&(source_id, attack_id, row.fragment_id)) != Some(&row.measure) {
            return None;
        }
        result.push(*row);
    }
    let keys = group.members.iter().copied().collect::<BTreeSet<_>>();
    let all = source
        .receipt
        .occurrences
        .iter()
        .filter(|row| keys.contains(&(row.source_event_id, row.attack_id)))
        .count();
    if all != result.len() {
        let actual = source.receipt.occurrences.iter()
            .filter(|row|keys.contains(&(row.source_event_id,row.attack_id)))
            .map(|row|(row.source_event_id,row.attack_id,row.fragment_id)).collect::<Vec<_>>();
        let unique=actual.iter().copied().collect::<BTreeSet<_>>();
        let expected=group.source_resources.iter()
            .filter(|row|keys.contains(&(row.source_event_id,row.attack_id))
                && !source.receipt.certified_omitted_fragment_ids.contains(&row.fragment_id))
            .map(|row|(row.source_event_id,row.attack_id,row.fragment_id)).collect::<BTreeSet<_>>();
        if unique.len()!=actual.len() || unique!=expected { return None; }
    }
    Some(result)
}

fn renderer_component_cohorts(
    source: &FinalPdfSourceOccurrenceContinuation,
    group: &OriginalBeamVoiceGroup,
) -> Option<Vec<PrimaryBeamVoicePartitionRendererVerificationCohort>> {
    let rows = exact_members(source, group)?;
    let bind = |id: &str, measure, expected_count, mut ordinals: Vec<usize>| {
        ordinals.sort_unstable();
        if ordinals.len() != expected_count || ordinals.windows(2).any(|pair| pair[0] == pair[1]) {
            return None;
        }
        let unusual = beam_diagnostics::record(Observation::RendererCohort);
        if unusual || beam_diagnostics::detail_requested_for_measure(measure) {
            eprintln!("OriginalBeamRendererCohort: group={id}; measure={measure}; staff={}; pitched_ordinals={ordinals:?}; origin=actual_owner_continuation; authority=none", group.staff);
        }
        Some(PrimaryBeamVoicePartitionRendererVerificationCohort {
            metric_line_id: id.to_owned(),
            measure,
            staff: group.staff,
            pitched_ordinals: ordinals,
        })
    };
    if group.is_measure_local() {
        return Some(vec![bind(&group.id, group.measure, group.members.len(),
            rows.iter().map(|row| row.pitched_ordinal).collect())?]);
    }
    group.positive_origins.iter().map(|origin| {
        let ordinals = rows.iter().filter(|row| origin.members.contains(&(
            row.source_event_id, row.attack_id, row.fragment_id,
        ))).map(|row| row.pitched_ordinal).collect();
        bind(&origin.id, origin.measure, origin.members.len(), ordinals)
    }).collect()
}

pub(super) fn renderer_cohorts(
    source: &FinalPdfSourceOccurrenceContinuation,
) -> Vec<PrimaryBeamVoicePartitionRendererVerificationCohort> {
    source.receipt.original_beam_voice_groups.iter()
        .filter_map(|group| renderer_component_cohorts(source, group))
        .flatten().collect()
}

#[cfg(test)]
fn admitted_multi_measure_renderer_cohorts(
    source: &FinalPdfSourceOccurrenceContinuation,
    ids: &[String],
) -> io::Result<Vec<PrimaryBeamVoicePartitionRendererVerificationCohort>> {
    let mut cohorts = Vec::new();
    let mut seen = BTreeSet::new();
    for id in ids {
        let group = exact_group(source, id)?;
        if group.is_measure_local() {
            return Err(invalid_alignment("multi-measure original beam lost its measure relation"));
        }
        let local = renderer_component_cohorts(source, group)
            .ok_or_else(|| invalid_alignment("multi-measure original beam lost exact renderer occurrences"))?;
        if !local.iter().all(|cohort| seen.insert(cohort.metric_line_id.clone())) {
            return Err(invalid_alignment("multi-measure original beam repeats a renderer identity"));
        }
        cohorts.extend(local);
    }
    Ok(cohorts)
}

fn invalid_alignment(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn exact_group<'a>(
    source: &'a FinalPdfSourceOccurrenceContinuation,
    id: &str,
) -> io::Result<&'a OriginalBeamVoiceGroup> {
    let mut matches = source.receipt.original_beam_voice_groups.iter().filter(|group| group.id == id);
    let group = matches.next().ok_or_else(|| invalid_alignment("original beam origin group lost"))?;
    if matches.next().is_some() {
        return Err(invalid_alignment("original beam origin group is ambiguous"));
    }
    Ok(group)
}

fn pitched_note(
    measures: &[FinalChordBindingMeasure],
    measure: usize,
    ordinal: usize,
) -> Option<&FinalChordEncodingNote> {
    measures.iter().find(|row| row.measure == measure)?.notes.iter()
        .filter(|note| !note.rest_like && note.pitch_midi.is_some()).nth(ordinal)
}

pub(super) struct AlignmentInput<'a> {
    pub(super) source_xml: &'a str,
    pub(super) source: &'a FinalPdfSourceOccurrenceContinuation,
    pub(super) voices: &'a FrozenSerializedVoicePlan,
    pub(super) classification_xml: &'a str,
    pub(super) current: &'a [FinalChordBindingMeasure],
    pub(super) classification: &'a [FinalChordBindingMeasure],
    pub(super) visible_fragments: &'a BTreeMap<String, usize>,
}

pub(super) struct AlignmentOutput<'a> {
    pub(super) bridge_proofs: &'a mut Vec<rest_bridge_extension_course::BridgeAlignmentProof>,
    pub(super) grace_hosts: &'a mut Vec<OriginalBeamGraceHost>,
    pub(super) repaired: &'a mut FrozenSerializedVoicePlan,
    pub(super) edits: &'a mut Vec<PrimaryBeamVoicePartitionBoundEdit>,
    pub(super) selected: &'a mut Vec<PrimaryBeamVoicePartitionSelectedMember>,
}

pub(super) fn append_proven_alignment(
    input: AlignmentInput<'_>,
    output: AlignmentOutput<'_>,
) -> io::Result<Vec<String>> {
    let AlignmentInput {
        source_xml,
        source,
        voices,
        classification_xml,
        current,
        classification,
        visible_fragments,
    } = input;
    let AlignmentOutput {
        bridge_proofs,
        grace_hosts,
        repaired,
        edits,
        selected,
    } = output;
    if !source.has_authentic_beam_surface()? {
        return Ok(Vec::new());
    }
    let claimed = selected
        .iter()
        .map(|row| (row.source_event_id, row.attack_id))
        .collect::<BTreeSet<_>>();
    let Ok(baseline) = batch_occupancy_baseline(source_xml, current) else {
        return Ok(Vec::new());
    };
    if validate_batch_against_baseline(source_xml, current, &baseline, edits).is_err() {
        return Ok(Vec::new());
    }
    // MuseScore's voice slots are allocated over the whole part. Closing a
    // split original component can expose another original group sharing its
    // import labels, although the initial PDF happened to draw that group
    // connected. The resource closure starts only at an actually witnessed
    // split (or the already authenticated metric partition); it never creates
    // a new musical beam group from XML scalars or a report count.
    let mut component_labels = BTreeSet::<String>::new();
    for member in selected.iter() {
        if let Some(label) = voices.label_for_source(member.source_event_id) {
            component_labels.insert(label.to_owned());
        }
        component_labels.insert(member.target_label.clone());
    }
    let mut group_labels = BTreeMap::new();
    for group in source.receipt.original_beam_voice_groups.iter().filter(|group| group.is_measure_local()) {
        let Some(rows) = exact_members(source, group) else { continue; };
        let Some(labels) = rows.iter().map(|row| pitched_note(current, row.measure, row.pitched_ordinal)
            .map(|note| note.voice.trim().to_owned())).collect::<Option<BTreeSet<_>>>() else { continue; };
        if group.observed_fragments(visible_fragments, source, current).is_some_and(|count| count > group.primary_group_count) {
            component_labels.extend(labels.iter().cloned());
        }
        group_labels.insert(group.id.clone(), labels);
    }
    loop {
        let before = component_labels.len();
        for labels in group_labels.values() {
            if !labels.is_disjoint(&component_labels) {
                component_labels.extend(labels.iter().cloned());
            }
        }
        if component_labels.len() == before { break; }
    }
    let mut proposals = Vec::new();
    for group in &source.receipt.original_beam_voice_groups {
        let measure_local = group.is_measure_local();
        let bridges = source.receipt.rest_bridge_extension_course.as_ref()
            .filter(|_| measure_local && voices.has_canonical_role_allocation()).map_or_else(Vec::new,
            |course| course.alignment_resources(source.xml(), &group.source_resources,
                &group.bridge_component_occurrences(), &group.notation_voices,
                &group.semantic_voice, (group.measure, group.staff)));
        let bridges = bridges.into_iter().filter(|proof| proof.head_sources().all(|id|
            voices.role_for_source(id).is_some_and(|role|
                channel_role_staff(role) == group.staff))).collect::<Vec<_>>();
        let bridge_split = bridges.iter().any(|proof| proof.split_labels());
        let observed_fragments = group.observed_fragments(visible_fragments, source, current);
        let observe = |reason: &str| {
            let unusual = beam_diagnostics::record(Observation::Decision(reason));
            if unusual || beam_diagnostics::detail_requested_for_measure(group.measure) {
                eprintln!("OriginalBeamAlignmentDecision: group={}; measure={}; staff={}; primary_groups={}; visible_fragments={:?}; reason={reason}; phase=prepared_not_committed; authority=none", group.id, group.measure, group.staff, group.primary_group_count, observed_fragments);
            }
        };
        let import_component = measure_local && group_labels.get(&group.id)
            .is_some_and(|labels| labels.len() > 1 && !labels.is_disjoint(&component_labels));
        if observed_fragments.is_none() && !import_component && !bridge_split { observe("geometry_unknown"); continue; }
        if observed_fragments.is_some_and(|count| count <= group.primary_group_count) && !import_component && !bridge_split { observe("original_components_preserved"); continue; }
        if import_component { observe("original_import_resource_component"); }
        if group.members.iter().any(|member| claimed.contains(member)) { observe("existing_partition_claim"); continue; }
        let Some(rows) = exact_members(source, group) else {
            observe("source_closure"); continue;
        };
        let mut notes = Vec::new();
        let mut complete = true;
        for (index, row) in rows.iter().enumerate() {
            let (Some(note), Some(classified_note)) = (
                pitched_note(current, row.measure, row.pitched_ordinal),
                pitched_note(classification, row.measure, row.pitched_ordinal),
            ) else {
                complete = false;
                break;
            };
            if note.chord_member
                || note.rest_like
                || note.duration <= 0
                || note.staff.trim().parse::<u8>().ok() != Some(group.staff)
                || classified_note.chord_member
                || classified_note.staff != note.staff
                || classified_note.onset != note.onset
                || classified_note.duration != note.duration
                || classified_note.pitch_identity != note.pitch_identity
                || !same_owned_primary_level(source_xml, note, &group.levels[index])
                || !same_owned_primary_level(classification_xml, classified_note, &group.levels[index])
                || !same_actual_beam_surface(source_xml, note, classification_xml, classified_note)
                || voices.label_for_source(row.source_event_id) != Some(note.voice.trim())
            {
                eprintln!("OriginalBeamMemberRejection: group={}; member={index}; source_chord={}; classified_chord={}; source_onset={}; classified_onset={}; source_duration={}; classified_duration={}; source_voice={}; sealed_voice={:?}; owned_levels={:?}; source_levels_match={}; classified_levels_match={}; source_note={}; classified_note={}; authority=none", group.id, note.chord_member, classified_note.chord_member, note.onset, classified_note.onset, note.duration, classified_note.duration, note.voice, voices.label_for_source(row.source_event_id), group.levels[index], same_owned_levels(source_xml,note,&group.levels[index]),same_owned_levels(classification_xml,classified_note,&group.levels[index]), source_xml[note.range.clone()].replace(['\r','\n']," "), classification_xml[classified_note.range.clone()].replace(['\r','\n']," "));
                complete = false;
                break;
            }
            notes.push(note);
        }
        if !complete
            // A rest gap does not rescind the original owner's explicit
            // group. Pitched occupancy may not overlap; the full batch still
            // protects every visible bystander rest and channel resource.
            || notes.windows(2).any(|pair| pair[0].measure == pair[1].measure
                && pair[0].onset.checked_add(pair[0].duration)
                    .is_none_or(|end| end > pair[1].onset))
        {
            eprintln!("OriginalBeamComponentRejection: group={}; complete={complete}; members={:?}; authority=none", group.id, notes.iter().map(|n|(n.onset,n.duration,n.voice.as_str())).collect::<Vec<_>>());
            observe("member_levels_timing_roles_or_single_label"); continue;
        }
        let expected_primary = match group.staff {
            1 => SerializedVoicePresentationRole::UpperPrimary,
            2 => SerializedVoicePresentationRole::LowerPrimary,
            _ => continue,
        };
        let expected_secondary = match group.staff {
            1 => SerializedVoicePresentationRole::UpperSecondary,
            _ => SerializedVoicePresentationRole::LowerSecondary,
        };
        // Technical target alternatives come only from this actual original
        // component's existing channels. Source roles stay immutable. Prefer
        // the old congruent primary anchor; a safe existing secondary channel
        // can be necessary to preserve a partwide import registration resource.
        let channel_role = |note: &FinalChordEncodingNote| voices.channel_for_label(note.voice.trim())
            .and_then(|channel| voices.channel_to_role.get(&channel)).copied();
        let anchor = |role| group.members.iter().zip(&notes).position(|(&(id,_), note)|
            voices.role_for_source(id) == Some(role) && channel_role(note) == Some(role));
        let all_roles_admissible = group.members.iter().all(|&(id,_)|
            voices.role_for_source(id).is_some_and(|role| role == expected_primary || role == expected_secondary));
        let has_primary_source = group.members.iter().any(|&(id,_)| voices.role_for_source(id) == Some(expected_primary));
        let has_secondary_source = group.members.iter().any(|&(id,_)| voices.role_for_source(id) == Some(expected_secondary));
        let primary_channel = notes.iter().position(|note| channel_role(note) == Some(expected_primary));
        let all_congruent_secondary = group.members.iter().zip(&notes).all(|(&(id,_),note)|
            voices.role_for_source(id) == Some(expected_secondary) && channel_role(note) == Some(expected_secondary));
        if !all_roles_admissible { observe("foreign_source_role"); continue; }
        let mut targets = Vec::new();
        if let Some(index) = anchor(expected_primary) { targets.push(index); }
        // In a crossed source/channel relation the original component still
        // contains a primary source and already occupies a primary channel.
        // Its channel endpoint need not itself own the primary source role.
        if has_primary_source && let Some(index) = primary_channel
            && !targets.contains(&index) { targets.push(index); }
        if (all_congruent_secondary || (has_primary_source && has_secondary_source && primary_channel.is_some()))
            && let Some(index) = anchor(expected_secondary)
            && !targets.contains(&index) { targets.push(index); }
        // A genuine canonical allocation can already place a source role
        // in a different technical presentation-role channel. The additional
        // immutable allocator proof admits that existing channel without
        // deriving or changing any musical source role. A fabricated/mutated
        // source/channel-role map cannot create this additional class.
        if voices.has_canonical_role_allocation() {
            if let Some(index) = primary_channel
                && !targets.contains(&index) { targets.push(index); }
            if let Some(index) = notes.iter().position(|note| channel_role(note) == Some(expected_secondary))
                && !targets.contains(&index) { targets.push(index); }
        }
        // Different technical channels can carry the same presentation role.
        // Keep the established anchors first, then consider each remaining
        // channel already occupied by an original member of this component.
        if voices.has_canonical_role_allocation() {
            let mut seen = targets.iter().map(|&index| notes[index].voice.trim())
                .collect::<BTreeSet<_>>();
            for (index, note) in notes.iter().enumerate() {
                if channel_role(note).is_some_and(|role| role == expected_primary || role == expected_secondary)
                    && seen.insert(note.voice.trim())
                {
                    targets.push(index);
                }
            }
        }
        if targets.is_empty() {
            eprintln!("OriginalBeamAllocationRejection: group={}; canonical_allocation={}; source_and_channel_roles={:?}; authority=none",group.id,voices.has_canonical_role_allocation(),group.members.iter().zip(&notes).map(|(&(id,_),note)|(voices.role_for_source(id),channel_role(note))).collect::<Vec<_>>());
            observe("no_existing_congruent_staff_anchor"); continue;
        }
        let resource_context = BoundChordResourceContext {
            source, classification_xml, group, current, classification, voices, claimed: &claimed,
            bridge_occurrences: bridges.iter().flat_map(|proof| proof.occurrences()).collect(),
        };
        let Some(mut resources) = bound_chord_resources(&resource_context, &notes, &rows) else {
            observe("chord_resource_closure"); continue;
        };
        let mut hosts = Vec::new();
        if !measure_local {
            let Some((graces, proved_hosts)) = bound_graces(&resource_context, &resources) else {
                observe("grace_host_binding"); continue;
            };
            resources.extend(graces);
            hosts = proved_hosts;
        }
        if resources.iter().map(|(note,_)|note.voice.trim()).collect::<BTreeSet<_>>().len()<=1 {
            observe("complete_resource_image_already_one_label");continue;
        }
        let original_component = group.bridge_component_occurrences();
        let foreign_members = source.receipt.original_beam_voice_groups.iter()
            .flat_map(OriginalBeamVoiceGroup::bridge_component_occurrences)
            .filter(|identity| !original_component.contains(identity))
            .collect::<BTreeSet<_>>();
        let mut admitted_target = None;
        for target_index in targets {
            let target_label = notes[target_index].voice.trim();
            // Tied resources may already occupy this target. Their complete
            // custody stays bound, but a foreign original member may not move.
            if resources.iter().any(|(note, row)| {
                note.voice.trim() != target_label
                    && foreign_members.contains(&(row.source_event_id, row.attack_id, row.fragment_id))
            }) {
                observe("foreign_original_beam_member_would_move");
                continue;
            }
            let Some(target_channel) = voices.channel_for_label(target_label) else { continue; };
            let mut group_edits = Vec::new();
            for (note, row) in &resources {
                if note.voice.trim() == target_label { continue; }
                let (start,end) = primary_beam_voice_partition_voice_text_range(source_xml,note.range.clone())?;
                group_edits.push(PrimaryBeamVoicePartitionBoundEdit { measure: row.measure,
                    pitched_ordinal: row.pitched_ordinal, voice_text_start: start, voice_text_end: end,
                    expected_old_label: note.voice.trim().to_owned(), target_label: target_label.to_owned() });
            }
            let mut trial = edits.clone(); trial.extend(group_edits.clone());
            let (collisions, lost_registration) = new_batch_conflicts(source_xml, current, &baseline, &trial)?;
            let unusual = beam_diagnostics::record(Observation::Admission {
                scalar_changes: group_edits.len(),
                collisions: !collisions.is_empty(),
                lost_registration: !lost_registration.is_empty(),
            });
            if unusual || beam_diagnostics::detail_requested_for_measure(group.measure) {
                eprintln!("OriginalBeamTargetResourceAdmission: group={}; target={target_label}; scalar_changes={}; collisions={collisions:?}; lost_registration={lost_registration:?}; authority=none",group.id,group_edits.len());
            }
            // The initial batch check already proved both the unchanged
            // source and the existing edits admissible.
            let occupancy_admissible = collisions.is_empty() && lost_registration.is_empty();
            let trial_import_admissible = occupancy_admissible
                && projected_import_profile_admissible(source_xml, &trial);
            if beam_diagnostics::detail_requested_for_measure(group.measure) {
                eprintln!("BeamImportProfileTrial: group={}; measure={}; target={target_label}; existing_edits={}; candidate_edits={:?}; occupancy_admissible={}",
                    group.id, group.measure, edits.len(), group_edits.iter().map(|edit|
                        (edit.measure, edit.pitched_ordinal, &edit.expected_old_label, &edit.target_label)
                    ).collect::<Vec<_>>(), occupancy_admissible);
                eprintln!("BeamImportProfile: stage=existing_edits; source=true; projected=true");
                if occupancy_admissible {
                    eprintln!("BeamImportProfile: stage=trial; source=true; projected={trial_import_admissible}");
                }
            }
            // Each existing channel is assessed against the complete written
            // occupancy and import profile. A blocked channel can coexist
            // with another member channel that already has room for the group.
            if !collisions.is_empty() && lost_registration.is_empty() && !voices.has_canonical_role_allocation() {
                break;
            }
            if trial_import_admissible {
                let improves = admitted_target.as_ref().is_none_or(|(_,_,_,prior): &(usize,usize,String,Vec<PrimaryBeamVoicePartitionBoundEdit>)|group_edits.len()<prior.len());
                if improves { admitted_target = Some((target_index,target_channel,target_label.to_owned(),group_edits)); }
                // All options retain exactly the original musical membership.
                // With an authentic allocator choose the admissible technical
                // relation that changes the fewest complete resource scalars.
                // Ties keep the existing primary preference; no musical score
                // or opaque ID determines a grouping decision.
                if !voices.has_canonical_role_allocation() {break;}
            }
        }
        let Some((target_index,target_channel,target_label,group_edits)) = admitted_target else {
            observe("all_existing_channel_targets_violate_occupancy_or_import_profile"); continue;
        };
        observe("proposal");
        proposals.push((group,target_index,target_channel,target_label,group_edits,resources,bridges,hosts));
    }
    // All proposals are evaluated together. If simultaneous assignments
    // conflict, every proposal whose changed occurrence participates is
    // withdrawn. No opaque group ordering chooses a winner.
    loop {
        // Shared actual resources are a conflict group, never a ranked winner.
        let mut counts = BTreeMap::new();
        for (_,_,_,_,_,resources,_,_) in &proposals {
            for (_,row) in resources { *counts.entry((row.source_event_id,row.attack_id,row.fragment_id)).or_insert(0usize) += 1; }
        }
        proposals.retain(|(_,_,_,_,_,resources,_,_)| resources.iter().all(|(_,row)|
            counts[&(row.source_event_id,row.attack_id,row.fragment_id)] == 1));
        let mut batch = edits.clone();
        for (_, _, _, _, proposed, _, _, _) in &proposals {
            batch.extend(proposed.clone());
        }
        let (conflicts, headless) = new_batch_conflicts(source_xml, current, &baseline, &batch)?;
        if conflicts.is_empty() && headless.is_empty() {
            break;
        }
        let before = proposals.len();
        proposals.retain(|(_, _, _, _, proposed, _, _, _)| {
            !proposed.iter().any(|edit| {
                if headless.contains(&edit.expected_old_label) {
                    return true;
                }
                let raw = current
                    .iter()
                    .find(|m| m.measure == edit.measure)
                    .and_then(|m| {
                        m.notes
                            .iter()
                            .filter(|n| !n.rest_like && n.pitch_midi.is_some())
                            .nth(edit.pitched_ordinal)
                    })
                    .map(|n| n.raw_note_index);
                raw.is_some_and(|raw| {
                    conflicts.iter().any(|&(measure, left, right)| {
                        measure == edit.measure && (left == raw || right == raw)
                    })
                })
            })
        });
        if proposals.len() == before {
            return Ok(Vec::new());
        }
    }
    let mut admitted = Vec::new();
    for (group, target_index, target_channel, target_label, proposed, resources, proofs, hosts) in proposals {
        bridge_proofs.extend(proofs);
        grace_hosts.extend(hosts);
        edits.extend(proposed);
        let target = group.members[target_index];
        for (_, resource) in resources {
            let id = resource.source_event_id;
            let attack = resource.attack_id;
            repaired.source_to_channel.insert(id, target_channel);
            repaired.source_to_label.insert(id, target_label.clone());
            selected.push(PrimaryBeamVoicePartitionSelectedMember {
                source_event_id: id,
                attack_id: attack,
                fragment_id: resource.fragment_id,
                target_primary_fragment_id: group.fragments[target_index],
                target_primary_source_event_id: target.0,
                target_primary_attack_id: target.1,
                target_channel,
                target_label: target_label.clone(),
            });
        }
        admitted.push(group.id.clone());
    }
    Ok(admitted)
}

fn occupancy_conflicts(
    xml: &str,
    measures: &[FinalChordBindingMeasure],
    labels: &BTreeMap<(usize, usize), String>,
) -> io::Result<BTreeSet<(usize, usize, usize)>> {
    let mut result = BTreeSet::new();
    for measure in measures {
        let heads = measure
            .notes
            .iter()
            .filter(|note| {
                !note.chord_member
                    && note.duration > 0
                    && !xml[note.range.clone()].contains("print-spacing=\"no\"")
            })
            .collect::<Vec<_>>();
        for (index, left) in heads.iter().enumerate() {
            for right in heads.iter().skip(index + 1) {
                let left_voice = labels
                    .get(&(measure.measure, left.raw_note_index))
                    .map_or(left.voice.trim(), String::as_str);
                let right_voice = labels
                    .get(&(measure.measure, right.raw_note_index))
                    .map_or(right.voice.trim(), String::as_str);
                let left_end = left.onset.checked_add(left.duration).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "beam occupancy interval overflow",
                    )
                })?;
                let right_end = right.onset.checked_add(right.duration).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "beam occupancy interval overflow",
                    )
                })?;
                if left_voice == right_voice && left.onset < right_end && right.onset < left_end {
                    result.insert((measure.measure, left.raw_note_index, right.raw_note_index));
                }
            }
        }
    }
    Ok(result)
}

// Both sets describe the same immutable source before any trial relabeling.
fn batch_occupancy_baseline(
    source: &str,
    measures: &[FinalChordBindingMeasure],
) -> io::Result<ChannelSafety> {
    let labels = BTreeMap::new();
    Ok((
        occupancy_conflicts(source, measures, &labels)?,
        headless_labels_from_measures(source, measures, &labels),
    ))
}

fn new_batch_conflicts(
    source: &str,
    measures: &[FinalChordBindingMeasure],
    baseline: &ChannelSafety,
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> io::Result<ChannelSafety> {
    let (before_conflicts, before_headless) = baseline;
    let mut labels = BTreeMap::new();
    let mut edits = edits.iter().collect::<Vec<_>>();
    edits.sort_by_key(|edit| edit.voice_text_start);
    if edits
        .windows(2)
        .any(|pair| pair[0].voice_text_end > pair[1].voice_text_start)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "original beam alignment scalar edits overlap",
        ));
    }
    for edit in edits {
        if source
            .get(edit.voice_text_start..edit.voice_text_end)
            .map(str::trim)
            != Some(edit.expected_old_label.as_str())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "original beam alignment scalar precondition changed",
            ));
        }
        let note = measures
            .iter()
            .find(|m| m.measure == edit.measure)
            .and_then(|m| {
                m.notes
                    .iter()
                    .filter(|n| !n.rest_like && n.pitch_midi.is_some())
                    .nth(edit.pitched_ordinal)
            })
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "original beam scalar has no exact pitched occurrence",
                )
            })?;
        if primary_beam_voice_partition_voice_text_range(source, note.range.clone())?
            != (edit.voice_text_start, edit.voice_text_end)
            || labels
                .insert(
                    (note.measure, note.raw_note_index),
                    edit.target_label.clone(),
                )
                .is_some()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "original beam scalar footprint differs from its occurrence",
            ));
        }
    }
    let new_headless = headless_labels_from_measures(source, measures, &labels)
        .difference(before_headless)
        .cloned()
        .collect();
    Ok((
        occupancy_conflicts(source, measures, &labels)?
            .difference(before_conflicts)
            .copied()
            .collect(),
        new_headless,
    ))
}

pub(super) fn validate_batch_occupancy(
    source: &str,
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> io::Result<()> {
    let measures = final_chord_parse_binding_measures(source)?;
    validate_batch_occupancy_with_measures(source, &measures, edits)
}

pub(super) fn validate_batch_occupancy_with_measures(
    source: &str,
    measures: &[FinalChordBindingMeasure],
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> io::Result<()> {
    let baseline = batch_occupancy_baseline(source, measures)?;
    validate_batch_against_baseline(source, measures, &baseline, edits)
}

fn validate_batch_against_baseline(
    source: &str,
    measures: &[FinalChordBindingMeasure],
    baseline: &ChannelSafety,
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> io::Result<()> {
    let (conflicts, headless) = new_batch_conflicts(source, measures, baseline, edits)?;
    if !conflicts.is_empty() || !headless.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "original beam alignment introduces a channel occupancy conflict",
        ));
    }
    if !crate::audit_only::import_channel_profile_readback::partwide_import_channels_admissible(source)
        || !projected_import_profile_admissible(source, edits)
    {
        return Err(io::Error::new(io::ErrorKind::InvalidData,
            "original beam alignment violates the complete partwide import-channel profile"));
    }
    Ok(())
}

fn projected_import_profile_admissible(
    source: &str,
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> bool {
    let mut projected = source.to_owned();
    let mut ordered = edits.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|edit| std::cmp::Reverse(edit.voice_text_start));
    for edit in ordered {
        projected.replace_range(edit.voice_text_start..edit.voice_text_end, &edit.target_label);
    }
    crate::audit_only::import_channel_profile_readback::partwide_import_channels_admissible(&projected)
}

pub(super) fn validate_late_groups(
    xml: &str,
    source: &FinalPdfSourceOccurrenceContinuation,
    ids: &[String],
) -> io::Result<()> {
    if !source.has_authentic_beam_surface()? {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "original beam course surface was changed outside its transport"));
    }
    let source_measures = final_chord_parse_binding_measures(source.xml())?;
    let measures = final_chord_parse_binding_measures(xml)?;
    for id in ids {
        let group = exact_group(source, id)?;
        let rows = exact_members(source, group)
            .ok_or_else(|| invalid_alignment("original beam late occurrence ambiguity"))?;
        for (index, row) in rows.iter().enumerate() {
            let note = pitched_note(&measures, row.measure, row.pitched_ordinal)
                .ok_or_else(|| invalid_alignment("original beam late note missing"))?;
            let original_note = pitched_note(&source_measures, row.measure, row.pitched_ordinal)
                .ok_or_else(|| invalid_alignment("original beam course lost its source note"))?;
            if !same_owned_primary_level(xml, note, &group.levels[index])
                || !same_actual_beam_surface(source.xml(), original_note, xml, note) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "original beam decision changed before late voice write",
                ));
            }
        }
    }
    Ok(())
}

// Original primary membership remains the sole musical grouping decision.
// Later accepted owners may evolve secondary beams. Those current decisions are
// carried unchanged, not inferred anew or reset to an obsolete early surface.
pub(super) fn actual_beam_levels(xml: &str, note: &FinalChordEncodingNote) -> Option<BTreeMap<u8, String>> {
    let doc = roxmltree::Document::parse(&xml[note.range.clone()]).ok()?;
    let mut levels = BTreeMap::new();
    for beam in doc.root_element().children().filter(|node| node.has_tag_name("beam")) {
        let level = beam.attribute("number").unwrap_or("1").parse::<u8>().ok()?;
        if levels.insert(level, beam.text().unwrap_or_default().trim().to_owned()).is_some() { return None; }
    }
    Some(levels)
}

fn same_owned_primary_level(xml: &str, note: &FinalChordEncodingNote, levels: &crate::pipeline::ActualBeamLevels) -> bool {
    let Some(written) = actual_beam_levels(xml, note) else { return false; };
    let Some((_, _, state)) = levels.iter().find(|(level, _, _)| *level == 1) else { return false; };
    // An accepted later owner may extend a boundary while retaining the
    // original group's positive incoming/outgoing edge. Its actual full
    // current surface remains frozen by the occurrence-course guard.
    matches!((state.as_deref(), written.get(&1).map(String::as_str)),
        (Some("begin"),Some("begin" | "continue"))
        | (Some("end"),Some("end" | "continue"))
        | (Some("continue"),Some("continue")) | (None,None))
}

fn same_actual_beam_surface(before: &str, a: &FinalChordEncodingNote, after: &str, b: &FinalChordEncodingNote) -> bool {
    actual_beam_levels(before, a).zip(actual_beam_levels(after, b)).is_some_and(|(a,b)| a == b)
}

fn same_owned_levels(
    xml: &str,
    note: &FinalChordEncodingNote,
    levels: &crate::pipeline::ActualBeamLevels,
) -> bool {
    let Ok(doc) = roxmltree::Document::parse(&xml[note.range.clone()]) else {
        return false;
    };
    let mut written = BTreeMap::new();
    for beam in doc
        .root_element()
        .children()
        .filter(|n| n.has_tag_name("beam"))
    {
        let Some(level) = beam.attribute("number").unwrap_or("1").parse::<u8>().ok() else {
            return false;
        };
        if written
            .insert(level, beam.text().unwrap_or_default().trim())
            .is_some()
        {
            return false;
        }
    }
    if written
        .keys()
        .any(|level| !levels.iter().any(|(owned, _, _)| owned == level))
    {
        return false;
    }
    levels
        .iter()
        .all(|(level, _, state)| written.get(level).copied() == state.as_deref())
}

#[cfg(test)]
fn headless_labels(
    xml: &str,
    labels: &BTreeMap<(usize, usize), String>,
) -> io::Result<BTreeSet<String>> {
    let measures = final_chord_parse_binding_measures(xml)?;
    Ok(headless_labels_from_measures(xml, &measures, labels))
}

fn headless_labels_from_measures(
    xml: &str,
    measures: &[FinalChordBindingMeasure],
    labels: &BTreeMap<(usize, usize), String>,
) -> BTreeSet<String> {
    let mut registered = BTreeSet::new();
    let mut required = BTreeSet::new();
    for measure in measures {
        for note in &measure.notes {
            if note.duration <= 0 || xml[note.range.clone()].contains("print-spacing=\"no\"") {
                continue;
            }
            let label = labels
                .get(&(measure.measure, note.raw_note_index))
                .map_or(note.voice.trim(), String::as_str)
                .to_owned();
            if !note.chord_member {
                registered.insert(label.clone());
            }
            if !note.rest_like && note.pitch_midi.is_some() {
                required.insert(label);
            }
        }
    }
    required.difference(&registered).cloned().collect()
}

#[cfg(test)]
pub(crate) fn original_beam_alignment_test_fixture(
    input: &str,
) -> (
    CanonicalFinalChordStructuralPlan,
    FrozenSerializedVoicePlan,
    FinalPdfSourceOccurrenceContinuation,
    PrimaryBeamVoicePartitionBoundPlan,
) {
    let (canonical, voices, _) = tests::fixture_from_input(input);
    // Opaque label permutation makes the existing rest lane the primary lane;
    // the test does not depend on a numeric channel convention.
    let labels = voices
        .channel_to_label
        .values()
        .rev()
        .cloned()
        .collect::<Vec<_>>();
    let voices = final_chord_permute_sealed_voice_labels(&voices, &labels).unwrap();
    let original = canonical
        .written_projection
        .as_ref()
        .unwrap()
        .original_xml
        .as_ref();
    let projected = project_canonical_final_chord_render_occurrence_continuation(
        original,
        &canonical,
        &voices,
        &BTreeSet::new(),
    )
    .unwrap();
    let source = FinalPdfSourceOccurrenceContinuation::from_original_transport(projected.xml().to_owned(), projected.projected.receipt.clone()).unwrap();
    // Supply explicit stale display anchors through the existing scalar owner,
    // representing upstream glyph placement before the final lane is complete.
    let document = roxmltree::Document::parse(source.xml()).unwrap();
    let replacements = document
        .descendants()
        .filter(|node| node.has_tag_name("note"))
        .filter(|note| note.children().any(|node| node.has_tag_name("rest")))
        .map(|note| {
            let range = note.range();
            let replacement = source.xml()[range.clone()].replace(
                "<rest/>",
                "<rest><display-step>D</display-step><display-octave>3</display-octave></rest>",
            );
            (range.start, range.end, replacement)
        })
        .collect();
    let (with_display, receipt) =
        crate::structural_write_authority::apply_final_rest_glyph_height_authorized(
            source.xml(),
            replacements,
        );
    let with_display = with_display.unwrap();
    assert_eq!(
        receipt
            .unwrap()
            .realize_post_document(source.xml())
            .as_deref(),
        Some(with_display.as_str())
    );
    let source = source
        .through_final_clef_and_rest_placement(
            source.xml(),
            &with_display,
            &canonical,
            &voices,
            Some(&BTreeSet::new()),
        )
        .unwrap();
    let prepared = tests::plan(&voices, &source, 4);
    (canonical, voices, source, prepared)
}

// Attached chord notes are physical resources of an already authenticated head,
// not additional original beam members. Their opaque occurrence identities come
// exclusively from the existing source continuation. The complete source image
// must remain inside this closed component; no convenient single copy is selected.
struct BoundChordResourceContext<'a> {
    source: &'a FinalPdfSourceOccurrenceContinuation,
    classification_xml: &'a str,
    group: &'a OriginalBeamVoiceGroup,
    current: &'a [FinalChordBindingMeasure],
    classification: &'a [FinalChordBindingMeasure],
    voices: &'a FrozenSerializedVoicePlan,
    claimed: &'a BTreeSet<(usize, usize)>,
    bridge_occurrences: BTreeSet<(usize, usize, usize)>,
}
fn bound_chord_resources<'a>(
    context: &BoundChordResourceContext<'a>,
    heads: &[&'a FinalChordEncodingNote],
    rows: &[&'a FinalChordTransactionOutputOccurrenceWitness],
) -> Option<Vec<BoundNote<'a>>> {
    let BoundChordResourceContext { source, classification_xml, group, current, classification, voices, claimed, ref bridge_occurrences } = *context;
    if bridge_occurrences.iter().any(|identity| source.receipt.occurrences.iter()
        .filter(|row| (row.source_event_id,row.attack_id,row.fragment_id) == *identity).count() != 1) {
        return None;
    }
    let mut resources=heads.iter().copied().zip(rows.iter().copied()).collect::<Vec<_>>();
    loop {
        let before=resources.len();
        let keys=resources.iter().map(|(_,r)|(r.source_event_id,r.attack_id)).collect::<BTreeSet<_>>();
        let mut required=source.receipt.occurrences.iter().filter(|r|keys.contains(&(r.source_event_id,r.attack_id)))
            .collect::<Vec<_>>();
        required.extend(source.receipt.occurrences.iter().filter(|row|
            bridge_occurrences.contains(&(row.source_event_id,row.attack_id,row.fragment_id))));
        // Physical chord attachment is a resource relation. It does not create
        // another original positive group or erase any negative beam decision.
        for (head,_) in resources.iter().filter(|(n,_)|!n.chord_member) {
            let measure=current.iter().find(|m|m.measure==head.measure)?;
            for (ordinal,_) in measure.notes.iter().filter(|n|!n.rest_like && n.pitch_midi.is_some()).enumerate()
                .filter(|(_,n)|n.chord_member && n.anchor_index==Some(head.raw_note_index)) {
                let matches=source.receipt.occurrences.iter().filter(|r|r.measure==head.measure && r.pitched_ordinal==ordinal).collect::<Vec<_>>();
                let [row]=matches.as_slice() else { return None; };required.push(*row);
            }
        }
        let mut pending=std::collections::VecDeque::from(required);
        while let Some(row)=pending.pop_front() {
            let identity=(row.source_event_id,row.attack_id,row.fragment_id);
            if resources.iter().any(|(_,r)|(r.source_event_id,r.attack_id,r.fragment_id)==identity) {continue;}
            let measure=current.iter().find(|m|m.measure==row.measure)?;
            let classified=classification.iter().find(|m|m.measure==row.measure)?;
            let pitched=measure.notes.iter().filter(|n|!n.rest_like && n.pitch_midi.is_some()).collect::<Vec<_>>();
            let observed=classified.notes.iter().filter(|n|!n.rest_like && n.pitch_midi.is_some()).collect::<Vec<_>>();
            let member=*pitched.get(row.pitched_ordinal)?;let other=*observed.get(row.pitched_ordinal)?;
            if !group.source_resources.iter().any(|loc| (loc.source_event_id,loc.attack_id,loc.fragment_id)==identity)
                || member.duration<=0 || member.chord_member!=other.chord_member
                || member.staff!=other.staff || member.onset!=other.onset || member.duration!=other.duration
                || member.pitch_identity!=other.pitch_identity || member.anchor_index!=other.anchor_index
                || !same_actual_beam_surface(source.xml(),member,classification_xml,other)
                || voices.label_for_source(row.source_event_id)!=Some(member.voice.trim())
                || claimed.contains(&(row.source_event_id,row.attack_id)) {return None;}
            let role=voices.role_for_source(row.source_event_id)?;
            if member.chord_member {
                let head_resource=resources.iter().find(|(n,_)|n.measure==member.measure && Some(n.raw_note_index)==member.anchor_index);
                let Some((head,head_row))=head_resource else {
                    let head_index=pitched.iter().position(|n|Some(n.raw_note_index)==member.anchor_index && !n.chord_member)?;
                    let candidates=source.receipt.occurrences.iter().filter(|r|r.measure==member.measure && r.pitched_ordinal==head_index).collect::<Vec<_>>();
                    let [parent]=candidates.as_slice() else {return None;};
                    pending.push_front(row);pending.push_front(*parent);continue;
                };
                if member.onset!=head.onset || member.staff!=head.staff || member.voice!=head.voice
                    || channel_role_staff(role)!=channel_role_staff(voices.role_for_source(head_row.source_event_id)?) {return None;}
            } else if !voices.has_canonical_role_allocation() {return None;}
            resources.push((member,row));
        }
        if resources.len()==before {break;}
    }
    let identities=resources.iter().map(|(_,r)|(r.source_event_id,r.attack_id,r.fragment_id)).collect::<BTreeSet<_>>();
    if identities.len()!=resources.len() {return None;}
    let keys=resources.iter().map(|(_,r)|(r.source_event_id,r.attack_id)).collect::<BTreeSet<_>>();
    let actual=source.receipt.occurrences.iter().filter(|r|keys.contains(&(r.source_event_id,r.attack_id)))
        .map(|r|(r.source_event_id,r.attack_id,r.fragment_id)).collect::<Vec<_>>();
    let expected=group.source_resources.iter().filter(|r|keys.contains(&(r.source_event_id,r.attack_id))
        && !source.receipt.certified_omitted_fragment_ids.contains(&r.fragment_id))
        .map(|r|(r.source_event_id,r.attack_id,r.fragment_id)).collect::<BTreeSet<_>>();
    if identities!=expected || actual.len()!=identities.len() || actual.iter().any(|id|!identities.contains(id)) {return None;}
    Some(resources)
}

// In this narrow form MusicXML has an explicit grace immediately followed by
// its principal head, with one cursor, staff, onset and old voice for both.
// A different attachment syntax remains outside the write scope.
fn directly_attached_graces<'a>(
    xml: &str,
    measure: &'a FinalChordBindingMeasure,
    host: &FinalChordEncodingNote,
) -> Option<Vec<&'a FinalChordEncodingNote>> {
    let end = measure.notes.iter().position(|nota| nota.raw_note_index == host.raw_note_index)?;
    let mut head_start = host.range.start;
    let mut graces = Vec::new();
    for note in measure.notes[..end].iter().rev().take_while(|note| note.duration == 0) {
        if note.staff != host.staff || note.onset != host.onset || note.voice != host.voice
            || note.rest_like || note.chord_member || note.pitch_midi.is_none()
            || note.tie_start || note.tie_stop
            || !xml.get(note.range.end..head_start)?.trim().is_empty()
        {
            return None;
        }
        let doc = roxmltree::Document::parse(xml.get(note.range.clone())?).ok()?;
        let glyph = doc.root_element();
        if glyph.children().filter(|child| child.has_tag_name("grace")).count() != 1
            || glyph.children().any(|child| child.has_tag_name("duration") || child.has_tag_name("beam"))
        {
            return None;
        }
        head_start = note.range.start;
        graces.push(note);
    }
    Some(graces)
}

fn bound_graces<'a>(
    context: &BoundChordResourceContext<'a>,
    resources: &[BoundNote<'_>],
) -> Option<(Vec<BoundNote<'a>>, Vec<OriginalBeamGraceHost>)> {
    let mut graces = Vec::new();
    let mut links = Vec::new();
    let source = context.source;
    for (host, host_row) in resources.iter().filter(|(note, _)| !note.chord_member) {
        let measure = context.current.iter().find(|measure| measure.measure == host.measure)?;
        let classified = context.classification.iter().find(|measure| measure.measure == host.measure)?;
        let classified_host = pitched_note(context.classification, host.measure, host_row.pitched_ordinal)?;
        let before = directly_attached_graces(source.xml(), measure, host)?;
        let shadow = directly_attached_graces(context.classification_xml, classified, classified_host)?;
        if before.len() != shadow.len() {
            return None;
        }
        for (grace, observed) in before.into_iter().zip(shadow) {
            if grace.pitch_identity != observed.pitch_identity
                || grace.staff != observed.staff || grace.onset != observed.onset
                || source.xml().get(grace.range.clone())? != context.classification_xml.get(observed.range.clone())?
            {
                return None;
            }
            let ordinal = measure.notes.iter().filter(|note| !note.rest_like && note.pitch_midi.is_some())
                .position(|note| note.raw_note_index == grace.raw_note_index)?;
            let mut rows = source.receipt.occurrences.iter()
                .filter(|row| row.measure == grace.measure && row.pitched_ordinal == ordinal);
            let row = rows.next()?;
            if rows.next().is_some() || context.claimed.contains(&(row.source_event_id, row.attack_id))
                || context.voices.label_for_source(row.source_event_id) != Some(grace.voice.trim())
                || context.voices.role_for_source(row.source_event_id).map(channel_role_staff) != Some(context.group.staff)
                || source.receipt.occurrences.iter().filter(|other| other.source_event_id == row.source_event_id).count() != 1
                || !context.group.source_resources.iter().any(|locator| {
                    (locator.source_event_id, locator.attack_id, locator.fragment_id) == occurrence_key(row)
                        && locator.duration == 0
                })
            {
                return None;
            }
            graces.push((grace, row));
            links.push(OriginalBeamGraceHost { grace: occurrence_key(row), host: occurrence_key(host_row) });
        }
    }
    Some((graces, links))
}

pub(super) fn validate_late_grace_hosts(
    source: &FinalPdfSourceOccurrenceContinuation,
    measures: &[FinalChordBindingMeasure],
    selected: &[PrimaryBeamVoicePartitionSelectedMember],
    links: &[OriginalBeamGraceHost],
) -> io::Result<()> {
    let row = |identity| {
        let mut found = source.receipt.occurrences.iter().filter(|row| occurrence_key(row) == identity);
        let member = found.next()?;
        found.next().is_none().then_some(member)
    };
    let target = |identity| {
        let mut found = selected.iter().filter(|item| {
            (item.source_event_id, item.attack_id, item.fragment_id) == identity
        });
        let member = found.next()?;
        found.next().is_none().then_some(member.target_label.as_str())
    };
    for link in links {
        let valid = (|| {
            let host = row(link.host)?;
            let grace = row(link.grace)?;
            let measure = measures.iter().find(|measure| measure.measure == host.measure)?;
            let principal = pitched_note(measures, host.measure, host.pitched_ordinal)?;
            let approach = pitched_note(measures, grace.measure, grace.pitched_ordinal)?;
            let label = target(link.host)?;
            (host.measure == grace.measure && label == target(link.grace)?
                && directly_attached_graces(source.xml(), measure, principal)?
                    .iter().any(|note| note.raw_note_index == approach.raw_note_index)).then_some(())
        })();
        if valid.is_none() {
            return Err(invalid_alignment("original beam grace lost its exact principal attachment"));
        }
    }
    Ok(())
}


#[cfg(test)]
pub(super) fn bridge_test_actual_alignment_plan(
    voices:&FrozenSerializedVoicePlan,source:&FinalPdfSourceOccurrenceContinuation,
) -> PrimaryBeamVoicePartitionBoundPlan { tests::plan(voices,source,4) }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_notation_component_is_allocated_before_voice_materialization() {
        for (scope, copies, staff, competing_rest) in [(37usize, 1usize, 1u8, false), (137, 1, 1, false), (237, 2, 1, false), (337, 1, 2, false), (437, 2, 2, false), (537, 1, 1, true)] {
            let mut input = crate::pipeline::actual_beam_owner_capture_tests::input()
                .replace("probe", &scope.to_string())
                .replace("<voice>3</voice>", &format!("<voice>{staff}</voice>"));
            if staff == 2 {
                input = input.replace("<staff>1</staff>", "<staff>2</staff>")
                    .replace("<clef><sign>G</sign><line>2</line></clef>",
                        "<staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef>");
            }
            if competing_rest {
                let notes = ["C", "D", "E", "F"].into_iter().cycle().take(16).map(|step|
                    format!("<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>")).collect::<String>();
                input = format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{scope}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes>{notes}<backup><duration>384</duration></backup><note><rest measure=\"yes\"/><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note></measure></part></score-partwise>");
            }
            if copies == 2 {
                let doc = roxmltree::Document::parse(&input).unwrap();
                let measure = doc.descendants().find(|node| node.has_tag_name("measure")).unwrap().range();
                let duplicate = input[measure].replace(&format!("number=\"{scope}\""), &format!("number=\"{}\"", scope + 1));
                input.insert_str(input.find("</part>").unwrap(), &duplicate);
            }
            let (original, receipt, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&input);
            capture.binds_original_receipt(&receipt).unwrap();
            let evidence = final_chord_declassified_evidence_from_xml(&original).unwrap();
            let (mut canonical, _) = plan_canonical_final_chord_structure_once(&original, &evidence).unwrap();
            let notation = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
                &original, canonical.source_locators.iter().map(|locator|
                    ((locator.source_event_id, locator.attack_id), "independent-line-name".to_owned())));
            canonical.bind_original_beam_voice_groups(&original, Some(&Ok(capture.clone())), &notation);
            let observation = super::super::actual_source_beam_binding::observe_actual_source_beam_binding(
                &canonical, &original, Some(&Ok(capture.clone())));
            let mut offset = 0;
            let offsets = canonical.measures.iter().map(|measure| {
                let start = offset; offset += measure.expected_duration; (measure.measure, start)
            }).collect::<BTreeMap<_, _>>();
            let mut attacks = BTreeMap::new();
            for locator in canonical.source_locators.iter() {
                let lane = 1 + attacks.len() % 2;
                attacks.entry(locator.source_event_id).or_insert(FrozenVoiceAttackEvidenceRow {
                    source_event_id: locator.source_event_id, attack_id: locator.attack_id,
                    absolute_start: offsets[&locator.measure] + locator.onset,
                    absolute_end: offsets[&locator.measure] + locator.onset + locator.duration + 1,
                    staff: locator.staff.parse().unwrap(), technical_lane: lane,
                });
            }
            let attacks = attacks.into_values().collect::<Vec<_>>();
            assert!(attacks.windows(2).any(|pair| pair[0].absolute_end > pair[1].absolute_start));
            let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
            assert!(channels.channel_count() > 1);
            let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
            let mut report = String::new();
            let replacement = channels.checked_original_notation_beam_components(
                &mut report, &canonical, &attacks, &observation, &[], &voices);
            if competing_rest {
                assert!(replacement.is_none(), "{report}");
                assert!(report.contains("original_component_source_rest_collision"), "{report}");
                continue;
            }
            assert!(replacement.is_some(), "{report}");
            let (replacement_channels, replacement) = replacement.unwrap();
            for group in &canonical.original_beam_voice_groups {
                assert_eq!(group.allocation_sources().iter().map(|source|
                    replacement.label_for_source(*source)).collect::<BTreeSet<_>>().len(), 1);
            }
            let labels = voices.channel_to_label.values().cloned().collect::<BTreeSet<_>>();
            let names = labels.iter().enumerate().map(|(index, _)| format!("opaque-lane-{index}"))
                .collect::<Vec<_>>();
            let alpha = final_chord_permute_sealed_voice_labels(&voices, &names).unwrap();
            let rename = labels.into_iter().zip(names).collect::<BTreeMap<_, _>>();
            let sources = canonical.original_beam_voice_groups.iter()
                .flat_map(OriginalBeamVoiceGroup::allocation_sources).collect::<BTreeSet<_>>();
            let rebound = replacement_channels.bind_original_component_labels(&canonical, &alpha, &sources).unwrap();
            for source in sources {
                assert_eq!(rebound.label_for_source(source),
                    Some(rename[replacement.label_for_source(source).unwrap()].as_str()));
            }
            let independent = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
                &original, canonical.source_locators.iter().enumerate().map(|(index, locator)|
                    ((locator.source_event_id, locator.attack_id), format!("independent-line-{}", index % 2))));
            canonical.bind_original_beam_voice_groups(&original, Some(&Ok(capture)), &independent);
            assert!(canonical.original_beam_voice_groups.is_empty());
            assert!(channels.checked_original_notation_beam_components(
                &mut String::new(), &canonical, &attacks, &observation, &[], &voices).is_none());
            canonical.original_beam_voice_groups.clear();
            assert!(channels.checked_original_notation_beam_components(
                &mut String::new(), &canonical, &attacks, &observation, &[], &voices).is_none());
        }
    }

    #[test]
    fn original_group_equality_and_written_time_are_composed_before_materialization() {
        for (scope, copies, written_overlap, independent_line, missing_origin, foreign_component, coupled_channels, external_line) in [
            (43, 1, false, false, false, false, false, false), (1043, 2, false, false, false, false, false, false),
            (2043, 1, true, false, false, false, false, false), (3043, 1, false, true, false, false, false, false),
            (4043, 1, false, false, true, false, false, false), (5043, 1, false, false, false, true, false, false),
            (6043, 2, false, false, false, false, true, false), (7043, 1, false, false, false, false, false, true),
        ] {
            let note = |step: &str, duration: i64, kind: &str|
                format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>{duration}</duration><voice>1</voice><type>{kind}</type><staff>1</staff></note>");
            let prefix = ["C", "D", "E"].iter().map(|step| note(step,24,"16th")).collect::<String>();
            let last = note("F", if written_overlap {48} else {24}, if written_overlap {"eighth"} else {"16th"});
            let tail = if foreign_component {
                format!("{}{}<note><rest/><duration>48</duration><voice>1</voice><type>eighth</type><staff>1</staff></note>", note("C",24,"16th"),note("D",24,"16th"))
            } else { "<note><rest/><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>".to_owned() };
            let body = format!("{prefix}{last}{}{}{}<note><rest/><duration>48</duration><voice>1</voice><type>eighth</type><staff>1</staff></note>{}{tail}",
                if written_overlap {"<backup><duration>24</duration></backup>"} else {""},
                note("G",24,"16th"),note("A",24,"16th"),note("B",96,"quarter"));
            let measures = (0..copies).map(|index| {
                // Different staffs have independent role/consumer resources.
                // The coupled duplicate separately preserves the old refusal.
                let body = if index == 1 && !coupled_channels { body.replace("<staff>1</staff>", "<staff>2</staff>") } else { body.clone() };
                format!("<measure number=\"{}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{body}</measure>", scope+index)
            }).collect::<String>();
            let measures = if external_line {
                format!("<measure number=\"{}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes>{}</measure>{measures}",scope-1,note("C",384,"whole"))
            } else { measures };
            let input = format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\">{measures}</part></score-partwise>");
            let (original, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&input);
            let evidence = final_chord_declassified_evidence_from_xml(&original).unwrap();
            let (mut canonical,_) = plan_canonical_final_chord_structure_once(&original,&evidence).unwrap();
            let notation = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
                &original, canonical.source_locators.iter()
                    .filter(|row| !missing_origin || row.onset != 0)
                    .map(|row| ((row.source_event_id,row.attack_id),
                        if external_line && row.measure < scope {"unrelated-earlier-line"}
                        else if foreign_component && row.onset >= 288 {"foreign-line"}
                        else if independent_line && row.onset < 96 {"independent-line"} else {"named-line"}.to_owned())));
            canonical.bind_original_beam_voice_groups(&original,Some(&Ok(capture.clone())),&notation);
            let observation = super::super::actual_source_beam_binding::observe_actual_source_beam_binding(&canonical,&original,Some(&Ok(capture)));
            let offsets = canonical.measures.iter().scan(0,|offset,row| {
                let start=*offset;*offset+=row.expected_duration;Some((row.measure,start))
            }).collect::<BTreeMap<_,_>>();
            let attacks = canonical.source_locators.iter().map(|row| FrozenVoiceAttackEvidenceRow {
                source_event_id:row.source_event_id,attack_id:row.attack_id,
                absolute_start:offsets[&row.measure]+row.onset,
                absolute_end:offsets[&row.measure]+row.onset+if row.onset==72 {48} else if foreign_component && row.onset==192 {120} else {row.duration},
                staff:row.staff.parse().unwrap(),technical_lane:if row.onset==96 {2} else {1},
            }).collect::<Vec<_>>();
            let baseline = build_frozen_serialization_voice_channels_for_canonical(&canonical,&attacks).unwrap();
            let labels = bind_musicxml_voice_labels(&baseline,&[]).unwrap();
            let mut report=String::new();
            let replacement = baseline.checked_original_notation_beam_components(
                &mut report,&canonical,&attacks,&observation,&[],&labels);
            if coupled_channels {
                assert!(replacement.is_none(), "{report}");
                assert!(report.contains("original_component_has_no_label_preserving_binding"), "{report}");
                continue;
            }
            let (refined,refined_voices) = replacement.expect(&report);
            let reuse_expected = !written_overlap && !independent_line && !missing_origin;
            let expected=if !reuse_expected || foreign_component {2} else {copies};
            assert_eq!(refined.channel_count(),expected,"{report}");
            assert_eq!(report.contains("OriginalNotationWrittenRefinement: admitted"),reuse_expected,"{report}");
            if foreign_component {
                let label_at = |onset| {
                    let source = canonical.source_locators.iter().find(|row| row.onset == onset).unwrap().source_event_id;
                    refined_voices.label_for_source(source).unwrap()
                };
                assert_eq!(label_at(0), label_at(96));
                assert_ne!(label_at(0), label_at(288));
            }
            if external_line {
                let component = canonical.source_locators.iter().filter(|row|
                    row.measure == scope && (row.onset == 96 || row.onset == 120))
                    .map(|row|row.source_event_id).collect::<BTreeSet<_>>();
                let outside = canonical.source_locators.iter().find(|row|row.measure < scope).unwrap().source_event_id;
                let host_sources = baseline.source_to_channel.iter().filter_map(|(source, channel)|
                    (*channel == baseline.source_to_channel[&outside]).then_some(*source)).collect::<BTreeSet<_>>();
                let mut whole_channel = host_sources.clone();whole_channel.extend(&component);
                assert!(!canonical.proves_common_original_notation_voice(&whole_channel));
                assert!(canonical.proves_original_notation_voice_for_component_reuse(&component,&host_sources));
                assert!(!canonical.proves_original_notation_voice_for_component_reuse(&component,&BTreeSet::from([outside])));
                assert_eq!(refined_voices.label_for_source(outside), labels.label_for_source(outside));
            }
            // A sounding overlap still exists in the positive case. It is not
            // erased or shortened to manufacture written compatibility.
            assert!(attacks.iter().any(|a| attacks.iter().any(|b|
                a.source_event_id!=b.source_event_id && a.absolute_start < b.absolute_start
                    && a.absolute_end > b.absolute_start)));
        }
    }

    fn fixture() -> (
        CanonicalFinalChordStructuralPlan,
        FrozenSerializedVoicePlan,
        FinalPdfSourceOccurrenceContinuation,
    ) {
        let before = crate::pipeline::actual_beam_owner_capture_tests::input()
            .replace("probe", &37usize.to_string());
        fixture_from_input(&before)
    }

    pub(super) fn fixture_from_input(
        before: &str,
    ) -> (
        CanonicalFinalChordStructuralPlan,
        FrozenSerializedVoicePlan,
        FinalPdfSourceOccurrenceContinuation,
    ) {
        fixture_with_source_holds(before, &[])
    }

    fn fixture_with_source_holds(
        before: &str,
        held_pairs: &[(usize, usize)],
    ) -> (
        CanonicalFinalChordStructuralPlan,
        FrozenSerializedVoicePlan,
        FinalPdfSourceOccurrenceContinuation,
    ) {
        let (original, receipt, capture) =
            crate::pipeline::actual_beam_owner_capture_tests::execute(before);
        capture.binds_original_receipt(&receipt).unwrap();
        let mut evidence = final_chord_declassified_evidence_from_xml(&original).unwrap();
        let measures = final_chord_parse_binding_measures(&original).unwrap();
        for &(left, right) in held_pairs {
            // This synthetic fixture declares the two written fragments of one
            // source attack explicitly. The legacy XML declassifier only
            // follows ties within one measure and has no cross-measure IDs.
            let start = evidence.iter().filter(|row| row.measure == left && row.tie_start)
                .collect::<Vec<_>>();
            let [start] = start.as_slice() else { panic!("synthetic hold must have one start"); };
            let (source, attack, pitch, staff, fragment) =
                (start.source_event_id, start.attack_id, start.pitch, start.staff, start.fragment_id);
            let left_index = measures.iter().position(|measure| measure.measure == left).unwrap();
            assert_eq!(measures[left_index + 1].measure, right);
            assert_eq!(start.onset + start.duration, measures[left_index].expected_duration);
            let mut stops = evidence.iter_mut().filter(|row| row.measure == right && row.tie_stop);
            let stop = stops.next().expect("synthetic hold must have one stop");
            assert!(stops.next().is_none());
            assert_eq!((stop.pitch, stop.staff, stop.onset), (pitch, staff, 0));
            assert_ne!(stop.fragment_id, fragment);
            stop.source_event_id = source;
            stop.attack_id = attack;
        }
        let (mut canonical, _) =
            plan_canonical_final_chord_structure_once(&original, &evidence).unwrap();
        let notation = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
            &original,
            canonical
                .source_locators
                .iter()
                .map(|locator| ((locator.source_event_id, locator.attack_id), "3".to_owned())),
        );
        canonical.bind_original_beam_voice_groups(&original, Some(&Ok(capture)), &notation);
        assert!(!canonical.original_beam_voice_groups.is_empty());
        let mut offset = 0i64;
        let offsets = canonical
            .measures
            .iter()
            .map(|measure| {
                let start = offset;
                offset += measure.expected_duration;
                (measure.measure, start)
            })
            .collect::<BTreeMap<_, _>>();
        let mut attacks = BTreeMap::<usize, FrozenVoiceAttackEvidenceRow>::new();
        for locator in canonical.source_locators.iter() {
            let index = attacks.len();
            let row =
                attacks
                    .entry(locator.source_event_id)
                    .or_insert(FrozenVoiceAttackEvidenceRow {
                        source_event_id: locator.source_event_id,
                        attack_id: locator.attack_id,
                        absolute_start: offsets[&locator.measure] + locator.onset,
                        absolute_end: offsets[&locator.measure] + locator.onset + locator.duration,
                        staff: locator.staff.parse().unwrap(),
                        technical_lane: 1 + index % 2,
                    });
            row.absolute_start = row
                .absolute_start
                .min(offsets[&locator.measure] + locator.onset);
            row.absolute_end = row
                .absolute_end
                .max(offsets[&locator.measure] + locator.onset + locator.duration);
        }
        let attacks = attacks.into_values().collect::<Vec<_>>();
        let channels =
            build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
        let projected = project_canonical_final_chord_render_occurrence_continuation(
            &original,
            &canonical,
            &voices,
            &BTreeSet::new(),
        )
        .unwrap();
        let continuation = FinalPdfSourceOccurrenceContinuation::from_original_transport(projected.xml().to_owned(), projected.projected.receipt.clone()).unwrap();
        (canonical, voices, continuation)
    }

    pub(super) fn plan(
        voices: &FrozenSerializedVoicePlan,
        source: &FinalPdfSourceOccurrenceContinuation,
        fragments: usize,
    ) -> PrimaryBeamVoicePartitionBoundPlan {
        let geometry = source
            .receipt
            .original_beam_voice_groups
            .iter()
            .map(|group| (group.id.clone(), fragments))
            .collect();
        bind_primary_beam_voice_partition_plan(
            source.xml(),
            PrimaryBeamVoicePartitionBindingContext {
                source_continuation: source,
                voice_plan: voices,
                candidates: &[],
                classification_xml: source.xml(),
                renderer_visible_fragments_by_metric_line_id: &geometry,
                materialization_safety_proven_metric_line_ids: &BTreeSet::new(),
            },
        )
        .unwrap()
    }

    #[test]
    fn later_primary_target_survives_rebind_writer_and_occurrence_transport() {
        let (canonical, mut voices, source) = fixture();
        let group = &source.receipt.original_beam_voice_groups[0];
        let first = group.members[0].0;
        voices
            .source_to_role
            .insert(first, SerializedVoicePresentationRole::UpperSecondary);
        let prepared = plan(&voices, &source, 4);
        assert_eq!(prepared.scope_candidates(), 1);
        let target = group
            .members
            .iter()
            .enumerate()
            .find(|(_, (id, _))| {
                voices.role_for_source(*id) == Some(SerializedVoicePresentationRole::UpperPrimary)
            })
            .unwrap();
        assert!(target.0 > 0);
        for selected in &prepared.selected_members {
            assert_eq!(selected.target_primary_source_event_id, target.1.0);
            assert_eq!(selected.target_primary_attack_id, target.1.1);
            assert_eq!(
                selected.target_primary_fragment_id,
                group.fragments[target.0]
            );
        }
        let rebound = rebind_primary_beam_voice_partition_plan_to_current_xml(
            source.xml(),
            &source,
            &prepared,
        )
        .unwrap();
        assert_eq!(rebound.edits().len(), prepared.edits().len());
        let edits = rebound
            .edits()
            .iter()
            .map(|e| (e.voice_text_start, e.voice_text_end, e.target_label.clone()))
            .collect();
        let (after, receipt) =
            crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
                source.xml(),
                edits,
            );
        let after = after.unwrap();
        assert_eq!(
            receipt
                .unwrap()
                .realize_post_document(source.xml())
                .as_deref(),
            Some(after.as_str())
        );
        let transported = transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
            &source, source.xml(), &after, &canonical, &rebound, Some(&BTreeSet::new())).unwrap();
        assert_eq!(transported.xml(), after);
        assert_eq!(
            rebound.repaired_voice_plan().source_to_role,
            voices.source_to_role
        );
    }

    #[test]
    fn primary_source_in_secondary_channel_does_not_mask_later_primary_channel() {
        let (_, mut voices, source) = fixture();
        let group = &source.receipt.original_beam_voice_groups[0];
        let first = group.members[0].0;
        let next = group.members[1].0;
        let first_channel = voices
            .channel_for_label(voices.label_for_source(first).unwrap())
            .unwrap();
        let next_channel = voices
            .channel_for_label(voices.label_for_source(next).unwrap())
            .unwrap();
        assert_ne!(first_channel, next_channel);
        voices.channel_to_role.insert(
            first_channel,
            SerializedVoicePresentationRole::UpperSecondary,
        );
        voices
            .channel_to_role
            .insert(next_channel, SerializedVoicePresentationRole::UpperPrimary);
        voices
            .source_to_role
            .insert(next, SerializedVoicePresentationRole::UpperPrimary);
        let prepared = plan(&voices, &source, 4);
        assert_eq!(prepared.scope_candidates(), 1);
        for selected in &prepared.selected_members {
            assert_eq!(selected.target_primary_source_event_id, next);
            assert_eq!(selected.target_primary_fragment_id, group.fragments[1]);
            assert_eq!(selected.target_channel, next_channel);
        }
    }

    #[test]
    fn no_primary_source_or_no_primary_channel_refuses_the_complete_group() {
        let (_, voices, source) = fixture();
        let mut secondary = voices.clone();
        for &(id, _) in &source.receipt.original_beam_voice_groups[0].members {
            secondary
                .source_to_role
                .insert(id, SerializedVoicePresentationRole::UpperSecondary);
        }
        let refused = plan(&secondary, &source, 4);
        assert_eq!(refused.scope_candidates(), 0);
        assert!(refused.edits().is_empty());
        let mut secondary_channels = voices;
        for role in secondary_channels.channel_to_role.values_mut() {
            *role = SerializedVoicePresentationRole::UpperSecondary;
        }
        let refused = plan(&secondary_channels, &source, 4);
        assert_eq!(refused.scope_candidates(), 0);
        assert!(refused.edits().is_empty());
    }

    #[test]
    fn later_primary_target_still_refuses_a_new_bystander_rest_overlap() {
        let (_, mut voices, mut source) = fixture();
        let group = &source.receipt.original_beam_voice_groups[0];
        voices.source_to_role.insert(
            group.members[0].0,
            SerializedVoicePresentationRole::UpperSecondary,
        );
        let prepared = plan(&voices, &source, 4);
        assert_eq!(prepared.scope_candidates(), 1);
        let edit = &prepared.edits()[0];
        let measures = final_chord_parse_binding_measures(source.xml()).unwrap();
        let measure = measures.iter().find(|m| m.measure == edit.measure).unwrap();
        let note = measure
            .notes
            .iter()
            .filter(|n| !n.rest_like && n.pitch_midi.is_some())
            .nth(edit.pitched_ordinal)
            .unwrap();
        let cursor = measure
            .notes
            .iter()
            .map(|n| n.onset + n.duration)
            .max()
            .unwrap();
        let bystander = format!(
            "<backup><duration>{}</duration></backup><note><rest/><duration>{}</duration><voice>{}</voice><staff>1</staff></note>",
            cursor - note.onset,
            note.duration,
            edit.target_label
        );
        let end = source.xml.find("</measure>").unwrap();
        source.xml.insert_str(end, &bystander);
        validate_batch_occupancy(source.xml(), &[]).unwrap();
        let refused = plan(&voices, &source, 4);
        assert_eq!(refused.scope_candidates(), 0);
        assert!(refused.edits().is_empty());
    }

    #[test]
    fn individually_safe_assignments_with_a_joint_channel_collision_are_refused() {
        for synthetic_measure in [17usize, 83] {
            let xml = format!(
                "<score-partwise><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{synthetic_measure}\"><attributes><divisions>4</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><backup><duration>4</duration></backup><note><pitch><step>E</step><octave>5</octave></pitch><duration>4</duration><voice>2</voice><staff>1</staff></note></measure></part></score-partwise>"
            );
            let parsed = final_chord_parse_binding_measures(&xml).unwrap();
            let edits = parsed[0]
                .notes
                .iter()
                .enumerate()
                .map(|(ordinal, note)| {
                    let (start, end) =
                        primary_beam_voice_partition_voice_text_range(&xml, note.range.clone())
                            .unwrap();
                    PrimaryBeamVoicePartitionBoundEdit {
                        measure: parsed[0].measure,
                        pitched_ordinal: ordinal,
                        voice_text_start: start,
                        voice_text_end: end,
                        expected_old_label: note.voice.clone(),
                        target_label: "3".to_owned(),
                    }
                })
                .collect::<Vec<_>>();
            assert_eq!(edits.len(), 2);
            for edit in &edits {
                validate_batch_occupancy(&xml, std::slice::from_ref(edit)).unwrap();
            }
            assert!(validate_batch_occupancy(&xml, &edits).is_err());
        }
    }

    #[test]
    fn original_owner_group_can_align_without_using_old_metric_audit_as_authority() {
        let (canonical, voices, source) = fixture();
        let prepared = plan(&voices, &source, 4);
        assert_eq!(prepared.scope_candidates(), 1);
        assert_eq!(prepared.edits().len(), 2);
        let edits = prepared
            .edits()
            .iter()
            .map(|e| (e.voice_text_start, e.voice_text_end, e.target_label.clone()))
            .collect();
        let (after, receipt) =
            crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
                source.xml(),
                edits,
            );
        let after = after.unwrap();
        assert_eq!(
            receipt
                .unwrap()
                .realize_post_document(source.xml())
                .as_deref(),
            Some(after.as_str())
        );
        let continuation = transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
            &source, source.xml(), &after, &canonical, &prepared, Some(&BTreeSet::new())).unwrap();
        assert_eq!(continuation.xml(), after);
        assert_eq!(
            prepared.repaired_voice_plan().source_to_role,
            voices.source_to_role
        );
    }

    #[test]
    fn absent_origin_or_preserved_renderer_geometry_never_creates_scope() {
        let (_, voices, mut source) = fixture();
        assert_eq!(plan(&voices, &source, 1).scope_candidates(), 0);
        assert_eq!(plan(&voices, &source, 0).scope_candidates(), 0);
        source.receipt.original_beam_voice_groups.clear();
        assert_eq!(plan(&voices, &source, 4).scope_candidates(), 0);
    }

    #[test]
    fn stale_original_document_does_not_mint_origin_groups() {
        let before =
            crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe", "31");
        let (original, _, capture) =
            crate::pipeline::actual_beam_owner_capture_tests::execute(&before);
        let evidence = final_chord_declassified_evidence_from_xml(&original).unwrap();
        let (mut canonical, _) =
            plan_canonical_final_chord_structure_once(&original, &evidence).unwrap();
        let notation = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
            &original,
            canonical
                .source_locators
                .iter()
                .map(|locator| ((locator.source_event_id, locator.attack_id), "3".to_owned())),
        );
        canonical.bind_original_beam_voice_groups(
            &format!("{original} "),
            Some(&Ok(capture)),
            &notation,
        );
        assert!(canonical.original_beam_voice_groups.is_empty());
    }

    #[test]
    fn foreign_fragment_or_duplicate_physical_source_refuses_whole_group() {
        let (_, voices, mut source) = fixture();
        let first = source.receipt.occurrences[0];
        source.receipt.occurrences.push(first);
        assert_eq!(plan(&voices, &source, 4).scope_candidates(), 0);
        source.receipt.occurrences.pop();
        source.receipt.original_beam_voice_groups[0].fragments[0] += 1000;
        assert_eq!(plan(&voices, &source, 4).scope_candidates(), 0);
    }

    #[test]
    fn changing_an_owned_secondary_decision_refuses_even_with_primary_unchanged() {
        let (_, voices, mut source) = fixture();
        let before = source.xml.clone();
        source.xml = source.xml.replacen(
            "<beam number=\"2\">begin</beam>",
            "<beam number=\"2\">end</beam>",
            1,
        );
        assert_ne!(before, source.xml);
        assert_eq!(plan(&voices, &source, 4).scope_candidates(), 0);
    }

    #[test]
    fn late_rebind_refuses_a_changed_original_beam_decision() {
        let (_, voices, mut source) = fixture();
        let prepared = plan(&voices, &source, 4);
        assert!(!prepared.original_group_alignment.is_empty());
        source.xml = source.xml.replacen(
            "<beam number=\"1\">begin</beam>",
            "<beam number=\"1\">end</beam>",
            1,
        );
        assert!(
            rebind_primary_beam_voice_partition_plan_to_current_xml(
                source.xml(),
                &source,
                &prepared
            )
            .is_err()
        );
    }

    #[test]
    fn foreign_canonical_origin_carrier_cannot_validate_the_same_bytes() {
        let (mut canonical, voices, source) = fixture();
        canonical.original_beam_voice_groups.clear();
        assert!(
            verify_final_pdf_source_occurrence_continuation(
                source.xml(),
                &canonical,
                &voices,
                Some(&BTreeSet::new()),
                &source
            )
            .is_err()
        );
    }

    #[test]
    fn opaque_group_and_voice_renaming_preserves_local_alignment() {
        let (mut canonical, voices, source) = fixture();
        let baseline = plan(&voices, &source, 4);
        let labels = voices
            .channel_to_label
            .values()
            .enumerate()
            .map(|(index, _)| format!("{}", 101 + index))
            .collect::<Vec<_>>();
        let renamed = final_chord_permute_sealed_voice_labels(&voices, &labels).unwrap();
        let projected = project_canonical_final_chord_render_occurrence_continuation(
            canonical
                .written_projection
                .as_ref()
                .unwrap()
                .original_xml
                .as_ref(),
            &canonical,
            &renamed,
            &BTreeSet::new(),
        )
        .unwrap();
        let renamed_source = FinalPdfSourceOccurrenceContinuation::from_original_transport(projected.xml().to_owned(), projected.projected.receipt.clone()).unwrap();
        let next = plan(&renamed, &renamed_source, 4);
        assert_eq!(next.edits().len(), baseline.edits().len());
        assert_eq!(next.scope_candidates(), baseline.scope_candidates());
        canonical.original_beam_voice_groups[0].id = "independent-opaque-name".into();
        let mut source = source;
        source.receipt.original_beam_voice_groups = canonical.original_beam_voice_groups;
        assert_eq!(
            plan(&voices, &source, 4).edits().len(),
            baseline.edits().len()
        );
    }

    #[test]
    fn tied_original_source_closes_adjacent_beam_groups_as_one_atomic_assignment() {
        let note = |step: &str, tie: &str| {
            format!(
                "<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>24</duration>{tie}<voice>3</voice><type>16th</type><staff>1</staff></note>"
            )
        };
        let mut notes = ["C", "D", "E"]
            .into_iter()
            .map(|step| note(step, ""))
            .collect::<String>();
        notes.push_str(&note("F", "<tie type=\"start\"/>"));
        notes.push_str(&note("F", "<tie type=\"stop\"/>"));
        for step in ["G", "A", "B"] {
            notes.push_str(&note(step, ""));
        }
        let scope = 47usize;
        let input = format!(
            "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{scope}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes><note><rest/><duration>192</duration><voice>3</voice><type>half</type><staff>1</staff></note>{notes}</measure></part></score-partwise>"
        );
        let (canonical, voices, source) = fixture_from_input(&input);
        let group = &source.receipt.original_beam_voice_groups[0];
        assert_eq!(group.primary_group_count, 2);
        assert_eq!(group.members.len(), 8);
        assert_eq!(group.members.iter().collect::<BTreeSet<_>>().len(), 7);
        let prepared = plan(&voices, &source, 8);
        assert_eq!(prepared.scope_candidates(), 1);
        assert_eq!(prepared.edits().len(), 4);
        let rebound = rebind_primary_beam_voice_partition_plan_to_current_xml(
            source.xml(),
            &source,
            &prepared,
        )
        .unwrap();
        let edits = rebound
            .edits()
            .iter()
            .map(|e| (e.voice_text_start, e.voice_text_end, e.target_label.clone()))
            .collect();
        let (after, receipt) =
            crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
                source.xml(),
                edits,
            );
        let after = after.unwrap();
        assert_eq!(
            receipt
                .unwrap()
                .realize_post_document(source.xml())
                .as_deref(),
            Some(after.as_str())
        );
        transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
            &source,
            source.xml(),
            &after,
            &canonical,
            &rebound,
            Some(&BTreeSet::new()),
        )
        .unwrap();
    }

    #[test]
    fn a_held_source_connects_measure_local_beam_evidence_into_one_write() {
        let component = |left_label: usize, staff: u8| {
            let right_label = left_label + 1;
            let note = |step: &str, tie: &str| format!(
                "<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>48</duration>{tie}<voice>3</voice><type>eighth</type><staff>{staff}</staff></note>"
            );
            let rest = format!(
                "<note><rest/><duration>288</duration><voice>3</voice><type>half</type><dot/><staff>{staff}</staff></note>"
            );
            format!(
                "<measure number=\"{left_label}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{rest}{}{}</measure>                 <measure number=\"{right_label}\">{}{}{rest}</measure>",
                note("F", ""), note("G", "<tie type=\"start\"/>"),
                note("G", "<tie type=\"stop\"/>"), note("A", ""),
            )
        };
        for left_label in [21usize, 87] {
            let mut original_footprint = None;
            for copies in [1, 2] {
                // Separate staffs and source attacks give the repeated figure
                // independent occupied channels in the same part.
                let single = component(left_label, 1);
                let repeated = if copies == 2 { component(left_label + 2, 2) } else { String::new() };
                let input = format!(
                    "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\">{single}{repeated}</part></score-partwise>",
                );
                let held_pairs: Vec<_> = (0..copies).map(|copy| {
                    let start = left_label + 2 * copy;
                    (start, start + 1)
                }).collect();
                let (canonical, voices, source) = fixture_with_source_holds(&input, &held_pairs);
                assert_eq!(canonical.allocation_original_beam_groups().count(), 0);
                let cohorts = renderer_cohorts(&source);
                assert_eq!(cohorts.len(), copies * 2);
                assert_eq!(cohorts.iter().map(|cohort| cohort.measure()).collect::<BTreeSet<_>>().len(), copies * 2);
                assert_eq!(cohorts.iter().map(|cohort| cohort.metric_line_id()).collect::<BTreeSet<_>>().len(), copies * 2);
                let measures = final_chord_parse_binding_measures(source.xml()).unwrap();
                let geometry: BTreeMap<_, _> = cohorts.iter().map(|cohort| {
                    let channels = cohort.pitched_ordinals().iter()
                        .map(|&ordinal| pitched_note(&measures, cohort.measure(), ordinal).unwrap().voice.clone())
                        .collect::<BTreeSet<_>>();
                    (cohort.metric_line_id().to_owned(), channels.len())
                }).collect();
                assert!(geometry.values().filter(|&&fragments| fragments > 1).count() >= copies);
                let bind = |geometry: &BTreeMap<String, usize>| bind_primary_beam_voice_partition_plan(
                    source.xml(), PrimaryBeamVoicePartitionBindingContext {
                        source_continuation: &source,
                        voice_plan: &voices,
                        candidates: &[],
                        classification_xml: source.xml(),
                        renderer_visible_fragments_by_metric_line_id: geometry,
                        materialization_safety_proven_metric_line_ids: &BTreeSet::new(),
                    },
                ).unwrap();
                for id in geometry.keys() {
                    let mut incomplete = geometry.clone();
                    incomplete.remove(id);
                    assert_eq!(bind(&incomplete).scope_candidates(), copies - 1);
                }
                let prepared = bind(&geometry);
                assert_eq!(prepared.scope_candidates(), copies);
                let footprint = prepared.edits.iter().filter(|edit| edit.measure <= left_label + 1)
                    .map(|edit| (edit.measure - left_label, edit.pitched_ordinal)).collect::<Vec<_>>();
                if let Some(original) = &original_footprint {
                    assert_eq!(&footprint, original);
                } else {
                    original_footprint = Some(footprint);
                }
                let sealed = admitted_multi_measure_renderer_cohorts(&source, &prepared.original_group_alignment).unwrap();
                assert_eq!(sealed.len(), cohorts.len());
                for cohort in &cohorts {
                    assert!(sealed.contains(cohort));
                }
                let rebound = rebind_primary_beam_voice_partition_plan_to_current_xml(
                    source.xml(), &source, &prepared,
                ).unwrap();
                let edits = rebound.edits().iter().map(|edit|
                    (edit.voice_text_start, edit.voice_text_end, edit.target_label.clone())).collect();
                let (after, receipt) =
                    crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
                        source.xml(), edits,
                    );
                let after = after.unwrap();
                assert_eq!(receipt.unwrap().realize_post_document(source.xml()).as_deref(), Some(after.as_str()));
                let carried = transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
                    &source, source.xml(), &after, &canonical, &rebound, Some(&BTreeSet::new()),
                ).unwrap();
                assert_eq!(
                    admitted_multi_measure_renderer_cohorts(&carried, &rebound.original_group_alignment).unwrap(),
                    sealed,
                );
            }
        }
    }

    #[test]
    fn a_visible_split_within_one_channel_does_not_prove_a_channel_boundary() {
        for (first, second) in [("alpha", "beta"), ("counter", "principal")] {
            let labels = [first, first, second];
            assert!(!proves_missing_channel_boundary(&labels, 2));
            assert!(proves_missing_channel_boundary(&labels, 3));
            assert!(!proves_missing_channel_boundary(&labels, 0));
            assert!(!proves_missing_channel_boundary(&labels, 4));
        }
    }

    #[test]
    fn a_grace_host_requires_the_same_physical_cursor_staff_and_voice() {
        for label in [35usize, 96] {
            let grace = r#"<note><grace/><pitch><step>C</step><octave>4</octave></pitch><voice>two</voice><staff>1</staff></note>"#;
            let principal = r#"<note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>two</voice><type>16th</type><staff>1</staff></note>"#;
            let attach = |approach: &str, cursor: &str| {
                let input = format!(r#"<score-partwise version="4.0"><part id="P"><measure number="{label}"><attributes><divisions>96</divisions></attributes>{approach}{cursor}{principal}</measure></part></score-partwise>"#);
                let measures = final_chord_parse_binding_measures(&input).unwrap();
                let measure = &measures[0];
                directly_attached_graces(&input, measure, &measure.notes[1]).map(|notes| notes.len())
            };
            assert_eq!(attach(grace, ""), Some(1));
            assert_eq!(attach(grace, "<forward><duration>24</duration></forward>"), None);
            assert_eq!(attach(&grace.replace(">two<", ">one<"), ""), None);
            assert_eq!(attach(&grace.replace("<staff>1", "<staff>2"), ""), None);
            assert_eq!(attach(&grace.replace("<grace/>", "<grace/><beam number=\"1\">begin</beam>"), ""), None);
        }
    }

    #[test]
    fn beamless_tied_fragment_is_an_atomic_source_resource_without_beam_membership() {
        for scope in [17usize,83] {
            let note = |step: &str,duration: i64,kind: &str,tie: &str| format!("<note><pitch><step>{step}</step><octave>5</octave></pitch><duration>{duration}</duration>{tie}<voice>3</voice><type>{kind}</type><staff>1</staff></note>");
            let notes = [note("C",72,"eighth","<tie type=\"start\"/><dot/>"),
                note("C",48,"eighth","<tie type=\"stop\"/>"),
                note("D",48,"eighth","<tie type=\"start\"/>"),
                note("D",24,"16th","<tie type=\"stop\"/>"),
                note("E",24,"16th",""),note("F",48,"eighth","")].concat();
            let input = format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{scope}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes><note><rest/><duration>96</duration><voice>3</voice><type>quarter</type><staff>1</staff></note><note><rest/><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>{notes}</measure></part></score-partwise>");
            let (canonical,voices,source)=fixture_from_input(&input);
            let group=source.receipt.original_beam_voice_groups.iter().find(|g|g.primary_group_count==2).unwrap();
            let bound=group.fragments.iter().copied().collect::<BTreeSet<_>>();
            let keys=group.members.iter().copied().collect::<BTreeSet<_>>();
            let extra=*source.receipt.occurrences.iter().find(|r|keys.contains(&(r.source_event_id,r.attack_id)) && !bound.contains(&r.fragment_id)).unwrap();
            let prepared=plan(&voices,&source,7);
            assert_eq!(prepared.scope_candidates(),1);
            assert!(prepared.selected_members.iter().any(|r|r.fragment_id==extra.fragment_id));
            let rebound=rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(),&source,&prepared).unwrap();
            let edits=rebound.edits().iter().map(|e|(e.voice_text_start,e.voice_text_end,e.target_label.clone())).collect();
            let (after,receipt)=crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(source.xml(),edits);
            let after=after.unwrap();
            assert_eq!(receipt.unwrap().realize_post_document(source.xml()).as_deref(),Some(after.as_str()));
            transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(&source,source.xml(),&after,&canonical,&rebound,Some(&BTreeSet::new())).unwrap();
            let mut duplicate=source.clone();duplicate.receipt.occurrences.push(extra);
            assert_eq!(plan(&voices,&duplicate,7).scope_candidates(),0);
        }
    }

    #[test]
    fn missing_original_member_or_semantic_line_cannot_become_a_smaller_successful_group() {
        let before =
            crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe", "29");
        let (original, _, capture) =
            crate::pipeline::actual_beam_owner_capture_tests::execute(&before);
        let evidence = final_chord_declassified_evidence_from_xml(&original).unwrap();
        let (mut canonical, _) =
            plan_canonical_final_chord_structure_once(&original, &evidence).unwrap();
        let mixed = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
            &original,
            canonical
                .source_locators
                .iter()
                .enumerate()
                .map(|(index, locator)| {
                    (
                        (locator.source_event_id, locator.attack_id),
                        if index == 1 { "independent-line" } else { "3" }.to_owned(),
                    )
                }),
        );
        canonical.bind_original_beam_voice_groups(&original, Some(&Ok(capture.clone())), &mixed);
        assert!(canonical.original_beam_voice_groups.is_empty());
        let complete = crate::pipeline::actual_beam_owner_capture_tests::notation_voice_bindings(
            &original,
            canonical
                .source_locators
                .iter()
                .map(|locator| ((locator.source_event_id, locator.attack_id), "3".to_owned())),
        );
        let mut locators = canonical.source_locators.to_vec();
        locators.remove(1);
        canonical.source_locators = std::sync::Arc::from(locators);
        canonical.bind_original_beam_voice_groups(&original, Some(&Ok(capture)), &complete);
        assert!(canonical.original_beam_voice_groups.is_empty());
    }

    fn scalar_test_edit(xml: &str, target: &str) -> PrimaryBeamVoicePartitionBoundEdit {
        let measure = final_chord_parse_binding_measures(xml).unwrap().remove(0);
        let note = measure
            .notes
            .iter()
            .find(|note| note.pitch_midi.is_some())
            .unwrap();
        let (start, end) =
            primary_beam_voice_partition_voice_text_range(xml, note.range.clone()).unwrap();
        PrimaryBeamVoicePartitionBoundEdit {
            measure: measure.measure,
            pitched_ordinal: 0,
            voice_text_start: start,
            voice_text_end: end,
            expected_old_label: note.voice.clone(),
            target_label: target.to_owned(),
        }
    }

    #[test]
    fn scalar_proposal_cannot_overlap_a_bystander_rest_or_orphan_an_import_key() {
        let pitched = "<note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let rest = "<backup><duration>4</duration></backup><note><rest/><duration>4</duration><voice>2</voice><staff>1</staff></note>";
        let measure = 61usize;
        let score = |body: String| {
            format!(
                "<score-partwise><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{measure}\"><attributes><divisions>4</divisions></attributes>{body}</measure></part></score-partwise>"
            )
        };
        let overlap = score(format!("{pitched}{rest}"));
        assert!(validate_batch_occupancy(&overlap, &[scalar_test_edit(&overlap, "2")]).is_err());
        let other = "<note><pitch><step>D</step><octave>5</octave></pitch><duration>4</duration><voice>2</voice><staff>1</staff></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note>";
        let headless = score(format!("{pitched}{other}"));
        assert!(
            headless_labels(&headless, &BTreeMap::new())
                .unwrap()
                .is_empty()
        );
        assert!(validate_batch_occupancy(&headless, &[scalar_test_edit(&headless, "2")]).is_err());
    }

    #[test]
    fn independent_original_component_duplication_preserves_existing_local_decision() {
        let (_, voices, source) = fixture();
        let baseline = plan(&voices, &source, 4);
        let measure = source.receipt.original_beam_voice_groups[0].measure;
        let mut input = crate::pipeline::actual_beam_owner_capture_tests::input()
            .replace("probe", &37usize.to_string());
        let start = input.find("<measure ").unwrap();
        let end = input.find("</measure>").unwrap() + "</measure>".len();
        let duplicate = input[start..end].replace(
            &format!("number=\"{measure}\""),
            &format!("number=\"{}\"", measure + 1),
        );
        input.insert_str(end, &duplicate);
        let (_, voices, source) = fixture_from_input(&input);
        let duplicated = plan(&voices, &source, 4);
        let first = |plan: &PrimaryBeamVoicePartitionBoundPlan| {
            plan.edits()
                .iter()
                .filter(|edit| edit.measure == measure)
                .map(|edit| {
                    (
                        edit.pitched_ordinal,
                        edit.expected_old_label.clone(),
                        edit.target_label.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(first(&baseline), first(&duplicated));
        assert_eq!(duplicated.scope_candidates(), 2);
    }

    #[test]
    #[ignore = "manual renderer test; set ETUSCHLOPF_BEAM_RENDER_WRAPPER (verified with MuseScore 4.7.5)"]
    fn actual_musescore_roundtrip_confirms_the_later_primary_target_becomes_visible() {
        let wrapper =
            std::env::var("ETUSCHLOPF_BEAM_RENDER_WRAPPER").expect("explicit renderer wrapper");
        let first_measure = 1usize;
        let input = crate::pipeline::actual_beam_owner_capture_tests::input()
            .replace("probe", &first_measure.to_string())
            .replace("<type>16th</type>", "<type>16th</type><stem>down</stem>")
            .replace("<clef>", "<staves>2</staves><clef number=\"1\">")
            .replace("</clef>", "</clef><clef number=\"2\"><sign>F</sign><line>4</line></clef>")
            .replace("</measure>", "<backup><duration>384</duration></backup><note><rest/><duration>384</duration><voice>4</voice><type>whole</type><staff>2</staff></note></measure>");
        let (_, mut voices, source) = fixture_from_input(&input);
        voices.source_to_role.insert(
            source.receipt.original_beam_voice_groups[0].members[0].0,
            SerializedVoicePresentationRole::UpperSecondary,
        );
        let directory =
            std::env::temp_dir().join(format!("etuschlopf-original-beam-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let before_xml = directory.join("before.render.musicxml");
        let before_pdf = directory.join("before.pdf");
        std::fs::write(&before_xml, source.xml()).unwrap();
        assert!(
            std::process::Command::new("python3")
                .arg(&wrapper)
                .arg("-o")
                .arg(&before_pdf)
                .arg(&before_xml)
                .status()
                .unwrap()
                .success()
        );
        let cohort = renderer_cohorts(&source).pop().unwrap();
        let geometry = crate::audit_only::renderer_system_break_readback::primary_beam_geometry_readback_xml_pdf(source.xml(), &before_pdf).unwrap();
        let before = geometry
            .cohort_geometry(cohort.measure(), cohort.staff(), cohort.pitched_ordinals())
            .expect("exact renderer geometry");
        assert!(before.visible_fragments > 1);
        let prepared = plan(&voices, &source, before.visible_fragments);
        let edits = prepared
            .edits()
            .iter()
            .map(|e| (e.voice_text_start, e.voice_text_end, e.target_label.clone()))
            .collect();
        let (after, receipt) =
            crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
                source.xml(),
                edits,
            );
        let after = after.unwrap();
        assert!(receipt.is_some());
        let after_xml = directory.join("after.render.musicxml");
        let after_pdf = directory.join("after.pdf");
        std::fs::write(&after_xml, &after).unwrap();
        assert!(
            std::process::Command::new("python3")
                .arg(&wrapper)
                .arg("-o")
                .arg(&after_pdf)
                .arg(&after_xml)
                .status()
                .unwrap()
                .success()
        );
        let geometry = crate::audit_only::renderer_system_break_readback::primary_beam_geometry_readback_xml_pdf(&after, &after_pdf).unwrap();
        let after = geometry
            .cohort_geometry(cohort.measure(), cohort.staff(), cohort.pitched_ordinals())
            .expect("exact corrected renderer geometry");
        assert_eq!(after.visible_fragments, 1);
    }
    #[test]
    fn attached_chord_resources_follow_the_real_original_owner_through_writer_and_transport() {
        let mut before = crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe", "29");
        let start = before.find("<note><pitch>").unwrap();
        let end = start + before[start..].find("</note>").unwrap() + "</note>".len();
        before.insert_str(end,"<note><chord/><pitch><step>G</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><type>16th</type><staff>1</staff></note>");
        let (canonical, voices, source) = fixture_from_input(&before);
        let prepared = plan(&voices,&source,4);
        assert_eq!(prepared.scope_candidates(),1);
        let rebound = rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(),&source,&prepared).unwrap();
        let edits = rebound.edits().iter().map(|e|(e.voice_text_start,e.voice_text_end,e.target_label.clone())).collect();
        let (after,receipt)=crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(source.xml(),edits);
        let after=after.unwrap();
        assert_eq!(receipt.unwrap().realize_post_document(source.xml()).as_deref(),Some(after.as_str()));
        let transported=transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(&source,source.xml(),&after,&canonical,&rebound,Some(&BTreeSet::new())).unwrap();
        assert_eq!(transported.xml(),after);
        assert_eq!(voices.source_to_role,rebound.repaired_voice_plan().source_to_role);
        for measure in final_chord_parse_binding_measures(&after).unwrap() {
            for chord in measure.notes.iter().filter(|n|n.chord_member) {
                let head=measure.notes.iter().find(|n|Some(n.raw_note_index)==chord.anchor_index).unwrap();
                assert_eq!(head.voice,chord.voice);
            }
        }
    }

    #[test]
    fn full_partwide_profile_refuses_new_unregistered_chord_resource() {
        for synthetic_measure in [17usize,83] {
        let xml=format!("<score-partwise><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{synthetic_measure}\"><attributes><divisions>4</divisions></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>");
        validate_batch_occupancy(&xml,&[]).unwrap();
        let parsed=final_chord_parse_binding_measures(&xml).unwrap();
        let chord=&parsed[0].notes[1];
        let (start,end)=primary_beam_voice_partition_voice_text_range(&xml,chord.range.clone()).unwrap();
        let edit=PrimaryBeamVoicePartitionBoundEdit { measure:synthetic_measure,pitched_ordinal:1,voice_text_start:start,voice_text_end:end,expected_old_label:"1".to_owned(),target_label:"17".to_owned() };
        assert!(validate_batch_occupancy(&xml,&[edit]).is_err());
        }
    }

    #[test]
    fn a_foreign_classification_pitch_cannot_select_an_original_owner_group() {
        let (_,voices,source)=fixture();
        let classification=source.xml().replacen("<step>C</step>","<step>G</step>",1);
        assert_ne!(classification,source.xml());
        let geometry=source.receipt.original_beam_voice_groups.iter().map(|group|(group.id.clone(),4usize)).collect();
        let bound=bind_primary_beam_voice_partition_plan(source.xml(),PrimaryBeamVoicePartitionBindingContext {
            source_continuation:&source,voice_plan:&voices,candidates:&[],classification_xml:&classification,
            renderer_visible_fragments_by_metric_line_id:&geometry,materialization_safety_proven_metric_line_ids:&BTreeSet::new(),
        }).unwrap();
        assert_eq!(bound.scope_candidates(),0);
    }

    #[test]
    fn original_import_component_closes_a_currently_connected_group_after_a_real_split_seed() {
        for first_number in [17usize,83] {
            let mut input=crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe",&first_number.to_string());
            let doc=roxmltree::Document::parse(&input).unwrap();
            let measure=doc.descendants().find(|n|n.has_tag_name("measure")).unwrap();
            let second=input[measure.range()].replace(&format!("number=\"{first_number}\""),&format!("number=\"{}\"",first_number+1));
            input=input.replace("</part>",&(second+"</part>"));
            let (canonical,voices,source)=fixture_from_input(&input);
            let geometry=source.receipt.original_beam_voice_groups.iter().map(|group|
                (group.id.clone(),if group.measure==first_number {4} else {1})).collect();
            let prepared=bind_primary_beam_voice_partition_plan(source.xml(),PrimaryBeamVoicePartitionBindingContext {
                source_continuation:&source,voice_plan:&voices,candidates:&[],classification_xml:source.xml(),
                renderer_visible_fragments_by_metric_line_id:&geometry,materialization_safety_proven_metric_line_ids:&BTreeSet::new(),
            }).unwrap();
            assert_eq!(prepared.scope_candidates(),2,"both actual original groups share the same import resources");
            assert_eq!(prepared.repaired_voice_plan().source_to_role,voices.source_to_role);
            let rebound=rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(),&source,&prepared).unwrap();
            let edits=rebound.edits().iter().map(|e|(e.voice_text_start,e.voice_text_end,e.target_label.clone())).collect();
            let (post,receipt)=crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(source.xml(),edits);
            let post=post.unwrap();
            assert_eq!(receipt.unwrap().realize_post_document(source.xml()).as_deref(),Some(post.as_str()));
            transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(&source,source.xml(),&post,&canonical,&rebound,Some(&BTreeSet::new())).unwrap();
        }
    }

    #[test]
    fn congruent_secondary_group_uses_its_existing_channel_without_changing_roles() {
        let (canonical,mut voices,source)=fixture();
        for role in voices.source_to_role.values_mut() { *role=SerializedVoicePresentationRole::UpperSecondary; }
        for role in voices.channel_to_role.values_mut() { *role=SerializedVoicePresentationRole::UpperSecondary; }
        let prepared=plan(&voices,&source,4);
        assert_eq!(prepared.scope_candidates(),1);
        assert_eq!(voices.source_to_role,prepared.repaired_voice_plan().source_to_role);
        let rebound=rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(),&source,&prepared).unwrap();
        let edits=rebound.edits().iter().map(|e|(e.voice_text_start,e.voice_text_end,e.target_label.clone())).collect();
        let (after,receipt)=crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(source.xml(),edits);
        let after=after.unwrap();
        assert_eq!(receipt.unwrap().realize_post_document(source.xml()).as_deref(),Some(after.as_str()));
        transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(&source,source.xml(),&after,&canonical,&rebound,Some(&BTreeSet::new())).unwrap();
    }

}

#[cfg(test)]
impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn synthetic_assert_adopted_bridge_alignment_negatives(
        &self, source: &FinalPdfSourceOccurrenceContinuation, voices: &FrozenSerializedVoicePlan,
        prepared: &PrimaryBeamVoicePartitionBoundPlan, carried: &FinalPdfSourceOccurrenceContinuation,
    ) {
        let bind = |candidate: &FinalPdfSourceOccurrenceContinuation, labels: &FrozenSerializedVoicePlan| {
            bind_primary_beam_voice_partition_plan(candidate.xml(), PrimaryBeamVoicePartitionBindingContext {
                source_continuation: candidate, voice_plan: labels, candidates: &[], classification_xml: candidate.xml(),
                renderer_visible_fragments_by_metric_line_id: &BTreeMap::new(),
                materialization_safety_proven_metric_line_ids: &BTreeSet::new(),
            }).unwrap()
        };
        assert!(!prepared.bridge_alignment_proofs.is_empty());
        let mut unallocated = voices.clone();
        unallocated.canonical_role_allocation = None;
        assert_eq!(bind(source,&unallocated).scope_candidates(),0,"extension resources require canonical allocation; existing original-only admission stays separate");
        let mut missing = source.clone();
        missing.receipt.rest_bridge_extension_course = None;
        assert_eq!(bind(&missing,voices).scope_candidates(),0,"beam tags do not recreate adoption");
        assert!(rebind_primary_beam_voice_partition_plan_to_current_xml(missing.xml(),&missing,prepared).is_err());
        assert!(transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
            &missing,missing.xml(),carried.xml(),self,prepared,Some(&BTreeSet::new())).is_err(),
            "direct typed transport must also refuse lost adoption");
        assert!(rebind_primary_beam_voice_partition_plan_to_current_xml(carried.xml(),carried,prepared).is_err(),"old connection proof is stale after the admitted voice write");
        let proof = &prepared.bridge_alignment_proofs[0];
        let group = source.receipt.original_beam_voice_groups.iter().find(|group|
            proof.head_sources().any(|id| group.members.iter().any(|member|member.0==id))).unwrap();
        let extension = proof.head_sources()
            .find(|id| !group.members.iter().any(|member|member.0==*id)).unwrap();
        let mut foreign = source.clone();
        for group in &mut foreign.receipt.original_beam_voice_groups {
            for ((id,_), voice) in std::sync::Arc::make_mut(&mut group.notation_voices) {
                if *id==extension {*voice="independent-semantic-course".to_owned();}
            }
        }
        assert!(bind(&foreign,voices).scope_candidates()<prepared.scope_candidates(),"a foreign semantic line is not an extension resource");
        let mut incomplete = source.clone();
        incomplete.receipt.occurrences.retain(|row|row.source_event_id!=extension);
        assert!(bind(&incomplete,voices).scope_candidates()<prepared.scope_candidates(),"every actual extension occurrence must be present");
        let again = bind(carried,&prepared.repaired_voice_plan);
        assert_eq!(again.scope_candidates(),0,"a completed component is idempotent");
    }
}
