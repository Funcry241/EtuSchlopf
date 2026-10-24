# r1152 SemanticObligationAccountingGenesisSemanticWorld

Parent: accepted v0.10.1839 r1151 `SemanticObligationAccountingOriginDomainTotality`. ChangeClass: KodexOnly.

## Purpose

r1152 closes two remaining gaps in the causal features of the error-class contract `semantic_obligation_accounting_failure`, before any take consumes that contract. It stays a `causal_class_first_v1` error_class_definition step without selector-sensitive code.

The corrected contract is:

`docs/selector_derivation_contract_semantic_obligation_accounting_failure.md`

The r1148, r1150 and r1151 records stay unchanged as history.

This take does not change MusicXML, Render-MusicXML, PDF output, productive rest selection, writer reachability, thresholds, scoring, corpus targeting, gates or release behavior.

## The two gaps

1. **Contract version alone is not the semantic world.** r1151 bound fulfilment and discharge to the contract version that derived the obligation. The contract, however, reads structural facts through an observation schema and lineage through an identity and correspondence schema. A byte-identical contract evaluated over a changed observation schema (for example a changed reading of visibility) or a changed identity schema can reach a different verdict on the same bytes. A lost obligation could then look fulfilled with no visible normative change. Found by external adversarial review of r1151.
2. **Certification defined by acceptance.** "Complete certified lineage" did not say what makes a lineage certified. If certification meant acceptance by some checker, a weak or changed checker could turn missing accounting into apparent accounting, and class membership would again depend on the detector inventory. Found by the author's own probe while applying the first correction.

## The correction

- Fulfilment and discharge are judged under the genesis semantic world of the obligation: the contract version that derived it, the structural-observation schema version whose facts that contract read, and the identity and correspondence schema version that defines its lineage. A later version of any of the three never decides whether an obligation derived under an earlier world is accounted.
- A lineage counts as certified only if it is valid under the identity and correspondence schema of the genesis semantic world. Acceptance by a checker or proof procedure is a detector property and never establishes accounting by itself.
- Meaning and proof procedure are separated explicitly. The genesis semantic world fixes what an obligation means and when it is accounted; the proof regime (proof epoch, proof schema, pipeline contracts) only determines how accounting is demonstrated and may change without changing class membership.
- A later structural-observation take must version its observation schema and the identity and correspondence schema explicitly, because both become part of every genesis semantic world.

All other contract keys, excluded contexts, the norm-neutrality, the origin-domain totality, the mandatory continuation across every boundary, the separation of accounting failure, semantic violation and gate escape, and the relation to the r1117 class are unchanged.

## Review probes applied

- Exclusion probe, changed observation schema under a byte-identical contract: judged within the genesis world, the lost obligation stays unaccounted, hence a member.
- Exclusion probe, changed identity schema reinterpreting a stored relation: same result.
- Exclusion probe, weak checker accepting an invalid lineage: certification requires validity under the genesis identity schema, so the obligation stays a member; the acceptance is a gate escape, not accounting.
- Inclusion probe, proof regime change only: meaning and accounting are unchanged, so no obligation enters or leaves the class merely because the proof procedure changed.
- Inclusion probe, corrected observation schema in a later run: each run derives its obligations within its own genesis world, so a schema fix in a later run neither creates members from earlier runs nor removes them.

## Why correction instead of a new class

No classifier, repair scope or other take references the contract yet. The class name and intent are unchanged. From this revision on, any later take consuming the contract must reference it byte-unchanged from an accepted parent.

## No measurement, no authority

`CorpusDistributionPurpose` stays `not_yet_measured`. r1152 adds no reader, ledger, classifier, selector, repair scope, MusicMutation, writer, threshold, weighting, gate or productive data path.
