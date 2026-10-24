use super::*;
use crate::master_state_identity::{MasterLineage, RenderTransitionReceipts};
use crate::structural_write_authority::{MasterAdoptedState, RenderSinkKind, RenderSovereignState};

fn pitched(step: &str, octave: usize, staff: usize, member: bool, voice: usize) -> String {
    let chord = if member { "<chord/>" } else { "" };
    format!(
        "<note>{chord}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>96</duration><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff></note>"
    )
}

fn score(base: usize, body: &str) -> String {
    format!(
        "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{base}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>{body}</measure></part></score-partwise>"
    )
}

fn pair(base: usize, upper_head: bool) -> String {
    let low = pitched("C", 3, 2, false, 2).repeat(4);
    let chord = if upper_head {
        format!(
            "{}{}",
            pitched("E", 5, 1, false, 2),
            pitched("C", 5, 2, true, 2)
        )
    } else {
        format!(
            "{}{}",
            pitched("C", 5, 2, false, 2),
            pitched("E", 5, 1, true, 2)
        )
    };
    score(
        base,
        &format!(
            "{low}</measure><measure number=\"{}\">{}</measure><measure number=\"{}\">{low}",
            base + 1,
            chord.repeat(4),
            base + 2
        ),
    )
}

fn previous_element<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
) -> Option<roxmltree::Node<'a, 'input>> {
    let mut previous = node.prev_sibling();
    while previous.is_some_and(|n| !n.is_element()) {
        previous = previous.and_then(|n| n.prev_sibling());
    }
    previous
}

fn assert_atomic(before: &str, after: &str) {
    let original = roxmltree::Document::parse(before).unwrap();
    let result = roxmltree::Document::parse(after).unwrap();
    let notes = |doc: &roxmltree::Document<'_>| {
        doc.descendants()
            .filter(|n| n.has_tag_name("note"))
            .map(|n| doc.input_text()[n.range()].to_owned())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        notes(&original),
        notes(&result),
        "all note fragments preserved"
    );
    for member in result
        .descendants()
        .filter(|n| n.has_tag_name("note") && n.children().any(|c| c.has_tag_name("chord")))
    {
        let previous = previous_element(member);
        // An existing input boundary is allowed, but a new boundary before a member is not.
        if previous.is_some_and(|n| n.has_tag_name("attributes")) {
            let index = result
                .descendants()
                .filter(|n| n.has_tag_name("note"))
                .position(|n| n == member)
                .unwrap();
            let old = original
                .descendants()
                .filter(|n| n.has_tag_name("note"))
                .nth(index)
                .unwrap();
            assert!(
                previous_element(old).is_some_and(|n| n.has_tag_name("attributes")),
                "new clef inside a chord"
            );
        }
    }
}

fn apply(src: &str) -> String {
    let mut post = src.to_owned();
    let (result, receipt) =
        crate::structural_write_authority::apply_rustig_clef_course_to_buffer(&mut post);
    result.unwrap();
    if post != src {
        receipt
            .as_ref()
            .expect("actual E024 change")
            .binds_recorded_representation(src, &post)
            .unwrap();
    }
    let mut receipts = RenderTransitionReceipts::new();
    receipts.push_preserving(receipt, &post);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    assert_atomic(src, &post);
    post
}

#[test]
fn chord_attack_projection_pair_has_one_course_and_atomic_materialization() {
    for base in [1, 40] {
        let a = pair(base, true);
        let b = pair(base, false);
        let first = derive_clef_readability_shadow_course_audit(&a);
        let second = derive_clef_readability_shadow_course_audit(&b);
        assert_eq!(first.part_rows[0].readability_course, "F4>G2>F4");
        assert_eq!(
            first.part_rows[0].readability_course,
            second.part_rows[0].readability_course
        );
        assert_eq!(
            first.part_rows[0].readability_cost,
            second.part_rows[0].readability_cost
        );
        for src in [&a, &b] {
            let post = apply(src);
            assert_ne!(post, *src);
            assert_eq!(apply(&post), post);
        }
    }
}

