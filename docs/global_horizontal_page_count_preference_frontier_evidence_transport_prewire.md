# GlobalHorizontalPageCountPreferenceFrontierEvidenceTransportPrewire

## Zweck

Dieser Same-Version-r03 ist ausschliesslich ein verhaltensneutraler Refactor-Prewire.
Der reale r02-Kandidat wurde vom Semantic-Capability-Diff korrekt abgewiesen, weil
`OutcomeProjection` einen neuen writer-reachable Pfad aus
`pipeline_orchestrator_report_compaction.rs` erzeugt haette. Das Gate bleibt
unveraendert.

## Architektur

Die bereits vorhandenen Result-Writer werden nicht erweitert und kein neuer Writer
wird eingefuehrt. Stattdessen wird ein audit-only Transportadapter vorbereitet:

- im deaktivierten r03 liefert `batch_child_stdout()` weiterhin exakt `Stdio::null()`,
- `BatchChildShadowCapture` liefert im deaktivierten Zustand immer `None`,
- bestehende Batch-Failure- und Missing-Output-Writer besitzen nur optionale, aktuell
  unerreichbare Evidence-Hooks,
- die bestehende FinalReport-Projektion besitzt denselben optionalen, aktuell
  deaktivierten Hook,
- Planner, Reflow, PageCount, Safety-Gates, MusicXML, PDF und Exit-Semantik bleiben
  unveraendert.

Damit ist r03 `RefactorOnly` ehrlich: Die neue Verdrahtung ist vorhanden, aber die
Projekt- und Korpusartefakte bleiben unveraendert. Eine spaetere Revision darf
ausschliesslich im `src/audit_only/`-Adapter die Outcome-Projektion aktivieren, ohne
den Music-domain Writergraph erneut anzufassen.


## r06 transactional install clarification

The prewire itself remains behavior-neutral and hard-disabled. A take transaction commits only when its automatic reexec returns exit 0. Because the current 10-project corpus intentionally returns exit 1 while seven projects fail closed, this prewire must be installed through a known-green project run first; running `--take --all` would correctly roll the source transaction back and must not be used as the commit mechanism for this bridge.
