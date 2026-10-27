// Product realization is distinct from transformation adoption. Consume the
// unchanged causal definition in accepted r1528; never mint Render history.
// Scope: docs/selector_derivation_contract_required_render_product_implementation.md

impl RenderSovereignState {
    /// Close both the transformation lineage and the mandatory XML product.
    /// Existing Render history keeps its original strict artifact witness.
    /// Without history, only the final selected, witnessed Master may supply an
    /// unchanged product representation. This is not a Render Genesis/reset.
    pub(crate) fn close_required_render_product(
        &self,
        master: &MasterAdoptedState,
        source: &crate::source::SourcePlan,
        selected_render: bool,
    ) -> std::io::Result<RenderClosureVerdict> {
        use std::io::{Error, ErrorKind, Write};
        if self.artifact_path() != source.render_musicxml_out {
            return Err(Error::new(ErrorKind::InvalidInput, "RequiredRenderProduct: destination differs from bound Render artifact"));
        }
        if self.document().is_some() {
            return Ok(self.close_render_sovereignty());
        }
        if selected_render {
            return Err(Error::new(ErrorKind::InvalidData, "RequiredRenderProduct: selected Render has no sovereign state"));
        }
        let master_path = std::fs::canonicalize(&source.musicxml_out)?;
        let parent = self.artifact_path().parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
        let target = std::fs::canonicalize(parent)?.join(self.artifact_path().file_name().ok_or_else(||
            Error::new(ErrorKind::InvalidInput, "RequiredRenderProduct: missing destination filename"))?);
        if target == master_path || !std::fs::metadata(&master_path)?.is_file() {
            return Err(Error::new(ErrorKind::InvalidInput, "RequiredRenderProduct: source/destination are not distinct regular artifacts"));
        }
        let expected = master.document();
        if std::fs::read(&master_path)? != expected.as_bytes() {
            return Err(Error::new(ErrorKind::InvalidData, "RequiredRenderProduct: final Master artifact differs from sovereign Master"));
        }
        // Never overwrite/adopt an existing, stale, foreign or symlink target.
        // create_new also closes the absence-check/create race.
        let mut output = std::fs::OpenOptions::new().write(true).create_new(true).open(&target)?;
        output.write_all(expected.as_bytes())?;
        output.sync_all()?;
        drop(output);
        let witnessed = std::fs::read(&target)? == expected.as_bytes()
            && std::fs::read(&master_path)? == expected.as_bytes();
        if !witnessed {
            return Err(Error::new(ErrorKind::InvalidData, "RequiredRenderProduct: final source/product byte witness failed"));
        }
        // State remains explicitly absent. The product is witnessed against
        // the final Master, not adopted into a fictitious Render lineage.
        Ok(self.finish_render_closure(witnessed))
    }
}

