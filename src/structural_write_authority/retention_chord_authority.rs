//! Typed authority for adding a chord marker to one already selected note
//! fragment. The caller retains all retention and pitch selection logic.

use crate::master_state_identity::BlockTransformationReceipt;
use super::authority_primitives::{fingerprint, parsed_exact_note as exact_note};

#[derive(Debug)]
struct ChordMarkerCapability {
    source: String,
    source_fingerprint: u64,
}

impl ChordMarkerCapability {
    fn issue(source: &str) -> Option<Self> {
        if !exact_note(source) {
            return None;
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
        })
    }

    fn apply(self, source: &str) -> Option<String> {
        if self.source != source || self.source_fingerprint != fingerprint(source) {
            return None;
        }
        let mut out = self.source;
        if !out.contains("<chord") {
            let open_end = out.find('>')?;
            out.insert_str(open_end + 1, "\n        <chord/>");
        }
        Some(out)
    }
}



pub fn ensure_bound_chord_marker_authorized(source: &str) -> (Option<String>, Option<BlockTransformationReceipt>) {
    
    
    let output = (|| {
    ChordMarkerCapability::issue(source)?.apply(source)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow7BlockMint { _private: () }, source, post, Some((0, source.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

pub(crate) struct RenderRow7BlockMint { _private: () }
