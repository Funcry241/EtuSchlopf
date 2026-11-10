# Preserving source-bound compact chords at the arrangement visibility boundary

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
ParentErrorClassContract: docs/selector_derivation_contract_semantic_obligation_accounting_failure.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: distinct source occurrences arrive with exact origin binding and a common physical onset in one contiguous written chord
CausalFeature: the complete chord has distinct pitches and fits the count and span bounds already used by the simultaneous-hand checks
CausalFeature: counting the chord members as separate attacks can suppress source occurrences under the onset and measure limits even though one carrier advances the cursor

The existing arrangement owner protects the admitted chord members from its
chord-count, parallel-import and measure-density reductions. The head still
counts as one attack. Secondary-voice and chord-anchor checks continue to
apply. The owner does not rewrite notes, move a hand or infer a fingering.
Independent groups make their own decisions. The same-pitch case, groups
without complete binding, and groups beyond the simultaneous-hand bounds
retain their existing visibility policy.

NonCausalContextExcluded: project, etude, title, measure number, fixed pitch, voice or staff label values, corpus frequency and candidate count across independent onsets
