# r1209 ArrangementGenesisCandidateTransport

## Scope

One provenance edge only: transport the already-bound arrangement GENESIS #62 from the selected `ArrangementVariant` into the resulting `SolverCandidate`.

## Contract

`arrange_piano_musicxml_with_profile` already binds `ArrangementGenesisIdentity` at the generator boundary in accepted r1208. `evaluate_arrangement_variant` validates that identity against the exact unpolished generator output and retains it in `ArrangementVariant`.

r1209 adds one optional field to `SolverCandidate`:

`arrangement_source_genesis: Option<ArrangementGenesisIdentity>`

The two construction paths are intentionally distinct:

- MIDI-only / `no_polish` or non-repairable-preledger early return: `None`, because no arrangement generator was used for that candidate document.
- normal arranged candidate: `Some(chosen_variant.arrangement_source_genesis.clone())`.

The identity is transported unchanged. r1209 does not mint a second arrangement GENESIS, infer identity from post-polish XML, or validate the generator identity against the later layout document.

This is provenance, not yet current-state identity. Polish, layout and later Master mutations are not yet represented as an ordered EXACT/BLOCK chain, so the arrangement GENESIS must not be described as the identity of the final `layout_xml` state.

## Explicit non-goals

This take does not:

- add Arrangement GENESIS to the MIDI-only path;
- introduce a current-Master GENESIS selector;
- introduce EXACT or BLOCK lineage;
- change candidate selection, ranking, targets or musical rules;
- add, remove or relocate any XML writer;
- change adoption or swap/drop semantics;
- change Render-to-Best, WAV, AccentReserve, Proof or Permit;
- change Master, Render or PDF bytes.
## Compile-closure note

The accepted codebase uses `#![deny(dead_code)]`. The transported optional field is therefore read immediately after construction of the normal arranged `SolverCandidate` as a transport witness. This read does not inspect, compare, rank, select, mutate, or reinterpret the identity; it only proves that the field is part of the executable transport surface rather than an unread placeholder.

