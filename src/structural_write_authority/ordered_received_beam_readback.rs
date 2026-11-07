//! Conditional readback of surviving owner edges at the FinalChord projection.
use super::*;
use super::actual_source_beam_binding::BoundOccurrence;
use super::original_beam_voice_alignment::actual_beam_levels;
use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Debug, PartialEq, Eq)]
enum EdgeReadback {
    Present,
    Missing,
    Unknown,
}

fn endpoint_readback(left: Option<&str>, right: Option<&str>) -> EdgeReadback {
    let outgoing = match left {
        Some("begin" | "continue") => true,
        Some("end") | None => false,
        _ => return EdgeReadback::Unknown,
    };
    let incoming = match right {
        Some("continue" | "end") => true,
        Some("begin") | None => false,
        _ => return EdgeReadback::Unknown,
    };
    if outgoing && incoming { EdgeReadback::Present } else { EdgeReadback::Missing }
}

struct Projection<'a> {
    written: &'a FinalChordWrittenProjectionEvidence,
    emissions: BTreeMap<usize, Vec<&'a FinalChordWrittenOccurrence>>,
    locations: BTreeMap<usize, (usize, usize)>,
    measures: Vec<FinalChordBindingMeasure>,
}

impl<'a> Projection<'a> {
    fn bind(written: &'a FinalChordWrittenProjectionEvidence) -> Option<Self> {
        let measures = final_chord_parse_binding_measures(&written.xml).ok()?;
        let mut locations = BTreeMap::new();
        for (measure_index, measure) in measures.iter().enumerate() {
            for (index, note) in measure.notes.iter().enumerate() {
                if locations.insert(note.range.start, (measure_index, index)).is_some() {
                    return None;
                }
            }
        }
        let mut emissions = BTreeMap::<usize, Vec<_>>::new();
        for row in written.occurrences.iter() {
            emissions.entry(row.fragment_id).or_default().push(row);
        }
        Some(Self { written, emissions, locations, measures })
    }

    fn locate(&self, original: &BoundOccurrence) -> Option<(usize, usize)> {
        let [row] = self.emissions.get(&original.fragment)?.as_slice() else { return None; };
        if row.is_grace || row.source_event_id != original.source || row.attack_id != original.attack
            || !row.has_pitched_chord_anchor || row.chord_anchor_start != row.projected_note_start
        {
            return None;
        }
        let &(measure, index) = self.locations.get(&row.projected_note_start)?;
        let note = &self.measures[measure].notes[index];
        (!note.chord_member && !note.rest_like && note.duration > 0).then_some((measure, index))
    }

    fn edge(&self, level: u8, left: &BoundOccurrence, right: &BoundOccurrence) -> Option<EdgeReadback> {
        let (measure, start) = self.locate(left)?;
        let (other_measure, end) = self.locate(right)?;
        if measure != other_measure || start >= end {
            return None;
        }
        let notes = &self.measures[measure].notes;
        let (a, b) = (&notes[start], &notes[end]);
        if a.staff != b.staff || a.voice != b.voice || a.voice.is_empty()
            || a.onset.checked_add(a.duration)? > b.onset
        {
            return None;
        }
        let xml = &self.written.xml;
        // A different pitched event or a marked rest in the same observed
        // strand needs its own ordered readback before adjacency is knowable.
        for note in &notes[start + 1..end] {
            if note.staff == a.staff && note.voice == a.voice
                && (!note.rest_like || actual_beam_levels(xml, note)?.contains_key(&level))
            {
                return None;
            }
        }
        let a = actual_beam_levels(xml, a)?;
        let b = actual_beam_levels(xml, b)?;
        Some(endpoint_readback(a.get(&level).map(String::as_str), b.get(&level).map(String::as_str)))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OwnerEdgeReadback {
    Present,
    SplitVoice,
    Interrupted { pitched_ordinal: usize },
    MarkedRest { note_index: usize },
    EndpointMarkersMissing,
    Unknown,
}

#[derive(Clone, Copy)]
pub(super) struct OwnerEdge<'a> {
    pub(super) level: u8,
    pub(super) group: (usize, usize),
    pub(super) group_size: usize,
    pub(super) left: &'a BoundOccurrence,
    pub(super) right: &'a BoundOccurrence,
}

type OwnerGroups<'a> = BTreeMap<(u8, (usize, usize)), Vec<(&'a BoundOccurrence, Option<&'a str>)>>;

fn owner_groups(rows: &[BoundOccurrence]) -> OwnerGroups<'_> {
    let mut groups = BTreeMap::<_, Vec<_>>::new();
    for row in rows {
        for (level, group, marker) in &row.levels {
            if let Some(group) = group {
                groups.entry((*level, *group)).or_default().push((row, marker.as_deref()));
            }
        }
    }
    groups
}

