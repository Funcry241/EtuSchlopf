// v0.10.728 real report writer ownership module; no wrapper/include shell.
// --- mechanical module boundary: former src/report.rs ---
use crate::formal_green_contract::{format_result_line, format_summary_line};
use crate::model::{CheckReport, Issue, Severity};
use crate::rule_id::{
    priority_score_for_issue_type, repair_hint_for_issue_type,
    validator_issue_authority_invariant_status,
};
use crate::StaticNamedReportLineV766;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PdfFileFingerprintReport {
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Clone)]
pub struct MusicXmlArtifactLocatorReport {
    pub master: PathBuf,
    pub render: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ReportOptions {
    pub top: usize,
    pub pdf_file_fingerprint: Option<PdfFileFingerprintReport>,
    pub musicxml_artifact_locators: Option<MusicXmlArtifactLocatorReport>,
}

impl Default for ReportOptions {
    fn default() -> Self {
        Self {
            top: 12,
            pdf_file_fingerprint: None,
            musicxml_artifact_locators: None,
        }
    }
}





fn append_wav_pitch_external_sensor_benchmark(out: &mut String, report: &CheckReport) {
    let Some(audit) = crate::audit_only::wav_pitch_external_sensor_benchmark::build_for_score_title(
        report.score_title.as_deref(),
    ) else {
        return;
    };
    out.push_str("WAV-PITCH-EXTERNAL-SENSOR-BENCHMARK-AUDIT\n");
    out.push_str("=========================================\n");
    out.push_format_args(format_args!(
        "ChangeClass: AuditOnly; ContractSchema {}; Authority {}; Sensor {} {}; Observed {}; Reason {}.\n",
        audit.contract_schema,
        audit.authority,
        audit.sensor_identity,
        audit.sensor_version,
        if audit.observed { "ja" } else { "nein" },
        audit.reason,
    ));
    out.push_format_args(format_args!(
        "Provenance: PackageFingerprint {}; ModelFingerprint {}; FeatureSchema {}; SourceWavFingerprint {}; SensorOutputFreeze {}; OriginalFrameMapping {}.\n",
        audit.package_fingerprint_sha256,
        audit.model_fingerprint_sha256,
        audit.feature_schema,
        audit.source_wav_sha256,
        audit.freeze_sha256,
        audit.original_frame_mapping,
    ));
    out.push_format_args(format_args!(
        "Runtime: WindowsOffline {}; SetupNetworkUsed {}; DeterministicRepeat {}.\n",
        audit.windows_offline_runtime,
        audit.setup_network_used,
        audit.deterministic_repeat,
    ));
    out.push_format_args(format_args!(
        "Evidence80ms: SensorEvents {}; OracleEvents {}; Precision {}.{:01}%; Recall {}.{:01}%; F1 {}.{:01}%; ChordToneRecall {}.{:01}%; RearticulationBoth {}.{:01}%; MultiplicityRecall {}.{:01}%; BassRecall {}.{:01}%; HighRecall {}.{:01}%.\n",
        audit.sensor_events,
        audit.oracle_events,
        audit.precision_80_permille / 10,
        audit.precision_80_permille % 10,
        audit.recall_80_permille / 10,
        audit.recall_80_permille % 10,
        audit.f1_80_permille / 10,
        audit.f1_80_permille % 10,
        audit.chord_tone_recall_80_permille / 10,
        audit.chord_tone_recall_80_permille % 10,
        audit.rearticulation_both_80_permille / 10,
        audit.rearticulation_both_80_permille % 10,
        audit.multiplicity_recall_80_permille / 10,
        audit.multiplicity_recall_80_permille % 10,
        audit.bass_recall_80_permille / 10,
        audit.bass_recall_80_permille % 10,
        audit.high_recall_80_permille / 10,
        audit.high_recall_80_permille % 10,
    ));
    out.push_format_args(format_args!(
        "FalseSensorEvents80: {}; HarmonicExplainableFalse80: {}; MidiAuthority {}; CanonicalSourceEventPromotion {}.\n",
        audit.false_sensor_events_80,
        audit.harmonic_explainable_false_80,
        audit.midi_authority,
        audit.canonical_source_event_promotion,
    ));
    out.push_str("OrderingContract: WAV-only quarter-frame timing and WAV fingerprint are passed in-memory; external sensor projection is frozen before this benchmark may read legacy source-oracle evidence from productive MIDI. No result.txt dependency. ProductiveSourceAuthority MIDI; StaffHandAuthority none; PhraseAuthority unchanged; WriterAuthority none.\n");
    out.push_format_args(format_args!(
        "PieceGate: {}. Corpus GO/STOP is intentionally deferred until the accepted single-project gate is followed by the normal full-corpus run.\n\n",
        if audit.observed {
            "EXTERNAL SENSOR PIECE EVIDENCE OBSERVED"
        } else {
            "EXTERNAL SENSOR PIECE EVIDENCE NOT OBSERVED"
        },
    ));
}

fn append_wav_pitch_sensor_qualification_boundary(out: &mut String, report: &CheckReport) {
    let Some(audit) = crate::audit_only::wav_pitch_sensor_qualification::build_for_score_title(
        report.score_title.as_deref(),
    ) else {
        return;
    };
    let contract = audit.contract;
    out.push_str("WAV-PITCH-SENSOR-QUALIFICATION-BOUNDARY-AUDIT\n");
    out.push_str("================================================\n");
    out.push_format_args(format_args!(
        "ChangeClass: AuditOnly; ContractSchema {}; Authority {}; WavPresent {}; ExternalSensorRuntimePresent {}; ProductiveSensorSelected {}.\n",
        contract.schema,
        contract.authority.as_str(),
        if audit.wav_present { "ja" } else { "nein" },
        if audit.external_sensor_runtime_present { "ja" } else { "nein" },
        if audit.productive_sensor_selected { "ja" } else { "nein" },
    ));
    out.push_format_args(format_args!(
        "RequiredProvenance: SensorIdentity {}; SensorVersion {}; ModelFingerprint {}; FeatureSchema {}; SourceWavFingerprint {}; DerivedViewIdentity {}; OriginalFrameMapping {}.\n",
        yes_no(contract.requires_sensor_identity),
        yes_no(contract.requires_sensor_version),
        yes_no(contract.requires_model_fingerprint),
        yes_no(contract.requires_feature_schema),
        yes_no(contract.requires_source_wav_fingerprint),
        yes_no(contract.requires_derived_view_identity),
        yes_no(contract.requires_original_frame_mapping),
    ));
    out.push_format_args(format_args!(
        "RuntimeQualification: WindowsOffline {}; DeterministicDecisionOutput {}; NetworkDependencyForbidden {}.\n",
        yes_no(contract.requires_windows_offline_runtime),
        yes_no(contract.requires_deterministic_decision_output),
        yes_no(contract.forbids_network_dependency),
    ));
    out.push_format_args(format_args!(
        "AuthorityProhibitions: MidiAuthority {}; SourceEventPromotion {}; StaffHandDecision {}; PhraseChange {}; WriterPath {}.\n",
        yes_no(contract.forbids_midi_authority),
        yes_no(contract.forbids_source_event_promotion),
        yes_no(contract.forbids_staff_hand_decision),
        yes_no(contract.forbids_phrase_change),
        yes_no(contract.forbids_writer_path),
    ));
    out.push_format_args(format_args!(
        "BoundaryGate: {}; external candidates may now be benchmarked behind this frozen evidence-only contract; ProductiveSensorSelected remains false until a separate authority decision.\n\n",
        if contract.closed() {
            "WAV PITCH SENSOR QUALIFICATION BOUNDARY READY"
        } else {
            "WAV PITCH SENSOR QUALIFICATION BOUNDARY NOT READY"
        },
    ));
}

fn yes_no(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}

fn append_wav_observation_legacy_oracle_qualification(out: &mut String, report: &CheckReport) {
    let Some(audit) = crate::audit_only::wav_observation_legacy_oracle::build_for_score_title(
        report.score_title.as_deref(),
    ) else {
        return;
    };
    let wav = &audit.wav;
    let legacy = &audit.legacy;
    out.push_str("WAV-OBSERVATION-LEGACY-ORACLE-QUALIFICATION\n");
    out.push_str("===========================================\n");
    out.push_str("ChangeClass: AuditOnly; ProductiveSourceAuthority MIDI; WavPerception pitch/metric/source-event authority none; WriterAuthority none.\n");
    out.push_format_args(format_args!(
        "WavObservation: Qualified {}; Reason {}; FileSha256 {}; DecodedSignal {}; Format {}; SampleRate {}; Channels {}; BitsPerSample {}; BlockAlign {}; OriginalFrames {}; DecodedSampleValues {}; TrailingPartialDataBytes {}; FirstNonzeroFrame {}; LastNonzeroFrame {}.\n",
        if wav.qualified { "ja" } else { "nein" },
        wav.reason,
        wav.source_file_sha256,
        wav.decoded_signal_fnv64,
        wav.audio_format,
        wav.sample_rate,
        wav.channels,
        wav.bits_per_sample,
        wav.block_align,
        wav.original_frame_count,
        wav.decoded_sample_values,
        wav.trailing_partial_data_bytes,
        wav.first_nonzero_frame
            .map_or_else(|| "none".to_string(), |index| index.0.to_string()),
        wav.last_nonzero_frame
            .map_or_else(|| "none".to_string(), |index| index.0.to_string()),
    ));
    out.push_str("PhysicalTimeContract: OriginalFrameIndex is the original WAV frame ordinal; seconds are derived only. Original channels remain observable; legacy mono averaging is a derived consumer view and owns no source truth.\n");
    out.push_format_args(format_args!(
        "FrozenLegacySourceOracle: Qualified {}; Reason {}; SourcePPQ {}; CanonicalDivisions {}; Events {}; DuplicatePitchOnsetEvents {}; TerminalExtentTicks {}; EventFingerprintSha256 {}.\n",
        if legacy.qualified { "ja" } else { "nein" },
        legacy.reason,
        legacy.source_ppq,
        legacy.canonical_divisions,
        legacy.event_count,
        legacy.duplicate_pitch_onset_events,
        legacy.terminal_extent_ticks,
        legacy.event_fingerprint_sha256,
    ));
    out.push_format_args(format_args!(
        "Gate: {}. No model, MIDI authority, metric reconstruction, source-event promotion, phrase change or audio-to-writer path introduced.\n\n",
        if wav.qualified && legacy.qualified {
            "WAV OBSERVATION QUALIFIED"
        } else {
            "WAV OBSERVATION NOT QUALIFIED"
        },
    ));
}


fn append_wav_attack_pulse_metric_reconstruction(out: &mut String, report: &CheckReport) {
    let Some(audit) = crate::audit_only::wav_attack_pulse_metric::build_for_score_title(
        report.score_title.as_deref(),
    ) else {
        return;
    };
    out.push_str("WAV-ATTACK-PULSE-METRIC-RECONSTRUCTION-AUDIT\n");
    out.push_str("============================================\n");
    out.push_str("ChangeClass: AuditOnly; audio hypotheses, zero-origin grid and local PerformanceTimeMap are frozen before FrozenLegacySourceOracle loading; the oracle is backed by productive MIDI and participates in evaluation only, never in attack, tempo, phase, timing-residual or metric hypothesis generation. PitchAuthority none; ProductiveSourceAuthority MIDI; WriterAuthority none.\n");
    out.push_format_args(format_args!(
        "AudioEvidence: SampleRate {}; HopFrames {}; EnvelopeBins {}; EnergyFamilyPeaks {}; TransientFamilyPeaks {}; AttackCandidates {}; DualFamilyAttackCandidates {}; PulseAttackCandidates {}; AudioMetricHypotheses {}.\n",
        audit.sample_rate,
        audit.hop_frames,
        audit.envelope_bins,
        audit.energy_family_peaks,
        audit.transient_family_peaks,
        audit.attack_candidates,
        audit.dual_family_attack_candidates,
        audit.pulse_attack_candidates,
        audit.audio_hypothesis_count,
    ));
    for hypothesis in &audit.audio_top_hypotheses {
        let bpm_whole = hypothesis.bpm_milli / 1_000;
        let bpm_fraction = hypothesis.bpm_milli % 1_000;
        out.push_format_args(format_args!(
            "AudioTop#{rank}: TempoFamilyRank {tempo_rank}; PhaseFamilyRank {phase_rank}; QuarterFrames {quarter}; BPM {bpm_whole}.{bpm_fraction:03}; OriginFrame {origin}; TempoVotes {tempo_votes}; PhaseVotes {phase_votes}.\n",
            rank = hypothesis.audio_rank,
            tempo_rank = hypothesis.tempo_family_rank,
            phase_rank = hypothesis.phase_family_rank,
            quarter = hypothesis.quarter_frames,
            origin = hypothesis.origin_frame,
            tempo_votes = hypothesis.tempo_votes,
            phase_votes = hypothesis.phase_votes,
        ));
    }
    for family in &audit.audio_tempo_families {
        let bpm_whole = family.bpm_milli / 1_000;
        let bpm_fraction = family.bpm_milli % 1_000;
        out.push_format_args(format_args!(
            "AudioTempoFamily#{rank}: QuarterFrames {quarter}; BPM {bpm_whole}.{bpm_fraction:03}; TempoVotes {tempo_votes}.\n",
            rank = family.tempo_family_rank,
            quarter = family.quarter_frames,
            tempo_votes = family.tempo_votes,
        ));
    }
    out.push_str("ZeroOriginPolicy: LegacySourceParityPolicy frame0_is_project_time0; OriginFrame 0; existing 4/4 and 96-ticks-per-quarter source contract retained for Phase A. FirstNonzeroFrame is observed evidence only and never replaces frame 0 as project-time origin.\n");
    for family in &audit.zero_origin_grid_families {
        let bpm_whole = family.bpm_milli / 1_000;
        let bpm_fraction = family.bpm_milli % 1_000;
        let grid_whole = family.grid_alignment_permille / 10;
        let grid_fraction = family.grid_alignment_permille % 10;
        let dual_whole = family.dual_grid_alignment_permille / 10;
        let dual_fraction = family.dual_grid_alignment_permille % 10;
        let median_whole = family.median_grid_residual_permille / 10;
        let median_fraction = family.median_grid_residual_permille % 10;
        let first_nonzero = family
            .first_nonzero_frame
            .map_or_else(|| "none".to_string(), |frame| frame.to_string());
        let first_nonzero_residual_ms =
            crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
                audit.sample_rate,
                family.first_nonzero_grid_residual_frames,
            );
        out.push_format_args(format_args!(
            "AudioZeroOriginGrid#{grid_rank}: CoarseTempoFamilyRank {coarse_rank}; CoarseQuarterFrames {coarse_quarter}; RefinedQuarterFrames {refined_quarter}; BPM {bpm_whole}.{bpm_fraction:03}; TempoVotes {tempo_votes}; GridAlignment {grid_whole}.{grid_fraction}%; DualGridAlignment {dual_whole}.{dual_fraction}%; MedianGridResidualNorm {median_whole}.{median_fraction}%; FirstNonzeroFrame {first_nonzero}; FirstNonzeroGridResidualMs {first_nonzero_residual_ms}.\n",
            grid_rank = family.grid_rank,
            coarse_rank = family.coarse_tempo_family_rank,
            coarse_quarter = family.coarse_quarter_frames,
            refined_quarter = family.refined_quarter_frames,
            tempo_votes = family.tempo_votes,
            first_nonzero = first_nonzero,
            first_nonzero_residual_ms = first_nonzero_residual_ms,
        ));
    }
    out.push_format_args(format_args!(
        "OracleEvaluation: UniqueLegacyOnsets {}; evaluation starts only after audio hypothesis freeze. Global timeline-span coherence and ordered one-to-one matches are evaluation evidence only.\n",
        audit.oracle_unique_onsets,
    ));
    for (index, evaluation) in audit.best_evaluations.iter().enumerate() {
        let bpm_whole = evaluation.bpm_milli / 1_000;
        let bpm_fraction = evaluation.bpm_milli % 1_000;
        let match_10 = percent_tenths(evaluation.match_10, evaluation.oracle_unique_onsets);
        let match_20 = percent_tenths(evaluation.match_20, evaluation.oracle_unique_onsets);
        let match_40 = percent_tenths(evaluation.match_40, evaluation.oracle_unique_onsets);
        let match_80 = percent_tenths(evaluation.match_80, evaluation.oracle_unique_onsets);
        let attack_explained = percent_tenths(
            evaluation.attack_explained_40,
            evaluation.attack_candidates,
        );
        let ordered_40 = percent_tenths(
            evaluation.ordered_match_40,
            evaluation.oracle_unique_onsets,
        );
        let ordered_80 = percent_tenths(
            evaluation.ordered_match_80,
            evaluation.oracle_unique_onsets,
        );
        let span_ratio_whole = evaluation.span_ratio_permille / 10;
        let span_ratio_fraction = evaluation.span_ratio_permille % 10;
        let median_ms = crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
            audit.sample_rate,
            evaluation.median_residual_frames,
        );
        let p90_ms = crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
            audit.sample_rate,
            evaluation.p90_residual_frames,
        );
        out.push_format_args(format_args!(
            "Evaluation#{rank}: AudioRank {audio_rank}; TempoFamilyRank {tempo_family_rank}; PhaseFamilyRank {phase_family_rank}; QuarterFrames {quarter}; BPM {bpm_whole}.{bpm_fraction:03}; OriginFrame {origin}; TempoVotes {tempo_votes}; PhaseVotes {phase_votes}; Match10 {m10_whole}.{m10_fraction}% ({m10_count}/{onsets}); Match20 {m20_whole}.{m20_fraction}% ({m20_count}/{onsets}); Match40 {m40_whole}.{m40_fraction}% ({m40_count}/{onsets}); Match80 {m80_whole}.{m80_fraction}% ({m80_count}/{onsets}); Ordered40 {o40_whole}.{o40_fraction}% ({o40_count}/{onsets}); Ordered80 {o80_whole}.{o80_fraction}% ({o80_count}/{onsets}); MedianResidualMs {median_ms}; P90ResidualMs {p90_ms}; AttackExplained40 {ax_whole}.{ax_fraction}% ({ax_count}/{attacks}); ProjectedLegacySpanFrames {projected_span}; PulseAttackSpanFrames {attack_span}; SpanRatio {span_ratio_whole}.{span_ratio_fraction}%; SpanCoherent {span_coherent}; OracleDiscoverable {ready}.\n",
            rank = index.saturating_add(1),
            audio_rank = evaluation.audio_rank,
            tempo_family_rank = evaluation.tempo_family_rank,
            phase_family_rank = evaluation.phase_family_rank,
            quarter = evaluation.quarter_frames,
            origin = evaluation.origin_frame,
            tempo_votes = evaluation.tempo_votes,
            phase_votes = evaluation.phase_votes,
            m10_whole = match_10.0,
            m10_fraction = match_10.1,
            m10_count = evaluation.match_10,
            m20_whole = match_20.0,
            m20_fraction = match_20.1,
            m20_count = evaluation.match_20,
            m40_whole = match_40.0,
            m40_fraction = match_40.1,
            m40_count = evaluation.match_40,
            m80_whole = match_80.0,
            m80_fraction = match_80.1,
            m80_count = evaluation.match_80,
            onsets = evaluation.oracle_unique_onsets,
            o40_whole = ordered_40.0,
            o40_fraction = ordered_40.1,
            o40_count = evaluation.ordered_match_40,
            o80_whole = ordered_80.0,
            o80_fraction = ordered_80.1,
            o80_count = evaluation.ordered_match_80,
            ax_whole = attack_explained.0,
            ax_fraction = attack_explained.1,
            ax_count = evaluation.attack_explained_40,
            attacks = evaluation.attack_candidates,
            projected_span = evaluation.projected_legacy_span_frames,
            attack_span = evaluation.pulse_attack_span_frames,
            span_ratio_whole = span_ratio_whole,
            span_ratio_fraction = span_ratio_fraction,
            span_coherent = if evaluation.span_coherent { "ja" } else { "nein" },
            ready = if evaluation.ready { "ja" } else { "nein" },
        ));
    }
    for evaluation in &audit.audio_identifiability_evaluations {
        let bpm_whole = evaluation.bpm_milli / 1_000;
        let bpm_fraction = evaluation.bpm_milli % 1_000;
        let match_40 = percent_tenths(evaluation.match_40, evaluation.oracle_unique_onsets);
        let match_80 = percent_tenths(evaluation.match_80, evaluation.oracle_unique_onsets);
        let ordered_40 =
            percent_tenths(evaluation.ordered_match_40, evaluation.oracle_unique_onsets);
        let ordered_80 =
            percent_tenths(evaluation.ordered_match_80, evaluation.oracle_unique_onsets);
        let span_ratio_whole = evaluation.span_ratio_permille / 10;
        let span_ratio_fraction = evaluation.span_ratio_permille % 10;
        let median_ms = crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
            audit.sample_rate,
            evaluation.median_residual_frames,
        );
        let p90_ms = crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
            audit.sample_rate,
            evaluation.p90_residual_frames,
        );
        out.push_format_args(format_args!(
            "AudioIdentifiabilityEvaluation#{grid_rank}: CoarseTempoFamilyRank {tempo_rank}; RefinedQuarterFrames {quarter}; BPM {bpm_whole}.{bpm_fraction:03}; OriginFrame 0; Match40 {m40_whole}.{m40_fraction}% ({m40_count}/{onsets}); Match80 {m80_whole}.{m80_fraction}% ({m80_count}/{onsets}); Ordered40 {o40_whole}.{o40_fraction}% ({o40_count}/{onsets}); Ordered80 {o80_whole}.{o80_fraction}% ({o80_count}/{onsets}); MedianResidualMs {median_ms}; P90ResidualMs {p90_ms}; SpanRatio {span_ratio_whole}.{span_ratio_fraction}%; OracleCompatible {ready}.\n",
            grid_rank = evaluation.audio_rank,
            tempo_rank = evaluation.tempo_family_rank,
            quarter = evaluation.quarter_frames,
            m40_whole = match_40.0,
            m40_fraction = match_40.1,
            m40_count = evaluation.match_40,
            m80_whole = match_80.0,
            m80_fraction = match_80.1,
            m80_count = evaluation.match_80,
            onsets = evaluation.oracle_unique_onsets,
            o40_whole = ordered_40.0,
            o40_fraction = ordered_40.1,
            o40_count = evaluation.ordered_match_40,
            o80_whole = ordered_80.0,
            o80_fraction = ordered_80.1,
            o80_count = evaluation.ordered_match_80,
            span_ratio_whole = span_ratio_whole,
            span_ratio_fraction = span_ratio_fraction,
            ready = if evaluation.ready { "ja" } else { "nein" },
        ));
    }
    if let Some(map) = audit.performance_time_map {
        let median_offset_ms =
            crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
                audit.sample_rate,
                map.median_abs_offset_frames,
            );
        let p90_offset_ms =
            crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
                audit.sample_rate,
                map.p90_abs_offset_frames,
            );
        let max_offset_ms =
            crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
                audit.sample_rate,
                map.max_abs_offset_frames,
            );
        out.push_format_args(format_args!(
            "AudioPerformanceTimeMap: QuarterFrames {quarter}; WindowQuarters {window_quarters}; SupportedWindows {supported}/{total}; DualSupportedWindows {dual_supported}; Nodes {nodes}; Monotone {monotone}; MedianAbsOffsetMs {median_offset_ms}; P90AbsOffsetMs {p90_offset_ms}; MaxAbsOffsetMs {max_offset_ms}; FirstOffsetFrames {first_offset}; LastOffsetFrames {last_offset}.\n",
            quarter = map.quarter_frames,
            window_quarters = map.window_quarters,
            supported = map.supported_windows,
            total = map.total_windows,
            dual_supported = map.dual_supported_windows,
            nodes = map.node_count,
            monotone = if map.monotone { "ja" } else { "nein" },
            first_offset = map.first_offset_frames,
            last_offset = map.last_offset_frames,
        ));
    }
    if let Some(evaluation) = audit.performance_time_map_evaluation {
        let match_40 = percent_tenths(evaluation.match_40, evaluation.oracle_unique_onsets);
        let match_80 = percent_tenths(evaluation.match_80, evaluation.oracle_unique_onsets);
        let ordered_40 =
            percent_tenths(evaluation.ordered_match_40, evaluation.oracle_unique_onsets);
        let ordered_80 =
            percent_tenths(evaluation.ordered_match_80, evaluation.oracle_unique_onsets);
        let median_ms = crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
            audit.sample_rate,
            evaluation.median_residual_frames,
        );
        let p90_ms = crate::audit_only::wav_attack_pulse_metric::frames_to_milliseconds(
            audit.sample_rate,
            evaluation.p90_residual_frames,
        );
        let span_ratio_whole = evaluation.span_ratio_permille / 10;
        let span_ratio_fraction = evaluation.span_ratio_permille % 10;
        out.push_format_args(format_args!(
            "PerformanceTimeMapEvaluation: Match40 {m40_whole}.{m40_fraction}% ({m40_count}/{onsets}); Match80 {m80_whole}.{m80_fraction}% ({m80_count}/{onsets}); Ordered40 {o40_whole}.{o40_fraction}% ({o40_count}/{onsets}); Ordered80 {o80_whole}.{o80_fraction}% ({o80_count}/{onsets}); MedianResidualMs {median_ms}; P90ResidualMs {p90_ms}; ProjectedSpanFrames {projected_span}; PulseAttackSpanFrames {attack_span}; SpanRatio {span_ratio_whole}.{span_ratio_fraction}%; OracleCompatible {ready}.\n",
            m40_whole = match_40.0,
            m40_fraction = match_40.1,
            m40_count = evaluation.match_40,
            m80_whole = match_80.0,
            m80_fraction = match_80.1,
            m80_count = evaluation.match_80,
            onsets = evaluation.oracle_unique_onsets,
            o40_whole = ordered_40.0,
            o40_fraction = ordered_40.1,
            o40_count = evaluation.ordered_match_40,
            o80_whole = ordered_80.0,
            o80_fraction = ordered_80.1,
            o80_count = evaluation.ordered_match_80,
            projected_span = evaluation.projected_span_frames,
            attack_span = evaluation.pulse_attack_span_frames,
            span_ratio_whole = span_ratio_whole,
            span_ratio_fraction = span_ratio_fraction,
            ready = if evaluation.ready { "ja" } else { "nein" },
        ));
    }
    out.push_format_args(format_args!(
        "PerformanceTimeMapGate: {}. The map is generated from WAV-only pulse/grid residuals before oracle loading; Canonical Score Time remains unchanged and the map stores performance timing residual only.\n",
        if audit.performance_time_map_identifiable {
            "WAV PERFORMANCE-TIME-MAP AUDIO-IDENTIFIABILITY READY"
        } else {
            "WAV PERFORMANCE-TIME-MAP AUDIO-IDENTIFIABILITY NOT READY"
        },
    ));
    out.push_format_args(format_args!(
        "QualificationContract: SpanRatio 80.0%-125.0%; Match40 >= 75%; Match80 >= 90%; MedianResidual <= 25 ms; P90Residual <= 80 ms; at least 8 unique legacy onsets. Ordered40/80 remain diagnostic. r152 evaluates the already-selected audio ZeroOriginGrid#1 without oracle re-ranking; r154 additionally evaluates an audio-only local PerformanceTimeMap built from that frozen grid before oracle loading.\nOracleDiscoverabilityGate: {}; Reason {}.\nAudioIdentifiabilityGate: {}. This gate is READY only when the audio-ranked ZeroOriginGrid#1 itself satisfies the unchanged oracle-compatibility contract; The legacy source oracle cannot choose or reorder it. No pitch reconstruction, source-event promotion, phrase change, renderer mutation or audio-to-writer path introduced.\n\n",
        if audit.qualified {
            "WAV METRIC ORACLE-DISCOVERABILITY READY"
        } else {
            "WAV METRIC ORACLE-DISCOVERABILITY NOT READY"
        },
        audit.reason,
        if audit.audio_identifiable {
            "WAV METRIC AUDIO-IDENTIFIABILITY READY"
        } else {
            "WAV METRIC AUDIO-IDENTIFIABILITY NOT READY"
        },
    ));
}


