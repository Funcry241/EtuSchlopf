//! Audit-only readback of voice-run integrity in the final MusicXML artifacts.
//!
//! MusicXML serializes the voices of one measure as consecutive runs separated
//! by `<backup>`. A note that carries a voice label different from the run it
//! is written in is counted twice by importers that group events by voice
//! label: its own voice is overfull and the run voice keeps a hole. This
//! module only reads the final Master and Render MusicXML, reports such notes
//! with their staff, pitch and the simultaneous notes of the other staff, and
//! owns no write path, no ranking and no musical authority.

use roxmltree::{Document, Node};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ForeignVoiceNote {
    part_ordinal: usize,
    measure: String,
    onset: i64,
    measure_length: i64,
    duration: i64,
    staff: u8,
    voice: String,
    run_voice: String,
    run_staff: u8,
    pitch: String,
    voice_occupancy: i64,
    crossing: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct VoiceRunAudit {
    source_read: bool,
    parse_green: bool,
    measures: usize,
    foreign_voice_notes: Vec<ForeignVoiceNote>,
    chord_voice_mismatches: usize,
    visible_pitched_chord_voice_mismatches: usize,
    hidden_rest_carrier_voice_mismatches: usize,
    other_chord_voice_mismatches: usize,
    overfull_voice_measures: usize,
    crossing_foreign_notes: usize,
    voice_timeline_collisions: usize,
}

#[derive(Debug, Clone)]
struct Event {
    onset: i64,
    duration: i64,
    staff: u8,
    voice: String,
    midi: Option<i32>,
    pitch: String,
    run_index: usize,
    chord: bool,
}

fn inspect_path(path: &Path) -> VoiceRunAudit {
    let Ok(xml) = fs::read_to_string(path) else {
        return VoiceRunAudit::default();
    };
    inspect_xml(&xml)
}

fn inspect_xml(xml: &str) -> VoiceRunAudit {
    let mut audit = VoiceRunAudit {
        source_read: true,
        ..VoiceRunAudit::default()
    };
    let Ok(document) = Document::parse(xml) else {
        return audit;
    };
    audit.parse_green = true;
    audit.voice_timeline_collisions = collisions_in_document(&document).len();
    for (part_index, part) in document
        .descendants()
        .filter(|node| node.has_tag_name("part"))
        .enumerate()
    {
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.has_tag_name("measure"))
            .enumerate()
        {
            audit.measures += 1;
            if let Some(attributes) = measure.children().find(|node| node.has_tag_name("attributes")) {
                if let Some(value) = child_i64(attributes, "divisions") {
                    divisions = value.max(1);
                }
                if let Some(time) = attributes.children().find(|node| node.has_tag_name("time")) {
                    beats = child_i64(time, "beats").unwrap_or(beats).max(1);
                    beat_type = child_i64(time, "beat-type").unwrap_or(beat_type).max(1);
                }
            }
            let measure_length = divisions * beats * 4 / beat_type;
            let label = measure
                .attribute("number")
                .map_or_else(|| (measure_index + 1).to_string(), str::to_string);
            audit_measure(
                &mut audit,
                part_index + 1,
                &label,
                measure,
                measure_length,
            );
        }
    }
    audit
}

