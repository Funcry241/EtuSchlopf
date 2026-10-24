use super::*;

fn all_partitions(slots: usize) -> Vec<Vec<Vec<usize>>> {
    fn extend(left: usize, pages: Vec<Vec<usize>>, out: &mut Vec<Vec<Vec<usize>>>) {
        if left == 0 { out.push(pages); return; }
        for length in 1..=left {
            if !pages.is_empty() {
                let mut next = pages.clone(); next.last_mut().unwrap().push(length);
                extend(left - length, next, out);
            }
            let mut next = pages.clone(); next.push(vec![length]); extend(left - length, next, out);
        }
    }
    let mut out = Vec::new(); extend(slots, Vec::new(), &mut out); out
}

fn score(parts: &[Vec<usize>], request: &RendererFeasibilityFrontierRequest<'_>) -> Option<JointPageSystemCandidate> {
    if !request.feasible_page_counts.contains(&parts.len()) { return None; }
    let mut joint = JointPageSystemCandidate::empty();
    let mut cursor = 0;
    for page in parts {
        if page.len() > request.vertical_max_systems_per_page { return None; }
        let count = page.len();
        let slots = page.iter().sum::<usize>();
        for &length in page {
            if length > request.max_system_measures { return None; }
            let end = cursor + length;
            let load = request.measure_demands[cursor..end].iter().sum::<usize>();
            if length > 1 && load > request.accepted_system_capacity { return None; }
            if !renderer_clearance_system_has_structural_slack(&request.measure_clearance_edges[cursor..end], length, request.max_system_measures)
                || request.no_goods.contains(&(cursor + 1, end)) { return None; }
            joint.systems += 1;
            joint.singletons += usize::from(length == 1);
            joint.forced_over_capacity_singletons += usize::from(length == 1 && load > request.accepted_system_capacity);
            joint.max_system_load = joint.max_system_load.max(load);
            joint.slack_square_sum += (request.accepted_system_capacity.saturating_sub(load) as u128).pow(2);
            cursor = end;
        }
        joint.system_count_balance_penalty += (count.saturating_mul(parts.len()).abs_diff(request.reference_systems) as u128).pow(2);
        joint.page_slot_balance_penalty += (slots.saturating_mul(parts.len()).abs_diff(request.measure_demands.len()) as u128).pow(2);
        joint.page_slot_lengths.push(slots);
        joint.page_system_lengths.push(page.clone());
    }
    Some(joint)
}

#[test]
fn exact_search_matches_complete_small_partition_enumeration() {
    for slots in 2..=6 {
        let universe = all_partitions(slots);
        for varied in [false, true] {
            let demands = (0..slots).map(|index| if varied && index % 2 == 0 { 120 } else { 60 }).collect::<Vec<_>>();
            let edges = (0..slots).map(|index| usize::from(varied && index % 3 == 0)).collect::<Vec<_>>();
            let frontier = (1..=slots).collect::<Vec<_>>();
            let no_goods = if varied { std::collections::BTreeSet::from([(slots / 2, slots / 2 + 1)]) } else { std::collections::BTreeSet::new() };
            for patterns in universe.chunks(3) {
                let rejected = patterns.iter().cloned().collect();
                let request = RendererFeasibilityFrontierRequest { measure_demands: &demands, measure_clearance_edges: &edges,
                    feasible_page_counts: &frontier, preferred_pages: slots, accepted_system_capacity: 180,
                    max_system_measures: 3, vertical_max_systems_per_page: 3, reference_systems: slots,
                    no_goods: &no_goods, rejected_partitions: &rejected };
                let mut best: Option<JointPageSystemCandidate> = None;
                // Evaluate complete plans independently; preserve the existing
                // two-level owner ordering: per-page-count, then frontier.
                for &pages in &frontier {
                    let mut local: Option<JointPageSystemCandidate> = None;
                    for partition in universe.iter().filter(|p| p.len() == pages && !rejected.contains(*p)) {
                        if let Some(item) = score(partition, &request)
                            && local.as_ref().is_none_or(|old| item.is_better_than(old)) { local = Some(item); }
                    }
                    if let Some(item) = local
                        && best.as_ref().is_none_or(|old| item.is_preference_better_than(old, slots)) { best = Some(item); }
                }
                let actual = build_renderer_feasibility_frontier_candidate(&request);
                match best {
                    Some(expected) => assert_eq!(actual.unwrap().page_system_lengths, expected.page_system_lengths),
                    None => assert!(actual.is_err()),
                }
            }
        }
    }
}

