//! Synthetic, corpus-independent adapter tests. Fixtures build their own
//! carried heads from fixture metadata; product code never does this.
use super::*;

#[test]
fn final_primary_beam_readback_survives_a_voice_relabel() {
    let begin=note_xml(&n('C',4,DIV,1)).replace("</note>","<beam number=\"1\">begin</beam></note>");
    let mut end=n('D',4,DIV,1);end.voice=3;
    let end=note_xml(&end).replace("</note>","<beam number=\"1\">end</beam></note>");
    let xml=score(&[head_attrs(DIV,4,4,"F")+&begin+&end+&rest(2*DIV,1)]);
    let (before,after,metrics)=compare_actual_output(&xml,&xml,"1").unwrap();
    assert_eq!(before,after);assert_eq!(metrics.visible_heads,2);
}

#[test]
fn final_primary_beam_readback_refuses_competing_open_groups() {
    let begin=note_xml(&n('C',4,DIV,1)).replace("</note>","<beam number=\"1\">begin</beam></note>");
    let mut other=n('D',4,DIV,1);other.voice=3;
    let other=note_xml(&other).replace("</note>","<beam number=\"1\">begin</beam></note>");
    let xml=score(&[head_attrs(DIV,4,4,"F")+&begin+&other+&rest(2*DIV,1)]);
    assert!(compare_actual_output(&xml,&xml,"1").is_err());
}

#[test]
fn final_tie_readback_survives_voice_relabel_but_refuses_parallel_unisons() {
    let mut start=n('C',4,DIV,1);start.tie=Some("start");
    let mut stop=n('C',4,DIV,1);stop.voice=3;stop.tie=Some("stop");
    let xml=score(&[head_attrs(DIV,4,4,"F")+&note_xml(&start)+&note_xml(&stop)+&rest(2*DIV,1)]);
    let (before,after,_)=compare_actual_output(&xml,&xml,"1").unwrap();assert_eq!(before,after);
    let mut competing=n('C',4,DIV,1);competing.voice=3;competing.tie=Some("start");
    let xml=score(&[head_attrs(DIV,4,4,"F")+&note_xml(&start)+&note_xml(&competing)+&note_xml(&stop)+&rest(DIV,1)]);
    assert!(compare_actual_output(&xml,&xml,"1").is_err());
}

const DIV: i64 = 2; // divisions per quarter in most fixtures

