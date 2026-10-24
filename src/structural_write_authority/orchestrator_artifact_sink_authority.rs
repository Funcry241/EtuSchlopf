//! Typed terminal sinks for the orchestrator's already verified artifacts.
//!
//! The public surface is deliberately stage-specific. There is no public raw
//! XML/file writer and no caller-selectable generic effect enum. Each private
//! capability binds the exact path and bytes, consumes itself once, validates
//! complete MusicXML where applicable, and performs an exact readback.

use crate::final_report::FinalizedReport;
use super::authority_primitives::{fingerprint, fingerprint_bytes};
use super::rustig_clef_planning::derive_clef_readability_primary_course_plan;
use super::materialization_verifier::validate_serialized_temporal_materialization;
use crate::master_state_identity::{ExactEdit, ExactTransformationReceipt};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct VerifiedDirectionOnlyCandidateSnapshot {
    base_checkpoint_fingerprint: u64,
    candidate_fingerprint: u64,
    note_count: usize,
    note_beam_fingerprint: u64,
}

/// The frozen #60 function shape after the receipt-only return migration.
pub(crate) type FunctionalRespellingRewrite =
    fn(&str, &str, &crate::master_state_identity::OriginalFunctionalRespellingChains)
        -> (Result<String, String>, Option<ExactTransformationReceipt>);

/// Frozen nested-owner carrier of population owner #60 inside #59. It carries
/// the #60 function and the sealed `TonalFrameFact` bound at the metric stage;
/// #59 consumes it exactly once.
pub struct FunctionalMasterRespellingOperation {
    rewrite: FunctionalRespellingRewrite,
    tonal_frame: Option<crate::pipeline::TonalFrameFact>,
    original_chains: Option<crate::master_state_identity::OriginalFunctionalRespellingChains>,
}

impl FunctionalMasterRespellingOperation {
    pub const fn new(rewrite: FunctionalRespellingRewrite) -> Self {
        Self {
            rewrite,
            tonal_frame: None,
            original_chains: None,
        }
    }

    /// Binds the typed tonal-frame fact of exactly one Render snapshot. `None`
    /// means no Render snapshot exists; #59/#60 then leave the Master untouched.
    pub(crate) fn bind_tonal_frame(
        mut self,
        tonal_frame: Option<crate::pipeline::TonalFrameFact>,
    ) -> Self {
        self.tonal_frame = tonal_frame;
        self
    }

    fn tonal_frame(&self) -> Option<&crate::pipeline::TonalFrameFact> {
        self.tonal_frame.as_ref()
    }

    pub(crate) fn bind_original_chains(mut self,
        original_chains: crate::master_state_identity::OriginalFunctionalRespellingChains) -> Self {
        self.original_chains = Some(original_chains);
        self
    }

    /// Runs #60 against the bound snapshot. Before the frozen `&str` crosses
    /// into #60, the fact is re-derived from exactly those bytes and must equal
    /// the transported fact (fail-closed).
    fn apply(
        self,
        export_input: &str,
        master_xml: &str,
        export_receipt: Option<&ExactTransformationReceipt>,
    ) -> Option<(Result<String, String>, Option<ExactTransformationReceipt>)> {
        let tonal_frame = self.tonal_frame?;
        if crate::pipeline::TonalFrameFact::derive_from_render_snapshot(
            tonal_frame.bound_snapshot(),
        )
        .as_ref()
            != Some(&tonal_frame)
        {
            return Some((
                Err("functional master mirror: transported TonalFrameFact does not match its bound snapshot".to_string()),
                None,
            ));
        }
        let original_chains = self.original_chains.unwrap_or_else(||
            crate::master_state_identity::OriginalFunctionalRespellingChains::received(
                Err("Missing original functional-respelling chain input".to_string())))
            .after_original_master_export(export_input, master_xml, export_receipt);
        Some((self.rewrite)(master_xml, tonal_frame.bound_snapshot(), &original_chains))
    }
}

/// Binds the typed tonal-frame fact for #59/#60 from the Render snapshot that
/// the metric stage read. The retired thread-local Render->Master path is
/// evaluated exactly once for comparison: any divergence is fatal, and the old
/// path never supplies productive truth.
pub(crate) fn bind_tonal_frame_fact_from_render_snapshot(
    render_snapshot: Option<String>,
) -> io::Result<Option<crate::pipeline::TonalFrameFact>> {
    let Some(render_snapshot) = render_snapshot else {
        return Ok(None);
    };
    validate_complete_musicxml(&render_snapshot)?;
    crate::pipeline::TonalFrameFact::derive_from_render_snapshot(&render_snapshot)
        .map(Some)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "TonalFrameFact: Render snapshot fifths could not be parsed",
            )
        })
}

/// Sealed EXACT mint capability of population owner #59.
#[derive(Debug, Clone, Copy)]
pub(crate) struct MasterExportFifthsExactMint {
    _sealed: (),
}

