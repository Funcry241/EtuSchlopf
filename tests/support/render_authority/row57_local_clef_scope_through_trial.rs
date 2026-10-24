// Synthetic, relocatable #57 through-tests: actual planner -> V2 writer -> #57 trial
// -> BLOCK receipt -> RenderSovereignState adoption -> verified render artifact.
// Included in the private pipeline scope; no corpus fixture and no witness identity.

    const STAFF2_F: &str = "<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>";
    const STAFF2_G: &str = "<attributes><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>";
    const STAFF1_F: &str = "<attributes><clef number=\"1\"><sign>F</sign><line>4</line></clef></attributes>";

    fn bar(ordinal: usize, body: &str) -> String {
        format!("<measure number=\"{ordinal}\">{body}</measure>")
    }
    fn score(bars: &[String]) -> String {
        format!("<score-partwise><part id=\"P1\">{}</part></score-partwise>", bars.concat())
    }
    fn opening_attributes(staff2_sign: &str, staff2_line: u8) -> String {
        format!(
            "<attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>{staff2_sign}</sign><line>{staff2_line}</line></clef></attributes>"
        )
    }
    fn rest(voice: u8, duration: u8, staff: u8) -> String {
        let kind = match duration { 4 => "whole", 2 => "half", _ => "quarter" };
        format!("<note><rest/><duration>{duration}</duration><voice>{voice}</voice><type>{kind}</type><staff>{staff}</staff></note>")
    }
    fn pitch(step: char, octave: u8, voice: u8) -> String {
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>1</duration><voice>{voice}</voice><type>quarter</type><staff>2</staff></note>")
    }
    fn high_lower_staff_bar(ordinal: usize, leading: &str) -> String {
        bar(ordinal, &format!("{leading}{}", [pitch('A', 4, 1), pitch('B', 4, 1), pitch('A', 4, 1), pitch('B', 4, 1)].concat()))
    }
    fn treble_opening(ordinal: usize) -> String {
        high_lower_staff_bar(ordinal, &opening_attributes("G", 2))
    }
    fn low_tail() -> String {
        [pitch('C', 3, 1), pitch('D', 3, 1), pitch('E', 3, 1)].concat()
    }

    // ---------------- independent musical oracle (roxmltree, no planner helpers) ----------------

    #[derive(Debug, Clone)]
    enum OracleItem {
        Attributes { text: String, staff2_sign: Option<char> },
        Note { text: String, staff2: bool, rest: bool, hidden: bool, midi: Option<i32> },
        Cursor { text: String },
        Other { text: String },
    }
    impl OracleItem {
        fn text(&self) -> &str {
            match self {
                Self::Attributes { text, .. } | Self::Note { text, .. } | Self::Cursor { text } | Self::Other { text } => text,
            }
        }
    }

    fn oracle_bars(xml: &str) -> Result<Vec<Vec<OracleItem>>, String> {
        let doc = roxmltree::Document::parse(xml).map_err(|error| format!("oracle parse: {error}"))?;
        let child_text = |node: roxmltree::Node, name: &str| {
            node.children().find(|child| child.has_tag_name(name)).and_then(|child| child.text()).map(str::trim).map(str::to_string)
        };
        let mut bars = Vec::new();
        for measure in doc.descendants().filter(|node| node.has_tag_name("measure")) {
            let mut items = Vec::new();
            for child in measure.children().filter(|node| node.is_element()) {
                let text = xml[child.range()].to_string();
                let item = match child.tag_name().name() {
                    "attributes" => {
                        let staff2_sign = child
                            .children()
                            .filter(|clef| clef.has_tag_name("clef"))
                            .find(|clef| clef.attribute("number") == Some("2"))
                            .and_then(|clef| child_text(clef, "sign"))
                            .and_then(|sign| sign.chars().next());
                        OracleItem::Attributes { text, staff2_sign }
                    }
                    "note" => {
                        let staff2 = child_text(child, "staff").as_deref() == Some("2");
                        let midi = child.children().find(|node| node.has_tag_name("pitch")).and_then(|pitch| {
                            let step = match child_text(pitch, "step")?.as_str() {
                                "C" => 0, "D" => 2, "E" => 4, "F" => 5, "G" => 7, "A" => 9, "B" => 11, _ => return None,
                            };
                            let alter = child_text(pitch, "alter").and_then(|value| value.parse::<i32>().ok()).unwrap_or(0);
                            let octave = child_text(pitch, "octave")?.parse::<i32>().ok()?;
                            Some(12 * (octave + 1) + step + alter)
                        });
                        OracleItem::Note {
                            text,
                            staff2,
                            rest: child.children().any(|node| node.has_tag_name("rest")),
                            hidden: child.attribute("print-object") == Some("no"),
                            midi,
                        }
                    }
                    "backup" | "forward" => OracleItem::Cursor { text },
                    _ => OracleItem::Other { text },
                };
                items.push(item);
            }
            bars.push(items);
        }
        Ok(bars)
    }

    /// Effective lower-staff clef of every staff-2 note plus the carry after every bar.
    fn oracle_clef_state(bars: &[Vec<OracleItem>]) -> (Vec<Vec<Option<char>>>, Vec<Option<char>>) {
        let mut active = None;
        let mut per_bar = Vec::new();
        let mut carries = Vec::new();
        for items in bars {
            let mut notes = Vec::new();
            for item in items {
                match item {
                    OracleItem::Attributes { staff2_sign: Some(sign), .. } => active = Some(*sign),
                    OracleItem::Note { staff2: true, .. } => notes.push(active),
                    _ => {}
                }
            }
            per_bar.push(notes);
            carries.push(active);
        }
        (per_bar, carries)
    }

    /// Accepts exactly intended local advances: a lower-staff F clef may move earlier over
    /// low/rest/hidden staff-2 events of its own cursor domain; attribute blocks keep their
    /// mutual order and cursor domain, foreign events keep their bytes and order, and the
    /// end-of-bar carry is unchanged. Returns the number of intentionally re-cleffed events.
    fn local_clef_oracle(pre: &str, post: &str) -> Result<usize, String> {
        let before = oracle_bars(pre)?;
        let after = oracle_bars(post)?;
        if before.len() != after.len() {
            return Err("bar count changed".into());
        }
        let (eff_before, carry_before) = oracle_clef_state(&before);
        let (eff_after, carry_after) = oracle_clef_state(&after);
        if carry_before != carry_after {
            return Err(format!("end-of-bar clef carry changed: {carry_before:?} -> {carry_after:?}"));
        }
        let mut advanced = 0usize;
        for (bar_index, (old, new)) in before.iter().zip(&after).enumerate() {
            let events = |items: &[OracleItem]| items.iter().filter(|item| !matches!(item, OracleItem::Attributes { .. })).map(|item| item.text().to_string()).collect::<Vec<_>>();
            if events(old) != events(new) {
                return Err(format!("bar #{bar_index}: foreign event bytes or order changed"));
            }
            // (event position, cursor domain, text, sign) for every attribute block.
            let placed = |items: &[OracleItem]| {
                let (mut position, mut cursor, mut out) = (0usize, 0usize, Vec::new());
                for item in items {
                    match item {
                        OracleItem::Attributes { text, staff2_sign } => out.push((position, cursor, text.clone(), *staff2_sign)),
                        OracleItem::Cursor { .. } => { cursor += 1; position += 1; }
                        _ => position += 1,
                    }
                }
                out
            };
            let old_blocks = placed(old);
            let new_blocks = placed(new);
            let mut moved = Vec::new();
            let mut search = 0usize;
            for (new_position, new_cursor, text, sign) in &new_blocks {
                let Some(offset) = old_blocks[search..].iter().position(|(_, _, old_text, _)| old_text == text) else {
                    return Err(format!("bar #{bar_index}: attribute block reordered or invented"));
                };
                let (old_position, old_cursor, _, _) = &old_blocks[search + offset];
                search += offset + 1;
                if new_cursor != old_cursor {
                    return Err(format!("bar #{bar_index}: attribute block crossed a backup/forward cursor boundary"));
                }
                if new_position > old_position {
                    return Err(format!("bar #{bar_index}: attribute block moved later"));
                }
                if new_position < old_position {
                    if *sign != Some('F') {
                        return Err(format!("bar #{bar_index}: only lower-staff F clefs may be advanced"));
                    }
                    moved.push((*new_position, *old_position));
                }
            }
            let staff2_events = old
                .iter()
                .filter(|item| !matches!(item, OracleItem::Attributes { .. }))
                .enumerate()
                .filter_map(|(position, item)| match item {
                    OracleItem::Note { staff2: true, rest, hidden, midi, .. } => Some((position, *rest || *hidden || midi.is_some_and(|value| value < 69))),
                    _ => None,
                })
                .collect::<Vec<_>>();
            for (index, (position, low_or_rest)) in staff2_events.iter().enumerate() {
                let (was, now) = (eff_before[bar_index][index], eff_after[bar_index][index]);
                if was == now {
                    continue;
                }
                let authorized = now == Some('F')
                    && *low_or_rest
                    && moved.iter().any(|(new_position, old_position)| new_position <= position && position < old_position);
                if !authorized {
                    return Err(format!("bar #{bar_index}: staff-2 event #{index} changed clef {was:?}->{now:?} outside an authorized local advance"));
                }
                advanced += 1;
            }
        }
        Ok(advanced)
    }

    // ---------------- helpers over the actual owner path ----------------

    fn planned_advance(xml: &str) -> (String, usize) {
        let mut candidate = xml.to_string();
        let moved = advance_lower_staff_bass_clef_before_preceding_low_or_rest_events(&mut candidate)
            .expect("synthetic local batch must authorize");
        (candidate, moved)
    }

    fn trial_post(xml: &str, label: &str) -> (RenderXmlPrintCleanupOneSafeVisualTrial, String) {
        let path = std::env::temp_dir().join(format!("etuschlopf-row57-local-{label}-{}.render.musicxml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let (trial, receipt) = build_render_xml_print_cleanup_one_safe_visual_trial(xml, label, &path);
        let trial = trial.expect("#57 trial must stay a successful owner execution");
        assert!(!path.exists(), "#57 preparation must not write before sovereign adoption");
        let receipt = receipt.expect("changed #57 preparation must mint its BLOCK receipt");
        let (receipt, realized) = receipt.split_realized_post_document();
        let realized = realized.expect("#57 file-bound receipt carries its realized post");
        receipt
            .validate_block_transition::<RenderRow57BlockMint>(xml, &realized)
            .expect("#57 BLOCK receipt binds exact pre/post");
        (trial, realized)
    }

    fn adopt_through_render_state(xml: &str, label: &str) -> String {
        let master = MasterAdoptedState::adopt_genesis_state(
            xml.to_string(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(xml)),
        );
        let path = std::env::temp_dir().join(format!("etuschlopf-row57-adopt-{label}-{}.render.musicxml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(false);
        let sink_writes = std::cell::Cell::new(0usize);
        let trial = state
            .adopt_path_bound_transition(
                root,
                RenderSinkKind::PrintCleanupOneSafeVisual,
                |source_xml| {
                    let (trial_result, receipt) = build_render_xml_print_cleanup_one_safe_visual_trial(source_xml, label, &path);
                    let failed = trial_result.is_err();
                    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
                    let planned_post = receipt.map(|receipt| {
                        let (receipt, realized_post) = receipt.split_realized_post_document();
                        let post = realized_post.expect("#57 receipt carries realized post");
                        receipts.push_block(Some(receipt), &post);
                        post
                    });
                    crate::structural_write_authority::RenderPathBoundOutcome { value: trial_result, planned_post, receipts, failed }
                },
                |artifact_path, _source_xml, candidate_xml| {
                    sink_writes.set(sink_writes.get() + 1);
                    crate::structural_write_authority::write_visual_cleanup_candidate_authorized(artifact_path, candidate_xml)
                },
            )
            .expect("verified #57 adoption")
            .expect("#57 owner execution succeeds");
        assert!(trial.written);
        assert_eq!(sink_writes.get(), 1, "verified #57 sink writes exactly once");
        let adopted = state.document().expect("#57 adopted Render state").to_string();
        assert_eq!(std::fs::read_to_string(&path).ok().as_deref(), Some(adopted.as_str()), "artifact witnesses adopted state");
        let downstream = state.initial_trial_root(false);
        assert_eq!(downstream.document(), adopted, "next trial root consumes the adopted #57 post");
        let _ = std::fs::remove_file(path);
        adopted
    }


    /// Later clef cleanups of the same #57 trial may delete attribute blocks, but only
    /// without changing any staff-2 event's effective clef, any foreign event or the carry.
    fn effect_preserving_oracle(advanced: &str, realized: &str) -> Result<(), String> {
        let before = oracle_bars(advanced)?;
        let after = oracle_bars(realized)?;
        if before.len() != after.len() {
            return Err("bar count changed".into());
        }
        let events = |items: &[OracleItem]| items.iter().filter(|item| !matches!(item, OracleItem::Attributes { .. })).map(|item| item.text().to_string()).collect::<Vec<_>>();
        let blocks = |items: &[OracleItem]| items.iter().filter(|item| matches!(item, OracleItem::Attributes { .. })).map(|item| item.text().to_string()).collect::<Vec<_>>();
        for (old, new) in before.iter().zip(&after) {
            if events(old) != events(new) {
                return Err("cleanup changed foreign events".into());
            }
            let old_blocks = blocks(old);
            let mut remaining = old_blocks.iter();
            if !blocks(new).iter().all(|block| remaining.any(|candidate| candidate == block)) {
                return Err("cleanup reordered or invented attribute blocks".into());
            }
        }
        if oracle_clef_state(&before) != oracle_clef_state(&after) {
            return Err("cleanup changed an effective staff-2 clef or the bar carry".into());
        }
        Ok(())
    }

    /// Two-phase oracle for the complete #57 trial: authorized local advances first,
    /// then effect-free cleanup. Parse success and sounding stability are not sufficient.
    fn trial_effect_oracle(pre: &str, realized: &str) -> Result<usize, String> {
        let (advanced, _) = planned_advance(pre);
        let count = local_clef_oracle(pre, &advanced)?;
        effect_preserving_oracle(&advanced, realized)?;
        Ok(count)
    }

    // ---------------- tests ----------------

    #[test]
    fn row57_single_local_advance_is_exact_and_oracle_bounded() {
        let body = format!("{}{STAFF2_F}{}", rest(1, 1, 2), low_tail());
        let xml = score(&[treble_opening(1), bar(2, &body)]);
        let expected = score(&[treble_opening(1), bar(2, &format!("{STAFF2_F}{}{}", rest(1, 1, 2), low_tail()))]);
        let (advanced, moved) = planned_advance(&xml);
        assert_eq!(moved, 1);
        assert_eq!(advanced, expected);
        assert_eq!(local_clef_oracle(&xml, &advanced), Ok(1));
        let (trial, realized) = trial_post(&xml, "single");
        assert!(trial.written && trial.sounding_signature_stable);
        assert_eq!(trial_effect_oracle(&xml, &realized), Ok(1));
        let adopted = adopt_through_render_state(&xml, "single");
        assert_eq!(adopted, realized);
    }

    #[test]
    fn row57_independent_bars_duplicate_without_changing_local_decisions() {
        let local = format!("{}{STAFF2_F}{}", rest(1, 1, 2), low_tail());
        let one = score(&[treble_opening(1), bar(2, &local)]);
        let two = score(&[treble_opening(1), bar(2, &local), high_lower_staff_bar(3, STAFF2_G), bar(4, &local)]);
        let (one_post, one_moved) = planned_advance(&one);
        let (two_post, two_moved) = planned_advance(&two);
        assert_eq!((one_moved, two_moved), (1, 2));
        let local_ordinal = 2usize;
        let first_bar = |xml: &str| {
            let start = xml.find(&format!("<measure number=\"{local_ordinal}\"")).expect("local bar");
            xml[start..start + xml[start..].find("</measure>").expect("bar end")].to_string()
        };
        assert_eq!(first_bar(&one_post), first_bar(&two_post), "candidate duplication must not alter the local decision");
        assert_eq!(local_clef_oracle(&two, &two_post), Ok(2));
        let (_, realized) = trial_post(&two, "duplicate");
        assert_eq!(trial_effect_oracle(&two, &realized), Ok(2));
        assert_eq!(adopt_through_render_state(&two, "duplicate"), realized);
    }

    #[test]
    fn row57_touching_envelopes_are_independent_and_overlaps_fail_before_mutation() {
        let (a, b) = (rest(1, 1, 2), pitch('C', 3, 1));
        let touching = bar(7, &format!("{a}{STAFF2_F}{b}{STAFF2_F}"));
        let a_at = touching.find(&a).expect("a");
        let first_f = touching.find(STAFF2_F).expect("first F");
        let b_at = first_f + STAFF2_F.len();
        let second_f = b_at + b.len();
        let request = |old: usize, insert: usize| {
            crate::structural_write_authority::LowerStaffClefMoveRequest::new(old, old + STAFF2_F.len(), insert, STAFF2_F.to_string())
        };
        let mut xml = touching.clone();
        let changed = crate::structural_write_authority::apply_lower_staff_clef_moves_authorized(&mut xml, vec![request(first_f, a_at), request(second_f, b_at)])
            .expect("touching envelopes are independent");
        assert_eq!(changed, 2);
        assert_eq!(xml, bar(7, &format!("{STAFF2_F}{a}{STAFF2_F}{b}")));

        for (label, requests) in [
            ("shared insertion target", vec![request(first_f, a_at), request(second_f, a_at)]),
            ("nested envelope", vec![request(second_f, a_at), request(first_f, a_at)]),
            ("duplicate source", vec![request(first_f, a_at), request(first_f, a_at)]),
        ] {
            let mut xml = touching.clone();
            let error = crate::structural_write_authority::apply_lower_staff_clef_moves_authorized(&mut xml, requests)
                .expect_err(label);
            assert_eq!(xml, touching, "{label}: rejected batch must leave original bytes untouched ({error})");
        }
    }

    #[test]
    fn row57_f_g_f_produces_two_disjoint_local_advances() {
        let body = format!(
            "{}{STAFF2_F}{}{STAFF2_G}{}{STAFF2_F}{}",
            rest(1, 1, 2), pitch('C', 3, 1), rest(1, 1, 2), pitch('D', 3, 1)
        );
        let xml = score(&[treble_opening(1), bar(2, &body)]);
        let expected_body = format!(
            "{STAFF2_F}{}{}{STAFF2_G}{STAFF2_F}{}{}",
            rest(1, 1, 2), pitch('C', 3, 1), rest(1, 1, 2), pitch('D', 3, 1)
        );
        let (advanced, moved) = planned_advance(&xml);
        assert_eq!(moved, 2);
        assert_eq!(advanced, score(&[treble_opening(1), bar(2, &expected_body)]));
        assert_eq!(local_clef_oracle(&xml, &advanced), Ok(2));
        let (trial, realized) = trial_post(&xml, "fgf");
        assert!(trial.empty_clef_ranges_removed + trial.redundant_clefs_removed + trial.rest_only_clef_noops_removed > 0, "the now-empty G is pruned by the existing trial");
        assert_eq!(trial_effect_oracle(&xml, &realized), Ok(2));
        assert_eq!(adopt_through_render_state(&xml, "fgf"), realized);
    }

    #[test]
    fn row57_cursor_and_attribute_boundaries_bound_the_insertion_target() {
        let backup = format!(
            "{}<backup><duration>4</duration></backup>{}{STAFF2_F}{}",
            rest(1, 4, 2), rest(2, 1, 2), [pitch('C', 3, 2), pitch('D', 3, 2), pitch('E', 3, 2)].concat()
        );
        let backup_expected = format!(
            "{}<backup><duration>4</duration></backup>{STAFF2_F}{}{}",
            rest(1, 4, 2), rest(2, 1, 2), [pitch('C', 3, 2), pitch('D', 3, 2), pitch('E', 3, 2)].concat()
        );
        let forward = format!(
            "{}<forward><duration>1</duration></forward>{}{STAFF2_F}{}",
            rest(1, 1, 2), rest(1, 1, 2), pitch('C', 3, 1)
        );
        let forward_expected = format!(
            "{}<forward><duration>1</duration></forward>{STAFF2_F}{}{}",
            rest(1, 1, 2), rest(1, 1, 2), pitch('C', 3, 1)
        );
        let attributes = format!("{}{STAFF1_F}{}{STAFF2_F}{}{}", rest(1, 1, 2), rest(1, 1, 2), pitch('C', 3, 1), pitch('D', 3, 1));
        let attributes_expected = format!("{}{STAFF1_F}{STAFF2_F}{}{}{}", rest(1, 1, 2), rest(1, 1, 2), pitch('C', 3, 1), pitch('D', 3, 1));
        for (label, body, expected) in [
            ("backup", backup, backup_expected),
            ("forward", forward, forward_expected),
            ("attributes", attributes, attributes_expected),
        ] {
            let xml = score(&[treble_opening(1), bar(2, &body)]);
            let (advanced, moved) = planned_advance(&xml);
            assert_eq!(moved, 1, "{label}");
            assert_eq!(advanced, score(&[treble_opening(1), bar(2, &expected)]), "{label}");
            assert_eq!(local_clef_oracle(&xml, &advanced), Ok(1), "{label}");
            let (_, realized) = trial_post(&xml, label);
            assert_eq!(trial_effect_oracle(&xml, &realized), Ok(1), "{label}");
            assert_eq!(adopt_through_render_state(&xml, label), realized, "{label}");
        }
    }

    #[test]
    fn row57_multibyte_text_inside_and_before_the_envelope_keeps_char_boundaries() {
        let words = |text: &str| format!("<direction><direction-type><words>{text}</words></direction-type></direction>");
        let body = format!(
            "{}{}{}{STAFF2_F}{}",
            words("Ausdrücklich – ruhig"), rest(1, 1, 2), words("ü ö ä ß ✓"), low_tail()
        );
        let expected = format!(
            "{}{STAFF2_F}{}{}{}",
            words("Ausdrücklich – ruhig"), rest(1, 1, 2), words("ü ö ä ß ✓"), low_tail()
        );
        let xml = score(&[treble_opening(1), bar(2, &body)]);
        let (advanced, moved) = planned_advance(&xml);
        assert_eq!(moved, 1);
        assert_eq!(advanced, score(&[treble_opening(1), bar(2, &expected)]));
        assert_eq!(local_clef_oracle(&xml, &advanced), Ok(1));
        let (_, realized) = trial_post(&xml, "multibyte");
        assert_eq!(trial_effect_oracle(&xml, &realized), Ok(1));
        assert_eq!(adopt_through_render_state(&xml, "multibyte"), realized);
    }

    #[test]
    fn row57_opening_clef_is_never_moved_or_crossed() {
        let body = format!("{}{}{STAFF2_F}{}", opening_attributes("G", 2), rest(1, 1, 2), low_tail());
        let xml = score(&[bar(1, &body)]);
        let (advanced, moved) = planned_advance(&xml);
        assert_eq!(moved, 1);
        let opening = opening_attributes("G", 2);
        let opening_ordinal = 1usize;
        assert!(advanced.starts_with(&format!("<score-partwise><part id=\"P1\"><measure number=\"{opening_ordinal}\">{opening}{STAFF2_F}")));
        assert_eq!(local_clef_oracle(&xml, &advanced), Ok(1));
        let opening_only = score(&[bar(1, &format!("{}{}", opening_attributes("F", 4), low_tail() + &rest(1, 1, 2)))]);
        assert_eq!(planned_advance(&opening_only), (opening_only.clone(), 0));
    }

    #[test]
    fn row57_oracle_rejects_parse_valid_sound_stable_scope_violations() {
        let body = format!("{}{STAFF2_G}{}{STAFF2_F}{}", rest(1, 1, 2), rest(1, 1, 2), [pitch('C', 3, 1), pitch('D', 3, 1)].concat());
        let pre = score(&[treble_opening(1), bar(2, &body)]);
        // Legacy-style first-event target: F moved across G. Parseable, same sounding events.
        let crossed = score(&[treble_opening(1), bar(2, &format!("{STAFF2_F}{}{STAFF2_G}{}{}", rest(1, 1, 2), rest(1, 1, 2), [pitch('C', 3, 1), pitch('D', 3, 1)].concat()))]);
        assert!(parse_musicxml(&crossed).is_ok());
        assert!(local_clef_oracle(&pre, &crossed).is_err(), "crossing a clef changes the carry and foreign scope");
        let backup = format!("{}<backup><duration>4</duration></backup>{}{STAFF2_F}{}", rest(1, 4, 2), rest(2, 1, 2), [pitch('C', 3, 2), pitch('D', 3, 2), pitch('E', 3, 2)].concat());
        let pre = score(&[treble_opening(1), bar(2, &backup)]);
        let crossed = score(&[treble_opening(1), bar(2, &format!("{STAFF2_F}{}<backup><duration>4</duration></backup>{}{}", rest(1, 4, 2), rest(2, 1, 2), [pitch('C', 3, 2), pitch('D', 3, 2), pitch('E', 3, 2)].concat()))]);
        assert!(local_clef_oracle(&pre, &crossed).unwrap_err().contains("cursor"));
        // The current planner never emits these: it stays inside the local scope.
        assert_eq!(local_clef_oracle(&pre, &planned_advance(&pre).0), Ok(1));
    }

    #[test]
    fn row57_writer_rejection_is_a_failed_owner_without_receipt_or_sink_write() {
        let xml = score(&[treble_opening(1), bar(2, &format!("{}{STAFF2_G}{}{STAFF2_F}{}", rest(1, 1, 2), rest(1, 1, 2), low_tail()))]);
        let local_ordinal = 2usize;
        let local_bar = xml.find(&format!("<measure number=\"{local_ordinal}\"")).expect("local bar");
        let target = local_bar + xml[local_bar..].find("<note").expect("first local event");
        let old_start = xml.rfind(STAFF2_F).expect("F block");
        let crossing = vec![crate::structural_write_authority::LowerStaffClefMoveRequest::new(old_start, old_start + STAFF2_F.len(), target, STAFF2_F.to_string())];
        let master = MasterAdoptedState::adopt_genesis_state(xml.clone(), MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(&xml)));
        let path = std::env::temp_dir().join(format!("etuschlopf-row57-reject-{}.render.musicxml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(false);
        let sink_called = std::cell::Cell::new(false);
        let value = state
            .adopt_path_bound_transition(
                root,
                RenderSinkKind::PrintCleanupOneSafeVisual,
                |source_xml| {
                    let mut candidate = source_xml.to_string();
                    let result = crate::structural_write_authority::apply_lower_staff_clef_moves_authorized(&mut candidate, crossing.clone());
                    assert_eq!(candidate, source_xml, "rejected V2 batch leaves original bytes untouched");
                    crate::structural_write_authority::RenderPathBoundOutcome {
                        value: result,
                        planned_post: None,
                        receipts: crate::master_state_identity::RenderTransitionReceipts::new(),
                        failed: true,
                    }
                },
                |_path, _pre, _candidate| {
                    sink_called.set(true);
                    std::io::Result::Ok(())
                },
            )
            .expect("failed owner is transported, not a sink error");
        assert!(value.unwrap_err().to_string().contains("crosses"));
        assert!(!sink_called.get(), "rejected candidate is never written");
        assert!(state.document().is_none(), "rejected candidate is never adopted");
        assert!(!path.exists());
    }

    #[test]
    fn row57_existing_single_scope_adoption_reaches_witnessed_artifact() {
        let body = format!("{}{STAFF2_F}{}", rest(1, 1, 2), low_tail());
        let xml = score(&[treble_opening(1), bar(2, &body)]);
        let expected = score(&[treble_opening(1), bar(2, &format!("{STAFF2_F}{}{}", rest(1, 1, 2), low_tail()))]);
        let adopted = adopt_through_render_state(&xml, "existing-single-scope");
        assert_eq!(adopted, expected);
        assert_eq!(local_clef_oracle(&xml, &adopted), Ok(1));
    }

    #[test]
    fn row57_reaches_live_pdf_commit_and_released_closure() {
        const CHILD_FLAG: &str = "ETUSCHLOPF_ROW57_RELEASE_CHILD";
        if std::env::var_os(CHILD_FLAG).is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "pipeline::row57_local_clef_scope_through_tests::row57_reaches_live_pdf_commit_and_released_closure", "--nocapture", "--test-threads=1"])
                .env(CHILD_FLAG, "1")
                .output().unwrap();
            assert!(output.status.success(), "isolated release/PDF owner chain must close: stdout={} stderr={}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
            return;
        }
        use crate::structural_write_authority as swa;
        use crate::master_state_identity as identity;
        identity::reset_render_lineage_measurement();
        let pre = score(&[treble_opening(1), bar(2, &format!("{}{STAFF2_F}{}", pitch('C', 3, 2), low_tail()))]).replace("<voice>1</voice>", "<voice>2</voice>");
        let master = MasterAdoptedState::adopt_genesis_state(pre.clone(), MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(&pre)));
        let path = std::env::temp_dir().join(format!("etuschlopf-row57-release-{}.render.musicxml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(false);
        state.adopt_path_bound_transition(root, RenderSinkKind::PrintCleanupOneSafeVisual,
            |source| {
                let (value, receipt) = build_render_xml_print_cleanup_one_safe_visual_trial(source, "synthetic-release", &path);
                let failed = value.is_err();
                let mut receipts = identity::RenderTransitionReceipts::new();
                let planned_post = receipt.map(|receipt| {
                    let (receipt, post) = receipt.split_realized_post_document();
                    let post = post.unwrap();
                    receipts.push_block(Some(receipt), &post);
                    post
                });
                swa::RenderPathBoundOutcome { value, planned_post, receipts, failed }
            },
            |artifact, _pre, post| swa::write_visual_cleanup_candidate_authorized(artifact, post))
            .unwrap().unwrap();
        let adopted = state.document().unwrap().to_string();
        assert_eq!(local_clef_oracle(&pre, &adopted), Ok(1));
        let checked = crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition("synthetic-row57-master", master.document()).expect("temporally valid row57 fixture");
        state.bind_late_master_epoch(identity::RenderMasterEpoch::after_cursor_lane_before_final_chord(master.lineage(), master.document(), &checked, master.document()));
        // Synthetic voice-free source evidence is independent of the clef planner.
        let mut evidence = Vec::new();
        let mut attacks = Vec::new();
        let mut labels = Vec::new();
        for (id, measure, raw_note_index, onset, pitch) in [
            (1usize, 1usize, 0usize, 0i64, 69i32), (2, 1, 1, 1, 71),
            (3, 1, 2, 2, 69), (4, 1, 3, 3, 71),
            (5, 2, 0, 0, 48), (6, 2, 1, 1, 48), (7, 2, 2, 2, 50), (8, 2, 3, 3, 52),
        ] {
            evidence.push(swa::VoiceFreeFinalChordEvidenceRow { fragment_id: id, source_event_id: id, attack_id: id, measure, raw_note_index, onset, duration: 1, pitch, staff: 2, chord_syntax: false, tie_start: false, tie_stop: false });
            let absolute_start = (measure as i64 - 1) * 4 + onset;
            attacks.push(swa::FrozenVoiceAttackEvidenceRow { source_event_id: id, attack_id: id, absolute_start, absolute_end: absolute_start + 1, staff: 2, technical_lane: 1 });
            labels.push(swa::LegacyVoiceLabelPreferenceRow { source_event_id: id, label: "2".to_string() });
        }
        let (canonical, _) = swa::plan_canonical_final_chord_structure_once(&adopted, &evidence).unwrap();
        let channels = swa::build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
        let voice = swa::bind_musicxml_voice_labels(&channels, &labels).unwrap();
        let omissions = std::collections::BTreeSet::new();
        let mut projection_receipts = identity::RenderTransitionReceipts::new();
        let projected = swa::project_canonical_final_chord_render_occurrence_continuation_with_receipts(&adopted, &canonical, &voice, &omissions, &mut projection_receipts).unwrap();
        // Match the existing read-only projection wrapper: receipts are explicitly
        // discarded; exact byte equality below forbids an unadopted projection change.
        projection_receipts.discard_unadopted(identity::RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(projected.xml(), adopted);
        let continuation = swa::advance_final_pdf_source_occurrence_after_cursor_commit(projected, &adopted, &adopted, false, true,
            swa::FinalPdfSourceOccurrenceValidationContext { canonical: &canonical, voice_plan: &voice, render_omissions: Some(&omissions) }).unwrap();
        let commit = render::final_pdf_source_commit::finalize_pdf_source_before_renderer_authorized(
            &mut state, &path, render::final_pdf_source_commit::FinalPdfSourceDisposition::Live,
            render::final_pdf_source_commit::FinalPdfSourcePlans { chord: &canonical, voice: &voice },
            &omissions, &continuation, None).unwrap();
        assert_eq!(commit.committed_xml(), state.document());
        let before = oracle_clef_state(&oracle_bars(&adopted).unwrap());
        let after = oracle_clef_state(&oracle_bars(state.document().unwrap()).unwrap());
        assert_eq!(before, after, "PDF commit must preserve the #57 clef state");
        // Stage markers prepare the gate; the actual PDF commit and final
        // artifact witness above/below remain the owners under test.
        for stage in [identity::RenderStageBoundary::InitialRender, identity::RenderStageBoundary::RestArticulation,
            identity::RenderStageBoundary::MetricBeam, identity::RenderStageBoundary::FinalNotation,
            identity::RenderStageBoundary::PdfSourceSelection, identity::RenderStageBoundary::Release] {
            identity::record_render_stage_boundary(stage);
        }
        identity::record_final_master_lineage(master.lineage(), master.document());
        assert!(state.close_render_sovereignty().closed);
        let _ = std::fs::remove_file(&path);
    }
