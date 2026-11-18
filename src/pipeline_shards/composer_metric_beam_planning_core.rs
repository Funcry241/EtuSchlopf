// Pure metric-beam grouping and plan construction. Productive writer/audit ownership stays in the caller shard.

fn add_beat_scoped_beam_groups(
    note_infos: &[PartialBeatNoteInfo],
    beat_unit: i64,
    clef_boundaries: &BTreeMap<String, Vec<i64>>,
    states_by_note: &mut BTreeMap<usize, Vec<(usize, &'static str)>>,
    explicit_negative_notes: &mut BTreeSet<usize>,
    stats: &mut BeatScopedBeamPatchStats,
) {
    let mut groups: BTreeMap<(String, String, i64), Vec<&PartialBeatNoteInfo>> = BTreeMap::new();
    for info in note_infos {
        if partial_beat_is_beam_candidate(info) {
            groups
                .entry((info.staff.clone(), info.voice.clone(), info.start.div_euclid(beat_unit)))
                .or_default()
                .push(info);
        }
    }
    let mut used_notes: BTreeSet<usize> = BTreeSet::new();
    let written_courses = groups.into_iter().flat_map(|((staff, _, _), candidates)| {
        let split = independent_metric_beam_cursor_courses(&candidates);
        let separate = split.is_some();
        split.unwrap_or_else(|| vec![candidates]).into_iter()
            .map(move |course| (staff.clone(), course, separate))
    });
    for (staff, mut candidates, separate) in written_courses {
        let boundaries = clef_boundaries.get(&staff).map_or(&[][..], Vec::as_slice);
        candidates.sort_by_key(|info| (info.start, info.index));
        let mut runs: Vec<Vec<&PartialBeatNoteInfo>> = Vec::new();
        let mut current: Vec<&PartialBeatNoteInfo> = Vec::new();
        let mut previous_end: Option<i64> = None;
        for info in &candidates {
            if current.last().is_none_or(|left: &&PartialBeatNoteInfo| {
                previous_end == Some(info.start)
                    && !crosses_metric_beam_clef_boundary(boundaries, left.start, info.start)
            }) {
                current.push(*info);
            } else {
                runs.push(current);
                current = vec![*info];
            }
            previous_end = Some(info.start + info.duration);
        }
        if !current.is_empty() {
            runs.push(current);
        }
        let mut accepted_any = false;
        let mut selected_in_group = BTreeSet::new();
        for run in runs {
            if run.len() >= 2 && beat_group_stays_inside_one_beat(&run, beat_unit) {
                let fresh = run.iter().filter(|info| !used_notes.contains(&info.index)).count();
                if let Some(emission) = partial_beat_extend_states_with_actual_capture(&run, states_by_note) {
                    stats.actual_beam_emissions.push(emission);
                    for info in &run {
                        used_notes.insert(info.index);
                        selected_in_group.insert(info.index);
                    }
                    stats.groups += 1;
                    stats.notes += fresh;
                    accepted_any = true;
                }
            }
        }
        if !separate {
            // A rest edge can join two otherwise unclaimed attacks. A complete
            // sounding run already owns its endpoints. An attack with two
            // eligible neighbours does not uniquely determine a rest edge.
            let rest_edges = candidates.windows(2).filter(|edge| {
                let [left, right] = [edge[0], edge[1]];
                !selected_in_group.contains(&left.index)
                    && !selected_in_group.contains(&right.index)
                    && beat_group_stays_inside_one_beat(edge, beat_unit)
                    && !crosses_metric_beam_clef_boundary(boundaries, left.start, right.start)
                    && left.start + left.duration < right.start
                    && visible_rests_cover_gap(note_infos, left, right)
            }).collect::<Vec<_>>();
            let mut degree = BTreeMap::<usize, usize>::new();
            for note in rest_edges.iter().flat_map(|edge| edge.iter()) {
                *degree.entry(note.index).or_default() += 1;
            }
            for edge in rest_edges.iter().filter(|edge| edge.iter().all(|note| degree[&note.index] == 1)) {
                let fresh = edge.iter().filter(|info| !used_notes.contains(&info.index)).count();
                if let Some(emission) = partial_beat_extend_states_with_actual_capture(edge, states_by_note) {
                    stats.actual_beam_emissions.push(emission);
                    for info in *edge {
                        used_notes.insert(info.index);
                        selected_in_group.insert(info.index);
                    }
                    stats.groups += 1;
                    stats.notes += fresh;
                    accepted_any = true;
                }
            }
        }

        // v0.10.1779 BeatScopedBeamDecisionScopeMaterializationClosure:
        // once this owner accepts at least one run, the complete same-Staff/Voice/Beat
        // candidate group is its decision scope. Candidate notes not selected into a
        // positive run are explicit negative Beam decisions; otherwise an older chain
        // may survive as a stale prefix after only part of that chain is rewritten.
        if accepted_any {
            for info in candidates {
                if !selected_in_group.contains(&info.index) {
                    explicit_negative_notes.insert(info.index);
                    stats.actual_beam_emissions.push(ActualBeamEmission::negative(info.index));
                }
            }
        }
    }
}

// A full rewind gives a distinct serialized course, not a musical voice label.
// Use it only when overlapping positive intervals prove that combining courses
// under the provisional label would give the planner an invalid linear timeline.
fn independent_metric_beam_cursor_courses<'a>(
    candidates: &[&'a PartialBeatNoteInfo],
) -> Option<Vec<Vec<&'a PartialBeatNoteInfo>>> {
    let mut courses = BTreeMap::<usize, Vec<_>>::new();
    for &note in candidates {
        if note.existing_beams != 0 {
            return None;
        }
        courses.entry(note.cursor_run?).or_default().push(note);
    }
    let overlap = candidates.iter().enumerate().any(|(index, left)| {
        candidates[index + 1..].iter().any(|right| {
            left.cursor_run != right.cursor_run
                && left.start < right.start + right.duration
                && right.start < left.start + left.duration
        })
    });
    overlap.then(|| courses.into_values().collect())
}

