//! Exact continuation of received course obligations. This supplies neither a
//! canonical pitched-role decision nor an XML writer/channel-allocation permit.
use super::*;
use crate::arrangement::physical_births::{PhysicalBirthKind, PhysicalRhythmicBirths};
use crate::master_state_identity::{ExactTransformationReceipt};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
#[path = "owned_rhythmic_consolidation.rs"]
mod consolidation;
use consolidation::ConsolidatedOrigin;
#[path = "owned_rhythmic_final_export.rs"]
mod final_export;
use final_export::EncodedOccurrence;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Origin {
    parent: Option<RhythmicCreationIdentity>,
    root: RhythmicCreationIdentity,
    course: Option<RhythmicCreationIdentity>,
    source: Option<usize>,
    interval: Range<i64>,
    // The role is the generator's positively authored course presentation.
    // It never substitutes for the independently decided canonical source role.
    presentation: Option<SeedCoursePresentation>,
    birth: Option<PhysicalBirthKind>,
    // Explicit course transport of an authorized ConsensusGesture relocation.
    relocation: Option<Relocation>,
}

/// The only edge at which course and presentation may change: an actual row 26
/// relocation onto a carried anchor. The moved occurrence keeps root and source;
/// its obligation over `vacated` is transported with it, never refilled by a rest.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Relocation {
    from_course: Option<RhythmicCreationIdentity>,
    from_presentation: Option<SeedCoursePresentation>,
    anchor: RhythmicCreationIdentity,
    vacated: Range<i64>,
}

/// Sounding MIDI number from step, alter and octave; spelling is not identity
/// (enharmonic_occurrence_identity_v1).
fn sounding_midi(note: &str) -> Option<i64> {
    let step = match extract_first_text(note, "step")?.trim() {
        "C" => 0, "D" => 2, "E" => 4, "F" => 5, "G" => 7, "A" => 9, "B" => 11, _ => return None,
    };
    let alter = extract_first_text(note, "alter").map_or(Some(0), |a| a.trim().parse::<i64>().ok())?;
    let octave = extract_first_text(note, "octave")?.trim().parse::<i64>().ok()?;
    Some((octave + 1) * 12 + step + alter)
}

#[derive(Debug, Clone)]
struct Occurrence {
    creation: RhythmicCreationIdentity,
    origin: Origin,
    staff: u8,
    bytes: Range<usize>,
}

/// Created solely from authenticated initial custody and transported through
/// original owner operations. No constructor accepts XML labels as identity.
#[derive(Clone)]
pub(crate) struct OwnedRhythmicContinuation {
    snapshot: Arc<str>,
    initial_custody: Arc<ArrangementRhythmicCourses>,
    coverage: Vec<SeedArrangementCourse>,
    history: BTreeMap<RhythmicCreationIdentity, Origin>,
    occurrences: Vec<Occurrence>,
    encoded: Vec<EncodedOccurrence>,
    final_chord_execution: Option<crate::master_state_identity::MintedIdentity>,
    consolidations: BTreeMap<RhythmicCreationIdentity, ConsolidatedOrigin>,
    validated: ValidatedMemo,
}

/// Success belongs only to this immutable state, never to a cloned successor.
/// Relaxed is sufficient: this flag publishes no other writes. All state is
/// constructed before shared validation; simultaneous misses may both validate.
#[derive(Default)]
struct ValidatedMemo(AtomicBool);

impl Clone for ValidatedMemo {
    fn clone(&self) -> Self { Self::default() }
}

// Preserve the previous diagnostic representation; memo state is not custody.
impl std::fmt::Debug for OwnedRhythmicContinuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OwnedRhythmicContinuation")
            .field("snapshot", &self.snapshot)
            .field("initial_custody", &self.initial_custody)
            .field("coverage", &self.coverage)
            .field("history", &self.history)
            .field("occurrences", &self.occurrences)
            .field("encoded", &self.encoded)
            .field("final_chord_execution", &self.final_chord_execution)
            .field("consolidations", &self.consolidations)
            .finish()
    }
}

#[derive(Debug)]
struct PhysicalOccurrence {
    bytes: Range<usize>,
    interval: Range<i64>,
    staff: u8,
    text: String,
}