fn closed_group(members: &[(&BoundOccurrence, Option<&str>)]) -> bool {
    members.len() >= 2 && members.first().and_then(|row| row.1) == Some("begin")
        && members.last().and_then(|row| row.1) == Some("end")
        && members[1..members.len() - 1].iter().all(|row| row.1 == Some("continue"))
}

pub(super) fn owner_edges<'a>(
    canonical: &CanonicalFinalChordStructuralPlan,
    capture: &'a ActualSourceBeamBindingObservation,
) -> Result<Vec<OwnerEdge<'a>>, &'static str> {
    let rows = capture.graph_received_occurrences(canonical)?;
    let locators = canonical.source_locators.iter().map(|row| (row.fragment_id, row))
        .collect::<BTreeMap<_, _>>();
    let mut edges = Vec::new();
    for ((level, group), members) in owner_groups(rows) {
        if !closed_group(&members) { continue; }
        for pair in members.windows(2) {
            let (left, right) = (pair[0].0, pair[1].0);
            if locators.get(&left.fragment).zip(locators.get(&right.fragment)).is_some_and(|(a, b)|
                a.measure == b.measure && a.staff == b.staff
                    && a.onset.checked_add(a.duration).is_some_and(|end| end <= b.onset))
            {
                edges.push(OwnerEdge { level, group, group_size: members.len(), left, right });
            }
        }
    }
    Ok(edges)
}

