# r374 RustSourceScanPinnedFacadeImportWarningPinOwnerClosure

Stand: EtuSchlopf 0.10.1839, Parent r372 `HiddenChordToneRecoveryAudioSalienceDecisionOwnerClosure` real DARK GREEN.

r373 wurde vom Take-Change-Control vor dem Kopieren fail-closed abgelehnt, weil `source_integrity_security.rs` als gepinnter Architekturpfad geaendert war, aber `security_build_orchestrator.rs` nicht als synchronisierter Pin-Eigentuemer im Diff stand. r374 basiert deshalb weiterhin direkt auf r372 und behaelt den fachlichen Warning-Cleanup unveraendert; es schliesst ausschliesslich die vorgeschriebene Pin-Eigentuemer-Koordination.

## Befund

Der Windows-`--take`-Bootstrap meldet vor jedem eigentlichen Take-Lauf:

```text
warning: unused imports: `FNV1A64_OFFSET_BASIS` and `fnv1a64_update`
 --> src\bin\..\build_gate\rust_source_scan_primitives.rs
```

Das ist kein toter produktiver Owner und kein Anlass fuer `#[allow(unused_imports)]`.
Die beiden Symbole bleiben fuer andere autorisierte Compile-Units produktiv noetig,
insbesondere fuer `productive_write_path_scan`. Nur die schmalere
Take-Bootstrap-Compile-Unit konsumiert sie nicht.

## Closure

Die bestehende path-stabile Negative-Check-Fassade

```text
src/build_gate/rust_source_scan_primitives.rs
```

behaelt denselben einzigen Implementierungsowner:

```text
tools/RUST_SECURITY_GATE_CORE/src/rust_source_scan_primitives.rs
```

und dieselbe gepinnte oeffentliche Basissurface. Statt vier einzelne Symbole
explizit zu importieren/re-exportieren, projiziert die Fassade den vollstaendigen
bytegepinnten Public-Surface des Owners als genau einen Import:

```rust
pub use implementation::*;
```

Der Implementierungsowner selbst bleibt byteidentisch und ist weiterhin durch
`PINNED_RUST_SOURCE_SCAN_PRIMITIVES_FNV1A64` gebunden. Damit kann ein spaeteres
neues Public-Symbol nicht unbemerkt ueber die Glob-Projektion erscheinen: jede
Owner-Aenderung bricht weiterhin fail-closed die SourceIntegrity-Pin-Policy.

Die SourceIntegritySecurity wird auf diese exakte Fassadenform umgestellt und
deren eigener Trust-Anchor-Pin aus den finalen Bytes neu gebunden.

## Nicht geaendert

- keine Warning-Suppression
- kein `allow`/`expect`
- kein Fake-Use oder Dummy-Read
- kein neuer Scanner oder zweiter FNV-Owner
- keine Source-Fact- oder Security-Policy-Aenderung
- keine Musik-, Writer-, Pass-, Repair- oder Recovery-Aenderung
- keine Cargo-/`Cargo.lock`-Semantik
- keine Aenderung der produktiven Scannerergebnisse

`ChangeClass=SecurityOnly`.

Erwarteter Effekt: Der Take-Bootstrap kompiliert ohne die beiden
`unused_imports`-Warnungen; alle bisherigen Scanner- und Security-Vertraege
bleiben semantisch unveraendert.


## Pin-Eigentuemer-Closure r374

Da `src/build_gate/source_integrity_security.rs` gepinnte Architektur ist, werden die beiden bestehenden Pin-Eigentuemer `src/security_build_orchestrator.rs` und `src/identity_gate_trust_anchor.rs` in derselben SecurityOnly-Transition explizit synchronisiert. Der Build-Policy-Fingerprint im Trust Anchor wird ausschliesslich aus den finalen Bytes des Orchestrators neu gebunden. Keine Gate-Semantik wird gelockert oder erweitert.