fn physical_inventory(xml: &str) -> Result<Vec<PhysicalOccurrence>> {
    let mut offset = 0i64;
    let mut context = RhythmContext::default();
    let mut inventory = Vec::new();
    for span in find_measure_spans(xml) {
        let measure = &xml[span.start..span.end];
        context = context.update_from_measure(measure);
        let notes = collect_raw_notes(measure);
        let metas = collect_note_metas(measure, &notes);
        ensure!(notes.len() == metas.len(), "Incomplete rhythmic physical readback");
        for (note, meta) in notes.into_iter().zip(metas) {
            let staff = extract_first_i64(&note.text, "staff");
            ensure!(matches!(staff, Some(1 | 2)) && meta.duration > 0,
                "Unsupported explicit staff or rhythmic duration");
            inventory.push(PhysicalOccurrence {
                bytes: span.start + note.start..span.start + note.end,
                interval: offset + meta.start..offset + meta.start + meta.duration,
                staff: staff.unwrap() as u8, text: note.text,
            });
        }
        offset = offset.checked_add(context.expected_duration())
            .ok_or_else(|| anyhow!("Rhythmic domain overflow"))?;
    }
    ensure!(inventory.len() == collect_raw_notes(xml).len(), "Physical readback omitted an occurrence");
    Ok(inventory)
}

fn same_pitch_and_disposition(before: &str, after: &str) -> bool {
    extract_first_text(before, "pitch") == extract_first_text(after, "pitch")
        && before.contains("<rest") == after.contains("<rest")
        && before.contains("<chord") == after.contains("<chord")
        && before.contains("<grace") == after.contains("<grace")
        && tie_marks(before) == tie_marks(after)
}

/// Used only by the original #60 receipt path after complete-chain validation.
fn same_enharmonic_disposition(before: &str, after: &str) -> bool {
    sounding_midi(before).is_some() && sounding_midi(before) == sounding_midi(after)
        && before.contains("<rest") == after.contains("<rest")
        && before.contains("<chord") == after.contains("<chord")
        && before.contains("<grace") == after.contains("<grace")
        && tie_marks(before) == tie_marks(after)
}

fn tie_marks(note: &str) -> Vec<String> {
    marks_named(note, &["tie", "tied"])
}

fn marks_named(note: &str, names: &[&str]) -> Vec<String> {
    let Ok(doc) = roxmltree::Document::parse(note) else { return vec!["invalid".into()]; };
    doc.descendants().filter(|n| n.is_element() && names.contains(&n.tag_name().name()))
        .map(|n| format!("{}:{:?}", n.tag_name().name(), n.attribute("type"))).collect()
}

/// Disposition after an original notation-only endpoint cleanup: pitch, rest,
/// chord, grace and every sounding `<tie>` stay; only `<tied>` may disappear.
fn same_sounding_disposition(before: &str, after: &str) -> bool {
    extract_first_text(before, "pitch") == extract_first_text(after, "pitch")
        && before.contains("<rest") == after.contains("<rest")
        && before.contains("<chord") == after.contains("<chord")
        && before.contains("<grace") == after.contains("<grace")
        && marks_named(before, &["tie"]) == marks_named(after, &["tie"])
}

impl OwnedRhythmicContinuation {
    #[cfg(test)]
    pub(crate) fn synthetic_source_received_for_test(rows: &[(usize, i64, i64, u8, i32)], count: usize) -> Self {
        let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_source_lengths_fixture(rows, count);
        let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
        Self::receive(&custody, &xml).unwrap()
    }

    #[cfg(test)]
    pub(crate) fn synthetic_received_for_test() -> Self {
        let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_fixture(&[(71, 0, 1)], 1);
        let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
        Self::receive(&custody, &xml).unwrap()
    }

    #[cfg(test)]
    pub(crate) fn synthetic_pitched_received_for_test(rows: &[(usize, i64, u8, i32)], measures: usize) -> Self {
        let (xml, seed) = crate::midi_seed::rhythmic_course_creation_tests::arrangement_pitched_fixture(rows, measures);
        let custody = ArrangementRhythmicCourses::receive(&seed, &xml).unwrap();
        Self::receive(&custody, &xml).unwrap()
    }

