# r1123 CorpusManifestProjectWriteOutcomeFailureEvidenceProjection

## Scope

r1123 is a failure-evidence-only projection over the accepted r1122 CorpusRunManifest self-check. It changes no manifest validity condition, no project outcome classification, no writer, no notation and no music.

The accepted r1122 corpus remains musically 10/10 green but the hash-bound manifest self-check still returns `manifest_project_write_outcome_inconsistent`. r1122 proved that positively classified SilentVoice safety residuals are not unresolved `candidate_rows_after`, so at least one different existing project-write consistency relation remains false.

When and only when the existing `project_outcomes_consistent` result is false, r1123 appends the already persisted per-project writer projection fields to the existing failure reason. The manifest remains invalid and the corpus exit remains fail-closed exactly as before.

No new evidence producer is introduced. The diagnostic uses only fields already parsed into `CorpusRunProjectEvidence` and does not select, rank, repair or mutate any candidate.

## Required follow-up

Use the next full corpus failure line to identify the exact remaining relation before changing manifest semantics again. No musical change is authorized by this take.
