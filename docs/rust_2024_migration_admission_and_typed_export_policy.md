# Rust 2024 Migration Admission and Typed Export Policy

## Zweck

Dieser Take ist absichtlich **noch keine Edition-Umstellung**. `Cargo.toml` bleibt auf `edition = "2021"`. Er schafft nur die technisch notwendige, fail-closed vorbereitete Basis fuer einen spaeteren separaten Wechsel auf Rust Edition 2024.

## Reale Migrationsbefunde aus v0.10.1796 r06

Der autoritative Source-Snapshot wurde auf bekannte Rust-2024-Migrationsgrenzen untersucht. Dabei wurden insbesondere `gen` als neues Keyword, Unsafe-Attribute/Extern-Bloecke, `static mut`, Prozess-Environment-Mutationen und Macro-`expr`-Fragmente betrachtet.

Der unmittelbar harte Befund ist der Launcher-Aufruf `env::set_var("ETUSCHLOPF_ASSISTANT_EXPORT_PDF", "1")`. In Rust 2024 ist `std::env::set_var` eine unsafe Operation. EtuSchlopf verbietet `unsafe_code`; der Editionswechsel darf dieses Verbot nicht aufweichen.

Der Exportwunsch liegt bereits als `AssistantExportPolicy` im typisierten `ExecutionRequest` vor. Dieser Take transportiert dieselbe Policy deshalb direkt ueber `RunState` und `AssistantExportRunStatus` in den Visual-Preflight-Exporter. Der bisherige Prozess-global mutierte Environment-Schalter und sein spaeteres Ruecklesen entfallen. Es entsteht keine neue globale Wahrheit und kein Unsafe-Block.

## Take-Change-Control

Der bisherige Vertrag erlaubt in `Cargo.toml` fuer alle bekannten ChangeClasses ausschliesslich die package-Version. Das ist korrekt und blockiert auch einen Editionswechsel fail-closed.

Dieser Take oeffnet **keine allgemeine Cargo.toml-Freigabe**. Stattdessen wird genau ein expliziter Marker eingefuehrt:

```text
rust_edition_transition=closed_2021_to_2024_v1
```

Er ist nur fuer `SecurityOnly` gueltig und autorisiert ausschliesslich die exakte Kombination:

```text
package.version: geaendert
package.edition: 2021 -> 2024
alles andere in Cargo.toml: unveraendert
```

Ohne Marker bleibt der bisherige Version-only-Vertrag bestehen. Ein Marker ohne Cargo-Aenderung, ein Marker in einer anderen ChangeClass, ein anderer Editionssprung oder irgendeine weitere Manifest-Aenderung wird fail-closed abgewiesen.

## Deep-Clippy als Migrationsmessung

`--clippy-deep` bleibt diagnostisch und nicht musikalisch autoritativ, fuehrt nun aber zusaetzlich die rustc-Lintgruppe `rust-2024-compatibility` aus. Damit kann nach Installation dieses Bridge-Takes mit dem bestehenden Ein-Befehl-Weg frische Compiler-Evidenz gesammelt werden, bevor die Edition tatsaechlich gewechselt wird.

## Nicht geaendert

- Rust Edition bleibt 2021.
- Keine Dependency wird hinzugefuegt oder aktualisiert.
- Keine Musik-, MusicXML-, Render- oder PDF-Entscheidung wird geaendert.
- Keine Writer-, Repair- oder Recovery-Authority wird hinzugefuegt.
- `#![forbid(unsafe_code)]` bleibt unveraendert.
- Der separate `tools/etuschlopf_watch`-Crate bleibt vorerst ebenfalls auf Edition 2021; dessen Migration wird nicht mit der Root-Crate vermischt.

## Abnahme

Der Bridge-Take soll zuerst auf dem Zielsystem normal durch die bestehenden Gates gehen. Danach ist `.\run.cmd --clippy-deep` die frische Editions-Kompatibilitaetsmessung. Erst deren Befund entscheidet, ob vor dem eigentlichen `edition = "2024"`-Take noch weitere 2021-kompatible Quellanpassungen erforderlich sind.
## r02 – Pin-Owner-Closure

Der zuerst gelieferte r01-Kandidat wurde vor dem Kopieren vom bestehenden Architecture-Pin-Gate abgewiesen, weil drei byte-gepinnte Security-Pfade geaendert wurden, ohne ihre beiden Pin-Eigentuemer gemeinsam zu aktualisieren. r02 schliesst exakt diese vorhandene Sicherheitsinvariante: `identity_debt_build.rs` pinnt die neuen Bytes von `take_change_control.rs`, `etuschlopf_launcher.rs` und `assistant_export.rs`; `identity_gate_trust_anchor.rs` bindet diese Pins sowie den daraus geaenderten Build-Policy-Fingerprint compile-time. Die Bridge-Logik selbst bleibt gegenueber r01 unveraendert.

