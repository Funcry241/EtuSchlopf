# r1580 - Preserve existing pauses before evaluating existing rest bridges

Parent: natively accepted r1579 / 0.10.1839. Class: MusicMutation.

## Native r1579 findings

The accepted r1579 export confirms ten successful projects, build and Clippy
exit zero and 2881 passed tests, zero failed and three ignored. All 1957
delivered source files match the executed source; only the local root lockfile
is additional. Every pitched XML element with its part, measure and exact onset
is unchanged against r1577 in all twenty Master/Render files. Octave-shift
direction elements and their exact times are also unchanged.

Etude 6 measure 6 lower staff now visibly represents the gap 96..168 with a
dotted eighth rest and 192..384 with two quarter rests. Its note at 168..192 is
unchanged. Those missing intervals are closed. Half-rest compaction is separate.

Etude 1 measure 13 remains unchanged at the reported location: its lower
pitched attacks at 0, 48, 96 and 288 are still individual sixteenths. Its master
has four hidden quarter-rest carriers. Its render has no visible lower rests.

## Corrected beam diagnosis

The ordinary metric-beam rule requires contiguous notes. However, a separate
existing E050 rule already supports bridges across visible rests. Describing
the entire engine as forbidding beams over rests was too broad.
`choose_structural_rest_bridge_candidate_groups` has no visible-rest bridge
for the first-beat lower pair while the intervening rest is absent. With two
sixteenths and no eighth layer, its mixed-hierarchy alternative is also false.

A later original owner, `patch_beat_scoped_rest_completion_in_measure`, can
hide an entire auxiliary rest lane after a cross-staff gap refusal. This runs
on both the Master and Render before final-chord projection. It can therefore
remove an existing visible pause before the final canonical voice assignment
can reconcile that carrier. The original regression explicitly tested that
elision. The export does not contain this owner's original input at the exact
reported location, so its involvement there is still a native-test hypothesis.

## Narrow change

Keep the existing auxiliary candidates and their original occurrence footprints,
but preserve their original bytes. Retain the existing receipt equality check
and mint signature. It returns no receipt on the unchanged carrier. Correct the
hidden count accordingly. The existing test is strengthened with preservation,
hidden-neighbor, label, staff-permutation, repetition and exact-map controls.

There is no new beam rule, no beam forced onto a known measure, no new visible
rest, no hidden-carrier reveal, no pitch/time/staff/voice edit, no new receipt
origin, no new writer and no gate/pin/descriptor weakening. Existing canonical
rest owners and all positive bridge custody/assignment safeguards remain active.

## Native acceptance

Inspect Etude 1 measure 13 lower staff for correctly represented silent spans
and, only if its existing positive beam authority is complete, the first-beat
rest bridge. A correct pause does not itself authorize a beam. Keep the other
beats independent; a lone attack is no group. Check complete attached chords,
final serialization assignment, exact pitched events, ordinary PDF conversion
and all ten etudes for redundant rest lanes or new collisions. Recheck Etude 6
measure 6. A green exit alone does not establish that the requested beam is fixed.

This take is prepared for native validation. It does not claim a completed
Etude-1 beam repair before that validation.
