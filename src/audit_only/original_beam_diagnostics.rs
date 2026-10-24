//! Counts observations only; detail selection never controls musical work.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Mutex, OnceLock};

// One project per process. The CLI request is immutable and visible to workers.
// Uninitialized means disabled.
static DETAIL_REQUEST: OnceLock<DetailRequest> = OnceLock::new();

enum DetailRequest {
    Disabled,
    All,
    Measures(BTreeSet<String>),
}
static SUMMARY: Mutex<Option<Summary>> = Mutex::new(None);

#[derive(Default)]
struct Summary {
    project: String,
    counts: BTreeMap<&'static str, usize>,
    reasons: BTreeMap<String, usize>,
    scalar_changes: usize,
    collision_trials: usize,
    lost_registration_trials: usize,
    unbound_seeds: usize,
    unretained_components: usize,
    missing_source_locations: usize,
}

pub(crate) enum Observation<'a> {
    Census {
        bound_seed: bool,
        retained_component: bool,
        source_locations_present: bool,
    },
    RendererCohort,
    Decision(&'a str),
    Admission {
        scalar_changes: usize,
        collisions: bool,
        lost_registration: bool,
    },
    CohortProject,
}

pub(crate) fn start(project: &str, request: Option<Option<BTreeSet<String>>>) {
    let request = match request {
        None => DetailRequest::Disabled,
        Some(None) => DetailRequest::All,
        Some(Some(selected)) => DetailRequest::Measures(selected),
    };
    let _ = DETAIL_REQUEST.set(request);
    *SUMMARY
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(Summary {
        project: project.to_owned(),
        ..Summary::default()
    });
}

fn request_matches(request: Option<&DetailRequest>, measure: usize) -> bool {
    match request {
        Some(DetailRequest::All) => true,
        Some(DetailRequest::Measures(selected)) => selected.contains(&measure.to_string()),
        _ => false,
    }
}

pub(crate) fn detail_requested_for_measure(measure: usize) -> bool {
    request_matches(DETAIL_REQUEST.get(), measure)
}

impl Summary {
    // Return value affects only the associated diagnostic output.
    fn record(&mut self, observation: Observation<'_>) -> bool {
        let (kind, unusual) = match observation {
            Observation::Census {
                bound_seed,
                retained_component,
                source_locations_present,
            } => {
                self.unbound_seeds += usize::from(!bound_seed);
                self.unretained_components += usize::from(!retained_component);
                self.missing_source_locations += usize::from(!source_locations_present);
                (
                    "OriginCensus",
                    !bound_seed || !retained_component || !source_locations_present,
                )
            }
            Observation::RendererCohort => ("RendererCohort", false),
            Observation::Decision(reason) => {
                *self.reasons.entry(reason.to_owned()).or_default() += 1;
                // All other reasons, including future unknown reasons, remain visible.
                let ordinary = matches!(
                    reason,
                    "original_components_preserved"
                        | "original_import_resource_component"
                        | "member_levels_timing_roles_or_single_label"
                        | "complete_resource_image_already_one_label"
                        | "proposal"
                );
                ("AlignmentDecision", !ordinary)
            }
            Observation::Admission {
                scalar_changes,
                collisions,
                lost_registration,
            } => {
                self.scalar_changes += scalar_changes;
                self.collision_trials += usize::from(collisions);
                self.lost_registration_trials += usize::from(lost_registration);
                ("TargetResourceAdmission", collisions || lost_registration)
            }
            Observation::CohortProject => ("CohortProject", false),
        };
        *self.counts.entry(kind).or_default() += 1;
        unusual
    }

    fn line(&self) -> String {
        format!(
            "OriginalBeamSummary: project={:?}; OriginCensus={}; RendererCohort={}; AlignmentDecision={}; TargetResourceAdmission={}; CohortProject={}; reasons={:?}; scalar_changes={}; collision_trials={}; lost_registration_trials={}; unbound_seeds={}; unretained_components={}; missing_source_locations={}; scope=observations_not_unique_groups; authority=none\n",
            self.project,
            self.counts.get("OriginCensus").copied().unwrap_or(0),
            self.counts.get("RendererCohort").copied().unwrap_or(0),
            self.counts.get("AlignmentDecision").copied().unwrap_or(0),
            self.counts
                .get("TargetResourceAdmission")
                .copied()
                .unwrap_or(0),
            self.counts.get("CohortProject").copied().unwrap_or(0),
            self.reasons,
            self.scalar_changes,
            self.collision_trials,
            self.lost_registration_trials,
            self.unbound_seeds,
            self.unretained_components,
            self.missing_source_locations,
        )
    }
}

pub(crate) fn record(observation: Observation<'_>) -> bool {
    let mut summary = SUMMARY
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(summary) = summary.as_mut() {
        summary.record(observation)
    } else {
        // Outside a project session preserve all diagnostics rather than lose evidence.
        true
    }
}

pub(crate) fn report_body() -> String {
    SUMMARY
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .as_ref()
        .map_or_else(String::new, Summary::line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immutable_selection_is_visible_to_workers_and_disabled_means_disabled() {
        let request = OnceLock::new();
        assert!(!request_matches(request.get(), 7));
        assert!(
            request
                .set(DetailRequest::Measures(BTreeSet::from(["7".to_owned()])))
                .is_ok()
        );
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    assert!(request_matches(request.get(), 7));
                    assert!(!request_matches(request.get(), 8));
                })
                .join()
                .unwrap();
        });
        assert!(!request_matches(Some(&DetailRequest::Disabled), 7));
        assert!(request_matches(Some(&DetailRequest::All), 8));
    }

    #[test]
    fn ordinary_and_unusual_observations_are_all_counted_without_hiding_unknowns() {
        let mut summary = Summary {
            project: "synthetic".to_owned(),
            ..Summary::default()
        };
        assert!(!summary.record(Observation::Census {
            bound_seed: true,
            retained_component: true,
            source_locations_present: true,
        }));
        assert!(summary.record(Observation::Census {
            bound_seed: false,
            retained_component: false,
            source_locations_present: false,
        }));
        assert!(!summary.record(Observation::Decision("proposal")));
        assert!(summary.record(Observation::Decision("future_unknown_reason")));
        assert!(summary.record(Observation::Decision("multiple_complete_assignments")));
        assert!(summary.record(Observation::Decision("global_pitched_sequence_drift")));
        assert!(!summary.record(Observation::Admission {
            scalar_changes: 2,
            collisions: false,
            lost_registration: false,
        }));
        assert!(summary.record(Observation::Admission {
            scalar_changes: 3,
            collisions: true,
            lost_registration: true,
        }));
        assert!(!summary.record(Observation::RendererCohort));
        assert!(!summary.record(Observation::CohortProject));
        let line = summary.line();
        assert!(line.contains("OriginCensus=2; RendererCohort=1; AlignmentDecision=4; TargetResourceAdmission=2; CohortProject=1"));
        assert!(line.contains("scalar_changes=5; collision_trials=1; lost_registration_trials=1; unbound_seeds=1; unretained_components=1; missing_source_locations=1"));
        assert!(line.contains("\"proposal\": 1"));
        assert!(line.contains("\"future_unknown_reason\": 1"));
        assert_eq!(line, summary.line()); // Report projection does not consume the counters.
    }
}
