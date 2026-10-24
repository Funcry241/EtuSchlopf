// Synthetic controls for the consumer admission of frozen serialization
// channels. Every case runs the real canonical planner and the real channel
// owner; the consumer side is checked by an independent emulation of
// MuseScore 4.7.4 `allocateStaves` (including the no-op `setStaff(-2)`).
use super::*;

#[derive(Clone, Copy)]
struct Spec {
    written: (i64, i64),
    sounding: (i64, i64),
    staff: u8,
    lane: usize,
    midi_step: usize,
}

#[derive(Clone, Copy)]
struct RestSpec {
    written: (i64, i64),
    staff: u8,
    label: &'static str,
}

const STEPS: [&str; 7] = ["C", "D", "E", "F", "G", "A", "B"];

fn note_type(duration: i64) -> &'static str {
    match duration {
        16 => "whole",
        12 | 8 => "half",
        6 | 4 => "quarter",
        3 | 2 => "eighth",
        _ => "16th",
    }
}

fn synthetic_xml(scope: usize, notes: &[Spec], rests: &[RestSpec]) -> String {
    let measure = format!("{scope}");
    let mut body = String::new();
    let mut cursor = 0_i64;
    let mut lane = |body: &mut String, start: i64, end: i64, element: String| {
        if cursor > 0 {
            body.push_str(&format!("<backup><duration>{cursor}</duration></backup>"));
        }
        if start > 0 {
            body.push_str(&format!("<forward><duration>{start}</duration></forward>"));
        }
        body.push_str(&element);
        cursor = end;
    };
    for spec in notes {
        let (start, end) = spec.written;
        let duration = end - start;
        let step = STEPS[spec.midi_step % 7];
        let octave = 3 + spec.midi_step / 7;
        let element = format!(
            "<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>1</voice><type>{}</type><staff>{}</staff></note>",
            note_type(duration),
            spec.staff
        );
        lane(&mut body, start, end, element);
    }
    for rest in rests {
        let (start, end) = rest.written;
        let duration = end - start;
        let element = format!(
            "<note><rest/><duration>{duration}</duration><voice>{}</voice><type>{}</type><staff>{}</staff></note>",
            rest.label,
            note_type(duration),
            rest.staff
        );
        lane(&mut body, start, end, element);
    }
    let beats = notes.iter().map(|note| note.written.1).chain(rests.iter().map(|rest| rest.written.1)).max().unwrap_or(16).max(16) / 4;
    format!(
        r#"<score-partwise><part id="synthetic"><measure number="{measure}"><attributes><divisions>4</divisions><time><beats>{beats}</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>{body}</measure></part></score-partwise>"#
    )
}

struct Case {
    canonical: CanonicalFinalChordStructuralPlan,
    attacks: Vec<FrozenVoiceAttackEvidenceRow>,
}

fn case(notes: &[Spec], rests: &[RestSpec]) -> Case {
    let xml = synthetic_xml(83, notes, rests);
    let evidence = final_chord_declassified_evidence_from_xml(&xml).unwrap();
    assert_eq!(evidence.len(), notes.len());
    let (canonical, _) = plan_canonical_final_chord_structure_once(&xml, &evidence).unwrap();
    let attacks = evidence
        .iter()
        .zip(notes)
        .map(|(row, spec)| FrozenVoiceAttackEvidenceRow {
            source_event_id: row.source_event_id,
            attack_id: row.attack_id,
            absolute_start: spec.sounding.0,
            absolute_end: spec.sounding.1,
            staff: spec.staff,
            technical_lane: spec.lane,
        })
        .collect();
    Case { canonical, attacks }
}

fn primary(staff: u8) -> Spec {
    Spec { written: (0, 16), sounding: (0, 16), staff, lane: 1, midi_step: 0 }
}

fn secondary(index: usize, written: (i64, i64), sounding: (i64, i64)) -> Spec {
    Spec { written, sounding, staff: 1, lane: 2, midi_step: 8 + index }
}

