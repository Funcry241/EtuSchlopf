use roxmltree::{Document, Node};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SemanticEventKey {
    part_ordinal: usize,
    measure_ordinal: usize,
    voice: String,
    staff: String,
    onset: i64,
    duration: i64,
    hidden: bool,
    rest: bool,
    pitch_identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EncodingEventKey {
    semantic: SemanticEventKey,
    chord_member: bool,
}

#[derive(Debug, Clone, Default)]
struct ParsedEventInventory {
    parse_ok: bool,
    cursor_balance_ok: bool,
    semantic_events: Vec<SemanticEventKey>,
    sounding_events: Vec<SemanticEventKey>,
    encoding_events: Vec<EncodingEventKey>,
    chord_members: usize,
}

#[derive(Debug, Clone, Default)]
pub struct AuthorizedMaterializationEquivalenceAudit {
    #[cfg(test)]
    semantic_event_delta_rows: usize,
    #[cfg(test)]
    sounding_identity_delta_rows: usize,
    #[cfg(test)]
    serialization_encoding_delta_rows: usize,
    #[cfg(test)]
    chord_removals: usize,
    #[cfg(test)]
    artificial_chord_additions: usize,
    #[cfg(test)]
    verified_checkpoint_direction_only: bool,
    materialization_equivalence_green: bool,
}

impl AuthorizedMaterializationEquivalenceAudit {
    pub fn green(&self) -> bool {
        self.materialization_equivalence_green
    }


}


pub(super) fn validate_serialized_temporal_materialization(xml: &str) -> std::io::Result<()> {
    let document = Document::parse(xml).map_err(|error| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("serialized temporal materialization parse failed: {error}"),
        )
    })?;

    for part in document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "part")
    {
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        for measure in part
            .children()
            .filter(|node| node.is_element() && node.tag_name().name() == "measure")
        {
            let mut cursor = 0_i64;
            let mut last_anchor: Option<(i64, i64, String, String, String)> = None;
            for child in measure.children().filter(|node| node.is_element()) {
                match child.tag_name().name() {
                    "attributes" => {
                        if let Some(value) = child_i64(child, "divisions")
                            && value > 0
                        {
                            divisions = value;
                        }
                        if let Some(time) = child
                            .children()
                            .find(|node| node.is_element() && node.tag_name().name() == "time")
                        {
                            if let Some(value) = child_i64(time, "beats")
                                && value > 0
                            {
                                beats = value;
                            }
                            if let Some(value) = child_i64(time, "beat-type")
                                && value > 0
                            {
                                beat_type = value;
                            }
                        }
                    }
                    "note" => {
                        let duration = child_i64(child, "duration").unwrap_or(0).max(0);
                        let chord_member = has_child(child, "chord");
                        let expected_duration = divisions
                            .saturating_mul(beats)
                            .saturating_mul(4)
                            / beat_type.max(1);
                        if chord_member {
                            let Some((
                                anchor_onset,
                                anchor_duration,
                                anchor_pitch,
                                anchor_voice,
                                anchor_staff,
                            )) = last_anchor.as_ref()
                            else {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "serialized temporal materialization contains an orphan chord member",
                                ));
                            };
                            if duration > *anchor_duration {
                                let part_id = part.attribute("id").unwrap_or("?");
                                let measure_number = measure.attribute("number").unwrap_or("?");
                                let chord_pitch = pitch_identity(child);
                                let chord_voice =
                                    child_text(child, "voice").unwrap_or_else(|| "?".to_string());
                                let chord_staff =
                                    child_text(child, "staff").unwrap_or_else(|| "?".to_string());
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    format!(
                                        "serialized temporal materialization contains a chord member longer than its cursor-carrying anchor: part={part_id}; measure={measure_number}; anchor_onset={anchor_onset}; anchor_duration={anchor_duration}; anchor_pitch={anchor_pitch}; anchor_voice={anchor_voice}; anchor_staff={anchor_staff}; chord_duration={duration}; chord_pitch={chord_pitch}; chord_voice={chord_voice}; chord_staff={chord_staff}"
                                    ),
                                ));
                            }
                            if expected_duration > 0
                                && anchor_onset.saturating_add(duration) > expected_duration
                            {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "serialized temporal materialization contains a chord note ending beyond the measure extent",
                                ));
                            }
                        } else {
                            let onset = cursor;
                            let end = onset.saturating_add(duration);
                            if expected_duration > 0 && end > expected_duration {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "serialized temporal materialization contains a note ending beyond the measure extent",
                                ));
                            }
                            last_anchor = Some((
                                onset,
                                duration,
                                pitch_identity(child),
                                child_text(child, "voice").unwrap_or_else(|| "?".to_string()),
                                child_text(child, "staff").unwrap_or_else(|| "?".to_string()),
                            ));
                            cursor = end;
                        }
                    }
                    "backup" => {
                        let duration = child_i64(child, "duration").unwrap_or(0).max(0);
                        if duration > cursor {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "serialized temporal materialization contains a backup underflow",
                            ));
                        }
                        cursor -= duration;
                        last_anchor = None;
                    }
                    "forward" => {
                        let duration = child_i64(child, "duration").unwrap_or(0).max(0);
                        cursor = cursor.saturating_add(duration);
                        let expected_duration = divisions
                            .saturating_mul(beats)
                            .saturating_mul(4)
                            / beat_type.max(1);
                        if expected_duration > 0 && cursor > expected_duration {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "serialized temporal materialization contains a forward beyond the measure extent",
                            ));
                        }
                        last_anchor = None;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

