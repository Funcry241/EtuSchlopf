//! Pure graph predicate. It observes candidate labels and supplies no authority.
use super::WrittenInterval;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct OccId {
    pub(super) source: usize, pub(super) attack: usize,
    pub(super) fragment: usize, pub(super) emission: usize,
}
impl std::fmt::Debug for OccId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("WrittenOccurrence(opaque)") }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Staff(pub(super) u8);
#[derive(Clone)]
pub(super) struct Occurrence {
    pub(super) id: OccId, pub(super) staff: Staff,
    pub(super) written: WrittenInterval, pub(super) chord: ChordBinding,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ChordBinding {
    Head,
    Member(OccId),
    Unbound,
}
impl Occurrence {
    fn proven_chord_head(&self) -> Option<OccId> {
        match self.chord {
            ChordBinding::Head => Some(self.id),
            ChordBinding::Member(head) => Some(head),
            ChordBinding::Unbound => None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum FactOrigin {
    Chord(OccId), HardSame(usize), SourceContinuation(usize, usize),
    ReceivedBeam { level: u8, group: (usize, usize) }, WrittenOverlap(OccId, OccId),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Reason { Chord, HardSame, SourceContinuation, ReceivedBeam, WrittenOverlap }
#[derive(Clone)]
pub(super) struct HardLink { pub(super) left: OccId, pub(super) right: OccId, pub(super) origin: FactOrigin }
#[derive(Clone)]
pub(super) struct Model {
    pub(super) occurrences: Vec<Occurrence>, pub(super) links: Vec<HardLink>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Fact { pub(super) left: OccId, pub(super) right: OccId, pub(super) reason: Reason, pub(super) origin: FactOrigin }
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Contradiction { pub(super) equality_path: Vec<Fact>, pub(super) separation: Fact }
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Analysis {
    pub(super) classes: Vec<Vec<OccId>>, pub(super) equalities: Vec<Fact>, pub(super) separations: Vec<Fact>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Violation { HardLinkSplit(Box<Fact>), SeparationMerged(Box<Fact>) }
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    Unknown(&'static str), Contradiction(Box<Contradiction>), Violation(Violation),
    GraphSatisfied { classes: Vec<Vec<OccId>>, pending_proofs: &'static [&'static str] },
}
pub(super) const PENDING_PROOFS: &[&str] = &[
    "original_notation_voice_separations", "musical_silence_obligations",
    "ordered_beam_automaton", "optional_connection_lifecycle", "grace_occurrences",
    "actual_materialization_readback", "whole_part_consumer_admission", "final_visible_preservation",
];
fn facts(model: &Model) -> Result<(Vec<Fact>, Vec<Fact>), &'static str> {
    let rows: BTreeMap<_, _> = model.occurrences.iter().map(|row| (&row.id, row)).collect();
    if rows.len() != model.occurrences.len() { return Err("duplicate written occurrence identity"); }
    let mut equalities = Vec::new();
    for row in &model.occurrences {
        if !matches!(row.staff.0, 1 | 2) || row.written.start >= row.written.end { return Err("unsupported written support/domain"); }
        if let ChordBinding::Member(head_id) = &row.chord {
            let Some(head) = rows.get(head_id) else { return Err("absent positively bound chord head"); };
            if head.chord != ChordBinding::Head
                || head.written.measure != row.written.measure || head.written.start != row.written.start
                || row.written.end > head.written.end { return Err("unsupported chord support"); }
            equalities.push(Fact { left: *head_id, right: row.id, reason: Reason::Chord, origin: FactOrigin::Chord(row.id) });
        }
    }
    for link in &model.links {
        let (Some(left), Some(right)) = (rows.get(&link.left), rows.get(&link.right)) else { return Err("absent link endpoint"); };
        let reason = match link.origin {
            FactOrigin::HardSame(_) => Reason::HardSame,
            FactOrigin::SourceContinuation(_, _) => Reason::SourceContinuation,
            FactOrigin::ReceivedBeam { .. } if left.staff == right.staff => Reason::ReceivedBeam,
            FactOrigin::ReceivedBeam { .. } => return Err("cross-staff received beam requires ordered lane evidence"),
            _ => return Err("unsupported link origin"),
        };
        equalities.push(Fact { left: link.left, right: link.right, reason, origin: link.origin.clone() });
    }
    let mut separations = Vec::new();
    for (index, left) in model.occurrences.iter().enumerate() {
        for right in &model.occurrences[index + 1..] {
            let (Some(a), Some(b)) = (left.proven_chord_head(), right.proven_chord_head()) else {
                continue;
            };
            if left.staff == right.staff && a != b && left.written.overlaps(right.written) {
                separations.push(Fact { left: left.id, right: right.id, reason: Reason::WrittenOverlap,
                    origin: FactOrigin::WrittenOverlap(left.id, right.id) });
            }
        }
    }
    Ok((equalities, separations))
}
fn root(parent: &mut [usize], mut id: usize) -> usize {
    while parent[id] != id { parent[id] = parent[parent[id]]; id = parent[id]; }
    id
}

pub(super) fn analyze(model: &Model) -> Result<Analysis, Outcome> {
    let (equalities, separations) = facts(model).map_err(Outcome::Unknown)?;
    let indices: BTreeMap<_, _> = model.occurrences.iter().enumerate().map(|(i, row)| (&row.id, i)).collect();
    let mut parent: Vec<_> = (0..model.occurrences.len()).collect();
    let mut graph = vec![Vec::new(); parent.len()];
    for (edge, fact) in equalities.iter().enumerate() {
        let (left, right) = (indices[&fact.left], indices[&fact.right]);
        let (a, b) = (root(&mut parent, left), root(&mut parent, right));
        parent[b] = a;
        graph[left].push((right, edge)); graph[right].push((left, edge));
    }
    for separation in &separations {
        let (left, right) = (indices[&separation.left], indices[&separation.right]);
        if root(&mut parent, left) != root(&mut parent, right) { continue; }
        let mut queue = VecDeque::from([left]);
        let mut seen = vec![false; parent.len()]; seen[left] = true;
        let mut previous = vec![None; parent.len()];
        while let Some(current) = queue.pop_front() {
            if current == right { break; }
            for &(next, edge) in &graph[current] {
                if !seen[next] { seen[next] = true; previous[next] = Some((current, edge)); queue.push_back(next); }
            }
        }
        let mut path = Vec::new(); let mut current = right;
        while let Some((old, edge)) = previous[current] { path.push(equalities[edge].clone()); current = old; }
        path.reverse();
        return Err(Outcome::Contradiction(Box::new(Contradiction { equality_path: path, separation: separation.clone() })));
    }
    let mut positions = BTreeMap::new(); let mut classes: Vec<Vec<OccId>> = Vec::new();
    for (index, row) in model.occurrences.iter().enumerate() {
        let representative = root(&mut parent, index);
        let next_position = classes.len();
        let position = *positions.entry(representative).or_insert(next_position);
        if position == classes.len() { classes.push(Vec::new()); }
        classes[position].push(row.id);
    }
    Ok(Analysis { classes, equalities, separations })
}

pub(super) fn verify_contradiction(model: &Model, proof: &Contradiction) -> bool {
    let Ok((equalities, separations)) = facts(model) else { return false; };
    if !separations.contains(&proof.separation) { return false; }
    let mut current = &proof.separation.left;
    let mut visited = BTreeSet::from([current]);
    for edge in &proof.equality_path {
        if !equalities.contains(edge) { return false; }
        current = if current == &edge.left { &edge.right } else if current == &edge.right { &edge.left } else { return false; };
        if !visited.insert(current) { return false; }
    }
    current == &proof.separation.right
}

pub(super) fn check(model: &Model, assignment: &BTreeMap<OccId, String>) -> Outcome {
    let analysis = match analyze(model) { Ok(value) => value, Err(outcome) => return outcome };
    if assignment.len() != model.occurrences.len() || model.occurrences.iter().any(|row|
        assignment.get(&row.id).is_none_or(String::is_empty)) { return Outcome::Unknown("incomplete or foreign assignment"); }
    for edge in &analysis.equalities {
        if assignment[&edge.left] != assignment[&edge.right] { return Outcome::Violation(Violation::HardLinkSplit(Box::new(edge.clone()))); }
    }
    for edge in &analysis.separations {
        if assignment[&edge.left] == assignment[&edge.right] { return Outcome::Violation(Violation::SeparationMerged(Box::new(edge.clone()))); }
    }
    if model.occurrences.iter().any(|row| row.chord == ChordBinding::Unbound) {
        return Outcome::Unknown("unsupported_unbound_projected_chord_host");
    }
    Outcome::GraphSatisfied { classes: analysis.classes, pending_proofs: PENDING_PROOFS }
}

#[cfg(test)]
#[path = "../../tests/unit/written_assignment_graph.rs"]
mod tests;
