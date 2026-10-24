include!("horizontal_layout_decision_plan.rs");
include!("horizontal_layout_fact_extraction_core.rs");
include!("horizontal_layout_global_reflow.rs");
include!("horizontal_layout_global_page_system.rs");
include!("horizontal_layout_page_count_preference.rs");

fn same_notation_stream(a: &NotationEndpointNote, b: &NotationEndpointNote) -> bool {
    a.staff.trim() == b.staff.trim() && a.voice.trim() == b.voice.trim()
}

fn next_local_tie_target(notes: &[NotationEndpointNote], idx: usize, midi: i32) -> Option<&NotationEndpointNote> {
    let source = notes.get(idx)?;
    let other = notes
        .iter()
        .skip(idx + 1)
        .find(|n| n.visible && !n.rest && same_notation_stream(source, n))?;
    (other.pitch_midi == Some(midi)).then_some(other)
}

fn previous_local_tie_source(notes: &[NotationEndpointNote], idx: usize, midi: i32) -> Option<&NotationEndpointNote> {
    let target = notes.get(idx)?;
    let other = notes
        .iter()
        .take(idx)
        .rev()
        .find(|n| n.visible && !n.rest && same_notation_stream(target, n))?;
    (other.pitch_midi == Some(midi)).then_some(other)
}

fn collect_notation_endpoint_notes(xml: &str) -> Vec<NotationEndpointNote> {
    let mut out = Vec::new();
    let mut scan = 0usize;
    while let Some(rel_start) = xml[scan..].find("<note") {
        let note_start = scan + rel_start;
        let Some(close_rel) = xml[note_start..].find("</note>") else { break; };
        let note_end = note_start + close_rel + "</note>".len();
        let block = &xml[note_start..note_end];
        let open_end = block.find('>').unwrap_or(0);
        let open_tag = &block[..=open_end.min(block.len().saturating_sub(1))];
        let visible = !(open_tag.contains("print-object=\"no\"") || open_tag.contains("print-object='no'"));
        let rest = block.contains("<rest") || !block.contains("<pitch");
        let staff = extract_first_text_from_fragment(block, "staff").unwrap_or_else(|| "1".to_string());
        let voice = extract_first_text_from_fragment(block, "voice").unwrap_or_else(|| "1".to_string());
        let pitch_midi = if rest { None } else { midi_from_note_fragment(block) };
        let mut slurs = collect_endpoint_tags(block, note_start, "slur");
        // FinalNotationPolishGuard is a representation owner. MusicXML <tie> elements
        // carry sounding/playback semantics and must never be removed by its visual
        // orphan-endpoint cleanup. Only notation-level <tied> endpoints are observed here.
        let mut ties = collect_endpoint_tags(block, note_start, "tied");
        // Ungueltige Nummern bei Slurs werden wie Nummer 1 behandelt; bei Ties
        // ist die Nummer fuer die Paarung nicht relevant, bleibt aber stabil.
        for tag in &mut slurs {
            if tag.number.is_empty() { tag.number = "1".to_string(); }
        }
        for tag in &mut ties {
            if tag.number.is_empty() { tag.number = "1".to_string(); }
        }
        out.push(NotationEndpointNote { visible, rest, staff, voice, pitch_midi, slurs, ties });
        scan = note_end;
    }
    out
}

fn collect_endpoint_tags(block: &str, abs_block_start: usize, tag_name: &'static str) -> Vec<NotationEndpointTag> {
    let mut out = Vec::new();
    let needle = format!("<{tag_name}");
    let mut scan = 0usize;
    while let Some(rel) = block[scan..].find(&needle) {
        let start = scan + rel;
        let next = block.as_bytes().get(start + needle.len()).copied();
        if !matches!(next, Some(b' ' | b'\t' | b'\n' | b'\r' | b'/' | b'>')) {
            scan = start + needle.len();
            continue;
        }
        let Some(end_rel) = block[start..].find('>') else { break; };
        let end = start + end_rel + 1;
        let tag = &block[start..end];
        let type_attr = parse_attr_value(tag, "type").unwrap_or_default();
        let number = parse_attr_value(tag, "number").unwrap_or_else(|| "1".to_string());
        out.push(NotationEndpointTag {
            type_attr,
            number,
            abs_start: abs_block_start + start,
            abs_end: abs_block_start + end,
        });
        scan = end;
    }
    out
}

#[cfg(test)]
mod notation_endpoint_sound_semantics_tests {
    use super::*;

    #[test]
    fn endpoint_reader_excludes_musicxml_sound_tie_from_visual_cleanup_set() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><tie type=\"start\"/><staff>1</staff><notations><tied type=\"start\"/></notations></note></measure></part></score-partwise>";
        let notes = collect_notation_endpoint_notes(xml);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].ties.len(), 1);
        let tag = &notes[0].ties[0];
        assert_eq!(&xml[tag.abs_start..tag.abs_end], r#"<tied type="start"/>"#);
    }
}

fn hide_exact_duplicate_visible_noteheads(
    xml: &mut String,
) -> (usize, master_state_identity::OwnerReceipts) {
    let notes = parse_atomic_notes(xml);
    let mut seen: BTreeSet<(String, String, i64, i32)> = BTreeSet::new();
    let mut duplicates: Vec<(usize, usize)> = Vec::new();
    for note in notes.iter().filter(|n| n.visible && !n.is_chord && n.pitch.is_some()) {
        let midi = note.pitch.as_ref().map_or(0, |p| p.midi);
        let key = (note.measure.clone(), note.staff.clone(), note.start, midi);
        if !seen.insert(key) {
            duplicates.push((note.note_start, note.note_end));
        }
    }
    duplicates.sort_by_key(|a| std::cmp::Reverse(a.0));
    let mut changed = 0usize;
    let mut receipts = master_state_identity::OwnerReceipts::default();
    for (start, end) in duplicates {
        let (hidden, receipt) = hide_note_print_object_range(xml, start, end);
        receipts.push_exact(receipt);
        if hidden {
            changed += 1;
        }
    }
    (changed, receipts)
}


fn hide_redundant_visible_staff_rests(
    xml: &mut String,
) -> (usize, master_state_identity::OwnerReceipts) {
    let notes = parse_atomic_notes(xml);
    let mut groups: BTreeMap<(String, String, String), Vec<&AtomicNote>> = BTreeMap::new();
    for note in notes.iter().filter(|n| n.visible && n.pitch.is_none()) {
        groups.entry((note.measure.clone(), note.staff.clone(), note.voice.clone())).or_default().push(note);
    }

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for ((_measure, _staff, _voice), mut rests) in groups {
        let has_visible_pitch_in_voice = notes.iter().any(|n|
            n.visible
                && n.pitch.is_some()
                && n.measure == _measure
                && n.staff == _staff
                && n.voice == _voice
        );
        if has_visible_pitch_in_voice || rests.len() <= 1 {
            continue;
        }
        rests.sort_by(|a, b| b.duration.cmp(&a.duration).then_with(|| a.start.cmp(&b.start)));
        // Eine sichtbare Pauseninformation bleibt als Orientierung erhalten;
        // die übrigen reinen Füllpausen dieser schweigenden Stimme werden
        // druckbildlich ausgeblendet.
        for rest in rests.into_iter().skip(1) {
            ranges.push((rest.note_start, rest.note_end));
        }
    }

    ranges.sort_by_key(|a| std::cmp::Reverse(a.0));
    let mut changed = 0usize;
    let mut receipts = master_state_identity::OwnerReceipts::default();
    for (start, end) in ranges {
        let (hidden, receipt) = hide_note_print_object_range(xml, start, end);
        receipts.push_exact(receipt);
        if hidden {
            changed += 1;
        }
    }
    (changed, receipts)
}


