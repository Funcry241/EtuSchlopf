//! One source-bound horizontal-layout materializer.
//!
//! The planner owns only structural slot directives. This authority receives no
//! project identity, measure number, pitch, staff or voice identity. It binds
//! one immutable directive vector to the complete source and materializes width,
//! system-break and page-break state in one transaction.

use super::authority_primitives::{fingerprint as source_fingerprint, is_measure_tag};
use anyhow::{bail, Context, Result};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HorizontalSystemBreakMaterialization {
    None,
    Enabled,
    ExplicitlyDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HorizontalLayoutMaterializationDirective {
    pub(crate) explicit_width: Option<String>,
    pub(crate) system_break: HorizontalSystemBreakMaterialization,
    pub(crate) starts_new_page: bool,
}

#[derive(Debug)]
struct AuthorizedHorizontalLayoutPlanMaterialization {
    source_fingerprint: u64,
    directives_fingerprint: u64,
    replacement: String,
    replacement_fingerprint: u64,
    changed_slots: usize,
}

/// Constructive observation of the unchanged E012 calculation. The algorithm
/// visits original measure spans in reverse, and inside each replacement copies
/// every complete rhythmic child in lexical order. Width, print and generated
/// direction edits cannot permute those children. This sealed relation is made
/// from that calculation before the file writer runs, never by matching the
/// persisted output on pitch, time, staff, voice or text similarity.
#[derive(Debug)]
pub(super) struct OriginalHorizontalLayoutCopies {
    before: String,
    after: String,
    copies: std::collections::BTreeMap<(usize, usize), std::ops::Range<usize>>,
}

impl OriginalHorizontalLayoutCopies {
    pub(super) fn documents(&self) -> (&str, &str) { (&self.before, &self.after) }

    pub(super) fn carry(&self, range: std::ops::Range<usize>) -> Result<std::ops::Range<usize>> {
        self.copies.get(&(range.start, range.end)).cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing original E012 complete-element copy"))
    }
}

pub(super) fn observe_original_horizontal_layout_copies(
    source: &str, directives: &[HorizontalLayoutMaterializationDirective],
) -> Result<OriginalHorizontalLayoutCopies> {
    // Use the existing source-bound immutable plan calculation, including its
    // directive validation. No second materializer or mutation primitive.
    let plan = AuthorizedHorizontalLayoutPlanMaterialization::issue(source, directives)?;
    let original = roxmltree::Document::parse(source)?;
    let calculated = roxmltree::Document::parse(&plan.replacement)?;
    let children = |doc: &roxmltree::Document<'_>| -> Result<Vec<std::ops::Range<usize>>> {
        doc.descendants().filter(|node| node.is_element()
            && matches!(node.tag_name().name(), "note" | "forward" | "backup"))
            .map(|node| {
                anyhow::ensure!(node.parent().is_some_and(|parent| parent.has_tag_name("measure")),
                    "E012 copy theorem requires direct rhythmic measure children");
                Ok(node.range())
            }).collect()
    };
    let input = children(&original)?;
    let output = children(&calculated)?;
    anyhow::ensure!(input.len() == output.len(), "E012 calculation changed rhythmic copy coverage");
    let mut copies = std::collections::BTreeMap::new();
    for (before, after) in input.into_iter().zip(output) {
        anyhow::ensure!(source[before.clone()] == plan.replacement[after.clone()],
            "E012 calculation changed an original complete rhythmic child");
        copies.insert((before.start, before.end), after);
    }
    Ok(OriginalHorizontalLayoutCopies {
        before: source.to_owned(), after: plan.replacement, copies,
    })
}

impl AuthorizedHorizontalLayoutPlanMaterialization {
    fn issue(
        source_xml: &str,
        directives: &[HorizontalLayoutMaterializationDirective],
    ) -> Result<Self> {
        validate_directives(directives)?;
        let (replacement, changed_slots) = materialize_like_canonical(source_xml, directives)?;
        Ok(Self {
            source_fingerprint: source_fingerprint(source_xml),
            directives_fingerprint: fingerprint_directives(directives),
            replacement_fingerprint: source_fingerprint(&replacement),
            replacement,
            changed_slots,
        })
    }
}

