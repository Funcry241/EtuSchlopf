# r1462 — Final rest placement after serialized voice binding

Historical record: r1462 was rejected in the native corpus. Its rest-order fix
was incomplete because the sink could still change the clef course. See
`final_clef_rest_fixpoint_after_beam_voice_r1463.md` for the current correction.

Accepted parent: r1460, version 0.10.1839. Rejected r1461 is runtime evidence,
not an accepted parent. ChangeClass: MusicMutation.

The original-beam alignment implementation and authority/safety conjunction
from the rejected attempt are carried forward. Native tests and zero-warning
Clippy passed there, but the complete corpus rejected: Etudes 7 and 10 retained
five and thirteen final rest-placement obligations respectively. The whole take
was rolled back to r1460. Eight individual successful runs do not constitute
corpus acceptance or a completed visual review.

## Causal integration correction

The existing final-rest placement predicate depends on the final per-voice end
and overlap relation. A complete-source technical beam voice reassignment may
complete a formerly partial lane. An opposite-clef display anchor that was not
eligible for canonical rebinding before that reassignment then becomes eligible.
The previous order could therefore prove rest closure before the late scalar
write and fail the same unchanged predicate on the committed artifact.

Move the single existing `stage_final_rest_vertical_placement_candidate` call
after late beam voice materialization and its exact occurrence transport.
Keep silent-lane coverage, orphan-chord normalization, primary clef projection
and same-lane interior splitting on their original input relation. No structural
writer consumes the rebound relation. Only the existing final rest-glyph owner
consumes the final relation, exactly once, before the existing checkpoint sink.
This supersedes the rest-placement portion of the historical r1067 ordering;
its other structural-writer isolation requirements remain in force.

The rest selector and glyph policy are unchanged. No new rest role, voice,
fragment, rest split, duration, display policy or writer root is introduced.
The existing glyph owner may change only the existing authorized rest display
coordinates. Its exact receipt remains in the same FinalNotation chain. A thin
continuation adapter uses the existing pitched-topology-neutral transport and
verifies the resulting source against the final canonical structure and rebound
voice plan. The persisted zero-residual closure and all release gates remain
unchanged. An unsafe or ambiguous plan still fails closed.

## Regression evidence and acceptance

A generic synthetic original-owner fixture must reproduce zero eligible stale
anchors before the late scalar change and nonzero anchors after it. The actual
FinalNotation preparation must then close them in its one final rest stage,
retain exact receipts, canonical pitched source continuation, the beam voice
assignment and a zero-change second placement probe. This is mechanism evidence,
not a reconstruction of unseen native artifact bytes.

Final local qualification will use the unchanged r1460 admission and installed
reexec validators, focused regression tests, full all-target tests and strict
zero-warning Clippy. Native acceptance still requires ten successful fresh
unforced PDFs, unchanged ten Master XMLs, Render changes restricted to the
admitted voice scalars and canonical rest display coordinates, and visual
inspection of Etude 6 plus the full corpus. Custody remains paused.

## Completed local qualification, 2026-10-20

- Rust 1.98.0: all 2,335 tests across 15 executables passed with
  `cargo test --locked --all-targets --config profile.test.opt-level=0
  -- --test-threads=4`. One optional external-renderer test was excluded here
  and executed separately below. The source profiles were not changed.
- The targeted actual FinalNotation preparation/readback regression passed,
  reproducing the newly eligible rest anchors after a late complete-source
  voice rebind, closing them with the existing rest owner and proving the final
  canonical source/voice continuation and idempotent rest placement.
- Strict all-target Clippy with `-D warnings -D clippy::correctness
  -D clippy::suspicious -D clippy::perf` passed with zero warnings.
- The separate actual MuseScore 4.7.4 test passed: fresh unforced PDF geometry
  changes from disconnected flags to one canonical primary beam group after
  the existing voice-scalar owner. This is a synthetic renderer proof, not
  native Etude 6 visual acceptance.
- The source is checked using the unchanged r1460 incoming validator. The
  final ZIP is byte-roundtripped and checked again, then checked as an installed
  `reexec` transaction with an exact r1460 managed-source backup. No validator,
  Kodex, audit, dependency, profile or renderer-force policy is changed.

Native ten-project acceptance and visual review remain pending. Put this ZIP
in the project directory or Downloads, then execute `run.cmd --take --all`.
Return its fresh assistant export. The rejected r1461 must not be treated as
an accepted parent or as completed musical acceptance.
