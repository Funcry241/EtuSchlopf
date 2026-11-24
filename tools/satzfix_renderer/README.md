# MuseScore-Zeichenschnittstelle für Satzfix

Die Schnittstelle übernimmt geplante Bogenkonturen im letzten Zeichenschritt eines PDF-Exports. EtuSchlopf erzeugt den Plan aus Vorschauen derselben Notationsquelle und prüft die Bestätigung für jeden übergebenen Pfad.

Die Änderung vom 23. November 2026 basiert auf [MuseScore Studio 4.7.5, Commit `3654226`](https://github.com/musescore/MuseScore/tree/3654226c2e99289916916953a98e585a3d3b315a).
Der Patch ergänzt in `tdraw.cpp` die optionale Übergabe der Bogenkonturen.
`etuschlopf_slur_paint_plan.h` liest den Zeichenplan. Beide Dateien stehen
unter der [GNU General Public License, Version 3](MuseScore-LICENSE.txt). Der Patch
erhält die vorhandenen Urheberrechts- und Lizenzhinweise in `tdraw.cpp`.
Die [Lizenzhinweise von MuseScore](https://github.com/musescore/MuseScore/blob/3654226c2e99289916916953a98e585a3d3b315a/LICENSE.txt)
gelten für den MuseScore-Quellbaum.

## Lokaler Build unter Windows

Wechsle im MuseScore-Checkout zum angegebenen Commit und wende den Patch an.

```powershell
git apply C:/EtuSchlopf/tools/satzfix_renderer/musescore-v4.7.5.patch
Copy-Item C:/EtuSchlopf/tools/satzfix_renderer/etuschlopf_slur_paint_plan.h src/engraving/rendering/score/
```

`C:/EtuSchlopf` steht hier für deinen EtuSchlopf-Projektordner.

Baue MuseScore nach der [Build-Anleitung des Projekts](https://github.com/musescore/MuseScore/wiki/Compilation).
Mit diesen Einstellungen verwendet EtuSchlopf den gepatchten Renderer.

```powershell
$env:ETUSCHLOPF_PDF_RENDERER = 'C:\MuseScoreSatzfix\MuseScore4.exe'
$env:ETUSCHLOPF_SLUR_PAINT = '1'
.\run.cmd --all
```

## Weitergabe eines Renderer-Builds

Dieses Quellpaket enthält die Änderungen für MuseScore. Bei Weitergabe einer
gepatchten EXE muss der vollständige, zu dieser EXE gehörende Quellcode unter
der GPL verfügbar sein. Dazu gehören der MuseScore-Quellstand mit angewendeten
Änderungen und die zur Erstellung und Installation benötigten Skripte gemäß
Abschnitten 1 und 6 der GPLv3. Lizenz- und Urheberrechtshinweise der
mitgelieferten Komponenten gehören zum Build-Paket.
