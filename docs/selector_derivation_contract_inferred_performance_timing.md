# Inferred timing of binary MIDI performances

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
ParentErrorClassContract: docs/selector_derivation_contract_source_time_notation_projection.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: performed onset clusters repeatedly support one local binary subdivision and recurring bass accents support a quarter phase
CausalFeature: short neighbouring attacks inside one group can form a stepwise approach to a longer host
NonCausalContextExcluded: project title, filename, source hash, absolute measure number, absolute pitch identity and corpus membership

The owner only considers inference after exact projection fails. The complete
performance is the shared pulse domain. The projected notation preserves
source-note identities and the pipeline validates the resulting document.

This contract also applies the existing attack-preservation and hand-split
policy to the inferred timing plan. The project reads MIDI and WAV as its
input pair. Exact MIDI and embedded timing markers are evaluated first.

CausalFeature: an inferred written release overlaps later attacks on the
same staff and can use the existing independent lane within the measure.
The onset must include a primary note with a release at the next onset.
The held value must be representable as one note and cannot cover a repeated
attack of the same pitch. The longest eligible release uses the independent
lane when that lane is free. Exact timing retains its existing rules.
