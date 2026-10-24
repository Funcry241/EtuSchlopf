# r1084 PrintCleanupRestProxyObservationPreservingRetirement

Parent: canonical accepted r1082 `MultiplicityProxyAuthorityRetirementSuperCut`.

Rejected r1083 proved that collapsing a retired writer's observation count together with authority is too
broad. r1084 therefore preserves the exact candidate-class observation while retiring only materialization.

Before:
`apply_pianistic_rest_consolidation(xml, 1)` detected all candidates, selected at most one, materialized it,
and returned the full detector count.

After:
`build_rest_consolidation_candidates(xml).len()` preserves the same full detector count; the PrintCleanup
owner starts from unchanged XML with no rest action and continues its independent clef cleanup exactly as
before.

Thus MI-A05 loses write authority without falsifying its diagnostic surface.

MI-A12 productive Rust remains byte-identical. Its witness-independent causal class is frozen as a parent
contract for the next selector-sensitive closure.

Expected state after acceptance: 11/12 class-A multiplicity debts productively closed; only MI-A12 open.

Validation: `run.cmd --take --all`.
