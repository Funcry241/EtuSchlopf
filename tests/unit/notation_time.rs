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
        vec![(0, 5, 3)],
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
    assert!(!plan.binds_written_intervals());
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

#[test]
fn outer_line_can_continue_to_the_quarter_when_played_release_runs_short() {
    let mut notes = Vec::new();
    for index in 0..44 {
        let onset = 20 + 100 * index;
        let phase = index % 4;
        let bass = if phase == 0 { 43 } else { 55 };
        notes.push((0, onset, 75, bass));
        if phase == 0 {
            let length = if index == 20 { 280 } else if matches!(index, 8 | 16 | 32) { 310 } else { 76 };
            notes.push((0, onset + 4, length, 76));
        }
        if index == 34 {
            notes.push((0, onset + 4, 75, 77));
        }
    }
    let source = decoded(&notes, 384, &[]);
    let plan = NotationTimePlan::from_midi(&source).unwrap();

    for (group, duration) in [(8, 96), (16, 96), (20, 72), (24, 24), (32, 72)] {
        let raw = source.notes.iter()
            .find(|note| note.pos * 384 == (24 + 100 * group) * source.time_domain.divisions && note.midi == 76)
            .unwrap();
        assert_eq!(plan.event(raw.source_ordinal).unwrap().len, duration, "group {group}");
    }
    for original in source.notes {
        assert_eq!(plan.source_event(original.source_ordinal).unwrap(), original);
    }
}

#[test]
fn moving_bass_with_overlapping_releases_uses_the_next_subdivision() {
    for (companion, written) in [(72, 24), (74, 72)] {
        let mut notes = Vec::new();
        for index in 0..40 {
            let onset = 20 + 100 * index;
            let phase = index % 4;
            let (bass, length, treble) = match phase {
                0 => (40, 270, 72),
                2 => (46, 74, companion),
                _ => (52, 74, 76),
            };
            notes.push((0, onset, length, bass));
            notes.push((0, onset + 4, 76, treble));
        }
        let source = decoded(&notes, 384, &[]);
        let plan = NotationTimePlan::from_midi(&source).unwrap();
        let bass = source.notes.iter().filter(|note| note.midi == 40)
            .min_by_key(|note| note.pos).unwrap();
        // The first group has no earlier attack. Check the interior figures.
        assert_eq!(plan.event(bass.source_ordinal).unwrap().len, 72);
        for attack in source.notes.iter().filter(|note| note.midi == 40 && note.pos > bass.pos) {
            assert_eq!(plan.event(attack.source_ordinal).unwrap().len, written);
            assert_eq!(plan.source_event(attack.source_ordinal).unwrap(), *attack);
        }
    }
}

#[test]
fn short_release_in_a_continuing_bass_line_can_write_a_quarter() {
    for (companion, written) in [(60, 96), (62, 48)] {
        let mut notes = Vec::new();
        for group in 0..41 {
            let onset = 20 + 100 * group;
            let (bass, length, upper) = match group % 4 {
                0 => {
                    let short_release = group % 8 == 4;
                    (if short_release { 42 } else { 40 },
                     if short_release { 220 } else { 320 }, 64)
                }
                1 => (54, 75, 60),
                2 => (57, 75, 65),
                _ => (56, 75, companion),
            };
            notes.push((0, onset, length, bass));
            notes.push((0, onset + 4, 75, upper));
        }
        let source = decoded(&notes, 384, &[]);
        let plan = NotationTimePlan::from_midi(&source).unwrap();
        for attack in source.notes.iter().filter(|note| note.midi == 42) {
            assert_eq!(plan.event(attack.source_ordinal).unwrap().len, written);
            assert_eq!(plan.source_event(attack.source_ordinal).unwrap(), *attack);
        }
    }
}

#[test]
fn repeated_chord_member_uses_next_subdivision_despite_overlapping_release() {
    for (returning_pitch, written) in [(72, 24), (75, 48)] {
        let mut notes = Vec::new();
        for group in 0..40 {
            let onset = 20 + 100 * group;
            let bass = if group % 4 == 0 { 43 } else { 55 };
            let pitch = match group {
                17 => 77,
                18 => returning_pitch,
                _ => 72,
            };
            let length = if group == 16 { 140 } else { 76 };
            notes.push((0, onset, 75, bass));
            notes.push((0, onset + 3, 75, 61));
            notes.push((0, onset + 6, 75, 67));
            notes.push((0, onset + 9, length, pitch));
        }
        let source = decoded(&notes, 384, &[]);
        let plan = NotationTimePlan::from_midi(&source).unwrap();
        let attack = source.notes.iter().find(|note| note.midi == 72
            && note.pos * 384 == 1629 * source.time_domain.divisions).unwrap();
        assert_eq!(plan.event(attack.source_ordinal).unwrap().len, written);
        assert_eq!(plan.source_event(attack.source_ordinal).unwrap(), *attack);
    }
}

