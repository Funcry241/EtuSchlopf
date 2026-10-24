# CanonicalMuseScorePdfCommandFirst

**Version:** v0.10.1619 r01  
**Change-Klasse:** RefactorOnly  
**Ausgangsstand:** v0.10.1618  
**Musikmutation:** keine

## Gemessene Ursache

Export 559 zerlegt die 160.622 ms Vorberichtslaufzeit wie folgt:

- Arrangement/Solver: 20.183 ms
- Render-/Writer-Vorbereitung: 1.843 ms
- PDF-Quellenauswahl und Renderer: 137.946 ms
- übrige klassifizierte Phasen: 640 ms

Der gültige Rendererabschluss nennt als erfolgreiche Kommandoform ausdrücklich
`-o <pdf> <input>`. Diese Form stand im Variantenledger erst an vierter Stelle.
Davor liefen `-s -w -o`, `-s -o` und `-w -o`. Jeder erfolglose Versuch kann nach
Prozessende bis zu 45 Sekunden auf eine zweimal bestätigte PDF-Präsenz warten.
Drei solche Diagnosefenster erklären nahezu exakt die gemessenen rund 138 Sekunden.

## Änderung

Die bereits erfolgreiche und im Quellkommentar als dokumentierter Convertermodus
bezeichnete Form `-o <pdf> <input>` wird an die erste Stelle gesetzt. Alle bisherigen
Varianten bleiben als Fallbacks in derselben relativen Reihenfolge erhalten. Die
`-f`-Varianten bleiben weiterhin ausschließlich am Ende.

## Unverändert

- Renderer-Erkennung und ausführbare Datei,
- MusicXML-PDF-Quelle,
- PDF-Ausgabepfad und Vorab-Löschung,
- Prozess- und PDF-Strukturprüfung,
- 45-Sekunden-Diagnosefenster für tatsächlich benötigte Fallbackversuche,
- Kandidaten, Solver, Writer, MusicXML und MasterApply,
- Takt-, Etüden-, Projekt-, Pitch- und Locatorauswahl.

## Erwartete Zielsystemwirkung

Für Etüde Nr. 4 soll der erste Versuch bereits dieselbe gültige PDF erzeugen. Erwartet:

- `RenderPdfSourceSelectionMs` fällt von 137.946 ms auf wenige Sekunden,
- `PreReportTotalMs` fällt ungefähr von 160.622 ms auf 25–30 Sekunden,
- beide MusicXML-Dateien sind nach Versionsnormalisierung identisch,
- alle sechs PDF-Seiten sind pixelidentisch,
- Build, Strict Clippy und FinalExit bleiben 0.

Ein Korpuslauf ist nicht erforderlich. Abnahme mit:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```
