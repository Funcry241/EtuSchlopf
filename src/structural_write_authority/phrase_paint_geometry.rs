use super::*;

const GRID_STEPS: i32 = 64;

#[derive(Clone, Copy)]
pub(in super::super) struct PaintSegment {
    pub page: usize,
    pub before: [[f64; 2]; 7],
    pub after: [[f64; 2]; 7],
}

struct StaffRegion {
    lines: Vec<(Point, Point)>,
    top: f64,
    bottom: f64,
}

fn staff_regions(page: &Page, staves: usize) -> Option<Vec<StaffRegion>> {
    let rows = staves.checked_mul(5)?;
    if rows == 0 {
        return None;
    }
    let mut lines = page
        .shapes
        .iter()
        .filter(|s| s.class == "StaffLines")
        .map(Shape::extent)
        .collect::<Option<Vec<_>>>()?;
    lines.sort_by(|a, b| a.0.y.total_cmp(&b.0.y));
    if lines.is_empty() || lines.len() % rows != 0 {
        return None;
    }
    let view_box = &page.format.iter().find(|(key, _)| key == "viewBox")?.1;
    let values = view_box
        .split_ascii_whitespace()
        .map(number)
        .collect::<Option<Vec<_>>>()?;
    let &[_, y, _, height] = values.as_slice() else {
        return None;
    };
    let mut top = y;
    let mut result = Vec::new();
    for chunk in lines.chunks_exact(rows) {
        let end = (result.len() + 1) * rows;
        let bottom = lines
            .get(end)
            .map_or(y + height, |next| (chunk[rows - 1].0.y + next.0.y) * 0.5);
        result.push(StaffRegion {
            lines: chunk.to_vec(),
            top,
            bottom,
        });
        top = bottom;
    }
    Some(result)
}

fn ribbon(shape: &Shape) -> Option<[Point; 7]> {
    let [
        Command::Move(a),
        Command::Cubic([b, c, d]),
        Command::Cubic([e, f, g]),
    ] = shape.commands.as_slice()
    else {
        return None;
    };
    (*a == *g && a.x < d.x).then_some([*a, *b, *c, *d, *e, *f, *g])
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= PREVIEW_RESERVE
}

fn translation(a: &[Point], b: &[Point]) -> Option<Point> {
    let (a0, b0) = (a.first()?, b.first()?);
    let delta = Point {
        x: a0.x - b0.x,
        y: a0.y - b0.y,
    };
    (a.len() == b.len()
        && a.iter()
            .zip(b)
            .all(|(x, y)| close(x.x, y.x + delta.x) && close(x.y, y.y + delta.y)))
    .then_some(delta)
}

// Compare the outlines of the notes identified by the marker export. The
// outline check refuses reassociation to another note after moving the slur.
fn note_translation(a: &Page, b: &Page, color: &str) -> Option<Point> {
    let note = |p: &Page| -> Option<Vec<Point>> {
        let mut matches = p.shapes.iter().filter(|s| {
            s.class == "Note" && s.paint.iter().any(|(_, v)| v.eq_ignore_ascii_case(color))
        });
        let hit = matches.next()?;
        matches.next().is_none().then(|| {
            hit.commands
                .iter()
                .flat_map(Command::points)
                .copied()
                .collect()
        })
    };
    translation(&note(a)?, &note(b)?)
}

fn territory(a: &StaffRegion, b: &StaffRegion) -> Option<Point> {
    let flat = |r: &StaffRegion| -> Vec<_> {
        r.lines
            .iter()
            .flat_map(|(start, stop)| [*start, *stop])
            .collect()
    };
    translation(&flat(a), &flat(b))
}

fn preserve_continuation(
    points: &mut [Point; 7],
    width: f64,
    free: [bool; 2],
    staff: &StaffRegion,
    space: f64,
) -> Option<()> {
    let [left, _, _, right, ..] = *points;
    let span = right.x - left.x;
    if span <= 0.0 {
        return None;
    }
    let free_count = free.iter().filter(|&&b| b).count();
    if span >= width || free_count == 0 {
        return Some(());
    }
    let extension = (width - span) / free_count as f64;
    let left_shift = if free[0] { -extension } else { 0.0 };
    let right_shift = if free[1] { extension } else { 0.0 };
    if extension > space
        || left.x + left_shift < staff.lines.first()?.0.x - PREVIEW_RESERVE
        || right.x + right_shift > staff.lines.first()?.1.x + PREVIEW_RESERVE
    {
        return None;
    }

    // Move the two matching control points by the same vector. This restores
    // the span and scales the centreline rise without thickening the ribbon.
    for (i, j) in [(1, 5), (2, 4)] {
        let mid = points[i].mid(points[j]);
        let t = (mid.x - left.x) / span;
        let dx = (1.0 - t) * left_shift + t * right_shift;
        let dy = (mid.y - ((1.0 - t) * left.y + t * right.y)) * (width / span - 1.0);
        points[i].x += dx;
        points[i].y += dy;
        points[j].x += dx;
        points[j].y += dy;
    }
    points[0].x += left_shift;
    points[6] = points[0];
    points[3].x += right_shift;
    Some(())
}

