//! Exact counterplans for a finite, source-bound passage. No allocation is returned to the writer.
use super::actual_source_beam_binding::BoundOccurrence;
use super::ordered_received_beam_readback::{OwnerEdge, OwnerEdgeReadback};
use super::primary_beam_pair_readback::MasterOccurrences;
use super::primary_beam_tie_trials::primary_edges;
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

type Identity = (usize, usize, usize);
type Interval = (i64, i64);
type Relation = (usize, usize);

fn identity(row: &BoundOccurrence) -> Identity {
    (row.source, row.attack, row.fragment)
}

fn overlaps(a: Interval, b: Interval) -> bool {
    a.0 < b.1 && b.0 < a.1
}

fn ordinary_role(role: SerializedVoicePresentationRole, staff: u8) -> bool {
    use SerializedVoicePresentationRole::*;
    matches!(
        (staff, role),
        (1, UpperPrimary | UpperSecondary) | (2, LowerPrimary | LowerSecondary)
    )
}

struct Variable {
    id: Identity,
    written: Interval,
    sounding: Interval,
    current: usize,
    choices: Vec<usize>,
}

#[derive(Default)]
struct Domains {
    choices: Vec<Vec<usize>>,
    // For variable i, all earlier partners and the required channel equality.
    neighbors: Vec<Vec<(usize, bool)>>,
}

#[derive(Default)]
struct Witnesses {
    with: Vec<Vec<usize>>,
    positive_seen: usize,
    without: Option<Vec<usize>>,
}

impl Witnesses {
    fn result(&self, complete: bool) -> &'static str {
        match (!self.with.is_empty(), self.without.is_some(), complete) {
            (true, true, _) => "ambiguous_in_model",
            (true, false, true) => "forced_in_model",
            (false, true, true) => "excluded_in_model",
            _ => "open",
        }
    }
}

struct Search {
    witnesses: Vec<Witnesses>,
    plans: usize,
    steps: usize,
    limit: usize,
    complete: bool,
}

impl Domains {
    fn accepts(&self, plan: &[usize]) -> bool {
        plan.len() == self.choices.len()
            && plan.iter().enumerate().all(|(i, channel)| {
                self.choices[i].contains(channel)
                    && self.neighbors[i]
                        .iter()
                        .all(|&(j, equal)| (plan[j] == *channel) == equal)
            })
    }

    fn search(&self, queries: &[Relation], limit: usize) -> Search {
        let mut result = Search {
            witnesses: queries.iter().map(|_| Witnesses::default()).collect(),
            plans: 0,
            steps: 0,
            limit,
            complete: true,
        };
        self.visit(queries, &mut vec![], &mut result);
        result
    }

    fn visit(&self, queries: &[Relation], plan: &mut Vec<usize>, result: &mut Search) {
        let index = plan.len();
        if index == self.choices.len() {
            result.plans += 1;
            for (&(left, right), witnesses) in queries.iter().zip(&mut result.witnesses) {
                if plan[left] == plan[right] {
                    witnesses.positive_seen += 1;
                    // Each full assignment is visited once. Only the expensive
                    // owner projections are capped; the model search continues.
                    if witnesses.with.len() < 8 {
                        witnesses.with.push(plan.clone());
                    }
                } else {
                    witnesses.without.get_or_insert_with(|| plan.clone());
                }
            }
            return;
        }
        for &channel in &self.choices[index] {
            if result.steps == result.limit {
                result.complete = false;
                return;
            }
            result.steps += 1;
            if self.neighbors[index]
                .iter()
                .all(|&(earlier, equal)| (plan[earlier] == channel) == equal)
            {
                plan.push(channel);
                self.visit(queries, plan, result);
                plan.pop();
                if !result.complete {
                    return;
                }
            }
        }
    }
}

struct Observation<'a> {
    channels: &'a FrozenSerializationVoiceChannelPlan,
    canonical: &'a CanonicalFinalChordStructuralPlan,
    original: &'a ActualSourceBeamBindingObservation,
    voices: &'a FrozenSerializedVoicePlan,
    attacks: &'a [FrozenVoiceAttackEvidenceRow],
    by_source: BTreeMap<usize, Vec<&'a FrozenVoiceAttackEvidenceRow>>,
    written: &'a FinalChordWrittenProjectionEvidence,
    written_by_source: BTreeMap<usize, Vec<&'a FinalChordWrittenOccurrence>>,
    master: MasterOccurrences,
    // The Master receipt maps chord members to their beamed head. Only
    // independently bound singleton carriers enter this first comparison.
    by_location: BTreeMap<(usize, usize), Vec<Identity>>,
    retained: Vec<(usize, usize)>,
}

