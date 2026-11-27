//! Source-bound tie fragments in two controlled MuseScore layouts.

use std::collections::BTreeSet;

use super::super::super::{
    Side,
    stem_transition::{StemObservation, observe_stem, preserves_pair, proves_direction_transition},
};
use super::super::binding;
use super::super::pair_inventory_audit::{
    ExpectedMark, ObservedMark, affected_pairs, exhaustive_pairs,
};
use super::{
    ANCHORS, Document, Kind, Mark, Page, Role, Shape, SvgInk, child, expected_roster,
    inventory_role, native_anchors, native_node, paint_value, rendering_agrees, same_viewport,
    score_in_layout,
};

const NOTE_SECTIONS: [usize; 6] = [0, 0, 1, 1, 0, 0];
const OUTGOING_TIE: u32 = 6;
const INCOMING_TIE: u32 = 7;
const START_STEM: u32 = 1;
const END_STEM: u32 = 3;

#[derive(Clone, Copy)]
pub(crate) enum SplitLayout {
    System,
    Page,
}

impl SplitLayout {
    pub(crate) fn score(self, tie_direction: &str, first_measure: u16, paint: bool) -> String {
        let marker = match self {
            Self::System => r#"<print new-system="yes"/>"#,
            Self::Page => r#"<print new-page="yes"/>"#,
        };
        score_in_layout("down", tie_direction, first_measure, paint, marker)
    }

    fn page_count(self) -> usize {
        match self {
            Self::System => 1,
            Self::Page => 2,
        }
    }

    fn verify_native(self, score: &str) -> Option<()> {
        native_anchors(score)?;
        let doc = Document::parse(score).ok()?;
        let root = doc.root_element();
        let start = native_node(root, ANCHORS[0].0, "Note")?;
        let end = native_node(root, ANCHORS[2].0, "Note")?;
        let start_measure = start
            .ancestors()
            .find(|node| node.has_tag_name("Measure"))?;
        let end_measure = end.ancestors().find(|node| node.has_tag_name("Measure"))?;
        let staff = start_measure.parent()?;
        if staff != end_measure.parent()? {
            return None;
        }
        let measures: Vec<_> = staff
            .children()
            .filter(|n| n.has_tag_name("Measure"))
            .collect();
        if measures != [start_measure, end_measure] {
            return None;
        }
        let breaks: Vec<_> = root
            .descendants()
            .filter(|n| n.has_tag_name("LayoutBreak"))
            .collect();
        let [boundary] = breaks.as_slice() else {
            return None;
        };
        let expected = match self {
            Self::System => "line",
            Self::Page => "page",
        };
        (boundary.parent() == Some(start_measure)
            && child(*boundary, "subtype")?.text()? == expected)
            .then_some(())
    }
}

pub(crate) struct SvgFrame {
    pub(crate) native: String,
    pub(crate) plain: Vec<String>,
    pub(crate) painted: Vec<String>,
    pub(crate) reread: Vec<String>,
}

impl SvgFrame {
    fn views(&self) -> Option<[Vec<Page>; 3]> {
        let read = |pages: &[String]| {
            pages
                .iter()
                .map(|svg| Page::read(svg))
                .collect::<Option<Vec<_>>>()
        };
        Some([
            read(&self.plain)?,
            read(&self.painted)?,
            read(&self.reread)?,
        ])
    }
}

fn matches_ink(shape: &Shape, color: &str) -> bool {
    ["fill", "stroke"]
        .into_iter()
        .any(|kind| paint_value(shape, kind).eq_ignore_ascii_case(color))
}

fn role(shape: &Shape) -> Option<Role> {
    Kind::from_svg(&shape.class).and_then(inventory_role)
}

fn split_section(page: usize, split: Option<f64>, shape: &Shape) -> Option<usize> {
    let Some(split) = split else {
        return Some(page);
    };
    let reserve = super::super::super::PREVIEW_RESERVE;
    if shape.bounds.bottom + reserve < split {
        Some(0)
    } else if shape.bounds.top - reserve > split {
        Some(1)
    } else {
        None
    }
}

fn qualified_inventory<'a>(
    layout: SplitLayout,
    views: &'a [Vec<Page>; 3],
) -> Option<Vec<ObservedMark<SvgInk<'a>>>> {
    let [plain, painted, reread] = views;
    if views.iter().any(|pages| pages.len() != layout.page_count()) {
        return None;
    }
    let marks = ANCHORS
        .map(|(color, _, mark)| (color.to_owned(), mark))
        .into();
    let mut result = Vec::new();
    for (index, ((plain, painted), reread)) in plain.iter().zip(painted).zip(reread).enumerate() {
        if plain.format != painted.format
            || !same_viewport(painted, reread)
            || plain.shapes.len() != painted.shapes.len()
        {
            return None;
        }
        let split = match (layout, plain.systems(2)?.as_slice()) {
            (SplitLayout::System, [split]) => Some(*split),
            (SplitLayout::Page, []) => None,
            _ => return None,
        };
        let mut used_reread = BTreeSet::new();
        for (shape, tagged) in plain.shapes.iter().zip(&painted.shapes) {
            let bound = binding(shape, tagged, &marks)?;
            let Some(role) = role(shape) else { continue };
            let mark = bound?;
            let mut ink = ANCHORS
                .iter()
                .enumerate()
                .filter(|(_, (color, _, _))| matches_ink(tagged, color));
            let (anchor, (color, _, expected_mark)) = ink.next()?;
            if ink.next().is_some() || mark != *expected_mark {
                return None;
            }
            let section = split_section(index, split, shape)?;
            let id = if mark == Mark::Tie {
                [OUTGOING_TIE, INCOMING_TIE].get(section).copied()?
            } else {
                if NOTE_SECTIONS.get(anchor) != Some(&section) {
                    return None;
                }
                u32::try_from(anchor).ok()?
            };
            let mut readback =
                reread.shapes.iter().enumerate().filter(|(_, other)| {
                    matches_ink(other, color) && rendering_agrees(tagged, other)
                });
            let (readback_index, _) = readback.next()?;
            if readback.next().is_some() || !used_reread.insert(readback_index) {
                return None;
            }
            result.push(ObservedMark {
                id: Some(id),
                page: index + 1,
                role,
                geometry: SvgInk(shape),
            });
        }
        if reread
            .shapes
            .iter()
            .filter(|shape| role(shape).is_some())
            .count()
            != used_reread.len()
        {
            return None;
        }
    }
    let roster = split_roster();
    affected_pairs(&roster, &result, &roster, &result).ok()?;
    Some(result)
}

