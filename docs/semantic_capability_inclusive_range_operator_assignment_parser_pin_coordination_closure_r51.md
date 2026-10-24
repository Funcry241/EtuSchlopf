# r51 — SemanticCapabilityInclusiveRangeOperatorAssignmentParserPinCoordinationClosure

## Purpose

Close the concrete Security-Gate parser defect discovered by rejected r49 while respecting
EtuSchlopf's byte-pinned architecture ownership contract.

## Parser defect

`standalone_assignment_operator()` treated the `=` byte in Rust `..=` as an assignment.
For a loop such as `for end in min_end..=max_end`, threshold inventory could therefore
invent a second `min_end` assignment and bind unrelated later source text into its RHS.

The parser correction is deliberately minimal: `.` is an excluded predecessor for a
standalone assignment `=`. The regression test proves that the real
`let min_end = start.saturating_add(1);` assignment remains inventoried while
`min_end..=max_end` adds no assignment.

## Pin coordination

Rejected r50 changed the pinned scanner without changing its pin owners and was therefore
correctly rejected before copy. r51 atomically updates the existing trust chain:

1. `semantic_capability_diff.rs` — parser fix plus regression test.
2. `identity_debt_build.rs` — expected FNV-1a64 for the semantic scanner only.
3. `identity_gate_trust_anchor.rs` — compile-time and test pins for the semantic scanner and
   for the changed build-policy owner.

No pin assertion, ownership check, security policy or fail-closed boundary is removed.

## Non-goals / invariants

- ChangeClass `SecurityOnly`.
- Parent is committed r48; rejected r49/r50 are not parents.
- Productive renderer-feasibility adoption remains disabled.
- No Music-sensitive source change.
- No calibration threshold value or policy change.
- No writer, reader, planner, renderer, oracle or artifact behavior change.
- No `--force` and no replay of a rejected SHA-256.
