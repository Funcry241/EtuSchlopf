// Global horizontal system-reflow decision planner.
//
// The planner itself is writer-free. It observes final-notation facts and the
// fixed authoritative page boundaries, then computes a deterministic structural
// system partition. Promotion is fail-closed: only the system-break dimension
// may replace the baseline system partition, while widths and page breaks remain owned
// by that final plan and the existing one materializer remains the sole writer.

const GLOBAL_REFLOW_NOMINAL_MEASURE_DEMAND: usize = 100;
const GLOBAL_REFLOW_MAX_MEASURES_PER_SYSTEM: usize = 4;
const GLOBAL_REFLOW_TYPICAL_SYSTEM_MEASURES: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlobalHorizontalSystemReflowPlan {
    starts_new_system: Vec<bool>,
    starts_new_page: Vec<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct GlobalHorizontalSystemReflowAudit {
    slots_compared: usize,
    pages_compared: usize,
    baseline_systems: usize,
    reflow_systems: usize,
    baseline_singletons: usize,
    reflow_singletons: usize,
    baseline_three_measure_systems: usize,
    reflow_three_measure_systems: usize,
    baseline_four_measure_systems: usize,
    reflow_four_measure_systems: usize,
    system_break_changes: usize,
    page_break_changes: usize,
    added_system_breaks: usize,
    removed_system_breaks: usize,
    capacity_raised_pages: usize,
    forced_over_capacity_singletons: usize,
    nonterminal_pages_below_three_systems: usize,
    max_baseline_system_measures: usize,
    max_reflow_system_measures: usize,
    typical_system_capacity: usize,
    max_page_capacity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlobalReflowPartitionCandidate {
    system_lengths: Vec<usize>,
    systems: usize,
    singletons: usize,
    forced_over_capacity_singletons: usize,
    slack_square_sum: u128,
}

impl GlobalReflowPartitionCandidate {
    fn empty() -> Self {
        Self {
            system_lengths: Vec::new(),
            systems: 0,
            singletons: 0,
            forced_over_capacity_singletons: 0,
            slack_square_sum: 0,
        }
    }

    fn is_better_than(&self, other: &Self) -> bool {
        (
            self.systems,
            self.singletons,
            self.forced_over_capacity_singletons,
            self.slack_square_sum,
            &self.system_lengths,
        ) < (
            other.systems,
            other.singletons,
            other.forced_over_capacity_singletons,
            other.slack_square_sum,
            &other.system_lengths,
        )
    }
}

fn global_reflow_measure_demand(facts: &MeasureHorizontalLayoutFacts) -> usize {
    let (pressure_score, _) = visual_layout_pressure_score(facts);
    let clearance_reserve_per_edge = GLOBAL_REFLOW_NOMINAL_MEASURE_DEMAND
        .div_ceil(GLOBAL_REFLOW_TYPICAL_SYSTEM_MEASURES);
    GLOBAL_REFLOW_NOMINAL_MEASURE_DEMAND
        .saturating_add(pressure_score.max(0) as usize)
        .saturating_add(
            facts
                .following_accidental_clearance_edges
                .saturating_mul(clearance_reserve_per_edge),
        )
}

fn global_reflow_typical_system_capacity(demands: &[usize]) -> anyhow::Result<usize> {
    if demands.is_empty() {
        anyhow::bail!("GlobalHorizontalSystemReflowPlanner: keine finalen Demand-Slots");
    }
    let mut ordered = demands.to_vec();
    ordered.sort_unstable();
    let middle = ordered.len() / 2;
    let median = if ordered.len().is_multiple_of(2) {
        ordered[middle - 1]
            .saturating_add(ordered[middle])
            .div_ceil(2)
    } else {
        ordered[middle]
    };
    Ok(median.saturating_mul(GLOBAL_REFLOW_TYPICAL_SYSTEM_MEASURES))
}

fn global_reflow_page_partition(
    demands: &[usize],
    capacity: usize,
) -> anyhow::Result<GlobalReflowPartitionCandidate> {
    if demands.is_empty() {
        anyhow::bail!("GlobalHorizontalSystemReflowPlanner: leere Page-Partition");
    }

    let mut suffix: Vec<Option<GlobalReflowPartitionCandidate>> =
        vec![None; demands.len() + 1];
    suffix[demands.len()] = Some(GlobalReflowPartitionCandidate::empty());

    for start in (0..demands.len()).rev() {
        let max_len = GLOBAL_REFLOW_MAX_MEASURES_PER_SYSTEM.min(demands.len() - start);
        let mut best: Option<GlobalReflowPartitionCandidate> = None;
        let mut load = 0usize;

        for len in 1..=max_len {
            load = load.saturating_add(demands[start + len - 1]);
            if len > 1 && load > capacity {
                continue;
            }
            let Some(tail) = suffix[start + len].as_ref() else {
                continue;
            };

            let slack = capacity.saturating_sub(load) as u128;
            let mut system_lengths = Vec::with_capacity(tail.system_lengths.len() + 1);
            system_lengths.push(len);
            system_lengths.extend(tail.system_lengths.iter().copied());
            let candidate = GlobalReflowPartitionCandidate {
                system_lengths,
                systems: tail.systems + 1,
                singletons: tail.singletons + usize::from(len == 1),
                forced_over_capacity_singletons: tail.forced_over_capacity_singletons
                    + usize::from(len == 1 && load > capacity),
                slack_square_sum: tail
                    .slack_square_sum
                    .saturating_add(slack.saturating_mul(slack)),
            };

            if best
                .as_ref()
                .is_none_or(|current| candidate.is_better_than(current))
            {
                best = Some(candidate);
            }
        }
        suffix[start] = best;
    }

    suffix[0].clone().with_context(|| {
        "GlobalHorizontalSystemReflowPlanner: keine zulaessige strukturelle Page-Partition gefunden"
    })
}

fn global_reflow_page_starts(plan: &HorizontalLayoutPlan) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (slot_index, slot) in plan.slots.iter().enumerate().skip(1) {
        if slot.directives.starts_new_page {
            starts.push(slot_index);
        }
    }
    starts
}

fn global_reflow_baseline_system_lengths_for_page(
    plan: &HorizontalLayoutPlan,
    page_start: usize,
    page_end: usize,
) -> Vec<usize> {
    let mut starts = vec![page_start];
    for slot_index in (page_start + 1)..page_end {
        if plan.slots[slot_index].directives.starts_new_system {
            starts.push(slot_index);
        }
    }
    starts
        .iter()
        .enumerate()
        .map(|(index, start)| {
            starts
                .get(index + 1)
                .copied()
                .unwrap_or(page_end)
                .saturating_sub(*start)
        })
        .collect()
}

fn global_reflow_system_loads(demands: &[usize], lengths: &[usize]) -> Vec<usize> {
    let mut loads = Vec::with_capacity(lengths.len());
    let mut cursor = 0usize;
    for length in lengths {
        let end = cursor.saturating_add(*length).min(demands.len());
        loads.push(
            demands[cursor..end]
                .iter()
                .copied()
                .fold(0usize, usize::saturating_add),
        );
        cursor = end;
    }
    loads
}

fn build_global_horizontal_system_reflow(
    final_notation_xml: &str,
    baseline_plan: &HorizontalLayoutPlan,
) -> anyhow::Result<(
    GlobalHorizontalSystemReflowPlan,
    GlobalHorizontalSystemReflowAudit,
)> {
    let facts = HorizontalLayoutFactSnapshot::capture_final_notation(final_notation_xml)?;
    if facts.slots.len() != baseline_plan.slots.len() {
        anyhow::bail!(
            "GlobalHorizontalSystemReflowPlanner: finale Fact-Slots {} stimmen nicht mit Baseline-Plan-Slots {} ueberein",
            facts.slots.len(),
            baseline_plan.slots.len()
        );
    }

    let demands = facts
        .slots
        .iter()
        .map(global_reflow_measure_demand)
        .collect::<Vec<_>>();
    let typical_system_capacity = global_reflow_typical_system_capacity(&demands)?;
    let page_starts = global_reflow_page_starts(baseline_plan);
    let mut reflow_plan = GlobalHorizontalSystemReflowPlan {
        starts_new_system: vec![false; baseline_plan.slots.len()],
        starts_new_page: baseline_plan
            .slots
            .iter()
            .map(|slot| slot.directives.starts_new_page)
            .collect(),
    };
    let mut audit = GlobalHorizontalSystemReflowAudit {
        slots_compared: baseline_plan.slots.len(),
        pages_compared: page_starts.len(),
        typical_system_capacity,
        ..GlobalHorizontalSystemReflowAudit::default()
    };

    for (page_index, page_start) in page_starts.iter().copied().enumerate() {
        let page_end = page_starts
            .get(page_index + 1)
            .copied()
            .unwrap_or(baseline_plan.slots.len());
        if page_start >= page_end {
            anyhow::bail!(
                "GlobalHorizontalSystemReflowPlanner: ungueltige strukturelle Page-Grenze"
            );
        }

        let page_demands = &demands[page_start..page_end];
        let baseline_lengths =
            global_reflow_baseline_system_lengths_for_page(baseline_plan, page_start, page_end);
        let baseline_loads = global_reflow_system_loads(page_demands, &baseline_lengths);
        let baseline_capacity = baseline_loads.iter().copied().max().unwrap_or(0);
        let capacity = typical_system_capacity.max(baseline_capacity);
        if capacity > typical_system_capacity {
            audit.capacity_raised_pages += 1;
        }
        audit.max_page_capacity = audit.max_page_capacity.max(capacity);

        let selected = global_reflow_page_partition(page_demands, capacity)?;
        let mut cursor = page_start;
        for (system_index, length) in selected.system_lengths.iter().copied().enumerate() {
            if system_index > 0 {
                reflow_plan.starts_new_system[cursor] = true;
            }
            cursor = cursor.saturating_add(length);
        }
        if cursor != page_end {
            anyhow::bail!(
                "GlobalHorizontalSystemReflowPlanner: Reflow-Partition deckt Page-Slots nicht exakt ab"
            );
        }

        audit.baseline_systems += baseline_lengths.len();
        audit.reflow_systems += selected.system_lengths.len();
        audit.baseline_singletons += baseline_lengths.iter().filter(|length| **length == 1).count();
        audit.reflow_singletons += selected
            .system_lengths
            .iter()
            .filter(|length| **length == 1)
            .count();
        audit.baseline_three_measure_systems += baseline_lengths
            .iter()
            .filter(|length| **length == 3)
            .count();
        audit.reflow_three_measure_systems += selected
            .system_lengths
            .iter()
            .filter(|length| **length == 3)
            .count();
        audit.baseline_four_measure_systems += baseline_lengths
            .iter()
            .filter(|length| **length == 4)
            .count();
        audit.reflow_four_measure_systems += selected
            .system_lengths
            .iter()
            .filter(|length| **length == 4)
            .count();
        audit.forced_over_capacity_singletons += selected.forced_over_capacity_singletons;
        audit.max_baseline_system_measures = audit
            .max_baseline_system_measures
            .max(baseline_lengths.iter().copied().max().unwrap_or(0));
        audit.max_reflow_system_measures = audit
            .max_reflow_system_measures
            .max(selected.system_lengths.iter().copied().max().unwrap_or(0));
        if page_index + 1 < page_starts.len() && selected.system_lengths.len() < 3 {
            audit.nonterminal_pages_below_three_systems += 1;
        }
    }

    for slot_index in 1..baseline_plan.slots.len() {
        if reflow_plan.starts_new_page[slot_index] {
            continue;
        }
        let baseline_break = baseline_plan.slots[slot_index].directives.starts_new_system;
        let reflow_break = reflow_plan.starts_new_system[slot_index];
        if baseline_break != reflow_break {
            audit.system_break_changes += 1;
            if reflow_break {
                audit.added_system_breaks += 1;
            } else {
                audit.removed_system_breaks += 1;
            }
        }
    }
    audit.page_break_changes = baseline_plan
        .slots
        .iter()
        .enumerate()
        .filter(|(slot_index, baseline)| {
            baseline.directives.starts_new_page != reflow_plan.starts_new_page[*slot_index]
        })
        .count();

    Ok((reflow_plan, audit))
}

// Productive system-only promotion was retired by the joint Page/System
// preference cutover. This module remains the read-only source of demand,
// capacity and structural-system evidence used by the joint planner.


#[cfg(test)]
mod horizontal_layout_global_reflow_tests {
    use super::{
        GLOBAL_REFLOW_NOMINAL_MEASURE_DEMAND, GLOBAL_REFLOW_TYPICAL_SYSTEM_MEASURES,
        HorizontalLayoutFactSnapshot, global_reflow_measure_demand, global_reflow_page_partition,
    };

    #[test]
    fn final_notation_clearance_edge_adds_model_scaled_reserve_without_new_literal_weight() {
        let control = HorizontalLayoutFactSnapshot::capture_final_notation(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n            <note><pitch><step>B</step><alter>-1</alter><octave>3</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>\n            <note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>\n            <note><pitch><step>B</step><alter>-1</alter><octave>3</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>\n            </measure></part></score-partwise>",
        )
        .expect("control facts");
        let clearance = HorizontalLayoutFactSnapshot::capture_final_notation(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\">\n            <note><pitch><step>B</step><alter>-1</alter><octave>3</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>\n            <note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>\n            <note><pitch><step>B</step><alter>-1</alter><octave>3</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff><accidental>flat</accidental></note>\n            </measure></part></score-partwise>",
        )
        .expect("clearance facts");

        let control_demand = global_reflow_measure_demand(&control.slots[0]);
        let clearance_demand = global_reflow_measure_demand(&clearance.slots[0]);
        assert_eq!(control.slots[0].following_accidental_clearance_edges, 0);
        assert_eq!(clearance.slots[0].following_accidental_clearance_edges, 1);
        assert_eq!(
            clearance_demand.saturating_sub(control_demand),
            GLOBAL_REFLOW_NOMINAL_MEASURE_DEMAND
                .div_ceil(GLOBAL_REFLOW_TYPICAL_SYSTEM_MEASURES)
        );
    }

    #[test]
    fn balanced_five_slot_page_avoids_a_singleton_when_three_slots_fit() {
        let selected = global_reflow_page_partition(&[100, 100, 100, 100, 100], 300)
            .expect("reflow partition");
        assert_eq!(selected.system_lengths.len(), 2);
        assert_eq!(selected.singletons, 0);
        assert_eq!(selected.system_lengths.iter().sum::<usize>(), 5);
    }

    #[test]
    fn dense_three_slot_group_is_split_without_exceeding_capacity() {
        let selected = global_reflow_page_partition(&[170, 170, 100], 300)
            .expect("reflow partition");
        assert_eq!(selected.system_lengths, vec![1, 2]);
        assert_eq!(selected.forced_over_capacity_singletons, 0);
    }

    #[test]
    fn dense_single_slot_remains_representable_without_hiding_overflow() {
        let selected = global_reflow_page_partition(&[340], 300).expect("reflow partition");
        assert_eq!(selected.system_lengths, vec![1]);
        assert_eq!(selected.forced_over_capacity_singletons, 1);
    }
}
