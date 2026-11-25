use super::{Bounds, Command, PREVIEW_RESERVE, Page, Shape};
use std::collections::{BTreeMap, BTreeSet};

type Locator = (usize, usize);

pub(crate) struct Scene {
    pages: Vec<Page>,
    tags: BTreeMap<String, Option<Locator>>,
}

pub(crate) struct Reading {
    pub height: f64,
    pub goal: f64,
    pub space: f64,
    location: Locator,
    heads: [Locator; 2],
}

impl Scene {
    pub fn read(svg: &[String]) -> Option<Self> {
        let pages = svg
            .iter()
            .map(|s| Page::read(s))
            .collect::<Option<Vec<_>>>()?;
        let mut tags = BTreeMap::new();
        for (page, content) in pages.iter().enumerate() {
            for (index, shape) in content.shapes.iter().enumerate() {
                if let Some(color) = marker(shape) {
                    tags.entry(color.to_owned())
                        .and_modify(|location| *location = None)
                        .or_insert(Some((page, index)));
                }
            }
        }
        Some(Self { pages, tags })
    }

    pub fn proof_of_marks(&self, plain: &Self) -> bool {
        plain.tags.is_empty()
            && self.pages.len() == plain.pages.len()
            && self.pages.iter().zip(&plain.pages).all(|(a, b)| {
                a.format == b.format
                    && a.shapes.len() == b.shapes.len()
                    && a.shapes
                        .iter()
                        .zip(&b.shapes)
                        .all(|(tagged, clean)| same_shape(clean, tagged, 0.0, true, 0.0))
            })
    }

    pub fn reading(&self, tags: &[String; 3]) -> Option<Reading> {
        let [left, rest, right] = tags
            .each_ref()
            .map(|tag| self.tags.get(tag).copied().flatten());
        let (left, location, right) = (left?, rest?, right?);
        let page = self.pages.get(left.0)?;
        if left.0 != location.0 || left.0 != right.0 {
            return None;
        }
        let (a, r, b) = (
            &page.shapes[left.1],
            &page.shapes[location.1],
            &page.shapes[right.1],
        );
        if a.class != "Note" || b.class != "Note" || r.class != "Rest" {
            return None;
        }
        let (x_left, x_rest, x_right) =
            (middle_x(a.bounds), middle_x(r.bounds), middle_x(b.bounds));
        if !(x_left < x_rest && x_rest < x_right) {
            return None;
        }
        let (ya, yb) = (middle_y(a.bounds), middle_y(b.bounds));
        if (ya - yb).abs() > PREVIEW_RESERVE {
            return None;
        }
        let goal = f64::midpoint(ya, yb);
        let bounds = page.systems(2)?;
        let system = bounds.partition_point(|boundary| *boundary < goal);
        let mut lines = page
            .shapes
            .iter()
            .filter(|shape| shape.class == "StaffLines")
            .map(|shape| shape.extent())
            .collect::<Option<Vec<_>>>()?;
        lines.sort_by(|a, b| a.0.y.total_cmp(&b.0.y));
        let bottom_staff = lines.get(system * 10 + 5..system * 10 + 10)?;
        let space = (bottom_staff[4].0.y - bottom_staff[0].0.y) / 4.0;
        if space <= 0.0
            || bottom_staff
                .iter()
                .any(|(start, end)| x_left < start.x || x_right > end.x)
            || bottom_staff
                .windows(2)
                .any(|pair| (pair[1].0.y - pair[0].0.y - space).abs() > PREVIEW_RESERVE)
        {
            return None;
        }
        Some(Reading {
            height: middle_y(r.bounds),
            goal,
            space,
            location,
            heads: [left, right],
        })
    }

    pub fn clearance(&self, reading: &Reading) -> f64 {
        let (page, index) = reading.location;
        let shapes = &self.pages[page].shapes;
        let rest = shapes[index].bounds;
        let gap = shapes
            .iter()
            .enumerate()
            .filter(|(i, s)| *i != index && !matches!(s.class.as_str(), "" | "StaffLines"))
            .map(|(_, s)| gap(rest, s.bounds))
            .fold(f64::INFINITY, f64::min);
        // Both enclosing boxes may shift by the renderer's rounding reserve.
        (gap - 2.0 * PREVIEW_RESERVE).max(0.0) / reading.space
    }

    pub fn verifies_move(&self, old: &Reading, new: &Reading) -> bool {
        old.location == new.location
            && old.heads == new.heads
            && improves(old, new)
            && self.clearance(new) >= 0.50
            && (old.space - new.space).abs() <= PREVIEW_RESERVE
    }

    pub fn verifies_neighborhood(&self, baseline: &Self, moves: &[&Reading]) -> bool {
        let shown = moves
            .iter()
            .map(|old| old.location)
            .collect::<BTreeSet<_>>();
        shown.len() == moves.len()
            && self.pages.len() == baseline.pages.len()
            && baseline
                .pages
                .iter()
                .zip(&self.pages)
                .enumerate()
                .all(|(page, (before, after))| {
                    before.format == after.format
                        && before.shapes.len() == after.shapes.len()
                        && before.shapes.iter().zip(&after.shapes).enumerate().all(
                            |(index, (a, b))| {
                                let dy = if shown.contains(&(page, index)) {
                                    middle_y(b.bounds) - middle_y(a.bounds)
                                } else {
                                    0.0
                                };
                                same_shape(a, b, dy, false, PREVIEW_RESERVE)
                            },
                        )
                })
    }
}

