//! Same-owner readback of the actual admitted label assignment. No allocation.
use super::*;
use super::written_assignment_graph::{self as graph, ChordBinding, FactOrigin, HardLink, Model, OccId, Occurrence, Outcome, Staff};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

fn identity(row: &FinalChordWrittenOccurrence) -> OccId {
    OccId { source: row.source_event_id, attack: row.attack_id,
        fragment: row.fragment_id, emission: row.emission_ordinal }
}

fn model(out: &mut String, canonical: &CanonicalFinalChordStructuralPlan, production: &FrozenSerializedVoicePlan)
    -> Result<(Model, BTreeMap<OccId, String>, usize), &'static str> {
    let written = canonical.written_projection.as_ref().ok_or("missing_written_projection")?;
    let document = roxmltree::Document::parse(&written.original_xml).map_err(|_| "unreadable_source_document")?;
    if document.root_element().children().filter(|n| n.has_tag_name("part")).count() != 1 {
        return Err("source_has_no_single_bound_part_domain");
    }
    let locators = canonical.source_locators.iter().map(|row| (row.fragment_id, row)).collect::<BTreeMap<_, _>>();
    if locators.len() != canonical.source_locators.len() { return Err("ambiguous_canonical_fragment_identity"); }
    let mut represented = BTreeSet::new();
    let mut identities = BTreeSet::new();
    let mut by_start = BTreeMap::new();
    let mut by_fragment = BTreeMap::<usize, Vec<OccId>>::new();
    let mut by_attack = BTreeMap::<(usize, usize), Vec<OccId>>::new();
    let mut assignment = BTreeMap::new();
    let mut occurrences = Vec::new(); let mut graces = 0;
    for row in written.occurrences.iter() {
        let locator = locators.get(&row.fragment_id).ok_or("foreign_written_fragment")?;
        if locator.source_event_id != row.source_event_id || locator.attack_id != row.attack_id {
            return Err("written_source_attack_binding_changed");
        }
        represented.insert(row.fragment_id);
        if row.is_grace { graces += 1; continue; }
        let id = identity(row);
        if !identities.insert(id) || by_start.insert(row.projected_note_start, id).is_some() {
            return Err("duplicate_actual_producer_emission");
        }
        by_fragment.entry(row.fragment_id).or_default().push(id);
        by_attack.entry((row.source_event_id, row.attack_id)).or_default().push(id);
        assignment.insert(id, production.label_for_source(row.source_event_id)
            .ok_or("missing_actual_production_label")?.to_owned());
        occurrences.push(Occurrence { id, staff: Staff(row.staff), written: row.written_interval(), chord: ChordBinding::Head });
    }
    if represented != locators.keys().copied().collect() { return Err("incomplete_written_fragment_coverage"); }
    let written_by_id = written.occurrences.iter().filter(|row| !row.is_grace)
        .map(|row| (identity(row), row)).collect::<BTreeMap<_, _>>();
    for (target, row) in occurrences.iter_mut().zip(written.occurrences.iter().filter(|row| !row.is_grace)) {
        if row.has_pitched_chord_anchor && row.chord_anchor_start == row.projected_note_start {
            continue;
        }
        let head = by_start.get(&row.chord_anchor_start)
            .and_then(|id| written_by_id.get(id)).copied();
        let reason = match head {
            Some(head) if head.chord_anchor_start == head.projected_note_start
                && row.shares_projected_chord(head)
                && (head.staff == row.staff || canonical.hard_same_edges.iter().any(|edge|
                    edge.left_fragment_id == row.fragment_id
                        && edge.right_fragment_id == head.fragment_id
                        && edge.presentation_staff == head.staff)) => {
                    target.chord = ChordBinding::Member(identity(head));
                    continue;
                }
            _ if !row.has_pitched_chord_anchor => "no_positive_pitched_head",
            None => "missing_projected_head_emission",
            Some(head) if head.staff != row.staff => "no_canonical_cross_staff_chord_binding",
            Some(head) if head.measure != row.measure || head.onset != row.onset => "different_written_onset",
            Some(head) if row.end > head.end => "member_outlasts_head",
            Some(_) => "unsupported_head_binding",
        };
        target.chord = ChordBinding::Unbound;
        let head_id = head.map(|head| (head.source_event_id, head.attack_id, head.fragment_id, head.emission_ordinal));
        let _ = writeln!(out, "WrittenAssignmentChordEvidence: reason={reason}; source={}; attack={}; fragment={}; emission={}; staff={}; written={}..{}; head={head_id:?}; physical_anchor={}..{}; authority=none",
            row.source_event_id, row.attack_id, row.fragment_id, row.emission_ordinal, row.staff,
            row.onset, row.end, row.chord_anchor_onset, row.chord_anchor_end);
    }
    let mut links = Vec::new();
    for (&(source, attack), members) in &by_attack {
        for pair in members.windows(2) {
            links.push(HardLink { left: pair[0], right: pair[1], origin: FactOrigin::SourceContinuation(source, attack) });
        }
    }
    for (edge, relation) in canonical.hard_same_edges.iter().enumerate() {
        let (Some(left), Some(right)) = (by_fragment.get(&relation.left_fragment_id), by_fragment.get(&relation.right_fragment_id)) else {
            return Err("hard_same_has_unsupported_or_missing_written_endpoint");
        };
        links.push(HardLink { left: left[0], right: right[0], origin: FactOrigin::HardSame(edge) });
    }
    Ok((Model { occurrences, links }, assignment, graces))
}

