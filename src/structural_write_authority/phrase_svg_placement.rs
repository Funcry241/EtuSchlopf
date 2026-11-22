#[path = "phrase_svg_geometry.rs"]
mod geometry;

use crate::pipeline::PhraseSvgPreview;
use geometry::{Page, Side};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;

const MARKER: &str = "#2173C5";
const COLOR_ATTRIBUTE: &str = " color=\"#2173C5\"";

struct Trial {
    insert_at: usize,
    systems: Vec<(usize, usize)>,
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
                            .insert(number, (staff, voice, systems.len() - 1, insert))
                            .is_some()
                        {
                            return None;
                        }
                    }
                    Some("stop") => {
                        let (start_staff, start_voice, start_system, insert) =
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
    let svg = preview.render(xml, |path, source| {
        super::super::write_renderer_feasibility_shadow_candidate_authorized(path, source).ok()
    })?;
    if svg.len() != counts.len() {
        return None;
    }
    svg.iter()
        .zip(counts)
        .map(|(svg, &count)| {
            let page = Page::read(svg)?;
            (page.system_count(staves)? == count).then_some(page)
        })
        .collect()
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
    changed_pages: BTreeMap<usize, Page>,
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
    if !geometry::stable_contacts(baseline, &segments) {
        return None;
    }
    let side = geometry::side(baseline, &segments)?.opposite();
    let candidate = render(preview, Some(side), "")?;
    let tagged = render(preview, Some(side), COLOR_ATTRIBUTE)?;
    let candidate_segments =
        geometry::marked_segments(&candidate, &tagged, MARKER, staves, &trial.systems)?;
    let remaining = geometry::contacts(&candidate, &candidate_segments, 0.0);
    let clear = geometry::side(&candidate, &candidate_segments) == Some(side)
        && geometry::clear(&candidate, &candidate_segments)
        && geometry::neighborhood_preserved(baseline, &candidate, &segments, &candidate_segments);
    eprintln!(
        "PhraseSlurClearance: segments={}; baseline_contacts={}; candidate_contacts={remaining}; clearance={clear}",
        segments.len(),
        geometry::contacts(baseline, &segments, 0.0)
    );
    clear.then(|| Proposal {
        edit: (trial.insert_at, side.attribute()),
        changed_pages: candidate
            .into_iter()
            .enumerate()
            .filter(|(index, page)| !page.same_ink(&baseline[*index]))
            .collect(),
    })
}

fn independent_proposals(proposals: &[Proposal]) -> impl Iterator<Item = &Proposal> {
    // A page is a vertical layout unit. A change in its drawing can affect
    // the measured clearance of any other proposal changing the same page.
    let mut page_users = BTreeMap::new();
    for proposal in proposals {
        for page in proposal.changed_pages.keys() {
            *page_users.entry(page).or_insert(0) += 1;
        }
    }
    proposals.iter().filter(move |proposal| {
        !proposal.changed_pages.is_empty()
            && proposal
                .changed_pages
                .keys()
                .all(|page| page_users[page] == 1)
    })
}

pub(super) fn placement(
    source: &str,
    preview: &mut PhraseSvgPreview,
) -> Option<Vec<(usize, &'static str)>> {
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
    let independent = independent_proposals(&proposals).collect::<Vec<_>>();
    let mut edits = independent
        .iter()
        .map(|proposal| proposal.edit)
        .collect::<Vec<_>>();
    edits.sort_unstable();
    if edits.len() > 1 {
        // Show that the single final plan realizes each independent decision
        // exactly, with all unaffected pages equal to the baseline.
        let combined = render_pages(preview, &variant(source, &edits)?, &page_systems, staves)?;
        let verified = combined.iter().enumerate().all(|(index, observed)| {
            let expected = independent
                .iter()
                .find_map(|p| p.changed_pages.get(&index))
                .unwrap_or(&baseline[index]);
            observed.same_ink(expected)
        });
        if !verified {
            return None;
        }
    }
    (!edits.is_empty()).then_some(edits)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn proposal(at: usize, page: usize) -> Proposal {
        let shape = Page::read(r#"<svg xmlns="http://www.w3.org/2000/svg"><path class="Note" d="M1,1 L2,1 L2,2 L1,1"/></svg>"#).unwrap();
        Proposal {
            edit: (at, Side::Above.attribute()),
            changed_pages: BTreeMap::from([(page, shape)]),
        }
    }

    #[test]
    fn duplication_keeps_independent_trials_and_refuses_only_shared_pages() {
        let independent = [proposal(2, 0), proposal(4, 1)];
        let edits = independent_proposals(&independent)
            .map(|p| p.edit)
            .collect::<Vec<_>>();
        assert_eq!(
            edits,
            [(2, Side::Above.attribute()), (4, Side::Above.attribute())]
        );
        let conflict = [proposal(2, 0), proposal(4, 1), proposal(7, 1)];
        assert_eq!(
            independent_proposals(&conflict)
                .map(|p| p.edit)
                .collect::<Vec<_>>(),
            [(2, Side::Above.attribute())]
        );
    }
}
