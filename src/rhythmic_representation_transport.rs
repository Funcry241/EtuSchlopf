//! Owner-recorded representation operations. These carry locations, never mint origins.
use anyhow::{ensure, Result};
use std::ops::Range;

/// Read-only context from the existing PRESERVING projection. Never supplies
/// a receipt, adoption verdict, origin or write permission.
pub(crate) fn preserving_stage_evidence<O: crate::master_state_identity::PreservingMintOwner>(
    before: &str, after: &str, receipt_present: bool,
) -> String {
    let before_projection = O::conserved_projection(before);
    let after_projection = O::conserved_projection(after);
    let projection_evidence = match (&before_projection, &after_projection) {
        (Some(before), Some(after)) => {
            let mut before_lines = before.lines();
            let mut after_lines = after.lines();
            let mut line = 0;
            loop {
                let pre = before_lines.next();
                let post = after_lines.next();
                if pre != post {
                    break format!("projection_difference_line={line}; pre={pre:?}; post={post:?}");
                }
                if pre.is_none() { break "projection_equal=true".to_owned(); }
                line += 1;
            }
        }
        _ => format!("pre_projection_available={}; post_projection_available={}; pre_parse_error={:?}; post_parse_error={:?}",
            before_projection.is_some(), after_projection.is_some(),
            parse_failure_evidence(before), parse_failure_evidence(after)),
    };
    format!("changed={}; receipt_present={receipt_present}; pre_bytes={}; post_bytes={}; {projection_evidence}",
        before != after, before.len(), after.len())
}

fn parse_failure_evidence(xml: &str) -> Option<String> {
    let error = roxmltree::Document::parse(xml).err()?;
    let row = (error.pos().row as usize).saturating_sub(1);
    let context = xml.lines().enumerate()
        .filter(|(index, _)| *index == row || index.checked_add(1) == Some(row))
        .map(|(_, text)| text)
        .collect::<Vec<_>>().join("\n");
    Some(format!("{error}; source_context={context:?}"))
}

/// Caller-owned journal, explicitly passed to the existing owner context.
#[derive(Debug, Clone, Default)]
pub(crate) struct RepresentationCapture(std::rc::Rc<std::cell::RefCell<Option<RepresentationStep>>>);

impl RepresentationCapture {
    pub(crate) fn record(&self, step: RepresentationStep) {
        assert!(self.0.replace(Some(step)).is_none(), "Representation capture reused");
    }

    pub(crate) fn take(&self) -> Result<RepresentationStep> {
        self.0.borrow_mut().take().ok_or_else(|| anyhow::anyhow!("Missing actual owner capture"))
    }
}

#[cfg(test)]
#[path = "../tests/unit/rhythmic_representation_transport.rs"]
mod tests;

#[derive(Debug, Clone)]
pub(crate) struct RepresentationStep {
    before: String,
    after: String,
    edits: Vec<(Range<usize>, String)>,
}

impl RepresentationStep {
    pub(crate) fn recorded(before: &str, after: &str, edits: Vec<(Range<usize>, String)>) -> Self {
        Self { before: before.to_owned(), after: after.to_owned(), edits }
    }

    /// Lift an emitter's local operation journal into its actual document write.
    /// The original span, unchanged exterior and replay all remain mandatory.
    pub(crate) fn enclosed(self, before: &str, after: &str, span: Range<usize>) -> Result<Self> {
        self.validate()?;
        ensure!(before.get(span.clone()) == Some(self.before.as_str()), "Foreign enclosed operation pre-state");
        let mut expected = before[..span.start].to_string();
        expected.push_str(&self.after); expected.push_str(&before[span.end..]);
        ensure!(expected == after, "Enclosed operation changed the exterior or endpoint");
        Ok(Self::recorded(before, after, self.edits.into_iter().map(|(range, text)|
            (span.start + range.start..span.start + range.end, text)).collect()))
    }

