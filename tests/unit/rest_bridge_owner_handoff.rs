use super::*;
use crate::master_state_identity::{MasterLineage, RenderDiscardReason, RenderTransitionReceipts};
use crate::structural_write_authority::{MasterAdoptedState, RenderSinkKind, RenderSovereignState};

fn rest(duration: usize, kind: &str) -> String {
    format!("<note><rest/><duration>{duration}</duration><voice>low</voice><type>{kind}</type><staff>2</staff></note>")
}

fn independent_rest_and_lane(offset: usize, copies: usize) -> String {
    let first = offset + 1;
    let short = rest(24, "16th");
    let mut body = format!("<measure number=\"{first}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{short}<forward><duration>24</duration></forward><note><pitch><step>C</step><octave>3</octave></pitch><duration>336</duration><voice>low</voice><type>half</type><dot/><dot/><staff>2</staff></note><backup><duration>384</duration></backup><forward><duration>24</duration></forward>{short}<forward><duration>336</duration></forward></measure>");
    for index in 0..copies {
        let number = offset + index + 2;
        body.push_str(&format!("<measure number=\"{number}\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward>{short}<note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>phrase</voice><staff>2</staff></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>phrase</voice><staff>2</staff></note><forward><duration>24</duration><voice>low</voice><staff>1</staff></forward><backup><duration>288</duration></backup><forward><duration>264</duration></forward><note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>phrase</voice><staff>2</staff></note></measure>"));
    }
    format!("<score-partwise version=\"4.0\"><part id=\"synthetic\">{body}</part></score-partwise>")
}

fn cursor_shape(xml: &str, number: usize) -> Vec<(String, Vec<String>)> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let number = number.to_string();
    let measure = doc.descendants().find(|node| node.has_tag_name("measure")
        && node.attribute("number") == Some(number.as_str())).unwrap();
    measure.children().filter(|node| node.is_element()
        && matches!(node.tag_name().name(), "note" | "backup" | "forward"))
        .map(|node| {
            let fields = ["step", "octave", "duration", "voice", "staff"]
                .map(|name| node.descendants().find(|child| child.has_tag_name(name))
                    .and_then(|child| child.text()).unwrap_or("").to_owned()).to_vec();
            (node.tag_name().name().to_owned(), fields)
        }).collect()
}

fn run_early_stage_case(offset: usize, copies: usize) {
    let xml = independent_rest_and_lane(offset, copies);
    let path = std::env::temp_dir().join(format!("etuschlopf-owner-handoff-{}-{offset}-{copies}.render.musicxml", std::process::id()));
    let (trial, post, receipts) = build_fragmented_rest_chain_transactional_write_trial(&xml, "Synthetic rest owner", &path).unwrap();
    assert!(trial.consolidation_adopted > 0);
    let post = post.unwrap();
    receipts.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
    let ((legacy, audit), receipt) = human_bound_cursor_lane_owner_transfer_xml_with_receipt(post.as_str());
    assert!(audit.written, "the independent XML-only cursor probe must be positive");
    assert_ne!(cursor_shape(&legacy, offset + 2), cursor_shape(&post, offset + 2));
    let mut probe = RenderTransitionReceipts::new();
    probe.push_preserving(receipt, &legacy);
    probe.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);

    let empty = structural_write_authority::CanonicalFinalChordStructuralPlan::default();
    let ((unchanged, audit), receipt) = human_bound_cursor_lane_owner_transfer_xml_with_receipt((post.as_str(), &empty));
    assert_eq!(unchanged, post); assert!(!audit.written); assert!(receipt.is_none());

    let master = MasterAdoptedState::adopt_genesis_state(xml.clone(),
        MasterLineage::lineage_from_genesis(crate::master_state_identity::tests::synthetic_genesis_for_tests(&xml)));
    let mut state = RenderSovereignState::begin_project_render(&path, &master);
    let root = state.initial_trial_root(true);
    state.adopt_transition(root, &xml, RenderTransitionReceipts::new(), RenderSinkKind::PrunedPdfTrial,
        |path, _, value| (std::fs::write(path, value), None)).unwrap();
    let mut plan = structural_write_authority::CanonicalFinalChordStructuralPlan::default();
    let voices = structural_write_authority::FrozenSerializedVoicePlan::default();
    let omissions = Default::default();
    // This prefix deliberately has no later canonical voice context. Its
    // downstream refusal is allowed; the positive rest commit and untouched
    // independent cursor sequences must already hold at that boundary.
    let result = run_rest_and_articulation_render_stage(RestAndArticulationRenderStageInputs {
        render_state: &mut state, final_render_source_available: true,
        reference: &ReferenceReport::default(), render_xml_trial_path: &path,
        pre_preview_render_xml_for_gap: &xml, render_final_chord_structural_plan: &mut plan,
        frozen_serialized_voice_plan: &voices, render_pruned_final_chord_omissions: &omissions,
    });
    match result {
        Ok(outcome) => assert!(outcome.fragmented_rest_chain_transactional_write_trial.written),
        Err(error) => assert!(error.to_string().contains("Bridge voice-plan binding to current Render input"),
            "the stage may refuse missing later context, but must reach that boundary: {error:#}"),
    }
    let actual = state.document().unwrap();
    let doc = roxmltree::Document::parse(actual).unwrap();
    let first_rest = doc.descendants().find(|node| node.has_tag_name("note")
        && node.children().any(|child| child.has_tag_name("rest"))).unwrap();
    assert_eq!(first_rest.children().find(|node| node.has_tag_name("duration")).unwrap().text(), Some("48"),
        "the actual stage must commit the positive rest transaction");
    for index in 0..copies {
        assert_eq!(cursor_shape(actual, offset + index + 2), cursor_shape(&post, offset + index + 2),
            "a rest commit must not authorize an unrelated XML-only cursor permutation");
    }
    crate::master_state_identity::reset_render_lineage_measurement();
    let _ = std::fs::remove_file(path);
}

