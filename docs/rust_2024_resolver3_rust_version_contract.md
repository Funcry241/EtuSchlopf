# Rust2024Resolver3RustVersionContract

Stand: v0.10.1812 r01

## Ausgangspunkt

Basis ist der real gruene v0.10.1811-r01-Stand
`Rust2024Resolver3RustVersionAdmissionBoundary`.

v0.10.1811 hat den fail-closed Take-Change-Control um genau eine geschlossene
SecurityOnly-Zulassung erweitert. Diese Zulassung darf einmalig den Root-
`Cargo.toml` von Edition 2024 ohne expliziten Resolver/MSRV-Vertrag auf exakt
Edition 2024 plus Resolver 3 plus Rust-Version 1.88 heben.

## Produktiver Cargo-Vertrag

Der Root-Manifestvertrag lautet ab diesem Take:

```toml
[package]
edition = "2024"
rust-version = "1.88"
resolver = "3"
```

Die Versionsnummer wird parallel regulaer von `0.10.1811` auf `0.10.1812`
fortgeschrieben.

`rust-version = "1.88"` beschreibt die deklarierte Mindest-Rust-Version des
aktuellen EtuSchlopf-Quellbestands. Der produktive Source verwendet seit
v0.10.1809 echte Let-Chains; deshalb wird der Vertrag nicht auf das niedrigere
reine Edition-2024-Minimum gesetzt.

`resolver = "3"` macht den Rust-2024-Resolververtrag explizit. Die
Dependency-Deklarationen selbst bleiben bytegleich.

## Autorisierung

Der bereits in v0.10.1811 real-gruen etablierte Take-Controller akzeptiert
diesen Manifestwechsel nur mit:

```text
change_class=SecurityOnly
rust_2024_resolver_contract=closed_resolver3_msrv_1_88_v1
```

Der Marker akzeptiert ausschliesslich:

- bisher `edition = "2024"` ohne `rust-version` und ohne `resolver`,
- eingehend weiterhin `edition = "2024"`,
- eingehend exakt `rust-version = "1.88"`,
- eingehend exakt `resolver = "3"`,
- normale Versionsfortschreibung,
- keine weiteren Cargo-/Dependency-Aenderungen.

Damit autorisiert v0.10.1812 sein Cargo-Manifest nicht selbst. Die notwendige
Zulassungslogik wurde bereits vorher separat installiert und gruen abgenommen.

## Nicht geaendert

Dieser Take aendert insbesondere nicht:

- Dependency-Versionen oder Features,
- Root-`Cargo.lock` im Take-Paket,
- Rust-Launcher-/Bootstrap-Logik,
- Security-Gate-Quellcode,
- Musik-, Writer-, Solver-, Repair-, Recovery- oder Passorder-Semantik,
- Render-, PDF- oder Artifact-Semantik.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Der normale Strict-Pfad bleibt autoritativ. Deep-Clippy bleibt
nicht-blockierende Diagnostik.
