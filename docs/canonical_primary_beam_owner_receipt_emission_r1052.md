# r1052 CanonicalPrimaryBeamOwnerReceiptEmission

## Classification

- Parent: accepted r1051 `CanonicalPrimaryBeamOwnerReceiptTransportAudit`
- Change class: `RefactorOnly`
- MusicMutation: no
- Rule change: no
- Productive selector/repair change: no
- Take-Change-Control selector-sensitive governance change: yes (error_classification contract only)
- New Beam decision authority: no
- Historical owner causality claim: no
- Renderer geometry change: no

## Purpose

r1051 proved that the current canonical `structural_metric_beam_hierarchy_plan(...)` semantics can be bound read-only to all 144 occurrence-identified Cross-Voice cohorts, but the r1051 receipt is still reconstructed in the audit layer and carries the audit-only `proof_chain_id`.

r1052 moves only the provenance origin, not the musical decision. The existing shared metric Beam DecisionOwner now returns one typed decision object containing:

1. the exact `BeamPlan` already produced by the owner; and
2. an identity-free `CanonicalPrimaryBeamOwnerBoundaryReceipt` derived directly from that same in-memory plan.

No source-event, attack, project, measure, witness or proof-chain identity is admitted at the productive owner boundary.

## Owner-boundary receipt

The receipt contains only owner-local facts:

- schema version;
- owner kind `structural_metric_beam_hierarchy_plan`;
- event count;
- highest owned Beam level;
- fingerprint of the complete actual `BeamPlan`;
- fingerprint of the actual primary Level-1 projection;
- primary states read from the actual plan.

The primary fingerprint intentionally uses the same stable `ordinal:level:state;` FNV64 material as the accepted r1051 read-only primary-plan receipt, so a later take can compare the emitted owner receipt with the existing proof chain without reconstructing a second plan.

## Semantics preserved

`structural_metric_beam_hierarchy_plan(...)` still builds the same `level_groups` and calls the same `structural_write_authority::BeamPlan::from_level_groups(...)` exactly once. The owner receipt is created only after that real plan exists. Receipt construction is total and cannot veto, replace or narrow the already-produced BeamPlan. Any later event-count or primary-projection mismatch is evidence for a later audit, not productive decision authority.

Existing consumers continue to consume the contained `BeamPlan`; no Beam state, write scope, grouping rule, threshold, ranking, tie-break or materialization rule changes.

The two legacy helper sites that require an owned `BeamPlan` explicitly discard the receipt with `into_plan()`. Other consumers retain the decision wrapper only for the lifetime of their existing local plan use. r1052 does not yet transport the receipt to the audit layer.

## r1051 audit continuity

Because r1052 intentionally changes the canonical owner source bytes, the existing r1051 source pin is advanced to the exact r1052 owner-source FNV64 while preserving the same r1051 audit semantics. This is a maintenance synchronization only: the audit still reconstructs its existing current-canonical receipt and still makes no historical writer-causality claim.

## r1052 r01 rejection and r02 correction

The first r1052 candidate (SHA-256 `7dece6b0d6897702cca9919c1a8a44f982c8a288fa91cd5795d4312743265843`) was correctly rejected by `PROXY-IDENTITY SELECTOR KODEX` because selector-sensitive changes in `composer_metric_beam_planning_core.rs` did not carry the required single implementation contract. r02 is rebuilt independently from accepted r1051 and adds exactly one contract: `docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition_owner_receipt_emission.md`, whose unchanged parent error-class contract already exists in r1051.

r02 also removes the accidental receipt-induced `Option` veto from r01: receipt projection is now total after the existing BeamPlan has been produced, so the evidence layer cannot change whether the musical plan exists.

## Regression coverage

The existing shared-owner Level-1 test is strengthened without adding a new test count. It now requires that the emitted boundary receipt:

- identifies the shared owner;
- carries the exact event count and owned level;
- carries the expected `begin/continue/continue/end` primary states; and
- for an L1-only plan, has identical complete-plan and primary-plan fingerprints.

Expected strict-test count therefore remains 1370.

## Explicit non-goals

r1052 does not:

- add `proof_chain_id` to the productive owner;
- transport the boundary receipt to pre-serialization audit state;
- bind the boundary receipt to SourceEventId/AttackId;
- declare canonical Beam authority closed;
- claim which historical writer produced existing MusicXML bytes;
- inspect renderer-visible Beam geometry;
- perform MusicXML, render-MusicXML or PDF mutation.

## Next step

Only after r1052 is accepted should a separate carrier take transport the exact emitted receipt read-only toward the existing proof layer. A later audit take may then bind that real owner receipt to the already-closed 144/144 occurrence identity before any dedicated authority-closure take.
