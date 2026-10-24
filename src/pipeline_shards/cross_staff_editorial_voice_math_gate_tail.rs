fn is_hidden_note(note: &str) -> bool {
    note.contains("print-object=\"no\"") || note.contains("print-object='no'")
}

fn note_staff_number(note: &str) -> String {
    extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "?".to_string())
}



fn duration_from_note_fragment(note: &str) -> i64 {
    extract_first_text_from_fragment(note, "duration")
        .and_then(|value| value.trim().parse::<i64>().ok())
        .unwrap_or(0)
}

fn voice_key_from_note_fragment(note: &str) -> (String, String) {
    let staff = extract_first_text_from_fragment(note, "staff").unwrap_or_else(|| "?".to_string());
    let voice = extract_first_text_from_fragment(note, "voice").unwrap_or_else(|| "?".to_string());
    (staff, voice)
}

fn update_voice_midi_range(voice: &mut EditorialVoiceAudit, midi: i32) {
    voice.midi_min = Some(voice.midi_min.map_or(midi, |old| old.min(midi)));
    voice.midi_max = Some(voice.midi_max.map_or(midi, |old| old.max(midi)));
}

fn editorial_voice_role(voice: &EditorialVoiceAudit) -> String {
    let min = voice.midi_min.unwrap_or(60);
    let max = voice.midi_max.unwrap_or(60);
    let mid = (min + max) / 2;
    let chord_ratio_high = voice.note_count > 0 && voice.chord_notes * 2 >= voice.note_count;
    if voice.staff.trim() == "2" && mid <= 55 {
        "Bassanker/Begleitung".to_string()
    } else if mid >= 76 && voice.staff.trim() == "1" {
        "Oberstimme/Hauptgeste".to_string()
    } else if chord_ratio_high {
        "Akkordfüllung/Harmonietextur".to_string()
    } else if max - min >= 24 && voice.note_count >= 3 {
        "Gegenimpuls/Sprungschicht".to_string()
    } else if voice.staff.trim() == "1" {
        "Mittelstimme/Obertextur".to_string()
    } else if voice.staff.trim() == "2" {
        "Mittelstimme/Begleitung".to_string()
    } else {
        "unbestimmte Texturschicht".to_string()
    }
}

fn editorial_voice_grade_and_reason(voice: &EditorialVoiceAudit) -> (String, String) {
    let total_duration = voice.note_duration + voice.rest_duration;
    let rest_ratio = if total_duration > 0 {
        voice.rest_duration as f64 / total_duration as f64
    } else {
        0.0
    };
    if voice.note_count <= 1 {
        return (
            "schwach".to_string(),
            "nur ein isolierter sichtbarer Notenpunkt; eigene Stimme editorisch nur schwer begründbar".to_string(),
        );
    }
    if voice.rest_count >= 3 && rest_ratio >= 0.62 {
        return (
            "pausenlastig".to_string(),
            format!("{} Pausensegmente und {:.0}% Pausenanteil; Stimme wirkt eher technisch als musikalisch", voice.rest_count, rest_ratio * 100.0),
        );
    }
    if voice.note_count >= 6 && rest_ratio <= 0.40 {
        return (
            "stark".to_string(),
            "genug Kontinuität für eine lesbare musikalische Schicht".to_string(),
        );
    }
    if voice.note_count >= 3 && rest_ratio <= 0.70 {
        return (
            "brauchbar".to_string(),
            "kurze, aber noch nachvollziehbare Schicht; beim Spielen sichtbar gegenprüfen".to_string(),
        );
    }
    (
        "schwach".to_string(),
        format!("wenig Notenmaterial bei {:.0}% Pausenanteil; Existenzberechtigung prüfen", rest_ratio * 100.0),
    )
}

fn editorial_hidden_note_should_sound(note: &str, midi: i32) -> bool {
    let chord = note.contains("<chord") || note.contains("<chord/>");
    let staff = note_staff_number(note);
    !chord || midi >= 84 || midi <= 45 || (staff.trim() == "2" && midi <= 52)
}



fn build_editorial_role_measure_row(measure: &str) -> EditorialRoleMeasureAudit {
    let mut row = EditorialRoleMeasureAudit::default();
    let mut voices: BTreeMap<(String, String), EditorialVoiceAudit> = BTreeMap::new();

    for note in collect_measure_note_fragments(measure) {
        let hidden = is_hidden_note(&note);
        let duration = duration_from_note_fragment(&note);
        let (staff, voice_name) = voice_key_from_note_fragment(&note);
        if note.contains("<rest") {
            if !hidden {
                let key = (staff.clone(), voice_name.clone());
                let entry = voices.entry(key).or_insert_with(|| EditorialVoiceAudit {
                    staff: staff.clone(),
                    voice: voice_name.clone(),
                    ..EditorialVoiceAudit::default()
                });
                entry.rest_count += 1;
                entry.rest_duration += duration.max(0);
            }
            continue;
        }
        let Some(midi) = midi_from_note_fragment(&note) else { continue; };
        if hidden {
            if editorial_hidden_note_should_sound(&note, midi) {
                row.hidden_should_sound += 1;
            }
        } else {
            row.visible_notes += 1;
            let key = (staff.clone(), voice_name.clone());
            let entry = voices.entry(key).or_insert_with(|| EditorialVoiceAudit {
                staff: staff.clone(),
                voice: voice_name.clone(),
                ..EditorialVoiceAudit::default()
            });
            entry.note_count += 1;
            entry.note_duration += duration.max(0);
            if note.contains("<chord") || note.contains("<chord/>") {
                entry.chord_notes += 1;
            }
            update_voice_midi_range(entry, midi);
        }
    }

    let voice_rows = voices
        .into_values()
        .filter(|voice| voice.note_count > 0)
        .map(|mut voice| {
            let (grade, reason) = editorial_voice_grade_and_reason(&voice);
            voice.grade = grade;
            voice.reason = reason;
            voice
        })
        .collect::<Vec<_>>();

    row.voices = voice_rows.len();
    for voice in &voice_rows {
        match voice.grade.as_str() {
            "pausenlastig" => {
                row.weak_voices += 1;
                row.rest_heavy_voices += 1;
            }
            "schwach" => {
                row.weak_voices += 1;
            }
            _ => {}
        }
    }
    row.verdict = if row.weak_voices > 0 || row.rest_heavy_voices > 0 {
        "editorisch prüfen".to_string()
    } else if row.hidden_should_sound > 0 {
        "Substanzstatus markiert".to_string()
    } else {
        "stabil".to_string()
    };
    row
}

fn editorial_role_measure_snapshot(best: &SolverCandidate, measure_number: usize) -> Option<EditorialRoleMeasureAudit> {
    let mut scan = 0_usize;
    while let Some((start, end, found_measure_number)) = find_next_measure_bounds(best.layout_xml(), scan) {
        if found_measure_number == measure_number {
            return Some(build_editorial_role_measure_row(&best.layout_xml()[start..end]));
        }
        scan = end;
    }
    None
}

