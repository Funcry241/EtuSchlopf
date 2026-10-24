type PreparedInitialRestCoalescing = ((String, usize, usize, String, i32, usize, usize, String, i32, usize, String), crate::master_state_identity::RenderTransitionReceipts);

fn build_visible_lane_completeness_readback_gate_no_mutation(
    xml: &str,
    source_label: &str,
) -> VisibleLaneCompletenessClassifierPurgeGateAudit {
    let (events, raw_time_ok) = collect_render_xml_note_events(xml);
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let mut by_lane: BTreeMap<(usize, String, String), Vec<RenderXmlNoteEvent>> = BTreeMap::new();
    let mut by_scope_voice: BTreeMap<(usize, String), Vec<RenderXmlNoteEvent>> = BTreeMap::new();
    for event in events {
        by_scope_voice
            .entry((event.scope, event.voice.clone()))
            .or_default()
            .push(event.clone());
        by_lane
            .entry((event.scope, event.staff.clone(), event.voice.clone()))
            .or_default()
            .push(event);
    }
    let mut cross_staff_composite_complete: BTreeMap<(usize, String), (i32, usize)> = BTreeMap::new();
    for ((scope, voice), mut voice_events) in by_scope_voice {
        let (expected, _) = metric_by_scope.get(&scope).copied().unwrap_or((384, 96));
        let visible_events: Vec<RenderXmlNoteEvent> = voice_events
            .iter()
            .filter(|event| !event.is_hidden && !event.is_chord && event.duration > 0 && (event.is_rest || event.midi.is_some()))
            .cloned()
            .collect();
        let staffs: BTreeSet<String> = visible_events
            .iter()
            .map(|event| event.staff.trim().to_string())
            .filter(|staff| staff.as_str() == "1" || staff.as_str() == "2")
            .collect();
        if staffs.len() <= 1 || visible_events.is_empty() {
            continue;
        }
        voice_events = visible_events;
        voice_events.sort_by_key(|event| (event.onset, event.duration, event.staff.clone(), event.is_chord));
        let mut cursor = 0_i32;
        let mut gaps = Vec::<(i32, i32, &'static str)>::new();
        for event in &voice_events {
            let onset = event.onset.max(0);
            let end = onset.saturating_add(event.duration.max(0));
            if onset > cursor {
                gaps.push((cursor, onset, "cross_staff_gap"));
            }
            if end > cursor {
                cursor = end;
            }
        }
        if cursor < expected {
            gaps.push((cursor, expected, "cross_staff_gap"));
        }
        if gaps.is_empty() && cursor >= expected {
            cross_staff_composite_complete.insert((scope, voice), (cursor.min(expected).max(0), staffs.len()));
        }
    }
    let mut audit = VisibleLaneCompletenessClassifierPurgeGateAudit {
        source_label: source_label.to_string(),
        measures_checked: metric_by_scope.len(),
        raw_time_ok,
        ..VisibleLaneCompletenessClassifierPurgeGateAudit::default()
    };

    for ((scope_ref, staff_ref, voice_ref), lane_events_src) in &by_lane {
        let scope = *scope_ref;
        let staff = staff_ref.clone();
        let voice = voice_ref.clone();
        let mut lane_events = lane_events_src.clone();
        audit.lanes_checked += 1;
        lane_events.sort_by_key(|event| (event.onset, event.duration, event.is_chord));
        let visible_independent: Vec<RenderXmlNoteEvent> = lane_events
            .iter()
            .filter(|event| !event.is_hidden && !event.is_chord && event.duration > 0 && (event.is_rest || event.midi.is_some()))
            .cloned()
            .collect();
        let visible_pitched_non_chord = visible_independent.iter().filter(|event| event.midi.is_some() && !event.is_rest).count();
        let visible_rest_non_chord = visible_independent.iter().filter(|event| event.is_rest).count();
        let hidden_or_chord_only = visible_independent.is_empty();
        let (expected, beat_unit) = metric_by_scope.get(&scope).copied().unwrap_or((384, 96));
        if hidden_or_chord_only {
            audit.rows.push(VisibleLaneCompletenessRow {
                scope,
                staff,
                voice,
                duplicates: 0,
                crossbeat_blocked: 0,
                verdict: "AUXILIARY_LANE_NO_FULL_REST_REQUIRED".to_string(),
            });
            continue;
        }
        if visible_pitched_non_chord == 0 && visible_rest_non_chord > 0 {
            audit.mixed_review_lanes += 1;
            audit.rows.push(VisibleLaneCompletenessRow {
                scope,
                staff,
                voice,
                duplicates: 0,
                crossbeat_blocked: 0,
                verdict: "MIXED_VISIBLE_LANE_REVIEW_NO_REWRITE".to_string(),
            });
            continue;
        }
        let visible_duration: i32 = visible_independent.iter().map(|event| event.duration.max(0)).sum();
        // v0.10.339: HumanTrace-/StrictStaff-Release wird nicht mehr aus der naiven
        // Summe sichtbarer Dauerwerte berechnet. Zusätzliche sichtbare Rest-Layer
        // dürfen im Render-PDF als Notationsdelta existieren, solange sie keine
        // sichtbare Rest-Dublette bilden und die staff-lokale Union-Abdeckung die
        // volle Taktdauer schließt. Damit wird der bisherige Restfall
        // "visible_sum 480 bei coverage 384" als mehrschichtige Abdeckung und
        // nicht als Taktüberfüllung gewertet.
        let intervals = visible_independent
            .iter()
            .map(|event| (event.onset.max(0), event.duration.max(0)))
            .collect::<Vec<_>>();
        let merged = strict_staff_merge_visible_intervals(intervals, expected);
        let gaps = strict_staff_gaps_from_merged_intervals(&merged, expected);
        let union_duration = merged.iter().map(|(_, duration)| *duration).sum::<i32>().min(expected).max(0);
        let pitched_duration: i32 = visible_independent
            .iter()
            .filter(|event| event.midi.is_some() && !event.is_rest)
            .map(|event| event.duration.max(0))
            .sum();
        let same_voice_cross_staff_complete = cross_staff_composite_complete.contains_key(&(scope, voice.clone()));
        let other_staff_pitched_merged = strict_staff_merge_visible_intervals(
            by_lane
                .iter()
                .filter(|((other_scope, other_staff, other_voice), _)| {
                    *other_scope == scope
                        && other_voice == &voice
                        && other_staff.trim() != staff.trim()
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
            || (same_voice_cross_staff_complete
                && !gaps.is_empty()
                && union_duration < expected
                && visible_pitched_non_chord > 0
                && pitched_duration <= beat_unit.max(1));
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
        let overfull_sum_union_coverage_ok = visible_duration > expected && union_duration == expected && gaps.is_empty();
        let mut duplicate_map: BTreeMap<(i32, i32), usize> = BTreeMap::new();
        for event in visible_independent.iter().filter(|event| event.is_rest) {
            *duplicate_map.entry((event.onset, event.duration)).or_insert(0) += 1;
        }
        let duplicate_visible_rests: usize = duplicate_map.values().map(|count| count.saturating_sub(1)).sum();
        let mut visible_rests = visible_independent
            .iter()
            .filter(|event| event.is_rest)
            .cloned()
            .collect::<Vec<_>>();
        visible_rests.sort_by_key(|event| (event.onset, event.duration));
        let mut crossbeat_blocked = 0usize;
        for pair in visible_rests.windows(2) {
            let first = &pair[0];
            let second = &pair[1];
            if first.onset.saturating_add(first.duration) != second.onset { continue; }
            let combined_start = first.onset;
            let combined_end = second.onset.saturating_add(second.duration);
            let combined = combined_end.saturating_sub(combined_start);
            if combined <= 0 || duration_to_rest_type_and_dots(combined).is_none() { continue; }
            let start_beat = combined_start.div_euclid(beat_unit.max(1));
            let end_beat = combined_end.saturating_sub(1).div_euclid(beat_unit.max(1));
            if start_beat != end_beat {
                crossbeat_blocked += 1;
            }
        }
        let missing_start = gaps.iter().filter(|(_, _, kind)| *kind == "start").count();
        let missing_internal = gaps.iter().filter(|(_, _, kind)| *kind == "internal").count();
        let missing_end = gaps.iter().filter(|(_, _, kind)| *kind == "end").count();
        if !gaps.is_empty()
            && let Some((composite_union_duration, staff_count)) =
                cross_staff_composite_complete.get(&(scope, voice.clone())).copied()
        {
            let _cross_staff_explanation_only = (composite_union_duration, staff_count);
            // Cross-staff composite coverage that depends on rests remains diagnostic only.
            // Only exact same-voice pitched coverage of the complete staff-local gap may be
            // classified above as bookkeeping; that evidence suppresses false rest demand
            // and never authorizes a write of its own.
        }
        audit.missing_start += missing_start;
        audit.missing_internal += missing_internal;
        audit.missing_end += missing_end;
        audit.forward_masking_rest += gaps.len();
        audit.duplicate_visible_rests += duplicate_visible_rests;
        audit.cross_beat_merge_blocked += crossbeat_blocked;
        let complete = gaps.is_empty() && duplicate_visible_rests == 0 && union_duration == expected;
        if complete {
            audit.complete_lanes += 1;
        } else {
            audit.incomplete_lanes += 1;
        }
        let verdict = if duplicate_visible_rests > 0 {
            "DUPLICATE_VISIBLE_REST"
        } else if !gaps.is_empty() {
            "VISIBLE_DURATION_INCOMPLETE"
        } else if overfull_sum_union_coverage_ok {
            "OVERFULL_SUM_UNION_COVERAGE_OK"
        } else if crossbeat_blocked > 0 {
            "CROSS_BEAT_MERGE_BLOCKED_OK"
        } else {
            "VISIBLE_DURATION_OK"
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
        let asev = if matches!(a.verdict.as_str(), "VISIBLE_DURATION_OK" | "CROSS_BEAT_MERGE_BLOCKED_OK" | "OVERFULL_SUM_UNION_COVERAGE_OK" | "AUXILIARY_LANE_NO_FULL_REST_REQUIRED" | "MIXED_VISIBLE_LANE_REVIEW_NO_REWRITE" | "CROSS_STAFF_BOOKKEEPING_FRAGMENT_OK") { 0 } else { 1 };
        let bsev = if matches!(b.verdict.as_str(), "VISIBLE_DURATION_OK" | "CROSS_BEAT_MERGE_BLOCKED_OK" | "OVERFULL_SUM_UNION_COVERAGE_OK" | "AUXILIARY_LANE_NO_FULL_REST_REQUIRED" | "MIXED_VISIBLE_LANE_REVIEW_NO_REWRITE" | "CROSS_STAFF_BOOKKEEPING_FRAGMENT_OK") { 0 } else { 1 };
        bsev.cmp(&asev)
            .then_with(|| b.duplicates.cmp(&a.duplicates))
            .then_with(|| b.crossbeat_blocked.cmp(&a.crossbeat_blocked))
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
    });
    audit.verdict = if !audit.raw_time_ok {
        "raw_time_not_ok_visible_lane_audit_untrusted".to_string()
    } else if audit.duplicate_visible_rests > 0 {
        "duplicate_visible_rest_readback_found_no_mutation".to_string()
    } else if audit.incomplete_lanes > 0 {
        "visible_duration_incomplete_readback_found_no_mutation".to_string()
    } else {
        "visible_lane_completeness_readback_green_no_mutation".to_string()
    };
    audit.done_fragment = format!(
        "VisibleLaneCompletenessClassifierPurgeGate: scopes {}; lanes {}; independent {}; auxiliary {}; cross_staff_bookkeeping {}; chord_shadow {}; mixed_review {}; complete {}; incomplete {}; missing {}/{}/{}; duplicate_visible_rests {}; crossbeat_blocked {}; RAW_TIME_OK {}; WriteAllowed nein; MusicMutation nein",
        audit.measures_checked,
        audit.lanes_checked,
        audit.independent_lanes,
        audit.auxiliary_lanes,
        audit.cross_staff_bookkeeping_lanes,
        audit.chord_shadow_lanes,
        audit.mixed_review_lanes,
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
        "VisibleLaneCompletenessClassifierPurgeGateNoMutation v{}: finaler Readback aus {}; scopes {}; lanes {}; independent {}; auxiliary/bookkeeping {}; cross_staff_bookkeeping {}; chord_shadow {}; mixed_review {}; complete {}; incomplete {}; missing_start/internal/end {}/{}/{}; duplicate_visible_rests {}; crossbeat_merge_blocked {}; RAW_TIME_OK {}; WriteAllowed nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.measures_checked,
        audit.lanes_checked,
        audit.independent_lanes,
        audit.auxiliary_lanes,
        audit.cross_staff_bookkeeping_lanes,
        audit.chord_shadow_lanes,
        audit.mixed_review_lanes,
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

fn apply_structural_rest_beat_materialization(
    xml: &str,
    max_actions: usize,
) -> anyhow::Result<PreparedRestTieActions> {
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let spans = collect_visible_rest_spans_for_repair(xml);
    let mut by_beat_lane: BTreeMap<(usize, String, String, i32), Vec<RenderXmlRestFragmentSpan>> = BTreeMap::new();
    for span in spans.into_iter().filter(|span| {
        let staff = span.staff.trim();
        (staff == "1" || staff == "2") && span.duration > 0
    }) {
        let Some((_, beat)) = metric_by_scope.get(&span.scope).copied() else {
            continue;
        };
        let beat = beat.max(1);
        let beat_index = span.onset.div_euclid(beat);
        let beat_start = beat_index.saturating_mul(beat);
        let beat_end = beat_start.saturating_add(beat);
        if span.onset < beat_start || span.onset.saturating_add(span.duration) > beat_end {
            continue;
        }
        by_beat_lane
            .entry((span.scope, span.staff.clone(), span.voice.clone(), beat_index))
            .or_default()
            .push(span);
    }

    let mut chains: Vec<Vec<RenderXmlRestFragmentSpan>> = Vec::new();
    for ((_scope, _staff, _voice, _beat), mut group) in by_beat_lane {
        group.sort_by_key(|span| (span.onset, span.start));
        let mut chain: Vec<RenderXmlRestFragmentSpan> = Vec::new();
        for span in group {
            let contiguous = chain
                .last()
                .is_some_and(|last| last.onset.saturating_add(last.duration) == span.onset);
            if !chain.is_empty() && !contiguous {
                if chain.len() >= 2 { chains.push(chain.clone()); }
                chain.clear();
            }
            chain.push(span);
        }
        if chain.len() >= 2 { chains.push(chain); }
    }

    let candidates_seen = chains.len();
    chains.sort_by(|a, b| {
        let adur: i32 = a.iter().map(|s| s.duration.max(0)).sum();
        let bdur: i32 = b.iter().map(|s| s.duration.max(0)).sum();
        b.len()
            .cmp(&a.len())
            .then_with(|| bdur.cmp(&adur))
            .then_with(|| a.first().map_or(0, |s| s.scope).cmp(&b.first().map_or(0, |s| s.scope)))
            .then_with(|| a.first().map_or(0, |s| s.onset).cmp(&b.first().map_or(0, |s| s.onset)))
    });

    let mut selected_replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut duration_promotions = Vec::<crate::structural_write_authority::DurationPromotion>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut actions = Vec::new();

    for chain in chains {
        if actions.len() >= max_actions { break; }
        let Some(first) = chain.first() else { continue; };
        let total_duration: i32 = chain.iter().map(|span| span.duration.max(0)).sum();
        if total_duration <= 0 || duration_to_rest_type_and_dots(total_duration).is_none() {
            continue;
        }
        let Some((expected, beat)) = metric_by_scope.get(&first.scope).copied() else {
            continue;
        };
        let expected = expected.max(1);
        let beat = beat.max(1);
        if chain.iter().any(|span| occupied.iter().any(|(used_start, used_end)| span.start < *used_end && span.end > *used_start)) { continue; }
        let chord_anchor_dependency = chain.iter().any(|span| {
            find_measure_bounds_containing_pos(xml, span.start)
                .is_some_and(|(_, measure_end, _)| {
                    note_is_followed_by_chord_note(xml, span.end, measure_end)
                })
        });
        if crate::model::metric_rest_notation::decide_rest_span_consolidation(
            crate::model::metric_rest_notation::RestSpanConsolidationFacts {
                start: i64::from(first.onset),
                duration: i64::from(total_duration),
                expected_measure_duration: i64::from(expected),
                beat_duration: i64::from(beat),
                source_fragment_count: chain.len(),
                has_chord_anchor_dependency: chord_anchor_dependency,
                all_source_fragments_short: false,
            },
            crate::model::metric_rest_notation::RestSpanConsolidationProjection::SinglePartBeatLocal,
        )
        .is_none()
        {
            continue;
        }
        let (owner_post, owner_receipt) =
            set_rest_note_duration_type_with_receipt(&first.fragment, total_duration);
        let Some(owner_post) = owner_post else { continue; };
        let mut first_replacement = replace_xml_tag_text(&owner_post, "staff", &first.staff);
        first_replacement = replace_xml_tag_text(&first_replacement, "voice", &first.voice);
        if let Some(receipt) = owner_receipt {
            duration_promotions.push(
                crate::structural_write_authority::DurationPromotion::new(
                    first.start,
                    first.end,
                    owner_post,
                    receipt,
                ),
            );
        }
        selected_replacements.push((first.start, first.end, first_replacement));
        for span in chain.iter().skip(1) {
            selected_replacements.push((span.start, span.end, String::new()));
        }
        for span in &chain { occupied.push((span.start, span.end)); }
        actions.push(BehindBarsRuleCheckRestAndTieAction {
            action_kind: "external_feedback_rest_beat_materialization".to_string(),
            old_rest_count: chain.len(),
            new_rest_count: 1,
            verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
        });
    }

    selected_replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    if selected_replacements.is_empty() {
        return Ok(((
            xml.to_string(),
            actions,
            candidates_seen,
        ), crate::master_state_identity::RenderTransitionReceipts::new()));
    }
    let (draft, rebased_replacements, mut receipts) =
        crate::structural_write_authority::mutate_selected_duration_fragments(
            xml,
            duration_promotions,
            selected_replacements,
        )?;
    let (out, consumer_receipt) =
        crate::structural_write_authority::apply_rest_beat_materialization_authorized(
            &draft,
            rebased_replacements,
        );
    let Some(out) = out else {
        receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::LosingCandidate,
        );
        return Ok(((
            xml.to_string(),
            actions,
            candidates_seen,
        ), crate::master_state_identity::RenderTransitionReceipts::new()));
    };
    receipts.push_exact(consumer_receipt);
    Ok(((out, actions, candidates_seen), receipts))
}

fn choose_free_aux_rest_voice_for_staff(fragments: &[RenderXmlTimelineFragmentSpan], staff: &str) -> Option<String> {
    for candidate in ["2", "3", "4", "5"] {
        let used = fragments.iter().any(|frag| {
            frag.staff.trim() == staff
                && frag.voice.trim() == candidate
                && (frag.is_visible_pitched || frag.is_visible_rest || (frag.is_rest && frag.is_hidden))
        });
        if !used {
            return Some(candidate.to_string());
        }
    }
    None
}

fn unhide_hidden_rest_note_fragment(fragment: &str) -> String {
    fragment
        .replace(" print-object=\"no\"", "")
        .replace(" print-spacing=\"no\"", "")
        .replace("print-object=\"no\" ", "")
        .replace("print-spacing=\"no\" ", "")
}

fn apply_beat_local_backup_anchor_rewrite(
    xml: &str,
    max_actions: usize,
) -> (
    (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut actions = Vec::<BehindBarsRuleCheckRestAndTieAction>::new();
    let mut candidates_seen = 0_usize;

    for observation in collect_beat_local_voice_rest_measure_observations(xml) {
        if actions.len() >= max_actions {
            break;
        }
        if !observation.has_cross_staff_visible_voice {
            continue;
        }
        for backup in observation.backups {
            let residual_shift = backup.cursor_before.saturating_sub(backup.duration);
            let metric_plausible = backup.cursor_before > backup.duration
                && backup.duration >= observation.expected
                && backup.cursor_before <= observation.expected.saturating_mul(4)
                && backup.cursor_before % 24 == 0
                && residual_shift % 24 == 0;
            if metric_plausible && backup.rest_lane_after {
                candidates_seen += 1;
                if actions.len() < max_actions {
                    let replacement = replace_xml_tag_text(
                        &backup.fragment,
                        "duration",
                        &backup.cursor_before.to_string(),
                    );
                    replacements.push((backup.start, backup.end, replacement));
                    actions.push(BehindBarsRuleCheckRestAndTieAction {
                        action_kind: "beat_local_voice_rest_rewrite_backup_anchor".to_string(),
                        old_rest_count: 1,
                        new_rest_count: 1,
                        verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
                    });
                }
            }
        }
    }

    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (out, receipt) = crate::structural_write_authority::apply_backup_anchor_rewrite_authorized(
        xml,
        replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    ((out, actions, candidates_seen), receipts)
}

fn apply_beat_local_missing_staff_edge_rest_rewrite(
    xml: &str,
    max_actions: usize,
) -> (
    (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let mut insertions = Vec::<(usize, String)>::new();
    let mut actions = Vec::<BehindBarsRuleCheckRestAndTieAction>::new();
    let mut candidates_seen = 0_usize;

    for observation in collect_beat_local_voice_rest_measure_observations(xml) {
        if actions.len() >= max_actions {
            break;
        }
        if observation.beat <= 0 || !observation.has_cross_staff_visible_voice {
            continue;
        }
        let fragments = &observation.fragments;
        let mut voices = BTreeSet::<String>::new();
        for frag in fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && !frag.is_chord && !frag.is_hidden)
        {
            voices.insert(frag.voice.clone());
        }
        let mut measure_action_written = false;
        for source_voice in voices {
            if measure_action_written || actions.len() >= max_actions {
                break;
            }
            let staffs_in_voice = fragments
                .iter()
                .filter(|frag| {
                    frag.voice == source_voice
                        && frag.is_visible_pitched
                        && !frag.is_chord
                        && !frag.is_hidden
                })
                .map(|frag| frag.staff.trim().to_string())
                .collect::<BTreeSet<_>>();
            if !(staffs_in_voice.contains("1") && staffs_in_voice.contains("2")) {
                continue;
            }
            for target_staff in ["1", "2"] {
                if measure_action_written || actions.len() >= max_actions {
                    break;
                }
                let other_staff = if target_staff == "1" { "2" } else { "1" };
                let mut target_notes = fragments
                    .iter()
                    .filter(|frag| {
                        frag.voice == source_voice
                            && frag.staff.trim() == target_staff
                            && frag.is_visible_pitched
                            && !frag.is_chord
                            && !frag.is_hidden
                            && frag.onset >= 0
                            && frag.onset < observation.expected
                    })
                    .collect::<Vec<_>>();
                if target_notes.is_empty() {
                    continue;
                }
                target_notes.sort_by_key(|frag| (frag.onset, frag.start));
                for beat_index in 0..4_i32 {
                    let beat_start = beat_index.saturating_mul(observation.beat);
                    let beat_end = beat_start
                        .saturating_add(observation.beat)
                        .min(observation.expected);
                    let Some(first_target) = target_notes
                        .iter()
                        .find(|frag| frag.onset > beat_start && frag.onset < beat_end)
                    else {
                        continue;
                    };
                    let gap_duration = first_target.onset.saturating_sub(beat_start);
                    if gap_duration <= 0 || gap_duration > 48 {
                        continue;
                    }
                    let metric_plan =
                        crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                            i64::from(beat_start),
                            i64::from(gap_duration),
                            i64::from(observation.expected),
                            i64::from(observation.beat),
                        );
                    if metric_plan.len() != 1
                        || metric_plan.first().copied() != Some(i64::from(gap_duration))
                        || duration_to_rest_type_and_dots(gap_duration).is_none()
                    {
                        continue;
                    }
                    let other_staff_anchor = fragments.iter().any(|frag| {
                        frag.voice == source_voice
                            && frag.staff.trim() == other_staff
                            && frag.is_visible_pitched
                            && !frag.is_chord
                            && !frag.is_hidden
                            && frag.onset <= beat_start
                            && frag.onset.saturating_add(frag.duration.max(0)) > beat_start
                    });
                    if !other_staff_anchor {
                        continue;
                    }
                    let already_visible_rest = fragments.iter().any(|frag| {
                        frag.staff.trim() == target_staff
                            && frag.is_visible_rest
                            && !frag.is_hidden
                            && frag.onset <= beat_start
                            && frag.onset.saturating_add(frag.duration.max(0))
                                >= beat_start.saturating_add(gap_duration)
                    });
                    if already_visible_rest {
                        continue;
                    }
                    let Some(rest_voice) =
                        choose_free_aux_rest_voice_for_staff(fragments, target_staff)
                    else {
                        continue;
                    };
                    if observation.final_cursor <= 0 {
                        continue;
                    }
                    let Some(rest_note) =
                        missing_rest_visible_template(target_staff, &rest_voice, gap_duration)
                    else {
                        continue;
                    };
                    let mut insertion = String::new();
                    insertion.push_str(&format!(
                        "\n      <backup><duration>{}</duration></backup>",
                        observation.final_cursor
                    ));
                    if beat_start > 0 {
                        insertion.push_str(&render_forward_fragment(beat_start));
                    }
                    insertion.push('\n');
                    insertion.push_str(&rest_note);
                    let tail = observation
                        .expected
                        .saturating_sub(beat_start)
                        .saturating_sub(gap_duration);
                    if tail > 0 {
                        insertion.push_str(&render_forward_fragment(tail));
                    }
                    let insert_pos = observation
                        .measure_end
                        .saturating_sub("</measure>".len());
                    candidates_seen += 1;
                    insertions.push((insert_pos, insertion));
                    actions.push(BehindBarsRuleCheckRestAndTieAction {
                        action_kind:
                            "beat_local_voice_rest_rewrite_missing_staff_edge_rest".to_string(),
                        old_rest_count: 0,
                        new_rest_count: 1,
                        verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
                    });
                    measure_action_written = true;
                    break;
                }
            }
        }
    }

    insertions.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (out, receipt) = crate::structural_write_authority::insert_missing_staff_edge_rest_authorized(
        xml,
        insertions,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    ((out, actions, candidates_seen), receipts)
}

fn apply_beat_local_hidden_rest_visibility_rewrite(
    xml: &str,
    max_actions: usize,
) -> (
    (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut actions = Vec::<BehindBarsRuleCheckRestAndTieAction>::new();
    let mut candidates_seen = 0_usize;

    for observation in collect_beat_local_voice_rest_measure_observations(xml) {
        if actions.len() >= max_actions {
            break;
        }
        if observation.beat <= 0 || !observation.has_cross_staff_visible_voice {
            continue;
        }
        let fragments = &observation.fragments;
        let visible_staffs = fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && !frag.is_chord && !frag.is_hidden)
            .map(|frag| frag.staff.trim().to_string())
            .collect::<BTreeSet<_>>();
        for frag in fragments
            .iter()
            .filter(|frag| frag.is_rest && frag.is_hidden && !frag.is_chord && frag.duration > 0)
        {
            if actions.len() >= max_actions {
                break;
            }
            let staff = frag.staff.trim();
            if !(staff == "1" || staff == "2") || !visible_staffs.contains(staff) {
                continue;
            }
            if frag.has_visible_dependency
                || duration_to_rest_type_and_dots(frag.duration).is_none()
            {
                continue;
            }
            let beat_index = frag.onset.div_euclid(observation.beat);
            let beat_end = beat_index
                .saturating_add(1)
                .saturating_mul(observation.beat)
                .min(observation.expected);
            let end_onset = frag.onset.saturating_add(frag.duration.max(0));
            let closes_beat_or_measure =
                end_onset == beat_end || end_onset == observation.expected;
            if !closes_beat_or_measure || frag.duration < 48 {
                continue;
            }
            let duplicate_visible_rest = fragments.iter().any(|other| {
                other.is_visible_rest
                    && !other.is_hidden
                    && other.staff.trim() == staff
                    && other.onset == frag.onset
                    && other.duration == frag.duration
            });
            if duplicate_visible_rest {
                continue;
            }
            candidates_seen += 1;
            replacements.push((
                frag.start,
                frag.end,
                unhide_hidden_rest_note_fragment(&frag.fragment),
            ));
            actions.push(BehindBarsRuleCheckRestAndTieAction {
                action_kind: "beat_local_voice_rest_rewrite_hidden_rest_visibility".to_string(),
                old_rest_count: 0,
                new_rest_count: 1,
                verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
            });
        }
    }

    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (out, receipt) = crate::structural_write_authority::apply_hidden_rest_visibility_rewrite_authorized(
        xml,
        replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    ((out, actions, candidates_seen), receipts)
}

fn apply_beat_local_voice_rest_rewrite(
    xml: &str,
    max_actions: usize,
) -> (
    (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let mut out = xml.to_string();
    let mut actions = Vec::<BehindBarsRuleCheckRestAndTieAction>::new();
    let mut seen_total = 0_usize;
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();

    let remaining = max_actions.saturating_sub(actions.len());
    let ((next_xml, mut backup_actions, backup_seen), backup_receipts) =
        apply_beat_local_backup_anchor_rewrite(&out, remaining);
    seen_total = seen_total.saturating_add(backup_seen);
    out = next_xml;
    actions.append(&mut backup_actions);
    receipts.append(backup_receipts);

    let remaining = max_actions.saturating_sub(actions.len());
    let ((next_xml, mut edge_actions, edge_seen), edge_receipts) =
        apply_beat_local_missing_staff_edge_rest_rewrite(&out, remaining);
    seen_total = seen_total.saturating_add(edge_seen);
    out = next_xml;
    actions.append(&mut edge_actions);
    receipts.append(edge_receipts);

    let remaining = max_actions.saturating_sub(actions.len());
    let ((next_xml, mut hidden_actions, hidden_seen), hidden_receipts) =
        apply_beat_local_hidden_rest_visibility_rewrite(&out, remaining);
    seen_total = seen_total.saturating_add(hidden_seen);
    out = next_xml;
    actions.append(&mut hidden_actions);
    receipts.append(hidden_receipts);

    ((out, actions, seen_total), receipts)
}

fn apply_duplicate_stem_same_pitch_render_suppression(
    xml: &str,
    max_actions: usize,
) -> (
    (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let plan = plan_strict_staff_duplicate_same_pitch_suppression(
        xml,
        max_actions,
        StrictStaffSamePitchGrouping::StaffAcrossVoices,
        "duplicate_stem_same_pitch_render_suppression",
    );
    let (out, receipt) = crate::structural_write_authority::apply_duplicate_same_pitch_suppression_authorized(
        xml,
        plan.replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    ((out, plan.actions, plan.candidates_seen), receipts)
}

fn build_behind_bars_rule_check_duplicate_stem_rows(
    xml: &str,
) -> Vec<BehindBarsRuleCheckDuplicateStemRow> {
    // v0.10.374: External PDF feedback exposed that the older observer missed same-voice
    // same-pitch duplicates when both noteheads lived in the same staff/voice/onset slot.
    // Therefore this readback is based on the final render timeline fragments, not only on
    // non-chord multi-voice stems. Takt numbers remain derived result data only.
    let mut by_note: BTreeMap<
        RenderNoteIdentityKey,
        Vec<RenderXmlTimelineFragmentSpan>,
    > = BTreeMap::new();
    for measure in collect_strict_staff_visible_pitch_observations(xml) {
        for observation in measure.notes {
            let span = observation.span;
            by_note
                .entry((
                    measure.scope,
                    span.staff.clone(),
                    span.voice.clone(),
                    span.onset,
                    span.duration,
                    observation.midi,
                ))
                .or_default()
                .push(span);
        }
    }

    let mut rows = Vec::new();
    for ((scope, staff, _voice, onset, _duration, midi), mut group) in by_note {
        if group.len() < 2 {
            continue;
        }
        group.sort_by_key(|span| (if span.is_chord { 1 } else { 0 }, span.start));
        rows.push(BehindBarsRuleCheckDuplicateStemRow {
            scope,
            staff,
            midi,
            onset,
        });
    }
    rows.sort_by_key(|row| (row.scope, row.staff.clone(), row.onset, row.midi));
    rows
}











fn set_pitched_note_duration_type_add_staccato(fragment: &str, duration: i32) -> Option<String> {
    let _retired_writer_keepalive =
        crate::structural_write_authority::set_pitched_note_duration_type_add_staccato_authorized;
    let _ = (fragment, duration);
    None
}



fn apply_behind_bars_rule_check_spurious_empty_voice_rest_suppression(
    xml: &str,
    max_actions: usize,
) -> (
    (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let mut candidates_seen = 0_usize;
    let mut selected_replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut actions = Vec::new();
    let mut occupied = Vec::<(usize, usize)>::new();

    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        if actions.len() >= max_actions {
            break;
        }
        let measure = &xml[measure_start..measure_end];
        let expected = infer_measure_duration_from_measure_xml(measure).unwrap_or(384).max(1);
        let fragments = collect_render_timeline_fragments_in_segment(measure, measure_start, scope_number);
        let mut visible_pitched_by_staff_voice: BTreeMap<(String, String), usize> = BTreeMap::new();
        let mut visible_pitched_by_staff: BTreeMap<String, usize> = BTreeMap::new();
        let mut visible_staffs_by_voice: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for frag in &fragments {
            if frag.is_visible_pitched {
                *visible_pitched_by_staff_voice.entry((frag.staff.clone(), frag.voice.clone())).or_insert(0) += 1;
                *visible_pitched_by_staff.entry(frag.staff.clone()).or_insert(0) += 1;
                visible_staffs_by_voice.entry(frag.voice.clone()).or_default().insert(frag.staff.clone());
            }
        }
        let mut rest_by_staff_voice: BTreeMap<(String, String), Vec<&RenderXmlTimelineFragmentSpan>> = BTreeMap::new();
        for frag in fragments.iter().filter(|frag| frag.is_visible_rest && !frag.is_chord) {
            rest_by_staff_voice.entry((frag.staff.clone(), frag.voice.clone())).or_default().push(frag);
        }

        // CrossStaffCoveredRestLaneSuppression v0.10.1458:
        // Suppress only duration-identical technical rest lanes in a genuine
        // cross-staff measure: (a) same-voice rests fully covered by pitches on
        // the other staff, or (b) a full-measure rest-only auxiliary lane on a
        // staff that already carries another visible voice.  The selector uses
        // only staff/voice/onset/duration coverage, never project or measure IDs.
        let measure_has_cross_staff_visible_voice = visible_staffs_by_voice.values().any(|staffs| {
            staffs.iter().any(|staff| staff.trim() == "1")
                && staffs.iter().any(|staff| staff.trim() == "2")
        });
        if measure_has_cross_staff_visible_voice {
            for ((staff, voice), rests) in &rest_by_staff_voice {
                if actions.len() >= max_actions {
                    break;
                }
                let mut selected_rests = rests
                    .iter()
                    .copied()
                    .filter(|rest| {
                        let start = rest.onset.max(0);
                        let end = rest.onset.saturating_add(rest.duration.max(0)).min(expected);
                        let duration = end.saturating_sub(start);
                        if duration <= 0 {
                            return false;
                        }
                        let coverage = fragments
                            .iter()
                            .filter(|frag| {
                                frag.is_visible_pitched
                                    && !frag.is_chord
                                    && !frag.is_hidden
                                    && frag.voice == *voice
                                    && frag.staff.trim() != staff.trim()
                            })
                            .map(|frag| {
                                end.min(frag.onset.saturating_add(frag.duration.max(0)))
                                    .saturating_sub(start.max(frag.onset))
                                    .max(0)
                            })
                            .sum::<i32>()
                            .min(duration);
                        coverage >= duration
                    })
                    .collect::<Vec<_>>();
                let mut action_kind = "cross_staff_same_voice_covered_rest_lane_to_forward";

                if selected_rests.is_empty()
                    && visible_pitched_by_staff_voice
                        .get(&(staff.to_string(), voice.to_string()))
                        .copied()
                        .unwrap_or(0)
                        == 0
                    && fragments.iter().any(|frag| {
                        frag.is_visible_pitched
                            && !frag.is_chord
                            && !frag.is_hidden
                            && frag.staff.trim() == staff.trim()
                            && frag.voice != *voice
                    })
                {
                    let mut ordered = rests.to_vec();
                    ordered.sort_by_key(|rest| (rest.onset, rest.duration, rest.start));
                    let mut covered_until = 0_i32;
                    for rest in &ordered {
                        let start = rest.onset.max(0);
                        let end = rest.onset.saturating_add(rest.duration.max(0)).min(expected);
                        if start > covered_until {
                            break;
                        }
                        covered_until = covered_until.max(end);
                    }
                    if covered_until >= expected {
                        selected_rests = ordered;
                        action_kind = "cross_staff_measure_full_rest_only_aux_lane_to_forward";
                    }
                }
                if selected_rests.is_empty()
                    || selected_rests.iter().any(|rest| {
                        occupied.iter().any(|(used_start, used_end)| {
                            rest.start < *used_end && rest.end > *used_start
                        })
                    })
                {
                    continue;
                }

                candidates_seen = candidates_seen.saturating_add(1);
                for rest in &selected_rests {
                    occupied.push((rest.start, rest.end));
                    selected_replacements.push((rest.start, rest.end, render_forward_fragment(rest.duration)));
                }
                actions.push(BehindBarsRuleCheckRestAndTieAction {
                    action_kind: action_kind.to_string(),
                    old_rest_count: selected_rests.len(),
                    new_rest_count: 0,
                    verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
                });
            }
        }

        for ((staff, voice), rests) in &rest_by_staff_voice {
            if actions.len() >= max_actions {
                break;
            }
            let staff_trimmed = staff.trim();
            if !(staff_trimmed == "1" || staff_trimmed == "2") || rests.len() != 1 {
                continue;
            }
            let rest = rests[0];
            if rest.onset != 0 || rest.duration != expected {
                continue;
            }
            if visible_pitched_by_staff_voice.get(&(staff.to_string(), voice.to_string())).copied().unwrap_or(0) > 0 {
                continue;
            }
            let same_staff_other_voice_visible = visible_pitched_by_staff.get(staff).copied().unwrap_or(0) > 0;
            let cross_staff_same_voice_visible = visible_staffs_by_voice
                .get(voice)
                .is_some_and(|staffs| staffs.iter().any(|visible_staff| visible_staff.trim() != staff.trim()));
            if !same_staff_other_voice_visible && !cross_staff_same_voice_visible {
                continue;
            }
            candidates_seen += 1;
            if occupied.iter().any(|(used_start, used_end)| rest.start < *used_end && rest.end > *used_start) {
                continue;
            }
            occupied.push((rest.start, rest.end));
            selected_replacements.push((rest.start, rest.end, render_forward_fragment(rest.duration)));
            let action_kind = if cross_staff_same_voice_visible {
                if staff_trimmed == "1" { "upper_cross_staff_empty_rest_lane_to_forward" } else { "cross_staff_empty_rest_lane_to_forward" }
            } else if staff_trimmed == "1" {
                "upper_staff_spurious_empty_aux_voice_rest_to_forward"
            } else {
                "spurious_empty_aux_voice_rest_to_forward"
            };
            actions.push(BehindBarsRuleCheckRestAndTieAction {
                action_kind: action_kind.to_string(),
                old_rest_count: 1,
                new_rest_count: 0,
                verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
            });
        }

        // UpperStaffRestLaneVisibleProof v0.10.263:
        // Die bisherige EmptyRestLane-Regel erwischte nur einzelne volltaktige
        // Reststimmen. Historische PDF-Evidenz zeigte aber eine obere
        // Zusatzstimme mit sichtbaren Füllpausen innerhalb einer laufenden
        // 8va-/Cross-Staff-Linie. Diese generische Erweiterung ersetzt solche
        // oberen Füllpausen nur durch Forward-Zeit, wenn die betroffene
        // Staff/Voice-Lane selbst keine klingenden Noten trägt und der Staff
        // oder dieselbe Voice bereits sichtbar klingend geführt ist. Es werden
        // keine Tonhöhen, Dauern, Onsets, Staff-/Voice-Zuordnungen klingender
        // Events oder Master-XML-Inhalte geändert.
        for ((staff, voice), mut rests) in rest_by_staff_voice {
            if actions.len() >= max_actions {
                break;
            }
            let staff_trimmed = staff.trim();
            if staff_trimmed != "1" || rests.is_empty() {
                continue;
            }
            rests.sort_by_key(|frag| (frag.onset, frag.duration, frag.start));
            let same_lane_visible_pitched = visible_pitched_by_staff_voice.get(&(staff.clone(), voice.clone())).copied().unwrap_or(0);
            let same_staff_other_voice_visible = visible_pitched_by_staff
                .get(&staff)
                .copied()
                .unwrap_or(0)
                .saturating_sub(same_lane_visible_pitched)
                > 0;
            let cross_staff_same_voice_visible = visible_staffs_by_voice
                .get(&voice)
                .is_some_and(|staffs| staffs.iter().any(|visible_staff| visible_staff.trim() != staff.trim()));
            let measure_has_octave_shift = measure.contains("<octave-shift");
            let dense_upper_same_voice_rest_lane = same_lane_visible_pitched >= 4
                && measure_has_octave_shift
                && rests.len() >= 2
                && rests.iter().all(|frag| frag.duration > 0 && frag.duration <= expected / 4);
            if !same_staff_other_voice_visible && !cross_staff_same_voice_visible && !dense_upper_same_voice_rest_lane {
                continue;
            }
            let total_duration: i32 = rests.iter().map(|frag| frag.duration.max(0)).sum();
            if total_duration <= 0 || total_duration > expected {
                continue;
            }
            // Der alte Volltakt-Einzelfall wurde oben bereits beschrieben;
            // hier geht es ausdrücklich um sichtbare Restspuren/Teilfragmente
            // in der oberen Zusatzstimme.
            if rests.len() == 1 && rests[0].onset == 0 && rests[0].duration == expected {
                continue;
            }
            candidates_seen += 1;
            let mut local_replacements = Vec::<(usize, usize, String)>::new();
            let mut overlap = false;
            for rest in &rests {
                if occupied.iter().any(|(used_start, used_end)| rest.start < *used_end && rest.end > *used_start) {
                    overlap = true;
                    break;
                }
                local_replacements.push((rest.start, rest.end, render_forward_fragment(rest.duration)));
            }
            if overlap {
                continue;
            }
            for rest in &rests {
                occupied.push((rest.start, rest.end));
            }
            selected_replacements.extend(local_replacements);
            actions.push(BehindBarsRuleCheckRestAndTieAction {
                action_kind: if dense_upper_same_voice_rest_lane {
                    "upper_staff_octave_shift_same_voice_rest_lane_visible_proof_to_forward".to_string()
                } else {
                    "upper_staff_rest_lane_visible_proof_to_forward".to_string()
                },
                old_rest_count: rests.len(),
                new_rest_count: 0,
                verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
            });
        }
        scan = measure_end;
    }

    selected_replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (out, receipt) = crate::structural_write_authority::apply_empty_voice_rest_suppression_authorized(
        xml,
        selected_replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    ((out, actions, candidates_seen), receipts)
}








fn build_cross_staff_hand_role_materialization_trial(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<CrossStaffHandRoleMaterializationTrial> {
    // Preserve the frozen sink signature in this retired/no-write boundary.
    // This function item is never invoked.
    let _retired_sink_signature = crate::structural_write_authority::write_cross_staff_hand_role_trial_authorized;

    let _ = output_path;
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut trial = CrossStaffHandRoleMaterializationTrial {
        source_label: source_label.to_string(),
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        parse_before_ok: parse_musicxml(xml).is_ok(),
        backup_forward_before_ok: before_backup_forward_ok,
        ..CrossStaffHandRoleMaterializationTrial::default()
    };

    let candidate_xml = xml.to_string();
    if trial.parse_before_ok && trial.backup_forward_before_ok {
        trial.attempted = true;
        trial.candidates_seen = 0;
        trial.candidate_attempted = 0;
    }

    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.visible_signature_after = render_visible_signature(&after_events, false);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    trial.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.visible_rest_delta = trial.visible_rests_after as i64 - trial.visible_rests_before as i64;

    let invariants_ok = trial.candidate_attempted > 0
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.hidden_pitched_before == trial.hidden_pitched_after
        && trial.hidden_rests_before == trial.hidden_rests_after
        && trial.visible_rests_after <= trial.visible_rests_before
        && trial.parse_after_ok
        && trial.backup_forward_after_ok;

    if invariants_ok {
        // IR7 is retired/no-write. The current producer is never called above;
        // an unexpected future productive candidate must stop before any sink.
        crate::master_state_identity::count_render_closure_failure(
            crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures,
        );
        return Err(anyhow::anyhow!(
            "IR7 CrossStaffHandRole is NO-WRITE: unexpected productive candidate"
        ));
    } else {
        trial.candidate_rejected = trial.candidate_attempted;
        trial.verdict = if trial.candidate_attempted == 0 {
            "cross_staff_hand_role_no_safe_empty_rest_lane_candidate".to_string()
        } else {
            "cross_staff_hand_role_trial_rejected_by_invariant_guard".to_string()
        };
        for action in &mut trial.actions {
            action.verdict = "rejected_by_global_invariant_guard".to_string();
        }
    }

    trial.examples.push("Handrollen-Materialisierung heißt hier nicht: neue Handstimme schreiben. Es heißt: eine leere technische Rest-Lane nicht als linke-Hand-Ganzpause anzeigen, wenn die Cross-Staff-Voice bereits vollständig gemappt ist.".to_string());
    trial.examples.push("Die Aktion ist negativ und eng: sichtbare Rest-Lane -> Forward. Keine klingende Note, kein Pitch, kein Onset, keine Dauer und keine klingende Staff-/Voice-Zuordnung ändern sich.".to_string());
    trial.done_fragment = format!(
        "CrossStaffHandRoleMaterializationTrial: gesehen {}, versucht {}, übernommen {}, sichtbare Pausen {}→{}, klingend stabil {}, geschrieben {}",
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
        trial.visible_rests_before,
        trial.visible_rests_after,
        if trial.sounding_signature_stable { "ja" } else { "nein" },
        if trial.written { "ja" } else { "nein" }
    );
    trial.pipeline_line = format!(
        "CrossStaffHandRoleMaterializationTrial v{}: Render-XML-only auf {}; seen/attempted/adopted/rejected {}/{}/{}/{}; sichtbare Pausen {}→{}; sichtbare klingende Noten {}→{}; hidden klingend {}→{}; hidden Pausen {}→{}; sounding_signature {}/{} stabil {}; visible_rest_delta {}; parse/back-forward nachher {}/{}; geschrieben {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
        trial.candidate_rejected,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.hidden_pitched_before,
        trial.hidden_pitched_after,
        trial.hidden_rests_before,
        trial.hidden_rests_after,
        trial.sounding_signature_before,
        trial.sounding_signature_after,
        if trial.sounding_signature_stable { "ja" } else { "nein" },
        trial.visible_rest_delta,
        if trial.parse_after_ok { "ok" } else { "fail" },
        if trial.backup_forward_after_ok { "ok" } else { "fail" },
        if trial.written { "ja" } else { "nein" },
        trial.verdict
    );
    Ok(trial)
}


fn apply_lower_staff_visible_rest_lane_anchor_suppression(
    xml: &str,
    max_actions: usize,
) -> (
    (String, usize, usize, i32, String, String),
    crate::master_state_identity::RenderTransitionReceipts,
) {
    let candidates = lower_staff_visible_rest_lane_anchor_suppression_candidates(xml);
    let candidates_seen = candidates.len();
    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut per_scope_selected = BTreeMap::<usize, usize>::new();
    let mut scopes = BTreeSet::<usize>::new();
    let mut total_duration = 0_i32;
    let mut selected = 0_usize;
    let mut evidence_rows = Vec::new();
    for (_score, start, end, replacement, scope, evidence, duration) in candidates {
        if selected >= max_actions {
            break;
        }
        if *per_scope_selected.get(&scope).unwrap_or(&0) >= 6 {
            continue;
        }
        if occupied.iter().any(|(used_start, used_end)| start < *used_end && end > *used_start) {
            continue;
        }
        occupied.push((start, end));
        replacements.push((start, end, replacement));
        *per_scope_selected.entry(scope).or_default() += 1;
        scopes.insert(scope);
        total_duration = total_duration.saturating_add(duration.max(0));
        evidence_rows.push(evidence);
        selected += 1;
    }
    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (out, receipt) = crate::structural_write_authority::apply_lower_visible_rest_suppression_authorized(
        xml,
        replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    let scope_text = if scopes.is_empty() {
        "-".to_string()
    } else {
        scopes.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(",")
    };
    let chosen_evidence = evidence_rows.join(" | ");
    ((out, candidates_seen, selected, total_duration, scope_text, chosen_evidence), receipts)
}

fn apply_lower_staff_overlapping_visible_rest_suppression(
    xml: &str,
    max_actions: usize,
) -> (String, usize, usize, i32, String, String) {
    let candidates = lower_staff_overlapping_rest_suppression_candidates(xml);
    let candidates_seen = candidates.len();
    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut per_scope_selected = BTreeMap::<usize, usize>::new();
    let mut scopes = BTreeSet::<usize>::new();
    let mut total_duration = 0_i32;
    let mut selected = 0_usize;
    let mut evidence_rows = Vec::new();
    for (_score, start, end, replacement, scope, evidence, duration) in candidates {
        if selected >= max_actions {
            break;
        }
        if *per_scope_selected.get(&scope).unwrap_or(&0) >= 3 {
            continue;
        }
        if occupied.iter().any(|(used_start, used_end)| start < *used_end && end > *used_start) {
            continue;
        }
        occupied.push((start, end));
        replacements.push((start, end, replacement));
        *per_scope_selected.entry(scope).or_default() += 1;
        scopes.insert(scope);
        total_duration = total_duration.saturating_add(duration.max(0));
        evidence_rows.push(evidence);
        selected += 1;
    }
    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let _observed_unproven_replacement_count = replacements.len();
    let out = crate::structural_write_authority::apply_lower_visible_rest_suppression_authorized(
        xml,
        Vec::new(),
    ).0
    .unwrap_or_else(|| xml.to_string());
    let scope_text = if scopes.is_empty() {
        "-".to_string()
    } else {
        scopes.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(",")
    };
    let _observed_unproven_duration = total_duration;
    drop(out);
    (
        xml.to_string(),
        candidates_seen,
        0,
        0,
        scope_text,
        evidence_rows.join("; "),
    )
}



type UpperStaffInitialRestCandidate = (usize, usize, String, i32, usize);


fn select_measure_local_upper_initial_rest_candidates(
    candidates: Vec<UpperStaffInitialRestCandidate>,
) -> Vec<UpperStaffInitialRestCandidate> {
    let mut candidates_by_scope = BTreeMap::<usize, Vec<UpperStaffInitialRestCandidate>>::new();
    for candidate in candidates {
        candidates_by_scope.entry(candidate.4).or_default().push(candidate);
    }

    let mut selected = Vec::new();
    for (_scope, mut candidate_group) in candidates_by_scope {
        if candidate_group.len() != 1 {
            continue;
        }
        if let Some(candidate) = candidate_group.pop() {
            selected.push(candidate);
        }
    }
    selected
}

fn apply_upper_staff_initial_rest_and_beat_local_coalescing(
    xml: &str,
    max_coalesces: usize,
) -> anyhow::Result<PreparedInitialRestCoalescing> {
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut duration_promotions = Vec::<crate::structural_write_authority::DurationPromotion>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut initial_candidates: Vec<(usize, usize, String, i32, usize)> = Vec::new();
    let mut coalesce_candidates: Vec<UpperStaffBeatLocalCoalesceCandidate> = Vec::new();
    let mut cross_beat_blocked = 0_usize;
    let mut cross_beat_scopes = BTreeSet::<usize>::new();
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);

    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let fragments = collect_render_timeline_fragments_in_segment(measure, measure_start, scope_number);
        let has_octave_context = measure.contains("octave-shift") || measure.contains("<clef number=\"2\"><sign>G</sign>");

        let mut pitched_by_onset: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && !frag.is_chord && !frag.is_hidden && frag.duration > 0)
            .cloned()
            .collect();
        pitched_by_onset.sort_by_key(|frag| (frag.voice.clone(), frag.onset, frag.start));
        let staff_switches = pitched_by_onset
            .windows(2)
            .filter(|pair| pair[0].voice == pair[1].voice && pair[0].staff != pair[1].staff)
            .count();

        let upper_first = fragments
            .iter()
            .filter(|frag| {
                frag.staff.trim() == "1"
                    && frag.is_visible_pitched
                    && !frag.is_chord
                    && !frag.is_hidden
                    && frag.duration > 0
            })
            .min_by_key(|frag| (frag.onset, frag.start));

        if let Some(first_upper) = upper_first {
            let first_onset = first_upper.onset;
            let has_upper_start_rest = fragments.iter().any(|frag| {
                frag.staff.trim() == "1"
                    && frag.is_visible_rest
                    && !frag.is_chord
                    && !frag.is_hidden
                    && frag.onset <= 0
                    && frag.onset.saturating_add(frag.duration.max(0)) >= first_onset
            });
            let lower_or_crossstaff_before_upper = fragments.iter().any(|frag| {
                frag.staff.trim() == "2"
                    && frag.is_visible_pitched
                    && !frag.is_hidden
                    && frag.onset < first_onset
                    && frag.onset.saturating_add(frag.duration.max(0)) > 0
            });
            let initial_metric_single_rest = metric_by_scope
                .get(&scope_number)
                .is_some_and(|(expected, beat_unit)| {
                    let metric_plan =
                        crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                            0,
                            i64::from(first_onset),
                            i64::from(*expected),
                            i64::from(*beat_unit),
                        );
                    metric_plan.len() == 1
                        && metric_plan.first().copied() == Some(i64::from(first_onset))
                });
            let upper_begins_after_cross_staff_material = first_onset > 0
                && first_onset <= 96
                && !has_upper_start_rest
                && initial_metric_single_rest
                && (lower_or_crossstaff_before_upper || staff_switches > 0 || has_octave_context);
            if upper_begins_after_cross_staff_material
                && let Some(rest_fragment) =
                    render_visible_rest_note_fragment(first_onset, "1", &first_upper.voice)
            {
                let replacement = format!(
                    "\n      <backup><duration>{first_onset}</duration></backup>{rest_fragment}"
                );
                initial_candidates.push((
                    first_upper.start,
                    first_upper.start,
                    replacement,
                    first_onset,
                    scope_number,
                ));
            }
        }

        let mut upper_rests: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| {
                frag.staff.trim() == "1"
                    && frag.is_visible_rest
                    && !frag.is_chord
                    && !frag.is_hidden
                    && frag.duration > 0
            })
            .cloned()
            .collect();
        upper_rests.sort_by_key(|frag| (frag.voice.clone(), frag.onset, frag.start));
        for pair in upper_rests.windows(2) {
            let first = &pair[0];
            let second = &pair[1];
            if first.voice != second.voice {
                continue;
            }
            let contiguous = first.onset.saturating_add(first.duration.max(0)) == second.onset;
            if !contiguous {
                continue;
            }
            let total = first.duration.saturating_add(second.duration);
            let both_sixteenth = first.duration == 24 && second.duration == 24;
            let inside_single_beat = duration_span_is_inside_single_beat(first.onset, total);
            let pair_end = second.onset.saturating_add(second.duration.max(0));
            let voice_span_has_sounding = fragments.iter().any(|frag| {
                frag.voice == first.voice
                    && frag.is_note
                    && !frag.is_rest
                    && frag.duration > 0
                    && midi_from_note_fragment(&frag.fragment).is_some()
                    && frag.onset >= first.onset
                    && frag.onset < pair_end
            });
            let initial_pre_entry_pair = first.onset == 0
                && !voice_span_has_sounding
                && upper_first.is_some_and(|first_upper| {
                    first_upper.voice == first.voice && pair_end <= first_upper.onset
                });
            let metric_single_rest = metric_by_scope
                .get(&scope_number)
                .is_some_and(|(expected, beat_unit)| {
                    let metric_plan =
                        crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                            i64::from(first.onset),
                            i64::from(total),
                            i64::from(*expected),
                            i64::from(*beat_unit),
                        );
                    metric_plan.len() == 1
                        && metric_plan.first().copied() == Some(i64::from(total))
                });
            let initial_metric_pair = initial_pre_entry_pair && metric_single_rest;
            let same_beat_micro_pair = both_sixteenth && inside_single_beat && metric_single_rest;
            if same_beat_micro_pair || initial_metric_pair {
                if let Some(new_first) = preview_upper_rest_duration(&first.fragment, total) {
                    let score = 900_i64
                        + if initial_metric_pair { 180 } else { 0 }
                        + if has_octave_context { 80 } else { 0 }
                        + (staff_switches as i64 * 25)
                        + if first.onset % 96 == 0 { 20 } else { 0 };
                    coalesce_candidates.push((
                        score,
                        first.start,
                        first.end,
                        second.start,
                        second.end,
                        new_first,
                        total,
                        scope_number,
                    ));
                }
            } else if !inside_single_beat {
                cross_beat_blocked += 1;
                cross_beat_scopes.insert(scope_number);
            }
        }
        scan = measure_end;
    }

    let initial_seen = initial_candidates.len();
    let initial_candidates = select_measure_local_upper_initial_rest_candidates(initial_candidates);
    let mut initial_written = 0_usize;
    let mut initial_duration = 0_i32;
    let mut initial_scopes = BTreeSet::<usize>::new();
    for (start, end, replacement, duration, scope) in initial_candidates.into_iter() {
        if occupied.iter().any(|(used_start, used_end)| start < *used_end && end > *used_start) {
            continue;
        }
        replacements.push((start, end, replacement));
        occupied.push((start, end));
        initial_written += 1;
        initial_duration = initial_duration.saturating_add(duration);
        initial_scopes.insert(scope);
    }

    let coalesce_seen = coalesce_candidates.len();
    coalesce_candidates.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let mut coalesce_written = 0_usize;
    let mut coalesce_duration = 0_i32;
    let mut coalesce_scopes = BTreeSet::<usize>::new();
    for (
        _score,
        first_start,
        first_end,
        second_start,
        second_end,
        new_first,
        total,
        scope,
    ) in coalesce_candidates.into_iter() {
        if coalesce_written >= max_coalesces {
            break;
        }
        if occupied.iter().any(|(used_start, used_end)| first_start < *used_end && *used_start < second_end) {
            continue;
        }
        let Some(first_fragment) = xml.get(first_start..first_end) else {
            continue;
        };
        let (owner_post, owner_receipt) =
            set_rest_note_duration_type_with_receipt(first_fragment, total);
        let Some(owner_post) = owner_post else {
            continue;
        };
        assert_eq!(
            owner_post, new_first,
            "IR8 selected #17 rematerialization must preserve legacy coalesce bytes"
        );
        if let Some(receipt) = owner_receipt {
            duration_promotions.push(
                crate::structural_write_authority::DurationPromotion::new(
                    first_start,
                    first_end,
                    owner_post,
                    receipt,
                ),
            );
        }
        replacements.push((first_start, first_end, new_first));
        replacements.push((second_start, second_end, String::new()));
        occupied.push((first_start, second_end));
        coalesce_written += 1;
        coalesce_duration = coalesce_duration.saturating_add(total);
        coalesce_scopes.insert(scope);
    }

    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    let (out, receipts) = if replacements.is_empty() {
        (
            xml.to_string(),
            crate::master_state_identity::RenderTransitionReceipts::new(),
        )
    } else {
        let (draft, rebased_replacements, mut receipts) =
            crate::structural_write_authority::mutate_selected_duration_fragments(
                xml,
                duration_promotions,
                replacements,
            )?;
        let (candidate, consumer_receipt) =
            crate::structural_write_authority::apply_upper_initial_rest_coalescing_authorized(
                &draft,
                rebased_replacements,
            );
        let Some(candidate) = candidate else {
            receipts.discard_unadopted(
                crate::master_state_identity::RenderDiscardReason::LosingCandidate,
            );
            return Ok(((
                xml.to_string(),
                initial_seen,
                initial_written,
                fmt_upper_staff_rest_scope_set(initial_scopes),
                initial_duration,
                coalesce_seen,
                coalesce_written,
                fmt_upper_staff_rest_scope_set(coalesce_scopes),
                coalesce_duration,
                cross_beat_blocked,
                fmt_upper_staff_rest_scope_set(cross_beat_scopes),
            ), crate::master_state_identity::RenderTransitionReceipts::new()));
        };
        receipts.push_exact(consumer_receipt);
        (candidate, receipts)
    };

    let fmt_scopes = |set: BTreeSet<usize>| -> String {
        if set.is_empty() {
            "-".to_string()
        } else {
            set.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
        }
    };

    Ok(((
        out,
        initial_seen,
        initial_written,
        fmt_scopes(initial_scopes),
        initial_duration,
        coalesce_seen,
        coalesce_written,
        fmt_scopes(coalesce_scopes),
        coalesce_duration,
        cross_beat_blocked,
        fmt_scopes(cross_beat_scopes),
    ), receipts))
}

#[cfg(test)]
mod upper_initial_metric_rest_coalescing_safety_tests {
    use super::*;

    fn discard_probe(receipts: crate::master_state_identity::RenderTransitionReceipts) {
        receipts.discard_unadopted(
            crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe,
        );
    }

    fn quarter_rest() -> &'static str {
        "<note><rest/><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>"
    }

    fn rest(duration: i32, note_type: &str) -> String {
        format!(
            "<note><rest/><duration>{duration}</duration><voice>1</voice><type>{note_type}</type><staff>1</staff></note>"
        )
    }

    fn pitched(step: &str, octave: i32, duration: i32, chord: bool, hidden: bool) -> String {
        let chord_tag = if chord { "<chord/>" } else { "" };
        let hidden_attr = if hidden { " print-object=\"no\" print-spacing=\"no\"" } else { "" };
        let note_type = match duration {
            24 => "16th",
            48 => "eighth",
            96 => "quarter",
            192 => "half",
            _ => "quarter",
        };
        format!(
            "<note{hidden_attr}>{chord_tag}<pitch><step>{step}</step><octave>{octave}</octave></pitch><duration>{duration}</duration><voice>1</voice><type>{note_type}</type><staff>1</staff></note>"
        )
    }

    fn score(body: &str) -> String {
        score_with_divisions(body, 96)
    }

    fn score_with_divisions(body: &str, divisions: i32) -> String {
        format!(
            "<score-partwise><part id=\"P1\"><measure number=\"1\"><attributes><divisions>{divisions}</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{body}</measure></part></score-partwise>"
        )
    }

    fn octave_context_forward_then_pitch(divisions: i32, onset: i32) -> String {
        score_with_divisions(
            &format!(
                "<direction><direction-type><octave-shift type=\"up\" size=\"8\"/></direction-type></direction><forward><duration>{onset}</duration></forward>{}",
                pitched("C", 5, divisions.saturating_mul(4).saturating_sub(onset), false, false),
            ),
            divisions,
        )
    }

    fn two_measure_octave_context_initial_rest_score(
        first_onset: i32,
        second_onset: i32,
    ) -> String {
        let measure = |number: usize, onset: i32| {
            format!(
                "<measure number=\"{number}\"><attributes><divisions>96</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes><direction><direction-type><octave-shift type=\"up\" size=\"8\"/></direction-type></direction><forward><duration>{onset}</duration></forward>{}</measure>",
                pitched("C", 5, 384_i32.saturating_sub(onset), false, false),
            )
        };
        format!(
            "<score-partwise><part id=\"P1\">{}{}</part></score-partwise>",
            measure(1, first_onset),
            measure(2, second_onset),
        )
    }

    #[test]
    fn beat_local_micro_pair_is_still_owned_by_metric_rest_notation() {
        let xml = score(&format!(
            "{}{}{}{}",
            pitched("C", 5, 24, false, false),
            rest(24, "16th"),
            rest(24, "16th"),
            pitched("D", 5, 312, false, false),
        ));
        let ((out, _, _, _, _, seen, written, _, duration, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&xml, 8).expect("prepare");
        assert_eq!(receipts.len(), 2, "actual #17 Fragment plus fresh #48 EXACT");
        discard_probe(receipts);
        assert_eq!(seen, 1);
        assert_eq!(written, 1);
        assert_eq!(duration, 48);
        assert_eq!(out.matches("<rest/>").count(), 1);
        assert_eq!(out.matches("<type>eighth</type>").count(), 1);
    }

    #[test]
    fn structural_beat_materialization_fails_closed_outside_canonical_metric_basis() {
        let xml = score_with_divisions(
            &format!("{}{}", rest(12, "16th"), rest(12, "16th")),
            48,
        );
        let ((out, actions, seen), receipts) = apply_structural_rest_beat_materialization(&xml, 8).expect("prepare");
        assert_eq!(receipts.len(), 0);
        discard_probe(receipts);
        assert_eq!(seen, 1);
        assert!(actions.is_empty());
        assert_eq!(out, xml);
    }

    #[test]
    fn inserted_initial_rest_uses_metric_authority_on_canonical_basis() {
        let xml = octave_context_forward_then_pitch(96, 48);
        let ((singleton_out, seen, written, _, duration, _, _, _, _, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&xml, 0).expect("prepare");
        assert_eq!(receipts.len(), 1, "insertion-only candidate is owned only by #48");
        discard_probe(receipts);
        assert_eq!(seen, 1);
        assert_eq!(written, 1);
        assert_eq!(duration, 48);

        // Candidate-duplication regression stays inside this already-existing writer caller.
        // Measure 2 deliberately carries the retired first_onset==24 preference.
        let duplicated = two_measure_octave_context_initial_rest_score(48, 24);
        let ((
            duplicated_out,
            duplicated_seen,
            duplicated_written,
            duplicated_scopes,
            duplicated_duration,
            _,
            _,
            _,
            _,
            _,
            _,
        ), receipts) = apply_upper_staff_initial_rest_and_beat_local_coalescing(&duplicated, 0).expect("prepare");
        discard_probe(receipts);
        assert_eq!(duplicated_seen, 2);
        assert_eq!(duplicated_written, 2);
        assert_eq!(duplicated_scopes, "1,2");
        assert_eq!(duplicated_duration, 72);
        assert_eq!(duplicated_out.matches("<rest/>").count(), 2);

        let (singleton_start, singleton_end, _) =
            find_next_measure_bounds(&singleton_out, 0).expect("singleton measure");
        let (duplicated_start, duplicated_end, _) =
            find_next_measure_bounds(&duplicated_out, 0).expect("duplicated first measure");
        assert_eq!(
            &singleton_out[singleton_start..singleton_end],
            &duplicated_out[duplicated_start..duplicated_end],
        );
    }

    #[test]
    fn inserted_initial_rest_fails_closed_outside_canonical_metric_basis() {
        let xml = octave_context_forward_then_pitch(48, 48);
        let ((out, seen, written, _, duration, _, _, _, _, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&xml, 0).expect("prepare");
        discard_probe(receipts);
        assert_eq!(seen, 0);
        assert_eq!(written, 0);
        assert_eq!(duration, 0);
        assert_eq!(out, xml);
    }

    #[test]
    fn same_measure_initial_rest_candidate_ambiguity_fails_closed() {
        let candidates = vec![
            (10, 10, "first".to_string(), 24, 1),
            (20, 20, "second".to_string(), 48, 1),
        ];
        let selected = select_measure_local_upper_initial_rest_candidates(candidates);
        assert!(selected.is_empty());
    }

    #[test]
    fn true_initial_voice_silence_can_coalesce_to_half_rest() {
        let xml = score(&format!(
            "{}{}{}",
            quarter_rest(),
            quarter_rest(),
            pitched("C", 5, 192, false, false),
        ));
        let ((out, _, _, _, _, seen, written, _, duration, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&xml, 8).expect("prepare");
        assert_eq!(receipts.len(), 2, "actual #17 Fragment plus fresh #48 EXACT");
        discard_probe(receipts);
        assert_eq!(seen, 1);
        assert_eq!(written, 1);
        assert_eq!(duration, 192);
        assert_eq!(out.matches("<type>half</type>").count(), 2);
        assert_eq!(out.matches("<rest/>").count(), 1);
    }

    #[test]
    fn chord_anchored_sounding_events_block_initial_rest_coalescing() {
        let xml = score(&format!(
            "{}{}{}{}{}",
            quarter_rest(),
            pitched("D", 4, 48, true, true),
            quarter_rest(),
            pitched("F", 6, 24, true, true),
            pitched("C", 6, 192, false, false),
        ));
        let ((out, _, _, _, _, seen, written, _, duration, _, _), receipts) =
            apply_upper_staff_initial_rest_and_beat_local_coalescing(&xml, 8).expect("prepare");
        discard_probe(receipts);
        assert_eq!(seen, 0);
        assert_eq!(written, 0);
        assert_eq!(duration, 0);
        assert_eq!(out, xml);

        let transactional_chord_xml = score(&format!(
            "{}{}{}{}",
            rest(24, "16th"),
            rest(24, "16th"),
            pitched("F", 4, 24, true, true),
            pitched("C", 6, 336, false, false),
        ));
        let (transactional_chord_out, transactional_chord_actions, transactional_chord_seen) =
            apply_structural_transactional_rest_coalesce_render_write(&transactional_chord_xml, 4);
        assert_eq!(transactional_chord_seen, 1);
        assert_eq!(transactional_chord_actions.len(), 1);
        assert_eq!(
            transactional_chord_actions[0].verdict,
            "rejected_chord_anchor_dependency"
        );
        assert_eq!(transactional_chord_out, transactional_chord_xml);
        let (pianistic_chord_out, pianistic_chord_actions, pianistic_chord_seen) =
            apply_pianistic_rest_consolidation(&transactional_chord_xml, 4);
        assert_eq!(pianistic_chord_seen, 1);
        assert!(pianistic_chord_actions.is_empty());
        assert_eq!(pianistic_chord_out, transactional_chord_xml);
        let lower_chord_xml = transactional_chord_xml.replace(
            "<staff>1</staff>",
            "<staff>2</staff>",
        );
        let ((behind_bars_chord_out, behind_bars_chord_actions, behind_bars_chord_seen), receipts) =
            apply_behind_bars_rule_check_lower_staff_rest_compaction(&lower_chord_xml, 4).expect("prepare");
        discard_probe(receipts);
        assert_eq!(behind_bars_chord_seen, 1);
        assert!(behind_bars_chord_actions.is_empty());
        assert_eq!(behind_bars_chord_out, lower_chord_xml);
        let ((beat_chord_out, beat_chord_actions, beat_chord_seen), receipts) =
            apply_structural_rest_beat_materialization(&lower_chord_xml, 4).expect("prepare");
        discard_probe(receipts);
        assert_eq!(beat_chord_seen, 1);
        assert!(beat_chord_actions.is_empty());
        assert_eq!(beat_chord_out, lower_chord_xml);

        let true_rest_xml = score(&format!(
            "{}{}{}",
            rest(24, "16th"),
            rest(24, "16th"),
            pitched("C", 6, 336, false, false),
        ));
        let (true_rest_out, true_rest_actions, true_rest_seen) =
            apply_structural_transactional_rest_coalesce_render_write(&true_rest_xml, 4);
        assert_eq!(true_rest_seen, 1);
        assert_eq!(true_rest_actions.len(), 1);
        assert_eq!(true_rest_actions[0].verdict, "adopted_transactionally");
        assert_ne!(true_rest_out, true_rest_xml);
        assert_eq!(true_rest_out.matches("<rest/>").count(), 1);
        assert!(true_rest_out.contains("<duration>48</duration>"));
        let (pianistic_true_rest_out, pianistic_true_rest_actions, pianistic_true_rest_seen) =
            apply_pianistic_rest_consolidation(&true_rest_xml, 4);
        assert_eq!(pianistic_true_rest_seen, 1);
        assert_eq!(pianistic_true_rest_actions.len(), 1);
        assert_ne!(pianistic_true_rest_out, true_rest_xml);
        assert!(pianistic_true_rest_out.contains("<duration>48</duration>"));
        let lower_true_rest_xml = true_rest_xml.replace(
            "<staff>1</staff>",
            "<staff>2</staff>",
        );
        let ((behind_bars_true_rest_out, behind_bars_true_rest_actions, behind_bars_true_rest_seen), receipts) =
            apply_behind_bars_rule_check_lower_staff_rest_compaction(&lower_true_rest_xml, 4).expect("prepare");
        assert_eq!(receipts.len(), 2, "actual #17 Fragment plus fresh #33 EXACT");
        discard_probe(receipts);
        assert_eq!(behind_bars_true_rest_seen, 1);
        assert_eq!(behind_bars_true_rest_actions.len(), 1);
        assert_ne!(behind_bars_true_rest_out, lower_true_rest_xml);
        assert!(behind_bars_true_rest_out.contains("<duration>48</duration>"));
        let ((beat_true_rest_out, beat_true_rest_actions, beat_true_rest_seen), receipts) =
            apply_structural_rest_beat_materialization(&lower_true_rest_xml, 4).expect("prepare");
        assert_eq!(receipts.len(), 2, "actual #17 Fragment plus fresh #46 EXACT");
        discard_probe(receipts);
        assert_eq!(beat_true_rest_seen, 1);
        assert_eq!(beat_true_rest_actions.len(), 1);
        assert_ne!(beat_true_rest_out, lower_true_rest_xml);
        assert!(beat_true_rest_out.contains("<duration>48</duration>"));
    }
}


include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/structural_write_authority/retired_rest_lane_no_write_tests.rs"));


// Receipt-bearing mutation preparation belongs to the canonical structural owner.
include!("../structural_write_authority/strict_staff_visible_lane_rest_rewrite_tail_receipt_ownership.rs");
