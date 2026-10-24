// Integration test for a grandfathered production entry, kept in its private lexical scope.
    #[test]
    fn row57_prepares_file_bound_post_without_writing_before_adoption() {
        let ordinal = 57_usize;
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><rest/><duration>4</duration><voice>1</voice><type>whole</type><staff>2</staff></note><attributes><clef number=\"2\"><sign>G</sign><line>2</line></clef></attributes><attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes></measure></part></score-partwise>"
        );
        let path = std::env::temp_dir().join(format!(
            "etuschlopf-r1230-ir4-row57-{}.render.musicxml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);

        let (trial, receipt) = build_render_xml_print_cleanup_one_safe_visual_trial(
            &xml,
            "r1230 IR4 row57 preparation regression",
            &path,
        );
        let trial = trial.expect("#57 preparation must remain valid");
        assert!(trial.written, "fixture must select one safe #57 cleanup");
        assert!(
            !path.exists(),
            "#57 preparation must not touch the productive artifact before sovereign verification"
        );

        let receipt = receipt.expect("changed #57 preparation must mint its BLOCK receipt");
        let (receipt, realized_post) = receipt.split_realized_post_document();
        let realized_post = realized_post.expect("#57 file-bound receipt must carry its planned post");
        assert_ne!(realized_post, xml);
        receipt
            .validate_block_transition::<RenderRow57BlockMint>(&xml, &realized_post)
            .expect("#57 receipt must bind the exact prepared pre/post");

        let _ = std::fs::remove_file(path);
    }
