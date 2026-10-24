# TypedReportEvidenceFlowBoundary

## Zweck

v0.10.1693 entfernt die bekannte Report-Text-Rueckrekonstruktion aus der Mutation-Ownership-Evidence.

Die bereits produktiv erzeugten `ShadowRestRepairLoopInvocationEvidence`- und `RestArticulationRepairLoopInvocationEvidence`-Zustaende besitzen nun selbst die gemeinsame typisierte Evidence-Grenze. Sowohl `REPAIR-LOOP-INVOCATION-EVIDENCE` als auch die verschachtelte Ownership-Projektion lesen dieselben Objekte direkt.

Entfernt werden insbesondere:

- `report_line(...)`-Ruecksuche nach bereits projizierten Runtime-Zeilen,
- `active_render_writer_labels(...)` aus Reporttext,
- `MusicalDecisionOwner`-/`MaterializationOwner`-Zaehlen aus Reporttext.

Die bewusst isolierte `FinalReportProjection`-Producer-Grenze bleibt unveraendert. Es entsteht kein globaler Evidence-Bus und kein neuer Writer. Musikregeln, Passreihenfolge, Schwellenwerte, Write-Authority, MusicXML und PDF-Semantik bleiben unveraendert.

## Erwarteter Beweis

Die Ownership-Projektion muss reale ShadowRest-, ShortRestGlyph- und Staccato-Runtimewerte aus den typisierten Evidence-Objekten zeigen und darf keine `not_projected`-Platzhalter mehr aus einem leeren lokalen Reportpuffer erzeugen.

Abnahme:

```powershell
.\run.cmd --take --strict --all
```
