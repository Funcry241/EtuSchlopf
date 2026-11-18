use super::*;

fn note(source: usize, pos: i64, staff: u8) -> MidiSeedNote {
    MidiSeedNote { source_ordinal: source, pos, len: 24, midi: 60, staff }
}

fn emitted(notes: &[MidiSeedNote], measures: usize) -> (String, MidiSeedRhythmicCourses) {
    let mut plan = MidiSeedRhythmicCourses::create_primary_presentations(
        notes, measures, MEASURE_DIVISIONS,
    ).unwrap();
    let (xml, _, _) = build_musicxml("Synthetic", notes, measures, &mut plan, None).unwrap();
    plan.validate_seed_binding(&xml).unwrap();
    (xml, plan)
}

#[test]
fn birth_is_before_xml_and_covers_empty_presentations() {
    let plan = MidiSeedRhythmicCourses::create_primary_presentations(&[], 3, 384).unwrap();
    assert!(plan.occurrences.is_empty());
    assert!(plan.emission_snapshot.is_none());
    assert!(plan.courses.iter().all(|course| course.coverage == (0..1152)));
    let (xml, plan) = emitted(&[], 3);
    assert_eq!(plan.occurrences.len(), 6);
    assert!(plan.occurrences.iter().all(|o| o.source.is_none()));
    plan.validate_seed_binding(&xml).unwrap();
}

#[test]
fn leading_silence_and_measure_splits_keep_created_course() {
    let (_, plan) = emitted(&[note(17, 96, 1), note(18, 700, 1)], 2);
    let upper = plan.courses.iter().find(|c| c.staff == 1).unwrap();
    let activity = plan.occurrences.iter().filter(|o| o.course == upper.creation)
        .collect::<Vec<_>>();
    assert_eq!(activity.len(), 4);
    assert_eq!(activity[0].interval, 0..96);
    assert_eq!(activity[2].interval, 384..700);
    assert!(activity[0].source.is_none() && activity[2].source.is_none());
    assert_eq!(upper.coverage, 0..768);
}

#[test]
fn coincident_sources_keep_distinct_creation_and_membership() {
    let (_, plan) = emitted(&[note(31, 0, 1), note(32, 0, 1)], 1);
    let sources = plan.occurrences.iter().filter(|o| o.source.is_some()).collect::<Vec<_>>();
    assert_eq!(sources.len(), 2);
    assert_eq!(sources[0].interval, sources[1].interval);
    assert_ne!(sources[0].creation, sources[1].creation);
    assert_ne!(sources[0].source, sources[1].source);
}

#[test]
fn equal_independent_creations_do_not_share_identity() {
    let (_, a) = emitted(&[], 1);
    let (_, b) = emitted(&[], 1);
    assert_ne!(a.courses[0].creation, b.courses[0].creation);
    assert_ne!(a.occurrences[0].creation, b.occurrences[0].creation);
    assert_eq!(format!("{:?}", a.courses[0].creation), "RhythmicCreationIdentity(opaque)");
    let transported = a.clone();
    assert_eq!(a.occurrences[0].creation, transported.occurrences[0].creation);
}

#[test]
fn missing_duplicate_or_foreign_origins_refuse_binding() {
    let (xml, original) = emitted(&[], 1);
    let (_, foreign) = emitted(&[], 1);
    let mut missing = original.clone();
    missing.occurrences.pop();
    assert!(missing.validate_seed_binding(&xml).is_err());
    let mut duplicate = original.clone();
    duplicate.occurrences[1].creation = duplicate.occurrences[0].creation;
    assert!(duplicate.validate_seed_binding(&xml).is_err());
    let mut wrong = original;
    wrong.occurrences[0].course = foreign.courses[0].creation;
    assert!(wrong.validate_seed_binding(&xml).is_err());
}

#[test]
fn stale_xml_and_unrepresented_extension_refuse_binding() {
    let (xml, mut plan) = emitted(&[], 1);
    assert!(plan.validate_seed_binding(&xml.replace("<voice>1</voice>", "<voice>9</voice>")).is_err());
    plan.courses[0].coverage.end += 96;
    assert!(plan.validate_seed_binding(&xml).is_err());
}

#[test]
fn source_change_or_repeated_emission_is_not_continuation() {
    let notes = [note(11, 24, 1)];
    let mut fresh = MidiSeedRhythmicCourses::create_primary_presentations(&notes, 1, 384).unwrap();
    assert!(build_musicxml("Synthetic", &[note(11, 24, 2)], 1, &mut fresh, None).is_err());
    let (_, mut used) = emitted(&notes, 1);
    assert!(build_musicxml("Synthetic", &notes, 1, &mut used, None).is_err());
}

#[test]
fn invalid_domain_and_source_inventory_refuse_creation() {
    assert!(MidiSeedRhythmicCourses::create_primary_presentations(&[], 0, 384).is_err());
    assert!(MidiSeedRhythmicCourses::create_primary_presentations(&[], 1, 0).is_err());
    assert!(MidiSeedRhythmicCourses::create_primary_presentations(&[note(1, 0, 1), note(1, 24, 2)], 1, 384).is_err());
    assert!(MidiSeedRhythmicCourses::create_primary_presentations(&[note(1, -1, 1)], 1, 384).is_err());
    assert!(MidiSeedRhythmicCourses::create_primary_presentations(&[note(1, 384, 1)], 1, 384).is_err());
    assert!(MidiSeedRhythmicCourses::create_primary_presentations(&[note(1, 0, 3)], 1, 384).is_err());
}

