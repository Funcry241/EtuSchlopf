//! Master adopted state: the productive Master document and its lineage as one
//! sovereign object.
//!
//! The document field is private to this module. It can be read through
//! `document()`, and it changes only through the adoption functions below.
//! Each of them commits the owners' receipts into the lineage in the same call,
//! so XML and lineage can never be adopted or overwritten independently.
//!
//! r1219 Master sovereignty closure: adoption accepts no gap reason. A Master
//! transition is adopted only if the owners' receipts cover it completely and
//! continuously; otherwise the run aborts with a fatal sovereignty violation.
//! There is no candidate fallback, no alternative selection and no untrusted
//! gap. The only lineage-commit capability is constructed in this module.
//!
//! Adoption carries provenance only. It makes no selection, ranking, target,
//! repair-scope or musical decision, and it never writes MusicXML bytes of its
//! own: the bytes it adopts were produced by a population owner or a frozen
//! PRESERVING/ADDITIVE exclusion owner.

use crate::master_state_identity::{
    count_master_adoption, master_sovereignty_violation, record_master_artifact_witness,
    MasterLineage, OwnerReceipts, PreservingTransformationReceipt,
};

/// Read-only course custody census written to stderr before the unchanged
/// commit; text-only transitions of already carried kinds are not written.
/// Part of the selected Master observation only (see `SelectedCustody::observe`).
/// Nothing here can refuse, alter, order or consume a receipt.
fn emit_custody_census(owner_receipts: &OwnerReceipts, before: &str, after: &str, transition_label: &str) {
    if before != after
        && let Some(line) = owner_receipts.custody_census_line(before, after, transition_label)
    {
        let _ = std::io::Write::write_fmt(&mut std::io::stderr().lock(), format_args!("{line}\n"));
    }
}

/// The Master adoption boundary's lineage-commit capability. Its constructor is
/// private to this module.
#[derive(Debug)]
pub(crate) struct MasterAdoptionCommitBoundary {
    _sealed: (),
}

const COMMIT_BOUNDARY: MasterAdoptionCommitBoundary = MasterAdoptionCommitBoundary { _sealed: () };

#[derive(Debug, Clone)]
pub(crate) struct MasterAdoptedState {
    document: String,
    lineage: MasterLineage,
    selected_custody: Option<SelectedCustody>,
    actual_beam_capture: Option<Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>>,
}

/// Observational continuation of the selected candidate only. Unknown is
/// sticky and retains the last authenticated context and first failed boundary.
#[derive(Debug, Clone)]
struct SelectedCustody {
    project: String,
    bound: Option<std::sync::Arc<crate::arrangement::OwnedRhythmicContinuation>>,
    unknown: Option<String>,
}

impl SelectedCustody {
    fn continued(&self, receipts: &OwnerReceipts, after: &str, label: &str) -> Self {
        let mut next = self.clone();
        if next.unknown.is_none() && let Some(bound) = &self.bound {
            match receipts.continue_owned_rhythm(bound, after).and_then(|state| {
                state.require_complete_course_context(after)?;
                Ok(state)
            }) {
                Ok(state) => next.bound = Some(std::sync::Arc::new(state)),
                Err(error) => next.unknown = Some(format!("Unknown Boundary={label}: {error}")),
            }
        }
        next
    }

    /// One observation of an adopted selected-Master transition: its census line,
    /// then its continued custody. Unselected solver trials have no selected
    /// custody and therefore pay for neither.
    fn observe(&self, receipts: &OwnerReceipts, before: &str, after: &str, label: &str) -> Self {
        emit_custody_census(receipts, before, after, label);
        self.continued(receipts, after, label)
    }

