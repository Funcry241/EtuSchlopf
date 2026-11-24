#[path = "phrase_svg_geometry.rs"]
mod geometry;

use crate::pipeline::PhraseSvgPreview;
use geometry::{Page, Side};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;

const MARKER: &str = "#2173C5";
const COLOR_ATTRIBUTE: &str = " color=\"#2173C5\"";
const ANCHOR_COLORS: [&str; 2] = ["#b93270", "#528329"];
const ANCHOR_ATTRIBUTES: [&str; 2] = [" color=\"#b93270\"", " color=\"#528329\""];

fn note_marker(source: &str, note: Node<'_, '_>) -> Option<usize> {
    let at = note.range().start;
    (note.attribute("color").is_none() && source.get(at..)?.starts_with("<note"))
        .then_some(at + "<note".len())
}

struct Trial {
    insert_at: usize,
    systems: Vec<(usize, usize)>,
    anchors: Option<[usize; 2]>,
}

struct Course {
    staves: usize,
    page_systems: Vec<usize>,
    trials: Vec<Trial>,
}

fn text<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.children().find(|n| n.has_tag_name(name))?.text()
}

fn course(source: &str) -> Option<Course> {
    let document = Document::parse(source).ok()?;
    let root = document.root_element();
    if !root.has_tag_name("score-partwise") {
        return None;
    }
    let mut parts = root.children().filter(|n| n.has_tag_name("part"));
    let part = parts.next()?;
    if parts.next().is_some() {
        return None;
    }
    let mut sizes = part.descendants().filter(|n| n.has_tag_name("staves"));
    let staves = sizes
        .next()
        .and_then(|n| n.text()?.parse::<usize>().ok())
        .unwrap_or(1);
    if staves == 0 || sizes.any(|node| node.text().and_then(|s| s.parse().ok()) != Some(staves)) {
        return None;
    }
    let mut open = BTreeMap::new();
    let mut trials = Vec::new();
    let mut page_systems = vec![1];
    let (mut page, mut system) = (0, 0);
    let mut systems = vec![(page, system)];
    for (index, measure) in part
        .children()
        .filter(|n| n.has_tag_name("measure"))
        .enumerate()
    {
        if index != 0
            && let Some(print) = measure.children().find(|n| n.has_tag_name("print"))
        {
            if print.attribute("new-page") == Some("yes") {
                page += 1;
                system = 0;
                page_systems.push(1);
                systems.push((page, system));
            } else if print.attribute("new-system") == Some("yes") {
                system += 1;
                page_systems[page] += 1;
                systems.push((page, system));
            }
        }
        for note in measure.children().filter(|n| n.has_tag_name("note")) {
            let staff = text(note, "staff").unwrap_or("1");
            let voice = text(note, "voice").unwrap_or("1");
            for slur in note
                .children()
                .filter(|n| n.has_tag_name("notations"))
                .flat_map(|n| n.children().filter(|item| item.has_tag_name("slur")))
            {
                let number = slur.attribute("number").unwrap_or("1");
                match slur.attribute("type") {
                    Some("start") => {
                        let eligible = slur.attribute("placement").is_none()
                            && slur.attribute("color").is_none()
                            && slur.attribute("line-type").is_none()
                            && source.get(slur.range())?.ends_with("/>");
                        let insert = eligible.then_some(slur.range().end - 2);
                        if open
                            .insert(
                                number,
                                (
                                    staff,
                                    voice,
                                    systems.len() - 1,
                                    insert,
                                    note_marker(source, note),
                                ),
                            )
                            .is_some()
                        {
                            return None;
                        }
                    }
                    Some("stop") => {
                        let (start_staff, start_voice, start_system, insert, start_note) =
                            open.remove(number)?;
                        if let Some(insert_at) = insert
                            && start_staff == staff
                            && start_voice == voice
                            && slur.attribute("placement").is_none()
                            && slur.attribute("color").is_none()
                        {
                            trials.push(Trial {
                                insert_at,
                                systems: systems[start_system..].to_vec(),
                                anchors: start_note
                                    .zip(note_marker(source, note))
                                    .map(|(a, b)| [a, b]),
                            });
                        }
                    }
                    _ => return None,
                }
            }
        }
    }
    open.is_empty().then_some(Course {
        staves,
        page_systems,
        trials,
    })
}