fn crosses_metric_beam_clef_boundary(boundaries: &[i64], left: i64, right: i64) -> bool {
    boundaries.iter().any(|&boundary| left < boundary && boundary <= right)
}

fn beat_group_stays_inside_one_beat(group: &[&PartialBeatNoteInfo], beat_unit: i64) -> bool {
    let Some(first) = group.first().copied() else { return false; };
    let beat = first.start.div_euclid(beat_unit);
    let beat_end = (beat + 1) * beat_unit;
    group.iter().all(|info| info.start.div_euclid(beat_unit) == beat && info.start + info.duration <= beat_end)
}

fn visible_rests_cover_gap(note_infos: &[PartialBeatNoteInfo], left: &PartialBeatNoteInfo, right: &PartialBeatNoteInfo) -> bool {
    let mut cursor = left.start + left.duration;
    let mut rests = note_infos
        .iter()
        .filter(|info| {
            info.staff == left.staff
                && info.voice == left.voice
                && info.is_rest
                && !info.is_hidden
                && !info.is_chord
                && info.start >= cursor
                && info.start < right.start
                && info.duration > 0
        })
        .collect::<Vec<_>>();
    rests.sort_by_key(|info| (info.start, info.index));
    for rest in rests {
        if rest.start != cursor {
            return false;
        }
        cursor = rest.start + rest.duration;
        if cursor > right.start {
            return false;
        }
    }
    cursor == right.start
}

