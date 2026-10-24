// Writer-free strict-staff lower/upper rest-lane planning.

#[derive(Debug, Clone)]
struct StrictStaffRestLaneMeasureObservation {
    scope: usize,
    expected: i32,
    fragments: Vec<RenderXmlTimelineFragmentSpan>,
}

fn collect_strict_staff_rest_lane_measure_observations(
    xml: &str,
) -> Vec<StrictStaffRestLaneMeasureObservation> {
    let mut observations = Vec::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) =
        find_next_measure_bounds(xml, scan)
    {
        let measure = &xml[measure_start..measure_end];
        observations.push(StrictStaffRestLaneMeasureObservation {
            scope: scope_number,
            expected: infer_measure_duration_from_measure_xml(measure)
                .unwrap_or(384)
                .max(1),
            fragments: collect_render_timeline_fragments_in_segment(
                measure,
                measure_start,
                scope_number,
            ),
        });
        scan = measure_end;
    }
    observations
}

fn format_interval_parts(parts: &[(i32, i32)]) -> String {
    if parts.is_empty() {
        return "-".to_string();
    }
    parts
        .iter()
        .map(|(start, duration)| format!("{start}+{duration}"))
        .collect::<Vec<_>>()
        .join(";")
}

fn push_render_xml_timeline_piece(out: &mut String, piece: &str) {
    if !out.is_empty() {
        out.push('\n');
    }
    out.push_str(piece);
}

fn lower_staff_rest_visual_cluster_key(onset: i32) -> i32 {
    // MusicXML divisions in this project are normally quarter=96; 24/48 are the
    // short rhythmic values that created the visible duplicate-rest artefacts.
    // A half-beat bucket catches visually stacked rests that are not strictly
    // overlapping in cursor math but sit in the same printed beat region.
    onset.max(0) / 48
}

fn lower_staff_rest_collision_relation(a: &RenderXmlTimelineFragmentSpan, b: &RenderXmlTimelineFragmentSpan) -> bool {
    if a.voice == b.voice {
        return false;
    }
    let a_start = a.onset;
    let b_start = b.onset;
    let a_end = a_start.saturating_add(a.duration.max(0));
    let b_end = b_start.saturating_add(b.duration.max(0));
    let strict_overlap = a_start < b_end && b_start < a_end;
    let same_visual_beat = lower_staff_rest_visual_cluster_key(a_start) == lower_staff_rest_visual_cluster_key(b_start);
    let near_onset = (a_start - b_start).abs() <= 24;
    strict_overlap || same_visual_beat || near_onset
}

fn count_lower_staff_visible_rest_collisions(xml: &str) -> (usize, String) {
    let mut collision_count = 0_usize;
    let mut scopes = BTreeSet::<usize>::new();
    for observation in collect_strict_staff_rest_lane_measure_observations(xml) {
        let scope_number = observation.scope;
        let fragments = &observation.fragments;
        let lower_visible_rests: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| {
                frag.is_visible_rest
                    && frag.staff.trim() == "2"
                    && !frag.is_chord
                    && frag.duration > 0
            })
            .cloned()
            .collect();
        for i in 0..lower_visible_rests.len() {
            for j in (i + 1)..lower_visible_rests.len() {
                let a = &lower_visible_rests[i];
                let b = &lower_visible_rests[j];
                if lower_staff_rest_collision_relation(a, b) {
                    collision_count += 1;
                    scopes.insert(scope_number);
                }
            }
        }
    }
    let scope_text = if scopes.is_empty() {
        "-".to_string()
    } else {
        scopes.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(",")
    };
    (collision_count, scope_text)
}

