# Octave display span boundary at an incoming tie continuation

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
ClassDerivationBasis: documented consumer import semantics in which the octave-shift state changes the written-to-sounding pitch mapping at the span boundary; corpus witnesses do not define the predicate
CausalFeature: an octave display span (octave-shift start or stop) is planned or serialized for a part and staff
CausalFeature: the span's start or end position coincides with a note event in the same part and staff that is the stop of an incoming tie from an event before that position, including hidden events and chord members
CausalFeature: the tied predecessor and the continuation therefore lie on different sides of the display boundary, so their written pitches differ by the display transposition although their sounding pitch is identical
CausalFeature: a consumer that matches tie stops against the written or display-adjusted predecessor cannot pair them; the tie stop is lost or reinterpreted, for example as an unterminated let-ring, although the sounding continuation is unchanged
NonCausalContextExcluded: project etude measure number pitch spelling register voice label source or occurrence identifiers page system revision and corpus rarity
NonCausalContextExcluded: renderer force flags converter exit code PDF existence octave span length octave size direction and whether the span is semantic or an intermediate island

## Class meaning

A tie is a sounding continuation; an octave display span is a notation
attribute. Placing the display boundary exactly at a tie continuation makes the
two notated halves of one sounding note look different to a consumer. The
continuation is musically intact in the model but not in the imported score.

Continuations strictly inside a constant span are not members: both halves carry
the same display transposition. Independent parts or staves do not affect
membership. An unmatched incoming tie is still a continuation at the boundary
and is a member; it never grants permission for a display boundary there.

## Necessity review

- Without an octave display boundary there is no transposition change.
- Without a tie continuation at that boundary no pairing is required.
- Inside a constant span the transposition is equal on both halves.
- A different part or staff is paired independently by the consumer.

## Invariance

Uniform duration scaling, renaming labels, measures or projects, adding
independent parts, and permuting unrelated events preserve membership.
Duplicating an unrelated independent span elsewhere does not change membership
of an existing boundary.

## Authority boundary

This definition grants no planner filter, writer, relabeling, tie edit or span
extension. A later implementation must cite this file as
`ParentErrorClassContract`. It may only refuse a complete display plan whose
boundary is a member; it must not move, shorten, lengthen or substitute spans,
re-pitch events or invent tie elements, and it must keep all existing receipt,
materialization and adoption requirements unchanged.
