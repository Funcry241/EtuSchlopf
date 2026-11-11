use super::*;

#[test]
fn joint_clef_materialization_removes_redundancy_and_refuses_stale_bytes() {
    let _lock = crate::registernotation::sperre_fuer_tests();
    reset_joint_reading_course();
    let initial="<attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>";
    let redundant="<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>";
    let note="<note><pitch><step>C</step><octave>3</octave></pitch><duration>1</duration><voice>2</voice><type>quarter</type><staff>2</staff></note>";
    let source=format!("<score-partwise><part id=\"P\"><measure number=\"One\">{initial}{note}{redundant}{note}{note}{note}</measure></part></score-partwise>");
    let first=source.find(initial).unwrap();let second=source.find(redundant).unwrap();
    let decision=crate::structural_write_authority::RustigClefSurfaceDecision::prepare_joint_course(&source,
        &[(false,vec![first],None),(false,vec![second],None)]).unwrap();
    let expected=decision.joint_course_post().unwrap().to_owned();
    assert_eq!(expected.matches("<clef").count(),1);
    let mut stale=format!("{source}<!--changed-->");let retained=stale.clone();
    let (result,receipt)=decision.apply_prepared_joint_course(&mut stale);
    assert!(result.is_err());assert!(receipt.is_none());assert_eq!(stale,retained);
    let lineage=master_state_identity::MasterLineage::lineage_from_genesis(
        master_state_identity::tests::synthetic_genesis_for_tests(&source));
    let mut master=crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(source.clone(),lineage);
    let mut post=source.clone();let (result,receipt)=decision.apply_prepared_joint_course(&mut post);
    result.unwrap();assert!(receipt.is_some());assert_eq!(post,expected);
    let mut receipts=master_state_identity::OwnerReceipts::default();receipts.push_preserving(receipt);
    master.adopt_materialized_document(post,receipts,"SyntheticJointClefCourse");
    assert_eq!(master.document(),expected);
    assert!(master.lineage().lineage_continuity_holds(master.document()));
}

#[test]
fn joint_replacement_uses_original_writers_and_coupled_origin_commit() {
    let _lock = crate::registernotation::sperre_fuer_tests();
    crate::registernotation::loesche();
    // G-sharp 6 needs four treble ledger lines before the octave shift.
    let notes = (0..8).map(|_| "<note><pitch><step>G</step><alter>1</alter><octave>6</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>").collect::<String>();
    let initial = format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Probe</part-name></score-part></part-list><part id=\"P\"><measure number=\"SyntheticC3\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes>{notes}<note><rest/><duration>192</duration><voice>1</voice><type>half</type><staff>1</staff></note></measure></part></score-partwise>");
    crate::registernotation::beobachte_vor_k00(&initial);
    let lineage = master_state_identity::MasterLineage::lineage_from_genesis(
        master_state_identity::tests::synthetic_genesis_for_tests(&initial));
    let mut master = crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(initial.clone(), lineage);
    master.adopt_in_place_transformation("SyntheticInitialOctave", |xml| {
        ((), apply_octave_notation_policy(xml).block_receipts)
    });
    let before = master.document().to_owned();
    assert!(before.contains("<octave-shift"));
    let path = std::env::temp_dir().join(format!("ets-c3-coupled-{}.render.musicxml",std::process::id()));
    let mut render = crate::structural_write_authority::RenderSovereignState::begin_project_render(&path,&master);
    let root = render.initial_trial_root(true);
    render.adopt_transition(root,&before,master_state_identity::RenderTransitionReceipts::new(),
        crate::structural_write_authority::RenderSinkKind::PrunedPdfTrial,
        |path,_,post| (std::fs::write(path,post),None)).unwrap();
    let plan = crate::registernotation::RegisternotationsSichtmaterialisierungsauftrag {
        semantische_plaene:0,kontaktfusionen:0,stille_bruecken:0,
        semantischer_plan_fingerabdruck:"synthetic-c3-empty".into(),sicht_fingerabdruck:"synthetic-c3-empty".into(),plaene:Vec::new() };
    let origins = crate::registernotation::reader_policy::prepare_joint_origins(&plan,&[],&before,&before).unwrap();
    let prepared = prepare_joint_octave_write(&before,&[]).unwrap();
    assert!(!prepared.post.contains("<octave-shift"));
    let origins = origins.validate_documents(&prepared.post,&prepared.post).unwrap();
    // A declined transaction must leave XML, lineage, artifact and registry usable.
    assert!(origins.commit_with(|| Err::<(),_>(anyhow!("synthetic decline"))).is_err());
    assert_eq!(master.document(),before);
    assert_eq!(render.document(),Some(before.as_str()));
    assert_eq!(std::fs::read_to_string(&path).unwrap(),before);
    let origins = crate::registernotation::reader_policy::prepare_joint_origins(&plan,&[],&before,&before)
        .unwrap().validate_documents(&prepared.post,&prepared.post).unwrap();
    origins.commit_with(|| {
        let (master_post,receipts)=prepared.materialize()?;
        assert!(receipts.coverage_diagnostic(&before,&master_post).is_none());
        let mut next=master.clone();
        next.adopt_materialized_document(master_post,receipts,"SyntheticC3PrivateMaster");
        let (render_post,receipts)=prepared.materialize()?;
        let mut render_receipts=master_state_identity::RenderTransitionReceipts::new();
        render_receipts.append_original_owner_chain(receipts)?;
        let root=render.require_state_root("SyntheticC3CoupledCommit");
        render.adopt_final_notation_checkpoint(root,&render_post,render_receipts)?;
        master=next;
        Ok(())
    }).unwrap();
    assert_eq!(master.document(),prepared.post);
    assert_eq!(render.document(),Some(prepared.post.as_str()));
    assert_eq!(std::fs::read_to_string(&path).unwrap(),prepared.post);
    assert!(master.lineage().lineage_continuity_holds(master.document()));
    crate::registernotation::reader_policy::confirm_materialization(master.document()).unwrap();
    std::fs::remove_file(path).unwrap();
    crate::registernotation::loesche();
}

