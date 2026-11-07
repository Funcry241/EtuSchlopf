use super::*;
use crate::structural_write_authority::CanonicalFinalChordStructuralPlan;

fn synthetic_bridge_measure(label: &str) -> String {
    format!(r#"<measure number="{label}"><attributes><divisions>96</divisions></attributes>
<note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number="1">begin</beam><beam number="2">begin</beam></note>
<note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number="1">end</beam><beam number="2">end</beam></note>
<note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>
<note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note></measure>"#)
}
fn score(measures: &str) -> String {
    format!("<score-partwise><part id=\"P1\">{measures}</part></score-partwise>")
}
fn apply(xml: &str, labels: &[&str]) -> (String, MetricRestContinuationAudit, Option<PreservingTransformationReceipt>) {
    let guard=CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(xml, labels).unwrap();
    let ((after,audit), receipt, _pending_extension) = realize_metric_rest_continuation(xml,"independent synthetic proposal",std::path::Path::new("synthetic.render.musicxml"),&guard);
    (after,audit,receipt)
}
#[test]
fn split_sealed_lane_withholds_whole_bridge_without_receipt() {
    let xml=score(&synthetic_bridge_measure("1"));let (after,audit,receipt)=apply(&xml,&["primary","primary","secondary"]);
    assert_eq!(after,xml);assert!(!audit.written);assert!(receipt.is_none());assert_eq!(audit.frozen_voice_split_groups,1);
}
#[test]
fn shared_sealed_lane_keeps_existing_hierarchy_and_preserving_receipt() {
    let xml=score(&synthetic_bridge_measure("7"));let (after,audit,receipt)=apply(&xml,&["shared","shared","shared"]);
    assert_ne!(after,xml);assert!(audit.written);assert_eq!(audit.frozen_voice_split_groups,0);
    assert!(receipt.unwrap().validate_preserving_transition::<PartialBeatBeamPreservingMint>(&xml,&after).is_ok());
    assert!(after.contains("<voice>2</voice>"));
    assert_eq!(after.matches("<note>").count(),xml.matches("<note>").count());
}
#[test]
fn independent_duplicate_proposals_have_independent_admission() {
    let xml=score(&(synthetic_bridge_measure("1")+&synthetic_bridge_measure("2")));
    let (after,audit,receipt)=apply(&xml,&["X","X","Y","X","X","X"]);
    assert!(after.contains(&synthetic_bridge_measure("1")));assert!(audit.written);assert_eq!(audit.frozen_voice_split_groups,1);
    assert_eq!(audit.target_rows,3);assert!(receipt.is_some());
    let (_,both,_)=apply(&xml,&["X","X","Y","X","X","Y"]);assert!(!both.written);assert_eq!(both.frozen_voice_split_groups,2);
}
#[test]
fn relocation_renaming_and_label_permutation_preserve_decision() {
    for name in ["9","842"] {
        let prefix_label = "8";
        let preceding=format!("<measure number=\"{prefix_label}\"><attributes><divisions>96</divisions></attributes><note><rest/><duration>384</duration><voice>1</voice><staff>1</staff></note></measure>");
        for prefix in ["",preceding.as_str()] {
            let xml=score(&(prefix.to_owned()+&synthetic_bridge_measure(name)));
            for labels in [["1","1","7"],["lane-b","lane-b","lane-a"]] {
                let (after,audit,_)=apply(&xml,&labels);assert_eq!(after,xml);assert_eq!(audit.frozen_voice_split_groups,1);
            }
        }
    }
}
#[test]
fn default_guard_cannot_enable_existing_writer() {
    let xml=score(&synthetic_bridge_measure("1"));
    let ((after,audit), receipt, _pending_extension) = realize_metric_rest_continuation(&xml,"missing sealed binding",std::path::Path::new("synthetic.render.musicxml"),&CanonicalFinalChordStructuralPlan::default());
    assert_eq!(after,xml);assert!(!audit.written);assert_eq!(audit.frozen_voice_unbound_groups,1);assert!(receipt.is_none());
}
#[test]
fn guard_for_other_input_cannot_enable_writer() {
    let original=score(&synthetic_bridge_measure("1"));let xml=score(&synthetic_bridge_measure("2"));
    let guard=CanonicalFinalChordStructuralPlan::synthetic_bridge_guard_with_labels(&original,&["x","x","x"]).unwrap();
    let ((after,audit), receipt, _pending_extension) = realize_metric_rest_continuation(&xml,"stale binding",std::path::Path::new("synthetic.render.musicxml"),&guard);
    assert_eq!(after,xml);assert_eq!(audit.frozen_voice_unbound_groups,1);assert!(receipt.is_none());
}
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/synthetic_score.rs"));
fn shifted_bridge(pattern: &'static [&'static str]) -> impl Fn(&str) -> String {
    move |xml: &str| {
        let labels = pattern
            .iter()
            .copied()
            .cycle()
            .take(xml.matches("<pitch>").count())
            .collect::<Vec<_>>();
        apply(xml, &labels).0
    }
}
#[test]
fn bridge_decisions_survive_measure_shift() {
    let xml = score(&synthetic_bridge_measure("1"));
    assert_shift_invariant("split", &xml, shifted_bridge(&["lane-a", "lane-a", "lane-b"]));
    assert_shift_invariant("shared", &xml, shifted_bridge(&["lane-a", "lane-a", "lane-a"]));
    assert_ne!(shifted_bridge(&["lane-a", "lane-a", "lane-a"])(&xml), xml);
}
fn bridge_then_late_closure(pattern: &'static [&'static str]) -> impl Fn(&str) -> String {
    move |xml: &str| {
        let mut after = shifted_bridge(pattern)(xml);
        let (_late, receipt) = close_committed_cross_staff_beam_gestures(&mut after);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
        probe.push_preserving(receipt, &after);
        probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        after
    }
}
#[test]
fn late_closure_does_not_restore_a_withheld_bridge() {
    let xml = score(&synthetic_bridge_measure("1"));
    assert_eq!(bridge_then_late_closure(&["lane-a", "lane-a", "lane-b"])(&xml), xml);
    assert_shift_invariant("late_split", &xml, bridge_then_late_closure(&["lane-a", "lane-a", "lane-b"]));
}
#[test]
fn late_closure_keeps_an_admitted_bridge() {
    let xml = score(&synthetic_bridge_measure("1"));
    let early = shifted_bridge(&["lane-a", "lane-a", "lane-a"])(&xml);
    assert_ne!(early, xml);
    assert_eq!(bridge_then_late_closure(&["lane-a", "lane-a", "lane-a"])(&xml), early);
    assert_shift_invariant("late_shared", &xml, bridge_then_late_closure(&["lane-a", "lane-a", "lane-a"]));
}

