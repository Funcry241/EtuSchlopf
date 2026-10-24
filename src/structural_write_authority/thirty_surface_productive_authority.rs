//! Productive authority cutover for the thirty surfaces pinned in v0.10.1226.
//!
//! Each public facade represents one semantic effect. The move-only capability
//! types are private, source-bound, range-bound where applicable, and consumed
//! exactly once by their writer. No facade performs musical selection.

use super::bound_tie_fragment_insertions;
use crate::master_state_identity::BlockTransformationReceipt;
use super::authority_primitives::{
    fingerprint as source_fingerprint,
    xml_attr_range,
};
use crate::master_state_identity::{
    AdditiveTransformationReceipt, ExactEdit, ExactTransformationReceipt,
};
use anyhow::{bail, Context, Result};
use std::path::Path;

#[derive(Debug)]
struct ExactSourceBinding {
    bytes: String,
    fingerprint: u64,
}

impl ExactSourceBinding {
    fn issue(source: &str) -> Self {
        Self {
            bytes: source.to_string(),
            fingerprint: source_fingerprint(source),
        }
    }

    fn verify(&self, source: &str) -> bool {
        self.bytes == source && self.fingerprint == source_fingerprint(source)
    }

    fn into_verified_string(self, source: &str) -> Option<String> {
        self.verify(source).then_some(self.bytes)
    }
}


fn is_xml_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':'))
}

fn exact_note_fragment(fragment: &str) -> bool {
    fragment.trim_start().starts_with("<note") && fragment.trim_end().ends_with("</note>")
}

fn exact_direction_fragment(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<direction")
        && trimmed.ends_with("</direction>")
        && !trimmed.contains("<measure")
        && trimmed.matches("</direction>").count() == 1
}

fn exact_direction_insertion_payload(payload: &str) -> bool {
    let trimmed = payload.trim();
    if exact_direction_fragment(trimmed) {
        return true;
    }
    let Some(after_comment_open) = trimmed.strip_prefix("<!--") else {
        return false;
    };
    let Some(comment_end_relative) = after_comment_open.find("-->") else {
        return false;
    };
    let comment_body = &after_comment_open[..comment_end_relative];
    if comment_body.contains("--") {
        return false;
    }
    let direction = after_comment_open[comment_end_relative + "-->".len()..].trim_start();
    exact_direction_fragment(direction)
}

#[cfg(test)]
fn exact_pitch_fragment(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<pitch")
        && trimmed.ends_with("</pitch>")
        && trimmed.contains("<step>")
        && trimmed.contains("<octave>")
}

fn child_block_range(text: &str, tag: &str) -> Option<(usize, usize)> {
    let open_plain = format!("<{tag}>");
    let open_attr = format!("<{tag} ");
    let open_start = text.find(&open_plain).or_else(|| text.find(&open_attr))?;
    let open_end = text[open_start..].find('>')? + open_start + 1;
    let close = format!("</{tag}>");
    let close_end = text[open_end..].find(&close)? + open_end + close.len();
    Some((open_start, close_end))
}

fn remove_child_blocks(mut text: String, tag: &str) -> String {
    while let Some((start, end)) = child_block_range(&text, tag) {
        text.replace_range(start..end, "");
    }
    for empty in [format!("<{tag} />"), format!("<{tag}/>")] {
        text = text.replace(&empty, "");
    }
    text
}

fn child_value_range(text: &str, tag: &str) -> Option<(usize, usize)> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)? + open.len();
    let end = text[start..].find(&close)? + start;
    Some((start, end))
}

fn extract_i64(text: &str, tag: &str) -> Option<i64> {
    let (start, end) = child_value_range(text, tag)?;
    text[start..end].trim().parse::<i64>().ok()
}

fn arrangement_duration_type(duration: i64) -> Option<(&'static str, usize)> {
    match duration {
        384 => Some(("whole", 0)),
        288 => Some(("half", 1)),
        192 => Some(("half", 0)),
        144 => Some(("quarter", 1)),
        96 => Some(("quarter", 0)),
        72 => Some(("eighth", 1)),
        48 => Some(("eighth", 0)),
        36 => Some(("16th", 1)),
        24 => Some(("16th", 0)),
        18 => Some(("32nd", 1)),
        12 => Some(("32nd", 0)),
        9 => Some(("64th", 1)),
        6 => Some(("64th", 0)),
        _ => None,
    }
}

fn render_duration_type(duration: i32) -> Option<(&'static str, usize)> {
    match duration {
        384 => Some(("whole", 0)),
        288 => Some(("half", 1)),
        192 => Some(("half", 0)),
        144 => Some(("quarter", 1)),
        96 => Some(("quarter", 0)),
        72 => Some(("eighth", 1)),
        48 => Some(("eighth", 0)),
        36 => Some(("16th", 1)),
        24 => Some(("16th", 0)),
        18 => Some(("32nd", 1)),
        12 => Some(("32nd", 0)),
        _ => None,
    }
}

fn insert_before_note_end(mut text: String, insertion: &str) -> String {
    let Some(close_pos) = text.rfind("</note>") else {
        return text;
    };
    if close_pos > 0 && !text[..close_pos].ends_with('\n') {
        text.insert(close_pos, '\n');
        text.insert_str(close_pos + 1, insertion);
    } else {
        text.insert_str(close_pos, insertion);
    }
    text
}

fn remove_note_attribute_from_opening_tag(text: String, attr: &str) -> String {
    let Some(open_end) = text.find('>') else {
        return text;
    };
    let mut opening = text[..open_end].to_string();
    for quote in ['"', '\''] {
        loop {
            let pattern = format!(" {attr}={quote}");
            let Some(start) = opening.find(&pattern) else {
                break;
            };
            let value_start = start + pattern.len();
            let Some(end_relative) = opening[value_start..].find(quote) else {
                break;
            };
            opening.replace_range(start..value_start + end_relative + 1, "");
        }
    }
    format!("{}{}", opening, &text[open_end..])
}

fn replace_xml_tag_text(mut fragment: String, tag: &str, value: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let Some(start) = fragment.find(&open) else {
        return fragment;
    };
    let value_start = start + open.len();
    let Some(end_relative) = fragment[value_start..].find(&close) else {
        return fragment;
    };
    fragment.replace_range(value_start..value_start + end_relative, value);
    fragment
}

