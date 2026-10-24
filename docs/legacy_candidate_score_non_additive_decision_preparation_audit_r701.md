# r701 – Legacy CandidateScore Non-Additive Decision Preparation Audit

Parent: r700 `LegacyCandidateScoreRetentionPolicyStructuralRetirementClosure`

ChangeClass: `RefactorOnly`

r700 has fully removed the retired ArrangementVisibleRetention penalty structure from the
productive CandidateScore. The next separate goal is the retirement of the remaining
additive CandidateScore decision authority itself.

This take does **not** change candidate selection, ranking, MusicXML, writer authority,
renderer behavior or HumanPlayability authority.

Instead it makes the raw decision facts required for a non-additive transition explicit in
the existing source-selection causal projection:

- blocker count,
- critical issue count,
- maximum critical hardness,
- warning count,
- hint count,
- ArrangementVisibleRetention as a separate monotonic conservation measurement,
- human-potential notes moved as an intervention-count proxy.

The existing weighted CandidateScore and RuleMath penalty remain untouched and productive
in r701. They are observed only as the legacy comparator.

The read-only counterfactual order is:

1. fewer blockers,
2. fewer critical issues,
3. lower maximum critical hardness,
4. fewer warnings,
5. fewer hints,
6. higher ArrangementVisibleRetention,
7. fewer human-potential notes moved,
8. deterministic candidate / solver identity.

The counterfactual key does not use `score_total`, any CandidateScore component weight, or
`rule_math_penalty`.

This take also bumps the deterministic audit fingerprint domains for the source-candidate
and solver-frontier serialization because the causal evidence rows now contain the new raw
facts.

The purpose of r701 is to answer, before a productive cutover:

- whether the immutable SourceCandidate winner changes without additive score authority,
- whether the final live solver winner changes without additive score authority,
- and whether the existing final clean state is already selected by the non-additive order.

No score retirement is performed in this take.
