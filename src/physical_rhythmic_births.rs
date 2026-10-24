//! Physical birth evidence, captured by the emitter. Neither locators nor this
//! journal establish source ancestry, a course, a role or a writer permit.
use super::*;
use anyhow::{ensure, Result};
use crate::midi_seed::RhythmicCreationIdentity;
use std::ops::Range;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PhysicalBirthKind { Restified, Transferred, MetricFragment, GapRest, GestureRelocated, RestConsolidated }

#[derive(Debug, Clone)]
pub(super) struct PhysicalBirth {
    pub(super) creation: RhythmicCreationIdentity,
    pub(super) kind: PhysicalBirthKind,
    // A transaction-local reference to an input, not an issued parent identity.
    pub(super) input: Option<Range<usize>>,
    pub(super) output: Range<usize>,
    // Pre-state bytes of the visible note the relocated chord member joins.
    // Only a GestureRelocated birth carries it; it is a byte binding, not a search.
    pub(super) anchor: Option<Range<usize>>,
}

/// N:1 evidence from the actual editorial rewrite; it grants no course or role.
#[derive(Debug, Clone)]
pub(super) struct RestConsolidationCapture {
    pub(super) creation: RhythmicCreationIdentity,
    pub(super) execution: crate::master_state_identity::MintedIdentity,
    pub(super) inputs: Vec<Range<usize>>,
    pub(super) output: Range<usize>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct BirthDrafts { records: Vec<PhysicalBirth>, carried: Vec<(Range<usize>, Range<usize>)> }

impl BirthDrafts {
    pub(super) fn capture_continuation(&mut self, input: Range<usize>, offset: usize, sequence: &str) {
        let notes = collect_raw_notes(sequence);
        assert_eq!(notes.len(), 1, "A retained physical occurrence must be emitted once");
        self.carried.push((input, offset + notes[0].start..offset + notes[0].end));
    }
    pub(super) fn capture_visible_sequence(&mut self, owner: &VisibleRhythmBlockMint,
        input: Option<Range<usize>>, kind: PhysicalBirthKind, offset: usize, sequence: &str) {
        assert!(matches!(kind, PhysicalBirthKind::MetricFragment | PhysicalBirthKind::GapRest));
        for note in collect_raw_notes(sequence) {
            self.records.push(PhysicalBirth {
                creation: RhythmicCreationIdentity::visible_rhythm_birth(owner)
                    .expect("Physical rhythmic birth identity exhausted"),
                kind, input: input.clone(), output: offset + note.start..offset + note.end, anchor: None,
            });
        }
    }

    pub(super) fn carry_emission(&mut self, original: &Self, from: Range<usize>, to: usize) {
        for (input, output) in &original.carried {
            if output.start >= from.start && output.end <= from.end {
                self.carried.push((input.clone(), to + output.start - from.start..to + output.end - from.start));
            }
        }
        for birth in original.records.iter().filter(|b| b.output.start >= from.start && b.output.end <= from.end) {
            let mut carried = birth.clone();
            carried.output = to + birth.output.start - from.start..to + birth.output.end - from.start;
            self.records.push(carried);
        }
    }

    pub(super) fn append_measure(&mut self, mut measure: Self, input_offset: usize, output_offset: usize) {
        for birth in &mut measure.records {
            if let Some(input) = &mut birth.input { input.start += input_offset; input.end += input_offset; }
            birth.output.start += output_offset; birth.output.end += output_offset;
        }
        for (input, output) in &mut measure.carried {
            input.start += input_offset; input.end += input_offset;
            output.start += output_offset; output.end += output_offset;
        }
        self.carried.extend(measure.carried);
        self.records.extend(measure.records);
    }

