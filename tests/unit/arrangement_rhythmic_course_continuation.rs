use super::*;
use crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture;

fn arranged(rows: &[(usize, i64, u8)], measures: usize) -> ArrangementResult {
    let (xml, seed) = arrangement_fixture(rows, measures);
    arrange_piano_musicxml_with_profile(&xml, ArrangementProfile {
        rhythmic_courses: Some(&seed), preserve_reference: true,
        ..ArrangementProfile::default()
    })
}

#[test]
fn empty_presentations_continue_across_measures() {
    let result = arranged(&[], 3);
    result.stats.rhythmic_courses.as_ref().unwrap().validate_binding(&result.xml).unwrap();
    assert_eq!(result.stats.visible_rhythm_rests_inserted, 0);
}

#[test]
fn actual_metric_splits_conserve_ancestry_and_coverage() {
    let result = arranged(&[(7, 24, 1)], 1);
    let courses = result.stats.rhythmic_courses.as_ref().unwrap();
    courses.validate_binding(&result.xml).unwrap();
    let fragments = courses.occurrences.iter().filter(|o| o.source == Some(7)).collect::<Vec<_>>();
    assert!(fragments.len() > 1);
    assert!(fragments.iter().all(|o| o.parent.is_some()));
    assert_eq!(fragments.iter().map(|o| o.interval.end - o.interval.start).sum::<i64>(), 360);
}

#[test]
fn chord_cohort_support_survives_actual_owner_splitting() {
    let result = arranged(&[(7, 0, 1), (8, 0, 1)], 1);
    result.stats.rhythmic_courses.as_ref().unwrap().validate_binding(&result.xml).unwrap();
}

#[test]
fn stale_seed_is_refused_before_receiving_origin() {
    let (xml, seed) = arrangement_fixture(&[], 1);
    assert!(ArrangementRhythmicCourses::receive(&seed, &xml.replace("<voice>1</voice>", "<voice>23</voice>")).is_err());
}

#[test]
fn imported_notes_cannot_supply_seed_custody() {
    let (xml, seed) = arrangement_fixture(&[], 1);
    let (_, foreign) = arrangement_fixture(&[(4, 0, 1)], 1);
    assert!(ArrangementRhythmicCourses::receive(&foreign, &xml).is_err());
    ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
}

#[test]
fn final_snapshot_does_not_authorize_later_xml() {
    let result = arranged(&[], 1);
    let courses = result.stats.rhythmic_courses.as_ref().unwrap();
    assert!(courses.validate_binding(&result.xml.replace("<voice>1</voice>", "<voice>23</voice>")).is_err());
}

#[test]
fn hidden_members_keep_their_complete_physical_support() {
    let result = arranged(&[(11, 0, 1), (12, 0, 1), (13, 0, 1), (14, 0, 1)], 1);
    let courses = result.stats.rhythmic_courses.as_ref().unwrap();
    courses.validate_binding(&result.xml).unwrap();
    assert!(courses.occurrences.iter().any(|o| o.source.is_some() && !is_note_block_visible(&o.note)));
    assert_eq!(result.stats.chord_cohorts_source_bound_split, 1);
    for source in 11..=14 {
        let fragments = courses.occurrences.iter().filter(|o| o.source == Some(source)).collect::<Vec<_>>();
        assert_eq!(fragments.len(), 4);
        assert_eq!(fragments.iter().map(|o| o.interval.end - o.interval.start).sum::<i64>(), 384);
        assert!(fragments.iter().all(|o| o.note.contains("<pitch>")));
    }
}

#[test]
fn independent_cohorts_and_staffs_preserve_each_source_partition() {
    let result = arranged(&[(21, 0, 1), (22, 0, 1), (31, 384, 2), (32, 384, 2)], 2);
    let courses = result.stats.rhythmic_courses.as_ref().unwrap();
    courses.validate_binding(&result.xml).unwrap();
    assert_eq!(result.stats.chord_cohorts_source_bound_split, 2);
    for (source, start) in [(21, 0), (22, 0), (31, 384), (32, 384)] {
        let fragments = courses.occurrences.iter().filter(|o| o.source == Some(source)).collect::<Vec<_>>();
        assert_eq!(fragments.first().unwrap().interval.start, start);
        assert_eq!(fragments.last().unwrap().interval.end, start + 384);
    }
}

