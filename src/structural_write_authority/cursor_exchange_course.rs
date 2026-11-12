//! Encoding-only exchange plans derived from live adopted connection custody.
//! This module never materializes XML. E057 remains the actual exchange owner.
use super::*;
use std::ops::Range;
type CursorExchangeEdits = (usize, Vec<(usize, usize, String)>);

#[derive(Debug, Clone)]
struct ExchangeSlot {
    range: Range<usize>,
    replacement: Range<usize>,
}
#[derive(Debug, Clone)]
pub(crate) struct BoundCursorExchange {
    input: String,
    origin: std::sync::Arc<()>,
    canonical_source: std::sync::Arc<[FinalChordCanonicalSourceLocator]>,
    slots: Vec<ExchangeSlot>,
    groups: usize,
}
#[derive(Debug)]
struct CursorUnit {
    range: Range<usize>,
    measure: usize,
    segment: usize,
    onset: i64,
    duration: i64,
    staff: Option<String>,
    voice: Option<String>,
    pitched: bool,
    rest: bool,
    visible: bool,
    forward: bool,
    closed: bool,
}

fn cursor_units(xml: &str) -> io::Result<Vec<CursorUnit>> {
    let doc =
        roxmltree::Document::parse(xml).map_err(|_| invalid("Cursor exchange XML is invalid"))?;
    let mut units: Vec<CursorUnit> = Vec::new();
    for measure in doc.descendants().filter(|n| n.has_tag_name("measure")) {
        let number = measure
            .attribute("number")
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| invalid("Cursor exchange measure is not bound"))?;
        let mut cursor = 0i64;
        let mut segment = 0;
        let mut anchor = None::<usize>;
        for node in measure.children().filter(|n| n.is_element()) {
            let duration = final_chord_direct_child_i64(node, "duration").unwrap_or(0);
            if node.has_tag_name("backup") {
                cursor -= duration;
                segment += 1;
                anchor = None;
            } else if node.has_tag_name("note") && final_chord_direct_has_child(node, "chord") {
                if let Some(index) = anchor {
                    let unit = &mut units[index];
                    unit.closed &= xml[unit.range.end..node.range().start].trim().is_empty()
                        && duration == unit.duration
                        && final_chord_direct_child_text(node, "voice") == unit.voice
                        && final_chord_direct_child_text(node, "staff") == unit.staff;
                    unit.range.end = node.range().end;
                } else if let Some(unit) = units.last_mut()
                    && unit.measure == number
                    && unit.segment == segment
                    && unit.forward
                {
                    // A forward with orphan chord children is not an empty slot.
                    unit.closed = false;
                }
            } else if node.has_tag_name("note") || node.has_tag_name("forward") {
                let forward = node.has_tag_name("forward");
                let rest = final_chord_direct_has_child(node, "rest");
                let index = units.len();
                units.push(CursorUnit {
                    range: node.range(),
                    measure: number,
                    segment,
                    onset: cursor,
                    duration,
                    staff: final_chord_direct_child_text(node, "staff"),
                    voice: final_chord_direct_child_text(node, "voice"),
                    pitched: !forward && final_chord_direct_has_child(node, "pitch"),
                    rest,
                    visible: node.attribute("print-object") != Some("no"),
                    forward,
                    closed: true,
                });
                anchor = (!forward).then_some(index);
                cursor += duration;
            }
        }
    }
    Ok(units)
}

fn cover_gap(
    units: &[CursorUnit],
    measure: usize,
    segment: usize,
    staff: &str,
    voice: &str,
    start: i64,
    end: i64,
) -> bool {
    let mut intervals = units
        .iter()
        .filter(|u| {
            u.measure == measure
                && u.segment == segment
                && u.staff.as_deref() == Some(staff)
                && u.voice.as_deref() == Some(voice)
                && u.rest
                && u.visible
                && u.closed
                && u.duration > 0
                && u.onset < end
                && u.onset + u.duration > start
        })
        .collect::<Vec<_>>();
    intervals.sort_by_key(|u| u.onset);
    let mut covered = start;
    for interval in intervals {
        if interval.onset != covered || interval.onset + interval.duration > end {
            return false;
        }
        covered += interval.duration;
    }
    covered == end
}

