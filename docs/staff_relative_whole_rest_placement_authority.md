# StaffRelativeWholeRestPlacementAuthority

## Anlass

Die visuelle Rueckmeldung nennt Etuede 5, Takt 6, unteres System als reproduzierbaren
Fall einer falschen Pausenhoehe. Der aktuelle Render-Stand enthaelt dort eine kanonische
`whole`-Pause auf Staff 2 im G-Schluessel mit `display-step D` / `display-octave 5`.
Dasselbe Strukturmuster kommt scoreweit mehrfach vor.

## Architekturursache

`render::rest_placement` ist bereits die alleinige produktive Pausen-Policy. Ihr Vertrag
sagt, dass sichtbare Pausen eine feste visuelle Staff-Position besitzen und die
MusicXML-Pitches nur die schluesselabhaengige Projektion dieser Position sind. Die
Implementation kodierte diese Wahrheit aber noch direkt als Pitch-Tabelle. Dadurch war
`whole + canonical` zwischen den beiden Klavierschluesseln asymmetrisch:

- F-Schluessel: `D3` (rendererbewiesenes AUTO-Aequivalent),
- G-Schluessel: `D5` (eine visuelle Linie hoeher).

Der Fehler ist damit keine Etueden-/Taktregel, sondern eine undichte Owner-Grenze:
die fachliche Wahrheit `visuelle Staff-Lage` war mit ihrer MusicXML-Projektion vermischt.

## Closure

Die Policy besitzt nun intern genau eine staff-relative Slot-Wahrheit:

- `MiddleLine`,
- `UpperLine`,
- `TopLine`.

Erst danach projiziert `RestStaffSlot::display_anchor()` den Slot auf den aktuellen
Klavierschluessel. Damit gilt fuer kanonische `whole`-Pausen in beiden Schluesseln
`MiddleLine`: `D3` im F-Schluessel und `B4` im G-Schluessel. Der F-Schluessel-Wert
bleibt bytegleich zum rendererbewiesenen Stand; der G-Schluessel wird auf denselben
visuellen Slot gebunden.

Bestehende Spezialfaelle bleiben erhalten:

- Breve/Long/Maxima -> `UpperLine`,
- polyphone Ganztaktpause -> `TopLine`,
- 32nd und kuerzer behalten die bisherige clef-spezifische Kalibrierung,
- C-/unbekannte Schluessel bleiben fail-closed `Preserve`.

## Sicherheitsgrenze

Keine Aenderung an Rhythmus, Dauer, Voice, Staff, Pitch-Ereignissen, Master-MusicXML,
Rest-Konsolidierung, Writer-Rechten, PDF-Source-Selection, Release, Manifest, Watch,
Launcher oder Recovery. Der vorhandene `PianoFinalRestGlyphCanonicalHeightRenderWrite`
und der spaete `FinalRestVerticalPlacementRenderWriterStage` konsumieren weiterhin
dieselbe eine `render::rest_placement`-Autoritaet.

Der erwartete Render-Delta betrifft ausschliesslich kanonische sichtbare `whole`-Pausen
im G-Schluessel: `D5 -> B4`. Etuede 5/Takt 6 ist nur der Reproduktionsanker, kein
Selektor.

## Abnahmefolge

Zuerst nur der Reproduktionsanker als fruehes Gate:

```powershell
.\run.cmd --take --strict "Etüde Nr. 5" --display --visual-preflight
```

Erwartet: FinalExit 0, Takt 6 unten visuell korrigiert, keine klingende/zeitliche
Aenderung und die strukturell gleichen G-Schluessel-`whole`-Pausen derselben Etuede
auf demselben staff-relativen Slot. Erst danach ist wegen der 17 bereits im letzten
Zehnerkorpus nachgewiesenen Strukturtreffer genau ein Korpuslauf gerechtfertigt:

```powershell
.\run.cmd --strict --all --display --visual-preflight
```
