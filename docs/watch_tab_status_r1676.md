# r1676 – Laufstatus und Korpusfortschritt im Browser-Tab

ChangeClass: RefactorOnly. Parent ist der nativ angenommene r1674 / 0.10.1839,
Take-SHA256 `a364d57a69a58639be59aaf3bb9491bc03a032768d3218093b23218e89c16a29`.
Der Export `assistant_export_latest(20261108-023748).zip` bindet den geprüften
Annahmebeleg und vollständigen Source-Snapshot. Alle 2.057 verwalteten Dateien
stimmen bytegenau mit dem ausgelieferten r1674 überein. Root-Cargo.lock ist die
einzige zusätzliche Snapshot-Datei und bleibt aus dem Take-Paket ausgeschlossen.

Der bestätigte Korpus umfasst zehn formal grüne Projekte. 2.985 native Tests
bestanden, vier ignoriert; Build, Clippy und Launcher-Test-Exit jeweils 0.
Die Korpuszeit beträgt 12:53 Minuten gegenüber 12:56 im vorherigen Export.
Alle 20 MusicXML-Dateien sind gegenüber dem r1671-Export bytegleich. Das ist
ein beobachteter Laufvergleich, keine isolierte Performanceaussage.

Dieser Take ersetzt den bereitgestellten r1675 und enthält dessen vorbereiteten
Feinschliff unter einer neuen eindeutigen Revision direkt auf r1674. r1675
muss zuvor nicht angewendet werden. Er ist hier kein angenommener Parent.

## Eigenständig gewählter Take

Bei einem längeren Lauf wechselt man häufig zu einem anderen Browser-Tab.
Der bisher unveränderliche Titel „EtuSchlopf Watch“ verrät dann nichts über
den beobachteten Lauf. Um seinen Zustand festzustellen, muss man die Ansicht
wieder öffnen.

Der Tabtitel zeigt jetzt den bereits vorhandenen Status der oberen Statusanzeige,
zum Beispiel `RUNNING · EtuSchlopf Watch` oder `GELB · EtuSchlopf Watch`.
Schon das initiale HTML meldet `WAITING`. Die bestehende Statusprojektion setzt
anschließend Badge und Titel gemeinsam. Die vorhandene Auswahl der aktuellen
Sitzung, Vorrangregeln und gemeldete Abschlussfarben bleiben maßgeblich;
der Titel berechnet keine eigene Erfolgsbewertung.

Bei einem Korpuslauf ergänzt er die vom aktuellen Korpus-Snapshot gemeldeten
Zähler, etwa `RUNNING · 3/7 · EtuSchlopf Watch`. Ein Zählerwechsel aktualisiert
den Titel auch bei unverändertem Laufstatus. Der gemeldete Endstand bleibt beim
Abschluss sichtbar. Ohne Korpusfortschritt mit positiver Gesamtzahl entfällt
dieser Zusatz. Die Ansicht zählt dafür weder Projektkarten nach noch übernimmt
sie Zähler früherer Sitzungen.

Ein Verbindungsende ändert den letzten beobachteten Laufstatus nicht; die
separate Transportanzeige bleibt dafür zuständig. Der Titel behauptet weder
aktuelle Erreichbarkeit noch einen aus Zeitablauf abgeleiteten Abschluss.
Er wird nur bei tatsächlich geändertem Text neu zugewiesen. Identische
Snapshots und reine Uhrenticks lassen seinen DOM-Knoten unverändert.

Die Änderung betrifft ausschließlich den Browser-Dokumenttitel und die
gemeinsame Zuweisung seiner vorhandenen Statusbeschriftung. Keine zusätzliche
Abhängigkeit, Anfrage, Benachrichtigungsberechtigung oder Laufsteuerung.

## Browserprobe

`tools/etuschlopf_watch/tests/web_tab_title.cjs` lädt die eingebettete
Produktionsseite in Chromium Headless Shell. Nur EventSource-Nachrichten sind
synthetisch. Zehn Szenarien prüfen:

- Initiales Warten vor dem ersten Snapshot.
- Laufender Zustand und wiederholte direkte Uhrenticks.
- Unveränderten Titel-DOM bei wiederholtem Snapshot und echtem Sekundentimer;
  ein tatsächlicher Statuswechsel muss weiterhin eine DOM-Änderung erzeugen.
- Abschluss, Fehler und Abbruch nach einem laufenden Zustand.
- Alle vier gemeldeten Abschlussfarben.
- Laufenden Zustand mit vorzeitig vorhandenem Farbwert.
- Alte sowie fehlende aktuelle Sitzungen.
- Transportende nach gemeldetem Abschluss.
- Fortschreitende Korpuszähler einschließlich Abschluss und Transportende.
- Fehlende/Null-Gesamtzahlen und Ausschluss früherer Korpussitzungen.

Am unveränderten r1674 scheitert die Titelprobe bereits beim initialen Titel.
Am ausgelieferten r1675 scheitert die DOM-Probe an wiederholten Titelwrites.
Die Zählerprobe scheitert am ersten Feinschliff ohne Fortschrittszusatz.
Diese Gegenproben und alle Eingaben sind synthetisch und benötigen keine
MIDI-/WAV-Dateien oder einen Renderer.

Die manuelle Probe benötigt separat installiertes Node.js, Playwright und
Chromium, ebenso wie die bestehende Fokusprobe. Aus der Projektwurzel:

```text
node tools/etuschlopf_watch/tests/web_tab_title.cjs
```

Ein optionales erstes Argument benennt eine andere `web_watch.rs` für den
Alt-/Neu-Vergleich. Der reguläre Launcher startet die Browserprobe nicht.

## Lokale Qualifikation

Zehn Titelproben und die bestehenden neun Fokusproben bestehen ohne
JavaScript-Seitenfehler. 38 Watch-Rust-Tests, strenges Watch-Clippy für alle
Targets mit `-D warnings`, Strict-Build und statische Take-Prüfung sind grün.
Die lokalen Browserproben verwenden Playwright 1.56.1 und Chromium Headless
Shell 141.0.7390.37; sie sind kein Nachweis für alle Browser.

Alle Hauptprogrammquellen und Root-Tests bleiben zum angenommenen r1674
bytegleich; lediglich das Take-Änderungsmanifest wird neu gebunden. Die native
Abnahme von r1676 bleibt separat offen.
