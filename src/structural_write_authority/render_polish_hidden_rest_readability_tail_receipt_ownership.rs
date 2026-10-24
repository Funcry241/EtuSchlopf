// Canonical structural mutation ownership.
// Included in the pipeline lexical namespace to retain private selection facts.
// The complete preparation and mutation implementations live here; no forwarding facade.

fn set_rest_note_duration_type_with_receipt(
    fragment: &str,
    duration: i32,
) -> (Option<String>, Option<BlockTransformationReceipt>) {
    crate::structural_write_authority::set_rest_note_duration_type_authorized(fragment, duration)
}

fn apply_pianistic_rest_consolidation_with_receipts(
    xml: &str,
    max_actions: usize,
) -> (
    String,
    Vec<RenderXmlPianisticRestRepairAction>,
    usize,
    Vec<GeneratedRestDurationReceipt>,
    Option<crate::master_state_identity::ExactTransformationReceipt>,
) {
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let candidates = build_rest_consolidation_candidates(xml);
    let mut selected: Vec<RenderXmlRestConsolidationCandidate> = Vec::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    for candidate in &candidates {
        if selected.len() >= max_actions {
            break;
        }
        let overlaps = candidate.spans.iter().any(|span| {
            occupied.iter().any(|(start, end)| span.start < *end && span.end > *start)
        });
        if overlaps {
            continue;
        }
        let chord_anchor_dependency = candidate.spans.iter().any(|span| {
            find_measure_bounds_containing_pos(xml, span.start)
                .is_some_and(|(_, measure_end, _)| {
                    note_is_followed_by_chord_note(xml, span.end, measure_end)
                })
        });
        let Some((expected, beat_unit)) = metric_by_scope.get(&candidate.scope).copied() else {
            continue;
        };
        let all_short = candidate.spans.iter().all(|span| span.duration <= 96);
        if crate::model::metric_rest_notation::decide_rest_span_consolidation(
            crate::model::metric_rest_notation::RestSpanConsolidationFacts {
                start: i64::from(candidate.first_onset),
                duration: i64::from(candidate.total_duration),
                expected_measure_duration: i64::from(expected),
                beat_duration: i64::from(beat_unit),
                source_fragment_count: candidate.spans.len(),
                has_chord_anchor_dependency: chord_anchor_dependency,
                all_source_fragments_short: all_short,
            },
            crate::model::metric_rest_notation::RestSpanConsolidationProjection::SinglePartShortBeatLocal,
        )
        .is_none()
        {
            continue;
        }
        for span in &candidate.spans {
            occupied.push((span.start, span.end));
        }
        selected.push(candidate.clone());
    }
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut actions = Vec::new();
    let mut generated_receipts = Vec::new();
    for candidate in &selected {
        let Some(first) = candidate.spans.first() else { continue; };
        let (new_first, duration_receipt) =
            set_rest_note_duration_type_with_receipt(&first.fragment, candidate.total_duration);
        let Some(new_first) = new_first else { continue; };
        if let Some(receipt) = duration_receipt {
            generated_receipts.push(GeneratedRestDurationReceipt {
                receipt,
                pre_fragment: first.fragment.clone(),
                post_fragment: new_first.clone(),
                located: first.start,
            });
        }
        replacements.push((first.start, first.end, new_first));
        for span in candidate.spans.iter().skip(1) {
            replacements.push((span.start, span.end, String::new()));
        }
        actions.push(RenderXmlPianisticRestRepairAction {
            scope: candidate.scope,
            staff: candidate.staff.clone(),
            voice: candidate.voice.clone(),
            verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
        });
    }
    let (output, consumer_receipt) =
        crate::structural_write_authority::apply_pianistic_rest_consolidation_authorized(
            xml,
            replacements,
        );
    let out = output.unwrap_or_else(|| xml.to_string());
    (out, actions, candidates.len(), generated_receipts, consumer_receipt)
}

fn apply_pianistic_rest_consolidation_prepared(
    xml: &str,
    max_actions: usize,
) -> (String, Vec<RenderXmlPianisticRestRepairAction>, usize, crate::master_state_identity::RenderTransitionReceipts) {
    let (out, actions, seen, generated, consumer) =
        apply_pianistic_rest_consolidation_with_receipts(xml, max_actions);
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    for generated in generated {
        receipts.push_generated_block(Some(generated.receipt), &generated.pre_fragment,
            &generated.post_fragment, Some(generated.located));
    }
    receipts.push_exact(consumer);
    (out, actions, seen, receipts)
}

