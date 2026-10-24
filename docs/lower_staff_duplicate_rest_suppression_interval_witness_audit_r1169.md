# r1169 — LowerStaffDuplicateRestSuppressionIntervalWitnessAudit

## Parent

Accepted dark-green r1168 `RestTimelineStageRuntimeEvidenceProjectionClosure`.

## Observed gap

The existing lower-staff duplicate-rest candidate builder already derives a generic evidence row for every candidate:

- scope
- staff
- technical voice
- onset
- duration
- overlap with another visible rest / same visual beat
- overlap with lower pitched material in another technical voice
- measure-local auxiliary-rest condition

The unchanged application loop selects from the same scored candidate sequence but previously discarded that evidence as `_evidence`. The r1168 runtime report could therefore prove `scope=40` and `duration_to_forward=96`, but not the exact onset/voice of the selected rest.

## Change

r1169 carries the already-computed evidence string only for candidates that the unchanged application loop actually accepts. The selected evidence is appended to the existing `UpperStaffLocalVisualRestLedgerProof` report line as `duplicate_rest_chosen_evidence`.

The candidate order, score, per-scope limit, overlap rejection, replacement XML, authorization call and write/adoption invariant are unchanged.

## Non-goals

- no musical repair
- no selector or threshold change
- no new semantic relation
- no new writer or authority
- no new corpus-specific test identity
- no Master mutation

## Expected diagnostic result

For the E6 witness, the next run should reveal whether the single lower-staff duplicate-rest write in the observed scope has the same onset and duration as the previously observed Whole-R interval. The report is evidence only; it does not classify the elision as authorized or unauthorized.
