# Read back metric beaming at a clef transition

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
ParentErrorClassContract: docs/selector_derivation_contract_metric_beam_clef_boundary.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the existing metric owner observes an effective visible clef transition between the sounding attacks on the same staff
CausalFeature: the committed notes match the metric owner's exact positive and negative beam decisions in the observed measure, and the rest carries no beam
NonCausalContextExcluded: piece, measure label, pitch, clef direction, discovery order and number of observed groups

The existing complete rest-bridge comparison is tried first. When it fails,
the readback can accept the clef-bounded metric plan from the same serialized
measure and the same metric cursor as the existing planner. Require exact
markers on every group member. An unresolved clef boundary or a missing owner
decision leaves the mismatch in force. The readback performs no materialization.
