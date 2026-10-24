# r1320 — Written voice collision class and implementation boundary

Parent: accepted r1319, SHA-256
ae43515d52e57f4347c808632ad6fc45255840fd19414ccae5fd94840ec91579.
ChangeClass: KodexOnly. Rust line balance: +0 / -0. Every Rust file remains
byte-identical to the accepted parent. This is the prerequisite definition take,
not a collision fix and not a proof that MuseScore imports without force.

## Why this prerequisite is required

Take control requires selector-sensitive new logic to reference an error-class
contract already present, unchanged, in an accepted parent. The accepted contracts
about visible beam partition loss and silent-voice shadow-rest conflicts have
different causal requirements; they do not authorize a general written timeline
collision repair. Creating this definition and its implementation in the same take
would violate the parent-contract requirement. This take freezes the general
written-channel incompatibility class before its implementation.

## Current source findings and limits

The accepted channel builder derives SemanticDifferent from frozen sounding
intervals. Written FinalChord durations may differ. A sounding-only constraint
therefore does not prove that the written obligations of a shared channel fit.

Channel reuse currently iterates individual member classes. A channel is offered
by any compatible member; there is no explicit all-members rejection for that
channel. A later implementation must establish compatibility with every member
of an offered channel, not merely one member. This is a source-level condition;
no claim is made that it caused a particular historical collision.

Label binding processes all channels of each presentation role before proceeding
to the next role. Fallback chooses the smallest unused number. Additional channels
of an earlier role can consume the canonical label of a later main role. A later
binder must protect positively designated main-channel labels before allocating
additional labels. No numeric voice label may establish musical role.

The rejected implementation's opposite-hand fallback bypassed role incompatibility.
That is excluded. A written conflict can demand an additional compatible channel,
but does not independently prove a new hand, staff or presentation role.

## Concrete implementation requirements after acceptance

1. Produce written-occurrence evidence inside the canonical production owner at
   FinalChord projection. Bind exact output note spans to its existing origin
   provenance and canonical source/fragment/attack identities. Consume the actual
   written onset/duration/chord witness; never read audit_only occurrence rows as
   productive authority. Audit projections may observe this production evidence.
2. Establish an admissible production carrier to the existing freeze boundary.
   The initial-render orchestrator is hash-frozen by SSC governance. A new reader
   must not secretly rerun materializing writers from an external caller, infer
   occurrence identity by matching note geometry, or edit a frozen orchestration
   file in an ordinary MusicMutation. The carrier/closure route must be verified
   against the existing architecture before implementation is packaged.
3. Build local written incompatibility under the unchanged accepted class. Keep
   legal positively bound chord cohorts together. Verify complete represented
   occurrence coverage, explicit omissions and all-member channel compatibility.
   Preserve HARD SAME classes and sealed canonical presentation roles.
4. Protect independently designated main labels and allocate additional channels
   without displacing those main labels. Define cases with multiple channels of
   one role and shared secondary/auxiliary preferences explicitly. A blanket
   reservation of six role numbers is insufficient because roles share 3/4.
5. Bind the semantic channel evidence to both the written conflict relation and
   resulting channel partition. Preserve alpha-invariance: opaque ID or label
   renaming cannot change musical constraints or roles. Check existing fingerprint
   consumers before altering the fingerprint's meaning.
6. Keep materialization in existing StructuralWriteAuthority owners, with complete
   existing exact receipts and SSC permits. No new external writer-call layer,
   fallback, acceptance bypass or witness-specific repair is permitted.

## Required checks for the later music take

Synthetic owner-local tests must cover: written vs sounding duration divergence;
positive legal chord cohorts; endpoint adjacency; both staff presentations;
contradictory HARD SAME evidence; complete vs missing occurrence provenance;
all-member channel compatibility; additional independent channels preserving
main labels; alpha-renaming and candidate duplication; preserved beam/rest role
consumers and exact receipt coverage. The concrete inputs must be relocatable
invariants, not named historical etude/measure witnesses.

After native strict build/tests and full corpus execution, inspect actual final
Master and Render written timelines. The objective is zero incompatible voice
collisions, preserved sounding pitch/onset/duration and legitimate chord notation.
Separately inspect changed hand, rest and beam presentation; preserved pitches
alone do not prove musical correctness. Run the actual MuseScore import/export
without -f and retain command/exit/output evidence. A previous successful PDF
render with -f is not that evidence. Musical quality remains a distinct review.

## Current execution and next stage

This candidate was packaged and inspected statically; no native Rust/Cargo or
Windows MuseScore execution is available here. Put only r1320 in the normal Take
input and run the authorized one-command flow:

    ./run.cmd --take --all

After acceptance, the new class exists in an accepted parent. Implementation then
requires exactly one changed implementation contract referencing this definition
unchanged, with authority/safety-based repair scope and multiplicity invariance.
The source findings above are design constraints, not a promise that a specific
future algorithm or governance carrier has already passed native admission.
