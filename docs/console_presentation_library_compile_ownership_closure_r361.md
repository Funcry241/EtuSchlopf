# r361 ConsolePresentationLibraryCompileOwnershipClosure

Parent: real GREEN r358 `RustSourceScanCompileLivenessOwnershipClosure`.

r359 and r360 were rejected before copy and are not parents. r360 showed that changing launcher/bootstrap together with ordinary presentation sources cannot be truthfully classified as `RefactorOnly`; changing the label to `SecurityOnly` would be equally wrong because that class intentionally excludes ordinary source/test paths.

This closure therefore removes the mixed-domain edit entirely.

## Compile-ownership closure

`tests/console_presentation_contract.rs` no longer compiles `src/console_presentation_contract.rs` as a private `#[path]` module instance. The existing package library now exports `console_presentation_contract`, and the integration test consumes that package-owned module. This removes the test-only duplicate liveness world without fake uses or a new dead-code suppression.

Launcher, take bootstrap, pipeline and their security boundaries are not changed by this take.

## Proven zero-consumer retirement

The repository-wide zero-consumer types `PresentationComponent`, `PresentationContext`, `PresentationProgress`, and `TerminalCapability` are retired. No productive consumer is replaced by a synthetic use.

## Non-goals

- no Cargo dead-code lint cutover
- no new `allow(dead_code)`
- no SecurityOnly path or policy change
- no launcher/bootstrap/pipeline change
- no music, writer, repair, recovery, renderer, threshold or output change

The remaining legacy dead-code suppressions are not treated as a target count in this take.
