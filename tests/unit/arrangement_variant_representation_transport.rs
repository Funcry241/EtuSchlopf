use super::*;
use crate::midi_seed::rhythmic_course_creation_tests::arrangement_pitched_fixture;

fn actual_hint_target() -> crate::arrangement::HumanPotentialStructuralTarget {
    let rows = [60, 72, 64, 76, 67, 79, 65, 77].into_iter().enumerate()
        .map(|(i, pitch)| (i + 1, i as i64 * 48, 1, pitch)).collect::<Vec<_>>();
    let (xml, seed) = arrangement_pitched_fixture(&rows, 1);
    let result = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        crate::arrangement::ArrangementProfile {
            rhythmic_courses: Some(&seed), preserve_reference: true,
            human_potential_enabled: true, ..Default::default()
        });
    *result.stats.human_potential_structural_targets.first()
        .expect("synthetic source must reach the actual target owner")
}

#[test]
fn local_hint_receiver_continues_actual_clef_insertions_without_new_origins() {
    let target = actual_hint_target();
    for (offset, duplicate) in [(0, false), (384, false), (0, true)] {
        let mut rows = vec![(101, offset, 2, 69), (102, offset + 48, 2, 60),
            (103, offset + 96, 2, 71)];
        if duplicate {
            rows.extend([(201, 384, 2, 69), (202, 432, 2, 60), (203, 480, 2, 71)]);
        }
        let parent = adopted(&rows, 3);
        let before = parent.master_state.document().to_owned();
        let origins = parent.arrangement_stats.rhythmic_courses.as_ref().unwrap();
        let result = evaluate_human_potential_local_hint_repair("RenamedSyntheticHint",
            &parent, target, &CheckOptions { strict: true }).unwrap();
        assert_ne!(result.master_state.document(), before, "actual clef owner must write");
        let continued = result.arrangement_stats.rhythmic_courses.as_ref().unwrap();
        assert!(continued.same_origins_as(origins));
        continued.validate_binding(result.master_state.document()).unwrap();
        assert_eq!(parent.master_state.document(), before);
        origins.validate_binding(&before).unwrap();
        assert!(result.master_state.lineage().lineage_continuity_holds(result.master_state.document()));
        let repeated = evaluate_human_potential_local_hint_repair("AnotherName",
            &result, target, &CheckOptions { strict: true }).unwrap();
        assert_eq!(repeated.master_state.document(), result.master_state.document());
        repeated.arrangement_stats.rhythmic_courses.as_ref().unwrap()
            .validate_binding(repeated.master_state.document()).unwrap();
    }
}

#[test]
fn local_hint_receiver_keeps_empty_custody_and_never_infers_unbound_origins() {
    let target = actual_hint_target();
    let empty = adopted(&[], 2);
    let result = evaluate_human_potential_local_hint_repair("EmptyHint", &empty,
        target, &CheckOptions { strict: true }).unwrap();
    assert_eq!(result.master_state.document(), empty.master_state.document());
    let courses = result.arrangement_stats.rhythmic_courses.as_ref().unwrap();
    assert!(courses.same_origins_as(empty.arrangement_stats.rhythmic_courses.as_ref().unwrap()));
    courses.validate_binding(result.master_state.document()).unwrap();
    let mut unbound = adopted(&[(1, 0, 2, 69)], 1);
    unbound.arrangement_stats.rhythmic_courses = None;
    let result = evaluate_human_potential_local_hint_repair("UnboundHint", &unbound,
        target, &CheckOptions { strict: true }).unwrap();
    assert!(result.arrangement_stats.rhythmic_courses.is_none());
    assert_ne!(result.master_state.document(), unbound.master_state.document());
}

#[test]
fn local_hint_receiver_refuses_stale_custody_before_owner_adoption() {
    let target = actual_hint_target();
    let mut parent = adopted(&[(1, 0, 2, 69)], 2);
    let foreign = adopted(&[(1, 0, 2, 68)], 2);
    parent.arrangement_stats.rhythmic_courses = foreign.arrangement_stats.rhythmic_courses;
    let before = parent.master_state.document().to_owned();
    assert!(evaluate_human_potential_local_hint_repair("StaleHint", &parent,
        target, &CheckOptions { strict: true }).is_err());
    assert_eq!(parent.master_state.document(), before);
}

