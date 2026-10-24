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

#[derive(Debug, Clone, PartialEq, Eq)]
struct OriginalPositiveBeamOrigin {
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
}

impl CanonicalFinalChordStructuralPlan {
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
            self.original_beam_voice_groups
                .push(OriginalBeamVoiceGroup {
                    positive_origins: vec![OriginalPositiveBeamOrigin {
                        members: members.iter().zip(&fragments).map(|(&(source,attack),&fragment)|(source,attack,fragment)).collect(),
                        states: states.clone(),
                    }],
                    id: format!(
                        "original-beam-{:016x}-{}-{}",
                        self.semantic_fingerprint_fnv64, identity.0, identity.1
                    ),
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
                });
        }
        let bound_seeds = self.original_beam_voice_groups.iter().map(|group|group.id.clone()).collect::<BTreeSet<_>>();
        self.original_beam_voice_groups =
            close_shared_sources(std::mem::take(&mut self.original_beam_voice_groups));
        let retained = self.original_beam_voice_groups.iter().flat_map(|group|group.id.split('+').map(str::to_owned)).collect::<BTreeSet<_>>();
        for ((owner_group,owner_piece),count) in original_counts {
            let id=format!("original-beam-{:016x}-{owner_group}-{owner_piece}",self.semantic_fingerprint_fnv64);
            let locations = original_locations.get(&(owner_group,owner_piece));
            let unusual = beam_diagnostics::record(Observation::Census {
                bound_seed: bound_seeds.contains(&id),
                retained_component: retained.contains(&id),
                source_locations_present: locations.is_some(),
            });
            if unusual || locations.is_some_and(|rows| rows.iter().any(|row| beam_diagnostics::detail_requested_for_measure(row.0))) {
                eprintln!("OriginalBeamOriginCensus: group={id}; original_members={count}; bound_seed={}; retained_component={}; source_locations={:?}; origin=actual_original_owner; authority=none",bound_seeds.contains(&id),retained.contains(&id),locations);
            }
        }
    }
}

