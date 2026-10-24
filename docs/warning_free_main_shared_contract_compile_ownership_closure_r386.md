# r386 WarningFreeMainSharedContractCompileOwnershipClosure

Parent: r385 `WarningFreePipelineConsoleContractCompileOwnershipClosure` (real DARK GREEN).

This RefactorOnly take removes two legacy `dead_code` suppressions from the main binary by retiring duplicate private compilation of two contracts that are shared across package compile units:

- `src/corpus_run_manifest.rs`
- `src/build_gate/formal_green_contract.rs`

`src/lib.rs` becomes the single package compile owner for both modules. The main binary imports those existing modules from the package library instead of compiling private copies under `#[allow(dead_code)]`. No function body, threshold, report grammar, corpus-manifest rule, music decision, writer, renderer, repair/recovery authority, or Security policy is changed.

The legacy debt ceiling is intentionally not edited here because it is Security policy. Parent-to-incoming monotonic suppression integrity already prevents reintroduction of the retired suppressions. A later SecurityOnly closure can reduce/pension the debt ceiling after the real compile evidence is green.

Expected evidence after a real Windows take run:

- Strict Clippy warnings remain 0.
- Integrated tests remain green.
- Real suppression count decreases from 22 to 20.
- `dead_code` suppressions decrease from 14 to 12.
