//! r610 read-only keyboard-state transition frontier.
//!
//! r607 closed the static distinct-key span-demand axis and r608 closed the exact score-side
//! timing axis. This audit composes those two score truths without inventing a human capability
//! threshold: for every pair of consecutive attack boundaries it compares the *distinct active
//! physical key sets* on the 88-key keyboard and reports exact set persistence, entering/leaving
//! keys, nearest-key geometry and a geometry/time quotient. The quotient is a score-demand index,
//! not a hand speed and not a playability criterion.
//!
//! Same-key overlapping obligations remain a separate temporal/technique demand owned by r608;
//! this owner intentionally deduplicates active MIDI keys before geometric comparison. It assigns
//! no hand, finger, staff affinity or execution witness and has no writer or repair authority.

use crate::audit_only::pianistic_execution_plan_witness::{
    PhysicalKeyObligation, build_boundaries, build_demand_topology,
};
use crate::audit_only::pianistic_execution_score::{
    PianisticExecutionEvent, PianisticExecutionScore, ProjectionStatus, TempoPoint, project_file,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-KEYBOARD-STATE-TRANSITION-FRONTIER-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticKeyboardStateTransitionFrontierAudit";
pub const SCHEMA: &str = "pianistic-keyboard-state-transition-frontier-v1";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

#[derive(Debug, Clone, PartialEq, Eq)]
struct KeyboardStateTransition {
    from_tick: i64,
    to_tick: i64,
    delta_ticks: i64,
    delta_time_us: u64,
    from_keys: Vec<i32>,
    to_keys: Vec<i32>,
    persistent_keys: Vec<i32>,
    entering_keys: Vec<i32>,
    leaving_keys: Vec<i32>,
    directed_to_from_nearest_max_semitones: i32,
    directed_from_to_nearest_max_semitones: i32,
    symmetric_hausdorff_semitones: i32,
    geometry_time_quotient_milli_semitones_per_second: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TransitionFrontierAudit {
    projection_status: &'static str,
    reason: String,
    source_execution_events: usize,
    physical_key_obligations: usize,
    attack_boundary_count: usize,
    transition_count: usize,
    exact_state_repeat_transitions: usize,
    changed_state_transitions: usize,
    maximum_distinct_active_keys: usize,
    maximum_entering_keys: usize,
    maximum_leaving_keys: usize,
    maximum_symmetric_hausdorff_semitones: Option<i32>,
    maximum_geometry_time_quotient_milli_semitones_per_second: Option<u64>,
    hausdorff_histogram: BTreeMap<i32, usize>,
    entering_count_histogram: BTreeMap<usize, usize>,
    top_geometry_transitions: Vec<KeyboardStateTransition>,
    top_geometry_time_transitions: Vec<KeyboardStateTransition>,
}

fn distinct_active_midi(
    active: &[usize],
    obligations: &[PhysicalKeyObligation],
) -> Result<Vec<i32>, String> {
    let mut set = BTreeSet::new();
    for id in active {
        let obligation = obligations
            .get(*id)
            .ok_or_else(|| format!("boundary-obligation-id-out-of-range:{id}"))?;
        if !(21..=108).contains(&obligation.midi) {
            return Err(format!("active-key-outside-standard-88:midi-{}", obligation.midi));
        }
        set.insert(obligation.midi);
    }
    Ok(set.into_iter().collect())
}

fn intersection(left: &[i32], right: &[i32]) -> Vec<i32> {
    let right_set = right.iter().copied().collect::<BTreeSet<_>>();
    left.iter()
        .copied()
        .filter(|midi| right_set.contains(midi))
        .collect()
}

fn difference(left: &[i32], right: &[i32]) -> Vec<i32> {
    let right_set = right.iter().copied().collect::<BTreeSet<_>>();
    left.iter()
        .copied()
        .filter(|midi| !right_set.contains(midi))
        .collect()
}

fn directed_nearest_max(from: &[i32], to: &[i32]) -> Option<i32> {
    if from.is_empty() || to.is_empty() {
        return None;
    }
    from.iter()
        .map(|source| {
            to.iter()
                .map(|target| source.saturating_sub(*target).abs())
                .min()
                .unwrap_or(0)
        })
        .max()
}

fn geometry_time_quotient_milli(
    hausdorff_semitones: i32,
    delta_time_us: u64,
) -> Result<u64, String> {
    if hausdorff_semitones < 0 || delta_time_us == 0 {
        return Err("invalid-geometry-time-quotient-input".to_string());
    }
    let numerator = u128::try_from(hausdorff_semitones)
        .map_err(|_| "hausdorff-negative-conversion".to_string())?
        .saturating_mul(1_000_000_000_u128);
    let value = numerator / u128::from(delta_time_us);
    u64::try_from(value).map_err(|_| "geometry-time-quotient-overflow".to_string())
}

fn transition_between(
    from_tick: i64,
    from_time_us: u64,
    from_keys: Vec<i32>,
    to_tick: i64,
    to_time_us: u64,
    to_keys: Vec<i32>,
) -> Result<KeyboardStateTransition, String> {
    let delta_ticks = to_tick
        .checked_sub(from_tick)
        .ok_or_else(|| "transition-tick-underflow".to_string())?;
    let delta_time_us = to_time_us
        .checked_sub(from_time_us)
        .ok_or_else(|| "transition-time-underflow".to_string())?;
    if delta_ticks <= 0 || delta_time_us == 0 {
        return Err("non-positive-attack-transition-window".to_string());
    }
    if from_keys.is_empty() || to_keys.is_empty() {
        return Err("attack-boundary-distinct-key-state-empty".to_string());
    }
    let persistent_keys = intersection(&from_keys, &to_keys);
    let entering_keys = difference(&to_keys, &from_keys);
    let leaving_keys = difference(&from_keys, &to_keys);
    let directed_to_from_nearest_max_semitones = directed_nearest_max(&to_keys, &from_keys)
        .ok_or_else(|| "directed-to-from-nearest-unavailable".to_string())?;
    let directed_from_to_nearest_max_semitones = directed_nearest_max(&from_keys, &to_keys)
        .ok_or_else(|| "directed-from-to-nearest-unavailable".to_string())?;
    let symmetric_hausdorff_semitones = directed_to_from_nearest_max_semitones
        .max(directed_from_to_nearest_max_semitones);
    let geometry_time_quotient_milli_semitones_per_second =
        geometry_time_quotient_milli(symmetric_hausdorff_semitones, delta_time_us)?;
    Ok(KeyboardStateTransition {
        from_tick,
        to_tick,
        delta_ticks,
        delta_time_us,
        from_keys,
        to_keys,
        persistent_keys,
        entering_keys,
        leaving_keys,
        directed_to_from_nearest_max_semitones,
        directed_from_to_nearest_max_semitones,
        symmetric_hausdorff_semitones,
        geometry_time_quotient_milli_semitones_per_second,
    })
}

fn collect_transitions(
    score: &PianisticExecutionScore,
) -> Result<(usize, usize, Vec<KeyboardStateTransition>), String> {
    let topology = build_demand_topology(score)?;
    let boundaries = build_boundaries(&topology.obligations)?;
    let attack_boundaries = boundaries
        .iter()
        .filter(|boundary| !boundary.attacking.is_empty())
        .collect::<Vec<_>>();
    let mut transitions = Vec::with_capacity(attack_boundaries.len().saturating_sub(1));
    for pair in attack_boundaries.windows(2) {
        let from = pair[0];
        let to = pair[1];
        transitions.push(transition_between(
            from.tick,
            from.time_us_floor,
            distinct_active_midi(&from.active, &topology.obligations)?,
            to.tick,
            to.time_us_floor,
            distinct_active_midi(&to.active, &topology.obligations)?,
        )?);
    }
    Ok((topology.source_execution_events, topology.obligations.len(), transitions))
}

fn collect_audit(score: &PianisticExecutionScore) -> TransitionFrontierAudit {
    if score.status != ProjectionStatus::Complete {
        return TransitionFrontierAudit {
            projection_status: "MODEL_INPUT_UNAVAILABLE",
            reason: score.reason.clone(),
            source_execution_events: score.execution_events.len(),
            physical_key_obligations: 0,
            attack_boundary_count: 0,
            transition_count: 0,
            exact_state_repeat_transitions: 0,
            changed_state_transitions: 0,
            maximum_distinct_active_keys: 0,
            maximum_entering_keys: 0,
            maximum_leaving_keys: 0,
            maximum_symmetric_hausdorff_semitones: None,
            maximum_geometry_time_quotient_milli_semitones_per_second: None,
            hausdorff_histogram: BTreeMap::new(),
            entering_count_histogram: BTreeMap::new(),
            top_geometry_transitions: Vec::new(),
            top_geometry_time_transitions: Vec::new(),
        };
    }
    let topology = match build_demand_topology(score) {
        Ok(topology) => topology,
        Err(reason) => {
            return TransitionFrontierAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: score.execution_events.len(),
                physical_key_obligations: 0,
                attack_boundary_count: 0,
                transition_count: 0,
                exact_state_repeat_transitions: 0,
                changed_state_transitions: 0,
                maximum_distinct_active_keys: 0,
                maximum_entering_keys: 0,
                maximum_leaving_keys: 0,
                maximum_symmetric_hausdorff_semitones: None,
                maximum_geometry_time_quotient_milli_semitones_per_second: None,
                hausdorff_histogram: BTreeMap::new(),
                entering_count_histogram: BTreeMap::new(),
                top_geometry_transitions: Vec::new(),
                top_geometry_time_transitions: Vec::new(),
            };
        }
    };
    let boundaries = match build_boundaries(&topology.obligations) {
        Ok(boundaries) => boundaries,
        Err(reason) => {
            return TransitionFrontierAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: topology.source_execution_events,
                physical_key_obligations: topology.obligations.len(),
                attack_boundary_count: topology.attack_boundary_count,
                transition_count: 0,
                exact_state_repeat_transitions: 0,
                changed_state_transitions: 0,
                maximum_distinct_active_keys: 0,
                maximum_entering_keys: 0,
                maximum_leaving_keys: 0,
                maximum_symmetric_hausdorff_semitones: None,
                maximum_geometry_time_quotient_milli_semitones_per_second: None,
                hausdorff_histogram: BTreeMap::new(),
                entering_count_histogram: BTreeMap::new(),
                top_geometry_transitions: Vec::new(),
                top_geometry_time_transitions: Vec::new(),
            };
        }
    };
    let attack_boundary_count = boundaries
        .iter()
        .filter(|boundary| !boundary.attacking.is_empty())
        .count();
    let maximum_distinct_active_keys = boundaries
        .iter()
        .filter_map(|boundary| distinct_active_midi(&boundary.active, &topology.obligations).ok())
        .map(|keys| keys.len())
        .max()
        .unwrap_or(0);
    let (_, _, transitions) = match collect_transitions(score) {
        Ok(value) => value,
        Err(reason) => {
            return TransitionFrontierAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: topology.source_execution_events,
                physical_key_obligations: topology.obligations.len(),
                attack_boundary_count,
                transition_count: 0,
                exact_state_repeat_transitions: 0,
                changed_state_transitions: 0,
                maximum_distinct_active_keys,
                maximum_entering_keys: 0,
                maximum_leaving_keys: 0,
                maximum_symmetric_hausdorff_semitones: None,
                maximum_geometry_time_quotient_milli_semitones_per_second: None,
                hausdorff_histogram: BTreeMap::new(),
                entering_count_histogram: BTreeMap::new(),
                top_geometry_transitions: Vec::new(),
                top_geometry_time_transitions: Vec::new(),
            };
        }
    };

    let exact_state_repeat_transitions = transitions
        .iter()
        .filter(|item| item.from_keys == item.to_keys)
        .count();
    let changed_state_transitions = transitions.len().saturating_sub(exact_state_repeat_transitions);
    let maximum_entering_keys = transitions
        .iter()
        .map(|item| item.entering_keys.len())
        .max()
        .unwrap_or(0);
    let maximum_leaving_keys = transitions
        .iter()
        .map(|item| item.leaving_keys.len())
        .max()
        .unwrap_or(0);
    let maximum_symmetric_hausdorff_semitones = transitions
        .iter()
        .map(|item| item.symmetric_hausdorff_semitones)
        .max();
    let maximum_geometry_time_quotient_milli_semitones_per_second = transitions
        .iter()
        .map(|item| item.geometry_time_quotient_milli_semitones_per_second)
        .max();
    let mut hausdorff_histogram = BTreeMap::new();
    let mut entering_count_histogram = BTreeMap::new();
    for item in &transitions {
        *hausdorff_histogram
            .entry(item.symmetric_hausdorff_semitones)
            .or_default() += 1;
        *entering_count_histogram
            .entry(item.entering_keys.len())
            .or_default() += 1;
    }
    let mut top_geometry_transitions = transitions.clone();
    top_geometry_transitions.sort_by_key(|item| {
        (
            std::cmp::Reverse(item.symmetric_hausdorff_semitones),
            item.delta_time_us,
            item.to_tick,
        )
    });
    top_geometry_transitions.truncate(8);
    let mut top_geometry_time_transitions = transitions.clone();
    top_geometry_time_transitions.sort_by_key(|item| {
        (
            std::cmp::Reverse(item.geometry_time_quotient_milli_semitones_per_second),
            std::cmp::Reverse(item.symmetric_hausdorff_semitones),
            item.to_tick,
        )
    });
    top_geometry_time_transitions.truncate(8);

    TransitionFrontierAudit {
        projection_status: "COMPLETE",
        reason: "exact-keyboard-state-transition-frontier-complete".to_string(),
        source_execution_events: topology.source_execution_events,
        physical_key_obligations: topology.obligations.len(),
        attack_boundary_count,
        transition_count: transitions.len(),
        exact_state_repeat_transitions,
        changed_state_transitions,
        maximum_distinct_active_keys,
        maximum_entering_keys,
        maximum_leaving_keys,
        maximum_symmetric_hausdorff_semitones,
        maximum_geometry_time_quotient_milli_semitones_per_second,
        hausdorff_histogram,
        entering_count_histogram,
        top_geometry_transitions,
        top_geometry_time_transitions,
    }
}