struct N {
    step: char,
    oct: i32,
    dur: i64,
    staff: u8,
    voice: u8,
    hidden: bool,
    chord: bool,
    tie: Option<&'static str>,
    grace: bool,
}
fn n(step: char, oct: i32, dur: i64, staff: u8) -> N {
    N { step, oct, dur, staff, voice: staff, hidden: false, chord: false, tie: None, grace: false }
}
fn note_xml(x: &N) -> String {
    let po = if x.hidden { " print-object=\"no\"" } else { "" };
    let ch = if x.chord { "<chord/>" } else { "" };
    let gr = if x.grace { "<grace/>" } else { "" };
    let tie = x.tie.map(|t| format!("<tie type=\"{t}\"/>")).unwrap_or_default();
    let dur = if x.grace { String::new() } else { format!("<duration>{}</duration>", x.dur) };
    format!("<note{po}>{gr}{ch}<pitch><step>{}</step><octave>{}</octave></pitch>{dur}{tie}<voice>{}</voice><staff>{}</staff></note>", x.step, x.oct, x.voice, x.staff)
}
fn rest(dur: i64, staff: u8) -> String {
    format!("<note><rest/><duration>{dur}</duration><voice>{staff}</voice><staff>{staff}</staff></note>")
}
fn backup(d: i64) -> String {
    format!("<backup><duration>{d}</duration></backup>")
}
fn shift(typ: &str, size: u8, staff: u8, offset: Option<i64>) -> String {
    let off = offset.map(|o| format!("<offset>{o}</offset>")).unwrap_or_default();
    format!("<direction><direction-type><octave-shift type=\"{typ}\" size=\"{size}\"/></direction-type>{off}<staff>{staff}</staff></direction>")
}
fn clef(staff: u8, sign: &str) -> String {
    let line = if sign == "G" { 2 } else if sign == "F" { 4 } else { 3 };
    format!("<attributes><clef number=\"{staff}\"><sign>{sign}</sign><line>{line}</line></clef></attributes>")
}
fn head_attrs(div: i64, beats: u8, beat_type: u8, lower: &str) -> String {
    format!("<attributes><divisions>{div}</divisions><time><beats>{beats}</beats><beat-type>{beat_type}</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>{lower}</sign><line>{}</line></clef></attributes>", if lower == "G" { 2 } else { 4 })
}
fn score(measures: &[String]) -> String {
    let body: String = measures.iter().enumerate().map(|(i, m)| format!("<measure number=\"{}\">{m}</measure>", i + 1)).collect();
    format!("<score-partwise><part-list><score-part id=\"P1\"/></part-list><part id=\"P1\">{body}</part></score-partwise>")
}
/// Fixture-side carried heads: span, structural measure, raw local onset, staff,
/// visibility, sounding spelling. Grace notes are skipped like unsupported input.
fn carried(xml: &str) -> Vec<AdapterHead> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    let part = doc.root_element().children().find(|n| n.has_tag_name("part")).unwrap();
    let mut out = Vec::new();
    for (mi, m) in part.children().filter(|n| n.has_tag_name("measure")).enumerate() {
        let (mut cur, mut last) = (0i64, 0i64);
        for el in m.children().filter(|n| n.is_element()) {
            let txt = |name: &str| el.children().find(|c| c.has_tag_name(name)).and_then(|c| c.text()).unwrap_or("0").to_string();
            match el.tag_name().name() {
                "backup" => cur -= txt("duration").parse::<i64>().unwrap(),
                "forward" => cur += txt("duration").parse::<i64>().unwrap(),
                "note" => {
                    let chord = el.children().any(|c| c.has_tag_name("chord"));
                    let on = if chord { last } else { cur };
                    if !chord {
                        last = cur;
                        cur += txt("duration").parse::<i64>().unwrap();
                    }
                    if let Some(p) = el.children().find(|c| c.has_tag_name("pitch")) {
                        let step = p.children().find(|c| c.has_tag_name("step")).unwrap().text().unwrap();
                        let oct: i32 = p.children().find(|c| c.has_tag_name("octave")).unwrap().text().unwrap().parse().unwrap();
                        let s = "CDEFGAB".find(step).unwrap() as i32;
                        out.push(AdapterHead {
                            span: el.range(),
                            measure: mi + 1,
                            onset: on,
                            staff: txt("staff"),
                            visible: el.attribute("print-object") != Some("no"),
                            diatonic: oct * 7 + s,
                            continuation: el.children().any(|c| c.has_tag_name("tie") && c.attribute("type") == Some("stop")),
                        });
                    }
                }
                _ => {}
            }
        }
    }
    out
}
fn st(i: u8) -> k::State {
    k::State::new(i).unwrap()
}
fn all_states() -> Vec<k::State> {
    (0..10).map(st).collect()
}
fn cap(writable: Vec<k::State>, xml: &str) -> StaffCapability {
    // Synthetic owners explicitly attest all fixture boundaries. Production
    // uses the actual existing materializers' independently checked anchors.
    let mut boundaries = BTreeMap::new();
    if let Ok(observed) = observe(xml) {
        for at in observed.notes.values().map(|n| n.at)
            .chain(observed.events.iter().map(|e| e.at)) {
            boundaries.insert(at, BoundaryCapability { clef: true, octave_start: true, octave_stop: true });
        }
    }
    StaffCapability {
        evidence: "synthetic-test".into(),
        entry: writable.clone(),
        writable,
        boundaries,
        exit_octave_stop: true,
        fixed_clefs: BTreeMap::new(),
        clef_change_inside_primary_beam: false,
        clef_change_inside_tie: false,
        octave_change_inside_primary_beam: true,
        octave_change_inside_tie: false,
        boundary_in_silence: false,
        boundary_at_hidden_head_only: false,
    }
}
fn caps_all(xml: &str) -> Capabilities {
    let mut c = Capabilities::new(xml);
    c.staves.insert("1".into(), cap(all_states(), xml));
    c.staves.insert("2".into(), cap(all_states(), xml));
    c
}
fn solved(p: &C3Proposal, staff: &str) -> (k::Domain, Vec<LayerAnchor>, k::Outcome) {
    match &p.staves[staff] {
        StaffVerdict::Solved(x) => (x.domain.clone(), x.anchors.clone(), x.outcome.clone()),
        StaffVerdict::Unknown { reason } => panic!("staff {staff} unknown: {reason}"),
    }
}
fn unknown(p: &C3Proposal, staff: &str) -> String {
    match &p.staves[staff] {
        StaffVerdict::Unknown { reason } => reason.clone(),
        StaffVerdict::Solved(_) => panic!("staff {staff} unexpectedly solved"),
    }
}
fn q(n: i128, d: i128) -> Q {
    Q::new(n, d).unwrap()
}
fn idx(s: k::State) -> usize {
    usize::from(s.index())
}
/// Four quarter notes per staff in one 4/4 bar, inside both staves.
fn plain_bar(upper: [char; 4], lower: [char; 4]) -> String {
    let u: String = upper.iter().map(|&s| note_xml(&n(s, 5, DIV, 1))).collect();
    let l: String = lower.iter().map(|&s| note_xml(&n(s, 3, DIV, 2))).collect();
    format!("{u}{}{l}", backup(4 * DIV))
}