fn connection_slots(
    members: &[Member],
    units: &[CursorUnit],
    current: &BTreeMap<Position, FinalChordEncodingNote>,
) -> Option<Vec<ExchangeSlot>> {
    let mut heads = members.iter().filter(|m| m.head).collect::<Vec<_>>();
    heads.sort_by_key(|m| m.onset);
    let first = *heads.first()?;
    if heads.len() < 2
        || heads.iter().any(|m| {
            m.position.0 != first.position.0 || m.staff != first.staff || m.voice != first.voice
        })
    {
        return None;
    }
    let mut head_units = Vec::new();
    for member in &heads {
        let bound = current.get(&member.position)?;
        let matching = units
            .iter()
            .filter(|u| {
                u.range.start == bound.range.start && u.measure == member.position.0 && u.pitched
            })
            .collect::<Vec<_>>();
        let [unit] = matching.as_slice() else {
            return None;
        };
        if !unit.closed || !unit.visible {
            return None;
        }
        head_units.push(*unit);
    }
    let mut gaps = Vec::new();
    for pair in heads.windows(2) {
        let end = pair[0].onset.checked_add(pair[0].duration)?;
        if end > pair[1].onset {
            return None;
        }
        if end < pair[1].onset {
            gaps.push((end, pair[1].onset));
        }
    }
    if gaps.is_empty() {
        return None;
    }
    let end = heads.last()?.onset + heads.last()?.duration;
    if units.iter().any(|u| {
        u.measure == first.position.0
            && u.pitched
            && u.staff.as_deref() == Some(first.staff.as_str())
            && u.voice.as_deref() == Some(first.voice.as_str())
            && u.onset < end
            && u.onset + u.duration > first.onset
            && !head_units.iter().any(|head| head.range == u.range)
    }) {
        return None;
    }
    let segments = units
        .iter()
        .filter(|u| u.measure == first.position.0 && u.rest)
        .map(|u| u.segment)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|segment| {
            head_units.iter().all(|head| head.segment < *segment)
                && gaps.iter().all(|(a, b)| {
                    cover_gap(
                        units,
                        first.position.0,
                        *segment,
                        &first.staff,
                        &first.voice,
                        *a,
                        *b,
                    )
                })
        })
        .collect::<Vec<_>>();
    let [segment] = segments.as_slice() else {
        return None;
    };
    let mut slots = Vec::new();
    for unit in head_units {
        if unit.segment == *segment {
            continue;
        }
        let targets = units
            .iter()
            .filter(|target| {
                target.measure == unit.measure
                    && target.segment == *segment
                    && target.forward
                    && target.closed
                    && target.onset == unit.onset
                    && target.duration == unit.duration
                    && target.staff == unit.staff
                    && target.voice == unit.voice
            })
            .collect::<Vec<_>>();
        let [target] = targets.as_slice() else {
            return None;
        };
        slots.push(ExchangeSlot {
            range: unit.range.clone(),
            replacement: target.range.clone(),
        });
        slots.push(ExchangeSlot {
            range: target.range.clone(),
            replacement: unit.range.clone(),
        });
    }
    (!slots.is_empty()).then_some(slots)
}