pub fn validate_terminal_render_temporal_postcondition_authorized(path: &Path, sovereign_xml: &str) -> io::Result<()> {
    if !path.file_name().and_then(|v| v.to_str()).is_some_and(|v| v.ends_with(".render.musicxml")) { return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid render artifact path")); }
    let actual = std::fs::read_to_string(path).inspect_err(|_error| { crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::ArtifactReadbackMismatch); })?;
    if actual != sovereign_xml { crate::master_state_identity::count_render_closure_failure(crate::master_state_identity::RenderClosureMetric::ArtifactReadbackMismatch); return Err(io::Error::new(io::ErrorKind::InvalidData, "terminal artifact differs from sovereign bytes")); }
    validate_complete_musicxml(sovereign_xml)?;
    validate_serialized_temporal_materialization(sovereign_xml)
}
pub(super) fn verify_direction_only_candidate(base_checkpoint_xml: &str, candidate_xml: &str) -> Option<VerifiedDirectionOnlyCandidateSnapshot> {
    if base_checkpoint_xml == candidate_xml || validate_complete_musicxml(candidate_xml).is_err() { return None; }
    let base = note_beam_fingerprint(base_checkpoint_xml).ok()?;
    let candidate = note_beam_fingerprint(candidate_xml).ok()?;
    if base != candidate { return None; }
    Some(VerifiedDirectionOnlyCandidateSnapshot { base_checkpoint_fingerprint: fingerprint(base_checkpoint_xml), candidate_fingerprint: fingerprint(candidate_xml), note_count: candidate.0, note_beam_fingerprint: candidate.1 })
}
impl VerifiedDirectionOnlyCandidateSnapshot {
    pub(super) fn matches(&self, base: &str, candidate: &str) -> bool {
        fingerprint(base) == self.base_checkpoint_fingerprint && fingerprint(candidate) == self.candidate_fingerprint
        && note_beam_fingerprint(base).ok() == Some((self.note_count, self.note_beam_fingerprint))
        && note_beam_fingerprint(candidate).ok() == Some((self.note_count, self.note_beam_fingerprint))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderCheckpointStage {
    ClefIslandDecomposition,
    FinalNotation,
    FinalNotationPreparation,
    CursorLaneRepair,
    RetentionReturn,
    DuplicateRestRewrite,
    VisibleRestRole,
    FinalPianoRestGlyph,

    
    SisterFeedbackBeam,
    ExternalFeedbackClosure,
    KeySignatureSpelling,
    DirectionRelocation,
}

impl RenderCheckpointStage {
    fn tag(self) -> &'static str {
        match self {
            Self::ClefIslandDecomposition => "clef_island_decomposition",
            Self::FinalNotation | Self::FinalNotationPreparation => "final_notation",
            Self::CursorLaneRepair => "cursor_lane_repair",
            Self::RetentionReturn => "retention_return",
            Self::DuplicateRestRewrite => "duplicate_rest_rewrite",
            Self::VisibleRestRole => "visible_rest_role",
            Self::FinalPianoRestGlyph => "final_piano_rest_glyph",

            
            Self::SisterFeedbackBeam => "sister_feedback_beam",
            Self::ExternalFeedbackClosure => "external_feedback_closure",
            Self::KeySignatureSpelling => "key_signature_spelling",
            Self::DirectionRelocation => "direction_relocation",
        }
    }
}

#[derive(Debug)]
struct OrchestratedRenderCheckpointCapability<'a> {
    path: PathBuf,
    path_fingerprint: u64,
    candidate_xml: &'a str,
    candidate_fingerprint: u64,
    stage: RenderCheckpointStage,
    stage_fingerprint: u64,
}

impl<'a> OrchestratedRenderCheckpointCapability<'a> {
    fn issue(path: &Path, candidate_xml: &'a str, stage: RenderCheckpointStage) -> io::Result<Self> {
        validate_complete_musicxml(candidate_xml)?;
        let path_text = path.to_string_lossy();
        let file_name = path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
        if path_text.trim().is_empty() || !file_name.ends_with(".render.musicxml") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "render checkpoint path is not a .render.musicxml artifact",
            ));
        }
        Ok(Self {
            path: path.to_path_buf(),
            path_fingerprint: fingerprint(path_text.as_ref()),
            candidate_xml,
            candidate_fingerprint: fingerprint(candidate_xml),
            stage,
            stage_fingerprint: fingerprint(stage.tag()),
        })
    }

    fn write(self) -> io::Result<()> {
        let path_text = self.path.to_string_lossy();
        if fingerprint(path_text.as_ref()) != self.path_fingerprint
            || fingerprint(self.candidate_xml) != self.candidate_fingerprint
            || fingerprint(self.stage.tag()) != self.stage_fingerprint
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "render checkpoint binding changed before write",
            ));
        }
        if self.stage == RenderCheckpointStage::DirectionRelocation {
            validate_serialized_temporal_materialization(self.candidate_xml)?;
        }
        let writer_trace_before = crate::audit_only::writer_trace::capture_before(&self.path);
        std::fs::write(&self.path, self.candidate_xml.as_bytes())?;
        exact_readback(&self.path, self.candidate_xml.as_bytes(), self.candidate_fingerprint)?;
        if self.stage == RenderCheckpointStage::DirectionRelocation {
            validate_serialized_temporal_materialization(self.candidate_xml)?;
        }
        crate::audit_only::writer_trace::record_after(
            self.stage.tag(),
            &self.path,
            writer_trace_before.as_deref(),
            self.candidate_xml,
        );
        Ok(())
    }
}

