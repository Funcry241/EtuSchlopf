//! Typed central authority for the last bounded semantic rewrites that can be
//! separated safely from their musical selectors.
//!
//! Callers continue to select every measure, note, voice, staff, pitch and
//! candidate. This module only binds exact source bytes and already-computed
//! edit plans, validates the narrow semantic effect and consumes each
//! capability once. It intentionally exposes no general raw-XML writer.

use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt};
use super::destructive_edit_attribution::{DestructiveEditBatchAttribution, IndividualEditIdentity};
use super::authority_primitives::{
    fingerprint,
    parsed_exact_note as exact_note,
    parsed_exact_rest_note as exact_rest_note,
    xml_fragment_parses,
    child_text,
};

#[derive(Debug)]
struct BoundEdit {
    identity: Option<IndividualEditIdentity>,
    start: usize,
    end: usize,
    original: String,
    original_fingerprint: u64,
    replacement: String,
}

#[derive(Debug)]
struct VoiceOverfullCropCapability {
    source: String,
    source_fingerprint: u64,
    edits_descending: Vec<BoundEdit>,
    attribution: DestructiveEditBatchAttribution,
}

impl VoiceOverfullCropCapability {
    fn issue(
        source: &str,
        edits: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        let edits_descending = bind_non_overlapping_edits(
            source,
            edits,
            Some(&attribution),
            |source, edit| {
            if edit.replacement.is_empty() {
                exact_note(&edit.original)
            } else {
                range_is_exact_child_text(source, edit.start, edit.end, "duration")
                    && positive_integer(&edit.original)
                    && positive_integer(&edit.replacement)
            }
        })?;
        for edit in &edits_descending {
            if let Some(identity) = edit.identity {
                attribution.mark_bound(identity);
            }
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            edits_descending,
            attribution,
        })
    }

    fn consume(self, source: &str) -> Option<String> {
        let Self {
            source: bound_source,
            source_fingerprint,
            edits_descending,
            attribution,
        } = self;
        let identities = edits_descending
            .iter()
            .filter_map(|edit| edit.identity)
            .collect::<Vec<_>>();
        let attributed_xml = consume_bound_edits(
            bound_source,
            source_fingerprint,
            source,
            edits_descending,
        )?;
        attribution.mark_applied_batch(&identities, source, &attributed_xml);
        Some(attributed_xml)
    }
}

#[derive(Debug)]
struct EditorialVoiceCompleteRewriteCapability {
    source: String,
    source_fingerprint: u64,
    note_ranges_descending: Vec<BoundEdit>,
    insert_position: usize,
    replacement_rest_note: String,
    replacement_fingerprint: u64,
}

impl EditorialVoiceCompleteRewriteCapability {
    fn issue(
        source: &str,
        note_ranges: Vec<(usize, usize)>,
        insert_position: usize,
        replacement_rest_note: String,
    ) -> Option<Self> {
        if note_ranges.is_empty() || !exact_rest_note(&replacement_rest_note) {
            return None;
        }
        let expected_insert = note_ranges.iter().map(|(start, _)| *start).min()?;
        if insert_position != expected_insert || insert_position > source.len() {
            return None;
        }
        let edits = note_ranges
            .into_iter()
            .map(|(start, end)| (start, end, String::new()))
            .collect();
        let note_ranges_descending = bind_non_overlapping_edits(source, edits, None, |_source, edit| {
            edit.replacement.is_empty() && exact_note(&edit.original)
        })?;
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            note_ranges_descending,
            insert_position,
            replacement_fingerprint: fingerprint(&replacement_rest_note),
            replacement_rest_note,
        })
    }

    fn consume(self, source: &str) -> Option<String> {
        if self.source != source || self.source_fingerprint != fingerprint(source) {
            return None;
        }
        let mut candidate_xml = self.source;
        for edit in self.note_ranges_descending {
            let current = candidate_xml.get(edit.start..edit.end)?;
            if current != edit.original
                || fingerprint(current) != edit.original_fingerprint
                || !exact_note(current)
            {
                return None;
            }
            candidate_xml.replace_range(edit.start..edit.end, "");
        }
        if self.insert_position > candidate_xml.len()
            || fingerprint(&self.replacement_rest_note) != self.replacement_fingerprint
            || !exact_rest_note(&self.replacement_rest_note)
        {
            return None;
        }
        candidate_xml.insert_str(self.insert_position, &self.replacement_rest_note);
        Some(candidate_xml)
    }
}



