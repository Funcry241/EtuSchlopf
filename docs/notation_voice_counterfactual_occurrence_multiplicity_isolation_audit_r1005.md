# r1005 NotationVoiceCounterfactualOccurrenceMultiplicityIsolationAudit

r1005 baut direkt auf dem akzeptierten r1004 `NotationVoiceBeamOriginAndMaterializationCounterfactualReportContractClosure` auf. r1004 ist 10/10 formal gruen, audit-only und liefert den ersten vollstaendigen Beam-Origin-/Materialisierungs-Counterfactual-Bericht. Der Counterfactual selbst wird dort jedoch korpusweit vorzeitig durch `duplicate_occurrence_identity` blockiert.

Die Blockade ist keine Widerlegung der SAME-Line-Kandidaten. Der bestehende S0-Proof bindet jedes konkrete Beam-Member bereits eindeutig ueber `(measure, pitched_ordinal)` an eine physische Occurrence und von dort an `(SourceEventId, SoundingAttackId)`. Der r1004-Counterfactual verwirft diese gebundene Occurrence-Identitaet anschliessend und fordert projektweit faelschlich, dass jedes `(SourceEventId, SoundingAttackId)` genau eine physische Occurrence besitzen duerfe. Das ist fuer legitime 1->n-Repräsentationen wie materialisierte Tie-/Fragmentketten zu stark.

r1005 aendert deshalb ausschliesslich den read-only Counterfactual-Vertrag:

- die geschlossene SAME-Line-Komponente traegt ihre bereits bewiesenen Member-Locators `(measure, pitched_ordinal, represented onset/duration, SourceEventId, SoundingAttackId)` bis in den Counterfactual weiter;
- jede Komponente wird einzeln gebunden und simuliert;
- globale SourceAttack->Occurrence-Eindeutigkeit ist keine Voraussetzung mehr;
- besitzt ein Member-SourceAttack mehrere physische Occurrences, wird nur diese Komponente als `BLOCKED_INCOMPLETE_MULTI_OCCURRENCE_REPRESENTATION_BINDING` gesperrt;
- eindeutig gebundene Komponenten werden einzeln gegen den bestehenden `FrozenSerializationVoiceChannelPlan` simuliert und auf Zielrelation sowie externe Channel-/Role-Erhaltung geprueft;
- nur lokal gruene und source-disjunkte Komponenten werden zusaetzlich gemeinsam in einem Batch-Counterfactual geprueft;
- kein lokaler Multiplicity-Fall darf den gesamten Projekt-Audit abbrechen.

Die Transformation bleibt rein hypothetisch und in-memory: Fuer die jeweils simulierte Komponente wird nur im Shadow die technische Lane fuer die Presentation-Entscheidung neutralisiert und das akustische Intervall durch das bereits gebundene repraesentierte Intervall ersetzt. SourceEventId, SoundingAttackId, Staff, Pitch sowie die semantischen Source-Zeilen bleiben unveraendert. Es wird weder MusicXML noch PDF noch Musik geschrieben.

Die Beam-Historical-Owner-Provenienz bleibt weiterhin offen. Ein gruener Counterfactual ist kein Writer-Recht und keine allgemeine Korrektheitsaussage. Der visuelle Katalog bleibt ausschliesslich Abnahme-Oracle und ist kein Selektor.

Gegenueber r1004 kommt eine Regression hinzu, die eine legitime zweite physische Occurrence desselben SourceAttack einspeist und verlangt, dass nur der betroffene Kandidat lokal gesperrt wird, ohne globales `CounterfactualInputFailures` und ohne Projektabbruch. Erwarteter Strict-Testbestand: 1314.
