use super::*;
use crate::arrangement::OwnedRhythmicContinuation;
use crate::master_state_identity::{MasterLineage, OwnerReceipts};
use crate::structural_write_authority::MasterAdoptedState;

fn received_master(rows: &[(usize, i64, u8)], measures: usize)
    -> (MasterAdoptedState, OwnedRhythmicContinuation) {
    let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture(rows, measures);
    let arrangement = crate::arrangement::arrange_piano_musicxml_with_profile(&xml,
        crate::arrangement::ArrangementProfile { rhythmic_courses: Some(&seed),
            preserve_reference: true, ..Default::default() });
    let state = OwnedRhythmicContinuation::receive(
        arrangement.stats.rhythmic_courses.as_ref().unwrap(), &arrangement.xml).unwrap();
    let mut master = MasterAdoptedState::adopt_genesis_state(arrangement.xml,
        MasterLineage::lineage_from_genesis(arrangement.arrangement_source_genesis.unwrap()));
    let before = master.document().to_owned();
    let normalized = crate::arrangement::normalize_visible_rhythm_musicxml(&before);
    let mut receipts = OwnerReceipts::default();
    receipts.push_visible_block_with_births(normalized.visible_rhythm_block_identity,
        normalized.stats.physical_births.first().cloned(), &before).unwrap();
    let state = receipts.continue_owned_rhythm(&state, &normalized.xml).unwrap();
    master.adopt_materialized_document(normalized.xml, receipts, "SyntheticOriginalNormalizer");
    state.require_complete_course_context(master.document()).unwrap();
    master.observe_selected_custody("SyntheticEndpointCleanup", Some(std::sync::Arc::new(state.clone())), None);
    (master, state)
}

// Offbeat fixtures require actual metric ties even when whole-bar tones are preserved.
fn hidden_tied_master(rows: &[(usize, i64, u8)], measures: usize)
    -> (MasterAdoptedState, OwnedRhythmicContinuation) {
    let (mut master, state) = received_master(rows, measures);
    let before = master.document().to_owned();
    let doc = roxmltree::Document::parse(&before).unwrap();
    let range = doc.descendants().find(|node| node.has_tag_name("note")
        && node.children().any(|child| child.has_tag_name("pitch"))
        && node.descendants().any(|child| child.has_tag_name("tied")))
        .expect("normalizer fixture must contain a real tied fragment").range();
    let mut after = before.clone();
    let (hidden, receipt) = hide_note_print_object_range(&mut after, range.start, range.end);
    assert!(hidden);
    let mut receipts = OwnerReceipts::default(); receipts.push_exact(receipt);
    let state = receipts.continue_owned_rhythm(&state, &after).unwrap();
    master.adopt_materialized_document(after, receipts, "SyntheticOriginalVisibilityOwner");
    (master, state)
}

fn sounding_ties(xml: &str) -> Vec<String> {
    let doc = roxmltree::Document::parse(xml).unwrap();
    doc.descendants().filter(|node| node.has_tag_name("tie"))
        .map(|node| xml[node.range()].to_owned()).collect()
}

#[test]
fn actual_orphan_cleanup_reaches_original_master_adoption_with_retained_custody() {
    for rows in [vec![(7, 24, 1)], vec![(91, 24, 1), (53, 408, 2)]] {
        let (mut master, state) = hidden_tied_master(&rows, 2);
        let before = master.document().to_owned();
        let mut after = before.clone();
        let ((_, removed_tied), receipts) = remove_orphan_slur_and_tie_endpoints(&mut after);
        assert!(removed_tied > 0, "actual orphan detector must reach the original writer");
        assert!(receipts.len() >= 2, "endpoint deletion and empty-wrapper removal are separate original receipts");
        assert_ne!(after, before);
        assert_eq!(sounding_ties(&after), sounding_ties(&before));
        let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
        assert!(next.same_identities_as(&state));
        next.require_complete_course_context(&after).unwrap();
        master.adopt_materialized_document(after.clone(), receipts, "SyntheticActualOrphanCleanup");
        assert_eq!(master.document(), after);
        assert!(master.lineage().lineage_continuity_holds(&after));
        let (counts, noop) = remove_orphan_slur_and_tie_endpoints(&mut after);
        assert_eq!(counts, (0, 0)); assert_eq!(noop.len(), 0);
        assert!(noop.continue_owned_rhythm(&next, &after).unwrap().same_identities_as(&next));
    }
}

