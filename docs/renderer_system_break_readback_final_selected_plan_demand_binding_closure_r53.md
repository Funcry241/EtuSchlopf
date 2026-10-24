# r53 RendererSystemBreakReadbackFinalSelectedPlanDemandBindingClosure

Parent: committed `0.10.1839 r52`.

r52 is the first corpus-wide productive renderer-feasibility closure:

- full corpus 10/10 DARK GREEN,
- productive renderer readback MATCH for all ten etudes,
- no inserted or collapsed system/page breaks,
- productive renderer-feasibility adoption enabled,
- MuseScore remains feasibility-veto-only while EtuSchlopf remains plan/selection owner.

The only remaining diagnostic mismatch is the legacy
`RendererSystemBreakReadbackDemandBinding` projection. Before r53 it still
validated the post-render readback against the **initial** Joint-Plan system
grouping. For the five pieces where r52 productively adopts a renderer-confirmed
alternative plan, the underlying accepted capacity and per-measure demands are
still valid, but the initial system groups are no longer the final selected
groups. This produced `Status UNAVAILABLE` despite the productive renderer
readback being `MATCH`.

r53 closes only that evidence projection.

## Contract

The already existing `GlobalHorizontalPageSystemShadowAudit` continues to own:

- `AcceptedSystemCapacity`,
- the already computed per-measure demand evidence.

No demand model is recomputed and no threshold/capacity is changed.

After the already existing productive plan selector has returned the final
`HorizontalLayoutLegacyPlan`, r53 recomposes **only the system-load sums** from
those existing per-measure demands across the final selected system ranges and
emits one distinct console evidence row:

`ETUSCHLOPF: HorizontalLayoutFinalSelectedPlanDemandBinding: ...`

The existing readback projection prefers this final-plan demand row for
`RendererSystemBreakReadbackDemandBinding`, while it deliberately keeps the
original `HorizontalLayoutGlobalPageSystemPreferenceCandidate` evidence for the
legacy shadow/replan/proof projections. Therefore the original planner evidence
is neither replaced nor made ambiguous.

The strict parser remains fail-closed:

- final plan slot count must equal the bound measure-demand count,
- accepted capacity must be nonzero,
- each final system range must be valid,
- each recomposed final system load must stay within the already bound capacity,
- the readback validator still requires exact final planned-system ranges and
  exact measure-demand recomposition.

## Non-goals

r53 introduces:

- no music mutation,
- no planner or selector change,
- no renderer-oracle change,
- no new No-Good logic,
- no new width/demand/capacity/threshold model,
- no writer entry point or writer caller,
- no reader authority,
- no repair/recovery layer,
- no persistent state,
- no etude/measure identity special case.

Expected result on the existing r52 corpus: productive renderer behavior remains
10/10 MATCH; the five previously unavailable adopted-plan demand bindings become
`Status BOUND` with source
`final_selected_plan_existing_joint_planner_evidence`.

ChangeClass: `RefactorOnly`.