fn append_outcome(out: &mut String, scope: &str, model: &Model, assignment: &BTreeMap<OccId, String>) {
    match graph::check(model, assignment) {
        Outcome::GraphSatisfied { classes, pending_proofs } => {
            let _ = writeln!(out, "WrittenAssignmentGraphResult: graph={scope}; result=GraphSatisfied; classes={}; pending={pending_proofs:?}; admission=none", classes.len());
        },
        Outcome::Contradiction(proof) => {
            let verified = graph::verify_contradiction(model, &proof);
            let _ = writeln!(out, "WrittenAssignmentGraphResult: graph={scope}; result=Contradiction; certificate_verified={verified}; equality_path={:?}; separation={:?}; admission=none",
                proof.equality_path, proof.separation);
            let ids = proof.equality_path.iter().chain(std::iter::once(&proof.separation))
                .flat_map(|edge| [edge.left, edge.right]).collect();
            append_witnesses(out, scope, model, &ids);
        },
        Outcome::Violation(violation) => {
            let _ = writeln!(out, "WrittenAssignmentGraphResult: graph={scope}; result=AssignmentViolation; violation={violation:?}; admission=none");
            let ids = match violation {
                graph::Violation::HardLinkSplit(edge) | graph::Violation::SeparationMerged(edge) =>
                    [edge.left, edge.right].into_iter().collect(),
            };
            append_witnesses(out, scope, model, &ids);
        },
        Outcome::Unknown(reason) => {
            let _ = writeln!(out, "WrittenAssignmentGraphResult: graph={scope}; result=Unknown; reason={reason}; admission=none");
        },
    }
}

fn append_witnesses(out: &mut String, scope: &str, model: &Model, ids: &BTreeSet<OccId>) {
    for row in model.occurrences.iter().filter(|row| ids.contains(&row.id)) {
        let _ = writeln!(out, "WrittenAssignmentGraphWitness: graph={scope}; measure={} staff={} written={}..{} source={} attack={} fragment={} emission={}; authority=none",
            row.written.measure, row.staff.0, row.written.start, row.written.end,
            row.id.source, row.id.attack, row.id.fragment, row.id.emission);
    }
}

