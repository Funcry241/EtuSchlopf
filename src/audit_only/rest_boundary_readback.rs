//! Read-only byte evidence from already observed checkpoints and successful sinks.
//! A snapshot number, hash or XML range is a physical locator, never an identity,
//! canonical role, owner receipt, silence obligation or permission to change music.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Default)]
struct Readback {
    documents: Vec<String>,
    by_hash: BTreeMap<String, Vec<usize>>,
    observations: Vec<Observation>,
}
struct Observation {
    kind: &'static str,
    label: String,
    before: Option<usize>,
    after: Option<usize>,
}
thread_local! {
    static STATE: RefCell<Option<Readback>> = const { RefCell::new(None) };
}

pub(super) fn start() {
    STATE.with(|state| *state.borrow_mut() = Some(Readback::default()));
}
pub(super) fn clear() {
    STATE.with(|state| *state.borrow_mut() = None);
}

impl Readback {
    fn intern(&mut self, xml: &str) -> usize {
        let hash = crate::sha256::hex(xml.as_bytes());
        // Full-byte equality, including when hashes coincide. No first-wins identity.
        if let Some(index) = self.by_hash.get(&hash).and_then(|indices| {
            indices
                .iter()
                .copied()
                .find(|index| self.documents[*index] == xml)
        }) {
            return index;
        }
        let index = self.documents.len();
        self.documents.push(xml.to_string());
        self.by_hash.entry(hash).or_default().push(index);
        index
    }
    fn observe(
        &mut self,
        kind: &'static str,
        label: &str,
        before: Option<&str>,
        after: Option<&str>,
    ) {
        let before = before.map(|xml| self.intern(xml));
        let after = after.map(|xml| self.intern(xml));
        self.observations.push(Observation {
            kind,
            label: label.to_string(),
            before,
            after,
        });
    }
}

pub(super) fn observe(kind: &'static str, label: &str, before: Option<&str>, after: Option<&str>) {
    STATE.with(|state| {
        if let Some(state) = state.borrow_mut().as_mut() {
            state.observe(kind, label, before, after);
        }
    });
}

fn snapshot_label(index: Option<usize>) -> String {
    index.map_or_else(
        || "UNAVAILABLE".to_string(),
        |index| format!("D{}", index + 1),
    )
}

pub(super) fn append_report(out: &mut String) {
    out.push_str("\nRestBoundaryByteReadback: active.\n");
    out.push_str("Authority: none; MusicMutation: nein; RoleBinding: not_inferred; RestObligationBinding: not_inferred.\n");
    out.push_str("Scope: existing same-thread lineage checkpoints, actual successful Render sink operands and committed FinalChord adoption operands; candidate-only states, unobserved steps and foreign threads are not covered.\n");
    out.push_str("Semantics: each observation names its actual operands only; adjacent rows do not imply a transition; a sink label identifies a persistence boundary, not an individual musical writer.\n");
    out.push_str("Locators: D/M numbers, document/measure SHA256, ranges and XML labels locate captured bytes only; equal coordinates or bytes do not establish semantic identity or role. Missing before is unavailable, never an empty or proven silent source.\n");
    STATE.with(|slot| {
        let slot = slot.borrow();
        let Some(state) = slot.as_ref() else {
            out.push_str("RestReadbackSession: UNAVAILABLE.\n");
            return;
        };
        append_state(out, state);
    });
}

