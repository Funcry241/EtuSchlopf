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
    /// Shadow-only partial closure: the two primary-clef terms are red (C3 beam/tie
    /// conflict on pre-partition voices); all other closure terms are green.
    /// Never closed, never Live, never releasable.
    shadow_partial: Option<ShadowPartialClosure>,
}

// Minted only by the Shadow disposition after all unrelated checks pass.
// It is bound to the witnessed shadow input, never a general render permit.
#[derive(Debug, Clone)]
struct ShadowPartialClosure { source_path: std::path::PathBuf }

impl FinalPdfSourceCommit {
    pub(super) fn permits_shadow_renderer(&self, input: &Path) -> bool {
        !self.closed && self.shadow_partial.as_ref().is_some_and(|proof| proof.source_path == input)
    }
    pub(in crate::pipeline) fn continue_phrase_course(&mut self,
        course: &crate::structural_write_authority::ApprovedPhraseOccurrenceCourse,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(self.shadow_partial.is_none(), "E007 phrase continuation cannot target a shadow-partial commit");
        let continuation = self.committed_occurrence_continuation.as_ref()
            .ok_or_else(|| anyhow::anyhow!("E007 phrase continuation lacks committed original occurrences"))?;
        self.phrase_occurrence_course = Some(course.carry(continuation)?);
        Ok(())
    }

    pub(in crate::pipeline) fn adopt_layout_and_phrase_course(&self, state: &mut RenderSovereignState,
        directives: &[crate::structural_write_authority::HorizontalLayoutMaterializationDirective],
        intents: &[crate::audit_only::phrase_boundary_evidence::PhraseNotationIntentProjection],
    ) -> anyhow::Result<usize> {
        anyhow::ensure!(self.shadow_partial.is_none(), "Shadow partial closure cannot adopt into Live layout");
        crate::structural_write_authority::adopt_final_layout_phrase_projection_with_course(
            state, directives, intents, self.phrase_occurrence_course.as_ref())
    }

    pub(super) fn pipeline_line(&self) -> &str {
        &self.pipeline_line
    }

    fn horizontal_layout_pipeline_line(&self) -> String {
        // Layout renews the byte witness; it does not replace the actual writer
        // outcome that the final report and corpus manifest must still observe.
        if self.shadow_partial.is_some() {
            return format!("{}\nFinalPdfSourceShadowHorizontalLayoutPreservation: partial=true; FullClosure=false; original occurrence course and exact layout bytes rebound; ReleaseEffect=none.", self.pipeline_line);
        }
        format!("{}\nFinalPdfSourceCommitHorizontalLayoutPreservation: prior committed FinalNotation state preserved through authorized HorizontalLayout; persisted readback rebound; FinalChord occurrence continuation verified; full FinalNotation recommit skipped.", self.pipeline_line)
    }

    pub(super) const fn closed(&self) -> bool {
        self.closed && self.shadow_partial.is_none()
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
        primary_beam_voice_partition_changed_voice_elements: 0, phrase_occurrence_course: None, shadow_partial: None,
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
    let _diagnostics = crate::audit_only::c3_diagnostics::shadow_scope(disposition == FinalPdfSourceDisposition::Shadow);
    let FinalPdfSourcePlans { chord: final_chord_structural_plan, voice: frozen_serialized_voice_plan } = plans;
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
    if shadow_snapshot_may_continue(disposition, &snapshot, &a) {
        let line = format!("FinalPdfSourceShadowPartialClosure: Project={:?}; Disposition=shadow; partial=true; PrimaryClefProjectionGreen nein; CommittedPrimaryClefReentryChanges {}; FailureKind=beam_or_tie_conflict; OtherClosureTerms green; Authority=plan_and_phrase_derivation_only; ReleaseEffect=none.", input_musicxml, snapshot.committed_primary_clef_reentry_changes);
        eprintln!("{line}");
        crate::audit_only::c3_diagnostics::record("Render","all","shadow_partial_closure",&line);
        return Ok(FinalPdfSourceCommit {
            pipeline_line: snapshot.pipeline_line, closed: false,
            committed_xml: Some(a),
            committed_occurrence_continuation: snapshot.committed_occurrence_continuation,
            committed_voice_plan: None,
            primary_beam_voice_partition_changed_voice_elements: 0,
            phrase_occurrence_course: None,
            shadow_partial: Some(ShadowPartialClosure { source_path: input_musicxml.to_path_buf() }),
        });
    }
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
        shadow_partial: None,
    })
}