fn apply_single_rest_consolidation_candidate_prepared(
    xml: &str,
    candidate: &RenderXmlRestConsolidationCandidate,
) -> (Option<String>, crate::master_state_identity::RenderTransitionReceipts) {
    let Some(first) = candidate.spans.first() else { return (None, crate::master_state_identity::RenderTransitionReceipts::new()); };
    let (new_first, receipt) = set_rest_note_duration_type_with_receipt(&first.fragment, candidate.total_duration);
    let Some(new_first) = new_first else { return (None, crate::master_state_identity::RenderTransitionReceipts::new()); };
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_generated_block(receipt, &first.fragment, &new_first, Some(first.start));
    let mut replacements = vec![(first.start, first.end, new_first)];
    for span in candidate.spans.iter().skip(1) { replacements.push((span.start, span.end, String::new())); }
    let (out, consumer) = crate::structural_write_authority::apply_single_rest_consolidation_authorized(xml, replacements);
    receipts.push_exact(consumer);
    if out.is_none() {
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        return (None, crate::master_state_identity::RenderTransitionReceipts::new());
    }
    (out, receipts)
}

fn apply_layer_balanced_single_rest_consolidation_candidate_xml(
    xml: &str,
    candidate: &RenderXmlRestConsolidationCandidate,
    carrier_projection: Option<(&[i32], usize)>,
) -> anyhow::Result<(Option<String>, crate::master_state_identity::RenderTransitionReceipts)> {
    use crate::master_state_identity::{RenderDiscardReason, RenderTransitionReceipts};
    use crate::structural_write_authority::DurationPromotion;
    // All fallible selection/locator work precedes minting. Decisions stay on A.
    let selected = (|| -> Option<Vec<(usize, usize, String)>> {
        if let Some((parts, insertion_pos)) = carrier_projection {
            let mut overlay = format!("\n      <backup><duration>{}</duration></backup>", candidate.total_duration);
            for duration in parts {
                overlay.push_str(&render_visible_rest_note_fragment(*duration, &candidate.staff, &candidate.voice)?);
            }
            if candidate.spans.iter().any(|span| !span.fragment.contains("<rest") || span.fragment.contains("<pitch>")) { return None; }
            // Placeholder payloads are filled by the actual #20 below.
            let mut edits = candidate.spans.iter().map(|span| (span.start, span.end, String::new())).collect::<Vec<_>>();
            edits.push((insertion_pos, insertion_pos, overlay));
            return Some(edits);
        }
        let anchor = candidate.spans.first()?;
        let extension = candidate.total_duration.saturating_sub(anchor.duration);
        if extension <= 0 { return None; }
        duration_to_rest_type_and_dots(candidate.total_duration)?;
        let (measure_start, measure_end, scope_number) = find_measure_bounds_containing_pos(xml, anchor.start)?;
        let fragments = collect_render_timeline_fragments_in_segment(&xml[measure_start..measure_end], measure_start, scope_number);
        let forward = fragments.iter().filter(|frag| frag.is_forward && frag.start >= anchor.end && frag.onset == anchor.onset.saturating_add(anchor.duration.max(0)) && frag.duration >= extension).min_by_key(|frag| frag.start)?;
        let mut edits = vec![(anchor.start, anchor.end, String::new()), (forward.start, forward.end, replace_forward_fragment_duration(&forward.fragment, forward.duration.saturating_sub(extension)))];
        for span in candidate.spans.iter().skip(1) {
            if span.start != anchor.start || span.end != anchor.end {
                edits.push((span.start, span.end, render_staff_voice_forward_fragment(span.duration, &span.staff, &span.voice)));
            }
        }
        Some(edits)
    })();
    let Some(mut edits) = selected else { return Ok((None, RenderTransitionReceipts::new())); };
    let mut promotions = Vec::new();
    if carrier_projection.is_some() {
        for (span, edit) in candidate.spans.iter().zip(edits.iter_mut()) {
            let normalized = span.fragment.replace("print-object=\"yes\"", "print-object=\"no\"").replace("print-object='yes'", "print-object='no'");
            let (hidden, receipt) = hide_note_fragment_print_object_no(&normalized);
            if let Some(receipt) = receipt {
                assert_eq!(normalized, span.fragment, "#20 pre must be the original fragment");
                promotions.push(DurationPromotion::new(span.start, span.end, hidden.clone(), receipt));
            }
            edit.2 = if hidden.contains("print-spacing=\"yes\"") { hidden.replace("print-spacing=\"yes\"", "print-spacing=\"no\"") }
                else if hidden.contains("print-spacing='yes'") { hidden.replace("print-spacing='yes'", "print-spacing='no'") }
                else if hidden.contains("print-spacing=\"no\"") || hidden.contains("print-spacing='no'") { hidden }
                else { hidden.replacen("<note", "<note print-spacing=\"no\"", 1) };
        }
    } else {
        let anchor = &candidate.spans[0];
        let (duration_post, receipt) = set_rest_note_duration_type_with_receipt(&anchor.fragment, candidate.total_duration);
        let Some(duration_post) = duration_post else { return Ok((None, RenderTransitionReceipts::new())); };
        if let Some(receipt) = receipt { promotions.push(DurationPromotion::new(anchor.start, anchor.end, duration_post.clone(), receipt)); }
        edits[0].2 = replace_xml_tag_text(&replace_xml_tag_text(&duration_post, "staff", &anchor.staff), "voice", &anchor.voice);
        let (_, measure_end, _) = find_measure_bounds_containing_pos(xml, anchor.start).expect("locator checked before mint");
        for span in candidate.spans.iter().skip(1) {
            if note_is_followed_by_chord_note(xml, span.end, measure_end) {
                let (hidden, receipt) = hide_note_fragment_print_object_no(&span.fragment);
                if let Some(receipt) = receipt { promotions.push(DurationPromotion::new(span.start, span.end, hidden.clone(), receipt)); }
                if let Some(edit) = edits.iter_mut().find(|edit| edit.0 == span.start && edit.1 == span.end) { edit.2 = hidden; }
            }
        }
    }
    let (draft, rebased, mut receipts) = crate::structural_write_authority::mutate_selected_duration_fragments(xml, promotions, edits)?;
    let (post, receipt) = crate::structural_write_authority::apply_layer_balanced_rest_consolidation_authorized(&draft, rebased);
    receipts.push_exact(receipt);
    if post.is_none() {
        receipts.discard_unadopted(RenderDiscardReason::LosingCandidate);
        return Ok((None, RenderTransitionReceipts::new()));
    }
    Ok((post, receipts))
}

