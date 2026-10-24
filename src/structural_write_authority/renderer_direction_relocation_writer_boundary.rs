//! Capability-typed writer for the renderer-neutral direction relocation.
//!
//! The verifier token is first consumed against the exact pre-writer shadow
//! snapshot for which it was issued.  The proven direction-only delta is then
//! rebound to the current final note/beam sink.  This composition is allowed
//! only when the source direction and its metric host are unique and when every
//! note ordinal and every beam state of the sink remain unchanged.

use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt};

pub(crate) struct DirectionRelocationExactMint { _sealed: () }

use super::locator_resolution::stable_xml_snapshot_fingerprint;
use super::renderer_direction_relocation_authorization::{
    canonical_direction_without_offset_fingerprint, direct_child_i64, direct_child_text,
    AuthorizedRendererDirectionRelocation, RendererDirectionRelocationWriterInstruction,
};
use roxmltree::{Document, Node};
use std::ops::Range;

#[derive(Debug, Clone, Default)]
pub(super) struct RendererDirectionRelocationWriterAudit {
    pub(super) token_handed_to_writer: bool,
    pub(super) token_consumed: bool,
    pub(super) writer_token_used: bool,
    pub(super) source_snapshot_fingerprint_match: bool,
    pub(super) bound_direction_range_valid: bool,
    pub(super) bound_host_insertion_valid: bool,
    pub(super) exact_edits_applied: usize,
    pub(super) parse_after_ok: bool,
    pub(super) expected_post_snapshot_fingerprint_match: bool,
    pub(super) sink_snapshot_parse_ok: bool,
    pub(super) sink_direction_target_unique: bool,
    pub(super) sink_note_beam_preserved: bool,
    pub(super) sink_composition_applied: bool,
    pub(super) write_produced: bool,
}

pub(super) fn authorized_renderer_direction_relocation_writer_entry(
    token: AuthorizedRendererDirectionRelocation,
    pre_writer_xml: &str,
    note_beam_sink_xml: &str,
    _source_stage: &str,
) -> (String, String, RendererDirectionRelocationWriterAudit, Option<ExactTransformationReceipt>) {
    let mut audit = RendererDirectionRelocationWriterAudit {
        token_handed_to_writer: true,
        token_consumed: true,
        ..RendererDirectionRelocationWriterAudit::default()
    };

    let Some(instruction) = token.into_writer_instruction(pre_writer_xml) else {
        return (pre_writer_xml.to_string(), pre_writer_xml.to_string(), audit, None);
    };
    audit.writer_token_used = true;
    audit.source_snapshot_fingerprint_match = true;
    audit.bound_direction_range_valid = instruction.direction_range.start
        < instruction.direction_range.end
        && instruction.direction_range.end <= pre_writer_xml.len();
    audit.bound_host_insertion_valid = instruction.host_insert_at
        < instruction.direction_range.start
        && instruction.host_insert_at <= pre_writer_xml.len();
    if !audit.bound_direction_range_valid || !audit.bound_host_insertion_valid {
        return (pre_writer_xml.to_string(), pre_writer_xml.to_string(), audit, None);
    }

    let Some(shadow_candidate) = apply_exact_bound_edits(
        pre_writer_xml,
        instruction.direction_range.clone(),
        instruction.host_insert_at,
        &instruction.insertion_text,
    ) else {
        return (pre_writer_xml.to_string(), pre_writer_xml.to_string(), audit, None);
    };
    audit.exact_edits_applied = 2;
    audit.expected_post_snapshot_fingerprint_match =
        stable_xml_snapshot_fingerprint(&shadow_candidate)
            == instruction.expected_post_snapshot_fingerprint;

    audit.sink_snapshot_parse_ok = Document::parse(note_beam_sink_xml).is_ok();
    let sink_before_note_beam = note_beam_fingerprint(note_beam_sink_xml);
    let (composed, receipt) = compose_authorized_direction_delta_onto_sink(
        note_beam_sink_xml,
        &instruction,
    );
    audit.sink_direction_target_unique = composed.is_some();
    let composed_candidate = composed.unwrap_or_else(|| note_beam_sink_xml.to_string());
    audit.parse_after_ok = Document::parse(&composed_candidate).is_ok();
    audit.sink_note_beam_preserved = sink_before_note_beam.is_some()
        && sink_before_note_beam == note_beam_fingerprint(&composed_candidate);
    audit.sink_composition_applied = composed_candidate != note_beam_sink_xml;

    audit.write_produced = audit.writer_token_used
        && audit.source_snapshot_fingerprint_match
        && audit.bound_direction_range_valid
        && audit.bound_host_insertion_valid
        && audit.exact_edits_applied == 2
        && audit.expected_post_snapshot_fingerprint_match
        && audit.sink_snapshot_parse_ok
        && audit.sink_direction_target_unique
        && audit.parse_after_ok
        && audit.sink_note_beam_preserved
        && audit.sink_composition_applied;
    if audit.write_produced {
        (composed_candidate, shadow_candidate, audit, receipt)
    } else {
        let mut rejected = crate::master_state_identity::RenderTransitionReceipts::new();
        rejected.push_exact(receipt);
        rejected.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        (pre_writer_xml.to_string(), pre_writer_xml.to_string(), audit, None)
    }
}

