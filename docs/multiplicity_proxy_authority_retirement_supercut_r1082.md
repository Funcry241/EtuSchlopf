# r1082 MultiplicityProxyAuthorityRetirementSuperCut

Parent: canonical accepted r1081 `ExternalFeedbackAccentTenutoRetirementCompileClosure`.

Change class: `MusicMutation` under `authority_retirement_v1`.

## Why this SuperCut

The accepted parent has already retired MI-A11. The remaining MI-A07, MI-A08, MI-A09 and MI-A10 surfaces
share one architectural defect: score-wide multiplicity or ranking can act as hidden identity for a product
writer. None requires a new musical rule to remove that authority. Their common safe operation is monotone
retirement.

The four retirements are combined so that one full-corpus validation proves one coherent writer-authority
reduction. The two semantically different residuals remain out of scope: MI-A05 is intertwined with the
broader print-cleanup trial, while MI-A12 requires a per-candidate or explicit conflict-group blocker proof
before score-wide uniqueness semantics may be changed.

## Exact productive cuts

1. MI-A07 SamePitchTieMerge: the productive call with `max_actions=1` is no longer invoked.
2. MI-A08 UpperTerminalShadowRest: the globally selected/capped writer is no longer invoked.
3. MI-A09 CrossStaff HandRoleMaterialization: the one-winner materialization trial is no longer invoked.
4. MI-A10 LowerStaff VisibleRhythmRestLane: the one-winner lane trial is no longer invoked; later
   independent lower-staff suppression/coalescing stages remain active and unchanged.

Each detached legacy implementation remains compile-live through a non-invoking function-value reference,
matching the accepted r1046 authority-retirement pattern.

## Intentionally unchanged

- all four legacy candidate classes;
- all score-wide ranking/ordering logic inside the detached implementations;
- all `max_actions` semantics inside those implementations;
- thresholds and calibration constants;
- accepted r1081 MI-A11 retirement and its compile closure;
- MI-A05 and MI-A12;
- unrelated beam, clef, register, rest and articulation writers;
- sounding-event semantics.

## Validation

The authoritative command is:

`run.cmd --take --all`

Full corpus is required because four productive render writer roots are retired across several stages.
