//! Optional, report-only trace of real Render-MusicXML writer deltas.
//!
//! The trace is disabled by default. When enabled through `--writer-trace`
//! or `--writer-trace=<measure-list>`, authorized sinks capture the actual
//! file state immediately before and after a successful write. Only real byte
//! deltas are retained. The collected evidence is appended to the ordinary
//! project result report; no extra public artifact or MusicXML writer exists.

use crate::StaticNamedReportLineV766;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Debug, Clone)]
struct MeasureFragment {
    key: String,
    label: String,
    number: String,
    xml: String,
}

#[derive(Debug, Clone, Default)]
struct XmlCounters {
    notes: usize,
    rests: usize,
    forwards: usize,
    clefs: usize,
    beams: usize,
    directions: usize,
    articulations: usize,
    octave_shifts: usize,
    prints: usize,
}

#[derive(Debug, Clone)]
struct WriterTraceEntry {
    ordinal: usize,
    writer: String,
    file: String,
    bytes_before: usize,
    bytes_after: usize,
    changed_measures: Vec<String>,
    counters_before: XmlCounters,
    counters_after: XmlCounters,
    snapshots: Vec<WriterTraceSnapshot>,
}

#[derive(Debug, Clone)]
struct WriterTraceSnapshot {
    measure_label: String,
    before_xml: String,
    after_xml: String,
}

#[derive(Debug)]
struct WriterTraceState {
    project: String,
    request: String,
    selected_measures: Option<BTreeSet<String>>,
    entries: Vec<WriterTraceEntry>,
}

#[derive(Debug, Clone)]
struct PrimaryBeamWriterDeltaEntry {
    ordinal: usize,
    writer: String,
    file: String,
    measure_label: String,
    changes: Vec<String>,
}

#[derive(Debug)]
struct PrimaryBeamWriterDeltaState {
    project: String,
    entries: Vec<PrimaryBeamWriterDeltaEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PrimaryBeamNoteState {
    ordinal: usize,
    onset: i64,
    duration: i64,
    staff: String,
    voice: String,
    pitch: String,
    is_chord: bool,
    primary_beam: Option<String>,
}

thread_local! {
    static WRITER_TRACE_STATE: RefCell<Option<WriterTraceState>> = const { RefCell::new(None) };
    static PRIMARY_BEAM_WRITER_DELTA_STATE: RefCell<Option<PrimaryBeamWriterDeltaState>> =
        const { RefCell::new(None) };
    static RENDER_EPOCH_CONTINUITY_STATE: RefCell<Option<RenderEpochContinuityState>> =
        const { RefCell::new(None) };
}

#[derive(Debug)]
pub struct WriterTraceSession {
    active: bool,
}

impl Drop for WriterTraceSession {
    fn drop(&mut self) {
        if self.active {
            WRITER_TRACE_STATE.with(|slot| {
                *slot.borrow_mut() = None;
            });
        }
        PRIMARY_BEAM_WRITER_DELTA_STATE.with(|slot| {
            *slot.borrow_mut() = None;
        });
        RENDER_EPOCH_CONTINUITY_STATE.with(|slot| {
            *slot.borrow_mut() = None;
        });
        crate::audit_only::destructive_edit_attribution::clear();
    }
}

pub fn start(
    request: Option<&str>,
    project: &str,
    final_render_path: Option<&Path>,
) -> Result<WriterTraceSession, String> {
    crate::audit_only::destructive_edit_attribution::start(project, final_render_path);
    RENDER_EPOCH_CONTINUITY_STATE.with(|slot| {
        *slot.borrow_mut() = Some(RenderEpochContinuityState::default());
    });
    PRIMARY_BEAM_WRITER_DELTA_STATE.with(|slot| {
        *slot.borrow_mut() = Some(PrimaryBeamWriterDeltaState {
            project: project.to_string(),
            entries: Vec::new(),
        });
    });
    let Some(raw_request) = request else {
        super::original_beam_diagnostics::start(project, None);
        return Ok(WriterTraceSession { active: false });
    };
    let request = raw_request.trim();
    if request.is_empty() {
        return Err("--writer-trace darf nicht leer sein; verwende --writer-trace oder --writer-trace=5,7".to_string());
    }
    let selected_measures = if request.eq_ignore_ascii_case("all") {
        None
    } else {
        let mut selected = BTreeSet::new();
        for token in request.split(',') {
            let normalized = normalize_measure_selector(token).ok_or_else(|| {
                format!(
                    "ungueltiger --writer-trace-Takt {:?}; erlaubt sind positive Taktnummern, kommasepariert, oder all",
                    token.trim()
                )
            })?;
            selected.insert(normalized);
        }
        if selected.is_empty() {
            return Err("--writer-trace enthaelt keine Taktnummer".to_string());
        }
        Some(selected)
    };
    super::original_beam_diagnostics::start(project, Some(selected_measures.clone()));
    WRITER_TRACE_STATE.with(|slot| {
        *slot.borrow_mut() = Some(WriterTraceState {
            project: project.to_string(),
            request: request.to_string(),
            selected_measures,
            entries: Vec::new(),
        });
    });
    Ok(WriterTraceSession { active: true })
}

// Existing optional diagnostics only: no argument becomes productive authority.
pub fn batch_child_trace_arguments() -> Vec<std::ffi::OsString> {
    trace_arguments(std::env::args_os().skip(1))
}

fn trace_arguments(args: impl Iterator<Item = std::ffi::OsString>) -> Vec<std::ffi::OsString> {
    args.filter(|arg| {
        arg.to_str().is_some_and(|text| text == "--writer-trace" || text.starts_with("--writer-trace="))
    }).collect()
}



pub fn capture_before(path: &Path) -> Option<String> {
    if !trace_accepts_path(path) || (!trace_is_active() && !primary_beam_writer_delta_is_active()) {
        return None;
    }
    std::fs::read_to_string(path).ok()
}

pub fn record_after(
    writer: &str,
    path: &Path,
    before: Option<&str>,
    after: &str,
) {
    observe_render_epoch(writer, path, before, after);
    if !trace_accepts_path(path) || before == Some(after) {
        return;
    }
    if primary_beam_writer_delta_is_active() {
        record_primary_beam_writer_delta(writer, path, before, after);
    }
    if !trace_is_active() {
        return;
    }
    let before_xml = before.unwrap_or("");
    let before_measures = collect_measure_fragments(before_xml);
    let after_measures = collect_measure_fragments(after);
    let changed = changed_measure_fragments(&before_measures, &after_measures);
    let changed_labels = changed
        .iter()
        .map(|(_, after_fragment, before_fragment)| {
            (*after_fragment)
                .or(*before_fragment)
                .map_or_else(|| "unbekannt".to_string(), |fragment| fragment.label.clone())
        })
        .collect::<Vec<_>>();
    let file = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("<render.musicxml>")
        .to_string();

    WRITER_TRACE_STATE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let Some(state) = slot.as_mut() else {
            return;
        };
        let snapshots = changed
            .iter()
            .filter_map(|(_, after_fragment, before_fragment)| {
                let fragment = (*after_fragment).or(*before_fragment)?;
                if !measure_selected(&state.selected_measures, &fragment.number) {
                    return None;
                }
                Some(WriterTraceSnapshot {
                    measure_label: fragment.label.clone(),
                    before_xml: (*before_fragment)
                        .map_or_else(|| "<nicht vorhanden>".to_string(), |value| value.xml.clone()),
                    after_xml: (*after_fragment)
                        .map_or_else(|| "<nicht vorhanden>".to_string(), |value| value.xml.clone()),
                })
            })
            .collect::<Vec<_>>();
        let ordinal = state.entries.len().saturating_add(1);
        state.entries.push(WriterTraceEntry {
            ordinal,
            writer: writer.to_string(),
            file,
            bytes_before: before_xml.len(),
            bytes_after: after.len(),
            changed_measures: changed_labels,
            counters_before: xml_counters(before_xml),
            counters_after: xml_counters(after),
            snapshots,
        });
    });
}

