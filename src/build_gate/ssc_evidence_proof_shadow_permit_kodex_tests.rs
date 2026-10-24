use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

struct Tree(std::path::PathBuf);
impl Tree {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "ssc-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let tree = Self(root);
        tree.put(CONTRACT_PATH, CONTRACT);
        tree.put(BASELINE_PATH, BASELINE);
        tree.put(REMAINING_PATH, INITIAL_REMAINING);
        tree.put("src/main.rs", "fn main() {}\n");
        tree.put("ZIP_KIND.txt", "revision=r1239\nparent_revision=r1238\nname=SscEvidenceProofShadowPermitContractKodex\n");
        tree
    }
    fn put(&self, path: &str, text: &str) {
        let file = self.0.join(path);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, text).unwrap();
    }
}
impl Drop for Tree {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn inert_contract_tree_is_admitted() {
    assert!(validate_current_tree(&Tree::new().0).is_ok());
}
#[test]
fn contract_install_epoch_is_closed() {
    let child = Tree::new();
    let parent = Tree::new();
    fs::remove_file(parent.0.join(CONTRACT_PATH)).unwrap();
    parent.put("ZIP_KIND.txt", "revision=r1238\n");
    for class in ["KodexOnly", "RefactorOnly"] {
        assert!(
            validate_transition(&parent.0, &child.0, class)
                .unwrap_err()
                .contains("epoch base")
        );
    }
}
#[test]
fn installed_contract_cannot_disappear_or_change() {
    let parent = Tree::new();
    let child = Tree::new();
    child.put(
        CONTRACT_PATH,
        &CONTRACT.replace("observation_only", "writer_authority"),
    );
    assert!(validate_transition(&parent.0, &child.0, "KodexOnly").is_err());
    fs::remove_file(child.0.join(CONTRACT_PATH)).unwrap();
    assert!(validate_transition(&parent.0, &child.0, "KodexOnly").is_err());
}
#[test]
fn baseline_is_complete_and_not_editable() {
    assert_eq!(
        baseline_ids(BASELINE).unwrap(),
        remaining(INITIAL_REMAINING).unwrap()
    );
    assert_eq!(remaining(INITIAL_REMAINING).unwrap().len(), 15);
    let tree = Tree::new();
    tree.put(BASELINE_PATH, &BASELINE.replace("OPEN", "RETIRED"));
    assert!(validate_current_tree(&tree.0).is_err());
}
#[test]
fn no_premature_empty_set_or_unwitnessed_retirement() {
    let tree = Tree::new();
    tree.put(
        REMAINING_PATH,
        "schema=ssc_exit_remaining_v1\nexit_claim=not_closed\n",
    );
    assert!(validate_current_tree(&tree.0).is_err());
    tree.put(
        REMAINING_PATH,
        &INITIAL_REMAINING.replace("open=e031_tenuto\n", ""),
    );
    assert!(validate_current_tree(&tree.0).is_err());
}
#[test]
fn remaining_set_rejects_duplicates_unknown_fields_and_exit_claim() {
    for suffix in ["open=e031_tenuto\n", "open=\n", "retired=e031_tenuto\n"] {
        assert!(remaining(&format!("{INITIAL_REMAINING}{suffix}")).is_err());
    }
    assert!(remaining(&INITIAL_REMAINING.replace("not_closed", "closed")).is_err());
    let tree = Tree::new();
    tree.put(
        REMAINING_PATH,
        &format!("{INITIAL_REMAINING}open=new_authority\n"),
    );
    assert!(validate_current_tree(&tree.0).is_err());
}
#[test]
fn monotonicity_rejects_new_and_reopened_obligations() {
    let original = remaining(INITIAL_REMAINING).unwrap();
    let mut reduced = original.clone();
    reduced.remove("e031_tenuto");
    assert!(monotone(&original, &reduced).is_ok());
    assert!(monotone(&reduced, &original).is_err());
}
#[test]
fn sinks_return_unit_and_cannot_export_decisions_callbacks_or_authority() {
    assert!(
        validate_sink(
            SINKS[0],
            "pub(crate) fn append_ssc_shadow_observation(out: &mut String) {}"
        )
        .is_ok()
    );
    for source in [
        "pub(crate) fn append_ssc_shadow_observation() -> u8 { 0 }",
        "pub(crate) fn append_ssc_shadow_observation(out: &mut String, authority: &mut Authority) {}",
        "pub(crate) fn append_ssc_shadow_observation<F: Fn()>(f: F) {}",
        "pub(crate) use crate::ssc_shadow_permit::ShadowPermitVerdict;",
        "pub(crate) fn append_ssc_shadow_observation() { structural_write_authority::write(); }",
    ] {
        assert!(validate_sink(SINKS[0], source).is_err());
    }
}
#[test]
fn partial_proof_or_shadow_implementation_is_rejected() {
    let tree = Tree::new();
    tree.put(PROOF_PATH, "// proof core without shadow\n");
    assert!(validate_current_tree(&tree.0).unwrap_err().contains("partial"));
}

#[test]
fn compiled_current_remaining_is_monotone_against_immutable_initial_baseline() {
    let initial = remaining(INITIAL_REMAINING).unwrap();
    assert_eq!(initial, baseline_ids(BASELINE).unwrap());
    assert_eq!(initial.len(), 15);
    let current = remaining(include_str!("ssc_exit_remaining_v1.txt")).unwrap();
    monotone(&initial, &current).unwrap();
    assert_eq!(initial.difference(&current).count() + current.len(), 15);
}
