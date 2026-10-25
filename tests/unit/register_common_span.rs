use super::*;
use crate::model::{Event, ExpectedDuration, Measure, Part, Pitch};

fn fixture(midis: &[i32]) -> Score {
    let steps = ["C", "C", "D", "D", "E", "F", "F", "G", "G", "A", "A", "B"];
    let alters = [0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0];
    let mut events = midis.iter().enumerate().map(|(index, midi)| Event {
        kind: EventKind::Note, staff: "1".into(), voice: "Upper".into(),
        start: index as i64 * 24, duration: 24,
        pitch: Some(Pitch::new(steps[midi.rem_euclid(12) as usize].into(),
            alters[midi.rem_euclid(12) as usize], midi / 12 - 1)),
        is_chord: false, tie_stop: false, visible: true, note_type: None, dots: 0,
    }).collect::<Vec<_>>();
    events.push(Event { kind: EventKind::Rest, staff: "1".into(), voice: "Upper".into(),
        start: midis.len() as i64 * 24, duration: 768 - midis.len() as i64 * 24,
        pitch: None, is_chord: false, tie_stop: false, visible: true, note_type: None, dots: 0 });
    Score { title: None, parts: vec![Part { id: "Local".into(), name: "Probe".into(),
        measures: vec![Measure { number: "SyntheticAlpha".into(), divisions: 96,
            beats: 8, beat_type: 4, expected: ExpectedDuration::new(96, 8, 4),
            cursor_min: 0, cursor_max: 768, events }] }] }
}

fn coherent() -> Score { fixture(&[90, 90, 90, 90, 102, 102, 90, 90]) }

fn selected(proof: &RegisternotationsBeweis) -> usize {
    proof.gemeinsame_spannen.iter().filter(|v| v.status == Status::Uebernommen).count()
}

#[test]
fn full_owner_selects_a_strictly_improving_common_span() {
    let proof = analysiere_score(&coherent()).unwrap();
    assert_eq!(selected(&proof), 1);
    assert_ne!(proof.plan_fingerabdruck, proof.basis_plan_fingerabdruck);
    assert_eq!(proof.plaene.len(), 1);
    assert_eq!((proof.plaene[0].beginn, proof.plaene[0].ende), (0, 192));
    assert_eq!(proof.plaene[0].darstellung, RegisternotationsDarstellung::Fuenfzehnma);
    assert!(proof.gemeinsame_spannen[0].kosten.is_some_and(|(old, new)| new < old));
}

#[test]
fn coverage_is_part_of_search_instead_of_losing_a_second_island() {
    let score = fixture(&[88, 89, 90, 91, 60, 61, 62, 63, 88, 89, 90, 91]);
    let proof = analysiere_score(&score).unwrap();
    assert_eq!(selected(&proof), 0);
    assert_eq!(proof.plaene.len(), 2);
    assert_eq!(proof.plan_fingerabdruck, proof.basis_plan_fingerabdruck);
}

#[test]
fn required_coverage_selects_the_best_feasible_window() {
    let score = fixture(&[88, 89, 90, 91, 72, 73, 74, 75, 102, 103, 104, 105]);
    let time = kanonische_taktzeiten(&score).unwrap();
    let events = beobachte_ereignisse(&score, time.skalierung, &time.zeiten).unwrap();
    let refs = events.iter().collect::<Vec<_>>();
    let key = SystemFolgeSchluessel { teil: "Local".into(), system: "1".into() };
    let required = events.iter().map(|e| e.id).collect();
    let plan = plane_segment_mit_pflichtabdeckung(&key, &refs, &[], &required).unwrap();
    assert_eq!((plan.beginn, plan.ende), (0, 288));
    assert!(required.iter().all(|id| plan.betroffene_ereignisse.contains(id)));
}

#[test]
fn independent_positive_and_negative_parts_are_not_globally_ranked() {
    for original in [coherent(), fixture(&[88, 89, 90, 91, 60, 61, 62, 63, 88, 89, 90, 91])] {
        let once = analysiere_score(&original).unwrap();
        let mut twice = original.clone();
        let mut copy = twice.parts[0].clone(); copy.id = "Independent".into();
        twice.parts.push(copy);
        let doubled = analysiere_score(&twice).unwrap();
        assert_eq!(selected(&doubled), selected(&once) * 2);
        let retained = doubled.plaene.iter().filter(|plan| plan.teil == "Local").cloned().collect::<Vec<_>>();
        assert_eq!(retained, once.plaene);
    }
}

#[test]
fn equivalent_time_scale_and_measure_labels_preserve_local_choice() {
    let first = analysiere_score(&coherent()).unwrap();
    let mut score = coherent();
    for part in &mut score.parts { for measure in &mut part.measures {
        measure.number = "RenamedSynthetic".into(); measure.divisions *= 4;
        measure.expected = ExpectedDuration::new(measure.divisions, measure.beats, measure.beat_type);
        measure.cursor_max *= 4;
        for event in &mut measure.events { event.start *= 4; event.duration *= 4; }
    } }
    let second = analysiere_score(&score).unwrap();
    assert_eq!(selected(&first), selected(&second));
    assert_eq!(first.gemeinsame_spannen[0].kosten, second.gemeinsame_spannen[0].kosten);
    assert_eq!(second.plaene[0].ende, first.plaene[0].ende * 4);
}

