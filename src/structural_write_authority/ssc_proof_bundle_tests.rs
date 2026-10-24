//! PRESERVING/E036 SSC proof bundle: real owner -> receipt -> Master/Render
//! adoption -> released report, plus constructive integrity cases replayed on
//! real owner receipts. Observation only; documents are compared byte-exactly.
use super::{
    MasterAdoptedState, RenderSinkKind, RenderSovereignState,
};
use crate::master_state_identity::{
    AdditiveTransformationReceipt, MasterLineage, OwnerReceipts, PreservingTransformationReceipt,
    RenderTransitionReceipts, reset_master_lineage_measurement,
};
use crate::ssc_evidence_proof::{SscBundleProbeCase as Case, SscBundleProbeReceipt as Probe, ssc_bundle_probe_for_test};

// Synthetic, relocatable score: the measure ordinal is a parameter, never a witness.
fn score_at(ordinal: usize, notes: &str) -> String {
    format!(
        "<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>P</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>2</divisions><time><beats>2</beats><beat-type>4</beat-type></time></attributes>{notes}</measure></part></score-partwise>"
    )
}
const RELOCATIONS: [usize; 2] = [1, 37];
fn eighth(step: &str, beam: Option<&str>) -> String {
    let beam = beam.map_or_else(String::new, |value| format!("<beam number=\"1\">{value}</beam>"));
    format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>eighth</type><staff>1</staff>{beam}</note>")
}
fn four_at(ordinal: usize, beams: [Option<&str>; 4]) -> String {
    score_at(ordinal, &["C", "D", "E", "F"].iter().zip(beams).map(|(step, beam)| eighth(step, beam)).collect::<String>())
}
fn four(beams: [Option<&str>; 4]) -> String {
    four_at(RELOCATIONS[0], beams)
}
fn note_ranges(xml: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut cursor = 0;
    while let Some(start) = xml[cursor..].find("<note>").map(|offset| cursor + offset) {
        let end = xml[start..].find("</note>").unwrap() + start + "</note>".len();
        ranges.push((start, end));
        cursor = end;
    }
    ranges
}
// Synthetic dotted-eighth/sixteenth pair on different staves. E049 is the
// surviving beam-chain owner, including complete-beat mixed durations.
fn cross_staff_pair_at(ordinal: usize, well_formed: bool) -> String {
    let first = if well_formed { "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>" }
        else { "<beam number=\"1\">begin</beam>" };
    let second = if well_formed { "<beam number=\"1\">end</beam><beam number=\"2\">end</beam>" } else { "" };
    let notes = format!("<note><pitch><step>C</step><octave>4</octave></pitch><duration>3</duration><voice>1</voice><type>eighth</type><dot/><staff>2</staff>{first}</note><note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>16th</type><staff>1</staff>{second}</note>");
    score_at(ordinal, &notes).replace("<divisions>2</divisions>", "<divisions>4</divisions>").replace("<beats>2</beats>", "<beats>1</beats>")
}
fn closing_e049(pre: &str) -> (String, PreservingTransformationReceipt) {
    let (post, receipt) = crate::pipeline::ssc_committed_cross_staff_beam_owner_probe(pre);
    (post, receipt.expect("real E049 receipt"))
}
fn direction(xml: &mut String, payload: &str) -> AdditiveTransformationReceipt {
    let locator = super::first_structural_measure_locator(xml).expect("structural measure");
    let (inserted, receipt) = super::insert_direction_into_measure_authorized(xml, &locator, payload);
    assert_eq!(inserted, 1);
    receipt.expect("real E036 receipt")
}
const DYNAMIC: &str = "      <direction placement=\"below\">\n        <direction-type>\n          <dynamics>\n            <mf/>\n          </dynamics>\n        </direction-type>\n        <staff>1</staff>\n      </direction>\n";
const TEMPO: &str = "      <!-- etuschlopf_visible_tempo -->\n      <direction placement=\"above\">\n        <direction-type>\n          <words>&#x2669; = 96</words>\n        </direction-type>\n        <staff>1</staff>\n        <sound tempo=\"96\"/>\n      </direction>\n";
fn adopted(pre: &str) -> MasterAdoptedState {
    MasterAdoptedState::adopt_genesis_state(
        pre.to_owned(),
        MasterLineage::lineage_from_genesis(crate::master_state_identity::tests::synthetic_genesis_for_tests(pre)),
    )
}
fn report() -> String {
    let report = "TYPED-LOWER-CLEF-SURFACE-MATERIALIZATION\n----------------------------------------\nStatus: present.\nSILENT-VOICE-COVERAGE-WRITE-OUTCOME\n--------------------------------\nStatus: present.\nRendererOutcome: observed_from_existing_writer_audit.\n";
    crate::final_report::DraftReport::new(report.to_owned()).compact(str::to_owned).finalize().expect("final report").as_str().to_owned()
}
fn assert_lines(report: &str, lines: &[&str]) {
    for line in lines {
        assert!(report.lines().any(|candidate| candidate == *line), "{line}: {report}");
    }
}
fn adopt_master(pre: &str, post: &str, receipts: OwnerReceipts) -> MasterAdoptedState {
    let mut state = adopted(pre);
    state.adopt_materialized_document(post.to_owned(), receipts, "test bound SSC bundle owner");
    crate::master_state_identity::record_final_master_lineage(state.lineage(), state.document());
    state
}