fn lower_staff_overlapping_rest_suppression_candidates(xml: &str) -> Vec<(i64, usize, usize, String, usize, String, i32)> {
    let mut candidates: Vec<(i64, usize, usize, String, usize, String, i32)> = Vec::new();
    for observation in collect_strict_staff_rest_lane_measure_observations(xml) {
        let scope_number = observation.scope;
        let fragments = &observation.fragments;
        let lower_visible_rests: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| {
                frag.is_visible_rest
                    && frag.staff.trim() == "2"
                    && !frag.is_chord
                    && frag.duration > 0
            })
            .cloned()
            .collect();
        if lower_visible_rests.len() < 2 {
            continue;
        }
        let lower_visible_pitched: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && frag.staff.trim() == "2" && frag.duration > 0)
            .cloned()
            .collect();
        let lower_pitch_present = !lower_visible_pitched.is_empty();

        for rest in &lower_visible_rests {
            let rest_start = rest.onset;
            let rest_end = rest.onset.saturating_add(rest.duration.max(0));
            if rest_end <= rest_start {
                continue;
            }
            let overlaps_other_rest = lower_visible_rests.iter().any(|other| {
                other.start != rest.start && lower_staff_rest_collision_relation(rest, other)
            });
            let overlaps_lower_pitched_other_voice = lower_visible_pitched.iter().any(|other| {
                other.voice != rest.voice
                    && rest_start < other.onset.saturating_add(other.duration.max(0))
                    && other.onset < rest_end
            });
            let measure_local_auxiliary_rest = lower_pitch_present
                && rest.voice.trim() != "1"
                && lower_visible_rests.iter().any(|other| other.start != rest.start && other.voice != rest.voice);

            if !overlaps_other_rest && !overlaps_lower_pitched_other_voice && !measure_local_auxiliary_rest {
                continue;
            }

            // Do not suppress the only primary rest in an otherwise empty lower staff.
            // Prefer auxiliary voices, short/clustered visible rests, and rests that collide
            // with a lower pitched voice. This is deliberately generic: no fixed measure.
            let aux_voice_bonus = if rest.voice.trim() != "1" { 2000 } else { 0 };
            let pitched_bonus = if overlaps_lower_pitched_other_voice { 900 } else { 0 };
            let rest_collision_bonus = if overlaps_other_rest { 700 } else { 0 };
            let measure_local_bonus = if measure_local_auxiliary_rest { 450 } else { 0 };
            let short_rest_bonus = if rest.duration <= 96 { 250 } else { 0 };
            let score = aux_voice_bonus
                + pitched_bonus
                + rest_collision_bonus
                + measure_local_bonus
                + short_rest_bonus
                + rest.duration.max(0) as i64
                + (scope_number as i64 / 1000);
            let evidence = format!(
                "scope {} staff {} voice {} onset {} duration {} overlap_rest_or_same_beat {} overlap_pitch {} measure_local_aux_rest {}",
                scope_number,
                rest.staff,
                rest.voice,
                rest.onset,
                rest.duration,
                if overlaps_other_rest { "ja" } else { "nein" },
                if overlaps_lower_pitched_other_voice { "ja" } else { "nein" },
                if measure_local_auxiliary_rest { "ja" } else { "nein" }
            );
            candidates.push((score, rest.start, rest.end, render_staff_voice_forward_fragment(rest.duration, &rest.staff, &rest.voice), scope_number, evidence, rest.duration.max(0)));
        }
    }
    candidates.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    candidates
}


fn lower_staff_visible_rest_lane_anchor_elision_permitted(
    existing_anchor_selector: bool,
    rest_start: i32,
    rest_duration: i32,
    same_lane_pitched_intervals: &[(i32, i32)],
) -> bool {
    if !existing_anchor_selector || rest_duration <= 0 {
        return false;
    }
    let candidate_has_bound_same_lane_rhythmic_proof = !same_lane_pitched_intervals.is_empty();
    if !candidate_has_bound_same_lane_rhythmic_proof {
        return false;
    }

    let rest_end = rest_start.saturating_add(rest_duration);
    if rest_end <= rest_start {
        return false;
    }
    let mut clipped = same_lane_pitched_intervals
        .iter()
        .filter_map(|(onset, duration)| {
            if *duration <= 0 {
                return None;
            }
            let start = (*onset).max(rest_start);
            let end = onset.saturating_add(*duration).min(rest_end);
            (end > start).then_some((start, end))
        })
        .collect::<Vec<_>>();
    clipped.sort_by_key(|(start, end)| (*start, *end));

    let mut covered_until = rest_start;
    for (start, end) in clipped {
        if start > covered_until {
            return false;
        }
        covered_until = covered_until.max(end);
        if covered_until >= rest_end {
            return true;
        }
    }
    false
}

