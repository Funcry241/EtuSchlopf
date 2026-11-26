//! Diagnostic readback of independently owned engraving ink. The native
//! shadow score supplies ownership; the plain preview supplies the geometry.
//! Neither the shadow score nor the observations can be used as a PDF source.

use roxmltree::{Document, Node};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use super::{
    CONTOUR_ERROR, GRID, GridPoint, PREVIEW_RESERVE, Page, Shape, contains, edges,
    point_near_segment, possible_contact,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mark {
    Head,
    Stem { chord: usize, isolated: bool },
    Flag { chord: usize, isolated: bool },
    Beam,
    Tie,
    Rest { short: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Head,
    Stem,
    Flag,
    Beam,
    Tie,
    Rest,
}

impl Kind {
    fn from_svg(class: &str) -> Option<Self> {
        match class {
            "Note" => Some(Self::Head),
            "Stem" => Some(Self::Stem),
            "Hook" => Some(Self::Flag),
            "Beam" => Some(Self::Beam),
            "TieSegment" => Some(Self::Tie),
            "Rest" => Some(Self::Rest),
            _ => None,
        }
    }
}

impl Mark {
    fn kind(self) -> Kind {
        match self {
            Self::Head => Kind::Head,
            Self::Stem { .. } => Kind::Stem,
            Self::Flag { .. } => Kind::Flag,
            Self::Beam => Kind::Beam,
            Self::Tie => Kind::Tie,
            Self::Rest { .. } => Kind::Rest,
        }
    }
}

pub(crate) struct MarkedNativeScore {
    pub(crate) xml: String,
    marks: BTreeMap<String, Mark>,
}

fn open_child_at(node: Node<'_, '_>, xml: &str) -> Option<usize> {
    let span = node.range();
    let ending = xml.get(span.clone())?.find('>')?;
    let point = span.start.checked_add(ending)?;
    (xml.as_bytes().get(point.checked_sub(1)?) != Some(&b'/')).then_some(point + 1)
}

fn child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|item| item.has_tag_name(name))
}

fn color_element(name: &str, index: u16) -> String {
    let [green, blue] = index.to_be_bytes();
    format!("<{name} r=\"49\" g=\"{green}\" b=\"{blue}\" a=\"255\"/>")
}

struct Tagger<'a> {
    xml: &'a str,
    marks: BTreeMap<String, Mark>,
    edits: Vec<(usize, usize, String)>,
}

impl Tagger<'_> {
    fn mark(&mut self, role: Mark) -> Option<String> {
        let index = u16::try_from(self.marks.len().checked_add(1)?).ok()?;
        self.marks.insert(format!("#31{index:04x}"), role);
        Some(color_element("color", index))
    }

    fn tag(&mut self, node: Node<'_, '_>, role: Mark) -> Option<()> {
        let color = self.mark(role)?;
        if let Some(old) = child(node, "color") {
            let span = old.range();
            self.edits.push((span.start, span.end, color));
        } else if let Some(at) = open_child_at(node, self.xml) {
            self.edits.push((at, at, color));
        } else {
            // Empty native engraving objects may be serialized as <Stem/>.
            let span = node.range();
            let tail = span.end.checked_sub(2)?;
            if self.xml.get(tail..span.end)? != "/>" {
                return None;
            }
            let name = node.tag_name().name();
            self.edits
                .push((tail, span.end, format!(">{color}</{name}>")));
        }
        Some(())
    }

    fn tag_chord(&mut self, node: Node<'_, '_>, chord: usize) -> Option<()> {
        let duration = child(node, "durationType")?.text()?;
        let isolated = child(node, "BeamMode").and_then(|n| n.text()) == Some("no");
        let mut missing = String::new();
        for (name, role) in [
            ("Stem", Mark::Stem { chord, isolated }),
            ("Hook", Mark::Flag { chord, isolated }),
        ] {
            if name == "Stem" && matches!(duration, "whole" | "breve" | "long" | "longa") {
                continue;
            }
            if name == "Hook"
                && !matches!(
                    duration,
                    "eighth" | "16th" | "32nd" | "64th" | "128th" | "256th" | "512th" | "1024th"
                )
            {
                continue;
            }
            if let Some(element) = child(node, name) {
                self.tag(element, role)?;
            } else {
                let color = self.mark(role)?;
                write!(missing, "<{name}>{color}</{name}>").ok()?;
            }
        }
        if !missing.is_empty() {
            let at = open_child_at(node, self.xml)?;
            self.edits.push((at, at, missing));
        }
        for head in node.children().filter(|n| n.has_tag_name("Note")) {
            self.tag(head, Mark::Head)?;
            for tie in head
                .children()
                .filter(|n| n.has_tag_name("Spanner"))
                .flat_map(|n| n.children().filter(|t| t.has_tag_name("Tie")))
            {
                self.tag(tie, Mark::Tie)?;
            }
        }
        Some(())
    }

    fn finish(mut self) -> Option<MarkedNativeScore> {
        self.edits.sort_by_key(|edit| (edit.0, edit.1));
        let mut cursor = 0;
        let mut result = String::with_capacity(self.xml.len());
        for (start, end, text) in self.edits {
            result.push_str(self.xml.get(cursor..start)?);
            result.push_str(&text);
            cursor = end;
        }
        result.push_str(self.xml.get(cursor..)?);
        Some(MarkedNativeScore {
            xml: result,
            marks: self.marks,
        })
    }
}

