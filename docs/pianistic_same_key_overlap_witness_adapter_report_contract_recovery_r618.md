# r618 PianisticSameKeyOverlapWitnessAdapterReportContractRecovery

## Parent and rejection lineage

- Parent: r615 `PianisticEmpiricalCalibrationSemanticSchemaReportContractRecovery`
- Version: `0.10.1839`
- ChangeClass: `AuditOnly`
- Rejected predecessors: r616 `PianisticSameKeyOverlapWitnessAdapterIntegration`; r617 `PianisticSameKeyOverlapWitnessAdapterTestContractRecovery`
- r616 authority: none; rollback to r615 completed
- r617 authority: none; rollback to r615 completed
- MusicMutation: none
- Writer / repair authority: none
- Human capability threshold: none
- HumanPlayabilityClaim: unchanged `NOT_EVALUATED`

## Exact r617 test-gate finding

The real integrated Rust test log identifies exactly one failure in `--bin etuschlopf`:

`audit_only::pianistic_empirical_calibration_profile_contract::tests::generic_observation_cannot_omit_concrete_static_or_transition_signature`

The failed assertion is:

`report.contains("SameKeyTechniqueSignature EXACT_WITNESS_BOUND")`

The test calls `build_for_score_title(None)`. In r617, `ProfileSemanticClosure` already names `SameKeyTechniqueSignature`, but the stronger `WitnessShapeCompatibility: ... SameKeyTechniqueSignature EXACT_WITNESS_BOUND` line is emitted only after the `score_title` early return. The scoreless path therefore cannot satisfy the contract. This is a report-path contradiction, not a Same-Key search/replay algorithm failure.

## Recovery

r618 is rebuilt directly on real green r615. It retains the intended Same-Key witness semantics from the rejected candidates, including the exhaustive one-key negative fixture, but moves the complete `WitnessShapeCompatibility` declaration before the score-title early return. The later duplicate is removed, so scoreless and score-bound reports share one authoritative compatibility declaration.

The resulting report contract is:

`StaticConfigurationSignature EXACT; TransitionSignature EXACT; SameKeyTechniqueSignature EXACT_WITNESS_BOUND.`

## Same-Key execution semantics

1. Exact duplicate physical demands remain coalesced.
2. Coincident non-duplicate attacks on the same MIDI may share one physical key / hand / finger slot.
3. Different abstract release obligations remain distinct.
4. A staggered re-attack during an older same-key obligation reuses the physical slot and requires explicit `SameKeyTechniqueSignature` capability evidence.
5. Missing Same-Key evidence remains `UNMODELED` and therefore yields `MODEL_INSUFFICIENT`, never an implicit negative.
6. A model-relative negative is permitted only when all relevant alternatives are explicitly modeled and rejected.
7. Search and Independent Replay enforce the same contract.

## Reference-assumption precheck

The r300 static precheck partitions distinct active MIDI keys only. Same-Key multiplicity is not treated as a hand-span obstruction and is delegated to the witness Same-Key adapter.

## Claim boundary

`CalibrationProfileBinding NONE` remains unchanged. No score-derived threshold, empirical pianist profile, capability interpolation, human playability claim, writer, MusicMutation or repair authority is introduced.

The architectural boundary remains:

`Score-Demand != Capability != Execution-Witness != Human-Playability-Claim`

and
`UNMODELED != REJECTED`.
