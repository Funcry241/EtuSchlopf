//! Source ownership of pitched heads and authored beam fragments.
//!
//! Ordinals are local to the score, stable across a stem edit, and carry no
//! engraved geometry. A renderer association needs independent ink evidence.

use std::{collections::BTreeMap, ops::Range};

use anyhow::{Context, Result, bail, ensure};
use roxmltree::{Document, Node};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct NoteOccurrence {
    pub(crate) part: usize,
    pub(crate) measure: usize,
    /// Ordinal among all note elements, including rests and grace notes.
    pub(crate) note: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HeadSource {
    pub(crate) id: NoteOccurrence,
    pub(crate) range: Range<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StemDirection {
    Up,
    Down,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StemOwner {
    /// The first head is the note that owns the shared stem.
    pub(crate) heads: Vec<HeadSource>,
    pub(crate) direction: Option<StemDirection>,
    /// Eligible flag depth when the source does not author a beam.
    /// The renderer can still choose an implicit beam.
    pub(crate) flag_levels: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BeamHook {
    Forward,
    Backward,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BeamOwner {
    pub(crate) members: Vec<NoteOccurrence>,
    pub(crate) level: u8,
    pub(crate) hook: Option<BeamHook>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct InkRoster {
    pub(crate) stems: Vec<StemOwner>,
    pub(crate) beams: Vec<BeamOwner>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BeamMark {
    Begin,
    Continue,
    End,
    Hook(BeamHook),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Lane<'a> {
    voice: &'a str,
    grace: bool,
}

struct ChordContext<'a> {
    lane: Lane<'a>,
    staff: u16,
    duration: Option<u64>,
    kind: &'a str,
    beams: BTreeMap<u8, BeamMark>,
}

type OpenBeams<'a> = BTreeMap<(Lane<'a>, u8), Vec<NoteOccurrence>>;

fn unique_child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Result<Option<Node<'a, 'input>>> {
    let mut matching = node.children().filter(|child| child.has_tag_name(name));
    let first = matching.next();
    ensure!(matching.next().is_none(), "duplicate MusicXML {name}");
    Ok(first)
}

fn scalar_value<'a>(node: Node<'a, '_>) -> Result<&'a str> {
    ensure!(
        !node.children().any(|child| child.is_element()),
        "nested MusicXML {} value",
        node.tag_name().name()
    );
    node.text()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .with_context(|| format!("empty MusicXML {} value", node.tag_name().name()))
}

fn child_value<'a>(node: Node<'a, '_>, name: &str) -> Result<Option<&'a str>> {
    unique_child(node, name)?.map(scalar_value).transpose()
}

fn note_kind(kind: &str) -> Result<(bool, u8)> {
    let flags = match kind {
        "maxima" | "long" | "breve" | "whole" => return Ok((false, 0)),
        "half" | "quarter" => 0,
        "eighth" => 1,
        "16th" => 2,
        "32nd" => 3,
        "64th" => 4,
        "128th" => 5,
        "256th" => 6,
        "512th" => 7,
        "1024th" => 8,
        _ => bail!("unsupported MusicXML note type {kind}"),
    };
    Ok((true, flags))
}

fn beam_marks(node: Node<'_, '_>, flags: u8) -> Result<BTreeMap<u8, BeamMark>> {
    let mut marks = BTreeMap::new();
    for beam in node.children().filter(|child| child.has_tag_name("beam")) {
        let level = beam.attribute("number").unwrap_or("1").parse::<u8>()?;
        ensure!((1..=flags).contains(&level), "beam level exceeds written note type");
        let mark = match scalar_value(beam)? {
            "begin" => BeamMark::Begin,
            "continue" => BeamMark::Continue,
            "end" => BeamMark::End,
            "forward hook" => BeamMark::Hook(BeamHook::Forward),
            "backward hook" => BeamMark::Hook(BeamHook::Backward),
            _ => bail!("unsupported MusicXML beam mark"),
        };
        ensure!(marks.insert(level, mark).is_none(), "duplicate MusicXML beam level");
    }
    Ok(marks)
}

fn track_beams<'a>(
    owner: NoteOccurrence,
    lane: Lane<'a>,
    marks: &BTreeMap<u8, BeamMark>,
    open: &mut OpenBeams<'a>,
    beams: &mut Vec<BeamOwner>,
) -> Result<()> {
    ensure!(
        open.keys().all(|(active_lane, level)| *active_lane != lane || marks.contains_key(level)),
        "active MusicXML beam has an unowned intervening note"
    );
    for (&level, &mark) in marks {
        let key = (lane, level);
        match mark {
            BeamMark::Begin => {
                ensure!(open.insert(key, vec![owner]).is_none(), "overlapping MusicXML beam starts");
            }
            BeamMark::Continue => open
                .get_mut(&key)
                .context("MusicXML beam continuation without start")?
                .push(owner),
            BeamMark::End => {
                let mut members = open.remove(&key).context("MusicXML beam end without start")?;
                members.push(owner);
                beams.push(BeamOwner { members, level, hook: None });
            }
            BeamMark::Hook(hook) => {
                ensure!(!open.contains_key(&key), "MusicXML beam hook interrupts an active beam");
                beams.push(BeamOwner { members: vec![owner], level, hook: Some(hook) });
            }
        }
    }
    Ok(())
}

fn ensure_nested_beams(beams: &[BeamOwner]) -> Result<()> {
    let mut owners = BTreeMap::new();
    for (index, beam) in beams.iter().enumerate() {
        for member in &beam.members {
            ensure!(
                owners.insert((beam.level, *member), index).is_none(),
                "multiple authored beams own the same note at one level"
            );
        }
    }
    for beam in beams.iter().filter(|beam| beam.level > 1) {
        let parent_level = beam.level - 1;
        let parent = owners.get(&(parent_level, beam.members[0]))
            .context("MusicXML beam has no enclosing lower level")?;
        ensure!(
            beams[*parent].members.windows(beam.members.len()).any(|window| window == beam.members),
            "higher MusicXML beam crosses its enclosing lower-level fragment"
        );
    }
    Ok(())
}

fn read_measure<'a>(
    measure: Node<'a, '_>,
    part: usize,
    measure_index: usize,
    roster: &mut InkRoster,
) -> Result<()> {
    let mut open = OpenBeams::new();
    let mut chord_owner: Option<ChordContext<'a>> = None;
    let mut note_index = 0;
    let first_beam = roster.beams.len();

    for note in measure.children().filter(|child| child.is_element()) {
        if !note.has_tag_name("note") {
            chord_owner = None;
            continue;
        }
        let id = NoteOccurrence { part, measure: measure_index, note: note_index };
        note_index += 1;
        let is_chord = unique_child(note, "chord")?.is_some();
        let lane = Lane {
            voice: child_value(note, "voice")?.unwrap_or("1"),
            grace: unique_child(note, "grace")?.is_some(),
        };
        if unique_child(note, "rest")?.is_some() {
            ensure!(!is_chord, "a MusicXML rest cannot own a chord head");
            ensure!(beam_marks(note, 0)?.is_empty(), "beamed rest has no qualified stem owner");
            ensure!(
                open.keys().all(|(active_lane, _)| *active_lane != lane),
                "MusicXML beam across a rest has no qualified stem owner"
            );
            chord_owner = None;
            continue;
        }
        ensure!(unique_child(note, "pitch")?.is_some(), "MusicXML note has no pitched head");
        ensure!(
            note.attribute("print-object") != Some("no")
                && child_value(note, "notehead")? != Some("none"),
            "invisible MusicXML head has no qualified ink identity"
        );
        let staff = child_value(note, "staff")?.unwrap_or("1").parse::<u16>()?;
        ensure!(staff > 0, "zero MusicXML staff number");
        let duration = child_value(note, "duration")?.map(str::parse::<u64>).transpose()?;
        ensure!(
            if lane.grace { duration.is_none() } else { duration.is_some_and(|value| value > 0) },
            "unqualified MusicXML note duration"
        );
        let kind = child_value(note, "type")?.context("MusicXML note type is missing")?;
        let (stemmed, flags) = note_kind(kind)?;
        let authored_direction = child_value(note, "stem")?;
        let direction = match authored_direction {
            Some("up") if stemmed => Some(StemDirection::Up),
            Some("down") if stemmed => Some(StemDirection::Down),
            Some("none") | None => None,
            _ => bail!("unqualified MusicXML stem direction"),
        };
        let marks = beam_marks(note, flags)?;
        let head = HeadSource { id, range: note.range() };

        if is_chord {
            let context = chord_owner.as_ref().context("MusicXML chord head has no preceding owner")?;
            let stem = roster.stems.last_mut().context("MusicXML chord stem owner is missing")?;
            ensure!(
                (context.lane, context.staff, context.duration, context.kind)
                    == (lane, staff, duration, kind)
                    && authored_direction.is_none_or(|_| direction == stem.direction)
                    && marks.iter().all(|(level, mark)| context.beams.get(level) == Some(mark)),
                "MusicXML chord head disagrees with its stem owner"
            );
            stem.heads.push(head);
            continue;
        }

        ensure!(!stemmed || authored_direction.is_some(), "MusicXML stem is not explicit");
        ensure!(
            marks.is_empty()
                || (direction.is_some() && marks.len() == usize::from(flags)),
            "unqualified MusicXML beam-level inventory"
        );
        track_beams(id, lane, &marks, &mut open, &mut roster.beams)?;
        roster.stems.push(StemOwner {
            heads: vec![head],
            direction,
            flag_levels: if direction.is_some() && marks.is_empty() { flags } else { 0 },
        });
        chord_owner = Some(ChordContext { lane, staff, duration, kind, beams: marks });
    }
    ensure!(open.is_empty(), "MusicXML beam is not closed at the measure boundary");
    ensure_nested_beams(&roster.beams[first_beam..])
}

pub(crate) fn read_ink_roster(xml: &str) -> Result<InkRoster> {
    let document = Document::parse(xml)?;
    let score = document.root_element();
    ensure!(score.has_tag_name("score-partwise"), "expected a partwise MusicXML score");
    let mut roster = InkRoster::default();
    for (part, node) in score.children().filter(|child| child.has_tag_name("part")).enumerate() {
        for (measure, node) in node.children().filter(|child| child.has_tag_name("measure")).enumerate() {
            read_measure(node, part, measure, &mut roster)
                .with_context(|| format!("MusicXML ink ownership in part {}, measure {}", part + 1, measure + 1))?;
        }
    }
    ensure!(!roster.stems.is_empty(), "MusicXML score has no qualified pitched notes");
    Ok(roster)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn score(notes: &str) -> String {
        format!("<score-partwise><part id=\"P1\"><measure number=\"fixture\">{notes}</measure></part></score-partwise>")
    }

    fn note(pitch: &str, kind: &str, before: &str, after: &str) -> String {
        let duration = if before.contains("<grace") { "" } else { "<duration>1</duration>" };
        format!("<note>{before}<pitch><step>{pitch}</step><octave>4</octave></pitch>{duration}<voice>1</voice><type>{kind}</type><stem>up</stem>{after}</note>")
    }

    #[test]
    fn one_chord_stem_owns_each_exact_head_in_both_beam_levels() {
        let start = note("C", "16th", "", "<beam>begin</beam><beam number=\"2\">begin</beam>");
        let member = note("E", "16th", "<chord/>", "");
        let end = note("G", "16th", "", "<beam>end</beam><beam number=\"2\">end</beam>");
        let xml = score(&format!("{start}{member}{end}"));
        let roster = read_ink_roster(&xml).unwrap();
        assert_eq!(roster.stems.len(), 2);
        assert_eq!(roster.stems[0].direction, Some(StemDirection::Up));
        assert_eq!(roster.stems[0].flag_levels, 0);
        assert_eq!(&xml[roster.stems[0].heads[1].range.clone()], member);
        assert_eq!(roster.stems[0].heads[1].id.note, 1);
        for (level, beam) in roster.beams.iter().enumerate() {
            assert_eq!(usize::from(beam.level), level + 1);
            assert_eq!(beam.members, [roster.stems[0].heads[0].id, roster.stems[1].heads[0].id]);
            assert_eq!(beam.hook, None);
        }
    }

    #[test]
    fn flags_and_hooks_keep_their_own_sources_across_grace_and_rest() {
        let flag = note("C", "16th", "<grace slash=\"yes\"/>", "");
        let start = note("E", "16th", "", "<beam>begin</beam><beam number=\"2\">forward hook</beam>");
        let end = note("G", "eighth", "", "<beam>end</beam>");
        let rest = "<note><rest/><duration>1</duration><voice>1</voice></note>";
        let xml = score(&format!("{flag}{start}{end}{rest}{flag}"));
        let roster = read_ink_roster(&xml).unwrap();
        assert_eq!(roster.stems[0].flag_levels, 2);
        assert_eq!(roster.stems[3].flag_levels, 2);
        assert_eq!(roster.stems[3].heads[0].id.note, 4);
        assert_eq!(roster.beams[0].level, 2);
        assert_eq!(roster.beams[0].hook, Some(BeamHook::Forward));
        assert_eq!(roster.beams[0].members, [roster.stems[1].heads[0].id]);
    }

    #[test]
    fn unowned_chord_or_beam_ink_fails_closed() {
        let base = note("C", "eighth", "", "");
        let chord = note("E", "eighth", "<chord/>", "");
        let start = note("C", "eighth", "", "<beam>begin</beam>");
        let end = note("E", "eighth", "", "<beam>end</beam>");
        let crossing = note("E", "16th", "", "<beam>end</beam><beam number=\"2\">continue</beam>");
        let nested_start = note("C", "16th", "", "<beam>begin</beam><beam number=\"2\">begin</beam>");
        let nested_end = note("G", "16th", "", "<beam>end</beam><beam number=\"2\">end</beam>");
        let new_parent = note("G", "16th", "", "<beam>begin</beam><beam number=\"2\">continue</beam>");
        for text in [
            chord.clone(),
            format!("{base}<backup><duration>1</duration></backup>{chord}"),
            format!("{base}{}", chord.replace("<stem>up", "<stem>down")),
            format!("{base}{}", chord.replace("<voice>1", "<voice>2")),
            start.clone(),
            end.clone(),
            format!("{start}{base}{end}"),
            format!("{nested_start}{crossing}{new_parent}{nested_end}"),
        ] {
            assert!(read_ink_roster(&score(&text)).is_err(), "{text}");
        }
    }
}