    pub(crate) fn receive(courses: &ArrangementRhythmicCourses, xml: &str) -> Result<Self> {
        courses.validate_binding(xml)?;
        let physical = physical_inventory(xml)?;
        ensure!(physical.len() == courses.occurrences.len(), "Incomplete received course custody");
        let history = courses.history.iter().map(|(&id, edge)| {
            let course = courses.coverage.iter().find(|c| c.creation == edge.course)
                .ok_or_else(|| anyhow!("Missing original course authority"))?;
            Ok((id, Origin { parent: edge.parent, root: edge.root,
                course: Some(edge.course), source: edge.source, interval: edge.interval.clone(),
                presentation: Some(course.presentation), birth: None, relocation: None }))
        }).collect::<Result<BTreeMap<_, _>>>()?;
        let occurrences = courses.occurrences.iter().zip(physical).map(|(o, p)| {
            ensure!(p.interval == o.interval && p.text == o.note, "Received physical custody drift");
            let origin = history.get(&o.creation).cloned().ok_or_else(|| anyhow!("Missing original occurrence ancestry"))?;
            Ok(Occurrence { creation: o.creation, bytes: p.bytes, staff: p.staff, origin })
        }).collect::<Result<Vec<_>>>()?;
        let state = Self { snapshot: Arc::from(xml), initial_custody: Arc::new(courses.clone()), coverage: courses.coverage.clone(), history, occurrences, encoded: Vec::new(), final_chord_execution: None, consolidations: BTreeMap::new(), validated: ValidatedMemo::default() };
        state.validate_binding(xml)?;
        Ok(state)
    }

    pub(crate) fn document(&self) -> &str { &self.snapshot }

