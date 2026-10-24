# r1150 SemanticObligationAccountingFailureCausalFeatureCorrection

Parent: accepted v0.10.1839 r1148 `SemanticObligationAccountingFailureErrorClassDefinition`. ChangeClass: KodexOnly.

## Purpose

r1150 corrects one causal feature of the error-class contract frozen in r1148, before any take consumes that contract. It stays a `causal_class_first_v1` error_class_definition step without selector-sensitive code.

The corrected contract is:

`docs/selector_derivation_contract_semantic_obligation_accounting_failure.md`

The r1148 record `docs/semantic_obligation_accounting_failure_error_class_definition_r1148.md` stays unchanged as history.

This take does not change MusicXML, Render-MusicXML, PDF output, productive rest selection, writer reachability, thresholds, scoring, corpus targeting, gates or release behavior.

## The defect in the r1148 wording

r1148 required as a causal feature that at least one transformation changes structural facts, occurrence or lane correspondence, or the representation epoch. The same contract states that the defect is the missing conserved object on which survival or legitimate release of an obligation must be accounted.

These two statements contradict each other. A step can leave structural facts and all correspondence relations untouched and still break the accounting continuation of an obligation. That case is exactly what the class exists for, yet the r1148 feature excluded it. Accounting itself is the object of the class, so a change of the musical representation cannot be a precondition for membership.

## The correction

Two causal features replace the narrow one and the per-cell feature:

- The obligation crosses at least one pipeline transformation or representation boundary for which provenance-bound continuation of its accounting identity is required, whether or not that transformation changes structural facts, correspondence relations or the representation epoch.
- For at least one cell of the obligation's mapped domain, that required continuation to a contract-conformant fulfilment in the final persisted epoch is absent, and no contract-admitted terminal discharge evidence applicable under the obligation's own lineage mapping exists.

The causal definition and necessity review state explicitly that a broken accounting chain over unchanged music is a member, even if the resulting music remains correct.

All other contract keys, excluded contexts, the norm-neutrality, the separation of accounting failure, semantic violation and gate escape, and the relation to the r1117 class are unchanged.

## Why correction instead of a new class

No classifier, repair scope or other take references the contract yet. The class name and its intent are unchanged; only a feature that contradicted the intent is removed. Freezing a second, near-identical class would create two contracts for one cause.

From this revision on, any later take consuming the contract must reference it byte-unchanged from an accepted parent.

## Review probes applied

- Exclusion probe: a case that is causally a member but excluded by a feature. Found in r1148: broken accounting over unchanged representation. Closed by this correction.
- Inclusion probe: a case that is causally not a member but satisfies all features. Obligations fulfilled in the origin epoch and never crossing a boundary requiring continuation stay outside, because the first feature requires such a boundary.

## No measurement, no authority

`CorpusDistributionPurpose` stays `not_yet_measured`. r1150 adds no reader, ledger, classifier, selector, repair scope, MusicMutation, writer, threshold, weighting, gate or productive data path.