pub(crate) fn materialize_horizontal_layout_plan_authorized(
    xml: &mut String,
    directives: &[HorizontalLayoutMaterializationDirective],
) -> Result<usize> {
    let token = AuthorizedHorizontalLayoutPlanMaterialization::issue(xml, directives)?;
    authorized_horizontal_layout_plan_writer(xml, directives, token)
}

pub(crate) fn materialize_horizontal_layout_file_authorized(
    path: &Path,
    directives: &[HorizontalLayoutMaterializationDirective],
) -> Result<usize> {
    let path_text = path.to_string_lossy();
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if path_text.trim().is_empty()
        || !file_name.ends_with(".musicxml")
    {
        bail!("horizontal-layout final artifact path is not a MusicXML file");
    }

    let mut xml = std::fs::read_to_string(path).with_context(|| {
        format!(
            "horizontal-layout final artifact could not be read: {}",
            path.display()
        )
    })?;
    let changed_slots = materialize_horizontal_layout_plan_authorized(&mut xml, directives)?;
    let expected_fingerprint = source_fingerprint(&xml);
    let render_epoch_before = crate::audit_only::writer_trace::capture_before(path);
    std::fs::write(path, xml.as_bytes()).with_context(|| {
        format!(
            "horizontal-layout final artifact could not be written: {}",
            path.display()
        )
    })?;
    let readback = std::fs::read_to_string(path).with_context(|| {
        format!(
            "horizontal-layout final artifact readback failed: {}",
            path.display()
        )
    })?;
    if source_fingerprint(&readback) != expected_fingerprint || readback != xml {
        bail!("horizontal-layout final artifact exact readback mismatch");
    }
    if render_epoch_before.is_some() {
        crate::audit_only::writer_trace::record_after(
            "horizontal_layout_persistence",
            path,
            render_epoch_before.as_deref(),
            &readback,
        );
    }
    Ok(changed_slots)
}

fn authorized_horizontal_layout_plan_writer(
    xml: &mut String,
    directives: &[HorizontalLayoutMaterializationDirective],
    token: AuthorizedHorizontalLayoutPlanMaterialization,
) -> Result<usize> {
    if source_fingerprint(xml) != token.source_fingerprint {
        bail!("horizontal-layout source changed after plan authorization");
    }
    if fingerprint_directives(directives) != token.directives_fingerprint {
        bail!("horizontal-layout directives changed after plan authorization");
    }
    let (expected, changed_slots) = materialize_like_canonical(xml, directives)?;
    if changed_slots != token.changed_slots
        || expected != token.replacement
        || source_fingerprint(&expected) != token.replacement_fingerprint
    {
        bail!("horizontal-layout materialization delta changed after authorization");
    }
    // r490: Horizontal layout is the last persisted MusicXML transformation.
    // Validate the complete authorized replacement before it can be assigned or
    // written. The existing file materializer then performs its unchanged exact
    // byte readback, so persisted final bytes are identical to these validated
    // bytes without giving a reader any mutation authority.
    super::materialization_verifier::validate_serialized_temporal_materialization(
        &token.replacement,
    )
    .context("horizontal-layout final temporal postcondition failed")?;
    *xml = token.replacement;
    Ok(token.changed_slots)
}

fn validate_directives(directives: &[HorizontalLayoutMaterializationDirective]) -> Result<()> {
    if directives.is_empty() {
        bail!("horizontal-layout materialization requires at least one structural slot");
    }
    for directive in directives {
        if directive.starts_new_page
            && directive.system_break == HorizontalSystemBreakMaterialization::Enabled
        {
            bail!("page break and enabled system break cannot own the same structural slot");
        }
        if let Some(width) = directive.explicit_width.as_deref() {
            let value = width
                .trim()
                .parse::<usize>()
                .map_err(|_| anyhow::anyhow!("horizontal-layout width is not numeric"))?;
            if value == 0 {
                bail!("horizontal-layout width must be positive");
            }
        }
    }
    Ok(())
}