fn append_wav_pitch_harmonic_evidence_probe(out: &mut String, report: &CheckReport) {
    let Some(audit) = crate::audit_only::wav_pitch_harmonic_evidence::build_for_score_title(
        report.score_title.as_deref(),
    ) else {
        return;
    };
    out.push_str("WAV-PITCH-HARMONIC-EVIDENCE-PROBE-AUDIT\n");
    out.push_str("=======================================\n");
    out.push_str("ChangeClass: AuditOnly; bounded attack probes and all 88-key spectral presence/attack-rise rankings are frozen from WAV before FrozenLegacySourceOracle loading. The oracle evaluates ranks only; PitchAuthority none; CanonicalSourceEventPromotion none; ProductiveSourceAuthority MIDI; WriterAuthority none.\n");
    out.push_format_args(format_args!(
        "AudioPitchProbe: SampleRate {}; QuarterFrames {}; AudioAttackCandidates {}; ProbeAttacks {}; DualProbeAttacks {}; PositiveRisePitchRows {}; Top8HarmonicParentRatioMedian {}.{:01}%.\n",
        audit.sample_rate,
        audit.quarter_frames,
        audit.audio_attack_candidates,
        audit.probe_attacks,
        audit.dual_probe_attacks,
        audit.positive_rise_pitch_rows,
        audit.top8_harmonic_parent_ratio_median_permille / 10,
        audit.top8_harmonic_parent_ratio_median_permille % 10,
    ));

    let top1 = percent_tenths(audit.rise_recall_top1, audit.oracle_pitch_instances_evaluated);
    let top3 = percent_tenths(audit.rise_recall_top3, audit.oracle_pitch_instances_evaluated);
    let top5 = percent_tenths(audit.rise_recall_top5, audit.oracle_pitch_instances_evaluated);
    let top8 = percent_tenths(audit.rise_recall_top8, audit.oracle_pitch_instances_evaluated);
    let presence8 =
        percent_tenths(audit.presence_recall_top8, audit.oracle_pitch_instances_evaluated);
    let false_harmonic =
        percent_tenths(audit.top8_false_harmonic_explainable, audit.top8_false_candidates);
    let rearticulation = percent_tenths(
        audit.same_pitch_rearticulation_both_top8,
        audit.same_pitch_rearticulation_pairs,
    );
    out.push_format_args(format_args!(
        "OracleEvaluationProbeConditioned: MatchedProbeAttacks {matched}/{probes}; OraclePitchInstances {pitch_instances}; RiseRecallTop1 {t1w}.{t1f}% ({t1}/{pitch_instances}); Top3 {t3w}.{t3f}% ({t3}/{pitch_instances}); Top5 {t5w}.{t5f}% ({t5}/{pitch_instances}); Top8 {t8w}.{t8f}% ({t8}/{pitch_instances}); PresenceRecallTop8 {p8w}.{p8f}% ({p8}/{pitch_instances}).\n",
        matched = audit.matched_probe_attacks,
        probes = audit.probe_attacks,
        pitch_instances = audit.oracle_pitch_instances_evaluated,
        t1w = top1.0,
        t1f = top1.1,
        t1 = audit.rise_recall_top1,
        t3w = top3.0,
        t3f = top3.1,
        t3 = audit.rise_recall_top3,
        t5w = top5.0,
        t5f = top5.1,
        t5 = audit.rise_recall_top5,
        t8w = top8.0,
        t8f = top8.1,
        t8 = audit.rise_recall_top8,
        p8w = presence8.0,
        p8f = presence8.1,
        p8 = audit.presence_recall_top8,
    ));
    out.push_format_args(format_args!(
        "HarmonicAndRearticulationEvaluation: Top8FalseCandidates {false_total}; FalseHarmonicExplainable {fhw}.{fhf}% ({false_harmonic_count}/{false_total}); SamePitchRearticulationPairs {re_pairs}; BothRecoveredTop8 {rew}.{refraction}% ({re_both}/{re_pairs}).\n",
        false_total = audit.top8_false_candidates,
        fhw = false_harmonic.0,
        fhf = false_harmonic.1,
        false_harmonic_count = audit.top8_false_harmonic_explainable,
        re_pairs = audit.same_pitch_rearticulation_pairs,
        rew = rearticulation.0,
        refraction = rearticulation.1,
        re_both = audit.same_pitch_rearticulation_both_top8,
    ));

    let complementarity_external = percent_tenths(
        audit.external_sensor_recovered_80,
        audit.complementarity_oracle_pitch_instances,
    );
    let complementarity_physical = percent_tenths(
        audit.rise_recall_top8,
        audit.complementarity_oracle_pitch_instances,
    );
    let complementarity_union = percent_tenths(
        audit.union_recovered_80,
        audit.complementarity_oracle_pitch_instances,
    );
    let high_external = percent_tenths(
        audit.high_external_recovered_80,
        audit.high_oracle_pitch_instances,
    );
    let high_physical = percent_tenths(
        audit.high_physical_recovered_80,
        audit.high_oracle_pitch_instances,
    );
    let high_union = percent_tenths(
        audit.high_union_recovered_80,
        audit.high_oracle_pitch_instances,
    );
    out.push_format_args(format_args!(
        "CrossViewComplementarity80: ExternalFrozen {external}; ProbeConditionedOraclePitchInstances {total}; BasicPitchRecovered {ew}.{ef}% ({external_count}/{total}); PhysicalRiseTop8Recovered {pw}.{pf}% ({physical_count}/{total}); UnionRecovered {uw}.{uf}% ({union_count}/{total}); BasicOnly {basic_only}; PhysicalOnly {physical_only}; Both {both}; Neither {neither}; HighOraclePitchInstances {high_total}; HighBasicPitchRecovered {hew}.{hef}% ({high_external_count}/{high_total}); HighPhysicalTop8Recovered {hpw}.{hpf}% ({high_physical_count}/{high_total}); HighUnionRecovered {huw}.{huf}% ({high_union_count}/{high_total}).\n",
        external = if audit.external_sensor_frozen_observed { "ja" } else { "nein" },
        total = audit.complementarity_oracle_pitch_instances,
        ew = complementarity_external.0,
        ef = complementarity_external.1,
        external_count = audit.external_sensor_recovered_80,
        pw = complementarity_physical.0,
        pf = complementarity_physical.1,
        physical_count = audit.rise_recall_top8,
        uw = complementarity_union.0,
        uf = complementarity_union.1,
        union_count = audit.union_recovered_80,
        basic_only = audit.external_only_recovered_80,
        physical_only = audit.physical_only_recovered_80,
        both = audit.both_recovered_80,
        neither = audit.neither_recovered_80,
        high_total = audit.high_oracle_pitch_instances,
        hew = high_external.0,
        hef = high_external.1,
        high_external_count = audit.high_external_recovered_80,
        hpw = high_physical.0,
        hpf = high_physical.1,
        high_physical_count = audit.high_physical_recovered_80,
        huw = high_union.0,
        huf = high_union.1,
        high_union_count = audit.high_union_recovered_80,
    ));
    out.push_str("CrossViewComplementarityGate: evaluation_only; Basic Pitch and physical Top-8 remain separately frozen evidence channels; union coverage grants no source-event, multiplicity, staff/hand, phrase or writer authority.\n");

    out.push_format_args(format_args!(
        "PitchEvidenceProbeGate: {}; Reason {}. This is a bounded sensor-qualification probe only: no universal confidence scalar, no pitch threshold, no MIDI authority, no source-event promotion, no staff/hand decision, no phrase change and no audio-to-writer path.\n\n",
        if audit.measured {
            "WAV PITCH/HARMONIC EVIDENCE PROBE MEASURED"
        } else {
            "WAV PITCH/HARMONIC EVIDENCE PROBE NOT MEASURED"
        },
        audit.reason,
    ));
}


