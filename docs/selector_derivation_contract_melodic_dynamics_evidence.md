# Dynamikverlauf einer Melodielinie

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
CausalFeature: a relative dynamics mark requires time-ordered evidence for the same sounding attacks of a bound musical line
CausalFeature: PCM evidence requires a verified mapping between source MIDI performance time and original audio frames
CausalFeature: isolated accents, accompaniment transients and ambiguous cross-source correspondence can mimic a rise
NonCausalContextExcluded: work identity, filename, measure address, absolute pitch, page, written voice labels and relative position in the piece

## Entscheidung am Expression-Owner

Der Expression-Owner beurteilt den Verlauf anhand gebundener
Melodieanschläge. Die MIDI-Anschlagstärke gehört zum ursprünglichen
Note-on-Ereignis. Die WAV-Beobachtung benötigt eine überprüfte Zuordnung
von MIDI-Spielzeit zu WAV-Frames. Die Zuordnung muss auch benachbarte
Anschläge tragen. Ein Versatz, der nur an den Endpunkten passt, genügt dafür
nicht. Die Messung am Melodieton muss seine Begleitung und ausklingende
Töne als mögliche Störquellen berücksichtigen.

Ein Crescendo setzt Unterstützung für eine Entwicklung über mehrere
Anschläge voraus. Ein lauter Zielton allein kann einen Akzent belegen.
Bei nicht auflösbarer Mehrdeutigkeit enthält sich der Owner der Ausgabe.
Ein belegter Verlauf ergibt einen gebundenen Anfang und ein gebundenes Ende.
Das Layout trägt die Endpunkte über einen Systemumbruch. Die gedruckte
Stufe am Beginn ist eine eigenständige Entscheidung.

## Synthetische Gegenproben

- Ein relativer Anstieg bleibt bei gemeinsamem Zeitversatz und konstanter
  Lautstärkeskalierung erkennbar.
- Eine einzelne laute Zielnote und eine lauter werdende Begleitung bei
  gleichbleibender Melodie erzeugen keinen Crescendo-Keil.
- Eine konkurrierende zeitliche Zuordnung oder ein verdeckter Melodieton
  lässt den Owner die Freigabe verweigern.
- Zwei unabhängige Wiederholungen behalten ihre jeweils gebundenen
  Endpunkte. Das Versetzen eines Systemumbruchs erhält dieselben
  Anschlagsidentitäten.

Dieser Take definiert die Fehlerklasse und die Gegenproben.