fn apply_structural_transactional_rest_coalesce_render_write_prepared(
    xml: &str,
    max_transactions: usize,
) -> (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize, crate::master_state_identity::RenderTransitionReceipts) {
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    let mut current_xml = xml.to_string();
    let mut actions: Vec<BehindBarsRuleCheckRestAndTieAction> = Vec::new();
    let initial_candidates = build_rest_consolidation_candidates(&current_xml);
    let seen = initial_candidates.len();
    let mut processed_keys: BTreeSet<(usize, String, String, i32, i32)> = BTreeSet::new();
    for _ in 0..max_transactions {
        let candidates = build_rest_consolidation_candidates(&current_xml);
        let Some(candidate) = candidates.into_iter().find(|candidate| {
            let key = (
                candidate.scope,
                candidate.staff.clone(),
                candidate.voice.clone(),
                candidate.first_onset,
                candidate.total_duration,
            );
            !processed_keys.contains(&key)
        }) else { break; };
        let key = (
            candidate.scope,
            candidate.staff.clone(),
            candidate.voice.clone(),
            candidate.first_onset,
            candidate.total_duration,
        );
        processed_keys.insert(key);
        let chord_anchor_dependency = candidate.spans.iter().any(|span| {
            note_is_followed_by_chord_note(&current_xml, span.end, current_xml.len())
        });
        let metric_by_scope = render_scope_metric_expected_and_beat_map(&current_xml);
        let Some((expected, beat_unit)) = metric_by_scope.get(&candidate.scope).copied() else {
            continue;
        };
        let all_short = candidate.spans.iter().all(|span| span.duration <= 96);
        if crate::model::metric_rest_notation::decide_rest_span_consolidation(
            crate::model::metric_rest_notation::RestSpanConsolidationFacts {
                start: i64::from(candidate.first_onset),
                duration: i64::from(candidate.total_duration),
                expected_measure_duration: i64::from(expected),
                beat_duration: i64::from(beat_unit),
                source_fragment_count: candidate.spans.len(),
                has_chord_anchor_dependency: chord_anchor_dependency,
                all_source_fragments_short: all_short,
            },
            crate::model::metric_rest_notation::RestSpanConsolidationProjection::SinglePartShortBeatLocal,
        )
        .is_none()
        {
            if chord_anchor_dependency {
                actions.push(BehindBarsRuleCheckRestAndTieAction {
                    action_kind: "external_feedback_transactional_rest_coalesce".to_string(),
                    old_rest_count: candidate.spans.len(),
                    new_rest_count: 1,
                    verdict: "rejected_chord_anchor_dependency".to_string(),
                });
            }
            continue;
        }
        let (before_events, before_backup_ok) = collect_render_xml_note_events(&current_xml);
        let mut before_pitched_timeline = before_events
            .iter()
            .filter_map(|event| {
                event.midi.map(|midi| {
                    (
                        event.scope,
                        event.staff.clone(),
                        event.voice.clone(),
                        event.onset,
                        event.duration,
                        midi,
                        event.is_hidden,
                        event.is_chord,
                    )
                })
            })
            .collect::<Vec<_>>();
        before_pitched_timeline.sort();
        let before_visible_pitched = before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
        let before_hidden_pitched = before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
        let before_visible_rests = before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
        let before_dedup = render_visible_sounding_signature_dedup(&before_events);
        let (candidate_xml, local_receipts) = apply_single_rest_consolidation_candidate_prepared(&current_xml, &candidate);
        let Some(candidate_xml) = candidate_xml else {
            local_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
            actions.push(BehindBarsRuleCheckRestAndTieAction {
                action_kind: "external_feedback_transactional_rest_coalesce".to_string(),
                old_rest_count: candidate.spans.len(),
                new_rest_count: 1,
                verdict: "rejected_xml_locator_failed".to_string(),
            });
            continue;
        };
        let (after_events, after_backup_ok) = collect_render_xml_note_events(&candidate_xml);
        let mut after_pitched_timeline = after_events
            .iter()
            .filter_map(|event| {
                event.midi.map(|midi| {
                    (
                        event.scope,
                        event.staff.clone(),
                        event.voice.clone(),
                        event.onset,
                        event.duration,
                        midi,
                        event.is_hidden,
                        event.is_chord,
                    )
                })
            })
            .collect::<Vec<_>>();
        after_pitched_timeline.sort();
        let after_visible_pitched = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
        let after_hidden_pitched = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
        let after_visible_rests = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
        let after_dedup = render_visible_sounding_signature_dedup(&after_events);
        let expected_rest_delta = candidate.spans.len().saturating_sub(1);
        let invariant_ok = before_backup_ok
            && after_backup_ok
            && parse_musicxml(&candidate_xml).is_ok()
            && before_visible_pitched == after_visible_pitched
            && before_hidden_pitched == after_hidden_pitched
            && before_pitched_timeline == after_pitched_timeline
            && before_dedup == after_dedup
            && after_visible_rests.saturating_add(expected_rest_delta) == before_visible_rests;
        let verdict = if invariant_ok {
            current_xml = candidate_xml;
            receipts.append(local_receipts);
            "adopted_transactionally"
        } else {
            local_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
            "rejected_by_individual_invariant_guard"
        };
        actions.push(BehindBarsRuleCheckRestAndTieAction {
            action_kind: "external_feedback_transactional_rest_coalesce".to_string(),
            old_rest_count: candidate.spans.len(),
            new_rest_count: 1,
            verdict: verdict.to_string(),
        });
    }
    (current_xml, actions, seen, receipts)
}

