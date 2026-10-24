# r1346 — Canonical source presentation roles independent of channels

Parent: accepted r1345, SHA-256
4b04bb1a32319c6fdef01324c8914a1a732a0b90ca7da70df715f9159ffe1e65.
Version: 0.10.1839. ChangeClass: RefactorOnly.

## Result and scope

The accepted canonical class-role relation is now projected directly to opaque
source identities before allocating any technical channel. The frozen channel
plan carries that source-role relation. `role_for_source` reads it directly and
does not reconstruct a source role through `source_to_channel/channel_to_role`.
No role inference, canonical class, conflict edge, channel assignment, label,
semantic fingerprint, selector, output material or productive writer changes.

This is the first executable dependency cut of the accepted r1345 preservation
contract. It is intentionally not a complete occurrence-role plan. A source can
have several written fragments; source identity alone is not their identity.
Rests and non-pitched hosts are not represented by the pitched source relation.
No channel compaction or force-flag retirement is included or authorized here.

The existing allocator assigns every class one accepted role and only reuses
channels of that role. Therefore for every constructed source the new lookup
equals the old channel-based lookup. The allocation algorithm and its existing
semantic signature are byte-for-byte unchanged except for constructing the
additional directly bound role table before allocation and moving it into the
existing plan. The source table is private and cannot grant writer authority.

## Why the planned larger Take had to be split

The current `FinalChordWrittenOccurrence`/canonical source and render-continuation
relations bind pitched material. `final_chord_bind_written_projection` explicitly
excludes rest-like/unpitched notes from its source and output provenance relation.
The final-PDF occurrence continuation likewise transports pitched occurrences.
Rest replacement receipts bind source bytes and edit identities, but do not
transport a canonical presentation role for each created/replaced rest occurrence.
The existing negative rest-ownership audit explicitly remains unbound and grants
no productive authority. It is not a substitute for the missing role provenance.

Three downstream families still use label-based roles: visible shadow-rest
selection, rest-readability grouping and lower primary beat/beam continuity.
The last family also selects a rest slot. Primary-beam partition planning and
validation contain additional label/channel-role reads. They are not migrated
in this Take. Blindly substituting the pitched source-role map for these reads
would either leave unbound rests or make up provenance. Freezing an XML offset,
voice number or geometric match cannot replace an authorized occurrence identity.

The next occurrence binding must originate at the existing creation/transformation
owners and be transported through rest insertion, splitting/consolidation,
carrier replacement and final materialization. It must distinguish multiple
fragments of one source and fail closed on incomplete/stale role provenance.
Only after every role consumer is independent may the role-incompatibility edge
be reconsidered under the unchanged accepted capacity class. Existing sounding,
written, HARD SAME and chord-host constraints remain mandatory. No claim is made
that the source-role cut establishes a feasible bounded channel assignment.

## Synthetic regression evidence

Three Rust tests exercise the real canonical allocator and its frozen role
accessor. They do not merely inspect a manifest or repeat a getter implementation.

- Distinct primary/secondary roles on each staff are first checked against the
  accepted allocation, then their technical channel mapping is changed and its
  channel-role table removed. The exact source roles remain stable. This is a
  test-only counterfactual; productive mixed-role sharing remains prohibited.
- A missing source-role binding returns None even when its technical channel
  still exists. Unknown sources also remain unknown; no channel fallback exists.
- Input order, opaque source identities, measure context, time-unit scaling and
  staff/role transport preserve the accepted canonical role relation.

Existing synthetic tests continue to cover universal channel reuse, incomplete
written provenance, HARD SAME, written/sounding conflicts, label permutations
and independent-candidate duplication. Native corpus material is not used in
tests, features or productive logic. Existing audits continue receiving exactly
the accepted source roles under normal constructed plans.

## Verification and Take payload

Admission uses the unchanged accepted parent implementation on complete source
trees. Run the strict all-target Rust tests, the existing blocking Clippy contract
and the optimized strict main build. None may be substituted by the three focused
tests. Linux checks are not Windows Take acceptance or original-input corpus proof.
The productive renderer command remains unchanged, and this Refactor cannot
provide an unforced-import success claim.

Only serialization_plan.rs, ZIP_KIND.txt, this report and the ordinary Take
manifest differ. All existing contracts, gates, qualified components and nested
locks remain unchanged. The incoming Take excludes local root Cargo.lock and
all derived targets. The source and line-delta checks accompany local validation.

Place the complete candidate in the normal Take input and run:

    ./run.cmd --take --all

The Windows run should retain identical Master/Render MusicXML and musical
decisions. Its actual accepted export, not this candidate, becomes the next parent.
