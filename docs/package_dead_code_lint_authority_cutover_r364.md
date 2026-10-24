# r364 PackageDeadCodeLintAuthorityCutover

## Parent und reale Evidenz

Autoritativer Parent ist EtuSchlopf `0.10.1839 r363`
`DeadCodeLintManifestContractSchemaFoundation`, real installiert und GREEN.

Der reale Windows-Lauf bestaetigt fuer r363:

- BuildExit 0
- ClippyExit 0
- LauncherTestExit 0
- FinalExit 0
- `dark_green/released`
- der installierte Parent kennt und validiert den geschlossenen Manifestvertrag
  `rust_dead_code_lint_contract=closed_root_package_deny_v1`

r362 wurde vor dem Kopieren verworfen, weil der damals installierte Parent den
neuen Manifest-Schluessel noch nicht kannte. r363 hat ausschliesslich diese
Schema-/Bootstrap-Reihenfolge geschlossen. r364 verwendet den Vertrag nun erstmals.

## Root-Package-Lint-Authority

Das Root-`Cargo.toml` erhaelt exakt:

```toml
[lints.rust]
dead_code = "deny"
```

Die Authority gilt fuer das Root-Package `etuschlopf`. Separate Cargo-Packages
unter `tools/` werden nicht in diesen Vertrag einbezogen.

Das bestehende crate-lokale `#![deny(dead_code)]` in `src/main.rs` bleibt in diesem
Security-Take bewusst unveraendert. Seine spaetere mechanische Pensionierung ist
nicht Bestandteil dieses Cutovers.

## Geschlossener Change-Control-Vertrag

Das r364-Take-Manifest verwendet erstmals den durch r363 installierten Marker:

```text
rust_dead_code_lint_contract=closed_root_package_deny_v1
```

Der bereits installierte r363-Validator akzeptiert damit ausschliesslich den
exakten Root-Cargo-Uebergang von einem Manifest ohne `[lints.rust]` zu dem
angehaengten Vertrag `[lints.rust] / dead_code = "deny"`.

Es gibt in r364 keine weitere `Cargo.toml`-Aenderung und keine Aenderung des
Take-Change-Control-Codes oder seiner Pins.

## Bewusste Nicht-Aenderungen

- keine neue oder entfernte `#[allow(dead_code)]`-Suppression
- keine Legacy-Debt-Baseline
- keine Parent->Incoming-Suppression-Monotonie
- kein `cfg_attr`-/Lintgruppen-Schutz
- kein Compiler-Session-Schutz gegen externe Lint-Abschwaechung
- keine allgemeine `unused_*`-, Clippy- oder `unsafe_code`-Migration
- keine Musik-, MusicXML-, Renderer-, Writer-, Repair-, Recovery- oder
  Threshold-Aenderung

Die Integritaetsschicht fuer die nun packageweite Authority folgt erst im
abschliessenden `DeadCodeDetectionIntegrityChangeControlClosure`.

ChangeClass: `SecurityOnly`.
