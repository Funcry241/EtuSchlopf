// Canonical structural mutation ownership.
// Included in the pipeline lexical namespace to retain private selection facts.
// The complete preparation and mutation implementations live here; no forwarding facade.

fn apply_strict_staff_local_release_completeness_materialization_with_receipts(
    xml: &str,
    max_actions: usize,
) -> (String, Vec<BehindBarsRuleCheckRestAndTieAction>, usize, Vec<String>, crate::master_state_identity::RenderTransitionReceipts) {
    let metric_by_scope = render_scope_metric_expected_and_beat_map(xml);
    let mut candidates = Vec::<StrictStaffVisibleMeasureCandidate>::new();
    let mut gap_trace = Vec::<String>::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let (expected, beat_unit) = metric_by_scope.get(&scope_number).copied().unwrap_or((384, 96));
        let end_cursor = render_timeline_cursor_at_segment_end(measure);
        let fragments = collect_render_timeline_fragments_in_segment(measure, measure_start, scope_number);
        let mut by_lane: BTreeMap<(String, String), Vec<RenderXmlTimelineFragmentSpan>> = BTreeMap::new();
        for frag in fragments.iter().filter(|frag| !frag.is_hidden && !frag.is_chord && (frag.is_visible_pitched || frag.is_visible_rest)) {
            let staff = frag.staff.trim();
            if staff == "1" || staff == "2" {
                by_lane.entry((frag.staff.clone(), frag.voice.clone())).or_default().push((*frag).clone());
            }
        }
        for ((staff_ref, voice_ref), lane_frags_src) in &by_lane {
            let staff = staff_ref.clone();
            let voice = voice_ref.clone();
            let mut lane_frags = lane_frags_src.clone();
            let visible_pitched = lane_frags.iter().filter(|frag| frag.is_visible_pitched).count();
            if visible_pitched == 0 { continue; }
            lane_frags.sort_by_key(|frag| (frag.onset, frag.duration, frag.start));
            let intervals = lane_frags.iter().map(|frag| (frag.onset, frag.duration.max(0))).collect::<Vec<_>>();
            let merged = strict_staff_merge_visible_intervals(intervals, expected);
            let gaps = strict_staff_gaps_from_merged_intervals(&merged, expected);
            if gaps.is_empty() { continue; }
            // r1016: A visible pitched <chord/> member without a visible non-chord
            // lane anchor is already positive occupancy evidence at this render
            // boundary, but StrictStaff cannot safely infer its missing anchor or
            // rewrite its chord topology. Therefore only the overlapping parent gap
            // loses rest write right. Other independent gaps in the same lane remain
            // eligible for the existing guards and materialization path.
            let gaps = gaps
                .into_iter()
                .filter(|(gap_start, gap_end, _)| {
                    !fragments.iter().any(|frag| {
                        frag.is_visible_pitched
                            && !frag.is_hidden
                            && frag.is_chord
                            && frag.staff.trim() == staff.trim()
                            && frag.voice.trim() == voice.trim()
                            && frag.onset < *gap_end
                            && *gap_start < frag.onset.saturating_add(frag.duration.max(0))
                    })
                })
                .collect::<Vec<_>>();
            if gaps.is_empty() { continue; }
            // r1015: StrictStaff is a repair writer, not an authority for inventing
            // polyphonic rest semantics. If a lane gap overlaps visible pitched
            // occupancy on the same staff under another render voice, the write
            // decision is ambiguous. Fail closed: preserve the current XML and let
            // later canonical role owners reason about any genuinely required rest.
            // Existing rests are never removed by this guard.
            let same_staff_other_voice_pitch_overlap =
                gaps.iter().any(|(gap_start, gap_end, _)| {
                    fragments.iter().any(|frag| {
                        frag.is_visible_pitched
                            && !frag.is_hidden
                            && !frag.is_chord
                            && frag.staff.trim() == staff.trim()
                            && frag.voice.trim() != voice.trim()
                            && frag.onset < *gap_end
                            && *gap_start < frag.onset.saturating_add(frag.duration.max(0))
                    })
                });
            if same_staff_other_voice_pitch_overlap {
                gap_trace.push(format!(
                    "scope {} staff {} voice {} gaps {} -> no_overlay_same_staff_other_voice_pitch_ambiguity",
                    scope_number,
                    staff,
                    voice,
                    format_visible_lane_gaps(&gaps, 8)
                ));
                continue;
            }
            // r418: A staff-local projection gap is not voice silence when the same
            // voice is visibly sounding on the other piano staff for the complete gap.
            // This is a negative write guard only: cross-staff evidence may suppress an
            // overlay rest, but it never authorizes a new note/rest/timeline mutation.
            let other_staff_pitched_merged = strict_staff_merge_visible_intervals(
                by_lane
                    .iter()
                    .filter(|((other_staff, other_voice), _)| {
                        other_voice == &voice
                            && other_staff.trim() != staff.trim()
                            && (other_staff.trim() == "1" || other_staff.trim() == "2")
                    })
                    .flat_map(|(_, other_frags)| other_frags.iter())
                    .filter(|frag| frag.is_visible_pitched && !frag.is_hidden && !frag.is_chord)
                    .map(|frag| (frag.onset, frag.duration.max(0)))
                    .collect::<Vec<_>>(),
                expected,
            );
            let same_voice_other_staff_gap_pitch_covered = gaps.iter().all(|(gap_start, gap_end, _)| {
                other_staff_pitched_merged.iter().any(|(cover_start, cover_duration)| {
                    *cover_start <= *gap_start
                        && cover_start.saturating_add(*cover_duration) >= *gap_end
                })
            });
            if same_voice_other_staff_gap_pitch_covered {
                gap_trace.push(format!(
                    "scope {} staff {} voice {} gaps {} -> no_overlay_same_voice_other_staff_pitch_coverage",
                    scope_number,
                    staff,
                    voice,
                    format_visible_lane_gaps(&gaps, 8)
                ));
                continue;
            }
            // A shared technical label does not bind a musical rest responsibility.
            // Even partial cross-staff pitched occupancy leaves the proposed silence
            // unbound here. Include hidden/chord carriers; neither licenses silence.
            // This records a refusal premise, never a role or a replacement label.
            let overlaps_other_staff_pitched_evidence = gaps.iter().any(|(start, end, _)| {
                fragments.iter().any(|frag| {
                    frag.is_note
                        && !frag.is_rest
                        && frag.staff.trim() != staff.trim()
                        && frag.voice.trim() == voice.trim()
                        && midi_from_note_fragment(&frag.fragment).is_some()
                        && frag.duration > 0
                        && frag.onset < *end
                        && *start < frag.onset.saturating_add(frag.duration)
                })
            });
            let gap_parts = strict_staff_gap_parts(&gaps, expected, beat_unit);
            if gap_parts.is_empty() {
                gap_trace.push(format!(
                    "scope {} staff {} voice {} gaps {} -> no_gap_parts_after_metric_rest_plan",
                    scope_number,
                    staff,
                    voice,
                    format_visible_lane_gaps(&gaps, 8)
                ));
                continue;
            }
            let gap_desc = format_visible_lane_gaps(&gaps, 8);
            let gap_part_desc = format_interval_parts(&gap_parts);
            gap_trace.push(format!(
                "scope {} staff {} voice {} expected {} beat_unit {} visible_union {} gaps {} gap_parts {} end_cursor {}",
                scope_number,
                staff,
                voice,
                expected,
                beat_unit,
                merged.iter().map(|(_, duration)| *duration).sum::<i32>().min(expected).max(0),
                gap_desc,
                gap_part_desc,
                end_cursor
            ));
            let Some((overlay, rest_count, rest_total, rest_parts)) =
                strict_staff_overlay_rest_layer_for_gaps(
                    expected,
                    end_cursor,
                    &gap_parts,
                    &staff,
                    &voice,
                )
            else {
                gap_trace.push(format!(
                    "scope {scope_number} staff {staff} voice {voice} -> blocked_by_overlay_builder gap_parts {gap_part_desc}"
                ));
                continue;
            };
            gap_trace.push(format!(
                "scope {} staff {} voice {} -> overlay_backup_rest_forward_candidate rest_count {} rest_total {} rest_parts {} reason explicit_staff_voice_overlay_materialization",
                scope_number,
                staff,
                voice,
                rest_count,
                rest_total,
                format_interval_parts(&rest_parts)
            ));
            let replacements = vec![(
                measure_end.saturating_sub("</measure>".len()),
                measure_end.saturating_sub("</measure>".len()),
                overlay,
            )];
            let score = (rest_total as i64)
                + (gaps.len() as i64 * 80)
                + if staff.trim() == "1" { 20 } else { 10 };
            candidates.push(StrictStaffVisibleMeasureCandidate {
                score,
                scope: scope_number,
                staff: staff.clone(),
                voice: voice.clone(),
                replacements,
                overlaps_other_staff_pitched_evidence,
                action: BehindBarsRuleCheckRestAndTieAction {
                    action_kind: "strict_staff_local_release_overlay_rest_materialization".to_string(),
                    old_rest_count: 0,
                    new_rest_count: rest_count,
                    verdict: "adopted_if_rest_delta_adoption_gate_holds".to_string(),
                },
            });
        }
        scan = measure_end;
    }
    let candidates_seen = candidates.len();
    candidates.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.scope.cmp(&b.scope)).then_with(|| a.staff.cmp(&b.staff)).then_with(|| a.voice.cmp(&b.voice)));
    let mut selected = Vec::<(usize, usize, String)>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    let mut actions = Vec::<BehindBarsRuleCheckRestAndTieAction>::new();
    let mut reserved_actions = 0usize;
    for candidate in candidates.into_iter() {
        if reserved_actions >= max_actions { break; }
        let overlaps = candidate.replacements.iter().any(|(start, end, _)| occupied.iter().any(|(used_start, used_end)| *start < *used_end && *used_start < *end));
        if overlaps { continue; }
        // Preserve the old action/footprint reservation: refusing an unbound rest
        // must not promote another candidate through the legacy bounded selection.
        reserved_actions += 1;
        occupied.extend(candidate.replacements.iter().map(|(start, end, _)| (*start, *end)));
        if candidate.overlaps_other_staff_pitched_evidence {
            gap_trace.push(format!(
                "scope {} staff {} voice {} -> no_overlay_unbound_cross_staff_rest_context; existing_carriers_preserved; legacy_slot_reserved",
                candidate.scope, candidate.staff, candidate.voice,
            ));
            continue;
        }
        selected.extend(candidate.replacements);
        actions.push(candidate.action);
    }
    gap_trace.push(format!(
        "candidate_selection: seen {} selected_actions {} selected_replacements {} max_actions {}",
        candidates_seen,
        actions.len(),
        selected.len(),
        max_actions
    ));
    let (out, receipt) = crate::structural_write_authority::apply_strict_staff_gap_materialization_authorized(
        xml,
        selected,
    );
    let out = out.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    (out, actions, candidates_seen, gap_trace, receipts)
}

