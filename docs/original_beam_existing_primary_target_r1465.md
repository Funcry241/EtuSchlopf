# r1465: existing primary target within an authenticated original beam group

Parent: accepted r1464, version 0.10.1839. Class: MusicMutation.
Parent ZIP SHA-256: 53b8bda66b5ea0513f254850a0b61f072492fc5092ff2ea9219fd84927c54085.

## Change

Use the first member in the unchanged authenticated group order whose source
role AND already bound channel role are the same-staff primary role. Never
synthesize a role, channel, source, attack or fragment. Carry that exact chosen
member through target source/attack/fragment fields and the existing late
rebind, scalar writer and occurrence continuation. The first member may be
secondary, or may have a primary source role in a secondary-role shared channel.

All existing membership, all-level beam, timing, staff, chord, label-sharing,
occupancy, batch, late-drift and receipt checks remain in force. The existing
final Voice -> Clef -> Rest -> Sink order and physical writers are unchanged.
No change to C7, renderer geometry coverage or chord-member support. Unknown
geometry remains unknown and skipped. No project, measure or numeric voice
label selects a productive change. No audit/proof reader or gate is changed.
The unchanged parent error-class contract remains authoritative; its existing
implementation contract is updated to describe this target choice accurately.

## Qualification and limits

The five additional synthetic cases exercise: a later primary target through
late rebind, the actual scalar writer and occurrence transport; a primary source
in a secondary-role channel with a later suitable member; absent primary source
or channel; a bystander-rest occupancy conflict despite an eligible later target;
and individually safe assignments that conflict when combined. Existing tests
retain negatives for missing/stale/foreign origin, ownership drift, tied closure,
label permutation, duplication, headless keys and unknown geometry.

Local qualification uses Rust 1.98, locked dependencies, the unchanged Strict
build gates and full target tests. The test command overrides only this package's
Strict optimization to 0, as in the native test launcher, with debug info and
incremental compilation disabled for host resources. Production profiles, source
configuration and gates are not edited. Strict Clippy and isolated parent incoming
and installed/reexec validators must also pass on the final source/ZIP.

Dachs' Linux E6 prototype reported 11 -> 23 prepared groups and 17 additional
voice scalars, with unchanged Master XML. This is advisory evidence from Dachs,
not a reproduced native result or a promise for other etudes. This Take is not
natively accepted until the user's fresh run succeeds.

## Native acceptance

Run `run.cmd --take --all` on accepted r1464, then upload assistant_export_latest.zip.
Require 10/10 projects, FinalExit/BuildExit/ClippyExit 0, unchanged ten Master XMLs,
Render differences confined to voice scalar values, unchanged slur endpoints by
measure/onset/pitch/staff, committed readback green and channel profiles admissible.
All ten PDFs must render with -o without -f. Compare prepared-component counts
for benefit, rather than accepting a fixed count as authority. Baseline is r1464,
whose music XMLs are byte-identical to r1463. A native failure triggers diagnosis;
no automatic second Take. Maximum two music Takes; this is the first candidate.