#[derive(Debug)]
struct MasterMusicXmlOutputCapability<'a> {
    path: PathBuf,
    path_fingerprint: u64,
    candidate_xml: &'a str,
    candidate_fingerprint: u64,
}

impl<'a> MasterMusicXmlOutputCapability<'a> {
    fn issue(path: &Path, candidate_xml: &'a str) -> io::Result<Self> {
        validate_complete_musicxml(candidate_xml)?;
        let path_text = path.to_string_lossy();
        let file_name = path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
        if path_text.trim().is_empty()
            || !file_name.ends_with(".musicxml")
            || file_name.ends_with(".render.musicxml")
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "master MusicXML path is not a master .musicxml artifact",
            ));
        }
        Ok(Self {
            path: path.to_path_buf(),
            path_fingerprint: fingerprint(path_text.as_ref()),
            candidate_xml,
            candidate_fingerprint: fingerprint(candidate_xml),
        })
    }

    fn write(self) -> io::Result<()> {
        let path_text = self.path.to_string_lossy();
        if fingerprint(path_text.as_ref()) != self.path_fingerprint
            || fingerprint(self.candidate_xml) != self.candidate_fingerprint
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "master MusicXML binding changed before write",
            ));
        }
        validate_serialized_temporal_materialization(self.candidate_xml)?;
        std::fs::write(&self.path, self.candidate_xml.as_bytes())?;
        exact_readback(&self.path, self.candidate_xml.as_bytes(), self.candidate_fingerprint)?;
        validate_serialized_temporal_materialization(self.candidate_xml)
    }
}

#[derive(Debug)]
struct ResultReportOutputCapability<'a> {
    path: PathBuf,
    path_fingerprint: u64,
    report_text: &'a str,
    report_fingerprint: u64,
}

impl<'a> ResultReportOutputCapability<'a> {
    fn issue(path: &Path, report_text: &'a str) -> io::Result<Self> {
        let path_text = path.to_string_lossy();
        let file_name = path.file_name().and_then(|value| value.to_str()).unwrap_or_default();
        if path_text.trim().is_empty()
            || !file_name.ends_with(".result.txt")
            || !report_text.starts_with("ETUSCHLOPF-ERGEBNIS")
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "result report binding is invalid",
            ));
        }
        Ok(Self {
            path: path.to_path_buf(),
            path_fingerprint: fingerprint(path_text.as_ref()),
            report_text,
            report_fingerprint: fingerprint(report_text),
        })
    }

    fn write(self) -> io::Result<()> {
        let path_text = self.path.to_string_lossy();
        if fingerprint(path_text.as_ref()) != self.path_fingerprint
            || fingerprint(self.report_text) != self.report_fingerprint
            || !self.report_text.starts_with("ETUSCHLOPF-ERGEBNIS")
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "result report binding changed before write",
            ));
        }
        std::fs::write(&self.path, self.report_text.as_bytes())?;
        exact_readback(&self.path, self.report_text.as_bytes(), self.report_fingerprint)
    }
}

fn validate_complete_musicxml(candidate_xml: &str) -> io::Result<()> {
    if candidate_xml.trim().is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "MusicXML is empty"));
    }
    let document = roxmltree::Document::parse(candidate_xml).map_err(|error| {
        io::Error::new(io::ErrorKind::InvalidData, format!("MusicXML parse failed: {error}"))
    })?;
    if document.root_element().tag_name().name() != "score-partwise" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "MusicXML root is not score-partwise",
        ));
    }
    Ok(())
}

fn exact_readback(path: &Path, expected: &[u8], expected_fingerprint: u64) -> io::Result<()> {
    let readback = std::fs::read(path)?;
    if readback.as_slice() != expected || fingerprint_bytes(readback.as_slice()) != expected_fingerprint {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "authorized artifact readback mismatch",
        ));
    }
    Ok(())
}



fn note_beam_fingerprint(xml: &str) -> io::Result<(usize, u64)> {
    let document = roxmltree::Document::parse(xml).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("MusicXML note/beam fingerprint parse failed: {error}"),
        )
    })?;
    let mut canonical = String::new();
    let mut note_count = 0_usize;
    for note in document.descendants().filter(|node| node.has_tag_name("note")) {
        note_count = note_count.saturating_add(1);
        canonical.push_str(&note_count.to_string());
        canonical.push(':');
        let mut has_beam = false;
        for beam in note.children().filter(|node| node.has_tag_name("beam")) {
            has_beam = true;
            canonical.push_str(beam.attribute("number").unwrap_or("?"));
            canonical.push('=');
            canonical.push_str(beam.text().unwrap_or("").trim());
            canonical.push(';');
        }
        if !has_beam {
            canonical.push('-');
        }
        canonical.push('|');
    }
    Ok((note_count, fingerprint(&canonical)))
}

fn validate_direction_relocation_preserves_note_beams(
    path: &Path,
    candidate_xml: &str,
) -> io::Result<()> {
    if !path.is_file() {
        return Ok(());
    }
    let current_xml = std::fs::read_to_string(path)?;
    let current = note_beam_fingerprint(&current_xml)?;
    let candidate = note_beam_fingerprint(candidate_xml)?;
    if current != candidate {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "direction relocation would change prior note/beam sink: notes {}->{}, beam_fingerprint {:016x}->{:016x}",
                current.0, candidate.0, current.1, candidate.1
            ),
        ));
    }
    Ok(())
}