/// Use native engraving objects in a disposable copy only. A change of layout,
/// drawing order, topology or styling makes the corresponding page unavailable.
pub(crate) fn tag_native(xml: &str) -> Option<MarkedNativeScore> {
    let doc = Document::parse(xml).ok()?;
    let root = doc.root_element();
    if !root.has_tag_name("museScore") {
        return None;
    }
    let score = child(root, "Score")?;
    let mut tagger = Tagger {
        xml,
        marks: BTreeMap::new(),
        edits: Vec::new(),
    };
    let mut chord = 0;
    for staff in score.children().filter(|n| n.has_tag_name("Staff")) {
        for measure in staff.children().filter(|n| n.has_tag_name("Measure")) {
            for voice in measure.children().filter(|n| n.has_tag_name("voice")) {
                for entry in voice.children().filter(Node::is_element) {
                    match entry.tag_name().name() {
                        "Chord" => {
                            chord += 1;
                            tagger.tag_chord(entry, chord)?;
                        }
                        "Rest" => {
                            let short = matches!(
                                child(entry, "durationType").and_then(|n| n.text()),
                                Some("eighth" | "16th" | "32nd" | "64th")
                            );
                            tagger.tag(entry, Mark::Rest { short })?;
                        }
                        "Beam" => tagger.tag(entry, Mark::Beam)?,
                        _ => {}
                    }
                }
            }
        }
    }
    tagger.finish()
}

fn paint_value<'a>(shape: &'a Shape, name: &str) -> &'a str {
    shape
        .paint
        .iter()
        .find_map(|(key, value)| (key == name).then_some(value.as_str()))
        .unwrap_or(if name == "fill" && shape.filled {
            "#000000"
        } else {
            "none"
        })
}

fn binding(a: &Shape, b: &Shape, marks: &BTreeMap<String, Mark>) -> Option<Option<Mark>> {
    if a.class != b.class
        || a.commands != b.commands
        || a.stroke_scale != b.stroke_scale
        || a.filled != b.filled
        || a.even_odd != b.even_odd
    {
        return None;
    }
    let structural_paint =
        |(key, _): &&(String, String)| !matches!(key.as_str(), "fill" | "stroke");
    if !a
        .paint
        .iter()
        .filter(structural_paint)
        .eq(b.paint.iter().filter(structural_paint))
    {
        return None;
    }
    let mut bound = None;
    for ink in ["fill", "stroke"] {
        let old = paint_value(a, ink);
        let new = paint_value(b, ink);
        if old.eq_ignore_ascii_case(new) {
            continue;
        }
        if old == "none" || new == "none" {
            return None;
        }
        let mark = *marks.get(&new.to_ascii_lowercase())?;
        if Kind::from_svg(&a.class) != Some(mark.kind())
            || bound.is_some_and(|previous| previous != mark)
        {
            return None;
        }
        bound = Some(mark);
    }
    Some(bound)
}

