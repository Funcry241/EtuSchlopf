// Typed metric-beam observation only: no candidate write, filesystem sink, or report projection.

#[derive(Debug, Clone)]
struct PartialBeatMeasureSpan {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct PartialBeatRawNote {
    start: usize,
    end: usize,
    text: String,
}

#[derive(Debug, Clone)]
struct PartialBeatNoteInfo {
    index: usize,
    cursor_run: Option<usize>,
    start: i64,
    duration: i64,
    staff: String,
    voice: String,
    note_type: String,
    is_rest: bool,
    is_chord: bool,
    is_hidden: bool,
    existing_beams: usize,
}

fn partial_beat_measure_spans(xml: &str) -> Vec<PartialBeatMeasureSpan> {
    let mut spans = Vec::new();
    let mut scan = 0_usize;
    while let Some(rel_start) = xml[scan..].find("<measure") {
        let start = scan + rel_start;
        let Some(open_end_rel) = xml[start..].find('>') else { break; };
        let content_start = start + open_end_rel + 1;
        let Some(close_rel) = xml[content_start..].find("</measure>") else { break; };
        let end = content_start + close_rel + "</measure>".len();
        spans.push(PartialBeatMeasureSpan { start, end });
        scan = end;
    }
    spans
}

fn partial_beat_raw_notes(text: &str) -> Vec<PartialBeatRawNote> {
    let mut notes = Vec::new();
    let mut scan = 0_usize;
    while let Some(rel_start) = text[scan..].find("<note") {
        let start = scan + rel_start;
        let Some(open_end_rel) = text[start..].find('>') else { break; };
        let start_tag = &text[start..start + open_end_rel + 1];
        if start_tag.ends_with("/>") {
            let end = start + open_end_rel + 1;
            notes.push(PartialBeatRawNote { start, end, text: text[start..end].to_string() });
            scan = end;
            continue;
        }
        let content_start = start + open_end_rel + 1;
        let Some(close_rel) = text[content_start..].find("</note>") else { break; };
        let end = content_start + close_rel + "</note>".len();
        notes.push(PartialBeatRawNote { start, end, text: text[start..end].to_string() });
        scan = end;
    }
    notes
}

fn partial_beat_note_infos(measure_text: &str, raw_notes: &[PartialBeatRawNote]) -> Vec<PartialBeatNoteInfo> {
    let mut infos = Vec::new();
    let mut cursor = 0_i64;
    let mut cursor_run = Some(0_usize);
    let mut last_non_chord_start = 0_i64;
    let mut note_pos = 0_usize;
    let mut scan = 0_usize;
    loop {
        let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
        let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
        let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
        let mut candidates = Vec::new();
        if let Some(p) = next_note { candidates.push((p, "note")); }
        if let Some(p) = next_backup { candidates.push((p, "backup")); }
        if let Some(p) = next_forward { candidates.push((p, "forward")); }
        candidates.sort_by_key(|(p, _)| *p);
        let Some((pos, kind)) = candidates.first().copied() else { break; };
        match kind {
            "backup" => {
                let Some(end) = measure_text[pos..].find("</backup>").map(|e| pos + e + "</backup>".len()) else { break; };
                let rewind = partial_beat_first_i64(&measure_text[pos..end], "duration").unwrap_or(0);
                cursor_run = cursor_run.and_then(|run| {
                    (rewind > 0 && rewind == cursor).then_some(run + 1)
                });
                cursor -= rewind.max(0);
                scan = end;
            }
            "forward" => {
                let Some(end) = measure_text[pos..].find("</forward>").map(|e| pos + e + "</forward>".len()) else { break; };
                cursor += partial_beat_first_i64(&measure_text[pos..end], "duration").unwrap_or(0).max(0);
                scan = end;
            }
            "note" => {
                let Some(raw) = raw_notes.get(note_pos) else { break; };
                note_pos += 1;
                let duration = partial_beat_first_i64(&raw.text, "duration").unwrap_or(0).max(0);
                let is_chord = raw.text.contains("<chord") || raw.text.contains("<chord/");
                let is_rest = raw.text.contains("<rest") || raw.text.contains("<rest/");
                let is_hidden = raw.text.contains("print-object=\"no\"");
                let start = if is_chord { last_non_chord_start } else { cursor };
                let staff = partial_beat_first_text(&raw.text, "staff").unwrap_or_else(|| "1".to_string());
                let voice = partial_beat_first_text(&raw.text, "voice").unwrap_or_else(|| "1".to_string());
                let note_type = partial_beat_first_text(&raw.text, "type").unwrap_or_default();
                let existing_beams = raw.text.match_indices("<beam").count();
                infos.push(PartialBeatNoteInfo {
                    index: raw.start,
                    cursor_run,
                    start,
                    duration,
                    staff,
                    voice,
                    note_type,
                    is_rest,
                    is_chord,
                    is_hidden,
                    existing_beams,
                });
                if !is_chord {
                    last_non_chord_start = start;
                    cursor += duration;
                }
                scan = raw.end;
            }
            _ => break,
        }
    }
    infos
}

fn partial_beat_level(info: &PartialBeatNoteInfo) -> usize {
    match info.note_type.as_str() {
        "eighth" => 1,
        "16th" => 2,
        "32nd" => 3,
        "64th" => 4,
        _ => 0,
    }
}

fn partial_beat_is_beam_candidate(info: &PartialBeatNoteInfo) -> bool {
    !info.is_rest
        && !info.is_hidden
        && !info.is_chord
        && info.duration > 0
        && matches!(info.note_type.as_str(), "eighth" | "16th" | "32nd" | "64th")
}

fn partial_beat_first_i64(text: &str, tag: &str) -> Option<i64> {
    partial_beat_first_text(text, tag).and_then(|value| value.trim().parse::<i64>().ok())
}

fn partial_beat_first_text(text: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)? + open.len();
    let end = text[start..].find(&close)? + start;
    Some(text[start..end].trim().to_string())
}

