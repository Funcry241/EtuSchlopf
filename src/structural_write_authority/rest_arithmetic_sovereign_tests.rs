// Canonical SWA boundary tests; included in the original private helper scope.
#[cfg(test)]
mod r1230_ra1_sovereign_tests {
    use super::*;
    use crate::master_state_identity::{MasterLineage, RenderDiscardReason, RenderTransitionReceipts};
    use crate::structural_write_authority::{MasterAdoptedState, RenderCheckpointSink, RenderSinkKind, RenderSovereignState};
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    fn path() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("etuschlopf-ra1-{}-{}.render.musicxml", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        let _ = std::fs::remove_file(&path);
        path
    }
    fn wrap_at(ordinal: usize, body: &str) -> String {
        format!("<score-partwise><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>")
    }
    fn wrap(body: &str) -> String {
        wrap_at(NEXT.fetch_add(1, Ordering::Relaxed) + 1, body)
    }
    fn rest(duration: i32, kind: &str) -> String {
        format!("<note><rest/><duration>{duration}</duration><voice>1</voice><type>{kind}</type><staff>2</staff></note>")
    }
    fn balance() -> String {
        balance_at(NEXT.fetch_add(1, Ordering::Relaxed) + 1)
    }
    fn balance_at(ordinal: usize) -> String {
        let r = rest(24, "16th");
        wrap_at(ordinal, &format!("{r}<forward><duration>24</duration></forward><note><pitch><step>C</step><octave>3</octave></pitch><duration>336</duration><voice>1</voice><type>half</type><dot/><dot/><staff>2</staff></note><backup><duration>384</duration></backup><forward><duration>24</duration></forward>{r}<forward><duration>336</duration></forward>"))
    }
    fn carrier() -> String {
        carrier_at(NEXT.fetch_add(1, Ordering::Relaxed) + 1)
    }
    fn carrier_at(ordinal: usize) -> String {
        let chord = |step: &str, duration: i32, kind: &str| format!("<note print-object=\"no\" print-spacing=\"no\"><chord/><pitch><step>{step}</step><octave>6</octave></pitch><duration>{duration}</duration><voice>1</voice><type>{kind}</type><staff>2</staff></note>");
        let notes = ["D", "C", "B", "A", "G", "F", "E"].into_iter().map(|step| format!("<note><pitch><step>{step}</step><octave>6</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>")).collect::<String>();
        wrap_at(ordinal, &format!("{}{}{}{}{}{}{}{}{}{}{}{}", rest(24,"16th"),rest(24,"16th"),rest(48,"eighth"),chord("D",48,"eighth"),rest(24,"16th"),rest(24,"16th"),chord("F",24,"16th"),rest(24,"16th"),rest(24,"16th"),rest(24,"16th"),chord("C",24,"16th"),notes))
    }
    fn lane_at(ordinal: usize) -> String {
        format!("<score-partwise version=\"4.0\"><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward><note><rest/><duration>24</duration><type>16th</type><voice>1</voice><staff>2</staff></note><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><forward><duration>24</duration><voice>1</voice><staff>1</staff></forward><backup><duration>288</duration></backup><forward><duration>264</duration></forward><note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>")
    }
    fn combined() -> String {
        let ordinal = NEXT.fetch_add(2, Ordering::Relaxed) + 1;
        let lane = lane_at(ordinal + 1);
        let measure_start = lane.find("<measure").expect("lane measure");
        let measure_end = lane.find("</part>").expect("lane part");
        balance_at(ordinal).replace("</part></score-partwise>", &format!("{}</part></score-partwise>", &lane[measure_start..measure_end]))
    }
    fn state(xml: &str, path: &Path) -> RenderSovereignState {
        let master = MasterAdoptedState::adopt_genesis_state(xml.to_owned(), MasterLineage::lineage_from_genesis(crate::master_state_identity::tests::synthetic_genesis_for_tests(xml)));
        let mut state = RenderSovereignState::begin_project_render(path, &master);
        let root = state.initial_trial_root(true);
        state.adopt_transition(root, xml, RenderTransitionReceipts::new(), RenderSinkKind::PrunedPdfTrial, |path, _, post| (std::fs::write(path, post), None)).expect("Genesis");
        state
    }
    fn prepare(xml: &str, path: &Path) -> (RenderXmlPianisticRestRepairTrial, String, RenderTransitionReceipts) {
        let (trial, post, receipts) = build_fragmented_rest_chain_transactional_write_trial(xml, "r1230 RA1", path).expect("pure prepare");
        assert!(trial.consolidation_adopted > 0, "real local candidate");
        assert!(!trial.written, "prepare is pure");
        (trial, post.expect("real final candidate"), receipts)
    }
    #[test]
    fn ra1_local_candidates_and_owner_chains_relocate_across_numeric_scopes() {
        for ordinal in [19_usize, 307_usize] {
            let balanced = balance_at(ordinal);
            let candidate = build_rest_consolidation_candidates(&balanced).remove(0);
            assert_eq!(candidate.scope, ordinal);
            let (post, receipts) = apply_layer_balanced_single_rest_consolidation_candidate_xml(
                &balanced, &candidate, None,
            ).expect("relocated balanced owner");
            assert!(post.is_some());
            assert!(!receipts.is_empty());
            receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
            let carrier = carrier_at(ordinal);
            let candidate = build_carrier_preserving_initial_rest_projection_candidates(&carrier).remove(0);
            assert_eq!(candidate.rest.scope, ordinal);
            let (post, receipts) = apply_layer_balanced_single_rest_consolidation_candidate_xml(
                &carrier, &candidate.rest, Some((&candidate.parts, candidate.insertion_pos)),
            ).expect("relocated carrier owner");
            assert!(post.is_some());
            assert!(!receipts.is_empty());
            receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
            let lane = lane_at(ordinal);
            assert_eq!(find_next_measure_bounds(&lane, 0).expect("numeric lane scope").2, ordinal);
            let (_, _, receipts) = build_fragmented_rest_chain_transactional_write_trial(
                &lane, "relocated lane", &path(),
            ).expect("pure relocated lane");
            receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        }
    }
    #[test]
    fn r1230_ra1_balance_positive_and_legacy_c_oracle() {
        let xml = balance(); let candidate = build_rest_consolidation_candidates(&xml).remove(0);
        let (post, receipts) = apply_layer_balanced_single_rest_consolidation_candidate_xml(&xml, &candidate, None).expect("prepare");
        let post = post.expect("balanced candidate");
        let anchor = &candidate.spans[0];
        let legacy_anchor = rest_fragment_with_staff_voice_duration(&anchor.fragment,candidate.total_duration,&anchor.staff,&anchor.voice).expect("legacy anchor");
        let (_, end, scope) = find_measure_bounds_containing_pos(&xml,anchor.start).expect("measure");
        let forward = collect_render_timeline_fragments_in_segment(&xml[..end],0,scope).into_iter().find(|fragment| fragment.is_forward && fragment.start >= anchor.end && fragment.onset == 24).expect("forward");
        let edits = vec![(anchor.start,anchor.end,replace_xml_tag_text(&replace_xml_tag_text(&legacy_anchor,"staff",&anchor.staff),"voice",&anchor.voice)),(forward.start,forward.end,replace_forward_fragment_duration(&forward.fragment,0)),(candidate.spans[1].start,candidate.spans[1].end,render_staff_voice_forward_fragment(24,"2","1"))];
        let (oracle, exact) = crate::structural_write_authority::apply_layer_balanced_rest_consolidation_authorized(&xml,edits);
        let mut oracle_chain = RenderTransitionReceipts::new(); oracle_chain.push_exact(exact); oracle_chain.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(Some(post),oracle); receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    }
    #[test]
    fn r1230_ra1_carrier_positive_overlay_rebase_and_legacy_c_oracle() {
        let xml = carrier(); let candidate = build_carrier_preserving_initial_rest_projection_candidates(&xml).remove(0);
        let (post, receipts) = apply_layer_balanced_single_rest_consolidation_candidate_xml(&xml,&candidate.rest,Some((&candidate.parts,candidate.insertion_pos))).expect("carrier prepare");
        let post = post.expect("real carrier owner"); assert!(carrier_projection_invariants_hold(&xml,&post,&candidate));
        let mut edits = candidate.rest.spans.iter().map(|span| (span.start,span.end,hide_rest_as_technical_carrier(&span.fragment).expect("rest"))).collect::<Vec<_>>();
        let mut overlay = format!("\n      <backup><duration>{}</duration></backup>",candidate.rest.total_duration);
        for duration in &candidate.parts { overlay.push_str(&render_visible_rest_note_fragment(*duration,&candidate.rest.staff,&candidate.rest.voice).expect("visible rest")); }
        edits.push((candidate.insertion_pos,candidate.insertion_pos,overlay));
        let (oracle, receipt) = crate::structural_write_authority::apply_layer_balanced_rest_consolidation_authorized(&xml,edits);
        let mut probe = RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(Some(post),oracle); receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    }
    #[test]
    fn r1230_ra1_carrier_rejects_wrong_overlay_duration() {
        let xml=carrier(); let candidate=build_carrier_preserving_initial_rest_projection_candidates(&xml).remove(0);
        let (post,receipts)=apply_layer_balanced_single_rest_consolidation_candidate_xml(&xml,&candidate.rest,Some((&[24],candidate.insertion_pos))).expect("prepared rejected owner");
        assert!(post.is_none()); receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
    }
    #[test]
    fn r1230_ra1_balance_rejects_missing_forward_locator() {
        let xml=balance(); let candidate=build_rest_consolidation_candidates(&xml).remove(0);
        let bad=xml.replace("<forward><duration>24</duration></forward>","");
        match apply_layer_balanced_single_rest_consolidation_candidate_xml(&bad, &candidate, None) {
            Err(error) => assert!(error.to_string().contains("invalid UTF-8 boundary")),
            Ok((_post, receipts)) => {
                receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
                panic!("stale forward locator must fail before materialization");
            }
        }
    }
    #[test]
    fn r1230_ra1_no_candidate_no_write() {
        let xml=wrap("<note><rest/><duration>384</duration><voice>1</voice><type>whole</type><staff>2</staff></note>"); let path=path();
        let (trial,post,receipts)=build_fragmented_rest_chain_transactional_write_trial(&xml,"no candidate",&path).expect("no candidate");
        assert_eq!(trial.consolidation_adopted,0); assert!(!trial.written); assert!(post.is_none()); assert!(!path.exists()); receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
    }
    #[test]
    fn r1230_ra1_final_guard_rejects_real_sounding_drift() {
        let xml=balance(); let path=path(); let (mut trial,post,receipts)=prepare(&xml,&path);
        assert!(fragmented_rest_chain_final_guard(&trial));
        let drift=post.replace("<step>C</step>","<step>D</step>"); let (events,balanced)=collect_render_xml_note_events(&drift);
        trial.sounding_signature_after=render_visible_sounding_signature(&events); trial.sounding_signature_stable=trial.sounding_signature_before==trial.sounding_signature_after;
        trial.backup_forward_after_ok=balanced; trial.parse_after_ok=parse_musicxml(&drift).is_ok();
        assert!(!fragmented_rest_chain_final_guard(&trial)); receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
    }
    fn rest_commit(state: &mut RenderSovereignState, post:&str, receipts:RenderTransitionReceipts) {
        let root=state.initial_trial_root(false);
        state.adopt_transition(root,post,receipts,RenderSinkKind::FragmentedRestChainTrial,|path,pre,candidate|(crate::structural_write_authority::write_fragmented_rest_chain_trial_authorized(pre,candidate,path),None)).expect("E022 commit");
    }
    #[test]
    fn r1230_ra1_e022_failure_does_not_commit() {
        let xml=balance(); let path=path(); let mut state=state(&xml,&path); let (_,post,receipts)=prepare(&xml,&path); let root=state.initial_trial_root(false);
        let result=state.adopt_transition(root,&post,receipts,RenderSinkKind::FragmentedRestChainTrial,|_,_,_|(Err(std::io::Error::other("E022 injected I/O failure")),None));
        assert!(result.is_err()); assert_eq!(state.document(),Some(xml.as_str())); crate::master_state_identity::reset_render_lineage_measurement(); let _=std::fs::remove_file(path);
    }
    #[test]
    fn r1230_ra1_e022_wrong_readback_does_not_commit() {
        let xml=balance(); let path=path(); let mut state=state(&xml,&path); let (_,post,receipts)=prepare(&xml,&path); let root=state.initial_trial_root(false);
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||state.adopt_transition(root,&post,receipts,RenderSinkKind::FragmentedRestChainTrial,|path,pre,candidate| { let result=crate::structural_write_authority::write_fragmented_rest_chain_trial_authorized(pre,candidate,path); if result.is_ok(){std::fs::write(path,"wrong").expect("sabotage witness");}(result,None)})));
        assert!(result.is_err()); assert_eq!(state.document(),Some(xml.as_str())); crate::master_state_identity::reset_render_lineage_measurement(); let _=std::fs::remove_file(path);
    }
    fn second_adoption(fail_write:bool, bad_readback:bool) {
        let xml=combined(); let path=path(); let mut state=state(&xml,&path); let (_,post,receipts)=prepare(&xml,&path); rest_commit(&mut state,&post,receipts); assert_eq!(state.document(),Some(post.as_str()));
        let root=state.initial_trial_root(false); let ((lane_post,stats),receipt)=human_bound_cursor_lane_owner_transfer_xml_with_receipt(root.document()); assert!(stats.written && lane_post!=post,"real second owner");
        let mut chain=RenderTransitionReceipts::new(); chain.push_preserving(receipt,&lane_post);
        let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||state.adopt_transition(root,&lane_post,chain,RenderSinkKind::Checkpoint(RenderCheckpointSink::CursorLaneRepair),|path,_,candidate| {
            if fail_write {return (Err(std::io::Error::other("checkpoint I/O failure")),None);}
            let result=crate::structural_write_authority::write_cursor_lane_repair_checkpoint_authorized(path,candidate);
            if bad_readback && result.0.is_ok() {std::fs::write(path,"wrong").expect("sabotage witness");} result
        })));
        if bad_readback { assert!(result.is_err()); assert_eq!(state.document(),Some(post.as_str())); }
        else if fail_write {assert!(result.expect("ordinary I/O result").is_err()); assert_eq!(state.document(),Some(post.as_str()));}
        else {assert!(result.expect("no panic").is_ok()); assert_eq!(state.document(),Some(lane_post.as_str())); assert_eq!(std::fs::read_to_string(&path).expect("artifact"),lane_post);}
        crate::master_state_identity::reset_render_lineage_measurement(); let _=std::fs::remove_file(path);
    }
    #[test] fn r1230_ra1_two_real_adoptions() {second_adoption(false,false);}
    #[test] fn r1230_ra1_checkpoint_failure_preserves_first_commit() {second_adoption(true,false);}
    #[test] fn r1230_ra1_checkpoint_wrong_readback_preserves_first_commit() {second_adoption(false,true);}
    #[test]
    fn r1230_ra2_real_height_owner_checkpoint_commits() {
        let xml=balance(); let path=path(); let mut state=state(&xml,&path);
        let root=state.require_state_root("RA2 test");
        let (post,audit,receipts)=build_piano_final_rest_glyph_canonical_height_write(root.document(),"RA2 real owner",&path).expect("prepare");
        assert!(audit.written && post!=xml);
        state.adopt_transition(root,&post,receipts,RenderSinkKind::Checkpoint(RenderCheckpointSink::FinalPianoRestGlyph),|path,_,candidate|crate::structural_write_authority::write_final_piano_rest_glyph_checkpoint_authorized(path,candidate)).expect("checkpoint");
        assert_eq!(state.document(),Some(post.as_str())); assert_eq!(std::fs::read_to_string(&path).expect("readback"),post);
        crate::master_state_identity::reset_render_lineage_measurement(); let _=std::fs::remove_file(path);
    }
    #[test]
    fn r1230_ra2_second_height_prepare_is_noop() {
        let xml=balance(); let path=path();
        let (post,audit,receipts)=build_piano_final_rest_glyph_canonical_height_write(&xml,"first",&path).expect("prepare");
        assert!(audit.written); receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
        let (again,audit,receipts)=build_piano_final_rest_glyph_canonical_height_write(&post,"second",&path).expect("prepare noop");
        assert_eq!(again,post); assert!(!audit.written); assert!(!path.exists()); receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
    }
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/render_authority/r1230_ra3_real_probe_is_never_a_physical_write.rs"));
    

}