fn apply_lower_staff_secondary_reanchored_lane_rest_visibility_suppression_with_receipts(
    xml: &str,
    max_actions: usize,
) -> (String, Vec<LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionAction>, usize, crate::master_state_identity::RenderTransitionReceipts) {
    let mut candidates_seen = 0_usize;
    let mut selected_replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut actions = Vec::<LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionAction>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let all_fragments = collect_render_timeline_fragments_in_segment(measure, measure_start, scope_number);
        let lower_staff_visible_notes = all_fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && frag.staff.trim() == "2")
            .count();
        if lower_staff_visible_notes == 0 {
            scan = measure_end;
            continue;
        }
        let mut local = 0_usize;
        while let Some(backup_rel) = measure[local..].find("<backup") {
            if actions.len() >= max_actions {
                break;
            }
            let backup_start = local + backup_rel;
            let Some(backup_close_rel) = measure[backup_start..].find("</backup>") else { break; };
            let backup_end = backup_start + backup_close_rel + "</backup>".len();
            let backup_fragment = &measure[backup_start..backup_end];
            let backup_duration = extract_first_text_from_fragment(backup_fragment, "duration")
                .and_then(|value| value.trim().parse::<i32>().ok())
                .unwrap_or(0);
            if backup_duration <= 0 {
                local = backup_end;
                continue;
            }
            let next_backup = measure[backup_end..].find("<backup").map(|pos| backup_end + pos);
            let measure_close = measure.rfind("</measure>").unwrap_or(measure.len());
            let segment_end = next_backup.unwrap_or(measure_close);
            if segment_end <= backup_end {
                local = backup_end;
                continue;
            }
            let segment = &measure[backup_end..segment_end];
            let fragments = collect_render_timeline_fragments_in_segment(segment, measure_start + backup_end, scope_number);
            let has_forward_anchor = fragments.iter().any(|frag| frag.is_forward);
            if !has_forward_anchor {
                local = backup_end;
                continue;
            }
            let mut rests_by_staff_voice: BTreeMap<(String, String), Vec<RenderXmlTimelineFragmentSpan>> = BTreeMap::new();
            for frag in fragments.iter().filter(|frag| frag.is_visible_rest && frag.staff.trim() == "2") {
                rests_by_staff_voice
                    .entry((frag.staff.clone(), frag.voice.clone()))
                    .or_default()
                    .push(frag.clone());
            }
            if rests_by_staff_voice.is_empty() {
                local = backup_end;
                continue;
            }
            for ((staff, voice), mut visible_lower_rests) in rests_by_staff_voice {
                if actions.len() >= max_actions {
                    break;
                }
                visible_lower_rests.sort_by_key(|frag| (frag.onset, frag.duration, frag.start));
                let has_unsafe_rest = visible_lower_rests
                    .iter()
                    .any(|frag| frag.has_visible_dependency || frag.duration <= 0 || frag.is_chord);
                if has_unsafe_rest {
                    continue;
                }
                let has_visible_pitched_in_same_lane = fragments.iter().any(|frag| {
                    frag.is_visible_pitched && frag.staff == staff && frag.voice == voice
                });
                let has_visible_pitched_in_lower_segment = fragments.iter().any(|frag| {
                    frag.is_visible_pitched && frag.staff.trim() == "2"
                });
                let rest_fragment_chain = visible_lower_rests.len() >= 2
                    || visible_lower_rests.iter().any(|frag| frag.duration <= 24);
                if !rest_fragment_chain {
                    continue;
                }
                if !has_visible_pitched_in_same_lane && !has_visible_pitched_in_lower_segment {
                    continue;
                }
                candidates_seen += 1;
                let mut replacements = Vec::<(usize, usize, String)>::new();
                let mut overlap = false;
                for rest in &visible_lower_rests {
                    if occupied.iter().any(|(used_start, used_end)| rest.start < *used_end && rest.end > *used_start) {
                        overlap = true;
                        break;
                    }
                    replacements.push((rest.start, rest.end, render_forward_fragment(rest.duration)));
                }
                if overlap {
                    continue;
                }
                for rest in &visible_lower_rests {
                    occupied.push((rest.start, rest.end));
                }
                selected_replacements.extend(replacements);
                actions.push(LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionAction {
                    scope: scope_number,
                    old_rest_count: visible_lower_rests.len(),
                    risk: "medium_render_only_behind_bars_red_finding".to_string(),
                });
            }
            local = backup_end;
        }
        scan = measure_end;
    }
    let (out, receipt) = crate::structural_write_authority::apply_secondary_reanchored_rest_visibility_authorized(
        xml,
        selected_replacements,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    (out, actions, candidates_seen, receipts)
}

