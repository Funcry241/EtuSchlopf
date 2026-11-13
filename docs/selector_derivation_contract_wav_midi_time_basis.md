# Zeitbasis für den Abgleich von MIDI und WAV

SelectorDerivationContract: causal_class_first_v1
SelectorChange: no
SelectorRole: error_class_definition
KnownWitnessUsedForFeatureDerivation: no
TargetConditionedFeatureNarrowing: no
ClassDefinitionPrecedesCorpusDistribution: yes
UniquenessIsOutcomeNotGoal: yes
RepairSelectorSeparatedFromErrorClass: yes
CausalFeatureNecessityReview: complete
CorpusDistributionPurpose: not_yet_measured
CorpusDistributionMayRefineClassToKnownWitness: no
ParentErrorClassContract: none
ErrorClassDefinitionIndependentOfKnownWitness: yes
CausalFeature: the audio sensor and the MIDI decoder expose independent event streams whose timestamps must be projected into the same physical time domain
CausalFeature: MIDI ticks require their own PPQ and tempo changes; a quarter length inferred from the recording cannot establish their time unit
CausalFeature: the recording and MIDI can start at different times or diverge during the performance; a constant origin shift is admissible only within its measured scope
CausalFeature: ordered one-to-one equal-pitch matches and their residuals must be computed from the frozen audio evidence and a separately bound MIDI time projection
NonCausalContextExcluded: filename, work identity, expected recall, absolute measure, fixture-specific offsets, externally generated note mappings

## Umfang

Der Audio-Sensor friert seine erkannten Ereignisse vor dem MIDI-Abgleich ein. Der Abgleich projiziert die MIDI-Einsätze mit ihrer eigenen Tempokarte in die physische Zeit. Ein belegter konstanter Versatz darf den gemeinsamen Ursprung bestimmen. Wenn sich die Abweichung über den Verlauf ändert, ist eine separate, überprüfbare Zuordnung für den zeitlichen Verlauf erforderlich. Ein hoher Trefferwert allein belegt keine gültige Zuordnung.

Die Auswertung weist die Zahl der Eins-zu-eins-Treffer und die verbleibenden Zeitabweichungen aus. Die MIDI-Notenerzeugung bleibt von der Auswertung getrennt. MIDI und WAV liefern die Eingaben. Diese Definition führt keine Auswertung und keine Änderung des Programmverhaltens ein.
