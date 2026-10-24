// Pipeline-owned WAV-truth / external-feedback support module.
// The module imports only the private pipeline contracts it actually consumes.
use super::{
    audio_salience_features, build_assertive_visible_articulation_candidates,
    decode_wav_mono_samples, parse_atomic_notes,
    render_scope_metric_expected_and_beat_map, select_assertive_visible_articulation_rows,
    WavTruthArticulationGateAudit, WavTruthArticulationRow,
};
use crate::reference::ReferenceReport;

pub fn wav_slice_rms_milli(samples: &[f32]) -> i64 {
    if samples.is_empty() {
        return 0;
    }
    let mut sq = 0.0_f64;
    for sample in samples {
        let v = *sample as f64;
        sq += v * v;
    }
    ((sq / samples.len() as f64).sqrt() * 1000.0).round().clamp(0.0, 4000.0) as i64
}

pub fn wav_truth_candidate_window(
    samples_len: usize,
    measure_count: usize,
    scope: usize,
    onset: i64,
    expected_divisions: i32,
) -> usize {
    if samples_len == 0 || measure_count == 0 {
        return 0;
    }
    let expected = (expected_divisions.max(1)) as f64;
    let local = (onset.max(0) as f64 / expected).clamp(0.0, 0.999_999);
    let global = ((scope.saturating_sub(1)) as f64 + local) / measure_count.max(1) as f64;
    ((global.clamp(0.0, 0.999_999)) * samples_len as f64).round() as usize
}