#[test]
fn all_complete_exclusions_exhaust_the_bound_domain() {
    let rejected = all_partitions(4).into_iter().collect();
    let request = RendererFeasibilityFrontierRequest { measure_demands: &[60; 4], measure_clearance_edges: &[0; 4],
        feasible_page_counts: &[1,2,3,4], preferred_pages: 4, accepted_system_capacity: 180,
        max_system_measures: 3, vertical_max_systems_per_page: 4, reference_systems: 2,
        no_goods: &std::collections::BTreeSet::new(), rejected_partitions: &rejected };
    assert!(build_renderer_feasibility_frontier_candidate(&request).is_err());
}

fn plan(parts: &[Vec<usize>]) -> HorizontalLayoutPlan {
    let mut slots = Vec::new();
    for (page, lengths) in parts.iter().enumerate() {
        for (system, &length) in lengths.iter().enumerate() {
            for position in 0..length {
                slots.push(HorizontalLayoutPlannedSlot::from_directives_as_seed(HorizontalLayoutDirectiveState {
                    explicit_width: None, starts_new_page: page > 0 && system == 0 && position == 0,
                    starts_new_system: system > 0 && position == 0,
                }));
            }
        }
    }
    HorizontalLayoutPlan { slots }
}

fn observation() -> crate::audit_only::renderer_system_break_readback::RendererSystemBreakReadbackAudit {
    crate::audit_only::renderer_system_break_readback::RendererSystemBreakReadbackAudit {
        planned_system_starts: vec![1,2,4,5], rendered_system_starts: vec![1,2,4,5],
        planned_page_starts: vec![1,4], rendered_page_starts: vec![1,2,4],
        inserted_system_breaks: vec![], collapsed_system_breaks: vec![], inserted_page_breaks: vec![2],
        collapsed_page_breaks: vec![], measure_count: 6, pdf_pages: 3,
    }
}

#[test]
fn observations_require_exact_source_consumer_and_submitted_partition() {
    let mut exclusions = RendererPartitionRejections::<&str>::new("immutable source");
    let original = plan(&[vec![1,2],vec![1,2]]);
    assert!(exclusions.record(&original, &observation()).is_err());
    assert!(exclusions.bind("immutable source", None).is_err());
    assert!(exclusions.bind("foreign source", Some(&"consumer")).is_err());
    exclusions.bind("immutable source", Some(&"consumer")).unwrap();
    assert!(exclusions.bind("immutable source", Some(&"other consumer")).is_err());
    let mut foreign = observation();foreign.measure_count += 1;
    assert!(exclusions.record(&original, &foreign).is_err());
    exclusions.record(&original, &observation()).unwrap();
    assert!(exclusions.record(&original, &observation()).is_err());
    assert!(exclusions.validate(&original).is_err());
    assert!(exclusions.validate(&plan(&[vec![2,1],vec![1,2]])).is_ok());
}

#[test]
fn unsupported_or_incomplete_observations_never_add_an_exclusion() {
    let original = plan(&[vec![1,2],vec![1,2]]);
    for kind in 0..4 {
        let mut exclusions = RendererPartitionRejections::new("source");exclusions.bind("source", Some(&"consumer")).unwrap();
        let mut data = observation();
        match kind { 0 => data.inserted_page_breaks.clear(), 1 => data.collapsed_page_breaks.push(4),
            2 => data.collapsed_system_breaks.push(5), _ => data.planned_page_starts = vec![1,5] }
        assert!(exclusions.record(&original,&data).is_err());assert!(exclusions.partitions.is_empty());
    }
}

#[test]
fn independent_source_scopes_do_not_share_rejections() {
    let original = plan(&[vec![1,2],vec![1,2]]);
    let mut a = RendererPartitionRejections::new("source renamed alpha");
    let mut b = RendererPartitionRejections::new("source renamed beta");
    a.bind("source renamed alpha", Some(&"consumer")).unwrap();a.record(&original,&observation()).unwrap();
    assert!(b.validate(&original).is_ok());
    b.bind("source renamed beta", Some(&"consumer")).unwrap();b.record(&original,&observation()).unwrap();
    assert!(a.validate(&original).is_err());assert!(b.validate(&original).is_err());
}

