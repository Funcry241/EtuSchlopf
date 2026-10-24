# r935 StrictClippyValidationReceiptReuseKodexClosure

## Parent and evidence

Parent is released/installed r934. The r934 Etüde-7 follow-up proved the strict all-targets receipt path in normal execution: the launcher reported `Rust-Tests alle Targets REUSE: exaktes Strict-Test-Receipt`, while strict Clippy still consumed about 17 seconds before the already verified strict build was reused. The musical result stayed unchanged at the known single HAN03 warning.

## Goal

Remove only that redundant second-run Clippy process while preserving the exact strict Clippy contract fail-closed. This is a Kodex/runtime-validation performance cut; it does not alter music, rules, repair, writer, renderer, report, public-artifact or release semantics.

## Contract

A successful strict Clippy run may write `target/etuschlopf_strict_clippy_validation.receipt` only when its warning budget is exactly zero. The receipt is derived cache with no release or writer authority and is bound to:

- current source build signature (Cargo.toml/Cargo.lock/build.rs/clippy.toml/rustfmt.toml/rust-toolchain.toml plus Rust source tree and Kodex gate core),
- rustc identity,
- cargo identity,
- clippy identity,
- immutable strict-Clippy contract id for `cargo clippy --locked --profile strict -- -D clippy::correctness -D clippy::suspicious -D clippy::perf`.

The verified-build reuse path may reuse Clippy only when this receipt matches byte-for-byte. Missing, unreadable or mismatching evidence executes current Clippy. Compile/forced-build paths continue to run current Clippy. A failed or warning-bearing Clippy run removes any old receipt and cannot create a new one.

The strict product binary remains independently protected by the existing version/source-signature/executable-FNV build verification. The Clippy receipt intentionally does not duplicate binary authority because Clippy validates the source/toolchain/lint contract, while binary reuse already has its own exact owner.

## Historical negative-check identity

The existing negative-check function names `verified_build_reuse_never_skips_current_strict_clippy` and `strict_all_targets_receipt_reuse_does_not_relax_current_clippy` remain present. Their assertions now enforce that verified-build reuse never skips the strict Clippy *contract*: it either runs current Clippy or consumes the exact clean receipt. This preserves the historical semantic-capability identity while replacing redundant process execution with exact proof reuse.

## Expected workflow

1. `run.cmd --take "Etüde Nr. 3"` executes current Clippy and the full all-target test contract and, only on success, records both receipts.
2. With unchanged source/toolchain/binary, `run.cmd "Etüde Nr. 7"` should report both `Clippy REUSE` and `Rust-Tests alle Targets REUSE` before running the full music/PDF pipeline.
3. Any relevant change invalidates the corresponding receipt and restores the full validation process automatically.
