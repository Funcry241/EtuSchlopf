# EtuSchlopf für Windows

Die portable EXE für Windows x64 enthält die Engine 0.10.1839 r1965. Lade die EXE und `SHA256SUMS.txt` von der [Release-Seite](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1965-windows-preview). Für die PDF-Ausgabe brauchst du MuseScore. Die EXE läuft ohne Rust, Cargo oder Python.

Lege deine Performance-MIDI und die gleichnamige WAV-Aufnahme in denselben Ordner. Ziehe die MIDI-Datei auf die EXE oder starte die EXE und gib den MIDI-Pfad ein. Das Programm arbeitet mit Kopien und legt für jeden Lauf einen neuen Unterordner unter `EtuSchlopf-Ergebnisse` neben der MIDI an. Dort findest du die PDF zum Lesen und die Datei mit der Endung `.render.musicxml` zum Bearbeiten in MuseScore.

Der native Windows-Vollkorpus umfasst vier klassische Werke und zehn Etüden. Das Notenarchiv enthält die 14 vollständigen PDFs aus diesem Lauf, einen Ausschnitt pro Werk und die Quellenangaben. [Vollkorpus-Nachweis](../../docs/evidence/r1965/README.md) · [Notenarchiv r1965](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1965-windows-preview/EtuSchlopf-Notenbilder-r1965.zip)

GitHub baut die EXE aus dem angenommenen Quellstand und prüft den isolierten Start, die Verarbeitung von MIDI und WAV und die PDF-Ausgabe mit MuseScore. Das separate Startbeispiel enthält MIDI, WAV und die mit dieser EXE erzeugte PDF. [Startbeispiel r1965](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1965-windows-preview/EtuSchlopf-Beispiel-r1965.zip)

Die EXE ist unsigniert. Vergleiche bei Bedarf die Ausgabe von `Get-FileHash .\EtuSchlopf-Windows-x64.exe` mit der veröffentlichten SHA-256-Prüfsumme. Die Fremdlizenzen sind im Laufzeitpaket enthalten. Das Quellarchiv enthält auch den Windows-Starter.
