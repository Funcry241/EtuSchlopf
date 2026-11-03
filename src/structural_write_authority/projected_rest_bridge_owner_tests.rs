// Actual E050 tests, owner-local source admitted under canonical SWA.
#[cfg(test)]
mod projected_rest_bridge_owner_tests {
    use crate::structural_write_authority::{CanonicalFinalChordStructuralPlan as Plan, MasterAdoptedState, RenderSovereignState, RenderSinkKind};
    use crate::master_state_identity::{MasterLineage, RenderTransitionReceipts};
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    pub(super) fn score(ordinal: usize, unit: i64, label: &str, copies: usize) -> String {
        let body = |number| format!(
            "<measure number=\"{number}\"><attributes><divisions>{unit}</divisions><time><beats>1</beats><beat-type>4</beat-type></time><staves>2</staves></attributes><note><pitch><step>E</step><octave>5</octave></pitch><duration>{unit}</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><backup><duration>{unit}</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>{short}</duration><voice>1</voice><type>16th</type><staff>2</staff></note><forward><duration>{short}</duration></forward><note><pitch><step>G</step><octave>3</octave></pitch><duration>{short}</duration><voice>1</voice><type>16th</type><staff>2</staff></note><forward><duration>{short}</duration></forward><backup><duration>{unit}</duration></backup><note><rest/><duration>{unit}</duration><voice>{label}</voice><type>quarter</type><staff>2</staff></note></measure>", short=unit/4);
        format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\">{}</part></score-partwise>", (0..copies).map(|index| body(ordinal+index)).collect::<String>())
    }
    pub(super) fn fixture(xml: &str, label: &str, copies: usize) -> (Plan, crate::structural_write_authority::FrozenSerializedVoicePlan, crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation) {
        let labels = (0..copies).flat_map(|_| ["1", label, label]).collect::<Vec<_>>();
        Plan::synthetic_projected_bridge_fixture(xml, &labels).unwrap()
    }
    pub(super) fn state_for(xml: &str) -> (RenderSovereignState, std::path::PathBuf) {
        let n=NEXT.fetch_add(1,Ordering::Relaxed);
        let path=std::env::temp_dir().join(format!("projected-e050-{}-{n}.render.musicxml", std::process::id()));
        let master=MasterAdoptedState::adopt_genesis_state(xml.to_owned(), MasterLineage::lineage_from_genesis(crate::master_state_identity::tests::synthetic_genesis_for_tests(xml)));
        let mut state=RenderSovereignState::begin_project_render(&path,&master);
        let root=state.initial_trial_root(true);
        state.adopt_transition(root,xml,RenderTransitionReceipts::new(),RenderSinkKind::PrunedPdfTrial,
            |path,_pre,post| (std::fs::write(path,post),None)).unwrap();
        (state,path)
    }






}