#[test]
fn ssc_bundle_master_e049_closing_a_malformed_beam_sequence_is_a_permit() {
    for ordinal in RELOCATIONS {
        reset_master_lineage_measurement();
        let pre = cross_staff_pair_at(ordinal, false);
        let (post, receipt) = closing_e049(&pre);
        assert_ne!(pre, post);
        let mut receipts = OwnerReceipts::default();
        receipts.push_preserving(Some(receipt));
        let state = adopt_master(&pre, &post, receipts);
        assert_lines(&report(), &[
            "ReleasedOwnerEvents: 1", "MasterOwnerEvents: 1", "ClassifiedOwnerEvents: 1", "OutOfScope: 0",
            "Permit: 1", "InvariantDenied: 0", "MissingEvidence: 0", "FamilyPRESERVINGPermit: 1",
            "ScopePRESERVING_BEAM_CHAINPermit: 1", "ReleasedBalance: complete",
            "V4Permitted: 1", "V4ContainmentDenied: 0", "V4ContainmentUnproven: 0", "V4OutOfScope: 0",
            "V4EnvelopePROJECTION: unproven=0; denied=0; permitted=1", "V4Balance: complete",
        ]);
        assert_eq!(state.document(), post, "report cannot mutate the adopted Master");
    }
}

#[test]
fn ssc_bundle_master_e049_regrouping_a_well_formed_sequence_is_necessity_unproven() {
    reset_master_lineage_measurement();
    let pre = cross_staff_pair_at(RELOCATIONS[0], true);
    let (post, receipt) = closing_e049(&pre);
    // The real E049 owner keeps the primary begin/end and removes only the
    // well-formed secondary chain, which cannot span this mixed-duration pair.
    let expected = pre
        .replace("<beam number=\"2\">begin</beam>", "")
        .replace("<beam number=\"2\">end</beam>", "");
    let compact = |xml: &str| xml.chars().filter(|ch| !ch.is_whitespace()).collect::<String>();
    assert_eq!(compact(&post), compact(&expected));
    assert_ne!(pre, post);
    let mut receipts = OwnerReceipts::default();
    receipts.push_preserving(Some(receipt));
    let state = adopt_master(&pre, &post, receipts);
    assert_lines(&report(), &[
        "ReleasedOwnerEvents: 1", "OutOfScope: 0", "Permit: 0", "NecessityUnproven: 1",
        "FamilyPRESERVINGNecessityUnproven: 1", "ScopePRESERVING_BEAM_CHAINNecessityUnproven: 1",
        // Contract v4: a contained beam regrouping is a Permit; necessity is only measured.
        "V4Permitted: 1", "V4ContainmentDenied: 0", "V4Balance: complete",
    ]);
    assert_eq!(state.document(), post);
}

