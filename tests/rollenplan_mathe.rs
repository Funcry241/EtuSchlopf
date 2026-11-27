//! Synthetische Modellproben für §§ 3.2a und 6.19.
//!
//! Die Proben rechnen mit vollständigen, hier ausdrücklich gegebenen
//! Planmengen. Sie prüfen den Vertrag und keine produktive Stimmerkennung.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Decision<T> {
    Adopt(T),
    Keep,
    Open,
}

#[derive(Clone, Copy)]
struct Attack {
    onset: i64,
    release: i64,
}

fn within_release_bound(attack: Attack, successor: Attack) -> bool {
    let gap = successor.onset - attack.onset;
    gap > 0 && 2 * (attack.release - successor.onset) <= gap
}

#[derive(Clone, Copy)]
enum Continuation {
    SameLine { next: Attack, following: Attack },
    IndependentLine,
    Unknown,
}

fn weighted_cost<const N: usize>(coefficients: [i64; N], weights: [i64; N]) -> i128 {
    coefficients
        .into_iter()
        .zip(weights)
        .map(|(coefficient, weight)| i128::from(coefficient) * i128::from(weight))
        .sum()
}

// Ein und derselbe zulässige Plan mit Kante dient an allen Ecken als Zeuge.
// Das Minimum der Kostenunterschiede zu den Gegenplänen ist konkav.
fn fixed_plan_separation<const N: usize>(
    fixed: [i64; N],
    with_edge: &[[i64; N]],
    without_edge: &[[i64; N]],
    bounds: [(i64, i64); N],
    tolerance: i128,
) -> bool {
    assert!(N < usize::BITS as usize && tolerance >= 0);
    assert!(bounds.iter().all(|&(lower, upper)| lower <= upper));
    with_edge.contains(&fixed)
        && (0..(1 << N)).all(|corner| {
            let weights = std::array::from_fn(|axis| {
                let (lower, upper) = bounds[axis];
                if corner & (1 << axis) == 0 {
                    lower
                } else {
                    upper
                }
            });
            let fixed_cost = weighted_cost(fixed, weights);
            without_edge
                .iter()
                .all(|&rival| weighted_cost(rival, weights) - fixed_cost > tolerance)
        })
}

fn best_cost<const N: usize>(plans: &[[i64; N]], weights: [i64; N]) -> Option<i128> {
    plans.iter().map(|&plan| weighted_cost(plan, weights)).min()
}

fn duration_decision(
    history: [Attack; 3],
    attack: Attack,
    continuation: Continuation,
    repeated_key: bool,
    present_duration: i64,
    written_gap: i64,
    separated: bool,
) -> Decision<i64> {
    let (next, following) = match continuation {
        Continuation::SameLine { next, following } => (next, following),
        Continuation::IndependentLine => return Decision::Keep,
        Continuation::Unknown => return Decision::Open,
    };
    let stable_history = [history[0], history[1], history[2], attack]
        .windows(2)
        .all(|pair| within_release_bound(pair[0], pair[1]));
    if next.onset <= attack.onset
        || attack.release <= next.onset
        || attack.release >= following.onset
        || !within_release_bound(attack, next)
        || !stable_history
        || repeated_key
        || !(0 < written_gap && written_gap < present_duration)
    {
        return Decision::Keep;
    }
    if separated {
        Decision::Adopt(written_gap)
    } else {
        Decision::Open
    }
}

type Rhythm = [(i64, i64)];

