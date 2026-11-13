# Zeitzuordnung zwischen MIDI und WAV

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
ParentErrorClassContract: docs/selector_derivation_contract_melodic_dynamics_evidence.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: original Note-on ticks and tempo events determine elapsed MIDI playback time
CausalFeature: ordered PCM attacks support a shared offset across separated time ranges
CausalFeature: distinct offsets compete to explain the same pair of input clocks
NonCausalContextExcluded: work identity, measure address, absolute pitch and written voice labels

Der Projektlauf meldet die zeitliche Übereinstimmung beider Eingaben in der
Konsole. Die Diagnose prüft einen konstanten Versatz im Bereich von ±10 s
in 10-ms-Schritten. Sie zählt eine WAV-Transiente innerhalb von ±30 ms
höchstens einmal. Mindestens zwölf Anschlagszeiten, mindestens 75 Prozent
Gesamtdeckung, ein Abstand von mindestens zehn Prozentpunkten zur besten
getrennten Alternative und mindestens zwei Drittel Deckung in jedem Zeitdrittel
stützen den Versatz. Die WAV-Diagnose kann bei dichten, leisen oder
wiederkehrenden Anschlägen offen bleiben. Die Zuordnung ist eine
Zeitmessung. Die Tonhöhe und Lautstärke einzelner Melodietöne erfordern
eigene Evidenz.
