# r1461 local qualification and native acceptance

Historical record: r1461 was later rejected in the native corpus and rolled
back to r1460. The r1462 correction was also rejected. The current r1463 fix and acceptance
requirements are in `final_clef_rest_fixpoint_after_beam_voice_r1463.md`.

Parent: accepted r1460, version 0.10.1839. Change class: MusicMutation.
This take addresses visible original beam fragmentation caused by a later
technical serialization voice partition. The general Custody task remains paused.

## Local evidence

The full `cargo test --locked --all-targets` run passed 2,334 tests across
15 test executables, with the optional external renderer test excluded.
The additional cases exercise the actual beam owner, canonical projector,
existing scalar owner, exact receipt and occurrence continuation. They cover
negative authority, complete source closure, secondary beam decisions, scalar
occupancy safety, label renaming and independent candidate duplication.
The test profile used opt-level=0 to fit the local Linux qualification host;
the source profiles and native Strict checks are unchanged.

The isolated renderer test uses an explicitly installed official MuseScore
4.7.4, without force. It renders the actual canonical projection before and
after the existing authorized scalar writer. PDF vector geometry must prove
fragmentation before and the original canonical group count after. Its test
score starts at measure one because that readback requires consecutive
score-measure numbering. Other synthetic tests use relocated measure domains.

Final local results (2026-10-20):

- All 13 focused tests passed, including the external renderer test, using
  `cargo test --locked --bin etuschlopf original_beam_voice_alignment::tests::
  --config profile.test.opt-level=0 -- --include-ignored --nocapture
  --test-threads=1` and the explicit MuseScore wrapper environment.
- The synthetic piano fixture uses two staves and explicit stem directions,
  matching the existing readback input contract. The unchanged PDF vector
  readback proves four disconnected primary fragments before and one after.
  Visual inspection independently shows four sixteenth-note flags before and
  the continuous double beam after. Both PDFs were freshly rendered without force.
- `cargo clippy --locked --profile strict --all-targets -- -D warnings
  -D clippy::correctness -D clippy::suspicious -D clippy::perf` passed on the
  final Rust source without warnings.
- The unchanged parent r1460 incoming and installed/reexec validators are
  used again on the final ZIP extraction. The source manifest declares the
  exact changed files; no Kodex or audit sources are modified.

Local qualification is not a claim that all affected native Etude 6 groups
are now repaired. Missing or ambiguous original identity, unsupported staff or
measure closure, additional source fragments, unsafe occupancy, or unknown
renderer geometry still cause refusal.

## Next native acceptance

Apply this take to r1460 and run `run.cmd --take --all` with the existing
native environment. Keep the original Build/Strict/Clippy/Tests and fresh
10-project renderer sequence. Do not force a PDF or bypass a refusal.

Return the fresh assistant export. Required acceptance evidence:

- FinalExit 0 and successful original strict checks;
- ten successful fresh unforced PDFs;
- ten Master MusicXML files byte-identical to r1460;
- Render MusicXML changes limited to the admitted technical voice scalars;
- source/attack/fragment continuation, timing, pitches, durations, chords,
  rests, ties, staff, all beam levels and the other independent protections
  retained;
- inspect Etude 6, especially the previously flagged measures 2, 30–33 and 40,
  then check the complete corpus for regressions.

The named measures are observation targets only. No productive selector or
write authority is derived from these score locations or their pitches.