#[test]
fn actual_clef_journal_requires_original_receipts_and_exact_states() {
    use crate::arrangement::representation_transport::{RepresentationStep, RhythmicRepresentationTransport};
    let parent = adopted(&[(1, 0, 2, 69), (2, 48, 2, 60)], 2);
    let before = parent.master_state.document();
    let mut after = before.to_owned();
    let owner = apply_lower_staff_clef_readability_policy(&mut after);
    assert!(!owner.block_receipts.is_empty());
    let transport = owner.rhythmic_transport.unwrap();
    let courses = parent.arrangement_stats.rhythmic_courses.as_ref().unwrap();
    courses.continue_representation(&after, &transport).unwrap();
    assert!(courses.continue_representation(&format!("{after} "), &transport).is_err());
    assert!(RhythmicRepresentationTransport::clef_readability(
        vec![RepresentationStep::recorded(before, &after, Vec::new())],
        &owner.block_receipts).is_err());
    assert!(RhythmicRepresentationTransport::clef_readability(
        vec![RepresentationStep::recorded(before, before, Vec::new())],
        &owner.block_receipts).is_err());
}

#[test]
fn dropped_actual_clef_receipt_remains_fatal_despite_valid_course_transport() {
    let parent = adopted(&[(1, 0, 2, 69)], 2);
    let before = parent.master_state.document().to_owned();
    let mut after = before.clone();
    let owner = apply_lower_staff_clef_readability_policy(&mut after);
    assert!(!owner.block_receipts.is_empty());
    parent.arrangement_stats.rhythmic_courses.as_ref().unwrap()
        .continue_representation(&after, &owner.rhythmic_transport.unwrap()).unwrap();
    let mut state = parent.master_state.clone();
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.adopt_materialized_document(after,
            crate::master_state_identity::OwnerReceipts::default(), "SyntheticDroppedClefReceipt");
    }));
    assert!(refusal.is_err());
    assert_eq!(state.document(), before);
    assert!(state.lineage().lineage_continuity_holds(&before));
}

#[test]
fn dropped_actual_polish_receipt_is_observable_and_still_fatal_at_master_adoption() {
    for _ in 0..2 {
        let arrangement = arranged(&[(1, 0, 2, 84), (2, 48, 2, 86), (3, 96, 2, 88)], 1);
        let polish = polish_musicxml(&arrangement.xml).unwrap();
        assert_ne!(arrangement.xml, polish.xml);
        assert!(polish.preserving_receipt.is_some());
        let genesis = arrangement.arrangement_source_genesis.unwrap();
        let mut state = structural_write_authority::MasterAdoptedState::adopt_genesis_state(
            arrangement.xml, crate::master_state_identity::MasterLineage::lineage_from_genesis(genesis));
        // An actual owner minted the receipt. This negative handoff deliberately
        // omits it; neither the observation nor transport may replace it.
        let missing = crate::master_state_identity::OwnerReceipts::default();
        let before = state.document().to_owned();
        let evidence = missing.coverage_diagnostic(&before, &polish.xml).unwrap();
        assert!(evidence.contains("receipt_count=0"));
        let stage = crate::arrangement::representation_transport::preserving_stage_evidence::<NotationPolishPreservingMint>(
            &before, &polish.xml, false);
        assert!(stage.contains("changed=true; receipt_present=false"));
        assert!(stage.contains("projection_equal=true"));
        let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            state.adopt_materialized_document(polish.xml, missing, "SyntheticDroppedPolishReceipt");
        }));
        assert!(refusal.is_err());
        assert_eq!(state.document(), before);
        assert!(state.lineage().lineage_continuity_holds(state.document()));
    }
}

#[test]
fn actual_empty_polish_noop_needs_no_receipt_and_has_no_coverage_diagnostic() {
    let arrangement = arranged(&[], 2);
    let polish = polish_musicxml(&arrangement.xml).unwrap();
    assert_eq!(arrangement.xml, polish.xml);
    assert!(polish.preserving_receipt.is_none());
    let receipts = crate::master_state_identity::OwnerReceipts::default();
    assert!(receipts.coverage_diagnostic(&arrangement.xml, &polish.xml).is_none());
    let stage = crate::arrangement::representation_transport::preserving_stage_evidence::<NotationPolishPreservingMint>(
        &arrangement.xml, &polish.xml, false);
    assert!(stage.contains("changed=false; receipt_present=false"));
    let genesis = arrangement.arrangement_source_genesis.unwrap();
    let mut state = structural_write_authority::MasterAdoptedState::adopt_genesis_state(
        arrangement.xml, crate::master_state_identity::MasterLineage::lineage_from_genesis(genesis));
    state.adopt_materialized_document(polish.xml, receipts, "SyntheticEmptyPolishNoop");
    assert!(state.lineage().lineage_continuity_holds(state.document()));
}