fn inner(shape: &Shape, point: GridPoint) -> bool {
    let guard = ((CONTOUR_ERROR + PREVIEW_RESERVE) * GRID).ceil() as i128;
    let in_fill = contains(shape, point)
        && !edges(shape)
            .any(|e| point_near_segment(point, e[0], e[1], guard.pow(2)).unwrap_or(true));
    let inside_pen = (shape.round_inner_radius.unwrap_or_default() * GRID).floor() as i128 - guard;
    in_fill
        || (inside_pen > 0
            && edges(shape).any(|e| {
                point_near_segment(point, e[0], e[1], (inside_pen - 1).pow(2)).unwrap_or(false)
            }))
}

#[derive(Debug, PartialEq, Eq)]
enum InkRelation {
    Separate,
    Overlap,
    Undecidable,
}

fn ink_relation(a: &Shape, b: &Shape) -> InkRelation {
    if !possible_contact(a, b, PREVIEW_RESERVE) {
        return InkRelation::Separate;
    }
    let left = a.bounds.left.max(b.bounds.left);
    let right = a.bounds.right.min(b.bounds.right);
    let top = a.bounds.top.max(b.bounds.top);
    let bottom = a.bounds.bottom.min(b.bounds.bottom);
    // Samples are only witnesses. Missing the overlap on this fixed grid
    // means unknown, never an inference that the two shapes are clear.
    if left < right && top < bottom {
        for row in 1..16 {
            for col in 1..16 {
                let sample = super::Point {
                    x: left + (right - left) * f64::from(col) / 16.0,
                    y: top + (bottom - top) * f64::from(row) / 16.0,
                };
                if sample.grid().is_some_and(|p| inner(a, p) && inner(b, p)) {
                    return InkRelation::Overlap;
                }
            }
        }
    }
    InkRelation::Undecidable
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Collision {
    FlagBeam,
    FlaggedStemBeam,
    IndependentStems,
    FlagTie,
    RestHead,
}

impl Collision {
    fn code(self) -> &'static str {
        match self {
            Self::FlagBeam => "flag_beam",
            Self::FlaggedStemBeam => "flagged_stem_beam",
            Self::IndependentStems => "independent_stems",
            Self::FlagTie => "flag_tie",
            Self::RestHead => "rest_head",
        }
    }

    fn of(self, a: Option<Mark>, b: Option<Mark>, whole_flags: &BTreeSet<usize>) -> Option<bool> {
        match (self, a?, b?) {
            (Self::FlagBeam, Mark::Flag { chord, isolated }, Mark::Beam)
            | (Self::FlaggedStemBeam, Mark::Stem { chord, isolated }, Mark::Beam) => {
                (isolated && whole_flags.contains(&chord)).then_some(true)
            }
            (Self::IndependentStems, Mark::Stem { chord: a, .. }, Mark::Stem { chord: b, .. }) => {
                Some(a != b && (whole_flags.contains(&a) || whole_flags.contains(&b)))
            }
            (Self::FlagTie, Mark::Flag { chord, .. }, Mark::Tie) => {
                whole_flags.contains(&chord).then_some(true)
            }
            (Self::RestHead, Mark::Rest { short }, Mark::Head) => Some(short),
            _ => None,
        }
    }
}

fn possible_pair(a: Kind, b: Kind) -> Option<(Collision, bool)> {
    match (a, b) {
        (Kind::Flag, Kind::Beam) => Some((Collision::FlagBeam, false)),
        (Kind::Stem, Kind::Beam) => Some((Collision::FlaggedStemBeam, false)),
        (Kind::Stem, Kind::Stem) => Some((Collision::IndependentStems, false)),
        (Kind::Flag, Kind::Tie) => Some((Collision::FlagTie, false)),
        (Kind::Rest, Kind::Head) => Some((Collision::RestHead, false)),
        (Kind::Beam, Kind::Flag) => Some((Collision::FlagBeam, true)),
        (Kind::Beam, Kind::Stem) => Some((Collision::FlaggedStemBeam, true)),
        (Kind::Tie, Kind::Flag) => Some((Collision::FlagTie, true)),
        (Kind::Head, Kind::Rest) => Some((Collision::RestHead, true)),
        _ => None,
    }
}

