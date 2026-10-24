// Run-local complete-partition exclusions. No range/capacity inference.
// The caller holds the immutable source and stable actual consumer binding.
type RendererRejectedPartitions = std::collections::BTreeSet<Vec<Vec<usize>>>;
type RendererSegmentState = (usize, Vec<usize>);
type RendererSegmentAlternatives = std::collections::BTreeMap<RendererSegmentState, JointPageSystemSegmentPartition>;
type RendererJointStates = std::collections::BTreeMap<Vec<usize>, JointPageSystemCandidate>;

struct RendererSegmentSearch<'a> {
    demands: &'a [usize],
    clearance: &'a [usize],
    start: usize,
    page: usize,
    patterns: &'a [&'a Vec<Vec<usize>>],
    matching: &'a [usize],
    request: &'a RendererFeasibilityFrontierRequest<'a>,
}

fn renderer_segment_alternatives(search: &RendererSegmentSearch<'_>) -> RendererSegmentAlternatives {
    let mut dp = vec![RendererSegmentAlternatives::new(); search.demands.len() + 1];
    dp[0].insert((0, search.matching.to_vec()), JointPageSystemSegmentPartition::empty());
    for offset in 0..search.demands.len() {
        for ((count, matching), prefix) in dp[offset].clone() {
            if count >= search.request.vertical_max_systems_per_page { continue; }
            let mut load = 0usize;
            for length in 1..=search.request.max_system_measures.min(search.demands.len() - offset) {
                load = load.saturating_add(search.demands[offset + length - 1]);
                if length > 1 && load > search.request.accepted_system_capacity { continue; }
                if !renderer_clearance_system_has_structural_slack(
                    &search.clearance[offset..offset + length], length, search.request.max_system_measures,
                ) { continue; }
                let start = search.start + offset;
                if search.request.no_goods.contains(&(start, start + length - 1)) { continue; }
                let remaining = matching.iter().copied().filter(|&index|
                    search.patterns[index][search.page].get(count) == Some(&length)).collect();
                let mut part = prefix.clone();
                part.system_lengths.push(length);
                part.systems += 1;
                part.singletons += usize::from(length == 1);
                part.forced_over_capacity_singletons += usize::from(length == 1 && load > search.request.accepted_system_capacity);
                let slack = search.request.accepted_system_capacity.saturating_sub(load) as u128;
                part.slack_square_sum = part.slack_square_sum.saturating_add(slack.saturating_mul(slack));
                part.max_system_load = part.max_system_load.max(load);
                let key = (count + 1, remaining);
                let cell = &mut dp[offset + length];
                if cell.get(&key).is_none_or(|old| part.is_better_than(old)) { cell.insert(key, part); }
            }
        }
    }
    let mut alternatives = RendererSegmentAlternatives::new();
    for ((count, matching), part) in dp.pop().unwrap_or_default() {
        let remaining = matching.into_iter().filter(|&index|
            search.patterns[index][search.page].len() == count).collect();
        let key = (count, remaining);
        if alternatives.get(&key).is_none_or(|old| part.is_better_than(old)) { alternatives.insert(key, part); }
    }
    alternatives
}

