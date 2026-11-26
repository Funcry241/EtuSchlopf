//! Strict readback for the observed MuseScore page content profile.
//!
//! The caller qualifies the page and resource envelope before consuming these
//! observations. Matching controls bounds the underlying cubic by convexity.
//! The readback describes PDF vector paint before device-specific rasterization.

use anyhow::{Result, bail, ensure};
use roxmltree::{Document, Node};

type Point = (f64, f64);
type Rgb = [f64; 3];

const BLACK: Rgb = [0.0; 3];

pub(super) struct InkPage {
    pub(super) ties: Vec<Contour>,
    pub(super) strokes: Vec<Contour>,
}

#[derive(Clone, Debug)]
pub(super) struct Contour {
    pub(super) points: Vec<Point>,
    pub(super) width: f64,
    pub(super) stroke: Rgb,
    pub(super) fill: Option<Rgb>,
    cap: f64,
    join: f64,
}

#[derive(Clone, Copy)]
struct View([f64; 4]);

impl View {
    fn around(points: &[Point], padding: f64) -> Result<Self> {
        ensure!(
            !points.is_empty() && padding.is_finite() && padding >= 0.0,
            "invalid contour extent"
        );
        let mut sides = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for &(x, y) in points {
            ensure!(
                x.is_finite() && y.is_finite(),
                "nonfinite contour coordinate"
            );
            sides = [
                sides[0].min(x),
                sides[1].min(y),
                sides[2].max(x),
                sides[3].max(y),
            ];
        }
        Self::checked([
            sides[0] - padding,
            sides[1] - padding,
            sides[2] + padding,
            sides[3] + padding,
        ])
    }

    fn checked(edges: [f64; 4]) -> Result<Self> {
        ensure!(
            edges.iter().all(|edge| edge.is_finite())
                && edges[0] <= edges[2]
                && edges[1] <= edges[3],
            "invalid contour bounds"
        );
        Ok(Self(edges))
    }

    fn contains(self, other: Self) -> bool {
        self.0[0] <= other.0[0]
            && self.0[1] <= other.0[1]
            && self.0[2] >= other.0[2]
            && self.0[3] >= other.0[3]
    }

    fn intersect(self, other: Self) -> Result<Self> {
        let [left, top, right, bottom] = self.0;
        let [l, t, r, b] = other.0;
        Self::checked([left.max(l), top.max(t), right.min(r), bottom.min(b)])
    }
}

#[derive(Clone, Copy)]
struct Frame {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

impl Frame {
    const IDENTITY: Self = Self {
        x: 1.0,
        y: 1.0,
        dx: 0.0,
        dy: 0.0,
    };

    fn multiply(self, [a, b, c, d, e, f]: [f64; 6]) -> Result<Self> {
        ensure!(
            b == 0.0 && c == 0.0 && a != 0.0 && d != 0.0,
            "unsupported PDF transform"
        );
        let next = Self {
            x: self.x * a,
            y: self.y * d,
            dx: self.x.mul_add(e, self.dx),
            dy: self.y.mul_add(f, self.dy),
        };
        ensure!(
            [next.x, next.y, next.dx, next.dy]
                .iter()
                .all(|v| v.is_finite())
                && next.x != 0.0
                && next.y != 0.0,
            "unrepresentable PDF transform"
        );
        Ok(next)
    }

