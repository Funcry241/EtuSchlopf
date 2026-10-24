# r1207 MidiSeedGenesisCandidateTransport

Parent: accepted/released r1206 `MidiSeedGenesisIdentityTransport`.

## Scope

This take performs only the next transport step for the already-existing MIDI-seed GENESIS (#68).

`SolverCandidate` receives a field named `midi_seed_genesis`. The name is intentionally narrow:
the field records the concrete MIDI-seed provenance from which the candidate was built. It does
**not** claim that an arranged candidate's current `layout_xml` still has the MIDI seed as its
current document GENESIS. Arrangement GENESIS (#62) remains a separate later step.

Both construction exits in `finish_solver_candidate_from_seed` copy the
`PreparedSolverSeed.master_genesis` object into the candidate. The existing seed-equivalence reuse
path can clone a representative candidate for a byte-identical seed; immediately after that clone,
r1207 replaces `midi_seed_genesis` with the identity carried by the concrete seed and validates it
against that seed's profile contract, productive source-plan fingerprint and exact seed XML.

No identity is reconstructed from iteration, measure, staff, voice, pitch, XML location or candidate
position.

## Explicitly out of scope

- no arrangement GENESIS (#62);
- no claim of a current-Master GENESIS for arranged `layout_xml`;
- no EXACT/BLOCK lineage;
- no `layout_xml` encapsulation;
- no swap/drop/field adoption conversion;
- no Render->Best reload change;
- no XML writer or StructuralWriteAuthority change;
- no Render-AdoptedState, WAV, AccentReserve, Proof or Permit work;
- no new selector, ranking, target or music mutation.

Expected product effect: none. Master MusicXML, Render MusicXML and notation/PDF must remain
unchanged from accepted r1206.
