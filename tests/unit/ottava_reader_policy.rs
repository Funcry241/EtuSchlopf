use super::*;
#[test]
fn unowned_competing_scopes_retain_their_staff_and_do_not_become_invisible() {
    let _guard = super::super::sperre_fuer_tests();
    reset();
    let xml = synthetic::course(RegisternotationsDarstellung::Achtva, "1", "alpha", "free");
    let document = roxmltree::Document::parse(&xml).unwrap();
    let cue = document
        .descendants()
        .find(|node| node.has_tag_name("words"))
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    for (staff, expected) in [("1", 0), ("2", 1)] {
        let foreign = format!(
            "<direction placement=\"above\"><direction-type><octave-shift type=\"down\" size=\"8\" number=\"2\"/></direction-type><staff>{staff}</staff></direction>"
        );
        let mut changed = xml.clone();
        changed.insert_str(cue.range().start, &foreign);
        assert_eq!(pdf_requests(&changed).len(), expected);
    }
    for foreign in [
        "<direction><direction-type><octave-shift type=\"stop\" size=\"8\" number=\"1\"/></direction-type><staff>1</staff></direction>",
        "<direction><direction-type><octave-shift type=\"down\" size=\"15\" number=\"1\"/></direction-type><staff>1</staff></direction>",
        "<direction><direction-type><octave-shift type=\"down\" size=\"8\" number=\"2\"/></direction-type></direction>",
    ] {
        let mut changed = xml.clone();
        changed.insert_str(cue.range().start, foreign);
        assert!(pdf_requests(&changed).is_empty());
    }
}
#[test]
fn creator_ids_are_explicit_and_unknown_inputs_never_create_origins() {
    let _guard = super::super::sperre_fuer_tests();
    reset();
    let xml = synthetic::course(
        RegisternotationsDarstellung::Achtva,
        "1",
        "renamed",
        "anything",
    );
    assert_eq!(pdf_requests(&xml).len(), 1);
    let plain = xml.replace("ets-ottava-", "unregistered-");
    assert!(pdf_requests(&plain).is_empty());
    let duplicated = xml.replace(
        "</measure>",
        &format!(
            "{}</measure>",
            &xml[xml.find("<direction placement").unwrap()
                ..xml.find("</direction>").unwrap() + 12]
        ),
    );
    assert!(pdf_requests(&duplicated).is_empty());
    reset();
    assert!(pdf_requests(&xml).is_empty());
}
#[test]
fn stale_or_missing_probe_preserves_words_but_cleans_only_owned_metadata() {
    let _guard = super::super::sperre_fuer_tests();
    reset();
    let xml = synthetic::course(RegisternotationsDarstellung::Achtvb, "2", "alpha", "repeat");
    let request = pdf_requests(&xml).pop().unwrap();
    let dirs = synthetic::directives();
    let wrong = PdfContext {
        xml_sha256: "foreign".into(),
        pdf_sha256: "synthetic_pdf".into(),
        supported: BTreeMap::from([(request.id.clone(), synthetic::signature())]),
    };
    observe_probe(&xml, &xml, &dirs, &wrong);
    plan_final_document(&xml, &dirs).unwrap();
    let state = session().lock().unwrap();
    let plan = state.plans.get(&digest(&xml)).unwrap();
    assert!(plan.omissions.is_empty());
    assert_eq!(plan.removals.len(), 3);
    drop(state);
    let mut changed_dirs = dirs.clone();
    changed_dirs[0].explicit_width = Some("800".into());
    assert!(final_removals(&xml, &changed_dirs).is_empty());
    assert!(!omission_is_adopted(&xml, request.ordinal));
}
#[test]
fn display_domain_scope_and_user_word_changes_are_unknown() {
    let _guard = super::super::sperre_fuer_tests();
    for replacement in ["size=\"15\"", "number=\"2\""] {
        reset();
        let xml = synthetic::course(RegisternotationsDarstellung::Achtva, "1", "alpha", "x");
        let changed = if replacement.contains("size") {
            xml.replacen(
                "type=\"continue\" size=\"8\"",
                &format!("type=\"continue\" {replacement}"),
                1,
            )
        } else {
            xml.replacen(
                "type=\"continue\" size=\"8\" number=\"1\"",
                &format!("type=\"continue\" size=\"8\" {replacement}"),
                1,
            )
        };
        assert!(pdf_requests(&changed).is_empty());
        assert!(
            pdf_requests(&xml.replace(
                "<words font-style=\"italic\">8va</words>",
                "<words>user text</words>"
            ))
            .is_empty()
        );
    }
}
#[test]
fn independent_origins_do_not_turn_a_local_decision_into_a_global_winner() {
    let _guard = super::super::sperre_fuer_tests();
    reset();
    let first = synthetic::course(RegisternotationsDarstellung::Achtva, "1", "alpha", "x");
    let original = pdf_requests(&first).pop().unwrap();
    let original_record = session().lock().unwrap().origins.clone();
    reset();
    let second = synthetic::course(RegisternotationsDarstellung::Achtvb, "2", "beta", "y");
    session().lock().unwrap().origins.extend(original_record);
    let a = roxmltree::Document::parse(&first).unwrap();
    let b = roxmltree::Document::parse(&second).unwrap();
    let part_a = a.descendants().find(|n| n.has_tag_name("part")).unwrap();
    let part_b = b.descendants().find(|n| n.has_tag_name("part")).unwrap();
    let both = format!(
        "<score-partwise>{}{}</score-partwise>",
        &first[part_a.range()],
        &second[part_b.range()]
    );
    let requests = pdf_requests(&both);
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].id, original.id);
    let ctx = PdfContext {
        xml_sha256: digest(&both),
        pdf_sha256: "synthetic independent physical observations".into(),
        supported: requests
            .iter()
            .map(|r| (r.id.clone(), synthetic::signature()))
            .collect(),
    };
    let mut dirs = synthetic::directives();
    dirs.extend(synthetic::directives());
    observe_probe(&both, &both, &dirs, &ctx);
    plan_final_document(&both, &dirs).unwrap();
    let state = session().lock().unwrap();
    let plan = state.plans.get(&digest(&both)).unwrap();
    assert_eq!(plan.omissions.len(), 2);
    assert!(plan.omissions.values().any(|(r, _)| r.id == original.id));
}
