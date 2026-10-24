use crate::final_report::ReportRewriteScope;

// r774: the accepted OutcomeProjection now keeps externally supplied score-title metadata
// opaque while internal report vocabulary remains sanitizer-owned. Ownership and call graph stay
// unchanged; only the Audit-owned projection policy changes.
pub(crate) const EXTERNAL_METADATA_OPAQUE: bool = true;

pub(crate) const ACTIVE_REPORT_REWRITE_SCOPE: ReportRewriteScope = ReportRewriteScope {
    external_metadata_opaque: EXTERNAL_METADATA_OPAQUE,
};
