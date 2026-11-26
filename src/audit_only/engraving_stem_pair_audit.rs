//! Synthetic measurements from one controlled native engraving tree.

use super::{
    InkRelation, MarkedNativeScore, Page, Shape, binding, child, ink_relation, paint_value,
    possible_contact,
};
use roxmltree::{Document, Node};
use std::fmt::Write as _;

#[derive(Debug, PartialEq, Eq)]
enum Observation {
    Clear,
    Overlap,
    Unknown,
}

fn pair_observation(stem: &Shape, neighbour: &Shape, minimum: f64) -> Observation {
    if !possible_contact(stem, neighbour, minimum) {
        Observation::Clear
    } else if ink_relation(stem, neighbour) == InkRelation::Overlap {
        Observation::Overlap
    } else {
        Observation::Unknown
    }
}

pub(super) fn marked_shape<'a>(
    plain: &'a Page,
    painted: &Page,
    native: &MarkedNativeScore,
    color: &str,
) -> Option<&'a Shape> {
    if plain.format != painted.format || plain.shapes.len() != painted.shapes.len() {
        return None;
    }
    let expected = native.marks.get(color)?;
    let mut found = None;
    for (shape, tagged) in plain.shapes.iter().zip(&painted.shapes) {
        let mark = binding(shape, tagged, &native.marks)?;
        let tagged_color = ["fill", "stroke"].iter().any(|ink| {
            !paint_value(shape, ink).eq_ignore_ascii_case(color)
                && paint_value(tagged, ink).eq_ignore_ascii_case(color)
        });
        if tagged_color {
            if mark.as_ref() != Some(expected) || found.is_some() {
                return None;
            }
            found = Some(shape);
        }
    }
    found
}

fn native_color(shape: Node<'_, '_>) -> String {
    let color = child(shape, "color").unwrap();
    let byte = |name| color.attribute(name).unwrap().parse::<u8>().unwrap();
    format!("#{:02x}{:02x}{:02x}", byte("r"), byte("g"), byte("b"))
}

/// Both native scores have the same object tree, apart from one stem length.
/// The caller verifies this before adding any paint.
pub(crate) fn assert_native_pairs(
    before: &MarkedNativeScore,
    after: &MarkedNativeScore,
    old: [&str; 2],
    new: [&str; 2],
) {
    assert_eq!(before.marks, after.marks);
    let authored = Document::parse(&before.xml).unwrap();
    let mut chords = authored
        .descendants()
        .filter(|node| node.has_tag_name("Chord"));
    let upper = chords.next().unwrap();
    let lower = chords.nth(1).unwrap();
    let head = child(upper, "Note").unwrap();
    let tie = child(child(head, "Spanner").unwrap(), "Tie").unwrap();
    let stem_color = native_color(child(lower, "Stem").unwrap());
    let old_plain = Page::read(old[0]).unwrap();
    let old_painted = Page::read(old[1]).unwrap();
    let new_plain = Page::read(new[0]).unwrap();
    let new_painted = Page::read(new[1]).unwrap();
    assert_eq!(old_plain.format, new_plain.format);
    let old_stem = marked_shape(&old_plain, &old_painted, before, &stem_color).unwrap();
    let new_stem = marked_shape(&new_plain, &new_painted, after, &stem_color).unwrap();
    assert!(old_stem.commands != new_stem.commands);

    for neighbour in [head, tie] {
        let color = native_color(neighbour);
        let old_mark = marked_shape(&old_plain, &old_painted, before, &color).unwrap();
        let new_mark = marked_shape(&new_plain, &new_painted, after, &color).unwrap();
        // The fixture places the foreign marks on a different staff. This
        // experimental SVG distance is not a musical threshold.
        assert_eq!(
            pair_observation(old_stem, old_mark, 1.0),
            Observation::Clear
        );
        assert_eq!(
            pair_observation(new_stem, new_mark, 1.0),
            Observation::Clear
        );
    }
}

fn outline(class: &str, color: &str, left: i32) -> String {
    let mut shape = String::new();
    let right = left + 6;
    write!(
        shape,
        "<path class=\"{class}\" fill=\"{color}\" d=\"M{left},0 L{right},0 L{right},8 L{left},8 L{left},0\"/>"
    )
    .unwrap();
    shape
}

fn frame(ink: &str) -> Page {
    Page::read(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\">{ink}</svg>"
    ))
    .unwrap()
}

#[test]
fn foreign_stem_pair_measures_clear_overlap_and_unresolved_clearance() {
    let stem = frame(&outline("Stem", "#000000", 0));
    let minimum = 1.0;
    for class in ["Note", "TieSegment"] {
        for (left, expected) in [
            (20, Observation::Clear),
            (2, Observation::Overlap),
            (6, Observation::Unknown),
            (7, Observation::Unknown),
        ] {
            let foreign = frame(&outline(class, "#000000", left));
            assert_eq!(
                pair_observation(&stem.shapes[0], &foreign.shapes[0], minimum),
                expected,
                "{class} at {left}"
            );
        }
    }
}

#[test]
fn missing_duplicate_and_geometry_changing_paint_do_not_bind() {
    let color = "#310001";
    let plain_path = outline("TieSegment", "#000000", 3);
    let painted_path = outline("TieSegment", color, 3);
    let native = MarkedNativeScore {
        xml: String::new(),
        marks: [(color.to_owned(), super::Mark::Tie)].into(),
    };
    let plain = frame(&plain_path);
    assert!(marked_shape(&plain, &frame(&painted_path), &native, color).is_some());
    assert!(marked_shape(&plain, &plain, &native, color).is_none());
    assert!(
        marked_shape(
            &frame(&plain_path.repeat(2)),
            &frame(&painted_path.repeat(2)),
            &native,
            color
        )
        .is_none()
    );
    assert!(
        marked_shape(
            &plain,
            &frame(&outline("TieSegment", color, 4)),
            &native,
            color
        )
        .is_none()
    );
}
