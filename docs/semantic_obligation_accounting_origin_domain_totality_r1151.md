# r1151 SemanticObligationAccountingOriginDomainTotality

Parent: v0.10.1839 r1150 `SemanticObligationAccountingFailureCausalFeatureCorrection`. ChangeClass: KodexOnly.

## Purpose

r1151 closes three remaining gaps in the causal features of the error-class contract frozen in r1148 and corrected in r1150, before any take consumes that contract. It stays a `causal_class_first_v1` error_class_definition step without selector-sensitive code.

The corrected contract is:

`docs/selector_derivation_contract_semantic_obligation_accounting_failure.md`

The r1148 and r1150 records stay unchanged as history.

This take does not change MusicXML, Render-MusicXML, PDF output, productive rest selection, writer reachability, thresholds, scoring, corpus targeting, gates or release behavior.

## The three gaps

1. **Quantification over the image.** r1150 required that continuation be absent "for at least one cell of the obligation's mapped domain". If correspondence is lost for an origin cell, that cell has no image at all. Quantifying over the image then silently drops exactly the cells whose correspondence was lost, so a case the class exists for could fall outside it whenever every remaining image cell is fulfilled.
2. **Opt-out per transformation.** r1150 spoke of a boundary "for which continuation is required". That left open who decides the requirement, and a transformation could declare itself exempt.
3. **Unbound contract version.** "Contract-conformant fulfilment" did not say which contract version judges fulfilment and discharge. A later, relaxed version could make a lost obligation look accounted without any visible normative change.

Gaps 1 and 2 were found by an external adversarial review of r1150. Gap 3 was found by the author's own exclusion probes before packaging.

## The correction

- Accounting must be total on the obligation's origin domain. A cell is accounted only by a complete certified lineage through every subsequent boundary to a fulfilment in the final persisted epoch, or by contract-admitted terminal discharge evidence applicable to that cell under the obligation's own lineage. A missing or broken correspondence is itself missing accounting.
- Accounting continuation is mandatory across every boundary after genesis and is not an opt-in of an individual transformation.
- Fulfilment and discharge are judged under the contract version that derived the obligation and under the image rule that version defines for the obligation kind. A later contract version never decides whether an obligation derived under an earlier version is accounted.

The causal definition and necessity review are updated accordingly. All other contract keys, excluded contexts, the norm-neutrality, the separation of accounting failure, semantic violation and gate escape, and the relation to the r1117 class are unchanged.

## Review probes applied

- Exclusion probe, lost correspondence: the origin cell has no image and is unaccounted, hence a member.
- Exclusion probe, self-declared exemption of a step: continuation is mandatory across every boundary, hence a member.
- Exclusion probe, relaxed later contract version: fulfilment is judged under the deriving version, hence a member.
- Inclusion probe, intended musical removal: a cell deliberately removed by an independently authorized MusicMutation is a member unless the deriving contract admits a terminal discharge form for it. This is intended fail-closed behavior; whether such a discharge form exists is a decision for the later normative contract, not for the class.
- Inclusion probe, no boundary: an obligation whose origin epoch is the final persisted epoch crosses no boundary and stays outside.

## Why correction instead of a new class

No classifier, repair scope or other take references the contract yet. The class name and intent are unchanged; only wording that contradicted the intent is replaced. From this revision on, any later take consuming the contract must reference it byte-unchanged from an accepted parent.

## No measurement, no authority

`CorpusDistributionPurpose` stays `not_yet_measured`. r1151 adds no reader, ledger, classifier, selector, repair scope, MusicMutation, writer, threshold, weighting, gate or productive data path.