fn hide_note_print_object_range(xml: &mut String, start: usize, end: usize) -> (bool, Option<ExactTransformationReceipt>) {
    let (hidden, receipt) = structural_write_authority::hide_note_print_object_authorized(xml, start, end);
    (hidden.expect("source-bound note-visibility authorization"), receipt)
}

#[derive(Debug, Clone)]
struct LowerStaffClefBlock {
    abs_start: usize,
    abs_end: usize,
    local_start: usize,
    local_end: usize,
    sign: char,
}



fn is_safe_to_move_lower_staff_clef_attributes(block: &str) -> bool {
    block.contains("<clef")
        && (block.contains("number=\"2\"") || block.contains("number='2'"))
        && !block.contains("<divisions")
        && !block.contains("<key")
        && !block.contains("<time")
        && !block.contains("<staves")
        && !block.contains("<transpose")
}

fn visible_staff2_event_local_starts(measure: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut scan = 0usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else { break; };
        let end = start + end_rel + "</note>".len();
        let note = &measure[start..end];
        let visible = !is_hidden_note(note);
        let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
        if visible && staff.trim() == "2" {
            out.push(start);
        }
        scan = end;
    }
    out
}

fn visible_staff2_pitch_midis_before(measure: &str, local_limit: usize) -> Vec<i32> {
    let mut out = Vec::new();
    let mut scan = 0usize;
    let limit = local_limit.min(measure.len());
    while let Some(rel) = measure[scan..limit].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else { break; };
        let end = start + end_rel + "</note>".len();
        if end > limit { break; }
        let note = &measure[start..end];
        let visible = !is_hidden_note(note);
        let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
        if visible
            && staff.trim() == "2"
            && !note.contains("<rest")
            && let Some(midi) = midi_from_note_fragment(note)
        {
            out.push(midi);
        }
        scan = end;
    }
    out
}

// A lower-staff clef may only be advanced inside its own preceding local
// clef/cursor scope: the bytes after the last attributes, backup or forward
// element of the same measure. Crossing such a boundary would move the clef into
// a different state or temporal domain (causal parent:
// docs/selector_derivation_contract_lower_staff_clef_original_source_envelope_r1252.md).
fn lower_staff_clef_local_scope_start(measure: &str, clef_local_start: usize) -> usize {
    const SCOPE_BOUNDARIES: [(&str, &str); 3] = [
        ("<attributes", "</attributes>"),
        ("<backup", "</backup>"),
        ("<forward", "</forward>"),
    ];
    let prefix = &measure[..clef_local_start];
    let mut scope_start = 0usize;
    loop {
        let next = SCOPE_BOUNDARIES
            .iter()
            .filter_map(|(open, close)| {
                prefix[scope_start..]
                    .find(open)
                    .map(|relative| (scope_start + relative, *close))
            })
            .min_by_key(|(start, _)| *start);
        let Some((boundary_start, close)) = next else {
            return scope_start;
        };
        let Some(close_relative) = prefix[boundary_start..].find(close) else {
            // An unterminated boundary leaves no provable local scope.
            return clef_local_start;
        };
        scope_start = boundary_start + close_relative + close.len();
    }
}

fn advance_lower_staff_bass_clef_before_preceding_low_or_rest_events(
    xml: &mut String,
) -> anyhow::Result<usize> {
    let mut ops = Vec::<structural_write_authority::LowerStaffClefMoveRequest>::new();
    let mut current_staff2_sign = 'F';
    let mut scan = 0usize;
    while let Some((measure_start, measure_end, _measure_number)) = find_next_measure_bounds(xml, scan) {
        let is_score_opening_measure = scan == 0;
        let measure = &xml[measure_start..measure_end];
        let clefs = collect_lower_staff_clef_attribute_blocks(measure, measure_start);
        if clefs.is_empty() {
            scan = measure_end;
            continue;
        }
        let staff2_events = visible_staff2_event_local_starts(measure);
        let mut active_sign = current_staff2_sign;
        for (idx, clef) in clefs.iter().enumerate() {
            let block = &xml[clef.abs_start..clef.abs_end];
            let is_initial_score_clef = is_score_opening_measure
                && idx == 0
                && clef.local_start < measure.find("<note").unwrap_or(measure.len());
            if !is_initial_score_clef
                && clef.sign == 'F'
                && active_sign != 'F'
                && is_safe_to_move_lower_staff_clef_attributes(block)
            {
                let local_scope_start =
                    lower_staff_clef_local_scope_start(measure, clef.local_start);
                let local_scope = &measure[local_scope_start..clef.local_start];
                let preceding_events: Vec<usize> = staff2_events
                    .iter()
                    .copied()
                    .filter(|pos| *pos >= local_scope_start && *pos < clef.local_start)
                    .collect();
                if let Some(first_event) = preceding_events.first().copied() {
                    let midis_before = visible_staff2_pitch_midis_before(local_scope, local_scope.len());
                    let high_before = midis_before
                        .iter()
                        .any(|midi| *midi >= LOWER_STAFF_TREBLE_CLEF_FROM_MIDI);
                    let has_low_or_rest_before = !midis_before.is_empty()
                        || count_visible_staff2_rests_before(local_scope, local_scope.len()) > 0;
                    if has_low_or_rest_before && !high_before {
                        ops.push(structural_write_authority::LowerStaffClefMoveRequest::new(
                            clef.abs_start,
                            clef.abs_end,
                            measure_start + first_event,
                            block.to_string(),
                        ));
                    }
                }
            }
            active_sign = clef.sign;
        }
        current_staff2_sign = active_sign;
        scan = measure_end;
    }

    // A rejected batch leaves `xml` byte-identical and is transported to the
    // #57 trial as an owner failure; it never becomes a silent zero-move success.
    structural_write_authority::apply_lower_staff_clef_moves_authorized(xml, ops).map_err(|error| {
        error.context("source-bound lower-staff clef move authorization rejected the local batch")
    })
}