fn write_render_checkpoint(
    path: &Path,
    candidate_xml: &str,
    stage: RenderCheckpointStage,
) -> (io::Result<()>, Option<ExactTransformationReceipt>) {
    let mut applied_geometry = Vec::new();
    let mut realized_post = None;
    
    let output = (|| {
    // v0.10.1715: ClefReadability planning remains writer-free; the existing
    // FinalNotation checkpoint is still the only render materialization boundary.
    // It now commits the complete F4/G2 course (safe removals plus pure-clef
    // insertions) rather than merely cleaning legacy surfaces.
    let planned_xml = if matches!(stage, RenderCheckpointStage::FinalNotation | RenderCheckpointStage::FinalNotationPreparation) {
        let plan = derive_clef_readability_primary_course_plan(candidate_xml);
        let insertions = plan.insertion_blocks();
        if plan.removal_ranges().is_empty() && insertions.is_empty() {
            None
        } else {
            let mut edits = plan
                .removal_ranges()
                .iter()
                .map(|&(start, end)| (start, Some(end), None))
                .chain(
                    insertions
                        .iter()
                        .map(|&(start, block)| (start, None, Some(block))),
                )
                .collect::<Vec<_>>();
            edits.sort_unstable_by_key(|(start, _, _)| *start);
            let mut xml = candidate_xml.to_string();
            for (start, end, block) in edits.into_iter().rev() {
                if let Some(end) = end {
                    if start > end || end > xml.len() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "ClefReadability plan contains an invalid removal range",
                        ));
                    }
                    applied_geometry.push((start, end, String::new()));
                    xml.replace_range(start..end, "");
                } else if let Some(block) = block {
                    if start > xml.len() || !xml.is_char_boundary(start) {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "ClefReadability plan contains an invalid insertion point",
                        ));
                    }
                    applied_geometry.push((start, start, block.to_string()));
                    xml.insert_str(start, block);
                }
            }
            if !plan.verifies(&xml) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "ClefReadability plan violated note or clef invariants",
                ));
            }
            Some(xml)
        }
    } else {
        None
    };
    let effective_xml = planned_xml.as_deref().unwrap_or(candidate_xml);

    // r462: FinalChordEncoding is an explicit writer-free SerializationPlan projection
    // before this sink. The same FinalNotation capability must never change behavior
    // based on whichever checkpoint happened to run immediately before it.
    // v0.10.1308: DirectionRelocation is the final Render-MusicXML checkpoint
    // in the orchestrator. It may move only direction payloads and must preserve
    // every note ordinal and Beam state already committed by preceding writers.
    // The capability write still performs its exact byte readback afterwards.
    if stage == RenderCheckpointStage::DirectionRelocation {
        validate_direction_relocation_preserves_note_beams(path, effective_xml)?;
    }
    realized_post = Some(effective_xml.to_string());
    if stage == RenderCheckpointStage::FinalNotationPreparation {
        return Ok(());
    }
    OrchestratedRenderCheckpointCapability::issue(path, effective_xml, stage)?.write()
})();
    let geometry = crate::master_state_identity::exact_script_from_descending_application(applied_geometry);
    let receipt = realized_post.as_deref().filter(|post| output.is_ok() && *post != candidate_xml).map(|post| crate::master_state_identity::ExactTransformationReceipt::mint_exact_receipt(RenderRow58ExactMint { _private: () }, candidate_xml, post, geometry).expect("frozen owner changed bytes without receipt"));
    (output, receipt)
}

/// Private non-I/O stage of the same frozen #58 owner and original exact script.
pub(super) fn prepare_final_notation_checkpoint(
    path: &Path, xml: &str,
) -> (io::Result<String>, Option<ExactTransformationReceipt>) {
    let (result, receipt) = write_render_checkpoint(path, xml, RenderCheckpointStage::FinalNotationPreparation);
    let prepared = result.and_then(|()| match receipt.as_ref() {
        Some(original) => original.realize_post_document(xml).ok_or_else(|| io::Error::new(
            io::ErrorKind::InvalidData, "Prepared #58 original receipt cannot replay its source")),
        None => Ok(xml.to_string()),
    });
    (prepared, receipt)
}

/// The sealed adoption boundary alone can persist the completely verified post.
pub(super) fn write_prepared_final_notation_checkpoint(
    _boundary: &super::RenderAdoptionCommitBoundary, path: &Path, xml: &str,
) -> io::Result<()> {
    OrchestratedRenderCheckpointCapability::issue(path, xml, RenderCheckpointStage::FinalNotation)?.write()
}

macro_rules! render_checkpoint_entry {
    ($name:ident, $stage:ident) => {
        pub fn $name(
            path: &Path,
            candidate_xml: &str,
        ) -> (io::Result<()>, Option<ExactTransformationReceipt>) {
            write_render_checkpoint(path, candidate_xml, RenderCheckpointStage::$stage)
        }
    };
}

