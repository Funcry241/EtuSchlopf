# r1048 BeamOccurrenceSourceAttackMultiplicityLocalBijectionAudit

## Classification

- Parent: accepted r1047 `BeamOccurrenceCrossPhaseIdentityAudit`
- Change class: `AuditOnly`
- MusicMutation: no
- Productive selector change: no
- Audit error-classification logic: yes, measurement-only under the existing frozen final-primary-beam causal error-class contract
- Writer change: no
- Frozen error-class definition change: no

## Accepted parent evidence

r1047 measured 144 persisted Cross-Voice observations. The frozen legacy occurrence binding remains 90 exact / 54 unresolved. The read-only cross-phase bridge explains 41 of those 54 without changing the frozen classifier, leaving exactly 13 observations blocked only because one `SourceEventId + SoundingAttackId` has multiple S0 occurrences. In each observed blocked group the S0 and canonical occurrence cardinalities are preserved (2:2 or 3:3).

## Purpose

r1048 does not close or mutate Beam identity. It measures whether each repeated `SourceEventId + SoundingAttackId` group admits a unique local bijection between its S0 occurrences and canonical/final S1 occurrences.

The conflict group is exactly one repeated `SourceEventId + SoundingAttackId` identity. Project identity, Etude identity, measure number, staff number, presentation voice, current corpus frequency, numeric source id and known witness location are not class-narrowing features.

## Local bipartite graph

Left side:

- all S0 `PreSerializationOccurrenceRow` occurrences for one exact `SourceEventId + SoundingAttackId`
- each row carries measure, S0 pitched ordinal, staff, represented onset, represented duration and pitch

Right side:

- all canonical FinalChord occurrence receipts for the same exact `SourceEventId + SoundingAttackId`
- each receipt locates one persisted S1 pitched event by canonical `measure + pitched_ordinal`
- `materialization_kind_rank` and `fragment_ordinal` are retained as evidence only

An edge exists only when the S1 event located by the canonical receipt has exactly the same:

- staff
- onset
- duration
- pitch

as the S0 occurrence.

## Decision states

A local group is observationally `PROVEN_UNIQUE_LOCAL_BIJECTION` only when:

1. S0 and canonical occurrence cardinality are equal and greater than one;
2. the metric member has exactly one S0 locator inside that group;
3. every canonical receipt locates exactly one persisted S1 event;
4. the bipartite geometry graph has exactly one perfect matching; and
5. the canonical occurrence paired to the metric member belongs to the already structurally selected persisted S1 line.

Fail-closed states are:

- `BLOCKED_CARDINALITY_MISMATCH`
- `BLOCKED_MEMBER_S0_LOCATOR`
- `BLOCKED_CANONICAL_S1_LOCATOR`
- `BLOCKED_NO_PERFECT_BIJECTION`
- `BLOCKED_AMBIGUOUS_PERFECT_BIJECTION`
- `BLOCKED_MEMBER_NOT_IN_SELECTED_S1`

The perfect-matching count is capped after the second solution because only the distinction zero / exactly one / more than one is relevant.

## Explicit non-authorities

r1048 does not use:

- first/last candidate
- numeric ranking
- source or attack numeric order
- pitched-ordinal closeness
- measure proximity
- presentation voice label
- staff preference
- `materialization_kind_rank` as a tie-break
- `fragment_ordinal` as a tie-break
- current corpus uniqueness

Therefore uniqueness is an outcome of exact physical compatibility, not an input authority.

## Frozen classifier separation

The accepted 90/54 frozen Beam occurrence classifier is not changed. r1048 adds only a second explanatory measurement for the 13 r1047 multiplicity-blocked observations. Even a `PROVEN_UNIQUE_LOCAL_BIJECTION` row grants no productive Beam selection or repair authority.

## Regression coverage

Four AuditOnly tests are added:

1. an end-to-end duplicated source/attack group is recovered by exact physical geometry;
2. one perfect matching is reported as unique;
3. two perfect matchings remain ambiguous and fail-closed;
4. no perfect matching remains blocked.

Parent strict count is 1356. Expected r1048 strict count is 1360.

## Acceptance expectation

A full `--take --all` run is required. Product MusicXML, render MusicXML and visible PDF geometry are expected to remain unchanged. The relevant result is the corpus distribution under `BeamOccurrenceSourceAttackMultiplicityLocalBijection*`.
