use super::{CandidateProfileFamily, CandidateProfile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateLineageStep {
    Imported,
    Trajectory,
    HandRepaired,
    GaussGesture,
    GaussChain,
    BrokenGesture,
    Combined,
    Tail,
}

impl CandidateLineageStep {
    const fn bit(self) -> u16 {
        match self {
            Self::Imported => 1 << 0,
            Self::Trajectory => 1 << 1,
            Self::HandRepaired => 1 << 2,
            Self::GaussGesture => 1 << 3,
            Self::GaussChain => 1 << 4,
            Self::BrokenGesture => 1 << 5,
            Self::Combined => 1 << 6,
            Self::Tail => 1 << 7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateLineage {
    steps: u16,
}

impl CandidateLineage {
    pub const fn from_profile(profile: &CandidateProfile) -> Self {
        let mut steps = CandidateLineageStep::Imported.bit();
        if matches!(profile.family(), CandidateProfileFamily::Trajectory) {
            steps |= CandidateLineageStep::Trajectory.bit();
        }
        Self { steps }
    }

    pub const fn contains(self, step: CandidateLineageStep) -> bool {
        self.steps & step.bit() != 0
    }

    pub const fn with_hand_repaired(mut self) -> Self {
        self.steps |= CandidateLineageStep::HandRepaired.bit();
        self
    }

    pub const fn with_gauss_gesture(mut self) -> Self {
        self.steps |= CandidateLineageStep::GaussGesture.bit();
        self
    }

    pub const fn with_gauss_chain(mut self) -> Self {
        self.steps |= CandidateLineageStep::GaussChain.bit();
        self
    }

    pub const fn with_broken_gesture(mut self) -> Self {
        self.steps |= CandidateLineageStep::BrokenGesture.bit();
        if self.contains(CandidateLineageStep::GaussChain) {
            self.steps |= CandidateLineageStep::Combined.bit();
        }
        self
    }

    pub const fn with_tail(mut self) -> Option<Self> {
        if !self.contains(CandidateLineageStep::Combined) {
            return None;
        }
        self.steps |= CandidateLineageStep::Tail.bit();
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{CandidateLineage, CandidateLineageStep};
    use crate::pipeline_profile::CandidateProfile;

    #[test]
    fn tail_transition_is_fail_closed_without_combined_history() {
        let imported = CandidateLineage::from_profile(&CandidateProfile::balanced());
        assert_eq!(imported.with_tail(), None);
        let combined = imported.with_gauss_chain().with_broken_gesture();
        assert!(combined.contains(CandidateLineageStep::Imported));
        assert!(combined.contains(CandidateLineageStep::Combined));
        assert!(combined.with_tail().is_some());
    }
}
