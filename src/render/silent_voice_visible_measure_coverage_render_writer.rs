// v0.10.1780 FinalBeamLostBeginRecoveryAuthorityRetirement:
// The productive master-bound lost-begin recovery is retired. The exact historical
// selector survives only as a read-only detector on the persisted final artifact;
// any reappearance fails the committed-state closure instead of being repaired.
//
use super::final_beam_observation_planning_core::{
    final_beam_child_i64, final_beam_child_text, final_beam_has_child,
    final_pdf_source_higher_beam_regression_candidates,
    final_pdf_source_lost_begin_regression_candidates,
};
use super::final_notation_readback_observation_core::observe_final_notation_readback;
use super::silent_voice_coverage_planning_core::{
    plan_silent_voice_visible_measure_coverage, silent_coverage_candidate_rows,
};


use super::final_pdf_source_commit::pdf_exact_probe;

use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt, RenderTransitionReceipts, RenderDiscardReason};

pub(crate) struct SameLaneRawSpliceExactMint { _sealed: () }

fn mint_same_lane_raw_splice_receipt(
    pre: &str, start: usize, end: usize, replacement: &str, post: &str,
) -> Option<ExactTransformationReceipt> {
    let receipt = ExactTransformationReceipt::mint_exact_receipt(
        SameLaneRawSpliceExactMint { _sealed: () }, pre, post,
        vec![ExactEdit::bound(start, end, replacement.to_string())],
    );
    assert!(pre == post || receipt.is_some(), "#67 changed bytes require an EXACT receipt");
    receipt
}

fn same_lane_strict_interior_raw_splice(
    final_candidate: &str, edit_start: usize, edit_end: usize, replacement: &str,
) -> (String, Option<ExactTransformationReceipt>) {
    let mut candidate = String::with_capacity(final_candidate.len() + replacement.len());
    candidate.push_str(&final_candidate[..edit_start]);
    candidate.push_str(replacement);
    candidate.push_str(&final_candidate[edit_end..]);
    let receipt = mint_same_lane_raw_splice_receipt(final_candidate, edit_start, edit_end, replacement, &candidate);
    (candidate, receipt)
}

fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum FinalNotationCommitOutcome {
    #[default]
    NotRequested,
    Committed,
    Rejected,
}

type FinalRenderSourceFinish = Box<dyn FnOnce(&str) -> FinalRenderSourceSnapshot>;

pub(super) struct FinalRenderSourceClosure {
    pub(super) candidate_xml: Option<String>,
    pub(super) receipts: RenderTransitionReceipts,
    pub(super) finish: Option<FinalRenderSourceFinish>,
}

#[derive(Debug, Clone)]
pub(super) struct FinalRenderSourceSnapshot {
    pub(super) pipeline_line: String,
    pub(super) closed: bool,
    pub(super) committed_xml: Option<String>,
    pub(super) committed_occurrence_continuation: Option<crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation>,
    pub(super) committed_voice_plan: Option<crate::structural_write_authority::FrozenSerializedVoicePlan>,
    pub(super) primary_beam_voice_partition_changed_voice_elements: usize,
}

fn rejected_pdf_preparation(receipts: RenderTransitionReceipts) -> FinalRenderSourceClosure {
    receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
    FinalRenderSourceClosure { candidate_xml: None, receipts: RenderTransitionReceipts::new(), finish: None }
}

#[derive(Debug, Clone, Default)]
pub(super) struct SilentVoiceVisibleMeasureCoverageRenderWriteAudit {
    pub(super) pipeline_line: String,
    attempted: bool,
    selected_render_source: bool,
    write_enabled: bool,
    candidates: usize,
    measures_accepted: usize,
    forward_nodes_planned: usize,
    forward_duration_planned: i64,
    forward_nodes_accepted: usize,
    forward_duration_accepted: i64,
    visible_rests_before: usize,
    visible_rests_after: usize,
    note_count_before: usize,
    note_count_after: usize,
    parse_before_ok: bool,
    parse_after_ok: bool,
    backup_forward_before_ok: bool,
    backup_forward_after_ok: bool,
    raw_cursor_stable: bool,
    pitched_event_multiset_stable: bool,
    candidate_rows_after: usize,
    exact_candidate_delta: bool,
    candidate_accepted: bool,
    commit_outcome: FinalNotationCommitOutcome,
    candidate_music_mutation: bool,
    master_write_applied: bool,
    source_higher_beam_notes_restored: usize,
    rest_vertical_candidates: usize,
    rest_vertical_anchors_written: usize,
    rest_vertical_candidate_rows_after: usize,
    ledger_bounded_clef_boundaries_accepted: usize,
    ledger_bounded_clef_boundaries_written: usize,
    primary_beam_voice_elements_written: usize,
    primary_clef_projection_green: bool,
    primary_clef_projection_changed: bool,
    committed_state_readback_green: bool,
    committed_primary_clef_reentry_changes: usize,
    committed_rest_vertical_candidates: usize,
    committed_higher_beam_candidates: usize,
    committed_beam_lost_begin_candidates: usize,
    committed_partial_beat_lane_split_candidates: usize,
    committed_lower_dual_edge_candidates: usize,
    committed_shadow_rest_candidates: usize,
    committed_shadow_rest_rows: String,
    committed_tied_tail_accent_candidates: usize,
    committed_raw_cursor_stable: bool,
    committed_raw_cursor_green: bool,
    committed_pitched_event_multiset_stable: bool,
    verdict: String,
}

const FINAL_REST_VERTICAL_WRITE_PIPELINE_PREFIX: &str =
    "FinalRestVerticalPlacementRenderWriterStage ";

#[derive(Debug, Clone, Default)]
struct FinalRestVerticalPlacementRenderWriteAudit {
    pipeline_line: String,
    attempted: bool,
    selected_render_source: bool,
    write_enabled: bool,
    candidates: usize,
    canonical_rebind_candidates: usize,
    polyphonic_whole_rest_candidates: usize,
    anchors_accepted: usize,
    display_nodes_accepted: usize,
    parse_before_ok: bool,
    parse_after_ok: bool,
    note_count_stable: bool,
    raw_cursor_stable: bool,
    pitched_event_multiset_stable: bool,
    candidate_rows_after: usize,
    exact_candidate_delta: bool,
    candidate_accepted: bool,
    commit_outcome: FinalNotationCommitOutcome,
    candidate_music_mutation: bool,
    master_write_applied: bool,
    final_clef_bound: bool,
    verdict: String,
}

#[derive(Debug, Default)]
struct FinalOrphanChordAnchorPromotionPlan {
    edits: Vec<(usize, usize, String)>,
    anchor_promotions: usize,
    beam_completions: usize,
}

#[derive(Debug, Clone, Default)]
struct FinalOrphanChordHostRestRenderWriteAudit {
    candidates: usize,
    forward_nodes_planned: usize,
    anchor_promotions_accepted: usize,
    forward_duration_planned: i64,
    forward_nodes_accepted: usize,
    forward_duration_accepted: i64,
    chord_notes_accepted: usize,
    parse_before_ok: bool,
    parse_after_ok: bool,
    raw_cursor_stable: bool,
    pitched_payload_stable: bool,
    candidate_rows_after: usize,
    exact_candidate_delta: bool,
    candidate_accepted: bool,
    candidate_music_mutation: bool,
}

