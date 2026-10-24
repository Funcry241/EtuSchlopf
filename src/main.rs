//! EtuSchlopf binary entrypoint and crate wiring.
#![forbid(unsafe_code)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(unused_mut)]
#![deny(unused_must_use)]
#![deny(unreachable_code)]
mod report_format;
pub use report_format::{
    push_static_dec_line_v765, push_static_ent_line_v765,
    push_static_ledger_header_v804, push_static_ledger_line_v804,
    push_static_named_policy_line_v765,
    push_static_section_header_v793, push_static_write_policy_line_v765,
    StaticNamedReportLineV766,
};

mod arrangement;
mod audit_only;
mod cursor_lane_temporal_check;
mod central_rulebook;
mod clef_readability_observation;
mod cli;
use etuschlopf::{corpus_run_manifest, formal_green_contract};
#[path = "midi_seed.rs"]
mod midi_seed;
mod master_state_identity;
mod feedback_structuralization_gate;
mod feedback_proof_transport;
mod final_report;
mod final_notation_timeline_facts;
mod identity_debt_inventory;
mod identity_gate_trust_anchor;
mod layout;
mod midi;
mod model;
mod musicxml;
pub mod pipeline_profile;
mod productive_write_path_inventory;
#[path = "build_gate/sha256.rs"]
mod sha256;
mod reference;
mod registernotation;
mod report;
mod report_sanitizer;
mod rule_id;
mod rules;
mod source;
mod source_event;
mod source_selection;
// Productive typed structural write authority; v0.10.1288 removes the orphaned enharmonic write capability after the tonal writers were quarantined. Algorithmic identity debt remains required at 0/0.
mod structural_write_authority;
use corpus_run_manifest::{
    file_evidence as corpus_run_file_evidence, project_evidence as corpus_run_project_evidence,
    validate_manifest as validate_corpus_run_manifest, CorpusRunManifest, CORPUS_RUN_MANIFEST_FILE,
};
fn gate_no_direct_write_no_mutation() -> &'static str {
    concat!(
        "DirectWriteReady 0; ",
        "RenderXmlWrite nein; ",
        "MusicMutation nein; ",
        "MasterApply nein; ",
        "P3/P4 nein; ",
        "MXL nein"
    )
}
fn push_kurz_no_direct_write_release_green_closed(out: &mut String, prefix: &str) {
    out.push_str(prefix);
    out.push_str(gate_no_direct_write_no_mutation());
    out.push_str("; ReleaseGreen nein.\n");
}
mod pipeline;

fn main() -> std::process::ExitCode {
    audit_only::panic_origin_observation::install();
    pipeline::run()
}

mod ssc_evidence_proof;
mod ssc_shadow_permit;
mod ssc_shadow_report;
mod ssc_shadow_measurement;