#[test]
fn empty_exclusions_keep_the_original_owner_choice() {
    let no_goods = std::collections::BTreeSet::new();
    let empty = RendererRejectedPartitions::new();
    let request = RendererFeasibilityFrontierRequest { measure_demands: &[60; 6], measure_clearance_edges: &[0; 6],
        feasible_page_counts: &[2], preferred_pages: 2, accepted_system_capacity: 180,
        max_system_measures: 3, vertical_max_systems_per_page: 3, reference_systems: 2,
        no_goods: &no_goods, rejected_partitions: &empty };
    let old = build_renderer_feasibility_constraint_candidate(&[60;6], &[0;6], 2, (180,3), 3, 2, &no_goods).unwrap();
    let new = build_renderer_feasibility_frontier_candidate(&request).unwrap();
    assert_eq!(old.page_system_lengths,new.page_system_lengths);
}

#[test]
fn alternate_plan_passes_original_promote_and_keeps_its_negative_guards() {
    let rejected = RendererRejectedPartitions::from([vec![vec![3],vec![3]]]);
    let no_goods = std::collections::BTreeSet::new();
    let request = RendererFeasibilityFrontierRequest { measure_demands: &[60;6], measure_clearance_edges: &[0;6],
        feasible_page_counts: &[1,2,3], preferred_pages: 3, accepted_system_capacity: 180,
        max_system_measures: 3, vertical_max_systems_per_page: 3, reference_systems: 2,
        no_goods: &no_goods, rejected_partitions: &rejected };
    let chosen = build_renderer_feasibility_frontier_candidate(&request).unwrap();
    let initial = plan(&[vec![3],vec![3]]);
    let basis = GlobalHorizontalPageSystemPlanAudit { feasible_page_counts: vec![1,2,3],
        selected_measure_demand_evidence: vec![60;6], selected_measure_clearance_edge_evidence: vec![0;6],
        accepted_system_capacity: 180, max_allowed_system_measures: 3, vertical_max_systems_per_page: 3,
        ..Default::default() };
    let promoted = promote_renderer_feasibility_constraint_candidate(&initial,&chosen,&basis,&no_goods).unwrap();
    assert!(promoted.provenance_complete());
    assert_eq!(authoritative_page_shape(&promoted).unwrap().page_system_lengths,chosen.page_system_lengths);
    assert!(!rejected.contains(&chosen.page_system_lengths));
    let forbidden = std::collections::BTreeSet::from([(1usize,chosen.page_system_lengths[0][0])]);
    assert!(promote_renderer_feasibility_constraint_candidate(&initial,&chosen,&basis,&forbidden).is_err());
    let mut incomplete = initial.clone();incomplete.slots.pop();
    assert!(promote_renderer_feasibility_constraint_candidate(&incomplete,&chosen,&basis,&no_goods).is_err());
}

fn subdivided_observation(concurrent_page: bool) -> crate::audit_only::renderer_system_break_readback::RendererSystemBreakReadbackAudit {
    crate::audit_only::renderer_system_break_readback::RendererSystemBreakReadbackAudit {
        planned_system_starts: vec![1,4], rendered_system_starts: vec![1,3,4,6],
        planned_page_starts: vec![1,4], rendered_page_starts: if concurrent_page { vec![1,3,4] } else { vec![1,4] },
        inserted_system_breaks: vec![3,6], collapsed_system_breaks: vec![],
        inserted_page_breaks: if concurrent_page { vec![3] } else { vec![] },
        collapsed_page_breaks: vec![], measure_count: 6, pdf_pages: if concurrent_page { 3 } else { 2 },
    }
}

