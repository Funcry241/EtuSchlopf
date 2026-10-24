use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateProfileFamily {
    Base,
    Residual,
    Trajectory,
    Hotspot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateProfileKind {
    Balanced,
    Span12,
    JumpSmooth,
    PlayableSafe,
    ResidualAfterburnerSpan,
    ResidualAfterburnerJump,
    ResidualAfterburnerPlayable,
    ResidualTargetedSafe,
    ResidualTargetedHard,
    ResidualMicro,
    TrajectorySafe,
    TrajectoryDeepSafe,
    HotspotRefineSpan,
    HotspotRefineJump,
    HotspotRefineRegister,
    HotspotRefineBalance,
    HotspotJumpSafeMicro,
    HotspotPlayableSafeMicro,
    HotspotSpan10Micro,
    HotspotMelodyBassSafeMicro,
}

// These legacy trajectory variants are part of the closed old policy even though
// the current catalog does not emit them. Constructing them here keeps that
// exhaustiveness visible to the compiler without exposing a raw profile builder.
const _: [CandidateProfileKind; 2] = [
    CandidateProfileKind::TrajectorySafe,
    CandidateProfileKind::TrajectoryDeepSafe,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileParameters {
    span_limit: i32,
    jump_limit: i32,
    smooth_jumps: bool,
    left_high_soft_limit: i32,
    right_low_soft_limit: i32,
}

impl ProfileParameters {
    pub const fn new(
        span_limit: i32,
        jump_limit: i32,
        smooth_jumps: bool,
        left_high_soft_limit: i32,
        right_low_soft_limit: i32,
    ) -> Self {
        Self {
            span_limit,
            jump_limit,
            smooth_jumps,
            left_high_soft_limit,
            right_low_soft_limit,
        }
    }

    pub const fn span_limit(self) -> i32 {
        self.span_limit
    }

    pub const fn jump_limit(self) -> i32 {
        self.jump_limit
    }

    pub const fn smooth_jumps(self) -> bool {
        self.smooth_jumps
    }

    pub const fn left_high_soft_limit(self) -> i32 {
        self.left_high_soft_limit
    }

    pub const fn right_low_soft_limit(self) -> i32 {
        self.right_low_soft_limit
    }
}

#[derive(Debug, Clone, Copy)]
enum CandidateProfileTraits {
    Standard,
    Playable,
    Targeted,
    Micro,
    DeepSafe,
    MicroJumpSafe,
    MicroPlayable,
    MicroMelodyBassSafe,
}

impl CandidateProfileTraits {
    const fn is_micro(self) -> bool {
        matches!(
            self,
            Self::Micro | Self::MicroJumpSafe | Self::MicroPlayable | Self::MicroMelodyBassSafe
        )
    }

    const fn is_targeted(self) -> bool {
        matches!(self, Self::Targeted)
    }

    const fn is_playable(self) -> bool {
        matches!(self, Self::Playable | Self::MicroPlayable)
    }

    const fn is_deep_safe(self) -> bool {
        matches!(self, Self::DeepSafe)
    }

    const fn is_jump_safe(self) -> bool {
        matches!(self, Self::MicroJumpSafe)
    }

    const fn is_melody_bass_safe(self) -> bool {
        matches!(self, Self::MicroMelodyBassSafe)
    }
}

#[derive(Debug, Clone)]
pub struct CandidateProfile {
    display_name: Arc<str>,
    kind: CandidateProfileKind,
    parameters: ProfileParameters,
    parameters_projected_in_display: bool,
}

impl CandidateProfile {
    pub fn balanced() -> Self {
        Self::new_unprojected(
            CandidateProfileKind::Balanced,
            "balanced",
            ProfileParameters::new(14, 24, false, 72, 55),
        )
    }

    pub fn span12() -> Self {
        Self::new_unprojected(
            CandidateProfileKind::Span12,
            "span12",
            ProfileParameters::new(12, 24, false, 70, 57),
        )
    }

    pub fn jump_smooth() -> Self {
        Self::new_unprojected(
            CandidateProfileKind::JumpSmooth,
            "jump_smooth",
            ProfileParameters::new(14, 19, true, 69, 57),
        )
    }

    pub fn playable_safe() -> Self {
        Self::new_unprojected(
            CandidateProfileKind::PlayableSafe,
            "playable_safe",
            ProfileParameters::new(12, 17, true, 67, 59),
        )
    }

    pub(super) fn new_projected(
        kind: CandidateProfileKind,
        display_name: impl Into<Arc<str>>,
        parameters: ProfileParameters,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            kind,
            parameters,
            parameters_projected_in_display: true,
        }
    }

    pub(super) fn new_unprojected(
        kind: CandidateProfileKind,
        display_name: impl Into<Arc<str>>,
        parameters: ProfileParameters,
    ) -> Self {
        Self {
            display_name: display_name.into(),
            kind,
            parameters,
            parameters_projected_in_display: false,
        }
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[cfg(test)]
    pub const fn kind(&self) -> CandidateProfileKind {
        self.kind
    }

    pub const fn family(&self) -> CandidateProfileFamily {
        match self.kind {
            CandidateProfileKind::Balanced
            | CandidateProfileKind::Span12
            | CandidateProfileKind::JumpSmooth
            | CandidateProfileKind::PlayableSafe => CandidateProfileFamily::Base,
            CandidateProfileKind::ResidualAfterburnerSpan
            | CandidateProfileKind::ResidualAfterburnerJump
            | CandidateProfileKind::ResidualAfterburnerPlayable
            | CandidateProfileKind::ResidualTargetedSafe
            | CandidateProfileKind::ResidualTargetedHard
            | CandidateProfileKind::ResidualMicro => CandidateProfileFamily::Residual,
            CandidateProfileKind::TrajectorySafe | CandidateProfileKind::TrajectoryDeepSafe => {
                CandidateProfileFamily::Trajectory
            }
            CandidateProfileKind::HotspotRefineSpan
            | CandidateProfileKind::HotspotRefineJump
            | CandidateProfileKind::HotspotRefineRegister
            | CandidateProfileKind::HotspotRefineBalance
            | CandidateProfileKind::HotspotJumpSafeMicro
            | CandidateProfileKind::HotspotPlayableSafeMicro
            | CandidateProfileKind::HotspotSpan10Micro
            | CandidateProfileKind::HotspotMelodyBassSafeMicro => CandidateProfileFamily::Hotspot,
        }
    }

    #[cfg(test)]
    pub const fn parameters(&self) -> ProfileParameters {
        self.parameters
    }

    pub const fn local_refinement_basis(&self) -> ProfileParameters {
        if self.parameters_projected_in_display {
            self.parameters
        } else {
            ProfileParameters::new(12, 18, true, 60, 60)
        }
    }

    pub const fn critical_micro_basis(&self) -> ProfileParameters {
        if self.parameters_projected_in_display {
            self.parameters
        } else {
            ProfileParameters::new(12, 18, true, 58, 60)
        }
    }

    pub fn without_projected_parameters(&self) -> Self {
        Self {
            display_name: self.display_name.clone(),
            kind: self.kind,
            parameters: self.parameters,
            parameters_projected_in_display: false,
        }
    }

    pub const fn span_limit(&self) -> i32 {
        self.parameters.span_limit()
    }

    pub const fn jump_limit(&self) -> i32 {
        self.parameters.jump_limit()
    }

    pub const fn smooth_jumps(&self) -> bool {
        self.parameters.smooth_jumps()
    }

    pub const fn left_high_soft_limit(&self) -> i32 {
        self.parameters.left_high_soft_limit()
    }

    pub const fn right_low_soft_limit(&self) -> i32 {
        self.parameters.right_low_soft_limit()
    }

    pub fn focused(&self, focus_score: i64) -> Self {
        if focus_score <= 0 {
            return self.clone();
        }
        let hard = focus_score >= 180;
        Self {
            display_name: self.display_name.clone(),
            kind: self.kind,
            parameters: ProfileParameters::new(
                (self.span_limit() - if hard { 3 } else { 2 }).max(9),
                (self.jump_limit() - if hard { 4 } else { 2 }).max(10),
                true,
                self.left_high_soft_limit() - if hard { 4 } else { 2 },
                self.right_low_soft_limit() + if hard { 3 } else { 2 },
            ),
            parameters_projected_in_display: self.parameters_projected_in_display,
        }
    }

    pub const fn runs_residual_pass(&self) -> bool {
        matches!(self.family(), CandidateProfileFamily::Residual)
    }

    pub const fn runs_trajectory_pass(&self) -> bool {
        matches!(self.family(), CandidateProfileFamily::Trajectory)
    }

    pub const fn runs_hotspot_reduction(&self) -> bool {
        matches!(
            self.family(),
            CandidateProfileFamily::Residual
                | CandidateProfileFamily::Trajectory
                | CandidateProfileFamily::Hotspot
        )
    }

    pub const fn residual_max_passes(&self) -> usize {
        let traits = self.traits();
        if traits.is_micro() || traits.is_targeted() {
            2
        } else if traits.is_playable() {
            4
        } else {
            3
        }
    }

    pub const fn trajectory_max_passes(&self) -> usize {
        if self.traits().is_deep_safe() {
            18
        } else {
            12
        }
    }

    pub const fn register_move_cap(&self, focus_score: i64) -> usize {
        let traits = self.traits();
        if traits.is_micro() {
            2
        } else if traits.is_targeted() {
            3
        } else if focus_score >= 900 {
            5
        } else {
            3
        }
    }

    pub const fn critical_removal_cap(&self, focus_score: i64) -> usize {
        let traits = self.traits();
        if traits.is_micro() {
            2
        } else if traits.is_targeted() {
            3
        } else if focus_score >= 1000 {
            5
        } else if focus_score >= 700 {
            4
        } else {
            2
        }
    }

    pub const fn per_hotspot_cap(&self) -> usize {
        let traits = self.traits();
        if traits.is_micro() {
            4
        } else if traits.is_targeted() {
            5
        } else if matches!(self.family(), CandidateProfileFamily::Residual) {
            10
        } else if traits.is_deep_safe() {
            18
        } else if matches!(self.kind, CandidateProfileKind::TrajectorySafe) {
            16
        } else if traits.is_jump_safe() {
            14
        } else if matches!(self.kind, CandidateProfileKind::HotspotPlayableSafeMicro) {
            16
        } else if matches!(self.kind, CandidateProfileKind::HotspotSpan10Micro) {
            12
        } else if traits.is_melody_bass_safe() {
            10
        } else {
            8
        }
    }

    pub const fn max_notes_kept_per_staff(&self) -> usize {
        if self.traits().is_melody_bass_safe() {
            3
        } else {
            2
        }
    }

    const fn traits(&self) -> CandidateProfileTraits {
        match self.kind {
            CandidateProfileKind::Balanced
            | CandidateProfileKind::Span12
            | CandidateProfileKind::JumpSmooth
            | CandidateProfileKind::HotspotRefineSpan
            | CandidateProfileKind::HotspotRefineJump
            | CandidateProfileKind::HotspotRefineRegister
            | CandidateProfileKind::HotspotRefineBalance
            | CandidateProfileKind::ResidualAfterburnerSpan
            | CandidateProfileKind::ResidualAfterburnerJump
            | CandidateProfileKind::TrajectorySafe => CandidateProfileTraits::Standard,
            CandidateProfileKind::PlayableSafe
            | CandidateProfileKind::ResidualAfterburnerPlayable => CandidateProfileTraits::Playable,
            CandidateProfileKind::ResidualTargetedSafe
            | CandidateProfileKind::ResidualTargetedHard => CandidateProfileTraits::Targeted,
            CandidateProfileKind::ResidualMicro
            | CandidateProfileKind::HotspotSpan10Micro => CandidateProfileTraits::Micro,
            CandidateProfileKind::TrajectoryDeepSafe => CandidateProfileTraits::DeepSafe,
            CandidateProfileKind::HotspotJumpSafeMicro => CandidateProfileTraits::MicroJumpSafe,
            CandidateProfileKind::HotspotPlayableSafeMicro => CandidateProfileTraits::MicroPlayable,
            CandidateProfileKind::HotspotMelodyBassSafeMicro => {
                CandidateProfileTraits::MicroMelodyBassSafe
            }
        }
    }

    #[cfg(test)]
    fn renamed_for_test(&self, display_name: &str) -> Self {
        Self {
            display_name: Arc::<str>::from(display_name),
            kind: self.kind,
            parameters: self.parameters,
            parameters_projected_in_display: self.parameters_projected_in_display,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CandidateProfileKind, CandidateProfile, ProfileParameters};

    #[test]
    fn display_name_does_not_change_profile_semantics() {
        let original = CandidateProfile::new_projected(
            CandidateProfileKind::HotspotMelodyBassSafeMicro,
            "hotspot_melody_bass_safe_micro_1_s12_j18_lh58_rl60",
            ProfileParameters::new(12, 18, true, 58, 60),
        );
        let renamed = original.renamed_for_test("arbitrary report label");
        assert_eq!(original.kind(), renamed.kind());
        assert_eq!(original.family(), renamed.family());
        assert_eq!(original.parameters(), renamed.parameters());
        assert_eq!(original.local_refinement_basis(), renamed.local_refinement_basis());
        assert_eq!(original.critical_micro_basis(), renamed.critical_micro_basis());
        assert_eq!(original.runs_residual_pass(), renamed.runs_residual_pass());
        assert_eq!(original.runs_trajectory_pass(), renamed.runs_trajectory_pass());
        assert_eq!(original.runs_hotspot_reduction(), renamed.runs_hotspot_reduction());
        assert_eq!(original.residual_max_passes(), renamed.residual_max_passes());
        assert_eq!(original.trajectory_max_passes(), renamed.trajectory_max_passes());
        assert_eq!(original.per_hotspot_cap(), renamed.per_hotspot_cap());
        assert_eq!(
            original.max_notes_kept_per_staff(),
            renamed.max_notes_kept_per_staff()
        );
        for focus_score in [0, 699, 700, 899, 900, 999, 1000, 1200] {
            assert_eq!(
                original.register_move_cap(focus_score),
                renamed.register_move_cap(focus_score)
            );
            assert_eq!(
                original.critical_removal_cap(focus_score),
                renamed.critical_removal_cap(focus_score)
            );
        }
    }
}
