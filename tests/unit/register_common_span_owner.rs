use super::*;

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
    assert_eq!(format!("{:?}", crate::musicxml::parse_musicxml(&before).unwrap()),
        format!("{:?}", crate::musicxml::parse_musicxml(master.document()).unwrap()));
    assert_eq!(plan, crate::registernotation::materialisierungsauftrag().unwrap());
    let committed = master.clone();
    assert_eq!(committed.document(), master.document());
    assert!(committed.lineage().lineage_continuity_holds(committed.document()));
    committed.witness_released_artifact(committed.document());
    crate::registernotation::loesche();
}