fn render_pages(
    preview: &mut PhraseSvgPreview,
    xml: &str,
    counts: &[usize],
    staves: usize,
) -> Option<Vec<Page>> {
    paint_pages(
        &mut |source| {
            preview.render(source, |path, xml| {
                super::super::write_renderer_feasibility_shadow_candidate_authorized(path, xml).ok()
            })
        },
        xml,
        counts,
        staves,
    )
}

fn variant<'a>(source: &'a str, edits: &[(usize, &'a str)]) -> Option<String> {
    let mut spans = Vec::new();
    let mut cursor = 0;
    for &(at, text) in edits {
        spans.extend([source.get(cursor..at)?, text]);
        cursor = at;
    }
    spans.push(source.get(cursor..)?);
    Some(spans.concat())
}

struct Proposal {
    edit: (usize, &'static str),
    candidate: Vec<Page>,
    segments: Vec<(usize, usize)>,
}

fn measure_trial(
    source: &str,
    preview: &mut PhraseSvgPreview,
    trial: Trial,
    baseline: &[Page],
    page_systems: &[usize],
    staves: usize,
) -> Option<Proposal> {
    let render = |preview: &mut PhraseSvgPreview, side: Option<Side>, marker| {
        let placement = side.map_or("", Side::attribute);
        let xml = variant(
            source,
            &[(trial.insert_at, placement), (trial.insert_at, marker)],
        )?;
        render_pages(preview, &xml, page_systems, staves)
    };
    let tagged = render(preview, None, COLOR_ATTRIBUTE)?;
    let segments = geometry::marked_segments(baseline, &tagged, MARKER, staves, &trial.systems)?;
    let baseline_contacts = geometry::contacts(baseline, &segments);
    if baseline_contacts == 0 {
        return None;
    }
    let side = geometry::side(baseline, &segments)?.opposite();
    let candidate = render(preview, Some(side), "")?;
    let tagged = render(preview, Some(side), COLOR_ATTRIBUTE)?;
    let candidate_segments =
        geometry::marked_segments(&candidate, &tagged, MARKER, staves, &trial.systems)?;
    let clearance = geometry::side(&candidate, &candidate_segments) == Some(side)
        && geometry::clear(&candidate, &candidate_segments);
    let neighborhood =
        geometry::neighborhood_preserved(baseline, &candidate, &segments, &candidate_segments);
    eprintln!(
        "PhraseSlurClearance: segments={}; baseline_contacts={baseline_contacts}; clearance={clearance}; neighborhood={neighborhood}",
        segments.len()
    );
    (clearance && neighborhood).then_some(Proposal {
        edit: (trial.insert_at, side.attribute()),
        candidate,
        segments,
    })
}

pub(super) fn placement(
    source: &str,
    preview: &mut PhraseSvgPreview,
) -> Option<Vec<(usize, &'static str)>> {
    if preview.wants_slur_paint() {
        if let Some((plan, count)) = paint_plan(source, |xml| {
            preview.render(xml, |path, bytes| {
                super::super::write_renderer_feasibility_shadow_candidate_authorized(path, bytes)
                    .ok()
            })
        }) {
            let _ = preview.set_slur_paint_plan(source, &plan, count);
        }
        return None;
    }
    if !source.contains("<slur ")
        || source
            .to_ascii_lowercase()
            .contains(&MARKER.to_ascii_lowercase())
    {
        return None;
    }
    let Course {
        staves,
        page_systems,
        trials,
    } = course(source)?;
    if trials.is_empty() {
        return None;
    }
    let baseline = render_pages(preview, source, &page_systems, staves)?;
    let proposals = trials
        .into_iter()
        .filter_map(|trial| measure_trial(source, preview, trial, &baseline, &page_systems, staves))
        .collect::<Vec<_>>();
    let mut edits = proposals
        .iter()
        .map(|proposal| proposal.edit)
        .collect::<Vec<_>>();
    edits.sort_unstable();
    if edits.len() > 1 {
        // Shared pages are allowed. The final combined preview must keep all
        // other ink and realize every selected shape at its verified position.
        let combined = render_pages(preview, &variant(source, &edits)?, &page_systems, staves)?;
        let segments = proposals
            .iter()
            .flat_map(|p| p.segments.iter().copied())
            .collect::<Vec<_>>();
        if !geometry::neighborhood_preserved(&baseline, &combined, &segments, &segments)
            || !geometry::clear(&combined, &segments)
            || !proposals
                .iter()
                .all(|p| geometry::planned_segments_match(&p.candidate, &combined, &p.segments))
        {
            return None;
        }
    }
    (!edits.is_empty()).then_some(edits)
}

fn paint_pages(
    export: &mut impl FnMut(&str) -> Option<Vec<String>>,
    xml: &str,
    counts: &[usize],
    staves: usize,
) -> Option<Vec<Page>> {
    let pages = export(xml)?;
    if pages.len() != counts.len() {
        return None;
    }
    pages
        .iter()
        .zip(counts)
        .map(|(page, &count)| {
            let page = Page::read(page)?;
            (page.system_count(staves)? == count).then_some(page)
        })
        .collect()
}

pub(super) fn paint_plan(
    source: &str,
    mut export: impl FnMut(&str) -> Option<Vec<String>>,
) -> Option<(String, usize)> {
    let lower = source.to_ascii_lowercase();
    if !source.contains("<slur ")
        || lower.contains(&MARKER.to_ascii_lowercase())
        || ANCHOR_COLORS.iter().any(|color| lower.contains(color))
    {
        return None;
    }
    let Course {
        staves,
        page_systems,
        trials,
    } = course(source)?;
    if trials.is_empty() {
        return None;
    }
    let mut render = |xml: &str| paint_pages(&mut export, xml, &page_systems, staves);
    let baseline = render(source)?;
    let mut result = Vec::new();
    for trial in trials {
        let attempt = (|| {
            let notes = trial.anchors?;
            let trial_xml = |side: &str, tagged: bool, anchors: bool| {
                let mut edits = vec![(trial.insert_at, side)];
                if tagged {
                    edits.push((trial.insert_at, COLOR_ATTRIBUTE));
                }
                if anchors {
                    edits.extend(notes.into_iter().zip(ANCHOR_ATTRIBUTES));
                }
                edits.sort_by_key(|&(at, _)| at);
                variant(source, &edits)
            };
            let tagged = render(&trial_xml("", true, false)?)?;
            let old =
                geometry::marked_segments(&baseline, &tagged, MARKER, staves, &trial.systems)?;
            if geometry::contacts(&baseline, &old) == 0 {
                return None;
            }
            let side = geometry::side(&baseline, &old)?.opposite().attribute();
            let alternate = render(&trial_xml(side, false, false)?)?;
            let alternate_tagged = render(&trial_xml(side, true, false)?)?;
            let alternate_segments = geometry::marked_segments(
                &alternate,
                &alternate_tagged,
                MARKER,
                staves,
                &trial.systems,
            )?;
            let baseline_anchors = render(&trial_xml("", true, true)?)?;
            let alternate_anchors = render(&trial_xml(side, true, true)?)?;
            geometry::paint::choose(
                geometry::paint::PaintPreview {
                    baseline: &baseline,
                    alternate: &alternate,
                    baseline_anchors: &baseline_anchors,
                    alternate_anchors: &alternate_anchors,
                },
                &old,
                &alternate_segments,
                &trial.systems,
                staves,
                ANCHOR_COLORS,
            )
        })();
        result.extend(attempt.into_iter().flatten());
    }
    if result.is_empty() || !geometry::paint::compatible(&baseline, &result) {
        return None;
    }
    use std::fmt::Write;
    let mut plan = String::from("etuschlopf-slur-paint-v1\n");
    for edit in &result {
        write!(plan, "{}", edit.page).ok()?;
        for point in edit.before.into_iter().chain(edit.after) {
            write!(plan, " {} {}", point[0], point[1]).ok()?;
        }
        plan.push('\n');
    }
    Some((plan, result.len()))
}