    fn emit(&self, label: &str) {
        let (context, reason) = self.unknown.as_deref().map_or(("complete_received_course_context", "none"), |reason| ("Unknown", reason));
        let rest_annotations = self.bound.as_ref().map(|bound| bound.nonphysical_rest_obligation_count());
        let reparameterized = self.bound.as_ref().map(|bound| bound.reparameterized_obligation_count());
        let _ = std::io::Write::write_fmt(&mut std::io::stderr().lock(), format_args!(
            "SelectedMasterCustody: Project={} Boundary={label} Context={context};Reason={reason};CanonicalPitchedRoleClosure=none;ChannelAllocationPermit=none;MusicMutation=none;RestObligationsAwaitingFinalGate={rest_annotations:?};ReparameterizedObligations={reparameterized:?};ObligationContextEpoch=last_authenticated_course\n", self.project));
    }
}

impl MasterAdoptedState {
    /// Adopts the first state of a candidate. The lineage must start at a
    /// minted GENESIS and its head must bind `document` exactly; there is no
    /// gap bridge.
    pub(crate) fn adopt_genesis_state(document: String, lineage: MasterLineage) -> Self {
        if !lineage.lineage_head_binds(&document) {
            master_sovereignty_violation(
                "genesis adoption: lineage head does not bind the adopted Master document",
            );
        }
        count_master_adoption();
        Self { document, lineage, selected_custody: None, actual_beam_capture: None }
    }

    pub(crate) fn observe_selected_custody(&mut self, project: &str,
        bound: Option<std::sync::Arc<crate::arrangement::OwnedRhythmicContinuation>>,
        unknown: Option<String>) {
        if self.selected_custody.is_some() { return; }
        let unknown = unknown.or_else(|| match &bound {
            Some(state) => state.require_complete_course_context(&self.document).err().map(|error| error.to_string()),
            None => Some("Unknown Missing selected received custody".to_string()),
        });
        let custody = SelectedCustody { project: project.to_string(), bound, unknown };
        custody.emit("SelectedSolverMaster");
        self.selected_custody = Some(custody);
    }

    pub(crate) fn document(&self) -> &str {
        &self.document
    }

    pub(crate) fn original_respelling_input(&self, export_xml: &str)
        -> crate::master_state_identity::OriginalFunctionalRespellingChains {
        let received = (|| {
            let custody = self.selected_custody.as_ref().ok_or_else(|| "Missing selected original course custody".to_string())?;
            if let Some(error) = &custody.unknown { return Err(error.clone()); }
            let state = custody.bound.as_ref().ok_or_else(|| "Missing original course binding".to_string())?;
            state.validate_binding(export_xml).map_err(|error| error.to_string())?;
            Ok(state.clone())
        })();
        crate::master_state_identity::OriginalFunctionalRespellingChains::received(received)
    }

    pub(crate) fn actual_beam_capture(&self) -> Option<&Result<std::sync::Arc<crate::pipeline::ActualCanonicalBeamCapture>, String>> {
        self.actual_beam_capture.as_ref()
    }

    pub(crate) fn lineage(&self) -> &MasterLineage {
        &self.lineage
    }

    /// Adopts a document that owners already materialized from the current
    /// document. The owners' receipts must cover the whole transition in
    /// application order. `transition_label` is diagnostic text for a fatal
    /// violation only; it carries no reason, authority or fallback.
    pub(crate) fn adopt_materialized_document(
        &mut self,
        document: String,
        owner_receipts: OwnerReceipts,
        transition_label: &'static str,
    ) {
        let next_custody = self.selected_custody.as_ref().map(|state|
            state.observe(&owner_receipts, &self.document, &document, transition_label));
        let next_beam_capture = owner_receipts.observe_actual_beam_continuation(self.actual_beam_capture.as_ref());
        self.lineage.commit_owner_receipts(
            &COMMIT_BOUNDARY,
            owner_receipts,
            &document,
            transition_label,
        );
        self.document = document;
        self.actual_beam_capture = next_beam_capture;
        self.selected_custody = next_custody;
        if let Some(state) = &self.selected_custody { state.emit(transition_label); }
        count_master_adoption();
    }

