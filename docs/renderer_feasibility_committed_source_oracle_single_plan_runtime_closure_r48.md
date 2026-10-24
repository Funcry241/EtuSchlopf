# r48 RendererFeasibilityCommittedSourceOracleSinglePlanRuntimeClosure

Parent: committed `0.10.1839 r42`.

`r47 RendererFeasibilityCommittedSourceOracleStrictClippyClosure` passed the
architectural preflight, strict Clippy, build and all Rust targets, then reached
the new committed-source renderer oracle at runtime. Etuede 6 was confirmed in
two oracle rounds with the exact run-local No-Goods `24-26,35-37`. Productive
adoption remained disabled as intended.

The run failed only after the productive PDF render because the existing
`PageCountPreferenceShadowProjection` correctly requires unique Joint-Planner
evidence and observed two
`HorizontalLayoutGlobalPageSystemPreferenceCandidate` lines. Inspection shows
that r47 executed `build_final_notation_legacy_plan(...)` twice on the same
committed pre-layout XML: first in the Metric/FinalNotation owner to seed the
oracle, then again in the later release owner before productive layout
selection. The projection failure therefore exposed a real duplicate planning
step rather than an audit-parser defect.

## Runtime closure

r48 removes that duplicate planning step instead of weakening the projection.

1. The committed pre-layout XML is closed exactly as in r47.
2. `build_final_notation_legacy_plan(...)` runs exactly once in the existing
   Metric/FinalNotation owner.
3. Its `HorizontalLayoutLegacyPlan` becomes a typed committed initial-plan token.
4. The veto-only renderer oracle clones that token for run-local candidate
   iteration.
5. The same original typed initial-plan token is transported to the later
   release stage.
6. The release stage no longer recomputes the Joint Planner. It only applies the
   existing `select_renderer_feasibility_productive_plan(...)` policy to the
   transported initial plan and oracle outcome.
7. Productive adoption remains disabled, so selection still returns the original
   committed initial plan.

This is a reduction in planning authority and work: no new writer, reader,
helper authority, threshold, width model, demand/capacity policy, persisted
No-Good state, repair layer, identity special case, or audit relaxation is
introduced. The strict unique-evidence projection remains unchanged.

## Expected Etuede-6 behavior

- committed-source oracle: `RENDERER_CONFIRMED`
- expected oracle rounds: `2`
- learned No-Goods: `24-26,35-37`
- `ProductiveAdoptionEnabled nein`
- exactly one Joint-Planner candidate projection for the committed source
- productive layout remains the r42 plan
- productive renderer drift is therefore still expected until a separately
  gated activation revision is designed

ChangeClass: `RefactorOnly`. Parent: `0.10.1839 r42`.
Expected strict test inventory remains `764/764`.
