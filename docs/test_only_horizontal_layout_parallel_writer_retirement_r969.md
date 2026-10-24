# r969 – TestOnlyHorizontalLayoutParallelWriterRetirement

## Zweck

r969 entfernt fünf historische StructuralWriteAuthority-Module, die in r968 ausschließlich hinter `#[cfg(test)]` eingebunden sind und deshalb in keinem Produktbuild existieren. Jedes dieser Module implementiert eine eigene, ausschließlich von den eigenen Modultests aufgerufene Layout-Writer-Mechanik. Es gibt keine produktiven, Tool-, Launcher- oder Build-Gate-Caller dieser APIs.

Entfernt werden:

- `final_short_system_break_neutralization.rs`
- `new_system_print_insertion.rs`
- `layout_system_break_suppression.rs`
- `measure_visual_width.rs`
- `page_break_layout_transaction.rs`

Zusammen mit ihren fünf `#[cfg(test)] mod ...`-Deklarationen entfallen 935 Rust-Zeilen und 20 Tests, die ausschließlich die jeweils test-only Parallelimplementierung prüfen.

## Lebender Owner bleibt unverändert

Die produktive horizontale Layout-Materialisierung bleibt vollständig in `horizontal_layout_plan_materialization.rs`. Dieser lebende source-bound Owner materialisiert Breite, `new-system="yes"`, `new-system="no"` und `new-page="yes"` in einem gemeinsamen Plan, bindet Source-, Directive- und Replacement-Fingerprint, validiert die finale temporale Materialisierung und besitzt eigene produktionsnahe Regressionstests einschließlich Idempotenz.

Die fünf pensionierten Module werden nicht durch neue Wrapper, Keepalives, `allow(dead_code)`, Registry-, Receipt- oder Proof-Schichten ersetzt.

## Schutzgrenze

Unverändert bleiben insbesondere:

- `horizontal_layout_plan_materialization.rs` und seine Tests,
- die produktive Layout-Planung und alle Directive-Erzeuger,
- MusicXML-/Render-/PDF-Entscheidungen,
- Kandidatenwahl, Scoring, Selection und musikalische Regeln,
- der test-only Negativvertrag `audit_only/structure_to_system_break_retirement_guard.rs`,
- alle Tool- und Qualified-Component-Crates,
- alle bestehenden Dead-Code-Suppression-Grenzen.

Der historische Eintrag `docs/semantic_owner_retirement_inventory_r723.tsv` bleibt unverändert als historische Inventur-Evidence; er besitzt keine Writer-, Delete- oder Release-Authority.

## Abnahme

ChangeClass: `RefactorOnly`.

Erwartung: keine Produktbytes ändern sich; der vollständige Korpus bleibt 10/10 grün. Der reale Windows-Compiler, Strict-Clippy, alle verbleibenden All-Target-Tests, Change-Control, Semantic-Capability-Diff und die bestehenden Gates bleiben autoritativ. Bei einem unerwarteten Delta oder Gate-Konflikt wird r969 verworfen statt durch Keepalive oder Folge-Reparaturschichten abgesichert.
