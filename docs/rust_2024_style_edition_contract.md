# Rust 2024 Style Edition Contract

## Ziel

Dieser SecurityOnly-Take schliesst nach dem real gruenen
v0.10.1815-r01-Watch-Rust-2024-Vertrag die letzte bewusst offene
Rust-2024-Projektkonfiguration: direkte rustfmt- und Editor-Aufrufe werden
explizit an dieselbe Sprach- und Style-Edition wie Cargo gebunden.

## Vertrag

Die neue Root-Datei `rustfmt.toml` enthaelt ausschliesslich:

```toml
edition = "2024"
style_edition = "2024"
```

`edition = "2024"` bindet direkte rustfmt-Aufrufe an die fuer das Projekt
autorisierte Parser-Edition. `style_edition = "2024"` macht die
Formatierungs-Style-Edition explizit und verhindert, dass Editor-/Direktaufrufe
eine implizite oder werkzeugabhaengige Style-Edition waehlen.

## Kein Formatierungs-Supertake

Dieser Take fuehrt absichtlich **kein** flaechenweites `cargo fmt` aus.
Es werden keine Rust-Quelldateien umformatiert und keine Imports,
Kommentare, Zeilenumbrueche oder sonstige Quelltextstruktur massenhaft
geaendert. Der Take etabliert nur den kuenftigen Formatierungsvertrag.

## Unveraendert

- Musik-, Writer-, Solver-, Repair-, Recovery- und Render-Semantik
- Launcher- und Runtime-Semantik
- Root-Dependencies und `Cargo.lock`
- `tools/etuschlopf_watch`
- Security-Scanner und Take-Change-Control
- normales Strict-Clippy-/Build-/Test-Gating

## Abnahme

```powershell
.\run.cmd --take "Etüde Nr. 6"
```