#[test]
fn offbeat_melody_can_hold_a_quarter_over_alternating_accompaniment() {
    for (returning_pitch, written) in [(74, 96), (76, 72)] {
        let mut notes = Vec::new();
        for group in 0..40 {
            let onset = 20 + 100 * group;
            let bass = if group % 4 == 0 { 43 } else { 55 };
            notes.push((0, onset, 75, bass));
            match group {
                14 => notes.push((0, onset + 4, 310, 72)),
                15 | 17 => notes.push((0, onset + 4, 75, 63)),
                18 => {
                    notes.push((0, onset + 15, 25, returning_pitch + 2));
                    notes.push((0, onset + 40, 84, returning_pitch));
                }
                _ => {}
            }
        }
        let source = decoded(&notes, 384, &[]);
        let plan = NotationTimePlan::from_midi(&source).unwrap();
        let attack = source.notes.iter().find(|note| note.midi == 72).unwrap();
        let grace = source.notes.iter().find(|note| note.midi == i32::from(returning_pitch) + 2).unwrap();
        let host = source.notes.iter().find(|note| note.midi == i32::from(returning_pitch)).unwrap();
        assert_eq!(plan.event(attack.source_ordinal).unwrap().len, written);
        assert_eq!(plan.source_event(attack.source_ordinal).unwrap(), *attack);
        assert_eq!(plan.event(grace.source_ordinal).unwrap().kind,
            NotationEventKind::Grace { host: host.source_ordinal });
    }
}

#[test]
fn embedded_note_evidence_binds_a_freely_played_triple_meter_pickup() {
    for shift in [0, 384] {
        let notes = [(0, 127 + shift, 553, 72), (0, 698 + shift, 45, 73), (0, 757 + shift, 677, 74)];
        let markers = [
            (0, "v2 meter 3 pickup 96"),
            (127 + shift, "v2 note 0 0 72 0 96"),
            (698 + shift, "v2 grace 1 0 73 2"),
            (757 + shift, "v2 note 2 0 74 96 288"),
        ];
        let source = decoded_with_markers(&notes, 384, &[(0, 3, 2)], &markers);
        let plan = NotationTimePlan::from_midi(&source).unwrap();
        assert!(plan.preserves_attacks());
        assert_eq!(plan.measure(0).unwrap().end, 96);
        assert_eq!(plan.measure(1).unwrap().end, 384);
        assert_eq!(plan.event(1).unwrap().kind, NotationEventKind::Grace { host: 2 });
        assert_eq!((plan.event(1).unwrap().pos, plan.event(1).unwrap().len), (96, 0));
        assert_eq!(plan.source_event(0).unwrap().pos, source.notes[0].pos);

        for invalid in [
            (vec![markers[0], markers[1], markers[3]], "Quellnote 1"),
            (vec![markers[0], (128 + shift, markers[1].1), markers[2], markers[3]], "MIDI-Tick"),
            (vec![markers[0], markers[1], (698 + shift, "v2 grace 1 0 73 1"), markers[3]], "Hauptnote"),
        ] {
            let source = decoded_with_markers(&notes, 384, &[(0, 3, 2)], &invalid.0);
            assert!(NotationTimePlan::from_midi(&source).unwrap_err().to_string().contains(invalid.1));
        }
    }
}

