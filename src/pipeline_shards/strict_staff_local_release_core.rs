type RenderTickInterval = (i32, i32);
type StrictStaffOverlayRestLayer = (String, usize, i32, Vec<RenderTickInterval>);
type RenderNoteIdentityKey = (usize, String, String, i32, i32, i32);
type UpperStaffDuplicateVisibleRestSuppressionCandidate = (
    i64,
    usize,
    usize,
    String,
    usize,
    String,
    i32,
    String,
);
type UpperStaffBeatLocalCoalesceCandidate = (i64, usize, usize, usize, usize, String, i32, usize);

fn build_strict_staff_local_release_completeness_readback(
    xml: &str,
    source_label: &str,
) -> VisibleLaneCompletenessClassifierPurgeGateAudit {
    let (events, raw_time_ok) = collect_render_xml_note_events(xml);
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let mut by_lane: BTreeMap<(usize, String, String), Vec<RenderXmlNoteEvent>> = BTreeMap::new();
    for event in events {
        by_lane.entry((event.scope, event.staff.clone(), event.voice.clone())).or_default().push(event);
    }
    let mut audit = VisibleLaneCompletenessClassifierPurgeGateAudit {
        source_label: source_label.to_string(),
        measures_checked: metric_by_scope.len(),
        raw_time_ok,
        ..VisibleLaneCompletenessClassifierPurgeGateAudit::default()
    };
    for ((scope_ref, staff_ref, voice_ref), lane_events_src) in by_lane.iter() {
        let scope = *scope_ref;
        let staff = staff_ref.clone();
        let voice = voice_ref.clone();
        let mut lane_events = lane_events_src.clone();
        let staff_trimmed = staff.trim();
        if staff_trimmed != "1" && staff_trimmed != "2" { continue; }
        audit.lanes_checked += 1;
        lane_events.sort_by_key(|event| (event.onset, event.duration, event.is_chord));
        let visible = lane_events
            .iter()
            .filter(|event| !event.is_hidden && !event.is_chord && event.duration > 0 && (event.is_rest || event.midi.is_some()))
            .cloned()
            .collect::<Vec<_>>();
        if visible.is_empty() {
            audit.auxiliary_lanes += 1;
            continue;
        }
        let (expected, beat_unit) = metric_by_scope.get(&scope).copied().unwrap_or((384, 96));
        let intervals = visible.iter().map(|event| (event.onset, event.duration.max(0))).collect::<Vec<_>>();
        let merged = strict_staff_merge_visible_intervals(intervals, expected);
        let gaps = strict_staff_gaps_from_merged_intervals(&merged, expected);
        let union_duration: i32 = merged.iter().map(|(_, duration)| *duration).sum::<i32>().min(expected).max(0);
        let visible_pitched = visible.iter().filter(|event| event.midi.is_some() && !event.is_rest).count();
        if visible_pitched == 0 {
            audit.mixed_review_lanes += 1;
            continue;
        }
        let same_voice_other_staff_complete = by_lane.iter().any(|((other_scope, other_staff, other_voice), other_events)| {
            *other_scope == scope
                && other_voice == &voice
                && other_staff.trim() != staff_trimmed
                && (other_staff.trim() == "1" || other_staff.trim() == "2")
                && {
                    let other_visible = other_events
                        .iter()
                        .filter(|event| !event.is_hidden && !event.is_chord && event.duration > 0 && (event.is_rest || event.midi.is_some()))
                        .cloned()
                        .collect::<Vec<_>>();
                    if other_visible.iter().filter(|event| event.midi.is_some() && !event.is_rest).count() == 0 {
                        false
                    } else {
                        let other_intervals = other_visible.iter().map(|event| (event.onset, event.duration.max(0))).collect::<Vec<_>>();
                        let other_merged = strict_staff_merge_visible_intervals(other_intervals, expected);
                        let other_union_duration: i32 = other_merged.iter().map(|(_, duration)| *duration).sum::<i32>().min(expected).max(0);
                        other_union_duration == expected
                    }
                }
        });
        let other_staff_pitched_merged = strict_staff_merge_visible_intervals(
            by_lane
                .iter()
                .filter(|((other_scope, other_staff, other_voice), _)| {
                    *other_scope == scope
                        && other_voice == &voice
                        && other_staff.trim() != staff_trimmed
                        && (other_staff.trim() == "1" || other_staff.trim() == "2")
                })
                .flat_map(|(_, other_events)| other_events.iter())
                .filter(|event| {
                    !event.is_hidden
                        && !event.is_chord
                        && event.duration > 0
                        && !event.is_rest
                        && event.midi.is_some()
                })
                .map(|event| (event.onset, event.duration.max(0)))
                .collect::<Vec<_>>(),
            expected,
        );
        let same_voice_other_staff_gap_pitch_covered = !gaps.is_empty()
            && gaps.iter().all(|(gap_start, gap_end, _)| {
                other_staff_pitched_merged.iter().any(|(cover_start, cover_duration)| {
                    *cover_start <= *gap_start
                        && cover_start.saturating_add(*cover_duration) >= *gap_end
                })
            });
        let cross_staff_bookkeeping_fragment = same_voice_other_staff_gap_pitch_covered
            || (same_voice_other_staff_complete
                && union_duration < expected
                && visible_pitched > 0
                && visible.iter().filter(|event| event.midi.is_some() && !event.is_rest).map(|event| event.duration.max(0)).sum::<i32>() <= beat_unit.max(1));
        if cross_staff_bookkeeping_fragment {
            audit.cross_staff_bookkeeping_lanes += 1;
            audit.rows.push(VisibleLaneCompletenessRow {
                scope,
                staff,
                voice,
                duplicates: 0,
                crossbeat_blocked: 0,
                verdict: "CROSS_STAFF_BOOKKEEPING_FRAGMENT_OK".to_string(),
            });
            continue;
        }
        audit.independent_lanes += 1;
        let mut duplicate_map: BTreeMap<(i32, i32), usize> = BTreeMap::new();
        for event in visible.iter().filter(|event| event.is_rest) {
            *duplicate_map.entry((event.onset, event.duration)).or_default() += 1;
        }
        let duplicate_visible_rests: usize = duplicate_map.values().map(|count| count.saturating_sub(1)).sum();
        let mut visible_rests = visible.iter().filter(|event| event.is_rest).cloned().collect::<Vec<_>>();
        visible_rests.sort_by_key(|event| (event.onset, event.duration));
        let mut crossbeat_blocked = 0usize;
        for pair in visible_rests.windows(2) {
            let first = &pair[0];
            let second = &pair[1];
            if first.onset.saturating_add(first.duration) != second.onset { continue; }
            let start_beat = first.onset.div_euclid(beat_unit.max(1));
            let end_beat = second.onset.saturating_add(second.duration).saturating_sub(1).div_euclid(beat_unit.max(1));
            if start_beat != end_beat { crossbeat_blocked += 1; }
        }
        let missing_start = gaps.iter().filter(|(_, _, kind)| *kind == "start").count();
        let missing_internal = gaps.iter().filter(|(_, _, kind)| *kind == "internal").count();
        let missing_end = gaps.iter().filter(|(_, _, kind)| *kind == "end").count();
        audit.missing_start += missing_start;
        audit.missing_internal += missing_internal;
        audit.missing_end += missing_end;
        audit.forward_masking_rest += gaps.len();
        audit.duplicate_visible_rests += duplicate_visible_rests;
        audit.cross_beat_merge_blocked += crossbeat_blocked;
        let complete = gaps.is_empty() && duplicate_visible_rests == 0 && union_duration == expected;
        if complete { audit.complete_lanes += 1; } else { audit.incomplete_lanes += 1; }
        let verdict = if duplicate_visible_rests > 0 {
            "DUPLICATE_VISIBLE_REST"
        } else if !gaps.is_empty() {
            "STRICT_STAFF_VISIBLE_DURATION_INCOMPLETE"
        } else if crossbeat_blocked > 0 {
            "CROSS_BEAT_MERGE_BLOCKED_OK"
        } else {
            "STRICT_STAFF_VISIBLE_DURATION_OK"
        };
        audit.rows.push(VisibleLaneCompletenessRow {
            scope,
            staff,
            voice,
            duplicates: duplicate_visible_rests,
            crossbeat_blocked,
            verdict: verdict.to_string(),
        });
    }
    audit.rows.sort_by(|a, b| {
        let asev = if a.verdict == "STRICT_STAFF_VISIBLE_DURATION_OK" || a.verdict == "CROSS_BEAT_MERGE_BLOCKED_OK" { 0 } else { 1 };
        let bsev = if b.verdict == "STRICT_STAFF_VISIBLE_DURATION_OK" || b.verdict == "CROSS_BEAT_MERGE_BLOCKED_OK" { 0 } else { 1 };
        bsev.cmp(&asev).then_with(|| a.scope.cmp(&b.scope)).then_with(|| a.staff.cmp(&b.staff)).then_with(|| a.voice.cmp(&b.voice))
    });
    audit.verdict = if !audit.raw_time_ok {
        "strict_staff_raw_time_not_ok_untrusted".to_string()
    } else if audit.incomplete_lanes > 0 {
        "strict_staff_local_release_incomplete_no_release".to_string()
    } else if audit.duplicate_visible_rests > 0 {
        "strict_staff_duplicate_visible_rest_found_no_release".to_string()
    } else {
        "strict_staff_local_release_completeness_green".to_string()
    };
    audit.done_fragment = format!(
        "StrictStaffOverfullCoverageNormalizationReadback: scopes {}; lanes {}; independent {}; complete {}; incomplete {}; missing {}/{}/{}; duplicate_visible_rests {}; crossbeat_blocked {}; RAW_TIME_OK {}; CrossStaffGluingRelease nein",
        audit.measures_checked,
        audit.lanes_checked,
        audit.independent_lanes,
        audit.complete_lanes,
        audit.incomplete_lanes,
        audit.missing_start,
        audit.missing_internal,
        audit.missing_end,
        audit.duplicate_visible_rests,
        audit.cross_beat_merge_blocked,
        yes_no(audit.raw_time_ok)
    );
    audit.pipeline_line = format!(
        "StrictStaffOverfullCoverageNormalizationReadback v{}: Quelle {}; independent {}; complete {}; incomplete {}; missing_start/internal/end {}/{}/{}; duplicate_visible_rests {}; crossbeat_blocked {}; RAW_TIME_OK {}; CrossStaffGluingUsedAsReleaseProof 0; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.independent_lanes,
        audit.complete_lanes,
        audit.incomplete_lanes,
        audit.missing_start,
        audit.missing_internal,
        audit.missing_end,
        audit.duplicate_visible_rests,
        audit.cross_beat_merge_blocked,
        yes_no(audit.raw_time_ok),
        audit.verdict
    );
    audit
}



