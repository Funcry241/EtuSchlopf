//! Opaque transaction-local identity transport for caller-supplied replacement batches.
//!
//! r1188 does not infer musical meaning from ranges, XML geometry, function names or
//! report labels. A batch identity is minted exactly when an already-selected batch
//! crosses a typed structural-write-authority entry point. Every input edit receives an
//! ordinal inside that opaque batch. Surviving bound edits carry that identity through
//! validation and successful in-memory application; identity never grants write authority.
//! r1192 additionally gives AuditOnly the exact complete-document before/after states at
//! successful application so the same identity can acquire a receipt lineage without
//! reconstructing an occurrence from physical XML coordinates.

use std::panic::Location;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_BATCH_NONCE: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct IndividualEditIdentity {
    batch_nonce: u64,
    ordinal: usize,
}

#[derive(Debug, Clone)]
pub(super) struct DestructiveEditBatchAttribution {
    batch_nonce: u64,
    origin_file: &'static str,
    origin_line: u32,
    origin_column: u32,
    edits: Vec<IndividualEditIdentity>,
}

impl DestructiveEditBatchAttribution {
    #[track_caller]
    pub(super) fn begin_destructive_edit_batch_attribution(edit_count: usize) -> Self {
        let origin = Location::caller();
        let batch_nonce = NEXT_BATCH_NONCE.fetch_add(1, Ordering::Relaxed);
        let edits = (0..edit_count)
            .map(|ordinal| IndividualEditIdentity {
                batch_nonce,
                ordinal,
            })
            .collect::<Vec<_>>();
        crate::audit_only::destructive_edit_attribution::observe_origin(
            batch_nonce,
            origin.file(),
            origin.line(),
            origin.column(),
            edits.len(),
        );
        Self {
            batch_nonce,
            origin_file: origin.file(),
            origin_line: origin.line(),
            origin_column: origin.column(),
            edits,
        }
    }

    pub(super) fn identity(&self, ordinal: usize) -> Option<IndividualEditIdentity> {
        self.edits.get(ordinal).copied()
    }

    fn owns(&self, identity: IndividualEditIdentity) -> bool {
        self.edits.get(identity.ordinal).copied() == Some(identity)
            && identity.batch_nonce == self.batch_nonce
    }

    pub(super) fn mark_bound(&self, identity: IndividualEditIdentity) {
        debug_assert!(self.owns(identity));
        crate::audit_only::destructive_edit_attribution::observe_bound(
            self.batch_nonce,
            identity.ordinal,
            self.origin_file,
            self.origin_line,
            self.origin_column,
            self.edits.len(),
        );
    }

    pub(super) fn mark_applied_batch(
        &self,
        identities: &[IndividualEditIdentity],
        source_before: &str,
        source_after: &str,
    ) {
        debug_assert!(identities.iter().all(|identity| self.owns(*identity)));
        let ordinals = identities
            .iter()
            .map(|identity| identity.ordinal)
            .collect::<Vec<_>>();
        crate::audit_only::destructive_edit_attribution::observe_applied_batch_transition(
            self.batch_nonce,
            &ordinals,
            self.origin_file,
            self.origin_line,
            self.origin_column,
            self.edits.len(),
            (source_before, source_after),
        );
    }
}