#[test]
fn common_span_uses_actual_writer_receipts_and_master_adoption() {
    let _sperre = crate::registernotation::sperre_fuer_tests();
    let notes = [6, 6, 6, 6, 7, 7, 6, 6].into_iter().map(|octave| {
        format!("<note><pitch><step>F</step><alter>1</alter><octave>{octave}</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>")
    }).collect::<String>();
    let before = format!("<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Probe</part-name></score-part></part-list><part id=\"P\"><measure number=\"SyntheticAlpha\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes>{notes}<note><rest/><duration>192</duration><voice>1</voice><type>half</type><staff>1</staff></note></measure></part></score-partwise>");
    crate::registernotation::loesche();
    crate::registernotation::beobachte_vor_k00(&before);
    let plan = crate::registernotation::materialisierungsauftrag().unwrap();
    assert_eq!(plan.plaene.len(), 1);
    assert_eq!(plan.plaene[0].darstellung,
        crate::registernotation::RegisternotationsDarstellung::Fuenfzehnma);
    let genesis = master_state_identity::tests::synthetic_genesis_for_tests(&before);
    let lineage = master_state_identity::MasterLineage::lineage_from_genesis(genesis);
    let mut master = crate::structural_write_authority::MasterAdoptedState::
        adopt_genesis_state(before.clone(), lineage);
    master.adopt_in_place_transformation("SyntheticCommonSpanMaterialization", |xml| {
        let materialized = apply_octave_notation_policy(xml);
        assert_eq!(materialized.block_receipts.len(), 1);
        assert!(materialized.block_receipts.coverage_diagnostic(&before, xml).is_none());
        ((), materialized.block_receipts)
    });
    assert!(master.lineage().lineage_continuity_holds(master.document()));
    assert_eq!(master.lineage().committed_event_count(), 1);
    assert_ne!(master.document(), before);
    assert!(master.document().contains("size=\"15\""));
    // Read-only removal evidence uses the real existing creator registration.
    // This assertion introduces no second writer execution or receipt mint.
    let document = roxmltree::Document::parse(master.document()).unwrap();
    for direction in document.descendants().filter(|node| node.has_tag_name("direction")
        && node.descendants().any(|child| child.has_tag_name("octave-shift"))) {
        let range = direction.range();
        let mut proposed = master.document().to_owned();
        proposed.replace_range(range.clone(), "");
        let journal = crate::arrangement::representation_transport::RepresentationStep::recorded(
            master.document(), &proposed, vec![(range, String::new())]);
        crate::registernotation::reader_policy::bind_original_octave_removal_evidence(journal).unwrap();
    }
    assert_eq!(format!("{:?}", crate::musicxml::parse_musicxml(&before).unwrap()),
        format!("{:?}", crate::musicxml::parse_musicxml(master.document()).unwrap()));
    assert_eq!(plan, crate::registernotation::materialisierungsauftrag().unwrap());
    let committed = master.clone();
    assert_eq!(committed.document(), master.document());
    assert!(committed.lineage().lineage_continuity_holds(committed.document()));
    committed.witness_released_artifact(committed.document());
    crate::registernotation::loesche();
}
