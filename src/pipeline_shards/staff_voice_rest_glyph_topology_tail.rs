fn launcher_version_contract_section() -> String {
    let binary_report_version = env!("CARGO_PKG_VERSION").to_string();
    let cargo_version = std::fs::read_to_string("Cargo.toml")
        .ok()
        .and_then(|text| extract_manifest_version_from_text(&text))
        .unwrap_or_else(|| binary_report_version.clone());
    let run_cmd_text = std::fs::read_to_string("run.cmd").unwrap_or_default();
    let launcher_source_text = std::fs::read_to_string("src/bin/etuschlopf_launcher.rs").unwrap_or_default();
    let cmd_manifest_dynamic = run_cmd_text.contains("Cargo.toml")
        && run_cmd_text.contains("ETUSCHLOPF_VERSION")
        && (run_cmd_text.contains("Select-String") || run_cmd_text.contains("findstr") || run_cmd_text.contains("powershell"));
    let rust_launcher_manifest_dynamic = run_cmd_text.contains("--bin etuschlopf_launcher")
        && launcher_source_text.contains("read_cargo_version")
        && launcher_source_text.contains("Cargo.toml");
    let launcher_dynamic = cmd_manifest_dynamic || rust_launcher_manifest_dynamic;
    let fixed_launcher_version = extract_fixed_launcher_version_from_text(&run_cmd_text);
    let launcher_source = if launcher_dynamic {
        if rust_launcher_manifest_dynamic {
            "RustLauncher-Cargo.toml-dynamic"
        } else {
            "CMD-Cargo.toml-dynamic"
        }
    } else if fixed_launcher_version.is_some() {
        "fixed-literal"
    } else if run_cmd_text.is_empty() {
        "run.cmd-nicht-lesbar"
    } else {
        "unbekannt"
    };
    let launcher_version = if launcher_dynamic {
        cargo_version.clone()
    } else {
        fixed_launcher_version.unwrap_or_else(|| "unbekannt".to_string())
    };
    let contract_green = cargo_version == binary_report_version
        && (launcher_dynamic || launcher_version == binary_report_version);
    let verdict = if contract_green {
        "launcher_version_contract_green_no_apply"
    } else {
        "launcher_version_contract_red_fix_required_no_apply"
    };
    let mut out = String::new();
    out.push_static_section_header_v795("LAUNCHER-VERSION-CONTRACT-AUDIT", "-------------------------------");
    out.push_format_args(format_args!(
        "Status: v{binary_report_version} audit-only als Versions-/Launcher-Sektion. Gesamtartefakt darf p2-begrenzte RenderMusicXML-Mutation und renderseitige Notationspolitur enthalten; diese Sektion selbst erzeugt keine zusätzliche Musikmutation, keine zusätzliche PDF-/MusicXML-Aenderung und keinen MasterApply.\n"
    ));
    out.push_format_args(format_args!(
        "Kurz: CargoVersion {}; BinaryReportVersion {}; LauncherVersion {}; LauncherSource {}; ContractGreen {}; Urteil {}.\n",
        cargo_version,
        binary_report_version,
        launcher_version,
        launcher_source,
        yes_no(contract_green),
        verdict
    ));
    crate::push_static_ent_line_v765(&mut out, " run.cmd darf keine alte feste Version mehr melden. Der aktuelle Bootstrap delegiert an den Rust-Launcher; dieser liest Cargo.toml und faellt nur auf seine Binary-Konstante zurueck. Report und Binary bleiben env!(CARGO_PKG_VERSION)-gefuehrt.");
    out
}





















fn upper_staff_rest_stack_intervals_overlap(a_start: i64, a_duration: i64, b_start: i64, b_duration: i64) -> bool {
    let a_end = a_start + a_duration.max(0);
    let b_end = b_start + b_duration.max(0);
    a_start < b_end && b_start < a_end
}






#[derive(Debug, Clone, Default)]
struct UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixRow {
    staff: String,
    voice: String,
    onset: i64,
    duration: i64,
    pitched_overlap_count: usize,
    same_voice_pitched_overlap_count: usize,
    pitched_overlap_coverage: i64,
    pitched_overlap_union_coverage: i64,
    pitched_geometry_union_coverage: i64,
    pitched_overlap_multiplicity_peak: usize,
    has_real_voice1_rest_overlap: bool,
    role_decision: String,
}

