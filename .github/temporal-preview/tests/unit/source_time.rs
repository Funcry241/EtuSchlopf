use super::*;

#[test]
fn source_units_are_lossless_and_invariant_under_equivalent_ppq_encoding() {
    for ppq in [48i64, 96, 384, 960, 1001] {
        let boundaries = (0..=17).map(|n| n * 7 + n % 3).collect::<Vec<u64>>();
        let domain = SourceTimeDomain::from_raw_boundaries(ppq, boundaries.clone(), &[]).unwrap();
        let equivalent = SourceTimeDomain::from_raw_boundaries(ppq * 2,
            boundaries.iter().map(|tick| tick * 2), &[]).unwrap();
        assert_eq!(domain.divisions, equivalent.divisions);
        for tick in boundaries {
            let pos = domain.position(tick).unwrap();
            assert_eq!(i128::from(pos) * i128::from(ppq), i128::from(tick) * i128::from(domain.divisions));
            assert_eq!(pos, equivalent.position(tick * 2).unwrap());
        }
    }
}

#[test]
fn historical_exact_events_keep_the_existing_unit() {
    let domain = SourceTimeDomain::from_raw_boundaries(480, (0..=32).map(|n| n * 20), &[]).unwrap();
    assert_eq!(domain.divisions, LEGACY_DIVISIONS);
    assert_eq!(domain.position(480).unwrap(), LEGACY_DIVISIONS);
}

#[test]
fn meter_changes_preserve_chronology_and_denominator_beats() {
    let ppq = 96;
    let changes = [MeterChange { tick: 12 * ppq, beats: 5, beat_type: 8 },
        MeterChange { tick: 0, beats: 3, beat_type: 4 },
        MeterChange { tick: 6 * ppq, beats: 6, beat_type: 8 }];
    let domain = SourceTimeDomain::from_raw_boundaries(ppq as i64, [], &changes).unwrap();
    let timeline = MetricTimeline::from_midi(domain, &changes, 15 * domain.divisions).unwrap();
    assert_eq!(timeline.len(), 6);
    for ordinal in 0..timeline.len() {
        let measure = timeline.measure(ordinal).unwrap();
        let (beats, beat_type) = if ordinal < 2 { (3, 4) } else if ordinal < 4 { (6, 8) } else { (5, 8) };
        assert_eq!((measure.beats, measure.beat_type), (beats, beat_type));
        assert_eq!(measure.end - measure.start, domain.divisions * i64::from(beats) * 4 / i64::from(beat_type));
        assert_eq!(timeline.ordinal_at(measure.start), Some(ordinal));
        assert_eq!(timeline.ordinal_at(measure.end - 1), Some(ordinal));
        assert!(!measure.truncated_by_change);
    }
}

#[test]
fn repeated_meter_is_not_an_invented_barline_and_conflicts_are_rejected() {
    let changes = [MeterChange { tick: 96, beats: 4, beat_type: 4 }];
    let domain = SourceTimeDomain::from_raw_boundaries(96, [], &changes).unwrap();
    let timeline = MetricTimeline::from_midi(domain, &changes, 384).unwrap();
    assert_eq!(timeline.len(), 1);
    assert!(!timeline.measure(0).unwrap().truncated_by_change);
    let conflict = [MeterChange { tick: 0, beats: 3, beat_type: 4 },
        MeterChange { tick: 0, beats: 6, beat_type: 8 }];
    assert!(MetricTimeline::from_midi(domain, &conflict, 384).is_err());
}

#[test]
fn mid_bar_change_has_an_exact_explicit_partial_span() {
    let changes = [MeterChange { tick: 96, beats: 3, beat_type: 4 }];
    let domain = SourceTimeDomain::from_raw_boundaries(96, [], &changes).unwrap();
    let timeline = MetricTimeline::from_midi(domain, &changes, 384).unwrap();
    let prior = timeline.measure(0).unwrap();
    assert_eq!((prior.start, prior.end, prior.truncated_by_change), (0, 96, true));
    let after = timeline.measure(1).unwrap();
    assert_eq!((after.start, after.end, after.beats, after.beat_type), (96, 384, 3, 4));
}

#[test]
fn extreme_time_ranges_do_not_allocate_one_object_per_measure() {
    let domain = SourceTimeDomain::from_raw_boundaries(96, [], &[]).unwrap();
    let extent = 384 * 1_000_000_000;
    let timeline = MetricTimeline::from_midi(domain, &[], extent).unwrap();
    assert_eq!(timeline.segments.len(), 1);
    assert_eq!(timeline.len(), 1_000_000_000);
    assert_eq!(timeline.ordinal_at(extent - 1), Some(timeline.len() - 1));
    assert_eq!(timeline.measure(timeline.len() - 1).unwrap().end, extent);
}

#[test]
fn invalid_and_unrepresentable_coordinates_are_rejected_without_rounding() {
    assert!(SourceTimeDomain::from_raw_boundaries(0, [], &[]).is_err());
    let domain = SourceTimeDomain::from_raw_boundaries(1, [], &[]).unwrap();
    assert!(domain.position(u64::MAX).is_err());
    let domain = SourceTimeDomain::from_raw_boundaries(384, [], &[]).unwrap();
    assert!(domain.position(1).is_err());
    assert!(SourceTimeDomain::from_raw_boundaries(96, [], &[MeterChange {
        tick: 0, beats: 0, beat_type: 4,
    }]).is_err());
}
