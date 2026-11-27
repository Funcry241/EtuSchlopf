# EtuSchlopf

EtuSchlopf macht Klaviernoten aus einer Einspielung. Aus einer passenden
MIDI-Datei und WAV-Aufnahme erzeugt das Rust-Programm ein Noten-PDF, MusicXML
für die Weiterbearbeitung und einen Laufbericht.

[Erster Start mit der Windows-EXE](.github/README.md#dein-erstes-stück) ·
[Notenbilder](docs/examples/README.md#notenbilder) ·
[Mathe-Seite](docs/MATHEMATIK.md)

## Stand der Notenausgabe

Der unter Windows abgenommene Quellstand **r2022 / 0.10.1839** erzeugt 15 Partituren
mit zusammen 58 PDF-Seiten. Die Galerie zeigt die Original-PDFs dieses Laufs,
Vorschauausschnitte und die jeweils verwendete MusicXML. Die Oktavbewertung
verwendet die vorhandene Zeitanalyse wieder. Das Notenbild stimmt mit r2021
überein.

[Windows-Download r2022](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r2022-windows-preview) ·
[Windows-Abnahme r2022](docs/evidence/r2022/README.md) ·
[Halsnachweis](docs/MATHEMATIK.md#kapitel-6-18)

<a name="dein-erstes-stück"></a>

## Aus dem Quellcode starten

Installiere [Rust 1.98.0 mit Clippy und rustfmt](rust-toolchain.toml) und
[MuseScore](https://musescore.org/de/download) für die PDF-Ausgabe. Lege
`Mein Projekt.mid` und `Mein Projekt.wav` im Projektordner ab. Beide Dateien
müssen zur selben Einspielung gehören.

Führe im Projektordner den passenden Befehl aus.

| Aufgabe | Windows-Aufruf |
| --- | --- |
| Ein Stück verarbeiten | `.\run.cmd "Mein Projekt"` |
| Alle Stücke im Ordner verarbeiten | `.\run.cmd --all` |
| Den Korpuslauf im Browser verfolgen | `.\run.cmd --watch --all` |
| Einen passenden Take einspielen und den Korpus prüfen | `.\run.cmd --take --all` |

Der Launcher koordiniert Build, Strict-Prüfungen und Export. EtuSchlopf Watch
zeigt Fortschritt, Laufzeiten und die Links zu den erzeugten PDFs.

## Vollständige Aufrufreferenz

Die [technische Referenz](docs/DEVELOPMENT.md#vollständige-aufrufreferenz)
enthält alle Launcher- und Programmoptionen, die Renderer-Konfiguration und
Beispiele für Diagnoseaufrufe.

## Ausgaben

| Datei | Inhalt |
| --- | --- |
| `Mein Projekt.musicxml` | Musikalischer Master |
| `Mein Projekt.render.musicxml` | Fassung für den Notensatz |
| `Mein Projekt.pdf` | Mit MuseScore gesetzte Partitur |
| `Mein Projekt.result.txt` | Laufbericht und Prüfbefunde |

Für den Vollkorpus erstellt `--all` zusätzlich `assistant_export_latest.zip`
mit Laufstatus und verfügbaren Prüfbelegen.

## Dokumentation und Beiträge

- [Dokumentationswegweiser](docs/README.md)
- [Beiträge und Fehlerberichte](docs/CONTRIBUTING.md)
- [Sicherheit](docs/SECURITY.md)
- [Umgang miteinander](docs/CODE_OF_CONDUCT.md)
- [Barrierefreiheit](docs/ACCESSIBILITY.md)

## Lizenz

Der eigene EtuSchlopf-Code steht unter der
[MIT-Lizenz](docs/licenses/EtuSchlopf-MIT.txt).
[Fremdlizenzhinweise](docs/THIRD_PARTY_NOTICES.md) ·
[Musikquellen und Rechte](docs/examples/README.md#quellen-und-rechte)
