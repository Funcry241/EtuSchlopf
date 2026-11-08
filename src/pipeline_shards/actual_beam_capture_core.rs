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
    actual_beam_note_signature_with_spelling(text, omit_beams, false)
}

fn actual_beam_note_signature_with_spelling(
    text: &str,
    omit_beams: bool,
    allow_respelling: bool,
) -> anyhow::Result<Vec<String>> {
    fn visit<'a, 'input>(
        node: roxmltree::Node<'a, 'input>,
        omit_beams: bool,
        pitch: Option<roxmltree::Node<'a, 'input>>,
        out: &mut Vec<String>,
    ) {
        if node.is_element() {
            if omit_beams && node.has_tag_name("beam") {
                return;
            }
            if pitch.is_some_and(|pitch| node == pitch
                || (node.has_tag_name("accidental") && node.parent() == pitch.parent()))
            {
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
                visit(child, omit_beams, pitch, out);
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
    let note = doc.root_element();
    let pitch = allow_respelling.then(|| note.children().find(|node| node.has_tag_name("pitch"))).flatten();
    visit(note, omit_beams, pitch, &mut output);
    if let Some(pitch) = pitch {
        let field = |name| pitch.children().find(|node| node.has_tag_name(name)).and_then(|node| node.text()).map(str::trim);
        let step = match field("step") {
            Some("C") => 0, Some("D") => 2, Some("E") => 4, Some("F") => 5,
            Some("G") => 7, Some("A") => 9, Some("B") => 11,
            _ => anyhow::bail!("Invalid pitch step in Render respelling"),
        };
        let octave = field("octave").ok_or_else(|| anyhow::anyhow!("Missing pitch octave"))?.parse::<i64>()?;
        let alter = field("alter").unwrap_or("0").parse::<i32>()?;
        let semitone = octave.checked_mul(12).and_then(|value| value.checked_add(step + i64::from(alter)))
            .ok_or_else(|| anyhow::anyhow!("Pitch overflow in Render respelling"))?;
        output.push(format!("sounding-pitch:{semitone}"));
    }
    Ok(output)
}

pub(crate) fn verify_render_respelling_note(before: &str, after: &str) -> anyhow::Result<()> {
    anyhow::ensure!(
        actual_beam_note_signature_with_spelling(before, false, true)?
            == actual_beam_note_signature_with_spelling(after, false, true)?,
        "Render respelling changed sounding pitch or other note content"
    );
    Ok(())
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
fn beam_note_field<'a>(note: roxmltree::Node<'a, '_>, name: &str) -> Option<&'a str> {
    note.children().find(|child| child.has_tag_name(name))?.text()
}

fn beam_written_strand<'a>(note: roxmltree::Node<'a, '_>) -> (&'a str, Option<&'a str>) {
    (beam_note_field(note, "staff").unwrap_or("1"), beam_note_field(note, "voice"))
}

/// Read one already owned edge in a forward written strand. A backup,
/// a competing event, or a beamed rest needs a stronger order proof.
fn e050_written_edge(
    left: roxmltree::Node<'_, '_>,
    right: roxmltree::Node<'_, '_>,
    level: u8,
) -> Option<bool> {
    if left.parent() != right.parent() || left.range().start >= right.range().start
        || beam_written_strand(left) != beam_written_strand(right) || beam_written_strand(left).1.is_none_or(str::is_empty)
    {
        return None;
    }
    for note in [left, right] {
        if beam_note_field(note, "duration")?.parse::<u64>().ok()? == 0
            || note.children().any(|child| child.has_tag_name("chord") || child.has_tag_name("grace"))
            || !note.children().any(|child| child.has_tag_name("pitch"))
        {
            return None;
        }
    }
    for sibling in left.next_siblings().skip(1).take_while(|node| *node != right) {
        if sibling.has_tag_name("backup") {
            return None;
        }
        if sibling.has_tag_name("note") && beam_written_strand(sibling) == beam_written_strand(left)
            && (!sibling.children().any(|child| child.has_tag_name("rest"))
                || actual_beam_written_states(sibling).ok()?.contains_key(&level))
        {
            return None;
        }
    }
    let outgoing = actual_beam_written_states(left).ok()?;
    let incoming = actual_beam_written_states(right).ok()?;
    let outgoing = match outgoing.get(&level).map(String::as_str) {
        Some("begin" | "continue") => true,
        Some("end") | None => false,
        _ => return None,
    };
    let incoming = match incoming.get(&level).map(String::as_str) {
        Some("continue" | "end") => true,
        Some("begin") | None => false,
        _ => return None,
    };
    Some(outgoing && incoming)
}

/// Locate the next written endpoint only when a visible rest of the same
/// strand intervenes. The edge readback independently verifies its markers.
fn e050_next_note_over_visible_rest<'a, 'input>(
    left: roxmltree::Node<'a, 'input>,
) -> Option<roxmltree::Node<'a, 'input>> {
    let strand = beam_written_strand(left);
    let mut visible_rest = false;
    for note in left.next_siblings().skip(1) {
        if note.has_tag_name("backup") {
            return None;
        }
        if !note.has_tag_name("note") || beam_written_strand(note) != strand {
            continue;
        }
        if !note.children().any(|child| child.has_tag_name("rest")) {
            return visible_rest.then_some(note);
        }
        visible_rest |= note.attribute("print-object") != Some("no")
            && beam_note_field(note, "duration").and_then(|text| text.parse::<u64>().ok())
                .is_some_and(|duration| duration > 0);
    }
    None
}

#[derive(Debug, Clone)]
struct OriginalBridgeBeamEdge {
    level: u8,
    group: (usize, usize),
    original_left: std::ops::Range<usize>,
    original_right: std::ops::Range<usize>,
    left: std::ops::Range<usize>,
    right: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub(crate) struct ActualCanonicalBeamCapture {
    execution: crate::master_state_identity::MintedIdentity,
    before: std::sync::Arc<str>,
    after: std::sync::Arc<str>,
    current: std::sync::Arc<str>,
    occurrences: Vec<(std::ops::Range<usize>, ActualBeamLevels)>,
    chord_attachments: Result<Vec<ActualBeamChordAttachment>, String>,
    bridge_edges: Vec<OriginalBridgeBeamEdge>,
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
            bridge_edges: Vec::new(),
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
                        next.original_head = row.original_head.as_ref().map(|range|
                            readback.carry_range(range.clone()).map_err(|e| e.to_string()))
                            .transpose()?;
                        next.original_rest_host = row.original_rest_host.as_ref().map(|range|
                            readback.carry_range(range.clone()).map_err(|e| e.to_string()))
                            .transpose()?;
                        Ok(next)
                    })
                    .collect::<Result<Vec<_>, String>>()
            });
        self.carried(readback.post_document(), ranges, attachments, false,
            |range| readback.carry_range(range.clone()).ok())
    }
    pub(crate) fn carry_recorded(
        &self,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport,
    ) -> anyhow::Result<Self> {
        self.carry_recorded_with_spelling(transport, false)
    }

    pub(crate) fn carry_respelling(
        &self,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport,
    ) -> anyhow::Result<Self> {
        self.carry_recorded_with_spelling(transport, true)
    }

    fn carry_recorded_with_spelling(
        &self,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport,
        allow_respelling: bool,
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
                let head_indices = rows.iter().enumerate().filter_map(|(index, row)|
                    row.original_head.as_ref().map(|range| (index, range.clone())))
                    .collect::<Vec<_>>();
                let moved_heads = transport.carry(before, after,
                    head_indices.iter().map(|(_, range)| range.clone()).collect())
                    .map_err(|error| error.to_string())?;
                if moved_heads.len() != head_indices.len() {
                    return Err("Incomplete original chord-head carry".into());
                }
                let moved_heads = head_indices.into_iter().zip(moved_heads)
                    .map(|((index, _), range)| (index, range)).collect::<std::collections::BTreeMap<_, _>>();
                let rest_indices = rows.iter().enumerate().filter_map(|(index, row)|
                    row.original_rest_host.as_ref().map(|range| (index, range.clone())))
                    .collect::<Vec<_>>();
                let moved_rests = transport.carry(before, after,
                    rest_indices.iter().map(|(_, range)| range.clone()).collect())
                    .map_err(|error| error.to_string())?;
                if moved_rests.len() != rest_indices.len() {
                    return Err("Incomplete original chord rest-host carry".into());
                }
                let moved_rests = rest_indices.into_iter().zip(moved_rests)
                    .map(|((index, _), range)| (index, range)).collect::<std::collections::BTreeMap<_, _>>();
                Ok(rows
                    .iter()
                    .cloned()
                    .zip(moved)
                    .enumerate()
                    .map(|(index, (mut row, range))| {
                        row.member = range;
                        row.original_head = moved_heads.get(&index).cloned();
                        row.original_rest_host = moved_rests.get(&index).cloned();
                        row
                    })
                    .collect())
            });
        let endpoints = self.bridge_edges.iter().flat_map(|edge| [&edge.left, &edge.right])
            .map(|range| (range.start, range.end)).collect::<std::collections::BTreeSet<_>>();
        let mapped = if endpoints.is_empty() {
            Vec::new()
        } else {
            transport.carry_observed_ranges(
                before, after, endpoints.iter().map(|&(start, end)| start..end).collect(),
            )?
        };
        let mapped = endpoints.into_iter().zip(mapped)
            .collect::<std::collections::BTreeMap<_, _>>();
        self.carried(after, ranges, attachments, allow_respelling,
            |range| mapped.get(&(range.start, range.end)).cloned().flatten())
    }
    /// Continue unaffected E053 evidence through the original E050 journal.
    /// An edit retires the entire emitter batch that owns its note, including
    /// every beam level. E050 cannot grant a new E053 group.
    pub(crate) fn carry_rest_bridge(
        &self,
        receipt: &crate::master_state_identity::PreservingTransformationReceipt,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport,
    ) -> anyhow::Result<Self> {
        let step = transport.rest_bridge_step(receipt)?;
        anyhow::ensure!(
            step.original_documents().0 == self.current.as_ref(),
            "Foreign rest bridge continuation document"
        );
        let touches = |range: &std::ops::Range<usize>| {
            step.original_operations().iter().any(|(edit, _)| {
                edit.start < range.end && range.start < edit.end
            })
        };
        let retained = self.retain_unaffected(touches);
        let mut carried = retained.carry_recorded(transport)?;
        let seam = "E053MetricRestBridgeReplay";
        match self.rest_bridge_readback(&retained, step, transport, seam) {
            Ok((report, edges)) => {
                carried.bridge_edges.extend(edges);
                eprint!("{report}");
            }
            Err(error) => eprintln!("OriginalBeamBridgeReadback: seam={seam}; result=Unknown; reason={error}; authority=none"),
        }
        Ok(carried)
    }

    fn rest_bridge_readback(
        &self,
        retained: &Self,
        step: &crate::arrangement::representation_transport::RepresentationStep,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport,
        seam: &str,
    ) -> anyhow::Result<(String, Vec<OriginalBridgeBeamEdge>)> {
        use std::collections::{BTreeMap, BTreeSet};
        use std::fmt::Write as _;

        let (before, after) = step.original_documents();
        let original = roxmltree::Document::parse(before)?;
        let projected = roxmltree::Document::parse(after)?;
        let by_range = |range: &std::ops::Range<usize>| (range.start, range.end);
        let source_notes = original.descendants().filter(|node| node.has_tag_name("note"))
            .map(|node| (by_range(&node.range()), node)).collect::<BTreeMap<_, _>>();
        let output_notes = projected.descendants().filter(|node| node.has_tag_name("note"))
            .map(|node| (by_range(&node.range()), node)).collect::<BTreeMap<_, _>>();
        let edited = step.original_operations().iter().map(|(range, _)| by_range(range))
            .collect::<BTreeSet<_>>();
        let edited_measures = edited.iter().filter_map(|key| source_notes.get(key))
            .filter_map(|note| note.parent().filter(|parent| parent.has_tag_name("measure")))
            .map(|measure| measure.range().start).collect::<BTreeSet<_>>();
        let surviving = retained.occurrences.iter().map(|(range, _)| by_range(range))
            .collect::<BTreeSet<_>>();
        let owned = self.occurrences.iter().map(|(range, levels)| (by_range(range), levels))
            .collect::<BTreeMap<_, _>>();

        // Include actual operations, every retired capture row, and retained
        // owner rows in the edited measures. The last group gives neighboring
        // source boundaries without treating them as bridge grants.
        let mut observed = edited.clone();
        for &key in owned.keys() {
            let in_edited_measure = source_notes.get(&key).and_then(|note| note.parent())
                .is_some_and(|measure| edited_measures.contains(&measure.range().start));
            if !surviving.contains(&key) || in_edited_measure {
                observed.insert(key);
            }
        }
        let mut compare = observed.clone();
        compare.extend(source_notes.iter().filter_map(|(&key, note)| {
            note.parent().filter(|measure| edited_measures.contains(&measure.range().start))
                .map(|_| key)
        }));
        let carried = transport.carry_observed_ranges(
            before, after, compare.iter().map(|&(start, end)| start..end).collect(),
        )?;

        let mut report = String::new();
        let mut unknown = 0;
        let mut mapped_notes = BTreeMap::new();
        for (key, post_range) in compare.into_iter().zip(carried) {
            let is_edited = edited.contains(&key);
            let owner_levels = owned.get(&key);
            let custody = match (owner_levels.is_some(), surviving.contains(&key)) {
                (true, true) => "retained",
                (true, false) => "retired",
                (false, _) => "no_capture",
            };
            let projected_note = post_range.as_ref().and_then(|range| output_notes.get(&by_range(range)));
            let (Some(&left), Some(&right)) = (source_notes.get(&key), projected_note) else {
                if !observed.contains(&key) { continue; }
                unknown += 1;
                let _ = writeln!(report, "OriginalBeamBridgeReadbackNote: before_range={key:?}; edited={is_edited}; capture={custody}; result=Unknown; authority=none");
                continue;
            };
            if actual_beam_note_signature(&before[left.range()], true)?
                != actual_beam_note_signature(&after[right.range()], true)?
            {
                if !observed.contains(&key) { continue; }
                unknown += 1;
                let _ = writeln!(report, "OriginalBeamBridgeReadbackNote: before_range={key:?}; edited={is_edited}; capture={custody}; result=Unknown; reason=non_beam_content_changed; authority=none");
                continue;
            }
            mapped_notes.insert(key, (left, right));
            if !observed.contains(&key) { continue; }
            let field = |tag| beam_note_field(left, tag);
            let measure = left.parent().and_then(|node| node.attribute("number"));
            let ordinal = left.prev_siblings().skip(1).filter(|node| node.has_tag_name("note")).count();
            let pitch = left.children().find(|node| node.has_tag_name("pitch")).map(|node| {
                let value = |name| node.children().find(|child| child.has_tag_name(name)).and_then(|child| child.text());
                (value("step"), value("alter"), value("octave"))
            });
            let ties = left.children().filter(|node| node.has_tag_name("tie"))
                .filter_map(|node| node.attribute("type")).collect::<Vec<_>>();
            let before_levels = actual_beam_written_states(left)?;
            let after_levels = actual_beam_written_states(right)?;
            let _ = writeln!(report,
                "OriginalBeamBridgeReadbackNote: measure={measure:?}; ordinal={ordinal}; staff={:?}; voice={:?}; pitch={pitch:?}; ties={ties:?}; before_range={key:?}; edited={is_edited}; capture={custody}; owner_levels={owner_levels:?}; before={before_levels:?}; after={after_levels:?}; authority=none",
                field("staff"), field("voice"));
        }
        let _ = writeln!(report,
            "OriginalBeamBridgeReadback: seam={seam}; operations={}; retired_occurrences={}; unknown={unknown}; authority=none",
            edited.len(), self.occurrences.len() - retained.occurrences.len());

        // Enumerate only original owner edges. The post-state can attest to
        // those edges, while additions have no original owner identity here.
        let mut groups = BTreeMap::<_, Vec<_>>::new();
        for (&key, levels) in &owned {
            for (level, group, marker) in *levels {
                if let Some(group) = group {
                    groups.entry((*level, *group)).or_default().push((key, marker.as_deref()));
                }
            }
        }
        let mut edges = Vec::new();
        let mut planned_edges = BTreeSet::new();
        let (mut present, mut missing, mut unknown) = (0, 0, 0);
        for ((level, group), members) in groups {
            if members.iter().all(|(key, _)| surviving.contains(key)) {
                continue;
            }
            if members.len() < 2 || members[0].1 != Some("begin")
                || members[members.len() - 1].1 != Some("end")
                || members[1..members.len() - 1].iter().any(|row| row.1 != Some("continue"))
            {
                let _ = writeln!(report, "OriginalBeamBridgeEdgeGroup: level={level}; group={group:?}; result=Unknown; reason=unclosed_capture; authority=none");
                continue;
            }
            for pair in members.windows(2) {
                let (left, right) = (pair[0].0, pair[1].0);
                planned_edges.insert((level, left, right));
                let mapped = mapped_notes.get(&left).zip(mapped_notes.get(&right));
                let outcome = mapped.and_then(|(&(before_left, after_left), &(before_right, after_right))| {
                    (e050_written_edge(before_left, before_right, level) == Some(true))
                        .then(|| e050_written_edge(after_left, after_right, level)).flatten()
                });
                let result = match (outcome, mapped) {
                    (Some(true), Some(((_, after_left), (_, after_right)))) => {
                        present += 1;
                        edges.push(OriginalBridgeBeamEdge {
                            level,
                            group,
                            original_left: left.0..left.1,
                            original_right: right.0..right.1,
                            left: after_left.range(),
                            right: after_right.range(),
                        });
                        "Present"
                    }
                    (Some(false), _) => { missing += 1; "Missing" }
                    _ => { unknown += 1; "Unknown" }
                };
                let measure = source_notes.get(&left).and_then(|note| note.parent())
                    .and_then(|node| node.attribute("number"));
                let _ = writeln!(report, "OriginalBeamBridgeEdge: measure={measure:?}; level={level}; group={group:?}; left_range={left:?}; right_range={right:?}; result={result}; authority=none");
            }
        }
        let _ = writeln!(report, "OriginalBeamBridgeEdgeReadback: seam={seam}; scope=affected_original_groups; present={present}; missing={missing}; unknown={unknown}; authority=none");
        // Compare the actual E050 post-state with edges from the already
        // executed metric BeamPlan. A missing level capture stays Unknown.
        let post_to_pre = mapped_notes.iter().map(|(&key, &(_, note))|
            (by_range(&note.range()), key)).collect::<BTreeMap<_, _>>();
        let level_was_captured = |key, level| owned.get(&key)
            .is_some_and(|rows| rows.iter().any(|(owned_level, _, _)| *owned_level == level));
        let (mut captured, mut outside, mut unbound) = (0, 0, 0);
        for (&left_key, &(before_left, after_left)) in &mapped_notes {
            let Some(after_right) = e050_next_note_over_visible_rest(after_left) else { continue; };
            let Some(&right_key) = post_to_pre.get(&by_range(&after_right.range())) else { continue; };
            if !edited.contains(&left_key) && !edited.contains(&right_key) { continue; }
            let (before_right, _) = mapped_notes[&right_key];
            for level in [1, 2] {
                if e050_written_edge(after_left, after_right, level) != Some(true) { continue; }
                let before_edge = match e050_written_edge(before_left, before_right, level) {
                    Some(true) => "Present",
                    Some(false) => "Missing",
                    None => "Unknown",
                };
                let plan = if planned_edges.contains(&(level, left_key, right_key)) {
                    captured += 1;
                    "Captured"
                } else if level_was_captured(left_key, level) && level_was_captured(right_key, level) {
                    outside += 1;
                    "OutsideCapturedPlan"
                } else {
                    unbound += 1;
                    "Unknown"
                };
                let measure = before_left.parent().and_then(|node| node.attribute("number"));
                let _ = writeln!(report, "MetricBeamRestEdge: measure={measure:?}; level={level}; left_range={left_key:?}; right_range={right_key:?}; before_e050={before_edge}; metric_plan={plan}; authority=none");
            }
        }
        let _ = writeln!(report, "MetricBeamRestEdgeReadback: seam={seam}; scope=confirmed_rest_edges_with_edited_endpoint; captured={captured}; outside_captured_plan={outside}; unknown={unbound}; authority=none");
        Ok((report, edges))
    }

    fn retain_unaffected(&self, touches: impl Fn(&std::ops::Range<usize>) -> bool) -> Self {
        let batches = self.occurrences.iter()
            .filter(|(range, _)| touches(range))
            .flat_map(|(_, levels)| levels.iter().filter_map(|(_, group, _)| group.map(|(batch, _)| batch)))
            .collect::<std::collections::BTreeSet<_>>();
        let mut retained = self.clone();
        retained.occurrences.clear();
        let mut indices = Vec::with_capacity(self.occurrences.len());
        for (range, levels) in &self.occurrences {
            let removed = touches(range) || levels.iter()
                .any(|(_, group, _)| group.is_some_and(|(batch, _)| batches.contains(&batch)));
            indices.push((!removed).then_some(retained.occurrences.len()));
            if !removed {
                retained.occurrences.push((range.clone(), levels.clone()));
            }
        }
        retained.chord_attachments = self.chord_attachments.as_ref().map_err(Clone::clone)
            .map(|rows| rows.iter().filter_map(|row| {
                if touches(&row.member) || row.original_head.as_ref().is_some_and(&touches) {
                    return None;
                }
                let mut row = row.clone();
                // A surviving pitched member must retain the negative record
                // when only its non-pitched support loses continuation. Dropping
                // the whole row would hide the missing dependency from C3.
                if row.original_rest_host.as_ref().is_some_and(&touches) {
                    row.original_rest_host = None;
                }
                if let Some(head) = row.head {
                    row.head = Some(indices.get(head).copied().flatten()?);
                }
                Some(row)
            }).collect());
        retained
    }

    pub(crate) fn current_document(&self) -> &str {
        &self.current
    }

    pub(crate) fn carry_final_chord<'a>(&self,
        after: &str,
        single_emissions: impl Iterator<Item = (&'a std::ops::Range<usize>, &'a std::ops::Range<usize>)>,
    ) -> anyhow::Result<Self> {
        let unchanged = single_emissions.filter_map(|(input, range)| {
            let old = self.current.get(input.clone())?;
            let new = after.get(range.clone())?;
            (actual_beam_note_signature(old, false).ok()? == actual_beam_note_signature(new, false).ok()?)
                .then_some(((input.start, input.end), range))
        }).collect::<std::collections::BTreeMap<_, _>>();
        let locate = |range: &std::ops::Range<usize>| unchanged.get(&(range.start, range.end)).map(|range| (*range).clone());
        let retained = self.retain_unaffected(|range| locate(range).is_none());
        let ranges = retained.occurrences.iter().map(|(range, _)| locate(range)
            .ok_or_else(|| anyhow::anyhow!("FinalChord retained note has no unique unchanged emission")))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let attachments = retained.chord_attachments.as_ref().map_err(Clone::clone).and_then(|rows|
            rows.iter().cloned().map(|mut row| {
                row.member = locate(&row.member).ok_or("Missing FinalChord chord member emission")?;
                row.original_head = row.original_head.as_ref().map(|range|
                    locate(range).ok_or("Missing FinalChord chord head emission")).transpose()?;
                row.original_rest_host = row.original_rest_host.as_ref().map(|range|
                    locate(range).ok_or("Missing FinalChord chord rest-host emission")).transpose()?;
                Ok(row)
            }).collect());
        retained.carried(after, ranges, attachments, false, locate)
    }

    fn carried(
        &self,
        after: &str,
        ranges: Vec<std::ops::Range<usize>>,
        attachments: Result<Vec<ActualBeamChordAttachment>, String>,
        allow_respelling: bool,
        locate: impl Fn(&std::ops::Range<usize>) -> Option<std::ops::Range<usize>>,
    ) -> anyhow::Result<Self> {
        anyhow::ensure!(
            ranges.len() == self.occurrences.len(),
            "Incomplete beam continuation"
        );
        let signature = |note: &str| actual_beam_note_signature_with_spelling(note, false, allow_respelling);
        let mut output = self.clone();
        for ((old, _), range) in self.occurrences.iter().zip(&ranges) {
            anyhow::ensure!(
                signature(
                    self.current
                        .get(old.clone())
                        .ok_or_else(|| anyhow::anyhow!("Invalid carried pre-note"))?,
                )? == signature(
                    after
                        .get(range.clone())
                        .ok_or_else(|| anyhow::anyhow!("Invalid carried post-note"))?,
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
                match (&before.original_rest_host, &next.original_rest_host) {
                    (Some(old), Some(new)) => {
                        let old = self.current.get(old.clone()).ok_or("Invalid original chord rest host")?;
                        let new = after.get(new.clone()).ok_or("Invalid continued chord rest host")?;
                        if signature(old).map_err(|error| error.to_string())?
                            != signature(new).map_err(|error| error.to_string())? {
                            return Err("Original chord rest-host content changed at a following owner".into());
                        }
                    }
                    (None, None) => {}
                    _ => return Err("Original chord rest-host binding changed at a following owner".into()),
                }
                match (&before.original_head, &next.original_head) {
                    (Some(old), Some(new)) => {
                        let old = self.current.get(old.clone()).ok_or("Invalid original physical chord head")?;
                        let new = after.get(new.clone()).ok_or("Invalid continued physical chord head")?;
                        if signature(old).map_err(|error| error.to_string())?
                            != signature(new).map_err(|error| error.to_string())?
                        {
                            return Err("Original physical chord-head content changed at a following owner".into());
                        }
                    }
                    (None, None) => {}
                    _ => return Err("Original physical chord-head binding changed at a following owner".into()),
                }
                let old = self
                    .current
                    .get(before.member.clone())
                    .ok_or("Invalid original chord member")?;
                let new = after
                    .get(next.member.clone())
                    .ok_or("Invalid continued chord member")?;
                if signature(old).map_err(|e| e.to_string())?
                    != signature(new).map_err(|e| e.to_string())?
                {
                    return Err("Original chord-member content changed at a following owner".into());
                }
            }
            Ok(rows)
        });
        output.bridge_edges = self.carry_bridge_edges(after, allow_respelling, locate);
        output.current = std::sync::Arc::from(after);
        Ok(output)
    }
    fn carry_bridge_edges(
        &self,
        after: &str,
        allow_respelling: bool,
        locate: impl Fn(&std::ops::Range<usize>) -> Option<std::ops::Range<usize>>,
    ) -> Vec<OriginalBridgeBeamEdge> {
        if self.bridge_edges.is_empty() {
            return Vec::new();
        }
        let Ok(document) = roxmltree::Document::parse(after) else {
            return Vec::new();
        };
        let notes = document.descendants().filter(|node| node.has_tag_name("note"))
            .map(|node| ((node.range().start, node.range().end), node))
            .collect::<std::collections::BTreeMap<_, _>>();
        let signature = |text| actual_beam_note_signature_with_spelling(text, true, allow_respelling).ok();
        let endpoint = |range: &std::ops::Range<usize>| {
            let next = locate(range)?;
            let note = *notes.get(&(next.start, next.end))?;
            let original = signature(self.current.get(range.clone())?)?;
            (signature(after.get(next)?)? == original).then_some(note)
        };
        self.bridge_edges.iter().filter_map(|edge| {
            let left = endpoint(&edge.left)?;
            let right = endpoint(&edge.right)?;
            (e050_written_edge(left, right, edge.level) == Some(true)).then(|| OriginalBridgeBeamEdge {
                left: left.range(),
                right: right.range(),
                ..edge.clone()
            })
        }).collect()
    }

    fn observe_original_bridge_edges(&self, document: &str, stage: &str) {
        if document != self.current.as_ref() || self.bridge_edges.is_empty() {
            return;
        }
        for edge in &self.bridge_edges {
            eprintln!(
                "RenderOriginalBridgeBeamEdge: stage={stage}; level={}; group={:?}; original_left={:?}; original_right={:?}; current_left={:?}; current_right={:?}; authority=none",
                edge.level, edge.group, edge.original_left, edge.original_right, edge.left, edge.right,
            );
        }
        eprintln!("RenderOriginalBridgeBeamEdges: stage={stage}; present={}; authority=none", self.bridge_edges.len());
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
    /// Physical head emitted from the same original operation, including heads
    /// outside the beam decision scope. This is not a positive beam decision.
    pub(crate) original_head: Option<std::ops::Range<usize>>,
    /// Non-pitched cursor host, captured by the same original emitter. This
    /// relationship is neither a pitched head nor an independent beam decision.
    pub(crate) original_rest_host: Option<std::ops::Range<usize>>,
    /// Original emitter proved there was no pitched physical head. Distinct
    /// from a head whose operation continuation is missing or ambiguous.
    pub(crate) original_head_absent: bool,
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
        let mut rest_host: Option<std::ops::Range<usize>> = None;
        for node in measure.children().filter(|node| node.is_element()) {
            if node.has_tag_name("forward") || node.has_tag_name("backup") {
                physical_head = None;
                rest_host = None;
            }
            if !node.has_tag_name("note") {
                continue;
            }
            let chord = node.children().any(|child| child.has_tag_name("chord"));
            let pitched = node.children().any(|child| child.has_tag_name("pitch"));
            if !chord {
                physical_head = pitched.then(|| node.range());
                rest_host = node.children().any(|child| child.has_tag_name("rest"))
                    .then(|| node.range());
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
            if let (Some(input), Some(output)) = (&physical_head, &head_range) {
                anyhow::ensure!(
                    actual_beam_note_signature(&before[input.clone()], true)?
                        == actual_beam_note_signature(after.get(output.clone())
                            .ok_or_else(|| anyhow::anyhow!("Invalid emitted physical chord head"))?, true)?,
                    "Original physical chord head changed inside the beam owner"
                );
            }
            let head = head_range.as_ref().and_then(|range| {
                let mut heads = occurrences
                    .iter()
                    .enumerate()
                    .filter(|(_, (candidate, _))| candidate == range);
                let first = heads.next()?.0;
                heads.next().is_none().then_some(first)
            });
            let original_rest_host = rest_host.as_ref().map(|range| {
                let output = beam_map.carry(range).and_then(|range| rest_map.carry(&range))
                    .ok_or_else(|| anyhow::anyhow!("Missing original chord rest-host operation continuation"))?;
                anyhow::ensure!(actual_beam_note_signature(&before[range.clone()], true)?
                    == actual_beam_note_signature(after.get(output.clone())
                        .ok_or_else(|| anyhow::anyhow!("Invalid emitted chord rest host"))?, true)?,
                    "Original chord rest host changed inside the beam owner");
                Ok::<_, anyhow::Error>(output)
            }).transpose()?;
            let unknown = head.is_none().then(|| {
                if physical_head.is_none() {
                    "No original pitched physical chord head".to_owned()
                } else {
                    "Original physical head has no captured owner beam decision".to_owned()
                }
            });
            rows.push(ActualBeamChordAttachment {
                member: output,
                original_head: head_range,
                original_rest_host,
                original_head_absent: physical_head.is_none(),
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
