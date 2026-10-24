# r1015 – StrictStaff Cross-Voice Ambiguity No-Write Closure

Parent: accepted r1012 `StrictStaffLocalGeometryWitness`.
Rejected r1013 and r1014 are not parents.

## Why this recovery is smaller

r1012 proved a directly bound E6 measure-40/staff-2 transition: the same voice-free
geometry is `G=0` immediately before StrictStaff and `G=1` in the adopted StrictStaff
candidate/readback. r1013/r1014 attempted to transport CandidateMaterial lineage into
the StrictStaff trial in order to recover same-material-voice coverage. r1013 was
rejected because new unit tests became productive writer callers; r1014 was rejected
because that transport modified the existing protected `ProofReentryReader`
`build_strict_staff_local_release_completeness_with_transient_wav_gate_trial`.

r1015 does not transport provenance through that boundary. The protected trial remains
byte-identical to r1012.

## Narrow write-right rule

StrictStaff is a repair/materialization writer. It may fill a proven staff/voice gap
only while its own evidence is unambiguous. If a proposed lane gap overlaps a visible
pitched fragment on the same staff under another current render voice, StrictStaff does
not possess enough local evidence to decide whether the missing rest is genuine
polyphony or a render-voice split of already occupied material.

Therefore that lane receives no new StrictStaff overlay write right.

This is intentionally a negative guard:

- it does not relabel voices;
- it does not delete, hide, shorten, or move any existing rest;
- it does not assert that a polyphonic rest would be musically invalid;
- it does not add a new writer, writer root, provenance channel, threshold, or selector;
- later canonical rest-role owners remain unchanged.

## E6 consequence

In the r1012 witness, staff 2 / voice 2 has the proposed gap `[192,288)`, while a
visible pitched fragment on staff 2 under another render voice occupies `[192,216)`.
The new guard therefore blocks this repair overlay before `gap_parts` are materialized.
The known bad quarter-rest geometry must remain `G=0` in the StrictStaff candidate.

The guard deliberately does not synthesize a residual `[216,288)` rest. Such a
segmentation would require a stronger musical identity proof than StrictStaff currently
owns. r1015 chooses the smaller fail-closed contract.

## Regression strategy

No test source is changed and no new productive writer caller is introduced. The already
installed r1012 `StrictStaffLocalGeometryWitness` is the integration regression for this
exact E6 transition. The expected total therefore remains **1332 tests**.

## First execution

Run only:

`./run.cmd --take "Etüde Nr. 6"`

Expected local evidence:

- the protected StrictStaff trial remains active through its existing topology;
- target trace contains `no_overlay_same_staff_other_voice_pitch_ambiguity` for E6
  measure 40 / staff 2 / voice 2;
- `StrictStaffLocalGeometryWitness` candidate transition for the old
  `[192,288)` × `[192,216)` geometry is `0->0` (or no StrictStaff write if no other
  safe candidate remains);
- master MusicXML stays byte-identical;
- render/PDF may change only if removal of this newly invented overlay propagates to
  the committed render.

No `--all`, no `--inventory` for this first gate.
