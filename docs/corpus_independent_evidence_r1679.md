# r1679 – Korpusunabhängige Evidenz und synthetische Tests

ChangeClass: KodexOnly. Parent ist der nativ angenommene r1676 / 0.10.1839,
Take-SHA256 `84c31d98978dc3a6294512c07385112a66fe74f88f2b7767e4ed315bb051fa1d`.
Der Export `assistant_export_latest(20261108-030840).zip` bindet den geprüften
Annahmebeleg und vollständigen Source-Snapshot. Alle 2.059 verwalteten Dateien
stimmen bytegenau mit dem ausgelieferten r1676 überein; Root-Cargo.lock bleibt
lokaler Zustand und wird nicht Teil des Take-Pakets.

2.985 native Tests bestanden, vier ignoriert; Build, Clippy und Launcher-Tests
Exit 0. Zehn Projekte sind formal grün. Die Konsole meldet 12:57 Minuten
Korpuslaufzeit; alle 20 MusicXML-Dateien sind gegenüber r1674 bytegleich.
Das ist ein beobachteter Vergleich, keine isolierte Performanceaussage.

Dieser Take ersetzt r1677 mit neuer eindeutiger Revision. Produktivcode und
Tests bleiben gegenüber dessen lokal qualifiziertem Stand unverändert.
Basisnachweis, Identität, Dokumentation und Änderungsmanifest sind aktualisiert.
Die native Annahme von r1679 bleibt bis zum nächsten Lauf separat offen.

## Gefundene operative Bindungen

Der Machine-Evidence-Korpusexport verlangte unabhängig vom aktuellen Manifest
genau die zehn historisch bekannten Projektnamen. Vorbereitung, Commit-Prüfung
und Aufräumplanung enthielten dieselbe feste Menge. Neue Publikationen beziehen
die nichtleere, eindeutige Projektmenge jetzt aus dem bereits hashgeprüften
Full-Corpus-Manifest. Erwartete Anzahl, Namen und Ergebnisbelege müssen exakt
übereinstimmen. Jedes erwartete Projekt benötigt weiterhin sein geschlossenes
Archiv und seine gebundenen Hash-/Provenienzbelege.

Neue Korpus-Commits verwenden Schema 2 mit explizitem `project-count`.
Leser prüfen die deklarierte Zahl gegen die tatsächlich gebundenen Mitglieder.
Schema 1 bleibt ausschließlich lesbar: dessen altes Format definierte zehn
Mitglieder ohne eigenes Anzahlfeld. Diese Formatkompatibilität ist keine
Auswahl der neuen Korpusgröße und enthält keine historischen Projektnamen.
Unbekannte Versionen, leere/mehrdeutige Zähler, fehlende Mitglieder und
inkonsistente Hashbindungen verweigern. Ein nicht validierbarer Commit stoppt
die Aufräumplanung, statt seine übrigen Archive als Einzeldateien freizugeben.

Die Root-Aufräumlogik behandelte beliebige PDF-Dateien mit einem Etüdenwort im
Namen als bekannte Programmausgaben. Diese namensabhängige Löschheuristik
entfällt. Die bestehenden generischen Ausgabe-/Vorschaumuster und der Schutz
der aktuell gebundenen Ausgaben bleiben erhalten. Ein Dateiname allein macht
ein privat abgelegtes Noten-PDF nicht zum Programmrest.

## Tests und verbleibende Schutzbeispiele

Allgemeine Inventur- und Cleanup-Tests verwenden synthetische Namen statt
konkreter Etüdennamen. Projektstamm, Watch und Sensor folgen im separaten r1680.
Die Cleanup-Probe verwendet einen synthetischen Korpus mit anderer Größe.
Ein überholter Visual-Audit-Text, der ein einzelnes Stück zur festen Referenz
erklärte, ist entfernt.

Das Tonal-Audit-Parserfixture wird aus relativen synthetischen Koordinaten
erzeugt. Eine Gegenprobe verschiebt die Koordinaten und verlangt unveränderte
Auditwerte. Die Tests binden keine bestimmte reale Taktposition.

Neue Proben prüfen Projektmengen mit einem, drei und elf Elementen unter
Umbenennung und Umsortierung sowie leere, doppelte, fehlende und fremde
Mitglieder. Echte lokale ZIP-Publikation und Retention-Readback prüfen dieselben
Größen, zusätzlich das alte Commit-Format. Beschädigte Anzahlbindungen stoppen
Cleanup und erhalten die vorhandenen Dateien.

Die Suche umfasste Rust-Produktivcode, Root- und Unit-Tests sowie die separaten
Rust-Werkzeuge. Es wurde dabei kein zusätzlicher produktiver Musikselektor
mit einer fest eingebauten realen Taktnummer nachgewiesen. Das ist kein
Vollständigkeitsbeweis gegen beliebig verschleierte Proxy-Identität.

Explizite verbotene Etüden-/Taktbeispiele in Scanner-Negativtests und
Schreibgrenzen bleiben erhalten: Sie müssen gerade nachweisen, dass solche
Auswahlregeln gesperrt werden. Ihre Entfernung würde den Schutz schwächen.
Historische Evaluationsberichte und reine Aufrufbeispiele sind keine operative
Auswahlregel und werden nicht als bereinigte Produktlogik ausgegeben.

## Qualifikation

Launcher-Tests, strenges Launcher-Clippy und die statische Take-Prüfung werden
lokal geprüft. Die native Korpusabnahme bleibt separat offen. Die musikalischen
Owner und Auswahlregeln sind unverändert.
