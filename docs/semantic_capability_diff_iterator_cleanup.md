# SemanticCapabilityDiffIteratorCleanup

## Ausgangslage

Der bestaetigte v0.10.1665-Deep-Clippy-Lauf meldet 1.716 Diagnosen. Gegenueber
v0.10.1663 sind exakt 28 Diagnosen entfallen:

- `redundant_closure_for_method_calls`: 73 -> 53,
- `or_fun_call`: 13 -> 8,
- `map_unwrap_or`: 28 -> 25.

Keine andere Warnfamilie hat sich veraendert.

## Scope

Dieser Take veraendert genau sieben physische Stellen in
`src/build_gate/semantic_capability_diff.rs`:

- dreimal `u8::is_ascii_whitespace` als direkte Methodenreferenz,
- einmal `u8::is_ascii_digit` als direkte Methodenreferenz,
- einmal `u8::is_ascii_uppercase` als direkte Methodenreferenz,
- zweimal lokale `Option`-Grenzpruefungen als `map_or`.

Dieselben Quellbytes werden in zwei All-Targets-Kompilationsidentitaeten
verwendet. Deshalb entsprechen sieben physische Stellen vierzehn
Deep-Clippy-Diagnosen.

## Security-Bindung

Der Semantic-Capability-Diff-Scanner ist compile-time gepinnt. Seine
Byteaenderung wird daher koordiniert gebunden:

1. neuer Scanner-FNV in `src/identity_debt_build.rs`,
2. derselbe Scanner-FNV im Trust Anchor,
3. neuer Build-Policy-FNV des dadurch geaenderten Build-Scripts ebenfalls im
   Trust Anchor.

Die Change-Class ist `SecurityOnly`. Die Capability-Klassifikation,
Pfadrollen, Change-Class-Regeln, Fehlermeldungen, Security-Entscheidungen,
Musiklogik, Writer, Schwellenwerte und `allow`-Direktiven bleiben unveraendert.

## Begrenzung

Nicht Teil dieses Takes sind Dateiendungsvergleiche,
`needless_pass_by_value`, `missing_const_for_fn`, `too_many_lines`, weitere
Scannerdateien oder eine neue Abstraktion.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```
