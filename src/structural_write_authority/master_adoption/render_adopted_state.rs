//! Sovereign Render state: the productive Render document and its lineage as
//! one object (r1230 RenderSovereigntyClosureMaxi).
//!
//! `RenderSovereignState` is the only Render state authority of a project run.
//! Its document field is private; it changes only through the adoption
//! functions below, each of which (1) binds the transition to the exact root
//! it was computed from, (2) replays the owners' receipts byte-exactly from
//! that root, (3) lets the typed sink write exactly the verified bytes, (4)
//! witnesses the artifact readback against those bytes and (5) commits the
//! receipts into the Render lineage. A verify, write or witness failure leaves
//! the sovereign state unchanged.
//!
//! The first Render state exists only through a closed Genesis G1,G2,G3,G5; a Master
//! document replaces existing Render history only through S4. Legacy
//! availability flags select branches; they never create, erase or replace
//! Render state. Reading the artifact is legal only as a witness against the
//! sovereign bytes. A missing state where the legacy branch requires one
//! (class F) is fatal.
//!
//! The adoption boundary makes no selection, ranking or musical decision and
//! writes no MusicXML bytes of its own: the bytes it adopts are produced by
//! population owners or frozen exclusion owners and persisted by their typed
//! sinks.

use crate::master_state_identity::{
    count_render_closure_failure, count_render_declined_sink_write,
    count_render_unchanged_sink_write, record_final_render_lineage,
    record_render_artifact_read, render_sovereignty_violation, ExactTransformationReceipt,
    RenderArtifactReadPurpose, RenderClosureMetric, RenderClosureVerdict, RenderLineage,
    RenderLineagePosition, RenderMasterEpoch, RenderMasterEpochBinding, RenderOriginFork,
    RenderOriginId, RenderSovereignIdentity, RenderTransitionBasis,
    RenderTransitionReceipts, VerifiedRenderTransition,
};
use std::path::{Path, PathBuf};

use super::MasterAdoptedState;

/// The Render adoption boundary's lineage-commit capability. Its constructor
/// is private to this module.
#[derive(Debug)]
pub(crate) struct RenderAdoptionCommitBoundary {
    _sealed: (),
}

const COMMIT_BOUNDARY: RenderAdoptionCommitBoundary = RenderAdoptionCommitBoundary { _sealed: () };

/// The active typed #58 checkpoint entries; retired entries cannot be adopted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderCheckpointSink {
    ClefIslandDecomposition,
    FinalNotation,
    CursorLaneRepair,
    RetentionReturn,
    DuplicateRestRewrite,
    VisibleRestRole,
    FinalPianoRestGlyph,

    SisterFeedbackBeam,
    ExternalFeedbackClosure,
    KeySignatureSpelling,
    DirectionRelocation,
}

/// Every productive Render artifact sink.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderSinkKind {
    PrunedPdfTrial,
    HiddenRestForwardCollapse,
    PianisticRestRepairTrial,
    PrintCleanupOneSafeVisual,
    MissingRestInsertTrial,
    BehindBarsRestTieAudit,
    LowerStaffVisibleRhythmTrial,
    SecondaryReanchoredRestTrial,
    StrictStaffReleaseTrial,
    StrictVisibleRestArithmeticTrial,
    ExternalNotationFeedbackAudit,
    FragmentedRestChainTrial,
    LowerClefSurfaceMaterialization,
    FinalLayoutPhraseProjection,
    Checkpoint(RenderCheckpointSink),
}

impl RenderSinkKind {
    /// Only the FinalNotation checkpoint carries the #58 owner's own
    /// ClefReadability edits.
    fn admits_checkpoint_owner(self) -> bool {
        self == Self::Checkpoint(RenderCheckpointSink::FinalNotation)
    }

    fn is_checkpoint(self) -> bool {
        matches!(self, Self::Checkpoint(_))
    }

    fn label(self) -> String {
        format!("{self:?}")
    }
}

/// The late FinalNotation legacy branches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderLateBranch {
    /// B1 lower clef surface: `avail || path.exists()` selects Render.
    LowerClefSurface,
    /// B4 cursor-neutral writer: `avail` selects Render.
    CursorNeutralWriter,
}

