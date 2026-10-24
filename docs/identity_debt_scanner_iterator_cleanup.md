# IdentityDebtScannerIteratorCleanup

## Scope

Der Take veraendert genau sechs physische Stellen in
`src/build_gate/identity_debt_scan.rs`:

- zweimal `or_insert(...)` zu `or_insert_with(...)`,
- einmal `Iterator::all` mit direkter `u8::is_ascii_digit`-Referenz,
- zweimal `take_while` mit direkter `char::is_ascii_digit`-Referenz,
- einmal `map(...).unwrap_or(0)` zu `map_or(0, ...)`.

Die beiden `or_insert_with`-Aenderungen vermeiden einen unnoetigen Clone des
Evidenz-Strings, wenn der Schluessel bereits vorhanden ist. Die vier anderen
Aenderungen sind reine Iterator-/Option-Schreibweisen mit identischen
Argumenten, Rueckgabewerten und Auswertungsreihenfolgen.

## Security-Bindung

Der Identity-Debt-Scanner ist compile-time gepinnt. Eine Aenderung seiner Bytes
muss deshalb koordiniert erfolgen:

1. neuer Scanner-FNV in `src/identity_debt_build.rs`,
2. derselbe Scanner-FNV in `src/identity_gate_trust_anchor.rs`,
3. neuer Build-Policy-FNV des dadurch geaenderten Build-Scripts ebenfalls im
   Trust Anchor.

Die Change-Class ist daher `SecurityOnly`, nicht `MechanicalOnly`. Es wird
keine Security-Regel gelockert, entfernt oder hinzugefuegt. Die leere
Identity-Debt-Baseline, die Scannerklassifikation und alle negativen Tests
bleiben unveraendert.

## Begrenzung

Nicht Teil dieses Takes sind weitere Scannerdateien, Dateiendungsvergleiche,
`missing_const_for_fn`, `too_many_lines`, Formatbereinigungen, Musiklogik,
Writer, Reports oder neue `allow`-Direktiven.

## Abnahme

```powershell
.\run.cmd --take --strict "Etüde Nr. 4"
```
