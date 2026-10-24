# r1050 CanonicalPrimaryBeamAuthorityBindingAudit

## Classification

- Parent: accepted r1049 `BeamOccurrenceIdentityClosure`
- Change class: `AuditOnly`
- MusicMutation: no
- Productive selector change: no
- Audit error-classification extension: yes, measurement-only under the frozen final-primary-beam causal class
- Writer change: no
- Frozen error-class definition change: no

## Accepted parent evidence

r1049 closes persisted Cross-Voice occurrence identity for all 144 structural observations:

- 90 legacy exact locator bindings;
- 41 exact `SourceEventId + SoundingAttackId` cross-phase bridges;
- 13 unique local Source/Attack geometry bijections;
- 144 exact bindings total;
- 0 occurrence-identity unresolved.

The remaining frozen-class gaps are canonical primary-Beam authority binding and renderer-visible primary-Beam geometry.

## Purpose

r1050 does not invent a Beam authority. It asks whether each occurrence-identified Cross-Voice observation has evidence stronger than beam encoding alone.

The accepted pre-serialization projection already compares the same primary-beam chain before and after FinalChord. r1050 declassifies that existing result as a structured read-only proof-chain receipt and binds it to the exact Cross-Voice metric line. An exact pre-FinalChord beam witness proves historical encoding continuity only.

A canonical Beam authority binding requires a typed receipt from an already existing canonical planner/verifier/owner bound to the exact occurrence cohort. No such owner receipt is currently carried into this audit projection. Therefore an exact `<beam>` chain, even when present before FinalChord and preserved into S0/S1, remains insufficient to prove authority.

## Fail-closed states

For each already structurally bound Cross-Voice observation:

1. persisted occurrence identity must first be exact under the accepted r1049 closure;
2. the matching pre-serialization proof chain is checked for an exact pre-FinalChord beam-encoding witness;
3. encoding continuity is reported as evidence only;
4. without a typed existing-owner receipt, canonical Beam authority remains unresolved.

No project name, measure number, current voice label, pitch contour, beam-fragment count, corpus rarity, ordinal proximity, first/last candidate, or ranking is admitted as authority.

## Expected corpus readout

The current r1049 corpus evidence predicts:

```text
CanonicalPrimaryBeamAuthorityBindingCrossVoiceObservations: 144
CanonicalPrimaryBeamAuthorityBindingOccurrenceIdentityBound: 144
CanonicalPrimaryBeamAuthorityBindingOccurrenceIdentityUnresolved: 0
CanonicalPrimaryBeamAuthorityBindingPreFinalChordExactEncodingWitnesses: 144
CanonicalPrimaryBeamAuthorityBindingPreFinalChordEncodingWitnessMissing: 0
CanonicalPrimaryBeamAuthorityBindingTypedOwnerReceiptsAvailable: 0
CanonicalPrimaryBeamAuthorityBindingProven: 0
CanonicalPrimaryBeamAuthorityBindingUnresolved: 144
CanonicalPrimaryBeamAuthorityBindingVerdict: AUTHORITY_PROVENANCE_RECEIPT_REQUIRED_ENCODING_CONTINUITY_ALONE_IS_NOT_AUTHORITY
```

The historical frozen readout must remain blocked on Beam authority plus renderer geometry; r1050 is an audit of the missing provenance bridge, not its repair.

## Regression coverage

Three new AuditOnly tests require that:

1. an exact pre-FinalChord encoding witness does not become Beam authority without an owner receipt;
2. a missing pre-FinalChord witness also remains unresolved without inventing authority;
3. occurrence identity must be proven before any Beam-authority binding is evaluated.

The existing pre-FinalChord witness regression is additionally strengthened to verify that the exact proof-chain id is exported structurally by the audit projection.

Parent strict count is 1363. Expected r1050 strict count is 1366.

## Acceptance expectation

A full `--take --all` run is required. Product MusicXML, render MusicXML and visible PDF geometry are expected to remain unchanged. A successful run is informative even when `CanonicalPrimaryBeamAuthorityBindingProven` remains zero: that result proves that the next missing artifact is a typed provenance receipt, not another Beam-shape heuristic.