#[test]
fn playback_only_marker_keeps_the_source_event_without_written_time() {
    let notes = [(0, 127, 350, 72), (0, 698, 45, 73), (0, 757, 677, 74)];
    let markers = [
        (0, "v2 meter 3 pickup 96"),
        (127, "v2 note 0 0 72 0 96"),
        (698, "v2 playback 1 0 73"),
        (757, "v2 note 2 0 74 96 288"),
    ];
    let source = decoded_with_markers(&notes, 384, &[(0, 3, 2)], &markers);
    let plan = NotationTimePlan::from_midi(&source).unwrap();
    assert_eq!(plan.event(1).unwrap().kind, NotationEventKind::PlaybackOnly);
    assert!(plan.written_sustain_interval(1).is_err());
    assert_eq!(plan.source_event(1).unwrap(), source.notes[1]);
    assert_eq!(plan.measure(0).unwrap().end, 96);
    let mut wrong = markers;
    wrong[2].1 = "v2 playback 1 0 76";
    let source = decoded_with_markers(&notes, 384, &[(0, 3, 2)], &wrong);
    assert!(NotationTimePlan::from_midi(&source).unwrap_err().to_string().contains("MIDI-Tick"));
}

#[test]
fn companion_binds_compound_meter_to_original_midi_and_exact_attacks() {
    let input = b"synthetic MIDI bytes";
    let checksum = crate::sha256::hex(input);
    let companion = format!(
        "ETUSCHLOPF-NOTATION-TIME-V1\nmidi-sha256 {checksum}\nkey 3 major\n0\tv2 meter 6/8 pickup 0\n13\tv2 note 0 0 65 0 72\n");
    let mut source = decoded(&[(0, 13, 287, 65)], 384, &[]);
    assert!(attach_notation_companion(&mut source, b"different bytes", &companion).is_err());
    attach_notation_companion(&mut source, input, &companion).unwrap();
    let plan = NotationTimePlan::from_midi(&source).unwrap();
    let bar = plan.metric.measure(0).unwrap();
    assert_eq!((bar.beats, bar.beat_type, bar.end), (6, 8, 288));
    assert_eq!((plan.events[0].pos, plan.events[0].len), (0, 72));
    assert_eq!(source.opening_key_signature.unwrap().fifths, 3);
}

