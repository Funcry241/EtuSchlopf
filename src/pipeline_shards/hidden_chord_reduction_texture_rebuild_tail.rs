type ConsensusGestureCandidateOutcome =
    Option<(SolverCandidate, Vec<HiddenChordToneRecoveryCandidate>, Vec<String>)>;

fn reconcile_consensus_gesture_material_lineage_with_longitudinal_audit(
    before_xml: &str,
    candidate: &mut SolverCandidate,
    owner: &str,
    move_carriers: &[CandidateMaterialExactNoteMoveCarrier],
) -> anyhow::Result<()> {
    let before_attacks = candidate.material_lineage.attacks.as_ref().to_vec();
    reconcile_candidate_material_lineage_with_exact_note_moves(
        before_xml,
        candidate,
        owner,
        CandidateMaterialDeclaredEffects::consensus_gesture_relocation(),
        move_carriers,
    )?;

    let after_by_source = candidate
        .material_lineage
        .attacks
        .iter()
        .map(|attack| (attack.source_event_id, attack))
        .collect::<BTreeMap<_, _>>();
    for before in before_attacks.iter().filter(|attack| attack.exists) {
        let Some(after) = after_by_source.get(&before.source_event_id).copied() else {
            continue;
        };
        if !after.exists || before.attack_id != after.attack_id {
            continue;
        }
        if before.measure == after.measure && before.onset == after.onset {
            continue;
        }
        candidate
            .consensus_relocation_longitudinal_trace
            .push(ConsensusRelocationLongitudinalTraceRow {
                owner: owner.to_string(),
                source_event_id: before.source_event_id,
                attack_id: before.attack_id,
                measure_before: before.measure,
                measure_after_owner: after.measure,
                pitch_before: before.pitch,
                pitch_after_owner: after.pitch,
                onset_before: before.onset,
                onset_after_owner: after.onset,
                sounding_end_before: before.sounding_end,
                sounding_end_after_owner: after.sounding_end,
            });
    }
    Ok(())
}

fn consensus_probe_cell_key(cand: &HiddenChordToneRecoveryCandidate) -> (usize, usize) {
    (cand.measure, if cand.start_tick < 192 { 1 } else { 2 })
}
fn consensus_probe_candidate_has_anchor(
    cand: &HiddenChordToneRecoveryCandidate,
    slots: &[VisibleChordToneSwapSlot],
) -> bool {
    slots.iter().any(|slot| {
        slot.measure == cand.measure
            && math_time_compatible(cand, slot)
            && math_staff_compatible(cand, slot)
            && math_voice_compatible(cand, slot)
            && hand_plan_candidate_risk(cand) < 110
            && math_rhythm_safe(cand, slot)
    })
}
fn consensus_probe_issue_rule_ids(report: &CheckReport) -> Vec<String> {
    let mut ids = Vec::new();
    for issue in &report.issues {
        ids.push(issue.rule_id.clone());
    }
    ids.sort();
    ids.dedup();
    ids
}
fn consensus_probe_record_rule_ids(stats: &mut ConsensusGestureProbeStats, report: &CheckReport) -> String {
    let ids = consensus_probe_issue_rule_ids(report);
    for id in &ids {
        *stats.rejected_rule_hist.entry(id.clone()).or_insert(0) += 1;
    }
    if ids.is_empty() {
        "RuleIds keine".to_string()
    } else {
        format!("RuleIds {}", ids.join("+"))
    }
}
fn consensus_probe_anchor_for_tone<'a>(tone: &HiddenChordToneRecoveryCandidate, slots: &'a [VisibleChordToneSwapSlot]) -> Option<&'a VisibleChordToneSwapSlot> {
    slots.iter()
        .filter(|slot| slot.measure == tone.measure)
        .filter(|slot| (slot.start_tick - tone.start_tick).abs() <= 48)
        .filter(|slot| slot.staff == tone.staff || midi_distance(slot.midi, tone.midi) <= 18)
        .min_by_key(|slot| {
            let time = (slot.start_tick - tone.start_tick).abs();
            let staff = if slot.staff == tone.staff { 0 } else { 96 };
            let voice = if slot.voice == tone.voice { 0 } else { 24 };
            let midi = midi_distance(slot.midi, tone.midi) as i64;
            time * 4 + staff + voice + midi
        })
}
fn consensus_probe_tone_label(tone: &HiddenChordToneRecoveryCandidate) -> String {
    format!(
        "m{} staff{} voice{} start{} dur{} midi{} klasse{}",
        tone.measure,
        tone.staff,
        tone.voice,
        tone.start_tick,
        tone.duration,
        tone.midi.map_or_else(|| "?".to_string(), |v| v.to_string()),
        reduction_role_consensus_class(tone)
    )
}
fn consensus_probe_accepted_details(tones: &[HiddenChordToneRecoveryCandidate], slots: &[VisibleChordToneSwapSlot]) -> Vec<String> {
    let mut details = Vec::new();
    for tone in tones.iter().take(8) {
        let anchor = consensus_probe_anchor_for_tone(tone, slots)
            .map_or_else(
                || "Anker nicht eindeutig".to_string(),
                |slot| format!(
                    "Anker staff{} voice{} start{} dur{} midi{} Δzeit{} Δmidi{}",
                    slot.staff,
                    slot.voice,
                    slot.start_tick,
                    slot.duration,
                    slot.midi.map(|v| v.to_string()).unwrap_or_else(|| "?".to_string()),
                    (slot.start_tick - tone.start_tick).abs(),
                    midi_distance(slot.midi, tone.midi)
                ),
            );
        details.push(format!("{} → {}", consensus_probe_tone_label(tone), anchor));
    }
    details
}
fn consensus_probe_remove_hidden_attrs(
    xml: &mut String,
    tones: &[HiddenChordToneRecoveryCandidate],
) -> (usize, master_state_identity::OwnerReceipts) {
    let mut ranges = tones.iter()
        .map(|tone| (tone.abs_start, tone.abs_end))
        .collect::<Vec<_>>();
    ranges.sort_by_key(|a| std::cmp::Reverse(a.0));
    let mut changed = 0usize;
    let mut receipts = master_state_identity::OwnerReceipts::default();
    for (start, end) in ranges {
        let (removed, receipt) = remove_note_hidden_attrs_in_range(xml, start, end);
        receipts.push_exact(receipt);
        if removed {
            changed += 1;
        }
    }
    (changed, receipts)
}
fn consensus_probe_reject_bucket(stats: &mut ConsensusGestureProbeStats, report: &CheckReport, baseline_hints: usize) -> String {
    let critical = critical_count(report);
    if report.summary.blockers > 0 {
        stats.rejected_blocker += 1;
        let ids = consensus_probe_record_rule_ids(stats, report);
        format!("B{} W{} H{} K{}; {}", report.summary.blockers, report.summary.warnings, report.summary.hints, critical, ids)
    } else if report.summary.warnings > 0 {
        stats.rejected_warning += 1;
        let ids = consensus_probe_record_rule_ids(stats, report);
        format!("B0 W{} H{} K{}; {}", report.summary.warnings, report.summary.hints, critical, ids)
    } else if critical > 0 {
        stats.rejected_critical += 1;
        let ids = consensus_probe_record_rule_ids(stats, report);
        format!("B0 W0 H{} K{}; {}", report.summary.hints, critical, ids)
    } else if report.summary.hints > baseline_hints {
        stats.rejected_hint += 1;
        let ids = consensus_probe_record_rule_ids(stats, report);
        format!("B0 W0 H{} K0; {}", report.summary.hints, ids)
    } else {
        stats.rejected_not_improved += 1;
        "formal sauber, aber ohne Sichtgewinn".to_string()
    }
}
fn gauss_nearest_record(records: &[GaussGestureRejectRecord]) -> Option<&GaussGestureRejectRecord> {
    records.iter().min_by_key(|record| {
        (
            if record.godel_certified { 0 } else { 1 },
            record.validator_blockers,
            record.validator_warnings,
            record.validator_critical,
            record.validator_hints,
            if record.visible_gain > 0 { 0 } else { 1 },
            record.residual,
            record.measure,
            record.half,
            record.take,
        )
    })
}
fn gauss_best_diagnostic_residual(records: &[GaussGestureRejectRecord]) -> Option<i64> {
    records.iter().map(|record| record.residual).min()
}