#[test]
fn metric_scaling_changes_coverage_not_creation_semantics() {
    let a = MidiSeedRhythmicCourses::create_primary_presentations(&[], 2, 384).unwrap();
    let b = MidiSeedRhythmicCourses::create_primary_presentations(&[], 2, 768).unwrap();
    assert_eq!(b.courses[0].coverage.end, 2 * a.courses[0].coverage.end);
    assert_eq!(b.courses[0].presentation, a.courses[0].presentation);
    // Current seed writer supports its existing metric only; no silent coercion.
    assert!(b.begin_emission(&[], 2).is_err());
}

#[test]
fn writer_labels_do_not_issue_or_select_a_course_role() {
    let mut plan = MidiSeedRhythmicCourses::create_primary_presentations(&[], 1, 384).unwrap();
    let original_ids = plan.courses.iter().map(|c| c.creation).collect::<Vec<_>>();
    let mut xml = String::new();
    let mut rows = Vec::new();
    let mut raw = 0;
    for (staff, voice) in [(1, "separate_upper_channel"), (2, "41")] {
        append_staff_voice(&mut xml, &mut [], 0, MidiSeedStaffVoiceSerialization {
            staff, voice, stem: "up", measure_ordinal: 1,
            raw_note_index: &mut raw, serialized_source_identity_rows: &mut rows,
            rhythmic_courses: &mut plan,
        }).unwrap();
    }
    plan.seal_emission(&xml).unwrap();
    plan.validate_seed_binding(&xml).unwrap();
    assert_eq!(original_ids, plan.courses.iter().map(|c| c.creation).collect::<Vec<_>>());
    assert!(plan.courses.iter().all(|c| c.presentation == SeedCoursePresentation::Primary));
}

#[test]
fn rest_over_sound_or_incomplete_source_membership_is_refused() {
    let (xml, mut plan) = emitted(&[note(7, 96, 1)], 1);
    plan.occurrences[0].interval.end = 192;
    assert!(plan.validate_seed_binding(&xml).is_err());
    let (xml, mut plan) = emitted(&[note(7, 96, 1)], 1);
    plan.sources.push(note(8, 96, 1));
    assert!(plan.validate_seed_binding(&xml).is_err());
}

#[test]
fn project_renaming_and_staff_permutation_preserve_coverage() {
    let notes = [note(7, 48, 1)];
    let (_, first) = emitted(&notes, 1);
    let mut second = MidiSeedRhythmicCourses::create_primary_presentations(&notes, 1, 384).unwrap();
    let (xml, _, _) = build_musicxml("Another project name", &notes, 1, &mut second, None).unwrap();
    second.validate_seed_binding(&xml).unwrap();
    assert_eq!(first.courses[0].coverage, second.courses[0].coverage);
    let (_, permuted) = emitted(&[note(7, 48, 2)], 1);
    let first_intervals = first.occurrences.iter().filter(|o| o.course == first.courses[0].creation)
        .map(|o| (o.interval.clone(), o.source)).collect::<Vec<_>>();
    let swapped_intervals = permuted.occurrences.iter().filter(|o| o.course == permuted.courses[1].creation)
        .map(|o| (o.interval.clone(), o.source)).collect::<Vec<_>>();
    assert_eq!(first_intervals, swapped_intervals);
}

pub(crate) fn arrangement_fixture(rows: &[(usize, i64, u8)], measures: usize) -> (String, MidiSeedRhythmicCourses) {
    emitted(&rows.iter().map(|&(id, start, staff)| note(id, start, staff)).collect::<Vec<_>>(), measures)
}

pub(crate) fn arrangement_pitched_fixture(rows: &[(usize, i64, u8, i32)], measures: usize) -> (String, MidiSeedRhythmicCourses) {
    emitted(&rows.iter().map(|&(source, pos, staff, midi)| MidiSeedNote {
        source_ordinal: source, pos, len: 24, staff, midi,
    }).collect::<Vec<_>>(), measures)
}

/// Synthetic source lengths passed through the unchanged original emitter policy.
pub(crate) fn arrangement_source_lengths_fixture(rows: &[(usize, i64, i64, u8, i32)], count: usize)
    -> (String, MidiSeedRhythmicCourses) {
    emitted(&rows.iter().map(|&(source, pos, len, staff, midi)| MidiSeedNote {
        source_ordinal: source, pos, len, staff, midi,
    }).collect::<Vec<_>>(), count)
}

#[test]
fn exact_pair_releases_keep_their_own_source_intervals() {
    for (staff, start, pitch, short_pitch) in [
        (1, 96, 72, 72), (2, 192, 52, 52),
        (1, 96, 72, 67), (2, 192, 52, 57),
    ] {
        let source = |id, pos, len, midi| MidiSeedNote {
            source_ordinal: id, pos, len, staff, midi,
        };
        let notes = [
            source(4, start, 96, pitch), source(9, start, 48, short_pitch),
            source(7, start + 48, 24, pitch + 2), source(11, start + 72, 24, pitch - 2),
            source(12, start + 96, 24, pitch + 4),
        ];
        let (xml, plan) = emitted(&notes, 1);
        for (id, span, separate) in [(4, start..start + 96, true), (9, start..start + 48, false)] {
            let planned = plan.planned.iter().find(|n| n.source == Some(id)).unwrap();
            assert_eq!(planned.interval, span);
            assert_eq!(planned.independent, separate);
            assert!(!planned.chord);
            let written = plan.occurrences.iter().find(|n| n.source == Some(id)).unwrap();
            assert!(!xml[written.bytes.clone()].contains("<chord"));
        }

        let mut reattack = notes;
        reattack[3].midi = pitch;
        let (_, plan) = emitted(&reattack, 1);
        assert!(!plan.planned.iter().any(|n| n.independent));
    }
}