fn materialize_like_canonical(
    source_xml: &str,
    directives: &[HorizontalLayoutMaterializationDirective],
) -> Result<(String, usize)> {
    let ranges = collect_measure_ranges(source_xml)?;
    if ranges.len() != directives.len() {
        bail!(
            "horizontal-layout plan/source slot mismatch: plan={} source={}",
            directives.len(),
            ranges.len()
        );
    }

    let mut out = source_xml.to_string();
    let reader_removals = crate::registernotation::reader_policy::final_removals(source_xml, directives);
    let mut changed_slots = 0usize;
    for (slot_index, (start, end)) in ranges.into_iter().enumerate().rev() {
        let current = &source_xml[start..end];
        let mut reader_projection = current.to_owned();
        for range in reader_removals.iter().rev().filter(|r|r.start>=start && r.end<=end) {
            reader_projection.replace_range(range.start-start..range.end-start, "");
        }
        let replacement = rewrite_measure(&reader_projection, &directives[slot_index])?;
        if replacement != current {
            out.replace_range(start..end, &replacement);
            changed_slots += 1;
        }
    }
    Ok((out, changed_slots))
}

fn collect_measure_ranges(source_xml: &str) -> Result<Vec<(usize, usize)>> {
    let mut ranges = Vec::new();
    let mut pos = 0usize;
    while let Some(relative) = source_xml[pos..].find("<measure") {
        let start = pos + relative;
        if !is_measure_tag(source_xml, start) {
            pos = start + "<measure".len();
            continue;
        }
        let Some(open_end_relative) = source_xml[start..].find('>') else {
            bail!("horizontal-layout measure opening tag is not closed");
        };
        let content_start = start + open_end_relative + 1;
        let Some(close_relative) = source_xml[content_start..].find("</measure>") else {
            bail!("horizontal-layout measure is not closed");
        };
        let end = content_start + close_relative + "</measure>".len();
        ranges.push((start, end));
        pos = end;
    }
    if ranges.is_empty() {
        bail!("horizontal-layout materialization found no structural measure slots");
    }
    Ok(ranges)
}

fn rewrite_measure(
    measure: &str,
    directive: &HorizontalLayoutMaterializationDirective,
) -> Result<String> {
    if !measure.starts_with("<measure") {
        bail!("horizontal-layout materializer received a non-measure fragment");
    }
    let Some(open_end) = measure.find('>') else {
        bail!("horizontal-layout measure opening tag is not closed");
    };
    let opening_tag = &measure[..=open_end];
    let rewritten_opening = rewrite_width_exact(opening_tag, directive.explicit_width.as_deref());

    let close = "</measure>";
    if !measure.ends_with(close) {
        bail!("horizontal-layout measure closing tag is not canonical");
    }
    let body = &measure[open_end + 1..measure.len() - close.len()];
    let mut rewritten_body = strip_horizontal_break_attributes(body)?;

    let desired_break = if directive.starts_new_page {
        Some(("new-page", "yes"))
    } else {
        match directive.system_break {
            HorizontalSystemBreakMaterialization::None => None,
            HorizontalSystemBreakMaterialization::Enabled => Some(("new-system", "yes")),
            HorizontalSystemBreakMaterialization::ExplicitlyDisabled => {
                Some(("new-system", "no"))
            }
        }
    };
    if let Some((attribute, value)) = desired_break {
        rewritten_body = apply_break_to_leading_print_or_insert(
            &rewritten_body,
            attribute,
            value,
        )?;
    }

    let finaler_layout_neustart = directive.starts_new_page
        || directive.system_break == HorizontalSystemBreakMaterialization::Enabled;
    if finaler_layout_neustart {
        rewritten_body = strip_generated_continue_recues_at_layout_restart(&rewritten_body)?;
    }

    let mut out = String::with_capacity(
        rewritten_opening.len() + rewritten_body.len() + close.len(),
    );
    out.push_str(&rewritten_opening);
    out.push_str(&rewritten_body);
    out.push_str(close);
    Ok(out)
}

