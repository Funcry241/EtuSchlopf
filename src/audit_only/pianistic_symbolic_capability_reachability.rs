//! r623 read-only direct-boundary structural clique obstruction closure on the r622 symbolic formula.
//!
//! This audit closes only the score-derived side of the r295/r296/r619 execution ontology. It
//! never invents a pianist and never promotes UNMODELED capability to SUPPORTED. Every
//! overlap-connected same-MIDI physical obligation component receives one existential variable
//! over the exact ten v2 hand/finger slots. Overlapping different-key components receive explicit
//! slot-inequality constraints. Boundary evaluators then derive the exact r295 static signature,
//! r296 transition signature plus available score time, and r619 latest-prior-same-slot
//! rearticulation signature from any candidate slot valuation.
//!
//! The result is a quantified executable formula, not a chosen fingering. A later external
//! capability valuation satisfies the score iff at least one structural slot valuation exists for
//! which every valuation-derived capability atom is supported. No slot valuation, ranking or
//! capability answer is stored as productive truth. Finger substitution during a hold, hand
//! takeover during a hold and multi-key-per-finger techniques remain outside execution-v2.

use crate::audit_only::pianistic_configuration_constraint_kernel::{
    configuration_signature, Finger, FingerKeyAssignment, Hand, KeySurface, KeyboardGeometry,
    StaticConfigurationRequest, StaticConfigurationSignature,
};
use crate::audit_only::pianistic_execution_plan_witness::{
    build_boundaries, build_demand_topology, transition_signature, BoundarySlice,
    PhysicalKeyObligation, SameKeyTechniqueSignature, TransitionSignature,
};
use crate::audit_only::pianistic_execution_score::{self, PianisticExecutionScore, ProjectionStatus};
use crate::sha256;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-SYMBOLIC-CAPABILITY-REACHABILITY-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticSymbolicCapabilityReachabilityAudit";
pub const SCHEMA: &str = "pianistic-symbolic-capability-reachability-v3";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

const STATIC_PREDICATE_SEMANTICS: &str =
    "nonempty Config(hand,boundary,slot_valuation)=>StaticCapability(r295_exact_signature)";
const TRANSITION_PREDICATE_SEMANTICS: &str =
    "r296_previous_nonempty_history(slot_valuation)=>TransitionCapability(r296_exact_signature,available_score_time)";
const SAME_KEY_PREDICATE_SEMANTICS: &str =
    "latest_prior_same_midi_attack_with_equal_slot(slot_valuation)=>SameKeyCapability(r619_exact_signature,attack_delta)";
const FORMULA_SEMANTICS: &str =
    "Exists(slot_variables): StructuralConstraints AND ValuationDerivedStaticAtoms AND ValuationDerivedTransitionAtoms AND ValuationDerivedSameKeyAtoms";

const SLOT_COUNT: usize = 10;
const HANDS: [Hand; 2] = [Hand::Left, Hand::Right];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SlotIdentity {
    hand: Hand,
    finger: Finger,
}

fn slot_identity(index: usize) -> Option<SlotIdentity> {
    let hand = if index < 5 { Hand::Left } else { Hand::Right };
    let finger = match index % 5 {
        0 => Finger::Thumb,
        1 => Finger::Index,
        2 => Finger::Middle,
        3 => Finger::Ring,
        4 => Finger::Little,
        _ => return None,
    };
    Some(SlotIdentity { hand, finger })
}

