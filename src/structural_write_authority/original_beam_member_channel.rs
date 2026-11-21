//! Whole-component member channel trials at the existing replacement owner.
use super::ordered_received_beam_readback::OwnerEdgeReadback::{Present, Unknown};
use super::primary_beam_tie_trials::{EdgeStates, lost_present_edges, master_edge_states, primary_edges};
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

type Identity = (usize, usize, usize);

struct RolePair {
    left: Identity,
    right: Identity,
    component: Option<usize>,
    joined_at: Option<&'static str>,
}

pub(super) struct RoleReadback {
    components: Vec<BTreeSet<usize>>,
    pairs: Vec<RolePair>,
}

impl RoleReadback {
    pub(super) fn new(
        canonical: &CanonicalFinalChordStructuralPlan,
        original: &ActualSourceBeamBindingObservation,
        baseline: &FrozenSerializedVoicePlan,
    ) -> Self {
        let components: Vec<_> = canonical
            .allocation_original_beam_groups()
            .map(|group| group.allocation_sources())
            .collect();
        let pairs = primary_edges(canonical, original)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|edge| {
                let (left, right) = (edge.left, edge.right);
                let roles = baseline
                    .source_to_role
                    .get(&left.source)
                    .zip(baseline.source_to_role.get(&right.source))?;
                if roles.0 == roles.1 {
                    return None;
                }
                let component = components.iter().position(|sources| {
                    sources.contains(&left.source) && sources.contains(&right.source)
                });
                Some(RolePair {
                    left: (left.source, left.attack, left.fragment),
                    right: (right.source, right.attack, right.fragment),
                    component,
                    joined_at: None,
                })
            })
            .collect();
        Self { components, pairs }
    }

    pub(super) fn observe(&mut self, plan: &FrozenSerializedVoicePlan, stage: &'static str) {
        for pair in &mut self.pairs {
            let labels = plan
                .label_for_source(pair.left.0)
                .zip(plan.label_for_source(pair.right.0));
            if labels.is_some_and(|(left, right)| left == right) {
                pair.joined_at = pair.joined_at.or(Some(stage));
            } else {
                pair.joined_at = None;
            }
        }
    }

    pub(super) fn append(&self, out: &mut String) {
        for pair in &self.pairs {
            let sources = pair.component.map(|index| &self.components[index]);
            let _ = writeln!(
                out,
                "OriginalBeamRolePair: source_attack_fragment={:?}; component_sources={sources:?}; joined_at={}; authority=none",
                (pair.left, pair.right),
                pair.joined_at.unwrap_or("none")
            );
        }
    }
}

impl FrozenSerializationVoiceChannelPlan {
    /// Replay the model's concrete allocation through the existing component
    /// owner. The reference keeps custody of all notes outside the witness.
    pub(super) fn check_exact_passage_witness(
        &self,
        canonical: &CanonicalFinalChordStructuralPlan,
        attacks: &[FrozenVoiceAttackEvidenceRow],
        original: &ActualSourceBeamBindingObservation,
        voices: &FrozenSerializedVoicePlan,
        moved: &BTreeMap<usize, usize>,
        target: (usize, usize),
    ) -> Result<(), String> {
        let mut allocation = moved.clone();
        for group in canonical.allocation_original_beam_groups() {
            let sources = group.allocation_sources();
            if sources.iter().any(|source| moved.contains_key(source)) {
                for source in sources {
                    let channel = self.channel_for_source(source).ok_or("unbound_component_source")?;
                    allocation.entry(source).or_insert(channel);
                }
            }
        }
        // A non-beam companion needs the separate countervoice owner's proof.
        // The model witness alone conveys no such permission.
        let reuse = cross_role_channel_sharing::ProjectionReuse::new(canonical, original)
            .with_source_voice_gaps(voices);
        let (_, mut trial) = self.checked_original_component_channel_assignment(
            canonical, attacks, original, voices, allocation, &reuse,
        ).map_err(|error| error.to_string())?;
        trial.bind_source_voice_gaps(
            voices.source_voice_gaps.iter().map(|(_, gap)| gap.clone()).collect(),
        );
        if trial.source_voice_gaps.len() != voices.source_voice_gaps.len() {
            return Err("witness_source_gap_binding_lost".into());
        }
        let gained = verify_master_retention(&mut String::new(), canonical, original, voices, &trial)
            .map_err(|error| error.to_string())?;
        gained.contains(&target).then_some(()).ok_or_else(|| "target_master_edge_not_gained".into())
    }

