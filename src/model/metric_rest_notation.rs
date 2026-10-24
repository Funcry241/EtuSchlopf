// Writer-free metric policy for visible partial-rest duration notation.
//
// The planner deliberately owns only the decomposition of an already proven
// silent span. Voice existence, staff/voice assignment, full-measure-rest
// canonicalization and physical MusicXML writes remain owned elsewhere.

const NOTATION_UNITS: [i64; 13] = [384, 288, 192, 144, 96, 72, 48, 36, 24, 18, 12, 9, 6];


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RestSpanConsolidationProjection {
    SinglePartShortBeatLocal,
    MetricCompaction,
    // Caller has proven that hidden chord hosts remain on their original timeline;
    // only the pianist-visible rest projection may be compacted.
    CarrierPreservingMetricCompaction,
    SinglePartBeatLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RestSpanConsolidationFacts {
    pub(crate) start: i64,
    pub(crate) duration: i64,
    pub(crate) expected_measure_duration: i64,
    pub(crate) beat_duration: i64,
    pub(crate) source_fragment_count: usize,
    pub(crate) has_chord_anchor_dependency: bool,
    pub(crate) all_source_fragments_short: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RestSpanConsolidationDecision {
    pub(crate) parts: Vec<i64>,
}

pub(crate) fn decide_rest_span_consolidation(
    facts: RestSpanConsolidationFacts,
    projection: RestSpanConsolidationProjection,
) -> Option<RestSpanConsolidationDecision> {
    if facts.source_fragment_count < 2 {
        return None;
    }
    let carrier_preserving_projection = matches!(
        projection,
        RestSpanConsolidationProjection::CarrierPreservingMetricCompaction
    );
    if facts.has_chord_anchor_dependency != carrier_preserving_projection {
        return None;
    }

    let parts = plan_partial_visible_rest_durations(
        facts.start,
        facts.duration,
        facts.expected_measure_duration,
        facts.beat_duration,
    );
    if parts.is_empty() || parts.len() >= facts.source_fragment_count {
        return None;
    }

    let end_exclusive = facts.start.checked_add(facts.duration)?;
    let single_beat = end_exclusive > facts.start
        && facts.start.div_euclid(facts.beat_duration)
            == end_exclusive
                .saturating_sub(1)
                .div_euclid(facts.beat_duration);

    let projection_allowed = match projection {
        RestSpanConsolidationProjection::SinglePartShortBeatLocal => {
            facts.all_source_fragments_short && parts.len() == 1 && single_beat
        }
        RestSpanConsolidationProjection::MetricCompaction
        | RestSpanConsolidationProjection::CarrierPreservingMetricCompaction => true,
        RestSpanConsolidationProjection::SinglePartBeatLocal => {
            parts.len() == 1 && single_beat
        }
    };
    projection_allowed.then_some(RestSpanConsolidationDecision { parts })
}

pub(crate) fn plan_partial_visible_rest_durations(
    start: i64,
    duration: i64,
    expected_measure_duration: i64,
    beat_duration: i64,
) -> Vec<i64> {
    if start < 0 || duration <= 0 || expected_measure_duration <= 0 || beat_duration <= 0 {
        return Vec::new();
    }
    // EtuSchlopf normalisiert seine produktive MusicXML-Zeitbasis auf
    // divisions-per-quarter=96. Die vorhandenen Rest-Materialisierer bilden
    // genau diese Dauerwerte auf MusicXML-Typen ab; bei einer anderen Basis
    // entscheidet dieser Planner deshalb bewusst nicht.
    if beat_duration != 96 {
        return Vec::new();
    }
    let end = match start.checked_add(duration) {
        Some(end) if end <= expected_measure_duration => end,
        _ => return Vec::new(),
    };

    // A complete silent measure has a separate canonical owner. Keep its
    // beat-explicit source representation here so this planner cannot become a
    // second full-measure-rest authority.
    let full_measure_span = start == 0 && end == expected_measure_duration;
    let four_beat_simple_grid = expected_measure_duration == beat_duration.saturating_mul(4);
    let paired_beat = beat_duration.saturating_mul(2);

    let mut cursor = start;
    let mut parts = Vec::new();
    while cursor < end {
        if four_beat_simple_grid
            && !full_measure_span
            && paired_beat > 0
            && cursor.rem_euclid(paired_beat) == 0
            && end.saturating_sub(cursor) >= paired_beat
        {
            parts.push(paired_beat);
            cursor = cursor.saturating_add(paired_beat);
            continue;
        }

        let offset = cursor.rem_euclid(beat_duration);
        let to_next_beat = if offset == 0 {
            beat_duration
        } else {
            beat_duration.saturating_sub(offset)
        };
        let take = end.saturating_sub(cursor).min(to_next_beat);
        let local = split_duration_for_visible_notation(take);
        if local.is_empty() {
            return Vec::new();
        }
        parts.extend(local);
        cursor = cursor.saturating_add(take);
        if parts.len() > 64 {
            return Vec::new();
        }
    }
    parts
}

fn split_duration_for_visible_notation(mut duration: i64) -> Vec<i64> {
    let mut parts = Vec::new();
    while duration > 0 {
        let Some(unit) = NOTATION_UNITS.iter().copied().find(|unit| *unit <= duration) else {
            return Vec::new();
        };
        parts.push(unit);
        duration = duration.saturating_sub(unit);
        if parts.len() > 32 {
            return Vec::new();
        }
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::{
        decide_rest_span_consolidation,
        plan_partial_visible_rest_durations,
        RestSpanConsolidationFacts,
        RestSpanConsolidationProjection,
    };

    #[test]
    fn four_beat_partial_span_uses_aligned_half_measure_then_exposes_later_beats() {
        assert_eq!(
            plan_partial_visible_rest_durations(0, 336, 384, 96),
            vec![192, 96, 48]
        );
    }

    #[test]
    fn three_quiet_beats_expose_the_third_beat_instead_of_using_a_dotted_half() {
        assert_eq!(
            plan_partial_visible_rest_durations(0, 288, 384, 96),
            vec![192, 96]
        );
    }

    #[test]
    fn second_half_of_four_beat_measure_can_be_one_half_rest() {
        assert_eq!(
            plan_partial_visible_rest_durations(192, 192, 384, 96),
            vec![192]
        );
    }

    #[test]
    fn four_beat_span_does_not_cross_the_half_measure_from_a_weak_beat() {
        assert_eq!(
            plan_partial_visible_rest_durations(96, 192, 384, 96),
            vec![96, 96]
        );
    }

    #[test]
    fn complete_measure_is_left_to_the_existing_measure_rest_owner() {
        assert_eq!(
            plan_partial_visible_rest_durations(0, 384, 384, 96),
            vec![96, 96, 96, 96]
        );
    }

    #[test]
    fn non_four_beat_meter_falls_back_to_beat_explicit_notation() {
        assert_eq!(
            plan_partial_visible_rest_durations(0, 240, 288, 96),
            vec![96, 96, 48]
        );
    }

    #[test]
    fn invalid_or_out_of_measure_span_fails_closed() {
        assert!(plan_partial_visible_rest_durations(-1, 96, 384, 96).is_empty());
        assert!(plan_partial_visible_rest_durations(336, 96, 384, 96).is_empty());
        assert!(plan_partial_visible_rest_durations(0, 168, 192, 48).is_empty());
    }

    #[test]
    fn shared_rest_span_owner_accepts_safe_two_fragment_single_part_projection() {
        let decision = decide_rest_span_consolidation(
            RestSpanConsolidationFacts {
                start: 0,
                duration: 48,
                expected_measure_duration: 384,
                beat_duration: 96,
                source_fragment_count: 2,
                has_chord_anchor_dependency: false,
                all_source_fragments_short: true,
            },
            RestSpanConsolidationProjection::SinglePartShortBeatLocal,
        )
        .expect("safe 24+24 span should consolidate");
        assert_eq!(decision.parts, vec![48]);
    }

    #[test]
    fn shared_rest_span_owner_rejects_chord_anchor_for_non_preserving_projections() {
        for projection in [
            RestSpanConsolidationProjection::SinglePartShortBeatLocal,
            RestSpanConsolidationProjection::MetricCompaction,
            RestSpanConsolidationProjection::SinglePartBeatLocal,
        ] {
            assert!(
                decide_rest_span_consolidation(
                    RestSpanConsolidationFacts {
                        start: 0,
                        duration: 48,
                        expected_measure_duration: 384,
                        beat_duration: 96,
                        source_fragment_count: 2,
                        has_chord_anchor_dependency: true,
                        all_source_fragments_short: true,
                    },
                    projection,
                )
                .is_none()
            );
        }
    }

    #[test]
    fn shared_rest_span_owner_accepts_explicit_carrier_preserving_metric_projection() {
        let decision = decide_rest_span_consolidation(
            RestSpanConsolidationFacts {
                start: 0,
                duration: 216,
                expected_measure_duration: 384,
                beat_duration: 96,
                source_fragment_count: 8,
                has_chord_anchor_dependency: true,
                all_source_fragments_short: true,
            },
            RestSpanConsolidationProjection::CarrierPreservingMetricCompaction,
        )
        .expect("a carrier-preserving projection may compact visible rest notation");
        assert_eq!(decision.parts, vec![192, 24]);
    }

    #[test]
    fn carrier_preserving_projection_requires_a_real_chord_anchor_dependency() {
        assert!(
            decide_rest_span_consolidation(
                RestSpanConsolidationFacts {
                    start: 0,
                    duration: 216,
                    expected_measure_duration: 384,
                    beat_duration: 96,
                    source_fragment_count: 8,
                    has_chord_anchor_dependency: false,
                    all_source_fragments_short: true,
                },
                RestSpanConsolidationProjection::CarrierPreservingMetricCompaction,
            )
            .is_none()
        );
    }

    #[test]
    fn shared_rest_span_owner_preserves_projection_specific_scope() {
        assert!(
            decide_rest_span_consolidation(
                RestSpanConsolidationFacts {
                    start: 0,
                    duration: 192,
                    expected_measure_duration: 384,
                    beat_duration: 96,
                    source_fragment_count: 4,
                    has_chord_anchor_dependency: false,
                    all_source_fragments_short: true,
                },
                RestSpanConsolidationProjection::SinglePartShortBeatLocal,
            )
            .is_none()
        );
        assert_eq!(
            decide_rest_span_consolidation(
                RestSpanConsolidationFacts {
                    start: 0,
                    duration: 192,
                    expected_measure_duration: 384,
                    beat_duration: 96,
                    source_fragment_count: 4,
                    has_chord_anchor_dependency: false,
                    all_source_fragments_short: true,
                },
                RestSpanConsolidationProjection::MetricCompaction,
            )
            .expect("metric compaction may span the aligned half measure")
            .parts,
            vec![192]
        );
    }
}