fn build_lower_staff_secondary_reanchored_lane_rest_visibility_suppression_trial_prepared(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionTrial, Option<String>, crate::master_state_identity::RenderTransitionReceipts)> {
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut trial = LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionTrial {
        source_label: source_label.to_string(),
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        parse_before_ok: parse_musicxml(xml).is_ok(),
        backup_forward_before_ok: before_backup_forward_ok,
        ..LowerStaffSecondaryReanchoredLaneRestVisibilitySuppressionTrial::default()
    };
    if !trial.parse_before_ok || !trial.backup_forward_before_ok {
        trial.verdict = "lower_staff_secondary_reanchored_lane_rest_visibility_suppression_blocked_by_input_parse_or_timeline".to_string();
        trial.done_fragment = "LowerStaffSecondaryReanchoredLaneRestVisibilitySuppression: blockiert vor Mutation; keine Änderung".to_string();
        trial.pipeline_line = format!(
            "ReanchorRestVisibilitySafetyNet v{}: blockiert vor Prüfung; parse/back-forward {}/{}; keine Render-XML-Änderung.",
            env!("CARGO_PKG_VERSION"),
            if trial.parse_before_ok { "ok" } else { "fail" },
            if trial.backup_forward_before_ok { "ok" } else { "fail" }
        );
        return Ok((trial, None, crate::master_state_identity::RenderTransitionReceipts::new()));
    }
    trial.attempted = true;
    let (candidate_xml, actions, candidates_seen, receipts) = apply_lower_staff_secondary_reanchored_lane_rest_visibility_suppression_with_receipts(xml, 24);
    trial.candidates_seen = candidates_seen;
    trial.candidate_attempted = actions.len();
    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    trial.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.raw_cursor_after_ok = scan_raw_cursor_xml("RenderXML", &candidate_xml)
        .into_iter()
        .all(|row| !row.hard_violation && row.partial_backup_count == 0 && row.post_expected_events == 0);
    trial.visible_rest_delta = trial.visible_rests_after as i64 - trial.visible_rests_before as i64;
    trial.affected_measures = actions.iter().map(|action| action.scope).collect::<BTreeSet<_>>().len();
    trial.affected_visible_rests = actions.iter().map(|action| action.old_rest_count).sum();
    trial.forward_segments_written = trial.affected_visible_rests;
    let invariants_ok = trial.candidate_attempted > 0
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.hidden_pitched_before == trial.hidden_pitched_after
        && trial.hidden_rests_before == trial.hidden_rests_after
        && trial.parse_after_ok
        && trial.backup_forward_after_ok
        && trial.raw_cursor_after_ok;
    if invariants_ok {
        trial.written = true;
        trial.candidate_adopted = trial.candidate_attempted;
        trial.verdict = "lower_staff_secondary_reanchored_lane_rest_visibility_suppression_adopted_render_xml_only".to_string();
    } else {
        trial.candidate_rejected = trial.candidate_attempted;
        trial.verdict = if trial.candidate_attempted == 0 {
            "lower_staff_secondary_reanchored_lane_rest_visibility_suppression_no_safe_candidate".to_string()
        } else {
            "lower_staff_secondary_reanchored_lane_rest_visibility_suppression_rejected_by_invariant_guard".to_string()
        };
    }
    trial.actions = actions;
    trial.examples.push("Render-XML-only: Der Operator ersetzt nur sichtbare Rest-Platzhalter einer Backup/Forward-reankerten unteren Rest-only-Lane durch Forward-Zeit.".to_string());
    trial.examples.push("Keine Musikmutation: sichtbare klingende Noten, versteckte klingende Noten, Tonhöhen, Onsets, Dauern, Staffs und Voices klingender Events bleiben gleich.".to_string());
    trial.examples.push("Kein allgemeiner Pausen-Flick: Kandidaten brauchen unteres System, mehrere/kleinteilige sichtbare Restfragmente, Forward-Reankerung und keine klingenden Events in genau derselben Staff-/Voice-Rest-Lane.".to_string());
    trial.done_fragment = format!(
        "LowerStaffSecondaryReanchoredLaneRestVisibilitySuppression: Kandidaten {}, übernommen {}, sichtbare Pausen {}→{}, RawCursor {}, KlangMutation nein",
        trial.candidates_seen,
        trial.candidate_adopted,
        trial.visible_rests_before,
        trial.visible_rests_after,
        if trial.raw_cursor_after_ok { "gruen" } else { "rot" }
    );
    trial.pipeline_line = format!(
        "ReanchorRestVisibilitySafetyNet v{}: Nachlaufprüfung auf {}; Kandidaten {}, versucht {}, übernommen {}, verworfen {}; betroffene Bereiche {}, sichtbare Rest-Platzhalter {}, Forward-Segmente {}; sichtbare Pausen {}→{}; sichtbare klingende Noten {}→{}; versteckte klingende Noten {}→{}; versteckte Pausen {}→{}; SoundingSignature stabil {}; parse/back-forward/raw-cursor nachher {}/{}/{}; geschrieben {}; Hauptbeweis ReanchorContract; MasterApply nein; MusicMutation nein; NotationRuleChange ja; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
        trial.candidate_rejected,
        trial.affected_measures,
        trial.affected_visible_rests,
        trial.forward_segments_written,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.hidden_pitched_before,
        trial.hidden_pitched_after,
        trial.hidden_rests_before,
        trial.hidden_rests_after,
        if trial.sounding_signature_stable { "ja" } else { "nein" },
        if trial.parse_after_ok { "ok" } else { "fail" },
        if trial.backup_forward_after_ok { "ok" } else { "fail" },
        if trial.raw_cursor_after_ok { "ok" } else { "fail" },
        if trial.written { "ja" } else { "nein" },
        trial.verdict
    );
    let candidate = trial.written.then_some(candidate_xml);
    let _ = output_path;
    Ok((trial, candidate, receipts))
}

