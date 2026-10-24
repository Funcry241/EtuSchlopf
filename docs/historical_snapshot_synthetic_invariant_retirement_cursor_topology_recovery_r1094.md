# r1094 HistoricalSnapshotSyntheticInvariantRetirementCursorTopologyRecovery

Parent: canonical accepted r1092 `CorpusIndependenceTestAuthorityFoundationOutcomeProjectionRecovery`.

## Purpose

This is the first monotonic E32 historical-test-debt retirement after the Corpus-Independence admission boundary became canonical.

The normative structural-write tests previously loaded `tests/fixtures/beat_aligned_cross_staff_tail_real_pre_operator_snapshot.xml`, a 1,131-line historical pre-operator measure snapshot carrying concrete corpus provenance. Four test owners depended on that witness: the read-only snapshot adapter, semantic serialization-plan proof, metamorphic safety net, and structural writer/direction boundary tests.

The durable invariant is general and remains valuable. The historical witness is not.

## Synthetic replacement

The historical snapshot is deleted. It is replaced by `tests/fixtures/synthetic/cross_staff_tail_structural_invariant.xml`, a deliberately minimal synthetic measure.

The synthetic fixture contains a neutral one-beat four-sixteenth prefix used only to exercise beam-element order invariance, followed by exactly the seven structural events required by the identity-free detector over one two-beat window:

- a three-sixteenth sounding event and a one-sixteenth sounding event forming the already-correct preceding beat;
- a two-sixteenth sounding event at the following beat start;
- one independent cross-staff sounding event;
- one visible overlap rest;
- one hidden carrier;
- one chord member sharing the hidden carrier onset.

The candidate window's normalized `StructuralEvent` sequence is exactly equivalent at the detector-view level to the former target window with respect to onset, duration, primary/secondary beam state, staff relation, voice relation, carrier role, visibility, rest role and chord membership. It intentionally does not preserve the historical measure number, pitch sequence, surrounding music, layout, octave shifts, ties, or unrelated voices/events.

The synthetic direction obstacle remains an instantaneous dynamic because the existing general direction-relocation safety tests need that structural obstacle. Its cursor is derived entirely from the synthetic 4/4 metric topology (beat-four interior) and the tests continue to prove the same relative +48 relocation geometry; no corpus locator participates in that decision.

## Test semantics retained

The same normative properties remain executable:

- one identity-free structural candidate is projected from the synthetic invariant;
- measure/pitch/voice/staff renaming does not alter the detector fingerprint or semantic plan;
- removal of the required overlap rest rejects the candidate;
- the adapter remains audit-only and never issues writer authority;
- semantic event equivalence, chord-normalization proof and cursor-neutral topology remain unchanged;
- five positive metamorphoses preserve intent/plan and six negative structural variants remain rejected;
- writer token consumption, renderer contiguity failure and direction-relocation fail-closed behavior remain covered;
- timing-spanner and `default-x` direction obstacles remain rejected.

No test is added merely to preserve a historical revision. Existing tests are rebound to the synthetic invariant; two historical test/helper names are renamed to describe their current general meaning. Expected total test count remains 1,409.

## Product semantics

No productive function, writer, repair, selector, threshold, ranking, renderer, layout, MusicXML, PDF, sounding-event, release or MasterApply semantics change.

The central `GenericityEvidence` / `candidate_count == 1` question is deliberately untouched and remains a later, separate product-authority topic.

## Change class

`RefactorOnly`.

The physical source changes occur only inside existing `#[cfg(test)]` modules plus the fixture replacement. The historical fixture is removed rather than retained under another historical name. The new fixture lives below the E32-approved `tests/fixtures/synthetic/` boundary.

## Validation target

- Take-Tree-Integrity green with root `Cargo.lock` absent from the incoming Take payload;
- ChangeDependencyPreflight green for `RefactorOnly`;
- Corpus-Independence admission green;
- Semantic-Capability-Diff green with no new productive repair/recovery authority;
- PRE-TAKE-DIFF-GATE green;
- strict formatting, build, Clippy and all-target tests green;
- expected all-target test total: 1,409 passed, 0 failed.

A full musical corpus run is not semantically required because no productive music bytes change. If the normal Take workflow nevertheless executes its standard project run, its output must remain unchanged except for ordinary run metadata.


## Reexec recovery

r1093 was rejected only after post-apply strict reexecution because its first synthetic witness preserved the seven semantic detector events but shortened the auxiliary lane with a partial backup. This recovery keeps the witness synthetic while restoring the general cursor-neutral contract: the target two-beat window is structurally identical, the auxiliary rest lane returns to measure origin with a full backup, and the measure closes on the inherited 4/4 duration. No real corpus identity is reintroduced.
