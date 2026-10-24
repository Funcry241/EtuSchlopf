# r1033 - FinalPrimaryBeamSerializationVoicePartitionErrorClassAudit

## Purpose

r1033 is stage B of the `causal_class_first_v1` process. The causal error class was frozen in accepted parent r1032. This take implements only read-only corpus classification against that unchanged definition and reports exactly which class facts the current evidence can and cannot bind.

## Parent

Authoritative parent: r1032 `FinalPrimaryBeamSerializationVoicePartitionErrorClassDefinition`.

The parent contract `docs/selector_derivation_contract_final_primary_beam_serialization_voice_partition.md` is not modified by r1033.

## Measurement boundary

r1033 intersects two pre-existing read-only evidence carriers without adding witness context:

- an exact pre-serialization same-line cohort that already carries an existing primary-beam encoding witness and closed semantic-line constraints;
- the existing S0 -> persisted-final-render structural binding, whose selection explicitly excludes voice labels and beam tags.

Only after that voice/beam-independent structural bind does r1033 observe whether the persisted render representation uses more than one presentation voice and how many primary-beam tag fragments are present in the persisted XML.

This is deliberately not promoted to full frozen-class membership. Three required proof facts remain unavailable at the current boundary:

1. the pre-serialization beam witness is existing encoding evidence, but its canonical historical beam-owner authority is explicitly still unproven;
2. persisted final Render-MusicXML has no source-event / sounding-attack occurrence identity, so its structural binding cannot be upgraded to the exact occurrence provenance required by r1032;
3. there is no final renderer/PDF primary-beam geometry readback, so renderer-visible continuity loss cannot be inferred from MusicXML beam tags.

Therefore every structurally cross-voice observation remains `UNRESOLVED_FROZEN_CLASS_EVIDENCE_GAP`; `FrozenErrorClassMembersProven` must remain zero until those missing facts are supplied independently.

## Why XML beam tags are not the missing visual proof

A single continuous primary-beam fragment in persisted MusicXML does not prove a continuous beam in the rendered PDF. The frozen class is explicitly renderer-effect-specific. r1033 therefore reports XML fragment counts only as diagnostics and never substitutes them for final renderer-visible geometry.

## Explicitly not used

r1033 does not use MaterializationSafety, repair feasibility, project or Etude identity, measure location, leading/trailing rests, exact beat positions, group counts, pitch/register context, staff identity, numeric voice labels, rarity, or corpus uniqueness as class predicates.

No known witness has a special code path. Corpus distribution may surprise us; it may not refine the class.

## Authority

AuditOnly. No MusicMutation, writer, repair scope, selection authority, release authority, threshold, weighting, or productive data-path change. No MusicXML or PDF delta is intended.

## Expected validation delta

Two generic classifier tests are added. Starting from the r1032 strict basis of 1340 tests, the expected strict count is 1342.