#[test]
fn exact_independent_release_can_pass_a_beat_and_an_exact_primary_chord() {
    for (staff, start) in [(1, 72_i64), (2, 168)] {
        let source = |id, offset: i64, len, midi| MidiSeedNote {
            source_ordinal: id, pos: start + offset, len, staff, midi,
        };
        let notes = [
            source(0, 0, 96, 70), source(1, 0, 24, 64),
            source(2, 24, 48, 66), source(3, 24, 48, 62),
            source(4, 72, 24, 67), source(5, 96, 24, 68),
        ];
        let (_, plan) = emitted(&notes, 1);
        let sustain = plan.planned.iter().find(|note| note.source == Some(0)).unwrap();
        assert!(sustain.independent);
        // The final attack delimits the independent interval.
        for note in notes.iter().filter(|note| note.pos < start + 96) {
            let written = plan.occurrences.iter().find(|item| item.source == Some(note.source_ordinal)).unwrap();
            assert_eq!(written.interval, note.pos..note.pos + note.len);
        }

        let mut enlarged_onset = notes.to_vec();
        enlarged_onset.insert(2, source(6, 0, 24, 61));
        let (_, plan) = emitted(&enlarged_onset, 1);
        assert!(plan.planned.iter().any(|note| {
            note.source == Some(0) && note.independent && note.interval == (start..start + 96)
        }));

        let mut chord_has_own_release = notes;
        chord_has_own_release[3].len = 24;
        let (_, plan) = emitted(&chord_has_own_release, 1);
        assert!(!plan.planned.iter().any(|note| note.independent));
    }
}

#[test]
fn mixed_release_chord_keeps_member_intervals_in_legal_serialization_order() {
    let notes = [
        MidiSeedNote { midi: 45, len: 48, ..note(0, 0, 2) },
        MidiSeedNote { midi: 49, len: 24, ..note(1, 0, 2) },
        MidiSeedNote { midi: 52, len: 48, ..note(2, 0, 2) },
        MidiSeedNote { midi: 56, len: 48, ..note(3, 0, 2) },
        MidiSeedNote { midi: 43, len: 48, ..note(4, 48, 2) },
    ];
    let (xml, plan) = emitted(&notes, 1);
    let written: Vec<_> = plan.planned.iter().filter_map(|item| {
        item.source.map(|source| (source, item.interval.clone(), item.chord))
    }).collect();
    assert_eq!(written, [
        (0, 0..48, false), (2, 0..48, true), (3, 0..48, true),
        (1, 0..24, true), (4, 48..384, false),
    ]);
    assert!(crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition(
        "synthetic-mixed-release-seed", &xml,
    ).is_ok());
}

#[test]
fn aligned_tied_octave_keeps_long_values_through_arrangement() {
    let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], 3, 384)
        .unwrap().metric;
    let notes = [
        MidiSeedNote { source_ordinal: 0, pos: 192, len: 960, midi: 36, staff: 2 },
        MidiSeedNote { source_ordinal: 1, pos: 192, len: 960, midi: 48, staff: 2 },
    ];
    let voices = BTreeMap::from([
        (0, SourceVoice { staff: 2, voice: 2, opposite_stem: false }),
        (1, SourceVoice { staff: 2, voice: 2, opposite_stem: false }),
    ]);
    let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&notes, &metric, Some(&voices), false)
        .unwrap();
    let (xml, _, _) = build_musicxml("Synthetic tied octave", &notes, 3, &mut plan, None).unwrap();
    let arrangement = crate::arrangement::arrange_piano_musicxml_with_profile(
        &xml,
        crate::arrangement::ArrangementProfile {
            rhythmic_courses: Some(&plan),
            ..Default::default()
        },
    );

    for document in [&xml, &arrangement.xml] {
        let doc = roxmltree::Document::parse(document).unwrap();
        for (index, measure) in doc.descendants().filter(|node| node.has_tag_name("measure")).enumerate() {
            let pitches: Vec<_> = measure.children().filter(|node| {
                node.has_tag_name("note") && node.children().any(|child| child.has_tag_name("pitch"))
            }).collect();
            let expected_type = if index == 0 { "half" } else { "whole" };
            assert_eq!(pitches.len(), 2);
            for (member, pitch) in pitches.iter().enumerate() {
                let tie = |kind| pitch.children().any(|child| {
                    child.has_tag_name("tie") && child.attribute("type") == Some(kind)
                });
                assert_eq!(pitch.children().any(|child| child.has_tag_name("chord")), member == 1);
                assert_eq!(tie("stop"), index > 0);
                assert_eq!(tie("start"), index < 2);
                assert!(pitch.children().any(|child| {
                    child.has_tag_name("type") && child.text() == Some(expected_type)
                }));
            }
        }
    }
    assert!(!aligned_long_tie_value(96, 192, 384, 96));
    assert!(!aligned_long_tie_value(0, 192, 288, 96));
}

#[test]
fn preserve_attacks_keeps_distinct_chord_attacks_visible() {
    use crate::arrangement::ArrangementMaterialDispositionKind::VisiblePitched;

    let pitches = [60, 64, 69, 74];
    for copies in [1, 2] {
        let notes = [0, MEASURE_DIVISIONS].into_iter().take(copies)
            .flat_map(|onset| pitches.map(|midi| (onset, midi)))
            .enumerate().map(|(ordinal, (onset, midi))| MidiSeedNote {
                len: MEASURE_DIVISIONS,
                midi,
                ..note(ordinal, onset, 1)
            }).collect::<Vec<_>>();
        let metric = MidiSeedRhythmicCourses::create_primary_presentations(
            &notes, copies, MEASURE_DIVISIONS,
        ).unwrap().metric;
        for preserve_attacks in [false, true] {
            let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(
                &notes, &metric, None, preserve_attacks,
            ).unwrap();
            let (seed, _, attacks) = build_musicxml(
                "Synthetic", &notes, copies, &mut plan, None,
            ).unwrap();
            let arrangement = crate::arrangement::arrange_piano_musicxml_with_profile(
                &seed,
                crate::arrangement::ArrangementProfile {
                    rhythmic_courses: Some(&plan),
                    preserve_reference: true,
                    ..Default::default()
                },
            );
            assert_eq!(attacks.len(), notes.len());
            for measure in 1..=copies {
                let visible_attacks = attacks.iter()
                    .filter(|attack| attack.measure_ordinal == measure)
                    .filter(|attack| arrangement.stats.material_disposition_rows.iter().any(|row| {
                        row.measure_ordinal == attack.measure_ordinal
                            && row.raw_note_index == attack.raw_note_index
                            && row.disposition == VisiblePitched
                    }))
                    .count();
                let expected = if preserve_attacks { pitches.len() } else { 2 };
                assert_eq!(visible_attacks, expected);
            }
        }
    }
}

