//! Read-only observation of the actual first assignment and admitted baseline.
use super::ordered_received_beam_readback::{closed_group, owner_groups};
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

type ClassPair = (usize, usize);
type ChannelRank = (usize, usize);
type Edges = BTreeSet<ClassPair>;
type Role = SerializedVoicePresentationRole;
type Attack<'a> = &'a FrozenVoiceAttackEvidenceRow;
type Written<'a> = &'a FinalChordWrittenOccurrence;

pub(super) struct Conflicts<'a> {
    sounding: &'a Edges,
    written: &'a Edges,
    role: &'a Edges,
    attacks: &'a BTreeMap<usize, Attack<'a>>,
    members: &'a BTreeMap<usize, Vec<usize>>,
    roles: &'a BTreeMap<usize, Role>,
    written_by_class: BTreeMap<usize, Vec<Written<'a>>>,
}

impl<'a> Conflicts<'a> {
    pub(super) fn new(
        (sounding, written, role): (&'a Edges, &'a Edges, &'a Edges),
        attacks: &'a BTreeMap<usize, Attack<'a>>,
        members: &'a BTreeMap<usize, Vec<usize>>,
        roles: &'a BTreeMap<usize, Role>,
        projection: &'a FinalChordWrittenProjectionEvidence,
        classes: &BTreeMap<usize, usize>,
    ) -> Self {
        let mut written_by_class = BTreeMap::<_, Vec<_>>::new();
        for row in projection.occurrences.iter() {
            written_by_class
                .entry(classes[&row.source_event_id])
                .or_default()
                .push(row);
        }
        Self {
            sounding,
            written,
            role,
            attacks,
            members,
            roles,
            written_by_class,
        }
    }

    fn append_for_classes(
        &self,
        out: &mut String,
        fragments: (usize, usize),
        assigned: usize,
        blocker: usize,
        kind: &str,
    ) -> bool {
        let pair = (assigned.min(blocker), assigned.max(blocker));
        let sounding = self.sounding.contains(&pair);
        let written = self.written.contains(&pair);
        let role = self.role.contains(&pair);
        if sounding {
            let witness = self.members[&assigned].iter().find_map(|source| {
                let a = self.attacks[source];
                self.members[&blocker].iter().find_map(|other| {
                    let b = self.attacks[other];
                    a.sounding_interval()
                        .overlaps(b.sounding_interval())
                        .then_some((a, b))
                })
            });
            if let Some((a, b)) = witness {
                let _ = writeln!(
                    out,
                    "PrimaryBeamFirstAssignmentBlocker: fragments={fragments:?}; kind={kind}; classes={:?}; reason=SoundingOverlap; staff={}; sources_attacks={:?}; intervals={:?}; authority=none",
                    (assigned, blocker),
                    a.staff,
                    [
                        (a.source_event_id, a.attack_id),
                        (b.source_event_id, b.attack_id)
                    ],
                    [
                        (a.absolute_start, a.absolute_end),
                        (b.absolute_start, b.absolute_end)
                    ]
                );
            }
        }
        if written {
            let mut left = self.written_by_class.get(&assigned).into_iter().flatten();
            let witness = left.find_map(|a| {
                self.written_by_class
                    .get(&blocker)
                    .into_iter()
                    .flatten()
                    .find_map(|b| {
                        a.written_interval()
                            .overlaps(b.written_interval())
                            .then_some((a, b))
                    })
            });
            if let Some((a, b)) = witness {
                let _ = writeln!(
                    out,
                    "PrimaryBeamFirstAssignmentBlocker: fragments={fragments:?}; kind={kind}; classes={:?}; reason=WrittenOverlap; measure={}; sources_attacks_fragments={:?}; intervals={:?}; authority=none",
                    (assigned, blocker),
                    a.measure,
                    [
                        (a.source_event_id, a.attack_id, a.fragment_id),
                        (b.source_event_id, b.attack_id, b.fragment_id)
                    ],
                    [(a.onset, a.end), (b.onset, b.end)]
                );
            }
        }
        if role {
            let _ = writeln!(
                out,
                "PrimaryBeamFirstAssignmentBlocker: fragments={fragments:?}; kind={kind}; classes={:?}; reason=PresentationRole; roles={:?}; class_members={:?}; authority=none",
                (assigned, blocker),
                (self.roles[&assigned], self.roles[&blocker]),
                (&self.members[&assigned], &self.members[&blocker])
            );
        }
        sounding || written || role
    }
}

struct Pair {
    fragments: (usize, usize),
    left_right_classes: ClassPair,
    earlier: usize,
    greedy: Option<(usize, usize)>,
}

pub(super) struct Readback<'a> {
    out: &'a mut String,
    ties: Option<&'a mut Vec<(usize, usize)>>,
    pairs: Vec<Pair>,
    by_later: BTreeMap<usize, Vec<usize>>,
}

