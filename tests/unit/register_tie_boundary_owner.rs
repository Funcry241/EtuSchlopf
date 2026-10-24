use super::*;

#[test]
fn actual_register_owner_preserves_incoming_tie_and_original_receipt_requirement() {
    const CHILD: &str = "ETUS_TEST_REGISTER_TIE_BOUNDARY_OWNER";
    if std::env::var_os(CHILD).is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "pipeline::register_tie_boundary_owner::actual_register_owner_preserves_incoming_tie_and_original_receipt_requirement", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert!(output.status.success(), "{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)); return;
    }
    let notes = (0..4).map(|_| "<note><pitch><step>C</step><octave>7</octave></pitch><duration>24</duration><voice>701</voice><type>quarter</type><staff>1</staff></note>").collect::<String>();
    let plain = format!("<score-partwise><part id=\"Synthetic\"><measure number=\"Renamed\"><attributes><divisions>24</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{notes}</measure></part></score-partwise>");
    crate::registernotation::beobachte_vor_k00(&plain);
    let mut positive = plain.clone(); let actual = apply_octave_notation_policy(&mut positive);
    assert!(positive.contains("<octave-shift")); assert_ne!(actual.block_receipts.len(), 0);
    assert!(actual.block_receipts.coverage_diagnostic(&plain, &positive).is_none());
    assert!(master_state_identity::OwnerReceipts::default().coverage_diagnostic(&plain, &positive).is_some());
    let protected = plain.replacen("<voice>", "<tie type=\"stop\"/><notations><tied type=\"stop\"/></notations><voice>", 1);
    crate::registernotation::beobachte_vor_k00(&protected);
    let mut guarded = protected.clone(); let refused = apply_octave_notation_policy(&mut guarded);
    assert_eq!(guarded, protected); assert_eq!(refused.block_receipts.len(), 0);
}
