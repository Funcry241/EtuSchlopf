struct RendererFixedPageBalanceEvidence {
    before: PageClearanceObservation,
    pair: usize,
    consumer: render::RendererProbeIdentity,
    source_fingerprint: u64,
    selected_system_pages: Vec<usize>,
    shadow_input: std::path::PathBuf,
    shadow_output: std::path::PathBuf,
}

use crate::audit_only::renderer_system_break_readback::PageClearanceObservation;

struct RendererFixedPageBalanceTrial {
    incumbent: HorizontalLayoutPlan,
    before: PageClearanceObservation,
    pair: usize,
}

fn fixed_page_pair_clearance(observation: &PageClearanceObservation, pair: usize) -> Option<f64> {
    let pages = observation.gaps.get(pair..pair.checked_add(2)?)?;
    let gap = pages
        .iter()
        .flatten()
        .copied()
        .fold(f64::INFINITY, f64::min);
    gap.is_finite().then_some(gap)
}

fn fixed_page_balance_improves(
    before: &PageClearanceObservation,
    after: &PageClearanceObservation,
    pair: usize,
) -> bool {
    if before.systems.len() != after.systems.len()
        || before.gaps.len() != after.gaps.len()
        || before.glyphs != after.glyphs
        || before.systems.iter().zip(&after.systems).any(|(a, b)| {
            a.start != b.start
                || a.end != b.end
                || (a.page != b.page
                    && (!(pair..=pair + 1).contains(&a.page)
                        || !(pair..=pair + 1).contains(&b.page)))
        })
        || after
            .gaps
            .iter()
            .flatten()
            .any(|g| !g.is_finite() || *g <= 0.0)
        || before.gaps.iter().enumerate().any(|(page, gaps)| {
            page != pair && page != pair + 1 && after.gaps.get(page) != Some(gaps)
        })
    {
        return false;
    }
    matches!((fixed_page_pair_clearance(before,pair),fixed_page_pair_clearance(after,pair)),(Some(a),Some(b)) if b>a)
}

fn score_fixed_system_partition(
    pages: &[Vec<usize>],
    basis: &GlobalHorizontalPageSystemPlanAudit,
) -> Option<JointPageSystemCandidate> {
    if pages.is_empty()
        || pages.iter().any(Vec::is_empty)
        || !basis.feasible_page_counts.contains(&pages.len())
    {
        return None;
    }
    let mut candidate = JointPageSystemCandidate::empty();
    let mut cursor = 0usize;
    let systems = pages.iter().map(Vec::len).sum::<usize>();
    for page in pages {
        if page.len() > basis.vertical_max_systems_per_page {
            return None;
        }
        let slots = page.iter().sum::<usize>();
        candidate.systems += page.len();
        candidate.singletons += page.iter().filter(|&&n| n == 1).count();
        candidate.system_count_balance_penalty +=
            (page.len().saturating_mul(pages.len()).abs_diff(systems) as u128).pow(2);
        candidate.page_slot_balance_penalty += (slots
            .saturating_mul(pages.len())
            .abs_diff(basis.slots_compared)
            as u128)
            .pow(2);
        for &length in page {
            if length == 0 || length > basis.max_allowed_system_measures {
                return None;
            }
            let end = cursor.checked_add(length)?;
            let load = basis
                .selected_measure_demand_evidence
                .get(cursor..end)?
                .iter()
                .sum::<usize>();
            if load > basis.accepted_system_capacity {
                return None;
            }
            candidate.slack_square_sum +=
                (basis.accepted_system_capacity.saturating_sub(load) as u128).pow(2);
            candidate.max_system_load = candidate.max_system_load.max(load);
            cursor = end;
        }
        candidate.page_slot_lengths.push(slots);
        candidate.page_system_lengths.push(page.clone());
    }
    (cursor == basis.slots_compared && cursor == basis.selected_measure_demand_evidence.len())
        .then_some(candidate)
}