struct TrialSegment<'a> {
    page: usize,
    shape: &'a Shape,
    before: [Point; 7],
    template: [Point; 7],
    obstacles: Vec<&'a Shape>,
    space: f64,
    margin: f64,
    corridor: (f64, f64),
}

fn shape_after(seed: &Shape, points: [Point; 7]) -> Option<Shape> {
    let [a, b, c, d, e, f, g] = points;
    let commands = vec![
        Command::Move(a),
        Command::Cubic([b, c, d]),
        Command::Cubic([e, f, g]),
    ];
    Some(Shape {
        class: seed.class.clone(),
        paint: seed.paint.clone(),
        contours: contours(&commands, seed.filled)?,
        bounds: Bounds::from_points(points.into_iter(), seed.radius)?,
        commands,
        stroke_scale: seed.stroke_scale,
        half_stroke_width: seed.half_stroke_width,
        radius: seed.radius,
        round_inner_radius: seed.round_inner_radius,
        filled: seed.filled,
        even_odd: seed.even_odd,
    })
}

fn test_shape(seg: &TrialSegment<'_>, a: i32, b: i32) -> Option<Shape> {
    let mut points = seg.template;
    let amount = seg.space / f64::from(GRID_STEPS);
    for i in [1, 5] {
        points[i].y += amount * f64::from(a);
    }
    for i in [2, 4] {
        points[i].y += amount * f64::from(b);
    }
    let candidate = shape_after(seg.shape, points)?;
    if candidate.side() != seg.shape.side().map(Side::opposite)
        || candidate.bounds.top <= seg.corridor.0
        || candidate.bounds.bottom >= seg.corridor.1
        || seg
            .obstacles
            .iter()
            .any(|o| possible_contact(&candidate, o, seg.margin))
    {
        return None;
    }
    Some(candidate)
}

pub(in super::super) struct PaintPreview<'a> {
    pub baseline: &'a [Page],
    pub alternate: &'a [Page],
    pub baseline_anchors: &'a [Page],
    pub alternate_anchors: &'a [Page],
}

