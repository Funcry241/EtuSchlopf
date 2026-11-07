# Ordered readback of surviving original beam edges

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
ParentErrorClassContract: docs/selector_derivation_contract_sounding_lane_role_authority.md
ScopeNarrowingBasis: causal_class_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: The original owner capture preserves its ordered marker sequence and binds each retained member to a canonical occurrence.
CausalFeature: An unambiguous one-to-one projection, written ordering and adjacency in the observed strand are necessary for comparing the two edge directions.
CausalFeature: A closed owner group, a complete locally bound pair and two projected endpoint markers distinguish present edges from missing edges.
NonCausalContextExcluded: project identity, printed measure, pitch, numeric voice, corpus frequency, discovery order

The probe reads the existing FinalChord projection and the surviving beam
owner capture. It reports per-level adjacency, without inferring an edge
from ties or chord members. Numeric voice labels are compared only for
identity within the observed projection. A split emission, uncertain
ordering, intervening pitched note or marked rest leaves the pair open.
An unsupported endpoint marker, including a hook, leaves the pair open.

The comparison is conditional on retaining the received owner edges.
It covers the surviving capture at this seam. Captures dropped earlier
and the authorization of additional rest bridges are outside this scope.
The acceptance condition is byte-identical Master and Render MusicXML
for all ten projects compared with accepted r1632.
