// Integration test for a grandfathered production entry, kept in its private lexical scope.
    #[test]
    fn r1230_ra3_real_probe_is_never_a_physical_write() {
        let xml=wrap("<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note><note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note><forward><duration>336</duration></forward>");
        let path=path();
        assert_eq!(articulation_single_clear_candidate_rows(&build_pianist_helpful_articulation_and_notation_audit_no_mutation(&xml)).len(),1);
        let result=apply_single_clear_staccato_render_write_with_placement_readback(&xml,"RA3 no-write",&path);
        if let Ok((_,audit,occurrence))=result { assert!(!audit.written); assert!(occurrence.is_none()); }
        assert!(!path.exists());
        crate::master_state_identity::reset_render_lineage_measurement();
    }
