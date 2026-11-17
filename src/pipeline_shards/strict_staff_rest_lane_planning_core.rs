// Writer-free strict-staff lower/upper rest-lane planning.

#[derive(Debug, Clone)]
struct StrictStaffRestLaneMeasureObservation {
    measure_start: usize,
    scope: usize,
    expected: i32,
    beat_unit: i32,
    fragments: Vec<RenderXmlTimelineFragmentSpan>,
}

fn collect_strict_staff_rest_lane_measure_observations(
    xml: &str,
) -> Vec<StrictStaffRestLaneMeasureObservation> {
    let mut observations = Vec::new();
    for (measure_start, measure_end, scope_number, (expected, beat_unit)) in render_measure_metric_entries(xml) {
        let measure = &xml[measure_start..measure_end];
        observations.push(StrictStaffRestLaneMeasureObservation {
            measure_start,
            scope: scope_number,
            expected,
            beat_unit,
            fragments: collect_render_timeline_fragments_in_segment(
                measure,
                measure_start,
                scope_number,
            ),
        });
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

fn lower_staff_visual_time_relation(a: i32, a_duration: i32, b: i32, b_duration: i32, beat: i32) -> bool {
    let (a, b, beat) = (i64::from(a), i64::from(b), i64::from(beat));
    let strict_overlap = a < b + i64::from(b_duration) && b < a + i64::from(a_duration);
    if beat <= 0 { return strict_overlap; }
    let same_half_beat = a.max(0) * 2 / beat == b.max(0) * 2 / beat;
    let near_onset = (a - b).abs() * 4 <= beat;
    strict_overlap || same_half_beat || near_onset
}

fn lower_staff_rest_collision_relation(
    a: &RenderXmlTimelineFragmentSpan, b: &RenderXmlTimelineFragmentSpan, beat: i32,
) -> bool {
    a.voice != b.voice
        && lower_staff_visual_time_relation(a.onset, a.duration, b.onset, b.duration, beat)
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
                if lower_staff_rest_collision_relation(a, b, observation.beat_unit) {
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

#[derive(Debug, PartialEq, Eq)]
struct LowerStaffRestCandidate {
    score: i64,
    start: usize,
    end: usize,
    replacement: String,
    measure_start: usize,
    scope: usize,
    evidence: String,
    duration: i32,
}

// Candidate lists are ranked before selection. Independent measures share
// the caller's overall limit, while each has its own local limit.
fn select_lower_staff_rest_candidates(
    candidates: Vec<LowerStaffRestCandidate>,
    max_actions: usize,
    max_per_measure: usize,
) -> Vec<LowerStaffRestCandidate> {
    let mut per_measure_selected = BTreeMap::<usize, usize>::new();
    let mut selected: Vec<LowerStaffRestCandidate> = Vec::new();
    for candidate in candidates {
        if selected.len() >= max_actions {
            break;
        }
        let count = per_measure_selected.entry(candidate.measure_start).or_default();
        if *count >= max_per_measure || selected.iter().any(|prior| {
            candidate.start < prior.end && candidate.end > prior.start
        }) {
            continue;
        }
        *count += 1;
        selected.push(candidate);
    }
    selected
}

// Keep the established score weight of one beat while deriving the duration
// contribution from the actual local meter and XML divisions.
fn lower_staff_rest_duration_score(duration: i32, beat_unit: i32) -> i64 {
    const SCORE_POINTS_PER_BEAT: i64 = 96;
    let short_bonus = if duration <= beat_unit { 250 } else { 0 };
    short_bonus + i64::from(duration) * SCORE_POINTS_PER_BEAT / i64::from(beat_unit)
}

fn lower_staff_overlapping_rest_suppression_candidates(xml: &str) -> Vec<LowerStaffRestCandidate> {
    let mut candidates = Vec::new();
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
        let lower_visible_pitched: Vec<RenderXmlTimelineFragmentSpan> = fragments
            .iter()
            .filter(|frag| frag.is_visible_pitched && frag.staff.trim() == "2" && frag.duration > 0)
            .cloned()
            .collect();

        for rest in &lower_visible_rests {
            let rest_start = rest.onset;
            let rest_end = rest.onset.saturating_add(rest.duration.max(0));
            if rest_end <= rest_start {
                continue;
            }
            let overlaps_other_rest = lower_visible_rests.iter().any(|other| {
                other.start != rest.start && lower_staff_rest_collision_relation(rest, other, observation.beat_unit)
            });
            let overlaps_lower_pitched_other_voice = lower_visible_pitched.iter().any(|other| {
                other.voice != rest.voice
                    && rest_start < other.onset.saturating_add(other.duration.max(0))
                    && other.onset < rest_end
            });
            if !overlaps_other_rest && !overlaps_lower_pitched_other_voice {
                continue;
            }

            // Do not suppress the only primary rest in an otherwise empty lower staff.
            // Prefer auxiliary voices, short/clustered visible rests, and rests that collide
            // with a lower pitched voice. This is deliberately generic: no fixed measure.
            let aux_voice_bonus = if rest.voice.trim() != "1" { 2000 } else { 0 };
            let pitched_bonus = if overlaps_lower_pitched_other_voice { 900 } else { 0 };
            let rest_collision_bonus = if overlaps_other_rest { 700 } else { 0 };
            let score = aux_voice_bonus
                + pitched_bonus
                + rest_collision_bonus
                + lower_staff_rest_duration_score(rest.duration, observation.beat_unit);
            let evidence = format!(
                "scope {} staff {} voice {} onset {} duration {} overlap_rest_or_same_beat {} overlap_pitch {}",
                scope_number,
                rest.staff,
                rest.voice,
                rest.onset,
                rest.duration,
                if overlaps_other_rest { "ja" } else { "nein" },
                if overlaps_lower_pitched_other_voice { "ja" } else { "nein" }
            );
            candidates.push(LowerStaffRestCandidate {
                score,
                start: rest.start,
                end: rest.end,
                replacement: render_staff_voice_forward_fragment(rest.duration, &rest.staff, &rest.voice),
                measure_start: observation.measure_start,
                scope: scope_number,
                evidence,
                duration: rest.duration,
            });
        }
    }
    candidates.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.start.cmp(&b.start)));
    candidates
}


fn pitched_intervals_cover_rest(
    rest_start: i32,
    rest_duration: i32,
    pitched_intervals: &[(i32, i32)],
) -> bool {
    let Some(rest_end) = rest_start.checked_add(rest_duration) else {
        return false;
    };
    if rest_start < 0 || rest_end <= rest_start || pitched_intervals.is_empty() {
        return false;
    }
    let mut clipped = pitched_intervals
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

fn lower_staff_visible_rest_lane_anchor_suppression_candidates(xml: &str) -> Vec<LowerStaffRestCandidate> {
    let mut candidates = Vec::new();
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
                lower_staff_visual_time_relation(
                    rest.onset, rest.duration, note.onset, note.duration, observation.beat_unit,
                )
            });
            let existing_anchor_selector = (near_lower_pitched
                || repeated_rest_lane
                || measure_has_cross_staff_voice)
                && (auxiliary_anchor_lane || pure_anchor_lane)
                || (!same_voice_lower_pitched
                    && rest.voice.trim() != "1"
                    && measure_has_cross_staff_voice
                    && other_lower_pitched);
            if !existing_anchor_selector || !pitched_intervals_cover_rest(
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
            let score = pure_bonus
                + aux_bonus
                + repeated_bonus
                + near_bonus
                + cross_staff_bonus
                + lower_staff_rest_duration_score(rest.duration, observation.beat_unit);
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
            candidates.push(LowerStaffRestCandidate {
                score,
                start: rest.start,
                end: rest.end,
                replacement: render_staff_voice_forward_fragment(rest.duration, &rest.staff, &rest.voice),
                measure_start: observation.measure_start,
                scope: scope_number,
                evidence,
                duration: rest.duration,
            });
        }
    }
    candidates.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.start.cmp(&b.start)));
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
        let beat_unit = observation.beat_unit;
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
                if !contiguous {
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
                if !duration_span_is_inside_single_beat(first.onset, total, beat_unit) {
                    cross_beat_blocked.push(item);
                } else if crate::model::metric_rest_notation::plan_partial_visible_rest_durations(
                    i64::from(first.onset), i64::from(total), i64::from(expected), i64::from(beat_unit),
                ).as_slice() == [i64::from(total)] {
                    coalesce_ok.push(item);
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
                    + if duration <= observation.beat_unit / 2 { 90 } else { 0 };
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
                    observation.measure_start,
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
    use super::pitched_intervals_cover_rest;

    #[test]
    fn partial_or_external_same_lane_sound_does_not_dispense_complete_visible_rest_interval() {
        assert!(!pitched_intervals_cover_rest(
            0,
            96,
            &[(96, 24), (144, 24)],
        ));
        assert!(!pitched_intervals_cover_rest(
            0,
            96,
            &[(0, 48), (72, 24)],
        ));
    }

    #[test]
    fn complete_same_lane_coverage_requires_bound_intervals() {
        assert!(pitched_intervals_cover_rest(
            12,
            80,
            &[(52, 40), (12, 40)],
        ));
        assert!(!pitched_intervals_cover_rest(
            12,
            80,
            &[],
        ));
        assert!(!pitched_intervals_cover_rest(12, 80, &[(12, 48), (20, 48)]));
    }
}



#[cfg(test)]
mod lower_staff_visual_time_tests {
    use super::lower_staff_visual_time_relation;

    #[test]
    fn visual_neighborhood_depends_on_relative_beat_geometry() {
        for scale in [1, 2, 3] {
            assert!(lower_staff_visual_time_relation(0, 6 * scale, 24 * scale, 6 * scale, 96 * scale));
            assert!(!lower_staff_visual_time_relation(0, 6 * scale, 48 * scale, 6 * scale, 96 * scale));
            assert!(lower_staff_visual_time_relation(0, 75 * scale, 72 * scale, 6 * scale, 96 * scale));
        }
    }
}


#[cfg(test)]
mod duplicate_rest_label_tests {
    use super::upper_staff_duplicate_visible_rest_suppression_candidates;

    #[test]
    fn relabeling_measures_cannot_change_duplicate_rest_priority() {
        let rest = "<note><rest/><duration>24</duration><voice>1</voice><type>16th</type><staff>1</staff></note>";
        let measure = |label| format!(
            r#"<measure number="{label}">{rest}<backup><duration>24</duration></backup>{rest}</measure>"#,
        );
        let candidates = |a, b| {
            let xml = format!("<score-partwise><part>{}{}</part></score-partwise>", measure(a), measure(b));
            upper_staff_duplicate_visible_rest_suppression_candidates(&xml)
                .into_iter().map(|row| row.0).collect::<Vec<_>>()
        };
        let original = candidates(10_001, 11);
        assert_eq!(original.len(), 2);
        assert_eq!(original, candidates(11, 10_001));
        assert_eq!(original[0], original[1]);
    }
}


#[cfg(test)]
mod overlapping_rest_locality_tests {
    use super::lower_staff_overlapping_rest_suppression_candidates;

    #[test]
    fn remote_pitch_and_rest_do_not_license_local_rest_suppression() {
        let rest = |voice| format!(
            "<note><rest/><duration>24</duration><voice>{voice}</voice><type>16th</type><staff>2</staff></note>"
        );
        let pitch = "<note><pitch><step>C</step><octave>3</octave></pitch><duration>24</duration><voice>1</voice><type>16th</type><staff>2</staff></note>";
        let label = 1 + pitch.len();
        let gap = "<forward><duration>192</duration></forward>";
        let rewind = "<backup><duration>24</duration></backup>";
        let score = |motion, remote_rest: &str| format!(
            "<score-partwise><part><measure number=\"{label}\">{}{motion}{pitch}{gap}{remote_rest}</measure></part></score-partwise>",
            rest(2),
        );
        assert!(lower_staff_overlapping_rest_suppression_candidates(&score(gap, &rest(1))).is_empty());
        let nearby = lower_staff_overlapping_rest_suppression_candidates(&score(rewind, &rest(1)));
        assert_eq!(nearby.len(), 1);
        assert!(nearby[0].evidence.contains("overlap_pitch ja"));
        assert_eq!(nearby, lower_staff_overlapping_rest_suppression_candidates(&score(rewind, "")));
    }
}


#[cfg(test)]
mod rest_lane_metric_extent_tests {
    use super::collect_strict_staff_rest_lane_measure_observations;

    #[test]
    fn sparse_regular_measure_inherits_meter_and_pickup_uses_its_extent() {
        for unit in [20, 32] {
            let (first, regular, pickup) = (unit, unit + 2, unit + 5);
            let xml = format!(
                "<score-partwise><part>                 <measure number=\"{first}\"><attributes><divisions>{unit}</divisions>                 <time><beats>3</beats><beat-type>8</beat-type></time></attributes></measure>                 <measure number=\"{regular}\"><note><rest/><duration>{}</duration></note></measure>                 <measure number=\"{pickup}\" implicit=\"yes\"><note><rest/><duration>{}</duration></note></measure>                 </part></score-partwise>",
                unit / 2, unit / 2,
            );
            let observations = collect_strict_staff_rest_lane_measure_observations(&xml);
            assert_eq!(observations.len(), 3);
            assert_eq!(observations[1].expected, 3 * unit / 2);
            assert_eq!(observations[1].beat_unit, unit / 2);
            assert_eq!(observations[2].expected, unit / 2);
        }
    }
}


#[cfg(test)]
mod rest_priority_metric_tests {
    use super::{
        lower_staff_overlapping_rest_suppression_candidates,
        lower_staff_visible_rest_lane_anchor_suppression_candidates,
        upper_staff_duplicate_visible_rest_suppression_candidates,
    };

    #[test]
    fn rest_candidate_order_and_scores_survive_scaling_in_both_staves() {
        let fixture = |scale| {
            let beat = 96 * scale;
            let long = 108 * scale;
            let short = 36 * scale;
            let span = long + short;
            let label = 14 * scale;
            let rest = |duration, staff, voice| format!(
                "<note><rest/><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>"
            );
            let backup = |duration| format!("<backup><duration>{duration}</duration></backup>");
            let pitch = |voice| format!(
                "<note><pitch><step>C</step><octave>3</octave></pitch><duration>{span}</duration><voice>{voice}</voice><staff>2</staff></note>"
            );
            let upper = [rest(long, 1, 5), rest(short, 1, 5)].concat();
            format!(
                r#"<score-partwise><part><measure number="{label}"><attributes><divisions>{beat}</divisions></attributes>{upper}{b}{upper}{b}{}{b}{}{b}{}</measure></part></score-partwise>"#,
                pitch(7), pitch(8), [rest(long, 2, 7), rest(short, 2, 7)].concat(), b = backup(span),
            )
        };
        let scores = |scale| {
            let xml = fixture(scale);
            let upper = upper_staff_duplicate_visible_rest_suppression_candidates(&xml)
                .into_iter().map(|row| (row.0, row.6 / scale)).collect::<Vec<_>>();
            let overlap = lower_staff_overlapping_rest_suppression_candidates(&xml)
                .into_iter().map(|row| (row.score, row.duration / scale)).collect::<Vec<_>>();
            let anchor = lower_staff_visible_rest_lane_anchor_suppression_candidates(&xml)
                .into_iter().map(|row| (row.score, row.duration / scale)).collect::<Vec<_>>();
            assert_eq!((upper.len(), overlap.len(), anchor.len()), (2, 2, 2));
            (upper, overlap, anchor)
        };
        assert_eq!(scores(1), scores(2));
        assert_eq!(scores(1), scores(3));
    }
}
