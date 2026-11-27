//! Geometry for the qualified native stem-transition experiment.
//!
//! The caller proves source identities, complete page inventories and PDF
//! readback separately. This module sees only the corresponding SVG ink.

use super::{
    Bounds, CONTOUR_ERROR, GRID, PREVIEW_RESERVE, Page, Point, Shape, Side, contains, edges,
    point_near_segment, possible_contact,
};

const INK_RESERVE: f64 = CONTOUR_ERROR + PREVIEW_RESERVE;

#[derive(Clone, Copy)]
pub(super) struct StemObservation<'a> {
    pub(super) side: Side,
    page: &'a Page,
    shape: &'a Shape,
    head: &'a Shape,
    stroke_width: f64,
    staff_space: f64,
}

fn page_contains(page: &Page, wanted: &Shape) -> bool {
    page.shapes.iter().any(|shape| std::ptr::eq(shape, wanted))
}

fn visible_rgb(shape: &Shape, kind: &str, default: &str) -> bool {
    let paint = shape
        .paint
        .iter()
        .find_map(|(name, value)| (name == kind).then_some(value.as_str()))
        .unwrap_or(default);
    paint
        .strip_prefix('#')
        .is_some_and(|value| value.len() == 6 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

fn notehead_bounds(head: &Shape) -> Option<Bounds> {
    Bounds::from_points(
        head.contours.iter().flatten().map(|point| Point {
            x: point.x as f64 / GRID,
            y: point.y as f64 / GRID,
        }),
        0.0,
    )
}

fn find_staff_space(page: &Page, middle: Point) -> Option<f64> {
    let mut lines = page
        .shapes
        .iter()
        .filter(|shape| shape.class == "StaffLines")
        .map(|shape| {
            let (a, b) = shape.extent()?;
            (a.y == b.y && a.x < b.x).then_some((a, b))
        })
        .collect::<Option<Vec<_>>>()?;
    lines.sort_by(|a, b| a.0.y.total_cmp(&b.0.y));
    let (staves, incomplete) = lines.as_chunks::<5>();
    if !incomplete.is_empty() {
        return None;
    }

    let mut scales = staves.iter().filter_map(|staff| {
        let (start, end) = staff[0];
        let space = (staff[4].0.y - start.y) / 4.0;
        let contains_head = middle.x > start.x + INK_RESERVE
            && middle.x < end.x - INK_RESERVE
            && middle.y > start.y - space + INK_RESERVE
            && middle.y < staff[4].0.y + space - INK_RESERVE;
        let regular = staff.iter().enumerate().all(|(i, (a, b))| {
            a.x == start.x
                && b.x == end.x
                && (a.y - (start.y + i as f64 * space)).abs() < INK_RESERVE
        });
        (space > 2.0 * INK_RESERVE && contains_head && regular).then_some(space)
    });
    let space = scales.next()?;
    scales.next().is_none().then_some(space)
}

fn solid_butt_stem(stem: &Shape) -> Option<(Point, Point, f64)> {
    let (a, b) = stem.extent()?;
    let width = stem.stroke_width();
    if stem.class != "Stem"
        || a.x != b.x
        || a.y == b.y
        || !visible_rgb(stem, "stroke", "none")
        || stem.stroke_scale[0] != stem.stroke_scale[1]
        || !stem.filled
        || stem.radius != 0.0
        || stem.contours.len() != 1
        || width <= 4.0 * INK_RESERVE
    {
        return None;
    }
    let (top, bottom) = if a.y < b.y { (a, b) } else { (b, a) };
    Some((top, bottom, width))
}

fn attachment(
    head: &Shape,
    top: Point,
    bottom: Point,
    middle: Point,
    bounds: Bounds,
    width: f64,
) -> Option<Side> {
    // The distal end extends past the head. The entire possible contact is
    // in the selected corner of the head's bounds, regardless of its shape.
    let half_width = width / 2.0;
    let (side, origin, dy) = if top.y < bounds.top - INK_RESERVE
        && bottom.y < middle.y - INK_RESERVE
        && bottom.x - half_width > middle.x + INK_RESERVE
    {
        (Side::Above, bottom, -1.0)
    } else if bottom.y > bounds.bottom + INK_RESERVE
        && top.y > middle.y + INK_RESERVE
        && top.x + half_width < middle.x - INK_RESERVE
    {
        (Side::Below, top, 1.0)
    } else {
        return None;
    };
    // A point in the interior of both marks proves a real attachment, not
    // merely touching envelopes. The guard covers rounded vector geometry.
    let sample = Point {
        x: origin.x,
        y: origin.y + dy * width / 4.0,
    }
    .grid()?;
    let guard = (INK_RESERVE * GRID).ceil() as i128;
    (contains(head, sample)
        && edges(head)
            .all(|edge| point_near_segment(sample, edge[0], edge[1], guard.pow(2)) == Some(false)))
    .then_some(side)
}

pub(super) fn observe_stem<'a>(
    page: &'a Page,
    stem: &'a Shape,
    own_head: &'a Shape,
) -> Option<StemObservation<'a>> {
    if !page_contains(page, stem)
        || !page_contains(page, own_head)
        || own_head.class != "Note"
        || !own_head.filled
        || own_head.stroke_width() != 0.0
        || !visible_rgb(own_head, "fill", "#000000")
    {
        return None;
    }
    let (top, bottom, width) = solid_butt_stem(stem)?;
    let bounds = notehead_bounds(own_head)?;
    let middle = Point {
        x: (bounds.left + bounds.right) / 2.0,
        y: (bounds.top + bounds.bottom) / 2.0,
    };
    let staff_space = find_staff_space(page, middle)?;
    if width >= staff_space || bottom.y - top.y <= width {
        return None;
    }
    let side = attachment(own_head, top, bottom, middle, bounds, width)?;
    Some(StemObservation {
        side,
        page,
        shape: stem,
        head: own_head,
        stroke_width: width,
        staff_space,
    })
}

