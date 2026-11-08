# r1681 – Wahrheitsgemäßer Lebenszyklus der Watch-Verbindung

ChangeClass: RefactorOnly. Parent ist der nativ angenommene r1680 / 0.10.1839,
Take-SHA256 `b0524969d7e0824f8d867ca16a28129ec21fba8d4255bfd7b2d3382ec46a5b9e`.
Der Export `assistant_export_latest(20261108-035922).zip` bindet den geprüften
Annahmebeleg und den vollständigen Snapshot. Alle 2.061 verwalteten Dateien
entsprechen dem ausgelieferten r1680; Root-Cargo.lock bleibt lokaler Zustand.
2.989 native Tests bestanden, vier ignoriert; zehn Projekte, Build und Clippy
sind grün. Alle 20 MusicXML-Dateien sind gegenüber r1676 bytegleich.

Die Paketprüfung verlangt eine vorher eingefrorene kausale Fehlerklasse für
Änderungen am PDF-Zugriff. Deshalb steht dieser Verbindungs-Take zuerst und
friert zusätzlich die Definition
`selector_derivation_contract_watch_pdf_resource_identity.md` ohne die
PDF-Implementierung ein. Diese folgt separat in r1682.

## Eigenständig gewählter Verbindungs-Take

Die Oberfläche leitete „CLOSED“ aus einer Zusammenfassung der Projektzustände
ab. Sobald ein Projekt fehlgeschlagen war, konnte sie die Verbindung als
geschlossen darstellen, obwohl andere Projekte weiterliefen und der Browser
bereits neu verband. Auch ein leerer Snapshot wurde so fehlinterpretiert.
Die Vorprobe mit echtem HTTP/Ereignisstrom zeigte „CLOSED“ bei EventSource-
Zustand CONNECTING; die zweite tatsächliche Verbindung gelang anschließend.

Bei einem Verbindungsfehler verwendet die Anzeige jetzt den tatsächlichen
EventSource-Zustand: geschlossen oder wieder verbindend. Ein erneut empfangener
Snapshot aktualisiert die Verbindungsanzeige auch bei unverändertem fachlichem
Zustand. Der darin gemeldete MQTT-Zustand bleibt während einer offenen
Browserverbindung maßgeblich für die Verfügbarkeit der Laufdaten. Die Anzeige
heißt deshalb „Live-Verbindung“ und behauptet nicht, jeder Browserfehler sei
ein MQTT-Abbruch.

Das Ende des gesamten Launcher-Laufs kommt ausschließlich aus dessen eigener
terminaler Meldung: completed, failed oder cancelled. Die Oberfläche übernimmt
den gemeldeten Zustand einschließlich Abschlussfarbe auch bei noch laufend
gemeldeten Kindquellen und schließt den
EventSource tatsächlich. Ab diesem Zeitpunkt wächst auch die geschätzte
Laufdauer einer noch laufend gemeldeten Kindquelle nicht weiter. Ein fertiges Einzelprojekt, eine laufende Launcher-
Finalisierung, ein leerer Snapshot oder verstrichene Zeit beendet ihn nicht.
Ohne Launcher-Abschluss wird eine unterbrochene Verbindung weiter beobachtet.
Eine endgültig geschlossene Verbindung erfindet kein abgeschlossenes Projekt.
Ein leerer aktueller Snapshot zeigt WAITING statt eines angenommenen Abschlusses.

Das Schließen des Ereignisstroms bricht keine laufenden PDF-Abrufe ab und
verwirft keine geladenen PDFs. Der folgende PDF-Take arbeitet unabhängig von
dieser Verbindungsanzeige. Sein frühes Vorladen bleibt wichtig, weil der
Launcher den Watch-Prozess kurz nach seiner eigenen Abschlussmeldung beendet.

## Beleg

`tools/etuschlopf_watch/tests/web_connection.cjs` verwendet das eingebettete
Produktions-HTML, einen echten lokalen HTTP/SSE-Server und den nativen
EventSource des Browsers. Sieben Szenarien prüfen:

- Fehlgeschlagenes und weiterlaufendes Projekt mit echter zweiter Verbindung.
- Leeren Zwischenstand mit WAITING und Wiederverbindung.
- Gemeldeten MQTT-Abbruch und Wiederkehr bei offenem Browserstrom.
- Projektabschluss vor der Launcher-Finalisierung; Launcher-Enden mit allen
  drei terminalen Zuständen und erhaltener Abschlussfarbe.
- PDF geladen, Launcher-Abschluss empfangen, tatsächlicher Server beendet,
  zuvor geladenes PDF weiterhin aus seiner Blob-URL lesbar.
- Abschluss des Ereignisstroms während eines noch laufenden PDF-Abrufs.
- Dauerhaft geschlossenen HTTP-Ereignisstrom ohne erfundenes Laufende.

Die vorhandene Titelprobe erhält einen vollständiger beschriebenen EventSource-
Testdouble mit `close()` und CLOSED; ihre bisherigen Status-/Farbprüfungen
bleiben erhalten. Die neue Netzwerkprobe scheitert am r1680 beim ersten
Wiederverbindungsfall. Es werden keine echten Korpusdateien oder Renderer
benötigt und keine Musik- oder Ergebnisautoritäten verändert.

Vor Auslieferung werden alle 38 Watch-Rust-Tests, sieben echten Netzwerkproben,
neun Fokus- und zehn Titelproben, strenges Watch-Clippy für alle
Targets, Strict-Build und die statische Take-Prüfung geprüft. Die Proben
verwenden lokal Chromium Headless Shell mit Playwright; sie ersetzen keine
native Abnahme auf dem Zielsystem. Die native Abnahme von r1681 bleibt offen.
