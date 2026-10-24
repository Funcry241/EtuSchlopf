use super::*;

fn fixture(staff: u8, unit: i64, voice: &str, tied: bool, mixed: bool) -> String {
    let other = if mixed { 3 - staff } else { staff };
    let scope = 83;
    let duration = if tied { 2 * unit } else { 4 * unit };
    let mut body = String::new();
    for event in 0..if tied { 2 } else { 1 } {
        let ties = if tied {
            let kind = if event == 0 { "start" } else { "stop" };
            format!("<tie type=\"{kind}\"/>")
        } else { String::new() };
        let notations = if tied {
            let kind = if event == 0 { "start" } else { "stop" };
            format!("<notations><tied type=\"{kind}\"/></notations>")
        } else { String::new() };
        body.push_str(&format!(r#"<note><pitch><step>C</step><octave>3</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note><note><chord/><pitch><step>G</step><octave>3</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>{duration}</duration>{ties}<voice>{voice}</voice><staff>{other}</staff>{notations}</note>"#));
    }
    format!(r#"<score-partwise version="4.0"><part-list><score-part id="synthetic"><part-name>Piano</part-name></score-part></part-list><part id="synthetic"><measure number="{scope}"><attributes><divisions>{unit}</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number="1"><sign>G</sign><line>2</line></clef><clef number="2"><sign>F</sign><line>4</line></clef></attributes>{body}</measure></part></score-partwise>"#)
}

fn run(xml: &str) -> (CanonicalFinalChordStructuralPlan, String) {
    let evidence = final_chord_declassified_evidence_from_xml(xml).unwrap();
    let (canonical, _) = plan_canonical_final_chord_structure_once(xml, &evidence).unwrap();
    let mut attacks = std::collections::BTreeMap::<usize, FrozenVoiceAttackEvidenceRow>::new();
    for row in &evidence {
        let offset = canonical.measures.iter().filter(|m| m.measure < row.measure).map(|m| m.expected_duration).sum::<i64>();
        let entry = attacks.entry(row.source_event_id).or_insert(FrozenVoiceAttackEvidenceRow {
            source_event_id: row.source_event_id, attack_id: row.attack_id,
            absolute_start: offset + row.onset, absolute_end: offset + row.onset + row.duration,
            staff: row.staff, technical_lane: 1,
        });
        entry.absolute_start = entry.absolute_start.min(offset + row.onset);
        entry.absolute_end = entry.absolute_end.max(offset + row.onset + row.duration);
    }
    let attacks = attacks.into_values().collect::<Vec<_>>();
    let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
    let labels = bind_musicxml_voice_labels(&channels, &[]).unwrap();
    let bound = bind_canonical_final_chord_encoding_plan_with_optional_render_omissions_bound(xml, &canonical, &labels, None).unwrap();
    let projected = project_canonical_bound_final_chord_encoding_plan_for_transaction_output_alpha(xml, &canonical, &bound).unwrap();
    verify_canonical_final_chord_transaction_local_output_alpha(
        &projected.xml, &canonical, &labels, None, &projected.receipt,
        bound.plan.source_fingerprint, bound.plan.source_len,
    ).unwrap();
    assert!(audit_musicxml_voice_label_binding_alpha(xml, &canonical, &labels).unwrap().green());
    (canonical, projected.xml)
}

type SemanticEvent = (usize, i64, i64, Option<i32>, String, bool, bool);

fn semantic_events(xml: &str) -> Vec<SemanticEvent> {
    let mut rows = final_chord_parse_binding_measures(xml).unwrap().iter().flat_map(|measure| {
        measure.notes.iter().filter(|n| !n.rest_like).map(|n| {
            (n.measure, n.onset, n.duration, n.pitch_midi, n.staff.clone(), n.tie_start, n.tie_stop)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    rows.sort(); rows
}

#[test]
fn bound_cross_staff_members_use_existing_independent_representation() {
    for staff in [1, 2] { for unit in [1, 3, 7] { for voice in ["1", "23"] {
        let xml = fixture(staff, unit, voice, false, true);
        let (canonical, output) = run(&xml);
        assert_eq!(semantic_events(&xml), semantic_events(&output));
        assert_eq!(canonical.actions.iter().filter(|row| matches!(row.kind, FinalChordCanonicalActionKind::AuxiliaryLane { .. })).count(), 1);
        assert_eq!(canonical.hard_same_edges.len(), 1);
        let notes = final_chord_parse_binding_measures(&output).unwrap();
        for note in &notes[0].notes {
            if let Some(anchor) = note.anchor_index { assert_eq!(note.staff, notes[0].notes[anchor].staff); }
        }
        if staff == 2 && unit == 1 && voice == "1" { println!("NativeSyntheticSource:{xml}"); println!("NativeSyntheticXml:{output}"); }
    } } }
}

#[test]
fn existing_tie_endpoints_and_attack_binding_survive_staff_separation() {
    for staff in [1, 2] { for unit in [1, 5] {
        let xml = fixture(staff, unit, "31", true, true);
        let (canonical, output) = run(&xml);
        assert_eq!(semantic_events(&xml), semantic_events(&output));
        let aux = canonical.actions.iter().filter(|row| matches!(row.kind, FinalChordCanonicalActionKind::AuxiliaryLane { .. })).collect::<Vec<_>>();
        assert_eq!(aux.len(), 2);
        assert_eq!(aux[0].source_event_id, aux[1].source_event_id);
        assert_eq!(aux[0].attack_id, aux[1].attack_id);
        assert_ne!(aux[0].fragment_id, aux[1].fragment_id);
        if staff == 2 && unit == 1 { println!("NativeSyntheticTiedSource:{xml}"); println!("NativeSyntheticTiedXml:{output}"); }
    } }
}

#[test]
fn homogeneous_chords_keep_their_bound_hard_same_cohort() {
    for staff in [1, 2] {
        let xml = fixture(staff, 3, "41", false, false);
        let (canonical, output) = run(&xml);
        assert_eq!(semantic_events(&xml), semantic_events(&output));
        assert_eq!(canonical.hard_same_edges.len(), 2);
        assert!(canonical.actions.iter().all(|row| !matches!(row.kind, FinalChordCanonicalActionKind::AuxiliaryLane { .. })));
    }
}

#[test]
fn no_canonical_source_evidence_does_not_enable_a_new_projection() {
    let xml = fixture(2, 1, "1", false, true);
    assert!(plan_canonical_final_chord_structure_once(&xml, &[]).is_err());
}

#[test]
fn independent_measure_duplication_preserves_each_source_obligation() {
    let single = fixture(2, 3, "11", true, true);
    let start = single.find("<measure ").unwrap(); let end = single.find("</measure>").unwrap() + "</measure>".len();
    let body = &single[start..end];
    let mut twice = single[..start].to_string();
    twice.push_str(body); twice.push_str(&body.replace("83", "129")); twice.push_str(&single[end..]);
    let (_, output) = run(&twice);
    assert_eq!(semantic_events(&twice), semantic_events(&output));
    assert_eq!(semantic_events(&output).len(), 12);
}

#[test]
fn unequal_durations_keep_full_bound_extent_and_do_not_mint_cross_staff_host_edges() {
    for duration in [2, 4] {
        let unit = 1;
        let scope = 91;
        let xml = format!(r#"<score-partwise><part id="synthetic"><measure number="{scope}"><attributes><divisions>{unit}</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>4</duration><voice>1</voice><staff>2</staff></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>{duration}</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>"#);
        let (canonical, output) = run(&xml);
        assert_eq!(semantic_events(&xml), semantic_events(&output));
        assert!(canonical.hard_same_edges.is_empty());
    }
}

#[test]
fn independent_staff_extent_precedes_an_other_staff_tied_anchor_chain() {
    let scope = 97;
    let xml = format!(r#"<score-partwise><part id="synthetic"><measure number="{scope}"><attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>2</duration><tie type="start"/><voice>1</voice><staff>2</staff><notations><tied type="start"/></notations></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>C</step><octave>3</octave></pitch><duration>2</duration><tie type="stop"/><voice>1</voice><staff>2</staff><notations><tied type="stop"/></notations></note></measure></part></score-partwise>"#);
    let (canonical, output) = run(&xml);
    assert_eq!(semantic_events(&xml), semantic_events(&output));
    assert!(canonical.hard_same_edges.is_empty());
    assert!(canonical.actions.iter().any(|row| matches!(row.kind, FinalChordCanonicalActionKind::AuxiliaryLane { .. })));
    assert!(canonical.actions.iter().all(|row| !matches!(row.kind, FinalChordCanonicalActionKind::TiedAnchorChain { .. })));
}

#[test]
fn cursor_only_members_do_not_gain_an_independent_participating_head() {
    let xml = fixture(2, 1, "1", false, true).replacen("<note><chord/><pitch><step>E", "<note print-spacing=\"no\"><chord/><pitch><step>E", 1);
    let (canonical, output) = run(&xml);
    assert!(canonical.actions.iter().all(|row| !matches!(row.kind, FinalChordCanonicalActionKind::AuxiliaryLane { .. })));
    assert_eq!(semantic_events(&xml), semantic_events(&output));
    assert!(output.contains("print-spacing=\"no\""));
}

fn independent_cohorts_fixture(onset: i64) -> String {
    let first = fixture(2, 1, "1", false, true);
    let start = first.find("<note>").unwrap();
    let end = first.find("</measure>").unwrap();
    let second = first[start..end].replace("<step>C</step>", "<step>D</step>").replace("<step>G</step>", "<step>A</step>").replace("<step>E</step>", "<step>F</step>").replace("<duration>4</duration>", &format!("<duration>{}</duration>", 4 - onset));
    format!("{}<backup><duration>4</duration></backup><forward><duration>{onset}</duration></forward>{second}{}", &first[..end], &first[end..])
}

#[test]
fn coincident_independent_cohorts_keep_distinct_auxiliary_heads() {
    let xml = independent_cohorts_fixture(0);
    let (canonical, output) = run(&xml);
    assert_eq!(semantic_events(&xml), semantic_events(&output));
    let runs = canonical.actions.iter().filter_map(|row| match row.kind {
        FinalChordCanonicalActionKind::AuxiliaryLane { serialization_run_id, .. } => Some(serialization_run_id), _ => None,
    }).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(runs.len(), 2);
    assert_eq!(canonical.hard_same_edges.len(), 2); // only the two original lower cohorts
    println!("NativeSyntheticCoincidentSource:{xml}"); println!("NativeSyntheticCoincidentXml:{output}");
}

#[test]
fn overlapping_independent_cohorts_receive_nonoverlapping_runs() {
    let xml = independent_cohorts_fixture(1);
    let (canonical, output) = run(&xml);
    assert_eq!(semantic_events(&xml), semantic_events(&output));
    let runs = canonical.actions.iter().filter_map(|row| match row.kind {
        FinalChordCanonicalActionKind::AuxiliaryLane { serialization_run_id, .. } => Some(serialization_run_id), _ => None,
    }).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(runs.len(), 2);
    let measures = final_chord_parse_binding_measures(&output).unwrap();
    for note in &measures[0].notes {
        if let Some(anchor) = note.anchor_index { assert_eq!(note.staff, measures[0].notes[anchor].staff); }
    }
    println!("NativeSyntheticOverlappingSource:{xml}"); println!("NativeSyntheticOverlappingXml:{output}");
}

#[test]
fn proven_auxiliary_cohort_binds_to_its_actual_generated_head() {
    let xml = fixture(2, 1, "1", false, true).replace("<staff>2</staff></note><note><chord/><pitch><step>E", "<staff>1</staff></note><note><chord/><pitch><step>E");
    let (canonical, output) = run(&xml);
    assert_eq!(semantic_events(&xml), semantic_events(&output));
    assert_eq!(canonical.hard_same_edges.len(), 1);
    let aux = canonical.actions.iter().filter(|row| matches!(row.kind, FinalChordCanonicalActionKind::AuxiliaryLane { .. })).map(|row| row.fragment_id).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(aux.len(), 2);
    assert!(canonical.hard_same_edges.iter().all(|edge| aux.contains(&edge.left_fragment_id) && aux.contains(&edge.right_fragment_id)));
    println!("NativeSyntheticCohortSource:{xml}"); println!("NativeSyntheticCohortXml:{output}");
}
