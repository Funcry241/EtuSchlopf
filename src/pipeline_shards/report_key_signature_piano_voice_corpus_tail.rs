// v0.10.713 ReportKeySignaturePianoVoiceCorpusTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the report/key-signature bridge tail and piano-voice/corpus endpoint tail.
// Former report_key_signature_bridge_tail and piano_voice_corpus_endpoint_tail are included below in the same order.
// No musical, render, solver, or report decision logic is changed.
// v0.10.703 ReportOrchestratorStatusTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the report/orchestrator status tail.
// Former part_41..part_42 are included below in the same order.
// ---- merged from src/pipeline_shards/part_42.rs ----
// v0.10.601 KeySignatureEnharmonicSpellingRenderWrite.
// Orchestrator-only text context for long status/done lines.
struct OrchestratorStatusLineContext {
    status: String,
    blockers: usize,
    warnings: usize,
    hints: usize,
    retention: String,}
fn orchestrator_status_summary_fragment(context: &OrchestratorStatusLineContext) -> String {
    format!(
        "Status {} | Blocker {} | Warnungen {} | Hinweise {}; Notenerhalt {}",
        context.status,
        context.blockers,
        context.warnings,
        context.hints,
        context.retention
    )
}
fn orchestrator_console_done_line(context: &OrchestratorStatusLineContext) -> String {
    format!(
        "Getan: v{}; {}; Details im Ergebnisbericht.",
        env!("CARGO_PKG_VERSION"),
        orchestrator_status_summary_fragment(context)
    )
}

// v0.10.708 KeySignatureEnharmonicBridgeTailShardConsolidationNoBehaviorChange.
// Mechanical consolidation of the key-signature/enharmonic render-write + audit + span-write bridge.
// Former part_43.rs, key_signature_enharmonic_audit_tail.rs and part_46.rs are included below in the same order.
