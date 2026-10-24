# Exact non-pitched FinalChord projection continuation

Parent: accepted r1346, candidate SHA-256 396b143ef8b9821182c30fe5e4507ff87cc0e7b33cafc411765577f7b4fc4c6c.
ChangeClass: RefactorOnly.

## Actual scope

The real FinalChord byte projector now returns an exact transaction-local
continuation for every input non-pitched note: visible rests, hidden rests,
unpitched carriers and rest chord hosts. The accepted visible-rest interval
projection carries each resulting rest fragment and Forward back to its actual
source rest. It verifies exact replacement bytes, full non-pitched output-note
coverage, distinct physical inputs, fragment ordinal and duration conservation.
The canonical written-projection binder checks source and output-note coverage.
Pitched provenance and existing byte materialization remain unchanged.

These ranges locate an already represented input and its actual replacement.
They are NOT opaque canonical identity, creation lineage, logical musical voice,
a negative rest obligation, a continuity/run partition or role authority.
No identity is minted from XML coordinates. The continuation is local to the
exact projection transaction, not a persisted origin binding. Existing Forward
objects are not claimed as source rest occurrences; newly emitted Forward
fragments from a rest replacement are covered explicitly.

## Missing authority remains explicit

Every non-pitched continuation has presentation_role=None (Unknown). The current
canonical source locator and independent role inference cover pitched material,
not source rest role origins. Neither label_to_role, a technical lane, staff,
pitch absence nor neighboring notes fill this missing relation.
The r1345 role-occurrence contract is not yet completely implemented. r1347 does
not claim otherwise. Canonical role creation capture at the original authorized
rest producer and transport through all later writers still need implementation.
The new continuation cannot authorize a role-sensitive mutation or channel reuse.
All current role-sensitive consumers and channel allocation remain unchanged.
There is no merge or inferred N:1 origin support in this cut.

## Why this precedes the role cutover

Previously FinalChord provenance deliberately skipped all non-pitched notes.
Consequently even an independently supplied future role origin would be lost
when one rest becomes multiple rests and cursor carriers. This cut closes that
specific transport hole at the sole existing byte-projector boundary. It does
not add a second writer, detector, selector, allocator or musical repair.
No corpus identity or measure-specific decision is introduced.

## Validation

Seven synthetic tests run the actual projector/receipt path and cover physically
equal but distinct hidden rests, exact one-to-many rest/Forward continuation,
opaque label and measure renaming, staff transport with its input, uniform time
scaling, actual rest chord-host protection, missing/extra/stale outputs and byte
displacement after a pitched-label edit. None uses corpus data as a normative
fixture. Full parent admission, Rust tests, Clippy and optimized build are
required before packaging; measured outcomes accompany the delivered Take.
Windows full-corpus acceptance and byte parity are still pending at delivery.
No unforced MuseScore success or capacity repair is claimed.

Rust line balance against r1346: +279 / -0, net +279; 128 added test lines.
The other 335 Rust source files and all frozen parent contracts are unchanged.