fn append_state(out: &mut String, state: &Readback) {
    let _ = writeln!(
        out,
        "RestReadbackSummary: observations {}; distinct_documents {}.",
        state.observations.len(),
        state.documents.len()
    );
    for (index, row) in state.observations.iter().enumerate() {
        let _ = writeln!(
            out,
            "RestReadbackObservation: ordinal {}; kind {}; label {:?}; before {}; after {}.",
            index + 1,
            row.kind,
            row.label,
            snapshot_label(row.before),
            snapshot_label(row.after)
        );
    }
    // Deduplicate only byte-identical measure text; every physical occurrence
    // keeps its own document/range reference, even for duplicated labels.
    let mut measures = Vec::<String>::new();
    let mut by_hash = BTreeMap::<String, Vec<usize>>::new();
    for (index, xml) in state.documents.iter().enumerate() {
        let hash = crate::sha256::hex(xml.as_bytes());
        let parsed = roxmltree::Document::parse(xml);
        let Ok(document) = parsed else {
            let _ = writeln!(
                out,
                "RestReadbackDocument: D{}; sha256 {}; bytes {}; parse UNAVAILABLE; measures UNAVAILABLE.",
                index + 1,
                hash,
                xml.len()
            );
            continue;
        };
        let parts = document
            .root_element()
            .children()
            .filter(|node| node.has_tag_name("part"));
        let mut count = 0usize;
        for (part_index, part) in parts.enumerate() {
            for (measure_index, measure) in part
                .children()
                .filter(|node| node.has_tag_name("measure"))
                .enumerate()
            {
                count += 1;
                let range = measure.range();
                let raw = &xml[range.clone()];
                let measure_hash = crate::sha256::hex(raw.as_bytes());
                let existing = by_hash.get(&measure_hash).and_then(|indices| {
                    indices
                        .iter()
                        .copied()
                        .find(|index| measures[*index] == raw)
                });
                let measure_id = existing.unwrap_or_else(|| {
                    let id = measures.len();
                    measures.push(raw.to_string());
                    by_hash.entry(measure_hash).or_default().push(id);
                    id
                });
                let _ = writeln!(
                    out,
                    "RestReadbackOccurrence: D{}; part_ordinal {}; part_label {:?}; measure_ordinal {}; measure_label {:?}; xml_range {}..{}; M{}.",
                    index + 1,
                    part_index + 1,
                    part.attribute("id"),
                    measure_index + 1,
                    measure.attribute("number"),
                    range.start,
                    range.end,
                    measure_id + 1
                );
            }
        }
        let _ = writeln!(
            out,
            "RestReadbackDocument: D{}; sha256 {}; bytes {}; parse OK; measures {}.",
            index + 1,
            hash,
            xml.len(),
            count
        );
    }
    for (index, xml) in measures.iter().enumerate() {
        let _ = writeln!(
            out,
            "RestReadbackMeasure: M{}; sha256 {}; bytes {}.",
            index + 1,
            crate::sha256::hex(xml.as_bytes()),
            xml.len()
        );
        let _ = writeln!(out, "RestReadbackXmlBegin M{}", index + 1);
        // Prefix every physical line so captured XML cannot open report sections.
        // Removing the prefix and taking the declared byte count restores the
        // exact original XML, including CRLF and a non-newline-terminated tail.
        for segment in xml.split_inclusive('\n') {
            out.push_str("RestReadbackXmlLine: ");
            out.push_str(segment);
            if !segment.ends_with('\n') {
                out.push('\n');
            }
        }
        let _ = writeln!(out, "RestReadbackXmlEnd M{}", index + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn score(label: &str, body: &str) -> String {
        format!(
            "<score-partwise><part id=\"{label}\"><measure number=\"x\">{body}</measure></part></score-partwise>"
        )
    }
    fn report(state: &Readback) -> String {
        let mut out = String::new();
        append_state(&mut out, state);
        out
    }
    #[test]
    fn actual_before_and_after_are_byte_exact_and_noop_is_not_lost() {
        let before = score("opaque", "<note><rest/><duration>4</duration></note>");
        let after = score("opaque", "<forward><duration>4</duration></forward>");
        let mut state = Readback::default();
        state.observe("sink", "boundary", Some(&before), Some(&after));
        state.observe("sink", "unchanged", Some(&after), Some(&after));
        assert_eq!(state.documents, vec![before.clone(), after.clone()]);
        assert_eq!(state.observations[1].before, state.observations[1].after);
        let out = report(&state);
        assert!(out.contains("before D1; after D2"));
        assert!(out.contains("before D2; after D2"));
        assert!(out.contains("<note><rest/><duration>4</duration></note>"));
        assert!(out.contains("<forward><duration>4</duration></forward>"));
        assert!(out.contains(&crate::sha256::hex(before.as_bytes())));
    }
    #[test]
    fn absent_and_malformed_are_never_reported_as_empty_sources() {
        let mut state = Readback::default();
        state.observe("sink", "seed", None, Some("not xml"));
        let out = report(&state);
        assert!(out.contains("before UNAVAILABLE; after D1"));
        assert!(out.contains("parse UNAVAILABLE; measures UNAVAILABLE"));
        assert!(!out.contains("parse OK"));
    }
    #[test]
    fn repeated_labels_and_independent_duplicates_keep_all_occurrences() {
        let measure =
            "<measure number=\"same\"><forward><duration>9</duration></forward></measure>";
        let xml = format!(
            "<score-partwise><part id=\"same\">{measure}{measure}</part><part id=\"same\">{measure}</part></score-partwise>"
        );
        let mut state = Readback::default();
        state.observe("checkpoint", "same", None, Some(&xml));
        let out = report(&state);
        assert_eq!(out.matches("RestReadbackOccurrence:").count(), 3);
        assert_eq!(out.matches("RestReadbackMeasure:").count(), 1);
        assert!(out.contains("part_ordinal 2"));
        assert!(out.contains("measure_ordinal 2"));
    }
    #[test]
    fn equal_hash_index_does_not_replace_full_byte_equality() {
        let mut state = Readback::default();
        state.documents.push("different".to_string());
        let xml = score("renamed", "<note print-object=\"no\"><rest/></note>");
        state
            .by_hash
            .insert(crate::sha256::hex(xml.as_bytes()), vec![0]);
        assert_eq!(state.intern(&xml), 1);
        assert_eq!(state.intern(&xml), 1);
        assert_eq!(state.documents[0], "different");
    }
    #[test]
    fn lifecycle_reset_and_foreign_thread_do_not_mix_sessions() {
        clear();
        observe("checkpoint", "outside", None, Some("outside"));
        start();
        observe("checkpoint", "owned", None, Some("owned"));
        std::thread::spawn(|| observe("checkpoint", "foreign", None, Some("foreign")))
            .join()
            .unwrap();
        STATE.with(|slot| assert_eq!(slot.borrow().as_ref().unwrap().observations.len(), 1));
        start();
        STATE.with(|slot| assert!(slot.borrow().as_ref().unwrap().observations.is_empty()));
        clear();
        let mut out = String::new();
        append_report(&mut out);
        assert!(out.contains("RestReadbackSession: UNAVAILABLE"));
    }
    #[test]
    fn default_session_captures_sinks_checkpoints_and_collision_free_adoption() {
        let path = std::path::Path::new("synthetic.render.musicxml");
        let session =
            crate::audit_only::writer_trace::start(None, "synthetic", Some(path)).unwrap();
        let xml = score("opaque", "<note><rest/><duration>4</duration></note>");
        crate::audit_only::writer_trace::record_after("sink", path, None, &xml);
        crate::audit_only::writer_trace::record_after("sink", path, Some(&xml), &xml);
        crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
            "checkpoint",
            &xml,
        );
        crate::audit_only::voice_run_collision_origin::observe_final_chord_adoption(
            "adopted", &xml, &xml, &xml,
        );
        let out = crate::audit_only::writer_trace::render_epoch_continuity_report_body();
        assert!(out.contains("RestReadbackSummary: observations 5; distinct_documents 1"));
        assert!(out.contains("successful_render_sink"));
        assert!(out.contains("existing_lineage_checkpoint"));
        assert!(out.contains("committed_final_chord_projection_operands"));
        assert!(out.contains("committed_final_chord_continuation_operands"));
        assert!(crate::audit_only::writer_trace::report_body().is_none());
        drop(session);
        let mut ended = String::new();
        append_report(&mut ended);
        assert!(ended.contains("RestReadbackSession: UNAVAILABLE"));
    }
    #[test]
    fn raw_measure_bytes_include_unbound_forwards_hidden_rests_and_chord_members() {
        let body = "<forward><duration>2</duration></forward><backup><duration>2</duration></backup><note print-object='no'><rest/><duration>2</duration></note><note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration></note><note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>2</duration></note>";
        let xml = score("arbitrary", body);
        let mut state = Readback::default();
        state.observe("checkpoint", "arbitrary", None, Some(&xml));
        assert!(report(&state).contains(body));
        assert!(!report(&state).contains("<voice>"));
        assert!(!report(&state).contains("<staff>"));
    }
    #[test]
    fn complete_epoch_body_passes_the_actual_owned_section_projection() {
        let path = std::path::Path::new("synthetic.render.musicxml");
        let _session =
            crate::audit_only::writer_trace::start(None, "synthetic", Some(path)).unwrap();
        let xml = score("opaque", "<note><rest/><duration>4</duration></note>");
        crate::audit_only::writer_trace::record_after("sink", path, None, &xml);
        let body = crate::audit_only::writer_trace::render_epoch_continuity_report_body();
        let mut report =
            crate::final_report::DraftReport::new(String::new()).compact(ToOwned::to_owned);
        report
            .project_owned_section(
                "render_epoch_continuity",
                "RENDER-EPOCH-CONTINUITY",
                "-----------------------",
                |out| out.push_str(&body),
            )
            .unwrap();
        assert!(
            report
                .render_snapshot()
                .contains("RestReadbackXmlLine: <measure")
        );
        let rejected_body = body.replacen(
            "RestBoundaryByteReadback: active.\n",
            "REST-BOUNDARY-BYTE-READBACK\n---------------------------\n",
            1,
        );
        let mut rejected =
            crate::final_report::DraftReport::new(String::new()).compact(ToOwned::to_owned);
        let error = rejected
            .project_owned_section(
                "render_epoch_continuity",
                "RENDER-EPOCH-CONTINUITY",
                "-----------------------",
                |out| out.push_str(&rejected_body),
            )
            .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("nicht deklarierte Sektion 'REST-BOUNDARY-BYTE-READBACK'")
        );
    }
    #[test]
    fn captured_xml_lines_roundtrip_and_cannot_open_foreign_sections() {
        start();
        let xml = score(
            "opaque",
            "<direction><direction-type><words>\r\nFOREIGN-SECTION\r\n---------------\r\nGrüße\r\n</words></direction-type></direction><note><rest/></note>",
        );
        observe("checkpoint", "synthetic", None, Some(&xml));
        let mut body = String::new();
        append_report(&mut body);
        let mut report =
            crate::final_report::DraftReport::new(String::new()).compact(ToOwned::to_owned);
        report
            .project_owned_section(
                "render_epoch_continuity",
                "RENDER-EPOCH-CONTINUITY",
                "-----------------------",
                |out| out.push_str(&body),
            )
            .unwrap();
        let snapshot = report.render_snapshot();
        let mut recovered = snapshot
            .split_inclusive('\n')
            .filter_map(|line| line.strip_prefix("RestReadbackXmlLine: "))
            .collect::<String>();
        let doc = roxmltree::Document::parse(&xml).unwrap();
        let measure = doc
            .descendants()
            .find(|node| node.has_tag_name("measure"))
            .unwrap();
        let original = &xml[measure.range()];
        recovered.truncate(original.len());
        assert_eq!(recovered.as_bytes(), original.as_bytes());
        clear();
        let mut unavailable = String::new();
        append_report(&mut unavailable);
        let mut report =
            crate::final_report::DraftReport::new(String::new()).compact(ToOwned::to_owned);
        report
            .project_owned_section(
                "render_epoch_continuity",
                "RENDER-EPOCH-CONTINUITY",
                "-----------------------",
                |out| out.push_str(&unavailable),
            )
            .unwrap();
    }
}