#[test]
fn chord_attack_projection_hidden_and_rest_heads_keep_foreign_voice_members_together() {
    for base in [1, 40] {
        for head in [
            "<note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>"
                .to_owned(),
            pitched("C", 4, 1, false, 2).replacen("<note>", "<note print-object='no'>", 1),
        ] {
            let member = pitched("F", 4, 2, true, 2);
            let other = pitched("D", 5, 2, true, 6);
            let src = score(
                base,
                &format!("{}{head}{member}{other}", pitched("C", 3, 2, false, 9)),
            );
            let pitches = scan_lower_pitches(&src, None);
            assert_eq!(pitches.len(), 3);
            assert_eq!(collect_lower_staff_attacks(&pitches).len(), 2);
            let head_start = src.find(&head).unwrap();
            assert_eq!(pitches[1].clef_transition_anchor, Some(head_start));
            assert!(pitches[1].counted_attack);
            assert!(!pitches[2].counted_attack);
            assert_eq!(pitches[2].clef_transition_anchor, Some(head_start));
            apply(&src);
        }
    }
}

#[test]
fn chord_attack_projection_unknown_entries_neither_merge_count_nor_restore_continuations() {
    for base in [1, 40] {
        for gap in [
            "<backup><duration>96</duration></backup>",
            "<forward><duration>96</duration></forward>",
            "<attributes><divisions>96</divisions></attributes>",
        ] {
            for beam in ["begin", "continue"] {
                let initial = pitched("C", 5, 2, false, 2).replace(
                    "</note>",
                    "<tie type='start'/><beam number='1'>begin</beam></note>",
                );
                let orphan = pitched("C", 5, 2, true, 2).replace(
                    "</note>",
                    &format!(
                        "<tie type='stop'/><tie type='start'/><beam number='1'>{beam}</beam></note>"
                    ),
                );
                let tail = pitched("C", 5, 2, false, 2).replace(
                    "</note>",
                    "<tie type='stop'/><beam number='1'>end</beam></note>",
                );
                let src = score(base, &format!("{initial}{gap}{orphan}{tail}"));
                let notes = scan_lower_pitches(&src, None);
                assert!(notes[1].starts_attack);
                assert!(!notes[1].counted_attack);
                assert_eq!(notes[1].clef_transition_anchor, None);
                assert_eq!(notes[2].clef_transition_anchor, None);
                assert_eq!(
                    passage_attack_count(&notes, notes[1].start, notes[2].start),
                    0
                );
                assert_eq!(collect_lower_staff_attacks(&notes).len(), 3);
                apply(&src);
            }
        }
        let src = score(
            base,
            &format!(
                "{}</measure><measure number=\"{}\">{}",
                pitched("C", 3, 2, false, 2),
                base + 1,
                pitched("C", 5, 2, true, 2)
            ),
        );
        let notes = scan_lower_pitches(&src, None);
        assert_eq!(notes[1].clef_transition_anchor, None);
        assert!(!notes[1].counted_attack);
    }
}

#[test]
fn chord_attack_projection_resolved_tie_stop_preserves_chain_origin() {
    for base in [1, 40] {
        let origin = pitched("C", 5, 2, false, 2).replace("</note>", "<tie type='start'/></note>");
        let head = pitched("E", 5, 1, false, 2);
        let stop = pitched("C", 5, 2, true, 2).replace("</note>", "<tie type='stop'/></note>");
        let src = score(base, &format!("{origin}{head}{stop}"));
        let notes = scan_lower_pitches(&src, None);
        assert_eq!(
            notes[1].clef_transition_anchor,
            Some(src.find(&origin).unwrap())
        );
        apply(&src);
    }
}

