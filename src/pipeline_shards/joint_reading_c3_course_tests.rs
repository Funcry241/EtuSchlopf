use super::*;

#[test]
fn carry_through_a_quarter_bar_does_not_create_an_artificial_short_section() {
    let mut d = domain(vec![layer(60, (1, 4), 0)]);
    d.entry = Entry::Carry {
        state: state(0),
        elapsed: Time::new(1, 2).unwrap(),
    };
    d.exit = Exit::Carry {
        state: state(0),
        remaining: Time::new(1, 2).unwrap(),
    };
    d.layers[0].allowed = vec![state(0)];
    assert_eq!(solve(&d, None).unwrap().selected_cost.0[7], 0);
}

#[test]
fn carry_uses_the_complete_length_on_either_or_both_sides() {
    for (before, after, expected) in [
        ((1, 4), (1, 4), 1), // Complete section is 3/4 bar.
        ((1, 2), (1, 4), 0), // Exactly one bar, not short.
        ((3, 4), (3, 4), 0),
    ] {
        let mut d = domain(vec![layer(28, (1, 4), 0)]);
        d.layers[0].allowed = vec![state(0)];
        d.entry = Entry::Carry {
            state: state(0),
            elapsed: Time::new(before.0, before.1).unwrap(),
        };
        d.exit = Exit::Carry {
            state: state(0),
            remaining: Time::new(after.0, after.1).unwrap(),
        };
        assert_eq!(solve(&d, None).unwrap().selected_cost.0[7], expected);
        assert_eq!(oracle_cost(&d, &[state(0)]).unwrap().0[7], expected);
    }
    let mut d = domain(vec![layer(28, (1, 4), 0)]);
    d.layers[0].allowed = vec![state(0)];
    d.entry = Entry::Carry {
        state: state(0),
        elapsed: Time::new(3, 4).unwrap(),
    };
    assert_eq!(solve(&d, None).unwrap().selected_cost.0[7], 0);
    d.entry = Entry::Initial;
    d.exit = Exit::Carry {
        state: state(0),
        remaining: Time::new(3, 4).unwrap(),
    };
    assert_eq!(solve(&d, None).unwrap().selected_cost.0[7], 0);
}

#[test]
fn a_boundary_switch_cannot_make_the_short_external_piece_disappear() {
    let mut d = domain(vec![layer(28, (1, 1), 5)]);
    d.layers[0].allowed = vec![state(5)];
    d.entry = Entry::Carry {
        state: state(0),
        elapsed: Time::new(1, 4).unwrap(),
    };
    assert_eq!(solve(&d, None).unwrap().selected_cost.0[7], 1);
    d.entry = Entry::Initial;
    d.exit = Exit::Carry {
        state: state(0),
        remaining: Time::new(1, 4).unwrap(),
    };
    assert_eq!(solve(&d, None).unwrap().selected_cost.0[7], 1);
}

#[test]
fn search_does_not_move_the_switch_to_the_cut_to_avoid_a_fake_short_section() {
    let mut d = domain(vec![layer(28, (1, 4), 0), layer(24, (1, 1), 5)]);
    d.entry = Entry::Carry {
        state: state(0),
        elapsed: Time::new(3, 4).unwrap(),
    };
    d.layers[0].allowed = vec![state(0), state(5)];
    d.layers[1].allowed = vec![state(5)];
    d.layers[1].group_crossing = true;
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected, vec![state(0), state(5)]);
    assert_eq!(result.selected_cost.0[7], 0);
    assert_eq!(oracle_cost(&d, &[state(5), state(5)]).unwrap().0[7], 1);
}

#[test]
fn unknown_carry_length_cannot_be_silently_supplied_as_zero() {
    let mut d = domain(vec![layer(28, (1, 1), 0)]);
    d.entry = Entry::Carry {
        state: state(0),
        elapsed: Time::ZERO,
    };
    assert!(matches!(solve(&d, None), Err(Error::Invalid(_))));
    d.entry = Entry::Initial;
    d.exit = Exit::Carry {
        state: state(0),
        remaining: Time::ZERO,
    };
    assert!(matches!(solve(&d, None), Err(Error::Invalid(_))));
}