#[derive(Debug, Clone)]
struct FinalOrphanChordHostReplacement {
    start: usize,
    end: usize,
    duration: i64,
    xml: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FinalRestPlacementDecision {
    CanonicalClefRebind,
    PolyphonicWholeRestSeparation,
}

#[derive(Debug, Clone)]
struct FinalRestVerticalTarget {
    note_range: std::ops::Range<usize>,
    display_step_range: std::ops::Range<usize>,
    display_octave_range: std::ops::Range<usize>,
    display_step_before: String,
    display_octave_before: String,
    expected_step: String,
    expected_octave: String,
    decision: FinalRestPlacementDecision,
}

#[derive(Debug, Clone)]
struct FinalRestMeasureEvent {
    staff: String,
    voice: String,
    start: i64,
    end: i64,
    visible: bool,
    pitched: bool,
}

#[derive(Debug, Clone)]
struct FinalRestRawRow {
    identity: String,
    staff: String,
    voice: String,
    onset: i64,
    duration: i64,
    display_step: String,
    display_octave: String,
    canonical_anchor: String,
    opposite_anchor: String,
    polyphonic_whole_rest_anchor: String,
    measure_rest: bool,
    whole_like: bool,
    timeline_underfill: i64,
    timeline_overflow: i64,
    staff_lane_end: i64,
    overlapping_voices: usize,
    overlapping_pitched_voices: usize,
    note_range: std::ops::Range<usize>,
    display_step_range: std::ops::Range<usize>,
    display_octave_range: std::ops::Range<usize>,
}

fn final_rest_expected_anchor(clef: &str, rest_type: &str) -> String {
    if let Some(anchor) = super::rest_placement::anchor_from_musicxml(
        clef,
        rest_type,
        super::rest_placement::RestPlacementMode::Canonical,
    ) {
        return format!("{}{}", anchor.step, anchor.octave);
    }
    match rest_type.trim() {
        "whole" | "breve" | "long" | "maxima" | "32nd" | "64th" | "128th"
        | "256th" | "512th" | "1024th" => "G4".to_string(),
        _ => "E4".to_string(),
    }
}

fn final_rest_opposite_anchor(clef: &str, rest_type: &str) -> String {
    if clef == "F" {
        final_rest_expected_anchor("G", rest_type)
    } else {
        final_rest_expected_anchor("F", rest_type)
    }
}

fn final_rest_polyphonic_whole_rest_anchor(clef: &str) -> String {
    super::rest_placement::anchor_from_musicxml(
        clef,
        "whole",
        super::rest_placement::RestPlacementMode::PolyphonicWholeRest,
    )
    .map_or_else(|| final_rest_expected_anchor(clef, "whole"), |anchor| format!("{}{}", anchor.step, anchor.octave))
}

fn final_rest_element_text_range(
    xml: &str,
    element: roxmltree::Node<'_, '_>,
) -> Option<std::ops::Range<usize>> {
    let range = element.range();
    let fragment = xml.get(range.clone())?;
    let open_end = fragment.find('>')?.saturating_add(1);
    let close_start = fragment.rfind('<')?;
    (close_start >= open_end).then_some(
        range.start.saturating_add(open_end)..range.start.saturating_add(close_start),
    )
}

fn final_rest_structural_targets(xml: &str) -> Option<Vec<FinalRestVerticalTarget>> {
    let document = roxmltree::Document::parse(xml).ok()?;
    let mut rows = Vec::<FinalRestRawRow>::new();
    for part in document
        .root_element()
        .children()
        .filter(|node| node.is_element() && node.tag_name().name() == "part")
    {
        let part_id = part.attribute("id").unwrap_or("P1").to_string();
        let mut divisions = 1_i64;
        let mut beats = 4_i64;
        let mut beat_type = 4_i64;
        let mut clefs = std::collections::BTreeMap::<String, String>::new();
        for measure in part
            .children()
            .filter(|node| node.is_element() && node.tag_name().name() == "measure")
        {
            if let Some(attributes) = measure
                .children()
                .find(|node| node.is_element() && node.tag_name().name() == "attributes")
            {
                if let Some(value) = final_beam_child_text(attributes, "divisions")
                    .and_then(|value| value.parse::<i64>().ok())
                {
                    divisions = value.max(1);
                }
                if let Some(time) = attributes
                    .children()
                    .find(|node| node.is_element() && node.tag_name().name() == "time")
                {
                    beats = final_beam_child_text(time, "beats")
                        .and_then(|value| value.parse::<i64>().ok())
                        .unwrap_or(beats)
                        .max(1);
                    beat_type = final_beam_child_text(time, "beat-type")
                        .and_then(|value| value.parse::<i64>().ok())
                        .unwrap_or(beat_type)
                        .max(1);
                }
            }
            let expected = divisions.saturating_mul(beats).saturating_mul(4) / beat_type.max(1);
            let bar_token = measure.attribute("number").unwrap_or("?").to_string();
            let mut cursor = 0_i64;
            let mut previous_onset = 0_i64;
            let mut voice_end = std::collections::BTreeMap::<(String, String), i64>::new();
            let mut events = Vec::<FinalRestMeasureEvent>::new();
            let mut local_rows = Vec::<FinalRestRawRow>::new();
            let mut explicit_lane_identity_complete = true;
            for node in measure.children().filter(|node| node.is_element()) {
                if node.tag_name().name() == "attributes" {
                    for clef in node
                        .children()
                        .filter(|child| child.is_element() && child.tag_name().name() == "clef")
                    {
                        let staff = clef.attribute("number").unwrap_or("1").to_string();
                        if let Some(sign) = final_beam_child_text(clef, "sign") {
                            clefs.insert(staff, sign);
                        }
                    }
                } else if node.tag_name().name() == "backup" {
                    cursor = cursor.saturating_sub(final_beam_child_i64(node, "duration"));
                } else if node.tag_name().name() == "forward" {
                    cursor = cursor.saturating_add(final_beam_child_i64(node, "duration"));
                } else if node.tag_name().name() == "note" {
                    let chord = final_beam_has_child(node, "chord");
                    let duration = final_beam_child_i64(node, "duration").max(0);
                    let onset = if chord { previous_onset } else { cursor };
                    if !chord {
                        previous_onset = onset;
                        cursor = cursor.saturating_add(duration);
                    }
                    let Some(staff) = final_beam_child_text(node, "staff") else {
                        explicit_lane_identity_complete = false;
                        continue;
                    };
                    let Some(voice) = final_beam_child_text(node, "voice") else {
                        explicit_lane_identity_complete = false;
                        continue;
                    };
                    let visible = node.attribute("print-object") != Some("no");
                    events.push(FinalRestMeasureEvent {
                        staff: staff.clone(),
                        voice: voice.clone(),
                        start: onset,
                        end: onset.saturating_add(duration),
                        visible,
                        pitched: !chord && final_beam_has_child(node, "pitch"),
                    });
                    voice_end
                        .entry((staff.clone(), voice.clone()))
                        .and_modify(|value| {
                            *value = (*value).max(onset.saturating_add(duration));
                        })
                        .or_insert(onset.saturating_add(duration));
                    let Some(rest) = node
                        .children()
                        .find(|child| child.is_element() && child.tag_name().name() == "rest")
                    else {
                        continue;
                    };
                    if !visible || duration <= 0 || chord {
                        continue;
                    }
                    let Some(display_step_node) = rest.children().find(|child| {
                        child.is_element() && child.tag_name().name() == "display-step"
                    }) else {
                        continue;
                    };
                    let Some(display_octave_node) = rest.children().find(|child| {
                        child.is_element() && child.tag_name().name() == "display-octave"
                    }) else {
                        continue;
                    };
                    let display_step = display_step_node.text().unwrap_or("").trim().to_string();
                    let display_octave = display_octave_node.text().unwrap_or("").trim().to_string();
                    if display_step.is_empty() || display_octave.is_empty() {
                        continue;
                    }
                    let Some(display_step_range) =
                        final_rest_element_text_range(xml, display_step_node)
                    else {
                        continue;
                    };
                    let Some(display_octave_range) =
                        final_rest_element_text_range(xml, display_octave_node)
                    else {
                        continue;
                    };
                    let measure_rest = matches!(rest.attribute("measure"), Some("yes" | "true"));
                    let rest_type = match final_beam_child_text(node, "type") {
                        Some(rest_type) => rest_type,
                        None if measure_rest => "whole".to_string(),
                        None => continue,
                    };
                    let clef = clefs
                        .get(&staff)
                        .cloned()
                        .unwrap_or_else(|| "unknown".to_string());
                    if !matches!(clef.as_str(), "G" | "F" | "C") {
                        continue;
                    }
                    let canonical_anchor = final_rest_expected_anchor(&clef, &rest_type);
                    let opposite_anchor = final_rest_opposite_anchor(&clef, &rest_type);
                    let polyphonic_whole_rest_anchor =
                        final_rest_polyphonic_whole_rest_anchor(&clef);
                    let whole_like = matches!(
                        rest_type.trim(),
                        "whole" | "breve" | "long" | "maxima"
                    );
                    let dots = node
                        .children()
                        .filter(|child| child.is_element() && child.tag_name().name() == "dot")
                        .count();
                    let identity = format!(
                        "{part_id}|{bar_token}|{staff}|{voice}|{onset}|{duration}|{rest_type}|{dots}"
                    );
                    local_rows.push(FinalRestRawRow {
                        identity,
                        staff,
                        voice,
                        onset,
                        duration,
                        display_step,
                        display_octave,
                        canonical_anchor,
                        opposite_anchor,
                        polyphonic_whole_rest_anchor,
                        measure_rest,
                        whole_like,
                        timeline_underfill: 0,
                        timeline_overflow: 0,
                        staff_lane_end: 0,
                        overlapping_voices: 0,
                        overlapping_pitched_voices: 0,
                        note_range: node.range(),
                        display_step_range,
                        display_octave_range,
                    });
                }
            }
            if !explicit_lane_identity_complete {
                local_rows.clear();
            }
            for row in &mut local_rows {
                let end = voice_end
                    .get(&(row.staff.clone(), row.voice.clone()))
                    .copied()
                    .unwrap_or(0);
                row.timeline_underfill = (expected - end).max(0);
                row.timeline_overflow = (end - expected).max(0);
                let row_end = row.onset.saturating_add(row.duration);
                row.staff_lane_end = events
                    .iter()
                    .filter(|event| event.staff == row.staff)
                    .map(|event| event.end)
                    .max()
                    .unwrap_or(row_end);
                row.overlapping_voices = events
                    .iter()
                    .filter(|event| {
                        event.visible
                            && event.staff == row.staff
                            && event.voice != row.voice
                            && event.end > row.onset
                            && event.start < row_end
                    })
                    .map(|event| event.voice.as_str())
                    .collect::<std::collections::BTreeSet<_>>()
                    .len();
                row.overlapping_pitched_voices = events
                    .iter()
                    .filter(|event| {
                        event.visible
                            && event.pitched
                            && event.staff == row.staff
                            && event.voice != row.voice
                            && event.end > row.onset
                            && event.start < row_end
                    })
                    .map(|event| event.voice.as_str())
                    .collect::<std::collections::BTreeSet<_>>()
                    .len();
            }
            rows.extend(local_rows);
        }
    }
    let mut identity_counts = std::collections::BTreeMap::<String, usize>::new();
    for row in &rows {
        *identity_counts.entry(row.identity.clone()).or_insert(0) += 1;
    }
    Some(
        rows.into_iter()
            .filter_map(|row| {
                if identity_counts.get(&row.identity) != Some(&1) {
                    return None;
                }
                let display = format!("{}{}", row.display_step, row.display_octave);
                let polyphonic_whole_rest = row.staff == "2"
                    && row.onset == 0
                    && row.duration >= row.staff_lane_end
                    && (row.measure_rest || row.whole_like)
                    && row.overlapping_pitched_voices > 0
                    && display != row.polyphonic_whole_rest_anchor;
                let stale_clef_anchor = row.timeline_underfill == 0
                    && row.timeline_overflow == 0
                    && row.overlapping_voices == 0
                    && display == row.opposite_anchor
                    && display != row.canonical_anchor;
                let (expected_anchor, decision) = if polyphonic_whole_rest {
                    (
                        row.polyphonic_whole_rest_anchor,
                        FinalRestPlacementDecision::PolyphonicWholeRestSeparation,
                    )
                } else if stale_clef_anchor {
                    (
                        row.canonical_anchor,
                        FinalRestPlacementDecision::CanonicalClefRebind,
                    )
                } else {
                    return None;
                };
                let mut chars = expected_anchor.chars();
                let expected_step = chars.next()?.to_string();
                let expected_octave = chars.as_str().to_string();
                (!expected_octave.is_empty()).then_some(FinalRestVerticalTarget {
                    note_range: row.note_range,
                    display_step_range: row.display_step_range,
                    display_octave_range: row.display_octave_range,
                    display_step_before: row.display_step,
                    display_octave_before: row.display_octave,
                    expected_step,
                    expected_octave,
                    decision,
                })
            })
            .collect(),
    )
}

fn final_rest_replacement_note(
    xml: &str,
    target: &FinalRestVerticalTarget,
) -> Option<(String, usize)> {
    if target.note_range.end > xml.len() || target.note_range.start >= target.note_range.end {
        return None;
    }
    let mut replacements = Vec::<(std::ops::Range<usize>, &str)>::new();
    if target.display_step_before != target.expected_step {
        replacements.push((
            target.display_step_range.clone(),
            target.expected_step.as_str(),
        ));
    }
    if target.display_octave_before != target.expected_octave {
        replacements.push((
            target.display_octave_range.clone(),
            target.expected_octave.as_str(),
        ));
    }
    if replacements.is_empty() {
        return None;
    }
    replacements.sort_by_key(|(range, _)| range.start);
    let mut replacement_note = String::with_capacity(target.note_range.len());
    let mut cursor = target.note_range.start;
    for (range, replacement) in &replacements {
        if range.start < cursor
            || range.end > target.note_range.end
            || range.start >= range.end
        {
            return None;
        }
        replacement_note.push_str(xml.get(cursor..range.start)?);
        replacement_note.push_str(replacement);
        cursor = range.end;
    }
    replacement_note.push_str(xml.get(cursor..target.note_range.end)?);
    Some((replacement_note, replacements.len()))
}

fn stage_final_rest_vertical_placement_candidate(
    xml: &str,
    write_enabled: bool,
) -> (String, FinalRestVerticalPlacementRenderWriteAudit, RenderTransitionReceipts) {
    let mut receipts = RenderTransitionReceipts::new();
    let parse_before_ok = roxmltree::Document::parse(xml).is_ok();
    let targets = if parse_before_ok {
        final_rest_structural_targets(xml).unwrap_or_default()
    } else {
        Vec::new()
    };
    let mut audit = FinalRestVerticalPlacementRenderWriteAudit {
        attempted: write_enabled,
        selected_render_source: true,
        write_enabled,
        candidates: targets.len(),
        canonical_rebind_candidates: targets
            .iter()
            .filter(|target| {
                target.decision == FinalRestPlacementDecision::CanonicalClefRebind
            })
            .count(),
        polyphonic_whole_rest_candidates: targets
            .iter()
            .filter(|target| {
                target.decision == FinalRestPlacementDecision::PolyphonicWholeRestSeparation
            })
            .count(),
        candidate_rows_after: targets.len(),
        parse_before_ok,
        verdict: "final_clef_bound_rest_placement_no_write".to_string(),
        ..FinalRestVerticalPlacementRenderWriteAudit::default()
    };
    if !write_enabled {
        audit.verdict = "final_clef_bound_rest_placement_write_disabled".to_string();
        finalize_final_rest_vertical_placement_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (xml.to_string(), audit, RenderTransitionReceipts::new());
    }
    if targets.is_empty() {
        audit.parse_after_ok = parse_before_ok;
        audit.note_count_stable = parse_before_ok;
        audit.raw_cursor_stable = parse_before_ok;
        audit.pitched_event_multiset_stable = parse_before_ok;
        audit.exact_candidate_delta = parse_before_ok;
        audit.verdict = "final_clef_bound_rest_placement_no_candidate".to_string();
        finalize_final_rest_vertical_placement_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (xml.to_string(), audit, RenderTransitionReceipts::new());
    }

    let before_observation = observe_final_notation_readback(xml);
    let mut replacements = Vec::<(usize, usize, String)>::with_capacity(targets.len());
    for target in &targets {
        let Some((replacement_note, replaced_nodes)) = final_rest_replacement_note(xml, target)
        else {
            audit.verdict = "final_clef_bound_rest_placement_invalid_target".to_string();
            finalize_final_rest_vertical_placement_render_write_audit(&mut audit);
            receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (xml.to_string(), audit, RenderTransitionReceipts::new());
        };
        audit.display_nodes_accepted = audit
            .display_nodes_accepted
            .saturating_add(replaced_nodes);
        replacements.push((
            target.note_range.start,
            target.note_range.end,
            replacement_note,
        ));
    }
    replacements.sort_by_key(|(start, end, _)| (*start, *end));
    let (candidate_xml, receipt) =
        crate::structural_write_authority::apply_final_rest_glyph_height_authorized(
            xml,
            replacements,
        );
    receipts.push_exact(receipt);
    let Some(candidate_xml) = candidate_xml else {
        audit.verdict = "final_clef_bound_rest_placement_authority_rejected".to_string();
        finalize_final_rest_vertical_placement_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (xml.to_string(), audit, RenderTransitionReceipts::new());
    };

    let after_observation = observe_final_notation_readback(&candidate_xml);
    audit.parse_after_ok = after_observation.parse_ok;
    audit.note_count_stable = before_observation.note_count == after_observation.note_count;
    audit.raw_cursor_stable =
        before_observation.raw_cursor_signature == after_observation.raw_cursor_signature;
    audit.pitched_event_multiset_stable = before_observation.pitched_timeline_signature
        == after_observation.pitched_timeline_signature;
    audit.candidate_rows_after = final_rest_structural_targets(&candidate_xml)
        .map_or(usize::MAX, |rows| rows.len());
    audit.exact_candidate_delta = audit.parse_before_ok
        && audit.parse_after_ok
        && audit.note_count_stable
        && audit.raw_cursor_stable
        && audit.pitched_event_multiset_stable
        && audit.candidate_rows_after == 0;
    if audit.exact_candidate_delta {
        audit.candidate_accepted = true;
        audit.anchors_accepted = targets.len();
        audit.candidate_music_mutation = false;
        audit.verdict = "final_clef_bound_rest_placement_candidate_accepted".to_string();
        crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
            "FinalRestPlacementSelected",
            &candidate_xml,
        );
        finalize_final_rest_vertical_placement_render_write_audit(&mut audit);
        (candidate_xml, audit, receipts)
    } else {
        audit.verdict = "final_clef_bound_rest_placement_rejected_by_readback".to_string();
        finalize_final_rest_vertical_placement_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        (xml.to_string(), audit, RenderTransitionReceipts::new())
    }
}

fn finalize_final_rest_vertical_placement_render_write_audit(
    audit: &mut FinalRestVerticalPlacementRenderWriteAudit,
) {
    let committed = audit.commit_outcome == FinalNotationCommitOutcome::Committed && audit.candidate_accepted;
    let sink_rejected = audit.commit_outcome == FinalNotationCommitOutcome::Rejected && audit.candidate_accepted;
    let anchors_written = if committed { audit.anchors_accepted } else { 0 };
    let display_nodes_replaced = if committed {
        audit.display_nodes_accepted
    } else {
        0
    };
    let candidate_rows_after = if sink_rejected {
        audit.candidates
    } else {
        audit.candidate_rows_after
    };
    let exact_candidate_delta = audit.exact_candidate_delta && !sink_rejected;
    let verdict = if sink_rejected {
        "final_rest_vertical_placement_authorized_sink_rejected"
    } else if committed {
        "final_clef_bound_rest_placement_written_render_xml_only"
    } else {
        audit.verdict.as_str()
    };
    audit.pipeline_line = format!(
        "{}v{}: Attempted {}; SelectedRenderSource {}; FinalClefBound {}; WriteEnabled {}; Candidates {}; CanonicalRebindCandidates {}; PolyphonicWholeRestCandidates {}; AnchorsWritten {}; DisplayNodesReplaced {}; ParseBeforeAfter {}/{}; NoteCountStable {}; RawCursorStable {}; PitchedEventMultisetStable {}; CandidateRowsAfter {}; ExactCandidateDelta {}; SelectorPolicy final_clef_bound_typed_canonical_rebind_or_polyphonic_whole_rest_separation_no_absolute_measure_or_etude_selector; RenderXmlWrite {}; NotationMutation {}; MusicMutation {}; MasterApply {}; Urteil {}.",
        FINAL_REST_VERTICAL_WRITE_PIPELINE_PREFIX,
        env!("CARGO_PKG_VERSION"),
        yes_no(audit.attempted),
        yes_no(audit.selected_render_source),
        yes_no(audit.final_clef_bound),
        yes_no(audit.write_enabled),
        audit.candidates,
        audit.canonical_rebind_candidates,
        audit.polyphonic_whole_rest_candidates,
        anchors_written,
        display_nodes_replaced,
        yes_no(audit.parse_before_ok),
        yes_no(audit.parse_after_ok),
        yes_no(audit.note_count_stable),
        yes_no(audit.raw_cursor_stable),
        yes_no(audit.pitched_event_multiset_stable),
        candidate_rows_after,
        yes_no(exact_candidate_delta),
        yes_no(committed),
        yes_no(committed),
        yes_no(committed && audit.candidate_music_mutation),
        yes_no(audit.master_write_applied),
        verdict,
    );
}

fn final_beam_lost_begin_committed_regression_line(
    master_source_bound: bool,
    write_enabled: bool,
    candidates: Option<usize>,
) -> String {
    let candidate_count = candidates.unwrap_or(0);
    let verdict = if !master_source_bound {
        "final_beam_lost_begin_committed_gate_master_source_missing"
    } else if candidates.is_none() {
        "final_beam_lost_begin_committed_gate_source_parse_failed"
    } else if candidate_count == 0 {
        "final_beam_lost_begin_retired_recovery_committed_state_green"
    } else {
        "final_beam_lost_begin_retired_recovery_regression_detected"
    };
    format!(
        "FinalBeamLostBeginRenderWriterStage v{}: Attempted {}; SelectedRenderSource {}; MasterSourceBound {}; WriteEnabled {}; RecoveryEnabled nein; Candidates {}; NotesWritten 0; CandidateRowsAfter {}; RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein; MasterApply nein; ReleaseVeto {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        yes_no(write_enabled),
        "ja",
        yes_no(master_source_bound),
        yes_no(write_enabled),
        candidate_count,
        candidate_count,
        yes_no(candidates.is_none_or(|value| value > 0)),
        verdict,
    )
}

fn final_beam_master_path(render_path: &std::path::Path) -> Option<std::path::PathBuf> {
    let filename = render_path.file_name()?.to_str()?;
    let stem = filename.strip_suffix(".render.musicxml")?;
    let mut master = render_path.to_path_buf();
    master.set_file_name(format!("{stem}.musicxml"));
    Some(master)
}


fn final_orphan_chord_child_text(
    node: roxmltree::Node<'_, '_>,
    tag: &str,
) -> String {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == tag)
        .and_then(|child| child.text())
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn final_orphan_chord_duration(node: roxmltree::Node<'_, '_>) -> i64 {
    final_orphan_chord_child_text(node, "duration")
        .parse::<i64>()
        .unwrap_or(0)
        .max(0)
}

fn final_orphan_chord_direct_elements<'a, 'input>(
    measure: roxmltree::Node<'a, 'input>,
) -> Vec<roxmltree::Node<'a, 'input>> {
    measure.children().filter(|child| child.is_element()).collect()
}

fn final_orphan_chord_hidden_rest_host_xml(
    duration: i64,
    voice: &str,
    staff: &str,
) -> Option<String> {
    if duration <= 0
        || voice.is_empty()
        || staff.is_empty()
        || !voice.chars().all(|ch| ch.is_ascii_digit())
        || !matches!(staff, "1" | "2")
    {
        return None;
    }
    Some(format!(
        "<note print-object=\"no\"><rest/><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>"
    ))
}

