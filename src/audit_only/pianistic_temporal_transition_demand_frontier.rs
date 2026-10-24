//! r608 read-only temporal transition-demand frontier.
//!
//! This module closes the score-side time axis for the playability-analysis branch without
//! inventing a human movement threshold. It projects exact attack-boundary windows from the r292
//! physical timeline and decomposes overlapping obligations on the same physical piano key into
//! explicit temporal demand classes. The output is calibration demand only: it does not assign
//! hands or fingers, does not decide whether a pianist can satisfy a transition, and cannot mutate
//! the score.

use crate::audit_only::pianistic_execution_plan_witness::{
    PhysicalKeyObligation, build_boundaries, build_demand_topology,
};
use crate::audit_only::pianistic_execution_score::{
    PianisticExecutionEvent, PianisticExecutionScore, ProjectionStatus, TempoPoint, project_file,
};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;

pub const SECTION: &str = "PIANISTIC-TEMPORAL-TRANSITION-DEMAND-FRONTIER-AUDIT";
pub const CHANGE_CLASS: &str = "AuditOnly";
pub const AUTHORITY: &str = "PianisticTemporalTransitionDemandFrontierAudit";
pub const SCHEMA: &str = "pianistic-temporal-transition-demand-frontier-v1";
pub const PLAYABILITY_CLAIM: &str = "NOT_EVALUATED";