fn prune_rest_only_staff2_clef_noops(
    xml: &mut String,
) -> (usize, Option<ExactTransformationReceipt>) {
    #[derive(Debug, Clone)]
    struct GlobalClef {
        abs_start: usize,
        abs_end: usize,
        score_opening_measure: bool,
        local_start: usize,
        sign: char,
    }

    let mut clefs: Vec<GlobalClef> = Vec::new();
    let mut note_positions: Vec<usize> = Vec::new();
    let mut scan = 0usize;

    while let Some((measure_start, measure_end, _measure_number)) = find_next_measure_bounds(xml, scan) {
        let score_opening_measure = scan == 0;
        let measure = &xml[measure_start..measure_end];
        for clef in collect_lower_staff_clef_attribute_blocks(measure, measure_start) {
            clefs.push(GlobalClef {
                abs_start: clef.abs_start,
                abs_end: clef.abs_end,
                score_opening_measure,
                local_start: clef.local_start,
                sign: clef.sign,
            });
        }
        for local_note in visible_staff2_pitch_note_local_starts(measure) {
            note_positions.push(measure_start + local_note);
        }
        scan = measure_end;
    }

    if clefs.len() < 2 {
        return (0, None);
    }

    let mut ranges = Vec::new();
    for idx in 0..clefs.len().saturating_sub(1) {
        let clef = &clefs[idx];
        let next = &clefs[idx + 1];
        let is_initial_score_clef = clef.score_opening_measure
            && clef.local_start
                < xml[clef.abs_start.saturating_sub(clef.local_start)..]
                    .find("<note")
                    .unwrap_or(usize::MAX);
        if is_initial_score_clef {
            continue;
        }
        let has_visible_pitch_until_next_clef = note_positions
            .iter()
            .any(|pos| *pos > clef.abs_end && *pos < next.abs_start);
        if has_visible_pitch_until_next_clef {
            continue;
        }
        // CLF09 ist absichtlich enger als die entfernten CLF07/CLF08:
        // Nur der aktuelle Schluesselwechsel wird entfernt, wenn bis zum
        // naechsten Staff-2-Schluessel keine sichtbare Staff-2-Note kommt.
        // Pausen, backup/forward und technische Stimmfuellung bleiben unveraendert.
        // Der Folgeschluessel bleibt stehen und schuetzt den Registerzustand der
        // naechsten sichtbaren Note.
        if clef.sign != next.sign || clef.abs_start < next.abs_start {
            ranges.push((clef.abs_start, clef.abs_end));
        }
    }

    let count = ranges.len();
    let receipt = remove_xml_ranges(xml, &merge_xml_ranges(ranges));
    (count, receipt)
}

fn prune_redundant_lower_staff_clefs(
    xml: &mut String,
) -> ((usize, usize, usize, usize), Option<ExactTransformationReceipt>) {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut empty_removed = 0usize;
    let mut trailing_removed = 0usize;
    let mut redundant_removed = 0usize;
    let mut advance_removed = 0usize;
    let mut current_staff2_sign: Option<char> = Some('F');
    let mut scan = 0usize;

    while let Some((measure_start, measure_end, measure_number)) = find_next_measure_bounds(xml, scan) {
        let is_score_opening_measure = scan == 0;
        let measure = &xml[measure_start..measure_end];
        let clefs = collect_lower_staff_clef_attribute_blocks(measure, measure_start);
        if clefs.is_empty() {
            scan = measure_end;
            continue;
        }
        let note_positions = visible_staff2_pitch_note_local_starts(measure);
        let mut remove_indices = BTreeSet::new();

        for idx in 0..clefs.len() {
            let sign = clefs[idx].sign;
            let next_start = clefs.get(idx + 1).map_or(measure.len(), |c| c.local_start);
            let has_note = note_positions.iter().any(|pos| *pos > clefs[idx].local_end && *pos < next_start);

            let is_initial_score_clef = is_score_opening_measure
                && idx == 0
                && clefs[idx].local_start < measure.find("<note").unwrap_or(measure.len());
            let remove_reason = if is_initial_score_clef {
                None
            } else if current_staff2_sign == Some(sign) {
                Some("redundant")
            } else if !has_note && idx + 1 < clefs.len() {
                Some("empty")
            } else if !has_note && next_measure_has_initial_staff2_clef(xml, measure_number, sign) {
                Some("advance")
            } else if !has_note && sign == 'F' && next_measure_has_initial_staff2_clef(xml, measure_number, 'F') {
                Some("trailing")
            } else {
                None
            };

            match remove_reason {
                Some("redundant") => {
                    remove_indices.insert(idx);
                    redundant_removed += 1;
                }
                Some("empty") => {
                    remove_indices.insert(idx);
                    empty_removed += 1;
                }
                Some("advance") => {
                    remove_indices.insert(idx);
                    advance_removed += 1;
                }
                Some("trailing") => {
                    remove_indices.insert(idx);
                    trailing_removed += 1;
                }
                _ => {
                    current_staff2_sign = Some(sign);
                }
            }
        }

        for idx in remove_indices {
            if let Some(clef) = clefs.get(idx) {
                ranges.push((clef.abs_start, clef.abs_end));
            }
        }
        scan = measure_end;
    }
    let receipt = remove_xml_ranges(xml, &merge_xml_ranges(ranges));
    (
        (empty_removed, trailing_removed, redundant_removed, advance_removed),
        receipt,
    )
}


fn collect_lower_staff_clef_attribute_blocks(measure: &str, abs_measure_start: usize) -> Vec<LowerStaffClefBlock> {
    let mut out = Vec::new();
    let mut scan = 0usize;
    while let Some(rel) = measure[scan..].find("<attributes") {
        let local_start = scan + rel;
        let Some(end_rel) = measure[local_start..].find("</attributes>") else { break; };
        let local_end = local_start + end_rel + "</attributes>".len();
        let block = &measure[local_start..local_end];
        if block.contains("<clef number=\"2\"") || block.contains("<clef number='2'") {
            let sign = if block.contains("<sign>G</sign>") { Some('G') } else if block.contains("<sign>F</sign>") { Some('F') } else { None };
            if let Some(sign) = sign {
                out.push(LowerStaffClefBlock {
                    abs_start: abs_measure_start + local_start,
                    abs_end: abs_measure_start + local_end,
                    local_start,
                    local_end,
                    sign,
                });
            }
        }
        scan = local_end;
    }
    out
}

fn visible_staff2_pitch_note_local_starts(measure: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut scan = 0usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else { break; };
        let end = start + end_rel + "</note>".len();
        let note = &measure[start..end];
        if note.contains("<pitch>") && !note.contains("<rest") {
            let open_end = note.find('>').unwrap_or(0);
            let open_tag = &note[..=open_end.min(note.len().saturating_sub(1))];
            let visible = !(open_tag.contains("print-object=\"no\"") || open_tag.contains("print-object='no'"));
            let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
            if visible && staff.trim() == "2" {
                out.push(start);
            }
        }
        scan = end;
    }
    out
}

