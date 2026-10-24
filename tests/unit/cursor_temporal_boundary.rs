use super::{check_master_cursor_boundary, check_render_cursor_boundary};

fn score(body: &str, implicit: bool) -> String {
    format!("<score-partwise><part><measure implicit=\"{}\"><attributes><divisions>2</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>", if implicit { "yes" } else { "no" })
}
fn note(duration: usize) -> String {
    format!("<note><pitch><step>C</step><octave>4</octave></pitch><duration>{duration}</duration><voice>1</voice></note>")
}

#[test]
fn both_real_boundary_adapters_preserve_clean_and_partial_serialization() {
    let clean = score(&note(8), false);
    let partial = score(&(note(8)+"<backup><duration>4</duration></backup>"+&note(4)), false);
    let pickup = score(&note(2), true);
    for xml in [&clean, &partial, &pickup] {
        let before=xml.as_bytes().to_vec();
        for checked in [check_master_cursor_boundary(xml), check_render_cursor_boundary(xml)] {
            let checked=checked.expect("valid temporal serialization");
            assert!(checked.binds(xml));
            assert_eq!(checked.audit().temporal_violation_measures,0);
            assert_eq!(checked.audit().unsupported_inputs,0);
        }
        assert_eq!(xml.as_bytes(),before);
    }
    assert_eq!(check_master_cursor_boundary(&partial).unwrap().audit().partial_serializations_observed,1);
}

#[test]
fn both_real_boundary_adapters_reject_time_failures_without_repair() {
    let cases = [
        (score(&note(10),false),"CursorOverflow"),
        (score(&("<backup><duration>2</duration></backup>".to_owned()+&note(10)),false),"BackupUnderflow"),
        (score(&note(2),false),"FinalCursorMismatch"),
        (score(&(note(8)+"<note><rest/><duration>2</duration></note><backup><duration>10</duration></backup>"+&note(8)),false),"NoteEndOverflow"),
        (score(&(note(4)+"<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>6</duration><voice>1</voice></note><forward><duration>4</duration></forward>"),false),"ChordTemporalConflict"),
    ];
    for (xml,reason) in cases {
        let before=xml.clone();
        for failure in [check_master_cursor_boundary(&xml),check_render_cursor_boundary(&xml)] {
            let failure=failure.expect_err("time error must propagate");
            assert_eq!(failure.audit.temporal_violation_measures,1);
            assert!(failure.to_string().contains(reason),"{failure}");
        }
        assert_eq!(xml,before);
    }
}

#[test]
fn both_real_boundary_adapters_reject_unparseable_or_uncovered_input() {
    for xml in ["<score", "<score-timewise/>", "<score-partwise/>",
        "<score-partwise><part><measure><note><duration>4</duration></note></measure></part></score-partwise>"] {
        for failure in [check_master_cursor_boundary(xml),check_render_cursor_boundary(xml)] {
            assert_eq!(failure.expect_err("missing coverage is not green").audit.unsupported_inputs,1);
        }
    }
}

#[test]
fn successful_proof_for_a_does_not_bind_b_or_even_a_byte_different_comment() {
    let a=score(&note(8),false);
    let b=a.replace("<step>C</step>","<step>D</step>");
    let checked=check_master_cursor_boundary(&a).unwrap();
    assert!(!checked.binds(&b));
    assert!(!checked.binds(&(a.clone()+"<!-- another byte sequence -->")));
    assert!(checked.binds(&a));
}

#[test]
fn non_temporal_attribute_changes_do_not_reduce_replay_coverage() {
    let xml=score(&(note(4)+"<attributes><clef><sign>F</sign><line>4</line></clef></attributes>"+&note(4)),false);
    assert!(check_master_cursor_boundary(&xml).is_ok());
    assert!(check_render_cursor_boundary(&xml).is_ok());
}

#[test]
fn unsupported_temporal_grammar_is_explicit_and_not_default_meter_green() {
    let clean=score(&note(8),false);
    for bad in [clean.replace("<beats>4</beats>","<beats>2+2</beats>"),
        clean.replace("<duration>8</duration>","<duration>oops</duration>"),
        clean.replace("<divisions>2</divisions>",""),
        clean.replace("</part>","</part><part><measure/></part>"),
        clean.replace("</note>","</note><attributes><divisions>4</divisions></attributes>")] {
        assert_eq!(check_master_cursor_boundary(&bad).unwrap_err().audit.unsupported_inputs,1);
    }
}

#[test]
fn independent_input_duplication_does_not_change_the_boundary_verdict() {
    for xml in [score(&note(8),false),score(&note(10),false)] {
        let independent=xml.clone();
        let first=check_master_cursor_boundary(&xml);
        let second=check_master_cursor_boundary(&independent);
        assert_eq!(first.is_ok(),second.is_ok());
        assert_eq!(xml,independent);
    }
}

#[test]
fn both_boundaries_replay_inherited_meter_and_divisions() {
    let xml=score(&note(8),false).replace("</part>", &format!("<measure>{}</measure></part>",note(8)));
    for checked in [check_master_cursor_boundary(&xml),check_render_cursor_boundary(&xml)] {
        assert_eq!(checked.expect("supported inherited temporal context").audit().checked_measures,2);
    }
}
