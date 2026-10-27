//! Exact source time units and the chronological MIDI meter map.
//! No source event, musical selection, notation or writer is modified here.

use anyhow::{anyhow, Result};

const LEGACY_DIVISIONS: i64 = 96;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MeterChange {
    pub(crate) tick: u64,
    pub(crate) beats: u8,
    pub(crate) beat_type: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceTimeDomain {
    pub(crate) source_ppq: i64,
    pub(crate) divisions: i64,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { (a, b) = (b, a % b); }
    a
}

fn lcm(a: i64, b: i64) -> Result<i64> {
    a.checked_div(gcd(a, b)).and_then(|value| value.checked_mul(b))
        .ok_or_else(|| anyhow!("Die exakte Zeitaufloesung ist zu gross"))
}

impl SourceTimeDomain {
    /// Choose the smallest multiple of the historical unit that exactly
    /// represents every raw note boundary, meter change and denominator beat.
    /// Equivalent MIDI encodings at different PPQ therefore select the same unit.
    pub(crate) fn from_raw_boundaries(
        source_ppq: i64,
        boundaries: impl IntoIterator<Item = u64>,
        meters: &[MeterChange],
    ) -> Result<Self> {
        if !(1..=32767).contains(&source_ppq) {
            return Err(anyhow!("MIDI-PPQ liegt ausserhalb der SMF-PPQ-Domaene"));
        }
        let mut divisions = LEGACY_DIVISIONS;
        for tick in boundaries.into_iter().chain(meters.iter().map(|meter| meter.tick)) {
            let residue = i64::try_from(tick % source_ppq as u64)?;
            divisions = lcm(divisions, source_ppq / gcd(source_ppq, residue))?;
        }
        for meter in meters {
            if meter.beats == 0 || meter.beat_type == 0 || !meter.beat_type.is_power_of_two() {
                return Err(anyhow!("Die MIDI-Taktart hat keinen gueltigen Zaehler/Nenner"));
            }
            let denominator = i64::from(meter.beat_type);
            divisions = lcm(divisions, denominator / gcd(denominator, 4))?;
        }
        Ok(Self { source_ppq, divisions })
    }

    pub(crate) fn position(self, tick: u64) -> Result<i64> {
        if self.source_ppq <= 0 || self.divisions <= 0 {
            return Err(anyhow!("Ungueltige gebundene Zeitdomaene"));
        }
        let numerator = i128::from(tick) * i128::from(self.divisions);
        if numerator % i128::from(self.source_ppq) != 0 {
            return Err(anyhow!("Quellzeit gehoert nicht zur gebundenen exakten Zeitdomaene"));
        }
        i64::try_from(numerator / i128::from(self.source_ppq))
            .map_err(|_| anyhow!("Die exakte MIDI-Zeitposition ist zu gross"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MetricMeasure {
    pub(crate) start: i64,
    pub(crate) end: i64,
    pub(crate) beats: u8,
    pub(crate) beat_type: u16,
    pub(crate) truncated_by_change: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MetricSegment {
    first_ordinal: usize,
    start: i64,
    end: i64,
    full_duration: i64,
    count: usize,
    beats: u8,
    beat_type: u16,
    truncated_last: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MetricTimeline {
    pub(crate) divisions: i64,
    segments: Vec<MetricSegment>,
    count: usize,
}

impl MetricTimeline {
    /// SMF defaults to 4/4 until an actual chronological change overrides it.
    /// Contradictory simultaneous declarations are rejected, never ordered by track.
    /// Segments keep long empty spans compact; there is no per-bar allocation.
    pub(crate) fn from_midi(
        domain: SourceTimeDomain, changes: &[MeterChange], extent: i64,
    ) -> Result<Self> {
        if extent <= 0 { return Err(anyhow!("Die zeitliche Quelldomaene ist leer")); }
        let mut ordered = changes.to_vec();
        ordered.sort_by_key(|meter| (meter.tick, meter.beats, meter.beat_type));
        ordered.dedup();
        if ordered.windows(2).any(|pair| pair[0].tick == pair[1].tick) {
            return Err(anyhow!("Widerspruechliche Taktarten am selben MIDI-Zeitpunkt"));
        }
        if ordered.first().is_none_or(|meter| meter.tick != 0) {
            ordered.insert(0, MeterChange { tick: 0, beats: 4, beat_type: 4 });
        }
        ordered.dedup_by(|next, prior|
            next.beats == prior.beats && next.beat_type == prior.beat_type);
        let mut segments = Vec::new();
        let mut count = 0usize;
        for (index, meter) in ordered.iter().enumerate() {
            let start = domain.position(meter.tick)?;
            if start >= extent { break; }
            let numerator = domain.divisions.checked_mul(i64::from(meter.beats))
                .and_then(|value| value.checked_mul(4))
                .ok_or_else(|| anyhow!("Die Taktlaenge ist zu gross"))?;
            let denominator = i64::from(meter.beat_type);
            if denominator <= 0 || numerator <= 0 || numerator % denominator != 0 {
                return Err(anyhow!("Die Taktart gehoert nicht zur exakten Zeitdomaene"));
            }
            let duration = numerator / denominator;
            let next_change = ordered.get(index + 1).map(|next| domain.position(next.tick)).transpose()?;
            let segment_extent = next_change.filter(|next| *next < extent).unwrap_or(extent);
            let distance = segment_extent.checked_sub(start).filter(|value| *value > 0)
                .ok_or_else(|| anyhow!("Taktverlauf ist nicht streng chronologisch"))?;
            let bars = (distance - 1) / duration + 1;
            let truncated_last = next_change == Some(segment_extent) && distance % duration != 0;
            let end = if truncated_last { segment_extent } else {
                bars.checked_mul(duration).and_then(|value| start.checked_add(value))
                    .ok_or_else(|| anyhow!("Die Taktgrenze ist zu gross"))?
            };
            let bars = usize::try_from(bars)?;
            segments.push(MetricSegment { first_ordinal: count, start, end,
                full_duration: duration, count: bars, beats: meter.beats,
                beat_type: meter.beat_type, truncated_last });
            count = count.checked_add(bars).ok_or_else(|| anyhow!("Zu viele Takte"))?;
        }
        Ok(Self { divisions: domain.divisions, segments, count })
    }

    pub(crate) fn len(&self) -> usize { self.count }

    pub(crate) fn measure(&self, ordinal: usize) -> Option<MetricMeasure> {
        let segment = self.segments.iter().find(|segment|
            ordinal >= segment.first_ordinal && ordinal - segment.first_ordinal < segment.count)?;
        let local = ordinal - segment.first_ordinal;
        let start = segment.start.checked_add(i64::try_from(local).ok()?.checked_mul(segment.full_duration)?)?;
        let end = i64::try_from((i128::from(start) + i128::from(segment.full_duration))
            .min(i128::from(segment.end))).ok()?;
        Some(MetricMeasure { start, end, beats: segment.beats, beat_type: segment.beat_type,
            truncated_by_change: segment.truncated_last && local + 1 == segment.count })
    }

    pub(crate) fn ordinal_at(&self, position: i64) -> Option<usize> {
        let segment = self.segments.iter().find(|segment| position >= segment.start && position < segment.end)?;
        segment.first_ordinal.checked_add(usize::try_from((position - segment.start) / segment.full_duration).ok()?)
    }
}

#[cfg(test)]
#[path = "../tests/unit/source_time.rs"]
mod tests;