/// Greedy needs four secondary channels, three suffice (found offline by
/// exhaustive search over sounding/written interval pairs; see solver notes).
fn crown_case_specs() -> Vec<Spec> {
    let written = [(8, 12), (0, 8), (0, 2), (4, 12), (8, 12), (4, 6)];
    let sounding = [(14, 15), (3, 5), (6, 7), (2, 3), (1, 2), (13, 16)];
    let mut specs = vec![primary(1)];
    specs.extend((0..6).map(|index| secondary(index, written[index], sounding[index])));
    specs
}

fn overlapping_case_specs(secondaries: usize) -> Vec<Spec> {
    let mut specs = vec![primary(1)];
    specs.extend((0..secondaries).map(|index| secondary(index, (0, 4), (0, 2))));
    specs
}

fn conflict(left: &Spec, right: &Spec) -> bool {
    let overlap = |a: (i64, i64), b: (i64, i64)| a.0 < b.1 && b.0 < a.1;
    left.staff == right.staff && (overlap(left.sounding, right.sounding) || overlap(left.written, right.written))
}

/// Independent emulation of MuseScore 4.7.4 pass 1 counting and
/// `allocateStaves` for single-measure synthetic input. Returns the number of
/// notes (pitched or rest) without an allocated voice.
fn musescore_lost(xml: &str) -> usize {
    let measures = final_chord_parse_binding_measures(xml).unwrap();
    #[derive(Default, Clone)]
    struct Desc {
        cr: [usize; 2],
        intervals: [Vec<(i64, i64)>; 2],
        overlaps: bool,
        staff: Option<usize>,
        staff_alloc: [Option<bool>; 2],
    }
    let mut voices = std::collections::BTreeMap::<u64, Desc>::new();
    for measure in &measures {
        for note in &measure.notes {
            if note.chord_member || note.duration <= 0 {
                continue;
            }
            let voice = note.voice.trim().parse::<u64>().unwrap();
            let staff = note.staff.trim().parse::<usize>().unwrap() - 1;
            let desc = voices.entry(voice).or_default();
            desc.cr[staff] += 1;
            desc.intervals[staff].push((note.onset, note.onset + note.duration));
        }
    }
    for desc in voices.values_mut() {
        desc.overlaps = desc.intervals[0]
            .iter()
            .any(|a| desc.intervals[1].iter().any(|b| a.0 < b.1 && b.0 < a.1));
    }
    let mut allocated = [0usize; 4];
    for _ in 0..voices.len() {
        let mut best: Option<(usize, u64)> = None;
        for (&key, desc) in &voices {
            let total = desc.cr[0] + desc.cr[1];
            if !desc.overlaps && desc.staff.is_none() && total > best.map_or(0, |b| b.0) {
                best = Some((total, key));
            }
        }
        let Some((_, key)) = best else { break };
        let desc = voices.get_mut(&key).unwrap();
        let preferred = usize::from(desc.cr[1] > desc.cr[0]);
        if allocated[preferred] < 4 {
            desc.staff = Some(preferred);
            allocated[preferred] += 1;
        }
        // else: setStaff(-2) is a no-op; the voice is re-selected forever.
    }
    for (staff, slots) in allocated.iter_mut().enumerate().take(2) {
        let mut order = voices
            .iter()
            .filter(|(_, desc)| desc.overlaps && desc.cr[staff] > 0)
            .map(|(&key, desc)| (std::cmp::Reverse(desc.cr[staff]), key))
            .collect::<Vec<_>>();
        order.sort();
        for (_, key) in order {
            let desc = voices.get_mut(&key).unwrap();
            desc.staff_alloc[staff] = Some(*slots < 4);
            if *slots < 4 {
                *slots += 1;
            }
        }
    }
    let mut lost = 0;
    for measure in &measures {
        for note in &measure.notes {
            // Pitched notes and rests both need an allocated voice.
            let staff = note.staff.trim().parse::<usize>().unwrap() - 1;
            let voice = note.voice.trim().parse::<u64>().unwrap();
            match voices.get(&voice) {
                Some(desc) if desc.overlaps => {
                    if desc.staff_alloc[staff] != Some(true) {
                        lost += 1;
                    }
                }
                Some(desc) => lost += usize::from(desc.staff.is_none()),
                None => lost += 1,
            }
        }
    }
    lost
}

