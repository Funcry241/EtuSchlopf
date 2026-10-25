use super::*;
use crate::audit_only::renderer_ottava_context::{observe, synthetic_pdf};
use crate::master_state_identity::{MasterLineage, RenderMasterEpoch, RenderTransitionReceipts};
use crate::registernotation::{RegisternotationsDarstellung as Display, reader_policy as policy};
use crate::structural_write_authority::{
    self as authority, MasterAdoptedState, RenderSinkKind, RenderSovereignState,
};

#[test]
fn ottava_reader_real_owner_receipt_adoption_guard_and_final_pdf_form_one_chain() {
    let _guard = crate::registernotation::sperre_fuer_tests();
    for display in [
        Display::Achtva,
        Display::Achtvb,
        Display::Fuenfzehnma,
        Display::Fuenfzehnmb,
    ] {
        for staff in [1u8, 2] {
            policy::reset();
            crate::master_state_identity::reset_render_lineage_measurement();
            let source =
                policy::synthetic::course(display, &staff.to_string(), "renamed", "free-label")
                    .replace(
                        "<staves>2</staves>",
                        "<time><beats>2</beats><beat-type>4</beat-type></time><staves>2</staves>",
                    );
            let dirs = policy::synthetic::directives();
            let request = policy::pdf_requests(&source).pop().unwrap();
            let baseline_pdf = synthetic_pdf(display, staff, true, true, true, "Leland", 0.0);
            let context = observe(&source, &baseline_pdf);
            assert_eq!(context.supported.len(), 1);
            policy::observe_probe(&source, &source, &dirs, &context);
            policy::plan_final_document(&source, &dirs).unwrap();
            let mut master = MasterAdoptedState::adopt_genesis_state(
                source.clone(),
                MasterLineage::lineage_from_genesis(
                    crate::master_state_identity::tests::synthetic_genesis_for_tests(&source),
                ),
            );
            let path = std::env::temp_dir().join(format!(
                "ottava-chain-{}-{staff}-{}.musicxml",
                display.bezeichnung(),
                std::process::id()
            ));
            std::fs::write(&path, &source).unwrap();
            let (result, receipt) =
                authority::materialize_final_render_notation_with_phrase_projection_authorized(
                    &path,
                    &dirs,
                    &[],
                );
            result.unwrap();
            let post = std::fs::read_to_string(&path).unwrap();
            assert!(!post.contains("<words"));
            assert!(!post.contains("ets-ottava-"));
            assert!(validate_final_layout_aware_octave_shift_direction_semantics(&post).is_err());
            assert!(!policy::omission_is_adopted(&post, request.ordinal));
            master.adopt_path_bound_preserving_transition(receipt, "Synthetic/OttavaReader/E007");
            assert_eq!(master.document(), post);
            policy::observe_master_adoption(&master);
            assert!(policy::final_pdf_observation_required(&post));
            validate_final_layout_aware_octave_shift_direction_semantics(&post).unwrap();
            assert!(
                validate_octave_shift_direction_semantics(&post).is_err(),
                "early guard must remain strict"
            );
            assert!(
                validate_final_layout_aware_octave_shift_direction_semantics(
                    &(post.clone() + "\n")
                )
                .is_err(),
                "foreign post must not inherit permission"
            );
            let final_pdf = synthetic_pdf(display, staff, false, true, true, "Leland", 0.0);
            policy::validate_final_pdf(&post, &observe(&post, &final_pdf)).unwrap();
            let mut wrong_glyph = final_pdf.clone();
            let (old, new): (&[u8], &[u8]) = if display.groesse() == 8 {
                (b"<E510>", b"<E514>")
            } else {
                (b"<E514>", b"<E510>")
            };
            let at = wrong_glyph
                .windows(old.len())
                .position(|bytes| bytes == old)
                .unwrap();
            wrong_glyph[at..at + old.len()].copy_from_slice(new);
            assert!(
                policy::validate_final_pdf(&post, &observe(&post, &wrong_glyph)).is_err(),
                "a different orientation glyph with identical bounds cannot satisfy the final observation"
            );
            for (line, orientation, shift) in [(false, true, 0.0), (true, false, 0.0)] {
                let changed =
                    synthetic_pdf(display, staff, false, line, orientation, "Leland", shift);
                assert!(policy::validate_final_pdf(&post, &observe(&post, &changed)).is_err());
            }
            // Engraving can translate the whole system without changing its
            // carried horizontal attachment or current readable orientation.
            for shift in [-4.0, 1.0, 17.0] {
                let moved = synthetic_pdf(display, staff, false, true, true, "Leland", shift);
                policy::validate_final_pdf(&post, &observe(&post, &moved)).unwrap();
            }
            assert!(
                policy::validate_final_pdf(&post, &context).is_err(),
                "old source observation is stale"
            );

            // Exercise the independent sovereign Render adoption with the real E007 owner.
            let origin_master = MasterAdoptedState::adopt_genesis_state(
                source.clone(),
                MasterLineage::lineage_from_genesis(
                    crate::master_state_identity::tests::synthetic_genesis_for_tests(&source),
                ),
            );
            let render_path = path.with_extension("render.musicxml");
            let mut render =
                RenderSovereignState::begin_project_render(&render_path, &origin_master);
            let root = render.initial_trial_root(true);
            render
                .adopt_transition(
                    root,
                    &source,
                    RenderTransitionReceipts::new(),
                    RenderSinkKind::PrunedPdfTrial,
                    |file, _, candidate| (std::fs::write(file, candidate), None),
                )
                .unwrap();
            render.bind_late_master_epoch(RenderMasterEpoch::after_cursor_lane_before_final_chord(
                origin_master.lineage(),
                &source,
                &crate::cursor_lane_temporal_check::check_cursor_lane_temporal_precondition(
                    "MasterXML",
                    &source,
                )
                .unwrap(),
                &source,
            ));
            authority::adopt_final_layout_phrase_projection(&mut render, &dirs, &[]).unwrap();
            policy::observe_render_adoption(&render);
            assert_eq!(render.document(), Some(post.as_str()));
            assert_eq!(std::fs::read_to_string(&render_path).unwrap(), post);
            policy::plan_final_document(&post, &dirs).unwrap();
            assert!(
                policy::final_removals(&post, &dirs).is_empty(),
                "reentry cannot remove more words"
            );
            let (result, _) =
                authority::materialize_final_render_notation_with_phrase_projection_authorized(
                    &render_path,
                    &dirs,
                    &[],
                );
            result.unwrap();
            assert_eq!(std::fs::read_to_string(&render_path).unwrap(), post);
            std::fs::remove_file(path).unwrap();
            std::fs::remove_file(render_path).unwrap();
            policy::reset();
            assert!(
                validate_final_layout_aware_octave_shift_direction_semantics(&post).is_err(),
                "reset revokes session evidence"
            );
        }
    }
}

#[test]
fn ottava_reader_missing_probe_uses_real_owner_and_preserves_the_baseline_word() {
    let _guard = crate::registernotation::sperre_fuer_tests();
    policy::reset();
    let source = policy::synthetic::course(Display::Achtva, "1", "independent", "arbitrary");
    let dirs = policy::synthetic::directives();
    policy::plan_final_document(&source, &dirs).unwrap();
    let path = std::env::temp_dir().join(format!("ottava-unknown-{}.musicxml", std::process::id()));
    std::fs::write(&path, &source).unwrap();
    let (result, _) =
        authority::materialize_final_render_notation_with_phrase_projection_authorized(
            &path,
            &dirs,
            &[],
        );
    result.unwrap();
    let post = std::fs::read_to_string(&path).unwrap();
    assert!(post.contains("<words font-style=\"italic\">8va</words>"));
    assert!(!post.contains("ets-ottava-"));
    validate_octave_shift_direction_semantics(&post).unwrap();
    assert!(!policy::final_pdf_observation_required(&post));
    std::fs::remove_file(path).unwrap();
}
