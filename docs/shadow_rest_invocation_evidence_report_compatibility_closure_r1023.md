# ShadowRest Invocation Evidence Report Compatibility Closure — r1023

## Scope

`r1023 ShadowRestInvocationEvidenceReportCompatibilityClosure` is a `RefactorOnly` follow-up to accepted parent `r1022 ShadowRestInvocationEvidenceRetirement`.

r1022 correctly retired the producerless `ShadowRestRepairLoopInvocationEvidence` runtime carrier, its InitialRender transport, and the duplicated StrictStaff value copies. The full r1022 corpus remains dark green: 10/10 projects OK and final-clean, no blocker/warning/hint project, Clippy warning budget 0, `FinalExit 0`, and `ActiveRepairTarget: none`.

The follow-up closes one report-compatibility gap discovered after that retirement. `docs/repair_loop_invocation_evidence_boundary.md` defines the `REPAIR-LOOP-INVOCATION-EVIDENCE` rows as stable and corpus-aggregable. Removing the two historical `ShadowRestPre` / `ShadowRestPost` rows therefore changed a documented text-report surface even though no live in-repository consumer and no musical delta was found.

## Change

The dead runtime carrier stays retired. No ShadowRest producer, transport object, copied StrictStaff field, writer, selector, threshold, readback, release gate or musical authority is restored.

Only the two stable text rows are restored at their historical positions as explicit compatibility tombstones:

```text
ShadowRestPre: invoked 0; candidates 0; written 0.
StrictStaffBetweenShadowInvocations: written <current>; adopted <current>.
[optional StrictStaffLocalGeometryWitness]
ShadowRestPost: invoked 0; candidates 0; written 0.
```

The two ShadowRest rows are no longer presented as measured invocation evidence from a mutable/default-filled carrier. Their zero values are fixed compatibility markers for the already retired dedicated pre/post owners. The live StrictStaff row continues to read directly from the existing `StrictStaffOverfullCoverageNormalizationTrial`, exactly as in r1022.

## Invariants

- `ShadowRestRepairLoopInvocationEvidence` remains absent.
- No InitialRender ShadowRest invocation-evidence transport returns.
- StrictStaff `written` / `candidate_adopted` remain read directly from the typed StrictStaff trial.
- No new function, helper, writer, selector, threshold, capability or dependency is introduced.
- Master MusicXML, render MusicXML and PDF rendering are expected to remain unchanged from r1022.
- The public `.result.txt` compatibility surface regains the two historically stable ShadowRest rows and otherwise remains unchanged.

## Acceptance

The early single-project gate must prove Build/Clippy/tests green and unchanged musical artifacts. The result report must contain both restored constant ShadowRest rows with the same live StrictStaff row between them. Full-corpus promotion follows only after the normal corpus gate remains green.