    pub(super) fn seal(self, before: &str, after: &str) -> Result<Option<PhysicalRhythmicBirths>> {
        if self.records.is_empty() && before == after { return Ok(None); }
        let births = PhysicalRhythmicBirths { before: Arc::from(before), after: Arc::from(after), records: self.records, consolidations: Vec::new(), carried: self.carried, original_visible_execution: None };
        births.validate_binding(before, after)?;
        Ok(Some(births))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PhysicalRhythmicBirths {
    before: Arc<str>, after: Arc<str>, records: Vec<PhysicalBirth>,
    carried: Vec<(Range<usize>, Range<usize>)>,
    consolidations: Vec<RestConsolidationCapture>,
    original_visible_execution: Option<crate::master_state_identity::MintedIdentity>,
}

fn captured_note_range(xml: &str, range: Range<usize>) -> Result<Range<usize>> {
    let emitted = xml.get(range.clone()).ok_or_else(|| anyhow::anyhow!("Invalid captured emission range"))?;
    let notes = collect_raw_notes(emitted);
    ensure!(notes.len() == 1 && emitted.trim() == notes[0].text,
        "Captured emission is not one complete physical note");
    Ok(range.start + notes[0].start..range.start + notes[0].end)
}

impl PhysicalRhythmicBirths {
    pub(super) fn records(&self) -> &[PhysicalBirth] { &self.records }
    pub(super) fn consolidations(&self) -> &[RestConsolidationCapture] { &self.consolidations }

    pub(crate) fn editorial_rest_consolidation(
        owner: &crate::structural_write_authority::EditorialVoiceCompleteRewriteExactMint,
        receipt: &crate::master_state_identity::ExactTransformationReceipt,
        before: &str, after: &str, inputs: &[Range<usize>], emitted: Range<usize>,
    ) -> Result<Self> {
        let inputs = inputs.iter().cloned().map(|range| captured_note_range(before, range))
            .collect::<Result<Vec<_>>>()?;
        let output = captured_note_range(after, emitted)?;
        let capture = Self { before: Arc::from(before), after: Arc::from(after),
            records: Vec::new(), carried: Vec::new(), original_visible_execution: None,
            consolidations: vec![RestConsolidationCapture {
                creation: RhythmicCreationIdentity::editorial_rest_birth(owner)?,
                execution: *receipt.ssc_bound_origin().0, inputs, output,
            }],
        };
        capture.validate_exact_execution(receipt)?;
        Ok(capture)
    }

    /// Carry-only capture of BLOCK #53 (E053), recorded by its own emitter.
    /// Every complete note is retained exactly once; any birth, removal or
    /// ambiguous mapping refuses the capture, so custody stays Unknown.
    pub(crate) fn beat_scoped_carry(
        receipt: &crate::master_state_identity::BlockTransformationReceipt,
        before: &str, after: &str, carried: Vec<(Range<usize>, Range<usize>)>) -> Result<Self> {
        receipt.validate_block_transition::<crate::pipeline::BeatScopedBeamingBlockMint>(before, after)?;
        let notes = collect_raw_notes(before).len();
        ensure!(notes == collect_raw_notes(after).len() && carried.len() == notes,
            "Beat-scoped carry does not retain every complete note exactly once");
        let (mut inputs, mut outputs) = (BTreeSet::new(), BTreeSet::new());
        for (input, output) in &carried {
            ensure!(captured_note_range(before, input.clone())? == *input
                && captured_note_range(after, output.clone())? == *output,
                "Beat-scoped carry is not bound to complete notes");
            ensure!(inputs.insert((input.start, input.end)) && outputs.insert((output.start, output.end)),
                "Ambiguous beat-scoped carry");
        }
        let births = Self { before: Arc::from(before), after: Arc::from(after),
            records: Vec::new(), consolidations: Vec::new(), carried, original_visible_execution: None };
        births.validate_binding(before, after)?;
        Ok(births)
    }

    pub(super) fn bind_original_visible_execution(&mut self, _: &VisibleRhythmBlockMint,
        receipt: &crate::master_state_identity::BlockTransformationReceipt) -> Result<()> {
        receipt.validate_block_transition::<VisibleRhythmBlockMint>(&self.before, &self.after)?;
        ensure!(self.original_visible_execution.is_none(), "Original visible execution already bound");
        self.original_visible_execution = Some(*receipt.ssc_bound_origin().0);
        Ok(())
    }

    /// The original #52 emitter copies a whole chord member after splitting
    /// its preceding lexical anchor. This is its recorded copy, not a tuple
    /// match against candidate output notes. The original split births prove
    /// the anchor operation; the bound original BLOCK supplies its execution.
    /// The caller retains the member's received identity.
    pub(super) fn original_retained_chord_copy(&self, before: &str, after: &str,
        input: &Range<usize>, output: &Range<usize>) -> Option<crate::master_state_identity::MintedIdentity> {
        self.validate_binding(before, after).ok()?;
        if !self.carried.iter().any(|pair| pair.0 == *input && pair.1 == *output)
            || before.get(input.clone())? != after.get(output.clone())? { return None; }
        let text = before.get(input.clone())?;
        if !text.contains("<chord") || !text.contains("<pitch>") { return None; }
        let notes = collect_raw_notes(before);
        let member = notes.iter().position(|note| note.start == input.start && note.end == input.end)?;
        let anchor = notes[..member].iter().rev().find(|note| !note.text.contains("<chord"))?;
        let children = self.records.iter().filter(|birth| birth.input.as_ref() == Some(&(anchor.start..anchor.end))).collect::<Vec<_>>();
        if children.len() < 2 || children.iter().any(|birth| birth.kind != PhysicalBirthKind::MetricFragment
            || birth.output.end > output.start) { return None; }
        self.original_visible_execution
    }

    pub(super) fn carried(&self) -> &[(Range<usize>, Range<usize>)] { &self.carried }
    pub(super) fn post_document(&self) -> &str { &self.after }
    pub(crate) fn block_post_document(&self, receipt: &crate::master_state_identity::BlockTransformationReceipt, before: &str) -> Result<String> {
        self.validate_binding(before, &self.after)?;
        // Closed owner set: BLOCK #52 as before; BLOCK #53 only as a carry-only capture.
        let beat_scoped_carry = self.records.is_empty() && self.consolidations.is_empty()
            && receipt.validate_block_transition::<crate::pipeline::BeatScopedBeamingBlockMint>(before, &self.after).is_ok();
        if !beat_scoped_carry {
            receipt.validate_block_transition::<VisibleRhythmBlockMint>(before, &self.after)?;
            ensure!(self.original_visible_execution == Some(*receipt.ssc_bound_origin().0),
                "Foreign original visible emission execution");
        }
        Ok(self.after.to_string())
    }
    pub(crate) fn validate_exact_execution(&self, receipt: &crate::master_state_identity::ExactTransformationReceipt) -> Result<()> {
        if !self.consolidations.is_empty() {
            ensure!(receipt.minted_by::<crate::structural_write_authority::EditorialVoiceCompleteRewriteExactMint>(),
                "Rest consolidation is not the original editorial owner execution");
            ensure!(self.consolidations.iter().all(|capture| &capture.execution == receipt.ssc_bound_origin().0),
                "Rest consolidation belongs to a different original owner event");
        }
        let readback = receipt.operation_readback(&self.before)?;
        self.validate_binding(&self.before, readback.post_document())
    }
    pub(crate) fn restification(owner: &crate::structural_write_authority::RestificationBatchExactMint,
        before: &str, after: &str, edits: &[(usize, usize, String)]) -> Result<Self> {
        let mut edits = edits.iter().collect::<Vec<_>>(); edits.sort_by_key(|e| e.0);
        let mut removed = 0; let mut inserted = 0; let mut records = Vec::new();
        for (start, end, replacement) in edits {
            let output_start = start - removed + inserted;
            if before.get(*start..*end) == Some(replacement.as_str()) { continue; }
            records.push(PhysicalBirth {
                creation: RhythmicCreationIdentity::restification_birth(owner)?,
                kind: PhysicalBirthKind::Restified, input: Some(captured_note_range(before, *start..*end)?),
                output: captured_note_range(after, output_start..output_start + replacement.len())?,
                anchor: None,
            });
            removed += end - start; inserted += replacement.len();
        }
        let births = Self { before: Arc::from(before), after: Arc::from(after), records, consolidations: Vec::new(), carried: Vec::new(), original_visible_execution: None };
        births.validate_binding(before, after)?; Ok(births)
    }

    pub(crate) fn transfer(owner: &crate::structural_write_authority::SafeTransferExactMint,
        before: &str, after: &str, input: Range<usize>, rest_output: Range<usize>, pitch_output: Range<usize>) -> Result<Self> {
        let births = Self { before: Arc::from(before), after: Arc::from(after), records: vec![
            PhysicalBirth { creation: RhythmicCreationIdentity::transfer_birth(owner)?,
                kind: PhysicalBirthKind::Restified, input: Some(captured_note_range(before, input.clone())?), output: captured_note_range(after, rest_output)?, anchor: None },
            PhysicalBirth { creation: RhythmicCreationIdentity::transfer_birth(owner)?,
                kind: PhysicalBirthKind::Transferred, input: Some(captured_note_range(before, input)?), output: captured_note_range(after, pitch_output)?, anchor: None },
         ], consolidations: Vec::new(), carried: Vec::new(), original_visible_execution: None };
        births.validate_binding(before, after)?; Ok(births)
    }

    /// Captures the actual ConsensusGesture relocation of row 26 from the owner's
    /// own input pairs: each pair removes one note `(start, end, "")` and inserts its
    /// chord-member clone at the end of a visible note `(pos, pos, clone)`. Any other
    /// shape, an ambiguous insertion or a non-note range yields an error, so the
    /// receipt simply stays without capture and custody stays Unknown.
    pub(crate) fn gesture_relocation(owner: &crate::structural_write_authority::ChordGestureDistributionExactMint,
        before: &str, after: &str, pairs: &[(usize, usize, String)]) -> Result<Self> {
        ensure!(!pairs.is_empty() && pairs.len().is_multiple_of(2), "Relocation edits are not removal/insertion pairs");
        let mut ordered = pairs.iter().enumerate().collect::<Vec<_>>();
        // Stable descending application places a later equal-position insertion first.
        ordered.sort_by_key(|(index, (start, end, _))| (*start, *end, std::cmp::Reverse(*index)));
        let mut output_start = BTreeMap::new();
        let mut delta = 0isize;
        for (index, (start, end, replacement)) in &ordered {
            let at = usize::try_from(*start as isize + delta).map_err(|_| anyhow::anyhow!("Relocation offset underflow"))?;
            output_start.insert(*index, at);
            delta += replacement.len() as isize - (*end - *start) as isize;
        }
        let mut records = Vec::new();
        for (pair_index, pair) in pairs.chunks(2).enumerate() {
            let ((start, end, removed), (pos, pos_end, clone)) = (&pair[0], &pair[1]);
            ensure!(removed.is_empty() && start < end && pos == pos_end && !clone.is_empty(),
                "Relocation pair is not one removal followed by one insertion");
            // The second edit belongs to this pair even when its bytes repeat.
            let at = output_start[&(2 * pair_index + 1)];
            // Match the note start tag, never a nested notehead element.
            let anchor_start = before[..*pos].rmatch_indices("<note")
                .map(|(index, _)| index)
                .find(|index| matches!(before.as_bytes().get(index + 5), Some(b'>' | b' ' | b'\t' | b'\r' | b'\n')))
                .ok_or_else(|| anyhow::anyhow!("Relocation anchor is not a note"))?;
            records.push(PhysicalBirth {
                creation: RhythmicCreationIdentity::gesture_relocation_birth(owner)?,
                kind: PhysicalBirthKind::GestureRelocated,
                input: Some(captured_note_range(before, *start..*end)?),
                output: captured_note_range(after, at..at + clone.len())?,
                anchor: Some(captured_note_range(before, anchor_start..*pos)?),
            });
        }
        let births = Self { before: Arc::from(before), after: Arc::from(after), records, consolidations: Vec::new(), carried: Vec::new(), original_visible_execution: None };
        births.validate_binding(before, after)?; Ok(births)
    }

    pub(crate) fn validate_binding(&self, before: &str, after: &str) -> Result<()> {
        ensure!(self.before.as_ref() == before && self.after.as_ref() == after, "Physical births belong to another execution");
        let mut identities = BTreeSet::new(); let mut outputs = BTreeSet::new();
        for birth in &self.records {
            ensure!(birth.kind != PhysicalBirthKind::RestConsolidated, "N:1 birth requires its exact parent journal");
            ensure!(identities.insert(birth.creation) && outputs.insert((birth.output.start, birth.output.end)), "Duplicate physical birth");
            let note = after.get(birth.output.clone()).ok_or_else(|| anyhow::anyhow!("Invalid physical birth output"))?;
            ensure!(collect_raw_notes(note).len() == 1 && note.trim().ends_with("</note>"), "Incomplete physical birth output");
            match birth.kind {
                PhysicalBirthKind::Restified | PhysicalBirthKind::GapRest | PhysicalBirthKind::RestConsolidated => ensure!(note.contains("<rest"), "Physical rest birth has no rest"),
                PhysicalBirthKind::Transferred => ensure!(note.contains("<pitch>"), "Physical transfer birth has no pitch"),
                PhysicalBirthKind::MetricFragment => {},
                PhysicalBirthKind::GestureRelocated => ensure!(note.contains("<pitch>") && note.contains("<chord")
                    && birth.anchor.is_some(), "Physical relocation birth is not an anchored pitched chord member"),
            }
            if let Some(input) = &birth.input {
                let note = before.get(input.clone()).ok_or_else(|| anyhow::anyhow!("Invalid physical birth input"))?;
                ensure!(collect_raw_notes(note).len() == 1 && note.trim().ends_with("</note>"), "Incomplete physical birth input");
            } else { ensure!(birth.kind == PhysicalBirthKind::GapRest, "Missing physical input reference"); }
        }
        let mut inputs = BTreeSet::new();
        for capture in &self.consolidations {
            ensure!(identities.insert(capture.creation)
                && outputs.insert((capture.output.start, capture.output.end)),
                "Duplicate rest consolidation birth");
            ensure!(!capture.inputs.is_empty(), "Rest consolidation has no captured parents");
            let output = after.get(capture.output.clone()).ok_or_else(|| anyhow::anyhow!("Invalid consolidation output"))?;
            ensure!(output.contains("<rest") && captured_note_range(after, capture.output.clone())? == capture.output,
                "Consolidation output is not one complete rest");
            let mut ordered = capture.inputs.clone(); ordered.sort_by_key(|r| (r.start, r.end));
            ensure!(ordered.windows(2).all(|pair| pair[0].end <= pair[1].start), "Overlapping consolidation inputs");
            for input in &capture.inputs {
                ensure!(inputs.insert((input.start, input.end))
                    && captured_note_range(before, input.clone())? == *input,
                    "Duplicate or incomplete consolidation input");
                ensure!(!self.records.iter().any(|record| record.input.as_ref() == Some(input)),
                    "Consolidation parent also claimed by another birth");
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn count(&self) -> usize { self.records.len() + self.consolidations.len() }
    #[cfg(test)]
    pub(crate) fn same_creations_as(&self, other: &Self) -> bool {
        self.records.iter().map(|b| b.creation).chain(self.consolidations.iter().map(|b| b.creation))
            .eq(other.records.iter().map(|b| b.creation).chain(other.consolidations.iter().map(|b| b.creation)))
    }
}

/// An actual sealed staff-move execution. This transports an existing occurrence;
/// it issues no creation identity, source, course, role or adoption permission.
#[derive(Debug, Clone)]
pub(crate) struct StaffPresentationTransition {
    before: Arc<str>, after: Arc<str>,
    input: Range<usize>, output: Range<usize>, target_staff: u8,
    execution: crate::master_state_identity::MintedIdentity,
}

impl StaffPresentationTransition {
    pub(crate) fn capture(_owner: &crate::structural_write_authority::StaffVoiceMoveExactMint,
        receipt: &crate::master_state_identity::ExactTransformationReceipt,
        before: &str, after: &str, input: Range<usize>, target_staff: u8) -> Result<Self> {
        ensure!(matches!(target_staff, 1 | 2), "Unsupported staff transition target");
        let readback = receipt.operation_readback(before)?;
        ensure!(readback.post_document() == after, "Staff transition has foreign output");
        let input = captured_note_range(before, input)?;
        let output = captured_note_range(after, readback.carry_range(input.clone())?)?;
        let transition = Self { before: Arc::from(before), after: Arc::from(after), input, output, target_staff,
            execution: *receipt.ssc_bound_origin().0 };
        transition.validate_exact_execution(receipt)?;
        Ok(transition)
    }

    pub(crate) fn validate_exact_execution(&self,
        receipt: &crate::master_state_identity::ExactTransformationReceipt) -> Result<()> {
        ensure!(*receipt.ssc_bound_origin().0 == self.execution
            && receipt.minted_by::<crate::structural_write_authority::StaffVoiceMoveExactMint>(),
            "Staff transition requires its original sealed owner");
        let readback = receipt.operation_readback(&self.before)?;
        ensure!(readback.post_document() == self.after.as_ref()
            && readback.carry_range(self.input.clone())? == self.output,
            "Staff transition belongs to another execution");
        Ok(())
    }

    pub(super) fn binds(&self, before: &str, after: &str,
        input: &Range<usize>, output: &Range<usize>, target_staff: u8) -> bool {
        self.before.as_ref() == before && self.after.as_ref() == after
            && &self.input == input && &self.output == output && self.target_staff == target_staff
    }
}
