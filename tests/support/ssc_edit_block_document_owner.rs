use super::*;
#[test]
fn ssc_block_document_real_rhythm_owner_receipt_adoption_and_report() {
    reset_master_lineage_measurement();
    let pre = edit_block_score(); let mut normalized = crate::arrangement::normalize_visible_rhythm_musicxml(&pre);
    let receipt = normalized.visible_rhythm_block_identity.take().expect("real changed document owner");
    assert_eq!(crate::master_state_identity::ssc_block_bound_span(&receipt), None);
    let mut receipts = OwnerReceipts::default(); receipts.push_block(Some(receipt));
    let state = adopt_master(&pre, &normalized.xml, receipts);
    assert_lines(&report(), &["ReleasedOwnerEvents: 1", "V4OutOfScope: 0", "V4Permitted: 1", "V4EnvelopeBLOCK: unproven=0; denied=0; permitted=1", "V4ContentChangingPermitted: 1", "ReleasedBalance: complete", "V4Balance: complete"]);
    assert_eq!(state.document(), normalized.xml);
}
