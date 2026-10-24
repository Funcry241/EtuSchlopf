# r1201 DestructiveIdentityCoverageAndThreadLossSuperAudit

Parent: accepted/released r1200 `SelectedReceiptLineagePrintCleanupSuperClosure`.
Change class: `OutcomeProjection`. Music mutation: none. Removal of any gate: none.

## Why

The r1200 architecture review showed three unproven assumptions behind a later Carrier-Release Evidence/Proof/Permit design:

- `IndividualEditIdentity` exists only for the caller-supplied replacement family; the full r1182 inventory has many more productive destructive surfaces.
- The attribution ledger is `thread_local`; observations issued in `thread::scope` workers are dropped silently, so `NO_SELECTION_WITNESS` cannot be read as "not selected".
- Productive code contains nominal gates and literal coordinates that must be known before any Proof/Permit work.

r1201 measures all three. It builds no Evidence taxonomy, ProofType, Permit or typed adoption boundary.

## 1. Identity coverage (static + runtime)

Build time (`src/build_gate/destructive_carrier_effect_scan.rs`), per r1182 surface:

| Class | Definition |
|---|---|
| `IDENTITY_PRESENT` | productive transformation surface whose typed entry's owner closure calls `begin_destructive_edit_batch_attribution(` |
| `IDENTITY_MISSING` | productive transformation surface (typed authority or XML rebuild) without such minting |
| `NOT_APPLICABLE` | productive whole-file sink; it persists documents, not edits |
| `RETIRED` | not productively reachable (retired or keepalive-only) |
| `IDENTITY_LOST_CROSS_THREAD` | runtime refinement of `IDENTITY_PRESENT`: at least one of its mints was issued on a thread without an owner ledger |

The scanner also records every exact callsite (`path:line`) of an identity-minting entry. At runtime a ledger origin is matched to these sites by exact normalized path and line only; unmatched origins are counted (`RuntimeOriginsWithoutStaticSite`).

Report lines: `DestructiveIdentityCoverageSurface` (one per transformation surface), `DestructiveIdentityCoverageFamily` (per surface kind and physical effect), `DestructiveIdentityCoverage` (summary).

## 2. Thread-locality loss

The owner-thread ledger is untouched in behaviour; its r1188–r1200 report lines stay the same. The ledger code was refactored so that every transition is a function over one ledger state; `with_ledgers` applies it to the owner ledger and to a process-wide mirror. Observations from a thread without an owner ledger go to the mirror only, and only when exactly one project session is active (otherwise counted as unroutable).

Report lines:

- `DestructiveIdentityThreadLoss`: detached observations by kind, detached successors that carried receipts, detached batches/edits and their mirror dispositions.
- `DestructiveIdentityThreadLossNoSelectionWitness`: owner-thread `NO_SELECTION_WITNESS` count, how many reach `FINAL_RECEIPT` or an open selection in the mirror (explained by thread locality), and the remainder.
- `DestructiveIdentityThreadLossDisposition`: full owner-thread × process-wide disposition matrix.
- `DestructiveIdentityThreadLossOrigin`: callsites with detached batches or differing final receipts.

The mirror is observation-only; nothing productive reads it.

## 3. Nominal gate inventory

Rules (syntactic, fixed before measurement, applied to non-test functions outside `src/audit_only` and `src/audits`):

| Rule | Definition |
|---|---|
| `LITERAL_COORDINATE_TABLE` | array with ≥2 tuples of ≥3 integer literals, plus an equality test on a `measure`/`scope` binding |
| `LITERAL_MEASURE_BINDING` | `measure`/`measure_number`/`measure_index`/`measure_no` initialised with or compared to an integer literal > 1 |
| `CONSTANT_TRUE_CHECK_FIELD` | field whose name declares a check outcome (`invariant`, `stable`, `green`, `match`, `valid`, `consistent`, `safe`, `guard`, `proof`, `_ok`), read in a logical condition, only ever assigned literal `true` |
| `CONSTANT_ZERO_GATE_FIELD` | numeric field only ever assigned literal `0`, compared with `0` |
| `CONSTANT_OPEN_GATE_PARAMETER` | `bool` parameter whose name contains `gate`, passed as literal `true` by every caller |

Runtime association (`NominalGateInventory` lines) is REGION level: identity-bearing edits whose callsite is in the flagged function, its direct callers or callees. Influence classes: `UNKNOWN_NO_IDENTITY_LINK`, `LINKED_PATH_NOT_EXERCISED`, `EXERCISED_NO_APPLIED_EDIT`, `APPLIED_WITHOUT_FINAL_RECEIPT`, `APPLIED_WITH_FINAL_RECEIPT`. This is association, not proof that the construct decided an edit.

Static result on the r1200 source (build-time scan, before corpus): 11 flagged rows, all productive, including the literal coordinate table and constant-open user gate in the articulation accent path, the constant-zero WAV contradiction gate and the literal measure probe in strict-staff release, and the constant-true boundary invariant in the lower-staff register gate.

## 4. Instrument correction (r1182 scanner)

`test_only_spans` previously searched forward from `#[cfg(test)]` for the next `mod` text. After a `#[cfg(test)] fn` accessor this could mark later productive items as test-only. The span now covers exactly the annotated item. Effect on the r1200 source scan: surfaces 108 → 110 (two productive typed authorities were hidden), one surface RETIRED → PRODUCTIVE, one observability EXACT → UNKNOWN, `visual_cleanup` sink granularity `SINGLE_TRANSFORMATION` → `SINGLE_OR_BLOCK_UNRESOLVED`. The r1182 inventory fingerprint changes accordingly.

## Acceptance

Dependency preflight, Take-Change-Control, fmt, tests, build, strict Clippy, full 10/10 corpus. Master/Render MusicXML byte-identical to r1200, PDF visually unchanged, all r1188–r1200 attribution lines unchanged. `UNKNOWN` and unmatched results are valid outcomes.

## Synthetic controls

- coverage classes and exact minting-site line from a synthetic owner + pipeline, including a `#[cfg(test)] fn` followed by productive code and a later `mod`-prefixed identifier (the corrected span defect);
- all five gate rules with negative controls (event flag, computed counter, gate parameter with a `false` caller, measure literal 1);
- mirror ledger: detached successor recovers a final receipt, detached minting counted without touching the owner ledger, equal ledgers report no thread locality;
- projection smoke test against the generated static tables.
