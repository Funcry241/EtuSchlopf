use super::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

fn legacy() -> String {
    format!("fn {LEGACY_FUNCTION}() {{ {CONDITION} {{ {LEGACY_RAW_SPLICE} }} }}")
}

fn relocated(receipt: bool) -> String {
    let signature = if receipt {
        TARGET_SIGNATURE.replace(
            "-> String",
            "-> (String, Option<ExactTransformationReceipt>)",
        )
    } else {
        TARGET_SIGNATURE.to_string()
    };
    let tail = if receipt {
        "let receipt = sealed_mint::exact(final_candidate, edit_start, edit_end, replacement, &candidate); (candidate, receipt)"
    } else {
        "candidate"
    };
    let call = if receipt {
        RECEIPT_SPLICE_CALL
    } else {
        SPLICE_CALL
    };
    format!(
        "{signature} {{ {RAW_SPLICE} {tail} }} fn {LEGACY_FUNCTION}() {{ {CONDITION} {{ {call} }} }}"
    )
}

#[test]
fn legacy_missing_target_is_allowed_but_only_with_the_bound_single_splice() {
    assert_eq!(
        validate_row67_source(&legacy()).unwrap(),
        Row67State::Legacy
    );
    assert!(validate_row67_source(&legacy().replace(LEGACY_RAW_SPLICE, "")).is_err());
    assert!(
        validate_row67_source(&legacy().replace(LEGACY_RAW_SPLICE, &format!("{LEGACY_RAW_SPLICE}{LEGACY_RAW_SPLICE}")))
            .is_err()
    );
    assert!(validate_row67_source(&legacy().replace(CONDITION, "if arbitrary_selector")).is_err());
    assert!(validate_row67_source(&legacy().replace("..edit_start", "..edit_end")).is_err());
    assert!(
        validate_row67_source(&format!("// {}\nfn {LEGACY_FUNCTION}() {{ }}", legacy())).is_err()
    );
}

#[test]
fn atomic_relocation_accepts_only_the_same_splice_and_one_guarded_target() {
    for receipt in [false, true] {
        assert_eq!(
            validate_row67_source(&relocated(receipt)).unwrap(),
            Row67State::Relocated
        );
    }
    let valid = relocated(false);
    assert!(
        validate_row67_source(&format!(
            "{valid}\n{TARGET_SIGNATURE} {{ {RAW_SPLICE} candidate }}"
        ))
        .is_err()
    );
    assert!(validate_row67_source(&valid.replace(SPLICE_CALL, RAW_SPLICE)).is_err());
    assert!(
        validate_row67_source(&valid.replace(SPLICE_CALL, &format!("{SPLICE_CALL}{SPLICE_CALL}")))
            .is_err()
    );
    assert!(
        validate_row67_source(&valid.replace(
            SPLICE_CALL,
            &format!("{SPLICE_CALL}{}", RAW_SPLICE.replace("candidate", "second"))
        ))
        .is_err()
    );
    assert!(validate_row67_source(&valid.replace("+ replacement.len()", "+ 10")).is_err());
    assert!(
        validate_row67_source(&valid.replace("candidate }", "candidate.to_uppercase() }")).is_err()
    );
    assert!(
        validate_row67_source(&valid.replace("replacement: &str", "replacement: String")).is_err()
    );
    assert!(
        validate_row67_source(
            &relocated(true).replace("Option<ExactTransformationReceipt>", "OwnerReceipts")
        )
        .is_err()
    );
    assert!(
        validate_row67_source(
            &relocated(true).replace("replacement, &candidate", "replacement, &mut candidate")
        )
        .is_err()
    );
    assert!(
        validate_row67_source(&relocated(true).replace("sealed_mint::exact", "write_and_mint()"))
            .is_err()
    );
}

