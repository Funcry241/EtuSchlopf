# ClefReadabilityShadowCoursePlanningCompileClosure

## Scope

This is the compile closure for Clef Readability Take 2. The v0.10.1709 strict-clippy run exposed one hard build error: `ClefReadabilityShadowCourseAudit` was re-exported from `structural_write_authority::mod.rs` but no external consumer imported that type. With `#![deny(unused_imports)]`, the re-export is a compile error for the main binary.

## Change

Remove only the unused `ClefReadabilityShadowCourseAudit` re-export. The type remains private to the Rustig clef-planning module where it is constructed, formatted, and tested. External consumers continue to call `derive_clef_readability_shadow_course_audit()` and `append_clef_readability_shadow_course_audit_section()` without naming the type.

## Invariants

- no readability-policy change
- no cost change
- no frontier-planning change
- no golden-case change
- no MusicXML mutation
- no clef materialization
- no writer-authority change
- no report-content change
- Take 2 remains shadow/audit only

## Proof target

Run exactly:

```powershell
.\run.cmd --take --strict --all
```

The full corpus must compile and remain behaviorally neutral.