#[derive(Debug)]
struct CondensationAnchorCapability {
    source_fingerprint: u64,
    start: usize,
    end: usize,
    original: String,
    original_fingerprint: u64,
    replacement: String,
    replacement_fingerprint: u64,
}

impl CondensationAnchorCapability {
    fn issue(source: &str, start: usize, end: usize, replacement: String) -> Option<Self> {
        if start >= end || end > source.len() {
            return None;
        }
        let original = source.get(start..end)?.to_string();
        if !condensation_rewrite_allowed(&original, &replacement) {
            return None;
        }
        Some(Self {
            source_fingerprint: fingerprint(source),
            start,
            end,
            original_fingerprint: fingerprint(&original),
            original,
            replacement_fingerprint: fingerprint(&replacement),
            replacement,
        })
    }

    fn consume(self, xml: &mut String) -> bool {
        if self.source_fingerprint != fingerprint(xml)
            || self.end > xml.len()
            || xml.get(self.start..self.end) != Some(self.original.as_str())
            || fingerprint(&xml[self.start..self.end]) != self.original_fingerprint
            || fingerprint(&self.replacement) != self.replacement_fingerprint
            || !condensation_rewrite_allowed(&self.original, &self.replacement)
        {
            return false;
        }
        xml.replace_range(self.start..self.end, &self.replacement);
        true
    }
}

#[derive(Debug)]
struct AccentCleanupCapability {
    source: String,
    source_fingerprint: u64,
    accent_start: usize,
    accent_end: usize,
}

impl AccentCleanupCapability {
    fn issue(source: &str) -> Option<Self> {
        if !exact_note(source) {
            return None;
        }
        let accent_start = source.find("<accent")?;
        let accent_end = accent_start + source[accent_start..].find("/>")? + 2;
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            accent_start,
            accent_end,
        })
    }

    fn consume(self, source: &str) -> Option<String> {
        if self.source != source || self.source_fingerprint != fingerprint(source) {
            return None;
        }
        let mut out = self.source;
        if out.get(self.accent_start..self.accent_end)?.starts_with("<accent") {
            out.replace_range(self.accent_start..self.accent_end, "");
        } else {
            return None;
        }
        let open = "<articulations>";
        let close = "</articulations>";
        if let Some(art_start) = out.find(open) {
            let content_start = art_start + open.len();
            if let Some(rel_close) = out[content_start..].find(close) {
                let content_end = content_start + rel_close;
                if out[content_start..content_end].trim().is_empty() {
                    out.replace_range(art_start..content_end + close.len(), "");
                }
            }
        }
        if exact_note(&out) { Some(out) } else { None }
    }
}

#[track_caller]
pub fn apply_voice_overfull_crop_edits_authorized(
    source: &str,
    edits: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        edits.len(),
    );
    let receipt_edits = edits
        .iter()
        .map(|(start, end, replacement)| ExactEdit::bound(*start, *end, replacement.clone()))
        .collect::<Vec<_>>();
    let out = VoiceOverfullCropCapability::issue(source, edits, attribution)
        .and_then(|capability| capability.consume(source));
    // EXACT receipt #25, minted by this owner from the bound crop edits.
    let receipt = out.as_deref().and_then(|out| {
        ExactTransformationReceipt::mint_exact_receipt(
            VoiceOverfullCropExactMint { _sealed: () },
            source,
            out,
            receipt_edits,
        )
    });
    (out, receipt)
}

/// Sealed mint capability of `apply_voice_overfull_crop_edits_authorized` (EXACT #25).
pub(crate) struct VoiceOverfullCropExactMint {
    _sealed: (),
}

