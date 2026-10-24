//! Central, source-bound insertion of global visual layout defaults.
//!
//! The layout pass may provide numeric system and staff distances. This module
//! receives no project identity, measure number, pitch, staff or voice identity.
//! Authorization is bound to the complete XML source, the exact insertion
//! boundary and the complete fixed-shape defaults fragment. The writer performs
//! exactly one `insert_str` mutation and never writes a file.
//! Paper geometry is a fixed A4 product setting, independent of input identity
//! and renderer defaults. Existing contradictory geometry is refused.

use super::authority_primitives::fingerprint as source_fingerprint;
use crate::master_state_identity::PreservingTransformationReceipt;
use anyhow::{Result, bail};

const MARKER: &str = "<!-- etuschlopf-visual-layout-v120 -->";

// Native MuseScore 4.7.4 export: 7 mm / 40 tenths, nominal A4,
// 15 mm margins. Only scaling and page layout are adopted.
const A4_GEOMETRY: &str = "<scaling><millimeters>7</millimeters><tenths>40</tenths></scaling>\n    <page-layout><page-height>1696.72</page-height><page-width>1200.33</page-width><page-margins type=\"both\"><left-margin>85.7143</left-margin><right-margin>85.7143</right-margin><top-margin>85.7143</top-margin><bottom-margin>85.7143</bottom-margin></page-margins></page-layout>";

fn existing_page_geometry(xml: &str) -> Result<bool> {
    let document = roxmltree::Document::parse(xml)?;
    let defaults = document
        .root_element()
        .children()
        .find(|node| node.has_tag_name("defaults"));
    let Some(defaults) = defaults else {
        return Ok(false);
    };
    let scales = defaults
        .children()
        .filter(|node| node.has_tag_name("scaling"))
        .collect::<Vec<_>>();
    let pages = defaults
        .children()
        .filter(|node| node.has_tag_name("page-layout"))
        .collect::<Vec<_>>();
    if scales.is_empty() && pages.is_empty() {
        return Ok(false);
    }
    if scales.len() != 1 || pages.len() != 1 {
        bail!("A4 defaults require one complete scaling and page-layout pair");
    }
    let value = |node: roxmltree::Node<'_, '_>, name: &str| -> Result<f64> {
        let nodes = node
            .children()
            .filter(|child| child.has_tag_name(name))
            .collect::<Vec<_>>();
        if nodes.len() != 1 {
            bail!("A4 defaults require one {name}");
        }
        Ok(nodes[0].text().unwrap_or("").parse::<f64>()?)
    };
    if value(scales[0], "millimeters")? != 7.0
        || value(scales[0], "tenths")? != 40.0
        || value(pages[0], "page-height")? != 1696.72
        || value(pages[0], "page-width")? != 1200.33
    {
        bail!("Existing page geometry contradicts the fixed A4 product setting");
    }
    let margins = pages[0]
        .children()
        .filter(|node| node.has_tag_name("page-margins"))
        .collect::<Vec<_>>();
    let kinds = margins
        .iter()
        .map(|node| node.attribute("type").unwrap_or("both"))
        .collect::<Vec<_>>();
    if kinds != ["both"] && kinds != ["even", "odd"] && kinds != ["odd", "even"] {
        bail!("A4 margins must cover both page parities exactly once");
    }
    for margin in margins {
        for name in ["left-margin", "right-margin", "top-margin", "bottom-margin"] {
            if value(margin, name)? != 85.7143 {
                bail!("Existing {name} contradicts fixed A4 margins");
            }
        }
    }
    Ok(true)
}

#[derive(Debug)]
struct AuthorizedVisualLayoutDefaultsInsertion {
    source_fingerprint: u64,
    insertion_offset: usize,
    insertion: String,
    insertion_fingerprint: u64,
    system_distance: usize,
    staff_distance: usize,
}

impl AuthorizedVisualLayoutDefaultsInsertion {
    fn issue(
        source_xml: &str,
        system_distance: usize,
        staff_distance: usize,
    ) -> Result<Option<Self>> {
        let complete_geometry = existing_page_geometry(source_xml)?;
        if source_xml.contains(MARKER) && !complete_geometry {
            bail!("Marked visual defaults lack the fixed A4 geometry");
        }
        let Some((insertion_offset, insertion)) =
            plan_visual_layout_defaults_like_legacy(source_xml, system_distance, staff_distance)
        else {
            return Ok(None);
        };
        verify_insertion_boundary(source_xml, insertion_offset)?;
        Ok(Some(Self {
            source_fingerprint: source_fingerprint(source_xml),
            insertion_offset,
            insertion_fingerprint: source_fingerprint(&insertion),
            insertion,
            system_distance,
            staff_distance,
        }))
    }
}

pub fn ensure_visual_layout_defaults_authorized(
    xml: &mut String,
    system_distance: usize,
    staff_distance: usize,
) -> (Result<bool>, Option<PreservingTransformationReceipt>) {
    let token = match AuthorizedVisualLayoutDefaultsInsertion::issue(
        xml,
        system_distance,
        staff_distance,
    ) {
        Ok(Some(token)) => token,
        Ok(None) => return (Ok(false), None),
        Err(error) => return (Err(error), None),
    };
    let (written, receipt) = authorized_visual_layout_defaults_writer(xml, token);
    (written.map(|_| true), receipt)
}