fn next_measure_has_initial_staff2_clef(xml: &str, current_measure: usize, sign: char) -> bool {
    let mut scan = 0usize;
    while let Some((start, end, number)) = find_next_measure_bounds(xml, scan) {
        if number <= current_measure {
            scan = end;
            continue;
        }
        let measure = &xml[start..end];
        let first_note = measure.find("<note").unwrap_or(measure.len());
        let before_first_note = &measure[..first_note];
        if before_first_note.contains("<clef number=\"2\">") || before_first_note.contains("<clef number='2'>") {
            return match sign {
                'G' => before_first_note.contains("<sign>G</sign>"),
                'F' => before_first_note.contains("<sign>F</sign>"),
                _ => false,
            };
        }
        return false;
    }
    false
}



#[cfg(test)]
fn dense_register_notation_score_and_width(facts: &MeasureHorizontalLayoutFacts) -> (i32, usize) {
    let lower_clefs = facts.lower_clef_element_count as i32;
    let effective_lower_clefs = if facts.is_initial_measure_in_sequence {
        lower_clefs.saturating_sub(1)
    } else {
        lower_clefs
    };
    let lower_visible_notes = facts.lower_visible_pitched_note_count as i32;
    let lower_visible_rests = facts.lower_visible_rest_count as i32;
    let octave_shift_count = facts.octave_shift_element_count as i32;
    let alter_count = facts.alter_element_count as i32;
    let beam_count = facts.beam_element_count as i32;
    let short_count = facts.sixteenth_type_count as i32
        + facts.thirty_second_type_count as i32 * 2;
    let extremes = facts.extreme_register_note_count as i32;

    let mut score = 0_i32;
    score += effective_lower_clefs * 20;
    score += octave_shift_count * 10;
    score += lower_visible_notes * 3;
    score += lower_visible_rests * 4;
    score += alter_count * 2;
    score += beam_count / 2;
    score += short_count / 2;
    score += extremes * 3;

    let target_width = (1120 + (score.max(0) as usize).saturating_mul(5)).min(1680);
    (score, target_width)
}

fn count_visible_staff_notes(measure: &str, staff_id: &str) -> usize {
    count_visible_staff_entries(measure, staff_id, true)
}

fn count_visible_staff_rests(measure: &str, staff_id: &str) -> usize {
    count_visible_staff_entries(measure, staff_id, false)
}

fn count_visible_staff_entries(measure: &str, staff_id: &str, pitched: bool) -> usize {
    let mut count = 0usize;
    let mut scan = 0usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else { break; };
        let end = start + end_rel + "</note>".len();
        let note = &measure[start..end];
        let open_end = note.find('>').unwrap_or(0);
        let open_tag = &note[..=open_end.min(note.len().saturating_sub(1))];
        let visible = !(open_tag.contains("print-object=\"no\"") || open_tag.contains("print-object='no'"));
        let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
        let is_pitched = note.contains("<pitch>") && !note.contains("<rest");
        if visible && staff.trim() == staff_id && is_pitched == pitched {
            count += 1;
        }
        scan = end;
    }
    count
}

fn remove_xml_ranges(
    xml: &mut String,
    ranges: &[(usize, usize)],
) -> Option<ExactTransformationReceipt> {
    let (removed, receipt) =
        structural_write_authority::remove_allowed_xml_ranges_authorized(xml, ranges);
    removed.expect("source-bound structural range-removal authorization");
    receipt
}

fn merge_xml_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.retain(|(a, b)| *a < *b);
    ranges.sort_unstable();
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = out.last_mut()
            && start <= last.1
        {
            last.1 = last.1.max(end);
            continue;
        }
        out.push((start, end));
    }
    out
}

fn remove_empty_notations_blocks(xml: &mut String) -> (usize, Option<ExactTransformationReceipt>) {
    let mut ranges = Vec::new();
    let mut scan = 0usize;
    while let Some(rel) = xml[scan..].find("<notations>") {
        let start = scan + rel;
        let content_start = start + "<notations>".len();
        let Some(close_rel) = xml[content_start..].find("</notations>") else { break; };
        let end = content_start + close_rel + "</notations>".len();
        let inner = &xml[content_start..content_start + close_rel];
        if inner.trim().is_empty() {
            ranges.push((start, end));
        }
        scan = end;
    }
    let count = ranges.len();
    let receipt = remove_xml_ranges(xml, &ranges);
    (count, receipt)
}


fn plan_visual_layout_pressure_decisions(
    xml: &str,
    plan_builder: &mut HorizontalLayoutPlanBuilder,
) -> anyhow::Result<Vec<(usize, i32, Vec<&'static str>)>> {
    let pressure_points = collect_visual_layout_pressure_points(xml);
    if pressure_points.is_empty() {
        return Ok(pressure_points);
    }

    let last_measure = find_last_measure_bounds(xml)
        .map_or(0, |(_, _, number)| number);
    let final_group_len = last_measure.min(4);
    let final_group_start = last_measure
        .saturating_sub(final_group_len.saturating_sub(1))
        .max(1);

    for measure in select_balanced_visual_system_breaks(
        &pressure_points,
        last_measure,
        final_group_start,
    ) {
        if let Some(slot_index) =
            horizontal_layout_slot_index_for_locator(xml, measure)
        {
            plan_builder.plan_new_system(
                slot_index,
                HorizontalLayoutDecisionOwner::VisualLayoutPressure,
            );
        }
    }

    for measure in select_balanced_visual_page_breaks(
        &pressure_points,
        last_measure,
        final_group_start,
    ) {
        if measure <= 1 {
            continue;
        }
        if let Some(slot_index) =
            horizontal_layout_slot_index_for_locator(xml, measure)
        {
            plan_builder.plan_new_page(
                slot_index,
                HorizontalLayoutDecisionOwner::VisualLayoutPressure,
            );
        }
    }

    Ok(pressure_points)
}

/// Visual layout pressure pass. A wrapper only: it forwards the PRESERVING
/// E044 receipt of its inner owner and mints none of its own.
fn apply_visual_layout_pressure_pass(
    xml: &mut String,
    plan_builder: &mut HorizontalLayoutPlanBuilder,
) -> (anyhow::Result<()>, master_state_identity::OwnerReceipts) {
    let mut receipts = master_state_identity::OwnerReceipts::default();
    let pressure_points = match plan_visual_layout_pressure_decisions(xml, plan_builder) {
        Ok(pressure_points) => pressure_points,
        Err(error) => return (Err(error), receipts),
    };
    if pressure_points.is_empty() {
        receipts.push_preserving(ensure_visual_layout_defaults(xml, 150, 96).1);
        return (Ok(()), receipts);
    }

    receipts.push_preserving(ensure_visual_layout_defaults(xml, 158, 104).1);
    for (measure, score, _reasons) in &pressure_points {
        let identity_resolved = raise_octave_shift_default_y_in_measure(
            xml,
            *measure,
            if *score >= 34 { 82 } else { 70 },
        );
        assert_eq!(
            identity_resolved, 1,
            "FeedbackIdentityChannel: Layoutdruck-Taktidentitaet ist nicht mehr aufloesbar"
        );
    }

    (Ok(()), receipts)
}


