# r1043 - PartialBeatLaneMeasureLocalMultiplicityClosure

## Parent and purpose

Parent: r1042 `PartialBeatLaneMultiplicityCausalClassAndDebtSync`.

r1042 froze the witness-blind `partial_beat_cursor_lane_serialization_split` class and the one-measure conflict boundary without changing productive Rust. r1043 is the implementation stage for MI-A03. It removes scorewide uniqueness as write authority while retaining the living cursor-lane owner and all existing music-preservation guards.

## Productive change

`human_bound_cursor_lane_owner_transfer_xml` still discovers exactly the same stitch plans. The old scorewide rule

`candidates.len() != 1 -> no write`

is replaced by deterministic physical-measure conflict groups.

- one existing plan in a physical measure: evaluate that plan through the existing stitch, BeamPlan and exact readback invariants;
- more than one plan in the same physical measure: that measure remains fail-closed;
- candidates in different physical measures: independent transactions; one may not veto the other;
- no global ranking, first/next winner, scorewide cap or measure-number preference is introduced.

The existing single-candidate path is preserved through a dedicated single-measure helper using the same mutation and invariant proof. Multi-measure adoption is final-guarded by unchanged note count, complete event-multiset equality, parse validity, backup-forward validity and exact readback for every adopted local transaction.

## Candidate-duplication proof

The regression suite now proves both directions required by `candidate_duplication_v1`:

1. duplicating a valid candidate into a second independent measure produces two repaired measures and leaves the first repaired measure byte-identical to its singleton decision;
2. multiple candidates in one physical measure remain fail-closed.

This is the intended MI-A03 closure. It does not touch MI-A04 or any later multiplicity surface.

## Expected validation

ChangeClass: `MusicMutation`.

The normal full-corpus take run is required. The previous 1,349-test baseline gains two regression tests, so the expected strict count is 1,351 if no unrelated test topology changes occur.