struct Passage {
    variables: Vec<Variable>,
    domains: Domains,
    measure: usize,
    window: Interval,
}

impl FrozenSerializationVoiceChannelPlan {
    pub(crate) fn append_exact_voice_passages(
        &self,
        out: &mut String,
        canonical: &CanonicalFinalChordStructuralPlan,
        attacks: &[FrozenVoiceAttackEvidenceRow],
        original: &ActualSourceBeamBindingObservation,
        voices: &FrozenSerializedVoicePlan,
    ) {
        let result = (|| -> Result<(), &'static str> {
            let written = canonical
                .written_projection
                .as_ref()
                .ok_or("missing_written_projection")?;
            let input = &written.original_xml;
            let mut written_by_source = BTreeMap::<_, Vec<_>>::new();
            for row in written.occurrences.iter() {
                written_by_source
                    .entry(row.source_event_id)
                    .or_default()
                    .push(row);
            }
            let plan = bind_canonical_final_chord_encoding_plan(input, canonical, voices)
                .map_err(|_| "master_plan_unbound")?;
            let (xml, receipt) = project_final_chord_encoding_plan(input, &plan);
            let xml = xml.map_err(|_| "master_projection_unavailable")?;
            let master = MasterOccurrences::bind(&plan, input, &xml, receipt.as_ref())?;
            let locators = canonical
                .source_locators
                .iter()
                .map(|row| (row.fragment_id, row))
                .collect();
            let edges = primary_edges(canonical, original)?;
            let mut states = Vec::new();
            let mut retained = Vec::new();
            for edge in &edges {
                let state = master.read_edge(&xml, edge, &locators);
                if state == OwnerEdgeReadback::Present {
                    retained.push((edge.left.source, edge.right.source));
                }
                states.push(state);
            }
            let mut by_location = BTreeMap::<_, Vec<_>>::new();
            for (&id, &location) in &master.by_source_occurrence {
                by_location.entry(location).or_default().push(id);
            }
            let mut by_source = BTreeMap::<_, Vec<_>>::new();
            for row in attacks {
                by_source.entry(row.source_event_id).or_default().push(row);
            }
            let split = states
                .iter()
                .filter(|&&s| s == OwnerEdgeReadback::SplitVoice)
                .count();
            let _ = writeln!(
                out,
                "ExactVoicePassageSummary: primary={}; present={}; split={split}; other={}; model=fixed_environment_same_channel; authority=none",
                edges.len(),
                retained.len(),
                edges.len() - split - retained.len()
            );
            let obs = Observation {
                channels: self,
                canonical,
                original,
                voices,
                attacks,
                by_source,
                written,
                written_by_source,
                master,
                by_location,
                retained,
            };
            let mut groups = BTreeMap::<_, Vec<_>>::new();
            for (edge, state) in edges.iter().zip(states) {
                groups.entry(edge.group).or_default().push((edge, state));
            }
            for group in groups.values() {
                if group
                    .iter()
                    .any(|(_, s)| *s == OwnerEdgeReadback::SplitVoice)
                {
                    obs.report(out, group);
                }
            }
            Ok(())
        })();
        if let Err(reason) = result {
            let _ = writeln!(
                out,
                "ExactVoicePassageSummary: result=open; reason={reason}; authority=none"
            );
        }
    }
}

