# Register-Sichtschreibauftrag-Projektionsgeschlossenheitsbeweis r281

## Zweck

r281 basiert direkt auf dem real installierten und dunkelgruenen r280 `RegisterSichtschreibauftragVertragsbeweis` mit 986/986 Pruefungen. r280 hat den geplanten Sichtschreibauftrag fuer Etuede 6 bewusst **nicht** freigegeben: 18 Auftraege, 18 Startanker, aber nur 17 Stopanker und 17 exakte Dokumentbereiche; `Sichtschreibbereitschaft: gesperrt; Blocker 3`.

r281 lockert diesen Vertrag nicht. Stattdessen klassifiziert es die gemeinsame Ursache der drei technischen Symptome als eigene allgemeine Projektionsgeschlossenheit an Akkordgrenzen.

## Beobachtete Fehlerklasse

Eine sichtbare Oktavspanne darf nicht als staffweite MusicXML-`octave-shift`-Richtung freigegeben werden, wenn ihre semantische Grenze einen dokumentseitig zusammengehoerigen Akkordverbund nur teilweise schneiden wuerde. Der problematische Fall ist nicht ueber Partiturtitel oder gedruckte Taktkennung identifiziert, sondern ausschliesslich ueber:

- gleiche Teil-/Systemzuordnung,
- gewuenschte semantische Sichtgrenze,
- MusicXML-Dokumentcursor,
- Beginn und Ende der dokumentseitigen Tonankergruppe.

Ein `AkkordgrenzenProjektionskonflikt` liegt vor, wenn der Dokumentcursor nach der Tonankergruppe exakt auf der gewuenschten Sichtgrenze liegt, die Tonankergruppe selbst aufgrund eines gleichzeitig notierten Akkordmitglieds aber ueber diese Grenze hinausreicht.

## Beziehung zur bestehenden MusicXML-Konformitaetsevidenz

Der vorhandene Format-4.0-Audit besitzt bereits die allgemeine Invariante `MusicXmlChordPredecessorDuration`: Ein `<chord/>`-Ton darf nicht laenger als der vorangehende, den MusicXML-Cursor bewegende Hauptton dauern. r281 erfindet dafuer weder eine zweite Norm noch Reparaturrecht. Es projiziert lediglich, ob eine solche bereits vorhandene Quellstruktur konkret eine geplante sichtbare Oktavgrenze unrepraesentierbar macht.

Die Gesamtzahl der beobachteten Akkordgruppen mit Dauerueberhang wird getrennt von der Zahl der fuer den aktuellen Sichtschreibauftrag relevanten Grenzkonflikte berichtet. Eine Quellverletzung ausserhalb einer Sichtgrenze wird dadurch nicht automatisch zum Oktavierungsblocker.

## Keine Doppelzaehlung

r280 zaehlt beim selben Grenzproblem technisch drei Symptome: fehlender Stopanker, fehlender sichtbarer Stopgrenzton und Dokument-Scope-Konflikt. r281 behaelt diese strengen Einzelkennzahlen bei, addiert den Akkordgrenzenkonflikt aber **nicht** als vierten Blocker. Er ist eine Ursachenklassifikation, keine zusaetzliche Ablehnung.

## Architekturgrenze

r281 bleibt `AuditOnly`:

- keine MusicXML-Mutation,
- keine StructuralWriteAuthority,
- keine Aenderung des semantischen RegisterNotationPlan,
- keine Fusion oder Verschiebung einer Sichtspanne,
- keine automatische Korrektur von Akkorddauern,
- keine Takt-/Etueden-/Dateisonderlogik.

Ein spaeterer Schritt darf erst entscheiden, wie ein projektionsgesperrter Sichtauftrag allgemein behandelt wird. Insbesondere darf r281 **nicht** stillschweigend die semantische Spanne verlaengern, einen verdeckten Ton aus der Quellstruktur entfernen oder die MusicXML-Konformitaetsverletzung als Oktavierungsreparatur beseitigen.

## Erwartung Etuede 6

Aus dem realen r280-Lauf wird fuer den naechsten Einzelstueckbeweis erwartet:

- 18 Sichtschreibauftraege,
- 17 strenge Stopanker,
- 17 exakte Dokumentbereiche,
- 24 vorhandene Akkordgruppen mit Dauerueberhang im Master-Kontext,
- genau 1 fuer eine Sichtgrenze relevanter Akkordgrenzen-Projektionskonflikt,
- weiterhin gesperrte Sichtschreibbereitschaft,
- keine Aenderung an MusicXML oder PDF.

Zwei neue reine Audit-Pruefungen unterscheiden einen Akkordueberhang an der Sichtgrenze von einer projektionsgeschlossenen, konformen Akkordgrenze. Ausgehend von 986/986 r280-Pruefungen lautet die Erwartung 988/988.
