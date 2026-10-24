# r1053 TakeIdentityFailedReexecRejectionDurabilityClosure

## Classification

- Parent: accepted r1052 `CanonicalPrimaryBeamOwnerReceiptEmission`
- Change class: `KodexOnly`
- MusicMutation: no
- Rule change: yes, Lifecycle-/Recovery-Kodex only
- Productive music logic: unchanged
- Evidence authority change: no
- Acceptance/Release authority change: no

## Parent proof

The fresh assistant export after the r1052/r02 run contains the canonical acceptance marker

`target/etuschlopf_take_rejection_ledger_v1/0.10.1839__0.10.1839/accepted-r1052-a406e16412e634340cf8f1050b99b5831a2661bbead41fd777981502c5e893db.txt`

and no pending r1052 transaction marker. Therefore r1052 is the authority parent for this take.

## Defect

The failed-reexec path historically executed:

```text
transaction.rollback(&project_root)?
record_rejection(...)?
```

but `rollback()` itself also called `finish_rollback()`. This allowed the Pending marker and transaction root that carry candidate/recovery provenance to disappear before the canonical Rejection record was durably written. A subsequent Rejection-ledger I/O failure could therefore leave Source restored to Parent while the recovery source for the rejected candidate had already been destroyed.

## Closure

`rollback()` now performs only:

```text
Public Artifact Recovery
-> Source Restore
-> Restore Verification
-> Bootstrap Cache Cleanup
-> state = rolled_back
-> return
```

It no longer calls `finish_rollback()`.

The existing adversarially bound failed-reexec call site remains literally:

```rust
transaction.rollback(&project_root)?;
```

and the path now completes in the required order:

```text
rollback
< successful record_rejection
< finish_rollback
```

If `record_rejection()` fails, the function returns while `state = rolled_back`, the Pending marker remains and the transaction root remains. The next recovery run can therefore persist the same Rejection and only then clean up provenance.

## Deliberate temporary duplication

`rollback()` and `rollback_preserving_provenance()` intentionally retain near-identical provenance-preserving core semantics in this take. They are not merged, renamed or wrapped. Source-witness stability is preferred over local cleanup; consolidation is out of scope.

## Regression witnesses

The existing bound test name `transactional_apply_can_rollback_exactly` remains unchanged. It now proves the two-phase lifecycle explicitly:

1. `rollback()` restores Parent Source and Parent `Cargo.lock`;
2. transaction state is `rolled_back`;
3. Pending marker and transaction root still exist;
4. explicit `finish_rollback()` removes both.

The existing positive recovery witness `pending_rolled_back_transaction_recovers_rejection_before_cleanup` remains registered.

A new deterministic test `failed_reexec_rejection_io_failure_preserves_recovery_provenance` creates the Rejection-ledger root as a regular file. `record_rejection()` therefore fails without platform-specific permission tricks. The test proves that rolled-back transaction provenance survives the failure, then removes the conflict, invokes `recover_pending_transaction()`, verifies persistent Rejection provenance and finally verifies cleanup.

## Kodex binding

`recovery_transaction_order_kodex.rs` now fails closed unless:

- `rollback()` contains no `finish_rollback()` cleanup;
- the bound failed-reexec `transaction.rollback(&project_root)?;` witness remains;
- `record_rejection()` follows that rollback;
- `transaction.finish_rollback()?;` follows successful Rejection persistence;
- the bound rollback test, deterministic I/O-failure test and existing rolled-back recovery test remain registered.

`identity_gate_trust_anchor.rs` is rebound only to the final bytes of the changed bootstrap and recovery-order Kodex owners.

## Explicit non-goals

r1053 does not:

- change which candidate is accepted or rejected;
- change Rejection semantic classification;
- add Evidence Foundation artifacts;
- add candidate intent/effect schemas;
- change Rejection schema versions;
- change Beam/Owner logic;
- change MusicXML, render MusicXML or PDF behavior;
- weaken any gate;
- remove or relocate the bound rollback call site or bound test name.

## Next step

Only after r1053 is canonically accepted may the separate `TakeEvidenceFoundationPrewire` work begin. Evidence remains observation, never authority.