fn percent_tenths(count: usize, total: usize) -> (usize, usize) {
    if total == 0 {
        return (0, 0);
    }
    let tenths = count.saturating_mul(1_000) / total;
    (tenths / 10, tenths % 10)
}

pub fn write_console_summary(report: &CheckReport, out_path: &std::path::Path) -> String {
    let mut out = String::new();
    out.push_format_args(format_args!("ETUSCHLOPF: {}\n", report.summary.status));
    out.push_str(&format_summary_line(
        report.summary.blockers,
        report.summary.warnings,
        report.summary.hints,
    ));
    out.push('\n');
    out.push_format_args(format_args!("Ergebnisdatei: {}", out_path.display()));
    out
}

// r1291: writes the wall time of one report section to stderr (console log only) and
// returns a fresh clock; never part of the report text.
fn report_section_timing(title: Option<&str>, section: &str, started: std::time::Instant) -> std::time::Instant {
    eprintln!("ETUSCHLOPF: PhaseTiming: project={}; section={section}; ms={}", title.unwrap_or_default(), started.elapsed().as_millis());
    std::time::Instant::now()
}

pub fn write_text_report(report: &CheckReport, options: &ReportOptions) -> String {
    let mut out = String::new();
    out.push_str("ETUSCHLOPF-ERGEBNIS\n");
    out.push_str("==================\n\n");
    out.push_format_args(format_args!("Werkzeug: {} {}\n", report.tool, report.version));
    if let Some(title) = report.score_title.as_ref() {
        out.push_format_args(format_args!("Partitur: {title}\n"));
    }
    out.push_str(&format_result_line(&report.summary.status.to_string()));
    out.push('\n');
    out.push_str(&format_summary_line(
        report.summary.blockers,
        report.summary.warnings,
        report.summary.hints,
    ));
    out.push('\n');
    out.push_format_args(format_args!("Ergebnismodus: ein Bericht | Top-Limit: {}\n", options.top));
    if let Some(fingerprint) = options.pdf_file_fingerprint.as_ref() {
        out.push_format_args(format_args!(
            "PDF-Datei-Fingerprint: PdfFileSha256 {}; PdfBytes {}; Scope file-bytes-metadata-sensitive.\n",
            fingerprint.sha256, fingerprint.bytes
        ));
    }
    out.push('\n');
    // r1291: wall time per report section on stderr only (console log).
    let mut section_clock = std::time::Instant::now();
    append_rule_severity_authority_audit(&mut out, report);
    section_clock = report_section_timing(report.score_title.as_deref(), "rule_severity_authority", section_clock);
    append_wav_pitch_external_sensor_benchmark(&mut out, report);
    section_clock = report_section_timing(report.score_title.as_deref(), "wav_pitch_external_sensor_benchmark", section_clock);
    append_wav_observation_legacy_oracle_qualification(&mut out, report);
    section_clock = report_section_timing(report.score_title.as_deref(), "wav_observation_legacy_oracle", section_clock);
    append_wav_attack_pulse_metric_reconstruction(&mut out, report);
    section_clock = report_section_timing(report.score_title.as_deref(), "wav_attack_pulse_metric", section_clock);
    append_wav_pitch_harmonic_evidence_probe(&mut out, report);
    section_clock = report_section_timing(report.score_title.as_deref(), "wav_pitch_harmonic_evidence", section_clock);
    append_wav_pitch_sensor_qualification_boundary(&mut out, report);
    section_clock = report_section_timing(report.score_title.as_deref(), "wav_pitch_sensor_boundary", section_clock);
    crate::audit_only::phrase_boundary_evidence::append_report_section(&mut out);
    section_clock = report_section_timing(report.score_title.as_deref(), "phrase_boundary_evidence", section_clock);
    let musicxml_generator_identity = options
        .musicxml_artifact_locators
        .as_ref()
        .map(|_| crate::audit_only::musicxml_generator_snapshot_identity::current());
    crate::audit_only::format40_conformance_foundation::append_report_section(
        &mut out,
        report.score_title.as_deref(),
        options
            .musicxml_artifact_locators
            .as_ref()
            .map(|locators| locators.master.as_path()),
        options
            .musicxml_artifact_locators
            .as_ref()
            .map(|locators| locators.render.as_path()),
        musicxml_generator_identity.as_ref(),
    );
    section_clock = report_section_timing(report.score_title.as_deref(), "format40_pianistic_chain", section_clock);
    crate::registernotation::fuege_berichtsteil_hinzu(&mut out);
    section_clock = report_section_timing(report.score_title.as_deref(), "registernotation", section_clock);
    crate::audit_only::voice_run_integrity::append_report_section(
        &mut out,
        options
            .musicxml_artifact_locators
            .as_ref()
            .map(|locators| locators.master.as_path()),
        options
            .musicxml_artifact_locators
            .as_ref()
            .map(|locators| locators.render.as_path()),
    );
    section_clock = report_section_timing(report.score_title.as_deref(), "voice_run_integrity", section_clock);
    crate::audit_only::import_channel_profile_readback::append_report_section(
        &mut out,
        options
            .musicxml_artifact_locators
            .as_ref()
            .map(|locators| locators.master.as_path()),
        options
            .musicxml_artifact_locators
            .as_ref()
            .map(|locators| locators.render.as_path()),
    );
    section_clock = report_section_timing(report.score_title.as_deref(), "import_channel_profile", section_clock);
    crate::audit_only::voice_run_collision_origin::append_report_section(
        &mut out,
        options
            .musicxml_artifact_locators
            .as_ref()
            .map(|locators| locators.master.as_path()),
    );
    section_clock = report_section_timing(report.score_title.as_deref(), "voice_run_collision_origin", section_clock);
    let _ = section_clock;

    if report.issues.is_empty() {
        out.push_str("Keine formalen Blocker, keine formalen musikalisch-notatorischen Warnungen, keine Hinweise.\n");
        out.push_static_named_report_line_v766("Freigabe", " MusicXML formal und PDF-Renderkette stabil. Notationsfreigabe folgt ausschließlich den maschinenprüfbaren Bienenkönigin-/Behind-Bars-Gates; ein separates redaktionelles oder menschliches Regelgate existiert nicht.");
        return out;
    }

    append_section(&mut out, "FORMALE BLOCKER", report, Severity::Blocker, None);

    if report.summary.warnings > 0 {
        append_section(
            &mut out,
            "MUSIKALISCH-NOTATORISCHE WARNUNGEN - WICHTIGSTE EINZELBEFUNDE",
            report,
            Severity::Warning,
            Some(options.top),
        );
        if report.summary.warnings > options.top {
            out.push_format_args(format_args!(
                "Hinweis: {} weitere Warnungen wurden wegen des Top-Limits nicht ausgeschrieben.\n\n",
                report.summary.warnings - options.top
            ));
        }
    }

    append_section(&mut out, "HINWEISE", report, Severity::Hint, Some(options.top));

    if report.summary.blockers > 0 {
        out.push_static_named_report_line_v766("\nEntscheidung", " PDF-Ausgabe nicht freigeben. Erst Blocker reparieren.");
    } else if report.summary.warnings > 0 {
        out.push_static_named_report_line_v766("\nEntscheidung", " formal freigebbar, aber musikalisch-notatorische Warnungen prüfen.");
    } else {
        out.push_static_named_report_line_v766("\nEntscheidung", " freigebbar.");
    }

    out
}



