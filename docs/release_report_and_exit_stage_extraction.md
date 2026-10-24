# ReleaseReportAndExitStageExtraction

Stand: v0.10.1673 r01  
Ausgangspunkt: v0.10.1672 r01

## Ziel

Dieser `RefactorOnly`-Take setzt ausschließlich den dritten und letzten
vereinbarten Orchestrator-Rückbau um. Der verbleibende Abschlussabschnitt vom
kanonischen Master-MusicXML-Sink über Evidence-Ledger, PDF-Quellentscheidung,
Post-Render-Audits und Releaseblocker bis zu Finalreport, Result-Sink, Konsole
und Exitcode wird aus `real_main()` in
`run_release_report_and_exit_stage` verschoben.

Damit ist der vereinbarte Dreischritt abgeschlossen. Eine weitere physische
Dateiteilung, Clippy-Kosmetik oder Musikänderung ist nicht Teil dieses Takes.

## Neue Grenze

Die neue `ReleaseReportAndExitStageInputs`-Grenze erhält genau:

- die unveränderte CLI-Projektion,
- den bestehenden `SourcePlan`,
- den bestehenden Solver-Kandidaten,
- den vollständigen `InitialRenderTransaction`-Zustand aus Take 1,
- das vollständige `MetricBeamAndFinalNotationStageOutcome` aus Take 2.

Die 30 Take-2-Ergebnisse werden erst innerhalb der Abschluss-Stage vollständig
und explizit destructuriert. `real_main()` transportiert dadurch keine
Einzelaudits mehr und enthält nur noch die Phasenfolge.

## Verhaltensgleichheit

Der bisherige Abschlussblock wurde ohne inhaltliche Änderung verschoben. Der
byteweise Vergleich ergibt:

```text
Abschlussblock in v0.10.1672: 11.522 Bytes
Abschlussblock in neuer Stage: 11.522 Bytes
identisch: ja
```

Damit bleiben insbesondere unverändert:

- kanonischer Master-MusicXML-Write und Exportvalidierung,
- Reihenfolge und Beschriftung des Render-PDF-Evidence-Ledgers,
- PDF-Quellentscheidung und Rendereraufruf,
- Post-Render-Audit- und Raw-Cursor-Gates,
- Retention-, Status- und Fingerprint-Projektion,
- Releaseblocker und Finalreport-Projektion,
- Result-TXT-Sink, Konsolentext und Exitcode.

Die erste Kommentarzeile des eingeschlossenen
`final_report_projection_owner.rs` benennt lediglich den neuen lexikalischen
Owner. Der Include-Inhalt und seine Projektionen bleiben unverändert.

Es wird keine Musikregel, kein Schwellenwert, kein Writer, keine
`#[allow(...)]`-Direktive und keine Ausgabe hinzugefügt, entfernt oder
umgeordnet.

## Strukturgewinn

Physische Rust-Zeilen:

```text
real_main():                                      287 ->    64
neue ReleaseReportAndExit-Funktion:                       243
pipeline_orchestrator_report_compaction.rs:      3.755 -> 3.784
Rust gesamt unter src:                         117.602 -> 117.631
Netto Rust:                                                 +29
```

`real_main()` besteht damit nur noch aus Sondermoduswahl und sechs klaren
Projektphasen: Vorbereitung, Solver, Master-Notation, frühe
Render-Transaktion, metrische Finalnotation und Abschluss.

Die Zahl der `#[allow(...)]`-Direktiven unter `src` bleibt bei 25.

## Statische Prüfungen vor Paketierung

- Abschlussblock mit 11.522 Bytes byteidentisch,
- alle 30 Felder des Take-2-Outcomes vollständig destructuriert,
- Klammer-, String-, Zeichen- und Kommentargrenzen ausgeglichen,
- `real_main()` auf 64 physische Zeilen reduziert,
- keine neue Writer-Namensoberfläche,
- tatsächlicher Managed-Diff entspricht dem Manifest,
- alle deklarierten FNV64-Werte entsprechen den Paketdateien,
- keine Änderung an Take 1 oder Take 2 innerhalb ihrer Stage-Körper.

## Stopkriterium

Dieser Take endet nach der grünen Abnahme der Release-/Report-/Exit-Stage. Der
vereinbarte dreistufige Orchestrator-Rückbau ist dann abgeschlossen. Eine
weitere Zerlegung oder Clippy-Reduktion ist ein neuer, ausdrücklich zu
vereinbarender Arbeitsblock.

## Abnahme

Einmalig:

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```

Erwartet werden Build, Strict-Clippy, Program und FinalExit 0 sowie gegenüber
v0.10.1672 versionsnormalisiert identische Master-/Render-MusicXML und sechs
pixelidentische PDF-Seiten. Kein Korpuslauf und kein `--force`.
