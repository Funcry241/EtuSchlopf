# r705 – Legacy CandidateScore Guard Transition Contract Audit

Parent: r701 `LegacyCandidateScoreNonAdditiveDecisionPreparationAudit`

ChangeClass: `AuditOnly`

r703 proved that the productive non-additive CandidateScore authority cutover passes the
PRE-TAKE semantic gate, Strict-Clippy and Build, but the reexec rolled back because the
historical `source_selection_candidate_guard_differential_audit` contract still required
`candidate.score` as an explicit productive guard key. r704 correctly attempted to rebind
that contract, but Take-Change-Control rejected the package before semantic admission because
`MusicMutation` is not allowed to modify an Audit-owned test path.

r705 changes no productive source and no candidate decision. It prepares the existing guard
contract for the already-proven authority cutover without weakening the guard invariant.
The test accepts exactly two named architecture states:

1. the current r701 legacy guard, where issue criticality precedes `candidate.score`, or
2. the intended `CandidateDecisionKey` guard, where blocker count, critical count, maximum
   critical hardness, warnings and hints precede ArrangementVisibleRetention and minimal
   intervention, while `candidate.score` is absent from the guard.

Both states continue to reject `CanonicalSourceRetention` as a selection/guard key. The
future-state branch is activated only by the explicit `struct CandidateDecisionKey` surface;
it is not a generic fallback and does not permit an intermediate unknown ordering.

No MusicMutation, calibration, writer, MusicXML, renderer, HumanPlayability or ranking
semantics are changed in r705. After r705 is accepted, the productive authority cutover can
be retried as a separate MusicMutation take without touching the Audit owner.