fn split_roster() -> Vec<ExpectedMark> {
    let mut roster = expected_roster();
    roster.push(ExpectedMark {
        id: INCOMING_TIE,
        role: Role::Tie,
    });
    roster
}

fn mark_for<'a, 'b>(
    marks: &'a [ObservedMark<SvgInk<'b>>],
    id: u32,
) -> &'a ObservedMark<SvgInk<'b>> {
    marks
        .iter()
        .find(|mark| mark.id == Some(id))
        .expect("validated source inventory")
}

fn stem_on_page<'a>(
    pages: &'a [Page],
    marks: &[ObservedMark<SvgInk<'a>>],
    stem_id: u32,
) -> StemObservation<'a> {
    let stem = mark_for(marks, stem_id);
    // The fixture roster places each explicitly authored head before its stem.
    let head = mark_for(marks, stem_id - 1);
    assert_eq!(stem.page, head.page);
    observe_stem(&pages[stem.page - 1], stem.geometry.0, head.geometry.0)
        .expect("visible stem joined to its own source-bound note head")
}

fn pair_on_page<'a>(
    pages: &'a [Page],
    marks: &[ObservedMark<SvgInk<'a>>],
    (stem_id, neighbour_id): (u32, u32),
) -> Option<(StemObservation<'a>, &'a Shape)> {
    let stem = mark_for(marks, stem_id);
    let neighbour = mark_for(marks, neighbour_id);
    (stem.page == neighbour.page)
        .then(|| (stem_on_page(pages, marks, stem_id), neighbour.geometry.0))
}

fn unchanged_background(before: &[Page], after: &[Page]) -> bool {
    before.iter().zip(after).all(|(old, new)| {
        let unowned = |shape: &&Shape| role(shape).is_none();
        let mut remaining = new.shapes.iter().filter(unowned);
        old.format == new.format
            && old
                .shapes
                .iter()
                .filter(unowned)
                .all(|shape| remaining.next().is_some_and(|other| shape.same_ink(other)))
            && remaining.next().is_none()
    })
}

pub(crate) fn assert_split_transition(
    system: &SvgFrame,
    page_start: &SvgFrame,
    page_end: &SvgFrame,
) {
    for (layout, frame) in [
        (SplitLayout::System, system),
        (SplitLayout::Page, page_start),
        (SplitLayout::Page, page_end),
    ] {
        layout
            .verify_native(&frame.native)
            .expect("native tie and break provenance");
    }
    let system = system.views().unwrap();
    let mut page_start = page_start.views().unwrap();
    let mut page_end = page_end.views().unwrap();
    let same_page =
        qualified_inventory(SplitLayout::System, &system).expect("two systems on one page");
    let before =
        qualified_inventory(SplitLayout::Page, &page_start).expect("tie split across two pages");
    let after =
        qualified_inventory(SplitLayout::Page, &page_end).expect("tie split after stem change");
    let roster = split_roster();
    let compare = |old: &[ObservedMark<SvgInk<'_>>], new: &[ObservedMark<SvgInk<'_>>]| {
        let pairs = affected_pairs(&roster, old, &roster, new).unwrap();
        assert_eq!(pairs, exhaustive_pairs(old, new));
        pairs
    };
    assert!(compare(&before, &before).is_empty());
    let pairs = compare(&before, &after);
    assert!(pairs.contains(&(START_STEM, OUTGOING_TIE)));
    assert!(pairs.contains(&(END_STEM, INCOMING_TIE)));
    assert!(!pairs.contains(&(START_STEM, INCOMING_TIE)));
    assert!(!pairs.contains(&(END_STEM, OUTGOING_TIE)));
    let repaginated = compare(&same_page, &after);
    assert!(repaginated.contains(&(START_STEM, INCOMING_TIE)));
    assert!(repaginated.contains(&(END_STEM, OUTGOING_TIE)));

    let before_pages = &page_start[0];
    let after_pages = &page_end[0];
    assert!(unchanged_background(before_pages, after_pages));
    let stems = [START_STEM, END_STEM].map(|id| {
        (
            stem_on_page(before_pages, &before, id),
            stem_on_page(after_pages, &after, id),
        )
    });
    assert!(stems.iter().all(|(old, _)| old.side == Side::Above));
    assert!(proves_direction_transition(&stems, Side::Below));
    for pair in pairs {
        assert!(
            preserves_pair(
                pair_on_page(before_pages, &before, pair),
                pair_on_page(after_pages, &after, pair),
            ),
            "clearance holds for affected pair {pair:?}"
        );
    }

    for views in [&mut page_start, &mut page_end] {
        for view in views.iter_mut() {
            view[1]
                .shapes
                .retain(|shape| role(shape) != Some(Role::Tie));
        }
        assert!(qualified_inventory(SplitLayout::Page, views).is_none());
    }
}
