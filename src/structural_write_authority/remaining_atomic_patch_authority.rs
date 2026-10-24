//! Typed central authority for the two remaining atomic staff/voice patch
//! materializations. The caller owns target selection and replacement
//! construction; this module binds exact source bytes and consumes once.

#[cfg(test)]
mod owner_receipt_replay_tests {
    use super::*;

    fn pitched_note(step: &str, octave: u8, voice: &str, staff: &str, duration: i64) -> String {
        format!("<note><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><type>quarter</type><staff>{staff}</staff></note>")
    }

    fn assert_receipt_covers(before: &str, after: &str, receipt: Option<ExactTransformationReceipt>) {
        assert_ne!(before, after);
        let receipt = receipt.expect("changed owner output must be certified");
        assert_eq!(receipt.realize_post_document(before).as_deref(), Some(after));
        let mut receipts = crate::master_state_identity::OwnerReceipts::default();
        receipts.push_exact(Some(receipt));
        assert_eq!(receipts.len(), 1);
        assert_eq!(receipts.coverage_diagnostic(before, after), None);
    }

    #[test]
    fn transfer_receipts_replay_both_anchor_orders_and_duration_changes() {
        for (upper_voice, lower_voice) in [("1", "2"), ("10", "20"), ("100", "200")] {
            for (upper_duration, lower_duration) in [(24, 24), (24, 48), (48, 24)] {
                let upper = pitched_note("C", 5, upper_voice, "1", upper_duration);
                let lower = pitched_note("G", 3, lower_voice, "2", lower_duration);
                let source = format!("<score-partwise><part id=\"P1\"><measure number=\"synthetic\">{upper}{lower}</measure></part></score-partwise>");
                for (original, anchor, step, octave, voice, staff, duration, new_voice, new_staff, new_duration) in [
                    (&upper, &lower, "C", 5, upper_voice, "1", upper_duration, lower_voice, "2", lower_duration),
                    (&lower, &upper, "G", 3, lower_voice, "2", lower_duration, upper_voice, "1", upper_duration),
                ] {
                    let start = source.find(original.as_str()).expect("source note");
                    let end = start + original.len();
                    let insert_at = source.find(anchor.as_str()).expect("anchor note") + anchor.len();
                    let rest = format!("<note><rest/><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>");
                    let transferred = format!("<note><chord/><pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{new_duration}</duration><voice>{new_voice}</voice><staff>{new_staff}</staff></note>");
                    let (after, receipt) = apply_bound_safe_transfer_authorized(
                        &source, start, end, insert_at, rest, transferred,
                    );
                    assert_receipt_covers(&source, &after.expect("valid bound transfer"), receipt);
                }
            }
        }
    }

    #[test]
    fn bound_lane_receipts_replay_width_changes_and_missing_staff() {
        for (source_voice, destination_voice) in [("1", "20"), ("200", "2"), ("10", "30")] {
            for staff_tag in ["<staff>1</staff>", ""] {
                let source = format!("<note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>{source_voice}</voice><type>quarter</type>{staff_tag}</note>");
                let start = source.find("<voice>").expect("voice") + "<voice>".len();
                let end = start + source[start..].find('<').expect("voice end");
                let staff_range = source.find("<staff>").map(|at| (at + "<staff>".len(), at + "<staff>".len() + 1));
                let (after, receipt) = apply_bound_staff_voice_move_authorized(
                    &source, Some((start, end)), staff_range, source.len(),
                    destination_voice.to_string(), "2".to_string(),
                );
                assert_receipt_covers(&source, &after.expect("valid bound lane move"), receipt);
            }
        }
    }

    #[test]
    #[should_panic(expected = "SafeAtomicCandidateProducer/Patch: owner receipts do not cover the adopted Master transition")]
    fn overlapping_transfer_anchor_replays_uncovered_sovereign_adoption() {
        // Invalid inside-note anchor: mechanism probe, never a corpus cause claim.
        let pre = pitched_note("C", 4, "1", "1", 24);
        let rest = "<note><rest/><duration>24</duration><voice>1</voice><staff>1</staff></note>";
        let transferred = "<note><chord/><pitch><step>C</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>";
        let (post, receipt) = apply_bound_safe_transfer_authorized(
            &pre, 0, pre.len(), 1, rest.to_string(), transferred.to_string(),
        );
        let post = post.expect("overlapping bound anchor materializes");
        assert_ne!(post, pre);
        assert!(receipt.is_none());
        let mut receipts = crate::master_state_identity::OwnerReceipts::default();
        receipts.push_exact(receipt);
        let diagnostic = receipts.coverage_diagnostic(&pre, &post).expect("uncovered transition");
        assert!(diagnostic.contains("receipt_count=0"));
        assert!(diagnostic.contains("first_discontinuous_receipt=None"));
        let genesis = crate::master_state_identity::tests::synthetic_genesis_for_tests(&pre);
        let mut state = crate::structural_write_authority::MasterAdoptedState::adopt_genesis_state(
            pre, crate::master_state_identity::MasterLineage::lineage_from_genesis(genesis),
        );
        state.adopt_materialized_document(post, receipts, "SafeAtomicCandidateProducer/Patch");
    }
}


