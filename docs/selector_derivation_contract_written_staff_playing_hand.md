# Spielhand und Notensystem bei selbstständigen Begleitstimmen

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
CausalFeature: a source-bound pitched occurrence has distinct playing-hand and written-staff obligations
CausalFeature: a staff-based playability check treats the written staff as the playing hand and attributes simultaneous or still sounding keys to the wrong hand
CausalFeature: a written accompaniment course needs its own complete chord attachments, independent voice and continuous metric coverage
NonCausalContextExcluded: piece identity, absolute measure labels, specific pitch values, raw voice labels, candidate count and renderer exit code

Ein Klaviersystem trägt die Notendarstellung. Die Prüfung der Handspanne
verwendet die gebundene Spielhand und berücksichtigt bereits klingende Tasten.
Fallen Spielhand und Notensystem auseinander, kann eine nach Systemen gruppierte
Prüfung eine unspielbare Handspanne melden, obwohl die Spielhände den Abschnitt
ausführen können. Umgekehrt kann sie eine Überschreitung innerhalb einer Spielhand
übersehen, wenn deren Noten auf beide Systeme verteilt sind.

Die Darstellung einer Begleitstimme erhält ihre vollständigen Akkorde, ihre
metrische Abdeckung und die eigenen Balken. Die Darstellung benötigt einen
eigenen kollisionsfreien Stimmverlauf. Einsätze, Tonhöhen, Dauern, Haltebögen
und gebundene Spielhände behalten ihre Quellzuordnung.

Eine synthetische Gegenprobe vergleicht die Handprüfung vor und nach der
Darstellung derselben Begleitstimme im anderen System. Eine zweite Gegenprobe
enthält eine tatsächlich zu große Spanne in derselben Spielhand. Eine dritte
enthält eine Kollision mit einer bereits vorhandenen Zielstimme. Eine
unabhängige Wiederholung der synthetischen Fälle prüft die Lokalität der Regel.
