// Central source-bound authorization boundary for the already-existing final
// PDF-source notation commit. This module adds no detector, rule, threshold,
// repair, or recovery semantics. Master bytes are already committed before this
// boundary; only a selected Render source enters the FinalNotation materializer.

use std::path::Path;
use crate::master_state_identity::{count_render_closure_failure, record_render_artifact_read, RenderArtifactReadPurpose, RenderClosureMetric, RenderDiscardReason};
use crate::structural_write_authority::RenderSovereignState;

#[derive(Debug, Clone)]
pub(in crate::pipeline) struct FinalPdfSourceCommit {
    pipeline_line: String,
    closed: bool,
    committed_xml: Option<String>,
    committed_occurrence_continuation: Option<
        crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation,
    >,
    committed_voice_plan: Option<crate::structural_write_authority::FrozenSerializedVoicePlan>,
    primary_beam_voice_partition_changed_voice_elements: usize,
    phrase_occurrence_course: Option<crate::structural_write_authority::BoundPhraseOccurrenceCourse>,
}

impl FinalPdfSourceCommit {
    pub(in crate::pipeline) fn approve_phrase_course(&self,
        intents: &[crate::audit_only::phrase_boundary_evidence::PhraseNotationIntentProjection],
    ) -> anyhow::Result<crate::structural_write_authority::ApprovedPhraseOccurrenceCourse> {
        let continuation = self.committed_occurrence_continuation.as_ref()
            .ok_or_else(|| anyhow::anyhow!("E007 phrase approval lacks committed original occurrence course"))?;
        crate::structural_write_authority::approve_phrase_occurrence_course(continuation, intents)
    }

