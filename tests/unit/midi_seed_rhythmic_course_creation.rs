use super::*;

fn note(source: usize, pos: i64, staff: u8) -> MidiSeedNote {
    MidiSeedNote { source_ordinal: source, pos, len: 24, midi: 60, staff }
}

fn emitted(notes: &[MidiSeedNote], measures: usize) -> (String, MidiSeedRhythmicCourses) {
    let mut plan = MidiSeedRhythmicCourses::create_primary_presentations(
        notes, measures, MEASURE_DIVISIONS,
    ).unwrap();
    let (xml, _, _) = build_musicxml("Synthetic", notes, measures, &mut plan).unwrap();
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
    assert!(build_musicxml("Synthetic", &[note(11, 24, 2)], 1, &mut fresh).is_err());
    let (_, mut used) = emitted(&notes, 1);
    assert!(build_musicxml("Synthetic", &notes, 1, &mut used).is_err());
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
    let (xml, _, _) = build_musicxml("Another project name", &notes, 1, &mut second).unwrap();
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