    /// Adopts the transition of a path-bound PRESERVING owner whose frozen
    /// signature writes a file. The owner captured its real pre bytes and
    /// carries its own verified post bytes in the receipt; they are adopted
    /// here together with the receipt. `None` means the owner left the bytes
    /// unchanged (no event). A receipt without its realized bytes is fatal.
    pub(crate) fn adopt_path_bound_preserving_transition(
        &mut self,
        receipt: Option<PreservingTransformationReceipt>,
        transition_label: &'static str,
    ) {
        let Some(receipt) = receipt else {
            return;
        };
        let (receipt, document) = receipt.split_realized_post_document();
        let Some(document) = document else {
            master_sovereignty_violation(&format!(
                "{transition_label}: path-bound PRESERVING receipt carries no realized post bytes"
            ));
        };
        let mut owner_receipts = OwnerReceipts::default();
        owner_receipts.push_preserving(Some(receipt));
        self.adopt_materialized_document(document, owner_receipts, transition_label);
    }

    /// Gate D witness: the written Master artifact is only an integrity witness
    /// of the sovereign state. Its bytes must equal the sovereign document byte
    /// for byte (not merely by fingerprint); any deviation is fatal. The
    /// artifact bytes are never adopted as Master state.
    pub(crate) fn witness_released_artifact(&self, artifact_bytes: &str) {
        let artifact_matches = artifact_bytes.as_bytes() == self.document.as_bytes();
        record_master_artifact_witness(artifact_matches);
        if !artifact_matches {
            master_sovereignty_violation(
                "ReleasedMaster: written Master artifact bytes differ from the sovereign Master state",
            );
        }
    }

    /// Adopts the result of owners that transform the document in place under
    /// their frozen `&mut String` signatures. The transformation returns the
    /// owners' receipts; the lineage is committed to the resulting document in
    /// the same call.
    pub(crate) fn adopt_in_place_transformation<T>(
        &mut self,
        transition_label: &'static str,
        transformation: impl FnOnce(&mut String) -> (T, OwnerReceipts),
    ) -> T {
        let observed_before = self.selected_custody.as_ref().map(|_| self.document.clone());
        let (value, owner_receipts) = transformation(&mut self.document);
        let next_custody = self.selected_custody.as_ref().zip(observed_before.as_deref()).map(|(state, before)|
            state.observe(&owner_receipts, before, &self.document, transition_label));
        let next_beam_capture = owner_receipts.observe_actual_beam_continuation(self.actual_beam_capture.as_ref());
        self.lineage.commit_owner_receipts(
            &COMMIT_BOUNDARY,
            owner_receipts,
            &self.document,
            transition_label,
        );
        self.actual_beam_capture = next_beam_capture;
        self.selected_custody = next_custody;
        if let Some(state) = &self.selected_custody { state.emit(transition_label); }
        count_master_adoption();
        value
    }
}

#[cfg(test)]
mod tests {
    use super::MasterAdoptedState;
    use crate::master_state_identity::tests::{
        permitted_exact_receipt_for_tests, synthetic_file_bound_preserving_receipt_for_tests,
        synthetic_genesis_for_tests, synthetic_score_for_tests,
    };
    use crate::master_state_identity::{MasterLineage, OwnerReceipts};

