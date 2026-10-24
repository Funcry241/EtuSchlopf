# r1061 - PrimaryBeamSerializationVoicePartitionRepairScopeClosure

## Purpose

r1061 freezes the first repair scope for the accepted `final_primary_beam_serialization_voice_partition_fragmentation` error class without changing any music, writer or renderer behavior.

The r1060 corpus result changes the repair strategy materially: renderer-visible fragmentation is broad rather than a single Etuede-9 writer anomaly. Across the 144 exact cross-voice observations, r1060 observed 122 renderer-fragmented, 5 renderer-continuity-preserved and 17 fail-closed geometry-unavailable cases. Combined with canonical owner authority, 121 observations are proven frozen-class members and 18 remain unresolved; the 5 preserved observations are explicit non-members.

The correct next step is therefore not a witness-specific `structural_lower_beam` patch. It is to intersect proven class membership with the already existing r1029 materialization-safety proof before any productive channel rebinding is permitted.

## Repair-scope rule

A metric notation line is `REPAIR_SCOPE_ELIGIBLE_ANALYSIS_ONLY` only when both facts are independently true:

1. `FrozenErrorClass` classifies the exact occurrence cohort as `PROVEN_FROZEN_CLASS_MEMBER` under the unchanged r1032 class definition, including current canonical primary-beam authority and final renderer-visible fragmentation.
2. `MaterializationSafetyConjunction` classifies the same metric notation line as `MATERIALIZATION_SAFETY_CONJUNCTION_PROVEN_ANALYSIS_ONLY`.

The join uses the existing opaque `metric_line_id` only as a deterministic equality key. Its value, rarity or location grants no authority.

A proven class member without materialization-safety proof remains `BLOCKED_REPAIR_SCOPE_MATERIALIZATION_SAFETY`. A renderer-continuity-preserved non-member and every unresolved class observation are never selected.

## Current expected corpus evidence

From the accepted r1060 source/run evidence, the expected full-corpus projection is:

```text
Frozen class members proven:        121
Frozen class members unresolved:     18
Renderer-preserved non-members:        5
Materialization-safety proven:       138   (across all metric-line candidates)
Repair-scope eligible intersection:   94
Proven class but safety-blocked:       27
```

These counts are acceptance witnesses only. They are not selector features.

## Multiplicity invariance

The decision is per independent metric line. Adding another independently eligible line cannot change the decision for an existing line. No global ranking, `len()==1`, first/last candidate rule or scorewide cap is introduced.

## Authority

ChangeClass: `AuditOnly`.

MusicMutation: `no`.

WriterAuthority: `none`.

The take records the scope required by a later MusicMutation, but performs no channel rebinding, no `<voice>` rewrite, no Beam rewrite and no renderer rerun.

## Expected validation

The r1060 strict basis is 1385 tests. r1061 adds two generic scope/intersection tests, so the expected strict count is 1387 if no unrelated test topology changes occur.

The full-corpus run must remain 10/10, Clippy warnings must remain zero, and all product outputs must remain governed by the unchanged r1060 productive pipeline.