#[test]
fn valid_cleanup_custody_never_replaces_dropped_original_master_receipts() {
    let (mut master, state) = hidden_tied_master(&[(17, 24, 1)], 1);
    let before = master.document().to_owned(); let mut after = before.clone();
    let ((_, removed), receipts) = remove_orphan_slur_and_tie_endpoints(&mut after);
    assert!(removed > 0);
    receipts.continue_owned_rhythm(&state, &after).unwrap();
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        master.adopt_materialized_document(after, OwnerReceipts::default(), "SyntheticDroppedEndpointReceipts");
    }));
    assert!(refusal.is_err()); assert_eq!(master.document(), before);
    assert!(master.lineage().lineage_continuity_holds(&before));
}

#[test]
fn actual_endpoint_cleanup_noop_preserves_empty_course_context() {
    let (mut master, state) = received_master(&[], 2);
    let before = master.document().to_owned(); let mut after = before.clone();
    let (counts, receipts) = remove_orphan_slur_and_tie_endpoints(&mut after);
    assert_eq!(counts, (0, 0)); assert_eq!(after, before);
    let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
    assert!(next.same_identities_as(&state)); next.require_complete_course_context(&after).unwrap();
    master.adopt_materialized_document(after, receipts, "SyntheticEmptyEndpointCleanup");
    assert!(master.lineage().lineage_continuity_holds(&before));
}

fn slur_scan_support(notes: &[NotationEndpointNote], ordinal: usize, number: &str) -> (bool, bool) {
    let earlier = notes.iter().take(ordinal).any(|other| other.visible && !other.rest
        && other.slurs.iter().any(|tag| tag.number == number && tag.type_attr == "start"));
    let later = notes.iter().skip(ordinal + 1).any(|other| other.visible && !other.rest
        && other.slurs.iter().any(|tag| tag.number == number && tag.type_attr == "stop"));
    (earlier, later)
}

fn assert_scan_equivalent_slur_cleanup(before: &str) -> String {
    let notes = collect_notation_endpoint_notes(before);
    let mut ranges = Vec::new();
    for (position, observed) in notes.iter().enumerate() {
        for tag in &observed.slurs {
            let (earlier, later) = slur_scan_support(&notes, position, &tag.number);
            let paired = match tag.type_attr.as_str() { "start" => later, "stop" => earlier, _ => false };
            if !observed.visible || observed.rest || !paired { ranges.push((tag.abs_start, tag.abs_end)); }
        }
    }
    let mut expected = before.to_owned();
    let _ = remove_xml_ranges(&mut expected, &ranges);
    let _ = remove_empty_notations_blocks(&mut expected);
    let mut actual = before.to_owned();
    let (counts, _) = remove_orphan_slur_and_tie_endpoints(&mut actual);
    assert_eq!(counts, (ranges.len(), 0)); assert_eq!(actual, expected);
    assert_eq!(sounding_ties(&actual), sounding_ties(before));
    actual
}

#[test]
fn slur_extrema_equal_full_scans_for_all_small_observation_sequences() {
    let patterns = ["", "<slur type='start' number='1'/>", "<slur type='stop' number='1'/>",
        "<slur type='continue' number='1'/>",
        "<slur type='start' number='1'/><slur type='stop' number='1'/><slur type='stop' number='1'/>",
        "<slur type='start' number='2'/><slur type='stop' number='1'/>"];
    let pitch = "<pitch><step>C</step><octave>3</octave></pitch>";
    let flags = [("", pitch), (" print-object='no'", pitch), ("", "<rest/>")];
    let note = |opening: &str, payload: &str, staff: u8, voice: u8, tags: &str| format!(
        "<note{opening}>{payload}<duration>96</duration><tie type='start'/><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff><notations>{tags}</notations></note>");
    let prefix = format!("{}{}", note("", pitch, 3, 9, "<slur type='start' number='separate'/>"),
        note("", pitch, 3, 9, "<slur type='stop' number='separate'/>"));
    let open = "<score-partwise><part id='synthetic'><measure number='alpha'>";
    let close = "</measure></part></score-partwise>";
    for a in 0..18 {
        for b in 0..18 {
            for c in 0..18 {
                let (ao, ap) = flags[a / 6]; let (bo, bp) = flags[b / 6]; let (co, cp) = flags[c / 6];
                let body = format!("{}{}{}", note(ao, ap, 1, 2, patterns[a % 6]),
                    note(bo, bp, 2, 7, patterns[b % 6]), note(co, cp, 1, 2, patterns[c % 6]));
                let base = assert_scan_equivalent_slur_cleanup(&format!("{open}{body}{close}"));
                let extended = assert_scan_equivalent_slur_cleanup(&format!("{open}{prefix}{body}{close}"));
                let base_body = base.strip_prefix(open).unwrap().strip_suffix(close).unwrap();
                assert_eq!(extended, format!("{open}{prefix}{base_body}{close}"));
            }
        }
    }
    assert_eq!(assert_scan_equivalent_slur_cleanup(&format!("{open}{close}")), format!("{open}{close}"));
}

