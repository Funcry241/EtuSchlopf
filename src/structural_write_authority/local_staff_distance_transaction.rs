//! Central, source-bound transaction for local staff-distance layout writes.
//!
//! The layout pass may select an exact measure and a minimum visual distance.
//! This module receives no project identity, measure number, pitch, staff or
//! voice identity. Authorization is bound to the complete source, the exact
//! measure bytes and the full replacement. The writer performs exactly one
//! `replace_range` mutation and never writes a file.

use super::authority_primitives::fingerprint as source_fingerprint;
use crate::master_state_identity::PreservingTransformationReceipt;
use anyhow::{bail, Result};

#[derive(Debug)]
struct AuthorizedLocalStaffDistanceTransaction {
    source_fingerprint: u64,
    measure_start: usize,
    measure_end: usize,
    measure_fingerprint: u64,
    target_distance: usize,
    replacement: String,
    replacement_fingerprint: u64,
}

impl AuthorizedLocalStaffDistanceTransaction {
    fn issue(
        source_xml: &str,
        measure_start: usize,
        target_distance: usize,
    ) -> Result<Option<Self>> {
        let measure_end = verify_measure_bounds(source_xml, measure_start)?;
        let measure = &source_xml[measure_start..measure_end];
        let replacement = ensure_local_staff_distance_like_legacy(measure, target_distance);
        if replacement == measure {
            return Ok(None);
        }
        Ok(Some(Self {
            source_fingerprint: source_fingerprint(source_xml),
            measure_start,
            measure_end,
            measure_fingerprint: source_fingerprint(measure),
            target_distance,
            replacement_fingerprint: source_fingerprint(&replacement),
            replacement,
        }))
    }
}

pub fn ensure_local_staff_distance_authorized(
    xml: &mut String,
    measure_start: usize,
    target_distance: usize,
) -> (Result<bool>, Option<PreservingTransformationReceipt>) {
    let token = match AuthorizedLocalStaffDistanceTransaction::issue(
        xml,
        measure_start,
        target_distance,
    ) {
        Ok(Some(token)) => token,
        Ok(None) => return (Ok(false), None),
        Err(error) => return (Err(error), None),
    };
    let (written, receipt) = authorized_local_staff_distance_writer(xml, token);
    (written.map(|_| true), receipt)
}

/// The measure-local operations behind `ensure_local_staff_distance_like_legacy`,
/// branch by branch. The replacement itself is still produced by the unchanged
/// legacy function; the journal is only kept when it reproduces it exactly.
fn local_staff_distance_operations(measure: &str, target_distance: usize) -> Vec<(std::ops::Range<usize>, String)> {
    let staff_layout = format!(
        "\n        <staff-layout number=\"2\"><staff-distance>{target_distance}</staff-distance></staff-layout>"
    );
    if measure.contains("<staff-layout number=\"2\">") {
        let mut edits = Vec::new();
        let mut scan = 0_usize;
        while let Some(relative) = measure[scan..].find("<staff-layout number=\"2\">") {
            let start = scan + relative;
            let Some(close_relative) = measure[start..].find("</staff-layout>") else { break; };
            let end = start + close_relative + "</staff-layout>".len();
            if let Some((value_start, value_end, current)) = staff_distance_value_range(&measure[start..end])
                && current < target_distance
            {
                edits.push((start + value_start..start + value_end, target_distance.to_string()));
            }
            scan = end;
        }
        return edits;
    }
    let Some(measure_open_end) = measure.find('>') else { return Vec::new(); };
    let content_start = measure_open_end + 1;
    if let Some(print_relative) = measure[content_start..].find("<print") {
        let print_start = content_start + print_relative;
        if measure[content_start..print_start].trim().is_empty()
            && let Some(tag_end_relative) = measure[print_start..].find('>')
        {
            let tag_end = print_start + tag_end_relative + 1;
            let open_tag = &measure[print_start..tag_end];
            if let Some(head) = open_tag.strip_suffix("/>") {
                return vec![(print_start..tag_end, format!("{head}>{staff_layout}\n      </print>"))];
            }
            if let Some(close_relative) = measure[tag_end..].find("</print>") {
                let close = tag_end + close_relative;
                return vec![(close..close, staff_layout)];
            }
        }
    }
    vec![(content_start..content_start, format!("\n      <print>{staff_layout}\n      </print>"))]
}