#[test]
fn incumbent_course_is_reproduced_with_zero_cost_and_retained() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F']), plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    for staff in ["1", "2"] {
        let (domain, anchors, outcome) = solved(&p, staff);
        assert_eq!(domain.layers.len(), 8);
        assert_eq!(anchors.len(), 8);
        assert!(!outcome.improved);
        assert_eq!(outcome.incumbent_cost, k::Cost::default());
        let incumbent: Vec<_> = domain.layers.iter().map(|l| l.incumbent).collect();
        assert_eq!(outcome.selected, incumbent);
    }
    assert!(solved(&p, "2").0.layers.iter().all(|l| l.incumbent == st(5)));
}

#[test]
fn boundary_in_silence_is_kept_and_new_silent_boundaries_need_capability() {
    // Staff 1: 8va from the start, stop during rests at beat 3, notes resume at bar 2.
    let upper = shift("down", 8, 1, None) + &note_xml(&n('C', 6, DIV, 1)) + &note_xml(&n('D', 6, DIV, 1)) + &shift("stop", 8, 1, None) + &rest(2 * DIV, 1);
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let m2 = plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F']);
    let xml = score(&[m1, m2]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    let (domain, anchors, _) = solved(&p, "1");
    let silent = anchors.iter().position(|a| a.at == q(2, 1)).expect("cut at the silent stop");
    assert!(anchors[silent].heads.is_empty());
    assert!(!anchors[silent].incumbent_events.is_empty());
    assert_eq!(domain.layers[silent - 1].incumbent, st(1));
    assert_eq!(domain.layers[silent].incumbent, st(0));
    let inc = &domain.layers[silent].incoming;
    assert!(inc[1][0], "incumbent transition stays legal");
    assert!(!inc[1][3], "new transition in silence needs evidence");
    // The silent half bar stays in the section age: 8va lasts exactly half a bar.
    assert_eq!(domain.layers[0].duration, k::Time::new(1, 4).unwrap());
    assert_eq!(domain.layers[silent].duration, k::Time::new(1, 2).unwrap());
}

#[test]
fn direction_offset_moves_the_boundary_exactly() {
    let upper = shift("down", 8, 1, None) + &note_xml(&n('C', 6, 4 * DIV, 1)) + &backup(4 * DIV) + &shift("stop", 8, 1, Some(3));
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &forward_full(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1, plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    let (_, anchors, _) = solved(&p, "1");
    assert!(anchors.iter().any(|a| a.at == q(3, 2)), "stop at cursor 0 + offset 3 divisions = 1.5 quarters");
}
fn forward_full(d: i64) -> String {
    format!("{}{}", format_args!("<forward><duration>{d}</duration></forward>"), backup(d))
}

#[test]
fn same_time_register_element_after_a_head_is_unknown() {
    let upper = note_xml(&n('C', 6, 4 * DIV, 1)) + &backup(4 * DIV) + &shift("down", 8, 1, None) + &rest(4 * DIV, 1);
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    assert!(unknown(&p, "1").contains("document order"));
    solved(&p, "2");
}

#[test]
fn hidden_heads_shape_cuts_but_carry_no_visible_ledger_cost() {
    let mut hidden = n('C', 8, DIV, 1);
    hidden.hidden = true;
    let upper = note_xml(&n('C', 5, DIV, 1)) + &note_xml(&hidden) + &note_xml(&n('E', 5, 2 * DIV, 1));
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    let (domain, anchors, outcome) = solved(&p, "1");
    assert_eq!(anchors.len(), 3);
    assert!(domain.layers[1].heads.iter().all(|h| !h.visible));
    assert_eq!(outcome.incumbent_cost, k::Cost::default());
    // Hidden-only anchor: no new boundary without capability.
    assert!(!domain.layers[1].incoming[0][1]);
}

#[test]
fn tie_crossing_counts_as_group_and_forbids_clef_change_without_capability() {
    let mut a = n('C', 3, 2 * DIV, 2);
    a.tie = Some("start");
    let mut b = n('C', 3, 2 * DIV, 2);
    b.tie = Some("stop");
    let lower = note_xml(&a) + &note_xml(&b);
    let m1 = head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, 4 * DIV, 1)) + &backup(4 * DIV) + &lower;
    let xml = score(&[m1]);
    let heads = carried(&xml);
    let lower_idx: Vec<usize> = (0..heads.len()).filter(|&i| heads[i].staff == "2").collect();
    let groups = vec![AdapterGroup { kind: AdapterGroupKind::TiedAttack, members: lower_idx }];
    let p = propose(&xml, &heads, &groups, &caps_all(&xml), None).unwrap();
    let (domain, _, _) = solved(&p, "2");
    assert!(domain.layers[1].group_crossing);
    assert!(!domain.layers[1].incoming[5][0], "clef change inside a tie needs capability");
    assert!(!domain.layers[1].incoming[5][6], "octave change inside a tie needs capability");
    assert!(domain.layers[1].incoming[5][5]);
}