fn build_renderer_fixed_page_balance_candidate(
    pages: &[Vec<usize>],
    basis: &GlobalHorizontalPageSystemPlanAudit,
    observed: &PageClearanceObservation,
    rejected: &RendererRejectedPartitions,
) -> Option<(JointPageSystemCandidate, usize)> {
    if observed.gaps.len() != pages.len()
        || observed.page_bottom.len() != pages.len()
        || observed.page_top.len() != pages.len()
        || observed
            .gaps
            .iter()
            .flatten()
            .any(|g| !g.is_finite() || *g <= 0.0)
    {
        return None;
    }
    let flat = pages.iter().flatten().copied().collect::<Vec<_>>();
    if flat.len() != observed.systems.len() || observed.glyphs.len() != flat.len() {
        return None;
    }
    let mut at = 1usize;
    for (&length, system) in flat.iter().zip(&observed.systems) {
        if system.start != at
            || system.end.checked_add(1)? != at.checked_add(length)?
            || !system.ink_top.is_finite()
            || !system.ink_bottom.is_finite()
            || system.ink_bottom <= system.ink_top
        {
            return None;
        }
        at += length;
    }
    let expected_pages = pages
        .iter()
        .enumerate()
        .flat_map(|(page, course)| std::iter::repeat_n(page, course.len()))
        .collect::<Vec<_>>();
    if observed
        .systems
        .iter()
        .zip(expected_pages)
        .any(|(system, page)| system.page != page)
    {
        return None;
    }
    score_fixed_system_partition(pages, basis)?;
    let mut best = None::<(f64, JointPageSystemCandidate, usize)>;
    for pair in 0..pages.len().saturating_sub(1) {
        let incumbent_gap = fixed_page_pair_clearance(observed, pair)?;
        for from_right in [true, false] {
            let mut next = pages.to_vec();
            if from_right {
                if next[pair + 1].len() <= 1 {
                    continue;
                }
                let length = next[pair + 1].remove(0);
                next[pair].push(length);
            } else {
                if next[pair].len() <= 1 {
                    continue;
                }
                let length = next[pair].pop()?;
                next[pair + 1].insert(0, length);
            }
            if rejected.contains(&next) {
                continue;
            }
            let Some(candidate) = score_fixed_system_partition(&next, basis) else {
                continue;
            };
            // An estimate orders alternatives of this one bound whole-score
            // transaction only. It is never evidence of realized clearance.
            let mut first = 0usize;
            let mut prediction = f64::INFINITY;
            for (page, course) in next.iter().enumerate() {
                let end = first + course.len();
                if page == pair || page == pair + 1 {
                    let heights = observed.systems[first..end]
                        .iter()
                        .map(|s| s.ink_bottom - s.ink_top)
                        .sum::<f64>();
                    let available = observed.page_bottom[page] - observed.page_top[page];
                    if course.len() > 1 {
                        prediction =
                            prediction.min((available - heights) / (course.len() - 1) as f64);
                    }
                }
                first = end;
            }
            let estimated_gain = prediction - incumbent_gap;
            if !estimated_gain.is_finite() {
                continue;
            }
            if best.as_ref().is_none_or(|(gain, old, _)| {
                estimated_gain > *gain || (estimated_gain == *gain && candidate.is_better_than(old))
            }) {
                best = Some((estimated_gain, candidate, pair));
            }
        }
    }
    best.map(|(_, candidate, pair)| (candidate, pair))
}

fn fixed_page_balance_final_gain(
    before: &PageClearanceObservation,
    after: &PageClearanceObservation,
    pair: usize,
) -> bool {
    before.systems.len() == after.systems.len()
        && before.gaps.len() == after.gaps.len()
        && before
            .systems
            .iter()
            .zip(&after.systems)
            .all(|(a, b)| a.start == b.start && a.end == b.end)
        && after
            .gaps
            .iter()
            .flatten()
            .all(|g| g.is_finite() && *g > 0.0)
        && matches!((fixed_page_pair_clearance(before,pair),fixed_page_pair_clearance(after,pair)),(Some(a),Some(b)) if b>a)
}

#[cfg(test)]
#[path = "../../tests/unit/renderer_fixed_page_balance.rs"]
mod renderer_fixed_page_balance_tests;
