use super::*;

// Own synthetic seed declarations, not source IDs inferred by production code.
fn synthetic_parts(
    xml: &str,
) -> (
    Vec<CandidateMaterialFragmentState>,
    Vec<CandidateMaterialAttackState>,
    Vec<(MaterialSourceEventId, coincidence_occupancy::RawPair)>,
) {
    let mut fragments = material_fragments_from_xml(xml).unwrap();
    let mut raw = Vec::new();
    let mut attacks = Vec::new();
    let mut prior = BTreeMap::new();
    for (index, row) in fragments.iter_mut().enumerate() {
        let Some(pitch) = row.pitch else {
            continue;
        };
        let relation = (row.staff.clone(), row.voice.clone(), pitch);
        if row.tie_stop {
            let &(source, attack) = prior
                .get(&relation)
                .expect("synthetic continuation has a declared predecessor");
            row.source_event_id = Some(source);
            row.attack_id = Some(attack);
        } else {
            let source = MaterialSourceEventId(index);
            let attack = SoundingAttackId(index);
            row.source_event_id = Some(source);
            row.attack_id = Some(attack);
            raw.push((
                source,
                coincidence_occupancy::RawPair {
                    onset: row.onset,
                    pitch,
                },
            ));
            attacks.push(CandidateMaterialAttackState {
                source_event_id: source,
                attack_id: attack,
                exists: true,
                is_grace: row.is_grace,
                measure: row.measure,
                onset: row.onset,
                sounding_end: row.onset + row.duration,
                pitch,
                staff: row.staff.clone(),
                voice: row.voice.clone(),
            });
        }
        if row.tie_start {
            prior.insert(
                relation,
                (row.source_event_id.unwrap(), row.attack_id.unwrap()),
            );
        }
    }
    (fragments, attacks, raw)
}

pub(super) fn legacy_fixture(xml: &str) -> GesturePlacementOccupancy {
    let (fragments, attacks, raw) = synthetic_parts(xml);
    GesturePlacementOccupancy::from_bound_parts(xml, &fragments, &attacks, raw).unwrap()
}

fn xml(measure: usize) -> String {
    format!("<score-partwise><part id='synthetic'><measure number='{measure}'><attributes><divisions>96</divisions></attributes>
    <note><pitch><step>E</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>
    <note print-object='no'><chord/><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>
    <note><pitch><step>G</step><octave>4</octave></pitch><duration>48</duration><voice>1</voice><staff>1</staff></note>
    <note><chord/><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>
    <note><pitch><step>A</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff></note>
    </measure></part></score-partwise>")
}

fn tone_and_slots(
    xml: &str,
) -> (
    HiddenChordToneRecoveryCandidate,
    Vec<VisibleChordToneSwapSlot>,
) {
    let notes = parse_atomic_notes(xml);
    let source = &notes[1];
    let tone = HiddenChordToneRecoveryCandidate {
        measure: source.measure.parse().unwrap(),
        staff: 1,
        start_tick: source.start,
        duration: source.duration,
        midi: source.pitch.as_ref().map(|p| p.midi),
        voice: source.voice.clone(),
        pitch_xml: String::new(),
        abs_start: source.note_start,
        abs_end: source.note_end,
        score: 0,
    };
    let slots = [2, 4]
        .map(|i| {
            let n = &notes[i];
            VisibleChordToneSwapSlot {
                measure: n.measure.parse().unwrap(),
                staff: 1,
                start_tick: n.start,
                duration: n.duration,
                midi: n.pitch.as_ref().map(|p| p.midi),
                voice: n.voice.clone(),
                note_start: n.note_start,
                note_end: n.note_end,
            }
        })
        .to_vec();
    (tone, slots)
}

#[test]
fn cross_channel_coincidence_is_refused_and_free_alternative_remains() {
    for measure in [1, 17] {
        let xml = xml(measure);
        let (tone, slots) = tone_and_slots(&xml);
        let m = legacy_fixture(&xml);
        assert!(!m.permits(&tone, &slots[0]));
        assert!(m.permits(&tone, &slots[1]));
        for mode in 0..3 {
            let chosen =
                consensus_guarded_broken_anchor(&tone, &slots, &BTreeSet::new(), mode, 0, &m)
                    .unwrap();
            assert_eq!(chosen.note_start, slots[1].note_start);
        }
        let chosen =
            consensus_guarded_gauss_anchor(&tone, &slots, &BTreeSet::new(), 0, &m).unwrap();
        assert_eq!(chosen.note_start, slots[1].note_start);
    }
}

#[test]
fn complete_fragment_fields_and_missing_identity_are_checked() {
    let xml = xml(1);
    for field in 0..11 {
        let (mut fragments, attacks, raw) = synthetic_parts(&xml);
        let row = &mut fragments[1];
        match field {
            0 => row.measure += 1,
            1 => row.onset += 1,
            2 => row.duration += 1,
            3 => row.pitch = Some(61),
            4 => row.staff = "other".into(),
            5 => row.voice = "other".into(),
            6 => row.visible = !row.visible,
            7 => row.is_chord = !row.is_chord,
            8 => row.tie_start = !row.tie_start,
            9 => row.tie_stop = !row.tie_stop,
            _ => row.is_grace = !row.is_grace,
        }
        assert!(
            GesturePlacementOccupancy::from_bound_parts(&xml, &fragments, &attacks, raw).is_none()
        );
    }
    let (mut fragments, attacks, raw) = synthetic_parts(&xml);
    fragments[1].attack_id = None;
    assert!(GesturePlacementOccupancy::from_bound_parts(&xml, &fragments, &attacks, raw).is_none());
}

#[test]
fn slot_geometry_mismatch_refuses_without_state_change() {
    let xml = xml(1);
    let (tone, slots) = tone_and_slots(&xml);
    let mut m = legacy_fixture(&xml);
    let mut wrong = slots[1].clone();
    wrong.start_tick += 1;
    assert!(!m.advance(&tone, &wrong, false));
    assert!(m.permits(&tone, &slots[1]));
    // Slot duration is the actual serializer's proposed duration, not the
    // atomic member's duration. It is carried unchanged into both simulations.
    let mut different = slots[1].clone();
    different.duration += 7;
    assert!(m.advance(&tone, &different, false));
    assert_eq!(m.notes[1].duration, different.duration.max(1));
}

#[test]
fn missing_target_leaves_the_phase_seed_and_scoring_is_deterministic() {
    let xml = xml(1);
    let (tone, slots) = tone_and_slots(&xml);
    let m = legacy_fixture(&xml);
    let before = m
        .notes
        .iter()
        .map(|n| (n.note_start, n.start, n.duration))
        .collect::<Vec<_>>();
    assert!(consensus_guarded_gauss_anchor(&tone, &slots[..1], &BTreeSet::new(), 0, &m).is_none());
    let a = consensus_gauss_distribution_residual(std::slice::from_ref(&tone), &slots, false, &m);
    let b = consensus_gauss_distribution_residual(std::slice::from_ref(&tone), &slots, false, &m);
    assert_eq!(a, b);
    assert_eq!(
        before,
        m.notes
            .iter()
            .map(|n| (n.note_start, n.start, n.duration))
            .collect::<Vec<_>>()
    );
}
