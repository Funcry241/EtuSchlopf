//! Compare an externally rendered, source-marked set of disposable previews.

use anyhow::{Context, Result, ensure};
use roxmltree::{Document, Node};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::audit_only::{
    renderer_system_break_readback::pdf_source_anchor_audit::tied_stem_pdf_transition::{
        Mark, PdfCurve, PdfLine, PdfPaint, TrackedStem, bind_guides, delta_stays_on_marked_paint,
        foreign_clearance, read_paired_paint,
    },
    tied_stem_ink_roster::{NoteOccurrence, StemDirection, read_ink_roster},
    tied_stem_pair_decision::Decision,
};
use crate::pipeline::svg_notation_geometry::{Side, collision_observation::pdf_head_binding::proves_single_head_attachment};

struct End {
    note: NoteOccurrence,
    head: Mark,
    stem: Mark,
}

struct Proposal {
    id: usize,
    ends: [End; 2],
    tie: Mark,
}

pub(crate) struct PdfReviewFrame<'a> {
    pub(crate) source: &'a str,
    pub(crate) marked_source: &'a str,
    pub(crate) pdf: &'a [u8],
    pub(crate) marked_pdf: &'a [u8],
    pub(crate) svg: &'a [String],
}

fn mark(value: &str) -> Result<Mark> {
    let hex = value.strip_prefix('#').context("missing RGB mark")?;
    ensure!(hex.len() == 6 && hex.bytes().all(|byte| byte.is_ascii_hexdigit()), "invalid RGB mark");
    let [_, r, g, b] = u32::from_str_radix(hex, 16)?.to_be_bytes();
    ensure!([r, g, b] != [0; 3], "black source mark");
    Ok([r, g, b])
}

fn read_proposals(rows: &str) -> Result<Vec<Proposal>> {
    let mut seen = BTreeSet::new();
    let mut colors = BTreeSet::new();
    rows.lines().skip(1).map(|row| {
        let columns = row.split('\t').collect::<Vec<_>>();
        let [id, fm, fi, tm, ti, fh, fs, tie, th, ts] = columns.as_slice() else {
            anyhow::bail!("invalid selected source mark row");
        };
        let id = id.parse::<usize>()?;
        ensure!(id > 0 && seen.insert(id), "ambiguous proposal group");
        let endpoint = |measure: &str, index: &str, head: &str, stem: &str| -> Result<End> {
            Ok(End {
                note: NoteOccurrence { part: 0, measure: measure.parse::<usize>()?.checked_sub(1)
                    .context("zero measure ordinal")?, note: index.parse()? },
                head: mark(head)?, stem: mark(stem)?,
            })
        };
        let first = endpoint(fm, fi, fh, fs)?;
        let second = endpoint(tm, ti, th, ts)?;
        let tie = mark(tie)?;
        ensure!([first.head, first.stem, second.head, second.stem, tie].into_iter().all(|ink| colors.insert(ink)),
            "duplicate selected PDF mark");
        Ok(Proposal { id, ends: [first, second], tie })
    }).collect()
}

fn child<'a, 'i>(node: Node<'a, 'i>, name: &str) -> Option<Node<'a, 'i>> {
    node.children().find(|child| child.has_tag_name(name))
}

fn note_at<'a, 'i>(document: &'a Document<'i>, id: NoteOccurrence) -> Option<Node<'a, 'i>> {
    document.root_element().children().filter(|node| node.has_tag_name("part")).nth(id.part)?
        .children().filter(|node| node.has_tag_name("measure")).nth(id.measure)?
        .children().filter(|node| node.has_tag_name("note")).nth(id.note)
}

