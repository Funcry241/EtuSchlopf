# WatchRust2024Resolver3RustVersionContract

Stand: v0.10.1815 r01

## Ausgangslage

Die Watch ist ein eigenstaendiges Cargo-Projekt unter
`tools/etuschlopf_watch/` mit eigenem `Cargo.toml` und eigenem `Cargo.lock`.

Der Root-Crate von EtuSchlopf ist bereits auf Edition 2024, `rust-version = "1.88"`
und Resolver 3 geschlossen. Die Watch blieb bislang bewusst auf Edition 2021.

Der erste direkte Vertragskandidat v0.10.1814-r01 wurde auf dem realen
Windows-Zielsystem vor jeder Installation korrekt abgewiesen, weil
`semantic_capability_diff` das Watch-Manifest noch ueber den allgemeinen `tools/`-
Fallback als Musik-Domaene klassifizierte. v0.10.1814-r02
`WatchCargoSemanticSecurityDomainClosure` schloss ausschliesslich diese Domaenenluecke
und wurde danach real `dark_green` / `released` abgenommen.

Damit sind nun beide voneinander unabhaengigen Schutzgrenzen konsistent:

- `take_change_control` behandelt das Watch-Manifest und Watch-Lockfile als Security.
- `semantic_capability_diff` behandelt exakt dieselben beiden Pfade als Security.
- der bereits in v0.10.1813 etablierte geschlossene Marker erlaubt nur den exakten
  Watch-Manifestuebergang 2021 -> 2024 / MSRV 1.88 / Resolver 3.

## Aenderung in v0.10.1815

Dieser Take benutzt erstmals die bereits in frueheren real gruenen Takes
vorbereiteten Grenzen und aendert genau das Watch-Manifest:

```toml
[package]
name = "etuschlopf-watch"
version = "0.10.1737"
edition = "2024"
rust-version = "1.88"
resolver = "3"
```

Unveraendert bleiben:

- Watch-Package-Version `0.10.1737`
- alle Watch-Rust-Quellen
- Watch-Dependencies und Profile
- `tools/etuschlopf_watch/Cargo.lock`
- Root-Dependencies und Root-Cargo-Vertrag
- Musik-, Writer-, Solver-, Repair-, Recovery-, Passorder-, Render- und Launcher-Semantik

Der Root-`Cargo.toml` aendert nur die EtuSchlopf-Version
`0.10.1814 -> 0.10.1815`.

## Security-Vertrag

ChangeClass: `SecurityOnly`

Der Take deklariert:

```text
watch_rust_2024_resolver_contract=closed_watch_2021_to_2024_resolver3_msrv_1_88_v1
```

Zulaessig ist damit nur der exakte vorhandene 2021->2024-Schritt mit
`rust-version = "1.88"` und `resolver = "3"`. Andere Watch-Manifest-, Dependency-,
Feature- oder Lockfile-Aenderungen bleiben fail-closed.

Da das separate Watch-Projekt geaendert wird, muss der Rust-Take-Bootstrap es vor der
Installation mit dem vorhandenen Watch-Projekt-Gate kompilieren und testen. Erst ein
erfolgreicher realer Windows-Reexec macht v0.10.1815 zur neuen Baseline.

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```

Der normale Strict-Pfad bleibt die Freigabeautoritaet.
