use super::*;

fn note(voice: &str, duration: i64) -> String {
    format!("<note><pitch><step>C</step><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>1</staff></note>")
}
fn document(notes: &str, label: &str) -> String {
    format!("<score-partwise><part id=\"{label}\"><measure number=\"{label}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{notes}</measure></part></score-partwise>")
}
fn exact_births(pre: &str, post: &str, receipt: Option<ExactTransformationReceipt>) -> Vec<crate::arrangement::physical_births::PhysicalRhythmicBirths> {
    let mut receipts = master_state_identity::OwnerReceipts::default(); receipts.push_exact(receipt);
    let births = receipts.physical_birth_readback(pre, post).unwrap();
    assert!(receipts.physical_birth_readback(&format!("{pre} "), post).is_err());
    assert!(receipts.physical_birth_readback(pre, &format!("{post} ")).is_err());
    births
}

#[test]
fn actual_restification_births_distinguish_equal_executions_and_survive_readback() {
    let n = note("9001", 96); let pre = document(&format!("{n}{n}"), "Örtlich");
    let start = pre.find(&n).unwrap(); let rest = restify_note_block(&n);
    let execute = || {
        let (post, receipt) = structural_write_authority::apply_atomic_restification_batch_authorized(
            &pre, vec![(start, start + n.len(), rest.clone())]);
        let post = post.unwrap(); exact_births(&pre, &post, receipt)
    };
    let first = execute(); let second = execute();
    assert_eq!(first[0].count(), 1); assert!(!first[0].same_creations_as(&second[0]));
    assert!(first[0].same_creations_as(&first[0].clone()));
}

#[test]
fn actual_transfer_captures_both_creations_for_both_insertion_orders() {
    for before in [false, true] {
        let a = note("901", 96); let b = note("202", 96);
        let pre = document(&if before { format!("{b}{a}") } else { format!("{a}{b}") }, "Transfer");
        let start = pre.find(&a).unwrap(); let anchor = pre.find(&b).unwrap() + b.len();
        let rest = restify_note_block(&a); let transferred = transfer_chord_note_block(&a, "2", "4001", Some(96));
        let (post, receipt) = structural_write_authority::apply_bound_safe_transfer_authorized(
            &pre, start, start + a.len(), anchor, rest, transferred);
        let post = post.unwrap(); let births = exact_births(&pre, &post, receipt);
        assert_eq!(births[0].count(), 2);
        assert_eq!(post.matches("<note").count(), 3);
    }
}

#[test]
fn actual_normalizer_captures_splits_and_accepted_gap_rests_and_hands_off_its_block() {
    for label in ["Renamed", "Örtlich"] {
        let pre = document(&note("7001", 144), label);
        let normalized = normalize_visible_rhythm_musicxml(&pre);
        assert!(normalized.stats.visible_rhythm_values_split > 0);
        assert!(normalized.stats.visible_rhythm_rests_inserted > 0);
        let capture = &normalized.stats.physical_births[0];
        assert_eq!(capture.count(), 2 + normalized.stats.visible_rhythm_rests_inserted);
        capture.validate_binding(&pre, &normalized.xml).unwrap();
        assert!(capture.validate_binding(&format!("{pre} "), &normalized.xml).is_err());
        let mut receipts = master_state_identity::OwnerReceipts::default();
        receipts.push_visible_block_with_births(normalized.visible_rhythm_block_identity,
            Some(capture.clone()), &pre).unwrap();
        let readback = receipts.physical_birth_readback(&pre, &normalized.xml).unwrap();
        assert!(capture.same_creations_as(&readback[0]));
    }
}

#[test]
fn rejected_rest_insertion_publishes_only_retained_actual_metric_births() {
    let pre = document(&format!("{}<backup><duration>144</duration></backup>{}", note("701", 144), note("902", 96)), "Rollback");
    let normalized = normalize_visible_rhythm_musicxml(&pre);
    assert_eq!(normalized.stats.visible_rhythm_rests_inserted, 0);
    assert!(normalized.stats.visible_rhythm_values_split > 0);
    let births = &normalized.stats.physical_births[0];
    assert_eq!(births.count(), 2);
    births.validate_binding(&pre, &normalized.xml).unwrap();
}

#[test]
fn compound_original_exact_and_block_handoff_keeps_original_creations_in_order() {
    let n = note("901", 144); let pre = document(&n, "Compound");
    let start = pre.find(&n).unwrap();
    let (restified, receipt) = structural_write_authority::apply_atomic_restification_batch_authorized(
        &pre, vec![(start, start + n.len(), "<note><rest/><duration>144</duration><voice>901</voice><staff>1</staff></note>".into())]);
    let restified = restified.unwrap(); let mut receipts = master_state_identity::OwnerReceipts::default();
    receipts.push_exact(receipt); let original = receipts.physical_birth_readback(&pre, &restified).unwrap();
    let normalized = normalize_visible_rhythm_musicxml(&restified);
    let mut tail = master_state_identity::OwnerReceipts::default();
    tail.push_visible_block_with_births(normalized.visible_rhythm_block_identity,
        normalized.stats.physical_births.into_iter().next(), &restified).unwrap();
    receipts.append(tail);
    let births = receipts.physical_birth_readback(&pre, &normalized.xml).unwrap();
    assert!(births[0].same_creations_as(&original[0]));
    assert!(births.len() >= 2);
}

