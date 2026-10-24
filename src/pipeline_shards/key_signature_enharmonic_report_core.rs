// ---- merged from src/pipeline_shards/part_43.rs ----
// v0.10.1477 FinalSectionTonalKeySignatureFunctionalRespellingCanonicalMasterMirror.
// The historical fixed-position key-signature and global pitch-class writers
// remain retired. The existing writer may change either the opening key signature
// when a score-wide, identity-free editorial selector proves one stable tonal
// frame, or exactly one final-section key signature when the already calibrated
// six-channel segmentation audit yields one unique full-evidence boundary, the
// new profile remains dominant through the tail, and the final bass confirms its
// tonic. It never selects by project name or absolute measure number and remains
// fail-closed on modal, returning, non-saving or competing-key conflicts. Only
// key-diatonic spellings and context-proven raised minor leading tones may be
// respelled; sounding pitch, onset, duration, staff and voice stay
// invariant.

#[derive(Debug, Clone)]
struct KeySignatureEnharmonicSpellingRenderWriteAudit {
    source_label: String,
    output_path: String,
    evidence_intake_open: bool,
    structural_scope_bound: bool,
    attempted: bool,
    written: bool,
    selected_profile: String,
    global_profile: String,
    source_fifths: Option<i32>,
    target_fifths: Option<i32>,
    final_bass_pc: Option<i32>,
    accidental_cost_open: usize,
    accidental_cost_target: usize,
    accidental_cost_best: usize,
    accidental_saving_percent: i32,
    selection_reason: String,
    e_flat_before: usize,
    e_flat_after: usize,
    b_flat_before: usize,
    b_flat_after: usize,
    d_sharp_before: usize,
    d_sharp_after: usize,
    a_sharp_before: usize,
    a_sharp_after: usize,
    f_natural_before: usize,
    f_natural_after: usize,
    e_sharp_before: usize,
    e_sharp_after: usize,
    respelling_candidates: usize,
    diatonic_respellings: usize,
    minor_leading_respellings: usize,
    first_visible_pitch_before: String,
    first_visible_pitch_after: String,
    last_visible_pitch_before: String,
    last_visible_pitch_after: String,
    parse_before_ok: bool,
    parse_after_ok: bool,
    backup_forward_before_ok: bool,
    backup_forward_after_ok: bool,
    sounding_stable: bool,
    note_count_before: usize,
    note_count_after: usize,
    visible_pitched_before: usize,
    visible_pitched_after: usize,
    visible_rests_before: usize,
    visible_rests_after: usize,
    final_section_mode: bool,
    final_section_boundary: Option<usize>,
    final_section_end: Option<usize>,
    final_section_tail_notes: usize,
    final_section_chromatic_notes: usize,
    final_section_accidental_before: usize,
    final_section_accidental_after: usize,
    final_section_saving_percent: i32,
    hardcoded_measure_logic: bool,
    render_xml_write: bool,
    music_mutation: bool,
    verdict: String,
}

impl Default for KeySignatureEnharmonicSpellingRenderWriteAudit {
    fn default() -> Self {
        Self {
            source_label: String::new(),
            output_path: String::new(),
            evidence_intake_open: false,
            structural_scope_bound: false,
            attempted: false,
            written: false,
            selected_profile: "none".to_string(),
            global_profile: "unbound".to_string(),
            source_fifths: None,
            target_fifths: None,
            final_bass_pc: None,
            accidental_cost_open: 0,
            accidental_cost_target: 0,
            accidental_cost_best: 0,
            accidental_saving_percent: 0,
            selection_reason: "not_evaluated".to_string(),
            e_flat_before: 0,
            e_flat_after: 0,
            b_flat_before: 0,
            b_flat_after: 0,
            d_sharp_before: 0,
            d_sharp_after: 0,
            a_sharp_before: 0,
            a_sharp_after: 0,
            f_natural_before: 0,
            f_natural_after: 0,
            e_sharp_before: 0,
            e_sharp_after: 0,
            respelling_candidates: 0,
            diatonic_respellings: 0,
            minor_leading_respellings: 0,
            first_visible_pitch_before: "unbound".to_string(),
            first_visible_pitch_after: "unbound".to_string(),
            last_visible_pitch_before: "unbound".to_string(),
            last_visible_pitch_after: "unbound".to_string(),
            parse_before_ok: false,
            parse_after_ok: false,
            backup_forward_before_ok: false,
            backup_forward_after_ok: false,
            sounding_stable: false,
            note_count_before: 0,
            note_count_after: 0,
            visible_pitched_before: 0,
            visible_pitched_after: 0,
            visible_rests_before: 0,
            visible_rests_after: 0,
            final_section_mode: false,
            final_section_boundary: None,
            final_section_end: None,
            final_section_tail_notes: 0,
            final_section_chromatic_notes: 0,
            final_section_accidental_before: 0,
            final_section_accidental_after: 0,
            final_section_saving_percent: 0,
            hardcoded_measure_logic: false,
            render_xml_write: false,
            music_mutation: false,
            verdict: "scorewide_tonal_key_signature_not_attempted".to_string(),
        }
    }
}

