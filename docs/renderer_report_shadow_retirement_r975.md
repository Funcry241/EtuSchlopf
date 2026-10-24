# r975 RendererReportShadowRetirement

## Parent

`r975` is a `RefactorOnly` retirement built directly on the accepted `0.10.1839 r974` parent.

## Scope

The final renderer feasibility decision is already owned by the productive committed-source renderer oracle. Before productive adoption that owner binds the committed final-notation source, plans a renderer-feasibility frontier candidate, materializes the candidate through the authorized sink, invokes the renderer, reads the generated PDF through `renderer_system_break_readback::audit_paths`, learns renderer no-goods when required, and replans up to its bounded iteration limit. Productive plan adoption is fail-closed on the oracle outcome and source/provenance binding.

After that productive owner had completed, the report path still repeated a historical second planning mechanism:

- `renderer_feasibility_constraint_shadow_projection` rebuilt a page/system candidate from report evidence;
- `renderer_feasibility_constraint_shadow_render_proof_projection` rebuilt and rendered candidates again for up to eight additional shadow rounds;
- local candidate, segment-partition and no-good helpers duplicated the same planning idea;
- the report reconstructed the successful renderer invocation from console text or a transported productive renderer outcome;
- the result had no selection authority, no productive writer authority and no release veto; failures degraded to report `UNAVAILABLE`.

This take retires only that second report-side planning island.

## Productive renderer authority preserved

The following productive chain is intentionally unchanged:

1. committed final-notation source binding;
2. renderer-feasibility basis and frontier planning;
3. authorized candidate materialization;
4. MuseScore invocation;
5. `renderer_system_break_readback::audit_paths` over the candidate PDF;
6. renderer no-good learning and bounded replan;
7. confirmed-or-rejected oracle outcome;
8. fail-closed `select_renderer_feasibility_productive_plan` adoption;
9. regular final renderer execution and final artifact checks.

The central `write_renderer_feasibility_shadow_candidate_authorized` sink is therefore deliberately **not** retired: the productive oracle still owns a live use of it.

## Final PDF readback preserved

`src/audit_only/renderer_system_break_readback.rs` remains the owner of the actual final PDF observation boundary. The retirement preserves:

- `audit_paths`;
- the PDF object/stream/text parser;
- planned versus rendered system/page start extraction;
- `RendererSystemBreakReadback`;
- `RendererSystemBreakReadbackPlan`;
- `RendererSystemBreakReadbackRendered`;
- `RendererSystemBreakReadbackDemandBinding`;
- `RendererSystemBreakReadbackRhythmicShape`.

This boundary is later than the planning decision and is also reused by the productive oracle. It is evidence, not the retired counter-planner.

## Retired mechanism

The cut removes from `renderer_system_break_readback.rs`:

- `RendererFeasibilityConstraintBasis`;
- `RendererFeasibilitySegmentPartition`;
- `RendererFeasibilityConstraintCandidate`;
- both report-side shadow projection entry points;
- local basis parsing/validation used only by the shadow;
- local candidate/segment/page partitioning used only by the shadow;
- local observed-no-good reconstruction used only by the shadow;
- renderer command reconstruction and argument templating used only by the shadow;
- shadow target-path construction used only by the report proof;
- the report-owned materialize/render/readback fixed-point loop;
- ten unit tests that exclusively prove the retired report shadow.

The cut additionally removes from `page_count_preference_projection.rs` the renderer console/outcome transport that existed only to feed the retired render-proof projection, plus its one exclusive transport test. `final_report_projection_owner.rs` no longer forwards the productive renderer outcome into that projection and records the remaining evidence origin as `current_process_console_capture`.

## Mechanical balance

Relative to accepted r974 the exact edited Rust line counts are:

- `src/audit_only/renderer_system_break_readback.rs`: 2,995 -> 1,729 (**-1,266**);
- `src/audit_only/page_count_preference_projection.rs`: 246 -> 193 (**-53**);
- `src/pipeline_shards/final_report_projection_owner.rs`: 1,664 -> 1,661 (**-3**).

Total: **-1,322 Rust lines net**.

Exactly eleven `#[test]` cases belong only to the retired mechanism: ten renderer-feasibility report-shadow tests plus one renderer-outcome transport test. From the accepted r974 baseline of 1,279 strict tests, the expected r975 count is therefore **1,268**. Any different unexplained test loss is a blocker.

## Expected externally visible delta

No `.musicxml`, `.render.musicxml` or `.pdf` semantic/output change is intended. The `.result.txt` report is expected to change only at the retired diagnostic surface:

- `RendererFeasibilityConstraintShadow: ...` disappears;
- `RendererFeasibilityConstraintShadowRenderProof: ...` disappears;
- the surrounding page-count preference section no longer claims `productive_renderer_outcome` as an evidence origin, because that transport no longer exists.

The final `RendererSystemBreakReadback*` lines remain.

## Intentionally unchanged

- no music mutation;
- no final horizontal-layout algorithm change;
- no renderer-feasibility policy or weighting change;
- no source fingerprint rule change;
- no renderer no-good semantics change in the productive oracle;
- no materialization writer change;
- no final PDF parser/readback weakening;
- no release-decision weakening;
- no new gate, framework, ledger, registry, trust boundary or repair authority;
- root `Cargo.lock` remains LocalTransactional and is not part of the take payload.

## Validation expectation

The real Windows take run is authoritative:

```powershell
.\run.cmd --take --all
```

Expected if the retirement is exact:

- 1,268 tests, 0 failures;
- Build green;
- Strict Clippy green;
- 10/10 corpus projects dark-green;
- music/layout artifacts unchanged relative to r974;
- only the explicitly retired report diagnostics disappear.

If r975 is rejected, the payload remains Failure Evidence and any corrected payload receives a new revision.