pub fn clarify_legacy_music_mutation_report(text: String) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        out.push_str(&relabel_legacy_music_mutation_fields(line));
        out.push('\n');
    }
    out
}

pub(crate) fn append_mutation_semantics_authority_body(out: &mut String) {
    out.push_str(
        "Authority: RenderXmlWrite, RenderNotationMutation, SoundingEventMutation, MasterMusicXmlMutation and DeclaredTakeChangeClass are separate typed dimensions.\n",
    );
    out.push_str(
        "LegacyMusicMutation: diagnostic_only_non_authoritative_historical_field; its per-operator value must not be interpreted as sounding-event, master, notation or take-class authority.\n",
    );
}

fn relabel_legacy_music_mutation_fields(line: &str) -> String {
    let mut line = line.to_string();
    for (token, replacement) in [
        ("MusicMutation", "LegacyMusicMutation"),
        ("music_mutation", "legacy_music_mutation"),
    ] {
        let mut scan = 0usize;
        while let Some(relative) = line[scan..].find(token) {
            let start = scan + relative;
            let before_is_identifier = line[..start]
                .chars()
                .next_back()
                .is_some_and(|ch| ch.is_alphanumeric() || ch == '_');
            let after = start + token.len();
            if !before_is_identifier && legacy_mutation_value_follows(&line, after) {
                line.replace_range(start..after, replacement);
                scan = start + replacement.len();
            } else {
                scan = after;
            }
        }
    }
    line
}

