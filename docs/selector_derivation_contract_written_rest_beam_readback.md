# Written rest-beam continuity readback

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: error_classification
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: measurement_only
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
ParentErrorClassContract: docs/selector_derivation_contract_ordered_beam_lane_interference.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: closure is scoped to written contiguous note and visible-rest events on one bound staff and voice
CausalFeature: a continued or terminated beam after the gap requires an open same-level beam before the gap, with exactly one opening
CausalFeature: a visible clef change invalidates a continuation across its boundary
CausalFeature: a written closure immediately before a rest allows the later note to start a separate course
NonCausalContextExcluded: score identity, measure address, absolute beat, fixed staff, voice label, number of positions, and exact note value

The committed readback observes existing markers at the two levels used by
the metric rest-bridge owner. It can identify an interrupted written course
across a visible rest or a rest carrier at a claimed continuation. It cannot
infer a wholly absent connection or impose grouping on explicitly closed
beams. The production write admission and its structural source binding are
outside this change.

Tests move the same error across staves and offsets, vary note values, cover
several rest gaps, and preserve deliberate closure at a clef boundary.
