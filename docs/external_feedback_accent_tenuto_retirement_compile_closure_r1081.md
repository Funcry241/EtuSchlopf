# r1081 ExternalFeedbackAccentTenutoRetirementCompileClosure

Parent: canonical accepted r1077 `RejectionLearningEvidenceBoundaryClosure`.

Change class: `MusicMutation` with `authority_retirement_v1`.

## Failure evidence

r1078, r1079 and r1080 are rejected evidence only and never become parents.

- r1078 proved that cutting MI-A11 at the outer mutation entrypoint does not satisfy pure monotone
  authority-retirement shape.
- r1079 proved that cutting the final private source-bound materializer does satisfy the pre-take
  authority-retirement gate, but its post-apply reexec returned exit 101.
- r1080 preserved the accepted materializer-layer cut and fixed the two directly stale MI-A11 regression
  expectations. It again passed the complete pre-take gate and installed, but the post-apply reexec still
  returned exit 101. Its failure handover does not contain the compiler/test stderr.

## Remaining compile-closure evidence

Against accepted r1077, the admitted materializer retirement removes the sole productive call to:

`structural_write_authority::replace_bound_note_articulation_authorized`.

A full source scan of the r1080 tree finds only its declaration/re-export and no caller. The root project
runs warning/dead-code detection fail-closed; an authority retirement must therefore remove this causally
orphaned writer surface rather than suppress or fake-use it.

r1081 keeps the r1080 materializer cut and test closure and additionally removes only:
1. the `replace_bound_note_articulation_authorized` re-export;
2. the now-unreferenced function itself;
3. its private `RenderEditEffect::NoteArticulation` enum variant;
4. the dead variant side of the validation match, preserving the identical live `ArticulationBatch` rule.

No suppression, dummy call or replacement writer is added.

## Intentionally unchanged

- MI-A11 candidate discovery;
- scorewide `candidates.len() != 1` multiplicity veto;
- target selection and source fingerprint binding;
- all thresholds;
- the outer `close_structural_accent_tenuto` owner;
- MI-A12 and all other multiplicity-debt paths;
- sounding events, beams, rests, clefs and register logic except for removal of historical MI-A11
  Accent-to-Tenuto product writes.

The selector remains compile-live for later physical cleanup after productive authority retirement is
accepted.

## Validation expectation

The real Windows command remains authoritative:

`run.cmd --take --all`

Full-corpus validation is required because historical corpus evidence shows MI-A11 produced product writes
in more than one etude.
