//! Partial R6 implementation, deliberately not linked into the product.
//!
//! One instance represents one fully bound physical keyboard and one immutable
//! phase seed, followed by that alternative's sequentially confirmed proposals.
//! S/A are the existing carried source/attack ID types, not identities minted here.
//! The future adapter must establish seed custody, complete observation coverage,
//! exact timing, role evidence and writer-equivalent proposal geometry.
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawPair {
    pub onset: i64,
    pub pitch: i32,
}

/// Measure-local keys are supported only where the operation is proven to stay
/// within its bound non-overlapping measure partition. No time-grid conversion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlacedPair {
    pub measure: usize,
    pub onset: i64,
    pub pitch: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    RegularAttack,
    /// Requires a carried source/attack relation, never inferred from missing IDs.
    ProvenContinuation,
    NonPitched,
    /// Outside the regular-attack domain; cannot itself be moved by this kernel.
    Grace,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Occurrence<S, A> {
    pub source: Option<S>,
    pub attack: Option<A>,
    pub role: Role,
    pub placed: Option<PlacedPair>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    UnboundSeed,
    DuplicateSource,
    DuplicateAttack,
    InvalidOccurrence,
    UnsupportedRole,
    MissingIdentity,
    MissingRawPair,
    MissingPlacement,
    UnsupportedGeometry,
    NewCoincidence,
    UnresolvedOccupant,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoincidenceOccupancy<S, A> {
    raw: BTreeMap<S, RawPair>,
    occurrences: Vec<Occurrence<S, A>>,
}

impl<S: Copy + Ord, A: Copy + Ord> CoincidenceOccupancy<S, A> {
    pub fn from_observations(
        complete_seed_binding: bool,
        raw: impl IntoIterator<Item = (S, RawPair)>,
        occurrences: Vec<Occurrence<S, A>>,
    ) -> Result<Self, Refusal> {
        if !complete_seed_binding {
            return Err(Refusal::UnboundSeed);
        }
        let mut raw_map = BTreeMap::new();
        for (source, pair) in raw {
            if raw_map.insert(source, pair).is_some() {
                return Err(Refusal::DuplicateSource);
            }
        }
        let mut attacks = BTreeSet::new();
        let mut attack_sources = BTreeSet::new();
        for occurrence in &occurrences {
            if occurrence.role == Role::RegularAttack {
                if let Some(attack) = occurrence.attack
                    && !attacks.insert(attack)
                {
                    return Err(Refusal::DuplicateAttack);
                }
                if let Some(source) = occurrence.source
                    && !attack_sources.insert(source)
                {
                    return Err(Refusal::DuplicateSource);
                }
            }
        }
        Ok(Self {
            raw: raw_map,
            occurrences,
        })
    }

    fn raw_pair(&self, occurrence: &Occurrence<S, A>) -> Result<RawPair, Refusal> {
        let source = occurrence.source.ok_or(Refusal::MissingIdentity)?;
        occurrence.attack.ok_or(Refusal::MissingIdentity)?;
        self.raw
            .get(&source)
            .copied()
            .ok_or(Refusal::MissingRawPair)
    }

    /// Only the R6 necessary condition. Success never overrides A7 or authorizes
    /// geometry, source transfer, visibility, duration, scoring or an XML write.
    pub fn allows(&self, index: usize, proposed: PlacedPair) -> Result<(), Refusal> {
        let moving = self
            .occurrences
            .get(index)
            .ok_or(Refusal::InvalidOccurrence)?;
        if moving.role != Role::RegularAttack {
            return Err(Refusal::UnsupportedRole);
        }
        let current = moving.placed.ok_or(Refusal::MissingPlacement)?;
        // The existing gesture operation changes onset, not physical key/measure.
        if proposed.measure != current.measure || proposed.pitch != current.pitch {
            return Err(Refusal::UnsupportedGeometry);
        }
        let raw = self.raw_pair(moving)?;
        for (other_index, other) in self.occurrences.iter().enumerate() {
            if other_index == index {
                continue;
            }
            match other.role {
                Role::NonPitched | Role::Grace => continue,
                Role::ProvenContinuation => {
                    // A role label alone is insufficient. The future adapter
                    // must prove the continuation; missing IDs remain unknown.
                    if self.raw_pair(other).is_ok() {
                        continue;
                    }
                }
                Role::RegularAttack | Role::Unknown => {}
            }
            match other.placed {
                Some(placed) if placed != proposed => continue,
                None => return Err(Refusal::UnresolvedOccupant),
                _ => {}
            }
            if other.role != Role::RegularAttack {
                return Err(Refusal::UnresolvedOccupant);
            }
            let other_raw = self.raw_pair(other)?;
            if other_raw != raw {
                return Err(Refusal::NewCoincidence);
            }
        }
        Ok(())
    }

    /// Recheck against the current simulated state. A previously observed free
    /// target is no reservation and cannot be committed after another move.
    pub fn move_to(&mut self, index: usize, proposed: PlacedPair) -> Result<(), Refusal> {
        self.allows(index, proposed)?;
        self.occurrences[index].placed = Some(proposed);
        Ok(())
    }

    pub fn occurrences(&self) -> &[Occurrence<S, A>] {
        &self.occurrences
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Model = CoincidenceOccupancy<u32, u32>;
    fn pair(onset: i64) -> PlacedPair {
        PlacedPair {
            measure: 1,
            onset,
            pitch: 60,
        }
    }
    fn entry(source: u32, onset: i64) -> Occurrence<u32, u32> {
        Occurrence {
            source: Some(source),
            attack: Some(source + 100),
            role: Role::RegularAttack,
            placed: Some(pair(onset)),
        }
    }
    fn model(raw: &[(u32, i64)], rows: Vec<Occurrence<u32, u32>>) -> Model {
        Model::from_observations(
            true,
            raw.iter()
                .map(|&(s, onset)| (s, RawPair { onset, pitch: 60 })),
            rows,
        )
        .unwrap()
    }
    #[test]
    fn distinct_raw_onsets_cannot_coincide() {
        let m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), entry(2, 10)]);
        assert_eq!(m.allows(0, pair(10)), Err(Refusal::NewCoincidence));
    }
    #[test]
    fn raw_unisons_are_allowed_without_freezing() {
        let mut m = model(&[(1, 0), (2, 0)], vec![entry(1, 0), entry(2, 10)]);
        assert_eq!(m.move_to(0, pair(10)), Ok(()));
        assert_eq!(m.move_to(0, pair(20)), Ok(()));
    }
    #[test]
    fn another_raw_key_is_not_a_unison() {
        let rows = vec![entry(1, 0), entry(2, 10)];
        let m = Model::from_observations(
            true,
            [
                (
                    1,
                    RawPair {
                        onset: 0,
                        pitch: 60,
                    },
                ),
                (
                    2,
                    RawPair {
                        onset: 0,
                        pitch: 61,
                    },
                ),
            ],
            rows,
        )
        .unwrap();
        assert_eq!(m.allows(0, pair(10)), Err(Refusal::NewCoincidence));
    }
    #[test]
    fn other_keys_or_onsets_do_not_form_r6_pairs() {
        let mut other = entry(2, 10);
        other.placed.as_mut().unwrap().pitch = 61;
        let m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), other]);
        assert!(m.allows(0, pair(10)).is_ok());
        assert!(m.allows(0, pair(20)).is_ok());
    }
    #[test]
    fn reserved_target_is_seen_and_old_location_is_freed() {
        let mut m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), entry(2, 10)]);
        assert!(m.allows(0, pair(20)).is_ok());
        assert!(m.allows(1, pair(20)).is_ok());
        m.move_to(0, pair(20)).unwrap();
        assert_eq!(m.move_to(1, pair(20)), Err(Refusal::NewCoincidence));
        m.move_to(1, pair(0)).unwrap();
        assert_eq!(m.occurrences()[0].placed, Some(pair(20)));
        assert_eq!(m.occurrences()[1].placed, Some(pair(0)));
    }
    #[test]
    fn future_swap_creates_no_speculative_vacancy() {
        let mut m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), entry(2, 10)]);
        let before = m.clone();
        assert!(m.move_to(0, pair(10)).is_err());
        assert!(m.move_to(1, pair(0)).is_err());
        assert_eq!(m, before);
    }
    #[test]
    fn alternatives_and_scoring_copies_are_independent() {
        let mut a = model(&[(1, 0), (2, 10)], vec![entry(1, 0), entry(2, 10)]);
        let mut b = a.clone();
        let seed = a.clone();
        for (index, destination) in [(0, pair(20)), (1, pair(0))] {
            assert_eq!(a.allows(index, destination), b.allows(index, destination));
            a.move_to(index, destination).unwrap();
            b.move_to(index, destination).unwrap();
        }
        assert_eq!(a, b);
        assert_eq!(seed.occurrences()[0].placed, Some(pair(0)));
    }
    #[test]
    fn missing_attack_id_is_not_a_continuation() {
        let mut other = entry(2, 10);
        other.attack = None;
        let m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), other]);
        assert_eq!(m.allows(0, pair(10)), Err(Refusal::MissingIdentity));
    }
    #[test]
    fn missing_source_or_raw_binding_refuses_the_proposal() {
        let mut other = entry(2, 10);
        other.source = None;
        let m = model(&[(1, 0)], vec![entry(1, 0), other]);
        assert_eq!(m.allows(0, pair(10)), Err(Refusal::MissingIdentity));
        let m = model(&[(1, 0)], vec![entry(1, 0), entry(2, 10)]);
        assert_eq!(m.allows(0, pair(10)), Err(Refusal::MissingRawPair));
    }
    #[test]
    fn continuation_requires_carried_identity_and_role() {
        let mut other = entry(2, 10);
        other.role = Role::ProvenContinuation;
        let m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), other.clone()]);
        assert!(m.allows(0, pair(10)).is_ok());
        other.attack = None;
        let m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), other]);
        assert_eq!(m.allows(0, pair(10)), Err(Refusal::UnresolvedOccupant));
    }
    #[test]
    fn unknown_role_or_placement_cannot_prove_a_free_target() {
        let mut other = entry(2, 10);
        other.role = Role::Unknown;
        let m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), other.clone()]);
        assert_eq!(m.allows(0, pair(10)), Err(Refusal::UnresolvedOccupant));
        other.placed = None;
        let m = model(&[(1, 0), (2, 10)], vec![entry(1, 0), other]);
        assert_eq!(m.allows(0, pair(30)), Err(Refusal::UnresolvedOccupant));
    }
    #[test]
    fn invalid_seed_and_duplicate_identities_are_rejected() {
        let raw = [(
            1,
            RawPair {
                onset: 0,
                pitch: 60,
            },
        )];
        assert_eq!(
            Model::from_observations(false, raw, vec![entry(1, 0)]),
            Err(Refusal::UnboundSeed)
        );
        assert_eq!(
            Model::from_observations(true, [raw[0], raw[0]], vec![]),
            Err(Refusal::DuplicateSource)
        );
        let mut other = entry(2, 10);
        other.attack = Some(101);
        assert_eq!(
            Model::from_observations(true, raw, vec![entry(1, 0), other]),
            Err(Refusal::DuplicateAttack)
        );
    }
    #[test]
    fn failed_move_preserves_the_current_seed_position_and_identity() {
        let mut m = model(&[(1, 0), (2, 10)], vec![entry(1, 5), entry(2, 10)]);
        let before = m.clone();
        assert!(m.move_to(0, pair(10)).is_err());
        assert_eq!(m, before);
        assert_eq!(m.occurrences()[0].placed, Some(pair(5)));
    }
    #[test]
    fn geometry_cannot_change_key_or_leave_the_bound_measure() {
        let m = model(&[(1, 0)], vec![entry(1, 0)]);
        let mut p = pair(10);
        p.measure = 2;
        assert_eq!(m.allows(0, p), Err(Refusal::UnsupportedGeometry));
        p.measure = 1;
        p.pitch = 61;
        assert_eq!(m.allows(0, p), Err(Refusal::UnsupportedGeometry));
        assert!(m.allows(0, pair(0)).is_ok());
    }
    #[test]
    fn every_pair_in_a_mixed_cluster_is_checked() {
        let m = model(
            &[(1, 0), (2, 0), (3, 10)],
            vec![entry(1, 0), entry(2, 20), entry(3, 20)],
        );
        assert_eq!(m.allows(0, pair(20)), Err(Refusal::NewCoincidence));
    }
    #[test]
    fn unrelated_occurrences_and_identity_values_do_not_change_local_result() {
        for offset in [0, 50, 1000] {
            let mut m = model(
                &[(1 + offset, 0), (2 + offset, 10), (3 + offset, 40)],
                vec![
                    entry(1 + offset, 0),
                    entry(2 + offset, 10),
                    entry(3 + offset, 40),
                ],
            );
            assert_eq!(m.move_to(0, pair(10)), Err(Refusal::NewCoincidence));
            assert!(m.move_to(2, pair(50)).is_ok());
            assert_eq!(m.allows(0, pair(10)), Err(Refusal::NewCoincidence));
        }
    }
    #[test]
    fn non_attacks_do_not_grant_permission_to_move_themselves() {
        for role in [
            Role::NonPitched,
            Role::Grace,
            Role::Unknown,
            Role::ProvenContinuation,
        ] {
            let mut row = entry(1, 0);
            row.role = role;
            let m = model(&[(1, 0)], vec![row]);
            assert_eq!(m.allows(0, pair(10)), Err(Refusal::UnsupportedRole));
        }
    }

    #[test]
    fn sequential_moves_preserve_the_global_pair_relation_on_a_finite_domain() {
        // Independent whole-state postcondition, not another local target check.
        // Raw unisons are included, and failed moves must leave the model intact.
        for a in 0..3 {
            for b in 0..3 {
                for c in 0..3 {
                    let raw = [a, b, c];
                    for first in 0..9 {
                        for second in 0..9 {
                            let mut m = model(
                                &[(1, a), (2, b), (3, c)],
                                vec![entry(1, a), entry(2, b), entry(3, c)],
                            );
                            for choice in [first, second] {
                                let before = m.clone();
                                let result = m.move_to(choice / 3, pair((choice % 3) as i64));
                                if result.is_err() {
                                    assert_eq!(m, before);
                                }
                                for i in 0..3 {
                                    for j in i + 1..3 {
                                        if m.occurrences()[i].placed == m.occurrences()[j].placed {
                                            assert_eq!(raw[i], raw[j]);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