fn base_candidate() -> SolverCandidate {
    static NEXT_INPUT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let input = NEXT_INPUT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("etus_birth_handoff_{}_{input}.mid", std::process::id()));
    let mut midi = b"MThd\0\0\0\x06\0\0\0\x01\0\x60MTrk\0\0\0\x0c".to_vec();
    midi.extend([0, 0x90, 60, 96, 96, 0x80, 60, 0, 0, 0xff, 0x2f, 0]);
    std::fs::write(&path, midi).unwrap();
    let profile = midi_seed_solver_profiles().remove(0);
    let (xml, mut seed) = midi_to_musicxml_seed_with_profile_and_hotspots(&path, "SyntheticBirth", profile.clone(), None).unwrap();
    std::fs::remove_file(path).unwrap();
    let midi_seed_genesis = seed.master_genesis.take().unwrap();
    let arrangement = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        crate::arrangement::ArrangementProfile { rhythmic_courses: Some(&seed.rhythmic_courses), preserve_reference: true, ..Default::default() });
    let reference = ReferenceReport::default(); let score = parse_musicxml(&arrangement.xml).unwrap();
    let signature = build_reference_signature(&reference, &score);
    let variant = evaluate_arrangement_variant("SyntheticBirth", arrangement, &reference, &signature,
        &CheckOptions { strict: true }, Some(2)).unwrap();
    SolverCandidate { iteration: 0, profile_name: "SyntheticBirth".into(), profile_contract: profile.clone(),
        profile_lineage: CandidateLineage::from_profile(&profile), final_result: variant.final_result,
        master_state: variant.master_state, midi_seed_stats: seed, midi_seed_genesis,
        reference_signature: signature, fidelity: variant.fidelity, arrangement_stats: variant.arrangement_stats,
        material_lineage: CandidateMaterialLineageSpine::default(), consensus_relocation_longitudinal_trace: Vec::new(),
        consensus_relocation_freeze_boundary_xml: None, polish_stats: variant.polish_stats,
        horizontal_layout_seed: variant.horizontal_layout_seed }
}

#[test]
fn actual_patch_and_candidate_receiver_carry_original_birth_and_keep_missing_receipt_fatal() {
    let base = base_candidate(); let n = parse_atomic_notes(base.layout_xml()).into_iter().find(|n| n.pitch.is_some()).unwrap();
    let patch = AtomicPatch { name: "SyntheticRestify".into(), note_start: n.note_start, note_end: n.note_end,
        target_staff: n.staff, target_voice: n.voice, staff_value_range: n.staff_value_range,
        voice_value_range: n.voice_value_range, action: AtomicPatchAction::RestifyHidden,
        insert_after: None, anchor_duration: None, material_transfer_carrier: None };
    let (xml, receipt) = apply_atomic_patch(base.layout_xml(), &patch);
    let mut receipts = master_state_identity::OwnerReceipts::default(); receipts.push_exact(receipt);
    let original = receipts.physical_birth_readback(base.layout_xml(), &xml).unwrap();
    assert_eq!(original[0].count(), 1);
    let result = build_atomic_candidate(1, "SyntheticRestify", AtomicPatchedDocument {
        xml: xml.clone(), lineage_receipts: receipts }, None, &[], &base, &CheckOptions { strict: true }).unwrap();
    assert!(result.arrangement_stats.physical_births[0].same_creations_as(&original[0]));
    assert_eq!(result.arrangement_stats.physical_birth_capture_unknown, 0);
    let owned = result.arrangement_stats.owned_rhythm.as_ref().unwrap();
    owned.require_complete_course_context(result.layout_xml()).unwrap();
    assert!(owned.born_from(base.arrangement_stats.owned_rhythm.as_ref().unwrap()) > 0);
    assert!(result.master_state.lineage().lineage_continuity_holds(result.layout_xml()));
    let (same_xml, mut original_receipt) = apply_atomic_patch(base.layout_xml(), &patch);
    original_receipt.as_mut().unwrap().take_physical_births().unwrap();
    let mut missing_capture = master_state_identity::OwnerReceipts::default(); missing_capture.push_exact(original_receipt);
    let unknown = build_atomic_candidate(3, "MissingPhysicalCapture", AtomicPatchedDocument {
        xml: same_xml, lineage_receipts: missing_capture }, None, &[], &base, &CheckOptions { strict: true }).unwrap();
    assert_eq!(unknown.arrangement_stats.physical_birth_capture_unknown, 1);
    assert!(unknown.arrangement_stats.owned_rhythm.is_none());
    assert!(unknown.arrangement_stats.owned_rhythm_unknown.as_deref().unwrap().contains("capture"));
    assert_eq!(unknown.layout_xml(), result.layout_xml());
    let first_failure = unknown.arrangement_stats.owned_rhythm_unknown.clone().unwrap();
    assert!(first_failure.contains("AtomicPatch/OriginalOwners: OwnerStep=1 PopulationRow="));
    let descendant = build_atomic_candidate(4, "LaterUnchangedOwner", AtomicPatchedDocument {
        xml: unknown.layout_xml().to_owned(), lineage_receipts: master_state_identity::OwnerReceipts::default(),
    }, None, &[], &unknown, &CheckOptions { strict: true }).unwrap();
    let control = build_atomic_candidate(4, "LaterUnchangedOwner", AtomicPatchedDocument {
        xml: result.layout_xml().to_owned(), lineage_receipts: master_state_identity::OwnerReceipts::default(),
    }, None, &[], &result, &CheckOptions { strict: true }).unwrap();
    assert_eq!(descendant.layout_xml(), control.layout_xml());
    assert_eq!(descendant.arrangement_stats.owned_rhythm_unknown.as_deref(), Some(first_failure.as_str()));
    let evidence = owned_rhythm_diagnostic_line("Synthetic\nRenamed", &descendant);
    assert_eq!(evidence.lines().count(), 1);
    assert!(evidence.contains("Unknown: AtomicPatch/OriginalOwners:"));
    assert!(evidence.contains("boundary=ArrangementSolverFinal"));
    assert!(evidence.contains("MusicMutation=none"));
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        build_atomic_candidate(2, "MissingOriginalReceipt", AtomicPatchedDocument {
            xml, lineage_receipts: master_state_identity::OwnerReceipts::default() }, None, &[], &base, &CheckOptions { strict: true })
    }));
    assert!(refusal.is_err());
}