#[test]
fn ssc_bundle_render_e049_is_observed_through_render_adoption_and_released_closure() {
    reset_master_lineage_measurement();
    crate::master_state_identity::reset_render_lineage_measurement();
    let pre = cross_staff_pair_at(RELOCATIONS[0], false);
    let master = adopted(&pre);
    let path = std::env::temp_dir().join(format!("etuschlopf-ssc-bundle-{}.render.musicxml", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let mut render = RenderSovereignState::begin_project_render(&path, &master);
    let write = |artifact: &std::path::Path, _: &str, candidate: &str| (std::fs::write(artifact, candidate), None);
    let root = render.initial_trial_root(true);
    render.adopt_transition(root, &pre, RenderTransitionReceipts::new(), RenderSinkKind::PrunedPdfTrial, write).expect("Render genesis");
    let (post, receipt) = closing_e049(&pre);
    let mut chain = RenderTransitionReceipts::new();
    chain.push_preserving(Some(receipt), &post);
    let root = render.require_state_root("SSC E049 Render owner");
    render.adopt_transition(root, &post, chain, RenderSinkKind::Checkpoint(super::RenderCheckpointSink::ExternalFeedbackClosure), write).expect("Render E049 adoption");
    use crate::master_state_identity::{RenderStageBoundary as Stage, record_render_stage_boundary};
    for stage in [Stage::InitialRender, Stage::RestArticulation, Stage::MetricBeam, Stage::FinalNotation, Stage::PdfSourceSelection, Stage::Release] {
        record_render_stage_boundary(stage);
    }
    assert!(render.close_render_sovereignty().closed);
    assert_lines(&report(), &[
        "RenderOwnerEvents: 1", "MasterOwnerEvents: 0", "ReleasedOwnerEvents: 1", "OutOfScope: 0",
        "Permit: 1", "FamilyPRESERVINGPermit: 1", "ScopePRESERVING_BEAM_CHAINPermit: 1",
        "V4Permitted: 1", "V4Balance: complete",
    ]);
    assert_eq!(std::fs::read_to_string(&path).expect("Render artifact"), post);
    let _ = std::fs::remove_file(path);
    crate::master_state_identity::reset_render_lineage_measurement();
}

#[test]
fn ssc_bundle_master_e044_layout_defaults_are_preserving_but_necessity_unproven() {
    reset_master_lineage_measurement();
    let pre = four([None, None, None, None]);
    let mut post = pre.clone();
    let (written, receipt) = super::ensure_visual_layout_defaults_authorized(&mut post, 90, 70);
    assert!(written.expect("E044 owner"));
    let mut receipts = OwnerReceipts::default();
    receipts.push_preserving(Some(receipt.expect("real E044 receipt")));
    let state = adopt_master(&pre, &post, receipts);
    assert_lines(&report(), &[
        "ReleasedOwnerEvents: 1", "OutOfScope: 0", "Permit: 0", "InvariantDenied: 0", "NecessityUnproven: 1",
        "FamilyPRESERVINGNecessityUnproven: 1", "ScopePRESERVING_UNBOUNDNecessityUnproven: 1",
        "V4Permitted: 1", "V4ContainmentDenied: 0", "V4Balance: complete",
    ]);
    assert_eq!(state.document(), post);
}

#[test]
fn ssc_bundle_master_e036_dynamics_and_tempo_have_geometry_but_no_necessity_certificate() {
    for ordinal in RELOCATIONS {
        reset_master_lineage_measurement();
        let pre = four_at(ordinal, [None, None, None, None]);
        let mut post = pre.clone();
        let dynamic = direction(&mut post, DYNAMIC);
        let tempo = direction(&mut post, TEMPO);
        let mut receipts = OwnerReceipts::default();
        receipts.push_additive(Some(dynamic));
        receipts.push_additive(Some(tempo));
        let state = adopt_master(&pre, &post, receipts);
        assert_lines(&report(), &[
            "ReleasedOwnerEvents: 2", "OutOfScope: 0", "Permit: 0", "InvariantDenied: 0", "NecessityUnproven: 2",
            "FamilyADDITIVENecessityUnproven: 2", "FamilyADDITIVEPermit: 0",
            "ScopeDIRECTION_DYNAMICSNecessityUnproven: 1", "ScopeDIRECTION_TEMPONecessityUnproven: 1",
            "V4Permitted: 2", "V4EnvelopeINSERTION: unproven=0; denied=0; permitted=2", "V4Balance: complete",
        ]);
        assert_eq!(state.document(), post);
    }
}

#[test]
fn ssc_bundle_unadopted_and_unreleased_candidates_are_never_released_evidence() {
    reset_master_lineage_measurement();
    let pre = cross_staff_pair_at(RELOCATIONS[0], false);
    let (post, receipt) = closing_e049(&pre);
    let mut losing = adopted(&pre);
    let mut receipts = OwnerReceipts::default();
    receipts.push_preserving(Some(receipt));
    losing.adopt_materialized_document(post, receipts, "losing candidate");
    let mut dropped = pre.clone();
    drop(direction(&mut dropped, DYNAMIC));
    let released = adopted(&pre);
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    assert_lines(&report(), &["ReleasedOwnerEvents: 0", "ClassifiedOwnerEvents: 0", "Permit: 0", "FamilyPRESERVINGPermit: 0", "ReleasedBalance: complete", "V4Permitted: 0", "V4Balance: complete"]);
}

fn beam_probe() -> Probe {
    Probe::Preserving(closing_e049(&cross_staff_pair_at(RELOCATIONS[0], false)).1)
}
fn direction_probe() -> Probe {
    let mut post = four([None, None, None, None]);
    Probe::Additive(direction(&mut post, DYNAMIC))
}
fn layout_probe() -> Probe {
    let mut post = four([None, None, None, None]);
    Probe::Preserving(super::ensure_visual_layout_defaults_authorized(&mut post, 90, 70).1.expect("real E044 receipt"))
}
fn barline_probe() -> Probe {
    let mut post = four([None, None, None, None]);
    Probe::Additive(super::ensure_final_barline_authorized(&mut post).expect("E004").1.expect("real E004 receipt"))
}

#[test]
fn ssc_bundle_released_missing_duplicate_domain_identity_and_capture_cases_are_exact() {
    let pre = cross_staff_pair_at(RELOCATIONS[0], false);
    for owner in [beam_probe as fn() -> Probe, direction_probe] {
        let released = ssc_bundle_probe_for_test(owner(), beam_probe(), Case::Released);
        assert_eq!((released["Released"], released["Classified"], released["Invalid"]), (1, 1, 0));
        let missing = ssc_bundle_probe_for_test(owner(), beam_probe(), Case::Missing);
        assert_eq!((missing["Missing"], missing["Classified"]), (1, 0));
        let duplicate = ssc_bundle_probe_for_test(owner(), beam_probe(), Case::Duplicate);
        assert_eq!((duplicate["Duplicate"], duplicate["Classified"]), (1, 0));
        let domain = ssc_bundle_probe_for_test(owner(), beam_probe(), Case::WrongDomain);
        assert_eq!((domain.get("DomainMismatch"), domain["Classified"]), (Some(&1), 0));
        let foreign = ssc_bundle_probe_for_test(owner(), beam_probe(), Case::ForeignIdentity);
        assert_eq!((foreign.get("ReceiptIdentityMismatch"), foreign["Classified"]), (Some(&1), 0));
        for (case, expected) in [
            (Case::StalePre(four([Some("begin"), Some("end"), None, None])), "PreFingerprintMismatch"),
            (Case::WrongPost(pre.clone()), "PostFingerprintMismatch"),
            (Case::CorruptPre, "PreStateUnreadable"),
            (Case::CorruptPost, "PostStateUnreadable"),
        ] {
            let counts = ssc_bundle_probe_for_test(owner(), beam_probe(), case);
            assert_eq!((counts.get(expected), counts["Classified"]), (Some(&1), 0), "{expected}");
        }
    }
}

#[test]
fn ssc_adoption_decision_refuses_every_integrity_and_binding_failure() {
    // r1287 (contract v4 A7): the adoption decision is taken over the report's classifier.
    // Only the bound, contained release is admitted; foreign identity, foreign scope, stale
    // pre, deviating post and broken captures are refused.
    let pre = cross_staff_pair_at(RELOCATIONS[0], false);
    for owner in [beam_probe as fn() -> Probe, direction_probe] {
        assert_eq!(ssc_bundle_probe_for_test(owner(), beam_probe(), Case::Released)["AdoptionPermitted"], 1);
        for case in [
            Case::ForeignIdentity,
            Case::StalePre(four([Some("begin"), Some("end"), None, None])),
            Case::WrongPost(pre.clone()),
            Case::CorruptPre,
            Case::CorruptPost,
        ] {
            assert_eq!(ssc_bundle_probe_for_test(owner(), beam_probe(), case)["AdoptionPermitted"], 0);
        }
    }
    for (target, foreign_scope) in [(layout_probe as fn() -> Probe, beam_probe as fn() -> Probe), (barline_probe, direction_probe)] {
        assert_eq!(ssc_bundle_probe_for_test(target(), foreign_scope(), Case::ForeignScope)["AdoptionPermitted"], 0);
    }
}

#[test]
fn ssc_bundle_origin_scope_is_evidence_only_for_its_own_receipt_kind_and_descriptor() {
    for (target, foreign_scope) in [
        (layout_probe as fn() -> Probe, beam_probe as fn() -> Probe),
        (barline_probe, direction_probe),
        (direction_probe, barline_probe),
        (barline_probe, beam_probe),
        (beam_probe, direction_probe),
    ] {
        let counts = ssc_bundle_probe_for_test(target(), foreign_scope(), Case::ForeignScope);
        assert_eq!((counts.get("ProofFamilyMismatch"), counts["Classified"]), (Some(&1), 0));
    }
}

#[test]
fn ssc_bundle_changed_projection_and_broken_geometry_are_constructive_denials() {
    let pre = cross_staff_pair_at(RELOCATIONS[0], false);
    let changed_pitch = pre.replacen("<step>C</step>", "<step>G</step>", 1);
    let denied = ssc_bundle_probe_for_test(beam_probe(), beam_probe(), Case::ChangedProjection(changed_pitch.clone()));
    assert_eq!((denied["InvariantDenied"], denied["Permit"], denied["NecessityUnproven"]), (1, 0, 0));
    let plain = four([None, None, None, None]);
    let moved = plain.replacen("<step>D</step>", "<step>A</step>", 1).replacen("<note>", "<direction><direction-type><words>x</words></direction-type></direction><note>", 1);
    let denied = ssc_bundle_probe_for_test(direction_probe(), beam_probe(), Case::ChangedProjection(moved));
    assert_eq!((denied["InvariantDenied"], denied["Permit"]), (1, 0));
    let layout_changed = ssc_bundle_probe_for_test(layout_probe(), beam_probe(), Case::ChangedProjection(changed_pitch));
    assert_eq!((layout_changed["InvariantDenied"], layout_changed["Permit"]), (1, 0));
}

fn edit_block_score() -> String {
    "<score-partwise><part id=\"P\"><measure><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note></measure></part></score-partwise>".to_owned()
}
#[test]
fn ssc_edit_visibility_real_owner_receipt_adoption_and_report() {
    reset_master_lineage_measurement();
    let pre = edit_block_score(); let mut post = pre.clone(); let (start,end) = note_ranges(&pre)[0];
    let (changed, receipt) = super::hide_note_print_object_authorized(&mut post, start, end);
    assert!(changed.unwrap()); assert!(receipt.is_some());
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let state = adopt_master(&pre, &post, receipts);
    // r1284 4b: the sealed writer also sets print-spacing; the row now declares it.
    assert!(post.contains("print-object=\"no\" print-spacing=\"no\""));
    assert_lines(&report(), &["ReleasedOwnerEvents: 1", "V4OutOfScope: 0", "V4Permitted: 1", "V4ContainmentDenied: 0", "V4EnvelopeEDIT: unproven=0; denied=0; permitted=1", "V4ContentChangingPermitted: 0", "ReleasedBalance: complete", "V4Balance: complete", "PermitEnforcement: active"]);
    assert!(!report().contains("V4DeniedWitness:"));
    assert_eq!(state.document(), post);
}
fn allowed_range_score() -> String {
    "<score-partwise><part id=\"P\"><measure><attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff><notations><slur type=\"start\" number=\"1\"/></notations></note></measure></part></score-partwise>".to_owned()
}
#[test]
fn ssc_edit_allowed_range_real_clef_and_notation_removals_are_permitted() {
    let pre = allowed_range_score();
    let clef = (pre.find("<attributes>").unwrap(), pre.find("</attributes>").unwrap() + "</attributes>".len());
    let slur = (pre.find("<slur").unwrap(), pre.find("/></notations>").unwrap() + "/>".len());
    for range in [clef, slur] {
        reset_master_lineage_measurement();
        let mut post = pre.clone();
        let (changed, receipt) = super::remove_allowed_xml_ranges_authorized(&mut post, &[range]);
        assert_eq!(changed.unwrap(), 1); assert!(receipt.is_some());
        let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
        let state = adopt_master(&pre, &post, receipts);
        assert_lines(&report(), &["ReleasedOwnerEvents: 1", "V4OutOfScope: 0", "V4Permitted: 1", "V4ContainmentDenied: 0", "V4EnvelopeEDIT: unproven=0; denied=0; permitted=1", "V4ContentChangingPermitted: 1", "ReleasedBalance: complete", "V4Balance: complete", "PermitEnforcement: active"]);
        assert_eq!(state.document(), post);
    }
}
#[test]
fn ssc_edit_content_real_pitch_owner_receipt_adoption_and_report() {
    reset_master_lineage_measurement();
    let pre = edit_block_score(); let mut post = pre.clone();
    let start = pre.find("<pitch>").unwrap(); let end = pre.find("</pitch>").unwrap() + "</pitch>".len();
    let (changed, receipt) = super::apply_harmonic_pitch_swap_authorized(&mut post, start, end, "<pitch><step>D</step><octave>4</octave></pitch>");
    assert!(changed); assert!(receipt.is_some());
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let state = adopt_master(&pre, &post, receipts);
    assert_lines(&report(), &["ReleasedOwnerEvents: 1", "V4OutOfScope: 0", "V4Permitted: 1", "V4EnvelopeEDIT: unproven=0; denied=0; permitted=1", "V4ContentChangingPermitted: 1", "ReleasedBalance: complete", "V4Balance: complete"]);
    assert_eq!(state.document(), post);
}
#[test]
fn ssc_block_span_real_clef_owner_receipt_adoption_and_report() {
    reset_master_lineage_measurement();
    let pre = edit_block_score(); let start = pre.find("<measure>").unwrap(); let end = pre.find("</measure>").unwrap() + "</measure>".len();
    let replacement = pre[start..end].replace("<sign>G</sign><line>2", "<sign>F</sign><line>4");
    let (post, receipt) = super::replace_bound_clef_measure_authorized(&pre, start, end, replacement);
    let post = post.unwrap(); let receipt = receipt.unwrap();
    assert!(crate::ssc_evidence_proof::ssc_edit_scope_on_block_probe_for_test(&receipt));
    assert_eq!(crate::master_state_identity::ssc_block_bound_span(&receipt), Some((start,end)));
    let mut receipts = OwnerReceipts::default(); receipts.push_block(Some(receipt));
    let state = adopt_master(&pre, &post, receipts);
    assert_lines(&report(), &["ReleasedOwnerEvents: 1", "V4OutOfScope: 0", "V4Permitted: 1", "V4EnvelopeBLOCK: unproven=0; denied=0; permitted=1", "V4ContentChangingPermitted: 0", "ReleasedBalance: complete", "V4Balance: complete"]);
    assert_eq!(state.document(), post);
}
#[path = "../../tests/support/ssc_edit_block_document_owner.rs"]
mod edit_block_document_owner;

#[test]
fn ssc_edit_scalar_real_owner_receipt_adoption_and_report() {
    reset_master_lineage_measurement();
    let pre = edit_block_score(); let start = pre.find("<voice>").unwrap() + "<voice>".len();
    let (post, receipt) = super::apply_atomic_staff_voice_scalar_batch_authorized(&pre, vec![(start, start + 1, "2".to_owned())]);
    let post = post.unwrap(); assert!(receipt.is_some());
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let state = adopt_master(&pre, &post, receipts);
    assert_lines(&report(), &["ReleasedOwnerEvents: 1", "V4OutOfScope: 0", "V4Permitted: 1", "V4EnvelopeEDIT: unproven=0; denied=0; permitted=1", "V4ContentChangingPermitted: 0", "ReleasedBalance: complete", "V4Balance: complete"]);
    assert_eq!(state.document(), post);
}

// r1285: real candidate commits survive release selection but cannot enlarge
// the released denominator. The score has no project or measure identity.
fn commit_scalar_candidate(pre: &str, field: &str) -> MasterAdoptedState {
    let start = pre.find(field).unwrap() + field.len();
    let (post, receipt) = super::apply_atomic_staff_voice_scalar_batch_authorized(
        pre, vec![(start, start + 1, "2".to_owned())],
    );
    let post = post.expect("real scalar owner post");
    let mut receipts = OwnerReceipts::default();
    receipts.push_exact(Some(receipt.expect("real scalar owner receipt")));
    let mut state = adopted(pre);
    state.adopt_materialized_document(post, receipts, "SSC candidate measurement");
    state
}

fn close_empty_render_scope(master: &MasterAdoptedState, label: &str) {
    let path = std::env::temp_dir().join(format!("ssc-unreleased-{label}-{}.render.musicxml", std::process::id()));
    let _ = std::fs::remove_file(&path);
    let mut render = RenderSovereignState::begin_project_render(&path, master);
    let root = render.initial_trial_root(true);
    render.adopt_transition(root, master.document(), RenderTransitionReceipts::new(), RenderSinkKind::PrunedPdfTrial,
        |artifact, _, candidate| (std::fs::write(artifact, candidate), None)).expect("real Render genesis");
    use crate::master_state_identity::{RenderStageBoundary as Stage, record_render_stage_boundary};
    for stage in [Stage::InitialRender, Stage::RestArticulation, Stage::MetricBeam, Stage::FinalNotation, Stage::PdfSourceSelection, Stage::Release] {
        record_render_stage_boundary(stage);
    }
    assert!(render.close_render_sovereignty().closed);
    std::fs::remove_file(path).expect("remove Render test artifact");
}

#[test]
fn ssc_unreleased_real_owner_candidate_survives_retain_and_is_not_released() {
    reset_master_lineage_measurement();
    crate::master_state_identity::reset_render_lineage_measurement();
    let pre = edit_block_score();
    let losing = commit_scalar_candidate(&pre, "<voice>");
    let released = adopted(&pre);
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    close_empty_render_scope(&released, "retained");
    let measured = report();
    assert_lines(&measured, &[
        "ReleasedOwnerEvents: 0", "V4Permitted: 0", "ReleasedBalance: complete", "V4Balance: complete",
        "V4UnreleasedCommits: checked=1; permitted=1; denied=0; unproven=0; out_of_scope=0; integrity=0",
        "V4AllCommitsPermitted: yes", "PermitEnforcement: active",
    ]);
    assert_ne!(losing.document(), released.document());
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    assert_eq!(report(), measured, "repeated retain must not duplicate or remove evidence");
}

#[test]
fn ssc_exact_owner_outside_envelope_is_refused_before_commit_with_a_constructive_witness() {
    // r1287 (contract v4 A7): the real scalar owner edits the octave, outside its voice/staff
    // envelope. Under enforcement the candidate is never committed; the refusal carries the
    // constructive witness and leaves document and lineage unchanged.
    reset_master_lineage_measurement();
    crate::master_state_identity::reset_render_lineage_measurement();
    let pre = edit_block_score();
    let start = pre.find("<octave>").unwrap() + "<octave>".len();
    let (post, receipt) = super::apply_atomic_staff_voice_scalar_batch_authorized(&pre, vec![(start, start + 1, "2".to_owned())]);
    let post = post.expect("real scalar owner post");
    let mut state = adopted(&pre);
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut receipts = OwnerReceipts::default();
        receipts.push_exact(Some(receipt.expect("real scalar owner receipt")));
        state.adopt_materialized_document(post, receipts, "SSC candidate measurement");
    }))
    .expect_err("an envelope escape must be refused");
    let message = refusal.downcast_ref::<String>().cloned().unwrap_or_default();
    assert!(message.contains("MASTER SOVEREIGNTY VIOLATION") && message.contains("SSC adoption refused: permitted=0; denied=1"), "{message}");
    for field in ["V4DeniedWitness: descriptor=STAFF_VOICE_SCALAR_BATCH_EXACT", "reason=envelope_escape", "basis=", "pre_range=", "post_range=", "pre_excerpt=", "post_excerpt="] {
        assert!(message.contains(field), "{field}: {message}");
    }
    assert_eq!(state.document(), pre);
    assert_eq!(state.lineage().committed_event_count(), 0);
    crate::master_state_identity::record_final_master_lineage(state.lineage(), state.document());
    close_empty_render_scope(&state, "refused");
    assert_lines(&report(), &["ReleasedOwnerEvents: 0", "V4UnreleasedCommits: checked=0; permitted=0; denied=0; unproven=0; out_of_scope=0; integrity=0", "V4AllCommitsPermitted: yes", "PermitEnforcement: active"]);
}

