# RendererFeasibilityRunLocalWinningCommandReuseClosure r929

**Parent:** `0.10.1839 r928`  
**ChangeClass:** `RefactorOnly`  
**Musikmutation:** keine

## Gemessener Ausgangsbefund

Der akzeptierte r928-Lauf fuer Etuede Nr. 3 endete `dark_green`, `released`,
`FinalExit 0`, benoetigte aber weiterhin rund 7:24 Minuten. Neben Clippy,
Produktbuild und All-Targets-Testcompile lag die eigentliche Programmlaufzeit bei
rund 3:27 Minuten.

Der committed Renderer-Feasibility-Oracle benoetigte fuenf Shadow-Runden. Der
MuseScore-Fallbackpfad fand die erfolgreiche Kommandoform erst bei Variante 8
`-f -o <pdf> <input>`. Vor dem Erfolg entstanden sieben bereits beendete,
erfolglose Prozesse und rund 21 Sekunden Settle-Zeit. Dieselbe vollstaendige
Kompatibilitaetssuche wurde in jeder Shadow-Runde erneut begonnen, obwohl eine
Kommandoform im selben Prozess und fuer dasselbe Shadow-Projekt bereits real
erfolgreich bewiesen war.

## Aenderung

`render.rs` behaelt die bestehende elfteilige Kommando-Menge und alle bestehenden
Freshness-, Prozess-, PDF-Struktur- und FinalRender-Guards unveraendert.

Nur fuer Pfade unter dem bestehenden
`target/etuschlopf_renderer_feasibility_shadow/<project>` wird die im aktuellen
Prozess zuletzt erfolgreich ausgefuehrte Kommandoform fuer exakt dieses
Shadow-Projekt und Renderer-Executable run-lokal gemerkt. Beim naechsten
Shadow-Render wird diese real bewiesene Form zuerst versucht.

- Erfolg: dieselbe Form wird fuer die naechste Shadow-Runde erneut gemerkt.
- Fehlschlag: die vollstaendige bisherige Variantenmenge bleibt vorhanden und
  wird weiter abgearbeitet; ein neu erfolgreicher Gewinner ersetzt den Hinweis.
- Kein persistenter Cache, kein Cross-Run-Reuse, kein Cross-Project-Reuse.
- Produktiver Final-PDF-Pfad erhaelt keinen Hint und behaelt seine bisherige
  Kommando-Reihenfolge.

Damit wird die bereits historisch etablierte Regel "exakt bekannte erfolgreiche
MuseScore-Kommandoform im selben Shadow-Proof wiederverwenden" wieder auf den
heutigen committed-source Oracle angewandt, ohne die Oracle-Rundenzahl oder seine
Beweisbreite zu reduzieren.

## Unveraendert

- fuenf bzw. bis zur bestehenden Grenze erforderliche Oracle-Runden,
- Candidate-/NoGood-/HorizontalLayout-Logik,
- produktive MusicXML-Bytes und Writer,
- alle elf MuseScore-Kommandovarianten,
- Renderer-Discovery,
- PDF-Freshness- und Strukturpruefung,
- Release-/Semantic-/Take-Gates,
- Solver, Regeln und musikalische Semantik.

## Erwarteter Beweis

Auf einem Projekt, das mehrere Renderer-Oracle-Runden benoetigt, soll die erste
Shadow-Runde weiterhin den realen Kompatibilitaetspfad entdecken. Die folgenden
Runden sollen die bereits erfolgreiche Form direkt wiederverwenden. Die Konsole
projiziert dafuer `RendererRunLocalShadowWinningCommandReuse`.

Abnahme zuerst mit:

```powershell
.\\run.cmd --take "Etüde Nr. 3"
```

Nur `dark_green`, `released`, `FinalExit 0` akzeptiert r929 als neuen
AuthorityParent. Danach werden ProgramMs und Renderer-Zeiten gegen r928
verglichen.
