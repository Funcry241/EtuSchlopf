//! Internal pipeline namespace.
//!
//! The historical shards and the notation/render/WAV support that consume their private data
//! share one lexical owner. The boundary keeps implementation details out of the crate root without
//! widening them to `pub(crate)`. Its parent dependencies are explicit, and shard order and musical
//! behavior remain unchanged.

use crate::master_state_identity::{
    BlockTransformationReceipt, ExactTransformationReceipt, PreservingTransformationReceipt,
};
use super::final_report::{DraftReport, ProjectionIdentity};
use etuschlopf::{observation, observation_transport};
use super::{
    arrangement, cli, corpus_run_file_evidence,
    corpus_run_project_evidence, feedback_structuralization_gate, master_state_identity, midi_seed,
    identity_debt_inventory, identity_gate_trust_anchor, layout, model, musicxml,
    productive_write_path_inventory, push_kurz_no_direct_write_release_green_closed,
    reference, report, rules, sha256, structural_write_authority,
    validate_corpus_run_manifest, CorpusRunManifest, StaticNamedReportLineV766,
    CORPUS_RUN_MANIFEST_FILE,
};
use super::audit_only::serialized_temporal_replay::{
    observe_serialized_temporal_xml, scan_raw_cursor_xml,
    SerializedTemporalReplayRow as RawCursorInvariantRow,
};

#[path = "notation.rs"]
mod notation;
pub(crate) use notation::{LedgerBoundedLowerStaffClefReadbackPreservingMint, NotationClefIslandPreservingMint, NotationPolishPreservingMint};
#[path = "render.rs"]
mod render;
pub(crate) use render::SameLaneRawSpliceExactMint;
#[path = "persisted_schema_validation.rs"]
mod persisted_schema_validation;

// Validation-only facade over the existing canonical source resolver. It preserves the
// SourcePlan byte-for-byte while registering only its already-structuralized final
// artifact paths for the read-only persisted-byte XSD gate. No CLI/env identity is
// re-read or exposed to a productive writer boundary.
mod source {
    pub(super) use crate::source::SourcePlan;

    pub(super) fn resolve_source_plan(
        raw_input: Option<&std::path::Path>,
        report_out: Option<&std::path::Path>,
        musicxml_out: Option<&std::path::Path>,
        pdf_out: Option<&std::path::Path>,
    ) -> anyhow::Result<SourcePlan> {
        let plan = crate::source::resolve_source_plan(
            raw_input,
            report_out,
            musicxml_out,
            pdf_out,
        )?;
        super::persisted_schema_validation::register_final_artifacts(
            &plan.musicxml_out,
            &plan.render_musicxml_out,
            &plan.pdf_out,
        )
        .map_err(anyhow::Error::msg)?;
        Ok(plan)
    }
}
#[path = "wav_truth_articulation.rs"]
mod wav_truth_articulation;
use etuschlopf::console_presentation_contract::{
    resolve_stream_render_policy, ColorPreference, ConsolePreferences, GlyphCapability, GlyphMode,
    GlyphPreference, ProgressMode, ProgressPreference, StreamCapability, StreamDestination,
    StreamRenderPolicy, StyleMode,
};
use wav_truth_articulation::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipelineExit {
    Success,
    Failure(u8),
}

impl PipelineExit {
    fn from_process_code(code: i32) -> Self {
        match u8::try_from(code) {
            Ok(0) => Self::Success,
            Ok(code) => Self::Failure(code),
            Err(_) => Self::Failure(1),
        }
    }

    fn into_exit_code(self) -> std::process::ExitCode {
        match self {
            Self::Success => std::process::ExitCode::SUCCESS,
            Self::Failure(code) => std::process::ExitCode::from(code),
        }
    }
}

pub(super) fn run() -> std::process::ExitCode {
    match real_main() {
        Ok(code) => {
            if code == 0 {
                let gate_was_already_evaluated =
                    persisted_schema_validation::final_gate_evaluated();
                match persisted_schema_validation::validate_registered_final_artifacts() {
                    Ok(Some(summary)) if !gate_was_already_evaluated => {
                        println!("ETUSCHLOPF: [XSD-GATE   ] {summary}");
                    }
                    Ok(Some(_)) => {}
                    Ok(None) => {}
                    Err(error) => {
                        eprintln!("ETUSCHLOPF: FEHLER");
                        eprintln!("{error}");
                        return std::process::ExitCode::FAILURE;
                    }
                }
            }
            PipelineExit::from_process_code(code).into_exit_code()
        }
        Err(error) => {
            eprintln!("ETUSCHLOPF: FEHLER");
            eprintln!("{error:#}");
            std::process::ExitCode::FAILURE
        }
    }
}

