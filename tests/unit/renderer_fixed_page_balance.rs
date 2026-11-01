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
    assert!(fixed_page_balance_final_gain(&before, &after, 0));
    after.gaps[0][0] = 5.0;
    assert!(!fixed_page_balance_final_gain(&before, &after, 0));
    after.gaps[0][0] = f64::NAN;
    assert!(!fixed_page_balance_final_gain(&before, &after, 0));
}