struct GaussRejectRecordInput<'a> {
    candidate_name: String,
    measure: usize,
    half: usize,
    take: usize,
    residual: i64,
    visible_gain: i64,
    report: Option<&'a CheckReport>,
    baseline_hints: usize,
    parse_error: Option<&'a str>,
}
fn push_gauss_reject_record(
    stats: &mut ConsensusGestureProbeStats,
    input: GaussRejectRecordInput<'_>,
) {
    let (validator_blockers, validator_warnings, validator_hints, validator_critical, godel_certified) =
        if let Some(report) = input.report {
            let critical = critical_count(report);
            let formal_ok_relative = report.summary.blockers == 0
                && report.summary.warnings == 0
                && critical == 0
                && report.summary.hints <= input.baseline_hints;
            (
                report.summary.blockers,
                report.summary.warnings,
                report.summary.hints,
                critical,
                formal_ok_relative && input.visible_gain > 0,
            )
        } else {
            (usize::from(input.parse_error.is_some()), 0, 0, 0, false)
        };
    stats.gauss_reject_records.push(GaussGestureRejectRecord {
        candidate_name: input.candidate_name,
        measure: input.measure,
        half: input.half,
        take: input.take,
        residual: input.residual,
        visible_gain: input.visible_gain,
        validator_blockers,
        validator_warnings,
        validator_hints,
        validator_critical,
        godel_certified,
    });
}
fn consensus_probe_report_only_han02(report: &CheckReport) -> bool {
    if report.summary.blockers > 0 || report.summary.warnings == 0 || critical_count(report) > 0 {
        return false;
    }
    let ids = consensus_probe_issue_rule_ids(report);
    !ids.is_empty() && ids.iter().all(|id| id == "HAN02")
}
fn replace_or_insert_note_tag(block: &mut String, tag: &str, value: &str) -> bool {
    crate::structural_write_authority::replace_or_insert_note_tag_authorized(block, tag, value)
}
fn consensus_probe_target_staff_for_han02(tone: &HiddenChordToneRecoveryCandidate, mode: usize) -> usize {
    let midi = tone.midi.unwrap_or(64);
    match mode {
        // Ergonomische Staff-Verteilung: hohe Töne nach oben, tiefe nach unten.
        0 => {
            if midi >= 72 { 1 } else if midi <= 57 { 2 } else { tone.staff }
        }
        // Gegenhand-Probe: falls die lokale Gleichzeitigkeit genau auf der aktuellen Hand zu breit wird.
        _ => if tone.staff == 1 { 2 } else { 1 },
    }
}
fn consensus_probe_han02_repair_label(mode: usize) -> &'static str {
    match mode {
        0 => "ergonomic_staff_distribution",
        _ => "opposite_hand_distribution",
    }
}
fn consensus_probe_apply_han02_staff_distribution(
    base_xml: &str,
    tones: &[HiddenChordToneRecoveryCandidate],
    mode: usize,
) -> (String, usize, Vec<String>, master_state_identity::OwnerReceipts) {
    let mut ordered = tones.to_vec();
    ordered.sort_by_key(|a| std::cmp::Reverse(a.abs_start));
    let mut edits = Vec::new();
    let mut details = Vec::new();
    for tone in &ordered {
        if tone.abs_start >= tone.abs_end || tone.abs_end > base_xml.len() {
            continue;
        }
        let source_block = &base_xml[tone.abs_start..tone.abs_end];
        let original_staff = tag_text(source_block, "staff")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(tone.staff);
        let target_staff = consensus_probe_target_staff_for_han02(tone, mode);
        let mut block = crate::structural_write_authority::remove_note_start_attribute_authorized(
            source_block,
            "print-object",
        );
        block = crate::structural_write_authority::remove_note_start_attribute_authorized(
            &block,
            "print-spacing",
        );
        let staff_changed = replace_or_insert_note_tag(&mut block, "staff", &target_staff.to_string());
        if block != source_block {
            edits.push((tone.abs_start, tone.abs_end, block));
            details.push(format!(
                "{}: {} staff{}→staff{}{}",
                consensus_probe_han02_repair_label(mode),
                consensus_probe_tone_label(tone),
                original_staff,
                target_staff,
                if staff_changed { "" } else { " (Staff blieb)" }
            ));
        }
    }
    let changed = edits.len();
    let (distributed, distribution_receipt) =
        crate::structural_write_authority::apply_visibility_staff_distribution_batch_authorized(
            base_xml,
            edits,
        );
    let Some(xml) = distributed else {
        return (
            base_xml.to_string(),
            0,
            details,
            master_state_identity::OwnerReceipts::default(),
        );
    };
    details.reverse();
    let mut receipts = master_state_identity::OwnerReceipts::default();
    receipts.push_exact(distribution_receipt);
    (xml, changed, details, receipts)
}
fn consensus_broken_gesture_label(mode: usize) -> &'static str {
    match mode {
        0 => "neighbor_forward_broken",
        1 => "neighbor_balanced_broken",
        _ => "neighbor_melodic_broken",
    }
}
fn consensus_probe_half_of_tick(start_tick: i64) -> usize {
    if start_tick < 192 { 1 } else { 2 }
}
fn consensus_broken_slot_score(
    tone: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
    used: &BTreeSet<(usize, usize, String, i64)>,
    mode: usize,
    ordinal: usize,
) -> i64 {
    let time_distance = (slot.start_tick - tone.start_tick).abs();
    let same_staff = if slot.staff == tone.staff { 0 } else { 180 };
    let same_voice = if slot.voice == tone.voice { 0 } else { 36 };
    let midi = midi_distance(slot.midi, tone.midi) as i64;
    let used_penalty = if used.contains(&(slot.measure, slot.staff, slot.voice.clone(), slot.start_tick)) { 500 } else { 0 };
    let same_time_penalty = if slot.start_tick == tone.start_tick { 240 } else { 0 };
    let direction_penalty = match mode {
        // Vorwärts bevorzugen, damit aus einer Gleichzeitigkeit eine kleine Nachbarfigur wird.
        0 => if slot.start_tick >= tone.start_tick { 0 } else { 96 },
        // Balance: Nachbaranschläge auf beiden Seiten sind erlaubt, Hauptkosten ist Nähe.
        1 => 0,
        // Melodisch: spätere Töne der Auswahl eher später im Halbteil platzieren.
        _ => {
            let wanted = if ordinal == 0 { 0 } else { (ordinal as i64) * 24 };
            ((slot.start_tick - tone.start_tick) - wanted).abs() / 2
        }
    };
    time_distance * 5 + same_staff + same_voice + midi * 3 + used_penalty + same_time_penalty + direction_penalty
}
fn consensus_broken_anchor_for_tone<'a>(
    tone: &HiddenChordToneRecoveryCandidate,
    slots: &'a [VisibleChordToneSwapSlot],
    used: &BTreeSet<(usize, usize, String, i64)>,
    mode: usize,
    ordinal: usize,
) -> Option<&'a VisibleChordToneSwapSlot> {
    let half = consensus_probe_half_of_tick(tone.start_tick);
    slots
        .iter()
        .filter(|slot| slot.measure == tone.measure)
        .filter(|slot| consensus_probe_half_of_tick(slot.start_tick) == half)
        .filter(|slot| slot.start_tick != tone.start_tick || mode == 1)
        .filter(|slot| (slot.start_tick - tone.start_tick).abs() <= 144)
        .filter(|slot| midi_distance(slot.midi, tone.midi) <= 24)
        .min_by_key(|slot| {
            (
                consensus_broken_slot_score(tone, slot, used, mode, ordinal),
                slot.note_start,
            )
        })
}
fn consensus_note_block_as_visible_chord(block: &str, anchor: &VisibleChordToneSwapSlot) -> String {
    let mut out = crate::structural_write_authority::remove_note_start_attribute_authorized(
        block,
        "print-object",
    );
    out = crate::structural_write_authority::remove_note_start_attribute_authorized(
        &out,
        "print-spacing",
    );
    out = crate::structural_write_authority::ensure_bound_chord_marker_authorized(&out).0
        .expect("source-bound visible chord marker authorization");
    let _ = crate::structural_write_authority::replace_or_insert_note_tag_authorized(
        &mut out,
        "duration",
        &anchor.duration.max(1).to_string(),
    );
    let _ = crate::structural_write_authority::replace_or_insert_note_tag_authorized(
        &mut out,
        "voice",
        &anchor.voice,
    );
    let _ = crate::structural_write_authority::replace_or_insert_note_tag_authorized(
        &mut out,
        "staff",
        &anchor.staff.to_string(),
    );
    out
}
fn consensus_exact_note_move_carriers_from_edits(
    moves: &[(usize, usize, usize)],
    edits: &[(usize, usize, String)],
) -> Vec<CandidateMaterialExactNoteMoveCarrier> {
    let mut insertion_ordinal_by_start = BTreeMap::<usize, usize>::new();
    moves
        .iter()
        .filter_map(|(before_note_start, before_note_end, insertion_start)| {
            let insertion_ordinal = insertion_ordinal_by_start
                .entry(*insertion_start)
                .or_insert(0);
            let current_ordinal = *insertion_ordinal;
            *insertion_ordinal = current_ordinal.saturating_add(1);

            let mut final_start = *insertion_start as i128;
            let mut same_start_insertion_ordinal = 0_usize;
            for (start, end, replacement) in edits {
                if *start < *insertion_start {
                    final_start +=
                        replacement.len() as i128 - end.saturating_sub(*start) as i128;
                } else if *start == *insertion_start && *end == *start {
                    if same_start_insertion_ordinal > current_ordinal {
                        final_start += replacement.len() as i128;
                    }
                    same_start_insertion_ordinal =
                        same_start_insertion_ordinal.saturating_add(1);
                }
            }
            usize::try_from(final_start)
                .ok()
                .map(|after_note_start| CandidateMaterialExactNoteMoveCarrier {
                    before_note_start: *before_note_start,
                    before_note_end: *before_note_end,
                    after_note_start,
                })
        })
        .collect()
}

