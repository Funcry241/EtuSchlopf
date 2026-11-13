use super::*;

// Synthetic SMF source genesis; time coordinates below are 1/384 quarter.
fn decoded(notes: &[(u8, u64, u64, u8)], ppq: u16, meters: &[(u64, u8, u8)]) -> DecodedMidiSource {
    decoded_with_markers(notes, ppq, meters, &[])
}

fn decoded_with_markers(
    notes: &[(u8, u64, u64, u8)],
    ppq: u16,
    meters: &[(u64, u8, u8)],
    markers: &[(u64, &str)],
) -> DecodedMidiSource {
    fn vlq(mut n: u64) -> Vec<u8> {
        let mut bytes = vec![(n & 127) as u8];
        n >>= 7;
        while n != 0 {
            bytes.push(128 | (n & 127) as u8);
            n >>= 7;
        }
        bytes.reverse();
        bytes
    }
    let count = notes.iter().map(|n| n.0).max().unwrap_or(0) + 1;
    let mut data = b"MThd".to_vec();
    data.extend(6_u32.to_be_bytes());
    data.extend(1_u16.to_be_bytes());
    data.extend(u16::from(count).to_be_bytes());
    data.extend(ppq.to_be_bytes());
    for lane in 0..count {
        let mut events = Vec::new();
        if lane == 0 {
            for &(tick, text) in markers {
                let text = format!("EtuSchlopf notation {text}");
                let mut payload = vec![255, 6];
                payload.extend(vlq(text.len() as u64));
                payload.extend(text.bytes());
                events.push((tick * u64::from(ppq) / 384, 0, payload));
            }
            for &(tick, beats, exponent) in meters {
                events.push((
                    tick * u64::from(ppq) / 384,
                    0,
                    vec![255, 88, 4, beats, exponent, 24, 8],
                ));
            }
        }
        for &(_, start, len, key) in notes.iter().filter(|n| n.0 == lane) {
            events.push((start * u64::from(ppq) / 384, 2, vec![144, key, 80]));
            events.push(((start + len) * u64::from(ppq) / 384, 1, vec![128, key, 0]));
        }
        events.sort_by_key(|e| (e.0, e.1));
        let mut track = Vec::new();
        let mut prior = 0;
        for (time, _, payload) in events {
            track.extend(vlq(time - prior));
            track.extend(payload);
            prior = time;
        }
        track.extend([0, 255, 47, 0]);
        data.extend(b"MTrk");
        data.extend(u32::try_from(track.len()).unwrap().to_be_bytes());
        data.extend(track);
    }
    crate::midi::decode_standard_midi_bytes(&data).unwrap()
}

fn grace_fixture(gap: u64) -> Vec<(u8, u64, u64, u8)> {
    vec![
        (0, 0, 192, 72),
        (0, 192, 149 - gap, 67),
        (0, 341 - gap, 43, 71),
        (0, 384, 384, 69),
    ]
}

#[test]
fn grace_projection_keeps_every_exact_source_event_and_is_ppq_invariant() {
    for ppq in [384, 768, 1536] {
        for gap in [0, 1] {
            let source = decoded(&grace_fixture(gap), ppq, &[(0, 4, 2)]);
            let original = source.notes.clone();
            let plan = NotationTimePlan::from_source(&source).unwrap();
            assert_eq!(source.notes, original);
            assert_eq!(plan.events.len(), original.len());
            for note in &original {
                assert_eq!(plan.source_event(note.source_ordinal).unwrap(), *note);
            }
            let grace = original.iter().find(|e| e.midi == 71).unwrap();
            let host = original.iter().find(|e| e.midi == 69).unwrap();
            let event = plan.event(grace.source_ordinal).unwrap();
            assert_eq!(
                event.kind,
                NotationEventKind::Grace {
                    host: host.source_ordinal
                }
            );
            assert_eq!((event.pos, event.len), (96, 0));
            let predecessor = original.iter().find(|e| e.midi == 67).unwrap();
            assert_eq!(plan.event(predecessor.source_ordinal).unwrap().len, 48);
        }
    }
}

#[test]
fn gap_boundary_tuplet_and_unnotatable_position_are_refused_without_rounding() {
    for notes in [
        grace_fixture(2),
        vec![(0, 0, 128, 72), (0, 128, 128, 74), (0, 256, 128, 76)],
        vec![(0, 4, 384, 72)],
    ] {
        let source = decoded(&notes, 384, &[(0, 4, 2)]);
        assert!(NotationTimePlan::from_source(&source).is_err());
    }
}

