# r1463 — Final clef/rest fixpoint after original-beam voice alignment

ChangeClass: MusicMutation. Accepted parent: r1460, version 0.10.1839.
The r1461 and r1462 candidates were rejected and rolled back; neither is parent.
The r1462 rejection is independently read from
`assistant_export_latest(20261020-094900).zip`. Its exact candidate SHA-256 is
514eca2584bcc87f0423c249198260d639a144b71831968fc522cc477291c1c9.

## Cause

r1462 moved the existing rest placement behind late beam voice materialization.
The native corpus still rejected Etudes 10 and 7: final rest candidates were
0 before / 13 after the sink and 5 before / 5 after respectively. The final
PrimaryClef reentry check was already zero. No failed native PDFs/XML artifacts
are included, so this is log evidence, not native visual acceptance.

The unchanged FinalNotationPreparation sink applies
`derive_clef_readability_primary_course_plan`. Its legal boundaries depend on
voice-indexed open primary beams. The earlier PrimaryClef projection still saw
the split voices; the sink saw reunited groups and could reanchor a clef before
the group. Rest coordinates computed between these two different clef courses
then became stale. A zero post-sink clef reentry count does not prove the sink
preserved its prepared input.

## Single-owner correction

Move the single existing PrimaryClef projection after late beam scalar
materialization and exact source occurrence transport, directly before the
single existing final rest placement. The order is now:

1. Original structural FinalNotation bundle, including Ledger-Clef and Same-Lane.
2. Existing late beam voice scalar batch and exact occurrence transport.
3. Existing PrimaryClef projection, once, on those final voice/beam bindings.
4. Existing final rest placement, once, on that final clef course.
5. Existing pitch-topology-neutral transport across both authorized surfaces.
6. Unchanged prepared FinalNotation checkpoint sink and persisted release checks.

This ordering correction adds no selector, glyph/clef policy, channel allocation,
physical writer or gate. The adapter on the existing source continuation carries both clef and
rest surface changes under the already rebound voice plan. All existing receipts
remain in the same chain. Sink, #58 files, Kodex and audits stay unchanged.

The Ledger-Clef measure/chord decision uses pitches, duration, staff, clef and
octave state, not Voice labels. Same-Lane explicitly uses Voice and temporal
occupancy; it remains on the original input relation. Moving PrimaryClef cannot
change its Voice/timing predicate. The existing late batch safety still refuses
new occupancy collisions and headless pitched import keys. No additional late
structural rewrite or repeated finalizer is introduced.

This supersedes the PrimaryClef ordering in the historical r1067 and rejected
r1462 integration only. The original-beam authority/safety conjunction remains
unchanged. Master content and the solver are unchanged. Native Render deltas may
contain admitted voice scalars, the existing final clef-course consequences and
canonical rest display coordinates, all under their existing authorities.

## Causal regression

A synthetic lower-staff figure has split primary beam voices and a whole rest
in an independent voice between the serialized group members. It uses the
actual original beam producer, canonical projector, existing scalar owner,
authenticated synthetic genesis and the actual live FinalNotation checkpoint.

With the unchanged r1462 ordering the test fails at the real sink readback:
one rest anchor was written, one stale rest candidate reappears after the sink,
PrimaryClef reentry is zero, and cursor/pitched/other residual checks remain
green. This reproduces the mechanism independently of native project identity.

The corrected regression additionally captures the prepared source and requires
byte-identical actual sink output, zero remaining rest targets, a byte-identical
PrimaryClef reprojection and a closed canonical source continuation. The earlier
lane-completeness regression remains required.

## Acceptance

Local full all-target tests, zero-warning strict Clippy, the unchanged external
MuseScore beam proof and original r1460 incoming/installed-reexec validators
must pass. Native acceptance still requires ten fresh unforced successful PDFs,
ten invariant Master XMLs, only authorized Render surface changes, and visual
review including Etude 6. Custody and the withdrawn T0/T1 plan remain paused.

## Completed local qualification, 2026-10-20

- The real-sink regression fails on the r1462 ordering specifically with one
  rest candidate reappearing after the sink. PrimaryClef reentry is zero and
  all other recorded residual/cursor/pitched checks are green.
- Both final-rest regressions pass after the correction, including byte equality
  between independently prepared candidate and actual live sink output, zero
  remaining rest targets, final canonical source continuation and clef fixpoint.
- All 2,336 tests across 15 executables pass with Rust 1.98.0 using
  `cargo test --locked --all-targets --config profile.test.opt-level=0
  -- --test-threads=4`. The source profiles are unchanged.
- Strict all-target Clippy passes with `-D warnings -D clippy::correctness
  -D clippy::suspicious -D clippy::perf`, zero warnings.
- The separate actual MuseScore 4.7.4 regression passes without force, proving
  the original-owner beam alignment still restores the canonical visible group.
  This excluded optional test is executed separately, not counted above.
- The recovered r1462 native parent managed-source bytes equal this r1460
  baseline. The final ZIP is byte-roundtripped and independently submitted to
  the unchanged r1460 incoming validator and installed/reexec validator with
  an exact parent backup. All #58 pins, validators, audits, dependencies and
  profiles are unchanged.

Native ten-project acceptance and PDF review remain pending. Place the ZIP in
the project directory or Downloads and execute `run.cmd --take --all`. Return
the fresh assistant export. Rejected r1461/r1462 are not accepted parents.