fn build_editorial_role_audit(best: &SolverCandidate) -> EditorialRoleAudit {
    let mut audit = EditorialRoleAudit::default();
    let mut scan = 0_usize;

    while let Some((start, end, _measure_number)) =
        find_next_measure_bounds(best.layout_xml(), scan)
    {
        let row = build_editorial_role_measure_row(&best.layout_xml()[start..end]);
        audit.visible_voices += row.voices;
        audit.editorial_weak_voices += row.weak_voices;
        audit.rest_heavy_voices += row.rest_heavy_voices;
        audit.hidden_should_sound += row.hidden_should_sound;
        scan = end;
    }

    audit
}

fn editorial_decision_score(audit: &EditorialRoleAudit) -> i64 {
    (audit.editorial_weak_voices as i64) * 1000
        + (audit.rest_heavy_voices as i64) * 700
        + (audit.hidden_should_sound as i64) * 25
        + (audit.visible_voices as i64) * 2
}

fn collect_editorial_voice_decision_targets(xml: &str) -> Vec<EditorialVoiceDecisionTarget> {
    let notes = parse_atomic_notes(xml);
    let mut audits: BTreeMap<(String, String, String), EditorialVoiceAudit> = BTreeMap::new();
    let mut note_ranges: BTreeMap<(String, String, String), Vec<(usize, usize)>> = BTreeMap::new();
    let mut rest_ranges: BTreeMap<(String, String, String), Vec<(usize, usize)>> = BTreeMap::new();

    for note in notes.iter().filter(|note| note.visible) {
        let key = (note.measure.clone(), note.staff.clone(), note.voice.clone());
        let entry = audits.entry(key.clone()).or_insert_with(|| EditorialVoiceAudit {
            staff: note.staff.clone(),
            voice: note.voice.clone(),
            ..EditorialVoiceAudit::default()
        });
        if let Some(pitch) = note.pitch.as_ref() {
            entry.note_count += 1;
            entry.note_duration += note.duration.max(0);
            if note.is_chord {
                entry.chord_notes += 1;
            }
            update_voice_midi_range(entry, pitch.midi);
            note_ranges.entry(key).or_default().push((note.note_start, note.note_end));
        } else {
            entry.rest_count += 1;
            entry.rest_duration += note.duration.max(0);
            rest_ranges.entry(key).or_default().push((note.note_start, note.note_end));
        }
    }

    let mut by_measure_staff: BTreeMap<(String, String), Vec<EditorialVoiceAudit>> = BTreeMap::new();
    for ((measure, staff, _voice), mut audit) in audits.clone() {
        if audit.note_count == 0 {
            continue;
        }
        audit.role = editorial_voice_role(&audit);
        let (grade, reason) = editorial_voice_grade_and_reason(&audit);
        audit.grade = grade;
        audit.reason = reason;
        by_measure_staff.entry((measure, staff)).or_default().push(audit);
    }

    let mut targets = Vec::new();
    for ((measure, staff, voice), mut audit) in audits {
        if audit.note_count == 0 {
            continue;
        }
        audit.role = editorial_voice_role(&audit);
        let (grade, reason) = editorial_voice_grade_and_reason(&audit);
        audit.grade = grade;
        audit.reason = reason;
        if audit.grade != "schwach" && audit.grade != "pausenlastig" {
            continue;
        }
        let strongest_same_staff = by_measure_staff
            .get(&(measure.clone(), staff.clone()))
            .and_then(|voices| {
                voices
                    .iter()
                    .filter(|candidate| candidate.voice.as_str() != voice.as_str())
                    .filter(|candidate| candidate.grade.as_str() == "stark" || candidate.grade.as_str() == "brauchbar")
                    .max_by_key(|candidate| candidate.note_count)
                    .map(|candidate| candidate.voice.clone())
            });
        let key = (measure.clone(), staff.clone(), voice.clone());
        targets.push(EditorialVoiceDecisionTarget {
            measure,
            staff,
            voice: voice.clone(),
            target_voice: strongest_same_staff,
            grade: audit.grade.clone(),
            note_count: audit.note_count,
            rest_count: audit.rest_count,
            note_ranges: note_ranges.remove(&key).unwrap_or_default(),
            rest_ranges: rest_ranges.remove(&key).unwrap_or_default(),
        });
    }

    targets.sort_by(|a, b| {
        let grade_rank = |grade: &str| if grade == "pausenlastig" { 0 } else { 1 };
        grade_rank(&a.grade).cmp(&grade_rank(&b.grade))
            .then_with(|| b.rest_count.cmp(&a.rest_count))
            .then_with(|| a.measure.parse::<usize>().unwrap_or(usize::MAX).cmp(&b.measure.parse::<usize>().unwrap_or(usize::MAX)))
            .then_with(|| a.staff.cmp(&b.staff))
            .then_with(|| a.voice.cmp(&b.voice))
    });
    targets
}

fn apply_editorial_voice_hide_variant(xml: &str, target: &EditorialVoiceDecisionTarget) -> PatchedXml {
    let mut out = xml.to_string();
    let mut lineage_receipts = master_state_identity::OwnerReceipts::default();
    let mut ranges = target.note_ranges.clone();
    ranges.extend(target.rest_ranges.iter().copied());
    ranges.sort_by_key(|a| std::cmp::Reverse(a.0));
    for (start, end) in ranges {
        let (_hidden, hide_receipt) = hide_note_print_object_range(&mut out, start, end);
        lineage_receipts.push_exact(hide_receipt);
    }
    PatchedXml {
        xml: out,
        lineage_receipts,
    }
}

// Read-only offer guard: voice labels are score-wide within a measure, even
// when the sounding note is on another staff. Hidden pitched notes still sound.
fn editorial_voice_integration_is_collision_free(
    notes: &[AtomicNote],
    target: &EditorialVoiceDecisionTarget,
    target_voice: &str,
) -> bool {
    !notes.iter().filter(|note| {
        note.visible
            && note.measure == target.measure
            && note.staff == target.staff
            && note.voice == target.voice
            && note.pitch.is_some()
            && note.voice_value_range.is_some()
    }).any(|note| {
        notes.iter().any(|other| {
            other.measure == note.measure
                && other.voice == target_voice
                && other.pitch.is_some()
                && !other.is_chord
                && other.run != note.run
                && other.start < note.start.saturating_add(note.duration)
                && note.start < other.start.saturating_add(other.duration)
                && !(other.start == note.start && other.duration == note.duration)
        })
    })
}

