# r128 — PhraseBoundaryCanonicalTimeClippyTypeComplexityClosure

Parent: accepted DARK-GREEN r127 `PhraseBoundaryCanonicalCrossMeasureTimeAuditCompileRecovery`.

## Purpose

r127 completed the canonical cross-measure PhraseBoundary timing audit and passed the real Windows
strict build, Clippy, renderer and final release gates. Its full Clippy run reports 109 warnings.
The accepted r125 parent already reported 108 warnings; the only warning newly introduced by the
cross-measure implementation is `clippy::type_complexity` on the private return type of
`phrase_canonical_measure_timing`.

r128 closes only that newly introduced warning.

## Change

The existing private tuple return type

`(i64, BTreeMap<(String, usize), PhraseMeasureTiming>)`

is given the private type alias `PhraseCanonicalMeasureTiming`, and the function signature uses the
alias. Runtime representation, tuple destructuring, call order, parsing, timing arithmetic,
BoundaryEvidence values and reporting remain unchanged.

No `#[allow(...)]` suppression is added.

## Scope

- ChangeClass: `RefactorOnly`
- no music behavior change
- no PhraseBoundary evidence change
- no threshold or scalarization
- no writer or mutation capability
- no Security policy change
- no change to the 108 inherited Clippy warnings

The inherited warning budget is deliberately kept separate from the new Phrasing feature so a later
cleanup can be reviewed as historical technical debt rather than being mixed into musical work.