fn consensus_probe_apply_broken_gesture_distribution(
    base_xml: &str,
    tones: &[HiddenChordToneRecoveryCandidate],
    slots: &[VisibleChordToneSwapSlot],
    mode: usize,
) -> (
    String,
    usize,
    Vec<String>,
    Vec<CandidateMaterialExactNoteMoveCarrier>,
    master_state_identity::OwnerReceipts,
) {
    let mut used: BTreeSet<(usize, usize, String, i64)> = BTreeSet::new();
    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    let mut moves: Vec<(usize, usize, usize)> = Vec::new();
    let mut details = Vec::new();
    let mut ordered = tones.to_vec();
    ordered.sort_by(|a, b| {
        a.start_tick.cmp(&b.start_tick)
            .then_with(|| a.midi.unwrap_or(0).cmp(&b.midi.unwrap_or(0)))
            .then_with(|| a.staff.cmp(&b.staff))
    });
    for (idx, tone) in ordered.iter().enumerate() {
        if tone.abs_start >= tone.abs_end || tone.abs_end > base_xml.len() {
            continue;
        }
        if !single_note_gesture_support_is_independent(&base_xml[tone.abs_start..tone.abs_end]) {
            continue;
        }
        let Some(anchor) = consensus_broken_anchor_for_tone(tone, slots, &used, mode, idx) else {
            continue;
        };
        let source_block = &base_xml[tone.abs_start..tone.abs_end];
        let clone = consensus_note_block_as_visible_chord(source_block, anchor);
        used.insert((anchor.measure, anchor.staff, anchor.voice.clone(), anchor.start_tick));
        edits.push((tone.abs_start, tone.abs_end, String::new()));
        edits.push((anchor.note_end, anchor.note_end, clone));
        moves.push((tone.abs_start, tone.abs_end, anchor.note_end));
        details.push(format!(
            "{}: {} → Nachbaranker staff{} voice{} start{} dur{} midi{} Δzeit{} Δmidi{}",
            consensus_broken_gesture_label(mode),
            consensus_probe_tone_label(tone),
            anchor.staff,
            anchor.voice,
            anchor.start_tick,
            anchor.duration,
            anchor.midi.map_or_else(|| "?".to_string(), |value| value.to_string()),
            (anchor.start_tick - tone.start_tick).abs(),
            midi_distance(anchor.midi, tone.midi)
        ));
    }
    if edits.is_empty() {
        return (
            base_xml.to_string(),
            0,
            details,
            Vec::new(),
            master_state_identity::OwnerReceipts::default(),
        );
    }
    let carriers = consensus_exact_note_move_carriers_from_edits(&moves, &edits);
    let (distributed, distribution_receipt) =
        crate::structural_write_authority::apply_chord_gesture_distribution_batch_authorized(
            base_xml,
            edits,
        );
    let Some((xml, changed)) = distributed else {
        return (
            base_xml.to_string(),
            0,
            details,
            Vec::new(),
            master_state_identity::OwnerReceipts::default(),
        );
    };
    let mut receipts = master_state_identity::OwnerReceipts::default();
    receipts.push_exact(distribution_receipt);
    (xml, changed, details, carriers, receipts)
}
fn consensus_gauss_slot_score(
    tone: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
    used: &BTreeSet<(usize, usize, String, i64)>,
    ordinal: usize,
) -> i64 {
    let dt = slot.start_tick - tone.start_tick;
    let dp = midi_distance(slot.midi, tone.midi) as i64;
    let staff = if slot.staff == tone.staff { 0 } else { 42 };
    let voice = if slot.voice == tone.voice { 0 } else { 12 };
    let used_penalty = if used.contains(&(slot.measure, slot.staff, slot.voice.clone(), slot.start_tick)) { 12000 } else { 0 };
    let target_shift = (ordinal as i64).saturating_mul(24);
    let curve = (dt - target_shift).abs();
    // GAU01: kleinste Quadrate über Zeit/Pitch/Hand. GAU02: Nähe wird nicht hart
    // erzwungen, sondern quadratisch abgewertet. GAU03: curve glättet die lokale
    // Hand-/Pitch-Kette. Keine Freigabe ohne nachfolgenden Validator.
    dt * dt * 3 + dp * dp * 24 + staff * staff + voice * voice + curve * curve * 2 + used_penalty
}
fn consensus_gauss_anchor_for_tone<'a>(
    tone: &HiddenChordToneRecoveryCandidate,
    slots: &'a [VisibleChordToneSwapSlot],
    used: &BTreeSet<(usize, usize, String, i64)>,
    ordinal: usize,
) -> Option<&'a VisibleChordToneSwapSlot> {
    let half = consensus_probe_half_of_tick(tone.start_tick);
    slots
        .iter()
        .filter(|slot| slot.measure == tone.measure)
        .filter(|slot| consensus_probe_half_of_tick(slot.start_tick) == half)
        .filter(|slot| (slot.start_tick - tone.start_tick).abs() <= 144)
        .filter(|slot| midi_distance(slot.midi, tone.midi) <= 24)
        .min_by_key(|slot| (
            consensus_gauss_slot_score(tone, slot, used, ordinal),
            slot.note_start,
        ))
}
fn consensus_gauss_cell_score(tones: &[HiddenChordToneRecoveryCandidate], slots: &[VisibleChordToneSwapSlot]) -> i64 {
    let mut used: BTreeSet<(usize, usize, String, i64)> = BTreeSet::new();
    let mut ordered = tones.to_vec();
    ordered.sort_by(|a, b| {
        a.start_tick.cmp(&b.start_tick)
            .then_with(|| a.midi.unwrap_or(0).cmp(&b.midi.unwrap_or(0)))
            .then_with(|| a.staff.cmp(&b.staff))
    });
    let mut residual = 0_i64;
    for (idx, tone) in ordered.iter().enumerate() {
        if let Some(anchor) = consensus_gauss_anchor_for_tone(tone, slots, &used, idx) {
            residual += consensus_gauss_slot_score(tone, anchor, &used, idx);
            used.insert((anchor.measure, anchor.staff, anchor.voice.clone(), anchor.start_tick));
        } else {
            residual += 9_000_000;
        }
    }
    residual
}
fn consensus_probe_apply_gauss_gesture_distribution(
    base_xml: &str,
    tones: &[HiddenChordToneRecoveryCandidate],
    slots: &[VisibleChordToneSwapSlot],
) -> (
    String,
    usize,
    Vec<String>,
    i64,
    Vec<CandidateMaterialExactNoteMoveCarrier>,
    master_state_identity::OwnerReceipts,
) {
    consensus_probe_apply_gauss_gesture_distribution_inner(base_xml, tones, slots, false)
}
fn consensus_probe_apply_gauss_gesture_distribution_tail_residual_reduced(
    base_xml: &str,
    tones: &[HiddenChordToneRecoveryCandidate],
    slots: &[VisibleChordToneSwapSlot],
) -> (
    String,
    usize,
    Vec<String>,
    i64,
    Vec<CandidateMaterialExactNoteMoveCarrier>,
    master_state_identity::OwnerReceipts,
) {
    consensus_probe_apply_gauss_gesture_distribution_inner(base_xml, tones, slots, true)
}
fn consensus_gauss_tail_ordinal(
    tone: &HiddenChordToneRecoveryCandidate,
    normal_ordinal: usize,
    split_by_cell: bool,
    counters: &mut BTreeMap<(usize, usize, usize, String), usize>,
) -> usize {
    if !split_by_cell {
        return normal_ordinal;
    }
    let key = (
        tone.measure,
        consensus_probe_half_of_tick(tone.start_tick),
        tone.staff,
        tone.voice.clone(),
    );
    let ordinal = counters.get(&key).copied().unwrap_or(0);
    counters.insert(key, ordinal.saturating_add(1));
    ordinal
}
fn consensus_gauss_distribution_residual(
    tones: &[HiddenChordToneRecoveryCandidate],
    slots: &[VisibleChordToneSwapSlot],
    split_by_cell: bool,
) -> i64 {
    let mut used: BTreeSet<(usize, usize, String, i64)> = BTreeSet::new();
    let mut ordinal_by_cell: BTreeMap<(usize, usize, usize, String), usize> = BTreeMap::new();
    let mut ordered = tones.to_vec();
    ordered.sort_by(|a, b| {
        a.start_tick.cmp(&b.start_tick)
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.midi.unwrap_or(0).cmp(&b.midi.unwrap_or(0)))
            .then_with(|| a.staff.cmp(&b.staff))
    });
    let mut residual = 0_i64;
    for (idx, tone) in ordered.iter().enumerate() {
        let local_ordinal = consensus_gauss_tail_ordinal(tone, idx, split_by_cell, &mut ordinal_by_cell);
        if let Some(anchor) = consensus_gauss_anchor_for_tone(tone, slots, &used, local_ordinal) {
            residual += consensus_gauss_slot_score(tone, anchor, &used, local_ordinal);
            used.insert((anchor.measure, anchor.staff, anchor.voice.clone(), anchor.start_tick));
        } else {
            residual += 9_000_000;
        }
    }
    residual
}
fn consensus_probe_apply_gauss_gesture_distribution_inner(
    base_xml: &str,
    tones: &[HiddenChordToneRecoveryCandidate],
    slots: &[VisibleChordToneSwapSlot],
    split_by_cell: bool,
) -> (
    String,
    usize,
    Vec<String>,
    i64,
    Vec<CandidateMaterialExactNoteMoveCarrier>,
    master_state_identity::OwnerReceipts,
) {
    let mut used: BTreeSet<(usize, usize, String, i64)> = BTreeSet::new();
    let mut ordinal_by_cell: BTreeMap<(usize, usize, usize, String), usize> = BTreeMap::new();
    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    let mut moves: Vec<(usize, usize, usize)> = Vec::new();
    let mut details = Vec::new();
    let mut residual = 0_i64;
    let mut ordered = tones.to_vec();
    ordered.sort_by(|a, b| {
        a.start_tick.cmp(&b.start_tick)
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.midi.unwrap_or(0).cmp(&b.midi.unwrap_or(0)))
            .then_with(|| a.staff.cmp(&b.staff))
    });
    for (idx, tone) in ordered.iter().enumerate() {
        if tone.abs_start >= tone.abs_end || tone.abs_end > base_xml.len() {
            continue;
        }
        if !single_note_gesture_support_is_independent(&base_xml[tone.abs_start..tone.abs_end]) {
            continue;
        }
        let local_ordinal = consensus_gauss_tail_ordinal(tone, idx, split_by_cell, &mut ordinal_by_cell);
        let Some(anchor) = consensus_gauss_anchor_for_tone(tone, slots, &used, local_ordinal) else {
            continue;
        };
        let local_residual = consensus_gauss_slot_score(tone, anchor, &used, local_ordinal);
        residual += local_residual;
        let source_block = &base_xml[tone.abs_start..tone.abs_end];
        let clone = consensus_note_block_as_visible_chord(source_block, anchor);
        used.insert((anchor.measure, anchor.staff, anchor.voice.clone(), anchor.start_tick));
        edits.push((tone.abs_start, tone.abs_end, String::new()));
        edits.push((anchor.note_end, anchor.note_end, clone));
        moves.push((tone.abs_start, tone.abs_end, anchor.note_end));
        details.push(format!(
            "{}: {} → Anker staff{} voice{} start{} dur{} midi{} Δzeit{} Δmidi{} residual{}",
            if split_by_cell { "gauss_tail_residual_reduced" } else { "gauss_least_squares" },
            consensus_probe_tone_label(tone),
            anchor.staff,
            anchor.voice,
            anchor.start_tick,
            anchor.duration,
            anchor.midi.map_or_else(|| "?".to_string(), |value| value.to_string()),
            (anchor.start_tick - tone.start_tick).abs(),
            midi_distance(anchor.midi, tone.midi),
            local_residual
        ));
    }
    if edits.is_empty() {
        return (
            base_xml.to_string(),
            0,
            details,
            residual,
            Vec::new(),
            master_state_identity::OwnerReceipts::default(),
        );
    }
    let carriers = consensus_exact_note_move_carriers_from_edits(&moves, &edits);
    let (distributed, distribution_receipt) =
        crate::structural_write_authority::apply_chord_gesture_distribution_batch_authorized(
            base_xml,
            edits,
        );
    let Some((xml, changed)) = distributed else {
        return (
            base_xml.to_string(),
            0,
            details,
            residual,
            Vec::new(),
            master_state_identity::OwnerReceipts::default(),
        );
    };
    let mut receipts = master_state_identity::OwnerReceipts::default();
    receipts.push_exact(distribution_receipt);
    (xml, changed, details, residual, carriers, receipts)
}
fn run_gauss_gesture_candidate_extension(
    seed: &SolverCandidate,
    options: &CheckOptions,
    next_iteration: &mut usize,
    stats: &mut ConsensusGestureProbeStats,
) -> anyhow::Result<ConsensusGestureCandidateOutcome> {
    let baseline_hints = seed.final_result.summary.hints;
    let seed_visible = visible_pitched_note_count(seed.layout_xml());
    let hidden = collect_hidden_chord_tone_recovery_candidates(seed.layout_xml(), 4096);
    let mut consensus_tones = hidden.into_iter()
        .filter(|cand| matches!(reduction_role_consensus_class(cand), "konsens-muss" | "konsens-ersatzfigur"))
        .collect::<Vec<_>>();
    stats.gauss_input_fit_points = stats.gauss_input_fit_points.max(consensus_tones.len());
    consensus_tones.sort_by(|a, b| {
        consensus_gesture_candidate_weight(b).cmp(&consensus_gesture_candidate_weight(a))
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.start_tick.cmp(&b.start_tick))
    });
    let slots = collect_visible_gesture_attack_slots(seed.layout_xml());
    let mut grouped: BTreeMap<(usize, usize), Vec<HiddenChordToneRecoveryCandidate>> = BTreeMap::new();
    for cand in consensus_tones {
        if consensus_probe_candidate_has_anchor(&cand, &slots) {
            grouped.entry(consensus_probe_cell_key(&cand)).or_default().push(cand);
        }
    }
    stats.gauss_candidate_raw = stats.gauss_candidate_raw.saturating_add(grouped.len());
    let mut cells = grouped.into_iter().map(|(key, mut tones)| {
        tones.sort_by(|a, b| {
            a.start_tick.cmp(&b.start_tick)
                .then_with(|| a.midi.unwrap_or(0).cmp(&b.midi.unwrap_or(0)))
                .then_with(|| a.staff.cmp(&b.staff))
        });
        let residual = consensus_gauss_cell_score(&tones, &slots);
        let substance: i64 = tones.iter().map(consensus_gesture_candidate_weight).sum();
        (key, tones, residual, substance)
    }).collect::<Vec<_>>();
    let topology_ready = cells.iter().filter(|(_, tones, _, _)| tones.len() >= 2).count();
    stats.gauss_candidate_after_topology = stats.gauss_candidate_after_topology.saturating_add(topology_ready);
    cells.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| b.3.cmp(&a.3)).then_with(|| a.0.cmp(&b.0)));
    let mut best_candidate: Option<SolverCandidate> = None;
    let mut best_selected: Vec<HiddenChordToneRecoveryCandidate> = Vec::new();
    let mut best_details: Vec<String> = Vec::new();
    let mut best_residual = i64::MAX;
    for ((measure, half), tones, _cell_residual, _substance) in cells.into_iter().take(8) {
        let max_take = tones.len().min(4);
        if max_take == 0 {
            continue;
        }
        // v0.7.2 Aktivierungsdiagnose: auch eine einzelne Gauß-Kopplung darf bis
        // zum Validator laufen, damit ein leerer Generator nicht unsichtbar bleibt.
        // Mehrgliedrige Zellen bleiben durch gauss_candidate_after_topology markiert.
        let take_start = if tones.len() >= 2 { 2 } else { 1 };
        for take in take_start..=max_take {
            stats.gauss_candidate_after_rhythm = stats.gauss_candidate_after_rhythm.saturating_add(1);
            stats.gauss_candidate_after_hand = stats.gauss_candidate_after_hand.saturating_add(1);
            stats.variants_checked += 1;
            stats.gauss_gesture_variants_checked += 1;
            let selected = tones.iter().take(take).cloned().collect::<Vec<_>>();
            let candidate_name = format!(
                "gauss_gesture_m{measure}_h{half}_n{take}_least_squares_smooth"
            );
            let (xml, changed, details, residual, move_carriers, distribution_receipts) =
                consensus_probe_apply_gauss_gesture_distribution(
                    seed.layout_xml(),
                    &selected,
                    &slots,
                );
            if changed == 0 {
                stats.rejected_not_improved += 1;
                push_gauss_reject_record(
                    stats,
                    GaussRejectRecordInput {
                        candidate_name,
                        measure,
                        half,
                        take,
                        residual,
                        visible_gain: 0,
                        report: None,
                        baseline_hints,
                        parse_error: None,
                    },
                );
                continue;
            }
            let checked_score = match parse_musicxml(&xml) {
                Ok(score) => score,
                Err(err) => {
                    stats.rejected_parse += 1;
                    let err_text = err.to_string();
                    push_gauss_reject_record(
                        stats,
                        GaussRejectRecordInput {
                            candidate_name,
                            measure,
                            half,
                            take,
                            residual,
                            visible_gain: 0,
                            report: None,
                            baseline_hints,
                            parse_error: Some(&err_text),
                        },
                    );
                    if stats.examples.len() < 10 {
                        stats.examples.push(format!("m{measure}h{half} n{take} gauss_least_squares parse-reject: {err}"));
                    }
                    continue;
                }
            };
            let mut checked = run_checks(&checked_score, options);
            let _clef_hint_repair = suppress_lower_staff_treble_clef_high_register_hints(&mut checked, &xml);
            let visible_after = visible_pitched_note_count(&xml);
            let visible_gain = visible_after as i64 - seed_visible as i64;
            let decision = decide_hidden_chord_tone_recovery_adoption(
                HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
                hidden_chord_tone_recovery_decision_facts(
                    &seed.final_result,
                    &checked,
                    seed_visible,
                    visible_after,
                ),
            );
            push_gauss_reject_record(
                stats,
                GaussRejectRecordInput {
                    candidate_name: candidate_name.clone(),
                    measure,
                    half,
                    take,
                    residual,
                    visible_gain,
                    report: Some(&checked),
                    baseline_hints,
                    parse_error: None,
                },
            );
            if decision.adopt {
                stats.gauss_candidate_after_godel = stats.gauss_candidate_after_godel.saturating_add(1);
                let mut candidate = seed.clone();
                candidate.iteration = *next_iteration;
                *next_iteration += 1;
                candidate.profile_name = candidate_name;
                candidate.profile_lineage = candidate.profile_lineage.with_gauss_gesture();
                continue_candidate_owned_rhythm(seed, &mut candidate, &distribution_receipts, &xml, "ConsensusGesture/GaussCandidate");
                candidate.master_state.adopt_materialized_document(
                xml,
                distribution_receipts,
                "ConsensusGesture/GaussCandidate",
            );
                reconcile_consensus_gesture_material_lineage_with_longitudinal_audit(
                    seed.layout_xml(),
                    &mut candidate,
                    "ConsensusGestureProbe/GaussCandidate",
                    &move_carriers,
                )?;
                candidate.final_result = checked;
                let replace = best_candidate.as_ref().is_none_or(|current| {
                        let current_visible = visible_pitched_note_count(current.layout_xml());
                        visible_after > current_visible
                            || (visible_after == current_visible && residual < best_residual)
                            || (visible_after == current_visible && residual == best_residual && candidate_selection_key(&candidate) < candidate_selection_key(current))
                    },
                );
                if replace {
                    best_residual = residual;
                    best_selected = selected.clone();
                    best_details = details.clone();
                    best_candidate = Some(candidate);
                }
            } else {
                let reason = consensus_probe_reject_bucket(stats, &checked, baseline_hints);
                if stats.examples.len() < 10 {
                    stats.examples.push(format!(
                        "m{measure}h{half} n{take} gauss_least_squares abgelehnt: {reason}; residual {residual}; sichtbar {seed_visible}→{visible_after}"
                    ));
                }
            }
        }
    }
    if let Some(best_diagnostic_residual) = gauss_best_diagnostic_residual(&stats.gauss_reject_records) {
        // v0.7.7: Im Bericht bedeutet "bestes Residuum" eindeutig
        // das kleinste diagnostisch gemessene Residuum aller geprüften Gauß-Einzelkandidaten.
        // Die frühere Annahme-/Mehrton-Sortiergröße bleibt nur interne Auswahlheuristik.
        stats.gauss_gesture_best_residual = best_diagnostic_residual;
    } else if best_residual != i64::MAX {
        stats.gauss_gesture_best_residual = best_residual;
    }
    Ok(best_candidate.map(|candidate| (candidate, best_selected, best_details)))
}