fn partial_beat_group_needs_post_rest_beam(note_infos: &[PartialBeatNoteInfo], group: &[&PartialBeatNoteInfo], beat_unit: i64) -> bool {
    if group.len() < 2 {
        return false;
    }
    let beat = group[0].start.div_euclid(beat_unit);
    let beat_start = beat * beat_unit;
    let beat_end = (beat + 1) * beat_unit;
    if group[0].start <= beat_start {
        return false;
    }
    if !partial_beat_leading_visible_rest_covers_gap(note_infos, group, beat_start) {
        return false;
    }
    let mut previous_end: Option<i64> = None;
    let mut last_start: Option<i64> = None;
    for info in group {
        if info.start.div_euclid(beat_unit) != beat {
            return false;
        }
        if info.start + info.duration > beat_end {
            return false;
        }
        if Some(info.start) == last_start {
            return false;
        }
        if let Some(end) = previous_end
            && info.start != end {
                return false;
            }
        previous_end = Some(info.start + info.duration);
        last_start = Some(info.start);
    }
    true
}

fn partial_beat_leading_visible_rest_covers_gap(note_infos: &[PartialBeatNoteInfo], group: &[&PartialBeatNoteInfo], beat_start: i64) -> bool {
    let Some(first) = group.first().copied() else { return false; };
    let mut rests = note_infos
        .iter()
        .filter(|info| {
            info.staff == first.staff
                && info.voice == first.voice
                && info.is_rest
                && !info.is_hidden
                && !info.is_chord
                && info.duration > 0
                && info.start >= beat_start
                && info.start < first.start
        })
        .collect::<Vec<_>>();
    rests.sort_by_key(|info| (info.start, info.index));
    let mut cursor = beat_start;
    for rest in rests {
        if rest.start != cursor {
            return false;
        }
        let end = rest.start + rest.duration;
        if end > first.start {
            return false;
        }
        cursor = end;
    }
    cursor == first.start
}

fn partial_beat_extend_states_from_plan(
    group: &[&PartialBeatNoteInfo],
    states_by_note: &mut BTreeMap<usize, Vec<(usize, &'static str)>>,
) -> Option<usize> {
    partial_beat_extend_states_with_actual_capture(group, states_by_note).map(|e|e.edits.len())
}

fn partial_beat_extend_states_with_actual_capture(
    group: &[&PartialBeatNoteInfo],
    states_by_note: &mut BTreeMap<usize, Vec<(usize, &'static str)>>,
) -> Option<ActualBeamEmission> {
    let plan = structural_metric_beam_hierarchy_plan(group, 4)?;
    for edit in plan.edits().iter().copied() {
        let info = group.get(edit.event_ordinal())?;
        states_by_note.entry(info.index).or_default().push((usize::from(edit.level()), edit.state().as_musicxml_text()));
    }
    Some(ActualBeamEmission::planned(group, &plan.plan))
}





fn structural_cross_staff_voice_beam_groups<'a>(
    infos: &'a [PartialBeatNoteInfo],
    beat_unit: i64,
    quarter: i64,
    clef_boundaries: &BTreeMap<String, Vec<i64>>,
) -> Vec<Vec<&'a PartialBeatNoteInfo>> {
    let mut by_course: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for info in infos.iter().filter(|info| !info.is_chord) {
        if let Some(cursor_run) = info.cursor_run {
            by_course.entry((info.voice.as_str(), cursor_run, info.start.div_euclid(beat_unit)))
                .or_default().push(info);
        }
    }
    let mut groups = Vec::new();
    for mut notes in by_course.into_values() {
        notes.sort_by_key(|info| (info.start, info.index));
        if notes.windows(2).any(|pair| pair[0].start + pair[0].duration > pair[1].start) {
            continue;
        }
        for beamable in notes.split(|info| {
            if !partial_beat_is_beam_candidate(info) { return true; }
            let base = quarter * 4;
            let scale = 1_i64 << partial_beat_level(info);
            // Accept ordinary and dotted binary values. Timing-only duration
            // coincidence is insufficient when the written value disagrees.
            ![base, base * 3 / 2, base * 7 / 4].contains(&(info.duration * scale * 4))
        }) {
            for run in beamable.chunk_by(|left, right| {
                left.start + left.duration == right.start
                    && ![&left.staff, &right.staff].iter().any(|staff| {
                        clef_boundaries.get(*staff).is_some_and(|boundaries| {
                            crosses_metric_beam_clef_boundary(boundaries, left.start, right.start)
                        })
                    })
            }) {
                let [first, ..] = run else { continue; };
                if beat_group_stays_inside_one_beat(run, beat_unit)
                    && run.iter().any(|note| note.staff != first.staff)
                    && run.windows(2).all(|pair| pair[0].index < pair[1].index)
                {
                    groups.push(run.to_vec());
                }
            }
        }
    }
    groups
}