pub fn report_body() -> Option<String> {
    WRITER_TRACE_STATE.with(|slot| {
        let slot = slot.borrow();
        let state = slot.as_ref()?;
        let mut out = String::new();
        out.push_format_args(format_args!("Projekt: {}\n", state.project));
        out.push_format_args(format_args!("Anforderung: {}\n", state.request));
        out.push_format_args(format_args!("RealeWriterDeltas: {}\n", state.entries.len()));
        out.push_str("Aktivierung: optional; Standardlauf unveraendert.\n");
        out.push_str("Grenze: nur erfolgreiche Render-MusicXML-Dateischreibungen; reine No-Op-Aufrufe werden nicht ausgegeben.\n");
        out.push_str("MusikmutationDurchTrace: nein; Trace sammelt nur Vorher-/Nachher-Evidenz im bestehenden Ergebnisbericht.\n");
        if state.entries.is_empty() {
            out.push_str("Ergebnis: kein realer Render-MusicXML-Writer-Delta beobachtet.\n");
            return Some(out);
        }
        for entry in &state.entries {
            out.push_format_args(format_args!(
                "\n[{:03}] {}\nDatei: {}\nBytes: {} -> {}\nGeaenderteTakte: {}\nDelta: notes {:+}; rests {:+}; forwards {:+}; clefs {:+}; beams {:+}; directions {:+}; articulations {:+}; octave_shifts {:+}; prints {:+}\n",
                entry.ordinal,
                entry.writer,
                entry.file,
                entry.bytes_before,
                entry.bytes_after,
                compact_measure_labels(&entry.changed_measures),
                signed_delta(entry.counters_before.notes, entry.counters_after.notes),
                signed_delta(entry.counters_before.rests, entry.counters_after.rests),
                signed_delta(entry.counters_before.forwards, entry.counters_after.forwards),
                signed_delta(entry.counters_before.clefs, entry.counters_after.clefs),
                signed_delta(entry.counters_before.beams, entry.counters_after.beams),
                signed_delta(entry.counters_before.directions, entry.counters_after.directions),
                signed_delta(entry.counters_before.articulations, entry.counters_after.articulations),
                signed_delta(entry.counters_before.octave_shifts, entry.counters_after.octave_shifts),
                signed_delta(entry.counters_before.prints, entry.counters_after.prints),
            ));
            for snapshot in &entry.snapshots {
                out.push_format_args(format_args!(
                    "Snapshot {} BEFORE\n{}\nSnapshot {} AFTER\n{}\n",
                    snapshot.measure_label,
                    snapshot.before_xml,
                    snapshot.measure_label,
                    snapshot.after_xml
                ));
            }
        }
        Some(out)
    })
}

pub fn primary_beam_writer_delta_report_body() -> String {
    PRIMARY_BEAM_WRITER_DELTA_STATE.with(|slot| {
        let slot = slot.borrow();
        let Some(state) = slot.as_ref() else {
            return "ProjectionOnly: true\nAuthority: none\nMusicMutation: nein\nPrimaryBeamWriterDeltaTraceState: unavailable_no_project_session\n".to_string();
        };
        let mut out = String::new();
        out.push_str("ProjectionOnly: true\n");
        out.push_str("Authority: none\n");
        out.push_str("MusicMutation: nein\n");
        out.push_str("RuleChange: nein\n");
        out.push_str("TraceScope: all_authorized_render_musicxml_writes_generic_no_project_measure_selector\n");
        out.push_str("TraceSemantics: exact_same_ordinal_note_identity_primary_beam_level1_delta_only\n");
        out.push_format_args(format_args!("Project: {}\n", state.project));
        out.push_format_args(format_args!(
            "PrimaryBeamWriterDeltaEntries: {}\n",
            state.entries.len()
        ));
        if state.entries.is_empty() {
            out.push_str("PrimaryBeamWriterDeltaRows: none\n");
            return out;
        }
        for entry in &state.entries {
            out.push_format_args(format_args!(
                "PrimaryBeamWriterDeltaRow: writer_ordinal={} writer={} file={} measure={} changes={}\n",
                entry.ordinal,
                entry.writer,
                entry.file,
                entry.measure_label,
                entry.changes.len(),
            ));
            for change in &entry.changes {
                out.push_str("PrimaryBeamWriterDeltaChange: ");
                out.push_str(change);
                out.push('\n');
            }
        }
        out
    })
}

fn primary_beam_writer_delta_is_active() -> bool {
    PRIMARY_BEAM_WRITER_DELTA_STATE.with(|slot| slot.borrow().is_some())
}