#[test]
fn chord_attack_projection_real_checkpoint58_preserves_the_corrected_pair() {
    const CHILD: &str = "ETUS_TEST_CHORD_ATTACK_CHECKPOINT58";
    if std::env::var_os(CHILD).is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "structural_write_authority::rustig_clef_planning::lower_staff_chord_attack_checkpoint58::chord_attack_projection_real_checkpoint58_preserves_the_corrected_pair", "--nocapture", "--test-threads=1"])
            .env(CHILD, "1")
            .output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            output.status.success(),
            "{}{}",
            stdout,
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            stdout.contains("test result: ok. 1 passed;"),
            "the isolated checkpoint test must actually execute: {stdout}"
        );
        return;
    }
    for base in [1, 40] {
        let mut courses = Vec::new();
        for upper in [false, true] {
            crate::master_state_identity::reset_render_lineage_measurement();
            let src = pair(base, upper);
            let master = MasterAdoptedState::adopt_genesis_state(
                src.clone(),
                MasterLineage::lineage_from_genesis(
                    crate::master_state_identity::tests::synthetic_genesis_for_tests(&src),
                ),
            );
            let path = std::env::temp_dir().join(format!(
                "etuschlopf-chord-checkpoint-{}-{base}-{upper}.render.musicxml",
                std::process::id()
            ));
            let mut state = RenderSovereignState::begin_project_render(&path, &master);
            let root = state.initial_trial_root(true);
            state
                .adopt_transition(
                    root,
                    &src,
                    RenderTransitionReceipts::new(),
                    RenderSinkKind::PrunedPdfTrial,
                    |p, _, post| (std::fs::write(p, post), None),
                )
                .unwrap();
            let root = state.require_state_root("chord attack checkpoint");
            state
                .adopt_final_notation_checkpoint(root, &src, RenderTransitionReceipts::new())
                .unwrap();
            let post = std::fs::read_to_string(&path).unwrap();
            assert_eq!(state.document(), Some(post.as_str()));
            assert_atomic(&src, &post);
            assert_eq!(post.matches("<clef number=\"2\"").count(), 3);
            courses.push(
                derive_clef_readability_shadow_course_audit(&post).part_rows[0]
                    .source_course
                    .clone(),
            );
            std::fs::remove_file(path).unwrap();
        }
        assert_eq!(courses[0], courses[1]);
    }
    crate::master_state_identity::reset_render_lineage_measurement();
}

#[test]
fn chord_attack_projection_independent_parts_retain_their_local_materialization() {
    for base in [1, 40] {
        let first = pair(base, true);
        let second = pair(base + 10, true).replace("id=\"P\"", "id=\"Q\"");
        let doc = roxmltree::Document::parse(&second).unwrap();
        let part = doc.descendants().find(|n| n.has_tag_name("part")).unwrap();
        let combined = first
            .replace(
                "</part-list>",
                "<score-part id=\"Q\"><part-name>Piano</part-name></score-part></part-list>",
            )
            .replace(
                "</score-partwise>",
                &format!("{}</score-partwise>", &second[part.range()]),
            );
        let combined_post = apply(&combined);
        let combined_doc = roxmltree::Document::parse(&combined_post).unwrap();
        for (input, id) in [(&first, "P"), (&second, "Q")] {
            let single = apply(input);
            let single_doc = roxmltree::Document::parse(&single).unwrap();
            let expected = single_doc
                .descendants()
                .find(|n| n.has_tag_name("part"))
                .unwrap();
            let actual = combined_doc
                .descendants()
                .find(|n| n.has_tag_name("part") && n.attribute("id") == Some(id))
                .unwrap();
            assert_eq!(&single[expected.range()], &combined_post[actual.range()]);
        }
    }
}

fn scope_beam(voice: &str, staff: usize, beam: &str) -> String {
    format!("<note><pitch><step>C</step><octave>3</octave></pitch><duration>96</duration><voice>{voice}</voice><staff>{staff}</staff><beam number=\"1\">{beam}</beam></note>")
}

fn lower_scope_event(kind: &str, staff: usize) -> String {
    format!("<direction><direction-type><octave-shift type=\"{kind}\" size=\"8\"/></direction-type><staff>{staff}</staff></direction>")
}

