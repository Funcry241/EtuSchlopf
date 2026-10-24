# r1469 – Immutable owned-rhythm validation memo

RefactorOnly. Parent: accepted r1468. No declared music change.

## State boundary

`OwnedRhythmicContinuation` stores successful full validation for its own state.
A memo hit still compares the complete snapshot with the supplied XML, without
hashes or pointer identity. A miss retains the previous consolidated-origin,
XML, physical-coverage, ancestry, encoding and support checks in their order.
Only success at the end sets the flag. Errors are not cached.

Every Clone resets the flag, including unchanged clones. Production field
mutations are confined to receive, continue_operations and continue_final_chord
and their consolidation helper: constructors begin uncached, successors clone
before any changes and perform complete validation after their last field write.
There is no production mutation of an already validated instance. The shared
initial custody and snapshot have no interior-mutable musical state. Future
changes must retain this boundary or explicitly invalidate the memo before writes.

An AtomicBool retains Send/Sync. Relaxed operations publish no musical state;
concurrent misses can independently perform the same immutable validation.
Debug keeps exactly the previous fields and order and omits the memo.
No master-state identity, gate, writer, budget or selector is changed.

## Acceptance

Synthetic tests cover cached foreign XML, reset on every clone, unchanged Debug,
repeated failed physical coverage, altered ancestry with identical XML, recovery
from an uncached failure and compile-time Send/Sync.
Existing custody/encoding/transition negative tests remain unchanged.
Strict tests, strict Clippy and unchanged parent/candidate validators are required.
Native acceptance compares all 20 XMLs byte for byte with accepted r1468 and
compares PhaseTiming on the same machine. Old r1450 profiling is not a measured
speedup for this take. Native inputs are not available in this environment.
