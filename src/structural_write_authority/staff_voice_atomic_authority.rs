//! Typed central authority for exact atomic staff/voice edit batches.
//!
//! The caller computes move and restification targets. This module only binds
//! those exact ranges to the original source and consumes them once.

#[cfg(test)]
mod batch_owner_receipt_replay_tests {
    use super::*;

    fn batch_note(voice: &str, staff: &str, step: &str) -> String {
        format!("<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>24</duration><voice>{voice}</voice><staff>{staff}</staff></note>")
    }

    #[test]
    fn scalar_then_restification_batches_replay_intermediate_bytes_and_receipt_order() {
        for (upper_voice, lower_voice) in [("1", "2"), ("10", "20"), ("100", "200")] {
            let upper = batch_note(upper_voice, "1", "C");
            let lower = batch_note(lower_voice, "2", "G");
            let source = format!("<score-partwise><part id=\"P1\"><measure number=\"synthetic\">{upper}{lower}</measure></part></score-partwise>");
            let mut edits = Vec::new();
            for (note, staff) in [(&upper, "2"), (&lower, "1")] {
                let offset = source.find(note.as_str()).expect("original note");
                let start = offset + note.find("<staff>").expect("staff") + "<staff>".len();
                edits.push((start, start + 1, staff.to_string()));
            }
            let (moved, move_receipt) = apply_atomic_staff_voice_scalar_batch_authorized(&source, edits);
            let moved = moved.expect("scalar batch output");
            let move_receipt = move_receipt.expect("scalar batch receipt");
            assert_eq!(move_receipt.realize_post_document(&source).as_deref(), Some(moved.as_str()));
            let mut receipts = crate::master_state_identity::OwnerReceipts::default();
            receipts.push_exact(Some(move_receipt));
            assert_eq!(receipts.coverage_diagnostic(&source, &moved), None);
            let moved_upper = batch_note(upper_voice, "2", "C");
            let moved_lower = batch_note(lower_voice, "1", "G");
            let mut edits = Vec::new();
            for (note, voice, staff) in [(&moved_upper, upper_voice, "2"), (&moved_lower, lower_voice, "1")] {
                let start = moved.find(note.as_str()).expect("actual intermediate note");
                let rest = format!("<note><rest/><duration>24</duration><voice>{voice}</voice><staff>{staff}</staff></note>");
                edits.push((start, start + note.len(), rest));
            }
            let (restified, rest_receipt) = apply_atomic_restification_batch_authorized(&moved, edits);
            let restified = restified.expect("restification batch output");
            assert_ne!(restified, moved);
            let rest_receipt = rest_receipt.expect("restification batch receipt");
            assert_eq!(rest_receipt.realize_post_document(&moved).as_deref(), Some(restified.as_str()));
            let mut tail = crate::master_state_identity::OwnerReceipts::default();
            tail.push_exact(Some(rest_receipt));
            assert_eq!(tail.coverage_diagnostic(&moved, &restified), None);
            receipts.append(tail);
            assert_eq!(receipts.len(), 2);
            assert_eq!(receipts.coverage_diagnostic(&source, &restified), None);
        }
    }
}


use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt};
use super::destructive_edit_attribution::{DestructiveEditBatchAttribution, IndividualEditIdentity};
use super::authority_primitives::{
    fingerprint,
    parsed_exact_note as exact_note,
    parsed_exact_rest_note as exact_rest_note,
    scalar_lane_value,
    child_text,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AtomicEditEffect {
    RestifyNote,
    StaffVoiceScalar,
}

#[derive(Debug)]
struct BoundAtomicEdit {
    identity: IndividualEditIdentity,
    start: usize,
    end: usize,
    original: String,
    original_fingerprint: u64,
    replacement: String,
}

#[derive(Debug)]
struct AtomicEditBatch {
    source: String,
    source_fingerprint: u64,
    effect: AtomicEditEffect,
    edits_descending: Vec<BoundAtomicEdit>,
    attribution: DestructiveEditBatchAttribution,
}

impl AtomicEditBatch {
    fn issue(
        source: &str,
        edits: Vec<(usize, usize, String)>,
        effect: AtomicEditEffect,
        attribution: DestructiveEditBatchAttribution,
    ) -> Option<Self> {
        let mut bound = Vec::with_capacity(edits.len());
        for (ordinal, (start, end, replacement)) in edits.into_iter().enumerate() {
            let identity = attribution.identity(ordinal)?;
            if start >= end || end > source.len() {
                return None;
            }
            let original = source.get(start..end)?.to_string();
            if !atomic_edit_allowed(effect, &original, &replacement) {
                return None;
            }
            bound.push(BoundAtomicEdit {
                identity,
                start,
                end,
                original_fingerprint: fingerprint(&original),
                original,
                replacement,
            });
        }
        bound.sort_by_key(|left| std::cmp::Reverse(left.start));
        for pair in bound.windows(2) {
            let later = &pair[0];
            let earlier = &pair[1];
            if earlier.end > later.start {
                return None;
            }
        }
        for edit in &bound {
            attribution.mark_bound(edit.identity);
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            effect,
            edits_descending: bound,
            attribution,
        })
    }

    fn apply(self, source: &str) -> Option<String> {
        let Self {
            source: bound_source,
            source_fingerprint,
            effect,
            edits_descending,
            attribution,
        } = self;
        if bound_source != source || source_fingerprint != fingerprint(source) {
            return None;
        }
        let identities = edits_descending
            .iter()
            .map(|edit| edit.identity)
            .collect::<Vec<_>>();
        let mut attributed_xml = bound_source;
        for edit in edits_descending {
            let current = attributed_xml.get(edit.start..edit.end)?;
            if current != edit.original
                || fingerprint(current) != edit.original_fingerprint
                || !atomic_edit_allowed(effect, current, &edit.replacement)
            {
                return None;
            }
            attributed_xml.replace_range(edit.start..edit.end, &edit.replacement);
        }
        attribution.mark_applied_batch(&identities, source, &attributed_xml);
        Some(attributed_xml)
    }
}

