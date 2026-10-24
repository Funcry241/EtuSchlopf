//! Typed authority for the five remaining external XML-buffer owners whose
//! musical selectors already live outside the write boundary.
//!
//! The exported entry points accept only already-selected, source-bound edits
//! for one narrow semantic effect. Constructors are private, capabilities are
//! consumed once, and no general raw-XML edit entry point is exposed.

use super::authority_primitives::fingerprint;
use super::destructive_edit_attribution::{DestructiveEditBatchAttribution, IndividualEditIdentity};
use crate::master_state_identity::{
    exact_script_from_descending_application, ExactTransformationReceipt,
};

#[derive(Debug)]
struct BoundEdit {
    identity: IndividualEditIdentity,
    start: usize,
    end: usize,
    original: String,
    original_fingerprint: u64,
    replacement: String,
    replacement_fingerprint: u64,
}

#[derive(Debug)]
struct VisibilityStaffDistributionCapability {
    source: String,
    source_fingerprint: u64,
    edits_descending: Vec<BoundEdit>,
    attribution: DestructiveEditBatchAttribution,
}

impl VisibilityStaffDistributionCapability {
    fn issue(
        source: &str,
        edits: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        if edits.is_empty() {
            return None;
        }
        let edits_descending = bind_edits(source, edits, false, &attribution, |edit| {
            exact_note(&edit.original)
                && exact_note(&edit.replacement)
                && !note_start_has_visibility_attribute(&edit.replacement)
                && normalize_visibility_and_staff(&edit.original)
                    == normalize_visibility_and_staff(&edit.replacement)
        })?;
        for edit in &edits_descending {
            attribution.mark_bound(edit.identity);
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
            .map(|edit| edit.identity)
            .collect::<Vec<_>>();
        let attributed_xml = apply_bound_edits(
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
struct ChordGestureDistributionCapability {
    source: String,
    source_fingerprint: u64,
    edits_descending: Vec<BoundEdit>,
    attribution: DestructiveEditBatchAttribution,
}

impl ChordGestureDistributionCapability {
    fn issue(
        source: &str,
        edits: Vec<(usize, usize, String)>,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        if edits.is_empty() {
            return None;
        }
        let edits_descending = bind_edits(source, edits, true, &attribution, |_edit| true)?;

        // r1017/r02: the existing chord-gesture authority also accepts the exact
        // inverse orphan-host canonicalization: remove one hidden rest host and
        // promote the immediately following visible pitched chord member by
        // removing only its <chord/> marker. Validation is immutable here so this
        // selector does not itself become a new writer root.
        let host_removals = edits_descending
            .iter()
            .filter(|edit| {
                edit.start < edit.end
                    && edit.replacement.is_empty()
                    && exact_note(&edit.original)
                    && edit.original.contains("<rest")
                    && !edit.original.contains("<pitch>")
                    && !edit.original.contains("<chord")
                    && (edit.original.contains("print-object=\"no\"")
                        || edit.original.contains("print-object='no'"))
                    && (edit.original.contains("print-spacing=\"no\"")
                        || edit.original.contains("print-spacing='no'"))
            })
            .collect::<Vec<_>>();
        let anchor_promotions = edits_descending
            .iter()
            .filter(|edit| {
                edit.start < edit.end
                    && !edit.replacement.is_empty()
                    && exact_note(&edit.original)
                    && exact_note(&edit.replacement)
                    && edit.original.contains("<chord")
                    && edit.original.contains("<pitch>")
                    && edit.replacement.contains("<pitch>")
                    && !edit.replacement.contains("<chord")
                    && !note_start_has_visibility_attribute(&edit.original)
                    && !note_start_has_visibility_attribute(&edit.replacement)
            })
            .collect::<Vec<_>>();
        let beam_rewrites = edits_descending
            .iter()
            .filter(|edit| {
                edit.start < edit.end
                    && !edit.replacement.is_empty()
                    && exact_note(&edit.original)
                    && exact_note(&edit.replacement)
                    && edit.original.contains("<pitch>")
                    && edit.replacement.contains("<pitch>")
                    && !edit.original.contains("<chord")
                    && !edit.replacement.contains("<chord")
                    && edit.original.contains("<beam number=\"1\">begin</beam>")
                    && !edit.original.contains("<beam number=\"2\">")
                    && edit.replacement.contains("<beam number=\"1\">continue</beam>")
                    && edit.replacement.contains("<beam number=\"2\">end</beam>")
            })
            .collect::<Vec<_>>();
        if !host_removals.is_empty()
            && host_removals.len() == anchor_promotions.len()
            && host_removals.len() + anchor_promotions.len() + beam_rewrites.len()
                == edits_descending.len()
        {
            let extract = |note: &str, tag: &str| -> Option<String> {
                let open = format!("<{tag}>");
                let close = format!("</{tag}>");
                let start = note.find(&open)? + open.len();
                let end = start + note[start..].find(&close)?;
                Some(note[start..end].trim().to_string())
            };
            let mut beamed_anchor_promotions = 0_usize;
            for host in &host_removals {
                let anchor = anchor_promotions.iter().find(|anchor| {
                    host.end <= anchor.start
                        && source
                            .get(host.end..anchor.start)
                            .is_some_and(|between| between.trim().is_empty())
                })?;
                for tag in ["duration", "voice", "staff"] {
                    let host_value = extract(&host.original, tag)?;
                    let anchor_value = extract(&anchor.original, tag)?;
                    if host_value.is_empty() || host_value != anchor_value {
                        return None;
                    }
                }
                let chord_start = anchor.original.find("<chord")?;
                let chord_end = chord_start + anchor.original[chord_start..].find('>')? + 1;
                let expected_plain = format!(
                    "{}{}",
                    &anchor.original[..chord_start],
                    &anchor.original[chord_end..]
                );
                if anchor.replacement == expected_plain {
                    continue;
                }
                let close = expected_plain.rfind("</note>")?;
                let expected_beamed = format!(
                    "{}\n    <beam number=\"1\">begin</beam>\n    <beam number=\"2\">begin</beam>{}",
                    &expected_plain[..close],
                    &expected_plain[close..]
                );
                if anchor.replacement != expected_beamed {
                    return None;
                }
                beamed_anchor_promotions = beamed_anchor_promotions.saturating_add(1);
            }
            if beam_rewrites.len() != beamed_anchor_promotions {
                return None;
            }
            for beam in &beam_rewrites {
                let primary_begin = "<beam number=\"1\">begin</beam>";
                let primary_continue = "<beam number=\"1\">continue</beam>";
                let continued = beam.original.replacen(primary_begin, primary_continue, 1);
                let close = continued.rfind("</note>")?;
                let expected = format!(
                    "{}\n    <beam number=\"2\">end</beam>{}",
                    &continued[..close],
                    &continued[close..]
                );
                if beam.replacement != expected || extract(&beam.original, "type")? != "16th" {
                    return None;
                }
                let beam_staff = extract(&beam.original, "staff")?;
                let beam_duration = extract(&beam.original, "duration")?.parse::<i64>().ok()?;
                let anchor = anchor_promotions
                    .iter()
                    .filter(|anchor| anchor.end <= beam.start)
                    .filter(|anchor| extract(&anchor.original, "staff").as_deref() == Some(beam_staff.as_str()))
                    .filter(|anchor| {
                        extract(&anchor.original, "duration")
                            .and_then(|value| value.parse::<i64>().ok())
                            == Some(beam_duration)
                    })
                    .max_by_key(|anchor| anchor.start)?;
                if extract(&anchor.original, "voice")? == extract(&beam.original, "voice")? {
                    return None;
                }
                let between = source.get(anchor.end..beam.start)?;
                if between.contains("<backup")
                    || between.contains("<forward")
                    || between.contains("<rest")
                    || between.matches("<note").count() != between.matches("<chord").count()
                {
                    return None;
                }
                let suffix = source.get(beam.end..)?;
                let next_start = suffix.find("<note")?;
                let prefix = &suffix[..next_start];
                if prefix.contains("<backup") || prefix.contains("<forward") || prefix.contains("</measure") {
                    return None;
                }
                let next_end_rel = suffix[next_start..].find("</note>")? + "</note>".len();
                let next_note = &suffix[next_start..next_start + next_end_rel];
                if !exact_note(next_note)
                    || !next_note.contains("<pitch>")
                    || next_note.contains("<chord")
                    || !next_note.contains("<beam number=\"1\">end</beam>")
                    || next_note.contains("<beam number=\"2\">")
                    || extract(next_note, "type")? != "eighth"
                    || extract(next_note, "staff")? != beam_staff
                    || extract(next_note, "voice")? != extract(&anchor.original, "voice")?
                    || extract(next_note, "duration")?.parse::<i64>().ok()?
                        != beam_duration.saturating_mul(2)
                {
                    return None;
                }
            }
            for edit in &edits_descending {
                attribution.mark_bound(edit.identity);
            }
            return Some(Self {
                source: source.to_string(),
                source_fingerprint: fingerprint(source),
                edits_descending,
                attribution,
            });
        }

        let removals = edits_descending
            .iter()
            .filter(|edit| edit.start < edit.end && edit.replacement.is_empty())
            .collect::<Vec<_>>();
        let insertions = edits_descending
            .iter()
            .filter(|edit| edit.start == edit.end && !edit.replacement.is_empty())
            .collect::<Vec<_>>();
        if removals.is_empty()
            || removals.len() != insertions.len()
            || removals.len() + insertions.len() != edits_descending.len()
        {
            return None;
        }

        let mut removed_pitches = Vec::with_capacity(removals.len());
        for edit in removals {
            if !exact_note(&edit.original)
                || !(edit.original.contains("print-object=\"no\"")
                    || edit.original.contains("print-spacing=\"no\""))
            {
                return None;
            }
            removed_pitches.push(pitch_fragment(&edit.original)?.to_string());
        }

        let mut inserted_pitches = Vec::with_capacity(insertions.len());
        for edit in insertions {
            if !exact_note(&edit.replacement)
                || !edit.replacement.contains("<chord")
                || note_start_has_visibility_attribute(&edit.replacement)
                || !source.get(..edit.start)?.ends_with("</note>")
            {
                return None;
            }
            inserted_pitches.push(pitch_fragment(&edit.replacement)?.to_string());
        }
        removed_pitches.sort();
        inserted_pitches.sort();
        if removed_pitches != inserted_pitches {
            return None;
        }

        for edit in &edits_descending {
            attribution.mark_bound(edit.identity);
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
            .map(|edit| edit.identity)
            .collect::<Vec<_>>();
        let attributed_xml = apply_bound_edits(
            bound_source,
            source_fingerprint,
            source,
            edits_descending,
        )?;
        attribution.mark_applied_batch(&identities, source, &attributed_xml);
        Some(attributed_xml)
    }
}




#[track_caller]
pub fn apply_visibility_staff_distribution_batch_authorized(
    source: &str,
    edits: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        edits.len(),
    );
    let Some(capability) = VisibilityStaffDistributionCapability::issue(source, edits, attribution)
    else {
        return (None, None);
    };
    // EXACT receipt #27: the bound edits in the order the writer applies them.
    let bound_script = bound_application_script(&capability.edits_descending);
    let Some(out) = capability.consume(source) else {
        return (None, None);
    };
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        VisibilityStaffDistributionExactMint { _sealed: () },
        source,
        &out,
        exact_script_from_descending_application(bound_script),
    );
    (Some(out), receipt)
}

/// Sealed mint capability of `apply_visibility_staff_distribution_batch_authorized`
/// (EXACT #27).
pub(crate) struct VisibilityStaffDistributionExactMint {
    _sealed: (),
}

#[track_caller]
pub fn apply_chord_gesture_distribution_batch_authorized(
    source: &str,
    edits: Vec<(usize, usize, String)>,
) -> (Option<(String, usize)>, Option<ExactTransformationReceipt>) {
    let changed = edits.len();
    // The owner's own removal/insertion pairs, kept before binding consumes them.
    let relocation_pairs = edits.clone();
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        changed,
    );
    let Some(capability) = ChordGestureDistributionCapability::issue(source, edits, attribution)
    else {
        return (None, None);
    };
    // EXACT receipt #26: the bound edits in the order the writer applies them.
    let bound_script = bound_application_script(&capability.edits_descending);
    let Some(attributed_xml) = capability.consume(source) else {
        return (None, None);
    };
    let mut receipt = ExactTransformationReceipt::mint_exact_receipt(
        ChordGestureDistributionExactMint { _sealed: () },
        source,
        &attributed_xml,
        exact_script_from_descending_application(bound_script),
    );
    // Capture the actual relocations from this owner's pairs. Another edit shape
    // has no capture and leaves course custody Unknown, exactly as before.
    if let Some(receipt) = receipt.as_mut()
        && let Ok(births) = crate::arrangement::physical_births::PhysicalRhythmicBirths::gesture_relocation(
            &ChordGestureDistributionExactMint { _sealed: () }, source, &attributed_xml, &relocation_pairs)
    {
        receipt.attach_physical_births(births).expect("Actual relocation births must bind their exact execution");
    }
    (Some((attributed_xml, changed)), receipt)
}

/// Sealed mint capability of `apply_chord_gesture_distribution_batch_authorized`
/// (EXACT #26).
pub(crate) struct ChordGestureDistributionExactMint {
    _sealed: (),
}

fn bound_application_script(edits_descending: &[BoundEdit]) -> Vec<(usize, usize, String)> {
    edits_descending
        .iter()
        .map(|edit| (edit.start, edit.end, edit.replacement.clone()))
        .collect()
}



fn bind_edits<F>(
    source: &str,
    edits: Vec<(usize, usize, String)>,
    allow_insertions: bool,
    attribution: &DestructiveEditBatchAttribution,
    allowed: F,
) -> Option<Vec<BoundEdit>>
where
    F: Fn(&BoundEdit) -> bool,
{
    let mut bound = Vec::with_capacity(edits.len());
    for (ordinal, (start, end, replacement)) in edits.into_iter().enumerate() {
        let identity = attribution.identity(ordinal)?;
        if start > end || end > source.len() || (!allow_insertions && start == end) {
            return None;
        }
        let original = source.get(start..end)?.to_string();
        let edit = BoundEdit {
            identity,
            start,
            end,
            original_fingerprint: fingerprint(&original),
            original,
            replacement_fingerprint: fingerprint(&replacement),
            replacement,
        };
        if !allowed(&edit) {
            return None;
        }
        bound.push(edit);
    }

    for (left_index, left) in bound.iter().enumerate() {
        for right in bound.iter().skip(left_index + 1) {
            let left_non_empty = left.start < left.end;
            let right_non_empty = right.start < right.end;
            if left_non_empty
                && right_non_empty
                && left.start < right.end
                && right.start < left.end
            {
                return None;
            }
            if left_non_empty
                && right.start == right.end
                && (left.start.saturating_add(1)..left.end).contains(&right.start)
            {
                return None;
            }
            if right_non_empty
                && left.start == left.end
                && (right.start.saturating_add(1)..right.end).contains(&left.start)
            {
                return None;
            }
        }
    }

    bound.sort_by(|left, right| {
        right
            .start
            .cmp(&left.start)
            .then_with(|| right.end.cmp(&left.end))
    });
    Some(bound)
}

fn apply_bound_edits(
    expected_source: String,
    expected_source_fingerprint: u64,
    source: &str,
    edits_descending: Vec<BoundEdit>,
) -> Option<String> {
    if expected_source != source || expected_source_fingerprint != fingerprint(source) {
        return None;
    }
    for edit in &edits_descending {
        let current = source.get(edit.start..edit.end)?;
        if current != edit.original
            || fingerprint(current) != edit.original_fingerprint
            || fingerprint(&edit.replacement) != edit.replacement_fingerprint
        {
            return None;
        }
    }
    let mut candidate_xml = expected_source;
    for edit in edits_descending {
        candidate_xml.replace_range(edit.start..edit.end, &edit.replacement);
    }
    Some(candidate_xml)
}

fn exact_note(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with("<note")
        && trimmed.ends_with("</note>")
        && trimmed.matches("<note").count() == 1
        && trimmed.matches("</note>").count() == 1
}

fn note_start_has_visibility_attribute(note: &str) -> bool {
    let Some(open_end) = note.find('>') else {
        return true;
    };
    let start_tag = &note[..=open_end];
    start_tag.contains("print-object=") || start_tag.contains("print-spacing=")
}

fn normalize_visibility_and_staff(note: &str) -> String {
    let without_attributes = note
        .replace(" print-object=\"no\"", "")
        .replace(" print-object=\"yes\"", "")
        .replace(" print-spacing=\"no\"", "")
        .replace(" print-spacing=\"yes\"", "");
    normalize_child_text(&without_attributes, "staff")
}

fn normalize_child_text(source: &str, tag: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let Some(start) = source.find(&open) else {
        return source.to_string();
    };
    let content_start = start + open.len();
    let Some(relative_end) = source[content_start..].find(&close) else {
        return source.to_string();
    };
    let content_end = content_start + relative_end;
    format!("{}#{}", &source[..content_start], &source[content_end..])
}

fn pitch_fragment(note: &str) -> Option<&str> {
    let start = note.find("<pitch>")?;
    let end = start + note[start..].find("</pitch>")? + "</pitch>".len();
    note.get(start..end)
}





