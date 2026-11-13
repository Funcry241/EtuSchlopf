# Spannweitenaufteilung beim MIDI-Entwurf

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
CausalFeature: the early onset split and the aligned span check can use the same retained attack group and effective onset
CausalFeature: publishing a provisional hand assignment before the existing feasible split can lead to opposing decisions by the same owner
CausalFeature: jump smoothing preserves the committed wide onset split and later staff changes can invalidate it
NonCausalContextExcluded: work identity, measure labels, pitch values, source ordinal values and independent group counts

Bei Erhalt der Anschläge bewertet der Owner die bevorzugte Aufteilung
virtuell mit der bestehenden Spannweitenregel. Falls eine zulässige
Teilung existiert, schreibt er direkt deren Ergebnis und bindet es an
dieselbe Gruppe. Die Auswahl verwendet weiterhin die vorhandene
Reihenfolge und die Zahl der Änderungen relativ zur bevorzugten Aufteilung.
Die Sprungglättung respektiert die bereits festgelegte weite Gruppe.
Spätere Änderungen durch den Trajektorien- oder Restfehler-Owner können
eine erneute Prüfung auslösen. Gebundene MIDI-Stimmen folgen ihrer
Quellzuordnung.

Die synthetische Gegenprobe vergleicht die bevorzugte Aufteilung mit der
zulässigen Aufteilung und prüft, dass eine Wiederholung dieselben Noten
und keine weiteren Entscheidungen ergibt.