include!("pipeline_shards/pipeline_runtime_progress_console_core.rs");
include!("pipeline_shards/candidate_material_lineage_spine_core.rs");
include!("pipeline_shards/pipeline_foundation_runtime_passorder_tail.rs");
include!("pipeline_shards/sheaf_noether_harmonic_micro_tail.rs");
include!("pipeline_shards/sheaf_noether_projection_core.rs");
include!("pipeline_shards/render_write_pdf_source_adoption_truth_gate.rs");
include!("pipeline_shards/pipeline_orchestrator_report_compaction.rs");
include!("pipeline_shards/report_compaction_summary_extract_core.rs");
include!("pipeline_shards/voice_fiber_human_pdf_decision_core.rs");
include!("pipeline_shards/staff_voice_rest_glyph_topology_tail.rs");
include!("pipeline_shards/retention_composer_limit_core.rs");
include!("pipeline_shards/rest_glyph_retention_batch_corpus_tail.rs");
include!("pipeline_shards/corpus_fastjump_human_potential_core.rs");
include!("pipeline_shards/corpus_fastjump_human_potential_repair_tail.rs");
include!("pipeline_shards/staff_voice_atomic_move_patch_core.rs");
include!("pipeline_shards/staff_voice_move_restify_atomic_patch_tail.rs");
include!("pipeline_shards/render_hidden_material_prune_core.rs");
include!("pipeline_shards/render_polish_hidden_rest_readability_tail.rs");
include!("pipeline_shards/strict_staff_local_release_core.rs");
include!("pipeline_shards/strict_staff_beat_local_rest_observation_core.rs");
include!("pipeline_shards/strict_staff_same_pitch_planning_core.rs");
include!("pipeline_shards/strict_staff_rest_lane_planning_core.rs");
include!("pipeline_shards/strict_staff_visible_lane_rest_rewrite_tail.rs");
include!("pipeline_shards/lower_staff_register_feedback_gate_core.rs");
include!("pipeline_shards/noether_register_articulation_readback_tail.rs");
include!("pipeline_shards/human_pdf_pianistic_quality_core.rs");
include!("pipeline_shards/human_pdf_pianistic_cross_staff_ledger_tail.rs");
include!("pipeline_shards/cross_staff_editorial_voice_math_gate_tail.rs");
include!("pipeline_shards/hidden_chord_math_reduction_core.rs");
include!("pipeline_shards/hidden_chord_reduction_texture_rebuild_tail.rs");
include!("pipeline_shards/composer_recovery_hand_cause_core.rs");
include!("pipeline_shards/composer_metric_beam_observation_core.rs");
include!("pipeline_shards/composer_beat_scoped_rest_completion_planning_core.rs");
include!("pipeline_shards/composer_metric_beam_planning_core.rs");
include!("pipeline_shards/actual_beam_capture_core.rs");
include!("pipeline_shards/composer_beam_candidate_materialization_core.rs");
include!("pipeline_shards/composer_recovery_octave_beam_export_tail.rs");
include!("pipeline_shards/partial_beat_post_rest_triple_beam_final_repair.rs");
include!("pipeline_shards/lower_dual_edge_beat_beam_continuity_repair.rs");
include!("pipeline_shards/notation_endpoint_layout_core.rs");
include!("pipeline_shards/notation_endpoint_clef_layout_playability_tail.rs");
include!("pipeline_shards/audio_salience_discrete_translator_core.rs");
include!("pipeline_shards/audio_salience_discrete_action_conflict_gate_tail.rs");
include!("pipeline_shards/report_hygiene_cursor_localbest_trace_tail.rs");
include!("pipeline_shards/phrase_slur_register_octave_accent_stem_human_pdf_tail.rs");
include!("pipeline_shards/report_key_signature_piano_voice_corpus_tail.rs");
include!("pipeline_shards/key_signature_enharmonic_report_core.rs");

#[cfg(test)]
mod tests {
    use super::PipelineExit;

    #[test]
    fn process_codes_are_normalized_at_the_binary_boundary() {
        assert_eq!(PipelineExit::from_process_code(0), PipelineExit::Success);
        assert_eq!(PipelineExit::from_process_code(1), PipelineExit::Failure(1));
        assert_eq!(PipelineExit::from_process_code(255), PipelineExit::Failure(255));
        assert_eq!(PipelineExit::from_process_code(-1), PipelineExit::Failure(1));
        assert_eq!(PipelineExit::from_process_code(256), PipelineExit::Failure(1));
    }
}

#[cfg(test)]
#[path = "../tests/unit/ottava_reader_owner_chain.rs"]
mod ottava_reader_owner_chain_tests;