fn extension_state(xml: &str) -> (crate::structural_write_authority::RenderSovereignState, std::path::PathBuf) {
    use crate::structural_write_authority::{MasterAdoptedState,RenderSovereignState,RenderSinkKind};
    static NEXT:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
    let path=std::env::temp_dir().join(format!("r1481-extension-{}-{}.render.musicxml",std::process::id(),NEXT.fetch_add(1,std::sync::atomic::Ordering::Relaxed)));
    let master=MasterAdoptedState::adopt_genesis_state(xml.to_owned(),crate::master_state_identity::MasterLineage::lineage_from_genesis(crate::master_state_identity::tests::synthetic_genesis_for_tests(xml)));
    let mut state=RenderSovereignState::begin_project_render(&path,&master);
    let root=state.initial_trial_root(true);
    state.adopt_transition(root,xml,crate::master_state_identity::RenderTransitionReceipts::new(),RenderSinkKind::PrunedPdfTrial,
        |path,_pre,post|(std::fs::write(path,post),None)).unwrap();
    (state,path)
}
fn adopted_extension(input: &str) -> (String,String,CanonicalFinalChordStructuralPlan) {
    let (pre,mut guard)=CanonicalFinalChordStructuralPlan::synthetic_adopted_extension_guard(input);
    let (mut state,path)=extension_state(&pre);
    guard.bind_rest_bridge_main_state(&state);
    let ((post,audit),receipt,pending)=realize_metric_rest_continuation(&pre,"synthetic authored extension",&path,&guard);
    assert!(audit.written);assert!(pending.is_some());
    let root=state.require_state_root("synthetic extension adoption");
    
            state.adopt_metric_rest_continuation(root, &post, receipt.unwrap()).unwrap();
    guard.adopt_rest_bridge_extension_output(pending,&state).unwrap();
    std::fs::remove_file(path).unwrap();
    (pre,post,guard)
}
fn withdraw_extension(guard:&CanonicalFinalChordStructuralPlan,labels:&[&str])->String {
    let (bound,source)=guard.synthetic_extension_withdrawal(labels);
    let ((post,_audit),receipt,pending)=realize_metric_rest_continuation(source.xml(),"synthetic carried withdrawal",std::path::Path::new("synthetic.xml"),&bound);
    assert!(pending.is_none());
    if post!=source.xml() { assert_eq!(source.through_rest_bridge_withdrawal(&post,&bound,receipt.as_ref().unwrap()).unwrap().xml(),post); }
    post
}
#[test]
fn extension_pending_proposal_cannot_be_adopted_before_main_write() {
    let (pre,mut guard)=CanonicalFinalChordStructuralPlan::synthetic_adopted_extension_guard(&score(&synthetic_bridge_measure("71")));
    let (state,path)=extension_state(&pre);guard.bind_rest_bridge_main_state(&state);
    let ((_,audit),_,pending)=realize_metric_rest_continuation(&pre,"unadopted",&path,&guard);
    assert!(audit.written);assert!(pending.is_some());assert!(guard.adopt_rest_bridge_extension_output(pending,&state).is_err());
    std::fs::remove_file(path).unwrap();
}
#[test]
fn extension_withdrawal_restores_original_pair_without_voice_changes() {
    let (pre,post,guard)=adopted_extension(&score(&synthetic_bridge_measure("71")));
    let after=withdraw_extension(&guard,&["keep","keep","other"]);
    assert_ne!(post,after);
    let beam_image=|xml:&str|roxmltree::Document::parse(xml).unwrap().descendants().filter(|n|n.has_tag_name("beam")).map(|n|(n.attribute("number").unwrap().to_owned(),n.text().unwrap().to_owned())).collect::<Vec<_>>();
    assert_eq!(beam_image(&after),beam_image(&pre));
    let without_beams=|xml:&str| { let d=roxmltree::Document::parse(xml).unwrap();let mut text=xml.to_owned();for n in d.descendants().filter(|n|n.has_tag_name("beam")).collect::<Vec<_>>().into_iter().rev(){text.replace_range(n.range(),"");}text.split_whitespace().collect::<String>() };
    assert_eq!(without_beams(&pre),without_beams(&after));
    assert_eq!(withdraw_extension(&guard,&["one","one","one"]),post);
    assert_eq!(withdraw_extension(&guard,&["first","second","third"]),post);
}
#[test]
fn extension_decisions_are_local_under_duplication_and_relocation() {
    for name in ["21","981"] {
        let input=score(&(synthetic_bridge_measure(name)+&synthetic_bridge_measure("993")));
        let (_,post,guard)=adopted_extension(&input);
        let both=withdraw_extension(&guard,&["a","a","b","c","c","d"]);
        let one=withdraw_extension(&guard,&["a","a","b","c","c","c"]);
        assert_eq!(post.matches("<beam ").count()-both.matches("<beam ").count(),4);
        assert_eq!(post.matches("<beam ").count()-one.matches("<beam ").count(),2);
    }
}
#[test]
fn extension_relevant_drift_retires_evidence_without_resurrection() {
    for replacement in ["<duration>25</duration>","<duration>23</duration>"] {
        let (_,post,mut guard)=adopted_extension(&score(&synthetic_bridge_measure("37")));
        let drift=post.replacen("<duration>24</duration>",replacement,1);
        guard.continue_rest_bridge_extensions(&post,&drift).unwrap();
        guard.continue_rest_bridge_extensions(&drift,&post).unwrap();
        assert_eq!(withdraw_extension(&guard,&["a","a","b"]),post);
    }
}
#[test]
fn extension_skipped_owner_transition_is_rejected() {
    let (_,post,mut guard)=adopted_extension(&score(&synthetic_bridge_measure("37")));
    assert!(guard.continue_rest_bridge_extensions("foreign prestate",&post).is_err());
}
#[test]
fn extension_pending_evidence_rejects_foreign_main_and_source_instance() {
    let input=score(&synthetic_bridge_measure("87"));
    for foreign_source in [false,true] {
        let (pre,mut guard)=CanonicalFinalChordStructuralPlan::synthetic_adopted_extension_guard(&input);
        let (mut state,path)=extension_state(&pre);guard.bind_rest_bridge_main_state(&state);
        let ((post,audit),receipt,pending)=realize_metric_rest_continuation(&pre,"foreign custody",&path,&guard);
        assert!(audit.written);assert!(pending.is_some());
        if foreign_source {
            let root=state.require_state_root("authentic adoption before foreign receiver");
            
            state.adopt_metric_rest_continuation(root, &post, receipt.unwrap()).unwrap();
            let (_,mut foreign)=CanonicalFinalChordStructuralPlan::synthetic_adopted_extension_guard(&input);
            assert!(foreign.adopt_rest_bridge_extension_output(pending,&state).is_err());
        } else {
            let (foreign,foreign_path)=extension_state(&post);
            assert!(guard.adopt_rest_bridge_extension_output(pending,&foreign).is_err());
            std::fs::remove_file(foreign_path).unwrap();
        }
        std::fs::remove_file(path).unwrap();
    }
}
#[test]
fn extension_withdrawal_survives_all_synthetic_score_shifts() {
    let xml=score(&synthetic_bridge_measure("73"));
    assert_shift_invariant("adopted_extension_withdrawal",&xml,|input| {
        let (_,_,guard)=adopted_extension(input);
        let labels=["preserved","preserved","separate"].into_iter().cycle().take(input.matches("<pitch>").count()).collect::<Vec<_>>();
        withdraw_extension(&guard,&labels)
    });
}