fn atomic_edit_allowed(effect: AtomicEditEffect, original: &str, replacement: &str) -> bool {
    match effect {
        AtomicEditEffect::RestifyNote => {
            exact_note(original)
                && exact_rest_note(replacement)
                && child_text(original, "duration") == child_text(replacement, "duration")
                && child_text(original, "voice") == child_text(replacement, "voice")
                && child_text(original, "staff") == child_text(replacement, "staff")
        }
        AtomicEditEffect::StaffVoiceScalar => {
            scalar_lane_value(original) && scalar_lane_value(replacement)
        }
    }
}






#[track_caller]
pub fn apply_atomic_restification_batch_authorized(
    source: &str,
    edits: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        edits.len(),
    );
    let receipt_edits = bound_receipt_edits(&edits);
    let birth_edits = edits.clone();
    let out = AtomicEditBatch::issue(source, edits, AtomicEditEffect::RestifyNote, attribution)
        .and_then(|batch| batch.apply(source));
    // EXACT receipt #39, minted by this owner from the bound batch edits.
    let mut receipt = out.as_deref().and_then(|out| {
        ExactTransformationReceipt::mint_exact_receipt(
            RestificationBatchExactMint { _sealed: () },
            source,
            out,
            receipt_edits,
        )
    });
    if let (Some(out), Some(receipt)) = (out.as_deref(), receipt.as_mut())
        && let Ok(births) = crate::arrangement::physical_births::PhysicalRhythmicBirths::restification(
            &RestificationBatchExactMint { _sealed: () }, source, out, &birth_edits) {
        receipt.attach_physical_births(births).expect("Actual restification births must bind their exact execution");
    }
    (out, receipt)
}

fn bound_receipt_edits(edits: &[(usize, usize, String)]) -> Vec<ExactEdit> {
    edits
        .iter()
        .map(|(start, end, replacement)| ExactEdit::bound(*start, *end, replacement.clone()))
        .collect()
}

/// Sealed mint capability of `apply_atomic_restification_batch_authorized` (EXACT #39).
pub(crate) struct RestificationBatchExactMint {
    _sealed: (),
}

/// Sealed mint capability of `apply_atomic_staff_voice_scalar_batch_authorized` (EXACT #40).
pub(crate) struct StaffVoiceScalarBatchExactMint {
    _sealed: (),
}

#[track_caller]
pub fn apply_atomic_staff_voice_scalar_batch_authorized(
    source: &str,
    edits: Vec<(usize, usize, String)>,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let attribution = DestructiveEditBatchAttribution::begin_destructive_edit_batch_attribution(
        edits.len(),
    );
    let receipt_edits = bound_receipt_edits(&edits);
    let out = AtomicEditBatch::issue(source, edits, AtomicEditEffect::StaffVoiceScalar, attribution)
        .and_then(|batch| batch.apply(source));
    // EXACT receipt #40, minted by this owner from the bound batch edits.
    let receipt = out.as_deref().and_then(|out| {
        ExactTransformationReceipt::mint_exact_receipt(
            StaffVoiceScalarBatchExactMint { _sealed: () },
            source,
            out,
            receipt_edits,
        )
    });
    (out, receipt)
}