fn authorized_visual_layout_defaults_writer(
    xml: &mut String,
    token: AuthorizedVisualLayoutDefaultsInsertion,
) -> (Result<usize>, Option<PreservingTransformationReceipt>) {
    if let Err(error) = verify_visual_layout_defaults_token(xml, &token) {
        return (Err(error), None);
    }
    let pre_xml = xml.clone();
    xml.insert_str(token.insertion_offset, &token.insertion);
    // PRESERVING receipt E044, minted by this owner only if the conserved
    // musical content projection is unchanged.
    let receipt = PreservingTransformationReceipt::mint_preserving_receipt(
        VisualLayoutDefaultsPreservingMint { _sealed: () },
        &pre_xml,
        xml,
    );
    // The token's one insertion is this owner's own operation journal.
    let journal = crate::arrangement::representation_transport::RepresentationStep::recorded(
        &pre_xml,
        xml,
        vec![(
            token.insertion_offset..token.insertion_offset,
            token.insertion,
        )],
    );
    (
        Ok(1),
        receipt.map(|receipt| receipt.with_layout_journal(Some(journal))),
    )
}

/// Sealed mint capability of `authorized_visual_layout_defaults_writer`
/// (PRESERVING E044).
pub(crate) struct VisualLayoutDefaultsPreservingMint {
    _sealed: (),
}

fn verify_visual_layout_defaults_token(
    xml: &str,
    token: &AuthorizedVisualLayoutDefaultsInsertion,
) -> Result<()> {
    if source_fingerprint(xml) != token.source_fingerprint {
        bail!("source changed after visual layout defaults authorization");
    }
    verify_insertion_boundary(xml, token.insertion_offset)?;
    let Some((expected_offset, expected_insertion)) =
        plan_visual_layout_defaults_like_legacy(xml, token.system_distance, token.staff_distance)
    else {
        bail!("authorized visual layout defaults insertion became a no-op");
    };
    if expected_offset != token.insertion_offset
        || expected_insertion != token.insertion
        || source_fingerprint(&expected_insertion) != token.insertion_fingerprint
    {
        bail!("visual layout defaults delta changed after authorization");
    }
    Ok(())
}

fn plan_visual_layout_defaults_like_legacy(
    xml: &str,
    system_distance: usize,
    staff_distance: usize,
) -> Option<(usize, String)> {
    if xml.contains(MARKER) {
        return None;
    }
    let complete_geometry = existing_page_geometry(xml).ok()?;
    let geometry = if complete_geometry { "" } else { A4_GEOMETRY };
    let layout_defaults = format!(
        "\n    {}\n    {}\n    <system-layout>\n      <system-margins><left-margin>0</left-margin><right-margin>0</right-margin></system-margins>\n      <system-distance>{}</system-distance>\n      <top-system-distance>{}</top-system-distance>\n    </system-layout>\n    <staff-layout number=\"2\"><staff-distance>{}</staff-distance></staff-layout>",
        MARKER,
        geometry,
        system_distance,
        system_distance.saturating_sub(20),
        staff_distance,
    );
    if let Some(defaults_start) = xml.find("<defaults>") {
        let offset = if complete_geometry {
            xml.find("</page-layout>")? + "</page-layout>".len()
        } else {
            defaults_start + "<defaults>".len()
        };
        return Some((offset, layout_defaults));
    }
    let defaults = format!("\n  <defaults>{layout_defaults}\n  </defaults>");
    xml.find("<part-list>")
        .map(|part_list_start| (part_list_start, defaults))
}

