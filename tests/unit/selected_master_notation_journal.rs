use super::*;
fn direction(einsatz: i64, art: RegisternotationsGrenzart,
    darstellung: crate::registernotation::RegisternotationsDarstellung) -> RegisternotationsRichtung {
    RegisternotationsRichtung { teil: "P1".into(), takt_ordinal: 1, system: "1".into(),
        einsatz, art, darstellung, linie: 1, origin: None }
}
    #[test]
    fn actual_octave_emission_carries_duplicate_notes_and_original_receipts_in_order() {
        use crate::arrangement::OwnedRhythmicContinuation;
        for scale in [1, 3] {
            let rows = [(71, 0, 1), (72, 0, 1), (91, 384 * scale, 1)];
            let (before, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture(&rows, 4);
            let arranged = crate::arrangement::arrange_piano_musicxml_with_profile(&before,
                crate::arrangement::ArrangementProfile { rhythmic_courses: Some(&seed),
                    preserve_reference: true, ..Default::default() });
            let before = arranged.xml;
            let state = OwnedRhythmicContinuation::receive(arranged.stats.rhythmic_courses.as_ref().unwrap(), &before).unwrap();
            let mut after = before.clone();
            let mut receipts = master_state_identity::OwnerReceipts::default();
            let doc = roxmltree::Document::parse(&before).unwrap();
            let spans = doc.descendants().filter(|n| n.has_tag_name("measure"))
                .map(|n| n.range()).collect::<Vec<_>>();
            for span in spans.into_iter().rev() {
                let original = &after[span.clone()];
                let duration = registernotations_tonanker(original).unwrap().iter()
                    .filter(|a| a.system == "1" && a.sichtbar).map(|a| a.beginn).min();
                let Some(onset) = duration else { continue; };
                let mut first = direction(onset, RegisternotationsGrenzart::Start,
                    crate::registernotation::RegisternotationsDarstellung::Achtva);
                let mut second = first.clone(); first.linie = 1; second.linie = 2;
                let (replacement, journal) = registernotationsrichtungen_in_takt(original, &[first, second]).unwrap();
                let pre = after.clone();
                let (post, receipt) = crate::structural_write_authority::replace_bound_octave_measure_authorized(
                    &pre, span.start, span.end, replacement);
                after = post.unwrap();
                let mut step = master_state_identity::OwnerReceipts::default();
                step.push_octave_block_with_transport(receipt, journal.enclosed(&pre, &after, span));
                receipts.append(step);
            }
            assert_ne!(after, before);
            let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
            let mut master = crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(
                before.clone(), master_state_identity::MasterLineage::lineage_from_genesis(arranged.arrangement_source_genesis.unwrap()));
            assert!(next.same_identities_as(&state));
            next.require_complete_course_context(&after).unwrap();
            assert!(receipts.continue_owned_rhythm(&next, &after).is_err());
            master.observe_selected_custody("RenamedOctaveJournal", Some(std::sync::Arc::new(state.clone())), None);
            master.adopt_materialized_document(after.clone(), receipts, "RenamedOctaveJournal");
            assert!(master.lineage().lineage_continuity_holds(&after));
            let mut missing = master_state_identity::OwnerReceipts::default();
            let span = roxmltree::Document::parse(&before).unwrap().descendants()
                .find(|n| n.has_tag_name("measure")).unwrap().range();
            let (replacement, _) = registernotationsrichtungen_in_takt(&before[span.clone()],
                &[direction(0, RegisternotationsGrenzart::Start,
                    crate::registernotation::RegisternotationsDarstellung::Achtva)]).unwrap();
            let (post, receipt) = crate::structural_write_authority::replace_bound_octave_measure_authorized(
                &before, span.start, span.end, replacement);
            missing.push_block(receipt);
            assert!(missing.continue_owned_rhythm(&state, &post.unwrap()).is_err());
        }
    }

