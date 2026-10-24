# r1074 FailureAssistantHandoverLedgerIoFixtureClosure

## Zweck

Reapplies the FailureAssistantHandoverClosure directly on canonical accepted parent r1071 after rejected r1072/r1073, while closing the final Windows-only test-fixture defect exposed by r1073.

## Produktiver Vertrag

The productive failure-handover implementation remains unchanged from the rejected r1073 candidate:

- canonical Rejection is persisted or verified before Failure-Assistant-Handover projection;
- stale published Assistant ZIP projections are invalidated for a new changed candidate;
- a completed rejection publishes a current `take_failure` handover bound to exact Parent Acceptance, exact Candidate Rejection, Candidate Intent / Semantic-Effect evidence, terminal reason/stage, transaction/recovery context and available console evidence;
- rollback recovery retains provenance until the failure handover is durable;
- Failure-Handover I/O cannot revoke canonical Rejection;
- no Acceptance, Rejection, Music, Writer, Repair, Selection, Proof or Release authority is added, moved or weakened.

## r1072-Rejection

r1072 was rejected because four legacy recovery unit fixtures had no canonical accepted Parent identity. The new fail-closed Failure-Handover correctly rejected `Parent-Version unknown` against candidate `from_version=0.10.1839`.

## r1073-Rejection

r1073 supplied canonical accepted Parent r514 to those four recovery fixtures. Three formerly failing recovery tests then passed. The remaining test `failed_reexec_rejection_io_failure_preserves_recovery_provenance` failed before its intended assertion because the seeded Parent Acceptance already created `target/etuschlopf_take_rejection_ledger_v1`; the fixture then tried to create a regular file at that existing directory path. Windows reported `PermissionDenied` while constructing the synthetic conflict.

## r1074-Fixture-Closure

Only that test fixture changes relative to r1073:

1. seed the accepted Parent r514 as required by the real failure-handover lifecycle;
2. after transactional rollback, remove the test-only Parent ledger directory;
3. create the deterministic regular-file conflict at the ledger root and prove `record_rejection` fails with `Ledger-Ordner nicht anlegbar` while transaction state/marker remain recoverable;
4. remove the conflict file;
5. recreate the exact accepted Parent r514;
6. run normal pending recovery and prove canonical Candidate rejection plus cleanup.

This keeps the I/O-failure test deterministic on Windows without changing production recovery or failure-handover behavior.