    pub(crate) fn checked_open_original_member_channels(
        &self,
        canonical: &CanonicalFinalChordStructuralPlan,
        attacks: &[FrozenVoiceAttackEvidenceRow],
        original: &ActualSourceBeamBindingObservation,
        before: &FrozenSerializedVoicePlan,
        gaps: &[crate::midi_seed::SourceVoiceGap],
        out: &mut String,
    ) -> Option<(Self, FrozenSerializedVoicePlan)> {
        let mut reference_voices = before.clone();
        reference_voices.bind_source_voice_gaps(gaps.to_vec());
        let before = &reference_voices;
        let reuse = &cross_role_channel_sharing::ProjectionReuse::new(canonical, original)
            .with_source_voice_gaps(before);
        let mut proposed = BTreeMap::new();
        let mut incompatible_proposals = false;
        for group in canonical.allocation_original_beam_groups() {
            let sources = group.allocation_sources();
            let Some(targets) = sources
                .iter()
                .map(|source| self.source_to_channel.get(source).copied())
                .collect::<Option<BTreeSet<_>>>()
            else {
                let _ = writeln!(
                    out,
                    "OriginalBeamMemberComponent: sources={sources:?}; result=unbound_channel; authority=none"
                );
                continue;
            };
            if targets.len() < 2 {
                continue;
            }
            let mut admitted = vec![];
            for channel in targets {
                let allocation = sources.iter().map(|&source| (source, channel)).collect();
                let result = self.checked_original_component_channel_assignment(
                    canonical, attacks, original, before, allocation, reuse,
                );
                match result {
                    Ok(_) => {
                        admitted.push(channel);
                        let _ = writeln!(
                            out,
                            "OriginalBeamMemberTarget: sources={sources:?}; channel={channel}; result=admitted; authority=none"
                        );
                    }
                    Err(error) => {
                        let _ = writeln!(
                            out,
                            "OriginalBeamMemberTarget: sources={sources:?}; channel={channel}; result=refused; reason={error}; authority=none"
                        );
                    }
                }
            }
            let _ = writeln!(
                out,
                "OriginalBeamMemberComponent: sources={sources:?}; admissible_channels={admitted:?}; authority=none"
            );
            if let [channel] = admitted.as_slice() {
                for &source in &sources {
                    if proposed
                        .insert(source, *channel)
                        .is_some_and(|previous| previous != *channel)
                    {
                        incompatible_proposals = true;
                    }
                }
            }
        }
        let exchange = self.original_countervoice_exchange(canonical);
        let mut countervoices = BTreeSet::new();
        if let Some((allocation, companions)) = exchange {
            for (source, channel) in allocation {
                if proposed.insert(source, channel).is_some_and(|prior| prior != channel) {
                    incompatible_proposals = true;
                }
            }
            countervoices = companions;
        }
        let _ = writeln!(
            out,
            "OriginalBeamCountervoiceExchange: witnesses={countervoices:?}; authority=none"
        );
        if incompatible_proposals || proposed.is_empty() {
            let reason = if incompatible_proposals {
                "incompatible_component_destinations"
            } else {
                "no_unique_member_destination"
            };
            let _ = writeln!(
                out,
                "OriginalBeamMemberBatch: result=retained_reference; reason={reason}; authority=none"
            );
            return None;
        }
        let result = self
            .checked_original_component_allocation(
                canonical, attacks, original, before, (proposed, countervoices), reuse,
            )
            .and_then(|(channels, mut voices)| {
                voices.bind_source_voice_gaps(gaps.to_vec());
                verify_master_retention(out, canonical, original, before, &voices)?;
                Ok((channels, voices))
            });
        match result {
            Ok(pair) => {
                let _ = writeln!(
                    out,
                    "OriginalBeamMemberBatch: result=admitted_for_master; authority=existing_component_owner"
                );
                Some(pair)
            }
            Err(error) => {
                let _ = writeln!(
                    out,
                    "OriginalBeamMemberBatch: result=retained_reference; reason={error}; authority=none"
                );
                None
            }
        }
    }
}

type ChannelAllocation = BTreeMap<usize, usize>;

fn unique<T>(mut candidates: impl Iterator<Item = T>) -> Option<T> {
    let first = candidates.next()?;
    candidates.next().is_none().then_some(first)
}