fn merged_origin_bridge_measure(label:&str)->String {
    fn note(step:&str,octave:u8,duration:u16,kind:&str,ties:&str,chord:bool)->String {
        format!("<note>{}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration>{ties}<voice>2</voice><type>{kind}</type><staff>2</staff></note>",if chord {"<chord/>"} else {""})
    }
    let notes=[note("D",3,24,"16th","",false),note("E",3,48,"eighth","",false),
        note("F",2,24,"16th","<tie type=\"start\"/>",false),note("C",3,24,"16th","<tie type=\"start\"/>",true),
        note("F",2,24,"16th","<tie type=\"stop\"/>",false),note("C",3,24,"16th","<tie type=\"stop\"/>",true),
        note("G",3,24,"16th","",false),
        "<note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>".to_owned(),
        note("B",1,24,"16th","<tie type=\"start\"/>",false),note("B",1,48,"eighth","<tie type=\"stop\"/>",false),note("A",3,48,"eighth","",false)].concat();
    format!("<measure number=\"{label}\"><attributes><divisions>96</divisions><time><beats>3</beats><beat-type>4</beat-type></time></attributes>{notes}<backup><duration>288</duration></backup><note><pitch><step>A</step><octave>5</octave></pitch><duration>288</duration><voice>1</voice><type>half</type><dot/><staff>1</staff></note></measure>")
}
fn apply_actual_alignment(source:&crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation,
    canonical:&CanonicalFinalChordStructuralPlan,plan:&crate::structural_write_authority::PrimaryBeamVoicePartitionBoundPlan)->String {
    let rebound=crate::structural_write_authority::rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(),source,plan).unwrap();
    let edits=rebound.edits().iter().map(|e|(e.voice_text_start(),e.voice_text_end(),e.target_label().to_owned())).collect();
    let (post,receipt)=crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(source.xml(),edits);
    let post=post.unwrap();assert_eq!(receipt.unwrap().realize_post_document(source.xml()).as_deref(),Some(post.as_str()));
    crate::structural_write_authority::transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(source,source.xml(),&post,canonical,&rebound,Some(&std::collections::BTreeSet::new())).unwrap();
    post
}
fn merged_origin_real_chain(input:&str)->String {
    let (pre,mut guard)=CanonicalFinalChordStructuralPlan::synthetic_merged_origin_extension_guard(input);
    let (mut state,path)=extension_state(&pre);guard.bind_rest_bridge_main_state(&state);
    let ((post,audit),receipt,pending)=realize_metric_rest_continuation(&pre,"merged origin real chain",&path,&guard);
    assert!(audit.written);assert!(pending.is_some(),"merged origin must be captured with complete resources");
    guard.synthetic_assert_capture_contract(&pre,&post,receipt.as_ref().unwrap(),pending.as_ref().unwrap());
    let root=state.require_state_root("merged origin extension adoption");
            state.adopt_metric_rest_continuation(root, &post, receipt.unwrap()).unwrap();
    guard.adopt_rest_bridge_extension_output(pending,&state).unwrap();
    let (source,plan)=guard.synthetic_project_extension_for_alignment(&post);
    let baseline=apply_actual_alignment(&source.with_rest_bridge_shadow_diagnostics(),&guard,&plan);
    let bound=guard.bind_rest_bridge_withdrawal(&source,&plan).unwrap();
    let ((withdrawn,audit),receipt,pending)=realize_metric_rest_continuation(source.xml(),"merged origin late withdrawal",&path,&bound);
    assert!(pending.is_none());assert!(audit.written,"actual late assignment must retract extension");
    let carried=source.through_rest_bridge_withdrawal(&withdrawn,&bound,receipt.as_ref().unwrap()).unwrap();
    let result=apply_actual_alignment(&carried,&guard,&plan);
    let without_beams=|xml:&str| {let d=roxmltree::Document::parse(xml).unwrap();let mut s=xml.to_owned();for n in d.descendants().filter(|n|n.has_tag_name("beam")).collect::<Vec<_>>().into_iter().rev(){s.replace_range(n.range(),"");}s.split_whitespace().collect::<String>()};
    assert_eq!(without_beams(&baseline),without_beams(&result),"withdrawal preserves the actual voice assignment and every non-beam field");
    assert_eq!(result.matches("<beam ").count(),pre.matches("<beam ").count());
    std::fs::remove_file(path).unwrap();result
}
#[test]
fn merged_origin_extension_real_projection_and_late_assignment() {
    let xml=score(&merged_origin_bridge_measure("81"));
    assert_shift_invariant("merged_origin_resource_closure",&xml,merged_origin_real_chain);
}