fn relabeled_written_xml(case: &Case, channels: &FrozenSerializationVoiceChannelPlan) -> String {
    let labels = bind_musicxml_voice_labels(channels, &[]).unwrap();
    let written = case.canonical.written_projection.as_ref().unwrap();
    let measures = final_chord_parse_binding_measures(&written.xml).unwrap();
    let mut edits = Vec::new();
    for row in written.occurrences.iter() {
        let note = measures
            .iter()
            .flat_map(|measure| &measure.notes)
            .find(|note| note.range.start == row.projected_note_start)
            .unwrap();
        let source = &written.xml[note.range.clone()];
        let label = labels.label_for_source(row.source_event_id).unwrap();
        edits.push((note.range.start, note.range.end, final_chord_set_child_text(source, "voice", label).unwrap()));
    }
    final_chord_project_ranges(&written.xml, edits, "test relabel drift").unwrap()
}

fn assert_existing_constraints_hold(specs: &[Spec], case: &Case, channels: &FrozenSerializationVoiceChannelPlan) {
    for (left_index, left) in specs.iter().enumerate() {
        let left_source = case.attacks[left_index].source_event_id;
        let left_role = final_chord_default_presentation_role(left.staff, left.lane).unwrap();
        assert_eq!(channels.role_for_source(left_source), Some(left_role));
        assert_eq!(channels.channel_to_role[&channels.source_to_channel[&left_source]], left_role);
        // Staff preservation: a channel never presents on another staff than
        // the written staff of each of its members.
        assert_eq!(channel_role_staff(left_role), left.staff);
        for (right_index, right) in specs.iter().enumerate().skip(left_index + 1) {
            let right_source = case.attacks[right_index].source_event_id;
            let right_role = final_chord_default_presentation_role(right.staff, right.lane).unwrap();
            if conflict(left, right) || left_role != right_role {
                assert_ne!(
                    channels.source_to_channel[&left_source],
                    channels.source_to_channel[&right_source],
                    "an existing SemanticDifferent/WrittenDifferent/role edge was relaxed"
                );
            }
        }
    }
}

#[test]
fn simple_partition_is_admitted_as_first_candidate_and_unchanged() {
    let specs = vec![primary(1), primary(2), secondary(0, (0, 4), (0, 1)), secondary(1, (4, 8), (1, 2))];
    let mut specs = specs;
    specs[1].midi_step = 3;
    let case = case(&specs, &[]);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    assert_eq!(channels.consumer_allocation(), &ChannelConsumerAllocation::AdmittedFirstCandidate);
    assert_eq!(channels.channel_count(), 3);
    assert_existing_constraints_hold(&specs, &case, &channels);
    assert_eq!(musescore_lost(&relabeled_written_xml(&case, &channels)), 0);
}

#[test]
fn greedy_overflow_is_repaired_by_search_under_unchanged_edges_and_roles() {
    let specs = crown_case_specs();
    let case = case(&specs, &[]);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    assert_eq!(
        channels.consumer_allocation(),
        &ChannelConsumerAllocation::AdmittedAfterSearch { channels_before: 5, channels_after: 4 }
    );
    assert_eq!(channels.channel_count(), 4);
    assert_existing_constraints_hold(&specs, &case, &channels);
    assert_eq!(musescore_lost(&relabeled_written_xml(&case, &channels)), 0);
}

