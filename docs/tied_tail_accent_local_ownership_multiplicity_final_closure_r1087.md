# r1087 TiedTailAccentLocalOwnershipMultiplicityFinalClosure

Parent: canonical accepted r1084 `PrintCleanupRestProxyObservationPreservingRetirement`.

r1085 was rejected in dependency preflight because of an unsupported manifest key.
r1086 fixed that declaration, but was rejected before installation by NO-NEW-REPAIR-AUTHORITY because it
introduced three new callable functions in the legacy phrase/pipeline shard: one new readback helper and two
new tests. Neither rejection changed installed state.

r1087 keeps the r1086 local A12 product semantics but removes that callable-surface expansion:

- `unique_confusing_tied_tail_accent_key_with_planned` is removed;
- both historical uniqueness-specific tests and their fixture helper are removed;
- the already-existing callable
  `unique_confusing_tied_tail_accent_regression_candidates` is retained by name only for compatibility and
  now reports the full local tied-tail candidate count rather than `1 iff globally unique`;
- no new function is introduced in the phrase/pipeline shard;
- both productive accent paths use all locally matching `AccentAttackOwnershipKey` blockers;
- all three historical `<= 1 blocked row` assumptions are removed;
- the existing articulation-owner duplication test proves that a second independent blocker cannot unblock
  or redirect the first.

The render readback label is generalized from "Unique" to all tied-tail candidates, but calls the retained
existing helper surface. No new positive accent authority, ranking, threshold, project selector, measure
selector or writer root is introduced.

Expected accepted state: immutable r1035 MI-A inventory 12/12 closed, 0 open.

Validation: `run.cmd --take --all`.
