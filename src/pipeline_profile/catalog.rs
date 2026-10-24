use super::legacy_name_projection::{
    residual_afterburner_name, residual_micro_name, sequenced_profile_name,
};
use super::{CandidateProfileKind, CandidateProfile, ProfileParameters};

pub fn base_solver_profiles() -> Vec<CandidateProfile> {
    vec![
        CandidateProfile::balanced(),
        CandidateProfile::span12(),
        CandidateProfile::jump_smooth(),
        CandidateProfile::playable_safe(),
    ]
}

pub fn residual_afterburner_profiles() -> Vec<CandidateProfile> {
    let spans = [12, 11, 10, 9];
    let jumps = [18, 16, 14, 12, 10, 9];
    let left_highs = [66, 64, 62, 60, 58, 56];
    let right_lows = [58, 60, 62, 64, 66, 68];
    let mut profiles = Vec::new();
    for (i, span) in spans.iter().enumerate() {
        for (j, jump) in jumps.iter().enumerate() {
            let (family, kind) = match (i + j) % 3 {
                0 => (
                    "residual_afterburner_span",
                    CandidateProfileKind::ResidualAfterburnerSpan,
                ),
                1 => (
                    "residual_afterburner_jump",
                    CandidateProfileKind::ResidualAfterburnerJump,
                ),
                _ => (
                    "residual_afterburner_playable",
                    CandidateProfileKind::ResidualAfterburnerPlayable,
                ),
            };
            let left_high = left_highs[(i + j) % left_highs.len()];
            let right_low = right_lows[(i * 2 + j) % right_lows.len()];
            profiles.push(CandidateProfile::new_projected(
                kind,
                residual_afterburner_name(family, *span, *jump, left_high, right_low),
                ProfileParameters::new(*span, *jump, true, left_high, right_low),
            ));
        }
    }
    profiles
}

pub fn local_best_refinement_profiles(best: &CandidateProfile) -> Vec<CandidateProfile> {
    let base = best.local_refinement_basis();
    let base_span = base.span_limit();
    let base_jump = base.jump_limit();
    let base_left_high = base.left_high_soft_limit();
    let base_right_low = base.right_low_soft_limit();
    let span_offsets = [-2, -1, 0, 1, 2];
    let jump_offsets = [-4, -3, -2, -1, 0, 1, 2, 3];
    let register_offsets = [-2, 0, 2];
    let mut profiles = Vec::new();
    let mut sequence = 1;
    for span_offset in span_offsets {
        for jump_offset in jump_offsets {
            for register_offset in register_offsets {
                let span = (base_span + span_offset).clamp(8, 15);
                let jump = (base_jump + jump_offset).clamp(7, 22);
                let left_high = (base_left_high + register_offset).clamp(55, 69);
                let right_low = (base_right_low - register_offset).clamp(55, 69);
                let (family, kind) = match sequence % 4 {
                    0 => ("hotspot_refine_span", CandidateProfileKind::HotspotRefineSpan),
                    1 => ("hotspot_refine_jump", CandidateProfileKind::HotspotRefineJump),
                    2 => (
                        "hotspot_refine_register",
                        CandidateProfileKind::HotspotRefineRegister,
                    ),
                    _ => (
                        "hotspot_refine_balance",
                        CandidateProfileKind::HotspotRefineBalance,
                    ),
                };
                profiles.push(CandidateProfile::new_projected(
                    kind,
                    sequenced_profile_name(
                        family,
                        sequence,
                        span,
                        jump,
                        left_high,
                        right_low,
                    ),
                    ProfileParameters::new(span, jump, true, left_high, right_low),
                ));
                sequence += 1;
            }
        }
    }
    profiles
}

pub fn targeted_critical_profiles() -> Vec<CandidateProfile> {
    let spans = [12, 11, 10, 9, 8];
    let jumps = [18, 16, 14, 12, 10, 9, 8, 7];
    let left_highs = [66, 64, 62, 60, 58];
    let right_lows = [58, 60, 62, 64, 66];
    let mut profiles = Vec::new();
    let mut sequence = 1;
    for (i, span) in spans.iter().enumerate() {
        for (j, jump) in jumps.iter().enumerate() {
            for strict in [false, true] {
                let shift = if strict { 2 } else { 0 };
                let left_high = (left_highs[(i + j) % left_highs.len()] - shift).clamp(55, 69);
                let right_low =
                    (right_lows[(i * 2 + j) % right_lows.len()] + shift).clamp(55, 69);
                let (family, kind) = if strict {
                    (
                        "residual_targeted_hard",
                        CandidateProfileKind::ResidualTargetedHard,
                    )
                } else {
                    (
                        "residual_targeted_safe",
                        CandidateProfileKind::ResidualTargetedSafe,
                    )
                };
                profiles.push(CandidateProfile::new_projected(
                    kind,
                    sequenced_profile_name(
                        family,
                        sequence,
                        *span,
                        *jump,
                        left_high,
                        right_low,
                    ),
                    ProfileParameters::new(*span, *jump, true, left_high, right_low),
                ));
                sequence += 1;
            }
        }
    }
    profiles
}