pub(crate) fn improves(before: &Reading, after: &Reading) -> bool {
    (before.height - before.goal).abs() - (after.height - before.goal).abs() > PREVIEW_RESERVE
}

fn marker(shape: &Shape) -> Option<&str> {
    shape.paint.iter().find_map(|(key, value)| {
        (key == "fill" && value.starts_with("#a2")).then_some(value.as_str())
    })
}

fn middle_x(b: Bounds) -> f64 {
    f64::midpoint(b.left, b.right)
}

fn middle_y(b: Bounds) -> f64 {
    f64::midpoint(b.top, b.bottom)
}

fn gap(a: Bounds, b: Bounds) -> f64 {
    let dx = (a.left - b.right).max(b.left - a.right).max(0.0);
    let dy = (a.top - b.bottom).max(b.top - a.bottom).max(0.0);
    dx.hypot(dy)
}

fn same_shape(a: &Shape, b: &Shape, shift_y: f64, marked: bool, tolerance: f64) -> bool {
    a.class == b.class
        && a.stroke_scale == b.stroke_scale
        && normalized_paint(a, false).eq(normalized_paint(b, marked))
        && a.commands.len() == b.commands.len()
        && a.commands.iter().zip(&b.commands).all(|(a, b)| {
            matches!(
                (a, b),
                (Command::Move(_), Command::Move(_))
                    | (Command::Line(_), Command::Line(_))
                    | (Command::Cubic(_), Command::Cubic(_))
            ) && a.points().iter().zip(b.points()).all(|(p, q)| {
                (p.x - q.x).abs() <= tolerance && (p.y + shift_y - q.y).abs() <= tolerance
            })
        })
}

fn normalized_paint(shape: &Shape, marked: bool) -> impl Iterator<Item = (&str, &str)> {
    shape.paint.iter().filter_map(move |(key, value)| {
        let value =
            if marked && matches!(key.as_str(), "fill" | "stroke") && value.starts_with("#a2") {
                "#000000"
            } else {
                value.as_str()
            };
        // Unstyled paths use the same black fill as an explicit fill attribute.
        (key != "fill" || value != "#000000").then_some((key.as_str(), value))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(kind: &str, x: i32, y: i32, color: &str) -> String {
        format!(
            r#"<path class="{kind}" fill="{color}" d="M{x},{y} L{right},{y} L{right},{bottom} L{x},{bottom} L{x},{y}" />"#,
            right = x + 4,
            bottom = y + 4,
        )
    }

    fn scene(a: i32, b: i32, independent: Option<i32>) -> Scene {
        let mut svg = r#"<svg xmlns="http://www.w3.org/2000/svg">"#.to_owned();
        for y in (10..=50).step_by(10).chain((110..=150).step_by(10)) {
            svg.push_str(&format!(
                r##"<polyline class="StaffLines" fill="none" stroke="#000000" stroke-width="1" points="0,{y} 200,{y}"/> "##
            ));
        }
        for (lane, x, head, y) in [(0, 45, 130, a), (3, 45, 132, b)]
            .into_iter()
            .chain(independent.map(|y| (6, 145, 130, y)))
        {
            for (offset, kind, x, y) in [
                (0, "Note", x - 20, head),
                (1, "Rest", x, y),
                (2, "Note", x + 20, head),
            ] {
                svg.push_str(&shape(kind, x, y, &format!("#a2{:04x}", lane + offset)));
            }
        }
        svg.push_str("</svg>");
        Scene::read(&[svg]).expect("synthetic SVG")
    }

    fn reading(scene: &Scene, start: u16) -> Reading {
        scene
            .reading(&[start, start + 1, start + 2].map(|tag| format!("#a2{tag:04x}")))
            .expect("one rest and two noteheads")
    }

    #[test]
    fn individually_clear_moves_can_collide_in_the_combined_render() {
        let before = scene(105, 158, None);
        let first_only = scene(130, 158, None);
        let second_only = scene(105, 132, None);
        let combined = scene(130, 132, None);
        let a = reading(&before, 0);
        let b = reading(&before, 3);
        assert!(first_only.verifies_move(&a, &reading(&first_only, 0)));
        assert!(second_only.verifies_move(&b, &reading(&second_only, 3)));
        assert!(!combined.verifies_move(&a, &reading(&combined, 0)));
        assert!(!combined.verifies_move(&b, &reading(&combined, 3)));
    }

    #[test]
    fn an_independent_move_keeps_its_clearance_when_another_is_added() {
        let before = scene(105, 158, Some(105));
        let first_only = scene(130, 158, Some(105));
        let both = scene(130, 158, Some(130));
        let a = reading(&before, 0);
        let c = reading(&before, 6);
        let clearance = first_only.clearance(&reading(&first_only, 0));
        assert!((both.clearance(&reading(&both, 0)) - clearance).abs() < f64::EPSILON);
        assert!(both.verifies_move(&a, &reading(&both, 0)));
        assert!(both.verifies_move(&c, &reading(&both, 6)));
        assert!(both.verifies_neighborhood(&before, &[&a, &c]));
        assert!(!both.verifies_neighborhood(&before, &[&a]));
    }
}
