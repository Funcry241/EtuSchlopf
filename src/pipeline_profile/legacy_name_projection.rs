pub fn residual_afterburner_name(
    family: &str,
    span: i32,
    jump: i32,
    left_high: i32,
    right_low: i32,
) -> String {
    format!("{family}_s{span}_j{jump}_lh{left_high}_rl{right_low}")
}

pub fn sequenced_profile_name(
    family: &str,
    sequence: usize,
    span: i32,
    jump: i32,
    left_high: i32,
    right_low: i32,
) -> String {
    format!("{family}_{sequence}_s{span}_j{jump}_lh{left_high}_rl{right_low}")
}

pub fn residual_micro_name(
    sequence: usize,
    span: i32,
    jump: i32,
    left_high: i32,
    right_low: i32,
) -> String {
    sequenced_profile_name(
        "residual_micro",
        sequence,
        span,
        jump,
        left_high,
        right_low,
    )
}

pub fn broken_gesture_name(
    from_gauss_chain: bool,
    measure: usize,
    half: usize,
    take: usize,
    mode_label: &str,
) -> String {
    if from_gauss_chain {
        format!(
            "consensus_gauss_chain_broken_gesture_m{measure}_h{half}_n{take}_{mode_label}"
        )
    } else {
        format!("consensus_broken_gesture_m{measure}_h{half}_n{take}_{mode_label}")
    }
}

pub fn hybrid_tail_name(gauss_chain_display_name: &str) -> String {
    let detail = gauss_chain_display_name
        .strip_prefix("gauss_chain_")
        .unwrap_or(gauss_chain_display_name)
        .trim_end_matches("_least_squares_chain");
    format!("consensus_gauss_chain_broken_gesture_tail_{detail}")
}

pub fn combined_name(current_display_name: &str) -> String {
    let detail = current_display_name
        .strip_prefix("consensus_broken_gesture_")
        .or_else(|| {
            current_display_name.strip_prefix("consensus_gauss_chain_broken_gesture_")
        })
        .unwrap_or(current_display_name);
    format!("consensus_gauss_chain_broken_gesture_{detail}")
}


#[cfg(test)]
mod tests {
    use super::{
        broken_gesture_name, combined_name, hybrid_tail_name, residual_afterburner_name,
        residual_micro_name, sequenced_profile_name,
    };

    #[test]
    fn legacy_display_projection_remains_exact() {
        assert_eq!(
            residual_afterburner_name("residual_afterburner_span", 12, 18, 66, 58),
            "residual_afterburner_span_s12_j18_lh66_rl58"
        );
        assert_eq!(
            sequenced_profile_name("hotspot_refine_jump", 1, 10, 14, 58, 62),
            "hotspot_refine_jump_1_s10_j14_lh58_rl62"
        );
        assert_eq!(
            residual_micro_name(13, 11, 17, 57, 61),
            "residual_micro_13_s11_j17_lh57_rl61"
        );
        assert_eq!(
            broken_gesture_name(true, 12, 1, 2, "safe"),
            "consensus_gauss_chain_broken_gesture_m12_h1_n2_safe"
        );
        assert_eq!(
            hybrid_tail_name("gauss_chain_probe_least_squares_chain"),
            "consensus_gauss_chain_broken_gesture_tail_probe"
        );
        assert_eq!(
            combined_name("consensus_broken_gesture_probe"),
            "consensus_gauss_chain_broken_gesture_probe"
        );
    }
}
