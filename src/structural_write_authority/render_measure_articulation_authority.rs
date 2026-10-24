//! Typed central authority for the remaining measure, direction, rest and
//! articulation buffer edits selected by callers.
//!
//! This module never selects a measure, note, staff, voice, pitch or musical
//! action. It binds an already selected source range and replacement to the
//! exact source bytes, validates the narrow semantic effect and consumes the
//! capability once.

use crate::master_state_identity::{BlockTransformationReceipt, ExactEdit, ExactTransformationReceipt};
use super::destructive_edit_attribution::{DestructiveEditBatchAttribution, IndividualEditIdentity};
use super::authority_primitives::{
    fingerprint,
    parsed_exact_note as exact_note,
    parsed_exact_rest_note as exact_rest_note,
    xml_fragment_parses,
    child_text,
    child_block,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderEditEffect {
    ClefMeasure,
    OctaveMeasure,
    MissingRest,

    StaccatoPersistence,
}

#[derive(Debug)]
struct BoundRenderEdit {
    identity: IndividualEditIdentity,
    start: usize,
    end: usize,
    original: String,
    original_fingerprint: u64,
    replacement: String,
}

#[derive(Debug)]
struct RenderEditBatchCapability {
    source: String,
    source_fingerprint: u64,
    effect: RenderEditEffect,
    edits_descending: Vec<BoundRenderEdit>,
    attribution: DestructiveEditBatchAttribution,
}

#[derive(Debug, Clone)]
pub struct StaccatoPersistenceOccurrenceReceipt {
    source_xml: String,
    candidate_xml: String,
    pitched_source_ranges: Vec<(usize, usize)>,
}

impl StaccatoPersistenceOccurrenceReceipt {
    pub(crate) fn source_xml(&self) -> &str {
        &self.source_xml
    }

    pub(crate) fn candidate_xml(&self) -> &str {
        &self.candidate_xml
    }

    pub(crate) fn pitched_source_ranges(&self) -> &[(usize, usize)] {
        &self.pitched_source_ranges
    }
}

impl RenderEditBatchCapability {
    fn issue(
        source: &str,
        edits: Vec<(usize, usize, String)>,
        effect: RenderEditEffect,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        let mut bound = Vec::with_capacity(edits.len());
        for (ordinal, (start, end, replacement)) in edits.into_iter().enumerate() {
            let identity = attribution.identity(ordinal)?;
            if start > end || end > source.len() {
                return None;
            }
            let original = source.get(start..end)?.to_string();
            if !edit_allowed(effect, &original, &replacement) {
                return None;
            }
            bound.push(BoundRenderEdit {
                identity,
                start,
                end,
                original_fingerprint: fingerprint(&original),
                original,
                replacement,
            });
        }
        bound.sort_by_key(|left| std::cmp::Reverse(left.start));
        for pair in bound.windows(2) {
            let later = &pair[0];
            let earlier = &pair[1];
            if earlier.end > later.start {
                return None;
            }
        }
        if effect == RenderEditEffect::StaccatoPersistence
            && !staccato_persistence_batch_allowed(source, &bound)
        {
            return None;
        }
        for edit in &bound {
            attribution.mark_bound(edit.identity);
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            effect,
            edits_descending: bound,
            attribution,
        })
    }

    fn apply(self, source: &str) -> Option<String> {
        let Self {
            source: bound_source,
            source_fingerprint,
            effect,
            edits_descending,
            attribution,
        } = self;
        if bound_source != source || source_fingerprint != fingerprint(source) {
            return None;
        }
        let identities = edits_descending
            .iter()
            .map(|edit| edit.identity)
            .collect::<Vec<_>>();
        let mut attributed_xml = bound_source;
        for edit in edits_descending {
            let current = attributed_xml.get(edit.start..edit.end)?;
            if current != edit.original
                || fingerprint(current) != edit.original_fingerprint
                || !edit_allowed(effect, current, &edit.replacement)
            {
                return None;
            }
            attributed_xml.replace_range(edit.start..edit.end, &edit.replacement);
        }
        attribution.mark_applied_batch(&identities, source, &attributed_xml);
        Some(attributed_xml)
    }
}

