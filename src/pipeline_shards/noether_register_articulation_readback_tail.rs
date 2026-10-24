fn same_beat_interval_dynamic(start: i32, end: i32, beat_unit: i32) -> bool {
    if beat_unit <= 0 || end <= start {
        return false;
    }
    start.div_euclid(beat_unit) == end.saturating_sub(1).div_euclid(beat_unit)
}

fn build_staccato_proxy_candidate_rows(xml: &str) -> Vec<StaccatoProxyCandidateRow> {
    let (events, _backup_forward_ok) = collect_render_xml_note_events(xml);
    let metric = render_scope_metric_expected_and_beat_map(xml);
    let mut by_lane: BTreeMap<(usize, String, String), Vec<RenderXmlNoteEvent>> = BTreeMap::new();
    for event in events.into_iter().filter(|event| !event.is_hidden && !event.is_chord) {
        if event.midi.is_some() || event.is_rest {
            by_lane
                .entry((event.scope, event.staff.clone(), event.voice.clone()))
                .or_default()
                .push(event);
        }
    }
    let mut rows = Vec::new();
    for ((scope, staff, voice), mut lane) in by_lane {
        lane.sort_by_key(|event| (event.onset, event.duration, event.is_rest));
        let beat_unit = metric.get(&scope).map_or(96, |(_, beat)| *beat).max(1);
        for pair in lane.windows(2) {
            let note = &pair[0];
            let rest = &pair[1];
            let Some(midi) = note.midi else { continue; };
            if rest.midi.is_some() || !rest.is_rest {
                continue;
            }
            if note.onset.saturating_add(note.duration.max(0)) != rest.onset {
                continue;
            }
            if note.duration <= 0 || rest.duration <= 0 {
                continue;
            }
            let total = note.duration.saturating_add(rest.duration);
            if !same_beat_interval_dynamic(note.onset, note.onset.saturating_add(total), beat_unit) {
                continue;
            }
            if duration_to_rest_type_and_dots(total).is_none() {
                continue;
            }
            
            rows.push(StaccatoProxyCandidateRow {
                scope,
                staff: staff.clone(),
                voice: voice.clone(),
                midi,
                note_onset: note.onset,
                note_duration: note.duration,
                rest_duration: rest.duration,
                total_duration: total,
                beat_unit,
            });
        }
    }
    rows.sort_by(|a, b| {
        b.total_duration.cmp(&a.total_duration)
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
            .then_with(|| a.note_onset.cmp(&b.note_onset))
    });
    rows
}
fn note_block_has_any(block: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| block.contains(needle))
}