fn legacy_mutation_value_follows(line: &str, mut cursor: usize) -> bool {
    if line[cursor..].starts_with(':') || line[cursor..].starts_with('=') {
        cursor += 1;
    }
    cursor += line[cursor..]
        .chars()
        .take_while(|ch| ch.is_whitespace())
        .map(char::len_utf8)
        .sum::<usize>();
    ["ja", "nein", "yes", "no", "true", "false"]
        .into_iter()
        .any(|value| {
            line[cursor..].starts_with(value)
                && line[cursor + value.len()..]
                    .chars()
                    .next()
                    .is_none_or(|ch| !ch.is_alphanumeric() && ch != '_')
        })
}

pub fn sanitize_measure_locator_authority_report(text: String) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        let mut sanitized = mask_locator_lists_and_ranges(line.to_string());
        sanitized = mask_number_after_phrases(
            sanitized,
            &[
                "Takt ",
                "Takt-",
                "Analysebereich ",
                "Analysebereich-",
                "analysebereich ",
                "analysebereich-",
                "Bereich ",
                "Bereich-",
                "bereich ",
                "bereich-",
                "Seite ",
                "Page ",
                "page ",
                "Fokus Takt ",
                "Fokus Scope ",
                "Focus Scope ",
                "Zielbereich ",
                "zielbereich ",
                "Scope ",
                "Scope-",
                "scope ",
                "scope-",
                "TargetMeasure ",
                "Target measure ",
                "target measure ",
                "Measure ",
                "measure ",
                "measure_",
                "measure=",
                "Measure=",
                "target_measure=",
                "TargetMeasure=",
                "AffectedScopes: ",
                "AffectedScopes ",
                "affectedScopes: ",
                "affectedScopes ",
                "CompleteBeatMeasures ",
                "CompleteBeatOperatorMeasures ",
                "target M",
                "Target M",
                "Ziel M",
                "ziel M",
                "Anker ",
            ],
        );
        sanitized = mask_digits_after_word(sanitized, "Anchor");
        sanitized = mask_digits_after_word(sanitized, "Takt");
        sanitized = mask_digits_after_word(sanitized, "Analysebereich");
        sanitized = mask_digits_after_word(sanitized, "analysebereich");
        sanitized = mask_digits_after_word(sanitized, "Bereich");
        sanitized = mask_digits_after_word(sanitized, "bereich");
        sanitized = mask_digits_after_word(sanitized, "Measure");
        sanitized = mask_digits_after_word(sanitized, "measure");
        sanitized = mask_digits_after_word(sanitized, "Scope");
        sanitized = mask_digits_after_word(sanitized, "scope");
        sanitized = mask_locator_suffix_ranges(sanitized);
        sanitized = mask_slash_locator_digits(sanitized);
        sanitized = mask_first_numeric_ledger_cell(sanitized);
        out.push_str(&sanitized);
        out.push('\n');
    }
    out
}