#[test]
fn actual_candidate_diagnostic_checks_current_bytes_and_cannot_change_adoption() {
    let base = base_candidate();
    assert!(owned_rhythm_diagnostic_line("Positive", &base).contains("complete_received_course_context"));
    let original = base.layout_xml().to_owned();
    // A genuine EXACT voice-owner transition preserves the owned identities,
    // but the original pre-state custody must not certify its changed bytes.
    let n = parse_atomic_notes(base.layout_xml()).into_iter().find(|n| n.pitch.is_some()).unwrap();
    let range = n.voice_value_range.unwrap();
    let (post, receipt) = structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
        base.layout_xml(), vec![(range.0, range.1, "8001".into())]);
    let mut receipts = master_state_identity::OwnerReceipts::default(); receipts.push_exact(receipt);
    let current = build_atomic_candidate(5, "VoiceRename", AtomicPatchedDocument {
        xml: post.unwrap(), lineage_receipts: receipts,
    }, None, &[], &base, &CheckOptions { strict: true }).unwrap();
    let mut stale = current.clone(); stale.arrangement_stats.owned_rhythm = base.arrangement_stats.owned_rhythm.clone();
    assert!(owned_rhythm_diagnostic_line("Renamed", &stale).contains("Unknown: Owned rhythm belongs to another document state"));
    assert_eq!(base.layout_xml(), original);
    assert_eq!(stale.layout_xml(), current.layout_xml());
    assert!(current.master_state.lineage().lineage_continuity_holds(current.layout_xml()));
}

#[test]
fn actual_batch_stdout_filter_preserves_owned_context_on_stderr() {
    const CHILD: &str = "ETUS_TEST_OWNED_RHYTHM_STDERR_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let base = base_candidate();
        eprintln!("{}", owned_rhythm_diagnostic_line("SyntheticBatch", &base));
        return;
    }
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "pipeline::physical_rhythmic_birth_handoff::actual_batch_stdout_filter_preserves_owned_context_on_stderr", "--nocapture"])
        .env(CHILD, "1")
        .stdout(crate::audit_only::page_count_preference_projection::batch_child_stdout())
        .stderr(std::process::Stdio::piped()).spawn().unwrap();
    let capture = crate::audit_only::page_count_preference_projection::BatchChildPreferenceCapture::begin(&mut child).unwrap();
    let mut stderr = String::new();
    std::io::Read::read_to_string(&mut child.stderr.take().unwrap(), &mut stderr).unwrap();
    assert!(child.wait().unwrap().success());
    assert!(capture.finish().unwrap().is_none());
    let lines = stderr.lines().filter(|line| line.starts_with("ETUSCHLOPF: OwnedRhythmicContinuation:")).collect::<Vec<_>>();
    assert_eq!(lines.len(), 1, "child must execute the actual candidate diagnostic: {stderr}");
    assert!(lines[0].contains("SyntheticBatch"));
    assert!(lines[0].contains("complete_received_course_context"));
}

