use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Tree(std::path::PathBuf);
impl Tree {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "rsp-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for file in TABLES.iter().map(|v| v.0).chain([
            PERMIT,
            MACRO_PATH,
            ADDITIVE_PATH,
            "src/master_state_identity.rs",
            "ZIP_KIND.txt",
        ]) {
            let target = path.join(file);
            fs::create_dir_all(target.parent().unwrap()).unwrap();
            fs::copy(root.join(file), target).unwrap();
        }
        let tree = Self(path);
        // Always construct the frozen Legacy parent, never the mutable live
        // child. These test inputs are identical in either lifecycle state.
        tree.set(
            "src/build_gate/destructive_transformation_population_v1.tsv",
            include_str!("ssc_population_prerequisites_r1250_v1.txt"),
        );
        tree.set(
            "src/build_gate/render_origins_v1.tsv",
            include_str!("ssc_cursor_temporal_origins_parent_r1444.txt"),
        );
        // These tests model the accepted historical RSP parent. Its tables
        // must not be copied from a later mutable live tree after writer retirement.
        tree.set("src/build_gate/destructive_transformation_exclusions_v1.tsv", include_str!("receipt_surface_test_parent_exclusions_v1.txt"));
        tree.set("src/build_gate/destructive_transformation_surface_baseline_v1.tsv", include_str!("receipt_surface_test_parent_surface_v1.txt"));
        tree.set("src/build_gate/render_legacy_selection_v1.tsv", include_str!("receipt_surface_test_parent_selection_v1.txt"));
        tree.set(MACRO_PATH, MACRO_LEGACY);
        tree.set(
            ADDITIVE_PATH,
            include_str!("receipt_surface_test_parent_additive_v1.txt"),
        );
        tree.set(
            "src/master_state_identity.rs",
            include_str!("receipt_surface_test_parent_identity_v1.txt"),
        );
        tree.set("ZIP_KIND.txt", "kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r1228\nparent_revision=r1227\nname=RenderReceiptFixtureClosurePermit\n");
        tree
    }
    fn set(&self, path: &str, text: &str) {
        fs::write(self.0.join(path), text).unwrap();
    }
    fn mutate(&self, path: &str, old: &str, new: &str) {
        let text = read(&self.0, path).unwrap();
        assert!(text.contains(old));
        self.set(path, &text.replacen(old, new, 1));
    }
    // An uncompiled governance fixture, not a runtime verifier implementation.
    fn consume(&self) {
        self.set(MACRO_PATH, MACRO_MIGRATED);
        let mut source = read(&self.0, ADDITIVE_PATH).unwrap();
        for (_, old, new, _, _) in ADDITIVE {
            assert!(source.contains(old));
            source = source.replacen(old, new, 1);
        }
        self.set(ADDITIVE_PATH, &source);
        let mut runtime = read(&self.0, "src/master_state_identity.rs").unwrap();
        runtime = runtime.replacen(
            "    Direction,\n}",
            "    Direction,\n    ArticulationFragment,\n    TenutoFragment,\n    TieFragment,\n}",
            1,
        );
        runtime = runtime.replacen("Self::Direction => \"direction\",", "Self::Direction => \"direction\", Self::ArticulationFragment | Self::TenutoFragment | Self::TieFragment => \"__owner_local_fragment_only\",", 1);

        for (_, _, _, token, name) in ADDITIVE {
            runtime += &format!(
                "\nimpl AdditiveTransformationReceipt {{ fn {name}(_owner: crate::structural_write_authority::{token}, pre_xml: &str, post_xml: &str, insertions: Vec<ExactEdit>) -> Option<Self> {{ synthetic_signature_only }} }}\n"
            );
        }
        self.set("src/master_state_identity.rs", &runtime);
        self.set("ZIP_KIND.txt","kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r1229\nparent_revision=r1228\nname=RenderReceiptSurfaceNormalization\n");
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn parent_allows_absent_future_helpers() {
    let t = Tree::new();
    assert_eq!(state(&t.0).unwrap(), State::Legacy);
}
#[test]
fn exact_atomic_immediate_fixture_is_admitted() {
    let p = Tree::new();
    let c = Tree::new();
    c.consume();
    assert_eq!(state(&c.0).unwrap(), State::Consumed);
    validate_transition(&p.0, &c.0, "RefactorOnly").unwrap();
}
#[test]
fn wrong_take_parent_revision_class_version_are_rejected() {
    let p = Tree::new();
    let c = Tree::new();
    c.consume();
    assert!(validate_transition(&p.0, &c.0, "KodexOnly").is_err());
    let valid = read(&c.0, "ZIP_KIND.txt").unwrap();
    for (old, new) in [
        (
            "name=RenderReceiptSurfaceNormalization\n",
            "name=RenderSovereigntyClosureMaxi\n",
        ),
        (
            "name=RenderReceiptSurfaceNormalization\n",
            "name=AnyOtherTake\n",
        ),
        ("parent_revision=r1228", "parent_revision=r1227"),
        ("revision=r1229", "revision=r1230"),
        ("version=0.10.1839", "version=0.10.1840"),
    ] {
        c.set("ZIP_KIND.txt", &valid.replace(old, new));
        assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
    }
}
#[test]
fn permit_installation_is_kodex_only_and_legacy() {
    let p = Tree::new();
    fs::remove_file(p.0.join(PERMIT)).unwrap();
    p.set(
        "ZIP_KIND.txt",
        "kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r1225\nname=ToolchainSignalHygiene\n",
    );
    let c = Tree::new();
    // Preserve the original r1225 -> r1226 installation contract test.
    c.set("ZIP_KIND.txt", "kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r1226\nparent_revision=r1225\nname=RenderReceiptSurfaceNormalizationPermit\n");
    validate_transition(&p.0, &c.0, "KodexOnly").unwrap();
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
    c.consume();
    assert!(validate_transition(&p.0, &c.0, "KodexOnly").is_err());
}
#[test]
fn no_partial_or_reversed_or_deleted_permit() {
    let p = Tree::new();
    let c = Tree::new();
    c.set(MACRO_PATH, MACRO_MIGRATED);
    assert!(state(&c.0).is_err());
    c.consume();
    assert!(validate_transition(&c.0, &p.0, "RefactorOnly").is_err());
    fs::remove_file(c.0.join(PERMIT)).unwrap();
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
}
#[test]
fn expired_parent_cannot_consume() {
    let p = Tree::new();
    p.mutate("ZIP_KIND.txt", "r1228", "r1229");
    let c = Tree::new();
    c.consume();
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
}
#[test]
fn all_frozen_tables_and_permit_are_fail_closed() {
    for (file, _) in TABLES {
        let t = Tree::new();
        let text = read(&t.0, file).unwrap();
        t.set(file, &(text + "\n"));
        assert!(state(&t.0).is_err());
    }
    let t = Tree::new();
    t.mutate(PERMIT, "all eleven", "any eleven");
    assert!(state(&t.0).is_err());
}
#[test]
fn geometry_and_legacy_effect_tampering_rejected() {
    for (old, new) in [
        ("*start, *end", "0, *end"),
        ("*start, *end", "*start, 0"),
        ("replacement.clone()", "String::new()"),
        (
            "VisibleLaneEffect::BackupAnchor",
            "VisibleLaneEffect::HiddenRestVisibility",
        ),
        (
            "ExactTransformationReceipt",
            "AdditiveTransformationReceipt",
        ),
    ] {
        let t = Tree::new();
        t.consume();
        let source = read(&t.0, MACRO_PATH).unwrap();
        let start = one(&source, MACROS[0]).unwrap().start;
        let changed = format!(
            "{}{}",
            &source[..start],
            source[start..].replacen(old, new, 1)
        );
        assert_ne!(changed, source);
        t.set(MACRO_PATH, &changed);
        assert!(state(&t.0).is_err());
    }
}
#[test]
fn duplicate_missing_macro_owner_and_comment_decoy_rejected() {
    let t = Tree::new();
    t.consume();
    let source = read(&t.0, MACRO_PATH).unwrap();
    let owner = one(MACRO_MIGRATED, MACROS[0]).unwrap();
    t.set(
        MACRO_PATH,
        &format!(
            "{source}\n fn {} {{ {} }}",
            owner.signature.trim_start_matches("fn "),
            owner.body
        ),
    );
    assert!(state(&t.0).is_err());
    let t = Tree::new();
    t.mutate(
        MACRO_PATH,
        "define_replacement_authority!(\n    apply_backup_anchor_rewrite_authorized",
        "// define_replacement_authority!(\n    apply_backup_anchor_rewrite_authorized",
    );
    assert!(state(&t.0).is_err());
}
#[test]
fn missing_or_unsealed_runtime_is_fatal() {
    let t = Tree::new();
    t.consume();
    t.mutate(
        "src/master_state_identity.rs",
        "fn mint_articulation_fragment_receipt",
        "fn removed_fragment_minter",
    );
    assert!(state(&t.0).is_err());
    let t = Tree::new();
    t.consume();
    t.mutate(
        "src/master_state_identity.rs",
        "crate::structural_write_authority::TieFragmentAdditiveMint",
        "AnyOwner",
    );
    assert!(state(&t.0).is_err());
}
#[test]
fn existing_additive_verifier_cannot_be_weakened() {
    let t = Tree::new();
    t.mutate(
        "src/master_state_identity.rs",
        "elements == 1",
        "elements >= 1",
    );
    assert!(state(&t.0).is_err());
}
#[test]
fn additive_geometry_body_and_write_receipt_requirement_frozen() {
    for (old, new) in [
        (
            "bound_tie_fragment_insertions(source, start, stop)",
            "bound_tie_fragment_insertions(source, stop, start)",
        ),
        ("if token.stop", "if token.start"),
        ("output == source || receipt.is_some()", "true"),
        ("        </notations>\\n", "        </notations> \\n"),
    ] {
        let t = Tree::new();
        t.consume();
        t.mutate(ADDITIVE_PATH, old, new);
        assert!(state(&t.0).is_err());
    }
}
#[test]
fn only_eight_same_path_macro_owners_receive_exception() {
    let t = Tree::new();
    t.consume();
    for name in MACROS {
        assert!(
            admits_macro_owner(&t.0, &format!("{MACRO_PATH}::{name}"), "macro:fn frozen").unwrap()
        );
    }
    assert!(
        !admits_macro_owner(
            &t.0,
            &format!("{MACRO_PATH}::apply_same_pitch_tie_merge_authorized"),
            "macro:fn frozen"
        )
        .unwrap()
    );
    assert!(
        !admits_macro_owner(&t.0, &format!("other.rs::{}", MACROS[0]), "macro:fn frozen").unwrap()
    );
}
#[test]
fn shared_guard_change_rejected_during_consumption() {
    let p = Tree::new();
    let c = Tree::new();
    c.consume();
    c.mutate(MACRO_PATH, "fn edit_allowed(", "fn modified_edit_allowed(");
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
}
#[test]
fn unrelated_surface_return_migration_is_not_overfrozen() {
    let t = Tree::new();
    t.consume();
    let source = read(&t.0, MACRO_PATH).unwrap();
    let function = one(&source, "insert_missing_staff_edge_rest_authorized").unwrap();
    let signature = function.signature;
    assert!(signature.contains("-> Option<String>"));
    let changed = signature.replace(
        "-> Option<String>",
        "-> (Option<String>, Option<ExactTransformationReceipt>)",
    );
    t.set(MACRO_PATH, &source.replacen(signature, &changed, 1));
    assert_eq!(state(&t.0).unwrap(), State::Consumed);
}
#[test]
fn literal_whitespace_is_semantic() {
    assert_ne!(canonical("f(\"a b\")"), canonical("f(\"ab\")"));
    assert_eq!(canonical("f ( x )"), canonical("f(x)"));
}

#[test]
fn generic_runtime_addition_is_rejected() {
    let p = Tree::new();
    let c = Tree::new();
    c.consume();
    let runtime = read(&c.0, "src/master_state_identity.rs").unwrap();
    c.set(
        "src/master_state_identity.rs",
        &(runtime + "\nimpl AdditiveTransformationReceipt { fn mint_any_owner_fragment() { } }\n"),
    );
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
}
#[test]
fn shared_macro_definition_comment_decoy_cannot_bypass_freeze() {
    let t = Tree::new();
    let source = read(&t.0, MACRO_PATH).unwrap();
    let decoy = "/* macro_rules! define_replacement_authority { fake } */\n";
    t.set(MACRO_PATH, &(decoy.to_string() + &source));
    assert_eq!(state(&t.0).unwrap(), State::Legacy);
    t.mutate(MACRO_PATH, "edits.len(),", "0,");
    assert!(state(&t.0).is_err());
}

#[test]
fn amended_permit_rejects_original_parent_and_qualified_fragment_returns() {
    let p = Tree::new();
    let c = Tree::new();
    c.consume();
    let valid_parent = read(&p.0, "ZIP_KIND.txt").unwrap();
    p.set("ZIP_KIND.txt", "kind=EtuSchlopfTake\nversion=0.10.1839\nrevision=r1226\nname=RenderReceiptSurfaceNormalizationPermit\n");
    assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
    p.set("ZIP_KIND.txt", &valid_parent);
    validate_transition(&p.0, &c.0, "RefactorOnly").unwrap();
    let valid_source = read(&c.0, ADDITIVE_PATH).unwrap();
    for (name, _, _, _, _) in ADDITIVE {
        let owner = one(&valid_source, name).unwrap();
        let changed = owner.signature.replace(
            "Option<AdditiveTransformationReceipt>",
            "Option<crate::master_state_identity::AdditiveTransformationReceipt>",
        );
        assert_ne!(changed, owner.signature);
        c.set(
            ADDITIVE_PATH,
            &valid_source.replacen(owner.signature, &changed, 1),
        );
        assert!(validate_transition(&p.0, &c.0, "RefactorOnly").is_err());
    }
}

#[test]
fn actual_live_tree_is_valid_in_either_accepted_lifecycle_state() {
    // Evaluates the actual source tree, independently of the fixed test parent.
    validate_current_tree(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let parent = Tree::new();
    assert_eq!(state(&parent.0).unwrap(), State::Legacy);
    let child = Tree::new();
    child.consume();
    assert_eq!(state(&child.0).unwrap(), State::Consumed);
    validate_transition(&parent.0, &child.0, "RefactorOnly").unwrap();
}

#[test]
fn additive_retirement_has_all_four_closed_combinations() {
    for articulation in [false, true] {
        for tenuto in [false, true] {
            let t=Tree::new(); t.consume();
            let mut runtime=read(&t.0,"src/master_state_identity.rs").unwrap();
            if !articulation {runtime=runtime.replace("    ArticulationFragment,\n", "").replace("Self::ArticulationFragment | ", "");}
            if !tenuto {runtime=runtime.replace("    TenutoFragment,\n", "").replace("Self::TenutoFragment | ", "");}
            validate_additive_type_surface(&runtime, State::Consumed, true).unwrap();
            let broken=runtime.replace("TieFragment", "UnexpectedFragment");
            assert!(validate_additive_type_surface(&broken, State::Consumed, true).is_err());
        }
    }
}