fn record_primary_beam_writer_delta(
    writer: &str,
    path: &Path,
    before: Option<&str>,
    after: &str,
) {
    let Some(before_xml) = before else {
        return;
    };
    let before_measures = collect_measure_fragments(before_xml);
    let after_measures = collect_measure_fragments(after);
    let changed = changed_measure_fragments(&before_measures, &after_measures);
    if changed.is_empty() {
        return;
    }
    let file = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("<render.musicxml>")
        .to_string();
    PRIMARY_BEAM_WRITER_DELTA_STATE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let Some(state) = slot.as_mut() else {
            return;
        };
        for (_, after_fragment, before_fragment) in changed {
            let (Some(before_fragment), Some(after_fragment)) =
                (before_fragment, after_fragment)
            else {
                continue;
            };
            let changes = primary_beam_note_state_changes(&before_fragment.xml, &after_fragment.xml);
            if changes.is_empty() {
                continue;
            }
            let ordinal = state.entries.len().saturating_add(1);
            state.entries.push(PrimaryBeamWriterDeltaEntry {
                ordinal,
                writer: writer.to_string(),
                file: file.clone(),
                measure_label: after_fragment.label.clone(),
                changes,
            });
        }
    });
}

fn primary_beam_note_state_changes(before_measure: &str, after_measure: &str) -> Vec<String> {
    let Some(before_notes) = primary_beam_note_states(before_measure) else {
        return Vec::new();
    };
    let Some(after_notes) = primary_beam_note_states(after_measure) else {
        return Vec::new();
    };
    if before_notes.len() != after_notes.len() {
        return Vec::new();
    }
    before_notes
        .iter()
        .zip(after_notes.iter())
        .filter_map(|(before, after)| {
            let same_identity = before.ordinal == after.ordinal
                && before.onset == after.onset
                && before.duration == after.duration
                && before.staff == after.staff
                && before.voice == after.voice
                && before.pitch == after.pitch
                && before.is_chord == after.is_chord;
            if !same_identity || before.primary_beam == after.primary_beam {
                return None;
            }
            Some(format!(
                "note_ordinal={} onset={} duration={} staff={} voice={} pitch={} primary_beam={}=>{}",
                before.ordinal,
                before.onset,
                before.duration,
                before.staff,
                before.voice,
                before.pitch,
                before.primary_beam.as_deref().unwrap_or("none"),
                after.primary_beam.as_deref().unwrap_or("none"),
            ))
        })
        .collect()
}

fn primary_beam_note_states(measure_xml: &str) -> Option<Vec<PrimaryBeamNoteState>> {
    let wrapped = format!(
        "<score-partwise><part id=\"P1\">{measure_xml}</part></score-partwise>"
    );
    let document = roxmltree::Document::parse(&wrapped).ok()?;
    let measure = document
        .descendants()
        .find(|node| node.is_element() && node.has_tag_name("measure"))?;
    let mut notes = Vec::new();
    let mut cursor = 0_i64;
    let mut last_non_chord_onset = 0_i64;
    let mut ordinal = 0usize;
    for node in measure.children().filter(|node| node.is_element()) {
        if node.has_tag_name("backup") {
            cursor = cursor.saturating_sub(primary_beam_child_duration(node).max(0));
            continue;
        }
        if node.has_tag_name("forward") {
            cursor = cursor.saturating_add(primary_beam_child_duration(node).max(0));
            continue;
        }
        if !node.has_tag_name("note") {
            continue;
        }
        ordinal = ordinal.saturating_add(1);
        let duration = primary_beam_child_duration(node).max(0);
        let is_chord = node
            .children()
            .any(|child| child.is_element() && child.has_tag_name("chord"));
        let onset = if is_chord {
            last_non_chord_onset
        } else {
            cursor
        };
        if !is_chord {
            last_non_chord_onset = onset;
        }
        let staff = primary_beam_child_text(node, "staff").unwrap_or_else(|| "1".to_string());
        let voice = primary_beam_child_text(node, "voice").unwrap_or_else(|| "1".to_string());
        let pitch = primary_beam_pitch_label(node);
        let primary_beam = node
            .children()
            .filter(|child| child.is_element() && child.has_tag_name("beam"))
            .find(|beam| beam.attribute("number").unwrap_or("1") == "1")
            .and_then(|beam| beam.text())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_ascii_lowercase);
        notes.push(PrimaryBeamNoteState {
            ordinal,
            onset,
            duration,
            staff,
            voice,
            pitch,
            is_chord,
            primary_beam,
        });
        if !is_chord {
            cursor = cursor.saturating_add(duration);
        }
    }
    Some(notes)
}

fn primary_beam_child_text(node: roxmltree::Node<'_, '_>, name: &str) -> Option<String> {
    node.children()
        .find(|child| child.is_element() && child.has_tag_name(name))
        .and_then(|child| child.text())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn primary_beam_child_duration(node: roxmltree::Node<'_, '_>) -> i64 {
    primary_beam_child_text(node, "duration")
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
}

fn primary_beam_pitch_label(node: roxmltree::Node<'_, '_>) -> String {
    if node
        .children()
        .any(|child| child.is_element() && child.has_tag_name("rest"))
    {
        return "rest".to_string();
    }
    let Some(pitch) = node
        .children()
        .find(|child| child.is_element() && child.has_tag_name("pitch"))
    else {
        return "none".to_string();
    };
    let step = primary_beam_child_text(pitch, "step").unwrap_or_else(|| "?".to_string());
    let alter = primary_beam_child_text(pitch, "alter").unwrap_or_else(|| "0".to_string());
    let octave = primary_beam_child_text(pitch, "octave").unwrap_or_else(|| "?".to_string());
    format!("{step}:{alter}:{octave}")
}

fn trace_is_active() -> bool {
    WRITER_TRACE_STATE.with(|slot| slot.borrow().is_some())
}

fn trace_accepts_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|name| {
                let lower = name.to_ascii_lowercase();
                lower.ends_with(".musicxml") && lower.contains("render")
            },
        )
}

fn normalize_measure_selector(token: &str) -> Option<String> {
    let trimmed = token.trim();
    let without_prefix = trimmed
        .strip_prefix('M')
        .or_else(|| trimmed.strip_prefix('m'))
        .unwrap_or(trimmed);
    if without_prefix.is_empty() || !without_prefix.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    let number = without_prefix.parse::<usize>().ok()?;
    if number == 0 {
        return None;
    }
    Some(number.to_string())
}

fn measure_selected(selected: &Option<BTreeSet<String>>, number: &str) -> bool {
    match selected {
        None => true,
        Some(values) => normalize_measure_selector(number)
            .is_some_and(|normalized| values.contains(&normalized)),
    }
}