impl OriginalBeamVoiceGroup {
    pub(super) fn closed_bridge_origins(&self, authored: &BTreeSet<OriginalBeamOccurrenceKey>) -> ClosedBridgeOrigins {
        self.positive_origins.iter().enumerate().filter(|(_,origin)|
            closed_primary(&origin.states) && origin.members.is_subset(authored))
            .map(|(index,origin)|(index,origin.members.clone())).collect()
    }
    pub(super) fn bridge_component_occurrences(&self) -> BTreeSet<(usize,usize,usize)> {
        self.members.iter().zip(&self.fragments).map(|(&(source,attack),&fragment)|(source,attack,fragment)).collect()
    }

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
        if component.iter().any(|group| {
            group.measure != first.measure
                || group.staff != first.staff
                || group.semantic_voice != first.semantic_voice
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
        if row.measure != group.measure || row.fragment_id != group.fragments[index] {
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

pub(super) fn renderer_cohorts(
    source: &FinalPdfSourceOccurrenceContinuation,
) -> Vec<PrimaryBeamVoicePartitionRendererVerificationCohort> {
    source
        .receipt
        .original_beam_voice_groups
        .iter()
        .filter_map(|group| {
            let rows = exact_members(source, group)?;
            let mut ordinals = rows
                .iter()
                .map(|row| row.pitched_ordinal)
                .collect::<Vec<_>>();
            ordinals.sort_unstable();
            let unusual = beam_diagnostics::record(Observation::RendererCohort);
            if unusual || beam_diagnostics::detail_requested_for_measure(group.measure) {
                eprintln!("OriginalBeamRendererCohort: group={}; measure={}; staff={}; primary_groups={}; pitched_ordinals={ordinals:?}; origin=actual_owner_continuation; authority=none", group.id, group.measure, group.staff, group.primary_group_count);
            }
            Some(PrimaryBeamVoicePartitionRendererVerificationCohort {
                metric_line_id: group.id.clone(),
                measure: group.measure,
                staff: group.staff,
                pitched_ordinals: ordinals,
            })
        })
        .collect()
}

pub(super) struct AlignmentInput<'a> {
    pub(super) source_xml: &'a str,
    pub(super) source: &'a FinalPdfSourceOccurrenceContinuation,
    pub(super) voices: &'a FrozenSerializedVoicePlan,
    pub(super) classification_xml: &'a str,
    pub(super) visible_fragments: &'a BTreeMap<String, usize>,
}

pub(super) struct AlignmentOutput<'a> {
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
        visible_fragments,
    } = input;
    let AlignmentOutput {
        repaired,
        edits,
        selected,
    } = output;
    if !source.has_authentic_beam_surface()? {
        return Ok(Vec::new());
    }
    let current = final_chord_parse_binding_measures(source_xml)?;
    let classification = final_chord_parse_binding_measures(classification_xml)?;
    let claimed = selected
        .iter()
        .map(|row| (row.source_event_id, row.attack_id))
        .collect::<BTreeSet<_>>();
    if validate_batch_occupancy(source_xml, edits).is_err() {
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
    for group in &source.receipt.original_beam_voice_groups {
        let Some(rows) = exact_members(source, group) else { continue; };
        let Some(measure) = current.iter().find(|measure| measure.measure == group.measure) else { continue; };
        let pitched = measure.notes.iter().filter(|note| !note.rest_like && note.pitch_midi.is_some()).collect::<Vec<_>>();
        let Some(labels) = rows.iter().map(|row| pitched.get(row.pitched_ordinal)
            .map(|note| note.voice.trim().to_owned())).collect::<Option<BTreeSet<_>>>() else { continue; };
        if visible_fragments.get(&group.id).is_some_and(|count| *count > group.primary_group_count) {
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
        let observe = |reason: &str| {
            let unusual = beam_diagnostics::record(Observation::Decision(reason));
            if unusual || beam_diagnostics::detail_requested_for_measure(group.measure) {
                eprintln!("OriginalBeamAlignmentDecision: group={}; measure={}; staff={}; primary_groups={}; visible_fragments={:?}; reason={reason}; phase=prepared_not_committed; authority=none", group.id, group.measure, group.staff, group.primary_group_count, visible_fragments.get(&group.id));
            }
        };
        let import_component = group_labels.get(&group.id)
            .is_some_and(|labels| labels.len() > 1 && !labels.is_disjoint(&component_labels));
        if visible_fragments.get(&group.id).is_none() && !import_component { observe("geometry_unknown"); continue; }
        if visible_fragments.get(&group.id).is_some_and(|count| *count <= group.primary_group_count) && !import_component { observe("original_components_preserved"); continue; }
        if import_component { observe("original_import_resource_component"); }
        if group.members.iter().any(|member| claimed.contains(member)) { observe("existing_partition_claim"); continue; }
        let Some(rows) = exact_members(source, group) else {
            observe("source_closure"); continue;
        };
        let Some(measure) = current.iter().find(|m| m.measure == group.measure) else {
            observe("missing_current_measure"); continue;
        };
        let Some(classified) = classification.iter().find(|m| m.measure == group.measure) else {
            observe("missing_classification_measure"); continue;
        };
        let pitched = measure
            .notes
            .iter()
            .filter(|n| !n.rest_like && n.pitch_midi.is_some())
            .collect::<Vec<_>>();
        let observed = classified
            .notes
            .iter()
            .filter(|n| !n.rest_like && n.pitch_midi.is_some())
            .collect::<Vec<_>>();
        let mut notes = Vec::new();
        let mut complete = true;
        for (index, row) in rows.iter().enumerate() {
            let (Some(note), Some(classified_note)) = (
                pitched.get(row.pitched_ordinal),
                observed.get(row.pitched_ordinal),
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
            notes.push(*note);
        }
        if !complete
            // A rest gap does not rescind the original owner's explicit
            // group. Pitched occupancy may not overlap; the full batch still
            // protects every visible bystander rest and channel resource.
            || notes.windows(2).any(|pair| pair[0].onset.checked_add(pair[0].duration)
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
        if targets.is_empty() {
            eprintln!("OriginalBeamAllocationRejection: group={}; canonical_allocation={}; source_and_channel_roles={:?}; authority=none",group.id,voices.has_canonical_role_allocation(),group.members.iter().zip(&notes).map(|(&(id,_),note)|(voices.role_for_source(id),channel_role(note))).collect::<Vec<_>>());
            observe("no_existing_congruent_staff_anchor"); continue;
        }
        let Some(resources) = bound_chord_resources(BoundChordResourceContext { source, classification_xml, group, current: &current, classification: &classification, voices, claimed: &claimed }, &notes, &rows) else {
            observe("chord_resource_closure"); continue;
        };
        if resources.iter().map(|(note,_)|note.voice.trim()).collect::<BTreeSet<_>>().len()<=1 {
            observe("complete_resource_image_already_one_label");continue;
        }
        let mut admitted_target = None;
        for target_index in targets {
            let target_label = notes[target_index].voice.trim();
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
            let (collisions, lost_registration) = new_batch_conflicts(source_xml,&trial)?;
            let unusual = beam_diagnostics::record(Observation::Admission {
                scalar_changes: group_edits.len(),
                collisions: !collisions.is_empty(),
                lost_registration: !lost_registration.is_empty(),
            });
            if unusual || beam_diagnostics::detail_requested_for_measure(group.measure) {
                eprintln!("OriginalBeamTargetResourceAdmission: group={}; target={target_label}; scalar_changes={}; collisions={collisions:?}; lost_registration={lost_registration:?}; authority=none",group.id,group_edits.len());
            }
            // A bystander rest/pitched collision is still a whole-group veto.
            // An alternative is considered for an actual import resource loss,
            // never merely to evade that existing occupancy negative.
            if !collisions.is_empty() && lost_registration.is_empty() { break; }
            if collisions.is_empty() && lost_registration.is_empty()
                && validate_batch_import_profile(source_xml, &trial).is_ok() {
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
        proposals.push((group,target_index,target_channel,target_label,group_edits,resources));
    }
    // All proposals are evaluated together. If simultaneous assignments
    // conflict, every proposal whose changed occurrence participates is
    // withdrawn. No opaque group ordering chooses a winner.
    loop {
        let mut batch = edits.clone();
        for (_, _, _, _, proposed, _) in &proposals {
            batch.extend(proposed.clone());
        }
        let (conflicts, headless) = new_batch_conflicts(source_xml, &batch)?;
        if conflicts.is_empty() && headless.is_empty() {
            break;
        }
        let before = proposals.len();
        proposals.retain(|(_, _, _, _, proposed, _)| {
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
    for (group, target_index, target_channel, target_label, proposed, resources) in proposals {
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
    labels: &BTreeMap<(usize, usize), String>,
) -> io::Result<BTreeSet<(usize, usize, usize)>> {
    let mut result = BTreeSet::new();
    for measure in final_chord_parse_binding_measures(xml)? {
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

fn new_batch_conflicts(
    source: &str,
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> io::Result<ChannelSafety> {
    let before = occupancy_conflicts(source, &BTreeMap::new())?;
    let measures = final_chord_parse_binding_measures(source)?;
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
    let new_headless = headless_labels(source, &labels)?
        .difference(&headless_labels(source, &BTreeMap::new())?)
        .cloned()
        .collect();
    Ok((
        occupancy_conflicts(source, &labels)?
            .difference(&before)
            .copied()
            .collect(),
        new_headless,
    ))
}

pub(super) fn validate_batch_occupancy(
    source: &str,
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> io::Result<()> {
    let (conflicts, headless) = new_batch_conflicts(source, edits)?;
    if !conflicts.is_empty() || !headless.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "original beam alignment introduces a channel occupancy conflict",
        ));
    }
    validate_batch_import_profile(source, edits)
}

fn validate_batch_import_profile(
    source: &str,
    edits: &[PrimaryBeamVoicePartitionBoundEdit],
) -> io::Result<()> {
    let mut projected = source.to_owned();
    let mut ordered = edits.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|edit| std::cmp::Reverse(edit.voice_text_start));
    for edit in ordered {
        projected.replace_range(edit.voice_text_start..edit.voice_text_end, &edit.target_label);
    }
    if !crate::audit_only::import_channel_profile_readback::partwide_import_channels_admissible(source)
        || !crate::audit_only::import_channel_profile_readback::partwide_import_channels_admissible(&projected)
    {
        return Err(io::Error::new(io::ErrorKind::InvalidData,
            "original beam alignment violates the complete partwide import-channel profile"));
    }
    Ok(())
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
        let group = source
            .receipt
            .original_beam_voice_groups
            .iter()
            .find(|g| &g.id == id)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "original beam origin group lost",
                )
            })?;
        let rows = exact_members(source, group).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "original beam late occurrence ambiguity",
            )
        })?;
        let measure = measures
            .iter()
            .find(|m| m.measure == group.measure)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "original beam late measure missing",
                )
            })?;
        for (index, row) in rows.iter().enumerate() {
            let note = measure
                .notes
                .iter()
                .filter(|n| !n.rest_like && n.pitch_midi.is_some())
                .nth(row.pitched_ordinal)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "original beam late note missing",
                    )
                })?;
            let original_note = source_measures.iter().find(|m| m.measure == group.measure)
                .and_then(|m| m.notes.iter().filter(|n| !n.rest_like && n.pitch_midi.is_some()).nth(row.pitched_ordinal))
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "original beam course lost its source note"))?;
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
fn actual_beam_levels(xml: &str, note: &FinalChordEncodingNote) -> Option<BTreeMap<u8, String>> {
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

fn headless_labels(
    xml: &str,
    labels: &BTreeMap<(usize, usize), String>,
) -> io::Result<BTreeSet<String>> {
    let mut registered = BTreeSet::new();
    let mut required = BTreeSet::new();
    for measure in final_chord_parse_binding_measures(xml)? {
        for note in measure.notes {
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
    Ok(required.difference(&registered).cloned().collect())
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
}
fn bound_chord_resources<'a>(
    context: BoundChordResourceContext<'a>,
    heads: &[&'a FinalChordEncodingNote],
    rows: &[&'a FinalChordTransactionOutputOccurrenceWitness],
) -> Option<Vec<(&'a FinalChordEncodingNote, &'a FinalChordTransactionOutputOccurrenceWitness)>> {
    let BoundChordResourceContext { source, classification_xml, group, current, classification, voices, claimed } = context;
    let mut resources=heads.iter().copied().zip(rows.iter().copied()).collect::<Vec<_>>();
    loop {
        let before=resources.len();
        let keys=resources.iter().map(|(_,r)|(r.source_event_id,r.attack_id)).collect::<BTreeSet<_>>();
        let mut required=source.receipt.occurrences.iter().filter(|r|keys.contains(&(r.source_event_id,r.attack_id)))
            .collect::<Vec<_>>();
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


#[cfg(test)]
pub(super) fn bridge_test_actual_alignment_plan(
    voices:&FrozenSerializedVoicePlan,source:&FinalPdfSourceOccurrenceContinuation,
) -> PrimaryBeamVoicePartitionBoundPlan { tests::plan(voices,source,4) }


#[cfg(test)]
mod tests {
    use super::*;

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
        let (original, receipt, capture) =
            crate::pipeline::actual_beam_owner_capture_tests::execute(before);
        capture.binds_original_receipt(&receipt).unwrap();
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
    #[ignore = "requires a separately installed MuseScore 4.7.4 and ETUSCHLOPF_BEAM_RENDER_WRAPPER"]
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
    fn historical_phrase_purpose_excludes_original_group_extension_and_survives_rebind() {
        let (_,voices,source)=fixture();
        let combined=plan(&voices,&source,4);
        assert_eq!(combined.scope_candidates(),1);
        let geometry=source.receipt.original_beam_voice_groups.iter().map(|group|(group.id.clone(),4usize)).collect();
        let ordinary=bind_primary_beam_voice_partition_plan_for_purpose(source.xml(),PrimaryBeamVoicePartitionBindingContext {
            source_continuation:&source,voice_plan:&voices,candidates:&[],classification_xml:source.xml(),
            renderer_visible_fragments_by_metric_line_id:&geometry,materialization_safety_proven_metric_line_ids:&BTreeSet::new(),
        },true).unwrap();
        assert_eq!(ordinary.scope_candidates(),0);
        assert!(ordinary.edits().is_empty());
        assert_eq!(ordinary.repaired_voice_plan().source_to_label,voices.source_to_label);
        assert_eq!(combined.repaired_voice_plan().source_to_role,ordinary.repaired_voice_plan().source_to_role);
        assert_ne!(combined.repaired_voice_plan().source_to_label,ordinary.repaired_voice_plan().source_to_label);
        assert!(!ordinary.permits_pdf_finalization());
        assert!(combined.permits_pdf_finalization());
        assert!(!rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(),&source,&ordinary).unwrap().permits_pdf_finalization());
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