fn compose_authorized_direction_delta_onto_sink(
    sink_xml: &str,
    instruction: &RendererDirectionRelocationWriterInstruction,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let execution = (|| {
    let relocated_direction = instruction.insertion_text.trim();
    let relocated_document = Document::parse(relocated_direction).ok()?;
    let relocated_root = relocated_document.root_element();
    if !relocated_root.has_tag_name("direction")
        || canonical_direction_without_offset_fingerprint(relocated_direction)?
            != instruction.canonical_payload_without_offset_fingerprint
        || direct_child_i64(relocated_root, "offset")? != instruction.proposed_offset
    {
        return None;
    }

    let document = Document::parse(sink_xml).ok()?;
    let mut matches = Vec::<(Range<usize>, usize, String)>::new();
    for measure in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
    {
        let records = direct_child_cursor_records(measure);
        if records.len() < 4 {
            continue;
        }
        for index in 1..records.len().saturating_sub(2) {
            let (direction, direction_before, _) = records[index];
            if !direction.has_tag_name("direction")
                || direction_before != instruction.source_effective_cursor
                || direction
                    .children()
                    .any(|child| child.is_element() && child.has_tag_name("offset"))
            {
                continue;
            }
            let direction_range = direction.range();
            let direction_xml = sink_xml.get(direction_range.clone())?;
            if stable_xml_snapshot_fingerprint(direction_xml)
                != instruction.original_direction_xml_fingerprint
                || canonical_direction_without_offset_fingerprint(direction_xml)?
                    != instruction.canonical_payload_without_offset_fingerprint
            {
                continue;
            }

            let (host, host_before, host_after) = records[index - 1];
            let (right, right_before, right_after) = records[index + 1];
            let (last, last_before, last_after) = records[index + 2];
            if !host.has_tag_name("note")
                || !right.has_tag_name("note")
                || !last.has_tag_name("note")
                || host_before != instruction.proposed_host_cursor
                || host_after != instruction.source_effective_cursor
                || right_before != instruction.source_effective_cursor
                || right_after <= right_before
                || last_before != right_after
                || last_after != instruction.group_end_cursor
            {
                continue;
            }
            let direction_staff = direct_child_text(direction, "staff");
            let host_staff = direct_child_text(host, "staff");
            let right_staff = direct_child_text(right, "staff");
            if direction_staff.is_none()
                || direction_staff != right_staff
                || direction_staff == host_staff
            {
                continue;
            }

            let host_range = host.range();
            let host_indent_start = sink_xml[..host_range.start]
                .rfind('\n')
                .map_or(0, |position| position + 1);
            let indent_candidate = sink_xml.get(host_indent_start..host_range.start)?;
            let host_indent = if indent_candidate.chars().all(char::is_whitespace) {
                indent_candidate
            } else {
                ""
            };
            let insertion_text = format!("{relocated_direction}\n{host_indent}");
            matches.push((direction_range, host_range.start, insertion_text));
        }
    }
    if matches.len() != 1 {
        return None;
    }
    let (direction_range, host_insert_at, insertion_text) = matches.remove(0);
    let candidate = apply_exact_bound_edits(
        sink_xml,
        direction_range.clone(),
        host_insert_at,
        &insertion_text,
    )?;
    if candidate == sink_xml
        || Document::parse(&candidate).is_err()
        || note_beam_fingerprint(&candidate) != note_beam_fingerprint(sink_xml)
        || !relocated_direction_is_unique_and_bound(&candidate, instruction)
    {
        return None;
    }
    // Both edits are bound to the historical sink pre, never to #2's post.
    let geometry = vec![
        ExactEdit::bound(direction_range.start, direction_range.end, String::new()),
        ExactEdit::bound(host_insert_at, host_insert_at, insertion_text),
    ];
    Some((candidate, geometry))
    })();
    let Some((candidate, geometry)) = execution else { return (None, None); };
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        DirectionRelocationExactMint { _sealed: () }, sink_xml, &candidate, geometry,
    );
    assert!(sink_xml == candidate || receipt.is_some(), "Direction changed bytes require its own EXACT receipt");
    (Some(candidate), receipt)
}