fn paired_xml_marks(plain: &str, painted: &str, proposals: &[Proposal]) -> Result<()> {
    let plain = Document::parse(plain)?;
    let painted = Document::parse(painted)?;
    let signature = |doc: &Document<'_>| doc.descendants().filter(|node| node.is_element()).map(|node| {
        let tag = node.tag_name().name();
        let mut attrs: Vec<_> = node.attributes()
            .filter(|attr| !(attr.name() == "color" && matches!(tag, "note" | "stem" | "tied")))
            .map(|attr| (attr.name().to_owned(), attr.value().to_owned())).collect();
        attrs.sort();
        (node.ancestors().count(), tag.to_owned(), attrs, node.text().unwrap_or("").trim().to_owned())
    }).collect::<Vec<_>>();
    ensure!(signature(&plain) == signature(&painted), "marked MusicXML changed source semantics");
    for proposal in proposals {
        let [from, to] = &proposal.ends;
        let start_note = note_at(&painted, from.note).context("selected tie start missing")?;
        let stop_note = note_at(&painted, to.note).context("selected tie end missing")?;
        ensure!(from.note != to.note && ["pitch", "voice", "staff"].into_iter().all(|tag| {
            let signature = |note| child(note, tag).map(|element| element.descendants().filter_map(|node| {
                node.is_text().then(|| node.text().unwrap_or("").trim()).filter(|text| !text.is_empty())
            }).collect::<Vec<_>>());
            signature(start_note) == signature(stop_note)
        }), "selected tie endpoints do not share a written pitch and voice");
        for (end, tie_type) in [(from, "start"), (to, "stop")] {
            let note = note_at(&painted, end.note).context("selected marked note missing")?;
            let color = |node: Node<'_, '_>| mark(node.attribute("color").context("selected mark absent")?);
            let stem = child(note, "stem").context("selected marked stem missing")?;
            ensure!(color(note)? == end.head && color(stem)? == end.stem, "source marks do not bind to the selected stem and head");
            let tied = child(note, "notations").context("selected marked tie missing")?.children()
                .find(|node| node.has_tag_name("tied") && node.attribute("type") == Some(tie_type))
                .context("selected source tie end missing")?;
            if tie_type == "start" {
                ensure!(color(tied)? == proposal.tie, "source mark does not bind to tie start");
            }
        }
    }
    Ok(())
}

fn checked_directions(
    before: &str, after: &str, expected: &BTreeSet<NoteOccurrence>,
) -> Result<BTreeMap<NoteOccurrence, (Side, Side)>> {
    let baseline = read_ink_roster(before)?;
    let candidate = read_ink_roster(after)?;
    ensure!(baseline.beams == candidate.beams && baseline.stems.len() == candidate.stems.len(),
        "candidate changed the authored stem or beam inventory");
    let plain = Document::parse(before)?;
    let mut edits = Vec::new();
    let mut directions = BTreeMap::new();
    for (a, b) in baseline.stems.iter().zip(&candidate.stems) {
        ensure!(a.heads.iter().map(|head| head.id).eq(b.heads.iter().map(|head| head.id))
            && a.flag_levels == b.flag_levels, "candidate changed stem ownership");
        if a.direction == b.direction { continue; }
        let [owner] = a.heads.as_slice() else { anyhow::bail!("preview proof needs a single own attachment head"); };
        ensure!(a.flag_levels == 0 && !baseline.beams.iter().any(|beam| beam.members.contains(&owner.id)),
            "selected stem has its own beam or flag; attachment proof is incomplete");
        let side = |direction| match direction {
            Some(StemDirection::Up) => Ok((Side::Above, "<stem>up</stem>")),
            Some(StemDirection::Down) => Ok((Side::Below, "<stem>down</stem>")),
            None => anyhow::bail!("selected source stem has no explicit direction"),
        };
        let (old, old_xml) = side(a.direction)?;
        let (new, new_xml) = side(b.direction)?;
        let stem = child(note_at(&plain, owner.id).context("selected source note missing")?, "stem")
            .context("selected source stem missing")?;
        ensure!(&before[stem.range()] == old_xml, "selected source stem is not a plain direction");
        edits.push((stem.range(), new_xml));
        directions.insert(owner.id, (old, new));
    }
    ensure!(directions.keys().copied().eq(expected.iter().copied()), "candidate changes a different source roster");
    edits.sort_by_key(|(range, _)| range.start);
    let mut offset = 0;
    let mut remaining = after;
    for (range, direction) in edits {
        remaining = remaining.strip_prefix(&before[offset..range.start]).and_then(|part| part.strip_prefix(direction))
            .context("candidate changed MusicXML outside the planned stems")?;
        offset = range.end;
    }
    ensure!(remaining == &before[offset..], "candidate changed MusicXML outside the planned stems");
    Ok(directions)
}

fn check_attachments(
    baseline: &PdfPaint, candidate: &PdfPaint, proposal: &Proposal,
    directions: &BTreeMap<NoteOccurrence, (Side, Side)>,
) -> Result<Decision> {
    for end in &proposal.ends {
        let attachment = |frame: &PdfPaint, side| -> Result<bool> {
            let (_, head, stem) = frame.anchors.iter().find(|(id, _, _)| *id == end.stem)
                .context("selected PDF attachment not observed")?;
            proves_single_head_attachment(stem, head, side)
        };
        let (before, after) = directions[&end.note];
        if !(attachment(baseline, before)? && attachment(candidate, after)?) {
            return Ok(Decision::Unknown);
        }
    }
    Ok(Decision::Proven)
}

