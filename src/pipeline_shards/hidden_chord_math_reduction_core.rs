type HiddenChordToneRecoveryChainUnit = ((usize, usize), Vec<HiddenChordToneRecoveryCandidate>, i64, i64);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HiddenChordToneRecoveryMaterializationProjection {
    HarmonicPitchSwap,
    ConsensusGesture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HiddenChordToneRecoveryDecisionFacts {
    baseline_blockers: usize,
    baseline_warnings: usize,
    baseline_critical: usize,
    baseline_hints: usize,
    candidate_blockers: usize,
    candidate_warnings: usize,
    candidate_critical: usize,
    candidate_hints: usize,
    visible_before: usize,
    visible_after: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HiddenChordToneRecoveryDecision {
    adopt: bool,
}

fn hidden_chord_tone_recovery_decision_facts(
    baseline: &CheckReport,
    candidate: &CheckReport,
    visible_before: usize,
    visible_after: usize,
) -> HiddenChordToneRecoveryDecisionFacts {
    HiddenChordToneRecoveryDecisionFacts {
        baseline_blockers: baseline.summary.blockers,
        baseline_warnings: baseline.summary.warnings,
        baseline_critical: critical_count(baseline),
        baseline_hints: baseline.summary.hints,
        candidate_blockers: candidate.summary.blockers,
        candidate_warnings: candidate.summary.warnings,
        candidate_critical: critical_count(candidate),
        candidate_hints: candidate.summary.hints,
        visible_before,
        visible_after,
    }
}

fn decide_hidden_chord_tone_recovery_adoption(
    projection: HiddenChordToneRecoveryMaterializationProjection,
    facts: HiddenChordToneRecoveryDecisionFacts,
) -> HiddenChordToneRecoveryDecision {
    let hints_stable = facts.candidate_hints <= facts.baseline_hints;

    match projection {
        HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture => {
            HiddenChordToneRecoveryDecision {
                adopt: facts.candidate_blockers == 0
                    && facts.candidate_warnings == 0
                    && facts.candidate_critical == 0
                    && hints_stable
                    && facts.visible_after > facts.visible_before,
            }
        }
        HiddenChordToneRecoveryMaterializationProjection::HarmonicPitchSwap => {
            HiddenChordToneRecoveryDecision {
                adopt: facts.candidate_blockers <= facts.baseline_blockers
                    && facts.candidate_warnings <= facts.baseline_warnings
                    && facts.candidate_critical <= facts.baseline_critical
                    && hints_stable,
            }
        }
    }
}


#[cfg(test)]
mod hidden_chord_tone_recovery_decision_owner_tests {
    use super::{
        HiddenChordToneRecoveryDecisionFacts, HiddenChordToneRecoveryMaterializationProjection,
        decide_hidden_chord_tone_recovery_adoption,
    };

    fn clean_facts() -> HiddenChordToneRecoveryDecisionFacts {
        HiddenChordToneRecoveryDecisionFacts {
            baseline_blockers: 0,
            baseline_warnings: 0,
            baseline_critical: 0,
            baseline_hints: 2,
            candidate_blockers: 0,
            candidate_warnings: 0,
            candidate_critical: 0,
            candidate_hints: 2,
            visible_before: 10,
            visible_after: 10,
        }
    }

    #[test]
    fn harmonic_swap_can_adopt_without_visible_note_gain() {
        let decision = decide_hidden_chord_tone_recovery_adoption(
            HiddenChordToneRecoveryMaterializationProjection::HarmonicPitchSwap,
            clean_facts(),
        );
        assert!(decision.adopt);
    }



    #[test]
    fn consensus_requires_positive_visible_gain() {
        let no_gain = decide_hidden_chord_tone_recovery_adoption(
            HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
            clean_facts(),
        );
        assert!(!no_gain.adopt);

        let mut with_gain = clean_facts();
        with_gain.visible_after += 1;
        let gain = decide_hidden_chord_tone_recovery_adoption(
            HiddenChordToneRecoveryMaterializationProjection::ConsensusGesture,
            with_gain,
        );
        assert!(gain.adopt);
    }
}



fn collect_hidden_chord_tone_recovery_candidates(xml: &str, limit: usize) -> Vec<HiddenChordToneRecoveryCandidate> {
    let mut raw = Vec::new();
    let mut hidden_by_measure: BTreeMap<usize, usize> = BTreeMap::new();
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
            let start_tick = if is_chord {
                *last_non_chord_start_by_voice.get(&voice).unwrap_or(&0)
            } else {
                *cursor_by_voice.get(&voice).unwrap_or(&0)
            };
            if !is_chord {
                last_non_chord_start_by_voice.insert(voice.clone(), start_tick);
                *cursor_by_voice.entry(voice.clone()).or_insert(0) += duration;
            }
            let hidden = block.contains("print-object=\"no\"") || block.contains("print-object='no'");
            if hidden && is_chord && !rest && block.contains("<pitch>") {
                let staff = tag_text(block, "staff").and_then(|v| v.parse::<usize>().ok()).unwrap_or(1);
                let midi = note_block_midi(block);
                let abs_start = measure_start + note_start;
                let abs_end = measure_start + note_end;
                let pitch_xml = match (block.find("<pitch>"), block.find("</pitch>")) {
                    (Some(ps), Some(pe)) if ps < pe => block[ps..pe + "</pitch>".len()].to_string(),
                    _ => String::new(),
                };
                *hidden_by_measure.entry(measure_number).or_insert(0) += 1;
                raw.push(HiddenChordToneRecoveryCandidate {
                    measure: measure_number,
                    staff,
                    start_tick,
                    duration,
                    midi,
                    voice: voice.clone(),
                    pitch_xml,
                    abs_start,
                    abs_end,
                    score: 0,
                });
            }
            local_scan = note_end;
        }
        scan = measure_end;
    }

    for cand in &mut raw {
        let measure_density = hidden_by_measure.get(&cand.measure).copied().unwrap_or(0) as i64;
        let register_bonus = match cand.midi {
            Some(m) if m >= 81 => 22,
            Some(m) if m <= 47 => 18,
            Some(m) if (60..=80).contains(&m) => 10,
            Some(_) => 4,
            None => 0,
        };
        let staff_bonus = if cand.staff == 2 { 8 } else { 4 };
        let duration_bonus = if cand.duration >= 48 { 8 } else if cand.duration >= 24 { 5 } else { 2 };
        // Komponisten-Treue vor blindem Prozent: erst Takte mit viel verborgener
        // Klangfarbe, dann Register-/Staff-Konflikte. Keine festen Takte.
        cand.score = measure_density * 40 + register_bonus + staff_bonus + duration_bonus - (cand.start_tick / 96).max(0);
    }

    raw.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.start_tick.cmp(&b.start_tick))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.midi.unwrap_or(0).cmp(&b.midi.unwrap_or(0)))
    });
    raw.truncate(limit);
    raw
}



