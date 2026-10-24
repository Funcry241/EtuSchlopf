# r1324 — Written channel compatibility with exact chord-head bounds

Parent: accepted r1323, candidate SHA-256
f4cef99c9686dbcbd6348ed95f818fae73a469ec379627c62862b4e7b894a9b8.
ChangeClass: MusicMutation.

r1323 was accepted and completed ten corpus projects with zero process failures.
It changed no Rust and did not resolve the musical collision problem. r1322's
rejected implementation passed 1,921 Windows tests but aborted in one corpus
project with a generic HARD SAME contradiction. A synthetic production probe
confirmed that its equal-end chord predicate rejects a legal shorter member;
the exact historical conflicting pair is not established by that export.

This candidate implements written-channel compatibility from the newly accepted
class. Exact projected note provenance and complete XML bind the immutable written
occurrences to canonical source/fragment/attack evidence. Freeze verifies exact
projected XML. Reuse checks all channel members against written overlap, frozen
sounding constraints and existing role authority. Canonical HARD SAME is never
split. Supplemental labels cannot take reserved present main-role labels.

Synchronous chord authorization requires the actual positively pitched physical
nonmember head, exact cohort and canonical HARD SAME. Every positive member must
start with that head and end within its actual extent. A shorter member is legal;
member-to-member equality or the longest member is not substituted for head proof.
Rest/orphan encodings retain full provenance with false positive-anchor authority.
Contradictions still fail closed and now expose actual pair intervals, head bounds
and continuity IDs in the diagnostic. These IDs never select a repair.

Only two productive Rust owners change: serialization_plan.rs and the existing
candidate_material_lineage_spine_core.rs freeze boundary. All gates, SSC-pinned
initial-render orchestration, master-state receipt authority, existing beam/rest
consumers and the accepted class are unchanged. There is one new implementation
contract referencing that unchanged accepted class. No second projector, new
external writer caller or audit-to-write authority is introduced.

Twelve owner-local synthetic tests cover written versus sounding overlap,
universal channel compatibility, adjacency, independent geometry versus canonical
chords, missing provenance, identity/order invariance, independent multiplicity,
label reservations and projection continuity. Production-path chord probes vary
units, member order and both staff presentations. Head-bound counterfactuals
reject oversized or shifted members, inconsistent head bounds, unknown pitched
hosts and different exact cohorts; removing HARD SAME cannot authorize sharing.

Completed native validation: Rust/Cargo 1.98.0, Linux x86_64, dev profile.
`cargo test --locked --all-targets --no-fail-fast`: exit 0, 1,923 passed,
zero failed, across 15 targets, zero warning lines. All-target Clippy with
correctness, suspicious and performance deny flags: exit 0, zero warning lines.
The unchanged build gates execute; the host build-script optimization override
is opt-level=1. No Rust changes follow these successful checks.

Rust line-multiset balance against accepted r1323: +645 / -30 in two files;
334 other Rust files remain byte-identical. The accepted class, SSC-pinned
orchestrator, master-state receipt authority and all build-gate implementations
are unchanged. The existing rejected-auxiliary-lane preservation test retains
its complete body and passes. Root Cargo.lock is local-only and excluded from
the incoming archive; nested tool locks remain included.

The r1323 corpus reports ten VoiceTimelineCollisions in Master and ten in Render
across the ten projects. This is the unchanged baseline, not a candidate result.
The next Windows export must measure actual candidate collision counts, sounding
pitch/onset/duration continuity, canonical chords and legitimate hand roles.
Native local tests cannot replace the Windows strict and original-input run.
Original-input Windows corpus, actual collision counts and unforced MuseScore
import remain pending. This candidate does not claim zero musical collisions.
Removal and gate prohibition of MuseScore -f remain conditional on successful
musical convergence, not on the definition-only r1323 acceptance.

Put only this candidate into the normal Take input and run:

    ./run.cmd --take --all
