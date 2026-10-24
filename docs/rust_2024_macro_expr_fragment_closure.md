# Rust2024MacroExprFragmentClosure

Stand: v0.10.1810 r01

## Ausgangspunkt

Basis ist der real grüne v0.10.1809-r01-Stand nach erfolgreicher Installation des
`Rust2024LetChainStructuralClosureSuperTake`. Der zugehörige Deep-Clippy-Lauf
meldet CargoExit 0 und 1817 Diagnosewarnungen; gegenüber der direkten
v0.10.1808-Basis sind damit exakt die neun im Vorgänger-Take bearbeiteten
`collapsible_if`-Fundstellen verschwunden.

## Ziel

Rust 2024 erweitert den `macro_rules!`-Fragmenttyp `expr` um den vollständigen
2024-Ausdrucksraum einschließlich `const { ... }`- und `_`-Ausdrücken. Die
Edition-Migration hatte in vier EtuSchlopf-Makros noch insgesamt acht explizite
`expr_2021`-Matcher konserviert. Dieser Take schließt genau diese verbleibende
Edition-2021-Grenze.

## Scope

Geändert werden ausschließlich die Matcher der folgenden Makros:

- `define_replacement_authority`: 1 x `expr_2021` -> `expr`,
- `observed_project_try`: 1 x `expr_2021` -> `expr`,
- `observed_corpus_try`: 1 x `expr_2021` -> `expr`,
- `define_validator_issue_authority`: 5 x `expr_2021` -> `expr`.

Die Makros besitzen an diesen Stellen keine konkurrierenden `const`- oder
`_`-Arme. Bestehende Aufrufe behalten deshalb ihre bisherige Matcher-Auswahl.

Als echte Compile-Probe wird der bereits vorhandene, konstante
`VisibleLaneEffect::BackupAnchor`-Ausdruck in einem bestehenden Aufruf als
`const { VisibleLaneEffect::BackupAnchor }` formuliert. Der expandierte Wert und
die Laufzeitsemantik bleiben identisch; gleichzeitig kann dieser Aufruf mit
`expr_2021` nicht mehr gematcht werden und beweist damit beim normalen Build,
dass der reale EtuSchlopf-Makrogrenzpunkt die Rust-2024-`expr`-Semantik nutzt.

Zusätzlich schützt der Integrationstest
`rust_2024_macro_expr_fragment_contract.rs` die vier konkreten Matcher gegen
unbeabsichtigte Rückkehr auf `expr_2021` und prüft den `const`-Probeaufruf.

## Harte Nicht-Ziele

Unverändert bleiben insbesondere:

- Musik-, Solver-, Kandidaten- und Writer-Semantik,
- Structural-Write-Authority-Zuständigkeiten,
- Repair-/Recovery-Schichten,
- Passorder, Kosten, Schwellen und Regeln,
- Security-/Build-/Launcher-Pins und Trust-Fingerprints,
- Dependencies, Toolchain und Cargo-Edition,
- Master-/Render-/PDF-Artefaktsemantik.

Andere Deep-Clippy-Gruppen werden in diesem Take nicht bearbeitet.

## Abnahme

Nach der Ein-Befehl-Regel:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Der normale Strict-Pfad muss Build, blockierendes Strict-Clippy und die
integrierten Tests bestehen. Deep-Clippy bleibt Diagnose und ist kein
Release-Gate.