const STRUCTURAL_METRIC_BEAM_HIERARCHY_OWNER_KIND: &str =
    "structural_metric_beam_hierarchy_plan";
const CANONICAL_PRIMARY_BEAM_OWNER_BOUNDARY_RECEIPT_SCHEMA_VERSION: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CanonicalPrimaryBeamOwnerBoundaryReceipt {
    schema_version: u8,
    owner_kind: &'static str,
    event_count: usize,
    highest_owned_level: u8,
    plan_fingerprint_fnv64: u64,
    primary_plan_fingerprint_fnv64: u64,
    primary_states: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuralMetricBeamHierarchyDecision {
    plan: structural_write_authority::BeamPlan,
    owner_receipt: CanonicalPrimaryBeamOwnerBoundaryReceipt,
}

impl std::ops::Deref for StructuralMetricBeamHierarchyDecision {
    type Target = structural_write_authority::BeamPlan;

    fn deref(&self) -> &Self::Target {
        let _ = &self.owner_receipt;
        &self.plan
    }
}

impl StructuralMetricBeamHierarchyDecision {
    fn into_plan(self) -> structural_write_authority::BeamPlan {
        let _ = &self.owner_receipt;
        self.plan
    }
}

fn structural_metric_beam_plan_fingerprint(
    plan: &structural_write_authority::BeamPlan,
    primary_only: bool,
) -> u64 {
    use std::fmt::Write as _;

    let mut material = String::new();
    for edit in plan.edits().iter().copied() {
        if primary_only && edit.level() != 1 {
            continue;
        }
        let _ = write!(
            material,
            "{}:{}:{};",
            edit.event_ordinal(),
            edit.level(),
            edit.state().as_musicxml_text()
        );
    }
    let mut state = 0xcbf29ce484222325u64;
    for byte in material.as_bytes() {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(0x100000001b3);
    }
    state
}

fn canonical_primary_beam_owner_boundary_receipt(
    event_count: usize,
    highest_owned_level: u8,
    plan: &structural_write_authority::BeamPlan,
) -> CanonicalPrimaryBeamOwnerBoundaryReceipt {
    let primary_states = plan
        .edits()
        .iter()
        .copied()
        .filter(|edit| edit.level() == 1)
        .map(|edit| edit.state().as_musicxml_text())
        .collect::<Vec<_>>();
    CanonicalPrimaryBeamOwnerBoundaryReceipt {
        schema_version: CANONICAL_PRIMARY_BEAM_OWNER_BOUNDARY_RECEIPT_SCHEMA_VERSION,
        owner_kind: STRUCTURAL_METRIC_BEAM_HIERARCHY_OWNER_KIND,
        event_count,
        highest_owned_level,
        plan_fingerprint_fnv64: structural_metric_beam_plan_fingerprint(plan, false),
        primary_plan_fingerprint_fnv64: structural_metric_beam_plan_fingerprint(plan, true),
        primary_states,
    }
}

fn structural_metric_beam_hierarchy_plan(
    group: &[&PartialBeatNoteInfo],
    highest_owned_level: u8,
) -> Option<StructuralMetricBeamHierarchyDecision> {
    if group.len() < 2 || !(1..=4).contains(&highest_owned_level)
        || group.windows(2).any(|pair| pair[0].index >= pair[1].index)
    {
        return None;
    }
    let primary = (0..group.len()).collect::<Vec<_>>();
    let mut level_groups = vec![(primary, 1_u8)];
    for level in 2_u8..=highest_owned_level {
        let mut run = Vec::new();
        for (ordinal, info) in group.iter().copied().enumerate() {
            if partial_beat_level(info) >= usize::from(level) {
                run.push(ordinal);
            } else {
                if run.len() >= 2 {
                    level_groups.push((std::mem::take(&mut run), level));
                }
                run.clear();
            }
        }
        if run.len() >= 2 {
            level_groups.push((run, level));
        }
    }
    let borrowed_groups = level_groups
        .iter()
        .map(|(ordinals, level)| (ordinals.as_slice(), *level))
        .collect::<Vec<_>>();
    let plan = structural_write_authority::BeamPlan::from_level_groups(group.len(), &borrowed_groups)?;
    let owner_receipt = canonical_primary_beam_owner_boundary_receipt(
        group.len(),
        highest_owned_level,
        &plan,
    );
    Some(StructuralMetricBeamHierarchyDecision {
        plan,
        owner_receipt,
    })
}


fn extend_beam_states_from_plan(
    states: &mut BTreeMap<usize, Vec<(usize, &'static str)>>,
    group: &[&PartialBeatNoteInfo],
    plan: &structural_write_authority::BeamPlan,
) -> bool {
    let proposed = plan
        .edits()
        .iter()
        .copied()
        .filter_map(|edit| {
            group.get(edit.event_ordinal()).map(|info| {
                (
                    info.index,
                    usize::from(edit.level()),
                    edit.state().as_musicxml_text(),
                )
            })
        })
        .collect::<Vec<_>>();
    if proposed.len() != plan.edits().len()
        || proposed.iter().any(|(index, level, state)| {
            states.get(index).is_some_and(|rows| {
                rows.iter().any(|(existing_level, existing_state)| {
                    existing_level == level && existing_state != state
                })
            })
        })
    {
        return false;
    }
    for (index, level, state) in proposed {
        let rows = states.entry(index).or_default();
        if !rows.iter().any(|(existing_level, existing_state)| {
            *existing_level == level && *existing_state == state
        }) {
            rows.push((level, state));
        }
    }
    true
}

fn metric_rest_connection_is_covered(
    infos: &[PartialBeatNoteInfo],
    left: &PartialBeatNoteInfo,
    right: &PartialBeatNoteInfo,
) -> bool {
    let end = left.start + left.duration;
    end == right.start || (end < right.start && visible_rests_cover_gap(infos, left, right))
}

fn choose_structural_rest_bridge_candidate_groups(
    infos: &[PartialBeatNoteInfo],
    beat_unit: i64,
) -> Vec<MetricRestContinuationGroup<'_>> {
    let mut groups: BTreeMap<(String, String, i64), Vec<&PartialBeatNoteInfo>> = BTreeMap::new();
    for info in infos {
        if info.staff != "2" || !partial_beat_is_beam_candidate(info) {
            continue;
        }
        groups
            .entry((
                info.staff.clone(),
                info.voice.clone(),
                info.start.div_euclid(beat_unit),
            ))
            .or_default()
            .push(info);
    }
    let mut candidates = Vec::new();
    for ((staff, voice, beat), mut notes) in groups {
        notes.sort_by_key(|info| (info.start, info.index));
        if notes.len() < 2 || !beat_group_stays_inside_one_beat(&notes, beat_unit)
            || notes.windows(2).any(|pair| pair[0].start + pair[0].duration > pair[1].start)
        {
            continue;
        }
        for run in notes.chunk_by(|left, right| metric_rest_connection_is_covered(infos, left, right)) {
            if run.len() < 2 {
                continue;
            }
            let has_rest_bridge = run.windows(2).any(|pair| pair[0].start + pair[0].duration < pair[1].start);
            let has_primary_only_note = run.iter().any(|info| partial_beat_level(info) == 1);
            let secondary_capable_notes = run.iter()
                .filter(|info| partial_beat_level(info) >= 2).take(2).count();
            let has_mixed_hierarchy = has_primary_only_note && secondary_capable_notes == 2;
            if has_rest_bridge || has_mixed_hierarchy {
                candidates.push((beat, staff.clone(), voice.clone(), run.to_vec(), has_rest_bridge, has_mixed_hierarchy));
            }
        }
    }
    candidates.sort_by_key(|(beat, staff, voice, _, _, _)| (*beat, staff.clone(), voice.clone()));
    candidates
}

fn structural_rest_bridge_beam_states(
    targets: &[&PartialBeatNoteInfo],
) -> BTreeMap<usize, Vec<(usize, &'static str)>> {
    let mut states = BTreeMap::new();
    let mut ordered = targets.to_vec();
    ordered.sort_by_key(|info| (info.start, info.index));
    let Some(plan) = structural_metric_beam_hierarchy_plan(&ordered, 2) else {
        return states;
    };
    for edit in plan.edits().iter().copied() {
        let Some(info) = ordered.get(edit.event_ordinal()) else {
            return BTreeMap::new();
        };
        states.entry(info.index).or_default().push((
            usize::from(edit.level()),
            edit.state().as_musicxml_text(),
        ));
    }
    states
}

#[cfg(test)]
mod shared_metric_beam_hierarchy_owner_tests {
    use super::{structural_metric_beam_hierarchy_plan, PartialBeatNoteInfo};

    fn sounding_16th(index: usize, start: i64) -> PartialBeatNoteInfo {
        PartialBeatNoteInfo {
            index,
            cursor_run: Some(0),
            start,
            duration: 24,
            staff: "1".to_string(),
            voice: "1".to_string(),
            note_type: "16th".to_string(),
            is_rest: false,
            is_chord: false,
            is_hidden: false,
            existing_beams: 0,
        }
    }

    #[test]
    fn final_complete_beat_primary_checkpoint_is_shared_level_one_projection() {
        let events = [
            sounding_16th(0, 0),
            sounding_16th(1, 24),
            sounding_16th(2, 48),
            sounding_16th(3, 72),
        ];
        let group = events.iter().collect::<Vec<_>>();
        let plan = structural_metric_beam_hierarchy_plan(&group, 1).expect("shared L1 plan");
        assert_eq!(plan.owner_receipt.schema_version, 1);
        assert_eq!(
            plan.owner_receipt.owner_kind,
            "structural_metric_beam_hierarchy_plan"
        );
        assert_eq!(plan.owner_receipt.event_count, 4);
        assert_eq!(plan.owner_receipt.highest_owned_level, 1);
        assert_eq!(
            plan.owner_receipt.plan_fingerprint_fnv64,
            plan.owner_receipt.primary_plan_fingerprint_fnv64
        );
        assert_eq!(
            plan.owner_receipt.primary_states,
            vec!["begin", "continue", "continue", "end"]
        );
        let rows = plan
            .edits()
            .iter()
            .map(|edit| {
                (
                    edit.event_ordinal(),
                    edit.level(),
                    edit.state().as_musicxml_text(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            rows,
            vec![
                (0, 1, "begin"),
                (1, 1, "continue"),
                (2, 1, "continue"),
                (3, 1, "end"),
            ]
        );
    }
}

#[cfg(test)]
mod structural_lower_rest_bridge_decision_scope_tests {
    use super::{
        choose_structural_rest_bridge_candidate_groups,
        structural_rest_bridge_beam_states,
        PartialBeatNoteInfo,
    };

    fn event(
        index: usize,
        start: i64,
        duration: i64,
        note_type: &str,
        is_rest: bool,
    ) -> PartialBeatNoteInfo {
        PartialBeatNoteInfo {
            index,
            cursor_run: Some(0),
            start,
            duration,
            staff: "2".to_string(),
            voice: "2".to_string(),
            note_type: note_type.to_string(),
            is_rest,
            is_chord: false,
            is_hidden: false,
            existing_beams: 0,
        }
    }

    #[test]
    fn rest_bridge_selector_exposes_sounding_anchors_not_rest_carriers() {
        let infos = vec![
            event(0, 0, 24, "16th", false),
            event(1, 24, 48, "eighth", true),
            event(2, 72, 24, "16th", false),
        ];
        let groups = choose_structural_rest_bridge_candidate_groups(&infos, 96);
        assert_eq!(groups.len(), 1);
        let (_, staff, voice, targets, has_rest_bridge, _) = &groups[0];
        assert_eq!(staff, "2");
        assert_eq!(voice, "2");
        assert!(*has_rest_bridge);
        assert_eq!(
            targets.iter().map(|info| info.start).collect::<Vec<_>>(),
            vec![0, 72]
        );
    }

    #[test]
    fn rest_bridge_stops_at_uncovered_time_within_the_same_beat() {
        for scale in [1, 3] {
            let infos = vec![
                event(0, 0, 12 * scale, "32nd", false),
                event(1, 12 * scale, 12 * scale, "32nd", true),
                event(2, 24 * scale, 12 * scale, "32nd", false),
                event(3, 48 * scale, 12 * scale, "32nd", false),
                event(4, 60 * scale, 12 * scale, "32nd", true),
                event(5, 72 * scale, 12 * scale, "32nd", false),
            ];
            let groups = choose_structural_rest_bridge_candidate_groups(&infos, 96 * scale);
            let actual: Vec<Vec<_>> = groups.iter().map(|group| {
                group.3.iter().map(|note| note.index).collect()
            }).collect();
            assert_eq!(actual, [vec![0, 2], vec![3, 5]]);
        }
    }

    #[test]
    fn mixed_hierarchy_follows_written_beam_levels_independently_of_tick_lengths() {
        for scale in [1, 3] {
            let groups = |values: [&str; 3]| {
                let infos = values.into_iter().enumerate().map(|(index, value)| {
                    event(index, index as i64 * 24 * scale, 24 * scale, value, false)
                }).collect::<Vec<_>>();
                choose_structural_rest_bridge_candidate_groups(&infos, 96 * scale)
                    .iter().filter(|group| group.5).count()
            };
            assert_eq!(groups(["eighth", "eighth", "eighth"]), 0);
            assert_eq!(groups(["16th", "eighth", "16th"]), 1);
            assert_eq!(groups(["eighth", "16th", "32nd"]), 1);
        }
    }

    #[test]
    fn rest_bridge_plan_keeps_secondary_on_two_sixteenth_anchors_only() {
        let left = event(0, 0, 24, "16th", false);
        let right = event(1, 72, 24, "16th", false);
        let targets = vec![&left, &right];
        let states = structural_rest_bridge_beam_states(&targets);
        assert_eq!(
            states.get(&0).cloned().unwrap_or_default(),
            vec![(1, "begin"), (2, "begin")]
        );
        assert_eq!(
            states.get(&1).cloned().unwrap_or_default(),
            vec![(1, "end"), (2, "end")]
        );
    }

    #[test]
    fn rest_bridge_plan_never_assigns_secondary_to_eighth_target() {
        let short = event(0, 0, 24, "16th", false);
        let eighth = event(1, 48, 48, "eighth", false);
        let targets = vec![&short, &eighth];
        let states = structural_rest_bridge_beam_states(&targets);
        assert_eq!(
            states.get(&0).cloned().unwrap_or_default(),
            vec![(1, "begin")]
        );
        assert_eq!(
            states.get(&1).cloned().unwrap_or_default(),
            vec![(1, "end")]
        );
        assert!(states
            .values()
            .flatten()
            .all(|(level, _)| *level == 1));
    }
}
