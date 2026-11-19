# Fortsetzung einer gehaltenen Oberstimme

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
ErrorClassName: uniquely_supported_held_voice_continuation_excluded_by_pitch_motion
CausalFeature: the MIDI notation-time owner already resolves an independently held upper note and distinct simultaneous accompaniment
CausalFeature: the held note has a bound release at the beginning of a uniquely assignable written continuation
CausalFeature: all source attacks in the local decision can be assigned without overlap within either voice and without changing any source attack, pitch, or release
CausalFeature: more than one feasible voice assignment leaves the previous assignment in effect
NonCausalContextExcluded: work identity, measure number, absolute pitches, fixed semitone thresholds, exact pattern counts, and corpus prevalence

## Regel

Der MIDI-Import führt eine bereits abgesicherte gehaltene Oberstimme innerhalb
des Takts ab ihrer belegten Loslasszeit weiter, wenn genau eine vollständige
Zuordnung zum bestehenden Stimmenverlauf passt. Die Zuordnung berücksichtigt
die belegten Einsatz- und Loslasszeiten beider Stimmen sowie metrisch
darstellbare Dauern. Eine Tonwiederholung ist ein eigener Anschlag und darf
wie ein Schritt oder ein Sprung Teil der Fortsetzung sein. Die Tonfolge
allein entscheidet keine Mehrdeutigkeit. Bei fehlendem Zeitnachweis oder
mehreren möglichen Zuordnungen gilt die bisherige Zuordnung.

Die Entscheidung liegt bei der bestehenden Stimmenzuordnung in
`src/midi_seed.rs` am Aufruf von `upper_sustain_continuation`. Der
Implementierungs-Take muss die Zuordnung prüfen, bevor er Quellanschläge
einer Stimme zuweist. Auch die Begleitung muss mit unveränderten Einsätzen,
Tonhöhen und Loslasszeiten darstellbar sein. Die aktuelle metrische Zuständigkeit
und der Pfad für taktübergreifend gebundene Noten gelten weiterhin.

## Synthetische Gegenproben für die Umsetzung

| Fall | Erwartung |
|---|---|
| Eindeutige Fortsetzung mit Tonschritt, Tonwiederholung oder Terzsprung | In allen drei Varianten tragen dieselben Quellanschlagspositionen dieselbe Stimmrolle. |
| Gleichzeitige Folgetöne mit unterschiedlichen Loslasszeiten | Die weitere zeitliche Belegung löst die Zuordnung auf oder der Import behält die bisherige Zuordnung bei. Der höchste Ton allein reicht als Nachweis nicht aus. |
| Zwei vollständige Zuordnungen oder keine Fortsetzung am belegten Tonende | Die bisherige Zuordnung bleibt in Kraft. |
| Verschobener Einsatz, transponierte Passage und unabhängige Wiederholung | Jede Wiederholung ergibt dieselbe Entscheidung in ihrer lokalen Zeitlage. |
| Ausgabe des positiven Falls als MusicXML | Alle Quellanschläge bleiben einzeln gebunden. Einsatz, Tonhöhe und Loslasszeit bleiben erhalten. |

Ein späterer Take ergänzt die ausführbaren Gegenproben am bestehenden
MIDI-Import und prüft den Windows-Vollkorpus vor der nächsten Referenz.
