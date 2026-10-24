# Rust 2024 Edition Cutover

## Scope

Dieser Take vollzieht ausschliesslich den zuvor vorbereiteten Editionswechsel von Rust 2021 auf Rust 2024. Die Toolchain, Dependencies, Musiklogik, Writer-Authorities, Passorder und Schwellenwerte bleiben unveraendert.

## Reale Migrations-Evidenz

Der letzte reale `--clippy-deep`-Lauf auf v0.10.1798 r02 erreichte `CargoExit: 0` ueber `--all-targets`. Die acht zuvor gemeldeten `macro_rules!`-`expr`-Befunde waren nach der expliziten `expr_2021`-Closure verschwunden. Als einziger verbleibender `rust-2024-compatibility`-Befund blieb `tail_expr_drop_order` in `src/bin/etuschlopf_launcher.rs` an `match command.spawn()`.

## Drop-Order-Closure

Das betroffene `match command.spawn()` war die Tail-Expression von `start_watch_best_effort`. Edition 2024 verengt den Scope von Temporaries in Block-Tail-Expressions. Der Take macht das `match` deshalb mit einem abschliessenden Semikolon explizit zu einem Statement. Damit ist die Drop-Grenze nicht mehr editionsimplizit; Watch bleibt Best-Effort und der musikalische Lauf behaelt dieselbe Fehlersemantik.

Es wird kein `unsafe` eingefuehrt. Der bereits in v0.10.1797 entfernte prozessglobale Environment-Write bleibt entfernt.

## Edition

`Cargo.toml` wechselt exakt:

```toml
edition = "2021"
```

zu:

```toml
edition = "2024"
```

Die bestehende Take-Policy erlaubt diesen Wechsel nur bei `SecurityOnly` und nur mit `rust_edition_transition=closed_2021_to_2024_v1`. Alle anderen Cargo.toml-Aenderungen bleiben fail-closed verboten.

## Nicht Bestandteil

- keine Dependency-Aenderung
- kein Clippy-Massencleanup
- keine Musikmutation
- kein neuer Writer
- keine Repair-/Recovery-Authority
- keine Passorder- oder Schwellenwert-Aenderung
- keine Toolchain-Aenderung

## Erwarteter Nachweis

1. Der transaktionale Take wird vom expliziten 2021->2024-Vertrag akzeptiert.
2. Strict-Clippy, Build und Launcher-Tests bleiben gruen.
3. Der Vollkorpus bleibt 10/10 dunkelgruen und musikalisch invariant.
4. Ein anschliessender `--clippy-deep` erreicht erneut CargoExit 0; `rust-2024-compatibility` meldet keinen Editionsmigrationsrest mehr.