fn edit_allowed(effect: RenderEditEffect, original: &str, replacement: &str) -> bool {
    match effect {
        RenderEditEffect::ClefMeasure => {
            exact_measure(original)
                && exact_measure(replacement)
                && original.matches("<note").count() == replacement.matches("<note").count()
                && original.matches("<backup").count() == replacement.matches("<backup").count()
                && original.matches("<forward").count() == replacement.matches("<forward").count()
        }
        RenderEditEffect::OctaveMeasure => {
            exact_measure(original)
                && exact_measure(replacement)
                && original.matches("<note").count() == replacement.matches("<note").count()
                && original.matches("<backup").count() == replacement.matches("<backup").count()
                && original.matches("<forward").count() == replacement.matches("<forward").count()
        }
        RenderEditEffect::StaccatoPersistence => {
            let pitched_identity_preserved = exact_note(original)
                && exact_note(replacement)
                && child_block(original, "pitch") == child_block(replacement, "pitch")
                && child_text(original, "voice") == child_text(replacement, "voice")
                && child_text(original, "staff") == child_text(replacement, "staff");
            let anchor_retime = pitched_identity_preserved
                && !original.contains("<chord")
                && !replacement.contains("<chord")
                && !original.contains("<staccato")
                && replacement.contains("<staccato");
            let chord_member_retime = pitched_identity_preserved
                && original.contains("<chord")
                && replacement.contains("<chord")
                && original.contains("<staccato") == replacement.contains("<staccato");
            anchor_retime
                || chord_member_retime
                || (exact_rest_note(original) && replacement.trim().is_empty())
        }
        RenderEditEffect::MissingRest => {
            exact_forward(original)
                && child_text(original, "duration").is_some_and(|duration| {
                    exact_rest_sequence_with_total_duration(replacement, &duration)
                })
        }
    }
}

