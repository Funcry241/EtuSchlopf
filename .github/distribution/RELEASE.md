# Windows-Download

Die Windows-Vorschau r2020 enthält die portable x64-EXE, das Quellpaket, das Startbeispiel, den Build-Nachweis und die SHA-256-Prüfsummen. Die EXE benötigt für die PDF-Ausgabe eine MuseScore-Installation. [Windows-Download r2020](https://github.com/Funcry241/EtuSchlopf/releases/tag/v0.10.1839-r2020-windows-preview)

[Einstieg](../README.md#dein-erstes-stück) · [Notenbilder](../../docs/examples/README.md#notenbilder) · [Windows-Prüfbeleg](../../docs/evidence/r2020/README.md) · [Mathe-Seite](../../docs/MATHEMATIK.md#kapitel-6-18)

## Build und Startbeispiel

Der Windows-Workflow rekonstruiert den abgenommenen r2020-Quellstand, baut die Vorschau und prüft das Paket. Für das Startbeispiel übernimmt er MIDI und WAV aus dem r1994-Release. Das Skript prüft die SHA-256-Summe dieses Eingabearchivs und erzeugt das Beispiel-PDF mit der aktuellen EXE. [Windows-Workflow](../workflows/windows-download.yml) · [Skript für das Startbeispiel](start-example.ps1)

Die früheren Windows-Workflows für r1992 und r1994 sind im damaligen Repository-Stand dokumentiert. [Historische Workflows](https://github.com/Funcry241/EtuSchlopf/tree/2f3889216637099de3f66dc7a808fdeac1d11771/.github/workflows)
