# r1179 LowerStaffVisibleRestLaneNoSameLaneProofPreservationEnforcementRecovery

Parent: accepted/released r1177 `LowerStaffVisibleRestLaneAnchorIntervalWitnessRefactor`  
ChangeClass: `MusicMutation`

## Cause

The accepted causal contract `visible_rest_elision_without_bound_rhythmic_dispensability_proof` requires positive evidence bound to an already-visible rest carrier and its complete interval before that carrier may become cursor-only time.

The existing lower-staff visible-rest lane-anchor owner already has an interval-bound proof path when the same staff/voice lane contains visible pitched intervals. Complete same-lane coverage may authorize the existing whole-carrier elision; partial coverage does not.

Its remaining no-same-lane branch is different. `lower_staff_visible_rest_lane_anchor_elision_permitted()` currently returns `true` immediately when `same_lane_pitched_intervals` is empty. Absence of same-lane sounding is therefore treated as positive technical-helper proof even though the accepted parent contract explicitly requires positive evidence that the carrier has no independent rhythmic obligation.

The r1177 full-corpus measurement confirms that this is a branch family rather than a witness-specific corner: all 16 selected anchor suppressions enter as `pure_anchor_lane`, hence all 16 have no same-lane pitched interval evidence at this decision boundary. That measurement is evaluation evidence only; no project, scope, onset, duration, staff, voice, pitch, count or corpus rarity is used by the productive repair selector.

## Productive change

The existing local anchor heuristic remains unchanged.

`lower_staff_visible_rest_lane_anchor_elision_permitted()` now requires a positive bound same-lane rhythmic proof before it can authorize whole-carrier elision:

- the pre-existing anchor selector must still admit the candidate;
- the rest duration must still be positive;
- an empty same-lane pitched-interval set is now `Keep`, because it contains no interval-bound dispensability proof;
- when same-lane pitched intervals exist, the existing union-style complete-coverage proof remains unchanged;
- partial coverage, gaps and sounding only outside the rest interval remain `Keep`.

This does not claim that a no-same-lane rest is musically necessary. It only removes deletion authority from this owner when this owner has no positive proof. A later already-authorized semantic role owner may still remove a carrier that it independently proves to be technical shadow bookkeeping.

## Anti-overfitting boundary

The change is not derived from the known corpus location that exposed the defect.

No condition uses:

- project or Etude identity;
- measure/scope identity;
- onset or exact duration;
- numeric staff or voice identity;
- pitch/register/clef;
- a particular combination of the old anchor flags chosen because it isolates a witness;
- current corpus multiplicity or uniqueness.

The repair follows directly from the prior causal obligation: absence of same-lane evidence is not positive evidence of rhythmic dispensability.

## Positive neighboring behavior retained

The existing same-lane full-coverage branch remains positive and unchanged. If visible pitched intervals in the same lane cover the complete rest interval, the current owner may still authorize the existing replacement.

The change creates no rest, reveals no hidden rest, changes no pitched event, and introduces no new writer, sink, threshold, score, ordering, action budget or release rule.

## Regression contract

The pre-existing helper-test function identity and its three call sites are retained; r1179 adds no new direct writer/helper caller.

The retained test function now asserts:

1. complete same-lane interval coverage remains positive;
2. an empty same-lane interval set is not a positive dispensability proof;
3. the pre-existing selector guard still blocks authorization even with complete interval coverage.

Multiplicity invariance is structural here: the productive proof function has no candidate-count, corpus-count or global-ranking input, and r1179 leaves the planner/order/action-budget machinery byte-identical to r1177.

The Windows take reexec is the authoritative compile/Clippy/test/product proof. The first product run is intentionally targeted; the subsequent full-corpus run decides the observable product distribution rather than preserving any predetermined number of prior suppressions.

## Recovery from rejected r1178

r1178 was rejected before transaction start solely because a newly added test function became a new caller in the legacy writer-reachability graph. r1179 does not consume r1178. It rebuilds the same general rule directly from accepted r1177 while retaining the pre-existing test-function identity and call-site count.