pub fn apply_editorial_voice_complete_rewrite_authorized(
    source: &str,
    note_ranges: Vec<(usize, usize)>,
    insert_position: usize,
    replacement_rest_note: String,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    // Receipt edits in pre-state order: the bound insertion lies at the start
    // of the first removed note, so it precedes every bound removal.
    let mut ascending_ranges = note_ranges.clone();
    ascending_ranges.sort_unstable();
    let captured_inputs = ascending_ranges.iter().map(|&(start, end)| start..end).collect::<Vec<_>>();
    let emitted_end = insert_position.checked_add(replacement_rest_note.len());
    let receipt_edits = std::iter::once(ExactEdit::bound(
        insert_position,
        insert_position,
        replacement_rest_note.clone(),
    ))
    .chain(
        ascending_ranges
            .into_iter()
            .map(|(start, end)| ExactEdit::bound(start, end, String::new())),
    )
    .collect::<Vec<_>>();
    let out = EditorialVoiceCompleteRewriteCapability::issue(
        source,
        note_ranges,
        insert_position,
        replacement_rest_note,
    )
    .and_then(|capability| capability.consume(source));
    // EXACT receipt #1, minted by this owner from its bound ranges and the
    // bound insertion; without byte-exact reproduction no receipt exists.
    let mut receipt = out.as_deref().and_then(|out| {
        ExactTransformationReceipt::mint_exact_receipt(
            EditorialVoiceCompleteRewriteExactMint { _sealed: () },
            source,
            out,
            receipt_edits,
        )
    });
    if let (Some(after), Some(receipt), Some(emitted_end)) = (out.as_deref(), receipt.as_mut(), emitted_end)
        && let Ok(capture) = crate::arrangement::physical_births::PhysicalRhythmicBirths::editorial_rest_consolidation(
            &EditorialVoiceCompleteRewriteExactMint { _sealed: () }, receipt, source, after,
            &captured_inputs, insert_position..emitted_end,
        ) {
        // Observation only: a failed capture never supplies context or changes the edit.
        let _ = receipt.attach_physical_births(capture);
    }
    (out, receipt)
}

/// Sealed mint capability of `apply_editorial_voice_complete_rewrite_authorized` (EXACT #1).
pub(crate) struct EditorialVoiceCompleteRewriteExactMint {
    _sealed: (),
}


pub fn replace_bound_condensation_rest_authorized(
    xml: &mut String,
    start: usize,
    end: usize,
    replacement: String,
) -> bool {
    CondensationAnchorCapability::issue(xml, start, end, replacement)
        .is_some_and(|capability| capability.consume(xml))
}

pub fn remove_first_bound_accent_authorized(source: &str) -> Option<String> {
    AccentCleanupCapability::issue(source)?.consume(source)
}

fn bind_non_overlapping_edits<F>(
    source: &str,
    edits: Vec<(usize, usize, String)>,
    attribution: Option<&DestructiveEditBatchAttribution>,
    allowed: F,
) -> Option<Vec<BoundEdit>>
where
    F: Fn(&str, &BoundEdit) -> bool,
{
    let mut bound = Vec::with_capacity(edits.len());
    for (ordinal, (start, end, replacement)) in edits.into_iter().enumerate() {
        let identity = attribution.and_then(|batch| batch.identity(ordinal));
        if attribution.is_some() && identity.is_none() {
            return None;
        }
        if start >= end || end > source.len() {
            return None;
        }
        let original = source.get(start..end)?.to_string();
        let edit = BoundEdit {
            identity,
            start,
            end,
            original_fingerprint: fingerprint(&original),
            original,
            replacement,
        };
        if !allowed(source, &edit) {
            return None;
        }
        bound.push(edit);
    }
    bound.sort_by_key(|left| std::cmp::Reverse(left.start));
    bound.dedup_by(|left, right| left.start == right.start && left.end == right.end);
    for pair in bound.windows(2) {
        if pair[1].end > pair[0].start {
            return None;
        }
    }
    Some(bound)
}

