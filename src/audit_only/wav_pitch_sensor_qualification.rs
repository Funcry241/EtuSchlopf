//! Audit-only qualification boundary for learned/external WAV pitch sensors.
//!
//! This module deliberately grants no sensor runtime, pitch decision, MIDI,
//! CanonicalSourceEvent, productive source, phrase or writer authority. It
//! freezes the provenance/evidence contract first so later sensor benchmarks
//! can be compared without changing the rules to fit a preferred model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SensorAuthority {
    EvidenceOnly,
}

impl SensorAuthority {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceOnly => "evidence_only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SensorQualificationContractV1 {
    pub(crate) schema: &'static str,
    pub(crate) authority: SensorAuthority,
    pub(crate) requires_sensor_identity: bool,
    pub(crate) requires_sensor_version: bool,
    pub(crate) requires_model_fingerprint: bool,
    pub(crate) requires_feature_schema: bool,
    pub(crate) requires_source_wav_fingerprint: bool,
    pub(crate) requires_derived_view_identity: bool,
    pub(crate) requires_original_frame_mapping: bool,
    pub(crate) requires_windows_offline_runtime: bool,
    pub(crate) requires_deterministic_decision_output: bool,
    pub(crate) forbids_network_dependency: bool,
    pub(crate) forbids_midi_authority: bool,
    pub(crate) forbids_source_event_promotion: bool,
    pub(crate) forbids_staff_hand_decision: bool,
    pub(crate) forbids_phrase_change: bool,
    pub(crate) forbids_writer_path: bool,
}

impl SensorQualificationContractV1 {
    pub(crate) const fn frozen() -> Self {
        Self {
            schema: "wav_pitch_sensor_qualification_v1",
            authority: SensorAuthority::EvidenceOnly,
            requires_sensor_identity: true,
            requires_sensor_version: true,
            requires_model_fingerprint: true,
            requires_feature_schema: true,
            requires_source_wav_fingerprint: true,
            requires_derived_view_identity: true,
            requires_original_frame_mapping: true,
            requires_windows_offline_runtime: true,
            requires_deterministic_decision_output: true,
            forbids_network_dependency: true,
            forbids_midi_authority: true,
            forbids_source_event_promotion: true,
            forbids_staff_hand_decision: true,
            forbids_phrase_change: true,
            forbids_writer_path: true,
        }
    }

    pub(crate) const fn closed(self) -> bool {
        self.requires_sensor_identity
            && self.requires_sensor_version
            && self.requires_model_fingerprint
            && self.requires_feature_schema
            && self.requires_source_wav_fingerprint
            && self.requires_derived_view_identity
            && self.requires_original_frame_mapping
            && self.requires_windows_offline_runtime
            && self.requires_deterministic_decision_output
            && self.forbids_network_dependency
            && self.forbids_midi_authority
            && self.forbids_source_event_promotion
            && self.forbids_staff_hand_decision
            && self.forbids_phrase_change
            && self.forbids_writer_path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WavPitchSensorQualificationBoundaryAudit {
    pub(crate) contract: SensorQualificationContractV1,
    pub(crate) wav_present: bool,
    pub(crate) external_sensor_runtime_present: bool,
    pub(crate) productive_sensor_selected: bool,
}

pub(crate) fn build_for_score_title(
    score_title: Option<&str>,
) -> Option<WavPitchSensorQualificationBoundaryAudit> {
    let title = score_title?.trim();
    if title.is_empty() {
        return None;
    }
    let current = std::env::current_dir().ok()?;
    let wav_present = current.join(format!("{title}.wav")).is_file();
    if !wav_present {
        return None;
    }

    Some(WavPitchSensorQualificationBoundaryAudit {
        contract: SensorQualificationContractV1::frozen(),
        wav_present,
        external_sensor_runtime_present:
            crate::audit_only::wav_pitch_external_sensor_benchmark::runtime_present_for_current_project(),
        productive_sensor_selected: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualification_contract_is_closed_and_evidence_only() {
        let contract = SensorQualificationContractV1::frozen();
        assert!(contract.closed());
        assert_eq!(contract.authority, SensorAuthority::EvidenceOnly);
        assert_eq!(contract.schema, "wav_pitch_sensor_qualification_v1");
    }
}