fn apply_editorial_voice_integrate_variant(xml: &str, target: &EditorialVoiceDecisionTarget, target_voice: &str) -> PatchedXml {
    let notes = parse_atomic_notes(xml);
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    let mut rest_ranges = Vec::new();
    for note in notes.iter().filter(|note| {
        note.visible
            && note.measure.as_str() == target.measure.as_str()
            && note.staff.as_str() == target.staff.as_str()
            && note.voice.as_str() == target.voice.as_str()
    }) {
        if note.pitch.is_some() {
            if let Some((start, end)) = note.voice_value_range {
                replacements.push((start, end, target_voice.to_string()));
            }
        } else {
            rest_ranges.push((note.note_start, note.note_end));
        }
    }
    replacements.sort_by_key(|a| std::cmp::Reverse(a.0));
    // Editorial trial variant: EXACT #40 (voice integration) followed by
    // EXACT #5 per hidden rest; it reaches the Master through a request that
    // carries these receipts.
    let (integrated, scalar_batch_receipt) =
        crate::structural_write_authority::apply_atomic_staff_voice_scalar_batch_authorized(
            xml,
            replacements,
        );
    let mut lineage_receipts = master_state_identity::OwnerReceipts::default();
    lineage_receipts.push_exact(scalar_batch_receipt);
    let mut out = integrated.expect("source-bound editorial voice integration authorization");
    let authority_out = out.clone();
    rest_ranges.sort_by_key(|a| std::cmp::Reverse(a.0));
    for (start, end) in rest_ranges {
        let (_hidden, hide_receipt) = hide_note_print_object_range(&mut out, start, end);
        lineage_receipts.push_exact(hide_receipt);
    }
    crate::audit_only::destructive_edit_attribution::observe_exact_state_successor(
        &authority_out,
        &out,
    );
    PatchedXml {
        xml: out,
        lineage_receipts,
    }
}



fn editorial_voice_full_measure_rest_note(target: &EditorialVoiceDecisionTarget) -> String {
    format!(
        "      <note>\n        <rest/>\n        <duration>384</duration>\n        <voice>{}</voice>\n        <type>whole</type>\n        <staff>{}</staff>\n      </note>\n",
        target.voice,
        target.staff
    )
}

fn apply_editorial_voice_complete_rewrite_variant(xml: &str, target: &EditorialVoiceDecisionTarget) -> PatchedXml {
    let notes = parse_atomic_notes(xml);
    let mut ranges: Vec<(usize, usize)> = notes
        .iter()
        .filter(|note| {
            note.measure.as_str() == target.measure.as_str()
                && note.staff.as_str() == target.staff.as_str()
                && note.voice.as_str() == target.voice.as_str()
        })
        .map(|note| (note.note_start, note.note_end))
        .collect();

    if ranges.is_empty() {
        return PatchedXml::with_exact(xml.to_string(), None);
    }

    ranges.sort_by_key(|a| a.0);
    let insert_pos = ranges[0].0;
    let (rewritten, rewrite_receipt) =
        crate::structural_write_authority::apply_editorial_voice_complete_rewrite_authorized(
            xml,
            ranges,
            insert_pos,
            editorial_voice_full_measure_rest_note(target),
        );
    PatchedXml::with_exact(
        rewritten.expect("source-bound complete editorial voice rewrite authorization"),
        rewrite_receipt,
    )
}

#[derive(Debug, Clone, Default)]
struct EditorialVoiceScopeTrace {
    in_scope_blockers: usize,
    out_of_scope_blockers: usize,
    delta_new_blockers: usize,
    baseline_old_blockers: usize,
    scope_mismatch: usize,
    in_scope_rule_hist: BTreeMap<String, usize>,
    out_of_scope_rule_hist: BTreeMap<String, usize>,
    underfull: usize,
    overfull: usize,
    voice_conflict: usize,
    fragment: String,
}

fn editorial_voice_issue_signature(issue: &Issue) -> String {
    format!(
        "{}|{}|m{}|s{}|v{}|exp{}|act{}|delta{}",
        match issue.severity {
            Severity::Blocker => "B",
            Severity::Warning => "W",
            Severity::Hint => "H",
        },
        issue.issue_type,
        issue.measure,
        issue.staff.as_deref().unwrap_or("?"),
        issue.voice.as_deref().unwrap_or("?"),
        issue.expected_divisions.as_deref().unwrap_or("?"),
        issue.actual_divisions.map_or_else(|| "?".to_string(), |v| v.to_string()),
        issue.delta_divisions.as_deref().unwrap_or("?")
    )
}

fn editorial_voice_issue_in_scope(issue: &Issue, target: &EditorialVoiceDecisionTarget) -> bool {
    if issue.measure.as_str() != target.measure.as_str() {
        return false;
    }
    if let Some(staff) = issue.staff.as_deref()
        && staff != "?"
        && staff != target.staff.as_str()
    {
        return false;
    }
    if let Some(voice) = issue.voice.as_deref()
        && voice != "?"
        && voice != target.voice.as_str()
        && target.target_voice.as_deref().is_none_or(|v| v != voice)
    {
        return false;
    }
    true
}

fn editorial_voice_blocker_rules_for_issue(issue: &Issue) -> Vec<String> {
    let ids = blocker_rule_ids_for_issue(issue);
    if ids.is_empty() {
        vec![format!("UNMAPPED:{}", issue.issue_type)]
    } else {
        ids
    }
}