#[test]
fn comments_and_strings_cannot_forge_the_target_or_legacy_witness() {
    let source = format!(
        "{}\n// {}\nconst HINT: &str = r###\"{}\"###;",
        legacy(),
        relocated(false),
        relocated(false)
    );
    assert_eq!(validate_row67_source(&source).unwrap(), Row67State::Legacy);
}

#[test]
fn relocation_contract_rejects_every_unrelated_owner_and_widening() {
    let text = include_str!("render_owner_relocation_v1.tsv");
    assert!(relocation(text).is_ok());
    for (before, after) in [
        ("67\t", "57\t"),
        ("EXACT", "BLOCK_DOCUMENT"),
        ("\tR\t", "\tB\t"),
        ("DEFERRED_RENDER_PREREQUISITE", "READY"),
        ("\t1\t1\t0\t", "\t1\t2\t0\t"),
        (TARGET_FUNCTION, "unrelated_owner"),
        ("r1220", "r1219"),
        ("RenderSovereigntyClosureMaxi", "AnyRefactor"),
    ] {
        assert!(
            relocation(&text.replace(before, after)).is_err(),
            "{before}"
        );
    }
    assert!(relocation(&format!("{text}{}\n", text.lines().nth(1).unwrap())).is_err());
}

#[test]
fn cursor_and_direction_events_are_separate_and_direction_basis_is_not_generic() {
    let text = include_str!("render_event_boundaries_v1.tsv");
    assert!(validate_boundaries(text).is_ok());
    for source in [
        "current file",
        "arbitrary previous state",
        "latest RenderAdoptedState",
    ] {
        assert!(
            validate_boundaries(&text.replace("last #58 committed render state", source)).is_err()
        );
    }
    assert!(
        validate_boundaries(&text.replace("CursorNeutral\t2", "DirectionRelocation\t2")).is_err()
    );
    assert!(
        validate_boundaries(&text.replace(
            "compose_authorized_direction_delta_onto_sink",
            "arbitrary_writer"
        ))
        .is_err()
    );
    assert!(validate_boundaries(&text.replace("Option<String>", "OwnerReceipts")).is_err());
}

