// Actual E053 beam decisions, carried from their emitter, never reconstructed
// from a later XML voice/pitch/time match. Read-only; no allocation permit.
#[derive(Debug, Clone, Default)]
struct ActualBeamNoteMap(Vec<(std::ops::Range<usize>, std::ops::Range<usize>)>);
impl ActualBeamNoteMap {
    fn unchanged(notes: &[PartialBeatRawNote]) -> Self {
        Self(
            notes
                .iter()
                .map(|n| (n.start..n.end, n.start..n.end))
                .collect(),
        )
    }
    fn lift(&mut self, source: usize, output: usize) {
        for (a, b) in &mut self.0 {
            *a = source + a.start..source + a.end;
            *b = output + b.start..output + b.end;
        }
    }
    fn carry(&self, range: &std::ops::Range<usize>) -> Option<std::ops::Range<usize>> {
        let mut found = self.0.iter().filter(|(a, _)| a == range);
        let result = found.next()?.1.clone();
        found.next().is_none().then_some(result)
    }
    fn then(&self, next: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter_map(|(a, b)| next.carry(b).map(|c| (a.clone(), c)))
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
struct ActualBeamEmission {
    // Source byte starts supplied by the actual group planner, resolved only
    // against the complete-note ranges of that same emitter execution.
    members: Vec<usize>,
    owned_levels: u8,
    edits: Vec<(usize, u8, String)>,
}
impl ActualBeamEmission {
    fn planned(
        group: &[&PartialBeatNoteInfo],
        plan: &structural_write_authority::BeamPlan,
    ) -> Self {
        Self {
            members: group.iter().map(|n| n.index).collect(),
            owned_levels: 4,
            edits: plan
                .edits()
                .iter()
                .map(|e| {
                    (
                        e.event_ordinal(),
                        e.level(),
                        e.state().as_musicxml_text().into(),
                    )
                })
                .collect(),
        }
    }
    fn negative(index: usize) -> Self {
        Self {
            members: vec![index],
            owned_levels: 4,
            edits: Vec::new(),
        }
    }
    fn lift(&mut self, offset: usize) {
        for member in &mut self.members {
            *member += offset;
        }
    }
}

pub(crate) type ActualBeamLevels = Vec<(u8, Option<(usize, usize)>, Option<String>)>;

// Compare the actual XML tree while retaining meaningful leaf text (including
// whitespace in lyrics). Formatting between elements is not musical content.
fn actual_beam_note_signature(text: &str, omit_beams: bool) -> anyhow::Result<Vec<String>> {
    fn visit(node: roxmltree::Node<'_, '_>, omit: bool, out: &mut Vec<String>) {
        if node.is_element() {
            if omit && node.has_tag_name("beam") {
                return;
            }
            out.push(format!(
                "open:{:?}:{:?}",
                node.tag_name().namespace(),
                node.tag_name().name()
            ));
            let mut attrs = node
                .attributes()
                .map(|a| (a.namespace(), a.name(), a.value()))
                .collect::<Vec<_>>();
            attrs.sort_unstable();
            out.push(format!("attrs:{attrs:?}"));
            for child in node.children() {
                visit(child, omit, out);
            }
            out.push("close".into());
        } else if node.is_text() {
            let value = node.text().unwrap_or_default();
            let layout_space = value.trim().is_empty()
                && node
                    .parent()
                    .is_some_and(|p| p.children().any(|c| c.is_element()));
            if !layout_space {
                out.push(format!("text:{value:?}"));
            }
        }
    }
    let doc = roxmltree::Document::parse(text)?;
    anyhow::ensure!(
        doc.root_element().has_tag_name("note"),
        "Not a complete note carrier"
    );
    let mut output = Vec::new();
    visit(doc.root_element(), omit_beams, &mut output);
    Ok(output)
}

fn actual_beam_written_states(
    note: roxmltree::Node<'_, '_>,
) -> anyhow::Result<std::collections::BTreeMap<u8, String>> {
    let mut result = std::collections::BTreeMap::new();
    for beam in note.children().filter(|n| n.has_tag_name("beam")) {
        let level = beam.attribute("number").unwrap_or("1").parse::<u8>()?;
        anyhow::ensure!(
            result
                .insert(level, beam.text().unwrap_or_default().trim().to_owned())
                .is_none(),
            "Duplicate written beam level"
        );
    }
    Ok(result)
}
#[derive(Debug, Clone)]
pub(crate) struct ActualCanonicalBeamCapture {
    execution: crate::master_state_identity::MintedIdentity,
    before: std::sync::Arc<str>,
    after: std::sync::Arc<str>,
    current: std::sync::Arc<str>,
    occurrences: Vec<(std::ops::Range<usize>, ActualBeamLevels)>,
    chord_attachments: Result<Vec<ActualBeamChordAttachment>, String>,
    // E053 emitter journal: every complete note, beam stage then rest stage.
    course_carry: std::sync::Arc<Vec<(std::ops::Range<usize>, std::ops::Range<usize>)>>,
}
impl ActualCanonicalBeamCapture {
    fn at_owner(
        receipt: &BlockTransformationReceipt,
        before: &str,
        after: &str,
        beam_map: &ActualBeamNoteMap,
        rest_map: &ActualBeamNoteMap,
        emissions: &[ActualBeamEmission],
    ) -> anyhow::Result<Self> {
        receipt.validate_block_transition::<BeatScopedBeamingBlockMint>(before, after)?;
        let mut bound = std::collections::BTreeMap::<(usize, usize), ActualBeamLevels>::new();
        for (decision, emission) in emissions.iter().enumerate() {
            anyhow::ensure!(
                !emission.members.is_empty() && (1..=4).contains(&emission.owned_levels),
                "Incomplete beam owner scope"
            );
            let mut ranges = Vec::new();
            let mut written = Vec::new();
            for &start in &emission.members {
                let mut candidates = beam_map.0.iter().filter(|(r, _)| r.start == start);
                let (input, beam_output) = candidates
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Missing actual beam note emission"))?;
                anyhow::ensure!(
                    candidates.next().is_none(),
                    "Ambiguous actual beam emission"
                );
                let output = rest_map.carry(beam_output).ok_or_else(|| {
                    anyhow::anyhow!("Missing original rest-stage continuation of beam member")
                })?;
                let old = roxmltree::Document::parse(
                    before
                        .get(input.clone())
                        .ok_or_else(|| anyhow::anyhow!("Invalid original beam note range"))?,
                )?;
                let new = roxmltree::Document::parse(
                    after
                        .get(output.clone())
                        .ok_or_else(|| anyhow::anyhow!("Invalid emitted beam note range"))?,
                )?;
                anyhow::ensure!(
                    old.root_element().has_tag_name("note")
                        && new.root_element().has_tag_name("note"),
                    "Beam carrier is not a complete note"
                );
                anyhow::ensure!(
                    !new.root_element()
                        .children()
                        .any(|n| n.has_tag_name("rest")),
                    "Beam owner captured a rest as pitched occurrence"
                );
                anyhow::ensure!(
                    actual_beam_note_signature(&before[input.clone()], true)?
                        == actual_beam_note_signature(&after[output.clone()], true)?,
                    "Non-beam note content changed inside the owner continuation"
                );
                let states = actual_beam_written_states(new.root_element())?;
                anyhow::ensure!(
                    states
                        .keys()
                        .all(|level| (1..=emission.owned_levels).contains(level)),
                    "Written beam exceeds captured owner levels"
                );
                written.push(states);
                ranges.push(output);
            }
            let mut by_event = vec![Vec::new(); ranges.len()];
            for level in 1..=emission.owned_levels {
                let mut edits = emission
                    .edits
                    .iter()
                    .filter(|(_, l, _)| *l == level)
                    .collect::<Vec<_>>();
                edits.sort_by_key(|(event, _, _)| *event);
                let mut open = None;
                let mut next_group = 0usize;
                let mut observed = std::collections::BTreeSet::new();
                let mut membership = vec![None; ranges.len()];
                let mut states = vec![None; ranges.len()];
                for (event, _, state) in edits {
                    anyhow::ensure!(
                        *event < ranges.len() && observed.insert(*event),
                        "Duplicate or foreign actual beam event"
                    );
                    match state.as_str() {
                        "begin" => {
                            anyhow::ensure!(open.is_none(), "Overlapping actual beam groups");
                            open = Some(next_group);
                            next_group += 1;
                        }
                        "continue" | "end" => {
                            anyhow::ensure!(open.is_some(), "Unowned actual beam continuation")
                        }
                        _ => anyhow::bail!("Unsupported actual beam decision"),
                    }
                    membership[*event] = open.map(|group| (decision, group));
                    states[*event] = Some(state.clone());
                    if state == "end" {
                        open = None;
                    }
                }
                anyhow::ensure!(open.is_none(), "Unclosed actual beam decision");
                for (event, group) in membership.into_iter().enumerate() {
                    anyhow::ensure!(
                        written[event].get(&level) == states[event].as_ref(),
                        "Written beam state differs from its actual owner plan"
                    );
                    by_event[event].push((level, group, states[event].clone()));
                }
            }
            for (range, levels) in ranges.into_iter().zip(by_event) {
                anyhow::ensure!(
                    bound.insert((range.start, range.end), levels).is_none(),
                    "One physical occurrence has conflicting actual beam decisions"
                );
            }
        }
        let occurrences = bound
            .into_iter()
            .map(|((a, b), levels)| (a..b, levels))
            .collect::<Vec<_>>();
        let chord_attachments =
            capture_actual_chord_beam_attachments(before, after, beam_map, rest_map, &occurrences)
                .map_err(|error| error.to_string());
        Ok(Self {
            execution: *receipt.ssc_bound_origin().0,
            before: std::sync::Arc::from(before),
            after: std::sync::Arc::from(after),
            current: std::sync::Arc::from(after),
            occurrences,
            chord_attachments,
            course_carry: std::sync::Arc::new(beam_map.then(rest_map).0),
        })
    }
    /// Course custody carry of the original E053 execution, bound to its own
    /// receipt and owner bytes. It reconstructs nothing from later output.
    pub(crate) fn course_carry_births(
        &self,
        receipt: &BlockTransformationReceipt,
    ) -> anyhow::Result<crate::arrangement::physical_births::PhysicalRhythmicBirths> {
        self.binds_original_receipt(receipt)?;
        crate::arrangement::physical_births::PhysicalRhythmicBirths::beat_scoped_carry(
            receipt, &self.before, &self.after,
            self.course_carry.as_ref().clone())
    }
    pub(crate) fn binds_original_receipt(
        &self,
        receipt: &BlockTransformationReceipt,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(
            receipt.ssc_bound_origin().0 == &self.execution,
            "Beam capture belongs to another owner execution"
        );
        receipt.validate_block_transition::<BeatScopedBeamingBlockMint>(&self.before, &self.after)
    }
    pub(crate) fn carry_exact(
        &self,
        receipt: &crate::master_state_identity::ExactTransformationReceipt,
    ) -> anyhow::Result<Self> {
        let readback = receipt.operation_readback(&self.current)?;
        let ranges = self
            .occurrences
            .iter()
            .map(|(range, _)| readback.carry_range(range.clone()))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let attachments = self
            .chord_attachments
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|rows| {
                rows.iter()
                    .map(|row| {
                        let mut next = row.clone();
                        next.member = readback
                            .carry_range(row.member.clone())
                            .map_err(|e| e.to_string())?;
                        Ok(next)
                    })
                    .collect::<Result<Vec<_>, String>>()
            });
        self.carried(readback.post_document(), ranges, attachments)
    }
    pub(crate) fn carry_recorded(
        &self,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport,
    ) -> anyhow::Result<Self> {
        let (before, after) = transport.recorded_documents()?;
        anyhow::ensure!(
            before == self.current.as_ref(),
            "Foreign beam continuation document"
        );
        let ranges = transport.carry(
            before,
            after,
            self.occurrences
                .iter()
                .map(|(range, _)| range.clone())
                .collect(),
        )?;
        let attachments = self
            .chord_attachments
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|rows| {
                let moved = transport
                    .carry(
                        before,
                        after,
                        rows.iter().map(|row| row.member.clone()).collect(),
                    )
                    .map_err(|e| e.to_string())?;
                if moved.len() != rows.len() {
                    return Err("Incomplete original chord-member carry".into());
                }
                Ok(rows
                    .iter()
                    .cloned()
                    .zip(moved)
                    .map(|(mut row, range)| {
                        row.member = range;
                        row
                    })
                    .collect())
            });
        self.carried(after, ranges, attachments)
    }
    fn carried(
        &self,
        after: &str,
        ranges: Vec<std::ops::Range<usize>>,
        attachments: Result<Vec<ActualBeamChordAttachment>, String>,
    ) -> anyhow::Result<Self> {
        anyhow::ensure!(
            ranges.len() == self.occurrences.len(),
            "Incomplete beam continuation"
        );
        let mut output = self.clone();
        for ((old, _), range) in self.occurrences.iter().zip(&ranges) {
            anyhow::ensure!(
                actual_beam_note_signature(
                    self.current
                        .get(old.clone())
                        .ok_or_else(|| anyhow::anyhow!("Invalid carried pre-note"))?,
                    false
                )? == actual_beam_note_signature(
                    after
                        .get(range.clone())
                        .ok_or_else(|| anyhow::anyhow!("Invalid carried post-note"))?,
                    false
                )?,
                "Original note or beam decision changed at a following owner"
            );
        }
        for ((range, _), next) in output.occurrences.iter_mut().zip(ranges) {
            *range = next;
        }
        output.chord_attachments = attachments.and_then(|rows| {
            let prior = self.chord_attachments.as_ref().map_err(Clone::clone)?;
            if rows.len() != prior.len() {
                return Err("Incomplete chord attachment continuation".into());
            }
            for (before, next) in prior.iter().zip(&rows) {
                let old = self
                    .current
                    .get(before.member.clone())
                    .ok_or("Invalid original chord member")?;
                let new = after
                    .get(next.member.clone())
                    .ok_or("Invalid continued chord member")?;
                if actual_beam_note_signature(old, false).map_err(|e| e.to_string())?
                    != actual_beam_note_signature(new, false).map_err(|e| e.to_string())?
                {
                    return Err("Original chord-member content changed at a following owner".into());
                }
            }
            Ok(rows)
        });
        output.current = std::sync::Arc::from(after);
        Ok(output)
    }
    pub(crate) fn chord_attachments(
        &self,
        document: &str,
    ) -> anyhow::Result<&[ActualBeamChordAttachment]> {
        anyhow::ensure!(
            document == self.current.as_ref(),
            "Chord attachments were not carried to the current document"
        );
        self.chord_attachments
            .as_deref()
            .map_err(|error| anyhow::anyhow!("{error}"))
    }
    pub(crate) fn occurrences(
        &self,
        document: &str,
    ) -> anyhow::Result<&[(std::ops::Range<usize>, ActualBeamLevels)]> {
        anyhow::ensure!(
            document == self.current.as_ref(),
            "Actual beam decisions were not transported to this source document"
        );
        Ok(&self.occurrences)
    }
}

