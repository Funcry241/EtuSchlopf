# r1177 — LowerStaffVisibleRestLaneAnchorIntervalWitnessRefactor

## Parent

Accepted/released r1176 `LowerStaffOverlappingRestUnprovenAdoptionEnforcementRecovery`.

## Purpose

Close only the remaining observability gap in the existing lower-staff visible-rest lane-anchor suppression path that is still active in the E2 corpus result.

The candidate builder already computes a generic evidence row for every anchor candidate containing:

- scope,
- staff,
- technical voice,
- onset,
- duration,
- `pure_anchor_lane`,
- `auxiliary_anchor_lane`,
- `repeated_rest_lane`,
- `near_lower_pitch`,
- `cross_staff_voice`.

The unchanged application loop previously discarded that already-computed row as `_evidence`. r1177 carries the row only after the same candidate has already passed the existing ordering, action budget, per-scope bound and byte-range overlap guard.

## Change

The selected evidence rows are joined into `visible_rest_lane_anchor_chosen_evidence` and appended to the already existing `UpperStaffLocalVisualRestLedgerProof` done/report projection.

No candidate derivation, score, ordering, multiplicity, eligibility predicate, rest-elision obligation, replacement bytes, StructuralWriteAuthority call, adoption invariant or music output changes.

## Expected diagnostic value

For the remaining E2 witness, the current corpus already reports two selected anchor suppressions in scopes 13 and 29. r1177 should expose which exact selected carrier belongs to each scope and which pre-existing anchor predicates admitted it. The run result remains evidence only; no musical repair or new authority is introduced.

## Mutation contract

`RefactorOnly`. MusicXML, Render MusicXML and PDF semantics are unchanged. No new tests call the legacy writer; the existing full validation suite remains authoritative.
