use super::*;
use super::super::written_assignment_graph_probe;

fn id(source: usize) -> OccId { OccId { source, attack: source, fragment: source, emission: 0 } }
fn note(source: usize, measure: usize, start: i64, end: i64) -> Occurrence {
    Occurrence { id: id(source), staff: Staff(1), written: WrittenInterval { measure, start, end }, chord: ChordBinding::Head }
}
fn model(rows: Vec<Occurrence>) -> Model { Model { occurrences: rows, links: Vec::new() } }
fn labels(rows: &[(usize, &str)]) -> BTreeMap<OccId, String> {
    rows.iter().map(|(source, label)| (id(*source), (*label).into())).collect()
}

#[test]
fn written_graph_distinguishes_contact_overlap_and_structural_time_blocks() {
    let m = model(vec![note(701, 1, 0, 2), note(503, 1, 2, 4)]);
    assert!(matches!(check(&m, &labels(&[(701, "x"), (503, "x")])), Outcome::GraphSatisfied { .. }));
    let mut overlap = m.clone(); overlap.occurrences[1].written.start = 1;
    assert!(matches!(check(&overlap, &labels(&[(701, "x"), (503, "x")])), Outcome::Violation(Violation::SeparationMerged(_))));
    overlap.occurrences[1].written.measure = 2;
    assert!(matches!(check(&overlap, &labels(&[(701, "x"), (503, "x")])), Outcome::GraphSatisfied { .. }));
}

#[test]
fn written_graph_replays_and_refuses_forged_transitive_certificates() {
    let mut m = model(vec![note(701, 1, 0, 3), note(503, 1, 4, 5), note(907, 1, 2, 4)]);
    m.links = vec![HardLink { left: id(701), right: id(503), origin: FactOrigin::HardSame(0) },
        HardLink { left: id(503), right: id(907), origin: FactOrigin::ReceivedBeam { level: 1, group: (4, 7) } }];
    let Outcome::Contradiction(mut proof) = analyze(&m).unwrap_err() else { panic!("missing certificate") };
    assert_eq!(proof.equality_path.len(), 2);
    assert!(verify_contradiction(&m, &proof));
    proof.equality_path[0].origin = FactOrigin::HardSame(99);
    assert!(!verify_contradiction(&m, &proof));
}

#[test]
fn written_graph_requires_positive_chord_binding_and_accepts_shorter_members() {
    let mut m = model(vec![note(701, 1, 0, 8), note(503, 1, 0, 4)]);
    let a = labels(&[(701, "x"), (503, "x")]);
    assert!(matches!(check(&m, &a), Outcome::Violation(_)));
    m.occurrences[1].chord = ChordBinding::Member(id(701));
    assert!(matches!(check(&m, &a), Outcome::GraphSatisfied { .. }));
    assert!(matches!(check(&m, &labels(&[(701, "x"), (503, "y")])), Outcome::Violation(Violation::HardLinkSplit(_))));
    m.occurrences[1].written.end = 9;
    assert!(matches!(analyze(&m), Err(Outcome::Unknown(_))));
}

#[test]
fn unbound_chord_keeps_proven_violations_visible_without_inventing_an_overlap() {
    let mut m = model(vec![note(701, 1, 0, 4), note(503, 1, 0, 4),
        note(907, 2, 0, 2), note(909, 2, 2, 4)]);
    m.occurrences[1].chord = ChordBinding::Unbound;
    m.links.push(HardLink { left: id(907), right: id(909), origin: FactOrigin::HardSame(0) });
    let split = labels(&[(701, "x"), (503, "x"), (907, "x"), (909, "y")]);
    let Outcome::Violation(Violation::HardLinkSplit(edge)) = check(&m, &split) else {
        panic!("the unrelated hard-link split is proved");
    };
    assert_eq!((edge.left, edge.right), (id(907), id(909)));
    let joined = labels(&[(701, "x"), (503, "x"), (907, "x"), (909, "x")]);
    assert_eq!(check(&m, &joined), Outcome::Unknown("unsupported_unbound_projected_chord_host"));
    m.occurrences[1].chord = ChordBinding::Member(id(701));
    assert!(matches!(check(&m, &joined), Outcome::GraphSatisfied { .. }));
}