    fn point(self, x: f64, y: f64, height: f64) -> Point {
        (
            self.x.mul_add(x, self.dx),
            height - self.y.mul_add(y, self.dy),
        )
    }
}

#[derive(Clone, Copy)]
struct Paint {
    frame: Frame,
    clip: View,
    width: f64,
    cap: f64,
    join: f64,
    fill: Rgb,
    stroke: Rgb,
    fill_rgb: bool,
    stroke_rgb: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Segment {
    Start(Point),
    Line(Point),
    Cubic([Point; 3]),
    Close,
}

fn pdf_number(raw: &str) -> Result<f64> {
    let raw_body = raw.trim_start_matches(['+', '-']);
    ensure!(
        raw_body.bytes().any(|b| b.is_ascii_digit())
            && raw_body.bytes().all(|b| b.is_ascii_digit() || b == b'.'),
        "unsupported PDF number"
    );
    let value = raw.parse::<f64>()?;
    ensure!(
        value.is_finite() && (value != 0.0 || !raw_body.bytes().any(|b| matches!(b, b'1'..=b'9'))),
        "unrepresentable PDF number"
    );
    Ok(value)
}

fn numbers<const COUNT: usize>(args: &[&str]) -> Result<[f64; COUNT]> {
    ensure!(args.len() == COUNT, "wrong PDF operand count");
    let mut values = [0.0; COUNT];
    for (value, arg) in values.iter_mut().zip(args) {
        *value = pdf_number(arg)?;
    }
    Ok(values)
}

fn is_pdf_space(c: u8) -> bool {
    matches!(c, 0 | b'\t' | b'\n' | b'\x0c' | b'\r' | b' ')
}

fn is_delimiter(c: u8) -> bool {
    b"()<>[]{}/%".contains(&c)
}

struct Operations<'a> {
    stream: &'a str,
}

impl<'a> Operations<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self> {
        let stream = std::str::from_utf8(bytes)?;
        ensure!(stream.is_ascii(), "non-ASCII PDF content");
        Ok(Self { stream })
    }

    fn next(&mut self) -> Result<Option<(&'a str, Vec<&'a str>)>> {
        let mut args = Vec::with_capacity(6);
        loop {
            self.stream = self
                .stream
                .trim_start_matches(|c: char| c.is_ascii() && is_pdf_space(c as u8));
            let Some(first) = self.stream.as_bytes().first().copied() else {
                ensure!(args.is_empty(), "PDF operands without operator");
                return Ok(None);
            };
            let token_len = match first {
                b'%' => {
                    let end = self.stream.find(['\n', '\r']).unwrap_or(self.stream.len());
                    self.stream = &self.stream[end..];
                    continue;
                }
                b'<' => {
                    let end = self
                        .stream
                        .find('>')
                        .ok_or_else(|| anyhow::anyhow!("open PDF hex string"))?;
                    ensure!(
                        self.stream[1..end].bytes().all(|c| c.is_ascii_hexdigit()),
                        "unsupported PDF hex string"
                    );
                    end + 1
                }
                b'[' => {
                    let end = self
                        .stream
                        .find(']')
                        .ok_or_else(|| anyhow::anyhow!("open PDF array"))?;
                    ensure!(
                        self.stream.as_bytes()[1..end]
                            .iter()
                            .copied()
                            .all(is_pdf_space),
                        "unsupported PDF array"
                    );
                    end + 1
                }
                _ => {
                    1 + self.stream.as_bytes()[1..]
                        .iter()
                        .position(|b| is_pdf_space(*b) || is_delimiter(*b))
                        .unwrap_or(self.stream.len() - 1)
                }
            };
            let token = &self.stream[..token_len];
            self.stream = &self.stream[token_len..];
            if first.is_ascii_alphabetic() {
                return Ok(Some((token, args)));
            }
            ensure!(
                matches!(first, b'<' | b'[' | b'/' | b'+' | b'-' | b'.' | b'0'..=b'9')
                    && args.len() < 6,
                "unsupported PDF content token {token:?}"
            );
            args.push(token);
        }
    }
}

fn rgb(args: &[&str]) -> Result<Rgb> {
    let components = numbers(args)?;
    ensure!(
        components.iter().all(|value| (0.0..=1.0).contains(value)),
        "PDF RGB component out of range"
    );
    Ok(components)
}

fn rectangle(path: &[Segment]) -> Result<View> {
    let (Segment::Start(a), Segment::Line(b), Segment::Line(c), Segment::Line(d)) = (
        path.first().copied().unwrap_or(Segment::Close),
        path.get(1).copied().unwrap_or(Segment::Close),
        path.get(2).copied().unwrap_or(Segment::Close),
        path.get(3).copied().unwrap_or(Segment::Close),
    ) else {
        bail!("unsupported PDF clip path");
    };
    ensure!(
        path[4..] == [Segment::Close] || path[4..] == [Segment::Line(a), Segment::Close],
        "open or nonrectangular PDF clip"
    );
    let clip = View::around(&[a, b, c, d], 0.0)?;
    let [left, top, right, bottom] = clip.0;
    ensure!(left < right && top < bottom, "empty PDF clip");
    for (u, v) in [(a, b), (b, c), (c, d), (d, a)] {
        ensure!(
            (u.0 == v.0) != (u.1 == v.1)
                && (u.0 == left || u.0 == right)
                && (u.1 == top || u.1 == bottom),
            "nonrectangular PDF clip"
        );
    }
    ensure!(a != c && b != d, "degenerate PDF clip");
    Ok(clip)
}