fn strip_generated_continue_recues_at_layout_restart(body: &str) -> Result<String> {
    let mut out = String::with_capacity(body.len());
    let mut pos = 0usize;
    while let Some(relative) = body[pos..].find("<direction") {
        let start = pos + relative;
        let Some(close_relative) = body[start..].find("</direction>") else {
            bail!("layout-aware Ottava Re-Cue projection found an unclosed direction");
        };
        let end = start + close_relative + "</direction>".len();
        out.push_str(&body[pos..start]);
        let block = &body[start..end];
        let staff = simple_tag_text(block, "staff");
        let layout_boundary_recue = staff
            .as_deref()
            .and_then(|staff| first_pitched_note_start_for_staff(body, staff))
            .is_some_and(|first| start < first);
        if layout_boundary_recue
            && block.contains("<octave-shift type=\"continue\"")
        {
            let mut cleaned = block.to_string();
            let mut removed = 0usize;
            for label in ["8va", "15ma", "8vb", "15mb"] {
                let cue = format!(
                    "<direction-type><words font-style=\"italic\">{label}</words></direction-type>"
                );
                let count = cleaned.matches(&cue).count();
                if count > 0 {
                    removed += count;
                    cleaned = cleaned.replace(&cue, "");
                }
            }
            if removed > 1 {
                bail!(
                    "layout-aware Ottava Re-Cue projection found more than one generated visible cue in one continue direction"
                );
            }
            out.push_str(&cleaned);
        } else {
            out.push_str(block);
        }
        pos = end;
    }
    out.push_str(&body[pos..]);
    Ok(out)
}

fn first_pitched_note_start_for_staff(body: &str, staff: &str) -> Option<usize> {
    let mut pos = 0usize;
    while let Some(relative) = body[pos..].find("<note") {
        let start = pos + relative;
        let close_relative = body[start..].find("</note>")?;
        let end = start + close_relative + "</note>".len();
        let note = &body[start..end];
        if note.contains("<pitch>")
            && simple_tag_text(note, "staff").as_deref() == Some(staff)
        {
            return Some(start);
        }
        pos = end;
    }
    None
}

fn simple_tag_text(fragment: &str, tag: &str) -> Option<String> {
    let opening = format!("<{tag}>");
    let closing = format!("</{tag}>");
    let start = fragment.find(&opening)? + opening.len();
    let end = start + fragment[start..].find(&closing)?;
    Some(fragment[start..end].trim().to_string())
}

fn rewrite_width_exact(opening_tag: &str, wanted: Option<&str>) -> String {
    let without = remove_attribute(opening_tag, "width");
    let Some(width) = wanted else {
        return without;
    };
    let Some(end) = without.rfind('>') else {
        return without;
    };
    let mut out = String::with_capacity(without.len() + width.len() + 10);
    out.push_str(&without[..end]);
    out.push_str(" width=\"");
    out.push_str(width.trim());
    out.push('"');
    out.push_str(&without[end..]);
    out
}

fn remove_attribute(tag: &str, attribute: &str) -> String {
    for quote in ['"', '\''] {
        let needle = format!("{attribute}={quote}");
        if let Some(value_marker) = tag.find(&needle) {
            let value_start = value_marker + needle.len();
            if let Some(value_end_relative) = tag[value_start..].find(quote) {
                let value_end = value_start + value_end_relative + 1;
                let mut remove_start = value_marker;
                while remove_start > 0
                    && tag
                        .as_bytes()
                        .get(remove_start - 1)
                        .is_some_and(|byte| byte.is_ascii_whitespace())
                {
                    remove_start -= 1;
                }
                let mut out = String::with_capacity(tag.len());
                out.push_str(&tag[..remove_start]);
                out.push_str(&tag[value_end..]);
                return out;
            }
        }
    }
    tag.to_string()
}

