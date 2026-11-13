# Handwechsel anhand beider Handverläufe

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
ParentErrorClassContract: docs/selector_derivation_contract_hand_jump_reassignment_oscillation.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
MultiplicityInvarianceContract: candidate_duplication_v1
IndependentCandidateDuplicationInvariant: yes
ScorewideMultiplicityAsWriteAuthority: forbidden
GlobalRankingOfIndependentCandidates: forbidden
MultiplicityDecisionScope: local_candidate_or_explicit_conflict_group
CandidateDuplicationProbe: required_before_productive_write
CausalFeature: a handoff replaces local neighboring transitions in both hands including the donor bridge and the receiving edge displaced by insertion
CausalFeature: each accepted handoff strictly reduces the sum of positive jump excess over the existing locally focused limits
CausalFeature: the receiving hand must fit the existing key count and span limits throughout the transferred attack
NonCausalContextExcluded: work, bar, source ordinal, absolute pitch value, profile name and number of independent proposals

Der MIDI-Seed-Owner nutzt die bereits vorhandene zeit- und staffabhängige
Sprungschwelle. Eine Kante zwischen zeitlich getrennten Anschlägen zählt
innerhalb des bestehenden Fensters mit ihrem positiven Überschuss über
diese Schwelle. Gleichzeitige Anschläge zählen zur Belegung. Für einen
Handwechsel vergleicht der Owner die betroffenen Kanten in beiden Händen.
Unberührte Kanten fallen aus der Differenz heraus. Bei Gleichstand bleibt
die Handzuordnung bestehen. Jeder angenommene Wechsel senkt denselben
ganzzahligen Aufwand der aktuellen Zuordnung, sodass diese Zuordnung
innerhalb der Sprungglättung nicht zyklisch wiederkehren kann.

Die bisherige registerabhängige Wahl des angebotenen Anschlags, die
Obergrenze für Durchläufe und der bestehende Quellbeleg bleiben erhalten.
Die Spannweite und Tastenanzahl der empfangenden Hand werden für alle
Belegungszunahmen während des Anschlags geprüft. Synthetische Tests
prüfen Gleichstand, zulässige Übergabe, größere Gegensprünge, die
Belegung bei Gleichzeitigkeit und zeitlich unabhängige Wiederholung.