#[test]
fn compact_carry_context_matches_explicit_fixed_prefix_and_suffix_layers() {
    for before in [(1, 4), (1, 2), (3, 4), (1, 1)] {
        for after in [(1, 4), (1, 2), (3, 4), (1, 1)] {
            let mut compact = domain(vec![
                layer(28, (1, 4), 0),
                layer(24, (1, 2), 5),
                layer(28, (1, 4), 0),
            ]);
            compact.entry = Entry::Carry {
                state: state(0),
                elapsed: Time::new(before.0, before.1).unwrap(),
            };
            compact.exit = Exit::Carry {
                state: state(0),
                remaining: Time::new(after.0, after.1).unwrap(),
            };
            for l in &mut compact.layers {
                l.allowed = vec![state(0), state(5)];
            }
            let mut prefix = layer(28, before, 0);
            prefix.heads.clear();
            prefix.allowed = vec![state(0)];
            let mut suffix = layer(28, after, 0);
            suffix.heads.clear();
            suffix.allowed = vec![state(0)];
            let mut expanded = domain(vec![prefix]);
            expanded.layers.extend(compact.layers.clone());
            expanded.layers.push(suffix);
            let a = solve(&compact, None).unwrap();
            let b = solve(&expanded, None).unwrap();
            assert_eq!(a.incumbent_cost, b.incumbent_cost);
            assert_eq!(a.selected_cost, b.selected_cost);
            assert_eq!(a.selected, b.selected[1..b.selected.len() - 1]);
            assert_eq!(a.improved, b.improved);
            assert_eq!((a.selected_cost, a.selected), enumerate(&compact));
        }
    }
}

fn state(i: u8) -> State {
    State::new(i).unwrap()
}
fn layer(pitch: i32, duration: (u128, u128), incumbent: u8) -> Layer {
    Layer {
        duration: Time::new(duration.0, duration.1).unwrap(),
        incumbent: state(incumbent),
        heads: vec![Head {
            diatonic: pitch,
            visible: true,
        }],
        allowed: (0..10).map(state).collect(),
        group_crossing: false,
        incoming: [[true; 10]; 10],
    }
}
fn domain(layers: Vec<Layer>) -> Domain {
    Domain {
        entry: Entry::Initial,
        exit: Exit::End,
        layers,
        exit_allowed: [true; 10],
        exit_group_crossing: false,
    }
}