pub fn sanitize_final_locator_suffixes(text: String) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        out.push_str(&mask_locator_suffix_ranges(line.to_string()));
        out.push('\n');
    }
    out
}

fn mask_number_after_phrases(mut line: String, phrases: &[&str]) -> String {
    for phrase in phrases {
        let mut scan = 0_usize;
        while let Some(rel) = line[scan..].find(phrase) {
            let start = scan + rel + phrase.len();
            if !line[start..].chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
                scan = start;
                continue;
            }
            let end = start + line[start..]
                .chars()
                .take_while(char::is_ascii_digit)
                .map(char::len_utf8)
                .sum::<usize>();
            line.replace_range(start..end, "<locator>");
            scan = start + "<locator>".len();
        }
    }
    line
}

fn mask_digits_after_word(mut line: String, word: &str) -> String {
    let mut scan = 0_usize;
    while let Some(rel) = line[scan..].find(word) {
        let start = scan + rel + word.len();
        if !line[start..].chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
            scan = start;
            continue;
        }
        let end = start + line[start..]
            .chars()
            .take_while(char::is_ascii_digit)
            .map(char::len_utf8)
            .sum::<usize>();
        line.replace_range(start..end, "<locator>");
        scan = start + "<locator>".len();
    }
    line
}

fn mask_locator_lists_and_ranges(mut line: String) -> String {
    for phrase in [
        "Takt ",
        "Fokus Takt ",
        "TargetMeasure ",
        "Target measure ",
        "target measure ",
        "Measure ",
        "measure ",
        "Bereiche ",
        "bereiche ",
        "Scope ",
        "scope ",
        "FirstScopeRange ",
        "ScopeRange ",
        "scopeRange ",
        "FocusScopeRange ",
        "AffectedScopes: ",
        "AffectedScopes ",
        "affectedScopes: ",
        "affectedScopes ",
        "CompleteBeatMeasures ",
        "CompleteBeatOperatorMeasures ",
        "Measures ",
        "measures ",
    ] {
        let mut scan = 0_usize;
        while let Some(rel) = line[scan..].find(phrase) {
            let start = scan + rel + phrase.len();
            if !line[start..].chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
                scan = start;
                continue;
            }
            let end = start + line[start..]
                .chars()
                .take_while(|ch| ch.is_ascii_digit() || *ch == ',' || *ch == '-')
                .map(char::len_utf8)
                .sum::<usize>();
            let token = &line[start..end];
            if token.contains(',') {
                line.replace_range(start..end, "<locator-list>");
                scan = start + "<locator-list>".len();
            } else if token.contains('-') {
                line.replace_range(start..end, "<locator-range>");
                scan = start + "<locator-range>".len();
            } else {
                scan = end;
            }
        }
    }
    line
}