fn overlaps(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

impl BoundCursorExchange {
    fn shift_before(&self, position: usize) -> io::Result<isize> {
        self.slots
            .iter()
            .filter(|s| s.range.end <= position)
            .try_fold(0isize, |sum, s| {
                let difference = isize::try_from(s.replacement.len())
                    .ok()
                    .zip(isize::try_from(s.range.len()).ok())
                    .and_then(|(a, b)| a.checked_sub(b))
                    .ok_or_else(|| invalid("Cursor exchange range overflow"))?;
                sum.checked_add(difference)
                    .ok_or_else(|| invalid("Cursor exchange offset overflow"))
            })
    }
    fn shifted(position: usize, shift: isize) -> io::Result<usize> {
        position
            .checked_add_signed(shift)
            .ok_or_else(|| invalid("Cursor exchange offset out of range"))
    }
    fn image(&self, original: &Range<usize>) -> io::Result<Range<usize>> {
        let containing = self
            .slots
            .iter()
            .find(|s| s.range.start <= original.start && original.end <= s.range.end);
        let start = if let Some(source) = containing {
            let targets = self
                .slots
                .iter()
                .filter(|s| s.replacement == source.range)
                .collect::<Vec<_>>();
            let [target] = targets.as_slice() else {
                return Err(invalid("Cursor exchange is not bijective"));
            };
            Self::shifted(target.range.start, self.shift_before(target.range.start)?)?
                .checked_add(original.start - source.range.start)
                .ok_or_else(|| invalid("Cursor exchange relative offset overflow"))?
        } else {
            if self.slots.iter().any(|s| overlaps(&s.range, original)) {
                return Err(invalid("Cursor exchange splits a resource"));
            }
            Self::shifted(original.start, self.shift_before(original.start)?)?
        };
        Ok(start
            ..start
                .checked_add(original.len())
                .ok_or_else(|| invalid("Cursor exchange image overflow"))?)
    }
    fn verify_bytes(&self, output: &str) -> io::Result<()> {
        let mut old = 0;
        let mut new = 0;
        for slot in &self.slots {
            let unchanged = &self.input[old..slot.range.start];
            if output.get(new..new + unchanged.len()) != Some(unchanged) {
                return Err(invalid("Cursor exchange exterior bytes changed"));
            }
            new += unchanged.len();
            let replacement = &self.input[slot.replacement.clone()];
            if output.get(new..new + replacement.len()) != Some(replacement) {
                return Err(invalid("Cursor exchange differs from its bound operation"));
            }
            new += replacement.len();
            old = slot.range.end;
        }
        if output.get(new..) != self.input.get(old..) {
            return Err(invalid("Cursor exchange trailing bytes changed"));
        }
        Ok(())
    }
}

impl CanonicalFinalChordStructuralPlan {
    pub(crate) fn bind_rest_bridge_cursor_exchange(
        &self,
        source: &FinalPdfSourceOccurrenceContinuation,
        voices: &FrozenSerializedVoicePlan,
        omissions: &BTreeSet<RenderPrunedFinalChordOmissionKey>,
    ) -> io::Result<Self> {
        verify_final_pdf_source_occurrence_continuation(
            source.xml(),
            self,
            voices,
            Some(omissions),
            source,
        )?;
        let mut proposals = Vec::new();
        if let Some(course) = &source.receipt.rest_bridge_extension_course {
            if course.xml != source.xml()
                || !std::sync::Arc::ptr_eq(&course.source, &self.source_locators)
            {
                return Err(invalid(
                    "Cursor exchange has stale adopted connection custody",
                ));
            }
            let current = notes(source.xml())?;
            let identities = source
                .receipt
                .occurrences
                .iter()
                .map(|w| {
                    (
                        (w.measure, w.pitched_ordinal),
                        (w.source_event_id, w.attack_id, w.fragment_id),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            let units = cursor_units(source.xml())?;
            for members in course
                .connections
                .iter()
                .map(|c| c.all().cloned().collect::<Vec<_>>())
                .chain(course.authored.iter().map(|c| c.members.clone()))
            {
                if !members.iter().all(|m| {
                    identities.get(&m.position) == Some(&m.identity)
                        && current
                            .get(&m.position)
                            .is_some_and(|n| m.matches(source.xml(), n, false))
                }) || !resource_attachments_match(&members.iter().collect::<Vec<_>>(), &current)
                {
                    return Err(invalid("Cursor exchange connection binding drift"));
                }
                if let Some(slots) = connection_slots(&members, &units, &current) {
                    // Each moved unit must be the exact occurrence selected by
                    // existing custody, not a replacement chosen by resemblance.
                    let exact = members.iter().filter(|m| m.head).all(|m| {
                        let n = &current[&m.position];
                        units.iter().any(|u| {
                            u.range.start == n.range.start
                                && u.measure == m.position.0
                                && u.onset == m.onset
                                && u.duration == m.duration
                        })
                    });
                    if exact {
                        proposals.push(slots);
                    }
                }
            }
        }
        let mut rejected = BTreeSet::new();
        for (i, a) in proposals.iter().enumerate() {
            if a.iter()
                .enumerate()
                .any(|(n, s)| a.iter().skip(n + 1).any(|t| overlaps(&s.range, &t.range)))
            {
                rejected.insert(i);
            }
            for (j, b) in proposals.iter().enumerate().skip(i + 1) {
                if a.iter()
                    .any(|s| b.iter().any(|t| overlaps(&s.range, &t.range)))
                {
                    rejected.insert(i);
                    rejected.insert(j);
                }
            }
        }
        let groups = proposals.len() - rejected.len();
        let mut slots = proposals
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !rejected.contains(i))
            .flat_map(|(_, slots)| slots)
            .collect::<Vec<_>>();
        slots.sort_by_key(|s| s.range.start);
        let mut guard = self.clone();
        guard.cursor_exchange = Some(BoundCursorExchange {
            input: source.xml().to_owned(),
            origin: source.receipt.phrase_origin.clone(),
            canonical_source: self.source_locators.clone(),
            slots,
            groups,
        });
        Ok(guard)
    }
    pub(crate) fn has_cursor_exchange_for(&self, input: &str) -> bool {
        self.cursor_exchange.as_ref().is_some_and(|plan| plan.input == input)
    }

    pub(crate) fn cursor_exchange_edits(&self, input: &str) -> io::Result<CursorExchangeEdits> {
        let plan = self
            .cursor_exchange
            .as_ref()
            .ok_or_else(|| invalid("Cursor exchange has no bound plan"))?;
        if plan.input != input {
            return Err(invalid("Cursor exchange input is stale"));
        }
        Ok((
            plan.groups,
            plan.slots
                .iter()
                .map(|s| {
                    (
                        s.range.start,
                        s.range.end,
                        input[s.replacement.clone()].to_owned(),
                    )
                })
                .collect(),
        ))
    }
    pub(crate) fn carry_rest_bridge_cursor_exchange(
        &self,
        source: &FinalPdfSourceOccurrenceContinuation,
        output: &str,
        receipt: Option<&crate::master_state_identity::PreservingTransformationReceipt>,
        voices: &FrozenSerializedVoicePlan,
        omissions: &BTreeSet<RenderPrunedFinalChordOmissionKey>,
    ) -> io::Result<FinalPdfSourceOccurrenceContinuation> {
        let plan = self
            .cursor_exchange
            .as_ref()
            .ok_or_else(|| invalid("Cursor exchange has no transport plan"))?;
        if plan.input != source.xml() {
            return Err(invalid("Cursor exchange transport source is stale"));
        }
        if !std::sync::Arc::ptr_eq(&plan.origin, &source.receipt.phrase_origin)
            || !std::sync::Arc::ptr_eq(&plan.canonical_source, &self.source_locators)
        {
            return Err(invalid("Cursor exchange has foreign occurrence epoch"));
        }
        verify_final_pdf_source_occurrence_continuation(
            source.xml(),
            self,
            voices,
            Some(omissions),
            source,
        )?;
        if plan.slots.is_empty() {
            if output != source.xml() || receipt.is_some() {
                return Err(invalid("Unplanned cursor exchange"));
            }
            return Ok(source.clone());
        }
        receipt.ok_or_else(||invalid("Cursor exchange lacks its original E057 receipt"))?
            .validate_preserving_transition::<crate::pipeline::PartialBeatLaneTransferPreservingMint>(source.xml(),output)
            .map_err(|_|invalid("Cursor exchange receipt does not bind the actual operation"))?;
        plan.verify_bytes(output)?;
        let before = final_chord_parse_binding_measures(source.xml())?;
        let after = final_chord_parse_binding_measures(output)?;
        if before.len() != after.len() {
            return Err(invalid("Cursor exchange measure topology drift"));
        }
        let mut positions = BTreeMap::new();
        for (a, b) in before.iter().zip(&after) {
            if a.measure != b.measure || a.notes.len() != b.notes.len() {
                return Err(invalid("Cursor exchange note cardinality drift"));
            }
            let by_range = b
                .notes
                .iter()
                .enumerate()
                .map(|(i, n)| ((n.range.start, n.range.end), (i, n)))
                .collect::<BTreeMap<_, _>>();
            let mut next_ordinal = 0usize;
            let post_ordinals = b
                .notes
                .iter()
                .map(|n| {
                    let ordinal = next_ordinal;
                    if !n.rest_like && n.pitch_midi.is_some() {
                        next_ordinal += 1;
                    }
                    ordinal
                })
                .collect::<Vec<_>>();
            let mut used = BTreeSet::new();
            let mut pitched = 0;
            for old in &a.notes {
                let image = plan.image(&old.range)?;
                let (new_index, new) = by_range
                    .get(&(image.start, image.end))
                    .ok_or_else(|| invalid("Cursor exchange lost a note image"))?;
                if !used.insert(*new_index)
                    || old.onset != new.onset
                    || old.duration != new.duration
                    || old.staff != new.staff
                    || old.voice != new.voice
                    || old.chord_member != new.chord_member
                    || old.rest_like != new.rest_like
                    || source.xml()[old.range.clone()] != output[new.range.clone()]
                {
                    return Err(invalid("Cursor exchange changed a timed note resource"));
                }
                let old_anchor = old
                    .anchor_index
                    .map(|i| &a.notes[i].range)
                    .map(|r| plan.image(r))
                    .transpose()?;
                let new_anchor = new.anchor_index.map(|i| b.notes[i].range.clone());
                if old_anchor != new_anchor || old.orphan_after_forward != new.orphan_after_forward
                {
                    return Err(invalid("Cursor exchange changed chord attachment"));
                }
                if !old.rest_like && old.pitch_midi.is_some() {
                    let ordinal = post_ordinals[*new_index];
                    if positions
                        .insert((a.measure, pitched), (b.measure, ordinal))
                        .is_some()
                    {
                        return Err(invalid("Cursor exchange duplicate old position"));
                    }
                    pitched += 1;
                }
            }
        }
        if positions.values().collect::<BTreeSet<_>>().len() != positions.len() {
            return Err(invalid("Cursor exchange duplicate new position"));
        }
        let mut carried = source.receipt.clone();
        for row in &mut carried.occurrences {
            let mapped = positions
                .get(&(row.measure, row.pitched_ordinal))
                .ok_or_else(|| invalid("Cursor exchange omitted an occurrence"))?;
            row.measure = mapped.0;
            row.pitched_ordinal = mapped.1;
        }
        let mut beam_surface = BTreeMap::new();
        for (position, image) in &source.beam_surface {
            beam_surface.insert(
                *positions
                    .get(position)
                    .ok_or_else(|| invalid("Cursor exchange omitted a beam image"))?,
                image.clone(),
            );
        }
        if beam_surface != current_occurrence_beam_surface(output)? {
            return Err(invalid("Cursor exchange changed occurrence beam state"));
        }
        if let Some(course) = &mut carried.rest_bridge_extension_course {
            let post_notes = notes(output)?;
            for connection in &mut course.connections {
                for member in connection
                    .members
                    .iter_mut()
                    .chain(&mut connection.resources)
                {
                    member.position = *positions
                        .get(&member.position)
                        .ok_or_else(|| invalid("Cursor exchange omitted a connection resource"))?;
                    if !post_notes
                        .get(&member.position)
                        .is_some_and(|n| member.matches(output, n, false))
                    {
                        return Err(invalid("Cursor exchange changed live connection custody"));
                    }
                }
                if !attachments_match(connection, &post_notes) {
                    return Err(invalid("Cursor exchange lost an attached resource"));
                }
            }
            for connection in &mut course.authored {
                for member in &mut connection.members {
                    member.position = *positions
                        .get(&member.position)
                        .ok_or_else(|| invalid("Cursor exchange omitted an authored resource"))?;
                    if !post_notes
                        .get(&member.position)
                        .is_some_and(|n| member.matches(output, n, false))
                    {
                        return Err(invalid("Cursor exchange changed authored custody"));
                    }
                }
                if !resource_attachments_match(
                    &connection.members.iter().collect::<Vec<_>>(),
                    &post_notes,
                ) {
                    return Err(invalid("Cursor exchange lost authored attachment"));
                }
            }
            // Preserve the original withdrawal history. Only surviving live
            // connections move; an intentionally withdrawn origin stays retired.
            course.xml = output.to_owned();
        }
        carried.output_topology_fingerprint_fnv64 =
            final_chord_transaction_output_alpha_topology_fingerprint(&after);
        let result = FinalPdfSourceOccurrenceContinuation {
            xml: output.to_owned(),
            beam_surface,
            receipt: carried,
            ..source.clone()
        };
        verify_final_pdf_source_occurrence_continuation(
            output,
            self,
            voices,
            Some(omissions),
            &result,
        )?;
        Ok(result)
    }
}

#[cfg(test)]
impl CanonicalFinalChordStructuralPlan {
    /// Explicit synthetic adopted-input fixture. It does not claim an E050
    /// authoring integration run; production has no constructor for this course.
    pub(crate) fn synthetic_cursor_exchange_fixture(
        input: &str,
        groups: &[Vec<usize>],
    ) -> io::Result<(
        Self,
        FrozenSerializedVoicePlan,
        FinalPdfSourceOccurrenceContinuation,
    )> {
        let parsed = final_chord_parse_binding_measures(input)?;
        let labels = parsed
            .iter()
            .flat_map(|m| &m.notes)
            .filter(|n| !n.rest_like && n.pitch_midi.is_some())
            .map(|n| n.voice.as_str())
            .collect::<Vec<_>>();
        let (mut canonical, voices, mut source) =
            Self::synthetic_projected_bridge_fixture(input, &labels)?;
        let current = notes(source.xml())?;
        let rows = source
            .receipt
            .occurrences
            .iter()
            .map(|row| {
                (
                    (row.measure, row.pitched_ordinal),
                    (row.source_event_id, row.attack_id, row.fragment_id),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut connections = Vec::new();
        for (index, group) in groups.iter().enumerate() {
            let heads = current
                .iter()
                .filter(|(p, n)| !n.chord_member && group.contains(&rows[p].0))
                .map(|(p, _)| *p)
                .collect::<BTreeSet<_>>();
            if heads.len() != group.len() {
                return Err(invalid("Synthetic exchange group is not explicit"));
            }
            let attached = current
                .iter()
                .filter(|(_, n)| {
                    n.chord_member
                        && heads.iter().any(|p| {
                            current[p].measure == n.measure
                                && n.anchor_index == Some(current[p].raw_note_index)
                        })
                })
                .map(|(p, _)| *p)
                .collect::<BTreeSet<_>>();
            let mut members = Vec::new();
            let mut resources = Vec::new();
            for p in heads.iter().chain(&attached) {
                let n = &current[p];
                let identity = rows[p];
                let anchor = n.anchor_index.map(|raw| {
                    current
                        .iter()
                        .find(|(_, a)| a.measure == n.measure && a.raw_note_index == raw)
                        .map(|(p, _)| rows[p])
                        .unwrap()
                });
                let member = Member {
                    identity,
                    position: *p,
                    original: group.first() == Some(&identity.0),
                    anchor,
                    head: heads.contains(p),
                    onset: n.onset,
                    duration: n.duration,
                    pitch: n.pitch_midi,
                    staff: n.staff.clone(),
                    voice: n.voice.clone(),
                    sealed_voice: n.voice.clone(),
                    chord: n.chord_member,
                    tie_start: n.tie_start,
                    tie_stop: n.tie_stop,
                    before: Vec::new(),
                    written: beams(source.xml(), n)?,
                };
                if heads.contains(p) {
                    members.push(member);
                } else {
                    resources.push(member);
                }
            }
            connections.push(Connection {
                component: members
                    .iter()
                    .chain(&resources)
                    .map(|m| m.identity)
                    .collect(),
                members,
                resources,
                origin: (index, 0),
                provisional: false,
            });
        }
        let course = RestBridgeExtensionCourse {
            project: "synthetic adopted input".into(),
            branch: "main",
            xml: source.xml().to_owned(),
            source: canonical.source_locators.clone(),
            connections,
            authored: Vec::new(),
            provisional_origins: BTreeSet::new(),
        };
        source.receipt.rest_bridge_extension_course = Some(course.clone());
        canonical.rest_bridge_extension_course = Some(course);
        Ok((canonical, voices, source))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn input(ordinal: usize) -> String {
        let pitch = |step: &str, voice: &str, staff: usize, beam: &str| {
            format!(
                "<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>24</duration><voice>{voice}</voice><type>16th</type><staff>{staff}</staff>{beam}</note>"
            )
        };
        let mut body = String::new();
        for _ in 0..2 {
            body += &pitch("C", "lower", 2, "<beam number=\"1\">begin</beam>");
            body += &pitch("E", "upper", 1, "");
            body += &pitch("G", "lower", 2, "<beam number=\"1\">end</beam>");
            body += &pitch("E", "upper", 1, "");
        }
        let rest = "<note><rest/><duration>24</duration><voice>lower</voice><type>16th</type><staff>2</staff></note>";
        let forward =
            "<forward><duration>24</duration><voice>lower</voice><staff>2</staff></forward>";
        format!(
            "<score-partwise version=\"4.0\"><part-list><score-part id=\"P\"><part-name>Piano</part-name></score-part></part-list><part id=\"P\"><measure number=\"{ordinal}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>{body}<forward><duration>192</duration></forward><backup><duration>384</duration></backup>{forward}{rest}{forward}{rest}{forward}{rest}{forward}{rest}<note><rest/><duration>192</duration><voice>lower</voice><type>half</type><staff>2</staff></note></measure></part></score-partwise>"
        )
    }
    #[test]
    fn disjoint_same_measure_exchanges_survive_unknown_independent_placeholder() {
        let input = input(47);
        let groups = [vec![1, 3], vec![5, 7]];
        let (canonical, voices, source) =
            CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(&input, &groups)
                .unwrap();
        assert!(!canonical.has_cursor_exchange_for(source.xml()));
        let guard = canonical
            .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
            .unwrap();
        assert!(guard.has_cursor_exchange_for(source.xml()));
        assert!(!guard.has_cursor_exchange_for(&format!("{} ", source.xml())));
        assert_eq!(guard.cursor_exchange_edits(source.xml()).unwrap().0, 2);
        let altered = input.replacen(
            "<forward><duration>24</duration><voice>lower</voice>",
            "<forward><duration>24</duration><voice>foreign</voice>",
            1,
        );
        let (canonical, voices, source) =
            CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(&altered, &groups)
                .unwrap();
        let guard = canonical
            .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
            .unwrap();
        assert_eq!(guard.cursor_exchange_edits(source.xml()).unwrap().0, 1);
    }
    #[test]
    fn shared_exchange_resources_veto_only_their_conflict_component() {
        let (canonical, voices, source) =
            CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(
                &input(47),
                &[vec![1, 3], vec![1, 3], vec![5, 7]],
            )
            .unwrap();
        let guard = canonical
            .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
            .unwrap();
        assert_eq!(guard.cursor_exchange_edits(source.xml()).unwrap().0, 1);
    }
    #[test]
    fn unsafe_cursor_slots_decline_locally_without_disabling_independent_group() {
        let xml = input(67);
        let forward =
            "<forward><duration>24</duration><voice>lower</voice><staff>2</staff></forward>";
        let rest = "<note><rest/><duration>24</duration><voice>lower</voice><type>16th</type><staff>2</staff></note>";
        for replacement in [
            forward.replace("<duration>24</duration>", "<duration>12</duration>"),
            forward.replace("<staff>2</staff>", "<staff>1</staff>"),
            forward.replace("<voice>lower</voice>", ""),
            format!(
                "{forward}<note><chord/><pitch><step>B</step><octave>3</octave></pitch><duration>24</duration><voice>lower</voice><staff>2</staff></note>"
            ),
        ] {
            let altered = xml.replacen(forward, &replacement, 1);
            let (canonical, voices, source) =
                CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(
                    &altered,
                    &[vec![1, 3]],
                )
                .unwrap();
            let guard = canonical
                .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
                .unwrap();
            assert_eq!(guard.cursor_exchange_edits(source.xml()).unwrap().0, 0);
        }
        for replacement in [
            rest.replace("<note>", "<note print-object=\"no\">"),
            rest.replace("<voice>lower</voice>", "<voice>foreign</voice>"),
        ] {
            let altered = xml.replacen(rest, &replacement, 1);
            let (canonical, voices, source) =
                CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(
                    &altered,
                    &[vec![1, 3], vec![5, 7]],
                )
                .unwrap();
            let guard = canonical
                .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
                .unwrap();
            assert_eq!(guard.cursor_exchange_edits(source.xml()).unwrap().0, 1);
        }
    }
    #[test]
    fn unchanged_endpoint_tags_cannot_replace_absent_adopted_custody() {
        let (canonical, voices, mut source) =
            CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(
                &input(47),
                &[vec![1, 3]],
            )
            .unwrap();
        source.receipt.rest_bridge_extension_course = None;
        let guard = canonical
            .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
            .unwrap();
        assert!(
            guard
                .cursor_exchange_edits(source.xml())
                .unwrap()
                .1
                .is_empty()
        );
    }
    #[test]
    fn equal_xml_cannot_replace_the_original_projection_epoch() {
        let input = input(47);
        let (canonical, voices, source) =
            CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(
                &input,
                &[vec![1, 3]],
            )
            .unwrap();
        let (_, _, foreign) = CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(
            &input,
            &[vec![1, 3]],
        )
        .unwrap();
        assert_eq!(source.xml(), foreign.xml());
        let guard = canonical
            .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
            .unwrap();
        let error = guard
            .carry_rest_bridge_cursor_exchange(
                &foreign,
                foreign.xml(),
                None,
                &voices,
                &BTreeSet::new(),
            )
            .unwrap_err();
        assert!(error.to_string().contains("foreign occurrence epoch"));
    }
}

#[cfg(test)]
mod authored_custody_tests {
    use super::*;
    #[test]
    fn authored_custody_retires_on_resource_drift_and_never_reappears() {
        let xml = super::tests::input(61);
        let (canonical, voices, mut source) =
            CanonicalFinalChordStructuralPlan::synthetic_cursor_exchange_fixture(
                &xml,
                &[vec![1, 3], vec![5, 7]],
            )
            .unwrap();
        let course = source
            .receipt
            .rest_bridge_extension_course
            .as_mut()
            .unwrap();
        course.authored = course
            .connections
            .drain(..)
            .map(|connection| AuthoredConnection {
                members: connection.all().cloned().collect(),
            })
            .collect();
        let guard = canonical
            .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
            .unwrap();
        assert_eq!(guard.cursor_exchange_edits(&xml).unwrap().0, 2);
        let course = source
            .receipt
            .rest_bridge_extension_course
            .as_mut()
            .unwrap();
        let drift = xml.replacen(">begin</beam>", ">continue</beam>", 1);
        course.through(&xml, &drift).unwrap();
        assert_eq!(course.authored.len(), 1);
        course.through(&drift, &xml).unwrap();
        assert_eq!(course.authored.len(), 1);
        assert!(course.through("stale", &xml).is_err());
        assert!(
            course
                .projected(&xml, &xml, &source.receipt.occurrences)
                .is_err()
        );
        let guard = canonical
            .bind_rest_bridge_cursor_exchange(&source, &voices, &BTreeSet::new())
            .unwrap();
        assert_eq!(guard.cursor_exchange_edits(&xml).unwrap().0, 1);
    }
}

#[cfg(test)]
impl FinalPdfSourceOccurrenceContinuation {
    /// Explicitly declared synthetic new-connection custody. Production can
    /// obtain this variant only from the original E050 execution and adoption.
    pub(crate) fn synthetic_authored_cursor_connections(mut self) -> Self {
        let course = self
            .receipt
            .rest_bridge_extension_course
            .as_mut()
            .expect("synthetic connection course");
        course.authored = course
            .connections
            .drain(..)
            .map(|connection| AuthoredConnection {
                members: connection.all().cloned().collect(),
            })
            .collect();
        self
    }
}
