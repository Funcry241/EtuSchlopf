// Read-only contours from the same MuseScore converter used for the PDF.
// Accept the flat, absolute-coordinate vector format we can verify completely.
use roxmltree::{Document, Node};

const FLATNESS: f64 = 0.06;
// SVG rounds coordinates on the page to six significant digits.
const ROUNDING: f64 = 0.16;

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

    fn distance_squared(self, other: Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
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
pub(super) enum Side {
    Above,
    Below,
}

impl Side {
    pub(super) fn opposite(self) -> Self {
        match self {
            Self::Above => Self::Below,
            Self::Below => Self::Above,
        }
    }

    pub(super) fn attribute(self) -> &'static str {
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
    contours: Vec<Vec<Point>>,
    bounds: Bounds,
    radius: f64,
    filled: bool,
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
        let filled = node.attribute("fill") != Some("none")
            && node.has_tag_name(("http://www.w3.org/2000/svg", "path"));
        let width = match node.attribute("stroke") {
            None | Some("none") => 0.0,
            Some(_) => number(node.attribute("stroke-width")?)?,
        };
        if width < 0.0 {
            return None;
        }
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
            }
        }
        let contours = contours(&commands, filled)?;
        let bounds = Bounds::from_points(
            contours.iter().flatten().copied(),
            half_stroke_width + FLATNESS,
        )?;
        Some(Self {
            class,
            paint,
            commands,
            contours,
            bounds,
            radius: half_stroke_width,
            filled,
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
            (a, b) if a < -ROUNDING && b < -ROUNDING => Some(Side::Above),
            (a, b) if a > ROUNDING && b > ROUNDING => Some(Side::Below),
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

    fn vertical_translation(&self, other: &Self) -> Option<f64> {
        if self.class != other.class
            || self.paint != other.paint
            || self.radius != other.radius
            || self.commands.len() != other.commands.len()
        {
            return None;
        }
        let dy = other.commands.first()?.points().first()?.y
            - self.commands.first()?.points().first()?.y;
        self.commands
            .iter()
            .zip(&other.commands)
            .all(|(a, b)| {
                std::mem::discriminant(a) == std::mem::discriminant(b)
                    && a.points().iter().zip(b.points()).all(|(p, q)| {
                        (p.x - q.x).abs() <= ROUNDING && (p.y + dy - q.y).abs() <= ROUNDING
                    })
            })
            .then_some(dy)
    }

    fn extent(&self) -> Option<(Point, Point)> {
        match self.commands.as_slice() {
            [Command::Move(first), Command::Line(last)] => Some((*first, *last)),
            _ => None,
        }
    }
}

pub(super) struct Page {
    format: Vec<(String, String)>,
    shapes: Vec<Shape>,
}

impl Page {
    pub(super) fn read(svg: &str) -> Option<Self> {
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

    pub(super) fn same_ink(&self, other: &Self) -> bool {
        self.format == other.format
            && self.shapes.len() == other.shapes.len()
            && self.shapes.iter().zip(&other.shapes).all(|(a, b)| {
                a.class == b.class
                    && a.commands == b.commands
                    && a.paint == b.paint
                    && a.radius == b.radius
            })
    }

    pub(super) fn system_count(&self, staff_count: usize) -> Option<usize> {
        Some(self.systems(staff_count)?.len() + 1)
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

fn point_segment_distance_squared(p: Point, a: Point, b: Point) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let length = a.distance_squared(b);
    if length == 0.0 {
        return p.distance_squared(a);
    }
    let t = (((p.x - a.x) * dx + (p.y - a.y) * dy) / length).clamp(0.0, 1.0);
    p.distance_squared(Point {
        x: a.x + t * dx,
        y: a.y + t * dy,
    })
}

fn flatten(out: &mut Vec<Point>, curve: [Point; 4], depth: u8) -> Option<()> {
    let [a, b, c, d] = curve;
    if point_segment_distance_squared(b, a, d).max(point_segment_distance_squared(c, a, d))
        <= FLATNESS * FLATNESS
    {
        out.push(d);
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

fn contours(commands: &[Command], filled: bool) -> Option<Vec<Vec<Point>>> {
    let mut contours = Vec::<Vec<Point>>::new();
    for command in commands {
        match *command {
            Command::Move(p) => contours.push(vec![p]),
            Command::Line(p) => contours.last_mut()?.push(p),
            Command::Cubic([b, c, d]) => {
                let contour = contours.last_mut()?;
                let a = *contour.last()?;
                flatten(contour, [a, b, c, d], 0)?;
            }
        }
    }
    if filled {
        for contour in &mut contours {
            if contour.first() != contour.last() {
                contour.push(*contour.first()?);
            }
        }
    }
    Some(contours)
}

fn orient(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn segments_close(a: &[Point], b: &[Point], distance: f64) -> bool {
    let [p, q] = [a[0], a[1]];
    let [r, s] = [b[0], b[1]];
    let Some(ab) = Bounds::from_points([p, q].into_iter(), 0.0) else {
        return true;
    };
    let Some(bb) = Bounds::from_points([r, s].into_iter(), 0.0) else {
        return true;
    };
    if !ab.overlaps(bb, distance) {
        return false;
    }
    if orient(p, q, r) * orient(p, q, s) < 0.0 && orient(r, s, p) * orient(r, s, q) < 0.0 {
        return true;
    }
    let threshold = distance * distance;
    point_segment_distance_squared(p, r, s)
        .min(point_segment_distance_squared(q, r, s))
        .min(point_segment_distance_squared(r, p, q))
        .min(point_segment_distance_squared(s, p, q))
        <= threshold
}

fn contains(shape: &Shape, p: Point) -> bool {
    shape.filled
        && shape
            .contours
            .iter()
            .flat_map(|c| c.windows(2))
            .filter(|edge| {
                let [a, b] = [edge[0], edge[1]];
                (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x
            })
            .count()
            % 2
            == 1
}

fn contact(a: &Shape, b: &Shape, tolerance: f64) -> bool {
    a.bounds.overlaps(b.bounds, tolerance)
        && (a.contours.iter().any(|c| contains(b, c[0]))
            || b.contours.iter().any(|c| contains(a, c[0]))
            || a.contours.iter().flat_map(|c| c.windows(2)).any(|edge| {
                b.contours
                    .iter()
                    .flat_map(|c| c.windows(2))
                    .any(|other| segments_close(edge, other, a.radius + b.radius + tolerance))
            }))
}

// A tagging export must preserve every drawing, including order. The only
// permitted differences are the two paint attributes of the tagged slur.
pub(super) fn marked_segments(
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
            if a.class != b.class || a.commands != b.commands || a.radius != b.radius {
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
            segments.push((p, i));
            observed.push((p, top));
        }
    }
    observed.sort_unstable();
    (observed == systems).then_some(segments)
}

pub(super) fn side(pages: &[Page], segments: &[(usize, usize)]) -> Option<Side> {
    let &(p, i) = segments.first()?;
    let side = pages[p].shapes[i].side()?;
    segments
        .iter()
        .all(|&(p, i)| pages[p].shapes[i].side() == Some(side))
        .then_some(side)
}

pub(super) fn contacts(pages: &[Page], segments: &[(usize, usize)], tolerance: f64) -> usize {
    segments
        .iter()
        .map(|&(p, i)| {
            let shapes = &pages[p].shapes;
            shapes
                .iter()
                .enumerate()
                .filter(|(j, s)| *j != i && !s.guide() && contact(&shapes[i], s, tolerance))
                .count()
        })
        .sum()
}

pub(super) fn stable_contacts(pages: &[Page], segments: &[(usize, usize)]) -> bool {
    // Agreement beyond the contour approximation envelope.
    let inner = contacts(pages, segments, 0.0);
    inner > 0 && inner == contacts(pages, segments, FLATNESS * 2.0 + ROUNDING)
}

pub(super) fn clear(pages: &[Page], segments: &[(usize, usize)]) -> bool {
    contacts(pages, segments, FLATNESS * 2.0 + ROUNDING) == 0
}

fn unchanged_guide(a: &Shape, b: &Shape) -> bool {
    if a.vertical_translation(b).is_some() {
        return true;
    }
    if a.class == "Bracket" && b.class == a.class && a.paint == b.paint {
        return a.commands.iter().zip(&b.commands).all(|(x, y)| {
            std::mem::discriminant(x) == std::mem::discriminant(y)
                && x.points()
                    .iter()
                    .zip(y.points())
                    .all(|(p, q)| (p.x - q.x).abs() <= ROUNDING)
        }) && a.commands.len() == b.commands.len();
    }
    if a.class != "BarLine" || b.class != a.class || a.paint != b.paint {
        return false;
    }
    matches!((a.extent(), b.extent()), (Some((p,q)), Some((r,s)))
        if p.x == q.x && p.x == r.x && r.x == s.x && (p.y - q.y) * (r.y - s.y) > 0.0)
}

pub(super) fn neighborhood_preserved(
    before: &[Page],
    after: &[Page],
    before_segments: &[(usize, usize)],
    after_segments: &[(usize, usize)],
) -> bool {
    if before.len() != after.len() || before_segments != after_segments {
        return false;
    }
    for (page, (a, b)) in before.iter().zip(after).enumerate() {
        if a.format != b.format || a.shapes.len() != b.shapes.len() {
            return false;
        }
        let mut independent_ink: Vec<(&Shape, f64)> = Vec::new();
        for (index, (x, y)) in a.shapes.iter().zip(&b.shapes).enumerate() {
            if before_segments.contains(&(page, index)) {
                continue;
            }
            if x.guide() {
                if !unchanged_guide(x, y) {
                    return false;
                }
            } else {
                let Some(dy) = x.vertical_translation(y) else {
                    return false;
                };
                independent_ink.push((y, dy));
            }
        }
        // Only relative movement can affect contacts. If two shapes move
        // relative to one another, require clear ink after the move. This also
        // rejects changing the strength of a pre-existing shared contact.
        for (index, &(y, dy)) in independent_ink.iter().enumerate() {
            if independent_ink[index + 1..]
                .iter()
                .any(|&(other, other_dy)| {
                    (dy - other_dy).abs() > ROUNDING * 2.0
                        && contact(y, other, FLATNESS * 2.0 + ROUNDING)
                })
            {
                return false;
            }
        }
    }
    true
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
        assert!(stable_contacts(&baseline, &segments));
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
}
