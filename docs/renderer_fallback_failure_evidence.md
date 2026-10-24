# RendererFallbackFailureEvidence

Version: 0.10.1751 r01

## Anlass

v0.10.1750 hat den Etuede-9-Programmlauf von rund 343 s auf rund 47 s reduziert.
Die Laufmessung bestaetigt den Renderer-Hotspot exakt: sieben Varianten scheitern
vor dem erfolgreichen `-f -o <pdf> <input>`-Aufruf jeweils mit Nonzero-Exit und
ohne PDF. Die verbleibende Frage ist nicht mehr, ob die Varianten scheitern,
sondern warum MuseScore fuer diesen Input erst die Force-Variante akzeptiert.

## Aenderung

Dieser RefactorOnly-Take veraendert keine Rendererentscheidung, keine
Kommando-Reihenfolge, keine Wartezeit und keine PDF-/MusicXML-Semantik. Die bereits
existierende `RendererFallbackTiming`-Evidenz wird lediglich um kompakte
Fehlerbelege der vor dem Erfolg gescheiterten Varianten erweitert.

Jeder Beleg enthaelt:

- Variantenposition,
- Prozess-ExitCode,
- eine grobe diagnostische Klasse,
- einen auf eine Zeile begrenzten und auf 160 Zeichen gekuerzten
  stderr-/stdout-Ausschnitt.

Die Klassen unterscheiden insbesondere CLI-Optionsfehler,
Force-/Versionswarnungen, Input-Open-Fehler, Runtime-Subsystemfehler und
sonstige Rendererfehler. Die Klassifikation besitzt keinerlei Auswahl- oder
Schreibrecht.

## Invarianten

- `render_command_variants()` bleibt byte-semantisch in derselben Reihenfolge.
- `renderer_post_process_settle_timeout_v1750()` bleibt unveraendert.
- Kein fehlgeschlagener Versuch wird uebersprungen oder bevorzugt.
- Keine Musik-, MusicXML-, PDF-, Writer-, Selector- oder Release-Policy-Aenderung.
- Die neue Evidenz ist reine Diagnose nach bereits beobachtetem Prozessausgang.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 9" --display
```

Danach entscheidet die reale `FailureEvidence`, ob ein spaeterer Performance-Take
ueberhaupt Varianten sicher ueberspringen darf oder ob zuerst der Force-Open-Grund
im Render-XML untersucht werden muss.
