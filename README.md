# EtuSchlopf

[![Windows-Prüfung](https://github.com/Funcry241/EtuSchlopf/actions/workflows/windows-download.yml/badge.svg?branch=main)](https://github.com/Funcry241/EtuSchlopf/actions/workflows/windows-download.yml)
[![Lizenz: MIT](https://img.shields.io/badge/Lizenz-MIT-blue.svg)](LICENSE)

**EtuSchlopf macht Klaviernoten aus deiner Einspielung.** Aus der MIDI-Datei und der zugehörigen WAV-Aufnahme erstellt EtuSchlopf ein Noten-PDF und MusicXML für die Weiterbearbeitung in MuseScore. Bei frei gespielten Aufnahmen leitet es Takt und Notenwerte aus der Einspielung ab.

## Dein erstes Stück

1. Lade die **Windows-EXE für x64** herunter. Für die PDF-Ausgabe brauchst du MuseScore. [EtuSchlopf-Windows-x64.exe](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1964-windows-preview/EtuSchlopf-Windows-x64.exe) · [MuseScore](https://musescore.org/de/download)
2. Lege `Mein Stück.mid` und `Mein Stück.wav` in denselben Ordner. Beide Dateien müssen zur selben Einspielung gehören. Zum Ausprobieren kannst du das fertige MIDI/WAV-Paar entpacken. [Startbeispiel als ZIP](https://github.com/Funcry241/EtuSchlopf/releases/download/v0.10.1839-r1964-windows-preview/EtuSchlopf-Beispiel-r1964.zip)
3. Ziehe die MIDI-Datei auf die EXE. Du kannst die EXE auch öffnen und den Pfad zur MIDI eingeben.
4. Öffne im neuen Laufordner unter `EtuSchlopf-Ergebnisse` das PDF. Für die Weiterbearbeitung in MuseScore öffnest du die `.render.musicxml`. Verwendet der PDF-Export direkt den Master, öffnest du die `.musicxml`.

Der Ergebnisordner liegt neben der MIDI. EtuSchlopf arbeitet dort mit Kopien der Eingaben. Bei einem Fehler zeigt die Konsole die Meldung. Wenn der Lauf einen Bericht geschrieben hat, findest du ihn im Laufordner als `.result.txt`. [Version und Prüfsummen](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r1964-windows-preview)

> **In English:** EtuSchlopf uses a matching MIDI and WAV pair to produce piano sheet music as PDF and MusicXML. The Windows x64 preview uses MuseScore for PDF export. Issues in English are welcome.

## Notenbilder

| Mozart · Sonate A-Dur KV 331 | Bach · Fuge c-Moll BWV 847 |
| --- | --- |
| [![Mozart, Andante grazioso, r1964](docs/images/mozart-k331-p01-r1964-ausschnitt.png)](docs/examples/mozart-k331-p01-r1964.pdf) | [![Bach, Fuge c-Moll, r1961](docs/images/bwv847-fuge-r1961-ausschnitt.png)](docs/examples/bwv847-fuge-r1961.pdf) |
| **Chopin · Etüde E-Dur op. 10 Nr. 3** | **Schubert · Deutscher Tanz As-Dur D. 783 Nr. 15** |
| [![Chopin, Etüde E-Dur, r1961](docs/images/chopin-op10-no3-p01-r1961-ausschnitt.png)](docs/examples/chopin-op10-no3-p01-r1961.pdf) | [![Schubert, Deutscher Tanz, r1961](docs/images/schubert-d783-no15-p18-r1961-ausschnitt.png)](docs/examples/schubert-d783-no15-p18-r1961.pdf) |

Ein Klick auf den Ausschnitt öffnet das vollständige PDF. [Alle 14 Stücke mit Notenbildern, Druckvorlagen und Quellen](docs/examples/README.md)

## Stand auf main

**r1964 / 0.10.1839** wurde unter Windows an Mozarts Andante grazioso aus KV 331 geprüft. Alle 474 verglichenen Noteneinsätze stimmen in Zeitpunkt und Tonhöhe mit der Referenz überein. Bei 460 passen auch die Notendauern. Gegenüber r1963 sank die Zahl der abweichenden Dauern von 36 auf 14. EtuSchlopf erhielt für diesen Lauf MIDI und WAV und erzeugte zwei PDF-Seiten. [Windows-Einzellauf r1964](docs/evidence/r1964/README.md) · [Mathematik hinter den Notationsentscheidungen](docs/MATHEMATIK-VOLLKORPUS.md)

Der letzte veröffentlichte Windows-Vollkorpus stammt aus r1961 und umfasst 14 Stücke auf 57 PDF-Seiten. EtuSchlopf ist experimentell. Bleiben Einsätze oder Taktgrenzen in einer frei gespielten Aufnahme mehrdeutig, kann EtuSchlopf den Lauf abbrechen. Chopins Ballade op. 38 wird separat als Stresstest geführt. [Vollkorpus r1961](docs/evidence/r1961/README.md) · [Ballade op. 38](docs/examples/chopin-op38-p01.md)

## Dokumentation und Beiträge

[Dokumentationswegweiser](docs/README.md) · [Technische Referenz](docs/DEVELOPMENT.md) · [Mitmachen](docs/CONTRIBUTING.md) · [Fehler melden](https://github.com/Funcry241/EtuSchlopf/issues/new?template=01_fehler.md)

Nenne im Fehlerbericht die Version, die Meldung und bei einem Notationsfehler die betroffene Stelle. Ein Notenausschnitt hilft bei der Zuordnung.

## Lizenz

Der Code steht unter der [MIT-Lizenz](LICENSE). Für Fremdkomponenten und Musikdateien gelten die jeweils ausgewiesenen Rechte. [Fremdlizenzen](docs/THIRD_PARTY_NOTICES.md) · [Musikquellen und Namensnennung](docs/examples/README.md#quellen-und-rechte) · [Chopin op. 38](docs/examples/chopin-op38-p01.md)