#[test]
fn cross_staff_group_makes_both_staves_unknown() {
    let m1 = head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, 4 * DIV, 1)) + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    let heads = carried(&xml);
    let groups = vec![AdapterGroup { kind: AdapterGroupKind::Chord, members: vec![0, 1] }];
    let p = propose(&xml, &heads, &groups, &caps_all(&xml), None).unwrap();
    assert!(unknown(&p, "1").contains("cross-staff"));
    assert!(unknown(&p, "2").contains("cross-staff"));
}

#[test]
fn incumbent_outside_capability_and_missing_evidence_are_unknown() {
    let upper = shift("down", 8, 1, None) + &note_xml(&n('C', 6, 4 * DIV, 1)) + &shift("stop", 8, 1, None);
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    let mut caps = Capabilities::new(&xml);
    caps.staves.insert("1".into(), cap(vec![st(0)], &xml));
    let mut no_evidence = cap(all_states(), &xml);
    no_evidence.evidence = " ".into();
    caps.staves.insert("2".into(), no_evidence);
    let p = propose(&xml, &carried(&xml), &[], &caps, None).unwrap();
    assert!(unknown(&p, "1").contains("outside the evidenced capability"));
    assert!(unknown(&p, "2").contains("no evidenced"));
}

#[test]
fn pickup_and_meter_change_give_exact_nominal_bar_durations() {
    // Pickup: one quarter in 4/4; then a 3/4 bar of three quarters.
    let m1 = head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, DIV, 1)) + &backup(DIV) + &note_xml(&n('C', 3, DIV, 2));
    let m2 = format!("<attributes><time><beats>3</beats><beat-type>4</beat-type></time></attributes>{}{}{}",
        ['C', 'D', 'E'].iter().map(|&s| note_xml(&n(s, 5, DIV, 1))).collect::<String>(), backup(3 * DIV), note_xml(&n('C', 3, 3 * DIV, 2)));
    let xml = score(&[m1, m2]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    let (domain, anchors, _) = solved(&p, "1");
    assert_eq!(domain.layers[0].duration, k::Time::new(1, 4).unwrap());
    for l in &domain.layers[1..] {
        assert_eq!(l.duration, k::Time::new(1, 3).unwrap());
    }
    assert_eq!(anchors[1].measure, 2);
    // Lower staff: 1/4 + 1 bar; a single section of 5/4 bar is not short.
    let (lower, _, outcome) = solved(&p, "2");
    // The declared all-boundaries capability also offers the upper onsets.
    // They divide the lower sustained head's full bar into three exact thirds.
    assert_eq!(lower.layers[1..].len(), 3);
    assert!(lower.layers[1..].iter().all(|layer| layer.duration == k::Time::new(1, 3).unwrap()));
    assert_eq!(outcome.incumbent_cost.0[7], 0);
}