fn remove_tag_blocks_all(mut fragment: String, tag: &str) -> String {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    while let Some(start) = fragment.find(&open) {
        let Some(open_end_relative) = fragment[start..].find('>') else {
            break;
        };
        let open_end = start + open_end_relative + 1;
        if fragment[start..open_end].ends_with("/>") {
            fragment.replace_range(start..open_end, "");
            continue;
        }
        let Some(close_relative) = fragment[open_end..].find(&close) else {
            break;
        };
        let close_end = open_end + close_relative + close.len();
        fragment.replace_range(start..close_end, "");
    }
    fragment
}

fn remove_simple_empty_tags(mut fragment: String, tag: &str) -> String {
    for pattern in [
        format!("<{tag} />"),
        format!("<{tag}/>"),
        format!("<{tag}></{tag}>"),
    ] {
        fragment = fragment.replace(&pattern, "");
    }
    fragment
}


#[cfg(test)]
fn strip_beam_tags(fragment: &str) -> String {
    let mut out = String::new();
    let mut scan = 0usize;
    while let Some(relative) = fragment[scan..].find("<beam") {
        let start = scan + relative;
        out.push_str(&fragment[scan..start]);
        let Some(close_relative) = fragment[start..].find("</beam>") else {
            out.push_str(&fragment[start..]);
            return out;
        };
        scan = start + close_relative + "</beam>".len();
    }
    out.push_str(&fragment[scan..]);
    out
}

#[derive(Debug, Clone, Copy)]
enum ChildInsertionStyle {
    Arrangement,
    AtomicMove,
}

#[derive(Debug)]
struct ChildTextValueCapability {
    source: ExactSourceBinding,
    tag: String,
    value: String,
    style: ChildInsertionStyle,
}

impl ChildTextValueCapability {
    fn issue(source: &str, tag: &str, value: &str, style: ChildInsertionStyle) -> Option<Self> {
        if !exact_note_fragment(source) || !is_xml_name(tag) {
            return None;
        }
        Some(Self {
            source: ExactSourceBinding::issue(source),
            tag: tag.to_string(),
            value: value.to_string(),
            style,
        })
    }
}

fn child_text_value_writer(source: &str, token: ChildTextValueCapability) -> Option<String> {
    let mut out = token.source.into_verified_string(source)?;
    let close = format!("</{}>", token.tag);
    let value_range = match token.style {
        ChildInsertionStyle::Arrangement => {
            let open = format!("<{}>", token.tag);
            out.find(&open).and_then(|start| {
                let value_start = start + open.len();
                out[value_start..]
                    .find(&close)
                    .map(|end_relative| (value_start, value_start + end_relative))
            })
        }
        ChildInsertionStyle::AtomicMove => {
            let open = format!("<{}", token.tag);
            out.find(&open).and_then(|start| {
                let value_start = start + out[start..].find('>')? + 1;
                let value_end = value_start + out[value_start..].find(&close)?;
                Some((value_start, value_end))
            })
        }
    };
    if let Some((value_start, value_end)) = value_range {
        out.replace_range(value_start..value_end, &token.value);
        return Some(out);
    }
    match token.style {
        ChildInsertionStyle::Arrangement => {
            let insertion = format!(
                "        <{}>{}</{}>\n",
                token.tag, token.value, token.tag
            );
            Some(insert_before_note_end(out, &insertion))
        }
        ChildInsertionStyle::AtomicMove => {
            let insert_at = out.rfind("</note>")?;
            out.insert_str(
                insert_at,
                &format!("\n        <{}>{}</{}>", token.tag, token.value, token.tag),
            );
            Some(out)
        }
    }
}

pub fn ensure_arrangement_child_text_value_authorized(
    source: &str,
    tag: &str,
    value: &str,
) -> String {
    ChildTextValueCapability::issue(source, tag, value, ChildInsertionStyle::Arrangement)
        .and_then(|token| child_text_value_writer(source, token))
        .unwrap_or_else(|| source.to_string())
}

pub fn ensure_atomic_move_child_text_value_authorized(
    source: &str,
    tag: &str,
    value: &str,
) -> String {
    ChildTextValueCapability::issue(source, tag, value, ChildInsertionStyle::AtomicMove)
        .and_then(|token| child_text_value_writer(source, token))
        .unwrap_or_else(|| source.to_string())
}

#[derive(Debug)]
struct NoteStartAttributeCapability {
    source: ExactSourceBinding,
    attr: String,
}

impl NoteStartAttributeCapability {
    fn issue(source: &str, attr: &str) -> Option<Self> {
        if !exact_note_fragment(source) || !is_xml_name(attr) {
            return None;
        }
        Some(Self {
            source: ExactSourceBinding::issue(source),
            attr: attr.to_string(),
        })
    }
}

fn note_start_attribute_writer(
    source: &str,
    token: NoteStartAttributeCapability,
) -> Option<String> {
    let out = token.source.into_verified_string(source)?;
    Some(remove_note_attribute_from_opening_tag(out, &token.attr))
}

pub fn remove_note_start_attribute_authorized(source: &str, attr: &str) -> String {
    NoteStartAttributeCapability::issue(source, attr)
        .and_then(|token| note_start_attribute_writer(source, token))
        .unwrap_or_else(|| source.to_string())
}

#[derive(Debug)]
struct DurationTypeDotsCapability {
    source: ExactSourceBinding,
    note_type: &'static str,
    dots: usize,
}

fn duration_type_dots_writer(source: &str, token: DurationTypeDotsCapability) -> Option<String> {
    let mut out = token.source.into_verified_string(source)?;
    out = remove_child_blocks(out, "type");
    out = remove_child_blocks(out, "dot");
    let insert_at = [
        "<accidental",
        "<time-modification",
        "<stem",
        "<notehead",
        "<staff",
        "<beam",
        "<notations",
        "<lyric",
        "<play",
        "<listen",
    ]
    .into_iter()
    .filter_map(|marker| out.find(marker))
    .min()
    .or_else(|| out.rfind("</note>"))?;
    let insert_at = out[..insert_at]
        .rfind('\n')
        .map_or(insert_at, |line_break| line_break + 1);
    let mut type_block = format!("        <type>{}</type>\n", token.note_type);
    for _ in 0..token.dots {
        type_block.push_str("        <dot/>\n");
    }
    out.insert_str(insert_at, &type_block);
    Some(out)
}

