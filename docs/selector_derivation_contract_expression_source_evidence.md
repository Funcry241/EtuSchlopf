# Ausdruckszeichen ohne Quellenbeleg

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
CausalFeature: the program emits a categorical dynamic or performance instruction without a source marking or independent musical evidence for that instruction
CausalFeature: relative position in a score supplies neither the class of dynamic nor the event or interval to which it applies
CausalFeature: inferred phrase spans need uniquely bound attack endpoints and evidence at the intended grouping level
NonCausalContextExcluded: work identity, filename, absolute measure, pitch, page, system, and relative location as a substitute for musical evidence

## Dynamik und Spielanweisungen

Eine gedruckte Anweisung behauptet eine Spielweise, ein Dynamikzeichen eine
dynamische Stufe. Der vorhandene Expression-Owner muss für jede solche Ausgabe
einen Beleg haben. MIDI-Anschlagstärken und die Hüllkurve einer Aufnahme können
relative Verläufe stützen. Die Zuordnung zu gedruckten Stufen wie p und mf
erfordert zusätzliche Evidenz. Die Position im Stück trägt dazu nichts bei.
Eine belegte Tempomessung ist eine eigene Aussage mit eigenem Prüfvertrag.

## Phrasierung

Ein Phrasierungsbogen verbindet musikalisch begründete und eindeutig
zugeordnete Anschläge. Ein gebundener Fortsetzungskopf ist kein neuer
Anschlag. Bei einem Akkord muss die führende Note als eigener Endpunkt
belegt sein. Der bestehende Phrasierungs-Owner trägt diese Bindung bis
ins Render-MusicXML. Eine Ausgabe erhält nur Bögen mit belegter Relation
und vollständigen Endpunkten. Der Tonhöhen- und Anschlagsbestand bleibt
prüfbar.

## Gegenproben

- Fehlender Beleg bei gleicher Musik und abweichender Stücklänge erzeugt
  keine Spielanweisung und keine Dynamik aus einer prozentualen Position.
- Vorhandene belegte Ausdruckszeichen und der bestehende Tempoausdruck
  bleiben bei einer Änderung am Expression-Owner erhalten.
- Eine Auftaktphrase mit belegtem melodischem Endpunkt in einem Akkord
  kann von einer mehrdeutigen Bindung und von einem bloßen Haltebogen
  unterschieden werden.
- Eine synthetisch verdoppelte Phrase behält ihre lokalen Entscheidungen.
  Bogenanfang und Bogenende sind im exportierten MusicXML gebunden.
- Die Chopin-Vorlage dient dem visuellen Abgleich. Die produktiven Eingaben
  sind MIDI und WAV. Der Abgleich vergleicht Auftakt, ersten Volltakt und
  Übergang zur nächsten Phrase sowie die sichtbare Dynamik und die Bögen.

Dieser Take definiert die Fehlerklasse und die Gegenproben.
