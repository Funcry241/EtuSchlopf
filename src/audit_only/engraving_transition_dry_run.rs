//! Synthetic decision cases for a source-bound engraving transition.
//!
//! The fixture supplies observations and a complete neighborhood explicitly.
//! Production receipts and PDF provenance are outside this model.

use std::collections::{BTreeMap, BTreeSet};

type Group = usize;
type Element = usize;
type Pair = (Element, Element);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

#[derive(Clone, Copy)]
enum Distance {
    Separated { lower: f64, upper: f64 },
    Overlap,
    Unknown,
}

impl Distance {
    fn bounds(self) -> Option<(f64, f64)> {
        match self {
            Self::Separated { lower, upper }
                if lower.is_finite() && upper.is_finite() && lower > 0.0 && lower <= upper =>
            {
                Some((lower, upper))
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Ink<'a> {
    element: Element,
    page: usize,
    contour: &'a [i32],
}

#[derive(Clone, Copy)]
enum PairObservation<'a> {
    SameInk {
        before: [Ink<'a>; 2],
        after: [Ink<'a>; 2],
    },
    Distances {
        before: Distance,
        after: Distance,
    },
}

impl PairObservation<'_> {
    fn proves_preservation(self, pair: Pair, minimum: f64) -> bool {
        let (left, right) = pair;
        if !minimum.is_finite() || minimum <= 0.0 {
            return false;
        }
        match self {
            Self::SameInk { before, after } => {
                left != right
                    && before.map(|ink| ink.element) == [left, right]
                    && before.iter().all(|ink| !ink.contour.is_empty())
                    && before == after
            }
            Self::Distances { before, after } => {
                let Some((lower, _)) = after.bounds() else {
                    return false;
                };
                lower >= minimum
                    || matches!(before, Distance::Overlap)
                    || before.bounds().is_some_and(|(_, upper)| lower >= upper)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Frame<'a> {
    source: &'a [u8],
    rendered: &'a [u8],
}

struct Proposal {
    target: Element,
    direction: Direction,
    pairs: BTreeSet<Pair>,
}

struct Draft {
    proposals: BTreeMap<Group, Proposal>,
    // Derived upstream from shared affected pairs or displacement in the
    // common render. Ties between groups are a partition, never a ranking.
    links: Vec<(Group, Group)>,
    minimum_gaps: BTreeMap<Pair, f64>,
}

struct Readback<'a> {
    before: Frame<'a>,
    after: Frame<'a>,
    included: BTreeSet<Group>,
    before_stems: BTreeMap<Element, Direction>,
    after_stems: BTreeMap<Element, Direction>,
    preserved_groups: BTreeSet<Group>,
    scope: BTreeMap<Group, BTreeSet<Pair>>,
    pairs: BTreeMap<Pair, PairObservation<'a>>,
}

impl Draft {
    fn group_proven(&self, group: Group, readback: &Readback<'_>) -> bool {
        let proposal = &self.proposals[&group];
        readback.preserved_groups.contains(&group)
            && readback.scope.get(&group) == Some(&proposal.pairs)
            && readback
                .before_stems
                .get(&proposal.target)
                .is_some_and(|direction| *direction != proposal.direction)
            && readback.after_stems.get(&proposal.target) == Some(&proposal.direction)
            && proposal.pairs.iter().all(|pair| {
                self.minimum_gaps
                    .get(pair)
                    .zip(readback.pairs.get(pair))
                    .is_some_and(|(minimum, observed)| {
                        observed.proves_preservation(*pair, *minimum)
                    })
            })
    }

    fn screen<'a>(
        &'a self,
        baseline: Frame<'a>,
        combined: Frame<'_>,
        readback: &Readback<'_>,
    ) -> Selection<'a> {
        let mut included = BTreeSet::new();
        let proposals: BTreeSet<_> = self.proposals.keys().copied().collect();
        let targets: BTreeSet<_> = self
            .proposals
            .values()
            .map(|proposal| proposal.target)
            .collect();
        if readback.before != baseline
            || readback.after != combined
            || readback.included != proposals
            || targets.len() != proposals.len()
            || self
                .links
                .iter()
                .any(|(left, right)| !proposals.contains(left) || !proposals.contains(right))
        {
            return Selection {
                draft: self,
                baseline,
                included,
            };
        }
        let mut remaining = proposals;
        while let Some(group) = remaining.pop_first() {
            let mut component = BTreeSet::from([group]);
            let mut pending = vec![group];
            while let Some(group) = pending.pop() {
                for &(left, right) in &self.links {
                    let neighbor = if left == group {
                        Some(right)
                    } else if right == group {
                        Some(left)
                    } else {
                        None
                    };
                    if let Some(neighbor) = neighbor.filter(|neighbor| remaining.remove(neighbor)) {
                        component.insert(neighbor);
                        pending.push(neighbor);
                    }
                }
            }
            if component
                .iter()
                .all(|&group| self.group_proven(group, readback))
            {
                included.extend(component);
            }
        }
        Selection {
            draft: self,
            baseline,
            included,
        }
    }
}

struct Selection<'a> {
    draft: &'a Draft,
    baseline: Frame<'a>,
    included: BTreeSet<Group>,
}

impl<'a> Selection<'a> {
    /// Verify the already selected subset against the exact output. Failure
    /// retains the previously witnessed baseline without selecting again.
    fn finish(self, published: Frame<'a>, readback: &Readback<'a>) -> Frame<'a> {
        if !self.included.is_empty()
            && readback.before == self.baseline
            && readback.after == published
            && readback.included == self.included
            && self
                .included
                .iter()
                .all(|&group| self.draft.group_proven(group, readback))
        {
            published
        } else {
            self.baseline
        }
    }
}

#[test]
fn distance_evidence_distinguishes_clear_unchanged_and_unresolved_pairs() {
    let separated = |lower, upper| Distance::Separated { lower, upper };
    let measured_distances = |before, after| PairObservation::Distances { before, after };
    for (pair, accepted) in [
        (
            measured_distances(Distance::Unknown, Distance::Unknown),
            false,
        ),
        (
            measured_distances(Distance::Overlap, Distance::Overlap),
            false,
        ),
        (
            measured_distances(Distance::Unknown, separated(3.0, 4.0)),
            true,
        ),
        (
            measured_distances(Distance::Unknown, separated(1.0, 2.0)),
            false,
        ),
        (
            measured_distances(Distance::Overlap, separated(1.0, 2.0)),
            true,
        ),
        (
            measured_distances(separated(1.0, 2.0), separated(2.0, 2.5)),
            true,
        ),
        (
            measured_distances(separated(1.0, 2.0), separated(1.5, 2.5)),
            false,
        ),
        (
            measured_distances(Distance::Overlap, separated(0.0, 1.0)),
            false,
        ),
        (
            measured_distances(Distance::Overlap, separated(1.0, f64::INFINITY)),
            false,
        ),
    ] {
        assert_eq!(pair.proves_preservation((10, 90), 3.0), accepted);
    }
}

#[test]
fn exact_ink_exception_requires_the_same_anchored_contours_on_the_same_pages() {
    let stem = Ink {
        element: 10,
        page: 0,
        contour: &[0, 0, 0, 12, 1, 12, 1, 0],
    };
    let foreign = Ink {
        element: 90,
        page: 0,
        contour: &[2, 3, 2, 5, 6, 5, 6, 3],
    };
    let before = [stem, foreign];
    let same = |after| PairObservation::SameInk { before, after };
    assert!(same(before).proves_preservation((10, 90), 3.0));
    assert!(!same(before).proves_preservation((10, 91), 3.0));
    assert!(!same([foreign, stem]).proves_preservation((10, 90), 3.0));
    let moved = Ink { page: 1, ..foreign };
    assert!(!same([stem, moved]).proves_preservation((10, 90), 3.0));
    let changed = Ink {
        contour: &[3, 3, 3, 5, 7, 5, 7, 3],
        ..foreign
    };
    assert!(!same([stem, changed]).proves_preservation((10, 90), 3.0));
}

const BEFORE: Frame<'static> = Frame {
    source: b"two authored up stems",
    rendered: b"both up",
};
const BOTH: Frame<'static> = Frame {
    source: b"two authored down stems",
    rendered: b"both down",
};
const FIRST: Frame<'static> = Frame {
    source: b"first authored down stem",
    rendered: b"first down",
};