#[test]
fn beam_scope_start_stop_and_repeated_values_end_only_the_old_scope() {
    for voice in ["4", "renamed"] {
        for kind in ["up", "down", "stop"] {
            let src = format!("<part><measure>{}{}{}{}</measure></part>",
                scope_beam(voice, 2, "begin"), lower_scope_event(kind, 2),
                lower_scope_event(kind, 2), scope_beam(voice, 2, "end"));
            let at = src.rfind("<note>").unwrap();
            assert!(!lower_primary_beam_group_contains_position(&src, at));
        }
    }
}

#[test]
fn beam_scope_future_boundary_does_not_release_an_old_scope_interior() {
    for kind in ["up", "stop"] {
        let src = format!("<part><measure>{}<attributes/>{}{}{}</measure></part>",
            scope_beam("v", 2, "begin"), pitched("G", 4, 2, false, 9),
            lower_scope_event(kind, 2), scope_beam("v", 2, "end"));
        assert!(lower_primary_beam_group_contains_position(&src, src.find("<attributes/>").unwrap()));
        assert!(!lower_primary_beam_group_contains_position(&src, src.rfind("<note>").unwrap()));
    }
    // Even without a later note, the known boundary cannot release its prefix.
    let src = format!("<part><measure>{}<attributes/>{}</measure></part>",
        scope_beam("v", 2, "begin"), lower_scope_event("stop", 2));
    let at = src.find("<attributes/>").unwrap();
    assert_eq!(lower_primary_beam_position(&src, at), PrimaryBeamPosition::UnresolvedPrefix);
    assert!(!lower_primary_beam_group_contains_position(&src, at));
    assert!(lower_primary_beam_insertion_blocked(&src, at));
}

#[test]
fn beam_scope_intervening_lower_attack_does_not_resurrect_an_old_group() {
    let src = format!("<part><measure>{}{}{}{}</measure></part>",
        scope_beam("v", 2, "begin"), lower_scope_event("stop", 2),
        pitched("E", 5, 2, false, 9), scope_beam("v", 2, "end"));
    let at = src.rfind("<note>").unwrap();
    assert!(!lower_octave_shift_boundary_precedes_position(&src, at));
    assert!(!lower_primary_beam_group_contains_position(&src, at));
}

#[test]
fn beam_scope_fresh_group_and_upper_only_directions_keep_protection() {
    for kind in ["up", "down", "stop"] {
        let src = format!("<part><measure>{}{}{}{}</measure></part>",
            scope_beam("v", 2, "begin"), lower_scope_event(kind, 1),
            scope_beam("v", 2, "continue"), scope_beam("v", 2, "end"));
        assert!(lower_primary_beam_group_contains_position(&src, src.rfind("<note>").unwrap()));
        let src = format!("<part><measure>{}{}{}{}</measure></part>",
            scope_beam("v", 2, "begin"), lower_scope_event(kind, 2),
            scope_beam("v", 2, "begin"), scope_beam("v", 2, "end"));
        assert!(lower_primary_beam_group_contains_position(&src, src.rfind("<note>").unwrap()));
    }
}

#[test]
fn beam_scope_cross_staff_backup_and_barline_do_not_become_reset_rules() {
    for gap in ["", "<backup><duration>96</duration></backup>", "</measure><measure number=\"renamed\">"] {
        let src = format!("<part><measure>{}{gap}{}</measure></part>",
            scope_beam("v", 1, "begin"), scope_beam("v", 2, "end"));
        assert!(lower_primary_beam_group_contains_position(&src, src.rfind("<note>").unwrap()));
    }
}

#[test]
fn beam_scope_boundary_survives_visibility_filtering_and_unknown_stays_anchorless() {
    let hidden = scope_beam("v", 2, "continue").replace("<note>", "<note print-object=\"no\">");
    let src = format!("<part><measure>{}{}{hidden}{}</measure></part>",
        scope_beam("v", 2, "begin"), lower_scope_event("stop", 2), scope_beam("v", 2, "end"));
    let at = src.rfind("<note>").unwrap();
    assert!(!lower_primary_beam_group_contains_position(&src, at));
    let notes = scan_lower_pitches(&src, None);
    assert_eq!(notes.last().unwrap().clef_transition_anchor, None);
    let unmatched = format!("<part><measure>{}</measure></part>", scope_beam("v", 2, "continue"));
    assert_eq!(scan_lower_pitches(&unmatched, None)[0].clef_transition_anchor, None);
}

