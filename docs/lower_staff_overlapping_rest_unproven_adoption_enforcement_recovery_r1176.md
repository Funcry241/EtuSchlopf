# r1176 LowerStaffOverlappingRestUnprovenAdoptionEnforcementRecovery

Parent: accepted/released r1173 `LowerStaffBoundDispensabilityAssistantProjectionTrustAnchorRecovery`  
ChangeClass: `MusicMutation`

## Accepted causal contract

This take consumes `docs/selector_derivation_contract_visible_rest_elision_rhythmic_obligation.md` unchanged from the accepted parent lineage.

That contract defines `visible_rest_elision_without_bound_rhythmic_dispensability_proof`: removing an already visible rest requires positive dispensability evidence bound to that carrier and its complete interval. Same-staff activity, raw voice labels, sounding stability, cursor correctness, serialization shape, or activity on another technical voice are not sufficient by themselves. Missing proof yields `Keep` for the existing visible carrier.

r1173 persistently established the concrete lower-staff overlapping-rest witness with `R 192..288`, bound Same-Lane coverage only `192..216`, uncovered `216..288`, `CoverageAuthority none`, and `Status UNPROVEN`. That runtime witness establishes membership in the already-frozen causal class; it is not encoded as a corpus identity in productive logic.

## Why r1175 was rejected

Rejected r1175 preserved the desired product-adoption edge, but added three unit tests that directly called `apply_lower_staff_overlapping_visible_rest_suppression()`. The incoming NO-NEW-REPAIR-AUTHORITY gate named exactly those three test functions as `new_writer_caller_outside_structural_write_authority`. The transaction never started; r1175 is not a parent.

The rejection did not name the existing productive call site itself as a new writer caller. r1176 therefore reconstructs the same minimal product delta directly against accepted r1173 and adds no new direct caller of the legacy writer.

## r1176 product change

`apply_lower_staff_overlapping_visible_rest_suppression()` retains the accepted candidate derivation and selection relation:

- same candidate planner;
- same score and ordering;
- same `max_actions` budget;
- same per-scope bound;
- same byte-range conflict reservation;
- same chosen scope/evidence observation;
- same existing StructuralWriteAuthority call site.

Only product adoption changes. The selected replacement vector remains observable but is not supplied as a productive edit batch. The existing `StructuralWriteAuthority::apply_lower_visible_rest_suppression_authorized` surface receives an empty edit vector. The function returns the original XML and reports zero productive suppressions and zero duration converted to Forward, while retaining candidate/evidence observation.

This is conservative preservation of an already-visible carrier under an existing Keep obligation. It does not create a new positive proof, rest, voice, selector feature, threshold, writer root or release rule.

## Regression boundary

No new unit test directly calls the legacy writer, because doing so would itself create a new writer caller under the accepted NO-NEW-REPAIR-AUTHORITY topology rule. Existing parent tests remain unchanged. The targeted real Etüde Nr. 6 take run is the end-to-end witness for the product edge: the overlap candidate may still be observed, but productive `duplicate_rest_suppressed` and `duplicate_rest_duration_to_forward` must be zero for this writer; the final T40 Render then determines whether any later writer remains in the causal chain.