pub(super) fn proves_direction_transition(
    stems: &[(StemObservation<'_>, StemObservation<'_>)],
    target: Side,
) -> bool {
    stems.iter().all(|(_, after)| after.side == target)
        && stems.iter().any(|(before, _)| before.side != target)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PairRole {
    Attachment,
    ForeignHead,
    Tie,
}

type Pair<'a> = (StemObservation<'a>, &'a Shape);

fn pair_role((stem, neighbour): Pair<'_>) -> Option<PairRole> {
    if !page_contains(stem.page, neighbour) {
        return None;
    }
    match neighbour.class.as_str() {
        "Note" if std::ptr::eq(stem.head, neighbour) => Some(PairRole::Attachment),
        "Note" => Some(PairRole::ForeignHead),
        "TieSegment" => Some(PairRole::Tie),
        _ => None,
    }
}

fn same_bound_ink(a: &Shape, b: &Shape) -> bool {
    a.same_ink(b) && a.filled == b.filled && a.even_odd == b.even_odd
}

/// An absent pair means its separately proven source marks do not share a
/// page in that frame. A changed foreign pair needs absolute clearance. The
/// required gap is one ordinary rendered stem width normalized by the local
/// staff space. This separates two distinct strokes by at least one stroke
/// of white space. `possible_contact` adds the contour and export reserves.
pub(super) fn preserves_pair(before: Option<Pair<'_>>, after: Option<Pair<'_>>) -> bool {
    let Some(after) = after else {
        return before.is_some_and(|before| pair_role(before).is_some());
    };
    let Some(role) = pair_role(after) else {
        return false;
    };
    let (stem, neighbour) = after;
    let mut normalized_gap = stem.stroke_width / stem.staff_space;
    if let Some(before) = before {
        let (old_stem, old_neighbour) = before;
        if pair_role(before) != Some(role) {
            return false;
        }
        normalized_gap = normalized_gap.max(old_stem.stroke_width / old_stem.staff_space);
        if old_stem.staff_space == stem.staff_space
            && same_bound_ink(old_stem.shape, stem.shape)
            && same_bound_ink(old_neighbour, neighbour)
        {
            return true;
        }
    }
    role == PairRole::Attachment
        || !possible_contact(stem.shape, neighbour, normalized_gap * stem.staff_space)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write as _;

    const HEAD: &str = r#"<path class="Note" d="M25,40 L53,40 L53,51 L25,51 L25,40"/>"#;
    const UP: &str = r##"<polyline class="Stem" stroke="#000000" stroke-width="2" fill="none" points="51,43 51,10"/>"##;
    const DOWN: &str = r##"<polyline class="Stem" stroke="#000000" stroke-width="2" fill="none" points="27,48 27,80"/>"##;
    const FAR_HEAD: &str = r#"<path class="Note" d="M70,12 L80,12 L80,20 L70,20 L70,12"/>"#;
    const NEAR_HEAD: &str = r#"<path class="Note" d="M40,12 L49.7,12 L49.7,20 L40,20 L40,12"/>"#;
    const NEAR_TIE: &str =
        r#"<path class="TieSegment" d="M40,12 L49.7,12 L49.7,20 L40,20 L40,12"/>"#;

    fn frame(stem: &str, neighbour: &str, step: usize) -> Page {
        let mut svg = r#"<svg xmlns="http://www.w3.org/2000/svg">"#.to_owned();
        svg.push_str(HEAD);
        svg.push_str(stem);
        svg.push_str(neighbour);
        for y in (1..=5).map(|index| index * step) {
            write!(svg, r##"<polyline class="StaffLines" stroke="#000000" stroke-width="1" fill="none" points="0,{y} 120,{y}"/>"##).unwrap();
        }
        svg.push_str("</svg>");
        Page::read(&svg).unwrap()
    }

    fn observation(page: &Page) -> Option<StemObservation<'_>> {
        observe_stem(page, &page.shapes[1], &page.shapes[0])
    }

    fn pair(page: &Page, index: usize) -> Pair<'_> {
        (observation(page).unwrap(), &page.shapes[index])
    }

    #[test]
    fn signed_stems_are_bound_by_their_actual_attachment() {
        let down = frame(DOWN, "", 20);
        let up = frame(UP, "", 20);
        let detached = frame(&UP.replace("51,43", "51,38"), "", 20);
        let wrong_half = frame(&UP.replace("51,", "27,"), "", 20);
        let old = observation(&down).unwrap();
        let new = observation(&up).unwrap();
        assert!(proves_direction_transition(&[(old, new)], Side::Above));
        assert!(!proves_direction_transition(&[(new, new)], Side::Above));
        assert!(!proves_direction_transition(
            &[(old, new), (old, old)],
            Side::Above
        ));
        assert!(!proves_direction_transition(&[], Side::Above));
        assert!(observation(&detached).is_none());
        assert!(observation(&wrong_half).is_none());
        assert!(observe_stem(&down, &down.shapes[1], &up.shapes[0]).is_none());
    }

    #[test]
    fn changed_foreign_pairs_need_clearance_without_any_overlap_guess() {
        let old = frame(DOWN, FAR_HEAD, 20);
        let clear = frame(UP, FAR_HEAD, 20);
        let near = frame(UP, NEAR_HEAD, 20);
        let tie = frame(UP, NEAR_TIE, 20);
        let resized = frame(UP, NEAR_HEAD, 22);
        let prior = Some(pair(&old, 2));
        assert!(preserves_pair(prior, Some(pair(&clear, 2))));
        assert!(!preserves_pair(prior, Some(pair(&near, 2))));
        assert!(!preserves_pair(None, Some(pair(&tie, 2))));
        assert!(preserves_pair(Some(pair(&near, 2)), Some(pair(&near, 2))));
        assert!(!preserves_pair(
            Some(pair(&near, 2)),
            Some(pair(&resized, 2))
        ));
        assert!(preserves_pair(prior, None));
        assert!(!preserves_pair(None, None));
        assert!(!preserves_pair(
            prior,
            Some((observation(&near).unwrap(), &old.shapes[2]))
        ));
        assert!(preserves_pair(Some(pair(&old, 0)), Some(pair(&clear, 0))));
    }
}
