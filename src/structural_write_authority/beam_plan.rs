//! Shared identity-free beam planning primitives.
//!
//! Rules inspect relative timing and return typed edits. Source binding,
//! authorization, conflict handling and XML mutation remain outside this model.

use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeamState {
    Begin,
    Continue,
    End,
}

impl BeamState {
    pub fn as_musicxml_text(self) -> &'static str {
        match self {
            Self::Begin => "begin",
            Self::Continue => "continue",
            Self::End => "end",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BeamEdit {
    event_ordinal: usize,
    level: u8,
    state: BeamState,
}

impl BeamEdit {
    pub fn new(event_ordinal: usize, level: u8, state: BeamState) -> Self {
        Self {
            event_ordinal,
            level: level.max(1),
            state,
        }
    }

    pub fn event_ordinal(self) -> usize {
        self.event_ordinal
    }

    pub fn level(self) -> u8 {
        self.level
    }

    pub fn state(self) -> BeamState {
        self.state
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeamPlan {
    edits: Vec<BeamEdit>,
}

impl BeamPlan {
    pub fn new(edits: Vec<BeamEdit>) -> Self {
        Self { edits }
    }

    /// Builds one fail-closed plan from disjoint, ordered event groups whose
    /// beam levels all span the complete group.
    pub fn from_groups(event_count: usize, groups: &[(&[usize], u8)]) -> Option<Self> {
        let mut level_groups = Vec::new();
        for (ordinals, highest_level) in groups {
            if *highest_level == 0 {
                return None;
            }
            for level in 1..=*highest_level {
                level_groups.push((*ordinals, level));
            }
        }
        Self::from_level_groups(event_count, &level_groups)
    }

    /// Builds one fail-closed plan from explicit, ordered groups per beam level.
    /// This supports shorter secondary and tertiary runs inside one primary group.
    pub fn from_level_groups(
        event_count: usize,
        groups: &[(&[usize], u8)],
    ) -> Option<Self> {
        if event_count == 0 || groups.is_empty() {
            return None;
        }
        let mut occupied = BTreeSet::new();
        let mut edits = Vec::new();
        for (ordinals, level) in groups {
            if ordinals.len() < 2
                || *level == 0
                || ordinals.iter().any(|ordinal| *ordinal >= event_count)
                || ordinals.windows(2).any(|pair| pair[0] >= pair[1])
            {
                return None;
            }
            for (position, event_ordinal) in ordinals.iter().copied().enumerate() {
                if !occupied.insert((event_ordinal, *level)) {
                    return None;
                }
                let state = match position {
                    0 => BeamState::Begin,
                    position if position + 1 == ordinals.len() => BeamState::End,
                    _ => BeamState::Continue,
                };
                edits.push(BeamEdit::new(event_ordinal, *level, state));
            }
        }
        edits.sort_by_key(|edit| (edit.event_ordinal, edit.level));
        Some(Self::new(edits))
    }

    pub fn edits(&self) -> &[BeamEdit] {
        &self.edits
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grouped_plan_is_typed_and_fail_closed() {
        let first = [0usize, 1, 2];
        let last = [5usize, 6];
        let plan = BeamPlan::from_groups(7, &[(&first[..], 2), (&last[..], 1)]).unwrap();
        assert_eq!(
            plan.edits().iter().copied().map(|edit| (
                edit.event_ordinal(), edit.level(), edit.state().as_musicxml_text()
            )).collect::<Vec<_>>(),
            vec![
                (0, 1, "begin"), (0, 2, "begin"),
                (1, 1, "continue"), (1, 2, "continue"),
                (2, 1, "end"), (2, 2, "end"),
                (5, 1, "begin"), (6, 1, "end"),
            ]
        );
        let overlap = [2usize, 3];
        assert!(BeamPlan::from_groups(7, &[(&first[..], 1), (&overlap[..], 1)]).is_none());
        assert!(BeamPlan::from_groups(3, &[(&[2usize, 1][..], 1)]).is_none());
        assert!(BeamPlan::from_groups(3, &[(&[1usize, 3][..], 1)]).is_none());

        let primary = [0usize, 1, 2, 3];
        let secondary = [0usize, 1];
        let tertiary = [2usize, 3];
        let mixed = BeamPlan::from_level_groups(
            4,
            &[(&primary[..], 1), (&secondary[..], 2), (&tertiary[..], 3)],
        )
        .unwrap();
        assert_eq!(
            mixed
                .edits()
                .iter()
                .copied()
                .map(|edit| (
                    edit.event_ordinal(),
                    edit.level(),
                    edit.state().as_musicxml_text(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (0, 1, "begin"),
                (0, 2, "begin"),
                (1, 1, "continue"),
                (1, 2, "end"),
                (2, 1, "continue"),
                (2, 3, "begin"),
                (3, 1, "end"),
                (3, 3, "end"),
            ]
        );
        assert!(BeamPlan::from_level_groups(
            4,
            &[(&primary[..], 1), (&secondary[..], 1)],
        )
        .is_none());

        let complete = [0usize, 1, 2];
        let sparse_levels = BeamPlan::from_level_groups(
            3,
            &[(&complete[..], 3), (&complete[..], 1)],
        )
        .unwrap();
        assert_eq!(
            sparse_levels
                .edits()
                .iter()
                .copied()
                .map(|edit| (
                    edit.event_ordinal(),
                    edit.level(),
                    edit.state().as_musicxml_text(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (0, 1, "begin"),
                (0, 3, "begin"),
                (1, 1, "continue"),
                (1, 3, "continue"),
                (2, 1, "end"),
                (2, 3, "end"),
            ]
        );
        let singleton = [0usize];
        let pair = [0usize, 1];
        assert!(BeamPlan::from_level_groups(1, &[(&singleton[..], 1)]).is_none());
        assert!(BeamPlan::from_level_groups(2, &[]).is_none());
        assert!(BeamPlan::from_level_groups(2, &[(&pair[..], 0)]).is_none());
        assert!(BeamPlan::from_level_groups(
            2,
            &[(&pair[..], 1), (&pair[..], 1)],
        )
        .is_none());

    }
}
