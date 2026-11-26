//! Evidence limited to the native score bytes and visible staff/stem strokes.
//! The proof cannot select a variant or authorize any product write.

use super::renderer_system_break_readback::extract_pdf_vector_geometry;
use crate::pipeline::svg_notation_geometry::Page;
use roxmltree::{Document, Node};

const PDF_POINT_RESERVE: f64 = 0.015;
const NATIVE_CHORD_PATH: [&str; 6] = ["Chord", "voice", "Measure", "Staff", "Score", "museScore"];

fn meaningful(node: &Node<'_, '_>) -> bool {
    node.is_element() || node.text().is_some_and(|text| !text.trim().is_empty())
}

fn scalar_stem<'a, 'input>(chord: Node<'a, 'input>) -> Option<Option<(Node<'a, 'input>, f64)>> {
    let mut stems = chord.children().filter(|child| child.has_tag_name("Stem"));
    let Some(stem) = stems.next() else {
        return Some(None);
    };
    if stems.next().is_some()
        || stem.attributes().len() != 0
        || stem.tag_name().namespace().is_some()
    {
        return None;
    }
    let mut children = stem.children().filter(meaningful);
    let Some(length) = children.next() else {
        return Some(Some((stem, 0.0)));
    };
    if !length.has_tag_name("userLen")
        || length.tag_name().namespace().is_some()
        || length.attributes().len() != 0
        || length.children().any(|child| !child.is_text())
        || length.children().count() != 1
        || children.next().is_some()
    {
        return None;
    }
    let value = length.text()?.trim().parse::<f64>().ok()?;
    value.is_finite().then_some(Some((stem, value)))
}

fn same_tree(before: Node<'_, '_>, after: Node<'_, '_>, edits: &mut usize) -> bool {
    if before.node_type() != after.node_type() {
        return false;
    }
    if !before.is_element() {
        return before.text() == after.text();
    }
    if before.tag_name() != after.tag_name()
        || before.attributes().len() != after.attributes().len()
        || before.attributes().any(|attribute| {
            !after.attributes().any(|other| {
                attribute.name() == other.name()
                    && attribute.namespace() == other.namespace()
                    && attribute.value() == other.value()
            })
        })
    {
        return false;
    }

    let mut skip_before = None;
    let mut skip_after = None;
    if before
        .ancestors()
        .filter(Node::is_element)
        .map(|node| node.tag_name().name())
        .eq(NATIVE_CHORD_PATH)
        && let (Some(old), Some(new)) = (scalar_stem(before), scalar_stem(after))
    {
        let old_length = old.map_or(0.0, |(_, value)| value);
        let new_length = new.map_or(0.0, |(_, value)| value);
        *edits += usize::from(old_length != new_length);
        skip_before = old.map(|(node, _)| node);
        skip_after = new.map(|(node, _)| node);
    }

    let mut before = before
        .children()
        .filter(meaningful)
        .filter(|node| Some(*node) != skip_before);
    let mut after = after
        .children()
        .filter(meaningful)
        .filter(|node| Some(*node) != skip_after);
    loop {
        match (before.next(), after.next()) {
            (Some(before), Some(after)) if same_tree(before, after, edits) => {}
            (None, None) => return true,
            _ => return false,
        }
    }
}

/// Every non-stem field and its order must agree. This validates a variant of
/// one native import and makes no assertion about a separate MusicXML import.
pub(crate) fn stem_length_edits(before: &str, after: &str) -> Option<usize> {
    let old = Document::parse(before).ok()?;
    let original = old.root_element();
    if !original.has_tag_name("museScore") || original.tag_name().namespace().is_some() {
        return None;
    }
    if before == after {
        return Some(0);
    }
    let new = Document::parse(after).ok()?;
    let mut count = 0;
    same_tree(original, new.root_element(), &mut count).then_some(count)
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() <= PDF_POINT_RESERVE
}

fn staff_segments(anchor: &[f64; 5], printed: &[[f64; 5]]) -> Option<Vec<usize>> {
    let [left, y, right, _, width] = *anchor;
    let mut segments = printed
        .iter()
        .enumerate()
        .filter(|(_, line)| {
            near(y, line[1])
                && near(y, line[3])
                && near(width, line[4])
                && line[0] >= left - PDF_POINT_RESERVE
                && line[2] <= right + PDF_POINT_RESERVE
                && line[2] - line[0] > 2.0 * PDF_POINT_RESERVE
        })
        .collect::<Vec<_>>();
    segments.sort_unstable_by(|(_, a), (_, b)| a[0].total_cmp(&b[0]));
    let mut end = left;
    for (_, line) in &segments {
        if !near(end, line[0]) {
            return None;
        }
        end = line[2];
    }
    (!segments.is_empty() && near(end, right))
        .then(|| segments.into_iter().map(|(index, _)| index).collect())
}

