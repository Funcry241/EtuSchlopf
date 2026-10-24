# LauncherBuildControlPlaneBoundary

## Ziel

Dieser `SecurityOnly`-Take ist Schritt 2 des vereinbarten Launcher-/Build-/Audit-Superwurfs.
Er trennt die bereits vorhandenen Build-, Clippy-, Build-Reuse- und Quellsignatur-Mechaniken
physisch aus `src/bin/etuschlopf_launcher.rs` in den privaten Include-Shard
`src/bin/etuschlopf_launcher/launcher_build_control.rs`.

## Grenze

Im Launcher verbleiben bewusst die typisierten Build-Vertraege und die kleinen
Orchestrierungsgrenzen, die der bestehende Security-Contract direkt am Launcher verankert:

- `CargoBuildProfile`, `ValidationPlan`, `CompileReason`, `BuildPlan`,
  `ArtifactAvailability` und `BuildOutcome`
- `plan_launcher()`
- `plan_build()` als Boundary fuer die verifizierte Artefaktentscheidung
- `materialize_launcher_build()`
- `write_successful_build_stamp()` als gepinnter Stamp-Commit

Ausgelagert werden ausschliesslich die bereits vorhandenen Mechaniken fuer:

- Deep-Clippy-Diagnose und deren Logprojektion
- strict Clippy und Warning-Budget
- Cargo-Build und Build-Materialisierung
- verifizierte Build-Wiederverwendung
- Build-Plan-Hilfen
- Build-Stamp-Pfade und Artefaktverifikation
- Source-/Binary-FNV und Source-Build-Signatur
- Cargo-Prozessadapter und Strict-Log-Hilfen

Die verschobenen Funktionen bleiben einzeln byteidentisch; `include!` behaelt denselben
Rust-Modul- und Privacy-Namensraum. Es werden keine Build-Regeln, Profile, Reuse-Kriterien,
Clippy-Gates, Signaturinhalte, Prozessargumente oder Exitentscheidungen veraendert.

## Nicht geaendert

- keine `--strict`-/`--clippy-deep`-Semantik
- keine Build-Reuse- oder `--force`-Semantik
- keine Source-/Binary-Fingerprint-Regel
- keine Launcher-Test-, Run-, Corpus- oder Visual-Preflight-Reihenfolge
- keine Take-/Handover-Logik
- keine Cleanup-Regel
- keine Report-/Release-Semantik
- keine Musiklogik, Writer oder Artefaktpfade

## Gate-/Pin-Vorpruefung

r01 zeigte im realen Reexec-Build eine zusaetzliche wortwoertliche Ownership-Invariante:
`etuschlopf_launcher.rs` muss den Security-Gate-Core-Quellpfad selbst besitzen. r02
laesst deshalb `security_gate_core_source_root()` als kleine gepinnte Boundary im Launcher
und laesst die ausgelagerte Source-Build-Signatur diese Boundary verwenden.


Vor der Paketierung werden die bestehenden Launcher-Security-Textvertraege explizit
gegen den neuen Hauptdatei-Inhalt geprueft. Die vom Gate direkt erwarteten Boundary-Namen
bleiben dort erhalten; deshalb ist keine Lockerung oder Erweiterung des Security-Gates
notwendig. Der Launcher-Pin wird wie vorgeschrieben zusammen mit
`identity_debt_build.rs` und `identity_gate_trust_anchor.rs` synchronisiert.

## Stop

Mit einem gruenen Etuede-4-Artefaktbeweis endet Take 2. Visual/Audit wird in diesem Take
nicht begonnen.
