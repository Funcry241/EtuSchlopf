# r1669 – Tastaturfokus in der Watch-Webansicht

ChangeClass: RefactorOnly. Basis ist der nativ angenommene r1667 / 0.10.1839,
Take-SHA256 `63945622c8d4dec57d7114e2fb638e0f266c92236e8846a01a9aaad2086f253c`.
Der Export `assistant_export_latest(20261108-001838).zip` bindet Annahmebeleg und
vollständigen Source-Snapshot. Dessen verwaltete Dateien sind bytegleich mit der
bisher vorbereiteten r1667-Basis; Root-Cargo.lock bleibt lokaler Zustand.
Dieser Take ersetzt den bereitgestellten, noch nicht nativ angenommenen r1668
mit einer neuen eindeutigen Revision und enthält dessen Fokuserhalt.

Der neue r1667-Korpus ist formal grün: 2.980 Tests bestanden, 4 ignoriert,
Build/Clippy/Launcher-Test-Exit jeweils 0, zehn gültige Projekte. Die Laufzeit
beträgt 13:27 gegenüber 14:13 im vorherigen Export (-46 s, etwa -5,4 %).
Alle 20 MusicXML-Dateien sind bytegleich. Das ist ein beobachteter Laufvergleich,
keine isolierte kausale Messung der Performanceänderung.

## Problem und Verhalten

Bei einer Änderung von Projektphase, Zustand oder Projektliste ersetzt
`renderStructure` die HTML-Struktur. Der Browser verliert dadurch den Fokus auf
der bisherigen Projektzeile, ihrem PDF-Link oder den technischen Details.
Die nächste Tab-Taste setzt an einer anderen Stelle fort. Das bisherige
Wiederöffnen von Details war außerdem nur an die Projekt-ID gebunden und konnte
bei einer neuen Sitzung dieselben IDs unbeabsichtigt übernehmen.

Jetzt bindet die Ansicht ihre Bedienbereiche an eine eindeutige DOM-ID aus
Sitzung, Source-ID und Darstellungsort. Der Darstellungsort unterscheidet
Projektzeile, technische Karte und Einzeltitel. Vor dem Austausch merkt sie sich
den fokussierten Link beziehungsweise Summary und die geöffneten Details.
Anschließend öffnet sie die erhaltenen Details und fokussiert das entsprechende
Element ohne Scrollsprung. DOM-IDs werden als Daten direkt aufgelöst; sie werden
nicht in dynamische CSS-Ausdrücke eingesetzt.

Umbenennen, Einfügen und Umsortieren von Projekten ändern diese Zuordnung nicht.
Gleichnamige Projekte bleiben getrennt. Titel- und Detail-PDF desselben Projekts
bleiben ebenfalls getrennt. Ein neuer Lauf, ein verschwundenes Projekt oder ein
entfallener Darstellungsort erhält keinen ersatzweise ausgewählten Fokus.
Fokus außerhalb der ausgetauschten Ansicht bleibt unangetastet. Die reine
Uhrenaktualisierung tauscht weiterhin keine Struktur aus.

PDF-Erzeugung, MQTT-Nachrichten, Runtime-Zustände und musikalische Verarbeitung
bleiben unverändert. Die Änderung betrifft ausschließlich den lokalen
Bedienzustand der bestehenden Webansicht.

## Browserprobe

`tools/etuschlopf_watch/tests/web_focus.cjs` lädt die tatsächlich eingebettete
Produktionsseite in Chromium Headless Shell. Ausschließlich Beobachtungsnachrichten
und PDF-Antworten sind synthetisch; DOM, HTML-Parser, Fokus, Tastatureingaben und
Scrollverhalten stammen vom Browser. Es werden keine Korpusdateien benötigt.

Neun Szenarien prüfen:

- Fokus auf Projektzeilen und Fortsetzung mit Tab, Umschalt-Tab und Leertaste.
- PDF-Fokus nach Umbenennung, Einfügen und Umsortieren; Sonderzeichen in IDs.
- Mehrere aufgeklappte Projektzeilen und technische Details.
- Getrennte Titel-/Kartenlinks im Einzelprojekt.
- Verschiedene Projekte mit gleichem Namen.
- Neue Sitzung mit wiederverwendeten Source-IDs ohne Übernahme des Bedienzustands.
- Entfernte Projekte und Wechsel des Darstellungsorts ohne Ersatzfokus.
- Unveränderter externer Fokus und erhaltenes DOM-Element bei reinen Uhrenticks.
- Fokuserhalt ohne Scrollsprung zu einem weit unten liegenden Projekt.

Die Probe scheitert am unveränderten r1667 bereits beim ersten Statuswechsel
mit verlorener Summary-Fokussierung. Mit r1669 bestehen alle neun Szenarien,
ohne JavaScript-Seitenfehler. Lokal verwendet: Playwright 1.56.1 und Chromium
Headless Shell 141.0.7390.37. Das ist kein Nachweis für alle Browser,
Windows-Konfigurationen oder assistiven Technologien.

Die manuelle Probe benötigt eine separat vorhandene Node.js-/Playwright-
Installation samt Chromium Headless Shell. Aus der Projektwurzel:

```text
node tools/etuschlopf_watch/tests/web_focus.cjs
```

Playwright muss über die normale Node-Modulauflösung beziehungsweise `NODE_PATH`
verfügbar sein. Ein optionales erstes Argument benennt einen anderen
`web_watch.rs`-Stand für den Alt-/Neu-Vergleich. Die Probe startet keinen echten
Korpus, keinen MQTT-Dienst und keinen PDF-Renderer. Das reguläre `run.cmd` startet
die Browserprobe nicht und benötigt keine neue Node-Abhängigkeit.

## Bestehende Clippy-Befunde

Die vier im Watch-Elternstand vorhandenen Befunde werden an ihren Quellen
behoben: `repeat(...).take(...)` wird durch `repeat_n` ersetzt, die unnötige
Referenz beim CONNACK-Payloadvergleich entfällt und die beiden Verbindungs-
`match`-Ausdrücke mit leerem Fehlerarm werden als `if let Ok(...)` geschrieben.
Erfolgspfad, Fehlerbehandlung, Wiederverbindung und MQTT-Bytes bleiben gleich.
Es werden keine Lint-Ausnahmen oder zusätzlichen Abhängigkeiten eingeführt.

## Qualifikation

30 Rust-Tests und neun echte Browserproben des Watch-Projekts bestehen.
Watch-Clippy für alle Targets mit `-D warnings` und der Strict-Produktionsbuild
sind grün. Die statische Take-Prüfung gegen den angenommenen r1667 ist grün.
Die Musikquellen und das Hauptprogramm bleiben gegenüber r1667 bytegleich;
das Änderungsmanifest beschreibt ausschließlich diesen Folgetake.
Die native Annahme von r1669 steht bis zum neuen Lauf separat aus.