fn format_histogram<K: std::fmt::Display>(histogram: &BTreeMap<K, usize>) -> String {
    if histogram.is_empty() {
        return "none".to_string();
    }
    histogram
        .iter()
        .map(|(key, count)| format!("{key}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn format_keys(keys: &[i32]) -> String {
    if keys.is_empty() {
        return "none".to_string();
    }
    keys.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn synthetic_score(events: Vec<PianisticExecutionEvent>) -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "synthetic-keyboard-state-transition-frontier".to_string(),
        source_path: PathBuf::from("synthetic.musicxml"),
        pitched_note_segments: events.len(),
        execution_events: events,
        tie_continuation_segments: 0,
        tempo_points: vec![TempoPoint {
            tick: 0,
            milli_bpm: 120_000,
        }],
        terminal_tick: 384,
        terminal_physical_time_us_floor: Some(2_000_000),
        musical_execution_fingerprint_sha256: "synthetic-musical-fingerprint".to_string(),
        source_affinity_fingerprint_sha256: "synthetic-affinity-fingerprint".to_string(),
    }
}

fn synthetic_event(
    midi: i32,
    attack_tick: i64,
    release_tick: i64,
    attack_time_us_floor: u64,
    release_time_us_floor: u64,
) -> PianisticExecutionEvent {
    PianisticExecutionEvent {
        part_id: "P1".to_string(),
        source_staff: "1".to_string(),
        source_voice: "1".to_string(),
        midi,
        attack_tick,
        release_tick,
        attack_time_us_floor,
        release_time_us_floor,
        note_segments: 1,
    }
}

fn verify_runtime_integrity() -> Result<String, String> {
    let sequential = synthetic_score(vec![
        synthetic_event(60, 0, 96, 0, 500_000),
        synthetic_event(72, 96, 192, 500_000, 1_000_000),
    ]);
    let audit = collect_audit(&sequential);
    if audit.projection_status != "COMPLETE"
        || audit.transition_count != 1
        || audit.maximum_symmetric_hausdorff_semitones != Some(12)
        || audit.maximum_geometry_time_quotient_milli_semitones_per_second != Some(24_000)
    {
        return Err("synthetic-sequential-geometry-time-frontier-mismatch".to_string());
    }

    let held_anchor = synthetic_score(vec![
        synthetic_event(60, 0, 192, 0, 1_000_000),
        synthetic_event(72, 96, 192, 500_000, 1_000_000),
    ]);
    let held_audit = collect_audit(&held_anchor);
    if held_audit.transition_count != 1
        || held_audit.top_geometry_transitions[0].persistent_keys != vec![60]
        || held_audit.top_geometry_transitions[0].entering_keys != vec![72]
        || held_audit.maximum_symmetric_hausdorff_semitones != Some(12)
    {
        return Err("synthetic-held-anchor-transition-mismatch".to_string());
    }

    let same_key_duplicate_geometry = synthetic_score(vec![
        synthetic_event(60, 0, 192, 0, 1_000_000),
        synthetic_event(60, 96, 288, 500_000, 1_500_000),
    ]);
    let same_key_audit = collect_audit(&same_key_duplicate_geometry);
    if same_key_audit.transition_count != 1
        || same_key_audit.exact_state_repeat_transitions != 1
        || same_key_audit.maximum_symmetric_hausdorff_semitones != Some(0)
    {
        return Err("same-key-overlap-not-separated-from-distinct-key-geometry".to_string());
    }

    Ok("PASS; DistinctPhysicalKeySetGeometry yes; AttackBoundaryTime exact; SameKeyOverlapSeparated yes; HandAssignment none; FingerAssignment none; HumanThreshold none; PlayabilityClaim NOT_EVALUATED".to_string())
}

pub fn build_for_score_title(score_title: Option<&str>) -> String {
    let mut out = String::new();
    out.push_str(SECTION);
    out.push('\n');
    out.push_str("==============================================================\n");
    let _ = writeln!(
        out,
        "ChangeClass: {CHANGE_CLASS}; Authority: {AUTHORITY}; Schema: {SCHEMA}; InputAuthority: PianisticExecutionScoreAuthority+PianisticExecutionPlanWitnessDemandTopology; WriterAuthority: none; MusicMutation: no; MasterApply: no; RenderXmlWrite: no."
    );
    out.push_str("TransitionFrontierContract: consecutive attack boundaries are compared only as distinct active physical-key sets on the standard 88-key pitch axis. Set persistence, entering/leaving keys and symmetric nearest-key/Hausdorff geometry are exact score-demand observations. The geometry/time quotient is a dimensioned score index only; it is not hand velocity and has no human threshold.\n");
    out.push_str("SameKeyBoundary: non-identical overlapping obligations on one physical key remain owned by the r608 temporal/technique frontier. This audit deduplicates MIDI keys for geometry and therefore cannot hide or resolve same-key re-articulation technique.\n");
    out.push_str("InterpretationBoundary: no staff-to-hand inference, no hand partition, no fingering, no movement capability and no tempo-feasibility decision is introduced. This audit cannot authorize score mutation or repair.\n");
    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticKeyboardStateTransitionFrontierAudit runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "KeyboardStateTransitionRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; KeyboardStateTransitionStatus NOT_EVALUATED; HumanPlayabilityClaim NOT_EVALUATED.\n");
        out.push_str("HumanPlayabilityClaim NOT_EVALUATED; HandTransitionCapability NOT_BOUND; FingerTransitionCapability NOT_BOUND; TempoFeasibility NOT_EVALUATED; CapabilityCalibration REQUIRED_BEFORE_HUMAN_CLAIM.\n\n");
        return out;
    };
    let score = project_file(&PathBuf::from(format!("{title}.musicxml")));
    let audit = collect_audit(&score);
    let _ = writeln!(
        out,
        "InputProjectionStatus: {}; Reason {}; SourceExecutionEvents {}; PhysicalKeyObligations {}; AttackBoundaryCount {}; TransitionCount {}.",
        audit.projection_status,
        audit.reason,
        audit.source_execution_events,
        audit.physical_key_obligations,
        audit.attack_boundary_count,
        audit.transition_count,
    );
    let _ = writeln!(
        out,
        "KeyboardStateFrontier: ExactStateRepeatTransitions {}; ChangedStateTransitions {}; MaximumDistinctActiveKeys {}; MaximumEnteringKeys {}; MaximumLeavingKeys {}; MaximumSymmetricHausdorffSemitones {}; MaximumGeometryTimeQuotientMilliSemitonesPerSecond {}; HumanThreshold none.",
        audit.exact_state_repeat_transitions,
        audit.changed_state_transitions,
        audit.maximum_distinct_active_keys,
        audit.maximum_entering_keys,
        audit.maximum_leaving_keys,
        audit.maximum_symmetric_hausdorff_semitones
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        audit.maximum_geometry_time_quotient_milli_semitones_per_second
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
    );
    let _ = writeln!(
        out,
        "KeyboardStateHistograms: SymmetricHausdorff [{}]; EnteringKeyCount [{}].",
        format_histogram(&audit.hausdorff_histogram),
        format_histogram(&audit.entering_count_histogram),
    );
    if audit.top_geometry_transitions.is_empty() {
        out.push_str("TopGeometryTransition: none.\n");
    } else {
        for item in &audit.top_geometry_transitions {
            let _ = writeln!(
                out,
                "TopGeometryTransition: FromTick {}; ToTick {}; DeltaTicks {}; DeltaTimeUs {}; FromKeys [{}]; ToKeys [{}]; Persistent [{}]; Entering [{}]; Leaving [{}]; DirectedToFromNearestMaxSemitones {}; DirectedFromToNearestMaxSemitones {}; SymmetricHausdorffSemitones {}; GeometryTimeQuotientMilliSemitonesPerSecond {}.",
                item.from_tick,
                item.to_tick,
                item.delta_ticks,
                item.delta_time_us,
                format_keys(&item.from_keys),
                format_keys(&item.to_keys),
                format_keys(&item.persistent_keys),
                format_keys(&item.entering_keys),
                format_keys(&item.leaving_keys),
                item.directed_to_from_nearest_max_semitones,
                item.directed_from_to_nearest_max_semitones,
                item.symmetric_hausdorff_semitones,
                item.geometry_time_quotient_milli_semitones_per_second,
            );
        }
    }
    if audit.top_geometry_time_transitions.is_empty() {
        out.push_str("TopGeometryTimeTransition: none.\n");
    } else {
        for item in &audit.top_geometry_time_transitions {
            let _ = writeln!(
                out,
                "TopGeometryTimeTransition: FromTick {}; ToTick {}; DeltaTimeUs {}; SymmetricHausdorffSemitones {}; GeometryTimeQuotientMilliSemitonesPerSecond {}; Persistent [{}]; Entering [{}]; Leaving [{}].",
                item.from_tick,
                item.to_tick,
                item.delta_time_us,
                item.symmetric_hausdorff_semitones,
                item.geometry_time_quotient_milli_semitones_per_second,
                format_keys(&item.persistent_keys),
                format_keys(&item.entering_keys),
                format_keys(&item.leaving_keys),
            );
        }
    }
    out.push_str("CapabilityHandoff: r610 closes the assignment-free spatiotemporal keyboard-state demand surface. A later capability take may bind independent hand/finger evidence against explicit configuration and transition signatures; these score-derived geometry values themselves are not capability evidence.\n");
    let _ = writeln!(
        out,
        "HumanPlayabilityClaim {PLAYABILITY_CLAIM}; HandTransitionCapability NOT_BOUND; FingerTransitionCapability NOT_BOUND; TempoFeasibility NOT_EVALUATED; CapabilityCalibration REQUIRED_BEFORE_HUMAN_CLAIM.\n"
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_octave_change_has_exact_hausdorff_and_time_quotient() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 500_000),
            synthetic_event(72, 96, 192, 500_000, 1_000_000),
        ]);
        let audit = collect_audit(&score);
        assert_eq!(audit.transition_count, 1);
        assert_eq!(audit.maximum_symmetric_hausdorff_semitones, Some(12));
        assert_eq!(
            audit.maximum_geometry_time_quotient_milli_semitones_per_second,
            Some(24_000)
        );
    }

    #[test]
    fn persisted_key_is_reported_as_anchor_not_reassigned_hand() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 192, 0, 1_000_000),
            synthetic_event(72, 96, 192, 500_000, 1_000_000),
        ]);
        let audit = collect_audit(&score);
        let item = &audit.top_geometry_transitions[0];
        assert_eq!(item.persistent_keys, vec![60]);
        assert_eq!(item.entering_keys, vec![72]);
        assert_eq!(item.leaving_keys, Vec::<i32>::new());
    }

    #[test]
    fn same_key_reattack_is_geometrically_repeat_and_remains_r608_technique_demand() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 192, 0, 1_000_000),
            synthetic_event(60, 96, 288, 500_000, 1_500_000),
        ]);
        let audit = collect_audit(&score);
        assert_eq!(audit.exact_state_repeat_transitions, 1);
        assert_eq!(audit.maximum_symmetric_hausdorff_semitones, Some(0));
    }

    #[test]
    fn directed_nearest_geometry_is_symmetric_only_after_max_composition() {
        assert_eq!(directed_nearest_max(&[60], &[60, 72]), Some(0));
        assert_eq!(directed_nearest_max(&[60, 72], &[60]), Some(12));
    }

    #[test]
    fn runtime_integrity_exercises_spatiotemporal_domain() {
        assert!(verify_runtime_integrity().expect("integrity").starts_with("PASS;"));
    }

    #[test]
    fn report_keeps_human_capability_unbound() {
        let text = build_for_score_title(None);
        assert!(text.contains("HumanPlayabilityClaim NOT_EVALUATED"));
        assert!(text.contains("HandTransitionCapability NOT_BOUND"));
        assert!(!text.contains("REALIZATION_WITNESS_FOUND"));
    }
}
