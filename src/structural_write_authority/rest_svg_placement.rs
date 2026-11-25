use super::parse_atomic_notes;
use super::svg_notation_geometry::rests::{Scene, improves};
use crate::pipeline::PhraseSvgPreview;
use anyhow::{Context, Result};
use roxmltree::{Document, Node};
use std::collections::BTreeMap;
use std::ops::Range;

type Edit = (usize, usize, String);

struct Course {
    rests: Vec<Rest>,
    marks: Vec<Edit>,
}

struct Rest {
    note: Range<usize>,
    step: Range<usize>,
    octave: Range<usize>,
    anchor: i32,
    tags: [String; 3],
}

struct RestMove {
    note: Range<usize>,
    fields: [Edit; 2],
}

impl Rest {
    fn movement(&self, anchor: i32) -> Option<RestMove> {
        let letter = *b"CDEFGAB".get(usize::try_from(anchor.rem_euclid(7)).ok()?)?;
        let octave = anchor.div_euclid(7);
        if !(0..=9).contains(&octave) {
            return None;
        }
        Some(RestMove {
            note: self.note.clone(),
            fields: [
                (
                    self.step.start,
                    self.step.end,
                    char::from(letter).to_string(),
                ),
                (self.octave.start, self.octave.end, octave.to_string()),
            ],
        })
    }
}

impl RestMove {
    fn note_replacement(&self, source: &str) -> Option<Edit> {
        let replacement = splice(source, self.note.clone(), self.fields.iter())?;
        Some((self.note.start, self.note.end, replacement))
    }
}

fn child<'a, 'input>(node: Node<'a, 'input>, tag: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|n| n.has_tag_name(tag))
}

fn course(source: &str) -> Option<Course> {
    let doc = Document::parse(source).ok()?;
    let root = doc.root_element();
    let mut parts = root.children().filter(|n| n.has_tag_name("part"));
    let part = parts.next()?;
    if !root.has_tag_name("score-partwise") || parts.next().is_some() {
        return None;
    }
    if !part
        .descendants()
        .filter(|n| n.has_tag_name("staves"))
        .map(|n| n.text())
        .eq(std::iter::once(Some("2")))
    {
        return None;
    }

    // Same written voice through the whole measure, including hidden carriers
    // and other staffs. These can interrupt a local three-event course.
    let notes: BTreeMap<_, _> = part
        .descendants()
        .filter(|n| n.has_tag_name("note"))
        .map(|n| (n.range().start, n))
        .collect();
    let mut voices = BTreeMap::<_, Vec<_>>::new();
    for event in parse_atomic_notes(source) {
        let measure = notes.get(&event.note_start)?.parent_element()?;
        if !measure.has_tag_name("measure") {
            return None;
        }
        let key = (measure.range().start, event.voice.clone());
        voices.entry(key).or_default().push(event);
    }
    let mut bass_clef = false;
    let mut bass_measures = BTreeMap::new();
    for measure in part.children().filter(|n| n.has_tag_name("measure")) {
        let mut saw_note = false;
        let mut stable = true;
        for item in measure.children().filter(Node::is_element) {
            if item.has_tag_name("note") {
                saw_note = true;
            }
            if item.has_tag_name("attributes") {
                for clef in item
                    .children()
                    .filter(|n| n.has_tag_name("clef") && n.attribute("number") == Some("2"))
                {
                    stable &= !saw_note;
                    bass_clef = child(clef, "sign").and_then(|n| n.text()) == Some("F")
                        && child(clef, "line").and_then(|n| n.text()) == Some("4")
                        && child(clef, "clef-octave-change").is_none();
                }
            }
        }
        bass_measures.insert(measure.range().start, stable && bass_clef);
    }

    let mut rests = Vec::new();
    let mut markers = BTreeMap::<usize, String>::new();
    for ((measure, _), voice) in &mut voices {
        if !bass_measures.get(measure).copied().unwrap_or(false) {
            continue;
        }
        voice.sort_by_key(|event| (event.start, event.note_start));
        for (index, window) in voice.windows(3).enumerate() {
            let [a, r, b] = window else { unreachable!() };
            if [a, r, b].iter().any(|event| {
                event.staff != "2" || !event.visible || event.is_chord || event.duration <= 0
            }) || a.pitch.is_none()
                || r.pitch.is_some()
                || b.pitch.is_none()
                || a.start.checked_add(a.duration) != Some(r.start)
                || r.start.checked_add(r.duration) != Some(b.start)
                || voice
                    .get(index + 3)
                    .is_some_and(|next| next.start == b.start)
            {
                continue;
            }
            let positions = [a.note_start, r.note_start, b.note_start];
            let [Some(l), Some(rest_note), Some(rr)] =
                positions.map(|start| notes.get(&start).copied())
            else {
                continue;
            };
            if [l, rest_note, rr].iter().any(|n| {
                n.attribute("color").is_some() || n.attribute("print-object") == Some("no")
            }) || !matches!(
                child(rest_note, "type").and_then(|n| n.text()),
                Some("16th" | "eighth" | "32nd" | "64th")
            ) || child(rest_note, "dot").is_some()
                || child(rest_note, "beam").is_some()
                || child(rest_note, "time-modification").is_some()
                || child(rest_note, "notations").is_some()
            {
                continue;
            }
            let Some(rest) = child(rest_note, "rest") else {
                continue;
            };
            if rest.attribute("measure") == Some("yes") {
                continue;
            }
            let (Some(step), Some(octave)) =
                (child(rest, "display-step"), child(rest, "display-octave"))
            else {
                continue;
            };
            let (Some(letter), Some(number)) = (
                step.text()
                    .and_then(|s| "CDEFGAB".find(s).filter(|_| s.len() == 1)),
                octave
                    .text()
                    .and_then(|s| s.parse::<i32>().ok())
                    .filter(|v| (0..=9).contains(v)),
            ) else {
                continue;
            };
            let (Some(step), Some(octave)) = (
                step.first_child().filter(Node::is_text),
                octave.first_child().filter(Node::is_text),
            ) else {
                continue;
            };
            for start in positions {
                let marker = u16::try_from(markers.len()).ok()?;
                markers
                    .entry(start)
                    .or_insert_with(|| format!("#a2{marker:04x}"));
            }
            rests.push(Rest {
                note: rest_note.range(),
                step: step.range(),
                octave: octave.range(),
                anchor: 7 * number + i32::try_from(letter).ok()?,
                tags: positions.map(|start| markers[&start].clone()),
            });
        }
    }
    let marks = markers
        .into_iter()
        .map(|(start, tag)| {
            let at = start + "<note".len();
            (at, at, format!(" color=\"{tag}\""))
        })
        .collect();
    Some(Course { rests, marks })
}

