# r1049 BeamOccurrenceIdentityClosure

## Classification

- Parent: accepted r1048 `BeamOccurrenceSourceAttackMultiplicityLocalBijectionAudit`
- Change class: `AuditOnly`
- MusicMutation: no
- Productive selector change: no
- Audit error-classification closure: yes, measurement-only under the existing frozen final-primary-beam causal error-class contract
- Writer change: no
- Frozen error-class definition change: no

## Accepted parent evidence

r1048 leaves the historical frozen readout at 90 exact / 54 unresolved by design, while independent accepted audits prove the complete Cross-Voice occurrence identity as:

- 90 legacy exact measure + pitched-ordinal bindings;
- 41 additional exact `SourceEventId + SoundingAttackId` cross-phase bridges;
- 13 additional unique local Source/Attack geometry bijections;
- 0 remaining observations unresolved by the accepted identity evidence.

The accepted r1048 corpus therefore provides 144 Cross-Voice structural observations and 144 independent occurrence-identity proofs, but the frozen measurement has not yet consumed the two newer proof paths.

## Purpose

r1049 consolidates the three already accepted read-only proof paths into the frozen error-class occurrence-identity measurement. It introduces no fourth path and no new heuristic.

For each already structurally bound Cross-Voice observation the closure evaluates, in fail-closed evidence order:

1. legacy exact persisted `measure + pitched_ordinal` to canonical Source/Attack binding;
2. otherwise, unique Source/Attack bridge from S0 to canonical receipt to selected persisted S1 with exact staff/onset/duration/pitch geometry;
3. otherwise, repeated Source/Attack local conflict group with preserved cardinality and exactly one perfect S0-to-canonical-located-S1 geometry bijection whose metric member maps into the selected persisted S1 line;
4. otherwise unresolved.

No ranking or tie-break chooses among candidates. The local perfect matching is accepted only when exactly one complete bijection exists.

## Expected readout

If the accepted r1048 evidence remains stable, the 10-project corpus should report:

```text
FrozenErrorClassPersistedRenderCrossVoiceStructuralObservations: 144
FrozenErrorClassPersistedRenderOccurrenceIdentityLegacyExactBindings: 90
FrozenErrorClassPersistedRenderOccurrenceIdentitySourceAttackBridgeBindings: 41
FrozenErrorClassPersistedRenderOccurrenceIdentityLocalBijectionBindings: 13
FrozenErrorClassPersistedRenderOccurrenceIdentityExactBindings: 144
FrozenErrorClassPersistedRenderOccurrenceIdentityUnresolved: 0
FrozenErrorClassPersistedRenderOccurrenceIdentity: available_exact_for_all_cross_voice_observations
```

The next blocker must then move forward to the already declared gaps:

- canonical/historical primary-Beam authority binding; and
- renderer-visible primary-Beam geometry readback.

r1049 does not close either of those two later gaps.

## Explicit non-authorities

r1049 does not use project identity, Etude identity, current corpus uniqueness, numeric source/attack ordering, first/last occurrence, ordinal proximity, measure proximity, voice label, staff preference, hand-role label, materialization-kind rank or fragment ordinal as selection authority.

`materialization_kind_rank` and `fragment_ordinal` remain reported provenance only.

## Regression coverage

Three AuditOnly regression tests are added:

1. a shifted but unique Source/Attack bridge is promoted into the closed frozen occurrence-identity count;
2. a repeated Source/Attack group with exactly one local geometry bijection is promoted into the closed count;
3. an ambiguous local perfect matching remains fail-closed and cannot become closure evidence.

Parent strict count is 1360. Expected r1049 strict count is 1363.

## Acceptance expectation

A full `--take --all` run is required. Product MusicXML, render MusicXML and visible PDF geometry are expected to remain unchanged. Acceptance requires the corpus-level frozen occurrence-identity readout to become 144 exact / 0 unresolved without changing the later Beam-authority and renderer-geometry gaps.
