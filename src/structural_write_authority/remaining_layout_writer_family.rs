//! Batched central authority for the four remaining layout-core write surfaces.
//!
//! The public entry points accept structural plans, never project names or
//! free-form XML replacement text. Each effect has its own private, move-only
//! capability. The writers re-read the exact source bytes before applying the
//! authorized delta and never write files.

use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt};
use super::authority_primitives::{
    fingerprint as source_fingerprint,
    xml_attr_range,
};
use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerStaffClefMoveRequest {
    old_start: usize,
    old_end: usize,
    insert_at: usize,
    block: String,
}

impl LowerStaffClefMoveRequest {
    pub fn new(
        old_start: usize,
        old_end: usize,
        insert_at: usize,
        block: String,
    ) -> Self {
        Self {
            old_start,
            old_end,
            insert_at,
            block,
        }
    }
}

#[derive(Debug)]
struct AuthorizedLowerStaffClefMoveBatch {
    source_fingerprint: u64,
    moves: Vec<BoundClefMove>,
}

#[derive(Debug)]
struct BoundClefMove {
    old_start: usize,
    old_end: usize,
    insert_at: usize,
    block: String,
    block_fingerprint: u64,
}

impl AuthorizedLowerStaffClefMoveBatch {
    fn issue(source_xml: &str, mut requests: Vec<LowerStaffClefMoveRequest>) -> Result<Option<Self>> {
        if requests.is_empty() {
            return Ok(None);
        }
        requests.sort_by_key(|left| std::cmp::Reverse(left.old_start));

        let mut ascending_ranges: Vec<(usize, usize)> = requests
            .iter()
            .map(|request| (request.old_start, request.old_end))
            .collect();
        ascending_ranges.sort_unstable();
        if ascending_ranges
            .windows(2)
            .any(|window| window[0].1 > window[1].0)
        {
            bail!("overlapping lower-staff clef move sources are not authorizable");
        }

        let mut moves = Vec::with_capacity(requests.len());
        for request in requests {
            verify_lower_staff_clef_move_request(source_xml, &request)?;
            moves.push(BoundClefMove {
                old_start: request.old_start,
                old_end: request.old_end,
                insert_at: request.insert_at,
                block_fingerprint: source_fingerprint(&request.block),
                block: request.block,
            });
        }
        Ok(Some(Self {
            source_fingerprint: source_fingerprint(source_xml),
            moves,
        }))
    }
}

pub fn apply_lower_staff_clef_moves_authorized(
    xml: &mut String,
    requests: Vec<LowerStaffClefMoveRequest>,
) -> Result<usize> {
    let Some(token) = AuthorizedLowerStaffClefMoveBatch::issue(xml, requests)? else {
        return Ok(0);
    };
    authorized_lower_staff_clef_move_writer(xml, token)
}

fn authorized_lower_staff_clef_move_writer(
    xml: &mut String,
    token: AuthorizedLowerStaffClefMoveBatch,
) -> Result<usize> {
    if source_fingerprint(xml) != token.source_fingerprint {
        bail!("source changed after lower-staff clef move authorization");
    }
    for movement in &token.moves {
        verify_bound_clef_move(xml, movement)?;
    }
    let mut moves = token.moves;
    moves.sort_by_key(|movement| movement.insert_at);
    if moves.windows(2).any(|pair| pair[0].old_end > pair[1].insert_at) {
        bail!("overlapping lower-staff clef move envelopes are not authorizable");
    }
    let changed = moves.len();
    let mut candidate = String::with_capacity(xml.len());
    let mut cursor = 0;
    for movement in moves {
        candidate.push_str(&xml[cursor..movement.insert_at]);
        candidate.push_str(&movement.block);
        candidate.push_str(&xml[movement.insert_at..movement.old_start]);
        cursor = movement.old_end;
    }
    candidate.push_str(&xml[cursor..]);
    xml.replace_range(.., &candidate);
    Ok(changed)
}

