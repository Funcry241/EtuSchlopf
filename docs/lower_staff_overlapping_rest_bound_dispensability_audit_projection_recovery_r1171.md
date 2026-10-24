# r1171 LowerStaffOverlappingRestBoundDispensabilityAuditProjectionRecovery

## Parent

Accepted r1169 `LowerStaffDuplicateRestSuppressionIntervalWitnessAudit`.

The rejected r1170 candidate is evidence only and is not a parent.

## Purpose

Audit the already selected lower-staff duplicate-rest suppression witnesses without changing any mixed/productive Source path.

r1169 already reports, for each actually selected candidate, the bound locator fields `scope`, `staff`, `voice`, `onset`, `duration` and the selector predicates. r1171 consumes only that accepted evidence from the final report projection and compares the complete rest carrier with visible pitched occupancy in the final Master and Render timelines.

For each selected carrier the audit reports:

- full rest interval `R`,
- foreign-voice visible-pitch coverage inside `R`,
- uncovered foreign-voice subintervals,
- same-lane visible-pitch coverage inside `R`,
- uncovered same-lane subintervals,
- corresponding final Render coverage,
- whether the r1169 witness reported pitch overlap,
- positive technical-role evidence observed at this writer,
- coverage authority,
- verdict.

Coverage is observation only. It does not become a write right. Because the identified writer does not consult a positive technical-role/dispensability authority, this audit reports `UNPROVEN` rather than deriving permission from foreign-voice overlap or even full foreign-voice coverage.

## Recovery boundary

r1170 failed before transaction start because AuditOnly added data transport to a mixed Source file. r1171 does not modify that file or any other productive rest planner/writer file. The new implementation is contained in `src/audit_only/` and the explicit Audit-owned final report projection.

## Expected E6 evidence

The real corpus witness is not encoded as a normative test identity. On the actual E6 run, r1169 already locates the selected carrier. r1171 should make its final-artifact interval evidence visible. If the observed foreign-voice pitch occupies only the first part of the carrier, the remaining silent tail must be shown as uncovered and the verdict must remain `UNPROVEN`.

## Mutation contract

AuditOnly. No music mutation, no selector or score change, no candidate ordering change, no thresholds, no new authority, no release-gate change.
