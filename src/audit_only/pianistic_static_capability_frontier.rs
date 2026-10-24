//! r607 read-only exact static two-hand capability frontier audit.
//!
//! The previous r299/r300 reference assumption uses a fixed one-octave outer-span ceiling per
//! hand. This audit deliberately does not replace that ceiling with another purported human truth.
//! Instead it asks an assumption-minimal score-demand question: for every simultaneous physical-key
//! boundary, what is the smallest uniform per-hand outer-span ceiling that would admit at least one
//! exact left/right partition when each hand may cover at most five distinct keys?
//!
//! The result is a model-calibration demand frontier, not a human-playability claim. Finger motion,
//! transition time, hand morphology, black/white-key geometry beyond pitch order, substitution and
//! pedal technique remain outside this static frontier.

use crate::audit_only::pianistic_execution_plan_witness::{
    build_boundaries, build_demand_topology, PhysicalKeyObligation,
};
use crate::audit_only::pianistic_execution_score::{self, PianisticExecutionScore, ProjectionStatus};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-STATIC-CAPABILITY-FRONTIER-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticStaticCapabilityFrontierAudit";
pub const SCHEMA: &str = "pianistic-static-capability-frontier-v1";
pub const REFERENCE_BASELINE_SPAN_SEMITONES: i32 = 12;
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryFrontier {
    tick: i64,
    active_midi: Vec<i32>,
    partitions_checked: usize,
    minimum_uniform_outer_span_semitones: Option<i32>,
    same_key_overlap_excess_obligations: usize,
    unresolved_reason: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FrontierAudit {
    projection_status: &'static str,
    reason: String,
    source_execution_events: usize,
    physical_key_obligations: usize,
    boundary_count: usize,
    evaluated_boundaries: usize,
    unresolved_boundaries: usize,
    reference_baseline_failed_boundaries: usize,
    same_key_overlap_boundaries: usize,
    same_key_overlap_excess_obligations: usize,
    evaluated_boundary_maximum_minimum_uniform_outer_span_semitones: Option<i32>,
    worst_boundaries: Vec<BoundaryFrontier>,
    frontier_histogram: BTreeMap<i32, usize>,
    unresolved_reason_histogram: BTreeMap<&'static str, usize>,
}

fn hand_outer_span(pitches: &[i32]) -> i32 {
    let Some(lowest) = pitches.iter().min() else {
        return 0;
    };
    let highest = pitches.iter().max().unwrap_or(lowest);
    highest.saturating_sub(*lowest)
}

fn minimum_uniform_outer_span(active_midi: &[i32]) -> BoundaryFrontier {
    let mut sorted = active_midi.to_vec();
    sorted.sort_unstable();

    if sorted.iter().any(|midi| !(21..=108).contains(midi)) {
        return BoundaryFrontier {
            tick: 0,
            active_midi: sorted,
            partitions_checked: 0,
            minimum_uniform_outer_span_semitones: None,
            same_key_overlap_excess_obligations: 0,
            unresolved_reason: Some("key-outside-standard-88"),
        };
    }

    let original_len = sorted.len();
    sorted.dedup();
    let same_key_overlap_excess_obligations = original_len.saturating_sub(sorted.len());
    if sorted.len() > 10 {
        return BoundaryFrontier {
            tick: 0,
            active_midi: sorted,
            partitions_checked: 0,
            minimum_uniform_outer_span_semitones: None,
            same_key_overlap_excess_obligations,
            unresolved_reason: Some("more-than-ten-simultaneous-distinct-key-obligations"),
        };
    }

    let partition_count = 1usize.checked_shl(sorted.len() as u32).unwrap_or(0);
    let mut checked = 0usize;
    let mut best: Option<i32> = None;
    for mask in 0..partition_count {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (index, midi) in sorted.iter().copied().enumerate() {
            if mask & (1usize << index) == 0 {
                left.push(midi);
            } else {
                right.push(midi);
            }
        }
        checked = checked.saturating_add(1);
        if left.len() > 5 || right.len() > 5 {
            continue;
        }
        let required = hand_outer_span(&left).max(hand_outer_span(&right));
        best = Some(best.map_or(required, |current| current.min(required)));
    }

    BoundaryFrontier {
        tick: 0,
        active_midi: sorted,
        partitions_checked: checked,
        minimum_uniform_outer_span_semitones: best,
        same_key_overlap_excess_obligations,
        unresolved_reason: if best.is_some() {
            None
        } else {
            Some("no-two-hand-partition-with-five-distinct-keys-per-hand")
        },
    }
}

fn active_midi_for_boundary(
    active: &[usize],
    obligations: &[PhysicalKeyObligation],
) -> Result<Vec<i32>, String> {
    let mut out = Vec::with_capacity(active.len());
    for obligation_id in active {
        let obligation = obligations
            .get(*obligation_id)
            .ok_or_else(|| format!("boundary-obligation-id-out-of-range:{obligation_id}"))?;
        out.push(obligation.midi);
    }
    out.sort_unstable();
    Ok(out)
}

fn audit_score(score: &PianisticExecutionScore) -> FrontierAudit {
    if score.status != ProjectionStatus::Complete {
        return FrontierAudit {
            projection_status: "MODEL_INPUT_UNAVAILABLE",
            reason: score.reason.clone(),
            source_execution_events: score.execution_events.len(),
            physical_key_obligations: 0,
            boundary_count: 0,
            evaluated_boundaries: 0,
            unresolved_boundaries: 0,
            reference_baseline_failed_boundaries: 0,
            same_key_overlap_boundaries: 0,
            same_key_overlap_excess_obligations: 0,
            evaluated_boundary_maximum_minimum_uniform_outer_span_semitones: None,
            worst_boundaries: Vec::new(),
            frontier_histogram: BTreeMap::new(),
            unresolved_reason_histogram: BTreeMap::new(),
        };
    }

    let topology = match build_demand_topology(score) {
        Ok(topology) => topology,
        Err(reason) => {
            return FrontierAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: score.execution_events.len(),
                physical_key_obligations: 0,
                boundary_count: 0,
                evaluated_boundaries: 0,
                unresolved_boundaries: 0,
                reference_baseline_failed_boundaries: 0,
                same_key_overlap_boundaries: 0,
                same_key_overlap_excess_obligations: 0,
                evaluated_boundary_maximum_minimum_uniform_outer_span_semitones: None,
                worst_boundaries: Vec::new(),
                frontier_histogram: BTreeMap::new(),
                unresolved_reason_histogram: BTreeMap::new(),
            };
        }
    };
    let boundaries = match build_boundaries(&topology.obligations) {
        Ok(boundaries) => boundaries,
        Err(reason) => {
            return FrontierAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: topology.source_execution_events,
                physical_key_obligations: topology.obligations.len(),
                boundary_count: topology.boundary_count,
                evaluated_boundaries: 0,
                unresolved_boundaries: 0,
                reference_baseline_failed_boundaries: 0,
                same_key_overlap_boundaries: 0,
                same_key_overlap_excess_obligations: 0,
                evaluated_boundary_maximum_minimum_uniform_outer_span_semitones: None,
                worst_boundaries: Vec::new(),
                frontier_histogram: BTreeMap::new(),
                unresolved_reason_histogram: BTreeMap::new(),
            };
        }
    };

    let mut evaluated_boundaries = 0usize;
    let mut unresolved_boundaries = 0usize;
    let mut reference_baseline_failed_boundaries = 0usize;
    let mut same_key_overlap_boundaries = 0usize;
    let mut same_key_overlap_excess_obligations = 0usize;
    let mut evaluated_frontier_maximum = None::<i32>;
    let mut worst_boundaries = Vec::<BoundaryFrontier>::new();
    let mut frontier_histogram = BTreeMap::<i32, usize>::new();
    let mut unresolved_reason_histogram = BTreeMap::<&'static str, usize>::new();

    for boundary in boundaries {
        let active_midi = match active_midi_for_boundary(&boundary.active, &topology.obligations) {
            Ok(active_midi) => active_midi,
            Err(reason) => {
                return FrontierAudit {
                    projection_status: "MODEL_INPUT_UNAVAILABLE",
                    reason,
                    source_execution_events: topology.source_execution_events,
                    physical_key_obligations: topology.obligations.len(),
                    boundary_count: topology.boundary_count,
                    evaluated_boundaries,
                    unresolved_boundaries,
                    reference_baseline_failed_boundaries,
                    same_key_overlap_boundaries,
                    same_key_overlap_excess_obligations,
                    evaluated_boundary_maximum_minimum_uniform_outer_span_semitones:
                        evaluated_frontier_maximum,
                    worst_boundaries,
                    frontier_histogram,
                    unresolved_reason_histogram,
                };
            }
        };
        let mut frontier = minimum_uniform_outer_span(&active_midi);
        frontier.tick = boundary.tick;
        if frontier.same_key_overlap_excess_obligations > 0 {
            same_key_overlap_boundaries = same_key_overlap_boundaries.saturating_add(1);
            same_key_overlap_excess_obligations = same_key_overlap_excess_obligations
                .saturating_add(frontier.same_key_overlap_excess_obligations);
        }
        match frontier.minimum_uniform_outer_span_semitones {
            Some(required) => {
                evaluated_boundaries = evaluated_boundaries.saturating_add(1);
                *frontier_histogram.entry(required).or_default() += 1;
                if required > REFERENCE_BASELINE_SPAN_SEMITONES {
                    reference_baseline_failed_boundaries =
                        reference_baseline_failed_boundaries.saturating_add(1);
                }
                match evaluated_frontier_maximum {
                    None => {
                        evaluated_frontier_maximum = Some(required);
                        worst_boundaries.clear();
                        worst_boundaries.push(frontier);
                    }
                    Some(current) if required > current => {
                        evaluated_frontier_maximum = Some(required);
                        worst_boundaries.clear();
                        worst_boundaries.push(frontier);
                    }
                    Some(current) if required == current => worst_boundaries.push(frontier),
                    Some(_) => {}
                }
            }
            None => {
                unresolved_boundaries = unresolved_boundaries.saturating_add(1);
                let reason = frontier
                    .unresolved_reason
                    .unwrap_or("unspecified-static-frontier-gap");
                *unresolved_reason_histogram.entry(reason).or_default() += 1;
            }
        }
    }

    FrontierAudit {
        projection_status: if unresolved_boundaries == 0 {
            "COMPLETE"
        } else {
            "PARTIAL_UNMODELED_BOUNDARIES"
        },
        reason: if unresolved_boundaries == 0 {
            "exact-static-two-hand-frontier-complete".to_string()
        } else {
            format!("{unresolved_boundaries}-static-boundaries-outside-frontier-domain")
        },
        source_execution_events: topology.source_execution_events,
        physical_key_obligations: topology.obligations.len(),
        boundary_count: topology.boundary_count,
        evaluated_boundaries,
        unresolved_boundaries,
        reference_baseline_failed_boundaries,
        same_key_overlap_boundaries,
        same_key_overlap_excess_obligations,
        evaluated_boundary_maximum_minimum_uniform_outer_span_semitones:
            evaluated_frontier_maximum,
        worst_boundaries,
        frontier_histogram,
        unresolved_reason_histogram,
    }
}