#[derive(Debug, Clone, Default)]
struct UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixAudit {
    source_label: String,
    scopes_checked: usize,
    candidate_rows: usize,
    full_forward_rows: usize,
    split_guard_rows: usize,
    keep_voice1_rows: usize,
    protected_only_rows: usize,
    real_hand_rest_rows: usize,
    independent_countervoice_rows: usize,
    mixed_review_rows: usize,
    non_focus_shadow_rows: usize,
    forward_duration: i32,
    visible_rests_before: usize,
    visible_rests_after: usize,
    visible_pitched_before: usize,
    visible_pitched_after: usize,
    hidden_pitched_before: usize,
    hidden_pitched_after: usize,
    hidden_rests_before: usize,
    hidden_rests_after: usize,
    sounding_stable: bool,
    parse_after_ok: bool,
    backup_forward_after_ok: bool,
    raw_cursor_after_ok: bool,
    written: bool,
    candidate_rewrite_path_enabled: bool,
    rows: Vec<UpperStaffShadowRestVoice1KeepWithoutShadowLaneFixRow>,
    verdict: String,
    pipeline_line: String,
    done_fragment: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct AtomicVisibleRestRoleObservation {
    pitched_overlap_count: usize,
    rest_overlap_count: usize,
    same_voice_pitched_overlap_count: usize,
    pitched_overlap_coverage: i64,
    pitched_overlap_union_coverage: i64,
    pitched_geometry_union_coverage: i64,
    pitched_overlap_multiplicity_peak: usize,
    has_real_voice1_rest_overlap: bool,
}

fn observe_atomic_visible_rest_role(
    rest: &AtomicNote,
    notes: &[AtomicNote],
) -> AtomicVisibleRestRoleObservation {
    let same_staff_measure = notes
        .iter()
        .filter(|note| note.visible && note.measure == rest.measure && note.staff == rest.staff)
        .collect::<Vec<_>>();
    let pitched_overlaps = same_staff_measure
        .iter()
        .copied()
        .filter(|note| {
            note.pitch.is_some()
                && note.voice != rest.voice
                && upper_staff_rest_stack_intervals_overlap(
                    rest.start,
                    rest.duration,
                    note.start,
                    note.duration,
                )
        })
        .collect::<Vec<_>>();
    let rest_overlaps = same_staff_measure
        .iter()
        .copied()
        .filter(|note| {
            note.pitch.is_none()
                && note.duration > 0
                && note.note_start != rest.note_start
                && upper_staff_rest_stack_intervals_overlap(
                    rest.start,
                    rest.duration,
                    note.start,
                    note.duration,
                )
        })
        .collect::<Vec<_>>();
    let same_voice_pitched_overlap_count = same_staff_measure
        .iter()
        .filter(|note| {
            note.pitch.is_some()
                && note.voice == rest.voice
                && upper_staff_rest_stack_intervals_overlap(
                    rest.start,
                    rest.duration,
                    note.start,
                    note.duration,
                )
        })
        .count();
    let pitched_overlap_coverage = pitched_overlaps
        .iter()
        .map(|note| {
            let a = rest.start.max(note.start);
            let b = rest
                .start
                .saturating_add(rest.duration.max(0))
                .min(note.start.saturating_add(note.duration.max(0)));
            (b - a).max(0)
        })
        .sum::<i64>()
        .min(rest.duration.max(0));
    let overlap_intervals = pitched_overlaps
        .iter()
        .filter_map(|note| {
            let a = rest.start.max(note.start);
            let b = rest
                .start
                .saturating_add(rest.duration.max(0))
                .min(note.start.saturating_add(note.duration.max(0)));
            (b > a).then_some((a, b))
        })
        .collect::<Vec<_>>();
    let pitched_geometry_intervals = same_staff_measure
        .iter()
        .copied()
        .filter(|note| {
            note.pitch.is_some()
                && upper_staff_rest_stack_intervals_overlap(
                    rest.start,
                    rest.duration,
                    note.start,
                    note.duration,
                )
        })
        .filter_map(|note| {
            let a = rest.start.max(note.start);
            let b = rest
                .start
                .saturating_add(rest.duration.max(0))
                .min(note.start.saturating_add(note.duration.max(0)));
            (b > a).then_some((a, b))
        })
        .collect::<Vec<_>>();
    let mut sorted_intervals = overlap_intervals.clone();
    sorted_intervals.sort_unstable();
    let mut pitched_overlap_union_coverage = 0_i64;
    let mut merged: Option<(i64, i64)> = None;
    for (start, end) in sorted_intervals {
        match merged {
            None => merged = Some((start, end)),
            Some((merged_start, merged_end)) if start <= merged_end => {
                merged = Some((merged_start, merged_end.max(end)));
            }
            Some((merged_start, merged_end)) => {
                pitched_overlap_union_coverage = pitched_overlap_union_coverage
                    .saturating_add(merged_end.saturating_sub(merged_start));
                merged = Some((start, end));
            }
        }
    }
    if let Some((merged_start, merged_end)) = merged {
        pitched_overlap_union_coverage = pitched_overlap_union_coverage
            .saturating_add(merged_end.saturating_sub(merged_start));
    }
    pitched_overlap_union_coverage =
        pitched_overlap_union_coverage.min(rest.duration.max(0));
    let mut sorted_geometry_intervals = pitched_geometry_intervals;
    sorted_geometry_intervals.sort_unstable();
    let mut pitched_geometry_union_coverage = 0_i64;
    let mut merged_geometry: Option<(i64, i64)> = None;
    for (start, end) in sorted_geometry_intervals {
        match merged_geometry {
            None => merged_geometry = Some((start, end)),
            Some((merged_start, merged_end)) if start <= merged_end => {
                merged_geometry = Some((merged_start, merged_end.max(end)));
            }
            Some((merged_start, merged_end)) => {
                pitched_geometry_union_coverage = pitched_geometry_union_coverage
                    .saturating_add(merged_end.saturating_sub(merged_start));
                merged_geometry = Some((start, end));
            }
        }
    }
    if let Some((merged_start, merged_end)) = merged_geometry {
        pitched_geometry_union_coverage = pitched_geometry_union_coverage
            .saturating_add(merged_end.saturating_sub(merged_start));
    }
    pitched_geometry_union_coverage =
        pitched_geometry_union_coverage.min(rest.duration.max(0));
    let mut overlap_events = Vec::<(i64, i32)>::new();
    for (start, end) in &overlap_intervals {
        overlap_events.push((*start, 1));
        overlap_events.push((*end, -1));
    }
    overlap_events.sort_unstable_by_key(|(time, delta)| (*time, *delta));
    let mut active = 0_i32;
    let mut pitched_overlap_multiplicity_peak = 0_usize;
    for (_time, delta) in overlap_events {
        active += delta;
        pitched_overlap_multiplicity_peak =
            pitched_overlap_multiplicity_peak.max(active.max(0) as usize);
    }
    let has_real_voice1_rest_overlap = rest_overlaps
        .iter()
        .any(|note| note.voice.trim() == "1");
    AtomicVisibleRestRoleObservation {
        pitched_overlap_count: pitched_overlaps.len(),
        rest_overlap_count: rest_overlaps.len(),
        same_voice_pitched_overlap_count,
        pitched_overlap_coverage,
        pitched_overlap_union_coverage,
        pitched_geometry_union_coverage,
        pitched_overlap_multiplicity_peak,
        has_real_voice1_rest_overlap,
    }
}

fn atomic_visible_rest_is_shadow_bookkeeping_candidate(
    rest: &AtomicNote,
    observation: AtomicVisibleRestRoleObservation,
) -> bool {
    rest.visible
        && rest.pitch.is_none()
        && rest.duration > 0
        && !rest.is_chord
        && rest.voice.trim() != "1"
        && observation.pitched_overlap_count > 0
        // r1181: this early raw-voice owner may retire a whole visible rest carrier
        // only when the concrete other-voice occupancy covers the complete carrier
        // interval. Partial or multiplicity-inflated overlap is not whole-R
        // rhythmic-dispensability evidence; later bound FinalChord projection owns
        // any interval split after canonical voice binding.
        && observation.pitched_overlap_union_coverage >= rest.duration.max(1)
        && (observation.same_voice_pitched_overlap_count == 0
            || observation.has_real_voice1_rest_overlap)
}


fn atomic_rest_has_physical_chord_member_dependency(
    rest: &AtomicNote,
    notes: &[AtomicNote],
) -> bool {
    let Some(rest_index) = notes.iter().position(|note| {
        note.note_start == rest.note_start && note.note_end == rest.note_end
    }) else {
        // The caller normally passes a rest borrowed from the same parsed note list.
        // If that identity contract is ever violated, stay fail-closed rather than
        // authorizing removal of a potentially structural MusicXML carrier.
        return true;
    };
    notes.get(rest_index.saturating_add(1)).is_some_and(|note| {
        note.measure == rest.measure && note.is_chord
    })
}

fn atomic_visible_rest_is_shadow_bookkeeping_candidate_with_voice_binding(
    rest: &AtomicNote,
    notes: &[AtomicNote],
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> bool {
    let Some(rest_role) = voice_plan.role_for_label(rest.voice.trim()) else {
        return false;
    };
    let Some(rest_channel) = voice_plan.channel_for_label(rest.voice.trim()) else {
        return false;
    };
    let rest_start = rest.start;
    let rest_end = rest.start.saturating_add(rest.duration.max(0));
    let pitched_overlaps = notes
        .iter()
        .filter(|note| {
            note.measure == rest.measure
                && note.staff == rest.staff
                && note.pitch.is_some()
                && note.visible
                && note.start < rest_end
                && note.start.saturating_add(note.duration.max(0)) > rest_start
        })
        .collect::<Vec<_>>();
    let rest_overlaps = notes
        .iter()
        .filter(|note| {
            note.measure == rest.measure
                && note.staff == rest.staff
                && note.pitch.is_none()
                && note.visible
                && note.start < rest_end
                && note.start.saturating_add(note.duration.max(0)) > rest_start
        })
        .collect::<Vec<_>>();
    let same_channel_pitched_overlap_count = pitched_overlaps
        .iter()
        .filter(|note| voice_plan.channel_for_label(note.voice.trim()) == Some(rest_channel))
        .count();
    let pitched_overlap_coverage = pitched_overlaps.iter().fold(0_i64, |sum, note| {
        let start = note.start.max(rest_start);
        let end = note.start.saturating_add(note.duration.max(0)).min(rest_end);
        sum.saturating_add(end.saturating_sub(start).max(0))
    });
    let has_real_primary_rest_overlap = rest_overlaps.iter().any(|note| {
        voice_plan.label_is_shadow_rest_primary_exempt(note.voice.trim()) == Some(true)
    });
    rest.visible
        && rest.pitch.is_none()
        && rest.duration > 0
        && !rest.is_chord
        && !rest_role.shadow_rest_primary_exempt()
        // r1144 reuses the accepted r1136 chord-anchor safety rule. A non-chord
        // rest can be the physical MusicXML anchor for immediately following
        // <chord/> members; replacing that carrier with <forward> would reset the
        // anchor state and change pitched occurrence identity. Such a carrier is
        // structurally obligated and therefore not a shadow-purge candidate.
        && !atomic_rest_has_physical_chord_member_dependency(rest, notes)
        && !pitched_overlaps.is_empty()
        && pitched_overlap_coverage.saturating_mul(2) >= rest.duration.max(1)
        && (same_channel_pitched_overlap_count == 0 || has_real_primary_rest_overlap)
}

pub(crate) fn final_visible_shadow_bookkeeping_regression_candidates_with_voice_binding(
    xml: &str,
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> Option<usize> {
    if !voice_plan.is_bound() || parse_musicxml(xml).is_err() {
        return None;
    }
    let notes = parse_atomic_notes(xml);
    Some(
        notes
            .iter()
            .filter(|rest| {
                atomic_visible_rest_is_shadow_bookkeeping_candidate_with_voice_binding(
                    rest,
                    &notes,
                    voice_plan,
                )
            })
            .count(),
    )
}


pub(crate) fn final_visible_shadow_bookkeeping_replacements_with_voice_binding(
    xml: &str,
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> Option<Vec<(usize, usize, String)>> {
    if !voice_plan.is_bound() || parse_musicxml(xml).is_err() {
        return None;
    }
    let notes = parse_atomic_notes(xml);
    let mut replacements = Vec::<(usize, usize, String)>::new();
    for rest in notes.iter().filter(|rest| {
        atomic_visible_rest_is_shadow_bookkeeping_candidate_with_voice_binding(
            rest,
            &notes,
            voice_plan,
        )
    }) {
        replacements.push((
            rest.note_start,
            rest.note_end,
            render_forward_fragment(rest.duration as i32),
        ));
    }
    Some(replacements)
}

pub(crate) fn newly_introduced_final_visible_shadow_bookkeeping_candidates_with_voice_binding(
    before_xml: &str,
    after_xml: &str,
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> Option<usize> {
    let before_rows =
        final_visible_shadow_bookkeeping_regression_candidate_rows(before_xml, voice_plan)?;
    let after_rows =
        final_visible_shadow_bookkeeping_regression_candidate_rows(after_xml, voice_plan)?;
    let mut remaining = std::collections::BTreeMap::<String, usize>::new();
    for row in before_rows {
        *remaining.entry(row).or_default() += 1;
    }
    let mut introduced = 0usize;
    for row in after_rows {
        match remaining.get_mut(&row) {
            Some(count) if *count > 0 => *count -= 1,
            _ => introduced = introduced.saturating_add(1),
        }
    }
    Some(introduced)
}


#[cfg(test)]
pub(crate) fn final_visible_shadow_bookkeeping_regression_candidates(
    xml: &str,
) -> Option<usize> {
    if parse_musicxml(xml).is_err() {
        return None;
    }
    let notes = parse_atomic_notes(xml);
    Some(
        notes
            .iter()
            .filter(|rest| {
                let observation = observe_atomic_visible_rest_role(rest, &notes);
                atomic_visible_rest_is_shadow_bookkeeping_candidate(rest, observation)
            })
            .count(),
    )
}

fn rest_role_counterfactual_measure_rows(notes: &[AtomicNote], measure: &str) -> Vec<String> {
    let mut rows = Vec::new();
    for rest in notes.iter().filter(|rest| {
        rest.visible
            && rest.pitch.is_none()
            && rest.duration > 0
            && !rest.is_chord
            && rest.measure == measure
    }) {
        let observation = observe_atomic_visible_rest_role(rest, notes);
        let verdict = atomic_visible_rest_is_shadow_bookkeeping_candidate(rest, observation);
        rows.push(format!(
            "m{}@{}+{}:s{}:v{} other_voice={} same_voice={} summed={} other_voice_union={} geometry_union={} peak={} voice1_rest={} verdict={}",
            rest.measure,
            rest.start,
            rest.duration,
            rest.staff,
            rest.voice,
            observation.pitched_overlap_count,
            observation.same_voice_pitched_overlap_count,
            observation.pitched_overlap_coverage,
            observation.pitched_overlap_union_coverage,
            observation.pitched_geometry_union_coverage,
            observation.pitched_overlap_multiplicity_peak,
            if observation.has_real_voice1_rest_overlap { "yes" } else { "no" },
            if verdict { "shadow" } else { "keep" },
        ));
    }
    rows
}

fn staff_voice_rest_role_counterfactual_decision_audit(
    xml: &str,
    spine: &CandidateMaterialLineageSpine,
) -> String {
    use std::fmt::Write as _;

    let mut out = String::new();
    let _ = writeln!(out, "StaffVoiceRestRoleCounterfactualAudit: read_only");
    let _ = writeln!(out, "CurrentCounterfactualCausality: AVAILABLE");
    let _ = writeln!(
        out,
        "CurrentCounterfactualCausalityScope: committed_r788_atomic_model_only"
    );
    let _ = writeln!(out, "HistoricalR787Attribution: UNAVAILABLE");
    let _ = writeln!(
        out,
        "HistoricalR787AttributionReason: rejected_r787_intermediate_state_not_lineage_resident"
    );
    let _ = writeln!(out, "StaffVoiceRestRoleCounterfactualMusicMutation: nein");
    let _ = writeln!(out, "StaffVoiceRestRoleCounterfactualWriterAuthority: none");
    let _ = writeln!(out, "StaffVoiceRestRoleCounterfactualClassifierChanged: nein");
    let _ = writeln!(out, "StaffVoiceRestRoleCounterfactualReleaseDecisionChanged: nein");

    if parse_musicxml(xml).is_err() {
        let _ = writeln!(out, "DecisionAuditGreen: nein");
        let _ = writeln!(out, "ChordCause: AMBIGUOUS");
        let _ = writeln!(out, "CurrentRestRoleCause: AMBIGUOUS");
        let _ = writeln!(out, "ReleaseCoupling: AMBIGUOUS");
        let _ = writeln!(out, "NextMutationClass: NONE");
        return out;
    }

    let notes = parse_atomic_notes(xml);
    let staff_voice_receipts = spine
        .receipts
        .iter()
        .filter(|receipt| receipt.staff_changes > 0 || receipt.voice_changes > 0)
        .collect::<Vec<_>>();
    let mut changed_sources = staff_voice_receipts
        .iter()
        .flat_map(|receipt| receipt.changed_source_ordinals.iter().copied())
        .collect::<Vec<_>>();
    changed_sources.sort_unstable();
    changed_sources.dedup();
    let _ = writeln!(
        out,
        "SelectedStaffVoiceMutationReceipts: {}",
        staff_voice_receipts.len()
    );
    let _ = writeln!(
        out,
        "SelectedStaffVoiceChangedSources: [{}]",
        changed_sources
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
    for receipt in &staff_voice_receipts {
        let _ = writeln!(
            out,
            "SelectedStaffVoiceReceipt owner={} staff_changes={} voice_changes={} changed_sources=[{}]",
            receipt.owner,
            receipt.staff_changes,
            receipt.voice_changes,
            receipt
                .changed_source_ordinals
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(","),
        );
    }

    let mut mismatches = Vec::<(usize, usize)>::new();
    let mut serialized_anchor = None::<usize>;
    let mut previous_note_end = 0_usize;
    let mut previous_measure = None::<String>;
    let mut missing_anchor_count = 0_usize;
    let mut duration_conflict_count = 0_usize;
    for (note_index, note) in notes.iter().enumerate() {
        if previous_measure.as_deref() != Some(note.measure.as_str())
            || xml
                .get(previous_note_end..note.note_start)
                .is_some_and(|between| between.contains("<backup") || between.contains("<forward"))
        {
            serialized_anchor = None;
        }
        if note.is_chord {
            if let Some(host_index) = serialized_anchor {
                let host = &notes[host_index];
                if note.duration > host.duration {
                    duration_conflict_count = duration_conflict_count.saturating_add(1);
                }
                if host.voice != note.voice {
                    mismatches.push((note_index, host_index));
                }
            } else {
                missing_anchor_count = missing_anchor_count.saturating_add(1);
            }
        } else {
            serialized_anchor = Some(note_index);
        }
        previous_note_end = note.note_end;
        previous_measure = Some(note.measure.clone());
    }
    let _ = writeln!(out, "ChordHostVoiceMismatchCount: {}", mismatches.len());
    let _ = writeln!(out, "ChordMissingSerializedAnchorCount: {missing_anchor_count}");
    let _ = writeln!(out, "ChordDurationConflictCount: {duration_conflict_count}");

    if mismatches.is_empty() {
        let clean_chord_state = missing_anchor_count == 0 && duration_conflict_count == 0;
        let _ = writeln!(out, "DecisionAuditGreen: {}", if clean_chord_state { "ja" } else { "nein" });
        let _ = writeln!(
            out,
            "ChordCause: {}",
            if clean_chord_state { "NONE" } else { "AMBIGUOUS" }
        );
        let _ = writeln!(out, "CurrentRestRoleCause: NO_DELTA");
        let _ = writeln!(
            out,
            "ReleaseCoupling: {}",
            if clean_chord_state { "NONE" } else { "AMBIGUOUS" }
        );
        let _ = writeln!(out, "NextMutationClass: NONE");
        return out;
    }

    let mut aggregate_chord_voice_only = true;
    let mut aggregate_lineage_bound = true;
    let mut aggregate_current_to_chord_correct_shadow_delta = false;
    let mut aggregate_current_to_chord_correct_union_constant = true;
    let mut aggregate_current_to_chord_correct_geometry_union_constant = true;
    let mut aggregate_current_to_chord_correct_summed_constant = true;

    for (ordinal, (member_index, host_index)) in mismatches.iter().copied().enumerate() {
        let member = &notes[member_index];
        let host = &notes[host_index];
        let pitch = member
            .pitch
            .as_ref()
            .map(|pitch| pitch.label.as_str())
            .unwrap_or("-");
        let source_matches = spine
            .attacks
            .iter()
            .filter(|attack| {
                attack.exists
                    && attack.measure.to_string() == member.measure
                    && attack.onset == member.start
                    && member
                        .pitch
                        .as_ref()
                        .is_some_and(|member_pitch| attack.pitch == member_pitch.midi)
            })
            .collect::<Vec<_>>();
        let source_match_ids = source_matches
            .iter()
            .map(|attack| attack.source_event_id.0)
            .collect::<Vec<_>>();
        let source_match_rows = source_matches
            .iter()
            .map(|attack| {
                format!(
                    "{}:s{}v{}",
                    attack.source_event_id.0, attack.staff, attack.voice
                )
            })
            .collect::<Vec<_>>();
        let changed_source_match = changed_sources.len() == 1
            && source_match_ids
                .iter()
                .any(|source_event_id| changed_sources.contains(source_event_id));
        aggregate_lineage_bound &= changed_source_match;
        let _ = writeln!(
            out,
            "ChordMismatch#{} measure={} onset={} duration={} pitch={} host=s{}v{} member=s{}v{} source_matches=[{}] lineage_staff_voice_changed={}",
            ordinal + 1,
            member.measure,
            member.start,
            member.duration,
            pitch,
            host.staff,
            host.voice,
            member.staff,
            member.voice,
            source_match_rows.join(","),
            if changed_source_match { "yes" } else { "no" },
        );
        let states = [
            ("A_HOST_STAFF_HOST_VOICE", host.staff.clone(), host.voice.clone()),
            ("B_MEMBER_STAFF_HOST_VOICE", member.staff.clone(), host.voice.clone()),
            ("C_HOST_STAFF_MEMBER_VOICE", host.staff.clone(), member.voice.clone()),
            ("D_MEMBER_STAFF_MEMBER_VOICE", member.staff.clone(), member.voice.clone()),
        ];
        let mut state_verdicts = Vec::<Vec<bool>>::new();
        let mut state_summed = Vec::<Vec<i64>>::new();
        let mut state_union = Vec::<Vec<i64>>::new();
        let mut state_geometry_union = Vec::<Vec<i64>>::new();
        let mut state_chord_conflicts = Vec::<bool>::new();
        for (label, staff, voice) in states {
            let mut projected = notes.clone();
            projected[member_index].staff = staff.clone();
            projected[member_index].voice = voice.clone();
            let chord_conflict = voice != host.voice;
            state_chord_conflicts.push(chord_conflict);
            let mut verdicts = Vec::new();
            let mut summed = Vec::new();
            let mut union = Vec::new();
            let mut geometry_union = Vec::new();
            for rest in projected.iter().filter(|rest| {
                rest.visible
                    && rest.pitch.is_none()
                    && rest.duration > 0
                    && !rest.is_chord
                    && rest.measure == member.measure
            }) {
                let observation = observe_atomic_visible_rest_role(rest, &projected);
                verdicts.push(atomic_visible_rest_is_shadow_bookkeeping_candidate(rest, observation));
                summed.push(observation.pitched_overlap_coverage);
                union.push(observation.pitched_overlap_union_coverage);
                geometry_union.push(observation.pitched_geometry_union_coverage);
            }
            let _ = writeln!(
                out,
                "ChordCounterfactual#{} state={} staff={} voice={} host_voice_conflict={} shadow_candidates={} pitch_onset_duration_geometry=unchanged",
                ordinal + 1,
                label,
                staff,
                voice,
                if chord_conflict { "yes" } else { "no" },
                verdicts.iter().filter(|verdict| **verdict).count(),
            );
            for row in rest_role_counterfactual_measure_rows(&projected, &member.measure) {
                let _ = writeln!(out, "ChordCounterfactual#{} {} RestRole {}", ordinal + 1, label, row);
            }
            state_verdicts.push(verdicts);
            state_summed.push(summed);
            state_union.push(union);
            state_geometry_union.push(geometry_union);
        }
        let voice_only_pattern = state_chord_conflicts == vec![false, false, true, true];
        aggregate_chord_voice_only &= voice_only_pattern;
        let current_state_index = 3;
        let chord_correct_state_index = 1;
        let current_to_chord_correct_shadow_delta =
            state_verdicts[current_state_index] != state_verdicts[chord_correct_state_index];
        if current_to_chord_correct_shadow_delta {
            aggregate_current_to_chord_correct_shadow_delta = true;
        }
        if state_union[current_state_index] != state_union[chord_correct_state_index] {
            aggregate_current_to_chord_correct_union_constant = false;
        }
        if state_geometry_union[current_state_index]
            != state_geometry_union[chord_correct_state_index]
        {
            aggregate_current_to_chord_correct_geometry_union_constant = false;
        }
        if state_summed[current_state_index] != state_summed[chord_correct_state_index] {
            aggregate_current_to_chord_correct_summed_constant = false;
        }
        let _ = writeln!(
            out,
            "ChordCounterfactual#{} CurrentToChordCorrectRestRoleDelta: {}",
            ordinal + 1,
            if current_to_chord_correct_shadow_delta { "yes" } else { "no" }
        );
    }

    let chord_cause = if mismatches.len() == 1
        && missing_anchor_count == 0
        && duration_conflict_count == 0
        && aggregate_chord_voice_only
        && aggregate_lineage_bound
    {
        "VOICE_ONLY"
    } else {
        "AMBIGUOUS"
    };
    let current_rest_role_cause = if !aggregate_current_to_chord_correct_shadow_delta {
        "NO_DELTA"
    } else if !aggregate_current_to_chord_correct_geometry_union_constant {
        "AMBIGUOUS"
    } else if aggregate_current_to_chord_correct_union_constant
        && !aggregate_current_to_chord_correct_summed_constant
    {
        "MULTIPLICITY_AND_VOICE"
    } else {
        "VOICE_REPRESENTATION_ONLY"
    };
    let release_coupling = match (chord_cause, current_rest_role_cause) {
        ("VOICE_ONLY", "NO_DELTA") => "CHORD_ONLY",
        ("VOICE_ONLY", "VOICE_REPRESENTATION_ONLY" | "MULTIPLICITY_AND_VOICE") => {
            "CHORD_PLUS_RESTROLE"
        }
        _ => "AMBIGUOUS",
    };
    let _ = writeln!(
        out,
        "CurrentToChordCorrectRestRoleDelta: {}",
        if aggregate_current_to_chord_correct_shadow_delta {
            "yes"
        } else {
            "no"
        }
    );
    let _ = writeln!(
        out,
        "CurrentToChordCorrectOtherVoiceUnionConstant: {}",
        if aggregate_current_to_chord_correct_union_constant {
            "yes"
        } else {
            "no"
        }
    );
    let _ = writeln!(
        out,
        "CurrentToChordCorrectGeometryUnionConstant: {}",
        if aggregate_current_to_chord_correct_geometry_union_constant {
            "yes"
        } else {
            "no"
        }
    );
    let _ = writeln!(
        out,
        "CurrentToChordCorrectSummedCoverageConstant: {}",
        if aggregate_current_to_chord_correct_summed_constant {
            "yes"
        } else {
            "no"
        }
    );
    let _ = writeln!(out, "ChordCause: {chord_cause}");
    let _ = writeln!(out, "CurrentRestRoleCause: {current_rest_role_cause}");
    let _ = writeln!(out, "ReleaseCouplingBasis: CurrentCounterfactualCausality");
    let _ = writeln!(out, "HistoricalR787AttributionExcludedFromCoupling: yes");
    let _ = writeln!(out, "ReleaseCoupling: {release_coupling}");
    let _ = writeln!(
        out,
        "NextMutationClass: {}",
        match release_coupling {
            "CHORD_ONLY" => "CHORD_AUTHORITY_CLOSURE",
            "CHORD_PLUS_RESTROLE" => "SINGLE_ATOMIC_CHORD_RESTROLE_CLOSURE",
            _ => "NONE",
        }
    );
    let _ = writeln!(out, "DecisionAuditGreen: ja");
    out
}

pub(crate) fn final_visible_shadow_bookkeeping_regression_candidate_rows(
    xml: &str,
    voice_plan: &crate::structural_write_authority::FrozenSerializedVoicePlan,
) -> Option<Vec<String>> {
    if !voice_plan.is_bound() || parse_musicxml(xml).is_err() {
        return None;
    }
    let notes = parse_atomic_notes(xml);
    let mut rows = Vec::new();
    for rest in notes
        .iter()
        .filter(|rest| rest.visible && rest.pitch.is_none() && rest.duration > 0 && !rest.is_chord)
    {
        if !atomic_visible_rest_is_shadow_bookkeeping_candidate_with_voice_binding(
            rest,
            &notes,
            voice_plan,
        ) {
            continue;
        }
        // r805 keeps the r788 diagnostic schema, but candidate membership is now
        // selected by the exact same frozen-voice predicate as the release count.
        // The legacy observation below is diagnostic only and owns no verdict.
        let observation = observe_atomic_visible_rest_role(rest, &notes);
        let overlaps = notes
            .iter()
            .filter(|note| {
                note.visible
                    && note.pitch.is_some()
                    && note.measure == rest.measure
                    && note.staff == rest.staff
                    && upper_staff_rest_stack_intervals_overlap(
                        rest.start,
                        rest.duration,
                        note.start,
                        note.duration,
                    )
            })
            .map(|note| {
                let pitch = note
                    .pitch
                    .as_ref()
                    .map(|pitch| pitch.label.as_str())
                    .unwrap_or("-");
                format!(
                    "{}@{}+{}:v{}:s{}{}",
                    pitch,
                    note.start,
                    note.duration,
                    note.voice,
                    note.staff,
                    if note.is_chord { ":chord" } else { "" },
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        rows.push(format!(
            "m{}@{}+{}:s{}:v{} pitched_overlap_count={} rest_overlap_count={} same_voice_pitched_overlap_count={} pitched_overlap_coverage={} pitched_overlap_summed_coverage={} pitched_overlap_union_coverage={} pitched_geometry_union_coverage={} overlap_multiplicity_peak={} has_real_voice1_rest_overlap={} overlaps=[{}]",
            rest.measure,
            rest.start,
            rest.duration,
            rest.staff,
            rest.voice,
            observation.pitched_overlap_count,
            observation.rest_overlap_count,
            observation.same_voice_pitched_overlap_count,
            observation.pitched_overlap_coverage,
            observation.pitched_overlap_coverage,
            observation.pitched_overlap_union_coverage,
            observation.pitched_geometry_union_coverage,
            observation.pitched_overlap_multiplicity_peak,
            yes_no(observation.has_real_voice1_rest_overlap),
            overlaps,
        ));
    }
    Some(rows)
}




fn infer_current_clef_sign_for_staff_at(xml: &str, note_start: usize, staff: &str) -> String {
    let mut current = if staff.trim() == "2" { "F".to_string() } else { "G".to_string() };
    let upper_bound = note_start.min(xml.len());
    let mut scan = 0usize;
    while scan < upper_bound {
        let Some(rel) = xml[scan..upper_bound].find("<clef") else { break; };
        let clef_start = scan + rel;
        let Some(rel_end) = xml[clef_start..upper_bound].find("</clef>") else { break; };
        let clef_end = clef_start + rel_end + "</clef>".len();
        let block = &xml[clef_start..clef_end];
        let number_matches = if let Some(pos) = block.find("number=\"") {
            let a = pos + "number=\"".len();
            block[a..].find('"').is_some_and(|b| block[a..a + b].trim() == staff.trim())
        } else {
            staff.trim() == "1"
        };
        if number_matches
            && let Some(sign) = tag_text(block, "sign")
        {
            current = sign.trim().to_string();
        }
        scan = clef_end;
    }
    current
}

fn standard_rest_display_pitch_for_context(
    rest_fragment: &str,
    staff: &str,
    clef_sign: &str,
) -> (String, i32, String) {
    let rest_type = tag_text(rest_fragment, "type").unwrap_or_else(|| "quarter".to_string());

    // Visible rest height is a fixed visual staff position. MusicXML display
    // pitches must be clef-aware to land on the same print slot.
    let plan = render::rest_placement::fixed_staff_rest_placement_plan(
        staff,
        clef_sign,
        rest_type.trim(),
    );
    (
        plan.anchor.step.to_string(),
        plan.anchor.octave,
        plan.placement_class,
    )
}

fn standard_rest_display_pitch_for_atomic_context(
    xml: &str,
    rest: &AtomicNote,
) -> (String, i32, String) {
    let rest_fragment = xml.get(rest.note_start..rest.note_end).unwrap_or_default();
    let clef_sign = infer_current_clef_sign_for_staff_at(xml, rest.note_start, &rest.staff);
    standard_rest_display_pitch_for_context(rest_fragment, &rest.staff, &clef_sign)
}

fn apply_standard_rest_display_pitch(rest_fragment: &str, step: &str, octave: i32) -> String {
    crate::structural_write_authority::apply_standard_rest_display_pitch_authorized(rest_fragment, step, octave)
}


#[cfg(test)]
mod sixteenth_rest_vertical_clearance_retirement_regression_tests {
    use super::{parse_atomic_notes, standard_rest_display_pitch_for_atomic_context};

    fn rest_anchor(xml: &str) -> (String, i32, String) {
        let notes = parse_atomic_notes(xml);
        let rest = notes
            .iter()
            .find(|note| note.visible && note.pitch.is_none() && note.duration == 24)
            .expect("synthetic visible 16th rest");
        standard_rest_display_pitch_for_atomic_context(xml, rest)
    }

    #[test]
    fn bass_sixteenth_rest_keeps_canonical_middle_slot_before_following_flat_state_change() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><key><fifths>0</fifths></key><time><beats>1</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>B</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><pitch><step>B</step><alter>-1</alter><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note></measure></part></score-partwise>";
        let (step, octave, class) = rest_anchor(xml);
        assert_eq!((step.as_str(), octave), ("D", 3));
        assert_eq!(class, "fixed_visual_table_staff2_fclef_16th_rest");
    }

    #[test]
    fn bass_sixteenth_rest_keeps_canonical_middle_slot_before_following_natural_against_sharp_key() {
        let xml = "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><key><fifths>2</fifths></key><time><beats>1</beats><beat-type>4</beat-type></time><staves>2</staves><clef number=\"2\"><sign>F</sign><line>4</line></clef></attributes><note><pitch><step>F</step><alter>1</alter><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note><note><pitch><step>F</step><octave>3</octave></pitch><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff><accidental>natural</accidental></note><note><rest/><duration>24</duration><voice>2</voice><type>16th</type><staff>2</staff></note></measure></part></score-partwise>";
        let (step, octave, class) = rest_anchor(xml);
        assert_eq!((step.as_str(), octave), ("D", 3));
        assert_eq!(class, "fixed_visual_table_staff2_fclef_16th_rest");
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct WholeRestDisplayCoordinateReadbackVerification {
    checked_rows: usize,
    residual_candidates: usize,
    measure_yes_rows: usize,
    missing_coordinate_rows: usize,
}

fn verify_whole_rest_display_coordinate_readback_truth(
    xml: &str,
) -> WholeRestDisplayCoordinateReadbackVerification {
    // Verifier-only closure. Productive placement belongs to the typed final
    // rest writer; this pass may only prove that no legacy self-closing/
    // measure=yes whole-rest candidate survived into the actual PDF source.
    let mut audit = WholeRestDisplayCoordinateReadbackVerification::default();
    let mut scan_measure = 0usize;
    while let Some((measure_start, measure_end, _scope_number)) =
        find_next_measure_bounds(xml, scan_measure)
    {
        let measure = &xml[measure_start..measure_end];
        let expected_duration = infer_measure_duration_from_measure_xml(measure)
            .unwrap_or(384)
            .max(1) as i64;
        let mut scan_note = 0usize;
        while let Some(rel_note) = measure[scan_note..].find("<note") {
            let local_note_start = scan_note + rel_note;
            let abs_note_start = measure_start + local_note_start;
            let Some(rel_note_end) = measure[local_note_start..].find("</note>") else {
                break;
            };
            let local_note_end = local_note_start + rel_note_end + "</note>".len();
            let abs_note_end = measure_start + local_note_end;
            let note_fragment = &xml[abs_note_start..abs_note_end];
            scan_note = local_note_end;

            if !note_fragment.contains("<rest")
                || note_fragment.contains("<pitch>")
                || note_fragment.contains("<chord")
                || note_fragment.contains("print-object=\"no\"")
                || note_fragment.contains("print-object='no'")
            {
                continue;
            }
            let duration = tag_text_i64(note_fragment, "duration")
                .unwrap_or(0)
                .max(0);
            let rest_type = tag_text(note_fragment, "type").unwrap_or_default();
            let is_whole_like = matches!(
                rest_type.trim(),
                "whole" | "breve" | "long" | "maxima"
            ) || duration >= expected_duration;
            if !is_whole_like {
                continue;
            }

            audit.checked_rows = audit.checked_rows.saturating_add(1);
            let has_measure_yes = note_fragment.contains("<rest measure=\"yes\"")
                || note_fragment.contains("<rest measure='yes'");
            let has_display_step = note_fragment.contains("<display-step>");
            let has_display_octave = note_fragment.contains("<display-octave>");
            let missing_coordinate = !has_display_step || !has_display_octave;
            if has_measure_yes {
                audit.measure_yes_rows = audit.measure_yes_rows.saturating_add(1);
            }
            if missing_coordinate {
                audit.missing_coordinate_rows =
                    audit.missing_coordinate_rows.saturating_add(1);
            }
            if has_measure_yes || missing_coordinate {
                audit.residual_candidates = audit.residual_candidates.saturating_add(1);
            }
        }
        scan_measure = measure_end;
    }
    audit
}


fn current_rest_display_position(fragment: &str) -> String {
    match (tag_text(fragment, "display-step"), tag_text(fragment, "display-octave")) {
        (Some(step), Some(oct)) => format!("{}{}", step.trim(), oct.trim()),
        _ => "auto_renderer_or_voice_default".to_string(),
    }
}


#[derive(Debug, Clone, Default)]
struct PianoFinalRestGlyphCanonicalHeightWriteAudit {
    source_label: String,
    output_path: String,
    visible_rests_before: usize,
    visible_rests_after: usize,
    visible_pitched_before: usize,
    visible_pitched_after: usize,
    hidden_pitched_before: usize,
    hidden_pitched_after: usize,
    hidden_rests_before: usize,
    hidden_rests_after: usize,
    coordinates_before: usize,
    visible_rests: usize,
    whole_like_rows: usize,
    half_rows: usize,
    quarter_rows: usize,
    eighth_rows: usize,
    sixteenth_rows: usize,
    thirty_second_or_shorter_rows: usize,
    dotted_rows: usize,
    double_dotted_rows: usize,
    already_canonical_rows: usize,
    write_candidates: usize,
    written_rows: usize,
    sounding_stable: bool,
    parse_after_ok: bool,
    backup_forward_after_ok: bool,
    raw_cursor_hard_before: usize,
    raw_cursor_hard_after: usize,
    raw_cursor_stable: bool,
    written: bool,
    verdict: String,
    done_fragment: String,
    pipeline_line: String,
}

fn build_piano_final_rest_glyph_canonical_height_write(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> anyhow::Result<(String, PianoFinalRestGlyphCanonicalHeightWriteAudit, crate::master_state_identity::RenderTransitionReceipts)> {
    let mut audit = PianoFinalRestGlyphCanonicalHeightWriteAudit {
        source_label: source_label.to_string(),
        output_path: output_path.display().to_string(),
        ..PianoFinalRestGlyphCanonicalHeightWriteAudit::default()
    };
    let notes = parse_atomic_notes(xml);
    let (before_events, before_backup_forward_ok) = collect_render_xml_note_events(xml);
    let before_signature = render_visible_sounding_signature(&before_events);
    audit.visible_rests_before = before_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    audit.visible_pitched_before = before_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    audit.hidden_pitched_before = before_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    audit.hidden_rests_before = before_events.iter().filter(|event| event.is_hidden && event.is_rest).count();

    let mut replacements = Vec::<(usize, usize, String)>::new();
    let mut occupied = Vec::<(usize, usize)>::new();
    for rest in notes.iter().filter(|note| note.visible && note.pitch.is_none() && note.duration > 0 && !note.is_chord) {
        let Some(fragment) = xml.get(rest.note_start..rest.note_end) else { continue; };
        audit.visible_rests = audit.visible_rests.saturating_add(1);
        let rest_type = tag_text(fragment, "type").unwrap_or_else(|| "quarter".to_string());
        let rest_type_trimmed = rest_type.trim();
        match rest_type_trimmed {
            "whole" | "breve" | "long" | "maxima" => audit.whole_like_rows = audit.whole_like_rows.saturating_add(1),
            "half" => audit.half_rows = audit.half_rows.saturating_add(1),
            "quarter" => audit.quarter_rows = audit.quarter_rows.saturating_add(1),
            "eighth" => audit.eighth_rows = audit.eighth_rows.saturating_add(1),
            "16th" => audit.sixteenth_rows = audit.sixteenth_rows.saturating_add(1),
            "32nd" | "64th" | "128th" | "256th" | "512th" | "1024th" => {
                audit.thirty_second_or_shorter_rows = audit.thirty_second_or_shorter_rows.saturating_add(1)
            }
            _ => {}
        }
        let dot_count = fragment.matches("<dot").count();
        if dot_count > 0 {
            audit.dotted_rows = audit.dotted_rows.saturating_add(1);
        }
        if dot_count > 1 {
            audit.double_dotted_rows = audit.double_dotted_rows.saturating_add(1);
        }
        let current_position = current_rest_display_position(fragment);
        if current_position != "auto_renderer_or_voice_default" {
            audit.coordinates_before = audit.coordinates_before.saturating_add(1);
        }
        let (expected_step, expected_octave, _) =
            standard_rest_display_pitch_for_atomic_context(xml, rest);
        let expected_position = format!("{expected_step}{expected_octave}");
        if current_position == expected_position {
            audit.already_canonical_rows = audit.already_canonical_rows.saturating_add(1);
            continue;
        }
        let normalized = apply_standard_rest_display_pitch(fragment, &expected_step, expected_octave);
        if normalized == fragment || occupied.iter().any(|(s, e)| rest.note_start < *e && rest.note_end > *s) {
            continue;
        }
        replacements.push((rest.note_start, rest.note_end, normalized));
        occupied.push((rest.note_start, rest.note_end));
        audit.write_candidates = audit.write_candidates.saturating_add(1);
        audit.written_rows = audit.written_rows.saturating_add(1);
    }


    let before_raw_cursor_rows = scan_raw_cursor_xml(
        "RenderXMLBeforeFinalRestGlyphCanonicalHeightWrite",
        xml,
    );
    audit.raw_cursor_hard_before = before_raw_cursor_rows
        .iter()
        .filter(|row| row.hard_violation)
        .count();

    let (candidate, receipt) = crate::structural_write_authority::apply_final_rest_glyph_height_authorized(
        xml,
        replacements,
    );
    let candidate_xml = candidate.unwrap_or_else(|| xml.to_string());
    let mut receipts = crate::master_state_identity::RenderTransitionReceipts::new();
    receipts.push_exact(receipt);
    let changed = candidate_xml != xml;
    let (after_events, after_backup_forward_ok) = collect_render_xml_note_events(&candidate_xml);
    let after_signature = render_visible_sounding_signature(&after_events);
    audit.visible_rests_after = after_events.iter().filter(|event| !event.is_hidden && event.is_rest).count();
    audit.visible_pitched_after = after_events.iter().filter(|event| !event.is_hidden && event.midi.is_some()).count();
    audit.hidden_pitched_after = after_events.iter().filter(|event| event.is_hidden && event.midi.is_some()).count();
    audit.hidden_rests_after = after_events.iter().filter(|event| event.is_hidden && event.is_rest).count();
    audit.sounding_stable = before_signature == after_signature;
    audit.parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    audit.backup_forward_after_ok = before_backup_forward_ok && after_backup_forward_ok;
    let after_raw_cursor_rows = scan_raw_cursor_xml(
        "RenderXMLAfterFinalRestGlyphCanonicalHeightWrite",
        &candidate_xml,
    );
    audit.raw_cursor_hard_after = after_raw_cursor_rows
        .iter()
        .filter(|row| row.hard_violation)
        .count();
    audit.raw_cursor_stable = before_raw_cursor_rows.len() == after_raw_cursor_rows.len()
        && before_raw_cursor_rows
            .iter()
            .zip(after_raw_cursor_rows.iter())
            .all(|(before, after)| {
                before.measure == after.measure
                    && before.expected_duration == after.expected_duration
                    && before.cursor_min == after.cursor_min
                    && before.cursor_max == after.cursor_max
                    && before.cursor_final == after.cursor_final
                    && before.partial_backup_count == after.partial_backup_count
                    && before.overflow_events == after.overflow_events
                    && before.post_expected_events == after.post_expected_events
                    && before.first_bad == after.first_bad
                    && before.trace == after.trace
                    && before.hard_violation == after.hard_violation
            });
    let invariants_ok = changed
        && audit.sounding_stable
        && audit.parse_after_ok
        && audit.backup_forward_after_ok
        && audit.raw_cursor_stable
        && audit.visible_pitched_before == audit.visible_pitched_after
        && audit.hidden_pitched_before == audit.hidden_pitched_after
        && audit.visible_rests_before == audit.visible_rests_after
        && audit.hidden_rests_before == audit.hidden_rests_after;
    audit.written = invariants_ok;
    audit.verdict = if audit.written {
        "piano_final_rest_glyph_canonical_height_write_green_render_xml_only".to_string()
    } else if !changed {
        "piano_final_rest_glyph_canonical_height_no_candidate".to_string()
    } else {
        "piano_final_rest_glyph_canonical_height_rejected_by_invariant_guard".to_string()
    };
    audit.done_fragment = format!(
        "PianoFinalRestGlyphCanonicalHeightRenderWrite: sichtbare Pausen {}; Whole/Half/Quarter/Eighth/16th/32+ {}/{}/{}/{}/{}/{}; Dotted/DoubleDotted {}/{}; Kandidaten {}; geschrieben {}; bereits kanonisch {}; KlangMutation nein",
        audit.visible_rests,
        audit.whole_like_rows,
        audit.half_rows,
        audit.quarter_rows,
        audit.eighth_rows,
        audit.sixteenth_rows,
        audit.thirty_second_or_shorter_rows,
        audit.dotted_rows,
        audit.double_dotted_rows,
        audit.write_candidates,
        if audit.written { audit.written_rows } else { 0 },
        audit.already_canonical_rows
    );
    audit.pipeline_line = format!(
        "PianoFinalRestGlyphCanonicalHeightRenderWrite v{}: Source {}; output {}; visible_rests {}; coordinates_before {}; whole/half/quarter/eighth/16th/32+ {}/{}/{}/{}/{}/{}; dotted/double_dotted {}/{}; candidates {}; written_rows {}; already_canonical {}; visible_rests {}→{}; visible_pitched {}→{}; hidden_pitched {}→{}; hidden_rests {}→{}; parse/back-forward/raw-stable {}/{}/{}; raw-hard before/after {}/{}; SoundingStable {}; RenderXmlWrite {}; MusicXmlWrite nein; PdfWrite source-selected; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        audit.source_label,
        audit.output_path,
        audit.visible_rests,
        audit.coordinates_before,
        audit.whole_like_rows,
        audit.half_rows,
        audit.quarter_rows,
        audit.eighth_rows,
        audit.sixteenth_rows,
        audit.thirty_second_or_shorter_rows,
        audit.dotted_rows,
        audit.double_dotted_rows,
        audit.write_candidates,
        if audit.written { audit.written_rows } else { 0 },
        audit.already_canonical_rows,
        audit.visible_rests_before,
        audit.visible_rests_after,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.hidden_pitched_before,
        audit.hidden_pitched_after,
        audit.hidden_rests_before,
        audit.hidden_rests_after,
        if audit.parse_after_ok { "ok" } else { "fail" },
        if audit.backup_forward_after_ok { "ok" } else { "fail" },
        if audit.raw_cursor_stable { "ok" } else { "fail" },
        audit.raw_cursor_hard_before,
        audit.raw_cursor_hard_after,
        yes_no(audit.sounding_stable),
        yes_no(audit.written),
        audit.verdict
    );
    if audit.written { Ok((candidate_xml, audit, receipts)) } else {
        receipts.discard_unadopted(crate::master_state_identity::RenderDiscardReason::LosingCandidate);
        Ok((xml.to_string(), audit, crate::master_state_identity::RenderTransitionReceipts::new()))
    }
}








#[cfg(test)]
mod final_visible_shadow_bookkeeping_regression_tests {
    use super::{
        final_visible_shadow_bookkeeping_regression_candidates,
        final_visible_shadow_bookkeeping_replacements_with_voice_binding,
        observe_atomic_visible_rest_role, parse_atomic_notes,
    };

    fn score_with_overlap(pitched_duration: i32) -> String {
        let tail = 96_i32.saturating_sub(pitched_duration);
        let tail_fragment = if tail > 0 {
            format!("<forward><duration>{tail}</duration></forward>")
        } else {
            String::new()
        };
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
    <measure number="1">
      <attributes><divisions>96</divisions><time><beats>1</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>
      <note><rest/><duration>96</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>
      <backup><duration>96</duration></backup>
      <note><rest/><duration>96</duration><voice>2</voice><type>quarter</type><staff>1</staff></note>
      <backup><duration>96</duration></backup>
      <note><pitch><step>C</step><octave>5</octave></pitch><duration>{pitched_duration}</duration><voice>3</voice><type>quarter</type><staff>1</staff></note>
      {tail_fragment}
    </measure>
  </part>
</score-partwise>"#,
        )
    }

    #[test]
    fn canonical_role_owner_detects_majority_covered_auxiliary_shadow_rest() {
        assert_eq!(
            final_visible_shadow_bookkeeping_regression_candidates(&score_with_overlap(96)),
            Some(1)
        );
    }

    #[test]
    fn canonical_role_owner_protects_minor_overlap_instead_of_preemptive_deletion() {
        assert_eq!(
            final_visible_shadow_bookkeeping_regression_candidates(&score_with_overlap(72)),
            Some(0)
        );
        assert_eq!(
            final_visible_shadow_bookkeeping_regression_candidates(&score_with_overlap(24)),
            Some(0)
        );
    }
    #[test]
    fn frozen_voice_bound_selector_is_relocatable_and_returns_identity_neutral_forward() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
    <measure number="x">
      <attributes><divisions>48</divisions><time><beats>1</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>
      <note><rest/><duration>48</duration><voice>9</voice><type>quarter</type><staff>1</staff></note>
      <backup><duration>48</duration></backup>
      <note><pitch><step>D</step><octave>5</octave></pitch><duration>48</duration><voice>7</voice><type>quarter</type><staff>1</staff></note>
    </measure>
  </part>
</score-partwise>"#;
        let voice_plan =
            crate::structural_write_authority::frozen_serialized_voice_plan_for_rest_role_replay_test(
                "7", "9",
            );
        let replacements =
            final_visible_shadow_bookkeeping_replacements_with_voice_binding(xml, &voice_plan)
                .expect("synthetic bound plan must parse");
        assert_eq!(replacements.len(), 1);
        assert!(replacements[0].2.contains("<forward>"));
        assert!(!replacements[0].2.contains("<voice>"));
        assert!(!replacements[0].2.contains("<staff>"));

        let chord_anchored = r#"<?xml version="1.0" encoding="UTF-8"?>
<score-partwise version="4.0">
  <part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
  <part id="P1">
    <measure number="x">
      <attributes><divisions>48</divisions><time><beats>1</beats><beat-type>4</beat-type></time><staves>2</staves></attributes>
      <note><rest/><duration>48</duration><voice>9</voice><type>quarter</type><staff>1</staff></note>
      <note><chord/><pitch><step>D</step><octave>5</octave></pitch><duration>48</duration><voice>7</voice><type>quarter</type><staff>1</staff></note>
    </measure>
  </part>
</score-partwise>"#;
        let anchored_replacements =
            final_visible_shadow_bookkeeping_replacements_with_voice_binding(
                chord_anchored,
                &voice_plan,
            )
            .expect("synthetic chord-anchored bound plan must parse");
        assert!(anchored_replacements.is_empty());
    }

    #[test]
    fn summed_overlap_is_not_mistaken_for_union_coverage_in_readback_metric() {
        let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"\
1\"><attributes><divisions>96</divisions><time><beats>1</beats><beat-type>4</beat-type></time></attributes><note><rest/><duration>96</duration><voice>2</voice><staff>1</staff></note><backup><duration>96</duration></backup><note><pitch><step>C</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><staff>1</staff></note><note><chord/><pitch><step>E</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><staff>1</staff></note><note><chord/><pitch><step>G</step><octave>5</octave></pitch><duration>24</duration><voice>3</voice><staff>1</staff></note><forward><duration>72</duration></forward></measure></part></score-partwise>";
        let notes = parse_atomic_notes(xml);
        let rest = notes.iter().find(|note| note.pitch.is_none()).expect("rest");
        let observation = observe_atomic_visible_rest_role(rest, &notes);
        assert_eq!(observation.pitched_overlap_coverage, 72);
        assert_eq!(observation.pitched_overlap_union_coverage, 24);
        assert_eq!(observation.pitched_geometry_union_coverage, 24);
        assert_eq!(observation.pitched_overlap_multiplicity_peak, 3);
        assert!(!super::atomic_visible_rest_is_shadow_bookkeeping_candidate(
            rest,
            observation,
        ));
    }

}


// Receipt-bearing mutation preparation belongs to the canonical structural owner.
include!("../structural_write_authority/staff_voice_rest_glyph_topology_tail_receipt_ownership.rs");