// Whole-path oracle. It does not call the production cost/transition/extension helpers.
fn oracle_cost(d: &Domain, path: &[State]) -> Option<Cost> {
    let mut costs = [0u64; 13];
    let mut unit = 1u128;
    for l in &d.layers {
        unit = unit / gcd(unit, l.duration.d) * l.duration.d;
    }
    let mut timeline = Vec::new();
    if let Entry::Carry { state, elapsed } = d.entry {
        unit = unit / gcd(unit, elapsed.d) * elapsed.d;
        timeline.push((state, elapsed));
    }
    for (layer, state) in d.layers.iter().zip(path) {
        timeline.push((*state, layer.duration));
    }
    if let Exit::Carry { state, remaining } = d.exit {
        unit = unit / gcd(unit, remaining.d) * remaining.d;
        timeline.push((state, remaining));
    }
    let clef = |s: State| s.index() / 5;
    let octave = |s: State| match s.index() % 5 {
        0 => 0i64,
        1 => 1,
        2 => -1,
        3 => 2,
        _ => -2,
    };
    let ledgers = |s: State, pitch: i32| {
        let position = i64::from(pitch) - 7 * octave(s);
        let lines = if clef(s) == 0 {
            [30, 32, 34, 36, 38]
        } else {
            [18, 20, 22, 24, 26]
        };
        if position < lines[0] {
            (lines[0] - position) as u64 / 2
        } else if position > lines[4] {
            (position - lines[4]) as u64 / 2
        } else {
            0
        }
    };
    let mut borders = Vec::new();
    match d.entry {
        Entry::Initial => costs[10] += u64::from(octave(path[0]) != 0),
        Entry::Carry { state: from, .. } => {
            if !d.layers[0].incoming[from.index() as usize][path[0].index() as usize] {
                return None;
            }
            borders.push((from, path[0], d.layers[0].group_crossing));
        }
    }
    for (i, (l, &s)) in d.layers.iter().zip(path).enumerate() {
        if !l.allowed.contains(&s) {
            return None;
        }
        if i != 0 {
            let before = path[i - 1];
            if !l.incoming[before.index() as usize][s.index() as usize] {
                return None;
            }
            borders.push((before, s, l.group_crossing));
        }
        for head in l.heads.iter().filter(|h| h.visible) {
            let count = ledgers(s, head.diatonic);
            if count >= 4 && count > ledgers(l.incumbent, head.diatonic) {
                return None;
            }
            match count {
                0 => (),
                1 => costs[12] += 1,
                2 => costs[11] += 1,
                3 => costs[9] += 1,
                4 => costs[4] += 1,
                5 => costs[3] += 1,
                6 => costs[2] += 1,
                7 => costs[1] += 1,
                _ => costs[0] += 1,
            }
            costs[6] +=
                u64::from((clef(s) == 0 && octave(s) < 0) || (clef(s) == 1 && octave(s) > 0));
        }
    }
    // Independent whole contextual timeline: merge maximal constant sections,
    // then count their full lengths. No production age/entry/exit helper is used.
    let mut sections: Vec<(State, u128)> = Vec::new();
    for (state, duration) in timeline {
        let length = duration.n * (unit / duration.d);
        match sections.last_mut() {
            Some((previous, total)) if *previous == state => *total += length,
            _ => sections.push((state, length)),
        }
    }
    costs[7] = sections.iter().filter(|(_, length)| *length < unit).count() as u64;
    let last = *path.last().unwrap();
    if !d.exit_allowed[last.index() as usize] {
        return None;
    }
    match d.exit {
        Exit::End => costs[10] += u64::from(octave(last) != 0),
        Exit::Carry { state: to, .. } => borders.push((last, to, d.exit_group_crossing)),
    }
    for (a, b, group) in borders {
        let change_clef = clef(a) != clef(b);
        let change_octave = octave(a) != octave(b);
        costs[5] += u64::from(change_clef && change_octave);
        costs[8] += u64::from(a != b && group);
        costs[10] += u64::from(change_clef)
            + u64::from(change_octave && octave(a) != 0)
            + u64::from(change_octave && octave(b) != 0);
    }
    Some(Cost(costs))
}

fn enumerate(d: &Domain) -> (Cost, Vec<State>) {
    fn visit(d: &Domain, path: &mut Vec<State>, best: &mut Option<(Cost, u64, Vec<State>)>) {
        if path.len() < d.layers.len() {
            for s in &d.layers[path.len()].allowed {
                path.push(*s);
                visit(d, path, best);
                path.pop();
            }
        } else if let Some(cost) = oracle_cost(d, path) {
            let changed = path
                .iter()
                .zip(&d.layers)
                .filter(|(s, l)| !l.heads.is_empty() && **s != l.incumbent)
                .count() as u64;
            let candidate = (cost, changed, path.clone());
            if best.as_ref().is_none_or(|old| candidate < *old) {
                *best = Some(candidate);
            }
        }
    }
    let original: Vec<_> = d.layers.iter().map(|l| l.incumbent).collect();
    let original_cost = oracle_cost(d, &original).unwrap();
    let mut best = None;
    visit(d, &mut Vec::new(), &mut best);
    let (cost, _, path) = best.unwrap();
    if cost < original_cost {
        (cost, path)
    } else {
        (original_cost, original)
    }
}

