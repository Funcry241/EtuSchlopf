# Satzfix-Zeichenexport

Der Quellcode stammt aus Commit `619443e`. Der Take ist ein Kandidat auf Basis von r1994. Der GPLv3-Lizenztext und der datierte Änderungshinweis für den MuseScore-Patch liegen bei der [Renderer-Dokumentation](../../../tools/satzfix_renderer/README.md).

Für 15 Partituren wurden 59 Seiten mit dem Linux-Renderadapter geprüft. Zwei Seiten ändern sich an genau fünf Bogenpfaden. Alle vier bekannten Kollisionen sind behoben. Die 57 übrigen Seiten sind bildgleich. Der nachgewiesene Abstand bei Chopin beträgt mindestens 0,5552 pt. Bei Schubert weisen die vier Segmente jeweils mindestens 1,0000 pt Abstand auf. Die Anforderung beträgt rund 0,5454 pt.

Clippy, 2.394 Rust-Tests und der statische Rust-Check für das Windows-Ziel waren für den Integrationscommit `09f09b4` erfolgreich. Der Take reicht den Zeichenplan direkt an den vorhandenen Rendereraufruf weiter und verwendet die vorhandene Zeichentoleranz sowie die gemessene Linienbreite. Für den Windows-PDF-Lauf ist ein Windows-Build von MuseScore 4.7.5 mit dem beiliegenden Patch erforderlich. Die Windows-PDF-Freigabe steht aus.

Baue den Renderer nach [MuseScore-Zeichenschnittstelle](../../../tools/satzfix_renderer/README.md). Lege das Quell-ZIP in Downloads und starte im vorhandenen r1994-Projektordner in PowerShell:

```powershell
$env:ETUSCHLOPF_PDF_RENDERER = 'C:\MuseScoreSatzfix\MuseScore4.exe'
$env:ETUSCHLOPF_SLUR_PAINT = '1'
.\run.cmd --take --all
```