    /// Planning-branch execution of the already-authored ordinary partition on the
    /// real committed planning branch. It uses the existing scalar owner and
    /// its EXACT receipt, discards that probe, and creates no artifact or Main
    /// adoption. FinalVoice, Clef and Rest still execute once on the live course.
    pub(in crate::pipeline) fn approve_phrase_course_after_historical_metric_partition(&self,
        intents: &[crate::audit_only::phrase_boundary_evidence::PhraseNotationIntentProjection],
        combined: &crate::structural_write_authority::PrimaryBeamVoicePartitionBoundPlan,
        canonical: &crate::structural_write_authority::CanonicalFinalChordStructuralPlan,
        omissions: &std::collections::BTreeSet<crate::structural_write_authority::RenderPrunedFinalChordOmissionKey>,
    ) -> anyhow::Result<crate::structural_write_authority::ApprovedPhraseOccurrenceCourse> {
        anyhow::ensure!(!combined.permits_pdf_finalization(), "E007 reference requires its sealed historical-only purpose");
        let ordinary = combined;
        let source = self.committed_occurrence_continuation.as_ref()
            .ok_or_else(|| anyhow::anyhow!("E007 ordinary partition lacks its committed occurrence course"))?;
        let rebound = crate::structural_write_authority::rebind_primary_beam_voice_partition_plan_to_current_xml(
            source.xml(), source, ordinary)?;
        eprintln!("PhraseOrdinaryPartitionDecisionCourse: pending={}; edits={:?}; authority=existing_actual_metric_partition",intents.len(),rebound.edits().iter().map(|edit|(edit.voice_text_start(),edit.voice_text_end(),edit.target_label())).collect::<Vec<_>>());
        if rebound.edits().is_empty() { return self.approve_phrase_course(intents); }
        let edits = rebound.edits().iter().map(|edit|
            (edit.voice_text_start(), edit.voice_text_end(), edit.target_label().to_owned())).collect();
        let (post, receipt) = crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(source.xml(), edits);
        let owner_exact = post.as_ref().zip(receipt.as_ref()).is_some_and(|(post, receipt)|
            receipt.realize_post_document(source.xml()).as_deref() == Some(post.as_str()));
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
        probe.push_exact(receipt);
        let carried = (|| -> anyhow::Result<crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation> {
            anyhow::ensure!(owner_exact, "E007 ordinary projection lacks the scalar owner's exact byte course");
            let post = post.ok_or_else(|| anyhow::anyhow!("E007 ordinary scalar projection was refused"))?;
            Ok(crate::structural_write_authority::transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
                source, source.xml(), &post, canonical, &rebound, Some(omissions))?)
        })();
        probe.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        let carried = carried?;
        crate::structural_write_authority::approve_phrase_occurrence_course(&carried, intents)
    }

    pub(in crate::pipeline) fn continue_phrase_course(&mut self,
        course: &crate::structural_write_authority::ApprovedPhraseOccurrenceCourse,
    ) -> anyhow::Result<()> {
        let continuation = self.committed_occurrence_continuation.as_ref()
            .ok_or_else(|| anyhow::anyhow!("E007 phrase continuation lacks committed original occurrences"))?;
        self.phrase_occurrence_course = Some(course.carry(continuation)?);
        Ok(())
    }

    pub(in crate::pipeline) fn adopt_layout_and_phrase_course(&self, state: &mut RenderSovereignState,
        directives: &[crate::structural_write_authority::HorizontalLayoutMaterializationDirective],
        intents: &[crate::audit_only::phrase_boundary_evidence::PhraseNotationIntentProjection],
    ) -> anyhow::Result<usize> {
        crate::structural_write_authority::adopt_final_layout_phrase_projection_with_course(
            state, directives, intents, self.phrase_occurrence_course.as_ref())
    }

    pub(super) fn pipeline_line(&self) -> &str {
        &self.pipeline_line
    }

    fn horizontal_layout_pipeline_line(&self) -> String {
        // Layout renews the byte witness; it does not replace the actual writer
        // outcome that the final report and corpus manifest must still observe.
        format!("{}\nFinalPdfSourceCommitHorizontalLayoutPreservation: prior committed FinalNotation state preserved through authorized HorizontalLayout; persisted readback rebound; FinalChord occurrence continuation verified; full FinalNotation recommit skipped.", self.pipeline_line)
    }

    pub(super) const fn closed(&self) -> bool {
        self.closed
    }

    pub(in crate::pipeline) fn committed_xml(&self) -> Option<&str> {
        self.committed_xml.as_deref()
    }

    pub(in crate::pipeline) fn committed_voice_plan(
        &self,
    ) -> Option<&crate::structural_write_authority::FrozenSerializedVoicePlan> {
        self.committed_voice_plan.as_ref()
    }

    pub(in crate::pipeline) const fn primary_beam_voice_partition_changed_voice_elements(
        &self,
    ) -> usize {
        self.primary_beam_voice_partition_changed_voice_elements
    }
}

pub(in crate::pipeline) fn bind_committed_master_source_before_renderer_authorized(
    input_musicxml: &Path,
) -> FinalPdfSourceCommit {
    let committed_xml = std::fs::read_to_string(input_musicxml).ok();
    FinalPdfSourceCommit {
        pipeline_line: "FinalPdfSourceCommit: committed Master source bound directly; Render FinalNotation materialization not applicable.".to_string(),
        closed: committed_xml.is_some(),
        committed_xml,
        committed_occurrence_continuation: None,
        committed_voice_plan: None,
        primary_beam_voice_partition_changed_voice_elements: 0, phrase_occurrence_course: None,
    }
}

/// FinalNotation has one source-bound authorization owner. The disposition
/// chooses which existing sink consumes its independently prepared receipt chain.
/// Shadow writes never adopt into Main or advance its historical #58 basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::pipeline) enum FinalPdfSourceDisposition {
    Live,
    Shadow,
}

pub(in crate::pipeline) struct FinalPdfSourcePlans<'a> {
    pub(in crate::pipeline) chord: &'a crate::structural_write_authority::CanonicalFinalChordStructuralPlan,
    pub(in crate::pipeline) voice: &'a crate::structural_write_authority::FrozenSerializedVoicePlan,
}