pub(in super::super) fn choose(
    preview: PaintPreview<'_>,
    old: &[(usize, usize)],
    alt: &[(usize, usize)],
    systems: &[(usize, usize)],
    staves: usize,
    colors: [&str; 2],
) -> Option<Vec<PaintSegment>> {
    let mut trials = Vec::new();
    if old.len() != alt.len() || old.len() != systems.len() || old.is_empty() {
        return None;
    }
    for (n, (&(page, system), (&(p, i), &(q, j)))) in
        systems.iter().zip(old.iter().zip(alt)).enumerate()
    {
        if p != page || q != page {
            return None;
        }
        let shape = &preview.baseline[page].shapes[i];
        let other = &preview.alternate[page].shapes[j];
        if shape.side()?.opposite() != other.side()? || shape.stroke_scale != other.stroke_scale {
            return None;
        }
        let systems_before = staff_regions(&preview.baseline[page], staves)?;
        let systems_after = staff_regions(&preview.alternate[page], staves)?;
        let (left, right) = (systems_before.get(system)?, systems_after.get(system)?);
        let attached = [n == 0, n + 1 == old.len()];
        let a = attached[0]
            .then(|| {
                note_translation(
                    &preview.baseline_anchors[page],
                    &preview.alternate_anchors[page],
                    colors[0],
                )
            })
            .flatten();
        let b = attached[1]
            .then(|| {
                note_translation(
                    &preview.baseline_anchors[page],
                    &preview.alternate_anchors[page],
                    colors[1],
                )
            })
            .flatten();
        if (attached[0] && a.is_none()) || (attached[1] && b.is_none()) {
            return None;
        }
        let shift = if let (Some(a), Some(b)) = (a, b) {
            if !close(a.x, b.x) || !close(a.y, b.y) {
                return None;
            }
            a.mid(b)
        } else {
            let d = territory(left, right)?;
            if a.or(b)
                .is_some_and(|anchor| !close(anchor.x, d.x) || !close(anchor.y, d.y))
            {
                return None;
            }
            d
        };
        let before = ribbon(shape)?;
        let mut template = ribbon(other)?.map(|p| Point {
            x: p.x + shift.x,
            y: p.y + shift.y,
        });
        let space = left.lines[1].0.y - left.lines[0].0.y;
        let margin = preview.baseline[page]
            .shapes
            .iter()
            .filter(|s| s.class == "StaffLines")
            .map(|s| s.stroke_width())
            .max_by(f64::total_cmp)?;
        if space <= 0.0 || margin <= 0.0 {
            return None;
        }
        preserve_continuation(
            &mut template,
            before[3].x - before[0].x,
            attached.map(|a| !a),
            left,
            space,
        )?;
        let region = Bounds::from_points(template.into_iter(), space + shape.radius + margin)?;
        let obstacles = preview.baseline[page]
            .shapes
            .iter()
            .enumerate()
            .filter_map(|(j, s)| {
                (i != j
                    && !s.guide()
                    && region.overlaps(s.bounds, 2.0 * CONTOUR_ERROR + PREVIEW_RESERVE))
                .then_some(s)
            })
            .collect();
        trials.push(TrialSegment {
            page,
            shape,
            before,
            template,
            obstacles,
            space,
            margin,
            corridor: (left.top, left.bottom),
        });
    }

    // One choice of control-point motion across all systems. Where more than
    // one choice has the same minimum cost, the course is left alone.
    let mut candidates = (-GRID_STEPS..=GRID_STEPS)
        .flat_map(|a| {
            (-GRID_STEPS..=GRID_STEPS).map(move |b| {
                (
                    (a.abs().max(b.abs()), a.abs() + b.abs(), (a - b).abs()),
                    a,
                    b,
                )
            })
        })
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    for group in candidates.chunk_by(|a, b| a.0 == b.0) {
        let mut best = None;
        for &(_, a, b) in group {
            if let Some(shapes) = trials
                .iter()
                .map(|s| test_shape(s, a, b))
                .collect::<Option<Vec<_>>>()
            {
                if best.is_some() {
                    return None;
                }
                best = Some(shapes);
            }
        }
        if let Some(shapes) = best {
            return trials
                .iter()
                .zip(shapes)
                .map(|(s, shape)| {
                    Some(PaintSegment {
                        page: s.page,
                        before: s.before.map(|p| [p.x, p.y]),
                        after: ribbon(&shape)?.map(|p| [p.x, p.y]),
                    })
                })
                .collect();
        }
    }
    None
}

pub(in super::super) fn compatible(pages: &[Page], edits: &[PaintSegment]) -> bool {
    let shapes = edits
        .iter()
        .map(|edit| {
            let page = &pages[edit.page];
            let mut source = page.shapes.iter().filter(|s| {
                s.class == "SlurSegment"
                    && ribbon(s).is_some_and(|r| r.map(|p| [p.x, p.y]) == edit.before)
            });
            let shape = source.next()?;
            if source.next().is_some() {
                return None;
            }
            let margin = page
                .shapes
                .iter()
                .filter(|s| s.class == "StaffLines")
                .map(|s| s.stroke_width())
                .max_by(f64::total_cmp)?;
            let final_shape = shape_after(shape, edit.after.map(|[x, y]| Point { x, y }))?;
            Some((margin, final_shape))
        })
        .collect::<Option<Vec<_>>>();
    let Some(shapes) = shapes else { return false };
    edits.iter().enumerate().all(|(i, a)| {
        edits[i + 1..].iter().enumerate().all(|(j, b)| {
            let (q, r) = (&shapes[i], &shapes[i + j + 1]);
            a.page != b.page
                || (a.before != b.before && !possible_contact(&q.1, &r.1, q.0.max(r.0)))
        })
    })
}

#[cfg(test)]
mod continuation_tests {
    use super::*;

    #[test]
    fn retaining_the_original_span_moves_only_the_free_end() {
        let point = |x, y| Point { x, y };
        let mut curve = [
            point(12.0, 10.0),
            point(14.0, 14.0),
            point(16.0, 14.0),
            point(18.0, 10.0),
            point(16.0, 15.0),
            point(14.0, 15.0),
            point(12.0, 10.0),
        ];
        let staff = StaffRegion {
            lines: vec![(point(0.0, 0.0), point(30.0, 0.0))],
            top: 0.0,
            bottom: 40.0,
        };
        assert!(preserve_continuation(&mut curve, 8.0, [true, false], &staff, 4.0).is_some());
        assert!(curve[0] == point(10.0, 10.0));
        assert!(curve[0] == curve[6]);
        assert!(curve[3] == point(18.0, 10.0));
        assert!((curve[5].y - curve[1].y - 1.0).abs() < 1e-12);
    }
}
