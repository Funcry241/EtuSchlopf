//! Verdicts for an already authenticated pair of drawn marks.
//!
//! The caller binds both marks to the authored score in both frames, proves
//! the inventory is complete, and derives all interactions between proposals.
//! Rectangles enclose ink but their intersection is not an ink intersection.

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Decision {
    Proven,
    Unknown,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PairRole {
    OwnedAttachment,
    Foreign,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Contact {
    /// The actual painted contours have a witnessed common interior. An own
    /// attachment requires that witness at its intended attachment region.
    Present,
    /// The attachment is proven absent, rather than merely unwitnessed.
    Absent,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InkBounds {
    pub(crate) page: usize,
    pub(crate) bounds: [[f64; 2]; 2],
}

impl InkBounds {
    fn is_valid(self) -> bool {
        let [[left, top], [right, bottom]] = self.bounds;
        self.page > 0
            && self.bounds.iter().flatten().all(|x| x.is_finite())
            && left <= right
            && top <= bottom
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct PairFrame {
    pub(crate) items: [InkBounds; 2],
    pub(crate) contact: Contact,
}

impl PairFrame {
    fn can_have_common_interior(self) -> bool {
        let [first, second] = self.items;
        let [[left, top], [right, bottom]] = first.bounds;
        let [[other_left, other_top], [other_right, other_bottom]] = second.bounds;
        first.page == second.page
            && left.max(other_left) < right.min(other_right)
            && top.max(other_top) < bottom.min(other_bottom)
    }

    fn is_valid(self) -> bool {
        self.items.into_iter().all(InkBounds::is_valid)
            && (self.contact != Contact::Present || self.can_have_common_interior())
    }

    fn has_clearance(self, minimum: f64) -> bool {
        let [first, second] = self.items;
        if first.page != second.page {
            return true;
        }
        let [[left, top], [right, bottom]] = first.bounds;
        let [[other_left, other_top], [other_right, other_bottom]] = second.bounds;
        let dx = (left - other_right)
            .max(other_left - right)
            .next_down()
            .max(0.0);
        let dy = (top - other_bottom)
            .max(other_top - bottom)
            .next_down()
            .max(0.0);
        // The rectangle distance is a lower bound on painted-ink distance.
        // Round all terms of the square down, and the threshold up.
        let square = |gap: f64| (gap * gap).next_down().max(0.0);
        (square(dx) + square(dy)).next_down() >= (minimum * minimum).next_up()
    }
}

/// `same_ink` must come from exact bound contour comparison in both frames.
/// Equal rectangle bounds alone do not justify the preservation exception.
/// `minimum` includes the already qualified measurement reserve.
pub(crate) fn decide_pair(
    role: PairRole,
    before: Option<PairFrame>,
    after: Option<PairFrame>,
    minimum: f64,
    same_ink: bool,
) -> Decision {
    let (Some(before), Some(after)) = (before, after) else {
        return Decision::Unknown;
    };
    if !before.is_valid() || !after.is_valid() {
        return Decision::Unknown;
    }
    let inconsistent_contact = matches!(
        (before.contact, after.contact),
        (Contact::Present, Contact::Absent) | (Contact::Absent, Contact::Present)
    );
    if same_ink && (before.items != after.items || inconsistent_contact) {
        return Decision::Unknown;
    }
    match role {
        PairRole::OwnedAttachment => {
            if after.contact == Contact::Present {
                Decision::Proven
            } else if after.contact == Contact::Absent || !after.can_have_common_interior() {
                Decision::Rejected
            } else {
                Decision::Unknown
            }
        }
        PairRole::Foreign => {
            if !minimum.is_finite() || minimum <= 0.0 {
                Decision::Unknown
            } else if same_ink || after.has_clearance(minimum) {
                Decision::Proven
            } else if after.contact == Contact::Present {
                Decision::Rejected
            } else {
                Decision::Unknown
            }
        }
    }
}

/// A connected set has one result. A disproven requirement rejects the set;
/// otherwise every remaining requirement must be witnessed to prove the set.
pub(crate) fn coupled_decisions(
    groups: &[(usize, Decision)],
    links: &[(usize, usize)],
) -> Option<BTreeMap<usize, Decision>> {
    let mut graph: BTreeMap<_, _> = groups
        .iter()
        .map(|&(id, verdict)| (id, (verdict, Vec::new())))
        .collect();
    if graph.len() != groups.len() {
        return None;
    }
    for &(first, second) in links {
        graph.get_mut(&first)?.1.push(second);
        graph.get_mut(&second)?.1.push(first);
    }

    let mut results = BTreeMap::new();
    while let Some((id, (mut verdict, mut pending))) = graph.pop_first() {
        let mut component = vec![id];
        while let Some(id) = pending.pop() {
            if let Some((local, neighbors)) = graph.remove(&id) {
                component.push(id);
                verdict = verdict.max(local);
                pending.extend(neighbors);
            }
        }
        results.extend(component.into_iter().map(|group| (group, verdict)));
    }
    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ink(page: usize, left: f64, top: f64) -> InkBounds {
        InkBounds {
            page,
            bounds: [[left, top], [left + 1.0, top + 1.0]],
        }
    }

    fn pair(other: InkBounds, contact: Contact) -> PairFrame {
        PairFrame {
            items: [ink(1, 0.0, 0.0), other],
            contact,
        }
    }

    #[test]
    fn foreign_clearance_requires_separation_or_exact_contours() {
        let old = pair(ink(1, 0.2, 0.2), Contact::Present);
        let far = pair(ink(1, 4.0, 5.0), Contact::Unknown);
        let near = pair(ink(1, 0.1, 0.1), Contact::Unknown);
        let overlap = pair(ink(1, 0.1, 0.1), Contact::Present);
        let decide =
            |after, minimum, same| decide_pair(PairRole::Foreign, Some(old), after, minimum, same);
        assert_eq!(decide(Some(far), 4.9, false), Decision::Proven);
        assert_eq!(decide(Some(far), 5.0, false), Decision::Unknown);
        assert_eq!(decide(Some(near), 1.0, false), Decision::Unknown);
        assert_eq!(decide(Some(overlap), 1.0, false), Decision::Rejected);
        assert_eq!(decide(Some(old), 1.0, false), Decision::Rejected);
        assert_eq!(decide(Some(old), 1.0, true), Decision::Proven);
        assert_eq!(decide(Some(overlap), 1.0, true), Decision::Unknown);
        assert_eq!(decide(None, 1.0, false), Decision::Unknown);
    }

    #[test]
    fn owns_a_contact_only_with_a_witness_at_the_attachment() {
        let old = pair(ink(1, 0.2, 0.2), Contact::Present);
        let uncertain = pair(ink(1, 0.8, 0.8), Contact::Unknown);
        let detached = pair(ink(1, 1.0, 1.0), Contact::Unknown);
        let witnessed = pair(ink(1, 0.8, 0.8), Contact::Present);
        let moved = pair(ink(2, 0.8, 0.8), Contact::Unknown);
        let decide = |after| {
            decide_pair(
                PairRole::OwnedAttachment,
                Some(old),
                Some(after),
                1.0,
                false,
            )
        };
        assert_eq!(decide(uncertain), Decision::Unknown);
        assert_eq!(
            decide(PairFrame {
                contact: Contact::Absent,
                ..uncertain
            }),
            Decision::Rejected
        );
        assert_eq!(decide(detached), Decision::Rejected);
        assert_eq!(decide(witnessed), Decision::Proven);
        assert_eq!(decide(moved), Decision::Rejected);
        assert_eq!(
            decide_pair(PairRole::Foreign, Some(old), Some(moved), 1.0, false),
            Decision::Proven,
        );
    }

    #[test]
    fn malformed_evidence_cannot_prove_an_unchanged_pair() {
        let good = pair(ink(1, 0.2, 0.2), Contact::Present);
        let false_overlap = pair(ink(2, 0.2, 0.2), Contact::Present);
        let invalid_page = pair(ink(0, 0.2, 0.2), Contact::Unknown);
        let nonfinite = pair(ink(1, f64::NAN, 0.2), Contact::Unknown);
        let contradicts_ink = PairFrame {
            contact: Contact::Absent,
            ..good
        };
        for after in [false_overlap, invalid_page, nonfinite, contradicts_ink] {
            assert_eq!(
                decide_pair(PairRole::Foreign, Some(good), Some(after), 1.0, true),
                Decision::Unknown,
            );
        }
        assert_eq!(
            decide_pair(PairRole::Foreign, Some(good), Some(good), f64::NAN, true),
            Decision::Unknown,
        );
    }

    #[test]
    fn each_coupled_component_has_one_verdict() {
        use Decision::{Proven, Rejected, Unknown};
        let groups = [
            (1, Proven),
            (2, Unknown),
            (3, Proven),
            (4, Rejected),
            (5, Proven),
        ];
        assert_eq!(
            coupled_decisions(&groups, &[(3, 2), (1, 2)]),
            Some(BTreeMap::from([
                (1, Unknown),
                (2, Unknown),
                (3, Unknown),
                (4, Rejected),
                (5, Proven)
            ])),
        );
        assert_eq!(
            coupled_decisions(&groups, &[(2, 3), (2, 1), (3, 4)]),
            Some(BTreeMap::from([
                (1, Rejected),
                (2, Rejected),
                (3, Rejected),
                (4, Rejected),
                (5, Proven)
            ])),
        );
        assert_eq!(coupled_decisions(&groups, &[(8, 2)]), None);
        assert_eq!(coupled_decisions(&[(1, Proven), (1, Proven)], &[]), None);
    }
}