fn request(row: &[&'static str; 9]) -> OriginRequest<'static> {
    OriginRequest {
        id: row[0],
        kind: row[1],
        fork_point: row[2],
        owner: row[3],
        reason: row[4],
        source: row[5],
        source_epoch: row[6],
        legacy_selection: row[7],
        has_lineage: row[8] == "present_preserved",
        lineage_preserved: true,
    }
}

#[test]
fn only_five_first_geneses_and_two_history_preserving_substitutions_are_allowed() {
    assert!(validate_origins(include_str!("ssc_cursor_temporal_origins_parent_r1444.txt")).is_ok());
    for row in &ORIGINS {
        let positive = request(row);
        assert!(validate_origin_request(&positive).is_ok());
        let mut negative = positive.clone();
        negative.owner = "src/other.rs::any_function";
        assert!(validate_origin_request(&negative).is_err());
        let mut negative = positive.clone();
        negative.reason = "AnyMasterFallback";
        assert!(validate_origin_request(&negative).is_err());
        let mut negative = positive.clone();
        negative.source = "preview_xml";
        assert!(validate_origin_request(&negative).is_err());
        let mut negative = positive.clone();
        negative.source_epoch = "latest Master";
        assert!(validate_origin_request(&negative).is_err());
        let mut negative = positive.clone();
        negative.fork_point = "arbitrary_fork";
        assert!(validate_origin_request(&negative).is_err());
        let mut negative = positive.clone();
        negative.legacy_selection = "read failed";
        assert!(validate_origin_request(&negative).is_err());
        let mut negative = positive.clone();
        negative.has_lineage = !positive.has_lineage;
        assert!(validate_origin_request(&negative).is_err());
        if positive.has_lineage {
            let mut negative = positive.clone();
            negative.lineage_preserved = false;
            assert!(validate_origin_request(&negative).is_err());
        }
    }
    let from_master = ORIGINS.iter().find(|row| row[0] == "S4")
        .expect("S4 remains the history-preserving FromMaster origin");
    let mut unlisted = request(from_master);
    unlisted.id = "S5";
    assert!(validate_origin_request(&unlisted).is_err());
    assert!(
        validate_origins(
            &include_str!("ssc_cursor_temporal_origins_parent_r1444.txt").replace("S4\tFromMaster", "S5\tFromMaster")
        )
        .is_err()
    );
}

#[test]
fn trial_flags_and_audit_or_class_f_reads_cannot_erase_or_replace_history() {
    let text = include_str!("receipt_surface_test_parent_selection_v1.txt");
    assert!(validate_legacy_selection(text).is_ok());
    assert!(
        validate_legacy_selection(
            &text.replace("retain actual Render history", "forget unwritten flags")
        )
        .is_err()
    );
    assert!(
        validate_legacy_selection(&text.replace("fatal sovereignty error", "Master fallback"))
            .is_err()
    );
    assert!(validate_legacy_selection(&text.replace("\tforbidden", "\tFromMaster")).is_err());
}

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!(
            "kr-governance-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }
    fn put(&self, path: &str, text: &str) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    fn populate(&self, relocated_owner: bool) {
        for (path, text) in [
            (
                RELOCATION_PATH,
                include_str!("render_owner_relocation_v1.tsv"),
            ),
            (
                BOUNDARIES_PATH,
                include_str!("render_event_boundaries_v1.tsv"),
            ),
            (ORIGINS_PATH, include_str!("ssc_cursor_temporal_origins_parent_r1444.txt")),
            (
                SELECTION_PATH,
                include_str!("receipt_surface_test_parent_selection_v1.txt"),
            ),
            (
                POPULATION_PATH,
                include_str!("ssc_population_prerequisites_r1250_v1.txt"),
            ),
            (
                BUDGET_PATH,
                include_str!("receipt_surface_test_parent_surface_v1.txt"),
            ),
        ] {
            self.put(path, text);
        }
        self.put(
            "src/structural_write_authority/writer_entry_boundary.rs",
            &format!("{CURSOR_SIGNATURE} {{ }}"),
        );
        self.put(
            "src/structural_write_authority/renderer_direction_relocation_writer_boundary.rs",
            &format!("{DIRECTION_SIGNATURE} {{ }}"),
        );
        let population = include_str!("ssc_population_prerequisites_r1250_v1.txt");
        let signature = population
            .lines()
            .find(|line| line.starts_with("67\t"))
            .unwrap()
            .split('\t')
            .nth(2)
            .unwrap();
        self.put(
            LEGACY_PATH,
            &if relocated_owner {
                relocated(false)
            } else {
                legacy().replace(&format!("fn {LEGACY_FUNCTION}()"), signature)
            },
        );
        self.put("src/pipeline_shards/pipeline_orchestrator_initial_render_stage.rs", "fn run_initial_render_transaction_stage() { build_render_xml_pruned_pdf_trial(best.layout_xml(),); RenderXmlCheckpointState::new(render_xml_snapshot_reader,best.layout_xml()); build_render_xml_hidden_rest_forward_collapse_trial(initial_render_checkpoint.current_xml(),); build_render_xml_pianistic_rest_repair_trial(initial_render_checkpoint.current_xml(),); build_render_xml_print_cleanup_one_safe_visual_trial(initial_render_checkpoint.current_xml(),); build_render_xml_pianistic_missing_rest_insert_trial(initial_render_checkpoint.current_xml(),); build_behind_bars_rule_check_rest_and_tie_audit(initial_render_checkpoint.current_xml(),); build_cross_staff_hand_role_materialization_trial(initial_render_checkpoint.current_xml(),); build_lower_staff_visible_rhythm_rest_lane_trial(initial_render_checkpoint.current_xml(),); }");
        self.put("src/pipeline_shards/pipeline_orchestrator_final_notation_support.rs", "fn run_final_render_notation_authority_stage() { let lower_clef_pair_prewrite_xml = render_xml_snapshot_reader.select_after_write(final_render_source_available || render_xml_trial_path.exists(), final_export_xml,); let beam_source_xml = render_xml_snapshot_reader.select_after_write(final_render_source_available, final_export_xml,); let structural_authority_source_xml = render_xml_snapshot_reader.select_after_write(final_render_source_available, final_export_xml,); }");
        self.put("src/structural_write_authority/mod.rs", "fn execute_authorized_cursor_neutral_writer_candidate() { let productive_note_beam_sink_xml = orchestrator_artifact_sink_authority::current_thread_render_checkpoint_xml().unwrap_or_default(); }");
        self.put("ZIP_KIND.txt", &format!("kind=EtuSchlopfTake\nversion=0.10.1839\nrevision={}\nparent_revision={}\nname={}\n", if relocated_owner { "synthetic_child" } else { PERMIT_PARENT_REVISION }, if relocated_owner { PERMIT_PARENT_REVISION } else { "r1220" }, if relocated_owner { REFACTOR_NAME } else { PERMIT_PARENT_NAME }));
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn relocation_epoch_is_closed_for_every_legacy_parent() {
    let before = Fixture::new();
    before.populate(false);
    assert_eq!(validate_current_tree(&before.0).unwrap(), Row67State::Legacy);
    let after = Fixture::new();
    after.populate(true);
    for class in ["RefactorOnly", "KodexOnly", "MusicMutation", "AuditOnly"] {
        assert!(
            validate_transition(&before.0, &after.0, class)
                .unwrap_err()
                .contains("epoch base")
        );
    }
    let relocated = Fixture::new();
    relocated.populate(true);
    validate_transition(&after.0, &relocated.0, "RefactorOnly").unwrap();
    assert!(validate_transition(&after.0, &before.0, "RefactorOnly").is_err());
}

#[test]
fn missing_target_after_migration_and_population_drift_are_fatal() {
    let fixture = Fixture::new();
    fixture.populate(true);
    fixture.put(LEGACY_PATH, &legacy());
    assert!(
        validate_current_tree(&fixture.0)
            .unwrap_err()
            .contains("missing")
    );
    fixture.populate(true);
    let population = read(&fixture.0, POPULATION_PATH).unwrap();
    for altered in [
        population.replace(
            "\tEXACT\tR\t(edit_start",
            "\tBLOCK_DOCUMENT\tR\t(edit_start",
        ),
        population.replace("\tR\t(edit_start", "\tB\t(edit_start"),
        population.replace("DEFERRED_RENDER_PREREQUISITE", "READY"),
    ] {
        fixture.put(POPULATION_PATH, &altered);
        assert!(validate_current_tree(&fixture.0).is_err());
    }
}

#[test]
fn row69_and_cursor_registration_cannot_drift_or_gain_multi_receipts() {
    let population = include_str!("ssc_population_prerequisites_r1250_v1.txt");
    validate_population_contract(population, false).unwrap();
    for (before, after) in [
        ("69\t", "70\t"),
        ("apply_bound_missing_rest_batch_authorized", "another_batch"),
        ("(forward_start, forward_end, rest_note)", "arbitrary_diff"),
        (CURSOR_OWNER, DIRECTION_OWNER),
    ] {
        assert!(validate_population_contract(&population.replace(before, after), false).is_err());
    }
    let fixture = Fixture::new();
    fixture.populate(true);
    fixture.put(
        "src/structural_write_authority/writer_entry_boundary.rs",
        &format!(
            "{} {{ }}",
            CURSOR_SIGNATURE.replace(
                "-> String",
                "-> (String, Option<ExactTransformationReceipt>)"
            )
        ),
    );
    assert!(validate_current_tree(&fixture.0).is_ok());
    fixture.put(
        "src/structural_write_authority/writer_entry_boundary.rs",
        &format!(
            "{} {{ }}",
            CURSOR_SIGNATURE.replace("-> String", "-> (String, OwnerReceipts)")
        ),
    );
    assert!(validate_current_tree(&fixture.0).is_err());
}

#[test]
fn boundary_normalization_permit_changes_only_the_effective_parent_and_consumer() {
    let base = include_str!("render_owner_relocation_v1.tsv");
    let original = base.lines().nth(1).unwrap().split('\t').collect::<Vec<_>>();
    let effective = relocation(base).unwrap();
    assert_eq!(&effective[..11], &original[..11]);
    assert_eq!(
        &effective[11..],
        &[PERMIT_PARENT_REVISION, PERMIT_PARENT_NAME, REFACTOR_NAME]
    );
}

#[test]
fn immutable_wrapper_bindings_are_exact_and_do_not_widen_relocation() {
    for receipt in [false, true] {
        let source = relocated(receipt);
        assert_eq!(
            validate_row67_source(&source).unwrap(),
            Row67State::Relocated
        );
        let frozen_call = if receipt {
            RECEIPT_SPLICE_CALL
        } else {
            SPLICE_CALL
        };
        let mutable_call = if receipt {
            frozen_call.replace("let (candidate,", "let (mut candidate,")
        } else {
            frozen_call.replace("let candidate =", "let mut candidate =")
        };
        assert!(validate_row67_source(&source.replace(frozen_call, &mutable_call)).is_err());
        assert!(
            validate_row67_source(&source.replace(
                frozen_call,
                &frozen_call.replace("&replacement)", "&other_replacement)")
            ))
            .is_err()
        );
        assert!(
            validate_row67_source(&source.replace(
                frozen_call,
                &frozen_call.replace("edit_start, edit_end", "edit_end, edit_start")
            ))
            .is_err()
        );
    }
}

#[test]
fn immutable_raw_splice_callers_compile_with_unused_mut_denied() {
    let fixture = Fixture::new();
    let plain = format!(
        "mod plain {{ {TARGET_SIGNATURE} {{ {RAW_SPLICE} candidate }} fn run(final_candidate: &str, same_lane_strict_interior_edit: Option<(usize, usize, String)>) -> String {{ {CONDITION} {{ {SPLICE_CALL} candidate }} else {{ final_candidate.to_string() }} }} pub fn check() {{ assert_eq!(run(\"abcd\", Some((1, 3, \"X\".to_string()))), \"aXd\"); assert_eq!(run(\"abcd\", None), \"abcd\"); }} }}"
    );
    let receipt_signature = TARGET_SIGNATURE.replace(
        "-> String",
        "-> (String, Option<ExactTransformationReceipt>)",
    );
    let receipt = format!(
        "mod receipt {{ #[derive(Debug)] struct ExactTransformationReceipt; mod sealed_mint {{ pub fn exact(pre: &str, start: usize, end: usize, replacement: &str, post: &str) -> Option<super::ExactTransformationReceipt> {{ assert_eq!(format!(\"{{}}{{}}{{}}\", &pre[..start], replacement, &pre[end..]), post); (pre != post).then_some(super::ExactTransformationReceipt) }} }} {receipt_signature} {{ {RAW_SPLICE} let receipt = sealed_mint::exact(final_candidate, edit_start, edit_end, replacement, &candidate); (candidate, receipt) }} fn run(final_candidate: &str, same_lane_strict_interior_edit: Option<(usize, usize, String)>) -> (String, Option<ExactTransformationReceipt>) {{ {CONDITION} {{ {RECEIPT_SPLICE_CALL} (candidate, same_lane_splice_receipt) }} else {{ (final_candidate.to_string(), None) }} }} pub fn check() {{ let (post, event) = run(\"abcd\", Some((1, 3, \"X\".to_string()))); assert_eq!(post, \"aXd\"); assert!(event.is_some()); assert!(run(\"abcd\", None).1.is_none()); }} }}"
    );
    let source = format!(
        "#![deny(warnings)]\n{plain}\n{receipt}\nfn main() {{ plain::check(); receipt::check(); }}\n"
    );
    fixture.put("caller_probe.rs", &source);
    let executable = fixture.0.join(if cfg!(windows) {
        "caller_probe.exe"
    } else {
        "caller_probe"
    });
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let result = std::process::Command::new(&rustc)
        .arg("--edition=2024")
        .arg(fixture.0.join("caller_probe.rs"))
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(
        std::process::Command::new(&executable)
            .status()
            .unwrap()
            .success()
    );
    fixture.put(
        "caller_probe.rs",
        &source.replace("let candidate = same_lane", "let mut candidate = same_lane"),
    );
    let rejected = std::process::Command::new(&rustc)
        .arg("--edition=2024")
        .arg(fixture.0.join("caller_probe.rs"))
        .arg("-o")
        .arg(&executable)
        .output()
        .unwrap();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("unused_mut"));
}

#[test]
fn canonical_helper_insertion_replaces_exactly_one_old_source_form() {
    for receipt in [false, true] {
        let valid = relocated(receipt);
        assert_eq!(validate_row67_source(&valid).unwrap(), Row67State::Relocated);
        for insertion in [
            "candidate.push_str(&replacement);",
            "candidate.push_str(&&replacement);",
            "candidate.push_str(&replacement.to_string());",
            "candidate.push_str(&replacement.to_owned());",
            "let copied = replacement.to_string(); candidate.push_str(&copied);",
            "candidate.push_str(other_replacement);",
        ] {
            assert!(validate_row67_source(&valid.replace("candidate.push_str(replacement);", insertion)).is_err(), "{insertion}");
        }
        assert!(validate_row67_source(&valid.replace(TARGET_FUNCTION, "other_splice_owner")).is_err());
        assert!(validate_row67_source(&valid.replace("edit_start: usize, edit_end: usize", "edit_end: usize, edit_start: usize")).is_err());
        assert!(validate_row67_source(&format!("{valid} fn unrelated_owner() {{ {LEGACY_RAW_SPLICE} }}")).is_err());
        assert!(validate_row67_source(&format!("{valid} fn unrelated_owner() {{ {RAW_SPLICE} }}")).is_err());
    }
    assert_eq!(validate_row67_source(&legacy()).unwrap(), Row67State::Legacy);
    assert!(validate_row67_source(&legacy().replace("push_str(&replacement)", "push_str(replacement)")).is_err());
}

#[test]
fn ready_population_requires_k2_and_preserves_all_other_pins() {
    let population=include_str!("ssc_population_prerequisites_r1250_v1.txt");
    let old=population.lines().find(|line|line.starts_with("67\t")).unwrap();
    let next=population.replace(old,&old.replace("DEFERRED_RENDER_PREREQUISITE","READY"));
    validate_population_contract(&next, true).unwrap();
    assert!(validate_population_contract(&next, false).is_err());
    assert!(validate_population_contract(&next.replace("\tEXACT\tR\t(edit_start","\tBLOCK_DOCUMENT\tR\t(edit_start"),true).is_err());
}

#[test]
fn ready_population_is_admitted_only_with_k2_without_changing_identity() {
    let original = include_str!("ssc_population_prerequisites_r1250_v1.txt");
    let population = original.lines().map(|line| {
        let mut row = line.split('\t').collect::<Vec<_>>();
        if row[0] == "67" { row[9] = "READY"; }
        row.join("\t")
    }).collect::<Vec<_>>().join("\n") + "\n";
    validate_population_contract(&population, true).unwrap();
    assert!(validate_population_contract(&population, false).is_err());
}