fn verify_lower_staff_clef_move_request(
    source_xml: &str,
    request: &LowerStaffClefMoveRequest,
) -> Result<()> {
    verify_range(source_xml, request.old_start, request.old_end, "clef source")?;
    if request.insert_at >= request.old_start {
        bail!("lower-staff clef insertion must precede its source block");
    }
    if request.insert_at > source_xml.len() || !source_xml.is_char_boundary(request.insert_at) {
        bail!("lower-staff clef insertion offset is invalid");
    }
    if !source_xml[request.insert_at..].starts_with("<note") {
        bail!("lower-staff clef insertion is not bound to a note event boundary");
    }
    if &source_xml[request.old_start..request.old_end] != request.block.as_str() {
        bail!("lower-staff clef source bytes differ from the planned block");
    }
    if !is_safe_lower_staff_bass_clef_attributes(&request.block) {
        bail!("lower-staff clef move block is outside the allowed structural class");
    }
    verify_clef_move_envelope(source_xml, request.insert_at, request.old_start)?;
    Ok(())
}

fn verify_bound_clef_move(source_xml: &str, movement: &BoundClefMove) -> Result<()> {
    verify_range(source_xml, movement.old_start, movement.old_end, "bound clef source")?;
    if movement.insert_at >= movement.old_start
        || movement.insert_at > source_xml.len()
        || !source_xml.is_char_boundary(movement.insert_at)
        || !source_xml[movement.insert_at..].starts_with("<note")
    {
        bail!("bound lower-staff clef move lost its structural insertion boundary");
    }
    let current = &source_xml[movement.old_start..movement.old_end];
    if current != movement.block.as_str()
        || source_fingerprint(current) != movement.block_fingerprint
        || !is_safe_lower_staff_bass_clef_attributes(current)
    {
        bail!("bound lower-staff clef source changed before writer consumption");
    }
    verify_clef_move_envelope(source_xml, movement.insert_at, movement.old_start)?;
    Ok(())
}

fn verify_clef_move_envelope(source: &str, insert_at: usize, old_start: usize) -> Result<()> {
    // Both callers validate byte boundaries before this original-source slice.
    let envelope = &source[insert_at..old_start];
    if ["<attributes", "<clef", "<backup", "<forward", "</measure", "<measure"]
        .iter()
        .any(|barrier| envelope.contains(barrier))
    {
        bail!("lower-staff clef move crosses a clef, attributes, cursor or measure boundary");
    }
    Ok(())
}

fn is_safe_lower_staff_bass_clef_attributes(block: &str) -> bool {
    block.starts_with("<attributes")
        && block.ends_with("</attributes>")
        && block.contains("<clef")
        && (block.contains("number=\"2\"") || block.contains("number='2'"))
        && block.contains("<sign>F</sign>")
        && !block.contains("<divisions")
        && !block.contains("<key")
        && !block.contains("<time")
        && !block.contains("<staves")
        && !block.contains("<transpose")
}

#[derive(Debug)]
struct AuthorizedNoteVisibilityRewrite {
    source_fingerprint: u64,
    open_tag_start: usize,
    open_tag_end: usize,
    original_tag_fingerprint: u64,
    replacement_tag: String,
    replacement_tag_fingerprint: u64,
}

impl AuthorizedNoteVisibilityRewrite {
    fn issue(source_xml: &str, note_start: usize, note_end: usize) -> Result<Option<Self>> {
        verify_exact_note_range(source_xml, note_start, note_end)?;
        let open_end_relative = source_xml[note_start..note_end]
            .find('>')
            .ok_or_else(|| anyhow!("note opening tag is incomplete"))?;
        let open_tag_end = note_start + open_end_relative + 1;
        let original_tag = &source_xml[note_start..open_tag_end];
        let Some(replacement_tag) = rewrite_note_opening_tag_hidden(original_tag) else {
            return Ok(None);
        };
        Ok(Some(Self {
            source_fingerprint: source_fingerprint(source_xml),
            open_tag_start: note_start,
            open_tag_end,
            original_tag_fingerprint: source_fingerprint(original_tag),
            replacement_tag_fingerprint: source_fingerprint(&replacement_tag),
            replacement_tag,
        }))
    }
}

