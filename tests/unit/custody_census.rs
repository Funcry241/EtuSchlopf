use super::*;

fn document(notes: &str) -> String {
    format!("<score-partwise><part id=\"P1\"><measure number=\"probe\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{notes}</measure></part></score-partwise>")
}

fn pitched(step: &str, duration: i64, staff: u8) -> String {
    format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>{duration}</duration><voice>1</voice><staff>{staff}</staff></note>")
}

fn rest(duration: i64, staff: u8) -> String {
    format!("<note><rest/><duration>{duration}</duration><voice>1</voice><staff>{staff}</staff></note>")
}

fn census(before: &str, after: &str) -> CustodyCensusCounts {
    OwnedRhythmicContinuation::custody_census(&document(before), &document(after)).unwrap()
}

#[test]
fn identical_documents_are_fully_unchanged() {
    let notes = format!("{}{}", pitched("C", 8, 1), rest(8, 1));
    assert_eq!(census(&notes, &notes), CustodyCensusCounts { unchanged: 2, ..Default::default() });
}

#[test]
fn print_object_change_is_text_only() {
    let before = format!("{}{}", pitched("C", 8, 1), rest(8, 1));
    let after = format!("{}{}", pitched("C", 8, 1), rest(8, 1).replace("<note>", "<note print-object=\"no\">"));
    assert_eq!(census(&before, &after), CustodyCensusCounts { unchanged: 1, text_only: 1, ..Default::default() });
}

#[test]
fn staff_move_with_equal_interval_and_pitch_is_counted_as_move() {
    let before = format!("{}{}", pitched("C", 8, 1), rest(8, 1));
    let after = format!("{}{}", pitched("C", 8, 2), rest(8, 1));
    assert_eq!(census(&before, &after), CustodyCensusCounts { unchanged: 1, staff_moved: 1, ..Default::default() });
}

#[test]
fn pitch_or_restification_is_disposition_change() {
    let before = format!("{}{}", pitched("C", 8, 1), pitched("D", 8, 1));
    let after = format!("{}{}", pitched("E", 8, 1), rest(8, 1));
    assert_eq!(census(&before, &after), CustodyCensusCounts { disposition_changed: 2, ..Default::default() });
}

#[test]
fn split_counts_as_removed_and_added_without_inventing_continuation() {
    let before = pitched("C", 16, 1);
    let after = format!("{}{}", pitched("C", 8, 1), pitched("C", 8, 1));
    assert_eq!(census(&before, &after), CustodyCensusCounts { removed: 1, added: 2, ..Default::default() });
}

#[test]
fn repeated_equal_notes_pair_once_each() {
    let before = format!("{}{}", pitched("C", 8, 1), pitched("C", 8, 1));
    let after = format!("{}{}", pitched("C", 8, 1), rest(8, 1));
    assert_eq!(census(&before, &after), CustodyCensusCounts { unchanged: 1, disposition_changed: 1, ..Default::default() });
}

#[test]
fn unreadable_inventory_is_an_error_not_a_count() {
    let bad = "<score-partwise><part id=\"P1\"><measure number=\"probe\"><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>3</staff></note></measure></part></score-partwise>";
    assert!(OwnedRhythmicContinuation::custody_census(bad, bad).is_err());
}

#[test]
fn census_line_is_projection_text_for_empty_and_unreadable_transitions() {
    let receipts = crate::master_state_identity::OwnerReceipts::default();
    let line = receipts.custody_census_line(&document(&pitched("C", 8, 1)), &document(&pitched("C", 8, 2)), "Probe").unwrap();
    assert!(line.starts_with("ETUSCHLOPF: CustodyCensus: transition=\"Probe\"; receipts=0; "));
    assert!(line.contains("staff_moved=1; disposition_changed=0; removed=0; added=0"));
    assert!(line.ends_with("owners=[]; projection_only=true"));
    let unreadable = receipts.custody_census_line("<x/>", "<score-partwise><part id=\"P1\"><measure number=\"probe\"><note><rest/><duration>4</duration><staff>3</staff></note></measure></part></score-partwise>", "Probe").unwrap();
    assert!(unreadable.contains("inventory_unreadable="));
}

#[test]
fn text_only_transition_without_receipts_is_not_written() {
    let receipts = crate::master_state_identity::OwnerReceipts::default();
    let before = document(&rest(8, 1));
    let after = document(&rest(8, 1).replace("<note>", "<note print-object=\"no\">"));
    assert_eq!(receipts.custody_census_line(&before, &after, "Probe"), None);
}