pub(in crate::pipeline) fn finalize_pdf_source_before_renderer_authorized(
    render_state: &mut RenderSovereignState,
    input_musicxml: &Path,
    disposition: FinalPdfSourceDisposition,
    plans: FinalPdfSourcePlans<'_>,
    render_pruned_final_chord_omissions: &std::collections::BTreeSet<
        crate::structural_write_authority::RenderPrunedFinalChordOmissionKey,
    >,
    occurrence_continuation: &crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation,
    primary_beam_voice_partition_plan:
        Option<&crate::structural_write_authority::PrimaryBeamVoicePartitionBoundPlan>,
) -> std::io::Result<FinalPdfSourceCommit> {
    let FinalPdfSourcePlans { chord: final_chord_structural_plan, voice: frozen_serialized_voice_plan } = plans;
    if primary_beam_voice_partition_plan.is_some_and(|plan|!plan.permits_pdf_finalization()) {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied,
            "historical E007 metric decision course cannot authorize PDF finalization"));
    }

    if disposition == FinalPdfSourceDisposition::Live
        && input_musicxml != render_state.artifact_path()
    {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "PDF path differs from Main artifact"));
    }
    let root = render_state.require_state_root("FinalPdfSourceCommit");
    if occurrence_continuation.xml() != root.document() {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "stale PDF continuation"));
    }
    let observed = std::fs::read_to_string(render_state.artifact_path()).ok();
    record_render_artifact_read(RenderArtifactReadPurpose::GuardAgainstWitnessedArtifact,
        observed.as_deref(), root.document(), "FinalPdfSourceCommit");
    let closure = super::silent_voice_visible_measure_coverage_render_writer::materialize_silent_voice_visible_measure_coverage_for_pdf_source(
        input_musicxml, true, final_chord_structural_plan, frozen_serialized_voice_plan,
        render_pruned_final_chord_omissions, occurrence_continuation, primary_beam_voice_partition_plan);
    let super::silent_voice_visible_measure_coverage_render_writer::FinalRenderSourceClosure { candidate_xml, receipts, finish } = closure;
    let (candidate, finish) = match (candidate_xml, finish) {
        (Some(candidate), Some(finish)) => (candidate, finish),
        _ => {
            receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
            count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "PDF preparation did not close"));
        }
    };
    let a = match disposition {
        FinalPdfSourceDisposition::Shadow => {
            render_state.materialize_shadow_final_notation_checkpoint(
                root, &candidate, receipts, input_musicxml,
            )?
        }
        FinalPdfSourceDisposition::Live => {
            if receipts.is_empty() && candidate == root.document() {
                receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
                root.document().to_string()
            } else {
                render_state.adopt_final_notation_checkpoint(root, &candidate, receipts)?;
                render_state.require_document("FinalPdf witnessed post").to_string()
            }
        }
    };
    let snapshot = finish(&a);
    if !snapshot.closed || snapshot.committed_xml.as_deref() != Some(a.as_str()) {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, snapshot.pipeline_line));
    }
    Ok(FinalPdfSourceCommit {
        pipeline_line: snapshot.pipeline_line, closed: snapshot.closed,
        committed_xml: Some(a),
        committed_occurrence_continuation: snapshot.committed_occurrence_continuation,
        committed_voice_plan: snapshot.committed_voice_plan,
        primary_beam_voice_partition_changed_voice_elements: snapshot.primary_beam_voice_partition_changed_voice_elements,
        phrase_occurrence_course: None,
    })
}

/// The existing E007 owner already witnessed the branch layout post. Close
/// that branch receipt and transport its bound bytes, never another file read.
pub(in crate::pipeline) fn discard_shadow_layout_receipt_post(
    pre: &str,
    receipt: Option<crate::master_state_identity::PreservingTransformationReceipt>,
) -> std::io::Result<String> {
    let Some(receipt) = receipt else { return Ok(pre.to_string()); };
    let (receipt, post) = receipt.split_realized_post_document();
    let Some(post) = post else {
        let mut trial = crate::master_state_identity::RenderTransitionReceipts::new();
        trial.push_preserving(Some(receipt), pre);
        trial.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Shadow E007 has no receipt-bound post"));
    };
    let binding = receipt.validate_preserving_transition::<crate::structural_write_authority::FinalLayoutPhrasePreservingMint>(pre, &post);
    let mut trial = crate::master_state_identity::RenderTransitionReceipts::new();
    trial.push_preserving(Some(receipt), &post);
    trial.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    if let Err(error) = binding {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,
            format!("Shadow E007 receipt differs from branch pre/post: {error}")));
    }
    Ok(post)
}

