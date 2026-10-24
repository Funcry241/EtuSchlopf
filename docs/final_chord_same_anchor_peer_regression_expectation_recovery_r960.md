# r960 – FinalChordSameAnchorPeerRegressionExpectationRecovery

## Authority

r960 builds directly on accepted r956 `FinalChordCanonicalIdentityDirectCarryRetirement`.
r957 was diagnostic and rolled back; r958 was blocked by change-control; r959 installed only transactionally and rolled back because an integrated Rust unit test failed. None of r957-r959 is Authority.

## r959 failure classification

The r959 semantic change compiled cleanly and Clippy passed. The strict failure was in `--bin etuschlopf` unit tests before the Etuede-1 reexec could exercise the productive witness.

The newly added positive regression expected `final_chord_temp_upper_aux_voice_count(&out) == 2`. That helper counts every `<voice>9001</voice>` occurrence, not only generated notes. The existing auxiliary lane serializes two generated notes **plus** one tail `<forward>` carrying the same technical voice to the measure end. The correct count is therefore 3. Existing older tests already encode the same accounting convention (N generated notes + one lane-tail forward where a tail exists).

## Semantic change retained from r959

Only the existing same-pitch/same-onset ambiguity guard in `plan_auxiliary_lane()` is narrowed:

- peers that are chord members of the same physical anchor do not classify each other as foreign owners;
- a same-pitch/same-onset event from another anchor context remains fail-closed;
- tie/notation guards, duration bounds, lane overlap, temporal validator, CandidateMaterial freeze and writer authority remain unchanged.

## Regression correction

The positive same-anchor test still requires:

- two source occurrences -> two AuxiliaryLane provenance rows;
- both pitches preserved;
- one generated note is lane anchor and one remains `<chord/>`;
- second projection is idempotent;
- technical voice occurrence count is 3 because it includes the two notes and the lane-tail `<forward>`.

This is a test-expectation correction only; it does not widen the productive mutation beyond the r959 semantic delta.