    #[test]
    fn selected_unknown_retains_first_boundary_and_never_heals_after_later_owner() {
        let custody = crate::arrangement::OwnedRhythmicContinuation::synthetic_received_for_test();
        let mut master = state(custody.document());
        master.observe_selected_custody("synthetic", Some(std::sync::Arc::new(custody.clone())), None);
        // An actual note-free owner succeeds through the in-place adoption path.
        master.adopt_in_place_transformation("SyntheticFinalBarline", |xml| {
            let (_, receipt) = crate::structural_write_authority::ensure_final_barline_authorized(xml).unwrap();
            let mut receipts = OwnerReceipts::default(); receipts.push_additive(receipt);
            ((), receipts)
        });
        let carried = master.selected_custody.as_ref().unwrap();
        assert!(carried.unknown.is_none());
        carried.bound.as_ref().unwrap().require_complete_course_context(master.document()).unwrap();
        let last_bound = carried.bound.as_ref().unwrap().clone();
        // An unsupported original PRESERVING transition is still admitted by the
        // existing commit but cannot supply a fabricated custody continuation.
        let before = master.document().to_string();
        let after = before.replace("<attributes>", "<print new-page=\"yes\"/><attributes>");
        let mut receipts = OwnerReceipts::default();
        receipts.push_preserving(synthetic_file_bound_preserving_receipt_for_tests(&before, after.clone()));
        master.adopt_materialized_document(after.clone(), receipts, "SyntheticUnsupportedPresentation");
        let unknown = master.selected_custody.as_ref().unwrap().unknown.clone().unwrap();
        assert!(unknown.contains("SyntheticUnsupportedPresentation") && unknown.contains("OwnerStep=1"));
        assert_eq!(master.selected_custody.as_ref().unwrap().bound.as_ref().unwrap().document(), last_bound.document());
        master.observe_selected_custody("synthetic", Some(last_bound), None);
        assert_eq!(master.selected_custody.as_ref().unwrap().unknown.as_deref(), Some(unknown.as_str()));
        // A later unchanged boundary cannot replace the first failure with success.
        master.adopt_materialized_document(after.clone(), OwnerReceipts::default(), "SyntheticLaterNoop");
        assert_eq!(master.selected_custody.as_ref().unwrap().unknown.as_deref(), Some(unknown.as_str()));
        assert!(master.lineage().lineage_continuity_holds(&after));
        assert_eq!(master.document(), after);
    }

    fn state(document: &str) -> MasterAdoptedState {
        MasterAdoptedState::adopt_genesis_state(
            document.to_string(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(document)),
        )
    }

    #[test]
    #[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
    fn selected_custody_observation_cannot_admit_missing_original_receipts() {
        let custody = crate::arrangement::OwnedRhythmicContinuation::synthetic_received_for_test();
        let mut master = state(custody.document());
        master.observe_selected_custody("synthetic", Some(std::sync::Arc::new(custody)), None);
        let mut post = master.document().to_string();
        let (_, original) = crate::structural_write_authority::ensure_final_barline_authorized(&mut post).unwrap();
        assert!(original.is_some());
        master.adopt_materialized_document(post, OwnerReceipts::default(), "SyntheticMissingReceipt");
    }

    #[test]
    fn genesis_state_is_continuous() {
        let adopted = state("<score/>");
        assert_eq!(adopted.document(), "<score/>");
        assert!(adopted.lineage().lineage_continuity_holds(adopted.document()));
    }

    #[test]
    #[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
    fn genesis_adoption_of_a_foreign_document_is_fatal() {
        let _ = MasterAdoptedState::adopt_genesis_state(
            "<other/>".to_string(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests("<score/>")),
        );
    }

    #[test]
    fn covered_adoption_moves_document_and_lineage_together() {
        let mut adopted = state("<pitch>0</pitch>");
        let mut receipts = OwnerReceipts::default();
        receipts.push_exact(Some(permitted_exact_receipt_for_tests("<pitch>0</pitch>", "<pitch>1</pitch>")));
        adopted.adopt_materialized_document("<pitch>1</pitch>".to_string(), receipts, "test");
        assert!(adopted.lineage().lineage_continuity_holds(adopted.document()));
        assert_eq!(adopted.lineage().committed_event_count(), 1);

        let length = adopted.adopt_in_place_transformation("test", |xml| {
            let pre = xml.clone();
            xml.replace_range(7..8, "2");
            let mut receipts = OwnerReceipts::default();
            receipts.push_exact(Some(permitted_exact_receipt_for_tests(&pre, xml)));
            (xml.len(), receipts)
        });
        assert_eq!(length, adopted.document().len());
        let cloned = adopted.clone();
        assert!(cloned.lineage().lineage_continuity_holds(cloned.document()));
        assert_eq!(cloned.lineage().committed_event_count(), 2);
    }

    #[test]
    #[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
    fn uncovered_materialized_adoption_is_fatal() {
        let mut adopted = state("<score/>");
        adopted.adopt_materialized_document(
            "<score><a/></score>".to_string(),
            OwnerReceipts::default(),
            "test",
        );
    }

