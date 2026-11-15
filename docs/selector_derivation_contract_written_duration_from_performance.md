# Geschriebene Notenwerte aus einer Einspielung

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
CausalFeature: the input carries performed attacks and key releases without explicit written timing
CausalFeature: local rounding of key-release duration supplies a written note value without evidence from the rhythmic line it belongs to
CausalFeature: the unsupported duration becomes a written release boundary when the seed represents the attack as a separate note or tie chain
NonCausalContextExcluded: work, recording, filename, bar address, absolute pitch, staff number, voice number and corpus prevalence

Die Fehlerklasse liegt in der Ableitung der Notationszeit. Ein gespieltes
Notenende kann auf beiden Seiten des geschriebenen Endes liegen. Dieselbe
gespielte Länge kann je nach Artikulation und Stimme zu verschiedenen
Notenwerten gehören. Das Notenende aus der MIDI bleibt als Quellinformation
erhalten. Eine Änderung des geschriebenen Endes braucht eigene Evidenz aus
dem Einsatzraster und dem Verlauf der jeweiligen rhythmischen Linie.
Gleichzeitige Stimmen können an verschiedenen Stellen enden.

Eine spätere Umsetzung muss verkürzte lange Werte, überlappende kurze
Werte und tatsächlich selbstständige Haltewerte unterscheiden. Der
Vergleich eines Grenzwerts für gespielte Längen genügt dafür nicht.
Bei mehrdeutigem Verlauf bleibt der bisherige Wert bestehen. Gegenproben
prüfen beide Enden der Fehlerklasse sowie echte punktierte Werte,
Stimmen mit verschiedenen Notenenden, Halteketten und Vorschläge.
Direkt aus MIDI notierte Zeiten und ausdrücklich verankerte Zeiten
liegen außerhalb dieser Fehlerklasse.

## Diagnose des Laufs r1799

Die Befunde dienen als Gegenprobe. Werkname und Takte gehen in keine
Auswahlentscheidung ein. Taktzahlen zählen den Auftakt als Takt 1.
Der MIDI-Decoder aus r1799 meldet 452 Anschläge und denselben
Ereignis-Fingerprint wie der Windows-Lauf.
Das Diagnoseprogramm ruft den unveränderten produktiven
`NotationTimePlan::from_midi` auf. Den ursprünglichen Notenwert liefert
der Referenzabgleich des Vienna 4x22 Piano Corpus
(`match/Chopin_op10_no3_p01.match`, CC BY 4.0).
Die Einspielung stammt aus dem Originalkorpus
(Werner Goebl, DOI 10.21939/4X22).

Die Tabelle verwendet 96 Einheiten pro Viertel. Bindungen sind über ihre
Fragmente summiert.

| Takt und Lage | Ton | Gespielt (MIDI-Ticks) | Notationszeit | Master und Render | Referenz |
| --- | --- | ---: | ---: | ---: | ---: |
| 6 oben, Zählzeit 2 | E4 | 194370–207290 | 72 | 72 | 96 |
| 8 oben, zweite Achtel | Fis4 | 247010–257510 | 72 | 48 + 24 | 96 |
| 9 unten, Zählzeit 1 | Cis3 | 267140–279190 | 72 | 72 | 96 |
| 9 unten, Zählzeit 2 | H2 | 282910–293370 | 48 | 48 | 96 |
| 10 oben, Zählzeit 1 | Gis4 | 302750–311510 | 48 | 48 | 48 |
| 14 oben, Zählzeit 2 | E4 | 446780–459350 | 72 | 72 | 96 |
| 17 unten, Zählzeit 2 | Fis3 | 534600–539640 | 48 | 48 | 24 |
| 17 oben, Zählzeit 2 | B4 | 534620–540680 | 48 | 48 | 24 |
| 18 oben, Zählzeit 1 | E5 | 552330–571070 | 144 | 96 + 48 | 96 |
| 20 oben, Zählzeit 1 | H4 | 619990–631930 | 72 | 72 | 96 |
| 21 unten, Zählzeit 1 | H2 | 652910–663200 | 72 | 72 | 24 |
| 21 unten, Zählzeit 2 | H2 | 669610–679190 | 72 | 72 | 24 |

Der erste vom Referenzwert abweichende Notenwert entsteht in
`inferred_performance_timing::infer`. Dort teilt das Programm die
gespielte Länge durch die lokale Unterteilung, multipliziert mit 1,10
und rundet auf ganze Sechzehntel. Beim E4 in Takt 6 ergibt das drei
Sechzehntel. Beim H2 in Takt 21 ergibt es ebenfalls drei Sechzehntel,
obwohl die Referenz dort ein Sechzehntel schreibt. `midi_seed` kann
diese Werte als eigenständige Noten oder in metrisch geteilten
Halteketten ausgeben. Der Renderer übernimmt sie.

Das Gis4 in Takt 10 ist im Referenzabgleich tatsächlich ein Achtel.
Das kleine B4 in Takt 8 hat bereits im Notationszeitplan die Dauer null
und ist an seine Hauptnote gebunden. Der Referenzabgleich weist es als
Vorschlag aus. Beide Stellen dienen als Gegenproben für eine Korrektur.

Dieser Take dokumentiert die Fehlerklasse und die Diagnose.
