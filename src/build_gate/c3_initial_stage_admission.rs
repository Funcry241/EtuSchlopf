//! User-authorized exact C3 wiring extension. No runtime writer or selector.
//! The old checked sequence is unchanged except for the two bound owner calls.
const PARENT: &[u8] = include_bytes!("ssc_c3_initial_stage_parent_r1562.txt");
const SUCCESSOR: &[u8] = include_bytes!("ssc_c3_initial_stage_successor_r1564.txt");
const PARENT_SHA256: &str = "72a4088ddbdde09c7961e9be6d9026658ae117685a632f3fd3dff6379523801c";
const SUCCESSOR_SHA256: &str = "679392f7a05ece779bbee90fca82114c34184b338a18e138990b1c396b74ba7b";

// Exact source image for retaining the existing custody in the initial Render result.
const RENDER_BEAM_CUSTODY_SHA256: &str = "cafaaa1b2cfecb9d428cd93623ed8627ee3aa2ee37d46bb8072be5b673d15e6d";

pub(super) fn accepts(bytes: &[u8]) -> bool {
    (bytes == SUCCESSOR || crate::sha256::hex(bytes) == RENDER_BEAM_CUSTODY_SHA256)
        && crate::sha256::hex(PARENT) == PARENT_SHA256
        && crate::sha256::hex(SUCCESSOR) == SUCCESSOR_SHA256
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn successor_adds_only_the_two_owner_calls_to_the_actual_parent() {
        let parent = std::str::from_utf8(PARENT).unwrap();
        let first = "    console_artifact_line(\n        ConsoleArtifactLine::PreRenderXml,";
        let last = "    let pre_preview_render_xml_for_gap = render_state.observed_document_or(best.layout_xml()).to_owned();";
        assert_eq!(parent.matches(first).count(), 1);
        assert_eq!(parent.matches(last).count(), 1);
        let expected = parent.replacen(first, &format!("    let joint_reading_custody = begin_joint_reading_render_custody(&best);\n{first}"), 1)
            .replacen(last, &format!("    apply_joint_reading_course(&mut best, &mut render_state, joint_reading_custody)?;\n{last}"), 1);
        assert_eq!(expected.as_bytes(), SUCCESSOR);
        assert!(accepts(SUCCESSOR));
        assert!(!accepts(PARENT)); // the established matcher still admits PARENT
    }
    #[test]
    fn custody_successor_changes_only_the_three_handoff_sites() {
        let source = std::str::from_utf8(SUCCESSOR).unwrap();
        let edits = [
            ("    pre_preview_render_xml_for_gap: String,\n",
             "    pre_preview_render_xml_for_gap: String,\n    render_beam_custody: JointReadingRenderCustody,\n"),
            ("    apply_joint_reading_course(&mut best, &mut render_state, joint_reading_custody)?;",
             "    apply_joint_reading_course(&mut best, &mut render_state, joint_reading_custody.clone())?;"),
            ("            pre_preview_render_xml_for_gap,\n",
             "            pre_preview_render_xml_for_gap,\n            render_beam_custody: joint_reading_custody,\n"),
        ];
        let mut draft = source.to_owned();
        for (old, new) in edits {
            assert_eq!(draft.matches(old).count(), 1);
            draft = draft.replacen(old, new, 1);
        }
        assert_eq!(crate::sha256::hex(draft.as_bytes()), RENDER_BEAM_CUSTODY_SHA256);
        assert!(accepts(draft.as_bytes()));
        for (old, new) in edits {
            assert!(!accepts(source.replacen(old, new, 1).as_bytes()));
            assert!(!accepts(draft.replacen(new, old, 1).as_bytes()));
        }
        assert!(!accepts(format!("{draft}\n").as_bytes()));
    }

    #[test]
    fn partial_additional_or_relocated_hooks_do_not_match() {
        let source = std::str::from_utf8(SUCCESSOR).unwrap();
        for line in ["    let joint_reading_custody = begin_joint_reading_render_custody(&best);\n",
            "    apply_joint_reading_course(&mut best, &mut render_state, joint_reading_custody)?;\n"] {
            assert!(!accepts(source.replacen(line, "", 1).as_bytes()));
            assert!(!accepts(format!("{line}{source}").as_bytes()));
            let moved = source.replacen(line, "", 1) + line;
            assert!(!accepts(moved.as_bytes()));
        }
        assert!(!accepts(format!("{source}\n").as_bytes()));
    }
    #[test]
    fn guard_epoch_and_original_owner_edits_stay_rejected() {
        let source = std::str::from_utf8(SUCCESSOR).unwrap();
        for (old, new) in [("check_master_cursor_boundary(best.layout_xml())?", "check_master_cursor_boundary(best.layout_xml()).unwrap()"),
            ("render_state.bind_late_master_epoch(", "render_state.bind_other_master_epoch("),
            ("let final_export_xml = best.layout_xml().to_owned();", "let final_export_xml = String::new();") ] {
            assert!(source.contains(old));
            assert!(!accepts(source.replacen(old,new,1).as_bytes()));
        }
    }
}