fn collect_final_orphan_chord_host_replacements(
    render_xml: &str,
) -> Option<(Vec<FinalOrphanChordHostReplacement>, usize)> {
    let render_document = roxmltree::Document::parse(render_xml).ok()?;
    let mut replacements = Vec::<FinalOrphanChordHostReplacement>::new();
    let mut chord_notes = 0_usize;
    for part in render_document
        .descendants()
        .filter(|node| node.is_element() && node.tag_name().name() == "part")
    {
        for measure in part.children().filter(|node| {
            node.is_element() && node.tag_name().name() == "measure"
        }) {
            let elements = final_orphan_chord_direct_elements(measure);
            for (index, element) in elements.iter().copied().enumerate() {
                if element.tag_name().name() != "forward" {
                    continue;
                }
                let duration = final_orphan_chord_duration(element);
                if duration <= 0 {
                    continue;
                }

                let mut render_chord_count = 0_usize;
                let mut group_voice: Option<String> = None;
                let mut group_staff: Option<String> = None;
                let mut group_green = true;
                for following in elements.iter().copied().skip(index + 1) {
                    if following.tag_name().name() != "note"
                        || !following.children().any(|child| {
                            child.is_element() && child.tag_name().name() == "chord"
                        })
                    {
                        break;
                    }
                    let pitched = following.children().any(|child| {
                        child.is_element() && child.tag_name().name() == "pitch"
                    });
                    let chord_duration = final_orphan_chord_duration(following);
                    let voice = final_orphan_chord_child_text(following, "voice");
                    let staff = final_orphan_chord_child_text(following, "staff");
                    if !pitched
                        || chord_duration <= 0
                        || voice.is_empty()
                        || staff.is_empty()
                    {
                        group_green = false;
                        break;
                    }
                    if group_voice.as_ref().is_some_and(|value| value != &voice)
                        || group_staff.as_ref().is_some_and(|value| value != &staff)
                    {
                        group_green = false;
                        break;
                    }
                    group_voice.get_or_insert(voice);
                    group_staff.get_or_insert(staff);
                    render_chord_count = render_chord_count.saturating_add(1);
                }
                if !group_green || render_chord_count == 0 {
                    continue;
                }

                let voice = group_voice?;
                let staff = group_staff?;
                let carrier_voice = final_orphan_chord_child_text(element, "voice");
                let carrier_staff = final_orphan_chord_child_text(element, "staff");
                if (!carrier_voice.is_empty() && carrier_voice != voice)
                    || (!carrier_staff.is_empty() && carrier_staff != staff)
                {
                    continue;
                }
                let Some(host_xml) =
                    final_orphan_chord_hidden_rest_host_xml(duration, &voice, &staff)
                else {
                    continue;
                };
                let render_range = element.range();
                replacements.push(FinalOrphanChordHostReplacement {
                    start: render_range.start,
                    end: render_range.end,
                    duration,
                    xml: host_xml,
                });
                chord_notes = chord_notes.saturating_add(render_chord_count);
            }
        }
    }
    replacements.sort_by_key(|left| std::cmp::Reverse(left.start));
    Some((replacements, chord_notes))
}

fn stage_final_orphan_chord_host_rest_candidate(
    render_xml: &str,
    write_enabled: bool,
) -> (String, FinalOrphanChordHostRestRenderWriteAudit, RenderTransitionReceipts) {
    let mut receipts = RenderTransitionReceipts::new();
    let before_observation = observe_final_notation_readback(render_xml);
    let parse_before_ok = before_observation.parse_ok;
    let backup_forward_before_ok = before_observation.backup_forward_ok;
    let visible_rests_before = before_observation.visible_rests;

    // r1017/r02: keep the inverse orphan-host selector inside this already-existing
    // stage. It intentionally contains no direct String mutation primitive; the
    // actual byte write remains owned by ChordGestureDistributionCapability.
    let collect_anchor_promotions =
        |source: &str| -> Option<FinalOrphanChordAnchorPromotionPlan> {
            let document = roxmltree::Document::parse(source).ok()?;
            let mut edits = Vec::<(usize, usize, String)>::new();
            let mut promotions = 0_usize;
            let mut beam_completions = 0_usize;
            for measure in document
                .descendants()
                .filter(|node| node.is_element() && node.tag_name().name() == "measure")
            {
                let elements = final_orphan_chord_direct_elements(measure);
                for (index, pair) in elements.windows(2).enumerate() {
                    let host = pair[0];
                    let member = pair[1];
                if host.tag_name().name() != "note" || member.tag_name().name() != "note" {
                    continue;
                }
                let member_xml = &source[member.range()];
                let host_hidden_rest = host.children().any(|child| {
                    child.is_element() && child.tag_name().name() == "rest"
                }) && !host.children().any(|child| {
                    child.is_element() && child.tag_name().name() == "pitch"
                }) && !host.children().any(|child| {
                    child.is_element() && child.tag_name().name() == "chord"
                }) && host.attribute("print-object") == Some("no")
                    && host.attribute("print-spacing") == Some("no");
                let member_visible_pitched_chord = member.children().any(|child| {
                    child.is_element() && child.tag_name().name() == "chord"
                }) && member.children().any(|child| {
                    child.is_element() && child.tag_name().name() == "pitch"
                }) && member.attribute("print-object") != Some("no")
                    && member.attribute("print-spacing") != Some("no");
                if !host_hidden_rest || !member_visible_pitched_chord {
                    continue;
                }
                let duration = final_orphan_chord_duration(host);
                let member_duration = final_orphan_chord_duration(member);
                let voice = final_orphan_chord_child_text(host, "voice");
                let staff = final_orphan_chord_child_text(host, "staff");
                if duration <= 0
                    || duration != member_duration
                    || voice.is_empty()
                    || staff.is_empty()
                    || voice != final_orphan_chord_child_text(member, "voice")
                    || staff != final_orphan_chord_child_text(member, "staff")
                {
                    continue;
                }
                    let chord_start = member_xml.find("<chord")?;
                    let chord_end = chord_start + member_xml[chord_start..].find('>')? + 1;
                    let plain_promoted = format!(
                        "{}{}",
                        &member_xml[..chord_start],
                        &member_xml[chord_end..]
                    );
                    let mut promoted = plain_promoted.clone();
                    let mut beam_rewrite = None;

                    // r1019: if the promoted 16th is the missing metric head of an
                    // already serialized 16th+eighth primary chain, promote the
                    // beam ownership with the anchor. This is deliberately bound
                    // to the existing orphan-host promotion and does not touch
                    // unrelated cross-voice beam groups.
                    let member_is_sixteenth =
                        final_orphan_chord_child_text(member, "type") == "16th";
                    let member_has_beam = member.children().any(|child| {
                        child.is_element() && child.tag_name().name() == "beam"
                    });
                    let following_notes = elements
                        .iter()
                        .skip(index + 2)
                        .filter(|node| {
                            node.tag_name().name() == "note"
                                && !node.children().any(|child| {
                                    child.is_element() && child.tag_name().name() == "chord"
                                })
                        })
                        .take(2)
                        .copied()
                        .collect::<Vec<_>>();
                    if member_is_sixteenth && !member_has_beam && following_notes.len() == 2 {
                        let bridge = following_notes[0];
                        let tail = following_notes[1];
                        let bridge_xml = &source[bridge.range()];
                        let tail_xml = &source[tail.range()];
                        let bridge_visible_pitch = bridge.children().any(|child| {
                            child.is_element() && child.tag_name().name() == "pitch"
                        }) && !bridge.children().any(|child| {
                            child.is_element() && child.tag_name().name() == "rest"
                        }) && bridge.attribute("print-object") != Some("no")
                            && bridge.attribute("print-spacing") != Some("no");
                        let tail_visible_pitch = tail.children().any(|child| {
                            child.is_element() && child.tag_name().name() == "pitch"
                        }) && !tail.children().any(|child| {
                            child.is_element() && child.tag_name().name() == "rest"
                        }) && tail.attribute("print-object") != Some("no")
                            && tail.attribute("print-spacing") != Some("no");
                        let primary_begin = "<beam number=\"1\">begin</beam>";
                        let primary_end = "<beam number=\"1\">end</beam>";
                        let bridge_has_secondary = bridge_xml.contains("<beam number=\"2\">");
                        let tail_has_secondary = tail_xml.contains("<beam number=\"2\">");
                        let exact_metric_tail = bridge_visible_pitch
                            && tail_visible_pitch
                            && final_orphan_chord_duration(bridge) == duration
                            && final_orphan_chord_duration(tail) == duration.saturating_mul(2)
                            && final_orphan_chord_child_text(bridge, "type") == "16th"
                            && final_orphan_chord_child_text(tail, "type") == "eighth"
                            && final_orphan_chord_child_text(bridge, "staff") == staff
                            && final_orphan_chord_child_text(tail, "staff") == staff
                            && final_orphan_chord_child_text(bridge, "voice") != voice
                            && final_orphan_chord_child_text(tail, "voice") == voice
                            && bridge_xml.matches(primary_begin).count() == 1
                            && tail_xml.matches(primary_end).count() == 1
                            && !bridge_has_secondary
                            && !tail_has_secondary;
                        if exact_metric_tail {
                            let close = plain_promoted.rfind("</note>")?;
                            promoted = format!(
                                "{}\n    <beam number=\"1\">begin</beam>\n    <beam number=\"2\">begin</beam>{}",
                                &plain_promoted[..close],
                                &plain_promoted[close..]
                            );
                            let continued = bridge_xml.replacen(
                                primary_begin,
                                "<beam number=\"1\">continue</beam>",
                                1,
                            );
                            let close = continued.rfind("</note>")?;
                            let continued = format!(
                                "{}\n    <beam number=\"2\">end</beam>{}",
                                &continued[..close],
                                &continued[close..]
                            );
                            beam_rewrite = Some((bridge.range().start, bridge.range().end, continued));
                            beam_completions = beam_completions.saturating_add(1);
                        }
                    }

                    edits.push((host.range().start, host.range().end, String::new()));
                    edits.push((member.range().start, member.range().end, promoted));
                    if let Some(beam_rewrite) = beam_rewrite {
                        edits.push(beam_rewrite);
                    }
                    promotions = promotions.saturating_add(1);
                }
            }
            Some(FinalOrphanChordAnchorPromotionPlan {
                edits,
                anchor_promotions: promotions,
                beam_completions,
            })
        };

    let anchor_promotion_plan = if parse_before_ok {
        collect_anchor_promotions(render_xml).unwrap_or_default()
    } else {
        FinalOrphanChordAnchorPromotionPlan::default()
    };
    let (initial_replacements, initial_chord_notes) = if parse_before_ok {
        collect_final_orphan_chord_host_replacements(render_xml).unwrap_or_default()
    } else {
        (Vec::new(), 0)
    };
    let mut audit = FinalOrphanChordHostRestRenderWriteAudit {
        candidates: initial_replacements
            .len()
            .saturating_add(anchor_promotion_plan.anchor_promotions),
        forward_nodes_planned: initial_replacements.len(),
        anchor_promotions_accepted: 0,
        forward_duration_planned: initial_replacements
            .iter()
            .map(|replacement| replacement.duration)
            .sum(),
        forward_nodes_accepted: 0,
        forward_duration_accepted: 0,
        chord_notes_accepted: 0,
        parse_before_ok,
        parse_after_ok: parse_before_ok,
        raw_cursor_stable: true,
        pitched_payload_stable: true,
        candidate_rows_after: 0,
        exact_candidate_delta: false,
        candidate_accepted: false,
        candidate_music_mutation: false,
    };
    audit.candidate_rows_after = audit.candidates;

    if audit.candidates == 0 || !parse_before_ok || !backup_forward_before_ok {
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (render_xml.to_string(), audit, RenderTransitionReceipts::new());
    }
    if !write_enabled {
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (render_xml.to_string(), audit, RenderTransitionReceipts::new());
    }

    let mut candidate_xml = render_xml.to_string();
    if anchor_promotion_plan.anchor_promotions > 0 {
        let expected_edit_count = anchor_promotion_plan.edits.len();
        let expected_authorized_edit_count = anchor_promotion_plan
            .anchor_promotions
            .saturating_mul(2)
            .saturating_add(anchor_promotion_plan.beam_completions);
        let (promotion, receipt) =
            crate::structural_write_authority::apply_chord_gesture_distribution_batch_authorized(
                &candidate_xml,
                anchor_promotion_plan.edits,
            );
        receipts.push_exact(receipt);
        let Some((promoted_xml, changed_edits)) = promotion else {
            receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (render_xml.to_string(), audit, RenderTransitionReceipts::new());
        };
        if changed_edits != expected_edit_count || changed_edits != expected_authorized_edit_count {
            receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (render_xml.to_string(), audit, RenderTransitionReceipts::new());
        }
        candidate_xml = promoted_xml;
    }

    let (replacements, chord_notes) =
        collect_final_orphan_chord_host_replacements(&candidate_xml).unwrap_or_default();
    if replacements.len() != initial_replacements.len() || chord_notes != initial_chord_notes {
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (render_xml.to_string(), audit, RenderTransitionReceipts::new());
    }
    let forward_duration_planned = replacements
        .iter()
        .map(|replacement| replacement.duration)
        .sum::<i64>();
    if forward_duration_planned != audit.forward_duration_planned {
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (render_xml.to_string(), audit, RenderTransitionReceipts::new());
    }
    audit.forward_nodes_planned = replacements.len();

    if !replacements.is_empty() {
        let authorized_replacements = replacements
            .iter()
            .map(|replacement| {
                (
                    replacement.start,
                    replacement.end,
                    replacement.xml.clone(),
                )
            })
            .collect::<Vec<_>>();
        let (with_rest_hosts, receipt) =
            crate::structural_write_authority::apply_strict_staff_gap_materialization_authorized(
                &candidate_xml,
                authorized_replacements,
            );
        receipts.push_exact(receipt);
        let Some(with_rest_hosts) = with_rest_hosts else {
            receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (render_xml.to_string(), audit, RenderTransitionReceipts::new());
        };
        candidate_xml = with_rest_hosts;
    }

    let after_observation = observe_final_notation_readback(&candidate_xml);
    let backup_forward_after_ok = after_observation.backup_forward_ok;
    let parse_after_ok = after_observation.parse_ok;
    let visible_rests_after = after_observation.visible_rests;
    let remaining_forward_rows = collect_final_orphan_chord_host_replacements(&candidate_xml)
        .map_or(replacements.len(), |(rows, _)| rows.len());
    let remaining_anchor_promotions = collect_anchor_promotions(&candidate_xml).map_or(
        anchor_promotion_plan.anchor_promotions,
        |plan| plan.anchor_promotions,
    );
    let candidate_rows_after = remaining_forward_rows
        .saturating_add(remaining_anchor_promotions);

    audit.parse_after_ok = parse_after_ok;
    audit.raw_cursor_stable = before_observation.raw_cursor_signature
        == after_observation.raw_cursor_signature;
    let pitched_payload_exact = before_observation.pitched_payload_signature
        == after_observation.pitched_payload_signature;
    let before_payload_without_chord = before_observation
        .pitched_payload_signature
        .iter()
        .map(|row| {
            row.rsplit_once('|')
                .map_or_else(|| row.to_string(), |(payload, _)| payload.to_string())
        })
        .collect::<Vec<_>>();
    let after_payload_without_chord = after_observation
        .pitched_payload_signature
        .iter()
        .map(|row| {
            row.rsplit_once('|')
                .map_or_else(|| row.to_string(), |(payload, _)| payload.to_string())
        })
        .collect::<Vec<_>>();
    let before_chord_members = before_observation
        .pitched_payload_signature
        .iter()
        .filter(|row| row.ends_with("|true"))
        .count();
    let after_chord_members = after_observation
        .pitched_payload_signature
        .iter()
        .filter(|row| row.ends_with("|true"))
        .count();
    let exact_anchor_promotion_payload = anchor_promotion_plan.anchor_promotions > 0
        && before_payload_without_chord == after_payload_without_chord
        && before_chord_members
            == after_chord_members.saturating_add(anchor_promotion_plan.anchor_promotions);
    audit.pitched_payload_stable = pitched_payload_exact || exact_anchor_promotion_payload;
    audit.candidate_rows_after = candidate_rows_after;
    audit.exact_candidate_delta = visible_rests_after == visible_rests_before
        && after_observation
            .note_count
            .saturating_add(anchor_promotion_plan.anchor_promotions)
            == before_observation
                .note_count
                .saturating_add(replacements.len())
        && candidate_rows_after == 0;

    let green = audit.parse_before_ok
        && audit.parse_after_ok
        && backup_forward_before_ok
        && backup_forward_after_ok
        && audit.raw_cursor_stable
        && audit.pitched_payload_stable
        && audit.exact_candidate_delta;
    if green {
        audit.candidate_accepted = true;
        audit.anchor_promotions_accepted = anchor_promotion_plan.anchor_promotions;
        audit.forward_nodes_accepted = replacements.len();
        audit.forward_duration_accepted = audit.forward_duration_planned;
        audit.chord_notes_accepted =
            chord_notes.saturating_add(anchor_promotion_plan.anchor_promotions);
        audit.candidate_music_mutation = true;
        (candidate_xml, audit, receipts)
    } else {
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        (render_xml.to_string(), audit, RenderTransitionReceipts::new())
    }
}
fn apply_silent_voice_visible_measure_coverage_render_write(
    xml: &str,
    write_enabled: bool,
    frozen_voice_plan: Option<&crate::structural_write_authority::FrozenSerializedVoicePlan>,
) -> (String, SilentVoiceVisibleMeasureCoverageRenderWriteAudit, RenderTransitionReceipts) {
    let mut receipts = RenderTransitionReceipts::new();
    let before_observation = observe_final_notation_readback(xml);
    let backup_forward_before_ok = before_observation.backup_forward_ok;
    let parse_before_ok = before_observation.parse_ok;
    let visible_rests_before = before_observation.visible_rests;
    let plan = plan_silent_voice_visible_measure_coverage(
        xml,
        parse_before_ok && backup_forward_before_ok,
    );
    let candidates = plan.candidates;
    let raw_forward_nodes_planned = plan.forward_nodes_planned;
    let raw_forward_duration_planned = plan.forward_duration_planned;
    let cross_staff_rest_suppressions = plan.cross_staff_rest_suppressions;
    let mut replacements = plan.replacements;
    let mut canonical_shadow_conflict_suppressions = 0usize;

    if let Some(voice_plan) = frozen_voice_plan.filter(|plan| plan.is_bound()) {
        let mut filtered = Vec::with_capacity(replacements.len());
        for replacement in replacements {
            let materializes_visible_rest =
                replacement.2.contains("<note") && replacement.2.contains("<rest");
            if materializes_visible_rest {
                let singleton = vec![replacement.clone()];
                let conflict = pdf_exact_probe(xml, singleton)
                    .and_then(|probe_xml| {
                        crate::pipeline::
                            newly_introduced_final_visible_shadow_bookkeeping_candidates_with_voice_binding(
                                xml,
                                &probe_xml,
                                voice_plan,
                            )
                    })
                    .is_some_and(|count| count > 0);
                if conflict {
                    canonical_shadow_conflict_suppressions =
                        canonical_shadow_conflict_suppressions.saturating_add(1);
                    continue;
                }
            }
            filtered.push(replacement);
        }
        replacements = filtered;
    }

    let forward_nodes_planned =
        raw_forward_nodes_planned.saturating_sub(canonical_shadow_conflict_suppressions);
    let all_planned_replacements_canonical_shadow_conflicts = raw_forward_nodes_planned > 0
        && canonical_shadow_conflict_suppressions == raw_forward_nodes_planned
        && replacements.is_empty()
        && cross_staff_rest_suppressions == 0
        && parse_before_ok
        && backup_forward_before_ok;
    let forward_duration_accepted = if canonical_shadow_conflict_suppressions == 0 {
        raw_forward_duration_planned
    } else {
        replacements
            .iter()
            .filter(|(_, _, replacement_xml)| {
                replacement_xml.contains("<note") && replacement_xml.contains("<rest")
            })
            .filter_map(|(_, _, replacement_xml)| {
                let start = replacement_xml.find("<duration>")? + "<duration>".len();
                let end = replacement_xml[start..].find("</duration>")? + start;
                replacement_xml[start..end].trim().parse::<i64>().ok()
            })
            .sum()
    };
    let mut audit = SilentVoiceVisibleMeasureCoverageRenderWriteAudit {
        attempted: write_enabled,
        selected_render_source: true,
        write_enabled,
        candidates,
        measures_accepted: 0,
        forward_nodes_planned,
        forward_duration_planned: forward_duration_accepted,
        forward_nodes_accepted: 0,
        forward_duration_accepted: 0,
        visible_rests_before,
        visible_rests_after: visible_rests_before,
        note_count_before: before_observation.note_count,
        note_count_after: before_observation.note_count,
        parse_before_ok,
        parse_after_ok: parse_before_ok,
        backup_forward_before_ok,
        backup_forward_after_ok: backup_forward_before_ok,
        raw_cursor_stable: true,
        pitched_event_multiset_stable: true,
        candidate_rows_after: candidates,
        exact_candidate_delta: false,
        candidate_accepted: false,
        candidate_music_mutation: false,
        master_write_applied: false,
        verdict: "silent_voice_visible_measure_coverage_no_candidate".to_string(),
        ..SilentVoiceVisibleMeasureCoverageRenderWriteAudit::default()
    };

    if (forward_nodes_planned == 0 && cross_staff_rest_suppressions == 0)
        || !parse_before_ok
        || !backup_forward_before_ok
    {
        if all_planned_replacements_canonical_shadow_conflicts {
            audit.exact_candidate_delta = true;
            audit.verdict =
                "silent_voice_visible_measure_coverage_all_materializations_canonical_shadow_conflict_suppressed"
                    .to_string();
        }
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (xml.to_string(), audit, RenderTransitionReceipts::new());
    }
    if !write_enabled {
        audit.verdict = "silent_voice_visible_measure_coverage_writer_staged_no_write".to_string();
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (xml.to_string(), audit, RenderTransitionReceipts::new());
    }

    let authorized_replacements = replacements;
    let (candidate_xml, receipt) =
        crate::structural_write_authority::apply_strict_staff_gap_materialization_authorized(
            xml,
            authorized_replacements,
        );
    receipts.push_exact(receipt);
    let Some(candidate_xml) = candidate_xml else {
        audit.verdict =
            "silent_voice_visible_measure_coverage_central_authority_rejected".to_string();
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return (xml.to_string(), audit, RenderTransitionReceipts::new());
    };

    let after_observation = observe_final_notation_readback(&candidate_xml);
    let backup_forward_after_ok = after_observation.backup_forward_ok;
    let parse_after_ok = after_observation.parse_ok;
    let visible_rests_after = after_observation.visible_rests;
    let candidate_rows_after = silent_coverage_candidate_rows(&candidate_xml);

    audit.note_count_after = after_observation.note_count;
    audit.visible_rests_after = visible_rests_after;
    audit.parse_after_ok = parse_after_ok;
    audit.backup_forward_after_ok = backup_forward_after_ok;
    audit.raw_cursor_stable = before_observation.raw_cursor_signature
        == after_observation.raw_cursor_signature;
    audit.pitched_event_multiset_stable = before_observation.pitched_timeline_signature
        == after_observation.pitched_timeline_signature;
    audit.candidate_rows_after = candidate_rows_after;
    let residual_candidates_are_canonical_shadow_conflicts = if candidate_rows_after == 0 {
        true
    } else if let Some(voice_plan) = frozen_voice_plan.filter(|plan| plan.is_bound()) {
        let residual_plan = plan_silent_voice_visible_measure_coverage(&candidate_xml, false);
        residual_plan.candidates == candidate_rows_after
            && !residual_plan.replacements.is_empty()
            && residual_plan.replacements.iter().all(|replacement| {
                pdf_exact_probe(&candidate_xml, vec![replacement.clone()])
                    .and_then(|probe_xml| {
                        crate::pipeline::
                            newly_introduced_final_visible_shadow_bookkeeping_candidates_with_voice_binding(
                                &candidate_xml,
                                &probe_xml,
                                voice_plan,
                            )
                    })
                    .is_some_and(|count| count > 0)
            })
    } else {
        false
    };
    audit.exact_candidate_delta = audit
        .note_count_after
        .saturating_add(cross_staff_rest_suppressions)
        == audit.note_count_before.saturating_add(audit.forward_nodes_planned)
        && audit
            .visible_rests_after
            .saturating_add(cross_staff_rest_suppressions)
            == audit
                .visible_rests_before
                .saturating_add(audit.forward_nodes_planned)
        && residual_candidates_are_canonical_shadow_conflicts;

    let green = audit.parse_before_ok
        && audit.parse_after_ok
        && audit.backup_forward_before_ok
        && audit.backup_forward_after_ok
        && audit.raw_cursor_stable
        && audit.pitched_event_multiset_stable
        && audit.exact_candidate_delta;
    if green {
        audit.candidate_accepted = true;
        audit.measures_accepted = audit.candidates.saturating_sub(candidate_rows_after);
        audit.forward_nodes_accepted = audit.forward_nodes_planned;
        audit.forward_duration_accepted = audit.forward_duration_planned;
        audit.candidate_music_mutation = true;
        audit.verdict = if cross_staff_rest_suppressions > 0 {
            "silent_voice_visible_measure_coverage_plus_cross_staff_technical_rest_suppression_candidate_accepted".to_string()
        } else {
            "silent_voice_visible_measure_coverage_candidate_accepted".to_string()
        };
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        (candidate_xml, audit, receipts)
    } else {
        audit.verdict = "silent_voice_visible_measure_coverage_rejected_by_readback".to_string();
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        (xml.to_string(), audit, RenderTransitionReceipts::new())
    }
}