pub fn sync_arrangement_note_type_and_dots_authorized(source: &str) -> String {
    let Some(duration) = extract_i64(source, "duration") else {
        return source.to_string();
    };
    let Some((note_type, dots)) = arrangement_duration_type(duration) else {
        return source.to_string();
    };
    let token = DurationTypeDotsCapability {
        source: ExactSourceBinding::issue(source),
        note_type,
        dots,
    };
    duration_type_dots_writer(source, token).unwrap_or_else(|| source.to_string())
}

pub fn sync_atomic_move_note_type_and_dots_authorized(source: &str) -> String {
    let Some(duration) = extract_i64(source, "duration") else {
        return source.to_string();
    };
    if duration <= 0 {
        return source.to_string();
    }
    let (note_type, dots) = crate::midi_seed::type_and_dots_for_duration(duration);
    let token = DurationTypeDotsCapability {
        source: ExactSourceBinding::issue(source),
        note_type,
        dots,
    };
    duration_type_dots_writer(source, token).unwrap_or_else(|| source.to_string())
}

#[derive(Debug)]
struct RhythmLedgerRestifyCapability {
    source: ExactSourceBinding,
}

pub fn restify_rhythm_ledger_note_authorized(source: &str) -> String {
    let token = RhythmLedgerRestifyCapability {
        source: ExactSourceBinding::issue(source),
    };
    let Some(mut out) = token.source.into_verified_string(source) else {
        return source.to_string();
    };
    if let Some((start, end)) = child_block_range(&out, "pitch") {
        out.replace_range(start..end, "<rest/>");
    }
    for tag in ["stem", "beam", "notations", "chord"] {
        out = remove_child_blocks(out, tag);
    }
    out = sync_arrangement_note_type_and_dots_authorized(&out);
    out = remove_note_start_attribute_authorized(&out, "print-object");
    remove_note_start_attribute_authorized(&out, "print-spacing")
}

#[derive(Debug)]
struct AtomicMoveRestifyCapability {
    source: ExactSourceBinding,
}

pub fn restify_atomic_move_note_authorized(source: &str) -> String {
    let token = AtomicMoveRestifyCapability {
        source: ExactSourceBinding::issue(source),
    };
    let Some(mut out) = token.source.into_verified_string(source) else {
        return source.to_string();
    };
    if let Some((start, end)) = child_block_range(&out, "pitch") {
        out.replace_range(start..end, "<rest/>");
    }
    for tag in ["stem", "beam", "notations", "chord"] {
        out = remove_child_blocks(out, tag);
    }
    out = sync_atomic_move_note_type_and_dots_authorized(&out);
    out = remove_note_start_attribute_authorized(&out, "print-object");
    remove_note_start_attribute_authorized(&out, "print-spacing")
}

#[derive(Debug)]
struct TieMarkupCapability {
    source: ExactSourceBinding,
    start: bool,
    stop: bool,
}

pub fn add_tie_markup_authorized(source: &str, start: bool, stop: bool) -> (String, Option<AdditiveTransformationReceipt>) {
    let geometry = bound_tie_fragment_insertions(source, start, stop);
    let output = (|| {
    let token = TieMarkupCapability {
        source: ExactSourceBinding::issue(source),
        start,
        stop,
    };
    let Some(mut out) = token.source.into_verified_string(source) else {
        return source.to_string();
    };
    if let Some(position) = out.find("</duration>") {
        let insert_at = position + "</duration>".len();
        let mut ties = String::new();
        if token.stop {
            ties.push_str("\n        <tie type=\"stop\"/>");
        }
        if token.start {
            ties.push_str("\n        <tie type=\"start\"/>");
        }
        out.insert_str(insert_at, &ties);
    }
    let Some(close_pos) = out.rfind("</note>") else {
        return out;
    };
    let mut notations = String::from("        <notations>\n");
    if token.stop {
        notations.push_str("          <tied type=\"stop\"/>\n");
    }
    if token.start {
        notations.push_str("          <tied type=\"start\"/>\n");
    }
    notations.push_str("        </notations>\n");
    out.insert_str(close_pos, &notations);
    out
})();
    let receipt = crate::master_state_identity::AdditiveTransformationReceipt::mint_tie_fragment_receipt(super::TieFragmentAdditiveMint, source, &output, geometry);
    assert!(output == source || receipt.is_some(), "fragment owner changed bytes without additive receipt");
    (output, receipt)
}









#[derive(Debug)]
struct PitchedDurationStaccatoCapability {
    source: ExactSourceBinding,
    duration: i32,
    note_type: &'static str,
    dots: usize,
}

pub fn set_pitched_note_duration_type_add_staccato_authorized(
    source: &str,
    duration: i32,
) -> Option<String> {
    let (note_type, dots) = render_duration_type(duration)?;
    let token = PitchedDurationStaccatoCapability {
        source: ExactSourceBinding::issue(source),
        duration,
        note_type,
        dots,
    };
    let mut out = token.source.into_verified_string(source)?;
    out = replace_xml_tag_text(out, "duration", &token.duration.to_string());
    out = remove_xml_tag_block_all_authorized(out, "type");
    out = remove_simple_empty_tags(out, "dot");
    out = remove_xml_tag_block_all_authorized(out, "time-modification");
    out = remove_xml_tag_block_all_authorized(out, "beam");
    let mut type_block = format!("\n        <type>{}</type>", token.note_type);
    for _ in 0..token.dots {
        type_block.push_str("\n        <dot/>");
    }
    let insert_at = [
        "<accidental",
        "<time-modification",
        "<stem",
        "<notehead",
        "<staff",
        "<beam",
        "<notations",
        "<lyric",
        "<play",
        "<listen",
    ]
    .into_iter()
    .filter_map(|marker| out.find(marker))
    .min()
    .or_else(|| out.rfind("</note>"));
    if let Some(insert_at) = insert_at {
        out.insert_str(insert_at, &type_block);
    }
    if out.contains("<staccato") {
        return Some(out);
    }
    if let Some(position) = out.find("</notations>") {
        out.insert_str(
            position,
            "\n          <articulations><staccato/></articulations>",
        );
    } else if let Some(position) = out.find("</note>") {
        out.insert_str(
            position,
            "\n        <notations>\n          <articulations><staccato/></articulations>\n        </notations>",
        );
    }
    Some(out)
}

#[cfg(test)]
#[derive(Debug)]
struct PitchedDurationClearCapability {
    source: ExactSourceBinding,
    duration: i32,
    note_type: &'static str,
    dots: usize,
}

