# Final Serialized Voice Rest Obligation Projection Identity Transport Closure r1136

## Status

r1136 is an independent recovery candidate from accepted parent r1126. Rejected r1128 through r1135 are evidence only and are not parents.

ChangeClass: `MusicMutation`.

## Causal target

The accepted parent error class is `visible_rest_elision_without_bound_rhythmic_dispensability_proof` from `docs/selector_derivation_contract_visible_rest_elision_rhythmic_obligation.md`.

The diagnosed failure has two ordered facts:

1. pre-serialization final-notation polish can hide an entire visible rest from raw same-staff/same-onset coincidence before final serialized voice identity exists;
2. canonical FinalChord encoding later materializes the sealed final voice labels, at which point only the actual same-final-lane pitched interval can justify replacing a subinterval of that visible rest obligation by cursor time.

r1136 removes the early whole-object collision hide and keeps the already-existing canonical `FinalChordEncodingPlan` projector as the single Master-side owner of the interval-local rest projection.

## r1134 runtime evidence and new safety boundary

Rejected r1134 passed the incoming governance and installation gates and reached the real program run. The run then failed closed at the existing pitched-timeline invariant:

`FinalChord visible-rest interval projection changed pitched timeline`

The failure exposed a second structural role that a visible rest object can already own: a non-chord MusicXML rest may be the physical anchor for following `<chord/>` members. Replacing such a rest with a rest/forward sequence can reset the parser anchor state at `<forward>` and thereby move a pitched occurrence even when total cursor duration is conserved.

That role is not dispensable evidence. The r1134/r1135 safety prerequisite remains:

> A visible rest carrier is eligible for interval projection only when no currently parsed chord member is physically anchored to that rest object.

This condition is derived from MusicXML topology and the existing parser's `anchor_index`; it is not a corpus locator, pitch identity, measure identity, or known-witness feature.

## r1135 runtime evidence and identity-transport closure

Rejected r1135 passed the governance/install path and the chord-anchor pitched-timeline invariant, then failed closed in the existing provenance transport with:

`FinalChordProjectionIdentityTransport: plan provenance has no projected pitched note at byte ...`

The failure is byte-topological, not musical. The canonical projector applies normal FinalChord edits first and then applies the already-authorized rest-only interval edits. Those rest edits can change the byte offsets of later pitched notes without changing any pitch, onset, duration, staff, final voice, chord membership or tie endpoint.

The pre-existing provenance transport already carries note offsets through the normal FinalChord edits. r1136 extends that same transport to carry the resulting offsets through the exact rest-only edit set derived from the pre-rest projection. Any provenance offset that would overlap a rest edit fails closed. No source identity is inferred from pitch, onset, measure, staff, voice or corpus location.

## Productive owner

No new pipeline writer caller and no new WriterReachability root is introduced.

`project_final_chord_encoding_plan` remains the already-existing StructuralWriteAuthority canonical Master projector. The rest-only interval application is performed inside that pre-existing projector with the same string-rebuild pattern already used there. New helper functions are calculation-only and do not own XML write authority.

The plan carries one transaction-local authorization bit:

- ordinary/sounding plan: rest interval projection disabled;
- canonical final-voice-bound plan: rest interval projection enabled.

Only after the existing FinalChord edits have physically materialized final voice labels may the projector inspect already-visible rest carriers and same-final-staff/same-final-voice visible pitched occupancy.

The Render/PDF writer is unchanged from r1126.

## Interval rule

For each independently eligible visible rest carrier `R`, collect the temporal union `N` of visible positive-duration pitched occupancy in the same final serialized staff and voice.

Only `R ∩ N` may become a `<forward>` carrier. Every silent component of `R \ N` remains a visible rest, decomposed only through the already-existing metric-rest policy.

Class examples:

- beginning overlap: rest `[a,d)`, pitch `[a,b)` -> forward `[a,b)` + visible rest `[b,d)`;
- ending overlap: rest `[a,d)`, pitch `[c,d)` -> visible rest `[a,c)` + forward `[c,d)`;
- interior overlap -> visible rest + forward + visible rest;
- complete overlap -> cursor carrier only;
- cross-voice overlap -> no edit.

## Fail-closed boundaries

The projection does not act on:

- hidden rests;
- whole-measure rest carriers;
- rest objects that physically anchor any parsed `<chord/>` member;
- hidden pitched events;
- cross-voice or cross-staff pitch;
- absent rests;
- overlapping visible rest carriers in the same final lane;
- non-positive durations;
- silent remainders that the existing metric-rest planner cannot represent.

Every produced edit conserves the source rest duration. The complete pitched semantic timeline `(measure, onset, duration, pitch identity, staff, final voice, chord membership, tie endpoints)` must remain unchanged across the rest-only projection. The r1134 invariant remains in force and is not weakened.

## Synthetic proof

The existing relocatable StructuralWriteAuthority test still proves:

- beginning-overlap projection;
- cross-voice exclusion;
- hidden-pitch exclusion;
- independent-candidate additivity;
- early-plan non-authorization;
- pitched-timeline preservation.

The same synthetic test retains the rest-backed chord-anchor exclusion and additionally proves that a rest-only byte delta occurring before a surviving pitched note is carried through `FinalChordProjectionIdentityTransport`, so projected provenance still lands on the exact final pitched note range.

## Scope

This take does not redesign voice assignment, infer new silence, reveal hidden rests, alter pitch, change Staff/Voice selection, or change the Render/PDF owner. It closes only the Master-side member of the accepted visible-rest-elision error class after authoritative final voice materialization while preserving pre-existing chord-anchor topology.