    #[test]
    #[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
    fn uncovered_in_place_adoption_is_fatal() {
        let mut adopted = state("<score/>");
        adopted.adopt_in_place_transformation("test", |xml| {
            xml.push_str("<!-- tail -->");
            ((), OwnerReceipts::default())
        });
    }

    #[test]
    fn unchanged_in_place_transformation_needs_no_receipt() {
        let mut adopted = state("<score/>");
        adopted.adopt_in_place_transformation("test", |_xml| ((), OwnerReceipts::default()));
        assert_eq!(adopted.lineage().committed_event_count(), 0);
    }

    fn panics_with_sovereignty_violation(run: impl FnOnce() + Send + 'static) {
        let outcome = std::thread::spawn(run).join();
        let payload = outcome.expect_err("adoption must abort");
        let message = payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|text| (*text).to_string()))
            .unwrap_or_default();
        assert!(message.contains("MASTER SOVEREIGNTY VIOLATION"), "{message}");
    }

    #[test]
    fn missing_receipt_is_fatal_for_candidate_trial_and_worker_states() {
        // A solver candidate, a trial clone and a worker-thread clone all share
        // the one adoption contract: an uncovered change aborts, no fallback.
        let candidate = state("<d>0</d>");
        let trial = candidate.clone();
        let worker = candidate.clone();
        panics_with_sovereignty_violation(move || {
            let mut candidate = candidate;
            candidate.adopt_materialized_document("<d>1</d>".to_string(), OwnerReceipts::default(), "candidate");
        });
        panics_with_sovereignty_violation(move || {
            let mut trial = trial;
            trial.adopt_in_place_transformation("trial", |xml| {
                xml.replace_range(3..4, "1");
                ((), OwnerReceipts::default())
            });
        });
        panics_with_sovereignty_violation(move || {
            let mut worker = worker;
            let _ = std::thread::spawn(move || {
                worker.adopt_materialized_document("<d>2</d>".to_string(), OwnerReceipts::default(), "worker");
            })
            .join()
            .map_err(std::panic::resume_unwind);
        });
    }

    #[test]
    fn artifact_witness_accepts_byte_identical_artifact() {
        let adopted = state("<score>final</score>");
        adopted.witness_released_artifact("<score>final</score>");
    }

    #[test]
    #[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
    fn artifact_witness_rejects_a_single_byte_deviation() {
        let adopted = state("<score>final</score>");
        adopted.witness_released_artifact("<score>finaL</score>");
    }

    #[test]
    fn path_bound_preserving_transition_adopts_the_owner_bytes() {
        let pre = synthetic_score_for_tests("");
        let post = pre.replace("<attributes>", "<print new-page=\"yes\"/><attributes>");
        let mut adopted = state(&pre);
        adopted.adopt_path_bound_preserving_transition(
            synthetic_file_bound_preserving_receipt_for_tests(&pre, post.clone()),
            "test",
        );
        assert_eq!(adopted.document(), post);
        assert_eq!(adopted.lineage().committed_event_count(), 1);
        assert!(adopted.lineage().lineage_continuity_holds(&post));
        // An unchanged path-bound owner yields no receipt and no event.
        adopted.adopt_path_bound_preserving_transition(None, "test");
        assert_eq!(adopted.lineage().committed_event_count(), 1);
        adopted.witness_released_artifact(&post);
    }

    #[test]
    #[should_panic(expected = "MASTER SOVEREIGNTY VIOLATION")]
    fn path_bound_preserving_receipt_from_a_foreign_pre_state_is_fatal() {
        let pre = synthetic_score_for_tests("");
        let foreign = pre.replace("<divisions>2</divisions>", "<divisions>2</divisions><staves>1</staves>");
        let post = foreign.replace("<attributes>", "<print new-page=\"yes\"/><attributes>");
        let mut adopted = state(&pre);
        adopted.adopt_path_bound_preserving_transition(
            synthetic_file_bound_preserving_receipt_for_tests(&foreign, post),
            "test",
        );
    }

}

#[cfg(test)]
#[path = "../../../tests/unit/selected_preserving_receipt_adoption.rs"]
mod selected_preserving_receipt_adoption_tests;
