# Security Gate Core Ownership and Compilation Closure

## Ziel

Die reine Rust-Quellmaskierungs- und FNV-1a-Implementierung erhaelt einen
eindeutigen Besitzer ausserhalb der Policy-Module. Die bestehenden
Security-/Build-Gate-Policies, Cargo-Grenzen und negativen Sicherheitschecks
bleiben unveraendert wirksam.

## Durch den realen Pre-Copy-Beweis geschaerfte Vertragsgrenze

Der installierte Semantic-Capability-Diff bindet negative Sicherheitschecks
nicht nur an Namen und Verhalten, sondern auch an ihre bisherigen Quellpfade.
Darum werden die Check-Eigentuemer in
`src/build_gate/rust_source_scan_primitives.rs`,
`src/identity_debt_inventory.rs` und
`tests/global_identity_debt_inventory.rs` nicht verschoben oder entfernt.

Der alte Primitive-Pfad ist ab r02 ein reiner, pfadstabiler
Negativtest-Vertrag. Er enthaelt keine zweite Implementierung, sondern bindet
die Implementierung aus
`tools/RUST_SECURITY_GATE_CORE/src/rust_source_scan_primitives.rs` und behaelt
die fuenf bestehenden negativen Scannerchecks am autorisierten Pfad.

## Besitzmatrix

| Flaeche | Besitzer | Verantwortung |
|---|---|---|
| Rust-Maskierung und FNV-1a | `tools/RUST_SECURITY_GATE_CORE/src/rust_source_scan_primitives.rs` | reine gemeinsame Implementierung |
| Scanner-Negativtestvertrag | `src/build_gate/rust_source_scan_primitives.rs` | pfadstabile Security-Checks, keine zweite Implementierung |
| Identity-Debt-Scan | `src/build_gate/identity_debt_scan.rs` | Identity-/Lokalisierungs-Policy |
| Semantic-Capability-Diff | `src/build_gate/semantic_capability_diff.rs` | Capability- und Change-Class-Policy |
| Take-Change-Control | `src/take_change_control.rs` | Manifest-, Diff- und Annahmepolicy |
| Productive-Write-Scan | `src/build_gate/productive_write_path_scan.rs` | Write-Surface-Policy |
| Thirty-Surface-Scan | `src/build_gate/thirty_surface_batch_scan.rs` | Batch-Gate-Policy |

## Vollstaendige Erfassung

Build-Signatur, Identity-Debt-Scan, Productive-Write-Scan und adversariale
Markerpruefung erfassen weiterhin beide Wurzeln:

```text
src
tools/RUST_SECURITY_GATE_CORE/src
```

Der Build pinnt Implementierung, pfadstabilen Negativtestvertrag und
Ownership-Vertrag. Er stoppt bei einer zweiten Funktionsimplementierung, bei
fehlender Pfadbindung oder bei entfernten negativen Sicherheitschecks.

## Bewusst nicht behauptet

Dieser Take reduziert die unvermeidbaren Kompilationsrollen noch nicht. Der
erste reale Pre-Copy-Lauf hat bewiesen, dass deren Testidentitaeten im aktuellen
Vertrag nicht innerhalb desselben Takes umziehen duerfen. Eine spaetere
Reduktion benoetigte zuerst eine explizite, eigenstaendige Relocation-Policy.
Sie wird hier weder eingeschmuggelt noch vorweggenommen.

## Nicht geaendert

Keine Aenderung an Musik, Solver, Cleaner, Repair, Polish, Readback, Kandidaten,
Schwellen, Writer, Passreihenfolge, MusicXML, PDF, Recovery, Rejection-Ledger,
Public-Artifact-Commit oder Konsolen-UX.
