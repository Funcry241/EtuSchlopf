# Written chord spans supported by incoming tempo

SelectorDerivationContract: causal_class_first_v1
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionMayRefineClassToKnownWitness: no
NonCausalContextExcluded: title, filename, performer, source ordinal, absolute measure, absolute pitch labels, corpus membership and reference output.
SelectorChange: yes
SelectorRole: error_classification
ParentErrorClassContract: docs/selector_derivation_contract_ambiguous_performance_meter_class.md
CorpusDistributionPurpose: measurement_only
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: distinct other occurrences of the same harmonic transition agree on a written span.
CausalFeature: the incoming pulse, extrapolated without the disputed interval, supports the peer span over the original span across the available lookback widths.
CausalFeature: the alternative preserves the established relative phrase positions and introduces no new metric crossing at a corroborated release.

All disputed transitions in one onset course belong to the same conflict
group because moving a later onset can affect more than one observation.
The rhythm owner evaluates one jointly corroborated alternative and checks
its recurring transitions before it can proceed. Other attack intervals
stay paired with their original source events. Bar phase and agreement
across the shared pulse estimates still require their existing evidence.