fn splice<'a>(
    source: &'a str,
    bounds: Range<usize>,
    edits: impl Iterator<Item = &'a Edit>,
) -> Option<String> {
    let mut edits = edits.collect::<Vec<_>>();
    edits.sort_unstable_by_key(|(start, _, _)| *start);
    let mut spans = Vec::with_capacity(2 * edits.len() + 1);
    let mut cursor = bounds.start;
    for (start, end, text) in edits {
        if start > end || *end > bounds.end {
            return None;
        }
        spans.extend([source.get(cursor..*start)?, text.as_str()]);
        cursor = *end;
    }
    spans.push(source.get(cursor..bounds.end)?);
    Some(spans.concat())
}

fn preview_xml(source: &str, moves: &[RestMove], marks: &[Edit]) -> Option<String> {
    // The trials are composed from disjoint source spans. Color marks are
    // before the field replacements, even when both belong to one note.
    let edits = moves.iter().flat_map(|plan| &plan.fields).chain(marks);
    splice(source, 0..source.len(), edits)
}

struct VerifiedPlacement {
    moves: Vec<RestMove>,
    plain_xml: String,
}

fn placement(
    source: &str,
    mut render: impl FnMut(&str) -> Option<Vec<String>>,
) -> Option<VerifiedPlacement> {
    let Course { rests, marks } = course(source)?;
    if rests.is_empty() {
        return None;
    }
    let mut read = |xml: &str| Scene::read(&render(xml)?);
    let baseline = read(source)?;
    let marked = read(&preview_xml(source, &[], &marks)?)?;
    if !marked.proof_of_marks(&baseline) {
        return None;
    }
    let origins = rests
        .iter()
        .map(|rest| marked.reading(&rest.tags))
        .collect::<Vec<_>>();
    let anchors = rests
        .iter()
        .zip(&origins)
        .map(|(rest, origin)| {
            let origin = origin.as_ref()?;
            // A diatonic step is half a staff space. This only seeds the trials:
            // the renderer can clamp or move the result by a different amount.
            let steps = (2.0 * (origin.height - origin.goal) / origin.space).round() as i32;
            rest.anchor.checked_add(steps)
        })
        .collect::<Vec<_>>();
    if anchors.iter().all(Option::is_none) {
        return None;
    }
    let mut best = vec![None::<(f64, f64, i32)>; rests.len()];
    for offset in [0, -1, 1, -2, 2] {
        let edits = rests
            .iter()
            .zip(&anchors)
            .filter_map(|(rest, anchor)| {
                let anchor = (*anchor)?.checked_add(offset)?;
                (rest.anchor != anchor)
                    .then(|| rest.movement(anchor))
                    .flatten()
            })
            .collect::<Vec<_>>();
        if edits.is_empty() {
            continue;
        }
        let Some(trial) = read(&preview_xml(source, &edits, &marks)?) else {
            continue;
        };
        for (((rest, anchor), origin), ranking) in
            rests.iter().zip(&anchors).zip(&origins).zip(&mut best)
        {
            let (Some(anchor), Some(origin), Some(after)) =
                (anchor, origin, trial.reading(&rest.tags))
            else {
                continue;
            };
            let anchor = anchor.checked_add(offset)?;
            if rest.anchor == anchor
                || rest.movement(anchor).is_none()
                || !improves(origin, &after)
                || trial.clearance(&after) < 0.50
            {
                continue;
            }
            let error = (after.height - origin.goal).abs();
            let travel = (after.height - origin.height).abs();
            if ranking
                .is_none_or(|(old_error, old_travel, _)| (error, travel) < (old_error, old_travel))
            {
                *ranking = Some((error, travel, anchor));
            }
        }
    }
    // A rejected move can no longer be an obstacle. Check the surviving
    // combination again, with no ranking across independent rests.
    for _ in 0..2 {
        let edits = rests
            .iter()
            .zip(&best)
            .filter_map(|(rest, rank)| {
                let (_, _, anchor) = (*rank)?;
                rest.movement(anchor)
            })
            .collect::<Vec<_>>();
        if edits.is_empty() {
            return None;
        }
        let plain_xml = preview_xml(source, &edits, &[])?;
        let final_scene = read(&plain_xml)?;
        let final_marked = read(&preview_xml(source, &edits, &marks)?)?;
        if !final_marked.proof_of_marks(&final_scene) {
            return None;
        }
        let mut verified = Vec::new();
        for ((rest, origin), rank) in rests.iter().zip(&origins).zip(&mut best) {
            if rank.is_none() {
                continue;
            }
            let before = origin.as_ref()?;
            if let Some(after) = final_marked.reading(&rest.tags)
                && final_scene.verifies_move(before, &after)
            {
                verified.push(before);
            } else {
                *rank = None;
            }
        }
        if verified.len() == edits.len() {
            return final_scene
                .verifies_neighborhood(&baseline, &verified)
                .then_some(VerifiedPlacement {
                    moves: edits,
                    plain_xml,
                });
        }
    }
    None
}

