# Rust2024LetChainStructuralClosureSuperTake

Stand: v0.10.1809 r01

## Ausgangspunkt

Basis ist der real gruene v0.10.1808-r04-Stand mit unconditional Strict-Baseline,
aktuellem blockierendem Strict-Clippy und 138/138 gruenen Launcher-Tests. Die
vorbereitete, aber nie als Baseline abgenommene v0.10.1808-r01-Wave wird nicht
uebernommen; diese Fassung wird neu aus r04 aufgebaut.

Der zu r04 gehoerende Deep-Clippy meldet 96 `collapsible_if`-Stellen. Dieser
Super-Take bearbeitet davon ausschliesslich neun bereits voranalysierte
semantikneutrale Read-/Collector-Bedingungen.

## Scope

### Metrische Read-only-Map-Builder

In `render_polish_hidden_rest_readability_tail.rs` werden sechs verschachtelte
`if let Some(value)` + `value > 0`-Bedingungen fuer `divisions`, `beats` und
`beat-type` in zwei metrischen Read-only-Helfern zu Rust-2024-Let-Chains
linearisiert. Parse-Reihenfolge, Positivitaetspruefung, Defaultwerte und
Map-Inhalte bleiben identisch.

### Staff-/MIDI-Collector

In `staff_voice_atomic_move_patch_core.rs` werden im reinen Collector
`collect_measure_staff_notes` drei optionale Filter fuer Staff, Mindest-MIDI und
Maximal-MIDI zu Rust-2024-Let-Chains linearisiert. Die Funktion sammelt weiterhin
nur Referenzen; sie mutiert weder Noten noch XML.

## Harte Nicht-Ziele

Unveraendert bleiben insbesondere:

- Musik-, Solver- und Kandidatensemantik,
- Writer- und Structural-Write-Authorities,
- Repair-/Recovery-Schichten,
- Passorder, Kosten, Schwellen und Regeln,
- Security-/Build-/Launcher-Pins und Trust-Fingerprints,
- Dependencies, Toolchain und Edition,
- Master-/Render-/PDF-Artefaktsemantik.

Es werden keine Deep-Clippy-Funde aus `suspicious_operation_groupings`, Casts,
Ownership, grossen Funktionszerlegungen oder gepinnten Security-Pfaden vermischt.

## Diagnoseerwartung

Da exakt neun aktuelle `collapsible_if`-Fundstellen ersetzt werden, ist bei
unveraenderter Clippy-Diagnostik ein Rueckgang von 96 auf ungefaehr 87 plausibel.
87 ist ausdruecklich keine Release-Bedingung und erst der reale Deep-Clippy nach
gruener Installation ist autoritativ.

## Abnahme

Die Strict-Basis ist unconditional. Fuer die erste Abnahme genuegt nach der
Ein-Befehl-Regel:

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Der Take muss Build, aktuelles blockierendes Strict-Clippy und das integrierte
Launcher-Testgate bestehen. Ein fehlgeschlagener Gate-Lauf darf die Installation
nicht als neuen gruenen Stand etablieren.