    fn validate(&self) -> Result<()> {
        let mut cursor = 0;
        let mut expected = String::new();
        for (range, replacement) in &self.edits {
            ensure!(range.start >= cursor && range.end >= range.start,
                "Overlapping representation operations");
            expected.push_str(self.before.get(cursor..range.start)
                .ok_or_else(|| anyhow::anyhow!("Invalid representation range"))?);
            ensure!(self.before.get(range.clone()).is_some(), "Invalid representation operation");
            expected.push_str(replacement);
            cursor = range.end;
        }
        expected.push_str(self.before.get(cursor..)
            .ok_or_else(|| anyhow::anyhow!("Invalid representation tail"))?);
        ensure!(expected == self.after, "Owner operations do not reproduce the representation");
        Ok(())
    }

    fn carry(&self, range: Range<usize>) -> Result<Range<usize>> {
        let mut removed = 0usize;
        let mut inserted = 0usize;
        for (edit, replacement) in &self.edits {
            if edit.end <= range.start {
                removed += edit.end - edit.start;
                inserted += replacement.len();
            } else if edit.start >= range.end {
                break;
            } else {
                ensure!(*edit == range, "Representation operation crosses an occurrence boundary");
                let start = range.start - removed + inserted;
                return Ok(start..start + replacement.len());
            }
        }
        Ok(range.start - removed + inserted..range.end - removed + inserted)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RhythmicRepresentationTransport {
    steps: Vec<RepresentationStep>,
}

impl RhythmicRepresentationTransport {
    pub(crate) fn octave_notation(step: RepresentationStep,
        receipt: &crate::master_state_identity::BlockTransformationReceipt) -> Result<Self> {
        step.validate()?;
        receipt.validate_block_transition::<crate::structural_write_authority::OctaveMeasureBlockMint>(
            &step.before, &step.after)?;
        let (start, end) = crate::master_state_identity::ssc_block_bound_span(receipt)
            .ok_or_else(|| anyhow::anyhow!("Missing octave owner span"))?;
        for (range, text) in &step.edits {
            ensure!(range.start == range.end && range.start >= start && range.end <= end,
                "Octave journal is not a bound insertion");
            let doc = roxmltree::Document::parse(text)?;
            ensure!(doc.root_element().tag_name().name() == "direction"
                && !doc.descendants().any(|n| n.is_element()
                    && matches!(n.tag_name().name(), "note" | "backup" | "forward")),
                "Octave journal contains a rhythmic operation");
        }
        Ok(Self { steps: vec![step] })
    }

    /// Owner-recorded journal of a frozen PRESERVING layout write (E044/E013).
    /// Every removed and inserted fragment must be free of rhythmic elements.
    pub(crate) fn preserving_layout(step: RepresentationStep,
        receipt: &crate::master_state_identity::PreservingTransformationReceipt) -> Result<Self> {
        step.validate()?;
        receipt.binds_layout_journal(&step.before, &step.after)?;
        ensure!(!step.edits.is_empty(), "Layout journal records no operation");
        for (range, text) in &step.edits {
            let removed = step.before.get(range.clone())
                .ok_or_else(|| anyhow::anyhow!("Invalid layout journal range"))?;
            ensure!([removed, text.as_str()].iter().all(|fragment|
                !["<note", "<backup", "<forward", "<measure", "</measure"].iter().any(|token| fragment.contains(token))),
                "Layout journal touches a rhythmic element");
        }
        Ok(Self { steps: vec![step] })
    }

    /// Split the already emitted clef journal at the original receipt boundaries.
    pub(crate) fn clef_receipt_steps(&self,
        receipts: &[crate::master_state_identity::BlockTransformationReceipt]) -> Result<Vec<Self>> {
        let steps = self.steps.iter().filter(|step| step.before != step.after).collect::<Vec<_>>();
        ensure!(steps.len() == receipts.len(), "Clef journal/receipt coverage differs");
        let mut output = Vec::new();
        for (step, receipt) in steps.into_iter().zip(receipts) {
            step.validate()?;
            receipt.validate_block_transition::<crate::structural_write_authority::ClefMeasureBlockMint>(
                &step.before, &step.after)?;
            output.push(Self { steps: vec![step.clone()] });
        }
        Ok(output)
    }

    /// Complete original operation chain, including every intermediate state.
    pub(crate) fn recorded_documents(&self) -> Result<(&str, &str)> {
        let first = self.steps.first().ok_or_else(|| anyhow::anyhow!("Missing recorded representation"))?;
        let mut previous = first.before.as_str();
        for step in &self.steps {
            ensure!(step.before == previous, "Discontinuous recorded representation");
            step.validate()?;
            previous = &step.after;
        }
        Ok((&first.before, previous))
    }

    pub(crate) fn partial_beam(_: &crate::pipeline::PartialBeatBeamPreservingMint,
        step: RepresentationStep) -> Self { Self { steps: vec![step] } }

    /// Owner-recorded journal of the PRESERVING Rustig clef course (E024).
    /// Every removed and inserted fragment must be free of rhythmic elements.
    pub(crate) fn rustig_clef_course(_: &crate::structural_write_authority::RustigClefCoursePreservingMint,
        step: RepresentationStep) -> Result<Self> {
        step.validate()?;
        ensure!(!step.edits.is_empty(), "Clef course journal records no operation");
        for (range, text) in &step.edits {
            let removed = step.before.get(range.clone())
                .ok_or_else(|| anyhow::anyhow!("Invalid clef course journal range"))?;
            ensure!([removed, text.as_str()].iter().all(|fragment|
                !["<note", "<backup", "<forward", "<measure", "</measure"].iter().any(|token| fragment.contains(token))),
                "Clef course journal touches a rhythmic element");
        }
        Ok(Self { steps: vec![step] })
    }

    pub(crate) fn ledger_clef(_: &crate::pipeline::LedgerBoundedLowerStaffClefReadbackPreservingMint,
        step: RepresentationStep) -> Self { Self { steps: vec![step] } }

    pub(crate) fn single_step_documents(&self) -> Result<(&str, &str)> {
        ensure!(self.steps.len() == 1, "Recorded block transport is not one original step");
        Ok((&self.steps[0].before, &self.steps[0].after))
    }
    /// Uses the actual BLOCK #49 receipts, without minting or bridging authority.
    pub(crate) fn clef_readability(steps: Vec<RepresentationStep>,
        receipts: &[crate::master_state_identity::BlockTransformationReceipt]) -> Result<Self> {
        let mut receipts = receipts.iter();
        for step in &steps {
            step.validate()?;
            if step.before != step.after {
                receipts.next().ok_or_else(|| anyhow::anyhow!("Missing clef owner receipt"))?
                    .validate_block_transition::<crate::structural_write_authority::ClefMeasureBlockMint>(
                        &step.before, &step.after)?;
            }
        }
        ensure!(receipts.next().is_none(), "Unconsumed clef owner receipt");
        Ok(Self { steps })
    }

    pub(crate) fn polish(_: &crate::pipeline::NotationPolishPreservingMint,
        steps: Vec<RepresentationStep>) -> Self {
        Self { steps }
    }

    pub(crate) fn layout(_: &crate::layout::PianistLayoutPreservingMint,
        steps: Vec<RepresentationStep>) -> Self {
        Self { steps }
    }

    pub(crate) fn carry(&self, before: &str, after: &str,
        ranges: Vec<Range<usize>>) -> Result<Vec<Range<usize>>> {
        let mut state = before;
        let mut carried = ranges;
        ensure!(!self.steps.is_empty(), "Missing actual representation operations");
        for step in &self.steps {
            ensure!(state == step.before, "Foreign representation pre-state");
            step.validate()?;
            carried = carried.into_iter().map(|range| step.carry(range)).collect::<Result<_>>()?;
            state = &step.after;
        }
        ensure!(state == after, "Foreign representation post-state; {}",
            crate::audit_only::panic_origin_observation::difference_evidence(state, after));
        Ok(carried)
    }
}