fn arranged(rows: &[(usize, i64, u8, i32)], measures: usize) -> ArrangementResult {
    let (xml, seed) = arrangement_pitched_fixture(rows, measures);
    crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        crate::arrangement::ArrangementProfile {
            rhythmic_courses: Some(&seed), preserve_reference: true,
            ..Default::default()
        })
}

fn adopted(rows: &[(usize, i64, u8, i32)], measures: usize) -> ArrangementVariant {
    let arrangement = arranged(rows, measures);
    let reference = ReferenceReport::default();
    let score = parse_musicxml(&arrangement.xml).unwrap();
    let signature = build_reference_signature(&reference, &score);
    evaluate_arrangement_variant("SyntheticTransport", arrangement,
        &reference, &signature, &CheckOptions { strict: true }, Some(2)).unwrap()
}

#[test]
fn actual_receiver_adopts_hidden_split_members_and_rest_hosts_with_current_custody() {
    let result = adopted(&[(501, 24, 1, 60), (502, 24, 1, 64), (503, 24, 1, 67)], 2);
    result.arrangement_stats.rhythmic_courses.as_ref().unwrap()
        .validate_binding(result.master_state.document()).unwrap();
    assert!(result.master_state.document().contains("<tie"));
}

#[test]
fn empty_courses_reach_the_actual_adopted_candidate() {
    let result = adopted(&[], 3);
    result.arrangement_stats.rhythmic_courses.as_ref().unwrap()
        .validate_binding(result.master_state.document()).unwrap();
}

#[test]
fn independent_duplicate_cohorts_are_all_carried() {
    for rows in [vec![(11, 0, 1, 60), (12, 0, 1, 64)],
        vec![(11, 0, 1, 60), (12, 0, 1, 64), (91, 384, 2, 48), (92, 384, 2, 55)]] {
        let result = adopted(&rows, 2);
        result.arrangement_stats.rhythmic_courses.as_ref().unwrap()
            .validate_binding(result.master_state.document()).unwrap();
    }
}

#[test]
fn sealed_polish_transport_refuses_a_different_output_and_foreign_origin_state() {
    let arrangement = arranged(&[(1, 0, 1, 60), (2, 48, 1, 62)], 1);
    let polish = polish_musicxml(&arrangement.xml).unwrap();
    let courses = arrangement.stats.rhythmic_courses.as_ref().unwrap();
    assert!(courses.continue_representation(&polish.xml.replace("<duration>48</duration>",
        "<duration>24</duration>"), &polish.rhythmic_transport).is_err());
    let foreign = arranged(&[(1, 0, 1, 61), (2, 48, 1, 63)], 1);
    assert!(foreign.stats.rhythmic_courses.as_ref().unwrap()
        .continue_representation(&polish.xml, &polish.rhythmic_transport).is_err());
}

#[test]
fn actual_layout_print_removal_carries_exact_occurrence_locations() {
    let arrangement = arranged(&[(1, 0, 1, 60)], 2);
    let xml = arrangement.xml.replacen("<attributes>", "\n      <print new-page=\"yes\"/>\n<attributes>", 1);
    let layout = apply_pianist_layout(&xml, 2);
    // This independent owner probe exercises actual deletion capture. It does
    // not rebind arrangement origins to a manually changed document.
    let note_start = xml.find("<note").unwrap();
    let note_end = note_start + xml[note_start..].find("</note>").unwrap() + 7;
    let ranges = layout.rhythmic_transport.carry(&xml, &layout.xml, std::iter::once(note_start..note_end).collect()).unwrap();
    assert_eq!(&xml[note_start..note_end], &layout.xml[ranges[0].clone()]);
    assert!(!layout.xml.contains("<print new-page=\"yes\"/>"));
}

