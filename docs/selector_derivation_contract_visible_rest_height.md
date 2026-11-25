# Vertikaler Anschluss kurzer sichtbarer Pausen an ihre Stimme

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
CausalFeature: Eine kurze sichtbare Pause liegt zeitlich zwischen unmittelbar anschließenden sichtbaren Einzelnoten derselben Stimme und desselben Systems.
CausalFeature: Die gezeichneten Notenköpfe tragen dieselbe vertikale Lage und liefern damit eine eindeutige lokale Referenzhöhe.
CausalFeature: Eine alternative gezeichnete Pausenlage nähert sich dieser Höhe an und erhält den nachgewiesenen Abstand zu den benachbarten Zeichen, die musikalischen Daten und das übrige Layout.
NonCausalContextExcluded: Werk, Dateiname, Takt- oder Seitennummer, absolute Tonhöhe, Stimmenbezeichnung und Anzahl der Fälle im Korpus.

Der vertikale Fehler ist ein Vergleich zwischen tatsächlich dargestellten Lagen.
Die Höhe eines einzelnen Zeichens reicht für die Feststellung ebenso wenig aus
wie aus MusicXML abgeleitete Abstände. Die Nachbarschaft bindet die Referenz an
die konkrete Stimme. Der Abstand zu benachbarten Zeichen begrenzt zulässige
Verbesserungen. Notenlinien dürfen die Pause kreuzen.

Ein späterer produktiver Versuch muss die Pause, beide Notenköpfe und das
Notensystem eindeutig im Render belegen. Bei gleichzeitig vorgesehenen Änderungen
muss der gemeinsame Render die Abstände einschließlich der übrigen Pausen
bestätigen. Mehrdeutige oder fehlende Belege erhalten die bisherige Darstellung.
Dieser Vertrag legt weder eine universelle Abstandsschwelle noch ein Verfahren
zum Durchsuchen möglicher Lagen fest.