#[test]
fn aligned_hand_span_split_requires_two_playable_hands() {
    let notes = [
        MidiSeedNote { midi: 39, ..note(0, 0, 2) },
        MidiSeedNote { midi: 52, ..note(1, 0, 1) },
        MidiSeedNote { midi: 58, ..note(2, 0, 1) },
        MidiSeedNote { midi: 67, ..note(3, 0, 1) },
    ];
    let group = |notes: &[MidiSeedNote]| notes.iter().enumerate()
        .map(|(index, note)| (index, note.midi)).collect::<Vec<_>>();
    assert_eq!(aligned_hand_span_split(&group(&notes), |_, index| notes[index].staff), None);

    let mut notes = notes;
    notes[0].midi = 41;
    assert_eq!(aligned_hand_span_split(&group(&notes), |_, index| notes[index].staff), Some((2, 15)));

    notes[1].staff = 2;
    assert_eq!(aligned_hand_span_split(&group(&notes), |_, index| notes[index].staff), None);
}

#[test]
fn upper_stepwise_continuation_keeps_its_own_voice_over_lower_activity() {
    let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], 1, 384).unwrap().metric;
    let source = |source_ordinal, pos, len, midi| MidiSeedNote {
        source_ordinal, pos, len, midi, staff: 1,
    };
    let notes = [
        source(0, 96, 48, 77),
        source(1, 96, 24, 65),
        source(2, 120, 24, 69),
        source(3, 144, 24, 76),
        source(4, 168, 24, 74),
        source(5, 168, 24, 67),
    ];
    let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&notes, &metric, None, true)
        .unwrap();
    let (xml, _, _) = build_musicxml("Synthetic", &notes, 1, &mut plan, None).unwrap();
    let upper = plan.planned.iter()
        .filter(|note| note.independent)
        .map(|note| (note.pitch, note.interval.clone()))
        .collect::<Vec<_>>();
    assert_eq!(upper, [(Some(77), 96..144), (Some(76), 144..168), (Some(74), 168..192)]);
    assert!(plan.planned.iter()
        .filter(|note| matches!(note.pitch, Some(65 | 69 | 67)))
        .all(|note| !note.independent && note.opposite_stem));
    plan.validate_seed_binding(&xml).unwrap();
}

#[test]
fn upper_sustain_keeps_its_release_over_multiple_lower_attacks() {
    let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], 2, 384).unwrap().metric;
    let mut notes = [
        MidiSeedNote { midi: 79, len: 96, ..note(0, 96, 1) },
        MidiSeedNote { midi: 67, ..note(1, 120, 1) },
        MidiSeedNote { midi: 69, ..note(2, 144, 1) },
        MidiSeedNote { midi: 72, ..note(3, 168, 1) },
    ];
    let repeated = notes.iter().copied().chain(notes.iter().map(|source| MidiSeedNote {
        pos: source.pos + 384,
        source_ordinal: source.source_ordinal + notes.len(),
        ..*source
    })).collect::<Vec<_>>();
    let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&repeated, &metric, None, true)
        .unwrap();
    let (xml, _, attacks) = build_musicxml("Synthetic", &repeated, 2, &mut plan, None).unwrap();
    let sustains = plan.planned.iter().filter(|occurrence| occurrence.independent)
        .map(|occurrence| (occurrence.source, occurrence.interval.clone())).collect::<Vec<_>>();
    assert_eq!(sustains, [(Some(0), 96..192), (Some(4), 480..576)]);
    assert_eq!(attacks.len(), repeated.len());
    plan.validate_seed_binding(&xml).unwrap();

    notes[2].midi = 81;
    let plan = MidiSeedRhythmicCourses::create_metric_presentations(&notes, &metric, None, true)
        .unwrap();
    let sustain = plan.planned.iter().find(|occurrence| occurrence.source == Some(0)).unwrap();
    assert!(!sustain.independent);
}