impl<'a> Readback<'a> {
    pub(super) fn new(
        (out, ties): (&'a mut String, Option<&'a mut Vec<(usize, usize)>>),
        capture: Option<&ActualSourceBeamBindingObservation>,
        canonical: &CanonicalFinalChordStructuralPlan,
        classes: &BTreeMap<usize, usize>,
        order: &[usize],
    ) -> Self {
        let mut readback = Self {
            out,
            ties,
            pairs: vec![],
            by_later: BTreeMap::new(),
        };
        let Some(rows) =
            capture.and_then(|capture| capture.graph_received_occurrences(canonical).ok())
        else {
            let _ = writeln!(
                readback.out,
                "PrimaryBeamFirstAssignment: capture=unbound; authority=none"
            );
            return readback;
        };
        let position: BTreeMap<_, _> = order
            .iter()
            .enumerate()
            .map(|(index, &class)| (class, index))
            .collect();
        for ((level, _), members) in owner_groups(rows) {
            if level != 1 || !closed_group(&members) {
                continue;
            }
            for window in members.windows(2) {
                let (a, b) = (window[0].0, window[1].0);
                let fragments = (a.fragment, b.fragment);
                let (Some(&left), Some(&right)) = (classes.get(&a.source), classes.get(&b.source))
                else {
                    let _ = writeln!(
                        readback.out,
                        "PrimaryBeamFirstAssignmentPair: fragments={fragments:?}; first_observed=open; reason=source_class_unbound; authority=none"
                    );
                    continue;
                };
                let (earlier, later) = if position[&left] <= position[&right] {
                    (left, right)
                } else {
                    (right, left)
                };
                readback
                    .by_later
                    .entry(later)
                    .or_default()
                    .push(readback.pairs.len());
                readback.pairs.push(Pair {
                    fragments,
                    left_right_classes: (left, right),
                    earlier,
                    greedy: None,
                });
            }
        }
        readback
    }

