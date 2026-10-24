# Rust 2024 Macro Semantics and All-Targets Compile Closure

## Scope

Dieser Take ist ein rein technischer, verhaltensinvarianter Zwischenschritt auf Edition 2021. Er schliesst nur die vom realen `--clippy-deep`-Lauf nachgewiesenen nicht-securityseitigen Rust-2024-Migrationsbefunde.

## Macro-Semantik

Acht `macro_rules!`-Matcher verwendeten `$...:expr`. In Edition 2024 erweitert sich die von `expr` akzeptierte Grammatik. Um die bisherige 2021-Semantik explizit zu konservieren, werden ausschliesslich die vom Compiler markierten Matcher auf `$...:expr_2021` gestellt. Damit ist der spaetere Editionswechsel fuer diese Macros semantisch geschlossen, ohne neue Match-Faelle einzufuehren.

Betroffene Stellen:

- `src/rule_id.rs` – 5 Matcher
- `src/structural_write_authority/strict_staff_visible_lane_authority.rs` – 1 Matcher
- `src/pipeline_shards/pipeline_orchestrator_report_compaction.rs` – 1 Matcher
- `src/pipeline_shards/rest_glyph_retention_batch_corpus_tail.rs` – 1 Matcher

## All-Targets Compile Closure

`cargo clippy --all-targets` kompilierte zusaetzlich den Testpfad des Main-Binaries und fand zwei E0425-Fehler in `composer_beat_scoped_rest_completion_planning_core.rs`. Beide Aufrufe gehoeren zum selben Test und verwenden den bereits im Parent-Modul vorhandenen Helper `beat_scoped_rest_completion_pitched_timeline`. Das verschachtelte Testmodul hatte diesen Helper lediglich nicht in seiner `use super::{...}`-Liste.

Der fachliche r01-Payload ergaenzt nur diesen Import. Die Helper-Implementierung selbst bleibt bytegleich. r02 ist ausschliesslich die Paketintegritaets-Closure nach fail-closed Ablehnung eines versehentlich mitgelieferten Root-`Cargo.lock`; der fachliche Source-Payload bleibt identisch.

## Nicht Bestandteil

- kein Editionswechsel
- keine Dependency-Aenderung
- keine Musikmutation
- keine Writer-Aenderung
- keine neue Repair-/Recovery-Authority
- kein Clippy-Massencleanup
- kein Launcher-Drop-Order-Fix; dieser bleibt fuer den SecurityOnly-Editions-Take

## Erwarteter Nachweis

1. `--take --strict --all` bleibt 10/10 dunkelgruen.
2. `--clippy-deep` erreicht CargoExit 0 fuer alle Targets.
3. Die acht `edition_2024_expr_fragment_specifier`-Befunde sind verschwunden.
4. Als verbleibender Rust-2024-Kompatibilitaetsbefund darf nur der bekannte Launcher-Tail-Drop-Order-Fall verbleiben.