#[derive(Default)]
struct PageFindings {
    overlaps: BTreeMap<Collision, usize>,
    undecidable: BTreeMap<Collision, usize>,
    locations: Vec<(Collision, f64, f64)>,
    unbound_marks: usize,
}

fn rendering_agrees(a: &Shape, b: &Shape) -> bool {
    let paint_without_width = |(key, _): &&(String, String)| key != "stroke-width";
    if a.class != b.class
        || !a
            .paint
            .iter()
            .filter(paint_without_width)
            .eq(b.paint.iter().filter(paint_without_width))
        || !a
            .stroke_scale
            .iter()
            .zip(b.stroke_scale)
            .all(|(source, shadow)| (source - shadow).abs() <= PREVIEW_RESERVE)
        || a.commands.len() != b.commands.len()
        || !a.bounds.overlaps(b.bounds, PREVIEW_RESERVE)
    {
        return false;
    }
    a.commands.iter().zip(&b.commands).all(|(a, b)| {
        std::mem::discriminant(a) == std::mem::discriminant(b)
            && a.points()
                .iter()
                .zip(b.points())
                .all(|(a, b)| (a.x - b.x).hypot(a.y - b.y) <= PREVIEW_RESERVE)
    })
}

fn same_viewport(source: &Page, native: &Page) -> bool {
    if source.format == native.format {
        return true;
    }
    let ignore_box = |(name, _): &&(String, String)| name != "viewBox";
    if !source
        .format
        .iter()
        .filter(ignore_box)
        .eq(native.format.iter().filter(ignore_box))
    {
        return false;
    }
    let coordinates = |page: &Page| {
        let (_, value) = page.format.iter().find(|(name, _)| name == "viewBox")?;
        let values = value
            .split_ascii_whitespace()
            .map(super::number)
            .collect::<Option<Vec<_>>>()?;
        <[f64; 4]>::try_from(values).ok()
    };
    let Some([x, y, width, height]) = coordinates(source) else {
        return false;
    };
    let Some([nx, ny, nw, nh]) = coordinates(native) else {
        return false;
    };
    // The converter may round the serialized width and height when it
    // imports its own native file. Physical size and the origin are fixed.
    x == nx
        && y == ny
        && width > 0.0
        && height > 0.0
        && (width - nw).hypot(height - nh) < PREVIEW_RESERVE
}

fn printed_marks(
    plain: &Page,
    native: &Page,
    tagged: &Page,
    marks: &BTreeMap<String, Mark>,
) -> Option<Vec<Option<Mark>>> {
    if !same_viewport(plain, native)
        || native.format != tagged.format
        || native.shapes.len() != tagged.shapes.len()
    {
        return None;
    }
    let native_bindings = native
        .shapes
        .iter()
        .zip(&tagged.shapes)
        .map(|(a, b)| binding(a, b, marks))
        .collect::<Option<Vec<_>>>()?;
    let mut by_kind = BTreeMap::<String, Vec<(usize, &Shape)>>::new();
    for (i, n) in native.shapes.iter().enumerate() {
        if Kind::from_svg(&n.class).is_some() {
            by_kind.entry(n.class.clone()).or_default().push((i, n));
        }
    }
    let mut references = vec![None; plain.shapes.len()];
    let mut native_uses = vec![0usize; native.shapes.len()];
    for (index, p) in plain.shapes.iter().enumerate() {
        let Some(same_kind) = by_kind.get(&p.class) else {
            continue;
        };
        let mut unique = None;
        let mut count = 0;
        for &(native_index, _) in same_kind.iter().filter(|(_, n)| rendering_agrees(p, n)) {
            native_uses[native_index] += 1;
            unique = Some(native_index);
            count += 1;
        }
        if count == 1 {
            references[index] = unique;
        }
    }
    Some(
        references
            .into_iter()
            .map(|reference| {
                let id = reference?;
                (native_uses[id] == 1)
                    .then_some(native_bindings[id])
                    .flatten()
            })
            .collect(),
    )
}