fn build_pianist_helpful_articulation_and_notation_audit_no_mutation(
    xml: &str,
) -> PianistHelpfulArticulationAndNotationAudit {
    let global_non_legato_seen = xml.to_ascii_lowercase().contains("non legato");
    let mut rows = Vec::new();

    for row in build_staccato_proxy_candidate_rows(xml) {
        let beat_unit = row.beat_unit.max(1);
        let starts_on_beat = row.note_onset.rem_euclid(beat_unit) == 0;
        let short_pair = row.note_duration <= beat_unit / 4
            && row.rest_duration <= beat_unit / 4
            && row.total_duration <= beat_unit / 2;
        let long_weighted_note = row.note_duration >= beat_unit / 2
            && row.rest_duration <= beat_unit / 4;
        let registral_or_metric_impulse =
            starts_on_beat && (row.midi <= 52 || row.midi >= 84);
        let phrase_or_tie_like = row.total_duration > beat_unit / 2
            || row.rest_duration > row.note_duration.saturating_mul(2);

        let (class_name, class_score) = if short_pair {
            (
                "SAFE_STACCATO_MUSICALLY_PLACEMENT_GATED".to_string(),
                1000_i64,
            )
        } else if registral_or_metric_impulse {
            ("BETTER_ACCENT_THAN_STACCATO".to_string(), 760_i64)
        } else if long_weighted_note {
            ("BETTER_TENUTO_THAN_STACCATO".to_string(), 700_i64)
        } else if phrase_or_tie_like {
            ("BETTER_SLUR_OR_TIE_OR_NO_MARK".to_string(), 620_i64)
        } else if global_non_legato_seen && row.total_duration <= beat_unit / 2 {
            ("NO_MARK_NON_LEGATO_ALREADY_CLEAR".to_string(), 520_i64)
        } else {
            ("REVIEW_STACCATO_MUSICAL_CONTEXT".to_string(), 580_i64)
        };

        let owner_is_single_upper_voice = row.staff == "1" && row.voice == "1";
        let owner_is_lower_or_aux_voice = row.staff == "2" || row.voice != "1";
        let rest_geometry_needs_review = row.rest_duration > row.note_duration
            || row.rest_duration > beat_unit / 4
            || (row.voice != "1" && !short_pair);
        let stem_geometry_needs_review = owner_is_lower_or_aux_voice && row.voice != "1";
        let notehead_or_chord_mass_anchor_clear = owner_is_single_upper_voice
            && !rest_geometry_needs_review
            && !stem_geometry_needs_review
            && class_name.starts_with("SAFE_STACCATO");

        let (placement, placement_risk_label, placement_score_penalty) =
            if notehead_or_chord_mass_anchor_clear {
                (
                    "anchor_geometry_notehead_or_chord_mass_clear".to_string(),
                    "clear: upper-staff notehead/chord-mass proxy clear; actual render-write remains separately gated".to_string(),
                    0_i64,
                )
            } else if row.staff == "2" {
                (
                    "anchor_geometry_lower_staff_strict_review".to_string(),
                    "medium: untere Hand bleibt streng; Punkt darf nicht wie Hals-/Leer-Lane-Zeichen wirken".to_string(),
                    120_i64,
                )
            } else {
                (
                    "anchor_geometry_pdf_readback_required".to_string(),
                    "low: Anker-Geometrie noch nur Proxy; echte PDF-Bindung vor Write prüfen".to_string(),
                    60_i64,
                )
            };

        rows.push(PianistHelpfulArticulationRow {
            scope: row.scope,
            staff: row.staff,
            voice: row.voice,
            onset: row.note_onset,
            midi: row.midi,
            duration: row.note_duration,
            class_name,
            placement,
            placement_risk: placement_risk_label,
            score: class_score.saturating_sub(placement_score_penalty),
        });
    }

    rows.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
            .then_with(|| a.onset.cmp(&b.onset))
    });

    PianistHelpfulArticulationAndNotationAudit {
        rows: rows.into_iter().take(12).collect(),
    }
}

fn articulation_single_clear_candidate_rows(audit: &PianistHelpfulArticulationAndNotationAudit) -> Vec<&PianistHelpfulArticulationRow> {
    audit.rows.iter()
        .filter(|row| row.class_name.starts_with("SAFE_STACCATO") && row.placement == "anchor_geometry_notehead_or_chord_mass_clear")
        .collect()
}
fn note_block_has_true_articulation(block: &str) -> bool {
    note_block_has_any(block, &["<staccato", "<staccatissimo", "<tenuto", "<detached-legato", "<accent", "<strong-accent", "<sfz", "<sforzando"])
}













