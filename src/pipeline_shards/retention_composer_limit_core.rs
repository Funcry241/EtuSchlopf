fn retention_shadow_event_key(note: &AtomicNote) -> Option<String> {
    let pitch = note.pitch.as_ref()?;
    Some(format!(
        "area={}|onset={}|dur={}|midi={}",
        note.measure, note.start, note.duration, pitch.midi
    ))
}

fn retention_shadow_area_key(note: &AtomicNote) -> String {
    format!("{}@{}", note.measure, note.start)
}

fn retention_shadow_classify_note(note: &AtomicNote, local_density: usize) -> (&'static str, &'static str, &'static str, &'static str) {
    let Some(pitch) = note.pitch.as_ref() else {
        return ("REST_OR_UNKNOWN", "ABANDON_RISK", "kein klingendes Pitch-Event", "nicht fuer Retention-Rueckholung verwenden");
    };
    let lower_cross_staff = note.staff.trim() == "1" && pitch.midi <= 52;
    let upper_cross_staff = note.staff.trim() == "2" && pitch.midi >= 64;
    if lower_cross_staff || upper_cross_staff {
        return (
            "STAFFVOICE_REDISTRIBUTE_FIBER",
            "REDISTRIBUTE",
            "Pitch liegt gegen die aktuelle Staff-/Hand-Faser",
            "erst HandCurve/VoiceFiber pruefen, nicht loeschen",
        );
    }
    if local_density >= 10 && note.duration <= 48 {
        return (
            "DENSE_CLUSTER_ABANDONMENT_REVIEW",
            "ABANDON_RISK",
            "sehr dichter verlorener Klebecluster mit kurzer Dauer",
            "nur bei echter Aufgabegefahr reduzieren, sonst als Cluster neu verteilen",
        );
    }
    if pitch.midi <= 48 {
        return (
            "BASS_FOUNDATION_LOSS",
            "ALLOW_HARD",
            "Bass-/Fundamentsubstanz verloren",
            "schwer zulassen oder in Bassfigur verdichten, nicht Komfort-loeschen",
        );
    }
    if note.duration <= 48 {
        return (
            "RHYTHMIC_DRIVE_LOSS",
            "ALLOW_HARD",
            "kurzdauernder rhythmischer Antrieb verloren",
            "Retention-first pruefen; Schwierigkeit allein ist kein Ablehnungsgrund",
        );
    }
    if note.staff.trim() == "1" && pitch.midi >= 64 && !note.is_chord {
        return (
            "MOTIF_UPPER_LINE_LOSS",
            "ALLOW_HARD",
            "obere Linien-/Motivsubstanz verloren",
            "als harte Haupt-/Nebenlinie zulassen, bevor Playability filtert",
        );
    }
    if note.is_chord {
        return (
            "HARMONIC_COLOR_CHORD_LOSS",
            "ALLOW_HARD",
            "Akkord-/Farbsubstanz verloren",
            "als harte Farbe oder arpeggierte Ersatzgestalt pruefen",
        );
    }
    (
        "INNER_TEXTURE_RETENTION_DEBT",
        "ALLOW_HARD",
        "Innen-/Texturmaterial ohne belegte Aufgabegefahr verloren",
        "erst musikalisch begruenden; sonst in ALLOW_HARD-Shadow aufnehmen",
    )
}

