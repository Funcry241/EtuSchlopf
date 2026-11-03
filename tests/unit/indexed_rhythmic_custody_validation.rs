use super::*;
use crate::midi_seed::rhythmic_course_creation_tests::arrangement_pitched_fixture;

// Exact r1580 validator, retained only as a differential oracle.
impl ArrangementRhythmicCourses {
    fn validate_binding_linear_reference(&self, xml: &str) -> Result<()> {
        ensure!(self.snapshot == xml, "Course custody belongs to another document state");
        let mut offset = 0;
        let mut context = RhythmContext::default();
        let mut count = 0;
        let mut identities = self.coverage.iter().map(|c| c.creation).collect::<BTreeSet<_>>();
        ensure!(identities.len() == self.coverage.len(), "Duplicate course creation");
        for occurrence in &self.occurrences {
            ensure!(identities.insert(occurrence.creation), "Duplicate physical creation");
            ensure!(self.history.get(&occurrence.creation) == Some(&occurrence.edge()), "Missing actual creation edge");
            let root = self.roots.get(&occurrence.root).ok_or_else(|| anyhow!("Foreign root origin"))?;
            let mut visited = BTreeSet::new();
            let mut id = occurrence.creation;
            loop {
                ensure!(visited.insert(id), "Cyclic creation ancestry");
                let edge = self.history.get(&id).ok_or_else(|| anyhow!("Missing creation ancestor"))?;
                ensure!(edge.root == root.creation && edge.course == root.course && edge.source == root.source,
                    "Creation ancestry changed course or source");
                if let Some(parent) = edge.parent {
                    let ancestor = self.history.get(&parent).ok_or_else(|| anyhow!("Missing parent creation"))?;
                    ensure!(edge.interval.start >= ancestor.interval.start && edge.interval.end <= ancestor.interval.end,
                        "Creation ancestry extended licensed support");
                    id = parent;
                } else {
                    ensure!(id == root.creation && edge.interval == root.interval, "Foreign creation root");
                    break;
                }
            }
            let root_staff = extract_first_i64(&root.note, "staff");
            let final_staff = extract_first_i64(&occurrence.note, "staff");
            let course = self.coverage.iter().find(|c| c.creation == root.course)
                .ok_or_else(|| anyhow!("Missing original course"))?;
            ensure!(course.presentation == SeedCoursePresentation::Primary && root_staff == Some(i64::from(course.staff)),
                "Original presentation is not the issued seed decision");
            ensure!(if occurrence.restaffed { root.source.is_some() && root_staff == Some(1) && final_staff == Some(2) }
                else { final_staff == root_staff }, "Unbound staff transition");
            ensure!(if occurrence.restified { root.source.is_some() && root.note.contains("<pitch>") && occurrence.note.contains("<rest") }
                else { extract_first_text(&occurrence.note, "pitch") == extract_first_text(&root.note, "pitch") },
                "Unbound sounding disposition change");
        }
        for root in self.roots.values() {
            let mut fragments = self.occurrences.iter().filter(|o| o.root == root.creation).collect::<Vec<_>>();
            fragments.sort_by_key(|o| (o.interval.start, o.interval.end));
            let mut end = root.interval.start;
            for fragment in fragments {
                ensure!(fragment.interval.start == end && (fragment.interval.end > end
                    || (root.is_grace() && fragment.is_grace())),
                    "Source-origin split lost or duplicated represented support");
                end = fragment.interval.end;
            }
            ensure!(end == root.interval.end, "Source-origin split does not cover its original interval");
        }
        for span in find_measure_spans(xml) {
            let text = &xml[span.start..span.end];
            context = context.update_from_measure(text);
            let measure = self.measure(text, offset, context.expected_duration())?;
            count += measure.incoming.len();
            offset += context.expected_duration();
        }
        ensure!(count == self.occurrences.len(), "Unbound occurrence outside the metric domain");
        ensure!(self.coverage.iter().all(|c| c.coverage.start == 0 && c.coverage.end == offset),
            "Arrangement implicitly changed the declared lifetime");
        Ok(())
    }
}

fn outcome(result: Result<()>) -> std::result::Result<(), String> {
    result.map_err(|error| format!("{error:#}"))
}

fn compare(state: &ArrangementRhythmicCourses, xml: &str) {
    assert_eq!(outcome(state.validate_binding(xml)),
        outcome(state.validate_binding_linear_reference(xml)));
}

#[test]
fn indexed_custody_matches_linear_validation_and_exact_failures() {
    for measures in [1, 3, 16] {
        let rows = (0..measures).flat_map(|m| [
            (m * 4, m as i64 * 384, 1, 60),
            (m * 4 + 1, m as i64 * 384, 1, 64),
            (m * 4 + 2, m as i64 * 384 + 96, 2, 48),
            (m * 4 + 3, m as i64 * 384 + 360, 2, 52),
        ]).collect::<Vec<_>>();
        let (xml, seed) = arrangement_pitched_fixture(&rows, measures);
        let initial = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
        compare(&initial, &xml);
        compare(&initial, &(xml.clone() + " "));
        let mut variants = Vec::new();
        let mut changed = initial.clone(); changed.occurrences.pop(); variants.push(changed);
        let mut changed = initial.clone(); changed.occurrences.push(changed.occurrences[0].clone()); variants.push(changed);
        let mut changed = initial.clone(); changed.occurrences.reverse(); variants.push(changed);
        let mut changed = initial.clone(); changed.history.remove(&changed.occurrences[0].creation); variants.push(changed);
        let mut changed = initial.clone(); changed.roots.remove(&changed.occurrences[0].root); variants.push(changed);
        let mut changed = initial.clone(); changed.coverage.push(changed.coverage[0].clone()); variants.push(changed);
        let mut changed = initial.clone(); changed.coverage[0].coverage.end += 24; variants.push(changed);
        let mut changed = initial.clone(); changed.occurrences[0].note.push(' '); variants.push(changed);
        // Even a repeated root value under an unused key keeps legacy behavior.
        let mut changed = initial.clone();
        let repeated = changed.roots.values().next().unwrap().clone();
        changed.roots.insert(changed.coverage[0].creation, repeated);
        variants.push(changed);
        for changed in variants { compare(&changed, &xml); }
    }
}

#[test]
fn indexed_custody_keeps_actual_split_and_hidden_chord_support() {
    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture(
        &[(11, 0, 1), (12, 0, 1), (13, 384, 2)], 2);
    let result = arrange_piano_musicxml_with_profile(&xml, ArrangementProfile {
        rhythmic_courses: Some(&seed), preserve_reference: true,
        ..ArrangementProfile::default()
    });
    let state = result.stats.rhythmic_courses.as_ref().unwrap();
    compare(state, &result.xml);
    for index in 0..state.occurrences.len() {
        let mut missing = state.as_ref().clone(); missing.occurrences.remove(index);
        compare(&missing, &result.xml);
    }
}
