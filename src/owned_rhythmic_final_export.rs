//! Representation of the same received obligation after original owner #65.
//! A forward retains the rest obligation; it is not evidence of its fulfilment.
use super::*;
use crate::master_state_identity::FinalChordCourseCapture;
#[path = "owned_rhythmic_origin_image.rs"]
mod origin_image;
use origin_image::OriginImage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EncodingExecution {
    FinalChord(crate::master_state_identity::MintedIdentity),
    VisibleCopy(crate::master_state_identity::MintedIdentity),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct OriginalEmission {
    execution: EncodingExecution,
    input: (usize, usize),
    ordinal: usize,
}

#[derive(Debug, Clone)]
struct EncodingFragment {
    bytes: Range<usize>,
    // Local slot in the already authenticated emission traversal, never genesis.
    // The path below retains each original owner's actual ordinal separately.
    ordinal: usize,
    original_emissions: Vec<OriginalEmission>,
    interval: Range<i64>,
    correspondence: OriginImage,
    forward: bool,
    rest: bool,
    chord: bool,
    ties: Vec<String>,
}

#[derive(Debug, Clone)]
pub(super) struct EncodedOccurrence {
    pub(super) received: Occurrence,
    fragments: Vec<EncodingFragment>,
    emission_count: usize,
    original_emissions: Vec<Vec<OriginalEmission>>,
}

impl EncodedOccurrence {
    pub(super) fn complete_ranges(&self) -> impl Iterator<Item = &Range<usize>> {
        self.fragments.iter().map(|fragment| &fragment.bytes)
    }

    pub(super) fn note_ranges(&self) -> impl Iterator<Item = &Range<usize>> {
        self.fragments.iter().filter(|fragment| !fragment.forward).map(|fragment| &fragment.bytes)
    }
}

type ForwardReadback = BTreeMap<(usize, usize), (Range<i64>, Option<u8>)>;

fn forwards(xml: &str) -> Result<ForwardReadback> {
    let mut result = BTreeMap::new();
    let mut offset = 0_i64;
    let mut context = RhythmContext::default();
    for span in find_measure_spans(xml) {
        let measure = &xml[span.start..span.end];
        context = context.update_from_measure(measure);
        let doc = roxmltree::Document::parse(measure)?;
        let mut cursor = 0_i64;
        for node in doc.root_element().children().filter(roxmltree::Node::is_element) {
            let text = &measure[node.range()];
            let duration = extract_first_i64(text, "duration").unwrap_or(0);
            match node.tag_name().name() {
                "backup" => cursor = cursor.checked_sub(duration).ok_or_else(|| anyhow!("Cursor overflow"))?,
                "forward" => {
                    let end = cursor.checked_add(duration).ok_or_else(|| anyhow!("Cursor overflow"))?;
                    let staff = extract_first_i64(text, "staff").and_then(|staff| u8::try_from(staff).ok());
                    result.insert((span.start + node.range().start, span.start + node.range().end),
                        (offset + cursor..offset + end, staff));
                    cursor = end;
                },
                "note" if !text.contains("<chord") => {
                    cursor = cursor.checked_add(duration).ok_or_else(|| anyhow!("Cursor overflow"))?;
                },
                _ => {},
            }
        }
        offset = offset.checked_add(context.expected_duration()).ok_or_else(|| anyhow!("Domain overflow"))?;
    }
    Ok(result)
}

fn exact_partition(mut spans: Vec<Range<i64>>, original: &Range<i64>) -> Result<()> {
    spans.sort_by_key(|range| (range.start, range.end));
    let mut end = original.start;
    for span in spans {
        ensure!(span.start == end && span.end > span.start, "Encoding lost or duplicated origin support");
        end = span.end;
    }
    ensure!(end == original.end, "Encoding extended or lost origin support");
    Ok(())
}

impl OwnedRhythmicContinuation {
    pub(crate) fn validate_original_respelling_sources(&self,
        sources: &[(Range<usize>, usize)]) -> Result<()> {
        self.validate_binding(&self.snapshot)?;
        let expected = self.occurrences.iter().filter(|old|
            self.snapshot[old.bytes.clone()].contains("<pitch>"))
            .map(|old| Ok(((old.bytes.start, old.bytes.end), old.origin.source
                .ok_or_else(|| anyhow!("Original pitched occurrence has no source"))?)))
            .collect::<Result<BTreeMap<_, _>>>()?;
        let mut expected = expected;
        for encoded in &self.encoded {
            for bytes in encoded.note_ranges() {
                if self.snapshot[bytes.clone()].contains("<pitch>") {
                    ensure!(expected.insert((bytes.start, bytes.end), encoded.received.origin.source
                        .ok_or_else(|| anyhow!("Encoded pitched occurrence has no original source"))?).is_none(),
                        "Ambiguous original pitched carrier");
                }
            }
        }
        let actual = sources.iter().map(|(range, source)|
            ((range.start, range.end), *source)).collect::<BTreeMap<_, _>>();
        ensure!(self.final_chord_execution.is_none() && actual.len() == sources.len() && actual == expected,
            "Original canonical binding does not cover the received sources");
        Ok(())
    }

    /// Source/course provenance defines membership; support and MIDI only
    /// verify unique original tie edges within that already bound membership.
    pub(crate) fn original_respelling_components(&self,
        requested: impl Fn(&Range<usize>) -> Result<bool>) -> Result<Vec<Vec<Range<usize>>>> {
        self.validate_binding(&self.snapshot)?;
        let inventory = physical_inventory(&self.snapshot)?.into_iter().map(|note|
            ((note.bytes.start, note.bytes.end), note)).collect::<BTreeMap<_, _>>();
        let mut notes = Vec::new();
        for old in &self.occurrences { notes.push((&old.origin, old.bytes.clone())); }
        for encoded in &self.encoded {
            for bytes in encoded.note_ranges() { notes.push((&encoded.received.origin, bytes.clone())); }
        }
        let mut groups = BTreeMap::<_, Vec<_>>::new();
        for (origin, bytes) in notes {
            let note = inventory.get(&(bytes.start, bytes.end)).ok_or_else(|| anyhow!("Missing original chain note"))?;
            if !note.text.contains("<pitch>") { continue; }
            ensure!(origin.source.is_some() && origin.course.is_some(), "Original chain lacks source/course provenance");
            groups.entry((origin.source, origin.course)).or_default().push(note);
        }
        let mut components = Vec::new();
        for group in groups.values() {
            let mut visited = BTreeSet::new();
            for root in 0..group.len() {
                if visited.contains(&root) || !requested(&group[root].bytes)? { continue; }
                let mut pending = vec![root];
                let mut component = Vec::new();
                while let Some(index) = pending.pop() {
                    if !visited.insert(index) { continue; }
                    let note = group[index];
                    component.push(note.bytes.clone());
                    let marks = marks_named(&note.text, &["tie"]);
                    for (kind, incoming) in [("start", false), ("stop", true)] {
                        if !marks.contains(&format!("tie:Some(\"{kind}\")")) { continue; }
                        let peers = group.iter().enumerate().filter(|(_, peer)| {
                            let expected = if incoming { "start" } else { "stop" };
                            marks_named(&peer.text, &["tie"]).contains(&format!("tie:Some(\"{expected}\")"))
                                && (if incoming { peer.interval.end == note.interval.start }
                                    else { note.interval.end == peer.interval.start })
                                && sounding_midi(&peer.text) == sounding_midi(&note.text)
                        }).map(|(peer, _)| peer).collect::<Vec<_>>();
                        ensure!(peers.len() == 1, "Original chain lacks one unique complete tie edge");
                        pending.push(peers[0]);
                    }
                }
                components.push(component);
            }
        }
        Ok(components)
    }

    pub(super) fn validate_original_respelling_chains(&self,
        receipt: &ExactTransformationReceipt) -> Result<()> {
        ensure!(receipt.minted_by::<crate::pipeline::FunctionalRespellingExactMint>(),
            "Respelling lacks its original owner");
        let readback = receipt.operation_readback(&self.snapshot)?;
        let after = readback.post_document();
        let inventory = physical_inventory(&self.snapshot)?.into_iter().map(|note|
            ((note.bytes.start, note.bytes.end), note)).collect::<BTreeMap<_, _>>();
        let mut notes = Vec::new();
        for old in &self.occurrences {
            notes.push((&old.origin, old.bytes.clone()));
        }
        for encoded in &self.encoded {
            for bytes in encoded.note_ranges() { notes.push((&encoded.received.origin, bytes.clone())); }
        }
        let mut groups = BTreeMap::<_, Vec<_>>::new();
        let mut changed = BTreeSet::new();
        for (origin, bytes) in notes {
            let before = inventory.get(&(bytes.start, bytes.end)).ok_or_else(|| anyhow!("Lost respelling input"))?;
            if !before.text.contains("<pitch>") { continue; }
            let carried = readback.carry_range(bytes)?;
            let post = after.get(carried).ok_or_else(|| anyhow!("Respelling escaped complete note"))?;
            ensure!(same_enharmonic_disposition(&before.text, post), "Respelling changed sounding disposition");
            let key = (origin.source, origin.course);
            if extract_first_text(&before.text, "pitch") != extract_first_text(post, "pitch") {
                ensure!(origin.source.is_some() && origin.course.is_some(), "Respelling lacks original source/course");
                changed.insert(key);
            }
            groups.entry(key).or_default().push((before, post));
        }
        for key in changed {
            let group = &groups[&key];
            for (before, post) in group {
                let marks = marks_named(&before.text, &["tie"]);
                for (kind, incoming) in [("start", false), ("stop", true)] {
                    if !marks.contains(&format!("tie:Some(\"{kind}\")")) { continue; }
                    // Existing source/course provenance establishes the group;
                    // support and MIDI verify its unique original edge only.
                    let peers = group.iter().filter(|(peer, _)| {
                        let expected = if incoming { "start" } else { "stop" };
                        marks_named(&peer.text, &["tie"]).contains(&format!("tie:Some(\"{expected}\")"))
                            && (if incoming { peer.interval.end == before.interval.start }
                                else { before.interval.end == peer.interval.start })
                            && sounding_midi(&peer.text) == sounding_midi(&before.text)
                    }).collect::<Vec<_>>();
                    ensure!(peers.len() == 1, "Respelling lacks one complete original tie chain");
                    ensure!(extract_first_text(post, "pitch") == extract_first_text(peers[0].1, "pitch"),
                        "Respelling changed only part of an original tie chain");
                }
            }
        }
        Ok(())
    }

    pub(super) fn retain_original_visible_copy(next: &mut Self, old: &Occurrence,
        note: &PhysicalOccurrence, original_emission: crate::master_state_identity::MintedIdentity) -> Result<()> {
        let correspondence = OriginImage::new(old.origin.interval.clone(), note.interval.clone())?;
        let provenance = vec![OriginalEmission { execution: EncodingExecution::VisibleCopy(original_emission),
            input: (old.bytes.start, old.bytes.end), ordinal: 0 }];
        next.encoded.push(EncodedOccurrence { received: old.clone(), emission_count: 1,
            original_emissions: vec![provenance.clone()],
            fragments: vec![EncodingFragment { bytes: note.bytes.clone(), ordinal: 0,
                original_emissions: provenance, interval: note.interval.clone(), correspondence,
                forward: false, rest: false, chord: true, ties: tie_marks(&note.text) }] });
        Ok(())
    }

    pub(super) fn continue_final_chord(&self, receipt: &ExactTransformationReceipt,
        capture: &FinalChordCourseCapture) -> Result<Self> {
        self.validate_binding(&self.snapshot)?;
        ensure!(self.final_chord_execution.is_none(), "FinalChord applied twice to an encoded obligation");
        let after = capture.validate_execution(receipt, &self.snapshot)?;
        let physical = physical_inventory(after)?;
        let before_physical = physical_inventory(&self.snapshot)?.into_iter().map(|note|
            ((note.bytes.start, note.bytes.end), note)).collect::<BTreeMap<_, _>>();
        let mut output = physical.iter().map(|note| ((note.bytes.start, note.bytes.end), note))
            .collect::<BTreeMap<_, _>>();
        let mut cursor_output = forwards(after)?;
        let mut rows = capture.rows().iter().map(|row| ((row.input().start, row.input().end), row))
            .collect::<BTreeMap<_, _>>();
        ensure!(rows.len() == capture.rows().len() && rows.len() == before_physical.len(),
            "Incomplete original FinalChord capture");
        let mut next = self.clone();
        next.occurrences.clear(); next.encoded.clear();
        let mut roots = Vec::new();
        for old in &self.occurrences {
            roots.push((old, vec![EncodingFragment { bytes: old.bytes.clone(), ordinal: 0,
                original_emissions: Vec::new(), interval: old.origin.interval.clone(),
                correspondence: OriginImage::new(old.origin.interval.clone(), old.origin.interval.clone())?,
                forward: false, rest: self.snapshot[old.bytes.clone()].contains("<rest"),
                chord: self.snapshot[old.bytes.clone()].contains("<chord"),
                ties: tie_marks(&self.snapshot[old.bytes.clone()]) }]));
        }
        for encoded in &self.encoded { roots.push((&encoded.received, encoded.fragments.clone())); }
        let readback = receipt.operation_readback(&self.snapshot)?;
        for (old, inputs) in roots {
            let mut fragments = Vec::new();
            for input in inputs {
                if input.forward {
                    // #65 need not emit a pre-existing cursor. Its actual edit
                    // script must carry this same full original carrier.
                    let bytes = readback.carry_range(input.bytes.clone())?;
                    ensure!(self.snapshot.get(input.bytes.clone()) == after.get(bytes.clone()),
                        "FinalChord changed an existing original cursor");
                    let mut carried = input; carried.bytes = bytes; carried.ordinal = fragments.len();
                    fragments.push(carried); continue;
                }
                let row = rows.remove(&(input.bytes.start, input.bytes.end))
                    .ok_or_else(|| anyhow!("Missing original FinalChord occurrence"))?;
                let source = before_physical.get(&(input.bytes.start, input.bytes.end))
                    .ok_or_else(|| anyhow!("Missing original input carrier"))?;
                if source.text.contains("<pitch>") {
                    ensure!(row.source().is_some() && row.source() == old.origin.source,
                        "Canonical source is not the received source");
                } else { ensure!(row.source().is_none() && source.text.contains("<rest"), "Invented rest source"); }
                let emitted = read_original_final_chord_row(receipt, row, source, &mut output, &mut cursor_output)?;
                for mut fragment in emitted {
                    if let Some(composed) = input.correspondence.compose(&fragment.correspondence)? {
                        let mut ancestry = input.original_emissions.clone();
                        ancestry.append(&mut fragment.original_emissions);
                        fragment.original_emissions = ancestry;
                        fragment.correspondence = composed;
                        fragment.ordinal = fragments.len();
                        fragments.push(fragment);
                    }
                }
            }
            let original_emissions = fragments.iter().map(|fragment| fragment.original_emissions.clone()).collect();
            next.encoded.push(EncodedOccurrence { received: old.clone(), emission_count: fragments.len(),
                original_emissions, fragments });
        }
        ensure!(rows.is_empty() && output.is_empty(), "Unexplained FinalChord note or source");
        next.final_chord_execution = Some(*receipt.ssc_bound_origin().0);
        next.snapshot = Arc::from(after);
        next.validate_binding(after)?;
        Ok(next)
    }

    pub(crate) fn nonphysical_rest_obligation_count(&self) -> usize {
        self.encoded.iter().filter(|encoded| encoded.fragments.iter().any(|fragment| fragment.forward)).count()
    }
    pub(crate) fn reparameterized_obligation_count(&self) -> usize {
        self.encoded.iter().filter(|encoded| encoded.fragments.iter().any(|fragment| fragment.correspondence.origin != fragment.correspondence.image)).count()
    }

    pub(super) fn encoded_note_count(&self) -> usize {
        self.encoded.iter().flat_map(|encoded| &encoded.fragments).filter(|fragment| !fragment.forward).count()
    }

    pub(super) fn validate_encoded(&self, xml: &str, used: &mut BTreeSet<(usize, usize)>,
        identities: &mut BTreeSet<RhythmicCreationIdentity>) -> Result<()> {
        let physical = physical_inventory(xml)?.into_iter().map(|note|
            ((note.bytes.start, note.bytes.end), note)).collect::<BTreeMap<_, _>>();
        let cursors = forwards(xml)?;
        let mut emission_identities = BTreeSet::new();
        for encoded in &self.encoded {
            let old = &encoded.received;
            ensure!(encoded.emission_count == encoded.fragments.len()
                && encoded.fragments.iter().map(|fragment| fragment.ordinal).collect::<BTreeSet<_>>() ==
                    (0..encoded.emission_count).collect(), "Original emission ordinal drift");
            ensure!(encoded.original_emissions == encoded.fragments.iter()
                .map(|fragment| fragment.original_emissions.clone()).collect::<Vec<_>>()
                && encoded.fragments.iter().all(|fragment| !fragment.original_emissions.is_empty()
                    && emission_identities.insert((old.creation, fragment.original_emissions.clone()))),
                "Original emission execution reused or changed");
            ensure!(identities.insert(old.creation) && self.history.get(&old.creation) == Some(&old.origin),
                "Encoded occurrence lost or reused its original ancestry");
            self.validate_received_origin_path(old.creation)?;
            for emission in encoded.original_emissions.iter().flatten() {
                if let EncodingExecution::FinalChord(execution) = emission.execution {
                    ensure!(self.final_chord_execution == Some(execution), "FinalChord execution epoch was reset or substituted");
                }
            }
            let spans = encoded.fragments.iter().map(|fragment| {
                OriginImage::new(fragment.correspondence.origin.clone(), fragment.correspondence.image.clone())?;
                ensure!(fragment.correspondence.image == fragment.interval, "Partial carrier lacks certified partition");
                Ok(fragment.correspondence.origin.clone())
            }).collect::<Result<Vec<_>>>()?;
            exact_partition(spans, &old.origin.interval)?;
            for fragment in &encoded.fragments {
                ensure!(used.insert((fragment.bytes.start, fragment.bytes.end)), "Duplicate encoded output");
                if fragment.forward {
                    ensure!(cursors.get(&(fragment.bytes.start, fragment.bytes.end)) ==
                        Some(&(fragment.interval.clone(), Some(old.staff))), "Rest obligation cursor drift");
                } else {
                    let note = physical.get(&(fragment.bytes.start, fragment.bytes.end))
                        .ok_or_else(|| anyhow!("Encoded note is no longer an entire note"))?;
                    ensure!(note.interval == fragment.interval && note.staff == old.staff
                        && note.text.contains("<rest") == fragment.rest
                        && note.text.contains("<chord") == fragment.chord
                        && tie_marks(&note.text) == fragment.ties, "Encoded support or disposition drift");
                }
            }
        }
        Ok(())
    }

    pub(super) fn carry_encoded(&self, next: &mut Self, after: &str,
        carry: &impl Fn(Range<usize>) -> Result<Range<usize>>, respelling: bool) -> Result<()> {
        for encoded in &mut next.encoded {
            for fragment in &mut encoded.fragments {
                let bytes = carry(fragment.bytes.clone())?;
                let before_text = &self.snapshot[fragment.bytes.clone()];
                let after_text = after.get(bytes.clone()).ok_or_else(|| anyhow!("Encoded carry escaped document"))?;
                ensure!(if fragment.forward { before_text == after_text } else {
                    same_pitch_and_disposition(before_text, after_text)
                        || (respelling && super::same_enharmonic_disposition(before_text, after_text))
                }, "Encoded occurrence changed without its original transition");
                fragment.bytes = bytes;
            }
        }
        Ok(())
    }
}

/// Validate a complete original row before composing it with any original
/// root pieces. All source membership comes from the sealed original capture.
fn read_original_final_chord_row(
    receipt: &ExactTransformationReceipt,
    row: &crate::master_state_identity::FinalChordCourseRow,
    source: &PhysicalOccurrence,
    output: &mut BTreeMap<(usize, usize), &PhysicalOccurrence>,
    cursors: &mut ForwardReadback,
) -> Result<Vec<EncodingFragment>> {
    let old_text = &source.text;
    let mut fragments = Vec::new();
    ensure!(row.outputs().len() == row.emission_ordinals().len(), "Missing original emission ordinals");
    ensure!(row.emission_ordinals().iter().copied().collect::<BTreeSet<_>>() ==
        (0..row.outputs().len()).collect(), "Original emission ordinal coverage drift");
    for ((bytes, is_forward), ordinal) in row.outputs().iter().zip(row.emission_ordinals()) {
        let (interval, rest, chord, ties) = if *is_forward {
            ensure!(old_text.contains("<rest") && !row.tied_chain() && !row.auxiliary(),
                "A pitched obligation became a cursor");
            let (interval, staff) = cursors.remove(&(bytes.start, bytes.end))
                .ok_or_else(|| anyhow!("Original rest cursor is not an entire forward"))?;
            ensure!(staff == Some(source.staff), "Rest cursor changed original presentation");
            (interval, true, false, Vec::new())
        } else {
            let note = output.remove(&(bytes.start, bytes.end))
                .ok_or_else(|| anyhow!("FinalChord emission is not one unique full note"))?;
            ensure!(note.staff == source.staff
                && extract_first_text(old_text, "pitch") == extract_first_text(&note.text, "pitch")
                && old_text.contains("<rest") == note.text.contains("<rest")
                && old_text.contains("<grace") == note.text.contains("<grace"),
                "FinalChord changed source pitch or presentation");
            if !row.tied_chain() {
                ensure!(tie_marks(old_text) == tie_marks(&note.text), "FinalChord changed outer ties");
                ensure!(old_text.contains("<chord") == note.text.contains("<chord")
                    || (row.auxiliary() && old_text.contains("<chord") && !note.text.contains("<chord")),
                    "Chord encoding lacks its original canonical decision");
            }
            (note.interval.clone(), note.text.contains("<rest"), note.text.contains("<chord"), tie_marks(&note.text))
        };
        fragments.push(EncodingFragment { bytes: bytes.clone(), ordinal: *ordinal,
            original_emissions: vec![OriginalEmission { execution: EncodingExecution::FinalChord(*receipt.ssc_bound_origin().0),
                input: (row.input().start, row.input().end), ordinal: *ordinal }],
            correspondence: OriginImage::translated(interval.clone(), row.original_offset())?, interval,
            forward: *is_forward, rest, chord, ties });
    }
    exact_partition(fragments.iter().map(|fragment| fragment.correspondence.origin.clone()).collect(), &source.interval)?;
    if row.tied_chain() {
        ensure!(old_text.contains("<pitch>") && !old_text.contains("<grace"), "Invalid tied source");
        fragments.sort_by_key(|fragment| fragment.interval.start);
        for (index, fragment) in fragments.iter().enumerate() {
            let mut expected = Vec::new();
            let outer = tie_marks(old_text);
            for name in ["tie", "tied"] {
                if index > 0 || outer.contains(&format!("{name}:Some(\"stop\")")) {
                    expected.push(format!("{name}:Some(\"stop\")"));
                }
                if index + 1 < fragments.len() || outer.contains(&format!("{name}:Some(\"start\")")) {
                    expected.push(format!("{name}:Some(\"start\")"));
                }
            }
            let mut actual = fragment.ties.clone(); expected.sort(); actual.sort();
            ensure!(!fragment.forward && fragment.chord == old_text.contains("<chord")
                && actual == expected, "Tied encoding lost an inner or outer endpoint");
        }
    }
    Ok(fragments)
}

#[cfg(test)]
#[path = "../tests/unit/final_export_course_continuation.rs"]
mod tests;
