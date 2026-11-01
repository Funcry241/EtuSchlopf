use super::*;
fn xml(body: &str) -> String {
    format!(
        "<score-partwise><part id='P1'><measure number='synthetic_alpha'><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><direction><sound tempo='120'/></direction>{body}</measure></part></score-partwise>"
    )
}
fn note(midi: i32, visible: bool, extra: &str) -> String {
    let names = ["C", "C", "D", "D", "E", "F", "F", "G", "G", "A", "A", "B"];
    let pc = midi.rem_euclid(12) as usize;
    let alter = usize::from([1, 3, 6, 8, 10].contains(&pc));
    format!(
        "<note print-object='{}'>{extra}<pitch><step>{}</step><alter>{alter}</alter><octave>{}</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note>",
        if visible { "yes" } else { "no" },
        names[pc],
        midi / 12 - 1
    )
}
fn output(
    body: &str,
    active: usize,
    lineage: usize,
    canonical: usize,
    points: &[MaterialPoint],
) -> String {
    let mut out = String::new();
    append_reference_coverage(
        &mut out,
        &xml(body),
        active,
        lineage,
        canonical,
        true,
        points,
    );
    out
}
#[test]
fn r299_is_counted_once_in_disjoint_partition() {
    let text = output(&note(60, false, ""), 1, 1, 1, &[]);
    assert!(text.contains(
        "group hidden_master_attacks; R299_SUPPORTED 1; STRUCTURAL_ONLY_SPAN_SENSITIVE 0"
    ));
    assert!(text.contains("ExecutionCoverage: status COMPLETE"));
}
#[test]
fn wide_three_key_context_is_structural_only() {
    let text = output(
        &(note(24, false, "") + &note(60, false, "<chord/>") + &note(96, false, "<chord/>")),
        3,
        3,
        3,
        &[],
    );
    assert!(text.contains(
        "group hidden_master_attacks; R299_SUPPORTED 0; STRUCTURAL_ONLY_SPAN_SENSITIVE 3"
    ));
}
#[test]
fn duplicate_heads_preserve_matrix_multiplicity_and_one_attack_boundary() {
    let text = output(
        &(note(60, false, "") + &note(60, false, "<chord/>")),
        2,
        2,
        2,
        &[],
    );
    assert!(text.contains("expected_attack_boundaries 1; measured_attack_boundaries 1"));
    assert!(text.contains("physical_obligations Some(1)"));
    assert!(text.contains("group hidden_master_attacks; R299_SUPPORTED 2"));
}
#[test]
fn release_boundaries_are_not_attack_boundaries() {
    let text = output(&note(60, true, ""), 1, 1, 1, &[]);
    assert!(text.contains("expected_attack_boundaries 1; measured_attack_boundaries 1"));
    assert!(text.contains("expected_all_boundaries Some(2); measured_all_boundaries 2"));
}
#[test]
fn identical_boundary_counts_with_different_ticks_fail_set_equality() {
    let (missing, additional) = set_difference(&BTreeSet::from([0, 96]), &BTreeSet::from([0, 192]));
    assert_eq!(missing, vec![96]);
    assert_eq!(additional, vec![192]);
}
#[test]
fn inactive_and_preselection_rows_use_separate_entry_contexts() {
    let points = [
        MaterialPoint {
            source: 3,
            retained: true,
            tick: Some(0),
        },
        MaterialPoint {
            source: 5,
            retained: false,
            tick: Some(0),
        },
    ];
    let text = output(&note(60, true, ""), 1, 2, 3, &points);
    assert!(text.contains("group inactive_within_lineage; R299_SUPPORTED 1"));
    assert!(text.contains("group removed_before_lineage; R299_SUPPORTED 1"));
    assert!(text.contains("population_rows_exact true; total 2"));
}
#[test]
fn missing_exact_entry_boundary_is_not_approximated() {
    let points = [MaterialPoint {
        source: 3,
        retained: true,
        tick: Some(48),
    }];
    let text = output(&note(60, true, ""), 1, 2, 2, &points);
    assert!(text.contains("group inactive_within_lineage; R299_SUPPORTED 0; STRUCTURAL_ONLY_SPAN_SENSITIVE 0; UNRESOLVED 0; NOT_MEASURED 0; NOT_JOINABLE 1"));
}
#[test]
fn failed_tie_projection_retains_hidden_population_and_original_reason() {
    let body = note(60, false, "<tie type='start'/>") + &note(60, true, "");
    let text = output(&body, 2, 2, 2, &[]);
    assert!(text.contains("ExecutionCoverage: status INCOMPLETE"));
    assert!(text.contains("new-attack-before-open-tie-stop"));
    assert!(text.contains("group hidden_master_attacks; R299_SUPPORTED 0; STRUCTURAL_ONLY_SPAN_SENSITIVE 0; UNRESOLVED 0; NOT_MEASURED 1"));
}
#[test]
fn tied_continuation_does_not_create_hidden_attack() {
    let text = output(
        &(note(60, false, "<tie type='start'/>") + &note(60, false, "<tie type='stop'/>")),
        1,
        1,
        1,
        &[],
    );
    assert!(text.contains("population_rows_exact true; total 1"));
    assert!(text.contains("ExecutionCoverage: status COMPLETE"));
}
#[test]
fn unavailable_zero_input_is_not_complete_zero_coverage() {
    let mut text = String::new();
    append_reference_coverage(&mut text, "broken", 0, 0, 0, true, &[]);
    assert!(text.contains("ExecutionCoverage: status INCOMPLETE"));
    assert!(text.contains("hidden_count unknown"));
}
#[test]
fn duplicate_source_points_cannot_claim_exact_population() {
    let points = [
        MaterialPoint {
            source: 3,
            retained: true,
            tick: Some(0),
        },
        MaterialPoint {
            source: 3,
            retained: true,
            tick: Some(0),
        },
    ];
    let text = output(&note(60, true, ""), 1, 3, 3, &points);
    assert!(text.contains("population_rows_exact false"));
}
#[test]
fn out_of_keyboard_input_fails_closed_before_boundary_classification() {
    let text = output(&note(12, false, ""), 1, 1, 1, &[]);
    assert!(text.contains("class NOT_MEASURED"));
    assert!(text.contains("execution-event-outside-keyboard"));
    assert!(text.contains("all_boundary_coverage_equal false"));
}
#[test]
fn grace_is_not_silently_converted_to_a_regular_attack() {
    let body =
        "<note print-object='no'><grace/><pitch><step>C</step><octave>4</octave></pitch></note>";
    let text = output(body, 1, 1, 1, &[]);
    assert!(text.contains("pitched-grace-note-unmodeled"));
    assert!(text.contains("group hidden_master_attacks; R299_SUPPORTED 0; STRUCTURAL_ONLY_SPAN_SENSITIVE 0; UNRESOLVED 0; NOT_MEASURED 0; NOT_JOINABLE 0; sum 0"));
}
#[test]
fn partial_bar_uses_execution_parser_time_without_nominal_padding() {
    let body = xml(&note(60, false, ""));
    let body = body.replace(
        "</measure></part>",
        &format!(
            "</measure><measure number='synthetic_beta'>{}</measure></part>",
            note(62, false, "")
        ),
    );
    let positions = printed_attack_positions(&body).unwrap();
    assert_eq!(
        positions.iter().map(|p| p.tick).collect::<Vec<_>>(),
        vec![0, 96]
    );
}
#[test]
fn divisions_change_remains_exact_on_shared_time_axis() {
    let body = xml(&(note(60, false, "")
        + "<attributes><divisions>48</divisions></attributes>"
        + &note(62, false, "")));
    let positions = printed_attack_positions(&body).unwrap();
    assert_eq!(
        positions.iter().map(|p| p.tick).collect::<Vec<_>>(),
        vec![0, 96]
    );
    let score = project_xml("synthetic".into(), &body);
    assert_eq!(
        score
            .execution_events
            .iter()
            .map(|p| p.attack_tick)
            .collect::<Vec<_>>(),
        vec![0, 96]
    );
}
#[test]
fn differing_metric_padding_prevents_inactive_time_join() {
    let body = xml(&note(60, true, ""));
    let body = body.replace(
        "</measure></part>",
        &format!(
            "</measure><measure number='synthetic_beta'>{}</measure></part>",
            note(62, true, "")
        ),
    );
    let points = [MaterialPoint {
        source: 3,
        retained: true,
        tick: Some(96),
    }];
    let mut text = String::new();
    append_reference_coverage(&mut text, &body, 2, 3, 3, true, &points);
    assert!(text.contains("exact_master_attack_time_axes_equal false"));
    assert!(text.contains("group inactive_within_lineage; R299_SUPPORTED 0; STRUCTURAL_ONLY_SPAN_SENSITIVE 0; UNRESOLVED 0; NOT_MEASURED 0; NOT_JOINABLE 1"));
}
#[test]
fn fractional_tick_is_unknown_without_rounding() {
    let body =
        xml(&note(60, false, "")).replace("<divisions>96</divisions>", "<divisions>5</divisions>");
    let mut text = String::new();
    append_reference_coverage(&mut text, &body, 1, 1, 1, true, &[]);
    assert!(text.contains("nonexact-canonical-96-projection"));
    assert!(text.contains("ExecutionCoverage: status INCOMPLETE"));
    assert!(text.contains("NOT_MEASURED 1"));
}