fn staccato_persistence_batch_allowed(source: &str, edits: &[BoundRenderEdit]) -> bool {
    if edits.is_empty() {
        return true;
    }

    let mut anchors = edits.iter().filter(|edit| {
        exact_note(&edit.original)
            && !edit.original.contains("<chord")
            && exact_note(&edit.replacement)
            && !edit.replacement.contains("<chord")
            && !edit.original.contains("<staccato")
            && edit.replacement.contains("<staccato")
    });
    let Some(anchor) = anchors.next() else {
        return false;
    };
    if anchors.next().is_some() {
        return false;
    }

    let mut removed_rests = edits.iter().filter(|edit| {
        exact_rest_note(&edit.original) && edit.replacement.trim().is_empty()
    });
    let Some(removed_rest) = removed_rests.next() else {
        return false;
    };
    if removed_rests.next().is_some() {
        return false;
    }

    let chord_edits = edits
        .iter()
        .filter(|edit| edit.original.contains("<chord") || edit.replacement.contains("<chord"))
        .collect::<Vec<_>>();
    if edits.len() != chord_edits.len().saturating_add(2) {
        return false;
    }

    let Some(note_before) = child_text(&anchor.original, "duration")
        .and_then(|value| value.parse::<i64>().ok())
    else {
        return false;
    };
    let Some(note_after) = child_text(&anchor.replacement, "duration")
        .and_then(|value| value.parse::<i64>().ok())
    else {
        return false;
    };
    let Some(rest_duration) = child_text(&removed_rest.original, "duration")
        .and_then(|value| value.parse::<i64>().ok())
    else {
        return false;
    };
    if note_after != note_before.saturating_add(rest_duration)
        || child_text(&anchor.original, "voice") != child_text(&removed_rest.original, "voice")
        || child_text(&anchor.original, "staff") != child_text(&removed_rest.original, "staff")
    {
        return false;
    }

    for chord_edit in &chord_edits {
        let Some(member_before) = child_text(&chord_edit.original, "duration")
            .and_then(|value| value.parse::<i64>().ok())
        else {
            return false;
        };
        let Some(member_after) = child_text(&chord_edit.replacement, "duration")
            .and_then(|value| value.parse::<i64>().ok())
        else {
            return false;
        };
        if !exact_note(&chord_edit.original)
            || !exact_note(&chord_edit.replacement)
            || !chord_edit.original.contains("<chord")
            || !chord_edit.replacement.contains("<chord")
            || member_before != note_before
            || member_after != note_after
            || child_text(&chord_edit.original, "voice") != child_text(&anchor.original, "voice")
            || child_text(&chord_edit.original, "staff") != child_text(&anchor.original, "staff")
            || child_block(&chord_edit.original, "pitch") != child_block(&chord_edit.replacement, "pitch")
            || chord_edit.original.contains("<staccato") != chord_edit.replacement.contains("<staccato")
        {
            return false;
        }
    }

    let mut expected_member_ranges = Vec::<(usize, usize)>::new();
    let mut cursor = anchor.end;
    loop {
        let Some(tail) = source.get(cursor..) else {
            return false;
        };
        let trimmed = tail.trim_start_matches(|ch: char| ch.is_whitespace());
        cursor = cursor.saturating_add(tail.len().saturating_sub(trimmed.len()));
        let Some(next) = source.get(cursor..) else {
            return false;
        };
        if !next.starts_with("<note") {
            break;
        }
        let Some(close_rel) = next.find("</note>") else {
            return false;
        };
        let end = cursor.saturating_add(close_rel).saturating_add("</note>".len());
        let Some(fragment) = source.get(cursor..end) else {
            return false;
        };
        if !fragment.contains("<chord") {
            break;
        }
        if child_text(fragment, "duration").and_then(|value| value.parse::<i64>().ok()) != Some(note_before)
            || child_text(fragment, "voice") != child_text(&anchor.original, "voice")
            || child_text(fragment, "staff") != child_text(&anchor.original, "staff")
            || child_block(fragment, "pitch").is_none()
        {
            return false;
        }
        expected_member_ranges.push((cursor, end));
        cursor = end;
    }

    if expected_member_ranges.len() != chord_edits.len() {
        return false;
    }
    expected_member_ranges.into_iter().all(|(start, end)| {
        chord_edits
            .iter()
            .any(|edit| edit.start == start && edit.end == end)
    })
}

fn exact_measure(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<measure")
        && trimmed.ends_with("</measure>")
        && trimmed.matches("</measure>").count() == 1
        && xml_fragment_parses(trimmed)
}



fn exact_forward(fragment: &str) -> bool {
    let trimmed = fragment.trim();
    trimmed.starts_with("<forward")
        && trimmed.ends_with("</forward>")
        && trimmed.matches("</forward>").count() == 1
        && trimmed.contains("<duration>")
        && xml_fragment_parses(trimmed)
}

fn exact_rest_sequence_with_total_duration(fragment: &str, expected_duration: &str) -> bool {
    let Ok(expected_duration) = expected_duration.parse::<u64>() else {
        return false;
    };
    if expected_duration == 0 {
        return false;
    }

    let wrapped = format!("<authority-root>{fragment}</authority-root>");
    let Ok(document) = roxmltree::Document::parse(&wrapped) else {
        return false;
    };
    let root = document.root_element();
    let mut note_count = 0_usize;
    let mut total_duration = 0_u64;

    for node in root.children() {
        if node.is_text() && node.text().is_some_and(|text| text.trim().is_empty()) {
            continue;
        }
        if !node.is_element() || node.tag_name().name() != "note" {
            return false;
        }
        if node.descendants().any(|descendant| {
            descendant.is_element() && descendant.tag_name().name() == "pitch"
        }) {
            return false;
        }
        if node
            .children()
            .filter(|child| child.is_element() && child.tag_name().name() == "rest")
            .count()
            != 1
        {
            return false;
        }

        let mut duration_nodes = node
            .children()
            .filter(|child| child.is_element() && child.tag_name().name() == "duration");
        let Some(duration_node) = duration_nodes.next() else {
            return false;
        };
        if duration_nodes.next().is_some() {
            return false;
        }
        let Some(duration_text) = duration_node.text() else {
            return false;
        };
        let Ok(duration) = duration_text.trim().parse::<u64>() else {
            return false;
        };
        if duration == 0 {
            return false;
        }
        let Some(next_total) = total_duration.checked_add(duration) else {
            return false;
        };
        total_duration = next_total;
        note_count += 1;
    }

    note_count > 0 && total_duration == expected_duration
}