#[test]
fn rational_comparison_and_addition_are_exact_near_integer_limits() {
    let a = Time::new(u128::MAX - 1, u128::MAX).unwrap();
    let b = Time::new(u128::MAX - 2, u128::MAX - 1).unwrap();
    assert!(a > b && a < Time::ONE);
    assert_eq!(a.aged(b), Ok(Time::ONE));
    assert_eq!(
        Time::new(1, 3).unwrap().add(Time::new(1, 6).unwrap()),
        Time::new(1, 2)
    );
    for a in 0..20 {
        for b in 1..20 {
            for c in 0..20 {
                for d in 1..20 {
                    assert_eq!(
                        Time::new(a, b).unwrap().cmp(&Time::new(c, d).unwrap()),
                        (a * d).cmp(&(c * b))
                    );
                }
            }
        }
    }
}

#[test]
fn written_spelling_and_double_octaves_have_exact_staff_mapping() {
    assert_eq!(state(0).ledgers(30), Ok(0)); // E4 on first G line.
    assert_eq!(state(0).ledgers(29), Ok(0)); // D4 in adjacent space.
    assert_eq!(state(0).ledgers(28), Ok(1)); // C4 ledger.
    assert_eq!(state(0).ledgers(39), Ok(0));
    assert_eq!(state(0).ledgers(40), Ok(1));
    assert_eq!(state(5).ledgers(18), Ok(0)); // G2 first F line.
    assert_eq!(state(1).ledgers(44), Ok(0));
    assert_eq!(state(3).ledgers(44), Ok(0));
    assert_ne!(state(1).ledgers(52), state(3).ledgers(52));
    // A diatonic spelling difference can change ledger demand at the same chromatic pitch.
    assert_ne!(state(0).ledgers(28), state(0).ledgers(29));
}

#[test]
fn single_first_last_and_exact_bar_sections_have_expected_costs() {
    for (duration, short) in [((1, 2), 1), ((1, 1), 0), ((3, 2), 0)] {
        let mut d = domain(vec![layer(34, duration, 0)]);
        d.layers[0].allowed = vec![state(0)];
        let result = solve(&d, None).unwrap();
        assert_eq!(result.selected_cost.0[7], short);
    }
    let mut d = domain(vec![layer(34, (1, 2), 0), layer(24, (1, 2), 5)]);
    for l in &mut d.layers {
        l.allowed = vec![l.incumbent];
    }
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected_cost.0[7], 2);
    assert_eq!(result.selected_cost.0[10], 1);
}

#[test]
fn existing_high_ledger_load_is_allowed_but_new_or_worse_load_is_not() {
    let mut d = domain(vec![layer(54, (1, 1), 0)]);
    d.layers[0].allowed = vec![state(0), state(5), state(3)];
    assert_eq!(state(0).ledgers(54), Ok(8));
    assert_eq!(state(5).ledgers(54), Ok(14));
    let result = solve(&d, None).unwrap();
    assert!(result.improved);
    assert_eq!(result.selected, vec![state(3)]);
    d.layers[0].allowed = vec![state(0), state(5)];
    let result = solve(&d, None).unwrap();
    assert!(!result.improved);
    assert_eq!(result.selected_cost.0[0], 1);
}

#[test]
fn hidden_carriers_do_not_add_visible_head_costs() {
    let mut d = domain(vec![layer(34, (1, 1), 0)]);
    d.layers[0].heads.push(Head {
        diatonic: i32::MIN,
        visible: false,
    });
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected_cost, Cost::default());
    assert!(!result.improved);
}

#[test]
fn simultaneous_component_changes_and_non_loco_returns_are_not_free() {
    let mut d = domain(vec![layer(24, (1, 1), 5)]);
    d.entry = Entry::Carry {
        state: state(1),
        elapsed: Time::ONE,
    };
    d.exit = Exit::Carry {
        state: state(3),
        remaining: Time::ONE,
    };
    d.exit_group_crossing = true;
    d.layers[0].allowed = vec![state(5)];
    d.layers[0].group_crossing = true;
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected_cost.0[5], 2); // Two clef + mapping transitions.
    assert_eq!(result.selected_cost.0[8], 2);
    assert_eq!(result.selected_cost.0[10], 4);
    assert_eq!(
        Some(result.selected_cost),
        oracle_cost(&d, &result.selected)
    );
}