fn lower_staff_visible_rest_lane_anchor_suppression_candidates(xml: &str) -> Vec<(i64, usize, usize, String, usize, String, i32)> {
    let mut candidates: Vec<(i64, usize, usize, String, usize, String, i32)> = Vec::new();
    for observation in collect_strict_staff_rest_lane_measure_observations(xml) {
        let scope_number = observation.scope;
        let fragments = &observation.fragments;
        let lower_visible_rests: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| frag.is_visible_rest && frag.staff.trim() == "2" && !frag.is_chord && frag.duration > 0)
            .cloned()
            .collect();
        if lower_visible_rests.is_empty() {
            continue;
        }
        let lower_visible_pitched: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && frag.staff.trim() == "2" && frag.duration > 0)
            .cloned()
            .collect();
        if lower_visible_pitched.is_empty() {
            continue;
        }
        let voices_with_visible_pitched: BTreeSet<String> = fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && frag.duration > 0)
            .map(|frag| frag.voice.clone())
            .collect();
        let measure_has_cross_staff_voice = voices_with_visible_pitched.iter().any(|voice| {
            let has_upper = fragments.iter().any(|frag| frag.is_visible_pitched && frag.voice == *voice && frag.staff.trim() == "1");
            let has_lower = fragments.iter().any(|frag| frag.is_visible_pitched && frag.voice == *voice && frag.staff.trim() == "2");
            has_upper && has_lower
        });
        let rest_voice_count_by_voice: BTreeMap<String, usize> = lower_visible_rests.iter().fold(BTreeMap::new(), |mut acc, rest| {
            *acc.entry(rest.voice.clone()).or_insert(0) += 1;
            acc
        });
        for rest in &lower_visible_rests {
            let rest_voice_has_visible_pitched = voices_with_visible_pitched.contains(&rest.voice);
            let other_lower_pitched = lower_visible_pitched.iter().any(|note| note.voice != rest.voice);
            let same_lane_pitched_intervals = lower_visible_pitched
                .iter()
                .filter(|note| note.voice == rest.voice)
                .map(|note| (note.onset, note.duration.max(0)))
                .collect::<Vec<_>>();
            let same_voice_lower_pitched = !same_lane_pitched_intervals.is_empty();
            let pure_anchor_lane = !rest_voice_has_visible_pitched && other_lower_pitched;
            let auxiliary_anchor_lane = rest.voice.trim() != "1" && other_lower_pitched;
            let repeated_rest_lane = *rest_voice_count_by_voice.get(&rest.voice).unwrap_or(&0) >= 2 && other_lower_pitched;
            let near_lower_pitched = lower_visible_pitched.iter().any(|note| {
                let note_start = note.onset;
                let note_end = note.onset.saturating_add(note.duration.max(0));
                let rest_start = rest.onset;
                let rest_end = rest.onset.saturating_add(rest.duration.max(0));
                let strict_overlap = rest_start < note_end && note_start < rest_end;
                let same_visual_bucket = lower_staff_rest_visual_cluster_key(rest_start) == lower_staff_rest_visual_cluster_key(note_start);
                let near_onset = (rest_start - note_start).abs() <= 24;
                strict_overlap || same_visual_bucket || near_onset
            });
            let existing_anchor_selector = (near_lower_pitched
                || repeated_rest_lane
                || measure_has_cross_staff_voice)
                && (auxiliary_anchor_lane || pure_anchor_lane)
                || (!same_voice_lower_pitched
                    && rest.voice.trim() != "1"
                    && measure_has_cross_staff_voice
                    && other_lower_pitched);
            if !lower_staff_visible_rest_lane_anchor_elision_permitted(
                existing_anchor_selector,
                rest.onset,
                rest.duration.max(0),
                &same_lane_pitched_intervals,
            ) {
                continue;
            }
            let pure_bonus = if pure_anchor_lane { 3200 } else { 0 };
            let aux_bonus = if auxiliary_anchor_lane { 1400 } else { 0 };
            let repeated_bonus = if repeated_rest_lane { 700 } else { 0 };
            let near_bonus = if near_lower_pitched { 500 } else { 0 };
            let cross_staff_bonus = if measure_has_cross_staff_voice { 450 } else { 0 };
            let short_bonus = if rest.duration <= 96 { 250 } else { 0 };
            let score = pure_bonus
                + aux_bonus
                + repeated_bonus
                + near_bonus
                + cross_staff_bonus
                + short_bonus
                + rest.duration.max(0) as i64
                + (scope_number as i64 / 1000);
            let evidence = format!(
                "scope {} staff {} voice {} onset {} duration {} pure_anchor_lane {} auxiliary_anchor_lane {} repeated_rest_lane {} near_lower_pitch {} cross_staff_voice {}",
                scope_number,
                rest.staff,
                rest.voice,
                rest.onset,
                rest.duration,
                if pure_anchor_lane { "ja" } else { "nein" },
                if auxiliary_anchor_lane { "ja" } else { "nein" },
                if repeated_rest_lane { "ja" } else { "nein" },
                if near_lower_pitched { "ja" } else { "nein" },
                if measure_has_cross_staff_voice { "ja" } else { "nein" }
            );
            candidates.push((
                score,
                rest.start,
                rest.end,
                render_staff_voice_forward_fragment(rest.duration, &rest.staff, &rest.voice),
                scope_number,
                evidence,
                rest.duration.max(0),
            ));
        }
    }
    candidates.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    candidates
}