fn apply_final_short_system_balance(
    plan_builder: &mut HorizontalLayoutPlanBuilder,
) -> anyhow::Result<()> {
    let measure_count = plan_builder.slot_count();
    if measure_count == 0 {
        return Ok(());
    }

    let starts = plan_builder.planned_system_start_slots();
    if starts.len() < 2 {
        return Ok(());
    }

    let last_slot_index = measure_count - 1;
    let last_start = *starts.last().unwrap_or(&0);
    let previous_start = starts.iter().rev().nth(1).copied().unwrap_or(0);
    let final_system_measures = measure_count.saturating_sub(last_start);
    let previous_system_measures = last_start.saturating_sub(previous_start).max(1);
    let combined_measures = final_system_measures + previous_system_measures;
    let final_slot_is_page_start = plan_builder
        .planned_starts_new_page(last_slot_index)
        .unwrap_or(false);

    let should_merge = last_start == last_slot_index
        && final_system_measures <= 1
        && previous_system_measures <= 3
        && combined_measures <= 4
        && !final_slot_is_page_start;

    if should_merge {
        plan_builder.plan_system_state(
            last_slot_index,
            false,
            HorizontalLayoutDecisionOwner::FinalShortSystemBalance,
        );
    }

    Ok(())
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FinalNotationPlanEvidenceProjection {
    AuthoritativeConsole,
    RunLocalProbeSilent,
}

impl FinalNotationPlanEvidenceProjection {
    const fn emits_global_console_evidence(self) -> bool {
        matches!(self, Self::AuthoritativeConsole)
    }
}

#[cfg(test)]
mod final_notation_plan_evidence_projection_tests {
    use super::FinalNotationPlanEvidenceProjection;

    #[test]
    fn run_local_probe_does_not_claim_global_console_evidence() {
        assert!(FinalNotationPlanEvidenceProjection::AuthoritativeConsole
            .emits_global_console_evidence());
        assert!(!FinalNotationPlanEvidenceProjection::RunLocalProbeSilent
            .emits_global_console_evidence());
    }
}

fn build_final_notation_plan(
    final_notation_xml: &str,
    seed: &HorizontalLayoutSeedPlan,
    evidence_projection: FinalNotationPlanEvidenceProjection,
) -> anyhow::Result<(HorizontalLayoutPlan, GlobalHorizontalPageSystemPlanAudit)> {
    let mut plan_builder =
        HorizontalLayoutPlanBuilder::from_seed_for_final_notation(
            seed,
            final_notation_xml,
        )?;
    let _ = plan_visual_layout_pressure_decisions(final_notation_xml, &mut plan_builder)?;
    apply_final_short_system_balance(&mut plan_builder)?;

    let base_plan = plan_builder.close()?;
    let (_global_reflow_candidate_plan, global_reflow_audit) =
        build_global_horizontal_system_reflow(final_notation_xml, &base_plan)?;
    let page_count_preference_audit = evaluate_global_horizontal_page_count_preference(
        final_notation_xml,
        &base_plan,
        &global_reflow_audit,
    )?;
    let feasible_pages = if page_count_preference_audit.feasible_page_counts.is_empty() {
        "none".to_string()
    } else {
        page_count_preference_audit
            .feasible_page_counts
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    };
    let frontier_candidates = if page_count_preference_audit.feasible_candidates.is_empty() {
        "none".to_string()
    } else {
        page_count_preference_audit
            .feasible_candidates
            .iter()
            .map(|candidate| {
                format!(
                    "p{}{{sys{};sing{};legacyThin{};slots{}..{};spp={}..{};maxLoad{};forced{};sysBal{};slotBal{};slack{}}}",
                    candidate.pages,
                    candidate.systems,
                    candidate.singletons,
                    candidate.nonterminal_thin_pages(),
                    candidate.min_page_slots(),
                    candidate.max_page_slots(),
                    candidate.min_systems_per_page(),
                    candidate.max_systems_per_page(),
                    candidate.max_system_load,
                    candidate.forced_over_capacity_singletons,
                    candidate.system_count_balance_penalty,
                    candidate.page_slot_balance_penalty,
                    candidate.slack_square_sum,
                )
            })
            .collect::<Vec<_>>()
            .join(",")
    };
    let thin_first_frontier_candidates =
        if page_count_preference_audit.thin_first_candidates.is_empty() {
            "none".to_string()
        } else {
            page_count_preference_audit
                .thin_first_candidates
                .iter()
                .map(|candidate| {
                    format!(
                        "p{}{{sys{};sing{};legacyThin{};thinLB{};slots{}..{};spp={}..{};maxLoad{};forced{};sysBal{};slotBal{};slack{}}}",
                        candidate.pages,
                        candidate.systems,
                        candidate.singletons,
                        candidate.nonterminal_thin_pages(),
                        candidate.combinatorial_thin_lower_bound(),
                        candidate.min_page_slots(),
                        candidate.max_page_slots(),
                        candidate.min_systems_per_page(),
                        candidate.max_systems_per_page(),
                        candidate.max_system_load,
                        candidate.forced_over_capacity_singletons,
                        candidate.system_count_balance_penalty,
                        candidate.page_slot_balance_penalty,
                        candidate.slack_square_sum,
                    )
                })
                .collect::<Vec<_>>()
                .join(",")
        };
    let preference_selection =
        page_count_preference_audit
            .selected_preference_candidate
            .as_ref()
            .map_or_else(
                || "none".to_string(),
                |candidate| {
                    format!(
                        "p{}{{sys{};sing{};legacyThin{};thinLB{};avoidableThin{};preferredDistance{};slots{}..{};spp={}..{};maxLoad{};forced{};sysBal{};slotBal{};slack{}}}",
                        candidate.pages,
                        candidate.systems,
                        candidate.singletons,
                        candidate.nonterminal_thin_pages(),
                        candidate.combinatorial_thin_lower_bound(),
                        candidate.avoidable_thin_pages(),
                        candidate.pages.abs_diff(page_count_preference_audit.preferred_pages),
                        candidate.min_page_slots(),
                        candidate.max_page_slots(),
                        candidate.min_systems_per_page(),
                        candidate.max_systems_per_page(),
                        candidate.max_system_load,
                        candidate.forced_over_capacity_singletons,
                        candidate.system_count_balance_penalty,
                        candidate.page_slot_balance_penalty,
                        candidate.slack_square_sum,
                    )
                },
            );
    if evidence_projection.emits_global_console_evidence() {
        console_progress(format!(
            "HorizontalLayoutGlobalPageCountPreferenceShadow: Slots {} | PreferredPages {} | TestedPages {}..{} | FeasiblePages {} | FullFrontier {} | ThinFirstFrontier {} | PreferenceSelection {} | LowestFeasiblePages {} | ClosestLowerFeasiblePages {} | PreferredPagesFeasible {} | VerticalMaxSystemsPerPage {} | MaxSystemMeasures {} | AcceptedSystemCapacity {} | HorizontalMinimumSystems {} | ClosestLowerSystems {} | ClosestLowerSingletons {} | ClosestLowerPageSlots {}..{} | ClosestLowerSystemsPerPage {}..{} | ClosestLowerMaxSystemLoad {} | ClosestLowerForcedOverCapacitySingletons {} | PolicyWrite nein",
        page_count_preference_audit.slots_compared,
        page_count_preference_audit.preferred_pages,
        page_count_preference_audit.first_tested_pages,
        page_count_preference_audit.last_tested_pages,
        feasible_pages,
        frontier_candidates,
        thin_first_frontier_candidates,
        preference_selection,
        page_count_preference_audit.lowest_feasible_pages,
        page_count_preference_audit.closest_lower_feasible_pages,
        if page_count_preference_audit.preferred_pages_feasible { "ja" } else { "nein" },
        page_count_preference_audit.vertical_max_systems_per_page,
        page_count_preference_audit.max_system_measures,
        page_count_preference_audit.accepted_system_capacity,
        page_count_preference_audit.horizontal_minimum_systems,
        page_count_preference_audit.closest_lower_systems,
        page_count_preference_audit.closest_lower_singletons,
        page_count_preference_audit.closest_lower_min_page_slots,
        page_count_preference_audit.closest_lower_max_page_slots,
        page_count_preference_audit.closest_lower_min_systems_per_page,
        page_count_preference_audit.closest_lower_max_systems_per_page,
        page_count_preference_audit.closest_lower_max_system_load,
        page_count_preference_audit.closest_lower_forced_over_capacity_singletons,
        ));
    }
    let (joint_page_system_candidate, joint_page_system_audit) =
        build_global_horizontal_page_system_plan(
            final_notation_xml,
            &base_plan,
            &page_count_preference_audit,
        )?;
    if evidence_projection.emits_global_console_evidence() {
        console_progress(format!(
            "HorizontalLayoutGlobalPageSystemPreferenceCandidate: Slots {} | PreferredPages {} | SelectedPages {} | AuthoritativeSystems {} | SelectedSystems {} | AuthoritativeSingletons {} | SelectedSingletons {} | SystemBreakChanges {} | PageBreakChanges {} | AuthoritativePageSlots {}..{} | SelectedPageSlots {}..{} | AuthoritativeSystemsPerPage {}..{} | SelectedSystemsPerPage {}..{} | LegacyThin {} | AvoidableThin {} | MaxAuthoritativeSystemMeasures {} | MaxAllowedSystemMeasures {} | MaxSelectedSystemMeasures {} | VerticalMaxSystemsPerPage {} | AcceptedSystemCapacity {} | MaxSelectedSystemLoad {} | SelectedSystemLoads {} | SelectedMeasureDemands {} | ForcedOverCapacitySingletons {} | PolicyWrite nein",
        joint_page_system_audit.slots_compared,
        joint_page_system_audit.preferred_pages,
        joint_page_system_audit.selected_pages,
        joint_page_system_audit.authoritative_systems,
        joint_page_system_audit.selected_systems,
        joint_page_system_audit.authoritative_singletons,
        joint_page_system_audit.selected_singletons,
        joint_page_system_audit.system_break_changes,
        joint_page_system_audit.page_break_changes,
        joint_page_system_audit.authoritative_min_page_slots,
        joint_page_system_audit.authoritative_max_page_slots,
        joint_page_system_audit.selected_min_page_slots,
        joint_page_system_audit.selected_max_page_slots,
        joint_page_system_audit.authoritative_min_systems_per_page,
        joint_page_system_audit.authoritative_max_systems_per_page,
        joint_page_system_audit.selected_min_systems_per_page,
        joint_page_system_audit.selected_max_systems_per_page,
        joint_page_system_audit.selected_nonterminal_thin_pages,
        joint_page_system_audit.selected_avoidable_thin_pages,
        joint_page_system_audit.max_authoritative_system_measures,
        joint_page_system_audit.max_allowed_system_measures,
        joint_page_system_audit.max_selected_system_measures,
        joint_page_system_audit.vertical_max_systems_per_page,
        joint_page_system_audit.accepted_system_capacity,
        joint_page_system_audit.max_selected_system_load,
        joint_page_system_audit.selected_system_load_projection(),
        joint_page_system_audit.selected_measure_demand_projection(),
        joint_page_system_audit.forced_over_capacity_singletons,
        ));
    }

    let authoritative_plan = base_plan.with_global_page_system_reflow(
        &joint_page_system_candidate,
        &joint_page_system_audit,
    )?;
    let joint_cutover_drift = base_plan.compare_decisions(&authoritative_plan)?;
    if joint_cutover_drift.width_changes != 0
        || joint_cutover_drift.system_break_changes != joint_page_system_audit.system_break_changes
        || joint_cutover_drift.page_break_changes != joint_page_system_audit.page_break_changes
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemPreferenceCutover: Promotion veraendert Widths oder stimmt nicht exakt mit der gebundenen Preference-Evidence ueberein"
        );
    }

    let (readback_candidate, readback_audit) =
        build_global_horizontal_page_system_plan(
            final_notation_xml,
            &authoritative_plan,
            &page_count_preference_audit,
        )?;
    let readback_plan =
        authoritative_plan.with_global_page_system_reflow(&readback_candidate, &readback_audit)?;
    let readback_drift = authoritative_plan.compare_decisions(&readback_plan)?;
    if readback_drift.width_changes != 0
        || readback_drift.system_break_changes != 0
        || readback_drift.page_break_changes != 0
        || readback_drift.provenance_changes != 0
    {
        anyhow::bail!(
            "GlobalHorizontalPageSystemPreferenceCutover: promovierter Joint-Plan ist unter identischen finalen Facts und gebundener Preference nicht idempotent"
        );
    }

    console_progress(format!(
        "HorizontalLayoutGlobalPageSystemPreferenceCutover: Slots {} | ChangedSlots {} | WidthChanges {} | SystemBreakChanges {} | PageBreakChanges {} | PreferredPages {} | ProductivePages {} | AuthoritativeSystems {} | ProductiveSystems {} | AuthoritativeSingletons {} | ProductiveSingletons {} | LegacyThin {} | AvoidableThin {} | ProvenanceChanges {} | PolicyWrite ja",
        joint_cutover_drift.slots_compared,
        joint_cutover_drift.changed_slots,
        joint_cutover_drift.width_changes,
        joint_cutover_drift.system_break_changes,
        joint_cutover_drift.page_break_changes,
        joint_page_system_audit.preferred_pages,
        joint_page_system_audit.selected_pages,
        joint_page_system_audit.authoritative_systems,
        joint_page_system_audit.selected_systems,
        joint_page_system_audit.authoritative_singletons,
        joint_page_system_audit.selected_singletons,
        joint_page_system_audit.selected_nonterminal_thin_pages,
        joint_page_system_audit.selected_avoidable_thin_pages,
        joint_cutover_drift.provenance_changes,
    ));

    if authoritative_plan.slots.iter().any(|slot| {
        slot.directives.explicit_width.is_some() || slot.width_owner.is_some()
    }) {
        anyhow::bail!(
            "HorizontalWidthPolicyRetirementClosure: finaler Plan enthaelt trotz RendererAuto-Only-Policy noch explizite Width-Autoritaet"
        );
    }
    console_progress(format!(
        "HorizontalLayoutWidthPolicyRetirementClosure: Slots {} | ExplicitWidths 0 | RendererAutoSlots {} | WidthPolicyWriters 0 | PolicyWrite nein",
        authoritative_plan.slots.len(),
        authoritative_plan.slots.len(),
    ));

    Ok((authoritative_plan, joint_page_system_audit))
}