fn renderer_exact_partition_candidate(
    request: &RendererFeasibilityFrontierRequest<'_>, pages: usize,
) -> anyhow::Result<JointPageSystemCandidate> {
    let slots = request.measure_demands.len();
    if request.rejected_partitions.iter().any(|plan|
        plan.is_empty() || plan.iter().any(|page| page.is_empty() || page.contains(&0))
            || plan.iter().flatten().try_fold(0usize, |sum, &length| sum.checked_add(length)) != Some(slots)) {
        anyhow::bail!("RendererFeasibilityExactPartition: invalid complete exclusion");
    }
    let patterns = request.rejected_partitions.iter().filter(|plan| plan.len() == pages).collect::<Vec<_>>();
    let mut dp = vec![vec![RendererJointStates::new(); slots + 1]; pages + 1];
    dp[0][0].insert((0..patterns.len()).collect(), JointPageSystemCandidate::empty());
    for page in 0..pages {
        for start in 0..slots {
            for (matching, prefix) in dp[page][start].clone() {
                let remaining_pages = pages - page - 1;
                for (end, cell) in dp[page + 1].iter_mut().enumerate()
                    .take(slots.saturating_sub(remaining_pages) + 1).skip(start + 1) {
                    let alternatives = renderer_segment_alternatives(&RendererSegmentSearch {
                        demands: &request.measure_demands[start..end],
                        clearance: &request.measure_clearance_edges[start..end],
                        start: start + 1, page, patterns: &patterns, matching: &matching, request,
                    });
                    for ((_, remaining), part) in alternatives {
                        let mut joint = prefix.clone();
                        joint.systems = joint.systems.saturating_add(part.systems);
                        joint.singletons = joint.singletons.saturating_add(part.singletons);
                        joint.forced_over_capacity_singletons = joint.forced_over_capacity_singletons.saturating_add(part.forced_over_capacity_singletons);
                        let delta = part.systems.saturating_mul(pages).abs_diff(request.reference_systems) as u128;
                        joint.system_count_balance_penalty = joint.system_count_balance_penalty.saturating_add(delta.saturating_mul(delta));
                        let page_slots = end - start;
                        let delta = page_slots.saturating_mul(pages).abs_diff(slots) as u128;
                        joint.page_slot_balance_penalty = joint.page_slot_balance_penalty.saturating_add(delta.saturating_mul(delta));
                        joint.slack_square_sum = joint.slack_square_sum.saturating_add(part.slack_square_sum);
                        joint.max_system_load = joint.max_system_load.max(part.max_system_load);
                        joint.page_slot_lengths.push(page_slots);
                        joint.page_system_lengths.push(part.system_lengths);
                        if cell.get(&remaining).is_none_or(|old| joint.is_better_than(old)) { cell.insert(remaining, joint); }
                    }
                }
            }
        }
    }
    dp[pages][slots].get(&Vec::new()).cloned().ok_or_else(|| anyhow::anyhow!(
        "RendererFeasibilityBoundedJointPlanCutover: keine zulaessige gemeinsame Page/System-Partition"))
}

#[cfg(test)]
#[path = "../../tests/unit/renderer_exact_partition_search.rs"]
mod renderer_exact_partition_search_tests;

// Lifetime-bound to one immutable oracle source; never persisted or shared.
struct RendererPartitionRejections<'a, T> {
    source: &'a str,
    consumer: Option<T>,
    partitions: RendererRejectedPartitions,
}

