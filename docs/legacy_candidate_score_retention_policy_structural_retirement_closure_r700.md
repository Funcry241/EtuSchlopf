# r700 – Legacy CandidateScore Retention Policy Structural Retirement Closure

Parent: r699 `LegacyCandidateScoreRetentionAuthorityCalibrationRetirementRecovery`

ChangeClass: `RefactorOnly`

r699 removed the productive authority of the legacy U-shaped
`ArrangementVisibleRetention` CandidateScore policy by setting its two weights to zero.
r700 performs the promised structural cleanup without changing candidate ordering semantics.

This take:

- removes the dead 68/82 retention branches from `candidate_score_breakdown()`;
- removes retention from the `score_candidate()` API and all 15 productive call sites;
- removes the dead `retention_penalty` field from the score breakdown and source-selection evaluation;
- removes the historical retention-penalty/counterfactual audit projection;
- keeps `ArrangementVisibleRetention` as an independently reported measurement;
- keeps `CanonicalSourceRetention` separate;
- keeps the non-retention CandidateScore breakdown and external-score-adjustment inventory because the complete additive score retirement is the next separate semantic step;
- does not add writer authority, MusicXML mutation, HumanPlayability authority, or new calibration.

The separate monotonic fidelity classifications outside CandidateScore are intentionally unchanged.
They are not the retired U-shaped score policy and remain outside this refactor.

After this closure there is no productive CandidateScore retention input and no dormant
`retention_penalty = 0` compatibility surface left behind.
