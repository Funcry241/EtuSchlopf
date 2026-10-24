# r1069 - PostAcceptanceAssistantHandoverClosure

## Parent and classification

Parent is the canonically accepted r1068 `PrimaryBeamVoicePartitionTargetIdentityArgumentWarningDebtClosure` candidate with SHA-256 `01c9ac85049f1898c33652f96b6f7d8fa9ac0d52fee2216d76696e0c4bab270a`.

ChangeClass: `KodexOnly`.

MusicMutation: no.

Productive music logic: unchanged.

Acceptance-, Rejection-, Music-, Writer-, Repair-, Selection-, Proof- and Release-Authority remain outside the Assistant handover.

## Problem

The normal Assistant export is produced inside the Child reexec. At that moment the current Take is correctly still `not_yet_canonical`; canonical Acceptance is persisted only afterwards by the accepted outer Take bootstrap. Therefore a successful `assistant_export_latest.zip` can contain the complete current source/run snapshot but still require a separately supplied `accepted-rNNNN-<sha>.txt` before the next thread can establish its parent.

## Closure

After a successful Child reexec the outer bootstrap now preserves this strict order:

```text
mark_committed
-> canonical record_acceptance
-> advisory acceptance evidence
-> post-acceptance Assistant handover finalization
-> transaction commit/cleanup
```

The finalizer does not create Acceptance. It reads back the already persisted canonical Acceptance record and the already persisted advisory Acceptance evidence, byte-binds them into the existing current Assistant-export staging tree, appends one bounded projection-only lifecycle section to `assistant_handoff_latest.txt`, and republishes the existing bundle. The expensive Child export, source snapshot and run artifacts are preserved rather than recomputed.

The added flat handover payload contains:

- the byte-identical canonical `accepted-rNNNN-<sha>.txt` record;
- Acceptance-evidence metadata;
- Candidate Intent;
- Semantic Effect Snapshot;
- `take_lifecycle_handover.txt` with origin paths, byte counts, SHA-256 bindings and the rejection suffix since the preceding accepted revision;
- an explicit statement that `transition_history.json` is the pre-Acceptance advisory snapshot and is not retroactively rewritten.

## Recovery and durability

Canonical Acceptance is now create-or-verify idempotent. If committed recovery sees an already matching Acceptance record, it preserves those bytes instead of rewriting the record with a new timestamp. A conflicting existing record fails closed.

A Post-Acceptance handover I/O failure never records a Rejection and never revokes Acceptance. The transaction remains `committed`, so the next single bootstrap run verifies/restores Acceptance and advisory evidence, finalizes the handover, and only then removes the pending transaction.

Once Acceptance exists, the pre-Acceptance published ZIPs are removed before handover mutation. This prevents a hard process interruption from leaving an obsolete pre-Acceptance `assistant_export_latest.zip` under the current root name. The staging tree remains the recovery source.

## Authority boundary

The Assistant handover is projection/transport only:

```text
projection_only=true
authority=none
```

`transition_history.json` remains advisory. The canonical Acceptance ledger remains the authority source. Handover I/O cannot change the accepted/rejected verdict.

## Tests and witnesses

Four targeted lifecycle witnesses are added/extended:

1. no post-Acceptance claim can be produced before the canonical Acceptance record exists;
2. the finalizer binds exact Acceptance/Evidence while preserving existing Assistant-export and source-snapshot payloads;
3. deterministic handover I/O failure preserves canonical Acceptance and the recoverable `committed` transaction while invalidating published ZIP projections;
4. committed recovery finalizes Acceptance/Evidence/Handover before transaction cleanup.

The Acceptance ledger additionally proves create-or-verify byte idempotence.

Expected strict all-target test inventory: **1,401**.

## Bootstrap transition note

r1069 itself is installed by the r1068 outer bootstrap, so its own Acceptance still uses the legacy pre-r1069 handover timing. This is intentional. No self-invocation, background process or authority shortcut is introduced. The first productive end-to-end Post-Acceptance handover is expected on the next Take executed under installed r1069.

## Validation scope

No corpus-wide music dependency is invalidated. Validation should exercise the Take-bootstrap/Acceptance/Recovery/Assistant-export boundary and the strict test suite. A full ten-project corpus is not required solely for this lifecycle closure.
