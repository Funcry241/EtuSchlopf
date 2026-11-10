# Preserving declared MIDI voices

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
ParentErrorClassContract: docs/selector_derivation_contract_midi_declared_voice_continuity.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: opening voice labels declare distinct written roles only with complete note-on and note-off custody in their source lanes
CausalFeature: overlapping events within one declared lane must have the same notated interval and distinct pitches to be a chord
CausalFeature: a declared voice keeps separate cursor coverage and preserves each exact source interval through tied fragments across bars
CausalFeature: an unison with separate written voices and opposing stems is a readable independent presentation
NonCausalContextExcluded: project identity, score title, etude, measure number, absolute pitch, track order, numeric channel values, corpus frequency and renderer outcome

Source names with vocal roles form a declared mapping when each voice
occupies one internally consistent source lane. Missing or contradictory
roles retain the existing hand allocation. Each independent course carries
its own rests and starts and ends every note at the projected source time.
The seed emits the initial and continued fragments together. Arrangement
transports their original identities through existing forward slots.