fn shared_resource_bridge_measure(label:&str)->String {
    let head=|step:&str,ties:&str|format!("<note><pitch><step>{step}</step><octave>3</octave></pitch><duration>24</duration>{ties}<voice>2</voice><type>16th</type><staff>2</staff></note>");
    let rest="<note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note>";
    let notes=[head("C",""),head("D",""),rest.to_owned(),head("B","<tie type=\"stop\"/><tie type=\"start\"/>"),
        head("B","<tie type=\"stop\"/><tie type=\"start\"/>"),head("B","<tie type=\"stop\"/>"),rest.to_owned(),head("E","")].concat();
    format!("<measure number=\"{label}\"><attributes><divisions>96</divisions><time><beats>2</beats><beat-type>4</beat-type></time></attributes><forward><duration>48</duration><voice>3</voice><staff>2</staff></forward><note><pitch><step>B</step><octave>3</octave></pitch><duration>24</duration><tie type=\"start\"/><voice>3</voice><type>16th</type><staff>2</staff></note><backup><duration>72</duration></backup>{notes}</measure>")
}
#[test]
fn actual_capture_allows_shared_read_only_resources_and_retires_all_dependents() {
    let xml=score(&shared_resource_bridge_measure("89"));
    assert_shift_invariant("shared_resource_capture",&xml,|input| {
        let (_,post,guard)=adopted_extension(input);
        guard.synthetic_assert_shared_resource_retirement();post
    });
}

