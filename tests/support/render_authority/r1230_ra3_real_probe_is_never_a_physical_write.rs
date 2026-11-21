// Kept in the pipeline's private scope to exercise the production RA3 observer.
#[test]
fn r1230_ra3_real_probe_is_never_a_physical_write() {
    fn observe_primary_staccato(xml: &str) -> (usize, bool) {
        let (output, audit, receipt) =
            apply_single_clear_staccato_render_write_with_placement_readback(xml).unwrap();
        assert!(matches!(output, std::borrow::Cow::Borrowed(_)));
        assert_eq!(output.as_ref(), xml);
        assert!(!audit.written && !audit.render_xml_write && receipt.is_none());
        (audit.clear_candidate_count, single_clear_staccato_target_persisted_in_final_xml(&output, &audit))
    }

    fn pair(marked: bool, chord: bool) -> String {
        let marking = if marked { "<notations><articulations><staccato/></articulations></notations>" } else { "" };
        let member = if chord {
            "<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>"
        } else { "" };
        format!("<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff>{marking}</note>{member}<note><rest/><duration>24</duration><voice>1</voice><staff>1</staff></note>")
    }

    let unmarked = pair(false, false);
    let marked = pair(true, false);
    assert_eq!(observe_primary_staccato(&wrap("")), (0, false));
    assert_eq!(observe_primary_staccato(&wrap(&unmarked)), (1, false));
    assert_eq!(observe_primary_staccato(&wrap(&marked)), (1, true));
    assert_eq!(observe_primary_staccato(&wrap(&pair(true, true))), (1, false));
    assert_eq!(observe_primary_staccato(&wrap(&marked.replace("<staff>1", "<staff>2"))), (0, false));
    assert_eq!(observe_primary_staccato(&wrap_at(0, &marked)), (1, false));

    // Equally ranked candidates use the earliest attack for the existing-mark check.
    assert_eq!(observe_primary_staccato(&wrap(&format!("{marked}{unmarked}"))), (2, true));
    assert_eq!(observe_primary_staccato(&wrap(&format!("{unmarked}{marked}"))), (2, false));
    // A replayed simultaneous anchor retains its existing positive pitch/attack readback.
    let next = "<backup><duration>48</duration></backup>";
    assert_eq!(observe_primary_staccato(&wrap_at(1, &format!("{marked}{next}{unmarked}"))), (1, true));
    let mut measures = String::new();
    for ordinal in (1..=15).rev() {
        let body = if ordinal == 1 { &marked } else { &unmarked };
        measures.push_str(&format!("<measure number=\"{ordinal}\">{body}</measure>"));
    }
    let score = format!("<score-partwise><part id=\"P1\">{measures}</part></score-partwise>");
    assert_eq!(observe_primary_staccato(&score), (12, true));
}