#[test]
fn indexed_slur_cleanup_retains_actual_xml_reader_semantics() {
    let note = |opening: &str, payload: &str, tags: &str| format!(
        "<note{opening}>{payload}<duration>96</duration><tie type='start'/><voice>2</voice><type>quarter</type><staff>2</staff><notations>{tags}</notations></note>");
    let pitch = "<pitch><step>C</step><octave>3</octave></pitch>";
    let start = note("", pitch, "<slur type='start' number='1'/>");
    let stop = note("", pitch, "<slur type='stop' number='1'/>");
    for middle in [stop.clone(), note(" print-object='no'", pitch, "<slur type='stop' number='1'/>"),
        note("", "<rest/>", "<slur type='stop' number='1'/>"),
        note("", pitch, "<slur type='continue' number='1'/><slur type='stop' number='2'/>"),
        note("", pitch, "<slur type='stop'/>"), format!("<!-- {stop} -->"), String::new()] {
        for ordinal in [29, 113] {
            let before = format!("<score-partwise><part id='synthetic'><measure number='{ordinal}'>{start}{middle}</measure><measure number='{}'>{start}</measure></part></score-partwise>", ordinal + 5);
            let _ = assert_scan_equivalent_slur_cleanup(&before);
        }
    }
}

// Independent bounded orchestration reference. All discovery, writes and
// receipts still come from the original productive owners.
fn bounded_guard_reference(xml: &mut String) -> (OwnerReceipts, [usize; 3]) {
    let mut receipts = OwnerReceipts::default();
    let mut changed = [0; 3];
    receipts.append(hide_exact_duplicate_visible_noteheads(xml).1);
    receipts.append(hide_redundant_visible_staff_rests(xml).1);
    for _ in 0..4 {
        let ((slurs, ties), endpoints) = remove_orphan_slur_and_tie_endpoints(xml);
        receipts.append(endpoints);
        let (rests, rest_receipt) = prune_rest_only_staff2_clef_noops(xml);
        receipts.push_exact(rest_receipt);
        let ((empty, trailing, redundant, advance), clef_receipt) = prune_redundant_lower_staff_clefs(xml);
        receipts.push_exact(clef_receipt);
        changed[0] += slurs + ties;
        changed[1] += rests;
        changed[2] += empty + trailing + redundant + advance;
        if slurs + ties + rests + empty + trailing + redundant + advance == 0 { break; }
    }
    (receipts, changed)
}

fn assert_guard_owner_closure(before: &str) -> [usize; 3] {
    let mut expected = before.to_owned();
    let (reference, changed) = bounded_guard_reference(&mut expected);
    let mut actual = before.to_owned();
    let receipts = apply_final_notation_polish_guard(&mut actual);
    assert_eq!(actual, expected, "single owner visits must equal the bounded composition");
    assert_eq!(receipts.len(), reference.len(), "all original nonidentity receipts must remain");
    assert_eq!(sounding_ties(&actual), sounding_ties(before));
    let closed = actual.clone();
    let (endpoints, endpoint_receipts) = remove_orphan_slur_and_tie_endpoints(&mut actual);
    assert_eq!(endpoints, (0, 0)); assert_eq!(endpoint_receipts.len(), 0);
    let (rests, rest_receipt) = prune_rest_only_staff2_clef_noops(&mut actual);
    assert_eq!(rests, 0); assert!(rest_receipt.is_none());
    let (clefs, clef_receipt) = prune_redundant_lower_staff_clefs(&mut actual);
    assert_eq!(clefs, (0, 0, 0, 0)); assert!(clef_receipt.is_none());
    assert_eq!(actual, closed);
    assert_eq!(apply_final_notation_polish_guard(&mut actual).len(), 0);
    assert_eq!(actual, closed);
    changed
}