    pub(crate) fn validate_binding(&self, xml: &str) -> Result<()> {
        if self.validated.0.load(Ordering::Relaxed) {
            ensure!(self.snapshot.as_ref() == xml, "Owned rhythm belongs to another document state");
            return Ok(());
        }
        self.validate_consolidated_origins()?;
        ensure!(self.snapshot.as_ref() == xml, "Owned rhythm belongs to another document state");
        // receive validates the original custody before storing this immutable Arc.
        // Current ancestry, coverage and representation checks remain independent.
        let physical = physical_inventory(xml)?;
        ensure!(physical.len() == self.occurrences.len() + self.encoded_note_count(), "Owned rhythm has incomplete physical coverage");
        let by_range = physical.iter().map(|p| ((p.bytes.start, p.bytes.end), p)).collect::<BTreeMap<_, _>>();
        let mut identities = BTreeSet::new(); let mut ranges = BTreeSet::new();
        for occurrence in &self.occurrences {
            let origin = &occurrence.origin;
            ensure!(identities.insert(occurrence.creation) && ranges.insert((occurrence.bytes.start, occurrence.bytes.end)),
                "Duplicate owned rhythmic identity or occurrence");
            ensure!(self.history.get(&occurrence.creation) == Some(origin), "Missing owner-bound ancestry");
            let physical = by_range.get(&(occurrence.bytes.start, occurrence.bytes.end))
                .ok_or_else(|| anyhow!("Owner continuation is not an entire physical note"))?;
            ensure!(physical.interval == origin.interval && physical.staff == occurrence.staff,
                "Owner continuation changed licensed support or presentation without a transition");
            self.validate_received_origin_path(occurrence.creation)?;
        }
        self.validate_encoded(xml, &mut ranges, &mut identities)?;
        // The union, rather than an interval hull, must cover each received
        // obligation. A transfer explicitly represents both its rest and pitch.
        let support = self.support_by_root()?;
        for (id, root) in self.history.iter().filter(|(_, o)| o.parent.is_none()) {
            let mut intervals = support.get(id).cloned().unwrap_or_default();
            intervals.sort_by_key(|r| (r.start, r.end));
            let mut cursor = root.interval.start;
            for span in intervals {
                ensure!(span.start <= cursor && span.end <= root.interval.end, "Lost or extended origin support");
                cursor = cursor.max(span.end);
            }
            ensure!(cursor == root.interval.end, "Owner continuation lost a received obligation");
        }
        self.validated.0.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Missing context remains observable. It supplies no mutation permission.
    pub(crate) fn unbound_context_count(&self) -> usize {
        self.occurrences.iter().chain(self.encoded.iter().map(|encoded| &encoded.received)).filter(|o| o.origin.course.is_none() || o.origin.presentation.is_none()).count()
    }

    pub(crate) fn require_complete_course_context(&self, xml: &str) -> Result<()> {
        self.validate_binding(xml)?;
        ensure!(self.unbound_context_count() == 0, "Owned rhythm has unbound course/role context");
        let vacancies = self.relocation_vacancy_count();
        ensure!(vacancies == 0, "Owned rhythm has {vacancies} unrepresented relocation vacancy interval(s)");
        Ok(())
    }

    /// The span an occurrence answers for in its root obligation: after an explicit
    /// relocation edge this is the transported `vacated` span closest to the root.
    fn obligation_span(&self, creation: RhythmicCreationIdentity, own: &Range<i64>) -> Range<i64> {
        let mut span = own.clone(); let mut id = Some(creation); let mut visited = BTreeSet::new();
        while let Some(current) = id {
            let Some(edge) = self.history.get(&current) else { break };
            if !visited.insert(current) { break; }
            if let Some(relocation) = &edge.relocation { span = relocation.vacated.clone(); }
            id = edge.parent;
        }
        span
    }

    /// Measured only: vacated spans that no carried occurrence of the former course
    /// represents any more. Never closed, filled or licensed here.
    pub(crate) fn relocation_vacancy_count(&self) -> usize {
        self.history.values().filter_map(|o| o.relocation.as_ref()).filter(|relocation| {
            let mut spans = self.occurrences.iter().chain(self.encoded.iter().map(|encoded| &encoded.received)).filter(|o| relocation.from_course.is_some()
                && o.origin.course == relocation.from_course).map(|o| o.origin.interval.clone()).collect::<Vec<_>>();
            spans.sort_by_key(|r| (r.start, r.end));
            let mut cursor = relocation.vacated.start;
            for span in spans {
                if span.start <= cursor && span.end > cursor { cursor = span.end; }
            }
            cursor < relocation.vacated.end
        }).count()
    }

    /// The frozen note-free additive mint proves monotone insertion. Therefore
    /// the original complete-note order carries identities, including identical
    /// coincident notes; text equality alone never creates an identity.
    pub(crate) fn continue_additive(&self,
        receipt: &crate::master_state_identity::AdditiveTransformationReceipt) -> Result<Self> {
        let (pre, post) = crate::master_state_identity::additive_original_documents(receipt)?;
        ensure!(pre == self.snapshot.as_ref(), "Original additive capture belongs to another state");
        let old = roxmltree::Document::parse(&self.snapshot)?;
        let new = roxmltree::Document::parse(&post)?;
        let elements = |doc: &roxmltree::Document<'_>| doc.descendants()
            .filter(|node| node.is_element() && matches!(node.tag_name().name(), "note" | "forward" | "backup"))
            .map(|node| node.range()).collect::<Vec<_>>();
        let input = elements(&old);
        let output = elements(&new);
        ensure!(output.len() == input.len(), "Additive owner changed complete rhythmic coverage");
        let mut carried = BTreeMap::new();
        for (old, new) in input.into_iter().zip(output) {
            ensure!(self.snapshot[old.clone()] == post[new.clone()],
                "Additive owner changed a complete rhythmic element");
            carried.insert((old.start, old.end), new);
        }
        self.continue_operations(&post, None, None, false, |bytes| {
            carried.get(&(bytes.start, bytes.end)).cloned()
                .ok_or_else(|| anyhow!("Missing original additive note ordinal"))
        }, |_| false)
    }

    pub(crate) fn continue_final_layout(&self,
        transport: &crate::structural_write_authority::FinalLayoutCourseTransport) -> Result<Self> {
        let (before, after) = transport.documents();
        ensure!(before == self.snapshot.as_ref(), "Foreign E007 original copy pre-state");
        self.continue_operations(after, None, None, false, |bytes| transport.carry(bytes), |_| false)
    }

    fn current_complete_ranges(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        self.occurrences.iter().map(|occurrence| occurrence.bytes.clone())
            .chain(self.encoded.iter().flat_map(|encoded| encoded.complete_ranges().cloned()))
    }

    pub(crate) fn continue_representation(&self, xml: &str,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport) -> Result<Self> {
        self.validate_binding(&self.snapshot)?;
        let ranges = self.current_complete_ranges().collect::<Vec<_>>();
        let carried = transport.carry(&self.snapshot, xml, ranges.clone())?;
        let map = ranges.into_iter().zip(carried).map(|(input, output)|
            ((input.start, input.end), output)).collect::<BTreeMap<_, _>>();
        self.continue_operations(xml, None, None, false, |bytes| {
            map.get(&(bytes.start, bytes.end)).cloned()
                .ok_or_else(|| anyhow!("Missing original representation occurrence range"))
        }, |_| false)
    }

    pub(crate) fn continue_preserving_representation(&self,
        receipt: &crate::master_state_identity::PreservingTransformationReceipt,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport) -> Result<Self> {
        let (before, after) = transport.recorded_documents()?;
        receipt.binds_recorded_representation(before, after)?;
        ensure!(before == self.snapshot.as_ref(), "Foreign preserving representation pre-state");
        self.continue_representation(after, transport)
    }

    pub(crate) fn continue_recorded_block(&self,
        transport: &crate::arrangement::representation_transport::RhythmicRepresentationTransport) -> Result<Self> {
        let (before, after) = transport.single_step_documents()?;
        ensure!(before == self.snapshot.as_ref(), "Foreign recorded block pre-state");
        let next = self.continue_representation(after, transport)?;
        let before_ranges = self.current_complete_ranges().collect::<Vec<_>>();
        let after_ranges = next.current_complete_ranges().collect::<Vec<_>>();
        ensure!(before_ranges.len() == after_ranges.len(), "Recorded block lost an original occurrence");
        for (before, after) in before_ranges.into_iter().zip(after_ranges) {
            ensure!(self.snapshot[before] == next.snapshot[after],
                "Recorded notation block changed a complete note");
        }
        Ok(next)
    }

    pub(crate) fn continue_exact(&self, receipt: &ExactTransformationReceipt,
        births: Option<&PhysicalRhythmicBirths>) -> Result<Self> {
        if let Some(capture) = receipt.final_chord_capture() {
            return self.continue_final_chord(receipt, capture.as_ref()
                .map_err(|error| anyhow!("Original FinalChord capture refused: {error}"))?);
        }
        let readback = receipt.operation_readback(&self.snapshot)?;
        if let Some(births) = births { births.validate_exact_execution(receipt)?; }
        if let Some(transition) = receipt.staff_transition() { transition.validate_exact_execution(receipt)?; }
        // Only the frozen allowed-range removal owner may carry a notation-only
        // endpoint cleanup; its original edits, not the output, prove the scope.
        let endpoint_cleanup = receipt.minted_by::<crate::structural_write_authority::AllowedRangeRemovalExactMint>();
        let respelling = receipt.minted_by::<crate::pipeline::FunctionalRespellingExactMint>();
        if respelling { self.validate_original_respelling_chains(receipt)?; }
        self.continue_operations(readback.post_document(), births, receipt.staff_transition(), respelling,
            |bytes| readback.carry_range(bytes),
            |bytes| endpoint_cleanup && readback.notation_endpoint_cleanup_within(bytes))
    }

    pub(crate) fn continue_visible(&self, births: &PhysicalRhythmicBirths) -> Result<Self> {
        births.validate_binding(&self.snapshot, births.post_document())?;
        self.continue_operations(births.post_document(), Some(births), None, false, |bytes| {
            births.carried().iter().find(|(input, _)| *input == bytes).map(|(_, output)| output.clone())
                .ok_or_else(|| anyhow!("Missing actual retained normalizer emission"))
        }, |_| false)
    }

    fn continue_operations(&self, after: &str, births: Option<&PhysicalRhythmicBirths>,
        staff_transition: Option<&crate::arrangement::physical_births::StaffPresentationTransition>,
        respelling: bool,
        carry: impl Fn(Range<usize>) -> Result<Range<usize>>,
        notation_cleanup: impl Fn(&Range<usize>) -> bool) -> Result<Self> {
        self.validate_binding(&self.snapshot)?;
        let physical = physical_inventory(after)?;
        let mut output = physical.iter().map(|p| ((p.bytes.start, p.bytes.end), p)).collect::<BTreeMap<_, _>>();
        let mut next = self.clone(); next.occurrences.clear();
        self.carry_encoded(&mut next, after, &carry, respelling)?;
        for encoded in &next.encoded {
            for bytes in encoded.note_ranges() {
                ensure!(output.remove(&(bytes.start, bytes.end)).is_some(), "Encoded output disappeared");
            }
        }
        let records = births.map_or(&[][..], PhysicalRhythmicBirths::records);
        let consolidated = self.continue_consolidations(
            births.map_or(&[][..], PhysicalRhythmicBirths::consolidations), &mut output, &mut next)?;
        for old in &self.occurrences {
            if consolidated.contains(&(old.bytes.start, old.bytes.end)) { continue; }
            let children = records.iter().filter(|b| b.input.as_ref() == Some(&old.bytes)).collect::<Vec<_>>();
            if children.is_empty() {
                let bytes = carry(old.bytes.clone())?;
                let p = output.remove(&(bytes.start, bytes.end)).ok_or_else(|| anyhow!("Retained occurrence is not one complete note"))?;
                if p.interval != old.origin.interval {
                    let emission = births.and_then(|capture| capture.original_retained_chord_copy(
                        &self.snapshot, after, &old.bytes, &bytes));
                    ensure!(old.origin.source.is_some() && p.staff == old.staff && emission.is_some(),
                        "Retained occurrence changed licensed support without an original copy relation: received {:?}, observed {:?}", old.origin.interval, p.interval);
                    Self::retain_original_visible_copy(&mut next, old, p, emission.unwrap())?;
                    continue;
                }
                ensure!(p.staff == old.staff || (old.origin.source.is_some()
                    && self.snapshot[old.bytes.clone()].contains("<pitch>")
                    && staff_transition.is_some_and(|transition|
                        transition.binds(&self.snapshot, after, &old.bytes, &bytes, p.staff))),
                    "Retained occurrence changed staff without a course transition: received {}, observed {}", old.staff, p.staff);
                let old_text = &self.snapshot[old.bytes.clone()];
                ensure!(same_pitch_and_disposition(old_text, &p.text)
                    || (respelling && same_enharmonic_disposition(old_text, &p.text))
                    || (notation_cleanup(&old.bytes) && same_sounding_disposition(old_text, &p.text)),
                    "Retained occurrence changed pitch/rest/chord/tie disposition without a transition");
                let mut retained = old.clone(); retained.bytes = bytes; retained.staff = p.staff; next.occurrences.push(retained);
                continue;
            }
            let kinds = children.iter().map(|b| b.kind).collect::<Vec<_>>();
            ensure!(kinds.iter().all(|k| *k == PhysicalBirthKind::MetricFragment)
                || kinds == [PhysicalBirthKind::Restified]
                || kinds == [PhysicalBirthKind::Restified, PhysicalBirthKind::Transferred]
                || kinds == [PhysicalBirthKind::GestureRelocated],
                "Unsupported physical birth family");
            let mut fragments = Vec::new();
            for child in children {
                let p = output.remove(&(child.output.start, child.output.end))
                    .ok_or_else(|| anyhow!("Birth does not bind one unique complete output"))?;
                let old_text = &self.snapshot[old.bytes.clone()];
                match child.kind {
                    PhysicalBirthKind::Restified => ensure!(p.text.contains("<rest") && old_text.contains("<pitch>")
                        && p.interval == old.origin.interval && p.staff == old.staff,
                        "Restification changed licensed support or staff"),
                    PhysicalBirthKind::Transferred => ensure!(old_text.contains("<pitch>")
                        && extract_first_text(old_text, "pitch") == extract_first_text(&p.text, "pitch")
                        && p.interval == old.origin.interval && old.origin.source.is_some(),
                        "Transfer lacks received source/support preservation"),
                    PhysicalBirthKind::MetricFragment => {
                        ensure!(p.staff == old.staff
                            && extract_first_text(old_text, "pitch") == extract_first_text(&p.text, "pitch")
                            && old_text.contains("<rest") == p.text.contains("<rest")
                            && old_text.contains("<chord") == p.text.contains("<chord")
                            && !old_text.contains("<tie") && !old_text.contains("<grace") && !old_text.contains("<notations"),
                            "Metric split lacks disposition or existing-tie continuation");
                        fragments.push((p.interval.clone(), p.text.clone()));
                    },
                    PhysicalBirthKind::GestureRelocated => {},
                    PhysicalBirthKind::GapRest | PhysicalBirthKind::RestConsolidated => unreachable!(),
                }
                let mut origin = old.origin.clone(); origin.parent = Some(old.creation);
                origin.interval = p.interval.clone(); origin.birth = Some(child.kind); origin.relocation = None;
                if child.kind == PhysicalBirthKind::GestureRelocated {
                    // The anchor is found by its carried bytes, never by staff or time.
                    let anchor_bytes = child.anchor.clone().ok_or_else(|| anyhow!("Relocation without captured anchor"))?;
                    let anchor = self.occurrences.iter().find(|o| o.bytes == anchor_bytes)
                        .ok_or_else(|| anyhow!("Relocation anchor is not a carried occurrence"))?;
                    ensure!(anchor.origin.course.is_some() && anchor.origin.presentation.is_some(),
                        "Relocation anchor has no explicit course");
                    ensure!(old_text.contains("<pitch>") && old.origin.source.is_some()
                        && !old_text.contains("<tie") && !old_text.contains("<grace"),
                        "Relocation lacks a source-backed untied pitched occurrence");
                    ensure!(sounding_midi(old_text).is_some() && sounding_midi(old_text) == sounding_midi(&p.text),
                        "Relocation changed sounding pitch");
                    ensure!(p.text.contains("<chord") && p.staff == anchor.staff && p.interval == anchor.origin.interval,
                        "Relocated occurrence is not a chord member of its anchor: observed staff {} {:?}, anchor staff {} {:?}",
                        p.staff, p.interval, anchor.staff, anchor.origin.interval);
                    origin.relocation = Some(Relocation { from_course: old.origin.course,
                        from_presentation: old.origin.presentation, anchor: anchor.creation,
                        vacated: old.origin.interval.clone() });
                    origin.course = anchor.origin.course; origin.presentation = anchor.origin.presentation;
                }
                ensure!(next.history.insert(child.creation, origin.clone()).is_none(), "Reused birth identity");
                next.occurrences.push(Occurrence { creation: child.creation, origin, staff: p.staff, bytes: child.output.clone() });
            }
            if !fragments.is_empty() {
                fragments.sort_by_key(|(span, _)| (span.start, span.end));
                let mut end = old.origin.interval.start;
                for (index, (span, text)) in fragments.iter().enumerate() {
                    ensure!(span.start == end && span.end > end, "Split lost or duplicated source support");
                    if text.contains("<pitch>") {
                        let marks = tie_marks(text);
                        let mut expected: Vec<String> = Vec::new();
                        if index + 1 < fragments.len() { expected.extend(["tie:Some(\"start\")".into(), "tied:Some(\"start\")".into()]); }
                        if index > 0 { expected.extend(["tie:Some(\"stop\")".into(), "tied:Some(\"stop\")".into()]); }
                        let mut actual = marks; actual.sort(); expected.sort();
                        ensure!(actual == expected, "Split changed attack/tie continuity");
                    }
                    end = span.end;
                }
                ensure!(end == old.origin.interval.end, "Split changed received support");
            }
        }
        for birth in records.iter().filter(|b| b.input.is_none()) {
            ensure!(birth.kind == PhysicalBirthKind::GapRest, "Unexplained originless birth");
            let p = output.remove(&(birth.output.start, birth.output.end)).ok_or_else(|| anyhow!("Unbound birth coverage drift"))?;
            let origin = Origin { parent: None, root: birth.creation, course: None, source: None,
                interval: p.interval.clone(), presentation: None, birth: Some(birth.kind), relocation: None };
            ensure!(next.history.insert(birth.creation, origin.clone()).is_none(), "Reused unbound birth identity");
            next.occurrences.push(Occurrence { creation: birth.creation, origin, staff: p.staff, bytes: birth.output.clone() });
        }
        ensure!(output.is_empty(), "Output contains a note without actual birth/continuation evidence");
        next.occurrences.sort_by_key(|o| o.bytes.start);
        next.snapshot = Arc::from(after); next.validate_binding(after)?; Ok(next)
    }

    #[cfg(test)]
    pub(crate) fn same_identities_as(&self, other: &Self) -> bool {
        self.occurrences.iter().chain(self.encoded.iter().map(|encoded| &encoded.received)).map(|o| o.creation)
            .eq(other.occurrences.iter().chain(other.encoded.iter().map(|encoded| &encoded.received)).map(|o| o.creation))
            && self.history == other.history
    }

    #[cfg(test)]
    pub(crate) fn born_from(&self, parent: &Self) -> usize {
        self.occurrences.iter().filter(|o| o.origin.parent.is_some_and(|id| parent.history.contains_key(&id))).count()
    }
}

/// Read-only census of one adopted document transition, measured against the
/// physical continuation rules above. Pure observation: it issues no identity,
/// continues no custody and is never consulted by an owner or the lineage.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CustodyCensusCounts {
    pub(crate) unchanged: usize,
    pub(crate) text_only: usize,
    pub(crate) staff_moved: usize,
    pub(crate) disposition_changed: usize,
    pub(crate) removed: usize,
    pub(crate) added: usize,
}

