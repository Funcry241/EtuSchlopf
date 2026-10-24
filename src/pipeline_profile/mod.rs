mod catalog;
mod legacy_name_projection;
mod lineage;
mod model;

pub use catalog::{
    base_solver_profiles, critical_micro_repair_profiles, local_best_refinement_profiles,
    residual_afterburner_profiles, targeted_critical_profiles,
};
pub use legacy_name_projection::{
    broken_gesture_name, combined_name, hybrid_tail_name,
};
pub use lineage::{CandidateLineage, CandidateLineageStep};
pub use model::{
    CandidateProfileFamily, CandidateProfileKind, CandidateProfile, ProfileParameters,
};
