use super::*;

fn replay(measure: &str, edits: &[(std::ops::Range<usize>, String)]) -> String {
    let mut out = String::new();
    let mut cursor = 0;
    for (range, text) in edits {
        out.push_str(&measure[cursor..range.start]);
        out.push_str(text);
        cursor = range.end;
    }
    out.push_str(&measure[cursor..]);
    out
}

const NOTE: &str = "\n      <note><rest/><duration>4</duration></note>";

#[test]
fn journal_reproduces_every_legacy_branch_byte_for_byte() {
    let measures = [
        format!("<measure number=\"probe\">{NOTE}\n    </measure>"),
        format!("<measure number=\"probe\">\n      <print new-system=\"yes\"/>{NOTE}\n    </measure>"),
        format!("<measure number=\"probe\">\n      <print><system-layout/></print>{NOTE}\n    </measure>"),
        format!("<measure number=\"probe\"><print><staff-layout number=\"2\"><staff-distance>80</staff-distance></staff-layout></print>{NOTE}</measure>"),
        format!("<measure number=\"probe\"><print><staff-layout number=\"2\"><staff-distance>200</staff-distance></staff-layout></print>{NOTE}</measure>"),
    ];
    for measure in &measures {
        for target in [120, 200] {
            let legacy = ensure_local_staff_distance_like_legacy(measure, target);
            let edits = local_staff_distance_operations(measure, target);
            assert_eq!(replay(measure, &edits), legacy, "{measure} -> {target}");
            assert_eq!(edits.is_empty(), legacy == *measure);
            assert!(edits.iter().all(|(range, text)| !measure[range.clone()].contains("<note") && !text.contains("<note")));
        }
    }
}