#[test]
fn divisions_change_keeps_exact_time() {
    let m1 = head_attrs(2, 4, 4, "F") + &note_xml(&n('C', 5, 8, 1)) + &backup(8) + &note_xml(&n('C', 3, 8, 2));
    let m2 = format!("<attributes><divisions>6</divisions></attributes>{}{}{}", note_xml(&n('C', 5, 6, 1)) + &note_xml(&n('D', 5, 18, 1)), backup(24), note_xml(&n('C', 3, 24, 2)));
    let xml = score(&[m1, m2]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    let (domain, anchors, _) = solved(&p, "1");
    assert_eq!(anchors.iter().map(|a| a.at).collect::<Vec<_>>(), vec![q(0, 1), q(4, 1), q(5, 1)]);
    assert_eq!(domain.layers[1].duration, k::Time::new(1, 4).unwrap());
    assert_eq!(domain.layers[2].duration, k::Time::new(3, 4).unwrap());
}

#[test]
fn carried_field_mismatch_is_a_binding_error() {
    let m1 = head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F']);
    let xml = score(&[m1]);
    let mut heads = carried(&xml);
    heads[1].onset += 1;
    assert!(matches!(propose(&xml, &heads, &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
    let mut heads = carried(&xml);
    heads.pop();
    assert!(matches!(propose(&xml, &heads, &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}

#[test]
fn fifteen_is_two_octaves_and_unsupported_size_is_unknown() {
    let upper = shift("down", 15, 1, None) + &note_xml(&n('C', 8, 4 * DIV, 1)) + &shift("stop", 15, 1, None);
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1.clone(), plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    assert_eq!(solved(&p, "1").0.layers[0].incumbent, st(3));
    let bad = xml.replace("size=\"15\"", "size=\"22\"");
    let p = propose(&bad, &carried(&bad), &[], &caps_all(&bad), None).unwrap();
    assert!(unknown(&p, "1").contains("size"));
}

#[test]
fn deep_bass_under_treble_clef_is_improved_to_bass_clef() {
    let lower = clef(2, "G") + &note_xml(&n('C', 2, 4 * DIV, 2));
    let m1 = head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, 4 * DIV, 1)) + &backup(4 * DIV) + &lower;
    let m2 = plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F']);
    let xml = score(&[m1, m2]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    let (domain, _, outcome) = solved(&p, "2");
    assert_eq!(domain.layers[0].incumbent, st(0), "initial F, then G at the same time 0 normalizes to G");
    assert!(outcome.improved);
    assert!(outcome.selected.iter().all(|s| *s == st(5)));
    assert!(outcome.selected_cost < outcome.incumbent_cost);
}

#[test]
fn upper_staff_without_clef_capability_never_offers_bass_clef() {
    let upper = note_xml(&n('C', 3, 4 * DIV, 1));
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    let mut caps = caps_all(&xml);
    caps.staves.insert("1".into(), cap((0..5).map(st).collect(), &xml));
    let p = propose(&xml, &carried(&xml), &[], &caps, None).unwrap();
    let (domain, _, outcome) = solved(&p, "1");
    assert!(domain.layers.iter().all(|l| l.allowed.iter().all(|s| s.index() < 5)));
    assert!(outcome.selected.iter().all(|s| s.index() < 5));
}

#[test]
fn unsupported_clef_and_grace_are_not_guessed() {
    let m1 = head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, 2 * DIV, 1)) + &clef(1, "C") + &note_xml(&n('D', 5, 2 * DIV, 1)) + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    assert!(unknown(&p, "1").contains("clef"));
    let mut g = n('B', 4, 0, 1);
    g.grace = true;
    let m1 = head_attrs(DIV, 4, 4, "F") + &note_xml(&g) + &note_xml(&n('C', 5, 4 * DIV, 1)) + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    assert!(matches!(propose(&xml, &carried(&xml), &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}

#[test]
fn primary_beam_allows_octave_but_not_clef_change_inside_by_default() {
    // Quarter, then a beamed pair on beats 2-3, then a quarter.
    let upper = note_xml(&n('C', 5, DIV, 1)) + &note_xml(&n('D', 5, DIV, 1)) + &note_xml(&n('E', 5, DIV, 1)) + &note_xml(&n('F', 5, DIV, 1));
    let m1 = head_attrs(DIV, 4, 4, "F") + &upper + &backup(4 * DIV) + &note_xml(&n('C', 3, 4 * DIV, 2));
    let xml = score(&[m1]);
    let groups = vec![AdapterGroup { kind: AdapterGroupKind::PrimaryBeam, members: vec![1, 2] }];
    let p = propose(&xml, &carried(&xml), &groups, &caps_all(&xml), None).unwrap();
    let (domain, _, _) = solved(&p, "1");
    assert!(!domain.layers[1].group_crossing, "boundary at the first member is outside");
    assert!(domain.layers[2].group_crossing, "boundary at the last member splits the group");
    assert!(!domain.layers[2].incoming[idx(st(0))][idx(st(5))]);
    assert!(domain.layers[2].incoming[idx(st(0))][idx(st(1))]);
    assert!(!domain.layers[3].group_crossing, "boundary after the last member is outside");
}

#[test]
fn appending_independent_bars_does_not_change_an_unimproved_course() {
    let lower = clef(2, "G") + &note_xml(&n('C', 2, 4 * DIV, 2));
    let base = head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, 4 * DIV, 1)) + &backup(4 * DIV) + &lower;
    let one = score(&[base.clone(), plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let two = score(&[base.clone(), plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F']), plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let a = solved(&propose(&one, &carried(&one), &[], &caps_all(&one), None).unwrap(), "1").2;
    let b = solved(&propose(&two, &carried(&two), &[], &caps_all(&two), None).unwrap(), "1").2;
    assert!(!a.improved && !b.improved);
    assert!(a.selected.iter().all(|s| *s == st(0)) && b.selected.iter().all(|s| *s == st(0)));
}

#[test]
fn tie_continuation_without_carried_group_is_a_binding_error() {
    let mut a = n('C', 3, 2 * DIV, 2);
    a.tie = Some("start");
    let mut b = n('C', 3, 2 * DIV, 2);
    b.tie = Some("stop");
    let m1 = head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, 4 * DIV, 1)) + &backup(4 * DIV) + &note_xml(&a) + &note_xml(&b);
    let xml = score(&[m1]);
    assert!(matches!(propose(&xml, &carried(&xml), &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}


#[test]
fn duplicate_carried_locators_cannot_hide_a_missing_head() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let mut heads = carried(&xml);
    heads[1] = heads[0].clone();
    assert!(matches!(propose(&xml, &heads, &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}

#[test]
fn head_locator_must_end_at_its_own_container() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let mut heads = carried(&xml);
    heads[0].span.end = heads[1].span.end;
    assert!(matches!(propose(&xml, &heads, &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}

#[test]
fn mid_measure_meter_change_is_not_reinterpreted_as_last_meter() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, DIV, 1))
        + "<attributes><time><beats>3</beats><beat-type>4</beat-type></time></attributes>"
        + &note_xml(&n('D', 5, 3 * DIV, 1))]);
    assert!(matches!(propose(&xml, &carried(&xml), &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))),
        "an unsupported piecewise meter cannot use the last signature for the whole bar");
}

#[test]
fn rational_comparison_does_not_overflow_near_integer_limits() {
    let larger = q(i128::MAX - 1, i128::MAX);
    let smaller = q(i128::MAX - 2, i128::MAX - 1);
    assert!(larger > smaller);
    assert!(q(-1, 2) < q(0, 1));
    assert!(q(-2, 3) < q(-1, 2));
}

#[test]
fn minimum_signed_offset_is_checked_without_panicking() {
    assert!(Q::new(i128::MIN, 1).is_none(), "unrepresentable signed magnitude is unsupported");
}

#[test]
fn nonpositive_note_duration_is_not_a_legal_clock() {
    for duration in [0, -1] {
        let xml = score(&[head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, duration, 1))
            + &note_xml(&n('D', 5, 8, 1))]);
        assert!(matches!(propose(&xml, &carried(&xml), &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
    }
}

#[test]
fn duplicate_members_do_not_prove_a_carried_group() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let groups = vec![AdapterGroup { kind: AdapterGroupKind::TiedAttack, members: vec![0, 0] }];
    assert!(matches!(propose(&xml, &carried(&xml), &groups, &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}


#[test]
fn carried_visibility_is_checked_against_its_original_container() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let mut heads = carried(&xml);
    heads[0].visible = false;
    assert!(matches!(propose(&xml, &heads, &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}

#[test]
fn register_event_beyond_piece_end_cannot_disappear_from_the_incumbent() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &shift("down", 8, 1, Some(32))
        + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    assert!(unknown(&p, "1").contains("outside"));
    solved(&p, "2");
}

#[test]
fn terminal_clef_change_requires_a_real_exit_model() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])
        + &clef(1, "F")]);
    let p = propose(&xml, &carried(&xml), &[], &caps_all(&xml), None).unwrap();
    assert!(unknown(&p, "1").contains("terminal"));
    solved(&p, "2");
}


#[test]
fn octave_start_and_stop_capabilities_are_directional() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let mut capabilities = caps_all(&xml);
    let lower = capabilities.staves.get_mut("2").unwrap();
    lower.boundaries.insert(q(1, 1), BoundaryCapability { octave_start: true, ..Default::default() });
    let proposal = propose(&xml, &carried(&xml), &[], &capabilities, None).unwrap();
    let domain = solved(&proposal, "2").0;
    let edges = domain.layers[1].incoming;
    assert!(edges[idx(st(5))][idx(st(6))], "loco to 8va needs only a start");
    assert!(!edges[idx(st(6))][idx(st(5))], "8va to loco needs a stop");
    assert!(!edges[idx(st(6))][idx(st(7))], "8va to 8vb needs both stop and start");
    assert!(!edges[idx(st(5))][idx(st(0))], "a clef switch needs its own anchor");
    assert!(edges[idx(st(6))][idx(st(6))], "continuation needs no new boundary");
}

#[test]
fn nominal_states_without_boundary_evidence_do_not_grant_new_edges() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let mut capabilities = caps_all(&xml);
    let lower = capabilities.staves.get_mut("2").unwrap();
    lower.boundaries.clear(); lower.entry = vec![st(5)]; lower.exit_octave_stop = false;
    let proposal = propose(&xml, &carried(&xml), &[], &capabilities, None).unwrap();
    let domain = solved(&proposal, "2").0;
    assert_eq!(domain.layers[0].allowed, vec![st(5)]);
    for a in 0..10 { for b in 0..10 {
        assert_eq!(domain.layers[1].incoming[a][b], a == b);
    }}
    assert!(!domain.exit_allowed[idx(st(6))]);
    assert!(domain.exit_allowed[idx(st(5))]);
}

#[test]
fn capability_source_epoch_is_exact_not_a_note_count_or_length() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let other = xml.replace("<step>C</step>", "<step>D</step>");
    assert_eq!(xml.len(), other.len());
    assert!(matches!(propose(&other, &carried(&other), &[], &caps_all(&xml), None), Err(AdapterError::Binding(_))));
}

#[test]
fn adjacent_chord_containers_have_distinct_before_and_after_anchor_times() {
    let mut member = n('E', 5, DIV, 1); member.chord = true;
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 5, DIV, 1)) + &note_xml(&member)]);
    let heads = carried(&xml);
    assert_eq!(heads[0].span.end, heads[1].span.start);
    let clock = observe_clock(&xml).unwrap();
    assert_eq!(clock.times[&(heads[0].span.end, true)], q(1, 1));
    assert_eq!(clock.times[&(heads[1].span.start, false)], q(0, 1));
    assert_eq!(clock.times[&(heads[1].span.end, true)], q(1, 1));
}

