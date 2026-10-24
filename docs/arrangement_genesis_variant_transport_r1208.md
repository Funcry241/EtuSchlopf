# r1208 ArrangementGenesisVariantTransport

## Scope

One provenance edge only: create and retain GENESIS #62 at the actual arrangement generator boundary.

## Contract

`arrange_piano_musicxml_with_profile(source, profile)` independently creates a candidate document from source plus profile. Therefore its output receives an opaque `ArrangementGenesisIdentity` at creation time. The identity contains:

- creator locator `src/arrangement.rs::arrange_piano_musicxml_with_profile`;
- digest of the full `ArrangementProfile` debug contract;
- fingerprint of the exact input XML supplied to the generator;
- fingerprint of the exact XML emitted by the generator;
- opaque genesis id derived at creation time from those bound values.

The identity is stored in `ArrangementResult`, validated against the unpolished generator output at the `evaluate_arrangement_variant` boundary, and then carried by `ArrangementVariant`.

A local HumanPotential hint-repair does not mint a new GENESIS. It is a later transformation of the already generated arrangement and therefore retains the same `arrangement_source_genesis`.

`normalize_visible_rhythm_musicxml` is explicitly not GENESIS #62. It remains population row 52 / BLOCK_DOCUMENT and returns no arrangement GENESIS.

## Explicit non-goals

This take does not:

- put arrangement GENESIS into `SolverCandidate`;
- claim the arrangement GENESIS is the identity of post-polish/post-layout Master XML;
- introduce EXACT or BLOCK lineage;
- change candidate selection, ranking, targets or musical rules;
- add or relocate an XML writer;
- change Master/Render/PDF bytes;
- touch Render-to-Best, WAV, AccentReserve, Proof or Permit.

The next step, if this take is accepted, can separately decide how the selected `ArrangementVariant` provenance enters `SolverCandidate` without conflating it with the later current-Master state.