#[test]
fn written_graph_keeps_consumer_capacity_and_other_proofs_open() {
    let m = model(vec![note(701, 1, 0, 1), note(503, 1, 2, 3)]);
    let a = labels(&[(701, "x"), (503, "y")]);
    let Outcome::GraphSatisfied { pending_proofs, .. } = check(&m, &a) else { panic!() };
    assert!(pending_proofs.contains(&"musical_silence_obligations"));
    assert!(pending_proofs.contains(&"optional_connection_lifecycle"));
    assert!(pending_proofs.contains(&"final_visible_preservation"));
    assert!(matches!(check(&m, &BTreeMap::new()), Outcome::Unknown(_)));
}

#[test]
fn written_graph_uses_actual_emission_ordinals_and_is_invariant_under_independent_copy() {
    for copies in [1, 2] {
        for shift in [0, 1_000_000] {
            let mut m = model(Vec::new()); let mut a = BTreeMap::new();
            for copy in 0..copies {
                let mut first = note(701 + copy, copy + 1, shift, shift + 1);
                let mut second = note(701 + copy, copy + 1, shift + 2, shift + 3);
                first.id.emission = 0; second.id.emission = 1;
                a.insert(first.id, "opaque-label".into()); a.insert(second.id, "opaque-label".into());
                m.links.push(HardLink { left: first.id, right: second.id,
                    origin: FactOrigin::SourceContinuation(first.id.source, first.id.attack) });
                m.occurrences.extend([second, first]);
            }
            let Outcome::GraphSatisfied { classes, .. } = check(&m, &a) else { panic!() };
            assert_eq!(classes.len(), copies);
        }
    }
}

fn actual_inputs() -> (super::super::CanonicalFinalChordStructuralPlan,
    super::super::FrozenSerializationVoiceChannelPlan, super::super::FrozenSerializedVoicePlan,
    super::super::ActualSourceBeamBindingObservation) {
    use super::super::*;
    let source = crate::pipeline::actual_beam_owner_capture_tests::input().replace("probe", "47");
    let (xml, _, capture) = crate::pipeline::actual_beam_owner_capture_tests::execute(&source);
    let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
    let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
    let attacks = canonical.source_locators.iter().map(|row| FrozenVoiceAttackEvidenceRow {
        source_event_id: row.source_event_id, attack_id: row.attack_id,
        absolute_start: row.onset, absolute_end: row.onset + row.duration,
        staff: row.staff.parse().unwrap(), technical_lane: 1,
    }).collect::<Vec<_>>();
    let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
    let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
    let observation = observe_actual_source_beam_binding(&canonical, &xml, Some(&Ok(capture)));
    (canonical, channels, voices, observation)
}

#[test]
fn actual_owner_graph_probe_is_bound_and_leaves_the_real_candidate_unchanged() {
    let (canonical, channels, voices, observation) = actual_inputs();
    let xml = canonical.written_projection.as_ref().unwrap().xml.clone();
    let before = voices.source_to_channel.clone();
    let mut out = String::new();
    written_assignment_graph_probe::append(&mut out, &channels, &canonical, &observation, &voices);
    assert!(out.contains("WrittenAssignmentGraph: bound;"), "{out}");
    let readback = out.lines().find(|line| line.starts_with("OrderedReceivedBeamReadback: seam="))
        .expect("ordered readback ran on the original owner fixture");
    let present = readback.split("; ").find_map(|field| field.strip_prefix("present="))
        .and_then(|value| value.parse::<usize>().ok()).unwrap();
    assert!(present > 0, "{readback}");

    assert!(out.contains("graph=hard_received_bindings; result=GraphSatisfied"), "{out}");
    assert!(out.contains("WrittenAssignmentGraphRetentionPremises:"), "{out}");
    assert!(out.contains("retention=conditional; optional_lifecycle=pending"));
    assert!(out.contains("admission=none"));
    assert_eq!(canonical.written_projection.as_ref().unwrap().xml, xml);
    assert_eq!(voices.source_to_channel, before);
}