#[test]
fn beam_scope_part_isolation_and_independent_duplication_preserve_local_queries() {
    let first = format!("<part id=\"P\"><measure>{}</measure></part>", scope_beam("v", 2, "begin"));
    let second = format!("<part id=\"Q\"><measure>{}</measure></part>", scope_beam("v", 2, "end"));
    let all = format!("<score-partwise>{first}{second}</score-partwise>");
    assert!(!lower_primary_beam_group_contains_position(&all, all.rfind("<note>").unwrap()));
    let local = format!("<part id=\"P\"><measure>{}{}{}</measure></part>",
        scope_beam("v", 2, "begin"), lower_scope_event("stop", 2), scope_beam("v", 2, "end"));
    let independent = format!("<part id=\"Q\"><measure>{}{}</measure></part>", scope_beam("v", 2, "begin"), scope_beam("v", 2, "end"));
    let local_at = local.rfind("<note>").unwrap();
    for prefix in ["", independent.as_str()] {
        let all = format!("<score-partwise>{prefix}{local}{independent}</score-partwise>");
        let start = all.find("<part id=\"P\">").unwrap();
        assert_eq!(lower_primary_beam_group_contains_position(&local, local_at),
            lower_primary_beam_group_contains_position(&all, start + local_at));
        assert!(lower_primary_beam_group_contains_position(&all, all.rfind("<note>").unwrap()));
    }
}

#[test]
fn beam_scope_same_scope_tie_origin_is_not_replaced_by_the_beam_begin() {
    let tie = pitched("C", 3, 2, false, 4).replace("</note>", "<tie type=\"start\"/></note>");
    let begin = scope_beam("4", 2, "begin").replace("</note>", "<tie type=\"stop\"/></note>");
    let src = format!("<part><measure>{tie}{begin}{}</measure></part>", scope_beam("4", 2, "end"));
    let notes = scan_lower_pitches(&src, None);
    assert_eq!(notes.last().unwrap().clef_transition_anchor, Some(src.find("<note>").unwrap()));
    assert!(lower_primary_beam_group_contains_position(&src, src.rfind("<note>").unwrap()));
}

fn beam_scope_course_fixture(base: usize) -> String {
    let low = pitched("C", 3, 2, false, 2);
    let begin = low.replace("<voice>2</voice>", "<voice>7</voice>")
        .replace("</note>", "<beam number=\"1\">begin</beam></note>");
    let end = low.replace("<voice>2</voice>", "<voice>7</voice>")
        .replace("</note>", "<beam number=\"1\">end</beam></note>");
    score(base, &format!("{begin}{}</measure><measure number=\"{}\">{}{}{}</measure><measure number=\"{}\">{}{end}",
        low.repeat(3), base + 1, lower_scope_event("down", 2), lower_scope_event("stop", 2),
        pitched("C", 5, 2, false, 2).repeat(4), base + 2, low.repeat(3)))
}

#[test]
fn beam_scope_actual_materializer_preserves_notes_receipts_and_reentry() {
    for base in [1, 81] {
        let src = beam_scope_course_fixture(base);
        let post = apply(&src);
        assert_ne!(src, post, "scope closure must expose the ordinary high-register course");
        assert_eq!(derive_clef_readability_shadow_course_audit(&post).part_rows[0].source_course, "F4>G2>F4");
        assert_eq!(apply(&post), post);
    }
}

