// v0.10.726 real crate-root module split from the old score grouping.
// No behavior change: original module body, visibility and call paths are preserved.

    use crate::model::{Event, EventKind, ExpectedDuration, Measure, Part, Pitch, Score};
    use anyhow::{anyhow, Context};
    use roxmltree::{Document, Node};
    use std::collections::BTreeMap;
    
    pub fn parse_musicxml(xml: &str) -> anyhow::Result<Score> {
        let sanitized_xml = strip_doctype(xml);
        let doc = Document::parse(&sanitized_xml).context("XML ist nicht wohlgeformt")?;
        let root = doc.root_element();
    
        if !is_el(root, "score-partwise") && !is_el(root, "score-timewise") {
            return Err(anyhow!(
                "Erwartet score-partwise oder score-timewise, gefunden: {}",
                root.tag_name().name()
            ));
        }
    
        if is_el(root, "score-timewise") {
            return Err(anyhow!(
                "score-timewise wird in dieser MVP-Version noch nicht unterstützt; bitte als score-partwise exportieren"
            ));
        }
    
        let title = first_descendant_text(root, "movement-title")
            .or_else(|| first_descendant_text(root, "work-title"));
        let part_names = parse_part_names(root);
        let mut parts = Vec::new();
    
        for part_node in root.children().filter(|n| is_el(*n, "part")) {
            parts.push(parse_part(part_node, &part_names)?);
        }
    
        if parts.is_empty() {
            return Err(anyhow!("Keine <part>-Elemente im MusicXML gefunden"));
        }
    
        Ok(Score { title, parts })
    }
    
    fn parse_part_names(root: Node<'_, '_>) -> BTreeMap<String, String> {
        let mut names = BTreeMap::new();
        for score_part in root.descendants().filter(|n| is_el(*n, "score-part")) {
            if let Some(id) = score_part.attribute("id") {
                let name = child_text(score_part, "part-name").unwrap_or_else(|| id.to_string());
                names.insert(id.to_string(), normalize_ws(&name));
            }
        }
        names
    }
    
    fn parse_part(part_node: Node<'_, '_>, part_names: &BTreeMap<String, String>) -> anyhow::Result<Part> {
        let id = part_node.attribute("id").unwrap_or("P?").to_string();
        let name = part_names.get(&id).cloned().unwrap_or_else(|| id.clone());
    
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        let mut measures = Vec::new();
    
        for (idx, measure_node) in part_node
            .children()
            .filter(|n| is_el(*n, "measure"))
            .enumerate()
        {
            let parsed = parse_measure(
                measure_node,
                idx + 1,
                divisions,
                beats,
                beat_type,
            )?;
            divisions = parsed.divisions;
            beats = parsed.beats;
            beat_type = parsed.beat_type;
            measures.push(parsed);
        }
    
        Ok(Part { id, name, measures })
    }
    
    fn parse_measure(
        measure_node: Node<'_, '_>,
        fallback_number: usize,
        inherited_divisions: i64,
        inherited_beats: i64,
        inherited_beat_type: i64,
    ) -> anyhow::Result<Measure> {
        let number = measure_node
            .attribute("number")
            .map_or_else(|| fallback_number.to_string(), ToOwned::to_owned);
    
        let mut divisions = inherited_divisions;
        let mut beats = inherited_beats;
        let mut beat_type = inherited_beat_type;
        let mut cursor = 0_i64;
        let mut cursor_min = 0_i64;
        let mut cursor_max = 0_i64;
        let mut last_non_chord_start = 0_i64;
        let mut events = Vec::new();
    
        for child in measure_node.children().filter(|n| n.is_element()) {
            if is_el(child, "attributes") {
                if let Some(d) = child_text(child, "divisions").and_then(|s| parse_i64(&s)) {
                    divisions = d.max(1);
                }
                if let Some(time) = child.children().find(|n| is_el(*n, "time")) {
                    if let Some(b) = child_text(time, "beats").and_then(|s| parse_i64(&s)) {
                        beats = b.max(1);
                    }
                    if let Some(bt) = child_text(time, "beat-type").and_then(|s| parse_i64(&s)) {
                        beat_type = bt.max(1);
                    }
                }
            } else if is_el(child, "backup") {
                let duration = child_text(child, "duration")
                    .and_then(|s| parse_i64(&s))
                    .unwrap_or(0)
                    .max(0);
                cursor -= duration;
                cursor_min = cursor_min.min(cursor);
                cursor_max = cursor_max.max(cursor);
            } else if is_el(child, "forward") {
                let duration = child_text(child, "duration")
                    .and_then(|s| parse_i64(&s))
                    .unwrap_or(0)
                    .max(0);
                cursor += duration;
                cursor_min = cursor_min.min(cursor);
                cursor_max = cursor_max.max(cursor);
            } else if is_el(child, "note") {
                let duration = child_text(child, "duration")
                    .and_then(|s| parse_i64(&s))
                    .unwrap_or(0)
                    .max(0);
                let is_chord = child.children().any(|n| is_el(n, "chord"));
                let is_rest = child.children().any(|n| is_el(n, "rest"));
                let voice = child_text(child, "voice")
                    .map(|s| normalize_ws(&s))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "1".to_string());
                let staff = child_text(child, "staff")
                    .map(|s| normalize_ws(&s))
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "1".to_string());
                let start = if is_chord { last_non_chord_start } else { cursor };
                let kind = if is_rest { EventKind::Rest } else { EventKind::Note };
                let pitch = if is_rest { None } else { parse_pitch(child) };
    
                let tie_stop = child.descendants().any(|node| {
                    (is_el(node, "tie") || is_el(node, "tied"))
                        && node.attribute("type") == Some("stop")
                });
                let visible = child.attribute("print-object") != Some("no");
                let note_type = child_text(child, "type")
                    .map(|s| normalize_ws(&s))
                    .filter(|s| !s.is_empty());
                let dots = child.children().filter(|n| is_el(*n, "dot")).count();
    
                events.push(Event {
                    kind,
                    staff,
                    voice,
                    start,
                    duration,
                    pitch,
                    is_chord,
                    tie_stop,
                    visible,
                    note_type,
                    dots,
                });
    
                if !is_chord {
                    last_non_chord_start = start;
                    cursor += duration;
                    cursor_min = cursor_min.min(cursor);
                    cursor_max = cursor_max.max(cursor);
                }
            }
        }
    
        let expected = ExpectedDuration::new(divisions, beats, beat_type);
        Ok(Measure {
            number,
            divisions,
            beats,
            beat_type,
            expected,
            cursor_min,
            cursor_max,
            events,
        })
    }
    
    fn parse_pitch(note: Node<'_, '_>) -> Option<Pitch> {
        let pitch_node = note.children().find(|n| is_el(*n, "pitch"))?;
        let step = child_text(pitch_node, "step").map(|s| normalize_ws(&s))?;
        let alter = child_text(pitch_node, "alter")
            .and_then(|s| s.trim().parse::<i32>().ok())
            .unwrap_or(0);
        let octave = child_text(pitch_node, "octave")
            .and_then(|s| s.trim().parse::<i32>().ok())?;
        Some(Pitch::new(step, alter, octave))
    }
    
    fn child_text(node: Node<'_, '_>, tag: &str) -> Option<String> {
        node.children()
            .find(|n| is_el(*n, tag))
            .and_then(|n| n.text())
            .map(ToOwned::to_owned)
    }
    
    fn first_descendant_text(node: Node<'_, '_>, tag: &str) -> Option<String> {
        node.descendants()
            .find(|n| is_el(*n, tag))
            .and_then(|n| n.text())
            .map(normalize_ws)
            .filter(|s| !s.is_empty())
    }
    
    
    fn strip_doctype(xml: &str) -> String {
        let mut cleaned = String::with_capacity(xml.len());
        let mut cursor = 0usize;
    
        while let Some(relative_start) = xml[cursor..].find("<!DOCTYPE") {
            let start = cursor + relative_start;
            cleaned.push_str(&xml[cursor..start]);
            let rest = &xml[start..];
            let Some(end_rel) = find_doctype_end(rest) else {
                return cleaned;
            };
            cursor = start + end_rel;
        }
        cleaned.push_str(&xml[cursor..]);
        cleaned
    }
    
    fn find_doctype_end(text: &str) -> Option<usize> {
        let bytes = text.as_bytes();
        let mut in_quote: Option<u8> = None;
        let mut internal_subset_depth = 0_i32;
    
        for (idx, &byte) in bytes.iter().enumerate() {
            match in_quote {
                Some(q) if byte == q => in_quote = None,
                Some(_) => {}
                None if byte == b'\'' || byte == b'\"' => in_quote = Some(byte),
                None if byte == b'[' => internal_subset_depth += 1,
                None if byte == b']' && internal_subset_depth > 0 => internal_subset_depth -= 1,
                None if byte == b'>' && internal_subset_depth == 0 => return Some(idx + 1),
                None => {}
            }
        }
    
        None
    }
    
    fn parse_i64(s: &str) -> Option<i64> {
        s.trim().parse::<i64>().ok()
    }
    
    fn normalize_ws(s: &str) -> String {
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    }
    
    fn is_el(node: Node<'_, '_>, name: &str) -> bool {
        node.is_element() && node.tag_name().name() == name
    }