fn matching_strokes(expected: &[[f64; 5]], printed: &[[f64; 5]]) -> bool {
    let mut used = vec![false; printed.len()];
    for anchor in expected {
        let support = if anchor[1] == anchor[3] {
            // PDF can emit a staff line one measure at a time. Bind the
            // entire continuous span, with no gap or overlapping alternative.
            staff_segments(anchor, printed)
        } else {
            let mut matches = printed
                .iter()
                .enumerate()
                .filter(|(_, stroke)| anchor.iter().zip(*stroke).all(|(a, b)| near(*a, *b)));
            match (matches.next(), matches.next()) {
                (Some((index, _)), None) => Some(vec![index]),
                _ => None,
            }
        };
        if support.is_none_or(|indices| {
            indices
                .into_iter()
                .any(|index| std::mem::replace(&mut used[index], true))
        }) {
            return false;
        }
    }
    true
}

pub(crate) fn verify_strokes(previews: &[String], pdf: &[u8]) -> Result<usize, &'static str> {
    let (strokes, _) = extract_pdf_vector_geometry(pdf).map_err(|_| "pdf_strokes_unavailable")?;
    if strokes.iter().map(|stroke| stroke.page_index).max() != Some(previews.len()) {
        return Err("pdf_stroke_pages_mismatch");
    }
    let mut count = 0;
    for (index, svg) in previews.iter().enumerate() {
        let expected = Page::read(svg)
            .and_then(|page| page.native_export_stroke_anchors())
            .ok_or("svg_strokes_unavailable")?;
        if expected.iter().all(|line| line[0] != line[2])
            || expected.iter().all(|line| line[1] != line[3])
        {
            return Err("staff_or_stem_strokes_unavailable");
        }
        let printed = strokes
            .iter()
            .filter(|s| s.page_index == index + 1)
            .map(|s| {
                let (start, end) = ((s.x1, s.y1_top), (s.x2, s.y2_top));
                let (a, b) = if start <= end {
                    (start, end)
                } else {
                    (end, start)
                };
                [a.0, a.1, b.0, b.1, s.width]
            })
            .collect::<Vec<_>>();
        if !matching_strokes(&expected, &printed) {
            return Err("svg_pdf_stroke_mismatch");
        }
        count += expected.len();
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NATIVE: &str = "<museScore><Score><Staff id='1'><Measure><voice>        <Chord><durationType>16th</durationType><Note><pitch>65</pitch><Spanner><Tie/></Spanner></Note></Chord>        <Rest><durationType>eighth</durationType></Rest></voice></Measure></Staff></Score></museScore>";

    #[test]
    fn a_stem_length_is_the_only_editable_field() {
        let grown = NATIVE.replace("</Chord>", "<Stem><userLen>1.5</userLen></Stem></Chord>");
        assert_eq!(stem_length_edits(NATIVE, &grown), Some(1));
        assert_eq!(stem_length_edits(&grown, NATIVE), Some(1));
        assert_eq!(stem_length_edits(&grown, &grown), Some(0));
        for (old, changed) in [
            ("65", "67"),
            ("16th", "quarter"),
            ("eighth", "half"),
            ("id='1'", "id='2'"),
            ("<Tie/>", "<Slur/>"),
            ("<voice>", "<voice id='2'>"),
            ("<Stem>", "<Stem visible='0'>"),
            ("1.5", "NaN"),
        ] {
            assert_eq!(
                stem_length_edits(NATIVE, &grown.replace(old, changed)),
                None,
                "{old}"
            );
        }
        assert_eq!(
            stem_length_edits(NATIVE, &grown.replace("<Chord>", "<Chord><Stem/>")),
            None
        );
    }

    #[test]
    fn stroke_evidence_rejects_drift_and_ambiguous_assignment() {
        let visible = [21.0, 4.0, 21.0, 17.0, 0.5];
        assert!(matching_strokes(&[visible], &[visible]));
        assert!(!matching_strokes(&[visible], &[visible, visible]));
        assert!(!matching_strokes(&[visible, visible], &[visible]));
        let mut shifted = visible;
        shifted[3] += 0.02;
        assert!(!matching_strokes(&[visible], &[shifted]));
    }

    #[test]
    fn a_staff_line_can_bind_to_contiguous_pdf_segments_only() {
        let line = [4.0, 8.0, 40.0, 8.0, 0.55];
        let left = [4.0, 8.0, 24.0, 8.0, 0.55];
        let right = [24.002, 8.0, 40.0, 8.0, 0.55];
        assert!(matching_strokes(&[line], &[right, left]));
        assert!(!matching_strokes(&[line], &[right, left, line]));
        let mut gap = right;
        gap[0] += 0.02;
        assert!(!matching_strokes(&[line], &[left, gap]));
    }
}