#[test]
fn missing_and_duplicate_fragments_are_refused_even_when_course_union_is_complete() {
    let result = arranged(&[(41, 0, 1), (42, 0, 1)], 1);
    let original = result.stats.rhythmic_courses.as_ref().unwrap();
    let mut missing = original.as_ref().clone();
    let index = missing.occurrences.iter().position(|o| o.source == Some(42)).unwrap();
    missing.occurrences.remove(index);
    assert!(missing.validate_binding(&result.xml).is_err());
    let mut duplicate = original.as_ref().clone();
    duplicate.occurrences.push(duplicate.occurrences[index].clone());
    assert!(duplicate.validate_binding(&result.xml).is_err());
}

#[test]
fn missing_creation_ancestor_is_refused() {
    let result = arranged(&[(51, 24, 1)], 1);
    let mut courses = result.stats.rhythmic_courses.as_ref().unwrap().as_ref().clone();
    let parent = courses.occurrences.iter().find_map(|o| o.parent).unwrap();
    courses.history.remove(&parent);
    assert!(courses.validate_binding(&result.xml).is_err());
}

#[test]
fn unequal_member_support_is_refused_before_cohort_birth() {
    let (xml, seed) = arrangement_fixture(&[(61, 0, 1), (62, 0, 1)], 1);
    let received = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    let span = &find_measure_spans(&xml)[0];
    let mut measure = received.measure(&xml[span.start..span.end], 0, 384).unwrap();
    measure.incoming[1].interval.end -= 24;
    assert!(measure.cohort_support(0, 2, &[96, 96, 96, 96]).is_err());
    assert!(measure.history.is_empty());
    assert!(measure.outgoing.is_empty());
}

fn owner_transition(restify: bool) {
    let (xml, seed) = arrangement_fixture(&[(71, 24, 1), (72, 24, 1)], 1);
    let received = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    let span = &find_measure_spans(&xml)[0];
    let original = &xml[span.start..span.end];
    let mut measure = received.measure(original, 0, 384).unwrap();
    let raw = collect_raw_notes(original);
    let index = measure.incoming.iter().position(|o| o.source.is_some() && !o.note.contains("<chord")).unwrap();
    let source = measure.incoming[index].source;
    let note = if restify { restify_note_for_rhythm_ledger(&raw[index].text) }
        else { move_note_to_left_system_for_human_potential(&raw[index].text, false) };
    measure.disposition(index, &note, if restify { ArrangementMaterialDispositionKind::RestifiedPitched }
        else { ArrangementMaterialDispositionKind::VisiblePitched }, if restify { 1 } else { 2 }, !restify, false).unwrap();
    let mut changed = original.to_owned();
    changed.replace_range(raw[index].start..raw[index].end, &note);
    let (normalized, stats) = apply_visible_rhythm_ledger(&changed, RhythmContext::default(), Some(&mut measure), false, false);
    assert_eq!(stats.chord_cohorts_source_bound_split, 1);
    let mut final_xml = xml.clone();
    final_xml.replace_range(span.start..span.end, &normalized);
    let (occurrences, history) = measure.take();
    let final_courses = received.finish(occurrences, history, &final_xml).unwrap();
    final_courses.validate_binding(&final_xml).unwrap();
    let fragments = final_courses.occurrences.iter().filter(|o| o.source == source).collect::<Vec<_>>();
    assert_eq!(fragments.len(), if restify { 3 } else { 4 });
    assert!(fragments.iter().all(|o| if restify { o.restified && o.note.contains("<rest") }
        else { o.restaffed && extract_first_i64(&o.note, "staff") == Some(2) }));
    if restify {
        let parent = fragments[0].parent.unwrap();
        assert!(final_courses.history[&parent].parent.is_some());
    }
}

#[test]
fn restified_host_keeps_members_and_intermediate_creation() { owner_transition(true); }