fn render_timeline_cursor_at_segment_end(segment: &str) -> i32 {
    let mut scan = 0_usize;
    let mut cursor = 0_i32;
    loop {
        let next_note = segment[scan..].find("<note").map(|p| scan + p);
        let next_backup = segment[scan..].find("<backup").map(|p| scan + p);
        let next_forward = segment[scan..].find("<forward").map(|p| scan + p);
        let next = [
            next_note.map(|pos| (pos, "note")),
            next_backup.map(|pos| (pos, "backup")),
            next_forward.map(|pos| (pos, "forward")),
        ]
        .into_iter()
        .flatten()
        .min_by_key(|(pos, _)| *pos);
        let Some((pos, kind)) = next else { break; };
        match kind {
            "note" => {
                let Some(close_rel) = segment[pos..].find("</note>") else { break; };
                let end = pos + close_rel + "</note>".len();
                let note = &segment[pos..end];
                let duration = extract_first_text_from_fragment(note, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                let is_chord = note.contains("<chord") || note.contains("<chord/>");
                if !is_chord {
                    cursor = cursor.saturating_add(duration);
                }
                scan = end;
            }
            "backup" => {
                let Some(close_rel) = segment[pos..].find("</backup>") else { break; };
                let end = pos + close_rel + "</backup>".len();
                let backup = &segment[pos..end];
                let duration = extract_first_text_from_fragment(backup, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                cursor = cursor.saturating_sub(duration).max(0);
                scan = end;
            }
            "forward" => {
                let Some(close_rel) = segment[pos..].find("</forward>") else { break; };
                let end = pos + close_rel + "</forward>".len();
                let forward = &segment[pos..end];
                let duration = extract_first_text_from_fragment(forward, "duration")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                cursor = cursor.saturating_add(duration);
                scan = end;
            }
            _ => break,
        }
    }
    cursor.max(0)
}

fn strict_staff_overlay_rest_layer_for_gaps(
    expected: i32,
    backup_duration: i32,
    gap_parts: &[(i32, i32)],
    staff: &str,
    voice: &str,
) -> Option<StrictStaffOverlayRestLayer> {
    if expected <= 0 || gap_parts.is_empty() { return None; }
    let mut parts = gap_parts
        .iter()
        .copied()
        .filter(|(_, duration)| *duration > 0)
        .collect::<Vec<_>>();
    if parts.is_empty() { return None; }
    parts.sort_by_key(|(start, duration)| (*start, *duration));
    let mut out = String::new();
    if backup_duration > 0 {
        out.push_str(&format!("\n      <backup><duration>{backup_duration}</duration></backup>"));
    }
    let mut cursor = 0_i32;
    let mut rest_count = 0_usize;
    let mut rest_total = 0_i32;
    let mut rest_parts = Vec::<(i32, i32)>::new();
    for (start, duration) in parts {
        let s = start.max(0).min(expected);
        let e = start.saturating_add(duration.max(0)).max(s).min(expected);
        if e <= s { continue; }
        if s < cursor { return None; }
        if s > cursor {
            push_render_xml_timeline_piece(
                &mut out,
                &render_staff_voice_forward_fragment(s.saturating_sub(cursor), staff, voice),
            );
        }
        let rest_dur = e.saturating_sub(s);
        let rest = render_visible_rest_note_fragment(rest_dur, staff, voice)?;
        push_render_xml_timeline_piece(&mut out, &rest);
        rest_parts.push((s, rest_dur));
        rest_total = rest_total.saturating_add(rest_dur);
        rest_count += 1;
        cursor = e;
    }
    if cursor < expected {
        push_render_xml_timeline_piece(
            &mut out,
            &render_staff_voice_forward_fragment(expected.saturating_sub(cursor), staff, voice),
        );
    }
    Some((out, rest_count, rest_total, rest_parts))
}


#[cfg(test)]
fn apply_strict_staff_local_release_completeness_materialization(
    xml: &str,
    max_actions: usize,
) -> (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize, Vec<String>) {
    let (out, actions, seen, trace, receipts) = apply_strict_staff_local_release_completeness_materialization_with_receipts(xml, max_actions);
    receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    (out, actions, seen, trace)
}




// Receipt-bearing mutation preparation belongs to the canonical structural owner.
include!("../structural_write_authority/strict_staff_local_release_core_receipt_ownership.rs");

#[cfg(test)]
#[path = "../../tests/unit/strict_staff_rest_context_refusal.rs"]
mod strict_staff_rest_context_refusal;