#[derive(Debug, Clone, PartialEq, Eq)]
struct AttackWindow {
    from_tick: i64,
    to_tick: i64,
    from_time_us_floor: u64,
    to_time_us_floor: u64,
    delta_ticks: i64,
    delta_time_us: u64,
    to_attack_count: usize,
    to_active_obligation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SameKeyOverlapPair {
    midi: i32,
    first_obligation_id: usize,
    second_obligation_id: usize,
    first_attack_tick: i64,
    second_attack_tick: i64,
    first_release_tick: i64,
    second_release_tick: i64,
    inter_attack_ticks: i64,
    inter_attack_time_us: u64,
    remaining_first_hold_time_us_at_second_attack: u64,
    simultaneous_attack: bool,
    second_release_nested_in_first: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QuantilesU64 {
    minimum: u64,
    q25: u64,
    median: u64,
    q75: u64,
    maximum: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QuantilesI64 {
    minimum: i64,
    q25: i64,
    median: i64,
    q75: i64,
    maximum: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TemporalDemandAudit {
    projection_status: &'static str,
    reason: String,
    source_execution_events: usize,
    physical_key_obligations: usize,
    attack_boundary_count: usize,
    positive_attack_window_count: usize,
    attack_window_time_quantiles_us: Option<QuantilesU64>,
    attack_window_tick_quantiles: Option<QuantilesI64>,
    shortest_attack_windows: Vec<AttackWindow>,
    same_key_overlap_pairs: usize,
    simultaneous_same_key_attack_pairs: usize,
    staggered_same_key_reattack_pairs: usize,
    nested_release_overlap_pairs: usize,
    extending_release_overlap_pairs: usize,
    minimum_positive_same_key_reattack_time_us: Option<u64>,
    maximum_remaining_prior_hold_time_us_at_reattack: Option<u64>,
    overlap_pairs_by_midi: BTreeMap<i32, usize>,
    sampled_same_key_overlaps: Vec<SameKeyOverlapPair>,
}

fn quantile_index(len: usize, numerator: usize, denominator: usize) -> usize {
    if len <= 1 || denominator == 0 {
        return 0;
    }
    (len - 1).saturating_mul(numerator) / denominator
}

fn quantiles_u64(values: &[u64]) -> Option<QuantilesU64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    Some(QuantilesU64 {
        minimum: sorted[0],
        q25: sorted[quantile_index(sorted.len(), 1, 4)],
        median: sorted[quantile_index(sorted.len(), 1, 2)],
        q75: sorted[quantile_index(sorted.len(), 3, 4)],
        maximum: *sorted.last().unwrap_or(&sorted[0]),
    })
}

fn quantiles_i64(values: &[i64]) -> Option<QuantilesI64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    Some(QuantilesI64 {
        minimum: sorted[0],
        q25: sorted[quantile_index(sorted.len(), 1, 4)],
        median: sorted[quantile_index(sorted.len(), 1, 2)],
        q75: sorted[quantile_index(sorted.len(), 3, 4)],
        maximum: *sorted.last().unwrap_or(&sorted[0]),
    })
}

fn attack_windows(score: &PianisticExecutionScore) -> Result<Vec<AttackWindow>, String> {
    let topology = build_demand_topology(score)?;
    let boundaries = build_boundaries(&topology.obligations)?;
    let attacks = boundaries
        .iter()
        .filter(|boundary| !boundary.attacking.is_empty())
        .collect::<Vec<_>>();
    let mut windows = Vec::with_capacity(attacks.len().saturating_sub(1));
    for pair in attacks.windows(2) {
        let from = pair[0];
        let to = pair[1];
        let delta_ticks = to
            .tick
            .checked_sub(from.tick)
            .ok_or_else(|| "attack-boundary-tick-order-underflow".to_string())?;
        let delta_time_us = to
            .time_us_floor
            .checked_sub(from.time_us_floor)
            .ok_or_else(|| "attack-boundary-time-order-underflow".to_string())?;
        if delta_ticks <= 0 || delta_time_us == 0 {
            return Err("non-positive-distinct-attack-boundary-window".to_string());
        }
        windows.push(AttackWindow {
            from_tick: from.tick,
            to_tick: to.tick,
            from_time_us_floor: from.time_us_floor,
            to_time_us_floor: to.time_us_floor,
            delta_ticks,
            delta_time_us,
            to_attack_count: to.attacking.len(),
            to_active_obligation_count: to.active.len(),
        });
    }
    Ok(windows)
}

fn overlap_pair(
    first: &PhysicalKeyObligation,
    second: &PhysicalKeyObligation,
) -> Result<Option<SameKeyOverlapPair>, String> {
    if first.midi != second.midi {
        return Ok(None);
    }
    if second.attack_tick < first.attack_tick {
        return Err("same-key-overlap-input-not-attack-sorted".to_string());
    }
    if second.attack_tick >= first.release_tick {
        return Ok(None);
    }
    if second.attack_time_us_floor < first.attack_time_us_floor
        || second.attack_time_us_floor >= first.release_time_us_floor
    {
        return Err("same-key-overlap-physical-time-inconsistent-with-ticks".to_string());
    }
    let inter_attack_ticks = second
        .attack_tick
        .checked_sub(first.attack_tick)
        .ok_or_else(|| "same-key-inter-attack-tick-underflow".to_string())?;
    let inter_attack_time_us = second
        .attack_time_us_floor
        .checked_sub(first.attack_time_us_floor)
        .ok_or_else(|| "same-key-inter-attack-time-underflow".to_string())?;
    let remaining = first
        .release_time_us_floor
        .checked_sub(second.attack_time_us_floor)
        .ok_or_else(|| "same-key-remaining-hold-time-underflow".to_string())?;
    Ok(Some(SameKeyOverlapPair {
        midi: first.midi,
        first_obligation_id: first.id,
        second_obligation_id: second.id,
        first_attack_tick: first.attack_tick,
        second_attack_tick: second.attack_tick,
        first_release_tick: first.release_tick,
        second_release_tick: second.release_tick,
        inter_attack_ticks,
        inter_attack_time_us,
        remaining_first_hold_time_us_at_second_attack: remaining,
        simultaneous_attack: first.attack_tick == second.attack_tick,
        second_release_nested_in_first: second.release_tick <= first.release_tick,
    }))
}

fn same_key_overlap_pairs(
    obligations: &[PhysicalKeyObligation],
) -> Result<Vec<SameKeyOverlapPair>, String> {
    let mut by_midi = BTreeMap::<i32, Vec<&PhysicalKeyObligation>>::new();
    for obligation in obligations {
        by_midi.entry(obligation.midi).or_default().push(obligation);
    }
    let mut overlaps = Vec::new();
    for entries in by_midi.values_mut() {
        entries.sort_by_key(|item| (item.attack_tick, item.release_tick, item.id));
        for first_index in 0..entries.len() {
            let first = entries[first_index];
            for second in entries.iter().skip(first_index + 1).copied() {
                if second.attack_tick >= first.release_tick {
                    break;
                }
                if let Some(overlap) = overlap_pair(first, second)? {
                    overlaps.push(overlap);
                }
            }
        }
    }
    overlaps.sort_by_key(|item| {
        (
            item.inter_attack_time_us,
            std::cmp::Reverse(item.remaining_first_hold_time_us_at_second_attack),
            item.midi,
            item.first_obligation_id,
            item.second_obligation_id,
        )
    });
    Ok(overlaps)
}

fn collect_audit(score: &PianisticExecutionScore) -> TemporalDemandAudit {
    if score.status != ProjectionStatus::Complete {
        return TemporalDemandAudit {
            projection_status: "MODEL_INPUT_UNAVAILABLE",
            reason: score.reason.clone(),
            source_execution_events: score.execution_events.len(),
            physical_key_obligations: 0,
            attack_boundary_count: 0,
            positive_attack_window_count: 0,
            attack_window_time_quantiles_us: None,
            attack_window_tick_quantiles: None,
            shortest_attack_windows: Vec::new(),
            same_key_overlap_pairs: 0,
            simultaneous_same_key_attack_pairs: 0,
            staggered_same_key_reattack_pairs: 0,
            nested_release_overlap_pairs: 0,
            extending_release_overlap_pairs: 0,
            minimum_positive_same_key_reattack_time_us: None,
            maximum_remaining_prior_hold_time_us_at_reattack: None,
            overlap_pairs_by_midi: BTreeMap::new(),
            sampled_same_key_overlaps: Vec::new(),
        };
    }
    let topology = match build_demand_topology(score) {
        Ok(topology) => topology,
        Err(reason) => {
            return TemporalDemandAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: score.execution_events.len(),
                physical_key_obligations: 0,
                attack_boundary_count: 0,
                positive_attack_window_count: 0,
                attack_window_time_quantiles_us: None,
                attack_window_tick_quantiles: None,
                shortest_attack_windows: Vec::new(),
                same_key_overlap_pairs: 0,
                simultaneous_same_key_attack_pairs: 0,
                staggered_same_key_reattack_pairs: 0,
                nested_release_overlap_pairs: 0,
                extending_release_overlap_pairs: 0,
                minimum_positive_same_key_reattack_time_us: None,
                maximum_remaining_prior_hold_time_us_at_reattack: None,
                overlap_pairs_by_midi: BTreeMap::new(),
                sampled_same_key_overlaps: Vec::new(),
            };
        }
    };
    let windows = match attack_windows(score) {
        Ok(windows) => windows,
        Err(reason) => {
            return TemporalDemandAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: topology.source_execution_events,
                physical_key_obligations: topology.obligations.len(),
                attack_boundary_count: topology.attack_boundary_count,
                positive_attack_window_count: 0,
                attack_window_time_quantiles_us: None,
                attack_window_tick_quantiles: None,
                shortest_attack_windows: Vec::new(),
                same_key_overlap_pairs: 0,
                simultaneous_same_key_attack_pairs: 0,
                staggered_same_key_reattack_pairs: 0,
                nested_release_overlap_pairs: 0,
                extending_release_overlap_pairs: 0,
                minimum_positive_same_key_reattack_time_us: None,
                maximum_remaining_prior_hold_time_us_at_reattack: None,
                overlap_pairs_by_midi: BTreeMap::new(),
                sampled_same_key_overlaps: Vec::new(),
            };
        }
    };
    let overlaps = match same_key_overlap_pairs(&topology.obligations) {
        Ok(overlaps) => overlaps,
        Err(reason) => {
            return TemporalDemandAudit {
                projection_status: "MODEL_INPUT_UNAVAILABLE",
                reason,
                source_execution_events: topology.source_execution_events,
                physical_key_obligations: topology.obligations.len(),
                attack_boundary_count: topology.attack_boundary_count,
                positive_attack_window_count: windows.len(),
                attack_window_time_quantiles_us: None,
                attack_window_tick_quantiles: None,
                shortest_attack_windows: Vec::new(),
                same_key_overlap_pairs: 0,
                simultaneous_same_key_attack_pairs: 0,
                staggered_same_key_reattack_pairs: 0,
                nested_release_overlap_pairs: 0,
                extending_release_overlap_pairs: 0,
                minimum_positive_same_key_reattack_time_us: None,
                maximum_remaining_prior_hold_time_us_at_reattack: None,
                overlap_pairs_by_midi: BTreeMap::new(),
                sampled_same_key_overlaps: Vec::new(),
            };
        }
    };

