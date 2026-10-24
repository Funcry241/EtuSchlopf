use super::MasterAdoptedState;
use crate::master_state_identity::tests::{
    committed_identity_matches_for_test, synthetic_genesis_for_tests,
};
use crate::master_state_identity::{
    MasterLineage, OwnerReceipts, reset_master_lineage_measurement,
};
fn adopted(pre: &str) -> MasterAdoptedState {
    MasterAdoptedState::adopt_genesis_state(
        pre.to_owned(),
        MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(pre)),
    )
}
fn ssc_completed_report() -> String {
    let report = "TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION\n----------------------------------------\nStatus: present.\nSILENT-VOICE-COVERAGE-WRITE-OUTCOME\n--------------------------------\nStatus: present.\nRendererOutcome: observed_from_existing_writer_audit.\n";
    crate::final_report::DraftReport::new(report.to_owned())
        .compact(str::to_owned)
        .finalize()
        .expect("final report")
        .as_str()
        .to_owned()
}
fn ssc_score(note: &str) -> String {
    format!(
        "<score-partwise><part id=\"P\"><measure><attributes><divisions>1</divisions></attributes>{note}</measure></part></score-partwise>"
    )
}
#[test]
fn ssc_real_f5_owner_receipt_master_adoption_and_final_report_are_one_chain() {
    reset_master_lineage_measurement();
    let pre = ssc_score(
        "<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice></note>",
    );
    let end = pre.find("</note>").unwrap() + "</note>".len();
    let (post, receipt) =
        crate::structural_write_authority::apply_bound_staff_voice_move_authorized(
            &pre,
            None,
            None,
            end,
            "1".to_owned(),
            "2".to_owned(),
        );
    let receipt = receipt.expect("real original #4 receipt");
    let identity = *receipt.ssc_bound_origin().0;
    assert_eq!(
        receipt
            .ssc_bound_origin()
            .1
            .captured_pre_for_test()
            .as_deref(),
        Some(pre.as_str())
    );
    let mut state = adopted(&pre);
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(receipt));
    let post = post.unwrap();
    state.adopt_materialized_document(post.clone(), receipts, "test bound F5 owner");
    assert_eq!(state.document(), post);
    assert!(committed_identity_matches_for_test(
        state.lineage(),
        identity
    ));
    crate::master_state_identity::record_final_master_lineage(state.lineage(), state.document());
    let report = ssc_completed_report();
    for line in [
        "ReleasedOwnerEvents: 1",
        "ClassifiedOwnerEvents: 1",
        "Permit: 1",
        "InvariantDenied: 0",
        "OutOfScope: 0",
        "FamilyF5Permit: 1",
        "MissingEvidence: 0",
        "V4Permitted: 1",
        "V4EnvelopeLANE: unproven=0; denied=0; permitted=1",
    ] {
        assert!(report.contains(line), "{line}: {report}");
    }
    assert_eq!(
        state.document(),
        post,
        "report cannot mutate adopted document"
    );
}
#[test]
fn ssc_real_f7_hidden_rest_owner_adoption_preserves_xml_and_reports_positive() {
    reset_master_lineage_measurement();
    let old = "<note print-object=\"no\"><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>";
    let new = old.replace(" print-object=\"no\"", "");
    let pre = ssc_score(old);
    let start = pre.find(old).unwrap();
    let (post, receipt) =
        crate::structural_write_authority::apply_hidden_rest_visibility_rewrite_authorized(
            &pre,
            vec![(start, start + old.len(), new)],
        );
    let post = post.expect("real #44 output");
    let receipt = receipt.expect("real #44 receipt");
    let mut state = adopted(&pre);
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(receipt));
    state.adopt_materialized_document(post.clone(), receipts, "test bound F7 owner");
    crate::master_state_identity::record_final_master_lineage(state.lineage(), state.document());
    let report = ssc_completed_report();
    assert!(report.contains("Permit: 1"), "{report}");
    assert!(report.contains("FamilyF7Permit: 1"));
    assert!(report.contains("V4Permitted: 1"), "{report}");
    assert!(report.contains("V4EnvelopeLANE: unproven=0; denied=0; permitted=1"), "{report}");
    assert!(report.contains("InvariantDenied: 0"));
    assert_eq!(state.document(), post);
}
#[test]
fn ssc_real_additive_owner_preserves_frozen_verifier_and_bound_final_defect() {
    reset_master_lineage_measurement();
    let pre =
        ssc_score("<note><rest/><duration>4</duration><voice>1</voice><staff>1</staff></note>");
    let mut post = pre.clone();
    let (_, receipt) =
        crate::structural_write_authority::ensure_final_barline_authorized(&mut post)
            .expect("real E004 insertion");
    let receipt = receipt.expect("original E004 receipt");
    let identity = *crate::master_state_identity::ssc_additive_identity(&receipt);
    let mut state = adopted(&pre);
    let mut receipts = OwnerReceipts::default();
    receipts.push_additive(Some(receipt));
    state.adopt_materialized_document(post.clone(), receipts, "test bound additive owner");
    assert!(committed_identity_matches_for_test(
        state.lineage(),
        identity
    ));
    crate::master_state_identity::record_final_master_lineage(state.lineage(), state.document());
    let report = ssc_completed_report();
    for line in [
        "ReleasedOwnerEvents: 1",
        "ClassifiedOwnerEvents: 1",
        "Permit: 1",
        "InvariantDenied: 0",
        "OutOfScope: 0",
        "FamilyADDITIVEPermit: 1",
        "MissingEvidence: 0",
        "V4Permitted: 1",
        "V4EnvelopeINSERTION: unproven=0; denied=0; permitted=1",
    ] {
        assert!(report.contains(line), "{line}: {report}");
    }
    assert_eq!(state.document(), post);
}
#[test]
fn ssc_dropped_snapshot_cannot_change_current_canonical_documents() {
    reset_master_lineage_measurement();
    let pre = ssc_score("<note/>");
    let mut post = pre.clone();
    let (_, receipt) =
        crate::structural_write_authority::ensure_final_barline_authorized(&mut post).unwrap();
    drop(receipt);
    assert_eq!(
        ssc_completed_report().matches("SSC-SHADOW-PERMIT").count(),
        0
    );
    assert_ne!(pre, post);
}
