use super::*;
use crate::master_state_identity::OwnerReceipts;
use crate::structural_write_authority::{apply_atomic_restification_batch_authorized,
    apply_editorial_voice_complete_rewrite_authorized};

fn received(staff: u8, shift: i64, source: usize) -> OwnedRhythmicContinuation {
    OwnedRhythmicContinuation::synthetic_pitched_received_for_test(
        &[(source, shift + 24, staff, 65), (source + 1, shift + 144, staff, 69)],
        1 + usize::from(shift > 0))
}

fn consolidated(state: &OwnedRhythmicContinuation, staff: u8, voice: &str) -> OwnedRhythmicContinuation {
    let notes = physical_inventory(state.document()).unwrap();
    let inputs = notes.iter().filter(|p| p.staff == staff && p.interval.end <= 384)
        .map(|p| (p.bytes.start, p.bytes.end)).collect::<Vec<_>>();
    let insertion = inputs.iter().map(|p| p.0).min().unwrap();
    let replacement = format!("<note><rest/><duration>384</duration><voice>{voice}</voice><type>whole</type><staff>{staff}</staff></note>");
    let (post, mut receipt) = apply_editorial_voice_complete_rewrite_authorized(
        state.document(), inputs, insertion, replacement);
    let post = post.unwrap();
    let mut receipt = receipt.take().unwrap();
    let births = receipt.take_physical_births().unwrap();
    let next = state.continue_exact(&receipt, Some(&births)).unwrap();
    next.require_complete_course_context(&post).unwrap();
    next
}

#[test]
fn seed_coverage_rest_and_seed_pitch_are_distinct_origin_facts_under_renaming_and_relocation() {
    for staff in [1, 2] {
        for (source, shift) in [(701, 0), (31, 384)] {
            let state = received(staff, shift, source);
            state.received_notation_obligations(state.document()).unwrap();
            for occurrence in &state.occurrences {
                let facts = state.received_source_origins(occurrence.creation).unwrap();
                assert_eq!(facts.leaves.len(), 1);
                let leaf = &facts.leaves[0];
                assert_eq!(leaf.creation, occurrence.origin.root);
                assert_eq!(leaf.course, occurrence.origin.course);
                assert_eq!(leaf.source, occurrence.origin.source);
                assert_eq!(leaf.kind, if occurrence.origin.source.is_some() {
                    SourceOriginKind::SeedPitched
                } else { SourceOriginKind::SeedCoverageRest });
                let evidence = facts.to_string();
                assert!(evidence.contains("musical_line_authority=none"));
                assert!(evidence.contains("silence_visibility_authority=none"));
            }
        }
    }
}

#[test]
fn original_restification_and_metric_split_retain_pitched_ancestry_instead_of_issuing_silence() {
    let state = received(1, 0, 701);
    let target = state.occurrences.iter().find(|o| o.origin.source == Some(701)).unwrap();
    let raw = &state.snapshot[target.bytes.clone()];
    let replacement = format!("<note><rest/><duration>{}</duration><voice>{}</voice><staff>1</staff></note>",
        extract_first_i64(raw, "duration").unwrap(), extract_first_text(raw, "voice").unwrap());
    let (post, receipt) = apply_atomic_restification_batch_authorized(state.document(),
        vec![(target.bytes.start, target.bytes.end, replacement)]);
    let post = post.unwrap();
    let mut owners = OwnerReceipts::default(); owners.push_exact(receipt);
    let restified = owners.continue_owned_rhythm(&state, &post).unwrap();
    let mut normalized = crate::arrangement::normalize_visible_rhythm_musicxml(&post);
    let mut tail = OwnerReceipts::default();
    tail.push_visible_block_with_births(normalized.visible_rhythm_block_identity.take(),
        normalized.stats.physical_births.pop(), &post).unwrap();
    owners.append(tail);
    let split = owners.continue_owned_rhythm(&state, &normalized.xml).unwrap();
    split.received_notation_obligations(split.document()).unwrap();
    for current in [&restified, &split] {
        for row in current.occurrences.iter().filter(|o| o.origin.source == Some(701)) {
            let facts = current.received_source_origins(row.creation).unwrap();
            assert_eq!(facts.leaves.len(), 1);
            assert_eq!(facts.leaves[0].kind, SourceOriginKind::SeedPitched);
            assert_eq!(facts.leaves[0].creation, target.origin.root);
            assert!(facts.steps.iter().any(|step| step.birth == Some(PhysicalBirthKind::Restified)));
            assert!(facts.leaves.iter().all(|leaf| leaf.kind != SourceOriginKind::SeedCoverageRest));
        }
    }
    assert!(split.occurrences.iter().filter(|o| o.origin.source == Some(701)).count() > 1);
}

#[test]
fn original_consolidation_keeps_mixed_seed_parents_when_its_current_source_is_none() {
    let state = received(1, 0, 701);
    let next = consolidated(&state, 1, "997");
    let created = next.occurrences.iter().find(|o|
        o.origin.birth == Some(PhysicalBirthKind::RestConsolidated)).unwrap();
    assert!(created.origin.source.is_none());
    let facts = next.received_source_origins(created.creation).unwrap();
    assert!(facts.leaves.iter().any(|leaf| leaf.kind == SourceOriginKind::SeedPitched));
    assert!(facts.leaves.iter().any(|leaf| leaf.kind == SourceOriginKind::SeedCoverageRest));
    let expected = state.occurrences.iter().filter(|o| o.staff == 1)
        .map(|o| o.origin.root).collect::<BTreeSet<_>>();
    assert_eq!(facts.leaves.iter().map(|leaf| leaf.creation).collect::<BTreeSet<_>>(), expected);
    next.received_notation_obligations(next.document()).unwrap();
}

#[test]
fn nested_original_merges_keep_all_leaf_origins_without_duplicates() {
    let state = received(1, 0, 701);
    let first = consolidated(&state, 1, "997");
    let second = consolidated(&first, 1, "998");
    let row = second.occurrences.iter().find(|o| o.staff == 1).unwrap();
    let facts = second.received_source_origins(row.creation).unwrap();
    assert_eq!(facts.steps.iter().filter(|step|
        step.birth == Some(PhysicalBirthKind::RestConsolidated)).count(), 2);
    assert_eq!(facts.leaves.len(), state.occurrences.iter().filter(|o| o.staff == 1).count());
    let mut missing = second.clone();
    missing.history.remove(&facts.leaves[0].creation);
    assert!(missing.received_notation_obligations(missing.document()).is_err());
    assert!(missing.received_source_origins(row.creation).is_err());
}

#[test]
fn final_chord_rest_cursors_read_the_same_received_sources_through_the_actual_emissions() {
    for copies in [1, 2] {
        for partial in [false, true] {
            let state = OwnedRhythmicContinuation::original_forward_obligations_for_test(partial, copies);
            let bound = state.final_chord_notation_obligations(state.document()).unwrap();
            let mut evidence = String::new(); bound.append_evidence(&mut evidence);
            assert!(evidence.contains("source_origins=leaves=["));
            assert!(evidence.contains("SeedCoverageRest"));
            assert!(evidence.contains("SeedPitched"));
            assert!(evidence.contains("silence_visibility_authority=none"));
            let foreign = format!("{} ", state.document());
            assert!(state.final_chord_notation_obligations(&foreign).is_err());
        }
    }
}
