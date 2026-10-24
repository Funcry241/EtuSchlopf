use super::*;

#[test]
fn registered_wav_caller_preserves_lifetime_tokens_and_unrelated_error_exits() {
    let caller = closed_node("wav_salience_caller").unwrap();
    let inputs = closed_node("wav_salience_reference_destructure").unwrap();
    let body = node_text(&caller).unwrap();
    let before_inputs = node_text(&inputs).unwrap();
    let mut after_inputs = inputs.clone();
    after_inputs.hex = inputs.after_hex.unwrap();
    let after_inputs = node_text(&after_inputs).unwrap();
    let prefix = "fn run_master_notation_and_salience_stage(inputs: Inputs<'_>) {";
    let parent_text = format!("{prefix}{before_inputs}{body}unrelated()?; }}");
    let child_text = format!("{prefix}{after_inputs}unrelated()?; }}");
    let root = std::env::temp_dir().join(format!("wav-caller-contract-{}", std::process::id()));
    let parent = root.join("parent");
    let child = root.join("child");
    for (tree, text) in [(&parent, &parent_text), (&child, &child_text)] {
        let path = tree.join(caller.path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    let name = "run_master_notation_and_salience_stage";
    assert!(registered_caller_loss(&parent, &child, caller.path, name).unwrap());
    std::fs::write(child.join(caller.path), child_text.replace("unrelated()?", "unrelated()")).unwrap();
    assert!(!registered_caller_loss(&parent, &child, caller.path, name).unwrap());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn wav_registered_nodes_preserve_survivors_and_reject_forged_contexts() {
    for node in closed_nodes().unwrap().into_iter().filter(|n| n.key.starts_with("wav_") && n.kind != "proof") {
        let body = node_text(&node).unwrap();
        let after = node.after_hex.map(|hex| {
            let mut n = node.clone(); n.hex = hex; node_text(&n).unwrap()
        }).unwrap_or_default();
        let (parent, child) = if let Some(name) = node.container.strip_prefix("fn:") {
            (format!("fn {name}() {{ {body} alive()?; }}"), format!("fn {name}() {{ {after} alive()?; }}"))
        } else if let Some(name) = node.container.strip_prefix("enum:") {
            (format!("enum {name} {{ {body} Survivor, }}"), format!("enum {name} {{ {after} Survivor, }}"))
        } else {
            (format!("{body}\nfn alive() {{ unrelated()?; }}"), format!("{after}\nfn alive() {{ unrelated()?; }}"))
        };
        let class = if node.kind == "replacement" { "ClosedReplacement" } else if node.key.ends_with("_caller") { "BoundNeutralClosure" } else { "ClosedItem" };
        let regions = vec![format!("{class}::{}", node.key)];
        let names = [node.symbol.to_owned()].into_iter().collect();
        verify_in_path(&parent, &child, &regions, &names, Some(node.path)).unwrap();
        assert!(verify_in_path(&parent, &child.replace("Survivor", "Forged").replace("alive", "forged"), &regions, &names, Some(node.path)).is_err(), "{}", node.key);
        assert!(verify_in_path(&parent, &child, &regions, &names, Some("src/forged.rs")).is_err());
        assert!(verify_in_path(&parent, &child, &regions, &BTreeSet::new(), Some(node.path)).is_err());
        assert!(verify_in_path(&parent, &parent, &regions, &names, Some(node.path)).is_err());
    }
}
fn retired() -> BTreeSet<String> {
    ["retired".to_owned()].into_iter().collect()
}
#[test]
fn full_exclusive_guard_erasure_preserves_survivors() {
    let p = "fn run(){if ready { retired(); } alive();}";
    verify(
        p,
        "fn run(){alive();}",
        &["Statement::run::0".into()],
        &retired(),
    )
    .unwrap();
}
#[test]
fn guard_body_cannot_escape_or_reorder() {
    let p = "fn run(){if ready { retired(); alive(); } last();}";
    assert!(
        verify(
            p,
            "fn run(){alive();last();}",
            &["Statement::run::0".into()],
            &retired()
        )
        .is_err()
    );
    assert!(
        verify(
            "fn run(){retired();a();b();}",
            "fn run(){b();a();}",
            &["Statement::run::0".into()],
            &retired()
        )
        .is_err()
    );
}
#[test]
fn every_removed_error_exit_rejects() {
    for tail in [
        "?",
        ";ensure!(good)",
        ";return",
        ";panic!()",
        ".unwrap()",
        ";assert!(good)",
    ] {
        let p = format!("fn run(){{if ready {{retired(){tail};}} writer();}}");
        assert!(
            verify(
                &p,
                "fn run(){writer();}",
                &["Statement::run::0".into()],
                &retired()
            )
            .is_err(),
            "{tail}"
        );
    }
}
#[test]
fn literal_bytes_are_significant_and_comments_are_not() {
    assert_ne!(
        tokens("let x=\"old\";").unwrap(),
        tokens("let x=\"new\";").unwrap()
    );
    assert_eq!(
        tokens("let/* nested /* x */ */x=1;").unwrap(),
        tokens("let x = 1;").unwrap()
    );
    assert_eq!(
        tokens("r##\"{literal}\"##").unwrap(),
        vec!["r##\"{literal}\"##"]
    );
}
#[test]
fn whole_function_and_visibility_can_be_removed() {
    verify(
        "#[cfg(test)] pub(crate) fn retired(){panic!();} fn live(){}",
        "fn live(){}",
        &["WholeFunction::retired".into()],
        &retired(),
    )
    .unwrap();
}
#[test]
fn replacement_or_ambiguous_nodes_are_not_erasure() {
    assert!(
        verify(
            "fn retired(){} fn live(){}",
            "fn replacement(){} fn live(){}",
            &["WholeFunction::retired".into()],
            &retired()
        )
        .is_err()
    );
    assert!(
        verify(
            "fn live(){retired();}",
            "fn live(){}",
            &["Bytes::0::10".into()],
            &retired()
        )
        .is_err()
    );
}

#[test]
fn macros_generic_calls_indirect_calls_and_state_updates_cannot_hide_in_erasure() {
    for tail in [
        "write!(out, \"x\");",
        "commit::<Render>();",
        "state = 3;",
        "state += 1;",
        "unreachable!();",
        "todo!();",
        "(callback)();",
        "callbacks[0]();",
        "unsafe { touch(); }",
    ] {
        let parent = format!("fn run(){{if ready {{retired();{tail}}}alive();}}");
        assert!(
            verify(
                &parent,
                "fn run(){alive();}",
                &["Statement::run::0".into()],
                &retired()
            )
            .is_err(),
            "{tail}"
        );
    }
}
#[test]
fn closed_nodes_require_exact_key_path_symbol_and_parent_bytes() {
    for node in closed_nodes()
        .unwrap()
        .into_iter()
        .filter(|node| node.kind == "node")
    {
        let body = node_text(&node).unwrap();
        let parent = if let Some(name) = node.container.strip_prefix("fn:") {
            format!("fn {name}() {{{body}}}")
        } else if let Some(name) = node.container.strip_prefix("enum:") {
            format!("enum {name} {{{body}}}")
        } else {
            body.clone()
        };
        let child = parent.replacen(&body, "", 1);
        let class = if node.key.ends_with("_caller") {
            "BoundNeutralClosure"
        } else {
            "ClosedItem"
        };
        let regions = vec![format!("{class}::{}", node.key)];
        let symbols = [node.symbol.to_owned()].into_iter().collect();
        verify_in_path(&parent, &child, &regions, &symbols, Some(node.path)).unwrap();
        assert!(verify_in_path(&parent, &child, &regions, &symbols, Some("src/wrong.rs")).is_err());
        assert!(
            verify_in_path(&parent, &child, &regions, &BTreeSet::new(), Some(node.path)).is_err()
        );
        assert!(
            verify_in_path(
                &format!("{parent} {parent}"),
                &child,
                &regions,
                &symbols,
                Some(node.path)
            )
            .is_err()
        );
        assert!(verify(&parent, &child, &regions, &symbols).is_err());
    }
}
#[test]
fn neutral_contexts_are_mandatory_and_cannot_be_claimed_for_another_obligation() {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let root = std::env::temp_dir().join(format!(
        "ssc-neutral-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&root).unwrap();
    for group in ["e031_tenuto", "ra5_accent_reserve"] {
        let mut files = std::collections::BTreeMap::<String, String>::new();
        for node in closed_nodes()
            .unwrap()
            .into_iter()
            .filter(|node| node.kind == "proof" && node.obligation == group)
        {
            files
                .entry(node.path.into())
                .or_default()
                .push_str(&format!("{}\n", node_text(&node).unwrap()));
        }
        for (path, text) in &files {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        let key = if group == "e031_tenuto" {
            "e031_caller"
        } else {
            "ra5_caller"
        };
        let regions = vec![format!("BoundNeutralClosure::{key}")];
        let ids = [group.into()].into_iter().collect();
        validate_closed_contexts(&root, &regions, &ids).unwrap();
        assert!(validate_closed_contexts(&root, &regions, &BTreeSet::new()).is_err());
        for proof in closed_nodes()
            .unwrap()
            .into_iter()
            .filter(|node| node.kind == "proof" && node.obligation == group)
        {
            let original = &files[proof.path];
            let changed = original.replace(&node_text(&proof).unwrap(), "fn forged() {}\n");
            assert_ne!(&changed, original);
            std::fs::write(root.join(proof.path), changed).unwrap();
            assert!(
                validate_closed_contexts(&root, &regions, &ids).is_err(),
                "{}",
                proof.key
            );
            std::fs::write(root.join(proof.path), original).unwrap();
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn completion_replacement_is_exact_and_cannot_change_surviving_test_literals() {
    let node = closed_node("e031_geometry_program_literal").unwrap();
    let body = node_text(&node).unwrap();
    let mut after = node.clone();
    after.hex = node.after_hex.unwrap();
    let replacement = node_text(&after).unwrap();
    let parent =
        format!("fn raw_splice_helper_and_real_receipt_match_legacy_bytes() {{ {body} alive(); }}");
    let child = parent.replace(&body, &replacement);
    let regions = vec!["ClosedReplacement::e031_geometry_program_literal".into()];
    let retired = [node.symbol.to_owned()].into_iter().collect();
    verify_in_path(&parent, &child, &regions, &retired, Some(node.path)).unwrap();
    for forged in [
        child.replace("alive", "replacement_writer"),
        child.replace("bound_tie_fragment_insertions", "forged"),
        parent.clone(),
    ] {
        assert!(verify_in_path(&parent, &forged, &regions, &retired, Some(node.path)).is_err());
    }
    assert!(
        verify_in_path(
            &parent,
            &child,
            &["ClosedItem::e031_geometry_program_literal".into()],
            &retired,
            Some(node.path)
        )
        .is_err()
    );
}
#[test]
fn registered_caller_loss_requires_all_probe_contexts_and_exact_survivor() {
    let root = std::env::temp_dir().join(format!("ssc-caller-completion-{}", std::process::id()));
    let parent = root.join("parent");
    let child = root.join("child");
    let mut files = std::collections::BTreeMap::<String, String>::new();
    for node in closed_nodes()
        .unwrap()
        .into_iter()
        .filter(|n| n.kind == "proof" && n.obligation == "e031_tenuto")
    {
        files
            .entry(node.path.into())
            .or_default()
            .push_str(&format!("{}\n", node_text(&node).unwrap()));
    }
    for (path, text) in &files {
        for tree in [&parent, &child] {
            let p = tree.join(path);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, text).unwrap();
        }
    }
    let node = closed_node("e031_caller").unwrap();
    let original = &files[node.path];
    let p = tokens(original).unwrap();
    let (a, b) = closed_range(&p, &node).unwrap();
    let changed = p
        .iter()
        .enumerate()
        .filter(|(i, _)| *i < a || *i >= b)
        .map(|(_, s)| s.clone())
        .collect::<Vec<_>>()
        .join(" ");
    std::fs::write(child.join(node.path), &changed).unwrap();
    assert!(
        registered_caller_loss(
            &parent,
            &child,
            node.path,
            "apply_structural_beam_articulation_closure_render_write#1"
        )
        .unwrap()
    );
    let forged = changed.replace("candidate_xml . clone ( )", "candidate_xml . clear ( )");
    assert_ne!(changed, forged);
    std::fs::write(child.join(node.path), forged).unwrap();
    assert!(
        !registered_caller_loss(
            &parent,
            &child,
            node.path,
            "apply_structural_beam_articulation_closure_render_write#1"
        )
        .unwrap()
    );
    std::fs::write(child.join(node.path), changed).unwrap();
    let probe = closed_nodes()
        .unwrap()
        .into_iter()
        .find(|n| n.key == "e031_probe_context")
        .unwrap();
    std::fs::write(
        parent.join(probe.path),
        original.replace(
            &node_text(&probe).unwrap(),
            "fn add_tenuto_to_note_fragment() {}",
        ),
    )
    .unwrap();
    assert!(
        registered_caller_loss(
            &parent,
            &child,
            node.path,
            "apply_structural_beam_articulation_closure_render_write#1"
        )
        .is_err()
    );
    std::fs::remove_dir_all(root).unwrap();
}
