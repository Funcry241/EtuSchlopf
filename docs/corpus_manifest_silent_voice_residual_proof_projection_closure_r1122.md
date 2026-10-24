# r1122 CorpusManifestSilentVoiceResidualProofProjectionClosure

Parent: accepted v0.10.1839 r1121 `SilentVoiceCanonicalShadowRestConflictOwnerClosure`. ChangeClass: `RefactorOnly`.

## Purpose

r1121 intentionally permits residual `SilentVoiceVisibleMeasureCoverage` candidates only when `ExactCandidateDelta` proves that every remaining concrete materialization reproduces the already accepted canonical FinalVisibleRestRole shadow-rest conflict. Those residual rows are therefore authority-withheld safety conflicts, not unresolved writer candidates.

The corpus manifest still projected every raw `CandidateRowsAfter` value as unresolved and required it to be zero. On the fresh r1121 full-corpus run all ten projects were individually OK, all outputs and renderer outcomes were complete, and the batch had zero project errors, but the final manifest self-check failed with `manifest_project_write_outcome_inconsistent`.

## Change

`src/corpus_run_manifest.rs` now consumes the existing r1121 `ExactCandidateDelta` evidence when projecting the final SilentVoice writer outcome:

- raw residual rows with `ExactCandidateDelta=ja` project to zero unresolved candidate rows;
- raw residual rows without that exact proof remain unresolved and therefore continue to fail the existing manifest consistency requirement;
- all project hashes, renderer/writer observation requirements, mutation projections and aggregate write-cause checks remain unchanged.

No writer, musical selector, notation transformation, threshold or corpus work-selection behavior changes. The CorpusRunManifest projection is selector-sensitive classification code and is therefore bound by `selector_derivation_contract_corpus_manifest_silent_voice_residual_proof_projection_r1122.md`, which consumes the accepted r1120 error class unchanged. No new evidence producer is introduced; the manifest only stops misclassifying an already positively proven r1121 safety residual.

## Regression

Synthetic report-line tests cover both directions:

1. nonzero residual rows plus `ExactCandidateDelta=ja` project to zero unresolved rows;
2. the same residual rows with `ExactCandidateDelta=nein` remain unresolved.

## Expected effect

The same r1121 musical corpus should remain 10/10 project-green while the hash-bound `CorpusRunManifest` self-check also closes. Any unproven residual candidate row remains fail-closed.
