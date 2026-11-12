# Require the current canonical exchange before invoking E057

SelectorDerivationContract: causal_class_first_v1
SelectorChange: yes
SelectorRole: repair_scope
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
ParentErrorClassContract: docs/selector_derivation_contract_rest_bridge_consumer_order.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the existing E057 writer can act only on a canonical cursor-exchange plan bound to the exact input document
CausalFeature: a fragmented-rest write does not bind such a plan, and a missing or stale plan is refused before invoking the writer
CausalFeature: the PDF preparation binds the current continuation and the existing writer still verifies the plan and transports the original receipt
NonCausalContextExcluded: project identity, printed measure label, pitch, voice number, and scorewide candidate count

The admission check inspects only the plan already attached to the current owner. It cannot add a plan, change a resource, or grant write authority. The existing synthetic exchange test checks missing, current, and stale input.