impl<'a, T: Clone + Eq> RendererPartitionRejections<'a, T> {
    fn new(source: &'a str) -> Self { Self { source, consumer: None, partitions: RendererRejectedPartitions::new() } }

    fn bind(&mut self, source: &str, consumer: Option<&T>) -> anyhow::Result<()> {
        anyhow::ensure!(self.source == source, "RendererFeasibilityExactPartition: foreign committed source");
        let consumer = consumer.ok_or_else(|| anyhow::anyhow!("RendererFeasibilityExactPartition: missing actual consumer binding"))?;
        if let Some(previous) = &self.consumer {
            anyhow::ensure!(previous == consumer, "RendererFeasibilityExactPartition: consumer environment changed");
        } else { self.consumer = Some(consumer.clone()); }
        Ok(())
    }

    fn record(&mut self, plan: &HorizontalLayoutPlan,
        observation: &crate::audit_only::renderer_system_break_readback::RendererSystemBreakReadbackAudit,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(self.consumer.is_some(), "RendererFeasibilityExactPartition: unbound observation");
        let pages = plan.slots.iter().enumerate().filter_map(|(index, slot)|
            (index == 0 || slot.directives.starts_new_page).then_some(index + 1)).collect::<Vec<_>>();
        let systems = plan.slots.iter().enumerate().filter_map(|(index, slot)|
            (index == 0 || slot.directives.starts_new_page || slot.directives.starts_new_system).then_some(index + 1)).collect::<Vec<_>>();
        anyhow::ensure!(observation.measure_count == plan.slots.len()
            && observation.planned_page_starts == pages && observation.planned_system_starts == systems,
            "RendererFeasibilityExactPartition: observation does not bind submitted plan");
        anyhow::ensure!(!observation.inserted_page_breaks.is_empty()
            && observation.collapsed_page_breaks.is_empty() && observation.collapsed_system_breaks.is_empty(),
            "RendererFeasibilityExactPartition: unsupported partition observation");
        let partition = authoritative_page_shape(plan)?.page_system_lengths;
        anyhow::ensure!(self.partitions.insert(partition), "RendererFeasibilityExactPartition: repeated rejected complete plan");
        Ok(())
    }

    fn validate(&self, plan: &HorizontalLayoutPlan) -> anyhow::Result<()> {
        anyhow::ensure!(!self.partitions.contains(&authoritative_page_shape(plan)?.page_system_lengths),
            "RendererFeasibilityExactPartition: promoted plan repeats complete exclusion");
        Ok(())
    }
}

// Both observations belong to one immutable-source oracle invocation. The
// inspected fresh-import consumer forms systems before vertical page packing.
// Only page/system flags vary; all musical, style, section, frame and courtesy
// context stays in the original source, and per-measure widths stay unchanged.
impl<T: Clone + Eq> RendererPartitionRejections<'_, T> {
    fn system_ranges(
        &self, initial: &HorizontalLayoutPlan, plan: &HorizontalLayoutPlan,
        observation: &crate::audit_only::renderer_system_break_readback::RendererSystemBreakReadbackAudit,
    ) -> anyhow::Result<std::collections::BTreeSet<(usize, usize)>> {
        anyhow::ensure!(self.consumer.is_some(), "RendererHorizontalNonrealization: unbound actual consumer");
        self.validate_horizontal_context(initial, plan)?;
        let systems = plan.slots.iter().enumerate().filter_map(|(index, slot)|
            (index == 0 || slot.directives.starts_new_page || slot.directives.starts_new_system).then_some(index + 1)).collect::<Vec<_>>();
        let pages = plan.slots.iter().enumerate().filter_map(|(index, slot)|
            (index == 0 || slot.directives.starts_new_page).then_some(index + 1)).collect::<Vec<_>>();
        let valid_starts = |starts: &[usize]| starts.first() == Some(&1)
            && starts.windows(2).all(|pair| pair[0] < pair[1])
            && starts.last().is_some_and(|last| *last <= plan.slots.len());
        let difference = |left: &[usize], right: &[usize]| left.iter().copied()
            .filter(|value| !right.contains(value)).collect::<Vec<_>>();
        anyhow::ensure!(observation.measure_count == plan.slots.len()
            && observation.planned_system_starts == systems && observation.planned_page_starts == pages
            && valid_starts(&observation.rendered_system_starts) && valid_starts(&observation.rendered_page_starts)
            && observation.pdf_pages == observation.rendered_page_starts.len()
            && observation.rendered_page_starts.iter().all(|value| observation.rendered_system_starts.contains(value))
            && observation.inserted_system_breaks == difference(&observation.rendered_system_starts, &systems)
            && observation.inserted_page_breaks == difference(&observation.rendered_page_starts, &pages)
            && difference(&systems, &observation.rendered_system_starts).is_empty()
            && difference(&pages, &observation.rendered_page_starts).is_empty()
            && observation.collapsed_system_breaks.is_empty() && observation.collapsed_page_breaks.is_empty(),
            "RendererHorizontalNonrealization: unsupported or foreign submitted-partition readback");
        let mut ranges = std::collections::BTreeSet::new();
        for inserted in &observation.inserted_system_breaks {
            let mut matches = systems.iter().copied().enumerate().filter_map(|(index, start)| {
                let end = systems.get(index + 1).map_or(plan.slots.len(), |next| next - 1);
                (start < *inserted && *inserted <= end).then_some((start, end))
            });
            let range = matches.next().ok_or_else(|| anyhow::anyhow!("RendererHorizontalNonrealization: no internal interval"))?;
            anyhow::ensure!(matches.next().is_none(), "RendererHorizontalNonrealization: ambiguous interval");
            ranges.insert(range);
        }
        Ok(ranges)
    }

    fn validate_horizontal_context(&self, initial: &HorizontalLayoutPlan, plan: &HorizontalLayoutPlan) -> anyhow::Result<()> {
        anyhow::ensure!(!initial.slots.is_empty() && initial.slots.len() == plan.slots.len()
            && initial.slots.iter().zip(&plan.slots).all(|(before, after)| before.directives.explicit_width == after.directives.explicit_width),
            "RendererHorizontalNonrealization: horizontal width/context changed");
        Ok(())
    }
}