#[test]
fn final_guard_owner_chain_matches_bounded_composition_and_is_closed() {
    let clef = |sign: &str| format!("<attributes><clef number=\"2\"><sign>{sign}</sign><line>4</line></clef></attributes>");
    let f = clef("F"); let g = clef("G");
    let pitch = "<pitch><step>C</step><octave>3</octave></pitch>";
    let note = |opening: &str, body: &str, staff: usize, endpoints: &str| format!(
        "<note{opening}>{body}<duration>96</duration><tie type=\"start\"/><voice>1</voice><type>quarter</type><staff>{staff}</staff><notations>{endpoints}</notations></note>");
    let start = note("", pitch, 2, "<slur type=\"start\"/><tied type=\"start\"/>");
    let stop = note("", pitch, 2, "<slur type=\"stop\"/><tied type=\"stop\"/>");
    let hidden = note(" print-object=\"no\"", pitch, 2, "<slur type=\"stop\"/><tied type=\"stop\"/>");
    let rest = note("", "<rest/>", 2, "<slur type=\"start\"/>");
    let upper = note("", pitch, 1, "");
    let patterns = [String::new(), f.clone(), g.clone(), format!("{f}{start}"),
        format!("{g}{start}{stop}"), format!("{g}{rest}{f}"),
        format!("{g}{hidden}{upper}{f}"),
        format!("{g}{start}{f}{stop}<backup><duration>96</duration></backup>{rest}<forward><duration>96</duration></forward>{g}")];
    let mut totals = [0; 3];
    for initial in [&f, &g] {
        for a in &patterns { for b in &patterns { for c in &patterns {
            for numbers in [[1, 2, 3], [103, 17, 82]] {
                let [first, second, third] = numbers;
                let body = format!("<measure number=\"{first}\">{initial}{a}</measure><measure number=\"{second}\">{b}</measure><measure number=\"{third}\">{c}</measure>");
                let score = format!("<score-partwise><part id=\"synthetic\">{body}</part></score-partwise>");
                let changed = assert_guard_owner_closure(&score);
                for (total, count) in totals.iter_mut().zip(changed) { *total += count; }
            }
        } } }
    }
    assert!(totals.iter().all(|count| *count > 0), "each original cleanup family must actually write");
    // A long silent clef chain must close in one visit, independently of the
    // historical outer bound. Separate renamed parts exercise relocation.
    let chain = format!("{g}{rest}{f}{hidden}").repeat(24);
    let component = |offset: usize| {
        let first = offset + 1; let second = offset + 2;
        format!("<measure number=\"{first}\">{f}{chain}</measure><measure number=\"{second}\">{g}{start}{stop}</measure>")
    };
    let first = component(0); let relocated = component(100);
    let score = format!("<score-partwise><part id=\"first\">{first}</part><part id=\"second\">{relocated}</part></score-partwise>");
    let _ = assert_guard_owner_closure(&score);
    let _ = assert_guard_owner_closure("<score-partwise/>");
}

#[test]
fn final_guard_composition_retains_original_receipts_through_master_adoption() {
    let (mut master, state) = hidden_tied_master(&[(41, 24, 1), (73, 408, 2)], 2);
    let before = master.document().to_owned();
    let mut expected = before.clone();
    let (reference, _) = bounded_guard_reference(&mut expected);
    let mut after = before.clone();
    let receipts = apply_final_notation_polish_guard(&mut after);
    assert_ne!(after, before); assert_eq!(after, expected);
    assert!(receipts.len() > 0); assert_eq!(receipts.len(), reference.len());
    let next = receipts.continue_owned_rhythm(&state, &after).unwrap();
    assert!(next.same_identities_as(&state));
    next.require_complete_course_context(&after).unwrap();
    master.adopt_materialized_document(after.clone(), receipts, "SyntheticFinalGuardOwnerComposition");
    assert_eq!(master.document(), after);
    assert!(master.lineage().lineage_continuity_holds(&after));
}

#[test]
fn final_guard_composition_refuses_adoption_without_original_receipts() {
    let (mut master, state) = hidden_tied_master(&[(83, 24, 1)], 1);
    let before = master.document().to_owned(); let mut after = before.clone();
    let receipts = apply_final_notation_polish_guard(&mut after);
    assert_ne!(after, before);
    receipts.continue_owned_rhythm(&state, &after).unwrap();
    let refusal = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        master.adopt_materialized_document(after, OwnerReceipts::default(), "SyntheticUnreceiptedFinalGuard");
    }));
    assert!(refusal.is_err()); assert_eq!(master.document(), before);
    assert!(master.lineage().lineage_continuity_holds(&before));
}

#[test]
fn exact_head_suppression_preserves_unequal_written_releases() {
    let note = |duration, voice| format!(
        "<note><pitch><step>G</step><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>1</staff></note>"
    );
    let shorter = note(48, 1);
    let longer = note(96, 1);
    let cursor = "<backup><duration>48</duration></backup>";
    let mut xml = format!(
        "<score-partwise><part id='synthetic'><measure>{shorter}{cursor}{shorter}{cursor}{longer}</measure></part></score-partwise>"
    );
    let (hidden, receipts) = hide_exact_duplicate_visible_noteheads(&mut xml);
    assert_eq!(hidden, 1);
    assert_eq!(receipts.len(), 1);
    let notes = parse_atomic_notes(&xml);
    assert_eq!(notes.len(), 3);
    assert_eq!(notes.iter().filter(|n| n.visible).map(|n| n.duration).collect::<Vec<_>>(), [48, 96]);
}