#[derive(Debug, Clone, Copy)]
enum RootBasis {
    State(RenderLineagePosition),
    Origin {
        origin: RenderOriginId,
        fork: RenderOriginFork,
        legacy_render_source_available: bool,
        /// Opaque identity of the sovereign state that issued this root.
        issuer: RenderSovereignIdentity,
        /// Frozen Master epoch this origin was derived from.
        epoch_binding: RenderMasterEpochBinding,
        /// Existing Render lineage position at root creation. Genesis roots
        /// carry None; history-preserving S4 roots carry the exact current
        /// position so they become stale after any later Render commit.
        history_position: Option<RenderLineagePosition>,
    },
    HistoricalCheckpoint58(RenderLineagePosition),
}

/// The exact sovereign root a Render candidate or trial is computed from. It
/// is neither clonable nor reusable: one root admits at most one adoption.
#[derive(Debug)]
pub(crate) struct RenderTrialRoot {
    basis: RootBasis,
    document: String,
}

impl RenderTrialRoot {
    pub(crate) fn document(&self) -> &str {
        &self.document
    }

    /// Whether the root is an existing Render state (history present).
    pub(crate) fn has_render_history(&self) -> bool {
        match self.basis {
            RootBasis::Origin { history_position, .. } => history_position.is_some(),
            RootBasis::State(_) | RootBasis::HistoricalCheckpoint58(_) => true,
        }
    }
}

/// Outcome of a path-bound frozen owner that reads and writes the Render
/// artifact itself.
#[derive(Debug)]
pub(crate) struct RenderPathBoundOutcome<T> {
    pub(crate) value: T,
    /// The bytes a prepared path-bound owner intends to persist, carried by
    /// its receipt/realized post. They are verified before the physical sink.
    pub(crate) planned_post: Option<String>,
    pub(crate) receipts: RenderTransitionReceipts,
    /// Preparation failed before any productive artifact write. Nothing is
    /// adopted; closure records a failed candidate materialization.
    pub(crate) failed: bool,
}

#[derive(Debug)]
pub(crate) struct RenderAdoptedState {
    document: String,
    lineage: RenderLineage,
}

/// The sovereign Render state authority of one project run.
#[derive(Debug)]
pub(crate) struct RenderSovereignState {
    artifact_path: PathBuf,
    identity: RenderSovereignIdentity,
    adopted: Option<RenderAdoptedState>,
    early_epoch: RenderMasterEpoch,
    late_epoch: Option<RenderMasterEpoch>,
    last_checkpoint_58: Option<(RenderLineagePosition, String)>,
}

