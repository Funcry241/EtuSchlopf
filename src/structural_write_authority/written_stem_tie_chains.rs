//! Bind printed tie chains to the current final-PDF source occurrence course.

use super::{
    FinalChordEncodingNote, FinalPdfSourceOccurrenceContinuation,
    final_chord_parse_binding_measures,
};
use anyhow::{Context, Result, ensure};
use roxmltree::{Document, Node};
use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroU32;
use std::ops::Range;

type Position = (usize, i64);

#[derive(Clone, Copy, Default)]
struct TieEnds {
    start: Option<NonZeroU32>,
    stop: Option<NonZeroU32>,
}

impl TieEnds {
    fn read<'a, 'input: 'a>(marks: impl Iterator<Item = Node<'a, 'input>>) -> Option<Self> {
        let mut ends = Self::default();
        for mark in marks {
            if mark.attribute("time-only").is_some() {
                return None;
            }
            let slot = match mark.attribute("type")? {
                "start" => &mut ends.start,
                "stop" => &mut ends.stop,
                _ => return None,
            };
            let number = mark.attribute("number").unwrap_or("1").parse().ok()?;
            if slot.replace(number).is_some() {
                return None;
            }
        }
        Some(ends)
    }

    fn from_note(note: Node<'_, '_>) -> Option<Self> {
        if note.attribute("print-object") == Some("no") {
            return None;
        }
        let sound = Self::read(note.children().filter(|node| node.has_tag_name("tie")))?;
        let print = Self::read(
            note.children()
                .filter(|node| node.has_tag_name("notations"))
                .flat_map(|node| node.children().filter(|child| child.has_tag_name("tied"))),
        )?;
        (sound.start.is_some() == print.start.is_some()
            && sound.stop.is_some() == print.stop.is_some())
        .then_some(print)
    }
}

struct WrittenNote<'a> {
    note: &'a FinalChordEncodingNote,
    start: Position,
    end: Position,
    pitch: i32,
    ties: Option<TieEnds>,
}

impl FinalPdfSourceOccurrenceContinuation {
    /// This projection has no write authority. Ranges refer to this continuation's
    /// exact XML. Their membership comes from owned Source/Attack/Fragment rows.
    pub(crate) fn authenticated_tie_stem_chains(&self) -> Result<Vec<Vec<Range<usize>>>> {
        let document = Document::parse(self.xml())?;
        let parts = document
            .root_element()
            .children()
            .filter(|node| node.has_tag_name("part"));
        if parts.count() != 1 {
            return Ok(Vec::new());
        }
        let nodes: BTreeMap<_, _> = document
            .descendants()
            .filter(|node| node.has_tag_name("note"))
            .map(|node| (node.range().start, node))
            .collect();
        let measures = final_chord_parse_binding_measures(self.xml())?;
        let mut notes = BTreeMap::new();
        let mut attacks = BTreeMap::new();
        let mut releases = BTreeMap::new();
        for (index, measure) in measures.iter().enumerate() {
            let pitched = measure
                .notes
                .iter()
                .filter(|note| !note.rest_like && note.pitch_midi.is_some());
            for (ordinal, note) in pitched.enumerate() {
                let pitch = note
                    .pitch_midi
                    .context("written tie occurrence lost pitched identity")?;
                let end_tick = note
                    .onset
                    .checked_add(note.duration)
                    .context("written tie timing overflow")?;
                let start = (index, note.onset);
                let end = if end_tick == measure.expected_duration {
                    (index + 1, 0)
                } else {
                    (index, end_tick)
                };
                let node = nodes
                    .get(&note.range.start)
                    .context("written tie occurrence lacks its note element")?;
                ensure!(
                    node.range() == note.range,
                    "written tie note range differs from its bound element"
                );
                let ties = if note.onset >= 0
                    && note.duration > 0
                    && end_tick <= measure.expected_duration
                {
                    TieEnds::from_note(*node)
                } else {
                    None
                };
                let mark = WrittenNote {
                    note,
                    start,
                    end,
                    pitch,
                    ties,
                };
                ensure!(
                    notes.insert((measure.measure, ordinal), mark).is_none(),
                    "written tie occurrence slot is ambiguous"
                );
                // Include every sounding competitor. The renderer could choose a
                // simultaneous unison even if its own printed tie is unsupported.
                *attacks.entry((start, pitch)).or_insert(0usize) += 1;
                *releases.entry((end, pitch)).or_insert(0usize) += 1;
            }
        }
        let mut claims = BTreeSet::new();
        let mut fragments = BTreeSet::new();
        let mut by_attack = BTreeMap::<_, Vec<_>>::new();
        for row in self.current_occurrences() {
            let slot = (row.measure, row.pitched_ordinal);
            ensure!(
                claims.insert(slot) && fragments.insert((row.fragment_id, row.fragment_ordinal)),
                "written tie occurrence provenance is ambiguous"
            );
            let note = notes
                .get(&slot)
                .context("written tie occurrence lacks its current owner slot")?;
            by_attack
                .entry((row.source_event_id, row.attack_id))
                .or_default()
                .push(note);
        }
        let mut chains = Vec::new();
        for (_, mut chain) in by_attack {
            if chain.len() < 2 {
                continue;
            }
            chain.sort_by_key(|note| note.start);
            let first = chain[0];
            let last = chain[chain.len() - 1];
            if !first.ties.is_some_and(|ties| ties.stop.is_none())
                || !last.ties.is_some_and(|ties| ties.start.is_none())
                || !chain.iter().all(|mark| {
                    mark.note.staff == first.note.staff
                        && mark.note.voice == first.note.voice
                        && mark.note.pitch_identity == first.note.pitch_identity
                })
                || !chain.windows(2).all(|pair| {
                    let left = pair[0];
                    let right = pair[1];
                    let outgoing = left.ties.and_then(|ties| ties.start);
                    let incoming = right.ties.and_then(|ties| ties.stop);
                    left.end == right.start
                        && outgoing.is_some()
                        && outgoing == incoming
                        && releases.get(&(left.end, left.pitch)) == Some(&1)
                        && attacks.get(&(right.start, right.pitch)) == Some(&1)
                })
            {
                continue;
            }
            chains.push(
                chain
                    .iter()
                    .map(|mark| mark.note.range.clone())
                    .collect::<Vec<_>>(),
            );
        }
        chains.sort_by_key(|chain| chain[0].start);
        Ok(chains)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        FinalChordProjectionMaterializationKind, FinalChordTransactionLocalOutputAlphaReceipt,
        FinalChordTransactionOutputOccurrenceWitness,
    };
    use super::*;