use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt};
use super::authority_primitives::{
    fingerprint,
    parsed_exact_note as exact_note,
    parsed_exact_rest_note as exact_rest_note,
    syntactic_exact_note,
    scalar_lane_value,
    child_text,
    child_block,
};

#[derive(Debug)]
struct SafeTransferCapability {
    source: String,
    source_fingerprint: u64,
    note_start: usize,
    note_end: usize,
    insert_after_original: usize,
    original_note: String,
    original_note_fingerprint: u64,
    restified: String,
    transferred: String,
}

impl SafeTransferCapability {
    fn issue(
        source: &str,
        note_start: usize,
        note_end: usize,
        insert_after_original: usize,
        restified: String,
        transferred: String,
    ) -> Option<Self> {
        if note_start >= note_end || note_end > source.len() || insert_after_original > source.len() {
            return None;
        }
        let original_note = source.get(note_start..note_end)?.to_string();
        if !exact_note(&original_note)
            || !exact_rest_note(&restified)
            || !exact_note(&transferred)
            || child_text(&original_note, "duration") != child_text(&restified, "duration")
            || child_text(&original_note, "voice") != child_text(&restified, "voice")
            || child_text(&original_note, "staff") != child_text(&restified, "staff")
            || child_block(&original_note, "pitch") != child_block(&transferred, "pitch")
        {
            return None;
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            note_start,
            note_end,
            insert_after_original,
            original_note_fingerprint: fingerprint(&original_note),
            original_note,
            restified,
            transferred,
        })
    }

    fn apply(self, source: &str) -> Option<String> {
        if self.source != source || self.source_fingerprint != fingerprint(source) {
            return None;
        }
        let current = self.source.get(self.note_start..self.note_end)?;
        if current != self.original_note || fingerprint(current) != self.original_note_fingerprint {
            return None;
        }
        let mut out = self.source;
        out.replace_range(self.note_start..self.note_end, &self.restified);
        let delta = self.restified.len() as isize - (self.note_end - self.note_start) as isize;
        let mut insert_at = self.insert_after_original;
        if self.note_end <= self.insert_after_original {
            insert_at = ((insert_at as isize) + delta).max(0) as usize;
        }
        if insert_at > out.len() {
            return None;
        }
        out.insert_str(insert_at, &format!("\n{}", self.transferred));
        Some(out)
    }

    /// The two bound edits of this capability in pre-state coordinates.
    fn receipt_edits(&self) -> Vec<ExactEdit> {
        vec![
            ExactEdit::bound(self.note_start, self.note_end, self.restified.clone()),
            ExactEdit::bound(
                self.insert_after_original,
                self.insert_after_original,
                format!("\n{}", self.transferred),
            ),
        ]
    }
}

#[derive(Debug)]
struct StaffVoiceMoveCapability {
    source: String,
    source_fingerprint: u64,
    voice_value_range: Option<(usize, usize)>,
    staff_value_range: Option<(usize, usize)>,
    note_end: usize,
    target_voice: String,
    target_staff: String,
}

impl StaffVoiceMoveCapability {
    fn issue(
        source: &str,
        voice_value_range: Option<(usize, usize)>,
        staff_value_range: Option<(usize, usize)>,
        note_end: usize,
        target_voice: String,
        target_staff: String,
    ) -> Option<Self> {
        if note_end > source.len() || !scalar_lane_value(&target_voice) || !scalar_lane_value(&target_staff) {
            return None;
        }
        for (start, end) in [voice_value_range, staff_value_range].into_iter().flatten() {
            if start >= end || end > source.len() || !scalar_lane_value(source.get(start..end)?) {
                return None;
            }
        }
        if staff_value_range.is_none() {
            let close_start = note_end.checked_sub("</note>".len())?;
            if source.get(close_start..note_end)? != "</note>" {
                return None;
            }
        }
        Some(Self {
            source: source.to_string(),
            source_fingerprint: fingerprint(source),
            voice_value_range,
            staff_value_range,
            note_end,
            target_voice,
            target_staff,
        })
    }

    fn bound_note_range(&self) -> Option<std::ops::Range<usize>> {
        let first = self.lane_edits()?.iter().map(|edit| edit.0).min()?;
        let prefix = self.source.get(..first)?;
        let start = prefix.rfind("<note>").into_iter().chain(prefix.rfind("<note ")).max()?;
        Some(start..self.note_end)
    }

