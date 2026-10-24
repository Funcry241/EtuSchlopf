// Canonical SWA boundary tests; included in the original private helper scope.
#[cfg(test)]
mod r1230_ir4_row57_tests {
    use super::*;

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/render_authority/row57_prepares_file_bound_post_without_writing_before_adoption.rs"));
}

// #57 test-only precursor: existing trial, receipt, adoption and artifact readback.
#[cfg(test)]
mod row57_local_clef_scope_through_tests {
    use super::*;
    use crate::master_state_identity::tests::synthetic_genesis_for_tests;
    use crate::master_state_identity::MasterLineage;
    use crate::structural_write_authority::{MasterAdoptedState, RenderSinkKind, RenderSovereignState};

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/render_authority/row57_local_clef_scope_through_trial.rs"));
}