fn collect_measure_fragments(xml: &str) -> Vec<MeasureFragment> {
    let mut fragments = Vec::new();
    let mut part_cursor = 0usize;
    let mut part_ordinal = 0usize;
    while let Some(part_start) = find_open_tag(xml, "part", part_cursor) {
        let Some(part_open_end_relative) = xml[part_start..].find('>') else {
            break;
        };
        let part_open_end = part_start + part_open_end_relative;
        let part_open_tag = &xml[part_start..=part_open_end];
        let Some(part_close_relative) = xml[part_open_end + 1..].find("</part>") else {
            break;
        };
        let part_close = part_open_end + 1 + part_close_relative;
        part_ordinal = part_ordinal.saturating_add(1);
        let part_id = attribute_value(part_open_tag, "id")
            .unwrap_or_else(|| format!("P{part_ordinal}"));
        let mut measure_cursor = part_open_end + 1;
        let mut measure_ordinal = 0usize;
        while measure_cursor < part_close {
            let Some(measure_start) = find_open_tag(xml, "measure", measure_cursor) else {
                break;
            };
            if measure_start >= part_close {
                break;
            }
            let Some(measure_open_end_relative) = xml[measure_start..].find('>') else {
                break;
            };
            let measure_open_end = measure_start + measure_open_end_relative;
            if measure_open_end >= part_close {
                break;
            }
            let measure_open_tag = &xml[measure_start..=measure_open_end];
            let Some(measure_close_relative) = xml[measure_open_end + 1..].find("</measure>") else {
                break;
            };
            let measure_end = measure_open_end + 1 + measure_close_relative + "</measure>".len();
            if measure_end > part_close + "</part>".len() {
                break;
            }
            measure_ordinal = measure_ordinal.saturating_add(1);
            let number = attribute_value(measure_open_tag, "number")
                .unwrap_or_else(|| measure_ordinal.to_string());
            fragments.push(MeasureFragment {
                key: format!("{part_id}|{number}|{measure_ordinal}"),
                label: format!("{part_id}/M{number}"),
                number,
                xml: xml[measure_start..measure_end].to_string(),
            });
            measure_cursor = measure_end;
        }
        part_cursor = part_close + "</part>".len();
    }
    fragments
}

fn find_open_tag(xml: &str, tag: &str, mut cursor: usize) -> Option<usize> {
    let needle = format!("<{tag}");
    let bytes = xml.as_bytes();
    while let Some(relative) = xml.get(cursor..)?.find(&needle) {
        let start = cursor + relative;
        let boundary = start + needle.len();
        if bytes
            .get(boundary)
            .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>')
        {
            return Some(start);
        }
        cursor = boundary;
    }
    None
}

fn attribute_value(open_tag: &str, attribute: &str) -> Option<String> {
    let needle = format!("{attribute}=");
    let start = open_tag.find(&needle)? + needle.len();
    let quote = *open_tag.as_bytes().get(start)?;
    if quote != b'\'' && quote != b'"' {
        return None;
    }
    let value_start = start + 1;
    let value_end_relative = open_tag.get(value_start..)?.find(char::from(quote))?;
    Some(open_tag[value_start..value_start + value_end_relative].to_string())
}

type ChangedMeasure<'a> = (
    String,
    Option<&'a MeasureFragment>,
    Option<&'a MeasureFragment>,
);

fn changed_measure_fragments<'a>(
    before: &'a [MeasureFragment],
    after: &'a [MeasureFragment],
) -> Vec<ChangedMeasure<'a>> {
    let before_by_key = before
        .iter()
        .map(|fragment| (fragment.key.clone(), fragment))
        .collect::<BTreeMap<_, _>>();
    let after_by_key = after
        .iter()
        .map(|fragment| (fragment.key.clone(), fragment))
        .collect::<BTreeMap<_, _>>();
    let keys = before_by_key
        .keys()
        .chain(after_by_key.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    keys.into_iter()
        .filter_map(|key| {
            let before_fragment = before_by_key.get(&key).copied();
            let after_fragment = after_by_key.get(&key).copied();
            if before_fragment.map(|value| value.xml.as_str())
                == after_fragment.map(|value| value.xml.as_str())
            {
                None
            } else {
                Some((key, after_fragment, before_fragment))
            }
        })
        .collect()
}

fn xml_counters(xml: &str) -> XmlCounters {
    XmlCounters {
        notes: count_open_tag(xml, "note"),
        rests: count_open_tag(xml, "rest"),
        forwards: count_open_tag(xml, "forward"),
        clefs: count_open_tag(xml, "clef"),
        beams: count_open_tag(xml, "beam"),
        directions: count_open_tag(xml, "direction"),
        articulations: count_open_tag(xml, "articulations"),
        octave_shifts: count_open_tag(xml, "octave-shift"),
        prints: count_open_tag(xml, "print"),
    }
}

fn count_open_tag(xml: &str, tag: &str) -> usize {
    let needle = format!("<{tag}");
    let bytes = xml.as_bytes();
    let mut count = 0usize;
    let mut cursor = 0usize;
    while let Some(relative) = xml[cursor..].find(&needle) {
        let start = cursor + relative;
        let end = start + needle.len();
        if bytes
            .get(end)
            .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>' || *byte == b'/')
        {
            count = count.saturating_add(1);
        }
        cursor = end;
    }
    count
}

fn signed_delta(before: usize, after: usize) -> i64 {
    let before = i64::try_from(before).unwrap_or(i64::MAX);
    let after = i64::try_from(after).unwrap_or(i64::MAX);
    after.saturating_sub(before)
}

fn compact_measure_labels(labels: &[String]) -> String {
    if labels.is_empty() {
        return "keine/ausserhalb parsebarer Measures".to_string();
    }
    const LIMIT: usize = 18;
    if labels.len() <= LIMIT {
        return labels.join(", ");
    }
    format!("{}, ... (+{} weitere)", labels[..LIMIT].join(", "), labels.len() - LIMIT)
}

pub fn cleanup_clef_ab_probe_artifacts(project_root: &Path) -> String {
    let mut removed = 0usize;
    let mut failed = 0usize;
    let Ok(entries) = std::fs::read_dir(project_root) else {
        return "ClefAbProbeCleanup: Projektwurzel nicht lesbar; entfernt 0; Fehler 1.".to_string();
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        let remove_result = if path.is_dir() && is_clef_ab_probe_dir_name(name) {
            Some(std::fs::remove_dir_all(&path))
        } else if path.is_file() && is_clef_ab_probe_root_file(&path, name) {
            Some(std::fs::remove_file(&path))
        } else {
            None
        };
        if let Some(result) = remove_result {
            match result {
                Ok(()) => removed = removed.saturating_add(1),
                Err(_) => failed = failed.saturating_add(1),
            }
        }
    }
    format!(
        "ClefAbProbeCleanup: entfernte Diagnoseartefakte {removed}; Fehler {failed}; Musikmutation nein."
    )
}