#[test]
fn actual_probe_reports_unbound_projected_chord_without_discarding_the_graph() {
    let (mut canonical, channels, voices, observation) = actual_inputs();
    let written = canonical.written_projection.as_mut().unwrap();
    let xml = written.xml.clone();
    let index = written.occurrences.iter().position(|row| {
        !row.is_grace && row.has_pitched_chord_anchor
            && !written.occurrences.iter().any(|other| {
                other.projected_note_start != row.projected_note_start
                    && other.chord_anchor_start == row.projected_note_start
            })
    }).expect("fixture contains a note without dependent chord members");
    std::sync::Arc::make_mut(&mut written.occurrences)[index].has_pitched_chord_anchor = false;
    let mut out = String::new();
    written_assignment_graph_probe::append(&mut out, &channels, &canonical, &observation, &voices);
    assert!(out.contains("WrittenAssignmentGraphPendingChordBinding: occurrences=1"), "{out}");
    assert!(out.contains("WrittenAssignmentGraphWitness: graph=pending_chord_binding;"), "{out}");
    assert!(out.contains("graph=hard_received_bindings; result=Unknown; reason=unsupported_unbound_projected_chord_host"), "{out}");
    assert_eq!(canonical.written_projection.as_ref().unwrap().xml, xml);
}

#[test]
fn missing_original_capture_is_unknown_and_is_not_an_empty_retention_graph() {
    let (canonical, channels, voices, _) = actual_inputs();
    let mut out = String::new();
    written_assignment_graph_probe::append(&mut out, &channels, &canonical, &Default::default(), &voices);
    assert!(out.contains("graph=hard_received_bindings; result=GraphSatisfied"), "{out}");
    assert!(out.contains("graph=conditional_received_beam_retention; result=Unknown"), "{out}");
    assert!(out.contains("OrderedReceivedBeamReadback: result=Unknown;"), "{out}");
}

#[test]
fn mismatched_actual_assignment_and_duplicate_producer_ordinal_are_refused() {
    let (mut canonical, channels, mut voices, observation) = actual_inputs();
    let valid_voices = voices.clone();
    voices.source_to_channel.clear();
    let mut out = String::new();
    written_assignment_graph_probe::append(&mut out, &channels, &canonical, &observation, &voices);
    assert!(out.contains("foreign_or_unbound_actual_assignment"));
    let written = canonical.written_projection.as_mut().unwrap();
    let mut rows = written.occurrences.to_vec(); rows.push(rows[0].clone()); written.occurrences = rows.into();
    out.clear();
    written_assignment_graph_probe::append(&mut out, &channels, &canonical, &observation, &valid_voices);
    assert!(out.contains("duplicate_actual_producer_emission"), "{out}");
}

#[test]
fn canonical_cross_staff_links_keep_their_meaning_and_beams_need_lane_evidence() {
    let mut graph = model(vec![note(701, 1, 0, 2), note(503, 1, 2, 4)]);
    graph.occurrences[1].staff = Staff(2);
    graph.links.push(HardLink { left: id(701), right: id(503), origin: FactOrigin::HardSame(0) });
    let same = labels(&[(701, "x"), (503, "x")]);
    let different = labels(&[(701, "x"), (503, "y")]);
    assert!(matches!(check(&graph, &same), Outcome::GraphSatisfied { .. }));
    assert!(matches!(check(&graph, &different), Outcome::Violation(Violation::HardLinkSplit(_))));
    graph.links[0].origin = FactOrigin::ReceivedBeam { level: 1, group: (4, 7) };
    assert_eq!(check(&graph, &same), Outcome::Unknown("cross-staff received beam requires ordered lane evidence"));
}

#[test]
fn cross_staff_chord_with_a_bound_head_still_requires_one_channel() {
    let mut graph = model(vec![note(701, 1, 0, 4), note(503, 1, 0, 2)]);
    graph.occurrences[1].staff = Staff(2);
    graph.occurrences[1].chord = ChordBinding::Member(id(701));
    assert!(matches!(check(&graph, &labels(&[(701, "x"), (503, "x")])), Outcome::GraphSatisfied { .. }));
    assert!(matches!(check(&graph, &labels(&[(701, "x"), (503, "y")])),
        Outcome::Violation(Violation::HardLinkSplit(_))));
}
