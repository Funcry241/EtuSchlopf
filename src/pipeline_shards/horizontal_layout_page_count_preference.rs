// Side-effect-free page-count preference frontier and selection.
//
// The base auto page count is retained only as a preference. This planner
// evaluates page counts at or below that preference against final-notation
// demand, accepted system capacity, the structural system-length bound and the
// observed vertical systems-per-page ceiling. It also computes the validated
// balance-first FullFrontier and the deterministic PreferenceSelection.
//
// This module owns no writer. The selected candidate may be promoted only by
// the downstream joint Page/System cutover after exact readback of the same
// hard frontier invariants.

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageCountPreferenceCandidate {
    pages: usize,
    page_slot_lengths: Vec<usize>,
    page_system_lengths: Vec<Vec<usize>>,
    systems: usize,
    singletons: usize,
    forced_over_capacity_singletons: usize,
    system_count_balance_penalty: u128,
    page_slot_balance_penalty: u128,
    slack_square_sum: u128,
    max_system_load: usize,
}

impl PageCountPreferenceCandidate {
    fn empty(pages: usize) -> Self {
        Self {
            pages,
            page_slot_lengths: Vec::new(),
            page_system_lengths: Vec::new(),
            systems: 0,
            singletons: 0,
            forced_over_capacity_singletons: 0,
            system_count_balance_penalty: 0,
            page_slot_balance_penalty: 0,
            slack_square_sum: 0,
            max_system_load: 0,
        }
    }

    fn is_better_than(&self, other: &Self) -> bool {
        (
            self.systems,
            self.singletons,
            self.forced_over_capacity_singletons,
            self.system_count_balance_penalty,
            self.page_slot_balance_penalty,
            self.slack_square_sum,
            &self.page_slot_lengths,
            &self.page_system_lengths,
        ) < (
            other.systems,
            other.singletons,
            other.forced_over_capacity_singletons,
            other.system_count_balance_penalty,
            other.page_slot_balance_penalty,
            other.slack_square_sum,
            &other.page_slot_lengths,
            &other.page_system_lengths,
        )
    }

    fn is_thin_first_better_than(&self, other: &Self) -> bool {
        (
            self.systems,
            self.singletons,
            self.forced_over_capacity_singletons,
            self.nonterminal_thin_pages(),
            self.system_count_balance_penalty,
            self.page_slot_balance_penalty,
            self.slack_square_sum,
            &self.page_slot_lengths,
            &self.page_system_lengths,
        ) < (
            other.systems,
            other.singletons,
            other.forced_over_capacity_singletons,
            other.nonterminal_thin_pages(),
            other.system_count_balance_penalty,
            other.page_slot_balance_penalty,
            other.slack_square_sum,
            &other.page_slot_lengths,
            &other.page_system_lengths,
        )
    }

    fn min_page_slots(&self) -> usize {
        self.page_slot_lengths.iter().copied().min().unwrap_or(0)
    }

    fn max_page_slots(&self) -> usize {
        self.page_slot_lengths.iter().copied().max().unwrap_or(0)
    }

    fn min_systems_per_page(&self) -> usize {
        self.page_system_lengths.iter().map(Vec::len).min().unwrap_or(0)
    }

    fn max_systems_per_page(&self) -> usize {
        self.page_system_lengths.iter().map(Vec::len).max().unwrap_or(0)
    }

    fn nonterminal_thin_pages(&self) -> usize {
        self.page_system_lengths
            .iter()
            .enumerate()
            .filter(|entry| entry.0.saturating_add(1) < self.pages && entry.1.len() < 3)
            .count()
    }

    fn combinatorial_thin_lower_bound(&self) -> usize {
        let nonterminal_pages = self.pages.saturating_sub(1);
        let upgrade_budget = self.systems.saturating_sub(self.pages) / 2;
        nonterminal_pages.saturating_sub(upgrade_budget.min(nonterminal_pages))
    }