    fn apply(self, source: &str) -> Option<String> {
        if self.source != source || self.source_fingerprint != fingerprint(source) {
            return None;
        }
        // Every range was bound against the unmodified source. Edits are
        // therefore applied in descending start order, so an earlier edit can
        // never shift the offsets of a later one when a lane value changes
        // its byte length.
        let mut edits = self.lane_edits()?;
        edits.sort_by_key(|edit| std::cmp::Reverse(edit.0));
        for pair in edits.windows(2) {
            if pair[1].1 > pair[0].0 {
                return None;
            }
        }
        let first_edit_start = edits.last()?.0;
        let length_delta = edits
            .iter()
            .map(|(start, end, text)| text.len() as isize - (end - start) as isize)
            .sum::<isize>();
        let mut out = self.source;
        for (start, end, text) in edits {
            out.replace_range(start..end, &text);
        }
        // Readback: the edited note must still be one exact note whose lane
        // values are the bound replacement values.
        let final_note_end = usize::try_from(self.note_end as isize + length_delta).ok()?;
        let before_first_edit = out.get(..first_edit_start)?;
        let note_start = before_first_edit
            .rfind("<note>")
            .into_iter()
            .chain(before_first_edit.rfind("<note "))
            .max()?;
        let note = out.get(note_start..final_note_end)?;
        if !syntactic_exact_note(note)
            || (self.voice_value_range.is_some()
                && child_text(note, "voice").as_deref() != Some(self.target_voice.trim()))
            || child_text(note, "staff").as_deref() != Some(self.target_staff.trim())
        {
            return None;
        }
        Some(out)
    }

    /// The bound lane edits of this capability in pre-state coordinates. The
    /// same list is applied by `apply` and bound into the EXACT receipt.
    fn lane_edits(&self) -> Option<Vec<(usize, usize, String)>> {
        let mut edits: Vec<(usize, usize, String)> = Vec::with_capacity(2);
        if let Some((start, end)) = self.voice_value_range {
            edits.push((start, end, self.target_voice.clone()));
        }
        if let Some((start, end)) = self.staff_value_range {
            edits.push((start, end, self.target_staff.clone()));
        } else {
            let insert_at = self.note_end.checked_sub("</note>".len())?;
            edits.push((insert_at, insert_at, format!("\n        <staff>{}</staff>", self.target_staff)));
        }
        Some(edits)
    }

    fn receipt_edits(&self) -> Option<Vec<ExactEdit>> {
        Some(
            self.lane_edits()?
                .into_iter()
                .map(|(start, end, text)| ExactEdit::bound(start, end, text))
                .collect(),
        )
    }
}

/// Sealed mint capability of `apply_bound_safe_transfer_authorized` (EXACT #3).
pub(crate) struct SafeTransferExactMint {
    _sealed: (),
}

/// Sealed mint capability of `apply_bound_staff_voice_move_authorized` (EXACT #4).
pub(crate) struct StaffVoiceMoveExactMint {
    _sealed: (),
}







pub fn apply_bound_safe_transfer_authorized(
    source: &str,
    note_start: usize,
    note_end: usize,
    insert_after_original: usize,
    restified: String,
    transferred: String,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let Some(capability) = SafeTransferCapability::issue(
        source,
        note_start,
        note_end,
        insert_after_original,
        restified,
        transferred,
    ) else {
        return (None, None);
    };
    let receipt_edits = capability.receipt_edits();
    let rest_length = capability.restified.len();
    let transferred_length = capability.transferred.len();
    let Some(out) = capability.apply(source) else {
        return (None, None);
    };
    // EXACT receipt #3, minted by this owner at the execution. It exists only
    // if both bound edits, in pre-state coordinates, reproduce the output
    // byte-exactly; otherwise no receipt exists and a Master adoption of the change is fatal.
    let mut receipt = ExactTransformationReceipt::mint_exact_receipt(
        SafeTransferExactMint { _sealed: () },
        source,
        &out,
        receipt_edits,
    );
    // Capture the two distinct physical creations from this owner's actual
    // replacement and insertion arithmetic, never by searching the output.
    let delta = rest_length as isize - (note_end - note_start) as isize;
    let insertion = if note_end <= insert_after_original {
        usize::try_from(insert_after_original as isize + delta).ok()
    } else { Some(insert_after_original) };
    if let (Some(receipt), Some(insertion)) = (receipt.as_mut(), insertion) {
        let rest_start = note_start + if insert_after_original <= note_start { transferred_length + 1 } else { 0 };
        if let Ok(births) = crate::arrangement::physical_births::PhysicalRhythmicBirths::transfer(
            &SafeTransferExactMint { _sealed: () }, source, &out, note_start..note_end,
            rest_start..rest_start + rest_length, insertion + 1..insertion + 1 + transferred_length) {
            receipt.attach_physical_births(births).expect("Actual transfer births must bind their exact execution");
        }
    }
    (Some(out), receipt)
}