fn tie_section(curve: &[(f64, f64); 4], at: f64) -> Option<[f64; 2]> {
    let mut points = *curve;
    if points[0].0 > points[3].0 { points.reverse(); }
    if !(points[0].0 < at && at < points[3].0 && points.windows(2).all(|pair| pair[0].0 < pair[1].0)) {
        return None;
    }
    let evaluate = |t: f64| {
        let s = 1.0 - t;
        let weights = [s*s*s, 3.0*s*s*t, 3.0*s*t*t, t*t*t];
        points.iter().zip(weights).fold((0.0, 0.0), |(x, y), (&(px, py), w)| (x + w*px, y + w*py))
    };
    let (mut a, mut b) = (0.0_f64, 1.0_f64);
    for _ in 0..36 {
        let middle = a.midpoint(b);
        if evaluate(middle).0 < at { a = middle; } else { b = middle; }
    }
    let y = evaluate(a.midpoint(b)).1;
    let slope_bound = points.windows(2).map(|pair| 3.0 * (pair[1].1 - pair[0].1).abs()).fold(0.0_f64, f64::max);
    let error = slope_bound * (b - a) + 0.015;
    Some([y - error, y + error])
}

#[test]
fn tie_section_requires_a_strictly_monotone_crossing() {
    let curve = [(0.0, 3.0), (1.0, 3.0), (2.0, 3.0), (3.0, 3.0)];
    let [lower, upper] = tie_section(&curve, 1.0).unwrap();
    assert!(lower < 3.0 && upper > 3.0 && upper - lower < 0.031);
    assert!(tie_section(&curve, 3.0).is_none());
    assert!(tie_section(&[curve[0], curve[2], curve[1], curve[3]], 1.0).is_none());
}

fn crosses_barline(arc: &PdfCurve, line: &PdfLine) -> bool {
    let [(x, y1), (end_x, y2)] = line.points;
    let [a, b, c, d, e, f, g] = arc.controls.as_slice() else { return false; };
    if !line.barline || x != end_x || line.width <= 0.03 || a != g || arc.ink.page != line.ink.page {
        return false;
    }
    let Some([first, second]) = tie_section(&[*a, *b, *c, *d], x).zip(tie_section(&[*d, *e, *f, *g], x))
        .map(|(first, second)| [first, second]) else { return false; };
    first[0].min(second[0]) > y1.min(y2) && first[1].max(second[1]) < y1.max(y2)
        && (first[1] < second[0] || second[1] < first[0])
}

fn preserved_barline(baseline: &PdfPaint, candidate: &PdfPaint, proposal: &Proposal) -> Option<(usize, [(f64, f64); 2])> {
    if proposal.ends[0].note.measure.checked_add(1) != Some(proposal.ends[1].note.measure) { return None; }
    let unique_crossing = |frame: &PdfPaint| {
        let mut found = frame.pages.iter().flat_map(|page| page.ties.iter()
            .filter(|arc| arc.mark == proposal.tie).flat_map(|arc| {
                let endpoint = |mark| page.glyphs.iter().find(|glyph| glyph.mark == mark).map(|glyph| glyph.ink.bounds);
                let ends = endpoint(proposal.ends[0].head).zip(endpoint(proposal.ends[1].head));
                page.strokes.iter().filter_map(move |line| {
                    let (left, right) = ends?;
                    let x = line.points[0].0;
                    (left[1][0] < x && x < right[0][0] && crosses_barline(arc, line))
                        .then_some((line.ink.page, line.points, line.width))
                })
            }));
        let crossing = found.next()?;
        found.next().is_none().then_some(crossing)
    };
    let first = unique_crossing(baseline)?;
    let second = unique_crossing(candidate)?;
    (first == second).then_some((second.0, second.1))
}

