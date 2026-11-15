# Provenienz der notierten Anfangstempoangabe

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
CausalFeature: an explicit score-bound opening tempo and a rate derived from performance duration have different provenance and describe different quantities
CausalFeature: an authored metronome mark consists of a beat unit with its augmentation and a positive rate for that unit; equivalent rates in another unit do not preserve the authored representation
CausalFeature: a visible opening tempo fails source fidelity when it substitutes an estimate for a valid authored mark bound to the same score and opening, or presents that estimate alongside the authored mark as a competing opening indication
NonCausalContextExcluded: composition, project name, numeric measure identity, exact tempo, beat unit value, verbal tempo text, corpus frequency, soundtrack duration value and known evaluation locations

## Fachliche Grenze

Eine ausdrücklich vorgegebene Anfangsangabe bewahrt den notierten
Notenwert, die Punktierung und den Wert pro Einheit. Ein mitgelieferter
Tempoausdruck gehört zu derselben Angabe. Ein daraus abgeleiteter
MusicXML-Wiedergabewert ist in Vierteln pro Minute auszudrücken und
muss genau zu dieser Angabe passen. Die normierte Wiedergabezahl
ersetzt den sichtbaren Notenwert nicht.

Die Vorgabe muss als Eingabe an die konkrete Partitur und ihren Beginn
gebunden sein. Ein PDF-Vergleich und die Länge einer Einspielung sind
Evaluation. Auch ein identischer numerischer Wert ohne dieselbe Provenienz
belegt die notierte Angabe nicht.

Eine spätere Zulassung muss fehlende, ungültige, mehrdeutige und fremde
Vorgaben berücksichtigen. Ohne Vorgabe bleibt der bestehende WAV-Vertrag
maßgeblich. Ein Folgevertrag muss die bereits vorliegenden unabhängigen
Tempoereignisse und ihre Zeitpositionen erhalten und Konflikte ausdrücklich
behandeln. Eine Änderung des WAV-Tempo-Owners oder seiner Aufrufgrenzen
erfordert eine eigene Migration des gebundenen SSC-Vertrags.

Dieser Take friert die Fehlerklasse ein. Er definiert kein Eingabeformat,
keinen Selektor und keine Schreibautorität.
