#[cfg(test)]
mod empty_rest_lane_preservation_tests {
    use super::*;

    fn pitched(staff: &str, voice: &str, duration: i32, hidden: bool) -> String {
        let visibility = if hidden { " print-object=\"no\"" } else { "" };
        format!(
            "<note{visibility}><pitch><step>C</step><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff></note>"
        )
    }
    fn rest(staff: &str, voice: &str, duration: i32, unit: i32) -> String {
        let kind = match duration / unit {
            4 => "whole",
            2 => "half",
            _ => "quarter",
        };
        format!(
            "<note><rest/><duration>{duration}</duration><voice>{voice}</voice><type>{kind}</type><staff>{staff}</staff></note>"
        )
    }
    fn measure_body(
        unit: i32,
        staff: &str,
        other: &str,
        voice: &str,
        rest_voice: &str,
        durations: &[i32],
        hidden: bool,
    ) -> String {
        let end = unit * 4;
        let mut body = format!(
            "<attributes><divisions>{unit}</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>{}{}<forward><duration>{}</duration></forward><backup><duration>{end}</duration></backup>",
            pitched(staff, voice, unit, false),
            pitched(other, voice, unit, false),
            unit * 2
        );
        if hidden {
            body.push_str(&pitched(staff, rest_voice, unit, true));
            body.push_str(&format!("<forward><duration>{}</duration></forward><backup><duration>{end}</duration></backup>",unit*3));
        }
        for duration in durations {
            body.push_str(&rest(staff, rest_voice, *duration, unit));
        }
        let remaining = end - durations.iter().sum::<i32>();
        if remaining > 0 {
            body.push_str(&format!(
                "<forward><duration>{remaining}</duration></forward>"
            ));
        }
        body
    }
    fn document(bodies: &[String]) -> String {
        let mut xml = String::from(
            "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>synthetic</part-name></score-part></part-list><part id=\"P\">",
        );
        for (index, body) in bodies.iter().enumerate() {
            xml.push_str(&format!(
                "<measure number=\"{}\">{body}</measure>",
                index + 1
            ));
        }
        xml.push_str("</part></score-partwise>");
        xml
    }
    fn preserve(xml: &str) {
        let (edits, actions, _) = empty_voice_rest_suppression_plan(xml, usize::MAX);
        assert!(edits.is_empty());
        let (after, receipt) =
            crate::structural_write_authority::apply_empty_voice_rest_suppression_authorized(
                xml, edits,
            );
        let after = after.unwrap_or_else(|| xml.to_string());
        assert!(receipt.is_none());
        assert_eq!(after, xml);
        assert!(actions.is_empty());
        let (again_edits, again_actions, _) = empty_voice_rest_suppression_plan(&after, usize::MAX);
        let (again, again_receipt) =
            crate::structural_write_authority::apply_empty_voice_rest_suppression_authorized(
                &after,
                again_edits,
            );
        let again = again.unwrap_or_else(|| after.clone());
        assert_eq!(again, after);
        assert!(again_actions.is_empty());
        assert!(again_receipt.is_none());
    }
    #[test]
    fn empty_lanes_preserve_single_and_partitioned_rests_on_both_staffs_under_relabelling_and_scaling()
     {
        for unit in [12, 18] {
            for (staff, other) in [("1", "2"), ("2", "1")] {
                for (voice, rest_voice) in [("7", "11"), ("31", "47")] {
                    for partition in [vec![unit * 4], vec![unit * 2, unit * 2], vec![unit; 4]] {
                        preserve(&document(&[measure_body(
                            unit, staff, other, voice, rest_voice, &partition, false,
                        )]));
                    }
                }
            }
        }
    }
    #[test]
    fn independent_duplicate_and_unrelated_measure_do_not_change_preservation() {
        let unit = 12;
        let a = measure_body(unit, "2", "1", "7", "11", &[unit; 4], false);
        let unrelated = format!(
            "<attributes><divisions>{unit}</divisions></attributes>{}",
            pitched("1", "19", unit * 4, false)
        );
        preserve(&document(&[a.clone(), unrelated, a]));
    }
    #[test]
    fn hidden_pitches_do_not_prove_a_visible_technical_rest_role() {
        let unit = 12;
        preserve(&document(&[measure_body(
            unit, "2", "1", "7", "11", &[unit; 4], true,
        )]));
    }
    #[test]
    fn empty_upper_partial_rest_lanes_are_not_discarded_from_activity_alone() {
        let unit = 12;
        let body = measure_body(unit, "1", "2", "7", "11", &[unit, unit], false);
        preserve(&document(&[format!(
            "<direction><direction-type><octave-shift type=\"down\" size=\"8\"/></direction-type></direction>{body}"
        )]));
    }
    #[test]
    fn original_sounding_lane_covered_rest_branch_still_uses_its_original_receipt() {
        let unit = 12;
        let end = unit * 4;
        let body = format!(
            "<attributes><divisions>{unit}</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{}<backup><duration>{end}</duration></backup>{}<backup><duration>{end}</duration></backup>{}<forward><duration>{}</duration></forward>",
            pitched("1", "7", end, false),
            pitched("2", "7", end, false),
            rest("1", "7", unit, unit),
            end - unit
        );
        let xml = document(&[body]);
        let (edits, actions, _) = empty_voice_rest_suppression_plan(&xml, usize::MAX);
        let (after, receipt) =
            crate::structural_write_authority::apply_empty_voice_rest_suppression_authorized(
                &xml, edits,
            );
        let after = after.expect("original sounding-lane proposal materialized");
        let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
        receipts.push_exact(receipt);
        assert_ne!(after, xml);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            actions[0].action_kind,
            "cross_staff_same_voice_covered_rest_lane_to_forward"
        );
        assert!(!receipts.is_empty());
        assert_eq!(
            render_visible_sounding_signature(&collect_render_xml_note_events(&xml).0),
            render_visible_sounding_signature(&collect_render_xml_note_events(&after).0)
        );
        receipts
            .discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }
    #[test]
    fn zero_action_budget_cannot_create_an_elision_or_a_receipt() {
        let unit = 12;
        let xml = document(&[measure_body(unit, "2", "1", "7", "11", &[unit; 4], false)]);
        let (edits, actions, seen) = empty_voice_rest_suppression_plan(&xml, 0);
        assert!(edits.is_empty());
        assert!(actions.is_empty());
        assert_eq!(seen, 0);
        let (after, receipt) =
            crate::structural_write_authority::apply_empty_voice_rest_suppression_authorized(
                &xml, edits,
            );
        assert!(after.is_none_or(|after| after == xml));
        assert!(receipt.is_none());
    }
}