// Preserve the already-closed FinalNotation proof only across the existing
// HorizontalLayout representation boundary. That authority is source-bound,
// validates the complete temporal materialization before assignment, persists
// it with exact byte readback, and is limited to measure-width / print-break
// representation plus the generated visible Ottava continue recues at a layout
// restart. It does not own note/rest/beam/clef/tie/accent mutation. Therefore
// the previously proven FinalPdfSource closure remains valid; only the persisted
// bytes and the FinalChord occurrence representation need to be rebound here.
pub(in crate::pipeline) fn preserve_pdf_source_commit_through_horizontal_layout_authorized(
    input_musicxml: &Path,
    source_commit: &FinalPdfSourceCommit,
    observed_source_xml: &str,
    witnessed_layout_post: &str,
    final_chord_structural_plan:
        &crate::structural_write_authority::CanonicalFinalChordStructuralPlan,
    frozen_serialized_voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
    render_pruned_final_chord_omissions: &std::collections::BTreeSet<
        crate::structural_write_authority::RenderPrunedFinalChordOmissionKey,
    >,
) -> std::io::Result<FinalPdfSourceCommit> {
    if !source_commit.closed {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "HorizontalLayout cannot preserve an open FinalPdfSourceCommit; {}",
                source_commit.pipeline_line()
            ),
        ));
    }
    if source_commit.committed_xml.as_deref() != Some(observed_source_xml) {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "HorizontalLayout FinalPdfSourceCommit source binding is not byte-exact",
        ));
    }
    let observed_layout = std::fs::read_to_string(input_musicxml);
    record_render_artifact_read(RenderArtifactReadPurpose::WitnessAgainstSovereignState,
        observed_layout.as_ref().ok().map(String::as_str), witnessed_layout_post,
        "FinalPdfSourceCommit HorizontalLayout witnessed post");
    observed_layout?;
    let persisted_layout_xml = witnessed_layout_post.to_string();
    let source_occurrence_continuation = source_commit
        .committed_occurrence_continuation
        .as_ref()
        .ok_or_else(|| {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "HorizontalLayout FinalPdfSourceCommit preservation requires the committed FinalChord occurrence continuation",
            )
        })?;
    let committed_occurrence_continuation =
        crate::structural_write_authority::
            transport_final_pdf_source_occurrence_continuation_through_horizontal_layout(
                source_occurrence_continuation,
                observed_source_xml,
                &persisted_layout_xml,
                final_chord_structural_plan,
                frozen_serialized_voice_plan,
                Some(render_pruned_final_chord_omissions),
            ).inspect_err(|_error| {
                count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures); })?;
    if committed_occurrence_continuation.xml() != persisted_layout_xml {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "HorizontalLayout FinalPdfSourceCommit continuation is not bound to the persisted layout bytes",
        ));
    }

    Ok(FinalPdfSourceCommit {
        pipeline_line: source_commit.horizontal_layout_pipeline_line(),
        closed: true,
        committed_xml: Some(persisted_layout_xml),
        committed_occurrence_continuation: Some(committed_occurrence_continuation),
        committed_voice_plan: source_commit.committed_voice_plan.clone(),
        primary_beam_voice_partition_changed_voice_elements:
            source_commit.primary_beam_voice_partition_changed_voice_elements,
        phrase_occurrence_course: None,
    })
}

// Read-only candidate probes use the same canonical writer authorities and close
// their receipts locally; they never enter the accepted PDF receipt chain.
pub(super) fn pdf_exact_probe(
    pre: &str,
    replacements: Vec<(usize, usize, String)>,
) -> Option<String> {
    let (result, receipt) =
        crate::structural_write_authority::apply_strict_staff_gap_materialization_authorized(
            pre,
            replacements,
        );
    let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
    probe.push_exact(receipt);
    probe.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    result
}