impl FrozenSerializationVoiceChannelPlan {
    fn original_countervoice_exchange(
        &self,
        canonical: &CanonicalFinalChordStructuralPlan,
    ) -> Option<(ChannelAllocation, BTreeSet<usize>)> {
        let written = canonical.written_projection.as_ref()?;
        let groups: Vec<_> = canonical.allocation_original_beam_groups()
            .map(|group| group.allocation_sources())
            .collect();
        let mut occurrences_by_staff = BTreeMap::<_, Vec<_>>::new();
        let mut by_source = BTreeMap::<_, Vec<_>>::new();
        for row in written.occurrences.iter() {
            occurrences_by_staff.entry((row.measure, row.staff)).or_default().push(row);
            by_source.entry(row.source_event_id).or_default().push(row);
        }
        let beam_sources: BTreeSet<_> = groups.iter().flatten().copied().collect();
        let mut exchanges = Vec::new();
        let mut uses = BTreeMap::<_, usize>::new();
        for rows in occurrences_by_staff.values() {
            for countervoice in rows.iter().copied().filter(|row| {
                !row.is_grace && row.onset < row.end && !beam_sources.contains(&row.source_event_id)
                    && by_source[&row.source_event_id].len() == 1
            }) {
                let Some(from) = self.channel_for_source(countervoice.source_event_id) else { continue };
                let targets = rows.iter().filter_map(|row| self.channel_for_source(row.source_event_id))
                    .filter(|&channel| channel != from).collect::<BTreeSet<_>>();
                let allocation = unique(targets.into_iter().filter_map(|target| {
                    self.countervoice_window(
                        canonical, rows, &by_source, &groups, countervoice, (from, target),
                    )
                }));
                let Some(allocation) = allocation else { continue };
                for &source in allocation.keys() {
                    *uses.entry(source).or_default() += 1;
                }
                exchanges.push((countervoice.source_event_id, allocation));
            }
        }
        let mut batch = BTreeMap::new();
        let mut countervoices = BTreeSet::new();
        for (countervoice, allocation) in exchanges {
            if allocation.keys().all(|source| uses[source] == 1) {
                countervoices.insert(countervoice);
                batch.extend(allocation);
            }
        }
        (!batch.is_empty()).then_some((batch, countervoices))
    }

    fn countervoice_window(
        &self,
        canonical: &CanonicalFinalChordStructuralPlan,
        rows: &[&FinalChordWrittenOccurrence],
        by_source: &BTreeMap<usize, Vec<&FinalChordWrittenOccurrence>>,
        groups: &[BTreeSet<usize>],
        countervoice: &FinalChordWrittenOccurrence,
        (from, target): (usize, usize),
    ) -> Option<ChannelAllocation> {
        let (start, end) = (countervoice.onset, countervoice.end);
        let channel = |source| self.channel_for_source(source);
        let mut inside: Vec<_> = rows.iter().copied().filter(|row| {
            channel(row.source_event_id) == Some(target) && row.onset < end && start < row.end
        }).collect();
        inside.sort_by_key(|row| row.onset);
        let first = inside.first()?;
        let last = inside.last()?;
        if first.onset != start || last.end != end || inside.len() < 2
            || inside.iter().any(|row| row.is_grace || row.onset >= row.end || by_source[&row.source_event_id].len() != 1)
            || inside.windows(2).any(|pair| pair[0].end != pair[1].onset)
        {
            return None;
        }
        let inside_sources: BTreeSet<_> = inside.iter().map(|row| row.source_event_id).collect();
        let group_at_boundary = |source, left| {
            unique(groups.iter().filter(|group| {
                group.contains(&source) && group.iter().all(|member| {
                    by_source.get(member).is_some_and(|members| matches!(members.as_slice(), [row]
                        if row.measure == countervoice.measure && row.staff == countervoice.staff
                        && (inside_sources.contains(member) || (channel(*member) == Some(from)
                            && (row.end <= start || row.onset >= end)))))
                }) && group.iter().any(|member| {
                    let row = by_source[member][0];
                    channel(*member) == Some(from)
                        && if left { row.end == start } else { row.onset == end }
                })
            }))
        };
        let incoming = group_at_boundary(first.source_event_id, true)?;
        let outgoing = group_at_boundary(last.source_event_id, false)?;
        if incoming == outgoing || !incoming.is_disjoint(outgoing)
            || !inside_sources.iter().all(|source| incoming.contains(source) || outgoing.contains(source))
        {
            return None;
        }
        let beams = incoming.union(outgoing).copied().collect();
        if !canonical.proves_common_original_notation_voice(&beams) {
            return None;
        }
        let has_common_origin = |source| {
            canonical.proves_common_original_notation_voice(&BTreeSet::from([
                source, countervoice.source_event_id,
            ]))
        };
        let neighbors = rows.iter().filter(|row| !row.is_grace)
            .map(|row| (row.source_event_id, row.onset, row.end));
        let (previous, next) = countervoice_neighbors(neighbors, (start, end), has_common_origin)?;
        if channel(previous) != Some(target)
            || channel(next) != Some(target)
        {
            return None;
        }
        let mut allocation: ChannelAllocation = beams.into_iter().map(|source| (source, from)).collect();
        allocation.insert(countervoice.source_event_id, target);
        Some(allocation)
    }
}

