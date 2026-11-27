//! Contour and attachment proof for the qualified MuseScore export profile.

use std::collections::BTreeSet;

use anyhow::{Context, Result, ensure};

use super::super::{Bounds, CONTOUR_ERROR, GRID, Page, Point, Side, number};
use crate::audit_only::renderer_system_break_readback::pdf_source_anchor_audit::{
    PdfHeadInk, PdfHeadPage, PdfStemInk, probe_head_pages,
};

const MAX_POINTS: usize = 2048;
const MAX_CONTOURS: usize = 4;
const POLYLINE_ERROR: f64 = 0.0005;
const SAMPLE_STEP: f64 = 0.025;
const ROUNDING_RESERVE: f64 = 0.000_001;
const CONTOUR_MARGIN: f64 = 0.05;

pub(crate) struct Attachment {
    pub(crate) head: [u8; 3],
    pub(crate) stem: [u8; 3],
    pub(crate) side: Side,
}

struct SvgHead {
    mark: [u8; 3],
    outlines: Vec<Vec<Point>>,
}

struct SvgHeads {
    width: f64,
    height: f64,
    contour_error: f64,
    heads: Vec<SvgHead>,
}

fn marked_heads(svg: &str) -> Result<SvgHeads> {
    let page = Page::read(svg).context("unsupported SVG page for head proof")?;
    let attribute = |key: &str| {
        page.format.iter().find_map(|(name, value)| (name == key).then_some(value.as_str()))
    };
    let viewport = attribute("viewBox").context("missing SVG viewBox")?
        .split_ascii_whitespace().map(number).collect::<Option<Vec<_>>>().context("invalid SVG viewBox")?;
    let [0.0, 0.0, view_width, view_height] = viewport.as_slice() else {
        anyhow::bail!("unsupported SVG viewBox origin");
    };
    let size_in_points = |name| {
        number(attribute(name)?.strip_suffix("mm")?).map(|size| size * 72.0 / 25.4)
    };
    let width = size_in_points("width").context("invalid SVG page width")?;
    let height = size_in_points("height").context("invalid SVG page height")?;
    let sx = width / view_width;
    let sy = height / view_height;
    ensure!((0.0..2000.0).contains(&width) && width > 0.0
        && (0.0..2000.0).contains(&height) && height > 0.0
        && sx.is_finite() && sy.is_finite() && sx > 0.0 && sy > 0.0,
        "unqualified SVG page dimensions");

    let mut used = BTreeSet::new();
    let mut heads = Vec::new();
    for shape in page.shapes.iter().filter(|shape| shape.class == "Note") {
        let paint = shape.paint.iter().find(|(key, _)| key == "fill")
            .and_then(|(_, value)| value.strip_prefix('#')).context("unmarked SVG notehead")?;
        ensure!(paint.len() == 6 && paint.bytes().all(|b| b.is_ascii_hexdigit()), "invalid SVG head RGB");
        let [_, r, g, b] = u32::from_str_radix(paint, 16)?.to_be_bytes();
        let mark = [r, g, b];
        ensure!(mark != [0; 3] && used.insert(mark), "unbound or duplicate SVG head mark");
        ensure!(shape.filled && !shape.even_odd && shape.half_stroke_width == 0.0
            && (1..=MAX_CONTOURS).contains(&shape.contours.len()), "unsupported SVG notehead paint");
        let outlines = shape.contours.iter().map(|contour| {
            contour.iter().map(|p| Point { x: p.x as f64 * sx / GRID, y: p.y as f64 * sy / GRID })
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        for outline in &outlines {
            validate_polygon(outline)?;
        }
        heads.push(SvgHead { mark, outlines });
    }
    Ok(SvgHeads { width, height, contour_error: CONTOUR_ERROR * sx.max(sy), heads })
}

fn distance(first: Point, second: Point) -> f64 {
    (first.x - second.x).hypot(first.y - second.y)
}

fn flatten(start: Point, control: Point, end: Point, depth: usize, out: &mut Vec<Point>) -> Result<()> {
    ensure!(depth <= 14 && out.len() < MAX_POINTS, "PDF head contour subdivision limit");
    // The curve and its chord differ at parameter t by at most half the
    // distance from the control point to the chord midpoint.
    if 0.5 * distance(control, start.mid(end)) <= POLYLINE_ERROR {
        out.push(end);
    } else {
        let left = start.mid(control);
        let right = control.mid(end);
        let center = left.mid(right);
        flatten(start, left, center, depth + 1, out)?;
        flatten(center, right, end, depth + 1, out)?;
    }
    Ok(())
}

fn validate_polygon(points: &[Point]) -> Result<()> {
    ensure!((4..=MAX_POINTS).contains(&points.len()) && points.first() == points.last()
        && points.iter().all(|p| p.x.abs() < 2000.0 && p.y.abs() < 2000.0),
        "unqualified closed head contour");
    Ok(())
}

fn polygons(head: &PdfHeadInk) -> Result<Vec<Vec<Point>>> {
    ensure!((1..=MAX_CONTOURS).contains(&head.outline.len()), "unqualified PDF head contour count");
    head.outline.iter().map(|contour| {
        let [first, ..] = contour.as_slice() else {
            anyhow::bail!("empty PDF head contour");
        };
        let point = |(x, y)| Point { x, y };
        let mut polygon = vec![point(first[0])];
        for [start, control, end] in contour.iter().map(|curve| curve.map(point)) {
            ensure!(polygon.last() == Some(&start), "disconnected PDF head contour");
            flatten(start, control, end, 0, &mut polygon)?;
        }
        validate_polygon(&polygon)?;
        Ok(polygon)
    }).collect()
}

fn area(points: &[Point]) -> f64 {
    let origin = points[0];
    points.windows(2).map(|edge| {
        let (a, b) = (edge[0], edge[1]);
        (a.x - origin.x) * (b.y - origin.y) - (a.y - origin.y) * (b.x - origin.x)
    }).sum::<f64>() * 0.5
}

fn point_edge_distance(p: Point, a: Point, b: Point) -> f64 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let length = dx * dx + dy * dy;
    if length == 0.0 {
        return distance(p, a);
    }
    let t = (((p.x - a.x) * dx + (p.y - a.y) * dy) / length).clamp(0.0, 1.0);
    distance(p, Point { x: a.x + t * dx, y: a.y + t * dy })
}

fn distance_to_contour(p: Point, contour: &[Point]) -> f64 {
    contour.windows(2).map(|edge| point_edge_distance(p, edge[0], edge[1])).fold(f64::INFINITY, f64::min)
}

fn sampled_distance(source: &[Point], target: &[Point]) -> Result<f64> {
    let mut maximum = distance_to_contour(source[0], target);
    let mut remaining = 8192;
    for edge in source.windows(2) {
        let (a, b) = (edge[0], edge[1]);
        let samples = (distance(a, b) / SAMPLE_STEP).ceil() as usize;
        ensure!(samples <= remaining, "head contour comparison sample limit");
        remaining -= samples;
        for index in 1..=samples {
            let t = index as f64 / samples as f64;
            let p = Point { x: a.x + t * (b.x - a.x), y: a.y + t * (b.y - a.y) };
            maximum = maximum.max(distance_to_contour(p, target));
        }
    }
    Ok(maximum)
}

fn verify_outline(pdf: &[Vec<Point>], svg: &SvgHead, svg_error: f64) -> Result<()> {
    ensure!(pdf.len() == svg.outlines.len(), "PDF and SVG head contour count differs");
    let mut used = BTreeSet::new();
    for contour in pdf {
        let winding = area(contour);
        ensure!(winding.abs() > 0.05, "PDF head contour winding unresolved");
        let mut matched = None;
        for (index, outlined) in svg.outlines.iter().enumerate() {
            let other_winding = area(outlined);
            if used.contains(&index) || other_winding.abs() <= 0.05
                || other_winding.signum() != winding.signum() {
                continue;
            }
            // Distance to the other outline is 1-Lipschitz: every chord point
            // is within half a sample step of an evaluated point.
            let bound = sampled_distance(contour, outlined)?.max(sampled_distance(outlined, contour)?)
                + SAMPLE_STEP / 2.0 + svg_error + POLYLINE_ERROR + ROUNDING_RESERVE;
            if bound <= CONTOUR_MARGIN {
                ensure!(matched.replace(index).is_none(), "ambiguous PDF head contour correspondence");
            }
        }
        used.insert(matched.context("embedded PDF head disagrees with its SVG contour")?);
    }
    Ok(())
}

fn winding_at(point: Point, polygons: &[Vec<Point>]) -> i32 {
    let mut winding = 0;
    for edge in polygons.iter().flat_map(|polygon| polygon.windows(2)) {
        let (a, b) = (edge[0], edge[1]);
        let cross = (b.x - a.x) * (point.y - a.y) - (b.y - a.y) * (point.x - a.x);
        if a.y <= point.y && point.y < b.y && cross > 0.0 {
            winding += 1;
        } else if b.y <= point.y && point.y < a.y && cross < 0.0 {
            winding -= 1;
        }
    }
    winding
}

fn head_bounds(head: &PdfHeadInk) -> Result<Bounds> {
    // The Bézier curve and its filled region stay in the control-point hull.
    Bounds::from_points(head.outline.iter().flatten().flatten().map(|&(x, y)| Point { x, y }),
        ROUNDING_RESERVE).context("missing embedded head outline")
}

fn stem_bounds(stem: &PdfStemInk) -> Result<Bounds> {
    ensure!(stem.start.0 == stem.end.0 && stem.start.1 != stem.end.1
        && stem.width.is_finite() && stem.width > 0.0, "unsupported PDF stem rectangle");
    let top = stem.start.1.min(stem.end.1);
    let bottom = stem.start.1.max(stem.end.1);
    Ok(Bounds { left: stem.start.0 - stem.width / 2.0, right: stem.start.0 + stem.width / 2.0, top, bottom })
}

fn proves_attachment(stroke: Bounds, head: Bounds, outline: &[Vec<Point>], side: Side) -> bool {
    let middle_x = head.left.midpoint(head.right);
    let middle_y = head.top.midpoint(head.bottom);
    let x = stroke.right.midpoint(stroke.left);
    let width = stroke.right - stroke.left;
    let radius = (width / 8.0).min(0.02);
    let y = match side {
        Side::Above if stroke.top < head.top - ROUNDING_RESERVE
            && stroke.left > middle_x + ROUNDING_RESERVE && stroke.bottom < middle_y - ROUNDING_RESERVE => {
            stroke.bottom - width / 4.0
        }
        Side::Below if stroke.bottom > head.bottom + ROUNDING_RESERVE
            && stroke.right < middle_x - ROUNDING_RESERVE && stroke.top > middle_y + ROUNDING_RESERVE => {
            stroke.top + width / 4.0
        }
        _ => return false,
    };
    if y - radius <= stroke.top || y + radius >= stroke.bottom || radius <= ROUNDING_RESERVE {
        return false;
    }
    let point = Point { x, y };
    // The open disk is inside the actual painted stem and cannot cross any
    // true glyph contour. Winding proves the disk is in the filled head.
    winding_at(point, outline) != 0 && outline.iter().all(|contour| {
        distance_to_contour(point, contour) > radius + POLYLINE_ERROR + ROUNDING_RESERVE
    })
}

fn gap(a: Bounds, b: Bounds) -> f64 {
    (0.0_f64.max(a.left - b.right).max(b.left - a.right))
        .hypot(0.0_f64.max(a.top - b.bottom).max(b.top - a.bottom))
}

pub(crate) fn proves_single_head_attachment(stem: &PdfStemInk, head: &PdfHeadInk, side: Side) -> Result<bool> {
    Ok(proves_attachment(stem_bounds(stem)?, head_bounds(head)?, &polygons(head)?, side))
}

fn verify_foreign_text_clearance(stroke: Bounds, width: f64, foreground: &[[[f64; 2]; 2]]) -> Result<()> {
    for &[[left, top], [right, bottom]] in foreground {
        ensure!(left <= right && top <= bottom, "unqualified foreground text bounds");
        let corners = [Point { x: left, y: top }, Point { x: right, y: bottom }];
        let bounds = Bounds::from_points(corners.into_iter(), ROUNDING_RESERVE)
            .context("missing foreground text bounds")?;
        ensure!(gap(stroke, bounds) >= width + ROUNDING_RESERVE,
            "PDF stem has no proven clearance from foreground text ink");
    }
    Ok(())
}

fn verify_attachments(pdf: &[PdfHeadPage], attachments: &[Attachment]) -> Result<()> {
    let mut head_marks = BTreeSet::new();
    let mut stem_marks = BTreeSet::new();
    ensure!(!attachments.is_empty(), "missing stem-to-head source bindings");
    for attachment in attachments {
        ensure!(head_marks.insert(attachment.head) && stem_marks.insert(attachment.stem),
            "ambiguous source attachment roster");
        let mut pages = pdf.iter().filter_map(|page| {
            page.heads.iter().find(|head| head.mark == attachment.head).map(|head| (page, head))
        });
        let (page, own) = pages.next().context("PDF own head missing")?;
        ensure!(pages.next().is_none(), "PDF own head occurs on multiple pages");
        let mut stems = page.stems.iter().filter(|stem| stem.mark == attachment.stem);
        let stem = stems.next().context("PDF own stem absent on head page")?;
        ensure!(stems.next().is_none(), "PDF own stem is ambiguous");
        let stroke = stem_bounds(stem)?;
        ensure!(proves_attachment(stroke, head_bounds(own)?, &polygons(own)?, attachment.side),
            "embedded PDF head does not prove the intended stem attachment");
        for other in page.heads.iter().filter(|head| head.mark != own.mark) {
            ensure!(gap(stroke, head_bounds(other)?) >= stem.width + ROUNDING_RESERVE,
                "PDF stem has no proven clearance from a foreign head");
        }
        verify_foreign_text_clearance(stroke, stem.width, &page.foreground)?;
    }
    ensure!(head_marks.len() == pdf.iter().map(|page| page.heads.len()).sum::<usize>()
        && stem_marks.len() == pdf.iter().map(|page| page.stems.len()).sum::<usize>(),
        "incomplete PDF source attachment roster");
    Ok(())
}

/// The caller binds the authored marks and expected attachments. This
/// function validates their actual PDF contour and ink, with an independent
/// geometry cross-check against the marked SVG heads.
pub(crate) fn prove_bound_heads(
    plain_svg: &[&str], marked_svg: &[&str], plain: &[u8], marked: &[u8], attachments: &[Attachment],
) -> Result<()> {
    let svg = marked_svg.iter().map(|page| marked_heads(page)).collect::<Result<Vec<_>>>()?;
    let expected = svg.iter().map(|page| page.heads.iter().map(|head| head.mark).collect()).collect::<Vec<_>>();
    let pdf = probe_head_pages(plain_svg, marked_svg, plain, marked, &expected)?;
    ensure!(pdf.len() == svg.len(), "notehead page count differs");
    for (index, (pdf, svg)) in pdf.iter().zip(&svg).enumerate() {
        ensure!((pdf.width - svg.width).abs() <= 1.0 && (pdf.height - svg.height).abs() <= 1.0,
            "PDF and SVG head pages differ in physical size");
        ensure!(pdf.heads.len() == svg.heads.len(), "head inventory differs on page {}", index + 1);
        for head in &svg.heads {
            let mut matching = pdf.heads.iter().filter(|printed| printed.mark == head.mark);
            let printed = matching.next().context("marked glyph missing from PDF page")?;
            ensure!(matching.next().is_none(), "ambiguous marked glyph on PDF page");
            verify_outline(&polygons(printed)?, head, svg.contour_error)
                .with_context(|| format!("PDF head mark {:?} on page {}", head.mark, index + 1))?;
        }
    }
    verify_attachments(&pdf, attachments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_outer_dimensions_cannot_replace_the_embedded_contour() {
        let points = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (0.0, 0.0)]
            .map(|(x, y)| Point { x, y });
        let expected = SvgHead { mark: [1, 2, 3], outlines: vec![points.to_vec()] };
        assert!(verify_outline(&expected.outlines, &expected, POLYLINE_ERROR).is_ok());
        let different = [points[0], points[1], Point { x: 1.75, y: 1.0 }, points[2], points[3], points[4]];
        assert!(verify_outline(&[different.to_vec()], &expected, POLYLINE_ERROR).is_err());
    }

    #[test]
    fn foreign_text_bounds_must_clear_the_painted_stem() {
        let stroke = Bounds { left: 10.0, top: 20.0, right: 10.5, bottom: 24.0 };
        let separated = [[11.1, 20.5], [12.0, 22.0]];
        let overlapping = [[10.4, 20.5], [12.0, 22.0]];
        let too_close = [[10.9, 20.5], [12.0, 22.0]];
        let exactly_one_width = [[11.0, 20.5], [12.0, 22.0]];
        assert!(verify_foreign_text_clearance(stroke, 0.5, &[separated]).is_ok());
        for foreign in [overlapping, too_close, exactly_one_width] {
            assert!(verify_foreign_text_clearance(stroke, 0.5, &[separated, foreign]).is_err());
        }
        assert!(verify_foreign_text_clearance(stroke, 0.5, &[[[f64::NAN, 20.5], [12.0, 22.0]]]).is_err());
    }
}
