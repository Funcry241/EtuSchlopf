# EtuSchlopf v0.10.1839 r379
## WarningFreeNonSecurityClippyCleanup

Parent: real installed GREEN authority r378 `WarningFreeSecurityClippyCleanupPinnedPolicyClosure`.

r378 reduced the strict Clippy warning budget from 138 to 81 without suppression. The 81 diagnostics represent 80 unique source locations because `src/corpus_run_manifest.rs` is compiled through two legitimate compile units.

This take removes exactly the remaining currently observed non-security strict-Clippy findings by source-equivalent Rust refactors:

- 45 `collapsible_if`
- 21 unique `unnecessary_sort_by` source locations (22 diagnostics across compile units)
- 5 `while_let_loop`
- 3 `needless_lifetimes`
- 2 `manual_checked_ops`
- 1 `filter_next`
- 1 `unnecessary_cast`
- 1 `ptr_arg`
- 1 `useless_conversion`

No warning suppression is added or widened. Existing legacy suppression debt is intentionally unchanged in this take; it must be retired by removing its underlying lint/dead-code cause, not by deleting attributes blindly while `dead_code = deny` is already authoritative.

`ChangeClass=RefactorOnly`.

No Security/Build-Control path is changed. No threshold, pass order, candidate ranking, music rule, MusicXML policy, writer authority, repair/recovery authority, or Etude-specific logic is introduced. The source transformations follow the lint's behavior-preserving form: nested predicates become equivalent let/boolean chains, comparator sorts become equivalent key sorts, explicit lifetimes are elided, iterator selection uses the equivalent reverse predicate search, and zero-safe percentage divisions retain the zero-denominator branch through `NonZeroUsize`.

Expected falsification on the real Windows run:

- build/strict tests remain GREEN;
- Etude 6 remains behaviorally unchanged;
- current strict-Clippy warning budget falls from 81 to 0 before legacy suppressions are retired.

This is not the final Warning-Free closure. The project still contains pre-existing suppression debt whose underlying causes must be removed, and only after that may the central warning escalation be activated as the final fail-closed build contract.
