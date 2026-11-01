use super::*;

fn bound_candidate() -> SolverCandidate {
    let mut candidate = physical_rhythmic_birth_handoff::base_candidate();
    crate::source_selection::bind_productive_post_legacy_selection_disposition_plan(
        &mut candidate.midi_seed_stats.source_selection_full_causal,
        &candidate.arrangement_stats.material_disposition_rows).unwrap();
    bind_candidate_material_lineage_from_post_legacy(&mut candidate).unwrap();
    assert!(candidate.material_lineage.authority_bound);
    candidate
}

#[test]
fn actual_editorial_full_rest_cannot_erase_a_bound_source() {
    let base = bound_candidate();
    assert!(editorial_voice_preserves_bound_source_support(&base, &base));
    let first = parse_atomic_notes(base.layout_xml()).into_iter().find(|n| n.pitch.is_some()).unwrap();
    let target = EditorialVoiceDecisionTarget {
        measure: first.measure, staff: first.staff, voice: first.voice,
        target_voice: None, grade: "schwach".into(), note_count: 1,
        rest_count: 0, note_ranges: Vec::new(), rest_ranges: Vec::new(),
    };
    let patched = apply_editorial_voice_complete_rewrite_variant(base.layout_xml(), &target);
    let (_, request) = build_editorial_voice_trial_candidate(0, "SourceSupportTest", patched,
        &base, &CheckOptions { strict: true }).unwrap();
    let mut proposed = base.clone();
    structural_write_authority::apply_block_master_xml_request(&mut proposed.master_state, request).unwrap();
    assert!(!editorial_voice_preserves_bound_source_support(&base, &proposed));
    assert!(editorial_voice_preserves_bound_source_support(&base, &base));
}

#[test]
fn source_support_keeps_multiplicity_and_each_fragment_not_only_attack_extents() {
    let base = bound_candidate();
    let before = editorial_voice_bound_source_support(&base.material_lineage).unwrap();
    assert!(!before.is_empty());
    let mut lost = base.material_lineage.clone();
    let mut fragments = lost.fragments.to_vec();
    let index = fragments.iter().position(|f| f.pitch.is_some()).unwrap();
    fragments.remove(index); lost.fragments = std::sync::Arc::from(fragments);
    assert_ne!(editorial_voice_bound_source_support(&lost).unwrap(), before);
    let mut duplicated = base.material_lineage.clone();
    let mut fragments = duplicated.fragments.to_vec();
    let pitched = fragments.iter().find(|f| f.pitch.is_some()).unwrap().clone();
    fragments.push(pitched); duplicated.fragments = std::sync::Arc::from(fragments);
    assert_ne!(editorial_voice_bound_source_support(&duplicated).unwrap(), before);
}

#[test]
fn support_ignores_serialized_positions_and_labels_but_not_missing_ancestry() {
    let base = bound_candidate();
    let before = editorial_voice_bound_source_support(&base.material_lineage).unwrap();
    let mut relabeled = base.material_lineage.clone();
    let mut fragments = relabeled.fragments.to_vec();
    for f in &mut fragments { f.raw_note_index += 100; f.staff = "renamed".into(); f.voice = "other".into(); }
    relabeled.fragments = std::sync::Arc::from(fragments);
    assert_eq!(editorial_voice_bound_source_support(&relabeled).unwrap(), before);
    let mut unbound = base.clone(); unbound.material_lineage.authority_bound = false;
    assert!(!editorial_voice_preserves_bound_source_support(&unbound, &base));
    let mut fragments = relabeled.fragments.to_vec();
    fragments.iter_mut().find(|f| f.pitch.is_some()).unwrap().source_event_id = None;
    relabeled.fragments = std::sync::Arc::from(fragments);
    assert!(editorial_voice_bound_source_support(&relabeled).is_none());
}

#[test]
fn joint_reader_uses_adopted_course_identity_after_voice_owner_without_rewriting_fragment_snapshot() {
    let mut candidate = bound_candidate();
    assert!(JointReadingOwnerInput::from_carried_material(&candidate).is_err());
    candidate.master_state.observe_selected_custody("SyntheticJointSource",
        candidate.arrangement_stats.owned_rhythm.clone(), candidate.arrangement_stats.owned_rhythm_unknown.clone());
    let before = JointReadingOwnerInput::from_carried_material(&candidate).unwrap();
    let old_fragments = candidate.material_lineage.fragments.clone();
    let mut edits = Vec::new();
    for note in parse_atomic_notes(candidate.layout_xml()) {
        let text = &candidate.layout_xml()[note.note_start..note.note_end];
        if let Some(start) = text.find("<voice>") {
            let start = start + "<voice>".len();
            let end = start + text[start..].find("</voice>").unwrap();
            edits.push((note.note_start + start, note.note_start + end, "803".into()));
        }
    }
    let (post, receipt) = structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(candidate.layout_xml(), edits);
    let post = post.unwrap();
    let mut receipts = master_state_identity::OwnerReceipts::default(); receipts.push_exact(receipt);
    candidate.master_state.adopt_materialized_document(post, receipts, "SyntheticJointSourceVoiceOwner");
    assert_eq!(candidate.material_lineage.fragments.as_ref(), old_fragments.as_ref());
    let after = JointReadingOwnerInput::from_carried_material(&candidate).unwrap();
    assert_eq!(before.heads.len(), after.heads.len());
    for (old, new) in before.heads.iter().zip(&after.heads) {
        assert_eq!(old.source_event_id, new.source_event_id);
        assert_eq!(old.attack_id, new.attack_id);
    }
    assert!(after.binds(candidate.layout_xml()));
    let mut missing = candidate.clone();
    let mut attacks = missing.material_lineage.attacks.to_vec();
    for attack in &mut attacks { attack.exists = false; }
    missing.material_lineage.attacks = std::sync::Arc::from(attacks);
    assert!(JointReadingOwnerInput::from_carried_material(&missing).is_err());
}