fn musical_reduction_candidate_class(cand: &HiddenChordToneRecoveryCandidate) -> &'static str {
    let midi = cand.midi.unwrap_or(64);
    let register_anchor = midi >= 84 || midi <= 45 || (cand.staff == 2 && midi <= 52);
    let metric_anchor = cand.start_tick % 96 == 0 || cand.duration >= 48;
    if cand.score >= 760 || (register_anchor && metric_anchor && cand.score >= 500) {
        "muss klingen"
    } else if cand.score >= 560 || register_anchor {
        "als Ersatzfigur"
    } else if cand.score >= 360 {
        "nur Klangfarbe"
    } else {
        "darf fehlen"
    }
}

fn reduction_role_candidate_confirmed(cand: &HiddenChordToneRecoveryCandidate) -> bool {
    // Gleiche musikalische Schwelle wie EditorialRoleAudit für verborgene
    // Akkord-Innentöne: Ein Innenton ist nur dann wirklich prüfpflichtig,
    // wenn er Register-, Bassanker- oder extreme Oberstimmenfunktion hat.
    // Damit überstimmt v0.5.1 nicht mehr blind die editorische Rollenlogik.
    let Some(midi) = cand.midi else { return false; };
    midi >= 84 || midi <= 45 || (cand.staff == 2 && midi <= 52)
}