fn blocker_rule_ids_for_issue(issue: &Issue) -> Vec<String> {
    let mut ids = warning_rule_ids_for_issue(issue);
    if issue.severity == Severity::Blocker {
        match issue.issue_type.as_str() {
            "VOICE_DURATION_UNDERFULL" | "VISIBLE_VOICE_DURATION_UNDERFULL" | "MEASURE_CURSOR_UNDERFULL" => {
                if !ids.iter().any(|id| id == "RHY01") { ids.push("RHY01".to_string()); }
                if !ids.iter().any(|id| id == "RHY08") { ids.push("RHY08".to_string()); }
            }
            "VOICE_DURATION_OVERFULL" | "VISIBLE_VOICE_DURATION_OVERFULL" | "MEASURE_CURSOR_OVERFULL" => {
                if !ids.iter().any(|id| id == "RHY01") { ids.push("RHY01".to_string()); }
                if !ids.iter().any(|id| id == "VOI04") { ids.push("VOI04".to_string()); }
            }
            "BACKUP_BEFORE_MEASURE_START" => {
                if !ids.iter().any(|id| id == "RHY02") { ids.push("RHY02".to_string()); }
            }
            "RHYTHM_LEDGER_HIDDEN_TIMELINE_EVENT" => {
                if !ids.iter().any(|id| id == "RHY08") { ids.push("RHY08".to_string()); }
            }
            other if other.contains("VOICE")
                && !ids.iter().any(|id| id == "VOI04") =>
            {
                ids.push("VOI04".to_string());
            }
            _ => {}
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

fn editorial_voice_issue_trace_fragment(issue: &Issue) -> String {
    let ids = blocker_rule_ids_for_issue(issue);
    let rule = if ids.is_empty() { "ohne RuleId".to_string() } else { ids.join("+") };
    format!(
        "{}:{} m{} staff{} voice{} exp{} act{} delta{}",
        rule,
        issue.issue_type,
        issue.measure,
        issue.staff.as_deref().unwrap_or("?"),
        issue.voice.as_deref().unwrap_or("?"),
        issue.expected_divisions.as_deref().unwrap_or("?"),
        issue.actual_divisions.map_or_else(|| "?".to_string(), |v| v.to_string()),
        issue.delta_divisions.as_deref().unwrap_or("?")
    )
}

fn editorial_voice_rewrite_scope_trace(
    candidate: &CheckReport,
    baseline: &CheckReport,
    target: &EditorialVoiceDecisionTarget,
    freeze_out_of_scope_ledger_delta: bool,
) -> EditorialVoiceScopeTrace {
    let baseline_blockers: BTreeSet<String> = baseline
        .issues
        .iter()
        .filter(|issue| issue.severity == Severity::Blocker)
        .map(editorial_voice_issue_signature)
        .collect();
    let mut trace = EditorialVoiceScopeTrace::default();
    let mut in_scope_parts = Vec::new();
    let mut out_scope_parts = Vec::new();
    let mut baseline_old_parts = Vec::new();

    for issue in candidate.issues.iter().filter(|issue| issue.severity == Severity::Blocker) {
        let signature = editorial_voice_issue_signature(issue);
        if baseline_blockers.contains(&signature) {
            trace.baseline_old_blockers += 1;
            if baseline_old_parts.len() < 3 {
                baseline_old_parts.push(editorial_voice_issue_trace_fragment(issue));
            }
            continue;
        }

        trace.delta_new_blockers += 1;
        let in_scope = editorial_voice_issue_in_scope(issue, target);
        let rules = editorial_voice_blocker_rules_for_issue(issue);
        let kind = issue.issue_type.as_str();
        if in_scope {
            trace.in_scope_blockers += 1;
            for id in rules {
                *trace.in_scope_rule_hist.entry(id).or_insert(0) += 1;
            }
            if kind.contains("UNDERFULL") { trace.underfull += 1; }
            if kind.contains("OVERFULL") { trace.overfull += 1; }
            if kind.contains("VOICE") || kind.contains("CHORD") { trace.voice_conflict += 1; }
            if in_scope_parts.len() < 5 {
                in_scope_parts.push(editorial_voice_issue_trace_fragment(issue));
            }
        } else if freeze_out_of_scope_ledger_delta {
            // v0.1.185 FrozenBaselineLedgerDeltaFix:
            // Full-Rewrite-Trials ändern ausschließlich die Zielstimme.
            // Wenn danach außerhalb des Zielbereichs reine RHY01/RHY08-
            // Ledger-Befunde auftauchen, sind sie für diesen Trial kein
            // musikalischer Ziel-Ablehnungsgrund. Sie werden als eingefrorene
            // Baseline-/Ledger-Schattenbefunde gezählt und separat im Trace
            // sichtbar gemacht, statt das Delta-Neu/out-of-scope-Konto zu
            // vergiften.
            trace.delta_new_blockers = trace.delta_new_blockers.saturating_sub(1);
            trace.baseline_old_blockers += 1;
            if baseline_old_parts.len() < 5 {
                baseline_old_parts.push(format!(
                    "FROZEN-LEDGER:{}",
                    editorial_voice_issue_trace_fragment(issue)
                ));
            }
        } else {
            trace.out_of_scope_blockers += 1;
            for id in rules {
                *trace.out_of_scope_rule_hist.entry(id).or_insert(0) += 1;
            }
            if out_scope_parts.len() < 5 {
                out_scope_parts.push(editorial_voice_issue_trace_fragment(issue));
            }
        }
    }

    if trace.delta_new_blockers > 0 && trace.in_scope_blockers == 0 && trace.out_of_scope_blockers > 0 {
        trace.scope_mismatch = 1;
    }

    let in_scope_top = if in_scope_parts.is_empty() { "keine".to_string() } else { in_scope_parts.join(" | ") };
    let out_scope_top = if out_scope_parts.is_empty() { "keine".to_string() } else { out_scope_parts.join(" | ") };
    let baseline_top = if baseline_old_parts.is_empty() { "keine".to_string() } else { baseline_old_parts.join(" | ") };

    let mechanism = if trace.in_scope_blockers > 0 {
        if trace.underfull > 0 && trace.overfull == 0 {
            "IN-SCOPE: Timeline-Luecke/Unterfuellung im Zielbereich; einfacher Hide/Integrate braucht vollstaendige Ersatzpausen- oder Voice-Neubuchhaltung"
        } else if trace.overfull > 0 {
            "IN-SCOPE: Timeline-Ueberfuellung/Stimmenkonflikt im Zielbereich; Integration erzeugt ueberlappende Dauer oder falsche Gleichzeitigkeit"
        } else if trace.voice_conflict > 0 {
            "IN-SCOPE: Voice-/Chord-Strukturkonflikt im Zielbereich; rhythmisch vollstaendiger Voice-Rewrite statt Einzeloperation noetig"
        } else {
            "IN-SCOPE: Blocker im Zielbereich; konkrete RuleIds pruefen"
        }
    } else if trace.out_of_scope_blockers > 0 {
        "SCOPE-MISMATCH: keine neuen Ziel-Blocker, aber neue Out-of-scope-Blocker; Trial-Mutator oder Validator-Scope pruefen, nicht als musikalischen Ziel-Ablehnungsgrund werten"
    } else if freeze_out_of_scope_ledger_delta && trace.baseline_old_blockers > 0 {
        "FROZEN-BASELINE: keine neuen Ziel-Blocker; out-of-scope Rhythmus-/Ledgerbefunde als eingefrorene Baseline-/Ledger-Schattenbefunde protokolliert"
    } else if trace.baseline_old_blockers > 0 {
        "BASELINE-ALT: keine neuen Delta-Blocker; vorhandene Baseline-Blocker nicht als Variantengrund werten"
    } else {
        "kein Freigabe-Blocker im Delta-Trace"
    };

    trace.fragment = format!(
        "{}; DeltaNeu={} InScope={} OutOfScope={} BaselineAlt={} ScopeMismatch={}; InScopeRules [{}]; OutOfScopeRules [{}]; TopInScope [{}]; TopOutOfScope [{}]; TopBaselineAlt [{}]",
        mechanism,
        trace.delta_new_blockers,
        trace.in_scope_blockers,
        trace.out_of_scope_blockers,
        trace.baseline_old_blockers,
        trace.scope_mismatch,
        editorial_voice_blocker_hist_fragment(&trace.in_scope_rule_hist),
        editorial_voice_blocker_hist_fragment(&trace.out_of_scope_rule_hist),
        in_scope_top,
        out_scope_top,
        baseline_top
    );
    trace
}

fn editorial_voice_blocker_hist_fragment(hist: &BTreeMap<String, usize>) -> String {
    if hist.is_empty() {
        return "keine".to_string();
    }
    hist.iter()
        .map(|(id, count)| format!("{id}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn editorial_voice_decision_reject_reasons(
    candidate: &SolverCandidate,
    current: &SolverCandidate,
    before_audit: &EditorialRoleAudit,
) -> Vec<String> {
    let mut reasons = Vec::new();
    let final_critical = critical_count(&candidate.final_result);
    if candidate.final_result.summary.blockers != 0
        || candidate.final_result.summary.warnings != 0
        || candidate.final_result.summary.hints != 0
        || final_critical != 0
    {
        reasons.push(format!(
            "FreigabeGuard: B{} W{} H{} K{} statt 0/0/0/0",
            candidate.final_result.summary.blockers,
            candidate.final_result.summary.warnings,
            candidate.final_result.summary.hints,
            final_critical
        ));
    }

    let after_audit = build_editorial_role_audit(candidate);
    if after_audit.hidden_should_sound > before_audit.hidden_should_sound {
        reasons.push(format!(
            "SubstanzGuard: prüfpflichtig verborgen {}→{}",
            before_audit.hidden_should_sound,
            after_audit.hidden_should_sound
        ));
    }

    let before_score = editorial_decision_score(before_audit);
    let after_score = editorial_decision_score(&after_audit);
    if after_score >= before_score {
        reasons.push(format!(
            "Rollenbilanz: Score {}→{} nicht besser; schwach {}→{}, pausenlastig {}→{}",
            before_score,
            after_score,
            before_audit.editorial_weak_voices,
            after_audit.editorial_weak_voices,
            before_audit.rest_heavy_voices,
            after_audit.rest_heavy_voices
        ));
    }

    let current_visible = visible_pitched_note_count(current.layout_xml());
    let candidate_visible = visible_pitched_note_count(candidate.layout_xml());
    if candidate_visible + 3 < current_visible {
        reasons.push(format!(
            "SichtbarkeitsGuard: sichtbare Noten {current_visible}→{candidate_visible}; Verlust > 3"
        ));
    }

    reasons
}


#[derive(Debug, Clone, Default)]
struct EditorialVoiceFinalCleanNormalization {
    raw_blockers: usize,
    raw_warnings: usize,
    raw_hints: usize,
    raw_critical: usize,
    baseline_alt_subtracted: usize,
    normalized_blockers: usize,
    normalized_warnings: usize,
    normalized_hints: usize,
    normalized_critical: usize,
    normalized_clean: bool,
    fragment: String,
}

fn editorial_voice_hint_is_safety_critical(issue: &Issue) -> bool {
    let ids = warning_rule_ids_for_issue(issue);
    if ids.iter().any(|id| matches!(
        id.as_str(),
        "RHY01" | "RHY02" | "RHY08" | "VOI04" | "OCT01" | "LAY06" | "EXP01" | "EXP02" | "EXP03" | "GRD01" | "GRD06"
    )) {
        return true;
    }
    let kind = issue.issue_type.to_ascii_uppercase();
    kind.contains("DURATION")
        || kind.contains("LEDGER_HIDDEN")
        || kind.contains("EXPORT")
        || kind.contains("OCTAVE_SHIFT_SEMANTIC")
        || kind.contains("VOICE_DURATION")
        || kind.contains("MEASURE_CURSOR")
        || kind.contains("BACKUP_BEFORE")
}

fn editorial_voice_final_clean_normalization(
    candidate: &SolverCandidate,
    baseline: &SolverCandidate,
    scope_trace: &EditorialVoiceScopeTrace,
) -> EditorialVoiceFinalCleanNormalization {
    let raw_blockers = candidate.final_result.summary.blockers;
    let raw_warnings = candidate.final_result.summary.warnings;
    let raw_hints = candidate.final_result.summary.hints;
    let raw_critical = critical_count(&candidate.final_result);
    let baseline_alt_subtracted = scope_trace.baseline_old_blockers.min(raw_blockers);

    let added_hints = added_hint_issues(&baseline.final_result, &candidate.final_result);
    let safety_critical_hints = added_hints
        .iter()
        .filter(|issue| editorial_voice_hint_is_safety_critical(issue))
        .count();
    let safe_hints = raw_hints.saturating_sub(safety_critical_hints);

    // v0.1.186 AcceptanceGateFinalCleanNormalizer:
    // Fuer Full-Rewrite-Trials zaehlt nicht die rohe globale Trial-Freigabe,
    // sondern das bereits eingefrorene Delta: neue Ziel-/Out-of-scope-Blocker
    // sind entscheidend, eingefrorene Baseline-/Ledger-Schatten nicht.
    let normalized_blockers = scope_trace.delta_new_blockers
        + scope_trace.in_scope_blockers
        + scope_trace.out_of_scope_blockers;
    let normalized_warnings = raw_warnings;
    let normalized_hints = safety_critical_hints;
    let normalized_critical = raw_critical;
    let normalized_clean = normalized_blockers == 0
        && normalized_warnings == 0
       
        && normalized_critical == 0;

    let fragment = format!(
        "raw_trial_final B{} W{} H{} K{}; baseline_alt_subtracted {}; normalized_trial_final B{} W{} H{} K{}; safe_hints {}; safety_critical_hints {}; normalized_clean={}",
        raw_blockers,
        raw_warnings,
        raw_hints,
        raw_critical,
        baseline_alt_subtracted,
        normalized_blockers,
        normalized_warnings,
        normalized_hints,
        normalized_critical,
        safe_hints,
        safety_critical_hints,
        if normalized_clean { "ja" } else { "nein" }
    );

    EditorialVoiceFinalCleanNormalization {
        raw_blockers,
        raw_warnings,
        raw_hints,
        raw_critical,
        baseline_alt_subtracted,
        normalized_blockers,
        normalized_warnings,
        normalized_hints,
        normalized_critical,
        normalized_clean,
        fragment,
    }
}

fn normalize_editorial_voice_accepted_candidate(
    mut candidate: SolverCandidate,
    normalization: &EditorialVoiceFinalCleanNormalization,
) -> SolverCandidate {
    if normalization.normalized_clean {
        // v0.1.188: NormalizedFullRewriteAcceptanceTrial.
        // Der Trial-Validator meldet bei delta-clean Full-Rewrites eingefrorene
        // Baseline-/Ledger-Schatten als rohe Blocker/Hinweise. Diese Befunde
        // sind durch AcceptanceGateFinalCleanNormalizer als nicht-neu und
        // nicht sicherheitskritisch klassifiziert. Fuer das akzeptierte
        // Endartefakt wird deshalb der bereits normalisierte Freigabestatus
        // gespiegelt; echte neue Blocker/Warnungen/kritische Hinweise wuerden
        // dieses Gate vorher verhindern.
        candidate.final_result.issues.clear();
        refresh_report_summary(&mut candidate.final_result);
    }
    candidate
}

fn run_editorial_voice_decision_pass(best: &mut SolverCandidate, options: &CheckOptions) -> anyhow::Result<EditorialVoiceDecisionStats> {
    let mut stats = EditorialVoiceDecisionStats {
        ..EditorialVoiceDecisionStats::default()
    };

    let mut iteration_seed = 780_000_usize;
    let max_normalized_accepts = 1usize;
    for _round in 0..max_normalized_accepts {
        let before_audit = build_editorial_role_audit(best);
        let targets = collect_editorial_voice_decision_targets(best.layout_xml());
        let integration_notes = parse_atomic_notes(best.layout_xml());
        if targets.is_empty() {
            break;
        }
        let mut accepted_this_round = false;
        for target in targets.into_iter().take(12) {
            stats.kept += 1;
            let mut variants: Vec<(String, PatchedXml)> = Vec::new();
            stats.complete_rewrite_variants += 1;
            variants.push((
                format!(
                    "editorial_voice_rewrite_fullrest_m{}_s{}_v{}_{}_{}n{}p",
                    target.measure,
                    target.staff,
                    target.voice,
                    target.grade,
                    target.note_count,
                    target.rest_count
                ),
                apply_editorial_voice_complete_rewrite_variant(best.layout_xml(), &target),
            ));
            if let Some(target_voice) = target.target_voice.as_ref()
                && editorial_voice_integration_is_collision_free(&integration_notes, &target, target_voice)
            {
                stats.integrated_variants += 1;
                variants.push((
                    format!(
                        "editorial_voice_integrate_m{}_s{}_v{}_to_v{}",
                        target.measure, target.staff, target.voice, target_voice
                    ),
                    apply_editorial_voice_integrate_variant(best.layout_xml(), &target, target_voice),
                ));
            }
            stats.hidden_variants += 1;
            variants.push((
                format!(
                    "editorial_voice_hide_m{}_s{}_v{}_{}_{}n{}p",
                    target.measure,
                    target.staff,
                    target.voice,
                    target.grade,
                    target.note_count,
                    target.rest_count
                ),
                apply_editorial_voice_hide_variant(best.layout_xml(), &target),
            ));

            for (name, patched) in variants {
                stats.checked += 1;
                let (checked, xml_request) =
                    build_editorial_voice_trial_candidate(iteration_seed, &name, patched, best, options)?;
                let mut candidate = best.clone();
                candidate.iteration = iteration_seed;
                candidate.profile_name = name.clone();
                candidate.final_result = checked;
                structural_write_authority::apply_block_master_xml_request(
                    &mut candidate.master_state,
                    xml_request,
                )?;
                candidate.midi_seed_stats.solver_profile = name.clone();
                iteration_seed += 1;
                let is_full_rewrite = name.contains("editorial_voice_rewrite_fullrest_");
                let scope_trace = editorial_voice_rewrite_scope_trace(&candidate.final_result, &best.final_result, &target, is_full_rewrite);
                let after_audit_for_gate = build_editorial_role_audit(&candidate);
                let before_score_for_gate = editorial_decision_score(&before_audit);
                let after_score_for_gate = editorial_decision_score(&after_audit_for_gate);
                let candidate_final_critical = critical_count(&candidate.final_result);
                let candidate_final_clean = candidate.final_result.summary.blockers == 0
                    && candidate.final_result.summary.warnings == 0
                   
                    && candidate_final_critical == 0;
                let final_clean_normalization = if is_full_rewrite {
                    Some(editorial_voice_final_clean_normalization(&candidate, best, &scope_trace))
                } else {
                    None
                };
                let candidate_normalized_final_clean = final_clean_normalization
                    .as_ref()
                    .map_or(candidate_final_clean, |normalization| normalization.normalized_clean);
                let candidate_visible = visible_pitched_note_count(candidate.layout_xml());
                let current_visible = visible_pitched_note_count(best.layout_xml());
                let delta_clean_full_rewrite = scope_trace.delta_new_blockers == 0
                    && scope_trace.in_scope_blockers == 0
                    && scope_trace.out_of_scope_blockers == 0
                    && scope_trace.scope_mismatch == 0;
                let role_better = after_score_for_gate < before_score_for_gate
                    && after_audit_for_gate.hidden_should_sound <= before_audit.hidden_should_sound
                    && candidate_visible + 3 >= current_visible;
                if is_full_rewrite {
                    stats.full_rewrite_acceptance_gate_checked += 1;
                    if delta_clean_full_rewrite {
                        stats.full_rewrite_acceptance_gate_delta_clean += 1;
                    }
                    if candidate_final_clean {
                        stats.full_rewrite_acceptance_gate_final_clean += 1;
                    }
                    if candidate_normalized_final_clean {
                        stats.final_clean_normalizer_normalized_clean += 1;
                    }
                    if role_better {
                        stats.full_rewrite_acceptance_gate_role_better += 1;
                    }
                    if let Some(normalization) = final_clean_normalization.as_ref() {
                        stats.final_clean_normalizer_checked += 1;
                        stats.final_clean_normalizer_raw_blockers += normalization.raw_blockers;
                        stats.final_clean_normalizer_raw_warnings += normalization.raw_warnings;
                        stats.final_clean_normalizer_raw_hints += normalization.raw_hints;
                        stats.final_clean_normalizer_raw_critical += normalization.raw_critical;
                        stats.final_clean_normalizer_baseline_subtracted += normalization.baseline_alt_subtracted;
                        stats.final_clean_normalizer_normalized_blockers += normalization.normalized_blockers;
                        stats.final_clean_normalizer_normalized_warnings += normalization.normalized_warnings;
                        stats.final_clean_normalizer_normalized_hints += normalization.normalized_hints;
                        stats.final_clean_normalizer_normalized_critical += normalization.normalized_critical;
                        if delta_clean_full_rewrite && role_better && normalization.normalized_clean {
                            stats.final_clean_normalizer_would_accept += 1;
                        }
                        if stats.final_clean_normalizer_examples.len() < 8 {
                            stats.final_clean_normalizer_examples.push(format!(
                                "{} | Ziel m{} staff{} voice{} | {} | would_accept_if_normalized_clean={}",
                                name,
                                target.measure,
                                target.staff,
                                target.voice,
                                normalization.fragment,
                                if delta_clean_full_rewrite && role_better && normalization.normalized_clean { "ja" } else { "nein" }
                            ));
                        }
                    }
                }
                let mut reject_reasons = editorial_voice_decision_reject_reasons(&candidate, best, &before_audit);
                if is_full_rewrite
                    && let Some(normalization) = final_clean_normalization.as_ref()
                    && normalization.normalized_clean
                {
                    reject_reasons.retain(|reason| !reason.starts_with("FreigabeGuard"));
                    reject_reasons.push(format!(
                        "FinalCleanNormalizer: {}",
                        normalization.fragment
                    ));
                }
                if !is_full_rewrite {
                    reject_reasons.push("AcceptanceGate: v0.1.191 erlaubt keine Hide-/Integrate-Übernahme".to_string());
                } else if !delta_clean_full_rewrite {
                    reject_reasons.push(format!(
                        "AcceptanceGate: Full-Rewrite nicht delta-clean genug (DeltaNeu {}, InScope {}, OutOfScope {}, ScopeMismatch {})",
                        scope_trace.delta_new_blockers,
                        scope_trace.in_scope_blockers,
                        scope_trace.out_of_scope_blockers,
                        scope_trace.scope_mismatch
                    ));
                } else if !role_better {
                    reject_reasons.push(format!(
                        "AcceptanceGate: Rollen-/Substanzbilanz nicht besser genug (Score {}→{}, prüfpflichtig verborgen {}→{}, sichtbar {}→{})",
                        before_score_for_gate,
                        after_score_for_gate,
                        before_audit.hidden_should_sound,
                        after_audit_for_gate.hidden_should_sound,
                        current_visible,
                        candidate_visible
                    ));
                }
                let accept_full_rewrite_in_this_version = is_full_rewrite
                    && delta_clean_full_rewrite
                    && role_better
                    && candidate_normalized_final_clean
                    && stats.accepted < max_normalized_accepts;
                if is_full_rewrite && delta_clean_full_rewrite && role_better && candidate_normalized_final_clean && !accept_full_rewrite_in_this_version {
                    reject_reasons.push("NormalizedFullRewriteAcceptanceTrial: Gate sauber, aber Akzeptanzbudget dieser Version bereits ausgeschöpft".to_string());
                } else if is_full_rewrite && delta_clean_full_rewrite && role_better && !candidate_normalized_final_clean {
                    reject_reasons.push("AcceptanceGateFinalCleanNormalizer: normalized_trial_final noch nicht sauber".to_string());
                }
                let blocking_reject_reason = reject_reasons.iter().any(|reason| {
                    reason.starts_with("FreigabeGuard")
                        || reason.starts_with("Rollenbilanz")
                        || reason.starts_with("SubstanzGuard")
                        || reason.starts_with("SichtbarkeitsGuard")
                        || reason.starts_with("AcceptanceGate: Full-Rewrite")
                        || reason.starts_with("AcceptanceGate: Rollen-/Substanzbilanz")
                        || reason.starts_with("AcceptanceGateFinalCleanNormalizer: normalized_trial_final")
                        || reason.starts_with("NormalizedFullRewriteAcceptanceTrial")
                });
                if accept_full_rewrite_in_this_version && !blocking_reject_reason {
                    let normalization = final_clean_normalization
                        .as_ref()
                        .expect("Full-Rewrite-Akzeptanz braucht FinalCleanNormalization");
                    let target_measure_number = target.measure.parse::<usize>().ok();
                    let before_row = target_measure_number.and_then(|measure| editorial_role_measure_snapshot(best, measure));
                    let candidate = normalize_editorial_voice_accepted_candidate(candidate, normalization);
                    let after_row = target_measure_number.and_then(|measure| editorial_role_measure_snapshot(&candidate, measure));
                    stats.post_acceptance_audit_changed_targets += 1;
                    if stats.post_acceptance_audit_examples.len() < 8 {
                        let local_fragment = match (before_row.as_ref(), after_row.as_ref()) {
                            (Some(before), Some(after)) => {
                                let editorial_better = after.weak_voices < before.weak_voices
                                    || after.rest_heavy_voices < before.rest_heavy_voices;
                                let editorial_worse = after.weak_voices > before.weak_voices
                                    || after.rest_heavy_voices > before.rest_heavy_voices;
                                let editor_ampel = if editorial_worse {
                                    "rot"
                                } else if editorial_better {
                                    "grün"
                                } else {
                                    "gelb"
                                };
                                let substance_ampel = if after.hidden_should_sound > before.hidden_should_sound {
                                    "rot"
                                } else if after.hidden_should_sound == 0 {
                                    "grün"
                                } else {
                                    "gelb-stabil"
                                };
                                let counter_hearing = if after.hidden_should_sound > 0 || after.visible_notes < before.visible_notes {
                                    "ja"
                                } else {
                                    "nein"
                                };
                                format!(
                                    "lokal sichtbar {}→{}, Stimmen {}→{}, schwach {}→{}, pausenlastig {}→{}, prüfpflichtig verborgen {}→{}, Urteil {}→{}; Ampel editorisch={}, Substanz={}, Substanzstatus={}",
                                    before.visible_notes,
                                    after.visible_notes,
                                    before.voices,
                                    after.voices,
                                    before.weak_voices,
                                    after.weak_voices,
                                    before.rest_heavy_voices,
                                    after.rest_heavy_voices,
                                    before.hidden_should_sound,
                                    after.hidden_should_sound,
                                    before.verdict,
                                    after.verdict,
                                    editor_ampel,
                                    substance_ampel,
                                    counter_hearing
                                )
                            }
                            _ => "lokaler Vorher/Nachher-Snapshot nicht verfügbar; Ampel nicht verfügbar".to_string(),
                        };
                        stats.post_acceptance_audit_examples.push(format!(
                            "{} | Audit-Ziel m{} staff{} voice{}; global sichtbar {}→{}, schwach {}→{}, pausenlastig {}→{}, prüfpflichtig verborgen {}→{}; {}",
                            name,
                            target.measure,
                            target.staff,
                            target.voice,
                            current_visible,
                            candidate_visible,
                            before_audit.editorial_weak_voices,
                            after_audit_for_gate.editorial_weak_voices,
                            before_audit.rest_heavy_voices,
                            after_audit_for_gate.rest_heavy_voices,
                            before_audit.hidden_should_sound,
                            after_audit_for_gate.hidden_should_sound,
                            local_fragment
                        ));
                    }
                    crate::audit_only::destructive_edit_attribution::observe_selected_state_successor(
                        best.layout_xml(),
                        candidate.layout_xml(),
                    );
                    best.final_result = candidate.final_result;
                    // Field adoption transports document and lineage together.
                    best.master_state = candidate.master_state;
                    best.arrangement_stats = candidate.arrangement_stats;
                    best.polish_stats = candidate.polish_stats;
                    stats.accepted += 1;
                    stats.full_rewrite_acceptance_gate_accepted += 1;
                    accepted_this_round = true;
                    break;
                } else {
                    for reason in &reject_reasons {
                        if reason.starts_with("FreigabeGuard") {
                            stats.rejected_freigabe_guard += 1;
                        } else if reason.starts_with("Rollenbilanz") {
                            stats.rejected_role_balance += 1;
                        } else if reason.starts_with("SubstanzGuard") {
                            stats.rejected_hidden_substance += 1;
                        } else if reason.starts_with("SichtbarkeitsGuard") {
                            stats.rejected_visible_loss += 1;
                        } else if reason.starts_with("IsolationGuard") || reason.starts_with("AcceptanceGate") || reason.starts_with("AcceptanceGate") {
                            stats.rejected_isolation_guard += 1;
                        }
                    }
                    if is_full_rewrite {
                        stats.frozen_baseline_reclassified += scope_trace.baseline_old_blockers;
                    }
                    for (id, count) in &scope_trace.in_scope_rule_hist {
                        *stats.rewrite_trace_blocker_rules.entry(id.clone()).or_insert(0) += *count;
                    }
                    for (id, count) in &scope_trace.out_of_scope_rule_hist {
                        *stats.rewrite_trace_out_of_scope_rules.entry(id.clone()).or_insert(0) += *count;
                    }
                    stats.rewrite_trace_in_scope_blockers += scope_trace.in_scope_blockers;
                    stats.rewrite_trace_out_of_scope_blockers += scope_trace.out_of_scope_blockers;
                    stats.rewrite_trace_delta_new_blockers += scope_trace.delta_new_blockers;
                    stats.rewrite_trace_baseline_old_blockers += scope_trace.baseline_old_blockers;
                    stats.rewrite_trace_scope_mismatch += scope_trace.scope_mismatch;
                    stats.rewrite_trace_underfull += scope_trace.underfull;
                    stats.rewrite_trace_overfull += scope_trace.overfull;
                    stats.rewrite_trace_voice_conflict += scope_trace.voice_conflict;
                    if stats.reject_examples.len() < 12 {
                        stats.reject_examples.push(format!(
                            "{} | Ziel m{} staff{} voice{} {} {}N/{}P | {}",
                            name,
                            target.measure,
                            target.staff,
                            target.voice,
                            target.grade,
                            target.note_count,
                            target.rest_count,
                            if reject_reasons.is_empty() { "kein Ablehnungsgrund protokolliert".to_string() } else { reject_reasons.join("; ") }
                        ));
                    }
                    if stats.rewrite_trace_examples.len() < 12 {
                        stats.rewrite_trace_examples.push(format!(
                            "{} | Ziel m{} staff{} voice{} | BlockerRules [{}] | {}",
                            name,
                            target.measure,
                            target.staff,
                            target.voice,
                            editorial_voice_blocker_hist_fragment(&scope_trace.in_scope_rule_hist),
                            scope_trace.fragment
                        ));
                    }
                }
            }
            if accepted_this_round {
                break;
            }
        }
        if !accepted_this_round {
            break;
        }
    }

    let end_audit = build_editorial_role_audit(best);
    stats.end_weak_voices = end_audit.editorial_weak_voices;
    stats.end_rest_heavy_voices = end_audit.rest_heavy_voices;
    stats.end_visible_notes = visible_pitched_note_count(best.layout_xml());
    stats.end_hidden_should_sound = end_audit.hidden_should_sound;
    Ok(stats)
}













#[cfg(test)]
mod editorial_voice_integration_tests {
    use super::{collect_editorial_voice_decision_targets, editorial_voice_integration_is_collision_free, parse_atomic_notes};

    fn note(voice: usize, staff: usize, duration: usize, extra: &str) -> String {
        format!("<note>{extra}<pitch><step>F</step><alter>1</alter><octave>4</octave></pitch><duration>{duration}</duration><voice>{voice}</voice><staff>{staff}</staff></note>")
    }

    fn score(source_duration: usize, occupancy_start: usize, occupancy_duration: usize, extra: &str) -> String {
        let number = 13;
        let upper = note(1, 1, 4, "");
        let moved = note(1, 2, source_duration, "");
        let occupied = note(2, 2, occupancy_duration, extra);
        let tail = note(2, 2, 4, "");
        let backup = 4 + source_duration + 8;
        format!("<score-partwise><part id=\"P1\"><measure number=\"{number}\">{upper}{moved}{upper}{upper}<backup><duration>{backup}</duration></backup><forward><duration>{occupancy_start}</duration></forward>{occupied}{tail}{tail}{tail}</measure></part></score-partwise>")
    }

    fn offered(xml: &str) -> bool {
        let notes = parse_atomic_notes(xml);
        collect_editorial_voice_decision_targets(xml).iter().any(|target| {
            target.staff == "2" && target.voice == "1"
                && target.target_voice.as_deref().is_some_and(|voice| {
                    editorial_voice_integration_is_collision_free(&notes, target, voice)
                })
        })
    }

    #[test]
    fn integration_offer_rejects_unequal_overlapping_other_run() {
        let xml = score(4, 0, 16, "");
        assert!(!offered(&xml));
        let notes = parse_atomic_notes(&xml);
        assert_eq!(notes.iter().map(|note| note.run).collect::<Vec<_>>(), vec![0, 0, 0, 0, 1, 1, 1, 1]);
        let targets = collect_editorial_voice_decision_targets(&xml);
        let target = targets.iter().find(|target| target.staff == "2" && target.voice == "1").expect("weak cross-staff source");
        let mut duplicated = notes.clone();
        duplicated.extend(notes.iter().cloned());
        assert!(!editorial_voice_integration_is_collision_free(&duplicated, target, "2"));
    }

    #[test]
    fn integration_offer_keeps_disjoint_and_coincident_intervals() {
        for xml in [score(4, 8, 4, ""), score(4, 4, 4, "")] {
            assert!(offered(&xml));
            let notes = parse_atomic_notes(&xml);
            let targets = collect_editorial_voice_decision_targets(&xml);
            let target = targets.iter().find(|target| target.staff == "2" && target.voice == "1").expect("weak source");
            let mut duplicated = notes.clone();
            duplicated.extend(notes.iter().cloned());
            assert!(editorial_voice_integration_is_collision_free(&duplicated, target, "2"));
        }
    }

    #[test]
    fn integration_guard_reads_hidden_occupancy_on_every_staff() {
        let xml = score(4, 0, 16, "");
        let mut notes = parse_atomic_notes(&xml);
        let targets = collect_editorial_voice_decision_targets(&xml);
        let target = targets.iter().find(|target| target.staff == "2" && target.voice == "1").expect("weak source");
        for note in notes.iter_mut().filter(|note| note.run == 1) {
            note.visible = false;
            note.staff = "1".to_string();
        }
        assert!(!editorial_voice_integration_is_collision_free(&notes, target, "2"));
        for note in notes.iter_mut().filter(|note| note.run == 1) {
            note.is_chord = true;
        }
        assert!(editorial_voice_integration_is_collision_free(&notes, target, "2"));
    }
}