fn build_assertive_visible_articulation_candidates(xml: &str) -> Vec<AssertiveVisibleArticulationRow> {
    let metric = render_scope_metric_expected_and_beat_map(xml);
    let mut rows = Vec::new();
    for note in parse_atomic_notes(xml) {
        if !note.visible || note.is_chord || note.duration <= 0 {
            continue;
        }
        let Some(pitch) = note.pitch.as_ref() else { continue; };
        let Some(scope) = note.measure.parse::<usize>().ok() else { continue; };
        let block = xml.get(note.note_start..note.note_end).unwrap_or("");
        if note_block_has_true_articulation(block) {
            continue;
        }
        let (expected, beat_unit) = metric.get(&scope).copied().unwrap_or((384, 96));
        let beat = beat_unit.max(1) as i64;
        let expected_i64 = expected.max(1) as i64;
        if note.start < 0 || note.start >= expected_i64 {
            continue;
        }
        let onset_mod = note.start.rem_euclid(beat);
        let starts_on_beat = onset_mod == 0;
        let starts_on_half_beat = onset_mod == beat / 2 && beat >= 2;
        let closes_beat = (note.start + note.duration).rem_euclid(beat) == 0 || note.start + note.duration >= expected_i64;
        let short_note = note.duration <= (beat / 4).max(1);
        let medium_note = note.duration <= (beat / 2).max(1);
        let long_or_weighted = note.duration >= (beat / 2).max(1) || closes_beat;
        let register_impulse = pitch.midi <= 52 || pitch.midi >= 84;
        let primary_owner = note.voice.trim() == "1";
        let staff_weight = if note.staff.trim() == "1" { 40_i64 } else { 20_i64 };
        let owner_penalty = if primary_owner { 0_i64 } else { 80_i64 };

        let (mark, _class_name, _reason, base_score) = if starts_on_beat && (register_impulse || medium_note) {
            (
                "accent".to_string(),
                "ASSERTIVE_METRIC_OR_REGISTER_ACCENT".to_string(),
                "metrischer oder registerbetonter Impuls; sichtbarere Artikulation ohne Dauer-/Tonhöhenänderung".to_string(),
                950_i64,
            )
        } else if long_or_weighted && primary_owner {
            (
                "tenuto".to_string(),
                "ASSERTIVE_WEIGHTED_TENUTO".to_string(),
                "gewichtete/gebundene Note; Tenuto macht die lokale Artikulation lesbar, ohne Staccato-Verkürzung zu behaupten".to_string(),
                860_i64,
            )
        } else if starts_on_half_beat && register_impulse && primary_owner {
            (
                "accent".to_string(),
                "ASSERTIVE_OFFBEAT_REGISTER_ACCENT".to_string(),
                "Offbeat-Registerimpuls; Akzent statt zusätzlichem Staccato".to_string(),
                780_i64,
            )
        } else if starts_on_beat && primary_owner && !short_note {
            (
                "tenuto".to_string(),
                "ASSERTIVE_BEAT_TENUTO".to_string(),
                "Zählzeitträger; Tenuto ist sichtbar, aber defensiver als weitere Staccati".to_string(),
                720_i64,
            )
        } else {
            continue;
        };
        let placement_class = if note.staff.trim() == "1" && primary_owner {
            "notehead_or_chord_mass_visible_upper".to_string()
        } else if primary_owner {
            "notehead_or_chord_mass_visible_lower_tenuto_accent_only".to_string()
        } else {
            "auxiliary_voice_owner_review".to_string()
        };
        if placement_class == "auxiliary_voice_owner_review" {
            continue;
        }
        let score = base_score
            .saturating_add(staff_weight)
            .saturating_add(if starts_on_beat { 80 } else { 0 })
            .saturating_add(if register_impulse { 70 } else { 0 })
            .saturating_sub(owner_penalty)
            .saturating_sub((note.start / beat.max(1)).max(0) * 3);
        rows.push(AssertiveVisibleArticulationRow {
            scope,
            staff: note.staff,
            voice: note.voice,
            onset: note.start,
            midi: pitch.midi,
            duration: note.duration,
            mark,
            score,
        });
    }
    rows.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.onset.cmp(&b.onset))
            .then_with(|| a.midi.cmp(&b.midi))
    });
    rows
}