pub fn hide_note_print_object_authorized(
    xml: &mut String,
    note_start: usize,
    note_end: usize,
) -> (Result<bool>, Option<ExactTransformationReceipt>) {
    let token = match AuthorizedNoteVisibilityRewrite::issue(xml, note_start, note_end) {
        Ok(Some(token)) => token,
        Ok(None) => return (Ok(false), None),
        Err(error) => return (Err(error), None),
    };
    // EXACT receipt #5: the bound opening-tag rewrite, known at origin.
    let bound_edit = ExactEdit::bound(
        token.open_tag_start,
        token.open_tag_end,
        token.replacement_tag.clone(),
    );
    let pre_xml = xml.clone();
    if let Err(error) = authorized_note_visibility_writer(xml, token) {
        return (Err(error), None);
    }
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        NoteVisibilityExactMint { _sealed: () },
        &pre_xml,
        xml,
        vec![bound_edit],
    );
    (Ok(true), receipt)
}

/// Sealed mint capability of `hide_note_print_object_authorized` (EXACT #5).
pub(crate) struct NoteVisibilityExactMint {
    _sealed: (),
}

fn authorized_note_visibility_writer(
    xml: &mut String,
    token: AuthorizedNoteVisibilityRewrite,
) -> Result<()> {
    if source_fingerprint(xml) != token.source_fingerprint {
        bail!("source changed after note-visibility authorization");
    }
    verify_range(
        xml,
        token.open_tag_start,
        token.open_tag_end,
        "note opening tag",
    )?;
    let current_tag = &xml[token.open_tag_start..token.open_tag_end];
    if source_fingerprint(current_tag) != token.original_tag_fingerprint {
        bail!("note opening tag changed after authorization");
    }
    let expected = rewrite_note_opening_tag_hidden(current_tag)
        .ok_or_else(|| anyhow!("note became hidden before writer consumption"))?;
    if expected != token.replacement_tag
        || source_fingerprint(&expected) != token.replacement_tag_fingerprint
    {
        bail!("authorized note-visibility replacement no longer matches source");
    }
    xml.replace_range(
        token.open_tag_start..token.open_tag_end,
        &token.replacement_tag,
    );
    Ok(())
}

fn verify_exact_note_range(source_xml: &str, start: usize, end: usize) -> Result<()> {
    verify_range(source_xml, start, end, "note")?;
    let fragment = &source_xml[start..end];
    if !fragment.starts_with("<note") || !fragment.ends_with("</note>") {
        bail!("note-visibility request is not bound to an exact note element");
    }
    Ok(())
}

fn rewrite_note_opening_tag_hidden(tag: &str) -> Option<String> {
    if tag.contains("print-object=\"no\"") || tag.contains("print-object='no'") {
        return None;
    }
    let with_print_object = if let Some((start, end)) = xml_attr_range(tag, "print-object") {
        format!("{}print-object=\"no\"{}", &tag[..start], &tag[end..])
    } else {
        let insert_at = tag.len().checked_sub(1)?;
        format!(
            "{} print-object=\"no\"{}",
            &tag[..insert_at],
            &tag[insert_at..]
        )
    };
    if with_print_object.contains("print-spacing=") {
        return Some(with_print_object);
    }
    let insert_at = with_print_object.len().checked_sub(1)?;
    Some(format!(
        "{} print-spacing=\"no\"{}",
        &with_print_object[..insert_at],
        &with_print_object[insert_at..]
    ))
}

#[derive(Debug)]
struct AuthorizedAllowedRangeRemovalBatch {
    source_fingerprint: u64,
    ranges_descending: Vec<BoundDeletionRange>,
}