// r1484: a bridge split in the sealed plan is written only provisionally, with
// bound evidence, and the bound final assignment decides it.
fn adopted_provisional(input: &str, sealed: &[&str]) -> (String, String, CanonicalFinalChordStructuralPlan) {
    let (pre, mut guard) = CanonicalFinalChordStructuralPlan::synthetic_adopted_extension_guard_with_sealed(input, sealed);
    let (mut state, path) = extension_state(&pre);
    guard.bind_rest_bridge_main_state(&state);
    let ((post, audit), receipt, pending) = realize_metric_rest_continuation(&pre, "provisional extension", &path, &guard);
    assert!(audit.written);
    assert_eq!(audit.frozen_voice_split_groups, 0);
    assert!(pending.is_some(), "provisional bridge needs bound evidence");
    let root = state.require_state_root("provisional extension adoption");
    state.adopt_metric_rest_continuation(root, &post, receipt.unwrap()).unwrap();
    guard.adopt_rest_bridge_extension_output(pending, &state).unwrap();
    assert!(guard.has_provisional_rest_bridges());
    std::fs::remove_file(path).unwrap();
    (pre, post, guard)
}
fn beam_image(xml: &str) -> Vec<(String, String)> {
    roxmltree::Document::parse(xml).unwrap().descendants().filter(|n| n.has_tag_name("beam"))
        .map(|n| (n.attribute("number").unwrap().to_owned(), n.text().unwrap().to_owned())).collect()
}
#[test]
fn provisional_bridge_stays_when_the_final_lane_is_shared() {
    let (pre, post, guard) = adopted_provisional(&score(&synthetic_bridge_measure("71")), &["lane-a", "lane-a", "lane-b"]);
    assert_ne!(beam_image(&pre), beam_image(&post));
    assert_eq!(withdraw_extension(&guard, &["joint", "joint", "joint"]), post);
}
#[test]
fn provisional_bridge_is_withdrawn_when_final_lanes_split_or_are_unknown() {
    let (pre, post, guard) = adopted_provisional(&score(&synthetic_bridge_measure("71")), &["lane-a", "lane-a", "lane-b"]);
    for lanes in [&["joint", "joint", "other"][..], &["joint", "joint"][..]] {
        let after = withdraw_extension(&guard, lanes);
        assert_ne!(after, post);
        assert_eq!(beam_image(&after), beam_image(&pre));
    }
}
#[test]
fn provisional_bridge_without_bound_evidence_is_never_written() {
    let input = score(&synthetic_bridge_measure("71"));
    let (pre, guard) = CanonicalFinalChordStructuralPlan::synthetic_adopted_extension_guard_with_sealed(&input, &["lane-a", "lane-a", "lane-b"]);
    let mut guard = guard.synthetic_without_original_groups();
    let (state, path) = extension_state(&pre);
    guard.bind_rest_bridge_main_state(&state);
    let ((post, audit), receipt, pending) = realize_metric_rest_continuation(&pre, "unevidenced provisional", &path, &guard);
    assert_eq!(post, pre);
    assert!(!audit.written);
    assert!(receipt.is_none());
    assert!(pending.is_none());
    assert_eq!(audit.frozen_voice_split_groups, 1);
    std::fs::remove_file(path).unwrap();
}
#[test]
fn provisional_custody_loss_before_the_decision_fails_visibly() {
    let (_, post, mut guard) = adopted_provisional(&score(&synthetic_bridge_measure("37")), &["lane-a", "lane-a", "lane-b"]);
    let drift = post.replacen("<duration>24</duration>", "<duration>25</duration>", 1);
    guard.continue_rest_bridge_extensions(&post, &drift).unwrap();
    guard.continue_rest_bridge_extensions(&drift, &post).unwrap();
    assert!(guard.synthetic_try_extension_withdrawal(&["joint", "joint", "joint"]).is_err());
}
#[test]
fn provisional_bridge_is_withdrawn_without_a_late_plan() {
    let (pre, _, guard) = adopted_provisional(&score(&synthetic_bridge_measure("71")), &["lane-a", "lane-a", "lane-b"]);
    let (bound, source) = guard.synthetic_provisional_fallback_after_sealed_projection();
    let ((post, audit), receipt, pending) = realize_metric_rest_continuation(source.xml(), "provisional fallback", std::path::Path::new("synthetic.xml"), &bound);
    assert!(pending.is_none());
    assert!(audit.written);
    assert_eq!(beam_image(&post), beam_image(&pre));
    let voices = |xml: &str| roxmltree::Document::parse(xml).unwrap().descendants().filter(|n| n.has_tag_name("voice")).map(|n| n.text().unwrap().to_owned()).collect::<Vec<_>>();
    assert_eq!(voices(&post), voices(source.xml()));
    source.through_rest_bridge_withdrawal(&post, &bound, receipt.as_ref().unwrap()).unwrap();
}
#[test]
fn provisional_decisions_survive_measure_shift() {
    let xml = score(&synthetic_bridge_measure("73"));
    for (lanes, label) in [(["joint", "joint", "joint"], "provisional_kept"), (["joint", "joint", "other"], "provisional_withdrawn")] {
        assert_shift_invariant(label, &xml, |input| {
            let (_, _, guard) = adopted_provisional(input, &["lane-a", "lane-a", "lane-b"]);
            let labels = lanes.into_iter().cycle().take(input.matches("<pitch>").count()).collect::<Vec<_>>();
            withdraw_extension(&guard, &labels)
        });
    }
}
#[test]
fn withheld_split_group_without_beam_change_is_counted_as_unchanged() {
    let xml = score(&synthetic_bridge_measure("41"));
    let (bridged, _, _) = apply(&xml, &["lane-a", "lane-a", "lane-a"]);
    assert_ne!(bridged, xml);
    let (after, audit, receipt) = apply(&bridged, &["lane-a", "lane-a", "lane-b"]);
    assert_eq!(after, bridged);
    assert!(receipt.is_none());
    assert_eq!(audit.frozen_voice_split_groups, 0);
    assert_eq!(audit.frozen_voice_unchanged_split_groups, 1);
    let (_, changing, _) = apply(&xml, &["lane-a", "lane-a", "lane-b"]);
    assert_eq!(changing.frozen_voice_split_groups, 1);
    assert_eq!(changing.frozen_voice_unchanged_split_groups, 0);
}