fn painted_contour(
    points: Vec<Point>,
    paint: Paint,
    fill: Option<Rgb>,
    margin: f64,
) -> Result<Contour> {
    ensure!(
        paint.frame.x.abs() == paint.frame.y.abs(),
        "anisotropic PDF contour transform"
    );
    let width = paint.width * paint.frame.x.abs();
    ensure!(
        width.is_finite() && width > 0.0 && !points.is_empty(),
        "unsupported painted contour width"
    );
    // The rounded stroke lies inside its control hull expanded by half width.
    // A single square-ended line must also include its rotated end corners.
    let radius = if paint.cap == 2.0 {
        width * std::f64::consts::FRAC_1_SQRT_2
    } else {
        width / 2.0
    };
    ensure!(
        paint.clip.contains(View::around(&points, radius + margin)?),
        "PDF contour is not proven unclipped"
    );
    Ok(Contour {
        points,
        width,
        stroke: paint.stroke,
        fill,
        cap: paint.cap,
        join: paint.join,
    })
}

fn top_first(a: Point, b: Point) -> [Point; 2] {
    if a.1 <= b.1 { [a, b] } else { [b, a] }
}

pub(super) fn read_ink(
    content: &[u8],
    page_width_pt: f64,
    page_height_pt: f64,
    qualified_state: &str,
    margin_pt: f64,
) -> Result<InkPage> {
    ensure!(
        page_width_pt > 0.0
            && page_height_pt > 0.0
            && margin_pt.is_finite()
            && margin_pt > 0.0
            && qualified_state.starts_with('/'),
        "invalid PDF ink profile"
    );
    let mut paint = Paint {
        frame: Frame::IDENTITY,
        clip: View::checked([0.0, 0.0, page_width_pt, page_height_pt])?,
        width: 1.0,
        cap: 0.0,
        join: 0.0,
        fill: BLACK,
        stroke: BLACK,
        fill_rgb: false,
        stroke_rgb: false,
    };
    let mut stack = Vec::new();
    let mut path = Vec::new();
    let mut page = InkPage {
        ties: Vec::new(),
        strokes: Vec::new(),
    };
    let mut pending_clip = false;
    let mut text = false;
    let mut operations = Operations::new(content)?;
    while let Some((op, args)) = operations.next()? {
        ensure!(
            !pending_clip || op == "n",
            "unsupported PDF clipping sequence"
        );
        ensure!(
            !text || matches!(op, "Tf" | "Tm" | "Td" | "Tj" | "ET"),
            "unsupported PDF operation inside text"
        );
        ensure!(
            path.is_empty() || matches!(op, "l" | "c" | "h" | "W*" | "S" | "B*" | "f*" | "n"),
            "unsupported PDF operation inside a path"
        );
        match op {
            "q" | "Q" => {
                numbers::<0>(&args)?;
                if op == "q" {
                    stack.push(paint);
                } else {
                    paint = stack
                        .pop()
                        .ok_or_else(|| anyhow::anyhow!("PDF graphics stack underflow"))?;
                }
            }
            "cm" => paint.frame = paint.frame.multiply(numbers(&args)?)?,
            "J" | "j" | "w" => {
                let [value] = numbers(&args)?;
                if op == "w" {
                    ensure!(value >= 0.0, "negative PDF line width");
                    paint.width = value;
                } else {
                    ensure!(
                        [0.0, 1.0, 2.0].contains(&value),
                        "unsupported PDF line style"
                    );
                    if op == "J" {
                        paint.cap = value;
                    } else {
                        paint.join = value;
                    }
                }
            }
            "d" => {
                let [array, phase] = args.as_slice() else {
                    bail!("wrong PDF dash operands")
                };
                ensure!(
                    array.starts_with('[') && pdf_number(phase)? == 0.0,
                    "unsupported PDF dash"
                );
            }
            "gs" => ensure!(args == [qualified_state], "unqualified PDF graphics state"),
            "cs" | "CS" => {
                ensure!(args == ["/CSp"], "unsupported PDF color space");
                if op == "cs" {
                    paint.fill_rgb = true;
                    paint.fill = BLACK;
                } else {
                    paint.stroke_rgb = true;
                    paint.stroke = BLACK;
                }
            }
            "scn" | "SCN" => {
                if op == "scn" {
                    ensure!(paint.fill_rgb, "PDF fill color without bound RGB");
                    paint.fill = rgb(&args)?;
                } else {
                    ensure!(paint.stroke_rgb, "PDF stroke color without bound RGB");
                    paint.stroke = rgb(&args)?;
                }
            }
            "m" | "l" | "c" | "h" => {
                ensure!(
                    (op == "m") == path.is_empty() && path.last() != Some(&Segment::Close),
                    "unsupported PDF subpath"
                );
                let point = |x, y| paint.frame.point(x, y, page_height_pt);
                path.push(match op {
                    "m" | "l" => {
                        let [x, y] = numbers(&args)?;
                        if op == "m" {
                            Segment::Start(point(x, y))
                        } else {
                            Segment::Line(point(x, y))
                        }
                    }
                    "c" => {
                        let [a, b, c, d, e, f] = numbers(&args)?;
                        Segment::Cubic([point(a, b), point(c, d), point(e, f)])
                    }
                    _ => {
                        numbers::<0>(&args)?;
                        Segment::Close
                    }
                });
            }
            "W*" => {
                numbers::<0>(&args)?;
                paint.clip = paint.clip.intersect(rectangle(&path)?)?;
                pending_clip = true;
            }
            "S" | "B*" | "f*" | "n" => {
                numbers::<0>(&args)?;
                if op == "B*" {
                    let [
                        Segment::Start(start),
                        Segment::Cubic(first),
                        Segment::Cubic(second),
                        Segment::Close,
                    ] = path.as_slice()
                    else {
                        bail!("unsupported filled/stroked PDF curve")
                    };
                    ensure!(
                        *start == second[2]
                            && *start != first[2]
                            && paint.cap == 1.0
                            && paint.join == 1.0
                            && paint.fill == paint.stroke,
                        "unsupported PDF tie endpoints or paint style"
                    );
                    let points = std::iter::once(*start)
                        .chain(*first)
                        .chain(*second)
                        .collect();
                    page.ties
                        .push(painted_contour(points, paint, Some(paint.fill), margin_pt)?);
                } else if op == "S" {
                    let [Segment::Start(start), Segment::Line(end)] = path.as_slice() else {
                        bail!("unsupported stroked PDF path");
                    };
                    ensure!(start != end, "degenerate stroked PDF path");
                    page.strokes.push(painted_contour(
                        top_first(*start, *end).to_vec(),
                        paint,
                        None,
                        margin_pt,
                    )?);
                } else {
                    ensure!(
                        path.iter().all(|s| !matches!(s, Segment::Cubic(_))),
                        "unclassified PDF cubic path"
                    );
                }
                pending_clip = false;
                path.clear();
            }
            "BT" | "ET" => {
                numbers::<0>(&args)?;
                ensure!((op == "ET") == text, "unbalanced PDF text state");
                text = !text;
            }
            "Tf" | "Tm" | "Td" | "Tj" => {
                ensure!(text, "PDF text operation outside text");
                match op {
                    "Tf" => {
                        let [font, size] = args.as_slice() else {
                            bail!("invalid PDF font selection");
                        };
                        ensure!(
                            font.starts_with('/') && pdf_number(size)? > 0.0,
                            "invalid PDF font selection"
                        );
                    }
                    "Tm" => {
                        numbers::<6>(&args)?;
                    }
                    "Td" => {
                        numbers::<2>(&args)?;
                    }
                    _ => {
                        let [code] = args.as_slice() else {
                            bail!("invalid PDF glyph code");
                        };
                        ensure!(
                            code.starts_with('<') && (code.len() - 2).is_multiple_of(4),
                            "invalid PDF glyph code"
                        );
                    }
                }
            }
            _ => bail!("unsupported PDF content operator {op}"),
        }
    }
    ensure!(
        stack.is_empty() && path.is_empty() && !pending_clip && !text,
        "open PDF state at page end"
    );
    Ok(page)
}

