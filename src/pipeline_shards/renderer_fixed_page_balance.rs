struct RendererFixedPageBalanceEvidence {
    before: PageClearanceObservation,
    pair: PageBalanceObjective,
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
    objective: PageBalanceObjective,
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

// The ending may leave space below the last system. Earlier pages should
// not acquire a larger worst-case remainder just to open up a system gap.
fn max_nonfinal_page_remainder(observation: &PageClearanceObservation) -> Option<f64> {
    let nonfinal_pages = observation.page_bottom.len().checked_sub(1)?;
    observation.page_bottom[..nonfinal_pages].iter().enumerate().try_fold(
        0.0_f64,
        |worst, (page, bottom)| {
            let system = observation.systems.iter().rev().find(|system| system.page == page)?;
            let remainder = bottom - system.ink_bottom;
            remainder.is_finite().then_some(worst.max(remainder))
        },
    )
}

fn fixed_page_balance_keeps_page_fill(
    before: &PageClearanceObservation,
    after: &PageClearanceObservation,
) -> bool {
    matches!((max_nonfinal_page_remainder(before), max_nonfinal_page_remainder(after)),
        (Some(before), Some(after)) if after <= before)
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
    fixed_page_balance_keeps_page_fill(before, after)
        && matches!((fixed_page_pair_clearance(before,pair),fixed_page_pair_clearance(after,pair)),(Some(a),Some(b)) if b>a)
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
    objective: PageBalanceObjective,
) -> bool {
    let pair = match objective {
        PageBalanceObjective::PreserveSystems(pair) => pair,
        PageBalanceObjective::AddSystem(page) => return extra_system_page_fill_improves(before, after, page),
    };
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
        && fixed_page_balance_keeps_page_fill(before, after)
        && matches!((fixed_page_pair_clearance(before,pair),fixed_page_pair_clearance(after,pair)),(Some(a),Some(b)) if b>a)
}

#[derive(Clone, Copy, Debug)]
enum PageBalanceObjective {
    PreserveSystems(usize),
    AddSystem(usize),
}

fn non_title_bottom_remainders(observation: &PageClearanceObservation) -> Option<Vec<f64>> {
    observation.page_bottom.iter().enumerate().skip(1).map(|(page, bottom)| {
        let system = observation.systems.iter().rev().find(|system| system.page == page)?;
        let remainder = bottom - system.ink_bottom;
        (remainder.is_finite() && remainder >= 0.0).then_some(remainder)
    }).collect()
}

fn extra_system_page_fill_improves(
    before: &PageClearanceObservation,
    after: &PageClearanceObservation,
    page: usize,
) -> bool {
    if before.page_bottom != after.page_bottom
        || after.systems.len() != before.systems.len() + 1
        || before.systems.first().map(|s| s.start) != after.systems.first().map(|s| s.start)
        || before.systems.last().map(|s| s.end) != after.systems.last().map(|s| s.end)
        || after.systems.iter().any(|s| {
            !s.ink_top.is_finite() || s.ink_top < 0.0 || !s.ink_bottom.is_finite()
                || s.ink_bottom > after.page_bottom.get(s.page).copied().unwrap_or(f64::NEG_INFINITY)
        })
        || after.gaps.iter().flatten().any(|g| !g.is_finite() || *g <= 0.0)
    {
        return false;
    }

    let (Some(before), Some(after)) = (
        non_title_bottom_remainders(before),
        non_title_bottom_remainders(after),
    ) else {
        return false;
    };
    let maximum = |r: &[f64]| r.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let spread = |r: &[f64]| maximum(r) - r.iter().copied().fold(f64::INFINITY, f64::min);
    matches!((before.get(page - 1), after.get(page - 1)), (Some(a), Some(b)) if b < a)
        && maximum(&after) < maximum(&before)
        && spread(&after) < spread(&before)
}

fn build_extra_system_page_fill_candidate(
    pages: &[Vec<usize>],
    basis: &GlobalHorizontalPageSystemPlanAudit,
    observed: &PageClearanceObservation,
    rejected: &RendererRejectedPartitions,
    no_goods: &std::collections::BTreeSet<(usize, usize)>,
) -> Option<(JointPageSystemCandidate, usize)> {
    let remainders = non_title_bottom_remainders(observed)?;
    if remainders.len() < 2 || observed.gaps.len() != pages.len() {
        return None;
    }
    let (largest, _) = remainders.iter().enumerate().max_by(|(_, a), (_, b)| a.total_cmp(b))?;
    let page = largest + 1;
    let mut counts = pages.iter().map(Vec::len).collect::<Vec<_>>();
    let count = counts.get_mut(page)?;
    if *count >= basis.vertical_max_systems_per_page {
        return None;
    }
    *count += 1;

    let slots = basis.slots_compared;
    let demands = &basis.selected_measure_demand_evidence;
    let clearance = &basis.selected_measure_clearance_edge_evidence;
    let capacity = basis.accepted_system_capacity;
    let max_length = basis.max_allowed_system_measures;
    let systems = counts.iter().sum::<usize>();
    if systems > slots / 2 || demands.len() != slots || clearance.len() != slots {
        return None;
    }

    let mut prefixes = std::collections::BTreeMap::from([(0, JointPageSystemCandidate::empty())]);
    let mut remaining = systems;
    for count in counts {
        let mut next = std::collections::BTreeMap::<usize, JointPageSystemCandidate>::new();
        for (start, prefix) in prefixes {
            let mut segments = std::collections::BTreeMap::from([(start, (0_u128, Vec::new()))]);
            for index in 0..count {
                let systems_after = remaining - index - 1;
                let mut ends = std::collections::BTreeMap::new();
                for (offset, (cost, lengths)) in segments {
                    let available_slots = slots - offset - systems_after * 2;
                    for length in 2..=max_length.min(available_slots) {
                        let end = offset + length;
                        let load = demands[offset..end].iter().sum::<usize>();
                        if load > capacity {
                            break;
                        }
                        if no_goods.contains(&(offset + 1, end))
                            || !renderer_clearance_system_has_structural_slack(
                                &clearance[offset..end], length, max_length)
                        {
                            continue;
                        }
                        let slack = (capacity - load) as u128;
                        let mut path = lengths.clone();
                        path.push(length);
                        let segment = (cost + slack * slack, path);
                        if ends.get(&end).is_none_or(|old| &segment < old) {
                            ends.insert(end, segment);
                        }
                    }
                }
                segments = ends;
            }
            for (end, (cost, lengths)) in segments {
                let mut candidate = prefix.clone();
                let page_slots = end - start;
                let deviation = (page_slots * pages.len()).abs_diff(slots) as u128;
                candidate.page_slot_balance_penalty += deviation * deviation;
                candidate.slack_square_sum += cost;
                candidate.page_slot_lengths.push(page_slots);
                candidate.page_system_lengths.push(lengths);
                if next.get(&end).is_none_or(|old| candidate.is_better_than(old)) {
                    next.insert(end, candidate);
                }
            }
        }
        remaining -= count;
        prefixes = next;
    }
    let lengths = prefixes.remove(&slots)?.page_system_lengths;
    if rejected.contains(&lengths) {
        return None;
    }
    Some((score_fixed_system_partition(&lengths, basis)?, page))
}

#[cfg(test)]
#[path = "../../tests/unit/renderer_fixed_page_balance.rs"]
mod renderer_fixed_page_balance_tests;