#[derive(Debug, Clone)]
struct LegacyPianisticReadabilityTarget {
    measure: usize,
    score: i32,
}

fn plan_pianistic_readability_decisions(
    xml: &str,
    plan_builder: &HorizontalLayoutPlanBuilder,
) -> Vec<LegacyPianisticReadabilityTarget> {
    let mut targets: Vec<LegacyPianisticReadabilityTarget> = Vec::new();
    let mut scan = 0_usize;
    let mut slot_index = 0_usize;
    let mut is_initial_measure_in_sequence = true;
    while let Some((start, end, measure_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[start..end];
        let facts = extract_measure_horizontal_layout_facts(
            measure,
            is_initial_measure_in_sequence,
        );
        let planned_starts_new_system = plan_builder
            .planned_starts_new_system(slot_index)
            .unwrap_or(false);
        let legacy_layout_state =
            observe_legacy_horizontal_layout_state_from_planned_system_state(
                measure,
                planned_starts_new_system,
            );
        let (selected, score) =
            legacy_pianistic_readability_projection(&facts, &legacy_layout_state);
        if selected {
            targets.push(LegacyPianisticReadabilityTarget {
                measure: measure_number,
                score,
            });
        }
        is_initial_measure_in_sequence = false;
        slot_index += 1;
        scan = end;
    }

    targets.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.measure.cmp(&b.measure)));
    targets.truncate(3);
    targets
}