pub(super) fn same_except_rgb(a: &[u8], b: &[u8]) -> Result<()> {
    fn next_geometry<'a>(ops: &mut Operations<'a>) -> Result<Option<(&'a str, Vec<&'a str>)>> {
        while let Some((op, args)) = ops.next()? {
            match op {
                "cs" | "CS" => ensure!(args == ["/CSp"], "unsupported PDF color space"),
                "scn" | "SCN" => {
                    rgb(&args)?;
                }
                _ => return Ok(Some((op, args))),
            }
        }
        Ok(None)
    }
    let mut a = Operations::new(a)?;
    let mut b = Operations::new(b)?;
    loop {
        let left = next_geometry(&mut a)?;
        let right = next_geometry(&mut b)?;
        ensure!(
            left == right,
            "marked PDF changed a non-color operator or operand"
        );
        if left.is_none() {
            return Ok(());
        }
    }
}

fn physically_scaled_svg(root: Node<'_, '_>) -> Result<(f64, f64)> {
    ensure!(
        root.has_tag_name(("http://www.w3.org/2000/svg", "svg"))
            && root.attributes().all(|a| matches!(
                a.name(),
                "width" | "height" | "viewBox" | "version" | "baseProfile"
            ) && a.namespace().is_none()),
        "unsupported SVG root"
    );
    let view = root
        .attribute("viewBox")
        .ok_or_else(|| anyhow::anyhow!("missing SVG viewport"))?
        .split_ascii_whitespace()
        .collect::<Vec<_>>();
    let [x, y, w, h] = numbers(&view)?;
    ensure!(
        x == 0.0 && y == 0.0 && w > 0.0 && h > 0.0,
        "unsupported SVG viewport"
    );
    let pt_per_coordinate = |key, span: f64| -> Result<f64> {
        let physical = root
            .attribute(key)
            .and_then(|raw| raw.strip_suffix("mm"))
            .ok_or_else(|| anyhow::anyhow!("missing SVG physical extent"))?;
        let scale = pdf_number(physical)? * (72.0 / 25.4) / span;
        ensure!(
            scale.is_finite() && scale > 0.0,
            "invalid SVG physical scale"
        );
        Ok(scale)
    };
    let sx = pt_per_coordinate("width", w)?;
    let sy = pt_per_coordinate("height", h)?;
    ensure!(
        (sx - sy).abs() <= 0.000_000_1,
        "anisotropic SVG physical scale"
    );
    Ok((sx, sy))
}