#[test]
fn ambiguous_chord_predecessors_and_competing_graces_are_refused() {
    let mut chord = grace_fixture(1);
    chord.push((0, 192, 148, 64));
    let mut simultaneous = grace_fixture(1);
    simultaneous.push((0, 340, 43, 74));
    let mut held_voice = grace_fixture(1);
    held_voice.push((0, 0, 768, 48));
    for notes in [chord, simultaneous, held_voice] {
        let source = decoded(&notes, 384, &[(0, 4, 2)]);
        assert!(NotationTimePlan::from_source(&source).is_err());
    }
}

#[test]
fn unrelated_track_and_independent_duplicate_do_not_remove_a_valid_grace() {
    let base = grace_fixture(1);
    let mut doubled = base.clone();
    doubled.extend(
        base.iter()
            .map(|&(lane, start, len, key)| (lane, start + 1536, len, key)),
    );
    doubled.push((1, 384, 384, 48));
    doubled.push((1, 384, 384, 55));
    let source = decoded(&doubled, 384, &[(0, 4, 2)]);
    let plan = NotationTimePlan::from_source(&source).unwrap();
    assert_eq!(
        plan.events
            .iter()
            .filter(|e| matches!(e.kind, NotationEventKind::Grace { .. }))
            .count(),
        2
    );
    assert_eq!(plan.event(2).unwrap().pos, 96);
}

#[test]
fn meter_changes_are_explicit_and_unsupported_or_partial_meters_are_refused() {
    let notes = [(0, 0, 6144, 72)];
    let source = decoded(&notes, 384, &[(0, 4, 2), (3072, 3, 2)]);
    let plan = NotationTimePlan::from_source(&source).unwrap();
    assert_eq!(plan.measure(0).unwrap().end, 384);
    assert_eq!(
        (plan.measure(2).unwrap().start, plan.measure(2).unwrap().end),
        (768, 1056)
    );
    for meters in [
        vec![(0, 5, 2)],
        vec![(0, 7, 2)],
        vec![(0, 6, 3)],
        vec![(0, 2, 1)],
        vec![(384, 3, 2)],
    ] {
        assert!(NotationTimePlan::from_source(&decoded(&notes, 384, &meters)).is_err());
    }
    for beats in [2, 3, 4] {
        assert!(NotationTimePlan::from_source(&decoded(&notes, 384, &[(0, beats, 2)])).is_ok());
    }
}

#[test]
fn grace_hostfix_cannot_legalize_an_unnotatable_leading_rest() {
    let source = decoded(
        &[(0, 12, 360, 60), (0, 372, 12, 61), (0, 384, 192, 64)],
        384,
        &[(0, 4, 2)],
    );
    assert!(NotationTimePlan::from_source(&source).is_err());
}

#[test]
fn genesis_binds_temporal_meaning_and_preserves_the_identity_projection() {
    let source = decoded(&[(0, 0, 192, 60), (0, 384, 192, 64)], 384, &[(0, 4, 2)]);
    let plan = NotationTimePlan::from_source(&source).unwrap();
    assert_eq!(plan.bind_productive_plan("source_plan"), "source_plan");
    let source = decoded(&[(0, 0, 192, 60), (0, 384, 192, 64)], 384, &[(0, 3, 2)]);
    let changed = NotationTimePlan::from_source(&source).unwrap();
    assert_ne!(
        changed.bind_productive_plan("source_plan"),
        plan.bind_productive_plan("source_plan")
    );
    assert_eq!(
        changed.bind_productive_plan("source_plan"),
        changed.clone().bind_productive_plan("source_plan")
    );
    let source = decoded(
        &[(0, 0, 372, 60), (0, 372, 12, 61), (0, 384, 192, 64)],
        384,
        &[(0, 4, 2)],
    );
    let grace = NotationTimePlan::from_source(&source).unwrap();
    assert_ne!(
        grace.bind_productive_plan("source_plan"),
        plan.bind_productive_plan("source_plan")
    );
}

#[test]
fn odd_three_unit_lengths_and_onsets_are_refused_before_serialization() {
    for value in [9_u64, 15] {
        let length_case = decoded(&[(0, 0, value * 4, 72), (0, 384, 384, 74)], 384, &[(0, 4, 2)]);
        assert!(NotationTimePlan::from_source(&length_case).is_err());
        let onset_case = decoded(&[(0, value * 4, 384, 72), (0, 768, 384, 74)], 384, &[(0, 4, 2)]);
        assert!(NotationTimePlan::from_source(&onset_case).is_err());
    }
    for value in [6, 12, 18, 96] {
        let source = decoded(&[(0, 0, value * 4, 72), (0, 384, 384, 74)], 384, &[(0, 4, 2)]);
        assert!(NotationTimePlan::from_source(&source).is_ok());
    }
}

