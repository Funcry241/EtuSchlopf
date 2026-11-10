use super::*;
use crate::audit_only::renderer_system_break_readback::PageClearanceSystem;
fn fixture() -> (
    Vec<Vec<usize>>,
    GlobalHorizontalPageSystemPlanAudit,
    PageClearanceObservation,
) {
    let pages = vec![vec![2, 2], vec![2, 2, 2]];
    let basis = GlobalHorizontalPageSystemPlanAudit {
        slots_compared: 10,
        selected_measure_demand_evidence: vec![10; 10],
        accepted_system_capacity: 100,
        max_allowed_system_measures: 3,
        vertical_max_systems_per_page: 4,
        feasible_page_counts: vec![2],
        ..Default::default()
    };
    let systems = (0..5)
        .map(|i| PageClearanceSystem {
            start: i * 2 + 1,
            end: i * 2 + 2,
            page: usize::from(i >= 2),
            ink_top: 10.0,
            ink_bottom: 20.0,
        })
        .collect();
    let observed = PageClearanceObservation {
        systems,
        gaps: vec![vec![40.0], vec![6.0, 6.0]],
        page_top: vec![10.0, 10.0],
        page_bottom: vec![90.0, 80.0],
        glyphs: vec![std::collections::BTreeMap::from([('x', 2)]); 5],
    };
    (pages, basis, observed)
}
#[test]
fn fixed_page_candidate_copies_every_system_inside_the_existing_domain() {
    let (pages, basis, observed) = fixture();
    let (candidate, pair) = build_renderer_fixed_page_balance_candidate(
        &pages,
        &basis,
        &observed,
        &RendererRejectedPartitions::new(),
    )
    .unwrap();
    assert_eq!(pair, 0);
    assert_eq!(
        candidate.page_system_lengths,
        vec![vec![2, 2, 2], vec![2, 2]]
    );
    assert_eq!(candidate.pages(), pages.len());
    assert_eq!(candidate.systems, 5);
    assert_eq!(
        candidate
            .page_system_lengths
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>(),
        pages.iter().flatten().copied().collect::<Vec<_>>()
    );
    let mut tight = basis.clone();
    tight.vertical_max_systems_per_page = 2;
    assert!(
        build_renderer_fixed_page_balance_candidate(
            &pages,
            &tight,
            &observed,
            &RendererRejectedPartitions::new()
        )
        .is_none()
    );
}
#[test]
fn independent_documents_cannot_change_a_score_candidate() {
    let (pages, basis, observed) = fixture();
    let before = build_renderer_fixed_page_balance_candidate(
        &pages,
        &basis,
        &observed,
        &RendererRejectedPartitions::new(),
    )
    .unwrap()
    .0;
    let mut other = observed.clone();
    other.page_bottom = vec![130.0, 120.0];
    let _ = build_renderer_fixed_page_balance_candidate(
        &pages,
        &basis,
        &other,
        &RendererRejectedPartitions::new(),
    );
    assert_eq!(
        before,
        build_renderer_fixed_page_balance_candidate(
            &pages,
            &basis,
            &observed,
            &RendererRejectedPartitions::new()
        )
        .unwrap()
        .0
    );
}
#[test]
fn only_fresh_strict_gain_with_complete_matching_glyphs_can_adopt() {
    let (_, _, before) = fixture();
    let mut after = before.clone();
    after.gaps = vec![vec![9.0, 9.0], vec![20.0]];
    after.systems[2].page = 0;
    assert!(fixed_page_balance_improves(&before, &after, 0));
    assert!(!fixed_page_balance_improves(&before, &before, 0));
    after.gaps[0][0] = 0.0;
    assert!(!fixed_page_balance_improves(&before, &after, 0));
    after.gaps[0][0] = 9.0;
    after.glyphs[0].clear();
    assert!(!fixed_page_balance_improves(&before, &after, 0));
    after = before.clone();
    after.systems[0].end += 1;
    assert!(!fixed_page_balance_improves(&before, &after, 0));
}
#[test]
fn late_representation_must_retain_gain_without_reconstructing_identity() {
    let (_, _, before) = fixture();
    let mut after = before.clone();
    after.gaps = vec![vec![9.0, 9.0], vec![20.0]];
    after.systems[2].page = 0;
    assert!(fixed_page_balance_final_gain(&before, &after, PageBalanceObjective::PreserveSystems(0)));
    after.gaps[0][0] = 5.0;
    assert!(!fixed_page_balance_final_gain(&before, &after, PageBalanceObjective::PreserveSystems(0)));
    after.gaps[0][0] = f64::NAN;
    assert!(!fixed_page_balance_final_gain(&before, &after, PageBalanceObjective::PreserveSystems(0)));
}

#[test]
fn geometric_gain_cannot_leave_a_larger_nonfinal_page_remainder() {
    let (_, _, mut before) = fixture();
    before.systems[1].ink_bottom = 45.0;
    let mut after = before.clone();
    after.systems[2].page = 0;
    after.systems[2].ink_bottom = 40.0;
    after.gaps = vec![vec![9.0, 9.0], vec![20.0]];
    assert!(!fixed_page_balance_improves(&before, &after, 0));
    assert!(!fixed_page_balance_final_gain(&before, &after, PageBalanceObjective::PreserveSystems(0)));

    after.systems[2].ink_bottom = 50.0;
    after.systems[4].ink_bottom = 15.0;
    assert!(fixed_page_balance_improves(&before, &after, 0));
    assert!(fixed_page_balance_final_gain(&before, &after, PageBalanceObjective::PreserveSystems(0)));
}

#[test]
fn extra_system_requires_better_fill_across_the_non_title_pages() {
    let (_, _, mut before) = fixture();
    before.page_bottom[1] = 100.0;
    before.systems[3].ink_bottom = 70.0;
    before.systems[4].ink_bottom = 70.0;
    before.page_bottom.push(100.0);
    before.gaps.push(vec![12.0]);
    for (start, bottom) in [(11, 20.0), (13, 30.0)] {
        before.systems.push(PageClearanceSystem {
            start,
            end: start + 1,
            page: 2,
            ink_top: 10.0,
            ink_bottom: bottom,
        });
    }
    before.systems.last_mut().unwrap().end = 16;
    let mut after = before.clone();
    let mut extra = after.systems.last().unwrap().clone();
    after.systems.last_mut().unwrap().end = 14;
    extra.start = 15;
    extra.ink_bottom = 80.0;
    after.systems.push(extra);
    after.gaps[2].push(15.0);
    assert!(extra_system_page_fill_improves(&before, &after, 2));

    after.gaps[2][1] = 0.0;
    assert!(!extra_system_page_fill_improves(&before, &after, 2));
    after.gaps[2][1] = 15.0;
    after.systems[4].ink_bottom = 20.0;
    assert!(!extra_system_page_fill_improves(&before, &after, 2));
}