fn svg_rgb(raw: &str) -> Result<Rgb> {
    let hex = raw
        .strip_prefix('#')
        .filter(|hex| hex.is_ascii() && hex.len() == 6)
        .ok_or_else(|| anyhow::anyhow!("unsupported SVG contour color"))?;
    let mut triplet = BLACK;
    for (index, component) in triplet.iter_mut().enumerate() {
        *component = f64::from(u8::from_str_radix(&hex[2 * index..2 * index + 2], 16)?) / 255.0;
    }
    Ok(triplet)
}

pub(super) fn svg_ties(svg: &str) -> Result<Vec<Contour>> {
    let doc = Document::parse(svg)?;
    let root = doc.root_element();
    let (sx, sy) = physically_scaled_svg(root)?;
    let mut ties = Vec::new();
    for node in root
        .descendants()
        .filter(|node| node.attribute("class") == Some("TieSegment"))
    {
        ensure!(
            node.parent() == Some(root)
                && node.has_tag_name(("http://www.w3.org/2000/svg", "path"))
                && node.attributes().all(|a| a.namespace().is_none()
                    && matches!(
                        a.name(),
                        "class"
                            | "stroke"
                            | "stroke-width"
                            | "stroke-linecap"
                            | "stroke-linejoin"
                            | "fill-rule"
                            | "fill"
                            | "d"
                    ))
                && node.attribute("stroke-linecap") == Some("round")
                && node.attribute("stroke-linejoin") == Some("round")
                && node.attribute("fill-rule") == Some("evenodd"),
            "unsupported SVG tie paint style"
        );
        let required = |key| {
            node.attribute(key)
                .ok_or_else(|| anyhow::anyhow!("missing SVG tie attribute {key}"))
        };
        let stroke = svg_rgb(required("stroke")?)?;
        let fill = svg_rgb(node.attribute("fill").unwrap_or("#000000"))?;
        ensure!(stroke == fill, "unequal SVG tie fill and stroke");
        let width = pdf_number(required("stroke-width")?)? * sx;
        let data = required("d")?;
        let mut points = Vec::new();
        let mut remaining = data;
        for (command, count) in [('M', 1), ('C', 3), ('C', 3)] {
            remaining = remaining
                .trim_start()
                .strip_prefix(command)
                .ok_or_else(|| anyhow::anyhow!("unsupported SVG tie command"))?;
            let end = remaining
                .find(|character: char| character.is_ascii_alphabetic())
                .unwrap_or(remaining.len());
            let coordinates = remaining[..end]
                .split(|c: char| c == ',' || c.is_ascii_whitespace())
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>();
            ensure!(
                coordinates.len() == count * 2,
                "wrong SVG tie command arity"
            );
            for &[x, y] in coordinates.as_chunks::<2>().0 {
                points.push((pdf_number(x)? * sx, pdf_number(y)? * sy));
            }
            remaining = &remaining[end..];
        }
        ensure!(
            remaining.trim().is_empty()
                && width.is_finite()
                && width > 0.0
                && points[0] == points[6]
                && points[0] != points[3],
            "malformed SVG tie"
        );
        View::around(&points, 0.0)?;
        ties.push(Contour {
            points,
            width,
            stroke,
            fill: Some(fill),
            cap: 1.0,
            join: 1.0,
        });
    }
    Ok(ties)
}