fn inspect(
    plain: &Page,
    native: &Page,
    tagged: &Page,
    marks: &BTreeMap<String, Mark>,
) -> Option<PageFindings> {
    let bindings = printed_marks(plain, native, tagged, marks)?;
    let mut flag_count = BTreeMap::<usize, usize>::new();
    let mut stem_count = BTreeMap::<usize, usize>::new();
    for mark in bindings.iter().flatten() {
        match *mark {
            Mark::Stem { chord, .. } => *stem_count.entry(chord).or_default() += 1,
            Mark::Flag { chord, .. } => *flag_count.entry(chord).or_default() += 1,
            _ => {}
        }
    }
    let whole_flags = flag_count
        .into_iter()
        .filter_map(|(chord, count)| {
            (count == 1 && stem_count.get(&chord) == Some(&1)).then_some(chord)
        })
        .collect::<BTreeSet<_>>();

    let objects = plain
        .shapes
        .iter()
        .zip(bindings)
        .filter_map(|(s, role)| Kind::from_svg(&s.class).map(|kind| (s, kind, role)))
        .collect::<Vec<_>>();
    let hooks = objects
        .iter()
        .filter_map(|(shape, kind, binding)| (*kind == Kind::Flag).then_some((*shape, *binding)))
        .collect::<Vec<_>>();
    let potentially_flagged = objects.iter().map(|(shape, kind, binding)| {
        if *kind != Kind::Stem {
            return false;
        }
        match binding {
            Some(Mark::Stem { chord, .. }) => whole_flags.contains(chord)
                || hooks.iter().any(|(hook, owner)| {
                    (owner.is_none() || matches!(owner, Some(Mark::Flag { chord: other, .. }) if other == chord))
                        && possible_contact(shape, hook, PREVIEW_RESERVE)
                }),
            None => hooks.iter().any(|(hook, _)| possible_contact(shape, hook, PREVIEW_RESERVE)),
            _ => false,
        }
    }).collect::<Vec<_>>();
    let mut findings = PageFindings {
        unbound_marks: objects.iter().filter(|(_, _, role)| role.is_none()).count(),
        ..PageFindings::default()
    };
    for (i, (a, kind_a, ma)) in objects.iter().enumerate() {
        for (j, (b, kind_b, mb)) in objects.iter().enumerate().skip(i + 1) {
            let Some((class, reverse)) = possible_pair(*kind_a, *kind_b) else {
                continue;
            };
            // Lack of a matching native stem is uncertainty only where an
            // actual drawn hook could belong to it. Geometric proximity
            // does not itself certify the owner.
            if matches!(
                class,
                Collision::FlaggedStemBeam | Collision::IndependentStems
            ) && !(potentially_flagged[i] || potentially_flagged[j])
            {
                continue;
            }
            if !a
                .bounds
                .overlaps(b.bounds, 2.0 * (PREVIEW_RESERVE + CONTOUR_ERROR))
            {
                continue;
            }
            let (left, right) = if reverse { (*mb, *ma) } else { (*ma, *mb) };
            let role = class.of(left, right, &whole_flags);
            if role == Some(false) {
                continue;
            }
            let evidence = ink_relation(a, b);
            match (role, evidence) {
                (_, InkRelation::Separate) => {}
                (Some(true), InkRelation::Overlap) => {
                    *findings.overlaps.entry(class).or_default() += 1;
                    findings.locations.push((
                        class,
                        a.bounds.left.max(b.bounds.left),
                        a.bounds.top.max(b.bounds.top),
                    ));
                }
                _ => *findings.undecidable.entry(class).or_default() += 1,
            }
        }
    }
    Some(findings)
}