#[test]
fn free_performance_projects_a_supported_pickup_into_the_metric_timeline() {
    // The melody begins alone. Chords at the beginning and end of
    // each full bar provide evidence for the bar phase.
    let pickup = 4;
    let cycle = 12;
    let bars = 12;
    let mut notes = vec![(0, 3 * 96, 76, 77), (0, 5 * 96, 76, 77)];
    for bar in 0..bars {
        let bass = [43, 45, 48, 50, 47, 41][bar % 6];
        let start = 3 + pickup + bar as u64 * cycle;
        for (offset, pitches) in [
            (0, vec![bass, bass + 12, bass + 19]),
            (2, vec![bass + 17]),
            (5, vec![bass + 22]),
            (6, vec![bass + 15]),
            (8, vec![bass + 20]),
            (11, vec![bass, bass + 12, bass + 16]),
        ] {
            let final_bar = bar + 1 == bars;
            if final_bar && offset > cycle / 2 { continue; }
            let final_pulse = final_bar && offset == cycle / 2;
            let pitches = if final_pulse { vec![bass, bass + 12, bass + 16] } else { pitches };
            for pitch in pitches {
                let hold = if final_pulse {
                    // The final chord shares a short release and is followed by silence.
                    2 * 96 - 36
                } else if offset == 0 && pitch == bass {
                    // Hold the low chord tone while the other pitches move independently.
                    5 * 96 - 20
                } else {
                    76
                };
                notes.push((0, (start + offset) * 96, hold, pitch));
            }
        }
    }

    let mut source = decoded(&notes, 384, &[]);
    source.tempo_changes = vec![(0, 760_000)];
    let sample_rate = 12_000;
    let frames_per_sixteenth = 2_280;
    let mut samples = vec![0.0_f32; (pickup + cycle * 13) as usize * frames_per_sixteenth];
    for &(_, tick, _, pitch) in &notes {
        let phase = std::f64::consts::TAU * 440.0 * 2.0_f64.powf((f64::from(pitch) - 69.0) / 12.0);
        let start = (tick / 96) as usize * frames_per_sixteenth;
        for (frame, sample) in samples[start..start + sample_rate as usize / 8].iter_mut().enumerate() {
            let t = frame as f64 / f64::from(sample_rate);
            let envelope = (t / 0.004).min(1.0) * (-t / 0.065).exp();
            *sample += (0.16 * envelope * ((phase * t).sin() + 0.5 * (2.0 * phase * t).sin())) as f32;
        }
    }

    let path = std::path::Path::new("synthetic-compound-pickup.mid");
    let originals = source.notes.clone();
    let prepared = std::sync::Arc::new(
        PreparedPerformanceTiming::prepare_performance(path, source, &samples, sample_rate).unwrap(),
    );
    let (source, plan) = prepared.source_and_plan();
    let opening = plan.measure(0).unwrap();
    let full = plan.measure(1).unwrap();
    assert_eq!((opening.beats, opening.beat_type), (6, 8));
    assert_eq!((opening.start, opening.end), (0, 96));
    assert_eq!((full.start, full.end), (96, 384));
    assert_eq!(plan.event(2).unwrap().pos, full.start);
    assert_eq!(source.notes, originals);
    assert_eq!(plan.events.len(), originals.len());
    assert!(originals.iter().all(|note| plan.source_event(note.source_ordinal).unwrap() == *note));
    let original_events = plan.events.clone();
    let open: BTreeSet<_> = original_events.iter()
        .filter(|event| plan.duration_is_open(event.source_ordinal))
        .map(|event| event.source_ordinal)
        .collect();
    assert!(plan.has_open_durations() && !open.is_empty());
    assert!(original_events.iter().any(|event| {
        event.kind == NotationEventKind::Regular && !open.contains(&event.source_ordinal)
    }));

    let _scope = PreparedPerformanceTiming::enter(Some(&prepared));
    let (xml, stats) = crate::midi_seed::midi_to_musicxml_seed_with_profile_and_hotspots(
        path, "Synthetic performance", crate::midi_seed::midi_seed_solver_profiles()[0].clone(), None,
    ).unwrap();
    let settled = &stats.notation_time;
    assert!(settled.binds_written_intervals() && !settled.has_open_durations());
    assert_eq!(plan.events, original_events);
    for &before in &original_events {
        let id = before.source_ordinal;
        assert_eq!(plan.duration_is_open(id), open.contains(&id));
        let after = settled.event(id).unwrap();
        assert!(!settled.duration_is_open(id));
        assert_eq!((after.pos, after.kind), (before.pos, before.kind));
        if !open.contains(&id) {
            assert_eq!(after.len, before.len);
        }
    }
    assert!(open.iter().any(|&id| {
        let after = settled.event(id).unwrap();
        after.len != plan.event(id).unwrap().len
            && settled.events.iter().any(|next| {
                next.kind == NotationEventKind::Regular && next.pos == after.pos + after.len
            })
    }));
    assert!(settled.events.iter().any(|long| settled.events.iter().any(|short| {
        long.pos == short.pos && long.len > short.len && short.len > 0
    })));
    let last_pos = original_events.iter().map(|event| event.pos).max().unwrap();
    let last_bar = settled.measure(settled.metric.len() - 1).unwrap();
    let final_chord: Vec<_> = settled.events.iter().filter(|event| event.pos == last_pos).collect();
    assert!(final_chord.len() > 1);
    assert!(final_chord.iter().any(|event| open.contains(&event.source_ordinal)));
    for event in final_chord {
        assert_eq!(event.len, plan.event(event.source_ordinal).unwrap().len);
        assert!(event.pos + event.len < last_bar.end);
    }
    let inventory = stats.rhythmic_courses.arrangement_inventory(&xml).unwrap();
    let score = crate::musicxml::parse_musicxml(&xml).unwrap();
    let mut occurrences = inventory.occurrences.iter();
    let mut ends = BTreeMap::new();
    for (bar, measure) in score.parts[0].measures.iter().enumerate() {
        let start = settled.measure(bar).unwrap().start;
        for note in &measure.events {
            let fragment = occurrences.next().unwrap();
            let source = fragment.source;
            let Some(pitch) = &note.pitch else { assert!(source.is_none()); continue; };
            let id = source.unwrap();
            let prior_end = ends.insert(id, start + note.start + note.duration);
            let event = settled.event(id).unwrap();
            assert_eq!(pitch.midi, settled.source_event(id).unwrap().midi);
            assert_eq!(note.tie_stop, prior_end.is_some());
            assert_eq!(start + note.start, prior_end.unwrap_or(event.pos));
            let continues = start + note.start + note.duration < event.pos + event.len;
            assert_eq!(xml[fragment.bytes.clone()].contains("<tie type=\"start\"/>"), continues);
        }
    }
    assert!(occurrences.next().is_none());
    for event in &settled.events {
        let expected = (event.kind != NotationEventKind::PlaybackOnly).then_some(event.pos + event.len);
        assert_eq!(ends.remove(&event.source_ordinal), expected);
    }
    assert!(ends.is_empty());
}
