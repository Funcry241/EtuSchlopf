# RendererFeasibilityConstraintShadowRenderProofTargetPathClosure – r36

## Ausgangsbefund

r35 ist technisch DARK GREEN und transportiert die produktive MuseScore-Runtime-
Evidence erfolgreich bis zum Shadow-Render-Proof. Der Proof stoppt danach
fail-soft mit:

`renderer-feasibility shadow candidate path is not target-scoped`

Der produktive Einzelprojektaufruf verwendet einen relativen `SourcePlan`.
Dadurch ist die Parent-Directory des produktiven PDFs `.`. Der daraus gebildete
Shadowpfad beginnt folglich mit `./target/...`.

Die seit r34 zentrale `verified_render_candidate_sink_authority` verwirft
absichtlich Pfade mit `CurDir` oder `ParentDir`. Dieser Vertrag bleibt richtig
und wird nicht gelockert.

## Aenderung

Vor der Bildung des Shadowpfads wird die bereits existierende Projektwurzel mit
`std::fs::canonicalize` auf ihre reale absolute Form gebunden. Erst unter dieser
kanonischen Wurzel wird der bereits festgelegte target-only Pfad aufgebaut:

`target/etuschlopf_renderer_feasibility_shadow/<project>/constraint_candidate.render.musicxml`

Der zentrale Sink bleibt unveraendert. Es entsteht keine neue Write-Surface und
keine neue Authority.

## Regression

Ein neuer Test bildet explizit eine Dot-Project-Root (`<temp>/.`) und beweist,
dass der resultierende Shadowpfad weder `CurDir` noch `ParentDir` enthaelt und
weiterhin exakt unter `target/etuschlopf_renderer_feasibility_shadow/<project>`
liegt.

## Nicht geaendert

- keine Musikmutation
- keine produktive Layoutauswahl
- keine Demand-Neuberechnung
- keine Capacity-/Threshold-Aenderung
- keine neue Writer-Authority
- keine Productive-Write-Bypass-Baseline-Aenderung
- keine Lockerung der r34-Pfadpruefung
- kein Rendererpfad oder Command wird erfunden

## Erwarteter Beweis

Der Einzellauf muss 761/761 Rust-Tests erreichen. Der Shadow-Proof darf nicht
mehr an `not target-scoped` scheitern. Danach ist entweder
`RENDERER_CONFIRMED`, `RENDERER_REJECTED` oder eine spaetere, konkret
begruendete `UNAVAILABLE`-Stufe zulaessig; produktive Artefakte muessen
unveraendert bleiben.
