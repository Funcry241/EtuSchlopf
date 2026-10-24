# r1058 CanonicalPrimaryBeamAuthorityBindingClosure

## Classification

- Parent: r1057 `TakeTransitionHistoryFoundation` **only if r1057 is canonically accepted before execution**
- Change class: `AuditOnly`
- MusicMutation: no
- Productive Beam logic: unchanged
- Writer / repair authority: none
- Frozen error-class definition: unchanged
- Historical owner causality claim: none

## Purpose

The accepted Beam evidence chain already provides three independent facts for every known Cross-Voice observation:

1. exact persisted occurrence identity;
2. exact primary-beam encoding continuity before FinalChord;
3. a typed current-canonical owner receipt transported to the same closed proof-chain cohort.

The existing frozen classifier still reports canonical primary-Beam authority as unavailable because r1050 intentionally stopped before consuming the later owner-receipt evidence. r1058 closes only that stale evidence boundary.

## Authority closure

For an already structurally bound Cross-Voice observation, current canonical primary-Beam authority is now classified as bound only when all of the following are true:

```text
exact persisted occurrence identity
+
exact pre-FinalChord primary-beam encoding witness
+
exactly one typed current-canonical owner receipt for the same proof_chain_id
+
receipt event_count == closed S0 cohort event_count == persisted S1 cohort event_count
+
receipt intended Level-1 states == persisted S1 Level-1 states in structural order
```

Any missing or duplicate receipt, event-count mismatch, missing persisted primary state, or primary-state mismatch remains fail-closed.

## Historical-causality boundary

r1058 does **not** claim that the current owner invocation historically wrote the surviving MusicXML bytes. It proves only that the already accepted current canonical owner semantics are bound to the exact occurrence cohort and require exactly the primary states observed in the persisted representation.

Therefore every authority row continues to state:

```text
historical_owner_causality=not_claimed
```

This distinction is deliberate: canonical current authority and historical writer causality are different propositions.

## Frozen-class advancement

When authority is bound for an observation, the frozen row changes from:

```text
canonical_primary_beam_authority_binding=unavailable_current_projection_has_encoding_witness_not_historical_owner
```

to:

```text
canonical_primary_beam_authority_binding=current_canonical_owner_receipt_bound_exact_occurrence_cohort
```

If all 144 accepted-parent Cross-Voice observations remain stable, the expected corpus-level result is:

```text
CanonicalPrimaryBeamAuthorityBindingCrossVoiceObservations: 144
CanonicalPrimaryBeamAuthorityBindingOccurrenceIdentityBound: 144
CanonicalPrimaryBeamAuthorityBindingOccurrenceIdentityUnresolved: 0
CanonicalPrimaryBeamAuthorityBindingPreFinalChordExactEncodingWitnesses: 144
CanonicalPrimaryBeamAuthorityBindingPreFinalChordEncodingWitnessMissing: 0
CanonicalPrimaryBeamAuthorityBindingTypedOwnerReceiptsAvailable: 144
CanonicalPrimaryBeamAuthorityBindingOwnerReceiptCardinalityUnresolved: 0
CanonicalPrimaryBeamAuthorityBindingOwnerReceiptEventCountMismatch: 0
CanonicalPrimaryBeamAuthorityBindingOwnerReceiptPrimaryStateMismatch: 0
CanonicalPrimaryBeamAuthorityBindingProven: 144
CanonicalPrimaryBeamAuthorityBindingUnresolved: 0
CanonicalPrimaryBeamAuthorityBindingVerdict: CANONICAL_PRIMARY_BEAM_AUTHORITY_BOUND_FOR_ALL_CROSS_VOICE_OBSERVATIONS
```

The frozen class must then advance only to:

```text
FrozenErrorClassCanonicalPrimaryBeamAuthorityBinding:
available_current_canonical_owner_receipt_bound_to_all_cross_voice_occurrences

FrozenErrorClassRendererVisiblePrimaryBeamGeometryReadback:
unavailable

FrozenErrorClassVerdict:
CLASSIFICATION_BLOCKED_MISSING_RENDERER_BEAM_GEOMETRY
```

`FrozenErrorClassMembersProven` remains zero. r1058 does not infer a visible renderer defect from MusicXML beam tags.

## Regression coverage

Two new AuditOnly tests require that:

1. an exact current-canonical owner receipt with exact event count and exact persisted primary states closes canonical Beam authority for the occurrence cohort;
2. a primary-state mismatch remains unresolved even when occurrence identity, pre-FinalChord encoding and receipt cardinality all otherwise match.

Parent strict count is expected to be 1379. Therefore the expected r1058 strict count is 1381.

## Explicit non-goals

r1058 does not:

- mutate MusicXML, render MusicXML or PDF output;
- modify `structural_metric_beam_hierarchy_plan(...)`;
- create a Beam writer or repair scope;
- claim historical writer identity;
- read PDF/SVG Beam geometry;
- classify any frozen error-class member as proven;
- repair Etüde 6 or any other project;
- use project/measure/witness identity as a selector.

## Next boundary

Only after this authority closure is accepted should the next music take add renderer-visible primary-beam geometry readback. That later take must distinguish actual renderer continuity from persisted MusicXML `<beam>` continuity before any repair scope is defined.