fn partial_beat_measure_number(measure_text: &str) -> Option<String> {
    let marker = "<measure";
    let start = measure_text.find(marker)?;
    let open_end = measure_text[start..].find('>')? + start;
    let open = &measure_text[start..open_end];
    let key = "number=\"";
    let key_start = open.find(key)? + key.len();
    let key_end = open[key_start..].find('\"')? + key_start;
    Some(open[key_start..key_end].to_string())
}

/// Observe clef positions in the same measure cursor used for the beam attacks.
fn metric_beam_clef_boundaries(measure: &str) -> Option<BTreeMap<String, Vec<i64>>> {
    let mut boundaries = BTreeMap::new();
    if !measure.contains("<clef") {
        return Some(boundaries);
    }
    let document = roxmltree::Document::parse(measure).ok()?;
    let mut cursor = 0_i64;
    let mut indications = BTreeMap::new();
    for node in document.root_element().children().filter(roxmltree::Node::is_element) {
        let duration = || {
            node.children()
                .find(|child| child.has_tag_name("duration"))
                .and_then(|child| child.text())
                .and_then(|text| text.trim().parse::<i64>().ok())
                .unwrap_or(0)
                .max(0)
        };
        match node.tag_name().name() {
            "backup" => cursor = cursor.checked_sub(duration())?,
            "forward" => cursor = cursor.checked_add(duration())?,
            "note" if !node.children().any(|child| child.has_tag_name("chord")) => {
                cursor = cursor.checked_add(duration())?;
            }
            "attributes" => {
                for clef in node.children().filter(|child| child.has_tag_name("clef")) {
                    let staff = clef.attribute("number").unwrap_or("1");
                    let field = |name| {
                        clef.children()
                            .find(|child| child.has_tag_name(name))
                            .and_then(|child| child.text())
                            .map(str::trim)
                    };
                    indications.insert((staff, cursor), (
                        field("sign")?,
                        field("line").and_then(|text| text.parse::<i64>().ok()),
                        field("clef-octave-change").and_then(|text| text.parse::<i64>().ok()).unwrap_or(0),
                        clef.attribute("print-object") != Some("no"),
                    ));
                }
            }
            _ => {}
        }
    }
    let mut previous = BTreeMap::new();
    for ((staff, at), (sign, line, octave, visible)) in indications {
        let state = (sign, line, octave);
        if previous.insert(staff, state) != Some(state) && visible && at > 0 {
            boundaries.entry(staff.to_owned()).or_insert_with(Vec::new).push(at);
        }
    }
    Some(boundaries)
}