#[derive(Debug)]
struct BoundDeletionRange {
    start: usize,
    end: usize,
    fragment_fingerprint: u64,
}

impl AuthorizedAllowedRangeRemovalBatch {
    fn issue(source_xml: &str, ranges: &[(usize, usize)]) -> Result<Option<Self>> {
        let mut merged = merge_ranges(ranges.to_vec());
        if merged.is_empty() {
            return Ok(None);
        }
        for (start, end) in &merged {
            verify_range(source_xml, *start, *end, "allowed XML deletion")?;
            let fragment = &source_xml[*start..*end];
            if !is_allowed_structural_deletion_fragment(fragment) {
                bail!("requested XML deletion is outside the structural allowlist");
            }
        }
        merged.sort_by_key(|left| std::cmp::Reverse(left.0));
        let ranges_descending = merged
            .into_iter()
            .map(|(start, end)| BoundDeletionRange {
                start,
                end,
                fragment_fingerprint: source_fingerprint(&source_xml[start..end]),
            })
            .collect();
        Ok(Some(Self {
            source_fingerprint: source_fingerprint(source_xml),
            ranges_descending,
        }))
    }
}

pub fn remove_allowed_xml_ranges_authorized(
    xml: &mut String,
    ranges: &[(usize, usize)],
) -> (Result<usize>, Option<ExactTransformationReceipt>) {
    let token = match AuthorizedAllowedRangeRemovalBatch::issue(xml, ranges) {
        Ok(Some(token)) => token,
        Ok(None) => return (Ok(0), None),
        Err(error) => return (Err(error), None),
    };
    let changed = token.ranges_descending.len();
    // EXACT receipt #6: every bound deletion range, known at origin.
    let bound_edits = token
        .ranges_descending
        .iter()
        .map(|range| ExactEdit::bound(range.start, range.end, String::new()))
        .collect::<Vec<_>>();
    let pre_xml = xml.clone();
    if let Err(error) = authorized_allowed_range_removal_writer(xml, token) {
        return (Err(error), None);
    }
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        AllowedRangeRemovalExactMint { _sealed: () },
        &pre_xml,
        xml,
        bound_edits,
    );
    (Ok(changed), receipt)
}

/// Sealed mint capability of `remove_allowed_xml_ranges_authorized` (EXACT #6).
pub(crate) struct AllowedRangeRemovalExactMint {
    _sealed: (),
}

fn authorized_allowed_range_removal_writer(
    xml: &mut String,
    token: AuthorizedAllowedRangeRemovalBatch,
) -> Result<()> {
    if source_fingerprint(xml) != token.source_fingerprint {
        bail!("source changed after structural range-removal authorization");
    }
    for range in &token.ranges_descending {
        verify_range(xml, range.start, range.end, "bound structural deletion")?;
        let fragment = &xml[range.start..range.end];
        if source_fingerprint(fragment) != range.fragment_fingerprint
            || !is_allowed_structural_deletion_fragment(fragment)
        {
            bail!("bound structural deletion changed before writer consumption");
        }
    }
    for range in token.ranges_descending {
        xml.replace_range(range.start..range.end, "");
    }
    Ok(())
}

fn is_allowed_structural_deletion_fragment(fragment: &str) -> bool {
    let mut cursor = 0usize;
    let bytes = fragment.as_bytes();
    let mut consumed = false;
    while cursor < bytes.len() {
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor == bytes.len() {
            break;
        }
        let rest = &fragment[cursor..];
        let unit_len = if rest.starts_with("<attributes") {
            let Some(relative_end) = rest.find("</attributes>") else {
                return false;
            };
            let end = relative_end + "</attributes>".len();
            let block = &rest[..end];
            if !is_safe_lower_staff_clef_attributes_for_removal(block) {
                return false;
            }
            end
        } else if rest.starts_with("<notations>") {
            let Some(relative_end) = rest.find("</notations>") else {
                return false;
            };
            let end = relative_end + "</notations>".len();
            let block = &rest[..end];
            let inner = &block["<notations>".len()..relative_end];
            if !inner.trim().is_empty() {
                return false;
            }
            end
        } else if starts_with_xml_tag(rest, "slur")
            || starts_with_xml_tag(rest, "tie")
            || starts_with_xml_tag(rest, "tied")
        {
            let Some(relative_end) = rest.find('>') else {
                return false;
            };
            relative_end + 1
        } else {
            return false;
        };
        cursor += unit_len;
        consumed = true;
    }
    consumed
}