#[cfg(test)]
mod doctype_readback_tests {
    use super::strip_doctype;

    #[test]
    fn strip_doctype_keeps_xml_without_declaration_unchanged() {
        let xml = "<score-partwise><part-list/></score-partwise>";
        assert_eq!(strip_doctype(xml), xml);
    }

    #[test]
    fn strip_doctype_removes_simple_and_multiple_declarations_without_other_changes() {
        let xml = "before<!DOCTYPE score-partwise SYSTEM 'one.dtd'>middle<!DOCTYPE score-partwise SYSTEM 'two.dtd'>after";
        assert_eq!(strip_doctype(xml), "beforemiddleafter");
    }

    #[test]
    fn strip_doctype_respects_internal_subset_and_quotes() {
        let xml = r#"a<!DOCTYPE score-partwise [<!ENTITY x ">">]><score-partwise>&x;</score-partwise>"#;
        assert_eq!(strip_doctype(xml), "a<score-partwise>&x;</score-partwise>");
    }

    #[test]
    fn strip_doctype_malformed_declaration_preserves_old_fail_closed_truncation() {
        let xml = "prefix<!DOCTYPE score-partwise [suffix";
        assert_eq!(strip_doctype(xml), "prefix");
    }
    #[test]
    fn parse_musicxml_transports_tie_stop_as_continuity_state() {
        let xml = "<score-partwise version=\"4.0\">\n  <part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list>\n  <part id=\"P1\"><measure number=\"\
1\">\n    <attributes><divisions>48</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>\n    <note><pitch><step>G</step><alter>1</alter><octave>2</octave></pitch><duration>48</duration><tie type=\"stop\"/><voice>2</voice><staff>2</staff></note>\n    <note><pitch><step>G</step><alter>1</alter><octave>4</octave></pitch><duration>48</duration><tie type=\"start\"/><voice>2</voice><staff>2</staff></note>\n  </measure></part>\n</score-partwise>";
        let score = super::parse_musicxml(xml).expect("fixture must parse");
        let events = &score.parts[0].measures[0].events;
        assert!(events[0].tie_stop);
        assert!(!events[1].tie_stop);
    }

}

