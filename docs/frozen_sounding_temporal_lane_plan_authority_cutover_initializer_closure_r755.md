# Frozen Sounding TemporalLanePlan Authority Cutover Initializer Closure r755

## Parent and purpose

r755 is built independently on accepted r753. Rejected r754 is evidence only and is not a parent. r754 established the intended `RefactorOnly` data edge but failed Strict-Clippy compilation because the new `frozen_sounding_temporal_lane_plan` field was not initialized in the single explicit `CandidateMaterialLineageSpine` literal. r755 preserves that architecture and closes exactly this compile-time initializer omission.

## Binding frontier

The plan is bound only after `FinalSoundingMaterialFreeze` has proven that the CandidateMaterialState and the projected FinalChord sounding XML agree and after the final material seal has been written. Consequently the input is the final active sounding-material set, not the earlier 1312-event post-material lane plan. Authorized deletions, the existing staff/voice relocation and FinalChord onset translations are already reflected before this plan is created.

The temporal truth used by the plan is limited to state already owned by CandidateMaterialLineage:

- `SourceEventId` and `SoundingAttackId` as opaque sounding identities,
- final `staff`,
- final measure-local `onset`,
- semantic `SoundingEnd`,
- final freeze state/projection fingerprints.

No late XML matching reconstructs identity or duration.

## Exact interval plan

For every final active attack r755 computes an absolute half-open interval `[start,end)` from final measure/onset and semantic sounding duration. Exact chord cohorts require the same final staff, same absolute start and same absolute end. Membership retains both opaque identities. Cohorts are deterministically assigned to the first free technical lane of the same staff; any collision or incomplete coverage fails closed.

The immutable plan records coverage, exact cohort count, lane demand per staff, secondary-lane demand, cross-measure cohort/event segmentation demand and a fingerprint chained to the final material freeze fingerprints.

## Compile recovery

The only recovery delta beyond the rejected r754 intent is the explicit initialization of the already-defined field in `initialize_candidate_material_lineage`:

```rust
frozen_sounding_temporal_lane_plan: FrozenSoundingTemporalLanePlan::default(),
```

A full search of `src/` and `tests/` finds no other `CandidateMaterialLineageSpine { ... }` literal. The recovery therefore closes the complete explicit-initializer set without introducing a second constructor path.

## Authority boundary

r755 is deliberately not the MusicXML cutover. It grants none of the following:

- MusicXML voice authority,
- tie writer authority,
- duration/segmentation writer authority,
- note/rest/chord writer authority,
- selection/removal authority,
- staff relocation authority,
- calibration or scoring authority.

No MusicXML is changed in r755. The existing output must remain musically and visually unchanged.

## Next cut

After r755 is accepted, the next MusicMutation can consume this exact frozen plan as the sole temporal representation input for the release-atomic materialization of SourceDuration, segmentation, tie semantics and technical MusicXML voices. That later writer must still satisfy the normal StructuralWriteAuthority, semantic-capability and public-artifact 0/0 release contracts.