fn strip_horizontal_break_attributes(body: &str) -> Result<String> {
    let mut out = String::with_capacity(body.len());
    let mut pos = 0usize;

    while let Some(relative) = body[pos..].find("<print") {
        let start = pos + relative;
        let prefix = &body[pos..start];
        let Some(tag_end_relative) = body[start..].find('>') else {
            bail!("horizontal-layout print opening tag is not closed");
        };
        let tag_end = start + tag_end_relative + 1;
        let tag = &body[start..tag_end];
        let had_horizontal =
            contains_attribute(tag, "new-system") || contains_attribute(tag, "new-page");
        let mut clean_tag = remove_attribute(tag, "new-system");
        clean_tag = remove_attribute(&clean_tag, "new-page");

        if tag.trim_end().ends_with("/>") {
            if had_horizontal && print_tag_has_no_semantic_attributes(&clean_tag) {
                if !prefix.trim().is_empty() {
                    out.push_str(prefix);
                }
                pos = tag_end;
                continue;
            }
            out.push_str(prefix);
            out.push_str(&clean_tag);
            pos = tag_end;
            continue;
        }

        let Some(close_relative) = body[tag_end..].find("</print>") else {
            bail!("horizontal-layout print block is not closed");
        };
        let close_end = tag_end + close_relative + "</print>".len();
        let inner = &body[tag_end..tag_end + close_relative];
        if had_horizontal
            && print_tag_has_no_semantic_attributes(&clean_tag)
            && inner.trim().is_empty()
        {
            if !prefix.trim().is_empty() {
                out.push_str(prefix);
            }
            pos = close_end;
            continue;
        }
        out.push_str(prefix);
        out.push_str(&clean_tag);
        out.push_str(inner);
        out.push_str("</print>");
        pos = close_end;
    }

    out.push_str(&body[pos..]);
    Ok(out)
}

fn contains_attribute(tag: &str, attribute: &str) -> bool {
    tag.contains(&format!("{attribute}=\"")) || tag.contains(&format!("{attribute}='"))
}

fn print_tag_has_no_semantic_attributes(tag: &str) -> bool {
    let trimmed = tag.trim();
    trimmed == "<print>" || trimmed == "<print/>" || trimmed == "<print />"
}

fn apply_break_to_leading_print_or_insert(
    body: &str,
    attribute: &str,
    value: &str,
) -> Result<String> {
    let non_whitespace = body
        .char_indices()
        .find_map(|(index, ch)| (!ch.is_whitespace()).then_some(index));
    if let Some(start) = non_whitespace
        && body[start..].starts_with("<print")
    {
        let Some(tag_end_relative) = body[start..].find('>') else {
            bail!("leading print opening tag is not closed");
        };
        let tag_end = start + tag_end_relative + 1;
        let tag = &body[start..tag_end];
        let mut rewritten_tag = remove_attribute(tag, "new-system");
        rewritten_tag = remove_attribute(&rewritten_tag, "new-page");
        let Some(insert_at) = rewritten_tag.find('>') else {
            bail!("leading print tag cannot receive horizontal break");
        };
        let self_closing = rewritten_tag[..insert_at].trim_end().ends_with('/');
        let attribute_text = format!(" {attribute}=\"{value}\"");
        let actual_insert = if self_closing {
            rewritten_tag[..insert_at]
                .rfind('/')
                .unwrap_or(insert_at)
        } else {
            insert_at
        };
        rewritten_tag.insert_str(actual_insert, &attribute_text);

        let mut out = String::with_capacity(body.len() + attribute_text.len());
        out.push_str(&body[..start]);
        out.push_str(&rewritten_tag);
        out.push_str(&body[tag_end..]);
        return Ok(out);
    }

    let print = format!("\n      <print {attribute}=\"{value}\"/>");
    let mut out = String::with_capacity(body.len() + print.len());
    out.push_str(&print);
    out.push_str(body);
    Ok(out)
}