#[test]
fn necessary_double_octave_has_start_and_end_operations() {
    let mut d = domain(vec![layer(51, (1, 1), 3)]);
    d.layers[0].allowed = vec![state(3)];
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected_cost.0[10], 2);
    assert_eq!(result.selected_cost.0[6], 0);
    d.layers[0].incumbent = state(9);
    d.layers[0].allowed = vec![state(9)];
    d.layers[0].heads[0].diatonic = 10;
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected_cost.0[10], 2);
    assert_eq!(result.selected_cost.0[6], 0);
}

#[test]
fn incumbent_missing_unsupported_state_and_zero_time_are_not_silent_success() {
    assert!(State::new(10).is_err());
    let mut d = domain(vec![layer(34, (1, 1), 0)]);
    d.layers[0].allowed = vec![state(5)];
    assert!(matches!(solve(&d, None), Err(Error::Invalid(_))));
    d.layers[0].allowed = vec![state(0)];
    d.layers[0].duration = Time::ZERO;
    assert!(matches!(solve(&d, None), Err(Error::Invalid(_))));
}

#[test]
fn resource_abort_returns_only_the_complete_incumbent() {
    let d = domain(vec![layer(34, (1, 2), 5), layer(34, (1, 2), 5)]);
    let result = solve(&d, Some(0)).unwrap();
    assert!(!result.complete);
    assert!(!result.improved);
    assert_eq!(result.selected, vec![state(5), state(5)]);
    assert_eq!(result.selected_cost, result.incumbent_cost);
}

#[test]
fn equal_primary_cost_keeps_the_exact_incumbent() {
    let mut d = domain(vec![layer(28, (1, 1), 0)]);
    d.layers[0].allowed = vec![state(0), state(5)];
    // C4 requires one ledger in either clef, and an initial clef is free.
    assert_eq!(oracle_cost(&d, &[state(0)]), oracle_cost(&d, &[state(5)]));
    let result = solve(&d, None).unwrap();
    assert!(!result.improved);
    assert_eq!(result.selected, vec![state(0)]);
}

#[test]
fn exact_search_matches_independent_whole_path_enumeration() {
    for seed in 0..256u64 {
        let mut value = seed + 1;
        let mut layers = Vec::new();
        for i in 0..4 {
            value = value.wrapping_mul(6364136223846793005).wrapping_add(1);
            let choices = [state(0), state(5), state(if seed % 2 == 0 { 1 } else { 7 })];
            let original = choices[((value >> 32) % 3) as usize];
            let mut l = layer(
                16 + ((value >> 16) % 33) as i32,
                (1 + u128::from((value >> 8) % 3), 4),
                original.index(),
            );
            l.allowed = choices.to_vec();
            l.group_crossing = (seed + i) % 3 == 0;
            if seed % 5 == 0 {
                l.heads.push(Head {
                    diatonic: l.heads[0].diatonic + 4,
                    visible: true,
                });
            }
            if seed % 7 == 0 {
                l.heads.clear();
            } // Genuine interval in silence remains in the graph.
            layers.push(l);
        }
        let mut d = domain(layers);
        if seed % 3 == 0 {
            d.entry = Entry::Carry {
                state: state(0),
                elapsed: Time::new(1, 2).unwrap(),
            };
            d.exit = Exit::Carry {
                state: state(5),
                remaining: Time::new(3, 4).unwrap(),
            };
        }
        let expected = enumerate(&d);
        let actual = solve(&d, None).unwrap();
        assert_eq!(
            (actual.selected_cost, actual.selected),
            expected,
            "synthetic seed {seed}"
        );
    }
}

