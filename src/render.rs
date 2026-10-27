// v0.10.1521 pipeline-owned render support module.
// The implementation remains byte-for-byte behavioral; only its lexical owner changes from
// the crate root to `pipeline`, so private pipeline helpers are reachable as ancestor state.
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};
pub(super) mod rest_placement {
    // Typed, XML-free rest-placement decision authority.
    //
    // v0.10.1580 remains the sole rest-placement policy authority.
    // MusicXML strings are parsed once at this boundary; consumers receive only
    // typed decisions or typed fixed-placement plans.

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RestClef {
        Treble,
        Bass,
        C,
        Other,
    }

    impl RestClef {
        pub fn from_musicxml(value: &str) -> Self {
            match value.trim() {
                "G" => Self::Treble,
                "F" => Self::Bass,
                "C" => Self::C,
                _ => Self::Other,
            }
        }

        fn placement_label(self) -> &'static str {
            match self {
                Self::Treble => "gclef",
                Self::Bass => "fclef",
                Self::C | Self::Other => "cclef",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RestGlyph {
        Whole,
        Breve,
        Long,
        Maxima,
        Half,
        Quarter,
        Eighth,
        Sixteenth,
        ThirtySecond,
        SixtyFourthOrShorter,
        Other,
    }

    impl RestGlyph {
        pub fn from_musicxml(value: &str) -> Self {
            match value.trim() {
                "whole" => Self::Whole,
                "breve" => Self::Breve,
                "long" => Self::Long,
                "maxima" => Self::Maxima,
                "half" => Self::Half,
                "quarter" => Self::Quarter,
                "eighth" => Self::Eighth,
                "16th" => Self::Sixteenth,
                "32nd" => Self::ThirtySecond,
                "64th" | "128th" | "256th" | "512th" | "1024th" => {
                    Self::SixtyFourthOrShorter
                }
                _ => Self::Other,
            }
        }

        fn is_extended_whole(self) -> bool {
            matches!(self, Self::Breve | Self::Long | Self::Maxima)
        }

        fn is_short_high(self) -> bool {
            matches!(self, Self::ThirtySecond | Self::SixtyFourthOrShorter)
        }

        fn fixed_visual_class(self) -> &'static str {
            match self {
                Self::Whole | Self::Breve | Self::Long | Self::Maxima => "whole_like",
                Self::Half => "half",
                Self::Quarter => "quarter",
                Self::Eighth => "eighth",
                Self::Sixteenth => "16th",
                Self::ThirtySecond => "32nd",
                Self::SixtyFourthOrShorter => "64th_or_shorter",
                Self::Other => "other_visible",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RestPlacementMode {
        Canonical,
        PolyphonicWholeRest,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RestPlacementContext {
        pub clef: RestClef,
        pub glyph: RestGlyph,
        pub mode: RestPlacementMode,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct RestDisplayAnchor {
        pub step: &'static str,
        pub octave: i32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RestStaffSlot {
        Middle,
        Upper,
        Top,
    }

    impl RestStaffSlot {
        fn display_anchor(self, clef: RestClef) -> Option<RestDisplayAnchor> {
            match (self, clef) {
                (Self::Middle, RestClef::Treble) => Some(RestDisplayAnchor {
                    step: "B",
                    octave: 4,
                }),
                (Self::Upper, RestClef::Treble) => Some(RestDisplayAnchor {
                    step: "D",
                    octave: 5,
                }),
                (Self::Top, RestClef::Treble) => Some(RestDisplayAnchor {
                    step: "F",
                    octave: 5,
                }),
                (Self::Middle, RestClef::Bass) => Some(RestDisplayAnchor {
                    step: "D",
                    octave: 3,
                }),
                (Self::Upper, RestClef::Bass) => Some(RestDisplayAnchor {
                    step: "F",
                    octave: 3,
                }),
                (Self::Top, RestClef::Bass) => Some(RestDisplayAnchor {
                    step: "A",
                    octave: 3,
                }),
                (_, RestClef::C | RestClef::Other) => None,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum RestPlacementDecision {
        Preserve,
        Anchor(RestDisplayAnchor),
    }

    impl RestPlacementDecision {
        pub fn anchor(self) -> Option<RestDisplayAnchor> {
            match self {
                Self::Preserve => None,
                Self::Anchor(anchor) => Some(anchor),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FixedRestPlacementPlan {
        pub anchor: RestDisplayAnchor,
        pub placement_class: String,
    }

    pub fn decide_rest_placement(
        context: RestPlacementContext,
    ) -> RestPlacementDecision {
        let slot = match context.mode {
            RestPlacementMode::PolyphonicWholeRest => RestStaffSlot::Top,
            RestPlacementMode::Canonical if context.glyph.is_extended_whole() => {
                RestStaffSlot::Upper
            }
            RestPlacementMode::Canonical
                if context.clef == RestClef::Treble && context.glyph.is_short_high() =>
            {
                RestStaffSlot::Upper
            }
            RestPlacementMode::Canonical => RestStaffSlot::Middle,
        };
        slot.display_anchor(context.clef)
            .map_or(RestPlacementDecision::Preserve, RestPlacementDecision::Anchor)
    }

    pub fn anchor_from_musicxml(
        clef_sign: &str,
        rest_type: &str,
        mode: RestPlacementMode,
    ) -> Option<RestDisplayAnchor> {
        decide_rest_placement(RestPlacementContext {
            clef: RestClef::from_musicxml(clef_sign),
            glyph: RestGlyph::from_musicxml(rest_type),
            mode,
        })
        .anchor()
    }

    pub fn fixed_staff_rest_placement_plan(
        staff: &str,
        clef_sign: &str,
        rest_type: &str,
    ) -> FixedRestPlacementPlan {
        let staff_label = if staff.trim() == "2" { "staff2" } else { "staff1" };
        let clef = RestClef::from_musicxml(clef_sign);
        let glyph = RestGlyph::from_musicxml(rest_type);
        let decision = decide_rest_placement(RestPlacementContext {
            clef,
            glyph,
            mode: RestPlacementMode::Canonical,
        });
        let anchor = decision.anchor().unwrap_or(match glyph {
            RestGlyph::Whole | RestGlyph::Breve | RestGlyph::Long | RestGlyph::Maxima => {
                RestDisplayAnchor {
                    step: "G",
                    octave: 4,
                }
            }
            RestGlyph::Half => RestDisplayAnchor {
                step: "C",
                octave: 4,
            },
            RestGlyph::Quarter
            | RestGlyph::Eighth
            | RestGlyph::Sixteenth
            | RestGlyph::Other => RestDisplayAnchor {
                step: "E",
                octave: 4,
            },
            RestGlyph::ThirtySecond | RestGlyph::SixtyFourthOrShorter => RestDisplayAnchor {
                step: "G",
                octave: 4,
            },
        });
        FixedRestPlacementPlan {
            anchor,
            placement_class: format!(
                "fixed_visual_table_{}_{}_{}_rest",
                staff_label,
                clef.placement_label(),
                glyph.fixed_visual_class()
            ),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn anchor(clef: &str, glyph: &str, mode: RestPlacementMode) -> Option<(&'static str, i32)> {
            anchor_from_musicxml(clef, glyph, mode)
                .map(|value| (value.step, value.octave))
        }

        #[test]
        fn canonical_whole_uses_renderer_auto_equivalent_middle_line_in_both_piano_clefs() {
            assert_eq!(
                anchor("F", "whole", RestPlacementMode::Canonical),
                Some(("D", 3))
            );
            assert_eq!(
                anchor("G", "whole", RestPlacementMode::Canonical),
                Some(("B", 4))
            );
        }

        #[test]
        fn polyphonic_whole_rest_separation_remains_distinct() {
            assert_eq!(
                anchor("F", "whole", RestPlacementMode::PolyphonicWholeRest),
                Some(("A", 3))
            );
            assert_eq!(
                anchor("G", "whole", RestPlacementMode::PolyphonicWholeRest),
                Some(("F", 5))
            );
        }

        #[test]
        fn canonical_extended_whole_and_short_rest_slots_preserve_fixed_contract() {
            assert_eq!(
                anchor("F", "breve", RestPlacementMode::Canonical),
                Some(("F", 3))
            );
            assert_eq!(
                anchor("G", "breve", RestPlacementMode::Canonical),
                Some(("D", 5))
            );
            assert_eq!(
                anchor("G", "quarter", RestPlacementMode::Canonical),
                Some(("B", 4))
            );
            assert_eq!(
                anchor("F", "32nd", RestPlacementMode::Canonical),
                Some(("D", 3))
            );
            assert_eq!(
                anchor("G", "32nd", RestPlacementMode::Canonical),
                Some(("D", 5))
            );
            let c_clef_half = fixed_staff_rest_placement_plan("1", "C", "half");
            assert_eq!(c_clef_half.anchor.step, "C");
            assert_eq!(c_clef_half.anchor.octave, 4);
            assert_eq!(
                c_clef_half.placement_class,
                "fixed_visual_table_staff1_cclef_half_rest"
            );
        }
    }
}
#[path = "render/final_notation_readback_observation_core.rs"]
mod final_notation_readback_observation_core;
#[path = "render/silent_voice_coverage_planning_core.rs"]
mod silent_voice_coverage_planning_core;
#[path = "render/final_beam_observation_planning_core.rs"]
mod final_beam_observation_planning_core;
#[path = "render/silent_voice_visible_measure_coverage_render_writer.rs"]
mod silent_voice_visible_measure_coverage_render_writer;
pub(crate) use silent_voice_visible_measure_coverage_render_writer::SameLaneRawSpliceExactMint;
#[path = "structural_write_authority/final_pdf_source_commit.rs"]
pub(super) mod final_pdf_source_commit;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderStatus {
    Written,
    WrittenButNonZeroExit,
    NoRenderer,
    RendererFailed,
}
#[derive(Debug, Clone)]
pub struct RenderResult {
    pub message: String,
    pub status: RenderStatus,
    pub(super) probe_identity: Option<RendererProbeIdentity>,
}
// Actual invocation data from the existing renderer success, not ambient
// identity readback. The oracle instance owns the fixed process/profile scope.
#[derive(Clone, PartialEq, Eq)]
pub(super) struct RendererProbeIdentity {
    executable: String,
    command: String,
}
impl std::fmt::Debug for RendererProbeIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RendererProbeIdentity(bound_invocation)")
    }
}
/// v0.1.145 PostRenderReportPolish:
/// Liefert die tatsächlich im PDF-Artefakt vorhandene Seitenzahl, ohne eine
/// zusätzliche PDF-Abhängigkeit einzuführen. Für von MuseScore erzeugte PDFs
/// reicht die Suche nach Page-Objekten (`/Type /Page`, aber nicht `/Pages`)
/// als robuste PostRender-Diagnose. Bei ungewöhnlich komprimierten/anders
/// strukturierten PDFs gibt die Funktion None zurück statt zu raten.
pub fn count_pdf_pages(path: &Path) -> Option<usize> {
    let bytes = fs::read(path).ok()?;
    let needle = b"/Type /Page";
    let mut count = 0usize;
    let mut pos = 0usize;
    while pos + needle.len() <= bytes.len() {
        let Some(rel) = bytes[pos..].windows(needle.len()).position(|win| win == needle) else {
            break;
        };
        let idx = pos + rel;
        let next = bytes.get(idx + needle.len()).copied();
        // `/Type /Pages` ist der Seitenbaum, kein einzelnes Seitenobjekt.
        if next != Some(b's') {
            count += 1;
        }
        pos = idx + needle.len();
    }
    (count > 0).then_some(count)
}
#[derive(Debug, Clone)]
struct RendererCandidate {
    executable: String,
    source: &'static str,
}
#[derive(Debug, Clone)]
struct RenderCommandVariant {
    label: &'static str,
    args: Vec<String>,
}
#[derive(Debug, Clone)]
struct PdfOutputSnapshot {
    exists: bool,
    is_file: bool,
    bytes: u64,
    fnv64: String,
    modified_unix_ms: String,
}
#[derive(Debug, Clone)]
struct PdfWaitOutcome {
    confirmed_exists: bool,
    first_observed_after_process_ms: Option<u128>,
    second_presence_after_process_ms: Option<u128>,
    changed_during_settle: bool,
    samples: usize,
    second_presence_snapshot: PdfOutputSnapshot,
}
#[derive(Debug, Clone)]
struct PdfStructureProbe {
    readable: bool,
    header_valid: bool,
    eof_marker_present: bool,
    page_count: Option<usize>,
    status: &'static str,
}
#[derive(Debug, Clone)]
struct RendererProcessLifecycle {
    process_started_unix_ms: String,
    process_finished_unix_ms: String,
    process_elapsed_ms: u128,
    exit_code: Option<i32>,
    exit_code_hex: String,
    exit_class: &'static str,
    stdout_bytes: usize,
    stderr_bytes: usize,
    stdout_compact: String,
    stderr_compact: String,
    output_immediately_after_process: PdfOutputSnapshot,
    wait_outcome: PdfWaitOutcome,
    output_structure: PdfStructureProbe,
}
fn load_musicxml_bytes_for_renderer_guard(path: &Path) -> Option<Vec<u8>> {
    fs::read(path).ok()
}

pub fn render_pdf_if_possible(
    input_musicxml: &Path,
    output_pdf: &Path,
    final_commit: &final_pdf_source_commit::FinalPdfSourceCommit,
) -> RenderResult {
    if !final_commit.closed() {
        let _ = fs::remove_file(output_pdf);
        return RenderResult {
            message: format!(
                "{}\nFinalRenderCommittedStateClosure v{}: PreRenderFinalization BLOCKIERT; RendererMusicXmlWrites 0; MuseScore nicht gestartet.",
                final_commit.pipeline_line(),
                env!("CARGO_PKG_VERSION")
            ),
            status: RenderStatus::RendererFailed,
            probe_identity: None,
        };
    }

    let persisted_schema_gate_line =
        if super::persisted_schema_validation::is_registered_final_renderer_boundary(
            input_musicxml,
            output_pdf,
        ) {
            match super::persisted_schema_validation::validate_registered_final_artifacts() {
                Ok(Some(summary)) => Some(summary),
                Ok(None) => None,
                Err(error) => {
                    eprintln!("ETUSCHLOPF: [XSD-GATE   ] FEHLER: {error}");
                    return RenderResult {
                        message: format!(
                            "SchemaSchlopf r020 PersistedByteXsdGate: BLOCKIERT; MuseScore nicht gestartet; {error}"
                        ),
                        status: RenderStatus::RendererFailed,
            probe_identity: None,
                    };
                }
            }
        } else {
            None
        };

    let before_renderer = load_musicxml_bytes_for_renderer_guard(input_musicxml);
    let mut rendered = render_finalized_pdf_if_possible(input_musicxml, output_pdf);
    let after_renderer = load_musicxml_bytes_for_renderer_guard(input_musicxml);
    let renderer_musicxml_unchanged =
        before_renderer.is_some() && before_renderer == after_renderer;
    if !renderer_musicxml_unchanged {
        rendered.status = RenderStatus::RendererFailed;
    }
    rendered.message = format!(
        "{}\nFinalRenderCommittedStateClosure v{}: PreRenderFinalization GREEN; RendererMusicXmlWrites {}.\n{}",
        final_commit.pipeline_line(),
        env!("CARGO_PKG_VERSION"),
        usize::from(!renderer_musicxml_unchanged),
        rendered.message
    );
    if let Some(schema_gate_line) = persisted_schema_gate_line {
        println!("ETUSCHLOPF: [XSD-GATE   ] {schema_gate_line}");
        rendered.message = format!("{schema_gate_line}\n{}", rendered.message);
    }
    rendered
}

fn render_finalized_pdf_if_possible(input_musicxml: &Path, output_pdf: &Path) -> RenderResult {
    let candidates = discover_renderer_candidates();
    let mut attempts = Vec::new();
    let mut found_candidates = 0usize;
    let input_abs = absolute_path(input_musicxml);
    let output_abs = absolute_path(output_pdf);
    let before_render = pdf_output_snapshot_v994(&output_abs);
    let delete_result = fs::remove_file(&output_abs);
    let delete_status = match &delete_result {
        Ok(()) => "removed_existing_or_path".to_string(),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => "not_found_before_render".to_string(),
        Err(err) => format!("delete_failed_{err}"),
    };
    let after_delete = pdf_output_snapshot_v994(&output_abs);
    for candidate in candidates {
        let Some(executable) = validate_renderer_executable(&strip_optional_quotes(&candidate.executable), candidate.source) else {
            continue;
        };
        found_candidates += 1;
        match try_render_with_candidate(&executable, candidate.source, &input_abs, &output_abs) {
            RenderAttempt::Success {
                command_label,
                command_text,
                exit_code,
                lifecycle,
                fallback_timing,
                total_variants,
            } => {
                let after_render = pdf_output_snapshot_v994(&output_abs);
                let renderer_exit_zero = renderer_exit_code_is_zero(exit_code);
                let renderer_final_status = if renderer_exit_zero {
                    "Written"
                } else {
                    "WrittenButNonZeroExit"
                };
                let _ = write_pdf_output_freshness_runtime_artifact_v994(PdfOutputFreshnessRuntimeInput {
                    input_musicxml: &input_abs,
                    output_pdf: &output_abs,
                    renderer_final_status,
                    delete_status: &delete_status,
                    before_render: &before_render,
                    after_delete: &after_delete,
                    after_render: &after_render,
                    renderer_candidates_found: found_candidates,
                    winning_executable: &executable,
                    winning_source: candidate.source,
                    winning_command_label: command_label,
                    winning_command_text: &command_text,
                    winning_exit_code: exit_code,
                    lifecycle: Some(lifecycle.as_ref()),
                    attempts: &attempts,
                });
                let exit_label = exit_code
                    .map_or_else(|| "none".to_string(), |value| value.to_string());
                let probe_identity = Some(RendererProbeIdentity {
                    executable: executable.clone(), command: command_text.clone(),
                });
                return RenderResult {
                    probe_identity,
                    message: if renderer_exit_zero {
                        format!(
                            "PDF: {} geschrieben. Renderer: '{}' ({}). Aufruf: {}. ExitCode 0.\n{}",
                            output_abs.display(),
                            executable,
                            candidate.source,
                            command_label,
                            fallback_timing.line(total_variants)
                        )
                    } else {
                        format!(
                            "PDF: {} wurde erzeugt, aber Renderer-ExitCode ist {}. Renderer: '{}' ({}). Aufruf: {}.\n{}",
                            output_abs.display(),
                            exit_label,
                            executable,
                            candidate.source,
                            command_label,
                            fallback_timing.line(total_variants)
                        )
                    },
                    status: if renderer_exit_zero {
                        RenderStatus::Written
                    } else {
                        RenderStatus::WrittenButNonZeroExit
                    },
                };
            }
            RenderAttempt::NotFound => {
                // Nächsten Kandidaten probieren.
            }
            RenderAttempt::Failed(lines) => {
                attempts.extend(lines);
            }
        }
    }
    if found_candidates > 0 || !attempts.is_empty() {
        let after_render = pdf_output_snapshot_v994(&output_abs);
        let _ = write_pdf_output_freshness_runtime_artifact_v994(PdfOutputFreshnessRuntimeInput {
            input_musicxml: &input_abs,
            output_pdf: &output_abs,
            renderer_final_status: "RendererFailed",
            delete_status: &delete_status,
            before_render: &before_render,
            after_delete: &after_delete,
            after_render: &after_render,
            renderer_candidates_found: found_candidates,
            winning_executable: "none",
            winning_source: "none",
            winning_command_label: "none",
            winning_command_text: "none",
            winning_exit_code: None,
            lifecycle: None,
            attempts: &attempts,
        });
        let details = compact_attempts(&attempts);
        return RenderResult {
            message: format!(
                "PDF: Renderer gefunden, aber Export fehlgeschlagen. {details} Gesucht: ETUSCHLOPF_PDF_RENDERER, PATH, Windows-Standardpfade, Registry/App-Paths."
            ),
            status: RenderStatus::RendererFailed,
            probe_identity: None,
        };
    }
    let after_render = pdf_output_snapshot_v994(&output_abs);
    let _ = write_pdf_output_freshness_runtime_artifact_v994(PdfOutputFreshnessRuntimeInput {
        input_musicxml: &input_abs,
        output_pdf: &output_abs,
        renderer_final_status: "NoRenderer",
        delete_status: &delete_status,
        before_render: &before_render,
        after_delete: &after_delete,
        after_render: &after_render,
        renderer_candidates_found: found_candidates,
        winning_executable: "none",
        winning_source: "none",
        winning_command_label: "none",
        winning_command_text: "none",
        winning_exit_code: None,
        lifecycle: None,
        attempts: &attempts,
    });
    RenderResult {
        message: "PDF: übersprungen, kein MuseScore/mscore-Renderer gefunden. Gesucht: ETUSCHLOPF_PDF_RENDERER, PATH, Windows-Standardpfade, Registry/App-Paths."
            .to_string(),
        status: RenderStatus::NoRenderer,
        probe_identity: None,
    }
}
struct PdfOutputFreshnessRuntimeInput<'a> {
    input_musicxml: &'a Path,
    output_pdf: &'a Path,
    renderer_final_status: &'a str,
    delete_status: &'a str,
    before_render: &'a PdfOutputSnapshot,
    after_delete: &'a PdfOutputSnapshot,
    after_render: &'a PdfOutputSnapshot,
    renderer_candidates_found: usize,
    winning_executable: &'a str,
    winning_source: &'a str,
    winning_command_label: &'a str,
    winning_command_text: &'a str,
    winning_exit_code: Option<i32>,
    lifecycle: Option<&'a RendererProcessLifecycle>,
    attempts: &'a [String],
}
fn pdf_render_input_binding_kind_v992(actual_input: &Path, master_musicxml: &Path, render_musicxml: &Path) -> String {
    if pdf_render_input_binding_paths_equal_v992(actual_input, render_musicxml) {
        "render_musicxml".to_string()
    } else if pdf_render_input_binding_paths_equal_v992(actual_input, master_musicxml) {
        "master_musicxml".to_string()
    } else {
        "other_or_staged_musicxml".to_string()
    }
}
fn pdf_render_input_binding_paths_equal_v992(left: &Path, right: &Path) -> bool {
    if left == right {
        return true;
    }
    let left_abs = absolute_path(left);
    let right_abs = absolute_path(right);
    left_abs == right_abs
}
fn pdf_render_input_binding_yes_no_v992(value: bool) -> &'static str {
    if value { "ja" } else { "nein" }
}
fn pdf_render_input_binding_file_len_v992(path: &Path) -> u64 {
    fs::metadata(path).map(|meta| meta.len()).unwrap_or(0)
}
fn pdf_render_input_binding_file_hash_v992(path: &Path) -> String {
    let bytes = fs::read(path).unwrap_or_default();
    format!("fnv64:{:016x}", pdf_render_input_binding_fnv1a64_v992(&bytes))
}
fn pdf_render_input_binding_fnv1a64_v992(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}
fn pdf_render_input_binding_safe_component_v992(value: &str) -> String {
    let mut out = String::new();
    let mut previous_separator = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            previous_separator = false;
        } else if !previous_separator {
            out.push('_');
            previous_separator = true;
        }
    }
    let cleaned = out.trim_matches('_').to_string();
    if cleaned.is_empty() {
        "target".to_string()
    } else {
        cleaned
    }
}
fn pdf_render_input_binding_alias_components_v992(value: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    for candidate in [
        pdf_render_input_binding_safe_component_v992(value),
        pdf_render_input_binding_translit_component_v992(value),
    ] {
        if !aliases.iter().any(|existing| existing == &candidate) {
            aliases.push(candidate);
        }
    }
    aliases
}
fn pdf_render_input_binding_translit_component_v992(value: &str) -> String {
    let mut out = String::new();
    let mut previous_separator = false;
    for ch in value.chars() {
        let mapped = match ch {
            'ä' | 'Ä' => Some("ae"),
            'ö' | 'Ö' => Some("oe"),
            'ü' | 'Ü' => Some("ue"),
            'ß' => Some("ss"),
            _ => None,
        };
        if let Some(part) = mapped {
            out.push_str(part);
            previous_separator = false;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            previous_separator = false;
        } else if !previous_separator {
            out.push('_');
            previous_separator = true;
        }
    }
    let cleaned = out.trim_matches('_').to_string();
    if cleaned.is_empty() {
        "target".to_string()
    } else {
        cleaned
    }
}
fn write_pdf_output_freshness_runtime_artifact_v994(input: PdfOutputFreshnessRuntimeInput<'_>) -> std::io::Result<()> {
    let Some(project_root) = input.output_pdf.parent() else {
        return Ok(());
    };
    let Some(stem) = input.output_pdf.file_stem().and_then(|value| value.to_str()) else {
        return Ok(());
    };
    let master_musicxml = project_root.join(format!("{stem}.musicxml"));
    let render_musicxml = project_root.join(format!("{stem}.render.musicxml"));
    let input_kind = pdf_render_input_binding_kind_v992(input.input_musicxml, &master_musicxml, &render_musicxml);
    let output_created_after_delete = !input.after_delete.exists && input.after_render.exists;
    let output_byte_equal_before_after = input.before_render.exists
        && input.after_render.exists
        && input.before_render.bytes == input.after_render.bytes
        && input.before_render.fnv64 == input.after_render.fnv64;
    let output_mtime_changed = input.before_render.modified_unix_ms != input.after_render.modified_unix_ms;
    let freshness_status = pdf_output_freshness_status_v994(
        input.renderer_final_status,
        input.after_delete,
        input.after_render,
    );
    let renderer_winning_exit_zero = renderer_exit_zero_label(input.winning_exit_code);
    let out_dir = project_root
        .join("target")
        .join("etuschlopf_pdf_output_freshness_runtime");
    fs::create_dir_all(&out_dir)?;
    let attempts_compact = compact_attempts(input.attempts);
    let renderer_identity = renderer_executable_identity(input.winning_executable);

    let mut content = format!(
        "ETUSCHLOPF PDF OUTPUT FRESHNESS RUNTIME ARTIFACT\n\
Version: {}\n\
Project: {}\n\
Purpose: Runtime-Beweis direkt aus render_pdf_if_possible(): Input-Fingerprint, Output-PDF vor/nach Delete, Output-PDF nach Renderer, Renderer-Exit und Command.\n\
Boundary: this freshness-artifact writer does not mutate MusicXML; v0.10.1341 may already have reused the existing KeySignatureSpan sink before the external renderer.\n\
Instrumentation: render_rs_output_freshness_and_process_lifecycle_v1110\n\
InputMusicXmlPath: {}\n\
InputMusicXmlKind: {}\n\
InputMusicXmlExists: {}\n\
InputMusicXmlBytes: {}\n\
InputMusicXmlFnv64: {}\n\
OutputPdfPath: {}\n\
DeleteBeforeRenderStatus: {}\n\
SnapshotBeforeRender: {}\n\
SnapshotAfterDelete: {}\n\
SnapshotAfterRender: {}\n\
RendererFinalStatus: {}\n\
RendererCandidatesFound: {}\n\
RendererExecutableIdentity: {renderer_identity}\n\
RendererWinningExecutable: {}\n\
RendererWinningSource: {}\n\
RendererWinningCommandLabel: {}\n\
RendererWinningCommandText: {}\n\
RendererWinningExitCode: {}\n\
RendererWinningExitZero: {}\n\
OutputCreatedAfterDelete: {}\n\
OutputByteEqualBeforeAfter: {}\n\
OutputMtimeChangedBeforeAfter: {}\n\
FreshnessStatus: {}\n\
AttemptsCompact: {}\n",
        env!("CARGO_PKG_VERSION"),
        stem,
        input.input_musicxml.display(),
        input_kind,
        pdf_render_input_binding_yes_no_v992(input.input_musicxml.exists()),
        pdf_render_input_binding_file_len_v992(input.input_musicxml),
        pdf_render_input_binding_file_hash_v992(input.input_musicxml),
        input.output_pdf.display(),
        input.delete_status,
        input.before_render.line(),
        input.after_delete.line(),
        input.after_render.line(),
        input.renderer_final_status,
        input.renderer_candidates_found,
        input.winning_executable,
        input.winning_source,
        input.winning_command_label,
        input.winning_command_text,
        input.winning_exit_code
            .map_or_else(|| "none".to_string(), |value| value.to_string()),
        renderer_winning_exit_zero,
        pdf_render_input_binding_yes_no_v992(output_created_after_delete),
        pdf_render_input_binding_yes_no_v992(output_byte_equal_before_after),
        pdf_render_input_binding_yes_no_v992(output_mtime_changed),
        freshness_status,
        attempts_compact
    );
    content.push_str(&renderer_process_lifecycle_report_block_v1110(
        input.lifecycle,
        input.output_pdf,
    ));
    for component in pdf_render_input_binding_alias_components_v992(stem) {
        let out_file = out_dir.join(format!("pdf_output_freshness_{component}.txt"));
        fs::write(out_file, &content)?;
    }
    Ok(())
}
fn renderer_process_lifecycle_report_block_v1110(
    lifecycle: Option<&RendererProcessLifecycle>,
    output_pdf: &Path,
) -> String {
    let fallback_structure = pdf_structure_probe_v1110(output_pdf);
    let Some(lifecycle) = lifecycle else {
        return format!(
            "\nRENDERER-PROCESS-LIFECYCLE-AUDIT\n\
RendererProcessLifecycleAvailable: nein\n\
RendererProcessWaitModel: blocking_command_output_existing_behavior_preserved\n\
RendererProcessTimeoutPolicy: none_instrumentation_only_no_execution_semantics_change\n\
OutputStructureReadable: {}\n\
OutputPdfHeaderValid: {}\n\
OutputPdfEofMarkerPresent: {}\n\
OutputPdfPageCount: {}\n\
OutputStructureStatus: {}\n\
RendererLifecycleInterpretation: no_winning_renderer_process_lifecycle_available\n",
            pdf_render_input_binding_yes_no_v992(fallback_structure.readable),
            pdf_render_input_binding_yes_no_v992(fallback_structure.header_valid),
            pdf_render_input_binding_yes_no_v992(fallback_structure.eof_marker_present),
            fallback_structure
                .page_count
                .map_or_else(|| "none".to_string(), |value| value.to_string()),
            fallback_structure.status,
        );
    };
    let output_present_at_process_end = lifecycle.output_immediately_after_process.exists
        && lifecycle.output_immediately_after_process.is_file
        && lifecycle.output_immediately_after_process.bytes > 0;
    format!(
        "\nRENDERER-PROCESS-LIFECYCLE-AUDIT\n\
RendererProcessLifecycleAvailable: ja\n\
RendererProcessWaitModel: blocking_command_output_existing_behavior_preserved\n\
RendererProcessTimeoutPolicy: none_instrumentation_only_no_execution_semantics_change\n\
RendererProcessStartedUnixMs: {}\n\
RendererProcessFinishedUnixMs: {}\n\
RendererProcessElapsedMs: {}\n\
RendererProcessExitCode: {}\n\
RendererProcessExitCodeHex: {}\n\
RendererProcessExitClass: {}\n\
RendererProcessStdoutBytes: {}\n\
RendererProcessStderrBytes: {}\n\
RendererProcessStdoutCompact: {}\n\
RendererProcessStderrCompact: {}\n\
OutputImmediatelyAfterProcess: {}\n\
OutputPresentAtProcessEnd: {}\n\
OutputFirstObservedAfterProcessMs: {}\n\
OutputSecondPresenceAfterProcessMs: {}\n\
OutputChangedDuringSettle: {}\n\
OutputContentStableAcrossSettle: {}\n\
OutputSettleSamples: {}\n\
OutputSecondPresenceSnapshot: {}\n\
OutputStructureReadable: {}\n\
OutputPdfHeaderValid: {}\n\
OutputPdfEofMarkerPresent: {}\n\
OutputPdfPageCount: {}\n\
OutputStructureStatus: {}\n\
RendererLifecycleInterpretation: {}\n\
Boundary: audit_only_no_renderer_exit_acceptance_no_music_mutation_no_master_apply\n",
        lifecycle.process_started_unix_ms,
        lifecycle.process_finished_unix_ms,
        lifecycle.process_elapsed_ms,
        lifecycle
            .exit_code
            .map_or_else(|| "none".to_string(), |value| value.to_string()),
        lifecycle.exit_code_hex,
        lifecycle.exit_class,
        lifecycle.stdout_bytes,
        lifecycle.stderr_bytes,
        lifecycle.stdout_compact,
        lifecycle.stderr_compact,
        lifecycle.output_immediately_after_process.line(),
        pdf_render_input_binding_yes_no_v992(output_present_at_process_end),
        option_millis_label_v1110(lifecycle.wait_outcome.first_observed_after_process_ms),
        option_millis_label_v1110(lifecycle.wait_outcome.second_presence_after_process_ms),
        pdf_render_input_binding_yes_no_v992(lifecycle.wait_outcome.changed_during_settle),
        pdf_render_input_binding_yes_no_v992(!lifecycle.wait_outcome.changed_during_settle),
        lifecycle.wait_outcome.samples,
        lifecycle.wait_outcome.second_presence_snapshot.line(),
        pdf_render_input_binding_yes_no_v992(lifecycle.output_structure.readable),
        pdf_render_input_binding_yes_no_v992(lifecycle.output_structure.header_valid),
        pdf_render_input_binding_yes_no_v992(lifecycle.output_structure.eof_marker_present),
        lifecycle
            .output_structure
            .page_count
            .map_or_else(|| "none".to_string(), |value| value.to_string()),
        lifecycle.output_structure.status,
        renderer_lifecycle_interpretation_v1110(lifecycle),
    )
}
impl PdfOutputSnapshot {
    fn line(&self) -> String {
        format!(
            "exists {} is_file {} bytes {} fnv64 {} modified_unix_ms {}",
            pdf_render_input_binding_yes_no_v992(self.exists),
            pdf_render_input_binding_yes_no_v992(self.is_file),
            self.bytes,
            self.fnv64,
            self.modified_unix_ms
        )
    }
}
fn pdf_output_snapshot_v994(path: &Path) -> PdfOutputSnapshot {
    let Ok(meta) = fs::metadata(path) else {
        return PdfOutputSnapshot {
            exists: false,
            is_file: false,
            bytes: 0,
            fnv64: "fnv64:missing".to_string(),
            modified_unix_ms: "missing".to_string(),
        };
    };
    let is_file = meta.is_file();
    let bytes = if is_file { meta.len() } else { 0 };
    let fnv64 = if is_file {
        pdf_render_input_binding_file_hash_v992(path)
    } else {
        "fnv64:not_file".to_string()
    };
    let modified_unix_ms = meta
        .modified()
        .ok()
        .and_then(|value| value.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map_or_else(|| "unavailable".to_string(), |duration| duration.as_millis().to_string());
    PdfOutputSnapshot {
        exists: true,
        is_file,
        bytes,
        fnv64,
        modified_unix_ms,
    }
}
fn pdf_output_freshness_status_v994(
    renderer_final_status: &str,
    after_delete: &PdfOutputSnapshot,
    after_render: &PdfOutputSnapshot,
) -> &'static str {
    if renderer_final_status == "Written" && !after_delete.exists && after_render.exists && after_render.bytes > 0 {
        "green_renderer_wrote_fresh_output_after_delete"
    } else if renderer_final_status == "Written" && after_delete.exists {
        "red_renderer_written_but_old_output_survived_delete"
    } else if renderer_final_status == "Written" && !after_render.exists {
        "red_renderer_reported_written_but_output_missing"
    } else if renderer_final_status == "WrittenButNonZeroExit" && !after_delete.exists && after_render.exists && after_render.bytes > 0 {
        "yellow_renderer_wrote_fresh_output_after_delete_but_exit_nonzero"
    } else if renderer_final_status == "WrittenButNonZeroExit" && after_delete.exists {
        "red_renderer_nonzero_exit_and_old_output_survived_delete"
    } else if renderer_final_status == "WrittenButNonZeroExit" && !after_render.exists {
        "red_renderer_nonzero_exit_and_output_missing"
    } else if renderer_final_status == "RendererFailed" {
        "red_renderer_failed_no_fresh_output_proven"
    } else {
        "yellow_no_renderer_no_fresh_output_proven"
    }
}
fn renderer_exit_code_is_zero(code: Option<i32>) -> bool {
    matches!(code, Some(0))
}
fn renderer_exit_zero_label(code: Option<i32>) -> &'static str {
    match code {
        Some(0) => "ja",
        Some(_) => "nein",
        None => "unknown",
    }
}
#[derive(Debug, Clone, Default)]
struct RendererFallbackTimingAudit {
    variants_attempted: usize,
    failed_before_success: usize,
    nonzero_missing_output_short_waits: usize,
    zero_missing_output_long_waits: usize,
    process_ms_before_success: u128,
    settle_ms_before_success: u128,
    failure_evidence: Vec<String>,
}
impl RendererFallbackTimingAudit {
    fn line(&self, total_variants: usize) -> String {
        format!(
            "RendererFallbackTiming: VariantsAttempted {}/{}; FailedBeforeSuccess {}; NonzeroMissingOutputShortWaits {}; ZeroMissingOutputLongWaits {}; ProcessMsBeforeSuccess {}; SettleMsBeforeSuccess {}; FailureEvidence {}; Policy blocking_process_exit_plus_output_presence_bound; MusicMutation nein.",
            self.variants_attempted,
            total_variants,
            self.failed_before_success,
            self.nonzero_missing_output_short_waits,
            self.zero_missing_output_long_waits,
            self.process_ms_before_success,
            self.settle_ms_before_success,
            if self.failure_evidence.is_empty() {
                "none".to_string()
            } else {
                self.failure_evidence.join("|")
            }
        )
    }
}
fn renderer_failure_class_v1751(stderr: &str, stdout: &str) -> &'static str {
    let diagnostic = format!("{stderr}\n{stdout}").to_ascii_lowercase();
    if diagnostic.trim().is_empty() {
        "no_diagnostic_text"
    } else if diagnostic.contains("unknown option")
        || diagnostic.contains("unrecognized option")
        || diagnostic.contains("unknown command line option")
    {
        "cli_option_rejected"
    } else if diagnostic.contains("corrupt")
        || diagnostic.contains("force")
        || diagnostic.contains("version")
    {
        "force_or_version_warning"
    } else if diagnostic.contains("cannot open")
        || diagnostic.contains("failed to open")
        || diagnostic.contains("could not open")
        || diagnostic.contains("error reading")
    {
        "input_open_rejected"
    } else if diagnostic.contains("webengine")
        || diagnostic.contains("opengl")
        || diagnostic.contains("qml")
        || diagnostic.contains("qt")
    {
        "runtime_subsystem_failure"
    } else {
        "other_renderer_error"
    }
}
fn renderer_failure_sample_v1751(stderr: &str, stdout: &str) -> String {
    let diagnostic = if stderr.trim().is_empty() { stdout } else { stderr };
    let compact = one_line(diagnostic);
    let compact = compact.trim();
    if compact.is_empty() {
        "none".to_string()
    } else {
        let mut sample = compact.chars().take(160).collect::<String>();
        if compact.chars().count() > 160 {
            sample.push('…');
        }
        sample.replace('|', "/")
    }
}
#[derive(Debug, Clone)]
enum RenderAttempt {
    Success {
        command_label: &'static str,
        command_text: String,
        exit_code: Option<i32>,
        lifecycle: Box<RendererProcessLifecycle>,
        fallback_timing: RendererFallbackTimingAudit,
        total_variants: usize,
    },
    NotFound,
    Failed(Vec<String>),
}
fn renderer_executable_identity(executable: &str) -> String {
    let requested = Path::new(executable);
    let resolved = if requested.is_absolute() {
        requested.canonicalize().ok()
    } else {
        std::env::var_os("PATH").and_then(|paths| std::env::split_paths(&paths)
            .map(|directory| directory.join(executable))
            .find(|candidate| candidate.is_file())
            .and_then(|candidate| candidate.canonicalize().ok()))
    };
    let Some(path) = resolved else { return "unavailable_path_resolution; authority=none".to_string(); };
    match fs::read(&path) {
        Ok(bytes) => {
            let fingerprint = bytes.iter().fold(0xcbf29ce484222325_u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3));
            format!("path={}; bytes={}; fnv64={fingerprint:016x}; observation=after_winning_process; authority=none", path.display(), bytes.len())
        }
        Err(_) => format!("path={}; bytes_unavailable; authority=none", path.display()),
    }
}
fn validate_renderer_executable(executable: &str, source: &str) -> Option<String> {
    let executable = executable.trim();
    if executable.is_empty()
        || executable.contains('\0')
        || executable.contains('\n')
        || executable.contains('\r')
    {
        return None;
    }
    if !looks_like_path(executable) {
        let known = renderer_exe_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case(executable));
        return known.then(|| executable.to_string());
    }
    let path = Path::new(executable);
    if !path.is_absolute() {
        // Registry/App-Paths und Umgebungsvariablen dürfen keine relativen oder
        // argumentartigen Pfadfragmente einschleusen. Command::new verhindert
        // Shell-Injection, aber absolute, existierende Dateien sind wartbarer
        // und im Report eindeutig nachvollziehbar.
        return None;
    }
    if !path.is_file() {
        return None;
    }
    if cfg!(windows) {
        let is_exe = path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));
        if !is_exe {
            return None;
        }
    }
    // Für Registry/App-Paths zusätzlich grob prüfen, dass der Dateiname wie ein
    // MuseScore-Renderer aussieht. Damit bleibt ein versehentlich falscher
    // App-Path sichtbar aus dem Kandidatenraum draußen.
    if source.contains("Registry") {
        let file_name = path.file_name()?.to_string_lossy().to_ascii_lowercase();
        if !(file_name.contains("musescore") || file_name.contains("mscore")) {
            return None;
        }
    }
    Some(executable.to_string())
}
static RUN_LOCAL_SHADOW_RENDERER_WINNING_COMMANDS: OnceLock<Mutex<BTreeMap<String, String>>> =
    OnceLock::new();

fn run_local_shadow_renderer_scope_key(executable: &str, output_pdf: &Path) -> Option<String> {
    let shadow_root = "etuschlopf_renderer_feasibility_shadow";
    let is_shadow_renderer_output = output_pdf.components().any(|component| {
        component
            .as_os_str()
            .to_string_lossy()
            .eq_ignore_ascii_case(shadow_root)
    });
    if !is_shadow_renderer_output {
        return None;
    }
    let parent = output_pdf.parent()?;
    Some(format!(
        "{}|{}",
        executable.to_ascii_lowercase(),
        parent.to_string_lossy().to_ascii_lowercase()
    ))
}

fn take_run_local_shadow_renderer_winning_command(
    executable: &str,
    output_pdf: &Path,
) -> Option<String> {
    let key = run_local_shadow_renderer_scope_key(executable, output_pdf)?;
    let cache = RUN_LOCAL_SHADOW_RENDERER_WINNING_COMMANDS
        .get_or_init(|| Mutex::new(BTreeMap::new()));
    cache.lock().ok()?.remove(&key)
}

fn remember_run_local_shadow_renderer_winning_command(
    executable: &str,
    output_pdf: &Path,
    command_label: &str,
) {
    let Some(key) = run_local_shadow_renderer_scope_key(executable, output_pdf) else {
        return;
    };
    let cache = RUN_LOCAL_SHADOW_RENDERER_WINNING_COMMANDS
        .get_or_init(|| Mutex::new(BTreeMap::new()));
    if let Ok(mut cache) = cache.lock() {
        cache.insert(key, command_label.to_string());
    }
}

// One project run renders the same score lineage several times with the same
// renderer executable. The command form that last produced a confirmed PDF in
// this process is tried first; on failure the complete unchanged compatibility
// list follows. Process-local only, never persisted, no render authority.
static RUN_LOCAL_RENDERER_WINNING_COMMANDS: OnceLock<Mutex<BTreeMap<String, String>>> =
    OnceLock::new();

fn run_local_renderer_winning_command(executable: &str) -> Option<String> {
    let cache = RUN_LOCAL_RENDERER_WINNING_COMMANDS.get_or_init(|| Mutex::new(BTreeMap::new()));
    cache
        .lock()
        .ok()?
        .get(&executable.to_ascii_lowercase())
        .cloned()
}

fn remember_run_local_renderer_winning_command(executable: &str, command_label: &str) {
    let cache = RUN_LOCAL_RENDERER_WINNING_COMMANDS.get_or_init(|| Mutex::new(BTreeMap::new()));
    if let Ok(mut cache) = cache.lock() {
        cache.insert(executable.to_ascii_lowercase(), command_label.to_string());
    }
}

fn renderer_explicit_cli_reject_immediate_fallback_v1839(
    exit_code: Option<i32>,
    output_present_at_process_end: bool,
    failure_class: &str,
) -> bool {
    matches!(exit_code, Some(code) if code != 0)
        && !output_present_at_process_end
        && failure_class == "cli_option_rejected"
}

fn renderer_post_process_settle_timeout_v1750(
    exit_code: Option<i32>,
    output_present_at_process_end: bool,
) -> Duration {
    // Command::output() is blocking: at this boundary the renderer process has
    // already terminated. A non-zero process that produced no PDF is therefore
    // a failed synchronous attempt, not a reason to idle for 45 seconds before
    // trying the next compatibility form. Keep a short grace window for delayed
    // filesystem visibility or a renderer-owned helper process. Successful exits
    // and already-present PDFs retain the historical 45-second confirmation cap.
    if output_present_at_process_end || renderer_exit_code_is_zero(exit_code) {
        Duration::from_secs(45)
    } else {
        Duration::from_secs(3)
    }
}
fn promote_unforced_renderer_variant(variants: &mut Vec<RenderCommandVariant>, label: &str) -> bool {
    let Some(index) = variants.iter().position(|variant| variant.label == label && !variant.args.iter().any(|arg| arg == "-f")) else { return false; };
    let preferred=variants.remove(index);variants.insert(0,preferred);true
}
fn try_render_with_candidate(
    executable: &str,
    source: &str,
    input_musicxml: &Path,
    output_pdf: &Path,
) -> RenderAttempt {
    // Die Render-Quelle wird unverändert an MuseScore übergeben.
    // Vorzeichnungen dürfen nicht beim Renderer-Aufruf aus einem festen
    // Zwei-Kreuze-Sonderfall entstehen; ihre Autorität liegt in der finalen XML.
    let mut variants = render_command_variants(input_musicxml, output_pdf);
    let total_variants = variants.len();
    let run_local_shadow_hint =
        take_run_local_shadow_renderer_winning_command(executable, output_pdf);
    if let Some(label) = run_local_shadow_hint.as_deref()
        && promote_unforced_renderer_variant(&mut variants, label)
    {
        println!(
            "ETUSCHLOPF: RendererRunLocalShadowWinningCommandReuse: reuse {} before compatibility fallback scan; scope current_process_same_shadow_project; persistent_cache nein; authority none",
            label
        );
    }
    let run_local_process_hint = run_local_renderer_winning_command(executable);
    if run_local_shadow_hint.is_none()
        && let Some(label) = run_local_process_hint.as_deref()
        && promote_unforced_renderer_variant(&mut variants, label)
    {
        println!(
            "ETUSCHLOPF: RendererRunLocalWinningCommandReuse: reuse {label} before compatibility fallback scan; process_local same_executable; persistent_cache nein; authority none"
        );
    }
    let mut failures = Vec::new();
    let mut fallback_timing = RendererFallbackTimingAudit::default();
    for variant in variants {
        fallback_timing.variants_attempted += 1;
        let command_text = format_command(executable, &variant.args);
        match run_renderer_command(executable, &variant.args, None) {
            Ok(run) => {
                let output_immediately_after_process = pdf_output_snapshot_v994(output_pdf);
                let output_present_at_process_end = output_immediately_after_process.exists
                    && output_immediately_after_process.is_file
                    && output_immediately_after_process.bytes > 0;
                let failure_class = renderer_failure_class_v1751(&run.stderr, &run.stdout);
                let immediate_cli_reject_fallback =
                    renderer_explicit_cli_reject_immediate_fallback_v1839(
                        run.code,
                        output_present_at_process_end,
                        failure_class,
                    );
                let settle_timeout = renderer_post_process_settle_timeout_v1750(
                    run.code,
                    output_present_at_process_end,
                );
                if !output_present_at_process_end && !immediate_cli_reject_fallback {
                    if !renderer_exit_code_is_zero(run.code)
                        && settle_timeout < Duration::from_secs(45)
                    {
                        fallback_timing.nonzero_missing_output_short_waits += 1;
                    } else if renderer_exit_code_is_zero(run.code) {
                        fallback_timing.zero_missing_output_long_waits += 1;
                    }
                }
                let (wait_outcome, settle_elapsed_ms) = if immediate_cli_reject_fallback {
                    (
                        PdfWaitOutcome {
                            confirmed_exists: false,
                            first_observed_after_process_ms: None,
                            second_presence_after_process_ms: None,
                            changed_during_settle: false,
                            samples: 0,
                            second_presence_snapshot: output_immediately_after_process.clone(),
                        },
                        0,
                    )
                } else {
                    let settle_started = Instant::now();
                    let wait_outcome =
                        wait_for_output_pdf_diagnostic_v1110(output_pdf, settle_timeout);
                    (wait_outcome, settle_started.elapsed().as_millis())
                };
                let output_structure = pdf_structure_probe_v1110(output_pdf);
                let lifecycle = RendererProcessLifecycle {
                    process_started_unix_ms: run.started_unix_ms,
                    process_finished_unix_ms: run.finished_unix_ms,
                    process_elapsed_ms: run.elapsed_ms,
                    exit_code: run.code,
                    exit_code_hex: renderer_exit_code_hex_v1110(run.code),
                    exit_class: renderer_exit_class_v1110(run.code),
                    stdout_bytes: run.stdout.len(),
                    stderr_bytes: run.stderr.len(),
                    stdout_compact: one_line(&run.stdout),
                    stderr_compact: one_line(&run.stderr),
                    output_immediately_after_process,
                    wait_outcome,
                    output_structure,
                };
                if lifecycle.wait_outcome.confirmed_exists {
                    remember_run_local_shadow_renderer_winning_command(
                        executable,
                        output_pdf,
                        variant.label,
                    );
                    remember_run_local_renderer_winning_command(executable, variant.label);
                    return RenderAttempt::Success {
                        command_label: variant.label,
                        command_text,
                        exit_code: run.code,
                        lifecycle: Box::new(lifecycle),
                        fallback_timing,
                        total_variants,
                    };
                }
                fallback_timing.failed_before_success += 1;
                fallback_timing.process_ms_before_success += run.elapsed_ms;
                fallback_timing.settle_ms_before_success += settle_elapsed_ms;
                fallback_timing.failure_evidence.push(format!(
                    "{}:{}:{}:{}",
                    fallback_timing.variants_attempted,
                    run.code.map_or_else(|| "none".to_string(), |code| code.to_string()),
                    failure_class,
                    renderer_failure_sample_v1751(&run.stderr, &run.stdout)
                ));
                failures.push(format!(
                    "{} ({}): {} | Exit {:?} / {} / {} | Prozess {} ms | PDF nach {} ms Settle-Cap nicht zweifach vorhanden bestaetigt | sofort {} | stderr: {} | stdout: {}",
                    executable,
                    source,
                    command_text,
                    run.code,
                    lifecycle.exit_code_hex,
                    lifecycle.exit_class,
                    lifecycle.process_elapsed_ms,
                    if immediate_cli_reject_fallback {
                        0
                    } else {
                        settle_timeout.as_millis()
                    },
                    lifecycle.output_immediately_after_process.line(),
                    lifecycle.stderr_compact,
                    lifecycle.stdout_compact
                ));
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return RenderAttempt::NotFound,
            Err(err) => {
                fallback_timing.failed_before_success += 1;
                failures.push(format!(
                    "{executable} ({source}): {command_text} | Startfehler: {err}"
                ));
            }
        }
    }
    RenderAttempt::Failed(failures)
}
#[derive(Debug, Clone)]
struct CommandRun {
    code: Option<i32>,
    stdout: String,
    stderr: String,
    started_unix_ms: String,
    finished_unix_ms: String,
    elapsed_ms: u128,
}
fn run_renderer_command(
    executable: &str,
    args: &[String],
    current_dir: Option<&Path>,
) -> std::io::Result<CommandRun> {
    let mut command = Command::new(executable);
    command.args(args);
    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }
    let started_unix_ms = unix_ms_now_string_v1110();
    let started = Instant::now();
    let output = command.output()?;
    let elapsed_ms = started.elapsed().as_millis();
    let finished_unix_ms = unix_ms_now_string_v1110();
    Ok(CommandRun {
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        started_unix_ms,
        finished_unix_ms,
        elapsed_ms,
    })
}
fn render_command_variants(input_musicxml: &Path, output_pdf: &Path) -> Vec<RenderCommandVariant> {
    let input = input_musicxml.to_string_lossy().into_owned();
    let output = output_pdf.to_string_lossy().into_owned();
    // PDF conversion needs neither the synthesizer nor the embedded web view. Keep
    // those crash-prone subsystems out of the primary converter process and use the
    // documented -o converter mode. r1484: no variant uses MuseScore's force option
    // (-f); a score MuseScore rejects as corrupt must fail visibly, never be forced.
    vec![
        RenderCommandVariant {
            label: "-o <pdf> <input>",
            args: vec!["-o".to_string(), output.clone(), input.clone()],
        },
        RenderCommandVariant {
            label: "-s -w -o <pdf> <input>",
            args: vec![
                "-s".to_string(),
                "-w".to_string(),
                "-o".to_string(),
                output.clone(),
                input.clone(),
            ],
        },
        RenderCommandVariant {
            label: "-s -o <pdf> <input>",
            args: vec![
                "-s".to_string(),
                "-o".to_string(),
                output.clone(),
                input.clone(),
            ],
        },
        RenderCommandVariant {
            label: "-w -o <pdf> <input>",
            args: vec![
                "-w".to_string(),
                "-o".to_string(),
                output.clone(),
                input.clone(),
            ],
        },
        RenderCommandVariant {
            label: "--export-to <pdf> <input>",
            args: vec![
                "--export-to".to_string(),
                output.clone(),
                input.clone(),
            ],
        },
        RenderCommandVariant {
            label: "<input> -o <pdf>",
            args: vec![input.clone(), "-o".to_string(), output.clone()],
        },
        RenderCommandVariant {
            label: "<input> --export-to <pdf>",
            args: vec![
                input.clone(),
                "--export-to".to_string(),
                output.clone(),
            ],
        },
    ]
}
fn wait_for_output_pdf_diagnostic_v1110(
    output_pdf: &Path,
    timeout: Duration,
) -> PdfWaitOutcome {
    let start = Instant::now();
    let mut first_observed_after_process_ms = None;
    let mut previous_existing: Option<PdfOutputSnapshot> = None;
    let mut last_snapshot: Option<PdfOutputSnapshot> = None;
    let mut changed_during_settle = false;
    let mut samples = 0usize;
    while start.elapsed() <= timeout {
        let snapshot = pdf_output_snapshot_v994(output_pdf);
        samples += 1;
        last_snapshot = Some(snapshot.clone());
        if snapshot.exists && snapshot.is_file && snapshot.bytes > 0 {
            let elapsed_ms = start.elapsed().as_millis();
            if first_observed_after_process_ms.is_none() {
                first_observed_after_process_ms = Some(elapsed_ms);
            }
            if let Some(previous) = &previous_existing {
                changed_during_settle = !pdf_output_snapshots_content_equal_v1110(previous, &snapshot);
                return PdfWaitOutcome {
                    confirmed_exists: true,
                    first_observed_after_process_ms,
                    second_presence_after_process_ms: Some(elapsed_ms),
                    changed_during_settle,
                    samples,
                    second_presence_snapshot: snapshot,
                };
            }
            previous_existing = Some(snapshot);
            thread::sleep(Duration::from_millis(250));
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }
    PdfWaitOutcome {
        confirmed_exists: false,
        first_observed_after_process_ms,
        second_presence_after_process_ms: None,
        changed_during_settle,
        samples,
        second_presence_snapshot: last_snapshot.unwrap_or_else(|| pdf_output_snapshot_v994(output_pdf)),
    }
}
#[cfg(test)]
mod renderer_explicit_cli_reject_immediate_fallback_tests_v1839 {
    use super::renderer_explicit_cli_reject_immediate_fallback_v1839;

    #[test]
    fn explicit_cli_reject_nonzero_without_pdf_skips_settle() {
        assert!(renderer_explicit_cli_reject_immediate_fallback_v1839(
            Some(1),
            false,
            "cli_option_rejected"
        ));
    }

    #[test]
    fn generic_nonzero_without_pdf_keeps_existing_settle() {
        assert!(!renderer_explicit_cli_reject_immediate_fallback_v1839(
            Some(1320),
            false,
            "no_diagnostic_text"
        ));
    }

    #[test]
    fn cli_text_never_bypasses_when_pdf_already_exists_or_exit_is_not_nonzero() {
        assert!(!renderer_explicit_cli_reject_immediate_fallback_v1839(
            Some(1),
            true,
            "cli_option_rejected"
        ));
        assert!(!renderer_explicit_cli_reject_immediate_fallback_v1839(
            Some(0),
            false,
            "cli_option_rejected"
        ));
        assert!(!renderer_explicit_cli_reject_immediate_fallback_v1839(
            None,
            false,
            "cli_option_rejected"
        ));
    }
}

#[cfg(test)]
mod renderer_run_local_winning_command_tests {
    use super::{remember_run_local_renderer_winning_command, run_local_renderer_winning_command};

    #[test]
    fn winning_command_is_remembered_per_executable_in_process() {
        let executable = "C:/Test/RunLocalWinningRenderer.exe";
        assert!(run_local_renderer_winning_command(executable).is_none());
        remember_run_local_renderer_winning_command(executable, "-f -o <pdf> <input>");
        assert_eq!(
            run_local_renderer_winning_command("c:/test/runlocalwinningrenderer.EXE").as_deref(),
            Some("-f -o <pdf> <input>")
        );
        assert!(run_local_renderer_winning_command("C:/Test/OtherRenderer.exe").is_none());
        remember_run_local_renderer_winning_command(executable, "-o <pdf> <input>");
        assert_eq!(
            run_local_renderer_winning_command(executable).as_deref(),
            Some("-o <pdf> <input>")
        );
    }
}

#[cfg(test)]
mod renderer_settle_policy_tests_v1750 {
    use super::renderer_post_process_settle_timeout_v1750;
    use std::time::Duration;

    #[test]
    fn successful_process_without_immediate_pdf_keeps_historical_wait_cap() {
        assert_eq!(
            renderer_post_process_settle_timeout_v1750(Some(0), false),
            Duration::from_secs(45)
        );
    }

    #[test]
    fn nonzero_process_without_pdf_uses_short_grace_window() {
        assert_eq!(
            renderer_post_process_settle_timeout_v1750(Some(1), false),
            Duration::from_secs(3)
        );
        assert_eq!(
            renderer_post_process_settle_timeout_v1750(None, false),
            Duration::from_secs(3)
        );
    }

    #[test]
    fn already_present_pdf_keeps_confirmation_cap_even_after_nonzero_exit() {
        assert_eq!(
            renderer_post_process_settle_timeout_v1750(Some(1), true),
            Duration::from_secs(45)
        );
    }
}


#[cfg(test)]
mod renderer_failure_evidence_tests_v1751 {
    use super::{renderer_failure_class_v1751, renderer_failure_sample_v1751};

    #[test]
    fn classifies_force_or_version_warning_without_affecting_renderer_policy() {
        assert_eq!(
            renderer_failure_class_v1751("Score version warning: use --force", ""),
            "force_or_version_warning"
        );
    }

    #[test]
    fn classifies_cli_option_rejection_before_generic_runtime_text() {
        assert_eq!(
            renderer_failure_class_v1751("Unknown option: --export-to", ""),
            "cli_option_rejected"
        );
    }

    #[test]
    fn sample_is_single_line_bounded_and_pipe_safe() {
        let sample = renderer_failure_sample_v1751("line one\nline | two", "");
        assert!(!sample.contains('\n'));
        assert!(!sample.contains('|'));
        assert!(sample.chars().count() <= 161);
    }
}

fn pdf_output_snapshots_content_equal_v1110(
    left: &PdfOutputSnapshot,
    right: &PdfOutputSnapshot,
) -> bool {
    left.exists
        && right.exists
        && left.is_file
        && right.is_file
        && left.bytes == right.bytes
        && left.fnv64 == right.fnv64
}
fn pdf_structure_probe_v1110(path: &Path) -> PdfStructureProbe {
    let Ok(bytes) = fs::read(path) else {
        return PdfStructureProbe {
            readable: false,
            header_valid: false,
            eof_marker_present: false,
            page_count: None,
            status: "red_pdf_unreadable",
        };
    };
    let header_valid = bytes.starts_with(b"%PDF-");
    let tail_start = bytes.len().saturating_sub(4096);
    let eof_marker_present = bytes[tail_start..]
        .windows(b"%%EOF".len())
        .any(|window| window == b"%%EOF");
    let page_count = count_pdf_pages(path);
    let status = if header_valid && eof_marker_present && page_count.unwrap_or(0) > 0 {
        "green_basic_pdf_structure_plausible"
    } else if header_valid && eof_marker_present {
        "yellow_pdf_header_and_eof_present_page_count_unavailable"
    } else if header_valid {
        "yellow_pdf_header_present_eof_marker_missing"
    } else {
        "red_pdf_header_missing"
    };
    PdfStructureProbe {
        readable: true,
        header_valid,
        eof_marker_present,
        page_count,
        status,
    }
}
fn unix_ms_now_string_v1110() -> String {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or_else(|_| "unavailable".to_string(), |duration| duration.as_millis().to_string())
}
fn renderer_exit_code_hex_v1110(code: Option<i32>) -> String {
    code.map_or_else(|| "none".to_string(), |value| format!("0x{:08X}", value as u32))
}
fn renderer_exit_class_v1110(code: Option<i32>) -> &'static str {
    match code {
        Some(0) => "success_zero",
        Some(value) if value as u32 == 0xC000_0005 => "windows_ntstatus_access_violation",
        Some(value) if cfg!(windows) && value < 0 => "windows_negative_status_or_ntstatus",
        Some(_) => "nonzero_exit",
        None => "no_numeric_exit_code",
    }
}
fn renderer_lifecycle_interpretation_v1110(
    lifecycle: &RendererProcessLifecycle,
) -> &'static str {
    let output_present_at_process_end = lifecycle.output_immediately_after_process.exists
        && lifecycle.output_immediately_after_process.is_file
        && lifecycle.output_immediately_after_process.bytes > 0;
    let structure_plausible = lifecycle.output_structure.status
        == "green_basic_pdf_structure_plausible";
    if lifecycle.exit_class == "windows_ntstatus_access_violation"
        && output_present_at_process_end
        && structure_plausible
        && !lifecycle.wait_outcome.changed_during_settle
    {
        "access_violation_observed_when_parent_wait_returned_with_structurally_plausible_pdf_already_present_and_content_stable_across_settle"
    } else if lifecycle.exit_class == "windows_ntstatus_access_violation"
        && output_present_at_process_end
        && structure_plausible
    {
        "access_violation_observed_with_pdf_present_at_process_end_content_changed_during_settle_then_basic_structure_plausible"
    } else if lifecycle.exit_class != "success_zero"
        && output_present_at_process_end
        && structure_plausible
    {
        "nonzero_exit_observed_when_parent_wait_returned_with_structurally_plausible_pdf_already_present"
    } else if lifecycle.exit_class != "success_zero"
        && lifecycle.wait_outcome.first_observed_after_process_ms.is_some()
        && structure_plausible
    {
        "nonzero_exit_then_structurally_plausible_pdf_observed_during_post_process_settle_window"
    } else if lifecycle.exit_class == "success_zero" && structure_plausible {
        "zero_exit_and_structurally_plausible_pdf"
    } else {
        "renderer_process_and_pdf_sequence_requires_review"
    }
}
fn option_millis_label_v1110(value: Option<u128>) -> String {
    value
        .map_or_else(|| "none".to_string(), |millis| millis.to_string())
}
fn absolute_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_or_else(|_| path.to_path_buf(), |dir| dir.join(path))
    }
}
fn compact_attempts(attempts: &[String]) -> String {
    if attempts.is_empty() {
        return "Kein konkreter Fehlertext verfügbar.".to_string();
    }
    let mut out = String::new();
    out.push_str("Versuche: ");
    let shown = attempts
        .iter()
        .take(4)
        .map(|line| line.as_str())
        .collect::<Vec<_>>();
    out.push_str(&shown.join(" || "));
    if attempts.len() > shown.len() {
        out.push_str(&format!(" || ... {} weitere Versuche", attempts.len() - shown.len()));
    }
    out
}
fn format_command(executable: &str, args: &[String]) -> String {
    let mut parts = Vec::with_capacity(args.len() + 1);
    parts.push(quote_arg(executable));
    parts.extend(args.iter().map(|arg| quote_arg(arg)));
    parts.join(" ")
}
fn quote_arg(value: &str) -> String {
    if value.contains(' ') || value.contains('\t') || value.contains('"') {
        format!("\"{}\"", value.replace('"', "\\\""))
    } else {
        value.to_string()
    }
}
fn one_line(value: &str) -> String {
    let compact = value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if compact.is_empty() {
        "<leer>".to_string()
    } else if compact.chars().count() > 260 {
        compact.chars().take(260).collect::<String>() + "..."
    } else {
        compact
    }
}
// Read-only startup projection for the packaged EXE. Reuse the renderer's
// own candidates and validation; do not maintain another discovery policy.
pub(super) fn renderer_available() -> bool {
    discover_renderer_candidates().into_iter().any(|candidate| {
        let Some(executable) = validate_renderer_executable(
            &strip_optional_quotes(&candidate.executable), candidate.source,
        ) else { return false; };
        if looks_like_path(&executable) { return Path::new(&executable).is_file(); }
        env::var_os("PATH").is_some_and(|paths|
            env::split_paths(&paths).any(|dir| dir.join(&executable).is_file()))
    })
}

