//! Read-only geometry of a paired plain and source-marked PDF.

use super::{
    POINT_RESERVE, PdfHeadInk, PdfStemInk, foreground_bounds, glyph_font, head_glyph,
    page_streams_for, path_ink, read_text, source_color,
};
use crate::audit_only::tied_stem_pair_decision::{InkBounds, PairFrame, PairRole, Contact, Decision, decide_pair};
use anyhow::{Context, Result, ensure};

pub(crate) type Mark = [u8; 3];

pub(crate) struct TrackedStem {
    pub(crate) head: Mark,
    pub(crate) stem: Mark,
}

pub(crate) struct PdfPaintPage {
    pub(crate) glyphs: Vec<PdfGlyph>,
    pub(crate) strokes: Vec<PdfLine>,
    pub(crate) ties: Vec<PdfCurve>,
    fills: Vec<path_ink::FilledPath>,
    pub(crate) other_fills: Vec<InkBounds>,
    size: [f64; 2],
}

#[derive(PartialEq)]
pub(crate) struct PdfGlyph {
    pub(crate) ink: InkBounds,
    pub(crate) mark: Mark,
    font: String,
    character: Option<char>,
    outline: head_glyph::GlyphOutline,
    origin: (f64, f64),
    em: (f64, f64),
    color: [f64; 3],
}

pub(crate) struct PdfLine {
    pub(crate) ink: InkBounds,
    pub(crate) mark: Mark,
    pub(crate) points: [(f64, f64); 2],
    pub(crate) width: f64,
    pub(crate) barline: bool,
    pub(crate) staffline: bool,
    contour: path_ink::Contour,
}

pub(crate) struct PdfCurve {
    pub(crate) ink: InkBounds,
    pub(crate) mark: Mark,
    pub(crate) controls: Vec<(f64, f64)>,
    contour: path_ink::Contour,
}

pub(crate) struct PdfPaint {
    pub(crate) pages: Vec<PdfPaintPage>,
    pub(crate) anchors: Vec<(Mark, PdfHeadInk, PdfStemInk)>,
}

fn bounded(page: usize, points: &[(f64, f64)], margin: f64) -> Result<InkBounds> {
    ensure!(!points.is_empty(), "missing PDF paint points");
    let mut edges = [[f64::INFINITY; 2], [f64::NEG_INFINITY; 2]];
    for &(x, y) in points {
        ensure!(x.is_finite() && y.is_finite(), "nonfinite PDF paint point");
        edges[0] = [edges[0][0].min(x - margin), edges[0][1].min(y - margin)];
        edges[1] = [edges[1][0].max(x + margin), edges[1][1].max(y + margin)];
    }
    Ok(InkBounds { page, bounds: edges })
}

fn head_outline(
    font: &super::FontResource, show: &path_ink::TextInk, marked: &path_ink::TextInk, mark: Mark,
) -> Result<PdfHeadInk> {
    ensure!(font.name == "/Leland" && matches!(font.cmap.get(&show.cid), Some('\u{e0a3}' | '\u{e0a4}')),
        "unqualified marked notehead");
    let glyph = head_glyph::glyph_outline(&font.font, show.cid)?;
    let scale = 2.0 * f64::from(glyph.units_per_em);
    let project = |[x, y]: [i32; 2]| show.transform(f64::from(x) / scale, f64::from(y) / scale);
    let outline: Vec<Vec<_>> = glyph.contours.iter().map(|contour| contour.iter().map(|curve| {
        let a = project(curve.start);
        let b = project(curve.end);
        [a, curve.control.map_or_else(|| (a.0.midpoint(b.0), a.1.midpoint(b.1)), project), b]
    }).collect()).collect();
    let points: Vec<_> = outline.iter().flatten().flatten().copied().collect();
    show.proves_unclipped(&points, POINT_RESERVE)?;
    marked.proves_unclipped(&points, POINT_RESERVE)?;
    Ok(PdfHeadInk { mark, outline })
}