fn starts_with_xml_tag(text: &str, name: &str) -> bool {
    let prefix = format!("<{name}");
    if !text.starts_with(&prefix) {
        return false;
    }
    matches!(
        text.as_bytes().get(prefix.len()),
        Some(b' ' | b'\t' | b'\n' | b'\r' | b'/' | b'>')
    )
}

fn is_safe_lower_staff_clef_attributes_for_removal(block: &str) -> bool {
    block.starts_with("<attributes")
        && block.ends_with("</attributes>")
        && block.contains("<clef")
        && (block.contains("number=\"2\"") || block.contains("number='2'"))
        && (block.contains("<sign>F</sign>") || block.contains("<sign>G</sign>"))
        && !block.contains("<divisions")
        && !block.contains("<key")
        && !block.contains("<time")
        && !block.contains("<staves")
        && !block.contains("<transpose")
}

fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ranges.retain(|(start, end)| start < end);
    ranges.sort_unstable();
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (start, end) in ranges {
        if let Some(last) = out.last_mut()
            && start <= last.1 {
                last.1 = last.1.max(end);
                continue;
            }
        out.push((start, end));
    }
    out
}


fn verify_range(source: &str, start: usize, end: usize, label: &str) -> Result<()> {
    if start >= end
        || end > source.len()
        || !source.is_char_boundary(start)
        || !source.is_char_boundary(end)
    {
        bail!("{label} range is invalid");
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::{
        apply_lower_staff_clef_moves_authorized,
        authorized_allowed_range_removal_writer,
        authorized_note_visibility_writer,
        hide_note_print_object_authorized,
        remove_allowed_xml_ranges_authorized,
        AuthorizedAllowedRangeRemovalBatch,
        AuthorizedNoteVisibilityRewrite,
        LowerStaffClefMoveRequest,
    };

    #[test]
    fn note_visibility_matches_legacy_opening_tag_bytes() {
        let mut xml = "<measure><note id='x'><pitch/></note></measure>".to_string();
        let start = xml.find("<note").unwrap();
        let end = xml.find("</note>").unwrap() + "</note>".len();
        let (hidden, receipt) = hide_note_print_object_authorized(&mut xml, start, end);
        assert!(hidden.unwrap());
        assert!(receipt.is_some(), "a confined hide must carry its EXACT receipt");
        assert!(xml.contains("<note id='x' print-object=\"no\" print-spacing=\"no\">"));
    }

    #[test]
    fn existing_hidden_note_is_no_op_and_changed_source_is_rejected() {
        let source = "<measure><note print-object=\"no\"><rest/></note></measure>";
        let mut xml = source.to_string();
        let start = xml.find("<note").unwrap();
        let end = xml.find("</note>").unwrap() + "</note>".len();
        assert!(!hide_note_print_object_authorized(&mut xml, start, end).0.unwrap());
        assert_eq!(xml, source);

        let source = "<measure><note><rest/></note></measure>";
        let start = source.find("<note").unwrap();
        let end = source.find("</note>").unwrap() + "</note>".len();
        let token = AuthorizedNoteVisibilityRewrite::issue(source, start, end)
            .unwrap()
            .unwrap();
        let mut changed = source.replace("<measure>", "<measure width=\"10\">");
        assert!(authorized_note_visibility_writer(&mut changed, token).is_err());
    }

    #[test]
    fn clef_move_matches_legacy_order_and_rejects_arbitrary_blocks() {
        let block = "<attributes><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes>";
        let source = format!("<measure><note><rest/><staff>2</staff></note>{block}</measure>");
        let insert_at = source.find("<note").unwrap();
        let old_start = source.find("<attributes").unwrap();
        let old_end = old_start + block.len();
        let request = LowerStaffClefMoveRequest::new(
            old_start,
            old_end,
            insert_at,
            block.to_string(),
        );
        let mut actual = source.clone();
        assert_eq!(
            apply_lower_staff_clef_moves_authorized(&mut actual, vec![request]).unwrap(),
            1
        );
        assert_eq!(
            actual,
            format!("<measure>{block}<note><rest/><staff>2</staff></note></measure>")
        );

        let bad_block = "<attributes><divisions>24</divisions><clef number=\"2\"><sign>F</sign></clef></attributes>";
        let bad_source = format!("<measure><note/><staff>2</staff>{bad_block}</measure>");
        let bad_start = bad_source.find("<attributes").unwrap();
        let bad_request = LowerStaffClefMoveRequest::new(
            bad_start,
            bad_start + bad_block.len(),
            bad_source.find("<note").unwrap(),
            bad_block.to_string(),
        );
        let mut bad_xml = bad_source;
        assert!(apply_lower_staff_clef_moves_authorized(&mut bad_xml, vec![bad_request]).is_err());
    }

    #[test]
    fn range_removal_allows_only_bound_structural_classes() {
        let source = "<measure><notations></notations><slur type=\"start\"/><note/></measure>";
        let notations_start = source.find("<notations>").unwrap();
        let notations_end = notations_start + "<notations></notations>".len();
        let slur_start = source.find("<slur").unwrap();
        let slur_end = slur_start + source[slur_start..].find('>').unwrap() + 1;
        let mut xml = source.to_string();
        assert_eq!(
            remove_allowed_xml_ranges_authorized(
                &mut xml,
                &[(notations_start, notations_end), (slur_start, slur_end)],
            )
            .0
            .unwrap(),
            1
        );
        assert_eq!(xml, "<measure><note/></measure>");

        let note_start = source.find("<note").unwrap();
        let note_end = note_start + "<note/>".len();
        let mut rejected = source.to_string();
        assert!(remove_allowed_xml_ranges_authorized(
            &mut rejected,
            &[(note_start, note_end)]
        )
        .0
        .is_err());
    }

    #[test]
    fn changed_source_rejects_range_removal_token() {
        let source = "<measure><slur type=\"start\"/></measure>";
        let start = source.find("<slur").unwrap();
        let end = start + source[start..].find('>').unwrap() + 1;
        let token = AuthorizedAllowedRangeRemovalBatch::issue(source, &[(start, end)])
            .unwrap()
            .unwrap();
        let mut changed = source.replace("<measure>", "<measure width=\"12\">");
        assert!(authorized_allowed_range_removal_writer(&mut changed, token).is_err());
    }

    #[test]
    fn three_private_capabilities_and_three_token_only_writers_are_present() {
        let source = include_str!("remaining_layout_writer_family.rs");
        let production = source.split("#[cfg(test)]").next().unwrap_or(source);
        for capability in [
            "AuthorizedLowerStaffClefMoveBatch",
            "AuthorizedNoteVisibilityRewrite",
            "AuthorizedAllowedRangeRemovalBatch",
        ] {
            assert!(production.contains(&format!("struct {capability}")));
        }
        assert!(!production.contains("pub(crate) struct Authorized"));
        assert!(!production.contains("#[derive(Clone)]\nstruct Authorized"));
        assert!(!production.contains("#[derive(Copy)]\nstruct Authorized"));
        assert_eq!(production.matches("xml.replace_range(").count(), 3);
        assert_eq!(production.matches("xml.insert_str(").count(), 0);
        assert_eq!(production.matches("std::fs::write").count(), 0);
        assert_eq!(production.matches("fs::write").count(), 0);
    }
}