pub fn apply_bound_staff_voice_move_authorized(
    source: &str,
    voice_value_range: Option<(usize, usize)>,
    staff_value_range: Option<(usize, usize)>,
    note_end: usize,
    target_voice: String,
    target_staff: String,
) -> (Option<String>, Option<ExactTransformationReceipt>) {
    let Some(capability) = StaffVoiceMoveCapability::issue(
        source,
        voice_value_range,
        staff_value_range,
        note_end,
        target_voice,
        target_staff,
    ) else {
        return (None, None);
    };
    let input = capability.bound_note_range();
    let target_staff = capability.target_staff.trim().parse::<u8>().ok();
    let receipt_edits = capability.receipt_edits();
    let Some(out) = capability.apply(source) else {
        return (None, None);
    };
    // EXACT receipt #4, minted by this owner from the same bound lane edits
    // that `apply` just applied in descending order.
    let mut receipt = receipt_edits.and_then(|edits| {
        ExactTransformationReceipt::mint_exact_receipt(
            StaffVoiceMoveExactMint { _sealed: () },
            source,
            &out,
            edits,
        )
    });
    if let (Some(receipt), Some(input), Some(target_staff)) = (&mut receipt, input, target_staff)
        && let Ok(transition) = crate::arrangement::physical_births::StaffPresentationTransition::capture(
            &StaffVoiceMoveExactMint { _sealed: () }, receipt, source, &out, input, target_staff) {
        receipt.attach_staff_transition(transition).expect("Actual staff transition must bind its execution");
    }
    (Some(out), receipt)
}

#[cfg(test)]
mod staff_voice_move_offset_tests {
    use super::StaffVoiceMoveCapability;

    fn value_range(source: &str, tag: &str) -> Option<(usize, usize)> {
        let open = format!("<{tag}>");
        let start = source.find(&open)? + open.len();
        let end = start + source[start..].find('<')?;
        Some((start, end))
    }

    fn move_lanes(source: &str, voice: &str, staff: &str) -> Option<String> {
        let note_end = source.find("</note>")? + "</note>".len();
        StaffVoiceMoveCapability::issue(
            source,
            value_range(source, "voice"),
            value_range(source, "staff"),
            note_end,
            voice.to_string(),
            staff.to_string(),
        )?
        .apply(source)
    }

    #[test]
    fn voice_length_growth_does_not_shift_the_staff_edit() {
        let source = "<note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><note/>";
        let moved = move_lanes(source, "10", "2").expect("bound move must apply");
        assert_eq!(
            moved,
            "<note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>10</voice><type>quarter</type><staff>2</staff></note><note/>"
        );
    }

    #[test]
    fn voice_length_shrink_does_not_shift_the_staff_edit() {
        let source = "<note><pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><voice>12</voice><staff>2</staff></note>";
        let moved = move_lanes(source, "5", "1").expect("bound move must apply");
        assert_eq!(
            moved,
            "<note><pitch><step>E</step><octave>5</octave></pitch><duration>1</duration><voice>5</voice><staff>1</staff></note>"
        );
    }

    #[test]
    fn missing_staff_is_inserted_after_a_grown_voice() {
        let source = "<note><pitch><step>G</step><octave>3</octave></pitch><duration>4</duration><voice>2</voice><type>half</type></note>";
        let moved = move_lanes(source, "20", "2").expect("bound move must apply");
        assert_eq!(
            moved,
            "<note><pitch><step>G</step><octave>3</octave></pitch><duration>4</duration><voice>20</voice><type>half</type>\n        <staff>2</staff></note>"
        );
    }

    #[test]
    fn equal_length_move_is_unchanged_by_the_ordering_fix() {
        let source = "<note><pitch><step>D</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><notehead>x</notehead><staff>1</staff></note>";
        let moved = move_lanes(source, "2", "2").expect("bound move must apply");
        assert_eq!(
            moved,
            "<note><pitch><step>D</step><octave>4</octave></pitch><duration>2</duration><voice>2</voice><notehead>x</notehead><staff>2</staff></note>"
        );
    }

    #[test]
    fn stale_source_is_refused() {
        let source = "<note><duration>2</duration><voice>1</voice><staff>1</staff></note>";
        let note_end = source.len();
        let capability = StaffVoiceMoveCapability::issue(
            source,
            value_range(source, "voice"),
            value_range(source, "staff"),
            note_end,
            "3".to_string(),
            "2".to_string(),
        )
        .expect("capability must issue");
        assert!(capability.apply("<note><duration>2</duration><voice>4</voice><staff>1</staff></note>").is_none());
    }
}
