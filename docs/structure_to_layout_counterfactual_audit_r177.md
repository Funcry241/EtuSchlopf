# StructureToLayoutCounterfactualAudit r177

## Parent and change class

Parent: accepted EtuSchlopf v0.10.1839 r176 `StructureLayoutCapabilityGapAudit`.

Revision: `r177`  
Name: `StructureToLayoutCounterfactualAudit`  
ChangeClass: `AuditOnly`

r177 is the first implementation of the pre-registered Structure -> Layout information study. It does not select, mutate or materialize a layout candidate and introduces no new musical detector, threshold, scalar structure score, layout planner or writer.

## Research question

r177 asks only:

> Does the already-existing lane-local, multifamily musical structure evidence provide stable qualitative information that distinguishes otherwise technically admissible local break alternatives in the final renderer-selected layout domain?

It does not ask which layout is prettier, more readable or musically correct. A directional audit relation is not a productive preference.

## Final-plan evidence binding

The productive final layout remains owned by the existing pipeline. AuditOnly change control deliberately forbids adding new lines to that productive release owner. r177 therefore does not weaken or route around that boundary.

Instead it composes already-existing read-only evidence:

- the current-process final `RendererSystemBreakReadbackPlan`;
- `HorizontalLayoutFinalSelectedPlanDemandBinding`, whose source is the existing joint-planner measure-demand evidence recomposed exactly onto the final selected plan;
- `HorizontalLayoutRendererFeasibilityCommittedSourceOracle`, including its productive run-local `LearnedNoGoods`;
- the existing phrase/structure owner's canonical exact-measure-boundary evidence.

The final renderer readback must be `MATCH` with no inserted/collapsed system/page breaks before r177 defines a candidate domain.

## Exact-measure-boundary structure projection

r177 retains a complete read-only projection for every canonical measure boundary and every existing lane. It does not rerun phrase detection.

A lane can be observed as:

- `observed_attack_boundary`: an existing adjacent-attack BoundaryObservation arrives exactly at the canonical start of the measure;
- `sustained_continuation`: an existing sounding event crosses the exact boundary;
- `entry_attack_at_boundary`: the lane enters exactly on the boundary but has no preceding attack from which a BoundaryObservation could be formed;
- `silent_gap_across_boundary`: an actual silent interval spans the boundary;
- `inactive_at_boundary`: no stronger exact temporal relation is available.

Only existing exact BoundaryObservations carry their already-derived rhythm, meter, melody, parallelism, novelty, tonal and phrase-role facts. No nearest-boundary window and no distance threshold are introduced.

`local_group_boundary` reuses the existing local closure-structure gate. `closed_span_boundary` reuses the existing `closure_supported` span close. They remain separate structural scales rather than a numeric strength ladder.

## Candidate-domain gate

The final plan is scanned only for page-local adjacent system pairs with lengths `2+3` or `3+2`.

Each formal shape opportunity is transformed only by moving the shared break one measure while preserving:

- the same five measures;
- the same page;
- two systems;
- no singleton creation.

Before structure is consulted, the alternative must pass:

1. the already-bound `AcceptedSystemCapacity`, recomposed from the existing per-measure demands;
2. the productive renderer oracle's exact run-local `LearnedNoGoods` ranges.

The audit records separately:

- `ShapeOpportunities`;
- `CapacityEligible`;
- `NoGoodRejectedAfterCapacity`;
- `ConstraintEligible`.

No corpus count is hard-coded as an expected result.

## Lane-preserving qualitative comparison

r177 preserves lane identity. It does not average or majority-vote voices.

Primary lane witnesses are deliberately narrow:

- `Separation`: exact existing `closure_supported` or `structural_joint`;
- `Continuation`: exact existing `continuation_supported` or an event physically sustained across the measure boundary;
- `Conflict`: the existing narrow authorized tonal conflict;
- `Neutral`: all other states, including missing, inactive, entry-only, silent-gap and unresolved states.

A directional structure relation exists only for a direct, same-lane contrast:

- `Separation` at the baseline boundary versus `Continuation` at the alternative boundary; or
- the symmetric reverse.

If different lanes support opposite directions, or either boundary contains simultaneous separation and continuation witnesses, the result is `PolyphonicOpposition` rather than a vote.

The complete case relation vocabulary is:

- `Equivalent`;
- `DifferentUnordered`;
- `DirectionalBaseline`;
- `DirectionalAlternative`;
- `PolyphonicOpposition`;
- `InsufficientEvidence`.

`DirectionalBaseline` and `DirectionalAlternative` are audit-only information relations. They do not authorize layout change.

## Geometry remains separate

For constraint-eligible cases r177 recomposes only the already-existing joint-planner slack-square geometry from the bound capacity and the same per-measure demands.

It reports:

- `BaselineFavored`;
- `AlternativeFavored`;
- `Equal`;
- `NotComparable` when the alternative failed the constraint gate.

Structure and geometry are never fused into a combined score.

## Phrase-notation dependency confound

The existing pending `PhraseNotationIntentProjection` is observed separately. A candidate boundary is marked when it falls inside an existing phrase-notation span.

This field is never used to select or reject a break. It exists only for pre-declared sensitivity analysis because the accepted phrase-slur materializer is already break-sensitive.

## Report evidence

r177 appends read-only lines to the already-established horizontal-layout evidence projection:

- `STRUCTURE-TO-LAYOUT-COUNTERFACTUAL-AUDIT-r177`;
- one project summary;
- one structure-comparison summary;
- one `StructureCounterfactualCase` line for every final-plan shape opportunity;
- authority and interpretation contracts.

Every rejected case remains visible with the reason represented by `CapacityEligible`, `NoGoodEligible` and `ConstraintEligible`.

## Hard invariants

r177 adds none of the following:

- pitch/onset/duration/sounding-event mutation;
- phrase detector;
- `BoundaryStrength`/`PhraseConfidence` scalar;
- musical weights;
- nearest-boundary window;
- voice majority vote;
- new layout planner;
- structure-to-XML path;
- writer authority;
- project-specific or measure-specific rule;
- external runtime corpus.

There is no layout decision change, MusicXML delta or PDF delta by design.

## Interpretation after the corpus run

A positive information result still proves only that existing structure evidence can qualitatively distinguish some technically admissible break alternatives.

It does not prove improved engraving or sight-reading.

The route should stop if constraint-eligible evidence is too sparse, exact-boundary evidence is predominantly insufficient, directional information is project-specific or confound-dependent, or structure merely restates geometry. Productive influence requires a later, separately authorized decision after the r177 corpus evidence is reviewed.