type CensusDisposition = (Option<String>, bool, bool, bool, Vec<String>);

fn census_disposition(text: &str) -> CensusDisposition {
    (extract_first_text(text, "pitch"), text.contains("<rest"), text.contains("<chord"),
        text.contains("<grace"), tie_marks(text))
}

/// Pairs equal keys once, removes the paired entries from both sides and returns
/// the number of pairs. Pairing order is stable and carries no meaning.
fn census_pair<K: Ord>(before: &mut Vec<&PhysicalOccurrence>, after: &mut Vec<&PhysicalOccurrence>,
    key: impl Fn(&PhysicalOccurrence) -> K) -> usize {
    let mut pool: BTreeMap<K, Vec<usize>> = BTreeMap::new();
    for (index, item) in after.iter().enumerate().rev() {
        pool.entry(key(item)).or_default().push(index);
    }
    let mut paired = vec![false; after.len()];
    let mut unpaired = Vec::new();
    let mut count = 0usize;
    for item in before.drain(..) {
        match pool.get_mut(&key(item)).and_then(Vec::pop) {
            Some(index) => { paired[index] = true; count += 1; }
            None => unpaired.push(item),
        }
    }
    *before = unpaired;
    let mut flags = paired.into_iter();
    after.retain(|_| !flags.next().unwrap_or(false));
    count
}