fn apply_key_signature_enharmonic_spelling_render_write(
    xml: &str,
    source_label: &str,
    output_path: &Path,
) -> ((String, KeySignatureEnharmonicSpellingRenderWriteAudit), Option<BlockTransformationReceipt>) {
    
    
    let output = {
    let (before_events, backup_forward_before_ok) = collect_render_xml_note_events(xml);
    let before_atomic = parse_atomic_notes(xml);
    let before_sounding = render_visible_sounding_signature(&before_events);
    let (first_pitch, last_pitch) = first_last_visible_pitch_labels(&before_atomic);
    let counts_before = enharmonic_spelling_counts(&before_atomic);
    let visible_pitched_before = before_events
        .iter()
        .filter(|event| !event.is_hidden && event.midi.is_some())
        .count();
    let visible_rests_before = before_events
        .iter()
        .filter(|event| !event.is_hidden && event.is_rest)
        .count();
    let decision = scorewide_tonal_key_signature_decision(xml, &before_atomic);
    let scorewide_target = decision.candidate.map(|profile| profile.fifths);
    let scorewide_scope_bound = decision.candidate.is_some()
        && decision.opening_measure.is_some()
        && decision.source_fifths == Some(0);
    let scorewide_attempted = scorewide_scope_bound
        && scorewide_target.is_some()
        && scorewide_target != decision.source_fifths;
    let final_section = if scorewide_attempted {
        None
    } else {
        final_section_tonal_key_candidate(xml, &before_atomic)
    };
    let selected_profile = decision
        .candidate
        .or_else(|| final_section.as_ref().map(|candidate| candidate.profile));
    let target_fifths = selected_profile.map(|profile| profile.fifths);
    let selected_measure = if scorewide_attempted {
        decision.opening_measure
    } else {
        final_section.as_ref().map(|candidate| candidate.boundary_measure)
    };
    let selected_profile_label = selected_profile
        .map_or_else(|| "none".to_string(), |profile| profile.name.to_string());
    let structural_scope_bound = scorewide_scope_bound || final_section.is_some();
    let attempted = scorewide_attempted || final_section.is_some();

    let key_candidate_xml = selected_measure
        .zip(target_fifths)
        .filter(|_| attempted)
        .map_or_else(|| xml.to_string(), |(measure, fifths)| rewrite_key_signature_for_measure_plain(xml, measure, fifths));
    let (candidate_xml, respelling_stats) = if let Some(profile) = selected_profile.filter(|_| attempted) {
        if let Some(section) = final_section.as_ref() {
            functional_enharmonic_candidate_for_tonal_frame_from_measure(
                &key_candidate_xml,
                profile.fifths,
                profile.tonic,
                profile.minor,
                Some(section.boundary_measure),
                None,
            )
        } else {
            scorewide_functional_enharmonic_candidate(&key_candidate_xml, profile)
        }
    } else {
        (key_candidate_xml, FunctionalRespellingStats::default())
    };

    let (after_events, backup_forward_after_ok) = collect_render_xml_note_events(&candidate_xml);
    let after_atomic = parse_atomic_notes(&candidate_xml);
    let after_sounding = render_visible_sounding_signature(&after_events);
    let counts_after = enharmonic_spelling_counts(&after_atomic);
    let (first_pitch_after, last_pitch_after) = first_last_visible_pitch_labels(&after_atomic);
    let visible_pitched_after = after_events
        .iter()
        .filter(|event| !event.is_hidden && event.midi.is_some())
        .count();
    let visible_rests_after = after_events
        .iter()
        .filter(|event| !event.is_hidden && event.is_rest)
        .count();
    let parse_before_ok = parse_musicxml(xml).is_ok();
    let parse_after_ok = parse_musicxml(&candidate_xml).is_ok();
    let sounding_stable = before_sounding == after_sounding;
    let key_written = selected_measure
        .zip(target_fifths)
        .is_some_and(|(measure, fifths)| key_signature_measure_fifths(&candidate_xml, measure) == Some(fifths));
    let invariant_green = attempted
        && parse_before_ok
        && parse_after_ok
        && backup_forward_before_ok
        && backup_forward_after_ok
        && sounding_stable
        && before_atomic.len() == after_atomic.len()
        && visible_pitched_before == visible_pitched_after
        && visible_rests_before == visible_rests_after
        && key_written;
    let written = invariant_green;
    let render_xml_write = written;
    let final_section_mode = final_section.is_some();
    let verdict = if written && final_section_mode {
        "final_section_tonal_key_signature_functional_respelling_render_write_green"
    } else if written {
        "scorewide_tonal_key_signature_functional_respelling_render_write_green"
    } else if attempted {
        "tonal_key_signature_candidate_failed_invariants"
    } else {
        decision.reason
    }
    .to_string();

    let audit = KeySignatureEnharmonicSpellingRenderWriteAudit {
        source_label: source_label.to_string(),
        output_path: output_path.display().to_string(),
        evidence_intake_open: true,
        structural_scope_bound,
        attempted,
        written,
        selected_profile: selected_profile_label,
        global_profile: decision
            .global_profile
            .map_or_else(|| "unbound".to_string(), |profile| profile.name.to_string()),
        source_fifths: decision.source_fifths,
        target_fifths,
        final_bass_pc: decision.final_bass_pc,
        accidental_cost_open: decision.open_cost,
        accidental_cost_target: decision.target_cost,
        accidental_cost_best: decision.best_cost,
        accidental_saving_percent: decision.saving_percent,
        selection_reason: if final_section_mode {
            "final_section_full_evidence_tail_green".to_string()
        } else {
            decision.reason.to_string()
        },
        e_flat_before: counts_before.e_flat,
        e_flat_after: counts_after.e_flat,
        b_flat_before: counts_before.b_flat,
        b_flat_after: counts_after.b_flat,
        d_sharp_before: counts_before.d_sharp,
        d_sharp_after: counts_after.d_sharp,
        a_sharp_before: counts_before.a_sharp,
        a_sharp_after: counts_after.a_sharp,
        f_natural_before: counts_before.f_natural,
        f_natural_after: counts_after.f_natural,
        e_sharp_before: counts_before.e_sharp,
        e_sharp_after: counts_after.e_sharp,
        respelling_candidates: respelling_stats.total,
        diatonic_respellings: respelling_stats.diatonic,
        minor_leading_respellings: respelling_stats.minor_leading,
        first_visible_pitch_before: first_pitch.clone(),
        first_visible_pitch_after: first_pitch_after,
        last_visible_pitch_before: last_pitch,
        last_visible_pitch_after: last_pitch_after,
        parse_before_ok,
        parse_after_ok,
        backup_forward_before_ok,
        backup_forward_after_ok,
        sounding_stable,
        note_count_before: before_atomic.len(),
        note_count_after: after_atomic.len(),
        visible_pitched_before,
        visible_pitched_after,
        visible_rests_before,
        visible_rests_after,
        final_section_mode,
        final_section_boundary: final_section.as_ref().map(|section| section.boundary_measure),
        final_section_end: final_section.as_ref().map(|section| section.end_measure),
        final_section_tail_notes: final_section.as_ref().map_or(0, |section| section.tail_notes),
        final_section_chromatic_notes: final_section.as_ref().map_or(0, |section| section.chromatic_notes),
        final_section_accidental_before: final_section.as_ref().map_or(0, |section| section.accidental_before),
        final_section_accidental_after: final_section.as_ref().map_or(0, |section| section.accidental_after),
        final_section_saving_percent: final_section.as_ref().map_or(0, |section| section.saving_percent),
        hardcoded_measure_logic: false,
        render_xml_write,
        music_mutation: written,
        verdict,
    };

    if written {
        (candidate_xml, audit)
    } else {
        (xml.to_string(), audit)
    }
};
    
    let receipt = Some(output.0.as_str()).filter(|post| *post != xml).map(|post| crate::master_state_identity::BlockTransformationReceipt::mint_block_receipt(RenderRow66BlockMint { _private: () }, xml, post, Some((0, xml.len()))).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

fn append_key_signature_enharmonic_spelling_render_write_section(
    out: &mut String,
    audit: &KeySignatureEnharmonicSpellingRenderWriteAudit,
) {
    out.push_static_section_header_v795(
        "\nTONAL-KEY-SIGNATURE-FUNCTIONAL-RESPELLING-CANONICAL-MASTER-MIRROR",
        "------------------------------------------------------------------",
    );
    out.push_format_args(format_args!(
        "Status: v{} literaturgestuetzter, identity-freier Anfangs- oder Finalabschnitts-Vorzeichnungs- und Funktions-Respelling-Write mit kanonischer Spiegelung durch den bestehenden Master-Sink. Ein finaler Wechsel wird nur geschrieben, wenn der vorhandene sechsfaktorielle Segmentierungsbeweis genau eine vollstaendige Grenze liefert, das Tail-Gesamtprofil dieselbe Tonart traegt, der Schlussbass ihre Tonika bestaetigt und die funktionale Schreibweise die Vorzeichenlast tatsaechlich senkt. Tonikalisierungen, Rueckkehr, konkurrierende Ziele und Mehrdeutigkeit bleiben fail-closed.\n",
        env!("CARGO_PKG_VERSION")
    ));
    out.push_format_args(format_args!(
        "Kurz: FinalSectionTonalKeySignatureFunctionalRespellingCanonicalMasterMirror: EvidenceIntakeOpen {}; StructuralScopeBound {}; Attempted {}; Written {}; GlobalProfile {}; SelectedProfile {}; SourceFifths {}; TargetFifths {}; FinalBassPc {}; AccidentalCostOpenTargetBest {}/{}/{}; SavingPercent {}; SelectionReason {}; HardcodedMeasureLogic {}; FunctionalRespelling {}/{}/{}; RenderXmlWrite {}; MusicMutation {}; MasterApply {}.\n",
        yes_no(audit.evidence_intake_open),
        yes_no(audit.structural_scope_bound),
        yes_no(audit.attempted),
        yes_no(audit.written),
        audit.global_profile,
        audit.selected_profile,
        optional_i32_label(audit.source_fifths),
        optional_i32_label(audit.target_fifths),
        optional_i32_label(audit.final_bass_pc),
        audit.accidental_cost_open,
        audit.accidental_cost_target,
        audit.accidental_cost_best,
        audit.accidental_saving_percent,
        audit.selection_reason,
        yes_no(audit.hardcoded_measure_logic),
        audit.respelling_candidates,
        audit.diatonic_respellings,
        audit.minor_leading_respellings,
        yes_no(audit.render_xml_write),
        yes_no(audit.music_mutation),
        yes_no(audit.written),
    ));
    crate::push_static_named_policy_line_v765(
        out,
        "SafetyBoundary",
        " Keine Projekt-, Etueden- oder absolute Taktidentitaet. Der scoreweite Pfad bindet weiterhin nur den ersten realen Measure-Knoten. Der Finalabschnittspfad bindet ausschliesslich die einzige bereits sechsfaktoriell gruene relative Grenzposition; danach muessen alle nicht gehaltenen Gegenkandidaten dieselbe Tonart tragen, das Gesamtprofil des verbleibenden Abschnitts und der Schlussbass muessen sie bestaetigen, und die funktionale Schreibweise muss die Vorzeichenlast senken. Null, Rueckkehr, konkurrierendes Ziel oder Mehrdeutigkeit bedeutet fail-closed.",
    );
    out.push_format_args(format_args!(
        "InvariantEvidence: Source {}; Output {}; Eflat {}->{}; Bflat {}->{}; Dsharp {}->{}; Asharp {}->{}; Fnatural {}->{}; Esharp {}->{}; RespellingTotal {} (diatonisch {}, Moll-Leitton {}); FirstPitch {}->{}; LastPitch {}->{}; ParseOk {}/{}; BackupForward {}/{}; NoteCount {}/{}; VisiblePitched {}/{}; VisibleRests {}/{}; SoundingStable {}.\n",
        audit.source_label,
        audit.output_path,
        audit.e_flat_before,
        audit.e_flat_after,
        audit.b_flat_before,
        audit.b_flat_after,
        audit.d_sharp_before,
        audit.d_sharp_after,
        audit.a_sharp_before,
        audit.a_sharp_after,
        audit.f_natural_before,
        audit.f_natural_after,
        audit.e_sharp_before,
        audit.e_sharp_after,
        audit.respelling_candidates,
        audit.diatonic_respellings,
        audit.minor_leading_respellings,
        audit.first_visible_pitch_before,
        audit.first_visible_pitch_after,
        audit.last_visible_pitch_before,
        audit.last_visible_pitch_after,
        yes_no(audit.parse_before_ok),
        yes_no(audit.parse_after_ok),
        yes_no(audit.backup_forward_before_ok),
        yes_no(audit.backup_forward_after_ok),
        audit.note_count_before,
        audit.note_count_after,
        audit.visible_pitched_before,
        audit.visible_pitched_after,
        audit.visible_rests_before,
        audit.visible_rests_after,
        yes_no(audit.sounding_stable),
    ));
    out.push_format_args(format_args!(
        "FinalSectionEvidence: Mode {}; Boundary {}; TailEnd {}; TailNotes {}; ChromaticNotes {}; AccidentalCost {}->{}; SavingPercent {}; HardcodedMeasureLogic nein.\n",
        yes_no(audit.final_section_mode),
        audit.final_section_boundary.map_or_else(|| "none".to_string(), |value| value.to_string()),
        audit.final_section_end.map_or_else(|| "none".to_string(), |value| value.to_string()),
        audit.final_section_tail_notes,
        audit.final_section_chromatic_notes,
        audit.final_section_accidental_before,
        audit.final_section_accidental_after,
        audit.final_section_saving_percent,
    ));
    out.push_format_args(format_args!("Urteil: {}.\n", audit.verdict));
}

fn optional_i32_label(value: Option<i32>) -> String {
    value
        .map_or_else(|| "none".to_string(), |number| number.to_string())
}

#[derive(Debug, Clone, Copy, Default)]
struct EnharmonicSpellingCounts {
    e_flat: usize,
    b_flat: usize,
    d_sharp: usize,
    a_sharp: usize,
    f_natural: usize,
    e_sharp: usize,
}

fn enharmonic_spelling_counts(notes: &[AtomicNote]) -> EnharmonicSpellingCounts {
    let mut counts = EnharmonicSpellingCounts::default();
    for note in notes.iter().filter(|note| note.pitch.is_some()) {
        let pitch = note.pitch.as_ref().expect("filtered pitch");
        match (pitch.step.as_str(), pitch.alter) {
            ("E", -1) => counts.e_flat += 1,
            ("B", -1) => counts.b_flat += 1,
            ("D", 1) => counts.d_sharp += 1,
            ("A", 1) => counts.a_sharp += 1,
            ("F", 0) => counts.f_natural += 1,
            ("E", 1) => counts.e_sharp += 1,
            _ => {}
        }
    }
    counts
}



#[derive(Debug, Clone, Copy, Default)]
struct FunctionalRespellingStats {
    total: usize,
    diatonic: usize,
    minor_leading: usize,
}

#[derive(Debug, Clone)]
struct FunctionalRespellingPlan {
    note_start: usize,
    note_end: usize,
    step: &'static str,
    alter: i32,
    octave: i32,
    kind: FunctionalRespellingKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionalRespellingKind {
    Diatonic,
    MinorLeading,
}

fn scorewide_functional_enharmonic_candidate(
    xml: &str,
    profile: TonalEvidenceProfile,
) -> (String, FunctionalRespellingStats) {
    functional_enharmonic_candidate_for_tonal_frame(
        xml,
        profile.fifths,
        profile.tonic,
        profile.minor,
    )
}

fn functional_enharmonic_candidate_for_tonal_frame(
    xml: &str,
    fifths: i32,
    tonic: i32,
    minor: bool,
) -> (String, FunctionalRespellingStats) {
    functional_enharmonic_candidate_for_tonal_frame_from_measure(
        xml, fifths, tonic, minor, None,
        None,
    )
}

fn functional_respelling_plans_for_tonal_frame_from_measure(
    xml: &str,
    fifths: i32,
    tonic: i32,
    minor: bool,
    start_measure: Option<usize>,
) -> Vec<FunctionalRespellingPlan> {
    let notes = parse_atomic_notes(xml);
    let mut plans = Vec::<FunctionalRespellingPlan>::new();

    for note in notes
        .iter()
        .filter(|note| note.pitch.is_some())
        .filter(|note| {
            start_measure
                .is_none_or(|start| functional_measure_ordinal(note) >= start)
        })
    {
        let pitch = note.pitch.as_ref().expect("filtered pitch");
        let pitch_class = pitch.midi.rem_euclid(12);
        let Some((step, alter)) = functional_key_diatonic_spelling(fifths, pitch_class)
        else {
            continue;
        };
        if pitch.step == step && pitch.alter == alter {
            continue;
        }
        let Some(octave) = functional_respelling_octave(pitch.midi, step, alter, pitch.octave)
        else {
            continue;
        };
        plans.push(FunctionalRespellingPlan {
            note_start: note.note_start,
            note_end: note.note_end,
            step,
            alter,
            octave,
            kind: FunctionalRespellingKind::Diatonic,
        });
    }

    if minor {
        let leading_pitch_class = (tonic - 1).rem_euclid(12);
        let natural_seventh_pitch_class = (tonic - 2).rem_euclid(12);
        if let Some((leading_step, natural_seventh_alter)) =
            functional_key_diatonic_spelling(fifths, natural_seventh_pitch_class)
        {
            let leading_alter = natural_seventh_alter + 1;
            let dominant_root = (tonic + 7).rem_euclid(12);
            let dominant_fifth = (tonic + 2).rem_euclid(12);
            let mut lanes = std::collections::BTreeMap::<(String, String), Vec<&AtomicNote>>::new();
            for note in notes
                .iter()
                .filter(|note| note.pitch.is_some())
                .filter(|note| {
                    start_measure
                        .is_none_or(|start| functional_measure_ordinal(note) >= start)
                })
            {
                lanes
                    .entry((note.staff.clone(), note.voice.clone()))
                    .or_default()
                    .push(note);
            }
            for lane in lanes.values_mut() {
                lane.sort_by(|left, right| {
                    functional_measure_ordinal(left)
                        .cmp(&functional_measure_ordinal(right))
                        .then_with(|| left.start.cmp(&right.start))
                        .then_with(|| left.note_start.cmp(&right.note_start))
                });
                for (index, note) in lane.iter().enumerate() {
                    let pitch = note.pitch.as_ref().expect("pitched lane note");
                    if pitch.midi.rem_euclid(12) != leading_pitch_class
                        || (pitch.step == leading_step && pitch.alter == leading_alter)
                    {
                        continue;
                    }
                    if !functional_minor_leading_context_green(
                        lane,
                        index,
                        tonic,
                        dominant_root,
                        leading_pitch_class,
                        dominant_fifth,
                    ) {
                        continue;
                    }
                    let Some(octave) = functional_respelling_octave(
                        pitch.midi,
                        leading_step,
                        leading_alter,
                        pitch.octave,
                    ) else {
                        continue;
                    };
                    plans.push(FunctionalRespellingPlan {
                        note_start: note.note_start,
                        note_end: note.note_end,
                        step: leading_step,
                        alter: leading_alter,
                        octave,
                        kind: FunctionalRespellingKind::MinorLeading,
                    });
                }
            }
        }
    }

    plans.sort_by(|left, right| {
        right
            .note_start
            .cmp(&left.note_start)
            .then_with(|| right.note_end.cmp(&left.note_end))
    });
    plans.dedup_by_key(|plan| plan.note_start);
    plans
}

/// The bound edits of the functional respelling planner in application order
/// (start-descending), each `(note_start, note_end, rewritten note block)` in
/// the coordinates of `xml`. This is the planner's own geometry, not a diff.
fn functional_respelling_edits_for_tonal_frame_from_measure(
    xml: &str,
    fifths: i32,
    tonic: i32,
    minor: bool,
    start_measure: Option<usize>,
    bound_plans: Option<&[FunctionalRespellingPlan]>,
) -> Vec<(usize, usize, String)> {
    bound_plans.map(<[FunctionalRespellingPlan]>::to_vec).unwrap_or_else(|| functional_respelling_plans_for_tonal_frame_from_measure(
        xml,
        fifths,
        tonic,
        minor,
        start_measure,
    ))
    .into_iter()
    .filter_map(|plan| {
        let block = xml.get(plan.note_start..plan.note_end)?;
        let rewritten = functional_note_pitch_candidate(block, plan.step, plan.alter, plan.octave);
        (rewritten != block).then_some((plan.note_start, plan.note_end, rewritten))
    })
    .collect()
}

fn functional_enharmonic_candidate_for_tonal_frame_from_measure(
    xml: &str,
    fifths: i32,
    tonic: i32,
    minor: bool,
    start_measure: Option<usize>,
    bound_plans: Option<Vec<FunctionalRespellingPlan>>,
) -> (String, FunctionalRespellingStats) {
    let plans = bound_plans.unwrap_or_else(|| functional_respelling_plans_for_tonal_frame_from_measure(
        xml,
        fifths,
        tonic,
        minor,
        start_measure,
    ));

    let mut output = xml.to_string();
    let mut stats = FunctionalRespellingStats::default();
    for plan in plans {
        let Some(block) = output.get(plan.note_start..plan.note_end) else {
            continue;
        };
        let rewritten = functional_note_pitch_candidate(
            block,
            plan.step,
            plan.alter,
            plan.octave,
        );
        if rewritten == block {
            continue;
        }
        let mut next_output = String::with_capacity(
            output.len() + rewritten.len().saturating_sub(plan.note_end - plan.note_start),
        );
        next_output.push_str(&output[..plan.note_start]);
        next_output.push_str(&rewritten);
        next_output.push_str(&output[plan.note_end..]);
        output = next_output;
        stats.total = stats.total.saturating_add(1);
        match plan.kind {
            FunctionalRespellingKind::Diatonic => {
                stats.diatonic = stats.diatonic.saturating_add(1)
            }
            FunctionalRespellingKind::MinorLeading => {
                stats.minor_leading = stats.minor_leading.saturating_add(1)
            }
        }
    }

    (output, stats)
}

fn functional_minor_leading_context_green(
    lane: &[&AtomicNote],
    index: usize,
    tonic: i32,
    dominant_root: i32,
    leading_pitch_class: i32,
    dominant_fifth: i32,
) -> bool {
    let note = lane[index];
    let measure = functional_measure_ordinal(note);
    let adjacent_tonic = index
        .checked_sub(1)
        .and_then(|previous| lane.get(previous).copied())
        .into_iter()
        .chain(lane.get(index + 1).copied())
        .any(|candidate| {
            functional_measure_ordinal(candidate).abs_diff(measure) <= 1
                && candidate
                    .pitch
                    .as_ref()
                    .is_some_and(|pitch| pitch.midi.rem_euclid(12) == tonic)
        });
    if adjacent_tonic {
        return true;
    }

    let lower = index.saturating_sub(9);
    let upper = (index + 10).min(lane.len());
    let mut dominant_present = false;
    let mut leading_present = false;
    let mut fifth_present = false;
    for candidate in &lane[lower..upper] {
        if functional_measure_ordinal(candidate).abs_diff(measure) > 1 {
            continue;
        }
        let Some(pitch) = candidate.pitch.as_ref() else {
            continue;
        };
        let pitch_class = pitch.midi.rem_euclid(12);
        dominant_present |= pitch_class == dominant_root;
        leading_present |= pitch_class == leading_pitch_class;
        fifth_present |= pitch_class == dominant_fifth;
    }
    dominant_present && leading_present && fifth_present
}

fn functional_measure_ordinal(note: &AtomicNote) -> usize {
    note.measure.parse::<usize>().unwrap_or(usize::MAX)
}

fn functional_key_diatonic_spelling(
    fifths: i32,
    pitch_class: i32,
) -> Option<(&'static str, i32)> {
    const STEPS: [&str; 7] = ["C", "D", "E", "F", "G", "A", "B"];
    STEPS.iter().copied().find_map(|step| {
        let alter = scorewide_signature_default_alter(fifths, step);
        ((functional_step_pitch_class(step) + alter).rem_euclid(12)
            == pitch_class.rem_euclid(12))
        .then_some((step, alter))
    })
}

fn functional_respelling_octave(
    midi: i32,
    step: &str,
    alter: i32,
    current_octave: i32,
) -> Option<i32> {
    (current_octave - 1..=current_octave + 1).find(|octave| {
        (octave + 1) * 12 + functional_step_pitch_class(step) + alter == midi
    })
}

fn functional_step_pitch_class(step: &str) -> i32 {
    match step {
        "C" => 0,
        "D" => 2,
        "E" => 4,
        "F" => 5,
        "G" => 7,
        "A" => 9,
        "B" => 11,
        _ => -100,
    }
}

fn functional_note_pitch_candidate(
    note_block: &str,
    step: &str,
    alter: i32,
    octave: i32,
) -> String {
    let Some((pitch_start, pitch_end)) = functional_child_block_range(note_block, "pitch")
    else {
        return note_block.to_string();
    };
    let pitch = if alter == 0 {
        format!(
            "<pitch><step>{step}</step><octave>{octave}</octave></pitch>"
        )
    } else {
        format!(
            "<pitch><step>{step}</step><alter>{alter}</alter><octave>{octave}</octave></pitch>"
        )
    };
    let mut rewritten = String::with_capacity(
        note_block.len() + pitch.len().saturating_sub(pitch_end - pitch_start),
    );
    rewritten.push_str(&note_block[..pitch_start]);
    rewritten.push_str(&pitch);
    rewritten.push_str(&note_block[pitch_end..]);
    functional_without_all_child_blocks(&rewritten, "accidental")
}

fn functional_without_all_child_blocks(block: &str, tag: &str) -> String {
    let mut output = block.to_string();
    while let Some((start, end)) = functional_child_block_range(&output, tag) {
        let mut next_output = String::with_capacity(output.len().saturating_sub(end - start));
        next_output.push_str(&output[..start]);
        next_output.push_str(&output[end..]);
        output = next_output;
    }
    output
}

fn functional_child_block_range(block: &str, tag: &str) -> Option<(usize, usize)> {
    let open = format!("<{tag}");
    let start = block.find(&open)?;
    let open_end = start + block[start..].find('>')?;
    if block.as_bytes().get(open_end.saturating_sub(1)) == Some(&b'/') {
        return Some((start, open_end + 1));
    }
    let close = format!("</{tag}>");
    let end = open_end + 1 + block[open_end + 1..].find(&close)? + close.len();
    Some((start, end))
}


fn functional_opening_fifths(xml: &str) -> Option<i32> {
    let document = roxmltree::Document::parse(xml).ok()?;
    document
        .descendants()
        .find(|node| node.has_tag_name("fifths"))?
        .text()?
        .trim()
        .parse::<i32>()
        .ok()
}

/// Sealed tonal-frame fact of exactly one Render snapshot: the only Render
/// information #59 and #60 may consume. It is derived once from the bound
/// snapshot bytes and re-derived fail-closed wherever those bytes cross a
/// frozen `&str` boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TonalFrameFact {
    snapshot_fingerprint: u64,
    bound_snapshot: String,
    fifths_vector: Option<Vec<i32>>,
    opening_fifths: Option<i32>,
    explicit_key_vector: Vec<(usize, i32)>,
    scorewide_final_bass: Option<i32>,
    boundary_tail_bass: Option<(usize, Option<i32>)>,
}

impl TonalFrameFact {
    /// Derives the fact from the bound Render snapshot; `None` if the snapshot
    /// is not parseable MusicXML.
    pub(crate) fn derive_from_render_snapshot(render_xml: &str) -> Option<Self> {
        let document = roxmltree::Document::parse(render_xml).ok()?;
        let fifths_vector = document
            .descendants()
            .filter(|node| node.has_tag_name("fifths"))
            .map(|node| node.text()?.trim().parse::<i32>().ok())
            .collect::<Option<Vec<_>>>();
        let explicit_key_vector = functional_explicit_key_signatures(render_xml);
        let render_notes = parse_atomic_notes(render_xml);
        let render_visible = render_notes
            .iter()
            .filter(|note| note.visible && note.pitch.is_some())
            .collect::<Vec<_>>();
        let scorewide_final_bass = scorewide_final_bass_pitch_class(&render_visible);
        let boundary_tail_bass = explicit_key_vector.get(1).map(|(boundary_measure, _)| {
            let render_tail = render_visible
                .iter()
                .copied()
                .filter(|note| functional_measure_ordinal(note) >= *boundary_measure)
                .collect::<Vec<_>>();
            (*boundary_measure, scorewide_final_bass_pitch_class(&render_tail))
        });
        Some(Self {
            snapshot_fingerprint: tonal_frame_snapshot_fingerprint(render_xml),
            bound_snapshot: render_xml.to_string(),
            fifths_vector,
            opening_fifths: functional_opening_fifths(render_xml),
            explicit_key_vector,
            scorewide_final_bass,
            boundary_tail_bass,
        })
    }

    pub(crate) fn bound_snapshot(&self) -> &str {
        &self.bound_snapshot
    }

    pub(crate) fn fifths_vector(&self) -> Option<&[i32]> {
        self.fifths_vector.as_deref()
    }

    fn binds_snapshot(&self, render_xml: &str) -> bool {
        self.snapshot_fingerprint == tonal_frame_snapshot_fingerprint(render_xml)
            && self.bound_snapshot == render_xml
    }
}

fn tonal_frame_snapshot_fingerprint(render_xml: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in render_xml.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// Sealed EXACT mint capability of population owner #60.
#[derive(Debug, Clone, Copy)]
pub(crate) struct FunctionalRespellingExactMint {
    _sealed: (),
}

/// Complete original components receive one already chosen tonal spelling.
/// The carrier supplies membership only, never a new tonal decision.
fn original_chain_functional_respelling_plans(
    before: &str, keyed: &str, key_edits: &[crate::master_state_identity::ExactEdit],
    original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains,
    mut plans: Vec<FunctionalRespellingPlan>, start_measure: Option<usize>,
) -> Result<Vec<FunctionalRespellingPlan>, String> {
    if plans.is_empty() { return Ok(plans); }
    let requested = plans.iter().map(|plan| (plan.note_start, plan.note_end)).collect::<Vec<_>>();
    let components = original_chains.components_through_original_key_operations(before, keyed, key_edits, &requested)
        .map_err(|error| format!("functional master mirror original chains: {error}"))?;
    let notes = parse_atomic_notes(keyed);
    let by_range = notes.iter().map(|note| ((note.note_start, note.note_end), note))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut planned = plans.iter().map(|plan|
        ((plan.note_start, plan.note_end), plan.clone())).collect::<std::collections::BTreeMap<_, _>>();
    for component in components {
        let chosen = component.iter().filter_map(|range| planned.get(&(range.start, range.end)))
            .cloned().collect::<Vec<_>>();
        let Some(spelling) = chosen.first() else { continue; };
        if chosen.iter().any(|plan| (plan.step, plan.alter, plan.octave)
            != (spelling.step, spelling.alter, spelling.octave)) {
            return Err("functional master mirror conflicting original chain spellings".to_string());
        }
        for range in component {
            let note = by_range.get(&(range.start, range.end))
                .ok_or_else(|| "functional master mirror original chain is not a complete note".to_string())?;
            if start_measure.is_some_and(|start| functional_measure_ordinal(note) < start) {
                return Err("functional master mirror original chain exceeds the authorized tonal scope".to_string());
            }
            let pitch = note.pitch.as_ref().ok_or_else(|| "functional master mirror original chain is not pitched".to_string())?;
            if functional_respelling_octave(pitch.midi, spelling.step, spelling.alter, pitch.octave)
                != Some(spelling.octave) {
                return Err("functional master mirror original chain changed sounding pitch".to_string());
            }
            let mut plan = spelling.clone(); plan.note_start = range.start; plan.note_end = range.end;
            planned.remove(&(range.start, range.end));
            plans.retain(|plan| plan.note_start != range.start);
            plans.push(plan);
        }
    }
    if !planned.is_empty() {
        return Err("functional master mirror planned occurrence lacks original chain membership".to_string());
    }
    let completed = plans.iter().map(|plan| (plan.note_start, plan))
        .collect::<std::collections::BTreeMap<_, _>>();
    let mut chord = Vec::<&AtomicNote>::new();
    let verify_chord = |chord: &[&AtomicNote]| -> Result<(), String> {
        if !chord.iter().any(|note| completed.contains_key(&note.note_start)) { return Ok(()); }
        let mut pitches = std::collections::BTreeMap::new();
        for note in chord {
            let Some(pitch) = &note.pitch else { continue; };
            let spelling = completed.get(&note.note_start).map_or(
                (pitch.step.as_str(), pitch.alter, pitch.octave), |plan| (plan.step, plan.alter, plan.octave));
            if pitches.insert(pitch.midi, spelling).is_some_and(|old| old != spelling) {
                return Err("functional master mirror conflicting same-key chord spellings".to_string());
            }
        }
        Ok(())
    };
    for note in &notes {
        if !note.is_chord || chord.last().is_some_and(|previous|
            previous.measure != note.measure || previous.run != note.run) {
            verify_chord(&chord)?; chord.clear();
        }
        chord.push(note);
    }
    verify_chord(&chord)?;
    plans.sort_by_key(|plan| std::cmp::Reverse(plan.note_start));
    Ok(plans)
}

fn canonical_master_functional_respelling_from_render(
    master_xml: &str,
    render_xml: &str,
    original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains,
) -> (Result<String, String>, Option<ExactTransformationReceipt>) {
    // The frozen `&str` must be the bound Render snapshot: the fact is derived
    // from exactly these bytes and nothing else of the Render is interpreted.
    let Some(tonal_frame) = TonalFrameFact::derive_from_render_snapshot(render_xml)
        .filter(|fact| fact.binds_snapshot(render_xml))
    else {
        return (Err("functional master mirror lost render fifths".to_string()), None);
    };
    let mut edit_script: Option<Vec<crate::master_state_identity::ExactEdit>> = None;
    let candidate = (|| -> Result<String, String> {
        let render_fifths = tonal_frame
            .opening_fifths
            .ok_or_else(|| "functional master mirror lost render fifths".to_string())?;
        let render_keys = tonal_frame.explicit_key_vector.clone();
        let master_keys = functional_explicit_key_signatures(master_xml);

        let candidate = if render_fifths != 0 {
            if functional_opening_fifths(master_xml) != Some(render_fifths) {
                return Err("functional master mirror is not bound to the render key".to_string());
            }
            let relative_minor_tonic = (7 * render_fifths + 9).rem_euclid(12);
            let minor = tonal_frame.scorewide_final_bass == Some(relative_minor_tonic);
            let major_tonic = (7 * render_fifths).rem_euclid(12);
            let tonic = if minor { relative_minor_tonic } else { major_tonic };
            let plans = original_chain_functional_respelling_plans(
                master_xml, master_xml, &[], original_chains,
                functional_respelling_plans_for_tonal_frame_from_measure(
                    master_xml, render_fifths, tonic, minor, None), None,
            )?;
            edit_script = Some(
                crate::master_state_identity::exact_script_from_descending_application(
                    functional_respelling_edits_for_tonal_frame_from_measure(
                        master_xml,
                        render_fifths,
                        tonic,
                        minor,
                        None,
                        Some(&plans),
                    ),
                ),
            );
            functional_enharmonic_candidate_for_tonal_frame_from_measure(
                master_xml,
                render_fifths,
                tonic,
                minor,
                None,
                Some(plans),
            )
            .0
        } else if render_keys.len() == 1 {
            return Ok(master_xml.to_string());
        } else {
            let [(opening_measure, 0), (boundary_measure, target_fifths)] = render_keys.as_slice() else {
                return Err("functional final-section master mirror requires exactly one local key change".to_string());
            };
            if *target_fifths == 0
                || !TONAL_EVIDENCE_PROFILES
                    .iter()
                    .any(|profile| profile.fifths == *target_fifths)
                || *boundary_measure <= *opening_measure
                || master_keys.len() != 1
                || master_keys[0] != (*opening_measure, 0)
            {
                return Err("functional final-section master mirror key vector is not tightly bound".to_string());
            }
            let final_bass = match tonal_frame.boundary_tail_bass {
                Some((bound_boundary, final_bass)) if bound_boundary == *boundary_measure => final_bass,
                _ => None,
            }
            .ok_or_else(|| "functional final-section master mirror lost final bass".to_string())?;
            let major_tonic = (7 * *target_fifths).rem_euclid(12);
            let relative_minor_tonic = (major_tonic + 9).rem_euclid(12);
            let (tonic, minor) = if final_bass == major_tonic {
                (major_tonic, false)
            } else if final_bass == relative_minor_tonic {
                (relative_minor_tonic, true)
            } else {
                return Err("functional final-section master mirror final bass rejects target key".to_string());
            };
            let keyed = rewrite_key_signature_for_measure_plain(
                master_xml,
                *boundary_measure,
                *target_fifths,
            );
            let key_edits = key_signature_for_measure_edits(
                master_xml,
                *boundary_measure,
                *target_fifths,
            );
            let original_key_edits = crate::master_state_identity::exact_script_from_descending_application(key_edits.clone());
            let plans = original_chain_functional_respelling_plans(
                master_xml, &keyed, &original_key_edits, original_chains,
                functional_respelling_plans_for_tonal_frame_from_measure(
                    &keyed, *target_fifths, tonic, minor, Some(*boundary_measure)),
                Some(*boundary_measure),
            )?;
            let note_edits = functional_respelling_edits_for_tonal_frame_from_measure(
                &keyed,
                *target_fifths,
                tonic,
                minor,
                Some(*boundary_measure),
                Some(&plans),
            );
            edit_script = crate::master_state_identity::compose_exact_edit_scripts(
                master_xml,
                crate::master_state_identity::exact_script_from_descending_application(key_edits),
                crate::master_state_identity::exact_script_from_descending_application(note_edits),
            );
            functional_enharmonic_candidate_for_tonal_frame_from_measure(
                &keyed,
                *target_fifths,
                tonic,
                minor,
                Some(*boundary_measure),
                Some(plans),
            )
            .0
        };

        parse_musicxml(&candidate)
            .map_err(|error| format!("functional master mirror parse failed: {error}"))?;
        if functional_explicit_key_signatures(&candidate) != render_keys {
            return Err("functional master mirror failed exact key-vector readback".to_string());
        }
        let before = parse_atomic_notes(master_xml);
        let after = parse_atomic_notes(&candidate);
        if before.len() != after.len()
            || before.iter().zip(&after).any(|(left, right)| {
                left.pitch.as_ref().map(|pitch| pitch.midi)
                    != right.pitch.as_ref().map(|pitch| pitch.midi)
                    || left.start != right.start
                    || left.duration != right.duration
                    || left.staff != right.staff
                    || left.voice != right.voice
            })
        {
            return Err("functional master mirror changed a musical invariant".to_string());
        }
        Ok(candidate)
    })();
    let legacy_candidate = match candidate {
        Ok(legacy_candidate) => legacy_candidate,
        Err(error) => return (Err(error), None),
    };
    if legacy_candidate == master_xml {
        return (Ok(legacy_candidate), None);
    }
    // Fail-closed equivalence: the receipt geometry bound at origin must
    // reproduce the legacy algorithm's output byte-exactly.
    let receipt = edit_script.and_then(|edits| {
        ExactTransformationReceipt::mint_exact_receipt(
            FunctionalRespellingExactMint { _sealed: () },
            master_xml,
            &legacy_candidate,
            edits,
        )
    });
    if receipt.is_none() {
        return (
            Err("functional master mirror EXACT geometry diverges from the legacy respelling algorithm".to_string()),
            None,
        );
    }
    (Ok(legacy_candidate), receipt)
}

fn first_last_visible_pitch_labels(notes: &[AtomicNote]) -> (String, String) {
    let labels = notes
        .iter()
        .filter(|note| note.visible && note.pitch.is_some())
        .map(|note| note.pitch.as_ref().expect("filtered pitch").label.clone())
        .collect::<Vec<_>>();
    let first = labels
        .first()
        .cloned()
        .unwrap_or_else(|| "unbound".to_string());
    let last = labels
        .last()
        .cloned()
        .unwrap_or_else(|| "unbound".to_string());
    (first, last)
}

fn key_signature_measure_fifths(xml: &str, measure_number: usize) -> Option<i32> {
    let mut scan = 0_usize;
    while let Some((start, end, number)) = find_next_measure_bounds(xml, scan) {
        if number == measure_number {
            let measure = &xml[start..end];
            return extract_first_text_from_fragment(measure, "fifths")
                .and_then(|value| value.trim().parse::<i32>().ok());
        }
        scan = end;
    }
    None
}

fn rewrite_measure_key_signature(measure: &str, target_fifths: i32) -> String {
    let new_key = format!("        <key><fifths>{target_fifths}</fifths></key>");
    if let Some(attr_start_rel) = measure.find("<attributes") {
        let attr_start = attr_start_rel;
        if let Some(attr_end_rel) = measure[attr_start..].find("</attributes>") {
            let attr_end = attr_start + attr_end_rel + "</attributes>".len();
            let attr = &measure[attr_start..attr_end];
            let new_attr = if let Some(key_start_rel) = attr.find("<key") {
                if let Some(key_end_rel) = attr[key_start_rel..].find("</key>") {
                    let key_end = key_start_rel + key_end_rel + "</key>".len();
                    let mut rebuilt = String::with_capacity(attr.len() + 32);
                    rebuilt.push_str(&attr[..key_start_rel]);
                    rebuilt.push_str(&new_key);
                    rebuilt.push_str(&attr[key_end..]);
                    rebuilt
                } else {
                    attr.to_string()
                }
            } else if let Some(close_rel) = attr.find("</attributes>") {
                let mut rebuilt = String::with_capacity(attr.len() + new_key.len() + 1);
                rebuilt.push_str(&attr[..close_rel]);
                rebuilt.push('\n');
                rebuilt.push_str(&new_key);
                rebuilt.push_str(&attr[close_rel..]);
                rebuilt
            } else {
                attr.to_string()
            };
            let mut rebuilt_measure = String::with_capacity(measure.len() + 64);
            rebuilt_measure.push_str(&measure[..attr_start]);
            rebuilt_measure.push_str(&new_attr);
            rebuilt_measure.push_str(&measure[attr_end..]);
            return rebuilt_measure;
        }
    }

    let Some(open_end_rel) = measure.find('>') else {
        return measure.to_string();
    };
    let insert_at = open_end_rel + 1;
    let mut rebuilt = String::with_capacity(measure.len() + 96);
    rebuilt.push_str(&measure[..insert_at]);
    rebuilt.push_str("\n      <attributes>\n");
    rebuilt.push_str(&new_key);
    rebuilt.push_str("\n      </attributes>");
    rebuilt.push_str(&measure[insert_at..]);
    rebuilt
}

// ---- merged from src/pipeline_shards/key_signature_enharmonic_audit_tail.rs ----
// v0.10.702 KeySignatureEnharmonicAuditTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the key-signature/enharmonic audit-only tail.
// Former part_44..part_45 are included below in the same order.

// ---- merged from src/pipeline_shards/part_44.rs ----
// v0.10.602 KeySignatureEnharmonicMapAuditNoMutation.
// Whole-piece key/enharmonic map from the current source score.
// Audit-only: no key signature, pitch spelling, note, staff, voice, onset or
// duration may be changed here.

// r725: obsolete KeySignatureEnharmonicMap audit/profile island retired after r724
// Clippy proved the former observation consumer removal left this island unreachable.
// The active carried-key helper below remains because TonalSegmentation and current tonal evidence consume it.

fn carried_key_signature_fifths(xml: &str, measure: usize) -> Option<i32> {
    let mut last = None;
    for current in 1..=measure {
        if let Some(fifths) = key_signature_measure_fifths(xml, current) {
            last = Some(fifths);
        }
    }
    last
}

// ---- merged from src/pipeline_shards/part_45.rs ----
// r725 RetiredObservationProjectionHarvestClippyClosure:
// The former late KeySignatureEnharmonicSpan writer and its observation-only
// report projection are fully retired. The active score-wide tonal decision,
// spelling authority and TonalSegmentation evidence below remain unchanged.

// v0.10.1477 shared tonal evidence profiles.
// The score-wide opening-signature writer above continues to consume only its
// restricted global-profile/final-bass/notation-cost proof. The segmentation
// analysis remains an audit producer and owns no writer. Its unique six-channel
// full-evidence boundary may now be consumed only by the existing final-section
// writer after a non-returning tail, final-bass and notation-saving readback.
// It still grants no independent respelling or barline authority.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TonalEvidenceProfile {
    name: &'static str,
    fifths: i32,
    tonic: i32,
    minor: bool,
}

/// Read-only projection of the existing score-wide tonal evidence owner.
///
/// This is intentionally not a key-signature decision and grants no writer capability. It only
/// exposes the already computed best/second profile evidence in an identity-free form so audit
/// consumers do not need to duplicate tonal-profile truth.
pub(crate) type ScorewideTonalEvidenceProjection = (&'static str, i32, bool, i64, i64);

pub(crate) fn scorewide_tonal_evidence_projection_from_xml(
    xml: &str,
) -> Option<ScorewideTonalEvidenceProjection> {
    let notes = parse_atomic_notes(xml);
    let visible = notes
        .iter()
        .filter(|note| note.visible && note.pitch.is_some())
        .collect::<Vec<_>>();
    let (profile, best_score, second_score) = tonal_evidence_best_profile_audit(&visible)?;
    Some((
        profile.name,
        profile.tonic,
        profile.minor,
        best_score,
        second_score,
    ))
}

const TONAL_EVIDENCE_PROFILES: [TonalEvidenceProfile; 24] = [
    TonalEvidenceProfile { name: "C-Dur/C-major", fifths: 0, tonic: 0, minor: false },
    TonalEvidenceProfile { name: "G-Dur/G-major", fifths: 1, tonic: 7, minor: false },
    TonalEvidenceProfile { name: "D-Dur/D-major", fifths: 2, tonic: 2, minor: false },
    TonalEvidenceProfile { name: "A-Dur/A-major", fifths: 3, tonic: 9, minor: false },
    TonalEvidenceProfile { name: "E-Dur/E-major", fifths: 4, tonic: 4, minor: false },
    TonalEvidenceProfile { name: "H-Dur/B-major", fifths: 5, tonic: 11, minor: false },
    TonalEvidenceProfile { name: "Fis-Dur/F-sharp-major", fifths: 6, tonic: 6, minor: false },
    TonalEvidenceProfile { name: "Des-Dur/D-flat-major", fifths: -5, tonic: 1, minor: false },
    TonalEvidenceProfile { name: "As-Dur/A-flat-major", fifths: -4, tonic: 8, minor: false },
    TonalEvidenceProfile { name: "Es-Dur/E-flat-major", fifths: -3, tonic: 3, minor: false },
    TonalEvidenceProfile { name: "B-Dur/B-flat-major", fifths: -2, tonic: 10, minor: false },
    TonalEvidenceProfile { name: "F-Dur/F-major", fifths: -1, tonic: 5, minor: false },
    TonalEvidenceProfile { name: "a-Moll/A-minor", fifths: 0, tonic: 9, minor: true },
    TonalEvidenceProfile { name: "e-Moll/E-minor", fifths: 1, tonic: 4, minor: true },
    TonalEvidenceProfile { name: "h-Moll/B-minor", fifths: 2, tonic: 11, minor: true },
    TonalEvidenceProfile { name: "fis-Moll/F-sharp-minor", fifths: 3, tonic: 6, minor: true },
    TonalEvidenceProfile { name: "cis-Moll/C-sharp-minor", fifths: 4, tonic: 1, minor: true },
    TonalEvidenceProfile { name: "gis-Moll/G-sharp-minor", fifths: 5, tonic: 8, minor: true },
    TonalEvidenceProfile { name: "dis-Moll/D-sharp-minor", fifths: 6, tonic: 3, minor: true },
    TonalEvidenceProfile { name: "b-Moll/B-flat-minor", fifths: -5, tonic: 10, minor: true },
    TonalEvidenceProfile { name: "f-Moll/F-minor", fifths: -4, tonic: 5, minor: true },
    TonalEvidenceProfile { name: "c-Moll/C-minor", fifths: -3, tonic: 0, minor: true },
    TonalEvidenceProfile { name: "g-Moll/G-minor", fifths: -2, tonic: 7, minor: true },
    TonalEvidenceProfile { name: "d-Moll/D-minor", fifths: -1, tonic: 2, minor: true },
];


#[derive(Debug, Clone)]
struct ScorewideTonalKeySignatureDecision {
    candidate: Option<TonalEvidenceProfile>,
    global_profile: Option<TonalEvidenceProfile>,
    opening_measure: Option<usize>,
    source_fifths: Option<i32>,
    final_bass_pc: Option<i32>,
    open_cost: usize,
    target_cost: usize,
    best_cost: usize,
    saving_percent: i32,
    reason: &'static str,
}

fn scorewide_tonal_key_signature_decision(
    xml: &str,
    notes: &[AtomicNote],
) -> ScorewideTonalKeySignatureDecision {
    let opening_measure = tonal_evidence_measure_numbers_audit(xml).first().copied();
    let source_fifths = opening_measure
        .and_then(|measure| key_signature_measure_fifths(xml, measure));
    let visible = notes
        .iter()
        .filter(|note| note.visible && note.pitch.is_some())
        .collect::<Vec<_>>();
    let final_bass_pc = scorewide_final_bass_pitch_class(&visible);
    let open_cost = scorewide_signature_accidental_cost(&visible, 0);
    let best_cost = (-5..=6)
        .map(|fifths| scorewide_signature_accidental_cost(&visible, fifths))
        .min()
        .unwrap_or(open_cost);
    let weight_context = tonal_evidence_weight_context_audit(&visible);
    let mut scored = TONAL_EVIDENCE_PROFILES
        .iter()
        .map(|profile| {
            (
                *profile,
                tonal_evidence_profile_score_audit(&visible, *profile, &weight_context),
            )
        })
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| {
        right
            .1
            .cmp(&left.1)
            .then_with(|| left.0.fifths.abs().cmp(&right.0.fifths.abs()))
            .then_with(|| left.0.minor.cmp(&right.0.minor))
    });
    let global = scored.first().copied();
    let global_profile = global.map(|row| row.0);

    let fail = |reason| ScorewideTonalKeySignatureDecision {
        candidate: None,
        global_profile,
        opening_measure,
        source_fifths,
        final_bass_pc,
        open_cost,
        target_cost: 0,
        best_cost,
        saving_percent: 0,
        reason,
    };

    if source_fifths != Some(0) {
        return fail("source_signature_not_open_fail_closed");
    }
    if visible.len() < 24 || opening_measure.is_none() || final_bass_pc.is_none() {
        return fail("insufficient_scorewide_tonal_evidence");
    }
    let Some((global_profile_value, global_score)) = global else {
        return fail("global_tonal_profile_unbound");
    };
    let final_tonic = final_bass_pc.expect("checked final bass");
    let candidate_score = |profile: TonalEvidenceProfile| {
        scored
            .iter()
            .find(|row| row.0 == profile)
            .map_or(i64::MIN / 4, |row| row.1)
    };
    let qualifies = |profile: TonalEvidenceProfile| {
        if profile.fifths == 0 || profile.fifths.abs() > 5 {
            return false;
        }
        let target = scorewide_signature_accidental_cost(&visible, profile.fifths);
        let saving_green = open_cost > 0
            && open_cost.saturating_sub(target).saturating_mul(100)
                >= open_cost.saturating_mul(25);
        let near_best_green = target.saturating_mul(100)
            <= best_cost.max(1).saturating_mul(115);
        saving_green && near_best_green
    };

    let selected = if global_profile_value.tonic == final_tonic
        && qualifies(global_profile_value)
    {
        Some(global_profile_value)
    } else {
        TONAL_EVIDENCE_PROFILES
            .iter()
            .copied()
            .filter(|profile| profile.minor)
            .filter(|profile| profile.tonic == final_tonic)
            .filter(|profile| {
                global_profile_value.tonic
                    == (profile.tonic + 7).rem_euclid(12)
            })
            .filter(|profile| qualifies(*profile))
            .filter(|profile| {
                let score = candidate_score(*profile);
                score > 0
                    && (global_score <= 0
                        || score.saturating_mul(100)
                            >= global_score.saturating_mul(35))
            })
            .max_by(|left, right| {
                candidate_score(*left)
                    .cmp(&candidate_score(*right))
                    .then_with(|| {
                        scorewide_signature_accidental_cost(&visible, right.fifths)
                            .cmp(&scorewide_signature_accidental_cost(&visible, left.fifths))
                    })
            })
    };

    let Some(candidate) = selected else {
        return fail("sectional_modal_or_cost_conflict_fail_closed");
    };
    let target_cost = scorewide_signature_accidental_cost(&visible, candidate.fifths);
    let saving_display = if open_cost == 0 {
        0
    } else {
        open_cost
            .saturating_sub(target_cost)
            .saturating_mul(100)
            .saturating_div(open_cost) as i32
    };
    let reason = if candidate == global_profile_value {
        "global_profile_final_bass_and_notation_cost_agree"
    } else {
        "minor_tonic_final_bass_with_dominant_area_saturation_and_notation_cost_agree"
    };

    ScorewideTonalKeySignatureDecision {
        candidate: Some(candidate),
        global_profile,
        opening_measure,
        source_fifths,
        final_bass_pc,
        open_cost,
        target_cost,
        best_cost,
        saving_percent: saving_display,
        reason,
    }
}

fn scorewide_final_bass_pitch_class(notes: &[&AtomicNote]) -> Option<i32> {
    let last_measure = notes
        .iter()
        .filter_map(|note| note.measure.parse::<usize>().ok())
        .max()?;
    let final_start = notes
        .iter()
        .filter(|note| note.measure.parse::<usize>().ok() == Some(last_measure))
        .map(|note| note.start)
        .max()?;
    notes
        .iter()
        .filter(|note| note.measure.parse::<usize>().ok() == Some(last_measure))
        .filter(|note| note.start == final_start)
        .filter_map(|note| note.pitch.as_ref())
        .min_by_key(|pitch| pitch.midi)
        .map(|pitch| pitch.midi.rem_euclid(12))
}

fn scorewide_signature_accidental_cost(notes: &[&AtomicNote], fifths: i32) -> usize {
    let mut ordered = notes.to_vec();
    ordered.sort_by(|left, right| {
        left.measure
            .parse::<usize>()
            .unwrap_or(usize::MAX)
            .cmp(&right.measure.parse::<usize>().unwrap_or(usize::MAX))
            .then_with(|| left.start.cmp(&right.start))
            .then_with(|| left.note_start.cmp(&right.note_start))
    });
    let mut current_measure = None::<usize>;
    let mut state =
        std::collections::BTreeMap::<(String, String, i32), i32>::new();
    let mut cost = 0_usize;
    for note in ordered {
        let Some(pitch) = note.pitch.as_ref() else {
            continue;
        };
        let measure = note.measure.parse::<usize>().unwrap_or(usize::MAX);
        if current_measure != Some(measure) {
            state.clear();
            current_measure = Some(measure);
        }
        let key = (note.staff.clone(), pitch.step.clone(), pitch.octave);
        let default = scorewide_signature_default_alter(fifths, &pitch.step);
        let carried = state.get(&key).copied().unwrap_or(default);
        if carried != pitch.alter {
            cost = cost.saturating_add(1);
            state.insert(key, pitch.alter);
        }
    }
    cost
}

fn scorewide_signature_default_alter(fifths: i32, step: &str) -> i32 {
    const SHARP_ORDER: [&str; 7] = ["F", "C", "G", "D", "A", "E", "B"];
    const FLAT_ORDER: [&str; 7] = ["B", "E", "A", "D", "G", "C", "F"];
    if fifths > 0
        && SHARP_ORDER
            .iter()
            .take(fifths.min(7) as usize)
            .any(|candidate| *candidate == step)
    {
        1
    } else if fifths < 0
        && FLAT_ORDER
            .iter()
            .take((-fifths).min(7) as usize)
            .any(|candidate| *candidate == step)
    {
        -1
    } else {
        0
    }
}

#[derive(Debug, Clone)]
struct FinalSectionTonalKeyCandidate {
    boundary_measure: usize,
    end_measure: usize,
    profile: TonalEvidenceProfile,
    tail_notes: usize,
    chromatic_notes: usize,
    accidental_before: usize,
    accidental_after: usize,
    saving_percent: i32,
}

fn final_section_tonal_key_candidate(
    xml: &str,
    notes: &[AtomicNote],
) -> Option<FinalSectionTonalKeyCandidate> {
    let measures = tonal_evidence_measure_numbers_audit(xml);
    let opening_measure = measures.first().copied()?;
    let end_measure = measures.last().copied()?;
    let explicit_keys = functional_explicit_key_signatures(xml);
    if explicit_keys.len() != 1 || explicit_keys[0] != (opening_measure, 0) {
        return None;
    }

    let audit = tonal_segmentation_evidence_audit_no_mutation(xml);
    let full = audit
        .rows
        .iter()
        .filter(|row| row.evidence_groups_green == 6 && row.source_fifths == Some(0))
        .collect::<Vec<_>>();
    if full.len() != 1 {
        return None;
    }
    let row = full[0];
    if row.candidate.fifths == 0
        || row.boundary_measure <= opening_measure
        || row.boundary_measure >= end_measure
    {
        return None;
    }
    if audit.rows.iter().any(|candidate| {
        candidate.boundary_measure > row.boundary_measure
            && candidate.candidate != row.candidate
            && candidate.verdict != "held_as_tonicization_or_chromatic_colour"
    }) {
        return None;
    }

    let tail = notes
        .iter()
        .filter(|note| note.visible && note.pitch.is_some())
        .filter(|note| functional_measure_ordinal(note) >= row.boundary_measure)
        .collect::<Vec<_>>();
    if tail.is_empty()
        || scorewide_final_bass_pitch_class(&tail) != Some(row.candidate.tonic)
    {
        return None;
    }
    let (tail_best, _, _) = tonal_evidence_best_profile_audit(&tail)?;
    if tail_best != row.candidate {
        return None;
    }

    let accidental_before = scorewide_signature_accidental_cost(&tail, 0);
    let (spelled_xml, _) = functional_enharmonic_candidate_for_tonal_frame_from_measure(
        xml,
        row.candidate.fifths,
        row.candidate.tonic,
        row.candidate.minor,
        Some(row.boundary_measure),
        None,
    );
    let spelled_notes = parse_atomic_notes(&spelled_xml);
    let spelled_tail = spelled_notes
        .iter()
        .filter(|note| note.visible && note.pitch.is_some())
        .filter(|note| functional_measure_ordinal(note) >= row.boundary_measure)
        .collect::<Vec<_>>();
    if tail.len() != spelled_tail.len() {
        return None;
    }
    let accidental_after =
        scorewide_signature_accidental_cost(&spelled_tail, row.candidate.fifths);
    if accidental_after >= accidental_before {
        return None;
    }
    let saving = accidental_before.saturating_sub(accidental_after);
    let saving_share = match std::num::NonZeroUsize::new(accidental_before) {
        Some(accidental_before) => ((saving * 100) / accidental_before.get()) as i32,
        None => 0,
    };
    let chromatic_notes = tail
        .iter()
        .filter(|note| {
            note.pitch
                .as_ref()
                .is_some_and(|pitch| {
                        functional_key_diatonic_spelling(
                            row.candidate.fifths,
                            pitch.midi.rem_euclid(12),
                        )
                        .is_none()
                    },
                )
        })
        .count();

    Some(FinalSectionTonalKeyCandidate {
        boundary_measure: row.boundary_measure,
        end_measure,
        profile: row.candidate,
        tail_notes: tail.len(),
        chromatic_notes,
        accidental_before,
        accidental_after,
        saving_percent: saving_share,
    })
}

fn functional_explicit_key_signatures(xml: &str) -> Vec<(usize, i32)> {
    let mut keys = Vec::new();
    let mut scan = 0_usize;
    while let Some((start, end, measure)) = find_next_measure_bounds(xml, scan) {
        if let Some(fifths) = extract_first_text_from_fragment(&xml[start..end], "fifths")
            .and_then(|value| value.trim().parse::<i32>().ok())
        {
            keys.push((measure, fifths));
        }
        scan = end;
    }
    keys
}

#[derive(Debug, Clone)]
struct TonalSegmentationEvidenceRow {
    boundary_measure: usize,
    pre_start_measure: usize,
    post_end_measure: usize,
    source_fifths: Option<i32>,
    candidate: TonalEvidenceProfile,
    lbdm_boundary_score: i32,
    bee_queen_boundary_score: i32,
    tonal_margin: i64,
    stable_measures: usize,
    bass_cadence_score: i32,
    notation_cost_before: i64,
    notation_cost_after: i64,
    hmm_transition_score: i64,
    evidence_groups_green: usize,
    counterevidence: String,
    verdict: &'static str,
}

#[derive(Debug, Clone, Default)]
struct TonalSegmentationEvidenceAudit {
    boundaries_checked: usize,
    key_change_candidates: usize,
    full_evidence_candidates: usize,
    promising_candidates: usize,
    held_candidates: usize,
    rows: Vec<TonalSegmentationEvidenceRow>,
}

fn append_tonal_segmentation_evidence_audit_no_mutation_section(out: &mut String, xml: &str) {
    let audit = tonal_segmentation_evidence_audit_no_mutation(xml);
    out.push_static_section_header_v795(
        "\nTONAL-SEGMENTATION-EVIDENCE-AUDIT-NO-MUTATION",
        "---------------------------------------------------",
    );
    out.push_format_args(format_args!(
        "Status: v{} literaturgestuetzter Audit-only-Tonalitaetsbeweis. Cambouropoulos/LBDM und die Bienenkoenigin liefern nur Grenzkorrelation; Tonartautoritaet verlangt getrennt gewichtete Profil-, Bass-/Kadenz-, Transitions-, Stabilitaets- und Notationskostenbeweise. Dieser Audit schreibt selbst nichts. Nur der bestehende Finalabschnitts-Writer darf genau einen eindeutigen sechsfaktoriellen Endabschnitt nach zusaetzlichem Tail-, Schlussbass- und Notationsreadback konsumieren. Kein Doppelstrichrecht.\n",
        env!("CARGO_PKG_VERSION")
    ));
    out.push_format_args(format_args!(
        "Kurz: TonalSegmentationEvidenceAudit: BoundariesChecked {}; KeyChangeCandidates {}; FullEvidenceCandidates {}; PromisingCandidates {}; HeldCandidates {}; WriterOpen nein; RenderXmlWrite nein; MusicMutation nein; MasterApply nein.\n",
        audit.boundaries_checked,
        audit.key_change_candidates,
        audit.full_evidence_candidates,
        audit.promising_candidates,
        audit.held_candidates,
    ));
    crate::push_static_named_policy_line_v765(
        out,
        "BoundaryMethod",
        " LBDM-artige lokale Diskontinuitaet und Bienenkoenigin-Korrelation aus Fuehrungs-, Dichte- und Atemwechseln. Diese Kanaele lokalisieren nur ein moegliches Gelenk und beweisen keine Tonart.",
    );
    crate::push_static_named_policy_line_v765(
        out,
        "TonalMethod",
        " Alle 24 Dur-/Mollprofile werden ueber sichtbare Noten nach Dauer und metrischer Position gewichtet; Bass-/Kadenzsupport, mehrtaktige Stabilitaet und geringere Vorzeichenlast werden separat bilanziert.",
    );
    crate::push_static_named_policy_line_v765(
        out,
        "TransitionMethod",
        " Chew-inspirierter Quintabstands-Proxy und HMM-artige Wechselstrafe verhindern kurzlebige Tonikalisierungs- und Chromatikwechsel. Dies ist noch keine vollstaendige Spiral-Array- oder Viterbi-Implementierung.",
    );
    crate::push_static_ledger_header_v804(
        out,
        "TonalBoundary",
        "BoundaryM | Window | SourceFifths | Candidate | LBDM | Bee | Margin | Stable | BassCadence | AccidentalCostBeforeAfter | HMM | GreenGroups | Counterevidence | Verdict",
    );
    if audit.rows.is_empty() {
        out.push_str("none | no_identity_free_key_change_candidate | writer_closed\n");
    } else {
        for row in &audit.rows {
            out.push_format_args(format_args!(
                "{} | {}-{} | {} | {}({}) | {} | {} | {} | {} | {} | {}/{} | {} | {} | {} | {}\n",
                row.boundary_measure,
                row.pre_start_measure,
                row.post_end_measure,
                row.source_fifths.map_or_else(|| "unknown".to_string(), |value| value.to_string()),
                row.candidate.name,
                row.candidate.fifths,
                row.lbdm_boundary_score,
                row.bee_queen_boundary_score,
                row.tonal_margin,
                row.stable_measures,
                row.bass_cadence_score,
                row.notation_cost_before,
                row.notation_cost_after,
                row.hmm_transition_score,
                row.evidence_groups_green,
                row.counterevidence,
                row.verdict,
            ));
        }
    }
    crate::push_static_dec_line_v765(
        out,
        " Audit-only. Die Sektion selbst bleibt schreibgeschlossen. Der vorhandene Finalabschnitts-Writer darf nur den einzigen sechsfaktoriellen Kandidaten eines offen begonnenen Scores nach nicht zurueckkehrendem Tail-, Schlussbass- und Vorzeichenersparnis-Readback konsumieren. Vorzeichnung, enharmonische Schreibweise und Doppelstrich bleiben getrennte Entscheidungen.",
    );
}

fn tonal_segmentation_evidence_audit_no_mutation(xml: &str) -> TonalSegmentationEvidenceAudit {
    let notes = parse_atomic_notes(xml);
    let measure_numbers = tonal_evidence_measure_numbers_audit(xml);
    let mut audit = TonalSegmentationEvidenceAudit {
        boundaries_checked: measure_numbers.len().saturating_sub(1),
        ..TonalSegmentationEvidenceAudit::default()
    };
    if measure_numbers.len() < 4 {
        return audit;
    }

    let window_radius = 4_usize;
    for boundary_index in 1..measure_numbers.len() {
        let post_start_measure = measure_numbers[boundary_index];
        let pre_start_index = boundary_index.saturating_sub(window_radius);
        let post_end_index = (boundary_index + window_radius).min(measure_numbers.len());
        let pre_numbers = &measure_numbers[pre_start_index..boundary_index];
        let post_numbers = &measure_numbers[boundary_index..post_end_index];
        if pre_numbers.len() < 2 || post_numbers.len() < 3 {
            continue;
        }
        let pre_notes = tonal_evidence_notes_for_measures_audit(&notes, pre_numbers);
        let post_notes = tonal_evidence_notes_for_measures_audit(&notes, post_numbers);
        if pre_notes.len() < 8 || post_notes.len() < 12 {
            continue;
        }
        let Some((candidate, best_score, second_score)) = tonal_evidence_best_profile_audit(&post_notes) else {
            continue;
        };
        let source_fifths = carried_key_signature_fifths(xml, post_start_measure);
        if source_fifths == Some(candidate.fifths) {
            continue;
        }

        let tonal_margin = best_score.saturating_sub(second_score);
        let lbdm_boundary_score = tonal_evidence_lbdm_boundary_score_audit(&pre_notes, &post_notes);
        let bee_queen_boundary_score = tonal_evidence_bee_queen_boundary_score_audit(&pre_notes, &post_notes);
        let stable_measures = tonal_evidence_stable_measures_audit(&notes, post_numbers, candidate);
        let bass_cadence_score = tonal_evidence_bass_cadence_score_audit(&post_notes, candidate);
        let notation_cost_before = tonal_evidence_source_notation_cost_audit(&post_notes, source_fifths);
        let notation_cost_after = tonal_evidence_notation_cost_audit(&post_notes, candidate);
        let transition_penalty = tonal_evidence_transition_penalty_audit(source_fifths, candidate);
        let hmm_transition_score = tonal_margin.saturating_sub(transition_penalty);
        let notation_saving = notation_cost_before.saturating_sub(notation_cost_after);

        let boundary_green = lbdm_boundary_score >= 2 && bee_queen_boundary_score >= 1;
        let profile_green = tonal_margin >= 48;
        let stability_green = stable_measures >= 3;
        let bass_green = bass_cadence_score >= 2;
        let notation_green = notation_saving >= 24;
        let transition_green = hmm_transition_score >= 24;
        let evidence_groups_green = [
            boundary_green,
            profile_green,
            stability_green,
            bass_green,
            notation_green,
            transition_green,
        ]
        .iter()
        .filter(|green| **green)
        .count();

        let mut counterevidence = Vec::new();
        if !boundary_green { counterevidence.push("boundary_weak"); }
        if !profile_green { counterevidence.push("profile_margin_low"); }
        if !stability_green { counterevidence.push("short_tonicization_risk"); }
        if !bass_green { counterevidence.push("bass_cadence_unconfirmed"); }
        if !notation_green { counterevidence.push("notation_saving_small"); }
        if !transition_green { counterevidence.push("transition_prior_rejects"); }
        let counterevidence = if counterevidence.is_empty() {
            "none_but_corpus_calibration_open".to_string()
        } else {
            counterevidence.join(",")
        };
        let verdict = if evidence_groups_green == 6 {
            "full_evidence_candidate_write_still_closed"
        } else if evidence_groups_green >= 4 {
            "promising_candidate_more_corpus_calibration"
        } else {
            "held_as_tonicization_or_chromatic_colour"
        };

        audit.rows.push(TonalSegmentationEvidenceRow {
            boundary_measure: post_start_measure,
            pre_start_measure: *pre_numbers.first().unwrap_or(&post_start_measure),
            post_end_measure: *post_numbers.last().unwrap_or(&post_start_measure),
            source_fifths,
            candidate,
            lbdm_boundary_score,
            bee_queen_boundary_score,
            tonal_margin,
            stable_measures,
            bass_cadence_score,
            notation_cost_before,
            notation_cost_after,
            hmm_transition_score,
            evidence_groups_green,
            counterevidence,
            verdict,
        });
    }

    audit.rows.sort_by(|left, right| {
        right.evidence_groups_green
            .cmp(&left.evidence_groups_green)
            .then_with(|| right.tonal_margin.cmp(&left.tonal_margin))
            .then_with(|| left.boundary_measure.cmp(&right.boundary_measure))
    });
    audit.key_change_candidates = audit.rows.len();
    audit.full_evidence_candidates = audit.rows.iter().filter(|row| row.evidence_groups_green == 6).count();
    audit.promising_candidates = audit.rows.iter().filter(|row| row.evidence_groups_green >= 4 && row.evidence_groups_green < 6).count();
    audit.held_candidates = audit.rows.len().saturating_sub(audit.full_evidence_candidates + audit.promising_candidates);
    audit
}

fn tonal_evidence_measure_numbers_audit(xml: &str) -> Vec<usize> {
    let mut numbers = Vec::new();
    let mut scan = 0_usize;
    while let Some((_, end, number)) = find_next_measure_bounds(xml, scan) {
        if numbers.last().copied() != Some(number) {
            numbers.push(number);
        }
        scan = end;
    }
    numbers
}

fn tonal_evidence_notes_for_measures_audit<'a>(notes: &'a [AtomicNote], measures: &[usize]) -> Vec<&'a AtomicNote> {
    let wanted = measures.iter().copied().collect::<std::collections::BTreeSet<_>>();
    notes.iter()
        .filter(|note| note.visible && note.pitch.is_some())
        .filter(|note| note.measure.parse::<usize>().ok().is_some_and(|value| wanted.contains(&value)))
        .collect()
}

