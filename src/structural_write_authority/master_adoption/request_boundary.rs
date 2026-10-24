use anyhow::ensure;

use super::MasterAdoptedState;
use crate::master_state_identity::OwnerReceipts;

/// Immutable Master replacement request: the bytes a candidate's owners
/// produced from `source`, together with the receipts those owners minted, in
/// application order. There is no receipt-less form (r1219): a changed request
/// without covering receipts is a fatal sovereignty violation at adoption.
#[derive(Debug)]
pub(crate) struct MasterXmlReplacementRequest {
    source: String,
    replacement: String,
    owner_receipts: OwnerReceipts,
}

impl MasterXmlReplacementRequest {
    /// Binds a replacement together with the receipts its owners minted while
    /// producing it from `source`. Receipts are provenance, never authority.
    pub(crate) fn bind_with_owner_receipts(
        source: &str,
        replacement: String,
        owner_receipts: OwnerReceipts,
    ) -> Self {
        Self {
            source: source.to_string(),
            replacement,
            owner_receipts,
        }
    }
}

pub(crate) fn apply_block_master_xml_request(
    state: &mut MasterAdoptedState,
    request: MasterXmlReplacementRequest,
) -> anyhow::Result<()> {
    ensure!(
        state.document() == request.source,
        "MasterMutationRequestBoundary: source drift before StructuralWriteAuthority materialization"
    );
    state.adopt_materialized_document(
        request.replacement,
        request.owner_receipts,
        "MasterXmlReplacementRequest",
    );
    Ok(())
}
