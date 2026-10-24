//! Authorized measurement sink; no report parsing, selection or authority.
pub(crate) fn append_ssc_shadow_measurement(
    access: &crate::final_report::SscObservationReadAccess,
    out: &mut String,
) {
    crate::ssc_evidence_proof::append_capture_coverage(access, out);
}