#[test]
fn duration_partition_and_allowed_state_order_do_not_change_the_course() {
    let mut a = domain(vec![layer(34, (1, 2), 5), layer(34, (1, 2), 5)]);
    let expected = solve(&a, None).unwrap();
    for l in &mut a.layers {
        l.allowed.reverse();
    }
    assert_eq!(solve(&a, None).unwrap().selected, expected.selected);
    let mut b = domain(vec![layer(34, (1, 1), 5)]);
    b.layers[0].allowed.reverse();
    let actual = solve(&b, None).unwrap();
    assert_eq!(actual.selected_cost, expected.selected_cost);
    assert_eq!(actual.selected[0], expected.selected[0]);
}

#[test]
fn owner_edge_restrictions_are_preserved() {
    let mut d = domain(vec![layer(34, (1, 2), 0), layer(24, (1, 2), 0)]);
    d.layers[0].allowed = vec![state(0)];
    d.layers[1].incoming[0][5] = false;
    let result = solve(&d, None).unwrap();
    assert_ne!(result.selected[1], state(5));
    let expected = enumerate(&d);
    assert_eq!((result.selected_cost, result.selected), expected);
}

#[test]
fn uncapped_ledger_readout_does_not_mislabel_h8_as_exactly_eight() {
    let mut d = domain(vec![layer(60, (1, 1), 0)]);
    d.layers[0].allowed = vec![state(0)];
    d.layers[0].heads.push(Head {
        diatonic: 34,
        visible: true,
    });
    let result = solve(&d, None).unwrap();
    let metrics = ledger_metrics(&d, &result.selected).unwrap();
    assert_eq!(result.selected_cost.0[0], 1);
    assert_eq!(metrics.maximum, 11);
    assert_eq!(metrics.sum, 11);
    assert_eq!(metrics.visible_heads, 2);
    assert_eq!(metrics.histogram, BTreeMap::from([(0, 1), (11, 1)]));
}

#[test]
fn independent_domains_remain_independent_when_duplicated_or_perturbed() {
    let a = domain(vec![layer(34, (1, 2), 5), layer(37, (1, 2), 5)]);
    let b = domain(vec![layer(20, (1, 1), 0), layer(22, (1, 1), 0)]);
    let baseline = solve(&a, None).unwrap();
    let mut results = Vec::new();
    for domain in [&a, &b, &a] {
        results.push(solve(domain, None).unwrap());
    }
    assert_eq!(results[0], baseline);
    assert_eq!(results[2], baseline);
    assert_eq!(solve(&a, None).unwrap(), baseline);
}

#[test]
fn equal_register_states_with_different_ages_have_different_legal_future_costs() {
    let prefix = Label {
        cost: Cost::default(),
        changed_layers: 0,
        path: vec![state(0)],
    };
    let next = layer(24, (1, 4), 5);
    let (_, long) = extend(&prefix, Time::ONE, &next, state(5))
        .unwrap()
        .unwrap();
    let (_, short) = extend(&prefix, Time::new(1, 2).unwrap(), &next, state(5))
        .unwrap()
        .unwrap();
    assert_eq!(long.cost.0[7], 0);
    assert_eq!(short.cost.0[7], 1);
    assert!(long.cost < short.cost);
}

#[test]
fn changing_meter_coordinates_sum_exactly_without_a_fixed_tick_bar() {
    let mut d = domain(vec![
        layer(34, (1, 3), 0),
        layer(34, (1, 6), 0),
        layer(34, (1, 2), 0),
    ]);
    for l in &mut d.layers {
        l.allowed = vec![state(0)];
    }
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected_cost.0[7], 0);
    d.layers[2].duration = Time::new(1, 3).unwrap();
    let result = solve(&d, None).unwrap();
    assert_eq!(result.selected_cost.0[7], 1);
}

#[test]
fn policy_priority_is_lexicographic_and_never_a_weighted_tradeoff() {
    let mut severe = Cost::default();
    severe.0[4] = 1;
    let mut many_lesser = Cost::default();
    many_lesser.0[5] = 1_000_000;
    assert!(many_lesser < severe);
    let mut unconventional = Cost::default();
    unconventional.0[6] = 1;
    let mut many_short = Cost::default();
    many_short.0[7] = 1_000_000;
    assert!(many_short < unconventional);
}