fn mask_locator_suffix_ranges(mut line: String) -> String {
    let mut scan = 0_usize;
    while let Some(rel) = line[scan..].find("<locator>-") {
        let start = scan + rel;
        let digit_start = start + "<locator>-".len();
        if !line[digit_start..].chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
            scan = digit_start;
            continue;
        }
        let end = digit_start + line[digit_start..]
            .chars()
            .take_while(char::is_ascii_digit)
            .map(char::len_utf8)
            .sum::<usize>();
        line.replace_range(start..end, "<locator-range>");
        scan = start + "<locator-range>".len();
    }
    line
}

fn mask_slash_locator_digits(mut line: String) -> String {
    let mut scan = 0_usize;
    while let Some(rel) = line[scan..].find("<locator>/") {
        let mut cursor = scan + rel + "<locator>".len();
        while line[cursor..].starts_with('/') {
            let digit_start = cursor + 1;
            if !line[digit_start..]
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_digit())
            {
                break;
            }
            let digit_end = digit_start
                + line[digit_start..]
                    .chars()
                    .take_while(char::is_ascii_digit)
                    .map(char::len_utf8)
                    .sum::<usize>();
            line.replace_range(digit_start..digit_end, "<locator>");
            cursor = digit_start + "<locator>".len();
        }
        scan = cursor;
    }
    line
}

fn mask_first_numeric_ledger_cell(mut line: String) -> String {
    let Some(row_start) = line.find("Row: ") else {
        return line;
    };
    let start = row_start + "Row: ".len();
    if !line[start..].chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
        return line;
    }
    let end = start + line[start..]
        .chars()
        .take_while(char::is_ascii_digit)
        .map(char::len_utf8)
        .sum::<usize>();
    line.replace_range(start..end, "<locator>");
    line
}


fn append_section(
    out: &mut String,
    title: &str,
    report: &CheckReport,
    severity: Severity,
    limit: Option<usize>,
) {
    let mut issues: Vec<&Issue> = report
        .issues
        .iter()
        .filter(|i| i.severity == severity)
        .collect();

    if issues.is_empty() {
        return;
    }

    issues.sort_by(|a, b| {
        priority_score(b)
            .cmp(&priority_score(a))
            .then_with(|| a.measure.cmp(&b.measure))
            .then_with(|| a.part_id.cmp(&b.part_id))
            .then_with(|| a.staff.cmp(&b.staff))
    });

    let shown = limit.unwrap_or(issues.len()).min(issues.len());

    out.push_format_args(format_args!("{title}\n"));
    out.push_format_args(format_args!("{}\n", "-".repeat(title.len())));

    for issue in issues.into_iter().take(shown) {
        append_issue(out, issue);
    }
    out.push('\n');
}

fn append_issue(out: &mut String, issue: &Issue) {
    out.push_format_args(format_args!(
        "- [{}] [{}] Takt {}, Part {} ({})",
        urgency_label(issue),
        issue_authority_label(issue),
        issue.measure,
        issue.part_name,
        issue.part_id
    ));
    if let Some(staff) = issue.staff.as_ref() {
        out.push_format_args(format_args!(", Staff {staff}"));
    }
    if let Some(voice) = issue.voice.as_ref() {
        out.push_format_args(format_args!(", Voice {voice}"));
    }
    out.push('\n');
    out.push_format_args(format_args!("  EtuSchlopf: {}\n", issue.details));
    if let Some(expected) = issue.expected_divisions.as_ref() {
        out.push_format_args(format_args!("  Erwartet: {expected}\n"));
    }
    if let Some(actual) = issue.actual_divisions {
        out.push_format_args(format_args!("  Gefunden: {} {}\n", actual, actual_unit(issue)));
    }
    if let Some(delta) = issue.delta_divisions.as_ref() {
        out.push_format_args(format_args!("  Delta: {delta}\n"));
    }
    out.push_format_args(format_args!(
        "  RuleAuthority: runtime_severity={} | rule_severity={} | rule_weight={}\n",
        severity_authority_label(issue.severity),
        issue.rule_severity,
        issue.rule_weight
    ));
    out.push_format_args(format_args!("  Reparaturhinweis: {}\n", repair_hint_text(issue)));
}

fn issue_authority_label(issue: &Issue) -> String {
    format!("{}:{}", issue.rule_id, issue.issue_type)
}

fn append_rule_severity_authority_audit(out: &mut String, report: &CheckReport) {
    crate::push_static_section_header_v793(out, "RULE-SEVERITY-AUTHORITY-AUDIT", "-----------------------------");
    let mut by_rule: BTreeMap<&str, usize> = BTreeMap::new();
    let mut unmapped = 0_usize;
    let mut mismatches = Vec::new();
    for issue in &report.issues {
        *by_rule.entry(issue.rule_id.as_str()).or_insert(0) += 1;
        if issue.rule_id.starts_with("UNMAPPED:") || issue.rule_severity == "UNMAPPED" {
            unmapped += 1;
        }
        let runtime = severity_authority_label(issue.severity);
        if runtime != issue.rule_severity {
            mismatches.push(format!(
                "{}:{} runtime {} rule {}",
                issue.rule_id, issue.issue_type, runtime, issue.rule_severity
            ));
        }
    }
    out.push_format_args(format_args!(
        "IssueRows {}; RuleIds {}; Unmapped {}; RuntimeVsRuleSeverityMismatches {}.\n",
        report.issues.len(),
        by_rule.len(),
        unmapped,
        mismatches.len()
    ));
    out.push_format_args(format_args!(
        "ValidatorIssueAuthorityInvariant: {}.\n",
        validator_issue_authority_invariant_status()
    ));
    if by_rule.is_empty() {
        out.push_static_named_report_line_v766("RuleIdHistogram", " keine Issues in diesem Lauf.");
    } else {
        let histogram = by_rule
            .iter()
            .map(|(rule_id, count)| format!("{rule_id} {count}"))
            .collect::<Vec<_>>()
            .join("; ");
        out.push_format_args(format_args!("RuleIdHistogram: {histogram}.\n"));
    }
    if mismatches.is_empty() {
        out.push_static_named_report_line_v766("SeverityAuthority", " gruen; Runtime-Severity und RuleBook-Severity sind fuer aktive Issues deckungsgleich oder es gab keine Issues.");
    } else {
        out.push_format_args(format_args!("SeverityAuthority: pruefen; {}.\n", mismatches.join("; ")));
    }
    out.push_static_named_report_line_v766("NoBehaviorChange", " Dieses Audit berichtet nur. Runtime-Severity, Summary-Zaehlung und Freigabeentscheidung bleiben unveraendert.");
    out.push('\n');
}