fn adopted_bridge_alignment_chain_with_sealed(input: &str, sealed: &[&str]) -> String {
    let (_, post, guard) = adopted_provisional(input, sealed);
    let (source, voices, plan) = guard.synthetic_adopted_bridge_alignment(&post, ["7", "11"]);
    assert!(plan.scope_candidates() > 0);
    let bound = guard.bind_rest_bridge_withdrawal(&source, &plan).unwrap();
    let ((kept, audit), receipt, pending) = realize_metric_rest_continuation(
        source.xml(), "synthetic bound bridge alignment", std::path::Path::new("synthetic.xml"), &bound);
    assert!(!audit.written, "the complete admitted connection must survive withdrawal");
    assert!(receipt.is_none() && pending.is_none());
    assert_eq!(kept, source.xml());
    let rebound = crate::structural_write_authority::rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(), &source, &plan).unwrap();
    let result = apply_actual_alignment(&source, &guard, &rebound);
    let carried = crate::structural_write_authority::transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
        &source,source.xml(),&result,&guard,&rebound,Some(&std::collections::BTreeSet::new())).unwrap();
    assert_eq!(beam_image(&result), beam_image(source.xml()));
    assert!(result.contains("<voice>7</voice>"));
    assert!(!result.contains("<voice>11</voice>"));
    guard.synthetic_assert_adopted_bridge_alignment_negatives(&source,&voices,&plan,&carried);
    result
}
fn adopted_bridge_alignment_chain(input: &str) -> String {
    adopted_bridge_alignment_chain_with_sealed(input,&["7","7","11"])
}
#[test]
fn adopted_bridge_alignment_preserves_the_actual_connection() {
    let scope = 173usize;
    let xml = score(&synthetic_bridge_measure(&scope.to_string()));
    assert_shift_invariant("adopted_bridge_atomic_voice_resources", &xml, adopted_bridge_alignment_chain);
}