fn check_foreign_ink(baseline: &PdfPaint, candidate: &PdfPaint, proposal: &Proposal) -> Decision {
    let occurrences = |frame: &PdfPaint| frame.pages.iter().flat_map(|page| &page.ties)
        .filter(|arc| arc.mark == proposal.tie).count();
    if occurrences(baseline) != 1 || occurrences(candidate) != 1 { return Decision::Unknown; }
    let mut outside = Decision::Proven;
    let crossing = preserved_barline(baseline, candidate, proposal);
    let heads = proposal.ends.each_ref().map(|end| end.head);
    let stems = proposal.ends.each_ref().map(|end| end.stem);
    for page in &candidate.pages {
        let stems_here: Vec<_> = page.strokes.iter().filter(|line| stems.contains(&line.mark)).collect();
        if stems_here.is_empty() && !page.ties.iter().any(|arc| arc.mark == proposal.tie) { continue; }
        let minimum = candidate.anchors.iter().filter(|(mark, _, _)| stems.contains(mark))
            .map(|(_, _, stem)| stem.width).fold(0.0_f64, f64::max) + 0.015;
        for stem in stems_here {
            let own = proposal.ends.iter().find(|end| end.stem == stem.mark).map(|end| end.head);
            for ink in page.glyphs.iter().filter(|glyph| Some(glyph.mark) != own).map(|glyph| glyph.ink)
                .chain(page.ties.iter().map(|arc| arc.ink)).chain(page.other_fills.iter().copied())
                .chain(page.strokes.iter().filter(|line| line.mark != stem.mark && !line.staffline).map(|line| line.ink))
            {
                outside = outside.max(foreign_clearance(stem.ink, ink, minimum));
            }
        }
        for arc in page.ties.iter().filter(|arc| arc.mark == proposal.tie) {
            for ink in page.glyphs.iter().filter(|glyph| !heads.contains(&glyph.mark)).map(|glyph| glyph.ink)
                .chain(page.ties.iter().filter(|other| other.mark != arc.mark).map(|other| other.ink))
                .chain(page.other_fills.iter().copied())
                .chain(page.strokes.iter().filter(|line| {
                    crossing != Some((line.ink.page, line.points)) && !line.staffline
                }).map(|line| line.ink))
            {
                outside = outside.max(foreign_clearance(arc.ink, ink, minimum));
            }
        }
    }
    outside
}

fn bound_preview(frame: &PdfReviewFrame<'_>, proposals: &[Proposal]) -> Result<PdfPaint> {
    paired_xml_marks(frame.source, frame.marked_source, proposals)?;
    let targets: Vec<_> = proposals.iter().flat_map(|proposal| &proposal.ends)
        .map(|end| TrackedStem { head: end.head, stem: end.stem }).collect();
    let mut pages = read_paired_paint(frame.pdf, frame.marked_pdf, &targets)?;
    bind_guides(&mut pages, frame.svg)?;
    Ok(pages)
}

pub(crate) fn review_tied_stem_previews(
    baseline: &PdfReviewFrame<'_>, candidates: &[(usize, PdfReviewFrame<'_>)],
    composed: &PdfReviewFrame<'_>, source_marks: &str,
) -> Result<String> {
    let proposals = read_proposals(source_marks)?;
    ensure!(!proposals.is_empty() && candidates.len() == proposals.len(), "incomplete stem group previews");
    let by_group: BTreeMap<_, _> = candidates.iter().map(|(id, candidate)| (*id, candidate)).collect();
    ensure!(by_group.len() == candidates.len(), "duplicate rendered stem proposal");
    let before = bound_preview(baseline, &proposals).context("baseline readback")?;
    let mut report = String::new();
    for proposal in &proposals {
        let candidate = by_group.get(&proposal.id).context("group render absent")?;
        let after = bound_preview(candidate, &proposals).with_context(|| format!("proposal {} readback", proposal.id))?;
        let directions = checked_directions(baseline.source, candidate.source, &proposal.ends.iter().map(|end| end.note).collect())?;
        let stems = proposal.ends.each_ref().map(|end| end.stem);
        let scoped = delta_stays_on_marked_paint(&before, &after, &stems, &[proposal.tie]);
        let attachment = check_attachments(&before, &after, proposal, &directions)?;
        let clearance = check_foreign_ink(&before, &after, proposal);
        let decision = if scoped { attachment.max(clearance) } else { Decision::Unknown };
        writeln!(report, "Group {}: changes={} owned={attachment:?} foreign={clearance:?} scope={scoped} result={decision:?}",
            proposal.id, directions.len())?;
    }
    let candidate = bound_preview(composed, &proposals).context("combined preview readback")?;
    let expected = proposals.iter().flat_map(|proposal| &proposal.ends).map(|end| end.note).collect();
    let directions = checked_directions(baseline.source, composed.source, &expected)?;
    let stems = proposals.iter().flat_map(|proposal| &proposal.ends).map(|end| end.stem).collect::<Vec<_>>();
    let ties = proposals.iter().map(|proposal| proposal.tie).collect::<Vec<_>>();
    let scoped = delta_stays_on_marked_paint(&before, &candidate, &stems, &ties);
    let mut proven = 0;
    for proposal in &proposals {
        let owned = check_attachments(&before, &candidate, proposal, &directions)?;
        let foreign = check_foreign_ink(&before, &candidate, proposal);
        if scoped && owned.max(foreign) == Decision::Proven { proven += 1; }
        writeln!(report, "Combined group {}: owned={owned:?} foreign={foreign:?} scope={scoped}", proposal.id)?;
    }
    writeln!(report, "Combined preview: groups={} proven={proven} stem_changes={} scoped={scoped}; authority=none",
        proposals.len(), directions.len())?;
    Ok(report)
}
