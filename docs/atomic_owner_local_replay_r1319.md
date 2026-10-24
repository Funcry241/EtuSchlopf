# r1319 — Owner-local atomic receipt replay

Parent: accepted r1317, SHA-256
b564cf3f5b0e9cdc2715c90421124d2a2ccefb74feaa4d201eec8dfde8e83606.
ChangeClass: RefactorOnly. No productive source body or gate is changed.

## r1318 rejection and correction

r1318 SHA-256 f0832a23b99c798db5288585c210efb581cdca6f767c3175c563652f46d3e49e
was rejected at incoming_take by NO-NEW-REPAIR-AUTHORITY before installation/build.
The four new test callers were outside StructuralWriteAuthority. Its architectural
inventory scans cfg(test) functions as well as productive functions; placing
writer-reaching tests in Pipeline or Master tests was not admissible.

This candidate starts from the restored r1317 parent, not from rejected r1318.
All new writer-reaching tests reside inside their actual structural owner modules.
They call only existing canonical owner functions, never external Pipeline writers.
The rejected Pipeline include, new shard and Master test are absent. Nothing in
build_gate, take_change_control, identity inventory or SSC governance is changed.

## Four tests

1. SafeTransfer owner: 18 changed transitions across two anchor orders, three
   voice-width pairs and three source/anchor duration pairs. The origin-bound
   exact receipt must realize the actual materialized output and cover it.
2. Bound staff/voice owner: six moves with growing/shrinking/equal lane widths and
   an existing or missing staff tag. Actual receipts must reproduce actual bytes.
3. Scalar/restification owners: three two-owner chains (six changed transitions).
   Restification ranges are bound against the real intermediate scalar output;
   both local receipts and their ordered concatenation must cover the output.
4. Expected fatal mechanism probe: an invalid transfer anchor inside its original
   note creates overlapping pre-state edits. The real owner emits changed bytes,
   refuses the exact receipt, coverage observes an empty chain, and the existing
   sovereign adoption must panic with SafeAtomicCandidateProducer/Patch's message.

Thus 30 regular changed owner transitions and one expected fatal probe are tested.
These are owner-level tests, not a replay of rejected Pipeline wrappers. The #63
outer RestifyPatch owner and historical corpus selection are outside this scope.
The invalid anchor does not establish the cause of r1313 Etude 6 or the separate
Windows access violations. No historical musical claim is made.

## Validation and run

Static inspection checks unchanged productive bodies, canonical structural call
closure, selector contract linkage, corpus-independent fixtures, manifest hashes
and archive readback. No native compilation or execution is possible here because
Rust/Cargo and original corpus inputs are absent. Native admission remains pending.

Place only r1319 in the normal Take input and run:

    ./run.cmd --take --all

Expected tests after success: 1911 rather than accepted r1317's 1907. All music
outputs should remain unchanged. The negative test emits ExactReceiptRefusal and
passes only when the existing fatal check fires. A failing positive test rejects
the candidate. A green result excludes these synthetic local configurations only;
it is neither a musical collision fix nor a historical crash reproduction.