render_checkpoint_entry!(write_clef_island_decomposition_checkpoint_authorized, ClefIslandDecomposition);

render_checkpoint_entry!(write_cursor_lane_repair_checkpoint_authorized, CursorLaneRepair);
render_checkpoint_entry!(write_retention_return_checkpoint_authorized, RetentionReturn);
render_checkpoint_entry!(write_duplicate_rest_rewrite_checkpoint_authorized, DuplicateRestRewrite);
render_checkpoint_entry!(write_visible_rest_role_checkpoint_authorized, VisibleRestRole);
render_checkpoint_entry!(write_final_piano_rest_glyph_checkpoint_authorized, FinalPianoRestGlyph);


render_checkpoint_entry!(write_sister_feedback_beam_checkpoint_authorized, SisterFeedbackBeam);
render_checkpoint_entry!(write_external_feedback_closure_checkpoint_authorized, ExternalFeedbackClosure);
render_checkpoint_entry!(write_key_signature_spelling_checkpoint_authorized, KeySignatureSpelling);
render_checkpoint_entry!(write_direction_relocation_checkpoint_authorized, DirectionRelocation);



pub fn write_master_musicxml_output_authorized(
    path: &Path,
    candidate_xml: &str,
    functional_respelling: FunctionalMasterRespellingOperation,
) -> (io::Result<()>, Option<ExactTransformationReceipt>, Option<ExactTransformationReceipt>) {
    // Nested frozen-owner transport 59 -> 60: #59 mints its own opening-key
    // receipt at origin; #60 mints at its origin and the receipt is forwarded
    // unchanged. Order is outer then inner.
    let mut outer_receipt: Option<ExactTransformationReceipt> = None;
    let mut inner_receipt: Option<ExactTransformationReceipt> = None;
    let written = (|| -> io::Result<()> {
        validate_complete_musicxml(candidate_xml)?;
        let mut authorized_candidate = candidate_xml.to_string();
        if let Some(render_fifths) = functional_respelling
            .tonal_frame()
            .and_then(crate::pipeline::TonalFrameFact::fifths_vector)
        {
            let master_document = roxmltree::Document::parse(candidate_xml).map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Master-MusicXML key mirror parse failed: {error}"),
                )
            })?;
            let master_fifths = master_document
                .descendants()
                .filter(|node| node.has_tag_name("fifths"))
                .map(|node| node.text()?.trim().parse::<i32>().ok())
                .collect::<Option<Vec<_>>>();
            if let Some(master_fifths) = master_fifths
                && master_fifths.len() == render_fifths.len()
                    && !master_fifths.is_empty()
                    && master_fifths[0] == 0
                    && render_fifths[0] != 0
                {
                    if !(-5..=6).contains(&render_fifths[0])
                        || master_fifths[1..] != render_fifths[1..]
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "canonical opening-key mirror is not an exact first-key-only delta",
                        ));
                    }
                    let opening_fifths = master_document
                        .descendants()
                        .find(|node| node.has_tag_name("fifths"))
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                "canonical opening-key mirror lost the first fifths node",
                            )
                        })?;
                    let element_range = opening_fifths.range();
                    let element_text = &candidate_xml[element_range.clone()];
                    let value_start = element_text.find('>').map(|index| index + 1).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "canonical opening-key mirror could not bind the fifths opening tag",
                        )
                    })?;
                    let value_end = element_text.rfind("</fifths>").ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "canonical opening-key mirror could not bind the fifths closing tag",
                        )
                    })?;
                    if value_start > value_end
                        || element_text[value_start..value_end].trim() != "0"
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "canonical opening-key mirror source is not the open signature",
                        ));
                    }
                    let absolute_value_start = element_range.start + value_start;
                    let absolute_value_end = element_range.start + value_end;
                    let opening_key_replacement = render_fifths[0].to_string();
                    authorized_candidate.replace_range(
                        absolute_value_start..absolute_value_end,
                        &opening_key_replacement,
                    );
                    validate_complete_musicxml(&authorized_candidate)?;
                    if note_beam_fingerprint(candidate_xml)?
                        != note_beam_fingerprint(&authorized_candidate)?
                    {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "canonical opening-key mirror changed the note/beam fingerprint",
                        ));
                    }
                    let authorized_document = roxmltree::Document::parse(&authorized_candidate)
                        .map_err(|error| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Canonical Master-MusicXML key mirror parse failed: {error}"),
                            )
                        })?;
                    let authorized_fifths = authorized_document
                        .descendants()
                        .filter(|node| node.has_tag_name("fifths"))
                        .map(|node| node.text()?.trim().parse::<i32>().ok())
                        .collect::<Option<Vec<_>>>();
                    if authorized_fifths.as_deref() != Some(render_fifths) {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "canonical opening-key mirror failed exact key-vector readback",
                        ));
                    }
                    outer_receipt = ExactTransformationReceipt::mint_exact_receipt(
                        MasterExportFifthsExactMint { _sealed: () },
                        candidate_xml,
                        &authorized_candidate,
                        vec![ExactEdit::bound(
                            absolute_value_start,
                            absolute_value_end,
                            opening_key_replacement,
                        )],
                    );
                    if outer_receipt.is_none() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "canonical opening-key mirror could not mint its EXACT receipt",
                        ));
                    }
                }
        }
        if let Some((mirrored, functional_receipt)) =
            functional_respelling.apply(candidate_xml, &authorized_candidate, outer_receipt.as_ref())
        {
            let before_note_beam = note_beam_fingerprint(&authorized_candidate)?;
            let mirrored =
                mirrored.map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            if note_beam_fingerprint(&mirrored)? != before_note_beam {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "canonical functional master mirror changed the note/beam fingerprint",
                ));
            }
            let receipt_bound = match functional_receipt.as_ref() {
                Some(receipt) => {
                    receipt.minted_by::<crate::pipeline::FunctionalRespellingExactMint>()
                        && receipt.realize_post_document(&authorized_candidate).as_deref()
                            == Some(mirrored.as_str())
                }
                None => mirrored == authorized_candidate,
            };
            if !receipt_bound {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "canonical functional master mirror receipt is not the #60 descriptor bound to this transition",
                ));
            }
            inner_receipt = functional_receipt;
            authorized_candidate = mirrored;
        }
        MasterMusicXmlOutputCapability::issue(path, &authorized_candidate)?.write()
    })();
    (written, outer_receipt, inner_receipt)
}

