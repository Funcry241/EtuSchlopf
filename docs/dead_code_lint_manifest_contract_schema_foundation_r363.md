# r363 DeadCodeLintManifestContractSchemaFoundation

## Zweck

Dieser SecurityOnly-Foundation-Take loest die Bootstrap-Reihenfolge fuer den geplanten Root-Package-Dead-Code-Lint-Cutover. Der installierte Take-Bootstrap prueft das eingehende Take-Change-Manifest vor jedem Kopieren. Daher muss der Parent einen neuen geschlossenen Manifest-Schluessel bereits kennen, bevor ein spaeterer Take diesen Schluessel selbst verwenden darf.

## Aenderung

- `TakeChangeManifest` kennt den booleschen Vertrag `rust_dead_code_lint_contract`.
- Der Parser akzeptiert ausschliesslich `rust_dead_code_lint_contract=closed_root_package_deny_v1`.
- Der Cargo-Diff-Klassifizierer erkennt ausschliesslich das exakte Anhaengen von `[lints.rust]` mit `dead_code = "deny"` an ein Root-Manifest ohne bestehende `[lints.rust]`-Sektion.
- Der Vertrag ist nur fuer `SecurityOnly` zulaessig und darf nicht mit den bestehenden Edition-, Resolver/MSRV- oder Build-Owner-Transitionsmarkern kombiniert werden.
- Marker ohne echten Cargo-Uebergang sowie echter Root-Dead-Code-Uebergang ohne Marker werden fail-closed abgelehnt.
- Negative Tests decken falsche Lint-Level und falsche ChangeClass ab.

## Bewusste Nicht-Aenderungen

- `Cargo.toml` bleibt byteidentisch zum real GREEN r361.
- Dieses r363-Manifest verwendet den neuen Schluessel selbst noch nicht.
- Die 15 bestehenden `allow(dead_code)`-Suppressions bleiben unveraendert.
- `main.rs` behaelt sein lokales `#![deny(dead_code)]`.
- Keine Suppression-Policy, keine Compiler-Session-Haertung und keine Musik-/Writer-/Repair-/Recovery-Aenderung.

## Architekturgrund

Schema-Evolution ist selbst Teil der Security-Grenze: Ein Incoming-Take darf keinen Manifestvertrag voraussetzen, den der installierte Parent vor dem Kopieren noch nicht parsen kann. r363 installiert daher ausschliesslich den typisierten Parser-/Validator-Unterbau; der eigentliche Package-Lint-Cutover folgt erst auf einem real GREEN r363.