fn select_assertive_visible_articulation_rows(rows: &[AssertiveVisibleArticulationRow], target: usize) -> Vec<AssertiveVisibleArticulationRow> {
    let mut selected = Vec::<AssertiveVisibleArticulationRow>::new();
    let mut per_scope: BTreeMap<usize, usize> = BTreeMap::new();
    let mut per_scope_staff: BTreeMap<(usize, String), usize> = BTreeMap::new();
    let mut occupied = BTreeSet::<(usize, String, String, i64)>::new();
    for row in rows {
        if selected.len() >= target {
            break;
        }
        if *per_scope.get(&row.scope).unwrap_or(&0) >= 2 {
            continue;
        }
        if *per_scope_staff.get(&(row.scope, row.staff.clone())).unwrap_or(&0) >= 1 {
            continue;
        }
        let key = (row.scope, row.staff.clone(), row.voice.clone(), row.onset);
        if occupied.contains(&key) {
            continue;
        }
        occupied.insert(key);
        *per_scope.entry(row.scope).or_insert(0) += 1;
        *per_scope_staff.entry((row.scope, row.staff.clone())).or_insert(0) += 1;
        selected.push(row.clone());
    }
    if selected.len() < target {
        for row in rows {
            if selected.len() >= target {
                break;
            }
            if *per_scope.get(&row.scope).unwrap_or(&0) >= 3 {
                continue;
            }
            let key = (row.scope, row.staff.clone(), row.voice.clone(), row.onset);
            if occupied.contains(&key) {
                continue;
            }
            occupied.insert(key);
            *per_scope.entry(row.scope).or_insert(0) += 1;
            selected.push(row.clone());
        }
    }
    selected.sort_by(|a, b| {
        a.scope.cmp(&b.scope)
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.onset.cmp(&b.onset))
            .then_with(|| a.midi.cmp(&b.midi))
    });
    selected
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
            ..AssertiveVisibleArticulationRow::default()
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
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(
    String,
    SingleClearStaccatoRenderWriteAudit,
    Option<crate::structural_write_authority::StaccatoPersistenceOccurrenceReceipt>,
)> {
    // Preserve the frozen sink signature in this retired/no-write boundary.
    // This function item is never invoked.
    let _retired_sink_signature = crate::structural_write_authority::write_single_staccato_candidate_authorized;

    let before_articulation = build_pianist_helpful_articulation_and_notation_audit_no_mutation(xml);
    let clear_rows = articulation_single_clear_candidate_rows(&before_articulation);
    let (before_events, _before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let visible_pitched_before = before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    let visible_rests_before = before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    let hidden_pitched_before = before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    let hidden_rests_before = before_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    let existing_staccato_before = count_visible_staccato_marks(xml);

    let mut audit = SingleClearStaccatoRenderWriteAudit {
        output_path: output_path.display().to_string(),
        clear_candidate_count: clear_rows.len(),
        visible_pitched_before,
        visible_rests_before,
        hidden_pitched_before,
        hidden_rests_before,
        existing_staccato_before,
        write_allowed: clear_rows.len() == 1,
        placement_binding: "none".to_string(),
        placement_readback: "not_checked".to_string(),
        verdict: "single_clear_staccato_no_clear_candidate_no_write".to_string(),
        ..SingleClearStaccatoRenderWriteAudit::default()
    };

    let Some(target) = clear_rows.first().copied() else {
        audit.done_fragment = "SingleClearStaccatoRenderWriteWithPlacementReadback: ClearCandidateCount 0; NewStaccatoMarks 0; TargetMatched 0; UnexpectedStaccatoMarks 0; WriteAllowed nein; RenderXmlWrite nein; MusicMutation nein".to_string();
        audit.pipeline_line = format!(
            "SingleClearStaccatoRenderWriteWithPlacementReadback v{}: Quelle {}; ClearCandidateCount 0; kein Ziel; TargetMatched 0; NewStaccatoMarks 0; UnexpectedStaccatoMarks 0; Placement none; StemLikeRiskTarget 0; RestLaneRiskTarget 0; LowerStaffWrites 0; parse/back-forward -/-; WriteAllowed nein; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            source_label,
            audit.verdict
        );
        return Ok((xml.to_string(), audit, None));
    };

    audit.target_scope = target.scope;
    audit.target_staff = target.staff.clone();
    audit.target_voice = target.voice.clone();
    audit.target_onset = target.onset;
    audit.target_midi = target.midi;
    audit.target_note_duration = target.duration;
    audit.target_total_duration = target.duration;
    audit.placement_binding = target.placement.clone();
    audit.placement_readback = if target.placement.as_str() == "anchor_geometry_notehead_or_chord_mass_clear" {
        "notehead_or_chord_mass_clear".to_string()
    } else {
        "not_clear".to_string()
    };
    audit.lower_staff_writes = if target.staff.as_str() == "2" { 1 } else { 0 };
    audit.stem_like_risk_target = if target.placement_risk.contains("stem") || target.placement_risk.contains("Hals") || target.voice.as_str() != "1" { 1 } else { 0 };
    audit.rest_lane_risk_target = if target.placement_risk.contains("Rest") || target.placement_risk.contains("rest") || target.placement_risk.contains("Leer") { 1 } else { 0 };

    if clear_rows.len() != 1 || target.staff.as_str() != "1" || target.voice.as_str() != "1" || target.placement.as_str() != "anchor_geometry_notehead_or_chord_mass_clear" {
        audit.verdict = "single_clear_staccato_guard_rejected_no_write".to_string();
        audit.done_fragment = format!(
            "SingleClearStaccatoRenderWriteWithPlacementReadback: ClearCandidateCount {}; NewStaccatoMarks 0; TargetMatched 0; UnexpectedStaccatoMarks 0; WriteAllowed nein; RenderXmlWrite nein; MusicMutation nein",
            clear_rows.len()
        );
        audit.pipeline_line = format!(
            "SingleClearStaccatoRenderWriteWithPlacementReadback v{}: Quelle {}; ClearCandidateCount {}; Ziel scope {} staff {} voice {} onset {} midi {}; Guard rejected; TargetMatched 0; NewStaccatoMarks 0; UnexpectedStaccatoMarks 0; Placement {}; StemLikeRiskTarget {}; RestLaneRiskTarget {}; LowerStaffWrites {}; WriteAllowed nein; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"), source_label, clear_rows.len(), target.scope, target.staff, target.voice, target.onset, target.midi, audit.placement_readback, audit.stem_like_risk_target, audit.rest_lane_risk_target, audit.lower_staff_writes, audit.verdict
        );
        return Ok((xml.to_string(), audit, None));
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
                    && frag.staff.as_str() == target.staff.as_str()
                    && frag.voice.as_str() == target.voice.as_str()
            })
            .cloned()
            .collect::<Vec<_>>();
        lane.sort_by_key(|frag| (frag.onset, frag.duration, frag.is_rest));
        for pair in lane.windows(2) {
            let note = &pair[0];
            let rest = &pair[1];
            let Some(midi) = midi_from_note_fragment(&note.fragment) else { continue; };
            if midi != target.midi { continue; }
            if note.onset != target.onset || note.duration != target.duration { continue; }
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
            audit.target_matched = 1;
            audit.target_rest_duration = rest.duration;
            audit.target_total_duration = total;
            audit.target_chord_member_midis = chord_member_midis;
            audit.actions.push(BehindBarsRuleCheckRestAndTieAction {
                action_kind: "single_clear_staccato_render_write".to_string(),
                old_rest_count: 1,
                new_rest_count: 0,
                verdict: "adopted_if_readback_green".to_string(),
            });
            break 'outer;
        }
        scan = measure_end;
    }

    if audit.target_matched != 1 {
        audit.verdict = "single_clear_staccato_target_not_matched_no_write".to_string();
        audit.done_fragment = format!(
            "SingleClearStaccatoRenderWriteWithPlacementReadback: ClearCandidateCount {}; NewStaccatoMarks 0; TargetMatched 0; UnexpectedStaccatoMarks 0; WriteAllowed nein; RenderXmlWrite nein; MusicMutation nein",
            clear_rows.len()
        );
        audit.pipeline_line = format!(
            "SingleClearStaccatoRenderWriteWithPlacementReadback v{}: Quelle {}; ClearCandidateCount {}; Ziel scope {} staff {} voice {} onset {} midi {}; TargetMatched 0; NewStaccatoMarks 0; UnexpectedStaccatoMarks 0; Placement {}; StemLikeRiskTarget {}; RestLaneRiskTarget {}; LowerStaffWrites {}; WriteAllowed nein; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"), source_label, clear_rows.len(), target.scope, target.staff, target.voice, target.onset, target.midi, audit.placement_readback, audit.stem_like_risk_target, audit.rest_lane_risk_target, audit.lower_staff_writes, audit.verdict
        );
        return Ok((xml.to_string(), audit, None));
    }

    let Some((candidate_xml, occurrence_receipt)) =
        crate::structural_write_authority::apply_bound_staccato_persistence_batch_authorized(
            xml,
            replacements,
        )
    else {
        audit.target_matched = 0;
        audit.write_allowed = false;
        audit.verdict = "single_clear_staccato_chord_cohort_authority_rejected_no_write".to_string();
        for action in &mut audit.actions {
            action.verdict = "rejected_by_chord_cohort_authority".to_string();
        }
        audit.done_fragment =
            "SingleClearStaccatoRenderWriteWithPlacementReadback: ChordCohortAuthority rejected; RenderXmlWrite nein; MusicMutation nein"
                .to_string();
        audit.pipeline_line = format!(
            "SingleClearStaccatoRenderWriteWithPlacementReadback v{}: Quelle {}; ChordCohortAuthority rejected fail-closed; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            source_label,
            audit.verdict
        );
        return Ok((xml.to_string(), audit, None));
    };

    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    audit.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    audit.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    audit.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    audit.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    audit.existing_staccato_after = count_visible_staccato_marks(&candidate_xml);
    audit.new_staccato_marks = audit.existing_staccato_after.saturating_sub(audit.existing_staccato_before);
    audit.unexpected_staccato_marks = audit.new_staccato_marks.saturating_sub(1);
    audit.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    audit.backup_forward_after_ok = after_backup_forward_ok;
    audit.chord_cohort_readback =
        single_clear_staccato_target_persisted_in_final_xml(&candidate_xml, &audit);
    audit.staccato_equivalence_stable = audit.visible_pitched_after == audit.visible_pitched_before
        && audit.visible_rests_after.saturating_add(1) == audit.visible_rests_before
        && audit.hidden_pitched_after == audit.hidden_pitched_before
        && audit.hidden_rests_after == audit.hidden_rests_before
        && audit.new_staccato_marks == 1
        && audit.unexpected_staccato_marks == 0
        && audit.lower_staff_writes == 0
        && audit.stem_like_risk_target == 0
        && audit.chord_cohort_readback;

    let invariant_ok = audit.write_allowed
        && audit.target_matched == 1
        && audit.parse_after_ok
        && audit.backup_forward_after_ok
        && audit.staccato_equivalence_stable
        && audit.placement_readback == "notehead_or_chord_mass_clear";

    if invariant_ok {
        // RA3 is a declared no-write stage. An unexpectedly materializable
        // legacy candidate must never bypass sovereign transformation proof.
        crate::master_state_identity::count_render_closure_failure(
            crate::master_state_identity::RenderClosureMetric::CandidateCoverageFailures);
        anyhow::bail!("RA3 no-write contract: unexpected staccato candidate before physical sink");
    } else {
        audit.verdict = "single_clear_staccato_rejected_by_readback_guard".to_string();
        for action in &mut audit.actions { action.verdict = "rejected_by_single_clear_staccato_readback_guard".to_string(); }
    }
    audit.music_mutation = false;
    audit.done_fragment = format!(
        "SingleClearStaccatoRenderWriteWithPlacementReadback: ClearCandidateCount {}; NewStaccatoMarks {}; TargetMatched {}; UnexpectedStaccatoMarks {}; ChordMembers {}; ChordCohortReadback {}; Placement {}; StemLikeRiskTarget {}; RestLaneRiskTarget {}; LowerStaffWrites {}; RenderXmlWrite {}; MusicMutation nein",
        audit.clear_candidate_count,
        audit.new_staccato_marks,
        audit.target_matched,
        audit.unexpected_staccato_marks,
        audit.target_chord_member_midis.len(),
        yes_no(audit.chord_cohort_readback),
        audit.placement_readback,
        audit.stem_like_risk_target,
        audit.rest_lane_risk_target,
        audit.lower_staff_writes,
        yes_no(audit.render_xml_write)
    );
    audit.pipeline_line = format!(
        "SingleClearStaccatoRenderWriteWithPlacementReadback v{}: Quelle {}; Output {}; ClearCandidateCount {}; Ziel scope {} staff {} voice {} onset {} midi {}; NewStaccatoMarks {}; TargetMatched {}; UnexpectedStaccatoMarks {}; ChordMembers {}; ChordCohortReadback {}; Placement {}; StemLikeRiskTarget {}; RestLaneRiskTarget {}; LowerStaffWrites {}; visible_pitched {}→{}; visible_rests {}→{}; staccato_marks {}→{}; parse/back-forward {}/{}; StaccatoEquivalentStable {}; WriteAllowed {}; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite source-selected; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        source_label,
        output_path.display(),
        audit.clear_candidate_count,
        audit.target_scope,
        audit.target_staff,
        audit.target_voice,
        audit.target_onset,
        audit.target_midi,
        audit.new_staccato_marks,
        audit.target_matched,
        audit.unexpected_staccato_marks,
        audit.target_chord_member_midis.len(),
        yes_no(audit.chord_cohort_readback),
        audit.placement_readback,
        audit.stem_like_risk_target,
        audit.rest_lane_risk_target,
        audit.lower_staff_writes,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.visible_rests_before,
        audit.visible_rests_after,
        audit.existing_staccato_before,
        audit.existing_staccato_after,
        if audit.parse_after_ok { "ok" } else { "fail" },
        if audit.backup_forward_after_ok { "ok" } else { "fail" },
        yes_no(audit.staccato_equivalence_stable),
        yes_no(audit.write_allowed),
        yes_no(audit.render_xml_write),
        audit.verdict
    );
    let committed_occurrence_receipt = audit.written.then_some(occurrence_receipt);
    Ok((candidate_xml, audit, committed_occurrence_receipt))
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

