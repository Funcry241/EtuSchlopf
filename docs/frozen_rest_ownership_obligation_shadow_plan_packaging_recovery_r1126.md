# r1126 FrozenRestOwnershipObligationShadowPlanPackagingRecovery

Parent: accepted v0.10.1839 r1124 `SilentVoiceAllSuppressedProofTransportClosure`.
Rejected predecessor: r1125 `FrozenRestOwnershipObligationShadowPlan` was rejected before installation by the fail-closed Take-Tree-Integrity gate because its incoming root accidentally contained the local-transactional `Cargo.lock`. r1125 is rejected Failure Evidence, never acquired authority and is not a parent.

ChangeClass: `RefactorOnly`.

## Purpose

r1126 carries the same narrow semantic shadow cut that was intended by rejected r1125, rebuilt independently from the still-authoritative r1124 parent and with root `Cargo.lock` absent from the incoming Take payload.

The r1124 corpus is release-green, but the existing projection-only `STAFF-LOCAL-REST-OWNERSHIP-AUDIT` still exposes two distinct structural rest classes: exact cross-voice duplicate visible rests and same-lane visible-rest/pitched overlap. The current audit derives both only from already serialized MusicXML. That is sufficient to locate symptoms, but it does not bind the diagnosis to the already frozen positive sounding truth.

The accepted parent already owns a `FrozenSoundingTemporalLanePlan` after final sounding material has been frozen. Its rows are source-bound, staff-bound and temporally exact; it intentionally has no MusicXML writer authority. A later `PrimaryBeamVoicePartition` may still rebind the serialized XML voice of selected sounding members before `FinalPdfSourceCommit`. Therefore the pre-rebind frozen serialized voice labels are not sufficient as final rest-veto identity.

r1126 transports two already existing read-only truths into the rest-ownership audit:

- `FrozenSoundingTemporalLanePlan` supplies final frozen sounding source identity, staff and exact time;
- `FinalPdfSourceCommit::committed_voice_plan()` supplies the actually committed serialized voice after any late Primary-Beam voice rebind.

No new musical owner is invented in this take.

## Change

For every frozen sounding cohort, r1126 projects its exact measure-local temporal segments. Each source member is rebound read-only through the committed final voice plan, not through the earlier pre-rebind voice plan. Cross-measure cohorts are segmented using the same frozen temporal measure domain as the parent plan.

Every frozen sounding member must resolve through the committed final voice plan. If the final plan is absent/unbound, or if any frozen member lacks a committed voice label, `FrozenSoundingVoiceBindingComplete=no` and the new positive-veto classification is suppressed fail-closed.

The existing rest-ownership audit then adds two shadow classifications:

- a visible rest overlapping frozen sounding material on the same staff and the same committed final serialized voice is a positive `FROZEN_SOUNDING_SAME_VOICE_REST_CONFLICT`;
- an exact cross-voice duplicate visible-rest group is classified either as also intersecting frozen sounding truth for one of its voices, or as `NEGATIVE_OWNERSHIP_UNBOUND` when positive sounding truth alone cannot decide which silent voice run, if any, owns the rest obligation.

The second case is intentionally not solved in r1126. Absence of sounding material does not prove a visible-rest obligation. No continuity/VoiceRun inference is introduced and no XML voice label is promoted to global musical identity.

## Authority boundary

r1126 is shadow/projection only in product semantics. `RefactorOnly` is used because the final committed voice plan must be transported read-only across an existing release-stage boundary into the report owner.

- no MusicXML write;
- no render MusicXML write;
- no PDF change;
- no note/rest insertion, deletion, movement or duration change;
- no selector, threshold or musical-priority change;
- no release predicate change;
- no new writer capability;
- the existing `FrozenSoundingTemporalLanePlan` remains writer-authority `none`;
- the committed final voice plan is observed only after the productive commit already chose it.

The report states the boundary explicitly as:

`positive_sounding_veto_bound; negative_rest_obligation_unbound; writer_authority_none`.

## Synthetic proof

The existing audit test surface is extended with a relocatable synthetic case. A visible rest in one serialized voice overlaps a frozen sounding segment in that same voice while a second frozen segment in another voice overlaps the same geometry. Only the same-voice frozen segment may trigger the positive veto. This proves that r1126 does not collapse staff-level polyphony into a false rest conflict.

The proof intentionally uses the audit's final committed voice projection. The pre-rebind voice relation is not treated as authoritative for this classification.

## Expected corpus evidence

The full corpus should remain byte-identical for `.musicxml`, `.render.musicxml` and `.pdf`. Only `.result.txt` gains frozen/committed binding evidence in the existing rest-ownership section.

The new evidence should answer two questions before any productive rest rewrite is attempted:

1. Which current visible rests are already impossible from the accepted frozen positive sounding truth plus the committed final voice relation?
2. Which duplicate-rest cases remain genuinely undecidable until a separate canonical negative Rest-Obligation/VoiceRun model exists?

No corpus witness, project identity, measure identity, pitch, staff number or raw voice label is embedded in executable selection logic.

## Packaging recovery

The root `Cargo.lock` is deliberately absent. The accepted r1124 tree defines root `Cargo.lock` as `LocalTransactional`, never `TakeManaged`; qualified subproject locks under managed tool trees remain untouched. No gate is weakened and no top-level allowlist is changed.