fn format_histogram(histogram: &BTreeMap<i32, usize>) -> String {
    if histogram.is_empty() {
        return "none".to_string();
    }
    histogram
        .iter()
        .map(|(span, count)| format!("{span}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_reason_histogram(histogram: &BTreeMap<&'static str, usize>) -> String {
    if histogram.is_empty() {
        return "none".to_string();
    }
    histogram
        .iter()
        .map(|(reason, count)| format!("{reason}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn verify_runtime_integrity() -> Result<String, String> {
    let simple = minimum_uniform_outer_span(&[48, 55, 60, 67]);
    if simple.minimum_uniform_outer_span_semitones != Some(7) {
        return Err(format!(
            "frontier-simple-control-mismatch:{:?}",
            simple.minimum_uniform_outer_span_semitones
        ));
    }
    let former_reference_obstruction = minimum_uniform_outer_span(&[35, 49, 59, 63]);
    if former_reference_obstruction.minimum_uniform_outer_span_semitones != Some(14) {
        return Err(format!(
            "frontier-reference-obstruction-control-mismatch:{:?}",
            former_reference_obstruction.minimum_uniform_outer_span_semitones
        ));
    }
    let eleven_keys = minimum_uniform_outer_span(&(40..=50).collect::<Vec<_>>());
    if eleven_keys.minimum_uniform_outer_span_semitones.is_some()
        || eleven_keys.unresolved_reason
            != Some("more-than-ten-simultaneous-distinct-key-obligations")
    {
        return Err("frontier-cardinality-control-mismatch".to_string());
    }
    let duplicate = minimum_uniform_outer_span(&[60, 60]);
    if duplicate.minimum_uniform_outer_span_semitones != Some(0)
        || duplicate.same_key_overlap_excess_obligations != 1
        || duplicate.unresolved_reason.is_some()
    {
        return Err("frontier-duplicate-key-control-mismatch".to_string());
    }
    Ok(
        "PASS; ExactDistinctKeyPartitionEnumeration yes; FiveDistinctKeysPerHand yes; SameKeyOverlapSeparated yes; FixedSpanThreshold none; HumanPlayabilityAuthority no"
            .to_string(),
    )
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("=================================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; InputAuthority: PianisticExecutionScoreAuthority+PianisticExecutionPlanWitnessDemandTopology; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("FrontierContract: every simultaneous boundary is projected to its distinct physical key positions and exhaustively partitioned across labelled left/right hands; each hand may receive at most five distinct keys; the audit minimizes max(LeftOuterSpan,RightOuterSpan) over all such partitions. Same-key obligation multiplicity is counted separately and never converted into a feasibility claim. No fixed span threshold is used to define the frontier.\n");
    let _ = writeln!(
        out,
        "ReferenceComparison: ExistingReferenceBaselineOuterSpanSemitones {REFERENCE_BASELINE_SPAN_SEMITONES}; comparison only; r299 authority remains unchanged."
    );
    out.push_str("InterpretationBoundary: this is a positional lower-bound score-demand/calibration frontier. Same-key overlaps remain a separate temporal/technique demand. The audit does not state that a human hand supports the reported span, does not model transitions or tempo, and cannot authorize score mutation.\n");
    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticStaticCapabilityFrontierAudit runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "StaticCapabilityFrontierRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; Reason score-title-missing; HumanPlayabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let score = pianistic_execution_score::project_file(&PathBuf::from(format!("{title}.musicxml")));
    let audit = audit_score(&score);
    let _ = writeln!(
        out,
        "InputProjectionStatus: {}; Reason {}; SourceExecutionEvents {}; PhysicalKeyObligations {}; BoundaryCount {}; EvaluatedBoundaries {}; UnresolvedBoundaries {}.",
        audit.projection_status,
        audit.reason,
        audit.source_execution_events,
        audit.physical_key_obligations,
        audit.boundary_count,
        audit.evaluated_boundaries,
        audit.unresolved_boundaries,
    );
    let _ = writeln!(
        out,
        "DistinctKeyStaticSpanFrontier: EvaluatedBoundaryMaximumMinimumUniformOuterSpanSemitones {}; ReferenceBaselineFailedBoundaries {}; SameKeyOverlapBoundaries {}; SameKeyOverlapExcessObligations {}; FrontierHistogram [{}]; UnresolvedReasonHistogram [{}].",
        audit
            .evaluated_boundary_maximum_minimum_uniform_outer_span_semitones
            .map_or_else(|| "none".to_string(), |value| value.to_string()),
        audit.reference_baseline_failed_boundaries,
        audit.same_key_overlap_boundaries,
        audit.same_key_overlap_excess_obligations,
        format_histogram(&audit.frontier_histogram),
        format_reason_histogram(&audit.unresolved_reason_histogram),
    );
    if audit.worst_boundaries.is_empty() {
        out.push_str("WorstStaticSpanBoundary: none.\n");
    } else {
        for boundary in audit.worst_boundaries.iter().take(8) {
            let midi = boundary
                .active_midi
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            let _ = writeln!(
                out,
                "WorstStaticSpanBoundary: Tick {}; ActiveMidi [{}]; ActiveKeyCount {}; MinimumUniformOuterSpanSemitones {}; PartitionsChecked {}.",
                boundary.tick,
                midi,
                boundary.active_midi.len(),
                boundary
                    .minimum_uniform_outer_span_semitones
                    .map_or_else(|| "none".to_string(), |value| value.to_string()),
                boundary.partitions_checked,
            );
        }
        if audit.worst_boundaries.len() > 8 {
            let _ = writeln!(
                out,
                "WorstStaticSpanBoundaryAdditionalCount: {}.",
                audit.worst_boundaries.len() - 8
            );
        }
    }
    let _ = writeln!(
        out,
        "HumanPlayabilityClaim {PLAYABILITY_CLAIM}; TransitionTiming NOT_EVALUATED; TempoFeasibility NOT_EVALUATED; CapabilityCalibration REQUIRED_BEFORE_HUMAN_CLAIM.\n"
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_two_cluster_boundary_has_seven_semitone_frontier() {
        let frontier = minimum_uniform_outer_span(&[48, 55, 60, 67]);
        assert_eq!(frontier.minimum_uniform_outer_span_semitones, Some(7));
    }

    #[test]
    fn former_twelve_semitone_reference_obstruction_has_fourteen_semitone_frontier() {
        let frontier = minimum_uniform_outer_span(&[35, 49, 59, 63]);
        assert_eq!(frontier.minimum_uniform_outer_span_semitones, Some(14));
    }

    #[test]
    fn more_than_ten_keys_is_outside_two_hand_five_finger_frontier() {
        let pitches = (40..=50).collect::<Vec<_>>();
        let frontier = minimum_uniform_outer_span(&pitches);
        assert_eq!(frontier.minimum_uniform_outer_span_semitones, None);
        assert_eq!(
            frontier.unresolved_reason,
            Some("more-than-ten-simultaneous-distinct-key-obligations")
        );
    }

    #[test]
    fn overlapping_independent_same_key_obligations_are_separated_from_span_frontier() {
        let frontier = minimum_uniform_outer_span(&[60, 60]);
        assert_eq!(frontier.minimum_uniform_outer_span_semitones, Some(0));
        assert_eq!(frontier.same_key_overlap_excess_obligations, 1);
        assert_eq!(frontier.unresolved_reason, None);
    }

    #[test]
    fn runtime_integrity_exercises_frontier_domain() {
        assert!(verify_runtime_integrity().is_ok());
    }

    #[test]
    fn report_keeps_human_claim_outside_frontier() {
        let text = build_for_score_title(None);
        assert!(text.contains("HumanPlayabilityClaim NOT_EVALUATED"));
        assert!(text.contains("FixedSpanThreshold none"));
    }
}
