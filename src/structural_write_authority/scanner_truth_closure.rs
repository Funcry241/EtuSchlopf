use super::authority_primitives::fingerprint_bytes as fnv1a64;
use std::io;
use std::path::{Path, PathBuf};

struct BehindBarsRestTieAuditSinkCapability {
    path: PathBuf,
    candidate_fingerprint: u64,
    candidate_len: usize,
}

impl BehindBarsRestTieAuditSinkCapability {
    fn issue(path: &Path, candidate_xml: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            candidate_fingerprint: fnv1a64(candidate_xml.as_bytes()),
            candidate_len: candidate_xml.len(),
        }
    }

    fn consume(self, path: &Path, candidate_xml: &str) -> io::Result<()> {
        verify_sink_binding(
            &self.path,
            self.candidate_fingerprint,
            self.candidate_len,
            path,
            candidate_xml,
        )?;
        let writer_trace_before = crate::audit_only::writer_trace::capture_before(path);
        std::fs::write(path, candidate_xml)?;
        verify_exact_file_readback(path, candidate_xml)?;
        crate::audit_only::writer_trace::record_after(
            "behind_bars_rest_tie_audit",
            path,
            writer_trace_before.as_deref(),
            candidate_xml,
        );
        Ok(())
    }
}

pub fn write_behind_bars_rest_tie_audit_candidate_authorized(
    path: &Path,
    candidate_xml: &str,
) -> io::Result<()> {
    BehindBarsRestTieAuditSinkCapability::issue(path, candidate_xml).consume(path, candidate_xml)
}

struct ExternalNotationFeedbackAuditSinkCapability {
    path: PathBuf,
    candidate_fingerprint: u64,
    candidate_len: usize,
}

impl ExternalNotationFeedbackAuditSinkCapability {
    fn issue(path: &Path, candidate_xml: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            candidate_fingerprint: fnv1a64(candidate_xml.as_bytes()),
            candidate_len: candidate_xml.len(),
        }
    }

    fn consume(self, path: &Path, candidate_xml: &str) -> io::Result<()> {
        verify_sink_binding(
            &self.path,
            self.candidate_fingerprint,
            self.candidate_len,
            path,
            candidate_xml,
        )?;
        let writer_trace_before = crate::audit_only::writer_trace::capture_before(path);
        std::fs::write(path, candidate_xml)?;
        verify_exact_file_readback(path, candidate_xml)?;
        crate::audit_only::writer_trace::record_after(
            "external_notation_feedback_audit",
            path,
            writer_trace_before.as_deref(),
            candidate_xml,
        );
        Ok(())
    }
}

pub fn write_external_notation_feedback_audit_candidate_authorized(
    path: &Path,
    candidate_xml: &str,
) -> io::Result<()> {
    ExternalNotationFeedbackAuditSinkCapability::issue(path, candidate_xml)
        .consume(path, candidate_xml)
}

fn verify_sink_binding(
    authorized_path: &Path,
    authorized_fingerprint: u64,
    authorized_len: usize,
    path: &Path,
    candidate_xml: &str,
) -> io::Result<()> {
    if authorized_path != path
        || authorized_len != candidate_xml.len()
        || authorized_fingerprint != fnv1a64(candidate_xml.as_bytes())
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "typed XML sink binding changed before consumption",
        ));
    }
    Ok(())
}

fn verify_exact_file_readback(path: &Path, candidate_xml: &str) -> io::Result<()> {
    let readback = std::fs::read(path)?;
    if readback != candidate_xml.as_bytes() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "typed XML sink readback differs from authorized candidate",
        ));
    }
    Ok(())
}