fn tonal_evidence_best_profile_audit(notes: &[&AtomicNote]) -> Option<(TonalEvidenceProfile, i64, i64)> {
    let weight_context = tonal_evidence_weight_context_audit(notes);
    let mut scored = TONAL_EVIDENCE_PROFILES
        .iter()
        .map(|profile| (*profile, tonal_evidence_profile_score_audit(notes, *profile, &weight_context)))
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| {
        right.1.cmp(&left.1)
            .then_with(|| left.0.fifths.abs().cmp(&right.0.fifths.abs()))
            .then_with(|| left.0.minor.cmp(&right.0.minor))
    });
    let (best_profile, best_score) = scored.first().copied()?;
    let second_score = scored.get(1).map_or(best_score, |row| row.1);
    Some((best_profile, best_score, second_score))
}

fn tonal_evidence_profile_score_audit(
    notes: &[&AtomicNote],
    profile: TonalEvidenceProfile,
    weight_context: &TonalEvidenceWeightContext,
) -> i64 {
    let mut score = 0_i64;
    for note in notes {
        let Some(pitch) = note.pitch.as_ref() else { continue; };
        let pitch_class = pitch.midi.rem_euclid(12);
        let weight = tonal_evidence_note_weight_audit(note, weight_context);
        if tonal_evidence_scale_contains_audit(profile, pitch_class) {
            score += 3 * weight;
        } else {
            score -= 4 * weight;
        }
        if pitch_class == profile.tonic {
            score += 2 * weight;
        } else if pitch_class == (profile.tonic + 7).rem_euclid(12) {
            score += weight;
        }
        if profile.minor && pitch_class == (profile.tonic + 11).rem_euclid(12) {
            score += weight;
        }
    }
    score
}

