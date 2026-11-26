# Reparaturbereich für Halsrichtungen in Haltebogenketten

Dieser Dokumentations-Take legt den Reparaturbereich für die erste Anwendung der [SchlopfKopfFormel](MATHEMATIK.md#kapitel-2-1a) fest. Grundlage ist die bestehende [Fehlerklasse für falsche Halsrichtungen](selector_derivation_contract_final_stem_error_class.md). Sie beurteilt die Halsrichtung jeder vollständigen rhythmischen Einheit. Eine Haltebogenkette bestimmt nur, wann mehrere solcher Einheiten gemeinsam sicher geändert werden können.

## Kontext und Entscheidung

Eine Einheit ist ein Einzelanschlag, ein Akkord mit gemeinsamem Hals oder ein vollständiger Primärbalken mit seinen zugehörigen Köpfen. Eine Gruppe enthält alle Einheiten, die über nachgewiesene Haltebogenanschlüsse oder gemeinsame rhythmische Satzpflichten miteinander verbunden sind. Für die Halsentscheidung liest der Planer den für diese Einheiten nötigen gedruckten Notationskontext. Eine Taktgrenze begrenzt die Gruppe ebenso wenig, wie sie eine Änderung auslöst.

Der Nachweis bindet die geschriebenen Haltebogenenden über ihre Quell- und Fragmentprovenienz. Übereinstimmungen in Tonhöhe, Einsatz, Stimme oder Lage im Satz können die Bindung prüfen. Fehlt die Provenienz oder ist die Fortsetzung mehrdeutig, bleibt die Gruppe bei ihrer bisherigen Fassung.

Der erste Reparaturbereich verwendet die bestehende Regel für isolierte Einheiten. Die belegte Lage der sichtbaren Köpfe zur Mittellinie im geltenden Schlüssel und Oktavzustand bestimmt die Richtung unabhängig von den vorhandenen Hälsen. Eine Gruppe kommt zur Änderung infrage, wenn

- alle beteiligten rhythmischen Einheiten und ihre Haltebogenanschlüsse vollständig belegt sind
- jede Einheit aus ihrem eigenen Kontext eindeutig dieselbe Richtung verlangt
- mindestens ein sichtbarer, gewöhnlicher und expliziter Hals dieser Richtung widerspricht

Der bestehende finale Halsplaner wählt dafür genau einen Vorschlag. Er richtet die abweichenden Hälse gemeinsam aus. Eine unbestimmte oder widersprüchliche Richtung, ein ungebundener Balken oder eine konkurrierende Stimmrolle führt zur Ausgangsfassung. Der Kontext enthält weder Stückkennungen oder gedruckte Taktnummern noch Notenmuster, Merkmalskombinationen oder Grenzwerte, die eine bestimmte Fundstelle wiedererkennen.

## Erhalt und geometrische Freigabe

Der bestehende Notensatz-Owner bindet seinen Erhaltungsbeleg an den exakten Vor- und Nachzustand der übernommenen Fassung. Die Änderung erhält Quelldaten und geschriebene Tonhöhen, Einsatz und Dauer, Stimme und Notensystem, Akkordzugehörigkeit, Balkenverbindungen und Balkenstufen sowie Haltebogenenden und ihre Verbindungen. Eine Änderung der grafischen Hals-, Balken- oder Bogenseite durch den Renderer gehört in den geometrischen Vergleich.

Der geometrische Nachweis vergleicht die Ausgangsfassung mit einem gemeinsamen Vorschlag. Er bindet die geänderten Zeichen über beide Renderings und prüft auch ihre relevanten Paare mit Zeichen außerhalb der Gruppe. Die dafür nötige Koordinatentoleranz und die Mindestabstände je Paarklasse werden vor der Auswertung festgelegt. Ein möglicherweise betroffenes Paar mit ungeklärter Zuordnung oder ungeklärter Nichtverschlechterung verweigert die Übernahme für die betroffene Gruppe.

Gekoppelte Gruppen werden gemeinsam entschieden. Eine getrennte Übernahme setzt belegte Unabhängigkeit der betroffenen Befunde voraus. Der reguläre PDF-Render bestätigt die tatsächlich gewählte Fassung einschließlich einer zulässigen Teilfassung. Eine fehlende Bestätigung führt zu keiner weiteren Auswahl. Als Rückfall dient eine für die Ausgabe belegte Ausgangsfassung.

## Synthetische Prüfungen vor produktiver Übernahme

| Probe | Erwartung |
| --- | --- |
| Gleiche Ursache in verschieden gebauten Situationen, mit geänderten Bezeichnungen und anderer Lage im Material | Entsprechende Halsentscheidung. Eine Kette über eine Taktgrenze und eine fachlich entsprechende Kette ohne Taktwechsel werden bei gleichem Notationskontext gleich behandelt. |
| Minimales Paar mit bereits richtigen Hälsen | Ausgangsfassung. |
| Über und unter der Mittellinie sowie mit widersprüchlichem oder nicht belegtem Kontext | Entsprechende Richtung bei eindeutigem Kontext. Sonst Ausgangsfassung. |
| Fehlende oder mehrdeutige Verbindung eines Haltebogenendes | Ausgangsfassung. |
| Hinzugefügte unabhängige Kette, auch wenn deren Prüfung scheitert | Dieselbe Entscheidung für die ursprüngliche Kette, sofern die Unabhängigkeit belegt ist. |
| Neue oder verschärfte Kollision zwischen Hals und Kopf oder Hals und Bogen, auch mit einem Zeichen außerhalb der Gruppe | Verweigerung für die betroffene Gruppe. |
| Geänderte Balken- oder Bogenseite bei erhaltener Verbindung | Übernahme nur bei belegtem Erhalt und bestandener Geometrieprüfung. |
| Geprüfter Vorschlag und später abweichendes PDF | Keine Freigabe dieser Fassung, keine neue Auswahl. |

Die Proben verwenden ausschließlich synthetisches Material. Die geometrische Qualifikation folgt als eigener Take. Eine Messung am Korpus beginnt erst nach Festlegung von Fehlerklasse und Reparaturbereich. Ihr Ergebnis darf die Regel weder auf einzelne Fundstellen zuschneiden noch eine Schreibbefugnis begründen.

## Bindung für den Implementierungs-Take

Der spätere Selector-Vertrag verwendet das [Stage-C-Schema](selector_derivation_contract_template.md#stufe-c--repair-scope-falls-spaeter-ueberhaupt-gerechtfertigt) mit der Rolle `repair_scope` und dem bestehenden Fehlerklassen-Parent `docs/selector_derivation_contract_final_stem_error_class.md`. Das Feld `ScopeNarrowingBasis` erhält `authority_or_safety_only`. Die Einschränkungen dienen der Zuständigkeit und dem sicheren Erhalt der Notation. Die unabhängige Duplikationsprobe nach `candidate_duplication_v1` ist vor der produktiven Übernahme erforderlich.