fn shadow_snapshot_may_continue(disposition: FinalPdfSourceDisposition,
    snapshot: &super::silent_voice_visible_measure_coverage_render_writer::FinalRenderSourceSnapshot,
    witnessed: &str) -> bool {
    disposition == FinalPdfSourceDisposition::Shadow
        && crate::pipeline::joint_reading_course_active()
        && !snapshot.closed && snapshot.shadow_primary_clef_conflict_only
        && snapshot.committed_occurrence_continuation.as_ref().is_some_and(|course| course.xml() == witnessed)
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
    if source_commit.shadow_partial.as_ref().is_some_and(|proof| proof.source_path != input_musicxml) {
        count_render_closure_failure(RenderClosureMetric::CandidateCoverageFailures);
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Shadow layout path differs from its original partial closure"));
    }
    if !source_commit.closed && source_commit.shadow_partial.is_none() {
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
        closed: source_commit.shadow_partial.is_none(),
        committed_xml: Some(persisted_layout_xml),
        committed_occurrence_continuation: Some(committed_occurrence_continuation),
        committed_voice_plan: source_commit.committed_voice_plan.clone(),
        primary_beam_voice_partition_changed_voice_elements:
            source_commit.primary_beam_voice_partition_changed_voice_elements,
        phrase_occurrence_course: None,
        shadow_partial: source_commit.shadow_partial.clone(),
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
            primary_beam_voice_partition_changed_voice_elements: 0, phrase_occurrence_course: None, shadow_partial: None,
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


#[cfg(test)]
mod shadow_partial_tests {
    use super::*;
    #[test]
    fn only_active_shadow_with_exact_occurrence_witness_can_continue() {
        for ordinal in [17usize,83] {
            let input=crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe",&ordinal.to_string());
            let (_,_,course,_)=crate::structural_write_authority::CanonicalFinalChordStructuralPlan::original_beam_alignment_test_fixture(&input);
            let xml=course.xml().to_string();
            let mut snapshot=super::super::silent_voice_visible_measure_coverage_render_writer::FinalRenderSourceSnapshot {
                pipeline_line:String::new(),closed:false,committed_xml:None,
                committed_occurrence_continuation:Some(course),committed_voice_plan:None,
                primary_beam_voice_partition_changed_voice_elements:0,
                shadow_primary_clef_conflict_only:true,committed_primary_clef_reentry_changes:usize::MAX,
            };
            let _protection=crate::pipeline::protect_joint_reading_checkpoint(true);
            assert!(shadow_snapshot_may_continue(FinalPdfSourceDisposition::Shadow,&snapshot,&xml));
            assert!(!shadow_snapshot_may_continue(FinalPdfSourceDisposition::Live,&snapshot,&xml));
            assert!(!shadow_snapshot_may_continue(FinalPdfSourceDisposition::Shadow,&snapshot,"stale"));
            snapshot.closed=true;
            assert!(!shadow_snapshot_may_continue(FinalPdfSourceDisposition::Shadow,&snapshot,&xml));
            snapshot.closed=false;snapshot.shadow_primary_clef_conflict_only=false;
            assert!(!shadow_snapshot_may_continue(FinalPdfSourceDisposition::Shadow,&snapshot,&xml));
            snapshot.shadow_primary_clef_conflict_only=true;snapshot.committed_occurrence_continuation=None;
            assert!(!shadow_snapshot_may_continue(FinalPdfSourceDisposition::Shadow,&snapshot,&xml));
        }
    }
    #[test]
    fn partial_proof_is_bound_to_shadow_input_and_never_full_closure() {
        let input=Path::new("synthetic-shadow.xml");
        let mut commit=FinalPdfSourceCommit {
            pipeline_line:"PrimaryClefProjectionGreen nein".into(),closed:false,committed_xml:Some("A".into()),
            committed_occurrence_continuation:None,committed_voice_plan:None,
            primary_beam_voice_partition_changed_voice_elements:0,phrase_occurrence_course:None,
            shadow_partial:Some(ShadowPartialClosure {source_path:input.to_path_buf()}),
        };
        assert!(commit.permits_shadow_renderer(input));
        assert!(!commit.permits_shadow_renderer(Path::new("other.xml")));
        assert!(!commit.closed());
        let line=commit.horizontal_layout_pipeline_line();
        assert!(line.contains("partial=true"));assert!(line.contains("PrimaryClefProjectionGreen nein"));
        commit.closed=true;assert!(!commit.closed());assert!(!commit.permits_shadow_renderer(input));
        commit.closed=false;commit.shadow_partial=None;assert!(!commit.permits_shadow_renderer(input));
    }
}