#[derive(Debug, Clone)]
struct TonalEvidenceWeightContext {
    duration_unit: i64,
    measure_ends: std::collections::BTreeMap<String, i64>,
}

fn tonal_evidence_weight_context_audit(notes: &[&AtomicNote]) -> TonalEvidenceWeightContext {
    let duration_unit = notes
        .iter()
        .map(|note| note.duration)
        .filter(|duration| *duration > 0)
        .fold(0_i64, tonal_evidence_gcd_audit)
        .max(1);
    let mut measure_ends = std::collections::BTreeMap::<String, i64>::new();
    for note in notes {
        let end = note.start.saturating_add(note.duration.max(0));
        let entry = measure_ends.entry(note.measure.clone()).or_insert(0);
        *entry = (*entry).max(end);
    }
    TonalEvidenceWeightContext {
        duration_unit,
        measure_ends,
    }
}

fn tonal_evidence_gcd_audit(mut left: i64, mut right: i64) -> i64 {
    left = left.abs();
    right = right.abs();
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

fn tonal_evidence_note_weight_audit(
    note: &AtomicNote,
    context: &TonalEvidenceWeightContext,
) -> i64 {
    let duration_weight = note
        .duration
        .max(context.duration_unit)
        .saturating_div(context.duration_unit)
        .clamp(1, 16);
    let measure_end = context
        .measure_ends
        .get(&note.measure)
        .copied()
        .unwrap_or(note.start.saturating_add(note.duration.max(0)))
        .max(1);
    let metric_weight = if note.start == 0 {
        4
    } else if note.start.saturating_mul(2) == measure_end {
        3
    } else if note.start.saturating_mul(4) % measure_end == 0 {
        2
    } else {
        1
    };
    duration_weight.saturating_mul(metric_weight)
}

fn tonal_evidence_scale_contains_audit(profile: TonalEvidenceProfile, pitch_class: i32) -> bool {
    let steps: &[i32] = if profile.minor {
        &[0, 2, 3, 5, 7, 8, 10]
    } else {
        &[0, 2, 4, 5, 7, 9, 11]
    };
    steps.iter().any(|step| (profile.tonic + step).rem_euclid(12) == pitch_class)
}

fn tonal_evidence_lbdm_boundary_score_audit(pre_notes: &[&AtomicNote], post_notes: &[&AtomicNote]) -> i32 {
    let mut score = 0_i32;
    let pre_last = pre_notes.iter().max_by_key(|note| (note.measure.parse::<usize>().unwrap_or(0), note.start));
    let post_first = post_notes.iter().min_by_key(|note| (note.measure.parse::<usize>().unwrap_or(usize::MAX), note.start));
    if let (Some(left), Some(right)) = (pre_last, post_first) {
        if let (Some(left_pitch), Some(right_pitch)) = (left.pitch.as_ref(), right.pitch.as_ref())
            && (left_pitch.midi - right_pitch.midi).abs() >= 5
        {
            score += 1;
        }
        let pre_measure_end = pre_notes.iter()
            .filter(|note| note.measure == left.measure)
            .map(|note| note.start.saturating_add(note.duration.max(0)))
            .max()
            .unwrap_or(left.start.saturating_add(left.duration.max(0)))
            .max(1);
        if left.start.saturating_add(left.duration.max(0)).saturating_mul(4) <= pre_measure_end.saturating_mul(3)
            || right.start.saturating_mul(4) >= pre_measure_end
        {
            score += 1;
        }
    }
    let pre_density = pre_notes.len() as i64;
    let post_density = post_notes.len() as i64;
    if (pre_density - post_density).abs().saturating_mul(2) >= pre_density.min(post_density).max(1) { score += 1; }
    let pre_mean = tonal_evidence_mean_midi_audit(pre_notes);
    let post_mean = tonal_evidence_mean_midi_audit(post_notes);
    if (pre_mean - post_mean).abs() >= 7 { score += 1; }
    score
}

fn tonal_evidence_bee_queen_boundary_score_audit(pre_notes: &[&AtomicNote], post_notes: &[&AtomicNote]) -> i32 {
    let mut score = 0_i32;
    if tonal_evidence_dominant_label_audit(pre_notes, true) != tonal_evidence_dominant_label_audit(post_notes, true) { score += 1; }
    if tonal_evidence_dominant_label_audit(pre_notes, false) != tonal_evidence_dominant_label_audit(post_notes, false) { score += 1; }
    if tonal_evidence_lbdm_boundary_score_audit(pre_notes, post_notes) >= 2 { score += 1; }
    if tonal_evidence_attack_spread_audit(pre_notes) != tonal_evidence_attack_spread_audit(post_notes) { score += 1; }
    score
}

fn tonal_evidence_dominant_label_audit(notes: &[&AtomicNote], staff: bool) -> String {
    let mut counts = std::collections::BTreeMap::<String, i64>::new();
    for note in notes {
        let label = if staff { note.staff.clone() } else { note.voice.clone() };
        *counts.entry(label).or_insert(0) += note.duration.max(1);
    }
    counts.into_iter().max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(&left.0))).map(|row| row.0).unwrap_or_default()
}

