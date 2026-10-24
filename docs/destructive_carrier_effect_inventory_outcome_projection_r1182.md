# r1182 DestructiveCarrierEffectInventoryOutcomeProjection

Parent: accepted r1181. ChangeClass: `OutcomeProjection` (Kodex + Audit + Metadata only).

## Purpose

Measure the epistemic resolution of the productive pipeline with respect to destructive carrier effects: where EtuSchlopf can see an effect individually, where it only sees the combined delta of a block, and where a persisted state change escapes every observation boundary. The take repairs nothing, grants nothing and withdraws nothing.

## Changed paths

| Path | Role | Change |
|---|---|---|
| `src/build_gate/destructive_carrier_effect_scan.rs` | Kodex | new build-time scanner |
| `src/kodex_build_orchestrator.rs` | Kodex (pinned) | invokes the scanner, emits a generated include |
| `src/identity_gate_trust_anchor.rs` | Kodex (pin owner) | rebinds only the build-policy fingerprint |
| `src/audit_only/destructive_carrier_effect_inventory.rs` | Audit | projects the generated inventory; synthetic instrument tests |
| `src/audit_only/writer_trace.rs` | Audit | render epoch continuity recorder beside the unchanged beam recorder |
| `src/audit_only/mod.rs` | Audit | module declaration |
| `src/pipeline_shards/final_report_projection_owner.rs` | Audit | one additional report section |

## Static inventory (build time)

Effect surfaces are derived from source structure, never from function names:
- **TYPED_AUTHORITY:** public effect-owner functions (including macro-generated ones) called from outside `src/structural_write_authority/` whose owner-local closure mutates an XML buffer.
- **FILE_SINK:** public effect-owner functions whose owner-local closure persists bytes; `recorder` is `YES` when the runtime recorder kind is statically derivable (string literal or enum-variant tag), `UNRESOLVED` when the recorder is reachable but the kind is not derivable, `NO` when no recorder is reachable.
- **XML_REBUILD:** productive functions outside the effect owners and audit paths that rebuild an output string and emit timeline carrier markup (`<forward`, `print-object`).

`physical` is syntactic evidence from the surface's own literals and signature (`REST_TO_FORWARD`, `HIDE_CARRIER`, `CALLER_SUPPLIED_REPLACEMENT`, `WHOLE_FILE_WRITE`, `OTHER`). It is not a musical reason.

Region granularity for each productive call site of a traced file sink (region = calling function, transformation surfaces reachable from calls located before the sink call):
- `BLOCK` when at least two distinct transformation surfaces are called straight-line (function top level) before the sink;
- `SINGLE_TRANSFORMATION` when exactly one transformation surface is reachable and the region input is a persisted-state read;
- otherwise `SINGLE_OR_BLOCK_UNRESOLVED`.

A recorder kind with several call sites is `BLOCK` only if every site is `BLOCK`; otherwise it is unresolved. Unknown kinds at runtime are unresolved.

Name resolution is conservative: method calls bind within their own file; free and path calls bind to same-file definitions first; calls qualified by standard-library modules never bind to project functions.

## Runtime continuity (render only)

For each accepted render boundary: complete-byte FNV-1a-64 before/after, gap class (`FIRST`, `CONTINUOUS`, `DRIFT_OR_EPOCH_RESET`), boundary class (`SEED` when no prior file existed, else the static kind granularity), and one shared carrier census for before and after. Calls outside accepted paths are counted (`TracedCallsOutsideAcceptedPaths`) but produce no row. The chain restarts with every project session.

## Positive and negative controls

All controls exercise the real measurement code (scanner on synthetic source text; recorder with synthetic before/after states). No productive writer or sink is called from tests.

- one surface before the sink → `SINGLE_TRANSFORMATION`;
- two straight-line surfaces → `BLOCK`;
- a conditional second surface → `SINGLE_OR_BLOCK_UNRESOLVED`;
- untraced state change between boundaries → `InterBoundaryDrift 1`;
- identical bytes between boundaries → no drift;
- outside-path call → counted, no row;
- new project session → chain restarts.

## Interpretation rule

`UNKNOWN`, `UNRESOLVED`, `NOT_OBSERVABLE_IN_AUDITONLY` and drift are valid results. A block delta is never attributed to an individual owner. Corpus distributions are measurement results, not acceptance expectations.