#[test]
fn simultaneous_sustains_keep_one_attack_each_across_a_barline() {
    let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], 4, 192).unwrap().metric;
    let notes = [
        MidiSeedNote { midi: 77, len: 120, ..note(0, 96, 1) },
        MidiSeedNote { midi: 73, len: 120, ..note(1, 96, 1) },
        MidiSeedNote { midi: 65, ..note(2, 96, 1) },
        MidiSeedNote { midi: 67, ..note(3, 120, 1) },
        MidiSeedNote { midi: 69, ..note(4, 192, 1) },
        MidiSeedNote { midi: 77, ..note(5, 216, 1) },
    ];
    let repeated = notes.iter().copied().chain(notes.iter().map(|source| MidiSeedNote {
        pos: source.pos + 384,
        source_ordinal: source.source_ordinal + notes.len(),
        ..*source
    })).collect::<Vec<_>>();
    let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&repeated, &metric, None, true).unwrap();
    let (xml, _, attacks) = build_musicxml("Synthetic", &repeated, 4, &mut plan, None).unwrap();
    assert_eq!(attacks.len(), repeated.len());
    for (source, offset) in [(0, 0), (1, 0), (6, 384), (7, 384)] {
        let fragments = plan.planned.iter().filter(|note| note.source == Some(source))
            .map(|note| (note.interval.clone(), note.tie_start, note.tie_stop)).collect::<Vec<_>>();
        assert_eq!(fragments, [(96 + offset..192 + offset, true, false), (192 + offset..216 + offset, false, true)]);
    }
    assert_eq!(plan.source_voice_continuation_slots().len(), 4);
    plan.validate_seed_binding(&xml).unwrap();
    let arrangement = crate::arrangement::arrange_piano_musicxml_with_profile(
        &xml,
        crate::arrangement::ArrangementProfile {
            rhythmic_courses: Some(&plan),
            preserve_reference: true,
            ..Default::default()
        },
    );
    let arranged = crate::musicxml::parse_musicxml(&arrangement.xml).unwrap();
    for measure in [1, 3] {
        let tied_pitches = arranged.parts[0].measures[measure].events.iter()
            .filter(|event| event.start == 0 && event.tie_stop && event.duration == 24)
            .filter_map(|event| event.pitch.as_ref().map(|pitch| pitch.midi))
            .collect::<BTreeSet<_>>();
        assert_eq!(tied_pitches, BTreeSet::from([73, 77]));
    }

    let mut conflicting = notes;
    conflicting[5].pos = 204;
    let plan = MidiSeedRhythmicCourses::create_metric_presentations(&conflicting, &metric, None, true).unwrap();
    assert!(plan.source_voice_continuation_slots().is_empty());
}

#[test]
fn last_attack_keeps_its_exact_release_across_a_barline() {
    let metric = MetricTimeline::from_written_meter(2, 4, 48, 624).unwrap();
    let notes = [
        MidiSeedNote { midi: 77, len: 192, ..note(0, 0, 1) },
        MidiSeedNote { midi: 65, len: 48, ..note(1, 192, 1) },
        MidiSeedNote { midi: 74, len: 192, ..note(2, 384, 1) },
        MidiSeedNote { midi: 67, len: 48, ..note(3, 576, 1) },
    ];
    let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&notes, &metric, None, true)
        .unwrap();
    let (xml, _, attacks) = build_musicxml("Synthetic", &notes, metric.len(), &mut plan, None)
        .unwrap();
    assert_eq!(attacks.len(), notes.len());
    for (source, start, boundary, release) in [(0, 0, 48, 192), (2, 384, 432, 576)] {
        let fragments = plan.planned.iter().filter(|fragment| fragment.source == Some(source))
            .map(|fragment| (fragment.interval.clone(), fragment.tie_start, fragment.tie_stop))
            .collect::<Vec<_>>();
        assert_eq!(fragments, [(start..boundary, true, false), (boundary..release, false, true)]);
    }
    plan.validate_seed_binding(&xml).unwrap();
    let arrangement = crate::arrangement::arrange_piano_musicxml_with_profile(
        &xml,
        crate::arrangement::ArrangementProfile {
            rhythmic_courses: Some(&plan),
            preserve_reference: true,
            ..Default::default()
        },
    );
    let arranged = crate::musicxml::parse_musicxml(&arrangement.xml).unwrap();
    for (bar, pitch) in [(1, 77), (3, 74)] {
        let duration: i64 = arranged.parts[0].measures[bar].events.iter()
            .filter(|event| event.tie_stop && event.pitch.as_ref().is_some_and(|p| p.midi == pitch))
            .map(|event| event.duration)
            .sum();
        assert_eq!(duration, 144);
    }
}

#[test]
fn simultaneous_upper_notes_keep_distinct_releases() {
    let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], 2, 192).unwrap().metric;
    let notes = [
        MidiSeedNote { midi: 83, len: 120, ..note(0, 0, 1) },
        MidiSeedNote { midi: 79, len: 144, ..note(1, 0, 1) },
        MidiSeedNote { midi: 70, len: 18, ..note(2, 0, 1) },
        MidiSeedNote { midi: 74, ..note(3, 24, 1) },
        MidiSeedNote { midi: 72, ..note(4, 48, 1) },
        MidiSeedNote { midi: 76, ..note(5, 72, 1) },
        MidiSeedNote { midi: 79, ..note(6, 144, 1) },
    ];
    let repeated = notes.iter().copied().chain(notes.iter().map(|source| MidiSeedNote {
        pos: source.pos + 192,
        source_ordinal: source.source_ordinal + notes.len(),
        ..*source
    })).collect::<Vec<_>>();
    let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&repeated, &metric, None, true).unwrap();
    let (xml, _, attacks) = build_musicxml("Synthetic", &repeated, 2, &mut plan, None).unwrap();
    assert_eq!(attacks.len(), repeated.len());
    let arranged = crate::arrangement::arrange_piano_musicxml_with_profile(
        &xml,
        crate::arrangement::ArrangementProfile {
            rhythmic_courses: Some(&plan),
            preserve_reference: true,
            ..Default::default()
        },
    );
    let score = crate::musicxml::parse_musicxml(&arranged.xml).unwrap();
    for bar in &score.parts[0].measures {
        let continuations = bar.events.iter().filter(|event| event.tie_stop)
            .filter_map(|event| event.pitch.as_ref().map(|pitch| (pitch.midi, event.start, event.duration)))
            .collect::<BTreeSet<_>>();
        assert!(continuations.contains(&(83, 96, 24)));
        assert!(continuations.contains(&(79, 96, 48)));
    }
}

fn jump_fixture(pitches: &[(i64, i32, u8, i64)]) -> Vec<MidiSeedNote> {
    pitches.iter().enumerate().map(|(source_ordinal, &(pos, midi, staff, len))| {
        MidiSeedNote { source_ordinal, pos, len, midi, staff }
    }).collect()
}

