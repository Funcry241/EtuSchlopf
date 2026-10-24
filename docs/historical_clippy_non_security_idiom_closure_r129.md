# r129 — HistoricalClippyNonSecurityIdiomClosure

Parent: accepted DARK-GREEN r128 `PhraseBoundaryCanonicalTimeClippyTypeComplexityClosure`.

## Purpose

The accepted r128 strict Clippy log contains 108 inherited warnings. The new PhraseBoundary
`type_complexity` warning introduced by r127 is already closed in r128; the remaining warnings are
historical technical debt.

r129 starts that cleanup with the smallest non-security, non-branch-restructuring idiom cluster.
It changes exactly three physical warning sites and deliberately leaves all `collapsible_if`,
Security/Build-Gate/Launcher-pin work and the seven architectural-budget warnings for later takes.

## Changes

- `src/render.rs`
  - replaces a literal-only `format!` with the equivalent `.to_string()`.
- `src/pipeline_shards/composer_recovery_octave_beam_export_tail.rs`
  - replaces `Option::map_or(true, predicate)` with the equivalent `Option::is_none_or(predicate)`.
- `src/pipeline_shards/horizontal_layout_global_reflow.rs`
  - replaces `len() % 2 == 0` with `len().is_multiple_of(2)`.

These are value-preserving standard-library idioms. No threshold, branch ordering, music rule,
candidate selection, writer authority, Security policy, report schema, PhraseBoundary evidence,
MusicXML/PDF semantics or pass order changes.

## Expected Clippy effect

Accepted r128 baseline: 108 warnings.

Static expectation after r129: 105 warnings:
- `useless_format`: -1
- `unnecessary_map_or`: -1
- `manual_is_multiple_of`: -1

The exact warning count is not promoted to a release gate and must be confirmed by the real Windows
strict Clippy run.

## Scope

- ChangeClass: `RefactorOnly`
- no warning suppression
- no Security/Build-Gate source changes
- no Launcher source changes
- no PhraseBoundary source changes
- no music mutation change
