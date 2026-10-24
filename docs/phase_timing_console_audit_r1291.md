# r1291 – Phasenzeiten in der Konsole (RefactorOnly)

Parent: accepted r1290. Klasse `RefactorOnly`.

## Zweck

Der Vollkorpus braucht 8:13, und bisher weiß niemand, wohin die Zeit geht. Vor dem Entfernen von Diagnose-Strängen (Pianistik-Kette, WAV-Audits, Notations-Audits) soll jede Entscheidung auf gemessenen Zeiten beruhen.

## Messpunkte

Jede Zeile hat die Form `ETUSCHLOPF: PhaseTiming: project=<Projekt>; stage=<Stufe>|section=<Abschnitt>; ms=<Millisekunden>` und geht auf stderr. Sie erscheint im Konsolen-Log (`etuschlopf.console.txt`) und nirgends sonst.

| Ebene | Messpunkte |
|---|---|
| Projektstufen | ProjectRunPreparation, ArrangementSolver, MasterNotationAndSalience, InitialRenderTransaction, MetricBeamAndFinalNotation, PhraseBoundaryEvidenceCapture, ReleaseReportAndExit |
| ArrangementSolver | InitialSearchState, GuidedSearch, AtomicPlayabilityRepair, SubstanceGesture |
| ReleaseReportAndExit | RenderPdfSourceSelection, PostRenderAuditAssembly, ReleaseGatesAndOptions, FinalReportProjection |
| Berichtsabschnitte (`write_text_report`) | rule_severity_authority, fünf WAV-Abschnitte (Sensor-Benchmark, Legacy-Oracle, Attack-Pulse, Harmonic-Evidence, Sensor-Boundary), phrase_boundary_evidence, format40_pianistic_chain, registernotation, octave_kind_boundary_evidence |

## Garantie

Keine Änderung an Reihenfolge, Inhalt oder Ergebnis irgendeiner Stufe; nur Zeitnahme und eine stderr-Zeile. Ergebnisdateien, Bericht, XML und PDF bleiben bytegleich.
