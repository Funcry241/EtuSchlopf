#[test]
fn voice_entry_without_register_context_can_use_multiple_metric_rests_on_either_staff() {
    for staff in ["1", "2"] {
        let note = pitched("C", 5, 168, false, false)
            .replace("<staff>1</staff>", &format!("<staff>{staff}</staff>"));
        let input = score(&format!("<forward><duration>216</duration></forward>{note}"));
        let ((output, _, written, _, duration, _, _, _, _, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&input, false).expect("prepare");
        discard_probe(receipts);
        assert_eq!((written, duration), (1, 216));
        assert_eq!(output.matches("<rest/>").count(), 2);
        let (before, _) = collect_render_xml_note_events(&input);
        let (after, green) = collect_render_xml_note_events(&output);
        assert!(green);
        assert_eq!(render_visible_sounding_signature(&before), render_visible_sounding_signature(&after));
        let ((second, _, written, _, _, _, _, _, _, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&output, false).expect("prepare");
        discard_probe(receipts);
        assert_eq!(written, 0);
        assert_eq!(second, output);
    }
}

#[test]
fn activity_on_the_other_staff_only_blocks_a_rest_in_the_same_voice() {
    let first = pitched("F", 3, 48, false, false).replace("<staff>1</staff>", "<staff>2</staff>");
    let second = pitched("C", 5, 336, false, false);
    for (other_voice, expected) in [(false, 0), (true, 1)] {
        let first = if other_voice { first.replace("<voice>1</voice>", "<voice>2</voice>") }
            else { first.clone() };
        let input = score(&format!("{first}{second}"));
        let ((_, _, written, _, _, _, _, _, _, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&input, false).expect("prepare");
        discard_probe(receipts);
        assert_eq!(written, expected);
    }
}

#[test]
fn partial_coverage_keeps_existing_rests_and_fills_only_uncovered_intervals() {
    let input = score(&format!(
        "{}<forward><duration>36</duration></forward>{}<forward><duration>48</duration></forward>{}",
        rest(24, "16th"),
        rest(12, "32nd"),
        pitched("C", 5, 264, false, false),
    ));
    let ((output, _, written, _, duration, _, _, _, _, _, _), receipts) =
        apply_upper_staff_initial_rest_and_beat_local_coalescing(&input, false).expect("prepare");
    discard_probe(receipts);
    assert_eq!((written, duration), (1, 84));
    let (before, _) = collect_render_xml_note_events(&input);
    let (after, cursor_green) = collect_render_xml_note_events(&output);
    assert!(cursor_green);
    assert_eq!(render_visible_sounding_signature(&before), render_visible_sounding_signature(&after));
    let mut rests = after.iter().filter(|event| event.is_rest)
        .map(|event| (event.onset, event.duration)).collect::<Vec<_>>();
    rests.sort_unstable();
    assert_eq!(rests, [(0, 24), (24, 36), (60, 12), (72, 24), (96, 24)]);

    let ((second, _, written, _, _, _, _, _, _, _, _), receipts) =
        apply_upper_staff_initial_rest_and_beat_local_coalescing(&output, false).expect("prepare");
    discard_probe(receipts);
    assert_eq!(written, 0);
    assert_eq!(second, output);
}

