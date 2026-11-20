# Selector Derivation Contract - Primary Beam Onset Readback

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
CausalFeature: MusicXML cursor movement determines the written onset of each visible positive-duration primary-beam head.
CausalFeature: Marker balance is measured inside the current part, measure and voice, including a voice that crosses staves.
CausalFeature: A backward step in that marker stream distinguishes XML encounter order from onset order. An equal onset between marked heads leaves chronological balance ambiguous.
NonCausalContextExcluded: project, score title, measure number, staff number, pitch, fixed duration, numeric voice label, source identifier and corpus distribution used as target selectors.

The audit compares existing XML encounter-order totals with a reading by onset.
It reports positions and marker-state anomalies for the current Master and
Render documents. Simultaneous heads are marked as ambiguous. Chord members,
hidden heads and grace notes are outside the onset readback of ordinary visible
heads. A voice identifier groups current notation and is included as a diagnostic
coordinate.

This observation measures syntax within the seam described by the frozen error
class. Current markers alone supply neither the class's exact owner evidence
nor a before-after occurrence correspondence. These rows have no repair or
release authority. The productive writer remains under its existing contract.