fn is_clef_ab_probe_root_file(path: &Path, name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == "render_ab.cmd"
        || (lower.starts_with("clef_ab_v") && lower.ends_with(".zip"))
        || (lower == "befund.txt"
            && std::fs::read_to_string(path)
                .map(|text| text.contains("Clef-A/B") || text.contains("G-Schluessel"))
                .unwrap_or(false))
        || (lower.starts_with("a_v")
            && (lower.contains("bassschluessel") || lower.contains("original"))
            && (lower.ends_with(".pdf") || lower.ends_with(".render.musicxml")))
        || (lower.starts_with("b_gschluessel_")
            && (lower.ends_with(".pdf") || lower.ends_with(".render.musicxml")))
}

fn is_clef_ab_probe_dir_name(name: &str) -> bool {
    name.to_ascii_lowercase().starts_with("clef_ab_v")
}

// r1182 DestructiveCarrierEffectInventoryOutcomeProjection: render epoch continuity.
//
// Every persisted render boundary that already passes through `record_after` is fingerprinted
// over its complete bytes. Inter-boundary drift is decided exclusively by comparing the
// fingerprint of the previous boundary's persisted bytes with the bytes found immediately
// before the next boundary; carrier metrics only explain a delta and never decide drift.
// Nothing here writes, selects, repairs or attributes a delta to an individual owner.

type CensusIntervals = Vec<(i64, i64)>;
type CensusVoiceKey = (usize, usize, String, String);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct CarrierCensus {
    parsed: bool,
    visible_pitched: i64,
    hidden_pitched: i64,
    visible_rests: i64,
    hidden_rests: i64,
    forwards: i64,
    system_silence: i64,
    voice_label_silence: i64,
    sounding: u64,
}

#[derive(Debug, Clone)]
struct RenderEpochRow {
    kind: String,
    stream: u64,
    class: &'static str,
    gap: &'static str,
    transition_accounting: &'static str,
    transition_candidates: usize,
    unchanged: bool,
    before: Option<CarrierCensus>,
    after: CarrierCensus,
    before_fingerprint: Option<u64>,
    after_fingerprint: u64,
}

#[derive(Debug, Clone, Copy)]
struct ExcludedPathStateObservation {
    serial: u64,
    after_fingerprint: u64,
}

#[derive(Debug, Default)]
struct RenderEpochContinuityState {
    previous_after_by_path: BTreeMap<String, u64>,
    previous_serial_by_path: BTreeMap<String, u64>,
    excluded_path_states: Vec<ExcludedPathStateObservation>,
    observation_serial: u64,
    rows: Vec<RenderEpochRow>,
    outside_path_calls: usize,
}

fn continuity_path_identity(path: &Path) -> String {
    std::fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase()
}

fn observe_render_epoch(kind: &str, path: &Path, before: Option<&str>, after: &str) {
    RENDER_EPOCH_CONTINUITY_STATE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let Some(state) = slot.as_mut() else {
            return;
        };
        state.observation_serial = state.observation_serial.saturating_add(1);
        let serial = state.observation_serial;
        crate::audit_only::destructive_edit_attribution::observe_render_epoch_boundary(
            path, serial, before, after,
        );
        let after_fingerprint = continuity_fingerprint(after);
        if !trace_accepts_path(path) {
            state.outside_path_calls = state.outside_path_calls.saturating_add(1);
            state
                .excluded_path_states
                .push(ExcludedPathStateObservation {
                    serial,
                    after_fingerprint,
                });
            return;
        }
        let path_identity = continuity_path_identity(path);
        let stream = continuity_fingerprint(&path_identity);
        let previous_after = state.previous_after_by_path.get(&path_identity).copied();
        let previous_serial = state.previous_serial_by_path.get(&path_identity).copied();
        let before_fingerprint = before.map(continuity_fingerprint);
        let gap = match (previous_after, before_fingerprint) {
            (None, _) => "FIRST",
            (Some(previous), Some(current)) if previous == current => "CONTINUOUS",
            _ => "DRIFT_OR_EPOCH_RESET",
        };
        let transition_candidates = if gap == "DRIFT_OR_EPOCH_RESET" {
            before_fingerprint.map_or(0, |current_before| {
                state
                    .excluded_path_states
                    .iter()
                    .filter(|observation| {
                        previous_serial.is_none_or(|previous| observation.serial > previous)
                            && observation.serial < serial
                            && observation.after_fingerprint == current_before
                    })
                    .count()
            })
        } else {
            0
        };
        let transition_accounting = match gap {
            "FIRST" => "SEED",
            "CONTINUOUS" => "NOT_APPLICABLE",
            _ if transition_candidates == 1 => "EXPLAINED_BY_EXCLUDED_PATH_STATE",
            _ if transition_candidates > 1 => "AMBIGUOUS_EXCLUDED_PATH_STATE",
            _ => "UNEXPLAINED",
        };
        let class = if previous_after.is_none() {
            "SEED"
        } else {
            crate::audit_only::destructive_carrier_effect_inventory::static_kind_granularity(kind)
        };
        state.rows.push(RenderEpochRow {
            kind: kind.to_string(),
            stream,
            class,
            gap,
            transition_accounting,
            transition_candidates,
            unchanged: before == Some(after),
            before: before.map(carrier_census),
            after: carrier_census(after),
            before_fingerprint,
            after_fingerprint,
        });
        state
            .previous_after_by_path
            .insert(path_identity.clone(), after_fingerprint);
        state.previous_serial_by_path.insert(path_identity, serial);
    });
}