pub(crate) fn adopt(
    state: &mut crate::structural_write_authority::RenderSovereignState,
    preview: &mut PhraseSvgPreview,
) -> Result<usize> {
    if preview.wants_slur_paint() {
        return Ok(0);
    }
    let root = state.require_state_root("FinalRenderedRestHeight");
    let Some(verified) = placement(root.document(), |xml| {
        preview.render(xml, |path, bytes| {
            crate::structural_write_authority::write_renderer_feasibility_shadow_candidate_authorized(path, bytes).ok()
        })
    }) else {
        return Ok(0);
    };
    let edits = verified
        .moves
        .iter()
        .map(|plan| plan.note_replacement(root.document()))
        .collect::<Option<Vec<_>>>()
        .context("Rest-height proof spans")?;
    let count = edits.len();
    let (post, receipt) =
        crate::structural_write_authority::apply_final_rest_glyph_height_authorized(
            root.document(),
            edits,
        );
    let post = post.context("Rest-height authority rejected proven MusicXML spans")?;
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    if post != verified.plain_xml {
        receipts
            .discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        anyhow::bail!("Rest-height authority changed the proven bytes");
    }
    use crate::structural_write_authority::{RenderCheckpointSink, RenderSinkKind};
    state.adopt_transition(
        root,
        &post,
        receipts,
        RenderSinkKind::Checkpoint(RenderCheckpointSink::FinalPianoRestGlyph),
        |path, _, candidate| {
            crate::structural_write_authority::write_final_piano_rest_glyph_checkpoint_authorized(
                path, candidate,
            )
        },
    )?;
    preview.adopted = true;
    Ok(count)
}