fn fingerprint_directives(
    directives: &[HorizontalLayoutMaterializationDirective],
) -> u64 {
    let mut bytes = Vec::new();
    for directive in directives {
        match directive.explicit_width.as_deref() {
            Some(width) => {
                bytes.extend_from_slice(b"W:");
                bytes.extend_from_slice(width.trim().as_bytes());
            }
            None => bytes.extend_from_slice(b"W:-"),
        }
        bytes.extend_from_slice(if directive.starts_new_page {
            b"|P:1"
        } else {
            b"|P:0"
        });
        bytes.extend_from_slice(match directive.system_break {
            HorizontalSystemBreakMaterialization::None => b"|S:-",
            HorizontalSystemBreakMaterialization::Enabled => b"|S:1",
            HorizontalSystemBreakMaterialization::ExplicitlyDisabled => b"|S:0",
        });
        bytes.push(b'\n');
    }
    source_fingerprint(std::str::from_utf8(&bytes).unwrap_or(""))
}

#[cfg(test)]
mod tests {
    use super::{
        materialize_horizontal_layout_plan_authorized,
        HorizontalLayoutMaterializationDirective,
        HorizontalSystemBreakMaterialization,
    };

    #[test]
    fn one_transaction_materializes_width_system_and_page() {
        let mut xml = concat!(
            "<score><part>",
            "<measure number=\"a\"><note/></measure>",
            "<measure number=\"b\"><print><staff-layout number=\"2\"/></print><note/></measure>",
            "<measure number=\"c\"><note/></measure>",
            "</part></score>"
        )
        .to_string();
        let directives = vec![
            HorizontalLayoutMaterializationDirective {
                explicit_width: Some("1200".to_string()),
                system_break: HorizontalSystemBreakMaterialization::None,
                starts_new_page: false,
            },
            HorizontalLayoutMaterializationDirective {
                explicit_width: None,
                system_break: HorizontalSystemBreakMaterialization::Enabled,
                starts_new_page: false,
            },
            HorizontalLayoutMaterializationDirective {
                explicit_width: None,
                system_break: HorizontalSystemBreakMaterialization::None,
                starts_new_page: true,
            },
        ];

        let changed = materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("authorized horizontal materialization");
        assert_eq!(changed, 3);
        assert!(xml.contains("<measure number=\"a\" width=\"1200\">"));
        assert!(xml.contains("<print new-system=\"yes\"><staff-layout number=\"2\"/>"));
        assert!(xml.contains("<measure number=\"c\">\n      <print new-page=\"yes\"/>"));
    }