fn draft() -> Draft {
    Draft {
        proposals: BTreeMap::from([
            (
                1,
                Proposal {
                    target: 10,
                    direction: Direction::Down,
                    pairs: BTreeSet::from([(10, 90)]),
                },
            ),
            (
                2,
                Proposal {
                    target: 20,
                    direction: Direction::Down,
                    pairs: BTreeSet::from([(20, 91)]),
                },
            ),
        ]),
        links: Vec::new(),
        minimum_gaps: BTreeMap::from([((10, 90), 3.0), ((20, 91), 3.0)]),
    }
}

fn observed_after(frame: Frame<'static>, both: bool) -> Readback<'static> {
    let separated = Distance::Separated {
        lower: 4.0,
        upper: 5.0,
    };
    Readback {
        before: BEFORE,
        after: frame,
        included: if both {
            BTreeSet::from([1, 2])
        } else {
            BTreeSet::from([1])
        },
        before_stems: BTreeMap::from([(10, Direction::Up), (20, Direction::Up)]),
        after_stems: BTreeMap::from([
            (10, Direction::Down),
            (20, if both { Direction::Down } else { Direction::Up }),
        ]),
        preserved_groups: BTreeSet::from([1, 2]),
        scope: BTreeMap::from([
            (1, BTreeSet::from([(10, 90)])),
            (2, BTreeSet::from([(20, 91)])),
        ]),
        pairs: BTreeMap::from([(
            (10, 90),
            PairObservation::Distances {
                before: separated,
                after: separated,
            },
        )]),
    }
}

#[test]
fn incomplete_independent_group_does_not_erase_the_proven_subset() {
    let draft = draft();
    let proposal = observed_after(BOTH, true);
    let selection = draft.screen(BEFORE, BOTH, &proposal);
    assert_eq!(selection.included, BTreeSet::from([1]));
    let actual = observed_after(FIRST, false);
    assert_eq!(selection.finish(FIRST, &actual), FIRST);
}

#[test]
fn coupled_component_is_rejected_together() {
    let mut draft = draft();
    draft.links.push((1, 2));
    let proposal = observed_after(BOTH, true);
    let selection = draft.screen(BEFORE, BOTH, &proposal);
    assert!(selection.included.is_empty());
    assert_eq!(
        selection.finish(FIRST, &observed_after(FIRST, false)),
        BEFORE
    );
}

#[test]
fn final_readback_must_match_the_preselected_subset_and_exact_files() {
    let draft = draft();
    let proposal = observed_after(BOTH, true);
    let actual = observed_after(FIRST, false);
    assert_eq!(
        draft
            .screen(BEFORE, BOTH, &proposal)
            .finish(BOTH, &proposal),
        BEFORE
    );
    let altered = Frame {
        source: FIRST.source,
        rendered: b"wrong PDF",
    };
    assert_eq!(
        draft
            .screen(BEFORE, BOTH, &proposal)
            .finish(altered, &actual),
        BEFORE
    );
    let altered = Frame {
        source: b"wrong source",
        rendered: FIRST.rendered,
    };
    assert_eq!(
        draft
            .screen(BEFORE, BOTH, &proposal)
            .finish(altered, &actual),
        BEFORE
    );
    let mut invalid = observed_after(FIRST, false);
    invalid.preserved_groups.remove(&1);
    assert_eq!(
        draft
            .screen(BEFORE, BOTH, &proposal)
            .finish(FIRST, &invalid),
        BEFORE
    );
    invalid.preserved_groups.insert(1);
    invalid.before_stems.insert(10, Direction::Down);
    assert_eq!(
        draft
            .screen(BEFORE, BOTH, &proposal)
            .finish(FIRST, &invalid),
        BEFORE
    );
}

#[test]
fn an_unexpected_affected_pair_refuses_the_selected_output() {
    let draft = draft();
    let proposal = observed_after(BOTH, true);
    let mut actual = observed_after(FIRST, false);
    actual.scope.get_mut(&1).unwrap().insert((10, 92));
    assert_eq!(
        draft.screen(BEFORE, BOTH, &proposal).finish(FIRST, &actual),
        BEFORE
    );
}