fn consume_bound_edits(
    bound_source: String,
    source_fingerprint: u64,
    source: &str,
    edits_descending: Vec<BoundEdit>,
) -> Option<String> {
    if bound_source != source || source_fingerprint != fingerprint(source) {
        return None;
    }
    let mut candidate_xml = bound_source;
    for edit in edits_descending {
        let current = candidate_xml.get(edit.start..edit.end)?;
        if current != edit.original || fingerprint(current) != edit.original_fingerprint {
            return None;
        }
        candidate_xml.replace_range(edit.start..edit.end, &edit.replacement);
    }
    Some(candidate_xml)
}

fn range_is_exact_child_text(
    source: &str,
    start: usize,
    end: usize,
    tag: &str,
) -> bool {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    start >= open.len()
        && source.get(start - open.len()..start) == Some(open.as_str())
        && source.get(end..end + close.len()) == Some(close.as_str())
}




fn positive_integer(value: &str) -> bool {
    value.trim().parse::<i64>().is_ok_and(|parsed| parsed > 0)
}

fn condensation_rewrite_allowed(original: &str, replacement: &str) -> bool {
    if !exact_rest_note(original) || !xml_fragment_parses(replacement) {
        return false;
    }
    let notes = note_blocks(replacement);
    if notes.len() != 2
        || !exact_note(notes[0])
        || !exact_note(notes[1])
        || !notes[0].contains("<pitch>")
        || notes[0].contains("<rest")
        || !exact_rest_note(notes[1])
    {
        return false;
    }
    let Some(original_duration) = child_text(original, "duration")
        .and_then(|value| value.parse::<i64>().ok())
    else {
        return false;
    };
    let Some(first_duration) = child_text(notes[0], "duration")
        .and_then(|value| value.parse::<i64>().ok())
    else {
        return false;
    };
    let Some(second_duration) = child_text(notes[1], "duration")
        .and_then(|value| value.parse::<i64>().ok())
    else {
        return false;
    };
    first_duration > 0
        && second_duration > 0
        && first_duration.saturating_add(second_duration) == original_duration
        && child_text(original, "voice") == child_text(notes[0], "voice")
        && child_text(original, "voice") == child_text(notes[1], "voice")
        && child_text(original, "staff") == child_text(notes[0], "staff")
        && child_text(original, "staff") == child_text(notes[1], "staff")
}

fn note_blocks(fragment: &str) -> Vec<&str> {
    let mut notes = Vec::new();
    let mut scan = 0usize;
    while let Some(relative) = fragment[scan..].find("<note") {
        let start = scan + relative;
        let Some(close_relative) = fragment[start..].find("</note>") else {
            return Vec::new();
        };
        let end = start + close_relative + "</note>".len();
        notes.push(&fragment[start..end]);
        scan = end;
    }
    notes
}




#[cfg(test)]
mod editorial_complete_rewrite_receipt_tests {
    use super::apply_editorial_voice_complete_rewrite_authorized;

    #[test]
    fn complete_rewrite_carries_its_exact_receipt() {
        let first = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>2</voice></note>";
        let second = "<note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>2</voice></note>";
        let kept = "<note><pitch><step>E</step><octave>5</octave></pitch><duration>2</duration><voice>1</voice></note>";
        let source = format!("<measure>{kept}<backup><duration>2</duration></backup>{first}{second}</measure>");
        let first_start = source.find(first).expect("first note");
        let second_start = source.find(second).expect("second note");
        let rest = "<note><rest/><duration>2</duration><voice>2</voice></note>".to_string();
        // Ranges are passed in descending order on purpose: the receipt must
        // not depend on the caller's range order.
        let (rewritten, receipt) = apply_editorial_voice_complete_rewrite_authorized(
            &source,
            vec![
                (second_start, second_start + second.len()),
                (first_start, first_start + first.len()),
            ],
            first_start,
            rest.clone(),
        );
        let rewritten = rewritten.expect("bound complete rewrite must apply");
        assert_eq!(
            rewritten,
            format!("<measure>{kept}<backup><duration>2</duration></backup>{rest}</measure>")
        );
        assert!(receipt.is_some(), "a byte-exact complete rewrite must carry its EXACT receipt");
    }
}