fn finalize_silent_voice_visible_measure_coverage_render_write_audit(
    audit: &mut SilentVoiceVisibleMeasureCoverageRenderWriteAudit,
) {
    let committed = audit.commit_outcome == FinalNotationCommitOutcome::Committed;
    let sink_rejected = audit.commit_outcome == FinalNotationCommitOutcome::Rejected;
    let measures_written = if committed { audit.measures_accepted } else { 0 };
    let forward_nodes_replaced = if committed {
        audit.forward_nodes_accepted
    } else {
        0
    };
    let forward_duration_replaced = if committed {
        audit.forward_duration_accepted
    } else {
        0
    };
    let exact_candidate_delta = audit.exact_candidate_delta && !sink_rejected;
    let primary_beam_voice_elements_written = if committed {
        audit.primary_beam_voice_elements_written
    } else {
        0
    };
    let verdict = if sink_rejected {
        "silent_voice_visible_measure_coverage_authorized_sink_rejected".to_string()
    } else if committed {
        audit
            .verdict
            .replace("_candidate_accepted", "_written_render_xml_only")
    } else {
        audit.verdict.clone()
    };
    audit.pipeline_line = format!(
        "SilentVoiceVisibleMeasureCoverageWriterStage v{}: Attempted {}; SelectedRenderSource {}; WriteEnabled {}; Candidates {}; MeasuresWritten {}; ForwardNodesPlanned {}; ForwardDurationPlanned {}; ForwardNodesReplaced {}; ForwardDurationReplaced {}; BeamLostBeginNotesWritten {}; SourceHigherBeamNotesRestored {}; RestVerticalPlacementCandidates {}; RestVerticalPlacementAnchorsWritten {}; RestVerticalPlacementCandidateRowsAfter {}; LedgerBoundedClefBoundariesAccepted {}; LedgerBoundedClefBoundariesWritten {}; PrimaryClefProjectionGreen {}; PrimaryClefProjectionChanged {}; CommittedStateReadbackGreen {}; CommittedPrimaryClefReentryChanges {}; CommittedRestPlacementCandidates {}; CommittedHigherBeamCandidates {}; CommittedBeamLostBeginCandidates {}; CommittedPartialBeatLaneSplitCandidates {}; CommittedLowerDualEdgeCandidates {}; CommittedShadowRestCandidates {}; CommittedShadowRestRows {}; CommittedTiedTailAccentCandidates {}; CommittedRawCursorStable {}; CommittedRawCursorGreen {}; CommittedPitchedEventMultisetStable {}; VisibleRestsBeforeAfter {}/{}; NoteCountBeforeAfter {}/{}; ParseBeforeAfter {}/{}; BackupForwardBeforeAfter {}/{}; RawCursorStable {}; PitchedEventMultisetStable {}; CandidateRowsAfter {}; ExactCandidateDelta {}; SelectorPolicy scorewide_no_notes_full_timeline_single_staff_forward_carrier_plus_structural_cross_staff_covered_rest_lane_suppression_plus_master_bound_orphan_chord_rest_host_plus_ledger_clef_observation_plus_primary_clef_projection_plus_final_clef_bound_rest_anchor_rebind_plus_committed_higher_beam_regression_readback_plus_committed_lost_begin_regression_readback_plus_committed_partial_beat_lane_split_regression_readback_plus_committed_lower_dual_edge_regression_readback_plus_committed_shadow_rest_role_regression_readback_plus_committed_tied_tail_accent_regression_readback; PrimaryBeamVoiceElementsWritten {}; RenderXmlWrite {}; NotationMutation {}; MusicMutation {}; MasterApply {}; ReleaseVeto {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        yes_no(audit.attempted),
        yes_no(audit.selected_render_source),
        yes_no(audit.write_enabled),
        audit.candidates,
        measures_written,
        audit.forward_nodes_planned,
        audit.forward_duration_planned,
        forward_nodes_replaced,
        forward_duration_replaced,
        0_usize,
        audit.source_higher_beam_notes_restored,
        audit.rest_vertical_candidates,
        audit.rest_vertical_anchors_written,
        audit.rest_vertical_candidate_rows_after,
        audit.ledger_bounded_clef_boundaries_accepted,
        audit.ledger_bounded_clef_boundaries_written,
        yes_no(audit.primary_clef_projection_green),
        yes_no(audit.primary_clef_projection_changed),
        yes_no(audit.committed_state_readback_green),
        audit.committed_primary_clef_reentry_changes,
        audit.committed_rest_vertical_candidates,
        audit.committed_higher_beam_candidates,
        audit.committed_beam_lost_begin_candidates,
        audit.committed_partial_beat_lane_split_candidates,
        audit.committed_lower_dual_edge_candidates,
        audit.committed_shadow_rest_candidates,
        if audit.committed_shadow_rest_rows.is_empty() {
            "none"
        } else {
            audit.committed_shadow_rest_rows.as_str()
        },
        audit.committed_tied_tail_accent_candidates,
        yes_no(audit.committed_raw_cursor_stable),
        yes_no(audit.committed_raw_cursor_green),
        yes_no(audit.committed_pitched_event_multiset_stable),
        audit.visible_rests_before,
        audit.visible_rests_after,
        audit.note_count_before,
        audit.note_count_after,
        yes_no(audit.parse_before_ok),
        yes_no(audit.parse_after_ok),
        yes_no(audit.backup_forward_before_ok),
        yes_no(audit.backup_forward_after_ok),
        yes_no(audit.raw_cursor_stable),
        yes_no(audit.pitched_event_multiset_stable),
        audit.candidate_rows_after,
        yes_no(exact_candidate_delta),
        primary_beam_voice_elements_written,
        yes_no(committed),
        yes_no(committed),
        yes_no(committed && audit.candidate_music_mutation),
        yes_no(audit.master_write_applied),
        yes_no(audit.selected_render_source && !audit.committed_state_readback_green),
        verdict,
    );
}