pub(super) fn svg_stems(svg: &str) -> Result<Vec<Contour>> {
    let doc = Document::parse(svg)?;
    let root = doc.root_element();
    let (sx, sy) = physically_scaled_svg(root)?;
    let mut stems = Vec::new();
    for node in root
        .descendants()
        .filter(|node| node.attribute("class") == Some("Stem"))
    {
        ensure!(
            node.parent() == Some(root)
                && node.has_tag_name(("http://www.w3.org/2000/svg", "polyline"))
                && node.attributes().all(|a| a.namespace().is_none()
                    && matches!(
                        a.name(),
                        "class" | "stroke" | "stroke-width" | "stroke-linejoin" | "fill" | "points"
                    ))
                && node.attribute("stroke-linejoin") == Some("bevel")
                && node.attribute("fill") == Some("none"),
            "unsupported SVG stem paint style"
        );
        let required = |key| {
            node.attribute(key)
                .ok_or_else(|| anyhow::anyhow!("missing SVG stem attribute {key}"))
        };
        let coordinates = required("points")?
            .split(|c: char| c == ',' || c.is_ascii_whitespace())
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>();
        let [a, b, c, d] = numbers(&coordinates)?;
        ensure!(a == c && b != d, "unsupported SVG stem direction");
        let points = top_first((a * sx, b * sy), (c * sx, d * sy)).to_vec();
        View::around(&points, 0.0)?;
        let width = pdf_number(required("stroke-width")?)? * sx;
        ensure!(width.is_finite() && width > 0.0, "invalid SVG stem width");
        let stroke = svg_rgb(required("stroke")?)?;
        stems.push(Contour {
            points,
            width,
            stroke,
            fill: None,
            cap: 0.0,
            join: 2.0,
        });
    }
    Ok(stems)
}

fn matches_contour(expected: &Contour, actual: &Contour, margin: f64) -> bool {
    expected.points.len() == actual.points.len()
        && !expected.points.is_empty()
        && expected.fill.is_some() == actual.fill.is_some()
        && expected.cap == actual.cap
        && expected.join == actual.join
        && (expected.width - actual.width).abs() < margin
        && expected
            .points
            .iter()
            .zip(&actual.points)
            .all(|(&(x, y), &(xx, yy))| (x - xx).hypot(y - yy) < margin)
}

