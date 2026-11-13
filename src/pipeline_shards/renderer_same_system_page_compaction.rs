// Page-only refinement of an already renderer-confirmed ordered system course.
// The existing frontier supplies the next lower page count; the existing joint
// candidate comparator supplies balance. No new widths, heights or calibration.
fn build_renderer_same_system_page_candidate(
    current_pages: &[Vec<usize>],
    basis: &GlobalHorizontalPageSystemPlanAudit,
) -> anyhow::Result<Option<JointPageSystemCandidate>> {
    if basis.compact_geometry_planned {
        return Ok(None);
    }
    let systems = current_pages.iter().flatten().copied().collect::<Vec<_>>();
    if current_pages.is_empty()
        || current_pages.iter().any(Vec::is_empty)
        || systems.contains(&0)
        || systems.iter().sum::<usize>() != basis.selected_measure_demand_evidence.len()
        || basis.slots_compared != basis.selected_measure_demand_evidence.len()
        || basis.accepted_system_capacity == 0
        || basis.vertical_max_systems_per_page == 0
    {
        anyhow::bail!("RendererSameSystemPageRefinement: unbound confirmed system course");
    }
    let Some(pages) = basis.feasible_page_counts.iter().rev().copied()
        .find(|pages| *pages < current_pages.len()) else {
        return Ok(None);
    };
    let mut cursor = 0usize;
    let mut loads = Vec::with_capacity(systems.len());
    for &length in &systems {
        let end = cursor + length;
        let load = basis.selected_measure_demand_evidence[cursor..end].iter().sum::<usize>();
        if length > basis.max_allowed_system_measures || load > basis.accepted_system_capacity {
            anyhow::bail!("RendererSameSystemPageRefinement: confirmed course violates bound capacity or shape");
        }
        loads.push(load);
        cursor = end;
    }
    let mut dp = vec![vec![None::<JointPageSystemCandidate>; systems.len() + 1]; pages + 1];
    dp[0][0] = Some(JointPageSystemCandidate::empty());
    for page_index in 0..pages {
        for start in 0..systems.len() {
            let Some(prefix) = dp[page_index][start].clone() else { continue; };
            for end in start + 1..=systems.len() {
                if end - start > basis.vertical_max_systems_per_page { continue; }
                let mut candidate = prefix.clone();
                let page_lengths = &systems[start..end];
                let page_slots = page_lengths.iter().sum::<usize>();
                candidate.systems += page_lengths.len();
                candidate.singletons += page_lengths.iter().filter(|&&length| length == 1).count();
                candidate.system_count_balance_penalty +=
                    (page_lengths.len().saturating_mul(pages).abs_diff(systems.len()) as u128).pow(2);
                candidate.page_slot_balance_penalty +=
                    (page_slots.saturating_mul(pages).abs_diff(basis.slots_compared) as u128).pow(2);
                for &load in &loads[start..end] {
                    candidate.slack_square_sum +=
                        (basis.accepted_system_capacity.saturating_sub(load) as u128).pow(2);
                    candidate.max_system_load = candidate.max_system_load.max(load);
                }
                candidate.page_slot_lengths.push(page_slots);
                candidate.page_system_lengths.push(page_lengths.to_vec());
                if dp[page_index + 1][end].as_ref()
                    .is_none_or(|current| candidate.is_better_than(current)) {
                    dp[page_index + 1][end] = Some(candidate);
                }
            }
        }
    }
    Ok(dp[pages][systems.len()].clone())
}

#[cfg(test)]
#[path = "../../tests/unit/renderer_same_system_page_compaction.rs"]
mod renderer_same_system_page_compaction_tests;
