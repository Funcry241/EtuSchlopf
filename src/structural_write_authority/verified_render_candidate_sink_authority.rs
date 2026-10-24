//! Typed, effect-specific file sinks for already verified render candidates.
//!
//! Each public entry point names one semantic family. The private sink binds
//! the exact path and candidate bytes, accepts only complete MusicXML and
//! verifies an exact readback. It cannot select music or mutate XML.

use super::authority_primitives::fingerprint;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderCandidateEffect {
    VisualCleanup,
    SingleStaccato,
    SurfaceClefNeutralization,
    RendererFeasibilityShadow,
}

#[derive(Debug)]
struct VerifiedRenderCandidateSink {
    path: PathBuf,
    path_fingerprint: u64,
    candidate_xml: String,
    candidate_fingerprint: u64,
    effect: RenderCandidateEffect,
}

impl RenderCandidateEffect {
    fn tag(self) -> &'static str {
        match self {
            Self::VisualCleanup => "visual_cleanup",
            Self::SingleStaccato => "single_staccato_candidate",
            Self::SurfaceClefNeutralization => "surface_clef_neutralization",
            Self::RendererFeasibilityShadow => "renderer_feasibility_shadow_candidate",
        }
    }
}

impl VerifiedRenderCandidateSink {
    fn issue(path: &Path, candidate: &str, effect: RenderCandidateEffect) -> io::Result<Self> {
        if candidate.trim().is_empty()
            || roxmltree::Document::parse(candidate).is_err()
            || !candidate.contains("<score-partwise")
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "verified render candidate is not complete MusicXML",
            ));
        }
        let path_text = path.to_string_lossy();
        if path_text.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "verified render candidate path is empty",
            ));
        }
        Ok(Self {
            path: path.to_path_buf(),
            path_fingerprint: fingerprint(path_text.as_ref()),
            candidate_xml: candidate.to_string(),
            candidate_fingerprint: fingerprint(candidate),
            effect,
        })
    }

    fn write(self) -> io::Result<()> {
        let path_text = self.path.to_string_lossy();
        if fingerprint(path_text.as_ref()) != self.path_fingerprint
            || fingerprint(&self.candidate_xml) != self.candidate_fingerprint
            || roxmltree::Document::parse(&self.candidate_xml).is_err()
            || !effect_contract_green(self.effect, &self.candidate_xml)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "verified render candidate binding changed before write",
            ));
        }
        let writer_trace_before = crate::audit_only::writer_trace::capture_before(&self.path);
        std::fs::write(&self.path, self.candidate_xml.as_bytes())?;
        let readback = std::fs::read_to_string(&self.path)?;
        if readback != self.candidate_xml || fingerprint(&readback) != self.candidate_fingerprint {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "verified render candidate readback mismatch",
            ));
        }
        crate::audit_only::writer_trace::record_after(
            self.effect.tag(),
            &self.path,
            writer_trace_before.as_deref(),
            &self.candidate_xml,
        );
        Ok(())
    }
}

fn effect_contract_green(effect: RenderCandidateEffect, candidate: &str) -> bool {
    match effect {
        RenderCandidateEffect::VisualCleanup => candidate.contains("<measure"),
        RenderCandidateEffect::SingleStaccato => candidate.contains("<articulations>")
            || candidate.contains("<staccato")
            || candidate.contains("<accent"),
        RenderCandidateEffect::SurfaceClefNeutralization => candidate.contains("<clef"),
        RenderCandidateEffect::RendererFeasibilityShadow => candidate.contains("<measure"),
    }
}

fn write_candidate(path: &Path, candidate: &str, effect: RenderCandidateEffect) -> io::Result<()> {
    VerifiedRenderCandidateSink::issue(path, candidate, effect)?.write()
}

pub fn write_visual_cleanup_candidate_authorized(
    path: &Path,
    candidate: &str,
) -> io::Result<()> {
    write_candidate(path, candidate, RenderCandidateEffect::VisualCleanup)
}

pub fn write_single_staccato_candidate_authorized(
    path: &Path,
    candidate: &str,
) -> io::Result<()> {
    write_candidate(path, candidate, RenderCandidateEffect::SingleStaccato)
}

fn renderer_feasibility_shadow_path_green(path: &Path) -> bool {
    if path.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir | std::path::Component::CurDir
        )
    }) {
        return false;
    }
    if !matches!(
        path.file_name().and_then(|value| value.to_str()),
        Some("constraint_candidate.render.musicxml" | "constraint_candidate.musicxml")
    ) {
        return false;
    }
    let Some(project_slot) = path.parent() else {
        return false;
    };
    let Some(shadow_root) = project_slot.parent() else {
        return false;
    };
    let Some(target_root) = shadow_root.parent() else {
        return false;
    };
    shadow_root.file_name().and_then(|value| value.to_str())
        == Some("etuschlopf_renderer_feasibility_shadow")
        && target_root.file_name().and_then(|value| value.to_str()) == Some("target")
        && project_slot
            .file_name()
            .and_then(|value| value.to_str())
            .is_some_and(|value| !value.trim().is_empty())
}

pub(crate) fn write_renderer_feasibility_shadow_candidate_authorized(
    path: &Path,
    candidate: &str,
) -> io::Result<()> {
    if !renderer_feasibility_shadow_path_green(path) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "renderer-feasibility shadow candidate path is not target-scoped",
        ));
    }
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "renderer-feasibility shadow candidate parent is missing",
        )
    })?;
    std::fs::create_dir_all(parent)?;
    write_candidate(path, candidate, RenderCandidateEffect::RendererFeasibilityShadow)
}

pub(super) fn sink_surface_clef_candidate(
    path: &Path,
    candidate: &str,
) -> io::Result<()> {
    write_candidate(
        path,
        candidate,
        RenderCandidateEffect::SurfaceClefNeutralization,
    )
}

