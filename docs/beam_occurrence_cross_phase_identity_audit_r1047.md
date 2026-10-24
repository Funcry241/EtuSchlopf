# r1047 BeamOccurrenceCrossPhaseIdentityAudit

## Classification

- Parent: accepted r1046 `CrossStaffVoiceLedgerMaterializerAuthorityRetirement`
- Change class: `AuditOnly`
- MusicMutation: no
- Productive selector change: no
- Audit error-classification logic: yes, measurement-only under the existing causal-class-first contract
- Writer change: no
- Frozen error-class definition change: no

## Purpose

r1046 confirmed that MI-A06 can be retired without changing any current corpus product bytes, but the Beam projection remains at 147 Same-Line observations, 144 persisted Cross-Voice observations, 90 exact persisted occurrence bindings and 54 unresolved persisted occurrence bindings.

The current canonical FinalChord occurrence ledger is not itself unresolved. The remaining question is the cross-phase join between the pre-serialization S0 occurrence coordinate and the canonical/final occurrence receipt that is then observed in persisted S1 render MusicXML.

r1047 adds a read-only cross-phase identity audit. Because the new code classifies observations, it is governed as a measurement-only `error_classification` selector against the already accepted parent error-class contract. It does not alter the frozen Beam error-class definition or its 90/54 counters and has no productive selector or writer authority.

## Inputs

The audit compares three already existing evidence domains:

1. S0 pre-serialization occurrence rows:
   - `SourceEventId`
   - `SoundingAttackId`
   - measure
   - pitched ordinal
   - represented onset/duration
   - pitch
   - staff

2. Canonical FinalChord transaction occurrence receipt:
   - `SourceEventId`
   - `SoundingAttackId`
   - measure
   - pitched ordinal
   - materialization kind rank
   - fragment ordinal

3. Persisted final render MusicXML S1:
   - measure
   - pitched ordinal
   - onset/duration
   - pitch
   - staff
   - presentation voice
   - primary-beam encoding

The bridge key used for diagnosis is `SourceEventId + SoundingAttackId`. It is evidence only and gains no selection or write authority.

## Observation classes

For every structurally bound persisted Cross-Voice observation the new audit reports one of these states:

- `LEGACY_EXACT_MEASURE_PITCHED_ORDINAL_BINDING`
- `BLOCKED_S0_SOURCE_ATTACK_MISSING`
- `BLOCKED_S0_SOURCE_ATTACK_MULTIPLICITY`
- `BLOCKED_CANONICAL_SOURCE_ATTACK_MISSING`
- `BLOCKED_CANONICAL_SOURCE_ATTACK_MULTIPLICITY`
- `BLOCKED_CANONICAL_TO_S1_LOCATOR`
- `BLOCKED_CANONICAL_TO_S1_GEOMETRY_MISMATCH`
- `PROVEN_SOURCE_ATTACK_BRIDGE_WITH_LOCATOR_SHIFT`
- `PROVEN_SOURCE_ATTACK_BRIDGE_SAME_LOCATOR`

A source/attack bridge is reported as proven only when the S0 identity and canonical identity are each unique, the canonical locator binds exactly one selected S1 event, and S1 staff/onset/duration/pitch match the S0 occurrence geometry.

## Fail-closed boundary

Multiplicity, missing identity, missing S1 locator, or geometry mismatch remains unresolved. `materialization_kind_rank` and `fragment_ordinal` are emitted as evidence but are not used to force a unique choice when the source/attack identity is multiple.

Therefore r1047 cannot reduce the frozen 54 by declaration. It can only measure which of those 54 are explainable by a unique cross-phase bridge and which require a later causal identity closure.

## Regression coverage

Four AuditOnly tests are added:

1. legacy exact locator binding remains separate;
2. unique source/attack identity can explain a locator-ordinal shift without changing the frozen classifier;
3. S0 source/attack multiplicity remains fail-closed;
4. a source/attack bridge with swapped S1 geometry remains fail-closed.

Parent strict count is 1352. Expected r1047 strict count is 1356.

## Acceptance expectation

A full `--take --all` run is required. Product MusicXML, render MusicXML and visible PDF geometry are expected to remain unchanged because r1047 has no mutation or writer authority.

The important result is the new corpus distribution under `BeamOccurrenceCrossPhase*`. Only after that measurement should the next Beam identity closure be chosen.