pub(crate) fn read_paired_paint(plain: &[u8], marked: &[u8], targets: &[TrackedStem]) -> Result<PdfPaint> {
    let plain = page_streams_for(plain, true)?;
    let marked = page_streams_for(marked, true)?;
    ensure!(plain.len() == marked.len(), "PDF marks changed page count");
    let mut pages = Vec::new();
    let mut heads = std::collections::BTreeMap::new();
    for (index, (source, shadow)) in plain.iter().zip(&marked).enumerate() {
        let number = index + 1;
        ensure!(source.width == shadow.width && source.height == shadow.height && source.fonts == shadow.fonts,
            "PDF marks changed viewport or font resources");
        path_ink::same_except_rgb(&source.content, &shadow.content)
            .with_context(|| format!("paired PDF paint on page {number}"))?;
        let fonts = source.fonts.as_ref().context("missing paired PDF fonts")?;
        let original = read_text(source)?;
        let painted = read_text(shadow)?;
        ensure!(original.len() == painted.len(), "PDF marks changed glyph count");
        let mut glyphs = Vec::new();
        for (show, marked) in original.iter().zip(&painted) {
            ensure!(show.font == marked.font && show.cid == marked.cid, "PDF mark changed glyph identity");
            let font = glyph_font(fonts, show)?;
            let mark = source_color(marked.fill)?;
            if targets.iter().any(|target| target.head == mark) {
                let head = head_outline(font, show, marked, mark)?;
                ensure!(heads.insert(mark, (number, head)).is_none(), "duplicate selected PDF head mark");
            }
            if let Some(bounds) = foreground_bounds(show, font)? {
                ensure!(show.fill == [0.0; 3], "nonblack source text prevents unordered PDF paint comparison");
                let [[left, top], [right, bottom]] = bounds;
                show.proves_unclipped(&[(left, top), (right, bottom)], POINT_RESERVE)?;
                glyphs.push(PdfGlyph {
                    ink: InkBounds { page: number, bounds }, mark, font: font.name.clone(),
                    character: font.cmap.get(&show.cid).copied(),
                    outline: head_glyph::glyph_outline(&font.font, show.cid)?,
                    origin: show.transform(0.0, 0.0), em: show.transform(1.0, 1.0), color: show.fill,
                });
            }
        }
        let read_paths = |page: &super::PageStream| {
            path_ink::read_ink(&page.content, page.width, page.height, "/GSa", POINT_RESERVE)
        };
        let original = read_paths(source)?;
        let painted = read_paths(shadow)?;
        ensure!(original.strokes.len() == painted.strokes.len() && original.ties.len() == painted.ties.len(),
            "PDF marks changed stroke or tie count");
        ensure!(original.strokes.iter().chain(&original.ties).all(|paint|
            paint.stroke == [0.0; 3] && paint.fill.is_none_or(|color| color == [0.0; 3])),
            "nonblack source path prevents unordered PDF paint comparison");
        let strokes = original.strokes.into_iter().zip(painted.strokes).map(|(line, mark)| {
            let points = <[_; 2]>::try_from(line.points.as_slice()).context("unqualified PDF line geometry")?;
            let ink = bounded(number, &line.points, line.width / 2.0 + POINT_RESERVE)?;
            Ok(PdfLine { ink, mark: source_color(mark.stroke)?, points, width: line.width,
                barline: false, staffline: false, contour: line })
        }).collect::<Result<Vec<_>>>()?;
        let ties = original.ties.into_iter().zip(painted.ties).map(|(curve, mark)| {
            let ink = bounded(number, &curve.points, curve.width / 2.0 + POINT_RESERVE)?;
            Ok(PdfCurve { ink, mark: source_color(mark.stroke)?, controls: curve.points.clone(), contour: curve })
        }).collect::<Result<Vec<_>>>()?;
        let is_background = |fill: &path_ink::FilledPath| {
            fill.order == 0 && fill.rgb == [1.0; 3]
        };
        let mut fills = original.fills;
        ensure!(fills.iter().all(|fill| fill.rgb == [0.0; 3] || is_background(fill)),
            "nonblack source fill prevents unordered PDF paint comparison");
        let other_fills = fills.iter().filter(|fill| !is_background(fill)).map(|fill| {
            let [left, top, right, bottom] = fill.bounds;
            InkBounds { page: number, bounds: [[left, top], [right, bottom]] }
        }).collect();
        for fill in &mut fills {
            // The qualified opaque black paint layers commute. The sole white
            // background is still checked as the physically first paint.
            fill.order = 0;
        }
        pages.push(PdfPaintPage { glyphs, strokes, ties, fills, other_fills, size: [source.width, source.height] });
    }

    let mut anchors = Vec::new();
    for target in targets {
        let (number, head) = heads.remove(&target.head).context("selected PDF head mark is missing")?;
        let mut lines = pages.iter().flat_map(|page| &page.strokes).filter(|line| line.mark == target.stem);
        let line = lines.next().context("selected PDF stem mark is missing")?;
        ensure!(lines.next().is_none() && line.ink.page == number && line.contour.is_butt_stroke(),
            "selected PDF stem is ambiguous, clipped or on another page");
        anchors.push((target.stem, head, PdfStemInk {
            mark: target.stem, start: line.points[0], end: line.points[1], width: line.width,
        }));
    }
    Ok(PdfPaint { pages, anchors })
}

