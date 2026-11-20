# Gebundene Notendauer am MIDI-Seed

Fehlerklassendefinition für r1962 auf r1961. Die Annahme über den Windows-Launcher steht aus.

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
ErrorClassName: bound_written_duration_lost_in_seed
CausalFeature: the notation-time authority binds one retained regular pitched source occurrence to a mandatory nonempty written interval in the seed metric
CausalFeature: the MIDI seed proposes or emits fragments with the same source provenance whose union, internal boundaries or tie identity fail to realize that interval exactly once
CausalFeature: the mismatch is established at the MIDI seed planning or emission boundary before another owner receives the notation
NonCausalContextExcluded: work, filename, corpus membership, title, printed measure, absolute pitch, numeric staff or voice labels, occurrence ID values, absolute duration as a recognition pattern, candidate counts, renderer output and historical revision

## Geltungsbereich und notwendige Merkmale

Der Owner für Notationszeit legt die Bedeutung des geschriebenen Intervalls fest.
Eine explizit autorisierte Notenzeit oder ein bindendes Ergebnis einer exakten
Quellprojektion kann diese Verpflichtung begründen. Der physische MIDI-Release
und eine noch offene Schätzung der geschriebenen Dauer reichen dafür allein
nicht aus. Die Klasse setzt eine bereits getroffene Zeitentscheidung voraus.
Die Herleitung der Zeit bleibt beim bisherigen Owner.

Der Geltungsbereich umfasst reguläre Tonereignisse, die der produktive Seed
ausdrücklich zur Notation übernommen hat. Er umfasst auch gleichzeitige
Quellereignisse mit gleicher Tonhöhe, sofern sie eigenständig gebunden sind.
Die Herkunft bestimmt ihre Identität. Die Zeitkoordinate kann sie nicht
ersetzen. Zulässige Ausblendung, Restifizierung durch das nachgelagerte
Arrangement und Fälle ohne geschriebene Zeit werden durch eigene Verträge
beurteilt. Ein gebundener Vorschlag behält seinen Nullzeitpunkt und seine
Hauptnote. Für ihn gilt der vorhandene Vorschlagsvertrag.

Für eine Quelle mit Sollintervall `[a,b)` erfüllen ihre geordneten Fragmente
`[s_i,e_i)` gemeinsam alle folgenden Aussagen:

- Es gibt mindestens ein Fragment, alle Fragmente haben positive Länge und
  treffen mit `s_1 = a` und `e_n = b` die Sollgrenzen.
- Für jedes benachbarte Paar gilt `e_i = s_(i+1)`. Damit sind Lücken und
  Überdeckung ausgeschlossen. Gleiche Gesamtdauer allein genügt nicht.
- Alle Fragmente tragen dieselbe Quellidentität und klingende Tonhöhe.
  Nur das erste Fragment trägt den Ursprungsanschlag. Fortsetzungen tragen
  die zur Teilung passenden Haltebindungen.
- Die tatsächliche Ausgabe belegt dieselben Zeiten. Dazu zählen der
  geschriebene Cursor, die Akkordlage, die notierte Fragmentdauer und die
  ausgegebenen Haltebindungen. Eine zweite Attacke kann eine Haltefortsetzung
  nicht ersetzen.

Fehlt der bindende Ursprung, ist ein Verstoß gegen diese Klasse unbelegt.
Geht ein gebundenes Fragment verloren, gilt das auch bei einer zeitlich
gleichen Pause oder bei einem gleich langen Fragment einer anderen Quelle.
Beginnt eine andere Note, kann das eine gemeinsame Kanalbelegung verhindern.
Der Beginn der anderen Note verändert die Dauerverpflichtung nicht.
Ein technisch unzulässiger Plan bedarf einer anderen belegten Belegung
oder eines konkreten Ablehnungsgrundes am MIDI-Owner.

## Nachweisgrenze

Der Fehler entsteht zwischen `NotationTimePlan` und den veröffentlichten
rhythmischen Seed-Kursen. Maßgeblich sind die vorhandene Bindung beim
Übernehmen des Quellereignisses, die Planung in
`MidiSeedRhythmicCourses::create_metric_presentations` und der Rücklesepunkt
`seal_emission`. Ein späterer Abgleich im Master oder PDF bewertet, ob die
korrekte Zeit auch die weiteren Stufen überlebt.

Ein Beleg nennt Quell-ID, Herkunft der Zeitentscheidung, Sollintervall,
tatsächlich geplante oder geschriebene Fragmente, Prüfgrenze und verletzte
Eigenschaft. Beschriftung, Stimmnummer und musikalische Einordnung können
in der menschlichen Auswertung erscheinen. Der Vergleich verwendet die
vorwärts mitgeführte Quellidentität.

## Synthetische Gegenproben für eine spätere Umsetzung

Die Zeitkoordinaten stehen in einem einheitlichen notierbaren Raster. Die
Beispiele beschreiben Eigenschaften für den produktiven Seed-Einstieg.

| Gegenprobe | Prüfaussage |
| --- | --- |
| Zwei reguläre Ereignisse belegen `[0,72)` und `[24,48)`. | Beide Intervalle bleiben vollständig. Die zweite Attacke kürzt das erste Intervall nicht. |
| Zwei gleichzeitige Ereignisse belegen `[0,24)` und `[0,48)`. | Beide Enden bleiben erhalten. Eine gemeinsame Darstellung muss beide Enden legal tragen. |
| Auf `[0,24)` folgt ein Einsatz bei `48`. | Die Lücke `[24,48)` verlängert den ersten Ton nicht. |
| Ein Intervall überquert mehrere Taktgrenzen. | Die Fragmente schließen lückenlos an, mit genau einem Ursprungsanschlag und vollständiger Haltekette. |
| Soll ist `[24,96)`, ausgegeben werden `[24,72)` und `[48,72)`. | Die Summe der Fragmentdauern ist 72. Überdeckung und falsches Ende verletzen den Vertrag. |
| Eine geschriebene Dauer ist noch offen. Ein anderer Eintrag ist ein gebundener Vorschlag. | Die Dauerklasse greift für beide Fälle nicht. Quellzeit beziehungsweise Vorschlagsbindung bleiben anhand ihrer eigenen Verträge prüfbar. |
| Eine unabhängige Kopie derselben synthetischen Relation kommt hinzu. | Der jeweilige Befund bleibt gleich. Umbenennung, bloße Umnummerierung und eine äquivalente Verschiebung des Metrums ändern ihn ebenfalls nicht. |

Die Auswahl einer konkreten technischen Belegung und ihr zulässiger
Umfang gehören in den späteren Umsetzungsvertrag. Diese Revision führt
die Fehlerklasse als Voraussetzung dafür ein.