    #[test]
    fn explicit_system_disable_is_materialized_without_identity() {
        let mut xml = "<score><part><measure><note/></measure></part></score>".to_string();
        let directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: None,
            system_break: HorizontalSystemBreakMaterialization::ExplicitlyDisabled,
            starts_new_page: false,
        }];

        materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("authorized disabled break");
        assert!(xml.contains("<print new-system=\"no\"/>"));
    }

    #[test]
    fn final_layout_restart_removes_only_generated_continue_recue() {
        let mut xml = concat!(
            "<score><part><measure>",
            "<note><pitch><step>C</step><octave>5</octave></pitch><staff>1</staff></note>",
            "<direction placement=\"above\">",
            "<direction-type><words font-style=\"italic\">8va</words></direction-type>",
            "<direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type>",
            "<staff>2</staff></direction>",
            "<note><pitch><step>D</step><octave>4</octave></pitch><staff>2</staff></note>",
            "</measure></part></score>"
        )
        .to_string();
        let directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: None,
            system_break: HorizontalSystemBreakMaterialization::Enabled,
            starts_new_page: false,
        }];

        materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("final layout restart must project the visible Ottava cue");
        assert!(xml.contains("<print new-system=\"yes\"/>"));
        assert!(xml.contains("<octave-shift type=\"continue\" size=\"8\" number=\"1\"/>"));
        assert!(!xml.contains("<words font-style=\"italic\">8va</words>"));
    }

    #[test]
    fn later_continue_inside_a_restart_measure_keeps_its_visible_recue() {
        let mut xml = concat!(
            "<score><part><measure>",
            "<note><pitch><step>C</step><octave>5</octave></pitch><staff>1</staff></note>",
            "<direction placement=\"above\">",
            "<direction-type><words font-style=\"italic\">8va</words></direction-type>",
            "<direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type>",
            "<staff>1</staff></direction><note/>",
            "</measure></part></score>"
        )
        .to_string();
        let directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: None,
            system_break: HorizontalSystemBreakMaterialization::Enabled,
            starts_new_page: false,
        }];

        materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("internal continue after a pitched event must not be treated as the layout-boundary cue");
        assert!(xml.contains("<print new-system=\"yes\"/>"));
        assert!(xml.contains("<words font-style=\"italic\">8va</words>"));
    }

    #[test]
    fn inner_system_continue_keeps_generated_visible_recue() {
        let mut xml = concat!(
            "<score><part><measure>",
            "<direction placement=\"above\">",
            "<direction-type><words font-style=\"italic\">8va</words></direction-type>",
            "<direction-type><octave-shift type=\"continue\" size=\"8\" number=\"1\"/></direction-type>",
            "<staff>1</staff></direction><note/>",
            "</measure></part></score>"
        )
        .to_string();
        let directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: None,
            system_break: HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }];

        materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("inner-system continue must remain unchanged by final layout projection");
        assert!(xml.contains("<words font-style=\"italic\">8va</words>"));
        assert!(xml.contains("<octave-shift type=\"continue\" size=\"8\" number=\"1\"/>"));
    }

    #[test]
    fn score_partwise_temporal_postcondition_rejects_orphan_chord_transactionally() {
        let original = concat!(
            "<?xml version=\"1.0\"?>",
            "<score-partwise version=\"4.0\">",
            "<part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list>",
            "<part id=\"P1\"><measure number=\"\
1\">",
            "<attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>",
            "<forward><duration>1</duration></forward>",
            "<note><chord/><pitch><step>D</step><alter>1</alter><octave>4</octave></pitch>",
            "<duration>1</duration><voice>1</voice><staff>1</staff></note>",
            "</measure></part></score-partwise>"
        )
        .to_string();
        let mut xml = original.clone();
        let directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: Some("900".to_string()),
            system_break: HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }];

        assert!(materialize_horizontal_layout_plan_authorized(&mut xml, &directives).is_err());
        assert_eq!(xml, original);
    }

    #[test]
    fn score_partwise_temporal_postcondition_accepts_hosted_chord() {
        let mut xml = concat!(
            "<?xml version=\"1.0\"?>",
            "<score-partwise version=\"4.0\">",
            "<part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list>",
            "<part id=\"P1\"><measure number=\"\
1\">",
            "<attributes><divisions>1</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>",
            "<note print-object=\"no\"><rest/><duration>1</duration><voice>1</voice><staff>1</staff></note>",
            "<note><chord/><pitch><step>D</step><alter>1</alter><octave>4</octave></pitch>",
            "<duration>1</duration><voice>1</voice><staff>1</staff></note>",
            "</measure></part></score-partwise>"
        )
        .to_string();
        let directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: Some("900".to_string()),
            system_break: HorizontalSystemBreakMaterialization::None,
            starts_new_page: false,
        }];

        let changed = materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("hosted chord remains temporally valid after layout materialization");
        assert_eq!(changed, 1);
        assert!(xml.contains("<measure number=\"\
1\" width=\"900\">"));
    }

    #[test]
    fn rematerialization_is_idempotent() {
        let mut xml =
            "<score><part><measure width=\"900\">\n      <print new-system=\"yes\"/><note/></measure></part></score>"
                .to_string();
        let directives = vec![HorizontalLayoutMaterializationDirective {
            explicit_width: Some("900".to_string()),
            system_break: HorizontalSystemBreakMaterialization::Enabled,
            starts_new_page: false,
        }];
        let first = materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("first materialization");
        let after_first = xml.clone();
        let second = materialize_horizontal_layout_plan_authorized(&mut xml, &directives)
            .expect("second materialization");
        assert!(first <= 1);
        assert_eq!(second, 0);
        assert_eq!(xml, after_first);
    }
}
