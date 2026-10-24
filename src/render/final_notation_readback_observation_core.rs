// v0.10.1705 FinalNotationReadbackObservationBoundary:
// One typed, writer-free owner for the invariants repeatedly re-read by the
// final PDF-source candidate stages. No candidate selection or mutation lives
// here; consumers receive only parse/cursor/event signatures for readback.

use super::super::{collect_render_xml_note_events, scan_raw_cursor_xml, RenderXmlNoteEvent};

pub(super) type FinalNotationRawCursorRow =
    (usize, i64, i64, i64, i64, usize, usize, usize);

#[derive(Debug)]
pub(super) struct FinalNotationReadbackObservation {
    pub(super) parse_ok: bool,
    pub(super) backup_forward_ok: bool,
    pub(super) visible_rests: usize,
    pub(super) note_count: usize,
    pub(super) raw_cursor_signature: Vec<FinalNotationRawCursorRow>,
    pub(super) pitched_timeline_signature: Vec<String>,
    pub(super) pitched_payload_signature: Vec<String>,
}

fn pitched_timeline_signature(events: &[RenderXmlNoteEvent]) -> Vec<String> {
    let mut rows = events
        .iter()
        .filter_map(|event| {
            event.midi.map(|midi| {
                format!(
                    "{}|{}|{}|{}|{}|{}|{}|{}",
                    event.scope,
                    event.staff,
                    event.voice,
                    event.onset,
                    event.duration,
                    midi,
                    event.is_hidden,
                    event.is_chord
                )
            })
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn pitched_payload_signature(events: &[RenderXmlNoteEvent]) -> Vec<String> {
    let mut rows = events
        .iter()
        .filter_map(|event| {
            event.midi.map(|midi| {
                format!(
                    "{}|{}|{}|{}|{}|{}|{}",
                    event.scope,
                    event.staff,
                    event.voice,
                    event.duration,
                    midi,
                    event.is_hidden,
                    event.is_chord,
                )
            })
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn raw_cursor_signature(xml: &str) -> Vec<FinalNotationRawCursorRow> {
    scan_raw_cursor_xml("SilentCoverage", xml)
        .into_iter()
        .map(|row| {
            (
                row.measure,
                row.expected_duration,
                row.cursor_min,
                row.cursor_max,
                row.cursor_final,
                row.partial_backup_count,
                row.overflow_events,
                row.post_expected_events,
            )
        })
        .collect()
}

pub(super) fn observe_final_notation_readback(
    xml: &str,
) -> FinalNotationReadbackObservation {
    let (events, backup_forward_ok) = collect_render_xml_note_events(xml);
    FinalNotationReadbackObservation {
        parse_ok: roxmltree::Document::parse(xml).is_ok(),
        backup_forward_ok,
        visible_rests: events
            .iter()
            .filter(|event| event.is_rest && !event.is_hidden)
            .count(),
        note_count: events.len(),
        raw_cursor_signature: raw_cursor_signature(xml),
        pitched_timeline_signature: pitched_timeline_signature(&events),
        pitched_payload_signature: pitched_payload_signature(&events),
    }
}