    let window_times = windows.iter().map(|item| item.delta_time_us).collect::<Vec<_>>();
    let window_ticks = windows.iter().map(|item| item.delta_ticks).collect::<Vec<_>>();
    let mut shortest_attack_windows = windows.clone();
    shortest_attack_windows.sort_by_key(|item| {
        (
            item.delta_time_us,
            item.delta_ticks,
            item.from_tick,
            item.to_tick,
        )
    });
    shortest_attack_windows.truncate(8);

    let simultaneous_same_key_attack_pairs = overlaps
        .iter()
        .filter(|item| item.simultaneous_attack)
        .count();
    let staggered_same_key_reattack_pairs = overlaps.len().saturating_sub(simultaneous_same_key_attack_pairs);
    let nested_release_overlap_pairs = overlaps
        .iter()
        .filter(|item| item.second_release_nested_in_first)
        .count();
    let extending_release_overlap_pairs = overlaps.len().saturating_sub(nested_release_overlap_pairs);
    let minimum_positive_same_key_reattack_time_us = overlaps
        .iter()
        .filter_map(|item| (item.inter_attack_time_us > 0).then_some(item.inter_attack_time_us))
        .min();
    let maximum_remaining_prior_hold_time_us_at_reattack = overlaps
        .iter()
        .map(|item| item.remaining_first_hold_time_us_at_second_attack)
        .max();
    let mut overlap_pairs_by_midi = BTreeMap::<i32, usize>::new();
    for overlap in &overlaps {
        *overlap_pairs_by_midi.entry(overlap.midi).or_default() += 1;
    }
    let sampled_same_key_overlaps = overlaps.iter().take(12).cloned().collect::<Vec<_>>();