/// Original physical chord membership only. The head index addresses the actual
/// owner occurrence in this same execution, never a later geometry match.
#[derive(Debug, Clone)]
pub(crate) struct ActualBeamChordAttachment {
    pub(crate) member: std::ops::Range<usize>,
    pub(crate) head: Option<usize>,
    pub(crate) unknown: Option<String>,
}

fn capture_actual_chord_beam_attachments(
    before: &str,
    after: &str,
    beam_map: &ActualBeamNoteMap,
    rest_map: &ActualBeamNoteMap,
    occurrences: &[(std::ops::Range<usize>, ActualBeamLevels)],
) -> anyhow::Result<Vec<ActualBeamChordAttachment>> {
    let document = roxmltree::Document::parse(before)?;
    let mut rows = Vec::new();
    for measure in document
        .descendants()
        .filter(|node| node.has_tag_name("measure"))
    {
        let mut physical_head: Option<std::ops::Range<usize>> = None;
        for node in measure.children().filter(|node| node.is_element()) {
            if node.has_tag_name("forward") || node.has_tag_name("backup") {
                physical_head = None;
            }
            if !node.has_tag_name("note") {
                continue;
            }
            let chord = node.children().any(|child| child.has_tag_name("chord"));
            let pitched = node.children().any(|child| child.has_tag_name("pitch"));
            if !chord {
                physical_head = pitched.then(|| node.range());
                continue;
            }
            if !pitched {
                continue;
            }
            let input = node.range();
            let output = beam_map
                .carry(&input)
                .and_then(|range| rest_map.carry(&range))
                .ok_or_else(|| {
                    anyhow::anyhow!("Missing original chord-member operation continuation")
                })?;
            anyhow::ensure!(
                actual_beam_note_signature(&before[input], false)?
                    == actual_beam_note_signature(
                        after
                            .get(output.clone())
                            .ok_or_else(|| anyhow::anyhow!("Invalid emitted chord member"))?,
                        false
                    )?,
                "Original chord member changed inside the beam owner"
            );
            let head_range = physical_head
                .as_ref()
                .and_then(|range| beam_map.carry(range))
                .and_then(|range| rest_map.carry(&range));
            let head = head_range.as_ref().and_then(|range| {
                let mut heads = occurrences
                    .iter()
                    .enumerate()
                    .filter(|(_, (candidate, _))| candidate == range);
                let first = heads.next()?.0;
                heads.next().is_none().then_some(first)
            });
            let unknown = head.is_none().then(|| {
                if physical_head.is_none() {
                    "No original pitched physical chord head".to_owned()
                } else {
                    "Original physical head has no captured owner beam decision".to_owned()
                }
            });
            rows.push(ActualBeamChordAttachment {
                member: output,
                head,
                unknown,
            });
        }
    }
    Ok(rows)
}

#[cfg(test)]
#[path = "../../tests/unit/actual_beam_owner_capture.rs"]
pub(crate) mod actual_beam_owner_capture_tests;
