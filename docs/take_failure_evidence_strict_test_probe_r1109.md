# r1109 TakeFailureEvidenceStrictTestProbe

## Purpose

r1109 is an intentionally rejected operational probe. Its accepted parent is r1108 `TakeFailureEvidenceFoundationArchiveClosure`. The probe exists only to exercise the generation-bound Failure-Evidence authority that became operational after r1108 acceptance.

## Controlled failure

The existing integration test `wrapper_console_state_is_neutral_and_errors_use_stderr` retains all of its prior assertions and appends one runtime-only `std::hint::black_box(false)` sentinel. This is intentionally compiled cleanly but must fail when the strict all-target test suite executes. The diagnostic marker is `R1109_TAKE_FAILURE_EVIDENCE_PROBE`.

The probe therefore targets the strict-test stage rather than dependency preflight, compilation, Clippy, product runtime, music transformation, writer, selector, repair, threshold or release behavior. No product authority is changed.

## Expected lifecycle

The accepted r1108 bootstrap owns this transaction. Before child reexec it must bind the installed r1109 candidate source and create attempt-local write-ahead evidence. The launcher must register the strict-test artifact before starting the strict all-target run. The intentional test failure must produce a nonzero child outcome; the r1108 bootstrap must then roll back to r1108, persist canonical Rejection, finalize the candidate-bound rejection capsule and include that capsule in the current Failure Assistant export.

The expected final state is therefore **r1109 rejected / r1108 restored and still canonical**. Acceptance of r1109 would be a failed probe.

## Required evidence

The resulting Failure Assistant export should be sufficient by itself to prove at least: intended candidate identity, exact executed candidate source, reached launcher/test stage, strict-test evidence containing the probe marker, child exit, restored r1108 parent source, canonical Rejection and persistent rejection-evidence status.

No historical r1106/r1107 evidence is reconstructed. No `.musicxml`, `.render.musicxml` or `.pdf` change is intended.
