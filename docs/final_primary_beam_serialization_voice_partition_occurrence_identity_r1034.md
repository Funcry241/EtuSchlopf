# r1034 - FinalPrimaryBeamSerializationVoicePartitionOccurrenceIdentityClosure

## Parent and purpose

Authoritative parent: r1033 `FinalPrimaryBeamSerializationVoicePartitionErrorClassAudit`.

r1034 is stage B of `causal_class_first_v1`. The causal error-class definition frozen in r1032 remains byte-identical. r1034 closes exactly one of the three evidence gaps exposed by r1033: exact persisted source-occurrence identity for the final Render-MusicXML observations.

No MusicMutation, writer, repair scope, release authority, threshold, weighting, project identity, measure identity, or witness-conditioned narrowing is added.

## Existing canonical evidence reused

The repository already owns canonical output-occurrence identity through `FinalPdfSourceOccurrenceContinuation`. The existing `NotationVoiceCausalLanePressureAuditProjection` already receives its read-only declassification as rows containing:

`(SourceEventId, SoundingAttackId, measure, pitched_ordinal, MaterializationKind, FragmentOrdinal)`.

r1034 does not add another identity source. It preserves these rows in that existing audit projection and exposes them read-only to the existing persisted-render phase-delta audit.

## Exact persisted occurrence rebinding

r1034 independently enumerates every pitched note in the final persisted Render-MusicXML by physical XML order per measure. Hidden pitched notes and chord members increment `pitched_ordinal`, exactly matching the canonical FinalChord addressing contract established by r991.

For an already structurally bound same-line metric member, persisted occurrence identity is exact only if all of the following hold:

1. the final persisted pitched event has a physical `pitched_ordinal`;
2. exactly one metric member has the same measure, staff, represented onset, represented duration, and pitched ordinal;
3. exactly one canonical occurrence row exists for `(measure, pitched_ordinal)`;
4. that row's SourceEventId and SoundingAttackId equal the metric member's already-bound identity;
5. every metric member is claimed exactly once.

Any missing row, duplicate row, ordinal drift, SourceEventId/SoundingAttackId mismatch, or member multiplicity leaves the observation unresolved. There is no fallback to pitch, voice, beam state, contextual rarity, or known-witness geometry.

## What remains deliberately open

r1034 still does not prove either of the other two frozen-class requirements:

- canonical historical primary-beam authority for the required continuity;
- renderer/PDF-visible primary-beam geometry and the actual visible continuity loss.

MusicXML `<beam>` fragments remain diagnostic only. Therefore `FrozenErrorClassMembersProven` must remain zero in r1034 even if persisted occurrence identity becomes exact for every structural cross-voice observation.

## Expected report change

The frozen-class section gains separate counts for:

- `FrozenErrorClassPersistedRenderOccurrenceIdentityExactBindings`
- `FrozenErrorClassPersistedRenderOccurrenceIdentityUnresolved`

and reports whether exact persisted occurrence identity is available for all, some, or none of the structural cross-voice observations.

When occurrence identity is fully closed but structural cross-voice observations remain, the verdict advances only to:

`CLASSIFICATION_BLOCKED_MISSING_CANONICAL_PRIMARY_BEAM_AUTHORITY_AND_RENDERER_BEAM_GEOMETRY`.

It does not become a repair verdict.

## Tests

Two generic regressions are added:

1. the persisted ordinal join remains exact even when a hidden pitched note occupies an intervening physical ordinal;
2. duplicate canonical rows for one `(measure, pitched_ordinal)` fail closed.

Starting from accepted r1033 with 1342 Strict tests, r1034 therefore expects 1344 tests. The real Windows gate remains authoritative.