#[test]
fn document_voice_order_does_not_change_time_axis_equality() {
    let body = note(60, true, "")
        + &note(62, true, "")
        + "<backup><duration>192</duration></backup>"
        + &note(64, true, "");
    let points = [MaterialPoint {
        source: 3,
        retained: true,
        tick: Some(0),
    }];
    let text = output(&body, 3, 4, 4, &points);
    assert!(text.contains("exact_master_attack_time_axes_equal true"));
    assert!(text.contains("group inactive_within_lineage; R299_SUPPORTED 1"));
}

#[test]
fn normal_report_transformations_preserve_context_and_coverage() {
    let body = output(&note(60, false, ""), 1, 1, 1, &[]);
    let mut report = crate::final_report::DraftReport::new(String::new()).compact(str::to_string);
    report
        .project_owned_section(
            "material_reference_coverage",
            SECTION,
            SECTION_RULE,
            |out| out.push_str(&body),
        )
        .unwrap();
    let before = report.render_snapshot();
    let after = crate::report::sanitize_final_locator_suffixes(
        crate::report::sanitize_measure_locator_authority_report(
            crate::report::clarify_legacy_music_mutation_report(before.clone()),
        ),
    );
    assert_eq!(after, before);
    assert!(after.contains("ExecutionCoverage: status COMPLETE"));
    assert!(after.contains("group hidden_master_attacks; R299_SUPPORTED 1"));
}

#[test]
fn eleven_distinct_keys_are_unresolved_without_positive_support() {
    let body = (0..11)
        .map(|i| note(60 + i, false, if i == 0 { "" } else { "<chord/>" }))
        .collect::<Vec<_>>()
        .join("");
    let text = output(&body, 11, 11, 11, &[]);
    assert!(text.contains("class UNRESOLVED"));
    assert!(text.contains("more-than-ten-simultaneous-distinct-keys"));
    assert!(text.contains("all_boundary_coverage_equal false"));
}