    fn avoidable_thin_pages(&self) -> usize {
        self.nonterminal_thin_pages()
            .saturating_sub(self.combinatorial_thin_lower_bound())
    }

    fn is_preference_better_than(&self, other: &Self, preferred_pages: usize) -> bool {
        (
            self.systems,
            self.singletons,
            self.forced_over_capacity_singletons,
            self.nonterminal_thin_pages(),
            self.avoidable_thin_pages(),
            self.pages.abs_diff(preferred_pages),
            self.system_count_balance_penalty,
            self.page_slot_balance_penalty,
            self.slack_square_sum,
            &self.page_slot_lengths,
            &self.page_system_lengths,
        ) < (
            other.systems,
            other.singletons,
            other.forced_over_capacity_singletons,
            other.nonterminal_thin_pages(),
            other.avoidable_thin_pages(),
            other.pages.abs_diff(preferred_pages),
            other.system_count_balance_penalty,
            other.page_slot_balance_penalty,
            other.slack_square_sum,
            &other.page_slot_lengths,
            &other.page_system_lengths,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlobalHorizontalPageCountPreferenceAudit {
    slots_compared: usize,
    preferred_pages: usize,
    first_tested_pages: usize,
    last_tested_pages: usize,
    feasible_page_counts: Vec<usize>,
    feasible_candidates: Vec<PageCountPreferenceCandidate>,
    thin_first_candidates: Vec<PageCountPreferenceCandidate>,
    selected_preference_candidate: Option<PageCountPreferenceCandidate>,
    lowest_feasible_pages: usize,
    closest_lower_feasible_pages: usize,
    preferred_pages_feasible: bool,
    vertical_max_systems_per_page: usize,
    max_system_measures: usize,
    accepted_system_capacity: usize,
    horizontal_minimum_systems: usize,
    closest_lower_systems: usize,
    closest_lower_singletons: usize,
    closest_lower_min_page_slots: usize,
    closest_lower_max_page_slots: usize,
    closest_lower_min_systems_per_page: usize,
    closest_lower_max_systems_per_page: usize,
    closest_lower_max_system_load: usize,
    closest_lower_forced_over_capacity_singletons: usize,
}

fn page_count_preference_candidate_with_order(
    demands: &[usize],
    pages: usize,
    accepted_system_capacity: usize,
    max_system_measures: usize,
    vertical_max_systems_per_page: usize,
    reference_systems: usize,
    thin_first: bool,
) -> anyhow::Result<Option<PageCountPreferenceCandidate>> {
    if demands.is_empty()
        || pages == 0
        || max_system_measures == 0
        || vertical_max_systems_per_page == 0
    {
        anyhow::bail!("GlobalHorizontalPageCountPreference: ungueltige Candidate-Basis");
    }

    let slots = demands.len();
    let mut dp: Vec<Vec<Option<PageCountPreferenceCandidate>>> =
        vec![vec![None; slots + 1]; pages + 1];
    dp[0][0] = Some(PageCountPreferenceCandidate::empty(pages));

    for page_index in 0..pages {
        for start in 0..=slots {
            let Some(prefix) = dp[page_index][start].clone() else {
                continue;
            };
            let remaining_pages_after = pages - page_index - 1;
            let min_end = start.saturating_add(1);
            let max_end = slots.saturating_sub(remaining_pages_after);
            if min_end > max_end {
                continue;
            }

            for end in min_end..=max_end {
                let segment = joint_page_system_segment_partition(
                    &demands[start..end],
                    accepted_system_capacity,
                    max_system_measures,
                )?;
                if segment.systems > vertical_max_systems_per_page {
                    continue;
                }

                let mut candidate = prefix.clone();
                candidate.systems = candidate.systems.saturating_add(segment.systems);
                candidate.singletons = candidate.singletons.saturating_add(segment.singletons);
                candidate.forced_over_capacity_singletons = candidate
                    .forced_over_capacity_singletons
                    .saturating_add(segment.forced_over_capacity_singletons);
                let system_delta = segment
                    .systems
                    .saturating_mul(pages)
                    .abs_diff(reference_systems) as u128;
                candidate.system_count_balance_penalty = candidate
                    .system_count_balance_penalty
                    .saturating_add(system_delta.saturating_mul(system_delta));
                let page_slots = end - start;
                let slot_delta = page_slots.saturating_mul(pages).abs_diff(slots) as u128;
                candidate.page_slot_balance_penalty = candidate
                    .page_slot_balance_penalty
                    .saturating_add(slot_delta.saturating_mul(slot_delta));
                candidate.slack_square_sum = candidate
                    .slack_square_sum
                    .saturating_add(segment.slack_square_sum);
                candidate.max_system_load = candidate.max_system_load.max(segment.max_system_load);
                candidate.page_slot_lengths.push(page_slots);
                candidate.page_system_lengths.push(segment.system_lengths);

                let replace = dp[page_index + 1][end].as_ref().is_none_or(|current| {
                    if thin_first {
                        candidate.is_thin_first_better_than(current)
                    } else {
                        candidate.is_better_than(current)
                    }
                });
                if replace {
                    dp[page_index + 1][end] = Some(candidate);
                }
            }
        }
    }

    Ok(dp[pages][slots].clone())
}

fn page_count_preference_candidate(
    demands: &[usize],
    pages: usize,
    accepted_system_capacity: usize,
    max_system_measures: usize,
    vertical_max_systems_per_page: usize,
    reference_systems: usize,
) -> anyhow::Result<Option<PageCountPreferenceCandidate>> {
    page_count_preference_candidate_with_order(
        demands,
        pages,
        accepted_system_capacity,
        max_system_measures,
        vertical_max_systems_per_page,
        reference_systems,
        false,
    )
}

fn page_count_preference_thin_first_candidate(
    demands: &[usize],
    pages: usize,
    accepted_system_capacity: usize,
    max_system_measures: usize,
    vertical_max_systems_per_page: usize,
    reference_systems: usize,
) -> anyhow::Result<Option<PageCountPreferenceCandidate>> {
    page_count_preference_candidate_with_order(
        demands,
        pages,
        accepted_system_capacity,
        max_system_measures,
        vertical_max_systems_per_page,
        reference_systems,
        true,
    )
}

fn evaluate_global_horizontal_page_count_preference(
    final_notation_xml: &str,
    baseline_plan: &HorizontalLayoutPlan,
    global_reflow_audit: &GlobalHorizontalSystemReflowAudit,
) -> anyhow::Result<GlobalHorizontalPageCountPreferenceAudit> {
    let facts = HorizontalLayoutFactSnapshot::capture_final_notation(final_notation_xml)?;
    if facts.slots.len() != baseline_plan.slots.len()
        || global_reflow_audit.slots_compared != baseline_plan.slots.len()
    {
        anyhow::bail!(
            "GlobalHorizontalPageCountPreference: finale Facts, Baseline-Plan und Reflow-Audit besitzen keine exakte Slotbindung"
        );
    }

    let demands = facts
        .slots
        .iter()
        .map(global_reflow_measure_demand)
        .collect::<Vec<_>>();
    let authoritative_shape = authoritative_page_shape(baseline_plan)?;
    let preferred_pages = authoritative_shape.page_starts.len();
    let vertical_max_systems_per_page = authoritative_shape.max_systems_per_page;
    let max_system_measures = global_reflow_audit.max_reflow_system_measures.max(1);
    let accepted_system_capacity = global_reflow_audit
        .typical_system_capacity
        .max(global_reflow_audit.max_page_capacity);
    let reference_systems = global_reflow_audit.reflow_systems.max(1);
    let horizontal_minimum_systems = joint_page_system_segment_partition(
        &demands,
        accepted_system_capacity,
        max_system_measures,
    )?
    .systems
    .max(1);
    let first_tested_pages = horizontal_minimum_systems
        .div_ceil(vertical_max_systems_per_page.max(1))
        .max(1)
        .min(preferred_pages);

    let mut feasible_candidates = Vec::new();
    for pages in first_tested_pages..=preferred_pages {
        if let Some(candidate) = page_count_preference_candidate(
            &demands,
            pages,
            accepted_system_capacity,
            max_system_measures,
            vertical_max_systems_per_page,
            reference_systems,
        )?
            && candidate.forced_over_capacity_singletons == 0 {
                feasible_candidates.push(candidate);
            }
    }

    let mut thin_first_candidates = Vec::new();
    for pages in first_tested_pages..=preferred_pages {
        if let Some(candidate) = page_count_preference_thin_first_candidate(
            &demands,
            pages,
            accepted_system_capacity,
            max_system_measures,
            vertical_max_systems_per_page,
            reference_systems,
        )?
            && candidate.forced_over_capacity_singletons == 0 {
                thin_first_candidates.push(candidate);
            }
    }

    let feasible_page_counts = feasible_candidates
        .iter()
        .map(|candidate| candidate.pages)
        .collect::<Vec<_>>();
    let thin_first_page_counts = thin_first_candidates
        .iter()
        .map(|candidate| candidate.pages)
        .collect::<Vec<_>>();
    if thin_first_page_counts != feasible_page_counts {
        anyhow::bail!(
            "GlobalHorizontalPageCountPreference: Thin-first-Gegenfaktum driftet von der Feasibility-Frontier"
        );
    }
    let selected_preference_candidate =
        feasible_candidates
            .iter()
            .cloned()
            .reduce(|best, candidate| {
                if candidate.is_preference_better_than(&best, preferred_pages) {
                    candidate
                } else {
                    best
                }
            });
    if selected_preference_candidate.as_ref().is_some_and(|candidate| {
        candidate.forced_over_capacity_singletons != 0
            || !feasible_page_counts.contains(&candidate.pages)
    }) {
        anyhow::bail!(
            "GlobalHorizontalPageCountPreference: Preference-Auswahl driftet von der harten Feasibility-Frontier"
        );
    }

    let lowest_feasible_pages = feasible_page_counts.first().copied().unwrap_or(0);
    let preferred_pages_feasible = feasible_page_counts.contains(&preferred_pages);
    let closest_lower = feasible_candidates
        .iter()
        .rev()
        .find(|candidate| candidate.pages < preferred_pages);

    Ok(GlobalHorizontalPageCountPreferenceAudit {
        slots_compared: baseline_plan.slots.len(),
        preferred_pages,
        first_tested_pages,
        last_tested_pages: preferred_pages,
        feasible_page_counts,
        feasible_candidates: feasible_candidates.clone(),
        thin_first_candidates,
        selected_preference_candidate,
        lowest_feasible_pages,
        closest_lower_feasible_pages: closest_lower.map(|candidate| candidate.pages).unwrap_or(0),
        preferred_pages_feasible,
        vertical_max_systems_per_page,
        max_system_measures,
        accepted_system_capacity,
        horizontal_minimum_systems,
        closest_lower_systems: closest_lower.map(|candidate| candidate.systems).unwrap_or(0),
        closest_lower_singletons: closest_lower.map(|candidate| candidate.singletons).unwrap_or(0),
        closest_lower_min_page_slots: closest_lower
            .map(PageCountPreferenceCandidate::min_page_slots)
            .unwrap_or(0),
        closest_lower_max_page_slots: closest_lower
            .map(PageCountPreferenceCandidate::max_page_slots)
            .unwrap_or(0),
        closest_lower_min_systems_per_page: closest_lower
            .map(PageCountPreferenceCandidate::min_systems_per_page)
            .unwrap_or(0),
        closest_lower_max_systems_per_page: closest_lower
            .map(PageCountPreferenceCandidate::max_systems_per_page)
            .unwrap_or(0),
        closest_lower_max_system_load: closest_lower
            .map(|candidate| candidate.max_system_load)
            .unwrap_or(0),
        closest_lower_forced_over_capacity_singletons: closest_lower
            .map(|candidate| candidate.forced_over_capacity_singletons)
            .unwrap_or(0),
    })
}

#[cfg(test)]
mod horizontal_layout_page_count_preference_tests {
    use super::{page_count_preference_candidate, PageCountPreferenceCandidate};

    #[test]
    fn vertical_system_ceiling_can_make_lower_page_count_infeasible() {
        let demands = vec![150usize; 18];
        let three_pages = page_count_preference_candidate(&demands, 3, 504, 3, 2, 6)
            .expect("page-count preference");
        assert!(three_pages.is_some());
        let two_pages = page_count_preference_candidate(&demands, 2, 504, 3, 2, 6)
            .expect("page-count preference");
        assert!(two_pages.is_none());
    }

    #[test]
    fn thin_page_metric_is_evidence_only_and_excludes_terminal_page() {
        let candidate = page_count_preference_candidate(&[150usize; 6], 2, 504, 3, 4, 2)
            .expect("page-count preference")
            .expect("two-page candidate");
        assert_eq!(candidate.nonterminal_thin_pages(), 1);
    }

    #[test]
    fn partial_candidate_counts_all_already_fixed_nonterminal_pages() {
        let mut candidate = PageCountPreferenceCandidate::empty(3);
        candidate.page_system_lengths = vec![vec![2, 2], vec![2, 2]];
        candidate.systems = 4;
        assert_eq!(candidate.nonterminal_thin_pages(), 2);
        assert_eq!(candidate.combinatorial_thin_lower_bound(), 2);
    }

    #[test]
    fn thin_first_order_can_trade_balance_for_fewer_thin_pages() {
        let mut balance_first = PageCountPreferenceCandidate::empty(3);
        balance_first.page_slot_lengths = vec![3, 3, 3];
        balance_first.page_system_lengths = vec![vec![2, 1], vec![2, 1, 1], vec![2, 1, 1]];
        balance_first.systems = 8;
        balance_first.system_count_balance_penalty = 0;

        let mut thin_first = PageCountPreferenceCandidate::empty(3);
        thin_first.page_slot_lengths = vec![3, 3, 3];
        thin_first.page_system_lengths = vec![vec![1, 1, 1], vec![1, 1, 1], vec![2, 1]];
        thin_first.systems = 8;
        thin_first.system_count_balance_penalty = 100;

        assert!(balance_first.is_better_than(&thin_first));
        assert!(thin_first.is_thin_first_better_than(&balance_first));
    }

    #[test]
    fn preference_reduces_thin_pages_before_page_count_distance() {
        let mut preferred = PageCountPreferenceCandidate::empty(6);
        preferred.page_slot_lengths = vec![5; 6];
        preferred.page_system_lengths = vec![vec![2, 1]; 6];
        preferred.systems = 12;

        let mut lower = PageCountPreferenceCandidate::empty(4);
        lower.page_slot_lengths = vec![7, 7, 7, 9];
        lower.page_system_lengths = vec![vec![2, 1, 1]; 4];
        lower.systems = 12;

        assert!(lower.is_preference_better_than(&preferred, 6));
    }

    #[test]
    fn preference_keeps_preferred_pages_when_vertical_quality_is_equal() {
        let mut preferred = PageCountPreferenceCandidate::empty(6);
        preferred.page_slot_lengths = vec![5; 6];
        preferred.page_system_lengths = vec![vec![2, 1, 1]; 6];
        preferred.systems = 18;

        let mut lower = PageCountPreferenceCandidate::empty(5);
        lower.page_slot_lengths = vec![6; 5];
        lower.page_system_lengths = vec![vec![2, 1, 1], vec![2, 1, 1, 1], vec![2, 1, 1, 1], vec![2, 1, 1], vec![2, 1, 1, 1]];
        lower.systems = 18;

        assert!(preferred.is_preference_better_than(&lower, 6));
    }
}