#[test]
fn greedy_first_candidate_loses_notes_in_the_consumer_emulation() {
    // Mutation control for the emulator: the five-channel greedy partition is
    // exactly what the repaired case rejects, and the emulator sees the loss.
    let specs = crown_case_specs();
    let case = case(&specs, &[]);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    let mut greedy = channels.clone();
    // Rebuild the greedy partition independently: first fit in the owner's
    // class order (sounding start, then end) under the same conflict edges.
    let mut order = (1..specs.len()).collect::<Vec<_>>();
    order.sort_by_key(|&index| (specs[index].sounding.0, specs[index].sounding.1));
    let mut colour = std::collections::BTreeMap::<usize, usize>::new();
    for &index in &order {
        let used = colour
            .iter()
            .filter(|(other, _)| conflict(&specs[index], &specs[**other]))
            .map(|(_, colour)| *colour)
            .collect::<std::collections::BTreeSet<_>>();
        colour.insert(index, (0..).find(|candidate| !used.contains(candidate)).unwrap());
    }
    assert_eq!(colour.values().collect::<std::collections::BTreeSet<_>>().len(), 4);
    for (&index, &value) in &colour {
        greedy.source_to_channel.insert(case.attacks[index].source_event_id, 10 + value);
        greedy.channel_to_role.insert(10 + value, SerializedVoicePresentationRole::UpperSecondary);
    }
    greedy.channel_to_role.retain(|channel, _| greedy.source_to_channel.values().any(|used| used == channel));
    let lost = musescore_lost(&relabeled_written_xml(&case, &greedy));
    assert!(lost > 0);
}

#[test]
fn true_slot_obligation_yields_clique_witness_and_keeps_first_candidate() {
    let specs = overlapping_case_specs(4);
    let case = case(&specs, &[]);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    match channels.consumer_allocation() {
        ChannelConsumerAllocation::SlotObligationWitness { staff, demand, cliques } => {
            assert_eq!((*staff, *demand), (1, 5));
            assert_eq!(cliques.iter().map(|(_, members)| members.len()).sum::<usize>(), 5);
        }
        other => panic!("expected witness, got {other:?}"),
    }
    assert_eq!(channels.channel_count(), 5);
    assert_existing_constraints_hold(&specs, &case, &channels);
    let lost = musescore_lost(&relabeled_written_xml(&case, &channels));
    assert!(lost > 0, "the witness must correspond to a real consumer loss");
}

#[test]
fn staff_hits_alone_do_not_make_a_witness_when_a_free_label_may_join() {
    let specs = overlapping_case_specs(4);
    let rests = [RestSpec { written: (0, 16), staff: 2, label: "9" }];
    let case = case(&specs, &rests);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    assert!(matches!(channels.consumer_allocation(), ChannelConsumerAllocation::Undetermined(_)));
    assert_eq!(channels.channel_count(), 5);
    assert_existing_constraints_hold(&specs, &case, &channels);
}

#[test]
fn reserved_rest_label_joins_its_owner_without_a_role() {
    let specs = overlapping_case_specs(3);
    let rests = [RestSpec { written: (4, 8), staff: 1, label: "1" }];
    let case = case(&specs, &rests);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    assert_eq!(channels.consumer_allocation(), &ChannelConsumerAllocation::AdmittedFirstCandidate);
    assert_eq!(musescore_lost(&relabeled_written_xml(&case, &channels)), 0);
}

#[test]
fn unreserved_rest_label_is_a_technical_demand_and_never_a_role() {
    let specs = overlapping_case_specs(3);
    let rests = [RestSpec { written: (4, 8), staff: 1, label: "8" }];
    let case = case(&specs, &rests);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    // Primary + three pairwise conflicting secondaries + the rest's own voice
    // exceed four; no search can remove the unchanged rest bytes, and the free
    // label blocks a witness on its own staff only through demand, not roles.
    assert!(matches!(channels.consumer_allocation(), ChannelConsumerAllocation::Undetermined(_)));
    assert_eq!(channels.channel_count(), 4);
    assert!(channels.channel_to_role.len() == 4);
    let lost = musescore_lost(&relabeled_written_xml(&case, &channels));
    assert!(lost > 0);
}