pub fn write_result_report_output_authorized(
    path: &Path,
    finalized_report: &FinalizedReport,
) -> io::Result<()> {
    let report_text = finalized_report.as_str();
    if env!("CARGO_PKG_VERSION") != "0.10.1344" {
        return ResultReportOutputCapability::issue(path, report_text)?.write();
    }
    const AUTHORITY_LINE: &str =
        "BehaviorChange: ja; WriterChange ja; PredicateChange nein; MusicMutation ja; MasterApply ja.";
    const DELTA_LINE: &str =
        "KeySignatureOnlyBoundedApply: KeySignatureDeltaCount 2; ExplicitAccidentalRemovalCount 1; PitchDeltaCount 0; OnsetDeltaCount 0; DurationDeltaCount 0; VoiceDeltaCount 0; StaffDeltaCount 0; RespellingDeltaCount 0.";
    if report_text.lines().any(|line| line == AUTHORITY_LINE)
        || report_text.lines().any(|line| line == DELTA_LINE)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "KeySignatureOnlyBoundedApply: Bericht enthält die Apply-Zeilen bereits",
        ));
    }
    let prefix = "Getan: v0.10.1344 ";
    let mut decisions = report_text.lines().filter(|line| line.starts_with(prefix));
    let old_decision = decisions.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "KeySignatureOnlyBoundedApply: aktueller Entscheidungskopf fehlt",
        )
    })?;
    if decisions.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "KeySignatureOnlyBoundedApply: aktueller Entscheidungskopf ist mehrdeutig",
        ));
    }
    let replacement = format!(
        "Getan: v0.10.1344 KeySignatureOnlyBoundedApply; MusicMutation ja; MasterApply ja.\n{AUTHORITY_LINE}\n{DELTA_LINE}"
    );
    let bound = report_text.replacen(old_decision, &replacement, 1);
    ResultReportOutputCapability::issue(path, &bound)?.write()
}


#[cfg(test)]
mod final_sink_note_beam_guard_tests {
    use super::*;

    fn score(note_body: &str, direction: &str) -> String {
        format!(
            "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>Piano</part-name></score-part></part-list><part id=\"P1\"><measure number=\"1\">{direction}<note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>16th</type>{note_body}</note></measure></part></score-partwise>"
        )
    }

    #[test]
    fn note_beam_fingerprint_ignores_direction_payload() {
        let before = score(
            "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>",
            "<direction><direction-type><words>A</words></direction-type></direction>",
        );
        let after = score(
            "<beam number=\"1\">begin</beam><beam number=\"2\">begin</beam>",
            "<direction><direction-type><words>B</words></direction-type><offset>1</offset></direction>",
        );
        assert_eq!(note_beam_fingerprint(&before).unwrap(), note_beam_fingerprint(&after).unwrap());
    }

    #[test]
    fn note_beam_fingerprint_rejects_beam_reversion() {
        let before = score(
            "<beam number=\"1\">continue</beam><beam number=\"2\">continue</beam>",
            "",
        );
        let reverted = score(
            "<beam number=\"1\">end</beam><beam number=\"2\">end</beam>",
            "",
        );
        assert_ne!(
            note_beam_fingerprint(&before).unwrap(),
            note_beam_fingerprint(&reverted).unwrap()
        );
    }