fn verify_insertion_boundary(xml: &str, offset: usize) -> Result<()> {
    if offset > xml.len() || !xml.is_char_boundary(offset) {
        bail!("visual layout defaults insertion offset is invalid");
    }
    let suffix = &xml[offset..];
    if !suffix.starts_with("</defaults>")
        && !suffix.starts_with("<part-list>")
        && !xml[..offset].ends_with("<defaults>")
        && !xml[..offset].ends_with("</page-layout>")
    {
        bail!("visual layout defaults insertion is not bound to an allowed XML boundary");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        AuthorizedVisualLayoutDefaultsInsertion, MARKER, authorized_visual_layout_defaults_writer,
        ensure_visual_layout_defaults_authorized,
    };

    fn verify_a4(xml: &str) {
        let doc = roxmltree::Document::parse(xml).unwrap();
        assert_eq!(
            doc.descendants()
                .filter(|node| node.has_tag_name("scaling"))
                .count(),
            1
        );
        assert_eq!(
            doc.descendants()
                .filter(|node| node.has_tag_name("page-layout"))
                .count(),
            1
        );
        assert!(super::existing_page_geometry(xml).unwrap());
        let defaults = doc
            .root_element()
            .children()
            .find(|node| node.has_tag_name("defaults"))
            .unwrap();
        let names = defaults
            .children()
            .filter(|node| node.is_element())
            .map(|node| node.tag_name().name())
            .collect::<Vec<_>>();
        assert_eq!(
            &names[..4],
            &["scaling", "page-layout", "system-layout", "staff-layout"]
        );
    }

    #[test]
    fn fixed_a4_precedes_other_defaults_and_preserves_content() {
        let notes = "<part id=\"arbitrary\"><measure number=\"any\"><note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>7</voice><staff>1</staff></note></measure></part>";
        for defaults in ["", "<defaults><appearance/></defaults>"] {
            let mut xml = format!(
                "<score-partwise>{defaults}<part-list></part-list>{notes}</score-partwise>"
            );
            let (written, receipt) = ensure_visual_layout_defaults_authorized(&mut xml, 150, 96);
            assert!(written.unwrap());
            assert!(receipt.is_some());
            verify_a4(&xml);
            assert!(xml.contains(notes));
            let green = xml.clone();
            assert!(
                !ensure_visual_layout_defaults_authorized(&mut xml, 158, 104)
                    .0
                    .unwrap()
            );
            assert_eq!(xml, green);
        }
    }

    #[test]
    fn incomplete_or_contradictory_geometry_is_refused_without_a_write() {
        for defaults in [
            "<page-layout/>",
            "<scaling><millimeters>7</millimeters><tenths>40</tenths></scaling>",
        ] {
            let mut xml = format!(
                "<score-partwise><defaults>{defaults}</defaults><part-list></part-list></score-partwise>"
            );
            let before = xml.clone();
            let (result, receipt) = ensure_visual_layout_defaults_authorized(&mut xml, 150, 96);
            assert!(result.is_err());
            assert!(receipt.is_none());
            assert_eq!(xml, before);
        }
        let mut xml = format!(
            "<score-partwise><defaults>{}</defaults><part-list></part-list></score-partwise>",
            super::A4_GEOMETRY.replace("1200.33", "1233.71")
        );
        let before = xml.clone();
        assert!(
            ensure_visual_layout_defaults_authorized(&mut xml, 150, 96)
                .0
                .is_err()
        );
        assert_eq!(xml, before);
    }

    #[test]
    fn matching_geometry_is_not_duplicated() {
        let mut xml = format!(
            "<score-partwise><defaults>{}<appearance/></defaults><part-list></part-list></score-partwise>",
            super::A4_GEOMETRY
        );
        assert!(
            ensure_visual_layout_defaults_authorized(&mut xml, 150, 96)
                .0
                .unwrap()
        );
        verify_a4(&xml);
    }

    #[test]
    fn marker_and_missing_boundaries_remain_no_ops() {
        let mut marked = format!(
            "<score-partwise><defaults>{MARKER}{}</defaults><part-list/></score-partwise>",
            super::A4_GEOMETRY
        );
        let marked_before = marked.clone();
        assert!(
            !ensure_visual_layout_defaults_authorized(&mut marked, 150, 96)
                .0
                .expect("marked source no-op")
        );
        assert_eq!(marked, marked_before);

        let mut incomplete = "<score-partwise/>".to_string();
        let incomplete_before = incomplete.clone();
        assert!(
            !ensure_visual_layout_defaults_authorized(&mut incomplete, 150, 96)
                .0
                .expect("missing boundary no-op")
        );
        assert_eq!(incomplete, incomplete_before);
    }

    #[test]
    fn changed_source_is_rejected_after_authorization() {
        let source = "<score-partwise><part-list></part-list></score-partwise>";
        let token = AuthorizedVisualLayoutDefaultsInsertion::issue(source, 150, 96)
            .expect("authorization")
            .expect("write token");
        let mut changed = source.replace("<score-partwise>", "<score-partwise version=\"4.0\">");
        assert!(
            authorized_visual_layout_defaults_writer(&mut changed, token)
                .0
                .is_err()
        );
    }

    #[test]
    fn independent_documents_keep_the_same_decision_when_duplicated() {
        let source = "<score-partwise><part-list></part-list></score-partwise>";
        let mut first = source.to_string();
        let mut independent = source.to_string();
        assert!(ensure_visual_layout_defaults_authorized(&mut first, 150, 96).0.unwrap());
        assert!(ensure_visual_layout_defaults_authorized(&mut independent, 150, 96).0.unwrap());
        assert_eq!(first, independent);
        let previous = first.clone();
        let (written, receipt) = ensure_visual_layout_defaults_authorized(&mut first, 150, 96);
        assert!(!written.unwrap());
        assert!(receipt.is_none());
        assert_eq!(first, previous);
    }

    #[test]
    fn token_is_private_move_only_and_writer_is_token_only() {
        let source = include_str!("visual_layout_defaults.rs");
        let production = source.split("#[cfg(test)]").next().unwrap_or(source);
        assert!(production.contains("struct AuthorizedVisualLayoutDefaultsInsertion"));
        assert!(!production.contains("#[derive(Clone"));
        assert!(!production.contains("#[derive(Copy"));
        assert_eq!(production.matches("xml.insert_str(").count(), 1);
        assert_eq!(production.matches("std::fs::write").count(), 0);
        assert_eq!(production.matches("fs::write").count(), 0);
    }
}
