//! Named governance revision: exact original Render note journal successor.
//! No runtime writer, ranking, receipt issuance or admission from mutable hashes.
const ITEM_STRUCT: &[u8] = include_bytes!("ssc_c3_render_block_struct_r1568.txt");
const ITEM_STRUCT_SHA: &str = "ca4d8c5e737d553d0feee7e9bdc622bf511dda431e4635cdbaf79e98c55624a2";
const ITEM_IMPL: &[u8] = include_bytes!("ssc_c3_render_block_impl_r1568.txt");
const ITEM_IMPL_SHA: &str = "4eddf0b3a4d22f5819175adba763763393f47653201008b116fe83b18d1a412e";
const CLEF: &[u8] = include_bytes!("ssc_c3_render_clef_journal_r1568.txt");
const CLEF_SHA: &str = "2a84896d1203875f6095ba758acc5b5d3cc586b5433c4af96c0e853684d768ac";
const CLEANUP: &[u8] = include_bytes!("ssc_c3_render_cleanup_journal_r1568.txt");
const CLEANUP_SHA: &str = "f18a62e8df8a8f40e9adb7562450283168458d229325248147e18843973f5701";
const THROUGH_TEST: &[u8] = include_bytes!("ssc_c3_render_cleanup_test_r1568.txt");
const THROUGH_TEST_SHA: &str = "927eace0ed6d1b34de135341af025d1e731f212626cb37eaa0c89770aaed5546";
fn exact(candidate: &[u8], admitted: &[u8], hash: &str) -> bool {
    candidate == admitted && crate::sha256::hex(admitted) == hash
}
pub(super) fn accepts_item(path: &str, kind: &str, name: &str, bytes: &[u8]) -> bool {
    match (path, kind, name) {
        ("src/master_state_identity.rs", "struct", "BlockTransformationReceipt") => exact(bytes, ITEM_STRUCT, ITEM_STRUCT_SHA),
        ("src/master_state_identity.rs", "impl", "BlockTransformationReceipt") => exact(bytes, ITEM_IMPL, ITEM_IMPL_SHA),
        _ => false,
    }
}
pub(super) fn accepts_file(path: &str, bytes: &[u8]) -> bool {
    match path {
        "src/pipeline_shards/notation_endpoint_layout_core.rs" => exact(bytes, CLEF, CLEF_SHA),
        "src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs" => exact(bytes, CLEANUP, CLEANUP_SHA),
        "tests/support/render_authority/row57_local_clef_scope_through_trial.rs" => exact(bytes, THROUGH_TEST, THROUGH_TEST_SHA),
        _ => false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_successors_bind_their_frozen_item_or_file_only() {
        assert!(accepts_item("src/master_state_identity.rs", "struct", "BlockTransformationReceipt", ITEM_STRUCT));
        assert!(accepts_item("src/master_state_identity.rs", "impl", "BlockTransformationReceipt", ITEM_IMPL));
        assert!(accepts_file("src/pipeline_shards/notation_endpoint_layout_core.rs", CLEF));
        assert!(accepts_file("src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs", CLEANUP));
        assert!(accepts_file("tests/support/render_authority/row57_local_clef_scope_through_trial.rs", THROUGH_TEST));
        assert!(!accepts_file("src/structural_write_authority/remaining_layout_writer_family.rs", CLEANUP));
        assert!(!accepts_item("src/master_state_identity.rs", "impl", "OtherReceipt", ITEM_IMPL));
    }
    #[test]
    fn partial_additional_and_changed_successor_bytes_stay_rejected() {
        for mut bad in [ITEM_STRUCT[..ITEM_STRUCT.len()-1].to_vec(), [ITEM_STRUCT, b"\n"].concat(), ITEM_STRUCT.to_vec()] {
            if bad.len() == ITEM_STRUCT.len() {bad[0] ^= 1;}
            assert!(!accepts_item("src/master_state_identity.rs", "struct", "BlockTransformationReceipt", &bad));
        }
        for mut bad in [ITEM_IMPL[..ITEM_IMPL.len()-1].to_vec(), [ITEM_IMPL, b"\n"].concat(), ITEM_IMPL.to_vec()] {
            if bad.len() == ITEM_IMPL.len() {bad[0] ^= 1;}
            assert!(!accepts_item("src/master_state_identity.rs", "impl", "BlockTransformationReceipt", &bad));
        }
        for mut bad in [CLEF[..CLEF.len()-1].to_vec(), [CLEF, b"\n"].concat(), CLEF.to_vec()] {
            if bad.len() == CLEF.len() {bad[0] ^= 1;}
            assert!(!accepts_file("src/pipeline_shards/notation_endpoint_layout_core.rs", &bad));
        }
        for mut bad in [CLEANUP[..CLEANUP.len()-1].to_vec(), [CLEANUP, b"\n"].concat(), CLEANUP.to_vec()] {
            if bad.len() == CLEANUP.len() {bad[0] ^= 1;}
            assert!(!accepts_file("src/pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs", &bad));
        }
        for mut bad in [THROUGH_TEST[..THROUGH_TEST.len()-1].to_vec(), [THROUGH_TEST, b"\n"].concat(), THROUGH_TEST.to_vec()] {
            if bad.len() == THROUGH_TEST.len() {bad[0] ^= 1;}
            assert!(!accepts_file("tests/support/render_authority/row57_local_clef_scope_through_trial.rs", &bad));
        }
    }
}