pub fn audit_authorized_materialization_equivalence(
    source_xml: &str,
    candidate_xml: &str,
    _source_stage: &str,
    historical: Option<&str>,
    direction: Option<&super::orchestrator_artifact_sink_authority::VerifiedDirectionOnlyCandidateSnapshot>,
) -> AuthorizedMaterializationEquivalenceAudit {
    let before = parse_event_inventory(source_xml);
    let after = parse_event_inventory(candidate_xml);
    let semantic_event_delta_rows =
        sorted_multiset_delta_rows(&before.semantic_events, &after.semantic_events);
    let sounding_identity_delta_rows =
        sorted_multiset_delta_rows(&before.sounding_events, &after.sounding_events);
    let serialization_encoding_delta_rows =
        sorted_multiset_delta_rows(&before.encoding_events, &after.encoding_events);
    let chord_removals = before.chord_members.saturating_sub(after.chord_members);
    let artificial_chord_additions = after.chord_members.saturating_sub(before.chord_members);
    let note_count_before = before.semantic_events.len();
    let note_count_after = after.semantic_events.len();
    let candidate_changed = source_xml != candidate_xml;
    let semantic_event_invariant_green = before.parse_ok
        && after.parse_ok
        && before.cursor_balance_ok
        && after.cursor_balance_ok
        && note_count_before == note_count_after
        && semantic_event_delta_rows == 0
        && sounding_identity_delta_rows == 0;
    let encoding_normalization_proof_green = semantic_event_invariant_green
        && chord_removals > 0
        && serialization_encoding_delta_rows == chord_removals.saturating_mul(2)
        && artificial_chord_additions == 0;
    let verified_checkpoint_direction_only = semantic_event_invariant_green
        && serialization_encoding_delta_rows == 0
        && chord_removals == 0
        && artificial_chord_additions == 0
        && historical.zip(direction).is_some_and(|(base, witness)| witness.matches(base, candidate_xml));
    let materialization_equivalence_green = candidate_changed
        && semantic_event_invariant_green
        && (encoding_normalization_proof_green || verified_checkpoint_direction_only);

    AuthorizedMaterializationEquivalenceAudit {
        #[cfg(test)]
        semantic_event_delta_rows,
        #[cfg(test)]
        sounding_identity_delta_rows,
        #[cfg(test)]
        serialization_encoding_delta_rows,
        #[cfg(test)]
        chord_removals,
        #[cfg(test)]
        artificial_chord_additions,
        #[cfg(test)]
        verified_checkpoint_direction_only,
        materialization_equivalence_green,
    }
}

