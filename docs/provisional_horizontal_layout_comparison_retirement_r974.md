# r974 ProvisionalHorizontalLayoutComparisonRetirement

## Parent

`r974` is a `RefactorOnly` retirement built directly on the accepted `0.10.1839 r973` parent.

## Scope

The early `MasterNotationAndSalience` phase previously kept a `HorizontalLayoutPlanBuilder` alive through productive notation polish, then additionally:

1. ran `FinalShortSystemBalance` on that early state;
2. closed the builder into a provisional `HorizontalLayoutPlan`;
3. stored that plan in `SolverCandidate`;
4. carried it to release;
5. compared its decisions with the committed-source final plan;
6. compared its captured planning facts with final persisted notation facts.

Only steps 1-6 are retired. The productive XML polish and its live guard remain.

## Productive XML polish preserved

The early builder is still created from `HorizontalLayoutSeedPlan` before the relevant notation polish. It remains live while:

- `FinalNotationPolishGuard` runs;
- export XML normalization runs;
- `VisualLayoutPressure` performs its still-productive XML polish;
- `PianisticReadability` performs its still-productive staff-distance and octave-shift placement polish;
- `working_xml_is_compatible` continues to fail closed if any of those phases writes horizontal geometry outside the layout owner.

`PianisticReadability` still receives the same planned-system-state information from the local builder. No replacement context or forwarding layer is introduced.

## Final horizontal-layout authority preserved

Authoritative planning remains on the committed final-notation source:

- `build_final_notation_plan`;
- final `VisualLayoutPressure` decisions;
- final `FinalShortSystemBalance`;
- GlobalHorizontal reflow/page-system selection;
- RendererFeasibility productive selection;
- `materialize_horizontal_layout_file_authorized`;
- render phrase-projection materialization;
- `FinalNotationLayoutSnapshotSet`;
- `HorizontalLayoutPlanContractProjection`;
- final `matches_materialized` readback.

The final planner continues to own all productive system/page decisions and the single productive horizontal-layout materialization.

## Protection mapping

The retired plan-decision drift and fact-drift reports did not reject changed field values or changed decisions; they reported them. Their only fail-closed structural case was a mismatched slot count. That protection is already owned by `HorizontalLayoutPlanBuilder::from_seed_for_final_notation`, which rejects when the structural seed slot count differs from the committed final-notation slot count.

The early `close()` provenance check also ceases to be needed as a carried-plan gate. The local builder is constructed through typed seed ownership and the only surviving early mutation is `plan_new_system`/`plan_new_page`, which assigns the corresponding owner when it changes a directive. The authoritative final plan still performs its own `provenance_complete()` check before productive use.

## Retired mechanism

The cut removes:

- `SolverCandidate.horizontal_layout_plan`;
- its two `None` initializers;
- the early `FinalShortSystemBalance` invocation;
- early builder close and provisional-plan transport;
- release-stage dependency on the provisional plan;
- `HorizontalLayoutFinalNotationPlanningCutover` provisional-vs-final drift report;
- `HorizontalLayoutPlan.planning_facts`;
- `HorizontalLayoutFinalNotationFactDriftAudit`;
- the fact-drift audit data type and its exclusive unit test;
- writeback of the final plan into `SolverCandidate`, which had no later consumer.

## Mechanical balance

Relative to accepted r973, the probe removes 186 Rust lines and adds 10 comment/signature lines: **176 Rust lines net removed**.

Exactly one `#[test]` belongs only to the retired fact-drift mechanism. If no other test-target count changes, the expected real test total is therefore **1,279**, down from the accepted r973 baseline of 1,280. Any different unexplained test loss is a blocker.

## Intentionally unchanged

- no music mutation;
- no XML-polish semantic change;
- no final layout decision algorithm change;
- no renderer-feasibility policy change;
- no global page/system preference change;
- no materialization writer change;
- no Snapshot/PlanContract readback weakening;
- no threshold or weighting change;
- no new gate, ledger, registry, trust boundary, transition wrapper or recovery mechanism;
- root `Cargo.lock` remains LocalTransactional and is not part of the take payload.

## Validation expectation

The real Windows take run is authoritative:

```powershell
.\run.cmd --take --all
```

Expected if the retirement is exact: 1,279 tests, 0 failures, Build/Clippy green, 10/10 corpus dark-green. If r974 is rejected, the payload remains Failure Evidence and any corrected payload receives a new revision.
