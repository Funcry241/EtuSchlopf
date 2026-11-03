# r1579 - Preserve existing visible rests before final projection

Parent: r1578 / 0.10.1839. Class: MusicMutation.

This is a narrow monotone retirement of the final writes in
`hide_redundant_visible_staff_rests`, using the exact source successor admitted
by r1578. The original function used a raw rest-only channel and a single
remaining orientation glyph as permission to hide other rest intervals. That
does not prove the full intervals dispensable under the accepted causal class.

Existing visible rests survive this owner unchanged. Previously hidden carriers
stay hidden. The take introduces no rest, reveals none, changes no sounding
event, staff/voice assignment, source identity, tie, chord, clef or beam, and
does not alter C3 or its lexicographic policy. The input signature narrows
to a shared string slice, expressing that this former elision owner now only
observes its unchanged candidates. The original canonical rest
projection and consolidation owners continue independently.

Expected effect: previously elided visible rest intervals may now remain
available to the later canonical final-chord rest projection. That existing
owner can reconcile a visible carrier with its final same-lane sounding
projection. It deliberately does not treat a hidden carrier as visible silence.

This is a testable corpus repair hypothesis. The export contains the final
master and render, not the early input of this particular elision call. A new
native run must show whether the two reported locations are actually affected.
Do not report success merely because the new synthetic preservation tests or
ordinary launcher gates pass.

## Native checks

- Etude 6 measure 6 lower staff: inspect the gap from tick 96 to 168 and from
  tick 192 to 384; establish the visible rest representation without changing
  the pitched onset at tick 168 or its duration 24.
- Etude 1 measure 13 lower staff: inspect preserved/rest-projected intervals
  around attacks at ticks 0, 48, 96 and 288. The current ordinary beam policy
  forbids connecting noncontiguous members; this take does not resolve a new
  cross-rest beaming requirement.
- Inspect all ten etudes for newly visible redundant rest lanes, rest/pitch
  collisions, full-measure rest notation and unchanged sounding material.
- Require successful ordinary native conversion, final closure, full tests and
  Clippy. No new global veto, forced renderer option or gate relaxation applies.

This take does not claim a completed fix for all missing rests, the Etude-1
beam question, complete canonical negative-rest authority or total runtime.