#[test]
fn rest_stage_preserves_independent_cursor_sequences_without_owner_context() {
    const FLAG: &str = "ETUSCHLOPF_CURSOR_OWNER_PREFIX_CHILD";
    if std::env::var_os(FLAG).is_none() {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "pipeline::rest_bridge_owner_handoff_tests::rest_stage_preserves_independent_cursor_sequences_without_owner_context", "--nocapture"])
            .env(FLAG, "1").output().unwrap();
        assert!(output.status.success(), "isolated stage probe failed: {}\n{}",
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stdout).contains("test result: ok. 1 passed;"));
        return;
    }
    for (offset, copies) in [(0, 1), (100, 1), (200, 2)] { run_early_stage_case(offset, copies); }
}

fn supported_bound_input(offset: usize, copies: usize, lower: &str, upper: &str) -> String {
    let pitch = |step: &str, octave: usize, voice: &str, staff: usize, beams: &str| format!(
        "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>24</duration><voice>{voice}</voice><type>16th</type><staff>{staff}</staff>{beams}</note>");
    let mut measures = String::new();
    for index in 0..copies {
        let number = offset + index + 1;
        let notes = [pitch("C", 3, lower, 2, "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>"),
            pitch("G", 5, upper, 1, ""), pitch("G", 3, lower, 2, "<beam number=\"1\">end</beam><beam number=\"2\">end</beam>"),
            pitch("A", 5, upper, 1, "")].concat();
        measures.push_str(&format!("<measure number=\"{number}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>{notes}<forward><duration>288</duration></forward><backup><duration>384</duration></backup><forward><duration>24</duration><voice>{lower}</voice><staff>2</staff></forward><note><rest/><duration>24</duration><voice>{lower}</voice><type>16th</type><staff>2</staff></note><forward><duration>24</duration><voice>{lower}</voice><staff>2</staff></forward><note><rest/><duration>24</duration><voice>{lower}</voice><type>16th</type><staff>2</staff></note><note><rest/><duration>288</duration><voice>{lower}</voice><type>half</type><dot/><staff>2</staff></note></measure>"));
    }
    format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\">{measures}</part></score-partwise>")
}

#[test]
fn bound_original_owner_still_materializes_and_carries_its_exchange() {
    for (offset, copies, lower, upper) in [(0, 1, "low", "high"), (100, 1, "left", "right"), (200, 2, "low", "high")] {
        let input = supported_bound_input(offset, copies, lower, upper);
        let groups = (0..copies).map(|index| vec![index * 4 + 1, index * 4 + 3]).collect::<Vec<_>>();
        let (canonical, voices, source) = structural_write_authority::CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(&input, &groups).unwrap();
        let source = source.synthetic_authored_cursor_connections();
        let guard = canonical.bind_rest_bridge_cursor_exchange(&source, &voices, &Default::default()).unwrap();
        let ((post, audit), receipt) = human_bound_cursor_lane_owner_transfer_xml_with_receipt((source.xml(), &guard));
        assert!(audit.written); assert!(receipt.is_some());
        let carried = guard.carry_rest_bridge_cursor_exchange(&source, &post, receipt.as_ref(), &voices, &Default::default()).unwrap();
        assert_eq!(carried.xml(), post);
        assert!(guard.carry_rest_bridge_cursor_exchange(&source, &post, None, &voices, &Default::default()).is_err());
        let next = canonical.bind_rest_bridge_cursor_exchange(&carried, &voices, &Default::default()).unwrap();
        let ((again, audit), receipt) = human_bound_cursor_lane_owner_transfer_xml_with_receipt((carried.xml(), &next));
        assert_eq!(again, post); assert!(!audit.written); assert!(receipt.is_none());
    }
}
