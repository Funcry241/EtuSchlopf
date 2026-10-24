# r1206 MidiSeedGenesisIdentityTransport

Parent: accepted/released r1205 `MasterMutationRequestBoundaryClosure`.

## Scope

This take introduces only the first productive GENESIS carrier at the MIDI-seed boundary.

`PreparedSolverSeed` now carries an opaque `MasterGenesisIdentity` created immediately after
`midi_to_musicxml_seed_with_profile_and_hotspots` has produced its serialized Master seed and
bound the existing source-identity plan. The identity contains:

- opaque `genesis_id`,
- the fixed MIDI-seed creator locator,
- a digest of the complete `CandidateProfile` debug contract,
- the already-authoritative productive source-plan seed fingerprint,
- the post-seed XML fingerprint.

The identity travels with `PreparedSolverSeed` through the existing worker/result transport and is
validated at entry to `finish_solver_candidate_from_seed` before any arrangement work begins.

## Explicitly out of scope

- no `SolverCandidate` GENESIS field yet;
- no arrangement GENESIS (#62) yet;
- no EXACT/BLOCK lineage;
- no `layout_xml` encapsulation;
- no swap/drop/field adoption conversion;
- no Render->Best reload change;
- no Render-AdoptedState, WAV, AccentReserve, Proof or Permit work;
- no new selector, ranking, target or music mutation.

Expected product effect: none. Master MusicXML, Render MusicXML and notation/PDF must remain
unchanged from accepted r1205.
