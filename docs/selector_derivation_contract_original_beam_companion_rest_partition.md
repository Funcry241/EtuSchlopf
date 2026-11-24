# Complete rest partitions in original beam alignment

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
ParentErrorClassContract: docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: The existing original beam and frozen voice authorities must bind the closed beam component, and renderer observation must show excess fragments.
CausalFeature: The proposed pitched assignments must cover each colliding visible rest over its exact interval on the same staff.
CausalFeature: A gap-free, disjoint written rest partition must cover the full bounded silence in one original semantic voice, and an authenticated occurrence at its endpoint must identify that line's existing continuation channel.
CausalFeature: The destination must be free throughout the silence, with no beam crossing either boundary, and the complete scalar batch must pass occupancy and import registration checks.
NonCausalContextExcluded: Piece identity, measure number, pitch, interval shape, absolute onset, rest count, numeric staff and voice labels, corpus frequency, and evaluation order.

A rest collision limits the safe materialization of a beam correction. It
supplies no evidence of beam fragmentation. The silence is a half-open time
interval. The existing source gaps and all visible written rests must each
partition that same interval into positive, disjoint subintervals. The
partitions can have different internal endpoints.

The existing staff/voice writer commits the pitched assignments and the
complete companion rest partition together. The plan transports the associated
source gaps to the same channel. The rest receipt verifies staff, onset,
duration and the exact XML with only the technical voice value replaced.
A partial or ambiguous proof leaves the conflicting beam proposal ineligible.
The existing conflict handling continues to evaluate independent components
locally. Synthetic tests vary interval offsets and measure labels, and reject
incomplete partitions, overlaps, hidden rests and conflicting source voices.