#[test]
fn repeated_polish_and_layout_keep_the_bound_state() {
    let result = adopted(&[(1, 0, 2, 84), (2, 48, 2, 86), (3, 96, 2, 88)], 2);
    let xml = result.master_state.document();
    let polish = polish_musicxml(xml).unwrap();
    let next = result.arrangement_stats.rhythmic_courses.as_ref().unwrap()
        .continue_representation(&polish.xml, &polish.rhythmic_transport).unwrap();
    assert!(next.same_origins_as(result.arrangement_stats.rhythmic_courses.as_ref().unwrap()));
    let layout = apply_pianist_layout(&polish.xml, 2);
    let final_courses = next.continue_representation(&layout.xml, &layout.rhythmic_transport).unwrap();
    assert!(final_courses.same_origins_as(&next));
    final_courses.validate_binding(&layout.xml).unwrap();
}

#[test]
fn actual_beam_and_clef_writes_preserve_creation_history() {
    let arrangement = arranged(&[(1, 0, 2, 84), (2, 48, 2, 86), (3, 96, 2, 88),
        (4, 144, 2, 89), (5, 192, 2, 88), (6, 240, 2, 86), (7, 288, 2, 84)], 1);
    let polish = polish_musicxml(&arrangement.xml).unwrap();
    assert!(polish.stats.beamed_notes > 0, "fixture must reach the actual beam writer");
    assert!(polish.xml.contains("etuschlopf-ledger-bound-clef-readback"),
        "fixture must reach the actual clef insertion writer");
    let before = arrangement.stats.rhythmic_courses.as_ref().unwrap();
    let after = before.continue_representation(&polish.xml, &polish.rhythmic_transport).unwrap();
    assert!(after.same_origins_as(before));
    after.validate_binding(&polish.xml).unwrap();
}

#[test]
fn selected_master_receiver_carries_actual_clef_chain_and_keeps_original_adoption() {
    use crate::arrangement::OwnedRhythmicContinuation;
    for ids in [[1, 2, 3], [91, 72, 54]] {
        let parent = adopted(&[(ids[0], 0, 2, 69), (ids[1], 48, 2, 48),
            (ids[2], 384, 2, 71)], 3);
        let before = parent.master_state.document();
        let initial = OwnedRhythmicContinuation::receive(
            parent.arrangement_stats.rhythmic_courses.as_ref().unwrap(), before).unwrap();
        let mut after = before.to_owned();
        let owner = apply_lower_staff_clef_readability_policy(&mut after);
        assert!(!owner.block_receipts.is_empty(), "fixture must reach the actual clef owner");
        let mut receipts = crate::master_state_identity::OwnerReceipts::default();
        receipts.push_blocks(owner.block_receipts);
        let next = receipts.continue_owned_rhythm(&initial, &after).unwrap();
        assert!(next.same_identities_as(&initial));
        next.require_complete_course_context(&after).unwrap();
        let mut master = parent.master_state.clone();
        master.observe_selected_custody("RenamedSyntheticClefChain", Some(std::sync::Arc::new(initial.clone())), None);
        master.adopt_materialized_document(after.clone(), receipts, "RenamedSyntheticClefChain");
        assert_eq!(master.document(), after);
        assert!(master.lineage().lineage_continuity_holds(&after));
    }
}

#[test]
fn failed_clef_capture_stays_unknown_without_replacing_original_adoption() {
    use crate::arrangement::OwnedRhythmicContinuation;
    let parent = adopted(&[(1, 0, 2, 69)], 2);
    let before = parent.master_state.document();
    let initial = OwnedRhythmicContinuation::receive(
        parent.arrangement_stats.rhythmic_courses.as_ref().unwrap(), before).unwrap();
    let mut after = before.to_owned();
    let owner = apply_lower_staff_clef_readability_policy(&mut after);
    assert!(!owner.block_receipts.is_empty());
    let mut receipts = crate::master_state_identity::OwnerReceipts::default();
    receipts.push_blocks(owner.block_receipts.without_capture_for_test());
    assert!(receipts.continue_owned_rhythm(&initial, &after).is_err());
    let mut master = parent.master_state.clone();
    master.observe_selected_custody("UnknownClefObservation", Some(std::sync::Arc::new(initial)), None);
    master.adopt_materialized_document(after.clone(), receipts, "UnknownClefObservation");
    assert!(master.lineage().lineage_continuity_holds(&after));
}