#[cfg(test)]
pub fn set_pitched_note_duration_type_clear_tie_and_beams_authorized(
    source: &str,
    duration: i32,
) -> Option<String> {
    let (note_type, dots) = render_duration_type(duration)?;
    let token = PitchedDurationClearCapability {
        source: ExactSourceBinding::issue(source),
        duration,
        note_type,
        dots,
    };
    let mut out = token.source.into_verified_string(source)?;
    out = replace_xml_tag_text(out, "duration", &token.duration.to_string());
    for tag in ["type", "time-modification", "tie", "notations", "beam"] {
        out = remove_xml_tag_block_all_authorized(out, tag);
    }
    out = remove_simple_empty_tags(out, "dot");
    let mut type_block = format!("\n        <type>{}</type>", token.note_type);
    for _ in 0..token.dots {
        type_block.push_str("\n        <dot/>");
    }
    let insert_at = [
        "<accidental",
        "<time-modification",
        "<stem",
        "<notehead",
        "<staff",
        "<beam",
        "<notations",
        "<lyric",
        "<play",
        "<listen",
    ]
    .into_iter()
    .filter_map(|marker| out.find(marker))
    .min()
    .or_else(|| out.rfind("</note>"));
    if let Some(insert_at) = insert_at {
        out.insert_str(insert_at, &type_block);
    }
    Some(out)
}

#[derive(Debug)]
struct RestDurationTypeCapability {
    source: ExactSourceBinding,
    duration: i32,
    rest_type: &'static str,
    dots: usize,
}