fn synchronous_run(left: &Rhythm, right: &Rhythm, onset: i64) -> usize {
    let l = left.partition_point(|&(time, _)| time < onset);
    let r = right.partition_point(|&(time, _)| time < onset);
    if !matches!(left.get(l), Some(&(time, _)) if time == onset) || left.get(l) != right.get(r) {
        return 0;
    }
    let before = left[..l]
        .iter()
        .rev()
        .zip(right[..r].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let from_onset = left[l..]
        .iter()
        .zip(&right[r..])
        .take_while(|(a, b)| a == b)
        .count();
    before + from_onset
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Partition {
    Separate,
    Xy,
    Xz,
    Yz,
    Xyz,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Plan {
    partition: Partition,
    independent_cost: u32,
    // Zwei identifizierte harte Befunde, jeweils 0 wenn erfüllt.
    findings: [u32; 2],
}

fn group_decision(incumbent: Plan, candidate: Plan, admissible: &[Plan; 5]) -> Decision<Partition> {
    assert!(
        [
            Partition::Separate,
            Partition::Xy,
            Partition::Xz,
            Partition::Yz,
            Partition::Xyz,
        ]
        .iter()
        .all(|&partition| admissible
            .iter()
            .filter(|p| p.partition == partition)
            .count()
            == 1)
    );
    if !admissible.contains(&incumbent) || !admissible.contains(&candidate) {
        return Decision::Open;
    }
    if incumbent.partition == candidate.partition {
        return Decision::Keep;
    }
    let transitions = incumbent.findings.into_iter().zip(candidate.findings);
    if transitions.clone().any(|(old, new)| new > old)
        || !transitions.clone().any(|(old, new)| old > 0 && new == 0)
    {
        return Decision::Keep;
    }
    if admissible.iter().any(|alternative| {
        alternative.partition != candidate.partition
            && alternative.independent_cost <= candidate.independent_cost
    }) {
        return Decision::Open;
    }
    Decision::Adopt(candidate.partition)
}

#[test]
fn equal_performed_overlap_depends_on_independent_line_proof() {
    let history = [
        Attack {
            onset: -300,
            release: -190,
        },
        Attack {
            onset: -200,
            release: -89,
        },
        Attack {
            onset: -100,
            release: 4,
        },
    ];
    let attack = Attack {
        onset: 0,
        release: 135,
    };
    let next = Attack {
        onset: 100,
        release: 209,
    };
    let following = Attack {
        onset: 200,
        release: 302,
    };
    // In beiden Fällen beginnen dieselben Tastenintervalle am Folgeeinsatz.
    assert!(within_release_bound(attack, next));
    assert!(attack.release < following.onset);
    let proved = Continuation::SameLine { next, following };

    // Variierte Dauer-, Tonhöhen-, Raster-, Anschluss- und Eigenendkosten.
    // Die skalierten Gewichte 3 und 5 liegen um 25 % um den Nennwert 4.
    let fixed = [2, 1, 1, 1, 1];
    let with_edge = [fixed, [1, 2, 2, 1, 0]];
    let without_edge = [[3, 1, 1, 1, 2], [2, 3, 2, 1, 2]];
    let bounds = [(3, 5); 5];
    let separated = fixed_plan_separation(fixed, &with_edge, &without_edge, bounds, 5);
    assert!(separated);
    assert!(!fixed_plan_separation(
        fixed,
        &with_edge,
        &without_edge,
        bounds,
        6
    ));
    assert!(!fixed_plan_separation(
        [0, 0, 0, 0, 0],
        &with_edge,
        &without_edge,
        bounds,
        5
    ));
    for (continuation, expected) in [
        (proved, Decision::Adopt(24)),
        (Continuation::IndependentLine, Decision::Keep),
        (Continuation::Unknown, Decision::Open),
    ] {
        assert_eq!(
            duration_decision(history, attack, continuation, false, 48, 24, separated),
            expected
        );
    }
    assert_eq!(
        duration_decision(history, attack, proved, true, 48, 24, separated),
        Decision::Keep
    );

    // Die zwei Pläne mit Kante wechseln sich als Klassenoptimum ab.
    // Die Differenz der Klassenoptima ist an den Ecken positiv.
    let with_edge = [[20, 0], [0, 22]];
    let without_edge = [[10, 11]];
    for weights in [[15, 15], [15, 25], [25, 15], [25, 25]] {
        assert!(best_cost(&without_edge, weights) > best_cost(&with_edge, weights));
    }
    assert_eq!(best_cost(&with_edge, [22, 20]), Some(440));
    assert_eq!(best_cost(&without_edge, [22, 20]), Some(440));
    for fixed in with_edge {
        let separated = fixed_plan_separation(fixed, &with_edge, &without_edge, [(15, 25); 2], 0);
        assert!(!separated);
        assert_eq!(
            duration_decision(history, attack, proved, false, 48, 24, separated),
            Decision::Open
        );
    }
}

#[test]
fn a_unique_synchronous_pair_cannot_write_without_a_render_witness() {
    let x = [(0, 1), (2, 1)];
    let y = [(0, 1), (2, 1)];
    let z = [(0, 1), (2, 2)];
    assert_eq!(
        [
            synchronous_run(&x, &y, 0),
            synchronous_run(&x, &z, 0),
            synchronous_run(&y, &z, 0)
        ],
        [2, 1, 1]
    );
    let incumbent = Plan {
        partition: Partition::Separate,
        independent_cost: 1,
        findings: [1, 0],
    };
    let mut candidate = Plan {
        partition: Partition::Xy,
        independent_cost: 0,
        findings: [1, 0],
    };
    let xz = Plan {
        partition: Partition::Xz,
        independent_cost: 2,
        findings: [1, 0],
    };
    let yz = Plan {
        partition: Partition::Yz,
        independent_cost: 2,
        findings: [1, 0],
    };
    let xyz = Plan {
        partition: Partition::Xyz,
        independent_cost: 2,
        findings: [1, 0],
    };
    assert_eq!(
        group_decision(incumbent, candidate, &[incumbent, candidate, xz, yz, xyz]),
        Decision::Keep
    );

    candidate.findings = [0, 1];
    assert_eq!(
        group_decision(incumbent, candidate, &[incumbent, candidate, xz, yz, xyz]),
        Decision::Keep
    );

    candidate.findings = [0, 0];
    assert_eq!(
        group_decision(incumbent, candidate, &[incumbent, candidate, xz, yz, xyz]),
        Decision::Adopt(Partition::Xy)
    );
    let same_quality = Plan {
        independent_cost: candidate.independent_cost,
        ..incumbent
    };
    assert_eq!(
        group_decision(
            same_quality,
            candidate,
            &[same_quality, candidate, xz, yz, xyz]
        ),
        Decision::Open
    );
    assert_eq!(
        group_decision(
            same_quality,
            candidate,
            &[incumbent, candidate, xz, yz, xyz]
        ),
        Decision::Open
    );
    let stale_candidate = Plan {
        independent_cost: candidate.independent_cost + 1,
        ..candidate
    };
    assert_eq!(
        group_decision(
            incumbent,
            stale_candidate,
            &[incumbent, candidate, xz, yz, xyz]
        ),
        Decision::Open
    );
}

#[test]
fn one_written_duration_can_reverse_the_proposed_pair() {
    let x = [(0, 1), (2, 1), (4, 1), (6, 1), (8, 1)];
    let mut y = [(0, 1), (2, 1), (4, 1), (6, 1)];
    let z = [(4, 1), (6, 1), (8, 1)];
    let before = (synchronous_run(&x, &y, 4), synchronous_run(&x, &z, 4));
    assert_eq!(before, (4, 3));
    y[1].1 = 2;
    let after = (synchronous_run(&x, &y, 4), synchronous_run(&x, &z, 4));
    assert_eq!(after, (2, 3));
}

#[test]
fn equal_cost_pairs_and_three_note_groups_leave_the_decision_open() {
    let x = [(0, 2), (2, 1), (3, 1), (4, 1)];
    let y = [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)];
    let z = [(0, 1), (1, 1), (2, 1), (3, 2)];
    assert_eq!(
        [
            synchronous_run(&x, &y, 2),
            synchronous_run(&x, &z, 2),
            synchronous_run(&y, &z, 2)
        ],
        [3, 1, 3]
    );
    let incumbent = Plan {
        partition: Partition::Separate,
        independent_cost: 1,
        findings: [1, 0],
    };
    let candidate = Plan {
        partition: Partition::Xy,
        independent_cost: 0,
        findings: [0, 0],
    };
    let xz = Plan {
        partition: Partition::Xz,
        independent_cost: 2,
        findings: [1, 0],
    };
    let mut yz = Plan {
        partition: Partition::Yz,
        independent_cost: 0,
        findings: [0, 0],
    };
    let mut xyz = Plan {
        partition: Partition::Xyz,
        independent_cost: 2,
        findings: [1, 0],
    };
    assert_eq!(
        group_decision(incumbent, candidate, &[incumbent, candidate, xz, yz, xyz]),
        Decision::Open
    );

    yz.independent_cost = 1;
    xyz.independent_cost = 0;
    xyz.findings = [0, 0];
    assert_eq!(
        group_decision(incumbent, candidate, &[incumbent, candidate, xz, yz, xyz]),
        Decision::Open
    );
}