fn direct_child_cursor_records<'a, 'input>(
    measure: Node<'a, 'input>,
) -> Vec<(Node<'a, 'input>, i64, i64)> {
    let mut records = Vec::new();
    let mut cursor = 0_i64;
    for child in measure.children().filter(Node::is_element) {
        let before = cursor;
        if child.has_tag_name("backup") {
            cursor = cursor.saturating_sub(direct_child_i64(child, "duration").unwrap_or(0));
        } else if child.has_tag_name("forward") {
            cursor = cursor.saturating_add(direct_child_i64(child, "duration").unwrap_or(0));
        } else if child.has_tag_name("note") {
            let chord = child
                .children()
                .any(|node| node.is_element() && node.has_tag_name("chord"));
            if !chord {
                cursor = cursor.saturating_add(direct_child_i64(child, "duration").unwrap_or(0));
            }
        }
        records.push((child, before, cursor));
    }
    records
}

fn relocated_direction_is_unique_and_bound(
    xml: &str,
    instruction: &RendererDirectionRelocationWriterInstruction,
) -> bool {
    let Ok(document) = Document::parse(xml) else {
        return false;
    };
    let mut matches = 0_usize;
    for measure in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("measure"))
    {
        let records = direct_child_cursor_records(measure);
        for (index, (direction, before, _)) in records.iter().enumerate() {
            if !direction.has_tag_name("direction")
                || *before != instruction.proposed_host_cursor
                || direct_child_i64(*direction, "offset") != Some(instruction.proposed_offset)
                || canonical_direction_without_offset_fingerprint(
                    xml.get(direction.range()).unwrap_or_default(),
                ) != Some(instruction.canonical_payload_without_offset_fingerprint)
                || index + 1 >= records.len()
            {
                continue;
            }
            let (_, host_before, host_after) = records[index + 1];
            if host_before == instruction.proposed_host_cursor
                && host_after == instruction.source_effective_cursor
                && before.saturating_add(instruction.proposed_offset)
                    == instruction.source_effective_cursor
            {
                matches = matches.saturating_add(1);
            }
        }
    }
    matches == 1
}

