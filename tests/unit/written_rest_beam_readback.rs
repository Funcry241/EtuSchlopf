use super::lower_dual_edge_recovery_retirement_tests::score;
use super::read_fixture_rest_beam_errors;

#[test]
fn rest_gap_readback_is_independent_of_staff_position_and_duration() {
    for staff in ["1", "2"] {
        for (duration, value) in [(2, "eighth"), (1, "16th")] {
            let relocate = |xml: String| {
                xml.replace("<staff>2</staff>", &format!("<staff>{staff}</staff>"))
                    .replace("<duration>1</duration>", &format!("<duration>{duration}</duration>"))
                    .replace("<type>16th</type>", &format!("<type>{value}</type>"))
                    .replace("</attributes>", "</attributes><forward><duration>4</duration></forward>")
            };
            for (state, count) in [("continue", 0), ("begin", 1)] {
                let input = relocate(score(state, state, ""));
                assert_eq!(read_fixture_rest_beam_errors(&input), Some(count));
            }
        }
    }
}