pub(crate) fn report(
    plain: &[String],
    native_plain: &[String],
    tagged: &[String],
    native: &MarkedNativeScore,
) -> Option<String> {
    if plain.len() != tagged.len() || plain.len() != native_plain.len() || plain.is_empty() {
        return None;
    }
    let mut output = String::new();
    let mut overlaps = BTreeMap::<Collision, usize>::new();
    let mut undecidable = BTreeMap::<Collision, usize>::new();
    let mut unread_pages = 0;
    let mut unbound_marks = 0;
    let mut unread = String::new();
    for (number, ((a, b), c)) in plain.iter().zip(native_plain).zip(tagged).enumerate() {
        let inspected = Page::read(a)
            .zip(Page::read(b))
            .zip(Page::read(c))
            .and_then(|((a, b), c)| inspect(&a, &b, &c, &native.marks));
        let Some(found) = inspected else {
            unread_pages += 1;
            write!(unread, "{} ", number + 1).ok()?;
            continue;
        };
        unbound_marks += found.unbound_marks;
        for (c, count) in found.overlaps {
            *overlaps.entry(c).or_default() += count;
        }
        for (c, count) in found.undecidable {
            *undecidable.entry(c).or_default() += count;
        }
        for (class, x, y) in found.locations {
            writeln!(
                output,
                "EngravingCollisionObservation: page={}; kind={}; svg_x={x:.2}; svg_y={y:.2}",
                number + 1,
                class.code()
            )
            .ok()?;
        }
    }
    let show = |counts: BTreeMap<Collision, usize>| {
        counts
            .iter()
            .map(|(class, count)| format!("{}={count}", class.code()))
            .collect::<Vec<_>>()
            .join(",")
    };
    let summary = format!(
        "EngravingCollisionReadback: pages={}; unavailable_pages={unread_pages}; unavailable_page_numbers={}; unbound_marks={unbound_marks}; confirmed=[{}]; undecidable=[{}]; basis=committed_source_svg_with_native_ownership; authority=none\n",
        plain.len(),
        unread.trim(),
        show(overlaps),
        show(undecidable)
    );
    Some(summary + &output)
}

#[cfg(test)]
#[path = "engraving_stem_pair_audit.rs"]
pub(crate) mod stem_pair_audit;

#[cfg(test)]
#[path = "engraving_source_anchor_audit.rs"]
pub(crate) mod source_anchor_audit;