#[test]
fn horizontal_subdivision_is_reused_with_or_without_vertical_insertion() {
    let original = plan(&[vec![3],vec![3]]);
    for concurrent in [false,true] {
        let mut state = RendererPartitionRejections::new("fixed complete source and style");
        state.bind("fixed complete source and style",Some(&"actual fresh-import consumer")).unwrap();
        let data = subdivided_observation(concurrent);
        let ranges = state.system_ranges(&original,&original,&data).unwrap();
        assert_eq!(ranges,std::collections::BTreeSet::from([(1,3),(4,6)]));
        if concurrent { state.record(&original,&data).unwrap(); }
        assert_eq!(state.partitions.is_empty(),!concurrent);
        let request = RendererFeasibilityFrontierRequest { measure_demands: &[60;6], measure_clearance_edges: &[0;6],
            feasible_page_counts: &[2,3], preferred_pages: 3, accepted_system_capacity: 180,
            max_system_measures: 3, vertical_max_systems_per_page: 3, reference_systems: 2,
            no_goods: &ranges, rejected_partitions: &state.partitions };
        let chosen = build_renderer_feasibility_frontier_candidate(&request).unwrap();
        let basis = GlobalHorizontalPageSystemPlanAudit { feasible_page_counts: vec![2,3],
            selected_measure_demand_evidence: vec![60;6], selected_measure_clearance_edge_evidence: vec![0;6],
            accepted_system_capacity: 180, max_allowed_system_measures: 3, vertical_max_systems_per_page: 3,
            ..Default::default() };
        let promoted = promote_renderer_feasibility_constraint_candidate(&original,&chosen,&basis,&ranges).unwrap();
        assert!(promoted.provenance_complete());state.validate(&promoted).unwrap();
        state.validate_horizontal_context(&original,&promoted).unwrap();
        let mut shifted = promoted.clone();shifted.slots[0].directives.explicit_width = Some("different horizontal input".into());
        assert!(state.validate_horizontal_context(&original,&shifted).is_err());
    }
}

#[test]
fn vertical_insertion_alone_never_produces_a_horizontal_exclusion() {
    let original = plan(&[vec![1,2],vec![1,2]]);
    let mut state = RendererPartitionRejections::new("source");state.bind("source",Some(&"consumer")).unwrap();
    assert!(state.system_ranges(&original,&original,&observation()).unwrap().is_empty());
    state.record(&original,&observation()).unwrap();assert_eq!(state.partitions.len(),1);
}

#[test]
fn horizontal_evidence_refuses_unknown_changed_or_incomplete_dependencies() {
    let original = plan(&[vec![3],vec![3]]);
    let mut state = RendererPartitionRejections::new("source");
    assert!(state.system_ranges(&original,&original,&subdivided_observation(true)).is_err());
    state.bind("source",Some(&"consumer")).unwrap();
    assert!(state.bind("other source",Some(&"consumer")).is_err());
    assert!(state.bind("source",Some(&"changed command")).is_err());
    assert!(state.bind("source",None).is_err());
    for variant in 0..8 {
        let mut changed = original.clone();let mut data = subdivided_observation(true);
        match variant {
            0 => changed.slots[0].directives.explicit_width = Some("changed".into()),
            1 => data.collapsed_system_breaks.push(4),
            2 => data.collapsed_page_breaks.push(4),
            3 => data.planned_system_starts = vec![1,3],
            4 => data.inserted_system_breaks.push(7),
            5 => data.rendered_system_starts = vec![1,3,3,6],
            6 => data.rendered_page_starts = vec![1,2,4],
            _ => data.measure_count += 1,
        }
        assert!(state.system_ranges(&original,&changed,&data).is_err());
        assert!(state.partitions.is_empty());
    }
}

#[test]
fn independent_horizontal_observations_duplicate_without_suppression() {
    let original = plan(&[vec![3],vec![3]]);
    let mut left = RendererPartitionRejections::new("source alpha");
    let mut right = RendererPartitionRejections::new("renamed independent source beta");
    left.bind("source alpha",Some(&"consumer")).unwrap();
    right.bind("renamed independent source beta",Some(&"consumer")).unwrap();
    let data = subdivided_observation(true);
    let a = left.system_ranges(&original,&original,&data).unwrap();
    let b = right.system_ranges(&original,&original,&data).unwrap();
    assert_eq!(a,b);assert_eq!(a.len(),2);
    let mut only_one = data.clone();only_one.rendered_system_starts = vec![1,3,4];only_one.inserted_system_breaks = vec![3];
    assert_eq!(left.system_ranges(&original,&original,&only_one).unwrap(),std::collections::BTreeSet::from([(1,3)]));
    assert_eq!(right.system_ranges(&original,&original,&data).unwrap(),b);
}