fn continuity_fingerprint(text: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn census_child<'a, 'input>(
    node: roxmltree::Node<'a, 'input>,
    name: &str,
) -> Option<roxmltree::Node<'a, 'input>> {
    node.children().find(|child| child.has_tag_name(name))
}

fn census_duration(node: roxmltree::Node<'_, '_>) -> i64 {
    census_child(node, "duration")
        .and_then(|child| child.text())
        .and_then(|text| text.trim().parse::<i64>().ok())
        .unwrap_or(0)
}

fn census_label(node: roxmltree::Node<'_, '_>, name: &str) -> String {
    census_child(node, name)
        .and_then(|child| child.text())
        .map_or_else(|| "1".to_string(), |text| text.trim().to_string())
}

fn census_union_length(intervals: &[(i64, i64)]) -> i64 {
    let mut sorted = intervals.to_vec();
    sorted.sort_unstable();
    let mut total = 0i64;
    let mut current: Option<(i64, i64)> = None;
    for (start, end) in sorted {
        match current {
            Some((open, close)) if start <= close => current = Some((open, close.max(end))),
            Some((open, close)) => {
                total += close - open;
                current = Some((start, end));
            }
            None => current = Some((start, end)),
        }
    }
    if let Some((open, close)) = current {
        total += close - open;
    }
    total
}

fn carrier_census(xml: &str) -> CarrierCensus {
    let Ok(document) = roxmltree::Document::parse(xml) else {
        return CarrierCensus::default();
    };
    let mut census = CarrierCensus {
        parsed: true,
        ..CarrierCensus::default()
    };
    let mut system: BTreeMap<(usize, usize, String), CensusIntervals> = BTreeMap::new();
    let mut voice_label: BTreeMap<CensusVoiceKey, CensusIntervals> = BTreeMap::new();
    let mut sounding = Vec::new();
    let parts = document
        .descendants()
        .filter(|node| node.has_tag_name("part"));
    for (part_index, part) in parts.enumerate() {
        let measures = part.children().filter(|node| node.has_tag_name("measure"));
        for (measure_index, measure) in measures.enumerate() {
            let mut cursor = 0i64;
            let mut last_onset = 0i64;
            for child in measure.children().filter(roxmltree::Node::is_element) {
                match child.tag_name().name() {
                    "backup" => cursor -= census_duration(child),
                    "forward" => {
                        census.forwards += 1;
                        cursor += census_duration(child);
                    }
                    "note" => {
                        let grace = census_child(child, "grace").is_some();
                        let chord = census_child(child, "chord").is_some();
                        let duration = if grace { 0 } else { census_duration(child) };
                        let onset = if chord { last_onset } else { cursor };
                        if !chord && !grace {
                            last_onset = cursor;
                            cursor += duration;
                        }
                        let hidden = child.attribute("print-object") == Some("no");
                        let staff = census_label(child, "staff");
                        if census_child(child, "rest").is_some() {
                            if hidden {
                                census.hidden_rests += 1;
                            } else {
                                census.visible_rests += 1;
                                if duration > 0 {
                                    let interval = (onset, onset + duration);
                                    system
                                        .entry((part_index, measure_index, staff.clone()))
                                        .or_default()
                                        .push(interval);
                                    voice_label
                                        .entry((part_index, measure_index, staff, census_label(child, "voice")))
                                        .or_default()
                                        .push(interval);
                                }
                            }
                        } else if let Some(pitch) = census_child(child, "pitch") {
                            if hidden {
                                census.hidden_pitched += 1;
                            } else {
                                census.visible_pitched += 1;
                            }
                            sounding.push(format!(
                                "{part_index}|{measure_index}|{onset}|{}|{}|{}",
                                census_label(pitch, "step"),
                                census_child(pitch, "alter")
                                    .and_then(|node| node.text())
                                    .map_or("0", str::trim),
                                census_label(pitch, "octave")
                            ));
                        } else if census_child(child, "unpitched").is_some() {
                            if hidden {
                                census.hidden_pitched += 1;
                            } else {
                                census.visible_pitched += 1;
                            }
                            sounding.push(format!("{part_index}|{measure_index}|{onset}|unpitched"));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    census.system_silence = system.values().map(|intervals| census_union_length(intervals)).sum();
    census.voice_label_silence = voice_label
        .values()
        .map(|intervals| census_union_length(intervals))
        .sum();
    sounding.sort_unstable();
    census.sounding = continuity_fingerprint(&sounding.join("\n"));
    census
}

fn census_value(before: Option<CarrierCensus>, after: CarrierCensus, field: fn(&CarrierCensus) -> i64) -> String {
    match before {
        None => format!("={}", field(&after)),
        Some(before) => format!("{:+}", field(&after) - field(&before)),
    }
}

/// Report body for the render epoch continuity section, followed by the static inventory.
pub fn render_epoch_continuity_report_body() -> String {
    let mut out = String::new();
    out.push_str(
        "RenderEpochContinuityContract: read-only byte-fingerprint continuity over the persisted render boundaries already passing the existing recorder; continuity is scoped independently per persisted render path, the first observed boundary of each path is a seed, and drift is decided only by comparing complete-byte FNV-1a-64 fingerprints between consecutive observations of that same path; for a same-path drift, transition accounting may report that the current before-fingerprint exactly matches one or more states already observed by recorder calls outside accepted render paths after the preceding same-path boundary; such a match explains only that an excluded-path state bridges the observed byte state and never identifies an owner, proves causality, or authorizes a write; carrier metrics explain deltas and never decide drift.\n",
    );
    RENDER_EPOCH_CONTINUITY_STATE.with(|slot| {
        let slot = slot.borrow();
        let Some(state) = slot.as_ref() else {
            out.push_str("RenderEpochContinuitySummary: NOT_STARTED.\n");
            return;
        };
        let count_class = |class: &str| state.rows.iter().filter(|row| row.class == class).count();
        let count_transition = |value: &str| {
            state
                .rows
                .iter()
                .filter(|row| row.transition_accounting == value)
                .count()
        };
        out.push_str(&format!(
            "RenderEpochContinuitySummary: Boundaries {}; RenderPathStreams {}; Seed {}; Changed {}; Unchanged {}; InterBoundaryDrift {}; TracedCallsOutsideAcceptedPaths {}; DriftExplainedByExcludedPathState {}; DriftAmbiguousExcludedPathState {}; DriftUnexplained {}; SingleTransformationBoundaries {}; BlockBoundaries {}; UnresolvedBoundaries {}; MasterRuntimeObservation NOT_OBSERVABLE_IN_AUDITONLY; OwnerAttribution BLOCK_OR_REGION_ONLY; EditAttribution OPAQUE_INDIVIDUAL_IDENTITY_AVAILABLE; FinalEpochReceipt HISTORICAL_LINEAGE_ONLY.\n",
            state.rows.len(),
            state.previous_after_by_path.len(),
            count_class("SEED"),
            state.rows.iter().filter(|row| !row.unchanged).count(),
            state.rows.iter().filter(|row| row.unchanged).count(),
            state.rows.iter().filter(|row| row.gap == "DRIFT_OR_EPOCH_RESET").count(),
            state.outside_path_calls,
            count_transition("EXPLAINED_BY_EXCLUDED_PATH_STATE"),
            count_transition("AMBIGUOUS_EXCLUDED_PATH_STATE"),
            count_transition("UNEXPLAINED"),
            count_class("SINGLE_TRANSFORMATION"),
            count_class("BLOCK"),
            count_class("SINGLE_OR_BLOCK_UNRESOLVED"),
        ));
        for (index, row) in state.rows.iter().enumerate() {
            let parsed = row.after.parsed && row.before.is_none_or(|census| census.parsed);
            let sounding = match row.before {
                _ if !parsed => "unparsed",
                None => "seed",
                Some(before) if before.sounding == row.after.sounding => "stable",
                Some(_) => "changed",
            };
            let system = census_value(row.before, row.after, |census| census.system_silence);
            let voice = census_value(row.before, row.after, |census| census.voice_label_silence);
            let levels = match row.before {
                None => "seed",
                Some(before) => {
                    let system_delta = (row.after.system_silence - before.system_silence).signum();
                    let voice_delta = (row.after.voice_label_silence - before.voice_label_silence).signum();
                    if system_delta == voice_delta { "agree" } else { "diverge" }
                }
            };
            out.push_str(&format!(
                "RenderEpochBoundary: ordinal {}; kind {}; stream {:016x}; class {}; gap {}; transition_accounting {}; transition_candidates {}; unchanged {}; visible_pitched {}; hidden_pitched {}; visible_rests {}; hidden_rests {}; forwards {}; sounding {}; system_silence {}; voice_label_silence {}; silence_levels {}; before {}; after {:016x}.\n",
                index + 1,
                row.kind,
                row.stream,
                row.class,
                row.gap,
                row.transition_accounting,
                row.transition_candidates,
                if row.unchanged { "yes" } else { "no" },
                census_value(row.before, row.after, |census| census.visible_pitched),
                census_value(row.before, row.after, |census| census.hidden_pitched),
                census_value(row.before, row.after, |census| census.visible_rests),
                census_value(row.before, row.after, |census| census.hidden_rests),
                census_value(row.before, row.after, |census| census.forwards),
                sounding,
                system,
                voice,
                levels,
                row.before_fingerprint
                    .map_or_else(|| "none".to_string(), |value| format!("{value:016x}")),
                row.after_fingerprint,
            ));
        }
    });
    crate::audit_only::destructive_carrier_effect_inventory::append_static_effect_inventory(&mut out);
    crate::audit_only::destructive_edit_attribution::append_report(&mut out);
    crate::audit_only::destructive_edit_attribution::append_identity_coverage_and_thread_loss_report(&mut out);
    crate::master_state_identity::append_master_lineage_report(&mut out);
    crate::master_state_identity::append_render_lineage_report(&mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_children_receive_only_existing_trace_options() {
        let args = ["--all", "--jobs=19", "--writer-trace=2,8", "piece"].into_iter().map(std::ffi::OsString::from);
        assert_eq!(trace_arguments(args), vec![std::ffi::OsString::from("--writer-trace=2,8")]);
        assert_eq!(trace_arguments(["--writer-trace"].into_iter().map(std::ffi::OsString::from)).len(), 1);
        assert!(trace_arguments(["--all"].into_iter().map(std::ffi::OsString::from)).is_empty());
    }

    #[test]
    fn optional_snapshots_keep_complete_large_measures() {
        let _session = start(Some("all"), "synthetic", None).unwrap();
        let payload = "x".repeat(20_000);
        let label = 43;
        let before = format!("<score-partwise><part id=\"P1\"><measure number=\"{label}\"><direction>{payload}</direction></measure></part></score-partwise>");
        let after = before.replace("</direction>", "y</direction>");
        record_after("synthetic_writer", Path::new("synthetic.render.musicxml"), Some(&before), &after);
        let report = report_body().unwrap();
        assert!(report.contains(&payload));
        assert!(!report.contains("Bytes ausgelassen"));
    }



    #[test]
    fn measure_filter_accepts_prefix_and_csv() {
        let session = start(Some("M5,7"), "Test", None).unwrap();
        assert!(trace_is_active());
        drop(session);
        assert!(!trace_is_active());
    }

    #[test]
    fn changed_measure_trace_keeps_only_real_delta() {
        let before = "<score-partwise><part id=\"P1\"><measure number=\"\
5\"><attributes><clef number=\"2\"><sign>F</sign></clef></attributes></measure></part></score-partwise>";
        let after = "<score-partwise><part id=\"P1\"><measure number=\"\
5\"><attributes><clef number=\"2\"><sign>G</sign></clef></attributes></measure></part></score-partwise>";
        let before_fragments = collect_measure_fragments(before);
        let after_fragments = collect_measure_fragments(after);
        let changed = changed_measure_fragments(&before_fragments, &after_fragments);
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0].1.unwrap().number, "5");
    }

    #[test]
    fn automatic_primary_beam_writer_delta_trace_records_exact_state_change_without_cli_trace() {
        let session = start(None, "Test", None).unwrap();
        let before = "<score-partwise><part id=\"P1\"><measure number=\"\
17\"><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff><beam number=\"1\">continue</beam></note></measure></part></score-partwise>";
        let after = "<score-partwise><part id=\"P1\"><measure number=\"\
17\"><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff><beam number=\"1\">begin</beam></note></measure></part></score-partwise>";
        record_after(
            "structural_lower_beam",
            Path::new("Test.render.musicxml"),
            Some(before),
            after,
        );
        let report = primary_beam_writer_delta_report_body();
        assert!(report.contains("writer=structural_lower_beam"));
        assert!(report.contains("measure=P1/M17"));
        assert!(report.contains("primary_beam=continue=>begin"));
        drop(session);
    }

    #[test]
    fn automatic_primary_beam_writer_delta_trace_ignores_non_beam_note_stable_delta() {
        let session = start(None, "Test", None).unwrap();
        let before = "<score-partwise><part id=\"P1\"><measure number=\"\
7\"><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff><beam number=\"1\">continue</beam></note></measure></part></score-partwise>";
        let after = "<score-partwise><part id=\"P1\"><measure number=\"\
7\"><direction><direction-type><words>x</words></direction-type></direction><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff><beam number=\"1\">continue</beam></note></measure></part></score-partwise>";
        record_after(
            "direction_relocation",
            Path::new("Test.render.musicxml"),
            Some(before),
            after,
        );
        let report = primary_beam_writer_delta_report_body();
        assert!(report.contains("PrimaryBeamWriterDeltaEntries: 0"));
        drop(session);
    }
}

#[cfg(test)]
mod render_epoch_continuity_tests {
    use super::{record_after, render_epoch_continuity_report_body, start};
    use std::path::Path;

    const SEEDED: &str = "<score-partwise><part id=\"P1\"><measure><note><rest/><duration>2</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
    const RELEASED: &str = "<score-partwise><part id=\"P1\"><measure><forward><duration>2</duration></forward><note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";
    const HIDDEN: &str = "<score-partwise><part id=\"P1\"><measure><note print-object=\"no\"><rest/><duration>2</duration><voice>1</voice><staff>1</staff></note><note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><staff>1</staff></note></measure></part></score-partwise>";

    fn boundary_path() -> &'static Path {
        Path::new("synthetic.render.musicxml")
    }

    #[test]
    fn continuity_chain_without_gap_reports_no_drift() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", boundary_path(), None, SEEDED);
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), RELEASED);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("Boundaries 2; RenderPathStreams 1; Seed 1; Changed 2; Unchanged 0; InterBoundaryDrift 0;"));
        assert!(body.contains("class SEED; gap FIRST;"));
        assert!(body.contains("gap CONTINUOUS; transition_accounting NOT_APPLICABLE; transition_candidates 0; unchanged no; visible_pitched +0; hidden_pitched +0; visible_rests -1; hidden_rests +0; forwards +1; sounding stable; system_silence -2;"));
        drop(session);
    }

    #[test]
    fn independent_render_paths_keep_independent_continuity_chains() {
        let session = start(None, "synthetic", None).expect("session");
        let main_path = Path::new("synthetic.render.musicxml");
        let shadow_path = Path::new("constraint_candidate.render.musicxml");
        record_after("synthetic_kind", main_path, None, SEEDED);
        record_after("synthetic_shadow", shadow_path, Some(HIDDEN), RELEASED);
        record_after("synthetic_kind", main_path, Some(SEEDED), RELEASED);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("Boundaries 3; RenderPathStreams 2; Seed 2; Changed 3; Unchanged 0; InterBoundaryDrift 0;"));
        assert_eq!(body.matches("gap FIRST;").count(), 2);
        assert_eq!(body.matches("gap CONTINUOUS;").count(), 1);
        drop(session);
    }

    #[test]
    fn untraced_state_change_between_boundaries_is_drift() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), SEEDED);
        record_after("synthetic_kind", boundary_path(), Some(HIDDEN), RELEASED);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("InterBoundaryDrift 1;"));
        assert!(body.contains("gap DRIFT_OR_EPOCH_RESET;"));
        drop(session);
    }

    #[test]
    fn identical_bytes_between_boundaries_are_not_drift() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), HIDDEN);
        record_after("synthetic_kind", boundary_path(), Some(HIDDEN), HIDDEN);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("InterBoundaryDrift 0;"));
        assert!(body.contains("Unchanged 1;"));
        drop(session);
    }

    #[test]
    fn calls_outside_accepted_paths_are_counted_without_rows() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", Path::new("synthetic.render.musicxml.tmp"), Some(SEEDED), RELEASED);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("Boundaries 0;"));
        assert!(body.contains("TracedCallsOutsideAcceptedPaths 1;"));
        drop(session);
    }

    #[test]
    fn a_new_project_session_restarts_the_chain() {
        let first = start(None, "synthetic_one", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), RELEASED);
        drop(first);
        let second = start(None, "synthetic_two", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(HIDDEN), HIDDEN);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("Boundaries 1;"));
        assert!(body.contains("InterBoundaryDrift 0;"));
        assert!(body.contains("gap FIRST;"));
        drop(second);
    }



    #[test]
    fn excluded_path_state_can_explain_one_same_stream_drift() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), SEEDED);
        record_after(
            "synthetic_excluded",
            Path::new("synthetic.render.musicxml.tmp"),
            None,
            HIDDEN,
        );
        record_after("synthetic_kind", boundary_path(), Some(HIDDEN), RELEASED);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("InterBoundaryDrift 1;"));
        assert!(body.contains("DriftExplainedByExcludedPathState 1;"));
        assert!(body.contains("DriftAmbiguousExcludedPathState 0;"));
        assert!(body.contains("DriftUnexplained 0;"));
        assert!(body.contains(
            "gap DRIFT_OR_EPOCH_RESET; transition_accounting EXPLAINED_BY_EXCLUDED_PATH_STATE; transition_candidates 1;"
        ));
        drop(session);
    }

    #[test]
    fn duplicate_excluded_path_states_keep_drift_ambiguous() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), SEEDED);
        record_after(
            "synthetic_excluded_a",
            Path::new("synthetic.render.musicxml.a.tmp"),
            None,
            HIDDEN,
        );
        record_after(
            "synthetic_excluded_b",
            Path::new("synthetic.render.musicxml.b.tmp"),
            None,
            HIDDEN,
        );
        record_after("synthetic_kind", boundary_path(), Some(HIDDEN), RELEASED);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("InterBoundaryDrift 1;"));
        assert!(body.contains("DriftExplainedByExcludedPathState 0;"));
        assert!(body.contains("DriftAmbiguousExcludedPathState 1;"));
        assert!(body.contains("DriftUnexplained 0;"));
        assert!(body.contains(
            "gap DRIFT_OR_EPOCH_RESET; transition_accounting AMBIGUOUS_EXCLUDED_PATH_STATE; transition_candidates 2;"
        ));
        drop(session);
    }

    #[test]
    fn unmatched_excluded_path_state_does_not_explain_drift() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), SEEDED);
        record_after(
            "synthetic_excluded",
            Path::new("synthetic.render.musicxml.tmp"),
            None,
            RELEASED,
        );
        record_after("synthetic_kind", boundary_path(), Some(HIDDEN), RELEASED);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("InterBoundaryDrift 1;"));
        assert!(body.contains("DriftExplainedByExcludedPathState 0;"));
        assert!(body.contains("DriftAmbiguousExcludedPathState 0;"));
        assert!(body.contains("DriftUnexplained 1;"));
        assert!(body.contains(
            "gap DRIFT_OR_EPOCH_RESET; transition_accounting UNEXPLAINED; transition_candidates 0;"
        ));
        drop(session);
    }

    #[test]
    fn census_separates_visible_and_hidden_carriers() {
        let session = start(None, "synthetic", None).expect("session");
        record_after("synthetic_kind", boundary_path(), Some(SEEDED), HIDDEN);
        let body = render_epoch_continuity_report_body();
        assert!(body.contains("visible_rests -1; hidden_rests +1; forwards +0; sounding stable; system_silence -2; voice_label_silence -2; silence_levels agree;"));
        assert!(body.contains("DestructiveCarrierEffectInventory: SourceFiles"));
        drop(session);
    }
}