fn load_xml_text(path: &std::path::Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

pub(super) fn materialize_silent_voice_visible_measure_coverage_for_pdf_source(
    input_musicxml: &std::path::Path,
    write_enabled: bool,
    final_chord_structural_plan: &crate::structural_write_authority::CanonicalFinalChordStructuralPlan,
    frozen_serialized_voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
    render_pruned_final_chord_omissions: &std::collections::BTreeSet<
        crate::structural_write_authority::RenderPrunedFinalChordOmissionKey,
    >,
    occurrence_continuation: &crate::structural_write_authority::FinalPdfSourceOccurrenceContinuation,
    primary_beam_voice_partition_plan:
        Option<&crate::structural_write_authority::PrimaryBeamVoicePartitionBoundPlan>,
) -> FinalRenderSourceClosure {
    let mut receipts = RenderTransitionReceipts::new();
    let selected_render_source = input_musicxml
        .file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.ends_with(".render.musicxml"));
    if !selected_render_source {
        return rejected_pdf_preparation(receipts);
    }
    let source_xml = occurrence_continuation.xml().to_string();
    // Master remains audit context; Render preparation is bound to sovereign continuation bytes.
    let master_xml = final_beam_master_path(input_musicxml).and_then(|path| load_xml_text(&path));
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "FinalPdfMaterializerEntrySource",
        &source_xml,
    );

    // Preserve the original relation for structural FinalNotation writers.
    // The single final clef-course projection and rest-glyph owner consume
    // the final beam voice relation below, before the unchanged sink.
    let mut effective_voice_plan = frozen_serialized_voice_plan.clone();
    let effective_occurrence_continuation = occurrence_continuation.clone();
    let mut primary_beam_voice_partition_changed_voice_elements = 0usize;

    let (silent_candidate, mut silent_audit, silent_receipts) =
        apply_silent_voice_visible_measure_coverage_render_write(
            &source_xml,
            write_enabled,
            Some(&effective_voice_plan),
        );
    receipts.append(silent_receipts);
    let (chord_host_candidate, chord_host_audit, chord_receipts) =
        stage_final_orphan_chord_host_rest_candidate(
            &silent_candidate,
            write_enabled,
        );
    receipts.append(chord_receipts);
    // v0.10.1715: Der historische Ledger-Readback darf vor RestPlacement weiterhin
    // Clef-Surfaces als Kandidaten erzeugen. Der committed finale F4/G2-Kurs wird
    // jedoch erst in der bestehenden FinalNotation-Write-Authority durch den
    // ClefReadability-Primary-Plan gebunden. RestPlacement sieht weiterhin einen
    // gueltigen Clef-Kontext, besitzt aber keine Clef-Entscheidungsautoritaet.
    let candidate_before_clef_readback = chord_host_candidate;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "FinalPdfBeforeClefReadback",
        &candidate_before_clef_readback,
    );
    let (candidate_with_clef_readback, clef_receipt) =
        super::super::notation::ledger_bounded_lower_staff_clef_readback(
            &candidate_before_clef_readback,
            super::super::notation::LowerStaffClefReadbackScope::FinalPdfSource,
        );
    let clef_marker = "etuschlopf-ledger-bound-clef-readback";
    let clef_markers_before = candidate_before_clef_readback.matches(clef_marker).count();
    let clef_markers_after = candidate_with_clef_readback.matches(clef_marker).count();
    let mut ledger_bounded_clef_boundaries_accepted =
        clef_markers_after.saturating_sub(clef_markers_before);
    let clef_before_observation =
        observe_final_notation_readback(&candidate_before_clef_readback);
    let clef_after_observation = observe_final_notation_readback(&candidate_with_clef_readback);
    let clef_readback_green = clef_after_observation.parse_ok
        && clef_before_observation.pitched_timeline_signature
            == clef_after_observation.pitched_timeline_signature
        && clef_before_observation.raw_cursor_signature
            == clef_after_observation.raw_cursor_signature
        && clef_before_observation.note_count == clef_after_observation.note_count;
    let mut clef_trial = RenderTransitionReceipts::new();
    clef_trial.push_preserving(clef_receipt, &candidate_with_clef_readback);
    let clef_bound_candidate = if clef_readback_green {
        receipts.append(clef_trial);
        candidate_with_clef_readback
    } else {
        clef_trial.discard_unadopted(RenderDiscardReason::LosingCandidate);
        ledger_bounded_clef_boundaries_accepted = 0;
        candidate_before_clef_readback.clone()
    };

    // Rest glyph placement consumes the final serialized voice relation below.
    // It remains one invocation of the existing owner, before the same sink.
    let final_candidate_before_same_lane_split = clef_bound_candidate;

    // r1027: r1024-r1026 proved that the unique strict-interior same-lane
    // Rest/Pitch obstruction must be closed inside the already-existing
    // FinalPdfSourceCommit materializer itself. Earlier projections can be
    // superseded before this terminal candidate is committed. Keep the
    // decision identity-free, score-wide and fail-closed: exactly one row,
    // no project/measure/pitch selector, no new writer or authority.
    let plan_final_pdf_source_same_lane_strict_interior =
        |xml: &str| -> (usize, Option<(usize, usize, String)>) {
            let mut candidate_count = 0_usize;
            let mut selected_edit: Option<(usize, usize, String)> = None;
            let mut scan = 0_usize;
            let mut divisions = 1_i64;
            let mut beats = 4_i64;
            let mut beat_type = 4_i64;

            while let Some((measure_start, measure_end, _)) =
                super::super::find_next_measure_bounds(xml, scan)
            {
                let measure_xml = &xml[measure_start..measure_end];
                let (next_divisions, next_beats, next_beat_type, expected_duration) =
                    super::super::parse_musicxml_current_meter(
                        measure_xml,
                        divisions,
                        beats,
                        beat_type,
                    );
                divisions = next_divisions;
                beats = next_beats;
                beat_type = next_beat_type;
                let beat_unit = divisions.max(1);
                let raw_notes = super::super::partial_beat_raw_notes(measure_xml);
                let note_infos =
                    super::super::partial_beat_note_infos(measure_xml, &raw_notes);

                for rest in note_infos.iter().filter(|note| {
                    note.is_rest
                        && !note.is_hidden
                        && !note.is_chord
                        && note.duration > 0
                        && note.staff.trim() == "2"
                        && note.voice.trim() != "1"
                }) {
                    let rest_end = rest.start.saturating_add(rest.duration);
                    let same_lane_pitched = note_infos
                        .iter()
                        .filter(|note| {
                            !note.is_rest
                                && !note.is_hidden
                                && !note.is_chord
                                && note.duration > 0
                                && note.staff == rest.staff
                                && note.voice == rest.voice
                                && rest.start < note.start.saturating_add(note.duration)
                                && note.start < rest_end
                        })
                        .collect::<Vec<_>>();
                    if same_lane_pitched.len() != 1 {
                        continue;
                    }
                    let pitched = same_lane_pitched[0];
                    let pitched_end = pitched.start.saturating_add(pitched.duration);
                    if !(rest.start < pitched.start && pitched_end < rest_end) {
                        continue;
                    }
                    let other_voice_pitch_overlap = note_infos.iter().any(|note| {
                        !note.is_rest
                            && !note.is_hidden
                            && !note.is_chord
                            && note.duration > 0
                            && note.staff == rest.staff
                            && note.voice != rest.voice
                            && rest.start < note.start.saturating_add(note.duration)
                            && note.start < rest_end
                    });
                    let other_visible_rest_overlap = note_infos.iter().any(|note| {
                        note.is_rest
                            && !note.is_hidden
                            && !note.is_chord
                            && note.duration > 0
                            && note.index != rest.index
                            && note.staff == rest.staff
                            && rest.start < note.start.saturating_add(note.duration)
                            && note.start < rest_end
                    });
                    if other_voice_pitch_overlap || other_visible_rest_overlap {
                        continue;
                    }

                    let leading_duration = pitched.start.saturating_sub(rest.start);
                    let trailing_duration = rest_end.saturating_sub(pitched_end);
                    let Some(leading_parts) =
                        super::super::plan_exact_metric_visible_rest_durations(
                            rest.start,
                            leading_duration,
                            expected_duration,
                            beat_unit,
                        )
                    else {
                        continue;
                    };
                    let Some(trailing_parts) =
                        super::super::plan_exact_metric_visible_rest_durations(
                            pitched_end,
                            trailing_duration,
                            expected_duration,
                            beat_unit,
                        )
                    else {
                        continue;
                    };
                    if leading_parts.len() != 1
                        || leading_parts[0] != leading_duration
                        || trailing_parts.len() != 1
                        || trailing_parts[0] != trailing_duration
                    {
                        continue;
                    }
                    let Some(raw_rest) = raw_notes.iter().find(|raw| raw.start == rest.index)
                    else {
                        continue;
                    };
                    let Ok(leading_duration_i32) = i32::try_from(leading_duration) else {
                        continue;
                    };
                    let Ok(trailing_duration_i32) = i32::try_from(trailing_duration) else {
                        continue;
                    };
                    let Some(mut leading_rest) =
                        super::super::render_visible_rest_note_fragment(
                            leading_duration_i32,
                            &rest.staff,
                            &rest.voice,
                        )
                    else {
                        continue;
                    };
                    let Some(mut trailing_rest) =
                        super::super::render_visible_rest_note_fragment(
                            trailing_duration_i32,
                            &rest.staff,
                            &rest.voice,
                        )
                    else {
                        continue;
                    };
                    let display_step =
                        super::super::partial_beat_first_text(&raw_rest.text, "display-step");
                    let display_octave =
                        super::super::partial_beat_first_text(&raw_rest.text, "display-octave");
                    if let (Some(step), Some(octave)) = (display_step, display_octave) {
                        let display_rest = format!(
                            "<rest>\n          <display-step>{step}</display-step>\n          <display-octave>{octave}</display-octave>\n        </rest>"
                        );
                        leading_rest = leading_rest.replacen("<rest/>", &display_rest, 1);
                        trailing_rest = trailing_rest.replacen("<rest/>", &display_rest, 1);
                    }
                    let Ok(pitched_duration) = i32::try_from(pitched.duration) else {
                        continue;
                    };
                    candidate_count = candidate_count.saturating_add(1);
                    if candidate_count == 1 {
                        selected_edit = Some((
                            measure_start.saturating_add(raw_rest.start),
                            measure_start.saturating_add(raw_rest.end),
                            format!(
                                "{}{}{}",
                                leading_rest,
                                super::super::render_staff_voice_forward_fragment(
                                    pitched_duration,
                                    &rest.staff,
                                    &rest.voice,
                                ),
                                trailing_rest,
                            ),
                        ));
                    }
                }
                scan = measure_end;
            }

            if candidate_count == 1 {
                (candidate_count, selected_edit)
            } else {
                (candidate_count, None)
            }
        };

    let (same_lane_strict_interior_candidates, same_lane_strict_interior_edit) =
        plan_final_pdf_source_same_lane_strict_interior(
            &final_candidate_before_same_lane_split,
        );
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "FinalPdfBeforeSameLaneSplit",
        &final_candidate_before_same_lane_split,
    );
    let mut final_candidate = final_candidate_before_same_lane_split;
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "FinalPdfSameLaneInput",
        &final_candidate,
    );
    let mut same_lane_strict_interior_split_accepted = false;

    if let Some((edit_start, edit_end, replacement)) = same_lane_strict_interior_edit {
        let (candidate, same_lane_splice_receipt) = same_lane_strict_interior_raw_splice(
            &final_candidate, edit_start, edit_end, &replacement
        );
        let before_observation = observe_final_notation_readback(&final_candidate);
        let after_observation = observe_final_notation_readback(&candidate);
        let (residual_candidates, _) =
            plan_final_pdf_source_same_lane_strict_interior(&candidate);
        let split_green = same_lane_strict_interior_candidates == 1
            && residual_candidates == 0
            && before_observation.parse_ok
            && after_observation.parse_ok
            && before_observation.backup_forward_ok
            && after_observation.backup_forward_ok
            && before_observation.raw_cursor_signature == after_observation.raw_cursor_signature
            && before_observation.pitched_timeline_signature
                == after_observation.pitched_timeline_signature
            && before_observation.pitched_payload_signature
                == after_observation.pitched_payload_signature
            && after_observation.visible_rests
                == before_observation.visible_rests.saturating_add(1)
            && after_observation.note_count
                == before_observation.note_count.saturating_add(1);
        let mut splice_trial = RenderTransitionReceipts::new();
        splice_trial.push_exact(same_lane_splice_receipt);
        if split_green {
            receipts.append(splice_trial);
            crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
                &final_candidate,
                &candidate,
            );
            final_candidate = candidate;
            same_lane_strict_interior_split_accepted = true;
        } else { splice_trial.discard_unadopted(RenderDiscardReason::LosingCandidate); }
    }
    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "FinalPdfAfterSameLaneSplit",
        &final_candidate,
    );
    // v0.10.1773/r912: The orphan-chord host normalizer is an intentional
    // structural canonicalization. A raw Render source may contain <forward>
    // followed by receipt-bound orphan <chord/> members; replacing that carrier
    // with a hidden duration-identical rest host gives those chord members their
    // canonical anchor onset without changing pitch/duration/staff/voice payload.
    // FinalChord continuation, not Master representation, owns pitched identity.
    // r1067 keeps this entire accepted FinalNotation bundle byte-semantically
    // ahead of the primary-beam channel rebind. The rebind is applied only after
    // this bundle has closed, so it cannot steer any earlier writer.
    let base_candidate_occurrence_continuation =
        crate::structural_write_authority::
            transport_final_pdf_source_occurrence_continuation_through_final_notation_bundle(
                &effective_occurrence_continuation,
                &source_xml,
                &silent_candidate,
                &candidate_before_clef_readback,
                crate::structural_write_authority::OrphanChordHostContinuationEvidence {
                    normalization_accepted: chord_host_audit.candidate_accepted,
                    accepted_chord_notes: chord_host_audit.chord_notes_accepted,
                },
                &final_candidate,
                crate::structural_write_authority::FinalPdfSourceOccurrenceValidationContext {
                    canonical: final_chord_structural_plan,
                    voice_plan: frozen_serialized_voice_plan,
                    render_omissions: Some(render_pruned_final_chord_omissions),
                },
            );
    let mut candidate_occurrence_continuation = base_candidate_occurrence_continuation;

    if let Some(plan) = primary_beam_voice_partition_plan {
        let base_continuation = match candidate_occurrence_continuation.as_ref() {
            Ok(continuation) => continuation,
            Err(_) => {
                return rejected_pdf_preparation(receipts);
            }
        };
        let rebound_plan = match crate::structural_write_authority::
            rebind_primary_beam_voice_partition_plan_to_current_xml(
                &final_candidate,
                base_continuation,
                plan,
            )
        {
            Ok(plan) => plan,
            Err(_) => {
                return rejected_pdf_preparation(receipts);
            }
        };
        let mut atomic_voice_edits = Vec::<(usize, usize, String)>::new();
        for edit in rebound_plan.edits() {
            atomic_voice_edits.push((
                edit.voice_text_start(),
                edit.voice_text_end(),
                edit.target_label().to_string(),
            ));
        }
        // Render-side call: the Master lineage receipt (#40) is not attached here.
        let (rebound_xml, render_scalar_batch_receipt) =
            crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
                &final_candidate,
                atomic_voice_edits,
            );
        receipts.push_exact(render_scalar_batch_receipt);
        let Some(rebound_xml) = rebound_xml else {
            return rejected_pdf_preparation(receipts);
        };
        let transported = crate::structural_write_authority::
            transport_final_pdf_source_occurrence_continuation_through_primary_beam_voice_partition(
                base_continuation,
                &final_candidate,
                &rebound_xml,
                final_chord_structural_plan,
                &rebound_plan,
                Some(render_pruned_final_chord_omissions),
            );
        let Ok(transported) = transported else {
            return rejected_pdf_preparation(receipts);
        };
        final_candidate = rebound_xml;
        effective_voice_plan = rebound_plan.repaired_voice_plan().clone();
        primary_beam_voice_partition_changed_voice_elements =
            rebound_plan.changed_voice_elements();
        candidate_occurrence_continuation = Ok(transported);
    }

    // r1463: both final clef course and rest anchors depend on the final
    // beam voice relation. The unchanged sink must confirm this fixpoint.
    let before_final_surfaces = final_candidate;
    // v0.10.1772: RestPlacement depends on the committed PrimaryClef course.
    // Project that already-authoritative course writer-free before rest placement,
    // so the later FinalNotation sink can only confirm the same clef fixpoint.
    let mut primary_clef_bound_candidate = before_final_surfaces.clone();
    let (primary_projection, primary_receipt) = crate::structural_write_authority::apply_rustig_clef_course_to_buffer(&mut primary_clef_bound_candidate);
    let primary_clef_projection_green = primary_projection.is_ok();
    let mut primary_trial = RenderTransitionReceipts::new();
    primary_trial.push_preserving(primary_receipt, &primary_clef_bound_candidate);
    if primary_clef_projection_green { receipts.append(primary_trial); }
    else { primary_trial.discard_unadopted(RenderDiscardReason::LosingCandidate); }
    let primary_clef_projection_changed =
        primary_clef_projection_green && primary_clef_bound_candidate != before_final_surfaces;
    if !primary_clef_projection_green {
        primary_clef_bound_candidate = before_final_surfaces.clone();
    }
    silent_audit.primary_clef_projection_green = primary_clef_projection_green;
    silent_audit.primary_clef_projection_changed = primary_clef_projection_changed;

    let (rest_bound_candidate, mut rest_audit, rest_receipts) =
        stage_final_rest_vertical_placement_candidate(&primary_clef_bound_candidate, write_enabled);
    receipts.append(rest_receipts);
    rest_audit.final_clef_bound = primary_clef_projection_green;
    let before_rest_continuation = match candidate_occurrence_continuation.as_ref() {
        Ok(continuation) => continuation,
        Err(_) => return rejected_pdf_preparation(receipts),
    };
    candidate_occurrence_continuation = before_rest_continuation.through_final_clef_and_rest_placement(
            &before_final_surfaces,
            &rest_bound_candidate,
            final_chord_structural_plan,
            &effective_voice_plan,
            Some(render_pruned_final_chord_omissions),
        );
    final_candidate = rest_bound_candidate;
    if rest_audit.candidate_accepted {
        crate::audit_only::destructive_edit_attribution::observe_selected_final_artifact_state(
            input_musicxml, &final_candidate,
        );
    }

    crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
        "FinalPdfPrecommit",
        &final_candidate,
    );
    let canonical_precommit_observation =
        observe_final_notation_readback(&final_candidate);
    let write_requested = silent_audit.candidate_accepted
        || chord_host_audit.candidate_accepted
        || rest_audit.candidate_accepted
        || same_lane_strict_interior_split_accepted
        || ledger_bounded_clef_boundaries_accepted > 0
        || primary_clef_projection_changed
        || primary_beam_voice_partition_changed_voice_elements > 0;
    if candidate_occurrence_continuation.is_err() { return rejected_pdf_preparation(receipts); }
    let prepared_candidate = final_candidate.clone();
    let final_chord_structural_plan = final_chord_structural_plan.clone();
    let render_pruned_final_chord_omissions = render_pruned_final_chord_omissions.clone();
    let finish = Box::new(move |witnessed_post: &str| {
    let commit_outcome = if write_requested { FinalNotationCommitOutcome::Committed } else { FinalNotationCommitOutcome::NotRequested };
    let committed = commit_outcome == FinalNotationCommitOutcome::Committed;
    silent_audit.ledger_bounded_clef_boundaries_accepted =
        ledger_bounded_clef_boundaries_accepted;

    // v0.10.1772: final evidence is bound to the persisted artifact, not to an
    // in-memory pre-sink candidate. Re-open the selected Render-MusicXML and prove
    // the PrimaryClef fixpoint, final RestPlacement closure, retired-HigherBeam,
    // retired-LostBegin, retired-PartialBeat, retired-LowerDualEdge, retired-ShadowRest
    // and retired-TiedTailAccentSuppress regression closure plus timeline noninterference
    // on exactly the committed bytes.
    let mut committed_state_readback_green = false;
    let mut committed_primary_clef_reentry_changes = usize::MAX;
    let mut committed_rest_vertical_candidates = usize::MAX;
    let mut committed_higher_beam_candidates = usize::MAX;
    let mut committed_beam_lost_begin_candidates = usize::MAX;
    let mut committed_partial_beat_lane_split_candidates = usize::MAX;
    let mut committed_lower_dual_edge_candidates = usize::MAX;
    let mut committed_shadow_rest_candidates = usize::MAX;
    let mut committed_shadow_rest_rows = String::new();
    let mut committed_tied_tail_accent_candidates = usize::MAX;
    let mut committed_same_lane_strict_interior_candidates = usize::MAX;
    let mut committed_raw_cursor_stable = false;
    let mut committed_raw_cursor_green = false;
    let mut committed_pitched_event_multiset_stable = false;
    let mut committed_ledger_clef_markers = 0usize;
    let mut committed_occurrence_continuation = None;
    if commit_outcome != FinalNotationCommitOutcome::Rejected { let committed_xml = witnessed_post.to_string();
            crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
                "FinalPdfCommittedReadback",
                &committed_xml,
            );
            committed_ledger_clef_markers = committed_xml.matches(clef_marker).count();
            let committed_occurrence_transport = match candidate_occurrence_continuation.as_ref() {
                Ok(candidate_continuation) => crate::structural_write_authority::
                    bind_final_pdf_source_occurrence_continuation_to_exact_persisted_readback(
                        candidate_continuation,
                        &committed_xml,
                        &final_chord_structural_plan,
                        &effective_voice_plan,
                        Some(&render_pruned_final_chord_omissions),
                    ),
                Err(error) => Err(std::io::Error::new(error.kind(), error.to_string())),
            };
            let committed_final_chord_voice_readback_green =
                committed_occurrence_transport.as_ref().is_ok();
            if committed_final_chord_voice_readback_green {
                committed_occurrence_continuation = committed_occurrence_transport.ok();
            }
            let source_observation = observe_final_notation_readback(&source_xml);
            let committed_observation = observe_final_notation_readback(&committed_xml);
            committed_same_lane_strict_interior_candidates =
                plan_final_pdf_source_same_lane_strict_interior(&committed_xml).0;
            committed_raw_cursor_stable = source_observation.parse_ok
                && committed_observation.parse_ok
                && source_observation.raw_cursor_signature
                    == committed_observation.raw_cursor_signature;
            committed_raw_cursor_green = committed_observation.parse_ok
                && committed_observation.raw_cursor_signature.iter().all(|row| {
                    let (
                        _,
                        expected,
                        cursor_min,
                        cursor_max,
                        cursor_final,
                        partial_backups,
                        overflow_events,
                        post_expected_events,
                    ) = *row;
                    cursor_min >= 0
                        && cursor_max <= expected
                        && cursor_final == expected
                        && partial_backups == 0
                        && overflow_events == 0
                        && post_expected_events == 0
                });
            committed_pitched_event_multiset_stable = canonical_precommit_observation.parse_ok
                && committed_observation.parse_ok
                && canonical_precommit_observation.pitched_timeline_signature
                    == committed_observation.pitched_timeline_signature;

            let mut clef_fixpoint_probe = committed_xml.clone();
            let (probe_result, probe_receipt) = crate::structural_write_authority::apply_rustig_clef_course_to_buffer(
                    &mut clef_fixpoint_probe,
                );
            let mut probe = RenderTransitionReceipts::new();
            probe.push_preserving(probe_receipt, &clef_fixpoint_probe);
            probe.discard_unadopted(RenderDiscardReason::ReadOnlyProbe);
            committed_primary_clef_reentry_changes = match probe_result {
                    Ok(()) => usize::from(clef_fixpoint_probe != committed_xml),
                    Err(_) => usize::MAX,
                };
            committed_rest_vertical_candidates = final_rest_structural_targets(&committed_xml)
                .map_or(usize::MAX, |targets| targets.len());
            committed_higher_beam_candidates = match master_xml.as_deref() {
                Some(master_xml) => final_pdf_source_higher_beam_regression_candidates(
                    master_xml,
                    &committed_xml,
                )
                .unwrap_or(usize::MAX),
                None => 0,
            };
            committed_beam_lost_begin_candidates = match master_xml.as_deref() {
                Some(master_xml) => final_pdf_source_lost_begin_regression_candidates(
                    master_xml,
                    &committed_xml,
                )
                .unwrap_or(usize::MAX),
                None => 0,
            };
            committed_partial_beat_lane_split_candidates =
                crate::pipeline::partial_beat_post_rest_lane_split_regression_candidates(
                    &committed_xml,
                )
                .unwrap_or(usize::MAX);
            committed_lower_dual_edge_candidates =
                crate::pipeline::lower_dual_edge_beat_regression_candidates_with_voice_binding(
                    &committed_xml,
                    &effective_voice_plan,
                )
                .unwrap_or(usize::MAX);
            committed_shadow_rest_candidates =
                crate::pipeline::final_visible_shadow_bookkeeping_regression_candidates_with_voice_binding(
                    &committed_xml,
                    &effective_voice_plan,
                )
                .unwrap_or(usize::MAX);
            committed_shadow_rest_rows =
                crate::pipeline::final_visible_shadow_bookkeeping_regression_candidate_rows(
                    &committed_xml,
                    &effective_voice_plan,
                )
                .map(|rows| {
                    if rows.is_empty() {
                        String::new()
                    } else {
                        rows.join(" || ")
                    }
                })
                .unwrap_or_else(|| "unavailable".to_string());
            committed_tied_tail_accent_candidates =
                crate::pipeline::unique_confusing_tied_tail_accent_regression_candidates(
                    &committed_xml,
                )
                .unwrap_or(usize::MAX);
            committed_state_readback_green = primary_clef_projection_green
                && committed_final_chord_voice_readback_green
                && committed_primary_clef_reentry_changes == 0
                && committed_rest_vertical_candidates == 0
                && committed_higher_beam_candidates == 0
                && committed_beam_lost_begin_candidates == 0
                && committed_partial_beat_lane_split_candidates == 0
                && committed_lower_dual_edge_candidates == 0
                && committed_shadow_rest_candidates == 0
                && committed_tied_tail_accent_candidates == 0
                && committed_same_lane_strict_interior_candidates == 0
                && committed_raw_cursor_stable
                && committed_raw_cursor_green
                && committed_pitched_event_multiset_stable;
        }
    silent_audit.committed_state_readback_green = committed_state_readback_green;
    silent_audit.committed_primary_clef_reentry_changes =
        committed_primary_clef_reentry_changes;
    silent_audit.committed_rest_vertical_candidates = committed_rest_vertical_candidates;
    silent_audit.committed_higher_beam_candidates = committed_higher_beam_candidates;
    silent_audit.committed_beam_lost_begin_candidates = committed_beam_lost_begin_candidates;
    silent_audit.committed_partial_beat_lane_split_candidates =
        committed_partial_beat_lane_split_candidates;
    silent_audit.committed_lower_dual_edge_candidates =
        committed_lower_dual_edge_candidates;
    silent_audit.committed_shadow_rest_candidates = committed_shadow_rest_candidates;
    silent_audit.committed_shadow_rest_rows = committed_shadow_rest_rows;
    silent_audit.committed_tied_tail_accent_candidates =
        committed_tied_tail_accent_candidates;
    silent_audit.committed_raw_cursor_stable = committed_raw_cursor_stable;
    silent_audit.committed_raw_cursor_green = committed_raw_cursor_green;
    silent_audit.committed_pitched_event_multiset_stable =
        committed_pitched_event_multiset_stable;
    silent_audit.ledger_bounded_clef_boundaries_written = committed_ledger_clef_markers;
    silent_audit.primary_beam_voice_elements_written =
        primary_beam_voice_partition_changed_voice_elements;
    if committed_rest_vertical_candidates != usize::MAX {
        rest_audit.candidate_rows_after = committed_rest_vertical_candidates;
    }
    rest_audit.final_clef_bound =
        primary_clef_projection_green && committed_primary_clef_reentry_changes == 0;

    silent_audit.commit_outcome = commit_outcome;
    if rest_audit.candidate_accepted {
        rest_audit.commit_outcome = commit_outcome;
    }
    // v0.10.1776: productive HigherBeam recovery is retired. The historical
    // master-bound damage shape remains a committed-state regression detector.
    silent_audit.source_higher_beam_notes_restored = 0;
    silent_audit.candidates = silent_audit
        .candidates
        .saturating_add(chord_host_audit.candidates);
    silent_audit.forward_nodes_planned = silent_audit
        .forward_nodes_planned
        .saturating_add(chord_host_audit.forward_nodes_planned);
    silent_audit.forward_duration_planned = silent_audit
        .forward_duration_planned
        .saturating_add(chord_host_audit.forward_duration_planned);
    let chord_host_candidate_rows_after = if commit_outcome == FinalNotationCommitOutcome::Rejected
        && chord_host_audit.candidate_accepted
    {
        chord_host_audit.candidates
    } else {
        chord_host_audit.candidate_rows_after
    };
    silent_audit.candidate_rows_after = silent_audit
        .candidate_rows_after
        .saturating_add(chord_host_candidate_rows_after);
    if committed && chord_host_audit.candidate_accepted {
        silent_audit.measures_accepted = silent_audit
            .measures_accepted
            .saturating_add(
                chord_host_audit
                    .forward_nodes_accepted
                    .saturating_add(chord_host_audit.anchor_promotions_accepted),
            );
        silent_audit.forward_nodes_accepted = silent_audit
            .forward_nodes_accepted
            .saturating_add(chord_host_audit.forward_nodes_accepted);
        silent_audit.forward_duration_accepted = silent_audit
            .forward_duration_accepted
            .saturating_add(chord_host_audit.forward_duration_accepted);
    }
    silent_audit.rest_vertical_candidates = rest_audit.candidates;
    silent_audit.rest_vertical_anchors_written = if committed {
        rest_audit.anchors_accepted
    } else {
        0
    };
    silent_audit.rest_vertical_candidate_rows_after = if commit_outcome == FinalNotationCommitOutcome::Rejected
        && rest_audit.candidate_accepted
    {
        rest_audit.candidates
    } else {
        rest_audit.candidate_rows_after
    };
    if ledger_bounded_clef_boundaries_accepted > 0 {
        silent_audit.candidates = silent_audit
            .candidates
            .saturating_add(ledger_bounded_clef_boundaries_accepted);
        silent_audit.exact_candidate_delta = true;
        silent_audit.candidate_accepted = true;
        silent_audit.verdict = format!(
            "{}_plus_final_pdf_source_ledger_bounded_clef_readback",
            silent_audit.verdict
        );
    }
    if chord_host_audit.anchor_promotions_accepted > 0 {
        silent_audit.verdict = format!(
            "{}_plus_orphan_chord_rest_host_anchor_promotion",
            silent_audit.verdict
        );
    }
    if chord_host_audit.candidate_accepted
        || rest_audit.candidate_accepted
        || ledger_bounded_clef_boundaries_accepted > 0
        || primary_clef_projection_changed
        || primary_beam_voice_partition_changed_voice_elements > 0
    {
        silent_audit.candidate_accepted = true;
        silent_audit.candidate_music_mutation = silent_audit.candidate_music_mutation
            || chord_host_audit.candidate_music_mutation
            || rest_audit.candidate_music_mutation
            || primary_clef_projection_changed
            || primary_beam_voice_partition_changed_voice_elements > 0;
    }
    let beam_regression_line = final_beam_lost_begin_committed_regression_line(
        master_xml.is_some(),
        write_enabled,
        (committed_beam_lost_begin_candidates != usize::MAX)
            .then_some(committed_beam_lost_begin_candidates),
    );
    finalize_final_rest_vertical_placement_render_write_audit(&mut rest_audit);
    finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut silent_audit);
    silent_audit.pipeline_line.push_str(&format!(
        "\nFinalPdfSourceSameLaneStrictInteriorClosure: Candidates {}; SplitAccepted {}; CommittedResidual {}; FinalPdfSourceCommitBound ja; Selector identity_free_scorewide_exactly_one; ReleaseVeto {}.",
        same_lane_strict_interior_candidates,
        yes_no(same_lane_strict_interior_split_accepted),
        committed_same_lane_strict_interior_candidates,
        yes_no(committed_same_lane_strict_interior_candidates != 0),
    ));
    silent_audit.pipeline_line = format!(
        "{}\n{}\n{}",
        silent_audit.pipeline_line,
        beam_regression_line,
        rest_audit.pipeline_line
    );
    let closed = committed_state_readback_green;
    let committed_xml = closed.then(|| witnessed_post.to_string());
    FinalRenderSourceSnapshot {
        pipeline_line: silent_audit.pipeline_line,
        closed,
        committed_xml,
        committed_occurrence_continuation,
        committed_voice_plan: closed.then_some(effective_voice_plan),
        primary_beam_voice_partition_changed_voice_elements: if closed {
            primary_beam_voice_partition_changed_voice_elements
        } else {
            0
        },
    }
    });
    FinalRenderSourceClosure { candidate_xml: Some(prepared_candidate), receipts, finish: Some(finish) }
}