#[test]
fn declared_timing_projects_variable_pulse_without_changing_source_time() {
    let notes = [(0, 100, 99, 72), (0, 203, 237, 74), (0, 441, 119, 76)];
    let markers = [
        (100, "v1 meter 2"),
        (200, "v1 anchor 48"),
        (440, "v1 anchor 144"),
        (560, "v1 anchor 240"),
    ];
    for ppq in [384, 768] {
        let source = decoded_with_markers(&notes, ppq, &[], &markers);
        let plan = NotationTimePlan::from_source(&source).unwrap();
        assert_eq!((plan.measure(0).unwrap().start, plan.measure(0).unwrap().end), (0, 192));
        assert_eq!((plan.measure(1).unwrap().start, plan.measure(1).unwrap().end), (192, 384));
        assert_eq!(plan.measure(0).unwrap().beats, 2);
        for (ordinal, (pos, len)) in [(0, 48), (48, 96), (144, 96)].into_iter().enumerate() {
            assert_eq!(plan.source_event(ordinal).unwrap(), source.notes[ordinal]);
            let event = plan.event(ordinal).unwrap();
            assert_eq!((event.pos, event.len), (pos, len));
            assert_eq!(plan.written_sustain_interval(ordinal).unwrap(), pos..pos + len);
        }
    }
}

#[test]
fn declared_timing_refuses_unbracketed_ambiguous_and_conflicting_evidence() {
    let markers = [(100, "v1 meter 2"), (484, "v1 anchor 96")];
    for notes in [
        vec![(0, 99, 193, 72)],
        vec![(0, 112, 180, 72)],
        vec![(0, 100, 384, 72), (0, 103, 189, 72)],
    ] {
        let source = decoded_with_markers(&notes, 384, &[], &markers);
        assert!(NotationTimePlan::from_source(&source).is_err());
    }
    let source = decoded_with_markers(&[(0, 100, 384, 72)], 384, &[(0, 3, 2)], &markers);
    assert!(NotationTimePlan::from_source(&source).is_err());
}

#[test]
fn declared_pickup_uses_its_own_extent_and_moves_the_first_full_measure() {
    let notes = [(0, 100, 100, 72), (0, 200, 240, 74), (0, 440, 120, 76)];
    let markers = [
        (100, "v1 meter 2 pickup 48"),
        (200, "v1 anchor 48"),
        (440, "v1 anchor 144"),
        (560, "v1 anchor 240"),
    ];
    let source = decoded_with_markers(&notes, 384, &[], &markers);
    let plan = NotationTimePlan::from_source(&source).unwrap();
    let opening = plan.measure(0).unwrap();
    let first_full = plan.measure(1).unwrap();
    assert_eq!((opening.start, opening.end), (0, 48));
    assert_eq!((first_full.start, first_full.end), (48, 240));
    assert_eq!(opening.beats, first_full.beats);
    assert_eq!(plan.metric.ordinal_at(48), Some(1));
    assert_eq!(plan.event(1).unwrap().pos, 48);
    assert_eq!(plan.source_event(1).unwrap(), source.notes[1]);
}

#[test]
fn performed_binary_pulse_is_inferred_without_a_sidecar() {
    let mut notes = vec![(0, 640, 126, 71)];
    let first_downbeat = 796;
    for index in 0..40 {
        let onset = first_downbeat + index * 99 + index % 3;
        let bass = if index % 4 == 0 { 43 } else { 55 };
        notes.push((0, onset, 81, bass));
        notes.push((0, onset + 5, 86, 76));
    }
    let source = decoded(&notes, 384, &[]);
    let plan = NotationTimePlan::from_midi(&source).unwrap();
    assert_eq!(plan.timing_authority, TimingAuthority::InferredPerformance);
    assert_eq!(plan.events.len(), notes.len());
    assert_eq!(plan.events[0].len, 48);
    assert_eq!(plan.events[1].pos, 48);
    assert_eq!(plan.events[2].pos, 48);
    assert_eq!(plan.metric.measure(0).unwrap().end, 48);
    assert_eq!(plan.metric.measure(1).unwrap().beats, 2);
    assert_eq!(plan.source_events, source.notes);
}

#[test]
fn a_steady_pulse_without_distinct_accents_is_insufficient() {
    let notes = (0..40).map(|index| (0, 40 + index * 101, 83, 60))
        .collect::<Vec<_>>();
    let source = decoded(&notes, 384, &[]);
    assert!(NotationTimePlan::from_inferred_performance(&source).is_err());
}
