//! Coupled adoption through the existing octave BLOCK #50 and clef E024 owners.
//! Preparation and search have no sink. This boundary alone executes the owners.
use crate::structural_write_authority::{MasterAdoptedState, RenderSovereignState, RustigClefSurfaceDecision};
use crate::arrangement::representation_transport::RepresentationStep;
use crate::master_state_identity::{OwnerReceipts, RenderTransitionReceipts};
use crate::registernotation::reader_policy::PreparedOriginUpdate;
use anyhow::{Result, anyhow, ensure};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct PreparedJointOctaveWrite {
    pub(crate) source: Arc<str>,
    pub(crate) post: String,
    pub(crate) edits: Vec<(usize, usize, String, RepresentationStep)>,
}

impl PreparedJointOctaveWrite {
    pub(crate) fn materialize(&self) -> Result<(String, OwnerReceipts)> {
        let mut document = self.source.to_string();
        let mut receipts = OwnerReceipts::default();
        for (start, end, replacement, journal) in &self.edits {
            let (before, after) = journal.original_documents();
            ensure!(before == document, "C3 octave journal has a stale source");
            let (post, receipt) = crate::structural_write_authority::replace_bound_octave_measure_authorized(
                &document, *start, *end, replacement.clone());
            document = post.ok_or_else(|| anyhow!("Existing octave owner refused its prepared measure"))?;
            ensure!(after == document, "Existing octave owner diverged from its qualified original journal");
            receipts.push_octave_block_with_transport(receipt, Ok(journal.clone()));
        }
        ensure!(document == self.post, "C3 octave owner did not realize the complete preparation");
        Ok((document, receipts))
    }
}

pub(crate) struct PreparedJointReadingCourse<P> {
    pub(crate) master_octaves: PreparedJointOctaveWrite,
    pub(crate) render_octaves: PreparedJointOctaveWrite,
    pub(crate) master_clefs: RustigClefSurfaceDecision,
    pub(crate) render_clefs: RustigClefSurfaceDecision,
    pub(crate) origins: PreparedOriginUpdate,
    pub(crate) proof: P,
}

/// The candidate supplies immutable evidence and access to its sovereign Master.
/// No callback may mint a receipt or materialize XML.
pub(crate) trait JointReadingCandidate {
    type Custody;
    type Proof;
    fn joint_master_state(&mut self) -> &mut MasterAdoptedState;
    fn joint_lower_course_selected(&self, proof: &Self::Proof) -> bool;
    fn prepare_joint_course(&self, custody: &Self::Custody, render: &str)
        -> Result<PreparedJointReadingCourse<Self::Proof>>;
    fn observe_joint_course(&mut self, custody: Self::Custody, proof: Self::Proof,
        master: String, render: String) -> Result<()>;
}

pub(crate) fn apply_joint_reading_course<C: JointReadingCandidate>(
    best: &mut C, render: &mut RenderSovereignState, custody: C::Custody,
) -> Result<()> {
    crate::audit_only::c3_diagnostics::begin();
    let Some(render_source) = render.document().map(str::to_owned) else {
        crate::audit_only::c3_diagnostics::finish("retained","missing Render state before preparation");
        println!("C3JointOwner: UNKNOWN missing Render state; incumbent retained");
        return Ok(());
    };
    let prepared = match best.prepare_joint_course(&custody, &render_source) {
        Ok(prepared) => prepared,
        Err(error) => {
            crate::audit_only::c3_diagnostics::finish("retained",&format!("{error:#}"));
            println!("C3JointOwner: UNKNOWN {error}; complete incumbent retained");
            return Ok(());
        }
    };
    ensure!(best.joint_master_state().document() == prepared.master_octaves.source.as_ref()
        && render_source == prepared.render_octaves.source.as_ref(), "C3 prepared source epochs changed");
    let expected_master = prepared.master_clefs.joint_course_post()
        .ok_or_else(|| anyhow!("Missing prepared Master clef course"))?.to_owned();
    let expected_render = prepared.render_clefs.joint_course_post()
        .ok_or_else(|| anyhow!("Missing prepared Render clef course"))?.to_owned();
    let lower_selected = best.joint_lower_course_selected(&prepared.proof);
    let root = render.require_state_root("C3 existing register and clef owners");
    let (master_post, render_post) = prepared.origins.commit_with(|| {
        let (mut master_post, mut master_receipts) = prepared.master_octaves.materialize()?;
        let (status, receipt) = prepared.master_clefs.apply_prepared_joint_course(&mut master_post);
        status.map_err(|error| anyhow!("Existing Master clef owner failed: {error}"))?;
        master_receipts.push_preserving(receipt);
        let (mut render_post, octave_receipts) = prepared.render_octaves.materialize()?;
        let mut render_receipts = RenderTransitionReceipts::new();
        render_receipts.append_original_owner_chain(octave_receipts)?;
        let (status, receipt) = prepared.render_clefs.apply_prepared_joint_course(&mut render_post);
        status.map_err(|error| anyhow!("Existing Render clef owner failed: {error}"))?;
        render_receipts.push_preserving(receipt, &render_post);
        ensure!(master_post == expected_master && render_post == expected_render,
            "Existing owners diverged from the complete C3 preparations");
        // Fatal Master lineage checks finish on a copy before any Render write.
        let mut master = best.joint_master_state().clone();
        master.adopt_materialized_document(master_post.clone(), master_receipts,
            "C3/BoundExistingRegisterAndClefOwners");
        {
            // The prepared C3 course must already be protected while #58 runs.
            // Drop restores the prior temporary state on success, error or unwind;
            // observe_joint_course remains the sole permanent acceptance owner.
            let _protection = crate::pipeline::protect_joint_reading_checkpoint(lower_selected);
            render.adopt_final_notation_checkpoint(root, &render_post, render_receipts)?;
        }
        *best.joint_master_state() = master;
        Ok((master_post, render_post))
    })?;
    best.observe_joint_course(custody, prepared.proof, master_post, render_post)?;
    if let Some(actual_render) = render.document() {
        crate::pipeline::observe_joint_render_course_checkpoint("C3AdoptionReadback", actual_render);
    }
    Ok(())
}