fn tonal_evidence_attack_spread_audit(notes: &[&AtomicNote]) -> usize {
    notes.iter().map(|note| (note.measure.clone(), note.start)).collect::<std::collections::BTreeSet<_>>().len()
}

fn tonal_evidence_mean_midi_audit(notes: &[&AtomicNote]) -> i64 {
    let (sum, count) = notes.iter().filter_map(|note| note.pitch.as_ref()).fold(
        (0_i64, 0_i64),
        |(sum, count), pitch| (sum + pitch.midi as i64, count + 1),
    );
    if count == 0 { 0 } else { sum / count }
}

fn tonal_evidence_stable_measures_audit(notes: &[AtomicNote], measures: &[usize], candidate: TonalEvidenceProfile) -> usize {
    let mut stable = 0_usize;
    for measure in measures {
        let local = tonal_evidence_notes_for_measures_audit(notes, std::slice::from_ref(measure));
        let Some((best, _, _)) = tonal_evidence_best_profile_audit(&local) else { break; };
        if best == candidate { stable += 1; } else { break; }
    }
    stable
}

fn tonal_evidence_bass_cadence_score_audit(notes: &[&AtomicNote], candidate: TonalEvidenceProfile) -> i32 {
    let mut by_measure = std::collections::BTreeMap::<usize, Vec<&AtomicNote>>::new();
    for note in notes {
        if let Ok(number) = note.measure.parse::<usize>() {
            by_measure.entry(number).or_default().push(*note);
        }
    }
    let mut score = 0_i32;
    for local in by_measure.values().take(1).chain(by_measure.values().rev().take(1)) {
        if let Some(lowest) = local.iter().filter_map(|note| note.pitch.as_ref()).min_by_key(|pitch| pitch.midi) {
            let pc = lowest.midi.rem_euclid(12);
            if pc == candidate.tonic { score += 2; }
            else if pc == (candidate.tonic + 7).rem_euclid(12) { score += 1; }
        }
        if let Some(last) = local.iter().max_by_key(|note| note.start).and_then(|note| note.pitch.as_ref()) {
            let pc = last.midi.rem_euclid(12);
            if pc == candidate.tonic || pc == (candidate.tonic + 7).rem_euclid(12) {
                score += 1;
            }
        }
    }
    score
}

