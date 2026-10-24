use super::*;

// Synthetic fixtures only. The measure ordinal is formatted, not a fixture literal.
fn score(body: &str) -> String {
    format!(
        "<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{}\"><attributes><divisions>96</divisions><staves>2</staves></attributes>{body}</measure></part></score-partwise>",
        1
    )
}

fn head() -> &'static str {
    "<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>"
}

fn member(open: &str, step: &str) -> String {
    format!("{open}<chord/><pitch><step>{step}</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>")
}

const CONCEALED_CURSOR_ONLY: &str = "<note print-object=\"no\" print-spacing=\"no\">";
const CONCEALED_PARTICIPATING: &str = "<note print-object=\"no\">";

#[test]
fn opening_tag_participation_is_parsed_structurally() {
    assert_eq!(retention_p2_source_participates_natively(&member(CONCEALED_CURSOR_ONLY, "E")), Some(false));
    assert_eq!(retention_p2_source_participates_natively(&member(CONCEALED_PARTICIPATING, "E")), Some(true));
    assert_eq!(retention_p2_source_participates_natively(&member("<note print-spacing='no' print-object='no'>", "E")), Some(false));
    assert_eq!(retention_p2_source_participates_natively(&member("<note print-spacing=\"yes\">", "E")), Some(true));
    // text inside the note body must not decide participation
    assert_eq!(retention_p2_source_participates_natively("<note><pitch><step>E</step><octave>4</octave></pitch><notations><other-notation print-spacing=\"no\"/></notations></note>"), Some(true));
    assert_eq!(retention_p2_source_participates_natively("<rest/>"), None);
    assert_eq!(retention_p2_source_participates_natively("no tag"), None);
}

#[test]
fn cursor_only_source_is_refused_without_write_or_witness() {
    let master = score(&format!("{}{}", head(), member(CONCEALED_CURSOR_ONLY, "E")));
    let render = score(head());
    let ((out, audit), receipt) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master, &render);
    assert_eq!(out, render);
    assert!(!audit.render_written);
    assert!(audit.generated_occurrences.is_empty());
    assert!(receipt.is_none());
}

#[test]
fn participating_neighbour_is_still_returned() {
    let master = score(&format!("{}{}", head(), member(CONCEALED_PARTICIPATING, "E")));
    let render = score(head());
    let ((out, audit), receipt) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master, &render);
    assert!(audit.render_written);
    assert_eq!(audit.generated_occurrences.len(), 1);
    assert!(receipt.is_some());
    assert!(out.contains("<step>E</step>"));
    assert!(!out.contains("print-spacing=\"no\""));
}

#[test]
fn refused_candidate_is_not_replaced_by_a_later_admissible_one() {
    // two lost candidates -> ceil(2/2) = 1 selected; the selected one is cursor-only
    let master = score(&format!(
        "{}{}{}",
        head(),
        member(CONCEALED_CURSOR_ONLY, "E"),
        member(CONCEALED_PARTICIPATING, "G")
    ));
    let render = score(head());
    let ((out, audit), _receipt) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master, &render);
    assert_eq!(out, render, "no backfill after refusal");
    assert!(audit.generated_occurrences.is_empty());
}

#[test]
fn decision_is_invariant_under_relabeling_and_duplication() {
    let base = |voice: &str| -> String {
        member(CONCEALED_CURSOR_ONLY, "E").replace("<voice>1</voice>", &format!("<voice>{voice}</voice>"))
    };
    for voice in ["1", "7"] {
        let master = score(&format!("{}{}", head(), base(voice)));
        let render = score(head());
        let ((out, _), _) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master, &render);
        assert_eq!(out, render);
    }
}

#[test]
fn ambiguous_lost_identity_is_refused_before_the_writer() {
    // master: head, concealed cursor-only twin, visible twin; render: head, surviving twin.
    // The byte inventory cannot tell which twin survived, so nothing may be returned.
    let master = score(&format!("{}{}{}", head(), member(CONCEALED_CURSOR_ONLY, "E"), member("<note>", "E")));
    let render = score(&format!("{}{}", head(), member("<note>", "E")));
    let ((out, audit), receipt) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master, &render);
    assert_eq!(out, render);
    assert!(audit.generated_occurrences.is_empty());
    assert!(receipt.is_none());
    // same twins in the opposite source order
    let master = score(&format!("{}{}{}", head(), member("<note>", "E"), member(CONCEALED_CURSOR_ONLY, "E")));
    let ((out, audit), _) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master, &render);
    assert_eq!(out, render);
    assert!(audit.generated_occurrences.is_empty());
}

#[test]
fn ambiguous_participating_twins_are_refused_too() {
    let master = score(&format!("{}{}{}", head(), member(CONCEALED_PARTICIPATING, "E"), member("<note>", "E")));
    let render = score(&format!("{}{}", head(), member("<note>", "E")));
    let ((out, audit), _) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master, &render);
    assert_eq!(out, render);
    assert!(audit.generated_occurrences.is_empty());
}

#[test]
fn all_refused_keeps_total_retained_receipts_for_twins() {
    let surviving = format!("{}{}{}",head(),member("<note>","E"),member("<note>","E"));
    let render = score(&surviving);
    let master = score(&format!("{surviving}{}",member(CONCEALED_CURSOR_ONLY,"G")));
    let ((out,audit),receipt) = apply_retention_first_p2_assertive_return_write_explicit_gate(&master,&render);
    assert_eq!(out,render);assert!(!audit.render_written);assert!(receipt.is_none());
    assert!(audit.generated_occurrences.is_empty());assert!(audit.occurrence_receipt_complete);
    assert_eq!(audit.retained_occurrences.len(),3);
    for (ordinal,row) in audit.retained_occurrences.iter().enumerate() {
        assert_eq!(row.input_pitched_ordinal,ordinal);assert_eq!(row.pitched_ordinal,ordinal);
    }
}

#[test]
fn no_requested_return_keeps_total_retained_receipt_rows() {
    let render=score(&format!("{}{}{}",head(),member("<note>","E"),member("<note>","E")));
    let ((out,audit),receipt)=apply_retention_first_p2_assertive_return_write_explicit_gate(&render,&render);
    assert_eq!(out,render);assert!(!audit.render_written);assert!(receipt.is_none());
    assert!(audit.occurrence_receipt_complete);assert_eq!(audit.retained_occurrences.len(),3);
}
