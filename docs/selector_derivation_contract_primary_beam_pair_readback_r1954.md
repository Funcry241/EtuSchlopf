# Selector Derivation Contract - Primary Beam Pair Readback

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
CausalFeature: Direct owner adjacency requires consecutive, uniquely bound primary members in the same closed group.
CausalFeature: A production output must bind to the same source, attack and fragment through the FinalChord input range and that execution's pitched output ordinal. Exact byte identity carries a no-op binding.
CausalFeature: Two distinct written heads with different voices establish a separation. An intact edge requires a compatible span, ordered adjacency and compatible markers.
NonCausalContextExcluded: Project, score title, fixed location, pitch, fixed duration, specific voice label and corpus distribution are excluded as target selectors.

The readback considers consecutive pairs from observed owner groups. It also
considers outgoing and incoming primary markers on ordered, non-overlapping
source intervals within the same measure and staff, including markers observed
in the first production Master bytes. This makes different groups and
unbound original memberships observable. It does not infer a common group
from matching markers. An unmatched written carrier is recorded separately.

The readback reports original ownership, the already bound preprojection,
the selected voice plan for unchanged heads, and the actual Master
FinalChord output before stem projection. The first observed state is a
stage observation, not a claim about a specific allocator or replacement
decision. Incomplete provenance, multiple possible emissions, and unresolved
carrier identity leave that observation open. XML and allocation stay under
their existing owners. Score ordinals are zero-based original pitched
occurrence coordinates for diagnosis.
