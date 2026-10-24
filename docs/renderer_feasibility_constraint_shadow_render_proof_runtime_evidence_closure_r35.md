# Renderer Feasibility Constraint Shadow Render Proof Runtime Evidence Closure r35

## Ausgangslage

r34 schloss die vom Architecture-Build-Gate korrekt erkannte neue externe
Shadow-Write-Surface. Der r34-Einzellauf ist danach technisch vollstaendig gruen:
759/759 Rust-Tests, DARK GREEN, Revision r34 committed.

Der eigentliche Shadow-Render-Proof blieb jedoch read-only `UNAVAILABLE` mit
`Reason renderer_runtime_evidence_missing`.

Die Ursache ist keine Renderer- oder Layoutabweichung. Die produktive
Renderer-Evidence existiert bereits im selben Lauf und wird als
`silent_voice_coverage_projection_message` mit der exakten erfolgreichen
`PDF: ... Renderer: '...' ... Aufruf: ... ExitCode 0.`-Zeile bis in die
Final-Report-Phase transportiert. Sie wird jedoch nicht in `CONSOLE_CAPTURE`
geschrieben. Der r34-Proof suchte die Renderer-Evidence ausschliesslich dort.

## r35-Closure

r35 aendert keinen Renderer, keinen Planner und keinen Writer. Die bestehende
Page-Count-/Renderer-Readback-Projektion erhaelt lediglich den bereits
vorhandenen produktiven Renderer-Outcome als read-only Fallback-Evidence.

Prioritaet:

1. vorhandene eindeutige Renderer-Zeile aus `CONSOLE_CAPTURE`, falls vorhanden;
2. sonst die eindeutige Renderer-Zeile aus dem bereits berechneten produktiven
   Renderer-Outcome desselben Laufs;
3. sonst weiterhin `renderer_runtime_evidence_missing`.

Es wird kein Rendererpfad erraten, kein Standardpfad neu gesucht und keine
Command-Shape rekonstruiert. Der Shadow-Proof verwendet weiterhin nur die
tatsaechlich beobachtete erfolgreiche Renderer-Zeile des laufenden produktiven
Renders.

## Authority

Unveraendert:

- CandidateOnly ja
- SelectionAuthority nein
- ProductiveWriterAuthority nein
- PolicyWrite nein
- keine Demand-/Capacity-/Threshold-Aenderung
- keine Musikmutation
- Shadow-MusicXML nur unter der bereits in r34 geschlossenen zentralen
  `verified_render_candidate_sink_authority`
- produktive Master-/Render-MusicXML und PDF muessen waehrend des Shadow-Proofs
  byteidentisch bleiben.

## Regression

Ein neuer Rust-Test beweist den konkreten r34-Randfall: Wenn
`CONSOLE_CAPTURE` keine Renderer-Zeile enthaelt, aber der produktive
Renderer-Outcome die eindeutige erfolgreiche MuseScore-Zeile besitzt, wird genau
diese Evidence verwendet.

Erwarteter Testbestand: **760/760**.

## Prueflauf

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Erwartetes Ziel fuer Etuede 6 ist nicht mehr
`Reason renderer_runtime_evidence_missing`, sondern ein tatsaechlicher
Shadow-Rendererbefund: `RENDERER_CONFIRMED`, `RENDERER_REJECTED` oder ein
anderer konkret begruendeter `UNAVAILABLE`-Randfall. Erst
`RENDERER_CONFIRMED` legitimiert den anschliessenden Vollkorpus-Beweis.
