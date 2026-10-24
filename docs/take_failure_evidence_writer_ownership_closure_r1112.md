# r1112 TakeFailureEvidenceWriterOwnershipClosure

## Parent and witness

r1112 builds directly on canonical accepted r1110 `StrictAllTargetsReceiptTestUniverseBindingClosure`. r1111 `TakeFailureEvidenceExecutedCandidateProbe` is rejected Failure Evidence and is never a parent. The real r1111 failure proved the executed-candidate source, strict-test log, child exit and rollback correctly, but its persisted attempt stage was contaminated from `launcher_tests` back to `building` while `cargo test --all-targets` was running.

The direct cause is that the attempt-local evidence directory is inherited by Cargo and all descendant test processes. The existing launcher unit test `observation_stage_does_not_drive_console_phase` calls `Launcher::enter_observation_stage(Building)` and therefore wrote into the same `CHILD_STAGE.txt` as the real launcher. The same inherited surface could also append to the typed artifact registry.

## Closure

The primary launcher now claims an attempt-local `WRITER_OWNER_PID.txt` exactly once with `create_new` at the beginning of `run()`. Stage and artifact-registry writes are accepted only when the current process PID equals that recorded owner PID. An already existing owner file is never replaced.

This makes the writer authority process-local while preserving inheritance of the evidence-directory path for ordinary child execution. Cargo, unit tests, integration tests and any launcher binary started by those descendants may read the attempt context but cannot mutate the primary launcher's write-ahead stage or artifact registry. If the owner file is absent, malformed or cannot be created, write-ahead remains fail-closed and the existing parent-bootstrap recovery/rejection authority is unchanged.

No transaction, rollback, Acceptance, Rejection, music, writer, repair, selector, threshold, renderer or release authority is added or moved.

## Regression

The existing test that produced the r1111 contamination is extended rather than adding a new test. It proves that an existing non-owner PID cannot be replaced, cannot write `CHILD_STAGE.txt`, and cannot append `CHILD_ARTIFACT_REGISTRY.txt`; after explicit test-local ownership is changed to the current process, the same helper paths can write. The strict-test inventory therefore remains 1,394.

A later operational failure probe must show `launcher_stage=launcher_tests` while preserving the already proven executed-candidate, strict-test-log and rollback evidence.
