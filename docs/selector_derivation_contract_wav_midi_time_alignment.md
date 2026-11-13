# Physischer Zeitabgleich für eingefrorene WAV-Ereignisse

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
ParentErrorClassContract: docs/selector_derivation_contract_wav_midi_time_basis.md
ScopeNarrowingBasis: authority_or_safety_only
DiscriminativeContextOnlyNarrowing: forbidden
CausalFeature: PPQ and ordered MIDI tempo metadata define the time projection from raw ticks into audio frames
CausalFeature: a single recording-origin shift needs support from same-pitch events and must improve ordered one-to-one matching
NonCausalContextExcluded: project identity, filename, reference edition, measure number, expected event count and known recording offset

Der Abgleich arbeitet mit bereits eingefrorenen Sensordaten. Die Suche nach einem konstanten Versatz verwendet 20-ms-Raster und ein auf zehn Sekunden je Richtung begrenztes Fenster. Ein Vorschlag braucht mindestens acht stützende MIDI-Ereignisse und mindestens zehn Prozent der MIDI-Ereignisse. Der Abgleich übernimmt den Versatz nur, wenn er mehr Eins-zu-eins-Treffer innerhalb des bestehenden 80-ms-Fensters ergibt. Diese Grenzen sind Parameter der Auswertung. Die Treffer sind eine Anpassungsmessung am selben Material. Eine unabhängige Validierung der Passung und zeitlich variable Abweichungen bleiben offen.

Jedes MIDI/WAV-Paar bildet eine eigene Entscheidungsdomäne. Eine unabhängige Aufnahme oder deren Duplikat kann den Versatz dieses Paars nicht beeinflussen. Der Abgleich hat keine Autorität für Notenerzeugung oder Notensatz.
