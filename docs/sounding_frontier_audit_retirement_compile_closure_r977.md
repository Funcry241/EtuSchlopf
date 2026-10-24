# r977 SoundingFrontierAuditRetirementCompileClosure

## Parent and failure evidence

`r977` is a `RefactorOnly` compile-closure built directly on the last accepted `0.10.1839 r975` parent.

The preceding `r976 SoundingFrontierAuditRetirement` payload is rejected Failure Evidence and is not a parent. Its real Windows strict-Clippy run failed only because retiring the report-side `SoundingMaterialMutationFrontierAudit` removed the sole consumer of:

- `phrase_boundary_evidence::captured_atomic_observations()`.

The root crate has `#![deny(dead_code)]`, so that now-unreferenced export correctly stopped acceptance before the corpus run.

r977 reapplies the same Sounding frontier audit retirement directly to r975 and additionally removes only the observation-cache island that existed solely to serve that retired consumer. No other source-semantic repair is added.

## Retirement scope

The original retirement remains unchanged. From `src/audit_only/source_selection_full_causal_audit_shadow_plan.rs`, r977 removes the historical report-only `SoundingMaterialMutationFrontierAudit`, including:

- its ten `include_str!` source bindings and static source-fragment capability search;
- PostLegacy pitched-attack cardinality reconstruction and comparison with the final atomic readback;
- speculative `GapWitness` reconstruction;
- first-mutation / existing-owner report guesses;
- `RequiredNextArchitectureEvidence` requests for forward material-lineage receipts;
- its imports, constants, helpers and report call.

The old report fields disappear completely rather than being backfilled from newer evidence.

## Compile-closure scope

The r976 failure proves that `captured_atomic_observations()` had exactly one surviving consumer before the retirement: the removed Sounding frontier audit.

The corresponding `PHRASE_ATOMIC_OBSERVATIONS` thread-local cache therefore has no remaining reader. r977 removes only:

- `PHRASE_ATOMIC_OBSERVATIONS`;
- its clear-time reset;
- the capture-time clone/write;
- the unavailable-time reset;
- `captured_atomic_observations()` itself.

The living `PHRASE_BOUNDARY_EVIDENCE` snapshot, phrase analysis, pending notation intents and phrase report section remain unchanged. `PhraseAtomicObservation` remains the active input type passed directly into the phrase analysis; only the obsolete second copy/export is removed.

## Productive authority preserved

The retirement intentionally leaves the actual source/material architecture unchanged, including:

1. SourceSelection and immutable SelectionPlan authority;
2. serialized source identity and PostLegacy material disposition;
3. productive candidate material lineage;
4. explicit material mutation receipts at the real mutation owners;
5. final sounding material freeze;
6. final master/readback identity proofs;
7. all existing fail-closed productive checks around those owners;
8. the living phrase-boundary evidence snapshot and its analysis/reporting path.

## Mechanical balance

Relative to accepted r975:

- `src/audit_only/source_selection_full_causal_audit_shadow_plan.rs`: 737 -> 369 lines (**-368 Rust lines**);
- `src/audit_only/phrase_boundary_evidence.rs`: 4468 -> 4454 lines (**-14 Rust lines**);
- total Rust delta: **-382 lines**;
- no test belongs to the removed cache/export and no test is removed;
- expected strict test total therefore remains **1,268**.

Any unexplained test-count change is a blocker.

## Expected externally visible delta

No `.musicxml`, `.render.musicxml` or `.pdf` semantic/output change is intended. The `.result.txt` report changes only by removal of the historical `SoundingMaterialMutationFrontierAudit` block and its subordinate capability/cardinality/gap-witness fields.

## Intentionally unchanged

- no music mutation;
- no SourceSelection rule/order/ranking/weighting change;
- no material mutation owner change;
- no candidate-lineage or receipt semantics change;
- no freeze/readback weakening;
- no phrase-analysis semantic change;
- no new gate, framework, ledger, registry, trust boundary, writer or repair authority;
- root `Cargo.lock` remains LocalTransactional and is not part of the take payload.

## Validation expectation

The real Windows take run is authoritative:

```powershell
.\run.cmd --take --all
```

Expected if the closure is exact:

- 1,268 tests, 0 failures;
- Build green;
- Strict Clippy green;
- 10/10 corpus projects dark-green;
- `.musicxml`, `.render.musicxml` and layout/PDF semantics unchanged relative to r975;
- only the explicitly retired Sounding frontier diagnostics disappear.

If r977 is rejected, its payload remains Failure Evidence and any correction receives a new revision.