fn smooth_fixture(notes: &mut [MidiSeedNote]) -> Vec<SourceSelectionDecisionEvidence> {
    let metric = MetricTimeline::from_midi(
        crate::midi::source_time::SourceTimeDomain { source_ppq: 96, divisions: 96 },
        &[],
        768,
    ).unwrap();
    let mut decisions = Vec::new();
    smooth_fast_staff_jumps(&metric, notes, &CandidateProfile::jump_smooth(), None, false, &mut decisions);
    decisions
}

#[test]
fn equal_opposing_hand_jumps_keep_the_incumbent_assignment() {
    let mut notes = jump_fixture(&[(0, 85, 1, 12), (24, 64, 1, 12), (48, 43, 2, 12)]);
    let before = notes.clone();
    assert!(smooth_fixture(&mut notes).is_empty());
    assert_eq!(notes, before);
}

#[test]
fn beneficial_handoffs_preserve_attacks_and_are_independent_across_gaps() {
    let mut notes = jump_fixture(&[
        (0, 85, 1, 12), (24, 60, 1, 12), (48, 58, 2, 12),
        (384, 85, 1, 12), (408, 60, 1, 12), (432, 58, 2, 12),
    ]);
    let mut expected = notes.clone();
    expected[1].staff = 2;
    expected[4].staff = 2;
    let decisions = smooth_fixture(&mut notes);
    assert_eq!(notes, expected);
    assert_eq!(decisions.iter().map(|event| event.source_ordinal).collect::<Vec<_>>(), [1, 4]);
    assert_eq!(decisions[0].witness_a_source_ordinal, Some(0));
    assert_eq!(decisions[0].witness_b_source_ordinal, Some(1));
    assert!(smooth_fixture(&mut notes).is_empty());
}

#[test]
fn larger_receiving_jump_or_sustained_keys_prevent_a_handoff() {
    for pitches in [
        [(0, 85, 1, 12), (24, 64, 1, 12), (48, 40, 2, 12)],
        [(0, 85, 1, 12), (24, 64, 1, 48), (48, 48, 2, 24)],
        [(0, 85, 1, 12), (24, 64, 1, 48), (24, 48, 2, 24)],
    ] {
        let mut notes = jump_fixture(&pitches);
        let before = notes.clone();
        assert!(smooth_fixture(&mut notes).is_empty());
        assert_eq!(notes, before);
    }
}

#[test]
fn newly_joined_donor_neighbors_are_included_in_the_comparison() {
    let mut notes = jump_fixture(&[(0, 85, 1, 12), (24, 64, 1, 12), (48, 99, 1, 12)]);
    let before = notes.clone();
    // Removing the middle note really improves this hand, so it moves.
    assert_eq!(smooth_fixture(&mut notes).len(), 1);
    assert_eq!(notes[1].staff, 2);
    assert_eq!(notes[0], before[0]);
    assert_eq!(notes[2], before[2]);

    // The old edge exceeds the limit, but removing its high endpoint
    // would join the donor's remaining notes into a still larger jump.
    let mut notes = jump_fixture(&[(0, 45, 2, 12), (24, 70, 2, 12), (48, 82, 2, 12)]);
    let before = notes.clone();
    assert!(smooth_fixture(&mut notes).is_empty());
    assert_eq!(notes, before);
}

#[test]
fn hand_jump_smoothing_keeps_the_owner_split_of_a_wide_onset() {
    let mut notes = jump_fixture(&[
        (24, 45, 2, 12), (24, 52, 2, 12), (24, 63, 1, 12), (48, 30, 2, 12),
    ]);
    let profile = CandidateProfile::jump_smooth();
    let group = [(0, 45), (1, 52), (2, 63)];
    assert_eq!(wide_piano_hand_split(&group, 18, &profile), Some(2));
    let before = notes.clone();
    assert!(smooth_fixture(&mut notes).is_empty());
    assert_eq!(notes, before);

    // Bring the lower pitch closer so that the onset no longer needs a split.
    notes[0].midi = 50;
    assert_eq!(smooth_fixture(&mut notes).len(), 1);
    assert_eq!(notes[1].staff, 1);
}

#[test]
fn attack_preserving_seed_commits_the_feasible_onset_split_directly() {
    let metric = MetricTimeline::from_midi(
        crate::midi::source_time::SourceTimeDomain { source_ppq: 96, divisions: 96 },
        &[],
        768,
    ).unwrap();
    let profile = CandidateProfile::jump_smooth();
    let source = jump_fixture(&[(24, 43, 2, 12), (24, 44, 2, 12), (24, 56, 2, 12), (24, 68, 1, 12)]);
    let mut notes = source.clone();
    let mut decisions = Vec::new();
    redistribute_midi_seed_hands_for_piano(&metric, &mut notes, &profile, None, false, &mut decisions);
    assert_eq!(notes, source);
    assert!(decisions.is_empty());

    redistribute_midi_seed_hands_for_piano(&metric, &mut notes, &profile, None, true, &mut decisions);
    let mut expected = source;
    expected[2].staff = 1;
    assert_eq!(notes, expected);
    assert_eq!(decisions.len(), 1);
    assert_eq!(decisions[0].pass, SourceSelectionDecisionPass::AlignedHandSpanSplit);
    assert_eq!(decisions[0].observed_primary, 13);
    assert_eq!(decisions[0].threshold_primary, 12);
    assert_eq!(decisions[0].source_ordinal, 2);

    redistribute_wide_simultaneities(
        &metric, &mut notes, &profile, None,
        SourceSelectionDecisionPass::AlignedHandSpanSplit, 0, &mut decisions,
    );
    assert_eq!(notes, expected);
    assert_eq!(decisions.len(), 1);
}

