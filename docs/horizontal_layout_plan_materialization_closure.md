# HorizontalLayoutPlanMaterializationClosure

Take 6 closes horizontal legacy layout materialization without changing the
legacy policy.

## Contract

- `apply_pianist_layout` computes the existing page/system seed but no longer
  writes the computed bare `new-page` / `new-system` geometry.
- DenseRegisterNotation, VisualLayoutPressure, PianisticReadability and
  FinalShortSystemBalance update only the central
  `HorizontalLayoutLegacyPlanBuilder` for horizontal width/break decisions.
- Vertical layout behavior such as `staff-distance`, visual defaults and
  octave-shift `default-y` remains in its existing owner and is outside this
  horizontal materialization closure.
- After the last legacy decision the builder closes into an immutable plan.
- Exactly one source-bound writer,
  `materialize_horizontal_layout_plan_authorized`, materializes the complete
  structural slot vector.
- The materializer receives no project, etude, measure number, pitch, staff or
  voice identity.
- `FinalShortSystemBalance` preserves the legacy explicit
  `new-system="no"` surface as a behavior-neutral materialization detail.
- No redundant-system-break cleanup is invoked after PlanClose.
- Exact plan/materialized parity remains fail-closed immediately after the one
  materializer and again against the final snapshot-bound PDF source.

## Deliberately unchanged

The two-measure base grid, page arithmetic, all existing scores, thresholds,
caps, target widths, VisualLayoutPressure selectors, PianisticReadability
selection, FinalShortSystemBalance decision rule, musical mutation rules,
renderer behavior and PDF-source selection are unchanged.

This take centralizes writer ownership only. It does not introduce the future
global reflow policy.


## r03 dead-code closure

r02 reached Strict Clippy and proved that the old horizontal writer modules had
become productively unreachable. r03 closes that debt without an allow-list or
synthetic reachability: the five superseded writer modules are test-only, preserving
their local regression suites, and their product re-exports are removed. The closed
plan parity method remains; only the unused pre-close duplicate and an obsolete
number-based content-start locator are deleted. Productive horizontal materialization
continues to have exactly one source-bound entry point.