pub(crate) fn bind_guides(pdf: &mut PdfPaint, svgs: &[String]) -> Result<()> {
    ensure!(pdf.pages.len() == svgs.len(), "SVG and PDF page count differ");
    for (page, svg) in pdf.pages.iter_mut().zip(svgs) {
        let (expected, stems, staff) = path_ink::svg_stroke_roles(svg)?;
        let vertical: Vec<_> = page.strokes.iter().enumerate().filter(|(_, line)| line.points[0].0 == line.points[1].0)
            .map(|(index, line)| (index, line.contour.clone())).collect();
        let contours: Vec<_> = vertical.iter().map(|(_, line)| line.clone()).collect();
        for (ordinal, index) in path_ink::bind_contours(&expected, &contours, POINT_RESERVE)?.into_iter().enumerate() {
            page.strokes[vertical[index].0].barline = !stems.contains(&ordinal);
        }
        for shape in staff {
            let [(x1, y), (x2, _)] = shape.points[..] else { anyhow::bail!("unknown SVG staff line"); };
            let (left, right) = (x1.min(x2), x1.max(x2));
            let mut spans: Vec<_> = page.strokes.iter().enumerate().filter_map(|(index, line)| {
                let [(a, top), (b, bottom)] = line.points;
                let (start, end) = (a.min(b), a.max(b));
                (top == bottom && (y - top).abs() < POINT_RESERVE && start < end
                    && (line.width - shape.width).abs() < POINT_RESERVE
                    && start >= left - POINT_RESERVE && end <= right + POINT_RESERVE)
                    .then_some((index, start, end))
            }).collect();
            spans.sort_by(|a, b| a.1.total_cmp(&b.1));
            let mut previous = left;
            for &(index, start, end) in &spans {
                ensure!((start - previous).abs() <= POINT_RESERVE && !page.strokes[index].staffline,
                    "unbound or duplicated PDF staff line span");
                previous = end;
            }
            ensure!(!spans.is_empty() && (previous - right).abs() <= POINT_RESERVE, "incomplete PDF staff line");
            for (index, _, _) in spans { page.strokes[index].staffline = true; }
        }
    }
    Ok(())
}

fn same_paint<T: PartialEq>(a: impl Iterator<Item = T>, b: impl Iterator<Item = T>) -> bool {
    let mut remaining = b.collect::<Vec<_>>();
    // Bound the worst-case exact matching cost for untrusted external PDFs.
    if remaining.len() > 4096 { return false; }
    for paint in a {
        let Some(index) = remaining.iter().position(|other| *other == paint) else { return false };
        remaining.swap_remove(index);
    }
    remaining.is_empty()
}

pub(crate) fn delta_stays_on_marked_paint(
    before: &PdfPaint, after: &PdfPaint, stems: &[Mark], ties: &[Mark],
) -> bool {
    before.pages.len() == after.pages.len() && before.pages.iter().zip(&after.pages).all(|(a, b)| {
        let foreign_line = |line: &&PdfLine| !stems.contains(&line.mark);
        let foreign_tie = |arc: &&PdfCurve| !ties.contains(&arc.mark);
        a.size == b.size && same_paint(a.glyphs.iter(), b.glyphs.iter())
            && same_paint(a.fills.iter(), b.fills.iter())
            && same_paint(a.strokes.iter().filter(foreign_line).map(|line| &line.contour),
                b.strokes.iter().filter(foreign_line).map(|line| &line.contour))
            && same_paint(a.ties.iter().filter(foreign_tie).map(|arc| &arc.contour),
                b.ties.iter().filter(foreign_tie).map(|arc| &arc.contour))
    })
}

pub(crate) fn foreign_clearance(first: InkBounds, second: InkBounds, minimum: f64) -> Decision {
    let frame = PairFrame { items: [first, second], contact: Contact::Unknown };
    decide_pair(PairRole::Foreign, Some(frame), Some(frame), minimum, false)
}
