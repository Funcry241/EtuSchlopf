use super::*;

fn fragment(divisions: i32, staff: &str, voice: &str, hidden: bool, chord: bool) -> String {
    format!("<note{}>{}<pitch><step>C</step><octave>5</octave></pitch><duration>{divisions}</duration><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff></note>",
        if hidden { " print-object=\"no\"" } else { "" }, if chord { "<chord/>" } else { "" })
}

fn measure_body(divisions: i32, staff: &str, voice: &str, evidence: &str, simultaneous: bool) -> String {
    let other = if staff == "1" { "2" } else { "1" };
    let first = match evidence {
        "hidden" => fragment(divisions, staff, voice, true, false),
        "chord" => format!("{}{}", fragment(divisions, staff, "OtherHead", true, false), fragment(divisions, staff, voice, false, true)),
        _ => fragment(divisions, staff, voice, false, false),
    };
    let lead = if simultaneous { 0 } else { divisions };
    let lead_fragment = if lead > 0 { format!("<forward><duration>{lead}</duration></forward>") } else { String::new() };
    format!("<attributes><divisions>{divisions}</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{first}<forward><duration>{}</duration></forward><backup><duration>{}</duration></backup>{lead_fragment}{}<forward><duration>{}</duration></forward>",
        divisions * 3, divisions * 4, fragment(divisions, other, voice, false, false), divisions * 3 - lead)
}

fn score_at(scope: usize, body: &str) -> String {
    format!("<score-partwise><part id=\"Synthetic\"><measure number=\"{scope}\">{body}</measure></part></score-partwise>")
}

fn refused(xml: &str, budget: usize) -> Vec<String> {
    let (out, actions, seen, trace, receipts) =
        apply_strict_staff_local_release_completeness_materialization_with_receipts(xml, budget);
    assert!(seen > 0, "the original candidate must remain observable: {trace:?}");
    assert_eq!(out, xml);
    assert!(actions.is_empty());
    assert!(receipts.is_empty(), "no new occurrence or receipt may be fabricated");
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    assert!(trace.iter().any(|row| row.contains("no_overlay_unbound_cross_staff_rest_context")));
    trace
}

#[test]
fn partial_cross_staff_context_refuses_real_owner_without_relabeling() {
    for (scope, divisions, voice, staff) in [(17_usize, 96, "OpaqueA", "1"), (911_usize, 96, "Renamed", "2")] {
        let body = measure_body(divisions, staff, voice, "visible", false);
        let trace = refused(&score_at(scope, &body), 16);
        assert_eq!(trace.iter().filter(|row| row.contains("no_overlay_unbound_cross_staff_rest_context")).count(), 2);
    }
}

#[test]
fn hidden_and_chord_carriers_do_not_license_new_cross_staff_silence() {
    for evidence in ["hidden", "chord"] {
        let body = measure_body(96, "1", "SameChannel", evidence, false);
        refused(&score_at(257, &body), 16);
    }
}

#[test]
fn independent_duplication_preserves_each_local_refusal() {
    let body = measure_body(96, "1", "Shared", "visible", false);
    let first = score_at(37, &body);
    let second = score_at(853, &body);
    refused(&first, 16);
    refused(&second, 16);
    let duplicated = first.replace("</part>", &format!("<measure number=\"{}\">{body}</measure></part>", 853_usize));
    let trace = refused(&duplicated, 16);
    assert_eq!(trace.iter().filter(|row| row.contains("no_overlay_unbound_cross_staff_rest_context")).count(), 4);
}

#[test]
fn refusal_keeps_legacy_budget_from_promoting_an_unrelated_write() {
    let body = measure_body(96, "1", "Shared", "visible", false);
    let first = score_at(17, &body);
    let simple = format!("<measure number=\"{}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{}<forward><duration>288</duration></forward></measure>", 911_usize, fragment(96, "1", "Independent", false, false));
    let combined = first.replace("</part>", &format!("{simple}</part>"));
    refused(&combined, 1);
}

#[test]
fn existing_cross_staff_material_and_unambiguous_neighbor_are_preserved() {
    // Both pitched heads already overlap. Their later gaps have no pitched conflict;
    // the existing ordinary branch and its real receipt coverage remain available.
    let mut body = measure_body(96, "1", "Legitimate", "visible", true);
    // Keep the other staff complete so this positive control has one original
    // overlay insertion. Multiple same-offset insertions are not this guard's scope.
    let last = body.rfind("<forward><duration>288</duration></forward>").unwrap();
    body.replace_range(last.., "<note><rest/><duration>288</duration><voice>Legitimate</voice><type>half</type><dot/><staff>2</staff></note>");
    let source = score_at(73, &body);
    let (out, actions, _, trace, receipts) =
        apply_strict_staff_local_release_completeness_materialization_with_receipts(&source, 16);
    assert!(!actions.is_empty());
    assert!(!receipts.is_empty());
    assert!(!trace.iter().any(|row| row.contains("no_overlay_unbound_cross_staff_rest_context")));
    assert_eq!(out.matches("<pitch>").count(), source.matches("<pitch>").count());
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);

    let complete_body = body.replacen("<forward><duration>288</duration></forward>",
        "<note><rest/><duration>288</duration><voice>Legitimate</voice><type>half</type><dot/><staff>1</staff></note>", 1);
    let complete = score_at(73, &complete_body);
    let (out, actions, _, _, receipts) = apply_strict_staff_local_release_completeness_materialization_with_receipts(&complete, 16);
    assert_eq!(out, complete);
    assert!(actions.is_empty());
    assert!(receipts.is_empty());
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
}

#[test]
fn unsupported_metric_bases_remain_non_writing() {
    for divisions in [24, 48, 192] {
        let source = score_at(607, &measure_body(divisions, "1", "Renamed", "visible", false));
        let (out, actions, _, _, receipts) = apply_strict_staff_local_release_completeness_materialization_with_receipts(&source, 16);
        assert_eq!(out, source);
        assert!(actions.is_empty());
        assert!(receipts.is_empty());
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }
}
