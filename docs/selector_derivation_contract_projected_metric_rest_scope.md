# Metric rest continuation in the projected score

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
ParentErrorClassContract: docs/selector_derivation_contract_generated_metric_beam_coverage.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: the shared metric owner supplies a complete same-staff and same-voice run in a single metric beat, with a contiguous pair and a visible rest that exactly fills a gap elsewhere in the run
CausalFeature: the same exact projected occurrences must pass the existing E050 voice binding and unowned beam checks before materialization
CausalFeature: a clef boundary, overlapping events, or incomplete rest coverage prevent this run from supplying a continuation across the affected boundary
CausalFeature: an existing E050 candidate on the same lane and beat keeps priority over a projected addition
NonCausalContextExcluded: project, measure label, fixed staff or voice value, absolute beat, exact rhythm figure and corpus distribution

The extension applies after FinalChord projection. The existing early E050
and withdrawal phases keep their original scope. The E050 materializer and
its existing exact occurrence binding, receipt and sink apply to the new
scope and write only beam levels 1 and 2. Independent runs do not compete.

A tied fragment and an attack separated by a rest are insufficient by
themselves. A positive complete metric run can include a tied fragment.
Synthetic cases exercise the existing E050 sink and compare an isolated pair
with a complete run on both staves.