#[test]
fn ssc_unreleased_orphan_preserves_all_existing_released_report_lines() {
    reset_master_lineage_measurement();
    crate::master_state_identity::reset_render_lineage_measurement();
    let pre = edit_block_score();
    let released = commit_scalar_candidate(&pre, "<voice>");
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    close_empty_render_scope(&released, "orphan");
    let before = report();
    let losing = commit_scalar_candidate(&pre, "<staff>");
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    let after = report();
    // This extracts every existing SSC coverage line, including V2 families,
    // Missing/Duplicate, scopes and both balances, in its original order.
    let coverage = |report: &str| {
        report.split("ReleaseScopeMissing:").nth(1).unwrap()
            .split("V4UnreleasedCommits:").next().unwrap().to_owned()
    };
    assert_eq!(coverage(&before), coverage(&after));
    assert_lines(&after, &[
        "ReleasedOwnerEvents: 1", "V4Permitted: 1", "MissingEvidence: 0", "DuplicateEvidenceIdentities: 0",
        "ReleasedBalance: complete", "V4Balance: complete",
        "V4UnreleasedCommits: checked=1; permitted=1; denied=0; unproven=0; out_of_scope=0; integrity=0",
        "V4AllCommitsPermitted: yes",
    ]);
    drop(losing);
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    assert_eq!(report(), after);
}

#[test]
fn ssc_unreleased_synthetic_candidate_batch_counts_every_commit_once() {
    reset_master_lineage_measurement();
    crate::master_state_identity::reset_render_lineage_measurement();
    let note = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>";
    let pre = format!("<score-partwise><part><measure>{}</measure></part></score-partwise>", note.repeat(384));
    for _ in 0..32 {
        drop(commit_scalar_candidate(&pre, "<voice>"));
    }
    let released = adopted(&pre);
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    close_empty_render_scope(&released, "batch");
    let measured = report();
    assert_lines(&measured, &[
        "ReleasedOwnerEvents: 0",
        "V4UnreleasedCommits: checked=32; permitted=32; denied=0; unproven=0; out_of_scope=0; integrity=0",
        "V4AllCommitsPermitted: yes",
    ]);
    crate::master_state_identity::record_final_master_lineage(released.lineage(), released.document());
    assert_eq!(report(), measured);
}