pub(super) fn append(out: &mut String, channels: &FrozenSerializationVoiceChannelPlan,
    canonical: &CanonicalFinalChordStructuralPlan, received: &ActualSourceBeamBindingObservation,
    production: &FrozenSerializedVoicePlan) {
    if !channels.authority_bound || !canonical.is_bound() || !production.is_bound()
        || channels.source_to_channel != production.source_to_channel
        || channels.source_to_role != production.source_to_role
        || channels.channel_to_role != production.channel_to_role {
        let _ = writeln!(out, "WrittenAssignmentGraph: Unknown; reason=foreign_or_unbound_actual_assignment; admission=none");
        return;
    }
    match channels.require_consumer_admission(canonical, production) {
        Ok(()) => {
            let _ = writeln!(out, "WrittenAssignmentConsumerCapacity: result=Admitted; source=bound_consumer; authority=none");
        }
        Err(error) => {
            let _ = writeln!(out, "WrittenAssignmentConsumerCapacity: result=NotAdmitted; source=bound_consumer; reason={error}; authority=none");
        }
    }
    super::ordered_received_beam_readback::append(out, canonical, received);
    let (mut base, assignment, graces) = match model(out, canonical, production) {
        Ok(value) => value,
        Err(reason) => {
            let _ = writeln!(out, "WrittenAssignmentGraph: Unknown; reason={reason}; admission=none");
            return;
        },
    };
    let unbound = base.occurrences.iter().filter(|row| row.chord == ChordBinding::Unbound)
        .map(|row| row.id).collect::<BTreeSet<_>>();
    if !unbound.is_empty() {
        let _ = writeln!(out, "WrittenAssignmentGraphPendingChordBinding: occurrences={}; admission=none", unbound.len());
        append_witnesses(out, "pending_chord_binding", &base, &unbound);
    }
    let _ = writeln!(out, "WrittenAssignmentGraph: bound; scope=positive_written_pitch; occurrences={}; hard_links={}; grace_occurrences_not_checked={graces}; assignment=actual_serialized_labels; model_complete=false; musical_line_authority=none; admission=none", base.occurrences.len(), base.links.len());
    append_outcome(out, "hard_received_bindings", &base, &assignment);
    let rows = match received.graph_received_occurrences(canonical) {
        Ok(rows) => rows,
        Err(reason) => {
            let _ = writeln!(out, "WrittenAssignmentGraphResult: graph=conditional_received_beam_retention; result=Unknown; reason={reason}; admission=none");
            return;
        },
    };
    let mut by_fragment = BTreeMap::<usize, Vec<OccId>>::new();
    for row in &base.occurrences { by_fragment.entry(row.id.fragment).or_default().push(row.id); }
    let mut groups = BTreeMap::<(u8, (usize, usize)), Vec<OccId>>::new();
    for row in rows {
        let Some(members) = by_fragment.get(&row.fragment) else {
            let _ = writeln!(out, "WrittenAssignmentGraphResult: graph=conditional_received_beam_retention; result=Unknown; reason=beam_member_outside_positive_written_scope; admission=none");
            return;
        };
        if members.iter().any(|id| id.source != row.source || id.attack != row.attack) {
            let _ = writeln!(out, "WrittenAssignmentGraphResult: graph=conditional_received_beam_retention; result=Unknown; reason=original_beam_identity_changed; admission=none");
            return;
        }
        for (level, group, _) in &row.levels {
            if let Some(group) = group { groups.entry((*level, *group)).or_default().extend(members); }
        }
    }
    let before = base.links.len();
    for (&(level, group), members) in &groups {
        for pair in members.windows(2) {
            base.links.push(HardLink { left: pair[0], right: pair[1], origin: FactOrigin::ReceivedBeam { level, group } });
        }
    }
    let _ = writeln!(out, "WrittenAssignmentGraphRetentionPremises: groups={}; links={}; retention=conditional; optional_lifecycle=pending; ordered_markers=pending; admission=none", groups.len(), base.links.len() - before);
    append_outcome(out, "conditional_received_beam_retention", &base, &assignment);
}
