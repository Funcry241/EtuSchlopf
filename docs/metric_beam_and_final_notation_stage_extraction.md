# MetricBeamAndFinalNotationStageExtraction

Stand: v0.10.1672 r01  
Ausgangspunkt: v0.10.1671 r03

## Ziel

Dieser `RefactorOnly`-Take setzt ausschließlich den zweiten vereinbarten
Orchestrator-Rückbau um. Der zusammenhängende Abschnitt von der bestehenden
`RestAndArticulationRenderStage` über die scoreweiten metrischen Balken-,
Gould-, Register-, Tonart-, Tempo- und Layoutschritte bis einschließlich der
bereits vorhandenen `FinalRenderNotationAuthorityStage` wird aus `real_main()`
in `run_metric_beam_and_final_notation_stage` verschoben.

Take 3 – Release, Report und Exit – bleibt vollständig außerhalb des Scopes.

## Neue Grenze

Die neue Stage besteht aus:

- `MetricBeamAndFinalNotationStageInputs`,
- `run_metric_beam_and_final_notation_stage`,
- `MetricBeamAndFinalNotationStageOutcome`.

Sie erhält genau:

- den nur gelesenen `InitialRenderTransaction`-Zustand aus Take 1,
- den bestehenden `SourcePlan`,
- den bestehenden `ReferenceReport`.

Der `ReferenceReport` wird in die Stage verschoben, weil er nach dieser Phase
nicht mehr benötigt wird. Quelle und früher Renderzustand bleiben geborgt, da
sie in der nachfolgenden Master-/PDF-/Releasephase weiterverwendet werden.

Das Outcome transportiert ausschließlich die 30 bereits vorhandenen Audit-,
Writer-, Kandidaten- und Verifier-Ergebnisse, die der nachfolgende
PDF-Quellentscheid und die eingefrorene Final-Report-Projektion tatsächlich
benötigen.

## Verhaltensgleichheit

Der bisherige Produktionsblock wurde ohne inhaltliche Änderung verschoben. Der
byteweise Vergleich des Blockkörpers ergibt:

```text
Block in v0.10.1671: 21.980 Bytes
Block in neuer Stage: 21.980 Bytes
identisch: ja
```

Damit bleiben insbesondere unverändert:

- Reihenfolge aller Rest-, Artikulations-, Balken-, Gould-, Register-,
  Tonart-, Tempo-, Clef- und Finalnotation-Schritte,
- alle bestehenden Builder-, Resolver-, Materializer-, Verifier- und
  Writer-Aufrufe,
- Snapshot- und Fallback-Auswahl,
- Fail-Closed-Prüfung der Ganztaktpausen-Koordinaten,
- Writer-Pfade und Checkpoint-Reihenfolge,
- Fehlertexte, Verdicts und Pipeline-Texte,
- Master-/Render-MusicXML-, PDF-, Report- und Releaseverhalten.

Es wird keine Musikregel, kein Schwellenwert, kein Writer, keine
`#[allow(...)]`-Direktive und keine Ausgabe hinzugefügt, entfernt oder
umgeordnet.

## Strukturgewinn

Physische Rust-Zeilen:

```text
real_main():                                      601 ->   287
neue MetricBeamAndFinalNotation-Stage:                    393
pipeline_orchestrator_report_compaction.rs:      3.635 -> 3.755
Rust gesamt unter src:                         117.482 -> 117.602
Netto Rust:                                                +120
```

`real_main()` enthält nach diesem Take nur noch die grobe Phasenfolge, den
Master-MusicXML-Sink, PDF-Quellentscheidung, Post-Render-Audit sowie die
unveränderte Release-/Report-/Exit-Phase.

Die Zahl der `#[allow(...)]`-Direktiven unter `src` bleibt bei 25.

## Statische Prüfungen vor Paketierung

- verschobener Produktionsblock byteidentisch,
- alle 30 nachgelagert benötigten lokalen Werte im Stage-Outcome enthalten,
- Outcome-Felder, Rückgabeinitialisierung und Destructuring vollständig,
- Klammer-, String-, Zeichen- und Kommentargrenzen ausgeglichen,
- tatsächlicher Managed-Diff entspricht dem Manifest,
- keine neue Writer-Namensoberfläche,
- keine Änderung innerhalb von Take 1 oder Take 3,
- alle deklarierten FNV64-Werte entsprechen den Paketdateien.

## Stopkriterium

Dieser Take endet nach der Extraktion der metrischen Balken- und
Finalnotationsphase. Die Release-/Report-/Exit-Phase wird nicht im selben Take
umgebaut. Nach erfolgreicher Abnahme wird bewusst gestoppt.

## Abnahme

Einmalig:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartet werden Build, Strict-Clippy, Program und FinalExit 0 sowie gegenüber
v0.10.1671 versionsnormalisiert identische Master-/Render-MusicXML und sechs
pixelidentische PDF-Seiten. Kein Korpuslauf und kein `--force`.