#[test]
fn repeated_accompaniment_hand_preserves_attacks_and_refuses_conflicts() {
    let chord = [
        MidiSeedNote { midi: 52, len: 72, ..note(0, 72, 2) },
        MidiSeedNote { midi: 61, len: 72, ..note(1, 72, 1) },
        MidiSeedNote { midi: 52, len: 48, ..note(2, 144, 2) },
        MidiSeedNote { midi: 61, len: 48, ..note(3, 144, 2) },
        MidiSeedNote { midi: 69, len: 120, ..note(4, 48, 1) },
    ];
    let surrounding = [
        MidiSeedNote { midi: 45, len: 12, ..note(5, 120, 2) },
        MidiSeedNote { midi: 52, len: 72, ..note(5, 0, 1) },
        MidiSeedNote { midi: 61, len: 72, ..note(6, 0, 1) },
    ];
    let copy = chord.iter().map(|n| MidiSeedNote {
        source_ordinal: n.source_ordinal + chord.len(),
        pos: n.pos + 384,
        midi: n.midi + 4,
        ..*n
    });
    let mut notes: Vec<_> = chord.into_iter().chain(copy).collect();
    let mut trace = Vec::new();
    restore_repeated_accompaniment_hand(&mut notes, &mut trace);
    assert_eq!(notes[1].staff, 2);
    assert_eq!(notes[6].staff, 2);
    assert_eq!(trace.len(), 2);
    assert!(trace.iter().all(|decision| decision.pass == SourceSelectionDecisionPass::RepeatedAccompanimentHand));
    assert_eq!((notes[1].pos, notes[1].len, notes[1].midi), (72, 72, 61));

    for extra in [&surrounding[..1], &surrounding[1..]] {
        let mut notes: Vec<_> = chord.into_iter().chain(extra.iter().copied()).collect();
        restore_repeated_accompaniment_hand(&mut notes, &mut Vec::new());
        assert_eq!(notes[1].staff, 1);
    }
    let mut different_release = chord;
    different_release[1].len -= 12;
    restore_repeated_accompaniment_hand(&mut different_release, &mut Vec::new());
    assert_eq!(different_release[1].staff, 1);

    for (staff, expected, decisions) in [(1, 2, 1), (2, 1, 0)] {
        let companion = MidiSeedNote { midi: 63, len: 48, ..note(5, 144, staff) };
        let fixture: Vec<_> = chord.into_iter().chain([companion]).collect();
        let relocated = fixture.iter().map(|n| MidiSeedNote {
            source_ordinal: n.source_ordinal + fixture.len(),
            pos: n.pos + 384,
            midi: n.midi + 4,
            ..*n
        });
        let mut notes: Vec<_> = fixture.iter().copied().chain(relocated).collect();
        let mut trace = Vec::new();
        restore_repeated_accompaniment_hand(&mut notes, &mut trace);
        assert_eq!(notes[1].staff, expected);
        assert_eq!(notes[fixture.len() + 1].staff, expected);
        assert_eq!(notes[5], companion);
        assert_eq!(trace.len(), decisions * 2);
    }
}

#[test]
fn initial_split_keeps_held_keys_in_the_hand_geometry() {
    let metric = MetricTimeline::from_midi(
        crate::midi::source_time::SourceTimeDomain { source_ppq: 96, divisions: 96 },
        &[],
        768,
    ).unwrap();
    let profile = CandidateProfile::span12();
    let fixture = [
        (0, 41, 2, 48), (0, 65, 1, 24),
        (24, 56, 2, 24), (24, 63, 1, 24),
        (48, 56, 2, 24), (48, 63, 1, 24),
    ];
    let duplicated = fixture.into_iter().chain(fixture.map(|(onset, pitch, staff, length)| {
        (onset + 384, pitch + 2, staff, length)
    })).collect::<Vec<_>>();
    let mut notes = jump_fixture(&duplicated);
    let source = notes.clone();
    let mut decisions = Vec::new();
    redistribute_midi_seed_hands_for_piano(&metric, &mut notes, &profile, None, true, &mut decisions);

    let mut expected = source;
    expected[2].staff = 1;
    expected[8].staff = 1;
    assert_eq!(notes, expected);
    assert_eq!(decisions.iter().map(|decision| (decision.source_ordinal, decision.observed_primary))
        .collect::<Vec<_>>(), vec![(2, 15), (8, 15)]);
}

#[test]
fn tied_chord_stays_together_on_the_continuing_melodic_course() {
    let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], 4, 192).unwrap().metric;
    let fixture = [
        (96, 79, 1, 120), (96, 75, 1, 120), (96, 64, 1, 24), (120, 67, 1, 24),
        (192, 65, 1, 24), (216, 81, 1, 24), (216, 69, 1, 24),
        (240, 81, 1, 24), (240, 67, 1, 24), (264, 79, 1, 24), (264, 65, 1, 24),
    ];
    let duplicate = fixture.map(|(pos, pitch, staff, len)| (pos + 384, pitch - 2, staff, len));
    let notes = jump_fixture(&[fixture, duplicate].concat());
    let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&notes, &metric, None, true).unwrap();
    let (xml, _, attacks) = build_musicxml("Synthetic", &notes, 4, &mut plan, None).unwrap();
    plan.validate_seed_binding(&xml).unwrap();
    assert_eq!(attacks.len(), notes.len());
    assert_eq!(plan.source_voice_continuation_slots().len(), 4);
    for (source, start) in [(0, 192), (fixture.len(), 576)] {
        let tails = plan.planned.iter().filter(|note| note.tie_stop && note.interval.start == start)
            .map(|note| (note.source, note.chord, note.independent, note.interval.end)).collect::<Vec<_>>();
        assert_eq!(tails, [
            (Some(source + 1), false, false, start + 24),
            (Some(source), true, false, start + 24),
        ]);
        let melodic_attacks = plan.planned.iter().filter(|note| {
            !note.independent && !note.tie_stop && note.source.is_some()
                && (start..start + 96).contains(&note.interval.start)
        }).map(|note| note.source.unwrap()).collect::<Vec<_>>();
        assert_eq!(melodic_attacks, [source + 5, source + 7, source + 9]);
    }
}

