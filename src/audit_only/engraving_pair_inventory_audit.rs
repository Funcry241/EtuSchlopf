//! Inventory and pair coverage for qualified synthetic renderer pages.
//!
//! The caller qualifies the page and binds renderer marks to source marks.
//! Expected identities come from the authored score for each frame.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Role {
    Stem,
    Head,
    Tie,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ExpectedMark {
    pub(super) id: u32,
    pub(super) role: Role,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ObservedMark<T> {
    pub(super) id: Option<u32>,
    pub(super) role: Role,
    pub(super) page: usize,
    pub(super) geometry: T,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum InventoryError {
    DuplicateExpected(u32),
    Unbound(Role),
    Unexpected(u32),
    WrongRole(u32),
    InvalidPage(u32),
    DuplicateObserved(u32),
    MissingObserved(u32),
}

type Inventory<'a, T> = BTreeMap<u32, &'a ObservedMark<T>>;
type Pairs = BTreeSet<(u32, u32)>;

fn inventory<'a, T>(
    expected: &[ExpectedMark],
    observed: &'a [ObservedMark<T>],
) -> Result<Inventory<'a, T>, InventoryError> {
    let mut roster = BTreeMap::new();
    for mark in expected {
        if roster.insert(mark.id, mark.role).is_some() {
            return Err(InventoryError::DuplicateExpected(mark.id));
        }
    }

    let mut bound = BTreeMap::new();
    for mark in observed {
        let id = mark.id.ok_or(InventoryError::Unbound(mark.role))?;
        if mark.page == 0 {
            return Err(InventoryError::InvalidPage(id));
        }
        match roster.get(&id) {
            None => return Err(InventoryError::Unexpected(id)),
            Some(role) if *role != mark.role => return Err(InventoryError::WrongRole(id)),
            Some(_) => {}
        }
        if bound.insert(id, mark).is_some() {
            return Err(InventoryError::DuplicateObserved(id));
        }
    }
    if let Some(id) = roster.keys().find(|id| !bound.contains_key(id)) {
        return Err(InventoryError::MissingObserved(*id));
    }
    Ok(bound)
}

fn extend_pairs<T>(frame: &Inventory<'_, T>, changed: &BTreeSet<u32>, pairs: &mut Pairs) {
    let stems = frame.iter().filter(|(_, mark)| mark.role == Role::Stem);
    let neighbours = frame.iter().filter(|(_, mark)| mark.role != Role::Stem);
    for (&stem_id, stem) in stems {
        for (&neighbour_id, neighbour) in neighbours.clone() {
            if stem.page == neighbour.page
                && (changed.contains(&stem_id) || changed.contains(&neighbour_id))
            {
                pairs.insert((stem_id, neighbour_id));
            }
        }
    }
}

/// Pair endpoints are ordered as (stem, head or tie). The result includes
/// every same-page, role-compatible pair touching a changed mark in either frame.
/// An attachment or clearance does not exempt a pair in this audit.
pub(super) fn affected_pairs<T: PartialEq>(
    before_expected: &[ExpectedMark],
    before: &[ObservedMark<T>],
    after_expected: &[ExpectedMark],
    after: &[ObservedMark<T>],
) -> Result<Pairs, InventoryError> {
    let before = inventory(before_expected, before)?;
    let after = inventory(after_expected, after)?;
    let mut changed = BTreeSet::new();
    for (&id, old) in &before {
        if let Some(new) = after.get(&id)
            && old.role != new.role
        {
            return Err(InventoryError::WrongRole(id));
        }
        if after.get(&id) != Some(old) {
            changed.insert(id);
        }
    }
    changed.extend(after.keys().filter(|id| !before.contains_key(id)));
    let mut pairs = Pairs::new();
    extend_pairs(&before, &changed, &mut pairs);
    extend_pairs(&after, &changed, &mut pairs);
    Ok(pairs)
}

