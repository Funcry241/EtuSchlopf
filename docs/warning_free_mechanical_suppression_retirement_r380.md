# EtuSchlopf v0.10.1839 r380

## WarningFreeMechanicalSuppressionRetirement

Parent: r379 `WarningFreeNonSecurityClippyCleanup`

Purpose: retire seven non-security warning suppressions whose underlying code can be expressed without local lint suppression.

Changes are behavior-preserving only:
- five `clippy::comparison_chain` suppressions removed; three active comparison chains are rewritten equivalently and two stale suppressions are deleted;
- one stale `unused_variables` suppression removed from a function that already consumes its argument;
- one stale `clippy::redundant_closure` suppression removed from the final-report include boundary.

No `dead_code` debt, `too_many_arguments` debt, launcher/security pin surface, Cargo lint authority, global warning escalation, music rule, threshold, pass order, writer authority, repair/recovery authority or output policy is changed.

Expected invariant:
- strict Clippy remains at 0 warnings;
- real source suppression count decreases by seven;
- no new suppression is introduced.