/// Pianistic readability polish. A wrapper only: it forwards the PRESERVING
/// E013 receipts of its inner owner in application order.
fn apply_pianistic_readability_polish(
    xml: &mut String,
    plan_builder: &mut HorizontalLayoutPlanBuilder,
) -> master_state_identity::OwnerReceipts {
    let mut receipts = master_state_identity::OwnerReceipts::default();
    let targets = plan_pianistic_readability_decisions(xml, plan_builder);
    for target in targets {
        let staff_distance = if target.score >= 82 { 134 } else { 122 };
        receipts.push_preserving(
            ensure_local_staff_distance_print(xml, target.measure, staff_distance).1,
        );
        let identity_resolved = raise_octave_shift_default_y_in_measure(
            xml,
            target.measure,
            if target.score >= 82 { 104 } else { 94 },
        );
        assert_eq!(
            identity_resolved, 1,
            "FeedbackIdentityChannel: pianistische Taktidentitaet ist nicht mehr aufloesbar"
        );
    }
    receipts
}


fn legacy_pianistic_readability_projection(
    facts: &MeasureHorizontalLayoutFacts,
    legacy_layout_state: &LegacyHorizontalLayoutStateObservation,
) -> (bool, i32) {
    let clefs = legacy_layout_state.double_quoted_lower_clef_count;
    let octave_starts = facts.octave_shift_start_count;
    let lower_notes = facts.lower_visible_non_rest_note_count;
    let extreme_notes = facts.extreme_register_note_count;
    let has_dense_lower_clef_octave = clefs >= 2 && octave_starts >= 1 && lower_notes >= 4;
    let score = (clefs as i32 * 18)
        + (octave_starts as i32 * 12)
        + (lower_notes as i32 * 2)
        + (extreme_notes as i32 * 2)
        + if legacy_layout_state.has_new_system_print {
            0
        } else {
            6
        };
    (has_dense_lower_clef_octave && score >= 62, score)
}

fn count_octave_shift_starts_in_fragment(fragment: &str) -> usize {
    let mut count = 0_usize;
    let mut scan = 0_usize;
    while let Some(rel) = fragment[scan..].find("<octave-shift") {
        let start = scan + rel;
        let Some(end_rel) = fragment[start..].find('>') else { break; };
        let tag = &fragment[start..start + end_rel + 1];
        if !tag.contains("type=\"stop\"") {
            count += 1;
        }
        scan = start + end_rel + 1;
    }
    count
}

fn count_visible_staff_notes_in_measure(measure: &str, staff: &str) -> usize {
    let mut count = 0_usize;
    let mut scan = 0_usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else { break; };
        let end = start + end_rel + "</note>".len();
        let note = &measure[start..end];
        if !note.contains("<rest") && !note_contains_print_object_no(note) {
            let note_staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "1".to_string());
            if note_staff.trim() == staff {
                count += 1;
            }
        }
        scan = end;
    }
    count
}

fn note_contains_print_object_no(note: &str) -> bool {
    let Some(open_end) = note.find('>') else { return false; };
    let opening = &note[..=open_end];
    opening.contains("print-object=\"no\"") || opening.contains("print-object='no'")
}

fn ensure_local_staff_distance_print(
    xml: &mut String,
    measure_number: usize,
    target_distance: usize,
) -> (bool, Option<master_state_identity::PreservingTransformationReceipt>) {
    let Some((measure_start, _end, _)) = find_measure_bounds_by_number(xml, measure_number) else {
        return (false, None);
    };
    let (written, receipt) = structural_write_authority::ensure_local_staff_distance_authorized(
        xml,
        measure_start,
        target_distance,
    );
    (
        written.expect("source-bound local staff-distance authorization"),
        receipt,
    )
}


fn select_balanced_visual_system_breaks(
    pressure_points: &[(usize, i32, Vec<&'static str>)],
    last_measure: usize,
    final_group_start: usize,
) -> BTreeSet<usize> {
    let mut selected = BTreeSet::new();
    if pressure_points.is_empty() || last_measure < 4 {
        return selected;
    }

    let mut candidates = pressure_points
        .iter()
        .filter(|(measure, score, _)| {
            *measure > 1
                && *score >= 30
                && !(*measure >= final_group_start && *measure <= last_measure)
        })
        .map(|(measure, score, reasons)| (*measure, *score, reasons.len()))
        .collect::<Vec<_>>();

    candidates.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.2.cmp(&a.2)).then_with(|| a.0.cmp(&b.0)));

    // Balance-Ziel: wenige zusätzliche Umbrüche an den wirklich dichten Knoten,
    // nicht vor jeder Druckstelle. Die vorhandene Pianisten-Layoutschicht setzt
    // bereits generische 2-Takt-Systeme; hier nur noch lokale Systementlastung.
    let max_extra_breaks = last_measure.div_ceil(8).clamp(3, 7);
    let min_gap = 4_usize;

    for (measure, _score, _reason_count) in candidates {
        if selected.len() >= max_extra_breaks {
            break;
        }
        if selected.iter().any(|existing| measure.abs_diff(*existing) < min_gap) {
            continue;
        }
        selected.insert(measure);
    }

    selected
}