fn discover_renderer_candidates() -> Vec<RendererCandidate> {
    let mut candidates = Vec::new();
    if let Ok(explicit) = env::var("ETUSCHLOPF_PDF_RENDERER")
        && !explicit.trim().is_empty() {
            candidates.push(RendererCandidate {
                executable: explicit,
                source: "ETUSCHLOPF_PDF_RENDERER",
            });
        }
    candidates.extend(registry_candidates());
    candidates.extend(windows_standard_path_candidates());
    candidates.extend(path_candidates());
    // Fallback: Kommando-Namen direkt versuchen. Das deckt PATH-Fälle ab, bei denen die Datei nicht
    // per env::split_paths sichtbar ist oder ein Shell-Alias/Wrapper existiert.
    for name in renderer_exe_names() {
        candidates.push(RendererCandidate {
            executable: name.to_string(),
            source: "Kommando-Name",
        });
    }
    dedupe_candidates(candidates)
}
fn renderer_exe_names() -> &'static [&'static str] {
    &[
        "MuseScore4.exe",
        "MuseScore4",
        "MuseScore Studio 4.exe",
        "MuseScore Studio.exe",
        "MuseScore.exe",
        "MuseScore",
        "MuseScore3.exe",
        "MuseScore3",
        "musescore.exe",
        "musescore",
        "mscore4.exe",
        "mscore4",
        "mscore.exe",
        "mscore",
    ]
}
fn path_candidates() -> Vec<RendererCandidate> {
    let mut out = Vec::new();
    let Some(paths) = env::var_os("PATH") else {
        return out;
    };
    for dir in env::split_paths(&paths) {
        for name in renderer_exe_names() {
            let path = dir.join(name);
            if path.exists() {
                out.push(RendererCandidate {
                    executable: path.to_string_lossy().into_owned(),
                    source: "PATH",
                });
            }
        }
    }
    out
}
fn windows_standard_path_candidates() -> Vec<RendererCandidate> {
    let mut out = Vec::new();
    if !cfg!(windows) {
        return out;
    }
    let mut roots = Vec::new();
    push_env_path(&mut roots, "ProgramFiles");
    push_env_path(&mut roots, "ProgramFiles(x86)");
    push_env_path(&mut roots, "LOCALAPPDATA");
    let relative_paths: &[&[&str]] = &[
        &["MuseScore 4", "bin", "MuseScore4.exe"],
        &["MuseScore 4", "bin", "MuseScore.exe"],
        &["MuseScore Studio 4", "bin", "MuseScore4.exe"],
        &["MuseScore Studio 4", "bin", "MuseScore Studio 4.exe"],
        &["MuseScore Studio", "bin", "MuseScore4.exe"],
        &["MuseScore Studio", "bin", "MuseScore Studio.exe"],
        &["MuseScore 3", "bin", "MuseScore3.exe"],
        &["MuseScore 3", "bin", "MuseScore.exe"],
        &["Programs", "MuseScore 4", "bin", "MuseScore4.exe"],
        &["Programs", "MuseScore Studio 4", "bin", "MuseScore4.exe"],
        &["Programs", "MuseScore Studio", "bin", "MuseScore4.exe"],
    ];
    for root in roots {
        for rel in relative_paths {
            let mut path = root.clone();
            for &part in *rel {
                path.push(part);
            }
            out.push(RendererCandidate {
                executable: path.to_string_lossy().into_owned(),
                source: "Windows-Standardpfad",
            });
        }
    }
    out
}
fn push_env_path(paths: &mut Vec<PathBuf>, var: &str) {
    if let Some(value) = env::var_os(var)
        && !value.is_empty() {
            paths.push(PathBuf::from(value));
        }
}
fn registry_candidates() -> Vec<RendererCandidate> {
    let mut out = Vec::new();
    if !cfg!(windows) {
        return out;
    }
    let exe_names = [
        "MuseScore4.exe",
        "MuseScore Studio 4.exe",
        "MuseScore Studio.exe",
        "MuseScore.exe",
        "MuseScore3.exe",
        "mscore4.exe",
        "mscore.exe",
    ];
    let hive_roots = [
        r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths",
        r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths",
        r"HKLM\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\App Paths",
    ];
    for root in hive_roots {
        for exe in exe_names {
            let key = format!(r"{root}\{exe}");
            if let Some(path) = query_registry_default_value(&key) {
                out.push(RendererCandidate {
                    executable: path,
                    source: "Registry/App-Paths",
                });
            }
        }
    }
    out
}
fn query_registry_default_value(key: &str) -> Option<String> {
    let output = Command::new("reg")
        .arg("query")
        .arg(key)
        .arg("/ve")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if let Some((_, value)) = line.split_once("REG_SZ") {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}
fn dedupe_candidates(candidates: Vec<RendererCandidate>) -> Vec<RendererCandidate> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for candidate in candidates {
        let executable = strip_optional_quotes(&candidate.executable);
        if executable.trim().is_empty() {
            continue;
        }
        let key = if cfg!(windows) {
            executable.to_ascii_lowercase()
        } else {
            executable.clone()
        };
        if seen.insert(key) {
            out.push(RendererCandidate {
                executable,
                source: candidate.source,
            });
        }
    }
    out
}
fn strip_optional_quotes(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}
fn looks_like_path(value: &str) -> bool {
    value.contains('\\') || value.contains('/') || value.contains(':')
}