#[track_caller]
pub fn replace_bound_clef_measure_authorized(
    source: &str,
    start: usize,
    end: usize,
    replacement: String,
) -> (Option<String>, Option<BlockTransformationReceipt>) {
    let attribution =
        DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(1);
    let Some(capability) = RenderEditBatchCapability::issue(
        source,
        vec![(start, end, replacement)],
        RenderEditEffect::ClefMeasure,
        attribution,
    ) else {
        return (None, None);
    };
    let Some(out) = capability.apply(source) else {
        return (None, None);
    };
    // BLOCK_MEASURE receipt #49, minted by this owner at the execution. The
    // frozen population contract is BLOCK, so the bound measure span is kept
    // as a binding only and no EXACT claim is made.
    let receipt = BlockTransformationReceipt::mint_block_receipt(
        ClefMeasureBlockMint { _sealed: () },
        source,
        &out,
        Some((start, end)),
    );
    (Some(out), receipt)
}

/// Sealed mint capability of `replace_bound_clef_measure_authorized` (BLOCK #49).
pub(crate) struct ClefMeasureBlockMint {
    _sealed: (),
}

#[track_caller]
pub fn replace_bound_octave_measure_authorized(
    source: &str,
    start: usize,
    end: usize,
    replacement: String,
) -> (Option<String>, Option<BlockTransformationReceipt>) {
    let attribution =
        DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(1);
    let Some(capability) = RenderEditBatchCapability::issue(
        source,
        vec![(start, end, replacement)],
        RenderEditEffect::OctaveMeasure,
        attribution,
    ) else {
        return (None, None);
    };
    let Some(out) = capability.apply(source) else {
        return (None, None);
    };
    // BLOCK_MEASURE receipt #50, minted by this owner at the execution; the
    // bound measure span is kept as a binding only (frozen BLOCK contract).
    let receipt = BlockTransformationReceipt::mint_block_receipt(
        OctaveMeasureBlockMint { _sealed: () },
        source,
        &out,
        Some((start, end)),
    );
    (Some(out), receipt)
}

/// Sealed mint capability of `replace_bound_octave_measure_authorized` (BLOCK #50).
pub(crate) struct OctaveMeasureBlockMint {
    _sealed: (),
}

pub(crate) struct MissingRestRenderExactMint { _sealed: () }


#[track_caller]
pub fn apply_bound_missing_rest_batch_authorized(
    source: &str,
    edits: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        edits.len(),
    );
    let Some(capability) = RenderEditBatchCapability::issue(
        source, edits, RenderEditEffect::MissingRest, attribution,
    ) else { return (None, None); };
    let geometry = capability.edits_descending.iter().map(|edit|
        ExactEdit::bound(edit.start, edit.end, edit.replacement.clone())
    ).collect();
    let Some(out) = capability.apply(source) else { return (None, None); };
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        MissingRestRenderExactMint { _sealed: () }, source, &out, geometry,
    );
    assert!(source == out || receipt.is_some(), "Row69 changed bytes require an EXACT receipt");
    (Some(out), receipt)
}



