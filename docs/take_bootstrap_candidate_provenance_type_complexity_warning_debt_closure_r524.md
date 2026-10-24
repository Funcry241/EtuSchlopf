# r524 — TakeBootstrapCandidateProvenanceTypeComplexityWarningDebtClosure

Parent: committed r523 `CorpusRunManifestClefWriteCauseAlignmentCompileClosure`.

## Anlass

r523 hat den 10er-Korpus wieder auf `ProgramExitCode 0`, `ManifestValid true` und 10/10 formal gruene Projekte gebracht. Die Freigabe bleibt dennoch gelb, weil der Rust-Take-Bootstrap genau eine sichtbare Clippy-Warning traegt:

```text
clippy::type_complexity
src/bin/etuschlopf_take_bootstrap.rs
read_transaction_candidate_provenance
AppResult<Option<(String, String, u64, String, String)>>
```

Dieser Take schliesst ausschliesslich diese bereits isolierte Warning-Debt.

## Closure

Der bestehende private Tuple-Typ erhaelt den privaten Alias:

```rust
type TransactionCandidateProvenance = (String, String, u64, String, String);
```

`read_transaction_candidate_provenance()` verwendet anschliessend nur diesen Alias in seiner Rueckgabesignatur. Runtime-Repräsentation, Tuple-Reihenfolge, Destructuring, Feldinhalte, Recovery-Entscheidungen und Provenienzsemantik bleiben unveraendert. Es wird kein `#[allow(...)]`, kein Fake-Use und keine Lint-Abschwaechung eingefuehrt.

## Source-Witness-Inventur

Vor dem Patch wurde der r523-Bootstrap gegen seine konsumierenden Gates und adversarial Witnesses inventarisiert. Unveraendert bleiben insbesondere:

- `transaction.rollback(&project_root)?;` am autorisierten Failed-Reexec-Pfad,
- `transaction.commit()?;` am erfolgreichen Reexec-Pfad,
- Public-Artifact-Recovery vor Source-Rollback,
- `rollback_preserving_provenance()` und dessen Call-Order,
- Candidate-Metadata- und Snapshot-Dateinamen,
- Acceptance-/Rejection-Provenienz,
- MultiRevision- und TakeIdentity-Semantik,
- Source-Fact-/DeadCode-Integritaetsgrenzen,
- alle bestehenden Recovery-/Adversarial-Tests und Literal-Witnesses.

Die bestehenden Tests duerfen weiterhin direkt ueber `.0` bis `.4` auf den Tuple-Wert zugreifen; der Alias aendert daran nichts.

## Pin-Koordination

`src/bin/etuschlopf_take_bootstrap.rs` ist bytegepinnt. Deshalb werden ausschliesslich die bestehenden Pin-Owner aus den finalen Bytes synchronisiert:

- `src/build_gate/pinned_policy_kodex.rs`: `PINNED_TAKE_BOOTSTRAP_FNV1A64`,
- `src/identity_gate_trust_anchor.rs`: `TAKE_BOOTSTRAP_FNV1A64` sowie der durch die Pin-Aktualisierung geaenderte `PINNED_POLICY_KODEX_FNV1A64`, jeweils Compile-Time- und Regressionstest-Pin.

Keine Pin-Pruefung wird entfernt oder gelockert.

## ChangeClass

`KodexOnly` mit `warning_debt_closure=closed_current_warning_debt_v1`.

Keine Musik-, Writer-, Renderer-, Report-, Corpus-, Neuro-, Recovery- oder TakeIdentity-Semantik wird geaendert.

## Erwartete Abnahme

Zuerst ein sparsamer Reexec, vorzugsweise:

```powershell
./run.cmd --take "Etüde Nr. 10"
```

Erwartung:

- PRE-TAKE-DIFF-GATE gruen,
- Build und alle Rust-Tests gruen,
- Clippy `TotalWarnings: 0`,
- Projekt `FinalExit 0`,
- Freigabe nicht mehr wegen Warning-Debt gelb.

Erst danach erneut `./run.cmd --all`, um den warningfreien 10er-GreenBase zu beweisen.
