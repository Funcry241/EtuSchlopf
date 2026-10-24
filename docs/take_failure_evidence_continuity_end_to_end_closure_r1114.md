# r1114 TakeFailureEvidenceContinuityEndToEndClosure

## Parent and purpose

r1114 is a successful closeout take built directly from accepted r1112 `TakeFailureEvidenceWriterOwnershipClosure` after the intentionally rejected r1113 `TakeFailureEvidenceWriterOwnershipProbe`.

r1113 already proved the operational red-path invariants: the installed candidate source was bound before reexec, the real strict all-target suite executed and failed at the deliberate probe sentinel, the primary launcher retained `launcher_stage=launcher_tests`, typed child evidence was captured, rollback restored r1112, and the rejection capsule persisted with complete candidate / child / restored-parent binding.

r1114 changes no executable product, test, bootstrap, launcher, transaction, rollback, music, writer, selector, repair, threshold, renderer or release authority. Its purpose is the final continuity proof: a later successful Acceptance must project the already-persisted r1113 rejection capsule into the new canonical green Assistant export through the existing `rejection_suffix` mechanism.

## Required successful result

The accepted r1114 Assistant export must be sufficient by itself to establish both states:

1. r1114 is the current accepted canonical source state and the normal run remains green.
2. the earlier r1113 rejection evidence remains present under `take_attempt_evidence/` with its candidate-bound metadata, executed candidate source, typed child evidence, `launcher_stage=launcher_tests`, rollback provenance and restored r1112 parent source.

Historical evidence is not rewritten. In particular, earlier rejection capsules keep the state they recorded at their own attempt time.

## End-to-end invariant

After successful r1114 Acceptance the demonstrated lifecycle is:

`accepted r1112 -> executed r1113 red -> persistent rejection evidence -> rollback to r1112 -> accepted r1114 green -> r1113 evidence still present in the canonical green Assistant export`.

That is the closure condition for `TakeFailureEvidenceContinuityClosure`:

**Failure-Evidence survives both rollback and a later successful Acceptance without requiring an additional user-supplied diagnostic file.**