#[track_caller]
pub fn apply_bound_staccato_persistence_batch_authorized(
    source: &str,
    edits: Vec<(usize, usize, String)>,
) -> Option<(String, StaccatoPersistenceOccurrenceReceipt)> {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        edits.len(),
    );
    let capability = RenderEditBatchCapability::issue(
        source,
        edits,
        RenderEditEffect::StaccatoPersistence,
        attribution,
    )?;
    let pitched_source_ranges = capability
        .edits_descending
        .iter()
        .filter(|edit| exact_note(&edit.original) && child_block(&edit.original, "pitch").is_some())
        .map(|edit| (edit.start, edit.end))
        .collect::<Vec<_>>();
    if pitched_source_ranges.is_empty() {
        return None;
    }
    let source_xml = capability.source.clone();
    let candidate_xml = capability.apply(source)?;
    let receipt = StaccatoPersistenceOccurrenceReceipt {
        source_xml,
        candidate_xml: candidate_xml.clone(),
        pitched_source_ranges,
    };
    Some((candidate_xml, receipt))
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn missing_rest_receipts_bind_only_applied_pre_geometry() {
        let pre = format!("<measure>{FORWARD_120}{FORWARD_120}</measure>");
        let start = "<measure>".len();
        let end = start + FORWARD_120.len();
        let (unchanged, receipt) = apply_bound_missing_rest_batch_authorized(&pre, Vec::new());
        assert_eq!(unchanged.as_deref(), Some(pre.as_str()));
        assert!(receipt.is_none());
        for geometry in [vec![(start, end, REST_120.to_string())],
            vec![(start, end, REST_120.to_string()), (end, end + FORWARD_120.len(), REST_120.to_string())]] {
            let (post, receipt) = apply_bound_missing_rest_batch_authorized(&pre, geometry.clone());
            let post = post.unwrap();
            let receipt = receipt.unwrap();
            assert_eq!(receipt.realize_post_document(&pre).as_deref(), Some(post.as_str()));
            assert!(receipt.minted_by::<MissingRestRenderExactMint>());
            let mut legacy = pre.clone();
            for (start, end, replacement) in geometry.iter().rev() { legacy.replace_range(*start..*end, replacement); }
            assert_eq!(post, legacy);
            for altered in [ExactEdit::bound(start + 1, end, REST_120.to_string()),
                ExactEdit::bound(start, end - 1, REST_120.to_string()),
                ExactEdit::bound(start, end, REST_120.replace("120", "119"))] {
                assert!(ExactTransformationReceipt::mint_exact_receipt(
                    MissingRestRenderExactMint { _sealed: () }, &pre, &post, vec![altered]).is_none());
            }
        }
        for invalid in [vec![(start + 1, end, REST_120.to_string())],
            vec![(start, end, REST_120.replace("120", "119"))],
            vec![(start, end, REST_120.to_string()), (start, end, REST_120.to_string())]] {
            let (post, receipt) = apply_bound_missing_rest_batch_authorized(&pre, invalid);
            assert!(post.is_none());
            assert!(receipt.is_none());
        }
    }

    const FORWARD_120: &str = "<forward><duration>120</duration></forward>";
    const REST_120: &str =
        "<note><rest/><duration>120</duration><voice>2</voice><staff>2</staff></note>";
    const REST_96_24: &str = concat!(
        "<note><rest/><duration>96</duration><voice>2</voice><staff>2</staff></note>",
        "\n",
        "<note><rest/><duration>24</duration><voice>2</voice><staff>2</staff></note>"
    );

    fn apply_missing_rest(replacement: &str) -> Option<String> {
        let source = format!("<measure>{FORWARD_120}</measure>");
        let start = "<measure>".len();
        let end = start + FORWARD_120.len();
        apply_bound_missing_rest_batch_authorized(
            &source,
            vec![(start, end, replacement.to_string())],
        ).0
    }

    #[test]
    fn missing_rest_authority_accepts_single_and_split_exact_duration_sequences() {
        assert!(apply_missing_rest(REST_120).is_some());
        assert!(apply_missing_rest(REST_96_24).is_some());
    }

    #[test]
    fn missing_rest_authority_rejects_wrong_total_and_non_rest_members() {
        let wrong_total = REST_96_24.replace("<duration>24</duration>", "<duration>12</duration>");
        let pitched_member = REST_96_24.replacen(
            "<rest/>",
            "<pitch><step>C</step><octave>4</octave></pitch>",
            1,
        );
        assert!(apply_missing_rest(&wrong_total).is_none());
        assert!(apply_missing_rest(&pitched_member).is_none());
        assert!(apply_missing_rest("").is_none());

        let anchor = "<note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>";
        let member = "<note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>";
        let rest = "<note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>";
        let source = format!("<measure>{anchor}{member}{rest}</measure>");
        let anchor_start = "<measure>".len();
        let anchor_end = anchor_start + anchor.len();
        let member_start = anchor_end;
        let member_end = member_start + member.len();
        let rest_start = member_end;
        let rest_end = rest_start + rest.len();
        let anchor_after =
            crate::structural_write_authority::set_pitched_note_duration_type_add_staccato_authorized(
                anchor, 48,
            )
            .expect("anchor rewrite");
        let member_after =
            crate::structural_write_authority::ensure_atomic_move_child_text_value_authorized(
                member, "duration", "48",
            );
        let member_after =
            crate::structural_write_authority::sync_atomic_move_note_type_and_dots_authorized(
                &member_after,
            );
        assert!(
            apply_bound_staccato_persistence_batch_authorized(
                &source,
                vec![
                    (anchor_start, anchor_end, anchor_after.clone()),
                    (member_start, member_end, member_after),
                    (rest_start, rest_end, String::new()),
                ],
            )
            .is_some()
        );
        assert!(
            apply_bound_staccato_persistence_batch_authorized(
                &source,
                vec![
                    (anchor_start, anchor_end, anchor_after),
                    (rest_start, rest_end, String::new()),
                ],
            )
            .is_none(),
            "an immediately dependent equal-duration chord member must not be omitted"
        );

        let asymmetric_member = member.replace("<duration>24</duration>", "<duration>12</duration>");
        let asymmetric_source = format!("<measure>{anchor}{asymmetric_member}{rest}</measure>");
        let asymmetric_anchor_start = "<measure>".len();
        let asymmetric_anchor_end = asymmetric_anchor_start + anchor.len();
        let asymmetric_member_start = asymmetric_anchor_end;
        let asymmetric_member_end = asymmetric_member_start + asymmetric_member.len();
        let asymmetric_rest_start = asymmetric_member_end;
        let asymmetric_rest_end = asymmetric_rest_start + rest.len();
        let asymmetric_anchor_after =
            crate::structural_write_authority::set_pitched_note_duration_type_add_staccato_authorized(
                anchor, 48,
            )
            .expect("asymmetric anchor rewrite");
        let asymmetric_member_after =
            crate::structural_write_authority::ensure_atomic_move_child_text_value_authorized(
                &asymmetric_member, "duration", "48",
            );
        let asymmetric_member_after =
            crate::structural_write_authority::sync_atomic_move_note_type_and_dots_authorized(
                &asymmetric_member_after,
            );
        assert!(
            apply_bound_staccato_persistence_batch_authorized(
                &asymmetric_source,
                vec![
                    (
                        asymmetric_anchor_start,
                        asymmetric_anchor_end,
                        asymmetric_anchor_after,
                    ),
                    (
                        asymmetric_member_start,
                        asymmetric_member_end,
                        asymmetric_member_after,
                    ),
                    (asymmetric_rest_start, asymmetric_rest_end, String::new()),
                ],
            )
            .is_none(),
            "pre-existing shorter chord members are legal MusicXML but must not be normalized by the staccato transaction"
        );
    }
}
