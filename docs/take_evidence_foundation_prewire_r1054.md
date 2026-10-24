# r1054 TakeEvidenceFoundationPrewire

## Classification

- Parent: accepted r1053 `TakeIdentityFailedReexecRejectionDurabilityClosure`
- Parent SHA-256: `0e9d8c22a59e9b8f98aee0d094bc63c8ca486f7a702221cceddbb7e91f510bb3`
- Change class: `KodexOnly`
- MusicMutation: no
- Productive music logic: unchanged
- Reject-decision authority change: no
- Acceptance/Release authority change: no
- Evidence authority: observation only

## Parent proof

The fresh assistant export after the r1053 run contains the canonical acceptance marker

`target/etuschlopf_take_rejection_ledger_v1/0.10.1839__0.10.1839/accepted-r1053-0e9d8c22a59e9b8f98aee0d094bc63c8ca486f7a702221cceddbb7e91f510bb3.txt`

and no pending r1053 transaction marker. The exported r1053 source snapshot is byte-identical to the delivered r1053 Take payload for every managed Take file; only the local root `Cargo.lock` exists outside the Take payload. Therefore AuthorityParent, SourceSnapshot and PhysicalStart are r1053 for this Take.

## Governing invariant

```text
Evidence = Observation
Evidence != Authority
```

Evidence may describe a candidate, a semantic effect and an already authoritative verdict. It must not create, prevent or legitimize Acceptance or Rejection, change gate semantics, or take over lifecycle authority.

## Candidate Intent

The canonical candidate-intent source is the Incoming candidate file:

`src/build_gate/take_change_class.txt`

The bootstrap reads the Incoming bytes before candidate-specific preflight so early rejections can retain candidate intent when available. The existing Take transaction additionally copies the same Incoming file byte-for-byte into `candidate_intent.txt` with `fs::copy`.

No transaction schema bump is introduced. If intent evidence cannot be read or copied, the bootstrap warns and continues with unchanged Gate/Acceptance/Lifecycle semantics.

## Effect

The existing `semantic_effect_snapshot.tsv` remains the effect representation. Its semantics and format are unchanged in this Take.

No replacement effect schema is introduced.

## Rejection evidence schema v2

New rejection records add the following metadata to the existing SHA-bound rejection directory:

```text
schema=2
take_revision=...
rejection_stage=...
verdict=rejected
```

Existing fields, `semantic_effect_snapshot.tsv` and `reason.txt` remain.

When available, exact Incoming candidate-intent bytes are retained as `candidate_intent.txt`.

Legacy schema-1 rejection records remain readable. No historical migration is performed.

The permitted first-authoritative rejection stages are exactly:

```text
dead_code_integrity
dependency_preflight
effect_snapshot
incoming_take
watch_preflight
apply_verify
cargo_lock_resolution
reexec_launch
reexec_exit
crash_recovery
```

There is intentionally no `exact_replay` stage. `rejection_stage` denotes the first authoritative rejection stage of a candidate SHA. The existing SHA-bound ledger entry remains immutable on later attempts.

## Acceptance evidence sidecar

Canonical Acceptance remains the existing file:

```text
target/etuschlopf_take_rejection_ledger_v1/
  <version-pair>/
    accepted-rXXXX-<sha>.txt
```

r1054 does not alter its authority.

A separate advisory evidence root is added:

```text
target/etuschlopf_take_acceptance_evidence_v1/
  <version-pair>/
    accepted-evidence-rXXXX-<sha>/
      metadata.txt
      candidate_intent.txt
      semantic_effect_snapshot.tsv
```

The final sidecar bytes contain no recovery-unstable timestamp. Existing sidecars are accepted only when the exact expected file set and exact bytes match; differing bytes are an Evidence conflict.

The successful-reexec lifecycle order is:

```text
mark_committed
< canonical record_acceptance
< advisory record_acceptance_evidence
< commit
```

A controlled acceptance-evidence I/O or verification failure emits a warning only. Canonical Acceptance remains valid and transaction cleanup continues. This is required so Evidence cannot become Acceptance authority.

For a process crash after `mark_committed`, committed-transaction recovery first re-establishes canonical Acceptance, then retries advisory Acceptance evidence, and finally cleans up the transaction.

## Recovery and rejection durability

r1053's ordering remains unchanged:

```text
rollback
< successful record_rejection
< finish_rollback
```

r1054 only carries candidate intent and rejection-stage metadata through the existing recovery paths. It does not weaken or bypass r1053 rejection durability.

## Regression witnesses

The Take adds or extends tests for:

- exact Incoming candidate-intent copying into the existing transaction;
- rejection schema v2 with revision, first rejection stage, verdict and candidate intent;
- legacy schema-1 rejection readability;
- create-or-verify-exact Acceptance evidence;
- controlled Acceptance-evidence I/O failure preserving canonical Acceptance;
- committed recovery recreating advisory Acceptance evidence before cleanup.

`recovery_transaction_order_kodex.rs` additionally binds the successful order and the non-authoritative Evidence failure semantics to the bootstrap source.

## Explicit non-goals

r1054 does not:

- change candidate selection;
- change which condition accepts or rejects a Take;
- add or weaken any music/repair/write authority;
- change Beam/Owner logic;
- change MusicXML, render MusicXML or PDF behavior;
- change the existing `semantic_effect_snapshot.tsv` format;
- migrate legacy rejection records;
- add Transition History to the Assistant Export;
- add a deterministic candidate Materializer;
- add a Parent synthesis verifier;
- change r1053 rollback/rejection durability ordering.

## Next boundary

Only after r1054 is canonically accepted may later Takes consume this Evidence Foundation for transition history, synthesis context or deterministic materialization. Rule birth precedes rule use.