#[test]
fn adopted_bridge_alignment_carries_the_tied_continuation() {
    let scope=251usize;
    let measure=synthetic_bridge_measure(&scope.to_string())
        .replace("<divisions>96</divisions>","<divisions>96</divisions><time><beats>2</beats><beat-type>4</beat-type></time>")
        .replace("<step>E</step><octave>3</octave></pitch><duration>24</duration>","<step>E</step><octave>3</octave></pitch><duration>24</duration><tie type=\"start\"/>")
        .replace("</measure>","<note><pitch><step>E</step><octave>3</octave></pitch><duration>96</duration><tie type=\"stop\"/><voice>2</voice><type>quarter</type><staff>2</staff></note></measure>");
    assert_shift_invariant("adopted_bridge_tied_atomic_resource",&score(&measure),|input|
        adopted_bridge_alignment_chain_with_sealed(input,&["7","7","11","11"]));
}

fn bridge_with_foreign_tied_beam(scope: usize) -> String {
    synthetic_bridge_measure(&scope.to_string())
        .replace("<divisions>96</divisions>", "<divisions>96</divisions><time><beats>2</beats><beat-type>4</beat-type></time>")
        .replace("<step>E</step><octave>3</octave></pitch><duration>24</duration>", "<step>E</step><octave>3</octave></pitch><duration>24</duration><tie type=\"start\"/>")
        .replace("</measure>", "<note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><tie type=\"stop\"/><voice>2</voice><type>16th</type><staff>2</staff><beam number=\"1\">begin</beam><beam number=\"2\">begin</beam></note><note><pitch><step>F</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number=\"1\">end</beam><beam number=\"2\">end</beam></note><note><rest/><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff></note></measure>")
}