#[cfg(test)]
mod required_render_product_tests {
    use super::*;
    use crate::master_state_identity::tests::synthetic_genesis_for_tests;
    use crate::master_state_identity::{MasterLineage, record_render_stage_boundary, RenderStageBoundary as S};
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    const XML: &str = "<score-partwise><part><measure><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration></note></measure></part></score-partwise>";
    pub(super) struct Fixture { pub(super) root: PathBuf, pub(super) master: MasterAdoptedState, pub(super) state: RenderSovereignState, pub(super) source: crate::source::SourcePlan }
    impl Fixture {
        pub(super) fn new() -> Self {
            crate::master_state_identity::reset_render_lineage_measurement();
            let root = std::env::temp_dir().join(format!("required-product-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            std::fs::create_dir(&root).unwrap();
            let source = crate::source::SourcePlan {
                input_label: "Synthetic".to_owned(), expected_midi: root.join("source.mid"), expected_wav: root.join("source.wav"),
                musicxml_out: root.join("final.musicxml"), render_musicxml_out: root.join("final.render.musicxml"),
                pdf_out: root.join("final.pdf"), report_out: root.join("final.result.txt"),
            };
            let master = MasterAdoptedState::adopt_genesis_state(XML.to_owned(), MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(XML)));
            std::fs::write(&source.musicxml_out, XML).unwrap();
            let state = RenderSovereignState::begin_project_render(&source.render_musicxml_out, &master);
            for stage in [S::InitialRender,S::RestArticulation,S::MetricBeam,S::FinalNotation,S::PdfSourceSelection,S::Release] { record_render_stage_boundary(stage); }
            Self {root,master,state,source}
        }
    }
    impl Drop for Fixture { fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.root); } }
    #[test]
    fn required_product_realizes_final_master_without_minting_render_history() {
        let f = Fixture::new();
        assert!(f.state.close_required_render_product(&f.master,&f.source,false).unwrap().closed);
        assert_eq!(std::fs::read_to_string(&f.source.render_musicxml_out).unwrap(),XML);
        assert_eq!(f.state.document(),None);
        assert_eq!(f.master.document(),XML);
        assert!(!f.source.pdf_out.exists());
    }
    #[test]
    fn required_product_refuses_even_identical_or_foreign_existing_artifact() {
        for bytes in [XML,"foreign"] {
            let f=Fixture::new(); std::fs::write(&f.source.render_musicxml_out,bytes).unwrap();
            assert_eq!(f.state.close_required_render_product(&f.master,&f.source,false).unwrap_err().kind(),std::io::ErrorKind::AlreadyExists);
            assert_eq!(std::fs::read_to_string(&f.source.render_musicxml_out).unwrap(),bytes);
        }
    }
    #[test]
    fn required_product_refuses_stale_master_before_creation() {
        let f=Fixture::new();std::fs::write(&f.source.musicxml_out,"stale").unwrap();
        assert!(f.state.close_required_render_product(&f.master,&f.source,false).is_err());
        assert!(!f.source.render_musicxml_out.exists());
    }
    #[test]
    fn required_product_cannot_create_missing_selected_render_or_use_unbound_destination() {
        let f=Fixture::new();assert!(f.state.close_required_render_product(&f.master,&f.source,true).is_err());
        let mut other=f.source.clone();other.render_musicxml_out=f.root.join("foreign.xml");
        assert!(f.state.close_required_render_product(&f.master,&other,false).is_err());
        assert!(!f.source.render_musicxml_out.exists());assert!(!other.render_musicxml_out.exists());
    }
    #[test]
    fn required_product_cannot_alias_master() {
        let f=Fixture::new(); let state=RenderSovereignState::begin_project_render(&f.source.musicxml_out,&f.master);
        let mut synthetic_alias=f.source.clone();synthetic_alias.render_musicxml_out=synthetic_alias.musicxml_out.clone();
        assert!(state.close_required_render_product(&f.master,&synthetic_alias,false).is_err());
        assert_eq!(std::fs::read_to_string(&synthetic_alias.musicxml_out).unwrap(),XML);
    }
}

#[cfg(test)]
mod required_render_existing_history_tests {
    use super::required_render_product_tests::Fixture;
    use super::*;
    fn with_history() -> Fixture {
        let mut f = Fixture::new();
        let root=f.state.initial_trial_root(true);
        let post=root.document().replace("<step>C</step>","<step>D</step>");
        let mut receipts=RenderTransitionReceipts::new();
        receipts.push_exact(Some(crate::master_state_identity::tests::permitted_exact_receipt_for_tests(root.document(),&post)));
        f.state.adopt_transition(root,&post,receipts,RenderSinkKind::PrunedPdfTrial,|path,_pre,post| (std::fs::write(path,post),None)).unwrap();
        f
    }
    #[test]
    fn required_product_keeps_existing_render_bytes_and_history() {
        let f=with_history();let pre=f.state.document().unwrap().to_string();
        assert_ne!(pre,f.master.document());
        assert!(f.state.close_required_render_product(&f.master,&f.source,true).unwrap().closed);
        assert_eq!(f.state.document(),Some(pre.as_str()));
        assert_eq!(std::fs::read_to_string(&f.source.render_musicxml_out).unwrap(),pre);
    }
    #[test]
    fn required_product_never_falls_back_from_missing_or_mismatched_existing_history() {
        for missing in [true,false] {
            let f=with_history();
            if missing { std::fs::remove_file(&f.source.render_musicxml_out).unwrap(); }
            else { std::fs::write(&f.source.render_musicxml_out,"foreign").unwrap(); }
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                f.state.close_required_render_product(&f.master,&f.source,false))).is_err());
            assert!(f.state.document().is_some());
            if missing { assert!(!f.source.render_musicxml_out.exists()); }
            else { assert_eq!(std::fs::read_to_string(&f.source.render_musicxml_out).unwrap(),"foreign"); }
        }
    }
}
