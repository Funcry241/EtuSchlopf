use super::*;

fn score(body: &str) -> String {
    format!(
        "<score-partwise><part id='P'><measure number='x'><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes>{body}</measure></part></score-partwise>"
    )
}
fn tone(extra: &str, duration: i128, voice: &str, visible: bool) -> String {
    format!(
        "<note print-object='{}'>{extra}<pitch><step>C</step><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>1</staff></note>",
        if visible { "yes" } else { "no" }
    )
}
fn clef(sign: &str, line: usize) -> String {
    format!("<attributes><clef><sign>{sign}</sign><line>{line}</line></clef></attributes>")
}
fn octave(kind: &str, offset: Option<i128>) -> String {
    format!(
        "<direction><direction-type><octave-shift type='{kind}' size='8'/></direction-type>{}<staff>1</staff></direction>",
        offset.map_or_else(String::new, |n| format!("<offset>{n}</offset>"))
    )
}
fn balance(body: &str) -> Balance {
    calculate_balance(&score(body)).expect("synthetic measured score")
}

#[test]
fn redundant_clefs_and_empty_sections_have_separate_counts() {
    let head = tone("", 4, "1", true);
    assert_eq!(balance(&head).counts, [0; 5]);
    let b = balance(&format!("{}{}", clef("G", 2), head));
    assert_eq!(b.counts, [1, 0, 0, 0, 0]);
    let b = balance(&format!("{head}{}", clef("F", 4)));
    assert_eq!(b.counts, [0, 1, 0, 0, 0]);
}
#[test]
fn clef_line_and_octave_are_part_of_value() {
    let head = tone("", 4, "1", true);
    assert_eq!(
        balance(&format!("{head}{}{head}", clef("G", 1))).counts[0],
        0
    );
    assert_eq!(balance(&format!("{head}<attributes><clef><sign>G</sign><line>2</line><clef-octave-change>1</clef-octave-change></clef></attributes>{head}")).counts[0], 0);
}
#[test]
fn invisible_heads_do_not_cover_clef_sections() {
    assert_eq!(balance(&tone("", 4, "1", false)).counts[1], 1);
    assert_eq!(balance(&tone("", 4, "1", true)).counts[1], 0);
}
#[test]
fn repeated_clef_elements_at_one_time_remain_observable() {
    let b = balance(&format!(
        "{}{}{}",
        clef("G", 2),
        clef("G", 2),
        tone("", 4, "1", true)
    ));
    assert_eq!(b.counts[0], 2);
}
#[test]
fn octave_pair_has_defined_positive_and_negative_cases() {
    let head = tone("", 4, "1", true);
    assert_eq!(
        balance(&format!(
            "{}{head}{}",
            octave("down", None),
            octave("stop", None)
        ))
        .counts[2..4],
        [0, 0]
    );
    assert_eq!(
        balance(&format!("{}{head}", octave("stop", None))).counts[2],
        1
    );
    assert_eq!(
        balance(&format!("{}{head}", octave("down", None))).counts[2],
        1
    );
    assert_eq!(
        balance(&format!(
            "{}{}{head}{}",
            octave("down", None),
            octave("up", None),
            octave("stop", None)
        ))
        .counts[2],
        1
    );
}
#[test]
fn octave_section_requires_visible_head_and_includes_stop_instant() {
    let hidden = tone("", 4, "1", false);
    assert_eq!(
        balance(&format!(
            "{}{hidden}{}",
            octave("down", None),
            octave("stop", None)
        ))
        .counts[3],
        1
    );
    let visible = tone("", 4, "1", true);
    assert_eq!(
        balance(&format!(
            "{}{}{visible}",
            octave("down", None),
            octave("stop", None)
        ))
        .counts[3],
        0
    );
}
#[test]
fn direction_offset_changes_effective_time() {
    let head = tone("", 4, "1", true);
    let shifted = balance(&format!(
        "{}{head}{}",
        octave("down", Some(4)),
        octave("stop", Some(4))
    ));
    let unshifted = balance(&format!(
        "{}{head}{}",
        octave("down", None),
        octave("stop", Some(4))
    ));
    assert_eq!(shifted.counts[3], 1);
    assert_eq!(unshifted.counts[3], 0);
    let notation = collect_notation(&score(&format!(
        "{}{head}{}",
        octave("down", Some(4)),
        octave("stop", Some(4))
    )))
    .unwrap();
    assert_eq!(
        quarter_text(notation.surfaces[1].place.at, notation.resolution),
        "1/1"
    );
    assert_eq!(
        quarter_text(notation.surfaces[2].place.at, notation.resolution),
        "2/1"
    );
}
#[test]
fn temporal_order_across_voices_covers_octave_section() {
    // The head at t=1 is serialized after the octave stop at t=2.
    let b = balance(&format!(
        "{}<forward><duration>8</duration></forward>{}<backup><duration>4</duration></backup>{}",
        octave("down", None),
        octave("stop", None),
        tone("", 4, "2", true)
    ));
    assert_eq!(b.counts[2..4], [0, 0]);
}
#[test]
fn ties_have_defined_positive_and_negative_cases() {
    let start = tone("<tie type='start'/>", 4, "1", true);
    let stop = tone("<tie type='stop'/>", 4, "1", true);
    assert_eq!(balance(&format!("{start}{stop}")).counts[4], 0);
    assert_eq!(balance(&start).counts[4], 1);
    assert_eq!(balance(&stop).counts[4], 1);
    let head = tone("", 4, "1", true);
    let b = balance(&format!("{start}{head}{stop}"));
    assert_eq!(b.counts[4], 2);
    assert_eq!(
        b.findings
            .iter()
            .filter(|f| f.rule == 5)
            .map(|f| f.reason)
            .collect::<Vec<_>>(),
        ["new_attack_before_tie_stop", "tie_stop_without_start"]
    );
}
#[test]
fn ties_include_invisible_fragments_and_distinguish_voices() {
    let start = tone("<tie type='start'/>", 4, "a", false);
    let stop = tone("<tie type='stop'/>", 4, "a", false);
    assert_eq!(balance(&format!("{start}{stop}")).counts[4], 0);
    assert_eq!(
        balance(&format!(
            "{start}{}",
            tone("<tie type='stop'/>", 4, "b", true)
        ))
        .counts[4],
        2
    );
}
#[test]
fn tie_continuations_and_enharmonic_pitch_identity_are_exact() {
    let start = tone("<tie type='start'/>", 4, "1", true)
        .replace("<step>C</step>", "<step>B</step><alter>1</alter>")
        .replace("<octave>4</octave>", "<octave>3</octave>");
    let middle = tone("<tie type='stop'/><tie type='start'/>", 4, "1", false);
    let stop = tone("<tie type='stop'/>", 4, "1", true);
    assert_eq!(balance(&format!("{start}{middle}{stop}")).counts[4], 0);
}
#[test]
fn grace_same_pitch_inside_open_tie_is_excluded() {
    let start = tone("<tie type='start'/>", 4, "1", true);
    let grace =
        "<note><grace/><pitch><step>C</step><octave>4</octave></pitch><voice>1</voice></note>";
    let stop = tone("<tie type='stop'/>", 4, "1", true);
    let b = balance(&format!("{start}{grace}{stop}"));
    assert_eq!(b.counts[4], 0);
    assert_eq!(b.excluded_grace, 1);
    assert_eq!(b.demands[0].all, 1);
    assert_eq!(b.hidden, 0);
}
#[test]
fn grace_does_not_create_a_visible_partner_or_cover_surfaces() {
    let grace =
        "<note><grace/><pitch><step>C</step><octave>4</octave></pitch><voice>1</voice></note>";
    let b = balance(&format!("{grace}{}", tone("", 4, "1", false)));
    assert_eq!(b.counts[1], 1);
    assert_eq!((b.hidden, b.with_partner, b.without_partner), (1, 0, 1));
    assert_eq!(balance(grace).demands.len(), 0);
}
#[test]
fn equal_onset_duration_across_voices_forms_one_event() {
    let a = tone("", 4, "1", true);
    let b = tone("", 4, "2", true);
    let balance = balance(&format!("{a}<backup><duration>4</duration></backup>{b}"));
    assert_eq!(
        (balance.demands[0].all, balance.demands[0].used_all),
        (1, 2)
    );
}
#[test]
fn different_durations_at_same_onset_are_distinct_intervals() {
    let b = balance(&format!(
        "{}<backup><duration>4</duration></backup>{}",
        tone("", 4, "1", true),
        tone("", 8, "2", true)
    ));
    assert_eq!((b.demands[0].all, b.demands[0].visible), (2, 2));
}
#[test]
fn half_open_intervals_end_before_next_begin() {
    let b = balance(&format!(
        "{}{}",
        tone("", 4, "1", true),
        tone("", 4, "2", true)
    ));
    assert_eq!(b.demands[0].all, 1);
}
#[test]
fn visible_population_is_separate_and_partner_requires_same_voice() {
    let hidden = tone("", 8, "2", false);
    let b = balance(&format!(
        "{}<backup><duration>4</duration></backup>{hidden}",
        tone("", 4, "1", true)
    ));
    assert_eq!((b.demands[0].all, b.demands[0].visible), (2, 1));
    let b = balance(&format!(
        "{}{}",
        tone("", 4, "1", true),
        tone("<chord/>", 4, "2", false)
    ));
    assert_eq!((b.hidden, b.with_partner, b.without_partner), (1, 0, 1));
    let b = balance(&format!(
        "{}{}",
        tone("", 4, "1", true),
        tone("<chord/>", 4, "1", false)
    ));
    assert_eq!((b.hidden, b.with_partner, b.without_partner), (1, 1, 0));
}
#[test]
fn hidden_tie_continuations_are_fragments_and_duplicates_are_preserved() {
    let hidden = tone("<chord/><tie type='stop'/>", 4, "1", false);
    let b = balance(&format!("{}{hidden}{hidden}", tone("", 4, "1", true)));
    assert_eq!((b.hidden, b.with_partner), (2, 2));
    assert_eq!(b.counts[4], 2);
    assert_eq!(b.demands[0].all, 1);
}
#[test]
fn divisions_change_inside_measure_preserves_exact_time_and_equivalence() {
    let text = format!(
        "{}<backup><duration>4</duration></backup><attributes><divisions>8</divisions></attributes>{}",
        tone("", 4, "1", true),
        tone("", 8, "2", false)
    );
    let n = collect_notation(&score(&text)).unwrap();
    assert_eq!(quarter_text(n.tones[0].duration, n.resolution), "1/1");
    assert_eq!(quarter_text(n.tones[1].duration, n.resolution), "1/1");
    assert_eq!(n.tones[0].place.at, n.tones[1].place.at);
    assert_eq!(balance(&text).demands[0].all, 1);
}
#[test]
fn meter_change_and_pickup_have_explicit_measure_boundaries() {
    let head = tone("", 4, "1", true);
    let xml = format!(
        "<score-partwise><part id='P'><measure number='a'><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{head}</measure><measure number='b'><attributes><time><beats>3</beats><beat-type>4</beat-type></time></attributes>{head}</measure><measure number='c'>{head}</measure></part></score-partwise>"
    );
    let n = collect_notation(&xml).unwrap();
    assert_eq!(
        n.tones
            .iter()
            .map(|t| quarter_text(t.place.at, n.resolution))
            .collect::<Vec<_>>(),
        ["0/1", "4/1", "7/1"]
    );
    assert!(n.tones.iter().all(|t| t.place.local == 0));
    let pickup = xml.replace("number='a'", "number='a' implicit='yes'");
    let n = collect_notation(&pickup).unwrap();
    assert_eq!(quarter_text(n.tones[1].place.at, n.resolution), "1/1");
}
#[test]
fn fractional_durations_offsets_and_composite_meter_are_exact() {
    let text = "<direction><direction-type><octave-shift type='down'/></direction-type><offset>0.5</offset></direction><note><pitch><step>C</step><alter>0.5</alter><octave>4</octave></pitch><duration>1.5</duration></note><direction><direction-type><octave-shift type='stop'/></direction-type></direction>";
    let n = collect_notation(&score(text)).unwrap();
    assert_eq!(quarter_text(n.surfaces[1].place.at, n.resolution), "1/8");
    assert_eq!(quarter_text(n.tones[0].duration, n.resolution), "3/8");
    assert_eq!(
        n.tones[0].pitch,
        Decimal {
            numerator: 121,
            denominator: 2
        }
    );
    let xml = score(text).replace(
        "<beats>4</beats><beat-type>4</beat-type>",
        "<beats>3+2</beats><beat-type>8</beat-type>",
    );
    let n = collect_notation(&xml).unwrap();
    assert_eq!(quarter_text(n.ends[&0], n.resolution), "5/2");
}
#[test]
fn separate_parts_do_not_share_tie_or_clef_state() {
    let start = tone("<tie type='start'/>", 4, "1", true);
    let stop = tone("<tie type='stop'/>", 4, "1", true);
    let xml = format!(
        "<score-partwise><part id='a'><measure><attributes><divisions>4</divisions></attributes>{start}</measure></part><part id='b'><measure><attributes><divisions>4</divisions></attributes>{stop}</measure></part></score-partwise>"
    );
    let b = calculate_balance(&xml).unwrap();
    assert_eq!(b.counts[4], 2);
    assert_eq!(b.demands.len(), 2);
}
#[test]
fn unsupported_or_malformed_input_is_unavailable() {
    assert!(calculate_balance("<broken>").is_err());
    assert!(
        calculate_balance(
            &score(&tone("", 4, "1", true))
                .replace("<divisions>4</divisions>", "<divisions>0</divisions>")
        )
        .is_err()
    );
    assert!(calculate_balance(&score("<backup><duration>4</duration></backup>")).is_err());
    assert!(calculate_balance(&score(&tone("<chord/>", 4, "1", true))).is_err());
}

#[test]
fn declared_section_and_existing_report_transformations_preserve_measurements() {
    let xml = score(&tone("<tie type='stop'/>", 4, "1", true));
    let mut body = String::new();
    append_notation_balance(&mut body, &xml, Some(&xml));
    let mut report = crate::final_report::DraftReport::new(String::new()).compact(str::to_string);
    report
        .project_owned_section("notation_invariant_balance", SECTION, SECTION_RULE, |out| {
            out.push_str(&body)
        })
        .unwrap();
    let before = report.render_snapshot();
    let after = crate::report::sanitize_final_locator_suffixes(
        crate::report::sanitize_measure_locator_authority_report(
            crate::report::clarify_legacy_music_mutation_report(before.clone()),
        ),
    );
    assert_eq!(after, before);
    assert!(before.contains("R5 1"));
    assert!(before.contains("measure_index 1"));
    assert!(!before.contains("DOCUMENT_UNAVAILABLE"));
}