#[test]
fn adopted_bridge_alignment_refuses_a_tied_resource_in_another_original_beam() {
    let scope = 317usize;
    let measure = bridge_with_foreign_tied_beam(scope);
    assert_shift_invariant("foreign_beam_tied_resource_refusal", &score(&measure), |input| {
        let (_, post, guard) = adopted_provisional(input, &["7", "7", "11", "11", "11"]);
        use crate::structural_write_authority::SerializedVoicePresentationRole::{LowerPrimary, LowerSecondary};
        let (source, _voices, plan) = guard.synthetic_adopted_bridge_alignment_with_roles(
            &post, ["7", "11"], &[LowerPrimary, LowerPrimary, LowerSecondary, LowerSecondary]);
        assert_eq!(plan.scope_candidates(), 0, "a tied resource cannot split a different original beam");
        assert!(plan.edits().is_empty());
        source.xml().to_owned()
    });
}

#[test]
fn adopted_bridge_alignment_preserves_a_foreign_beam_already_on_target() {
    let scope = 317usize;
    let measure = bridge_with_foreign_tied_beam(scope);
    assert_shift_invariant("foreign_beam_unchanged_target", &score(&measure), |input| {
        let (_, post, guard) = adopted_provisional(input, &["11", "7", "7", "7", "7"]);
        use crate::structural_write_authority::SerializedVoicePresentationRole::{LowerPrimary, LowerSecondary};
        let (source, _, plan) = guard.synthetic_adopted_bridge_alignment_with_roles(
            &post, ["7", "11"], &[LowerSecondary, LowerPrimary, LowerPrimary, LowerPrimary]);
        assert!(plan.scope_candidates() > 0);
        let rebound = crate::structural_write_authority::rebind_primary_beam_voice_partition_plan_to_current_xml(
            source.xml(), &source, &plan).unwrap();
        let after = apply_actual_alignment(&source, &guard, &rebound);
        crate::structural_write_authority::transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
            &source, source.xml(), &after, &guard, &rebound, Some(&std::collections::BTreeSet::new())).unwrap();
        assert_eq!(beam_image(&after), beam_image(source.xml()));
        assert!(!after.contains("<voice>11</voice>"));
        after
    });
}

#[test]
fn adopted_bridge_alignment_preserves_an_authenticated_auxiliary_head_role() {
    let scope = 419usize;
    assert_shift_invariant("auxiliary_bridge_head_role", &score(&synthetic_bridge_measure(&scope.to_string())), |input| {
        let (_, post, guard) = adopted_provisional(input, &["7", "7", "11"]);
        use crate::structural_write_authority::SerializedVoicePresentationRole::{LowerAuxiliary, LowerPrimary};
        let (source, _voices, plan) = guard.synthetic_adopted_bridge_alignment_with_roles(
            &post, ["7", "11"], &[LowerPrimary, LowerPrimary, LowerAuxiliary]);
        assert!(plan.scope_candidates() > 0);
        let rebound = crate::structural_write_authority::rebind_primary_beam_voice_partition_plan_to_current_xml(source.xml(), &source, &plan).unwrap();
        let after = apply_actual_alignment(&source, &guard, &rebound);
        crate::structural_write_authority::transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
            &source, source.xml(), &after, &guard, &rebound, Some(&std::collections::BTreeSet::new())).unwrap();
        assert_eq!(beam_image(&after), beam_image(source.xml()));
        after
    });
}