    fn temporary_render_path(label: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("etuschlopf_{label}_{nonce}.render.musicxml"))
    }

    #[test]
    fn successful_checkpoint_is_available_as_current_thread_sink() {
        let path = temporary_render_path("registered_sink");
        let xml = score(
            "<beam number=\"1\">continue</beam><beam number=\"2\">continue</beam>",
            "",
        );
        let (result, receipt) = write_render_checkpoint(&path, &xml, RenderCheckpointStage::ExternalFeedbackClosure);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe); result.unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), xml);
        assert!(validate_terminal_render_temporal_postcondition_authorized(&path, &xml).is_ok());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn terminal_render_temporal_materialization_rejects_invalid_registered_checkpoint() {
        let path = temporary_render_path("terminal_temporal_invalid");
        let xml = score(
            "<beam number=\"1\">continue</beam><beam number=\"2\">continue</beam>",
            "",
        )
        .replace("<duration>1</duration>", "<duration>5</duration>");
        let (result, receipt) = write_render_checkpoint(&path, &xml, RenderCheckpointStage::ExternalFeedbackClosure);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe); result.unwrap();
        assert!(validate_terminal_render_temporal_postcondition_authorized(&path, &xml).is_err());
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn changed_checkpoint_file_invalidates_thread_sink() {
        let path = temporary_render_path("tampered_sink");
        let xml = score(
            "<beam number=\"1\">continue</beam><beam number=\"2\">continue</beam>",
            "",
        );
        let changed = score(
            "<beam number=\"1\">end</beam><beam number=\"2\">end</beam>",
            "",
        );
        let (result, receipt) = write_render_checkpoint(&path, &xml, RenderCheckpointStage::ExternalFeedbackClosure);
        let mut probe = crate::master_state_identity::RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe); result.unwrap();
        std::fs::write(&path, changed).unwrap();
        assert!(validate_terminal_render_temporal_postcondition_authorized(&path, &xml).is_err());
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(test)]
mod nested_master_export_receipt_tests {
    use super::*;
    use crate::master_state_identity::tests::{
        synthetic_exact_receipt_for_tests, synthetic_functional_respelling_receipt_for_tests,
    };

    fn export_score(fifths: i32, part_name: &str) -> String {
        let ordinal = usize::from(part_name.is_empty()) + 1;
        format!(
            "<?xml version=\"1.0\"?><score-partwise version=\"4.0\"><part-list><score-part id=\"P1\"><part-name>{part_name}</part-name></score-part></part-list><part id=\"P1\"><measure number=\"{ordinal}\"><attributes><divisions>1</divisions><key><fifths>{fifths}</fifths></key></attributes><note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>quarter</type></note></measure></part></score-partwise>"
        )
    }

    fn master_path(label: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        std::env::temp_dir().join(format!("etuschlopf_nested_{label}_{nonce}.musicxml"))
    }

    fn part_name_edit(master: &str) -> Option<(usize, usize)> {
        let start = master.find("<part-name>")? + "<part-name>".len();
        let end = start + master[start..].find("</part-name>")?;
        Some((start, end))
    }

    fn functional_noop(
        master: &str,
        _render: &str,
        _original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains,
    ) -> (Result<String, String>, Option<ExactTransformationReceipt>) {
        (Ok(master.to_string()), None)
    }

    fn functional_write(
        master: &str,
        _render: &str,
        _original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains,
    ) -> (Result<String, String>, Option<ExactTransformationReceipt>) {
        let (start, end) = part_name_edit(master).expect("part name");
        let mut post = master.to_string();
        post.replace_range(start..end, "Klavier");
        let receipt = synthetic_functional_respelling_receipt_for_tests(master, &post, start, end, "Klavier");
        (Ok(post), Some(receipt))
    }

    fn functional_wrong_descriptor(
        master: &str,
        _render: &str,
        _original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains,
    ) -> (Result<String, String>, Option<ExactTransformationReceipt>) {
        let (start, end) = part_name_edit(master).expect("part name");
        let mut post = master.to_string();
        post.replace_range(start..end, "Klavier");
        let receipt = synthetic_exact_receipt_for_tests(master, &post, start, end, "Klavier");
        (Ok(post), Some(receipt))
    }

    fn functional_unreceipted(
        master: &str,
        _render: &str,
        _original_chains: &crate::master_state_identity::OriginalFunctionalRespellingChains,
    ) -> (Result<String, String>, Option<ExactTransformationReceipt>) {
        let (start, end) = part_name_edit(master).expect("part name");
        let mut post = master.to_string();
        post.replace_range(start..end, "Klavier");
        (Ok(post), None)
    }

    fn run(
        render_fifths: Option<i32>,
        rewrite: FunctionalRespellingRewrite,
    ) -> (
        io::Result<()>,
        Option<ExactTransformationReceipt>,
        Option<ExactTransformationReceipt>,
        String,
        Option<String>,
    ) {
        let candidate = export_score(0, "Piano");
        let path = master_path("run");
        let operation = FunctionalMasterRespellingOperation::new(rewrite).bind_tonal_frame(
            render_fifths.map(|fifths| {
                crate::pipeline::TonalFrameFact::derive_from_render_snapshot(&export_score(fifths, "Piano"))
                    .expect("render fact")
            }),
        );
        let (written, outer, inner) = write_master_musicxml_output_authorized(&path, &candidate, operation);
        let artifact = std::fs::read_to_string(&path).ok();
        let _ = std::fs::remove_file(&path);
        (written, outer, inner, candidate, artifact)
    }

    fn realized(
        candidate: &str,
        outer: Option<&ExactTransformationReceipt>,
        inner: Option<&ExactTransformationReceipt>,
    ) -> String {
        let mut document = candidate.to_string();
        for receipt in [outer, inner].into_iter().flatten() {
            document = receipt.realize_post_document(&document).expect("receipt realizes");
        }
        document
    }

