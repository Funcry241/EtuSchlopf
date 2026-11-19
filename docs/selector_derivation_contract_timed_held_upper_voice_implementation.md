# Zeitlich eindeutige Fortsetzung einer gehaltenen Oberstimme

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
ParentErrorClassContract: docs/selector_derivation_contract_held_upper_voice_continuation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: the resolved held upper note anchors one continuous voice until its bound release
CausalFeature: a continuation fills the current metrical interval with exactly representable source durations and attacks at the preceding upper note releases
CausalFeature: the primary cursor can represent all other notes at their original source releases within this interval
CausalFeature: the existing allocation remains effective when the local proof allows multiple source assignments or cannot preserve both cursors
NonCausalContextExcluded: work identity, printed measure number, absolute pitches, fixed interval size, candidate frequency and scorewide ranking

Die Stimmenzuordnung im MIDI-Import prüft unmittelbar vor der Übernahme
einer Fortsetzung, ob die vorhandenen Stimmen sämtliche Quellanschläge
und Loslasszeiten im aktuellen metrischen Abschnitt erhalten können.
Während die Oberstimme klingt, gehören weitere Anschläge zum Hauptcursor.
Am Loslasszeitpunkt der Oberstimme muss genau ein Quellanschlag ihre Linie
fortsetzen, damit alle Loslasszeiten erhalten bleiben. Die übrigen Anschläge
müssen zeitlich exakt in den Hauptcursor passen. Kann der Hauptcursor die
gesamte Einsatzgruppe abbilden, bleibt die bisherige Zuordnung in Kraft.
Das gilt auch für einen einzelnen Ton, der in beide Stimmen passen würde.

Bei eindeutiger Zuordnung spielt die Intervallgröße zwischen den Folgetönen
keine Rolle. Fälle ohne eindeutigen Beleg verwenden die bisherige Zuordnung.
Der bestehende Owner serialisiert die gebundene Folge über seine vorhandenen
Cursor und prüft die Quellbindung. Taktübergreifend gebundene Fortsetzungen
verwenden ihren eigenen, bestehenden Pfad.