fn build_strict_staff_local_release_completeness_with_transient_wav_gate_trial_prepared(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(StrictStaffOverfullCoverageNormalizationTrial, Option<String>, crate::master_state_identity::RenderTransitionReceipts)> {
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let before_strict = build_strict_staff_local_release_completeness_readback(xml, source_label);
    let mut trial = StrictStaffOverfullCoverageNormalizationTrial {
        source_label: source_label.to_string(),
        parse_before_ok: parse_musicxml(xml).is_ok(),
        backup_forward_before_ok: before_backup_forward_ok,
        strict_incomplete_before: before_strict.incomplete_lanes,
        strict_missing_start_before: before_strict.missing_start,
        strict_missing_internal_before: before_strict.missing_internal,
        strict_missing_end_before: before_strict.missing_end,
        wav_gate_mode: "transient_attack_only_no_sustain_energy_no_overtone_block".to_string(),
        cross_staff_gluing_used_as_release_proof: 0,
        transient_wav_contradictions: 0,
        visible_pitched_before: before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count(),
        visible_rests_before: before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count(),
        hidden_pitched_before: before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count(),
        hidden_rests_before: before_events.iter().filter(|event| event.is_hidden && event.is_rest).count(),
        sounding_signature_before: render_visible_sounding_signature(&before_events),
        ..StrictStaffOverfullCoverageNormalizationTrial::default()
    };
    if !trial.parse_before_ok || !trial.backup_forward_before_ok {
        trial.verdict = "strict_staff_local_release_blocked_by_input_parse_or_timeline".to_string();
        trial.done_fragment = "StrictStaffOverfullCoverageNormalization: blockiert vor Materialisierung; Input-Parse/Timeline nicht gruen".to_string();
        trial.pipeline_line = format!(
            "StrictStaffOverfullCoverageNormalization v{}: blockiert auf {}; parse/back-forward {}/{}; CrossStaffGluingUsedAsReleaseProof 0; keine Änderung; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            trial.source_label,
            if trial.parse_before_ok { "ok" } else { "fail" },
            if trial.backup_forward_before_ok { "ok" } else { "fail" },
            trial.verdict
        );
        return Ok((trial, None, crate::master_state_identity::RenderTransitionReceipts::new()));
    }
    let (candidate_xml, mut actions, seen, _, receipts) =
        apply_strict_staff_local_release_completeness_materialization_with_receipts(xml, 32);
    trial.candidates_seen = seen;
    trial.candidate_attempted = actions.len();
    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    let after_strict = build_strict_staff_local_release_completeness_readback(&candidate_xml, "Render-XML nach StrictStaffOverfullCoverageNormalization-Kandidat");
    trial.strict_incomplete_after = after_strict.incomplete_lanes;
    trial.strict_missing_start_after = after_strict.missing_start;
    trial.strict_missing_internal_after = after_strict.missing_internal;
    trial.strict_missing_end_after = after_strict.missing_end;
    trial.rows_after = after_strict.rows.clone();
    trial.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    trial.backup_forward_after_ok = after_backup_forward_ok;
    trial.raw_cursor_after_ok = scan_raw_cursor_xml("RenderXML", &candidate_xml)
        .into_iter()
        .all(|row| !row.hard_violation && row.partial_backup_count == 0 && row.post_expected_events == 0);
    trial.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    trial.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    trial.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    trial.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    trial.sounding_signature_after = render_visible_sounding_signature(&after_events);
    trial.sounding_signature_stable = trial.sounding_signature_before == trial.sounding_signature_after;
    let strict_staff_improved = trial.strict_incomplete_after < trial.strict_incomplete_before
        || (trial.strict_missing_start_after + trial.strict_missing_internal_after + trial.strict_missing_end_after)
            < (trial.strict_missing_start_before + trial.strict_missing_internal_before + trial.strict_missing_end_before);
    let visible_rest_delta_allowed = trial.visible_rests_after >= trial.visible_rests_before;
    let invariants_ok = trial.candidate_attempted > 0
        && strict_staff_improved
        && visible_rest_delta_allowed
        && trial.transient_wav_contradictions == 0
        && trial.parse_after_ok
        && trial.backup_forward_after_ok
        && trial.raw_cursor_after_ok
        && trial.sounding_signature_stable
        && trial.visible_pitched_before == trial.visible_pitched_after
        && trial.hidden_pitched_before == trial.hidden_pitched_after
        && trial.hidden_rests_before == trial.hidden_rests_after;
    if invariants_ok {
        trial.written = true;
        trial.candidate_adopted = trial.candidate_attempted;
        trial.verdict = "strict_staff_rest_delta_adopted_render_xml_only".to_string();
        for action in &mut actions { action.verdict = "adopted".to_string(); }
    } else {
        trial.candidate_rejected = trial.candidate_attempted;
        trial.verdict = if trial.candidate_attempted == 0 {
            "strict_staff_rest_delta_no_safe_materialization_candidate".to_string()
        } else {
            "strict_staff_rest_delta_rejected_by_invariant_or_wav_gate".to_string()
        };
        for action in &mut actions { action.verdict = "rejected_by_rest_delta_adoption_gate".to_string(); }
    }
    trial.actions = actions;
    trial.examples.push("HumanTrace-Regel: Eine staff-lokale sichtbare Luecke erzwingt keine Pause, wenn sie vollstaendig durch klingende Events derselben Voice auf dem anderen Piano-Staff gedeckt ist; diese Cross-Staff-Evidenz verengt nur das Schreibrecht und erzeugt selbst keine Mutation.".to_string());
    trial.examples.push("Materialisierung erzeugt für eine fehlende sichtbare Staff-/Voice-Lane ausschließlich eine explizit gebundene Backup/Rest/Forward-Overlay-Schicht; vorhandene Forward-Fragmente bleiben reine Timeline-Evidenz und werden nicht als Staff-/Voice-besessene Restquelle umgewidmet.".to_string());
    trial.examples.push("WAV-Gate ist bewusst transient-/Attack-orientiert; Sustain, Raum, Nachhall und Obertöne dürfen sichtbare Pausen nicht blockieren.".to_string());
    trial.done_fragment = format!(
        "StrictStaffOverfullCoverageNormalization: candidates {}, attempted {}, adopted {}, incomplete {}→{}, missing {}/{}/{}→{}/{}/{}, visible_rests {}→{}, CrossStaffGluingRelease 0, TransientWavContradictions {}, RAW {}, SoundingStable {}, Written {}",
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
        trial.strict_incomplete_before,
        trial.strict_incomplete_after,
        trial.strict_missing_start_before,
        trial.strict_missing_internal_before,
        trial.strict_missing_end_before,
        trial.strict_missing_start_after,
        trial.strict_missing_internal_after,
        trial.strict_missing_end_after,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.transient_wav_contradictions,
        if trial.raw_cursor_after_ok { "gruen" } else { "rot" },
        yes_no(trial.sounding_signature_stable),
        yes_no(trial.written)
    );
    trial.pipeline_line = format!(
        "StrictStaffOverfullCoverageNormalization v{}: Render-XML-only auf {}; candidates/attempted/adopted/rejected {}/{}/{}/{}; strict_incomplete {}→{}; missing_start/internal/end {}/{}/{}→{}/{}/{}; CrossStaffGluingUsedAsReleaseProof {}; wav_gate {}; transient_wav_contradictions {}; visible_rests {}→{}; visible_pitched {}→{}; hidden_pitched {}→{}; hidden_rests {}→{}; parse/back-forward/raw {}/{}/{}; sounding_signature stabil {}; geschrieben {}; MusicMutation nein; MasterApply nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        trial.source_label,
        trial.candidates_seen,
        trial.candidate_attempted,
        trial.candidate_adopted,
        trial.candidate_rejected,
        trial.strict_incomplete_before,
        trial.strict_incomplete_after,
        trial.strict_missing_start_before,
        trial.strict_missing_internal_before,
        trial.strict_missing_end_before,
        trial.strict_missing_start_after,
        trial.strict_missing_internal_after,
        trial.strict_missing_end_after,
        trial.cross_staff_gluing_used_as_release_proof,
        trial.wav_gate_mode,
        trial.transient_wav_contradictions,
        trial.visible_rests_before,
        trial.visible_rests_after,
        trial.visible_pitched_before,
        trial.visible_pitched_after,
        trial.hidden_pitched_before,
        trial.hidden_pitched_after,
        trial.hidden_rests_before,
        trial.hidden_rests_after,
        if trial.parse_after_ok { "ok" } else { "fail" },
        if trial.backup_forward_after_ok { "ok" } else { "fail" },
        if trial.raw_cursor_after_ok { "ok" } else { "fail" },
        yes_no(trial.sounding_signature_stable),
        yes_no(trial.written),
        trial.verdict
    );
    let candidate = trial.written.then_some(candidate_xml);
    let _ = output_path;
    Ok((trial, candidate, receipts))
}