    #[test]
    fn outer_noop_and_inner_noop_return_no_receipts() {
        let (written, outer, inner, candidate, artifact) = run(Some(0), functional_noop);
        assert!(written.is_ok());
        assert!(outer.is_none() && inner.is_none());
        assert_eq!(artifact.as_deref(), Some(candidate.as_str()));
    }

    #[test]
    fn outer_write_and_inner_noop_return_the_outer_receipt_only() {
        let (written, outer, inner, candidate, artifact) = run(Some(2), functional_noop);
        assert!(written.is_ok());
        let outer = outer.expect("#59 receipt");
        assert_eq!(outer.descriptor_row(), 59);
        assert!(inner.is_none());
        assert_eq!(Some(realized(&candidate, Some(&outer), None)), artifact);
        assert!(artifact.is_some_and(|xml| xml.contains("<fifths>2</fifths>")));
    }

    #[test]
    fn outer_write_and_inner_write_return_both_in_order() {
        let (written, outer, inner, candidate, artifact) = run(Some(2), functional_write);
        assert!(written.is_ok());
        let (outer, inner) = (outer.expect("#59"), inner.expect("#60"));
        assert_eq!((outer.descriptor_row(), inner.descriptor_row()), (59, 60));
        assert_eq!(Some(realized(&candidate, Some(&outer), Some(&inner))), artifact);
        // Swapped application does not realize.
        assert!(inner.realize_post_document(&candidate).is_none());
    }

    #[test]
    fn outer_noop_and_inner_write_return_the_inner_receipt_only() {
        let (written, outer, inner, candidate, artifact) = run(Some(0), functional_write);
        assert!(written.is_ok());
        assert!(outer.is_none());
        let inner = inner.expect("#60");
        assert_eq!(Some(realized(&candidate, None, Some(&inner))), artifact);
    }

    #[test]
    fn no_tonal_frame_leaves_the_master_untouched() {
        let (written, outer, inner, candidate, artifact) = run(None, functional_write);
        assert!(written.is_ok());
        assert!(outer.is_none() && inner.is_none());
        assert_eq!(artifact.as_deref(), Some(candidate.as_str()));
    }

    #[test]
    fn inner_receipt_with_a_foreign_descriptor_is_rejected() {
        let (written, _, inner, _, artifact) = run(Some(0), functional_wrong_descriptor);
        assert!(written.is_err());
        assert!(inner.is_none());
        assert!(artifact.is_none());
    }

    #[test]
    fn changed_inner_transition_without_receipt_is_rejected() {
        let (written, _, inner, _, artifact) = run(Some(2), functional_unreceipted);
        assert!(written.is_err());
        assert!(inner.is_none());
        assert!(artifact.is_none());
    }

    #[test]
    fn tonal_frame_binding_rejects_a_snapshot_that_differs_from_the_legacy_view() {
        let render = export_score(2, "Piano");
        let render_path = std::env::temp_dir().join(format!(
            "etuschlopf_tonal_frame_{}.render.musicxml",
            std::process::id()
        ));
        std::thread::spawn(move || {
            // No legacy checkpoint on this thread: a present snapshot diverges.
            assert!(bind_tonal_frame_fact_from_render_snapshot(Some(render.clone())).is_ok());
            assert!(bind_tonal_frame_fact_from_render_snapshot(None)
                .is_ok_and(|fact| fact.is_none()));
            let (result, receipt) = write_final_notation_checkpoint_authorized(&render_path, &render);
            let mut probe = crate::master_state_identity::RenderTransitionReceipts::new(); probe.push_exact(receipt); probe.discard_unadopted(crate::master_state_identity::RenderDiscardReason::ReadOnlyProbe); result.expect("checkpoint");
            assert!(bind_tonal_frame_fact_from_render_snapshot(Some(render.clone()))
                .is_ok_and(|fact| fact.is_some()));
            assert!(bind_tonal_frame_fact_from_render_snapshot(Some(export_score(3, "Piano"))).is_ok());
            assert!(bind_tonal_frame_fact_from_render_snapshot(None).is_ok());
            let _ = std::fs::remove_file(&render_path);
        })
        .join()
        .expect("tonal frame binding thread");
    }
}


pub(crate) struct RenderRow58ExactMint { _private: () }

#[cfg(test)]
mod frozen_checkpoint_owner {
    use super::*;
    include!("../build_gate/ssc_test_fixture_r1458_checkpoint_owner.txt");
    pub(super) fn final_notation(path: &Path, xml: &str) -> (io::Result<()>, Option<ExactTransformationReceipt>) {
        write_render_checkpoint(path, xml, RenderCheckpointStage::FinalNotation)
    }
}

#[cfg(test)]
pub(crate) fn legacy_final_notation_checkpoint_for_tests(
    path: &Path, xml: &str,
) -> (io::Result<()>, Option<ExactTransformationReceipt>) {
    frozen_checkpoint_owner::final_notation(path, xml)
}

#[cfg(test)]
pub fn write_final_notation_checkpoint_authorized(
    path: &Path, candidate_xml: &str,
) -> (io::Result<()>, Option<ExactTransformationReceipt>) {
    write_render_checkpoint(path, candidate_xml, RenderCheckpointStage::FinalNotation)
}
