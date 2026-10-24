# r937 CanonicalStrictValidationBasisFailClosedCompileClosure

## Parent and rejected evidence

Parent is released r935 `StrictClippyValidationReceiptReuseKodexClosure`. Rejected r936 is evidence only and is not a parent. r936 reached the Rust reexec gate and was rolled back with E0308 because the new all-target receipt helper declared `StrictBinaryFNV` as `u64` although the long-established `file_fnv1a64()` contract returns the canonical textual form `fnv64:<hex>`.

r937 reapplies the intended final NarfSchlopf closure directly on r935 and changes only that local compile mismatch plus revision/test-fixture metadata. It does not change the existing FNV representation.

## Canonical validation basis

Clippy and all-target proof receipts share one authoritative basis:

`CanonicalStrictValidationBasis = Version + SourceSignature + RustcIdentity + CargoIdentity`.

Specialized proof material remains independent:

- Clippy adds `ClippyIdentity + ClippyContract`.
- All-target tests add `StrictBinaryFNV + TestContract`.

`StrictBinaryFNV` stays a canonical string produced by `file_fnv1a64()`. The helper now accepts `&str`, and regression fixtures use realistic `fnv64:` values rather than integers.

Changing version/source/rustc/cargo invalidates both proof types. A runtime Clippy-identity change invalidates only the Clippy proof; a strict-binary-FNV change invalidates only the all-target proof. Contract constants remain source-resident, so editing either contract conservatively changes the shared SourceSignature and invalidates both proofs. No narrower second source-signature authority is introduced. Build reuse remains separate.

## Fail-closed Clippy receipt creation

The proof-relevant Clippy warning log is read fail-closed. An unreadable or non-UTF-8 strict-Clippy log is an error and cannot become zero warnings. Current strict Clippy uses deterministic `--color never`, bound into the immutable Clippy contract id. Failed or warning-bearing Clippy runs cannot create a receipt; missing, unreadable or mismatching receipts force current Clippy.

## Scope and stop condition

No music rule, HAN03 threshold, repair, writer, renderer, report, public-artifact or release semantics change. After r937 is green and one unchanged Etüde-7 follow-up proves Clippy reuse, verified build reuse, all-target reuse, a real music/PDF run and the unchanged HAN03 remainder, NarfSchlopf is frozen for this cycle.