fn authorized_local_staff_distance_writer(
    xml: &mut String,
    token: AuthorizedLocalStaffDistanceTransaction,
) -> (Result<usize>, Option<PreservingTransformationReceipt>) {
    if let Err(error) = verify_local_staff_distance_token(xml, &token) {
        return (Err(error), None);
    }
    let pre_xml = xml.clone();
    xml.replace_range(token.measure_start..token.measure_end, &token.replacement);
    // PRESERVING receipt E013, minted by this owner only if the conserved
    // musical content projection is unchanged.
    let receipt = PreservingTransformationReceipt::mint_preserving_receipt(
        LocalStaffDistancePreservingMint { _sealed: () },
        &pre_xml,
        xml,
    );
    // Operation journal from this token; kept only if it replays the actual write.
    let span = token.measure_start..token.measure_end;
    let measure = &pre_xml[span.clone()];
    let journal = crate::arrangement::representation_transport::RepresentationStep::recorded(
        measure,
        &token.replacement,
        local_staff_distance_operations(measure, token.target_distance),
    )
    .enclosed(&pre_xml, xml, span)
    .ok();
    (Ok(1), receipt.map(|receipt| receipt.with_layout_journal(journal)))
}

/// Sealed mint capability of `authorized_local_staff_distance_writer`
/// (PRESERVING E013).
pub(crate) struct LocalStaffDistancePreservingMint {
    _sealed: (),
}

fn verify_local_staff_distance_token(
    xml: &str,
    token: &AuthorizedLocalStaffDistanceTransaction,
) -> Result<()> {
    if source_fingerprint(xml) != token.source_fingerprint {
        bail!("source changed after local staff-distance authorization");
    }
    let current_end = verify_measure_bounds(xml, token.measure_start)?;
    if current_end != token.measure_end {
        bail!("measure boundary changed after local staff-distance authorization");
    }
    let current_measure = &xml[token.measure_start..token.measure_end];
    if source_fingerprint(current_measure) != token.measure_fingerprint {
        bail!("measure changed after local staff-distance authorization");
    }

    let expected = ensure_local_staff_distance_like_legacy(
        current_measure,
        token.target_distance,
    );
    if expected == current_measure {
        bail!("authorized local staff-distance transaction became a no-op");
    }
    if expected != token.replacement
        || source_fingerprint(&expected) != token.replacement_fingerprint
    {
        bail!("local staff-distance delta changed after authorization");
    }
    Ok(())
}

fn ensure_local_staff_distance_like_legacy(
    measure: &str,
    target_distance: usize,
) -> String {
    let staff_layout = format!(
        "\n        <staff-layout number=\"2\"><staff-distance>{target_distance}</staff-distance></staff-layout>"
    );
    if measure.contains("<staff-layout number=\"2\">") {
        return raise_existing_staff_distance_like_legacy(measure, target_distance);
    }
    let Some(measure_open_end) = measure.find('>') else {
        return measure.to_string();
    };
    let content_start = measure_open_end + 1;
    let rest = &measure[content_start..];
    if let Some(print_relative) = rest.find("<print") {
        let print_start = content_start + print_relative;
        let before_print = &measure[content_start..print_start];
        if before_print.trim().is_empty()
            && let Some(tag_end_relative) = measure[print_start..].find('>') {
                let tag_end = print_start + tag_end_relative + 1;
                let open_tag = &measure[print_start..tag_end];
                if open_tag.ends_with("/>") {
                    let mut start_tag = open_tag.to_string();
                    start_tag.truncate(start_tag.len().saturating_sub(2));
                    start_tag.push('>');
                    let mut out = String::with_capacity(measure.len() + staff_layout.len() + 16);
                    out.push_str(&measure[..print_start]);
                    out.push_str(&start_tag);
                    out.push_str(&staff_layout);
                    out.push_str("\n      </print>");
                    out.push_str(&measure[tag_end..]);
                    return out;
                }
                if let Some(close_relative) = measure[tag_end..].find("</print>") {
                    let close = tag_end + close_relative;
                    let mut out = String::with_capacity(measure.len() + staff_layout.len());
                    out.push_str(&measure[..close]);
                    out.push_str(&staff_layout);
                    out.push_str(&measure[close..]);
                    return out;
                }
            }
    }
    let print = format!("\n      <print>{staff_layout}\n      </print>");
    let mut out = String::with_capacity(measure.len() + print.len());
    out.push_str(&measure[..content_start]);
    out.push_str(&print);
    out.push_str(&measure[content_start..]);
    out
}