fn parse_event_inventory(xml: &str) -> ParsedEventInventory {
    let Ok(document) = Document::parse(xml) else {
        return ParsedEventInventory::default();
    };
    let mut inventory = ParsedEventInventory {
        parse_ok: true,
        cursor_balance_ok: true,
        ..ParsedEventInventory::default()
    };

    for (part_ordinal, part) in document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "part")
        .enumerate()
    {
        for (measure_ordinal, measure) in part
            .children()
            .filter(|node| node.is_element() && node.tag_name().name() == "measure")
            .enumerate()
        {
            let mut cursor = 0_i64;
            let mut last_anchor_onset: Option<i64> = None;
            for child in measure.children().filter(|node| node.is_element()) {
                match child.tag_name().name() {
                    "note" => {
                        let duration = child_i64(child, "duration").unwrap_or(0);
                        let chord_member = has_child(child, "chord");
                        let onset = if chord_member {
                            if let Some(onset) = last_anchor_onset {
                                onset
                            } else {
                                inventory.cursor_balance_ok = false;
                                cursor
                            }
                        } else {
                            cursor
                        };
                        let semantic = SemanticEventKey {
                            part_ordinal,
                            measure_ordinal,
                            voice: child_text(child, "voice").unwrap_or_default(),
                            staff: child_text(child, "staff").unwrap_or_default(),
                            onset,
                            duration,
                            hidden: child.attribute("print-object") == Some("no"),
                            rest: has_child(child, "rest"),
                            pitch_identity: pitch_identity(child),
                        };
                        if !semantic.hidden && !semantic.rest {
                            inventory.sounding_events.push(semantic.clone());
                        }
                        inventory.encoding_events.push(EncodingEventKey {
                            semantic: semantic.clone(),
                            chord_member,
                        });
                        inventory.semantic_events.push(semantic);
                        if chord_member {
                            inventory.chord_members += 1;
                        } else {
                            last_anchor_onset = Some(onset);
                            cursor = cursor.saturating_add(duration);
                        }
                    }
                    "backup" => {
                        let duration = child_i64(child, "duration").unwrap_or(0);
                        if duration > cursor {
                            inventory.cursor_balance_ok = false;
                            cursor = 0;
                        } else {
                            cursor -= duration;
                        }
                        last_anchor_onset = None;
                    }
                    "forward" => {
                        let duration = child_i64(child, "duration").unwrap_or(0);
                        cursor = cursor.saturating_add(duration);
                        last_anchor_onset = None;
                    }
                    _ => {}
                }
            }
        }
    }

    inventory.semantic_events.sort();
    inventory.sounding_events.sort();
    inventory.encoding_events.sort();
    inventory
}

fn pitch_identity(note: Node<'_, '_>) -> String {
    if let Some(pitch) = note
        .children()
        .find(|child| child.is_element() && child.tag_name().name() == "pitch")
    {
        return format!(
            "pitch:{}:{}:{}",
            child_text(pitch, "step").unwrap_or_default(),
            child_text(pitch, "alter").unwrap_or_else(|| "0".to_string()),
            child_text(pitch, "octave").unwrap_or_default(),
        );
    }
    if let Some(unpitched) = note
        .children()
        .find(|child| child.is_element() && child.tag_name().name() == "unpitched")
    {
        return format!(
            "unpitched:{}:{}",
            child_text(unpitched, "display-step").unwrap_or_default(),
            child_text(unpitched, "display-octave").unwrap_or_default(),
        );
    }
    if let Some(rest) = note
        .children()
        .find(|child| child.is_element() && child.tag_name().name() == "rest")
    {
        return format!(
            "rest:{}:{}",
            child_text(rest, "display-step").unwrap_or_default(),
            child_text(rest, "display-octave").unwrap_or_default(),
        );
    }
    "unknown".to_string()
}

