// v0.10.1702 r02 CanonicalMetricBeamObservationBoundary cross-file interface closure.
// PartialBeatMeasureSpan/PartialBeatRawNote/PartialBeatNoteInfo are now owned by
// composer_metric_beam_observation_core; this consumer keeps identical semantics.

// v0.10.1818 PartialBeatPostRestRetiredRecoveryGraphClosureSuperCut:
// The late PartialBeatPostRestTripleBeamFinalRepair recovery/audit path is retired
// completely. The living post-Fragmented CursorLane owner transfer below remains
// unchanged and the committed-state regression detector remains the fail-closed
// authority for reappearance of the historical lane-split domain.
//
// v0.10.1603 FinalPartialBeatTripleBeamPlanAdoption:
// The cursor-neutral post-rest lane transfer derives write/readback expectations
// from the shared fail-closed BeamPlan. Candidate binding, lane stitching, XML
// serialization and all musical invariants remain owned by the living transfer.
//
// v0.10.1141 PartialBeatPostRestTripleBeamFinalRepair:
// v0.10.1140 correctly built a cursor-neutral lane stitch, but rejected it
// because the historic sounding hash was serialization-order-sensitive.  The
// note moved earlier in XML while retaining the same scope/staff/voice/onset/
// duration/pitch event, so the ordered hash changed although the musical event
// multiset did not.  This version keeps the strict cursor/readback guards and
// proves invariance with sorted, multiplicity-preserving event multisets.  The
// old ordered hash remains diagnostic only and must change for this stitch.
#[derive(Debug, Clone)]
struct HumanBoundPostRestTripleBeamCandidate {
    note_indices: [usize; 3],
    note_starts: [i64; 3],
    note_duration: i64,
    rest_voice: String,
    note_voice: String,
    staff: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HumanBoundCursorItemKind {
    Note,
    Forward,
    Backup,
}

#[derive(Debug, Clone)]
struct HumanBoundCursorItem {
    kind: HumanBoundCursorItemKind,
    start: usize,
    end: usize,
    text: String,
    duration: i64,
}

#[derive(Debug, Clone)]
struct HumanBoundLaneStitchPlan {
    candidate: HumanBoundPostRestTripleBeamCandidate,
    gap_forward: HumanBoundCursorItem,
    split_third_note: HumanBoundCursorItem,
}

#[derive(Debug, Clone, Copy, Default)]
struct HumanBoundTripleBeamReadback {
    exact: bool,
    serialized_contiguous_notes: usize,
    intervening_forward: usize,
    intervening_backup: usize,
}


#[derive(Debug, Clone, Default)]
struct PartialBeatLaneSerializationOwnerTransferStats {
    written: bool,
    candidate_groups: usize,
    target_measures: String,
    target_staff: String,
    rest_voice: String,
    note_voice: String,
    cross_voice_rest_owner: bool,
    swapped_forward_duration: i64,
    beam_plan_rows: usize,
    serialized_contiguous_notes: usize,
    intervening_forward: usize,
    intervening_backup: usize,
    event_multiset_delta_rows: usize,
    parse_after_ok: bool,
    backup_forward_after_ok: bool,
    exact_readback: bool,
    verdict: String,
}

pub(crate) struct PartialBeatLaneTransferPreservingMint {
    _sealed: (),
}

fn human_bound_cursor_lane_owner_transfer_xml(
    xml: &str,
) -> (String, PartialBeatLaneSerializationOwnerTransferStats) {
    let mut stats = PartialBeatLaneSerializationOwnerTransferStats {
        parse_after_ok: true,
        backup_forward_after_ok: true,
        verdict: "partial_beat_lane_serialization_owner_transfer_no_candidate".to_string(),
        ..PartialBeatLaneSerializationOwnerTransferStats::default()
    };
    let spans = partial_beat_measure_spans(xml);
    if spans.is_empty() {
        return (xml.to_string(), stats);
    }

    let (before_events, before_cursor_ok) = collect_render_xml_note_events(xml);
    let before_complete_multiset = human_bound_complete_note_event_multiset(&before_events);
    let mut inherited_divisions = 1_i64;
    let mut candidates =
        Vec::<(usize, String, i64, HumanBoundLaneStitchPlan)>::new();

    for (measure_index, span) in spans.iter().enumerate() {
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let cursor_items = human_bound_cursor_items(measure_text, &raw_notes);
        for candidate in
            human_bound_lower_post_rest_triple_beam_candidates(&infos, inherited_divisions)
        {
            if let Some(plan) =
                human_bound_lane_stitch_plan(measure_text, &candidate, &cursor_items)
            {
                candidates.push((
                    measure_index,
                    partial_beat_measure_number(measure_text).unwrap_or_else(|| "?".to_string()),
                    inherited_divisions,
                    plan,
                ));
            }
        }
    }

    stats.candidate_groups = candidates.len();
    if candidates.is_empty() {
        return (xml.to_string(), stats);
    }

    if candidates.len() == 1 {
        let (measure_index, target_measure, _target_divisions, plan) =
            candidates.pop().expect("single candidate");
        return human_bound_cursor_lane_owner_transfer_single_measure_xml(
            xml,
            measure_index,
            &target_measure,
            &plan,
        );
    }

    let mut by_measure =
        BTreeMap::<usize, Vec<(String, i64, HumanBoundLaneStitchPlan)>>::new();
    for (measure_index, target_measure, target_divisions, plan) in candidates {
        by_measure
            .entry(measure_index)
            .or_default()
            .push((target_measure, target_divisions, plan));
    }

    let mut current_xml = xml.to_string();
    let mut target_measures = Vec::<String>::new();
    let mut target_staffs = BTreeSet::<String>::new();
    let mut rest_voices = BTreeSet::<String>::new();
    let mut note_voices = BTreeSet::<String>::new();
    let mut adopted_groups = 0_usize;
    let mut ambiguous_measure_groups = 0_usize;
    let mut rejected_unique_groups = 0_usize;
    let mut all_adopted_exact = true;
    let mut last_rejection_verdict = None::<String>;

    for (measure_index, mut plans) in by_measure {
        if plans.len() != 1 {
            ambiguous_measure_groups += 1;
            continue;
        }
        let (target_measure, _target_divisions, plan) =
            plans.pop().expect("measure-local single candidate");
        let (candidate_xml, local_stats) =
            human_bound_cursor_lane_owner_transfer_single_measure_xml(
                &current_xml,
                measure_index,
                &target_measure,
                &plan,
            );
        if local_stats.written && candidate_xml != current_xml {
            current_xml = candidate_xml;
            adopted_groups += 1;
            target_measures.push(target_measure);
            if !local_stats.target_staff.is_empty() {
                target_staffs.insert(local_stats.target_staff.clone());
            }
            if !local_stats.rest_voice.is_empty() {
                rest_voices.insert(local_stats.rest_voice.clone());
            }
            if !local_stats.note_voice.is_empty() {
                note_voices.insert(local_stats.note_voice.clone());
            }
            stats.cross_voice_rest_owner |= local_stats.cross_voice_rest_owner;
            stats.swapped_forward_duration += local_stats.swapped_forward_duration;
            stats.beam_plan_rows += local_stats.beam_plan_rows;
            stats.serialized_contiguous_notes += local_stats.serialized_contiguous_notes;
            stats.intervening_forward += local_stats.intervening_forward;
            stats.intervening_backup += local_stats.intervening_backup;
            all_adopted_exact &= local_stats.exact_readback;
        } else {
            rejected_unique_groups += 1;
            last_rejection_verdict = Some(local_stats.verdict);
        }
    }

    stats.target_measures = target_measures.join(",");
    stats.target_staff = target_staffs.into_iter().collect::<Vec<_>>().join(",");
    stats.rest_voice = rest_voices.into_iter().collect::<Vec<_>>().join(",");
    stats.note_voice = note_voices.into_iter().collect::<Vec<_>>().join(",");

    if adopted_groups == 0 {
        stats.verdict = last_rejection_verdict.unwrap_or_else(|| {
            "partial_beat_lane_serialization_owner_transfer_ambiguous_measure_local_candidate"
                .to_string()
        });
        return (xml.to_string(), stats);
    }

    let (after_events, after_cursor_ok) = collect_render_xml_note_events(&current_xml);
    let after_complete_multiset = human_bound_complete_note_event_multiset(&after_events);
    stats.event_multiset_delta_rows = human_bound_sorted_multiset_delta_rows(
        &before_complete_multiset,
        &after_complete_multiset,
    );
    stats.parse_after_ok = parse_musicxml(&current_xml).is_ok();
    stats.backup_forward_after_ok = before_cursor_ok && after_cursor_ok;
    stats.exact_readback = adopted_groups > 0 && all_adopted_exact;
    stats.written = adopted_groups > 0 && current_xml != xml;

    let green = stats.written
        && xml.matches("<note").count() == current_xml.matches("<note").count()
        && stats.event_multiset_delta_rows == 0
        && stats.parse_after_ok
        && stats.backup_forward_after_ok
        && stats.exact_readback
        && stats.intervening_forward == 0
        && stats.intervening_backup == 0;
    if !green {
        stats.written = false;
        stats.verdict =
            "partial_beat_lane_serialization_owner_transfer_rejected_by_invariant_guard"
                .to_string();
        return (xml.to_string(), stats);
    }

    stats.verdict = if ambiguous_measure_groups > 0 || rejected_unique_groups > 0 {
        "partial_beat_lane_serialization_owner_transfer_measure_local_partial_green".to_string()
    } else {
        "partial_beat_lane_serialization_owner_transfer_measure_local_multiplicity_green"
            .to_string()
    };

    (current_xml, stats)
}

fn human_bound_cursor_lane_owner_transfer_single_measure_xml(
    xml: &str,
    target_measure_index: usize,
    target_measure: &str,
    plan: &HumanBoundLaneStitchPlan,
) -> (String, PartialBeatLaneSerializationOwnerTransferStats) {
    let mut stats = PartialBeatLaneSerializationOwnerTransferStats {
        candidate_groups: 1,
        target_measures: target_measure.to_string(),
        parse_after_ok: true,
        backup_forward_after_ok: true,
        verdict: "partial_beat_lane_serialization_owner_transfer_no_candidate".to_string(),
        ..PartialBeatLaneSerializationOwnerTransferStats::default()
    };
    let note_count_before = xml.matches("<note").count();
    let (before_events, before_cursor_ok) = collect_render_xml_note_events(xml);
    let before_complete_multiset = human_bound_complete_note_event_multiset(&before_events);
    let candidate = &plan.candidate;
    stats.target_staff = candidate.staff.clone();
    stats.rest_voice = candidate.rest_voice.clone();
    stats.note_voice = candidate.note_voice.clone();
    stats.cross_voice_rest_owner = candidate.rest_voice != candidate.note_voice;
    stats.swapped_forward_duration = plan.gap_forward.duration;

    let mut stitched_xml = String::with_capacity(xml.len() + 128);
    let mut last = 0_usize;
    for (measure_index, span) in partial_beat_measure_spans(xml).into_iter().enumerate() {
        stitched_xml.push_str(&xml[last..span.start]);
        let measure_text = &xml[span.start..span.end];
        if measure_index == target_measure_index {
            stitched_xml.push_str(&human_bound_swap_gap_forward_with_split_third_note(
                measure_text,
                plan,
            ));
        } else {
            stitched_xml.push_str(measure_text);
        }
        last = span.end;
    }
    stitched_xml.push_str(&xml[last..]);
    if stitched_xml == xml {
        stats.verdict =
            "partial_beat_lane_serialization_owner_transfer_stitch_noop".to_string();
        return (xml.to_string(), stats);
    }

    let target_starts = candidate.note_starts;
    let target_starts_set = target_starts.iter().copied().collect::<BTreeSet<_>>();
    let mut beamed_xml = String::with_capacity(stitched_xml.len() + 128);
    let mut last = 0_usize;
    let mut plan_rows = 0_usize;
    for (measure_index, span) in partial_beat_measure_spans(&stitched_xml)
        .into_iter()
        .enumerate()
    {
        beamed_xml.push_str(&stitched_xml[last..span.start]);
        let measure_text = &stitched_xml[span.start..span.end];
        if measure_index != target_measure_index {
            beamed_xml.push_str(measure_text);
            last = span.end;
            continue;
        }
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let mut target_infos = infos
            .iter()
            .filter(|info| {
                info.staff == candidate.staff
                    && info.voice == candidate.note_voice
                    && target_starts_set.contains(&info.start)
                    && !info.is_rest
                    && !info.is_hidden
                    && !info.is_chord
                    && info.note_type == "16th"
            })
            .collect::<Vec<_>>();
        target_infos.sort_by_key(|info| (info.start, info.index));
        if target_infos.len() != target_starts.len()
            || !target_infos
                .iter()
                .map(|info| info.start)
                .eq(target_starts.iter().copied())
        {
            stats.verdict =
                "partial_beat_lane_serialization_owner_transfer_target_readback_missing"
                    .to_string();
            return (xml.to_string(), stats);
        }

        let mut states_by_note = BTreeMap::<usize, Vec<(usize, &'static str)>>::new();
        let Some(rows) =
            partial_beat_extend_states_from_plan(target_infos.as_slice(), &mut states_by_note)
        else {
            stats.verdict =
                "partial_beat_lane_serialization_owner_transfer_general_beam_plan_rejected"
                    .to_string();
            return (xml.to_string(), stats);
        };
        if rows != 6 || states_by_note.len() != 3 {
            stats.verdict =
                "partial_beat_lane_serialization_owner_transfer_general_beam_plan_incomplete"
                    .to_string();
            return (xml.to_string(), stats);
        }

        let mut patched_measure = String::with_capacity(measure_text.len() + 128);
        let mut note_last = 0_usize;
        for raw in &raw_notes {
            patched_measure.push_str(&measure_text[note_last..raw.start]);
            if let Some(states) = states_by_note.get(&raw.start) {
                let cleaned = partial_beat_strip_beam_tags(&raw.text);
                patched_measure.push_str(&partial_beat_insert_beam_tags(&cleaned, states));
            } else {
                patched_measure.push_str(&raw.text);
            }
            note_last = raw.end;
        }
        patched_measure.push_str(&measure_text[note_last..]);
        beamed_xml.push_str(&patched_measure);
        plan_rows = rows;
        last = span.end;
    }
    beamed_xml.push_str(&stitched_xml[last..]);
    stats.beam_plan_rows = plan_rows;

    let (after_events, after_cursor_ok) = collect_render_xml_note_events(&beamed_xml);
    let after_complete_multiset = human_bound_complete_note_event_multiset(&after_events);
    stats.event_multiset_delta_rows = human_bound_sorted_multiset_delta_rows(
        &before_complete_multiset,
        &after_complete_multiset,
    );
    stats.parse_after_ok = parse_musicxml(&beamed_xml).is_ok();
    stats.backup_forward_after_ok = before_cursor_ok && after_cursor_ok;
    let readback = human_bound_lower_post_rest_triple_beam_exact_readback(
        &beamed_xml,
        target_measure_index,
        &candidate.staff,
        &candidate.note_voice,
        candidate.note_starts,
    );
    stats.exact_readback = readback.exact;
    stats.serialized_contiguous_notes = readback.serialized_contiguous_notes;
    stats.intervening_forward = readback.intervening_forward;
    stats.intervening_backup = readback.intervening_backup;

    let green = note_count_before == beamed_xml.matches("<note").count()
        && stats.candidate_groups == 1
        && stats.beam_plan_rows == 6
        && stats.event_multiset_delta_rows == 0
        && stats.parse_after_ok
        && stats.backup_forward_after_ok
        && stats.exact_readback
        && stats.serialized_contiguous_notes == 3
        && stats.intervening_forward == 0
        && stats.intervening_backup == 0;
    if !green {
        stats.verdict =
            "partial_beat_lane_serialization_owner_transfer_rejected_by_invariant_guard"
                .to_string();
        return (xml.to_string(), stats);
    }

    stats.written = beamed_xml != xml;
    stats.verdict =
        "partial_beat_lane_serialization_owner_transfer_cursor_lane_plus_general_beam_plan_green"
            .to_string();
    (beamed_xml, stats)
}

type HumanBoundCompleteNoteEventKey = (
    usize,
    String,
    String,
    i32,
    i32,
    bool,
    bool,
    bool,
    Option<i32>,
);

fn human_bound_complete_note_event_multiset(
    events: &[RenderXmlNoteEvent],
) -> Vec<HumanBoundCompleteNoteEventKey> {
    let mut keys = events
        .iter()
        .map(|event| {
            (
                event.scope,
                event.staff.clone(),
                event.voice.clone(),
                event.onset,
                event.duration,
                event.is_hidden,
                event.is_rest,
                event.is_chord,
                event.midi,
            )
        })
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

pub(crate) fn e057_existing_event_multiset_projection(xml: &str) -> Option<String> {
    roxmltree::Document::parse(xml).ok()?;
    let (events, cursor_ok) = collect_render_xml_note_events(xml);
    if !cursor_ok {
        return None;
    }
    let keys = human_bound_complete_note_event_multiset(&events);
    Some(format!("{keys:?}"))
}

fn human_bound_sorted_multiset_delta_rows<T: Ord>(before: &[T], after: &[T]) -> usize {
    let mut before_index = 0_usize;
    let mut after_index = 0_usize;
    let mut delta = 0_usize;
    while before_index < before.len() && after_index < after.len() {
        match before[before_index].cmp(&after[after_index]) {
            std::cmp::Ordering::Less => {
                delta += 1;
                before_index += 1;
            }
            std::cmp::Ordering::Greater => {
                delta += 1;
                after_index += 1;
            }
            std::cmp::Ordering::Equal => {
                before_index += 1;
                after_index += 1;
            }
        }
    }
    delta + (before.len() - before_index) + (after.len() - after_index)
}

pub(crate) fn partial_beat_post_rest_lane_split_regression_candidates(
    xml: &str,
) -> Option<usize> {
    parse_musicxml(xml).ok()?;
    let spans = partial_beat_measure_spans(xml);
    let mut inherited_divisions = 1_i64;
    let mut candidates = 0_usize;
    for span in spans {
        let measure_text = &xml[span.start..span.end];
        inherited_divisions = partial_beat_first_i64(measure_text, "divisions")
            .unwrap_or(inherited_divisions)
            .max(1);
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let cursor_items = human_bound_cursor_items(measure_text, &raw_notes);
        candidates = candidates.saturating_add(
            human_bound_lower_post_rest_triple_beam_candidates(&infos, inherited_divisions)
                .into_iter()
                .filter(|candidate| {
                    human_bound_lane_stitch_plan(measure_text, candidate, &cursor_items).is_some()
                })
                .count(),
        );
    }
    Some(candidates)
}

fn human_bound_cursor_items(
    measure_text: &str,
    raw_notes: &[PartialBeatRawNote],
) -> Vec<HumanBoundCursorItem> {
    let mut items = Vec::new();
    let mut scan = 0_usize;
    loop {
        let next_note = measure_text[scan..].find("<note").map(|p| scan + p);
        let next_backup = measure_text[scan..].find("<backup").map(|p| scan + p);
        let next_forward = measure_text[scan..].find("<forward").map(|p| scan + p);
        let mut next = Vec::new();
        if let Some(pos) = next_note { next.push((pos, HumanBoundCursorItemKind::Note)); }
        if let Some(pos) = next_backup { next.push((pos, HumanBoundCursorItemKind::Backup)); }
        if let Some(pos) = next_forward { next.push((pos, HumanBoundCursorItemKind::Forward)); }
        next.sort_by_key(|(pos, _)| *pos);
        let Some((pos, kind)) = next.first().copied() else { break; };

        match kind {
            HumanBoundCursorItemKind::Note => {
                let Some(raw) = raw_notes.iter().find(|note| note.start == pos) else { break; };
                items.push(HumanBoundCursorItem {
                    kind,
                    start: raw.start,
                    end: raw.end,
                    text: raw.text.clone(),
                    duration: partial_beat_first_i64(&raw.text, "duration").unwrap_or(0).max(0),
                });
                scan = raw.end;
            }
            HumanBoundCursorItemKind::Forward | HumanBoundCursorItemKind::Backup => {
                let close = if kind == HumanBoundCursorItemKind::Forward {
                    "</forward>"
                } else {
                    "</backup>"
                };
                let Some(end) = measure_text[pos..].find(close).map(|rel| pos + rel + close.len()) else { break; };
                let item_text = measure_text[pos..end].to_string();
                items.push(HumanBoundCursorItem {
                    kind,
                    start: pos,
                    end,
                    duration: partial_beat_first_i64(&item_text, "duration").unwrap_or(0).max(0),
                    text: item_text,
                });
                scan = end;
            }
        }
    }
    items
}

fn human_bound_lane_stitch_plan(
    measure_text: &str,
    candidate: &HumanBoundPostRestTripleBeamCandidate,
    items: &[HumanBoundCursorItem],
) -> Option<HumanBoundLaneStitchPlan> {
    let first_pos = items.iter().position(|item| {
        item.kind == HumanBoundCursorItemKind::Note && item.start == candidate.note_indices[0]
    })?;
    let second_pos = items.iter().position(|item| {
        item.kind == HumanBoundCursorItemKind::Note && item.start == candidate.note_indices[1]
    })?;
    let third_pos = items.iter().position(|item| {
        item.kind == HumanBoundCursorItemKind::Note && item.start == candidate.note_indices[2]
    })?;
    if second_pos != first_pos + 1 || third_pos <= second_pos + 1 {
        return None;
    }
    let first = &items[first_pos];
    let second = &items[second_pos];
    let third = &items[third_pos];
    if !measure_text[first.end..second.start].trim().is_empty() {
        return None;
    }
    let gap_forward = items.get(second_pos + 1)?;
    if gap_forward.kind != HumanBoundCursorItemKind::Forward
        || gap_forward.duration != candidate.note_duration
        || !measure_text[second.end..gap_forward.start].trim().is_empty()
    {
        return None;
    }
    if !items[second_pos + 2..third_pos]
        .iter()
        .any(|item| item.kind == HumanBoundCursorItemKind::Backup)
    {
        return None;
    }
    Some(HumanBoundLaneStitchPlan {
        candidate: candidate.clone(),
        gap_forward: gap_forward.clone(),
        split_third_note: third.clone(),
    })
}

fn human_bound_swap_gap_forward_with_split_third_note(
    measure_text: &str,
    plan: &HumanBoundLaneStitchPlan,
) -> String {
    let forward = &plan.gap_forward;
    let third = &plan.split_third_note;
    if forward.start >= third.start || forward.end > third.start {
        return measure_text.to_string();
    }
    let mut out = String::with_capacity(measure_text.len());
    out.push_str(&measure_text[..forward.start]);
    out.push_str(&third.text);
    out.push_str(&measure_text[forward.end..third.start]);
    out.push_str(&forward.text);
    out.push_str(&measure_text[third.end..]);
    out
}

fn human_bound_exact_double_beam_plan() -> Option<structural_write_authority::BeamPlan> {
    let ordinals = [0_usize, 1, 2];
    structural_write_authority::BeamPlan::from_groups(3, &[(ordinals.as_slice(), 2)])
}

fn human_bound_note_matches_beam_plan_ordinal(
    note_text: &str,
    plan: &structural_write_authority::BeamPlan,
    event_ordinal: usize,
) -> bool {
    let expected = plan
        .edits()
        .iter()
        .filter(|edit| edit.event_ordinal() == event_ordinal)
        .copied()
        .collect::<Vec<_>>();
    note_text.match_indices("<beam").count() == expected.len()
        && !expected.is_empty()
        && expected.iter().all(|edit| {
            note_text.contains(&format!(
                "<beam number=\"{}\">{}</beam>",
                edit.level(),
                edit.state().as_musicxml_text()
            ))
        })
}

fn human_bound_lower_post_rest_triple_beam_candidates(
    infos: &[PartialBeatNoteInfo],
    beat_unit: i64,
) -> Vec<HumanBoundPostRestTripleBeamCandidate> {
    let mut pitched = infos
        .iter()
        .filter(|info| {
            info.staff == "2"
                && !info.is_rest
                && !info.is_hidden
                && !info.is_chord
                && info.duration > 0
                && info.note_type == "16th"
        })
        .collect::<Vec<_>>();
    pitched.sort_by_key(|info| (info.voice.clone(), info.start, info.index));

    let mut candidates = Vec::new();
    for window in pitched.windows(3) {
        let first = window[0];
        let second = window[1];
        let third = window[2];
        if first.voice != second.voice || first.voice != third.voice {
            continue;
        }
        if first.start + first.duration != second.start
            || second.start + second.duration != third.start
            || first.duration != second.duration
            || first.duration != third.duration
        {
            continue;
        }
        let beat = first.start.div_euclid(beat_unit);
        let beat_start = beat * beat_unit;
        let beat_end = (beat + 1) * beat_unit;
        if first.start <= beat_start
            || third.start + third.duration != beat_end
            || second.start.div_euclid(beat_unit) != beat
            || third.start.div_euclid(beat_unit) != beat
        {
            continue;
        }
        let rest = infos.iter().find(|rest| {
            rest.staff == first.staff
                && rest.is_rest
                && !rest.is_hidden
                && !rest.is_chord
                && rest.duration > 0
                && rest.start >= beat_start
                && rest.start + rest.duration == first.start
        });
        let Some(rest) = rest else { continue; };
        candidates.push(HumanBoundPostRestTripleBeamCandidate {
            note_indices: [first.index, second.index, third.index],
            note_starts: [first.start, second.start, third.start],
            note_duration: first.duration,
            rest_voice: rest.voice.clone(),
            note_voice: first.voice.clone(),
            staff: first.staff.clone(),
        });
    }
    candidates
}

fn human_bound_lower_post_rest_triple_beam_exact_readback(
    xml: &str,
    target_measure_index: usize,
    target_staff: &str,
    target_voice: &str,
    target_starts: [i64; 3],
) -> HumanBoundTripleBeamReadback {
    for (measure_index, span) in partial_beat_measure_spans(xml).into_iter().enumerate() {
        let measure_text = &xml[span.start..span.end];
        if measure_index != target_measure_index {
            continue;
        }
        let raw_notes = partial_beat_raw_notes(measure_text);
        let infos = partial_beat_note_infos(measure_text, &raw_notes);
        let Some(plan) = human_bound_exact_double_beam_plan() else {
            return HumanBoundTripleBeamReadback::default();
        };
        let mut target_notes = Vec::new();
        let mut beams_exact = true;
        for (event_ordinal, start) in target_starts.iter().enumerate() {
            let Some(info) = infos.iter().find(|info| {
                info.staff == target_staff
                    && info.voice == target_voice
                    && info.start == *start
                    && !info.is_rest
                    && !info.is_hidden
                    && !info.is_chord
                    && info.note_type == "16th"
            }) else {
                return HumanBoundTripleBeamReadback::default();
            };
            let Some(raw) = raw_notes.iter().find(|note| note.start == info.index) else {
                return HumanBoundTripleBeamReadback::default();
            };
            beams_exact &=
                human_bound_note_matches_beam_plan_ordinal(&raw.text, &plan, event_ordinal);
            target_notes.push(raw);
        }
        if target_notes.len() != 3
            || target_notes[0].start >= target_notes[1].start
            || target_notes[1].start >= target_notes[2].start
        {
            return HumanBoundTripleBeamReadback::default();
        }
        let between_1_2 = &measure_text[target_notes[0].end..target_notes[1].start];
        let between_2_3 = &measure_text[target_notes[1].end..target_notes[2].start];
        let mut contiguous = 1_usize;
        if between_1_2.trim().is_empty() {
            contiguous += 1;
            if between_2_3.trim().is_empty() {
                contiguous += 1;
            }
        }
        let between_all = format!("{between_1_2}{between_2_3}");
        let intervening_forward = between_all.match_indices("<forward").count();
        let intervening_backup = between_all.match_indices("<backup").count();
        return HumanBoundTripleBeamReadback {
            exact: beams_exact
                && contiguous == 3
                && intervening_forward == 0
                && intervening_backup == 0,
            serialized_contiguous_notes: contiguous,
            intervening_forward,
            intervening_backup,
        };
    }
    HumanBoundTripleBeamReadback::default()
}

#[cfg(test)]
mod partial_beat_post_rest_triple_beam_final_repair_tests {
    use super::*;

    fn split_target_xml(rest_voice: &str, gap_duration: i64) -> String {
        format!(
            "<score-partwise version=\"4.0\"><part id=\"P1\"><measure number=\"99\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward><note><rest/><duration>24</duration><type>16th</type><voice>{}</voice><staff>2</staff></note><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><forward><duration>{}</duration><voice>1</voice><staff>1</staff></forward><backup><duration>288</duration></backup><forward><duration>264</duration></forward><note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>",
            rest_voice, gap_duration
        )
    }

    fn two_split_targets_same_measure_xml() -> String {
        "<score-partwise version=\"4.0\"><part id=\"P1\"><measure number=\"99\"><attributes><divisions>96</divisions></attributes><forward><duration>192</duration></forward><note><rest/><duration>24</duration><type>16th</type><voice>1</voice><staff>2</staff></note><note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><pitch><step>D</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><forward><duration>24</duration><voice>1</voice><staff>1</staff></forward><backup><duration>288</duration></backup><forward><duration>264</duration></forward><note><pitch><step>E</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><rest/><duration>24</duration><type>16th</type><voice>1</voice><staff>2</staff></note><note><pitch><step>F</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><note><pitch><step>G</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note><forward><duration>24</duration><voice>1</voice><staff>1</staff></forward><backup><duration>96</duration></backup><forward><duration>72</duration></forward><note><pitch><step>A</step><octave>3</octave></pitch><duration>24</duration><type>16th</type><voice>2</voice><staff>2</staff></note></measure></part></score-partwise>".to_string()
    }

    #[test]
    fn retired_recovery_detector_sees_split_and_owner_closes_it() {
        let xml = split_target_xml("2", 24);
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&xml),
            Some(1)
        );
        let (after_owner, owner_stats) = human_bound_cursor_lane_owner_transfer_xml(&xml);
        assert!(owner_stats.written);
        assert_eq!(owner_stats.candidate_groups, 1);
        assert_eq!(owner_stats.beam_plan_rows, 6);
        assert!(owner_stats.exact_readback);
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&after_owner),
            Some(0)
        );
    }

    #[test]
    fn e057_owner_projection_accepts_lane_order_relocation() {
        let xml = split_target_xml("1", 24);
        let (after, stats) = human_bound_cursor_lane_owner_transfer_xml(&xml);
        assert!(stats.written);
        assert_eq!(stats.event_multiset_delta_rows, 0);
        assert!(stats.exact_readback);
        assert_ne!(xml, after);

        let receipt = crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(
            PartialBeatLaneTransferPreservingMint { _sealed: () },
            &xml,
            &after,
        )
        .expect("E057 event-multiset preserving relocation must mint its own receipt");
        assert!(receipt
            .validate_preserving_transition::<PartialBeatLaneTransferPreservingMint>(&xml, &after)
            .is_ok());

        let drifted = after.replacen("<step>E</step>", "<step>F</step>", 1);
        assert!(receipt
            .validate_preserving_transition::<PartialBeatLaneTransferPreservingMint>(&xml, &drifted)
            .is_err());
    }

    #[test]
    fn owner_transfer_preserves_cross_voice_rest_domain() {
        let xml = split_target_xml("1", 24);
        let (after, stats) = human_bound_cursor_lane_owner_transfer_xml(&xml);
        assert!(stats.written);
        assert!(stats.cross_voice_rest_owner);
        assert_eq!(stats.beam_plan_rows, 6);
        assert!(stats.exact_readback);
        assert_eq!(after.matches("<beam number=\"1\">").count(), 3);
        assert_eq!(after.matches("<beam number=\"2\">").count(), 3);
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&after),
            Some(0)
        );
    }

    #[test]
    fn detector_rejects_wrong_forward_duration() {
        let xml = split_target_xml("1", 48);
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&xml),
            Some(0)
        );
    }

    #[test]
    fn independent_measure_candidates_are_duplication_invariant() {
        let first = split_target_xml("1", 24);
        let measure_start = first.find("<measure").expect("first measure");
        let measure_end = first[measure_start..]
            .find("</measure>")
            .map(|offset| measure_start + offset + "</measure>".len())
            .expect("first measure end");
        let second_measure = first[measure_start..measure_end].replace(
            "number=\"99\"",
            "number=\"100\"",
        );
        let xml = first.replace("</part>", &format!("{second_measure}</part>"));
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&xml),
            Some(2)
        );
        let (after, stats) = human_bound_cursor_lane_owner_transfer_xml(&xml);
        assert!(stats.written);
        assert_eq!(stats.candidate_groups, 2);
        assert_eq!(stats.target_measures, "99,100");
        assert_eq!(stats.beam_plan_rows, 12);
        assert!(stats.exact_readback);
        assert_eq!(after.matches("<beam number=\"1\">").count(), 6);
        assert_eq!(after.matches("<beam number=\"2\">").count(), 6);
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&after),
            Some(0)
        );
    }

    #[test]
    fn adding_independent_measure_candidate_preserves_first_measure_decision() {
        let single = split_target_xml("1", 24);
        let (single_after, single_stats) = human_bound_cursor_lane_owner_transfer_xml(&single);
        assert!(single_stats.written);

        let measure_start = single.find("<measure").expect("first measure");
        let measure_end = single[measure_start..]
            .find("</measure>")
            .map(|offset| measure_start + offset + "</measure>".len())
            .expect("first measure end");
        let second_measure = single[measure_start..measure_end].replace(
            "number=\"99\"",
            "number=\"100\"",
        );
        let duplicated = single.replace("</part>", &format!("{second_measure}</part>"));
        let (duplicated_after, duplicated_stats) =
            human_bound_cursor_lane_owner_transfer_xml(&duplicated);
        assert!(duplicated_stats.written);

        let first_single = partial_beat_measure_spans(&single_after)
            .into_iter()
            .next()
            .map(|span| single_after[span.start..span.end].to_string())
            .expect("single repaired measure");
        let first_duplicated = partial_beat_measure_spans(&duplicated_after)
            .into_iter()
            .next()
            .map(|span| duplicated_after[span.start..span.end].to_string())
            .expect("first duplicated repaired measure");
        assert_eq!(first_duplicated, first_single);
    }
    #[test]
    fn multiple_candidates_inside_one_measure_remain_fail_closed() {
        let xml = two_split_targets_same_measure_xml();
        assert_eq!(
            partial_beat_post_rest_lane_split_regression_candidates(&xml),
            Some(2)
        );
        let (after, stats) = human_bound_cursor_lane_owner_transfer_xml(&xml);
        assert!(!stats.written);
        assert_eq!(stats.candidate_groups, 2);
        assert_eq!(after, xml);
    }

}


fn human_bound_cursor_lane_owner_transfer_xml_with_receipt(
    xml: &str,
) -> ((String, PartialBeatLaneSerializationOwnerTransferStats), Option<crate::master_state_identity::PreservingTransformationReceipt>) {
    let output = human_bound_cursor_lane_owner_transfer_xml(xml);
    let receipt = crate::master_state_identity::PreservingTransformationReceipt::mint_preserving_receipt(PartialBeatLaneTransferPreservingMint { _sealed: () }, xml, &output.0);
    assert!(output.0 == xml || receipt.is_some(), "E057 changed bytes without own receipt");
    (output, receipt)
}
