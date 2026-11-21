fn same_beat_interval_dynamic(start: i32, end: i32, beat_unit: i32) -> bool {
    if beat_unit <= 0 || end <= start {
        return false;
    }
    start.div_euclid(beat_unit) == end.saturating_sub(1).div_euclid(beat_unit)
}

struct ClearStaccatoCandidate {
    scope: usize,
    onset: i32,
    midi: i32,
    note_duration: i32,
    total_duration: i32,
}

fn primary_staccato_candidates(xml: &str) -> (usize, Option<ClearStaccatoCandidate>) {
    let (events, _) = collect_render_xml_note_events(xml);
    let metric = render_scope_metric_expected_and_beat_map(xml);
    let mut lanes = BTreeMap::<usize, Vec<RenderXmlNoteEvent>>::new();
    for event in events {
        if event.staff == "1" && event.voice == "1" && !event.is_hidden && !event.is_chord
            && (event.midi.is_some() || event.is_rest)
        {
            lanes.entry(event.scope).or_default().push(event);
        }
    }

    let mut candidates = Vec::new();
    for (scope, mut lane) in lanes {
        let beat = metric.get(&scope).map_or(96, |(_, beat)| *beat).max(1);
        lane.sort_by_key(|event| (event.onset, event.duration, event.is_rest));
        for pair in lane.windows(2) {
            let note = &pair[0];
            let rest = &pair[1];
            let Some(midi) = note.midi else { continue; };
            let total = note.duration.saturating_add(rest.duration);
            if rest.is_rest && rest.midi.is_none() && rest.onset == note.onset.saturating_add(note.duration)
                && note.duration > 0 && rest.duration > 0 && rest.duration <= note.duration
                && note.duration <= beat / 4 && rest.duration <= beat / 4 && total <= beat / 2
                && same_beat_interval_dynamic(note.onset, note.onset.saturating_add(total), beat)
                && duration_to_rest_type_and_dots(total).is_some()
            {
                candidates.push(ClearStaccatoCandidate {
                    scope, onset: note.onset, midi, note_duration: note.duration, total_duration: total,
                });
            }
        }
    }
    // All clear pairs share the highest score. Equal attacks historically preferred the longest pair.
    let count = candidates.len().min(12);
    let first = candidates.into_iter().min_by_key(|row| {
        (row.scope, row.onset, std::cmp::Reverse(row.total_duration))
    });
    (count, first)
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct AccentAttackOwnershipKey {
    scope: usize,
    staff: String,
    voice: String,
    onset: i64,
    midi: i32,
    duration: i64,
}

impl AccentAttackOwnershipKey {
    #[cfg(test)]
fn matches_row(&self, row: &AssertiveVisibleArticulationRow) -> bool {
        self.scope == row.scope
            && self.staff == row.staff
            && self.voice == row.voice
            && self.onset == row.onset
            && self.midi == row.midi
            && self.duration == row.duration
    }

    fn matches_note(&self, note: &AtomicNote) -> bool {
        note.measure.trim().parse::<usize>().ok() == Some(self.scope)
            && note.staff == self.staff
            && note.voice == self.voice
            && note.start == self.onset
            && note.pitch.as_ref().map(|pitch| pitch.midi) == Some(self.midi)
            && note.duration == self.duration
    }
}





#[cfg(test)]
fn attack_eligible_accent_rows(
    rows: Vec<AssertiveVisibleArticulationRow>,
    blocked: &[AccentAttackOwnershipKey],
) -> Vec<AssertiveVisibleArticulationRow> {
    rows.into_iter()
        .filter(|row| !blocked.iter().any(|key| key.matches_row(row)))
        .collect()
}

#[cfg(test)]
mod tied_tail_attack_ownership_local_blocker_tests {
    use super::{
        attack_eligible_accent_rows, AccentAttackOwnershipKey, AssertiveVisibleArticulationRow,
    };

    fn row(scope: usize, onset: i64, midi: i32) -> AssertiveVisibleArticulationRow {
        AssertiveVisibleArticulationRow {
            scope,
            staff: "1".to_string(),
            voice: "1".to_string(),
            onset,
            midi,
            duration: 24,
            mark: "accent".to_string(),
        }
    }

    fn key(scope: usize, onset: i64, midi: i32) -> AccentAttackOwnershipKey {
        AccentAttackOwnershipKey {
            scope,
            staff: "1".to_string(),
            voice: "1".to_string(),
            onset,
            midi,
            duration: 24,
        }
    }

    #[test]
    fn independent_duplicate_blocker_does_not_change_existing_local_decision() {
        let rows = vec![row(1, 48, 60), row(2, 48, 62), row(3, 48, 64)];
        let first_only = attack_eligible_accent_rows(rows.clone(), &[key(1, 48, 60)]);
        let duplicated = attack_eligible_accent_rows(
            rows,
            &[key(1, 48, 60), key(2, 48, 62)],
        );

        assert!(first_only.iter().all(|row| row.scope != 1));
        assert!(duplicated.iter().all(|row| row.scope != 1));
        assert!(first_only.iter().any(|row| row.scope == 3));
        assert!(duplicated.iter().any(|row| row.scope == 3));
        assert!(duplicated.iter().all(|row| row.scope != 2));
    }
}







// v0.10.722: former WAV-Truth/external-feedback textual include is now main-inline; no behavior change.

fn count_visible_staccato_marks(xml: &str) -> usize {
    parse_atomic_notes(xml)
        .into_iter()
        .filter(|note| note.visible && note.pitch.is_some() && !note.is_chord && note.has_staccato)
        .count()
}

fn apply_single_clear_staccato_render_write_with_placement_readback(
    xml: &str,
) -> anyhow::Result<(
    std::borrow::Cow<'_, str>,
    SingleClearStaccatoRenderWriteAudit,
    Option<crate::structural_write_authority::StaccatoPersistenceOccurrenceReceipt>,
)> {
    // Preserve the frozen sink signature in this retired/no-write boundary.
    // This function item is never invoked.
    let _retired_sink_signature = crate::structural_write_authority::write_single_staccato_candidate_authorized;

    let (clear_candidate_count, target) = primary_staccato_candidates(xml);
    let mut audit = SingleClearStaccatoRenderWriteAudit {
        clear_candidate_count,
        ..SingleClearStaccatoRenderWriteAudit::default()
    };
    let Some(target) = target else {
        return Ok((std::borrow::Cow::Borrowed(xml), audit, None));
    };
    audit.target_scope = target.scope;
    audit.target_staff = "1".to_string();
    audit.target_voice = "1".to_string();
    audit.target_onset = target.onset;
    audit.target_midi = target.midi;
    audit.target_note_duration = target.note_duration;
    audit.target_total_duration = target.note_duration;
    if clear_candidate_count != 1 {
        return Ok((std::borrow::Cow::Borrowed(xml), audit, None));
    }

    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut scan = 0_usize;
    'outer: while let Some((measure_start, measure_end, scope_number)) = find_next_measure_bounds(xml, scan) {
        if scope_number != target.scope {
            scan = measure_end;
            continue;
        }
        let measure = &xml[measure_start..measure_end];
        let fragments = collect_render_timeline_fragments_in_segment(
            measure,
            measure_start,
            scope_number,
        );
        let mut lane = fragments
            .iter()
            .filter(|frag| {
                !frag.is_hidden
                    && !frag.is_chord
                    && (frag.is_visible_pitched || frag.is_visible_rest)
                    && frag.staff.as_str() == audit.target_staff.as_str()
                    && frag.voice.as_str() == audit.target_voice.as_str()
            })
            .cloned()
            .collect::<Vec<_>>();
        lane.sort_by_key(|frag| (frag.onset, frag.duration, frag.is_rest));
        for pair in lane.windows(2) {
            let note = &pair[0];
            let rest = &pair[1];
            let Some(midi) = midi_from_note_fragment(&note.fragment) else { continue; };
            if midi != target.midi { continue; }
            if note.onset != target.onset || note.duration != target.note_duration { continue; }
            if !rest.is_visible_rest || rest.onset != note.onset.saturating_add(note.duration.max(0)) || rest.duration <= 0 { continue; }
            let total = note.duration.saturating_add(rest.duration);
            if duration_to_rest_type_and_dots(total).is_none() { continue; }
            let Some(new_note) = set_pitched_note_duration_type_add_staccato(&note.fragment, total) else { continue; };
            let Some(anchor_index) = fragments
                .iter()
                .position(|frag| frag.start == note.start && frag.end == note.end)
            else {
                continue;
            };
            let mut chord_member_midis = Vec::<i32>::new();
            let mut chord_replacements = Vec::<(usize, usize, String)>::new();
            let mut chord_cohort_supported = true;
            for member in fragments.iter().skip(anchor_index.saturating_add(1)) {
                if !member.is_note || !member.is_chord {
                    break;
                }
                let Some(member_midi) = midi_from_note_fragment(&member.fragment) else {
                    chord_cohort_supported = false;
                    break;
                };
                if member.duration != note.duration
                    || member.staff.as_str() != note.staff.as_str()
                    || member.voice.as_str() != note.voice.as_str()
                {
                    chord_cohort_supported = false;
                    break;
                }
                let new_member = crate::structural_write_authority::ensure_atomic_move_child_text_value_authorized(
                    &member.fragment,
                    "duration",
                    &total.to_string(),
                );
                let new_member =
                    crate::structural_write_authority::sync_atomic_move_note_type_and_dots_authorized(
                        &new_member,
                    );
                chord_member_midis.push(member_midi);
                chord_replacements.push((member.start, member.end, new_member));
            }
            if !chord_cohort_supported {
                continue;
            }
            chord_member_midis.sort_unstable();
            replacements.push((note.start, note.end, new_note));
            replacements.extend(chord_replacements);
            replacements.push((rest.start, rest.end, String::new()));
            audit.target_total_duration = total;
            audit.target_chord_member_midis = chord_member_midis;
            break 'outer;
        }
        scan = measure_end;
    }

    if replacements.is_empty() {
        return Ok((std::borrow::Cow::Borrowed(xml), audit, None));
    }

    let Some((candidate_xml, occurrence_receipt)) =
        crate::structural_write_authority::apply_bound_staccato_persistence_batch_authorized(
            xml,
            replacements,
        )
    else {
        return Ok((std::borrow::Cow::Borrowed(xml), audit, None));
    };

    // The ordinary no-write path needs no comparative XML scans.
    let counts = |events: &[RenderXmlNoteEvent], hidden| {
        let (mut pitched, mut rests) = (0_usize, 0_usize);
        for event in events.iter().filter(|event| event.is_hidden == hidden) {
            pitched += usize::from(event.midi.is_some());
            rests += usize::from(event.is_rest);
        }
        (pitched, rests)
    };
    let (before, _) = collect_render_xml_note_events(xml);
    let (after, balanced) = collect_render_xml_note_events(&candidate_xml);
    let (before_pitched, before_rests) = counts(&before, false);
    let (after_pitched, after_rests) = counts(&after, false);
    let new_marks = count_visible_staccato_marks(&candidate_xml).saturating_sub(count_visible_staccato_marks(xml));
    audit.staccato_equivalence_stable = after_pitched == before_pitched
        && after_rests.saturating_add(1) == before_rests
        && counts(&after, true) == counts(&before, true)
        && new_marks == 1
        && single_clear_staccato_target_persisted_in_final_xml(&candidate_xml, &audit);
    if audit.staccato_equivalence_stable && balanced && parse_musicxml(&candidate_xml).is_ok() {
        // RA3 must never submit a materializable candidate to the physical sink.
        crate::master_state_identity::count_render_closure_failure(
            crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures);
        anyhow::bail!("RA3 no-write contract: unexpected staccato candidate before physical sink");
    }
    let committed_occurrence_receipt = audit.written.then_some(occurrence_receipt);
    Ok((std::borrow::Cow::Owned(candidate_xml), audit, committed_occurrence_receipt))
}