fn note_beam_fingerprint(xml: &str) -> Option<(usize, u64)> {
    let document = Document::parse(xml).ok()?;
    let mut canonical = String::new();
    let mut note_count = 0_usize;
    for note in document
        .descendants()
        .filter(|node| node.is_element() && node.has_tag_name("note"))
    {
        note_count = note_count.saturating_add(1);
        canonical.push_str(&note_count.to_string());
        canonical.push(':');
        let mut has_beam = false;
        for beam in note
            .children()
            .filter(|node| node.is_element() && node.has_tag_name("beam"))
        {
            has_beam = true;
            canonical.push_str(beam.attribute("number").unwrap_or("?"));
            canonical.push('=');
            canonical.push_str(beam.text().unwrap_or("").trim());
            canonical.push(';');
        }
        if !has_beam {
            canonical.push('-');
        }
        canonical.push('|');
    }
    Some((note_count, stable_xml_snapshot_fingerprint(&canonical)))
}

fn apply_exact_bound_edits(
    source_xml: &str,
    direction_range: Range<usize>,
    host_insert_at: usize,
    insertion_text: &str,
) -> Option<String> {
    if host_insert_at >= direction_range.start
        || direction_range.end > source_xml.len()
        || host_insert_at > source_xml.len()
        || insertion_text.is_empty()
    {
        return None;
    }
    let mut out = String::with_capacity(source_xml.len() + insertion_text.len());
    out.push_str(source_xml.get(..host_insert_at)?);
    out.push_str(insertion_text);
    out.push_str(source_xml.get(host_insert_at..direction_range.start)?);
    out.push_str(source_xml.get(direction_range.end..)?);
    Some(out)
}