fn tonal_evidence_notation_cost_audit(notes: &[&AtomicNote], profile: TonalEvidenceProfile) -> i64 {
    let weight_context = tonal_evidence_weight_context_audit(notes);
    notes.iter().map(|note| {
        let Some(pitch) = note.pitch.as_ref() else { return 0; };
        if tonal_evidence_scale_contains_audit(profile, pitch.midi.rem_euclid(12)) {
            0
        } else {
            tonal_evidence_note_weight_audit(note, &weight_context)
        }
    }).sum()
}

fn tonal_evidence_source_notation_cost_audit(notes: &[&AtomicNote], source_fifths: Option<i32>) -> i64 {
    let Some(fifths) = source_fifths else { return 0; };
    TONAL_EVIDENCE_PROFILES.iter()
        .filter(|profile| profile.fifths == fifths)
        .map(|profile| tonal_evidence_notation_cost_audit(notes, *profile))
        .min()
        .unwrap_or(0)
}

fn tonal_evidence_transition_penalty_audit(source_fifths: Option<i32>, candidate: TonalEvidenceProfile) -> i64 {
    let Some(source) = source_fifths else { return 48; };
    let raw = (source - candidate.fifths).abs() as i64;
    let normalized = raw % 12;
    let fifth_distance = normalized.min(12 - normalized);
    let source_has_candidate_mode = TONAL_EVIDENCE_PROFILES
        .iter()
        .any(|profile| profile.fifths == source && profile.minor == candidate.minor);
    let mode_penalty = if source_has_candidate_mode { 0 } else { 12 };
    fifth_distance.saturating_mul(12).saturating_add(mode_penalty)
}