#[test]
fn unremovable_original_clef_is_a_hard_layer_constraint() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &plain_bar(['C', 'D', 'E', 'F'], ['C', 'D', 'E', 'F'])]);
    let mut capabilities = caps_all(&xml);
    capabilities.staves.get_mut("2").unwrap().fixed_clefs.insert(q(1, 1), 1);
    let proposal = propose(&xml, &carried(&xml), &[], &capabilities, None).unwrap();
    assert!(solved(&proposal, "2").0.layers[1].allowed.iter().all(|state| state.index() >= 5));
}

/// Synthetic owner input: fixture IDs are assigned here, never by product XML traversal.
fn fixture_owner_input(xml: &str) -> crate::pipeline::JointReadingOwnerInput {
    let heads = carried(xml).into_iter().enumerate().map(|(i, h)| crate::pipeline::JointReadingOwnerHead {
        source_event_id: crate::pipeline::MaterialSourceEventId(i),
        attack_id: crate::pipeline::SoundingAttackId(i), measure: h.measure,
        onset: h.onset, duration: 1, staff: h.staff, visible: h.visible,
        diatonic: h.diatonic, continuation: h.continuation, source_span: h.span,
    }).collect();
    crate::pipeline::JointReadingOwnerInput { source: std::sync::Arc::from(xml), heads,
        groups: Vec::new(), unknown_groups: BTreeMap::new(),
        tonal: crate::pipeline::derive_tonal_spelling_decision(xml, &crate::pipeline::parse_atomic_notes(xml)) }
}