pub fn set_rest_note_duration_type_authorized(
    source: &str,
    duration: i32,
) -> (Option<String>, Option<BlockTransformationReceipt>) {
    
    
    let output = (|| {
    let (rest_type, dots) = render_duration_type(duration)?;
    let token = RestDurationTypeCapability {
        source: ExactSourceBinding::issue(source),
        duration,
        rest_type,
        dots,
    };
    let mut out = token.source.into_verified_string(source)?;
    out = replace_xml_tag_text(out, "duration", &token.duration.to_string());
    out = remove_xml_tag_block_all_authorized(out, "type");
    out = remove_simple_empty_tags(out, "dot");
    out = remove_xml_tag_block_all_authorized(out, "time-modification");
    let mut type_block = format!("\n        <type>{}</type>", token.rest_type);
    for _ in 0..token.dots {
        type_block.push_str("\n        <dot/>");
    }
    let insert_at = [
        "<accidental",
        "<time-modification",
        "<stem",
        "<notehead",
        "<staff",
        "<beam",
        "<notations",
        "<lyric",
        "<play",
        "<listen",
    ]
    .into_iter()
    .filter_map(|marker| out.find(marker))
    .min()
    .or_else(|| out.rfind("</note>"));
    if let Some(insert_at) = insert_at {
        out.insert_str(insert_at, &type_block);
    }
    Some(out)
})();
    
    let receipt = output.as_deref().filter(|post| *post != source).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow17BlockMint { _private: () }, source, post, Some((0, source.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

#[derive(Debug)]
struct ChildTagRemovalCapability {
    source: ExactSourceBinding,
    tag: String,
}

pub fn remove_xml_child_tags_authorized(source: &str, tag: &str) -> String {
    if !is_xml_name(tag) {
        return source.to_string();
    }
    let token = ChildTagRemovalCapability {
        source: ExactSourceBinding::issue(source),
        tag: tag.to_string(),
    };
    let Some(mut out) = token.source.into_verified_string(source) else {
        return source.to_string();
    };
    let open = format!("<{}>", token.tag);
    let close = format!("</{}>", token.tag);
    while let Some(start) = out.find(&open) {
        let Some(close_relative) = out[start..].find(&close) else {
            break;
        };
        out.replace_range(start..start + close_relative + close.len(), "");
    }
    out
}

#[derive(Debug)]
struct HiddenAttributeRemovalCapability {
    source: ExactSourceBinding,
    start: usize,
    end: usize,
    opening_fingerprint: u64,
}

impl HiddenAttributeRemovalCapability {
    fn issue(source: &str, start: usize, end: usize) -> Option<Self> {
        if start >= end || end > source.len() || !source.is_char_boundary(start) || !source.is_char_boundary(end) {
            return None;
        }
        let open_end_relative = source[start..end].find('>')?;
        let open_end = start + open_end_relative + 1;
        let opening = &source[start..open_end];
        if !opening.starts_with("<note") {
            return None;
        }
        Some(Self {
            source: ExactSourceBinding::issue(source),
            start,
            end: open_end,
            opening_fingerprint: source_fingerprint(opening),
        })
    }
}

pub fn remove_note_hidden_attrs_in_range_authorized(
    xml: &mut String,
    start: usize,
    end: usize,
) -> (bool, Option<ExactTransformationReceipt>) {
    let Some(token) = HiddenAttributeRemovalCapability::issue(xml, start, end) else {
        return (false, None);
    };
    if !token.source.verify(xml)
        || source_fingerprint(&xml[token.start..token.end]) != token.opening_fingerprint
    {
        return (false, None);
    }
    let mut tag = xml[token.start..token.end].to_string();
    let mut changed = false;
    for attr in ["print-object", "print-spacing"] {
        while let Some((attr_start, attr_end)) = xml_attr_range(&tag, attr) {
            let remove_start = if attr_start > 0 && tag.as_bytes().get(attr_start - 1) == Some(&b' ') {
                attr_start - 1
            } else {
                attr_start
            };
            tag.replace_range(remove_start..attr_end, "");
            changed = true;
        }
    }
    if !changed {
        return (false, None);
    }
    // EXACT receipt #11: the bound opening-tag rewrite, known at origin.
    let pre_xml = xml.clone();
    let bound_edit = ExactEdit::bound(token.start, token.end, tag.clone());
    xml.replace_range(token.start..token.end, &tag);
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        HiddenAttributeRemovalExactMint { _sealed: () },
        &pre_xml,
        xml,
        vec![bound_edit],
    );
    (true, receipt)
}

/// Sealed mint capability of `remove_note_hidden_attrs_in_range_authorized`
/// (EXACT #11).
pub(crate) struct HiddenAttributeRemovalExactMint {
    _sealed: (),
}

#[derive(Debug)]
struct NoteTagUpsertCapability {
    source: ExactSourceBinding,
    tag: String,
    value: String,
}

pub fn replace_or_insert_note_tag_authorized(
    block: &mut String,
    tag: &str,
    value: &str,
) -> bool {
    if !exact_note_fragment(block) || !is_xml_name(tag) {
        return false;
    }
    let token = NoteTagUpsertCapability {
        source: ExactSourceBinding::issue(block),
        tag: tag.to_string(),
        value: value.to_string(),
    };
    if !token.source.verify(block) {
        return false;
    }
    let open = format!("<{}>", token.tag);
    let close = format!("</{}>", token.tag);
    if let Some(start) = block.find(&open) {
        let content_start = start + open.len();
        if let Some(end_relative) = block[content_start..].find(&close) {
            let content_end = content_start + end_relative;
            if block[content_start..content_end] != token.value {
                block.replace_range(content_start..content_end, &token.value);
                return true;
            }
            return false;
        }
    }
    if let Some(position) = block.rfind("</note>") {
        block.insert_str(
            position,
            &format!("        <{}>{}</{}>\n", token.tag, token.value, token.tag),
        );
        return true;
    }
    false
}







// r1393: Row 8 has no productive caller any more (visibility exchange replaced it);
// the frozen owner stays as test fixture of the receipt matrix.
#[cfg(test)]
#[derive(Debug)]
struct HarmonicPitchSwapCapability {
    source: ExactSourceBinding,
    pitch_start: usize,
    pitch_end: usize,
    source_pitch_fingerprint: u64,
    replacement_pitch: String,
    replacement_pitch_fingerprint: u64,
}

#[cfg(test)]
impl HarmonicPitchSwapCapability {
    fn issue(source: &str, pitch_start: usize, pitch_end: usize, replacement_pitch: &str) -> Option<Self> {
        if pitch_start >= pitch_end
            || pitch_end > source.len()
            || !source.is_char_boundary(pitch_start)
            || !source.is_char_boundary(pitch_end)
            || !exact_pitch_fragment(&source[pitch_start..pitch_end])
            || !exact_pitch_fragment(replacement_pitch)
        {
            return None;
        }
        Some(Self {
            source: ExactSourceBinding::issue(source),
            pitch_start,
            pitch_end,
            source_pitch_fingerprint: source_fingerprint(&source[pitch_start..pitch_end]),
            replacement_pitch: replacement_pitch.to_string(),
            replacement_pitch_fingerprint: source_fingerprint(replacement_pitch),
        })
    }
}

#[cfg(test)]
pub fn apply_harmonic_pitch_swap_authorized(
    xml: &mut String,
    pitch_start: usize,
    pitch_end: usize,
    replacement_pitch: &str,
) -> (bool, Option<ExactTransformationReceipt>) {
    let Some(token) = HarmonicPitchSwapCapability::issue(
        xml,
        pitch_start,
        pitch_end,
        replacement_pitch,
    ) else {
        return (false, None);
    };
    if !token.source.verify(xml)
        || source_fingerprint(&xml[token.pitch_start..token.pitch_end])
            != token.source_pitch_fingerprint
        || source_fingerprint(&token.replacement_pitch) != token.replacement_pitch_fingerprint
    {
        return (false, None);
    }
    // EXACT receipt #8: the bound pitch-block replacement, known at origin.
    let pre_xml = xml.clone();
    let bound_edit = ExactEdit::bound(
        token.pitch_start,
        token.pitch_end,
        token.replacement_pitch.clone(),
    );
    xml.replace_range(
        token.pitch_start..token.pitch_end,
        &token.replacement_pitch,
    );
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        HarmonicPitchSwapExactMint { _sealed: () },
        &pre_xml,
        xml,
        vec![bound_edit],
    );
    (true, receipt)
}

/// Sealed mint capability of `apply_harmonic_pitch_swap_authorized` (EXACT #8).
#[cfg(test)]
pub(crate) struct HarmonicPitchSwapExactMint {
    _sealed: (),
}

#[derive(Debug)]
struct RestDisplayPitchCapability {
    source: ExactSourceBinding,
    step: String,
    octave: i32,
}

pub fn apply_standard_rest_display_pitch_authorized(
    source: &str,
    step: &str,
    octave: i32,
) -> String {
    if step.len() != 1 || !matches!(step.as_bytes()[0], b'A'..=b'G') {
        return source.to_string();
    }
    let token = RestDisplayPitchCapability {
        source: ExactSourceBinding::issue(source),
        step: step.to_string(),
        octave,
    };
    let Some(mut out) = token.source.into_verified_string(source) else {
        return source.to_string();
    };
    out = remove_xml_child_tags_authorized(&out, "display-step");
    out = remove_xml_child_tags_authorized(&out, "display-octave");
    if out.contains(" measure=\"yes\"") || out.contains(" measure='yes'") {
        out = out.replace(" measure=\"yes\"", "");
        out = out.replace(" measure='yes'", "");
    }
    let display_inner = format!(
        "\n          <display-step>{}</display-step>\n          <display-octave>{}</display-octave>\n        ",
        token.step, token.octave
    );
    let display_plain = format!("<rest>{display_inner}</rest>");
    if out.contains("<rest/>") {
        return out.replacen("<rest/>", &display_plain, 1);
    }
    if let Some(rest_pos) = out.find("<rest")
        && let Some(tag_close_relative) = out[rest_pos..].find('>') {
            let tag_close = rest_pos + tag_close_relative;
            let tag_text = &out[rest_pos..=tag_close];
            if tag_text.ends_with("/>") {
                let attrs = out[rest_pos + "<rest".len()..tag_close].trim_end_matches('/');
                let replacement = format!("<rest{attrs}>{display_inner}</rest>");
                out.replace_range(rest_pos..tag_close + 1, &replacement);
                return out;
            }
            out.insert_str(tag_close + 1, &display_inner);
        }
    out
}

#[cfg(test)]
#[derive(Debug)]
struct CrossStaffBeamCapability {
    source: ExactSourceBinding,
    beam_value: String,
}

#[cfg(test)]
pub fn add_cross_staff_voice_ledger_beams_to_note_authorized(
    source: &str,
    beam_value: &str,
) -> String {
    if !matches!(beam_value, "begin" | "continue" | "end") {
        return source.to_string();
    }
    let token = CrossStaffBeamCapability {
        source: ExactSourceBinding::issue(source),
        beam_value: beam_value.to_string(),
    };
    let Some(bound) = token.source.into_verified_string(source) else {
        return source.to_string();
    };
    let mut out = strip_beam_tags(&bound);
    let beam_xml = format!(
        "    <beam number=\"1\">{}</beam>\n    <beam number=\"2\">{}</beam>\n",
        token.beam_value, token.beam_value
    );
    let position = ["<notations", "<lyric", "<play", "<listen"]
        .into_iter()
        .filter_map(|marker| out.find(marker))
        .min()
        .or_else(|| out.rfind("</note>"));
    if let Some(position) = position {
        let insert_at = out[..position]
            .rfind('\n')
            .map_or(position, |line_break| line_break + 1);
        out.insert_str(insert_at, &beam_xml);
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralMeasureLocator {
    measure_number: usize,
}

impl StructuralMeasureLocator {
    pub fn measure_number(&self) -> usize {
        self.measure_number
    }
}

fn structural_measure_locators(source: &str) -> Vec<StructuralMeasureLocator> {
    let marker = "<measure number=\"";
    let mut measure_numbers = Vec::<usize>::new();
    let mut scan = 0usize;
    while let Some(relative) = source[scan..].find(marker) {
        let measure_start = scan + relative;
        let number_start = measure_start + marker.len();
        let Some(relative_end) = source[number_start..].find('"') else {
            break;
        };
        let number_end = number_start + relative_end;
        let Ok(measure_number) = source[number_start..number_end].trim().parse::<usize>() else {
            break;
        };
        let Some(close_relative) = source[measure_start..].find("</measure>") else {
            break;
        };
        measure_numbers.push(measure_number);
        scan = measure_start + close_relative + "</measure>".len();
    }
    measure_numbers.sort_unstable();
    measure_numbers.dedup();
    measure_numbers
        .into_iter()
        .map(|measure_number| StructuralMeasureLocator { measure_number })
        .collect()
}

pub fn first_structural_measure_locator(source: &str) -> Option<StructuralMeasureLocator> {
    structural_measure_locators(source).into_iter().next()
}

pub fn fractional_structural_measure_locator(
    source: &str,
    fraction: f64,
) -> Option<StructuralMeasureLocator> {
    if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
        return None;
    }
    let locators = structural_measure_locators(source);
    let idx = ((locators.len().saturating_sub(1)) as f64 * fraction).round() as usize;
    locators.get(idx.min(locators.len().saturating_sub(1))).cloned()
}

#[derive(Debug)]
struct DirectionInsertionCapability {
    source: ExactSourceBinding,
    measure_number: usize,
    direction: String,
    insert_at: usize,
}

impl DirectionInsertionCapability {
    fn issue(
        source: &str,
        locator: &StructuralMeasureLocator,
        direction: &str,
    ) -> Option<Self> {
        if !exact_direction_insertion_payload(direction) {
            return None;
        }
        let measure_number = locator.measure_number();
        let marker = format!("<measure number=\"{measure_number}\"");
        let measure_start = source.find(&marker)?;
        let measure_open_end_relative = source[measure_start..].find('>')?;
        let content_start = measure_start + measure_open_end_relative + 1;
        let close_relative = source[content_start..].find("</measure>")?;
        let measure_end = content_start + close_relative;
        let measure_text = &source[content_start..measure_end];
        if measure_text.contains(direction.trim()) {
            return None;
        }
        let insert_at = measure_text.find("</attributes>").map_or(content_start, |relative| {
            content_start + relative + "</attributes>".len()
        });
        Some(Self {
            source: ExactSourceBinding::issue(source),
            measure_number,
            direction: direction.to_string(),
            insert_at,
        })
    }
}

pub fn insert_direction_into_measure_authorized(
    xml: &mut String,
    locator: &StructuralMeasureLocator,
    direction: &str,
) -> (usize, Option<AdditiveTransformationReceipt>) {
    let Some(token) = DirectionInsertionCapability::issue(xml, locator, direction) else {
        return (0, None);
    };
    if !token.source.verify(xml)
        || !xml.contains(&format!("<measure number=\"{}\"", token.measure_number))
    {
        return (0, None);
    }
    // ADDITIVE receipt E036: one bound insertion of "\n" + direction at the
    // bound offset, known at origin.
    let bound_insertion = ExactEdit::bound(
        token.insert_at,
        token.insert_at,
        format!("\n{}", token.direction),
    );
    let pre_xml = xml.clone();
    xml.insert(token.insert_at, '\n');
    xml.insert_str(token.insert_at + 1, &token.direction);
    let receipt = AdditiveTransformationReceipt::mint_additive_receipt(
        DirectionInsertionAdditiveMint { _sealed: () },
        &pre_xml,
        xml,
        vec![bound_insertion],
    );
    crate::master_state_identity::capture_additive_origin(DirectionInsertionAdditiveMint { _sealed: () }, &receipt, &pre_xml, xml);
    (1, receipt)
}

/// Sealed mint capability of `insert_direction_into_measure_authorized`
/// (ADDITIVE E036).
pub(crate) struct DirectionInsertionAdditiveMint {
    _sealed: (),
}

#[derive(Debug)]
struct TagBlockRemovalCapability {
    source: ExactSourceBinding,
    tag: String,
}

pub fn remove_xml_tag_block_all_authorized(source: String, tag: &str) -> String {
    if !is_xml_name(tag) {
        return source;
    }
    let token = TagBlockRemovalCapability {
        source: ExactSourceBinding::issue(&source),
        tag: tag.to_string(),
    };
    if !token.source.verify(&source) {
        return source;
    }
    remove_tag_blocks_all(source, &token.tag)
}

#[derive(Debug)]
struct TrialFileBinding {
    path: std::path::PathBuf,
    source: ExactSourceBinding,
    candidate_xml: String,
    candidate_fingerprint: u64,
    source_existed: bool,
}

impl TrialFileBinding {
    fn issue(path: &Path, source_xml: &str, candidate_xml: &str) -> Result<Self> {
        if candidate_xml.is_empty() {
            bail!("authorized trial XML candidate is empty");
        }
        let source_existed = path.exists();
        if source_existed {
            let current = std::fs::read_to_string(path)
                .with_context(|| format!("cannot bind existing trial XML source {}", path.display()))?;
            if current != source_xml {
                bail!("trial XML source changed before capability issuance");
            }
        }
        Ok(Self {
            path: path.to_path_buf(),
            source: ExactSourceBinding::issue(source_xml),
            candidate_xml: candidate_xml.to_string(),
            candidate_fingerprint: source_fingerprint(candidate_xml),
            source_existed,
        })
    }

    fn verify_before_write(&self) -> Result<()> {
        if self.source_existed {
            let current = std::fs::read_to_string(&self.path).with_context(|| {
                format!("cannot re-read bound trial XML source {}", self.path.display())
            })?;
            if !self.source.verify(&current) {
                bail!("trial XML source changed after capability issuance");
            }
        }
        if source_fingerprint(&self.candidate_xml) != self.candidate_fingerprint {
            bail!("trial XML candidate changed after capability issuance");
        }
        Ok(())
    }

    fn verify_readback(&self) -> Result<()> {
        let readback = std::fs::read_to_string(&self.path)
            .with_context(|| format!("cannot read back authorized trial XML {}", self.path.display()))?;
        if readback != self.candidate_xml
            || source_fingerprint(&readback) != self.candidate_fingerprint
        {
            bail!("authorized trial XML readback differs from candidate");
        }
        Ok(())
    }
}

#[derive(Debug)]
struct MissingRestTrialSinkCapability {
    binding: TrialFileBinding,
}

impl MissingRestTrialSinkCapability {
    fn issue(path: &Path, source_xml: &str, candidate_xml: &str) -> Result<Self> {
        let binding = TrialFileBinding::issue(path, source_xml, candidate_xml)?;
        Ok(Self { binding })
    }
}

fn missing_rest_trial_sink_writer(token: MissingRestTrialSinkCapability) -> Result<()> {
    token.binding.verify_before_write()?;
    let path = &token.binding.path;
    let candidate_xml = &token.binding.candidate_xml;
    let writer_trace_before = crate::audit_only::writer_trace::capture_before(path);
    std::fs::write(path, candidate_xml).with_context(|| {
        format!(
            "cannot write authorized missing-rest trial XML {}",
            path.display()
        )
    })?;
    token.binding.verify_readback()?;
    crate::audit_only::writer_trace::record_after(
        "missing_rest_trial",
        path,
        writer_trace_before.as_deref(),
        candidate_xml,
    );
    Ok(())
}

pub fn write_missing_rest_trial_xml_authorized(
    path: &Path,
    source_xml: &str,
    candidate_xml: &str,
) -> Result<()> {
    missing_rest_trial_sink_writer(MissingRestTrialSinkCapability::issue(
        path,
        source_xml,
        candidate_xml,
    )?)
}



pub(crate) struct RenderRow17BlockMint { _private: () }

#[cfg(test)]
mod tests {
    use super::*;

    const NOTE: &str = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><staff>1</staff></note>";

    #[test]
    fn source_binding_rejects_changed_fragment() {
        let token = ChildTextValueCapability::issue(
            NOTE,
            "voice",
            "2",
            ChildInsertionStyle::Arrangement,
        )
        .expect("capability");
        assert!(child_text_value_writer("<note></note>", token).is_none());
    }

    #[test]
    fn exact_range_rejects_non_pitch_harmonic_swap() {
        let mut xml = NOTE.to_string();
        assert!(!apply_harmonic_pitch_swap_authorized(
            &mut xml,
            0,
            6,
            "<pitch><step>D</step><octave>4</octave></pitch>"
        )
        .0);
        assert_eq!(xml, NOTE);
    }

    #[test]
    fn harmonic_swap_changes_only_bound_pitch_bytes() {
        let mut xml = NOTE.to_string();
        let start = xml.find("<pitch>").unwrap();
        let end = xml.find("</pitch>").unwrap() + "</pitch>".len();
        let (swapped, receipt) = apply_harmonic_pitch_swap_authorized(
            &mut xml,
            start,
            end,
            "<pitch><step>D</step><octave>5</octave></pitch>",
        );
        assert!(swapped);
        assert!(receipt.is_some(), "a harmonic pitch swap must carry its EXACT receipt");
        assert!(xml.contains("<duration>96</duration><voice>1</voice><staff>1</staff>"));
    }

    #[test]
    fn second_direction_insertion_is_rejected() {
        let direction = "<direction placement=\"above\"></direction>";
        let mut xml = "<measure number=\"\
1\"><attributes></attributes></measure>".to_string();
        let locator = first_structural_measure_locator(&xml).unwrap();
        assert_eq!(insert_direction_into_measure_authorized(&mut xml, &locator, direction).0, 1);
        assert_eq!(insert_direction_into_measure_authorized(&mut xml, &locator, direction).0, 0);
    }

    #[test]
    fn audit_comment_prefixed_direction_preserves_legacy_insertion_contract() {
        let payload = "<!-- local_tempo_audit -->\n<direction placement=\"above\"></direction>";
        let mut xml = "<measure number=\"\
3\" width=\"100\"><attributes></attributes></measure>".to_string();
        let locator = first_structural_measure_locator(&xml).unwrap();
        assert_eq!(insert_direction_into_measure_authorized(&mut xml, &locator, payload).0, 1);
        assert!(xml.contains("local_tempo_audit"));
        assert_eq!(insert_direction_into_measure_authorized(&mut xml, &locator, payload).0, 0);
    }

    #[test]
    fn arbitrary_prefix_before_direction_is_rejected() {
        let payload = "raw-prefix<direction placement=\"above\"></direction>";
        let mut xml = "<measure number=\"\
1\"><attributes></attributes></measure>".to_string();
        let locator = first_structural_measure_locator(&xml).unwrap();
        assert_eq!(insert_direction_into_measure_authorized(&mut xml, &locator, payload).0, 0);
        assert!(!xml.contains("raw-prefix"));
    }

    #[test]
    fn duration_type_writer_preserves_musicxml_note_schema_order() {
        let source = "<note>\n        <pitch><step>C</step><octave>4</octave></pitch>\n        <duration>96</duration>\n        <voice>1</voice>\n        <type>eighth</type>\n        <stem>up</stem>\n        <staff>1</staff>\n      </note>";
        let out = sync_arrangement_note_type_and_dots_authorized(source);
        let duration = out.find("</duration>").expect("duration");
        let voice = out.find("<voice>").expect("voice");
        let note_type = out.find("<type>").expect("type");
        let stem = out.find("<stem>").expect("stem");
        assert!(duration < voice && voice < note_type && note_type < stem, "{out}");
    }

    #[test]
    fn duration_type_writer_keeps_ties_and_voice_before_type() {
        let source = "<note>\n        <pitch><step>C</step><octave>4</octave></pitch>\n        <duration>96</duration>\n        <tie type=\"start\"/>\n        <voice>1</voice>\n        <type>eighth</type>\n        <staff>1</staff>\n      </note>";
        let out = sync_arrangement_note_type_and_dots_authorized(source);
        let duration = out.find("</duration>").expect("duration");
        let tie = out.find("<tie ").expect("tie");
        let voice = out.find("<voice>").expect("voice");
        let note_type = out.find("<type>").expect("type");
        let staff = out.find("<staff>").expect("staff");
        assert!(
            duration < tie && tie < voice && voice < note_type && note_type < staff,
            "{out}"
        );
    }

    #[test]
    fn retired_clear_duration_witness_preserves_voice_and_removes_tie_beams() {
        let source = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>48</duration><tie type=\"start\"/><voice>1</voice><type>eighth</type><staff>1</staff><beam number=\"1\">begin</beam></note>";
        let output = set_pitched_note_duration_type_clear_tie_and_beams_authorized(source, 96).expect("bound duration witness");
        assert!(output.contains("<duration>96</duration>"));
        assert!(output.contains("<voice>1</voice>"));
        assert!(!output.contains("<tie") && !output.contains("<beam"));
        assert!(output.find("<voice>").unwrap() < output.find("<type>").unwrap());
    }

    #[test]
    fn duration_rewrite_variants_keep_voice_before_type() {
        let rest = "<note><rest/><duration>48</duration><voice>2</voice><staff>2</staff></note>";
        let rest_out = set_rest_note_duration_type_authorized(rest, 48).0.expect("rest rewrite");
        assert!(
            rest_out.find("<voice>").expect("voice")
                < rest_out.find("<type>").expect("type"),
            "{rest_out}"
        );

        let pitched = "<note><pitch><step>D</step><octave>5</octave></pitch><duration>48</duration><voice>1</voice><stem>up</stem><staff>1</staff><notations><articulations><accent/></articulations></notations></note>";
        let pitched_out = set_pitched_note_duration_type_add_staccato_authorized(pitched, 48)
            .expect("pitched rewrite");
        assert!(
            pitched_out.find("<voice>").expect("voice")
                < pitched_out.find("<type>").expect("type"),
            "{pitched_out}"
        );
    }

    #[test]
    fn cross_staff_beams_precede_notations() {
        let source = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff><notations><articulations><accent/></articulations></notations></note>";
        let out = add_cross_staff_voice_ledger_beams_to_note_authorized(source, "begin");
        assert!(
            out.find("<staff>").expect("staff")
                < out.find("<beam ").expect("beam")
                && out.find("<beam ").expect("beam")
                    < out.find("<notations>").expect("notations"),
            "{out}"
        );
    }

    #[test]
    fn capability_types_are_move_only_by_contract() {
        let source = include_str!("thirty_surface_productive_authority.rs");
        let production = source.split("#[cfg(test)]
mod tests {").next().unwrap_or(source);
        for name in [
            "ChildTextValueCapability",
            "NoteStartAttributeCapability",
            "RhythmLedgerRestifyCapability",
            "DurationTypeDotsCapability",
            "AtomicMoveRestifyCapability",
            "TieMarkupCapability",
            

            "PitchedDurationStaccatoCapability",
            "PitchedDurationClearCapability",
            "RestDurationTypeCapability",
            "ChildTagRemovalCapability",
            "HiddenAttributeRemovalCapability",
            "NoteTagUpsertCapability",

            "HarmonicPitchSwapCapability",
            "RestDisplayPitchCapability",
            "CrossStaffBeamCapability",
            "DirectionInsertionCapability",
            "TagBlockRemovalCapability",
            "MissingRestTrialSinkCapability",
        ] {
            assert!(production.contains(&format!("struct {name}")));
        }
        assert!(!production.contains("pub(crate) struct ChildTextValueCapability"));
        assert!(!production.contains("#[derive(Clone, Copy)]\nstruct"));
    }
}


#[cfg(test)]
mod fragment_receipt_parity_tests {
    use super::*;
    use crate::master_state_identity::{AdditiveInsertionClass, realize_bound_additive_fragment_receipt};


fn legacy_add_tie_markup_authorized(source: &str, start: bool, stop: bool) -> String {
    let token = TieMarkupCapability {
        source: ExactSourceBinding::issue(source),
        start,
        stop,
    };
    let Some(mut out) = token.source.into_verified_string(source) else {
        return source.to_string();
    };
    if let Some(position) = out.find("</duration>") {
        let insert_at = position + "</duration>".len();
        let mut ties = String::new();
        if token.stop {
            ties.push_str("\n        <tie type=\"stop\"/>");
        }
        if token.start {
            ties.push_str("\n        <tie type=\"start\"/>");
        }
        out.insert_str(insert_at, &ties);
    }
    let Some(close_pos) = out.rfind("</note>") else {
        return out;
    };
    let mut notations = String::from("        <notations>\n");
    if token.stop {
        notations.push_str("          <tied type=\"stop\"/>\n");
    }
    if token.start {
        notations.push_str("          <tied type=\"start\"/>\n");
    }
    notations.push_str("        </notations>\n");
    out.insert_str(close_pos, &notations);
    out
}
    fn sources() -> [&'static str; 14] {
        ["", "é💧", "<note>é</note>", "<note><duration>4</duration></note>",
         "<note><duration>4</duration>", "<note><notations></notations></note>",
         "<note><notations><articulations></articulations></notations></note>",
         "<note><notations><articulations><tenuto/></articulations></notations></note>",
         "<note><accent/></note>", "<note><staccato/></note>",
         "<note><strong-accent/></note>", "<note></note></duration>",
         "<note><duration>4</duration>é</note>", "</notations></duration></note>"]
    }
    
    
    #[test]
    fn tie_all_flags_and_malformed_partial_legacy_outcomes_match() {
        for source in sources() { for start in [false,true] { for stop in [false,true] {
            let expected = legacy_add_tie_markup_authorized(source, start, stop);
            let (post, receipt) = add_tie_markup_authorized(source, start, stop);
            assert_eq!(post, expected);
            assert_eq!(receipt.is_some(), post != source);
            if let Some(receipt) = receipt {
                let edits = bound_tie_fragment_insertions(source, start, stop);
                assert_eq!(realize_bound_additive_fragment_receipt(&receipt, AdditiveInsertionClass::TieFragment, source, &edits).as_deref(), Some(post.as_str()));
            }
        }}}
    }
}