#[cfg(test)]
mod tonal_segmentation_evidence_tests {
    use super::*;

    #[test]
    fn profiles_cover_twelve_major_and_twelve_minor_tonics() {
        let major_tonics = TONAL_EVIDENCE_PROFILES
            .iter()
            .filter(|profile| !profile.minor)
            .map(|profile| profile.tonic)
            .collect::<std::collections::BTreeSet<_>>();
        let minor_tonics = TONAL_EVIDENCE_PROFILES
            .iter()
            .filter(|profile| profile.minor)
            .map(|profile| profile.tonic)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(major_tonics.len(), 12);
        assert_eq!(minor_tonics.len(), 12);
    }

    #[test]
    fn nearby_fifth_transition_is_cheaper_than_remote_transition() {
        let nearby = TonalEvidenceProfile {
            name: "nearby",
            fifths: 1,
            tonic: 7,
            minor: false,
        };
        let remote = TonalEvidenceProfile {
            name: "remote",
            fifths: 6,
            tonic: 6,
            minor: false,
        };
        assert!(
            tonal_evidence_transition_penalty_audit(Some(0), nearby)
                < tonal_evidence_transition_penalty_audit(Some(0), remote)
        );
    }

    #[test]
    fn unknown_source_is_penalized_and_never_creates_notation_saving() {
        let candidate = TONAL_EVIDENCE_PROFILES[0];
        assert!(tonal_evidence_transition_penalty_audit(None, candidate) > 0);
        assert_eq!(tonal_evidence_source_notation_cost_audit(&[], None), 0);
    }

    #[test]
    fn tonal_boundary_ledger_is_complete_beyond_thirty_two_candidates() {
        let mut xml = String::from(
            "<score-partwise><part id=\"P1\"><measure number=\"\
1\"><attributes><key><fifths>0</fifths></key></attributes>",
        );
        let motif = "<note><pitch><step>G</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>\
                     <note><pitch><step>B</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>\
                     <note><pitch><step>D</step><octave>5</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>\
                     <note><pitch><step>F</step><alter>1</alter><octave>5</octave></pitch><duration>1</duration><voice>1</voice><staff>1</staff></note>";
        xml.push_str(motif);
        xml.push_str("</measure>");
        for measure in 2..=40 {
            xml.push_format_args(format_args!("<measure number=\"{measure}\">"));
            xml.push_str(motif);
            xml.push_str("</measure>");
        }
        xml.push_str("</part></score-partwise>");

        let audit = tonal_segmentation_evidence_audit_no_mutation(&xml);
        assert!(audit.key_change_candidates > 32);
        assert_eq!(audit.rows.len(), audit.key_change_candidates);

        let mut out = String::new();
        append_tonal_segmentation_evidence_audit_no_mutation_section(&mut out, &xml);
        let ledger_rows = out
            .lines()
            .filter(|line| {
                line.ends_with("full_evidence_candidate_write_still_closed")
                    || line.ends_with("promising_candidate_more_corpus_calibration")
                    || line.ends_with("held_as_tonicization_or_chromatic_colour")
            })
            .count();
        assert_eq!(ledger_rows, audit.key_change_candidates);
    }
}

// r725: retired late-span report-projection tests and now-unreachable map/barline helpers removed with the retired projection.


/// The bound key-signature edits of `rewrite_key_signature_for_measure_plain`
/// in application order (start-descending) and pre coordinates: replace an
/// existing `<key>`, insert into an existing `<attributes>`, or insert a new
/// attributes/key block after the measure opening tag.
fn key_signature_for_measure_edits(
    xml: &str,
    measure_number: usize,
    target_fifths: i32,
) -> Vec<(usize, usize, String)> {
    let mut edits = Vec::new();
    let mut scan = 0_usize;
    while let Some((start, end, number)) = find_next_measure_bounds(xml, scan) {
        if number == measure_number
            && let Some((local_start, local_end, replacement)) =
                measure_key_signature_edit(&xml[start..end], target_fifths)
        {
            edits.push((start + local_start, start + local_end, replacement));
        }
        scan = end;
    }
    edits.reverse();
    edits
}

/// Geometry of `rewrite_measure_key_signature` as one edit in measure-local
/// coordinates; `None` where that function leaves the measure unchanged.
fn measure_key_signature_edit(measure: &str, target_fifths: i32) -> Option<(usize, usize, String)> {
    let new_key = format!("        <key><fifths>{target_fifths}</fifths></key>");
    if let Some(attr_start) = measure.find("<attributes")
        && let Some(attr_end_rel) = measure[attr_start..].find("</attributes>")
    {
        let attr_end = attr_start + attr_end_rel + "</attributes>".len();
        let attr = &measure[attr_start..attr_end];
        if let Some(key_start_rel) = attr.find("<key") {
            let key_end_rel = attr[key_start_rel..].find("</key>")?;
            let key_end = key_start_rel + key_end_rel + "</key>".len();
            return Some((attr_start + key_start_rel, attr_start + key_end, new_key));
        }
        let close_rel = attr.find("</attributes>")?;
        return Some((
            attr_start + close_rel,
            attr_start + close_rel,
            format!("\n{new_key}"),
        ));
    }
    let insert_at = measure.find('>')? + 1;
    Some((
        insert_at,
        insert_at,
        format!("\n      <attributes>\n{new_key}\n      </attributes>"),
    ))
}

fn rewrite_key_signature_for_measure_plain(
    xml: &str,
    measure_number: usize,
    target_fifths: i32,
) -> String {
    let mut out = String::with_capacity(xml.len() + 128);
    let mut last = 0_usize;
    let mut scan = 0_usize;
    while let Some((start, end, number)) = find_next_measure_bounds(xml, scan) {
        out.push_str(&xml[last..start]);
        let measure = &xml[start..end];
        if number == measure_number {
            out.push_str(&rewrite_measure_key_signature(measure, target_fifths));
        } else {
            out.push_str(measure);
        }
        last = end;
        scan = end;
    }
    out.push_str(&xml[last..]);
    out
}



// ---- v0.10.713 former piano_voice_corpus_endpoint_tail.rs ----

// v0.10.709 PianoVoiceCorpusEndpointTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the PianoVoice/BehindBars wait tail and the late corpus/endpoint trace tail.
// Former piano_voice_behind_bars_wait_tail.rs and corpus_endpoint_trace_tail.rs are included below in the same order.

// ---- merged from src/pipeline_shards/corpus_endpoint_trace_tail.rs ----
// v0.10.701 CorpusEndpointTraceTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the late corpus/endpoint/staccato-report/voice-trace/WAV-onset tail.
// Former part_55..part_59 are included below in the same order.

// ---- merged from src/pipeline_shards/part_55.rs ----
// v0.10.627 Part06CorpusObservationTailShardNoBehaviorChange.
// Mechanical tail relief for corpus observation helpers previously at the end of part_06.rs.

