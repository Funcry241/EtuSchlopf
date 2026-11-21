# Measure anchors for one original register preview

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
ParentErrorClassContract: docs/selector_derivation_contract_joint_register_reading_context.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
NonCausalContextExcluded: score and file identity, printed measure labels, rarity, proposal count, visit order and timing

CausalFeature: the complete reading-course comparison borrows one immutable original document, so the same part and measure ordinal always denote the same byte range during that comparison.
CausalFeature: document intervals depend on the existing visible-anchor requirement and the earliest and latest matching note-group boundaries within that range.

The index records the first part with a given ID and its measure ranges in document order.
It decodes anchors only for a requested boundary measure. It retains both successful
and failed reads only inside this input. A visible matching member qualifies the
boundary, and every matching member contributes to its byte extent. Both endpoints
are decoded before start visibility is checked, as in the original lookup.

Each proposal still receives the same complete directions, costs and error at the
same stage. Direction assignment to duplicate part IDs retains the original traversal.
The productive writer uses the independent linear lookup. Repeating candidates
only reuses the same immutable observation. The index has no authority over
proposal ranking, line allocation, writing, or adoption.

The synthetic comparisons cover duplicate part IDs and labels, ambiguous visible
and hidden anchors, malformed isolated measure XML, missing boundaries, and
overlapping proposals. The complete-cost comparison and the original owner selection
compare the result with the linear reference, including UNKNOWN.