    fn written_score([first, last]: [usize; 2], tail: &str) -> String {
        format!(
            r#"<score-partwise version="4.0"><part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
<part id="P1"><measure number="{first}"><attributes><divisions>1</divisions><time><beats>1</beats><beat-type>4</beat-type></time></attributes>
<note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><tie type="start"/><voice>1</voice><type>quarter</type><staff>1</staff><notations><tied type="start"/></notations></note></measure>
<measure number="{last}">{tail}</measure></part></score-partwise>"#
        )
    }

    const TIE_STOP: &str = r#"<note><pitch><step>C</step><octave>5</octave></pitch><duration>1</duration><tie type="stop"/><voice>1</voice><type>quarter</type><staff>1</staff><notations><tied type="stop"/></notations></note>"#;

    fn observed(
        labels: [usize; 2],
        tail: &str,
        same_attack: bool,
    ) -> FinalPdfSourceOccurrenceContinuation {
        let occurrences = labels
            .into_iter()
            .enumerate()
            .map(
                |(fragment_id, measure)| FinalChordTransactionOutputOccurrenceWitness {
                    fragment_id,
                    source_event_id: if same_attack { 44 } else { 44 + fragment_id },
                    attack_id: if same_attack { 91 } else { 91 + fragment_id },
                    materialization_kind:
                        FinalChordProjectionMaterializationKind::SurvivingSourceNote,
                    fragment_ordinal: 0,
                    measure,
                    pitched_ordinal: 0,
                },
            )
            .collect();
        FinalPdfSourceOccurrenceContinuation::from_original_transport(
            written_score(labels, tail),
            FinalChordTransactionLocalOutputAlphaReceipt {
                rest_bridge_extension_course: None,
                phrase_origin: Default::default(),
                canonical_semantic_fingerprint_fnv64: 0,
                bound_source_fingerprint_fnv64: 0,
                bound_source_len: 0,
                output_topology_fingerprint_fnv64: 0,
                occurrences,
                certified_omitted_fragment_ids: Default::default(),
                independent_rest_anchor_fragments: Default::default(),
                original_beam_voice_groups: Vec::new(),
            },
        )
        .unwrap()
    }

    #[test]
    fn sealed_attack_and_written_tie_bind_exact_ranges_across_printed_measure_labels() {
        for labels in [[7, 12], [41, 53]] {
            let matching = observed(labels, TIE_STOP, true);
            let chains = matching.authenticated_tie_stem_chains().unwrap();
            assert_eq!((chains.len(), chains[0].len()), (1, 2));
            assert!(matching.xml()[chains[0][0].clone()].contains("<tied type=\"start\"/>"));
            assert_eq!(&matching.xml()[chains[0][1].clone()], TIE_STOP);
        }

        let foreign = observed([7, 12], TIE_STOP, false);
        assert!(foreign.authenticated_tie_stem_chains().unwrap().is_empty());
    }

    #[test]
    fn shared_attack_cannot_certify_cross_staff_missing_or_ambiguous_written_ends() {
        let cross_staff = TIE_STOP.replace("<staff>1</staff>", "<staff>2</staff>");
        let missing = TIE_STOP.replace(r#"<tied type="stop"/>"#, "");
        let wrong_number = TIE_STOP.replace(
            r#"<tied type="stop"/>"#,
            r#"<tied type="stop" number="2"/>"#,
        );
        let duplicate = format!("{TIE_STOP}<backup><duration>1</duration></backup>{TIE_STOP}");
        for tail in [cross_staff, missing, wrong_number, duplicate] {
            let observation = observed([7, 12], &tail, true);
            assert!(
                observation
                    .authenticated_tie_stem_chains()
                    .unwrap()
                    .is_empty()
            );
        }
    }
}
