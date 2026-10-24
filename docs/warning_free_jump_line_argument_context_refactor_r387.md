# r387 WarningFreeJumpLineArgumentContextRefactor

Parent: r386 `WarningFreeMainSharedContractCompileOwnershipClosure`

`check_jump_line` had eight positional parameters and therefore carried
`#[allow(clippy::too_many_arguments)]`. The immutable line-identification
inputs (`part`, `measure`, `staff`, `voice`, `line_kind`) are now grouped in
the local `JumpLineContext`. The candidate line, deduplication set and issue
sink remain explicit mutable/owned parameters.

This is a signature-only refactor. Jump thresholds, line construction,
deduplication keys, issue authority, severity, text, staff/voice semantics and
all productive music behavior are unchanged. No Security, build-gate, writer,
render, repair, recovery, pass-order or threshold authority is changed.

Expected warning/suppression effect after real GREEN:
- Strict Clippy warnings remain 0.
- Real suppressions 20 -> 19.
- Remaining suppressions: 12 `dead_code`, 7 `clippy::too_many_arguments`.