pub(super) fn read_projected_owner_edges(
    xml: &str,
    measures: &[FinalChordBindingMeasure],
    witnesses: &[FinalChordTransactionOutputOccurrenceWitness],
    edges: &[OwnerEdge<'_>],
) -> Vec<OwnerEdgeReadback> {
    let mut locations = BTreeMap::new();
    for (measure_index, measure) in measures.iter().enumerate() {
        for (ordinal, (index, _)) in measure.notes.iter().enumerate()
            .filter(|(_, note)| !note.rest_like && note.pitch_midi.is_some()).enumerate()
        {
            locations.insert((measure.measure, ordinal), (measure_index, index));
        }
    }
    let mut emissions = BTreeMap::<_, Vec<_>>::new();
    for witness in witnesses {
        emissions.entry(witness.fragment_id).or_default().push(witness);
    }
    let locate = |row: &BoundOccurrence| {
        let [witness] = emissions.get(&row.fragment)?.as_slice() else { return None; };
        if witness.source_event_id != row.source || witness.attack_id != row.attack {
            return None;
        }
        locations.get(&(witness.measure, witness.pitched_ordinal)).copied()
    };
    edges.iter().map(|edge| {
        let (Some((measure, left)), Some((other, right))) = (locate(edge.left), locate(edge.right)) else {
            return OwnerEdgeReadback::Unknown;
        };
        if measure != other { return OwnerEdgeReadback::Unknown; }
        read_projected_edge(xml, &measures[measure].notes, left, right, edge.level)
    }).collect()
}

fn read_projected_edge(
    xml: &str, notes: &[FinalChordEncodingNote], left: usize, right: usize, level: u8,
) -> OwnerEdgeReadback {
    let (a, b) = (&notes[left], &notes[right]);
    if left >= right || a.staff != b.staff || a.voice.is_empty() || b.voice.is_empty()
        || [a, b].iter().any(|note| note.chord_member || note.rest_like || note.duration <= 0)
        || a.onset.checked_add(a.duration).is_none_or(|end| end > b.onset)
    {
        return OwnerEdgeReadback::Unknown;
    }
    if a.voice != b.voice { return OwnerEdgeReadback::SplitVoice; }
    let mut pitched_ordinal = notes[..=left].iter()
        .filter(|note| !note.rest_like && note.pitch_midi.is_some()).count();
    for (index, note) in notes.iter().enumerate().take(right).skip(left + 1) {
        let ordinal = pitched_ordinal;
        if !note.rest_like && note.pitch_midi.is_some() { pitched_ordinal += 1; }
        if note.staff != a.staff || note.voice != a.voice { continue; }
        if note.rest_like {
            match actual_beam_levels(xml, note) {
                Some(levels) if levels.contains_key(&level) => return OwnerEdgeReadback::MarkedRest { note_index: index },
                Some(_) => continue,
                None => return OwnerEdgeReadback::Unknown,
            }
        }
        if note.chord_member {
            let Some(head) = note.anchor_index.filter(|head| *head < index).and_then(|head| notes.get(head)) else {
                return OwnerEdgeReadback::Unknown;
            };
            if !head.chord_member && !head.rest_like && head.pitch_midi.is_some()
                && head.staff == note.staff && head.voice == note.voice && head.onset == note.onset
            {
                continue;
            }
            return OwnerEdgeReadback::Unknown;
        }
        if note.pitch_midi.is_none() || note.duration <= 0 { return OwnerEdgeReadback::Unknown; }
        return OwnerEdgeReadback::Interrupted { pitched_ordinal: ordinal };
    }
    let (Some(left), Some(right)) = (actual_beam_levels(xml, a), actual_beam_levels(xml, b)) else {
        return OwnerEdgeReadback::Unknown;
    };
    match endpoint_readback(left.get(&level).map(String::as_str), right.get(&level).map(String::as_str)) {
        EdgeReadback::Present => OwnerEdgeReadback::Present,
        EdgeReadback::Missing => OwnerEdgeReadback::EndpointMarkersMissing,
        EdgeReadback::Unknown => OwnerEdgeReadback::Unknown,
    }
}

pub(super) fn append_owner_edge_comparison(
    out: &mut String, edges: &[OwnerEdge<'_>], before: &[OwnerEdgeReadback], after: &[OwnerEdgeReadback],
) {
    let (mut preserved, mut lost, mut unknown) = (0, 0, 0);
    for ((edge, before), after) in edges.iter().zip(before).zip(after) {
        if *before != OwnerEdgeReadback::Present { continue; }
        match after {
            OwnerEdgeReadback::Present => { preserved += 1; continue; }
            OwnerEdgeReadback::Unknown => unknown += 1,
            _ => lost += 1,
        }
        let _ = writeln!(out, "WrittenOnlyOwnerBeamEdgeWitness: level={}; group={:?}; left_source={}; left_attack={}; left_fragment={}; right_source={}; right_attack={}; right_fragment={}; before={before:?}; after={after:?}; retention=conditional; authority=none",
            edge.level, edge.group, edge.left.source, edge.left.attack, edge.left.fragment,
            edge.right.source, edge.right.attack, edge.right.fragment);
    }
    let not_present_before = edges.len() - preserved - lost - unknown;
    let _ = writeln!(out, "WrittenOnlyOwnerBeamEdges: candidates={}; baseline_present={}; preserved={preserved}; lost={lost}; unknown={unknown}; baseline_unproved={not_present_before}; optional_lifecycle=unassessed; downstream_render=unmeasured; authority=none",
        edges.len(), preserved + lost + unknown);
}

pub(super) fn append(
    out: &mut String,
    canonical: &CanonicalFinalChordStructuralPlan,
    capture: &ActualSourceBeamBindingObservation,
) {
    let (Ok(rows), Some(projection)) = (
        capture.graph_received_occurrences(canonical),
        canonical.written_projection.as_ref().and_then(Projection::bind),
    ) else {
        let _ = writeln!(out, "OrderedReceivedBeamReadback: result=Unknown; reason=unbound_capture_or_projection; authority=none");
        return;
    };
    let groups = owner_groups(rows);
    let locators = canonical.source_locators.iter()
        .map(|row| (row.fragment_id, row)).collect::<BTreeMap<_, _>>();
    let mut counts = [0; 4];
    for (&(level, group), members) in &groups {
        if !closed_group(members) {
            counts[3] += 1;
            continue;
        }
        for pair in members.windows(2) {
            let (left, right) = (pair[0].0, pair[1].0);
            let ordered = locators.get(&left.fragment).zip(locators.get(&right.fragment))
                .is_some_and(|(a, b)| a.measure == b.measure && a.staff == b.staff
                    && a.onset.checked_add(a.duration).is_some_and(|end| end <= b.onset));
            let readback = if ordered { projection.edge(level, left, right) } else { None }
                .unwrap_or(EdgeReadback::Unknown);
            match readback {
                EdgeReadback::Present => counts[0] += 1,
                EdgeReadback::Missing => counts[1] += 1,
                EdgeReadback::Unknown => counts[2] += 1,
            }
            if readback != EdgeReadback::Present {
                let _ = writeln!(out, "OrderedReceivedBeamReadbackWitness: level={level}; group={group:?}; left_fragment={}; right_fragment={}; result={readback:?}; authority=none",
                    left.fragment, right.fragment);
            }
        }
    }
    let [present, missing, unknown, unclosed_groups] = counts;
    let _ = writeln!(out, "OrderedReceivedBeamReadback: seam=FinalChordProjection; scope=surviving_owner_capture; groups={}; present={present}; missing={missing}; unknown={unknown}; unclosed_groups={unclosed_groups}; retention=conditional; bridge_authority=unassessed; authority=none", groups.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn another_head_interrupts_a_beam_even_when_its_endpoints_keep_their_labels() {
        let first = r#"<note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>one</voice><type>eighth</type><staff>1</staff><beam number="1">begin</beam></note>"#;
        let chord = r#"<note><chord/><pitch><step>E</step><octave>4</octave></pitch><duration>1</duration><voice>one</voice><type>eighth</type><staff>1</staff></note>"#;
        let middle = r#"<note><pitch><step>G</step><octave>4</octave></pitch><duration>1</duration><voice>other</voice><type>eighth</type><staff>1</staff></note>"#;
        let last = r#"<note><pitch><step>A</step><octave>4</octave></pitch><duration>1</duration><voice>one</voice><type>eighth</type><staff>1</staff><beam number="1">end</beam></note>"#;
        let xml = |measure, middle: &str| format!(r#"<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list><part id="P1"><measure number="{measure}"><attributes><divisions>2</divisions></attributes>{first}{chord}{middle}{last}</measure></part></score-partwise>"#);
        let read = |doc: &str| {
            let measures = final_chord_parse_binding_measures(doc).unwrap();
            read_projected_edge(doc, &measures[0].notes, 0, 3, 1)
        };
        for measure in [17, 83] {
            let before = xml(measure, middle);
            let after = xml(measure, &middle.replace(">other<", ">one<"));
            assert_eq!(read(&before), OwnerEdgeReadback::Present);
            assert_eq!(read(&after), OwnerEdgeReadback::Interrupted { pitched_ordinal: 2 });
        }
    }

    #[test]
    fn endpoint_direction_detects_a_break_and_keeps_hooks_unassessed() {
        assert_eq!(endpoint_readback(Some("begin"), Some("continue")), EdgeReadback::Present);
        assert_eq!(endpoint_readback(Some("end"), Some("begin")), EdgeReadback::Missing);
        assert_eq!(endpoint_readback(None, Some("end")), EdgeReadback::Missing);
        assert_eq!(endpoint_readback(Some("forward hook"), Some("end")), EdgeReadback::Unknown);
    }
}