fn fmt_upper_staff_rest_scope_set(scopes: BTreeSet<usize>) -> String {
    if scopes.is_empty() {
        "-".to_string()
    } else {
        scopes.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(",")
    }
}

fn upper_staff_local_visual_rest_ledger(xml: &str, max_rows: usize) -> String {
    let mut rows: Vec<(i64, usize, String)> = Vec::new();
    for observation in collect_strict_staff_rest_lane_measure_observations(xml) {
        let scope_number = observation.scope;
        let expected = observation.expected;
        let fragments = &observation.fragments;
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
        upper_rests.sort_by_key(|frag| (frag.onset, frag.voice.clone(), frag.start));
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
        let first_upper_onset = upper_first.map_or(0, |frag| frag.onset);
        let existing_initial = upper_first.is_some_and(|first| {
                upper_rests.iter().any(|rest| {
                    rest.onset <= 0 && rest.onset.saturating_add(rest.duration.max(0)) >= first.onset
                })
            },
        );
        let initial_status = if upper_first.is_none() {
            "no_upper_notes".to_string()
        } else if first_upper_onset == 0 {
            "upper_starts_at_zero".to_string()
        } else if existing_initial {
            format!("covered_0-{first_upper_onset}")
        } else {
            format!("missing_0-{first_upper_onset}")
        };

        let rest_intervals = if upper_rests.is_empty() {
            "-".to_string()
        } else {
            upper_rests
                .iter()
                .map(|rest| format!("v{}:{}-{}", rest.voice.trim(), rest.onset, rest.onset.saturating_add(rest.duration.max(0))))
                .collect::<Vec<_>>()
                .join("/")
        };

        let mut duplicate_groups = Vec::<String>::new();
        let mut by_identity: BTreeMap<(String, i32, i32), usize> = BTreeMap::new();
        for rest in &upper_rests {
            *by_identity.entry((rest.voice.clone(), rest.onset, rest.duration.max(0))).or_insert(0) += 1;
        }
        for ((voice, onset, duration), count) in by_identity {
            if count > 1 {
                duplicate_groups.push(format!("v{}:{}-{}x{}", voice.trim(), onset, onset.saturating_add(duration), count));
            }
        }

        let mut coalesce_ok = Vec::<String>::new();
        let mut cross_beat_blocked = Vec::<String>::new();
        let mut by_voice: BTreeMap<String, Vec<RenderXmlTimelineFragmentSpan>> = BTreeMap::new();
        for rest in &upper_rests {
            by_voice.entry(rest.voice.clone()).or_default().push(rest.clone());
        }
        for (voice, mut rests) in by_voice {
            rests.sort_by_key(|rest| (rest.onset, rest.start));
            for pair in rests.windows(2) {
                let first = &pair[0];
                let second = &pair[1];
                let contiguous = first.onset.saturating_add(first.duration.max(0)) == second.onset;
                let both_sixteenth = first.duration == 24 && second.duration == 24;
                if !contiguous || !both_sixteenth {
                    continue;
                }
                let total = first.duration.saturating_add(second.duration);
                let item = format!(
                    "v{}:{}-{}+{}-{}",
                    voice.trim(),
                    first.onset,
                    first.onset.saturating_add(first.duration.max(0)),
                    second.onset,
                    second.onset.saturating_add(second.duration.max(0))
                );
                if duration_span_is_inside_single_beat(first.onset, total) {
                    coalesce_ok.push(item);
                } else {
                    cross_beat_blocked.push(item);
                }
            }
        }

        let has_issue = first_upper_onset > 0
            || !duplicate_groups.is_empty()
            || !coalesce_ok.is_empty()
            || !cross_beat_blocked.is_empty();
        if has_issue {
            let score = (duplicate_groups.len() as i64 * 1200)
                + (cross_beat_blocked.len() as i64 * 1000)
                + (if first_upper_onset > 0 && !existing_initial { 800 } else { 0 })
                + (if first_upper_onset > 0 && existing_initial { 300 } else { 0 })
                + (coalesce_ok.len() as i64 * 400)
                + upper_rests.len() as i64;
            let duplicates = if duplicate_groups.is_empty() { "-".to_string() } else { duplicate_groups.join("/") };
            let ok_pairs = if coalesce_ok.is_empty() { "-".to_string() } else { coalesce_ok.join("/") };
            let blocked_pairs = if cross_beat_blocked.is_empty() { "-".to_string() } else { cross_beat_blocked.join("/") };
            rows.push((score, scope_number, format!(
                "scope {scope_number} expected {expected} first_upper {first_upper_onset} initial {initial_status} rests [{rest_intervals}] duplicates [{duplicates}] coalesce_ok [{ok_pairs}] crossbeat_blocked [{blocked_pairs}]"
            )));
        }
    }
    rows.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    if rows.is_empty() {
        "-".to_string()
    } else {
        rows.into_iter().take(max_rows).map(|(_, _, row)| row).collect::<Vec<_>>().join(" | ")
    }
}