#[test]
fn original_rest_host_closes_chord_context_without_a_beam_head() {
    for orphan_staff in [1, 2] { for copies in [1, 2] {
        let independent = 3 - orphan_staff;
        let mut orphan = n('E', 4, 4 * DIV, orphan_staff); orphan.chord = true; orphan.hidden = true;
        let independent_notes = (0..8).map(|i| note_xml(&n(if i%2==0 { 'C' } else { 'D' }, 4, 1, independent))
            .replace("</voice>", "</voice><type>eighth</type>")).collect::<String>();
        let xml = score(&[head_attrs(DIV, 4, 4, "F") + &rest(4*DIV, orphan_staff)
            + &note_xml(&orphan).repeat(copies) + &backup(4*DIV) + &independent_notes]);
        let (_, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&xml);
        let source = capture.current.as_ref();
        let mut owner = fixture_owner_input(source);
        owner.bind_original_groups(&capture).unwrap();
        assert!(owner.unknown_groups.is_empty());
        let attachment = &capture.chord_attachments(source).unwrap()[0];
        assert!(attachment.original_rest_host.is_some());
        assert!(attachment.original_head.is_none());
        assert!(attachment.head.is_none(), "a rest host is not a positive beam head");
        let mut lost = capture.as_ref().clone();
        lost.chord_attachments.as_mut().unwrap()[0].original_head_absent = false;
        assert!(fixture_owner_input(source).bind_original_groups(&lost).is_err());
        let mut capabilities = Capabilities::new(source);
        for staff in ["1", "2"] { capabilities.staves.insert(staff.into(), cap(all_states(), source)); }
        let proposal = owner.c3_proposal(&capabilities, None).unwrap();
        assert!(matches!(proposal.staves[&orphan_staff.to_string()], StaffVerdict::Solved(_)));
        assert!(matches!(proposal.staves[&independent.to_string()], StaffVerdict::Solved(_)));
        let mut missing = capture.as_ref().clone();
        for row in missing.chord_attachments.as_mut().unwrap() { row.original_rest_host = None; }
        let mut unbound = fixture_owner_input(source);
        unbound.bind_original_groups(&missing).unwrap();
        let proposal = unbound.c3_proposal(&capabilities, None).unwrap();
        assert!(matches!(&proposal.staves[&orphan_staff.to_string()], StaffVerdict::Unknown { reason }
            if reason.contains("no pitched physical head")));
        assert!(matches!(proposal.staves[&independent.to_string()], StaffVerdict::Solved(_)));
    }}
}