pub fn critical_micro_repair_profiles(best: &CandidateProfile) -> Vec<CandidateProfile> {
    let base = best.critical_micro_basis();
    let base_span = base.span_limit();
    let base_jump = base.jump_limit();
    let base_left_high = base.left_high_soft_limit();
    let base_right_low = base.right_low_soft_limit();
    let mut profiles = Vec::new();
    let mut sequence = 1;
    let candidates = [
        (0, 0, 0, 0, CandidateProfileKind::HotspotJumpSafeMicro, "hotspot_jump_safe_micro"),
        (-1, 0, 0, 0, CandidateProfileKind::HotspotJumpSafeMicro, "hotspot_jump_safe_micro"),
        (0, -1, 0, 0, CandidateProfileKind::HotspotJumpSafeMicro, "hotspot_jump_safe_micro"),
        (-1, -1, 0, 0, CandidateProfileKind::HotspotJumpSafeMicro, "hotspot_jump_safe_micro"),
        (0, -2, -1, 1, CandidateProfileKind::HotspotJumpSafeMicro, "hotspot_jump_safe_micro"),
        (-1, -2, -1, 1, CandidateProfileKind::HotspotPlayableSafeMicro, "hotspot_playable_safe_micro"),
        (0, -3, -2, 2, CandidateProfileKind::HotspotPlayableSafeMicro, "hotspot_playable_safe_micro"),
        (-2, -2, -2, 2, CandidateProfileKind::HotspotSpan10Micro, "hotspot_span10_micro"),
        (-2, -3, -3, 3, CandidateProfileKind::HotspotSpan10Micro, "hotspot_span10_micro"),
        (1, -2, -2, 2, CandidateProfileKind::HotspotMelodyBassSafeMicro, "hotspot_melody_bass_safe_micro"),
        (0, -4, -3, 3, CandidateProfileKind::HotspotMelodyBassSafeMicro, "hotspot_melody_bass_safe_micro"),
        (-1, -4, -4, 4, CandidateProfileKind::HotspotJumpSafeMicro, "hotspot_jump_safe_micro"),
    ];
    for (span_delta, jump_delta, left_delta, right_delta, kind, family) in candidates {
        let span = (base_span + span_delta).clamp(8, 15);
        let jump = (base_jump + jump_delta).clamp(8, 22);
        let left_high = (base_left_high + left_delta).clamp(55, 69);
        let right_low = (base_right_low + right_delta).clamp(55, 69);
        profiles.push(CandidateProfile::new_projected(
            kind,
            sequenced_profile_name(
                family,
                sequence,
                span,
                jump,
                left_high,
                right_low,
            ),
            ProfileParameters::new(span, jump, true, left_high, right_low),
        ));
        sequence += 1;
    }
    for (span_delta, jump_delta, left_delta, right_delta) in
        [(-1, -1, -1, 1), (-2, -2, -2, 2), (0, -2, -2, 2), (-1, -3, -3, 3)]
    {
        let span = (base_span + span_delta).clamp(8, 15);
        let jump = (base_jump + jump_delta).clamp(8, 22);
        let left_high = (base_left_high + left_delta).clamp(55, 69);
        let right_low = (base_right_low + right_delta).clamp(55, 69);
        profiles.push(CandidateProfile::new_projected(
            CandidateProfileKind::ResidualMicro,
            residual_micro_name(sequence, span, jump, left_high, right_low),
            ProfileParameters::new(span, jump, true, left_high, right_low),
        ));
        sequence += 1;
    }
    profiles
}

#[cfg(test)]
mod tests {
    use super::{
        base_solver_profiles, critical_micro_repair_profiles, local_best_refinement_profiles,
        residual_afterburner_profiles, targeted_critical_profiles,
    };
    use crate::pipeline_profile::{CandidateProfileFamily, CandidateProfile};
    use std::collections::BTreeSet;

    #[test]
    fn current_catalog_has_unique_typed_profile_names() {
        let base = base_solver_profiles();
        let mut profiles = base.clone();
        profiles.extend(residual_afterburner_profiles());
        profiles.extend(local_best_refinement_profiles(&base[0]));
        profiles.extend(targeted_critical_profiles());
        profiles.extend(critical_micro_repair_profiles(&base[0]));
        assert_eq!(base.len(), 4);
        assert_eq!(profiles.len(), 244);
        let names = profiles
            .iter()
            .map(CandidateProfile::display_name)
            .collect::<BTreeSet<_>>();
        assert_eq!(names.len(), profiles.len());
        assert!(names.contains("balanced"));
        assert!(names.contains("residual_afterburner_span_s12_j18_lh66_rl58"));
        assert!(names.iter().all(|name| !name.starts_with("residual_afterburner_v149_")));
        assert!(names.contains("hotspot_refine_jump_1_s10_j14_lh58_rl62"));
        assert!(names.contains("residual_targeted_safe_1_s12_j18_lh66_rl58"));
        assert!(names.contains("hotspot_jump_safe_micro_1_s12_j18_lh58_rl60"));
        assert!(profiles.iter().all(|profile| matches!(
            profile.family(),
            CandidateProfileFamily::Base
                | CandidateProfileFamily::Residual
                | CandidateProfileFamily::Hotspot
        )));
    }
}
