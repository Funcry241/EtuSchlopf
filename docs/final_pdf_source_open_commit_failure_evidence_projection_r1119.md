# r1119 FinalPdfSourceOpenCommitFailureEvidenceProjection

## Purpose

r1119 is a diagnostic-only RefactorOnly take on accepted r1118. It changes no musical selection, no writer authority, no notation mutation, no threshold, no release rule and no acceptance condition.

The r1118 full-corpus run is formally blocked in several projects by the existing fail-closed boundary `HorizontalLayout cannot preserve an open FinalPdfSourceCommit`. The underlying `FinalPdfSourceCommit` already carries the exact pipeline failure/readback line that explains why it remained open, but the outer HorizontalLayout preservation error currently discards that evidence.

## Change

When HorizontalLayout receives an already-open `FinalPdfSourceCommit`, the existing error now appends `source_commit.pipeline_line()` to the same failure. The commit remains open, the run remains failed, and no downstream authority is granted.

Before:

`HorizontalLayout cannot preserve an open FinalPdfSourceCommit`

After:

`HorizontalLayout cannot preserve an open FinalPdfSourceCommit; <existing FinalPdfSourceCommit pipeline line>`

## Scope

- failure-path text projection only;
- no music/XML/PDF mutation;
- no new diagnostic subsystem or persisted ledger;
- no gate weakening;
- no change to `FinalPdfSourceCommit.closed` or any readback predicate;
- no project, measure, pitch or witness identity in executable selection logic.

## Expected use

Run the corpus unchanged. For every project that still fails at this boundary, the console/error evidence exposes whether the open commit originates before the sink (for example occurrence continuation) or from one of the existing committed readback predicates. That evidence is then used to decide whether the blockers share one cause or require separate future error classes/repairs.
