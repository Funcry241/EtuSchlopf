# r1505 BeamAnchorPlannerInputTrace

Parent r1504, RefactorOnly. Productive policy and materialization unchanged.

The actual `derive_clef_readability_primary_course_plan` call collects existing
scanner observations alongside the same pitches, then traces the actual supplied
part solution. No rescan is substituted for the productive input. Eligible parts
have supported nonempty lower clef surfaces and pass the existing initial-surface
guard. Skipped ineligible parts have no trace; absence is not zero. Calls from
prepared/solver/probe paths are planning evidence, never committed writer evidence.

Trace goes to stderr (existing pipeline logging ingress), not a new global buffer
or sidecar. Each record binds exact input bytes via length and existing FNV64,
part content via FNV64, and descriptive part index. FNV64 is an existing diagnostic
fingerprint, not a cryptographic identity or globally unique call identifier.
Repeated identical inputs can have identical bindings; log occurrence order is the
only occurrence context. Interleaved emission is avoided with one stderr lock per
part block. Best-effort I/O failure is ignored and cannot veto writing.

`BeamAnchorPlannerInput` counts all visible lower primary-beam observations and
cross-measure stored-origin rows. Cross-measure is descriptive, not an error rule.
`BeamAnchorPlannerContext` emits every cross-measure row without truncation using
the existing r1504 formatter, preserving Unknown versus live stored origin.
For parts with such rows, `BeamAnchorPlannerCandidates` lists the unchanged
collector result with part-local byte positions. `BeamAnchorPlannerSolution`
projects retained surfaces, inserted clefs and costs of the actual part solution,
before global normalization; missing solution is unavailable, never empty success.
Every line says Authority=none. WriterCommit=not_observed is explicit.

This take does not reconstruct musical-time continuity, classify remote beam
errors, modify candidate selection, run a counterfactual solver, prove causality,
or claim final written artifacts. Compare repeated phase inputs and use source
fingerprints to recover the relevant input before any future repair decision.
The old final-document audit and reports remain byte-identical for equal inputs.

Native acceptance: run .\run.cmd --take --all; verify exact candidate hash,
full tests/build/clippy, 10/10, unchanged 20 XMLs and all PDF pages against r1504.
Check BeamAnchorPlannerInput/Context/Candidates/Solution in actual console log;
do not relabel them Master/Render without a matching artifact fingerprint.