#[test]
fn actual_staff_patch_candidate_carries_context_and_missing_capture_preserves_music() {
    let base = base_candidate(); let n = parse_atomic_notes(base.layout_xml()).into_iter().find(|n| n.pitch.is_some()).unwrap();
    let patch = AtomicPatch { name: "SyntheticStaffMove".into(), note_start: n.note_start, note_end: n.note_end,
        target_staff: (3 - n.staff.parse::<u8>().unwrap()).to_string(), target_voice: "90001".into(), staff_value_range: n.staff_value_range,
        voice_value_range: n.voice_value_range, action: AtomicPatchAction::MoveStaff,
        insert_after: None, anchor_duration: None, material_transfer_carrier: None };
    let (xml, receipt) = apply_atomic_patch(base.layout_xml(), &patch);
    let mut receipts = master_state_identity::OwnerReceipts::default(); receipts.push_exact(receipt);
    let result = build_atomic_candidate(1, "SyntheticStaffMove", AtomicPatchedDocument {
        xml, lineage_receipts: receipts }, None, &[], &base, &CheckOptions { strict: true }).unwrap();
    result.arrangement_stats.owned_rhythm.as_ref().unwrap().require_complete_course_context(result.layout_xml()).unwrap();
    let (same_xml, mut receipt) = apply_atomic_patch(base.layout_xml(), &patch);
    receipt.as_mut().unwrap().take_staff_transition().unwrap();
    let mut receipts = master_state_identity::OwnerReceipts::default(); receipts.push_exact(receipt);
    let unknown = build_atomic_candidate(1, "SyntheticStaffMove", AtomicPatchedDocument {
        xml: same_xml, lineage_receipts: receipts }, None, &[], &base, &CheckOptions { strict: true }).unwrap();
    assert_eq!(result.layout_xml(), unknown.layout_xml());
    assert!(unknown.arrangement_stats.owned_rhythm.is_none());
    assert!(unknown.arrangement_stats.owned_rhythm_unknown.as_deref().unwrap().contains("PopulationRow=4"));
}

#[test]
fn direct_candidate_handoff_transports_original_receipts_and_keeps_unknown_first_cause() {
    let base = base_candidate(); let n = parse_atomic_notes(base.layout_xml()).into_iter().find(|n| n.pitch.is_some()).unwrap();
    let range = n.voice_value_range.unwrap();
    let execute = || {
        let (post, receipt) = structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
            base.layout_xml(), vec![(range.0, range.1, "90101".into())]);
        let mut receipts = master_state_identity::OwnerReceipts::default(); receipts.push_exact(receipt);
        (post.unwrap(), receipts)
    };
    let (post, receipts) = execute(); let mut candidate = base.clone();
    continue_candidate_owned_rhythm(&base, &mut candidate, &receipts, &post, "SyntheticDirectGesture");
    candidate.master_state.adopt_materialized_document(post.clone(), receipts, "SyntheticDirectGesture");
    candidate.arrangement_stats.owned_rhythm.as_ref().unwrap().require_complete_course_context(candidate.layout_xml()).unwrap();
    let mut unknown_base = base.clone(); unknown_base.arrangement_stats.owned_rhythm = None;
    unknown_base.arrangement_stats.owned_rhythm_unknown = Some("ActualEarlierOwnerFailure".into());
    let (same_post, receipts) = execute(); let mut unknown = unknown_base.clone();
    continue_candidate_owned_rhythm(&unknown_base, &mut unknown, &receipts, &same_post, "SyntheticDirectGesture");
    unknown.master_state.adopt_materialized_document(same_post, receipts, "SyntheticDirectGesture");
    assert_eq!(unknown.layout_xml(), candidate.layout_xml());
    assert_eq!(unknown.arrangement_stats.owned_rhythm_unknown.as_deref(), Some("ActualEarlierOwnerFailure"));
    let (_, receipts) = execute(); let mut stale = base.clone();
    continue_candidate_owned_rhythm(&base, &mut stale, &receipts, &format!("{post} "), "SyntheticDirectGesture");
    assert!(stale.arrangement_stats.owned_rhythm.is_none());
    assert!(stale.arrangement_stats.owned_rhythm_unknown.as_deref().unwrap().contains("SyntheticDirectGesture/OriginalOwners"));
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut candidate = base.clone();
        candidate.master_state.adopt_materialized_document(post,
            master_state_identity::OwnerReceipts::default(), "MissingOriginalReceipt");
    }));
    assert!(refusal.is_err());
}
