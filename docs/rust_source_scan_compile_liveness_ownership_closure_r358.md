# RustSourceScanCompileLivenessOwnershipClosure r358

## Parent und reale Evidenz

Autoritativer Parent ist EtuSchlopf `0.10.1839 r357`
`RustSourceScanMechanicsOwnershipConvergencePinOwnerClosure`, real installiert und GREEN.

Der reale Windows-Reexec von r357 bestaetigt Build 0, Clippy 0, Launcher-Test 0,
FinalExit 0 und `dark_green/released`. Gleichzeitig zeigt der All-Targets-Testlauf
sechs neue diagnostische `dead_code`-Warnungen im Take-Bootstrap fuer genau die in
r357 in den gemeinsamen `RUST_SECURITY_GATE_CORE` konvergierten Source-Fact-Helfer:

- `collect_rust_files`
- `normalize_path`
- `line_starts`
- `line_number_at`
- `token_boundary_before`
- `token_boundary_after`

Das ist keine tote Security-Mechanik. Der Build-Gate-Scanner benutzt alle sechs
Funktionen produktiv; der Take-Bootstrap benoetigt aus demselben gemeinsamen Core
dagegen nur die schmalere Masking-/FNV-Basis fuer Take-Change-Control und Payload-
Fingerprinting. Der reale Compilerbefund beweist damit eine Compile-Liveness-
Asymmetrie innerhalb eines ansonsten richtigen gemeinsamen Owners.

## Ziel

r358 behaelt genau **einen** Owner:

`tools/RUST_SECURITY_GATE_CORE`

Der Owner wird jedoch in zwei schmale, policyfreie Compile-Units getrennt:

1. `rust_source_scan_primitives.rs`
   - FNV-1a
   - Rust-Lexik-Maskierung
2. `rust_source_fact_primitives.rs`
   - rekursive `.rs`-Sammlung
   - Pfadnormalisierung
   - Zeilenstarts/Zeilennummer
   - Token-Grenzen

Die sechs Source-Fact-Funktionskoerper sind gegen r357 byteinhaltlich unveraendert.
Es entsteht keine zweite Policy, kein zweiter Scanner-Owner und kein neues Cargo-
Package.

Der Build-Script-Owner bindet den bestehenden path-stabilen
`src/build_gate/rust_source_scan_primitives.rs`-Contract ein; dieser Contract
re-exportiert Base und SourceFacts gemeinsam fuer IdentityDebt,
ProductiveWrite und die bestehenden negativen Tests.

Der Take-Bootstrap bindet weiterhin direkt nur
`tools/RUST_SECURITY_GATE_CORE/src/rust_source_scan_primitives.rs`.
Dadurch kompiliert er die sechs dort nicht benoetigten Source-Fact-Helfer nicht mehr.

## Security-Bindung

Die neue Source-Fact-Compile-Unit wird explizit in:

- `SecurityBuildContext`
- `SourceIntegritySecurity`
- `IdentityGateTrustAnchor`
- `RUST_SECURITY_GATE_CORE/OWNERSHIP.txt`

aufgenommen und FNV-gepinnt.

Der Build-Orchestrator und der Trust-Anchor werden als die verpflichtenden
Pin-Eigentuemer synchronisiert. Pins werden aus den finalen Bytes abgeleitet.

## Bewusst unveraendert

- IdentityDebt-Policy und Findings
- ProductiveWrite-Policy und Findings
- ThirtySurface-Scanner
- Take-Change-Control-Policy
- Security-Capability-Retirements
- adversariale Angriffsklassen
- 15 bestehende `allow(dead_code)`-Suppressions
- Cargo-Lint-Authority
- Musik-, MusicXML-, Renderer-, PDF-, Writer-, Repair- und Recovery-Semantik
- Schwellenwerte und Auswahlentscheidungen

Es wird insbesondere **kein** neues `allow(dead_code)` eingefuehrt.

## Erwartete Abnahme

Der reale Windows-Lauf muss weiterhin alle bisherigen Security-Gates und Tests
bestehen. Zusaetzlich sollen die sechs r357-Bootstrap-Warnungen aus
`rust_source_scan_primitives.rs` verschwinden, weil diese Funktionen im
Take-Bootstrap nicht mehr kompiliert werden.

ChangeClass: `SecurityOnly`.
