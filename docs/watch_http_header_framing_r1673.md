# r1673 – Vollständige und begrenzte Watch-HTTP-Header

ChangeClass: RefactorOnly. Parent ist der nativ angenommene r1671 / 0.10.1839,
Take-SHA256 `3705cb0fb87f54ec837700f74f1b5b3d6691c2516c33ff5e79db43d453522abb`.
Der Export `assistant_export_latest(20261108-011544).zip` bindet den überprüften
Annahmebeleg und vollständigen Source-Snapshot. Alle verwalteten Dateien sind
bytegleich mit der bisher verwendeten Parent-Basis; Root-Cargo.lock bleibt
lokaler Zustand und gehört nicht in das eingehende Take-Paket.

Dieser Take ersetzt r1672 mit neuer eindeutiger Revision. Produktivcode und
Tests des Watch-Fixes bleiben gegenüber dem lokal qualifizierten r1672
bytegleich. Angepasst sind Take-Identität, Basisnachweis und Dokumentation
samt gebundenem Änderungsmanifest.

Der angenommene r1671-Korpus ist formal grün: 2.983 Tests bestanden, 4 ignoriert;
Build-, Clippy- und Launcher-Test-Exit jeweils 0; zehn gültige Projekte. Alle
20 MusicXML-Dateien sind gegenüber dem vorherigen r1669-Export bytegleich.
Die beobachtete Korpuszeit beträgt 12:56 Minuten.

## Eigenständig gewählter Befund

Der lokale HTTP-Leser gab bei Verbindungsende auch einen unvollständigen Header
an die Routenverarbeitung weiter. Schon eine vollständige GET-Zeile mit
nachfolgender, aber nicht abgeschlossener Headersektion konnte dadurch `/state`
ausführen. Umgekehrt lehnte er einen vollständig abgeschlossenen Header mit
genau 8.192 Bytes als zu groß ab. Seine Größenentscheidung konnte außerdem von
zusätzlich im selben TCP-Read angekommenen Bytes nach dem Headerende abhängen.

Die Änderung macht den vorhandenen Headerabschluss zur Voraussetzung der
Verarbeitung. Sie führt keine neue Route ein und erweitert keine Dateifreigabe.

## Änderung

`read_http_request` nimmt weiterhin Bytes bis zum ersten `CRLF CRLF` entgegen.
Ein vollständiger Header bis einschließlich der bestehenden 8-KiB-Grenze wird
zurückgegeben. Bytes hinter dem Headerabschluss gehören nicht zur zurückgegebenen
Headersektion. Jeder Read wird auf das verbleibende Bytebudget begrenzt; der
Sammelpuffer überschreitet die Grenze nicht.

Ohne Abschluss führt Verbindungsende zu `UnexpectedEof`; ein ausgeschöpftes
Bytebudget ohne Abschluss zu `InvalidData`. Diese Fehler verlassen den
HTTP-Clienthandler vor dem Aufruf einer Route. Der vorhandene Clientthread
schließt die Verbindung. `Interrupted` wird erneut gelesen; andere Lesefehler
bleiben unverändert erhalten. TCP-Lese-/Schreibtimeouts, GET-Pfadprüfung und
Antworten auf vollständige Anfragen behalten ihre bisherigen Zuständigkeiten.

Der private Leser verwendet die vorhandene `Read`-Schnittstelle. Produktiv
übergibt der Handler weiterhin seinen `TcpStream`. Deterministische Testleser
können dadurch dieselbe produktive Funktion mit beliebigen Chunkgrenzen und
gezielt ausgelösten Lesefehlern prüfen. Es gibt keine zusätzliche Abhängigkeit.

## Beleg

Eine neue Probe verwendet einen echten Loopback-Listener und den produktiven
`serve_http_client`. Sie sendet einen begonnenen `/state`-Header und schließt nur
die Schreibrichtung des Clients. Am unveränderten Parent scheitert die Probe,
weil der Handler den unvollständigen Header bereits erfolgreich verarbeitet.
Mit dieser Änderung liefert der Handler `UnexpectedEof` und keine Routenantwort.

Die acht neuen Tests prüfen:

- Keine Verarbeitung des abgebrochenen Headers über einen echten TCP-Client.
- Erfolgreiche echte `/state`-Antworten für vollständige Header mit 8.191 und
  8.192 Bytes.
- Sämtliche Zweiteilungen eines Headers, einschließlich geteilter Abschlussbytes.
- 8.191, 8.192 und 8.193 Bytes mit fünf unterschiedlichen Read-Chunkgrößen.
- Verbindungsende an jedem unvollständigen Präfix eines Headers.
- Nicht zum Header gehörende nachfolgende Bytes bei beinahe ausgeschöpftem Budget.
- Wiederholung eines einmalig unterbrochenen Reads.
- Unveränderte Weitergabe von Timeout- und Verbindungsfehlern.

Alle Daten sind synthetisch. Die Probe benötigt weder MIDI/WAV noch MQTT oder
einen Renderer. Die vollständige Watch-Testsuite umfasst damit 38 Rust-Tests.
Die bestehende Browserseite, ihre Tastaturbedienung und sämtliche Musikquellen
bleiben gegenüber r1671 bytegleich.

## Lokale Qualifikation

38 Watch-Tests bestanden, keine Fehler oder ignorierten Tests. Strenges
Watch-Clippy für alle Targets mit `-D warnings`, Strict-Produktionsbuild und
statische Take-Prüfung gegen r1671 sind grün. Die native Annahme von r1673
bleibt ein separater Nachweis.