#[cfg(test)]
#[path = "../../tests/unit/final_rest_after_beam_voice.rs"]
mod final_rest_after_beam_voice_tests;

#[cfg(test)]
mod silent_voice_visible_measure_coverage_render_write_tests {
    use super::super::final_pdf_source_commit::primary_clef_probe;
    use super::{
        apply_silent_voice_visible_measure_coverage_render_write,
        apply_silent_voice_visible_measure_coverage_render_write as apply_silent_voice_visible_measure_coverage_render_write_with_receipts,
        collect_final_orphan_chord_host_replacements,
        final_pdf_source_higher_beam_regression_candidates,
        final_pdf_source_lost_begin_regression_candidates,
        final_rest_structural_targets,
        finalize_final_rest_vertical_placement_render_write_audit,
        finalize_silent_voice_visible_measure_coverage_render_write_audit,
        observe_final_notation_readback, FinalNotationCommitOutcome,
        FinalRestPlacementDecision, SilentVoiceVisibleMeasureCoverageRenderWriteAudit,
        stage_final_orphan_chord_host_rest_candidate as stage_final_orphan_chord_host_rest_candidate_with_receipts,
        stage_final_rest_vertical_placement_candidate as stage_final_rest_vertical_placement_candidate_with_receipts,
    };

    fn score(carrier: &str) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{carrier}<backup><duration>384</duration></backup><note><pitch><step>C</step><octave>4</octave></pitch><duration>384</duration><voice>2</voice><type>whole</type><staff>2</staff></note></measure></part></score-partwise>"
        )
    }

    fn chord_host_score(host: &str) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><forward><duration>288</duration></forward>{host}<note print-object=\"no\"><chord/><pitch><step>F</step><octave>6</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><note print-object=\"no\"><chord/><pitch><step>C</step><octave>6</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>48</duration><voice>1</voice><staff>1</staff></forward></measure></part></score-partwise>"
        )
    }

    #[test]
    fn restores_master_bound_rest_host_for_overhanging_chord_cluster() {
        let render = chord_host_score("<forward><duration>48</duration></forward>");
        let (after, audit) =
            stage_final_orphan_chord_host_rest_candidate(&render, true);
        assert!(audit.candidate_accepted);
        assert_eq!(audit.candidates, 1);
        assert_eq!(audit.forward_nodes_accepted, 1);
        assert_eq!(audit.forward_duration_accepted, 48);
        assert_eq!(audit.chord_notes_accepted, 2);
        assert_eq!(audit.candidate_rows_after, 0);
        assert!(audit.raw_cursor_stable);
        assert!(audit.pitched_payload_stable);
        assert!(after.contains(
            "<note print-object=\"no\"><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>"
        ));

        // The raw source intentionally has no host for the following <chord/>
        // members. The hidden representation carrier consumes exactly the former
        // forward duration while preserving the complete pitched payload.
        let raw_observation = observe_final_notation_readback(&render);
        let canonical_observation = observe_final_notation_readback(&after);
        assert_ne!(
            raw_observation.pitched_timeline_signature,
            canonical_observation.pitched_timeline_signature
        );
        assert_eq!(
            raw_observation.pitched_payload_signature,
            canonical_observation.pitched_payload_signature
        );

        let (again, second_audit) =
            stage_final_orphan_chord_host_rest_candidate(&after, true);
        assert_eq!(again, after);
        assert_eq!(second_audit.candidates, 0);
        assert!(!second_audit.candidate_accepted);

        let rest_host = chord_host_score(
            "<note print-object=\"no\" print-spacing=\"no\"><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>",
        )
        .replace("<duration>96</duration>", "<duration>48</duration>")
        .replace("<note print-object=\"no\"><chord/>", "<note><chord/>");
        let (promoted, promotion_audit) =
            stage_final_orphan_chord_host_rest_candidate(&rest_host, true);
        assert!(promotion_audit.candidate_accepted);
        assert_eq!(promotion_audit.candidates, 1);
        assert_eq!(promotion_audit.forward_nodes_accepted, 0);
        assert_eq!(promotion_audit.anchor_promotions_accepted, 1);
        assert_eq!(promotion_audit.chord_notes_accepted, 1);
        assert_eq!(promotion_audit.candidate_rows_after, 0);
        assert!(!promoted.contains(
            "<note print-object=\"no\"><rest/><duration>48</duration><voice>1</voice><staff>1</staff></note>"
        ));
        assert!(promoted.contains(
            "<note><pitch><step>F</step><octave>6</octave></pitch><duration>48</duration><voice>1</voice>"
        ));
        assert!(promoted.contains(
            "<note><chord/><pitch><step>C</step><octave>6</octave></pitch><duration>48</duration><voice>1</voice>"
        ));

        let beamed_rest_host = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><forward><duration>288</duration></forward><note print-object=\"no\" print-spacing=\"no\"><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><chord/><pitch><step>F</step><octave>4</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><chord/><pitch><step>D</step><octave>5</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>24</duration><voice>4</voice><type>16th</type><staff>2</staff><beam number=\"1\">begin</beam></note><note><pitch><step>B</step><octave>3</octave></pitch><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff><beam number=\"1\">end</beam></note></measure></part></score-partwise>";
        let (beamed_promoted, beamed_audit) =
            stage_final_orphan_chord_host_rest_candidate(beamed_rest_host, true);
        assert!(beamed_audit.candidate_accepted);
        assert_eq!(beamed_audit.anchor_promotions_accepted, 1);
        assert!(beamed_promoted.contains(
            "<beam number=\"1\">begin</beam>\n    <beam number=\"2\">begin</beam>"
        ));
        assert!(beamed_promoted.contains(
            "<beam number=\"1\">continue</beam>\n    <beam number=\"2\">end</beam>"
        ));
        assert!(beamed_promoted.contains("<beam number=\"1\">end</beam>"));
    }

    #[test]
    fn non_overhang_master_bound_orphan_is_in_existing_candidate_domain() {
        let render = chord_host_score("<forward><duration>48</duration></forward>")
            .replace("<duration>96</duration>", "<duration>48</duration>");
        let (replacements, chord_notes) =
            collect_final_orphan_chord_host_replacements(&render)
                .expect("forward/orphan representation domain must be readable");
        assert_eq!(replacements.len(), 1);
        assert_eq!(replacements[0].duration, 48);
        assert!(replacements[0].xml.contains("print-object=\"no\""));
        assert!(replacements[0].xml.contains("<rest/>"));
        assert_eq!(chord_notes, 2);
    }

    #[test]
    fn orphan_host_identity_is_carrier_bound_not_master_chord_projection_bound() {
        let render = chord_host_score(
            "<forward><duration>48</duration><voice>1</voice><staff>1</staff></forward>",
        )
        .replace("<duration>96</duration>", "<duration>48</duration>");
        let (replacements, chord_notes) =
            collect_final_orphan_chord_host_replacements(&render)
                .expect("forward owns cursor transport while FinalChord owns pitched occurrences");
        assert_eq!(replacements.len(), 1);
        assert_eq!(replacements[0].duration, 48);
        assert!(replacements[0].xml.contains("<voice>1</voice><staff>1</staff>"));
        assert_eq!(chord_notes, 2);
    }

    #[test]
    fn converts_forward_carrier_without_touching_pitched_events() {
        let xml = score("<note><rest/><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>96</duration></forward><forward><duration>96</duration></forward><forward><duration>96</duration></forward>");
        let (after, audit, receipts) =
            apply_silent_voice_visible_measure_coverage_render_write(&xml, true, None);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(audit.candidate_accepted);
        assert_eq!(audit.candidates, 1);
        assert_eq!(audit.forward_nodes_accepted, 3);
        assert_eq!(audit.candidate_rows_after, 0);
        assert!(!after.contains("<forward>"));
        assert!(audit.pitched_event_multiset_stable);
        assert!(audit.raw_cursor_stable);

        // r1124 regression: exercise the actual producer early-return path and
        // carry its generated result line through the real report relabeling and
        // CorpusRunManifest project-evidence consumer. The first measure has a
        // silent upper-secondary lane whose three concrete rest materializations
        // all overlap the upper-primary whole note and are therefore positively
        // withheld as canonical shadow-rest conflicts. The second measure only
        // supplies a bound upper-secondary voice label for the frozen plan.
        let suppressed_xml = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note><backup><duration>384</duration></backup><note><rest/><duration>96</duration><voice>3</voice><type>quarter</type><staff>1</staff></note><forward><duration>96</duration><voice>3</voice><staff>1</staff></forward><forward><duration>96</duration><voice>3</voice><staff>1</staff></forward><forward><duration>96</duration><voice>3</voice><staff>1</staff></forward></measure><measure number=\"{}\"><note><pitch><step>E</step><octave>5</octave></pitch><duration>384</duration><voice>3</voice><type>whole</type><staff>1</staff></note></measure></part></score-partwise>",
            1usize,
            2usize,
        );
        let evidence = vec![
            crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: 1,
                source_event_id: 11,
                attack_id: 101,
                measure: 1,
                raw_note_index: 0,
                onset: 0,
                duration: 384,
                pitch: 72,
                staff: 1,
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            },
            crate::structural_write_authority::VoiceFreeFinalChordEvidenceRow {
                fragment_id: 2,
                source_event_id: 12,
                attack_id: 102,
                measure: 2,
                raw_note_index: 0,
                onset: 0,
                duration: 384,
                pitch: 76,
                staff: 1,
                chord_syntax: false,
                tie_start: false,
                tie_stop: false,
            },
        ];
        let (canonical, _) =
            crate::structural_write_authority::plan_canonical_final_chord_structure_once(
                &suppressed_xml,
                &evidence,
            )
            .expect("suppression regression canonical plan");
        let attacks = vec![
            crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: 11,
                attack_id: 101,
                absolute_start: 0,
                absolute_end: 384,
                staff: 1,
                technical_lane: 1,
            },
            crate::structural_write_authority::FrozenVoiceAttackEvidenceRow {
                source_event_id: 12,
                attack_id: 102,
                absolute_start: 384,
                absolute_end: 768,
                staff: 1,
                technical_lane: 2,
            },
        ];
        let channels =
            crate::structural_write_authority::build_frozen_serialization_voice_channels_for_canonical(
                &canonical,
                &attacks,
            )
            .expect("suppression regression voice channels");
        let labels = vec![
            crate::structural_write_authority::LegacyVoiceLabelPreferenceRow {
                source_event_id: 11,
                label: "1".to_string(),
            },
            crate::structural_write_authority::LegacyVoiceLabelPreferenceRow {
                source_event_id: 12,
                label: "3".to_string(),
            },
        ];
        let voice_plan = crate::structural_write_authority::bind_musicxml_voice_labels(
            &channels,
            &labels,
        )
        .expect("suppression regression bound voice plan");

        let (suppressed_after, suppressed_audit, receipts) =
            apply_silent_voice_visible_measure_coverage_render_write(
                &suppressed_xml,
                true,
                Some(&voice_plan),
            );
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(suppressed_after, suppressed_xml);
        assert_eq!(suppressed_audit.candidates, 1);
        assert_eq!(suppressed_audit.forward_nodes_planned, 0);
        assert_eq!(suppressed_audit.forward_nodes_accepted, 0);
        assert_eq!(suppressed_audit.candidate_rows_after, 1);
        assert!(suppressed_audit.exact_candidate_delta);
        assert!(!suppressed_audit.candidate_accepted);
        assert!(suppressed_audit.pipeline_line.contains("CandidateRowsAfter 1; ExactCandidateDelta ja"));
        assert!(suppressed_audit.pipeline_line.contains("ForwardNodesReplaced 0"));
        assert!(suppressed_audit.pipeline_line.contains("RenderXmlWrite nein"));

        let result = format!(
            "ETUSCHLOPF-ERGEBNIS\nWerkzeug: etuschlopf {}\nErgebnis: OK\nRendererOutcome: observed_test\n{}\nFinalBeamLostBeginRenderWriterStage v{}: Attempted ja; SelectedRenderSource ja; MasterSourceBound ja; WriteEnabled ja; Candidates 0; NotesWritten 0; CandidateRowsAfter 0; RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein; MasterApply nein; Urteil test.\nFinalRestVerticalPlacementRenderWriterStage v{}: Attempted ja; SelectedRenderSource ja; FinalClefBound ja; WriteEnabled ja; Candidates 0; AnchorsWritten 0; CandidateRowsAfter 0; RenderXmlWrite nein; NotationMutation nein; LegacyMusicMutation nein; MasterApply nein; Urteil test.\n",
            env!("CARGO_PKG_VERSION"),
            suppressed_audit.pipeline_line,
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_VERSION"),
        );
        let result = crate::report::clarify_legacy_music_mutation_report(result);
        let manifest_evidence = crate::corpus_run_manifest::project_evidence(
            "SuppressedProofTransport",
            std::path::Path::new("SuppressedProofTransport.result.txt"),
            &result,
            Some(0),
            true,
        );
        assert!(manifest_evidence.writer_outcome_observed);
        assert_eq!(manifest_evidence.candidate_rows_after, 0);
        assert!(!manifest_evidence.render_xml_write);
        assert!(!manifest_evidence.notation_mutation);
    }

    #[test]
    fn primary_beam_voice_write_evidence_requires_committed_sink() {
        let mut audit = SilentVoiceVisibleMeasureCoverageRenderWriteAudit {
            primary_beam_voice_elements_written: 2,
            ..SilentVoiceVisibleMeasureCoverageRenderWriteAudit::default()
        };

        audit.commit_outcome = FinalNotationCommitOutcome::Rejected;
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        assert!(audit.pipeline_line.contains("PrimaryBeamVoiceElementsWritten 0"));

        audit.commit_outcome = FinalNotationCommitOutcome::Committed;
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        assert!(audit.pipeline_line.contains("PrimaryBeamVoiceElementsWritten 2"));
    }

    #[test]
    fn accepted_candidate_is_not_projected_as_written_before_commit() {
        let xml = score("<note><rest/><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>96</duration></forward><forward><duration>96</duration></forward><forward><duration>96</duration></forward>");
        let (_, mut audit, receipts) =
            apply_silent_voice_visible_measure_coverage_render_write(&xml, true, None);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(audit.candidate_accepted);
        assert!(audit.exact_candidate_delta);
        assert_eq!(audit.forward_nodes_accepted, 3);
        assert!(audit.pipeline_line.contains("ForwardNodesReplaced 0"));
        assert!(audit.pipeline_line.contains("RenderXmlWrite nein"));

        audit.commit_outcome = FinalNotationCommitOutcome::Committed;
        finalize_silent_voice_visible_measure_coverage_render_write_audit(&mut audit);
        assert!(audit.pipeline_line.contains("ForwardNodesReplaced 3"));
        assert!(audit.pipeline_line.contains("RenderXmlWrite ja"));
        assert!(audit.pipeline_line.contains(
            "silent_voice_visible_measure_coverage_written_render_xml_only"
        ));
    }

    #[test]
    fn rejected_commit_preserves_candidate_proof_without_write_claim() {
        let xml = rest_anchor_score("B", "4", false);
        let (_, mut audit) =
            stage_final_rest_vertical_placement_candidate(&xml, true);
        assert!(audit.candidate_accepted);
        assert!(audit.exact_candidate_delta);
        assert_eq!(audit.anchors_accepted, 1);

        audit.commit_outcome = FinalNotationCommitOutcome::Rejected;
        finalize_final_rest_vertical_placement_render_write_audit(&mut audit);
        assert!(audit.candidate_accepted);
        assert!(audit.exact_candidate_delta);
        assert_eq!(audit.anchors_accepted, 1);
        assert!(audit.pipeline_line.contains("AnchorsWritten 0"));
        assert!(audit.pipeline_line.contains("ExactCandidateDelta nein"));
        assert!(audit.pipeline_line.contains("RenderXmlWrite nein"));
        assert!(audit.pipeline_line.contains(
            "final_rest_vertical_placement_authorized_sink_rejected"
        ));
    }

    #[test]
    fn does_not_materialize_cross_staff_auxiliary_rest_lane() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>2</staff></note><backup><duration>96</duration></backup><note><rest/><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><note><pitch><step>D</step><octave>4</octave></pitch><duration>288</duration><voice>1</voice><type>half</type><dot/><staff>1</staff></note><backup><duration>384</duration></backup><forward><duration>384</duration><voice>2</voice><staff>2</staff></forward></measure></part></score-partwise>";
        let (after, audit, receipts) =
            apply_silent_voice_visible_measure_coverage_render_write(xml, true, None);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert!(audit.candidate_accepted);
        assert_eq!(audit.forward_nodes_planned, 0);
        assert_eq!(audit.visible_rests_before, 1);
        assert_eq!(audit.visible_rests_after, 0);
        assert_eq!(audit.candidate_rows_after, 0);
        assert!(audit.pitched_event_multiset_stable);
        assert!(audit.raw_cursor_stable);
        assert!(!after.contains("<rest/>"));
    }


    #[test]
    fn stages_candidate_without_writing_when_disabled() {
        let xml = score("<note><rest/><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>96</duration></forward><forward><duration>96</duration></forward><forward><duration>96</duration></forward>");
        let (after, audit, receipts) =
            apply_silent_voice_visible_measure_coverage_render_write(&xml, false, None);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(after, xml);
        assert!(!audit.candidate_accepted);
        assert!(!audit.write_enabled);
        assert_eq!(audit.candidates, 1);
        assert_eq!(audit.forward_nodes_planned, 3);
        assert_eq!(audit.forward_nodes_accepted, 0);
        assert_eq!(audit.verdict, "silent_voice_visible_measure_coverage_writer_staged_no_write");
    }


    fn beam_score(first_beam: &str) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note><backup><duration>384</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff>{first_beam}</note><note><pitch><step>D</step><octave>3</octave></pitch><duration>48</duration><voice>2</voice><type>eighth</type><staff>2</staff><beam number=\"1\">end</beam></note><note><pitch><step>E</step><octave>3</octave></pitch><duration>288</duration><voice>2</voice><type>half</type><dot/></note></measure></part></score-partwise>"
        )
    }

    fn double_beam_score(first_beams: &str) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><note><pitch><step>C</step><octave>5</octave></pitch><duration>384</duration><voice>1</voice><type>whole</type><staff>1</staff></note><backup><duration>384</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff>{first_beams}</note><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number=\"1\">continue</beam><beam number=\"2\">continue</beam></note><note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number=\"1\">continue</beam><beam number=\"2\">continue</beam></note><note><pitch><step>F</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><beam number=\"1\">end</beam><beam number=\"2\">end</beam></note><note><pitch><step>G</step><octave>3</octave></pitch><duration>288</duration><voice>2</voice><type>half</type><dot/><staff>2</staff></note></measure></part></score-partwise>"
        )
    }

    #[test]
    fn committed_gate_detects_primary_lost_begin_without_repairing() {
        let master = beam_score("<beam number=\"1\">begin</beam>");
        let render = beam_score("");
        assert_eq!(
            final_pdf_source_lost_begin_regression_candidates(&master, &render),
            Some(1)
        );
        assert_eq!(render, beam_score(""));
    }

    #[test]
    fn committed_gate_detects_all_missing_begin_levels_for_complete_group() {
        let master = double_beam_score(
            "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>",
        );
        let render = double_beam_score("");
        assert_eq!(
            final_pdf_source_lost_begin_regression_candidates(&master, &render),
            Some(1)
        );
    }

    #[test]
    fn committed_gate_is_closed_when_group_begin_is_present() {
        let master = beam_score("<beam number=\"1\">begin</beam>");
        let render = master.clone();
        assert_eq!(
            final_pdf_source_lost_begin_regression_candidates(&master, &render),
            Some(0)
        );
    }

    #[test]
    fn rejects_lane_with_pitched_note() {
        let xml = score("<note><pitch><step>D</step><octave>4</octave></pitch><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note><forward><duration>288</duration></forward>");
        let (after, audit, receipts) =
            apply_silent_voice_visible_measure_coverage_render_write(&xml, true, None);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        assert_eq!(after, xml);
        assert_eq!(audit.candidates, 0);
        assert!(!audit.candidate_accepted);
    }

    fn rest_anchor_score(display_step: &str, display_octave: &str, overlapping_voice: bool) -> String {
        let overlap = if overlapping_voice {
            "<backup><duration>384</duration></backup><note><pitch><step>C</step><octave>3</octave></pitch><duration>384</duration><voice>3</voice><type>whole</type><staff>2</staff></note>"
        } else {
            ""
        };
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"1\"><sign>G</sign><line>2</line></clef><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><rest><display-step>{display_step}</display-step><display-octave>{display_octave}</display-octave></rest><duration>384</duration><voice>2</voice><type>whole</type><staff>2</staff></note>{overlap}</measure></part></score-partwise>"
        )
    }

    #[test]
    fn rest_anchor_target_is_final_clef_bound_and_idempotent() {
        let xml = rest_anchor_score("B", "4", false);
        let targets = final_rest_structural_targets(&xml).expect("rest score parses");
        assert_eq!(targets.len(), 1);
        let target = &targets[0];
        assert_eq!(target.display_step_before, "B");
        assert_eq!(target.expected_step, "D");
        assert_eq!(target.display_octave_before, "4");
        assert_eq!(target.expected_octave, "3");

        let (after, audit) =
            stage_final_rest_vertical_placement_candidate(&xml, true);
        assert!(audit.candidate_accepted);
        assert_eq!(audit.candidates, 1);
        assert_eq!(audit.anchors_accepted, 1);
        assert_eq!(audit.candidate_rows_after, 0);
        assert!(after.contains("<display-step>D</display-step>"));
        assert!(after.contains("<display-octave>3</display-octave>"));

        let (again, second_audit) =
            stage_final_rest_vertical_placement_candidate(&after, true);
        assert_eq!(again, after);
        assert_eq!(second_audit.candidates, 0);
        assert!(!second_audit.candidate_accepted);
    }

    #[test]
    fn polyphonic_whole_rest_is_owned_by_final_rest_placement_writer() {
        let xml = rest_anchor_score("F", "3", true);
        let targets = final_rest_structural_targets(&xml).expect("rest score parses");
        assert_eq!(targets.len(), 1);
        assert_eq!(
            targets[0].decision,
            FinalRestPlacementDecision::PolyphonicWholeRestSeparation
        );
        assert_eq!(targets[0].expected_step, "A");
        assert_eq!(targets[0].expected_octave, "3");

        let (after, audit) =
            stage_final_rest_vertical_placement_candidate(&xml, true);
        assert!(audit.candidate_accepted);
        assert_eq!(audit.canonical_rebind_candidates, 0);
        assert_eq!(audit.polyphonic_whole_rest_candidates, 1);
        assert!(!audit.candidate_music_mutation);
        assert!(after.contains("<display-step>A</display-step>"));
        assert!(after.contains("<display-octave>3</display-octave>"));

        let (again, second_audit) =
            stage_final_rest_vertical_placement_candidate(&after, true);
        assert_eq!(again, after);
        assert_eq!(second_audit.candidates, 0);
        assert!(!second_audit.candidate_accepted);
    }

    #[test]
    fn measure_rest_without_type_keeps_polyphonic_placement_capability() {
        let xml = rest_anchor_score("F", "3", true)
            .replace("<rest>", "<rest measure=\"yes\">")
            .replace("<type>whole</type>", "");
        let targets = final_rest_structural_targets(&xml).expect("measure rest parses");
        assert_eq!(targets.len(), 1);
        assert_eq!(
            targets[0].decision,
            FinalRestPlacementDecision::PolyphonicWholeRestSeparation
        );
    }

    #[test]
    fn plans_only_master_bound_missing_secondary_begin_in_complete_four_sixteenths() {
        let master = double_beam_score(
            r#"<beam number="1">begin</beam><beam number="2">begin</beam>"#,
        );
        let candidate = double_beam_score(r#"<beam number="1">begin</beam>"#);
        let candidates =
            final_pdf_source_higher_beam_regression_candidates(&master, &candidate)
                .expect("master-bound higher-beam regression detector");
        assert_eq!(candidates, 1);
    }

    #[test]
    fn complete_group_without_master_secondary_begin_stays_closed() {
        let master = double_beam_score(r#"<beam number="1">begin</beam>"#);
        let candidate = double_beam_score(r#"<beam number="1">begin</beam>"#);
        let candidates =
            final_pdf_source_higher_beam_regression_candidates(&master, &candidate)
                .expect("master without secondary begin stays closed");
        assert_eq!(candidates, 0);
    }

    #[test]
    fn split_secondary_pattern_is_not_reinterpreted_as_lost_group_begin() {
        let master = double_beam_score(
            r#"<beam number="1">begin</beam><beam number="2">begin</beam>"#,
        );
        let candidate = double_beam_score(r#"<beam number="1">begin</beam>"#)
            .replacen(
                r#"<beam number="2">continue</beam></note><note><pitch><step>E"#,
                r#"<beam number="2">begin</beam></note><note><pitch><step>E"#,
                1,
            );
        let candidates =
            final_pdf_source_higher_beam_regression_candidates(&master, &candidate)
                .expect("split secondary pattern stays closed");
        assert_eq!(candidates, 0);
    }

    #[test]
    fn mixed_eighth_sixteenth_group_is_not_promoted_to_complete_secondary_beam() {
        let master = double_beam_score(
            r#"<beam number="1">begin</beam><beam number="2">begin</beam>"#,
        )
        .replacen("<duration>24</duration>", "<duration>48</duration>", 1)
        .replacen("<type>16th</type>", "<type>eighth</type>", 1);
        let candidate = double_beam_score(r#"<beam number="1">begin</beam>"#)
            .replacen("<duration>24</duration>", "<duration>48</duration>", 1)
            .replacen("<type>16th</type>", "<type>eighth</type>", 1);
        let candidates =
            final_pdf_source_higher_beam_regression_candidates(&master, &candidate)
                .expect("mixed group stays closed");
        assert_eq!(candidates, 0);
    }

    #[test]
    fn primary_clef_projection_must_precede_final_rest_placement() {
        let high_notes = (0..6)
            .map(|_| "<note><pitch><step>B</step><octave>5</octave></pitch><duration>24</duration><voice>2</voice><staff>2</staff></note>")
            .collect::<String>();
        let xml = format!(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>C</step><octave>3</octave></pitch><duration>384</duration><voice>2</voice><staff>2</staff></note></measure><measure number=\"\
2\">{}<note><rest><display-step>D</display-step><display-octave>3</display-octave></rest><duration>240</duration><voice>2</voice><type>half</type><staff>2</staff></note></measure><measure number=\"\
3\"><note><pitch><step>C</step><octave>3</octave></pitch><duration>384</duration><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>",
            high_notes
        );

        let (rest_first, rest_first_audit) =
            stage_final_rest_vertical_placement_candidate(&xml, true);
        assert_eq!(rest_first, xml);
        assert_eq!(rest_first_audit.candidates, 0);
        let mut wrong_order = rest_first;
        primary_clef_probe(&mut wrong_order)
            .expect("primary clef projection");
        assert_eq!(
            final_rest_structural_targets(&wrong_order)
                .expect("wrong-order score parses")
                .len(),
            1
        );

        let mut primary_first = xml.clone();
        primary_clef_probe(&mut primary_first)
            .expect("primary clef projection");
        assert_ne!(primary_first, xml);
        let (closed, closed_audit) =
            stage_final_rest_vertical_placement_candidate(&primary_first, true);
        assert_eq!(closed_audit.candidates, 1);
        assert_eq!(closed_audit.candidate_rows_after, 0);
        assert!(closed.contains("<display-step>B</display-step><display-octave>4</display-octave>"));
        let mut primary_reentry = closed.clone();
        primary_clef_probe(&mut primary_reentry)
            .expect("primary clef reentry");
        assert_eq!(primary_reentry, closed);
        assert!(
            final_rest_structural_targets(&closed)
                .expect("closed score parses")
                .is_empty()
        );
    }

    fn stage_final_rest_vertical_placement_candidate(xml: &str, enabled: bool) -> (String, super::FinalRestVerticalPlacementRenderWriteAudit) {
        let (post, audit, receipts) = stage_final_rest_vertical_placement_candidate_with_receipts(xml, enabled);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        (post, audit)
    }

    fn stage_final_orphan_chord_host_rest_candidate(xml: &str, enabled: bool) -> (String, super::FinalOrphanChordHostRestRenderWriteAudit) {
        let (post, audit, receipts) = stage_final_orphan_chord_host_rest_candidate_with_receipts(xml, enabled);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
        (post, audit)
    }

    #[test]
    fn accepted_pdf_gap_transports_exact_owner_receipt() {
        let xml = score("<forward><duration>384</duration><voice>1</voice><staff>1</staff></forward>");
        let (post, audit, receipts) = apply_silent_voice_visible_measure_coverage_render_write_with_receipts(&xml, true, None);
        assert!(audit.candidate_accepted);
        assert_ne!(post, xml);
        assert_eq!(receipts.len(), 1);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn disabled_pdf_gap_has_no_owner_event() {
        let xml = score("<forward><duration>384</duration><voice>1</voice><staff>1</staff></forward>");
        let (post, audit, receipts) = apply_silent_voice_visible_measure_coverage_render_write_with_receipts(&xml, false, None);
        assert!(!audit.candidate_accepted);
        assert_eq!(post, xml);
        assert!(receipts.is_empty());
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn invalid_pdf_gap_has_no_pending_owner_chain() {
        let (post, audit, receipts) = apply_silent_voice_visible_measure_coverage_render_write_with_receipts("<not-xml", true, None);
        assert_eq!(post, "<not-xml");
        assert!(!audit.candidate_accepted);
        assert!(receipts.is_empty());
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn accepted_pdf_orphan_transports_only_accepted_owner() {
        let xml = chord_host_score("<forward><duration>48</duration></forward>");
        let (post, audit, receipts) = stage_final_orphan_chord_host_rest_candidate_with_receipts(&xml, true);
        assert!(audit.candidate_accepted);
        assert_ne!(post, xml);
        assert_eq!(receipts.len(), 1);
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }

    #[test]
    fn rejected_pdf_orphan_has_empty_chain() {
        let (post, audit, receipts) = stage_final_orphan_chord_host_rest_candidate_with_receipts("<not-xml", true);
        assert_eq!(post, "<not-xml");
        assert!(!audit.candidate_accepted);
        assert!(receipts.is_empty());
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    }



}