#[cfg(test)]
#[path = "engraving_transition_dry_run.rs"]
mod transition_dry_run;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct Drawing {
        class: &'static str,
        mark: Mark,
        bounds: [i32; 4],
    }

    fn drawing(class: &'static str, mark: Mark, bounds: [i32; 4]) -> Drawing {
        Drawing {
            class,
            mark,
            bounds,
        }
    }

    fn svg(drawings: &[Drawing], tagged: bool) -> String {
        let mut output = "<svg xmlns=\"http://www.w3.org/2000/svg\">".to_owned();
        for (
            i,
            Drawing {
                class,
                bounds: [left, top, right, bottom],
                ..
            },
        ) in drawings.iter().enumerate()
        {
            let fill = if tagged {
                format!(" fill=\"#31{:04x}\"", i + 1)
            } else {
                String::new()
            };
            write!(
                output,
                "<path class=\"{class}\"{fill} d=\"M{left},{top} L{right},{top} L{right},{bottom} L{left},{bottom} L{left},{top}\"/>"
            ).unwrap();
        }
        output.push_str("</svg>");
        output
    }

    fn finding(original: &[Drawing], native: &[Drawing]) -> PageFindings {
        let plain = Page::read(&svg(original, false)).unwrap();
        let intermediate = Page::read(&svg(native, false)).unwrap();
        let annotated = Page::read(&svg(native, true)).unwrap();
        let marks = native
            .iter()
            .enumerate()
            .map(|(i, shape)| (format!("#31{:04x}", i + 1), shape.mark))
            .collect();
        inspect(&plain, &intermediate, &annotated, &marks).unwrap()
    }

    fn all_roles(chord: usize) -> Vec<Drawing> {
        vec![
            drawing(
                "Stem",
                Mark::Stem {
                    chord,
                    isolated: true,
                },
                [4, 0, 6, 9],
            ),
            drawing(
                "Hook",
                Mark::Flag {
                    chord,
                    isolated: true,
                },
                [4, 6, 10, 9],
            ),
            drawing("Beam", Mark::Beam, [2, 5, 12, 8]),
            drawing(
                "Stem",
                Mark::Stem {
                    chord: chord + 1,
                    isolated: false,
                },
                [5, 1, 7, 8],
            ),
            drawing("TieSegment", Mark::Tie, [9, 7, 14, 10]),
            drawing("Rest", Mark::Rest { short: true }, [16, 2, 20, 7]),
            drawing("Note", Mark::Head, [19, 3, 22, 6]),
        ]
    }

    #[test]
    fn recognizes_all_five_bound_overlap_classes() {
        let shapes = all_roles(4);
        let result = finding(&shapes, &shapes);
        for class in [
            Collision::FlagBeam,
            Collision::FlaggedStemBeam,
            Collision::IndependentStems,
            Collision::FlagTie,
            Collision::RestHead,
        ] {
            assert_eq!(result.overlaps.get(&class), Some(&1), "{class:?}");
        }
        assert!(result.undecidable.is_empty());
    }

    #[test]
    fn renaming_relocation_and_independent_duplication_preserve_findings() {
        let original = all_roles(4);
        let mut relocated = all_roles(98);
        for shape in &mut relocated {
            for x in [0, 2] {
                shape.bounds[x] += 40;
            }
        }
        let once = finding(&original, &original).overlaps;
        assert_eq!(finding(&relocated, &relocated).overlaps, once);
        let both = [original, relocated].concat();
        assert_eq!(
            finding(&both, &both).overlaps,
            once.into_iter()
                .map(|(key, count)| (key, count * 2))
                .collect()
        );
    }

    #[test]
    fn every_class_distinguishes_contact_clearance_and_ambiguous_binding() {
        let original = all_roles(10);
        for (class, first, second) in [
            (Collision::FlagBeam, 1, 2),
            (Collision::FlaggedStemBeam, 0, 2),
            (Collision::IndependentStems, 0, 3),
            (Collision::FlagTie, 1, 4),
            (Collision::RestHead, 5, 6),
        ] {
            let mut moved = original.clone();
            let left = moved[first].bounds[2];
            let width = moved[second].bounds[2] - moved[second].bounds[0];
            moved[second].bounds[0] = left;
            moved[second].bounds[2] = left + width;
            let touch = finding(&moved, &moved);
            assert!(!touch.overlaps.contains_key(&class), "{class:?}");
            assert!(touch.undecidable.contains_key(&class), "{class:?}");

            moved[second].bounds[0] += 40;
            moved[second].bounds[2] += 40;
            let clear = finding(&moved, &moved);
            assert!(!clear.overlaps.contains_key(&class), "{class:?}");
            assert!(!clear.undecidable.contains_key(&class), "{class:?}");

            let mut ambiguous = original.clone();
            ambiguous.push(original[second]);
            let observed = finding(&original, &ambiguous);
            assert!(observed.unbound_marks > 0, "{class:?}");
            assert!(!observed.overlaps.contains_key(&class), "{class:?}");
            assert!(observed.undecidable.contains_key(&class), "{class:?}");
        }
    }

    #[test]
    fn own_stem_and_legitimate_shared_graphic_are_not_a_foreign_overlap() {
        let mark = Mark::Stem {
            chord: 5,
            isolated: true,
        };
        let a = vec![
            drawing("Stem", mark, [4, 0, 6, 9]),
            drawing(
                "Hook",
                Mark::Flag {
                    chord: 5,
                    isolated: true,
                },
                [4, 6, 10, 9],
            ),
        ];
        let clean = finding(&a, &a);
        assert!(clean.overlaps.is_empty() && clean.undecidable.is_empty());
        let mut duplicate = a.clone();
        duplicate.push(drawing("Stem", mark, [4, 0, 6, 9]));
        assert!(finding(&duplicate, &duplicate).overlaps.is_empty());
    }

    #[test]
    fn tags_native_shapes_without_touching_musical_fields() {
        let unmarked = r#"<museScore><Score><Staff><Measure><voice><Beam/><Chord>
          <durationType>16th</durationType><BeamMode>no</BeamMode><Stem/>
          <Note><pitch>65</pitch><Spanner><Tie/></Spanner></Note></Chord>
          <Rest><durationType>eighth</durationType></Rest>
        </voice></Measure></Staff></Score></museScore>"#;
        let native = tag_native(unmarked).unwrap();
        assert_eq!(native.marks.len(), 6);
        let parsed = Document::parse(&native.xml).unwrap();
        assert_eq!(
            parsed
                .descendants()
                .find(|n| n.has_tag_name("pitch"))
                .and_then(|n| n.text()),
            Some("65")
        );
        assert_eq!(
            parsed
                .descendants()
                .filter(|n| n.has_tag_name("color"))
                .count(),
            6
        );
    }
}