pub fn build_wav_truth_articulation_gate_no_mutation(
    reference: &ReferenceReport,
    xml: &str,
    source_label: &str,
) -> WavTruthArticulationGateAudit {
    let candidates = build_assertive_visible_articulation_candidates(xml);
    let mut audit = WavTruthArticulationGateAudit {
        candidate_count: candidates.len(),
        verdict: "wav_truth_articulation_gate_no_write".to_string(),
        ..WavTruthArticulationGateAudit::default()
    };

    let Some(wav) = reference.wav.as_ref() else {
        audit.done_fragment = format!(
            "WavTruthArticulationFeatureLedgerMaterializedNoMutation: WAV fehlt; ScoreKandidaten {}; WriteReady 0; RenderXmlWrite nein; MusicMutation nein",
            audit.candidate_count
        );
        audit.pipeline_line = format!(
            "WavTruthArticulationFeatureLedgerMaterializedNoMutation v{}: offen; keine WAV-Datei verfügbar; AssertiveVisibleArticulation bleibt score-only Kandidatenpool; WriteReady 0; RenderXmlWrite nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            audit.verdict
        );
        return audit;
    };
    audit.wav_path = wav.path.display().to_string();
    let Some((samples, sample_rate)) = decode_wav_mono_samples(&wav.path) else {
        audit.done_fragment = format!(
            "WavTruthArticulationFeatureLedgerMaterializedNoMutation: WAV nicht dekodiert; ScoreKandidaten {}; WriteReady 0; RenderXmlWrite nein; MusicMutation nein",
            audit.candidate_count
        );
        audit.pipeline_line = format!(
            "WavTruthArticulationFeatureLedgerMaterializedNoMutation v{}: offen; WAV {} nicht dekodiert; AssertiveVisibleArticulation bleibt score-only Kandidatenpool; WriteReady 0; RenderXmlWrite nein; MusicMutation nein; Urteil {}.",
            env!("CARGO_PKG_VERSION"),
            wav.path.display(),
            audit.verdict
        );
        return audit;
    };

    audit.wav_decoded = true;
    let measure_map = render_scope_metric_expected_and_beat_map(xml);
    let measure_count = measure_map.keys().copied().max().unwrap_or_else(|| {
        parse_atomic_notes(xml)
            .into_iter()
            .filter_map(|n| n.measure.parse::<usize>().ok())
            .max()
            .unwrap_or(1)
    }).max(1);
    let whole_energy = wav_slice_rms_milli(&samples).max(1);
    let (_whole_e, whole_attack, whole_flux) = audio_salience_features(&samples);
    let reference_attack = whole_attack.max(whole_flux / 2).max(1);
    audit.wav_reference_energy_milli = whole_energy;
    audit.wav_reference_attack_milli = reference_attack;
    audit.wav_reference_flux_milli = whole_flux.max(0);
    let sr = sample_rate.max(1) as usize;
    let pre_len = (sr / 20).max(64);   // ca. 50ms
    let main_len = (sr / 12).max(128); // ca. 83ms
    let post_len = (sr / 6).max(256);  // ca. 166ms
    audit.wav_windows = samples.len() / 1024;

    let mut selected_for_probe = select_assertive_visible_articulation_rows(&candidates, 96);
    if selected_for_probe.is_empty() {
        selected_for_probe = candidates.iter().take(96).cloned().collect();
    }
    audit.wav_checked_candidates = selected_for_probe.len();

    for row in selected_for_probe {
        let (expected, _beat_unit) = measure_map.get(&row.scope).copied().unwrap_or((384, 96));
        let center = wav_truth_candidate_window(samples.len(), measure_count, row.scope, row.onset, expected);
        let pre_start = center.saturating_sub(pre_len);
        let main_end = center.saturating_add(main_len).min(samples.len());
        let post_end = main_end.saturating_add(post_len).min(samples.len());
        let pre_slice = &samples[pre_start..center.min(samples.len())];
        let main_slice = &samples[center.min(samples.len())..main_end];
        let post_slice = &samples[main_end..post_end];
        let local_energy = wav_slice_rms_milli(main_slice);
        let pre_energy = wav_slice_rms_milli(pre_slice);
        let post_energy = wav_slice_rms_milli(post_slice);
        let (_energy_feature, attack, flux) = audio_salience_features(main_slice);
        let attack_score = attack.max(flux / 2).max(local_energy.saturating_sub(pre_energy).max(0));
        let decay_milli = if local_energy > 0 {
            (post_energy.saturating_mul(1000) / local_energy.max(1)).clamp(0, 4000)
        } else {
            0
        };
        let energy_ratio_milli = (local_energy.saturating_mul(1000) / whole_energy.max(1)).clamp(0, 8000);
        let attack_ratio_milli = (attack_score.saturating_mul(1000) / reference_attack.max(1)).clamp(0, 8000);
        let energy_strong = energy_ratio_milli >= 1150;
        let attack_strong = attack_ratio_milli >= 1150;
        let sustain_strong = decay_milli >= 620 && energy_ratio_milli >= 850;
        let short_release = decay_milli <= 420 && attack_strong;

        let (wav_suggested_mark, wav_confidence_milli) = if short_release {
            audit.wav_suggested_staccato += 1;
            ("staccato".to_string(), attack_ratio_milli.max(1000_i64.saturating_sub(decay_milli).max(0)))
        } else if attack_strong || energy_strong {
            audit.wav_suggested_accent += 1;
            ("accent".to_string(), attack_ratio_milli.max(energy_ratio_milli))
        } else if sustain_strong {
            audit.wav_suggested_tenuto += 1;
            ("tenuto".to_string(), decay_milli.max(energy_ratio_milli))
        } else {
            ("no_mark".to_string(), attack_ratio_milli.max(energy_ratio_milli).max(decay_milli))
        };

        let score_wav_relation = if wav_suggested_mark == "no_mark" {
            audit.wav_truth_blocked += 1;
            "score_only_blocked_no_wav_signal".to_string()
        } else if row.mark == wav_suggested_mark {
            audit.score_wav_match += 1;
            if wav_confidence_milli >= 1150 {
                audit.wav_truth_strong += 1;
            } else {
                audit.wav_truth_medium += 1;
            }
            "score_wav_match".to_string()
        } else {
            audit.score_wav_mismatch += 1;
            audit.wav_truth_medium += 1;
            format!("score_wav_mismatch_wav_suggests_{wav_suggested_mark}")
        };

        let truth_class = if wav_suggested_mark == "no_mark" {
            "SCORE_ONLY_BLOCKED_BY_WAV_TRUTH".to_string()
        } else if score_wav_relation == "score_wav_match" && wav_confidence_milli >= 1150 {
            "WAV_TRUTH_STRONG_MATCH".to_string()
        } else if score_wav_relation == "score_wav_match" {
            "WAV_TRUTH_MEDIUM_MATCH".to_string()
        } else {
            "WAV_TRUTH_REVIEW_DIFFERENT_MARK".to_string()
        };

        let (decision, _recommendation) = if row.mark == "accent" && wav_suggested_mark == "accent" {
            audit.wav_confirmed_accent += 1;
            ("WAV_ACCENT_PLAUSIBLE".to_string(), "Akzent durch WAV-Attack/Energie plausibel; audit-only, noch kein Write".to_string())
        } else if row.mark == "tenuto" && wav_suggested_mark == "tenuto" {
            audit.wav_confirmed_tenuto += 1;
            ("WAV_TENUTO_PLAUSIBLE".to_string(), "Tenuto durch WAV-Sustain/Decay plausibel; audit-only, noch kein Write".to_string())
        } else if row.mark == "staccato" && wav_suggested_mark == "staccato" {
            audit.wav_confirmed_staccato += 1;
            ("WAV_STACCATO_PLAUSIBLE".to_string(), "Staccato durch WAV-Attack und kurzen Release plausibel; audit-only, noch kein Write".to_string())
        } else if wav_suggested_mark != "no_mark" {
            audit.wav_review += 1;
            ("WAV_SUGGESTS_DIFFERENT_ARTICULATION".to_string(), format!("WAV zeigt Artikulationsgestik, aber eher {} als {}; spaeter Kandidatentyp/Placement pruefen", wav_suggested_mark, row.mark))
        } else {
            audit.wav_rejected_score_only += 1;
            ("SCORE_ONLY_BLOCKED_BY_WAV_TRUTH".to_string(), "Score-/Gestenlogik allein reicht nicht; kein Artikulationswrite".to_string())
        };

        let _ = decision;
        audit.rows.push(WavTruthArticulationRow {
            scope: row.scope,
            onset: row.onset,
            wav_energy_ratio_milli: energy_ratio_milli,
            wav_attack_ratio_milli: attack_ratio_milli,
            wav_confidence_milli,
            truth_class,
        });
    }

    audit.rows.sort_by(|a, b| {
        let ar = match a.truth_class.as_str() {
            "WAV_TRUTH_STRONG_MATCH" => 0,
            "WAV_TRUTH_MEDIUM_MATCH" => 1,
            "WAV_TRUTH_REVIEW_DIFFERENT_MARK" => 2,
            _ => 3,
        };
        let br = match b.truth_class.as_str() {
            "WAV_TRUTH_STRONG_MATCH" => 0,
            "WAV_TRUTH_MEDIUM_MATCH" => 1,
            "WAV_TRUTH_REVIEW_DIFFERENT_MARK" => 2,
            _ => 3,
        };
        ar.cmp(&br)
            .then_with(|| b.wav_confidence_milli.cmp(&a.wav_confidence_milli))
            .then_with(|| b.wav_attack_ratio_milli.cmp(&a.wav_attack_ratio_milli))
            .then_with(|| b.wav_energy_ratio_milli.cmp(&a.wav_energy_ratio_milli))
            .then_with(|| a.scope.cmp(&b.scope))
            .then_with(|| a.onset.cmp(&b.onset))
    });

    audit.write_ready = 0;
    audit.verdict = if audit.wav_confirmed_accent + audit.wav_confirmed_tenuto + audit.wav_confirmed_staccato > 0 {
        "wav_truth_candidates_plausible_but_audit_only_no_write".to_string()
    } else {
        "wav_truth_blocks_score_only_articulation_no_write".to_string()
    };
    audit.done_fragment = format!(
        "WavTruthArticulationFeatureLedgerMaterializedNoMutation: WAV decoded ja; ScoreKandidaten {}; WAVChecked {}; WAV-vorgeschlagen Accent/Tenuto/Staccato {}/{}/{}; ScoreWavMatch/Mismatch {}/{}; TruthStrong/Medium/Blocked {}/{}/{}; plausible Accent/Tenuto/Staccato {}/{}/{}; Review {}; ScoreOnlyBlocked {}; WriteReady 0; RenderXmlWrite nein; MusicMutation nein",
        audit.candidate_count,
        audit.wav_checked_candidates,
        audit.wav_suggested_accent,
        audit.wav_suggested_tenuto,
        audit.wav_suggested_staccato,
        audit.score_wav_match,
        audit.score_wav_mismatch,
        audit.wav_truth_strong,
        audit.wav_truth_medium,
        audit.wav_truth_blocked,
        audit.wav_confirmed_accent,
        audit.wav_confirmed_tenuto,
        audit.wav_confirmed_staccato,
        audit.wav_review,
        audit.wav_rejected_score_only
    );
    audit.pipeline_line = format!(
        "WavTruthArticulationFeatureLedgerMaterializedNoMutation v{}: Quelle {}; WAV {}; decoded ja; windows {}; ReferenceEnergy/Attack/Flux {}/{}/{}; ScoreKandidaten {}; WAVChecked {}; WAVSuggested Accent/Tenuto/Staccato {}/{}/{}; ScoreWavMatch/Mismatch {}/{}; TruthStrong/Medium/Blocked {}/{}/{}; plausible Accent/Tenuto/Staccato {}/{}/{}; Review {}; ScoreOnlyBlocked {}; WriteReady 0; RenderXmlWrite nein; MusicXmlWrite nein; PdfWrite nein; MasterApply nein; P3/P4 nein; MXL nein; MusicMutation nein; Urteil {}.",
        env!("CARGO_PKG_VERSION"),
        source_label,
        wav.path.display(),
        audit.wav_windows,
        audit.wav_reference_energy_milli,
        audit.wav_reference_attack_milli,
        audit.wav_reference_flux_milli,
        audit.candidate_count,
        audit.wav_checked_candidates,
        audit.wav_suggested_accent,
        audit.wav_suggested_tenuto,
        audit.wav_suggested_staccato,
        audit.score_wav_match,
        audit.score_wav_mismatch,
        audit.wav_truth_strong,
        audit.wav_truth_medium,
        audit.wav_truth_blocked,
        audit.wav_confirmed_accent,
        audit.wav_confirmed_tenuto,
        audit.wav_confirmed_staccato,
        audit.wav_review,
        audit.wav_rejected_score_only,
        audit.verdict
    );
    audit
}









