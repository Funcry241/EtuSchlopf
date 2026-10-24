# r1019 OrphanChordAnchorBeamContinuationClosure

Parent: r1018 `OrphanChordRestHostAnchorPromotionWarningDebtClosure`.

The accepted r1017/r1018 orphan-chord repair promotes an existing visible pitched chord member from a hidden rest host to the physical anchor. In the unique E4/T30 witness that promoted 16th is also the metric head of an already serialized `16th + eighth` primary beam tail. The pitch promotion was correct, but beam ownership remained on the later notes, leaving the promoted anchor visually unbeamed.

r1019 extends only the existing orphan-host promotion transaction. If, and only if, the promoted anchor is a visible 16th with no beam, followed on the same staff by a visible 16th in another voice carrying primary `begin`, then a visible eighth in the promoted anchor voice carrying primary `end`, the existing `ChordGestureDistributionCapability` authorizes the exact complete-beat beam continuation:

- promoted anchor: primary `begin`, secondary `begin`
- following 16th: primary `continue`, secondary `end`
- following eighth: existing primary `end` unchanged

The selector remains bound to the already unique hidden-rest-host anchor promotion. It does not rewrite unrelated cross-voice beams and introduces no new writer, function, capability family, project selector, measure selector, or pitch selector. Pitch, onset, duration, voice, staff, note cardinality delta and raw cursor remain under the existing r1017 readback proof.

## r02 compile recovery

The first r1019 candidate was rejected before tests because `anchor_promotion_edits` was moved into the existing authorized batch writer and then borrowed for `.len()`, producing Rust E0382. r02 preserves the r1019 selector, beam edits, writer authority and readback semantics byte-for-byte except for ownership bookkeeping: the edit count is captured as a scalar before the vector is moved, and the post-write cardinality check compares against that scalar. No musical decision or write surface is changed.