#[test]
fn search_result_is_invariant_under_input_order_and_opaque_renaming() {
    let specs = crown_case_specs();
    let first = case(&specs, &[]);
    let mut second = case(&specs, &[]);
    let rename = |value: usize| 1009 + value * 17;
    for row in std::sync::Arc::make_mut(&mut second.canonical.source_locators) {
        row.source_event_id = rename(row.source_event_id); row.attack_id = rename(row.attack_id); row.fragment_id = rename(row.fragment_id);
    }
    for row in std::sync::Arc::make_mut(&mut second.canonical.written_projection.as_mut().unwrap().occurrences) {
        row.source_event_id = rename(row.source_event_id); row.attack_id = rename(row.attack_id); row.fragment_id = rename(row.fragment_id);
    }
    for row in &mut second.attacks { row.source_event_id = rename(row.source_event_id); row.attack_id = rename(row.attack_id); }
    second.attacks.reverse();
    let mut locators = second.canonical.source_locators.to_vec();
    locators.reverse();
    second.canonical.source_locators = std::sync::Arc::from(locators);
    let first_channels = build_frozen_serialization_voice_channels_for_canonical(&first.canonical, &first.attacks).unwrap();
    let second_channels = build_frozen_serialization_voice_channels_for_canonical(&second.canonical, &second.attacks).unwrap();
    assert_eq!(first_channels.consumer_allocation(), second_channels.consumer_allocation());
    assert_eq!(first_channels.semantic_fingerprint_fnv64, second_channels.semantic_fingerprint_fnv64);
    for attack in &first.attacks {
        assert_eq!(first_channels.source_to_channel[&attack.source_event_id], second_channels.source_to_channel[&rename(attack.source_event_id)]);
    }
}

#[test]
fn unreadable_written_evidence_keeps_the_first_candidate() {
    let (canonical, attacks) = tests_support_unreadable_case();
    let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
    assert!(matches!(channels.consumer_allocation(), ChannelConsumerAllocation::Undetermined(_)));
}

fn tests_support_unreadable_case() -> (CanonicalFinalChordStructuralPlan, Vec<FrozenVoiceAttackEvidenceRow>) {
    let specs = crown_case_specs();
    let mut case = case(&specs, &[]);
    let written = case.canonical.written_projection.as_mut().unwrap();
    written.xml = std::sync::Arc::from("synthetic owner-local evidence");
    (case.canonical, case.attacks)
}

#[test]
fn hidden_cursor_only_notes_are_not_required_and_do_not_register() {
    let model = ChannelConsumerModel {
        notes: vec![
            ChannelConsumerNote { staff: 1, chordrest: false, required: false, class: Some(1), label: "1".into() },
            ChannelConsumerNote { staff: 1, chordrest: true, required: true, class: Some(2), label: "1".into() },
        ],
    };
    let roles = [(1, SerializedVoicePresentationRole::UpperPrimary), (2, SerializedVoicePresentationRole::UpperSecondary)]
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
    let by_class = [(1, 1), (2, 2)].into_iter().collect::<std::collections::BTreeMap<_, _>>();
    assert!(channel_consumer_overflow(&model, &by_class, &roles).is_ok());
    let mut required = model.clone();
    required.notes[0].required = true;
    // A required note whose voice registers no pitched chordrest is unreserved.
    assert_eq!(channel_consumer_overflow(&required, &by_class, &roles), Err([1].into_iter().collect()));
}

#[test]
fn unknown_lexical_and_numeric_alias_keys_are_not_admission_evidence() {
    for raw in ["01", "voice", "0", "-1", "1.0"] { assert!(channel_consumer_label_key(raw).is_none()); }
    assert_eq!(channel_consumer_label_key("23"), Some("23".into()));
    let specs = crown_case_specs();
    let mut case = case(&specs, &[]);
    let written = case.canonical.written_projection.as_mut().unwrap();
    written.xml = std::sync::Arc::from(written.xml.replacen("<voice>1</voice>", "<voice>0</voice>", 1));
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    assert!(matches!(channels.consumer_allocation(), ChannelConsumerAllocation::Undetermined(_)));
}