fn slot_name(index: usize) -> Option<&'static str> {
    Some(match index {
        0 => "Left/Thumb",
        1 => "Left/Index",
        2 => "Left/Middle",
        3 => "Left/Ring",
        4 => "Left/Little",
        5 => "Right/Thumb",
        6 => "Right/Index",
        7 => "Right/Middle",
        8 => "Right/Ring",
        9 => "Right/Little",
        _ => return None,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SlotVariable {
    id: usize,
    midi: i32,
    attack_tick: i64,
    release_tick: i64,
    obligation_ids: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryTerm {
    index: usize,
    tick: i64,
    time_us_floor: u64,
    active_variable_ids: Vec<usize>,
    attacking_obligation_ids: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StaticEvaluationSite {
    boundary_index: usize,
    hand: Hand,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TransitionEvaluationSite {
    boundary_index: usize,
    hand: Hand,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PriorSameKeyAttackTerm {
    obligation_id: usize,
    variable_id: usize,
    attack_time_us_floor: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SameKeyEvaluationSite {
    current_obligation_id: usize,
    current_variable_id: usize,
    midi: i32,
    current_attack_time_us_floor: u64,
    surface: KeySurface,
    prior_attacks_in_time_order: Vec<PriorSameKeyAttackTerm>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DirectBoundaryCliqueProof {
    maximum_distinct_midi_clique: usize,
    witness_variable_ids: Vec<usize>,
    left_boundary_index: Option<usize>,
    right_boundary_index: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolicFormula {
    score_fingerprint_sha256: String,
    source_execution_events: usize,
    physical_key_obligations: usize,
    variables: Vec<SlotVariable>,
    obligation_to_variable: Vec<usize>,
    inequality_constraints: Vec<(usize, usize)>,
    boundary_terms: Vec<BoundaryTerm>,
    static_evaluation_sites: Vec<StaticEvaluationSite>,
    transition_evaluation_sites: Vec<TransitionEvaluationSite>,
    same_key_evaluation_sites: Vec<SameKeyEvaluationSite>,
    maximum_concurrent_variables: usize,
    structural_graph_degeneracy: usize,
    structural_witness_found: bool,
    concurrent_clique_obstruction: bool,
    direct_boundary_clique_proof: DirectBoundaryCliqueProof,
    direct_boundary_clique_obstruction: bool,
    formula_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolicReplay {
    verified: bool,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolicAudit {
    status: &'static str,
    reason: String,
    source_execution_events: usize,
    physical_key_obligations: usize,
    slot_variables: usize,
    same_key_overlap_collapses: usize,
    inequality_constraints: usize,
    boundary_count: usize,
    static_evaluation_sites: usize,
    transition_evaluation_sites: usize,
    same_key_evaluation_sites: usize,
    same_key_prior_attack_terms: usize,
    maximum_concurrent_variables: usize,
    structural_graph_degeneracy: usize,
    direct_boundary_clique_maximum: usize,
    direct_boundary_clique_obstruction: bool,
    structural_execution_space: &'static str,
    formula_sha256: String,
    replay: SymbolicReplay,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ConcreteCapabilityAtom {
    Static {
        boundary_index: usize,
        signature: StaticConfigurationSignature,
    },
    Transition {
        boundary_index: usize,
        signature: TransitionSignature,
        available_time_us: u64,
    },
    SameKey {
        current_obligation_id: usize,
        signature: SameKeyTechniqueSignature,
        available_time_us: u64,
    },
}

#[derive(Debug, Clone)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DisjointSet {
    fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
            rank: vec![0; len],
        }
    }

    fn find(&mut self, value: usize) -> usize {
        let parent = self.parent[value];
        if parent != value {
            let root = self.find(parent);
            self.parent[value] = root;
        }
        self.parent[value]
    }

    fn union(&mut self, left: usize, right: usize) {
        let mut left_root = self.find(left);
        let mut right_root = self.find(right);
        if left_root == right_root {
            return;
        }
        if self.rank[left_root] < self.rank[right_root] {
            std::mem::swap(&mut left_root, &mut right_root);
        }
        self.parent[right_root] = left_root;
        if self.rank[left_root] == self.rank[right_root] {
            self.rank[left_root] = self.rank[left_root].saturating_add(1);
        }
    }
}

fn intervals_overlap(
    left_attack: i64,
    left_release: i64,
    right_attack: i64,
    right_release: i64,
) -> bool {
    left_attack < right_release && right_attack < left_release
}

fn build_slot_variables(
    obligations: &[PhysicalKeyObligation],
) -> Result<(Vec<SlotVariable>, Vec<usize>), String> {
    let mut sets = DisjointSet::new(obligations.len());
    for (left_index, left) in obligations.iter().enumerate() {
        for (right_index, right) in obligations.iter().enumerate().skip(left_index + 1) {
            if left.midi == right.midi
                && intervals_overlap(
                    left.attack_tick,
                    left.release_tick,
                    right.attack_tick,
                    right.release_tick,
                )
            {
                sets.union(left_index, right_index);
            }
        }
    }

    let mut grouped = BTreeMap::<usize, Vec<usize>>::new();
    for index in 0..obligations.len() {
        let root = sets.find(index);
        grouped.entry(root).or_default().push(index);
    }

    let mut variables = Vec::with_capacity(grouped.len());
    for ids in grouped.into_values() {
        let first = obligations
            .get(ids[0])
            .ok_or_else(|| "symbolic-component-first-obligation-missing".to_string())?;
        let midi = first.midi;
        if ids
            .iter()
            .any(|id| obligations.get(*id).is_none_or(|item| item.midi != midi))
        {
            return Err("symbolic-component-mixed-midi".to_string());
        }
        let attack_tick = ids
            .iter()
            .filter_map(|id| obligations.get(*id).map(|item| item.attack_tick))
            .min()
            .ok_or_else(|| "symbolic-component-attack-missing".to_string())?;
        let release_tick = ids
            .iter()
            .filter_map(|id| obligations.get(*id).map(|item| item.release_tick))
            .max()
            .ok_or_else(|| "symbolic-component-release-missing".to_string())?;
        variables.push(SlotVariable {
            id: 0,
            midi,
            attack_tick,
            release_tick,
            obligation_ids: ids,
        });
    }
    variables.sort_by_key(|item| {
        (
            item.attack_tick,
            item.midi,
            item.release_tick,
            item.obligation_ids.first().copied().unwrap_or(usize::MAX),
        )
    });

    let mut obligation_to_variable = vec![usize::MAX; obligations.len()];
    for (id, variable) in variables.iter_mut().enumerate() {
        variable.id = id;
        for obligation_id in &variable.obligation_ids {
            let Some(cell) = obligation_to_variable.get_mut(*obligation_id) else {
                return Err("symbolic-obligation-to-variable-out-of-range".to_string());
            };
            if *cell != usize::MAX {
                return Err("symbolic-obligation-covered-more-than-once".to_string());
            }
            *cell = id;
        }
    }
    if obligation_to_variable.contains(&usize::MAX) {
        return Err("symbolic-obligation-not-covered".to_string());
    }
    Ok((variables, obligation_to_variable))
}

fn build_inequality_constraints(
    variables: &[SlotVariable],
    boundary_terms: &[BoundaryTerm],
) -> Vec<(usize, usize)> {
    let mut out = BTreeSet::new();
    for (left_index, left) in variables.iter().enumerate() {
        for right in variables.iter().skip(left_index + 1) {
            if left.midi != right.midi
                && intervals_overlap(
                    left.attack_tick,
                    left.release_tick,
                    right.attack_tick,
                    right.release_tick,
                )
            {
                out.insert((left.id.min(right.id), left.id.max(right.id)));
            }
        }
    }

    for window in boundary_terms.windows(2) {
        let previous = &window[0];
        let current = &window[1];
        for previous_id in &previous.active_variable_ids {
            for current_id in &current.active_variable_ids {
                if previous_id == current_id {
                    continue;
                }
                let Some(previous_variable) = variables.get(*previous_id) else {
                    continue;
                };
                let Some(current_variable) = variables.get(*current_id) else {
                    continue;
                };
                if previous_variable.midi != current_variable.midi {
                    out.insert((
                        (*previous_id).min(*current_id),
                        (*previous_id).max(*current_id),
                    ));
                }
            }
        }
    }
    out.into_iter().collect()
}

fn build_boundary_terms(
    boundaries: &[BoundarySlice],
    obligation_to_variable: &[usize],
) -> Result<Vec<BoundaryTerm>, String> {
    boundaries
        .iter()
        .enumerate()
        .map(|(index, boundary)| {
            let mut active_variable_ids = boundary
                .active
                .iter()
                .map(|obligation_id| {
                    obligation_to_variable
                        .get(*obligation_id)
                        .copied()
                        .ok_or_else(|| "boundary-active-variable-missing".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?;
            active_variable_ids.sort_unstable();
            active_variable_ids.dedup();
            Ok(BoundaryTerm {
                index,
                tick: boundary.tick,
                time_us_floor: boundary.time_us_floor,
                active_variable_ids,
                attacking_obligation_ids: boundary.attacking.clone(),
            })
        })
        .collect()
}

fn build_static_evaluation_sites(boundary_terms: &[BoundaryTerm]) -> Vec<StaticEvaluationSite> {
    boundary_terms
        .iter()
        .flat_map(|boundary| {
            HANDS.map(|hand| StaticEvaluationSite {
                boundary_index: boundary.index,
                hand,
            })
        })
        .collect()
}

fn build_transition_evaluation_sites(
    boundary_terms: &[BoundaryTerm],
) -> Vec<TransitionEvaluationSite> {
    boundary_terms
        .iter()
        .flat_map(|boundary| {
            HANDS.map(|hand| TransitionEvaluationSite {
                boundary_index: boundary.index,
                hand,
            })
        })
        .collect()
}

fn build_same_key_evaluation_sites(
    obligations: &[PhysicalKeyObligation],
    obligation_to_variable: &[usize],
) -> Result<Vec<SameKeyEvaluationSite>, String> {
    let geometry = KeyboardGeometry::standard_88_key();
    let mut by_midi = BTreeMap::<i32, Vec<&PhysicalKeyObligation>>::new();
    for obligation in obligations {
        by_midi.entry(obligation.midi).or_default().push(obligation);
    }

    let mut sites = Vec::new();
    for sequence in by_midi.values_mut() {
        sequence.sort_by_key(|item| (item.attack_tick, item.id));
        for (current_position, current) in sequence.iter().enumerate() {
            let mut prior_attacks_in_time_order = Vec::new();
            for prior in sequence.iter().take(current_position) {
                if prior.attack_tick >= current.attack_tick {
                    continue;
                }
                prior_attacks_in_time_order.push(PriorSameKeyAttackTerm {
                    obligation_id: prior.id,
                    variable_id: *obligation_to_variable
                        .get(prior.id)
                        .ok_or_else(|| "same-key-prior-variable-missing".to_string())?,
                    attack_time_us_floor: prior.attack_time_us_floor,
                });
            }
            if prior_attacks_in_time_order.is_empty() {
                continue;
            }
            let surface = geometry
                .key(current.midi)
                .ok_or_else(|| "same-key-surface-unavailable".to_string())?
                .surface;
            sites.push(SameKeyEvaluationSite {
                current_obligation_id: current.id,
                current_variable_id: *obligation_to_variable
                    .get(current.id)
                    .ok_or_else(|| "same-key-current-variable-missing".to_string())?,
                midi: current.midi,
                current_attack_time_us_floor: current.attack_time_us_floor,
                surface,
                prior_attacks_in_time_order,
            });
        }
    }
    sites.sort();
    Ok(sites)
}

fn maximum_concurrent_variables(variables: &[SlotVariable]) -> usize {
    let mut ticks = variables
        .iter()
        .flat_map(|item| [item.attack_tick, item.release_tick])
        .collect::<Vec<_>>();
    ticks.sort_unstable();
    ticks.dedup();
    ticks
        .iter()
        .map(|tick| {
            variables
                .iter()
                .filter(|item| item.attack_tick <= *tick && *tick < item.release_tick)
                .count()
        })
        .max()
        .unwrap_or(0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuralColoringProof {
    coloring: Option<Vec<usize>>,
    degeneracy: usize,
}

fn structural_coloring_witness(
    variables: &[SlotVariable],
    inequalities: &[(usize, usize)],
) -> Result<StructuralColoringProof, String> {
    let mut adjacency = vec![BTreeSet::<usize>::new(); variables.len()];
    for (left, right) in inequalities {
        if left == right {
            return Err("structural-coloring-self-inequality".to_string());
        }
        let Some(left_neighbors) = adjacency.get_mut(*left) else {
            return Err("structural-coloring-left-variable-out-of-range".to_string());
        };
        left_neighbors.insert(*right);
        let Some(right_neighbors) = adjacency.get_mut(*right) else {
            return Err("structural-coloring-right-variable-out-of-range".to_string());
        };
        right_neighbors.insert(*left);
    }

    let mut degrees = adjacency.iter().map(BTreeSet::len).collect::<Vec<_>>();
    let mut removed = vec![false; variables.len()];
    let mut smallest_last_order = Vec::with_capacity(variables.len());
    let mut degeneracy = 0usize;

    for _ in 0..variables.len() {
        let Some(variable_id) = variables
            .iter()
            .map(|item| item.id)
            .filter(|id| removed.get(*id).is_some_and(|value| !value))
            .min_by_key(|id| {
                let item = variables.get(*id);
                (
                    degrees.get(*id).copied().unwrap_or(usize::MAX),
                    item.map(|value| value.attack_tick).unwrap_or(i64::MAX),
                    item.map(|value| value.release_tick).unwrap_or(i64::MAX),
                    *id,
                )
            })
        else {
            return Err("structural-coloring-smallest-last-selection-failed".to_string());
        };
        let degree = *degrees
            .get(variable_id)
            .ok_or_else(|| "structural-coloring-degree-out-of-range".to_string())?;
        degeneracy = degeneracy.max(degree);
        *removed
            .get_mut(variable_id)
            .ok_or_else(|| "structural-coloring-removal-out-of-range".to_string())? = true;
        smallest_last_order.push(variable_id);
        for neighbor in adjacency
            .get(variable_id)
            .ok_or_else(|| "structural-coloring-adjacency-out-of-range".to_string())?
        {
            if removed.get(*neighbor).is_some_and(|value| !value) {
                let neighbor_degree = degrees
                    .get_mut(*neighbor)
                    .ok_or_else(|| "structural-coloring-neighbor-degree-out-of-range".to_string())?;
                *neighbor_degree = neighbor_degree.saturating_sub(1);
            }
        }
    }

    let mut colors = vec![usize::MAX; variables.len()];
    for variable_id in smallest_last_order.into_iter().rev() {
        let mut used = [false; SLOT_COUNT];
        for neighbor in adjacency
            .get(variable_id)
            .ok_or_else(|| "structural-coloring-adjacency-out-of-range".to_string())?
        {
            let color = *colors
                .get(*neighbor)
                .ok_or_else(|| "structural-coloring-neighbor-out-of-range".to_string())?;
            if color != usize::MAX {
                let Some(occupied) = used.get_mut(color) else {
                    return Err("structural-coloring-neighbor-color-out-of-range".to_string());
                };
                *occupied = true;
            }
        }
        let Some(color) = used.iter().position(|occupied| !occupied) else {
            return Ok(StructuralColoringProof {
                coloring: None,
                degeneracy,
            });
        };
        *colors
            .get_mut(variable_id)
            .ok_or_else(|| "structural-coloring-variable-out-of-range".to_string())? = color;
    }

    if !coloring_satisfies_constraints(&colors, inequalities) {
        return Err("structural-coloring-constructed-witness-invalid".to_string());
    }
    Ok(StructuralColoringProof {
        coloring: Some(colors),
        degeneracy,
    })
}

fn coloring_satisfies_constraints(
    coloring: &[usize],
    inequalities: &[(usize, usize)],
) -> bool {
    inequalities.iter().all(|(left, right)| {
        coloring
            .get(*left)
            .zip(coloring.get(*right))
            .is_some_and(|(left_color, right_color)| {
                *left_color < SLOT_COUNT
                    && *right_color < SLOT_COUNT
                    && left_color != right_color
            })
    })
}

fn direct_boundary_clique_proof(
    variables: &[SlotVariable],
    boundary_terms: &[BoundaryTerm],
    inequalities: &[(usize, usize)],
) -> Result<DirectBoundaryCliqueProof, String> {
    let inequality_set = inequalities.iter().copied().collect::<BTreeSet<_>>();
    let mut best = DirectBoundaryCliqueProof {
        maximum_distinct_midi_clique: 0,
        witness_variable_ids: Vec::new(),
        left_boundary_index: None,
        right_boundary_index: None,
    };

    for window in boundary_terms.windows(2) {
        let left = &window[0];
        let right = &window[1];
        let mut by_midi = BTreeMap::<i32, usize>::new();
        for variable_id in left
            .active_variable_ids
            .iter()
            .chain(right.active_variable_ids.iter())
        {
            let variable = variables
                .get(*variable_id)
                .ok_or_else(|| "direct-boundary-clique-variable-out-of-range".to_string())?;
            by_midi
                .entry(variable.midi)
                .and_modify(|existing| *existing = (*existing).min(*variable_id))
                .or_insert(*variable_id);
        }
        let candidate = by_midi.into_values().collect::<Vec<_>>();
        for (position, left_id) in candidate.iter().enumerate() {
            for right_id in candidate.iter().skip(position + 1) {
                let edge = ((*left_id).min(*right_id), (*left_id).max(*right_id));
                if !inequality_set.contains(&edge) {
                    return Err(format!(
                        "direct-boundary-clique-missing-inequality:{}:{}:{}:{}",
                        left.index, right.index, edge.0, edge.1
                    ));
                }
            }
        }

        let candidate_key = (left.index, right.index, &candidate);
        let best_key = (
            best.left_boundary_index.unwrap_or(usize::MAX),
            best.right_boundary_index.unwrap_or(usize::MAX),
            &best.witness_variable_ids,
        );
        if candidate.len() > best.maximum_distinct_midi_clique
            || (candidate.len() == best.maximum_distinct_midi_clique
                && candidate_key < best_key)
        {
            best.maximum_distinct_midi_clique = candidate.len();
            best.witness_variable_ids = candidate;
            best.left_boundary_index = Some(left.index);
            best.right_boundary_index = Some(right.index);
        }
    }
    Ok(best)
}

fn append_len_prefixed(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    out.extend_from_slice(bytes);
}

fn append_hand(out: &mut Vec<u8>, hand: Hand) {
    out.push(match hand {
        Hand::Left => 0,
        Hand::Right => 1,
    });
}

fn formula_fingerprint(formula: &SymbolicFormula) -> String {
    let mut bytes = Vec::new();
    for value in [
        SCHEMA,
        STATIC_PREDICATE_SEMANTICS,
        TRANSITION_PREDICATE_SEMANTICS,
        SAME_KEY_PREDICATE_SEMANTICS,
        FORMULA_SEMANTICS,
    ] {
        append_len_prefixed(&mut bytes, value.as_bytes());
    }
    append_len_prefixed(&mut bytes, formula.score_fingerprint_sha256.as_bytes());
    bytes.extend_from_slice(&(SLOT_COUNT as u64).to_le_bytes());
    for index in 0..SLOT_COUNT {
        append_len_prefixed(
            &mut bytes,
            slot_name(index).unwrap_or("invalid-slot").as_bytes(),
        );
    }

    bytes.extend_from_slice(&(formula.variables.len() as u64).to_le_bytes());
    for variable in &formula.variables {
        bytes.extend_from_slice(&(variable.id as u64).to_le_bytes());
        bytes.extend_from_slice(&i64::from(variable.midi).to_le_bytes());
        bytes.extend_from_slice(&variable.attack_tick.to_le_bytes());
        bytes.extend_from_slice(&variable.release_tick.to_le_bytes());
        bytes.extend_from_slice(&(variable.obligation_ids.len() as u64).to_le_bytes());
        for obligation_id in &variable.obligation_ids {
            bytes.extend_from_slice(&(*obligation_id as u64).to_le_bytes());
        }
    }
    bytes.extend_from_slice(&(formula.obligation_to_variable.len() as u64).to_le_bytes());
    for variable_id in &formula.obligation_to_variable {
        bytes.extend_from_slice(&(*variable_id as u64).to_le_bytes());
    }
    bytes.extend_from_slice(&(formula.inequality_constraints.len() as u64).to_le_bytes());
    for (left, right) in &formula.inequality_constraints {
        bytes.extend_from_slice(&(*left as u64).to_le_bytes());
        bytes.extend_from_slice(&(*right as u64).to_le_bytes());
    }
    bytes.extend_from_slice(&(formula.boundary_terms.len() as u64).to_le_bytes());
    for boundary in &formula.boundary_terms {
        bytes.extend_from_slice(&(boundary.index as u64).to_le_bytes());
        bytes.extend_from_slice(&boundary.tick.to_le_bytes());
        bytes.extend_from_slice(&boundary.time_us_floor.to_le_bytes());
        bytes.extend_from_slice(&(boundary.active_variable_ids.len() as u64).to_le_bytes());
        for variable_id in &boundary.active_variable_ids {
            bytes.extend_from_slice(&(*variable_id as u64).to_le_bytes());
        }
        bytes.extend_from_slice(&(boundary.attacking_obligation_ids.len() as u64).to_le_bytes());
        for obligation_id in &boundary.attacking_obligation_ids {
            bytes.extend_from_slice(&(*obligation_id as u64).to_le_bytes());
        }
    }
    bytes.extend_from_slice(&(formula.static_evaluation_sites.len() as u64).to_le_bytes());
    for site in &formula.static_evaluation_sites {
        bytes.extend_from_slice(&(site.boundary_index as u64).to_le_bytes());
        append_hand(&mut bytes, site.hand);
    }
    bytes.extend_from_slice(&(formula.transition_evaluation_sites.len() as u64).to_le_bytes());
    for site in &formula.transition_evaluation_sites {
        bytes.extend_from_slice(&(site.boundary_index as u64).to_le_bytes());
        append_hand(&mut bytes, site.hand);
    }
    bytes.extend_from_slice(&(formula.same_key_evaluation_sites.len() as u64).to_le_bytes());
    for site in &formula.same_key_evaluation_sites {
        bytes.extend_from_slice(&(site.current_obligation_id as u64).to_le_bytes());
        bytes.extend_from_slice(&(site.current_variable_id as u64).to_le_bytes());
        bytes.extend_from_slice(&i64::from(site.midi).to_le_bytes());
        bytes.extend_from_slice(&site.current_attack_time_us_floor.to_le_bytes());
        bytes.push(match site.surface {
            KeySurface::White => 0,
            KeySurface::Black => 1,
        });
        bytes.extend_from_slice(&(site.prior_attacks_in_time_order.len() as u64).to_le_bytes());
        for prior in &site.prior_attacks_in_time_order {
            bytes.extend_from_slice(&(prior.obligation_id as u64).to_le_bytes());
            bytes.extend_from_slice(&(prior.variable_id as u64).to_le_bytes());
            bytes.extend_from_slice(&prior.attack_time_us_floor.to_le_bytes());
        }
    }
    bytes.extend_from_slice(&(formula.maximum_concurrent_variables as u64).to_le_bytes());
    bytes.extend_from_slice(&(formula.structural_graph_degeneracy as u64).to_le_bytes());
    bytes.push(u8::from(formula.structural_witness_found));
    bytes.push(u8::from(formula.concurrent_clique_obstruction));
    bytes.extend_from_slice(
        &(formula.direct_boundary_clique_proof.maximum_distinct_midi_clique as u64).to_le_bytes(),
    );
    bytes.extend_from_slice(
        &(formula
            .direct_boundary_clique_proof
            .left_boundary_index
            .map(|value| value as u64)
            .unwrap_or(u64::MAX))
        .to_le_bytes(),
    );
    bytes.extend_from_slice(
        &(formula
            .direct_boundary_clique_proof
            .right_boundary_index
            .map(|value| value as u64)
            .unwrap_or(u64::MAX))
        .to_le_bytes(),
    );
    bytes.extend_from_slice(
        &(formula.direct_boundary_clique_proof.witness_variable_ids.len() as u64).to_le_bytes(),
    );
    for variable_id in &formula.direct_boundary_clique_proof.witness_variable_ids {
        bytes.extend_from_slice(&(*variable_id as u64).to_le_bytes());
    }
    bytes.push(u8::from(formula.direct_boundary_clique_obstruction));
    sha256::hex(&bytes)
}

fn build_symbolic_formula(score: &PianisticExecutionScore) -> Result<SymbolicFormula, String> {
    let topology = build_demand_topology(score)?;
    let boundaries = build_boundaries(&topology.obligations)?;
    let (variables, obligation_to_variable) = build_slot_variables(&topology.obligations)?;
    let boundary_terms = build_boundary_terms(&boundaries, &obligation_to_variable)?;
    let inequality_constraints = build_inequality_constraints(&variables, &boundary_terms);
    let static_evaluation_sites = build_static_evaluation_sites(&boundary_terms);
    let transition_evaluation_sites = build_transition_evaluation_sites(&boundary_terms);
    let same_key_evaluation_sites =
        build_same_key_evaluation_sites(&topology.obligations, &obligation_to_variable)?;
    let maximum_concurrent_variables = maximum_concurrent_variables(&variables);
    let structural_coloring =
        structural_coloring_witness(&variables, &inequality_constraints)?;
    let structural_graph_degeneracy = structural_coloring.degeneracy;
    let structural_witness_found = structural_coloring.coloring.is_some();
    let concurrent_clique_obstruction = maximum_concurrent_variables > SLOT_COUNT;
    let direct_boundary_clique_proof =
        direct_boundary_clique_proof(&variables, &boundary_terms, &inequality_constraints)?;
    let direct_boundary_clique_obstruction =
        direct_boundary_clique_proof.maximum_distinct_midi_clique > SLOT_COUNT;
    let mut formula = SymbolicFormula {
        score_fingerprint_sha256: score.musical_execution_fingerprint_sha256.clone(),
        source_execution_events: topology.source_execution_events,
        physical_key_obligations: topology.obligations.len(),
        variables,
        obligation_to_variable,
        inequality_constraints,
        boundary_terms,
        static_evaluation_sites,
        transition_evaluation_sites,
        same_key_evaluation_sites,
        maximum_concurrent_variables,
        structural_graph_degeneracy,
        structural_witness_found,
        concurrent_clique_obstruction,
        direct_boundary_clique_proof,
        direct_boundary_clique_obstruction,
        formula_sha256: String::new(),
    };
    formula.formula_sha256 = formula_fingerprint(&formula);
    Ok(formula)
}

fn component_is_overlap_connected(
    variable: &SlotVariable,
    obligations: &[PhysicalKeyObligation],
) -> bool {
    let mut members = variable
        .obligation_ids
        .iter()
        .filter_map(|id| obligations.get(*id))
        .collect::<Vec<_>>();
    if members.len() != variable.obligation_ids.len() || members.is_empty() {
        return false;
    }
    if members.iter().any(|item| item.midi != variable.midi) {
        return false;
    }
    members.sort_by_key(|item| (item.attack_tick, item.release_tick, item.id));
    let mut covered_until = members[0].release_tick;
    for member in members.iter().skip(1) {
        if member.attack_tick >= covered_until {
            return false;
        }
        covered_until = covered_until.max(member.release_tick);
    }
    members
        .iter()
        .map(|item| item.attack_tick)
        .min()
        .is_some_and(|value| value == variable.attack_tick)
        && members
            .iter()
            .map(|item| item.release_tick)
            .max()
            .is_some_and(|value| value == variable.release_tick)
}

fn request_for_hand(
    formula: &SymbolicFormula,
    assignment: &[usize],
    boundary: &BoundaryTerm,
    hand: Hand,
) -> Result<StaticConfigurationRequest, String> {
    let mut request = BTreeSet::<FingerKeyAssignment>::new();
    for variable_id in &boundary.active_variable_ids {
        let variable = formula
            .variables
            .get(*variable_id)
            .ok_or_else(|| "valuation-boundary-variable-out-of-range".to_string())?;
        let slot_index = *assignment
            .get(*variable_id)
            .ok_or_else(|| "valuation-slot-variable-unassigned".to_string())?;
        let slot = slot_identity(slot_index)
            .ok_or_else(|| "valuation-slot-outside-v2-domain".to_string())?;
        if slot.hand == hand {
            request.insert(FingerKeyAssignment {
                finger: slot.finger,
                midi: variable.midi,
            });
        }
    }
    Ok(StaticConfigurationRequest {
        hand,
        assignments: request.into_iter().collect(),
    })
}

fn validate_structural_assignment(
    formula: &SymbolicFormula,
    assignment: &[usize],
) -> Result<(), String> {
    if assignment.len() != formula.variables.len() {
        return Err("valuation-cardinality-mismatch".to_string());
    }
    if assignment.iter().any(|slot| *slot >= SLOT_COUNT) {
        return Err("valuation-slot-outside-v2-domain".to_string());
    }
    if formula
        .inequality_constraints
        .iter()
        .any(|(left, right)| assignment.get(*left) == assignment.get(*right))
    {
        return Err("valuation-violates-overlap-slot-inequality".to_string());
    }
    Ok(())
}

fn evaluate_capability_atoms(
    formula: &SymbolicFormula,
    assignment: &[usize],
) -> Result<BTreeSet<ConcreteCapabilityAtom>, String> {
    validate_structural_assignment(formula, assignment)?;
    let geometry = KeyboardGeometry::standard_88_key();
    let mut atoms = BTreeSet::new();
    let mut previous_requests = BTreeMap::<Hand, StaticConfigurationRequest>::new();
    let mut previous_boundary_time = None;
    let mut last_nonempty = BTreeMap::<Hand, StaticConfigurationRequest>::new();
    let mut empty_since = BTreeMap::<Hand, u64>::new();

    for boundary in &formula.boundary_terms {
        for hand in HANDS {
            let current = request_for_hand(formula, assignment, boundary, hand)?;
            if !current.assignments.is_empty() {
                let signature = configuration_signature(geometry, &current)
                    .ok_or_else(|| "valuation-static-signature-unavailable".to_string())?;
                atoms.insert(ConcreteCapabilityAtom::Static {
                    boundary_index: boundary.index,
                    signature,
                });
            }

            if current.assignments.is_empty() {
                if let Some(previous) = previous_requests.get(&hand).cloned()
                    && !previous.assignments.is_empty()
                {
                    last_nonempty.insert(hand, previous);
                    empty_since.insert(hand, boundary.time_us_floor);
                }
                previous_requests.insert(hand, current);
                continue;
            }

            let transition_from_and_available = if let Some(previous) = previous_requests
                .get(&hand)
                .filter(|request| !request.assignments.is_empty())
                .cloned()
            {
                let available = previous_boundary_time
                    .and_then(|time: u64| boundary.time_us_floor.checked_sub(time))
                    .unwrap_or(0);
                Some((previous, available))
            } else if let (Some(previous), Some(since)) =
                (last_nonempty.get(&hand).cloned(), empty_since.get(&hand).copied())
            {
                Some((previous, boundary.time_us_floor.saturating_sub(since)))
            } else {
                None
            };

            if let Some((from, available_time_us)) = transition_from_and_available
                && from != current
            {
                let signature = transition_signature(geometry, &from, &current)
                    .ok_or_else(|| "valuation-transition-signature-unavailable".to_string())?;
                atoms.insert(ConcreteCapabilityAtom::Transition {
                    boundary_index: boundary.index,
                    signature,
                    available_time_us,
                });
            }
            last_nonempty.insert(hand, current.clone());
            empty_since.remove(&hand);
            previous_requests.insert(hand, current);
        }
        previous_boundary_time = Some(boundary.time_us_floor);
    }

    for site in &formula.same_key_evaluation_sites {
        let current_slot = *assignment
            .get(site.current_variable_id)
            .ok_or_else(|| "same-key-current-slot-unassigned".to_string())?;
        let Some(prior) = site
            .prior_attacks_in_time_order
            .iter()
            .rev()
            .find(|prior| assignment.get(prior.variable_id).copied() == Some(current_slot))
        else {
            continue;
        };
        let slot = slot_identity(current_slot)
            .ok_or_else(|| "same-key-slot-outside-v2-domain".to_string())?;
        let available_time_us = site
            .current_attack_time_us_floor
            .saturating_sub(prior.attack_time_us_floor);
        atoms.insert(ConcreteCapabilityAtom::SameKey {
            current_obligation_id: site.current_obligation_id,
            signature: SameKeyTechniqueSignature {
                hand: slot.hand,
                finger: slot.finger,
                surface: site.surface,
            },
            available_time_us,
        });
    }
    Ok(atoms)
}

fn verify_symbolic_formula(
    score: &PianisticExecutionScore,
    formula: &SymbolicFormula,
) -> SymbolicReplay {
    let topology = match build_demand_topology(score) {
        Ok(value) => value,
        Err(reason) => {
            return SymbolicReplay {
                verified: false,
                reason: format!("replay-demand-topology-failed:{reason}"),
            };
        }
    };
    if formula.score_fingerprint_sha256 != score.musical_execution_fingerprint_sha256
        || formula.source_execution_events != topology.source_execution_events
        || formula.physical_key_obligations != topology.obligations.len()
        || formula.obligation_to_variable.len() != topology.obligations.len()
    {
        return SymbolicReplay {
            verified: false,
            reason: "replay-cardinality-or-score-binding-mismatch".to_string(),
        };
    }

    if formula
        .variables
        .iter()
        .enumerate()
        .any(|(index, variable)| variable.id != index)
    {
        return SymbolicReplay {
            verified: false,
            reason: "replay-variable-id-order-mismatch".to_string(),
        };
    }

    let mut coverage = vec![0usize; topology.obligations.len()];
    for variable in &formula.variables {
        if !component_is_overlap_connected(variable, &topology.obligations) {
            return SymbolicReplay {
                verified: false,
                reason: format!("replay-invalid-same-key-component:{}", variable.id),
            };
        }
        for obligation_id in &variable.obligation_ids {
            let Some(cell) = coverage.get_mut(*obligation_id) else {
                return SymbolicReplay {
                    verified: false,
                    reason: "replay-component-obligation-out-of-range".to_string(),
                };
            };
            *cell = cell.saturating_add(1);
            if formula.obligation_to_variable.get(*obligation_id).copied() != Some(variable.id) {
                return SymbolicReplay {
                    verified: false,
                    reason: "replay-obligation-variable-map-mismatch".to_string(),
                };
            }
        }
    }
    if coverage.iter().any(|count| *count != 1) {
        return SymbolicReplay {
            verified: false,
            reason: "replay-obligation-coverage-not-exact".to_string(),
        };
    }

    for (left_index, left) in topology.obligations.iter().enumerate() {
        for right in topology.obligations.iter().skip(left_index + 1) {
            if left.midi == right.midi
                && intervals_overlap(
                    left.attack_tick,
                    left.release_tick,
                    right.attack_tick,
                    right.release_tick,
                )
                && formula.obligation_to_variable.get(left.id)
                    != formula.obligation_to_variable.get(right.id)
            {
                return SymbolicReplay {
                    verified: false,
                    reason: "replay-overlapping-same-key-not-collapsed".to_string(),
                };
            }
        }
    }

    let expected_inequalities =
        build_inequality_constraints(&formula.variables, &formula.boundary_terms);
    if expected_inequalities != formula.inequality_constraints {
        return SymbolicReplay {
            verified: false,
            reason: "replay-inequality-set-mismatch".to_string(),
        };
    }
    let expected_boundaries = match build_boundaries(&topology.obligations) {
        Ok(value) => value,
        Err(reason) => {
            return SymbolicReplay {
                verified: false,
                reason: format!("replay-boundary-rebuild-failed:{reason}"),
            };
        }
    };
    let expected_boundary_terms =
        match build_boundary_terms(&expected_boundaries, &formula.obligation_to_variable) {
            Ok(value) => value,
            Err(reason) => {
                return SymbolicReplay {
                    verified: false,
                    reason: format!("replay-boundary-term-rebuild-failed:{reason}"),
                };
            }
        };
    if expected_boundary_terms != formula.boundary_terms {
        return SymbolicReplay {
            verified: false,
            reason: "replay-boundary-term-set-mismatch".to_string(),
        };
    }
    if build_static_evaluation_sites(&formula.boundary_terms) != formula.static_evaluation_sites {
        return SymbolicReplay {
            verified: false,
            reason: "replay-static-evaluation-site-mismatch".to_string(),
        };
    }
    if build_transition_evaluation_sites(&formula.boundary_terms)
        != formula.transition_evaluation_sites
    {
        return SymbolicReplay {
            verified: false,
            reason: "replay-transition-evaluation-site-mismatch".to_string(),
        };
    }
    let expected_same_key = match build_same_key_evaluation_sites(
        &topology.obligations,
        &formula.obligation_to_variable,
    ) {
        Ok(value) => value,
        Err(reason) => {
            return SymbolicReplay {
                verified: false,
                reason: format!("replay-same-key-evaluation-rebuild-failed:{reason}"),
            };
        }
    };
    if expected_same_key != formula.same_key_evaluation_sites {
        return SymbolicReplay {
            verified: false,
            reason: "replay-same-key-evaluation-site-mismatch".to_string(),
        };
    }

    let expected_maximum = maximum_concurrent_variables(&formula.variables);
    let structural_coloring =
        match structural_coloring_witness(&formula.variables, &formula.inequality_constraints) {
            Ok(value) => value,
            Err(reason) => {
                return SymbolicReplay {
                    verified: false,
                    reason: format!("replay-structural-coloring-failed:{reason}"),
                };
            }
        };
    let expected_nonempty = structural_coloring.coloring.as_ref().is_some_and(|colors| {
        colors.len() == formula.variables.len()
            && coloring_satisfies_constraints(colors, &formula.inequality_constraints)
    });
    let expected_direct_boundary_clique = match direct_boundary_clique_proof(
        &formula.variables,
        &formula.boundary_terms,
        &formula.inequality_constraints,
    ) {
        Ok(value) => value,
        Err(reason) => {
            return SymbolicReplay {
                verified: false,
                reason: format!("replay-direct-boundary-clique-proof-failed:{reason}"),
            };
        }
    };
    if expected_maximum != formula.maximum_concurrent_variables
        || structural_coloring.degeneracy != formula.structural_graph_degeneracy
        || expected_nonempty != formula.structural_witness_found
        || (expected_maximum > SLOT_COUNT) != formula.concurrent_clique_obstruction
        || expected_direct_boundary_clique != formula.direct_boundary_clique_proof
        || (expected_direct_boundary_clique.maximum_distinct_midi_clique > SLOT_COUNT)
            != formula.direct_boundary_clique_obstruction
        || (formula.concurrent_clique_obstruction && formula.structural_witness_found)
        || (formula.direct_boundary_clique_obstruction && formula.structural_witness_found)
    {
        return SymbolicReplay {
            verified: false,
            reason: "replay-structural-space-proof-mismatch".to_string(),
        };
    }
    if formula.formula_sha256 != formula_fingerprint(formula) {
        return SymbolicReplay {
            verified: false,
            reason: "replay-formula-fingerprint-mismatch".to_string(),
        };
    }
    SymbolicReplay {
        verified: true,
        reason: "independent-symbolic-formula-replay-complete".to_string(),
    }
}

fn audit_score(score: &PianisticExecutionScore) -> SymbolicAudit {
    if score.status != ProjectionStatus::Complete {
        return SymbolicAudit {
            status: "MODEL_INPUT_UNAVAILABLE",
            reason: score.reason.clone(),
            source_execution_events: score.execution_events.len(),
            physical_key_obligations: 0,
            slot_variables: 0,
            same_key_overlap_collapses: 0,
            inequality_constraints: 0,
            boundary_count: 0,
            static_evaluation_sites: 0,
            transition_evaluation_sites: 0,
            same_key_evaluation_sites: 0,
            same_key_prior_attack_terms: 0,
            maximum_concurrent_variables: 0,
            structural_graph_degeneracy: 0,
            direct_boundary_clique_maximum: 0,
            direct_boundary_clique_obstruction: false,
            structural_execution_space: "NOT_EVALUATED",
            formula_sha256: "none".to_string(),
            replay: SymbolicReplay {
                verified: false,
                reason: "input-projection-unavailable".to_string(),
            },
        };
    }
    let formula = match build_symbolic_formula(score) {
        Ok(value) => value,
        Err(reason) => {
            return SymbolicAudit {
                status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: score.execution_events.len(),
                physical_key_obligations: 0,
                slot_variables: 0,
                same_key_overlap_collapses: 0,
                inequality_constraints: 0,
                boundary_count: 0,
                static_evaluation_sites: 0,
                transition_evaluation_sites: 0,
                same_key_evaluation_sites: 0,
                same_key_prior_attack_terms: 0,
                maximum_concurrent_variables: 0,
                structural_graph_degeneracy: 0,
                direct_boundary_clique_maximum: 0,
                direct_boundary_clique_obstruction: false,
                structural_execution_space: "NOT_EVALUATED",
                formula_sha256: "none".to_string(),
                replay: SymbolicReplay {
                    verified: false,
                    reason: "symbolic-formula-build-failed".to_string(),
                },
            };
        }
    };
    let replay = verify_symbolic_formula(score, &formula);
    let status = if replay.verified {
        "COMPLETE"
    } else {
        "INTEGRITY_FAILED"
    };
    let reason = if replay.verified {
        "quantified-capability-conditioned-execution-formula-complete".to_string()
    } else {
        replay.reason.clone()
    };
    SymbolicAudit {
        status,
        reason,
        source_execution_events: formula.source_execution_events,
        physical_key_obligations: formula.physical_key_obligations,
        slot_variables: formula.variables.len(),
        same_key_overlap_collapses: formula
            .physical_key_obligations
            .saturating_sub(formula.variables.len()),
        inequality_constraints: formula.inequality_constraints.len(),
        boundary_count: formula.boundary_terms.len(),
        static_evaluation_sites: formula.static_evaluation_sites.len(),
        transition_evaluation_sites: formula.transition_evaluation_sites.len(),
        same_key_evaluation_sites: formula.same_key_evaluation_sites.len(),
        same_key_prior_attack_terms: formula
            .same_key_evaluation_sites
            .iter()
            .map(|site| site.prior_attacks_in_time_order.len())
            .sum(),
        maximum_concurrent_variables: formula.maximum_concurrent_variables,
        structural_graph_degeneracy: formula.structural_graph_degeneracy,
        direct_boundary_clique_maximum: formula
            .direct_boundary_clique_proof
            .maximum_distinct_midi_clique,
        direct_boundary_clique_obstruction: formula.direct_boundary_clique_obstruction,
        structural_execution_space: if formula.structural_witness_found {
            "NONEMPTY_WITNESS_VERIFIED"
        } else if formula.concurrent_clique_obstruction {
            "EMPTY_CONCURRENT_CLIQUE_GT10"
        } else if formula.direct_boundary_clique_obstruction {
            "EMPTY_DIRECT_BOUNDARY_CLIQUE_GT10"
        } else {
            "UNRESOLVED_NO_FALSE_NEGATIVE"
        },
        formula_sha256: formula.formula_sha256,
        replay,
    }
}

fn synthetic_event(
    midi: i32,
    attack_tick: i64,
    release_tick: i64,
    attack_time_us_floor: u64,
    release_time_us_floor: u64,
) -> pianistic_execution_score::PianisticExecutionEvent {
    pianistic_execution_score::PianisticExecutionEvent {
        part_id: "P1".to_string(),
        source_staff: "1".to_string(),
        source_voice: "1".to_string(),
        midi,
        attack_tick,
        release_tick,
        attack_time_us_floor,
        release_time_us_floor,
        note_segments: 1,
    }
}

fn synthetic_score(
    events: Vec<pianistic_execution_score::PianisticExecutionEvent>,
) -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "synthetic-symbolic-capability-reachability".to_string(),
        source_path: PathBuf::from("synthetic-symbolic-capability-reachability.musicxml"),
        pitched_note_segments: events.len(),
        execution_events: events,
        tie_continuation_segments: 0,
        tempo_points: Vec::new(),
        terminal_tick: 384,
        terminal_physical_time_us_floor: Some(2_000_000),
        musical_execution_fingerprint_sha256:
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
        source_affinity_fingerprint_sha256:
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string(),
    }
}

fn runtime_structural_assignment(formula: &SymbolicFormula) -> Result<Vec<usize>, String> {
    structural_coloring_witness(&formula.variables, &formula.inequality_constraints)?
        .coloring
        .ok_or_else(|| "synthetic-structural-assignment-unavailable".to_string())
}

fn verify_runtime_integrity() -> Result<String, String> {
    let different_keys = synthetic_score(vec![
        synthetic_event(60, 0, 192, 0, 1_000_000),
        synthetic_event(64, 96, 288, 500_000, 1_500_000),
    ]);
    let different_formula = build_symbolic_formula(&different_keys)?;
    if different_formula.variables.len() != 2
        || different_formula.inequality_constraints != vec![(0, 1)]
        || !different_formula.structural_witness_found
        || !verify_symbolic_formula(&different_keys, &different_formula).verified
    {
        return Err("different-key-symbolic-constraint-integrity-failed".to_string());
    }
    let different_assignment = runtime_structural_assignment(&different_formula)?;
    let different_atoms = evaluate_capability_atoms(&different_formula, &different_assignment)?;
    if !different_atoms
        .iter()
        .any(|atom| matches!(atom, ConcreteCapabilityAtom::Static { .. }))
        || !different_atoms
            .iter()
            .any(|atom| matches!(atom, ConcreteCapabilityAtom::Transition { .. }))
    {
        return Err("valuation-derived-static-transition-atoms-missing".to_string());
    }

    let direct_boundary_different_key = synthetic_score(vec![
        synthetic_event(60, 0, 96, 0, 500_000),
        synthetic_event(62, 96, 192, 500_000, 1_000_000),
    ]);
    let direct_formula = build_symbolic_formula(&direct_boundary_different_key)?;
    if direct_formula.inequality_constraints != vec![(0, 1)]
        || evaluate_capability_atoms(&direct_formula, &[5, 5]).is_ok()
        || evaluate_capability_atoms(&direct_formula, &[5, 6]).is_err()
    {
        return Err("direct-boundary-different-key-slot-reuse-constraint-missing".to_string());
    }

    let direct_boundary_clique = synthetic_score(
        (0..6)
            .map(|offset| synthetic_event(48 + offset, 0, 96, 0, 500_000))
            .chain(
                (0..5)
                    .map(|offset| synthetic_event(60 + offset, 96, 192, 500_000, 1_000_000)),
            )
            .collect(),
    );
    let direct_clique_formula = build_symbolic_formula(&direct_boundary_clique)?;
    if direct_clique_formula.maximum_concurrent_variables != 6
        || direct_clique_formula.concurrent_clique_obstruction
        || direct_clique_formula
            .direct_boundary_clique_proof
            .maximum_distinct_midi_clique
            != 11
        || !direct_clique_formula.direct_boundary_clique_obstruction
        || direct_clique_formula.structural_witness_found
        || !verify_symbolic_formula(&direct_boundary_clique, &direct_clique_formula).verified
    {
        return Err("direct-boundary-clique-obstruction-integrity-failed".to_string());
    }

    let overlapping_same_key = synthetic_score(vec![
        synthetic_event(60, 0, 192, 0, 1_000_000),
        synthetic_event(60, 96, 288, 500_000, 1_500_000),
    ]);
    let overlap_formula = build_symbolic_formula(&overlapping_same_key)?;
    let overlap_assignment = runtime_structural_assignment(&overlap_formula)?;
    let overlap_atoms = evaluate_capability_atoms(&overlap_formula, &overlap_assignment)?;
    if overlap_formula.variables.len() != 1
        || overlap_formula.same_key_evaluation_sites.len() != 1
        || !overlap_atoms.iter().any(|atom| {
            matches!(
                atom,
                ConcreteCapabilityAtom::SameKey {
                    available_time_us: 500_000,
                    ..
                }
            )
        })
    {
        return Err("overlapping-same-key-symbolic-evaluator-integrity-failed".to_string());
    }

    let post_release_same_key = synthetic_score(vec![
        synthetic_event(60, 0, 96, 0, 500_000),
        synthetic_event(60, 96, 192, 500_000, 1_000_000),
    ]);
    let post_release_formula = build_symbolic_formula(&post_release_same_key)?;
    let same_slot_assignment = vec![5, 5];
    let different_slot_assignment = vec![5, 6];
    let same_slot_atoms = evaluate_capability_atoms(&post_release_formula, &same_slot_assignment)?;
    let different_slot_atoms =
        evaluate_capability_atoms(&post_release_formula, &different_slot_assignment)?;
    if post_release_formula.variables.len() != 2
        || !post_release_formula.inequality_constraints.is_empty()
        || !same_slot_atoms.iter().any(|atom| {
            matches!(
                atom,
                ConcreteCapabilityAtom::SameKey {
                    available_time_us: 500_000,
                    ..
                }
            )
        })
        || different_slot_atoms
            .iter()
            .any(|atom| matches!(atom, ConcreteCapabilityAtom::SameKey { .. }))
    {
        return Err("post-release-same-key-conditional-guard-integrity-failed".to_string());
    }

    let latest_prior = synthetic_score(vec![
        synthetic_event(60, 0, 64, 0, 250_000),
        synthetic_event(60, 96, 160, 500_000, 750_000),
        synthetic_event(60, 192, 256, 1_000_000, 1_250_000),
    ]);
    let latest_formula = build_symbolic_formula(&latest_prior)?;
    let all_same_slot_atoms = evaluate_capability_atoms(&latest_formula, &[5, 5, 5])?;
    let shadowed_middle_atoms = evaluate_capability_atoms(&latest_formula, &[5, 6, 5])?;
    let all_same_current_delta = all_same_slot_atoms.iter().find_map(|atom| match atom {
        ConcreteCapabilityAtom::SameKey {
            current_obligation_id: 2,
            available_time_us,
            ..
        } => Some(*available_time_us),
        _ => None,
    });
    let shadowed_current_delta = shadowed_middle_atoms.iter().find_map(|atom| match atom {
        ConcreteCapabilityAtom::SameKey {
            current_obligation_id: 2,
            available_time_us,
            ..
        } => Some(*available_time_us),
        _ => None,
    });
    if all_same_current_delta != Some(500_000) || shadowed_current_delta != Some(1_000_000) {
        return Err("latest-prior-same-slot-history-semantics-mismatch".to_string());
    }

    let eleven_events = (0..11)
        .map(|index| synthetic_event(48 + index, 0, 96, 0, 500_000))
        .collect::<Vec<_>>();
    let eleven_formula = build_symbolic_formula(&synthetic_score(eleven_events))?;
    if eleven_formula.maximum_concurrent_variables != 11
        || eleven_formula.structural_witness_found
        || !eleven_formula.concurrent_clique_obstruction
    {
        return Err("eleven-key-structural-empty-space-integrity-failed".to_string());
    }

    let crown_variables = (0..22)
        .map(|id| SlotVariable {
            id,
            midi: 40 + i32::try_from(id).unwrap_or(0),
            attack_tick: 0,
            release_tick: 1,
            obligation_ids: vec![id],
        })
        .collect::<Vec<_>>();
    let mut crown_inequalities = Vec::new();
    for left_index in 0..11 {
        for right_index in 0..11 {
            if left_index != right_index {
                crown_inequalities.push((left_index * 2, right_index * 2 + 1));
            }
        }
    }
    crown_inequalities.sort_unstable();
    crown_inequalities.dedup();
    let crown_coloring = structural_coloring_witness(&crown_variables, &crown_inequalities)?;
    if crown_coloring.degeneracy != 10
        || crown_coloring.coloring.as_ref().is_none_or(|colors| {
            !coloring_satisfies_constraints(colors, &crown_inequalities)
                || colors.iter().copied().max().unwrap_or(0) >= SLOT_COUNT
        })
    {
        return Err("smallest-last-crown-regression-integrity-failed".to_string());
    }

    let retimed = synthetic_score(vec![
        synthetic_event(60, 0, 96, 0, 400_000),
        synthetic_event(60, 96, 192, 400_000, 1_000_000),
    ]);
    let retimed_formula = build_symbolic_formula(&retimed)?;
    if retimed_formula.formula_sha256 == post_release_formula.formula_sha256 {
        return Err("symbolic-formula-fingerprint-does-not-bind-time".to_string());
    }

    Ok(format!(
        "PASS; QuantifiedSlotDomain {SLOT_COUNT}; SameKeyOverlapEquality VERIFIED; DifferentKeyOverlapInequality VERIFIED; DirectBoundaryDifferentKeyReuseConstraint VERIFIED; ExactStaticEvaluator VERIFIED; ExactTransitionEvaluator VERIFIED; SameKeyConditionalLatestPriorEvaluator VERIFIED; StructuralEmptySpaceAtElevenConcurrentKeys VERIFIED; SmallestLastCrownRegression VERIFIED; StructuralWitnessMethod smallest-last-degeneracy-first-fit; DirectBoundaryCliqueObstruction VERIFIED; FormulaTimeBinding VERIFIED; CapabilityDecision none; PathSelection none"
    ))
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("===============================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; InputAuthority: PianisticExecutionScoreAuthority+PianisticExecutionPlanWitnessDemandTopology; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("QuantifiedFormulaContract: each overlap-connected same-MIDI obligation component is one existential variable over exactly ten named execution-v2 hand/finger slots. Overlapping different-key components require unequal slot values. The existing v2 direct-boundary rule is also preserved: a slot occupied at the previous boundary cannot be reused by a different MIDI at the immediately following boundary. This is a quantified domain; no stored structural coloring is a chosen fingering.\n");
    out.push_str("CapabilityEvaluatorContract: score boundaries plus the slot valuation deterministically construct exact r295 StaticConfigurationSignature atoms. The exact r296 previous-nonempty history rule constructs TransitionSignature atoms with score-derived available time. For every later same-MIDI attack, the exact r619 rule selects the latest prior attack assigned to the same slot and constructs SameKeyTechniqueSignature plus attack-to-attack time. Concrete capability signatures are valuation-derived formula terms, not score-derived capability answers.\n");
    out.push_str("StructuralExistenceProofContract: r622 preserves deterministic smallest-last plus constraint-validated reverse first-fit as a positive witness search. r623 additionally proves the direct-boundary obstruction already encoded by the v2 inequality graph: the union of two consecutive active boundary sets, deduplicated by MIDI, is a pairwise inequality clique. More than ten distinct MIDI values in such a union therefore proves EMPTY_DIRECT_BOUNDARY_CLIQUE_GT10 without any capability valuation. Coloring failure without a proved clique remains UNRESOLVED. No coloring is exported as a preferred fingering and no capability atom is valued.\n");
    out.push_str("LogicBoundary: UNMODELED remains an external symbolic variable, never SUPPORTED and never REJECTED. FormulaSemantics: ");
    out.push_str(FORMULA_SEMANTICS);
    out.push_str(". Capability valuation, ranking, easiest-fingering selection and HumanPlayabilityClaim are outside this audit.\n");
    out.push_str("OntologyBoundaryV2: hand/finger identity is fixed for each physical obligation/overlap component; one slot controls at most one distinct MIDI at an overlapping instant. Finger substitution during hold, hand takeover during hold and multi-key-per-finger techniques remain unmodeled. Empty structural space therefore means only NO_REALIZATION_UNDER_V2_EXECUTION_ONTOLOGY, never human-unplayable.\n");
    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticSymbolicCapabilityReachability runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "SymbolicRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason score-title-missing; SymbolicFormulaStatus NOT_EVALUATED; CapabilityValuation UNBOUND; PathSelection NONE; HumanPlayabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let path = PathBuf::from(format!("{title}.musicxml"));
    let score = pianistic_execution_score::project_file(&path);
    let audit = audit_score(&score);
    let _ = writeln!(
        out,
        "SymbolicFormulaStatus: {}; Reason {}; SourceExecutionEvents {}; PhysicalKeyObligations {}; SlotVariables {}; SameKeyOverlapCollapses {}; InequalityConstraints {}; Boundaries {}; StaticEvaluationSites {}; TransitionEvaluationSites {}; SameKeyEvaluationSites {}; SameKeyPriorAttackTerms {}; MaximumConcurrentVariables {}; StructuralGraphDegeneracy {}; DirectBoundaryCliqueMaximum {}; DirectBoundaryCliqueObstruction {}; StructuralExecutionSpace {}; FormulaSha256 {}.",
        audit.status,
        audit.reason,
        audit.source_execution_events,
        audit.physical_key_obligations,
        audit.slot_variables,
        audit.same_key_overlap_collapses,
        audit.inequality_constraints,
        audit.boundary_count,
        audit.static_evaluation_sites,
        audit.transition_evaluation_sites,
        audit.same_key_evaluation_sites,
        audit.same_key_prior_attack_terms,
        audit.maximum_concurrent_variables,
        audit.structural_graph_degeneracy,
        audit.direct_boundary_clique_maximum,
        if audit.direct_boundary_clique_obstruction { "yes" } else { "no" },
        audit.structural_execution_space,
        audit.formula_sha256,
    );
    let _ = writeln!(
        out,
        "IndependentSymbolicReplay: {}; Reason {}; CapabilityFormulaRepresentation QUANTIFIED_SLOT_CONSTRAINTS_PLUS_EXACT_VALUATION_EVALUATORS; StructuralWitnessMethod SMALLEST_LAST_DEGENERACY_FIRST_FIT_PLUS_DIRECT_BOUNDARY_CLIQUE_PROOF_CONSTRAINT_VALIDATED; StaticSignature r295_EXACT_VALUATION_DERIVED; TransitionSignature r296_EXACT_VALUATION_DERIVED; SameKeyTechniqueSignature r619_EXACT_VALUATION_DERIVED; CapabilityValuation UNBOUND; PathSelection NONE; Ranking NONE; ScoreDerivedCapabilityAnswers none; HumanPlayabilityClaim {PLAYABILITY_CLAIM}.",
        if audit.replay.verified { "VERIFIED" } else { "REJECTED" },
        audit.replay.reason,
    );
    if audit.status == "COMPLETE" {
        if audit.structural_execution_space == "NONEMPTY_WITNESS_VERIFIED" {
            out.push_str("ObjectiveScoreDerivedPlayabilityDomain: CLOSED_WITHIN_V2_EXECUTION_ONTOLOGY; StructuralExecutionSpace NONEMPTY_WITNESS_VERIFIED; RemainingTruthSource external_capability_valuation_only. This closes the score-derived question but does not assert that any human valuation satisfies the formula.\n\n");
        } else if audit.structural_execution_space == "EMPTY_CONCURRENT_CLIQUE_GT10" {
            out.push_str("ObjectiveScoreDerivedPlayabilityDomain: CLOSED_WITHIN_V2_EXECUTION_ONTOLOGY; StructuralExecutionSpace EMPTY_CONCURRENT_CLIQUE_GT10; Result NO_REALIZATION_UNDER_V2_EXECUTION_ONTOLOGY; HumanPlayabilityClaim NOT_EVALUATED.\n\n");
        } else if audit.structural_execution_space == "EMPTY_DIRECT_BOUNDARY_CLIQUE_GT10" {
            out.push_str("ObjectiveScoreDerivedPlayabilityDomain: CLOSED_WITHIN_V2_EXECUTION_ONTOLOGY; StructuralExecutionSpace EMPTY_DIRECT_BOUNDARY_CLIQUE_GT10; Result NO_REALIZATION_UNDER_V2_EXECUTION_ONTOLOGY; BoundaryCliqueAuthority direct-v2-slot-inequalities-only; HumanPlayabilityClaim NOT_EVALUATED.\n\n");
        } else {
            out.push_str("ObjectiveScoreDerivedPlayabilityDomain: CLOSED_FORMULA_WITH_STRUCTURAL_SATISFIABILITY_UNRESOLVED; no negative playability inference is permitted; HumanPlayabilityClaim NOT_EVALUATED.\n\n");
        }
    } else {
        out.push_str("ObjectiveScoreDerivedPlayabilityDomain: NOT_CLOSED; no human or model-relative playability inference is permitted.\n\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_same_key_obligations_share_one_symbolic_slot_variable() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 192, 0, 1_000_000),
            synthetic_event(60, 96, 288, 500_000, 1_500_000),
        ]);
        let formula = build_symbolic_formula(&score).expect("formula");
        assert_eq!(formula.variables.len(), 1);
        assert_eq!(formula.obligation_to_variable, vec![0, 0]);
        assert!(formula.inequality_constraints.is_empty());
        assert_eq!(formula.same_key_evaluation_sites.len(), 1);
    }

    #[test]
    fn overlapping_different_keys_require_distinct_symbolic_slots() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 192, 0, 1_000_000),
            synthetic_event(64, 96, 288, 500_000, 1_500_000),
        ]);
        let formula = build_symbolic_formula(&score).expect("formula");
        assert_eq!(formula.variables.len(), 2);
        assert_eq!(formula.inequality_constraints, vec![(0, 1)]);
        assert!(formula.structural_witness_found);
    }

    #[test]
    fn direct_boundary_different_key_slot_reuse_is_structurally_forbidden() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 500_000),
            synthetic_event(62, 96, 192, 500_000, 1_000_000),
        ]);
        let formula = build_symbolic_formula(&score).expect("formula");
        assert_eq!(formula.variables.len(), 2);
        assert_eq!(formula.inequality_constraints, vec![(0, 1)]);
        assert!(evaluate_capability_atoms(&formula, &[5, 5]).is_err());
        assert!(evaluate_capability_atoms(&formula, &[5, 6]).is_ok());
    }

    #[test]
    fn valuation_evaluator_materializes_exact_static_transition_and_same_key_atoms() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 500_000),
            synthetic_event(64, 96, 192, 500_000, 1_000_000),
            synthetic_event(60, 192, 288, 1_000_000, 1_500_000),
        ]);
        let formula = build_symbolic_formula(&score).expect("formula");
        let atoms = evaluate_capability_atoms(&formula, &[5, 6, 5]).expect("atoms");
        assert!(atoms.iter().any(|atom| matches!(atom, ConcreteCapabilityAtom::Static { .. })));
        assert!(atoms.iter().any(|atom| matches!(atom, ConcreteCapabilityAtom::Transition { .. })));
        assert!(atoms.iter().any(|atom| matches!(atom, ConcreteCapabilityAtom::SameKey { .. })));
    }

    #[test]
    fn same_key_latest_prior_same_slot_is_assignment_dependent() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 64, 0, 250_000),
            synthetic_event(60, 96, 160, 500_000, 750_000),
            synthetic_event(60, 192, 256, 1_000_000, 1_250_000),
        ]);
        let formula = build_symbolic_formula(&score).expect("formula");
        let all_same = evaluate_capability_atoms(&formula, &[5, 5, 5]).expect("all same");
        let middle_other = evaluate_capability_atoms(&formula, &[5, 6, 5]).expect("middle other");
        let delta = |atoms: &BTreeSet<ConcreteCapabilityAtom>| {
            atoms.iter().find_map(|atom| match atom {
                ConcreteCapabilityAtom::SameKey {
                    current_obligation_id: 2,
                    available_time_us,
                    ..
                } => Some(*available_time_us),
                _ => None,
            })
        };
        assert_eq!(delta(&all_same), Some(500_000));
        assert_eq!(delta(&middle_other), Some(1_000_000));
    }

    #[test]
    fn more_than_ten_concurrent_distinct_keys_has_no_v2_structural_space() {
        let events = (0..11)
            .map(|index| synthetic_event(48 + index, 0, 96, 0, 500_000))
            .collect::<Vec<_>>();
        let formula = build_symbolic_formula(&synthetic_score(events)).expect("formula");
        assert_eq!(formula.maximum_concurrent_variables, 11);
        assert!(!formula.structural_witness_found);
        assert!(formula.concurrent_clique_obstruction);
    }

    #[test]
    fn symbolic_formula_is_time_bound_and_independently_replayed() {
        let first = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 500_000),
            synthetic_event(60, 96, 192, 500_000, 1_000_000),
        ]);
        let second = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 400_000),
            synthetic_event(60, 96, 192, 400_000, 1_000_000),
        ]);
        let first_formula = build_symbolic_formula(&first).expect("first formula");
        let second_formula = build_symbolic_formula(&second).expect("second formula");
        assert_ne!(first_formula.formula_sha256, second_formula.formula_sha256);
        assert!(verify_symbolic_formula(&first, &first_formula).verified);
        assert!(verify_symbolic_formula(&second, &second_formula).verified);
    }

    #[test]
    fn eleven_distinct_keys_across_direct_boundary_prove_v2_structural_empty() {
        let score = synthetic_score(
            (0..6)
                .map(|offset| synthetic_event(48 + offset, 0, 96, 0, 500_000))
                .chain(
                    (0..5).map(|offset| {
                        synthetic_event(60 + offset, 96, 192, 500_000, 1_000_000)
                    }),
                )
                .collect(),
        );
        let formula = build_symbolic_formula(&score).expect("formula");
        assert_eq!(formula.maximum_concurrent_variables, 6);
        assert!(!formula.concurrent_clique_obstruction);
        assert_eq!(
            formula
                .direct_boundary_clique_proof
                .maximum_distinct_midi_clique,
            11
        );
        assert!(formula.direct_boundary_clique_obstruction);
        assert!(!formula.structural_witness_found);
        assert!(verify_symbolic_formula(&score, &formula).verified);
        let audit = audit_score(&score);
        assert_eq!(
            audit.structural_execution_space,
            "EMPTY_DIRECT_BOUNDARY_CLIQUE_GT10"
        );
    }

    #[test]
    fn smallest_last_structural_witness_closes_bad_order_crown_false_unresolved() {
        let variables = (0..22)
            .map(|id| SlotVariable {
                id,
                midi: 40 + i32::try_from(id).expect("id fits"),
                attack_tick: 0,
                release_tick: 1,
                obligation_ids: vec![id],
            })
            .collect::<Vec<_>>();
        let mut inequalities = Vec::new();
        for left_index in 0..11 {
            for right_index in 0..11 {
                if left_index != right_index {
                    inequalities.push((left_index * 2, right_index * 2 + 1));
                }
            }
        }
        inequalities.sort_unstable();
        inequalities.dedup();
        let proof = structural_coloring_witness(&variables, &inequalities).expect("proof");
        let coloring = proof.coloring.expect("ten-slot coloring");
        assert_eq!(proof.degeneracy, 10);
        assert!(coloring_satisfies_constraints(&coloring, &inequalities));
        assert!(coloring.iter().all(|color| *color < SLOT_COUNT));
    }

    #[test]
    fn scoreless_report_keeps_symbolic_contract_without_capability_answer() {
        let report = build_for_score_title(None);
        assert!(report.contains("UNMODELED remains an external symbolic variable"));
        assert!(report.contains("CapabilityValuation UNBOUND"));
        assert!(report.contains("PathSelection NONE"));
        assert!(report.contains("HumanPlayabilityClaim NOT_EVALUATED"));
        assert!(report.contains("r295 StaticConfigurationSignature"));
        assert!(report.contains("r296 previous-nonempty history rule"));
        assert!(report.contains("exact r619 rule"));
    }
}
