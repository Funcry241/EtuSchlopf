# r1147 - LowerStaffVisibleRestLaneBoundRhythmicDispensabilityWriterReachabilityTestClosure

Parent: accepted r1144 `FinalVisibleRestRoleFrozenVoiceBindingChordAnchorSafetyClosure`  
ChangeClass: MusicMutation

## Cause

The render-only lower-staff anchor suppression selector can convert an already visible rest carrier into a forward when repeated-rest / auxiliary-anchor heuristics fire. Those heuristics can remain true even when the same lower staff/voice lane contains visible pitched material elsewhere, without proving that the concrete rest interval itself is sounding-covered.

That violates the already accepted error-class contract `visible_rest_elision_without_bound_rhythmic_dispensability_proof`: technical context, cursor stability and sounding preservation elsewhere do not prove dispensability of this visible rhythmic interval.

## Productive change

`lower_staff_visible_rest_lane_anchor_suppression_candidates()` keeps its existing local anchor selector. Before a selected candidate may become a rest-to-forward replacement:

- if the same lower staff/voice lane has no visible pitched material, the existing technical-anchor branch is unchanged;
- if the same lane does have visible pitched material, the entire rest interval must be covered by visible pitched intervals in that same lane;
- partial coverage, a gap, or sounding only outside the rest interval yields conservative `Keep`.

The proof is interval-local and performs union-style coverage over clipped same-lane pitched intervals. It does not use project, measure, corpus, pitch, register or numeric voice identity as a selector feature.

## Authority boundary

No new writer is introduced. The existing lower-visible-rest suppression materializer remains owned by `StructuralWriteAuthority`. No new sink, checkpoint, call root or release bypass is added.

## Tests

Two synthetic writer-free unit tests cover the pure interval decision:

1. partial/outside same-lane sounding cannot dispense the complete visible rest;
2. complete same-lane coverage remains eligible, and the pre-existing technical branch with no same-lane pitched material remains eligible when the old selector already authorizes it.

No MusicXML witness identity or concrete measure number occurs in the new normative tests.

## Expected product effect

A render-only visible rest can no longer disappear solely because it sits in a repeated auxiliary-looking lower-staff lane while that same lane has independent sounding responsibility outside the rest interval. Existing positive full-overlap and no-same-lane technical cases remain available to the old selector.

The Windows take reexec remains the authoritative compile, Clippy, strict-test, product, renderer and release proof.