fn build_strict_visible_rest_arithmetic_materializer_trial_prepared(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(RenderXmlPianisticRestRepairTrial, Option<String>, crate::master_state_identity::RenderTransitionReceipts)> {
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let mut trial = RenderXmlPianisticRestRepairTrial {
        source_label: source_label.to_string(),
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        backup_forward_before_ok: before_backup_forward_ok,
        parse_before_ok: parse_musicxml(xml).is_ok(),
        ..RenderXmlPianisticRestRepairTrial::default()
    };
    if !trial.parse_before_ok || !trial.backup_forward_before_ok {
        trial.verdict = "strict_visible_rest_arithmetic_materializer_blocked_by_input_parse_or_timeline".to_string();
        trial.done_fragment = "FinalRestClosureByInvariantProof: blockiert vor Mutation; keine Änderung".to_string();
        trial.pipeline_line = format!(
            "FinalRestClosureByInvariantProof v{}: blockiert vor Mutation; parse/back-forward {}/{}; keine Render-XML-Änderung.",
            env!("CARGO_PKG_VERSION"),
            if trial.parse_before_ok { "ok" } else { "fail" },
            if trial.backup_forward_before_ok { "ok" } else { "fail" }
        );
        return Ok((trial, None, crate::master_state_identity::RenderTransitionReceipts::new()));
    }
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    trial.attempted = true;
    let initial_consolidation_candidates = build_rest_consolidation_candidates(xml).len();
    let mut blocked_examples = Vec::new();
    let (_missing_probe_xml, _missing_probe_actions, missing_safe_seen, missing_rows_seen, probe_receipt) =
        apply_pianistic_missing_rest_insert_with_receipt(xml, 0, &mut blocked_examples);
    let mut probe = crate::master_state_identity::RenderTransitionReceipts::new();
    probe.push_exact(probe_receipt);
    probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    trial.missing_candidates_seen = missing_rows_seen;
    trial.consolidation_candidates_seen = initial_consolidation_candidates;
    let (mut candidate_xml, mut actions, first_consolidation_count, first_receipts) = apply_pianistic_rest_consolidation_prepared(xml, 256);
    receipts.append(first_receipts);
    trial.consolidation_candidates_seen = trial.consolidation_candidates_seen.max(first_consolidation_count);
    trial.consolidation_attempted += actions.len();
    let mut missing_examples = Vec::new();
    let (after_missing_xml, missing_actions, missing_safe_count, missing_rows_seen_after_consolidation, missing_receipt) =
        apply_pianistic_missing_rest_insert_with_receipt(&candidate_xml, 64, &mut missing_examples);
    let mut missing_receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    missing_receipts.push_exact(missing_receipt);
    trial.missing_candidates_seen = trial.missing_candidates_seen.max(missing_rows_seen_after_consolidation);
    trial.missing_inserted = missing_actions.len();
    if !missing_actions.is_empty() {
        receipts.append(missing_receipts);
        candidate_xml = after_missing_xml;
        for action in missing_actions {
            actions.push(RenderXmlPianisticRestRepairAction {
                scope: action.scope,
                staff: action.staff,
                voice: action.voice,
                verdict: "adopted_if_global_invariant_guard_holds".to_string(),
            });
        }
    }
    else { missing_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate); }
    let (after_second_consolidation_xml, mut second_actions, second_consolidation_count, second_receipts) =
        apply_pianistic_rest_consolidation_prepared(&candidate_xml, 256);
    trial.consolidation_candidates_seen = trial.consolidation_candidates_seen.max(second_consolidation_count);
    trial.consolidation_attempted += second_actions.len();
    if !second_actions.is_empty() {
        receipts.append(second_receipts);
        candidate_xml = after_second_consolidation_xml;
        actions.append(&mut second_actions);
    }
    else { second_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate); }
    let mut concrete_closure_rounds = 0_usize;
    let mut concrete_closure_remaining_probe = 0_usize;
    for _ in 0..12 {
        let (after_loop_consolidation_xml, mut loop_actions, loop_consolidation_count, loop_receipts) =
            apply_pianistic_rest_consolidation_prepared(&candidate_xml, 256);
        concrete_closure_remaining_probe = loop_consolidation_count;
        trial.consolidation_candidates_seen = trial.consolidation_candidates_seen.max(loop_consolidation_count);
        if loop_actions.is_empty() {
            loop_receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
            break;
        }
        receipts.append(loop_receipts);
        concrete_closure_rounds += 1;
        trial.consolidation_attempted += loop_actions.len();
        candidate_xml = after_loop_consolidation_xml;
        actions.append(&mut loop_actions);
    }
    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.visible_signature_after = render_visible_signature(&after_events, false);
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    trial.visible_rest_delta = trial.visible_rests_after as i64 - trial.visible_rests_before as i64;
    let final_audit_probe = build_render_xml_visible_rest_readability_audit(
        &candidate_xml,
        "FinalRestClosureByInvariantProof-Kandidat nach Writes",
    );
    let remaining_safe_consolidations = build_rest_consolidation_candidates(&candidate_xml).len();
    let mut final_missing_examples = Vec::new();
    let (_final_missing_xml, _final_missing_actions, final_missing_safe, final_missing_rows, final_probe_receipt) =
        apply_pianistic_missing_rest_insert_with_receipt(&candidate_xml, 0, &mut final_missing_examples);
    let mut final_probe = crate::master_state_identity::RenderTransitionReceipts::new();
    final_probe.push_exact(final_probe_receipt);
    final_probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe);
    let invariants_ok = !actions.is_empty()
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.backup_forward_after_ok
        && trial.parse_after_ok;
    if invariants_ok {
        trial.written = true;
        trial.consolidation_adopted = trial.consolidation_attempted;
        trial.verdict = if final_missing_safe == 0 && remaining_safe_consolidations == 0 {
            "strict_visible_rest_arithmetic_materialized_green_render_xml_only".to_string()
        } else {
            "strict_visible_rest_arithmetic_materialized_but_final_gate_must_recheck".to_string()
        };
        for action in &mut actions {
            action.verdict = "adopted".to_string();
        }
    } else {
        trial.consolidation_rejected = trial.consolidation_attempted + trial.missing_inserted;
        trial.verdict = if actions.is_empty() {
            "strict_visible_rest_arithmetic_materializer_no_safe_materialization_candidate".to_string()
        } else {
            "strict_visible_rest_arithmetic_materializer_rejected_by_global_invariant_guard".to_string()
        };
        for action in &mut actions {
            action.verdict = "rejected_by_global_invariant_guard".to_string();
        }
    }
    trial.actions = actions;
    trial.examples.push("Render-XML-only: Der Materializer setzt nur vorhandene forward-Zeit in sichtbare Pausen um oder konsolidiert bereits sichtbare Pausen derselben Staff/Voice; klingende Events bleiben unverändert.".to_string());
    trial.examples.push("CrossBeatMerge bleibt gesperrt: zusammengefasst wird nur, wenn die neue Pause vollständig innerhalb derselben dynamisch berechneten Zählzeit liegt.".to_string());
    trial.examples.push("Cross-Staff-/Bookkeeping-Stimmen werden nicht als fehlende eigenständige Pausenstimme materialisiert; solche Fälle müssen im Readback als nicht independent geführt werden, damit keine falsche Schattenpausenebene entsteht.".to_string());
    trial.examples.push(format!("Finale Sonden nach Kandidat: missing_rows {}, missing_safe {}, safe_consolidations {}, alter Planner missing {}, fragmented {}, consolidate {}; konkrete Closure-Runden {}, letzte Kandidatensonde {}.", final_missing_rows, final_missing_safe, remaining_safe_consolidations, final_audit_probe.missing_visible_rest_candidates, final_audit_probe.fragmented_chain_candidates, final_audit_probe.consolidate_candidates, concrete_closure_rounds, concrete_closure_remaining_probe));
    trial.done_fragment = format!(
        "FinalRestClosureByInvariantProof: fehlende sichtbare Pausen gesehen {}, sichere Missing-Kandidaten {}, eingefügt {}; Konsolidierungskandidaten {}, versucht {}, übernommen {}; konkrete Closure-Runden {}; Render-XML-only {}; klingend stabil {}",
        trial.missing_candidates_seen,
        missing_safe_count,
        trial.missing_inserted,
        trial.consolidation_candidates_seen,
        trial.consolidation_attempted,
        trial.consolidation_adopted,
        concrete_closure_rounds,
        if trial.written { "geschrieben" } else { "nicht geschrieben" },
        if trial.sounding_signature_stable { "ja" } else { "nein" }
    );
    trial.pipeline_line = format!(
        "FinalRestClosureByInvariantProof v{}: Render-XML-only auf {}; missing_rows_seen {}, missing_safe_seen {}, missing_safe_after_consolidation {}, missing_inserted {}; consolidation_candidates {}, attempted {}, adopted {}; concrete_closure_rounds {}; letzte_kandidatensonde {}; sichtbare Pausen {}→{}; sichtbare klingende Noten {}→{}; hidden pitched {}→{}; hidden rests {}→{}; sounding_signature {}/{} stabil {}; final_missing_safe {}; final_consolidations {}; parse/back-forward nachher {}/{}; geschrieben {}; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.missing_candidates_seen,
        missing_safe_seen,
        missing_safe_count,
        trial.missing_inserted,
        trial.consolidation_candidates_seen,
        trial.consolidation_attempted,
        trial.consolidation_adopted,
        concrete_closure_rounds,
        concrete_closure_remaining_probe,
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
        final_missing_safe,
        remaining_safe_consolidations,
        if trial.parse_after_ok { "ok" } else { "fail" },
        if trial.backup_forward_after_ok { "ok" } else { "fail" },
        if trial.written { "ja" } else { "nein" },
        trial.verdict
    );
    let candidate = trial.written.then_some(candidate_xml);
    let _ = output_path;
    Ok((trial, candidate, receipts))
}
