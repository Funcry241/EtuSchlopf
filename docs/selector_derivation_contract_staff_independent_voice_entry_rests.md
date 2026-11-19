# Metrisch nachgewiesene Stille vor dem ersten Stimmeinsatz

SelectorDerivationContract: causal_class_first_v1
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionMayRefineClassToKnownWitness: no
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
SelectorChange: yes
SelectorRole: repair_scope
CorpusDistributionPurpose: measurement_only
ParentErrorClassContract: docs/selector_derivation_contract_written_voice_entry_silence.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the first visible pitched entry on a staff has a unique written voice, a positive onset and a stable physical insertion anchor
CausalFeature: any sounding interval of that voice in the prefix, including a continuation or hidden chord carrier on another staff, invalidates a leading silence claim
CausalFeature: a wholly unrepresented silence prefix is decomposed by the existing metric rest planner and written with an exactly cursor-neutral rewind
CausalFeature: between-note rest completion consumes the same interval proof and leaves partially represented intervals intact at its original insertion anchor
NonCausalContextExcluded: project identity, printed measure label, fixed staff, register, clef, octave direction, other-voice activity, one-rest limit and global ranking

Diese Stufe verarbeitet vollständig unbelegte Anfangsintervalle. Teilweise
abgedeckte Intervalle benötigen eine eigene exakte Projektion. Jede
Anwendung bleibt an der vorhandenen Schreibstelle und an deren Prüfung
von Klangereignissen sowie MusicXML-Zeit gebunden.