fn refresh_single_clear_staccato_audit_from_primary_boundary_artifact(
    audit: &mut SingleClearStaccatoRenderWriteAudit,
    final_xml: &str,
    final_source_label: &str,
) {
    let (final_events, final_backup_forward_ok) = collect_render_xml_note_events(final_xml);
    audit.visible_pitched_after = final_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    audit.visible_rests_after = final_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    audit.hidden_pitched_after = final_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    audit.hidden_rests_after = final_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    audit.existing_staccato_after = count_visible_staccato_marks(final_xml);
    audit.new_staccato_marks = audit.existing_staccato_after.saturating_sub(audit.existing_staccato_before);
    audit.unexpected_staccato_marks = audit.new_staccato_marks.saturating_sub(1);
    audit.parse_after_ok = parse_musicxml(final_xml).is_ok();
    audit.backup_forward_after_ok = final_backup_forward_ok;
    let persisted = single_clear_staccato_target_persisted_in_final_xml(final_xml, audit);
    audit.chord_cohort_readback = persisted;
    audit.target_matched = if persisted { 1 } else { 0 };
    audit.staccato_equivalence_stable = audit.visible_pitched_after == audit.visible_pitched_before
        && audit.visible_rests_after.saturating_add(1) == audit.visible_rests_before
        && audit.hidden_pitched_after == audit.hidden_pitched_before
        && audit.hidden_rests_after == audit.hidden_rests_before
        && audit.new_staccato_marks == 1
        && audit.unexpected_staccato_marks == 0
        && audit.lower_staff_writes == 0
        && audit.stem_like_risk_target == 0
        && persisted;
    if persisted && audit.staccato_equivalence_stable {
        audit.written = true;
        audit.render_xml_write = true;
        audit.verdict = "single_clear_staccato_primary_boundary_green".to_string();
        for action in &mut audit.actions {
            action.verdict = "adopted_primary_boundary_persisted".to_string();
        }
    } else {
        audit.written = false;
        audit.render_xml_write = false;
        audit.verdict = "single_clear_staccato_primary_boundary_mismatch".to_string();
        for action in &mut audit.actions {
            action.verdict = "rejected_primary_boundary_persistence_mismatch".to_string();
        }
    }
    audit.music_mutation = false;
    audit.done_fragment = format!(
        "SingleClearStaccatoPrimaryBoundaryReadback: ClearCandidateCount {}; NewStaccatoMarks {}; TargetMatched {}; UnexpectedStaccatoMarks {}; ChordMembers {}; ChordCohortReadback {}; Placement {}; StemLikeRiskTarget {}; RestLaneRiskTarget {}; LowerStaffWrites {}; PrimaryBoundaryPersisted {}; RenderXmlWrite {}; MusicMutation nein",
        audit.clear_candidate_count,
        audit.new_staccato_marks,
        audit.target_matched,
        audit.unexpected_staccato_marks,
        audit.target_chord_member_midis.len(),
        yes_no(audit.chord_cohort_readback),
        audit.placement_readback,
        audit.stem_like_risk_target,
        audit.rest_lane_risk_target,
        audit.lower_staff_writes,
        yes_no(persisted),
        yes_no(audit.render_xml_write)
    );
    audit.pipeline_line = format!(
        "SingleClearStaccatoPrimaryBoundaryReadback v{}: Quelle {}; Output {}; PrimaryBoundaryReadback ja; ClearCandidateCount {}; Ziel scope {} staff {} voice {} onset {} midi {}; NewStaccatoMarks {}; TargetMatched {}; UnexpectedStaccatoMarks {}; ChordMembers {}; ChordCohortReadback {}; Placement {}; StemLikeRiskTarget {}; RestLaneRiskTarget {}; LowerStaffWrites {}; visible_pitched {}→{}; visible_rests {}→{}; staccato_marks {}→{}; parse/back-forward {}/{}; PrimaryBoundaryPersisted {}; StaccatoEquivalentStable {}; WriteAllowed {}; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite source-selected; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        final_source_label,
        audit.output_path,
        audit.clear_candidate_count,
        audit.target_scope,
        audit.target_staff,
        audit.target_voice,
        audit.target_onset,
        audit.target_midi,
        audit.new_staccato_marks,
        audit.target_matched,
        audit.unexpected_staccato_marks,
        audit.target_chord_member_midis.len(),
        yes_no(audit.chord_cohort_readback),
        audit.placement_readback,
        audit.stem_like_risk_target,
        audit.rest_lane_risk_target,
        audit.lower_staff_writes,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.visible_rests_before,
        audit.visible_rests_after,
        audit.existing_staccato_before,
        audit.existing_staccato_after,
        if audit.parse_after_ok { "ok" } else { "fail" },
        if audit.backup_forward_after_ok { "ok" } else { "fail" },
        yes_no(persisted),
        yes_no(audit.staccato_equivalence_stable),
        yes_no(audit.write_allowed),
        yes_no(audit.render_xml_write),
        audit.verdict
    );
}
