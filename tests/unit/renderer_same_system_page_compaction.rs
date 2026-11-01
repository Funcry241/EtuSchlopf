use super::*;

fn bound_basis(course: &[Vec<usize>], ceiling: usize, frontier: &[usize]) -> GlobalHorizontalPageSystemPlanAudit {
    let slots = course.iter().flatten().sum();
    GlobalHorizontalPageSystemPlanAudit {
        slots_compared: slots,
        feasible_page_counts: frontier.to_vec(),
        accepted_system_capacity: 1000,
        max_allowed_system_measures: 3,
        vertical_max_systems_per_page: ceiling,
        selected_measure_demand_evidence: vec![137; slots],
        ..Default::default()
    }
}

#[test]
fn page_refinement_preserves_the_complete_ordered_system_course() {
    let current = vec![vec![2, 3], vec![1, 2], vec![3], vec![2, 1]];
    let basis = bound_basis(&current, 3, &[2, 3, 4]);
    let selected = build_renderer_same_system_page_candidate(&current, &basis).unwrap().unwrap();
    assert_eq!(selected.pages(), 3);
    assert_eq!(selected.page_system_lengths.iter().flatten().copied().collect::<Vec<_>>(),
        current.iter().flatten().copied().collect::<Vec<_>>());
    assert_eq!(selected.systems, 7);
    assert_eq!(selected.singletons, 2);
    assert_eq!(selected.forced_over_capacity_singletons, 0);
    assert!(selected.page_system_lengths.iter().all(|p| p.len() <= 3));
}

#[test]
fn page_refinement_takes_the_next_lower_admitted_page_count_only() {
    let current = vec![vec![2, 2]; 4];
    let basis = bound_basis(&current, 4, &[2, 3, 4]);
    let selected = build_renderer_same_system_page_candidate(&current, &basis).unwrap().unwrap();
    assert_eq!(selected.pages(), 3);
}

#[test]
fn page_refinement_does_not_extend_the_existing_frontier() {
    let current = vec![vec![2, 2]; 4];
    assert!(build_renderer_same_system_page_candidate(&current, &bound_basis(&current, 4, &[4])).unwrap().is_none());
    assert!(build_renderer_same_system_page_candidate(&current, &bound_basis(&current, 4, &[1, 4])).unwrap().is_none());
}

#[test]
fn page_refinement_retains_the_existing_vertical_ceiling() {
    let current = vec![vec![2, 2]; 4];
    let basis = bound_basis(&current, 2, &[3, 4]);
    assert!(build_renderer_same_system_page_candidate(&current, &basis).unwrap().is_none());
}

#[test]
fn page_refinement_refuses_corrupt_binding_or_capacity() {
    let current = vec![vec![2, 2]; 4];
    let basis = bound_basis(&current, 4, &[3, 4]);
    let mut wrong = basis.clone();wrong.selected_measure_demand_evidence.pop();
    assert!(build_renderer_same_system_page_candidate(&current, &wrong).is_err());
    let mut overloaded = basis.clone();overloaded.selected_measure_demand_evidence[0] = 1001;
    assert!(build_renderer_same_system_page_candidate(&current, &overloaded).is_err());
    let zero = vec![vec![0, 2]; 4];
    assert!(build_renderer_same_system_page_candidate(&zero, &bound_basis(&zero, 4, &[3, 4])).is_err());
    let empty = vec![Vec::new()];
    assert!(build_renderer_same_system_page_candidate(&empty, &bound_basis(&empty, 4, &[1])).is_err());
}

#[test]
fn page_refinement_is_read_only_and_independent_documents_do_not_compete() {
    let current = vec![vec![2, 2]; 4];let basis = bound_basis(&current, 4, &[3, 4]);
    let before_course = current.clone();let before_basis = basis.clone();
    let before = build_renderer_same_system_page_candidate(&current, &basis).unwrap();
    let other = vec![vec![1, 3], vec![2, 1], vec![3, 2]];
    let _ = build_renderer_same_system_page_candidate(&other, &bound_basis(&other, 3, &[2, 3])).unwrap();
    assert_eq!(before, build_renderer_same_system_page_candidate(&current, &basis).unwrap());
    assert_eq!(current, before_course);assert_eq!(basis, before_basis);
}

#[test]
fn page_refinement_matches_exhaustive_small_fixed_course_partitions() {
    fn compositions(left: usize, pages: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if pages == 0 { if left == 0 { out.push(prefix.clone()); } return; }
        for n in 1..=left { prefix.push(n);compositions(left - n, pages - 1, prefix, out);prefix.pop(); }
    }
    for count in 3..=7 {
        let course = (0..count).map(|i| vec![i % 3 + 1]).collect::<Vec<_>>();
        let flat = course.iter().flatten().copied().collect::<Vec<_>>();
        let slots = flat.iter().sum::<usize>();
        for pages in 1..count {
            let basis = bound_basis(&course, 3, &[pages, count]);
            let actual = build_renderer_same_system_page_candidate(&course, &basis).unwrap();
            let mut all = Vec::new();compositions(count, pages, &mut Vec::new(), &mut all);
            let mut scored = Vec::new();
            for parts in all.into_iter().filter(|p| p.iter().all(|&n| n <= 3)) {
                let mut at = 0;let mut groups = Vec::new();
                for &n in &parts { groups.push(flat[at..at + n].to_vec());at += n; }
                let lengths = groups.iter().map(|p| p.iter().sum::<usize>()).collect::<Vec<_>>();
                let system_balance = parts.iter().map(|&n| (n.saturating_mul(pages).abs_diff(count) as u128).pow(2)).sum::<u128>();
                let slot_balance = lengths.iter().map(|&n| (n.saturating_mul(pages).abs_diff(slots) as u128).pow(2)).sum::<u128>();
                scored.push((system_balance, slot_balance, lengths, groups));
            }
            scored.sort();
            assert_eq!(actual.map(|p| p.page_system_lengths), scored.first().map(|p| p.3.clone()));
        }
    }
}

#[test]
fn confirmed_refinement_chain_preserves_systems_through_each_strict_frontier_step() {
    let initial = vec![vec![2, 1]; 8];
    let basis = bound_basis(&initial, 6, &[3, 5, 6, 8]);
    let expected_systems = initial.iter().flatten().copied().collect::<Vec<_>>();
    let untouched_basis = basis.clone();
    let mut confirmed = initial.clone();
    for expected_pages in [6, 5, 3] {
        let next = build_renderer_same_system_page_candidate(&confirmed, &basis).unwrap().unwrap();
        assert_eq!(next.pages(), expected_pages);
        assert!(next.pages() < confirmed.len());
        assert_eq!(next.page_system_lengths.iter().flatten().copied().collect::<Vec<_>>(), expected_systems);
        assert!(next.page_system_lengths.iter().all(|page| page.len() <= 6));
        confirmed = next.page_system_lengths;
    }
    assert!(build_renderer_same_system_page_candidate(&confirmed, &basis).unwrap().is_none());
    assert_eq!(basis, untouched_basis);
    let independent = vec![vec![3, 1]; 5];
    let _ = build_renderer_same_system_page_candidate(&independent, &bound_basis(&independent, 4, &[3, 4, 5])).unwrap();
    assert_eq!(build_renderer_same_system_page_candidate(&initial, &basis).unwrap().unwrap().pages(), 6);
}
