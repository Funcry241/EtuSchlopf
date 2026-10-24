//! Immutable original course evidence for the single #59 -> #60 invocation.
//! No identity, spelling decision, receipt or writer capability is minted here.
use super::*;
use crate::arrangement::OwnedRhythmicContinuation;
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct OriginalFunctionalRespellingChains {
    received: Result<Arc<OwnedRhythmicContinuation>, String>,
    epoch: OriginalChainEpoch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OriginalChainEpoch { Received, FinalChord, MasterExport }

impl std::fmt::Debug for OriginalFunctionalRespellingChains {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("OriginalFunctionalRespellingChains")
            .field("original_binding", &self.received.is_ok()).field("epoch", &self.epoch).finish()
    }
}

impl OriginalFunctionalRespellingChains {
    pub(crate) fn received(received: Result<Arc<OwnedRhythmicContinuation>, String>) -> Self {
        Self { received, epoch: OriginalChainEpoch::Received }
    }

    pub(crate) fn after_original_final_chord(
        &self,
        _: &crate::structural_write_authority::FinalChordEncodingExactMint,
        before: &str,
        after: &str,
        original_sources: &[(Range<usize>, usize)],
        receipt: Option<&ExactTransformationReceipt>,
    ) -> Self {
        let received = (|| -> anyhow::Result<_> {
            let state = self.received.as_ref().map_err(|error| anyhow::anyhow!("{error}"))?;
            ensure!(self.epoch == OriginalChainEpoch::Received, "Original chain input has already crossed FinalChord");
            state.validate_binding(before)?;
            state.validate_original_respelling_sources(original_sources)?;
            let next = match receipt {
                Some(receipt) => {
                    ensure!(receipt.minted_by::<crate::structural_write_authority::FinalChordEncodingExactMint>(),
                        "Original chain transport lacks the original FinalChord owner");
                    ensure!(receipt.final_chord_capture().is_some(), "Original FinalChord emission capture is absent");
                    state.continue_exact(receipt, None)?
                },
                None => {
                    ensure!(before == after, "Changed FinalChord cannot be an original no-op");
                    state.as_ref().clone()
                },
            };
            next.validate_binding(after)?;
            Ok(Arc::new(next))
        })().map_err(|error| error.to_string());
        Self { received, epoch: OriginalChainEpoch::FinalChord }
    }

    pub(crate) fn after_original_master_export(
        &self, before: &str, after: &str, receipt: Option<&ExactTransformationReceipt>,
    ) -> Self {
        let received = (|| -> anyhow::Result<_> {
            let state = self.received.as_ref().map_err(|error| anyhow::anyhow!("{error}"))?;
            ensure!(self.epoch == OriginalChainEpoch::FinalChord, "Original chain input lacks its FinalChord epoch");
            state.validate_binding(before)?;
            let next = match receipt {
                Some(receipt) => {
                    ensure!(receipt.minted_by::<crate::structural_write_authority::MasterExportFifthsExactMint>(),
                        "Original chain transport lacks the original Master-export owner");
                    state.continue_exact(receipt, None)?
                },
                None => {
                    ensure!(before == after, "Changed Master export cannot be an original no-op");
                    state.as_ref().clone()
                },
            };
            next.validate_binding(after)?;
            Ok(Arc::new(next))
        })().map_err(|error| error.to_string());
        Self { received, epoch: OriginalChainEpoch::MasterExport }
    }

    /// The #60 owner's own key operations transport pre-existing ranges. This
    /// verifies their replay and unchanged complete notes; it infers no origins.
    pub(crate) fn components_through_original_key_operations(
        &self, before: &str, after: &str, edits: &[ExactEdit], requested: &[(usize, usize)],
    ) -> anyhow::Result<Vec<Vec<Range<usize>>>> {
        let state = self.received.as_ref().map_err(|error| anyhow::anyhow!("{error}"))?;
        ensure!(self.epoch == OriginalChainEpoch::MasterExport, "Original chain input lacks its Master-export epoch");
        state.validate_binding(before)?;
        if edits.is_empty() {
            ensure!(before == after, "Unrecorded key operation before original chain repair");
            return state.original_respelling_components(|range| Ok(requested.contains(&(range.start, range.end))));
        }
        ensure!(exact_edits_reproduce(before, after, edits), "Original key operations do not reproduce their post");
        let readback = ExactOperationReadback {
            pre_document: before, post_document: after.to_owned(), edits,
        };
        let components = state.original_respelling_components(|range| {
            let carried = readback.carry_range(range.clone())?;
            Ok(requested.contains(&(carried.start, carried.end)))
        })?;
        components.into_iter().map(|component| component.into_iter().map(|range| {
            let carried = readback.carry_range(range.clone())?;
            ensure!(before.get(range) == after.get(carried.clone()),
                "Original key operations changed a complete pitched occurrence");
            Ok(carried)
        }).collect()).collect()
    }
}