#[test]
fn actual_staff_action_preserves_course_and_source() { owner_transition(false); }

#[test]
fn actual_received_note_refusal_exposes_bytes_without_replacing_its_origin() {
    let (xml, seed) = arrangement_fixture(&[(7, 0, 1)], 1);
    let received = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
    let span = find_measure_spans(&xml).into_iter().next().unwrap();
    let original = &xml[span.start..span.end];
    received.measure(original, 0, 384).unwrap();
    let foreign = original.replace("<staff>1</staff>", "<staff>2</staff>");
    assert_ne!(foreign, original);
    let refusal = received.measure(&foreign, 0, 384).err().expect("foreign note must remain refused").to_string();
    assert!(refusal.starts_with("Physical course continuation moved or changed support:"));
    assert!(refusal.contains("exact_note=false; observation_only=true;"));
    assert!(refusal.contains("expected_sha256=") && refusal.contains("observed_context_hex="));
    received.validate_binding(&xml).unwrap();
    received.measure(original, 0, 384).unwrap();
}

#[test]
fn single_target_trials_reuse_only_received_roots_and_keep_the_full_arrangement() {
    use crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture;
    for bars in [3, 4] {
        let mut rows = [60, 72, 64, 76, 67, 79, 65, 77].into_iter().enumerate()
            .map(|(i, pitch)| (i + 1, i as i64 * 48, 48, 1, pitch))
            .collect::<Vec<_>>();
        for (index, onset) in [384, 480, 576, 672].into_iter().enumerate() {
            rows.extend([(21 + index, onset, 96, 1, 72), (31 + index, onset, 96, 2, 48)]);
        }
        rows.extend([(41, 768, 384, 2, 50), (42, 768, 384, 2, 53)]);
        let (xml, seed) = arrangement_source_lengths_fixture(&rows, bars);
        let profile = ArrangementProfile {
            rhythmic_courses: Some(&seed), preserve_reference: true,
            human_potential_enabled: true, ..Default::default()
        };
        let discovery = arrange_piano_musicxml_with_profile(&xml, profile);
        let target = *discovery.stats.human_potential_structural_targets.first().unwrap();
        let trial = ArrangementProfile { human_potential_structural_target: Some(target), ..profile };
        let expected = arrange_piano_musicxml_with_profile(&xml, trial);
        let reuse = HumanPotentialMeasureReuse::new(&xml, &seed);
        let cached_trial = ArrangementProfile { human_potential_measure_reuse: Some(&reuse), ..trial };
        let separate_input = xml.clone();
        let foreign = arrange_piano_musicxml_with_profile(&separate_input, cached_trial);
        assert_eq!(foreign.xml, expected.xml);
        assert!(reuse.received.get().is_none());
        let first = arrange_piano_musicxml_with_profile(&xml, cached_trial);
        let received = reuse.received.get().expect("exact input custody received");
        assert_eq!(reuse.hits(), 0);
        let second = arrange_piano_musicxml_with_profile(&xml, cached_trial);
        assert!(std::ptr::eq(received, reuse.received.get().unwrap()));
        assert_eq!(reuse.hits(), reuse.measures.borrow().len());
        assert!(reuse.measures.borrow().contains_key(&2));
        assert!(!reuse.measures.borrow().contains_key(&3));
        assert!(!reuse.measures.borrow().contains_key(&target.measure_ordinal()));
        assert_eq!(first.xml, expected.xml);
        assert_eq!(second.xml, expected.xml);
        assert_eq!(second.stats.material_disposition_rows, expected.stats.material_disposition_rows);
        assert_eq!(second.stats.representation_lineage_rows, expected.stats.representation_lineage_rows);
        let a = first.stats.rhythmic_courses.unwrap();
        let b = second.stats.rhythmic_courses.unwrap();
        b.validate_binding(&second.xml).unwrap();
        for (old, new) in a.occurrences.iter().zip(&b.occurrences) {
            assert_eq!(old.root, new.root);
            assert_eq!(old.interval, new.interval);
            assert_eq!(old.creation == new.creation, old.parent.is_none());
        }
    }
}
