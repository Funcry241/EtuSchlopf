# r973 SourceDurationPreplanIslandRetirementCompileClosure

## Parent and failure evidence

`r973` is a `RefactorOnly` compile-closure built directly on the last accepted `0.10.1839 r971` parent.

The preceding `r972 SourceDurationPreplanIslandRetirement` payload is rejected Failure Evidence and is not a parent. Its real Windows strict-Clippy run failed only because two imports remained in the audit projection after their consumers had been retired:

- `SourceSelectionCandidateAuditRow`
- `SourceSelectionShadowPlanRow`

r973 reapplies the same SourceDuration retirement directly to r971 and additionally removes exactly those two dangling imports. No other source-semantic repair is added.

## Retirement scope

The cut removes two historical SourceDuration temporal preplanning families and the audit/shadow machinery that existed solely to compare or prepare those intermediate plans:

- `SourceDurationTechnicalLanePlan`
- `SourceDurationPostMaterialTechnicalLanePlan`

The living final temporal authority remains `FrozenSoundingTemporalLanePlan`.

## Living identity chain preserved

The productive identity/material chain remains:

`SourceSelectionPlan`
→ `MidiSeedSerializedSourceIdentityPlan`
→ `SourceDurationPostLegacyDispositionPlan`
→ `CandidateMaterialLineage`
→ final material freeze
→ `FrozenSoundingTemporalLanePlan`

`bind_candidate_material_lineage_from_post_legacy` binds from serialized source identity plus post-legacy disposition and does not depend on either retired lane plan.

## Protection-owner mapping

The retired preplans are not exclusive final owners of their checks:

- source onset/duration validity remains fail-closed in MIDI normalization;
- retained-source identity remains bound by `MidiSeedSerializedSourceIdentityPlan`;
- post-legacy disposition coverage and projected staff remain bound by `SourceDurationPostLegacyDispositionPlan`;
- material-fragment/source identity and attack coverage remain checked by `CandidateMaterialLineage`;
- final active-attack staff, onset, sounding duration, interval/collision assignment and freeze seal remain checked by `FrozenSoundingTemporalLanePlan`.

No removed audit/shadow comparison is reintroduced as a gate, witness, registry, ledger or forwarding layer.

## Audit contract retirement

The old intermediate plans are historical intermediate states, not aliases for the Frozen plan. Their report fields are retired rather than populated with Frozen values.

The r971 corpus demonstrates this distinction. Etüde 6, for example, had 1,314 post-material pitched events while the Frozen plan had 1,293 active attacks. Backfilling retired report fields with Frozen counts would therefore create false continuity.

The live post-legacy disposition report remains, with its continuation described as `into_candidate_material_lineage` rather than the retired temporal replan.

## Mechanical retirement balance

Relative to accepted r971, r973 removes 1,763 Rust source lines across the four affected Rust files. This is the r972 retirement balance of 1,761 lines plus the two dangling import lines proven dead by the r972 compile failure.

The retirement also removes 13 tests that exclusively exercised the retired preplans and their own audit/shadow contracts.

No replacement implementation is added.

## Intentionally unchanged

- no music mutation;
- no writer/candidate-selection change;
- no threshold or weighting change;
- no SourceSelection semantic change;
- no serialized source-identity semantic change;
- no post-legacy disposition semantic change;
- no CandidateMaterialLineage semantic change;
- no FrozenSoundingTemporalLanePlan change;
- no new ChangeClass, trust boundary, pin class, gate or recovery mechanism;
- root `Cargo.lock` remains LocalTransactional and is not part of the take payload.

## Validation expectation

The real Windows take run remains authoritative. Because this retirement deletes 13 self-tests of the removed mechanism, the expected test-count delta from the r971 baseline remains 1,293 → 1,280 if no other target-count change occurs. Any different unexplained loss is a blocker.

Use the normal one-command full-corpus take path:

```powershell
.\run.cmd --take --all
```

If r973 is rejected, its payload remains Failure Evidence and any correction receives a new revision.
