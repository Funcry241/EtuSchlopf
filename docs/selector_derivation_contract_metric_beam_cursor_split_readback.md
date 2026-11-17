# Metric beam cursor split readback

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
ParentErrorClassContract: docs/selector_derivation_contract_partial_beat_cursor_lane_serialization_split.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: adjacent events on the same staff and voice have a positive primary connection from the shared metric beam owner
CausalFeature: a written boundary or an overlapping same-lane event makes the connection insufficiently determinate
CausalFeature: an explicitly foreign forward and a backup separate the otherwise contiguous events on the serialized lane
NonCausalContextExcluded: project, measure label, fixed staff, fixed figure, count or duration of preceding notes, and corpus frequency

A readback candidate is a split between two contiguous events on the same
serialized staff and voice. The shared metric beam planner must connect the
events at beam level 1. A written primary beam boundary, a cut from the
metric owner's clef boundary logic, or overlapping events on the same lane
prevents the inference.

The readback still requires a local forward with the duration of the later
event followed by a backup before that event. The forward must carry an explicit staff or voice distinct from the notes and immediately
follow the earlier note. This supplies a concrete cursor-lane witness.
The previous staff, preceding-rest, note-count, equal-duration and end-of-beat
conditions do not determine whether the planner connects two events.

The check is read-only. The existing cursor exchange still requires the
canonical bound plan to write and verifies event provenance independently.
Synthetic tests cover both staves, three note values, changed voices,
intentional beam breaks, clef boundaries and independent split candidates.
