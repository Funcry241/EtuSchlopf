# StructureToLayoutEvaluationStateClosure r178

Version: `0.10.1839`  
Revision: `r178`  
ChangeClass: `AuditOnly`

## Purpose

r177 correctly gated final-plan `2+3 <-> 3+2` alternatives by existing capacity and productive run-local renderer no-goods before musical structure was consulted. Its report semantics nevertheless used `StructureRelation=InsufficientEvidence` for a constraint-rejected case whose structure bundle had deliberately not been evaluated.

That conflated two different research states:

1. the candidate is technically rejected, so structure evaluation must not occur;
2. the candidate is technically eligible and structure evaluation occurs, but the retained exact-boundary evidence is insufficient.

r178 closes only this reporting/data-state ambiguity. Candidate discovery, capacity eligibility, no-good eligibility, geometry comparison and structure comparison rules remain unchanged.

## Typed evaluation state

Each counterfactual case now carries an explicit `StructureEvaluationStatus`:

- `NotEvaluatedConstraintRejected`
- `Evaluated`

`StructureRelation` is optional. A constraint-rejected case has no structure relation at all and is emitted as:

```text
StructureEvaluationStatus NotEvaluatedConstraintRejected
StructureRelation n/a
```

A constraint-eligible case is evaluated. Only such evaluated cases may produce:

- `Equivalent`
- `DifferentUnordered`
- `DirectionalBaseline`
- `DirectionalAlternative`
- `PolyphonicOpposition`
- `InsufficientEvidence`

Therefore `InsufficientEvidence` now has one precise meaning: technical eligibility passed, structure evaluation ran, and the retained musical evidence did not support a stronger relation.

## Summary semantics

`StructureComparisonSummary` adds:

```text
NotEvaluatedConstraintRejected <count>
```

All relation counters operate only on actual `Some(StructureRelation)` values. Constraint-rejected cases therefore cannot inflate `InsufficientEvidence` or any other musical relation count.

## Regression coverage

The module pins both sides of the distinction:

- a rejected constraint returns `NotEvaluatedConstraintRejected`, `StructureRelation=None` and `n/a` in the report;
- an eligible case with missing structure rows returns `Evaluated` plus `InsufficientEvidence`.

## Authority and scope

r178 does not alter:

- final renderer-selected plan discovery;
- shape opportunity discovery;
- accepted capacity;
- renderer no-goods;
- geometry costs;
- lane witnesses;
- polyphonic comparison;
- phrase-notation dependency detection;
- layout selection;
- MusicXML/PDF materialization;
- musical content.

No candidate is selected or materialized. No new detector, threshold, scalar musical score, majority vote, proximity window, planner, writer, project identity rule or measure-specific rule is introduced.
