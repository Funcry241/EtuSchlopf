# Phrase Boundary Parallelism + Structural Novelty Evidence Audit r132

## Estimated plan position

Take 5/9 (estimated) on the optimized phrasing path toward conservative visible phrase slurs.
The denominator is a planning estimate, not a revision contract, and will be reassessed at the
planned hard stop immediately after this take.

## Parent

Accepted DARK-GREEN parent: EtuSchlopf v0.10.1839 r131
`PhraseBoundaryMetricMelodicEvidenceAuditCompileRecovery`.

The real 10-etude corpus after r131 confirms the technical phrase-evidence foundation and the
metric+melodic expansion:

- 10/10 projects released dark green;
- 6,978 true attacks;
- 6,949 boundary observation positions;
- 700/700 cross-measure rhythm relations resolved;
- metric descriptors on all 6,949 observations;
- melodic neighboring context on 6,920 observations;
- 0 promoted phrase boundaries;
- 0 pause-generated positions.

The corpus also shows the intended separation of roles: metric beat-grid location is broadly stable
and contextual, while melodic contour/overlap descriptors vary materially between etudes. No new
technical foundation gap is visible.

## Purpose

r132 adds two score-intrinsic evidence families to the existing audit-only
`BoundaryObservation`:

1. piece-internal exact Parallelism evidence;
2. Structural Novelty state-change evidence.

The take remains strictly pre-Structure. It does not classify boundaries, create phrase spans,
compute PhraseConfidence, aggregate evidence into a score, or authorize notation.

## Parallelism evidence

Parallelism is detected only from the already established true-attack timeline.

For every boundary with one true attack of context on both sides, r132 derives two exact local
signatures:

### Rhythmic ratio signature

The five positive quantities

- previous -> left attack IOI;
- left -> right boundary IOI;
- right -> following attack IOI;
- left sounding duration;
- right sounding duration

are divided by their exact integer greatest common divisor.

This preserves exact proportional rhythm under augmentation/diminution without introducing a
tolerance, floating-point distance, learned weight, or threshold.

### Melodic shape signature

The signature contains

- previous -> left pitch-centroid delta;
- left -> right pitch-centroid delta;
- right -> following pitch-centroid delta;
- chord-member cardinality for previous/left/right/following attacks.

Pitch-centroid deltas are transposition-invariant because only relative deltas are retained.

A boundary reports exact repeated joint signatures within the piece, including whether matches occur
in the same part/staff/voice lane or another lane. Exact repetition is a descriptor only. It is not
a vote for a phrase boundary.

There is deliberately no fuzzy similarity score in this take.

## Structural Novelty evidence

r132 reports independent exact state changes around every existing boundary:

- previous-vs-following IOI context change when both neighbors exist;
- left-vs-right chord-cardinality change;
- score-wide active-lane count immediately before vs. at the right attack;
- score-wide simultaneous onset-lane multiplicity at the left vs. right attack;
- score-wide true-attack density in one notated beat before vs. one beat after the boundary.

The beat window reuses the existing canonical beat duration. It introduces no new meter owner.

These axes remain independent. In particular there is no `NoveltyScore`, no weighted sum and no
rule that N changed axes imply a phrase boundary.

## Static 10-etude replay

A Python mirror was first validated against the accepted r131 reports and reproduces the established
per-etude attack, boundary, gap, beat-grid, contour, local-interval and pitch-overlap counts exactly.
Using the same projection for the new r132 descriptors predicts:

- BoundaryObservations: 6,949 unchanged
- ParallelismDescriptorKnown: 6,891
- ExactJointPatternRepeated: 1,333
- SameLaneExactRepeat: 1,179
- CrossLaneExactRepeat: 420
- per-project exact joint-pattern groups: 6,087 total
- largest exact group in any project: 12
- StructuralNoveltyDescriptorKnown: 6,949
- RhythmicContextKnown: 6,891
- RhythmicContextChange: 2,502
- ChordTextureChange: 292
- ScoreActiveLaneChange: 936
- ScoreOnsetMultiplicityChange: 2,168
- ScoreOneBeatAttackDensityChange: 4,718
- Evidence families observed: 6 on 6,891 positions, 5 on the 58 lane-edge positions

These values are diagnostics, not ground truth and not release thresholds. The high one-beat density
change rate is intentionally retained as a raw descriptor and must be judged at the next hard stop;
it is not promoted into evidence strength.

For Etüde Nr. 6 the static replay predicts:

- BoundaryObservations: 868
- ParallelismDescriptorKnown: 862
- ExactJointPatternRepeated: 158
- SameLaneExactRepeat: 144
- CrossLaneExactRepeat: 18
- UniqueJointPatterns: 778
- MaxExactGroupCardinality: 4
- StructuralNoveltyDescriptorKnown: 868
- RhythmicContextKnown: 862
- RhythmicContextChange: 320
- ChordTextureChange: 31
- ScoreActiveLaneChange: 137
- ScoreOnsetMultiplicityChange: 310
- ScoreOneBeatAttackDensityChange: 633
- Evidence families observed: 6 on 862 positions, 5 on 6 positions

## Invariants

r132 preserves all established contracts:

- PhraseClassification = none
- PhraseSpan = none
- SlurIntent = none
- NotationPolicy = none
- AnchorResolver = none
- PhraseConfidence = none
- scalarization = none
- similarity score = none
- novelty score = none
- learned/external weights = none
- external corpora/models/embeddings = none
- no voice collapse or majority vote
- no Render MusicXML write
- no Master MusicXML mutation
- no sounding-event mutation
- no new writer capability

## Required hard stop after r132

After a green single-target take, run the 10-etude corpus.

Do not automatically schedule Tonal Closure next.

The corpus must first be inspected with a human-readable set of strong, repeated and conflicting
boundary examples. The decision question is:

> Do Rhythm + Metric + Melodic + Parallelism + Structural Novelty now expose musically plausible
> phrase structure without forcing a classification?

Only if important ambiguities remain specifically tonal should a later tonal/closure evidence take
be admitted. Otherwise the next phase may proceed directly toward PhraseStructure audit.