fn read_artifact(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

impl RenderSovereignState {
    /// Starts the Render pass of a project on the sovereign Master selected
    /// before the initial Render trials (the G1/G2 epoch).
    pub(crate) fn begin_project_render(artifact_path: &Path, master: &MasterAdoptedState) -> Self {
        Self {
            artifact_path: artifact_path.to_path_buf(),
            identity: RenderSovereignIdentity::mint(),
            adopted: None,
            early_epoch: RenderMasterEpoch::before_initial_render_trials(
                master.lineage(),
                master.document(),
            ),
            late_epoch: None,
            last_checkpoint_58: None,
        }
    }

    pub(crate) fn artifact_path(&self) -> &Path {
        &self.artifact_path
    }

    /// Whether Render history exists (a Render state was adopted).
    #[cfg(test)]
    pub(crate) fn has_history(&self) -> bool {
        self.adopted.is_some()
    }

    /// The sovereign Render document, or explicit absence.
    pub(crate) fn document(&self) -> Option<&str> {
        self.adopted.as_ref().map(|state| state.document.as_str())
    }

    /// Class-F boundary: the legacy branch requires an existing Render state.
    pub(crate) fn require_document(&self, label: &str) -> &str {
        match self.document() {
            Some(document) => document,
            None => render_sovereignty_violation(
                RenderClosureMetric::FileStateAuthorityReads,
                &format!("{label}: legacy Render availability without a sovereign Render state (class F)"),
            ),
        }
    }

    /// The current Render document or, without history, the given legacy
    /// fallback. Used only by read-only audits that observe "Render state or
    /// explicit absence"; never by a transition.
    pub(crate) fn observed_document_or<'a>(&'a self, absent: &'a str) -> &'a str {
        self.document().unwrap_or(absent)
    }

    /// Binds the late Master epoch (after #56, before #65) for G3-G5/S4.
    pub(crate) fn bind_late_master_epoch(&mut self, epoch: RenderMasterEpoch) {
        if self.late_epoch.is_some() {
            render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedMasterSubstitution,
                "late Render Master epoch bound twice",
            );
        }
        self.late_epoch = Some(epoch);
    }

    fn late_epoch(&self) -> &RenderMasterEpoch {
        match self.late_epoch.as_ref() {
            Some(epoch) => epoch,
            None => render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedMasterSubstitution,
                "late Render origin without its bound Master epoch",
            ),
        }
    }

    fn state_root(&self, label: &str) -> RenderTrialRoot {
        let state = match self.adopted.as_ref() {
            Some(state) => state,
            None => render_sovereignty_violation(
                RenderClosureMetric::FileStateAuthorityReads,
                &format!("{label}: Render root requested without a sovereign Render state (class F)"),
            ),
        };
        RenderTrialRoot {
            basis: RootBasis::State(state.lineage.position()),
            document: state.document.clone(),
        }
    }

    /// Root of the current Render state; class F if there is none.
    pub(crate) fn require_state_root(&self, label: &str) -> RenderTrialRoot {
        self.state_root(label)
    }

    /// Root of an initial Render trial: the existing Render state, or without
    /// history the G1 (pruned trial) / G2 (trials 2..8) origin on the early
    /// Master epoch.
    pub(crate) fn initial_trial_root(&self, pruned_pdf_trial: bool) -> RenderTrialRoot {
        if self.adopted.is_some() {
            return self.state_root("initial Render trial");
        }
        let fork = if pruned_pdf_trial {
            RenderOriginFork::PrunedPdfTrial
        } else {
            RenderOriginFork::FirstInitialTrial2To8
        };
        let origin = RenderOriginId::admitted_at(fork, false, false)
            .unwrap_or_else(|| render_sovereignty_violation(RenderClosureMetric::UnauthorizedGenesis, "initial Render trial without admitted Genesis"));
        RenderTrialRoot {
            basis: RootBasis::Origin {
                origin,
                fork,
                legacy_render_source_available: false,
                issuer: self.identity,
                epoch_binding: self.early_epoch.binding(),
                history_position: None,
            },
            document: self.early_epoch.document().to_string(),
        }
    }

    /// Root of a late FinalNotation branch, reproducing the frozen legacy
    /// selection: an existing Render state when the branch selects Render,
    /// otherwise the closed G3/G5 (no history) or S4 (history) origin
    /// on the late Master epoch. Any other combination is class F.
    pub(crate) fn final_notation_branch_root(
        &self,
        branch: RenderLateBranch,
        legacy_render_source_available: bool,
    ) -> RenderTrialRoot {
        let selects_render = match branch {
            RenderLateBranch::LowerClefSurface => {
                legacy_render_source_available || self.adopted.is_some()
            }
            RenderLateBranch::CursorNeutralWriter => legacy_render_source_available,
        };
        if selects_render {
            return self.state_root(&format!("{branch:?}"));
        }
        let fork = match branch {
            RenderLateBranch::LowerClefSurface => RenderOriginFork::FinalNotationB1,
            RenderLateBranch::CursorNeutralWriter => RenderOriginFork::FinalNotationB4,
        };
        let origin = RenderOriginId::admitted_at(
            fork,
            self.adopted.is_some(),
            legacy_render_source_available,
        )
        .unwrap_or_else(|| {
            render_sovereignty_violation(
                RenderClosureMetric::UnauthorizedMasterSubstitution,
                &format!("{branch:?}: no closed Render origin for the actual history"),
            )
        });
        let history_position = self
            .adopted
            .as_ref()
            .map(|state| state.lineage.position());
        RenderTrialRoot {
            basis: RootBasis::Origin {
                origin,
                fork,
                legacy_render_source_available,
                issuer: self.identity,
                epoch_binding: self.late_epoch().binding(),
                history_position,
            },
            document: self.late_epoch().document().to_string(),
        }
    }

    /// The last #58 committed Render state, admitted only while the current
    /// sovereign state is still exactly that commit (the frozen legacy sink
    /// binding of the DirectionRelocation event).
    pub(crate) fn last_checkpoint_58_basis(&self) -> Option<&str> {
        let (_, document) = self.last_checkpoint_58.as_ref()?;
        (self.document() == Some(document.as_str())).then_some(document.as_str())
    }

    /// Test inspection of recorded history, independent of current eligibility.
    #[cfg(test)]
    pub(crate) fn last_checkpoint_58_recorded_document(&self) -> Option<&str> {
        self.last_checkpoint_58.as_ref().map(|(_, document)| document.as_str())
    }

    /// Root on the last #58 committed Render state for DirectionRelocation.
    pub(crate) fn historical_checkpoint_58_root(&self) -> Option<RenderTrialRoot> {
        let state = self.adopted.as_ref()?;
        let basis = self.last_checkpoint_58_basis()?;
        Some(RenderTrialRoot {
            basis: RootBasis::HistoricalCheckpoint58(state.lineage.position()),
            document: basis.to_string(),
        })
    }

    /// A PDF feasibility branch verifies and closes its own receipts without
    /// changing Main history or registering its checkpoint as Main's #58 basis.
    pub(crate) fn materialize_shadow_final_notation_checkpoint(
        &self, root: RenderTrialRoot, candidate: &str,
        receipts: RenderTransitionReceipts, shadow_path: &Path,
    ) -> std::io::Result<String> {
        self.materialize_shadow_final_notation_checkpoint_with_readback(root, candidate, receipts, shadow_path,
            |path| std::fs::read_to_string(path))
    }

    fn materialize_shadow_final_notation_checkpoint_with_readback(
        &self, root: RenderTrialRoot, candidate: &str,
        receipts: RenderTransitionReceipts, shadow_path: &Path,
        readback: impl FnOnce(&Path) -> std::io::Result<String>,
    ) -> std::io::Result<String> {
        use crate::master_state_identity::RenderDiscardReason;
        let aliases_main = std::fs::canonicalize(shadow_path).ok()
            .zip(std::fs::canonicalize(self.artifact_path()).ok())
            .is_some_and(|(shadow, main)| shadow == main);
        if shadow_path == self.artifact_path() || aliases_main {
            count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Shadow path equals Main artifact"));
        }
        self.verify_root_is_current(&root, "FinalPdfShadow");
        let (prepared, receipts) = Self::prepare_final_notation_owner(shadow_path, candidate, receipts)?;
        let verified = self.verify(&root, receipts, &prepared, "FinalPdfShadow");
        let result = super::super::orchestrator_artifact_sink_authority::write_prepared_final_notation_checkpoint(&COMMIT_BOUNDARY, shadow_path, verified.candidate());
        if let Err(error) = result {
            count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            verified.discard_verified(RenderDiscardReason::ReadOnlyProbe);
            return Err(error);
        }
        let a = verified.candidate().to_string();
        let observed = readback(shadow_path);
        verified.discard_verified(RenderDiscardReason::ReadOnlyProbe);
        match observed {
            Ok(bytes) if bytes == a => Ok(a),
            Ok(_) => {
                count_render_closure_failure(RenderClosureMetric::ArtifactReadbackMismatch);
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Shadow differs from witnessed post"))
            }
            Err(error) => {
                count_render_closure_failure(RenderClosureMetric::ArtifactReadbackMismatch);
                Err(error)
            }
        }
    }

    fn verify_root_is_current(&self, root: &RenderTrialRoot, label: &str) {
        let current = self.adopted.as_ref().map(|state| state.lineage.position());
        let fresh = match root.basis {
            RootBasis::State(position) | RootBasis::HistoricalCheckpoint58(position) => {
                current == Some(position)
                    && self.document() == Some(root.document.as_str())
            }
            RootBasis::Origin {
                origin,
                fork,
                issuer,
                epoch_binding,
                history_position,
                ..
            } => {
                if issuer != self.identity {
                    false
                } else {
                    let epoch = match fork {
                        RenderOriginFork::PrunedPdfTrial | RenderOriginFork::FirstInitialTrial2To8 => {
                            &self.early_epoch
                        }
                        _ => self.late_epoch(),
                    };
                    epoch_binding == epoch.binding()
                        && if origin.is_genesis() {
                            history_position.is_none() && current.is_none()
                        } else {
                            history_position.is_some() && current == history_position
                        }
                }
            }
        };
        if !fresh {
            render_sovereignty_violation(
                RenderClosureMetric::TrialCoverageFailures,
                &format!("{label}: candidate root is not the current sovereign Render state (stale or foreign root)"),
            );
        }
    }

    fn basis_of(&self, root: &RenderTrialRoot) -> RenderTransitionBasis<'_> {
        match root.basis {
            RootBasis::State(_) => RenderTransitionBasis::State,
            RootBasis::HistoricalCheckpoint58(_) => RenderTransitionBasis::HistoricalCheckpoint58,
            RootBasis::Origin {
                origin,
                fork,
                legacy_render_source_available,
                ..
            } => {
                let epoch = match fork {
                    RenderOriginFork::PrunedPdfTrial | RenderOriginFork::FirstInitialTrial2To8 => {
                        &self.early_epoch
                    }
                    _ => self.late_epoch(),
                };
                if origin.is_genesis() {
                    RenderTransitionBasis::Genesis {
                        origin,
                        fork,
                        epoch,
                        legacy_render_source_available,
                    }
                } else {
                    RenderTransitionBasis::Substitution {
                        origin,
                        fork,
                        epoch,
                        legacy_render_source_available,
                    }
                }
            }
        }
    }

    fn verify(
        &self,
        root: &RenderTrialRoot,
        receipts: RenderTransitionReceipts,
        candidate: &str,
        label: &str,
    ) -> VerifiedRenderTransition {
        let historical = match root.basis {
            RootBasis::HistoricalCheckpoint58(_) => self.last_checkpoint_58_basis(),
            _ => None,
        };
        VerifiedRenderTransition::verify(
            &COMMIT_BOUNDARY,
            self.basis_of(root),
            self.document(),
            historical,
            receipts,
            candidate,
            label,
        )
    }

    fn witness(&self, expected: Option<&str>, label: &str) {
        let artifact = read_artifact(&self.artifact_path);
        match expected {
            Some(expected) => record_render_artifact_read(
                RenderArtifactReadPurpose::WitnessAgainstSovereignState,
                artifact.as_deref(),
                expected,
                label,
            ),
            None => {
                if artifact.is_some() {
                    render_sovereignty_violation(
                        RenderClosureMetric::ArtifactReadbackMismatch,
                        &format!("{label}: Render artifact exists without a sovereign Render state"),
                    );
                }
            }
        }
    }

    fn commit(&mut self, verified: VerifiedRenderTransition, sink: RenderSinkKind, label: &str) {
        let current = self.document().map(str::to_string);
        let requires_commit = verified.requires_lineage_commit(current.as_deref());
        if requires_commit {
            let lineage = self.adopted.take().map(|state| state.lineage);
            let (lineage, document) = verified.commit(&COMMIT_BOUNDARY, lineage, label);
            self.adopted = Some(RenderAdoptedState { document, lineage });
        } else {
            // A sink write of unchanged bytes is no event: no receipt, no commit.
            count_render_unchanged_sink_write();
            drop(verified);
        }
        if sink.is_checkpoint()
            && let Some(state) = self.adopted.as_ref()
        {
            self.last_checkpoint_58 = Some((state.lineage.position(), state.document.clone()));
        }
    }

    /// Prepare the same original #58 owner event before replaying the complete chain.
    fn prepare_final_notation_owner(
        path: &Path,
        candidate: &str,
        mut receipts: RenderTransitionReceipts,
    ) -> std::io::Result<(String, RenderTransitionReceipts)> {
        let (prepared, receipt) = super::super::orchestrator_artifact_sink_authority::prepare_final_notation_checkpoint(path, candidate);
        receipts.push_exact(receipt);
        match prepared {
            Ok(xml) => Ok((xml, receipts)),
            Err(error) => {
                receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
                count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
                Err(error)
            }
        }
    }

    pub(crate) fn adopt_final_notation_checkpoint(
        &mut self,
        root: RenderTrialRoot,
        candidate: &str,
        receipts: RenderTransitionReceipts,
    ) -> std::io::Result<()> {
        self.adopt_final_notation_checkpoint_with_write(root, candidate, receipts,
            |path, xml| super::super::orchestrator_artifact_sink_authority::write_prepared_final_notation_checkpoint(&COMMIT_BOUNDARY, path, xml))
    }

    fn adopt_final_notation_checkpoint_with_write(
        &mut self,
        root: RenderTrialRoot,
        candidate: &str,
        receipts: RenderTransitionReceipts,
        write: impl FnOnce(&Path, &str) -> std::io::Result<()>,
    ) -> std::io::Result<()> {
        let sink = RenderSinkKind::Checkpoint(RenderCheckpointSink::FinalNotation);
        let label = sink.label();
        self.verify_root_is_current(&root, &label);
        let (prepared, receipts) = Self::prepare_final_notation_owner(&self.artifact_path, candidate, receipts)?;
        let verified = self.verify(&root, receipts, &prepared, &label);
        if let Err(error) = write(&self.artifact_path, verified.candidate()) {
            count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            verified.discard_verified(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
            return Err(error);
        }
        self.witness(Some(verified.candidate()), &label);
        self.commit(verified, sink, &label);
        Ok(())
    }

    /// Adopts a Render transition through a typed sink without additional owners.
    /// FinalNotation uses its separate preparation-before-write boundary.
    /// `write` receives the artifact path, root bytes and verified candidate;
    /// it must not introduce another owner event after verification.
    pub(crate) fn adopt_transition<E>(
        &mut self,
        root: RenderTrialRoot,
        candidate: &str,
        receipts: RenderTransitionReceipts,
        sink: RenderSinkKind,
        write: impl FnOnce(&Path, &str, &str) -> (Result<(), E>, Option<ExactTransformationReceipt>),
    ) -> Result<(), E> {
        if sink.admits_checkpoint_owner() {
            render_sovereignty_violation(RenderClosureMetric::ReceiptCoverageFailures,
                "FinalNotation must prepare its complete owner chain before the sink");
        }
        let label = sink.label();
        self.verify_root_is_current(&root, &label);
        let verified = self.verify(&root, receipts, candidate, &label);
        let (result, sink_receipt) = write(&self.artifact_path, root.document(), verified.candidate());
        if sink_receipt.is_some() {
            render_sovereignty_violation(
                RenderClosureMetric::ReceiptCoverageFailures,
                &format!("{label}: sink returned an owner receipt it does not own"),
            );
        }
        if let Err(error) = result {
            // The caller may handle the legacy error. Preserve the sovereign
            // state, but make the failed productive materialization fatal to
            // any later Render closure.
            count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            return Err(error);
        }
        let verified = verified.extend_with_sink_owner(&COMMIT_BOUNDARY, sink_receipt, &label);
        self.witness(Some(verified.candidate()), &label);
        self.commit(verified, sink, &label);
        Ok(())
    }

    /// Adopts a transition whose existing legacy sink first materializes a
    /// verified temporary/probe artifact and may then decline publication. The
    /// receipt chain is replayed before the callback may touch any path. `Ok(false)`
    /// is an ordinary losing candidate: no state/lineage commit is made.
    pub(crate) fn adopt_trial_publication_transition<E>(
        &mut self,
        root: RenderTrialRoot,
        candidate: &str,
        receipts: RenderTransitionReceipts,
        sink: RenderSinkKind,
        publish: impl FnOnce(&Path, &str, &str) -> Result<bool, E>,
    ) -> Result<bool, E> {
        let label = sink.label();
        if sink.is_checkpoint() {
            render_sovereignty_violation(
                RenderClosureMetric::ReceiptCoverageFailures,
                &format!("{label}: checkpoint sinks cannot use trial publication"),
            );
        }
        self.verify_root_is_current(&root, &label);
        let verified = self.verify(&root, receipts, candidate, &label);
        let published = match publish(&self.artifact_path, root.document(), verified.candidate()) {
            Ok(published) => published,
            Err(error) => {
                count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
                return Err(error);
            }
        };
        if !published {
            verified.discard_verified(
                crate::master_state_identity::RenderDiscardReason::LosingCandidate,
            );
            count_render_declined_sink_write();
            return Ok(false);
        }
        self.witness(Some(verified.candidate()), &label);
        self.commit(verified, sink, &label);
        Ok(true)
    }

    /// Explicitly declines a sink write the legacy branch did not perform.
    /// Unadopted receipts of the candidate are closed as losing candidate.
    pub(crate) fn decline_transition(
        &self,
        root: RenderTrialRoot,
        receipts: RenderTransitionReceipts,
    ) {
        drop(root);
        if !receipts.is_empty() {
            receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        }
        count_render_declined_sink_write();
    }

    /// Adopts a path-bound frozen owner in two phases. `prepare` runs without
    /// access to the productive artifact and returns the planned post plus its
    /// existing receipts. The transition is replayed and authority-checked
    /// before `write` may touch the artifact; readback and commit follow only
    /// after a successful physical write.
    pub(crate) fn adopt_path_bound_transition<T, E>(
        &mut self,
        root: RenderTrialRoot,
        sink: RenderSinkKind,
        prepare: impl FnOnce(&str) -> RenderPathBoundOutcome<T>,
        write: impl FnOnce(&Path, &str, &str) -> Result<(), E>,
    ) -> Result<T, E> {
        let label = sink.label();
        self.verify_root_is_current(&root, &label);
        let expected_before = if root.has_render_history() {
            self.document().map(str::to_string)
        } else {
            None
        };
        let artifact = read_artifact(&self.artifact_path);
        match expected_before.as_deref() {
            Some(expected) => record_render_artifact_read(
                RenderArtifactReadPurpose::GuardAgainstWitnessedArtifact,
                artifact.as_deref(),
                expected,
                &label,
            ),
            None => {
                if artifact.is_some() {
                    render_sovereignty_violation(
                        RenderClosureMetric::ArtifactReadbackMismatch,
                        &format!("{label}: Render artifact exists without a sovereign Render state"),
                    );
                }
            }
        }

        let outcome = prepare(root.document());
        if outcome.failed {
            // A failed owner execution is not a losing candidate. Its receipts,
            // if any, remain uncommitted and therefore drop as LineagePending;
            // the materialization failure itself also keeps closure open.
            count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            return Ok(outcome.value);
        }

        let value = outcome.value;
        let Some(post) = outcome.planned_post else {
            if !outcome.receipts.is_empty() {
                render_sovereignty_violation(
                    RenderClosureMetric::OwnerWriteWithoutReceipt,
                    &format!("{label}: path-bound owner returned receipts without a planned write"),
                );
            }
            count_render_declined_sink_write();
            return Ok(value);
        };

        // Critical ordering: no productive write occurs until the exact
        // planned post and all owner receipts have passed replay/authority.
        let verified = self.verify(&root, outcome.receipts, &post, &label);
        if let Err(error) = write(
            &self.artifact_path,
            root.document(),
            verified.candidate(),
        ) {
            count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            return Err(error);
        }
        self.witness(Some(verified.candidate()), &label);
        self.commit(verified, sink, &label);
        Ok(value)
    }

    /// Released-state closure: witnesses the final artifact against the
    /// sovereign state (or its explicit absence), records the final lineage and
    /// evaluates the closure gate. An open gate is fatal.
    pub(crate) fn close_render_sovereignty(&self) -> RenderClosureVerdict {
        let artifact = read_artifact(&self.artifact_path);
        let witnessed = artifact.as_deref() == self.document();
        self.finish_render_closure(witnessed)
    }

    fn finish_render_closure(&self, witnessed: bool) -> RenderClosureVerdict {
        let verdict = record_final_render_lineage(
            self.adopted
                .as_ref()
                .map(|state| (&state.lineage, state.document.as_str())),
            witnessed,
        );
        if !verdict.closed {
            let metric = verdict
                .failures
                .iter()
                .find(|(_, value)| *value > 0)
                .map_or(RenderClosureMetric::LineagePending, |(metric, _)| *metric);
            render_sovereignty_violation(
                metric,
                &format!("Render closure gate open: {}", verdict.detail),
            );
        }
        verdict
    }
}

include!("required_render_product.rs");

#[cfg(test)]
#[path = "render_adopted_state_tests.rs"]
mod tests;