fn reduction_role_consensus_class(cand: &HiddenChordToneRecoveryCandidate) -> &'static str {
    let reduction = musical_reduction_candidate_class(cand);
    let role_confirmed = reduction_role_candidate_confirmed(cand);
    match (reduction, role_confirmed) {
        ("muss klingen", true) => "konsens-muss",
        ("als Ersatzfigur", true) => "konsens-ersatzfigur",
        ("muss klingen", false) => "konflikt-herabgestuft",
        ("als Ersatzfigur", false) => {
            if cand.score >= 820 && hand_plan_candidate_risk(cand) < 96 {
                "konsens-ersatzfigur"
            } else {
                "konflikt-herabgestuft"
            }
        }
        ("nur Klangfarbe", _) => "nur Klangfarbe",
        _ => "darf fehlen",
    }
}

// v0.5.3: alte v0.5.1-Reduktionsauswahl entfernt.
// Die aktive Auswahl läuft ausschließlich über select_consensus_gesture_candidates.

fn consensus_gesture_candidate_weight(cand: &HiddenChordToneRecoveryCandidate) -> i64 {
    let class_bonus = match reduction_role_consensus_class(cand) {
        "konsens-muss" => 6200,
        "konsens-ersatzfigur" => 3900,
        _ => -5000,
    };
    let midi = cand.midi.unwrap_or(64);
    let register_fit = if cand.staff == 1 {
        if (64..=92).contains(&midi) { 420 } else { 0 }
    } else if (40..=72).contains(&midi) {
        420
    } else {
        0
    };
    let rhythm_fit = if cand.start_tick % 96 == 0 {
        360
    } else if cand.start_tick % 48 == 0 {
        240
    } else if cand.start_tick % 24 == 0 {
        120
    } else {
        -260
    };
    let duration_fit = if cand.duration == 24 || cand.duration == 48 {
        220
    } else if cand.duration >= 72 {
        -180
    } else {
        0
    };
    class_bonus + cand.score + register_fit + rhythm_fit + duration_fit - hand_plan_candidate_risk(cand) * 3
}






fn math_time_compatible(cand: &HiddenChordToneRecoveryCandidate, slot: &VisibleChordToneSwapSlot) -> bool {
    cand.start_tick == slot.start_tick
        || (cand.start_tick - slot.start_tick).abs() <= 24
        || (cand.duration == slot.duration && (cand.start_tick - slot.start_tick).abs() <= 48)
}

fn math_staff_compatible(cand: &HiddenChordToneRecoveryCandidate, slot: &VisibleChordToneSwapSlot) -> bool {
    if cand.staff == slot.staff {
        return true;
    }
    match cand.midi.unwrap_or(64) {
        0..=55 => slot.staff == 2,
        56..=76 => true,
        _ => slot.staff == 1,
    }
}

fn math_voice_compatible(cand: &HiddenChordToneRecoveryCandidate, slot: &VisibleChordToneSwapSlot) -> bool {
    cand.voice == slot.voice || cand.start_tick == slot.start_tick
}

fn math_rhythm_safe(cand: &HiddenChordToneRecoveryCandidate, slot: &VisibleChordToneSwapSlot) -> bool {
    cand.start_tick % 24 == 0
        && matches!(cand.duration, 24 | 48 | 72 | 96)
        && (cand.start_tick == slot.start_tick || cand.duration == slot.duration)
}
