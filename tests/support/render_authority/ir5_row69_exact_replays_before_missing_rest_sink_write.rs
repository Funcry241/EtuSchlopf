// Integration test for a grandfathered production entry, kept in its private lexical scope.
    #[test]
    fn ir5_row69_exact_replays_before_missing_rest_sink_write() {
        let ordinal = 41_usize;
        let master_xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>288</duration></forward><backup><duration>384</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>384</duration><voice>2</voice><type>whole</type><staff>2</staff></note></measure></part></score-partwise>"
        );
        let master = MasterAdoptedState::adopt_genesis_state(
            master_xml.clone(),
            MasterLineage::lineage_from_genesis(synthetic_genesis_for_tests(&master_xml)),
        );
        let path = std::env::temp_dir().join(format!(
            "etuschlopf-r1230-ir5-missing-rest-{}.render.musicxml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let (trial, candidate, receipts) = build_render_xml_pianistic_missing_rest_insert_trial(
            &master_xml,
            "r1230 IR5 row69 regression",
            &path,
        )
        .expect("IR5 candidate must build");
        assert!(trial.written);
        assert_eq!(receipts.len(), 1, "IR5 must carry the #69 EXACT receipt");
        let candidate = candidate.expect("safe IR5 candidate");
        assert!(!candidate.contains("<forward><duration>288</duration></forward>"));
        assert!(candidate.contains("<rest"));

        let mut state = RenderSovereignState::begin_project_render(&path, &master);
        let root = state.initial_trial_root(false);
        state
            .adopt_transition(
                root,
                &candidate,
                receipts,
                RenderSinkKind::MissingRestInsertTrial,
                |artifact_path, _source_xml, verified_candidate| {
                    (std::fs::write(artifact_path, verified_candidate), None)
                },
            )
            .expect("#69 EXACT must replay before the IR5 sink write");

        assert_eq!(state.document(), Some(candidate.as_str()));
        assert_eq!(
            std::fs::read_to_string(&path).ok().as_deref(),
            Some(candidate.as_str())
        );
        let _ = std::fs::remove_file(path);
    }