#[test]
fn a_head_on_the_other_staff_is_not_an_own_staff_registration_guarantee() {
    let model = ChannelConsumerModel { notes: vec![
        ChannelConsumerNote { staff: 1, chordrest: true, required: true, class: Some(1), label: "1".into() },
        ChannelConsumerNote { staff: 2, chordrest: false, required: true, class: Some(1), label: "1".into() },
    ] };
    let roles = [(1, SerializedVoicePresentationRole::UpperPrimary)].into_iter().collect();
    let classes = [(1, 1)].into_iter().collect();
    assert_eq!(channel_consumer_overflow(&model, &classes, &roles), Err([2].into_iter().collect()));
}

#[test]
fn independent_temporal_copies_preserve_the_recoloured_local_partitions() {
    let base = crown_case_specs();
    let baseline = case(&base, &[]);
    let baseline_channels = build_frozen_serialization_voice_channels_for_canonical(&baseline.canonical, &baseline.attacks).unwrap();
    for copies in [1, 2, 5] {
        let specs = (0..copies).flat_map(|copy| base.iter().map(move |original| {
            let offset = copy as i64 * 16;
            Spec { written: (original.written.0 + offset, original.written.1 + offset),
                sounding: (original.sounding.0 + offset, original.sounding.1 + offset), ..*original }
        })).collect::<Vec<_>>();
        let case = case(&specs, &[]);
        let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
        assert_eq!(channels.consumer_allocation(), baseline_channels.consumer_allocation());
        assert_existing_constraints_hold(&specs, &case, &channels);
        for copy in 0..copies {
            for left in 0..base.len() { for right in 0..base.len() {
                let a = baseline.attacks[left].source_event_id; let b = baseline.attacks[right].source_event_id;
                let x = case.attacks[copy * base.len() + left].source_event_id;
                let y = case.attacks[copy * base.len() + right].source_event_id;
                assert_eq!(baseline_channels.source_to_channel[&a] == baseline_channels.source_to_channel[&b],
                    channels.source_to_channel[&x] == channels.source_to_channel[&y]);
            } }
        }
    }
}

#[test]
fn productive_consumer_seal_refuses_overflow_even_when_diagnostic_plan_is_bound() {
    for secondary_count in [3, 4] {
        let specs = overlapping_case_specs(secondary_count);
        let case = case(&specs, &[]);
        let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
        let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
        assert_eq!(channels.require_consumer_admission(&case.canonical, &voices).is_ok(), secondary_count == 3);
        if secondary_count == 4 {
            assert!(musescore_lost(&relabeled_written_xml(&case, &channels)) > 0);
        }
    }
}

#[test]
fn productive_consumer_seal_checks_actual_rest_labels_and_unreadable_evidence() {
    let specs = overlapping_case_specs(3);
    let case = case(&specs, &[RestSpec { written: (4, 8), staff: 1, label: "8" }]);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
    assert!(channels.require_consumer_admission(&case.canonical, &voices).is_err());
    let (canonical, attacks) = tests_support_unreadable_case();
    let channels = build_frozen_serialization_voice_channels_for_canonical(&canonical, &attacks).unwrap();
    let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
    assert!(channels.require_consumer_admission(&canonical, &voices).is_err());
}

#[test]
fn productive_consumer_seal_rejects_foreign_binding_and_admits_repaired_partition() {
    let specs = crown_case_specs();
    let case = case(&specs, &[]);
    let channels = build_frozen_serialization_voice_channels_for_canonical(&case.canonical, &case.attacks).unwrap();
    let voices = bind_musicxml_voice_labels(&channels, &[]).unwrap();
    assert!(channels.require_consumer_admission(&case.canonical, &voices).is_ok());
    let mut foreign = voices.clone();
    foreign.source_to_channel.remove(&case.attacks[0].source_event_id);
    assert!(channels.require_consumer_admission(&case.canonical, &foreign).is_err());
    assert!(channels.require_consumer_admission(&case.canonical, &FrozenSerializedVoicePlan::default()).is_err());
}