fn pct_rounded(numerator: usize, denominator: usize) -> usize {
    match std::num::NonZeroUsize::new(denominator) {
        Some(denominator) => {
            let denominator = denominator.get();
            (numerator * 100 + denominator / 2) / denominator
        }
        None => 0,
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct RetentionFirstP2GeneratedOccurrenceAuditRow {
    source_note_start: usize,
    source_note_end: usize,
    measure: usize,
    pitched_ordinal: usize,
    sequence_fingerprint_fnv64: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RetentionFirstP2RetainedOccurrenceAuditRow {
    input_scope: usize,
    input_pitched_ordinal: usize,
    input_onset: i64,
    input_pitch: i32,
    input_chord_syntax: bool,
    measure: usize,
    pitched_ordinal: usize,
    sequence_fingerprint_fnv64: u64,
}

#[derive(Debug, Clone, Default)]
struct RetentionFirstP2AssertiveReturnWriteAudit {
    actual_pitch_retention: usize,
    render_written: bool,
    occurrence_receipt_complete: bool,
    retained_occurrences: Vec<RetentionFirstP2RetainedOccurrenceAuditRow>,
    generated_occurrences: Vec<RetentionFirstP2GeneratedOccurrenceAuditRow>,
}

#[derive(Debug, Clone)]
struct RetentionFirstP2PendingInsertion {
    insert_at: usize,
    payload: String,
    source_note_start: usize,
    source_note_end: usize,
    generated_note_offset: usize,
    generated_note_len: usize,
}

#[derive(Debug, Clone)]
struct RetentionFirstP2GeneratedSpan {
    source_note_start: usize,
    source_note_end: usize,
    generated_note_start: usize,
    generated_note_end: usize,
}

#[derive(Debug, Clone)]
struct RetentionFirstP2RetainedSpan {
    input_scope: usize,
    input_pitched_ordinal: usize,
    input_onset: i64,
    input_pitch: i32,
    input_chord_syntax: bool,
    persisted_note_start: usize,
    persisted_note_end: usize,
}

fn remove_print_object_no_from_note_open(block: &str) -> String {
    let Some(open_end) = block.find('>') else { return block.to_string(); };
    let mut open = block[..open_end].to_string();
    for needle in [" print-object=\"no\"", " print-object='no'", "print-object=\"no\"", "print-object='no'"] {
        open = open.replace(needle, "");
    }
    let rest = &block[open_end..];
    format!("{open}{rest}")
}

/// A returned occurrence must take part in the consumer import. An opening tag with
/// print-spacing="no" marks a cursor-only carrier that the consumer treats like a
/// forward; a copy of it cannot fulfil a return, so the candidate is refused before
/// any byte is materialized. `None` means the opening tag is not a parsable note.
fn retention_p2_source_participates_natively(block: &str) -> Option<bool> {
    let open_end = block.find('>')?;
    let open = block[..open_end].trim_end_matches('/');
    let probe = format!("{open}/>");
    let document = roxmltree::Document::parse(&probe).ok()?;
    let element = document.root_element();
    (element.tag_name().name() == "note").then(|| element.attribute("print-spacing") != Some("no"))
}

/// Admission for one already selected return source; refusal never backfills the set.
fn retention_p2_candidate_source_is_eligible(
    block: &str, note: &AtomicNote, master_key_multiplicity: &BTreeMap<String, usize>,
) -> bool {
    retention_p2_source_participates_natively(block) == Some(true)
        && retention_shadow_event_key(note)
            .and_then(|key| master_key_multiplicity.get(&key).copied()) == Some(1)
}

fn make_retention_p2_inserted_chord_block(block: &str) -> String {
    let cleaned = remove_print_object_no_from_note_open(block);
    crate::structural_write_authority::ensure_bound_chord_marker_authorized(&cleaned).0
        .expect("bound chord-marker capability must receive one complete note")
}

fn same_atomic_anchor_quality(anchor: &AtomicNote, lost: &AtomicNote) -> i32 {
    let mut score = 0_i32;
    if anchor.measure == lost.measure { score += 1000; }
    if anchor.start == lost.start { score += 1000; }
    if anchor.staff == lost.staff { score += 100; }
    if anchor.voice == lost.voice { score += 100; }
    if anchor.duration == lost.duration { score += 80; }
    if anchor.visible { score += 20; }
    if anchor.pitch.is_some() { score += 10; }
    score
}

fn apply_retention_first_p2_assertive_return_write_explicit_gate(
    master_xml: &str,
    render_xml: &str,
) -> ((String, RetentionFirstP2AssertiveReturnWriteAudit), Option<ExactTransformationReceipt>) {
    let mut applied_geometry = Vec::new();
    
    let output = {
    let master_notes = parse_atomic_notes(master_xml);
    let render_notes = parse_atomic_notes(render_xml);
    let master_pitched = master_notes.iter().filter(|note| note.pitch.is_some()).count();
    let render_pitched_before = render_notes.iter().filter(|note| note.pitch.is_some()).count();

    let mut render_counts: BTreeMap<String, usize> = BTreeMap::new();
    for note in render_notes.iter().filter(|note| note.pitch.is_some()) {
        if let Some(key) = retention_shadow_event_key(note) {
            *render_counts.entry(key).or_insert(0) += 1;
        }
    }

    let mut lost_notes: Vec<AtomicNote> = Vec::new();
    for note in master_notes.iter().filter(|note| note.pitch.is_some()) {
        let Some(key) = retention_shadow_event_key(note) else { continue; };
        if let Some(count) = render_counts.get_mut(&key)
            && *count > 0
        {
            *count -= 1;
            continue;
        }
        lost_notes.push(note.clone());
    }

    let mut density: BTreeMap<String, usize> = BTreeMap::new();
    for note in &lost_notes {
        *density.entry(retention_shadow_area_key(note)).or_insert(0) += 1;
    }
    let rhythmic_drive = lost_notes
        .iter()
        .filter(|note| {
            let local_density = density.get(&retention_shadow_area_key(note)).copied().unwrap_or(1);
            let (group, retention_class, _, _) = retention_shadow_classify_note(note, local_density);
            group == "RHYTHMIC_DRIVE_LOSS" && retention_class == "ALLOW_HARD"
        })
        .cloned()
        .collect::<Vec<_>>();
    let p2_requested = rhythmic_drive.len().div_ceil(2);
    // Even with no inserted copy, retained occurrences cross this boundary.
    // Continue their exact ranges through the same read-only receipt construction.

    let mut insertions = Vec::<RetentionFirstP2PendingInsertion>::new();
    // The byte inventory carries no survivor identity. A lost decision is bound to one source
    // only if no other master pitched note shares its comparison key; otherwise the survivor
    // could be any of them, so every candidate with a shared key is refused (no reconstruction).
    let mut master_key_multiplicity: BTreeMap<String, usize> = BTreeMap::new();
    for note in master_notes.iter().filter(|note| note.pitch.is_some()) {
        if let Some(key) = retention_shadow_event_key(note) {
            *master_key_multiplicity.entry(key).or_insert(0) += 1;
        }
    }
    for lost in rhythmic_drive.iter().take(p2_requested) {
        // Refusal keeps the selected set: a refused candidate is not replaced by a later one.
        if !retention_p2_candidate_source_is_eligible(
            &master_xml[lost.note_start..lost.note_end], lost, &master_key_multiplicity,
        ) {
            continue;
        }
        let anchor = render_notes
            .iter()
            .filter(|anchor| {
                anchor.measure == lost.measure
                    && anchor.start == lost.start
                    && anchor.visible
                    && anchor.pitch.is_some()
            })
            .max_by_key(|anchor| same_atomic_anchor_quality(anchor, lost));
        if let Some(anchor) = anchor {
            let block = &master_xml[lost.note_start..lost.note_end];
            let chord_block = make_retention_p2_inserted_chord_block(block);
            let master_raw_note_index = master_notes
                .iter()
                .filter(|note| note.measure == lost.measure && note.note_start < lost.note_start)
                .count();
            let anchor_raw_note_index = render_notes
                .iter()
                .filter(|note| note.measure == anchor.measure && note.note_start < anchor.note_start)
                .count();
            let lost_pitch = lost.pitch.as_ref().map(|pitch| pitch.midi);
            let anchor_pitch = anchor.pitch.as_ref().map(|pitch| pitch.midi);
            println!(
                "ETUSCHLOPF: RetentionFirstP2OccurrenceCausality: lost_m={} lost_raw={} lost_o={} lost_dur={} lost_p={:?} lost_s={} lost_v={} lost_chord={} -> anchor_raw={} anchor_o={} anchor_dur={} anchor_p={:?} anchor_s={} anchor_v={} anchor_chord={} insert_after_byte={}",
                lost.measure,
                master_raw_note_index,
                lost.start,
                lost.duration,
                lost_pitch,
                lost.staff,
                lost.voice,
                lost.is_chord,
                anchor_raw_note_index,
                anchor.start,
                anchor.duration,
                anchor_pitch,
                anchor.staff,
                anchor.voice,
                anchor.is_chord,
                anchor.note_end,
            );
            let payload = format!("\n{chord_block}");
            insertions.push(RetentionFirstP2PendingInsertion {
                insert_at: anchor.note_end,
                source_note_start: lost.note_start,
                source_note_end: lost.note_end,
                generated_note_offset: 1,
                generated_note_len: chord_block.len(),
                payload,
            });
        }
    }

    insertions.sort_by_key(|insertion| std::cmp::Reverse(insertion.insert_at));
    let mut patched = render_xml.to_string();
    let mut generated_spans = Vec::<RetentionFirstP2GeneratedSpan>::new();
    let mut next_pitched_by_scope = std::collections::BTreeMap::<usize, usize>::new();
    let mut retained_spans = Vec::<RetentionFirstP2RetainedSpan>::new();
    for note in render_notes.iter().filter(|note| note.pitch.is_some()) {
        let Ok(input_scope) = note.measure.parse::<usize>() else {
            continue;
        };
        let Some(input_pitch) = note.pitch.as_ref().map(|pitch| pitch.midi) else {
            continue;
        };
        let input_pitched_ordinal = next_pitched_by_scope.entry(input_scope).or_default();
        retained_spans.push(RetentionFirstP2RetainedSpan {
            input_scope,
            input_pitched_ordinal: *input_pitched_ordinal,
            input_onset: note.start,
            input_pitch,
            input_chord_syntax: note.is_chord,
            persisted_note_start: note.note_start,
            persisted_note_end: note.note_end,
        });
        *input_pitched_ordinal = input_pitched_ordinal.saturating_add(1);
    }
    for insertion in insertions {
        if insertion.insert_at > patched.len() {
            continue;
        }
        let payload_len = insertion.payload.len();
        for span in &mut generated_spans {
            if span.generated_note_start >= insertion.insert_at {
                span.generated_note_start = span.generated_note_start.saturating_add(payload_len);
                span.generated_note_end = span.generated_note_end.saturating_add(payload_len);
            }
        }
        for span in &mut retained_spans {
            if span.persisted_note_start >= insertion.insert_at {
                span.persisted_note_start = span.persisted_note_start.saturating_add(payload_len);
                span.persisted_note_end = span.persisted_note_end.saturating_add(payload_len);
            }
        }
        applied_geometry.push((insertion.insert_at, insertion.insert_at, insertion.payload.clone()));
        patched.insert_str(insertion.insert_at, &insertion.payload);
        let generated_note_start = insertion
            .insert_at
            .saturating_add(insertion.generated_note_offset);
        let generated_note_end = generated_note_start.saturating_add(insertion.generated_note_len);
        generated_spans.push(RetentionFirstP2GeneratedSpan {
            source_note_start: insertion.source_note_start,
            source_note_end: insertion.source_note_end,
            generated_note_start,
            generated_note_end,
        });
    }

    let mut retained_occurrences = Vec::<RetentionFirstP2RetainedOccurrenceAuditRow>::new();
    for span in &retained_spans {
        if let Ok((measure, pitched_ordinal, sequence_fingerprint_fnv64)) =
            crate::structural_write_authority::locate_final_chord_render_occurrence_by_exact_range(
                &patched,
                span.persisted_note_start,
                span.persisted_note_end,
            )
        {
            retained_occurrences.push(RetentionFirstP2RetainedOccurrenceAuditRow {
                input_scope: span.input_scope,
                input_pitched_ordinal: span.input_pitched_ordinal,
                input_onset: span.input_onset,
                input_pitch: span.input_pitch,
                input_chord_syntax: span.input_chord_syntax,
                measure,
                pitched_ordinal,
                sequence_fingerprint_fnv64,
            });
        }
    }
    let mut generated_occurrences = Vec::<RetentionFirstP2GeneratedOccurrenceAuditRow>::new();
    for span in &generated_spans {
        if let Ok((measure, pitched_ordinal, sequence_fingerprint_fnv64)) =
            crate::structural_write_authority::locate_final_chord_render_occurrence_by_exact_range(
                &patched,
                span.generated_note_start,
                span.generated_note_end,
            )
        {
            generated_occurrences.push(RetentionFirstP2GeneratedOccurrenceAuditRow {
                source_note_start: span.source_note_start,
                source_note_end: span.source_note_end,
                measure,
                pitched_ordinal,
                sequence_fingerprint_fnv64,
            });
        }
    }
    let restored = generated_spans.len();
    let occurrence_receipt_complete = generated_occurrences.len() == restored
        && retained_occurrences.len() == render_pitched_before;
    let render_pitched_after = render_pitched_before.saturating_add(restored).min(master_pitched);
    (
        patched,
        RetentionFirstP2AssertiveReturnWriteAudit {
            actual_pitch_retention: pct_rounded(render_pitched_after, master_pitched),
            render_written: restored > 0,
            occurrence_receipt_complete,
            retained_occurrences,
            generated_occurrences,
        },
    )
};
    let geometry = crate::master_state_identity::exact_script_from_descending_application(applied_geometry);
    let receipt = Some(output.0.as_str()).filter(|post| *post != render_xml).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow64ExactMint { _private: () }, render_xml, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

pub(crate) struct RenderRow64ExactMint { _private: () }

#[cfg(test)]
#[path = "../../tests/unit/retention_p2_native_participation.rs"]
mod retention_p2_native_participation_tests;