fn single_clear_staccato_target_persisted_in_final_xml(
    xml: &str,
    audit: &SingleClearStaccatoRenderWriteAudit,
) -> bool {
    if audit.target_scope == 0
        || audit.target_staff.trim().is_empty()
        || audit.target_voice.trim().is_empty()
    {
        return false;
    }
    let expected_duration = audit
        .target_total_duration
        .max(audit.target_note_duration);
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, scope_number)) =
        find_next_measure_bounds(xml, scan)
    {
        if scope_number != audit.target_scope {
            scan = measure_end;
            continue;
        }
        let measure = &xml[measure_start..measure_end];
        let fragments = collect_render_timeline_fragments_in_segment(
            measure,
            measure_start,
            scope_number,
        );
        for (anchor_index, frag) in fragments.iter().enumerate() {
            if frag.is_hidden
                || frag.is_chord
                || !frag.is_visible_pitched
                || frag.staff.as_str() != audit.target_staff.as_str()
                || frag.voice.as_str() != audit.target_voice.as_str()
                || frag.onset != audit.target_onset
                || frag.duration != expected_duration
                || midi_from_note_fragment(&frag.fragment) != Some(audit.target_midi)
                || !frag.fragment.contains("<staccato")
            {
                continue;
            }

            let mut member_midis = Vec::<i32>::new();
            for member in fragments.iter().skip(anchor_index.saturating_add(1)) {
                if !member.is_note || !member.is_chord {
                    break;
                }
                if member.duration != expected_duration
                    || member.staff.as_str() != audit.target_staff.as_str()
                    || member.voice.as_str() != audit.target_voice.as_str()
                {
                    return false;
                }
                let Some(member_midi) = midi_from_note_fragment(&member.fragment) else {
                    return false;
                };
                member_midis.push(member_midi);
            }
            member_midis.sort_unstable();
            return member_midis == audit.target_chord_member_midis;
        }
        return false;
    }
    false
}