fn raise_existing_staff_distance_like_legacy(
    measure: &str,
    target_distance: usize,
) -> String {
    let mut out = String::with_capacity(measure.len());
    let mut scan = 0_usize;
    let mut changed = false;
    while let Some(relative) = measure[scan..].find("<staff-layout number=\"2\">") {
        let start = scan + relative;
        let Some(close_relative) = measure[start..].find("</staff-layout>") else {
            break;
        };
        let end = start + close_relative + "</staff-layout>".len();
        out.push_str(&measure[scan..start]);
        let block = &measure[start..end];
        if let Some((value_start, value_end, current)) = staff_distance_value_range(block) {
            if current < target_distance {
                out.push_str(&block[..value_start]);
                out.push_str(&target_distance.to_string());
                out.push_str(&block[value_end..]);
                changed = true;
            } else {
                out.push_str(block);
            }
        } else {
            out.push_str(block);
        }
        scan = end;
    }
    out.push_str(&measure[scan..]);
    if changed {
        out
    } else {
        measure.to_string()
    }
}

fn staff_distance_value_range(block: &str) -> Option<(usize, usize, usize)> {
    let open = "<staff-distance>";
    let close = "</staff-distance>";
    let start = block.find(open)? + open.len();
    let end = start + block[start..].find(close)?;
    let value = block[start..end].trim().parse::<usize>().ok()?;
    Some((start, end, value))
}

fn verify_measure_bounds(source_xml: &str, measure_start: usize) -> Result<usize> {
    if measure_start >= source_xml.len() || !source_xml.is_char_boundary(measure_start) {
        bail!("invalid local staff-distance measure start {measure_start}");
    }
    if !source_xml[measure_start..].starts_with("<measure") {
        bail!("local staff-distance target is not a measure opening tag");
    }
    let next = source_xml
        .as_bytes()
        .get(measure_start + "<measure".len())
        .copied();
    if !matches!(next, Some(b' ' | b'\t' | b'\n' | b'\r' | b'>')) {
        bail!("local staff-distance target is not an exact measure tag");
    }
    let Some(opening_end_relative) = source_xml[measure_start..].find('>') else {
        bail!("local staff-distance measure opening tag is not closed");
    };
    let content_start = measure_start + opening_end_relative + 1;
    let Some(close_relative) = source_xml[content_start..].find("</measure>") else {
        bail!("local staff-distance measure is not closed");
    };
    let measure_end = content_start + close_relative + "</measure>".len();
    let mut nested_search = content_start;
    let content_end = measure_end - "</measure>".len();
    while nested_search < content_end {
        let Some(relative) = source_xml[nested_search..content_end].find("<measure") else {
            break;
        };
        let candidate = nested_search + relative;
        let next = source_xml
            .as_bytes()
            .get(candidate + "<measure".len())
            .copied();
        if matches!(next, Some(b' ' | b'\t' | b'\n' | b'\r' | b'>')) {
            bail!("nested measure encountered in local staff-distance authorization");
        }
        nested_search = candidate + "<measure".len();
    }
    Ok(measure_end)
}


