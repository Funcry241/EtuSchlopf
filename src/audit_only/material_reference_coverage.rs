//! Static temporal context and exact measurement coverage, without causal or human claims.
use crate::audit_only::notation_invariants::{material_attack_ticks, material_fragments};
use crate::audit_only::pianistic_execution_plan_witness::build_demand_topology;
use crate::audit_only::pianistic_execution_score::{printed_attack_positions, project_xml};
use crate::audit_only::pianistic_hand_configuration_reference_bracket::{
    ReferenceCell, reference_cells,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub(crate) const SECTION: &str = "MATERIAL-REFERENCE-COVERAGE";
pub(crate) const SECTION_RULE: &str = "---------------------------";
const CLASSES: [&str; 5] = [
    "R299_SUPPORTED",
    "STRUCTURAL_ONLY_SPAN_SENSITIVE",
    "UNRESOLVED",
    "NOT_MEASURED",
    "NOT_JOINABLE",
];
const GROUPS: [&str; 3] = [
    "hidden_master_attacks",
    "inactive_within_lineage",
    "removed_before_lineage",
];

pub(crate) struct MaterialPoint {
    pub(crate) source: usize,
    pub(crate) retained: bool,
    pub(crate) tick: Option<i64>,
}
fn cell_class(cell: &ReferenceCell) -> usize {
    if cell.r299 {
        0
    } else if cell.structural {
        1
    } else {
        2
    }
}
fn context_class(
    tick: Option<i64>,
    measured: bool,
    bound: bool,
    cells: &BTreeMap<i64, &ReferenceCell>,
) -> (usize, &'static str) {
    if !measured {
        return (3, "module_input_unavailable");
    }
    if !bound {
        return (4, "source_entry_time_binding_unavailable");
    }
    match tick.and_then(|t| cells.get(&t)) {
        Some(cell) => (cell_class(cell), "exact_existing_boundary"),
        None => (4, "no_exact_existing_boundary_or_time"),
    }
}
fn set_difference(expected: &BTreeSet<i64>, actual: &BTreeSet<i64>) -> (Vec<i64>, Vec<i64>) {
    (
        expected.difference(actual).copied().collect(),
        actual.difference(expected).copied().collect(),
    )
}
pub(crate) fn append_reference_coverage(
    out: &mut String,
    master: &str,
    active: usize,
    lineage: usize,
    canonical: usize,
    bound: bool,
    points: &[MaterialPoint],
) {
    out.push_str("ReferenceContextContract: final_in_memory_master; InactiveTime entry_notated_96_ticks_not_final_removed_position; Matching exact_existing_boundary_only; SourceIdentityForPrintedHeads not_claimed; Attribution temporal_context_only_not_cause; HumanPlayabilityClaim NOT_EVALUATED; TransitionTiming NOT_EVALUATED; MusicAuthority none.\n");
    out.push_str("ReferenceClassPartition: R299_SUPPORTED contained_in_structural_support; STRUCTURAL_ONLY_SPAN_SENSITIVE excludes_R299; UNRESOLVED is_not_human_impossibility; missing_input_and_missing_join_are_separate.\n");
    let score = project_xml("in-memory-master".into(), master);
    let cells_result = reference_cells(&score);
    let cells = cells_result.as_deref().unwrap_or(&[]);
    let measured = cells_result.is_ok();
    let reason = cells_result
        .as_ref()
        .err()
        .map_or("complete-reference-boundaries", String::as_str);
    let by_tick: BTreeMap<_, _> = cells.iter().map(|cell| (cell.tick, cell)).collect();
    let printed = printed_attack_positions(master);
    let entry_axis = material_attack_ticks(master);
    let entry_axis_equal = printed
        .as_ref()
        .ok()
        .zip(entry_axis.as_ref().ok())
        .is_some_and(|(a, b)| {
            let mut left = a.iter().map(|a| a.tick).collect::<Vec<_>>();
            let mut right = b.to_vec();
            left.sort_unstable();
            right.sort_unstable();
            left == right
        });
    let _ = writeln!(
        out,
        "EntryTimeAxisBinding: exact_master_attack_time_axes_equal {entry_axis_equal}; entry_axis_reason {:?}; source_authority_bound {bound}; inactive_time_context_requires_binding yes; same_key_overlap_is_separate_temporal_technique_demand.",
        entry_axis.as_ref().err()
    );
    let expected_ticks: BTreeSet<_> = printed
        .as_ref()
        .map(|p| p.iter().map(|a| a.tick).collect())
        .unwrap_or_default();
    let actual_ticks: BTreeSet<_> = cells
        .iter()
        .filter(|cell| cell.attacking)
        .map(|cell| cell.tick)
        .collect();
    let (missing, additional) = set_difference(&expected_ticks, &actual_ticks);
    let topology = build_demand_topology(&score);
    let expected_all = topology.as_ref().ok().map(|t| t.boundary_count);
    let evaluated = cells.iter().filter(|cell| cell.evaluated).count();
    let events_equal = bound && measured && score.execution_events.len() == active;
    let attacks_equal = measured && printed.is_ok() && missing.is_empty() && additional.is_empty();
    let boundaries_equal = measured
        && expected_all == Some(cells.len())
        && evaluated == cells.len()
        && by_tick.len() == cells.len();
    let _ = writeln!(
        out,
        "ExecutionCoverage: status {}; expected_active {active}; input_execution_events {}; event_coverage_equal {events_equal}; expected_attack_boundaries {}; measured_attack_boundaries {}; attack_boundary_sets_equal {attacks_equal}; expected_all_boundaries {:?}; measured_all_boundaries {}; evaluated_boundaries {evaluated}; all_boundary_coverage_equal {boundaries_equal}; InputReason {reason:?}.",
        if events_equal && attacks_equal && boundaries_equal {
            "COMPLETE"
        } else {
            "INCOMPLETE"
        },
        score.execution_events.len(),
        if printed.is_ok() {
            expected_ticks.len().to_string()
        } else {
            "unknown".to_string()
        },
        actual_ticks.len(),
        expected_all,
        cells.len()
    );
    let _ = writeln!(
        out,
        "ExecutionCoverageDifference: missing_attack_ticks {missing:?}; additional_attack_ticks {additional:?}; unit exact_96_quarter_ticks; all_boundaries_include_releases; input_events_before_exact_duplicate_coalescing; physical_obligations {:?}; printed_position_reason {:?}.",
        topology.as_ref().ok().map(|t| t.obligations.len()),
        printed.as_ref().err()
    );
    for cell in cells {
        let _ = writeln!(
            out,
            "StaticReferenceBoundary: tick {}; attacking {}; evaluated {}; class {}; reason {:?}.",
            cell.tick,
            cell.attacking,
            cell.evaluated,
            CLASSES[cell_class(cell)],
            cell.reason
        );
    }
    let mut matrix = [[0usize; 5]; 3];
    let hidden_ticks: Vec<Option<i64>> = match &printed {
        Ok(rows) => rows
            .iter()
            .filter(|a| !a.visible)
            .map(|a| Some(a.tick))
            .collect(),
        Err(_) => match material_fragments(master) {
            Ok(notes) => notes
                .iter()
                .filter(|a| !a.visible && !a.tie_stop)
                .map(|_| None)
                .collect(),
            Err(reason) => {
                let _ = writeln!(
                    out,
                    "MaterialReferenceMatrix: status MODEL_INPUT_UNAVAILABLE; hidden_count unknown; reason {reason:?}; no_zero_fallback."
                );
                return;
            }
        },
    };
    for (ordinal, tick) in hidden_ticks.iter().copied().enumerate() {
        let (class, join_reason) = context_class(tick, measured, true, &by_tick);
        matrix[0][class] += 1;
        let _ = writeln!(
            out,
            "MaterialReferenceContext: group {}; hidden_attack_ordinal {}; tick {tick:?}; class {}; reason {join_reason}; source_identity not_claimed.",
            GROUPS[0],
            ordinal + 1,
            CLASSES[class]
        );
    }
    let unique_sources: BTreeSet<_> = points.iter().map(|p| p.source).collect();
    let points_bound = bound && unique_sources.len() == points.len();
    for point in points {
        let group = if point.retained { 1 } else { 2 };
        let (class, join_reason) = context_class(
            point.tick,
            measured,
            points_bound && entry_axis_equal,
            &by_tick,
        );
        matrix[group][class] += 1;
        let _ = writeln!(
            out,
            "MaterialReferenceContext: group {}; source_ordinal {}; entry_tick {:?}; class {}; reason {join_reason}; final_removed_position not_claimed.",
            GROUPS[group], point.source, point.tick, CLASSES[class]
        );
    }
    let expected = [
        Some(hidden_ticks.len()),
        lineage.checked_sub(active),
        canonical.checked_sub(lineage),
    ];
    let mut complete = points_bound;
    for (group, row) in matrix.iter().enumerate() {
        let sum: usize = row.iter().sum();
        let equals = expected[group] == Some(sum);
        complete &= equals;
        let _ = writeln!(
            out,
            "MaterialReferenceRow: group {}; R299_SUPPORTED {}; STRUCTURAL_ONLY_SPAN_SENSITIVE {}; UNRESOLVED {}; NOT_MEASURED {}; NOT_JOINABLE {}; sum {sum}; expected {:?}; cardinality_equal {equals}.",
            GROUPS[group], row[0], row[1], row[2], row[3], row[4], expected[group]
        );
    }
    let total: usize = matrix.iter().flatten().sum();
    let _ = writeln!(
        out,
        "MaterialReferenceMatrix: population_rows_exact {complete}; total {total}; unknown_cells_retained yes; hidden_continuations excluded; shared_time_is_not_shared_source_identity."
    );
    let _ = writeln!(
        out,
        "StaticSupportedContext: R299_supported {}; structural_only_span_sensitive {}; interpretation temporal_context_not_visibility_or_removal_cause; no_transition_or_human_claim.",
        matrix.iter().map(|r| r[0]).sum::<usize>(),
        matrix.iter().map(|r| r[1]).sum::<usize>()
    );
}

#[cfg(test)]
#[path = "material_reference_coverage_tests.rs"]
mod tests;
