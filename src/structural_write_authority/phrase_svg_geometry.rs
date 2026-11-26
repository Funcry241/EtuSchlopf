// Read-only contours from the same MuseScore converter used for the PDF.
// Accept the flat, absolute-coordinate vector format we can verify completely.
use roxmltree::{Document, Node};

#[path = "phrase_paint_geometry.rs"]
pub(crate) mod paint;

#[path = "../audit_only/engraving_collision.rs"]
pub(crate) mod collision_observation;

const FLATNESS: f64 = 1.0 / 128.0;
const GRID: f64 = 4096.0;
// Euclidean allowance for snapping a vertex and evaluating the subdivision.
const CONTOUR_ERROR: f64 = FLATNESS + 1.0 / GRID + 0.000_001;
// Reserve for the renderer's rounded preview coordinates.
const PREVIEW_RESERVE: f64 = 0.16;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn mid(self, other: Self) -> Self {
        Self {
            x: (self.x + other.x) * 0.5,
            y: (self.y + other.y) * 0.5,
        }
    }

    fn grid(self) -> Option<GridPoint> {
        let x = (self.x * GRID).round();
        let y = (self.y * GRID).round();
        let coordinates = f64::from(i32::MIN)..=f64::from(i32::MAX);
        (coordinates.contains(&x) && coordinates.contains(&y)).then(|| GridPoint {
            x: i64::from(x as i32),
            y: i64::from(y as i32),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct GridPoint {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Bounds {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl Bounds {
    fn from_points(points: impl Iterator<Item = Point>, radius: f64) -> Option<Self> {
        let mut points = points.peekable();
        let first = *points.peek()?;
        let mut b = Self {
            left: first.x,
            right: first.x,
            top: first.y,
            bottom: first.y,
        };
        for p in points {
            b.left = b.left.min(p.x);
            b.right = b.right.max(p.x);
            b.top = b.top.min(p.y);
            b.bottom = b.bottom.max(p.y);
        }
        b.left -= radius;
        b.right += radius;
        b.top -= radius;
        b.bottom += radius;
        Some(b)
    }

    fn overlaps(self, other: Self, pad: f64) -> bool {
        self.left <= other.right + pad
            && self.right + pad >= other.left
            && self.top <= other.bottom + pad
            && self.bottom + pad >= other.top
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum Side {
    Above,
    Below,
}

impl Side {
    pub(crate) fn opposite(self) -> Self {
        match self {
            Self::Above => Self::Below,
            Self::Below => Self::Above,
        }
    }

    pub(crate) fn attribute(self) -> &'static str {
        match self {
            Self::Above => " placement=\"above\"",
            Self::Below => " placement=\"below\"",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Command {
    Move(Point),
    Line(Point),
    Cubic([Point; 3]),
}

impl Command {
    fn points(&self) -> &[Point] {
        match self {
            Self::Move(p) | Self::Line(p) => std::slice::from_ref(p),
            Self::Cubic(points) => points,
        }
    }
}

struct Shape {
    class: String,
    paint: Vec<(String, String)>,
    commands: Vec<Command>,
    contours: Vec<Vec<GridPoint>>,
    bounds: Bounds,
    stroke_scale: [f64; 2],
    half_stroke_width: f64,
    radius: f64,
    round_inner_radius: Option<f64>,
    filled: bool,
    even_odd: bool,
}

impl Shape {
    fn read(node: Node<'_, '_>) -> Option<Self> {
        if node.children().any(|child| child.is_element()) {
            return None;
        }
        let class = node.attribute("class")?.to_owned();
        let paint = node
            .attributes()
            .filter(|a| !matches!(a.name(), "d" | "points" | "class" | "transform"))
            .map(|a| (a.name().to_owned(), a.value().to_owned()))
            .collect::<Vec<_>>();
        if paint.iter().any(|(name, _)| {
            !matches!(
                name.as_str(),
                "fill"
                    | "fill-rule"
                    | "stroke"
                    | "stroke-width"
                    | "stroke-linecap"
                    | "stroke-linejoin"
                    | "stroke-dasharray"
                    | "stroke-dashoffset"
                    | "stroke-miterlimit"
            )
        }) {
            return None;
        }
        let mut filled = node.attribute("fill") != Some("none")
            && node.has_tag_name(("http://www.w3.org/2000/svg", "path"));
        let width = match node.attribute("stroke") {
            None | Some("none") => 0.0,
            Some(_) => number(node.attribute("stroke-width")?)?,
        };
        if width < 0.0 {
            return None;
        }
        let even_odd = match node.attribute("fill-rule") {
            Some("evenodd") => true,
            Some("nonzero") | None => false,
            _ => return None,
        };
        let [sx, sy, tx, ty] = if let Some(transform) = node.attribute("transform") {
            let values = transform
                .strip_prefix("matrix(")?
                .strip_suffix(')')?
                .split(',')
                .map(number)
                .collect::<Option<Vec<_>>>()?;
            let [sx, 0.0, 0.0, sy, tx, ty] = values.as_slice() else {
                return None;
            };
            [*sx, *sy, *tx, *ty]
        } else {
            [1.0, 1.0, 0.0, 0.0]
        };
        if sx <= 0.0 || sy <= 0.0 {
            return None;
        }
        let half_stroke_width = width * sx.max(sy) * 0.5;
        let mut commands = match node.tag_name().name() {
            "path" => read_path(node.attribute("d")?)?,
            "polyline" => node
                .attribute("points")?
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, token)| {
                    point(token).map(|p| {
                        if i == 0 {
                            Command::Move(p)
                        } else {
                            Command::Line(p)
                        }
                    })
                })
                .collect::<Option<Vec<_>>>()?,
            _ => return None,
        };
        for command in &mut commands {
            for p in match command {
                Command::Move(p) | Command::Line(p) => std::slice::from_mut(p),
                Command::Cubic(points) => points,
            } {
                *p = Point {
                    x: p.x * sx + tx,
                    y: p.y * sy + ty,
                };
                p.grid()?;
            }
        }
        let cap = match node.attribute("stroke-linecap") {
            None | Some("butt" | "round") => 1.0,
            Some("square") => 1.5,
            _ => return None,
        };
        let join = match node.attribute("stroke-linejoin") {
            Some("round" | "bevel") => 1.0,
            None | Some("miter") if commands.len() > 2 => {
                let limit = node
                    .attribute("stroke-miterlimit")
                    .map_or(Some(4.0), number)?;
                if limit < 1.0 {
                    return None;
                }
                limit * 2.0
            }
            None | Some("miter") => 1.0,
            _ => return None,
        };
        let radius = half_stroke_width * f64::max(cap, join);
        let closed = matches!(commands.first(), Some(Command::Move(p)) if commands.last().and_then(|c| c.points().last()) == Some(p));
        let round_inner_radius = ((!filled || closed)
            && node.attribute("stroke-linecap") == Some("round")
            && node.attribute("stroke-linejoin") == Some("round")
            && node.attribute("stroke-dasharray").is_none())
        .then(|| half_stroke_width * (sx.min(sy) / sx.max(sy)));
        let bounds =
            Bounds::from_points(commands.iter().flat_map(Command::points).copied(), radius)?;
        // Axis-aligned butt strokes are rectangles. Their caps stop at the
        // given endpoints, which matters when a stem just touches a bow.
        let (contours, stroke_expansion) = if !filled
            && width > 0.0
            && matches!(node.attribute("stroke-linecap"), None | Some("butt"))
            && node.attribute("stroke-dasharray").is_none()
            && let [Command::Move(a), Command::Line(b)] = commands.as_slice()
            && ((a.x == b.x) != (a.y == b.y))
        {
            let dx = if a.x == b.x { width * sx * 0.5 } else { 0.0 };
            let dy = if a.y == b.y { width * sy * 0.5 } else { 0.0 };
            let left = a.x.min(b.x) - dx;
            let right = a.x.max(b.x) + dx;
            let top = a.y.min(b.y) - dy;
            let bottom = a.y.max(b.y) + dy;
            let rectangle = [
                Point { x: left, y: top },
                Point { x: right, y: top },
                Point {
                    x: right,
                    y: bottom,
                },
                Point { x: left, y: bottom },
                Point { x: left, y: top },
            ];
            filled = true;
            (
                vec![
                    rectangle
                        .into_iter()
                        .map(Point::grid)
                        .collect::<Option<_>>()?,
                ],
                None,
            )
        } else {
            (contours(&commands, filled)?, Some(radius))
        };
        Some(Self {
            class,
            paint,
            commands,
            contours,
            bounds,
            stroke_scale: [width * sx, width * sy],
            half_stroke_width,
            radius: stroke_expansion.unwrap_or_default(),
            round_inner_radius,
            filled,
            even_odd,
        })
    }

    fn side(&self) -> Option<Side> {
        let [
            Command::Move(start),
            Command::Cubic([first, second, end]),
            Command::Cubic(_),
        ] = self.commands.as_slice()
        else {
            return None;
        };
        let width = end.x - start.x;
        if width <= 0.0 {
            return None;
        }
        let offset = |p: &Point| p.y - (start.y + (end.y - start.y) * ((p.x - start.x) / width));
        match (offset(first), offset(second)) {
            (a, b) if a < -PREVIEW_RESERVE && b < -PREVIEW_RESERVE => Some(Side::Above),
            (a, b) if a > PREVIEW_RESERVE && b > PREVIEW_RESERVE => Some(Side::Below),
            _ => None,
        }
    }

    fn guide(&self) -> bool {
        // Guide lines are intended to cross other ink. Their geometry and
        // ordering are checked separately from engraving-ink contacts.
        matches!(
            self.class.as_str(),
            "" | "StaffLines" | "BarLine" | "Bracket"
        )
    }

    fn stroke_width(&self) -> f64 {
        self.stroke_scale[0].max(self.stroke_scale[1])
    }

    fn same_ink(&self, other: &Self) -> bool {
        self.class == other.class
            && self.commands == other.commands
            && self.paint == other.paint
            && self.stroke_scale == other.stroke_scale
    }

    fn extent(&self) -> Option<(Point, Point)> {
        match self.commands.as_slice() {
            [Command::Move(first), Command::Line(last)] => Some((*first, *last)),
            _ => None,
        }
    }
}

pub(crate) struct Page {
    format: Vec<(String, String)>,
    shapes: Vec<Shape>,
}

impl Page {
    pub(crate) fn read(svg: &str) -> Option<Self> {
        let doc = Document::parse(svg).ok()?;
        let root = doc.root_element();
        if !root.has_tag_name(("http://www.w3.org/2000/svg", "svg")) {
            return None;
        }
        let format = root
            .attributes()
            .map(|a| (a.name().to_owned(), a.value().to_owned()))
            .collect::<Vec<_>>();
        let shapes = root
            .children()
            .filter(|n| n.is_element() && !matches!(n.tag_name().name(), "title" | "desc"))
            .map(Shape::read)
            .collect::<Option<Vec<_>>>()?;
        Some(Self { format, shapes })
    }

    pub(crate) fn system_count(&self, staff_count: usize) -> Option<usize> {
        Some(self.systems(staff_count)?.len() + 1)
    }

    /// Stroke endpoints and width in PDF points, ordered along the axis.
    pub(crate) fn native_export_stroke_anchors(&self) -> Option<Vec<[f64; 5]>> {
        let attribute = |name: &str| {
            self.format.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
        };
        let dimensions = attribute("viewBox")?.split_ascii_whitespace()
            .map(number).collect::<Option<Vec<_>>>()?;
        let [0.0, 0.0, width, height] = dimensions.as_slice() else { return None };
        if *width <= 0.0 || *height <= 0.0 {
            return None;
        }
        let points_per_mm = 72.0 / 25.4;
        let sx = number(attribute("width")?.strip_suffix("mm")?)? * points_per_mm / width;
        let sy = number(attribute("height")?.strip_suffix("mm")?)? * points_per_mm / height;
        if sx <= 0.0 || sy <= 0.0 {
            return None;
        }
        self.shapes.iter().filter(|shape| matches!(shape.class.as_str(), "Stem" | "StaffLines"))
            .map(|shape| {
                let (a, b) = shape.extent()?;
                let vertical = shape.class == "Stem";
                if (vertical && (a.x != b.x || a.y == b.y))
                    || (!vertical && (a.y != b.y || a.x == b.x))
                    || shape.stroke_scale[0] <= 0.0
                    || shape.stroke_scale[0] != shape.stroke_scale[1]
                    || shape.paint.iter().any(|(name, _)| name == "stroke-dasharray")
                {
                    return None;
                }
                let (lo, hi) = if (a.x, a.y) <= (b.x, b.y) { (a, b) } else { (b, a) };
                Some([lo.x * sx, lo.y * sy, hi.x * sx, hi.y * sy, shape.stroke_scale[0] * (sx * sy).sqrt()])
            }).collect()
    }

    fn systems(&self, staff_count: usize) -> Option<Vec<f64>> {
        let lines_per_system = staff_count.checked_mul(5)?;
        let staff_lines = self
            .shapes
            .iter()
            .filter(|s| s.class == "StaffLines")
            .map(|line| {
                let (a, b) = line.extent()?;
                (a.y == b.y && a.x < b.x).then_some(a.y)
            })
            .collect::<Option<Vec<_>>>()?;
        if lines_per_system == 0
            || staff_lines.is_empty()
            || staff_lines.len() % lines_per_system != 0
        {
            return None;
        }
        let mut staff_lines = staff_lines;
        staff_lines.sort_by(f64::total_cmp);
        if staff_lines.windows(2).any(|w| w[0] >= w[1]) {
            return None;
        }
        let mut bounds = Vec::new();
        for pair in staff_lines
            .chunks_exact(lines_per_system)
            .collect::<Vec<_>>()
            .windows(2)
        {
            bounds.push((pair[0][lines_per_system - 1] + pair[1][0]) * 0.5);
        }
        Some(bounds)
    }
}

fn number(text: &str) -> Option<f64> {
    let value = text.parse::<f64>().ok()?;
    (value.is_finite() && value.abs() <= 1_000_000.0).then_some(value)
}

fn point(text: &str) -> Option<Point> {
    let (x, y) = text.split_once(',')?;
    Some(Point {
        x: number(x)?,
        y: number(y)?,
    })
}

fn read_path(text: &str) -> Option<Vec<Command>> {
    let mut tokens = text.split_ascii_whitespace();
    let mut commands = Vec::new();
    while let Some(token) = tokens.next() {
        let (command, rest) = token.split_at_checked(1)?;
        let p = point(rest)?;
        commands.push(match command {
            "M" => Command::Move(p),
            "L" => Command::Line(p),
            "C" => Command::Cubic([p, point(tokens.next()?)?, point(tokens.next()?)?]),
            _ => return None,
        });
    }
    Some(commands)
}

fn flatten(out: &mut Vec<GridPoint>, curve: [Point; 4], depth: u8) -> Option<()> {
    let [a, b, c, d] = curve;
    // Degree-elevate the chord to compare corresponding points at the same t.
    // The norm of a convex combination of the control-point differences is
    // bounded by the largest norm among them.
    let error = |p: Point, fraction: f64| {
        let x = p.x - (a.x + (d.x - a.x) * fraction);
        let y = p.y - (a.y + (d.y - a.y) * fraction);
        x * x + y * y
    };
    if error(b, 1.0 / 3.0).max(error(c, 2.0 / 3.0)) <= FLATNESS * FLATNESS {
        out.push(d.grid()?);
        return Some(());
    }
    if depth >= 18 {
        return None;
    }
    let ab = a.mid(b);
    let bc = b.mid(c);
    let cd = c.mid(d);
    let left = ab.mid(bc);
    let right = bc.mid(cd);
    let center = left.mid(right);
    flatten(out, [a, ab, left, center], depth + 1)?;
    flatten(out, [center, right, cd, d], depth + 1)
}

fn contours(commands: &[Command], filled: bool) -> Option<Vec<Vec<GridPoint>>> {
    let mut contours = Vec::<Vec<GridPoint>>::new();
    let mut start = None;
    for command in commands {
        let p = match *command {
            Command::Move(p) => {
                contours.push(vec![p.grid()?]);
                p
            }
            Command::Line(p) => {
                contours.last_mut()?.push(p.grid()?);
                p
            }
            Command::Cubic([b, c, d]) => {
                flatten(contours.last_mut()?, [start?, b, c, d], 0)?;
                d
            }
        };
        start = Some(p);
    }
    for contour in &mut contours {
        if contour.len() < 2 {
            return None;
        }
        if filled && contour.first() != contour.last() {
            contour.push(contour[0]);
        }
    }
    Some(contours)
}

fn orient(a: GridPoint, b: GridPoint, c: GridPoint) -> i128 {
    i128::from(b.x - a.x) * i128::from(c.y - a.y) - i128::from(b.y - a.y) * i128::from(c.x - a.x)
}

// Differences and single products of signed grid coordinates fit in i128.
// Refuse an indeterminate comparison if a squared cross product overflows.
fn point_near_segment(
    p: GridPoint,
    a: GridPoint,
    b: GridPoint,
    limit_squared: i128,
) -> Option<bool> {
    let dx = i128::from(b.x - a.x);
    let dy = i128::from(b.y - a.y);
    let nx = i128::from(p.x - a.x);
    let ny = i128::from(p.y - a.y);
    let length = dx * dx + dy * dy;
    let dot = nx * dx + ny * dy;
    if dot <= 0 {
        Some(nx * nx + ny * ny <= limit_squared)
    } else if dot >= length {
        Some((nx - dx).pow(2) + (ny - dy).pow(2) <= limit_squared)
    } else {
        match (
            (dx * ny - dy * nx).checked_pow(2),
            limit_squared.checked_mul(length),
        ) {
            (Some(a), Some(b)) => Some(a <= b),
            (Some(_), None) => Some(true),
            (None, Some(_)) => Some(false),
            (None, None) => None,
        }
    }
}

fn segments_close(a: &[GridPoint], b: &[GridPoint], limit: i64) -> bool {
    let [p, q] = [a[0], a[1]];
    let [r, s] = [b[0], b[1]];
    let overlap = |p: i64, q: i64, r: i64, s: i64| {
        p.min(q) <= r.max(s) + limit && r.min(s) <= p.max(q) + limit
    };
    if !overlap(p.x, q.x, r.x, s.x) || !overlap(p.y, q.y, r.y, s.y) {
        return false;
    }
    let opposite = |a: i128, b: i128| a != 0 && b != 0 && a.signum() != b.signum();
    if opposite(orient(p, q, r), orient(p, q, s)) && opposite(orient(r, s, p), orient(r, s, q)) {
        return true;
    }
    let squared = i128::from(limit).pow(2);
    point_near_segment(p, r, s, squared).unwrap_or(true)
        || point_near_segment(q, r, s, squared).unwrap_or(true)
        || point_near_segment(r, p, q, squared).unwrap_or(true)
        || point_near_segment(s, p, q, squared).unwrap_or(true)
}

fn edges(shape: &Shape) -> impl Iterator<Item = &[GridPoint]> {
    shape.contours.iter().flat_map(|contour| contour.windows(2))
}

fn contains(shape: &Shape, p: GridPoint) -> bool {
    if !shape.filled {
        return false;
    }
    let winding = edges(shape)
        .map(|edge| {
            let [a, b] = [edge[0], edge[1]];
            if a.y <= p.y && p.y < b.y && orient(a, b, p) > 0 {
                1_i64
            } else if b.y <= p.y && p.y < a.y && orient(a, b, p) < 0 {
                -1
            } else {
                0
            }
        })
        .sum::<i64>();
    if shape.even_odd {
        winding % 2 != 0
    } else {
        winding != 0
    }
}

fn possible_contact(a: &Shape, b: &Shape, minimum: f64) -> bool {
    let allowance = minimum + 2.0 * CONTOUR_ERROR + PREVIEW_RESERVE;
    if !a.bounds.overlaps(b.bounds, allowance) {
        return false;
    }
    let distance = a.radius + b.radius + allowance;
    if distance * GRID >= f64::from(i32::MAX) {
        return true;
    }
    let limit = (distance * GRID).ceil() as i64;
    a.contours.iter().any(|c| contains(b, c[0]))
        || b.contours.iter().any(|c| contains(a, c[0]))
        || edges(a).any(|edge| edges(b).any(|other| segments_close(edge, other, limit)))
}

fn witnessed_contact(a: &Shape, b: &Shape) -> bool {
    if !a.bounds.overlaps(b.bounds, 0.0) {
        return false;
    }
    // A sample strictly inside the other contour by more than the total
    // approximation error is a witness of contact for the original paths.
    let witness = |a: &Shape, b: &Shape| {
        if !a.filled && a.round_inner_radius.unwrap_or_default() == 0.0 {
            return false;
        }
        let error = 2.0 * CONTOUR_ERROR + PREVIEW_RESERVE + 2.0 / GRID;
        let inset = (error * GRID).ceil() as i128;
        let pen = ((b.round_inner_radius.unwrap_or_default() - error) * GRID)
            .clamp(0.0, f64::from(i32::MAX))
            .floor() as i128;
        let midpoints = edges(a).map(|edge| GridPoint {
            x: (edge[0].x + edge[1].x) / 2,
            y: (edge[0].y + edge[1].y) / 2,
        });
        a.contours
            .iter()
            .flatten()
            .copied()
            .chain(midpoints)
            .any(|p| {
                (contains(b, p)
                    && !edges(b).any(|edge| {
                        point_near_segment(p, edge[0], edge[1], inset.pow(2)).unwrap_or(true)
                    }))
                    || (pen > 0
                        && edges(b).any(|edge| {
                            point_near_segment(p, edge[0], edge[1], pen.pow(2)).unwrap_or(false)
                        }))
            })
    };
    witness(a, b) || witness(b, a)
}

// A tagging export must preserve every drawing, including order. The only
// permitted differences are the two paint attributes of the tagged slur.
pub(crate) fn marked_segments(
    baseline: &[Page],
    tagged: &[Page],
    color: &str,
    staves: usize,
    systems: &[(usize, usize)],
) -> Option<Vec<(usize, usize)>> {
    if baseline.len() != tagged.len() {
        return None;
    }
    let mut segments = Vec::new();
    let mut observed = Vec::new();
    for (p, (a, b)) in baseline.iter().zip(tagged).enumerate() {
        if a.format != b.format || a.shapes.len() != b.shapes.len() {
            return None;
        }
        let boundaries = a.systems(staves)?;
        for (i, (a, b)) in a.shapes.iter().zip(&b.shapes).enumerate() {
            if a.class != b.class || a.commands != b.commands || a.stroke_scale != b.stroke_scale {
                return None;
            }
            let colors = b
                .paint
                .iter()
                .filter(|(key, value)| {
                    matches!(key.as_str(), "fill" | "stroke") && value.eq_ignore_ascii_case(color)
                })
                .count();
            if colors == 0 && a.paint == b.paint {
                continue;
            }
            if colors != 2
                || a.class != "SlurSegment"
                || a.paint
                    .iter()
                    .any(|(_, value)| value.eq_ignore_ascii_case(color))
            {
                return None;
            }
            let noncolor =
                |(key, _): &&(String, String)| !matches!(key.as_str(), "fill" | "stroke");
            if !a
                .paint
                .iter()
                .filter(noncolor)
                .eq(b.paint.iter().filter(noncolor))
            {
                return None;
            }
            let top = boundaries.partition_point(|y| *y < a.bounds.top);
            let bottom = boundaries.partition_point(|y| *y < a.bounds.bottom);
            if top != bottom {
                return None;
            }
            segments.push(((p, top), (p, i)));
            observed.push((p, top));
        }
    }
    observed.sort_unstable();
    segments.sort_unstable();
    (observed == systems).then(|| segments.into_iter().map(|(_, segment)| segment).collect())
}

pub(crate) fn side(pages: &[Page], segments: &[(usize, usize)]) -> Option<Side> {
    let &(p, i) = segments.first()?;
    let side = pages[p].shapes[i].side()?;
    segments
        .iter()
        .all(|&(p, i)| pages[p].shapes[i].side() == Some(side))
        .then_some(side)
}

pub(crate) fn contacts(pages: &[Page], segments: &[(usize, usize)]) -> usize {
    segments
        .iter()
        .map(|&(p, i)| {
            let shapes = &pages[p].shapes;
            shapes
                .iter()
                .enumerate()
                .filter(|(j, s)| *j != i && !s.guide() && witnessed_contact(&shapes[i], s))
                .count()
        })
        .sum()
}

pub(crate) fn clear(pages: &[Page], segments: &[(usize, usize)]) -> bool {
    segments.iter().all(|&(p, i)| {
        let shapes = &pages[p].shapes;
        let minimum = shapes
            .iter()
            .filter(|shape| shape.class == "StaffLines")
            .map(|shape| shape.stroke_width())
            .max_by(f64::total_cmp);
        minimum.is_some_and(|minimum| {
            minimum > 0.0
                && shapes.iter().enumerate().all(|(j, other)| {
                    i == j || other.guide() || !possible_contact(&shapes[i], other, minimum)
                })
        })
    })
}

pub(crate) fn neighborhood_preserved(
    before: &[Page],
    after: &[Page],
    before_segments: &[(usize, usize)],
    after_segments: &[(usize, usize)],
) -> bool {
    before.len() == after.len()
        && before_segments == after_segments
        && before.iter().zip(after).enumerate().all(|(page, (a, b))| {
            a.format == b.format
                && a.shapes.len() == b.shapes.len()
                && a.shapes
                    .iter()
                    .zip(&b.shapes)
                    .enumerate()
                    .all(|(index, (x, y))| {
                        before_segments.contains(&(page, index)) || x.same_ink(y)
                    })
        })
}

pub(crate) fn planned_segments_match(
    candidate: &[Page],
    combined: &[Page],
    segments: &[(usize, usize)],
) -> bool {
    segments.iter().all(|&(page, index)| {
        combined
            .get(page)
            .and_then(|p| p.shapes.get(index))
            .is_some_and(|shape| candidate[page].shapes[index].same_ink(shape))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(curve: &str, color: &str, beam_y: usize, neighbor_y: usize) -> Page {
        let staff = (20..=60).step_by(10).map(|y| {
            format!(r##"<polyline class="StaffLines" fill="none" stroke="#000" stroke-width="1" points="10,{y} 90,{y}"/>"##)
        }).collect::<String>();
        let beam_bottom = beam_y + 3;
        Page::read(&format!(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            {staff}
            <path class="SlurSegment" fill="{color}" stroke="{color}" stroke-width="0.5" d="{curve}"/>
            <path class="Beam" fill="#000" d="M45,{beam_y} L55,{beam_y} L55,{beam_bottom} L45,{beam_bottom} L45,{beam_y}"/>
            <polyline class="Stem" fill="none" stroke="#000" stroke-width="1" points="50,{neighbor_y} 54,{neighbor_y}"/>
            </svg>"##)).unwrap()
    }

    const DOWN: &str = "M20,38 C35,70 65,70 80,38 C65,74 35,74 20,38";
    const UP: &str = "M20,38 C35,4 65,4 80,38 C65,8 35,8 20,38";
    const MARK: &str = "#2173c5";

    #[test]
    fn curve_contact_and_neighbor_contacts_are_independent() {
        let baseline = [render(DOWN, "#000", 63, 77)];
        let tagged = [render(DOWN, MARK, 63, 77)];
        let segments = marked_segments(&baseline, &tagged, MARK, 1, &[(0, 0)]).unwrap();
        assert!(contacts(&baseline, &segments) > 0);
        assert!(side(&baseline, &segments) == Some(Side::Below));

        let free = [render(UP, "#000", 63, 77)];
        assert!(clear(&free, &segments));
        assert!(neighborhood_preserved(
            &baseline, &free, &segments, &segments
        ));

        let neighbor_hits_beam = [render(UP, "#000", 63, 64)];
        assert!(clear(&neighbor_hits_beam, &segments));
        assert!(!neighborhood_preserved(
            &baseline,
            &neighbor_hits_beam,
            &segments,
            &segments
        ));
    }

    #[test]
    fn missing_segments_and_changes_outside_the_marker_are_rejected() {
        let baseline = [render(DOWN, "#000", 63, 77), render(DOWN, "#000", 63, 77)];
        let expected = [(0, 0), (1, 0)];
        let two_segments = [render(DOWN, MARK, 63, 77), render(DOWN, MARK, 63, 77)];
        assert!(marked_segments(&baseline, &two_segments, MARK, 1, &expected).is_some());

        let missing = [render(DOWN, MARK, 63, 77), render(DOWN, "#000", 63, 77)];
        assert!(marked_segments(&baseline, &missing, MARK, 1, &expected).is_none());

        let moved = [render(DOWN, MARK, 63, 78), render(DOWN, MARK, 63, 77)];
        assert!(marked_segments(&baseline, &moved, MARK, 1, &expected).is_none());
    }

    #[test]
    fn clearance_uses_filled_ink_and_staff_line_width() {
        let render = |y: f64, stroke: f64| {
            Page::read(&format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg">
                <polyline class="StaffLines" fill="none" stroke="black"
                    stroke-width="{stroke}" points="0,0 40,0"/>
                <path class="SlurSegment" d="M10,10 L20,10 L20,11 L10,11 L10,10"/>
                <path class="Beam" d="M10,{y} L20,{y} L20,20 L10,20 L10,{y}"/>
                </svg>"#,
            ))
            .unwrap()
        };
        assert_eq!(contacts(&[render(11.1, 1.0)], &[(0, 1)]), 0);
        assert!(!clear(&[render(11.1, 1.0)], &[(0, 1)]));
        assert!(clear(&[render(12.5, 1.0)], &[(0, 1)]));
        assert!(!clear(&[render(12.5, 2.0)], &[(0, 1)]));
        assert_eq!(contacts(&[render(10.5, 1.0)], &[(0, 1)]), 1);
    }

    #[test]
    fn combined_preview_checks_selected_ink_and_unrelated_shapes() {
        let page = |first: usize, second: usize, note: usize| {
            Page::read(&format!(
                r#"<svg xmlns="http://www.w3.org/2000/svg">
                <path class="SlurSegment" d="M0,{first} L3,1 L7,1 L0,{first}"/>
                <path class="SlurSegment" d="M0,{second} L3,9 L7,9 L0,{second}"/>
                <path class="Note" d="M1,{note} L4,5 L3,6 L1,{note}"/>
                </svg>"#,
            ))
            .unwrap()
        };
        let before = [page(1, 9, 5)];
        let first = [page(2, 9, 5)];
        let second = [page(1, 8, 5)];
        let combined = [page(2, 8, 5)];
        let segments = [(0, 0), (0, 1)];
        assert!(neighborhood_preserved(
            &before, &combined, &segments, &segments
        ));
        assert!(planned_segments_match(&first, &combined, &[(0, 0)]));
        assert!(planned_segments_match(&second, &combined, &[(0, 1)]));
        assert!(!planned_segments_match(
            &second,
            &[page(2, 7, 5)],
            &[(0, 1)]
        ));
        assert!(!neighborhood_preserved(
            &before,
            &[page(2, 8, 6)],
            &segments,
            &segments
        ));
    }
}

#[path = "rest_svg_geometry.rs"]
pub(crate) mod rests;
