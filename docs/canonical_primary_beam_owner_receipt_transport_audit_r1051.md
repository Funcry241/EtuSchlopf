# r1051 CanonicalPrimaryBeamOwnerReceiptTransportAudit

## Classification

- Parent: accepted r1050 `CanonicalPrimaryBeamAuthorityBindingAudit`
- Change class: `AuditOnly`
- MusicMutation: no
- Productive selector change: no
- Audit error-classification extension: yes, measurement-only under the frozen final-primary-beam causal class
- Writer change: no
- Frozen error-class definition change: no
- Historical owner causality claim: no

## Accepted parent evidence

r1050 proves for all 144 occurrence-identified Cross-Voice observations:

- persisted occurrence identity is exact;
- the primary-beam encoding already exists exactly before FinalChord;
- no pre-FinalChord encoding witness is missing;
- encoding continuity alone is not canonical Beam authority;
- the missing artifact is a typed owner receipt.

The current source also retains the accepted shared BeamHierarchy DecisionOwner `structural_metric_beam_hierarchy_plan(...)`. r1051 does not infer historical causality from that fact. It asks a narrower question: can the current canonical owner semantics be bound, source-exactly and read-only, to the same pre-serialization proof-chain cohort?

## Receipt transport

The pre-serialization audit now emits a typed current-canonical-owner receipt only when all of the following hold:

1. the existing pre-FinalChord primary-beam witness is exact;
2. the complete current `composer_metric_beam_planning_core.rs` source has the accepted r1050 FNV64 `b304f23e30b16e79`;
3. that source still contains the named shared owner and its primary Level-1 complete-group construction;
4. the cohort contains at least two events;
5. the shared identity-free `BeamPlan` primitive can produce one primary Level-1 plan for the complete ordered cohort.

The receipt carries:

- the existing `proof_chain_id`;
- owner kind `structural_metric_beam_hierarchy_plan`;
- exact owner-source FNV64;
- event count;
- deterministic plan fingerprint;
- expected primary states (`begin`, zero or more `continue`, `end`).

This is a **current canonical owner decision receipt**, not proof that the historical bytes were originally produced by a particular invocation. The historical-causality field remains explicitly `none`.

## Cross-phase transport check

For each already occurrence-identified Cross-Voice observation, the later read-only audit requires:

- exactly one closed pre-serialization proof-chain match;
- exact pre-FinalChord encoding witness;
- exactly one current-canonical-owner receipt for the same proof-chain id;
- receipt event count equal to both the closed S0 cohort and the bound persisted S1 cohort.

No ranking, proximity, project identity, measure special case, voice preference, pitch contour, first/last candidate or corpus frequency is admitted.

## Expected corpus readout

The accepted r1050 evidence predicts:

```text
CanonicalPrimaryBeamOwnerReceiptTransportCrossVoiceObservations: 144
CanonicalPrimaryBeamOwnerReceiptTransportOccurrenceIdentityBound: 144
CanonicalPrimaryBeamOwnerReceiptTransportOccurrenceIdentityUnresolved: 0
CanonicalPrimaryBeamOwnerReceiptTransportPreFinalChordExactEncodingWitnesses: 144
CanonicalPrimaryBeamOwnerReceiptTransportPreFinalChordEncodingWitnessMissing: 0
CanonicalPrimaryBeamOwnerReceiptTransportTypedCurrentCanonicalOwnerReceipts: 144
CanonicalPrimaryBeamOwnerReceiptTransportEventCountMismatch: 0
CanonicalPrimaryBeamOwnerReceiptTransportProven: 144
CanonicalPrimaryBeamOwnerReceiptTransportUnresolved: 0
```

The r1050 `CanonicalPrimaryBeamAuthorityBindingAudit` and the frozen class remain unchanged in r1051. Even a 144/0 transport result is therefore evidence for a later dedicated closure take, not an in-place authority reclassification.

## Regression coverage

Four new AuditOnly tests require that:

1. the current canonical owner source is bound to the accepted source fingerprint and produces the expected primary-only typed receipt;
2. a one-event group fails closed and cannot receive a receipt;
3. an exact Cross-Voice cohort with one matching typed receipt transports successfully;
4. receipt/S0/S1 event-count mismatch remains fail-closed.

Parent strict count is 1366. Expected r1051 strict count is 1370.

## Acceptance expectation

A full `--take --all` run is required. Product MusicXML, render MusicXML and visible PDF geometry are expected to remain unchanged. If transport is 144/0, the next step is a separate canonical Beam-authority closure decision; renderer-visible geometry remains out of scope.
