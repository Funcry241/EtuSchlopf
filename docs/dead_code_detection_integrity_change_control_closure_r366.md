# EtuSchlopf v0.10.1839 r366
## DeadCodeDetectionIntegrityPolicyOwnerClosure

Parent ist der real installierte GREEN-Stand r364 `PackageDeadCodeLintAuthorityCutover`.
r364 hat den Root-Package-Vertrag `[lints.rust] dead_code = "deny"` im echten Windows-
`cargo test --profile strict --all-targets` bewiesen. r366 schützt diese Authority jetzt
gegen drei voneinander getrennte Abschwächungswege, ohne eine neue Musik-, Writer-,
Repair- oder Recovery-Authority einzuführen.

### 1. Ein Policy-Owner, bestehende Source-Facts

`src/build_gate/dead_code_detection_integrity.rs` ist der einzige neue fachliche
Policy-Owner. Rekursive Rust-Dateisammlung, Pfadnormalisierung, Zeilenindex,
Zeilennummern, Token-Grenzen und lexikalische Maskierung werden nicht dupliziert,
sondern aus dem bestehenden `RUST_SECURITY_GATE_CORE` konsumiert.

Der Take-Bootstrap bindet deshalb jetzt den bereits vorhandenen
`src/build_gate/rust_source_scan_primitives.rs`-Vertrag statt nur die schmale
Masking/FNV-Compile-Unit ein. Das ist keine Rückkehr zu ungenutzter Scannerbreite:
DeadCodeDetectionIntegrity konsumiert die SourceFacts im Parent->Incoming-Preflight
produktiv. Ein physischer Primitive-Owner bleibt erhalten.

### 2. Root-Package-Lint-Authority

Installed Tree und Incoming Tree müssen weiterhin genau eine Root-Sektion
`[lints.rust]` mit `dead_code = "deny"` besitzen. Eine Absenkung auf `warn`/`allow`,
eine unklare Mehrfachdefinition oder eine andere statisch nicht eindeutige Form ist
fail-closed.

### 3. Legacy-Debt-Ceiling ist monoton, keine Soll-Allowlist

`src/build_gate/dead_code_suppression_debt_ceiling.tsv` enthält die 15 beim r364-
Cutover vorhandenen Suppression-Identitäten als Obergrenze. Identität bindet
relativen Pfad, Scope-Art, Item-/Modulanker, Lint-Level, normalisierten Selector und
normalisierte cfg/cfg_attr-Bedingung.

- Entfernen einer Ausnahme: erlaubt.
- `allow -> deny` bzw. stärkere Lint-Authority: erlaubt.
- Neue Ausnahme: blockiert.
- Owner-/Pfadwechsel: blockiert.
- Scope-Verbreiterung: blockiert.
- cfg/cfg_attr-Verbreiterung: blockiert.
- `unused` als Gruppe wird wie `dead_code` geschützt.
- `warn`, `allow` und `expect` auf dem geschützten Lint werden als Abschwächung
  behandelt.

Parent->Incoming verlangt zusätzlich, dass der Incoming-Suppression-Satz eine
Teilmenge des real installierten Parent-Satzes bleibt. Eine einmal entfernte Ausnahme
kann deshalb nicht allein wegen eines historischen Ceiling-Eintrags zurückkehren.

### 4. CompilerLintSessionIntegrity

Der Build-Owner prüft `CARGO_ENCODED_RUSTFLAGS` gezielt auf Abschwächungen dieser
Authority. Insbesondere werden `--cap-lints=allow|warn`, `-A/-W`,
`--allow/--warn` und `--force-warn` für `dead_code` beziehungsweise die umfassende
Gruppe `unused` blockiert. Andere Rustflags werden nicht pauschal verboten.

### 5. Adversarial Matrix

Die geschlossene Matrix wächst von 18 auf 21 ausführbare Angriffsklassen:

- `new_dead_code_suppression`
- `dead_code_suppression_scope_widening`
- `compiler_dead_code_lint_weakening`

Die Matrixzeilen sind an reale Negativtests im Policy-Owner und an die bestehende
Trust-/Pin-Kette gebunden. Matrix, Policy, Ceiling, Take-Change-Control,
Take-Bootstrap, Build-Orchestrator und Trust-Anchor werden synchronisiert.

### Abgrenzung

Keine Quotenjagd 15->0. Keine vorsorgliche Zerlegung von `take_change_control.rs`
oder `public_artifact_recovery.rs`. Keine neue Allowlist als Sollzustand. Kein
zweiter Rust-Dateiwalker. Keine allgemeine Rustflags-Policy. Keine Änderung an
MusicXML, Renderer, Writer, Repair/Recovery-Semantik, Pass-Reihenfolge oder
musikalischen Schwellen.

Nach einem real GREEN r366 ist der geplante Dead-Code-/Security-Strang abgeschlossen.
Verbleibende Legacy-Suppressions werden danach neu inventarisiert und nur aufgrund
realer Compiler-/Ownership-Evidenz separat verengt oder pensioniert.


## r366 policy-owner closure

The real r365 Windows build proved that compiler-session source ownership was incomplete: `CARGO_ENCODED_RUSTFLAGS` was read by the take-control wrapper while the adversarial contract correctly requires the DeadCodeDetectionIntegrity policy owner itself to own that input. r366 moves only that environment read into the policy owner; the wrapper passes only the project root. The defense is not weakened and the attack matrix remains closed at 21 classes.