fn run_gauss_gesture_chain_extension(
    seed: &SolverCandidate,
    options: &CheckOptions,
    next_iteration: &mut usize,
    stats: &mut ConsensusGestureProbeStats,
    economy_constrained_tail: bool,
) -> anyhow::Result<ConsensusGestureCandidateOutcome> {
    let baseline_hints = seed.final_result.summary.hints;
    let seed_visible = visible_pitched_note_count(seed.layout_xml());
    let hidden = collect_hidden_chord_tone_recovery_candidates(seed.layout_xml(), 4096);
    let mut consensus_tones = hidden.into_iter()
        .filter(|cand| matches!(reduction_role_consensus_class(cand), "konsens-muss" | "konsens-ersatzfigur"))
        .collect::<Vec<_>>();
    consensus_tones.sort_by(|a, b| {
        consensus_gesture_candidate_weight(b).cmp(&consensus_gesture_candidate_weight(a))
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.start_tick.cmp(&b.start_tick))
    });
    let slots = collect_visible_gesture_attack_slots(seed.layout_xml());
    let mut grouped: BTreeMap<(usize, usize), Vec<HiddenChordToneRecoveryCandidate>> = BTreeMap::new();
    for cand in consensus_tones {
        if consensus_probe_candidate_has_anchor(&cand, &slots) {
            grouped.entry(consensus_probe_cell_key(&cand)).or_default().push(cand);
        }
    }
    let mut cells = grouped.into_iter().map(|(key, mut tones)| {
        tones.sort_by(|a, b| {
            a.measure.cmp(&b.measure)
                .then_with(|| a.start_tick.cmp(&b.start_tick))
                .then_with(|| a.midi.unwrap_or(0).cmp(&b.midi.unwrap_or(0)))
                .then_with(|| a.staff.cmp(&b.staff))
        });
        let residual = consensus_gauss_cell_score(&tones, &slots);
        let substance: i64 = tones.iter().map(consensus_gesture_candidate_weight).sum();
        (key, tones, residual, substance)
    }).collect::<Vec<_>>();
    cells.sort_by_key(|a| a.0);
    let mut chain_units: Vec<HiddenChordToneRecoveryChainUnit> = cells.into_iter()
        .filter_map(|(key, tones, residual, substance)| {
            if tones.is_empty() {
                None
            } else {
                let take = if tones.len() >= 2 { 2 } else { 1 };
                Some((key, tones.into_iter().take(take).collect::<Vec<_>>(), residual, substance))
            }
        })
        .collect();
    chain_units.sort_by_key(|a| a.0);
    let mut best_candidate: Option<SolverCandidate> = None;
    let mut best_selected: Vec<HiddenChordToneRecoveryCandidate> = Vec::new();
    let mut best_details: Vec<String> = Vec::new();
    let mut best_gain: i64 = -1;
    let mut best_residual: i64 = i64::MAX;
    let mut best_economy_pressure: i64 = i64::MAX;
    let mut windows: Vec<(String, Vec<HiddenChordToneRecoveryCandidate>)> = Vec::new();
    for start in 0..chain_units.len() {
        let mut selected: Vec<HiddenChordToneRecoveryCandidate> = Vec::new();
        let first_measure = chain_units[start].0.0;
        for end in start..chain_units.len().min(start + 5) {
            let last_measure = chain_units[end].0.0;
            if last_measure.saturating_sub(first_measure) > 4 {
                break;
            }
            selected.extend(chain_units[end].1.iter().cloned());
            if end > start && selected.len() >= 2 {
                let label = format!(
                    "m{}_h{}_to_m{}_h{}_n{}",
                    chain_units[start].0.0,
                    chain_units[start].0.1,
                    chain_units[end].0.0,
                    chain_units[end].0.1,
                    selected.len()
                );
                windows.push((label, selected.clone()));
            }
        }
    }
    // Zusätzlich eine streng begrenzte globale Front: nur die acht besten Residual-Zellen,
    // aber weiterhin über den harten Validator. Das ist der eigentliche v0.7.6-Test, ob
    // saubere Einzelpunkte zu einer stärkeren Gauß-Geste werden können.
    let mut front_units = chain_units.clone();
    front_units.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| b.3.cmp(&a.3)).then_with(|| a.0.cmp(&b.0)));
    for take_cells in 2..=front_units.len().min(8) {
        let mut selected = Vec::new();
        for (_, tones, _, _) in front_units.iter().take(take_cells) {
            selected.extend(tones.iter().cloned());
        }
        if selected.len() >= 2 {
            windows.push((format!("front{}_n{}", take_cells, selected.len()), selected));
        }
    }
    let mut seen_windows: BTreeSet<String> = BTreeSet::new();
    for (label, selected_raw) in windows.into_iter().take(64) {
        let mut selected = selected_raw;
        selected.sort_by(|a, b| {
            a.abs_start.cmp(&b.abs_start)
                .then_with(|| a.measure.cmp(&b.measure))
                .then_with(|| a.start_tick.cmp(&b.start_tick))
        });
        selected.dedup_by(|a, b| a.abs_start == b.abs_start && a.abs_end == b.abs_end);
        if selected.len() < 2 {
            continue;
        }
        let key = selected.iter().map(|tone| format!("{}:{}:{}", tone.measure, tone.start_tick, tone.midi.unwrap_or(0))).collect::<Vec<_>>().join("|");
        if !seen_windows.insert(key) {
            continue;
        }
        stats.variants_checked += 1;
        stats.gauss_chain_variants_checked += 1;
        let first_measure = selected.first().map_or(0, |tone| tone.measure);
        let first_half = selected.first().map_or(1, |tone| consensus_probe_half_of_tick(tone.start_tick));
        let candidate_name = format!("gauss_chain_{label}_least_squares_chain");
        let residual_before_reduction = if economy_constrained_tail {
            consensus_gauss_distribution_residual(&selected, &slots, false)
        } else {
            0
        };
        let (xml, changed, details, residual, move_carriers, distribution_receipts) = if economy_constrained_tail {
            consensus_probe_apply_gauss_gesture_distribution_tail_residual_reduced(
                seed.layout_xml(),
                &selected,
                &slots,
            )
        } else {
            consensus_probe_apply_gauss_gesture_distribution(
                seed.layout_xml(),
                &selected,
                &slots,
            )
        };
        if changed == 0 {
            stats.rejected_not_improved += 1;
            push_gauss_reject_record(
                stats,
                GaussRejectRecordInput {
                    candidate_name,
                    measure: first_measure,
                    half: first_half,
                    take: selected.len(),
                    residual,
                    visible_gain: 0,
                    report: None,
                    baseline_hints,
                    parse_error: None,
                },
            );
            if let Some(record) = stats.gauss_reject_records.pop() {
                stats.gauss_chain_reject_records.push(record);
            }
            continue;
        }
        let checked_score = match parse_musicxml(&xml) {
            Ok(score) => score,
            Err(err) => {
                stats.rejected_parse += 1;
                let err_text = err.to_string();
                push_gauss_reject_record(
                    stats,
                    GaussRejectRecordInput {
                        candidate_name,
                        measure: first_measure,
                        half: first_half,
                        take: selected.len(),
                        residual,
                        visible_gain: 0,
                        report: None,
                        baseline_hints,
                        parse_error: Some(&err_text),
                    },
                );
                if let Some(record) = stats.gauss_reject_records.pop() {
                    stats.gauss_chain_reject_records.push(record);
                }
                if stats.examples.len() < 12 {
                    stats.examples.push(format!("{label} gauss_chain parse-reject: {err}"));
                }
                continue;
            }
        };
        let mut checked = run_checks(&checked_score, options);
        let _clef_hint_repair = suppress_lower_staff_treble_clef_high_register_hints(&mut checked, &xml);
        let visible_after = visible_pitched_note_count(&xml);
        let visible_gain = visible_after as i64 - seed_visible as i64;
        let decision = decide_hidden_chord_tone_recovery_adoption(
            HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
            hidden_chord_tone_recovery_decision_facts(
                &seed.final_result,
                &checked,
                seed_visible,
                visible_after,
            ),
        );
        push_gauss_reject_record(
            stats,
            GaussRejectRecordInput {
                candidate_name: candidate_name.clone(),
                measure: first_measure,
                half: first_half,
                take: selected.len(),
                residual,
                visible_gain,
                report: Some(&checked),
                baseline_hints,
                parse_error: None,
            },
        );
        if let Some(record) = stats.gauss_reject_records.pop() {
            stats.gauss_chain_reject_records.push(record);
        }
        if decision.adopt {
            let mut candidate = seed.clone();
            candidate.iteration = *next_iteration;
            *next_iteration += 1;
            candidate.profile_name = candidate_name.clone();
            candidate.profile_lineage = candidate.profile_lineage.with_gauss_chain();
            continue_candidate_owned_rhythm(seed, &mut candidate, &distribution_receipts, &xml, "ConsensusGesture/GaussChain");
            candidate.master_state.adopt_materialized_document(
                xml,
                distribution_receipts,
                "ConsensusGesture/GaussChain",
            );
            reconcile_consensus_gesture_material_lineage_with_longitudinal_audit(
                seed.layout_xml(),
                &mut candidate,
                if economy_constrained_tail {
                    "ConsensusGestureProbe/ConstrainedTail"
                } else {
                    "ConsensusGestureProbe/GaussChain"
                },
                &move_carriers,
            )?;
            candidate.final_result = checked;
            let economy_pressure = residual
                .saturating_add((selected.len() as i64).saturating_mul(600))
                .saturating_add((stats.gauss_chain_variants_checked as i64).saturating_mul(7))
                .saturating_sub(visible_gain.max(0).saturating_mul(180));
            let replace = best_candidate.as_ref().is_none_or(|current| {
                    let current_visible = visible_pitched_note_count(current.layout_xml());
                    if economy_constrained_tail {
                        visible_after > current_visible
                            || (visible_after == current_visible && economy_pressure < best_economy_pressure)
                            || (visible_after == current_visible && economy_pressure == best_economy_pressure && residual < best_residual)
                            || (visible_after == current_visible && economy_pressure == best_economy_pressure && residual == best_residual && candidate_selection_key(&candidate) < candidate_selection_key(current))
                    } else {
                        visible_after > current_visible
                            || (visible_after == current_visible && residual < best_residual)
                            || (visible_after == current_visible && residual == best_residual && candidate_selection_key(&candidate) < candidate_selection_key(current))
                    }
                },
            );
            if replace {
                best_gain = visible_gain;
                best_residual = residual;
                best_economy_pressure = economy_pressure;
                best_selected = selected.clone();
                best_details = details.clone();
                stats.gauss_chain_best_name = Some(candidate_name);
                stats.gauss_chain_best_gain = visible_gain;
                stats.gauss_chain_best_residual = residual;
                if economy_constrained_tail {
                    stats.economy_tail_residual_reduction = true;
                    stats.economy_tail_residual_before = residual_before_reduction;
                    stats.economy_tail_residual_after = residual;
                    stats.economy_tail_residual_savings = residual_before_reduction.saturating_sub(residual).max(0);
                    stats.economy_tail_residual_strategy = "split_ordinal_by_scope_half_staff_voice".to_string();
                }
                best_candidate = Some(candidate);
            }
        } else {
            let reason = consensus_probe_reject_bucket(stats, &checked, baseline_hints);
            if stats.examples.len() < 12 {
                stats.examples.push(format!(
                    "{label} gauss_chain abgelehnt: {reason}; residual {residual}; sichtbar {seed_visible}→{visible_after}"
                ));
            }
        }
    }
    if best_candidate.is_some() {
        stats.gauss_chain_best_gain = stats.gauss_chain_best_gain.max(best_gain);
        if stats.gauss_chain_best_residual == 0 || best_residual < stats.gauss_chain_best_residual {
            stats.gauss_chain_best_residual = best_residual;
        }
    } else if let Some(best) = gauss_nearest_record(&stats.gauss_chain_reject_records) {
        stats.gauss_chain_best_name = Some(best.candidate_name.clone());
        stats.gauss_chain_best_gain = best.visible_gain;
        stats.gauss_chain_best_residual = best.residual;
    }
    Ok(best_candidate.map(|candidate| (candidate, best_selected, best_details)))
}
fn run_consensus_broken_gesture_extension(
    seed: &SolverCandidate,
    options: &CheckOptions,
    next_iteration: &mut usize,
    stats: &mut ConsensusGestureProbeStats,
) -> anyhow::Result<ConsensusGestureCandidateOutcome> {
    let baseline_hints = seed.final_result.summary.hints;
    let seed_visible = visible_pitched_note_count(seed.layout_xml());
    let hidden = collect_hidden_chord_tone_recovery_candidates(seed.layout_xml(), 4096);
    let mut consensus_tones = hidden.into_iter()
        .filter(|cand| matches!(reduction_role_consensus_class(cand), "konsens-muss" | "konsens-ersatzfigur"))
        .collect::<Vec<_>>();
    consensus_tones.sort_by(|a, b| {
        consensus_gesture_candidate_weight(b).cmp(&consensus_gesture_candidate_weight(a))
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.start_tick.cmp(&b.start_tick))
    });
    let slots = collect_visible_gesture_attack_slots(seed.layout_xml());
    let mut grouped: BTreeMap<(usize, usize), Vec<HiddenChordToneRecoveryCandidate>> = BTreeMap::new();
    for cand in consensus_tones {
        if consensus_probe_candidate_has_anchor(&cand, &slots) {
            grouped.entry(consensus_probe_cell_key(&cand)).or_default().push(cand);
        }
    }
    let mut cells = grouped.into_iter().map(|(key, mut tones)| {
        tones.sort_by(|a, b| {
            consensus_gesture_candidate_weight(b).cmp(&consensus_gesture_candidate_weight(a))
                .then_with(|| a.start_tick.cmp(&b.start_tick))
                .then_with(|| a.staff.cmp(&b.staff))
        });
        let score: i64 = tones.iter().map(consensus_gesture_candidate_weight).sum();
        (key, tones, score)
    }).collect::<Vec<_>>();
    cells.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
    let mut best_candidate: Option<SolverCandidate> = None;
    let mut best_selected: Vec<HiddenChordToneRecoveryCandidate> = Vec::new();
    let mut best_details: Vec<String> = Vec::new();
    for ((measure, half), tones, _score) in cells.into_iter().take(25) {
        let max_take = tones.len().min(4);
        for take in 1..=max_take {
            let selected = tones.iter().take(take).cloned().collect::<Vec<_>>();
            for mode in 0..=2 {
                stats.variants_checked += 1;
                stats.broken_gesture_variants_checked += 1;
                let (xml, changed, details, move_carriers, distribution_receipts) =
                    consensus_probe_apply_broken_gesture_distribution(
                        seed.layout_xml(),
                        &selected,
                        &slots,
                        mode,
                    );
                if changed == 0 {
                    stats.rejected_not_improved += 1;
                    continue;
                }
                let checked_score = match parse_musicxml(&xml) {
                    Ok(score) => score,
                    Err(err) => {
                        stats.rejected_parse += 1;
                        if stats.examples.len() < 10 {
                            stats.examples.push(format!("m{}h{} n{} {} parse-reject: {}", measure, half, take, consensus_broken_gesture_label(mode), err));
                        }
                        continue;
                    }
                };
                let mut checked = run_checks(&checked_score, options);
                let _clef_hint_repair = suppress_lower_staff_treble_clef_high_register_hints(&mut checked, &xml);
                let visible_after = visible_pitched_note_count(&xml);
                let decision = decide_hidden_chord_tone_recovery_adoption(
                    HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
                    hidden_chord_tone_recovery_decision_facts(
                        &seed.final_result,
                        &checked,
                        seed_visible,
                        visible_after,
                    ),
                );
                if decision.adopt {
                    let mut candidate = seed.clone();
                    candidate.iteration = *next_iteration;
                    *next_iteration += 1;
                    let from_gauss_chain = seed
                        .profile_lineage
                        .contains(CandidateLineageStep::GaussChain);
                    candidate.profile_name = broken_gesture_name(
                        from_gauss_chain,
                        measure,
                        half,
                        take,
                        consensus_broken_gesture_label(mode),
                    );
                    candidate.profile_lineage = seed.profile_lineage.with_broken_gesture();
                    continue_candidate_owned_rhythm(seed, &mut candidate, &distribution_receipts, &xml, "ConsensusGesture/BrokenGesture");
                    candidate.master_state.adopt_materialized_document(
                xml,
                distribution_receipts,
                "ConsensusGesture/BrokenGesture",
            );
                    reconcile_consensus_gesture_material_lineage_with_longitudinal_audit(
                        seed.layout_xml(),
                        &mut candidate,
                        "ConsensusGestureProbe/BrokenGesture",
                        &move_carriers,
                    )?;
                    candidate.final_result = checked;
                    let replace = best_candidate.as_ref().is_none_or(|current| {
                            let current_visible = visible_pitched_note_count(current.layout_xml());
                            visible_after > current_visible || (visible_after == current_visible && candidate_selection_key(&candidate) < candidate_selection_key(current))
                        },
                    );
                    if replace {
                        best_selected = selected.clone();
                        best_details = details.clone();
                        best_candidate = Some(candidate);
                    }
                } else {
                    let reason = consensus_probe_reject_bucket(stats, &checked, baseline_hints);
                    if stats.examples.len() < 10 {
                        stats.examples.push(format!(
                            "m{}h{} n{} {} abgelehnt: {}; sichtbar {}→{}",
                            measure,
                            half,
                            take,
                            consensus_broken_gesture_label(mode),
                            reason,
                            seed_visible,
                            visible_after
                        ));
                    }
                }
            }
        }
    }
    Ok(best_candidate.map(|candidate| (candidate, best_selected, best_details)))
}
fn run_consensus_gesture_probe(
    best: &mut SolverCandidate,
    options: &CheckOptions,
    next_iteration: &mut usize,
) -> anyhow::Result<ConsensusGestureProbeStats> {
    let mut stats = ConsensusGestureProbeStats::default();
    stats.start_visible_notes = visible_pitched_note_count(best.layout_xml());
    stats.end_visible_notes = stats.start_visible_notes;
    let baseline_hints = best.final_result.summary.hints;
    let hidden = collect_hidden_chord_tone_recovery_candidates(best.layout_xml(), 4096);
    let mut consensus_tones = hidden.into_iter()
        .filter(|cand| matches!(reduction_role_consensus_class(cand), "konsens-muss" | "konsens-ersatzfigur"))
        .collect::<Vec<_>>();
    consensus_tones.sort_by(|a, b| {
        consensus_gesture_candidate_weight(b).cmp(&consensus_gesture_candidate_weight(a))
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.start_tick.cmp(&b.start_tick))
    });
    let slots = collect_visible_gesture_attack_slots(best.layout_xml());
    let mut grouped: BTreeMap<(usize, usize), Vec<HiddenChordToneRecoveryCandidate>> = BTreeMap::new();
    for cand in consensus_tones {
        if consensus_probe_candidate_has_anchor(&cand, &slots) {
            grouped.entry(consensus_probe_cell_key(&cand)).or_default().push(cand);
        }
    }
    let mut cells = grouped.into_iter().map(|(key, mut tones)| {
        tones.sort_by(|a, b| {
            consensus_gesture_candidate_weight(b).cmp(&consensus_gesture_candidate_weight(a))
                .then_with(|| a.start_tick.cmp(&b.start_tick))
                .then_with(|| a.staff.cmp(&b.staff))
        });
        let score: i64 = tones.iter().map(consensus_gesture_candidate_weight).sum();
        (key, tones, score)
    }).collect::<Vec<_>>();
    cells.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
    let mut accepted_candidate: Option<SolverCandidate> = None;
    let mut accepted_selected: Vec<HiddenChordToneRecoveryCandidate> = Vec::new();
    let mut accepted_han02_repair_details: Vec<String> = Vec::new();
    let mut previous_best: Option<SolverCandidate> = None;
    for ((measure, half), tones, _score) in cells.into_iter().take(25) {
        stats.checked_cells += 1;
        let max_take = tones.len().min(4);
        for take in 1..=max_take {
            stats.variants_checked += 1;
            let selected = tones.iter().take(take).cloned().collect::<Vec<_>>();
            let mut xml = best.layout_xml().to_string();
            let (changed, unhide_receipts) = consensus_probe_remove_hidden_attrs(&mut xml, &selected);
            if changed == 0 {
                stats.rejected_not_improved += 1;
                continue;
            }
            let checked_score = match parse_musicxml(&xml) {
                Ok(score) => score,
                Err(err) => {
                    stats.rejected_parse += 1;
                    if stats.examples.len() < 6 {
                        stats.examples.push(format!("m{measure}h{half} n{take} parse-reject: {err}"));
                    }
                    continue;
                }
            };
            let mut checked = run_checks(&checked_score, options);
            let _clef_hint_repair = suppress_lower_staff_treble_clef_high_register_hints(&mut checked, &xml);
            let visible_after = visible_pitched_note_count(&xml);
            let decision = decide_hidden_chord_tone_recovery_adoption(
                HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
                hidden_chord_tone_recovery_decision_facts(
                    &best.final_result,
                    &checked,
                    stats.start_visible_notes,
                    visible_after,
                ),
            );
            if decision.adopt {
                let mut candidate = best.clone();
                candidate.iteration = *next_iteration;
                *next_iteration += 1;
                candidate.profile_name = format!("consensus_gesture_probe_m{measure}_h{half}_n{take}");
                continue_candidate_owned_rhythm(best, &mut candidate, &unhide_receipts, &xml, "ConsensusGestureProbe/DirectUnhide");
                candidate.master_state.adopt_materialized_document(
                    xml,
                    unhide_receipts,
                    "ConsensusGestureProbe/DirectUnhide",
                );
                reconcile_candidate_material_lineage(
                    &mut candidate,
                    "ConsensusGestureProbe/DirectUnhide",
                    CandidateMaterialDeclaredEffects::representation_only(),
                )?;
                candidate.final_result = checked;
                let replace = accepted_candidate.as_ref().is_none_or(|current| {
                        let current_visible = visible_pitched_note_count(current.layout_xml());
                        visible_after > current_visible || (visible_after == current_visible && candidate_selection_key(&candidate) < candidate_selection_key(current))
                    },
                );
                if replace {
                    accepted_selected = selected.clone();
                    accepted_han02_repair_details.clear();
                    accepted_candidate = Some(candidate);
                }
            } else {
                let only_han02 = consensus_probe_report_only_han02(&checked);
                let reason = consensus_probe_reject_bucket(&mut stats, &checked, baseline_hints);
                if stats.examples.len() < 8 {
                    stats.examples.push(format!("m{}h{} n{} abgelehnt: {}; sichtbar {}→{}", measure, half, take, reason, stats.start_visible_notes, visible_after));
                }
                // v0.5.8 ConsensusGestureProbeHan02Repair:
                // Nur wenn die enge Probe ausschließlich an HAN02 scheitert, wird nicht gelockert,
                // sondern eine alternative Staff-/Hand-Verteilung derselben vorhandenen Töne geprüft.
                if only_han02 {
                    for mode in 0..=1 {
                        stats.variants_checked += 1;
                        stats.han02_repair_variants_checked += 1;
                        let (repair_xml, repair_changed, repair_details, repair_receipts) = consensus_probe_apply_han02_staff_distribution(best.layout_xml(), &selected, mode);
                        if repair_changed == 0 {
                            stats.rejected_not_improved += 1;
                            continue;
                        }
                        let repair_score = match parse_musicxml(&repair_xml) {
                            Ok(score) => score,
                            Err(err) => {
                                stats.rejected_parse += 1;
                                if stats.examples.len() < 8 {
                                    stats.examples.push(format!("m{}h{} n{} {} parse-reject: {}", measure, half, take, consensus_probe_han02_repair_label(mode), err));
                                }
                                continue;
                            }
                        };
                        let mut repair_checked = run_checks(&repair_score, options);
                        let _repair_clef_hint = suppress_lower_staff_treble_clef_high_register_hints(&mut repair_checked, &repair_xml);
                        let repair_visible_after = visible_pitched_note_count(&repair_xml);
                        let decision = decide_hidden_chord_tone_recovery_adoption(
                            HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
                            hidden_chord_tone_recovery_decision_facts(
                                &best.final_result,
                                &repair_checked,
                                stats.start_visible_notes,
                                repair_visible_after,
                            ),
                        );
                        if decision.adopt {
                            let mut candidate = best.clone();
                            candidate.iteration = *next_iteration;
                            *next_iteration += 1;
                            candidate.profile_name = format!(
                                "consensus_gesture_probe_han02_m{}_h{}_n{}_{}",
                                measure,
                                half,
                                take,
                                consensus_probe_han02_repair_label(mode)
                            );
                            continue_candidate_owned_rhythm(best, &mut candidate, &repair_receipts, &repair_xml, "ConsensusGestureProbe/HanStaffDistribution");
                            candidate.master_state.adopt_materialized_document(
                repair_xml,
                repair_receipts,
                "ConsensusGestureProbe/HanStaffDistribution",
            );
                            reconcile_candidate_material_lineage(
                                &mut candidate,
                                "ConsensusGestureProbe/HanStaffDistribution",
                                CandidateMaterialDeclaredEffects::pitch_staff_voice(),
                            )?;
                            candidate.final_result = repair_checked;
                            let replace = accepted_candidate.as_ref().is_none_or(|current| {
                                    let current_visible = visible_pitched_note_count(current.layout_xml());
                                    repair_visible_after > current_visible || (repair_visible_after == current_visible && candidate_selection_key(&candidate) < candidate_selection_key(current))
                                },
                            );
                            if replace {
                                accepted_selected = selected.clone();
                                accepted_han02_repair_details = repair_details.clone();
                                accepted_candidate = Some(candidate);
                            }
                        } else {
                            let repair_reason = consensus_probe_reject_bucket(&mut stats, &repair_checked, baseline_hints);
                            if stats.examples.len() < 8 {
                                stats.examples.push(format!(
                                    "m{}h{} n{} {} abgelehnt: {}; sichtbar {}→{}",
                                    measure,
                                    half,
                                    take,
                                    consensus_probe_han02_repair_label(mode),
                                    repair_reason,
                                    stats.start_visible_notes,
                                    repair_visible_after
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    // v0.7.0 GaussGestureCandidateGenerator:
    // Gauß ist nun nicht nur Audit, sondern erzeugt aus den besten lokalen
    // Fitpunkten vorsichtige, geglättete Kandidaten. Freigabe bleibt unverändert
    // hart über RuleIds, GDL01 und den finalen 0/0/0/0-Validator.
    // v0.7.2: Gauß-Aktivierungsdiagnose läuft bewusst auf der unveränderten sicheren
    // Ausgangsfassung. So wird sichtbar, ob der Generator wirklich Kandidaten bis
    // zum Validator bringt, statt nach einer vorherigen Consensus-Übernahme leerzulaufen.
    let gauss_seed: SolverCandidate = (*best).clone();
    if let Some((gauss_candidate, gauss_selected, gauss_details)) =
        run_gauss_gesture_candidate_extension(&gauss_seed, options, next_iteration, &mut stats)?
    {
        let gauss_visible = visible_pitched_note_count(gauss_candidate.layout_xml());
        let replace = accepted_candidate.as_ref().is_none_or(|current| {
                let current_visible = visible_pitched_note_count(current.layout_xml());
                gauss_visible > current_visible || (gauss_visible == current_visible && candidate_selection_key(&gauss_candidate) < candidate_selection_key(current))
            },
        );
        if replace {
            accepted_selected = gauss_selected;
            stats.gauss_gesture_accepted = true;
            stats.gauss_gesture_details = gauss_details.clone();
            accepted_candidate = Some(gauss_candidate);
        }
    }
    if let Some((chain_candidate, chain_selected, chain_details)) =
        run_gauss_gesture_chain_extension(
            &gauss_seed,
            options,
            next_iteration,
            &mut stats,
            false,
        )?
    {
        let chain_visible = visible_pitched_note_count(chain_candidate.layout_xml());
        let replace = accepted_candidate.as_ref().is_none_or(|current| {
                let current_visible = visible_pitched_note_count(current.layout_xml());
                chain_visible > current_visible || (chain_visible == current_visible && candidate_selection_key(&chain_candidate) < candidate_selection_key(current))
            },
        );
        if replace {
            accepted_selected = chain_selected;
            stats.gauss_chain_accepted = true;
            stats.gauss_chain_details = chain_details.clone();
            accepted_candidate = Some(chain_candidate);
        }
    }
    if let Some(seed_candidate) = accepted_candidate.take() {
        if let Some((broken_candidate, broken_selected, broken_details)) =
            run_consensus_broken_gesture_extension(
                &seed_candidate,
                options,
                next_iteration,
                &mut stats,
            )?
        {
            accepted_selected = broken_selected;
            stats.broken_gesture_accepted = true;
            stats.broken_gesture_details = broken_details.clone();
            accepted_candidate = Some(broken_candidate);
        } else {
            accepted_candidate = Some(seed_candidate);
        }
    }
    // v0.7.8 PostHybridGaussTailProbe / v0.7.9 MathematicalEconomyAudit:
    // Nach der erfolgreichen Gauß+Broken-Hybridfassung wird exakt ein enger zweiter
    // Gauß-Kettenversuch auf der bereits verbesserten Fassung erlaubt. Er darf nur
    // zusätzlichen Sichtgewinn relativ zur Hybridfassung bringen und bleibt an den
    // gleichen GDL01-/0/0/0/0-Gates. Kein breiter Suchraum, keine Regel-Lockerung.
    if stats.gauss_chain_accepted
        && stats.broken_gesture_accepted
        && let Some(seed_candidate) = accepted_candidate.take()
    {
        let tail_before = stats.gauss_chain_variants_checked;
        stats.economy_constrained_tail_search = true;
        if let Some((mut tail_candidate, tail_selected, tail_details)) =
            run_gauss_gesture_chain_extension(
                &seed_candidate,
                options,
                next_iteration,
                &mut stats,
                true,
            )?
        {
            stats.gauss_tail_chain_variants_checked =
                stats.gauss_chain_variants_checked.saturating_sub(tail_before);
            stats.economy_constrained_tail_checked = stats.gauss_tail_chain_variants_checked;
            let seed_visible = visible_pitched_note_count(seed_candidate.layout_xml());
            let tail_visible = visible_pitched_note_count(tail_candidate.layout_xml());
            let tail_gain = tail_visible.saturating_sub(seed_visible);
            stats.economy_constrained_tail_floor_gain = tail_gain;
            stats.economy_constrained_tail_best_gain = tail_gain;
            stats.economy_constrained_tail_best_residual = if stats.economy_tail_residual_after > 0 {
                stats.economy_tail_residual_after
            } else {
                stats.gauss_chain_best_residual
            };
            if tail_gain > 0 {
                if let Some(tail_lineage) = tail_candidate.profile_lineage.with_tail() {
                    tail_candidate.profile_name = hybrid_tail_name(&tail_candidate.profile_name);
                    tail_candidate.profile_lineage = tail_lineage;
                    accepted_selected = tail_selected;
                    stats.gauss_tail_chain_accepted = true;
                    stats.gauss_tail_chain_net_notes = tail_gain;
                    stats.gauss_tail_chain_name = Some(tail_candidate.profile_name.clone());
                    stats.gauss_tail_chain_details = tail_details.clone();
                    stats.economy_constrained_tail_accepted = true;
                    accepted_candidate = Some(tail_candidate);
                } else {
                    accepted_candidate = Some(seed_candidate);
                }
            } else {
                accepted_candidate = Some(seed_candidate);
            }
        } else {
            stats.gauss_tail_chain_variants_checked =
                stats.gauss_chain_variants_checked.saturating_sub(tail_before);
            stats.economy_constrained_tail_checked = stats.gauss_tail_chain_variants_checked;
            stats.economy_constrained_tail_best_residual = if stats.economy_tail_residual_after > 0 {
                stats.economy_tail_residual_after
            } else {
                stats.gauss_chain_best_residual
            };
            accepted_candidate = Some(seed_candidate);
        }
    }
    if let Some(mut candidate) = accepted_candidate {
        let visible_after = visible_pitched_note_count(candidate.layout_xml());
        let decision = decide_hidden_chord_tone_recovery_adoption(
            HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
            hidden_chord_tone_recovery_decision_facts(
                &best.final_result,
                &candidate.final_result,
                stats.start_visible_notes,
                visible_after,
            ),
        );
        if !decision.adopt {
            return Ok(stats);
        }
        stats.accepted = true;
        stats.accepted_notes = visible_after.saturating_sub(stats.start_visible_notes);
        stats.end_visible_notes = visible_after;
        let anchor_details = consensus_probe_accepted_details(&accepted_selected, &slots);
        if stats.gauss_chain_accepted && stats.broken_gesture_accepted {
            if stats.gauss_tail_chain_accepted {
                if !candidate.profile_lineage.contains(CandidateLineageStep::Tail)
                    && let Some(tail_lineage) = candidate.profile_lineage.with_tail()
                {
                    candidate.profile_name = hybrid_tail_name(&candidate.profile_name);
                    candidate.profile_lineage = tail_lineage;
                }
            } else if !candidate.profile_lineage.contains(CandidateLineageStep::Combined) {
                candidate.profile_name = combined_name(&candidate.profile_name);
                candidate.profile_lineage = candidate.profile_lineage.with_broken_gesture();
            }
            stats.hybrid_acceptance_ledger = true;
            stats.hybrid_final_visible_gain = stats.accepted_notes;
            stats.hybrid_gauss_chain_net_notes = stats.gauss_chain_details.len().saturating_add(stats.gauss_tail_chain_net_notes);
            stats.hybrid_broken_gesture_net_notes = stats.broken_gesture_details.len();
            stats.hybrid_anchor_detail_lines = anchor_details.len();
            stats.hybrid_neighbor_detail_lines = stats.broken_gesture_details.len();
            stats.hybrid_profile_name = Some(candidate.profile_name.clone());
        }
        stats.accepted_name = Some(candidate.profile_name.clone());
        stats.accepted_iteration = Some(candidate.iteration);
        stats.accepted_details = anchor_details;
        if stats.gauss_gesture_accepted && !stats.gauss_gesture_details.is_empty() {
            stats.accepted_details.extend(stats.gauss_gesture_details.clone());
        }
        if stats.gauss_chain_accepted && !stats.gauss_chain_details.is_empty() {
            stats.accepted_details.extend(stats.gauss_chain_details.clone());
        }
        if stats.broken_gesture_accepted && !stats.broken_gesture_details.is_empty() {
            stats.accepted_details.extend(stats.broken_gesture_details.clone());
        }
        if stats.gauss_tail_chain_accepted && !stats.gauss_tail_chain_details.is_empty() {
            stats.accepted_details.extend(stats.gauss_tail_chain_details.clone());
        }
        if !accepted_han02_repair_details.is_empty() {
            stats.han02_repair_accepted = true;
            stats.han02_repair_details = accepted_han02_repair_details.clone();
        }
        previous_best = Some(std::mem::replace(best, candidate));
        crate::audit_only::destructive_edit_attribution::observe_selected_state(best.layout_xml());
        crate::audit_only::destructive_edit_attribution::observe_lineage_checkpoint(
            "HiddenChordSelectionExact",
            best.layout_xml(),
        );
    }
    if let Some(prev) = previous_best {
        let _ = prev;
    }
    Ok(stats)
}
fn collect_visible_gesture_attack_slots(xml: &str) -> Vec<VisibleChordToneSwapSlot> {
    let mut slots = Vec::new();
    let mut seen: BTreeSet<(usize, usize, i64, String, Option<i32>)> = BTreeSet::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, measure_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let mut cursor_by_voice: BTreeMap<String, i64> = BTreeMap::new();
        let mut last_non_chord_start_by_voice: BTreeMap<String, i64> = BTreeMap::new();
        let mut last_non_chord_duration_by_voice: BTreeMap<String, i64> = BTreeMap::new();
        let mut local_scan = 0_usize;
        while let Some(note_rel) = measure[local_scan..].find("<note") {
            let note_start = local_scan + note_rel;
            let Some(note_end_rel) = measure[note_start..].find("</note>") else { break; };
            let note_end = note_start + note_end_rel + "</note>".len();
            let block = &measure[note_start..note_end];
            let duration = tag_text_i64(block, "duration").unwrap_or(0);
            let voice = tag_text(block, "voice").unwrap_or_else(|| "1".to_string());
            let is_chord = block.contains("<chord");
            let rest = block.contains("<rest");
            let hidden = block.contains("print-object=\"no\"") || block.contains("print-object='no'");
            let start_tick = if is_chord {
                *last_non_chord_start_by_voice.get(&voice).unwrap_or(&0)
            } else {
                *cursor_by_voice.get(&voice).unwrap_or(&0)
            };
            let slot_duration = if is_chord {
                *last_non_chord_duration_by_voice.get(&voice).unwrap_or(&duration)
            } else {
                duration
            };
            if !is_chord {
                last_non_chord_start_by_voice.insert(voice.clone(), start_tick);
                last_non_chord_duration_by_voice.insert(voice.clone(), duration);
                *cursor_by_voice.entry(voice.clone()).or_insert(0) += duration;
            }
            if !rest && !hidden && block.contains("<pitch>") {
                let staff = tag_text(block, "staff").and_then(|v| v.parse::<usize>().ok()).unwrap_or(1);
                let midi = note_block_midi(block);
                let key = (measure_number, staff, start_tick, voice.clone(), midi);
                if seen.insert(key)
                    && let (Some(ps), Some(pe)) =
                        (block.find("<pitch>"), block.find("</pitch>"))
                    && ps < pe
                {
                    slots.push(VisibleChordToneSwapSlot {
                        measure: measure_number,
                        staff,
                        start_tick,
                        duration: slot_duration,
                        midi,
                        voice: voice.clone(),
                        note_start: measure_start + note_start,
                        note_end: measure_start + note_end,
                    });
                }
            }
            local_scan = note_end;
        }
        scan = measure_end;
    }
    slots
}
fn collect_visible_chord_tone_swap_slots(xml: &str) -> Vec<VisibleChordToneSwapSlot> {
    let mut slots = Vec::new();
    let mut scan = 0_usize;
    while let Some((measure_start, measure_end, measure_number)) = find_next_measure_bounds(xml, scan) {
        let measure = &xml[measure_start..measure_end];
        let mut cursor_by_voice: BTreeMap<String, i64> = BTreeMap::new();
        let mut last_non_chord_start_by_voice: BTreeMap<String, i64> = BTreeMap::new();
        let mut local_scan = 0_usize;
        while let Some(note_rel) = measure[local_scan..].find("<note") {
            let note_start = local_scan + note_rel;
            let Some(note_end_rel) = measure[note_start..].find("</note>") else { break; };
            let note_end = note_start + note_end_rel + "</note>".len();
            let block = &measure[note_start..note_end];
            let duration = tag_text_i64(block, "duration").unwrap_or(0);
            let voice = tag_text(block, "voice").unwrap_or_else(|| "1".to_string());
            let is_chord = block.contains("<chord");
            let rest = block.contains("<rest");
            let hidden = block.contains("print-object=\"no\"") || block.contains("print-object='no'");
            let start_tick = if is_chord {
                *last_non_chord_start_by_voice.get(&voice).unwrap_or(&0)
            } else {
                *cursor_by_voice.get(&voice).unwrap_or(&0)
            };
            if !is_chord {
                last_non_chord_start_by_voice.insert(voice.clone(), start_tick);
                *cursor_by_voice.entry(voice.clone()).or_insert(0) += duration;
            }
            if is_chord && !rest && !hidden && block.contains("<pitch>") {
                let staff = tag_text(block, "staff").and_then(|v| v.parse::<usize>().ok()).unwrap_or(1);
                let midi = note_block_midi(block);
                if let (Some(ps), Some(pe)) =
                    (block.find("<pitch>"), block.find("</pitch>"))
                    && ps < pe
                {
                    slots.push(VisibleChordToneSwapSlot {
                        measure: measure_number,
                        staff,
                        start_tick,
                        duration,
                        midi,
                        voice: voice.clone(),
                        note_start: measure_start + note_start,
                        note_end: measure_start + note_end,
                    });
                }
            }
            local_scan = note_end;
        }
        scan = measure_end;
    }
    slots
}
fn midi_distance(a: Option<i32>, b: Option<i32>) -> i32 {
    match (a, b) {
        (Some(x), Some(y)) => (x - y).abs(),
        _ => 999,
    }
}
/// Harmonic micro-delta (variant 2): the ordered print-state steps that exchange
/// which of two coincident chord tones is printed. `true` shows the concealed tone
/// (frozen owner #11), `false` conceals the shown tone (frozen owner #5); this
/// function writes nothing. Both played tones keep their own note element, pitch
/// and identity. Concealing requires explicit native-preserving spacing; tied tones are
/// refused: a continuation cannot change its printed disposition alone. The later
/// note comes first so the earlier note keeps its offsets.
fn coincident_chord_tone_print_exchange_steps(
    xml: &str,
    hidden: &HiddenChordToneRecoveryCandidate,
    slot: &VisibleChordToneSwapSlot,
) -> Option<[(usize, usize, bool); 2]> {
    let independent = |range: std::ops::Range<usize>|
        xml.get(range).is_some_and(single_note_gesture_support_is_independent);
    if !independent(slot.note_start..slot.note_end) || !independent(hidden.abs_start..hidden.abs_end) {
        return None;
    }
    // Frozen owner #5 preserves an explicit spacing attribute but otherwise
    // inserts print-spacing="no", which MuseScore imports as cursor movement.
    // Refuse before either writer runs unless this source note can remain a note.
    let shown_note = xml.get(slot.note_start..slot.note_end)?;
    let shown_document = roxmltree::Document::parse(shown_note).ok()?;
    if shown_document.root_element().attribute("print-spacing") != Some("yes") {
        return None;
    }
    let show = (hidden.abs_start, hidden.abs_end, true);
    let conceal = (slot.note_start, slot.note_end, false);
    Some(if hidden.abs_start > slot.note_start { [show, conceal] } else { [conceal, show] })
}

fn single_note_gesture_support_is_independent(note: &str) -> bool {
    !note.contains("<tie") && !note.contains("<tied")
}

#[cfg(test)]
#[path = "../../tests/unit/initial_chord_course_consumer_safety.rs"]
mod initial_chord_course_consumer_safety_tests;




fn remove_note_hidden_attrs_in_range(
    xml: &mut String,
    start: usize,
    end: usize,
) -> (bool, Option<ExactTransformationReceipt>) {
    crate::structural_write_authority::remove_note_hidden_attrs_in_range_authorized(xml, start, end)
}




