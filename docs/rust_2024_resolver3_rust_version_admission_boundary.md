# Rust2024Resolver3RustVersionAdmissionBoundary

Stand: v0.10.1811 r01

## Ausgangspunkt

Basis ist der real grüne v0.10.1810-r01-Stand nach erfolgreicher Installation der
Rust-2024-Makro-Expression-Fragment-Closure. Der produktive Quellstand verwendet
bereits Edition 2024 und seit v0.10.1809 echte Let-Chains.

## Warum dieser Take vorgeschaltet ist

Der bestehende Take-Change-Control ist absichtlich fail-closed: `Cargo.toml` darf
normalerweise ausschließlich in der package-Version geändert werden. Die einzige
bisherige SecurityOnly-Sonderzulassung ist der bereits abgeschlossene exakte
Edition-Wechsel 2021 -> 2024.

Darum darf ein neuer `rust-version`-/Resolver-Vertrag nicht im selben Take
"durch das eigene neue Gate" legitimiert werden. Der aktuelle v0.10.1810-Bootstrap
würde diese Manifeständerung vor jeder Source-Mutation korrekt ablehnen.

Dieser Take erweitert daher ausschließlich die Zulassungslogik. Der eigentliche
Cargo-Vertrag wird erst in einem nachfolgend auf v0.10.1811 grün basierenden Take
gesetzt.

## Neue exakt geschlossene Zulassung

`take_change_control.rs` kennt ab diesem Stand den Marker

```text
rust_2024_resolver_contract=closed_resolver3_msrv_1_88_v1
```

Er ist nur für `SecurityOnly` zulässig und akzeptiert ausschließlich den Übergang

```toml
[package]
version = "<alt>"        # -> neue EtuSchlopf-Version
edition = "2024"
rust-version = "1.88"
resolver = "3"
```

unter folgenden zusätzlichen Bedingungen:

- der bisherige Stand besitzt weiterhin `edition = "2024"`,
- `rust-version` und `resolver` fehlen im bisherigen Root-Manifest,
- der eingehende Stand setzt exakt `rust-version = "1.88"`,
- der eingehende Stand setzt exakt `resolver = "3"`,
- außer package-Version plus diesen beiden Feldern bleibt das Manifest byte-/strukturäquivalent,
- der Marker darf nicht mit dem historischen Editionswechsel-Marker kombiniert werden.

Abweichungen wie Resolver 2, MSRV 1.85/1.89, Dependency-Änderungen oder sonstige
Cargo-Manipulationen bleiben `Other` und damit fail-closed.

## Sicherheitskoordination

Da `take_change_control.rs` byte-gepinnte Security-Architektur ist, werden die
zugehörigen FNV-Vertrauensanker koordiniert nachgezogen:

- `src/identity_debt_build.rs`,
- `src/identity_gate_trust_anchor.rs`.

Der Take ändert keine Dependency-Version, keine Toolchain-Datei, kein `Cargo.lock`,
keine Musik-, Writer-, Solver-, Repair-, Recovery-, Launcher- oder Passorder-Semantik.

## Nachfolgender Schritt

Erst nach real grüner Abnahme von v0.10.1811 darf der nächste SecurityOnly-Take
den neuen Marker benutzen und `rust-version = "1.88"` sowie `resolver = "3"`
tatsächlich in `Cargo.toml` setzen. Damit bleibt die Ein-Befehl-/Pre-Copy-
Sicherheitsarchitektur vollständig selbstkonsistent.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Der normale Strict-Pfad muss Build, blockierendes Strict-Clippy und die integrierten
Launcher-/Security-Gates bestehen. Deep-Clippy bleibt Diagnose.