fn severity_authority_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Blocker => "BLOCKER",
        Severity::Warning => "WARNUNG",
        Severity::Hint => "HINWEIS",
    }
}


fn repair_hint_text(issue: &Issue) -> String {
    repair_hint_for_issue_type(issue.issue_type.as_str())
        .map_or_else(|| issue.message_for_ai.clone(), str::to_string)
}

fn priority_score(issue: &Issue) -> i32 {
    priority_score_for_issue_type(issue.issue_type.as_str(), issue.actual_divisions)
}

fn urgency_label(issue: &Issue) -> &'static str {
    match issue.severity {
        Severity::Blocker => "BLOCKER",
        Severity::Hint => "HINWEIS",
        Severity::Warning => {
            if priority_score(issue) >= 200 {
                "KRITISCH"
            } else {
                "WARNUNG"
            }
        }
    }
}

fn actual_unit(issue: &Issue) -> &'static str {
    match issue.actual_unit.as_deref() {
        Some("semitones") => "Halbtöne",
        Some("notes") => "Noten",
        Some("midi") => "MIDI",
        _ => "divisions",
    }
}



#[cfg(test)]
mod locator_sanitizer_metric_preservation_tests {
    use super::{clarify_legacy_music_mutation_report, sanitize_measure_locator_authority_report};

    #[test]
    fn slash_measure_chain_is_masked_without_masking_later_count_pairs() {
        let sanitized = sanitize_measure_locator_authority_report(
            "NoMeasureSpecificLogic: keine Takt 18/22/24/25-Ausnahme; NoteCountBeforeAfter 795/795; AccentCountBeforeAfter 12/11."
                .to_string(),
        );

        assert!(sanitized.contains(
            "Takt <locator>/<locator>/<locator>/<locator>-Ausnahme"
        ));
        assert!(sanitized.contains("NoteCountBeforeAfter 795/795"));
        assert!(sanitized.contains("AccentCountBeforeAfter 12/11"));
    }

    #[test]
    fn target_measure_list_is_masked_without_corrupting_offset_or_duration_pairs() {
        let sanitized = sanitize_measure_locator_authority_report(
            "Target measure 4,10,11,12; FirstGroupOffsets 192:24/216:24/240:24; LastPairOffsets 288:24/360:72."
                .to_string(),
        );

        assert!(sanitized.contains("Target measure <locator-list>"));
        assert!(sanitized.contains("FirstGroupOffsets 192:24/216:24/240:24"));
        assert!(sanitized.contains("LastPairOffsets 288:24/360:72"));
    }

    #[test]
    fn measure_word_elsewhere_does_not_turn_ratios_into_locators() {
        let sanitized = sanitize_measure_locator_authority_report(
            "AccidentalCostOpenTargetBest 221/132/89; FunctionalRespelling 181/170/11; HardcodedMeasureLogic nein."
                .to_string(),
        );

        assert!(sanitized.contains("AccidentalCostOpenTargetBest 221/132/89"));
        assert!(sanitized.contains("FunctionalRespelling 181/170/11"));
        assert!(!sanitized.contains("<locator>"));
    }

    #[test]
    fn shorthand_and_assignment_measure_locators_are_masked() {
        let sanitized = sanitize_measure_locator_authority_report(
            "target M22; Ziel M22; ContextRow: measure=5; SemanticRow: Measure=7."
                .to_string(),
        );

        assert!(sanitized.contains("target M<locator>"));
        assert!(sanitized.contains("Ziel M<locator>"));
        assert!(sanitized.contains("measure=<locator>"));
        assert!(sanitized.contains("Measure=<locator>"));
        assert!(!sanitized.contains("M22"));
    }

    #[test]
    fn structured_scope_lists_are_masked_without_masking_measure_counts() {
        let sanitized = sanitize_measure_locator_authority_report(
            "AffectedScopes: 2,10; affectedScopes 2,10; AffectedScopes 5; CompleteBeatMeasures 4,9,10; CompleteBeatOperatorMeasures 11,12; Measures 4,9,10; RawCursorGuard: Measures 31; voices=1,2."
                .to_string(),
        );

        assert!(sanitized.contains("AffectedScopes: <locator-list>"));
        assert!(sanitized.contains("affectedScopes <locator-list>"));
        assert!(sanitized.contains("AffectedScopes <locator>"));
        assert!(sanitized.contains("CompleteBeatMeasures <locator-list>"));
        assert!(sanitized.contains("CompleteBeatOperatorMeasures <locator-list>"));
        assert!(sanitized.contains("Measures <locator-list>"));
        assert!(sanitized.contains("RawCursorGuard: Measures 31"));
        assert!(sanitized.contains("voices=1,2"));
    }


    #[test]
    fn legacy_music_mutation_value_fields_are_explicitly_non_authoritative() {
        let clarified = clarify_legacy_music_mutation_report(
            "TakePolicy: AuditOnly, MusicMutation, CalibrationOnly.\nRenderXmlWrite ja; MusicMutation nein; MasterApply nein.\nMusicMutation: ja\nmusic_mutation=no\nDeclaredMusicMutation ja; NoMusicMutation nein; no_music_mutation."
                .to_string(),
        );

        assert!(clarified.contains("AuditOnly, MusicMutation, CalibrationOnly"));
        assert!(clarified.contains("LegacyMusicMutation nein"));
        assert!(clarified.contains("LegacyMusicMutation: ja"));
        assert!(clarified.contains("legacy_music_mutation=no"));
        assert!(clarified.contains("DeclaredMusicMutation ja"));
        assert!(clarified.contains("NoMusicMutation nein"));
        assert!(clarified.contains("no_music_mutation"));
        assert!(!clarified.contains("; MusicMutation nein"));
    }


    #[test]
    fn final_locator_sanitizers_follow_all_report_projections() {
        let source = include_str!("pipeline_shards/final_report_projection_owner.rs");
        let mutation_call = concat!(
            "text.rewrite_document(report::",
            "clarify_legacy_music_mutation_report);"
        );
        let locator_call = concat!(
            "text.rewrite_document(report::",
            "sanitize_measure_locator_authority_report);"
        );
        let suffix_call = concat!(
            "text.rewrite_document(report::",
            "sanitize_final_locator_suffixes);"
        );
        assert_eq!(source.matches(mutation_call).count(), 1);
        assert_eq!(source.matches(locator_call).count(), 1);
        assert_eq!(source.matches(suffix_call).count(), 1);
        let writer_trace = source.find("writer_trace_optional").unwrap();
        let mutation_authority = source.find("mutation_semantics_authority").unwrap();
        let mutation = source.find(mutation_call).unwrap();
        let locator = source.find(locator_call).unwrap();
        let suffix = source.find(suffix_call).unwrap();
        let finalize = source.find("let finalized_report = text.finalize()?;").unwrap();
        assert!(
            writer_trace < mutation_authority
                && mutation_authority < mutation
                && mutation < locator
                && locator < suffix
                && suffix < finalize
        );
    }
}
