//! Original canonical binding and original two-phase emission of owner #65.
//! This carries received obligations; it never discharges them or issues a role.
use super::*;
use anyhow::{ensure, Result};
use crate::master_state_identity::{FinalChordCourseCapture, FinalChordCourseRow};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OriginalCourseBindings {
    rows: Vec<(FinalChordCanonicalInputBindingRow, Option<FinalChordCanonicalActionKind>, i64)>,
}

impl OriginalCourseBindings {
    pub(super) fn respelling_sources(&self) -> Vec<(std::ops::Range<usize>, usize)> {
        self.rows.iter().map(|(binding, _, _)|
            (binding.source_note_start..binding.source_note_end, binding.source_event_id)).collect()
    }

    pub(super) fn from_original_bind(
        inputs: &[FinalChordCanonicalInputBindingRow], canonical: &CanonicalFinalChordStructuralPlan,
    ) -> io::Result<Self> {
        let mut rows = Vec::new();
        for input in inputs {
            let locator = canonical.source_locators.iter().find(|locator|
                locator.fragment_id == input.fragment_id).ok_or_else(||
                    io::Error::new(io::ErrorKind::InvalidData, "Missing original canonical course binding"))?;
            if (input.source_event_id, input.attack_id) != (locator.source_event_id, locator.attack_id) {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Foreign canonical course binding"));
            }
            let action = canonical.actions.iter().find(|action| action.fragment_id == input.fragment_id);
            if action.is_some_and(|action| (action.source_event_id, action.attack_id) !=
                (input.source_event_id, input.attack_id)) {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Foreign canonical course action"));
            }
            let offset = match action.map(|action| &action.kind) {
                Some(FinalChordCanonicalActionKind::TiedAnchorChain { fragments }) => {
                    let first = fragments.first().ok_or_else(|| io::Error::new(
                        io::ErrorKind::InvalidData, "Original tied action has no target"))?;
                    locator.onset.checked_sub(first.target_onset).ok_or_else(|| io::Error::new(
                        io::ErrorKind::InvalidData, "Original tied correspondence overflow"))?
                },
                _ => 0,
            };
            rows.push((input.clone(), action.map(|action| action.kind.clone()), offset));
        }
        Ok(Self { rows })
    }
}

pub(super) fn capture_original_execution(
    owner: &FinalChordEncodingExactMint, receipt: &ExactTransformationReceipt,
    before: &str, after: &str, plan: &FinalChordEncodingPlan,
    rest_edits: &[FinalChordEncodingEdit],
) -> Result<FinalChordCourseCapture> {
    let bindings = plan.original_course_bindings.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Missing original FinalChord canonical binding"))?
        .as_ref().map_err(|error| anyhow::anyhow!("Original canonical course capture: {error}"))?;
    let projection = final_chord_collect_original_projection(before, plan, rest_edits, after.to_owned())?;
    let original_notes = final_chord_parse_binding_measures(before)?.into_iter()
        .flat_map(|measure| measure.notes).map(|note| ((note.range.start, note.range.end), note))
        .collect::<BTreeMap<_, _>>();
    let mut rows = BTreeMap::<(usize, usize), FinalChordCourseRow>::new();
    for emitted in projection.provenance {
        let input = emitted.source_note_start..emitted.source_note_end;
        let (binding, action, original_offset) = bindings.rows.iter().find(|(binding, _, _)|
            binding.source_note_start == input.start && binding.source_note_end == input.end)
            .ok_or_else(|| anyhow::anyhow!("Emission lacks its original canonical source"))?;
        let expected_kind = match action {
            Some(FinalChordCanonicalActionKind::TiedAnchorChain { fragments }) => {
                ensure!(emitted.fragment_ordinal < fragments.len(), "Foreign tied fragment ordinal");
                FinalChordProjectionMaterializationKind::TiedAnchorChain
            },
            Some(FinalChordCanonicalActionKind::AuxiliaryLane { .. }) => {
                ensure!(emitted.fragment_ordinal == 0, "Foreign auxiliary fragment ordinal");
                FinalChordProjectionMaterializationKind::AuxiliaryLane
            },
            _ => {
                ensure!(emitted.fragment_ordinal == 0, "Foreign retained fragment ordinal");
                FinalChordProjectionMaterializationKind::SurvivingSourceNote
            },
        };
        ensure!(emitted.kind == expected_kind, "Emission differs from original canonical action");
        let row = rows.entry((input.start, input.end)).or_insert_with(|| FinalChordCourseRow {
            input, outputs: Vec::new(), ordinals: Vec::new(), source: Some(binding.source_event_id),
            tied_chain: expected_kind == FinalChordProjectionMaterializationKind::TiedAnchorChain,
            auxiliary: expected_kind == FinalChordProjectionMaterializationKind::AuxiliaryLane,
            original_offset: *original_offset,
        });
        row.ordinals.push(emitted.fragment_ordinal);
        row.outputs.push((emitted.projected_note_start..emitted.projected_note_end, false));
    }
    for emitted in projection.non_pitched_continuations {
        let input = emitted.source_note_start..emitted.source_note_end;
        ensure!(original_notes.get(&(input.start, input.end)).is_some_and(|note| note.rest_like),
            "Non-pitched continuation is not an original rest");
        let row = rows.entry((input.start, input.end)).or_insert_with(|| FinalChordCourseRow {
            input, outputs: Vec::new(), ordinals: Vec::new(), source: None,
            tied_chain: false, auxiliary: false, original_offset: 0,
        });
        row.ordinals.push(emitted.fragment_ordinal);
        row.outputs.push((emitted.projected_start..emitted.projected_end,
            emitted.carrier == FinalChordNonPitchedCarrier::Forward));
    }
    ensure!(rows.keys().eq(original_notes.keys()), "Original course emission coverage is incomplete");
    let mut outputs = BTreeSet::new();
    for row in rows.values_mut() {
        let mut emitted = row.outputs.drain(..).zip(row.ordinals.drain(..)).collect::<Vec<_>>();
        emitted.sort_by_key(|((range, _), _)| range.start);
        (row.outputs, row.ordinals) = emitted.into_iter().unzip();
        ensure!(row.ordinals.iter().copied().collect::<BTreeSet<_>>() ==
            (0..row.outputs.len()).collect(), "Original emission ordinal coverage is incomplete");
        ensure!(!row.outputs.is_empty() && row.outputs.iter().all(|(range, _)|
            range.start < range.end && range.end <= after.len() &&
            outputs.insert((range.start, range.end))), "Ambiguous original course emission");
    }
    FinalChordCourseCapture::recorded(owner, receipt, before, after, rows.into_values().collect())
}
