# r1068 - PrimaryBeamVoicePartitionTargetIdentityArgumentWarningDebtClosure

## Parent and classification

Parent is the canonically accepted r1067 `PrimaryBeamSerializationVoicePartitionLateRebindMaterialization` candidate with SHA-256 `395d7018d0946308702a2e05e300916103145c77efa4eedd1aa6767ba8d9e465`.

ChangeClass: `RefactorOnly`.

MusicMutation: no.

RuleChange: no.

SelectorChange: no.

WriterChange: no.

Productive music logic: unchanged.

## Motivation

The accepted r1067 full-corpus run completed 10/10 projects with FinalExit 0 and 1,397 strict tests, but the strict Clippy receipt contains one remaining warning:

```text
too_many_arguments (8/7)
```

The warning is emitted by `PrimaryBeamVoicePartitionRepairCandidate::new`. The typed RunOutcome is yellow only because the existing warning-debt projection converts an otherwise released result into `audit_blocked_expected_audit_block`. This take closes only that syntactic warning debt.

## Mechanical refactor

The constructor previously received the already inseparable existing-primary target identity as two adjacent scalar parameters:

```text
target_primary_source_event_id
target_primary_attack_id
```

r1068 passes those same two values across the private constructor boundary as one tuple:

```text
(target_primary_source_event_id, target_primary_attack_id)
```

The constructor immediately writes the tuple components into the exact same existing fields. `target_channel` remains a separate parameter. No stored field, value, derivation, comparison, branch, iteration order, error path or authority changes.

Exactly three constructor call sites are updated: one productive current-evidence call and two existing regression fixtures.

## Governing invariants

The following remain byte/value-semantic invariants across the constructor boundary:

- metric-line identity;
- member `(SourceEventId, AttackId)` pairs;
- measure and staff;
- canonical primary states;
- target primary SourceEventId;
- target primary AttackId;
- target channel.

The r1067 PrimaryBeam selection, late physical rebind, MaterializationSafetyConjunction, renderer-fragment evidence, existing writer authority and checkpoint sink are unchanged.

No selector derivation contract is introduced because no decision logic is added, removed or changed.

## Warning-debt contract

The take declares:

```text
warning_debt_closure=closed_current_warning_debt_v1
```

No lint suppression, threshold relaxation or Clippy-policy change is introduced.

Expected strict-Clippy delta:

```text
too_many_arguments: 1 -> 0
architectural_budget: 1 -> 0
```

## Validation expectation

The strict all-target test inventory remains exactly 1,397.

A narrow runtime witness may exercise Etuede Nr. 6, which had 14 committed `PrimaryBeamVoiceElementsWritten` in the accepted r1067 corpus run. No corpus-wide proof dependency is invalidated by this constructor-only refactor, so a full corpus is not required solely for this warning closure.

Expected result after the authoritative Windows take run:

```text
Strict tests: 1397 / 1397
Clippy warnings: 0
PrimaryBeam semantics: unchanged
MusicXML/PDF semantics: unchanged
```