#[test]
fn original_rest_host_cross_staff_dependency_remains_unknown() {
    let mut member = n('E', 4, 4 * DIV, 2); member.voice = 1; member.chord = true; member.hidden = true;
    let independent = (0..8).map(|i| note_xml(&n(if i % 2 == 0 { 'C' } else { 'D' }, 4, 1, 1))
        .replace("</voice>", "</voice><type>eighth</type>")).collect::<String>();
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &rest(4 * DIV, 1) + &note_xml(&member)
        + &backup(4 * DIV) + &independent]);
    let (_, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&xml);
    let source = capture.current.as_ref();
    let mut owner = fixture_owner_input(source);
    owner.bind_original_groups(&capture).unwrap();
    assert!(owner.unknown_groups.values().all(|reason| reason.contains("crosses staff domains")));
    assert_eq!(owner.unknown_groups.len(), 2);
}

#[test]
fn original_block_fragment_carries_only_exterior_heads_and_refuses_pitched_edits() {
    let xml = score(&[head_attrs(DIV, 4, 4, "F") + &note_xml(&n('C', 4, DIV, 1))
        + &rest(DIV, 1) + &note_xml(&n('C', 4, 2*DIV, 1))]);
    let owner = fixture_owner_input(&xml);
    let notes = crate::pipeline::parse_atomic_notes(&xml);
    let rest = notes.iter().find(|note| note.pitch.is_none()).unwrap();
    let range = rest.note_start..rest.note_end;
    let post = xml[range.clone()].replace("</voice>", "</voice><type>quarter</type>");
    let after = [&xml[..range.start], &post, &xml[range.end..]].concat();
    let next = owner.continue_render_fragment_exterior(range.start, range.end, &post, &after).unwrap();
    assert_eq!(next.heads.len(), owner.heads.len());
    for (before, after) in owner.heads.iter().zip(&next.heads) {
        assert_eq!(before.source_event_id, after.source_event_id);
        assert_eq!(before.attack_id, after.attack_id);
    }
    assert_ne!(owner.heads[1].source_span, next.heads[1].source_span);
    let head = &owner.heads[0].source_span;
    assert!(owner.continue_render_fragment_exterior(head.start, head.end, &xml[head.clone()], &xml).is_err());
    assert!(owner.continue_render_fragment_exterior(range.start, range.end, &post, &(after.clone()+" ")).is_err());
    let added = post.clone()+&note_xml(&n('D', 4, DIV, 1));
    let added_document = [&xml[..range.start], &added, &xml[range.end..]].concat();
    assert!(owner.continue_render_fragment_exterior(range.start, range.end, &added, &added_document).is_err());
}