fn child_text(node: Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
        .and_then(|child| child.text())
        .map(|text| text.trim().to_string())
}

fn child_i64(node: Node<'_, '_>, name: &str) -> Option<i64> {
    child_text(node, name)?.parse::<i64>().ok()
}

fn has_child(node: Node<'_, '_>, name: &str) -> bool {
    node.children()
        .any(|child| child.is_element() && child.tag_name().name() == name)
}

fn sorted_multiset_delta_rows<T: Ord>(before: &[T], after: &[T]) -> usize {
    let mut before_index = 0_usize;
    let mut after_index = 0_usize;
    let mut delta = 0_usize;
    while before_index < before.len() && after_index < after.len() {
        match before[before_index].cmp(&after[after_index]) {
            std::cmp::Ordering::Less => {
                delta += 1;
                before_index += 1;
            }
            std::cmp::Ordering::Greater => {
                delta += 1;
                after_index += 1;
            }
            std::cmp::Ordering::Equal => {
                before_index += 1;
                after_index += 1;
            }
        }
    }
    delta + before.len().saturating_sub(before_index) + after.len().saturating_sub(after_index)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r#"<score-partwise><part id="P1"><measure number="x"><attributes><divisions>24</divisions></attributes><note><pitch><step>A</step><octave>3</octave></pitch><duration>24</duration><voice>v</voice><staff>s</staff></note><note print-object="no"><rest/><duration>24</duration><voice>v</voice><staff>s</staff></note><note><chord/><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>v</voice><staff>s</staff></note><backup><duration>48</duration></backup></measure></part></score-partwise>"#;
    const CANDIDATE: &str = r#"<score-partwise><part id="P1"><measure number="renamed"><attributes><divisions>24</divisions></attributes><note><pitch><step>A</step><octave>3</octave></pitch><duration>24</duration><voice>v</voice><staff>s</staff></note><note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>v</voice><staff>s</staff></note><backup><duration>48</duration></backup><forward><duration>24</duration></forward><note print-object="no"><rest/><duration>24</duration><voice>v</voice><staff>s</staff></note><backup><duration>48</duration></backup></measure></part></score-partwise>"#;


    #[test]
    fn serialized_temporal_materialization_accepts_equal_duration_chord_members() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><rest/><duration>12</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        assert!(validate_serialized_temporal_materialization(xml).is_ok());
    }

    #[test]
    fn serialized_temporal_materialization_rejects_longer_chord_member() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>8</duration><voice>1</voice><staff>1</staff></note><note><rest/><duration>12</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        let error = validate_serialized_temporal_materialization(xml).unwrap_err();
        let message = error.to_string();
        assert!(message.contains("part=P1"));
        assert!(message.contains("measure=1"));
        assert!(message.contains("anchor_onset=0"));
        assert!(message.contains("anchor_duration=4"));
        assert!(message.contains("anchor_pitch=pitch:C:0:4"));
        assert!(message.contains("anchor_voice=1"));
        assert!(message.contains("anchor_staff=1"));
        assert!(message.contains("chord_duration=8"));
        assert!(message.contains("chord_pitch=pitch:E:0:4"));
        assert!(message.contains("chord_voice=1"));
        assert!(message.contains("chord_staff=1"));
    }

    #[test]
    fn serialized_temporal_materialization_rejects_backup_underflow() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff></note><backup><duration>8</duration></backup></measure></part></score-partwise>";
        assert!(validate_serialized_temporal_materialization(xml).is_err());
    }


    #[test]
    fn serialized_temporal_materialization_rejects_note_end_overflow() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>20</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
        assert!(validate_serialized_temporal_materialization(xml).is_err());
    }

    #[test]
    fn serialized_temporal_materialization_rejects_forward_overflow() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><forward><duration>20</duration></forward></measure></part></score-partwise>";
        assert!(validate_serialized_temporal_materialization(xml).is_err());
    }

    #[test]
    fn actual_candidate_equivalence_accepts_only_bound_chord_normalization() {
        let audit = audit_authorized_materialization_equivalence(SOURCE, CANDIDATE, "fixture", None, None);
        assert!(audit.green());
        assert_eq!(audit.semantic_event_delta_rows, 0);
        assert_eq!(audit.sounding_identity_delta_rows, 0);
        assert_eq!(audit.serialization_encoding_delta_rows, 2);
        assert_eq!(audit.chord_removals, 1);
        assert_eq!(audit.artificial_chord_additions, 0);
    }

    #[test]
    fn sounding_pitch_change_blocks_materialization() {
        let changed = CANDIDATE.replace("<step>C</step>", "<step>D</step>");
        let audit = audit_authorized_materialization_equivalence(SOURCE, &changed, "fixture", None, None);
        assert!(!audit.green());
        assert!(audit.semantic_event_delta_rows > 0);
        assert!(audit.sounding_identity_delta_rows > 0);
    }

    #[test]
    fn hidden_carrier_onset_change_blocks_materialization() {
        let changed = CANDIDATE.replace(
            "<forward><duration>24</duration></forward>",
            "<forward><duration>12</duration></forward>",
        );
        let audit = audit_authorized_materialization_equivalence(SOURCE, &changed, "fixture", None, None);
        assert!(!audit.green());
        assert!(audit.semantic_event_delta_rows > 0);
    }

    fn temporary_render_path(label: &str) -> std::path::PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("etuschlopf_{label}_{nonce}.render.musicxml"))
    }

    #[test]
    fn registered_verified_checkpoint_direction_only_candidate_is_equivalent() {
        let path = temporary_render_path("direction_only_equivalence");
        let base = r#"<?xml version="1.0"?><score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="x"><direction><direction-type><dynamics><mf/></dynamics></direction-type><staff>1</staff></direction><note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff><beam number="1">begin</beam></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><staff>1</staff><beam number="1">end</beam></note></measure></part></score-partwise>"#;
        let candidate = base.replace(
            "<direction><direction-type><dynamics><mf/></dynamics></direction-type><staff>1</staff></direction>",
            "<direction><direction-type><dynamics><mf/></dynamics></direction-type><offset>24</offset><staff>1</staff></direction>",
        );
        let (result, receipt) = super::super::orchestrator_artifact_sink_authority::write_external_feedback_closure_checkpoint_authorized(&path, base);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe); result.unwrap();
        let witness = super::super::orchestrator_artifact_sink_authority::verify_direction_only_candidate(base, &candidate).expect("explicit direction witness");
        let audit = audit_authorized_materialization_equivalence(base, &candidate, "fixture", Some(base), Some(&witness));
        assert!(audit.green());
        assert!(audit.verified_checkpoint_direction_only);
        assert_eq!(audit.serialization_encoding_delta_rows, 0);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn unregistered_direction_only_change_remains_blocked() {
        let base = r#"<score-partwise><part id="P1"><measure number="x"><direction><direction-type><words>x</words></direction-type><staff>1</staff></direction><note><pitch><step>A</step><octave>3</octave></pitch><duration>24</duration><voice>v</voice><staff>1</staff></note></measure></part></score-partwise>"#;
        let changed = base.replace(
            "<direction><direction-type><words>x</words></direction-type><staff>1</staff></direction>",
            "<direction><direction-type><words>x</words></direction-type><offset>24</offset><staff>1</staff></direction>",
        );
        let audit = audit_authorized_materialization_equivalence(base, &changed, "fixture", None, None);
        assert!(!audit.green());
        assert!(!audit.verified_checkpoint_direction_only);
    }
}
