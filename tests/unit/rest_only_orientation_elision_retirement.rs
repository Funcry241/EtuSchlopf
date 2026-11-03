use super::*;

fn score(scope: usize, staff: &str, voice: &str, scale: i32, pitched: bool) -> String {
    let other_voice = format!("{voice}-independent");
    let duration = 96 * scale;
    let mut rests = String::new();
    for _ in 0..4 {
        rests.push_str(&format!("<note><rest/><duration>{duration}</duration><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff></note>"));
    }
    let activity = if pitched {
        format!("<backup><duration>{}</duration></backup><forward><duration>{}</duration></forward><note><pitch><step>E</step><octave>4</octave></pitch><duration>{}</duration><voice>{other_voice}</voice><staff>{staff}</staff></note><forward><duration>{}</duration></forward>", 384 * scale, 120 * scale, 24 * scale, 240 * scale)
    } else { String::new() };
    format!("<score-partwise><part id=\"Synthetic\"><measure number=\"{scope}\"><attributes><divisions>{duration}</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{rests}{activity}</measure></part></score-partwise>")
}

fn retained(xml: String) {
    let original = xml.clone();
    let (changed, receipts) = hide_redundant_visible_staff_rests(&xml);
    assert_eq!(changed, 0);
    assert_eq!(receipts.len(), 0);
    assert_eq!(xml, original, "an orientation glyph is no proof for deleting other rest intervals");
    let (changed_again, receipts_again) = hide_redundant_visible_staff_rests(&xml);
    assert_eq!(changed_again, 0);
    assert_eq!(receipts_again.len(), 0);
    assert_eq!(xml, original);
}

#[test]
fn silent_course_preserves_all_existing_rest_intervals() {
    for (scope, staff, voice, scale) in [(41, "1", "opaque", 1), (817, "2", "renamed", 7)] {
        retained(score(scope, staff, voice, scale, false));
    }
}

#[test]
fn independent_pitched_activity_cannot_license_rest_elision() {
    for (scope, staff, voice) in [(59, "2", "a"), (913, "1", "unrelated")] {
        retained(score(scope, staff, voice, 1, true));
    }
}

#[test]
fn hidden_carriers_and_pitched_bytes_stay_unchanged() {
    let xml = score(109, "2", "opaque", 1, true);
    let xml = xml.replacen("<note><rest/>", "<note print-object=\"no\" print-spacing=\"no\"><rest/>", 1);
    retained(xml);
}

#[test]
fn independent_duplication_does_not_change_preservation() {
    let one = score(127, "2", "opaque", 1, true);
    let two = score(641, "1", "renamed", 1, false);
    let body_start = two.find("<measure ").unwrap();
    let body_end = two.find("</measure>").unwrap() + "</measure>".len();
    let combined = one.replace("</part>", &format!("{}</part>", &two[body_start..body_end]));
    retained(combined);
}