#[cfg(test)]
mod renderer_identity_and_force_tests {
    use super::*;
    #[test]
    fn cached_force_commands_cannot_lead_but_unforced_commands_can() {
        for label in ["-f -o <pdf> <input>","<input> -f --export-to <pdf>"] {
            let mut variants=render_command_variants(Path::new("input.xml"),Path::new("output.pdf"));
            assert!(!promote_unforced_renderer_variant(&mut variants,label));
            assert!(!variants[0].args.iter().any(|arg|arg=="-f"));
        }
        let mut variants=render_command_variants(Path::new("input.xml"),Path::new("output.pdf"));
        assert!(promote_unforced_renderer_variant(&mut variants,"-w -o <pdf> <input>"));
        assert_eq!(variants[0].label,"-w -o <pdf> <input>");
    }
    #[test]
    fn no_renderer_variant_forces_musescore() {
        let variants=render_command_variants(Path::new("input.xml"),Path::new("output.pdf"));
        assert!(!variants.is_empty());
        for variant in &variants {
            assert!(!variant.args.iter().any(|arg|arg=="-f"),"{}",variant.label);
            assert!(!variant.label.contains("-f "),"{}",variant.label);
        }
    }
    #[test]
    fn renderer_identity_is_bound_to_file_bytes_without_executing_them() {
        let root=Path::new(env!("CARGO_MANIFEST_DIR"));
        let first=renderer_executable_identity(root.join("src/render.rs").to_str().unwrap());
        let second=renderer_executable_identity(root.join("src/cli.rs").to_str().unwrap());
        assert!(first.contains("fnv64="));assert!(second.contains("fnv64="));assert_ne!(first,second);
        assert!(renderer_executable_identity(root.join("missing-renderer-identity-fixture").to_str().unwrap()).contains("unavailable"));
    }
}
