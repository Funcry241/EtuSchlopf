# Phrase Boundary Metric + Melodic Evidence Audit Compile Recovery r131

## Estimated plan position

Take 4/9 (estimated) on the optimized phrasing path toward conservative visible phrase slurs.
The denominator is a planning estimate, not a revision contract, and will be reassessed at the
planned corpus hard stops.

## Parent

Accepted DARK-GREEN parent: EtuSchlopf v0.10.1839 r129
`HistoricalClippyNonSecurityIdiomClosure`.

r129 is musically identical to the accepted phrase foundation through r128/r127. The full 10-etude
corpus after r129 confirms:

- 6,978 true attacks;
- 6,949 boundary observation positions;
- 700/700 cross-measure rhythm relations resolved;
- 0 promoted phrase boundaries;
- 0 pause-generated positions.

The same corpus also shows that rhythm alone must not be promoted into PhraseStructure: only 481 of
6,949 positions have positive temporal-gap evidence, and 438 of those 481 are accompanied by an
explicit rest. The next useful information must therefore come from independent score-intrinsic
families rather than a more elaborate pause rule.

## Purpose

r131 extends the existing audit-only `BoundaryObservation` with exact metric location and local,
relative melodic context.

It does **not** classify phrase boundaries.

The implementation deliberately avoids absolute melodic leap thresholds, metric-strength labels,
weights and scalar scores. A downbeat, barline, contour reversal or local interval peak is an
observation, not a verdict.

## Metric evidence

The already-authoritative parsed `Score -> Part -> Measure` metric owner is reused. The existing
canonical integer time scale is extended with exact beat duration for each measure.

For the right attack of every existing boundary observation r131 records:

- canonical offset from the containing measure start;
- zero-based beat index;
- exact beat-phase ticks;
- whether the attack is exactly on the measure start;
- whether the attack is exactly on the MusicXML beat grid;
- the inherited `beats/beat-type` meter context.

No new meter parser, meter authority or phrase rule is introduced. `RightOnMeasureStart` and
`RightOnBeatGrid` are diagnostic counts only. In particular:

`barline != phrase support`

and

`downbeat != phrase support`.

## Melodic evidence

The existing true-attack pitch sets remain the only melodic source. Tie-stop-only pitches remain
excluded from attack descriptors.

For each boundary r131 adds local relative context when neighboring attacks exist:

- previous -> left centroid motion;
- left -> right boundary centroid motion;
- right -> following centroid motion;
- previous, boundary and following nearest-pitch intervals;
- exact contour relation into and out of the boundary (`continuation`, `reversal`, `stable`,
  `plateau_transition`);
- whether the boundary interval is a strict local peak, strict local valley, or mixed/equal relative
  to both neighbors;
- exact pitch overlap across the boundary.

These are structural descriptors only. There is no rule such as `jump >= N semitones -> boundary`,
no register threshold and no conversion to `WEAK/MEDIUM/STRONG`.

## Expected corpus projection

Static replay of the accepted 10 master MusicXML files preserves the complete r129 attack/boundary
projection and adds descriptors to all 6,949 positions:

- MetricDescriptorKnown: 6,949
- RightOnMeasureStart: 421
- RightOnBeatGrid: 1,784
- RightOffBeatGrid: 5,165
- MelodicContextBeforeKnown: 6,920
- MelodicContextAfterKnown: 6,920
- ContourContinuationInto: 2,452
- ContourReversalInto: 3,446
- ContourContinuationOut: 2,452
- ContourReversalOut: 3,446
- BoundaryIntervalLocalPeak: 1,917
- BoundaryIntervalLocalValley: 1,886
- PitchOverlapAcrossBoundary: 626

These counts are diagnostic expectations, not ground truth and not release thresholds.

For Etüde Nr. 6 the static replay expects:

- TrueAttacks: 871
- BoundaryObservations: 868
- MetricDescriptorKnown: 868
- RightOnMeasureStart: 49
- RightOnBeatGrid: 220
- RightOffBeatGrid: 648
- ContextBeforeKnown: 865
- ContextAfterKnown: 865
- ContourContinuationInto: 250
- ContourReversalInto: 427
- ContourContinuationOut: 250
- ContourReversalOut: 427
- BoundaryIntervalLocalPeak: 247
- BoundaryIntervalLocalValley: 237
- PitchOverlapAcrossBoundary: 109
- PromotedBoundaryCandidates: 0
- PauseGeneratedPositions: 0

## Invariants

r131 keeps the established hard contracts:

- `PhraseClassification = none`
- `PhraseSpan = none`
- `SlurIntent = none`
- `NotationPolicy = none`
- `AnchorResolver = none`
- `PhraseConfidence = none`
- scalarization = none
- learned/external weights = none
- no voice collapse or majority vote
- no Render MusicXML write
- no Master MusicXML mutation
- no sounding-event mutation
- no new writer capability

## Next step

If the real Windows gate is green, run the 10-etude corpus again. The next optimized planned musical
block is Parallelism + Structural Novelty, followed by the explicit hard stop where the corpus and
human-readable boundary examples decide whether Tonal Closure is needed at all.


## Compile-recovery note

The rejected r130 candidate accidentally used the local binding name `nearest_pitch_distance`,
shadowing the helper function of the same name and causing Rust E0618 at the two neighboring-context
function calls. r131 changes only that local binding to `boundary_pitch_distance` and wires it back
into the unchanged `BoundaryObservation.nearest_pitch_distance` field. No metric, melodic, rhythmic,
phrase, writer, mutation, security, threshold, weighting, or scalarization behavior is changed.