impl OwnedRhythmicContinuation {
    /// Counts physical note changes between two documents in the order: unchanged
    /// bytes, text-only, staff move, disposition change, then removed/added rest.
    pub(crate) fn custody_census(before: &str, after: &str) -> Result<CustodyCensusCounts> {
        let old = physical_inventory(before)?;
        let new = physical_inventory(after)?;
        let mut left = old.iter().collect::<Vec<_>>();
        let mut right = new.iter().collect::<Vec<_>>();
        let unchanged = census_pair(&mut left, &mut right,
            |p| (p.staff, p.interval.start, p.interval.end, p.text.clone()));
        let text_only = census_pair(&mut left, &mut right,
            |p| (p.staff, p.interval.start, p.interval.end, census_disposition(&p.text)));
        let staff_moved = census_pair(&mut left, &mut right,
            |p| (p.interval.start, p.interval.end, census_disposition(&p.text)));
        let disposition_changed = census_pair(&mut left, &mut right,
            |p| (p.staff, p.interval.start, p.interval.end));
        Ok(CustodyCensusCounts { unchanged, text_only, staff_moved, disposition_changed,
            removed: left.len(), added: right.len() })
    }
}

#[cfg(test)]
#[path = "../tests/unit/owned_rhythmic_continuation.rs"]
mod tests;

#[cfg(test)]
#[path = "../tests/unit/custody_census.rs"]
mod custody_census_tests;

#[cfg(test)]
#[path = "../tests/unit/gesture_relocation_custody.rs"]
mod gesture_relocation_custody_tests;

#[cfg(test)]
#[path = "../tests/unit/notation_endpoint_cleanup_custody.rs"]
mod notation_endpoint_cleanup_custody_tests;

#[cfg(test)]
#[path = "../tests/unit/preserving_layout_journal_custody.rs"]
mod preserving_layout_journal_custody_tests;

#[cfg(test)]
#[path = "../tests/unit/final_layout_complete_course_transport.rs"]
mod final_layout_complete_course_transport_tests;