pub(super) fn bind_contours(
    expected: &[Contour],
    actual: &[Contour],
    margin_pt: f64,
) -> Result<Vec<usize>> {
    ensure!(
        !expected.is_empty()
            && expected.len() == actual.len()
            && margin_pt.is_finite()
            && margin_pt > 0.0,
        "missing/excess contours or invalid margin"
    );
    let mut used = vec![false; actual.len()];
    let mut binding = Vec::with_capacity(expected.len());
    for contour in expected {
        let mut candidates = actual
            .iter()
            .enumerate()
            .filter(|(_, ink)| matches_contour(contour, ink, margin_pt));
        let (index, _) = candidates
            .next()
            .ok_or_else(|| anyhow::anyhow!("unmatched PDF contour"))?;
        ensure!(
            candidates.next().is_none() && !used[index],
            "ambiguous PDF contour"
        );
        used[index] = true;
        binding.push(index);
    }
    Ok(binding)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESERVE: f64 = 0.015;
    const PROFILE: &str = "/GSa";
    const PAGE: &str = "q
        /GSa gs /CSp cs /CSp CS 0.06 0 0 -0.06 0 200 cm
        0 0 m 3000 0 l 3000 3100 l 0 3100 l 0 0 l h W* n
        q 4 w 1 J 1 j [] 0 d
        1000 825 m 1200 750 1600 750 1800 825 c
        1600 775 1200 775 1000 825 c h B* Q
        q 8 w 0 J 2 j [ ] 0 d 2000 1200 m 2000 1000 l S Q
        BT /F12 331 Tf 1 0 0 -1 0 0 Tm 0 0 Td <0001> Tj ET
        Q";

    fn read(content: &str) -> Result<InkPage> {
        read_ink(content.as_bytes(), 200.0, 200.0, PROFILE, RESERVE)
    }

    #[test]
    fn pdf_contour_rejects_mutations_despite_same_endpoints_and_envelope() {
        let genuine = read(PAGE).unwrap();
        let moved = read(&PAGE.replace("1600 775 1200 775", "1600 800 1200 800")).unwrap();
        let a = &genuine.ties[0];
        let b = &moved.ties[0];
        assert_eq!(
            [a.points[0], a.points[3], a.points[6]],
            [b.points[0], b.points[3], b.points[6]]
        );
        assert_eq!(
            View::around(&a.points, 0.0).unwrap().0,
            View::around(&b.points, 0.0).unwrap().0
        );
        assert_eq!(
            bind_contours(&genuine.ties, &genuine.ties, RESERVE).unwrap(),
            [0]
        );
        assert!(bind_contours(&genuine.ties, &moved.ties, RESERVE).is_err());
        let ambiguity = vec![a.clone(), a.clone()];
        assert!(bind_contours(&ambiguity, &ambiguity, RESERVE).is_err());
    }

    #[test]
    fn pdf_contour_rejects_clip_state_and_unknown_operators() {
        let cases = [
            PAGE.replace("3000 0 l 3000 3100 l", "1500 0 l 1500 3100 l"),
            PAGE.replace("1 J 1 j", "0 J 1 j"),
            PAGE.replace("[] 0 d", "[1] 0 d"),
            PAGE.replace("0.06 0 0 -0.06", "0.06 0 0 -0.05"),
            PAGE.replace("/GSa gs", "/Unknown gs"),
            PAGE.replace("B* Q", "B* /Unknown Do Q"),
            PAGE.replace("h B* Q", "h B*"),
            PAGE.replace("1 J 1 j", "1 2 J 1 j"),
        ];
        for case in cases {
            assert!(read(&case).is_err(), "{case}");
        }
    }

    #[test]
    fn marked_pdf_retains_geometry_text_and_stroke_style() {
        let color = [49.0 / 255.0, 16.0 / 255.0, 7.0 / 255.0];
        let rgb = format!("{} {} {}", color[0], color[1], color[2]);
        let colored = PAGE.replace("q 4 w", &format!("q {rgb} scn {rgb} SCN 4 w"));
        same_except_rgb(PAGE.as_bytes(), colored.as_bytes()).unwrap();
        let plain = read(PAGE).unwrap();
        let colored_ink = read(&colored).unwrap();
        assert_eq!(colored_ink.ties[0].stroke, color);
        assert_eq!(colored_ink.ties[0].fill, Some(color));
        bind_contours(&plain.ties, &colored_ink.ties, RESERVE).unwrap();
        assert_eq!(plain.strokes[0].points[0], (120.0, 60.0));
        assert!(same_except_rgb(PAGE.as_bytes(), PAGE.replace("8 w", "9 w").as_bytes()).is_err());
        assert!(
            same_except_rgb(PAGE.as_bytes(), PAGE.replace("<0001>", "<0002>").as_bytes()).is_err()
        );
    }
}