/// Independently enumerate every unordered pair in both frames. Call this
/// comparison only on the same inventories accepted by `affected_pairs`.
pub(super) fn exhaustive_pairs<T: PartialEq>(
    before: &[ObservedMark<T>],
    after: &[ObservedMark<T>],
) -> Pairs {
    let mut found = Pairs::new();
    for (frame, other) in [(before, after), (after, before)] {
        for (index, first) in frame.iter().enumerate() {
            for second in &frame[index + 1..] {
                if first.page != second.page || (other.contains(first) && other.contains(second)) {
                    continue;
                }
                let (stem, neighbour) = match (first.role, second.role) {
                    (Role::Stem, Role::Head | Role::Tie) => (first, second),
                    (Role::Head | Role::Tie, Role::Stem) => (second, first),
                    _ => continue,
                };
                found.insert((
                    stem.id.expect("validated stem"),
                    neighbour.id.expect("validated neighbour"),
                ));
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEM: ExpectedMark = ExpectedMark {
        id: 47,
        role: Role::Stem,
    };
    const HEAD: ExpectedMark = ExpectedMark {
        id: 29,
        role: Role::Head,
    };
    const TIE: ExpectedMark = ExpectedMark {
        id: 81,
        role: Role::Tie,
    };
    const ROSTER: [ExpectedMark; 3] = [STEM, HEAD, TIE];

    fn mark(expected: ExpectedMark, geometry: &str) -> ObservedMark<&str> {
        ObservedMark {
            id: Some(expected.id),
            role: expected.role,
            page: 1,
            geometry,
        }
    }

    #[test]
    fn complete_unchanged_inventory_has_no_affected_pairs() {
        let frame = [mark(STEM, "vertical"), mark(HEAD, "oval"), mark(TIE, "arc")];
        let reordered = [frame[2], frame[1], frame[0]];
        let found = affected_pairs(&ROSTER, &frame, &ROSTER, &reordered).unwrap();
        assert!(found.is_empty());
        assert_eq!(found, exhaustive_pairs(&frame, &reordered));
    }

    #[test]
    fn changed_stem_without_a_role_compatible_neighbour_yields_no_pairs() {
        let before = [mark(STEM, "up")];
        let after = [mark(STEM, "down")];
        let found = affected_pairs(&[STEM], &before, &[STEM], &after).unwrap();
        assert!(found.is_empty());
        assert_eq!(found, exhaustive_pairs(&before, &after));
    }

    #[test]
    fn moved_head_and_tie_are_checked_against_unchanged_stem() {
        let stem = mark(STEM, "vertical");
        let before = [stem, mark(HEAD, "oval left"), mark(TIE, "arc above")];
        let after = [mark(TIE, "arc below"), stem, mark(HEAD, "oval right")];
        let pairs = affected_pairs(&ROSTER, &before, &ROSTER, &after).unwrap();
        let expected = exhaustive_pairs(&before, &after);
        assert_eq!(
            expected,
            BTreeSet::from([(STEM.id, HEAD.id), (STEM.id, TIE.id)])
        );
        assert_eq!(pairs, expected);
        let mut omitted = pairs;
        omitted.remove(&(STEM.id, HEAD.id));
        assert_ne!(omitted, expected);
    }

    #[test]
    fn union_keeps_pairs_present_in_only_one_frame() {
        let before = [mark(STEM, "vertical"), mark(HEAD, "oval")];
        let after = [mark(TIE, "arc"), mark(STEM, "vertical")];
        let found = affected_pairs(&[STEM, HEAD], &before, &[STEM, TIE], &after).unwrap();
        assert_eq!(
            found,
            BTreeSet::from([(STEM.id, HEAD.id), (STEM.id, TIE.id)])
        );
        assert_eq!(found, exhaustive_pairs(&before, &after));
        assert_eq!(
            found,
            affected_pairs(&[STEM, TIE], &after, &[STEM, HEAD], &before).unwrap(),
        );
    }

    #[test]
    fn neighbour_missing_from_both_frames_fails_inventory_check() {
        let before = [mark(STEM, "up")];
        let after = [mark(STEM, "down")];
        assert!(exhaustive_pairs(&before, &after).is_empty());
        assert_eq!(
            affected_pairs(&[STEM, HEAD], &before, &[STEM, HEAD], &after),
            Err(InventoryError::MissingObserved(HEAD.id)),
        );
    }

    #[test]
    fn moved_stem_keeps_origin_and_destination_neighbours() {
        let stem = mark(STEM, "vertical");
        let head = mark(HEAD, "oval");
        let tie = ObservedMark { page: 2, ..mark(TIE, "arc") };
        let before = [stem, head, tie];
        let after = [ObservedMark { page: 2, ..stem }, head, tie];
        let found = affected_pairs(&ROSTER, &before, &ROSTER, &after).unwrap();
        assert_eq!(found, BTreeSet::from([(STEM.id, HEAD.id), (STEM.id, TIE.id)]));
        assert_eq!(found, exhaustive_pairs(&before, &after));

        let changed_without_move = [mark(STEM, "horizontal"), head, tie];
        assert_eq!(
            affected_pairs(&ROSTER, &before, &ROSTER, &changed_without_move).unwrap(),
            BTreeSet::from([(STEM.id, HEAD.id)]),
        );
    }

    #[test]
    fn invalid_observations_never_yield_pairs() {
        let stem = mark(STEM, "up");
        let head = mark(HEAD, "oval");
        assert_eq!(
            inventory(&[STEM, STEM], &[stem]),
            Err(InventoryError::DuplicateExpected(STEM.id))
        );
        assert_eq!(
            inventory(&[STEM], &[stem, stem]),
            Err(InventoryError::DuplicateObserved(STEM.id))
        );
        assert_eq!(
            inventory(&[STEM], &[stem, head]),
            Err(InventoryError::Unexpected(HEAD.id))
        );
        assert_eq!(
            inventory(&[STEM], &[ObservedMark { id: None, ..stem }]),
            Err(InventoryError::Unbound(Role::Stem)),
        );
        assert_eq!(
            inventory(
                &[STEM],
                &[ObservedMark {
                    role: Role::Head,
                    ..stem
                }]
            ),
            Err(InventoryError::WrongRole(STEM.id)),
        );
    }
}