fn audit_measure(
    audit: &mut VoiceRunAudit,
    part_ordinal: usize,
    label: &str,
    measure: Node<'_, '_>,
    measure_length: i64,
) {
    let mut events = Vec::<Event>::new();
    let mut cursor = 0_i64;
    let mut last_onset = 0_i64;
    let mut run_index = 0_usize;
    let mut head_voice = String::new();
    let mut head_visible_pitched = false;
    let mut head_rest = false;
    for child in measure.children().filter(|node| node.is_element()) {
        match child.tag_name().name() {
            "backup" => {
                cursor -= child_i64(child, "duration").unwrap_or(0);
                run_index += 1;
            }
            "forward" => cursor += child_i64(child, "duration").unwrap_or(0),
            "note" => {
                if child.children().any(|node| node.has_tag_name("grace")) {
                    continue;
                }
                let duration = child_i64(child, "duration").unwrap_or(0);
                let chord = child.children().any(|node| node.has_tag_name("chord"));
                let onset = if chord { last_onset } else { cursor };
                let voice = child_text(child, "voice").unwrap_or("1").to_string();
                if chord {
                    if voice != head_voice {
                        audit.chord_voice_mismatches += 1;
                        let pitched = child.children().any(|node| node.has_tag_name("pitch"));
                        let visible = child.attribute("print-object") != Some("no");
                        if head_visible_pitched && pitched && visible {
                            audit.visible_pitched_chord_voice_mismatches += 1;
                        } else if head_rest && pitched && !visible {
                            audit.hidden_rest_carrier_voice_mismatches += 1;
                        } else {
                            audit.other_chord_voice_mismatches += 1;
                        }
                    }
                } else {
                    cursor += duration;
                    head_voice.clone_from(&voice);
                    head_visible_pitched = child.attribute("print-object") != Some("no")
                        && child.children().any(|node| node.has_tag_name("pitch"));
                    head_rest = child.children().any(|node| node.has_tag_name("rest"));
                }
                last_onset = onset;
                let (midi, pitch) = note_pitch(child);
                events.push(Event {
                    onset,
                    duration,
                    staff: child_text(child, "staff")
                        .and_then(|text| text.trim().parse().ok())
                        .unwrap_or(1),
                    voice,
                    midi,
                    pitch,
                    run_index,
                    chord,
                });
            }
            _ => {}
        }
    }

    let mut occupancy = BTreeMap::<&str, i64>::new();
    for event in events.iter().filter(|event| !event.chord) {
        *occupancy.entry(event.voice.as_str()).or_default() += event.duration;
    }
    audit.overfull_voice_measures += occupancy
        .values()
        .filter(|total| **total > measure_length)
        .count();

    // Run voice and staff: the most frequent label among the run's non-chord
    // events, ties resolved by first appearance.
    let mut run_labels = BTreeMap::<usize, (String, u8)>::new();
    let mut counts = BTreeMap::<(usize, String, u8), (usize, usize)>::new();
    for (position, event) in events.iter().enumerate().filter(|(_, event)| !event.chord) {
        let entry = counts
            .entry((event.run_index, event.voice.clone(), event.staff))
            .or_insert((0, position));
        entry.0 += 1;
    }
    for ((run, voice, staff), (count, first)) in &counts {
        let replace = run_labels.get(run).is_none_or(|(current_voice, current_staff)| {
            let current = counts
                .get(&(*run, current_voice.clone(), *current_staff))
                .copied()
                .unwrap_or((0, usize::MAX));
            *count > current.0 || (*count == current.0 && *first < current.1)
        });
        if replace {
            run_labels.insert(*run, (voice.clone(), *staff));
        }
    }

    for event in events.iter().filter(|event| !event.chord) {
        let Some((run_voice, run_staff)) = run_labels.get(&event.run_index) else {
            continue;
        };
        if event.voice == *run_voice {
            continue;
        }
        let crossing = event.midi.is_some_and(|midi| {
            events.iter().any(|other| {
                other.staff != event.staff
                    && other.onset <= event.onset
                    && other.onset + other.duration > event.onset
                    && other.midi.is_some_and(|other_midi| {
                        (event.staff > other.staff && midi > other_midi)
                            || (event.staff < other.staff && midi < other_midi)
                    })
            })
        });
        if crossing {
            audit.crossing_foreign_notes += 1;
        }
        audit.foreign_voice_notes.push(ForeignVoiceNote {
            part_ordinal,
            measure: label.to_string(),
            onset: event.onset,
            measure_length,
            duration: event.duration,
            staff: event.staff,
            voice: event.voice.clone(),
            run_voice: run_voice.clone(),
            run_staff: *run_staff,
            pitch: event.pitch.clone(),
            voice_occupancy: occupancy.get(event.voice.as_str()).copied().unwrap_or(0),
            crossing,
        });
    }
}

/// One voice timeline collision of the parent error-class contract
/// docs/selector_derivation_contract_staff_relocation_notation_integrity_error_class.md.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct VoiceTimelineCollision {
    pub(crate) part_ordinal: usize,
    pub(crate) measure: String,
    pub(crate) onset: i64,
    pub(crate) pitch: String,
    pub(crate) staff: u8,
    pub(crate) voice: String,
    pub(crate) run_voice: String,
}

pub(crate) fn voice_timeline_collisions(xml: &str) -> Option<Vec<VoiceTimelineCollision>> {
    Document::parse(xml)
        .ok()
        .map(|document| collisions_in_document(&document))
}

