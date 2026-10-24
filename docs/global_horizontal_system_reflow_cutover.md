# GlobalHorizontalSystemReflowCutover

## Scope

Promote the already shadow-proven global system partition into the single final horizontal plan. Only `starts_new_system` may change. `measure/@width` and `starts_new_page` remain copied from the final-notation Legacy plan.

## Authority

`FinalNotation -> final Planning Authority -> Legacy width/page decisions + GlobalSystemReflow system decisions -> ONE HorizontalLayoutPlan -> existing ONE StructuralWriteAuthority materializer -> Snapshot -> MuseScore`.

Revision r02 keeps `run_render_pdf_source_selection_stage` byte-identical to the last GREEN parent. The r01 placement inside that ProofReentryReader was correctly rejected before installation by the Semantic-Capability-Diff. Promotion now occurs inside the already existing final planning owner `build_final_notation_legacy_plan`; the planner has no writer call. Promotion mutates only an in-memory cloned plan and stamps the system-decision owner as `GlobalSystemReflow`.

## Fail-closed promotion gates

- exact structural slot binding
- exact page-boundary binding / `PageBreakChanges == 0`
- `ForcedOverCapacitySingletons == 0`
- no nonterminal page below three systems
- maximum candidate system length must not exceed the already accepted Legacy maximum
- candidate singleton count must not increase
- promoted plan must be idempotent under the same final facts
- promotion drift must be width-neutral and page-neutral and must reproduce the candidate system-break drift exactly

## Explicit non-scope

No page redistribution, no width demotion, no renderer-auto width policy, no change to VisualLayoutPressure demand scoring, no repair/recovery layer, no project/Etude/measure identity and no second materializer.