fn observe_project_result(path: PathBuf, text: &str) -> CorpusProjectObservation {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("Projekt.result.txt");
    let mut project = file_name
        .strip_suffix(".result.txt")
        .unwrap_or(file_name)
        .to_string();
    for line in text.lines().take(20) {
        if let Some(value) = line.strip_prefix("Partitur:") {
            let value = value.trim();
            if !value.is_empty() {
                project = value.to_string();
            }
        }
    }
    let (blockers, warnings, hints) = parse_bwh_line(text).unwrap_or((0, 0, 0));
    let result_version = text
        .lines()
        .take(24)
        .find_map(|line| line.trim().strip_prefix("Werkzeug: etuschlopf "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let feedback_candidate_occurrences =
        parse_feedback_noninterference_summary_value(text, "CandidateOccurrences").unwrap_or(0);
    CorpusProjectObservation {
        project,
        report_path: path,
        result_version,
        result_ok: text.lines().any(|line| line.trim() == "Ergebnis: OK"),
        blockers,
        warnings,
        hints,
        retention_percent: parse_canonical_source_retention_percent(text),
        final_validator_clean: (blockers == 0 && warnings == 0)
            || text.contains("FinalValidator 0/0/0/0")
            || text.contains("Final B/W/H/K=0/0/0/0")
            || text.contains("finaler Validator B0 W0 H0 K0"),
        outputs_complete: None,
        execution_outcome_observed: None,
        feedback_candidate_occurrences,
        feedback_transport_green: parse_feedback_noninterference_variants_ratio(text)
            .is_some_and(|(stable, total)| total > 0 && stable == total),
        feedback_proof_green: parse_feedback_noninterference_summary_bool(text, "ProofGreen"),
        feedback_origin_can_select: parse_feedback_noninterference_summary_bool(
            text,
            "OriginCanSelectCandidate",
        ),
        tonal_boundaries_checked: parse_tonal_segmentation_summary_value(text, "BoundariesChecked")
            .unwrap_or(0),
    }
}



fn feedback_noninterference_summary_line(text: &str) -> Option<&str> {
    text.lines()
        .find(|line| line.contains("Kurz: FeedbackOriginNonInterferencePipelineProof:"))
}

fn parse_feedback_noninterference_summary_value(text: &str, label: &str) -> Option<usize> {
    let line = feedback_noninterference_summary_line(text)?;
    let marker = format!("{label} ");
    let tail = line.split_once(&marker)?.1;
    let digits = tail.chars().take_while(|ch| ch.is_ascii_digit()).collect::<String>();
    digits.parse::<usize>().ok()
}

fn parse_feedback_noninterference_variants_ratio(text: &str) -> Option<(usize, usize)> {
    let evidence = crate::feedback_proof_transport::parse_feedback_proof_variant_evidence(text);
    evidence.transport_green.then_some((evidence.stable, evidence.total))
}

fn parse_feedback_noninterference_summary_bool(text: &str, label: &str) -> bool {
    let Some(line) = feedback_noninterference_summary_line(text) else {
        return false;
    };
    let marker = format!("{label} ");
    let Some(tail) = line.split_once(&marker).map(|parts| parts.1) else {
        return false;
    };
    tail.starts_with("ja")
}

fn parse_tonal_segmentation_summary_value(text: &str, label: &str) -> Option<usize> {
    let line = text.lines().find(|line| line.contains("Kurz: TonalSegmentationEvidenceAudit:"))?;
    let marker = format!("{label} ");
    let tail = line.split_once(&marker)?.1;
    let digits = tail.chars().take_while(|ch| ch.is_ascii_digit()).collect::<String>();
    digits.parse::<usize>().ok()
}







fn missing_expected_projects(
    expected_projects: &[String],
    observations: &[CorpusProjectObservation],
) -> Vec<String> {
    expected_projects
        .iter()
        .filter(|expected| {
            !observations
                .iter()
                .any(|observation| observation.project == expected.as_str())
        })
        .cloned()
        .collect()
}

fn join_project_list(values: &[String]) -> String {
    values.join(", ")
}

// ---- merged from src/pipeline_shards/part_56.rs ----
// v0.10.627 Part16NotationEndpointTailShardNoBehaviorChange.
// Mechanical tail relief for notation endpoint cleanup helpers previously at the end of legacy part_16.rs (now composer_recovery_octave_beam_export_tail.rs).

#[derive(Debug, Clone)]
struct NotationEndpointTag {
    type_attr: String,
    number: String,
    abs_start: usize,
    abs_end: usize,
}

#[derive(Debug, Clone)]
struct NotationEndpointNote {
    visible: bool,
    rest: bool,
    staff: String,
    voice: String,
    pitch_midi: Option<i32>,
    slurs: Vec<NotationEndpointTag>,
    ties: Vec<NotationEndpointTag>,
}

fn remove_orphan_slur_and_tie_endpoints(
    xml: &mut String,
) -> ((usize, usize), master_state_identity::OwnerReceipts) {
    let notes = collect_notation_endpoint_notes(xml);
    if notes.is_empty() {
        return ((0, 0), master_state_identity::OwnerReceipts::default());
    }
    let mut remove_slur_ranges: Vec<(usize, usize)> = Vec::new();
    let mut remove_tie_ranges: Vec<(usize, usize)> = Vec::new();

    for (idx, note) in notes.iter().enumerate() {
        for tag in &note.slurs {
            if !note.visible || note.rest {
                remove_slur_ranges.push((tag.abs_start, tag.abs_end));
                continue;
            }
            match tag.type_attr.as_str() {
                "start" => {
                    let has_stop = notes.iter().skip(idx + 1).any(|other| {
                        other.visible
                            && !other.rest
                            && other.slurs.iter().any(|candidate| candidate.number == tag.number && candidate.type_attr == "stop")
                    });
                    if !has_stop {
                        remove_slur_ranges.push((tag.abs_start, tag.abs_end));
                    }
                }
                "stop" => {
                    let has_start = notes.iter().take(idx).any(|other| {
                        other.visible
                            && !other.rest
                            && other.slurs.iter().any(|candidate| candidate.number == tag.number && candidate.type_attr == "start")
                    });
                    if !has_start {
                        remove_slur_ranges.push((tag.abs_start, tag.abs_end));
                    }
                }
                _ => remove_slur_ranges.push((tag.abs_start, tag.abs_end)),
            }
        }

        for tag in &note.ties {
            if !note.visible || note.rest || note.pitch_midi.is_none() {
                remove_tie_ranges.push((tag.abs_start, tag.abs_end));
                continue;
            }
            let midi = note.pitch_midi.unwrap();
            match tag.type_attr.as_str() {
                "start" => {
                    let has_stop = next_local_tie_target(&notes, idx, midi)
                        .is_some_and(|other| other.ties.iter().any(|candidate| candidate.type_attr == "stop"));
                    if !has_stop {
                        remove_tie_ranges.push((tag.abs_start, tag.abs_end));
                    }
                }
                "stop" => {
                    let has_start = previous_local_tie_source(&notes, idx, midi)
                        .is_some_and(|other| other.ties.iter().any(|candidate| candidate.type_attr == "start"));
                    if !has_start {
                        remove_tie_ranges.push((tag.abs_start, tag.abs_end));
                    }
                }
                _ => remove_tie_ranges.push((tag.abs_start, tag.abs_end)),
            }
        }
    }

    let slur_count = remove_slur_ranges.len();
    let tie_count = remove_tie_ranges.len();
    let mut all = remove_slur_ranges;
    all.extend(remove_tie_ranges);
    let mut receipts = master_state_identity::OwnerReceipts::default();
    receipts.push_exact(remove_xml_ranges(xml, &all));
    receipts.push_exact(remove_empty_notations_blocks(xml).1);
    ((slur_count, tie_count), receipts)
}

// ---- merged from src/pipeline_shards/part_57.rs ----
// v0.10.627 Part12StaccatoReportTailShardNoBehaviorChange.
// Mechanical tail relief for the final staccato report helper now owned by the Noether/register/articulation readback tail.


// ---- merged from src/pipeline_shards/part_58.rs ----
// v0.10.627 Part04VoiceFiberTraceTailShardNoBehaviorChange.
// Mechanical tail relief for voice/fiber trace report helpers previously at the end of part_04.rs.

fn atomic_note_pitch_label(note: &AtomicNote) -> String {
    note.pitch
        .as_ref()
        .map_or_else(|| "Rest".to_string(), |p| format!("{}/MIDI{}", p.label, p.midi))
}

fn same_atomic_note(a: &AtomicNote, b: &AtomicNote) -> bool {
    a.note_start == b.note_start && a.note_end == b.note_end
}

// ---- merged from src/pipeline_shards/part_59.rs ----
// v0.10.635 WavOnsetOrderTruthAuditNoMutation.
// Coarse report-only bridge from notated onsets to WAV transient positions.

#[cfg(test)]
mod functional_respelling_exact_receipt_tests {
    use super::*;

    fn note(step: &str, alter: i32, octave: i32) -> String {
        let alter = if alter == 0 { String::new() } else { format!("<alter>{alter}</alter>") };
        format!(
            "<note><pitch><step>{step}</step>{alter}<octave>{octave}</octave></pitch><duration>1</duration><voice>1</voice><type>quarter</type><staff>1</staff></note>"
        )
    }

    fn part(id: &str, measures: &[String]) -> String {
        let body = measures
            .iter()
            .enumerate()
            .map(|(index, measure)| format!("<measure number=\"{}\">{measure}</measure>", index + 1))
            .collect::<String>();
        format!("<part id=\"{id}\">{body}</part>")
    }

    fn score(parts: &[String]) -> String {
        let list = parts
            .iter()
            .enumerate()
            .map(|(index, _)| format!("<score-part id=\"P{}\"><part-name>Piano</part-name></score-part>", index + 1))
            .collect::<String>();
        format!(
            "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list>{list}</part-list>{}</score-partwise>",
            parts.concat()
        )
    }

    fn opening(fifths: i32) -> String {
        format!("<attributes><divisions>1</divisions><key><fifths>{fifths}</fifths></key><time><beats>4</beats><beat-type>4</beat-type></time></attributes>")
    }

    fn respelling_receipt(master: &str, post: &str, edits: Vec<crate::master_state_identity::ExactEdit>) -> bool {
        ExactTransformationReceipt::mint_exact_receipt(
            FunctionalRespellingExactMint { _sealed: () },
            master,
            post,
            edits,
        )
        .is_some_and(|receipt| {
            receipt.descriptor_row() == 60
                && receipt.realize_post_document(master).as_deref() == Some(post)
        })
    }

    #[test]
    fn key_signature_edit_covers_all_three_cases() {
        let boundary = usize::from(u8::MAX) % 7;
        let new_key = "        <key><fifths>2</fifths></key>";
        let cases = [
            // existing <key>: replace
            (
                "<attributes><divisions>1</divisions><key><fifths>0</fifths></key></attributes><note/>".to_string(),
                format!("<attributes><divisions>1</divisions>{new_key}</attributes><note/>"),
            ),
            // <attributes> without key: insert before </attributes>
            (
                "<attributes><divisions>1</divisions></attributes><note/>".to_string(),
                format!("<attributes><divisions>1</divisions>\n{new_key}</attributes><note/>"),
            ),
            // no <attributes>: insert a new attributes/key block after the opening tag
            (
                "<note/><note/>".to_string(),
                format!("\n      <attributes>\n{new_key}\n      </attributes><note/><note/>"),
            ),
        ];
        for (body, expected_body) in cases {
            let measure = format!("<measure number=\"{boundary}\">{body}</measure>");
            let expected = format!("<measure number=\"{boundary}\">{expected_body}</measure>");
            let (start, end, replacement) = measure_key_signature_edit(&measure, 2).expect("edit");
            let applied = format!("{}{}{}", &measure[..start], replacement, &measure[end..]);
            assert_eq!(applied, expected);
        }
        // An unterminated key binds no edit.
        let unterminated = format!(
            "<measure number=\"{boundary}\"><attributes><key><fifths>0</fifths></attributes></measure>"
        );
        assert!(measure_key_signature_edit(&unterminated, 2).is_none());
        // Every part's boundary measure receives its own edit, start-descending.
        let two_parts = score(&[
            part("P1", &[opening(0), "<note/>".to_string(), "<note/>".to_string()]),
            part("P2", &[opening(0), "<note/>".to_string(), "<note/>".to_string()]),
        ]);
        let edits = key_signature_for_measure_edits(&two_parts, 3, 2);
        assert_eq!(edits.len(), 2);
        assert!(edits[0].0 > edits[1].0);
    }

    #[test]
    fn scorewide_respelling_geometry_reproduces_the_legacy_candidate() {
        let master_part = |id: &str| {
            part(id, &[format!("{}{}{}{}{}", opening(2), note("D", 0, 4), note("G", -1, 4), note("D", -1, 5), note("D", 0, 3))])
        };
        let master = score(&[master_part("P1"), master_part("P2")]);
        let legacy = functional_enharmonic_candidate_for_tonal_frame(&master, 2, 2, false).0;
        assert_ne!(legacy, master);
        assert_eq!(legacy.matches("<step>F</step><alter>1</alter>").count(), 2);
        assert_eq!(legacy.matches("<step>C</step><alter>1</alter>").count(), 2);
        let edits = crate::master_state_identity::exact_script_from_descending_application(
            functional_respelling_edits_for_tonal_frame_from_measure(&master, 2, 2, false, None, None),
        );
        assert_eq!(edits.len(), 4);
        assert!(respelling_receipt(&master, &legacy, edits.clone()));
        // Manipulated geometry is refused.
        let mut manipulated = edits;
        manipulated.pop();
        assert!(!respelling_receipt(&master, &legacy, manipulated));
        // Already functional spelling: no edit, no receipt.
        assert!(functional_respelling_edits_for_tonal_frame_from_measure(&legacy, 2, 2, false, None, None).is_empty());
    }

    #[test]
    fn final_section_key_and_respelling_edits_compose_into_one_script() {
        for boundary_body in [String::new(), "<attributes><divisions>1</divisions></attributes>".to_string()] {
            let master = score(&[part(
                "P1",
                &[
                    format!("{}{}", opening(0), note("C", 0, 4)),
                    note("E", 0, 4),
                    format!("{boundary_body}{}{}", note("G", -1, 4), note("D", 0, 3)),
                ],
            )]);
            let key_edits = key_signature_for_measure_edits(&master, 3, 2);
            assert_eq!(key_edits.len(), 1);
            let (start, end, replacement) = key_edits[0].clone();
            let keyed = format!("{}{}{}", &master[..start], replacement, &master[end..]);
            assert_eq!(functional_explicit_key_signatures(&keyed), vec![(1, 0), (3, 2)]);
            let legacy = functional_enharmonic_candidate_for_tonal_frame_from_measure(&keyed, 2, 2, false, Some(3), None).0;
            assert!(legacy.contains("<step>F</step><alter>1</alter>"));
            let composed = crate::master_state_identity::compose_exact_edit_scripts(
                &master,
                crate::master_state_identity::exact_script_from_descending_application(key_edits),
                crate::master_state_identity::exact_script_from_descending_application(
                    functional_respelling_edits_for_tonal_frame_from_measure(&keyed, 2, 2, false, Some(3), None),
                ),
            )
            .expect("composable");
            assert!(respelling_receipt(&master, &legacy, composed));
        }
    }

    #[test]
    fn tonal_frame_fact_binds_exactly_its_snapshot() {
        let render = score(&[part("P1", &[format!("{}{}", opening(2), note("D", 0, 3))])]);
        let fact = TonalFrameFact::derive_from_render_snapshot(&render).expect("fact");
        assert!(fact.binds_snapshot(&render));
        assert!(!fact.binds_snapshot(&render.replace("<fifths>2", "<fifths>3")));
        assert_eq!(fact.fifths_vector(), Some(&[2][..]));
        assert_eq!(fact.opening_fifths, Some(2));
        assert_eq!(fact.scorewide_final_bass, Some(2));
        assert_eq!(fact.boundary_tail_bass, None);
        assert_eq!(TonalFrameFact::derive_from_render_snapshot(&render), Some(fact));
        assert!(TonalFrameFact::derive_from_render_snapshot("<broken").is_none());
        let sectioned = score(&[part(
            "P1",
            &[
                format!("{}{}", opening(0), note("C", 0, 4)),
                format!("<attributes><key><fifths>2</fifths></key></attributes>{}", note("D", 0, 3)),
            ],
        )]);
        let sectioned_fact = TonalFrameFact::derive_from_render_snapshot(&sectioned).expect("fact");
        assert_eq!(sectioned_fact.explicit_key_vector, vec![(1, 0), (2, 2)]);
        assert_eq!(sectioned_fact.boundary_tail_bass, Some((2, Some(2))));
    }
}

pub(crate) struct RenderRow66BlockMint { _private: () }

#[cfg(test)]
#[path = "../../tests/unit/final_respelling_course_continuation.rs"]
mod original_respelling_course_tests;

#[cfg(test)]
#[path = "../../tests/unit/original_complete_tie_chain_respelling.rs"]
mod original_complete_chain_tests;