fn upper_staff_duplicate_visible_rest_suppression_candidates(xml: &str) -> Vec<UpperStaffDuplicateVisibleRestSuppressionCandidate> {
    let mut candidates: Vec<UpperStaffDuplicateVisibleRestSuppressionCandidate> = Vec::new();
    for observation in collect_strict_staff_rest_lane_measure_observations(xml) {
        let scope_number = observation.scope;
        let fragments = &observation.fragments;
        let mut by_identity: BTreeMap<(String, i32, i32), Vec<RenderXmlTimelineFragmentSpan>> = BTreeMap::new();
        for frag in fragments.iter().filter(|frag| {
            frag.staff.trim() == "1"
                && frag.is_visible_rest
                && !frag.is_chord
                && !frag.is_hidden
                && !frag.has_visible_dependency
                && frag.duration > 0
        }) {
            by_identity
                .entry((frag.voice.clone(), frag.onset, frag.duration.max(0)))
                .or_default()
                .push(frag.clone());
        }
        let upper_visible_pitched = fragments.iter().filter(|frag| frag.staff.trim() == "1" && frag.is_visible_pitched && frag.duration > 0).count();
        for ((voice, onset, duration), mut rests) in by_identity {
            if rests.len() < 2 || duration <= 0 {
                continue;
            }
            rests.sort_by_key(|rest| rest.start);
            for (duplicate_index, rest) in rests.iter().enumerate().skip(1) {
                let score = 5000_i64
                    + (rests.len() as i64 * 200)
                    + if upper_visible_pitched > 0 { 150 } else { 0 }
                    + if duration <= 48 { 90 } else { 0 }
                    + (scope_number as i64 / 1000);
                let evidence = format!(
                    "scope {} staff 1 voice {} duplicate_index {} onset {} duration {} group_count {} upper_visible_pitched {}",
                    scope_number,
                    voice.trim(),
                    duplicate_index + 1,
                    onset,
                    duration,
                    rests.len(),
                    upper_visible_pitched
                );
                let interval_proof = if rests.len() == 2 {
                    format!(
                        "scope {} staff1 voice{} duplicate_visible_rest_interval {}-{}: 2→1",
                        scope_number,
                        voice.trim(),
                        onset,
                        onset.saturating_add(duration)
                    )
                } else {
                    format!(
                        "scope {} staff1 voice{} duplicate_visible_rest_interval {}-{}: group_count {} duplicate_index {}→forward",
                        scope_number,
                        voice.trim(),
                        onset,
                        onset.saturating_add(duration),
                        rests.len(),
                        duplicate_index + 1
                    )
                };
                candidates.push((
                    score,
                    rest.start,
                    rest.end,
                    render_staff_voice_forward_fragment(duration, "1", &voice),
                    scope_number,
                    evidence,
                    duration,
                    interval_proof,
                ));
            }
        }
    }
    candidates.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    candidates
}

#[cfg(test)]
mod lower_staff_visible_rest_lane_anchor_bound_obligation_tests {
    use super::lower_staff_visible_rest_lane_anchor_elision_permitted;

    #[test]
    fn partial_or_external_same_lane_sound_does_not_dispense_complete_visible_rest_interval() {
        assert!(!lower_staff_visible_rest_lane_anchor_elision_permitted(
            true,
            0,
            96,
            &[(96, 24), (144, 24)],
        ));
        assert!(!lower_staff_visible_rest_lane_anchor_elision_permitted(
            true,
            0,
            96,
            &[(0, 48), (72, 24)],
        ));
    }

    #[test]
    fn complete_same_lane_coverage_or_proven_technical_lane_preserves_existing_anchor_selection() {
        assert!(lower_staff_visible_rest_lane_anchor_elision_permitted(
            true,
            12,
            80,
            &[(52, 40), (12, 40)],
        ));
        assert!(!lower_staff_visible_rest_lane_anchor_elision_permitted(
            true,
            12,
            80,
            &[],
        ));
        assert!(!lower_staff_visible_rest_lane_anchor_elision_permitted(
            false,
            12,
            80,
            &[(12, 80)],
        ));
    }
}

