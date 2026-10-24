# OrchestratorSupportModuleDecomposition

## Revision r02 – Boundary Compile Closure

Der reale r01-Strict-Lauf zeigte drei falsch getrennte Attribut-/Item-Grenzen. r02 schneidet die fünf Supportsegmente erneut aus `0.10.1675`, sodass `#[derive(Debug)]` bei `RenderPdfSourceCandidate`, `#[derive(Debug)]` bei `ArrangementSolverStageOutcome` und `#[derive(Debug, Clone)]` bei `LowerClefSurfaceMaterializationAudit` jeweils wieder Bestandteil desselben physischen Segments wie ihr Struct sind. Damit werden zugleich die drei Parserfehler und die zwei daraus folgenden fehlenden `Debug`-Implementierungen geschlossen. Keine produktive Logik ändert sich.

## Ziel

Der historische Sammelpfad `pipeline_orchestrator_report_compaction.rs` wird auf seine eigentliche Aufgabe reduziert: stabiler Pfadanker und lesbarer Ablauf in `real_main()`.

## Physische Supportgrenzen

- `pipeline_orchestrator_project_setup_stages.rs`
- `pipeline_orchestrator_arrangement_solver_stage.rs`
- `pipeline_orchestrator_final_notation_support.rs`
- `pipeline_orchestrator_release_support.rs`
- `pipeline_orchestrator_clef_layout_support.rs`

Zusammen mit den drei Phasendateien aus Take 1 besitzt der Orchestrator damit acht private, fachlich benannte Include-Einheiten. Alle bleiben im selben lexikalischen `pipeline`-Namensraum; keine Sichtbarkeit wird erweitert.

## Strukturwirkung

- Der Orchestrator-Index enthält nur `real_main()` und die privaten Include-Grenzen.
- Die fünf verschobenen Supportblöcke sind byteidentisch zum Stand `0.10.1675`.
- Der größte neue Orchestrator-Baustein bleibt unter 700 physischen Zeilen.
- Der bestehende Pfad für produktive Writer-Inventare, Change-Control und historische Baselines bleibt erhalten.

## Messbare Strukturwirkung

- Orchestrator-Index: 2.511 auf 84 physische Zeilen
- Supportdateien: 191, 602, 583, 411 und 650 Zeilen
- größte Orchestrator-Einheit: 650 Zeilen
- gesamter Orchestrator-Dateiverbund: 3.769 Zeilen
- Rust gesamt gegenüber `0.10.1673`: -15 physische Zeilen
- `#[allow(...)]`: unverändert 25

## Verhaltenstreue

Writer, Passreihenfolge, Musikregeln, Schwellenwerte, XML- und PDF-Pfade, Reports, Releaseentscheidungen, Fehlertexte und Allow-Direktiven bleiben unverändert.

## Stopkriterium

Nach der Support-Zerlegung wird keine weitere lokale Clippy- oder Funktionsbereinigung begonnen. Der dreiteilige Schönheitssuperwurf endet hier.