#[cfg(test)]
mod tests {
    use super::{
        authorized_local_staff_distance_writer,
        ensure_local_staff_distance_authorized,
        AuthorizedLocalStaffDistanceTransaction,
    };

    #[test]
    fn raises_existing_staff_distance_without_touching_other_print_content() {
        let mut xml = concat!(
            "<measure number=\"\
7\">",
            "\n      <print>",
            "\n        <staff-layout number=\"2\"><staff-distance>90</staff-distance></staff-layout>",
            "\n        <measure-layout><measure-distance>12</measure-distance></measure-layout>",
            "\n      </print>",
            "\n      <note/>",
            "</measure>"
        )
        .to_string();
        assert!(ensure_local_staff_distance_authorized(&mut xml, 0, 122)
            .0
            .expect("authorized staff-distance raise"));
        assert!(xml.contains("<staff-distance>122</staff-distance>"));
        assert!(xml.contains("<measure-distance>12</measure-distance>"));
    }

    #[test]
    fn sufficient_existing_distance_is_a_no_op() {
        let mut xml = concat!(
            "<measure number=\"\
7\">",
            "<print><staff-layout number=\"2\"><staff-distance>140</staff-distance></staff-layout></print>",
            "</measure>"
        )
        .to_string();
        let original = xml.clone();
        assert!(!ensure_local_staff_distance_authorized(&mut xml, 0, 122)
            .0
            .expect("authorized no-op"));
        assert_eq!(xml, original);
    }

    #[test]
    fn expands_leading_self_closing_print_like_legacy() {
        let mut xml = "<measure number=\"\
8\">\n      <print new-system=\"yes\"/>\n      <note/>\n</measure>"
            .to_string();
        assert!(ensure_local_staff_distance_authorized(&mut xml, 0, 134)
            .0
            .expect("authorized print expansion"));
        assert!(xml.contains("<print new-system=\"yes\">"));
        assert!(xml.contains("<staff-distance>134</staff-distance>"));
        assert!(xml.contains("</print>"));
    }

    #[test]
    fn inserts_into_leading_regular_print_like_legacy() {
        let mut xml = "<measure number=\"\
8\">\n      <print><measure-layout/></print>\n      <note/>\n</measure>"
            .to_string();
        assert!(ensure_local_staff_distance_authorized(&mut xml, 0, 122)
            .0
            .expect("authorized regular print insertion"));
        assert!(xml.contains("<measure-layout/>"));
        assert!(xml.contains("<staff-distance>122</staff-distance>"));
    }

    #[test]
    fn inserts_new_print_when_no_leading_print_exists() {
        let mut xml = "<measure number=\"\
8\">\n      <note/>\n</measure>"
            .to_string();
        assert!(ensure_local_staff_distance_authorized(&mut xml, 0, 122)
            .0
            .expect("authorized new print insertion"));
        assert!(xml.starts_with(concat!(
            "<measure number=\"\
8\">",
            "\n      <print>"
        )));
        assert!(xml.contains("<staff-distance>122</staff-distance>"));
    }

    #[test]
    fn changed_source_after_authorization_is_rejected() {
        let source = "<measure number=\"\
8\"><note/></measure>";
        let token = AuthorizedLocalStaffDistanceTransaction::issue(source, 0, 122)
            .expect("token result")
            .expect("token");
        let mut changed = source.replace("<note/>", "<note><rest/></note>");
        let error = authorized_local_staff_distance_writer(&mut changed, token)
            .0
            .expect_err("changed source must be rejected");
        assert!(error
            .to_string()
            .contains("source changed after local staff-distance authorization"));
    }
}

#[cfg(test)]
#[path = "../../tests/unit/local_staff_distance_journal.rs"]
mod local_staff_distance_journal_tests;