#[test]
fn genuine_excursion_readable_low_tail_and_double_octave_remain_unchanged() {
    for midis in [&[64, 65, 66, 67, 112, 113, 114, 115][..],
        &[90, 91, 92, 93, 64, 65, 66, 67], &[120, 121, 122, 123]] {
        let proof = analysiere_score(&fixture(midis)).unwrap();
        assert_eq!(selected(&proof), 0);
        assert_eq!(proof.plan_fingerabdruck, proof.basis_plan_fingerabdruck);
    }
}

#[test]
fn long_rest_remains_a_fence_between_independent_choices() {
    let mut score = fixture(&[90, 90, 90, 90, 102, 102, 90, 90,
        90, 90, 90, 90, 102, 102, 90, 90]);
    let events = &mut score.parts[0].measures[0].events;
    for event in events.iter_mut().skip(8) { event.start += 96; }
    events.last_mut().unwrap().duration -= 96;
    let proof = analysiere_score(&score).unwrap();
    assert_eq!(selected(&proof), 2);
    assert!(proof.plaene.iter().all(|plan| plan.ende <= 192 || plan.beginn >= 288));
}

#[test]
fn incoming_tie_and_following_measure_continuation_refuse_unsafe_boundaries() {
    let mut incoming = coherent(); incoming.parts[0].measures[0].events[0].tie_stop = true;
    let proof = analysiere_score(&incoming).unwrap();
    assert_eq!(selected(&proof), 0);
    assert!(proof.gemeinsame_spannen.iter().any(|v| v.status == Status::GrenzeUnsicher));
    let mut outgoing = coherent();
    let first = &mut outgoing.parts[0].measures[0];
    first.events.pop();
    for event in &mut first.events { event.start *= 4; event.duration *= 4; }
    let mut continuation = fixture(&[64]).parts.remove(0).measures.remove(0);
    continuation.number = "ContinuationSynthetic".into(); continuation.events[0].tie_stop = true;
    outgoing.parts[0].measures.push(continuation);
    let proof = analysiere_score(&outgoing).unwrap();
    assert_eq!(selected(&proof), 0);
    assert!(proof.gemeinsame_spannen.iter().any(|v| v.status == Status::GrenzeUnsicher));
}

#[test]
fn sounding_overlap_is_enclosed_without_an_internal_cut() {
    let mut score = coherent(); score.parts[0].measures[0].events[0].duration = 120;
    let proof = analysiere_score(&score).unwrap();
    assert_eq!(selected(&proof), 1);
    assert!(proof.plaene.iter().all(|plan| !(0 < plan.beginn && plan.beginn < 120)
        && !(0 < plan.ende && plan.ende < 120)));
}

#[test]
fn hidden_chord_carrier_and_mixed_durations_keep_complete_coverage() {
    let mut score = coherent();
    let events = &mut score.parts[0].measures[0].events;
    let mut hidden = events[0].clone(); hidden.visible = false; hidden.is_chord = true;
    hidden.pitch = Some(Pitch::new("G".into(), 0, 9)); events.insert(1, hidden);
    events[4].duration += 24;
    for event in events.iter_mut().skip(5) { event.start += 24; }
    events.last_mut().unwrap().duration -= 24;
    let proof = analysiere_score(&score).unwrap();
    assert_eq!(selected(&proof), 1);
    assert_eq!(proof.plaene[0].betroffene_ereignisse.len(), 9);
    assert!(geschlossen(&pruefe_topologie(&proof.ereignisse, &proof.plaene)));
}

#[test]
fn lower_clef_authority_and_subminimum_material_are_not_invented() {
    let mut lower = coherent();
    for event in &mut lower.parts[0].measures[0].events { event.staff = "2".into(); }
    let proof = analysiere_score(&lower).unwrap();
    assert_eq!(selected(&proof), 0);
    assert_eq!(proof.plan_fingerabdruck, proof.basis_plan_fingerabdruck);
    assert!(proof.gemeinsame_spannen.iter().any(|v| v.status == Status::AlternativeSchluesselAutoritaetUngeklaert));
    let proof = analysiere_score(&fixture(&[108, 109, 121])).unwrap();
    assert!(proof.plaene.is_empty()); assert_eq!(selected(&proof), 0);
}

#[test]
fn open_baseline_dependency_is_unknown_instead_of_a_partial_replacement() {
    let proof = analysiere_score(&coherent()).unwrap();
    let mut baseline = proof.plaene.clone();
    baseline[0].entscheidungskontext.push(RegisternotationsEreignisId {
        teil_ordinal: usize::MAX, takt_ordinal: 0, ereignis_ordinal: 0,
    });
    let sequences = gruppiere_systemfolge(&proof.ereignisse);
    let (plans, witnesses) = select_closed_common_spans(&sequences, &proof.grenzen, &proof.ereignisse, baseline.clone());
    assert_eq!(plans, baseline);
    assert!(witnesses.iter().any(|v| v.status == Status::KontextOffen));
}