#[derive(Debug, Clone)]
struct RunEvent {
    onset: i64,
    end: i64,
    voice: String,
    run: usize,
    pitched: bool,
    staff: u8,
    pitch: String,
}

fn collisions_in_document(document: &Document<'_>) -> Vec<VoiceTimelineCollision> {
    let mut out = Vec::new();
    for (part_index, part) in document
        .descendants()
        .filter(|node| node.has_tag_name("part"))
        .enumerate()
    {
        for (measure_index, measure) in part
            .children()
            .filter(|node| node.has_tag_name("measure"))
            .enumerate()
        {
            let label = measure
                .attribute("number")
                .map_or_else(|| (measure_index + 1).to_string(), str::to_string);
            let mut events = Vec::<RunEvent>::new();
            let mut cursor = 0_i64;
            let mut run = 0_usize;
            for child in measure.children().filter(|node| node.is_element()) {
                match child.tag_name().name() {
                    "backup" => {
                        cursor -= child_i64(child, "duration").unwrap_or(0);
                        run += 1;
                    }
                    "forward" => cursor += child_i64(child, "duration").unwrap_or(0),
                    "note" => {
                        if child
                            .children()
                            .any(|node| node.has_tag_name("grace") || node.has_tag_name("chord"))
                        {
                            continue;
                        }
                        let duration = child_i64(child, "duration").unwrap_or(0);
                        let (midi, pitch) = note_pitch(child);
                        events.push(RunEvent {
                            onset: cursor,
                            end: cursor + duration,
                            voice: child_text(child, "voice").unwrap_or("1").to_string(),
                            run,
                            pitched: midi.is_some(),
                            staff: child_text(child, "staff")
                                .and_then(|text| text.trim().parse().ok())
                                .unwrap_or(1),
                            pitch,
                        });
                        cursor += duration;
                    }
                    _ => {}
                }
            }
            let mut run_voices = BTreeMap::<usize, String>::new();
            for run_index in events.iter().map(|event| event.run).collect::<std::collections::BTreeSet<_>>() {
                let mut counts: Vec<(&str, usize)> = Vec::new();
                for event in events.iter().filter(|event| event.run == run_index) {
                    if let Some(entry) = counts.iter_mut().find(|(voice, _)| *voice == event.voice) {
                        entry.1 += 1;
                    } else {
                        counts.push((event.voice.as_str(), 1));
                    }
                }
                let mut best: Option<(&str, usize)> = None;
                for (voice, count) in counts {
                    if best.is_none_or(|(_, best_count)| count > best_count) {
                        best = Some((voice, count));
                    }
                }
                if let Some((voice, _)) = best {
                    run_voices.insert(run_index, voice.to_string());
                }
            }
            for event in events.iter().filter(|event| event.pitched) {
                let Some(run_voice) = run_voices.get(&event.run) else {
                    continue;
                };
                if event.voice == *run_voice {
                    continue;
                }
                let collides = events.iter().any(|other| {
                    other.pitched
                        && other.voice == event.voice
                        && other.run != event.run
                        && other.onset < event.end
                        && event.onset < other.end
                        && !(other.onset == event.onset && other.end == event.end)
                });
                if collides {
                    out.push(VoiceTimelineCollision {
                        part_ordinal: part_index + 1,
                        measure: label.clone(),
                        onset: event.onset,
                        pitch: event.pitch.clone(),
                        staff: event.staff,
                        voice: event.voice.clone(),
                        run_voice: run_voice.clone(),
                    });
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Compact event trace of one measure for audit readback.
pub(crate) fn measure_event_trace(xml: &str, part_ordinal: usize, measure: &str) -> Option<String> {
    let document = Document::parse(xml).ok()?;
    let part = document
        .descendants()
        .filter(|node| node.has_tag_name("part"))
        .nth(part_ordinal.checked_sub(1)?)?;
    let node = part
        .children()
        .filter(|node| node.has_tag_name("measure"))
        .find(|node| node.attribute("number") == Some(measure))?;
    let mut tokens = Vec::new();
    let mut cursor = 0_i64;
    let mut last_onset = 0_i64;
    for child in node.children().filter(|node| node.is_element()) {
        match child.tag_name().name() {
            "backup" => {
                let duration = child_i64(child, "duration").unwrap_or(0);
                cursor -= duration;
                tokens.push(format!("b:{duration}"));
            }
            "forward" => {
                let duration = child_i64(child, "duration").unwrap_or(0);
                cursor += duration;
                tokens.push(format!("f:{duration}"));
            }
            "note" => {
                let duration = child_i64(child, "duration").unwrap_or(0);
                let chord = child.children().any(|node| node.has_tag_name("chord"));
                let grace = child.children().any(|node| node.has_tag_name("grace"));
                let onset = if chord { last_onset } else { cursor };
                if !chord && !grace {
                    cursor += duration;
                }
                last_onset = onset;
                let (_, pitch) = note_pitch(child);
                let hidden = child.attribute("print-object") == Some("no");
                tokens.push(format!(
                    "n:{onset}/{duration}/{}/{}/{pitch}{}{}",
                    child_text(child, "staff").unwrap_or("?"),
                    child_text(child, "voice").unwrap_or("?"),
                    if chord { "c" } else { "" },
                    if hidden { "h" } else { "" },
                ));
            }
            _ => {}
        }
    }
    Some(tokens.join(" "))
}

fn child_text<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.children()
        .find(|child| child.has_tag_name(name))
        .and_then(|child| child.text())
}

fn child_i64(node: Node<'_, '_>, name: &str) -> Option<i64> {
    child_text(node, name).and_then(|text| text.trim().parse().ok())
}

fn note_pitch(note: Node<'_, '_>) -> (Option<i32>, String) {
    let Some(pitch) = note.children().find(|node| node.has_tag_name("pitch")) else {
        return (None, "rest".to_string());
    };
    let step = child_text(pitch, "step").unwrap_or("?");
    let alter = child_text(pitch, "alter")
        .and_then(|text| text.trim().parse::<f64>().ok())
        .unwrap_or(0.0);
    let octave = child_i64(pitch, "octave").unwrap_or(4);
    let base = match step {
        "C" => Some(0),
        "D" => Some(2),
        "E" => Some(4),
        "F" => Some(5),
        "G" => Some(7),
        "A" => Some(9),
        "B" => Some(11),
        _ => None,
    };
    let rounded_alter = alter.round();
    let alter_steps = if rounded_alter >= 1.0 {
        1
    } else if rounded_alter <= -1.0 {
        -1
    } else {
        0
    };
    let accidental = match alter_steps {
        1 => "#",
        -1 => "b",
        _ => "",
    };
    let midi = base.map(|value: i32| (i32::try_from(octave).unwrap_or(4) + 1) * 12 + value + alter_steps);
    (midi, format!("{step}{accidental}{octave}"))
}

fn sixteenths(value: i64, measure_length: i64) -> String {
    if measure_length > 0 && (value * 16) % measure_length == 0 {
        format!("{}/16", value * 16 / measure_length)
    } else {
        format!("{value}/{measure_length}div")
    }
}

fn append_source(out: &mut String, source: &str, audit: &VoiceRunAudit) {
    let _ = writeln!(
        out,
        "Readback: Source {source}; SourceRead {}; ParseGreen {}; Measures {}; VoiceTimelineCollisions {}; ForeignVoiceNotes {}; ForeignNotesInOverfullVoice {}; ChordVoiceMismatches {}; VisiblePitchedChordVoiceMismatches {}; HiddenRestCarrierVoiceMismatches {}; OtherChordVoiceMismatches {}; OverfullVoiceMeasures {}; CrossingForeignNotes {}.",
        yes_no(audit.source_read),
        yes_no(audit.parse_green),
        audit.measures,
        audit.voice_timeline_collisions,
        audit.foreign_voice_notes.len(),
        audit
            .foreign_voice_notes
            .iter()
            .filter(|row| row.voice_occupancy > row.measure_length)
            .count(),
        audit.chord_voice_mismatches,
        audit.visible_pitched_chord_voice_mismatches,
        audit.hidden_rest_carrier_voice_mismatches,
        audit.other_chord_voice_mismatches,
        audit.overfull_voice_measures,
        audit.crossing_foreign_notes,
    );
    for row in &audit.foreign_voice_notes {
        let _ = writeln!(
            out,
            "ForeignVoiceNote: Source {source}; Part {}; M{}; Onset {}; Duration {}; Pitch {}; Staff {}; Voice {}; RunVoice {}; RunStaff {}; VoiceOccupancy {}; HandCrossing {}.",
            row.part_ordinal,
            row.measure,
            sixteenths(row.onset, row.measure_length),
            sixteenths(row.duration, row.measure_length),
            row.pitch,
            row.staff,
            row.voice,
            row.run_voice,
            row.run_staff,
            sixteenths(row.voice_occupancy, row.measure_length),
            yes_no(row.crossing),
        );
    }
}

pub(crate) fn append_report_section(out: &mut String, master: Option<&Path>, render: Option<&Path>) {
    let (Some(master), Some(render)) = (master, render) else {
        return;
    };
    out.push_str("VOICE-RUN-INTEGRITY-AUDIT\n");
    out.push_str("=========================\n");
    out.push_str("ChangeClass: AuditOnly; EvidenceSource final Master- and Render-MusicXML; MusicMutation nein; MasterApply nein; WriteAuthority none.\n");
    append_source(out, "master", &inspect_path(master));
    append_source(out, "render", &inspect_path(render));
    out.push_str("ChordVoiceMismatchReading: ChordVoiceMismatches remains the raw total and equals VisiblePitchedChordVoiceMismatches + HiddenRestCarrierVoiceMismatches + OtherChordVoiceMismatches. VisiblePitched means both the non-chord head and the member have pitch and print-object is not no. HiddenRestCarrier means a rest head followed by a pitched member with print-object no, regardless of rest visibility. Other preserves all remaining cases. These are representation observations, not proof of a musical error, importer failure or playback correctness; hidden notation does not establish silence.\n");
    out.push_str("Reading: VoiceTimelineCollisions is the defect count. A collision is a pitched non-chord note serialized in the backup-delimited run of another voice whose own voice label has, in the same measure, a pitched non-chord note of a different run that overlaps it without sharing onset and duration; importers that group by voice label (MuseScore) see that voice overfull and reject the file without force. ForeignVoiceNotes alone is no defect count: most foreign notes are short single-note inner voices that overfill nothing.\n");
    out.push_str("Definition: a ForeignVoiceNote is a non-chord note whose voice label differs from the most frequent voice label of the backup-delimited run it is serialized in; VoiceOccupancy is the summed non-chord duration of its voice label in that measure; an importer grouping by voice label sees occupancy above the measure length as an overfull measure. HandCrossing yes means a simultaneously sounding note of the other staff lies on the wrong side of it (lower staff above upper staff).\n");
    out.push_str("InterpretationBoundary: readback only; the audit neither moves a note nor chooses a voice, staff or hand and owns no MusicXML write path.\n\n");
}

fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

#[cfg(test)]
mod tests {
    use super::inspect_xml;

    fn score(measure_body: &str) -> String {
        let number = 3;
        format!(
            "<score-partwise><part id=\"P1\"><measure number=\"{number}\"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{measure_body}</measure></part></score-partwise>"
        )
    }

    fn note(step: &str, octave: i32, duration: i32, staff: u8, voice: u8, chord: bool) -> String {
        format!(
            "<note>{}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>",
            if chord { "<chord/>" } else { "" }
        )
    }

    #[test]
    fn foreign_voice_note_in_upper_run_is_reported_with_overfull_and_crossing() {
        let body = [
            note("D", 4, 2, 1, 1, false),
            note("G", 4, 2, 2, 2, false),
            note("D", 4, 2, 1, 2, true),
            note("A", 5, 12, 1, 1, false),
            "<backup><duration>16</duration></backup>".to_string(),
            note("B", 3, 16, 2, 2, false),
        ]
        .concat();
        let audit = inspect_xml(&score(&body));
        assert!(audit.parse_green);
        assert_eq!(audit.foreign_voice_notes.len(), 1);
        let row = &audit.foreign_voice_notes[0];
        assert_eq!((row.staff, row.voice.as_str(), row.run_voice.as_str()), (2, "2", "1"));
        assert_eq!(row.voice_occupancy, 18);
        assert_eq!(audit.overfull_voice_measures, 1);
        assert_eq!(audit.chord_voice_mismatches, 0);
        assert!(row.crossing);
        assert_eq!(audit.crossing_foreign_notes, 1);
    }

    #[test]
    fn voice_timeline_collision_needs_overlap_without_shared_onset_and_duration() {
        let colliding = [
            note("E", 5, 4, 1, 1, false),
            note("F", 4, 4, 2, 2, false),
            note("E", 5, 8, 1, 1, false),
            "<backup><duration>16</duration></backup>".to_string(),
            note("C", 3, 2, 2, 2, false),
            note("C", 4, 14, 2, 2, false),
        ]
        .concat();
        let found = super::voice_timeline_collisions(&score(&colliding)).expect("parse");
        assert_eq!(found.len(), 1);
        assert_eq!((found[0].staff, found[0].voice.as_str(), found[0].run_voice.as_str()), (2, "2", "1"));
        let coincident = [
            note("E", 5, 4, 1, 1, false),
            note("F", 4, 4, 2, 2, false),
            note("E", 5, 8, 1, 1, false),
            "<backup><duration>16</duration></backup>".to_string(),
            note("C", 3, 4, 2, 2, false),
            note("C", 4, 4, 2, 2, false),
            note("C", 3, 8, 2, 2, false),
        ]
        .concat();
        assert!(super::voice_timeline_collisions(&score(&coincident)).expect("parse").is_empty());
    }

    #[test]
    fn clean_two_voice_measure_reports_nothing() {
        let body = [
            note("C", 5, 8, 1, 1, false),
            note("E", 5, 8, 1, 1, false),
            "<backup><duration>16</duration></backup>".to_string(),
            note("C", 3, 16, 2, 2, false),
        ]
        .concat();
        let audit = inspect_xml(&score(&body));
        assert!(audit.foreign_voice_notes.is_empty());
        assert_eq!(audit.overfull_voice_measures, 0);
        assert_eq!(audit.chord_voice_mismatches, 0);
    }

    #[test]
    fn chord_member_with_other_voice_is_counted() {
        let body = [note("C", 5, 16, 1, 1, false), note("E", 5, 16, 1, 2, true)].concat();
        let audit = inspect_xml(&score(&body));
        assert_eq!(audit.chord_voice_mismatches, 1);
        assert_eq!(audit.visible_pitched_chord_voice_mismatches, 1);
        assert!(audit.foreign_voice_notes.is_empty());
    }

    #[test]
    fn chord_voice_mismatch_partition_preserves_visibility_and_rest_boundaries() {
        let rest = "<note><rest/><duration>16</duration><voice>1</voice><staff>1</staff></note>";
        let head = note("C", 5, 16, 1, 1, false);
        let member = note("E", 5, 16, 1, 2, true);
        let hidden_member = member.replacen("<note>", "<note print-object='no'>", 1);
        let hidden_head = head.replacen("<note>", "<note print-object='no'>", 1);
        let hidden_rest = rest.replacen("<note>", "<note print-object='no'>", 1);
        let cases = [
            (head.as_str(), member.as_str(), (1, 0, 0)),
            (rest, hidden_member.as_str(), (0, 1, 0)),
            (hidden_rest.as_str(), hidden_member.as_str(), (0, 1, 0)),
            (rest, member.as_str(), (0, 0, 1)),
            (head.as_str(), hidden_member.as_str(), (0, 0, 1)),
            (hidden_head.as_str(), member.as_str(), (0, 0, 1)),
        ];
        for (head, member, expected) in cases {
            let body = format!("{head}{member}");
            for (xml, multiplier) in [(score(&body), 1), (score(&body.repeat(2)), 2)] {
                let audit = inspect_xml(&xml);
                assert_eq!(audit.chord_voice_mismatches, multiplier);
                assert_eq!(
                    (audit.visible_pitched_chord_voice_mismatches,
                     audit.hidden_rest_carrier_voice_mismatches,
                     audit.other_chord_voice_mismatches),
                    (expected.0 * multiplier, expected.1 * multiplier, expected.2 * multiplier),
                );
                let mut report = String::new();
                super::append_source(&mut report, "synthetic", &audit);
                assert!(report.contains(&format!("HiddenRestCarrierVoiceMismatches {}", expected.1 * multiplier)));
            }
        }
        let same_voice = hidden_member.replace("<voice>2</voice>", "<voice>1</voice>");
        let audit = inspect_xml(&score(&format!("{rest}{same_voice}")));
        assert_eq!(audit.chord_voice_mismatches, 0);
        assert_eq!(audit.hidden_rest_carrier_voice_mismatches, 0);
    }
}
