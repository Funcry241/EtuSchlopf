# Corroborated performance meter

SelectorDerivationContract: causal_class_first_v1
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionMayRefineClassToKnownWitness: no
NonCausalContextExcluded: title, filename, performer, source ordinal, absolute measure, pitch labels, corpus membership and reference output.
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
CausalFeature: competing meters must agree with a common observed pulse before comparing their written subdivisions.
CausalFeature: concordant MIDI key releases lend bounded support to a shared written onset.
CausalFeature: recurrent chord transitions corroborate a rhythmic interval when recorded peers and the local tempo agree.

Competing meters belong to one conflict group for the same onset course.
A repeated occurrence is a distinct source interval. Duplicated readings
cannot corroborate one another or create a second written event. A shared
pulse must still support one meter, one bar phase, and one onset course
through the existing admission checks. Releases and repeated transitions are
bounded evidence in that conflict group. All source events remain bound.