#[cfg(test)]
pub(crate) fn executed_cursor_then_direction_events_for_render_lineage_test() -> (
    String,
    String,
    crate::master_state_identity::ExactTransformationReceipt,
    String,
    crate::master_state_identity::ExactTransformationReceipt,
) {
    let (pre, file_after_cursor, cursor_receipt) =
        super::writer_entry_boundary::tests::executed_cursor_event();
    let doc = Document::parse(&pre).unwrap();
    let direction = doc
        .descendants()
        .find(|node| node.has_tag_name("direction"))
        .unwrap();
    let measure = direction.parent().unwrap();
    let records = direct_child_cursor_records(measure);
    let index = records
        .iter()
        .position(|(node, _, _)| *node == direction)
        .unwrap();
    let (host, host_before, _) = records[index - 1];
    let (_, direction_before, _) = records[index];
    let direction_xml = &pre[direction.range()];
    let insertion = direction_xml.replace(
        "<staff>1</staff>",
        &format!("<offset>{}</offset><staff>1</staff>", direction_before - host_before),
    );
    let plan = RendererDirectionRelocationWriterInstruction {
        direction_range: direction.range(),
        host_insert_at: host.range().start,
        insertion_text: insertion,
        expected_post_snapshot_fingerprint: 1,
        canonical_payload_without_offset_fingerprint:
            canonical_direction_without_offset_fingerprint(direction_xml).unwrap(),
        original_direction_xml_fingerprint: stable_xml_snapshot_fingerprint(direction_xml),
        proposed_host_cursor: host_before,
        source_effective_cursor: direction_before,
        proposed_offset: direction_before - host_before,
        group_end_cursor: 384,
    };
    let (post, receipt) = compose_authorized_direction_delta_onto_sink(&pre, &plan);
    (
        pre,
        file_after_cursor,
        cursor_receipt,
        post.unwrap(),
        receipt.unwrap(),
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(beam: &str, duplicate_direction: bool) -> String {
        let direction = "<direction><direction-type><dynamics><f/></dynamics></direction-type><staff>2</staff></direction>";
        let duplicate = if duplicate_direction { direction } else { "" };
        format!(
            "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>4</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>1</staff><beam number=\"1\">begin</beam></note>{direction}{duplicate}<note><pitch><step>D</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>2</staff><beam number=\"1\">{beam}</beam></note><note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><staff>2</staff><beam number=\"1\">end</beam></note></measure></part></score-partwise>"
        )
    }

    fn instruction(source: &str) -> RendererDirectionRelocationWriterInstruction {
        let document = Document::parse(source).unwrap();
        let direction = document
            .descendants()
            .find(|node| node.has_tag_name("direction"))
            .unwrap();
        let host = document
            .descendants()
            .find(|node| node.has_tag_name("note"))
            .unwrap();
        let direction_xml = &source[direction.range()];
        let relocated = direction_xml.replace(
            "<staff>2</staff>",
            "<offset>4</offset><staff>2</staff>",
        );
        RendererDirectionRelocationWriterInstruction {
            direction_range: direction.range(),
            host_insert_at: host.range().start,
            insertion_text: relocated,
            expected_post_snapshot_fingerprint: 1,
            canonical_payload_without_offset_fingerprint:
                canonical_direction_without_offset_fingerprint(direction_xml).unwrap(),
            original_direction_xml_fingerprint: stable_xml_snapshot_fingerprint(direction_xml),
            proposed_host_cursor: 0,
            source_effective_cursor: 4,
            proposed_offset: 4,
            group_end_cursor: 12,
        }
    }


    #[test]
    fn direction_reads_the_registered_checkpoint_when_the_source_file_differs() {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let checkpoint_path = std::env::temp_dir().join(format!("boundary-checkpoint-{stamp}.render.musicxml"));
        let file_path = std::env::temp_dir().join(format!("boundary-source-{stamp}.musicxml"));
        let checkpoint = fixture("end", false);
        let file = fixture("continue", false);
        let (result, receipt) = super::super::orchestrator_artifact_sink_authority::write_external_feedback_closure_checkpoint_authorized(&checkpoint_path, &checkpoint);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe); result.unwrap();
        for content in [&checkpoint, &file] {
            std::fs::write(&file_path, content).unwrap();
            let historical = checkpoint.clone();
            assert_eq!(historical, checkpoint);
            let (post, receipt) = compose_authorized_direction_delta_onto_sink(&historical, &instruction(&file));
            let post = post.unwrap();
            let receipt = receipt.unwrap();
            assert_eq!(receipt.realize_post_document(&checkpoint).as_deref(), Some(post.as_str()));
            assert_eq!(note_beam_fingerprint(&post), note_beam_fingerprint(&checkpoint));
            if content != &checkpoint {
                assert!(receipt.realize_post_document(content).is_none());
                assert_ne!(note_beam_fingerprint(content), note_beam_fingerprint(&post));
            }
        }
        std::fs::remove_file(checkpoint_path).unwrap();
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn cursor_then_direction_keep_two_disjoint_legacy_events() {
        let (pre, file_after_cursor, cursor_receipt) = super::super::writer_entry_boundary::tests::executed_cursor_event();
        let doc = Document::parse(&pre).unwrap();
        let direction = doc.descendants().find(|node| node.has_tag_name("direction")).unwrap();
        let measure = direction.parent().unwrap();
        let records = direct_child_cursor_records(measure);
        let index = records.iter().position(|(node, _, _)| *node == direction).unwrap();
        let (host, host_before, _) = records[index - 1];
        let (_, direction_before, _) = records[index];
        let direction_xml = &pre[direction.range()];
        let insertion = direction_xml.replace("<staff>1</staff>", &format!("<offset>{}</offset><staff>1</staff>", direction_before - host_before));
        let plan = RendererDirectionRelocationWriterInstruction {
            direction_range: direction.range(), host_insert_at: host.range().start,
            insertion_text: insertion,
            expected_post_snapshot_fingerprint: 1,
            canonical_payload_without_offset_fingerprint: canonical_direction_without_offset_fingerprint(direction_xml).unwrap(),
            original_direction_xml_fingerprint: stable_xml_snapshot_fingerprint(direction_xml),
            proposed_host_cursor: host_before, source_effective_cursor: direction_before,
            proposed_offset: direction_before - host_before, group_end_cursor: 384,
        };
        let (post, receipt) = compose_authorized_direction_delta_onto_sink(&pre, &plan);
        let post = post.unwrap();
        let receipt = receipt.unwrap();
        assert_eq!(cursor_receipt.realize_post_document(&pre).as_deref(), Some(file_after_cursor.as_str()));
        assert_eq!(receipt.realize_post_document(&pre).as_deref(), Some(post.as_str()));
        assert!(receipt.realize_post_document(&file_after_cursor).is_none());
        assert!(cursor_receipt.minted_by::<super::super::CursorNeutralRenderExactMint>());
        assert!(receipt.minted_by::<DirectionRelocationExactMint>());
        assert_ne!(post, file_after_cursor);
        assert_eq!(note_beam_fingerprint(&pre), note_beam_fingerprint(&post));
    }

    #[test]
    fn direction_receipt_is_bound_to_the_historical_sink_only() {
        for diverges in [false, true] {
            let file = fixture("continue", false);
            let checkpoint = fixture(if diverges { "end" } else { "continue" }, false);
            let plan = instruction(&file);
            let (post, receipt) = compose_authorized_direction_delta_onto_sink(&checkpoint, &plan);
            let post = post.unwrap();
            let receipt = receipt.unwrap();
            assert!(receipt.minted_by::<DirectionRelocationExactMint>());
            assert!(!receipt.minted_by::<super::super::CursorNeutralRenderExactMint>());
            assert_eq!(receipt.realize_post_document(&checkpoint).as_deref(), Some(post.as_str()));
            assert_eq!(note_beam_fingerprint(&post), note_beam_fingerprint(&checkpoint));
            let doc = Document::parse(&checkpoint).unwrap();
            let direction = doc.descendants().find(|node| node.has_tag_name("direction")).unwrap().range();
            let host = doc.descendants().find(|node| node.has_tag_name("note")).unwrap().range().start;
            let insertion = plan.insertion_text.trim();
            let legacy = apply_exact_bound_edits(&checkpoint, direction, host, &format!("{insertion}\n")).unwrap();
            assert_eq!(post, legacy);
            if diverges { assert!(receipt.realize_post_document(&file).is_none()); }
            let (second, second_receipt) = compose_authorized_direction_delta_onto_sink(&post, &plan);
            assert!(second.is_none());
            assert!(second_receipt.is_none());
        }
        let no_direction = fixture("continue", false).replace(
            "<direction><direction-type><dynamics><f/></dynamics></direction-type><staff>2</staff></direction>", "");
        let plan = instruction(&fixture("continue", false));
        let (none, receipt) = compose_authorized_direction_delta_onto_sink(&no_direction, &plan);
        assert!(none.is_none());
        assert!(receipt.is_none());
    }

    #[test]
    fn composition_preserves_sink_beams_and_moves_only_direction() {
        let pre = fixture("continue", false);
        let sink = fixture("end", false);
        let plan = instruction(&pre);
        let (after, receipt) = compose_authorized_direction_delta_onto_sink(&sink, &plan);
        let after = after.unwrap();
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(note_beam_fingerprint(&sink), note_beam_fingerprint(&after));
        assert!(after.contains("<offset>4</offset><staff>2</staff>"));
        assert!(after.find("<direction>").unwrap() < after.find("<note>").unwrap());
    }

    #[test]
    fn ambiguous_sink_direction_blocks_fail_closed() {
        let pre = fixture("continue", false);
        let sink = fixture("end", true);
        let plan = instruction(&pre);
        let (result, receipt) = compose_authorized_direction_delta_onto_sink(&sink, &plan);
        assert!(result.is_none()); assert!(receipt.is_none());
    }
}
