# r1165 — ReanchorRestVisibilityScopeWitnessProjectionCompileRecovery

Parent: accepted v0.10.1839 r1162 `FinalChordVisibleRestIntervalEpochWitnessAuditWarningDebtClosure`.
ChangeClass: `RefactorOnly`.

## Recovery reason

r1164 `ReanchorRestVisibilityScopeWitnessProjection` passed the pre-copy governance gates and was installed transactionally, but the post-apply reexec failed with Rust E0599 in `final_report_projection_owner.rs`. The new report projection used `writeln!` on `&mut String` without bringing `std::fmt::Write` into scope. The launcher rolled the source back to r1162, so r1164 is Failure Evidence and never became Parent authority.

## Compile closure

r1165 carries the r1164 diagnostic projection unchanged and adds only the missing local trait import `use std::fmt::Write as _;` inside the existing FinalChord epoch-witness report projection closure. No format string, evidence value, selection rule, action scope, FinalChord projection, writer, MusicXML/Render/PDF mutation, authority, release decision, threshold or semantic contract changes.

The carried r1164 projection still reports only already-computed `ReanchorRestVisibilitySafetyNet` outcome evidence plus dynamic runtime measure identities beside the existing FinalChord visible-rest epoch witness. A same-measure correlation remains a scope witness only; it does not prove exact interval consumption.

Root `Cargo.lock` remains LocalTransactional and is excluded from the Take payload.