    pub(super) fn observe_assignment(
        &mut self,
        class: usize,
        channel: usize,
        assigned: &BTreeMap<usize, usize>,
        ranked: &[ChannelRank],
        conflicts: &Conflicts<'_>,
    ) {
        let Some(indices) = self.by_later.get(&class) else {
            return;
        };
        for &index in indices {
            let pair = &mut self.pairs[index];
            if pair.earlier == class {
                pair.greedy = Some((channel, channel));
                continue;
            }
            let Some(&earlier_channel) = assigned.get(&pair.earlier) else {
                continue;
            };
            pair.greedy = Some(if pair.left_right_classes.0 == pair.earlier {
                (earlier_channel, channel)
            } else {
                (channel, earlier_channel)
            });
            if earlier_channel == channel {
                continue;
            }
            let mut direct = false;
            let mut cooccupant = false;
            for (&blocker, &assigned_channel) in assigned {
                if assigned_channel != earlier_channel {
                    continue;
                }
                if blocker == pair.earlier {
                    direct = conflicts.append_for_classes(
                        self.out,
                        pair.fragments,
                        class,
                        blocker,
                        "OtherEndpointClass",
                    );
                } else {
                    cooccupant |= conflicts.append_for_classes(
                        self.out,
                        pair.fragments,
                        class,
                        blocker,
                        "OtherChannelMember",
                    );
                }
            }
            let rank = |target| ranked.iter().find(|entry| entry.1 == target).copied();
            let earlier_rank = rank(earlier_channel);
            let chosen_rank = rank(channel);
            let result = if direct || cooccupant {
                "BlockedAtAssignment"
            } else {
                rank_reason(earlier_rank, chosen_rank)
            };
            if result == "ChannelNumberTieBreak"
                && let Some(ties) = &mut self.ties
            {
                ties.push(pair.fragments);
            }
            let _ = writeln!(
                self.out,
                "PrimaryBeamFirstAssignmentDecision: fragments={:?}; earlier_class={}; later_class={class}; earlier_channel={earlier_channel}; chosen_channel={channel}; earlier_rank={earlier_rank:?}; chosen_rank={chosen_rank:?}; direct_class_conflict={direct}; other_channel_member_conflict={cooccupant}; result={result}; authority=none",
                pair.fragments, pair.earlier
            );
        }
    }

    pub(super) fn finish(
        self,
        admitted: &BTreeMap<usize, usize>,
        allocation: &ChannelConsumerAllocation,
    ) {
        let method = allocation.label();
        let _ = writeln!(
            self.out,
            "PrimaryBeamFirstAssignment: admitted_allocation={method}; bound_pairs={}; authority=none",
            self.pairs.len()
        );
        for pair in self.pairs {
            let (left, right) = pair.left_right_classes;
            let baseline = admitted
                .get(&left)
                .copied()
                .zip(admitted.get(&right).copied());
            let step = first_split(pair.greedy, baseline);
            let _ = writeln!(
                self.out,
                "PrimaryBeamFirstAssignmentPair: fragments={:?}; greedy_channels={:?}; baseline_channels={baseline:?}; first_observed={step}; authority=none",
                pair.fragments, pair.greedy
            );
        }
    }
}

fn rank_reason(earlier: Option<ChannelRank>, chosen: Option<ChannelRank>) -> &'static str {
    match earlier.zip(chosen) {
        Some((earlier, chosen)) if chosen.0 < earlier.0 => "TechnicalLanePreference",
        Some((earlier, chosen)) if chosen.0 == earlier.0 && chosen.1 < earlier.1 => {
            "ChannelNumberTieBreak"
        }
        _ => "Open",
    }
}

fn first_split(greedy: Option<(usize, usize)>, baseline: Option<(usize, usize)>) -> &'static str {
    match (greedy, baseline) {
        (Some((a, b)), _) if a != b => "first_assignment",
        (Some(_), Some((a, b))) if a != b => "admission_transition",
        (Some(_), Some(_)) => "none",
        _ => "open",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_the_actual_rank_comparison() {
        assert_eq!(
            rank_reason(Some((1, 2)), Some((0, 7))),
            "TechnicalLanePreference"
        );
        assert_eq!(
            rank_reason(Some((0, 7)), Some((0, 2))),
            "ChannelNumberTieBreak"
        );
        assert_eq!(rank_reason(None, Some((0, 2))), "Open");
    }

    #[test]
    fn a_later_admission_split_has_no_first_assignment_cause() {
        assert_eq!(
            first_split(Some((2, 2)), Some((3, 4))),
            "admission_transition"
        );
        assert_eq!(first_split(Some((2, 3)), Some((4, 4))), "first_assignment");
        assert_eq!(first_split(Some((2, 2)), Some((4, 4))), "none");
        assert_eq!(first_split(None, Some((4, 5))), "open");
    }
}