#[cfg(test)]
mod checkpoint_protection_tests {
    use crate::pipeline::{joint_reading_course_active, protect_joint_reading_checkpoint};

    fn low_lower_staff_in_g() -> String {
        let note="<note><pitch><step>C</step><octave>2</octave></pitch><duration>96</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>";
        let mut xml=String::from("<score-partwise><part id=\"P1\">");
        for bar in 0..8 {
            xml.push_str(&format!("<measure number=\"{}\">",bar+1));
            if bar==0 {xml.push_str("<attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes>");}
            xml.push_str(&note.repeat(4));xml.push_str("</measure>");
        }
        xml.push_str("</part></score-partwise>");xml
    }
    #[test]
    fn original_checkpoint_preserves_prepared_course_while_protected() {
        let xml=low_lower_staff_in_g();
        let path=std::path::Path::new("synthetic-c3-no-sink.musicxml");
        let prepare=||crate::structural_write_authority::orchestrator_artifact_sink_authority::prepare_final_notation_checkpoint(path,&xml);
        let (unprotected,receipt)=prepare();
        assert_ne!(unprotected.unwrap(),xml,"the original planner must be capable of changing this synthetic course");
        assert!(receipt.is_some());
        {
            let _protection=protect_joint_reading_checkpoint(true);
            let (protected,receipt)=prepare();
            assert_eq!(protected.unwrap(),xml,"#58 must retain the prepared course before permanent acceptance");
            assert!(receipt.is_none(),"no invented no-op owner receipt");
        }
        let (after,receipt)=prepare();
        assert_ne!(after.unwrap(),xml,"temporary protection must not leak to a subsequent ordinary checkpoint");
        assert!(receipt.is_some());
    }
    #[test]
    fn checkpoint_protection_restores_on_error_unwind_and_nested_scopes() {
        let before=joint_reading_course_active();
        let aborted=(||->Result<(),()> {
            let _protection=protect_joint_reading_checkpoint(true);
            assert!(joint_reading_course_active());
            {let _nested=protect_joint_reading_checkpoint(false);assert!(joint_reading_course_active());}
            Err(())
        })();
        assert!(aborted.is_err());assert_eq!(joint_reading_course_active(),before);
        let unwound=std::panic::catch_unwind(||{
            let _protection=protect_joint_reading_checkpoint(true);
            assert!(joint_reading_course_active());panic!("synthetic owner abort");
        });
        assert!(unwound.is_err());assert_eq!(joint_reading_course_active(),before);
        {let _not_selected=protect_joint_reading_checkpoint(false);assert_eq!(joint_reading_course_active(),before);}
        assert_eq!(joint_reading_course_active(),before);
    }
}

/// Read-only diagnostic projection through the existing C3 module surface.
pub(crate) fn observe_guard_conflicts(xml:&str)->Result<Vec<String>, &'static str> {
    super::super::rustig_clef_planning::observe_lower_clef_guard_conflicts(xml)
}
