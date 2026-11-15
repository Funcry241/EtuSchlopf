# Geteilter Wiederholungsakkord in der Handzuordnung

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
ErrorClassName: split_repeated_accompaniment_at_seed_hand_assignment
CausalFeature: a source-time chord has simultaneous distinct attacks with a common written release and is divided by an inferred hand boundary
CausalFeature: the same chord recurs as a complete group in one hand at the previous or next accompaniment attack while separately bound notes can continue in the other hand
CausalFeature: the held keys and new attacks in the two hands admit the intact assignment for the full affected interval
CausalFeature: the split transfers a chord member into a separate written course and can shorten its value or attach it to the other course's beam
NonCausalContextExcluded: work, recording, filenames, measure labels, absolute pitches, numeric voice labels, repeated instance count, candidate rank and renderer outcome

## Befund und Zuständigkeit

Verglichen werden Anschläge auf derselben gebundenen Notationszeitachse.
Ein wiederholter Akkord hat dieselben Tonhöhen und eine gemeinsame
Einsatz- und Freigabezeit seiner Mitglieder. Andere Töne können separat
anschlagen oder weiterklingen. Die Wiederholung liefert Evidenz für die
Handzuordnung, wenn der vollständige Akkord dort dieselbe Hand teilt und
die übrigen Töne unabhängig davon gebunden sind.

Die Entscheidung gehört in die Handzuordnung des MIDI-Entwurfs vor der
Bildung der rhythmischen Stimmen. Dort sind die gesamten Akkordmitglieder
und die gehaltenen Tasten beider Hände gleichzeitig verfügbar. Die
Einteilung der Stimmen, die Balken und das PDF sind nachgelagerte Folgen.

## Grenzen eines späteren Eingriffs

Eine übernommene Handzuordnung erhält jeden Quellanschlag mit Tonhöhe,
Einsatz und gebundener Dauer. Sie darf keine deklarierte Quellstimme
umdeuten. Die Handspanne und die Anzahl der gehaltenen Tasten müssen
über den gesamten betroffenen Abschnitt in beiden Händen zulässig
bleiben. Widersprüchliche Wiederholungen, ungebundene Akkordmitglieder
und ein zwischenzeitlich belegter Zielbereich lassen die Zuordnung offen.

## Gegenproben

Die erste synthetische Gegenprobe enthält einen gemeinsam freigegebenen
Akkord, den eine vorläufige Registertrennung teilt. Ein benachbarter,
eindeutiger Wiederholungsakkord liefert dieselbe Hand für alle Mitglieder.
In einer zweiten Gegenprobe belegt ein gehaltener Ton die Zielhand so,
dass die vollständige Gruppe nicht passt. In einer dritten Gegenprobe
haben die vermeintlichen Akkordmitglieder unterschiedliche Freigaben
oder widersprüchliche Wiederholungsanker. Eine unabhängig transponierte
und zeitlich verschobene Kopie muss dieselbe lokale Entscheidung ergeben.
