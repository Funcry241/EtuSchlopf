# Reparaturbereich für Halsrichtungen in Haltebogenketten

Dieser Dokumentations-Take legt den Reparaturbereich für die erste Anwendung der [SchlopfKopfFormel](MATHEMATIK.md#kapitel-2-1a) fest. Grundlage ist die bestehende [Fehlerklasse für falsche Halsrichtungen](selector_derivation_contract_final_stem_error_class.md). Sie beurteilt die Halsrichtung jeder vollständigen rhythmischen Einheit. Eine Haltebogenkette bestimmt nur, wann mehrere solcher Einheiten gemeinsam sicher geändert werden können.

## Kontext und Entscheidung

Eine Einheit ist ein Einzelanschlag, ein Akkord mit gemeinsamem Hals oder ein vollständiger Primärbalken mit seinen zugehörigen Köpfen. Eine Gruppe enthält alle Einheiten, die über nachgewiesene Haltebogenanschlüsse oder gemeinsame rhythmische Satzpflichten miteinander verbunden sind. Für die Halsentscheidung liest der Planer den für diese Einheiten nötigen gedruckten Notationskontext. Eine Taktgrenze begrenzt die Gruppe ebenso wenig, wie sie eine Änderung auslöst.

Der Nachweis bindet die geschriebenen Haltebogenenden über ihre Quell- und Fragmentprovenienz. Übereinstimmungen in Tonhöhe, Einsatz, Stimme oder Lage im Satz können die Bindung prüfen. Fehlt die Provenienz oder ist die Fortsetzung mehrdeutig, bleibt die Gruppe bei ihrer bisherigen Fassung.

Der erste Reparaturbereich verwendet die bestehende Regel für isolierte Einheiten. Die belegte Lage der sichtbaren Köpfe zur Mittellinie im geltenden Schlüssel und Oktavzustand bestimmt die Richtung unabhängig von den vorhandenen Hälsen. Eine Gruppe kommt zur Änderung infrage, wenn

- alle beteiligten rhythmischen Einheiten und ihre Haltebogenanschlüsse vollständig belegt sind
- jede Einheit aus ihrem eigenen Kontext eindeutig dieselbe Richtung verlangt
- mindestens ein sichtbarer, gewöhnlicher und expliziter Hals dieser Richtung widerspricht

Der bestehende finale Halsplaner ist für die Übernahme zuständig. Der Reparaturbereich lässt je Gruppe höchstens einen Vorschlag zu. Dieser richtet die abweichenden Hälse gemeinsam aus. Eine unbestimmte oder widersprüchliche Richtung, ein ungebundener Balken oder eine konkurrierende Stimmrolle führt zur Ausgangsfassung. Der Kontext enthält weder Stückkennungen oder gedruckte Taktnummern noch Notenmuster, Merkmalskombinationen oder Grenzwerte, die eine bestimmte Fundstelle wiedererkennen. Die Halsregel verlangt die Nichtverschlechterung jedes betroffenen harten Befunds für sich und sieht keine weichen Abwägungen vor.

## Erhalt und geometrische Freigabe

Der bestehende Notensatz-Owner bindet seinen Erhaltungsbeleg an den exakten Vor- und Nachzustand der übernommenen Fassung. Die Änderung erhält Quelldaten und geschriebene Tonhöhen, Einsatz und Dauer, Stimme und Notensystem, Akkordzugehörigkeit, Balkenverbindungen und Balkenstufen sowie Haltebogenenden und ihre Verbindungen. Eine Änderung der grafischen Hals-, Balken- oder Bogenseite durch den Renderer gehört in den geometrischen Vergleich.

Der geometrische Nachweis vergleicht die Ausgangsfassung mit einem gemeinsamen Vorschlag. Vollständig belegte Inventare beider Fassungen erfassen auch ungeplant verschobene, geteilte, hinzugekommene und entfallene Zeichen. Die Prüfung umfasst relevante Paare mit Zeichen außerhalb der Gruppe auf Ursprungs- und Zielseiten. Für jedes ausgelassene Kandidatenpaar braucht es in allen relevanten Fassungen einen belegten Ausschluss. Fehlt für ein möglicherweise betroffenes Paar die Zuordnung oder der Erhaltungsnachweis, bleibt die betroffene Gruppe bei der Ausgangsfassung. [Vollständigkeit der Paarermittlung](MATHEMATIK.md#vollstaendigkeit-paarermittlung)

Vor der produktiven Übernahme werden Hals–Kopf und Hals–Haltebogen als Rollenpaare mit jeweils musikalisch begründetem Mindestabstand definiert. Ein belegter eigener Halsansatz erlaubt den fachlich vorgesehenen Kontakt am eigenen Kopf. Weitere Kontakte werden gemäß ihrer Rolle geprüft. Die Messreserve und der Maßstab werden vor der Auswertung festgelegt. Ein bereits belegter Verstoß darf bei nachgewiesener Nichtverschlechterung erhalten bleiben. [Abstandserhalt](MATHEMATIK.md#abstandserhalt) · [Bestehende Klasse sichtbarer Überdeckungen](selector_derivation_contract_visible_engraving_collision.md)

Gekoppelte Gruppen werden gemeinsam entschieden. Die Kopplung ergibt sich aus den Planungsabhängigkeiten und den möglichen Auswirkungen auf relevante Zeichen und Befunde. Eine getrennte Übernahme setzt belegte Unabhängigkeit auch beim Hinzunehmen oder Weglassen anderer Gruppen voraus. Bei ungeklärter Wechselwirkung werden die Gruppen gemeinsam geprüft.

Der Schlussbeleg prüft die tatsächlich ausgegebene PDF-Fassung einschließlich einer zulässigen Teilfassung. Er bestätigt die beabsichtigte Richtung jedes geänderten Halses. Mindestens ein Zielhals muss in der Ausgangsfassung nachweislich falsch ausgerichtet gewesen sein. Fehlt die Bestätigung, dient die für die Ausgabe belegte Ausgangsfassung als Rückfall. Damit endet die Auswahl.

## Synthetische Prüfungen vor produktiver Übernahme

| Probe | Erwartung |
| --- | --- |
| Gleiche Ursache in verschieden gebauten Situationen, mit geänderten Bezeichnungen und anderer Lage im Material | Entsprechende Halsentscheidung. Eine Kette über eine Taktgrenze und eine fachlich entsprechende Kette ohne Taktwechsel werden bei gleichem Notationskontext gleich behandelt. |
| Minimales Paar mit bereits richtigen Hälsen | Ausgangsfassung. |
| Über und unter der Mittellinie sowie mit widersprüchlichem oder nicht belegtem Kontext | Entsprechende Richtung bei eindeutigem Kontext. Sonst Ausgangsfassung. |
| Fehlende oder mehrdeutige Verbindung eines Haltebogenendes | Ausgangsfassung. |
| Hinzugefügte unabhängige Kette, auch wenn deren Prüfung scheitert | Dieselbe Entscheidung für die ursprüngliche Kette, sofern die Unabhängigkeit belegt ist. |
| Gegenläufige Verschiebungen, die sich im Gesamtvorschlag aufheben | Gemeinsame Prüfung, sofern die Wirkung der Teilfassungen keine Unabhängigkeit belegt. |
| Nachweislich leere Paarliste und Gegenfall mit einem in beiden Listen ausgelassenen Nachbarn, auch auf einer Zielseite | Die nachweislich leere Paarliste besteht die Vollständigkeitsprüfung. Das ausgelassene relevante Paar verweigert die Übernahme. |
| Neue oder verschärfte Kollision zwischen Hals und Kopf oder Hals und Bogen, auch mit einem Zeichen außerhalb der Gruppe | Verweigerung für die betroffene Gruppe. |
| Geänderte Balken- oder Bogenseite bei erhaltener Verbindung | Übernahme nur bei belegtem Erhalt und bestandener Geometrieprüfung. |
| Zweiter geänderter Hals fehlt im Schlussbeleg oder zeigt weiterhin in die falsche Richtung | Rückfall auf die belegte Ausgangsfassung. |
| Geprüfter Vorschlag und später abweichendes PDF | Keine Freigabe dieser Fassung, keine neue Auswahl. |
| Zweiter Lauf auf der übernommenen Teilfassung bei gleichem Kontext und Renderprofil, auch mit zuvor abgewiesener Gruppe | Gleicher gebundener Notationszustand und gleiche normalisierte Rendergeometrie. Die bereits angenommene Kette löst keinen weiteren Korrekturvorschlag aus. |

Die Proben verwenden ausschließlich synthetisches Material. r2014 belegt Teile der PDF-Zeichenbindung und der Übergangsentscheidung im synthetischen Modell. Die Zeilen beschreiben die Anforderungen an die produktive Freigabe. Eine Messung am Korpus beginnt erst nach Festlegung von Fehlerklasse und Reparaturbereich. Ihr Ergebnis darf die Regel weder auf einzelne Fundstellen zuschneiden noch eine Schreibbefugnis begründen. [PDF-Nachweis und Trockenlauf r2014](final_stem_pdf_evidence_r2014.md)

## Bindung für den Implementierungs-Take

Der spätere Selector-Vertrag verwendet das [Stage-C-Schema](selector_derivation_contract_template.md#stufe-c--repair-scope-falls-spaeter-ueberhaupt-gerechtfertigt) mit der Rolle `repair_scope` und dem bestehenden Fehlerklassen-Parent `docs/selector_derivation_contract_final_stem_error_class.md`. Das Feld `ScopeNarrowingBasis` erhält `authority_or_safety_only`. Die Einschränkungen dienen der Zuständigkeit und dem sicheren Erhalt der Notation. Die unabhängige Duplikationsprobe nach `candidate_duplication_v1` ist vor der produktiven Übernahme erforderlich.

