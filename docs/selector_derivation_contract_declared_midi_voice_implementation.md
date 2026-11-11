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
CausalFeature: an occurrence-bound source voice fixes written staff and voice before the solver may infer a hand distribution
CausalFeature: a complete continuous two-hand assignment can disprove a HAN02 or HAN03 demand inferred only from written staff allocation
CausalFeature: a source-bound tie fragment covering a complete 4/4 bar or aligned half bar has an exact unsplit value
CausalFeature: an unison with separate written voices and opposing stems is a readable independent presentation
NonCausalContextExcluded: project identity, score title, etude, measure number, absolute pitch, track order, numeric channel values, corpus frequency and renderer outcome

Source names with vocal roles form a declared mapping when each voice
occupies one internally consistent source lane. Missing or contradictory
roles retain the existing hand allocation. Each independent course carries
its own rests and starts and ends every note at the projected source time.
The seed emits the initial and continued fragments together. Arrangement
transports their original identities through existing forward slots.

The arrangement solver completes a qualified source at the initial
arrangement boundary. Guided search, playability repairs and gesture
variants may infer staff and voice assignments only for other sources.
The qualified path still produces the normal solver diagnostics, and the
notation and release stages still run.

For source-bound voices, the validator can establish a two-hand assignment
over the visible key intervals of a part. An occupied key keeps its hand
assignment until release. Each hand is checked against the existing key
count, octave span and immediate jump thresholds. If no assignment fits
or the timing representation is unsupported, the ordinary warnings apply.
This is a keyboard geometry check for HAN02 and HAN03. Fingering, timing
in seconds, and performer-specific reach require separate evidence.

For a source-bound tie in 4/4, the seed uses a whole note for a full bar
and a half note for an aligned half bar. The visible rhythm ledger keeps
those values when the host and each chord member have distinct source
bindings, matching metric support and explicit tie endpoints.
