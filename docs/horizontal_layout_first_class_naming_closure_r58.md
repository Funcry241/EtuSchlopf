# HorizontalLayoutFirstClassNamingClosure — r58

## Purpose

r58 closes the four-take cleanup series without changing productive behavior.
The renderer/horizontal-layout architecture is already corpus-proven. This take
only makes the Rust names reflect the authority that already exists.

## First-class renames

The productive central plan is no longer called a legacy plan:

- `HorizontalLayoutLegacyPlan` -> `HorizontalLayoutPlan`
- `HorizontalLayoutLegacyPlanBuilder` -> `HorizontalLayoutPlanBuilder`
- `LegacyHorizontalLayoutSeedPlan` -> `HorizontalLayoutSeedPlan`
- `LegacyHorizontalLayoutSeedSlot` -> `HorizontalLayoutSeedSlot`
- `HorizontalLayoutDecisionOwner::LegacySeed` -> `Seed`

The productive shard names are no longer shadow shards:

- `horizontal_layout_legacy_decision_plan.rs`
  -> `horizontal_layout_decision_plan.rs`
- `horizontal_layout_global_reflow_shadow.rs`
  -> `horizontal_layout_global_reflow.rs`
- `horizontal_layout_page_count_preference_shadow.rs`
  -> `horizontal_layout_page_count_preference.rs`
- `horizontal_layout_global_page_system_shadow.rs`
  -> `horizontal_layout_global_page_system.rs`

The active plan/audit vocabulary now expresses current semantics:

- baseline plan / baseline systems
- reflow plan / reflow systems
- selected Page/System plan / selected systems
- nonterminal thin-page metrics

The internal read-only report adapter is renamed from
`page_count_preference_shadow_projection` to
`page_count_preference_projection`.

## Intentionally retained historical terminology

This take does **not** blindly remove every occurrence of `legacy` or `shadow`.

Historical names remain only where the historical meaning is still real:

- retired/compatibility projection concepts;
- regression evidence that explicitly checks former behavior;
- renderer-feasibility temporary probe/shadow artifacts that are not the
  productive selected output;
- the legacy pianistic-readability compatibility projection;
- the protected retired page-slot calibration observation;
- persisted report labels such as
  `HORIZONTAL-LAYOUT-PAGE-COUNT-PREFERENCE-SHADOW`, `LegacyThin` and the
  corresponding console prefix.

Those persisted strings are kept byte-compatible deliberately. They are report
contract labels, not executable authority names. Changing them would turn a
pure refactor into an observable report-contract migration.

## Invariants

r58 changes no:

- music mutation or writer reachability;
- Page/System planning algorithm;
- candidate ordering or scoring;
- renderer-feasibility oracle authority;
- run-local No-Good behavior;
- demand/capacity calculation;
- threshold or policy;
- materialization call;
- report values;
- Security Control Plane;
- identity or persistence contract.

The expected target-system result is therefore behaviorally identical to r57:
Etüde 6 remains DARK GREEN with 19/19 systems, 6/6 pages, renderer readback
`MATCH` and final-selected-plan demand binding `BOUND`.

After that single-target commit, `run.cmd --all` is the acceptance closure for
the complete four-take cleanup series.