    TemporalDemandAudit {
        projection_status: "COMPLETE",
        reason: "exact-score-temporal-demand-frontier-complete".to_string(),
        source_execution_events: topology.source_execution_events,
        physical_key_obligations: topology.obligations.len(),
        attack_boundary_count: topology.attack_boundary_count,
        positive_attack_window_count: windows.len(),
        attack_window_time_quantiles_us: quantiles_u64(&window_times),
        attack_window_tick_quantiles: quantiles_i64(&window_ticks),
        shortest_attack_windows,
        same_key_overlap_pairs: overlaps.len(),
        simultaneous_same_key_attack_pairs,
        staggered_same_key_reattack_pairs,
        nested_release_overlap_pairs,
        extending_release_overlap_pairs,
        minimum_positive_same_key_reattack_time_us,
        maximum_remaining_prior_hold_time_us_at_reattack,
        overlap_pairs_by_midi,
        sampled_same_key_overlaps,
    }
}

fn format_midi_histogram(histogram: &BTreeMap<i32, usize>) -> String {
    if histogram.is_empty() {
        return "none".to_string();
    }
    histogram
        .iter()
        .map(|(midi, count)| format!("{midi}:{count}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn synthetic_score(events: Vec<PianisticExecutionEvent>) -> PianisticExecutionScore {
    PianisticExecutionScore {
        status: ProjectionStatus::Complete,
        reason: "synthetic-temporal-frontier".to_string(),
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
        synthetic_event(62, 96, 192, 500_000, 1_000_000),
    ]);
    let sequential_audit = collect_audit(&sequential);
    if sequential_audit.projection_status != "COMPLETE"
        || sequential_audit.attack_boundary_count != 2
        || sequential_audit.positive_attack_window_count != 1
        || sequential_audit
            .attack_window_time_quantiles_us
            .as_ref()
            .map(|item| item.minimum)
            != Some(500_000)
        || sequential_audit.same_key_overlap_pairs != 0
    {
        return Err("synthetic-sequential-temporal-frontier-mismatch".to_string());
    }

    let staggered_overlap = synthetic_score(vec![
        synthetic_event(60, 0, 192, 0, 1_000_000),
        synthetic_event(60, 96, 288, 500_000, 1_500_000),
    ]);
    let staggered_audit = collect_audit(&staggered_overlap);
    if staggered_audit.same_key_overlap_pairs != 1
        || staggered_audit.staggered_same_key_reattack_pairs != 1
        || staggered_audit.simultaneous_same_key_attack_pairs != 0
        || staggered_audit.minimum_positive_same_key_reattack_time_us != Some(500_000)
        || staggered_audit.maximum_remaining_prior_hold_time_us_at_reattack != Some(500_000)
    {
        return Err("synthetic-staggered-same-key-overlap-mismatch".to_string());
    }

    let simultaneous_overlap = synthetic_score(vec![
        synthetic_event(60, 0, 96, 0, 500_000),
        synthetic_event(60, 0, 192, 0, 1_000_000),
    ]);
    let simultaneous_audit = collect_audit(&simultaneous_overlap);
    if simultaneous_audit.same_key_overlap_pairs != 1
        || simultaneous_audit.simultaneous_same_key_attack_pairs != 1
        || simultaneous_audit.staggered_same_key_reattack_pairs != 0
    {
        return Err("synthetic-simultaneous-same-key-overlap-mismatch".to_string());
    }

    let exact_duplicate = synthetic_score(vec![
        synthetic_event(60, 0, 96, 0, 500_000),
        synthetic_event(60, 0, 96, 0, 500_000),
    ]);
    let duplicate_audit = collect_audit(&exact_duplicate);
    if duplicate_audit.physical_key_obligations != 1 || duplicate_audit.same_key_overlap_pairs != 0 {
        return Err("synthetic-exact-duplicate-coalescing-mismatch".to_string());
    }

    Ok("PASS; TempoMapPhysicalTime yes; AttackWindowsExact yes; ExactDuplicateCoalescingPreserved yes; SameKeyTemporalOverlapSeparated yes; HumanTransitionThreshold none; HandAssignment none; FingerAssignment none; PlayabilityClaim NOT_EVALUATED".to_string())
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
    out.push_str("TemporalFrontierContract: exact score-side physical time comes only from the r292 TempoMap projection. Distinct attack-boundary windows and overlapping same-physical-key obligations are measured; no human movement speed, hand span, finger speed, transition minimum or tempo-feasibility threshold is invented.\n");
    out.push_str("SameKeyBoundary: an overlapping later obligation on the same MIDI key is a temporal/technique demand, not an automatic impossibility. Exact duplicate attack/release events remain coalesced by the existing demand topology; non-identical overlaps stay visible here.\n");
    out.push_str("InterpretationBoundary: this audit measures when the score asks for state changes. It does not say which hand or finger performs them and does not state whether a pianist can satisfy them. It cannot authorize score mutation or repair.\n");

    let integrity = verify_runtime_integrity().unwrap_or_else(|reason| {
        panic!("PianisticTemporalTransitionDemandFrontierAudit runtime integrity failed: {reason}")
    });
    let _ = writeln!(out, "TemporalFrontierRuntimeIntegrity: {integrity}.");

    let Some(title) = score_title.map(str::trim).filter(|value| !value.is_empty()) else {
        out.push_str("InputProjectionStatus: MODEL_INPUT_UNAVAILABLE; TemporalFrontierStatus NOT_EVALUATED; HumanPlayabilityClaim NOT_EVALUATED.\n\n");
        return out;
    };
    let score = project_file(&PathBuf::from(format!("{title}.musicxml")));
    let audit = collect_audit(&score);
    let _ = writeln!(
        out,
        "InputProjectionStatus: {}; Reason {}; SourceExecutionEvents {}; PhysicalKeyObligations {}; AttackBoundaryCount {}; PositiveAttackWindowCount {}.",
        audit.projection_status,
        audit.reason,
        audit.source_execution_events,
        audit.physical_key_obligations,
        audit.attack_boundary_count,
        audit.positive_attack_window_count,
    );
    match &audit.attack_window_time_quantiles_us {
        Some(time) => {
            let ticks = audit.attack_window_tick_quantiles.as_ref().expect("paired tick quantiles");
            let _ = writeln!(
                out,
                "AttackBoundaryWindowFrontier: PhysicalTimeUs min {} q25 {} median {} q75 {} max {}; CanonicalTicks min {} q25 {} median {} q75 {} max {}; HumanThreshold none.",
                time.minimum,
                time.q25,
                time.median,
                time.q75,
                time.maximum,
                ticks.minimum,
                ticks.q25,
                ticks.median,
                ticks.q75,
                ticks.maximum,
            );
        }
        None => out.push_str("AttackBoundaryWindowFrontier: none.\n"),
    }
    let _ = writeln!(
        out,
        "SameKeyTemporalDemand: OverlapPairs {}; SimultaneousAttackPairs {}; StaggeredReattackDuringHoldPairs {}; NestedReleasePairs {}; ExtendingReleasePairs {}; MinimumPositiveSameKeyReattackTimeUs {}; MaximumRemainingPriorHoldTimeUsAtReattack {}; MidiPairHistogram [{}].",
        audit.same_key_overlap_pairs,
        audit.simultaneous_same_key_attack_pairs,
        audit.staggered_same_key_reattack_pairs,
        audit.nested_release_overlap_pairs,
        audit.extending_release_overlap_pairs,
        audit.minimum_positive_same_key_reattack_time_us
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        audit.maximum_remaining_prior_hold_time_us_at_reattack
            .map(|value| value.to_string())
            .unwrap_or_else(|| "none".to_string()),
        format_midi_histogram(&audit.overlap_pairs_by_midi),
    );
    if audit.shortest_attack_windows.is_empty() {
        out.push_str("ShortestAttackWindow: none.\n");
    } else {
        for window in &audit.shortest_attack_windows {
            let _ = writeln!(
                out,
                "ShortestAttackWindow: FromTick {}; ToTick {}; DeltaTicks {}; FromTimeUs {}; ToTimeUs {}; DeltaTimeUs {}; ToAttackCount {}; ToActiveObligationCount {}.",
                window.from_tick,
                window.to_tick,
                window.delta_ticks,
                window.from_time_us_floor,
                window.to_time_us_floor,
                window.delta_time_us,
                window.to_attack_count,
                window.to_active_obligation_count,
            );
        }
    }
    if audit.sampled_same_key_overlaps.is_empty() {
        out.push_str("SameKeyOverlapSample: none.\n");
    } else {
        for overlap in &audit.sampled_same_key_overlaps {
            let _ = writeln!(
                out,
                "SameKeyOverlapSample: Midi {}; FirstObligation {}; SecondObligation {}; FirstAttackTick {}; SecondAttackTick {}; FirstReleaseTick {}; SecondReleaseTick {}; InterAttackTicks {}; InterAttackTimeUs {}; RemainingPriorHoldTimeUs {}; SimultaneousAttack {}; SecondReleaseNestedInFirst {}.",
                overlap.midi,
                overlap.first_obligation_id,
                overlap.second_obligation_id,
                overlap.first_attack_tick,
                overlap.second_attack_tick,
                overlap.first_release_tick,
                overlap.second_release_tick,
                overlap.inter_attack_ticks,
                overlap.inter_attack_time_us,
                overlap.remaining_first_hold_time_us_at_second_attack,
                if overlap.simultaneous_attack { "yes" } else { "no" },
                if overlap.second_release_nested_in_first { "yes" } else { "no" },
            );
        }
    }
    out.push_str("CapabilityHandoff: r608 closes score-side transition timing demand only. A later capability model must bind independent evidence to explicit hand/finger transition signatures before any tempo-feasibility or human-playability claim.\n");
    let _ = writeln!(
        out,
        "HumanPlayabilityClaim {PLAYABILITY_CLAIM}; TransitionCapability NOT_BOUND; TempoFeasibility NOT_EVALUATED; CapabilityCalibration REQUIRED_BEFORE_HUMAN_CLAIM.\n"
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_attack_boundaries_preserve_exact_physical_window() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 500_000),
            synthetic_event(62, 96, 192, 500_000, 1_000_000),
        ]);
        let audit = collect_audit(&score);
        assert_eq!(audit.attack_boundary_count, 2);
        assert_eq!(audit.positive_attack_window_count, 1);
        assert_eq!(
            audit.attack_window_time_quantiles_us
                .as_ref()
                .map(|item| item.minimum),
            Some(500_000)
        );
    }

    #[test]
    fn staggered_same_key_reattack_is_not_hidden_as_span() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 192, 0, 1_000_000),
            synthetic_event(60, 96, 288, 500_000, 1_500_000),
        ]);
        let audit = collect_audit(&score);
        assert_eq!(audit.same_key_overlap_pairs, 1);
        assert_eq!(audit.staggered_same_key_reattack_pairs, 1);
        assert_eq!(audit.minimum_positive_same_key_reattack_time_us, Some(500_000));
        assert_eq!(
            audit.maximum_remaining_prior_hold_time_us_at_reattack,
            Some(500_000)
        );
    }

    #[test]
    fn simultaneous_same_key_nonduplicate_is_distinct_temporal_class() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 500_000),
            synthetic_event(60, 0, 192, 0, 1_000_000),
        ]);
        let audit = collect_audit(&score);
        assert_eq!(audit.same_key_overlap_pairs, 1);
        assert_eq!(audit.simultaneous_same_key_attack_pairs, 1);
        assert_eq!(audit.staggered_same_key_reattack_pairs, 0);
    }

    #[test]
    fn exact_duplicate_events_remain_coalesced_by_existing_topology() {
        let score = synthetic_score(vec![
            synthetic_event(60, 0, 96, 0, 500_000),
            synthetic_event(60, 0, 96, 0, 500_000),
        ]);
        let audit = collect_audit(&score);
        assert_eq!(audit.physical_key_obligations, 1);
        assert_eq!(audit.same_key_overlap_pairs, 0);
    }

    #[test]
    fn quartile_projection_is_deterministic_and_threshold_free() {
        let values = [100_u64, 200, 300, 400, 500];
        let q = quantiles_u64(&values).expect("quantiles");
        assert_eq!(q.minimum, 100);
        assert_eq!(q.q25, 200);
        assert_eq!(q.median, 300);
        assert_eq!(q.q75, 400);
        assert_eq!(q.maximum, 500);
    }

    #[test]
    fn runtime_integrity_exercises_temporal_domain() {
        assert!(verify_runtime_integrity().expect("integrity").starts_with("PASS;"));
    }

    #[test]
    fn report_preserves_human_claim_boundary() {
        let text = build_for_score_title(None);
        assert!(text.contains("HumanPlayabilityClaim NOT_EVALUATED"));
        assert!(text.contains("TransitionCapability NOT_BOUND") || text.contains("TemporalFrontierStatus NOT_EVALUATED"));
        assert!(!text.contains("REALIZATION_WITNESS_FOUND"));
    }
}