fn select_balanced_visual_page_breaks(
    pressure_points: &[(usize, i32, Vec<&'static str>)],
    last_measure: usize,
    final_group_start: usize,
) -> BTreeSet<usize> {
    // v0.1.148 / LAY10: Automatische Seitenumbrueche aus VisualPageBalance
    // erzeugten zu oft duenne Seiten. Der Pass darf weiterhin lokale Breiten
    // Systemumbrueche planen, aber keine eigenen Seitenumbrueche erzwingen.
    let _ = (pressure_points, last_measure, final_group_start);
    BTreeSet::new()
}

fn collect_visual_layout_pressure_points(xml: &str) -> Vec<(usize, i32, Vec<&'static str>)> {
    let mut points = Vec::new();
    let mut scan = 0_usize;
    let mut is_initial_measure_in_sequence = true;
    while let Some((start, end, number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[start..end];
        let facts = extract_measure_horizontal_layout_facts(
            measure,
            is_initial_measure_in_sequence,
        );
        let (score, reasons) = visual_layout_pressure_score(&facts);
        if score >= 18 {
            points.push((number, score, reasons));
        }
        is_initial_measure_in_sequence = false;
        scan = end;
    }
    points.sort_by_key(|(measure, _, _)| *measure);
    points
}

fn visual_layout_pressure_score(
    facts: &MeasureHorizontalLayoutFacts,
) -> (i32, Vec<&'static str>) {
    let note_count = facts.note_element_count as i32;
    let beam_count = facts.beam_element_count as i32;
    let alter_count = facts.alter_element_count as i32;
    let octave_shift_count = facts.octave_shift_element_count as i32;
    let clef_count = facts.clef_element_count as i32;
    let extra_clef_count = if facts.is_initial_measure_in_sequence {
        clef_count.saturating_sub(2)
    } else {
        clef_count
    };
    let direction_count = facts.direction_element_count as i32;
    let short_note_count = facts.sixteenth_type_count as i32
        + facts.thirty_second_type_count as i32
        + facts.eighth_type_count as i32 / 2;
    let high_or_low_notes = facts.extreme_register_note_count as i32;
    let voice_count = facts.distinct_voice_count as i32;
    let staff_count = facts.distinct_staff_count as i32;

    let mut score = 0_i32;
    let mut reasons = Vec::new();
    if note_count >= 24 {
        score += note_count / 2;
        reasons.push("Notendichte");
    }
    if beam_count >= 12 {
        score += beam_count / 3;
        reasons.push("Balkendichte");
    }
    if alter_count >= 8 {
        score += alter_count;
        reasons.push("Vorzeichendichte");
    }
    if octave_shift_count > 0 {
        score += 11 + octave_shift_count * 2;
        reasons.push("Oktavierungszeichen");
    }
    if extra_clef_count > 0 {
        score += 12 + extra_clef_count * 3;
        reasons.push("temporäre Schlüsselwechsel");
    }
    if direction_count >= 2 {
        score += direction_count * 2;
        reasons.push("Richtungs-/Dynamikzeichen");
    }
    if short_note_count >= 16 {
        score += short_note_count / 3;
        reasons.push("kurze Notenwerte");
    }
    if high_or_low_notes >= 4 {
        score += high_or_low_notes * 2;
        reasons.push("extreme Registerlage");
    }
    if voice_count >= 3 || staff_count >= 2 {
        score += 5;
        reasons.push("mehrere Stimmen/Systeme");
    }

    (score, reasons)
}

fn collect_simple_tag_values(text: &str, tag: &str) -> BTreeSet<String> {
    let mut values = BTreeSet::new();
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut scan = 0_usize;
    while let Some(rel) = text[scan..].find(&open) {
        let start = scan + rel + open.len();
        let Some(close_rel) = text[start..].find(&close) else {
            break;
        };
        let end = start + close_rel;
        values.insert(text[start..end].trim().to_string());
        scan = end + close.len();
    }
    values
}

fn count_extreme_register_notes(measure: &str) -> usize {
    let mut count = 0_usize;
    let mut scan = 0_usize;
    while let Some(rel) = measure[scan..].find("<note") {
        let start = scan + rel;
        let Some(end_rel) = measure[start..].find("</note>") else {
            break;
        };
        let end = start + end_rel + "</note>".len();
        let note = &measure[start..end];
        if !note.contains("<rest")
            && let Some(midi) = midi_from_note_fragment(note)
            && (midi >= 78 || midi <= 36)
        {
            count += 1;
        }
        scan = end;
    }
    count
}


fn raise_octave_shift_default_y_in_measure(
    xml: &mut str,
    measure_number: usize,
    wanted_y: i32,
) -> usize {
    // Der historische Funktionsname und die Taktidentitaet bleiben als geschuetzter
    // FeedbackIdentityChannel bestehen. Seit r287 besitzt dieser
    // Kanal jedoch kein Oktav-Geometrieschreibrecht mehr. Die fruehere Y-Anforderung
    // wird nur noch als Herkunftsevidenz entgegengenommen; produktiv wird ausschliesslich
    // geprueft, ob die identifizierte Taktstelle im aktuellen Dokument noch existiert.
    let _ = wanted_y;
    usize::from(find_measure_bounds_by_number(xml, measure_number).is_some())
}

fn ensure_visual_layout_defaults(
    xml: &mut String,
    system_distance: usize,
    staff_distance: usize,
) -> (bool, Option<master_state_identity::PreservingTransformationReceipt>) {
    let (written, receipt) = structural_write_authority::ensure_visual_layout_defaults_authorized(
        xml,
        system_distance,
        staff_distance,
    );
    (
        written.expect("source-bound visual-layout-defaults authorization"),
        receipt,
    )
}

fn find_measure_bounds_by_number(xml: &str, measure_number: usize) -> Option<(usize, usize, usize)> {
    let mut scan = 0_usize;
    while let Some((start, end, number)) = find_next_measure_bounds(xml, scan) {
        if number == measure_number {
            return Some((start, end, number));
        }
        scan = end;
    }
    None
}

fn find_last_measure_bounds(xml: &str) -> Option<(usize, usize, usize)> {
    let mut scan = 0_usize;
    let mut last = None;
    while let Some(found) = find_next_measure_bounds(xml, scan) {
        scan = found.1;
        last = Some(found);
    }
    last
}

fn ensure_final_barline(xml: &mut String) -> Option<master_state_identity::AdditiveTransformationReceipt> {
    structural_write_authority::ensure_final_barline_authorized(xml)
        .expect("source-bound final-barline authorization")
        .1
}