#[test]
fn a_single_tied_tail_joins_the_melody_over_a_complete_lower_course() {
    let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], 2, 192)
        .unwrap()
        .metric;
    let mut notes = jump_fixture(&[
        (96, 81, 1, 120),
        (96, 66, 1, 48),
        (144, 68, 1, 48),
        (192, 70, 1, 24),
        (216, 79, 1, 24),
        (216, 71, 1, 24),
        (240, 77, 1, 48),
        (240, 69, 1, 48),
    ]);

    for compatible in [true, false] {
        if !compatible {
            notes[5].len = 12;
        }
        let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(
            &notes, &metric, None, true,
        ).unwrap();
        let (xml, _, attacks) = build_musicxml("Synthetic", &notes, 2, &mut plan, None).unwrap();
        let tail = plan.planned.iter().find(|occurrence| occurrence.tie_stop).unwrap();
        assert_eq!(tail.source, Some(0));
        assert_eq!(tail.interval, 192..216);
        assert_eq!(tail.independent, !compatible);
        if compatible {
            for source in [3, 5, 7] {
                let lower = plan.planned.iter()
                    .find(|occurrence| occurrence.source == Some(source)).unwrap();
                assert!(lower.independent && lower.opposite_stem);
            }
            for source in [4, 6] {
                let melody = plan.planned.iter()
                    .find(|occurrence| occurrence.source == Some(source)).unwrap();
                assert!(!melody.independent && !melody.chord);
            }
        }
        assert_eq!(attacks.len(), notes.len());
        plan.validate_seed_binding(&xml).unwrap();
    }
}

#[test]
fn independent_accompaniment_keeps_its_playing_hand_across_staff_projection() {
    let figure = [
        (96, 65, 1, 120),
        (96, 54, 1, 48),
        (144, 56, 1, 48),
        (192, 55, 1, 24),
        (216, 64, 1, 24),
        (216, 57, 1, 24),
        (240, 62, 1, 48),
        (240, 55, 1, 48),
        (288, 64, 1, 96),
        (192, 36, 2, 96),
        (288, 43, 2, 96),
    ];
    for copies in [1, 2] {
        let notes = jump_fixture(&(0..copies).flat_map(|copy| {
            figure.map(|(time, pitch, staff, duration)| (time + copy * 384, pitch, staff, duration))
        }).collect::<Vec<_>>());
        let measures = usize::try_from(copies * 2).unwrap();
        let metric = MidiSeedRhythmicCourses::create_primary_presentations(&[], measures, 192)
            .unwrap().metric;
        let mut plan = MidiSeedRhythmicCourses::create_metric_presentations(&notes, &metric, None, true)
            .unwrap();
        let (xml, _, _) = build_musicxml("Synthetic", &notes, measures, &mut plan, None).unwrap();
        let arranged = crate::arrangement::arrange_piano_musicxml_with_profile(
            &xml, crate::arrangement::ArrangementProfile {
                rhythmic_courses: Some(&plan),
                preserve_reference: true,
                ..Default::default()
            },
        );
        let mut score = crate::musicxml::parse_musicxml(&arranged.xml).unwrap();
        let written = arranged.stats.rhythmic_courses.as_ref().unwrap()
            .written_staff_only_events(&score).unwrap();
        assert_eq!(written.len(), usize::try_from(copies).unwrap());
        let options = crate::rules::CheckOptions { strict: true };
        let hand_warnings = |report: &crate::model::CheckReport| {
            report.issues.iter().filter(|issue| issue.issue_type == "PIANO_HAND_SPAN_LARGE").count()
        };
        assert!(hand_warnings(&crate::rules::run_checks(&score, &options)) > 0);
        let actual = crate::rules::run_checks_with_written_staff(&score, &options, false, &written);
        assert_eq!(actual.summary.blockers, 0);
        assert!(hand_warnings(&actual) < hand_warnings(&crate::rules::run_checks(&score, &options)));
        let index = *written[&2].first().unwrap() - 1;
        let event = &mut score.parts[0].measures[1].events[index];
        event.pitch = Some(crate::model::Pitch::new("C".into(), 0, 7));
        assert!(hand_warnings(&crate::rules::run_checks_with_written_staff(
            &score, &options, false, &written,
        )) > hand_warnings(&actual));
    }
}

#[test]
fn adjacent_split_keeps_the_upper_line_and_its_metric_release() {
    let metric = MetricTimeline::from_written_meter(2, 4, 0, 192).unwrap();
    let profile = CandidateProfile::span12();
    let source = jump_fixture(&[
        (48, 42, 2, 24), (48, 58, 2, 24),
        (72, 53, 2, 24), (72, 59, 2, 24),
        (96, 48, 2, 24), (120, 65, 1, 24),
    ]);
    for (step, expected_staff) in [(59, 1), (63, 2)] {
        let mut notes = source.clone();
        notes[3].midi = step;
        let mut decisions = Vec::new();
        redistribute_midi_seed_hands_for_piano(&metric, &mut notes, &profile, None, true, &mut decisions);
        assert_eq!(notes[3].staff, expected_staff);
        assert_eq!(notes[3].len, 24);
        if expected_staff == 1 {
            let plan = MidiSeedRhythmicCourses::create_metric_presentations(&notes, &metric, None, true).unwrap();
            let note = plan.planned.iter().find(|note| note.source == Some(3)).unwrap();
            assert_eq!(note.interval, 72..96);
            assert!(!note.tie_start);
        }
    }
}