impl Observation<'_> {
    fn passage(
        &self,
        group: &[(&OwnerEdge<'_>, OwnerEdgeReadback)],
    ) -> Result<Passage, &'static str> {
        let mut frame = None;
        let mut start = i64::MAX;
        let mut end = i64::MIN;
        for (edge, _) in group {
            for event in [edge.left, edge.right] {
                let &(measure, index) = self
                    .master
                    .by_source_occurrence
                    .get(&identity(event))
                    .ok_or("group_carrier_unbound")?;
                let note = &self.master.measures[measure].notes[index];
                let staff = note
                    .staff
                    .parse::<u8>()
                    .map_err(|_| "group_staff_unbound")?;
                if frame
                    .replace((measure, staff))
                    .is_some_and(|before| before != (measure, staff))
                {
                    return Err("group_crosses_written_frame");
                }
                start = start.min(note.onset);
                end = end.max(
                    note.onset
                        .checked_add(note.duration)
                        .ok_or("invalid_written_end")?,
                );
            }
        }
        let (measure_index, staff) = frame.ok_or("group_empty")?;
        let measure = &self.master.measures[measure_index];
        let window = (start, end);
        // At a barline the adjacent measure is outside this model. Keep
        // each carrier touching the boundary on its selected channel.
        let last_release = measure
            .notes
            .iter()
            .map(|n| n.onset.checked_add(n.duration))
            .collect::<Option<Vec<_>>>()
            .ok_or("invalid_written_end")?
            .into_iter()
            .max()
            .ok_or("empty_measure")?;
        let mut variables = Vec::new();
        let mut boundary = Vec::new();
        let mut occupied_rests = Vec::new();
        for (index, note) in measure
            .notes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.staff.parse() == Ok(staff))
        {
            let times = (
                note.onset,
                note.onset
                    .checked_add(note.duration)
                    .ok_or("invalid_written_end")?,
            );
            let inside = overlaps(times, window);
            let adjacent = times.1 == start || times.0 == end;
            if !inside && !adjacent && !(note.duration == 0 && (start..=end).contains(&note.onset))
            {
                continue;
            }
            if note.duration <= 0 {
                return Err("grace_or_empty_carrier");
            }
            let current = self
                .voices
                .channel_for_label(&note.voice)
                .ok_or("master_channel_unbound")?;
            if note.rest_like {
                if inside {
                    occupied_rests.push((times, current));
                }
                continue;
            }
            let Some(&[id]) = self
                .by_location
                .get(&(measure_index, index))
                .map(Vec::as_slice)
            else {
                return Err("unbound_or_chord_carrier");
            };
            if note.chord_member
                || note.pitch_midi.is_none()
                || note.tie_start
                || note.tie_stop
                || self.channels.channel_for_source(id.0) != Some(current)
                || !self
                    .canonical
                    .proves_common_original_notation_voice(&[id.0].into())
            {
                return Err("non_simple_or_unbound_source");
            }
            if self
                .canonical
                .source_locators
                .iter()
                .filter(|row| row.source_event_id == id.0)
                .count()
                != 1
            {
                return Err("source_has_multiple_fragments");
            }
            let Some(&[occurrence]) = self.written_by_source.get(&id.0).map(Vec::as_slice) else {
                return Err("written_occurrence_not_unique");
            };
            if occurrence.attack_id != id.1
                || occurrence.fragment_id != id.2
                || occurrence.emission_ordinal != 0
                || occurrence.measure != measure.measure
                || occurrence.staff != staff
                || (occurrence.onset, occurrence.end) != times
                || occurrence.is_grace
                || !occurrence.has_pitched_chord_anchor
                || occurrence.chord_anchor_start != occurrence.projected_note_start
            {
                return Err("written_and_master_occurrence_disagree");
            }
            if !inside {
                boundary.push((id.0, times, current));
                continue;
            }
            let Some(&[attack]) = self.by_source.get(&id.0).map(Vec::as_slice) else {
                return Err("sounding_source_unbound");
            };
            if attack.attack_id != id.1
                || attack.staff != staff
                || attack.absolute_end <= attack.absolute_start
            {
                return Err("sounding_source_disagrees");
            }
            let role = self.channels.role_for_source(id.0).ok_or("missing_role")?;
            let touches_bar =
                times.0 == 0 || times.1 == last_release || times.1 >= measure.expected_duration;
            let choices =
                if times.0 < start || times.1 > end || touches_bar || !ordinary_role(role, staff) {
                    vec![current]
                } else {
                    self.channels
                        .channel_to_role
                        .iter()
                        .filter_map(|(&channel, &role)| {
                            ordinary_role(role, staff).then_some(channel)
                        })
                        .collect()
                };
            variables.push(Variable {
                id,
                written: times,
                sounding: (attack.absolute_start, attack.absolute_end),
                current,
                choices,
            });
        }
        if variables.len() > 24 {
            return Err("passage_size_limit");
        }
        let indices: BTreeMap<_, _> = variables
            .iter()
            .enumerate()
            .map(|(i, v)| (v.id.0, i))
            .collect();
        if variables.len() != indices.len() {
            return Err("repeated_source_in_passage");
        }

        // Bind each independently unique original continuation at the outer
        // boundaries. Several matching neighbors remain alternatives.
        for v in &mut variables {
            v.choices.retain(|channel| {
                !occupied_rests
                    .iter()
                    .any(|&(rest, owner)| owner == *channel && overlaps(v.written, rest))
            });
            for &(incoming, time) in &[(true, start), (false, end)] {
                if (if incoming { v.written.0 } else { v.written.1 }) != time {
                    continue;
                }
                let related: Vec<_> = boundary
                    .iter()
                    .filter(|(source, times, _)| {
                        (if incoming { times.1 } else { times.0 }) == time
                            && self
                                .canonical
                                .proves_common_original_notation_voice(&[v.id.0, *source].into())
                    })
                    .collect();
                if !related.is_empty() {
                    v.choices
                        .retain(|channel| related.iter().any(|row| row.2 == *channel));
                }
            }
            // Unchanged sources on every staff still occupy their written resources
            // on the replacement owner's shared channels, including other fragments.
            for row in self.written.occurrences.iter().filter(|row| {
                row.measure == measure.measure
                    && !indices.contains_key(&row.source_event_id)
                    && overlaps(v.written, (row.onset, row.end))
            }) {
                let fixed = self
                    .channels
                    .channel_for_source(row.source_event_id)
                    .ok_or("external_written_channel_unbound")?;
                v.choices.retain(|channel| *channel != fixed);
            }
            for other in self.attacks.iter().filter(|a| {
                !indices.contains_key(&a.source_event_id)
                    && overlaps(v.sounding, (a.absolute_start, a.absolute_end))
            }) {
                let fixed = self
                    .channels
                    .channel_for_source(other.source_event_id)
                    .ok_or("external_channel_unbound")?;
                v.choices.retain(|channel| *channel != fixed);
            }
        }

        let mut domains = Domains {
            neighbors: vec![vec![]; variables.len()],
            ..Default::default()
        };
        for (right, b) in variables.iter().enumerate() {
            for (left, a) in variables[..right].iter().enumerate() {
                if overlaps(a.written, b.written) || overlaps(a.sounding, b.sounding) {
                    domains.neighbors[right].push((left, false));
                }
            }
        }
        let fragments: BTreeMap<_, _> = self
            .canonical
            .source_locators
            .iter()
            .map(|row| (row.fragment_id, row.source_event_id))
            .collect();
        let mut equal = self.retained.clone();
        for edge in &*self.canonical.hard_same_edges {
            equal.push((
                *fragments
                    .get(&edge.left_fragment_id)
                    .ok_or("hard_same_endpoint_unbound")?,
                *fragments
                    .get(&edge.right_fragment_id)
                    .ok_or("hard_same_endpoint_unbound")?,
            ));
        }
        for (a, b) in equal {
            match (indices.get(&a), indices.get(&b)) {
                (Some(&left), Some(&right)) if left != right => {
                    domains.neighbors[left.max(right)].push((left.min(right), true))
                }
                (Some(&inside), None) | (None, Some(&inside)) => {
                    let outside = if indices.contains_key(&a) { b } else { a };
                    let channel = self
                        .channels
                        .channel_for_source(outside)
                        .ok_or("fixed_endpoint_unbound")?;
                    variables[inside].choices.retain(|c| *c == channel);
                }
                _ => {}
            }
        }
        // Visit the incumbent channel first where available; this affects
        // witness order only. No incomplete traversal establishes forcedness.
        for v in &mut variables {
            if let Some(i) = v.choices.iter().position(|c| *c == v.current) {
                v.choices.swap(0, i);
            }
        }
        domains.choices = variables.iter().map(|v| v.choices.clone()).collect();
        Ok(Passage {
            variables,
            domains,
            measure: measure.measure,
            window,
        })
    }

    fn report(&self, out: &mut String, group: &[(&OwnerEdge<'_>, OwnerEdgeReadback)]) {
        let splits: Vec<_> = group
            .iter()
            .filter(|(_, state)| *state == OwnerEdgeReadback::SplitVoice)
            .map(|(edge, _)| (identity(edge.left), identity(edge.right)))
            .collect();
        let passage = match self.passage(group) {
            Ok(passage) => passage,
            Err(reason) => {
                for pair in splits {
                    let _ = writeln!(
                        out,
                        "ExactVoicePassageTarget: pair={pair:?}; result=open; reason={reason}; authority=none"
                    );
                }
                return;
            }
        };
        let vars = &passage.variables;
        let identities: Vec<_> = vars.iter().map(|v| v.id).collect();
        let current: Vec<_> = vars.iter().map(|v| v.current).collect();
        let mut targets = BTreeSet::new();
        for &(a, b) in &splits {
            let indices = identities
                .iter()
                .position(|id| *id == a)
                .zip(identities.iter().position(|id| *id == b));
            let Some(pair) = indices else {
                let _ = writeln!(
                    out,
                    "ExactVoicePassageTarget: pair={:?}; result=open; reason=target_not_bound_to_singleton; authority=none",
                    (a, b)
                );
                continue;
            };
            targets.insert(pair);
        }
        let mut queries = targets.clone();
        for (left, a) in vars.iter().enumerate() {
            for (right, b) in vars.iter().enumerate() {
                if a.written.1 == b.written.0 {
                    queries.insert((left, right));
                }
            }
        }
        let queries: Vec<_> = queries.into_iter().collect();
        let reference_in_model = passage.domains.accepts(&current);
        let emitted: Vec<_> = identities
            .iter()
            .map(|&(s, a, f)| (s, a, f, 0usize))
            .collect();
        let search = passage.domains.search(&queries, 8192);
        let _ = writeln!(
            out,
            "ExactVoicePassage: group={:?}; measure={}; window={:?}; source_attack_fragment_output_ordinal={emitted:?}; reference={current:?}; reference_in_model={reference_in_model}; plans_seen={}; complete={}; steps={}; authority=none",
            group[0].0.group,
            passage.measure,
            passage.window,
            search.plans,
            search.complete,
            search.steps
        );
        for ((left, right), mut w) in queries.into_iter().zip(search.witnesses) {
            // The selected plan is itself an independently admitted witness
            // when it also passes every constraint of this local comparison.
            if reference_in_model {
                if current[left] == current[right] {
                    if w.with.is_empty() {
                        w.with.push(current.clone());
                    }
                } else {
                    w.without = Some(current.clone());
                }
            }
            let pair = (identities[left], identities[right]);
            let is_target = targets.contains(&(left, right));
            let tag = if is_target { "Target" } else { "Relation" };
            let result = w.result(search.complete);
            let reason = if search.complete && search.plans == 0 {
                "empty_model_set"
            } else if !search.complete {
                "search_limit"
            } else {
                "exhaustive"
            };
            let _ = writeln!(
                out,
                "ExactVoicePassage{tag}: pair={pair:?}; relation=same_channel; result={result}; search={reason}; with={:?}; without={:?}; authority=none",
                w.with.first(),
                w.without
            );
            if !is_target || w.with.is_empty() {
                continue;
            }
            let mut checked = 0;
            let mut admitted = false;
            for plan in &w.with {
                let moved = vars
                    .iter()
                    .zip(plan)
                    .filter_map(|(v, &c)| (v.current != c).then_some((v.id.0, c)))
                    .collect();
                let admission = self.channels.check_exact_passage_witness(
                    self.canonical,
                    self.attacks,
                    self.original,
                    self.voices,
                    &moved,
                    (pair.0.2, pair.1.2),
                );
                checked += 1;
                let _ = writeln!(
                    out,
                    "ExactVoicePassageWitness: pair={pair:?}; plan={plan:?}; owner={admission:?}; authority=none"
                );
                if admission.is_ok() {
                    admitted = true;
                    break;
                }
            }
            let _ = writeln!(
                out,
                "ExactVoicePassageOwner: pair={pair:?}; admitted_with={admitted}; positive_plans_seen={}; owner_plans_checked={checked}; check_limit=8; authority=none",
                w.positive_seen
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_boundary_can_force_one_continuation_while_two_lines_exchange() {
        let mut domains = Domains {
            choices: vec![
                vec![0],
                vec![1],
                vec![2],
                vec![0, 1, 2],
                vec![1, 2],
                vec![1, 2],
            ],
            neighbors: vec![vec![]; 6],
        };
        // Three simultaneous entrances need distinct channels. Two of them
        // share exactly two boundary channels; they exhaust those channels.
        domains.neighbors[4].push((3, false));
        domains.neighbors[5].extend([(3, false), (4, false)]);
        let result = domains.search(&[(0, 3), (1, 4), (2, 5)], 100);
        assert!(result.complete);
        assert_eq!(result.plans, 2);
        assert_eq!(result.witnesses[0].result(true), "forced_in_model");
        for witnesses in &result.witnesses[1..] {
            assert_eq!(witnesses.result(true), "ambiguous_in_model");
            assert_ne!(witnesses.with.first(), witnesses.without.as_ref());
        }
        let partial = domains.search(&[(0, 3)], 7);
        assert!(!partial.complete);
        assert_eq!(partial.plans, 1);
        assert_eq!(partial.witnesses[0].result(partial.complete), "open");
        domains.choices[3].clear();
        let empty = domains.search(&[(0, 3)], 100);
        assert!(empty.complete);
        assert_eq!(empty.plans, 0);
        assert_eq!(empty.witnesses[0].result(true), "open");
    }
}
