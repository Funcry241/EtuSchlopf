use super::*;

fn bound_fragment(tie_stop: bool, measure: usize, onset: i64) -> (FinalChordSourceBindingRow, CandidateMaterialAttackState) {
    let source = MaterialSourceEventId(73);
    let attack_id = SoundingAttackId(211);
    let attack = CandidateMaterialAttackState {
        source_event_id: source, attack_id, exists: true, measure: 17, onset: 0,
        sounding_end: 4, pitch: 76, staff: "1".into(), voice: "1".into(),
    };
    let fragment = CandidateMaterialFragmentState {
        measure, raw_note_index: 2, onset, duration: 2, pitch: Some(76),
        staff: "1".into(), voice: "1".into(), visible: true, is_chord: true,
        tie_start: !tie_stop, tie_stop, source_event_id: Some(source), attack_id: Some(attack_id),
    };
    (FinalChordSourceBindingRow { fragment_id: 97, measure, raw_note_index: 2, source_onset: onset, source_pitch: Some(76), fragment }, attack)
}

#[test]
fn unchanged_tie_continuations_keep_the_attack_origin_across_fragment_scopes() {
    for (measure, onset) in [(17, 2), (18, 0)] {
        let (binding, attack) = bound_fragment(true, measure, onset);
        let original = attack.clone();
        assert!(final_chord_forward_attack_translation(&binding, &attack, onset, onset).unwrap().is_none());
        assert_eq!(attack, original);
    }
}

#[test]
fn a_continuation_cannot_authorize_a_new_attack_translation() {
    let (binding, attack) = bound_fragment(true, 17, 2);
    assert!(final_chord_forward_attack_translation(&binding, &attack, 2, 3).is_err());
}

#[test]
fn real_head_translation_retains_exact_source_and_attack_identity() {
    let (binding, attack) = bound_fragment(false, 17, 0);
    assert!(final_chord_forward_attack_translation(&binding, &attack, 0, 0).unwrap().is_none());
    let row = final_chord_forward_attack_translation(&binding, &attack, 0, 1).unwrap().unwrap();
    assert_eq!(row.source_event_id, attack.source_event_id);
    assert_eq!(row.attack_id, attack.attack_id);
    assert_eq!(row.target_onset, 1);
}

#[test]
fn representation_only_is_not_permission_for_missing_or_mismatched_identity() {
    let (binding, attack) = bound_fragment(true, 17, 2);
    for variant in 0..9 {
        let mut b = binding.clone(); let mut a = attack.clone();
        match variant {
            0 => a.exists = false,
            1 => b.fragment.source_event_id = None,
            2 => b.fragment.attack_id = Some(SoundingAttackId(212)),
            3 => b.fragment.pitch = Some(75),
            4 => b.fragment.onset = 3,
            5 => b.fragment.measure = 19,
            6 => b.fragment.raw_note_index = 3,
            7 => b.fragment.duration = 0,
            _ => b.fragment.tie_stop = false,
        }
        assert!(final_chord_forward_attack_translation(&b, &a, 2, 2).is_err(), "variant {variant}");
    }
}

// The actual planner/projector audit rows pass through the actual pipeline
// source-range binder and material-state classifier. No real corpus fixture.
#[test]
fn tied_cross_staff_projection_reaches_the_production_material_identity_bridge() {
    for scope in [17, 43] { for unit in [1, 3] { for staff in [1, 2] { for copies in [1, 2] {
        let mut body = String::new();
        for event in 0..2 * copies {
            let tie = if event % 2 == 0 { "start" } else { "stop" };
            let duration = 2 * unit;
            let other = 3 - staff;
            body.push_str(&format!(r#"<note><pitch><step>C</step><octave>3</octave></pitch><duration>{duration}</duration><voice>1</voice><staff>{staff}</staff></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>{duration}</duration><tie type="{tie}"/><voice>1</voice><staff>{other}</staff><notations><tied type="{tie}"/></notations></note>"#));
        }
        let xml = format!(r#"<score-partwise><part id="synthetic"><measure number="{scope}"><attributes><divisions>{unit}</divisions><time><beats>{}</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>{body}</measure></part></score-partwise>"#,4*copies);
        let notes = parse_atomic_notes(&xml);
        let mut evidence = Vec::new(); let mut fragments = BTreeMap::new(); let mut ids = BTreeMap::new(); let mut ranges = FinalChordSourceByRange::new(); let mut attacks = BTreeMap::new();
        for (index, n) in notes.iter().enumerate() {
            let source_id = if n.is_chord { 100 + index / 4 } else { 500 + index };
            let attack_id = source_id + 1000;
            let pitch = n.pitch.as_ref().unwrap().midi;
            let f = CandidateMaterialFragmentState { measure: scope, raw_note_index: index, onset: n.start, duration: n.duration, pitch: Some(pitch), staff: n.staff.clone(), voice: n.voice.clone(), visible: n.visible, is_chord: n.is_chord, tie_start: n.tie_start, tie_stop: n.tie_stop, source_event_id: Some(MaterialSourceEventId(source_id)), attack_id: Some(SoundingAttackId(attack_id)) };
            evidence.push(structural_write_authority::VoiceFreeFinalChordEvidenceRow { fragment_id: index+1, source_event_id: source_id, attack_id, measure: scope, raw_note_index: index, onset: n.start, duration: n.duration, pitch, staff: n.staff.parse().unwrap(), chord_syntax: n.is_chord, tie_start: n.tie_start, tie_stop: n.tie_stop });
            attacks.entry(MaterialSourceEventId(source_id)).or_insert(CandidateMaterialAttackState { source_event_id: MaterialSourceEventId(source_id), attack_id: SoundingAttackId(attack_id), exists: true, measure: scope, onset: n.start, sounding_end: n.start + if n.tie_start { 2*n.duration } else { n.duration }, pitch, staff: n.staff.clone(), voice: n.voice.clone() });
            fragments.insert((scope,index),f); ids.insert((scope,index),index+1); ranges.insert((n.note_start,n.note_end),(scope,index,n.start,Some(pitch)));
        }
        let (_, plan) = structural_write_authority::plan_canonical_final_chord_structure_once(&xml,&evidence).unwrap();
        let projected = structural_write_authority::project_final_chord_encoding_plan_with_provenance(&xml,&plan).unwrap();
        assert!(!projected.provenance.is_empty());
        let before = attacks.clone(); let mut cache = BTreeMap::new(); let mut covered = 0;
        for row in plan.forward_materializations() {
            let binding = final_chord_source_binding(&mut cache,(row.source_note_start,row.source_note_end),&ranges,&fragments,&ids,"missing source","missing fragment").unwrap();
            let attack = &attacks[&binding.fragment.source_event_id.unwrap()];
            assert!(final_chord_forward_attack_translation(&binding,attack,row.source_onset,row.projected_sounding_onset).unwrap().is_none());
            covered += 1;
        }
        assert_eq!(covered,2*copies);
        assert_eq!(before,attacks);
    } } } }
}
