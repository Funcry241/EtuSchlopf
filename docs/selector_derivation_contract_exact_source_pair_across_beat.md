# Exact source releases of overlapping two-note onsets

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
ParentErrorClassContract: docs/selector_derivation_contract_independent_sustain_density_displacement.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: a source pair shares an attack, has two distinct releases and can be carried by one cursor-advancing note and one independent interval inside the same measure
CausalFeature: the shorter source interval reaches the next primary attack and the succeeding exact note or chord intervals cover the longer source release without a reattack of its key
CausalFeature: the existing seed route can preserve both releases without shifting any attack, and the existing density exemption prevents the independent interval from displacing a primary note
NonCausalContextExcluded: project, etude, measure number, fixed pitch, staff and voice label values, corpus prevalence and count of independent groups

The shorter note carries the main cursor even when pitch order would have
selected the longer one. The longer note keeps its original source identity
and runs on the existing independent cursor. Same-key releases keep their
existing opposite-stem treatment. The legacy one-intervening-note admission
remains available in its former domain. An intervening chord qualifies only when every member ends at the next
primary attack. A metric beat boundary does not terminate the independent
source interval. Gaps, reattacks, overlapping independent intervals and
unrepresentable durations remain outside this scope. This changes the source representation, not a late PDF repair.

A beam group must also advance in the order of its serialized containers.
A rewind can put a later-onset note earlier in the stream. An onset sort
does not authorize emitting beam markers in the opposite order. The
existing NotationPolish and metric planning owners refuse that group
before materialization. This implements the ordered-container boundary of
`selector_derivation_contract_generated_metric_beam_coverage.md` in the
accepted parent. The final readback continues to reject unbound markers.
