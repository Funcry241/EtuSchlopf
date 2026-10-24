//! Narrow source-bound Master XML request boundary.
//!
//! Pipeline owners may construct immutable replacement requests, but the XML
//! slot is materialized only here inside StructuralWriteAuthority. This module
//! intentionally carries no selection, ranking, repair-scope or adoption logic.

mod adopted_state;
mod render_adopted_state;
mod render_fragment_draft;
mod request_boundary;

pub(crate) use adopted_state::{MasterAdoptedState, MasterAdoptionCommitBoundary};
pub(crate) use render_adopted_state::{
    RenderAdoptionCommitBoundary, RenderCheckpointSink, RenderLateBranch, RenderPathBoundOutcome,
    RenderSinkKind, RenderSovereignState, RenderTrialRoot,
};
pub(crate) use render_fragment_draft::{mutate_selected_duration_fragments, DurationPromotion};
pub(crate) use request_boundary::{
    apply_block_master_xml_request, MasterXmlReplacementRequest,
};

mod final_layout_phrase_materialization;
pub(super) use final_layout_phrase_materialization::mutate_final_layout_phrase_insertions;
pub(crate) use final_layout_phrase_materialization::{adopt_final_layout_phrase_projection, adopt_final_layout_phrase_projection_with_course};

#[cfg(test)]
pub(crate) use final_layout_phrase_materialization::set_final_layout_transport_fault;