#[cfg(test)]
pub(super) fn primary_clef_probe(xml: &mut String) -> Result<(), &'static str> {
    let (result, receipt) =
        crate::structural_write_authority::apply_rustig_clef_course_to_buffer(xml);
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_preserving(receipt, xml);
    receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    result
}

#[cfg(test)]
mod sovereign_pdf_post_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_PDF_WITNESS: AtomicUsize = AtomicUsize::new(0);
    include!("../../tests/support/render_authority/pdf_writer_report_layout_transport.rs");

    #[test]
    fn later_readback_b_cannot_replace_explicit_sovereign_post_a() {
        let path = std::env::temp_dir().join(format!("etuschlopf-pdf-post-{}-{}.render.musicxml",
            std::process::id(), NEXT_PDF_WITNESS.fetch_add(1, Ordering::Relaxed)));
        std::fs::write(&path, "B").unwrap();
        let source = FinalPdfSourceCommit {
            pipeline_line: String::new(), closed: true, committed_xml: Some("source".to_string()),
            committed_occurrence_continuation: None, committed_voice_plan: None,
            primary_beam_voice_partition_changed_voice_elements: 0, phrase_occurrence_course: None,
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            preserve_pdf_source_commit_through_horizontal_layout_authorized(
                &path, &source, "source", "A",
                &crate::structural_write_authority::CanonicalFinalChordStructuralPlan::default(),
                &crate::structural_write_authority::FrozenSerializedVoicePlan::default(),
                &std::collections::BTreeSet::new())
        }));
        assert!(result.is_err(), "B must fail the explicit A witness before continuation rebinding");
        assert_eq!(source.committed_xml(), Some("source"));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn shadow_layout_noop_transports_pre_without_file_authority() {
        let post = discard_shadow_layout_receipt_post("A", None).unwrap();
        assert_eq!(post, "A");
    }
    fn real_e007_layout_trial(pre: &str) -> (std::path::PathBuf, Option<crate::master_state_identity::PreservingTransformationReceipt>) {
        let path = std::env::temp_dir().join(format!("etuschlopf-shadow-e007-{}-{}.render.musicxml",
            std::process::id(), NEXT_PDF_WITNESS.fetch_add(1, Ordering::Relaxed)));
        std::fs::write(&path, pre).unwrap();
        let directives = vec![crate::structural_write_authority::HorizontalLayoutMaterializationDirective {
            explicit_width: Some("900".to_string()),
            system_break: crate::structural_write_authority::HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }; 2];
        let (result, receipt) = crate::structural_write_authority::materialize_final_render_notation_with_phrase_projection_authorized(
            &path, &directives, &[]);
        result.expect("actual E007 typed layout transaction");
        assert!(receipt.is_some(), "width insertion must mint real file-bound E007 PR");
        (path, receipt)
    }

    fn shadow_e007_source(ordinal: usize) -> String {
        let following = ordinal + 1;
        format!(r#"<?xml version="1.0"?><score-partwise version="3.1"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="{ordinal}"><attributes><divisions>4</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff><notations><articulations><tenuto/></articulations></notations></note></measure><measure number="{following}"><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>"#)
    }

    #[test]
    fn shadow_e007_real_receipt_binds_exact_branch_pre() {
        for ordinal in [9usize, 83] {
            let pre = shadow_e007_source(ordinal);
            let (path, receipt) = real_e007_layout_trial(&pre);
            let post = discard_shadow_layout_receipt_post(&pre, receipt).unwrap();
            assert_ne!(pre, post);
            assert_eq!(post, std::fs::read_to_string(&path).unwrap());
            let _ = std::fs::remove_file(path);
        }
    }

    #[test]
    fn shadow_e007_foreign_file_pre_cannot_bind_main_branch() {
        for ordinal in [9usize, 83] {
            let expected_pre = shadow_e007_source(ordinal);
            let foreign_pre = expected_pre.replace("Piano", "Pianx");
            let (path, receipt) = real_e007_layout_trial(&foreign_pre);
            let result = discard_shadow_layout_receipt_post(&expected_pre, receipt);
            assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData);
            assert!(std::fs::read_to_string(&path).unwrap().contains("Pianx"));
            let _ = std::fs::remove_file(path);
        }
    }

}