#[test]
fn beam_scope_independent_part_duplication_preserves_actual_materialization() {
    let src = beam_scope_course_fixture(1);
    let part = &src[score_part_ranges(&src)[0].0..score_part_ranges(&src)[0].1];
    let copy = part.replace("id=\"P\"", "id=\"Q\"");
    let combined = src.replace("</part-list>", "<score-part id=\"Q\"><part-name>Piano</part-name></score-part></part-list>")
        .replace("</score-partwise>", &format!("{copy}</score-partwise>"));
    let single = apply(&src);
    let single_part = &single[score_part_ranges(&single)[0].0..score_part_ranges(&single)[0].1];
    let combined_post = apply(&combined);
    let parts = score_part_ranges(&combined_post);
    assert_eq!(&combined_post[parts[0].0..parts[0].1], single_part);
    assert_eq!(&combined_post[parts[1].0..parts[1].1], single_part.replace("id=\"P\"", "id=\"Q\""));
}

#[test]
fn beam_scope_real_checkpoint58_adopts_and_reenters_the_corrected_course() {
    const CHILD: &str = "ETUS_TEST_BEAM_SCOPE_CHECKPOINT58";
    if std::env::var_os(CHILD).is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "structural_write_authority::rustig_clef_planning::lower_staff_chord_attack_checkpoint58::beam_scope_real_checkpoint58_adopts_and_reenters_the_corrected_course", "--nocapture", "--test-threads=1"])
            .env(CHILD, "1").output().unwrap();
        assert!(output.status.success(), "{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stdout).contains("test result: ok. 1 passed;"));
        return;
    }
    for base in [1, 81] {
        crate::master_state_identity::reset_render_lineage_measurement();
        let src = beam_scope_course_fixture(base);
        let expected = apply(&src);
        let master = MasterAdoptedState::adopt_genesis_state(src.clone(), MasterLineage::lineage_from_genesis(
            crate::master_state_identity::tests::synthetic_genesis_for_tests(&src)));
        let path = std::env::temp_dir().join(format!("etuschlopf-beam-scope-{}-{base}.render.musicxml", std::process::id()));
        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(true);
        state.adopt_transition(root, &src, RenderTransitionReceipts::new(), RenderSinkKind::PrunedPdfTrial,
            |p, _, post| (std::fs::write(p, post), None)).unwrap();
        let root = state.require_state_root("beam scope checkpoint");
        state.adopt_final_notation_checkpoint(root, &src, RenderTransitionReceipts::new()).unwrap();
        let post = std::fs::read_to_string(&path).unwrap();
        assert_eq!(state.document(), Some(post.as_str()));
        assert_eq!(post, expected);
        assert_atomic(&src, &post);
        let root = state.require_state_root("beam scope reentry");
        state.adopt_final_notation_checkpoint(root, &post, RenderTransitionReceipts::new()).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), post);
        assert_eq!(state.document(), Some(post.as_str()));
        std::fs::remove_file(path).unwrap();
    }
    crate::master_state_identity::reset_render_lineage_measurement();
}

#[test]
fn beam_scope_unclosed_prefix_cannot_force_current_clef_removal_or_grant_insertion() {
    for base in [1, 81] {
        let low = pitched("C", 3, 2, false, 2);
        let begin = low.replace("<voice>2</voice>", "<voice>7</voice>")
            .replace("</note>", "<beam number=\"1\">begin</beam></note>");
        let g = "<attributes><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>";
        let src = score(base, &format!("{begin}{}</measure><measure number=\"{}\">{g}{}{}{}</measure><measure number=\"{}\">{}",
            low.repeat(3), base + 1, pitched("C", 5, 2, false, 2).repeat(4),
            lower_scope_event("down", 2), lower_scope_event("stop", 2), base + 2, low.repeat(4)));
        let at = src.find(g).unwrap();
        assert_eq!(lower_primary_beam_position(&src, at), PrimaryBeamPosition::UnresolvedPrefix);
        assert!(!lower_primary_beam_group_contains_position(&src, at));
        assert!(lower_primary_beam_insertion_blocked(&src, at));
        let plan = derive_clef_readability_primary_course_plan(&src);
        assert!(!plan.removal_ranges().iter().any(|&(start, end)| start <= at && at < end));
        let post = apply(&src);
        assert!(post.contains(g));
        assert_eq!(apply(&post), post);
    }
}
