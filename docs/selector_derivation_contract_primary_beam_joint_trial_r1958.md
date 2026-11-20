# Selector Derivation Contract - Primary Beam Joint Channel Tie Trial

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
CausalFeature: At each assignment, only channels accepted by the existing conflict checks and with the best available technical lane rank can receive a beam preference.
CausalFeature: A uniquely eligible partner channel among already assigned closed primary beam partners takes precedence over the numeric tiebreak. Zero or conflicting eligible partner channels preserve the numeric tiebreak.
CausalFeature: The complete alternate allocation must pass the existing admission and Master projection. Previously present reference pairs are checked individually together with the previously split reference tie targets.
NonCausalContextExcluded: Score identity, bar, pitch, specific fragment identifiers and reference totals do not select a channel.

The diagnostic executes one candidate. Its assignment state evolves as the
allocator progresses and its outputs remain in memory. The productive plan
supplies the reference and cannot adopt the diagnostic candidate.
