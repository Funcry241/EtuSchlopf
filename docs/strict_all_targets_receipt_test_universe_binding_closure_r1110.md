# r1110 StrictAllTargetsReceiptTestUniverseBindingClosure

## Parent and real witness

Parent is canonical accepted r1109 `TakeFailureEvidenceStrictTestProbe`. r1109 was intended to fail in an integration test but was accepted because the launcher reused an older strict all-target validation receipt. The accepted r1109 source therefore provides the direct current witness for a receipt-identity gap.

The r1109 sentinel itself is removed in this closure. No product, music, writer, repair, selector, threshold or release authority is added.

## Root cause

The shared `source_build_signature()` binds root Cargo/build configuration, `src/**/*.rs` and the qualified Kodex gate core. That is appropriate for product build and strict-Clippy reuse, but it does not bind integration-test sources or test fixtures below `tests/` and likewise does not cover future `examples/` or `benches/` target trees.

The strict all-target receipt previously combined that product source signature with toolchain identity, the strict product-binary FNV and the test contract. A test-only source change can therefore leave every bound receipt dimension unchanged and incorrectly reuse a prior green `cargo test --all-targets` proof. r1109 reproduced exactly this state.

## Closure

A separate `strict_all_targets_source_signature()` is introduced only for strict all-target validation. It incorporates the existing product `SourceSignature` plus every regular file recursively below `tests/`, `examples/` and `benches/`. Paths, byte lengths and bytes are hashed deterministically. Symlinks and non-file/non-directory entries in those target trees are rejected fail-closed.

The strict-test receipt gains the explicit dimension `StrictAllTargetsSourceSignature`. Strict-Clippy material deliberately continues to consume only the shared product validation basis. Therefore a test-only or fixture-only change invalidates all-target test reuse without needlessly invalidating strict-Clippy reuse. Product-source changes still invalidate both proof families.

## Regression

A focused regression creates a synthetic product source, integration test and XML test fixture. Changing either the test source or fixture must change the all-target source signature while leaving the product source signature unchanged. Existing receipt-contract tests additionally prove that the test-universe dimension invalidates the strict-test proof but not strict Clippy.

Because r1110 changes the receipt implementation itself, its product `SourceSignature` necessarily differs from r1109 and the stale r1109 receipt cannot match. The authoritative Windows run must therefore execute the current strict all-target suite rather than reuse the r1109 proof. Expected inventory after removing the r1109 sentinel and adding the new regression is 1,394 tests.