fn countervoice_neighbors(
    candidates: impl Iterator<Item = (usize, i64, i64)> + Clone,
    (start, end): (i64, i64),
    has_common_origin: impl Fn(usize) -> bool,
) -> Option<(usize, usize)> {
    let attached = |boundary, incoming| {
        unique(candidates.clone().filter_map(|(source, onset, end)| {
            let adjacent = if incoming { end == boundary } else { onset == boundary };
            (onset < end && adjacent && has_common_origin(source)).then_some(source)
        }))
    };
    Some((attached(start, true)?, attached(end, false)?))
}

#[cfg(test)]
mod tests {
    use super::countervoice_neighbors;

    #[test]
    fn a_countervoice_requires_one_bound_continuation_at_each_boundary() {
        let neighbors = [(0, 0, 2), (1, 1, 2), (2, 6, 8), (3, 6, 9), (4, 3, 5)];
        let select = |same_voice: &[usize]| {
            countervoice_neighbors(neighbors.iter().copied(), (2, 6), |source| {
                same_voice.contains(&source)
            })
        };
        assert_eq!(select(&[1, 2, 4]), Some((1, 2)));
        assert_eq!(select(&[0, 1, 2]), None);
        assert_eq!(select(&[1, 2, 3]), None);
        assert_eq!(select(&[0, 4]), None);
    }
}

pub(super) fn selected_master_edge_states(
    canonical: &CanonicalFinalChordStructuralPlan,
    original: &ActualSourceBeamBindingObservation,
    voices: &FrozenSerializedVoicePlan,
) -> io::Result<EdgeStates> {
    let input = canonical
        .written_projection
        .as_ref()
        .ok_or_else(|| io::Error::other("master_reference_input_unbound"))?
        .original_xml
        .as_ref();
    let edges = primary_edges(canonical, original).map_err(io::Error::other)?;
    let locators = canonical
        .source_locators
        .iter()
        .map(|row| (row.fragment_id, row))
        .collect();
    let plan = bind_canonical_final_chord_encoding_plan(input, canonical, voices)?;
    let (xml, receipt) = project_final_chord_encoding_plan(input, &plan);
    master_edge_states(&plan, (input, &xml?, receipt.as_ref()), &edges, &locators)
        .map_err(io::Error::other)
}

fn verify_master_retention(
    out: &mut String,
    canonical: &CanonicalFinalChordStructuralPlan,
    original: &ActualSourceBeamBindingObservation,
    before: &FrozenSerializedVoicePlan,
    after: &FrozenSerializedVoicePlan,
) -> io::Result<Vec<(usize, usize)>> {
    let input = canonical.written_projection.as_ref()
        .ok_or_else(|| io::Error::other("master_reference_input_unbound"))?.original_xml.as_ref();
    let alpha = audit_musicxml_voice_label_binding_alpha(input, canonical, after)?;
    if !alpha.green() {
        return Err(io::Error::other("master_candidate_label_alpha_failed"));
    }
    let reference = selected_master_edge_states(canonical, original, before)?;
    if reference.is_empty() || reference.values().any(|state| *state == Unknown) {
        return Err(io::Error::other("master_reference_edges_unresolved"));
    }
    let candidate = selected_master_edge_states(canonical, original, after)?;
    let lost = lost_present_edges(&reference, &candidate);
    let gained: Vec<_> = candidate
        .iter()
        .filter_map(|(&pair, &state)| {
            (state == Present && reference.get(&pair).is_some_and(|state| *state != Present))
                .then_some(pair)
        })
        .collect();
    let present = reference
        .values()
        .filter(|&&state| state == Present)
        .count();
    let _ = writeln!(
        out,
        "OriginalBeamMemberMasterProof: reference_present={present}; preserved={}; gained={gained:?}; lost={lost:?}; authority=none",
        present - lost.len()
    );
    if !lost.is_empty() {
        return Err(io::Error::other("master_reference_edge_lost"));
    }
    if gained.is_empty() {
        return Err(io::Error::other("no_master_edge_gained"));
    }
    Ok(gained)
}
